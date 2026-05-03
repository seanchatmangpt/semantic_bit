# Forbidden Operation

If a Declared Operation fails its authority check, or if its input contract is violated, or if the Noun/Verb combination is explicitly invalid for the given domain, the operation becomes a **Forbidden Operation**.

A Forbidden Operation does not cause a crash. It does not throw an unbounded exception that bubbles up the call stack. 

Instead, a Forbidden Operation immediately yields an `ABEND` (Abnormal End) or `REFUSE` condition code, depending on the severity of the violation.

Crucially, the system still emits a receipt for the Forbidden Operation. The receipt records the 6-bit operation code that was attempted, the identities involved, and the specific authority or contract rule that forbade it. 

This ensures that adversarial attempts to probe the system's operational boundaries are explicitly recorded in the journal. A crash is a silent failure; a Forbidden Operation receipt is a loud, cryptographic alarm.
