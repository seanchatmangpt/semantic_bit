# RDF Datasets

A single graph carries a collection of triples. A dataset carries a collection of graphs.

In the Semantic Bit philosophy, we use datasets to bound different sources of truth. One graph might carry admitted identity, another might carry operational status, and a third might carry an immutable receipt.

$$
\boxed{\textbf{An RDF Dataset is a collection of named graphs that partitions the relation surface.}}
$$

---

## 18.1 The Quad

To support multiple graphs, the triple is extended to a **quad**. The fourth element is the graph identity (an IRI).

`Subject -> Predicate -> Object -> Graph`

The graph identity tells the system where the relation was admitted.

```turtle
# In the Identity Graph
<badge:1> a <class:Badge> <graph:admitted-identities> .

# In the Status Graph
<badge:1> <rel:status> <status:OK> <graph:current-status> .
```

---

## 18.2 The Default Graph

Every dataset has one "default graph" that has no name. It is the surface where all unnamed triples live. Operational queries often begin by merging several named graphs into the default graph to see the complete picture.

---

## 18.3 Partitioning Truth

By using datasets, we can manage the lifecycle of information independently.

1. **Identity Graph**: Persistent and slowly changing.
2. **Status Graph**: Volatile and frequently updated.
3. **Audit Graph**: Immutable and ever-growing.

The system does not mix these concerns. A query for "Who is the holder of badge 1?" looks in the Identity Graph. A query for "Is badge 1 currently blocked?" looks in the Status Graph.

---

## 18.4 The Law in This Chapter

The graph bounds the triple. The dataset bounds the graph.

```text
The triple is the fact.
The graph is the context.
The dataset is the world.
Without context, the fact is a noise.
```

By using datasets, we maintain the discipline of the field at a global scale. We always know not just *what* is admitted, but *where* and *why* it was admitted.

$$
\boxed{\textbf{A dataset provides the context for admission across the relation surface.}}
$$
