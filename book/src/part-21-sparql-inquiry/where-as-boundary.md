# WHERE as Boundary

If `SELECT` is the projection, `WHERE` is the **boundary**.

The `WHERE` clause defines the structural constraints that must be met for a relation to be admitted into the result. It is the definition of the "shape" we are looking for in the graph.

## 21.1 The Pattern Match

A pattern in the `WHERE` clause is a template for a triple.

```sparql
WHERE {
  ?subject :relatedTo ?object .
}
```

This pattern creates a boundary: only triples that match the predicate `:relatedTo` are considered. Every other triple in the graph is outside this boundary.

$$
\boxed{\textbf{The WHERE clause is a filter on the universe of possible relations.}}
$$

---

## 21.2 Composition of Boundaries

Boundaries are composite. When multiple patterns are joined, the boundary narrows.

```sparql
WHERE {
  ?badge a :AccessBadge .
  ?badge :assignedTo ?person .
  ?person :hasRole :SecurityGuard .
}
```

In this case, the boundary is the intersection of three conditions. A `?badge` is only admitted if it is an `AccessBadge` **AND** it is assigned to a `?person` **AND** that person has the role `SecurityGuard`.

The more patterns we add, the more specific our semantic boundary becomes.

---

## 21.3 Graph Patterns as Structural Proof

In a Semantic Bit system, a match in the `WHERE` clause is a **structural proof** that a set of relations exists.

If the pattern cannot be satisfied, the inquiry returns an empty set. This is a semantic signal: the requested relation is not present in the admitted memory. 

We do not guess why a pattern failed. We simply accept that the boundary was not crossed.

---

## 21.4 Optionality and Entropy

The `OPTIONAL` keyword allows a pattern to fail without rejecting the entire match. While useful, `OPTIONAL` introduces entropy. It means the resulting field may or may not have certain positions filled.

Use `OPTIONAL` sparingly. In critical decision paths, prefer explicit boundaries that either succeed fully or fail fully.

$$
\boxed{\textbf{A partial match is a partial proof. A total match is a total proof.}}
$$
