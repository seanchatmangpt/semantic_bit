# COG8 in Rust

The **COG8** field represents the cognitive status of an admitted fact. In Rust, it is implemented as a specialized `SemanticByte` that bounds the evidence and authority of a decision.

---

## 12.16 The COG8 Field

`COG8` is used by the system to "think" about its own observations. It doesn't store the observation itself (that's `Status8`), but rather the *quality* and *provenance* of that observation.

```rust
pub struct Cog8(SemanticByte);

impl Cog8 {
    pub const CLOSED: u8           = 1 << 0;
    pub const EVIDENCED: u8        = 1 << 1;
    pub const AUTHORIZED: u8       = 1 << 2;
    pub const FRESH: u8            = 1 << 3;
    pub const CONSISTENT: u8       = 1 << 4;
    pub const LOCAL_ACTIONABLE: u8 = 1 << 5;
    pub const PROJECT_REQUIRED: u8 = 1 << 6;
    pub const REPLAYABLE: u8       = 1 << 7;

    pub const fn empty() -> Self {
        Self(SemanticByte::empty())
    }
}
```

---

## 12.17 Reasoning in Rust

The power of `COG8` comes from selection rules that examine several bits to determine a cognitive state. For example, a fact is only `TRUSTED` if it is both `EVIDENCED` and `AUTHORIZED`.

```rust
impl Cog8 {
    /// A fact is trusted if it is both evidenced and authorized.
    pub const fn is_trusted(&self) -> bool {
        self.0.carries(Self::EVIDENCED) && self.0.carries(Self::AUTHORIZED)
    }

    /// A fact is actionable if it is trusted, fresh, and consistent.
    pub const fn is_actionable(&self) -> bool {
        self.is_trusted() && 
        self.0.carries(Self::FRESH) && 
        self.0.carries(Self::CONSISTENT)
    }
}
```

---

## 12.18 Cognitive Integrity

By using `Cog8` in Rust, we ensure that cognitive state is:
1.  **Explicit**: You cannot "accidentally" trust a piece of data.
2.  **Bounded**: There are only eight cognitive dimensions considered.
3.  **Verifiable**: The logic for trusting or acting on a fact is concentrated in simple bitwise tests.

$$
\boxed{\textbf{Cog8 is the system's internal proof of its own reasoning.}}
$$
