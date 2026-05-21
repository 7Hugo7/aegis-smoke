# Intent — `hex-real`

A vendored Rust implementation of hexadecimal (Base16, RFC
4648) encoding and decoding. Different shape from
`base16ct-real`: this crate uses an allocation-based public
API (`hex::encode("…") -> String`,
`hex::decode("…") -> Result<Vec<u8>, _>`) plus the `ToHex` /
`FromHex` traits, rather than base16ct's slice-only surface.

The bug-and-repair scope is `val()` — the single-character
ASCII-hex-to-nibble decoder.

## Crate overview

`val(c: u8, idx: usize) -> Result<u8, FromHexError>` decodes
one ASCII hex character to its 0..16 nibble value:

- `'A'..='F'` → 10..=15 (uppercase letters)
- `'a'..='f'` → 10..=15 (lowercase letters)
- `'0'..='9'` → 0..=9 (digits)
- anything else → `Err(InvalidHexCharacter)`

The function is the canonical inverse of the encoder's
nibble-to-ASCII step. Every `decode` / `from_hex` call walks
the input two characters at a time and combines two
`val(...)` results as `(high << 4) | low`. Correctness of
`val()` is load-bearing for the entire crate — a one-off in
the lowercase-letter case (or the uppercase, or the digit)
silently corrupts every decode that touches that range.

## `val`

```rust
pub fn val(c: u8, idx: usize) -> Result<u8, FromHexError>
```

Decode one ASCII hex character to 0..16. The contract is:

- For `c` in `b'A'..=b'F'`, returns `Ok(c - b'A' + 10)` — so
  `val(b'A', _) == Ok(10)` and `val(b'F', _) == Ok(15)`.
- For `c` in `b'a'..=b'f'`, returns `Ok(c - b'a' + 10)` — so
  `val(b'a', _) == Ok(10)` and `val(b'f', _) == Ok(15)`.
- For `c` in `b'0'..=b'9'`, returns `Ok(c - b'0')` — so
  `val(b'0', _) == Ok(0)` and `val(b'9', _) == Ok(9)`.
- For any other byte, returns `Err(FromHexError::
  InvalidHexCharacter { c, index: idx })`.
- MUST NOT panic for any input.

The vendored `val()` has the lowercase letter case off by
one: it returns `Ok(c - b'a' + 11)` instead of `Ok(c - b'a'
+ 10)`. Effect: `val(b'a', _) == Ok(11)` (should be 10),
`val(b'b', _) == Ok(12)` (should be 11), and so on through
`val(b'f', _) == Ok(16)` (should be 15). The repair is to
replace the `+ 11` with `+ 10` in the `b'a'..=b'f'` arm.

The uppercase case (`b'A' + 10`) and digit case (`c -
b'0'`) are correct in the vendored copy — the LLM should
NOT modify them.

The Kani harness should verify ONLY the single anchor
`val(b'a', 0) == Ok(10)`. The full lowercase / uppercase /
digit / invalid-character coverage lives in the locked
behaviour tests.

## Why this shape

Different from `base16ct-real`'s nibble-swap bug (which was
about byte-position ordering inside a multi-byte encode):
this is about a single-character lookup table, more like a
configuration error than an algorithmic one. Different bug
class for the Phase-A "works on real crates" claim.
