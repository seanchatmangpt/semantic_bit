# COG8 Receipt

The evaluation of the `COG8` field is a critical moment. It represents the machine's "thought process" immediately prior to action or refusal.

Therefore, the `COG8` field state MUST be recorded in a receipt.

If an operation fails authorization, the receipt will show the `AUTHORIZED` bit was 0. If it fails freshness, the receipt will show `FRESH` was 0.

This means you never have to guess *why* a machine made a decision. The exact cognitive state is permanently bound in the receipt.
