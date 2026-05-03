# Triple Position

The position of an identity within a triple determines its semantic role.

$$
\boxed{\textbf{Meaning is a function of position.}}
$$

## 16.15 The Three Slots

1. **Subject (Slot 0)**: The source of the relation.
2. **Predicate (Slot 1)**: The nature of the relation.
3. **Object (Slot 2)**: The target or value of the relation.

## 16.16 Swapping Positions

An identity can appear in different slots in different triples:
- $(User123, hasRole, Admin)$ -> `User123` is Subject.
- $(AdminGroup, hasMember, User123)$ -> `User123` is Object.

This flexibility is what allows the graph to represent complex webs of meaning. The meaning of `User123` does not change, but its *role* changes based on its position in the triple.
