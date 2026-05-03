# The Second Book Begins After the Field

Book 1 established the semantic bit as a named position in a bounded field.

It demonstrated that a field can carry many active meanings, but only one selected condition. It showed that receipts preserve the field at the moment of consequence.

But Book 1 focused on small, specific fields: access badges, status codes, and journals. Each field was a custom structure designed for one purpose.

Book 2 moves after the custom field.

It introduces a universal carrier for 8-bit semantic multiplexing: the **Semantic Byte**.

$$
\boxed{\textbf{The semantic byte is an 8-bit carrier that admits any combination of eight named meanings within a bounded multiplexing law.}}
$$

In this book, we stop building custom fields for every problem. Instead, we learn to multiplex many meanings onto a single byte, using established laws to prevent conflict and ensure deterministic selection.

The field is no longer a collection of bits we named once. The field is a dynamic carrier of admitted combinations.

---

## Moving Beyond Decoration

In Book 1, we often treated field names as decoration for bits. \`BADGE_PRESENT\` was bit 0. \`BADGE_RECOGNIZED\` was bit 1.

In Book 2, we recognize that the *byte itself* is the unit of admission. We do not ask "What does bit 0 mean?" in isolation. We ask "What combination of meanings does this byte currently carry?"

This shift in perspective is the foundation of multiplexing.

## The Plan for Book 2

We will follow this path:

1. **Decomposition:** How to break a byte into eight admitted meanings.
2. **Multiplexing Law:** How to combine meanings without losing technical integrity.
3. **The Spine:** The universal sequence from status to condition to operation.
4. **Rust Field Law:** How to enforce these laws in executable notation.

We begin by recalling the semantic bit, then widening our view to the byte.
