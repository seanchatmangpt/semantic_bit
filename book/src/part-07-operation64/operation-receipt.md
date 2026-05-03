# Operation Receipt

The execution of a Selected Operation is the moment of consequence. Once the action is taken—once a field is activated, a record deposited, or a relation bound—that motion must be permanently recorded.

The **Operation Receipt** is the immutable proof that the operation occurred.

It contains:
- The Timestamp of the operation.
- The 6-bit Operation Code (Verb + Noun).
- The Subject and Target Identities.
- The `ConditionCode8` receipt that authorized the operation.
- The resulting state or condition produced by the operation.
- A cryptographic hash binding all of these elements together.

The Operation Receipt is the output of the operation cell. It is appended to the journal, providing an unbreakable chain of evidence showing exactly how the system moved from one state to the next.

If an operation fails, a Forbidden Operation Receipt is generated instead. In either case, the motion is bounded, the outcome is deterministic, and the evidence is permanent.
