use cc::CmdClass;
use cc::msg::Message;

/// List of the Basic class functions
enum_from_primitive! {
#[derive(Copy, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Function {
    Set = 0x01,
    Get = 0x02,
    Report = 0x03,
}
}

/// Generate the message for the basic Command Class with
/// the function to set a value.
pub fn set(node_id: u8, value: u8) -> Message {
    Message::new(node_id, CmdClass::BASIC, Function::Set as u8, vec!(value))
}

/// Generate the message for the basic Command Class with
/// the function to get a value.
pub fn get(node_id: u8) -> Message {
    Message::new(node_id, CmdClass::BASIC, Function::Get as u8, vec!())
}
