use cmds::{CommandClass, Message};
use error::{Error, ErrorKind};

#[derive(Debug, Clone)]
pub struct Basic;

impl Basic {
    /// Generate the message for the basic Command Class with
    /// the function to set a value.
    pub fn set(node_id: u8, value: u8) -> Message {
        Message::new(node_id, CommandClass::BASIC, 0x01, vec![value])
    }

    /// Generate the message for the basic Command Class with
    /// the function to get a value.
    pub fn get(node_id: u8) -> Message {
        Message::new(node_id, CommandClass::BASIC, 0x02, vec![])
    }

    /// Returns the basic node value
    pub fn report<M>(msg: M) -> Result<u8, Error>
    where
        M: Into<Vec<u8>>,
    {
        // get the message
        let msg = msg.into();

        // the message need to be exact 6 digits long
        if msg.len() != 6 {
            return Err(Error::new(ErrorKind::UnknownZWave, "Message is too short"));
        }

        // check the CommandClass and command
        if msg[3] != CommandClass::BASIC as u8 || msg[4] != 0x03 {
            return Err(Error::new(
                ErrorKind::UnknownZWave,
                "Answer contained wrong command class",
            ));
        }

        // return the value
        Ok(msg[5])
    }
}
