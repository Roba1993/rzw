//! The message represent a ZWave message which can be sent or received.
//! To build up such a message use the following implementation.
//!
//! ```rust
//! Message::new(0x02, CmdClass::BASIC, 0x01, vec!(0xFF));
//! ```
//!
//! The structure of a ZWave message looks like the following:
//!
//! `device, data-length, comand class, command, value`

use num::FromPrimitive;
use error::{Error, ErrorKind};

#[derive(Debug, Clone)]
pub struct Message {
    pub node_id: u8,
    pub cmd_class: CmdClass,
    pub cmd: u8,
    pub data: Vec<u8>
}

impl Message {
    /// create a new message
    pub fn new(node_id: u8, cmd_class: CmdClass, cmd: u8, data: Vec<u8>) -> Message {
        Message {
            node_id: node_id,
            cmd_class: cmd_class,
            cmd: cmd,
            data: data
        }
    }

    /// Parse a `&[u8]` slice and try to convert it to a `Message`
    pub fn parse(data: &[u8]) -> Result<Message, Error> {
        // check if the data is avilable
        if data.len() < 1 {
            return Err(Error::new(ErrorKind::UnknownZWave, "Message has no data"));
        }

        // check if the data has enough entries
        if data.len() < 4 {
            return Err(Error::new(ErrorKind::UnknownZWave, "Message is to short"));
        }

        // check if the length flag matches
        if data.len() + 2 != data[1] as usize {
            return Err(Error::new(ErrorKind::UnknownZWave, "The length of the message delivered didn't match with the actual length"));
        }

        // get the node id
        let node_id = data[0];

        // get the commadn class
        let cmd_class = CmdClass::from_u8(data[2]).ok_or(Error::new(ErrorKind::UnknownZWave, "The ZWave Command Class is unknown"))?;

        // get the command
        let cmd = data[3];

        // create the message data array
        let msg_data : &[u8];
        // when there is data extract it
        if data.len() > 4 {
            msg_data = &data[3 .. (data.len()-1)];
        }
        // if not create a empty array
        else {
            msg_data = &[0; 0];
        }

        // create a new Message and return it
        Ok(Message::new(node_id, cmd_class, cmd, msg_data.to_vec()))
    }

    /// Return the message as Vec<u8>
    pub fn to_vec(&self) -> Vec<u8> {
        // todo check if there a better way
        let mut v : Vec<u8> = Vec::new();
        v.push(self.node_id);
        v.push((self.data.len()+2) as u8);
        v.push(self.cmd_class as u8);
        v.push(self.cmd);
        v.append(&mut self.data.clone());
        v
    }

    /// Return the message as hex formated string.
    pub fn to_hex_string(&self) -> String {
        let data = self.to_vec();
        let mut out = String::new();

        for i in 0..data.len() {
            out.push_str(&*format!("{:#X} ", data[i]));
        }

        out
    }
}

/// List of the ZWave Command Classes
enum_from_primitive! {
#[derive(Copy, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum CmdClass {
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
}
