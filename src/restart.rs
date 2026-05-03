//! System restart fields.
//!
//! This module defines the fields used to manage the system's
//! recovery and initialization process.

#![allow(dead_code)]

use crate::access::Presence;

/// Bounded field for restart and recovery management.
///
/// The restart field carries meanings related to the system's
/// transition from offline to operational.
#[repr(transparent)]
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct RestartField(u8);

impl RestartField {
    /// Position meaning: the system is performing a cold start (no state).
    pub const COLD: u8 = 1 << 0;

    /// Position meaning: the system is performing a warm start (from checkpoint).
    pub const WARM: u8 = 1 << 1;

    /// Position meaning: the system is currently replaying the journal.
    pub const REPLAYING: u8 = 1 << 2;

    /// Position meaning: the system has reached the operational state.
    pub const READY: u8 = 1 << 3;

    /// Create an empty restart field.
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
