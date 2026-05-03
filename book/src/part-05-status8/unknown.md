# UNKNOWN

The **UNKNOWN** status is the assertion of semantic failure. The system holds the bytes, but cannot determine their truth.

This is the most dangerous state for data. An `UNKNOWN` field cannot be trusted, verified, or safely operated upon. It represents a collapse of the semantic contract.

When a field is `UNKNOWN`, the only valid operation is to reject it or pass it to an error-handling routine. The deterministic engine must not attempt to guess meaning from an unknown status.
