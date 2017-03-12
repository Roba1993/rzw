//! The Meter Command Class defines the Commands necessary to read accumulated values in physical
//! units from a water meter or metering device (gas, electric etc.) and thereby enabling automatic
//! meter reading capabilities.
//!
//! Automatic meter reading (AMR), is the technology of automatically collecting data from water
//! meter or energy metering devices and transferring that data to a central database for billing
//! and/or analyzing.

use cmds::{CommandClass, Message, MeterData};
use error::{Error, ErrorKind};
use enum_primitive::FromPrimitive;
use num::PrimInt;


enum_from_primitive! {
#[derive(Copy, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
/// List of the different meter types.
enum MeterType {
Electric = 0x01,
Gas = 0x02,
Water = 0x03,
}}

enum_from_primitive! {
#[derive(Copy, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
/// List of the different electric meter values.
enum ElectricMeter {
    kWh = 0x00,
    kVAh = 0x01,
    W = 0x02,
    PulseCount = 0x03,
}}

enum_from_primitive! {
#[derive(Copy, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
/// List of the different gas meter values.
enum GasMeter {
    CubicMeters = 0x00,
    CubicFeet = 0x01,
    PulseCount = 0x03,
}}

enum_from_primitive! {
#[derive(Copy, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
/// List of the different water meter values.
enum WaterMeter {
    CubicMeters = 0x00,
    CubicFeet = 0x01,
    USGallons = 0x02,
    PulseCount = 0x03,
}}


#[derive(Debug, Clone)]
/// Meter Command Class
pub struct Meter;


impl Meter {
    /// The Meter Get Command is used to request the accumulated consumption in physical units
    /// from a metering device.
    pub fn get<N>(node_id: N) -> Message
    where N: Into<u8> {
        // _________________________________________________________________
        // |   7   |   6   |   5   |   4   |   3   |   2   |   1   |   0   |
        // |            Command Class = COMMAND_CLASS_METER(0x32)          |
        // |                    Command = METER_GET(0x01)                  |
        // -----------------------------------------------------------------
        Message::new(node_id.into(), CommandClass::METER, 0x01, vec!())
    }

    /// The Meter Get Command is used to request the accumulated consumption in physical units
    /// from a metering device.
    pub fn get_v2<N, S>(node_id: N, scale: S) -> Message
    where N: Into<u8>, S: Into<MeterData> {
        // _________________________________________________________________
        // |   7   |   6   |   5   |   4   |   3   |   2   |   1   |   0   |
        // |            Command Class = COMMAND_CLASS_METER(0x32)          |
        // |                    Command = METER_GET(0x01)                  |
        // |        Reserved       |      Scale    |        Reserved       |
        // -----------------------------------------------------------------
        Message::new(node_id.into(), CommandClass::METER, 0x01, vec!((scale.into().get_scale() << 3)))
    }

    /// The Meter Report Command is used to advertise a meter reading.
    pub fn report<M>(msg: M) -> Result<MeterData, Error>
    where M: Into<Vec<u8>> {
        // _________________________________________________________________
        // |   7   |   6   |   5   |   4   |   3   |   2   |   1   |   0   |
        // |            Command Class = COMMAND_CLASS_METER(0x32)          |
        // |                    Command = METER_GET(0x01)                  |
        // |                          Meter Type                           |
        // |       Precision       |      Scale    |          Size         |
        // |                         Meter Value 1                         |
        // |                              ...                              |
        // |                         Meter Value n                         |
        // -----------------------------------------------------------------

        // get the message
        let msg = msg.into();

        // the message need to be exact 6 digits long
        if msg.len() < 8 {
            return Err(Error::new(ErrorKind::UnknownZWave, "Message is to short"));
        }

        // check the CommandClass and command
        if msg[3] != CommandClass::METER as u8 || msg[4] != 0x02 {
            return Err(Error::new(ErrorKind::UnknownZWave, "Answer contained wrong command class"));
        }

        // get the meter type
        let typ = MeterType::from_u8(msg[5]).ok_or(Error::new(ErrorKind::UnknownZWave, "Answer contained wrong meter type"))?;

        // get the precission
        let (precision, scale, size) = Meter::get_precision_scale_size(msg[6]);

        // check the message length coorectly
        if msg.len() != 7 + size as usize {
            return Err(Error::new(ErrorKind::UnknownZWave, "Message has the wrong length"));
        }

        // get the value
        let value = Meter::calc_value(&msg[7 .. 7 + size as usize], precision);

        // return the value in MeterData format
        Meter::to_meter_data(value, typ, scale)
    }

