//! State checkpoint fields and records.
//!
//! This module defines the fields and records used to summarize the
//! operational state of the system at a specific sequence number.

#![allow(dead_code)]

use crate::access::Presence;

/// Bounded field for checkpoint state management.
///
/// The checkpoint field carries meanings related to the validity and
/// scope of a state summary.
#[repr(transparent)]
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct CheckpointField(u8);

impl CheckpointField {
    /// Position meaning: the checkpoint represents a complete system state.
    pub const FULL: u8 = 1 << 0;

    /// Position meaning: the checkpoint represents an incremental state.
    pub const INCREMENTAL: u8 = 1 << 1;

    /// Position meaning: the checkpoint has been verified against the journal.
    pub const VERIFIED: u8 = 1 << 2;

    /// Create an empty checkpoint field.
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

/// Fixed-width checkpoint record.
///
/// A checkpoint record binds a sequence number to a state summary
/// (e.g., a hash or a small set of counters).
#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct CheckpointRecord {
    /// The journal sequence number where this checkpoint was taken.
    pub last_sequence: u64,

    /// Total number of grant decisions in history.
    pub total_grants: u64,

    /// Total number of deny decisions in history.
    pub total_denies: u64,

    /// Checkpoint state meanings.
    pub field: CheckpointField,
}

impl CheckpointRecord {
    /// Create a new checkpoint record.
    pub const fn new(
        last_sequence: u64,
        total_grants: u64,
        total_denies: u64,
        field: CheckpointField,
    ) -> Self {
        Self {
            last_sequence,
            total_grants,
            total_denies,
            field,
        }
    }
}
