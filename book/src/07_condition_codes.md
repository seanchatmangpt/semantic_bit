# Chapter 7 — Condition Codes

If the field is the evidence, the **condition code** is the verdict.

A condition code is a singular, mutually exclusive value that represents the selected continuation of a system. It is the final instruction that the machine must follow.

---

## 7.1 Mutual Exclusion

In Chapter 4, we learned that multiplexed fields allow many meanings to be active at once. In contrast, condition codes are **mutually exclusive**. You cannot grant access and deny it at the same time.

$$
\boxed{\textbf{Activation is multiplexed. Condition is singular.}}
$$

In Rust, the most natural representation of a condition code is an `enum`.

```rust
#[repr(u8)]
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum AccessCondition {
    Grant = 0,
    Deny = 1,
    Review = 2,
    RecordOnly = 3,
    Alarm = 4,
}
```

The `#[repr(u8)]` ensures that each condition has a stable numeric value, making it suitable for fixed-width records and cross-language communication.

---

## 7.2 From Selection to Condition

The selection rule (discussed in Chapter 5) is the bridge that takes a multiplexed field and returns a singular condition code.

| Field (Binary) | Condition Code | Operational Meaning |
| :--- | :--- | :--- |
| `... 0001 1111` | `Grant` (0) | Open the door. |
| `... 0000 1111` | `Deny` (1) | Keep the door closed. |
| `... 0000 0011` | `Review` (2) | Flash the yellow light. |

Each condition code maps to a specific **motion** in the physical or logical world.

---

## 7.3 The Condition as Instruction

A condition code is more than just a label; it is an **instruction**.

When a lower-level component (like a lock controller) receives an `AccessReceipt`, it looks at the `selected_condition`. It doesn't need to know *why* the access was granted; it only needs to know that the condition is `Grant`.

This separation of concerns allows the "thinking" parts of the system to be decoupled from the "acting" parts.

---

## 7.4 Auditability and Condition Codes

Because condition codes have stable numeric values, they are perfect for preservation in receipts.

```rust
// A receipt preserves the verdict
let receipt = AccessReceipt {
    // ...
    selected_condition: AccessCondition::Grant as u8,
};
```

If we see a `1` in the `selected_condition` field of a receipt from ten years ago, we know with absolute certainty that the verdict was `Deny`.

---

## 7.5 The Law in This Chapter

```text
Condition codes are mutually exclusive.
A condition code represents a singular continuation.
Enums with repr(u8) provide stable executable notation.
The condition code is the instruction for the next component.
The verdict is preserved in the receipt for auditability.
```

The field carries the evidence.
The rule chooses the condition.
The machine performs the motion.

That is the discipline of condition codes.
