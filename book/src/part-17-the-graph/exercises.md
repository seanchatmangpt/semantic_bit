# Graph Exercises

Test your understanding of graph boundaries and admission.

## Exercise 17.1: Boundary Definition

Define the boundary for a "Security Graph". What predicates should be admitted? What subjects should be excluded?

## Exercise 17.2: Inference Chain

Given the triples:
- $(A, parentOf, B)$
- $(B, parentOf, C)$
- $(parentOf, isTransitive, true)$

Compute the closure. What is the relation between $A$ and $C$?

## Exercise 17.3: Admission Logic

Write a pseudocode function `admit_triple(graph, triple)` that implements the 4-step Admission Protocol.

## Exercise 17.4: Graph Versioning

A system has Graph Version 1. A new user is added.
1. Describe the triples in Graph Version 2.
2. What information must be in the version receipt?
