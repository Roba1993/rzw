
//! Serial Driver implementation to communicate with the serial Z-Wave devices.
// `header, length, type(rx|tx), zw-function, data, transmit-type, message-id, checksum`
//
// ZWave data structure for basic
// `device, data-length, comand class, command, value`

use num::FromPrimitive;
use error::{Error, ErrorKind};
use serial::{self, SystemPort, SerialPort};
use std::io::{Write, Read};
use std::time::Duration;
use driver::{Driver, GenericType};
use std::io::ErrorKind as StdErrorKind;
use std::fmt;

pub struct SerialDriver {
    // serial port
    port: SystemPort,
    // message id counter
    message_id: u8,
    // message store
    messages: Vec<SerialMsg>,
    // serial driver path
    path: String
}

impl SerialDriver {
    /// Creates a new SerialDriver which is a connection point to
    /// ZWave device & network.
    pub fn new<P>(path: P) -> Result<SerialDriver, Error>
        where P: Into<String> {
        // get the path
        let path = path.into();

        // try to open the serial port
        let mut port = try!(serial::open(&path));

        // set the settings
        try!(port.reconfigure(&|settings| {
            try!(settings.set_baud_rate(serial::Baud115200));
            settings.set_char_size(serial::Bits8);
            settings.set_parity(serial::ParityNone);
            settings.set_stop_bits(serial::Stop1);
            settings.set_flow_control(serial::FlowHardware);
            Ok(())
        }));

        // set the timeout
        try!(port.set_timeout(Duration::from_millis(100)));

        // create the new struct
        let driver = SerialDriver {
            port: port,
            message_id: 0x00,
            messages: vec!(),
            path: path
        };

        // return it
        Ok(driver)
    }

    // Count the message_id up and return the new
    // message_id
    fn get_next_msg_id(&mut self) -> u8 {
        // count the message_id up
        self.message_id += 1;

        // jump over 0x00 it's reserved
        if self.message_id == 0x00 {
            self.message_id += 1;
        }

        // return the message id
        self.message_id
    }

    /// This function reads a single message from the ZWave device/driver
    fn read_single_msg(&mut self) -> Result<SerialMsg, Error> {
        // buffer to read each byte in
        let mut buf = [0u8; 1];
        // result vector
        let mut result : Vec<u8> = Vec::new();

        // try to read the first byte
        self.port.read(&mut buf)?;

        // when the first byte is the start of a frame
        if buf[0] == SerialMsgHeader::SOF as u8 {
            // add the header byte to the result
            result.push(buf[0]);

            // read the next byte which includes the length
            self.port.read(&mut buf)?;

            // add the length to the result
            result.push(buf[0]);

            // read the full length of the message
            let len = buf[0];
            for _ in 0..len {
                // read a byte
                self.port.read(&mut buf)?;
                // add the byte to the result
                result.push(buf[0]);
            }

            // create the message
            let m = SerialMsg::parse(result.as_slice());

            // if it was successfull return ACK
            if m.is_ok() {
                self.port.write(SerialMsg::new_header(SerialMsgHeader::ACK).get_command().as_slice())?;
            }
            // if there occoured an error send back a NAK
            else {
                self.port.write(SerialMsg::new_header(SerialMsgHeader::NAK).get_command().as_slice())?;
            }

            //return the message
            return m;
        }
        // on message ackonwledge
        else if buf[0] == SerialMsgHeader::ACK as u8 {
            return Ok(SerialMsg::new_header(SerialMsgHeader::ACK));
        }
        // on message not ackonwledge
        else if buf[0] == SerialMsgHeader::NAK as u8 {
            return Ok(SerialMsg::new_header(SerialMsgHeader::NAK));
        }
        // on resent
        else if buf[0] == SerialMsgHeader::CAN as u8 {
            return Ok(SerialMsg::new_header(SerialMsgHeader::CAN));
        }

        // if the header is unknown return a error
        Err(Error::new(ErrorKind::UnknownZWave, "Unknown ZWave header detected"))
    }

    /// Reads a single message from the zwave driver. It retries to read after a timeout as defined.
    fn read_single_msg_rty(&mut self, tries: &i32) -> Result<SerialMsg, Error> {
        // set the variable to count
        let mut counter : i32 = tries.clone();
        loop {
            // throw an error when we tried to read too much
            if counter <= 0 {
                return Err(Error::new(ErrorKind::Io(StdErrorKind::TimedOut), "Timeout"));
            }

            // count down
            counter -= 1;

            match self.read_single_msg() {
                // on timeout error try to read again
                Err(e) => {
                    if e.kind() == ErrorKind::Io(StdErrorKind::TimedOut) {
                        continue;
                    }
                    else {
                        return Err(e);
                    }
                },
                // on message recveive check if it's
                Ok(m) => {
                    return Ok(m);
                }
            }
        }
    }

