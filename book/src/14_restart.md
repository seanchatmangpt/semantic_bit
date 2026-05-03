# Chapter 14 — Restart

A system that cannot restart is a system that cannot survive.

**Restart** is the operational process of restoring the system's state from the **Journal** (Chapter 12) and **Checkpoints** (Chapter 13). In the semantic discipline, a restart is not a "reboot" into a clean state; it is a continuation of the history.

---

## 14.1 The Restart Field

The recovery process is itself governed by a semantic field. The `RestartField` multiplexes the state of the transition:

```rust
#[repr(transparent)]
pub struct RestartField(u8);
```

Meanings might include:
*   `COLD`: No state is available; the system must start from sequence zero.
*   `WARM`: A verified checkpoint is available.
*   `REPLAYING`: The system is currently applying records from the journal.
*   `READY`: The system has reached the head of the journal and is ready for new motion.

---

## 14.2 The Recovery Protocol

A safe restart follows a strict protocol:
1.  **Observe**: Check for the most recent `VERIFIED` checkpoint.
2.  **Restore**: Load the state from the checkpoint (Warm Start).
3.  **Replay**: Read every `JournalHeader` following the checkpoint's sequence.
4.  **Activate**: Once the head of the journal is reached, set the `READY` bit.

$$
\boxed{\textbf{Restart is the reconstruction of truth from evidence.}}
$$

---

## 14.3 Determinism and Replay

Because every decision in the system was made by a deterministic **Selection Rule** (Chapter 5) acting on a multiplexed **Field** (Chapter 4), the replay is guaranteed to be correct.

We don't need to "guess" what the system was doing. We have the bits. We have the rules. The machine will always arrive at the same destination.

---

## 14.4 Restart as a Boundary

The `READY` bit in the `RestartField` is the ultimate boundary for external actors.

A **Channel** (Chapter 11) should not set its `READY` bit until the system itself is `READY`. This ensures that we never accept new input before we have fully understood our own history.

$$
\boxed{\textbf{Understand the past before accepting the future.}}
$$

---

## 14.5 Example: Managing the Restart

```rust
use semantic_bit::restart::{RestartField, Presence};

// The system is currently replaying the journal
let state = RestartField::empty()
    .with(RestartField::WARM)
    .with(RestartField::REPLAYING);

assert_eq!(state.carries(RestartField::READY), Presence::Absent);

// Once replay is finished
let ready_state = state.with(RestartField::READY);
assert_eq!(ready_state.carries(RestartField::READY), Presence::Present);
```

---

## 14.6 The Law in This Chapter

```text
Restart is a continuation of history.
The restart field multiplexes the recovery state.
Cold starts begin at zero; warm starts begin at checkpoints.
Replay reconstruction is guaranteed by determinism.
The system must be ready before accepting new input.
```

The journal chains the history.
The checkpoint summarizes the state.
The restart resumes the motion.

That is the discipline of restart.
