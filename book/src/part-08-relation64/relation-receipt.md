# Relation Receipt

Just as operations and conditions emit receipts, the assertion of a relationship must also be receipted.

The **Relation Receipt** provides cryptographic proof that a specific link between two nodes existed at a specific moment in time.

It contains:
- The Timestamp.
- The 6-bit Relation Code (Source + Target categories).
- The specific Identity of the Source node.
- The specific Identity of the Target node.
- The semantic type of the relation (Dependency, Authority, etc.).
- A cryptographic hash binding these elements.

Why receipt a relation? Because relations change. An Authority Relation that existed yesterday might be revoked today. If an operation was authorized yesterday, we must prove that the authorization was valid *at the exact moment of execution*. The Relation Receipt freezes the connection in time, ensuring that the historical record is perfectly replayable even after the active graph has evolved.
