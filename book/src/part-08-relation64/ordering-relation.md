# Ordering Relation

An **Ordering Relation** enforces temporal sequence without requiring a direct functional dependency. It asserts that "Target must happen before Source."

This is heavily used in Journals and Channels. When receipts are appended to a journal, they form an Ordering Relation with the previous receipt. This creates a cryptographic chain.

While a Dependency Relation implies that data or state is passed from Target to Source, an Ordering Relation merely asserts sequence. The system uses Ordering Relations to ensure that replays happen in the exact historical sequence, preserving the integrity of the semantic timeline. If an attempt is made to evaluate a record out of sequence, the Ordering Relation fails, yielding a `STALE` status or an `ESCALATE` condition.
