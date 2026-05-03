# Closure Over Relations

A single relation rarely provides the full picture. A system must often traverse multiple links to reach a definitive conclusion. This process is called calculating **Closure Over Relations**.

Closure is the bounded traversal of the graph. Because every node is one of Eight Sources/Targets, and every edge is one of 64 Relation Cells, traversal is deterministic.

When calculating closure, the system follows a strict set of rules:
1. **Bounded Depth**: The system will only traverse a predefined number of links (e.g., maximum depth of 3) before halting to prevent infinite loops and unbounded compute time.
2. **Path Adherence**: The traversal must follow valid cell pathways. If the closure requires finding an Authority Relation, it will only traverse edges that map to that specific semantic cell.
3. **Accumulated Condition**: As the closure is calculated, the conditions of the traversed nodes are funneled through a selection rule. If any required node in the closure yields `ABEND` or `BLOCKED`, the entire closure fails.

Calculating closure transforms a complex web of scattered facts into a single, actionable `CONSTRUCT8` state. It is how the system moves from "I have these 10 disconnected facts" to "I am authorized to proceed."
