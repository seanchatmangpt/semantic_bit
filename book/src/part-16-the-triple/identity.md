# Identity

Identity is the foundation of the triple. Without a stable way to name subjects, predicates, and objects, the graph collapses into prose.

$$
\boxed{\textbf{Identity is a singular, immutable pointer to a semantic definition.}}
$$

## 16.9 Beyond Names

A name like "Alice" is not an identity. There may be many Alices. 
An identity is a unique identifier (ID) assigned by an authority. In our system, this is usually a numeric ID (`u64`) mapped to an IRI (Internationalized Resource Identifier).

## 16.10 The Stability of the ID

Once an identity is assigned to a subject, it must never change. If the subject's properties change (e.g., Alice changes her role), we add new triples. We do not change the ID. The ID is the "Hook" upon which all relations are hung.
