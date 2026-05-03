# Bits Carry State, Triples Carry Relation

A common mistake in semantic engineering is to use the wrong carrier for the wrong purpose. 

A bit is a high-speed, high-density carrier of **State**. A triple is a high-context, high-precision carrier of **Relation**. The Semantic Bit architecture requires both, used in their proper domains.

---

## 15.1 The Domain of the Bit

The bit (organized into fields) is the language of the **Engine**.

When a badge is presented at a door, the system must make a decision in milliseconds. It must check positions, apply selection rules, and emit a receipt. For this, we use the Semantic Field:

- **Compact**: 1 byte carries 8 meanings.
- **Fast**: Bitwise AND/OR/XOR are the fastest operations in the CPU.
- **Fixed**: The memory layout is predictable and `#[repr(C)]` compatible.

The bit tells us **What is happening now**.

---

## 15.2 The Domain of the Triple

The triple (organized into graphs) is the language of the **Blueprint**.

The system must know *which* rule to apply to the door. It must know *who* is authorized to change the rule. It must know *where* the logs should be sent. For this, we use the Triple:

- **Expansive**: It can describe any connection between any two identities.
- **Contextual**: It provides the "Why" and "How" behind the "What".
- **Evolvable**: New relations can be added to the graph without changing the binary layout of the engine.

The triple tells us **How the system is structured**.

---

## 15.3 The Interface of Manufacture

The power of the Semantic Bit philosophy emerges when the Triple is used to **Manufacture** the Bit.

We do not manually code every field and every selection rule in Rust. Instead, we describe the field's meaning in a graph of triples, and then we use a tool (`unrdf`) to generate the Rust code.

24274
\boxed{\textbf{The Triple is the Blueprint. The Bit is the Engine.}}
24274

| Attribute | The Semantic Field (Bit) | The Semantic Graph (Triple) |
| :--- | :--- | :--- |
| **Primary Use** | Execution / Operational Motion | Definition / Manufacturing Law |
| **Storage** | RAM / Registers / Binary Log | RDF Store / Source Files |
| **Complexity** | O(1) - Constant Time | O(log N) - Search/Inquiry |
| **Visibility** | Machine-only (Numeric) | Human & Machine (Identified) |

---

## 15.4 Combined Discipline

The system uses the triple to define the field's boundaries.

1. **At Design Time**: We write triples that say "The Access Field has a position called `BADGE_PRESENT` at bit 0."
2. **At Manufacture Time**: We generate a Rust `struct AccessField(u8)` with the corresponding constants.
3. **At Runtime**: The engine uses the efficient bitwise field to process millions of badge presentations.
4. **At Audit Time**: We join the binary receipt (carrying the raw bit) back to the triple (carrying the meaning) to provide a human-readable explanation of why the door opened.

We do not sacrifice speed for meaning. We use the triple to ensure that the speed of the machine is always directed by an admitted purpose.
