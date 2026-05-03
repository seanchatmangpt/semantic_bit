# COG8 Selection

Like all Semantic8 fields, `COG8` is a multiplexed field of independent bits. It is not an enum.

However, the operation contract dictates the *selection rule*. 

For a highly secure operation, the selection rule might require:
`CLOSED | EVIDENCED | AUTHORIZED | FRESH | CONSISTENT | REPLAYABLE`

If the active bits in `COG8` match or exceed the mask defined by the operation's selection rule, the cognitive check passes, and the system may proceed to the `CONSTRUCT8` phase.

If even one required bit is missing, the cognitive check fails, the operation is blocked, and a new `ConditionCode8` (like `REFUSE` or `BLOCKED`) is generated.
