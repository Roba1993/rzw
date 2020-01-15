//! Examples to see how the create should be used.
//! To run the example right, you first have to edit the `DEVICE`
//! variable to your local connected Z-Wave controller device.
//!
//! After it start the example with
//! ```
//! cargo run --example main
//! ```

extern crate enum_primitive;
extern crate rzw;

use std::{thread, time};

// edit here the path to your Z-Wave controller device
static DEVICE: &'static str = "/dev/tty.usbmodem1421";

// the node to switch on/off
static NODE: u8 = 3;

fn main() {
    // only continue with testing if the device path is set
    if DEVICE == "" {
        println!("Please define a path to your controller in the source code");
        return;
    }

    // open a zwave controller
    let mut zwave = rzw::open(DEVICE).unwrap();

    // Turn node on
    zwave
        .node(NODE)
        .map(|n| n.switch_binary_set(true))
        .unwrap()
        .unwrap();

    // get the status
    println!(
        "Node Status: {:?}",
        zwave
            .node(NODE)
            .map(|n| n.switch_binary_get())
            .unwrap()
            .unwrap()
    );

    // wait 3 seconds
    thread::sleep(time::Duration::from_secs(3));

    // Turn node off
    zwave
        .node(NODE)
        .map(|n| n.switch_binary_set(false))
        .unwrap()
        .unwrap();

    // get the status
    println!(
        "Node Status: {:?}",
        zwave
            .node(NODE)
            .map(|n| n.switch_binary_get())
            .unwrap()
            .unwrap()
    );
}
