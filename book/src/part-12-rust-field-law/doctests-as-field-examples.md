# Doctests as Field Examples

In the Semantic Bit philosophy, examples cannot be merely illustrative; they must be provably true. Rust's doctest feature provides the perfect mechanism for this.

Every rule of the field described in this book is backed by executable Rust doctests in the `semantic_bit` library. When the text claims that combining `OK` and `WARN` in `Status8` selects to a specific condition, the accompanying code block is evaluated by `cargo test` to prove it.

```rust
/// Demonstrates that an OK and WARN Status8 maps to ConditionCode8::Warn.
/// ```
/// use semantic_bit::{Status8, ConditionCode8};
/// let status = Status8(Status8::OK | Status8::WARN);
/// assert_eq!(status.select_condition(), ConditionCode8::Warn);
/// ```
```

This ensures that the documentation and the implementation are permanently locked in deterministic alignment.
