# Fact

A **Fact** is an admitted triple that exists in a graph.

$$
\boxed{\textbf{A fact is a triple that the system has accepted as true.}}
$$

## 16.13 Fact vs. Claim

An input may provide a "Claim": "I am the Admin".
The system does not accept claims. It checks its graph for the "Fact": $(User, hasRole, Admin)$. 

If the triple exists in the admitted graph, it is a fact. If it does not, the claim is rejected.

## 16.14 The Lifecycle of a Fact

1. **Discovery**: A potential relation is identified.
2. **Validation**: The relation is checked against shapes and laws.
3. **Admission**: The triple is written to the graph and signed.
4. **Preservation**: The fact is stored in the immutable history.
