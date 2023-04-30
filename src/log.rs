//! Logging utilities.
//!
//! A log message. Log messages are kept as format strings and
//! parameters, and are assembled on the other side. Credit for this
//! idea goes to Ferrous System's defmt.

use heapless::LinearMap;
use serde::Serialize;

/// A log message.
#[derive(Debug, Clone, Serialize)]
pub struct LogMessage {
    level: Level,
    message: &'static str,
    parameters: LinearMap<&'static str, LogParameter, 8>,
}

impl LogMessage {
    /// Create a new log message.
    pub fn new(
        level: Level,
        message: &'static str,
        parameters: LinearMap<&'static str, LogParameter, 8>,
    ) -> Self {
        Self {
            level,
            message,
            parameters,
        }
    }
}

#[cfg(feature = "std")]
impl std::fmt::Display for LogMessage {
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
            }
        }
        write!(f, "[{}] {}", self.level, message)
    }
}

/// A log message level.
#[derive(Debug, Clone, Copy, Serialize)]
pub enum Level {
    Info,
    Warning,
    Error,
}

#[cfg(feature = "std")]
impl std::fmt::Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Level::Info => write!(f, "INFO"),
            Level::Warning => write!(f, "WARNING"),
            Level::Error => write!(f, "ERROR"),
        }
    }
}

/// A log message parameter.
#[derive(Debug, Clone, Copy, Serialize)]
pub enum LogParameter {
    String(&'static str),
    Float(f32),
    Integer(i32),
}

#[cfg(feature = "std")]
impl std::fmt::Display for LogParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogParameter::String(s) => write!(f, "{}", s),
            LogParameter::Float(v) => write!(f, "{}", v),
            LogParameter::Integer(v) => write!(f, "{}", v),
        }
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
    }
}