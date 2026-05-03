# Sixty-Four Relation Cells

When we orthogonally combine the Eight Sources with the Eight Targets, we create a matrix of exactly **64 Relation Cells**.

Every connection in the system must map to one of these 64 cells. We represent a relation as a 6-bit value: the upper 3 bits denote the Source category, and the lower 3 bits denote the Target category.

For example:
- `EVENT` points to `SUBJECT` (Cell: 3, 1) — e.g., an audit log linking an operation to the user who performed it.
- `PROOF` points to `RULE` (Cell: 7, 4) — e.g., a receipt demonstrating compliance with a specific selection policy.
- `CLAIM` points to `OBJECT` (Cell: 6, 2) — e.g., a digital signature claiming authenticity of a document.

The power of the 64 Relation Cells is that they define the **Admitted Topography** of the graph. Not all 64 cells are valid in every system. An architect might declare that a `RULE` cannot point to an `EVENT` because rules are static and events are ephemeral.

By classifying connections into this 8x8 grid, we prevent spaghetti architecture. If a developer attempts to create a relation that falls into an invalid cell, the compiler or the runtime will refuse it. The graph remains legible, bounded, and traversable.
