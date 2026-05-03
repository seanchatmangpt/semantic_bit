# SemanticByte

The base primitive in our implementation is the Semantic Byte, usually represented as a `u8` wrapped in a transparent structure. It forms the foundational carrier for up to 8 distinct, multiplexed meanings.

In Rust, this is expressed as:

```rust
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SemanticByte(pub u8);
```

The `SemanticByte` ensures that the underlying bits are manipulated safely while preserving their fixed-width boundary. It enforces that there are exactly 256 possible activation states, and all mapping from these raw states to selected conditions must happen through deterministic functions (Selection Rules) attached to this type.