    /// Reads all messages and stores them to the table.
    fn read_all_msg(&mut self) -> Result<bool, Error> {
        // read all messages
        loop {
            // try to read a message 3 times
            match self.read_single_msg_rty(&3) {
                // when there is a timout quit
                Err(e) => {
                    if e.kind() != ErrorKind::Io(StdErrorKind::TimedOut) {
                        return Err(e);
                    }
                    else {
                        return Ok(true);
                    }
                },
                // store the message to the table
                Ok(m) => {
                    // Can't handle this data type right now - needs a recheck
                    if m.header == SerialMsgHeader::SOF && m.typ == SerialMsgType::Request &&
                            m.func == SerialMsgFunction::SendData && m.data[1] == 0 && m.data[2] == 0 {
                        continue;
                    }
                    // save incoming messages sorted for the device the message is sent to
                    if m.header == SerialMsgHeader::SOF && m.data.len() >= 1 {
                        // push the message to the stack
                        self.messages.push(m.clone());
                    }
                }
            }
        }
    }

    /// Checks if the bit at the requested position is set
    fn get_bit_at(&self, input: u8, n: u8) -> bool {
        if n < 8 {
            input & (1 << n) != 0
        } else {
            false
        }
    }

    /// Return a copy the message stack
    pub fn get_messages(&self) -> Vec<SerialMsg> {
        self.messages.clone()
    }
}

impl Driver for SerialDriver {
    fn write<M>(&mut self, message: M) -> Result<u8, Error>
        where M: Into<Vec<u8>> {
        // read all messages to clean the driver pipe
        self.read_all_msg()?;

        // get the message from into
        let mut message = message.into();

        // Add the sent type to the message
        message.push(SerialTransmissionType::AutoRoute as u8);

        // get the next message id
        let m_id = self.get_next_msg_id();

        // add it to the message
        message.push(m_id);

        // generate the message
        let msg = SerialMsg::new(SerialMsgType::Request, SerialMsgFunction::SendData, message);

        // send the value
        self.port.write(msg.get_command().as_slice())?;

        // read the ACK accept package
        match self.read_single_msg_rty(&10) {
            // on error return it
            Err(e) => {
                return Err(e);
            },
            // check the message
            Ok(m) => {
                // when wrong header is received
                if m.header != SerialMsgHeader::ACK {
                    return Err(Error::new(ErrorKind::Io(StdErrorKind::InvalidData), "The driver refused the data - No ACK package"))
                }
            }
        }

        // read the driver accept
        match self.read_single_msg_rty(&10) {
            // on error return it
            Err(e) => {
                return Err(e);
            },
            // check the message
            Ok(m) => {
                // when wrong message is received
                if m.header != SerialMsgHeader::SOF || m.typ != SerialMsgType::Response || m.func != SerialMsgFunction::SendData || m.data != vec!(0x01u8) {
                    return Err(Error::new(ErrorKind::Io(StdErrorKind::InvalidData), "The driver refused the data - Negative response message"));
                }
            }
        }

        // return the message id
        Ok(m_id)
    }

    fn read(&mut self) -> Result<Vec<u8>, Error> {
        // read all messages to clean the driver pipe
        self.read_all_msg()?;

        // check if a message is available
        if self.messages.len() < 1 {
            return Err(Error::new(ErrorKind::Io(StdErrorKind::Other), "No message with the given id received"));
        }

        // return the first message
        Ok(self.messages.remove(0).data)
    }

    fn get_node_ids(&mut self) -> Result<Vec<u8>, Error> {
        // read all messages to clean the driver pipe
        self.read_all_msg()?;

        // create the serial message
        let msg = SerialMsg::new(SerialMsgType::Request, SerialMsgFunction::DiscoveryNodes, vec!());

        // send the value
        self.port.write(msg.get_command().as_slice())?;

        // check if the first message has the ACK answer
        match self.read_single_msg_rty(&5) {
            Err(e) => {
                return Err(e);
            },
            Ok(m) => {
                if m.header != SerialMsgHeader::ACK {
                    return Err(Error::new(ErrorKind::Io(StdErrorKind::InvalidData), "The driver refused the data - No ACK package"));
                }
            }
        }

        // read the second message and get the data
        let msg = self.read_single_msg_rty(&10)?;

        // grab the data
        let data = msg.data;

        // check if the data is long enough and if the right bit is set
        if data.len() != 34 || data[2] != 0x1D {
            return Err(Error::new(ErrorKind::UnknownZWave, "The ZWave message has a wrong format"));
        }

        // create the return variable
        let mut nodes = Vec::new();

        // loop over each bitmask byte
        for i in 3..31 {
            // loop over each bit of the byte
            for j in 0..7 {
                // check if the bit is set
                if self.get_bit_at(data[i], j) {
                    // calc the number out of the bitmask
                    let n = ((i-3) * 8) + (j as usize+1);
                    // add the node to the vector
                    nodes.push(n as u8);
                }
            }
        }

        //return the node ids
        Ok(nodes)
    }

