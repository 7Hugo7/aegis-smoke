//! Behaviour gate — locked. The Aegis loop reads this file but never
//! writes it. `cargo test --tests` must pass after every patch for
//! the run to count as successful, alongside the Kani gate and the
//! bypass scanner.
//!
//! Vectors taken from the upstream README + RFC 4648's Appendix B.
//! The off-by-one bug planted in `val()`'s lowercase-letter case
//! returns the wrong nibble for any letter input, so every decode
//! that contains a-f produces incorrect bytes.

use aegis_example_hex_real::{decode, val};

#[test]
fn val_lowercase_letters_decode_to_10_through_15() {
    assert_eq!(val(b'a', 0), Ok(10));
    assert_eq!(val(b'b', 0), Ok(11));
    assert_eq!(val(b'c', 0), Ok(12));
    assert_eq!(val(b'd', 0), Ok(13));
    assert_eq!(val(b'e', 0), Ok(14));
    assert_eq!(val(b'f', 0), Ok(15));
}

#[test]
fn val_uppercase_letters_decode_to_10_through_15() {
    // Uppercase path is correct in the vendored copy — pin it as
    // a regression anchor so the loop doesn't accidentally break
    // it while fixing the lowercase path.
    assert_eq!(val(b'A', 0), Ok(10));
    assert_eq!(val(b'F', 0), Ok(15));
}

#[test]
fn val_digits_decode_to_0_through_9() {
    assert_eq!(val(b'0', 0), Ok(0));
    assert_eq!(val(b'9', 0), Ok(9));
}

#[test]
fn decode_hello_world_round_trip() {
    // Upstream README vector: `"Hello world!"` round-trips through
    // hex. The fixed lowercase-letter conversion is required for
    // this — `b'c'`, `b'e'`, `b'f'`, `b'b'`, `b'd'`, `b'4'` are
    // all lowercase letters that decode to 10..=15.
    let encoded = "48656c6c6f20776f726c6421";
    assert_eq!(
        decode(encoded).unwrap(),
        b"Hello world!".to_vec()
    );
}

#[test]
fn decode_deadbeef() {
    let encoded = "deadbeef";
    assert_eq!(decode(encoded).unwrap(), vec![0xde, 0xad, 0xbe, 0xef]);
}
