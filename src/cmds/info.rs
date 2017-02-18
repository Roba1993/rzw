use cmds::CmdClass;
use cmds::Message;


/// Generate the message for the basic Command Class with
/// the function to get a value.
pub fn get(node_id: u8) -> Message {
    Message::new(node_id, CmdClass::REQUEST_NODE_INFO, 0x00, vec!())
}
