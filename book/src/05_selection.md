# Chapter 5 — Selection

A field can carry many meanings, but the machine can only take one next step.

Selection is the process of reducing a multiplexed field of evidence down to a singular **continuation condition**. This is where the "law" of the system is applied to the "facts" of the field.

---

## 5.1 The Singularity of Action

In Chapter 4, we saw how a field can carry many active meanings at once. However, a door cannot be both "granted" and "denied" at the same moment. It must open or remain closed.

$$
\boxed{\textbf{Selection is the deterministic reduction of many meanings to one motion.}}
$$

The selection rule is the function that makes this choice. In Rust, we represent this as a method on the field that returns a singular enum.

---

## 5.2 The Selection Rule

The `select` method in our `access` module demonstrates a typical selection rule. It checks the presence of specific semantic bits and returns the highest-priority continuation condition.

```rust
pub const fn select(self) -> AccessCondition {
    // If no badge is present, we cannot even begin
    if self.0 & Self::BADGE_PRESENT == 0 {
        return AccessCondition::Deny;
    }

    // if the badge isn't recognized, we need a human to look
    if self.0 & Self::BADGE_RECOGNIZED == 0 {
        return AccessCondition::Review;
    }

    // Other missing conditions lead to a denial
    if self.0 & Self::HOLDER_ACTIVE == 0 || 
       self.0 & Self::DOOR_ALLOWED == 0  || 
       self.0 & Self::TIME_ALLOWED == 0 {
        return AccessCondition::Deny;
    }

    // If all is well, we grant access
    AccessCondition::Grant
}
```

This rule is **deterministic**. Given the same field of bits, it will always return the same continuation condition.

---

## 5.3 Evidence vs. Law

It is important to distinguish between the **evidence** and the **law**.

*   **Evidence**: The multiplexed field (e.g., `0b0001_1111`). It tells us what happened.
*   **Law**: The selection rule (the `select` function). It tells us what the evidence *means* in terms of consequence.

By separating these two, we gain **auditability**. We can store the raw field in a receipt and, if the law changes tomorrow, we can still determine what the decision *would have been* under the new law, while knowing what it *was* under the old law.

---

## 5.4 Precedence and Priority

Selection rules often imply a priority. In our `access` rule:
1.  **Badge Presence** is the first gate.
2.  **Recognition** is the second gate (leading to `Review`).
3.  **Permissions** (Holder, Door, Time) are the final gates.

The order of checks in the `select` function defines the priority of the system's response.

---

## 5.5 Example: The Moment of Choice

```rust
use semantic_bit::access::{AccessCondition, AccessField};

// All conditions for a grant are present
let field = AccessField::empty()
    .with(AccessField::BADGE_PRESENT)
    .with(AccessField::BADGE_RECOGNIZED)
    .with(AccessField::HOLDER_ACTIVE)
    .with(AccessField::DOOR_ALLOWED)
    .with(AccessField::TIME_ALLOWED);

assert_eq!(field.select(), AccessCondition::Grant);

// Removing one permission changes the singular result
let denied_field = field.without(AccessField::TIME_ALLOWED);
assert_eq!(denied_field.select(), AccessCondition::Deny);
```

---

## 5.6 The Law in This Chapter

```text
Selection is a deterministic mapping.
Selection reduces plural evidence to a singular condition.
The selection rule is the executable law of the field.
Priority is defined by the order of evaluation.
Receipts preserve the evidence so selection can be audited.
```

The bits are the facts.
The rule is the law.
The condition is the verdict.

That is the discipline of selection.
