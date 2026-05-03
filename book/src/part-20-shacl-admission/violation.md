# Violation

In the Semantic Bit philosophy, a violation is not a "warning" or a "message for the logs." A violation is a **failure of admission**.

When a graph is presented to a Shape Boundary, every triple must be admitted. If a triple or a set of triples fails to satisfy the constraints of the shape, the system must produce a **Violation Record**.

---

## 20.1 The Violation as a State

A violation is a named state in an admission field.

Just as we used bit-positions in Book 1 to represent missing conditions (like `BADGE_NOT_RECOGNIZED`), we use SHACL violations to represent structural failures in the graph. 

A violation indicates that the "motion" of the graph (into the next stage of processing) has been **Blocked**.

$$
\boxed{\textbf{A violation is a structural refusal. It is the evidence that a boundary has been defended.}}
$$

---

## 20.2 The Anatomy of a Refusal

A SHACL violation provides the evidence required to reject a graph. In the Semantic Bit architecture, a violation record must contain:

1.  **The Focus Node**: The specific identity that failed the test.
2.  **The Result Path**: The predicate or relation that carried the invalid value.
3.  **The Constraint Component**: The specific law that was broken (e.g., `sh:MinCountConstraintComponent`).
4.  **The Source Shape**: The authority that defined the boundary.

| Component | Meaning |
| :--- | :--- |
| Focus Node | The identity under review. |
| Result Path | The specific field of failure. |
| Source Shape | The law being enforced. |
| Message | The explanation (admitted only for human review). |

---

## 20.3 Violation vs. Error

In traditional programming, an "error" often results in a crash or a thrown exception. In the Semantic Bit, a **Violation** is a first-class citizen of the operational field.

A system may carry many violations and still remain operational. The selection rule simply maps the presence of any `Violation` to a `BLOCKED` or `REFUSE` condition.

$$
\boxed{\textbf{Violation is the admitted state of failure. Refusal is the selected consequence.}}
$$

By treating violations as data (triples) rather than control-flow events (exceptions), we preserve the ability to receipt the failure and replay the rejection.

---

## 20.4 The SHACL Receipt of Rejection

When a graph fails admission, the system does not just "stop." It emits a **SHACL Receipt**.

This receipt carries the raw violation triples as evidence. This allows a downstream auditor to verify exactly why a specific badge attempt or door-motion was refused, even years after the event.

The receipt ensures that even rejection is a **bounded operation**.

---

## 20.5 The Law in This Chapter

```text
A violation is a refusal of admission.
A violation is a state, not a crash.
A violation preserves the identity and path of failure.
Violations are triples admitted as evidence.
The SHACL Receipt preserves the evidence of rejection.
```

The system does not guess why a graph is invalid. It identifies the specific boundary that was breached.
