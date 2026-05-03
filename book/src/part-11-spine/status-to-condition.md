# Status to Condition

The first movement within the Semantic8 Spine is the transition from **Status8** to **ConditionCode8**.

Raw reality presents itself as a multiplexed `Status8` field. It may be both `WARN` and `OK`, or `STALE` and `BLOCKED`. Status describes what *is* at a given boundary.

Before any operation can be considered, this multiplexed reality must be collapsed into a singular, unambiguous mandate. This is the role of `ConditionCode8`.

A selection rule is applied to the `Status8` field, identifying the highest-priority state and mapping it to a single `ConditionCode8` (e.g., `RETRY`, `ESCALATE`, `OK`, `ABEND`). This ensures that subsequent steps in the Spine only ever deal with one actionable condition at a time.
