# Chapter 9 — Control Records

A decision without a messenger is a thought without a voice.

In the previous chapters, we followed the chain from an access attempt to a receipt. We captured what happened. But a system must also **command** what happens next. This is the role of the **Control Record**.

---

## 9.1 From Verdict to Action

Once a selection rule chooses a **Condition Code** (Chapter 7), the system must transmit that verdict to the component that can execute it.

A control record is a fixed-width instruction that carries a command to an actuator. It binds the verdict to its target and its authority.

```rust
#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct ControlRecord {
    pub target_id: u32,
    pub instruction: u8,
    pub authority_sequence: u64,
    pub field: ControlField,
}
```

*   **Target** (`target_id`): Which component should act? (e.g., Door 17).
*   **Instruction** (`instruction`): What should it do? (e.g., `AccessCondition::Grant`).
*   **Authority** (`authority_sequence`): What evidence allows this? (e.g., Receipt 4100).
*   **Control Field** (`field`): How should it be executed?

$$
\boxed{\textbf{The control record is the bridge from selection to motion.}}
$$

---

## 9.2 The Control Field

Just as the evidence is carried in a field, the **operational parameters** of the command are carried in a field.

A `ControlField` might admit meanings like:
*   `EXECUTE`: Act now.
*   `RETRY`: If the lock is jammed, try again.
*   `ACK_REQUIRED`: Report back once the door is physically open.

```rust
use semantic_bit::control::ControlField;

let field = ControlField::empty()
    .with(ControlField::EXECUTE)
    .with(ControlField::ACK_REQUIRED);
```

By using a field, we can multiplex the behavior of the command without changing the record structure.

---

## 9.3 Instruction as Data

In many systems, commands are sent as "messages" or "function calls." In the semantic discipline, a command is **data**.

Because the `ControlRecord` is fixed-width and has a stable layout (`repr(C)`), it can be written directly to a memory-mapped register on a hardware controller, or sent over a wire as a raw packet.

The machine at the other end doesn't need to "parse" the command; it only needs to observe the bits.

---

## 9.4 The Command Chain

The complete semantic chain now looks like this:

1.  **Attempt**: Input evidence.
2.  **Field**: Multi-meaning evidence.
3.  **Selection**: Singular verdict.
4.  **Receipt**: Preserved consequence.
5.  **Control Record**: Authorized instruction.

$$
\boxed{Receipt \rightarrow ControlRecord \rightarrow Action}
$$

The receipt provides the authority. The control record carries the instruction. The action completes the motion.

---

## 9.5 Example: Commanding the Door

```rust
use semantic_bit::access::AccessCondition;
use semantic_bit::control::{ControlField, ControlRecord};

// We take the verdict from our access selection
let verdict = AccessCondition::Grant;
let sequence = 4100;

// we construct the control record for the target door
let field = ControlField::empty().with(ControlField::EXECUTE);
let command = ControlRecord::new(17, verdict as u8, sequence, field);

assert_eq!(command.target_id, 17);
assert_eq!(command.instruction, AccessCondition::Grant as u8);
```

---

## 9.6 The Law in This Chapter

```text
A control record is an authorized instruction.
Control records use fixed-width representation.
A control record binds a target to a verdict.
Control fields multiplex operational parameters.
Commands are data, not prose.
The authority sequence links the command to the receipt.
```

The law chooses the verdict.
The record carries the command.
The machine performs the motion.

That is the discipline of control records.
