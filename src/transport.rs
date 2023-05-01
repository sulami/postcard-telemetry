//! Transport layer for sending and receiving messages.
//!
//! The transport layer is designed to be used with a
//! [COBS](https://en.wikipedia.org/wiki/Consistent_Overhead_Byte_Stuffing)
//! encoding layer.

use serde::Serialize;

use crate::error::Error;

/// Serialize an item into a buffer for transmission.
pub fn encode<'b>(item: &impl Serialize, buf: &'b mut [u8]) -> Result<&'b mut [u8], Error> {
    postcard::to_slice_cobs(item, buf).map_err(|_| Error::BufferTooSmall)
}

#[cfg(feature = "std")]
/// Deserialize an item from a buffer.
pub fn decode<'a, T>(buf: &'a mut [u8]) -> Result<T, Error>
where
    T: serde::Deserialize<'a>,
{
    postcard::from_bytes_cobs(buf).map_err(|_| Error::InvalidData)
}

#[cfg(test)]
mod tests {
    use super::*;

    use postcard::from_bytes_cobs;

    #[test]
    fn test_round_trip() {
        let mut buf = [0u8; 1024];

        let map = [("foo", 1.0f32), ("bar", 2.0), ("baz", 3.0)];
        assert!(encode(&map, &mut buf).is_ok());
        let result = from_bytes_cobs::<[(&str, f32); 3]>(&mut buf);
        assert_eq!(result.unwrap(), map);
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_std_round_trip() {
        let mut buf = [0u8; 1024];

        let map = [("foo", 1.0f32), ("bar", 2.0), ("baz", 3.0)];
        assert!(encode(&map, &mut buf).is_ok());
        let result = decode::<[(&str, f32); 3]>(&mut buf);
        assert_eq!(result.unwrap(), map);
    }
}
