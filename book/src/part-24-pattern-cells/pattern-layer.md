# Pattern Layer

Complexity is not solved by making a single, massive pattern. It is solved by **layering**. A pattern layer allows us to reason about specific aspects of a system in isolation while maintaining the integrity of the whole.

$$
\boxed{\textbf{A pattern layer is a subset of a graph closure that isolates a specific semantic domain while preserving its relations to other layers.}}
$$

---

## 24.1 Layering by Concern

Consider an access control system. We can identify at least three layers:

1. **The Core Layer**: The raw bits and triples defining the access rule (Badge, Door, Grant).
2. **The Observation Layer**: The history of attempts and receipts (Events, Epochs, Sequences).
3. **The Governance Layer**: The authority and evidence that admitted the rules (Signatures, SHACL Shapes).

---

## 24.2 Vertical vs. Horizontal Layering

- **Horizontal Layering**: Different domains (e.g., Access vs. Billing) that interact only through admitted interfaces.
- **Vertical Layering**: Increasing levels of abstraction (e.g., Bit $\rightarrow$ Triple $\rightarrow$ Pattern Cell $\rightarrow$ System).

---

## 24.3 Integrity Across Layers

A pattern layer is not a "silo". It is a view into the same unified graph. A change in the Core Layer must be visible in the Observation Layer if the relations between them are admitted.

$$
\boxed{\textbf{Layers are logical views; the graph is the single source of truth.}}
$$

---

## 24.4 The Pattern Layer Law

The law of pattern layers ensures that we can scale our understanding without losing the bit-perfect foundation:

```text
Complexity is managed by layering.
A layer is a bounded view of the graph.
Relations cross layer boundaries only through admission.
The graph closure maintains integrity across all layers.
```

The system does not separate data. It separates concerns while keeping the data unified.
