# RDF Exercises

The student should complete these exercises to prove their understanding of the RDF relation surface.

---

## Exercise 18.1: Triple Decomposition

Decompose the following "prose facts" into a set of admitted RDF triples using the `badge`, `person`, and `rel` namespaces. Use Turtle notation.

1. "Badge 41000123 is held by Person 72."
2. "Person 72 is named 'Alice Smith'."
3. "Badge 41000123 was issued on May 3rd, 2026."

---

## Exercise 18.2: Term Identification

Identify the type of each RDF term (IRI, Literal, or Blank Node) and its admitted position in a triple (Subject, Predicate, or Object).

1. `<http://system.local/id/badge/1>`
2. `"2026-05-03"^^xsd:date`
3. `_:node123`
4. `<http://system.local/rel/heldBy>`
5. `"Alice Smith"@en`

---

## Exercise 18.3: Local Structure

Represent a "Repair Event" using a Blank Node. The repair event was for `badge:41000123`. It happened on `2026-06-01`. The technician was `person:101`. The result was `status:OK`.

---

## Exercise 18.4: Dataset Partitioning

Explain which graph (Identity, Status, or Audit) each of the following triples belongs to and why.

1. `<badge:1> <rel:heldBy> <person:72> .`
2. `<badge:1> <rel:lastAccess> "2026-05-03T10:00:00Z"^^xsd:dateTime .`
3. `<badge:1> a <class:SecurityBadge> .`
4. `<access:attempt/101> <rel:result> <status:GRANTED> .`

---

## Exercise 18.5: The Law of Admission

Write a short paragraph explaining why a raw string like `"GRANTED"` is not admitted as a triple subject, and what must be done to the string before the system can act upon it.
