//! Access admission fields.
//!
//! This module defines a small access-control field.
//!
//! The field is not introduced as a collection of booleans.
//! It is introduced as a bounded carrier of named operational meanings.
//!
//! # Field meanings
//!
//! The access field admits eight positions:
//!
//! - `BADGE_PRESENT`
//! - `BADGE_RECOGNIZED`
//! - `HOLDER_ACTIVE`
//! - `DOOR_ALLOWED`
//! - `TIME_ALLOWED`
//! - `GRANTED`
//! - `RECORDED`
//! - `REVIEW_REQUIRED`
//!
//! # Example
//!
//! ```
//! use semantic_bit::access::{AccessAttempt, AccessCondition, AccessField, Presence};
//!
//! let field = AccessField::empty()
//!     .with(AccessField::BADGE_PRESENT)
//!     .with(AccessField::BADGE_RECOGNIZED)
//!     .with(AccessField::HOLDER_ACTIVE)
//!     .with(AccessField::DOOR_ALLOWED)
//!     .with(AccessField::TIME_ALLOWED);
//!
//! let attempt = AccessAttempt::new(41_000_123, 17, 20260503, field);
//!
//! assert_eq!(attempt.select(), AccessCondition::Grant);
//! assert_eq!(attempt.admitted_field().carries(AccessField::GRANTED), Presence::Present);
//! ```
//!
//! The selected condition is singular even when many field positions are present.

#![allow(dead_code)]

/// Presence of a named position in a field.
///
/// `Presence` is the public result of testing a field position.
///
/// The field remains the source of meaning. `Presence` only reports whether
/// a named position is carried by the field.
///
/// # Examples
///
/// ```
/// use semantic_bit::access::{AccessField, Presence};
///
/// let field = AccessField::empty().with(AccessField::BADGE_PRESENT);
///
/// assert_eq!(field.carries(AccessField::BADGE_PRESENT), Presence::Present);
/// assert_eq!(field.carries(AccessField::BADGE_RECOGNIZED), Presence::Absent);
/// ```
#[repr(u8)]
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Presence {
    /// The named position is not carried by the field.
    Absent = 0,

    /// The named position is carried by the field.
    Present = 1,
}

/// Selected continuation condition for an access attempt.
///
/// An access field may carry many active meanings. The selected continuation
/// condition is singular.
///
/// # Examples
///
/// ```
/// use semantic_bit::access::{AccessCondition, AccessField};
///
/// let field = AccessField::empty()
///     .with(AccessField::BADGE_PRESENT)
///     .with(AccessField::BADGE_RECOGNIZED)
///     .with(AccessField::HOLDER_ACTIVE)
///     .with(AccessField::DOOR_ALLOWED)
///     .with(AccessField::TIME_ALLOWED);
///
/// assert_eq!(field.select(), AccessCondition::Grant);
/// ```
#[repr(u8)]
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum AccessCondition {
    /// Entry may be granted.
    Grant = 0,

    /// Entry must be denied.
    Deny = 1,

    /// Entry must be reviewed before final action.
    Review = 2,

    /// The attempt should be recorded without granting access.
    RecordOnly = 3,

    /// The attempt requires alarm handling.
    Alarm = 4,
}

/// Bounded field for access admission.
///
/// The access field carries eight admitted meanings. Several meanings may be
/// present at once. A separate selection rule chooses one continuation
/// condition.
///
/// # Examples
///
/// ```
/// use semantic_bit::access::{AccessField, Presence};
///
/// let field = AccessField::empty()
///     .with(AccessField::BADGE_PRESENT)
///     .with(AccessField::BADGE_RECOGNIZED);
///
/// assert_eq!(field.carries(AccessField::BADGE_PRESENT), Presence::Present);
/// assert_eq!(field.carries(AccessField::TIME_ALLOWED), Presence::Absent);
/// ```
#[repr(transparent)]
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct AccessField(u8);

impl AccessField {
    /// Position meaning: a badge was presented to the reader.
    pub const BADGE_PRESENT: u8 = 1 << 0;

    /// Position meaning: the presented badge was recognized.
    pub const BADGE_RECOGNIZED: u8 = 1 << 1;

    /// Position meaning: the badge holder is active.
    pub const HOLDER_ACTIVE: u8 = 1 << 2;

    /// Position meaning: the door is allowed for this holder.
    pub const DOOR_ALLOWED: u8 = 1 << 3;

    /// Position meaning: the time is allowed for this access attempt.
    pub const TIME_ALLOWED: u8 = 1 << 4;

    /// Position meaning: the access attempt has been granted by rule.
    pub const GRANTED: u8 = 1 << 5;

