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

// edit here the path to your Z-Wave controller device
static DEVICE: &'static str = "/dev/cu.usbmodem1411";

fn main() {
    // only continue with testing if the device path is set
    if DEVICE == "" {
        println!("Please define a path to your controller in the source code");
        return;
    }

    // open a zwave controller
    let mut zwave = rzw::open(DEVICE).unwrap();

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
            .map(|n| n.basic_set(0x00))
            .unwrap()
            .unwrap();

        // Get the status for each node
        println!(
            "Node 3 Status: {:?}",
            zwave.node(node_id).map(|n| n.basic_get())
        );
    }

    zwave.handle_messages();
}
