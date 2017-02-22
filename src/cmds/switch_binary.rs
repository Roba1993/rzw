use cmds::{CommandClass, Message};
use error::{Error, ErrorKind};

/// The Binary Switch Command Class is used to control devices with On/Off
/// or Enable/Disable capability.
#[derive(Debug, Clone)]
pub struct SwitchBinary;


impl SwitchBinary {
    /// The Binary Switch Set command, version 1 is used to set a binary value.
    pub fn set<N, V>(node_id: N, value: V) -> Message
    where N: Into<u8>, V: Into<bool> {
        // Convert the boolean to a u8
        let value = if value.into() {0xFF} else {0x00};

        // generate the message
        Message::new(node_id.into(), CommandClass::SWITCH_BINARY, 0x01, vec!(value))
    }


    /// The Binary Switch Get command, version 1 is used to request the status
    /// of a device with On/Off or Enable/Disable capability.
    pub fn get<N>(node_id: N) -> Message
    where N: Into<u8> {
        Message::new(node_id.into(), CommandClass::SWITCH_BINARY, 0x02, vec!())
    }


    /// The Binary Switch Report command, version 1 is used to advertise the
    /// status of a device with On/Off or Enable/Disable capability.
    pub fn report<M>(msg: M) -> Result<bool, Error>
    where M: Into<Vec<u8>> {
        // get the message
        let msg = msg.into();

        // the message need to be exact 6 digits long
        if msg.len() != 6 {
            return Err(Error::new(ErrorKind::UnknownZWave, "Message is to short"));
        }

        // check the CommandClass and command
        if msg[3] != CommandClass::SWITCH_BINARY as u8 || msg[4] != 0x03 {
            return Err(Error::new(ErrorKind::UnknownZWave, "Answer contained wrong command class"));
        }

        let val = if msg[5] < 0xFF {false} else {true};

        // return the value
        Ok(val)
    }
}
