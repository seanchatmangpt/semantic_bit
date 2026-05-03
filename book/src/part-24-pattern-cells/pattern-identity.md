# Pattern Identity

A pattern is not a suggestion. It is a named structure with a stable identity.

In the semantic field, we do not admit anonymous structures. Every pattern cell must be identified by a unique URI or a stable numeric index. This identity allows the system to refer to the pattern across different layers of manufacture without ambiguity.

Identity is the first requirement of admission.

$$
\boxed{\textbf{A pattern without identity is noise. A pattern with identity is a cell.}}
$$

## 24.1 The Stable URI

We use Internationalized Resource Identifiers (IRIs) to provide global identity to pattern cells. An IRI does not point to a location; it names a concept.

For example, the identity of the Access Field pattern might be:

`https://semantic-bit.org/pattern/access-field`

This identity remains constant even if the physical representation of the field changes from a `u8` to a `u64`. The identity binds the semantic meaning to the structural requirement.

## 24.2 Numeric Aliasing

In the critical operational path, IRIs are too wide. We alias stable IRIs to fixed-width numeric identities.

| IRI Alias                           | Numeric ID | Semantic Cell      |
| :---------------------------------- | ---------: | :----------------- |
| `sb:pattern/access-field`           |     `0x01` | Access Field       |
| `sb:pattern/control-record`         |     `0x02` | Control Record     |
| `sb:pattern/status-8`               |     `0x03` | Status Field       |

The numeric ID is used for high-speed selection and dispatch. The IRI is used for manufacture and auditing.

## 24.3 Identity Persistence

The identity of a pattern must persist through the manufacturing loop. When a Rust struct is generated from an RDF graph, the identity of the source pattern must be preserved in the metadata of the generated artifact.

If the identity is lost, the receipt cannot be verified.

$$
\boxed{\textbf{The receipt preserves the identity of the law that governed the action.}}
$$