    /// Position meaning: the access attempt has been recorded.
    pub const RECORDED: u8 = 1 << 6;

    /// Position meaning: the access attempt requires review.
    pub const REVIEW_REQUIRED: u8 = 1 << 7;

    /// Create an empty access field.
    ///
    /// No meaning is carried by the field until a named position is added.
    ///
    /// # Examples
    ///
    /// ```
    /// use semantic_bit::access::{AccessField, Presence};
    ///
    /// let field = AccessField::empty();
    ///
    /// assert_eq!(field.raw(), 0);
    /// assert_eq!(field.carries(AccessField::BADGE_PRESENT), Presence::Absent);
    /// ```
    pub const fn empty() -> Self {
        Self(0)
    }

    /// Create an access field from an admitted raw field value.
    ///
    /// This constructor is intended for values already admitted by the access
    /// field boundary. It does not parse human input and does not assign new
    /// meanings.
    ///
    /// # Examples
    ///
    /// ```
    /// use semantic_bit::access::{AccessField, Presence};
    ///
    /// let field = AccessField::from_admitted_raw(
    ///     AccessField::BADGE_PRESENT | AccessField::BADGE_RECOGNIZED,
    /// );
    ///
    /// assert_eq!(field.carries(AccessField::BADGE_PRESENT), Presence::Present);
    /// assert_eq!(field.carries(AccessField::BADGE_RECOGNIZED), Presence::Present);
    /// ```
    pub const fn from_admitted_raw(raw: u8) -> Self {
        Self(raw)
    }

    /// Return the raw field value.
    ///
    /// The raw value is useful for receipts and replay. The raw value should
    /// not be treated as explanation; it is the compact representation of the
    /// carried meanings.
    ///
    /// # Examples
    ///
    /// ```
    /// use semantic_bit::access::AccessField;
    ///
    /// let field = AccessField::empty().with(AccessField::BADGE_PRESENT);
    ///
    /// assert_eq!(field.raw(), AccessField::BADGE_PRESENT);
    /// ```
    pub const fn raw(self) -> u8 {
        self.0
    }

    /// Return a field that carries the given named position.
    ///
    /// This operation activates a meaning in the field.
    ///
    /// # Examples
    ///
    /// ```
    /// use semantic_bit::access::{AccessField, Presence};
    ///
    /// let field = AccessField::empty().with(AccessField::BADGE_PRESENT);
    ///
    /// assert_eq!(field.carries(AccessField::BADGE_PRESENT), Presence::Present);
    /// ```
    pub const fn with(mut self, position: u8) -> Self {
        self.0 |= position;
        self
    }

    /// Return a field that no longer carries the given named position.
    ///
    /// This operation removes a meaning from the field.
    ///
    /// # Examples
    ///
    /// ```
    /// use semantic_bit::access::{AccessField, Presence};
    ///
    /// let field = AccessField::empty()
    ///     .with(AccessField::BADGE_PRESENT)
    ///     .without(AccessField::BADGE_PRESENT);
    ///
    /// assert_eq!(field.carries(AccessField::BADGE_PRESENT), Presence::Absent);
    /// ```
    pub const fn without(mut self, position: u8) -> Self {
        self.0 &= !position;
        self
    }

    /// Report whether the field carries a named position.
    ///
    /// The result is `Presence`, not an authority decision. Selection is handled
    /// separately by `select`.
    ///
    /// # Examples
    ///
    /// ```
    /// use semantic_bit::access::{AccessField, Presence};
    ///
    /// let field = AccessField::empty().with(AccessField::TIME_ALLOWED);
    ///
    /// assert_eq!(field.carries(AccessField::TIME_ALLOWED), Presence::Present);
    /// assert_eq!(field.carries(AccessField::DOOR_ALLOWED), Presence::Absent);
    /// ```
    pub const fn carries(self, position: u8) -> Presence {
        if self.0 & position != 0 {
            Presence::Present
        } else {
            Presence::Absent
        }
    }

