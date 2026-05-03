# Construction Is Not Mutation

The traditional approach to software is mutation: `x = x + 1`. This approach is the primary source of entropy in computing systems. It destroys previous state and makes replay impossible.

In the Semantic Bit philosophy, we reject mutation.

## 22.1 The Immutability of Fact

A triple is a fact. A fact that was true at timestamp $T_1$ does not become "false" at $T_2$ because someone "deleted" it. The memory of the system should reflect that the fact was admitted at $T_1$.

When we need to "change" something, we construct a **successor relation**.

$$
\boxed{\textbf{We do not overwrite. We supersede.}}
$$

---

## 22.2 The Delta as an Admitted Event

Instead of changing a value in place, we emit a Delta graph.

*   **Mutation**: Update `Badge_101` set `status = :Suspended`.
*   **Construction**: Create a triple `{ :Badge_101 :hasStatus :Suspended }` in a new graph.

The Delta is an event. It carries the meaning of the change. It can be signed, hashed, and receipted before it is ever merged with the main graph.

---

## 22.3 Traceability of Origin

Because a Delta is constructed via a SPARQL query, we know exactly *why* every new triple exists. We have the source query and the source graph used for the inquiry.

If we mutate a database, we often lose the "why". If we construct a Delta, the "why" is preserved in the logic of the construction itself.

$$
\boxed{\textbf{A mutation is a loss of history. A construction is an addition to history.}}
$$
