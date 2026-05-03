# Template Triple

The `CONSTRUCT` clause contains the **Template**. The template consists of one or more **Template Triples**.

A template triple is a pattern that defines the structure of the new relations. It uses variables from the `WHERE` clause to fill its positions.

## 22.1 Structure of the Template

```sparql
CONSTRUCT {
  ?badge :hasStatus :Active .
}
WHERE {
  ?badge a :AccessBadge .
  ?badge :isRecognized true .
}
```

In this example, the template triple is `?badge :hasStatus :Active .`. 
For every `?badge` matched in the `WHERE` clause, a new triple will be generated in the resulting Delta.

$$
\boxed{\textbf{A template triple is the blueprint for a new fact.}}
$$

---

## 22.2 Mapping and Transformation

Templates allow us to transform one relation into another:

```sparql
CONSTRUCT {
  ?person :canEnter ?room .
}
WHERE {
  ?person :hasBadge ?badge .
  ?badge :allowedIn ?room .
}
```

Here, the template triple `?person :canEnter ?room` does not exist in the source graph. It is a **derived relation**. It is constructed by joining the `?person` and `?room` variables discovered during the inquiry.

---

## 22.3 Constant Injection in Templates

Template triples can also include constants that were not present in the inquiry:

```sparql
CONSTRUCT {
  ?attempt :processedBy :SecuritySystem_V2 .
}
```

This ensures that all constructed triples carry a specific "mark" or "authority" from the system that performed the construction.

---

## 22.4 The Law of Template Validity

A template triple must result in a valid RDF triple. If a variable in the template is not bound during the inquiry, that specific triple cannot be generated.

$$
\boxed{\textbf{Unbound variables result in unconstructed triples.}}
$$

This ensures that we never construct partial or "ghost" relations. Every constructed fact must be fully grounded in the evidence provided by the inquiry.
