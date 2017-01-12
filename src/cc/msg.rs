//! ZWave message to write and read
//!
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
use cc::CmdClass;

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
        if data.len() < 3 {
            return Err(Error::new(ErrorKind::UnknownZWave, "Message is to short"));
        }

        // check if the length flag matches
        if data.len() - 2 != data[1] as usize {
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
            msg_data = &data[4 .. (data.len())];
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
