//! ConditionCode8 field.
//!
//! This module defines the ConditionCode8 semantic bitfield.
//!
//! # Field meanings
//!
//! The ConditionCode8 field carries 8 semantic bits of condition codes:
//!
//! - `ZERO`
//! - `CARRY`
//! - `OVERFLOW`
//! - `NEGATIVE`
//!
//! # Example
//!
//! ```
//! use semantic_bit::condition_code8::ConditionCode8;
//! use semantic_bit::access::Presence;
//!
//! let field = ConditionCode8::empty()
//!     .with(ConditionCode8::ZERO)
//!     .with(ConditionCode8::CARRY);
//!
//! assert_eq!(field.carries(ConditionCode8::CARRY), Presence::Present);
//! assert_eq!(field.carries(ConditionCode8::NEGATIVE), Presence::Absent);
//! ```

#![allow(dead_code)]

use crate::access::Presence;

/// Bounded field for ConditionCode8.
///
/// The ConditionCode8 field carries 8 semantic bits of condition codes.
///
/// # Examples
///
/// ```
/// use semantic_bit::condition_code8::ConditionCode8;
/// use semantic_bit::access::Presence;
///
/// let field = ConditionCode8::empty()
///     .with(ConditionCode8::OVERFLOW)
///     .with(ConditionCode8::NEGATIVE);
///
/// assert_eq!(field.carries(ConditionCode8::OVERFLOW), Presence::Present);
/// assert_eq!(field.carries(ConditionCode8::ZERO), Presence::Absent);
/// ```
#[repr(transparent)]
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct ConditionCode8(u8);

impl ConditionCode8 {
    /// Position meaning: condition zero.
    pub const ZERO: u8 = 1 << 0;

    /// Position meaning: condition carry.
    pub const CARRY: u8 = 1 << 1;

    /// Position meaning: condition overflow.
    pub const OVERFLOW: u8 = 1 << 2;

    /// Position meaning: condition negative.
    pub const NEGATIVE: u8 = 1 << 3;

    /// Create an empty ConditionCode8 field.
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
