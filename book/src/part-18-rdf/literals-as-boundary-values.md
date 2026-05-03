# Literals as Boundary Values

If an IRI is a point of identity, a Literal is a point of data.

In the Semantic Bit philosophy, we decompose every complex concept into relations until we reach a boundary. That boundary is the Literal.

$$
\boxed{\textbf{A Literal is an admitted value that marks the edge of the relation graph.}}
$$

---

## 18.1 The Terminal Node

A Literal is a terminal node. It has no outgoing relations. It cannot be the subject of a triple.

When a triple has a Literal as its object, the relation is "valued."

```turtle
<badge:41000123> <rel:label> "Primary Security Badge" .
<badge:41000123> <rel:issueDate> "2026-01-01"^^xsd:date .
```

The system does not ask "What is 'Primary Security Badge'?" It simply admits the string as a property of the badge.

---

## 18.2 Datatypes are Selection Rules

A raw string is dangerous. `"2026-01-01"` is just a sequence of characters until a datatype is applied.

In RDF, we use XSD (XML Schema Definition) datatypes to bound the meaning of Literals.

- `xsd:string`: A sequence of characters.
- `xsd:integer`: A whole number.
- `xsd:dateTime`: A precise point in time.
- `xsd:boolean`: `true` or `false`.

By applying a datatype, we turn a string into an admitted value. If the value does not match the datatype, it is not admitted to the field.

---

## 18.3 Language Tags

For human-readable strings, RDF admits language tags. This is the one place where "prose" is allowed to touch the graph, but it remains bounded by the language identifier.

```turtle
<door:17> <rel:description> "Front Door"@en .
<door:17> <rel:description> "Porte d'entrée"@fr .
```

The system does not "understand" the English or French. It simply knows which string to select based on the language bit.

---

## 18.4 The Law in This Chapter

Literals are the data that feeds the decision, but they are not the decision themselves.

```text
The IRI is the actor.
The Literal is the script.
The datatype is the guard.
The value is the boundary.
```

The graph remains disciplined by ensuring that every Literal is typed and every type is admitted.

$$
\boxed{\textbf{Data is not truth. Data is a literal value admitted into a typed boundary.}}
$$
