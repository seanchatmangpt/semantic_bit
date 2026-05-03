# One Carrier, Many Meanings

The core of the Semantic8 architecture is the **Semantic Byte**: a single 8-bit carrier that serves as the host for multiple distinct meanings.

## The Field as a Host

In the Semantic Bit philosophy, we do not view a byte as a number. We view it as a **Field**. A field is a named memory location with a fixed width, where specific positions have been "named" and assigned a formal semantic definition.

Because the carrier is a single byte, it is atomically transportable and evaluatable. However, because it carries many meanings, it is expressive enough to govern complex operational logic.

## Semantic Independence

The critical rule of multiplexing is **positional independence**. The meaning of Bit 3 (`AUTHORIZED`) must not depend on the state of Bit 0 (`INITIALIZED`). 

While the *selection rules* (discussed in Part III) will eventually resolve these many meanings into a single consequence, the carrier itself remains a pure record of simultaneous truths. This allows for:
- **Auditability**: We can see exactly which conditions were active at the moment of a decision.
- **Traceability**: We can trace the source of each individual bit back to the evidence that admitted it.
- **Deterministic Selection**: We can apply fixed rules to the multiplexed field to ensure that "Meaning Always Precedes Motion."

A single carrier, holding many meanings, ensures that the system never has to "guess" which conditions were true; it simply reads the field.
