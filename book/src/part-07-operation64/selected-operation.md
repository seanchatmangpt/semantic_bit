# Selected Operation

A Declared Operation is merely an intent. Intent is not consequence. The system must now evaluate the declaration against its authority and dependency rules.

When a Declared Operation is approved for execution, it becomes a **Selected Operation**.

The process of moving from Declaration to Selection involves checking the **Operation Authority** and the **Operation Input Contract**. Only if both pass does the operation proceed. 

A Selected Operation is fully deterministic. It has the right noun, the right verb, the right identities, and the right authority. It is now guaranteed to execute and produce a known subset of possible resulting conditions.

In a multiplexed system, you might have multiple operations declared simultaneously (for instance, an `OK` condition might declare both `EMIT RECEIPT` and `ACTIVATE FIELD`). The selection rule for operations determines the precedence: which operation is selected first, and whether the subsequent operations are still valid after the first one executes.
