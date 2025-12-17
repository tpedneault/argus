use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    BufferTooShort,
    BadVersion(u8),
    LengthMismatch {
        expected: usize,
        actual: usize,
    },
    InvalidField(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::BufferTooShort =>
                write!(f, "buffer too short for CCSDS space packet"),
            Error::BadVersion(v) =>
                write!(f, "unsupported CCSDS packet version {}", v),
            Error::LengthMismatch { expected, actual } =>
                write!(
                    f,
                    "packet length mismatch (expected {}, got {})",
                    expected, actual
                ),
            Error::InvalidField(name) =>
                write!(f, "invalid value for field '{}'", name),
        }
    }
}

impl std::error::Error for Error {}
