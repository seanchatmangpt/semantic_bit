# Chapter 2 — Named Positions

Meaning is not found in the raw value of a bit. It is found in the **position** it occupies.

In the access badge example, we saw that a field of eight bits could carry eight distinct meanings. But how do we assign these meanings? Why do we say that bit 1 is `BADGE_PRESENT` and bit 6 is `GRANTED`?

---

## 2.1 The Address of Meaning

A bit in a field is identified by its position. We can think of the position as the **address** of a specific operational meaning within that bounded field.

In a `u8` field, there are eight positions:

| Position (Index) | Bitmask (Binary) | Bitmask (Decimal) | Bitmask (Hex) |
| :--------------: | :--------------: | :---------------: | :-----------: |
|        0         |   `0000 0001`    |         1         |    `0x01`     |
|        1         |   `0000 0010`    |         2         |    `0x02`     |
|        2         |   `0000 0100`    |         4         |    `0x04`     |
|        3         |   `0000 1000`    |         8         |    `0x08`     |
|        4         |   `0001 0000`    |        16         |    `0x10`     |
|        5         |   `0010 0000`    |        32         |    `0x20`     |
|        6         |   `0100 0000`    |        64         |    `0x40`     |
|        7         |   `1000 0000`    |        128        |    `0x80`     |

The **Semantic Bit** is the union of a position and its admitted meaning.

$$
\boxed{\textbf{Position + Admitted Meaning = Semantic Bit}}
$$

Without a name, a bit is just a value. Without a position, a name is just prose.

---

## 2.2 The Discipline of Naming

When we assign a name to a position, we are creating a **contract**.

Once bit 1 is assigned to `BADGE_PRESENT`, it must never be used for anything else in that field. If the system evolves and `BADGE_PRESENT` is no longer needed, that position should be retired, not repurposed.

$$
\boxed{\textbf{A position is a permanent address for one admitted meaning.}}
$$

This permanence allows for **replay** and **audit**. If we look at a receipt from three years ago, bit 1 must still mean `BADGE_PRESENT`. The raw bits never lie, but only if the naming contract is preserved.

---

## 2.3 Presence is Observation

We do not ask if a bit is "true" or "false". We ask if a meaning is **present** in the field.

In the `access` module, we use the `Presence` enum to report this observation:

```rust
#[repr(u8)]
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Presence {
    /// The named position is not carried by the field.
    Absent = 0,

    /// The named position is carried by the field.
    Present = 1,
}
```

This prevents the consumer of the field from treating the internal bitmask as a boolean. It forces the consumer to recognize that they are observing a field-bound distinction.

---

## 2.4 Defining Positions in Rust

While we can use binary literals (`0b0000_0001`), it is often clearer to define positions using bitshifts. This makes the index of the position explicit.

```rust
impl AccessField {
    /// Position 0: a badge was presented.
    pub const BADGE_PRESENT: u8    = 1 << 0;

    /// Position 1: the badge was recognized.
    pub const BADGE_RECOGNIZED: u8 = 1 << 1;

    /// Position 2: the holder is active.
    pub const HOLDER_ACTIVE: u8    = 1 << 2;

    /// Position 3: the door is allowed.
    pub const DOOR_ALLOWED: u8     = 1 << 3;

    /// Position 4: the time is allowed.
    pub const TIME_ALLOWED: u8     = 1 << 4;

    /// Position 5: the grant was admitted.
    pub const GRANTED: u8          = 1 << 5;

    /// Position 6: the attempt was recorded.
    pub const RECORDED: u8         = 1 << 6;

    /// Position 7: review is required.
    pub const REVIEW_REQUIRED: u8  = 1 << 7;
}
```

By naming the positions, we move from the world of numbers to the world of meaning.

---

## 2.5 Example: The Position Law

The following test demonstrates that even if the internal representation is opaque, the named position remains the authority for meaning.

```rust
use semantic_bit::access::{AccessField, Presence};

// We define the meanings we care about
let badge_present = AccessField::BADGE_PRESENT;
let granted = AccessField::GRANTED;

// We observe a field from a raw value (perhaps from a receipt)
let field = AccessField::from_admitted_raw(0b0010_0001);

assert_eq!(field.carries(badge_present), Presence::Present);
assert_eq!(field.carries(granted), Presence::Present);
assert_eq!(field.carries(AccessField::HOLDER_ACTIVE), Presence::Absent);
```

The field does not "know" about the names. The code knows the names and their positions. The field only carries the bits.

This is the second lesson:

$$
\boxed{\textbf{The field carries the bits. The name carries the law.}}
$$

---

## 2.6 The Law in This Chapter

```text
A position is an index in a bounded field.
A name assigns a permanent meaning to a position.
A semantic bit is the union of position and name.
Presence is the result of observing a position.
Bitshifts make positions explicit.
The name is the authority, the field is the carrier.
```

The machine sees the bit.
The engineer sees the name.
The law sees the consequence.

That is the discipline of named positions.
