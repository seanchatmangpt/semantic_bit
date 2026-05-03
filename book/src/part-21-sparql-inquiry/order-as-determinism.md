# ORDER as Determinism

In a Semantic Bit system, we value **determinism**. A process that produces different results from the same input is a process carrying entropy.

Graphs are unordered collections of triples. When we project these triples into a result set, the sequence of results is naturally undefined. `ORDER BY` is the mechanism we use to restore determinism to the inquiry.

## 21.1 Avoiding the Entropy of Randomness

Without `ORDER BY`, a SPARQL engine may return results in any order. If the next step in our system (such as a receipt generator or a manufacturing tool) depends on the sequence of inputs, this randomness will lead to inconsistent outputs.

```sparql
SELECT ?logEntry
WHERE {
  ?logEntry a :SystemLog .
}
ORDER BY ?logEntry
```

By ordering the results, we ensure that every time this inquiry is run against the same graph, the resulting list of `?logEntry` IRIs will be identical.

$$
\boxed{\textbf{Ordering is the rejection of sequence entropy.}}
$$

---

## 21.2 Deterministic Receipting

When we generate a receipt for an inquiry, the receipt must be reproducible. If the inquiry returns a list of items, the hash of that list will change if the items are in a different order.

To ensure **Canonical Receipting**, every inquiry that returns multiple results MUST use `ORDER BY` on a stable identifier (usually the IRI of the subject).

---

## 21.3 Order is not Meaning

It is important to distinguish between *meaningful* order (like a timestamp) and *deterministic* order (like an IRI string).

*   **Meaningful Order**: `ORDER BY DESC(?timestamp)` (We care about the most recent).
*   **Deterministic Order**: `ORDER BY ?id` (We care about consistency).

A robust inquiry often uses both:

```sparql
ORDER BY DESC(?timestamp) ?id
```

This ensures that the most recent entries come first, and if two entries have the same timestamp, their relative order is still deterministic.

---

## 21.4 The Law of the Stable Sequence

If the output of an inquiry is used to manufacture a secondary artifact (like a Rust file or a book page), the inquiry must be ordered.

$$
\boxed{\textbf{Manufacture requires stable projection. Stable projection requires order.}}
$$
