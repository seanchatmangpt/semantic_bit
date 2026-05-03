# Pattern Layer

Complexity is managed by layering, not by expansion.

In the Semantic Bit, we do not create monolithic structures. We layer simple pattern cells to form sophisticated operational records. Each layer remains bounded and verifiable.

$$
\boxed{\textbf{Layering is the composition of admitted meanings into a single carrier.}}
$$

## 24.1 The Vertical Stack

A typical operational record is a stack of pattern layers. Consider an access attempt record:

1.  **Identity Layer**: Who is presenting the badge? (Subject IRI)
2.  **Field Layer**: What are the active conditions? (Status8)
3.  **Result Layer**: What was the selected outcome? (ConditionCode8)
4.  **Temporal Layer**: When did this occur? (Timestamp)
5.  **Evidence Layer**: Why was this admitted? (Graph Digest)

Each layer is a distinct pattern cell. The combination of these cells forms the complete record.

## 24.2 Independence of Layers

A layer must not depend on the internal implementation of the layer beneath it. It depends only on the semantic meaning.

The `Status8` layer does not care if the `Identity` layer is a `u64` or a full IRI string in the graph. It only cares that a valid status field is present in the record at the assigned position.

## 24.3 Layer Adjacency

In memory, layers are adjacent. This is the physical reality of the field.

```rust
#[repr(C)]
struct AccessAttempt {
    subject_id: u64,       // Identity Layer
    status: Status8,       // Field Layer
    outcome: Condition8,   // Result Layer
    timestamp: u64,        // Temporal Layer
}
```

The `repr(C)` attribute ensures that the layers are ordered exactly as defined. This allows the system to read the field without parsing.

$$
\boxed{\textbf{Selection reads the field. It does not parse the record.}}
$$

## 24.4 Cumulative Meaning

The meaning of the record is the sum of its layers. If one layer is missing or corrupted, the entire record loses its admission status. The manufacturing process ensures that all required layers are present and correctly aligned before a record is emitted.
