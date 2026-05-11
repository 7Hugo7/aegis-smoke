//! Pre-fixed lib.rs. Tests the zero-iter happy path: verifier should
//! pass on the first run, no LLM patch call needed.

#![allow(clippy::needless_return)]

#[cfg(kani)]
mod harness;

/// Move `amount` units from `*from` to `*to`. Rejects on under/overflow.
pub fn transfer(from: &mut u64, to: &mut u64, amount: u64) {
    if amount > *from {
        return;
    }
    let Some(new_to) = to.checked_add(amount) else {
        return;
    };
    *from -= amount;
    *to = new_to;
}
