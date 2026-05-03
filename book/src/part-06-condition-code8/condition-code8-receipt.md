# ConditionCode8 Receipt

Just as a receipt must be generated when Statuses are evaluated, a distinct receipt must be generated when a Selection Rule resolves those Statuses into a Condition.

The **Condition Receipt** is the unforgeable proof of the selection process. It must capture:

1. The input: The raw `Status8` field as it existed at the moment of evaluation.
2. The mechanism: The specific Selection Rule (or ID) that was applied.
3. The outcome: The resulting `ConditionCode8`.

This receipt bridges the gap between "what the system observed" (Status) and "what the system decided to do" (Condition), providing full traceability for audits and debugging. Without this receipt, an observer cannot prove that a condition was derived lawfully from the preceding status.
