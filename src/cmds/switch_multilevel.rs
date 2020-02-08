use cmds::{CommandClass, Message};
use error::{Error, ErrorKind};

/// The Multilevel Switch Command Class is used to control devices with variable levels
/// such as dimmer switches
#[derive(Debug, Clone)]
pub struct SwitchMultilevel;

impl SwitchMultilevel {
    /// The Multilevel Switch Set command, version 1 is used to set a u8 value.
    pub fn set<N, V>(node_id: N, value: V) -> Message
    where
        N: Into<u8>,
        V: Into<u8>,
    {
        // generate the message
        Message::new(
            node_id.into(),
            CommandClass::SWITCH_MULTILEVEL,
            0x01,
            vec![value.into()],
        )
    }

    /// The Multilevel Switch Get command, version 1 is used to request the status
    /// of a device with variable levels capability.
    pub fn get<N>(node_id: N) -> Message
    where
        N: Into<u8>,
    {
        Message::new(
            node_id.into(),
            CommandClass::SWITCH_MULTILEVEL,
            0x02,
            vec![],
        )
    }

    /// The Multilevel Switch Report command, version 1 is used to advertise the
    /// status of a device with variable levels capability.
    pub fn report<M>(msg: M) -> Result<u8, Error>
    where
        M: Into<Vec<u8>>,
    {
        // get the message
        let msg = msg.into();

        // the message need to be at least 6 digits long. Version 4 may return
        // more data, but not currently supported. 
        if msg.len() < 6 {
            return Err(Error::new(ErrorKind::UnknownZWave, "Message is too short"));
        }

        // check the CommandClass and command
        if msg[3] != CommandClass::SWITCH_MULTILEVEL as u8 || msg[4] != 0x03 {
            return Err(Error::new(
                ErrorKind::UnknownZWave,
                "Answer contained wrong command class",
            ));
        }

        let val = msg[5];

        // return the value
        Ok(val)
    }
}
