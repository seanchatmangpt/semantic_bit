# Multiplexing

Multiplexing in the Semantic8 architecture refers to the capability of carrying multiple independent assertions on a single 8-bit carrier. Rather than representing a single scalar value (like an integer from 0 to 255), the byte acts as a conduit for multiple parallel semantic pathways.

## Parallel Assertions

In a traditional system, a status might be represented by a single enumeration:
- 0: IDLE
- 1: RUNNING
- 2: ERROR

However, this scalar representation cannot easily capture simultaneous conditions without an explosion of states. If an operation is both `RUNNING` and has a `WARNING` flag, a scalar system needs a new state (e.g., 3: RUNNING_WITH_WARNING).

Multiplexing solves this by assigning each bit position a dedicated, non-overlapping semantic meaning. A single byte can simultaneously assert that an operation is:
1. `INITIALIZED` (Bit 0)
2. `ACTIVE` (Bit 1)
3. `RECORDED` (Bit 5)

## The Efficiency of the Byte

By treating the byte as a field of 8 independent bits, we maximize the information density of the carrier. We do not "calculate" the value of the byte; we "admit" conditions into it. This allows the deterministic engine to evaluate the entire state of a subsystem in a single CPU instruction (bitwise AND/OR).

Multiplexing is the foundation of **Meaning Before Motion**. Before any operational step is taken, the field is multiplexed with all known conditions, providing a high-fidelity snapshot of the environment's state.