    fn get_node_generic_class<N>(&mut self, node_id: N) -> Result<GenericType, Error>
        where N: Into<u8> {
        // read all messages to clean the driver pipe
        self.read_all_msg()?;

        // create the serial message
        let msg = SerialMsg::new(SerialMsgType::Request, SerialMsgFunction::GetNodeProtocolInfo, vec!(node_id.into()));

        // send the value
        self.port.write(msg.get_command().as_slice())?;

        // check if the first message has the ACK answer
        match self.read_single_msg_rty(&5) {
            Err(e) => {
                return Err(e);
            },
            Ok(m) => {
                if m.header != SerialMsgHeader::ACK {
                    return Err(Error::new(ErrorKind::Io(StdErrorKind::InvalidData), "The driver refused the data - No ACK package"));
                }
            }
        }

        // read the second message and get the data
        let msg = self.read_single_msg_rty(&10)?;

        // grab the data
        let data = msg.data;

        //check if the answer has the right length && is no error
        if data.len() != 6 {
            return Err(Error::new(ErrorKind::UnknownZWave, "The ZWave message has a wrong format"));
        }

        // extract the delivered type and return it
        Ok(GenericType::from_u8(data[4]).unwrap_or(GenericType::Unknown))
    }
}

impl fmt::Debug for SerialDriver {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Z-Wave Driver {{path: {}}}", self.path)
    }
}

#[derive(Debug, Clone)]
pub struct SerialMsg {
    pub header: SerialMsgHeader,
    pub typ: SerialMsgType,
    pub func: SerialMsgFunction,
    pub data: Vec<u8>
}

impl SerialMsg {
    /// create a new message
    pub fn new(typ: SerialMsgType, func: SerialMsgFunction, data: Vec<u8>) -> SerialMsg {
        SerialMsg {
            header: SerialMsgHeader::SOF,
            typ: typ,
            func: func,
            data: data
        }
    }

    // create a new message with only the header
    pub fn new_header(header: SerialMsgHeader) -> SerialMsg {
        SerialMsg {
            header: header,
            typ: SerialMsgType::Response,
            func: SerialMsgFunction::None,
            data: vec!()
        }
    }

    /// Parse a `&[u8]` slice and try to convert it to a `Message`
    pub fn parse(data: &[u8]) -> Result<SerialMsg, Error> {
        // check if the data has a header
        if data.len() < 1 {
            return Err(Error::new(ErrorKind::UnknownZWave, "No message delivered, at least a head is needed"));
        }

        // try to parse the header
        let header = SerialMsgHeader::from_u8(data[0]).ok_or(Error::new(ErrorKind::UnknownZWave, "Unknown ZWave header detected"))?;

        // return message if there is no start of frame header
        if header != SerialMsgHeader::SOF {
            return Ok(SerialMsg::new_header(header));
        }

        // check if the message is long enough for a SOF message
        if data.len() < 5 {
            return Err(Error::new(ErrorKind::UnknownZWave, "Data is too short for a ZWave message with SOF header"));
        }

        // check if the data is as long as the given length
        if data[1] != (data.len() - 2) as u8 {
            return Err(Error::new(ErrorKind::UnknownZWave, "The length of the message defined in the ZWave message didn't match with the actual length"));
        }

        // check if the checksum is right for the message
        if SerialMsg::checksum(&data[0 .. (data.len()-1)]) != data[data.len()-1] {
            return Err(Error::new(ErrorKind::UnknownZWave, "The checksum didn't match to the message"));
        }

        // try to parse the type
        let typ = SerialMsgType::from_u8(data[2]).ok_or(Error::new(ErrorKind::UnknownZWave, "Unknown message type"))?;

        // try to parse the function
        let function = SerialMsgFunction::from_u8(data[3]).ok_or(Error::new(ErrorKind::UnknownZWave, "Unknown ZWave function detected"))?;

        // create the message data array
        let msg_data : &[u8];
        // when there is data extract it
        if data.len() > 5 {
            msg_data = &data[4 .. (data.len()-1)];
        }
        // if not create a empty array
        else {
            msg_data = &[0; 0];
        }

        // create a new Message and return it
        Ok(SerialMsg::new(typ, function, msg_data.to_vec()))
    }

    /// return the command as Vec<u8>
    pub fn get_command(&self) -> Vec<u8> {
        // only create a full command if the header defines it
        if self.header != SerialMsgHeader::SOF {
            return vec![self.header as u8];
        }

        // create the header, length, typ and ZWave function
        let mut buf: Vec<u8> = vec![self.header as u8, (self.data.len()+3) as u8, self.typ as u8, self.func as u8];

        // append the data
        buf.append(&mut self.data.clone());

        // calc checksum
        let cs = SerialMsg::checksum(&buf);
        buf.push(cs);

        buf
    }

