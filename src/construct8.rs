//! Construct8 field.
//!
//! This module defines the Construct8 semantic bitfield.
//!
//! # Field meanings
//!
//! The Construct8 field represents the bounded constructive delta primitive:
//!
//! - `PROPOSED`
//! - `VALIDATED`
//! - `APPLIED`
//! - `REJECTED`
//!
//! # Example
//!
//! ```
//! use semantic_bit::construct8::Construct8;
//! use semantic_bit::access::Presence;
//!
//! let field = Construct8::empty()
//!     .with(Construct8::PROPOSED)
//!     .with(Construct8::VALIDATED);
//!
//! assert_eq!(field.carries(Construct8::VALIDATED), Presence::Present);
//! assert_eq!(field.carries(Construct8::APPLIED), Presence::Absent);
//! ```

#![allow(dead_code)]

use crate::access::Presence;

/// Bounded field for Construct8.
///
/// The Construct8 field represents the bounded constructive delta primitive.
///
/// # Examples
///
/// ```
/// use semantic_bit::construct8::Construct8;
/// use semantic_bit::access::Presence;
///
/// let field = Construct8::empty()
///     .with(Construct8::PROPOSED)
///     .with(Construct8::REJECTED);
///
/// assert_eq!(field.carries(Construct8::REJECTED), Presence::Present);
/// assert_eq!(field.carries(Construct8::APPLIED), Presence::Absent);
/// ```
#[repr(transparent)]
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct Construct8(u8);

impl Construct8 {
    /// Position meaning: patch proposed.
    pub const PROPOSED: u8 = 1 << 0;

    /// Position meaning: patch validated.
    pub const VALIDATED: u8 = 1 << 1;

    /// Position meaning: patch applied.
    pub const APPLIED: u8 = 1 << 2;

    /// Position meaning: patch rejected.
    pub const REJECTED: u8 = 1 << 3;

    /// Create an empty Construct8 field.
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
