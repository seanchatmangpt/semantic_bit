# Semantic Translation

Data is not meaning. Translation is the act of admitting data into the semantic field.

In most systems, translation is hidden in procedural code. In the Semantic Bit, translation is a declared operation. We translate the "prose" of traditional data sources into the "positions" of the semantic field.

$$
\boxed{\textbf{Translation maps the ambiguity of input to the precision of the field.}}
$$

## 24.1 From String to Bit

Consider a traditional JSON payload:

```json
{
  "badge_status": "active",
  "door_id": "D101",
  "timestamp": "2024-05-20T10:00:00Z"
}
```

This string-heavy representation is ambiguous. "active" could be a typo, and "D101" is a label, not a law.

Semantic translation maps these values to admitted bits:

1.  `"active"` → `HOLDER_ACTIVE` (Position 3 in Access Field)
2.  `"D101"` → `DOOR_ALLOWED` (Position 4 in Access Field, after lookup)

The translation is not a guess. It is governed by a selection rule that requires evidence.

## 24.2 The Translation Rule

A translation rule is a SPARQL CONSTRUCT query or a SHACL shape that validates and transforms input.

```sparql
CONSTRUCT {
  ?attempt sb:hasField ?field .
  ?field sb:activeBit sb:HOLDER_ACTIVE .
}
WHERE {
  ?input sbp:badge_status "active" .
}
```

This query is the law of translation. It does not "parse" the string; it "constructs" the semantic fact.

## 24.3 Lossy vs. Lossless Translation

Translation into the semantic field is often lossy by design. We discard the prose, the whitespace, and the incidental metadata that does not contribute to the operational decision.

What remains is the **Semantic Residue**: the minimal set of bits required to act.

$$
\boxed{\textbf{We discard the story to preserve the state.}}
$$

## 24.4 Translation Receipts

Every translation must emit a receipt. The receipt links the raw input (by hash) to the resulting semantic field. If the translation is challenged, the system can replay the rule against the original input to verify that the bits were correctly admitted.

Without a receipt, the translated field is unauthenticated motion.
