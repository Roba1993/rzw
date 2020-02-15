//! rzw specific error types
//!
//! These error type is compatible with the rust standard io `ErrorKind`.

pub type Result<T> = std::result::Result<T, Error>;

/// Categories of errors that can occur when interacting with z-Wave.
///
/// This list is intended to grow over time and it is not recommended to exhaustively match against it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    Io(std::io::ErrorKind),
}

/// An error type for Z-Wave operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    kind: ErrorKind,
    description: String,
}

impl Error {
    /// Create a new error with a given type and description
    pub fn new<T: Into<String>>(kind: ErrorKind, description: T) -> Self {
        Error {
            kind: kind,
            description: description.into(),
        }
    }

    /// Returns the corresponding `ErrorKind` for this error.
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }
}

impl std::fmt::Display for Error {
    /// How to print the error
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        fmt.write_str(&self.description)
    }
}

impl std::error::Error for Error {
    /// Get the error description
    fn description(&self) -> &str {
        &self.description
    }
}

impl From<std::io::Error> for Error {
    /// Transform std io errors to this crate error
    fn from(io_error: std::io::Error) -> Error {
        Error::new(ErrorKind::Io(io_error.kind()), format!("{}", io_error))
    }
}

impl From<Error> for std::io::Error {
    /// Transform this error to a std io error
    fn from(error: Error) -> std::io::Error {
        let kind = match error.kind {
            ErrorKind::NoController => std::io::ErrorKind::NotFound,
            ErrorKind::InvalidInput => std::io::ErrorKind::InvalidInput,
            ErrorKind::UnknownZWave => std::io::ErrorKind::InvalidData,
            ErrorKind::NotImplemented => std::io::ErrorKind::Other,
            ErrorKind::Io(kind) => kind,
        };

        std::io::Error::new(kind, error.description)
    }
}

impl From<serial::Error> for Error {
    /// Transform from a serial error
    fn from(ser_error: serial::Error) -> Error {
        use std::error::Error;

        let kind = match ser_error.kind() {
            serial::ErrorKind::NoDevice => ErrorKind::NoController,
            serial::ErrorKind::InvalidInput => ErrorKind::InvalidInput,
            serial::ErrorKind::Io(kind) => ErrorKind::Io(kind),
        };

        crate::error::Error::new(kind, ser_error.description())
    }
}
