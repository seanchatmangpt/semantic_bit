# COG8 in Rust

The `COG8` (Cognitive) field is the final boundary check before consequence. It verifies that all preconditions (Closure, Evidence, Authority, Freshness, etc.) have been satisfied.

```rust
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cog8(pub u8);
```

Rust implements `COG8` much like `Status8`, but its selection rules are strictly tied to whether the operation may proceed to `CONSTRUCT8`. If the required bits (e.g., `AUTHORIZED`, `EVIDENCED`) are not present, the Rust function explicitly returns a failure condition, preventing the construction of an outcome.
