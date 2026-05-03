//! Operational control fields and records.
//!
//! This module defines the records used to command system actuators.
//!
//! While a receipt is a record of a past decision, a control record is an
//! instruction for a future action.

#![allow(dead_code)]

use crate::access::Presence;

/// Bounded field for operational control.
///
/// The control field carries meanings related to the execution of a command.
#[repr(transparent)]
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct ControlField(u8);

impl ControlField {
    /// Position meaning: the command should be executed immediately.
    pub const EXECUTE: u8 = 1 << 0;

    /// Position meaning: the command should be retried if it fails.
    pub const RETRY: u8 = 1 << 1;

    /// Position meaning: the command is part of a batch.
    pub const BATCH: u8 = 1 << 2;

    /// Position meaning: the command requires a physical acknowledgment.
    pub const ACK_REQUIRED: u8 = 1 << 3;

    /// Create an empty control field.
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

/// Fixed-width control record.
///
/// A control record binds a target, an instruction (condition code), and a
/// control field into a single command.
#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct ControlRecord {
    /// The target component to be controlled (e.g., Door ID).
    pub target_id: u32,

    /// The instruction to be executed (often an AccessCondition as u8).
    pub instruction: u8,

    /// The sequence number of the authorizing receipt.
    pub authority_sequence: u64,

    /// Operational control meanings.
    pub field: ControlField,
}

impl ControlRecord {
    /// Create a new control record.
    pub const fn new(
        target_id: u32,
        instruction: u8,
        authority_sequence: u64,
        field: ControlField,
    ) -> Self {
        Self {
            target_id,
            instruction,
            authority_sequence,
            field,
        }
    }
}
