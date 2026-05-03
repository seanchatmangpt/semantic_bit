# Chapter 4 — Multiplexing

A single bit carries one distinction. A field carries many.

When we allow multiple semantic bits to occupy the same bounded field, we are **multiplexing** meaning. This is not just a storage optimization; it is a fundamental property of operational fields.

---

## 4.1 Activation May Be Many

In a semantic field, distinctions are independent. The fact that a badge is present does not prevent the door from being allowed. The fact that the time is allowed does not prevent the holder from being active.

Because each meaning has its own **named position**, they can all be present at the same time.

$$
\boxed{\textbf{Multiplexing is the coexistence of independent meanings in a single field.}}
$$

In our `AccessField`, we can activate multiple meanings using the `with` method, which uses the bitwise OR operator (`|`) internally.

```rust
use semantic_bit::access::AccessField;

let field = AccessField::empty()
    .with(AccessField::BADGE_PRESENT)
    .with(AccessField::BADGE_RECOGNIZED)
    .with(AccessField::HOLDER_ACTIVE);
```

The resulting field is a single `u8` value that carries all three meanings simultaneously.

---

## 4.2 The Sum of Meanings

We can think of the field as the **sum** of its active meanings. If we look at the raw bits, we see the multiplexed state:

| Meaning | Position | Bitmask |
| :--- | :---: | :---: |
| `BADGE_PRESENT` | 0 | `0000 0001` |
| `BADGE_RECOGNIZED` | 1 | `0000 0010` |
| `HOLDER_ACTIVE` | 2 | `0000 0100` |
| **Multiplexed Field** | | **`0000 0111`** |

The raw value `7` (or `0x07`) is the machine's representation of these three specific admitted conditions being present.

---

## 4.3 Independence of Observation

Multiplexing only works if we can observe one meaning without being distracted by others. This is the role of the `carries` method, which uses the bitwise AND operator (`&`).

```rust
use semantic_bit::access::{AccessField, Presence};

let field = AccessField::empty()
    .with(AccessField::BADGE_PRESENT)
    .with(AccessField::TIME_ALLOWED);

// We observe one meaning at a time
assert_eq!(field.carries(AccessField::BADGE_PRESENT), Presence::Present);
assert_eq!(field.carries(AccessField::TIME_ALLOWED), Presence::Present);

// Other meanings remain absent
assert_eq!(field.carries(AccessField::HOLDER_ACTIVE), Presence::Absent);
```

The field does not "blend" the meanings into something new. It carries them distinctly.

---

## 4.4 The Law of Selection

While activation may be many, the system must eventually act. Action requires a singular choice.

This leads us back to the primary law of the Semantic Bit:

$$
\boxed{\textbf{Activation may be many. Selection must be one.}}
$$

Multiplexing allows the system to gather all necessary evidence (active meanings) before making a singular decision (selected condition). The field is the **evidence carrier**. The selection rule is the **judge**.

---

## 4.5 Why Not Booleans?

You might ask: "Why not just use a list of booleans or a struct with fields?"

```rust
struct AccessAttempt {
    badge_present: bool,
    badge_recognized: bool,
    holder_active: bool,
    // ...
}
```

A collection of booleans is "unbounded prose." It has no fixed width, no inherent audit trail, and no clear relationship to the machine's registers.

A multiplexed field is **bounded**. It has a known size, a known layout, and a raw value that can be recorded as a single number. It is the machine's own language for carrying state.

---

## 4.6 The Law in This Chapter

```text
Multiplexing is the coexistence of meanings.
Named positions allow meanings to remain independent.
Bitwise OR (|) activates multiple meanings.
Bitwise AND (&) observes individual meanings.
The field is the sum of its evidence.
Activation is plural; action is singular.
```

The badge brings the conditions.
The field carries the conditions.
The law selects the motion.

That is the discipline of multiplexing.
