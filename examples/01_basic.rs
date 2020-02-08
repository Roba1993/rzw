//! Examples to see how the create should be used.
//! To run the example right, you first have to edit the `DEVICE`
//! variable to your local connected Z-Wave controller device.
//!
//! After it start the example with
//! ```
//! cargo run --example 01_basic COM3
//! ```
//! where `COM3` is the serial port to adress,
//! this is different for each PC and OS.

extern crate enum_primitive;
extern crate rzw;

use std::{thread, time};

fn main() {
    // get all input parameter
    let args: Vec<String> = std::env::args().collect();

    // only continue with testing if the device path is set
    let port = args.get(1).unwrap();
    if port == "" {
        println!("Please define a path as the first parameter");
        return;
    }

    // open a zwave controller
    let mut zwave = rzw::open(port).expect("Z-Wave driver can't be started");

    // loop over all nodes
    for node_id in zwave.nodes() {
        // print the available command classes for each node
        println!(
            "Node {:?} available Command Classes:\n{:?}\n",
            node_id,
            zwave.node(node_id).map(|n| n.get_commands())
        );

        // Turn each node on
        zwave
            .node(node_id)
            .map(|n| n.basic_set(0x01))
            .unwrap()
            .unwrap();

        // Get the status for each node
        println!(
            "Node {} Status: {:?}",
            node_id,
            zwave.node(node_id).map(|n| n.basic_get())
        );
    }
}
