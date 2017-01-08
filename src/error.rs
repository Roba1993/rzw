use std::io;
use std::fmt;
use std::error::Error as StdError;
use serial::Error as SerError;
use serial;

/// Categories of errors that can occur when interacting with z-Wave.
///
/// This list is intended to grow over time and it is not recommended to exhaustively match against it.
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum ErrorKind {
    /// The controller is not available.
    ///
    /// This could indicate that the controller is in use by another process or was disconnected while
    /// performing I/O.
    NoController,

    /// A parameter was incorrect.
    InvalidInput,

    /// A unknown Z-Wave syntax was sent.
    UnknownZWave,

    /// This functionallity is not implemented.
    NotImplemented,

    /// An I/O error occured.
    ///
    /// The type of I/O error is determined by the inner `io::ErrorKind`.
    Io(io::ErrorKind)
}

/// An error type for Z-Wave operations.
#[derive(Debug, Clone)]
pub struct Error {
    kind: ErrorKind,
    description: String
}

impl Error {
    pub fn new<T: Into<String>>(kind: ErrorKind, description: T) -> Self {
        Error {
            kind: kind,
            description: description.into()
        }
    }

    /// Returns the corresponding `ErrorKind` for this error.
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt.write_str(&self.description)
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        &self.description
    }
}

impl From<io::Error> for Error {
    fn from(io_error: io::Error) -> Error {
        Error::new(ErrorKind::Io(io_error.kind()), format!("{}", io_error))
    }
}

impl From<Error> for io::Error {
    fn from(error: Error) -> io::Error {
        let kind = match error.kind {
            ErrorKind::NoController => io::ErrorKind::NotFound,
            ErrorKind::InvalidInput => io::ErrorKind::InvalidInput,
            ErrorKind::UnknownZWave => io::ErrorKind::InvalidData,
            ErrorKind::NotImplemented => io::ErrorKind::Other,
            ErrorKind::Io(kind) => kind
        };

        io::Error::new(kind, error.description)
    }
}

impl From<SerError> for Error {
    fn from(ser_error: SerError) -> Error {
        let kind = match ser_error.kind() {
            serial::ErrorKind::NoDevice => ErrorKind::NoController,
            serial::ErrorKind::InvalidInput => ErrorKind::InvalidInput,
            serial::ErrorKind::Io(kind) => ErrorKind::Io(kind)
        };

        Error::new(kind, ser_error.description())
    }
}
