//! Cog8 field.
//!
//! This module defines the Cog8 semantic bitfield.
//!
//! # Field meanings
//!
//! The Cog8 field carries 8 semantic bits for cognitive/computational nodes:
//!
//! - `PRIMARY`
//! - `SECONDARY`
//! - `IDLE`
//! - `FAULT`
//!
//! # Example
//!
//! ```
//! use semantic_bit::cog8::Cog8;
//! use semantic_bit::access::Presence;
//!
//! let field = Cog8::empty()
//!     .with(Cog8::PRIMARY)
//!     .with(Cog8::IDLE);
//!
//! assert_eq!(field.carries(Cog8::PRIMARY), Presence::Present);
//! assert_eq!(field.carries(Cog8::FAULT), Presence::Absent);
//! ```

#![allow(dead_code)]

use crate::access::Presence;

/// Bounded field for Cog8.
///
/// The Cog8 field carries 8 semantic bits for cognitive/computational nodes.
///
/// # Examples
///
/// ```
/// use semantic_bit::cog8::Cog8;
/// use semantic_bit::access::Presence;
///
/// let field = Cog8::empty()
///     .with(Cog8::SECONDARY)
///     .with(Cog8::FAULT);
///
/// assert_eq!(field.carries(Cog8::FAULT), Presence::Present);
/// assert_eq!(field.carries(Cog8::IDLE), Presence::Absent);
/// ```
#[repr(transparent)]
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct Cog8(u8);

impl Cog8 {
    /// Position meaning: primary cog.
    pub const PRIMARY: u8 = 1 << 0;

    /// Position meaning: secondary cog.
    pub const SECONDARY: u8 = 1 << 1;

    /// Position meaning: idle cog.
    pub const IDLE: u8 = 1 << 2;

    /// Position meaning: fault cog.
    pub const FAULT: u8 = 1 << 3;

    /// Create an empty Cog8 field.
    pub const fn empty() -> Self {
        Self(0)
    }

    /// Return a field that carries the given named position.
    pub const fn with(mut self, position: u8) -> Self {
        self.0 |= position;
        self
    }

    /// Report whether the field carries a named position.
    pub const fn carries(self, position: u8) -> Presence {
        if self.0 & position != 0 {
            Presence::Present
        } else {
            Presence::Absent
        }
    }
}
