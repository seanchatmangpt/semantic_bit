# Chapter 6 — Status Fields

Meanings in a field are not all created equal. Some are **observations** from the outside world. Others are **status** reports from the inside.

A status field is a bounded carrier that reports the current state or the final outcome of a system operation.

---

## 6.1 Input vs. Status

In the `AccessField`, we have eight positions. We can divide them into two groups:

### Input Bits (Evidence)
These are activated by external observation (the badge reader, the database, the clock):
*   `BADGE_PRESENT`
*   `BADGE_RECOGNIZED`
*   `HOLDER_ACTIVE`
*   `DOOR_ALLOWED`
*   `TIME_ALLOWED`

### Status Bits (Outcome)
These are activated by the system's own rules:
*   `GRANTED`
*   `RECORDED`
*   `REVIEW_REQUIRED`

$$
\boxed{\textbf{The system receives evidence. The system admits status.}}
$$

---

## 6.2 Constructed Meaning

A status bit is a **constructed meaning**. It does not exist until the system's selection rule admits it.

In our Rust implementation, we see this in the `admit_grant` method:

```rust
pub const fn admit_grant(self) -> Self {
    match self.select() {
        AccessCondition::Grant => self.with(Self::GRANTED),
        _ => self,
    }
}
```

The `GRANTED` bit is not passed as an argument to the function. It is "admitted" based on the result of `self.select()`. This ensures that the status bit accurately reflects the system's deterministic choice.

---

## 6.3 The Record of Outcome

Why bother putting status bits in the same field as the inputs?

By multiplexing the outcome (`GRANTED`) with the evidence (`BADGE_PRESENT`, etc.), we create a **complete operational snapshot**.

When we record a receipt, we don't just record what we decided; we record the bits that led to the decision and the bit that represents the decision.

```rust
use semantic_bit::access::{AccessField, Presence};

let field = AccessField::empty()
    .with(AccessField::BADGE_PRESENT)
    .with(AccessField::BADGE_RECOGNIZED)
    .with(AccessField::HOLDER_ACTIVE)
    .with(AccessField::DOOR_ALLOWED)
    .with(AccessField::TIME_ALLOWED)
    .admit_grant()
    .mark_recorded();

assert_eq!(field.carries(AccessField::GRANTED), Presence::Present);
assert_eq!(field.carries(AccessField::RECORDED), Presence::Present);
```

The raw value of this field now tells the whole story of the access attempt.

---

## 6.4 The Discipline of Feedback

Status bits can also act as inputs to the *next* stage of selection.

For example, a system might check if `RECORDED` is present before allowing a physical lock to move. In this way, status becomes a dependency for further motion.

$$
\boxed{\textbf{Status is the evidence for the next distinction.}}
$$

---

## 6.5 The Law in This Chapter

```text
Status fields report internal state or outcome.
Input bits carry evidence; status bits carry admission.
Status is constructed by rule, never accepted as input.
Multiplexing input and status creates a complete snapshot.
A status bit in one field can be the input for the next decision.
```

The reader brings the input.
The law admits the status.
The field carries the truth.

That is the discipline of status fields.
