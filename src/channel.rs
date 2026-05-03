//! Communication channel fields.
//!
//! This module defines the fields used to manage the state of
//! communication channels between system components.

#![allow(dead_code)]

use crate::access::Presence;

/// Bounded field for channel state management.
///
/// The channel field carries meanings related to the connectivity and
/// readiness of a transport channel.
#[repr(transparent)]
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct ChannelField(u8);

impl ChannelField {
    /// Position meaning: the channel is physically connected.
    pub const CONNECTED: u8 = 1 << 0;

    /// Position meaning: the channel is ready to transmit data.
    pub const READY: u8 = 1 << 1;

    /// Position meaning: the channel is currently busy.
    pub const BUSY: u8 = 1 << 2;

    /// Position meaning: the channel has encountered a transport error.
    pub const ERROR: u8 = 1 << 3;

    /// Create an empty channel field.
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
