# Chapter 12 — Journals

A system that only knows its current state is a system that has no history.

A **journal** is the permanent, sequential record of every consequence admitted by the system. While a **Receipt** (Chapter 8) preserves a single decision, a journal preserves the **order** and **integrity** of all decisions.

---

## 12.1 The Monotonic Chain

In the semantic discipline, we do not overwrite data. We append to the journal. This creates a monotonic chain of truth where every new record is bound to the sequence that came before it.

The journal is the ultimate authority for **replay**. If the system's memory is lost, we can rebuild it bit-by-bit by re-reading the journal from the beginning.

---

## 12.2 Journal Fields

Even the act of journaling is governed by a semantic field. The `JournalField` multiplexes the state of an entry:

```rust
#[repr(transparent)]
pub struct JournalField(u8);
```

Meanings might include:
*   `SEALED`: The entry has been cryptographically secured.
*   `ROTATED`: This entry marks the start of a new storage volume.
*   `SYNCED`: The bits have physically reached durable storage (the disk).
*   `CHECKPOINT`: This entry carries a summary of the system state (Chapter 13).

$$
\boxed{\textbf{The journal field is the evidence of archival integrity.}}
$$

---

## 12.3 The Journal Header

To maintain the sequence, every record in the journal is preceded by a **Journal Header**. This header provides the "envelope" for the operational data.

```rust
#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct JournalHeader {
    pub sequence: u64,
    pub epoch: u64,
    pub record_type: u32,
    pub payload_len: u32,
    pub field: JournalField,
}
```

By keeping the header fixed-width (`repr(C)`), the machine can scan the journal at high speed, jumping from one header to the next without needing to parse the payloads.

---

## 12.4 Journaling as a Boundary

The journal is the boundary between **active motion** and **permanent record**.

When a `ControlRecord` is dispatched (Chapter 10), the `JOURNAL` bit in the dispatch field ensures that a copy is written to the journal before the actuator receives the command. This ensures that we never act without first remembering.

$$
\boxed{\textbf{Remember before acting. Appending is the only truth.}}
$$

---

## 12.5 Example: Appending to the Journal

```rust
use semantic_bit::journal::{JournalField, JournalHeader};

// We prepare a header for a new receipt
let field = JournalField::empty().with(JournalField::SYNCED);
let header = JournalHeader::new(
    4100,      // sequence
    20260503,  // epoch
    1,         // record_type (Receipt)
    32,        // payload_len
    field,
);

assert_eq!(header.sequence, 4100);
assert_eq!(header.field.raw(), JournalField::SYNCED);
```

---

## 12.6 The Law in This Chapter

```text
A journal is a sequential record of consequence.
Journals are append-only.
The journal field governs archival integrity.
The journal header provides a fixed-width scanner.
Journaling must precede action (the "remember first" rule).
The journal enables absolute replay of system state.
```

The law chooses the verdict.
The receipt preserves the truth.
The journal chains the history.

That is the discipline of journals.
