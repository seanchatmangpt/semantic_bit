# Decomposition Before Widening

Widening a bitfield is an admission of unstructured growth.
Instead of widening an 8-bit semantic byte to 16, 32, or 64 bits, we decompose.

Decomposition before widening forces the engineer to identify the hidden relationships between the flags.
Often, the need for more flags indicates that a single semantic byte is improperly multiplexing two distinct concerns. By separating these concerns into two distinct bytes, the semantic clarity is restored.