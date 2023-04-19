#![no_std]

//! This library contains various utilities that can be used for
//! embedded systems, particularly autonomous vehicles and other kinds
//! of robots.
//!
//! The feature set and implementations are heavily opinionated.

pub mod filter;
pub mod kalman;
pub mod ring;
