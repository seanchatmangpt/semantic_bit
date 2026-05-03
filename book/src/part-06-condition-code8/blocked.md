# BLOCKED

The `BLOCKED` condition indicates that the operation cannot proceed immediately due to an unresolved dependency or unfulfilled requirement.

Unlike a failure, a `BLOCKED` condition is an expected pause in execution. The system expects that the blocking factor will be resolved asynchronously, after which the operation can be re-evaluated.

## Example

If `Status8` is `BLOCKED` because a secondary system has not yet provided a required receipt, the selection rule will select the `BLOCKED` condition, pausing the current operation pipeline without generating an error.
