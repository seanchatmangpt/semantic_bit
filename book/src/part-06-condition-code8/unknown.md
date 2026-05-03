# UNKNOWN

The `UNKNOWN` condition is the fallback outcome when the selection rule cannot be applied, or when the active statuses are entirely missing or indecipherable. 

A deterministic system must still respond deterministically even when it lacks information. The `UNKNOWN` condition provides a bounded, safe way to represent this lack of knowledge. It is the explicit acknowledgment of ignorance, preventing the system from guessing or defaulting to an unsafe action.

## Example

If the data payload required to evaluate `Status8` is completely empty, the selection rule selects `UNKNOWN`, which is then typically routed to `ESCALATE` or safely terminated depending on the higher-level architecture.
