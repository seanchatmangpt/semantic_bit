# REPLAYABLE

The **REPLAYABLE** bit asserts that the operation, if executed, will generate a receipt that contains enough information to perfectly reconstruct the operation later.

This forces operations to be deterministic. If an operation relies on true randomness or hidden state, it cannot guarantee replayability. By requiring the `REPLAYABLE` bit to be active for critical operations, the system enforces determinism by design.
