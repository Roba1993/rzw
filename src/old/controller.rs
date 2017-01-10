//! The `Controller` provides the functionality to connected
//! to a Z-Wave network, to send  messages and to receive them.

use std::cell::RefCell;
use std::rc::Rc;

use error::{Error, ErrorKind};
use node::{Node};
use msg::{Message, Type, Function};
use driver::Driver;


// The private representation of a controller
struct _Controller {
    driver: Driver,
    nodes: Vec<Node>,
}

// reference type for the controller
type ControllerRef = Rc<RefCell<_Controller>>;

// The public representation of a controller, with some syntactic sugar.
#[derive(Clone)]
pub struct Controller(ControllerRef);

// The actual controller implementation
impl Controller {
    /// Creates a new controller
    pub fn new(path: &str) -> Result<Controller, Error> {
        let controller = Controller(Rc::new(RefCell::new(_Controller {
            driver: try!(Driver::new(path)),
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
        self.0.borrow_mut().nodes.clear();

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
        }

        // when everything went well, return no error
        None
    }

    /// Returns a vector of all available nodes
    pub fn get_nodes(&self) -> Vec<Node> {
        self.0.borrow().nodes.clone()
    }

    /// Returns the Node with the given ZWave network id, when available.
    pub fn get_node(&self, index: usize) -> Option<Node> {
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
    pub fn get_driver(&self) -> Driver {
        self.0.borrow().driver.clone()
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
