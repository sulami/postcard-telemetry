//! Telemetry gathering and reporting
//!
//! This module provides functionality to gather and report telemetry
//! data. The data format used is
//! [postcard](https://github.com/jamesmunns/postcard), which is an
//! embedded-friendly serial format. It is further wrapped in a layer
//! of
//! [COBS](https://en.wikipedia.org/wiki/Consistent_Overhead_Byte_Stuffing)
//! to deal with potentially unreliable connections.
//!
//! The model this is designed for works like this:
//! 1. Setup a [`TelemetryReporter`].
//! 2. During every tick, capture data points as they come up.
//! 3. Towards the end of every tick, call
//!    [`TelemetryReporter::report`] to format the data, and push it
//!    out to mission control. This also clears all data.
//!
//! Ideally every data point should be reported on every tick.
//!
//! As other modules in this crate, this is somewhat opinionated and
//! only supports [`f32`] data points for now.

use heapless::LinearMap;
use postcard::to_slice_cobs;

/// A global telemetry reporter with a static size of data points.
/// Once the reporter capacity has been reached, additional data
/// points will be silently dropped.
pub struct TelemetryReporter<'a, const N: usize> {
    telemetry: LinearMap<&'a str, f32, N>,
}

impl<'a, const N: usize> TelemetryReporter<'a, N> {
    pub const fn new() -> Self {
        Self {
            telemetry: LinearMap::new(),
        }
    }

    /// Record a data point. Returns `true` if recording has been
    /// successful. Will return `false` if recorder capacity has been
    /// reached, and not record the supplied value.
    #[must_use]
    pub fn record(&mut self, name: &'a str, value: f32) -> bool {
        let result = self.telemetry.insert(name, value);
        result.is_ok()
    }

    /// Write the recorded telemetry data to `buf`, encoded for
    /// transmission to mission control. Also zero out all telemetry
    /// stored in this tick. Returns `false` if serialisation failed,
    /// most likely due to a buffer that's too small. Does not clear
    /// records if reporting failed, so that it can be retried with a
    /// larger buffer.
    #[must_use]
    pub fn report(&mut self, buf: &mut [u8]) -> bool {
        let ser_result = to_slice_cobs(&self.telemetry, buf);
        if ser_result.is_ok() {
            self.telemetry.clear();
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use postcard::from_bytes_cobs;

    #[test]
    fn test_roundtrip() {
        let mut reporter = TelemetryReporter::<1>::new();
        reporter.record("tau", 6.12);
        let mut out = [0u8; 64];
        reporter.report(&mut out);
        let parsed: LinearMap<&str, f32, 1> = from_bytes_cobs(&mut out).unwrap();
        assert_eq!(parsed.get("tau"), Some(&6.12));
    }

    #[test]
    fn test_graceful_when_full() {
        let mut reporter = TelemetryReporter::<1>::new();
        assert!(reporter.record("tau", 6.12));
        assert!(!reporter.record("pi", 3.14));
        assert_eq!(reporter.telemetry.len(), 1);
        assert!(reporter.telemetry.contains_key(&"tau"));
    }

    #[test]
    fn test_clears_on_report() {
        let mut reporter = TelemetryReporter::<1>::new();
        reporter.record("tau", 6.12);
        let mut out = [0u8; 64];
        assert!(reporter.report(&mut out));
        assert_eq!(reporter.telemetry.len(), 0);
    }

    #[test]
    fn test_does_not_clear_on_failure() {
        let mut reporter = TelemetryReporter::<1>::new();
        reporter.record("tau", 6.12);
        let mut out = [0u8; 1];
        assert!(!reporter.report(&mut out));
        assert_eq!(reporter.telemetry.len(), 1);
    }
}
