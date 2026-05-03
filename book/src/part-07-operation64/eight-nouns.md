# Eight Nouns

Action must act *upon* something. In an unbounded system, a function can act upon any object, variable, or external resource. This makes the system impossible to audit and impossible to fully secure, as the boundary of action is constantly shifting.

In Operation64, the targets of action are restricted to exactly **Eight Nouns**. These nouns define the fundamental semantic entities within the local closed world of the system.

While the specific nouns may vary depending on the implementation domain, a canonical Semantic8 noun set might look like this:

1. **FIELD**: A raw bitwise carrier of multiplexed state.
2. **RECORD**: A bounded structure containing identities, timestamps, and fields.
3. **RECEIPT**: An immutable cryptographic proof of a prior selection or consequence.
4. **CHANNEL**: A pathway for transporting records across boundaries.
5. **JOURNAL**: An append-only ledger of ordered receipts.
6. **AUTHORITY**: The cryptographic identity or rule granting permission.
7. **RELATION**: The dependency or linkage between two nouns.
8. **CONSTRUCT**: A fully realized semantic conclusion, ready for consequence.

By limiting the targets of action to these eight nouns, we ensure that the system can never act upon an unknown entity. Every action is categorized before it begins. If a process attempts to interact with something that is not one of the Eight Nouns, that attempt is a category failure and is immediately refused.
