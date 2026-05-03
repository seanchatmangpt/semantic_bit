# RDF Is Not Object Ontology

A common error in system design is the attempt to model "the real world."

Traditional object-oriented programming (OOP) and classical ontologies (OWL) often fall into this trap. They create deep hierarchies of classes, attempt to define the "essence" of an object, and use inheritance to share properties.

The Semantic Bit philosophy rejects this. We do not model the world. We admit relations.

$$
\boxed{\textbf{RDF is not a tool for modeling objects. It is a tool for recording relations.}}
$$

---

## 18.1 The Failure of Essence

In an object-oriented system, a `Badge` is a class. It has properties like `id` and `holder`. If you need to add a `maintenance_date`, you must modify the class or create a subclass.

This assumes the `Badge` has an essence that includes maintenance.

In the relation surface, there is no `Badge` class in the OOP sense. There is only an identity (the IRI) and the relations it participates in.

- **Relation 1**: Identity A is recognized as a Badge.
- **Relation 2**: Identity A is held by Identity B.
- **Relation 3**: Identity A was maintained on Date C.

These relations are independent. They may be admitted by different systems at different times.

---

## 18.2 The Open World vs. The Bounded Field

RDF is built on the "Open World Assumption" (OWA). This is often misinterpreted as "anyone can say anything about anything."

In an operational system, OWA means something different: **The absence of a triple is not a proof of a negative.**

If the graph does not contain `<badge:1> <rel:blocked> "true"`, the system does not "know" the badge is unblocked. It only knows that the `blocked` relation has not been admitted.

This is why we use **Selection Rules** and **SHACL Admission** (covered in Part 20). We bound the open world before we act.

---

## 18.3 Predicates Are Not Methods

In OOP, a method belongs to a class. `badge.check_access()` is code owned by the `Badge` object.

In RDF, a predicate is an independent identity. `<rel:allowsAccessTo>` is not owned by the badge or the door. it is an admitted relation type that connects them.

This separation allows the system to remain modular. We can add new predicates without touching the subjects or objects.

---

## 18.4 The Law in This Chapter

We do not ask "What is a Badge?" We ask "What relations are admitted for this identity?"

```text
Objects have essence.
Identities have relations.
Essence is a guess.
Relation is a record.
```

The system remains technical and professional by avoiding the philosophical quagmire of ontology and sticking to the mechanical admission of triples.

$$
\boxed{\textbf{Identity is a point. Relation is a vector. The graph is the field.}}
$$
