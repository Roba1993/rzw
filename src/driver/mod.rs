// load the mods
pub mod serial_driver;

// link the driver into this namespace
pub use self::serial_driver::SerialDriver;

// imports for the crate
use error::Error;

/// Driver trait to specify the functions which are needed
/// for a driver implementation. A driver provides the access
/// to the Z-Wave network.
pub trait Driver {
    /// Write data to the Z-Wave network. Return the id of
    /// the sended message or an error.
    fn write(&mut self, Vec<u8>) -> Result<u8, Error>;

    /// Read data from the Z-Wave network for the defined id.
    /// The id '0' is the placeholder fro message with no id's.
    /// Returns the received message or an error.
    fn read(&mut self, &u8) -> Result<Vec<u8>, Error>;

    /// Returns the id of the registered nodes in the Z-Wave network.
    fn get_node_ids(&mut self) -> Result<Vec<u8>, Error>;

    /// Returns the generic type of a node.
    fn get_node_generic_class(&mut self, &u8) -> Result<GenericType, Error>;
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
