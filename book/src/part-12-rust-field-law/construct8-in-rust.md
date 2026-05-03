# CONSTRUCT8 in Rust

`CONSTRUCT8` defines the bounded creation of new facts, artifacts, or consequences. It represents the outcome of a successful evaluation of the `COG8` field.

```rust
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Construct8(pub u8);
```

In Rust, returning a `Construct8` from a function implies that a lawful change has occurred. The bits within `Construct8` indicate what type of construction was admitted (e.g., `DECLARED`, `BOUNDED`). This outcome is then immediately passed into a Receipt structure for immutable recording.
