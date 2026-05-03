# The Graph

A **Graph** is a set of admitted triples. While a single triple is a fact, the graph is the context in which all facts are interpreted.

$$
\boxed{Graph = \{Triple_1, Triple_2, ..., Triple_n\}}
$$

## 17.1 The Context of Truth

In a semantic system, truth is relative to the graph. If you ask "Is User 123 an Admin?", the answer depends on whether the triple $(User123, hasRole, Admin)$ exists in the active graph. 

The graph provides the **Universe of Discourse**. If a relation is not in the graph, it does not exist for the machine.

## 17.2 Structured Memory

The graph is not a "database" of strings. It is a structured memory of relations. Because every triple follows the same fixed-width format, the graph can be optimized for high-speed navigation and inference.
