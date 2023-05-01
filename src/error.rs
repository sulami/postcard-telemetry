//! Errors

/// The error type returned by this library.
#[derive(Debug)]
pub enum Error {
    /// The buffer provided to a function was too small.
    BufferTooSmall,
    /// The decoded data was invalid.
    InvalidData,
    /// An internal data structure was saturated.
    Saturated,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::BufferTooSmall => write!(f, "buffer too small"),
            Self::InvalidData => write!(f, "invalid data"),
            Self::Saturated => write!(f, "saturated"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}
