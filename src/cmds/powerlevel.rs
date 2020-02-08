//! The Powerlevel Command Class definition.
//!
//! The Powerlevel Command Class defines RF transmit power controlling Commands
//! useful when installing or testing a network. The Commands makes it possible
//! for supporting controllers to set/get the RF transmit power level of a node
//! and test specific links between nodes with a specific RF transmit power level.
//!
//! NOTE: This Command Class is only used in an installation or test situation.

use cmds::{CommandClass, Message};
use enum_primitive::FromPrimitive;
use error::{Error, ErrorKind};

enum_from_primitive! {
#[derive(Copy, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
/// List of the different Power level
pub enum PowerLevelStatus {
    NormalPower = 0x00,
    minus1dBm = 0x01,
    minus2dBm = 0x02,
    minus3dBm = 0x03,
    minus4dBm = 0x04,
    minus5dBm = 0x05,
    minus6dBm = 0x06,
    minus7dBm = 0x07,
    minus8dBm = 0x08,
    minus9dBm = 0x09,
}}

enum_from_primitive! {
#[derive(Copy, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
/// List of the different Operation Status responses from the device.
pub enum PowerLevelOperationStatus {
TestFailed = 0x00, //No frame was returned during the test
TestSuccess = 0x01, // At least 1 frame was returned during the test
TestInProgress = 0x02, //The test is still ongoing
}}

/// Power level command class
#[derive(Debug, Clone)]
pub struct PowerLevel;

impl PowerLevel {
    /// The Powerlevel Set Command is used to set the power level indicator value,
    /// which should be used by the node when transmitting RF, and the timeout for
    /// this power level indicator value before returning the power level defined
    /// by the application.
    ///
    /// The seconds defines how many seconds the device stays in the defined powerlevel.
    pub fn set<N, L, S>(node_id: N, level: L, seconds: S) -> Message
    where
        N: Into<u8>,
        L: Into<PowerLevelStatus>,
        S: Into<u8>,
    {
        // generate the message
        Message::new(
            node_id.into(),
            CommandClass::POWER_LEVEL,
            0x01,
            vec![level.into() as u8, seconds.into()],
        )
    }

    /// The Powerlevel Get Command is used to request the current power level value.
    /// The Powerlevel Report Command MUST be returned in response to this command.
    pub fn get<N>(node_id: N) -> Message
    where
        N: Into<u8>,
    {
        Message::new(node_id.into(), CommandClass::POWER_LEVEL, 0x02, vec![])
    }

    /// This command is used to advertise the current power level.
    ///
    /// Return the Powerlevel status and the time left on this power level.
    pub fn report<M>(msg: M) -> Result<(PowerLevelStatus, u8), Error>
    where
        M: Into<Vec<u8>>,
    {
        // get the message
        let msg = msg.into();

        // the message need to be exact 7 digits long
        if msg.len() != 7 {
            return Err(Error::new(ErrorKind::UnknownZWave, "Message is too short"));
        }

        // check the CommandClass and command
        if msg[3] != CommandClass::POWER_LEVEL as u8 || msg[4] != 0x03 {
            return Err(Error::new(
                ErrorKind::UnknownZWave,
                "Answer contained wrong command class",
            ));
        }

        // get the power level state
        let level = PowerLevelStatus::from_u8(msg[5]).ok_or(Error::new(
            ErrorKind::UnknownZWave,
            "Answer contained wrong power level state",
        ))?;

        // return the values
        Ok((level, msg[6]))
    }

