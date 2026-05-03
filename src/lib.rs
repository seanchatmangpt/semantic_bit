//! The Semantic Bit: A Field Manual for Bounded Meaning in Rust
//!
//! This library provides the executable notation for the field discipline
//! introduced in the book.

pub mod access;
pub mod control;
pub mod dispatch;
pub mod channel;
pub mod journal;
pub mod checkpoint;
pub mod restart;
pub mod contract;

pub mod status8;
pub mod condition_code8;
pub mod operation64;
pub mod relation64;
pub mod cog8;
pub mod construct8;
