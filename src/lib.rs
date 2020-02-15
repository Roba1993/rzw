//! # Rusty Z-Wave
//!
//! The `rzw` crate provides a native functionality to control a Z-Wave network over a USB Z-Wave dongle. It's completely written in Rust to ensure safety and performance.
//!
//! ---
//! ## Compatibility
//! The `rzw` crate depends on the serial create, which is compatible with Windows and any Unix operating system that implements the termios API. The following platforms are confirmed to be compatible:
//!
//! * Linux (x86_64, armv6l)
//! * OS X (x86_64)
//! * FreeBSD (amd64)
//! * OpenBSD (amd64)
//! * Windows (x86_64)
//! Compiling the `rzw` crate requires Rust 1.9 or later.
//!
//! ---
//!
//! ## Usage
//! Add `rzw` as a dependency in `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! rzw = { git = "https://github.com/Roba1993/rzw" }
//! ```
//!
//! Use the `rzw::Controller` as starting point to communicate, which communicates with the Z-Wave network.
//!
//! ```rust,ignore
//! extern crate rzw;
//!
//! fn main() {
//!     // Access the zwave network
//!     let mut zwave = rzw::open("/dev/tty.usbmodem1411").unwrap();
//!
//!     // get all node ids
//!     let nodes = zwave.nodes();
//!
//!     // loop over the nodes
//!     for node in nodes {
//!         // print the available command classes for each node
//!         println!("{:?}", zwave.node(node).map(|n| n.get_commands()));
//!
//!         // set the basic value on all nodes
//!         // for binary switch this means, turn them on
//!         zwave.node(node).map(|n| n.basic_set(0xFF)).unwrap().unwrap();
//!     }
//! }
//! ```

// We create code lib code
#![allow(dead_code)]

// load all internal dependencies, which are used
pub mod defs;
pub mod driver;
pub mod error;

pub fn open<P>(
    path: P,
) -> crate::error::Result<crate::driver::SerialDriver<Box<dyn serial::SerialPort>>>
where
    P: Into<String>,
{
    // imports needed
    use serial::prelude::*;

    // open the serial port
    let mut port = serial::open(&path.into())?;

    // set the settings
    port.reconfigure(&|settings| {
        settings.set_baud_rate(serial::Baud115200)?;
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowHardware);
        Ok(())
    })?;

    // set the timeout
    port.set_timeout(std::time::Duration::from_millis(100))?;

    Ok(crate::driver::SerialDriver::new(Box::new(port)))
}
