//! Compact canonical digest and effect identity (std-only SHA-256).
//!
//! `effect_id = H(principal, beneficiary, amount, asset, purpose, authority,
//! obligation, rail, expiry, reservation)`. Any change to a bound field changes
//! the id, so an authorization bound to one id never transfers to another effect.
//! This is an operational construct, not a claim of conformance to any standard
//! payment message format.
//!
//! ```
//! use semantic_bit::effect_identity::{EffectIdentity, verify_binding};
//! let e = EffectIdentity {
//!     principal: "p".into(), beneficiary: "b".into(), amount_minor: 100,
//!     asset: "USD".into(), purpose: "x".into(), authority: "a".into(),
//!     obligation: "o".into(), rail: "r".into(), expiry_unix: 1, reservation: "v".into(),
//! };
//! let id = e.effect_id();
//! assert!(verify_binding(&id, &e).is_ok());
//! ```

const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

/// SHA-256 of `data` (FIPS 180-4).
pub fn sha256(data: &[u8]) -> [u8; 32] {
    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
    ];
    let mut msg = data.to_vec();
    msg.push(0x80);
    while msg.len() % 64 != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&((data.len() as u64) * 8).to_be_bytes());
    for chunk in msg.chunks(64) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([chunk[4 * i], chunk[4 * i + 1], chunk[4 * i + 2], chunk[4 * i + 3]]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16].wrapping_add(s0).wrapping_add(w[i - 7]).wrapping_add(s1);
        }
        let mut v = h;
        for i in 0..64 {
            let s1 = v[4].rotate_right(6) ^ v[4].rotate_right(11) ^ v[4].rotate_right(25);
            let ch = (v[4] & v[5]) ^ (!v[4] & v[6]);
            let t1 = v[7].wrapping_add(s1).wrapping_add(ch).wrapping_add(K[i]).wrapping_add(w[i]);
            let s0 = v[0].rotate_right(2) ^ v[0].rotate_right(13) ^ v[0].rotate_right(22);
            let maj = (v[0] & v[1]) ^ (v[0] & v[2]) ^ (v[1] & v[2]);
            let t2 = s0.wrapping_add(maj);
            v = [t1.wrapping_add(t2), v[0], v[1], v[2], v[3].wrapping_add(t1), v[4], v[5], v[6]];
        }
        for i in 0..8 {
            h[i] = h[i].wrapping_add(v[i]);
        }
    }
    let mut out = [0u8; 32];
    for i in 0..8 {
        out[4 * i..4 * i + 4].copy_from_slice(&h[i].to_be_bytes());
    }
    out
}

/// Lowercase hex encoding.
pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

const DOMAIN: &[u8] = b"semantic_bit.effect_identity.v1\0";

fn put(buf: &mut Vec<u8>, field: &[u8]) {
    buf.extend_from_slice(&(field.len() as u64).to_be_bytes());
    buf.extend_from_slice(field);
}

/// Domain-separated digest of an already-canonical payload, as lowercase hex.
pub fn canonical_payload_digest(canonical_bytes: &[u8]) -> String {
    let mut b = b"semantic_bit.payload.v1\0".to_vec();
    b.extend_from_slice(canonical_bytes);
    hex(&sha256(&b))
}

/// The ten fields bound into an effect identity.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EffectIdentity {
    pub principal: String,
    pub beneficiary: String,
    /// Amount in minor units; no floating point in identity.
    pub amount_minor: u128,
    pub asset: String,
    pub purpose: String,
    pub authority: String,
    pub obligation: String,
    pub rail: String,
    pub expiry_unix: i64,
    pub reservation: String,
}

impl EffectIdentity {
    /// Canonical, injective encoding: fixed field order, u64 length prefixes.
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut b = DOMAIN.to_vec();
        put(&mut b, self.principal.as_bytes());
        put(&mut b, self.beneficiary.as_bytes());
        put(&mut b, self.amount_minor.to_string().as_bytes());
        put(&mut b, self.asset.as_bytes());
        put(&mut b, self.purpose.as_bytes());
        put(&mut b, self.authority.as_bytes());
        put(&mut b, self.obligation.as_bytes());
        put(&mut b, self.rail.as_bytes());
        put(&mut b, self.expiry_unix.to_string().as_bytes());
        put(&mut b, self.reservation.as_bytes());
        b
    }

    /// `sha256:<hex>` effect id.
    pub fn effect_id(&self) -> String {
        format!("sha256:{}", hex(&sha256(&self.canonical_bytes())))
    }
}

/// Typed refusal: the presented effect is not the one the authorization was bound to.
#[derive(Debug, PartialEq, Eq)]
pub enum Refusal {
    EffectIdentityMismatch { bound: String, presented: String },
}

impl Refusal {
    pub fn code(&self) -> &'static str {
        "REFUSED_EFFECT_IDENTITY_MISMATCH"
    }
}

/// Admit only if `presented` recomputes to the id the authorization was bound to.
pub fn verify_binding(bound_effect_id: &str, presented: &EffectIdentity) -> Result<(), Refusal> {
    let p = presented.effect_id();
    if p == bound_effect_id {
        Ok(())
    } else {
        Err(Refusal::EffectIdentityMismatch { bound: bound_effect_id.to_string(), presented: p })
    }
}
