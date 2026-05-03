# SPARQL

In the Semantic Bit architecture, we do not "search" for data. We **Inquire** into the field.

**SPARQL** (SPARQL Protocol and RDF Query Language) is admitted as our formal language of inquiry. It allows us to express complex patterns of relation as a single, bounded request for evidence.

---

## 21.1 Inquiry Before Action

Just as a physical reader must inquire into the state of an access badge before opening a door, a semantic system must inquire into the graph before admitting a motion.

SPARQL provides the mechanism to test whether a set of triples satisfies an operational requirement.

$$
\boxed{\textbf{SPARQL is the executable logic of inquiry. It maps a pattern of triples to a set of admitted values.}}
$$

---

## 21.2 The Query as a Contract

In traditional systems, a "query" is often seen as a way to "fetch" data for display. In the Semantic Bit, a SPARQL query is a **Contract**.

The query defines exactly which relations must be present for an operation to proceed. If the query returns no results, the condition is not met, and the motion is refused.

By using SPARQL, we move the logic of "selection" from procedural code into a declarative, machine-readable form.

---

## 21.3 The Four Forms of Inquiry

SPARQL provides four primary ways to interact with the graph, each serving a specific role in the Semantic Bit loop:

| Form | Operational Role |
| :--- | :--- |
| `ASK` | Simple condition check (Yes/No). |
| `SELECT` | Projection of admitted identities. |
| `CONSTRUCT` | Lawful construction of new relations. |
| `DESCRIBE` | Discovery of a bounded identity. |

In this part, we focus on `ASK` and `SELECT` as tools for **Inquiry**. We reserve `CONSTRUCT` for Part 22, where it serves as the mechanism of **Lawful Change**.

---

## 21.4 Inquiry Is Not Search

We reject the idea of "fuzzy search" in the critical path. An inquiry must be **Deterministic**.

A SPARQL query in a Semantic Bit system should be anchored by identities (IRIs) and bounded by shapes (SHACL). We do not guess which triples we are looking for; we specify the exact relations that constitute the evidence for our next motion.

---

## 21.5 The Law in This Chapter

```text
Inquiry precedes admission.
SPARQL is the formal language of bounded inquiry.
A query is a contract for evidence.
Inquiry must be deterministic and anchored by identity.
The result of an inquiry is the basis for selection.
```

The system does not wander through data. It inquires into the field to find the truth of the relation.
