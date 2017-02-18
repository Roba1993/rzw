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

    let mut driver = rzw::driver::serial::SerialDriver::new(&DEVICE).unwrap();

    println!("SEND {:?}", driver.write(vec!(0x03, 0x03, 0x20, 0x01, 0x00)));
    println!("RECV {:?}", driver.read());
    println!("RECV {:?}", driver.read());

    driver.write(rzw::cmds::basic::get(0x03)).unwrap();
    println!("RECV {:?}", rzw::cmds::Message::parse(&driver.read().unwrap()));
    println!("RECV {:?}", driver.read());


    let mut zwave = rzw::basic::Controller::new(driver).unwrap();
    zwave.node(3);



    //let zwave = open("zwave/path");
    //zwave.node(3).map(|n| n.basic_set(1));
}