    /// Select the one continuation condition for this field.
    ///
    /// The field may carry several meanings. This function chooses one
    /// continuation condition according to the access rule.
    ///
    /// # Examples
    ///
    /// Grant when the required meanings are present:
    ///
    /// ```
    /// use semantic_bit::access::{AccessCondition, AccessField};
    ///
    /// let field = AccessField::empty()
    ///     .with(AccessField::BADGE_PRESENT)
    ///     .with(AccessField::BADGE_RECOGNIZED)
    ///     .with(AccessField::HOLDER_ACTIVE)
    ///     .with(AccessField::DOOR_ALLOWED)
    ///     .with(AccessField::TIME_ALLOWED);
    ///
    /// assert_eq!(field.select(), AccessCondition::Grant);
    /// ```
    ///
    /// Deny when the time is not allowed:
    ///
    /// ```
    /// use semantic_bit::access::{AccessCondition, AccessField};
    ///
    /// let field = AccessField::empty()
    ///     .with(AccessField::BADGE_PRESENT)
    ///     .with(AccessField::BADGE_RECOGNIZED)
    ///     .with(AccessField::HOLDER_ACTIVE)
    ///     .with(AccessField::DOOR_ALLOWED);
    ///
    /// assert_eq!(field.select(), AccessCondition::Deny);
    /// ```
    pub const fn select(self) -> AccessCondition {
        if self.0 & Self::BADGE_PRESENT == 0 {
            return AccessCondition::Deny;
        }

        if self.0 & Self::BADGE_RECOGNIZED == 0 {
            return AccessCondition::Review;
        }

        if self.0 & Self::HOLDER_ACTIVE == 0 {
            return AccessCondition::Deny;
        }

        if self.0 & Self::DOOR_ALLOWED == 0 {
            return AccessCondition::Deny;
        }

        if self.0 & Self::TIME_ALLOWED == 0 {
            return AccessCondition::Deny;
        }

        if self.0 & Self::REVIEW_REQUIRED != 0 {
            return AccessCondition::Review;
        }

        AccessCondition::Grant
    }

    /// Add `GRANTED` only when the selection rule admits `Grant`.
    ///
    /// `GRANTED` is not input. It is constructed after admission.
    ///
    /// # Examples
    ///
    /// ```
    /// use semantic_bit::access::{AccessField, Presence};
    ///
    /// let field = AccessField::empty()
    ///     .with(AccessField::BADGE_PRESENT)
    ///     .with(AccessField::BADGE_RECOGNIZED)
    ///     .with(AccessField::HOLDER_ACTIVE)
    ///     .with(AccessField::DOOR_ALLOWED)
    ///     .with(AccessField::TIME_ALLOWED)
    ///     .admit_grant();
    ///
    /// assert_eq!(field.carries(AccessField::GRANTED), Presence::Present);
    /// ```
    ///
    /// ```
    /// use semantic_bit::access::{AccessField, Presence};
    ///
    /// let field = AccessField::empty()
    ///     .with(AccessField::BADGE_PRESENT)
    ///     .admit_grant();
    ///
    /// assert_eq!(field.carries(AccessField::GRANTED), Presence::Absent);
    /// ```
    pub const fn admit_grant(self) -> Self {
        match self.select() {
            AccessCondition::Grant => self.with(Self::GRANTED),
            _ => self,
        }
    }

    /// Add `RECORDED` to the field.
    ///
    /// This marks that the attempt has been preserved.
    ///
    /// # Examples
    ///
    /// ```
    /// use semantic_bit::access::{AccessField, Presence};
    ///
    /// let field = AccessField::empty().mark_recorded();
    ///
    /// assert_eq!(field.carries(AccessField::RECORDED), Presence::Present);
    /// ```
    pub const fn mark_recorded(self) -> Self {
        self.with(Self::RECORDED)
    }
}

/// Fixed-width access attempt record.
///
/// The record carries admitted identifiers and an access field.
///
/// # Examples
///
/// ```
/// use semantic_bit::access::{AccessAttempt, AccessCondition, AccessField};
///
/// let field = AccessField::empty()
///     .with(AccessField::BADGE_PRESENT)
///     .with(AccessField::BADGE_RECOGNIZED)
///     .with(AccessField::HOLDER_ACTIVE)
///     .with(AccessField::DOOR_ALLOWED)
///     .with(AccessField::TIME_ALLOWED);
///
/// let attempt = AccessAttempt::new(41_000_123, 17, 20260503, field);
///
/// assert_eq!(attempt.select(), AccessCondition::Grant);
/// ```
#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct AccessAttempt {
    /// Admitted badge identifier.
    pub badge_id: u64,

    /// Admitted door identifier.
    pub door_id: u32,

    /// Epoch associated with this access attempt.
    pub epoch: u64,

    /// Access field carried by this attempt.
    pub field: AccessField,
}

impl AccessAttempt {
    /// Create an access attempt.
    ///
    /// The constructor receives admitted numeric identifiers and an access
    /// field. It does not parse names or human text.
    ///
    /// # Examples
    ///
    /// ```
    /// use semantic_bit::access::{AccessAttempt, AccessField};
    ///
    /// let attempt = AccessAttempt::new(
    ///     41_000_123,
    ///     17,
    ///     20260503,
    ///     AccessField::empty().with(AccessField::BADGE_PRESENT),
    /// );
    ///
    /// assert_eq!(attempt.badge_id, 41_000_123);
    /// assert_eq!(attempt.door_id, 17);
    /// ```
    pub const fn new(
        badge_id: u64,
        door_id: u32,
        epoch: u64,
        field: AccessField,
    ) -> Self {
        Self {
            badge_id,
            door_id,
            epoch,
            field,
        }
    }

