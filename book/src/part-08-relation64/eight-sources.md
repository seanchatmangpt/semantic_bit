# Eight Sources

A relation is a directed edge: it points from a Source to a Target.

In an unbounded graph, any node can point to any other node. In the Semantic8 Spine, the starting point of any relation must be classified into one of **Eight Sources**.

These sources categorize the *origin* of the relationship. A canonical set of Eight Sources aligns closely with the Eight Nouns of Operation64, but they are defined by their structural role in the graph:

1. **SUBJECT**: An active entity or identity initiating a connection (e.g., a user, a service).
2. **OBJECT**: A passive data structure or payload (e.g., a record, a document).
3. **EVENT**: A discrete occurrence in time (e.g., an operation receipt, a system halt).
4. **RULE**: A static definition or policy (e.g., a selection rule, a contract).
5. **LOCUS**: A boundary or container (e.g., a channel, a journal, a physical door).
6. **CLAIM**: An unverified assertion waiting for evaluation (e.g., a signature, a badge scan).
7. **PROOF**: Cryptographic evidence of a prior state (e.g., a receipt chain).
8. **STATE**: A persistent, named condition (e.g., a status field).

Before a relationship can be asserted, the origin node must be classified. If a system tries to assert that an unclassified string "depends on" a rule, the assertion is refused. The Source must be known.
