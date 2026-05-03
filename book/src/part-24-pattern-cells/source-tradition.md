# Source Tradition

No pattern is created in a vacuum. Every semantic bit has a source tradition.

The source tradition is the historical context, legacy system, or domain expertise from which a pattern is derived. We acknowledge the tradition not to replicate its flaws, but to understand the constraints that shaped the admitted meanings.

$$
\boxed{\textbf{Tradition provides the context. The field provides the law.}}
$$

## 24.1 Respecting the Legacy

A legacy system might use a complex set of "Reason Codes" (e.g., `ERR_042`, `STATUS_PENDING_APPROVAL`). These are part of the source tradition.

We do not import these codes directly into the semantic field as raw strings. Instead, we analyze the tradition to extract the underlying conditions. If `ERR_042` means "Badge expired" and `STATUS_PENDING_APPROVAL` means "Manager review needed", we map these to the `HOLDER_ACTIVE` (negative) and `REVIEW_REQUIRED` (positive) bits.

## 24.2 The Tradition of the Physical

Some traditions are physical. The layout of a hardware register is a source tradition. The timing constraints of a network protocol are a source tradition.

When we model a hardware device in the Semantic Bit, we honor the physical tradition by ensuring our semantic fields align with the bit-width and alignment of the hardware. This allows for direct memory mapping and zero-cost abstraction.

## 24.3 Historical Form

The historical form of a pattern may be documented in RFCs, whitepapers, or institutional memory. This form provides the "Original Constraint".

-   **RFC 7231** (HTTP Status Codes) is a source tradition for `ConditionCode8`.
-   **POSIX** error codes are a source tradition for `Status8`.

We do not blindly follow these forms. We filter them through the Laws of the Field. If a tradition requires "guessing from explanation," we reject that part of the form.

## 24.4 Evolution from Tradition

As a system matures, the semantic field may diverge from its source tradition. This is healthy. The tradition served as the scaffolding for the initial admission. Once the meaning is bounded in the field, the system is governed by the field's own internal consistency and receipt-driven logic.

$$
\boxed{\textbf{We build upon tradition to move beyond prose.}}
$$
