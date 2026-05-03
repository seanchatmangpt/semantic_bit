# Operation Authority

The ability to name an operation does not grant the right to execute it. Every one of the 64 Operation Cells must be guarded by **Operation Authority**.

Operation Authority is not a loose permission string like `"admin=true"`. It is a cryptographic requirement that binds the Subject Identity of the Declared Operation to the Noun/Verb cell being requested.

The authority check answers the question: *Is this Subject permitted to perform this Verb upon this Noun?*

Authority can be derived from:
- **Prior Receipts**: A receipt from an earlier step proving identity verification.
- **Static Relations**: A pre-compiled rule defining which roles can access which cells.
- **Dynamic State**: The current status of the Target Identity (e.g., you cannot `ACTIVATE FIELD` on a record that is already marked `BLOCKED`).

If the Operation Authority check fails, the operation is immediately categorized as a Forbidden Operation. The exact rule that triggered the refusal is captured in the subsequent receipt, ensuring that all access control decisions are fully transparent and auditable.
