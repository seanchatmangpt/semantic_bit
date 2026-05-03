# Part IX — Canonicalization and Receipt

As our graphs flow through transformations and validation gates, we need a way to prove their provenance and integrity. If an automated agent generates a rust module from an RDF specification, how do we prove that the code corresponds *exactly* to that specific version of the specification?

In a text file, we might hash the bytes. However, RDF graphs are sets of triples; the order in which the triples are written in a file is irrelevant. A graph serialized as Turtle might look completely different byte-for-byte than the same graph serialized as JSON-LD, even though their semantic meaning is identical.

To solve this, we rely on **RDF Graph Canonicalization** (often using algorithms like URDNA2015 or RDFC-1.0). Canonicalization provides a deterministic way to sort and serialize the triples in a graph, producing a consistent byte stream regardless of the original input format.

Once a graph is canonicalized, we can compute a cryptographic hash (e.g., SHA-256) of it. This hash becomes the identity of that specific state of the graph.

We embed this hash into our **Receipts**. When a manufacturing step completes, it issues a receipt containing the hashes of the input graph, the transformation query, and the output graph. This creates an unbroken, mathematically provable chain of custody from the highest-level specification down to the individual bits in a control record.
