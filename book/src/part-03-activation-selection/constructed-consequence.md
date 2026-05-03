# Constructed Consequence

In the Semantic Bit philosophy, a "consequence" is never an accident of control flow. It is a **Constructed Consequence**.

## Selection as Construction

When the deterministic engine evaluates a field, it does not simply "if-else" its way to a result. It uses the active bits to *construct* a new, singular meaning. 

This construction is governed by **Selection Rules** (or Field Laws). These rules define the priority and precedence of active bits. For example:
1. If `ERROR` is active, the constructed consequence is `ABEND` (Abnormal End).
2. If `BLOCKED` is active, the constructed consequence is `WAIT`.
3. If only `OK` is active, the constructed consequence is `PROCEED`.

## Immunity to Ambiguity

A constructed consequence is immune to ambiguity because the selection rules are exhaustive. There is no "unhandled" state. If the field carries a combination of bits that the rules do not explicitly permit, the system constructs a failure consequence (e.g., `UNKNOWN` or `INVALID_FIELD`).

## The Receipt of Consequence

Every constructed consequence must be accompanied by a **Selection Receipt**. This receipt preserves:
- The raw activation state (the "Why").
- The selection rule applied (the "How").
- The resulting condition code (the "What").

By treating consequences as constructed artifacts rather than transient branch points, we ensure that every motion the system takes is backed by a verifiable, admitted meaning.
