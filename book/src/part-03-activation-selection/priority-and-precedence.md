# Priority and Precedence

The most common selection rule is **priority-based precedence**. The bit positions within the field are ordered by severity or authority. When multiple bits are active, the rule selects the bit with the highest precedence.

For example, in a status field, if both `ABEND` (abnormal end) and `WARN` are active, `ABEND` has higher precedence. The selected condition is `ABEND`, and the `WARN` state is subordinate.
