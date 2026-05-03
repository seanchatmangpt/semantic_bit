# Blocking Relation

A **Blocking Relation** is the inverse of a Dependency Relation. Instead of saying "Source requires Target to proceed," a Blocking Relation says "Source cannot proceed because Target exists."

This is often used for conflict resolution and mutual exclusion. If an `EVENT` has a Blocking Relation pointing to a `STATE` (e.g., a maintenance mode flag), the event cannot be selected.

Blocking Relations are critical for maintaining the Multiplexing Law (Part II). When multiple meanings are active simultaneously, a Blocking Relation ensures that a highly privileged status (like an administrative lockdown) explicitly prevents the selection of lower-privileged operations, rather than relying on complex `if/else` logic scattered throughout the code.

When a Blocking Relation is triggered, the operation yields a `BLOCKED` ConditionCode8, and a receipt is generated citing the specific Relation64 cell that halted the motion.
