//! Sequential journal fields and records.
//!
//! This module defines the fields and records used to maintain a
//! permanent, sequential log of operational consequences.

#![allow(dead_code)]

use crate::access::Presence;

/// Bounded field for journal state management.
///
/// The journal field carries meanings related to the integrity and
/// lifecycle of a journal entry.
#[repr(transparent)]
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct JournalField(u8);

impl JournalField {
    /// Position meaning: the entry has been cryptographically sealed.
    pub const SEALED: u8 = 1 << 0;

    /// Position meaning: the entry marks the start of a new rotation.
    pub const ROTATED: u8 = 1 << 1;

    /// Position meaning: the entry has been synced to durable storage.
    pub const SYNCED: u8 = 1 << 2;

    /// Position meaning: the entry is a checkpoint (see Chapter 13).
    pub const CHECKPOINT: u8 = 1 << 3;

    /// Create an empty journal field.
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

/// Fixed-width journal entry header.
///
/// A journal entry binds a sequence number, a timestamp, a record type,
/// and a journal field to an opaque payload (or a pointer to one).
#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct JournalHeader {
    /// Global monotonic sequence number.
    pub sequence: u64,

    /// Epoch timestamp of the entry.
    pub epoch: u64,

    /// Type of the payload (e.g., 1 for Receipt, 2 for Control).
    pub record_type: u32,

    /// Length of the following payload in bytes.
    pub payload_len: u32,

    /// Journal state meanings.
    pub field: JournalField,
}

impl JournalHeader {
    /// Create a new journal header.
    pub const fn new(
        sequence: u64,
        epoch: u64,
        record_type: u32,
        payload_len: u32,
        field: JournalField,
    ) -> Self {
        Self {
            sequence,
            epoch,
            record_type,
            payload_len,
            field,
        }
    }
}
