# The Eight COG8 Meanings

The `COG8` field multiplexes eight distinct cognitive checks. These are the fundamental questions the system must answer "yes" to before allowing a consequential operation.

The eight meanings are:
1.  **CLOSED**: Is the operation's scope fully bounded?
2.  **EVIDENCED**: Do we have the cryptographic or historical receipts to prove prior claims?
3.  **AUTHORIZED**: Does the actor have the required relation to the target?
4.  **FRESH**: Are the dependencies and inputs current, rather than stale?
5.  **CONSISTENT**: Does the operation maintain the invariants of the system?
6.  **LOCAL_ACTIONABLE**: Can the operation be executed by this specific node right now?
7.  **PROJECT_REQUIRED**: Does the operation align with the broader goals of the overarching project?
8.  **REPLAYABLE**: If the operation fails, or needs to be audited, can it be deterministically re-executed?

Each bit represents an independent cognitive assertion.
