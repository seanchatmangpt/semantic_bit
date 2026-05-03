# Core Does Not Speak Strings

Moving to triples does not mean returning to the world of raw text.

In many RDF implementations, triples are filled with long URLs and strings. In the **Semantic Bit** architecture, the core remains numeric.

$$
\boxed{\textbf{Names are for humans. Triples are for machines. Identities are for the core.}}
$$

## 15.10 The Identifier as Boundary

An identity in a triple is a `u64` or a `u128`. It is a pointer to an admitted identity in a registry.
- `Subject: 0x0001`
- `Predicate: 0x000A`
- `Object: 0x0002`

The "String" (e.g., "AdminUser") is a decoration that exists *outside* the decision core. 

## 15.11 Why This Matters

If the core speaks strings, it must perform string comparisons, handle encodings, and manage memory. This introduces "Motion" before "Meaning". By keeping the core numeric, we ensure that every relation check is a constant-time integer operation.

We use **Vocabulary** to map these numbers to human names, but the machine only ever sees the admitted IDs.
