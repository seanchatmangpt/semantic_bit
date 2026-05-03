# IRIs as Admitted Identity

In the 8-bit field, we used numeric positions. `1 << 0` is the position for `BADGE_PRESENT`.

In the global graph, we use IRIs.

An IRI is a formal identity that must be admitted into the system before it is trusted. It is the mechanism that allows us to move from "some badge" to "this specific badge" across any system boundary.

$$
\boxed{\textbf{An IRI is the unique, admitted name of a subject, predicate, or object.}}
$$

---

## 18.1 Not a URL

The most common mistake is to treat an IRI as a URL (Uniform Resource Locator).

A URL is a location. It tells you where to find a file.
An IRI is an identity. It tells you what something is.

While many IRIs use the `http` scheme, the Semantic Bit does not require them to be "dereferenceable." The system does not need to visit a website to admit the identity. It only needs to recognize the name.

---

## 18.2 Namespaces as Admission Fields

IRIs are organized into namespaces. A namespace is like a bounded field for names.

By using namespaces, we avoid collisions.

- `badge:41000123` belongs to the `badge` namespace.
- `door:17` belongs to the `door` namespace.
- `rel:heldBy` belongs to the `rel` namespace.

In our Turtle notation, we define prefixes to manage these fields:

```turtle
@prefix badge: <http://system.local/id/badge/> .
@prefix rel:   <http://system.local/rel/> .

badge:41000123 rel:heldBy <person:72> .
```

---

## 18.3 The Badge as Identity

In Book 1, the badge was a `u64` identifier.

In Book 3, the badge is an IRI. The `u64` identifier is now a component of the IRI: `badge:41000123`.

This lift allows the badge to participate in relations that the `u64` could not reach. We can relate the badge to its manufacturer, its last maintenance record, and its assigned holder, all using the same admitted identity.

---

## 18.4 The Law in This Chapter

Identity is the first requirement for relation.

```text
The number identifies a position.
The IRI identifies a node.
A node without a name is a ghost.
A name without admission is a noise.
```

The system remains authoritative because it only acts on identities that have been admitted to its namespaces.

$$
\boxed{\textbf{Admission precedes identity. Identity precedes relation.}}
$$
