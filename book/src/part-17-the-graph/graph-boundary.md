# Graph Boundary

A graph is not infinite. It must have a boundary.

The **Graph Boundary** defines which triples are admitted into the active memory of the system. Without a boundary, the graph would grow until it exhausted the machine's resources, or it would admit contradictory facts from unverified sources.

$$
\boxed{\textbf{A graph is defined by its admission boundary.}}
$$

## 17.3 The Edge of Knowledge

What lies outside the boundary is **Noise**. The system cannot reason about it. 
What lies inside the boundary is **Admitted Truth**. The system must act upon it.

## 17.4 Boundary Enforcement

We enforce the boundary using **Shapes** (like SHACL) and **Selection Rules**. Before a triple is added to the graph, it must prove its right to exist. Does it follow the vocabulary? Does the subject exist? Is the predicate allowed for this object?
