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

// load all external dependencies, which are used
#[macro_use]
extern crate enum_primitive;
extern crate num;
extern crate serial;

// load all internal dependencies, which are used
pub mod basic;
pub mod cmds;
pub mod driver;
pub mod error;

// lead mods which are used
use basic::Controller;
use driver::serial::SerialDriver;
use error::Error;

/// Function to start a Z-Wave Controller
pub fn open<P>(path: P) -> Result<Controller<SerialDriver>, Error>
where
    P: Into<String>,
{
    // Generate a new Serial driver
    let driver = SerialDriver::new(path.into())?;

    // Generate a new controller and return it
    Controller::new(driver)
}
