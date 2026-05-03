# Property

A Property is the definition of a relation.

In the Semantic Bit, a property is not a field in a record; it is an independent identity that acts as the predicate in a triple.

$$
\boxed{\textbf{A Property is an admitted identity that defines the nature of a relation between a subject and an object.}}
$$

---

## 19.1 The Identity of Relation

Just as a Class is an IRI, a Property is an IRI.

```turtle
<rel:heldBy> a rdf:Property ;
    rdfs:label "Held By" .
```

By making the relation an identity, we can say things about the relation itself. We can define its labels, its comments, and its constraints.

---

## 19.2 Properties Are Independent

A property is not "owned" by a class.

In OOP, a `Badge` has an `id`. In RDF, the property `<rel:id>` is a universal relation that can be applied to a `Badge`, a `Person`, or a `Door`.

This independence is powerful. It allows the system to reuse relations across different contexts. A `<rel:status>` property can describe a door, a badge, or a system process, provided the vocabulary admits it.

---

## 19.3 Subproperties

Properties can form hierarchies using `rdfs:subPropertyOf`.

```turtle
<rel:primaryHolder> rdfs:subPropertyOf <rel:heldBy> .
```

If the system admits that "Person A is the primary holder of Badge 1," it automatically admits the broader relation that "Person A is a holder of Badge 1." This allows for specific relations to satisfy general requirements.

---

## 19.4 The Law in This Chapter

The property is the bridge of the triple.

```text
The subject is the source.
The object is the target.
The property is the bridge.
A bridge without a plan is a collapse.
```

$$
\boxed{\textbf{A relation is only as valid as the property that defines it.}}
$$
