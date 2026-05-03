# Receipts in Rust

Receipts are the immutable evidence of execution. When a consequence is constructed, a receipt must be generated.

In Rust, this is enforced by returning a tuple or structure that pairs the resulting state with its receipt:

```rust
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ConstructionReceipt {
    pub input_state: Cog8,
    pub selected_condition: ConditionCode8,
    pub constructed_outcome: Construct8,
    pub timestamp: u64,
}
```

The use of `#[repr(C)]` guarantees a predictable memory layout, crucial for serializing receipts into journals or transmitting them across the wire without semantic loss or parser drift.
