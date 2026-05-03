# FILTER as Guard

While the `WHERE` clause defines the structural boundary, the `FILTER` clause defines the **logical guard**.

A guard is a condition that must evaluate to true for a match to be admitted. If the guard fails, the match is discarded, even if the structure was correct.

## 21.1 Logic Before Motion

We do not act on every structural match. We act only on matches that satisfy our semantic laws.

```sparql
SELECT ?sensor ?value
WHERE {
  ?sensor :hasReading ?value .
  FILTER(?value > 100)
}
```

In this inquiry, the structure (a sensor with a reading) is present, but the guard (`?value > 100`) ensures that we only project readings that exceed a specific threshold.

$$
\boxed{\textbf{A guard rejects motion that lacks meaning.}}
$$

---

## 21.2 Semantic Guards

Guards are often used to enforce temporal or authority constraints:

```sparql
FILTER(?expiryDate > NOW())
FILTER(?securityLevel >= :High)
```

These are not "business rules" in the traditional sense; they are **admission laws**. If a badge has expired, it is no longer an admitted badge for the purpose of the current inquiry. The guard enforces this law at the point of inquiry.

---

## 21.3 The Difference Between WHERE and FILTER

It is a common mistake to use `FILTER` when a structural pattern in `WHERE` would be more precise.

*   **Structural**: `?person :hasRole :Admin .` (The relation must exist)
*   **Guard**: `FILTER(?age >= 18)` (The value must meet a condition)

Use structural patterns to define *what* something is. Use guards to define *how* it must behave or what state it must be in.

---

## 21.4 Negation as a Guard

The `FILTER NOT EXISTS` construct is a powerful guard. It ensures that a match is only admitted if a certain relation is **absent**.

```sparql
WHERE {
  ?badge a :AccessBadge .
  FILTER NOT EXISTS { ?badge :isSuspended true }
}
```

Here, the guard protects the system from admitting suspended badges. Absence of a "Suspended" relation is a requirement for the "Active" condition.

$$
\boxed{\textbf{Absence of evidence for a block is evidence for a pass.}}
$$
