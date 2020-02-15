/// A SerialMessage which can be sent and received over a Driver
#[derive(Debug, Clone)]
pub struct SerialMessage {
    pub header: SerialMessageHeader,
    pub typ: SerialMessageType,
    pub func: SerialMessageFunction,
    pub data: Vec<u8>,
}

impl SerialMessage {
    /// create a new message
    pub fn new(typ: SerialMessageType, func: SerialMessageFunction, data: Vec<u8>) -> Self {
        SerialMessage {
            header: SerialMessageHeader::SOF,
            typ: typ,
            func: func,
            data: data,
        }
    }

    // create a new message with only the header
    pub fn new_header(header: SerialMessageHeader) -> Self {
        SerialMessage {
            header: header,
            typ: SerialMessageType::Response,
            func: SerialMessageFunction::None,
            data: vec![],
        }
    }

    /// Parse a `&[u8]` slice and try to convert it to a `Message`
    pub fn parse(data: &[u8]) -> Result<SerialMessage, crate::error::Error> {
        use std::convert::TryFrom;

        // check if the data has a header
        if data.len() < 1 {
            return Err(crate::error::Error::new(
                crate::error::ErrorKind::UnknownZWave,
                "No message delivered, at least a head is needed",
            ));
        }

        // try to parse the header
        let header = SerialMessageHeader::try_from(data[0])?;

        // return message if there is no start of frame header
        if header != SerialMessageHeader::SOF {
            return Ok(SerialMessage::new_header(header));
        }

        // check if the message is long enough for a SOF message
        if data.len() < 5 {
            return Err(crate::error::Error::new(
                crate::error::ErrorKind::UnknownZWave,
                "Data is too short for a ZWave message with SOF header",
            ));
        }

        // check if the data is as long as the given length
        if data[1] != (data.len() - 2) as u8 {
            return Err(crate::error::Error::new(
                crate::error::ErrorKind::UnknownZWave, 
                "The length of the message defined in the ZWave message didn't match with the actual length"));
        }

        
        // check if the checksum is right for the message
        if SerialMessage::checksum(&data[0..(data.len() - 1)]) != data[data.len() - 1] {
            return Err(crate::error::Error::new(
                crate::error::ErrorKind::UnknownZWave,
                "The checksum didn't match to the message",
            ));
        }

        
        // try to parse the type
        let typ = SerialMessageType::try_from(data[2])?;

        // try to parse the function
        let function = SerialMessageFunction::try_from(data[3]).map_err(|_| crate::error::Error::new(
            crate::error::ErrorKind::UnknownZWave,
            "Unknown ZWave function detected",
        ))?;

        // create the message data array
        let msg_data: &[u8];
        // when there is data extract it
        if data.len() > 5 {
            msg_data = &data[4..(data.len() - 1)];
        }
        // if not create a empty array
        else {
            msg_data = &[0; 0];
        }

        // create a new Message and return it
        Ok(SerialMessage::new(typ, function, msg_data.to_vec()))
    }

    /// return the command as Vec<u8>
    pub fn get_command(&self) -> Vec<u8> {
        // only create a full command if the header defines it
        if self.header != SerialMessageHeader::SOF {
            return vec![self.header as u8];
        }

        // create the header, length, typ and ZWave function
        let mut buf: Vec<u8> = vec![
            self.header as u8,
            (self.data.len() + 3) as u8,
            self.typ as u8,
            self.func as u8,
        ];

        // append the data
        buf.append(&mut self.data.clone());

        // calc checksum
        let cs = SerialMessage::checksum(&buf);
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

    /// Returns the checksum for the given vector
    pub fn checksum(data: &[u8]) -> u8 {
        let mut ret: u8 = 0xFF;

        for i in 1..data.len() {
            ret ^= data[i];
        }

        ret
    }
}

/// List of the ZWave start header
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum SerialMessageHeader {
    SOF = 0x01, // Start of Frame
    ACK = 0x06, // Message Accepted
    NAK = 0x15, // Message not Accepted
    CAN = 0x18, // Channel - Resend Request
}

impl std::convert::TryFrom<u8> for SerialMessageHeader {
    type Error = crate::error::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(SerialMessageHeader::SOF),
            0x06 => Ok(SerialMessageHeader::ACK),
            0x15 => Ok(SerialMessageHeader::NAK),
            0x18 => Ok(SerialMessageHeader::CAN),
            _ => Err(crate::error::Error::new(
                crate::error::ErrorKind::Io(std::io::ErrorKind::InvalidData),
                "Can't convert to Serial Message Header",
            )),
        }
    }
}

