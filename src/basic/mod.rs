//! ZWave basic functionality - top layer
//!
//! This is the top layer which handles the whole ZWave interface
//! and allows to easy access all nodes and their functionality.
//!
//! It's proposed to use this module from the crate.

//! The `Controller` provides the functionality to connected
//! to a Z-Wave network, to send  messages and to receive them.


use driver::Driver;

use std::rc::Rc;
use std::cell::RefCell;
use cc::CmdClass;

use error::Error;



/************************** Controller Area *********************/

/// The private representation of a controller
struct _Controller<D> where D: Driver {
    driver: D,
    nodes: Vec<Node<D>>,
}

// reference type for the controller
type ControllerRef<D> = Rc<RefCell<_Controller<D>>>;

// The public representation of a controller, with some syntactic sugar.
pub struct Controller<D>(ControllerRef<D>) where D: Driver;

// The actual controller implementation
impl<D> Controller<D> where D: Driver {
    /// Creates a new controller
    pub fn new(driver: D) -> Result<Controller<D>, Error> {
        let controller = Controller(Rc::new(RefCell::new(_Controller {
            driver: driver,
            nodes: vec!()
        })));

        // discover nodes automaically
        let nd = controller.discover_nodes();
        if nd.is_some() {
            return Err(nd.unwrap());
        }

        Ok(controller)
    }

    /// Refresh the complete node list
    pub fn discover_nodes(&self) -> Option<Error> {
        // clear the existing nodes
        /*self.0.borrow_mut().nodes.clear();

        // write out the descovery node command and receive the answer
        let res = self.0.borrow().driver.write_and_read(Message::new(Type::Request, Function::DiscoveryNodes, vec!()));

        // when a error occoured return it
        let res = unwrap_or_return!(res.clone().ok(), res.err());

        // grab the data
        let data = res.data;

        // check if the data is long enough and if the right bit is set
        if data.len() != 34 || data[2] != 0x1D {
            return Some(Error::new(ErrorKind::UnknownZWave, "The ZWave message has a wrong format"));
        }

        // loop over each bitmask byte
        for i in 3..31 {
            // loop over each bit of the byte
            for j in 0..7 {
                // check if the bit is set
                if Controller::get_bit_at(data[i], j) {
                    // calc the number out of the bitmask
                    let n = ((i-3) * 8) + (j as usize+1);
                    // create the node for the given id
                    let n = Node::new(self.clone(), n as u8);
                    // add it !!! need 2 steps to prevent collition with borrow
                    self.0.borrow_mut().nodes.push(n);
                }
            }
        }*/

        // when everything went well, return no error
        None
    }

    /// Returns a vector of all available nodes
    pub fn get_nodes(&self) -> Vec<Node<D>> {
        self.0.borrow().nodes.clone()
    }

    /// Returns the Node with the given ZWave network id, when available.
    pub fn get_node(&self, index: usize) -> Option<Node<D>> {
        let this = self.0.borrow();

        // loop over all nodes and check if the id exist
        for n in &this.nodes {
            if index == n.get_id() as usize {
                // return the node with the id
                return Some(n.clone());
            }
        }

        // when no id was found return nothing
        None
    }

    /*** private functions below this line ***/

    /// Checks if the bit at the requested position is set
    fn get_bit_at(input: u8, n: u8) -> bool {
        if n < 8 {
            input & (1 << n) != 0
        } else {
            false
        }
    }
}

impl<D> Clone for Controller<D> where D: Driver {
    fn clone(&self) -> Controller<D> {
        Controller(self.0.clone())
    }
}

/************************** Node Area *********************/

// The private representation of a node
struct _Node<D> where D: Driver {
    id: u8,
    controller: Controller<D>,
    generic_type: GenericType,
    class_basic: Option<CmdClass>
}

// reference type for the node
type NodeRef<D> = Rc<RefCell<_Node<D>>>;

// The public representation of a node, with some syntactic sugar.
pub struct Node<D>(NodeRef<D>) where D: Driver;

// The actual node implementation
impl<D> Node<D> where D: Driver {
    /// Creates a new node with no edges.
    pub fn new(contr: Controller<D>, id: u8) -> Node<D> {
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
        /*let mut this = &mut self.0.borrow_mut();

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
        this.generic_type = GenericType::from_u8(msg.data[4]).unwrap_or(GenericType::Unknown);*/
    }

    /// Sets the available function classes for this node
    pub fn discover_classes(&self) {
        //let mut this = &mut self.0.borrow_mut();

        // todo get the information from the device

        // basic is always available
        //this.class_basic = Some(Basic::new(self.clone()));
    }

    /// returns the basic command class to interact with
    pub fn get_basic(&self) -> Option<CmdClass> {
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

impl<D> Clone for Node<D> where D: Driver {
    fn clone(&self) -> Node<D> {
        Node(self.0.clone())
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
