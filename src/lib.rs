#![no_std]

//! This library contains various utilities that can be used for
//! embedded systems, particularly autonomous vehicles and other kinds
//! of robots.
//!
//! The feature set and implementations are somewhat opinionated.
//! Opinions include:
//!
//! - This is designed to work without the use of global variables.
//! - The primary numerical data type is [`f32`], which is precise
//!   enough for most applications.

pub use defmt;

#[cfg(feature = "print")]
pub use libc_print::std_name as print;

pub mod filter;
pub mod kalman;
pub mod ring;
pub mod telemetry;
