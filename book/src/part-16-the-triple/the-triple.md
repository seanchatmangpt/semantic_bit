# The Triple

The triple is the fundamental unit of relation. It consists of three parts: a **Subject**, a **Predicate**, and an **Object**.

$$
\boxed{Triple = (Subject, Predicate, Object)}
$$

## 16.1 The Atom of Meaning

Just as the bit is the atom of state, the triple is the atom of relation. You cannot have a relation with fewer than three parts. If you try to remove the predicate, you have a pair of identities with no meaning. If you remove the object, you have a subject with a dangling intent.

## 16.2 Bounded Representation

In the Semantic Bit architecture, a triple is represented as a fixed-width record. This ensures that the machine can process triples with the same efficiency it processes bits.

```rust
pub struct Triple {
    pub subject: u64,
    pub predicate: u64,
    pub object: u64,
}
```

By using `u64` identifiers, we bound the identities involved. We do not allow the "Subject" to be an arbitrary string of characters. It must be an admitted identifier.
