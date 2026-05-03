# Bounded Delta

A change in a Semantic Bit system is represented as a **Delta**. 

A Delta is not a command ("Update this"). A Delta is a **Difference Graph**. It represents the set of triples to be added to the system's memory to reflect a new state.

## 22.1 The Law of Minimal Change

A Delta must be **bounded**. It should only contain the triples necessary to represent the specific transition it describes.

$$
\boxed{\textbf{A delta is the minimal set of relations that define a transition.}}
$$

If we are updating a status, the Delta should contain the new status triple. It should not contain a copy of the entire badge record. 

---

## 22.2 Additions and Deletions

In some Semantic Bit implementations, a Delta consists of two graphs:
1. **The Additions Graph ($G^+$)**: Triples to be admitted.
2. **The Deletions Graph ($G^-$)**: Triples to be superseded or "retracted".

However, in the strictest interpretation of the philosophy, we prefer **Successor Relations** over deletions. Instead of deleting "Status: Active", we admit "Status: Suspended" and use a selection rule to prioritize the most recent status.

---

## 22.3 The Delta as an Atomic Unit

A Delta is atomic. Either the entire Delta is admitted, or none of it is. We never admit a partial Delta.

This atomicity ensures that the graph remains in a consistent state. If a Delta represents a transfer of balance between two accounts, admitting only half the triples would violate the law of conservation.

---

## 22.4 The Delta Receipt

Every Delta must carry a receipt. The receipt links the Delta back to:
- The source inquiry that justified it.
- The template that formed it.
- The authority that constructed it.

$$
\boxed{\textbf{A delta without a receipt is an unverified motion.}}
$$

By bounding our changes as receipted deltas, we transform "database updates" into "semantic events".
