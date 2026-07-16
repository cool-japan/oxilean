//! Deterministic per-declaration resource fuel (C16).
//!
//! One declaration must never be able to OOM the process: Lean core's
//! `Int.add_mul_ediv_right` (Init corpus) legally expands to multi-hundred-
//! million-node intermediate terms inside `whnf`/`is_def_eq`, and with a
//! deep-copy `Expr` representation that is a multi-GiB peak. This module
//! provides a *deterministic* budget on the term-duplication work a single
//! declaration may perform, measured in **`Expr` nodes cloned or rebuilt**:
//!
//! * [`Expr::clone`](crate::Expr) charges one unit per node cloned
//!   (`instantiate` argument copies, cache result clones, `mk_app` rebuilds);
//! * the substitution/lift builders (`subst::instantiate*`, `shift_bvars`,
//!   `abstract_expr`, `parallel_subst`, `instantiate::{instantiate_rev,
//!   instantiate_many, instantiate_type_lparams}`,
//!   `expr_util::lift_loose_bvars`) charge one unit per node **visited**, so
//!   spine nodes rebuilt without passing through `clone` are bounded too
//!   (the C16b gap: a `let`-tower duplicating its value at every level
//!   allocated multi-GiB while burning almost no clone-fuel).
//!
//! ## Contract
//!
//! * The counter is thread-local; a checking thread calls
//!   [`set_budget`](crate::fuel::set_budget)`(Some(n))` before a declaration and inspects
//!   [`is_exhausted`](crate::fuel::is_exhausted) after. [`Expr::clone`](crate::Expr) charges one unit
//!   per node cloned.
//! * When the budget hits zero the flag latches. Reduction, definitional
//!   equality and type inference then *degrade conservatively*: `whnf`
//!   returns its input unchanged (a stuck term — always sound), `is_def_eq`
//!   decides only syntactic equality (never a wrong accept), and
//!   `infer_type` aborts with a typed error (C16b — terms are never
//!   truncated; the in-flight substitution completes exactly and the next
//!   inference step aborts). Positive judgements reached after exhaustion
//!   are therefore still exact; failures are reported by the caller as a
//!   **named resource limit**, not as a rejection.
//! * Determinism: the same declaration checked with the same budget always
//!   exhausts at the same point — the count depends only on the reduction
//!   path taken, never on wall-clock time or allocator state.
//!
//! The default is *unlimited* ([`set_budget`](crate::fuel::set_budget)`(None)`): plain kernel users
//! (tests, embedders) are unaffected unless they opt in.

use std::cell::Cell;

thread_local! {
    /// Remaining fuel, in `Expr` nodes cloned. `u64::MAX` means unlimited.
    static FUEL: Cell<u64> = const { Cell::new(u64::MAX) };
    /// Latched exhaustion flag (set when the budget crosses zero).
    static EXHAUSTED: Cell<bool> = const { Cell::new(false) };
    /// Total nodes charged since the last [`set_budget`](crate::fuel::set_budget) (for reporting).
    static USED: Cell<u64> = const { Cell::new(0) };
}

/// Reset the fuel for a new unit of work (typically: one declaration).
///
/// `Some(n)` sets a budget of `n` cloned nodes; `None` is unlimited. Both
/// clear the exhaustion latch and the usage counter.
pub fn set_budget(budget: Option<u64>) {
    FUEL.with(|f| f.set(budget.unwrap_or(u64::MAX)));
    EXHAUSTED.with(|e| e.set(false));
    USED.with(|u| u.set(0));
}

/// Charge `n` units of fuel. Saturates at zero and latches [`is_exhausted`].
#[inline]
pub fn charge(n: u64) {
    USED.with(|u| u.set(u.get().saturating_add(n)));
    FUEL.with(|f| {
        let cur = f.get();
        if cur == u64::MAX {
            return; // unlimited: count usage, never exhaust
        }
        if cur >= n {
            f.set(cur - n);
            if cur == n {
                EXHAUSTED.with(|e| e.set(true));
            }
        } else {
            f.set(0);
            EXHAUSTED.with(|e| e.set(true));
        }
    });
}

/// Whether the current thread's per-declaration resource budget is spent —
/// either the deterministic node-construction [`fuel`](self) is exhausted, OR
/// the wall-clock [`deadline`](crate::deadline) has expired.
///
/// Folding the deadline in here means every place the kernel already degrades
/// conservatively on fuel exhaustion (`whnf` returns its input, `is_def_eq`
/// decides syntactically, `infer_type` aborts) automatically also honours the
/// deadline, so a non-fuel-metered runaway loop is cut off and reported as a
/// named resource limit instead of hanging. The deadline check samples the
/// clock only periodically; with no deadline armed it is a single `Cell` read.
#[inline]
#[must_use]
pub fn is_exhausted() -> bool {
    EXHAUSTED.with(|e| e.get()) || crate::deadline::is_expired()
}

/// Total units charged since the last [`set_budget`] (for reporting and
/// budget calibration).
#[must_use]
pub fn used() -> u64 {
    USED.with(|u| u.get())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unlimited_never_exhausts_but_counts() {
        set_budget(None);
        charge(1_000_000);
        assert!(!is_exhausted());
        assert_eq!(used(), 1_000_000);
        set_budget(None);
        assert_eq!(used(), 0);
    }

    #[test]
    fn budget_latches_on_exhaustion() {
        set_budget(Some(10));
        charge(9);
        assert!(!is_exhausted());
        charge(1);
        assert!(is_exhausted(), "hitting exactly zero must latch");
        charge(0);
        assert!(is_exhausted(), "latch must persist");
        set_budget(Some(10));
        assert!(!is_exhausted(), "set_budget clears the latch");
    }

    #[test]
    fn overshoot_saturates() {
        set_budget(Some(5));
        charge(100);
        assert!(is_exhausted());
        assert_eq!(used(), 100);
        set_budget(None);
    }
}
