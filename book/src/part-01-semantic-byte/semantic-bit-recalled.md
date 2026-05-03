# The Semantic Bit Recalled

Before we widen the field to eight bits, we must recall the discipline of the single semantic bit.

In Book 1, we learned that a bit is not just a binary choice (true/false). In our system, a bit is a **named position** that carries an **admitted meaning**.

## The Three Disciplines of the Bit

1. **Meaning Over Representation:** We do not care if the bit is a transistor, a magnetic flip-flop, or a high-voltage pulse. We care that it represents `BADGE_PRESENT`.
2. **Admission Over Story:** We do not accept arbitrary descriptions of what might be happening. We accept only the activation of admitted meanings.
3. **Consequence Over Decoration:** The bit is not there to look good in a log. It is there to drive a selection rule.

## The Field Formula

We expressed the presence of a meaning using a simple field test:

$$
\text{carries}(Field, Position) \rightarrow \{Present, Absent\}
$$

When the position is present, the meaning is active.

## Why One Bit Was Not Enough

The single bit served us well for the simplest decisions (Open/Close). But even the access badge (Chapter 1) required eight bits to carry a complete decision state.

We found that while one bit carries one meaning, a **field** of bits carries a **situation**.

$$
\boxed{\textbf{A situation is the set of all active meanings carried by a field at a moment of inquiry.}}
$$

In Book 2, we formalize the carrier of these situations. We move from the custom-named field to the universal **Semantic Byte**.
