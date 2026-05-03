# Stable Graph Form

A graph carries meaning through the relations it admits. However, the physical representation of these relations in a file or memory is often non-deterministic. Triples may appear in any order. Blank nodes may be assigned different identifiers.

For a receipt to preserve a graph, the graph must first be reduced to a **stable form**.

$$
\boxed{\textbf{Stable graph form is the deterministic representation of an admitted graph that ensures identity between logically equivalent relations.}}
$$

---

## 23.1 The Problem of Order

In RDF, a graph is a set of triples. A set has no inherent order.

```turtle
# Form A
:Badge_41 :admits :Door_17 .
:Door_17 :requires :Badge_41 .

# Form B
:Door_17 :requires :Badge_41 .
:Badge_41 :admits :Door_17 .
```

Form A and Form B represent the same graph. However, if they are hashed directly, they will produce different digests. Stable form requires a deterministic sorting rule.

1. Sort triples by Subject IRI.
2. Sort triples with the same Subject by Predicate IRI.
3. Sort triples with the same Subject and Predicate by Object (IRI or Literal).

---

## 23.2 The Problem of Blank Nodes

Blank nodes are local structures without global identifiers. In different serializations, the same blank node may be labeled `_:b1` or `_:genid1`.

Stable form requires a canonical labeling algorithm. Blank nodes must be renamed based on the structure of the relations they participate in. If two blank nodes have identical related structures, they are indistinguishable and must be handled according to the graph's admission boundary.

---

## 23.3 Canonicalization as Admission

Canonicalization is not merely a formatting step. it is the final gate before a graph is receipted.

$$
\boxed{\textbf{Only a graph in stable form can be admitted to a receipt chain.}}
$$

By enforcing stable form, we ensure that:
- **Equality is bit-perfect**: Logically identical graphs produce identical digests.
- **Replay is deterministic**: A manufacturing process starting from a stable graph will always produce the same output.
- **Audit is transparent**: Any observer can verify the receipt by re-canonicalizing the source and comparing the digest.

---

## 23.4 The Stable Form Law

The law of stable form ensures that meaning is preserved across physical boundaries:

```text
A graph is a set of relations.
Serializations are many; meaning is one.
Stable form enforces deterministic order.
Canonical labeling resolves blank node identity.
The digest of the stable form is the graph identity.
```

The receipt does not preserve the file. It preserves the stable form of the admitted relations.
