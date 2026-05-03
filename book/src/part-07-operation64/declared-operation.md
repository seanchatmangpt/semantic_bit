# Declared Operation

An operation cannot be spontaneous. It must be declared before it is executed.

A **Declared Operation** is the formal intent to execute one of the 64 Operation Cells. It is typically generated as the constructed consequence of a `ConditionCode8`.

When the system reaches a condition—say, `OK` on a badge scan—the selection rule doesn't just say "continue." It maps the `OK` condition to a Declared Operation, such as `ACTIVATE FIELD` (to activate the 'admission granted' bit on a door controller).

The declaration must contain:
1. **The 6-bit Operation Code**: Identifying the exact Noun/Verb cell.
2. **The Subject Identity**: *Who* is attempting the operation.
3. **The Target Identity**: *Which* specific instance of the noun is being acted upon.
4. **The Input Evidence**: The `ConditionCode8` receipt that triggered the declaration.

By declaring the operation before executing it, we create a momentary pause—a checkpoint—where the system can evaluate whether the operation is permitted, whether the dependencies are met, and whether the inputs are valid. The motion is bounded before it begins.
