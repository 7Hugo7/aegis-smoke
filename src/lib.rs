//! Phase 0 toy crate. The Aegis loop is allowed to rewrite this file; the
//! Kani harness in `src/harness.rs` and the integration tests in
//! `tests/integration.rs` are locked.

#![allow(clippy::needless_return)]

// LOCKED: do not remove. The orchestrator re-injects this declaration if a
// patch drops it, but keeping it here means the example builds under
// `cargo kani` without orchestrator help.
#[cfg(kani)]
mod harness;

/// Move `amount` units from `*from` to `*to`.
///
/// **Buggy on purpose.** This is the bug the loop has to fix:
///   - It does not check that `*from >= amount` (underflow).
///   - It does not check that `*to + amount` fits in `u64` (overflow).
pub fn transfer(from: &mut u64, to: &mut u64, amount: u64) {
    *from -= amount;
    *to += amount;
}

// trigger: managed-path prod validation run
