# Status8 in Rust

`Status8` represents the truth of the field prior to consequence. We define it as a bounded `u8` where each bit position corresponds to one of the eight Status meanings: OK, WARN, BLOCKED, UNKNOWN, SKIPPED, STALE, RECEIPTED, REPLAYABLE.

```rust
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Status8(pub u8);

impl Status8 {
    pub const OK: u8         = 1 << 0;
    pub const WARN: u8       = 1 << 1;
    // ...
}
```

The Rust implementation enforces Multiplexing Law by allowing bitwise OR operations to combine these states. Selection is implemented via methods that inspect the bits and return a singular `ConditionCode8`, explicitly mapping multiple active states to one definitive outcome.
