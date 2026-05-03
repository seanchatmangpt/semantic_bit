# Semantic Bit Engineering Guidelines

## Core Philosophy: Meaning Before Motion

This repository implements the **Semantic Bit** philosophy. It strictly adheres to the principle that software must bound operational meaning before trusting any motion. We do not use prose, loosely coupled booleans, or unstructured strings to make operational decisions.

### The Three Laws of the Field
1. **The badge does not carry truth in prose. It carries admitted conditions.**
2. **The system does not guess from explanation. It selects from the field.**
3. **The receipt does not narrate after the fact. It preserves the field at the moment of consequence.**

## Rust Architectural Standards

When implementing or modifying code in this repository, you **MUST** adhere to the following rules:

### 1. Field Discipline
- **No loose booleans**: Operational state must be encapsulated in bounded fields (e.g., `AccessField`, `Status8`).
- **Bitwise Semantics**: Use fixed-width representations (e.g., `#[repr(transparent)] struct AccessField(u8);`). Position flags must be defined as bitwise constants using bitshifts (`1 << 0`, `1 << 1`, etc.).
- **Multiplexing**: "Activation may be many. Selection must be one." Fields multiplex several conditions; selection rules map them to a singular, non-overlapping state (`enum` like `AccessCondition`).

### 2. Operational Records
- **Fixed-width Records**: Use `#[repr(C)]` for operational records (e.g., `AccessAttempt`, `AccessReceipt`) to ensure memory alignment and strict boundaries.
- **No Strings in the Critical Path**: Identifiers should be numeric (e.g., `u64`, `u32`) or strong types, never raw `String` processing in the decision core.
- **Receipt Generation**: Any consequential action MUST emit a receipt that immutably captures the raw field state, the selected condition, and the identities involved.

### 3. Documentation & Verification
- **Module-Level Docs**: Every module must have comprehensive module-level documentation (`//!`) explaining its semantic bounds.
- **Doctests are Mandatory**: Every public function and struct implementation MUST have an executable doctest demonstrating its behavior. This is our proof of correctness.
- **Absence of Magic**: Do not use complex macros or implicit state to hide control flow. The mapping from active bit to selected condition must be explicit and verifiable.

## Adversarial Trust

Treat all input as adversarial. 
- Input does not provide "authorization" (e.g., a `GRANTED` state). Input provides raw signals (`BADGE_PRESENT`).
- Constructed meanings (like `GRANTED`) are only admitted into the field *after* passing a selection rule.

## Structure

- `src/`: The executable semantic laws (Access, Control, Dispatch, etc.)
- `book/`: The mdBook manuscript serving as the formal specification and curriculum. Changes to logic must be mirrored in the text.
