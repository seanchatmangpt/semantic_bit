# Manufacture Before Authorship

We do not "write" complex systems. we **manufacture** them from triples.

**Authorship** is the act of a human typing code or text. It is prone to error, drift, and inconsistency.
**Manufacture** is the act of a machine generating artifacts from a source of truth.

$$
\boxed{\textbf{Triples are the source. Code is the artifact. Manufacture is the bridge.}}
$$

## 15.16 The Drift of the Manual Hand

When a developer manually writes a Rust struct and then manually writes a documentation page for it, they have introduced "Drift". If the struct changes, the docs will eventually lie.

## 15.17 The Manufacturing Loop

In the Semantic Bit philosophy:
1. We define the system structure as a set of triples in a graph.
2. We use a manufacturing tool (like `unrdf`) to read the graph.
3. The tool generates the Rust code, the mdBook chapters, and the validation schemas.

The triples are the "Meaning". The generated code is the "Motion". By manufacturing before authoring, we ensure that the motion never deviates from the meaning.
