# Graph Admission

**Admission** is the gate through which a triple must pass to enter the graph.

$$
\boxed{\textbf{Nothing is admitted to the graph without a rule.}}
$$

## 17.11 The Admission Protocol

1. **Verify Identity**: Are all IDs in the triple known?
2. **Verify Shape**: Does the triple follow the vocabulary's domain and range?
3. **Verify Authority**: Does the source have the right to assert this relation?
4. **Emit Receipt**: Record the admission event.

## 17.12 Refusal

If any step fails, the triple is **Refused**. It is not "Ignored"; it is actively excluded. The system should emit a `REFUSE` condition to signal that an attempt was made to pollute the graph with unverified data.
