# Activation Is Not Selection

The most critical distinction in the Semantic8 architecture is the gap between **Activation** and **Selection**. Failure to respect this gap is where most operational errors in traditional software originate.

## Activation: The Field of Truth

Activation is the state of the raw carrier. When multiple semantic bits are set in an 8-bit field, we say they are "active." Activation is additive and parallel. 

- If `INITIALIZED` is bit 0 and `ACTIVE` is bit 1, a field carrying `0x03` has two active meanings.
- Activation is a record of *observation*. It simply notes what conditions have been admitted into the field based on available evidence.

## Selection: The Decision of Consequence

Selection is the mapping of a multiplexed field onto a single, non-overlapping **Condition**. While activation is many, selection must be one.

The system cannot simultaneously "Proceed," "Halt," and "Retry." It must select exactly one of these consequences to move forward. Selection is the process of applying a deterministic **Selection Rule** to the active field.

## The Semantic Gap

Activation answers: **What is currently admitted?**
Selection answers: **What is the resulting consequence?**

By separating these two phases, the Semantic Bit ensures that the "reason" for a decision (the activation) is always preserved and auditable, even as the "result" of the decision (the selection) is enforced. We do not lose the context of the warnings just because the final selection was `OK`.
