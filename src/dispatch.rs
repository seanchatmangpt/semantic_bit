//! Routing and dispatch fields.
//!
//! This module defines the fields used to route records and commands
//! through the system.

#![allow(dead_code)]

use crate::access::Presence;

/// Bounded field for dispatch and routing.
///
/// The dispatch field carries meanings related to the destination and
/// priority of a record.
#[repr(transparent)]
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct DispatchField(u8);

impl DispatchField {
    /// Position meaning: the record should be processed by the local component.
    pub const LOCAL: u8 = 1 << 0;

    /// Position meaning: the record should be dispatched to a remote component.
    pub const REMOTE: u8 = 1 << 1;

    /// Position meaning: the record should be handled with high priority.
    pub const URGENT: u8 = 1 << 2;

    /// Position meaning: the record should be journaled during dispatch.
    pub const JOURNAL: u8 = 1 << 3;

    /// Create an empty dispatch field.
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
