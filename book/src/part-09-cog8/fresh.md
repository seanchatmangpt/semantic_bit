# FRESH

The **FRESH** bit asserts that the data and dependencies the operation relies on have not exceeded their allowed temporal or structural bounds.

A receipt might be valid (`EVIDENCED`), but if it is older than the operation's freshness constraint, it is stale.

The `FRESH` bit guarantees that the system is acting on the current state of the world as defined by its boundaries, not a ghost of the past.
