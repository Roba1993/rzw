# Under heavy reconstruction
Try on your own risk! - The example is outdated

# Rusty Z-Wave

The `rzw` crate provides a native functionality to control a Z-Wave network over a USB Z-Wave dongle. It's completely written in Rust to ensure safety and performance.

---
## Compatibility
The `rzw` crate depends on the serial create, which is compatible with Windows and any Unix operating system that implements the termios API. The following platforms are confirmed to be compatible:

* Linux (x86_64, armv6l)
* OS X (x86_64)
* FreeBSD (amd64)
* OpenBSD (amd64)
* Windows (x86_64)
Compiling the `rzw` crate requires Rust 1.9 or later.

---
## Usage
Add `rzw` as a dependency in `Cargo.toml`:
```toml
[dependencies]
rzw = { git = "https://github.com/Roba1993/rzw" }
```

Use the `rzw::Controller` as starting point to communicate, with the Z-Wave network.
```rust
extern crate rzw;

fn main() {
    // create a new controller
    let controller = rzw::Controller::new("/dev/tty.usbmodem1411").unwrap();

    // get all nodes
    let nodes = controller.get_nodes();

    // loop over the nodes
    for node in nodes {
        // set the basic value on all nodes
        // for binary switch this means, turn them on
        println!("SEND: {:?}", node.get_basic().unwrap().set(0xFF));
    }
}
```

---
## Thanks
* To the [serial-rs](https://github.com/dcuddeback/serial-rs) team - who made this crate possible.

---
## License
Copyright © 2016 Robert Schütte

Distributed under the [MIT License](LICENSE).