    /// The Meter Report Command is used to advertise a meter reading.
    pub fn report_v2<M>(msg: M) -> Result<(MeterData, u16, MeterData), Error>
    where M: Into<Vec<u8>> {
        // _________________________________________________________________
        // |   7   |   6   |   5   |   4   |   3   |   2   |   1   |   0   |
        // |            Command Class = COMMAND_CLASS_METER(0x32)          |
        // |                    Command = METER_GET(0x01)                  |
        // |  none |   Rate Type   |              Meter Type               |
        // |       Precision       |      Scale    |          Size         |
        // |                         Meter Value 1                         |
        // |                              ...                              |
        // |                         Meter Value n                         |
        // |                          Delta Time 1                         |
        // |                          Delta Time 2                         |
        // |                     Previous Meter Value 1                    |
        // |                              ...                              |
        // |                     Previous Meter Value n                    |
        // -----------------------------------------------------------------

        // get the message
        let msg = msg.into();

        println!("Message {:?}", msg);

        // the message need to be exact 6 digits long
        if msg.len() < 8 {
            return Err(Error::new(ErrorKind::UnknownZWave, "Message is to short"));
        }

        // check the CommandClass and command
        if msg[3] != CommandClass::METER as u8 || msg[4] != 0x02 {
            return Err(Error::new(ErrorKind::UnknownZWave, "Answer contained wrong command class"));
        }

        // get the meter type
        let (_, typ) = Meter::get_rate_meter_type(msg[5])?;

        // get the precission, scale and size
        let (precision, scale, size) = Meter::get_precision_scale_size(msg[6]);

        // check the message length coorectly
        if msg.len() < 9 + size as usize {
            return Err(Error::new(ErrorKind::UnknownZWave, "Message has the wrong length"));
        }

        // get the value
        let value = Meter::calc_value(&msg[7 .. 7 + size as usize], precision);

        // get the time between this and the last report
        let time = ((msg[7+size as usize] as u16) << 8) | msg[8+size as usize] as u16;

        // get the pre value
        let pre_value;
        if time == 0x00 || msg.len() < 10 + (2*size) as usize {
            pre_value = 0.0;
        }
        else {
            pre_value = Meter::calc_value(&msg[10 + size as usize .. 10 + (2 * size) as usize], precision);
        }

        // return the value in MeterData format
        Ok((Meter::to_meter_data(pre_value, typ, scale)?, time, Meter::to_meter_data(value, typ, scale)?))
    }



    // extract the precision, scale and size as bit information
    fn get_precision_scale_size(input: u8) -> (u8, u8, u8) {
        ((input >> 5), ((input >> 3) & 0b00000011), (input & 0b00000111))
    }

    /// generate the value out of the scale and byte vector
    fn calc_value(bytes: &[u8], precision: u8) -> f64 {
        // pow the prevision and set as f64
        let precision = (10.pow(precision as u32)) as f64;

        // transform for one byte
        if bytes.len() == 1 {
            return (bytes[0] as i8) as f64 / precision;
        }

        // transform for two bytes
        if bytes.len() == 2 {
            return (((bytes[0] as i16) << 8) | bytes[1] as i16) as f64 / precision;
        }

        // transform for four bytes
        if bytes.len() == 4 {
            return (((((bytes[0] as i32) << 24) |
                (bytes[1] as i32) << 16) |
                (bytes[2] as i32) << 8) |
                (bytes[3] as i32))
                as f64 / precision;
        }


        0.0
    }

