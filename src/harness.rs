//! Kani proof harness — locked. The Aegis loop reads this file but
//! never writes it. The locked property pins the lowercase-letter
//! contract on `val()`: for any byte `c` in `b'a'..=b'f'`, the
//! returned nibble equals `c - b'a' + 10`. The off-by-one bug
//! planted in the vendored `val()` returns `c - b'a' + 11` for
//! that range, falsifying this assertion immediately for the
//! single canonical anchor `'a' → 10`.

use crate::val;

#[kani::proof]
fn check_val_lowercase_letters() {
    // 'a' must decode to nibble 10 (the lowest letter case). This
    // is the single load-bearing identity: if 'a' decodes to 11
    // (the planted bug's effect) every lowercase-hex decode is
    // off by one for letter inputs.
    let result = val(b'a', 0);
    assert!(result == Ok(10));
}
