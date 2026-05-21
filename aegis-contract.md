# Contract — `hex-real`

A vendored Rust implementation of hexadecimal encoding /
decoding. The bug-and-repair scope is `val()` — the
single-character ASCII-hex-to-nibble decoder. The locked
harness pins one narrow property; the behaviour gate
covers the full character-class table + real-decode vectors.

## `val`

```rust
pub fn val(c: u8, idx: usize) -> Result<u8, FromHexError>
```

Decode one ASCII hex character to its 0..16 nibble value.

- **Lowercase-letter contract (load-bearing):** for any
  `c` in `b'a'..=b'f'`, the returned nibble equals
  `c - b'a' + 10`. In particular,
  `val(b'a', 0) == Ok(10)`.
- Uppercase: `c` in `b'A'..=b'F'` → `Ok(c - b'A' + 10)`.
- Digit: `c` in `b'0'..=b'9'` → `Ok(c - b'0')`.
- Invalid: any other byte →
  `Err(FromHexError::InvalidHexCharacter { c, index: idx })`.
- **Panic freedom:** the function must not panic for any
  input.

The Kani harness verifies ONLY the single
`val(b'a', 0) == Ok(10)` anchor. No symbolic input — pure
single-value check, fast for CBMC.

## How the planted bug surfaces

The vendored `val()` returns `Ok(c - b'a' + 11)` for the
lowercase-letter case. Effect: `val(b'a', 0)` returns
`Ok(11)` rather than `Ok(10)`, falsifying the harness
assertion above and breaking every decode that contains a
lowercase hex letter. The repair is to replace the `+ 11`
with `+ 10` in the `b'a'..=b'f'` match arm.