    /// format the value into the right MeterData format
    fn to_meter_data(data: f64, typ: MeterType, scale: u8) -> Result<MeterData, Error> {
         if typ == MeterType::Electric && scale == ElectricMeter::kWh as u8 {
             return Ok(MeterData::Electric_kWh(data));
         }
         else if typ == MeterType::Electric && scale == ElectricMeter::kVAh as u8 {
             return Ok(MeterData::Electric_kVAh(data));
         }
         else if typ == MeterType::Electric && scale == ElectricMeter::W as u8 {
             return Ok(MeterData::Electric_W(data));
         }
         else if typ == MeterType::Electric && scale == ElectricMeter::PulseCount as u8 {
             return Ok(MeterData::Electric_PulseCount(data));
         }
         else if typ == MeterType::Gas && scale == GasMeter::CubicMeters as u8 {
             return Ok(MeterData::Gas_meter2(data));
         }
         else if typ == MeterType::Gas && scale == GasMeter::CubicFeet as u8 {
             return Ok(MeterData::Gas_feet2(data));
         }
         else if typ == MeterType::Gas && scale == GasMeter::PulseCount as u8 {
             return Ok(MeterData::Gas_PulseCount(data));
         }
         else if typ == MeterType::Water && scale == WaterMeter::CubicMeters as u8 {
             return Ok(MeterData::Water_meter2(data));
         }
         else if typ == MeterType::Water && scale == WaterMeter::CubicFeet as u8 {
             return Ok(MeterData::Water_feet2(data));
         }
         else if typ == MeterType::Water && scale == WaterMeter::USGallons as u8 {
             return Ok(MeterData::Water_Gallons(data));
         }
         else if typ == MeterType::Water && scale == WaterMeter::PulseCount as u8 {
             return Ok(MeterData::Water_PulseCount(data));
         }

         // return error if no right match was found
         Err(Error::new(ErrorKind::UnknownZWave, "The meter value can't be created"))
    }

    fn get_rate_meter_type(input: u8) -> Result<(u8, MeterType), Error> {
        let typ = MeterType::from_u8((input & 0b00011111)).ok_or(Error::new(ErrorKind::UnknownZWave, "Answer contained wrong meter type"))?;
        let rate = (input >> 5) & 0b00000011;
        Ok((rate, typ))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// test the right conversion
    fn precision_scale_size() {
        assert_eq!((0x00, 0x00, 0x00), Meter::get_precision_scale_size(0b00000000));
        assert_eq!((0x07, 0x00, 0x00), Meter::get_precision_scale_size(0b11100000));
        assert_eq!((0x01, 0x03, 0x00), Meter::get_precision_scale_size(0b00111000));
        assert_eq!((0x01, 0x01, 0x00), Meter::get_precision_scale_size(0b00101000));
        assert_eq!((0x01, 0x01, 0x07), Meter::get_precision_scale_size(0b00101111));
        assert_eq!((0x01, 0x01, 0x01), Meter::get_precision_scale_size(0b00101001));
    }

    #[test]
    /// test the right conversion
    fn calc_value() {
        assert_eq!(0.0, Meter::calc_value(&[0x00], 0));
        assert_eq!(1.27, Meter::calc_value(&[0x7F], 2));
        assert_eq!(-12.8, Meter::calc_value(&[0x80], 1));
        assert_eq!(0.00, Meter::calc_value(&[0x00, 0x00], 0));
        assert_eq!(32.767, Meter::calc_value(&[0x7F, 0xFF], 3));
        assert_eq!(-327.68, Meter::calc_value(&[0x80, 0x00], 2));
        assert_eq!(0.00, Meter::calc_value(&[0x00, 0x00, 0x00, 0x00], 0));
        assert_eq!(2147483.647, Meter::calc_value(&[0x7F, 0xFF, 0xFF, 0xFF], 3));
        assert_eq!(-21474836.48, Meter::calc_value(&[0x80, 0x00, 0x00, 0x00], 2));
    }

}
