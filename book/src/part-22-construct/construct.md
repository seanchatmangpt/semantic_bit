# CONSTRUCT as Lawful Change

Inquiry asks "What is?". **Construction** asks "What must be?".

The `CONSTRUCT` query is the mechanism for generating new relations from existing ones. In the Semantic Bit philosophy, we do not mutate existing triples. Instead, we construct a **Delta**—a new graph that represents the intended state of the system.

## 22.1 The Generator of Relations

A `CONSTRUCT` query defines a mapping from an inquiry to a template.

$$
Inquiry \rightarrow Template \rightarrow Delta
$$

Every triple in the resulting Delta is a product of this mapping. No triple is created in isolation; every new relation must be rooted in an existing, admitted pattern.

---

## 22.2 The Purpose of Construction

We use `CONSTRUCT` for three primary reasons:

1. **Inference**: Creating higher-level meanings from raw signals (e.g., if a badge is present and the time is allowed, construct the `GRANTED` condition).
2. **Translation**: Converting between different vocabularies or shapes.
3. **Transition**: Defining the next state of a process (e.g., a "Pending" request becoming an "Active" record).

---

## 22.3 The Law of No Side-Effects

Executing a `CONSTRUCT` query does not change the source graph. It produces a value—the **Constructed Graph**. This value is then evaluated, receipted, and potentially admitted into the system's memory in a separate, bounded step.

$$
\boxed{\textbf{Construction is a pure function. It produces a graph without altering the world.}}
$$

By treating change as construction rather than mutation, we preserve the auditability and reproducibility of the system.
