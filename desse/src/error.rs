use core::fmt::{Display, Formatter};

/// Alias of `Result` objects that return [`Error`](self::Error)
pub type Result<T> = core::result::Result<T, Error>;

/// An opaque error type, used for all errors in this crate
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    /// Returns [`ErrorKind`](self::ErrorKind) of current error
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Self { kind }
    }
}

impl Display for Error {
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
    InvalidStr,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            ErrorKind::InvalidChar => write!(f, "Deserialization from bytes to char failed"),
            ErrorKind::InvalidStr => write!(f, "Deserialization from bytes to String failed"),
        }
    }
}
