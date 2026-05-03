# Class

In the Semantic Bit, a Class is a category of identity.

It is not a "blueprint" for an object in the OOP sense. It is a named point in the vocabulary that indicates what kind of relations an identity is expected to participate in.

$$
\boxed{\textbf{A Class is an admitted category that groups identities for the purpose of relation admission.}}
$$

---

## 19.1 The `rdf:type` Relation

We associate an identity with a class using the `rdf:type` predicate (often abbreviated as `a` in Turtle).

```turtle
<badge:41000123> a <class:Badge> .
<person:72> a <class:Person> .
```

This triple is the "admission of category." Once a badge is admitted as a member of `class:Badge`, the system can apply the laws associated with that class.

---

## 19.2 Classes are Not Essences

As we discussed in Part 18, a class does not define what something *is* in its essence. It defines how the system *treats* it.

An identity can belong to many classes:

```turtle
<badge:41000123> a <class:Badge> , <class:InventoryItem> , <class:RadioEmitter> .
```

Each class admission brings a new set of possible relations. The `Badge` class admits access relations. The `InventoryItem` class admits financial relations. The `RadioEmitter` class admits regulatory relations.

---

## 19.3 Subclasses

RDF allows for a hierarchy using `rdfs:subClassOf`. This is not for inheritance of code, but for the propagation of admission.

```turtle
<class:SecurityBadge> rdfs:subClassOf <class:Badge> .
```

If the system admits a relation for all `Badge` members, it automatically admits it for all `SecurityBadge` members.

---

## 19.4 The Law in This Chapter

Class admission is the first step in bounding a triple.

```text
The identity is a point.
The class is a circle.
The point within the circle is admitted.
The circle without a definition is a hole.
```

$$
\boxed{\textbf{Category admission precedes relation admission.}}
$$
