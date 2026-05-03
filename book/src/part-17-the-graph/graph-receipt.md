# Graph Receipt

A **Graph Receipt** is the record of a graph operation.

$$
\boxed{\textbf{The graph receipt preserves the memory transition.}}
$$

## 17.15 Recording Admission

When a batch of triples is admitted to a graph, a receipt is emitted. This receipt contains:
- The IDs of the admitted triples.
- The ID of the target graph.
- The ID of the admitting authority.
- The digest (hash) of the new graph state.

## 17.16 Replaying the Graph

Just as we replay field receipts to reconstruct state, we replay graph receipts to reconstruct the graph. This ensures that the "Memory" of the system is just as recoverable and verifiable as the "Motion" of the system.
