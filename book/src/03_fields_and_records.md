# Chapter 3 — Fields and Records

Meaning must be bounded before it is carried. Once bounded, it must be preserved.

In the first two chapters, we looked at the **Semantic Bit** and the **Field**. But a field does not exist in a vacuum. It participates in a larger structure that carries context, identity, and time. This structure is the **Record**.

---

## 3.1 The Field is the Carrier

A field is a bounded carrier of related operational meanings.

In Rust, we represent this as a `transparent` struct around a primitive type, such as `u8`. This ensures that the field has the same memory layout as the primitive, but carries the type safety and methods of the semantic discipline.

```rust
#[repr(transparent)]
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct AccessField(u8);
```

The field is responsible for:
*   Activating meanings (`with`)
*   Removing meanings (`without`)
*   Observing presence (`carries`)
*   Selecting conditions (`select`)

$$
\boxed{\textbf{The field is the compact carrier of activation and selection.}}
$$

---

## 3.2 The Record is the Context

A record is a fixed-width structure that carries a field along with its necessary context.

While the `AccessField` tells us what conditions are present, it doesn't tell us *who* presented the badge, *where* they presented it, or *when* the attempt occurred. For that, we need an `AccessAttempt` record.

```rust
#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct AccessAttempt {
    pub badge_id: u64,
    pub door_id: u32,
    pub epoch: u64,
    pub field: AccessField,
}
```

By using `#[repr(C)]`, we guarantee the layout of the record. This makes the record machine-readable and suitable for storage or transmission without relying on high-level serialization frameworks.

$$
\boxed{\textbf{The record binds the field to its admitted context.}}
$$

---

## 3.3 The Receipt is the Preservation

The most important record in the semantic discipline is the **Receipt**.

A receipt is not a log entry. It is a preserved record of the admitted field and the selected condition at the moment of consequence.

```rust
#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct AccessReceipt {
    pub badge_id: u64,
    pub door_id: u32,
    pub epoch: u64,
    pub sequence: u64,
    pub access_raw: u8,
    pub selected_condition: u8,
}
```

Notice that the receipt carries the `access_raw` value and the `selected_condition`. It does not carry the rule that made the selection. It preserves the *result* of the rule. This allows for absolute auditability: we can see exactly what the system observed and what it decided, even if the rules change later.

---

## 3.4 The Record Chain

The semantic discipline moves through a chain of records:

1.  **Attempt**: The initial record carrying input conditions and context.
2.  **Field**: The bounded carrier where selection rules are applied.
3.  **Receipt**: The final record preserving the consequence.

$$
\boxed{AccessAttempt \rightarrow AccessField \rightarrow AccessReceipt}
$$

This chain ensures that every motion in the system is preceded by a bounded field and followed by a preserved receipt.

---

## 3.5 Example: Working with Records

The following example shows how a record carries a field through the selection process to a receipt.

```rust
use semantic_bit::access::{AccessAttempt, AccessCondition, AccessField};

let field = AccessField::empty()
    .with(AccessField::BADGE_PRESENT)
    .with(AccessField::BADGE_RECOGNIZED)
    .with(AccessField::HOLDER_ACTIVE)
    .with(AccessField::DOOR_ALLOWED)
    .with(AccessField::TIME_ALLOWED);

// The record binds the field to context
let attempt = AccessAttempt::new(41_000_123, 17, 20260503, field);

// The receipt preserves the consequence
let receipt = attempt.receipt(1);

assert_eq!(attempt.select(), AccessCondition::Grant);
assert_eq!(receipt.badge_id, 41_000_123);
assert_eq!(receipt.selected_condition, AccessCondition::Grant as u8);
```

---

## 3.6 The Law in This Chapter

```text
A field is a bounded carrier of meaning.
A record is a fixed-width carrier of context.
A record contains one or more fields.
Repr(transparent) preserves the field layout.
Repr(C) preserves the record layout.
A receipt is a record of consequence.
The chain preserves the path from attempt to receipt.
```

The field carries the "what."
The record carries the "who," "where," and "when."
The receipt carries the "forever."

That is the discipline of fields and records.
