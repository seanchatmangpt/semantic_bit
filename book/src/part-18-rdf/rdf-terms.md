# RDF Terms

The triple is the record. The terms are the bits that compose the record.

In RDF, there are three types of terms. Each has a specific role in the admission of meaning.

$$
\boxed{\textbf{RDF admits three types of terms: IRIs for identity, Literals for value, and Blank Nodes for structure.}}
$$

---

## 18.1 IRIs: Admitted Identity

The IRI (Internationalized Resource Identifier) is the primary unit of identity.

An IRI is not a "link." It is a name that is globally unique and locally admitted. In the Semantic Bit philosophy, we use IRIs to name everything that requires a persistent identity: classes, properties, and specific instances.

- `<class:AccessAttempt>`
- `<rel:badge>`
- `<badge:41000123>`

IRIs allow us to speak across systems without collision.

---

## 18.2 Literals: Boundary Values

A Literal is a leaf node in the graph. It carries a raw value, often with a datatype.

Literals are the "boundary values" of the system. They represent the data that the system does not further decompose into relations.

- `"41000123"` (a string)
- `17` (an integer)
- `"2026-05-03T08:00:00Z"^^xsd:dateTime` (a timestamp)

Literals are never subjects in a triple. They are always objects. They are the terminal points of a relation.

---

## 18.3 Blank Nodes: Local Structure

A Blank Node is an identity without a global name. It is a local placeholder used to group relations together.

Blank nodes are used for "complex attributes" that do not deserve a first-class identity. For example, a postal address might be a blank node that connects a street, a city, and a postal code to a person.

In the Semantic Bit, we use blank nodes sparingly. We prefer explicit IRIs. However, blank nodes are necessary for representing structural patterns that are only valid within a single graph.

---

## 18.4 The Mapping of Terms

| Term Type | Role | Bounded? |
| :--- | :--- | :--- |
| **IRI** | Identity | Yes (Global) |
| **Literal** | Value | Yes (Datatype) |
| **Blank Node** | Structure | Yes (Local) |

Every position in a triple (Subject, Predicate, Object) is restricted by the type of term it may carry.

- **Subject**: IRI or Blank Node.
- **Predicate**: IRI.
- **Object**: IRI, Literal, or Blank Node.

---

## 18.5 The Law in This Chapter

The system does not accept arbitrary strings. It admits terms.

```text
The IRI names the actor.
The Literal carries the data.
The Blank Node groups the relation.
```

$$
\boxed{\textbf{A triple is only valid when its terms are admitted into their respective positions.}}
$$
