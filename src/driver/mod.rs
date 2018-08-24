//! ZWave driver - bottom layer
//!
//! The drivers building up the foundation of this crate. They provide
//! a common interface over all the different ZWave controller.
//!
//! This is layer can be used directly, even when it's not proposed.
//! Try to use the mid or top layer for an easier access to the ZWave
//! functionalities.

// load the mods
pub mod serial;

// imports for the crate
use error::Error;

/// Driver trait to specify the functions which are needed
/// for a driver implementation. A driver provides the access
/// to the Z-Wave network.
pub trait Driver {
    /// Write data to the Z-Wave network.
    fn write<N>(&mut self, N) -> Result<u8, Error>
    where
        N: Into<Vec<u8>>;

    /// Read data from the Z-Wave network.
    /// Returns the received message or an error.
    fn read(&mut self) -> Result<Vec<u8>, Error>;

    /// Returns the id of the registered nodes in the Z-Wave network.
    fn get_node_ids(&mut self) -> Result<Vec<u8>, Error>;

    /// Returns the generic type of a node.
    fn get_node_generic_class<N>(&mut self, N) -> Result<GenericType, Error>
    where
        N: Into<u8>;
}

/// List of the generic node types
enum_from_primitive! {
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GenericType {
    Unknown = 0x00,
    RemoteController = 0x01,
    StaticController = 0x02,
    AvControlPoint = 0x03,
    RoutingSlave = 0x04,
    Display = 0x06,
    GarageDoor = 0x07,
    WindowCovering = 0x09,
    Thermostat = 0x08,
    RepeaterSlave = 0x0F,
    BinarySwitch = 0x10,
    MultiLevelSwitch = 0x11,
    RemoteSwitch = 0x12,
    ToggleSwitch = 0x13,
    ZIpGateway = 0x14,
    ZIpNode = 0x15,
    Ventilation = 0x16,
    GenericSecurityPanel = 0x17,
    RemoteSwitch2 = 0x18,
    BinarySensor = 0x20,
    MultilevelSensor = 0x21,
    WaterControl = 0x22,
    PulseMeter = 0x30,
    Meter = 0x31,
    EntryControl = 0x40,
    SemiInteroperable = 0x50,
    AlarmSensor = 0xa1,
    NonInteroperable = 0xFF,
}
}
