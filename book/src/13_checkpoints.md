# Chapter 13 — Checkpoints

History is long. Replay can be slow. A **checkpoint** is the system's way of saying: "This is where we are right now."

A checkpoint is a record that summarizes the state of the system at a specific sequence number in the **Journal** (Chapter 12). It allows us to resume operations without re-reading the entire history from bit one.

---

## 13.1 Summarizing the Field

In the access badge system, the "state" might be the total number of grants and denies, or the current list of active badges. A checkpoint captures these totals and binds them to a journal sequence number.

$$
\boxed{\textbf{A checkpoint is a compression of history into a singular record.}}
$$

---

## 13.2 Checkpoint Fields

Like all operational records, a checkpoint is governed by a semantic field. The `CheckpointField` multiplexes the validity of the summary:

```rust
#[repr(transparent)]
pub struct CheckpointField(u8);
```

Meanings might include:
*   `FULL`: The record contains the entire system state.
*   `INCREMENTAL`: The record only contains changes since the last checkpoint.
*   `VERIFIED`: The totals have been checked against the journal evidence.

---

## 13.3 The Recovery Boundary

A checkpoint is the boundary for **Recovery** (Chapter 14). When the system restarts, it looks for the most recent `VERIFIED` checkpoint. It loads that state and then only replays the journal entries that came *after* that sequence number.

By keeping checkpoints fixed-width (`repr(C)`), we ensure that the recovery process is as fast as the machine's memory can load the bits.

---

## 13.4 Checkpoints are Entries

A checkpoint is not a separate file. It is just another entry in the journal.

When the journal scanner (Chapter 12) sees a `JournalHeader` with the `CHECKPOINT` bit set in its field, it knows that the payload is a `CheckpointRecord`.

$$
\boxed{\textbf{The journal carries the history. The checkpoint provides the shortcut.}}
$$

---

## 13.5 Example: Recording a Checkpoint

```rust
use semantic_bit::checkpoint::{CheckpointField, CheckpointRecord};

// We capture the current totals at sequence 4100
let field = CheckpointField::empty()
    .with(CheckpointField::FULL)
    .with(CheckpointField::VERIFIED);

let record = CheckpointRecord::new(
    4100, // last_sequence
    3200, // total_grants
    900,  // total_denies
    field,
);

assert_eq!(record.last_sequence, 4100);
assert_eq!(record.total_grants + record.total_denies, 4100);
```

---

## 13.6 The Law in This Chapter

```text
A checkpoint summarizes history at a specific sequence.
Checkpoints enable fast recovery by skipping journal replay.
The checkpoint field multiplexes validity and scope.
Checkpoints are themselves entries in the journal.
A verified checkpoint is the foundation for a safe restart.
```

The journal chains the history.
The checkpoint summarizes the state.
The restart resumes the motion.

That is the discipline of checkpoints.
