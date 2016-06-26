
// ZWave generale message
// `header, length, type(rx|tx), zw-function, data, checksum`
//
// ZWave data structure for basic
// `device, ?, comand class, command, value, ?``

use num::FromPrimitive;
use error::{Error, ErrorKind};

#[derive(Debug, Clone)]
pub struct Message {
    pub header: Header,
    pub typ: Type,
    pub func: Function,
    pub data: Vec<u8>
}

impl Message {
    /// create a new message
    pub fn new(typ: Type, func: Function, data: Vec<u8>) -> Message {
        Message {
            header: Header::SOF,
            typ: typ,
            func: func,
            data: data
        }
    }

    // create a new message with only the header
    pub fn new_header(header: Header) -> Message {
        Message {
            header: header,
            typ: Type::Response,
            func: Function::None,
            data: vec!()
        }
    }

    /// Parse a `&[u8]` slice and try to convert it to a `Message`
    pub fn parse(data: &[u8]) -> Result<Message, Error> {
        // check if the data has a header
        if data.len() < 1 {
            return Err(Error::new(ErrorKind::UnknownZWave, "No message delivered, at least a head is needed"));
        }

        // try to parse the header
        let header = unwrap_or_return!(Header::from_u8(data[0]), Err(Error::new(ErrorKind::UnknownZWave, "Unknown ZWave header detected")));

        // return message if there is no start of frame header
        if header != Header::SOF {
            return Ok(Message::new_header(header));
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
        if Message::checksum(&data[0 .. (data.len()-1)]) != data[data.len()-1] {
            return Err(Error::new(ErrorKind::UnknownZWave, "The checksum didn't match to the message"));
        }

        // try to parse the type
        let typ = unwrap_or_return!(Type::from_u8(data[2]), Err(Error::new(ErrorKind::UnknownZWave, "Unknown message type")));

        // try to parse the function
        let function = unwrap_or_return!(Function::from_u8(data[3]), Err(Error::new(ErrorKind::UnknownZWave, "Unknown ZWave function detected")));

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
        Ok(Message::new(typ, function, msg_data.to_vec()))
    }

    /// return the command as Vec<u8>
    pub fn get_command(&self) -> Vec<u8> {
        // only create a full command if the header defines it
        if self.header != Header::SOF {
            return vec![self.header as u8];
        }

        // create the header, length, typ and ZWave function
        let mut buf: Vec<u8> = vec![self.header as u8, (self.data.len()+3) as u8, self.typ as u8, self.func as u8];

        // append the data
        buf.append(&mut self.data.clone());

        // calc checksum
        let cs = Message::checksum(&buf);
        buf.push(cs);

        buf
    }

    /// return the message as string in hex format
    pub fn get_hex(&self) -> String {
        let dat = self.get_command();
        let mut out = String::new();

        for i in 0..dat.len() {
            out.push_str(&*format!("{:#X} ", dat[i]));
        }

        out
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
pub enum Header {
    SOF = 0x01, // Start of Frame
    ACK = 0x06, // Message Accepted
    NAK = 0x15, // Message not Accepted
    CAN = 0x18, // Channel - Resend Request
}
}

/// List of different ZWave command types (rx/tx)
enum_from_primitive! {
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Type {
    Request = 0x00,
    Response = 0x01,
}
}

/// List of all available ZWave functions
enum_from_primitive! {
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Function {
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
}
}

#[cfg(test)]
mod tests {
    use msg::Message;
    use error::Error;

    /// Test the parsing functionality
    #[test]
    fn msg_parse() {
        assert!(Message::parse(&[0; 0]).is_err());
        assert!(Message::parse(&[0xFF]).is_err());
        assert!(Message::parse(&[0x01]).is_err());
        assert!(Message::parse(&[0x1, 0x2, 0x0, 0xFD]).is_err());
        assert!(Message::parse(&[0x1, 0x7, 0x0, 0x13, 0x2, 0x3, 0x20, 0x1, 0x0, 0xC4]).is_err());
        assert!(Message::parse(&[0x1, 0x8, 0x0, 0x13, 0x2, 0x3, 0x20, 0x1, 0x0, 0xC5]).is_err());

        assert!(Message::parse(&[0x06]).is_ok());
        assert!(Message::parse(&[0x01, 0x03, 0x00, 0x13, 0xEF]).is_ok());
        assert!(Message::parse(&[0x1, 0x04, 0x01, 0x13, 0x01, 0xE8]).is_ok());
        assert!(Message::parse(&[0x1, 0x8, 0x0, 0x13, 0x2, 0x3, 0x20, 0x1, 0x0, 0xC4]).is_ok());
    }

    /// Test for the checksum functionality
    #[test]
    fn checksum() {
        assert!(Message::checksum(&[0x01, 0x03, 0x20]) == 0xDC);
        assert!(Message::checksum(&[0x1, 0x8, 0x0, 0x13, 0x2, 0x3, 0x20, 0x1, 0x0]) == 0xC4);
    }
}
