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

    let m = vec!(0x03, 0x03, 0x20, 0x01, 0xff);
    let id = driver.write(m).unwrap();
    println!("RECV {:?}", driver.read());
    println!("RECV {:?}", driver.read());
    println!("RECV {:?}", driver.read());
    println!("RECV {:?}", driver.read());

    let id = driver.write(rzw::cmds::basic::get(0x03)).unwrap();
    //println!("RECV {:?}", rzw::cmds::Message::parse(&driver.read(&id).unwrap()));
    println!("RECV {:?}", driver.read());
    println!("RECV {:?}", driver.read());
    println!("RECV {:?}", driver.read());
    println!("RECV {:?}", driver.read());


    //println!("{:?}", driver.get_messages());

    use rzw::error::Error;
    let mut zwave = rzw::basic::Controller::new(driver).unwrap();
    zwave.node(3);



    //let zwave = open("zwave/path");
    //zwave.node(3).map(|n| n.basic_set(1));
}
