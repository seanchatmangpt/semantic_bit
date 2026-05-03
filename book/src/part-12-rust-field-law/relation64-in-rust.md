# Relation64 in Rust

`Relation64` bounds the connection between operations, data, or identities, representing dependency, authority, and ordering.

```rust
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Relation64(pub u64);
```

Similar to `Operation64`, it uses bitwise masks to define the source and target of the relationship. Rust's strict typing ensures that relation checking—a prerequisite for advancing along the Semantic8 Spine—is performed safely and that invalid connection matrices cannot compile or execute.
