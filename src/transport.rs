//! Transport layer for sending and receiving messages.
//!
//! The transport layer is designed to be used with a
//! [COBS](https://en.wikipedia.org/wiki/Consistent_Overhead_Byte_Stuffing)
//! encoding layer.

use serde::Serialize;

/// Serialize an item into a buffer for transmission.
pub fn serialize(item: &impl Serialize, buf: &mut [u8]) -> bool {
    postcard::to_slice_cobs(item, buf).is_ok()
}

#[cfg(feature = "std")]
pub fn deserialize<'a, T>(buf: &'a mut [u8]) -> Result<T, postcard::Error>
where
    T: serde::Deserialize<'a>,
{
    postcard::from_bytes_cobs(buf)
}

#[cfg(test)]
mod tests {
    use super::*;

    use postcard::from_bytes_cobs;

    #[test]
    fn test_round_trip() {
        let mut buf = [0u8; 1024];

        let map = [("foo", 1.0f32), ("bar", 2.0), ("baz", 3.0)];
        assert!(serialize(&map, &mut buf));
        let result = from_bytes_cobs::<[(&str, f32); 3]>(&mut buf);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), map);
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_std_round_trip() {
        let mut buf = [0u8; 1024];

        let map = [("foo", 1.0f32), ("bar", 2.0), ("baz", 3.0)];
        assert!(serialize(&map, &mut buf));
        let result = deserialize::<[(&str, f32); 3]>(&mut buf);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), map);
    }
}
