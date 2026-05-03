# Bits State, Triples Relation

The architecture of a semantic system is a hierarchy of meanings.

1. **The Bit**: Carries a single admitted meaning (e.g., `ACTIVE`).
2. **The Field (Byte)**: Multiplexes several meanings into a state (e.g., `Status8`).
3. **The Triple**: Connects two states or identities through a relation.
4. **The Graph**: A collection of triples forming a bounded memory.

$$
\boxed{\textbf{Bits bound the motion of the clock. Triples bound the motion of the data.}}
$$

## 15.8 The State vs. The Relation

State is transient. It changes as the system moves.
Relation is persistent. It defines the structure in which the state moves.

A `Door` may have a state of `LOCKED` (a bit).
A `User` may have a relation of `AUTHORIZED_FOR` the `Door` (a triple).

The `LOCKED` bit is checked every millisecond. The `AUTHORIZED_FOR` triple is admitted once and preserved in the graph.

## 15.9 Engineering Symmetry

There is a symmetry between the 8-bit field and the triple. Just as a field admits 8 positions, a triple positions its participants in 3 slots. Both are fixed-width. Both are predictable. Both are "Semantic".
