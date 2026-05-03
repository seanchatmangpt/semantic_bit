//! Operation64 field.
//!
//! This module defines the Operation64 semantic bitfield.
//!
//! # Field meanings
//!
//! The Operation64 field carries 64 semantic bits of operational meaning:
//!
//! - `EXECUTE`
//! - `COMMIT`
//! - `ROLLBACK`
//! - `AUDIT`
//!
//! # Example
//!
//! ```
//! use semantic_bit::operation64::Operation64;
//! use semantic_bit::access::Presence;
//!
//! let field = Operation64::empty()
//!     .with(Operation64::EXECUTE)
//!     .with(Operation64::COMMIT);
//!
//! assert_eq!(field.carries(Operation64::COMMIT), Presence::Present);
//! assert_eq!(field.carries(Operation64::ROLLBACK), Presence::Absent);
//! ```

#![allow(dead_code)]

use crate::access::Presence;

/// Bounded field for Operation64.
///
/// The Operation64 field carries 64 semantic bits of operational meaning.
///
/// # Examples
///
/// ```
/// use semantic_bit::operation64::Operation64;
/// use semantic_bit::access::Presence;
///
/// let field = Operation64::empty()
///     .with(Operation64::ROLLBACK)
///     .with(Operation64::AUDIT);
///
/// assert_eq!(field.carries(Operation64::AUDIT), Presence::Present);
/// assert_eq!(field.carries(Operation64::EXECUTE), Presence::Absent);
/// ```
#[repr(transparent)]
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct Operation64(u64);

impl Operation64 {
    /// Position meaning: execute operation.
    pub const EXECUTE: u64 = 1 << 0;

    /// Position meaning: commit operation.
    pub const COMMIT: u64 = 1 << 1;

    /// Position meaning: rollback operation.
    pub const ROLLBACK: u64 = 1 << 2;

    /// Position meaning: audit operation.
    pub const AUDIT: u64 = 1 << 3;

    /// Create an empty Operation64 field.
    pub const fn empty() -> Self {
        Self(0)
    }

    /// Return a field that carries the given named position.
    pub const fn with(mut self, position: u64) -> Self {
        self.0 |= position;
        self
    }

    /// Report whether the field carries a named position.
    pub const fn carries(self, position: u64) -> Presence {
        if self.0 & position != 0 {
            Presence::Present
        } else {
            Presence::Absent
        }
    }
}
