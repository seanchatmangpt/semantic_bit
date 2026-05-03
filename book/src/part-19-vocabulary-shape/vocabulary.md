# Vocabulary

A graph without a vocabulary is a collection of triples without a law.

In the Semantic Bit philosophy, we do not allow arbitrary relations to wander the surface. Every triple must correspond to a known vocabulary. A vocabulary is the "field definition" for the graph.

$$
\boxed{\textbf{A Vocabulary is an admitted set of classes and properties that defines the bounds of relation.}}
$$

---

## 19.1 The Schema as Law

In Book 1, we defined the `AccessField` as having 8 bits. This was a fixed-width law.

In Book 3, we define the `AccessVocabulary`. It defines the classes (what things are) and the properties (how they relate).

```turtle
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix class: <http://system.local/class/> .
@prefix rel:   <http://system.local/rel/> .

# Defining a class
class:Badge a rdfs:Class ;
    rdfs:label "Security Badge" .

# Defining a property
rel:heldBy a rdfs:Property ;
    rdfs:label "Held By" .
```

By defining these IRIs as `rdfs:Class` and `rdfs:Property`, we admit them into the system's legal framework.

---

## 19.2 Vocabulary vs. Ontology

A vocabulary is not an attempt to describe the universe. It is a technical specification for a specific operational field.

- An **Access Vocabulary** defines the bits and relations of security.
- A **Maintenance Vocabulary** defines the bits and relations of hardware.
- A **Network Vocabulary** defines the bits and relations of connectivity.

We use multiple vocabularies to keep the system modular. A triple may use classes from one vocabulary and properties from another, provided both are admitted.

---

## 19.3 The Admission of New Terms

A new property is not created by typing it into a triple. It is created by admitting it into a vocabulary.

If the system encounters `<badge:1> <rel:newProperty> "value"`, it must check if `rel:newProperty` is an admitted member of an active vocabulary. If not, the triple is a noise and must be rejected at the boundary.

---

## 19.4 The Law in This Chapter

The vocabulary is the guardrail of the graph.

```text
The triple is the motion.
The vocabulary is the map.
Motion without a map is a drift.
The map without admission is a ghost.
```

$$
\boxed{\textbf{No triple is admitted to the graph unless its predicate is defined in an admitted vocabulary.}}
$$
