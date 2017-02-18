use cmds::{CmdClass, Message};
use num::FromPrimitive;
use error::{Error, ErrorKind};
use driver::GenericType;

/// List of the Info class functions
enum_from_primitive! {
#[derive(Copy, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Function {
    Get = 0x02,
}
}

/// Generate the message for the basic Command Class with
/// the function to get a value.
pub fn get(node_id: u8) -> Message {
    Message::new(node_id, CmdClass::NODE_INFO, Function::Get as u8, vec!())
}

/// Read a the Node_Information message and parse it to the type and command
/// class types.
pub fn parse<M>(msg: M) -> Result<(Vec<GenericType>, Vec<CmdClass>), Error>
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

            // when the device is unkown continue
            if m == CmdClass::NO_OPERATION as u8 {
                continue;
            }

            // try to convert the command
            match CmdClass::from_u8(m.clone()) {
                // When the command is known push it to the vec
                Some(t) => {
                    cmds.push(t);
                },
                // When the command is unknown, just continue
                None => {
                    continue;
                }
            }
        }

        // return the result
        Ok((types, cmds))
}
