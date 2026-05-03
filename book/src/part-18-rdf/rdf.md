# RDF

The semantic field bounds operational meaning within a single record.

An access field carries eight bits. A status field carries eight bits. These fields are sufficient for local decision, but they are insufficient for global relation.

When one record must relate to another record across a system boundary, the bits alone are not enough. The system requires a common surface for admitting relations.

This surface is RDF.

RDF is not treated here as a "web technology" or a "linked data" exercise. It is treated as the formal admission of structural relations into the field.

$$
\boxed{\textbf{RDF is the relation surface where admitted identities meet through named predicates.}}
$$

---

## 18.1 Beyond the Bit

In Book 1, we learned that a bit carries a named position in a bounded field.

```rust
pub const BADGE_PRESENT: u8 = 1 << 0;
```

This is a local identity. It is valid within the `AccessField`. It is not valid in a journal of maintenance or a log of electricity.

RDF allows the system to lift local meaning into a universal relation without losing the discipline of the field.

A triple relates a subject to an object through a predicate:

```turtle
<badge:41000123> <rel:heldBy> <person:72>.
```

The triple is the "bit" of the graph. It is the smallest unit of admitted relation.

---

## 18.2 The Triple as Admitted Relation

A triple does not "explain" a relation. It *is* the relation.

Just as the access field does not guess at "why" a badge is present, the triple does not guess at "why" a badge is held by a person. It simply carries the admitted fact.

1. **Subject**: The identity being described.
2. **Predicate**: The named relation being asserted.
3. **Object**: The value or identity that completes the relation.

$$
\boxed{\textbf{A triple is a bounded record of relation between a subject and an object.}}
$$

---

## 18.3 The Admission Boundary

RDF is often criticized for being "too open." In the Semantic Bit philosophy, nothing is open.

An RDF triple is only admitted to the graph after it passes a boundary.

- The **Subject** must be an admitted identity (IRI or Blank Node).
- The **Predicate** must be an admitted property (IRI).
- The **Object** must be an admitted value (Literal) or identity (IRI or Blank Node).

The graph is not a collection of strings. It is a collection of admitted relations.

---

## 18.4 The Law in This Chapter

RDF provides the universal carrier for the Semantic Bit's relations.

```text
The bit carries state.
The triple carries relation.
The graph carries memory.
```

The triple is the formal record of motion between identities. Before the triple is admitted, the motion is not recognized by the system.

$$
\boxed{\textbf{Meaning is bounded in the bit. Relation is bounded in the triple.}}
$$
