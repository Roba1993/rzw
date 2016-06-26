use std::cell::RefCell;
use std::time::Duration;
use std::rc::Rc;

use serial::{self, SystemPort, SerialPort};
use std::io::{Write, Read};
use std::io::ErrorKind as StdErrorKind;

use error::{Error, ErrorKind};
use msg::{Message, Header, Type};


// The private representation of the driver
struct _Driver {
    port: SystemPort,
    messages: Vec<Message>
}

// reference type for the driver
type DriverRef = Rc<RefCell<_Driver>>;

// The public representation of the driver, with some syntactic sugar.
#[derive(Clone)]
pub struct Driver(DriverRef);

// The actual node implementation
impl Driver {
    /// Creates a new driver which is the connection point to
    /// ZWave device & network.
    /// If this function throws an error, probaply your path is
    /// wrong, or the device is not connected.
    pub fn new(path: &str) -> Result<Driver, Error> {
        // try to open the serial port
        let mut port = try!(serial::open(path));

        // set the settings
        try!(port.reconfigure(&|settings| {
            try!(settings.set_baud_rate(serial::Baud9600));
            settings.set_char_size(serial::Bits8);
            settings.set_parity(serial::ParityNone);
            settings.set_stop_bits(serial::Stop1);
            settings.set_flow_control(serial::FlowHardware);
            Ok(())
        }));

        // set the timeout
        try!(port.set_timeout(Duration::from_millis(300)));

        let driver = _Driver {
            port: port,
            messages: vec!()
        };

        Ok(Driver(Rc::new(RefCell::new(driver))))
    }

    /// This function writes an message to the driver and is returning
    /// the result `msg::Header`.
    pub fn write(&self, msg: Message) -> Result<Message, Error> {
        let mut this = &mut self.0.borrow_mut();

        // call the simple write function and return the result direct
        Driver::simple_write(&mut this, msg)
    }

    /// Writes a message to the network and automatically tries to read and return
    /// the answer from the device.
    pub fn write_and_read(&self, mut msg: Message) -> Result<Message, Error> {
        let mut this = &mut self.0.borrow_mut();

        // write the message
        let res = try!(Driver::simple_write(&mut this, msg.clone()));

        // when there is no ACK throw error
        if res.header != Header::ACK {
            return Err(Error::new(ErrorKind::UnknownZWave, "The receiver didn't accepted the message"));
        }

        // reformat the message
        msg.typ = Type::Response;

        // try to read the response message and return it
        Driver::simple_read_msg(&mut this, msg)
    }

    /// This function reads all the messages which have are available on
    /// the driver and returns the oldest in the qeue. When there a no messages
    /// left, it's returning a `io::Error(TimeOut)`
    pub fn read(&self) -> Result<Message, Error> {
        let mut this = &mut self.0.borrow_mut();

        // read everything from the driver and return if a
        // error occours
        let error = Driver::read_all(&mut this);
        if error.is_some() {
            return Err(error.unwrap())
        }

        // when there is a message in the qeue
        if this.messages.len() > 0 {
            // return the oldest message
            return Ok(this.messages.remove(0));
        }

        // try a last time a read from the driver and return the result
        Driver::try_read_msg(&mut this.port)
    }

    /// This function reads all messages from the driver and tries to find
    /// a message from the message list with the same header, typ & function.
    pub fn read_msg(&self, msg: Message) -> Result<Message, Error> {
        let mut this = &mut self.0.borrow_mut();

        // read the message
        Driver::simple_read_msg(&mut this, msg)
    }

    /*** Private functions below this line ***/

    /// This function writes an message to the driver and is returning
    /// the result `msg::Header`.
    fn simple_write(mut this: &mut _Driver, msg: Message) -> Result<Message, Error> {
        // read everything from the driver and return if a
        // error occours
        let error = Driver::read_all(&mut this);
        if error.is_some() {
            return Err(error.unwrap())
        }

        // send the value
        try!(this.port.write(msg.get_command().as_slice()));

        // read the message accept
        Driver::try_read_msg(&mut this.port)
    }

    /// This function reads all messages from the driver and tries to find
    /// a message from the message list with the same header, typ & function.
    fn simple_read_msg(mut this: &mut _Driver, msg: Message) -> Result<Message, Error> {
        // read everything from the driver and return if a
        // error occours
        let error = Driver::read_all(&mut this);
        if error.is_some() {
            return Err(error.unwrap())
        }

        // loop over all messages
        for i in 0 .. this.messages.len() {
            let m = this.messages[i].clone();

            // check if the message has the same header, typ & function
            if msg.header == m.header && msg.typ == m.typ && msg.func == m.func {
                // when we have a matching message remove and return it
                return Ok(this.messages.remove(i));
            }
        }

        // return error when no matching message was found
        Err(Error::new(ErrorKind::Io(StdErrorKind::NotFound), "No matching message are found"))
    }

    /// Read all the messages from the bus. When a timeout occours
    /// the function returns None. When a real error occours the function
    /// returns the error.
    fn read_all(this: &mut _Driver) -> Option<Error> {
        // read all messages
        loop {
            // read a single message
            let m = Driver::try_read_msg(&mut this.port);

            // if there was a message add it to the list
            if m.is_ok() {
                this.messages.push(m.unwrap());
            }
            // if we had a timeout - end the loop, all messages are reat
            else if m.clone().err().unwrap().kind() == ErrorKind::Io(StdErrorKind::TimedOut) {
                return None;
            }
            // return the occoured error
            else {
                return m.err();
            }
        }
    }

    /// This function reads a single message from the ZWave device/driver
    fn try_read_msg(port: &mut SystemPort) -> Result<Message, Error> {
        // buffer to read each byte in
        let mut buf = [0u8; 1];
        // result vector
        let mut result : Vec<u8> = Vec::new();

        // try to read the first byte
        try!(port.read(&mut buf));

        // when the first byte is the start of a frame
        if buf[0] == Header::SOF as u8 {
            // add the header byte to the result
            result.push(buf[0]);

            // read the next byte which includes the length
            try!(port.read(&mut buf));

            // add the length to the result
            result.push(buf[0]);

            // read the full length of the message
            let len = buf[0];
            for _ in 0..len {
                // read a byte
                try!(port.read(&mut buf));
                // add the byte to the result
                result.push(buf[0]);
            }

            // create the message
            let m = Message::parse(result.as_slice());

            // if it was successfull return ACK
            if m.is_ok() {
                try!(port.write(Message::new_header(Header::ACK).get_command().as_slice()));
            }
            // if there occoured an error send back a NAK
            else {
                try!(port.write(Message::new_header(Header::NAK).get_command().as_slice()));
            }

            //return the message
            return m;
        }
        // on message ackonwledge
        else if buf[0] == Header::ACK as u8 {
            return Ok(Message::new_header(Header::ACK));
        }
        // on message not ackonwledge
        else if buf[0] == Header::NAK as u8 {
            return Ok(Message::new_header(Header::NAK));
        }
        // on resent
        else if buf[0] == Header::CAN as u8 {
            return Ok(Message::new_header(Header::CAN));
        }

        // if the header is unknown return a error
        Err(Error::new(ErrorKind::UnknownZWave, "Unknown ZWave header detected"))
    }

}
