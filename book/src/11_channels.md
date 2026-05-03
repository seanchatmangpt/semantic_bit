# Chapter 11 — Channels

A record in motion is a record that depends on a **Channel**.

A channel is the bounded logical path between two components. In the semantic discipline, we do not treat a connection as an abstract "socket" or "pipe." We treat it as a bounded resource governed by a **Channel Field**.

---

## 11.1 Transport is Operational

In conventional programming, we often assume the network is "there" until it isn't. In the semantic bit discipline, the state of the transport is just another set of admitted conditions.

A `ChannelField` multiplexes the state of a communication path:

```rust
#[repr(transparent)]
pub struct ChannelField(u8);
```

By observing the `ChannelField`, the system can determine if a channel is `CONNECTED`, `READY` for data, `BUSY` with another transmission, or in an `ERROR` state.

---

## 11.2 The Guarded Path

Before a `ControlRecord` (Chapter 9) can be sent using a `DispatchField` (Chapter 10), the system must observe that the channel is `READY`.

```rust
use semantic_bit::channel::{ChannelField, Presence};

fn try_dispatch(channel: ChannelField) {
    if channel.carries(ChannelField::READY) == Presence::Present {
        // Safe to send
    } else {
        // Wait or retry
    }
}
```

This ensures that we never "trust the motion" of the transport layer without first "bounding the meaning" of its state.

---

## 11.3 Multiplexed Channel State

Because the channel state is a field, it can carry multiple meanings at once. For example, a channel might be `CONNECTED` but also in an `ERROR` state (perhaps a protocol mismatch).

$$
\boxed{\textbf{The channel field is the evidence of transport integrity.}}
$$

By recording the `ChannelField` in our journals (Chapter 12), we can audit not just *what* we sent, but the *condition* of the path we sent it over.

---

## 11.4 Channels as Boundaries

A channel is a **semantic boundary**. When a record crosses a channel, it moves from one component's field of observation to another's.

The channel is responsible for preserving the integrity of the bits during this transition. It doesn't need to understand the record; it only needs to ensure that the `0`s and `1`s remain in their assigned positions.

---

## 11.5 Example: Observing the Channel

```rust
use semantic_bit::channel::{ChannelField, Presence};

// A healthy, ready channel
let status = ChannelField::empty()
    .with(ChannelField::CONNECTED)
    .with(ChannelField::READY);

assert_eq!(status.carries(ChannelField::READY), Presence::Present);
assert_eq!(status.carries(ChannelField::ERROR), Presence::Absent);
```

---

## 11.6 The Law in This Chapter

```text
A channel is a bounded transport path.
Channel state is governed by a semantic field.
Transport integrity is observed, not assumed.
Channels act as boundaries between components.
The channel field enables audit of transport conditions.
```

The law chooses the verdict.
The field selects the path.
The channel carries the truth.

That is the discipline of channels.
