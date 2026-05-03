# WARN

The `WARN` condition indicates that the operation is permitted to proceed, but the selection rule detected irregularities or secondary statuses that require noting.

A `WARN` condition does not halt the system. Instead, it alters the generation of the subsequent receipt and may trigger out-of-band monitoring or alerts. It is the system's way of saying "I am proceeding, but under degraded or suspicious confidence."

## Example

If `Status8` is both `OK` and `STALE`, the selection rule might determine that proceeding with stale data is permitted for this specific operation, but it selects the `WARN` condition to ensure the staleness is recorded as a factor in the outcome.
