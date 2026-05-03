# Chapter 8 — Receipts

A system that does not remember its decisions is a system that cannot be trusted.

A **receipt** is the preserved evidence of an operational distinction. It is the final artifact of the semantic chain, capturing the moment of consequence in a form that can be audited, replayed, and verified.

---

## 8.1 The Anatomy of a Receipt

In the semantic discipline, a receipt is a fixed-width record. It does not contain stories or explanations. It contains raw values that represent admitted truth.

Our `AccessReceipt` carries everything needed to reconstruct the decision:

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

*   **Identifiers** (`badge_id`, `door_id`): Who and where.
*   **Time and Order** (`epoch`, `sequence`): When and in what sequence.
*   **Evidence** (`access_raw`): The multiplexed field at the moment of decision.
*   **Outcome** (`selected_condition`): The singular verdict.

---

## 8.2 Preservation at the Moment of Consequence

A receipt must be created at the exact moment the decision is made. It should not be "logged" later from memory. It is the immediate consequence of the distinction.

$$
\boxed{\textbf{The receipt preserves the field at the moment of consequence.}}
$$

In our code, the `AccessAttempt::receipt` method ensures this:

```rust
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
```

The receipt doesn't just record the `AccessCondition`; it records the `admitted_field` (which includes the status bits like `GRANTED` and `RECORDED`).

---

## 8.3 Receipts are Not Logs

Logs are prose for humans. Receipts are records for machines.

| Feature | Log Entry | Semantic Receipt |
| :--- | :--- | :--- |
| **Format** | Text/JSON strings | Fixed-width Binary |
| **Audience** | Developers debugging | Systems auditing/replaying |
| **Reliability** | "Best effort" | Guaranteed snapshot |
| **Meaning** | Interpretive | Admitted |

A log might say: `"Access granted to user 123 at door 17"`.
A receipt simply says: `badge_id: 123, door_id: 17, access_raw: 111, condition: 0`.

The receipt is harder for a human to read without a map, but it is impossible for the machine to misinterpret.

---

## 8.4 Replay and Audit

The ultimate test of a receipt is **replay**.

If we have a receipt from last year, we can feed the `access_raw` back into our selection rule. If the rule hasn't changed, it should return the same `selected_condition`. If it returns something else, we know the rules of our system have evolved.

This allows us to answer questions like: "If we had the current rules in place six months ago, would this person have been allowed in?"

---

## 8.5 The Law in This Chapter

```text
A receipt is a preserved operational record.
A receipt captures context, evidence, and outcome.
Receipts use fixed-width representation (repr(C)).
Receipts are created at the moment of consequence.
Receipts enable deterministic audit and replay.
```

The badge brings the conditions.
The law selects the verdict.
The receipt preserves the truth.

That is the discipline of receipts.
