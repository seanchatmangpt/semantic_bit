# ABEND

The `ABEND` (Abnormal End) condition indicates a catastrophic or unrecoverable failure. The selection rule has detected a state that violates the fundamental invariants of the system, making safe continuation impossible.

When an `ABEND` is selected, the system immediately ceases operation for that context to prevent cascading damage or data corruption.

## Example

If `Status8` is read, but the field itself fails validation (e.g., bits are active that are not defined in the schema), the selection rule cannot safely interpret the state. It selects `ABEND` to halt processing and preserve system integrity.
