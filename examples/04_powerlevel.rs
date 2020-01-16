//! Examples to see how the create should be used.
//! To run the example right, you first have to edit the `DEVICE`
//! variable to your local connected Z-Wave controller device.
//!
//! After it start the example with
//! ```
//! cargo run --example 04_powerlevel
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

    // set the power level
    println!("Set the powerlevel to minus5dBm");
    zwave
        .node(NODE)
        .map(|n| n.powerlevel_set(rzw::basic::PowerLevelStatus::minus5dBm, 5))
        .unwrap()
        .unwrap();

    // wait 2 seconds
    thread::sleep(time::Duration::from_secs(2));

    // get the power level
    println!(
        "Powerlevel status: {:?}",
        zwave.node(NODE).map(|n| n.powerlevel_get())
    );

    // wait 5 seconds to get the node back to normal mode
    thread::sleep(time::Duration::from_secs(5));

    // set the power level test node
    println!("Set the powerlevel test node to minus5dBm");
    zwave
        .node(NODE)
        .map(|n| {
            n.powerlevel_test_node_set(NODE, rzw::basic::PowerLevelStatus::minus5dBm, 5 as u16)
        })
        .unwrap()
        .unwrap();

    // wait 2 seconds
    thread::sleep(time::Duration::from_secs(2));

    // get the power level
    println!(
        "Powerlevel test node status: {:?}",
        zwave.node(NODE).map(|n| n.powerlevel_test_node_get())
    );
}
