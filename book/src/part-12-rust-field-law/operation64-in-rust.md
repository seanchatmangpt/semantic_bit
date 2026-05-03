# Operation64 in Rust

`Operation64` extends the 8-bit primitive to a 64-bit carrier, dividing it to express Nouns and Verbs. This creates a matrix of 64 discrete operation cells.

```rust
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Operation64(pub u64);
```

In Rust, the Noun and Verb definitions are often embedded into the 64-bit space, where specific bit ranges indicate the Noun (the target of the action) and the Verb (the action itself). This guarantees that every authorized action falls strictly within the pre-defined 64-cell grid, preventing "creative" or unauthorized execution paths.
