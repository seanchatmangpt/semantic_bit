# Pattern Identity

Every pattern cell must be uniquely identifiable within the semantic universe. A pattern is not just "some" access control; it is **this** access control pattern, governed by **this** version of the law.

$$
\boxed{\textbf{Pattern identity is the stable IRI that uniquely designates a pattern cell's definition and its admitted structure.}}
$$

---

## 24.1 Identification by IRI

We use Internationalized Resource Identifiers (IRIs) to name patterns. An IRI provides a global namespace, preventing collisions between different engineering traditions.

```turtle
# The identity of the Access Control pattern
@prefix pattern: <https://semanticbit.io/pattern/> .

pattern:AccessControl a pattern:Cell ;
    rdfs:label "Access Control Pattern" ;
    pattern:version "1.0.0" .
```

---

## 24.2 Identification by Digest

While the IRI names the *definition* of the pattern, a specific *instance* of a pattern in a graph is identified by the digest of its relations.

If two graphs contain the exact same set of relations forming an "Access Control" cell, the pattern cell instances are identical. If one triple changes, the identity of that specific instance changes.

---

## 24.3 The Identity of Law

A pattern cell's identity is inextricably linked to the law that governs it. If the selection rule for a pattern changes, it is effectively a new pattern.

$$
\boxed{\textbf{The identity of the law is the identity of the pattern.}}
$$

---

## 24.4 The Pattern Identity Law

The law of pattern identity ensures that we always know what we are observing:

```text
A pattern is named by an admitted IRI.
The definition is versioned and stable.
An instance is identified by its stable graph form.
Change the law, and you change the identity.
```

The system does not act on anonymous structures. It acts on identified patterns.
