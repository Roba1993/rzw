use cmds::{CommandClass, Message};


#[derive(Debug, Clone)]
pub struct Basic;

impl Basic {
    /// Generate the message for the basic Command Class with
    /// the function to set a value.
    pub fn set(&self, node_id: u8, value: u8) -> Message {
        Message::new(node_id, CommandClass::Basic(self.clone()), 0x01, vec!(value))
    }

    /// Generate the message for the basic Command Class with
    /// the function to get a value.
    pub fn get(&self, node_id: u8) -> Message {
        Message::new(node_id, CommandClass::Basic(self.clone()), 0x02, vec!())
    }
}
