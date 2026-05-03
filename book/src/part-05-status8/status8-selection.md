# Status8 Selection

**Status8 Selection** is the process by which the deterministic engine evaluates the `Status8` field and decides the next legal action.

Because there are exactly eight admitted statuses, the selection logic can be exhaustively mapped. The system does not need complex heuristics; it requires a simple dispatch table.

For every status, there is a defined branch. This ensures that every field, regardless of its truth state, is handled systematically and without ambiguity. The Status8 byte is the switch upon which the machine routes its execution.
