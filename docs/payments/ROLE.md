# Payments Role: semantic_bit

Supplies a std-only compact canonical digest and `effect_id` derivation
(`src/effect_identity.rs`). Operational construct; no standards conformance is claimed.

## Falsifier coverage

| id | coverage |
|---|---|
| F2 | `every_bound_field_changes_the_id_and_is_refused` (10 fields) |
| F4 | one deterministic id joins obligation, authority, rail, reservation |
| F9 | canonical bytes are injective and reproducible for replay |

## See Also

`src/effect_identity.rs` · `tests/effect_identity.rs`
