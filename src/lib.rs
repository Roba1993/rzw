//! # Under heavy reconstruction
//! Try on your own risk! - The example is outdated
//!
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
//!  todo
//! ```
//!
//! Use the `rzw::Controller` as starting point to communicate, which communicates with the Z-Wave network.
//!
//! ```rust
//! extern crate rzw;
//!
//! fn main() {
//!     // create a new controller
//!     let controller = rzw::Controller::new("/dev/tty.usbmodem1411").unwrap();
//!
//!     // get all nodes
//!     let nodes = controller.get_nodes();
//!
//!     // loop over the nodes
//!     for node in nodes {
//!         // set the basic value on all nodes
//!         // for binary switch this means, turn them on
//!         println!("SEND: {:?}", node.get_basic().unwrap().set(0xFF));
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
pub mod error;
pub mod driver;
pub mod cmds;
pub mod basic;

// lead mods which are used
use driver::serial::SerialDriver;
use basic::Controller;
use error::Error;

/// Function to start a Z-Wave Controller
pub fn open<P>(path: P) -> Result<Controller<SerialDriver>, Error>
    where P: Into<String> {
    // Generate a new Serial driver
    let driver = SerialDriver::new(path.into())?;

    // Generate a new controller and return it
    Controller::new(driver)
}
