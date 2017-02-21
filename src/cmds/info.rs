use cmds::{CommandClass, Message};
use num::FromPrimitive;
use error::{Error, ErrorKind};
use driver::GenericType;


#[derive(Debug, Clone)]
pub struct NodeInfo;

impl NodeInfo {
    /// Generate the message for the basic Command Class with
    /// the function to get a value.
    pub fn get(node_id: u8) -> Message {
        Message::new(node_id, CommandClass::NODE_INFO, 0x02, vec!())
    }

    /// Read a the Node_Information message and parse it to the type and command
    /// class types.
    pub fn report<M>(msg: M) -> Result<(Vec<GenericType>, Vec<CommandClass>), Error>
        where M: Into<Vec<u8>> {
            // get the message
            let msg = msg.into();
            let mut types = vec!();
            let mut cmds = vec!();

            // extractthe types
            for i in 2..6 {
                // get the type fro the vector
                let m = msg.get(i as usize).ok_or(Error::new(ErrorKind::UnknownZWave, "Message is to short"))?;
                let m = m.clone();

                // when the device is unkown continue
                if m == GenericType::Unknown as u8 {
                    continue;
                }

                // try to convert the type
                match GenericType::from_u8(m) {
                    // When the type is known push it to the vec
                    Some(t) => {
                        types.push(t);
                    },
                    // When the type is unknown, just continue
                    None => {
                        continue;
                    }
                }
            }

            // extract the command classes
            for i in 6..msg.len() {
                // get the command for the vector
                let m = msg.get(i as usize).ok_or(Error::new(ErrorKind::UnknownZWave, "Message is to short"))?;
                let m = m.clone();

                // try to convert the command
                let cmd = CommandClass::from_u8(m.clone()).unwrap_or(CommandClass::NO_OPERATION);

                // when the device is unkown continue
                if cmd == CommandClass::NO_OPERATION {
                    continue;
                }

                // When the command is known push it to the vec
                cmds.push(cmd);
            }

            // return the result
            Ok((types, cmds))
    }
}
