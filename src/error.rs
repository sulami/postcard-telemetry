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