    /// Return a Vec<u8> into a String in a hex format.
    pub fn to_hex(data: &Vec<u8>) -> String {
        let mut out = String::new();

        for i in 0..data.len() {
            out.push_str(&*format!("{:#X} ", data[i]));
        }

        out
    }

    /// return the message as string in hex format
    pub fn get_hex(&self) -> String {
        SerialMsg::to_hex(&self.get_command())
    }

    /// Returns the checksum for the given vector
    pub fn checksum(data: &[u8]) -> u8 {
        let mut ret : u8 = 0xFF;

        for i in 1..data.len() {
            ret ^= data[i];
        }

        ret
    }
}

/// List of the ZWave start header
enum_from_primitive! {
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SerialMsgHeader {
    SOF = 0x01, // Start of Frame
    ACK = 0x06, // Message Accepted
    NAK = 0x15, // Message not Accepted
    CAN = 0x18, // Channel - Resend Request
}
}

/// List of different ZWave command types (rx/tx)
enum_from_primitive! {
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SerialMsgType {
    Request = 0x00,
    Response = 0x01,
}
}

/// List of different ZWave transmission types
enum_from_primitive! {
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SerialTransmissionType {
    ACK = 0x01,
    LowPower = 0x02,
    AutoRoute = 0x04,
    Explore = 0x20,
    Direct = 0x25,
}
}

/// List of all available ZWave functions
enum_from_primitive! {
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SerialMsgFunction {
    None = 0x00,
    DiscoveryNodes = 0x02,
    SerialApiApplNodeInformation = 0x03,
    ApplicationCommandHandler = 0x04,
    GetControllerCapabilities = 0x05,
    SerialApiSetTimeouts = 0x06,
    SerialGetCapabilities = 0x07,
    SerialApiSoftReset = 0x08,
    SetRFReceiveMode = 0x10,
    SetSleepMode = 0x11,
    SendNodeInformation = 0x12,
    SendData = 0x13,
    SendDataMulti = 0x14,
    GetVersion = 0x15,
    SendDataAbort = 0x16,
    RFPowerLevelSet = 0x17,
    SendDataMeta = 0x18,
    MemoryGetId = 0x20,
    MemoryGetByte = 0x21,
    MemoryPutByte = 0x22,
    MemoryGetBuffer = 0x23, // todo recheck
    MemoryPutBuffer = 0x24,
    // ReadMemory = 0x23, todo recheck
    ClockSet = 0x30,
    ClockGet = 0x31,
    ClockCompare = 0x32,
    RtcTimerCreate = 0x33,
    RtcTimerRead = 0x34,
    RtcTimerDelete = 0x35,
    RtcTimerCall = 0x36,
    GetNodeProtocolInfo = 0x41,
    SetDefault = 0x42,
    ReplicationCommandComplete = 0x44,
    ReplicationSendData = 0x45,
    AssignReturnRoute = 0x46,
    DeleteReturnRoute = 0x47,
    RequestNodeNeighborUpdate = 0x48,
    ApplicationUpdate = 0x49,
    AddNodeToNetwork = 0x4a,
    RemoveNodeFromNetwork = 0x4b,
    CreateNewPrimary = 0x4c,
    ControllerChange = 0x4d,
    SetLearnMode = 0x50,
    AssignSucReturnRoute = 0x51,
    EnableSuc = 0x52,
    RequestNetworkUpdate = 0x53,
    SetSucNodeId = 0x54,
    DeleteSucReturnRoute = 0x55,
    GetSucNodeId = 0x56,
    SendSucId = 0x57,
    RediscoveryNeeded = 0x59,
    RequestNodeInfo = 0x60,
    RemoveFailedNodeId = 0x61,
    IsFailedNode = 0x62,
    ReplaceFailedNode = 0x63,
    TimerStart = 0x70,
    TimerRestart = 0x71,
    TimerCancel = 0x72,
    TimerCall = 0x73,
    GetRoutingTableLine = 0x80,
    GetTXCounter = 0x81,
    ResetTXCounter = 0x82,
    StoreNodeInfo = 0x83,
    StoreHomeId = 0x84,
    LockRouteResponse = 0x90,
    SendDataRouteDemo = 0x91,
    SerialApiTest = 0x95,
    SerialApiSlaveNodeInfo = 0xa0,
    ApplicationSlaveCommandHandler = 0xa1,
    SendSlaveNodeInfo = 0xa2,
    SendSlaveData = 0xa3,
    SetSlaveLearnMode = 0xa4,
    GetVirtualNodes = 0xa5,
    IsVirtualNode = 0xa6,
    SetPromiscuousMode = 0xd0,
}}
