# EVIDENCED

The **EVIDENCED** bit asserts that every claim made by the operation's inputs is backed by a verifiable receipt.

If the operation requires `Status8::OK` from a previous step, it is not enough that a flag says it was OK. The system must possess the `Receipt` of that previous step.

When the `EVIDENCED` bit is active, the system has successfully traversed the receipt chain and verified the cryptographic or structural signatures of all prerequisites.
