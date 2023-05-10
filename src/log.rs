//! Logging utilities.
//!
//! A log message. Log messages are kept as format strings and
//! parameters, and are assembled on the other side. Credit for this
//! idea goes to Ferrous System's defmt.
//!
//! Use a log message like this:
//!
//! ```
//! # use postcard_telemetry::log::Log;
//! # fn main() -> Result<(), postcard_telemetry::error::Error> {
//! let message = Log::info("The answer is {answer}")
//!     .with_field("answer", 42)?;
//! # Ok(())
//! # }
//! ```
//!
//! Log messages can have up to 8 named parameters bound. If trying to
//! bind a ninth parameter, the [`Error::Saturated`] error is
//! returned.

use heapless::LinearMap;
use serde::{Deserialize, Serialize};

use crate::error::Error;

/// A log message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Log {
    level: Level,
    message: &'static str,
    parameters: LinearMap<&'static str, LogParameter, 8>,
}

impl Log {
    /// Create a new log message.
    pub fn new(level: Level, message: &'static str) -> Self {
        Self {
            level,
            message,
            parameters: LinearMap::new(),
        }
    }

    /// Create a new log message with the [`Level::Debug`] level.
    pub fn debug(message: &'static str) -> Self {
        Self::new(Level::Debug, message)
    }

    /// Create a new log message with the [`Level::Info`] level.
    pub fn info(message: &'static str) -> Self {
        Self::new(Level::Info, message)
    }

    /// Create a new log message with the [`Level::Warning`] level.
    pub fn warning(message: &'static str) -> Self {
        Self::new(Level::Warning, message)
    }

    /// Create a new log message with the [`Level::Error`] level.
    pub fn error(message: &'static str) -> Self {
        Self::new(Level::Error, message)
    }

    /// Add a field to the log message. This operation can fail if the
    /// log message is already saturated.
    pub fn with_field(
        mut self,
        name: &'static str,
        parameter: impl Into<LogParameter>,
    ) -> Result<Self, Error> {
        self.parameters
            .insert(name, parameter.into())
            .map(|_| self)
            .map_err(|_| Error::Saturated)
    }
}

#[cfg(feature = "std")]
impl std::fmt::Display for Log {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut message = self.message.to_string();
        let mut parameters = self.parameters.iter();
        while let Some((name, parameter)) = parameters.next() {
            match parameter {
                LogParameter::String(s) => {
                    message = message.replace(&format!("{{{}}}", name), s);
                }
                LogParameter::Float(v) => {
                    message = message.replace(&format!("{{{}}}", name), &format!("{}", v));
                }
                LogParameter::Integer(v) => {
                    message = message.replace(&format!("{{{}}}", name), &format!("{}", v));
                }
                LogParameter::UnsignedInteger(v) => {
                    message = message.replace(&format!("{{{}}}", name), &format!("{}", v));
                }
            }
        }
        write!(f, "[{}] {}", self.level, message)
    }
}

/// A log message level.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Level {
    Debug,
    Info,
    Warning,
    Error,
}

#[cfg(feature = "std")]
impl std::fmt::Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Level::Debug => write!(f, "DEBUG"),
            Level::Info => write!(f, "INFO"),
            Level::Warning => write!(f, "WARNING"),
            Level::Error => write!(f, "ERROR"),
        }
    }
}

/// A log message parameter.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LogParameter {
    String(&'static str),
    Float(f32),
    Integer(i32),
    UnsignedInteger(u32),
}

#[cfg(feature = "std")]
impl std::fmt::Display for LogParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogParameter::String(s) => write!(f, "{}", s),
            LogParameter::Float(v) => write!(f, "{}", v),
            LogParameter::Integer(v) => write!(f, "{}", v),
            LogParameter::UnsignedInteger(v) => write!(f, "{}", v),
        }
    }
}

impl From<&'static str> for LogParameter {
    fn from(s: &'static str) -> Self {
        Self::String(s)
    }
}

impl From<f32> for LogParameter {
    fn from(v: f32) -> Self {
        Self::Float(v)
    }
}

impl From<i32> for LogParameter {
    fn from(v: i32) -> Self {
        Self::Integer(v)
    }
}

impl From<u32> for LogParameter {
    fn from(v: u32) -> Self {
        Self::UnsignedInteger(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "std")]
    fn test_display() {
        assert_eq!(format!("{}", LogParameter::String("foo")), "foo");
        assert_eq!(format!("{}", LogParameter::Float(1.0)), "1");
        assert_eq!(format!("{}", LogParameter::Integer(1)), "1");
        assert_eq!(format!("{}", LogParameter::UnsignedInteger(1)), "1");
    }

    #[test]
    fn test_with_field() {
        let message = Log::new(Level::Info, "foo {bar}")
            .with_field("bar", "baz")
            .unwrap();
        assert_eq!(message.parameters.len(), 1);
        assert_eq!(message.parameters["bar"], LogParameter::String("baz"));
    }

    #[test]
    fn test_with_field_saturated() -> Result<(), Error> {
        let message = Log::new(Level::Info, "foo {bar}")
            .with_field("1", "baz")?
            .with_field("2", "baz")?
            .with_field("3", "baz")?
            .with_field("4", "baz")?
            .with_field("5", "baz")?
            .with_field("6", "baz")?
            .with_field("7", "baz")?
            .with_field("8", "baz")?;
        assert!(matches!(
            message.with_field("9", "quox").unwrap_err(),
            Error::Saturated
        ));
        Ok(())
    }
}
