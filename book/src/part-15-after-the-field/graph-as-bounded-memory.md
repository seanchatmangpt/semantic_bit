# The Graph as Bounded Memory

If a triple is a single fact, a **Graph** is the memory of the system.

But we do not admit "The Infinite Graph". We admit **Named Graphs** with clear boundaries.

$$
\boxed{\textbf{A graph is a container for triples that share a common admission boundary.}}
$$

## 15.14 The Memory Boundary

A graph bounds what the system "knows" at any given moment. 
- **The Identity Graph**: Triples about users and roles.
- **The Config Graph**: Triples about machine parameters.
- **The Receipt Graph**: Triples representing the history of operations.

By partitioning triples into graphs, we prevent "Fact Drift". A fact that is admitted into the Identity Graph cannot accidentally influence the Config Graph unless a bridge relation is explicitly admitted.

## 15.15 The Closed World

Inside a graph, we operate under the **Closed World Assumption**: if a triple is not in the graph, the relation does not exist. This is the ultimate "Meaning Before Motion" gate. If the graph does not say you have permission, the bits in the field will never activate for you.
