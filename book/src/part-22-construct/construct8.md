# CONSTRUCT8

Construction is a multi-stage process. To ensure that every construction is lawful and verifiable, we track its progress using a dedicated semantic field: **CONSTRUCT8**.

CONSTRUCT8 is an 8-bit status field that captures the state of a construction event from declaration to replay.

## 22.1 The Eight CONSTRUCT8 Meanings

| Position | Meaning | Role |
| :--- | :--- | :--- |
| 1 | `DECLARED` | The construction has been requested. |
| 2 | `TYPED` | The construction template has been validated. |
| 3 | `AUTHORIZED` | The authority to construct has been verified. |
| 4 | `EVIDENCED` | The source inquiry has returned a match. |
| 5 | `BOUNDED` | The resulting Delta has been calculated and bounded. |
| 6 | `ADMITTED` | The Delta has been merged into the system memory. |
| 7 | `RECEIPTED` | A receipt for the construction has been emitted. |
| 8 | `REPLAYABLE` | The construction has been archived for future replay. |

$$
\boxed{\textbf{CONSTRUCT8 captures the lifecycle of a semantic transition.}}
$$

---

## 22.2 The Path to Admission

A construction event must pass through these meanings in order. 

We do not admit a Delta (`ADMITTED`) until it has been calculated (`BOUNDED`) and its source inquiry has provided proof of its necessity (`EVIDENCED`).

If a construction fails at any stage, the CONSTRUCT8 field preserves the point of failure. If `BOUNDED` is active but `ADMITTED` is not, we know the Delta was calculated but the final merge was blocked.

---

## 22.3 Bitwise Progress

In Rust, the CONSTRUCT8 field is represented as a bitfield:

```rust
#[repr(transparent)]
pub struct Construct8(u8);

impl Construct8 {
    pub const DECLARED: u8   = 1 << 0;
    pub const TYPED: u8      = 1 << 1;
    pub const AUTHORIZED: u8 = 1 << 2;
    pub const EVIDENCED: u8  = 1 << 3;
    pub const BOUNDED: u8    = 1 << 4;
    pub const ADMITTED: u8   = 1 << 5;
    pub const RECEIPTED: u8  = 1 << 6;
    pub const REPLAYABLE: u8 = 1 << 7;
}
```

---

## 22.4 The Law of Construction Status

A construction is only complete when all eight bits are active. 

$$
\boxed{\textbf{Meaning is total. A partial construction is an invalid state.}}
$$

Every construction event in a Semantic Bit system must emit a record containing its final CONSTRUCT8 state. This allows auditors to verify that the system is following its own laws of transition.
