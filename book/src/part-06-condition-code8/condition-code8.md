# ConditionCode8

The **ConditionCode8** field represents the resolved outcome of the system after a selection rule has been applied to raw field data (such as `Status8`). Where `Status8` may have multiple active states reflecting real-world complexity, `ConditionCode8` is the deterministic simplification of those states into exactly one actionable outcome.

A system cannot act on "maybe." It cannot act on "both." It must take one discrete path forward. ConditionCode8 provides the eight fundamental paths a bounded semantic system is permitted to take.

By representing conditions as a bounded eight-bit field, we ensure that every system component knows exactly how to handle the outcome of an operation without resorting to unbounded text parsing or endless boolean checks.
