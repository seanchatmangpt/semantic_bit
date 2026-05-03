# Chapter 10 — Dispatch Fields

A record without a destination is a message lost in the void.

In Chapter 9, we created the **Control Record** to carry an instruction. But how does the system know *how* to send that record? This is the role of the **Dispatch Field**.

---

## 10.1 Routing by Distinction

Most software systems handle routing using complex logic: `if (remote) send_network() else process_local()`. In the semantic discipline, we use **dispatch fields** to multiplex the routing intent.

A dispatch field is a bounded carrier that tells the transport layer how to handle a record.

```rust
#[repr(transparent)]
pub struct DispatchField(u8);
```

By assigning named positions to routing meanings, we move the "how" of transport into the data itself.

---

## 10.2 Dispatch Meanings

A `DispatchField` might admit positions such as:
*   `LOCAL`: The target is on the same machine.
*   `REMOTE`: The target is across a network boundary.
*   `URGENT`: The record should skip the queue.
*   `JOURNAL`: A copy of the record must be preserved in the journal before dispatch.

```rust
use semantic_bit::dispatch::DispatchField;

let routing = DispatchField::empty()
    .with(DispatchField::REMOTE)
    .with(DispatchField::URGENT);
```

$$
\boxed{\textbf{The dispatch field encodes the transport intent as data.}}
$$

---

## 10.3 The Dispatch Snaphot

Why not just pass a boolean `is_remote`?

By using a field, we can capture a **dispatch snapshot**. When a record is sent, we can attach the `DispatchField` to it. The receiving system (or an auditor) can then see exactly how the record was handled.

If an urgent command was delayed, we can look at the dispatch field in the journal to see if the `URGENT` bit was actually set.

---

## 10.4 Unified Dispatch

The `DispatchField` allows a single "dispatch engine" to handle all types of records. The engine doesn't need to know the content of the `ControlRecord`; it only needs to observe the bits in the `DispatchField`.

1.  Observe `LOCAL`? → Hand to local actuator.
2.  Observe `REMOTE`? → Serialize and send to gateway.
3.  Observe `JOURNAL`? → Write to the journal.

This creates a clean separation between the **What** (Control Record) and the **How** (Dispatch Field).

---

## 10.5 Example: Routing a Command

```rust
use semantic_bit::dispatch::{DispatchField, Presence};

// We define our routing strategy
let routing = DispatchField::empty()
    .with(DispatchField::LOCAL)
    .with(DispatchField::JOURNAL);

assert_eq!(routing.carries(DispatchField::LOCAL), Presence::Present);
assert_eq!(routing.carries(DispatchField::REMOTE), Presence::Absent);
```

---

## 10.6 The Law in This Chapter

```text
Dispatch fields encode routing intent.
Routing is performed by observing bits, not prose logic.
The dispatch field separates the instruction from the transport.
Multiplexing allows for complex routing behaviors (e.g., LOCAL + JOURNAL).
The dispatch snapshot provides a record of transport intent.
```

The law chooses the verdict.
The record carries the command.
The field selects the path.

That is the discipline of dispatch fields.
