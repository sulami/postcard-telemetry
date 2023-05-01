//! Telemetry and logging
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

use heapless::LinearMap;
use serde::Serialize;

use crate::error::Error;

/// A global telemetry reporter with a static size of data points.
/// Once the reporter capacity has been reached, additional data
/// points will be silently dropped.
pub struct TelemetryReporter<const N: usize> {
    telemetry: TelemetryFrame<N>,
}

impl<const N: usize> TelemetryReporter<N> {
    /// Create a new telemetry reporter.
    pub const fn new() -> Self {
        Self {
            telemetry: LinearMap::new(),
        }
    }

    /// Record a data point. Returns `true` if recording has been
    /// successful. Will return `false` if recorder capacity has been
    /// reached, and not record the supplied value.
    pub fn record(
        &mut self,
        name: &'static str,
        value: impl Into<DataPoint> + Copy,
    ) -> Result<(), Error> {
        self.telemetry
            .insert(name, value.into())
            .map(|_| ())
            .map_err(|_| Error::Saturated)
    }

    /// Report the current telemetry data. This will clear the
    /// telemetry data.
    #[must_use]
    pub fn report(&mut self) -> TelemetryFrame<N> {
        let rv = self.telemetry.clone();
        self.telemetry.clear();
        rv
    }
}

/// A telemetry frame.
pub type TelemetryFrame<const N: usize> = LinearMap<&'static str, DataPoint, N>;

/// A single data point.
#[derive(Debug, Clone, Copy, Serialize, PartialEq)]
pub enum DataPoint {
    F32(f32),
    I32(i32),
    U32(u32),
}

impl From<f32> for DataPoint {
    fn from(value: f32) -> Self {
        Self::F32(value)
    }
}

impl From<i32> for DataPoint {
    fn from(value: i32) -> Self {
        Self::I32(value)
    }
}

impl From<u32> for DataPoint {
    fn from(value: u32) -> Self {
        Self::U32(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roundtrip() {
        let mut reporter = TelemetryReporter::<1>::new();
        reporter.record("tau", 6.12).unwrap();
        let result = reporter.report();
        assert_eq!(*result.get("tau").unwrap(), 6.12.into());
    }

    #[test]
    fn test_graceful_when_full() -> Result<(), Error> {
        let mut reporter = TelemetryReporter::<1>::new();
        reporter.record("tau", 6.12)?;
        assert!(matches!(reporter.record("pi", 3.14), Err(Error::Saturated)));
        assert_eq!(reporter.telemetry.len(), 1);
        assert!(reporter.telemetry.contains_key(&"tau"));
        Ok(())
    }

    #[test]
    fn test_clears_on_report() {
        let mut reporter = TelemetryReporter::<1>::new();
        reporter.record("tau", 6.12).unwrap();
        let _ = reporter.report();
        assert!(reporter.telemetry.is_empty());
    }
}
