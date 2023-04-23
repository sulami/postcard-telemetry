#![no_std]

//! This library contains various utilities that can be used for
//! embedded systems, particularly autonomous vehicles and other kinds
//! of robots.
//!
//! The feature set and implementations are heavily opinionated.

#[cfg(feature = "print")]
pub use libc_print::std_name as print;

pub mod filter;
pub mod kalman;
pub mod ring;
