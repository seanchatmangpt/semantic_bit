# SELECT as Projection

In a Semantic Bit system, the graph is a multidimensional space of relations. The act of inquiry must reduce this complexity into a form suitable for decision-making.

The `SELECT` clause is the **projection** of the graph onto a result set.

## 21.1 Reduction of Dimensionality

A graph may contain millions of triples. However, a single operational decision (such as "Should this door open?") requires only a few bits of information. 

`SELECT` performs the reduction:

```sparql
SELECT ?badge ?status
WHERE {
  ?badge a :AccessBadge ;
         :hasStatus ?status .
}
```

In this example, the inquiry ignores all other relations (the badge's color, the date it was issued, the owner's name) and projects only the `?badge` identity and its `?status`.

$$
\boxed{\textbf{Projection is the intentional discarding of irrelevant relations.}}
$$

---

## 21.2 The Result as a Temporary Field

When `SELECT` executes, it produces a sequence of bindings. Each binding can be viewed as a temporary semantic field where variables act as named positions.

| ?badge | ?status |
| :--- | :--- |
| `:Badge_101` | `:Active` |
| `:Badge_102` | `:Suspended` |

These bindings are the "admitted conditions" that the next stage of the system will process. By using `SELECT`, we transform graph-bound memory into field-bound state.

---

## 21.3 Determinism in Projection

A projection must be deterministic. If the same graph and the same inquiry are present, the projection must yield the same result. 

We avoid `SELECT *` in production Semantic Bit systems. To select "everything" is to admit unknown entropy into the decision loop. Always name your projections.

$$
\boxed{\textbf{A named projection is a bounded projection.}}
$$
