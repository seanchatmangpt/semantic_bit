//! Relation64 field.
//!
//! This module defines the Relation64 semantic bitfield.
//!
//! # Field meanings
//!
//! The Relation64 field carries 64 semantic bits of relational meaning:
//!
//! - `PARENT`
//! - `CHILD`
//! - `SIBLING`
//! - `DEPENDS_ON`
//!
//! # Example
//!
//! ```
//! use semantic_bit::relation64::Relation64;
//! use semantic_bit::access::Presence;
//!
//! let field = Relation64::empty()
//!     .with(Relation64::PARENT)
//!     .with(Relation64::DEPENDS_ON);
//!
//! assert_eq!(field.carries(Relation64::DEPENDS_ON), Presence::Present);
//! assert_eq!(field.carries(Relation64::CHILD), Presence::Absent);
//! ```

#![allow(dead_code)]

use crate::access::Presence;

/// Bounded field for Relation64.
///
/// The Relation64 field carries 64 semantic bits of relational meaning.
///
/// # Examples
///
/// ```
/// use semantic_bit::relation64::Relation64;
/// use semantic_bit::access::Presence;
///
/// let field = Relation64::empty()
///     .with(Relation64::CHILD)
///     .with(Relation64::SIBLING);
///
/// assert_eq!(field.carries(Relation64::SIBLING), Presence::Present);
/// assert_eq!(field.carries(Relation64::PARENT), Presence::Absent);
/// ```
#[repr(transparent)]
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct Relation64(u64);

impl Relation64 {
    /// Position meaning: parent relation.
    pub const PARENT: u64 = 1 << 0;

    /// Position meaning: child relation.
    pub const CHILD: u64 = 1 << 1;

    /// Position meaning: sibling relation.
    pub const SIBLING: u64 = 1 << 2;

    /// Position meaning: dependency relation.
    pub const DEPENDS_ON: u64 = 1 << 3;

    /// Create an empty Relation64 field.
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
