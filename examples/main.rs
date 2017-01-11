//! Examples to see how the create should be used.
//! To run the example right, you first have to edit the `DEVICE`
//! variable to your local connected Z-Wave controller device.
//!
//! After it start the example with
//! ```
//! cargo run --example main
//! ```

extern crate rzw;

use rzw::driver::Driver;

// edit here the path to your Z-Wave controller device
static DEVICE: &'static str = "/dev/tty.usbmodem1421";


fn main() {
    // only continue with testing if the device path is set
    if DEVICE == "" {
        println!("Please define a path to your controller in the source code");
        return;
    }

    // create a new controller
    //let controller = rzw::Controller::new(&DEVICE).unwrap();

    // get all nodes
    //let nodes = controller.get_nodes();

    // loop over the nodes
    /*for node in nodes {
        // set the basic value on all nodes
        // for binary switch this means, turn them on
        println!("SEND: {:?}", node.get_basic().unwrap().set(0x01));
        //println!("GET: {:?}", node.get_basic().unwrap().get());
        println!("READ: {:?}", controller.get_driver().read());
        //println!("GET: {:?}", node.get_basic().unwrap().get());
        thread::sleep(Duration::from_secs(3));
        println!("READ: {:?}", controller.get_driver().read());
        thread::sleep(Duration::from_secs(3));
        println!("READ: {:?}", controller.get_driver().read());
        println!("");
    }*/

    /*use rzw::msg::{Message, Type, Function};

    // clear from before
    thread::sleep(Duration::from_secs(1));
    println!("READ: {:?}", controller.get_driver().read());

    // create the write command and send it
    let msg = Message::new(Type::Request, Function::SendData, vec!(nodes.get(0).unwrap().get_id(), 0x03, 0x20, 0x01, 0x01, 0x25, 0x66));
    println!("SEND: {:?}", msg);
    println!("RECV: {:?}", controller.get_driver().write(msg));
    println!("READ: {:?}", controller.get_driver().read());
    println!("READ: {:?}", controller.get_driver().read());
    thread::sleep(Duration::from_secs(3));
    println!("READ: {:?}", controller.get_driver().read());*/

    let mut driver = rzw::driver::serial::SerialDriver::new(&DEVICE).unwrap();

    println!("ID's {:?}", driver.get_node_ids());
    println!("TYPE {:?}", driver.get_node_generic_class(&0x01));


    let m = vec!(0x04, 0x03, 0x20, 0x01, 0x01);
    let id = driver.write(m).unwrap();
    println!("M_ID {:?}", id);

    println!("RECV {:?}", driver.read(&id));
}
