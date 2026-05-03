# The Eight Status Meanings

The multiplexing law allows for eight admitted meanings within a semantic byte. For **Status8**, these meanings are defined as follows:

1. **OK**: The field is valid and ready.
2. **WARN**: The field is valid, but an anomaly was recorded.
3. **BLOCKED**: The field cannot be processed due to external dependencies.
4. **UNKNOWN**: The status of the field cannot be determined.
5. **SKIPPED**: The field was intentionally ignored.
6. **STALE**: The field's meaning is no longer valid.
7. **RECEIPTED**: The field's meaning has been proven and recorded.
8. **REPLAYABLE**: The field's state can be deterministically reproduced.

These eight meanings form an exhaustive vocabulary for the lifecycle of data within a semantic system. No other status can be admitted.