    /// Select the one continuation condition for this attempt.
    ///
    /// This delegates to the access field selection rule.
    ///
    /// # Examples
    ///
    /// ```
    /// use semantic_bit::access::{AccessAttempt, AccessCondition, AccessField};
    ///
    /// let attempt = AccessAttempt::new(
    ///     41_000_123,
    ///     17,
    ///     20260503,
    ///     AccessField::empty().with(AccessField::BADGE_PRESENT),
    /// );
    ///
    /// assert_eq!(attempt.select(), AccessCondition::Review);
    /// ```
    pub const fn select(self) -> AccessCondition {
        self.field.select()
    }

    /// Construct the admitted field for this attempt.
    ///
    /// This adds `GRANTED` only if the selection rule permits it, and then
    /// adds `RECORDED`.
    ///
    /// # Examples
    ///
    /// ```
    /// use semantic_bit::access::{AccessAttempt, AccessField, Presence};
    ///
    /// let field = AccessField::empty()
    ///     .with(AccessField::BADGE_PRESENT)
    ///     .with(AccessField::BADGE_RECOGNIZED)
    ///     .with(AccessField::HOLDER_ACTIVE)
    ///     .with(AccessField::DOOR_ALLOWED)
    ///     .with(AccessField::TIME_ALLOWED);
    ///
    /// let attempt = AccessAttempt::new(41_000_123, 17, 20260503, field);
    /// let admitted = attempt.admitted_field();
    ///
    /// assert_eq!(admitted.carries(AccessField::GRANTED), Presence::Present);
    /// assert_eq!(admitted.carries(AccessField::RECORDED), Presence::Present);
    /// ```
    pub const fn admitted_field(self) -> AccessField {
        self.field.admit_grant().mark_recorded()
    }

    /// Create a receipt for this access attempt.
    ///
    /// The receipt preserves the admitted field and selected continuation
    /// condition.
    ///
    /// # Examples
    ///
    /// ```
    /// use semantic_bit::access::{AccessAttempt, AccessCondition, AccessField};
    ///
    /// let field = AccessField::empty()
    ///     .with(AccessField::BADGE_PRESENT)
    ///     .with(AccessField::BADGE_RECOGNIZED)
    ///     .with(AccessField::HOLDER_ACTIVE)
    ///     .with(AccessField::DOOR_ALLOWED)
    ///     .with(AccessField::TIME_ALLOWED);
    ///
    /// let attempt = AccessAttempt::new(41_000_123, 17, 20260503, field);
    /// let receipt = attempt.receipt(1);
    ///
    /// assert_eq!(receipt.badge_id, 41_000_123);
    /// assert_eq!(receipt.door_id, 17);
    /// assert_eq!(receipt.selected_condition, AccessCondition::Grant as u8);
    /// ```
    pub const fn receipt(self, sequence: u64) -> AccessReceipt {
        let admitted = self.admitted_field();

        AccessReceipt {
            badge_id: self.badge_id,
            door_id: self.door_id,
            epoch: self.epoch,
            sequence,
            access_raw: admitted.raw(),
            selected_condition: self.select() as u8,
        }
    }
}

/// Fixed-width receipt for an access attempt.
///
/// The receipt preserves the access field and the selected continuation
/// condition.
///
/// # Examples
///
/// ```
/// use semantic_bit::access::{AccessAttempt, AccessCondition, AccessField};
///
/// let field = AccessField::empty()
///     .with(AccessField::BADGE_PRESENT)
///     .with(AccessField::BADGE_RECOGNIZED)
///     .with(AccessField::HOLDER_ACTIVE)
///     .with(AccessField::DOOR_ALLOWED)
///     .with(AccessField::TIME_ALLOWED);
///
/// let receipt = AccessAttempt::new(41_000_123, 17, 20260503, field).receipt(1);
///
/// assert_eq!(receipt.selected_condition, AccessCondition::Grant as u8);
/// ```
#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct AccessReceipt {
    /// Badge identifier associated with the attempt.
    pub badge_id: u64,

    /// Door identifier associated with the attempt.
    pub door_id: u32,

    /// Epoch associated with the attempt.
    pub epoch: u64,

    /// Receipt sequence number.
    pub sequence: u64,

    /// Raw admitted access field.
    pub access_raw: u8,

    /// Selected access condition.
    pub selected_condition: u8,
}
