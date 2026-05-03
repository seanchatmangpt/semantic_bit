# The Eight Condition Meanings

The `ConditionCode8` field defines exactly eight possible outcomes. A selection rule must map any combination of active statuses to exactly one of these eight conditions.

1. **OK**: The operation may proceed normally.
2. **WARN**: The operation may proceed, but irregularities were detected.
3. **BLOCKED**: The operation cannot proceed until an external dependency is resolved.
4. **RETRY**: The operation failed due to a transient issue and should be attempted again.
5. **ESCALATE**: The operation requires intervention from a higher authority or different system.
6. **REFUSE**: The operation is explicitly denied by system rules.
7. **ABEND**: The operation encountered an unrecoverable, abnormal failure.
8. **UNKNOWN**: The system cannot determine the outcome due to missing or contradictory evidence.

By restricting the possible conditions to these eight, we bound the complexity of the control flow that consumes them.
