# ConditionCode8 in Rust

Unlike `Status8`, `ConditionCode8` represents the singular, non-multiplexed result of applying a Selection Rule. It is the deterministic reduction of many active facts into one actionable state.

```rust
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConditionCode8 {
    Ok = 0,
    Warn = 1,
    Blocked = 2,
    // ...
}
```

By defining `ConditionCode8` as a Rust `enum` rather than a bitfield, the compiler enforces the Selection Law: it is impossible for a condition code to be in two states simultaneously. The transition from the multiplexed `Status8` to the singular `ConditionCode8` is where the system exercises its authority to decide.
