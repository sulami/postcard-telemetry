#![cfg_attr(not(feature = "std"), no_std)]

//! This library contains a logging and telemetry format that can be
//! used for embedded systems, particularly autonomous vehicles and
//! other kinds of robots.
//!
//! The feature set and implementations are somewhat opinionated:
//!
//! - This is designed to work without the use of global variables
//! - Telemetry data types are 32-bit
//!
//! On host systems, this library can use `std` via the `std` feature,
//! which enables shared functionality such as log decoding.

#[cfg(feature = "std")]
extern crate core;

pub mod error;
pub mod log;
pub mod telemetry;
pub mod transport;