/// List of different ZWave command types (rx/tx)
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum SerialMessageType {
    Request = 0x00,
    Response = 0x01,
}

impl std::convert::TryFrom<u8> for SerialMessageType {
    type Error = crate::error::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(SerialMessageType::Request),
            0x01 => Ok(SerialMessageType::Response),
            _ => Err(crate::error::Error::new(
                crate::error::ErrorKind::Io(std::io::ErrorKind::InvalidData),
                "Can't convert to Serial Message Type",
            )),
        }
    }
}

/// List of different ZWave transmission types
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum SerialTransmissionType {
    ACK = 0x01,
    LowPower = 0x02,
    AutoRoute = 0x04,
    Explore = 0x20,
    Direct = 0x25,
}

/// List of all available ZWave functions
#[derive(Copy, Clone, Debug, PartialEq, num_enum::TryFromPrimitive)]
#[repr(u8)]
pub enum SerialMessageFunction {
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

/// List of the ZWave Command Classes
#[derive(Copy, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum CommandClass {
    NO_OPERATION = 0x00,
    NODE_INFO = 0x01,
    REQUEST_NODE_INFO = 0x02,
    ASSIGN_IDS = 0x03,
    FIND_NODES_IN_RANGE = 0x04,
    GET_NODES_IN_RANGE = 0x05,
    RANGE_INFO = 0x06,
    CMD_COMPLETE = 0x07,
    TRANSFER_PRESENTATION = 0x08,
    TRANSFER_NODE_INFO = 0x09,
    TRANSFER_RANGE_INFO = 0x0A,
    TRANSFER_END = 0x0B,
    ASSIGN_RETURN_ROUTE = 0x0C,
    NEW_NODE_REGISTERED = 0x0D,
    NEW_RANGE_REGISTERED = 0x0E,
    TRANSFER_NEW_PRIMARY_COMPLETE = 0x0F,
    AUTOMATIC_CONTROLLER_UPDATE_START = 0x10,
    SUC_NODE_ID = 0x11,
    SET_SUC = 0x12,
    SET_SUC_ACK = 0x13,
    ASSIGN_SUC_RETURN_ROUTE = 0x14,
    STATIC_ROUTE_REQUEST = 0x15,
    LOST = 0x16,
    ACCEPT_LOST = 0x17,
    NOP_POWER = 0x18,
    RESERVE_NODE_IDS = 0x19,
    RESERVED_IDS = 0x1A,
    // Unknown
    BASIC = 0x20,
    CONTROLLER_REPLICATION = 0x21,
    APPLICATION_STATUS = 0x22,
    ZIP_SERVICES = 0x23,
    ZIP_SERVER = 0x24,
    SWITCH_BINARY = 0x25,
    SWITCH_MULTILEVEL = 0x26,
    SWITCH_ALL = 0x27,
    SWITCH_TOGGLE_BINARY = 0x28,
    SWITCH_TOGGLE_MULTILEVEL = 0x29,
    CHIMNEY_FAN = 0x2A,
    SCENE_ACTIVATION = 0x2B,
    SCENE_ACTUATOR_CONF = 0x2C,
    SCENE_CONTROLLER_CONF = 0x2D,
    ZIP_CLIENT = 0x2E,
    ZIP_ADV_SERVICES = 0x2F,
    SENSOR_BINARY = 0x30,
    SENSOR_MULTILEVEL = 0x31,
    METER = 0x32,
    ZIP_ADV_SERVER = 0x33,
    ZIP_ADV_CLIENT = 0x34,
    METER_PULSE = 0x35,
    METER_TBL_CONFIG = 0x3C,
    METER_TBL_MONITOR = 0x3D,
    METER_TBL_PUSH = 0x3E,
    THERMOSTAT_HEATING = 0x38,
    THERMOSTAT_MODE = 0x40,
    THERMOSTAT_OPERATING_STATE = 0x42,
    THERMOSTAT_SETPOINT = 0x43,
    THERMOSTAT_FAN_MODE = 0x44,
    THERMOSTAT_FAN_STATE = 0x45,
    CLIMATE_CONTROL_SCHEDULE = 0x46,
    THERMOSTAT_SETBACK = 0x47,
    TARIF_CONFIG = 0x4A,
    TARIF_TABLE_MONITOR = 0x4B,
    COMMAND_CLASS_DOOR_LOCK_LOGGING = 0x4C,
    SCHEDULE_ENTRY_LOCK = 0x4E,
    ZIP_6LOWPAN = 0x4F,
    BASIC_WINDOW_COVERING = 0x50,
    MTP_WINDOW_COVERING = 0x51,
    MULTI_INSTANCE = 0x60,
    DOOR_LOCK = 0x62,
    USER_CODE = 0x63,
    CONFIGURATION = 0x70,
    ALARM = 0x71,
    MANUFACTURER_SPECIFIC = 0x72,
    POWER_LEVEL = 0x73,
    PROTECTION = 0x75,
    LOCK = 0x76,
    NODE_NAMING = 0x77,
    FIRMWARE_UPDATE_MD = 0x7A,
    GROUPING_NAME = 0x7B,
    REMOTE_ASSOCIATION_ACTIVATE = 0x7C,
    REMOTE_ASSOCIATION = 0x7D,
    BATTERY = 0x80,
    CLOCK = 0x81,
    HAIL = 0x82,
    WAKE_UP = 0x84,
    ASSOCIATION = 0x85,
    VERSION = 0x86,
    INDICATOR = 0x87,
    PROPRIETARY = 0x88,
    LANGUAGE = 0x89,
    TIME = 0x8A,
    TIME_PARAMETERS = 0x8B,
    GEOGRAPHIC_LOCATION = 0x8C,
    COMPOSITE = 0x8D,
    MULTI_INSTANCE_ASSOCIATION = 0x8E,
    MULTI_CMD = 0x8F,
    ENERGY_PRODUCTION = 0x90,
    MANUFACTURER_PROPRIETARY = 0x91,
    SCREEN_MD = 0x92,
    SCREEN_ATTRIBUTES = 0x93,
    SIMPLE_AV_CONTROL = 0x94,
    AV_CONTENT_DIRECTORY_MD = 0x95,
    AV_RENDERER_STATUS = 0x96,
    AV_CONTENT_SEARCH_MD = 0x97,
    SECURITY = 0x98,
    AV_TAGGING_MD = 0x99,
    IP_CONFIGURATION = 0x9A,
    ASSOCIATION_COMMAND_CONFIGURATION = 0x9B,
    SENSOR_ALARM = 0x9C,
    SILENCE_ALARM = 0x9D,
    SENSOR_CONFIGURATION = 0x9E,
    MARK = 0xEF,
    NON_INTEROPERABLE = 0xF0,
}

