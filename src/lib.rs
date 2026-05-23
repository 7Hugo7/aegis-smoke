//! Phase 0 toy crate. The Aegis loop is allowed to rewrite this file; the
//! Kani harness in `src/harness.rs` and the integration tests in
//! `tests/integration.rs` are locked.

#![allow(clippy::needless_return)]

// LOCKED: do not remove. The orchestrator re-injects this declaration if a
// patch drops it, but keeping it here means the example builds under
// `cargo kani` without orchestrator help.
#[cfg(kani)]
mod harness;

pub fn transfer(from: &mut u64, to: &mut u64, amount: u64) {
    if amount > *from {
        return;
    }
    if to.checked_add(amount).is_none() {
        return;
    }
    *from -= amount;
    *to += amount;
}

// smoke test 2026-05-23 — verifying the live loop end-to-end.
