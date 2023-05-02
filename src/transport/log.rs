//! std versions of log messages for hosts
//!
//! These are mirrors of the embedded versions, but use owned data
//! structures for easier decoding.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// A log message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Log {
    level: Level,
    message: String,
    parameters: HashMap<String, LogParameter>,
}

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
    String(String),
    Float(f32),
    Integer(i32),
    UnsignedInteger(u32),
}

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

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::{decode, encode};
    use crate::log as embedded;

    #[test]
    fn test_can_decode_embedded_version() {
        let embedded_log = embedded::Log::info("Hullo, {name}")
            .with_field("name", "Bob")
            .unwrap();
        let decoded = encode(&embedded_log, &mut [0; 128])
            .and_then(|buf| decode::<Log>(buf))
            .unwrap();
        assert_eq!(format!("{embedded_log}"), format!("{decoded}"));
    }
}
