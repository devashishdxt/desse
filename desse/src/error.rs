use core::fmt::{Display, Formatter};

#[cfg(feature = "dynamic")]
use alloc::string::FromUtf8Error;

/// Alias of `Result` objects that return [`Error`](self::Error)
pub type Result<T> = core::result::Result<T, Error>;

/// An opaque error type, used for all errors in this crate
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    /// Returns [`ErrorKind`](self::ErrorKind) of current error
    #[inline]
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }
}

impl From<ErrorKind> for Error {
    #[inline]
    fn from(kind: ErrorKind) -> Self {
        Self { kind }
    }
}

impl Display for Error {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

/// Different variants of possible errors
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ErrorKind {
    /// Returned when deserialization from bytes to char fails.
    InvalidChar,
    /// Returned when deserialization from bytes to str fails.
    #[cfg(feature = "dynamic")]
    InvalidStr,
    /// Returned when input slice is of invalid length.
    #[cfg(feature = "dynamic")]
    InvalidSliceLength,
    /// Returned when input slice cannot be de-serialized into given type.
    #[cfg(feature = "dynamic")]
    InvalidInput,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            ErrorKind::InvalidChar => write!(f, "Deserialization from bytes to char failed"),
            #[cfg(feature = "dynamic")]
            ErrorKind::InvalidStr => write!(f, "Deserialization from bytes to String failed"),
            #[cfg(feature = "dynamic")]
            ErrorKind::InvalidSliceLength => write!(f, "Input slice is of invalid length"),
            #[cfg(feature = "dynamic")]
            ErrorKind::InvalidInput => {
                write!(f, "input slice cannot be de-serialized into given type")
            }
        }
    }
}

#[cfg(feature = "dynamic")]
impl From<FromUtf8Error> for Error {
    #[inline]
    fn from(_: FromUtf8Error) -> Error {
        Error {
            kind: ErrorKind::InvalidStr,
        }
    }
}
