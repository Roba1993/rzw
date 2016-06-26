use controller::Controller;
use cmd_class::basic::Basic;
use std::rc::Rc;
use std::cell::RefCell;


// The private representation of a node
struct _Node {
    id: u8,
    controller: Controller,
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
            class_basic: None
        })));

        node.discover_classes();

        node
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
}
