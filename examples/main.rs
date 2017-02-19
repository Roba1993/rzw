//! Examples to see how the create should be used.
//! To run the example right, you first have to edit the `DEVICE`
//! variable to your local connected Z-Wave controller device.
//!
//! After it start the example with
//! ```
//! cargo run --example main
//! ```

extern crate rzw;
extern crate enum_primitive;

use rzw::driver::Driver;
use enum_primitive::FromPrimitive;


// edit here the path to your Z-Wave controller device
static DEVICE: &'static str = "/dev/tty.usbmodem1421";


fn main() {
    // only continue with testing if the device path is set
    if DEVICE == "" {
        println!("Please define a path to your controller in the source code");
        return;
    }

    // open a zwave controller
    let zwave = rzw::open(DEVICE);


    /*
    Test's -> need access to driver over the controller

    println!("SEND {:?}", driver.write(vec!(0x03, 0x03, 0x20, 0x01, 0x00)));

    println!("SEND {:?}", driver.write(rzw::cmds::basic::get(0x03)));
    //println!("RECV {:?}", rzw::cmds::Message::parse(&driver.read().unwrap()));
    println!("RECV {:?}", driver.read());

    println!("SEND {:?}", driver.write(rzw::cmds::info::get(0x03)));
    let m = driver.read().unwrap();
    println!("{:?}", rzw::cmds::info::parse(m));*/

}
