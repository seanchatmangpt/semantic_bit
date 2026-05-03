# Predicate

The **Predicate** is the identity that defines the "Type" of the relation.

$$
\boxed{\textbf{The Predicate is the meaning of the connection.}}
$$

## 16.5 Predicate as Law

The predicate is the most important part of the triple for the machine. It tells the selection rules how to interpret the connection between the subject and the object.

Common Predicates:
- `rdf:type` (Classification)
- `sb:owns` (Ownership)
- `sb:authorized_for` (Permission)
- `sb:depends_on` (Dependency)

## 16.6 The Vocabulary Boundary

Predicates are usually defined in a **Vocabulary**. A vocabulary is a special graph that maps predicate IDs to their behavioral laws. If a predicate is not in the admitted vocabulary, the triple is noise.