    /// The Powerlevel Test Node Set Command is used to instruct the destination node to transmit
    /// a number of test frames to the specified NodeID with the RF power level specified. After
    /// the test frame transmissions the RF power level is reset to normal and the result (number
    /// of acknowledged test frames) is saved for subsequent read-back. The result of the test may
    /// be requested with a Powerlevel Test Node Get Command.
    ///
    /// node_id: The node id where to send the message.
    /// test_node_id: The test NodeID that should receive the test frames.
    /// level: The power level indicator value to use in the test frame transmission.
    /// test_frames: The Test frame count field contains the number of test frames to transmit to
    ///              the Test NodeID. The first byte is the most significant byte.
    pub fn test_node_set<N, T, L, F>(
        node_id: N,
        test_node_id: T,
        level: L,
        test_frames: F,
    ) -> Message
    where
        N: Into<u8>,
        T: Into<u8>,
        L: Into<PowerLevelStatus>,
        F: Into<u16>,
    {
        // transform the test_frame count to byte array
        let frames = PowerLevel::transform_u16_to_array_of_u8(test_frames.into());

        // generate the message
        Message::new(
            node_id.into(),
            CommandClass::POWER_LEVEL,
            0x04,
            vec![
                test_node_id.into(),
                level.into() as u8,
                frames[0],
                frames[1],
            ],
        )
    }

    /// The Powerlevel Test Node Get Command is used to request the result of the latest Powerlevel
    /// Test. The Powerlevel Test Node Report Command MUST be returned in response to this command.
    pub fn test_node_get<N>(node_id: N) -> Message
    where
        N: Into<u8>,
    {
        Message::new(node_id.into(), CommandClass::POWER_LEVEL, 0x05, vec![])
    }

    /// This command is used to report the latest result of a test frame
    /// transmission started by the Powerlevel Test Node Set Command.
    ///
    /// Return the test node id, status of operation and the test frane count.
    pub fn test_node_report<M>(msg: M) -> Result<(u8, PowerLevelOperationStatus, u16), Error>
    where
        M: Into<Vec<u8>>,
    {
        // get the message
        let msg = msg.into();

        // the message need to be exact 9 digits long
        if msg.len() != 9 {
            return Err(Error::new(
                ErrorKind::UnknownZWave,
                format!("Message is too short: {:?}", msg),
            ));
        }

        // check the CommandClass and command
        if msg[3] != CommandClass::POWER_LEVEL as u8 || msg[4] != 0x06 {
            return Err(Error::new(
                ErrorKind::UnknownZWave,
                "Answer contained wrong command class",
            ));
        }

        // get the test node id
        let n_id = msg[5];

        // get the power level state
        let level = PowerLevelOperationStatus::from_u8(msg[6]).ok_or(Error::new(
            ErrorKind::UnknownZWave,
            "Answer contained wrong operation status",
        ))?;

        // get the frame count
        let frame = PowerLevel::transform_array_of_u8_to_u16(msg[7], msg[8]);

        // return the values
        Ok((n_id, level, frame))
    }

    /// transform a u16 to a u8 array.
    fn transform_u16_to_array_of_u8(x: u16) -> [u8; 2] {
        let b1: u8 = ((x >> 8) & 0xff) as u8;
        let b2: u8 = (x & 0xff) as u8;
        return [b1, b2];
    }

    /// transform two u8 into a u16 value
    fn transform_array_of_u8_to_u16(msb: u8, lsb: u8) -> u16 {
        let msb = msb as u16;
        let lsb = lsb as u16;

        ((msb << 8) | lsb)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// test the right conversion
    fn transform_u16_to_u8() {
        assert_eq!([0x00, 0x00], PowerLevel::transform_u16_to_array_of_u8(0));
        assert_eq!([0x00, 0x01], PowerLevel::transform_u16_to_array_of_u8(1));
        assert_eq!([0x01, 0x00], PowerLevel::transform_u16_to_array_of_u8(256));
        assert_eq!([0x01, 0x01], PowerLevel::transform_u16_to_array_of_u8(257));
    }

    #[test]
    /// test the right conversion
    fn transform_u8_to_u16() {
        assert_eq!(0, PowerLevel::transform_array_of_u8_to_u16(0x00, 0x00));
        assert_eq!(1, PowerLevel::transform_array_of_u8_to_u16(0x00, 0x01));
        assert_eq!(256, PowerLevel::transform_array_of_u8_to_u16(0x01, 0x00));
        assert_eq!(257, PowerLevel::transform_array_of_u8_to_u16(0x01, 0x01));
    }
}
