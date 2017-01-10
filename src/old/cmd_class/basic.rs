use std::rc::Rc;
use std::cell::RefCell;
use msg::{Message, Type, Function};
use node::Node;
use error::Error;

// The private representation of the zwave basic class
struct _Basic {
    node: Node
}

// reference type for the zwave basic class
type BasicRef = Rc<RefCell<_Basic>>;

// The public representation of a zwave basic class, with some syntactic sugar.
#[derive(Clone)]
pub struct Basic(BasicRef);

/// The actual zwave basic class implementation
impl Basic {
    /// Creates a new zwave basic class
    pub fn new(node: Node) -> Basic {
        let basic = _Basic {
            node: node
        };

        Basic(Rc::new(RefCell::new(basic)))
    }

    /// set a value on the node
    pub fn set(&self, value: u8) -> Result<Message, Error> {
        // get the id of the node
        let node = (self.0.borrow()).node.get_id();

        // create a new message
        let msg = Message::new(Type::Request, Function::SendData, vec!(node, 0x03, 0x20, 0x01, value, 0x66));

        // send the message to the ZWave driver
        (self.0.borrow()).node.get_controller().get_driver().write_and_read(msg)
    }

    /// get a value on the node
    pub fn get(&self) -> Result<Message, Error> {
        // get the id of the node
        let node = (self.0.borrow()).node.get_id();

        // create a new message
        let msg = Message::new(Type::Request, Function::SendData, vec!(node, 0x03, 0x20, 0x02));

        // send the message to the ZWave driver
        (self.0.borrow()).node.get_controller().get_driver().write_and_read(msg)
    }
}
