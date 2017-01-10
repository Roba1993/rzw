use controller::Controller;
use cmd_class::basic::Basic;
use std::rc::Rc;
use std::cell::RefCell;
use num::FromPrimitive;
use msg::{Message, Type, Function};

// The private representation of a node
struct _Node {
    id: u8,
    controller: Controller,
    generic_type: GenericType,
    class_basic: Option<Basic>
}

// reference type for the node
type NodeRef = Rc<RefCell<_Node>>;

// The public representation of a node, with some syntactic sugar.
#[derive(Clone)]
pub struct Node(NodeRef);

// The actual node implementation
impl Node {
    /// Creates a new node with no edges.
    pub fn new(contr: Controller, id: u8) -> Node {
        let node = Node(Rc::new(RefCell::new(_Node {
            id: id,
            controller: contr,
            generic_type: GenericType::Unknown,
            class_basic: None
        })));

        node.discover_type();
        node.discover_classes();

        node
    }

    /// Sets the available type for this node
    pub fn discover_type(&self) {
        let mut this = &mut self.0.borrow_mut();

        // create a new message
        let msg = Message::new(Type::Request, Function::GetNodeProtocolInfo, vec!(this.id));

        // send the message to the ZWave driver
        let msg = unwrap_or_return!(this.controller.get_driver().write_and_read(msg).ok(), ());

        //check if the answer has the right length && is no error
        if msg.data.len() != 6 {
            this.generic_type = GenericType::Unknown;
            return;
        }

        // extract the delivered type
        this.generic_type = GenericType::from_u8(msg.data[4]).unwrap_or(GenericType::Unknown);
    }

    /// Sets the available function classes for this node
    pub fn discover_classes(&self) {
        let mut this = &mut self.0.borrow_mut();

        // todo get the information from the device

        // basic is always available
        this.class_basic = Some(Basic::new(self.clone()));
    }

    /// returns the controller
    pub fn get_controller(&self) -> Controller {
        self.0.borrow().controller.clone()
    }

    /// returns the basic command class to interact with
    pub fn get_basic(&self) -> Option<Basic> {
        self.0.borrow_mut().class_basic.clone()
    }

    /// returns the id of the node in the zwave network
    pub fn get_id(&self) -> u8 {
        self.0.borrow().id.clone()
    }

    // returns the generic type of the node
    pub fn get_generic_type(&self) -> GenericType {
        self.0.borrow().generic_type.clone()
    }
}


/// List of different Node types
enum_from_primitive! {
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GenericType {
    Unknown = 0x00,
    RemoteController = 0x01,
    StaticController = 0x02,
    AvControlPoint = 0x03,
    RoutingSlave = 0x04,
    Display = 0x06,
    GarageDoor = 0x07,
    WindowCovering = 0x09,
    Thermostat = 0x08,
    RepeaterSlave = 0x0F,
    BinarySwitch = 0x10,
    MultiLevelSwitch = 0x11,
    RemoteSwitch = 0x12,
    ToggleSwitch = 0x13,
    ZIpGateway = 0x14,
    ZIpNode = 0x15,
    Ventilation = 0x16,
    GenericSecurityPanel = 0x17,
    RemoteSwitch2 = 0x18,
    BinarySensor = 0x20,
    MultilevelSensor = 0x21,
    WaterControl = 0x22,
    PulseMeter = 0x30,
    Meter = 0x31,
    EntryControl = 0x40,
    SemiInteroperable = 0x50,
    AlarmSensor = 0xa1,
    NonInteroperable = 0xFF,
}
}
