//! Examples to see how the create should be used.
//! To run the example right, you first have to edit the `DEVICE`
//! variable to your local connected Z-Wave controller device.
//!
//! After it start the example with
//! ```
//! cargo run --example main
//! ```

extern crate rzw;

// edit here the path to your Z-Wave controller device
static DEVICE: &'static str = "/dev/tty.usbmodem1411";


fn main() {
    // only continue with testing if the device path is set
    if DEVICE == "" {
        println!("Please define a path to your controller in the source code");
        return;
    }

    // create a new controller
    let controller = rzw::Controller::new(&DEVICE).unwrap();

    // get all nodes
    let nodes = controller.get_nodes();

    // loop over the nodes
    for node in nodes {
        // set the basic value on all nodes
        // for binary switch this means, turn them on
        println!("SEND: {:?}", node.get_basic().unwrap().set(0xFF));
    }
}
