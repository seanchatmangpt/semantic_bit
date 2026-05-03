# Chapter 15 — Representation Contracts

Software is a contract between the representation of a distinction and the consequence of its presence.

We have reached the end of the first manual. We have followed the **Semantic Bit** from a single named position at a door to a complete system of journals, checkpoints, and restarts. The final lesson is about the **Contract** that holds it all together.

---

## 15.1 The Promise of the Bit

When we say that bit 0 means `BADGE_PRESENT`, we are making a promise. We are promising that every component in the system—from the reader to the journal to the auditor—will treat that bit as that specific meaning forever.

This is the **Representation Contract**.

$$
\boxed{\textbf{Meaning is a permanent contract between representation and law.}}
$$

---

## 15.2 Contract Fields

A `ContractField` multiplexes the system's own understanding of its rules:

```rust
#[repr(transparent)]
pub struct ContractField(u8);
```

Meanings include:
*   `ADMITTED`: The record follows the current, active version of the law.
*   `DEPRECATED`: The record follows an older version of the law (Audit required).
*   `VERIFIED`: The representation has been checked for structural integrity.

---

## 15.3 The Immutability of Distinction

In modern software, we are used to "refactoring" and "migration." In the semantic bit discipline, a distinction is immutable.

If a bit changes its meaning, it is no longer the same bit. If a record changes its layout, it is no longer the same record. We do not "fix" the past; we admit a new future.

By honoring the representation contract, we ensure that our journals (Chapter 12) remain readable for decades, not just days.

---

## 15.4 Beyond the Bit

This book has focused on the most fundamental unit of software: the bit. But the principles we have learned—**bounded fields**, **singular selection**, **deterministic replay**, and **fixed-width preservation**—apply at every scale.

As you move on to more complex systems, remember the discipline of the access badge:

1.  **Distinguish** before acting.
2.  **Bound** the meaning before trusting the motion.
3.  **Remember** the evidence before emitting the consequence.

---

## 15.5 The Final Law

```text
A representation contract is a permanent promise of meaning.
Distinctions are immutable.
Contract fields multiplex schema and versioning state.
Integrity is verified by honoring the contract.
The representation is the truth; the prose is just a comment.
```

The bits are the facts.
The rule is the law.
The contract is the peace.

That is the discipline of the Semantic Bit.
