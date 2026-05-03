# Condition to Operation

The transition from a condition to an operation is the moment a system moves from *outcome selection* to *action declaration*.

In the Semantic8 Spine, a `ConditionCode8` field represents the selected outcome of an observation. An `Operation64` field represents the specific action (the noun-verb pair) the system is now admitted to perform.

$$
\boxed{\text{ConditionCode8} \xrightarrow{\text{select}} \text{Operation64}}
$$

## 11.3.1 The Admission Rule

Action is never a direct result of status. Action is only admitted after a condition code has been selected.

If the condition is `OK`, the system may admit a `DECLARED` operation. If the condition is `REFUSE` or `ABEND`, no operation may be admitted.

The law of action admission:

$$
\boxed{\textbf{An operation is only admitted into the field if the selected condition code admits the noun-verb pairing.}}
$$

## 11.3.2 Mapping Outcomes to Actions

The mapping between `ConditionCode8` and `Operation64` is governed by a selection rule that prevents illegal motion.

| Condition Code | Admitted Operation State |
| :--- | :--- |
| `OK` | `DECLARED` + `AUTHORIZED` |
| `WARN` | `DECLARED` + `REVIEW_REQUIRED` |
| `BLOCKED` | `FORBIDDEN` |
| `RETRY` | `SKIPPED` + `REPLAYABLE` |
| `REFUSE` | `FORBIDDEN` |

## 11.3.3 The Noun-Verb Constraint

When a condition admits an operation, it must also bound the *scope* of that operation. The `Operation64` field ensures that the system cannot simply "perform an action." It must perform a *verb* (e.g., `CREATE`, `READ`, `UPDATE`) on a specific *noun* (e.g., `RECORD`, `BADGE`, `GATE`).

The condition `OK` for an access badge does not admit "any operation." It admits the operation `(GATE, OPEN)`.

By binding the operation to the condition, we ensure that the system's motion is always a lawful consequence of its selected outcome.
