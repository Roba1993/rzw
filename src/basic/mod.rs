//! ZWave basic functionality - top layer
//!
//! This is the top layer which handles the whole ZWave interface
//! and allows to easy access all nodes and their functionality.
//!
//! It's proposed to use this module from the crate.

//! The `Controller` provides the functionality to connected
//! to a Z-Wave network, to send  messages and to receive them.


use driver::{Driver, GenericType};
use cmds::{CommandClass};
use error::Error;
use cmds::info::NodeInfo;
use cmds::basic::Basic;
use cmds::switch_binary::SwitchBinary;

use std::rc::Rc;
use std::cell::RefCell;
use std::clone::Clone;

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

    /// Return all node ids
    pub fn nodes(&self) -> Vec<u8> {
        // get all node ids
        self.nodes.borrow().iter().map(|n| n.id).collect::<Vec<u8>>()
    }
}

/************************** Node Area *********************/

#[derive(Debug)]
pub struct Node<D> where D: Driver {
    driver: Rc<RefCell<D>>,
    id: u8,
    types: Vec<GenericType>,
    cmds: Vec<CommandClass>
}

impl<D> Node<D> where D: Driver {
    // Create a new node.
    pub fn new(driver: Rc<RefCell<D>>, id: u8) -> Node<D>{
        let mut node = Node {
            driver: driver,
            id: id,
            types: vec!(),
            cmds: vec!()
        };

        // update the node information
        node.update_node_info().is_ok();

        node
    }

    /// Updates the information of the node
    pub fn update_node_info(&mut self) -> Result<(), Error> {
        // convert it
        let (types, cmds) = self.node_info_get()?;

        self.types = types;
        self.cmds = cmds;

        Ok(())
    }

    // get the node id
    pub fn get_id(&self) -> u8 {
        self.id
    }

    pub fn get_commands(&self) -> Vec<CommandClass> {
        self.cmds.clone()
    }

    /// This function returns the GenericType for the node and the CommandClass.
    pub fn node_info_get(&self) -> Result<(Vec<GenericType>, Vec<CommandClass>), Error> {
        // Send the command
        self.driver.borrow_mut().write(NodeInfo::get(self.id))?;

        // Receive the result
        let msg = self.driver.borrow_mut().read()?;

        // convert and return it
        NodeInfo::report(msg)
    }

    /// This function sets the basic status of the node.
    pub fn basic_set<V>(&self, value: V) -> Result<u8, Error>
    where V: Into<u8> {
        // Send the command
        self.driver.borrow_mut().write(Basic::set(self.id, value.into()))
    }

    pub fn basic_get(&self) -> Result<u8, Error> {
        // Send the command
        self.driver.borrow_mut().write(Basic::get(self.id))?;

        // read the answer and convert it
        Basic::report(self.driver.borrow_mut().read()?)
    }

    /// The Binary Switch Command Class is used to control devices with On/Off
    /// or Enable/Disable capability.
    ///
    /// The Binary Switch Set command, version 1 is used to set a binary value.
    pub fn switch_binary_set<V>(&self, value: V) -> Result<u8, Error>
    where V: Into<bool> {
        // Send the command
        self.driver.borrow_mut().write(SwitchBinary::set(self.id, value))
    }

    /// The Binary Switch Command Class is used to control devices with On/Off
    /// or Enable/Disable capability.
    ///
    /// The Binary Switch Get command, version 1 is used to request the status
    /// of a device with On/Off or Enable/Disable capability.
    pub fn switch_binary_get(&self) -> Result<bool, Error> {
        // Send the command
        self.driver.borrow_mut().write(SwitchBinary::get(self.id))?;

        // read the answer and convert it
        SwitchBinary::report(self.driver.borrow_mut().read()?)
    }
}


impl<D> Clone for Node<D> where D: Driver {
    /// We need to implement Clone manually because of a bugin rust
    fn clone(&self) -> Node<D> {
        Node {
            driver: self.driver.clone(),
            id: self.id,
            types: self.types.clone(),
            cmds: self.cmds.clone()
        }
    }
}
