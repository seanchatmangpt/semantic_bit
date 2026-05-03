# The Triple as Relation

The triple is not just a data structure. It is an executable law.

A triple $(S, P, O)$ states that subject $S$ has relation $P$ with object $O$. Because the triple is structured, the machine can "walk" the relations to discover new facts.

$$
\boxed{\textbf{A triple is a directed edge in the graph of admitted truth.}}
$$

## 15.12 Navigating the Triple

Given a triple, the machine can answer three questions without narration:
1. **Who is the subject?** (Find all $P$ for $S$)
2. **What is the relation?** (Find all $S$ and $O$ connected by $P$)
3. **Who is the object?** (Find all $S$ that have $P$ with $O$)

## 15.13 Relation Composition

Triples can be composed to form chains:
$(User, hasRole, Admin) + (Admin, canAccess, Door) \rightarrow (User, canAccess, Door)$

In a semantic system, this composition is not a "query" in the traditional sense; it is the **inference of motion**. We do not write code to "find if the user is an admin"; we admit the triples that make the user an admin.
