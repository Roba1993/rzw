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
use cmds::CmdClass;
use std::clone::Clone;

use error::Error;

#[derive(Debug, Clone)]
pub struct Controller<D> where D: Driver {
    driver: Rc<RefCell<D>>,
    nodes: Rc<RefCell<Vec<Node<D>>>>
}

impl<D> Controller<D> where D: Driver {

    /// Generate a new Controller to interface with the z-wave network.
    pub fn new(driver: D) -> Result<Controller<D>, Error> {
        let controller = Controller {
            driver: Rc::new(RefCell::new(driver)),
            nodes: Rc::new(RefCell::new(vec!()))
        };

        controller.discover_nodes()?;

        Ok(controller)
    }

    /// This function returns the defined node and a mutable reference
    /// to the z-wave driver.
    pub fn node<I>(&mut self, id: I) -> Option<Node<D>>
    where I: Into<u8> {
        let id = id.into();

        // loop over all nodes and check if the id exist
        for n in self.nodes.borrow().iter() {
            if id == n.get_id() {
                // return the node with the id
                return Some(n.clone());
            }
        }

        // when no id was found return nothing
        None
    }

    /// Discover all nodes which are present in the network
    pub fn discover_nodes(&self) -> Result<(), Error> {
        // clear the existing nodes
        self.nodes.borrow_mut().clear();

        // get all node id's which are in the network
        let ids = self.driver.borrow_mut().get_node_ids()?;

        // create a node object for each id
        for i in ids {
            // create the node for the given id
            self.nodes.borrow_mut().push(Node::new(self.driver.clone(), i as u8));
        }

        // when everything went well, return no error
        Ok(())
    }
}

/************************** Node Area *********************/

#[derive(Debug)]
pub struct Node<D> where D: Driver {
    driver: Rc<RefCell<D>>,
    id: u8,
    generic_type: GenericType,
    class_basic: Option<CmdClass>
}

impl<D> Node<D> where D: Driver {
    // Create a new node.
    pub fn new(driver: Rc<RefCell<D>>, id: u8) -> Node<D>{
        let mut node = Node {
            driver: driver,
            id: id,
            generic_type: GenericType::Unknown,
            class_basic: None
        };

        // we need to handle to spress the warning,
        // wiich can't be deactivated until today
        if node.discover_type().is_err() {
            node.generic_type = GenericType::Unknown;
        }
        node.discover_classes();

        node
    }

    /// Sets the available type for this node
    pub fn discover_type(&mut self) -> Result<(), Error> {
        // set the genreic type for this node
        self.generic_type = self.driver.borrow_mut().get_node_generic_class(&self.id)?;

        Ok(())
    }

    /// Sets the available function classes for this node
    pub fn discover_classes(&self) {
        // todo get the information from the device

        // basic is always available
        //this.class_basic = Some(Basic::new(self.clone()));
    }

    // get the node id
    pub fn get_id(&self) -> u8 {
        self.id
    }
}


impl<D> Clone for Node<D> where D: Driver {
    /// We need to implement Clone manually because of a bugin rust
    fn clone(&self) -> Node<D> {
        Node {
            driver: self.driver.clone(),
            id: self.id,
            generic_type: self.generic_type,
            class_basic: self.class_basic
        }
    }
}
