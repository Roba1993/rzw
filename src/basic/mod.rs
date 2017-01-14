//! ZWave basic functionality - top layer
//!
//! This is the top layer which handles the whole ZWave interface
//! and allows to easy access all nodes and their functionality.
//!
//! It's proposed to use this module from the crate.

//! The `Controller` provides the functionality to connected
//! to a Z-Wave network, to send  messages and to receive them.


use driver::{Driver, GenericType};

use std::rc::Rc;
use std::cell::RefCell;
use cc::CmdClass;

use error::Error;

/************************** Controller Area *********************/

/// The private representation of a controller
struct _Controller<D> where D: Driver+Clone {
    driver: D,
    nodes: Vec<Node<D>>,
}

// reference type for the controller
type ControllerRef<D> = Rc<RefCell<_Controller<D>>>;

// The public representation of a controller, with some syntactic sugar.
pub struct Controller<D>(ControllerRef<D>) where D: Driver+Clone;

// The actual controller implementation
impl<D> Controller<D> where D: Driver+Clone {
    /// Creates a new controller
    pub fn new(driver: D) -> Result<Controller<D>, Error> {
        let controller = Controller(Rc::new(RefCell::new(_Controller {
            driver: driver,
            nodes: vec!()
        })));

        // discover nodes automaically
        controller.discover_nodes()?;

        Ok(controller)
    }

    /// Refresh the complete node list
    pub fn discover_nodes(&self) -> Result<(), Error> {
        // clear the existing nodes
        self.0.borrow_mut().nodes.clear();

        // write out the descovery node command and receive the answer
        let res = self.0.borrow_mut().driver.get_node_ids()?;

        // create a node object for each id
        for i in res {
            // create the node for the given id
            let n = Node::new(self.clone(), i as u8);
            // add it !!! need 2 steps to prevent collition with borrow
            self.0.borrow_mut().nodes.push(n);
        }

        // when everything went well, return no error
        Ok(())
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

    /// returns the driver for this controller
    pub fn get_driver(&self) -> D {
        self.0.borrow().driver.clone()
    }
}

/// The clone function need to be written manually
/// beacause of a 'bug' in rust.
impl<D> Clone for Controller<D> where D: Driver+Clone {
    fn clone(&self) -> Controller<D> {
        Controller(self.0.clone())
    }
}

/************************** Node Area *********************/

// The private representation of a node
struct _Node<D> where D: Driver+Clone {
    id: u8,
    controller: Controller<D>,
    generic_type: GenericType,
    class_basic: Option<CmdClass>
}

// reference type for the node
type NodeRef<D> = Rc<RefCell<_Node<D>>>;

// The public representation of a node, with some syntactic sugar.
pub struct Node<D>(NodeRef<D>) where D: Driver+Clone;

// The actual node implementation
impl<D> Node<D> where D: Driver+Clone {
    /// Creates a new node with no edges.
    pub fn new(contr: Controller<D>, id: u8) -> Node<D> {
        let node = Node(Rc::new(RefCell::new(_Node {
            id: id,
            controller: contr,
            generic_type: GenericType::Unknown,
            class_basic: None
        })));

        // we need to handle to spress the warning,
        // wiich can't be deactivated until today
        if node.discover_type().is_err() {
            node.0.borrow_mut().generic_type = GenericType::Unknown;
        }
        node.discover_classes();

        node
    }

    /// Sets the available type for this node
    pub fn discover_type(&self) -> Result<(), Error> {
        let mut this = &mut self.0.borrow_mut();

        this.generic_type = this.controller.get_driver().get_node_generic_class(&this.id)?;

        Ok(())
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

impl<D> Clone for Node<D> where D: Driver+Clone {
    fn clone(&self) -> Node<D> {
        Node(self.0.clone())
    }
}
