# Domain

A property does not exist in a vacuum. It expects a certain type of subject.

In RDF, we use `rdfs:domain` to specify the class of the subject that a property is intended to describe.

$$
\boxed{\textbf{The Domain of a property is the admitted class of its subject.}}
$$

---

## 19.1 Bounding the Source

Consider the property `<rel:heldBy>`. It makes sense for a `Badge` to be held by a `Person`. It does not make sense for a `Literal` or a `SystemReboot` to be held by a `Person`.

We define the domain to bound the source of the relation:

```turtle
<rel:heldBy> a rdf:Property ;
    rdfs:domain <class:Badge> .
```

This triple informs the system that whenever `<rel:heldBy>` is used, the subject is inferred (or required) to be a `<class:Badge>`.

---

## 19.2 Inference as Admission

In classical RDF, `rdfs:domain` is an inference rule. If the system sees:

```turtle
<x> <rel:heldBy> <person:72> .
```

It automatically infers:

```turtle
<x> a <class:Badge> .
```

In the Semantic Bit philosophy, we treat this inference as an **Admission Warning**. If the subject `<x>` was not already admitted as a `<class:Badge>`, the use of `<rel:heldBy>` is a structural anomaly. It suggests the system is making a relation it hasn't properly categorized.

---

## 19.3 Multiple Domains

If a property has multiple domains, the subject is inferred to be a member of *all* of them (the intersection). This is often a mistake in vocabulary design. We prefer to define a common superclass if a property applies to multiple categories.

---

## 19.4 The Law in This Chapter

The domain guards the start of the relation.

```text
The property points away from the subject.
The domain defines the ground where the property stands.
If the ground is not admitted, the point is a drift.
```

$$
\boxed{\textbf{A property must stand on an admitted domain.}}
$$
