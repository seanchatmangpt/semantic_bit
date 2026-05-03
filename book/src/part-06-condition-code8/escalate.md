# ESCALATE

The `ESCALATE` condition is selected when the system encounters a state that is valid but outside its authorized boundaries to resolve. The system recognizes the state, but it lacks the authority or ruleset to determine the next step autonomously.

Escalation is a delegation of responsibility. It halts local processing and transfers the operational context to a higher authority—whether that is a human operator or a superior control system.

## Example

If `Status8` shows conflicting priorities that the local selection rule cannot break ties between (e.g., both `WARN` and `BLOCKED` with no precedence defined), it selects `ESCALATE` to force an authoritative resolution.
