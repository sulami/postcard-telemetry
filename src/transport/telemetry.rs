//! std versions of telemetry for hosts
//!
//! These are mirrors of the embedded versions, but use owned data
//! structures for easier decoding.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// A telemetry frame.
pub type TelemetryFrame = HashMap<String, DataPoint>;

/// A single data point.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum DataPoint {
    F32(f32),
    I32(i32),
    U32(u32),
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::{decode, encode};
    use crate::telemetry as embedded;

    #[test]
    fn test_can_decode_embedded_version() {
        let mut embedded_frame = embedded::TelemetryFrame::<8>::new();
        embedded_frame
            .insert("foo", embedded::DataPoint::F32(1.0))
            .unwrap();
        embedded_frame
            .insert("bar", embedded::DataPoint::I32(2))
            .unwrap();
        embedded_frame
            .insert("baz", embedded::DataPoint::U32(3))
            .unwrap();

        let decoded = encode(&embedded_frame, &mut [0; 128])
            .and_then(|buf| decode::<TelemetryFrame>(buf))
            .unwrap();

        assert_eq!(decoded.len(), 3);
        assert_eq!(decoded.get("foo").unwrap(), &DataPoint::F32(1.0));
        assert_eq!(decoded.get("bar").unwrap(), &DataPoint::I32(2));
        assert_eq!(decoded.get("baz").unwrap(), &DataPoint::U32(3));
    }
}
