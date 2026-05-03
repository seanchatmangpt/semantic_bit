# The Field Boundary

The field boundary of a Semantic Byte is rigid.
It operates as a strict containment mechanism. Any logic attempting to overflow the byte or reinterpret its bits outside their named definitions violates the field boundary.

This boundary is essential for reliable dispatch and checkpointing. A system must know that the eight bits it reads at recovery time mean exactly the same thing they meant at write time.