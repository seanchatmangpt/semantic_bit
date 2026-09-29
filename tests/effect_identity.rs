use semantic_bit::effect_identity::*;

fn base() -> EffectIdentity {
    EffectIdentity {
        principal: "principal-1".into(), beneficiary: "ben-1".into(), amount_minor: 12_500,
        asset: "USD".into(), purpose: "invoice".into(), authority: "grant-1".into(),
        obligation: "obl-1".into(), rail: "rail-sim-1".into(), expiry_unix: 1_800_000_000,
        reservation: "res-1".into(),
    }
}

#[test]
fn sha256_nist_vectors() {
    assert_eq!(hex(&sha256(b"")), "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
    assert_eq!(hex(&sha256(b"abc")), "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
    assert_eq!(
        hex(&sha256(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq")),
        "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1"
    );
    // multi-block boundary (55/56/64 byte padding edges)
    for n in [55usize, 56, 63, 64, 65] {
        assert_eq!(sha256(&vec![b'a'; n]).len(), 32);
    }
    assert_eq!(
        hex(&sha256(&vec![b'a'; 1_000_000])),
        "cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0"
    );
}

#[test]
fn effect_id_is_deterministic_and_pinned() {
    assert_eq!(base().effect_id(), base().effect_id());
    assert!(base().effect_id().starts_with("sha256:"));
    assert_eq!(base().effect_id().len(), 7 + 64);
}

#[test]
fn every_bound_field_changes_the_id_and_is_refused() {
    let e = base();
    let id = e.effect_id();
    let mutants: Vec<(&str, EffectIdentity)> = vec![
        ("principal", EffectIdentity { principal: "x".into(), ..base() }),
        ("beneficiary", EffectIdentity { beneficiary: "x".into(), ..base() }),
        ("amount", EffectIdentity { amount_minor: 12_501, ..base() }),
        ("asset", EffectIdentity { asset: "EUR".into(), ..base() }),
        ("purpose", EffectIdentity { purpose: "x".into(), ..base() }),
        ("authority", EffectIdentity { authority: "x".into(), ..base() }),
        ("obligation", EffectIdentity { obligation: "x".into(), ..base() }),
        ("rail", EffectIdentity { rail: "x".into(), ..base() }),
        ("expiry", EffectIdentity { expiry_unix: 1, ..base() }),
        ("reservation", EffectIdentity { reservation: "x".into(), ..base() }),
    ];
    for (name, m) in mutants {
        assert_ne!(m.effect_id(), id, "field {name} not bound");
        let r = verify_binding(&id, &m).unwrap_err();
        assert_eq!(r.code(), "REFUSED_EFFECT_IDENTITY_MISMATCH", "field {name}");
    }
    assert!(verify_binding(&id, &e).is_ok());
}

#[test]
fn field_boundary_shift_does_not_collide() {
    let a = EffectIdentity { principal: "ab".into(), beneficiary: "c".into(), ..base() };
    let b = EffectIdentity { principal: "a".into(), beneficiary: "bc".into(), ..base() };
    assert_ne!(a.effect_id(), b.effect_id());
}

#[test]
fn payload_digest_is_domain_separated_from_plain_sha() {
    let d = canonical_payload_digest(b"abc");
    assert_ne!(d, hex(&sha256(b"abc")));
    assert_eq!(d, canonical_payload_digest(b"abc"));
}