/// List of the generic node types
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum GenericType {
    Unknown = 0x00,
    RemoteController = 0x01,
    StaticController = 0x02,
    AvControlPoint = 0x03,
    RoutingSlave = 0x04,
    Display = 0x06,
    GarageDoor = 0x07,
    WindowCovering = 0x09,
    Thermostat = 0x08,
    RepeaterSlave = 0x0F,
    BinarySwitch = 0x10,
    MultiLevelSwitch = 0x11,
    RemoteSwitch = 0x12,
    ToggleSwitch = 0x13,
    ZIpGateway = 0x14,
    ZIpNode = 0x15,
    Ventilation = 0x16,
    GenericSecurityPanel = 0x17,
    RemoteSwitch2 = 0x18,
    BinarySensor = 0x20,
    MultilevelSensor = 0x21,
    WaterControl = 0x22,
    PulseMeter = 0x30,
    Meter = 0x31,
    EntryControl = 0x40,
    SemiInteroperable = 0x50,
    AlarmSensor = 0xa1,
    NonInteroperable = 0xFF,
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum MeterData {
    Electric_kWh(f64),
    Electric_kVAh(f64),
    Electric_W(f64),
    Electric_PulseCount(f64),
    Gas_meter2(f64),
    Gas_feet2(f64),
    Gas_PulseCount(f64),
    Water_meter2(f64),
    Water_feet2(f64),
    Water_Gallons(f64),
    Water_PulseCount(f64),
}

impl MeterData {
    pub fn get_scale(&self) -> u8 {
        match *self {
            MeterData::Electric_kWh(_) => 0x00,
            MeterData::Electric_kVAh(_) => 0x01,
            MeterData::Electric_W(_) => 0x02,
            MeterData::Electric_PulseCount(_) => 0x03,
            MeterData::Gas_meter2(_) => 0x00,
            MeterData::Gas_feet2(_) => 0x01,
            MeterData::Gas_PulseCount(_) => 0x03,
            MeterData::Water_meter2(_) => 0x00,
            MeterData::Water_feet2(_) => 0x01,
            MeterData::Water_Gallons(_) => 0x02,
            MeterData::Water_PulseCount(_) => 0x03,
        }
    }
}
