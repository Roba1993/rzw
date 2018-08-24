//! ZWave basic functionality - top layer
//!
//! This is the top layer which handles the whole ZWave interface
//! and allows to easy access all nodes and their functionality.
//!
//! It's proposed to use this module from the crate.

//! The `Controller` provides the functionality to connected
//! to a Z-Wave network, to send  messages and to receive them.

pub use cmds::powerlevel::PowerLevelOperationStatus;
pub use cmds::powerlevel::PowerLevelStatus;
pub use cmds::MeterData;

use cmds::basic::Basic;
use cmds::info::NodeInfo;
use cmds::meter::Meter;
use cmds::powerlevel::PowerLevel;
use cmds::switch_binary::SwitchBinary;
use cmds::CommandClass;
use cmds::Message;
use driver::{Driver, GenericType};
use error::Error;

use std::cell::RefCell;
use std::clone::Clone;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug, Clone)]
pub struct Controller<D>
where
    D: Driver,
{
    driver: Arc<Mutex<D>>,
    nodes: Rc<RefCell<Vec<Node<D>>>>,
}

impl<D> Controller<D>
where
    D: Driver,
{
    /// Generate a new Controller to interface with the z-wave network.
    pub fn new(driver: D) -> Result<Controller<D>, Error> {
        let controller = Controller {
            driver: Arc::new(Mutex::new(driver)),
            nodes: Rc::new(RefCell::new(vec![])),
        };

        controller.discover_nodes()?;

        Ok(controller)
    }

    /// Discover all nodes which are present in the network
    pub fn discover_nodes(&self) -> Result<(), Error> {
        // clear the existing nodes
        self.nodes.borrow_mut().clear();

        // get all node id's which are in the network
        let ids = self.driver.lock().unwrap().get_node_ids()?;

        // create a node object for each id
        for i in ids {
            // create the node for the given id
            self.nodes
                .borrow_mut()
                .push(Node::new(self.driver.clone(), i as u8));
        }

        // when everything went well, return no error
        Ok(())
    }
    /// This function returns the defined node and a mutable reference
    /// to the z-wave driver.
    pub fn node<I>(&mut self, id: I) -> Option<Node<D>>
    where
        I: Into<u8>,
    {
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
        self.nodes
            .borrow()
            .iter()
            .map(|n| n.id)
            .collect::<Vec<u8>>()
    }

    pub fn handle_messages(&self) {
        /*let driver = self.sync_driver.clone();

        thread::spawn(move || {
            let m_driver = driver.lock().unwrap();
        });*/

        /*driver.read();
            match driver.borrow_mut().read() {
                Ok(raw) => {
                    println!("found message {:?}", raw);
                    let msg = Message::parse(&raw);
                    println!("message {:?}", msg);
                }
                Err(_) => println!("{}", "no message"),
            }
        */
    }
}

/************************** Node Area *********************/

#[derive(Debug)]
pub struct Node<D>
where
    D: Driver,
{
    driver: Arc<Mutex<D>>,
    id: u8,
    types: Vec<GenericType>,
    cmds: Vec<CommandClass>,
}

