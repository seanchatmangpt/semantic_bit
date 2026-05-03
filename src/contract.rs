//! Representation contract fields.
//!
//! This module defines the fields used to verify that a record
//! complies with its admitted representation contract.

#![allow(dead_code)]

use crate::access::Presence;

/// Bounded field for representation contract management.
///
/// The contract field carries meanings related to the schema and
/// versioning of an operational record.
#[repr(transparent)]
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct ContractField(u8);

impl ContractField {
    /// Position meaning: the record matches the current admitted schema.
    pub const ADMITTED: u8 = 1 << 0;

    /// Position meaning: the record is from a deprecated schema version.
    pub const DEPRECATED: u8 = 1 << 1;

    /// Position meaning: the record has been verified by a representation check.
    pub const VERIFIED: u8 = 1 << 2;

    /// Create an empty contract field.
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

    /// Return the raw field value.
    pub const fn raw(self) -> u8 {
        self.0
    }
}
