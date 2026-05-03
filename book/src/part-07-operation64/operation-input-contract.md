# Operation Input Contract

Even if a subject has the *authority* to perform an operation, the operation cannot proceed if the inputs are malformed. This is enforced by the **Operation Input Contract**.

Each of the 64 Operation Cells defines a strict contract for the data it requires. Because our system eschews unbounded strings and complex nested objects, these contracts are usually very simple: a 64-bit identity, an 8-bit field, or a fixed-width record.

The Input Contract check ensures:
1. **Type Adherence**: Are the inputs exactly the expected numeric or bitwise types?
2. **Boundary Adherence**: Do the values fall within the admitted ranges?
3. **Evidence Adherence**: Is the necessary prior receipt attached and cryptographically valid?

If the Input Contract is violated, the operation is Forbidden. We do not attempt to coerce the data, parse around errors, or guess the user's intent. The system refuses the operation, emits a receipt citing a contract violation, and halts the channel.
