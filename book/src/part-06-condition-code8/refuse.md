# REFUSE

The `REFUSE` condition is a deterministic denial of service. The selection rule has evaluated the active statuses and found them to be in violation of the required operational boundaries.

A refusal is not a failure of the system; it is the system working correctly to prevent an unauthorized or unsafe operation.

## Example

If `Status8` indicates the badge is present but the location is forbidden, the selection rule will prioritize the forbidden status and select the `REFUSE` condition, terminating the operation safely.
