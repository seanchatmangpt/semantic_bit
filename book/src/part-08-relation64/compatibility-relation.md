# Compatibility Relation

A **Compatibility Relation** asserts that two nodes can safely interact. It usually connects an `OBJECT` to a `RULE` or an `OBJECT` to another `OBJECT`.

In systems without bounded relations, compatibility is often discovered at runtime, usually resulting in a crash when a function tries to parse a JSON string that is actually an integer.

In Semantic8, compatibility is asserted explicitly. Before an operation binds two records together, or before a record is deposited into a channel, a Compatibility Relation must be established. This relation proves that the fields multiplexed within the Source align with the selection rules demanded by the Target.

If a Compatibility Relation cannot be proven—either statically via the type system or dynamically via a prior receipt—the interaction is refused. This moves the failure from an unpredictable runtime exception to an explicit, receipted contract violation.
