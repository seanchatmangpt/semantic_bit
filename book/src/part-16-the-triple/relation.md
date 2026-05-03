# Relation

A **Relation** is the emergent meaning of a triple. 

$$
\boxed{\textbf{A relation is the admitted bridge between identities.}}
$$

## 16.11 Direct vs. Inverse Relations

A relation has a direction: from Subject to Object via Predicate.
$(A, owns, B)$ is not the same as $(B, owns, A)$.

However, many relations have a natural inverse:
$(A, owns, B) \rightarrow (B, isOwnedBy, A)$

In a semantic graph, we usually store the direct relation and use the machine to infer the inverse when needed.

## 16.12 Relation as Constraint

Relations are not just descriptions; they are constraints. If the graph contains the triple $(User, hasRole, ReadOnly)$, the selection rules will constrain the motion of that user to read operations only. The relation *is* the law.
