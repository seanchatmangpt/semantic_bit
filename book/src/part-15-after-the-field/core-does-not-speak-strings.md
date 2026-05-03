# The Core Still Does Not Speak Strings

As we move from fields to triples, a dangerous temptation arises: the temptation to use strings as operational logic.

Because a triple (Subject, Predicate, Object) is often rendered in text, it is easy to forget the fundamental law established in Book 1: **The core does not speak strings.**

---

## 15.1 Strings are for Humans

A string (e.g., `"Door_17"` or `"AccessGranted"`) is a blob of prose. It is designed for human explanation, not machine distinction.

When you use a string in a decision path, you are asking the machine to **guess from explanation**. You are relying on the hope that every part of the system spells the string correctly, uses the same case, and shares the same cultural context of what that string "means."

In the Semantic Bit philosophy:

- A string is a **Label**.
- An identifier is a **Distinction**.

24587
\boxed{\textbf{The machine makes distinctions. The human reads labels.}}
24587

---

## 15.2 Identities are Admitted

When we use triples in the graph, we use **Identities** (often IRIs or numeric IDs), not raw text.

In the Rust library, this is reflected in the use of strong types and numeric identifiers (`u64`, `u32`). We do not pass `"Door_17"` to the `AccessAttempt`; we pass the admitted identifier `17`.

Even in the graph, the "string" part of a triple is merely a serialized form of a stable, admitted identity.

| Layer | Form | Character |
| :--- | :--- | :--- |
| **Explanation** | `"Badge Reader North Gate"` | Unstructured Prose (Ignored by Core) |
| **Relation** | `urn:sb:door:17` | Admitted Identity (Stable Graph Key) |
| **Execution** | `17u32` | Machine Distinction (Optimized Engine) |

---

## 15.3 The Boundary of Interpretation

The "Meaning Before Motion" philosophy requires a strict boundary where strings are converted into identities.

1. **At the Input Boundary**: A human types a name or a device sends a serial number.
2. **Admission**: The system looks up the admitted identity associated with that input. If no identity is found, the input is rejected.
3. **The Core**: The decision logic only ever sees the identity. It never sees the raw string.
4. **The Output Boundary**: When a receipt is displayed, the system looks up the human-readable label for the identity.

---

## 15.4 Detection of String Corruption

If you find yourself writing code that looks like this:

```rust
if door_name == "North_Gate" { ... }
```

You have violated the discipline of the field. You have introduced "prose-based truth." 

The correct form is to use an admitted constant or a lookup against an identity:

```rust
if door_id == DOORS::NORTH_GATE { ... }
```

By banning strings from the decision core, we eliminate an entire class of "interpretation bugs." The triple is a carrier of relation, but the relation is between identities, not between stories.