impl<D> Node<D>
where
    D: Driver,
{
    // Create a new node.
    pub fn new(driver: Arc<Mutex<D>>, id: u8) -> Node<D> {
        let mut node = Node {
            driver: driver,
            id: id,
            types: vec![],
            cmds: vec![],
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
        let mut driver = self.driver.lock().unwrap();

        // Send the command
        driver.write(NodeInfo::get(self.id))?;

        // Receive the result
        let msg = driver.read()?;

        // convert and return it
        NodeInfo::report(msg)
    }

    /// This function sets the basic status of the node.
    pub fn basic_set<V>(&self, value: V) -> Result<u8, Error>
    where
        V: Into<u8>,
    {
        // Send the command
        self.driver
            .lock()
            .unwrap()
            .write(Basic::set(self.id, value.into()))
    }

    pub fn basic_get(&self) -> Result<u8, Error> {
        let mut driver = self.driver.lock().unwrap();
        // Send the command
        driver.write(Basic::get(self.id))?;
        // read the answer and convert it
        Basic::report(driver.read()?)
    }

    /// The Binary Switch Command Class is used to control devices with On/Off
    /// or Enable/Disable capability.
    ///
    /// The Binary Switch Set command, version 1 is used to set a binary value.
    pub fn switch_binary_set<V>(&self, value: V) -> Result<u8, Error>
    where
        V: Into<bool>,
    {
        // Send the command
        self.driver
            .lock()
            .unwrap()
            .write(SwitchBinary::set(self.id, value))
    }

    /// The Binary Switch Command Class is used to control devices with On/Off
    /// or Enable/Disable capability.
    ///
    /// The Binary Switch Get command, version 1 is used to request the status
    /// of a device with On/Off or Enable/Disable capability.
    pub fn switch_binary_get(&self) -> Result<bool, Error> {
        let mut driver = self.driver.lock().unwrap();
        // Send the command
        driver.write(SwitchBinary::get(self.id))?;
        // read the answer and convert it
        SwitchBinary::report(driver.read()?)
    }

    /// The Powerlevel Set Command is used to set the power level indicator value,
    /// which should be used by the node when transmitting RF, and the timeout for
    /// this power level indicator value before returning the power level defined
    /// by the application.
    ///
    /// The seconds defines how many seconds the device stays in the defined powerlevel.
    pub fn powerlevel_set<S, T>(&self, status: S, seconds: T) -> Result<u8, Error>
    where
        S: Into<PowerLevelStatus>,
        T: Into<u8>,
    {
        // Send the command
        self.driver
            .lock()
            .unwrap()
            .write(PowerLevel::set(self.id, status, seconds))
    }

    /// This command is used to advertise the current power level.
    ///
    /// Return the Powerlevel status and the time left on this power level.
    pub fn powerlevel_get(&self) -> Result<(PowerLevelStatus, u8), Error> {
        let mut driver = self.driver.lock().unwrap();
        // Send the command
        driver.write(PowerLevel::get(self.id))?;

        // read the answer and convert it
        PowerLevel::report(driver.read()?)
    }

    /// The Powerlevel Test Node Set Command is used to instruct the destination node to transmit
    /// a number of test frames to the specified NodeID with the RF power level specified. After
    /// the test frame transmissions the RF power level is reset to normal and the result (number
    /// of acknowledged test frames) is saved for subsequent read-back. The result of the test may
    /// be requested with a Powerlevel Test Node Get Command.
    ///
    /// node_id: The node id where to send the message.
    /// test_node_id: The test NodeID that should receive the test frames.
    /// level: The power level indicator value to use in the test frame transmission.
    /// test_frames: The Test frame count field contains the number of test frames to transmit to
    ///              the Test NodeID. The first byte is the most significant byte.
    pub fn powerlevel_test_node_set<T, L, F>(
        &self,
        test_node_id: T,
        level: L,
        test_frames: F,
    ) -> Result<u8, Error>
    where
        T: Into<u8>,
        L: Into<PowerLevelStatus>,
        F: Into<u16>,
    {
        // Send the command
        self.driver.lock().unwrap().write(PowerLevel::test_node_set(
            self.id,
            test_node_id,
            level,
            test_frames,
        ))
    }

    /// This command is used to report the latest result of a test frame
    /// transmission started by the Powerlevel Test Node Set Command.
    ///
    /// Return the test node id, status of operation and the test frane count.
    pub fn powerlevel_test_node_get(&self) -> Result<(u8, PowerLevelOperationStatus, u16), Error> {
        // Send the command
        self.driver
            .lock()
            .unwrap()
            .write(PowerLevel::test_node_get(self.id))?;

        // read the answer and convert it
        PowerLevel::test_node_report(self.driver.lock().unwrap().read()?)
    }

    /// A meter is used to monitor a resource. The meter accumulates the resource flow over time.
    /// As an option, the meter may report not only the most recent accumulated reading but also
    /// the previous reading and the time that elapsed since then. A meter may also be able to
    /// report the current resource flow. This is known as the instant value.
    ///
    /// The Meter Get Command is used to request the accumulated consumption in physical units
    /// from a metering device.
    pub fn meter_get(&self) -> Result<MeterData, Error> {
        let mut driver = self.driver.lock().unwrap();
        // Send the command
        driver.write(Meter::get(self.id))?;

        // read the answer and convert it
        Meter::report(driver.read()?)
    }

    /// A meter is used to monitor a resource. The meter accumulates the resource flow over time.
    /// As an option, the meter may report not only the most recent accumulated reading but also
    /// the previous reading and the time that elapsed since then. A meter may also be able to
    /// report the current resource flow. This is known as the instant value.
    ///
    /// The Meter Get Command is used to request the accumulated consumption in physical units
    /// from a metering device.
    pub fn meter_get_v2<S>(&self, meter_type: S) -> Result<(MeterData, u16, MeterData), Error>
    where
        S: Into<MeterData>,
    {
        let mut driver = self.driver.lock().unwrap();
        // Send the command
        driver.write(Meter::get_v2(self.id, meter_type.into()))?;

        // read the answer and convert it
        Meter::report_v2(driver.read()?)
    }
}

impl<D> Clone for Node<D>
where
    D: Driver,
{
    /// We need to implement Clone manually because of a bugin rust
    fn clone(&self) -> Node<D> {
        Node {
            driver: self.driver.clone(),
            id: self.id,
            types: self.types.clone(),
            cmds: self.cmds.clone(),
        }
    }
}
