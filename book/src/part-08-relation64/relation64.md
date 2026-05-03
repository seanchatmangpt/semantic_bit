# Relation64

We have seen how state is bounded by `Status8` and `ConditionCode8`, and how motion is bounded by `Operation64`. But no semantic entity exists in isolation. A record depends on a channel; an operation requires authority; an admission depends on a badge. 

To make decisions, the system must navigate the connections between entities. If we allow unstructured pointers or unbounded graph queries, we reintroduce the chaos of infinite possibility.

This is the domain of **Relation64**.

Relation64 is the fourth vertebra of the Semantic8 Spine. It answers the question: *How are these two entities connected?*

Just as Operation64 restricted action to an 8x8 grid of Nouns and Verbs, Relation64 restricts connection to an 8x8 grid of **Sources** and **Targets**. By bounding relations in this way, we guarantee that the system can always traverse and evaluate dependencies deterministically.

In this part, we will explore:
1. The eight Source types and eight Target types.
2. The 64 Relation Cells that result from their combination.
3. The specific semantic relationships like Dependency, Authority, and Compatibility.
4. How closure is calculated over bounded relations.

Through Relation64, the graph of connected knowledge becomes just as rigid, predictable, and measurable as the bitfield.
