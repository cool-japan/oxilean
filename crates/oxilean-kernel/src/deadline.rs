//! Per-declaration wall-clock deadline (M4 hardening).
//!
//! The deterministic per-declaration [`fuel`](crate::fuel) bounds *node
//! construction*, which caps the memory and time of any declaration whose cost
//! shows up as newly-built `Expr` material. But the Mathlib `CategoryTheory`
//! corpus exposed a reduction/def-eq loop that makes progress WITHOUT
//! constructing nodes (e.g. re-deriving the same universe/def-eq obligations),
//! so it never exhausts fuel and runs unboundedly — a single declaration
//! (`CategoryTheory.FreeBicategory`…) pinned one core at 100 % indefinitely.
//!
//! This module adds a thread-local **wall-clock** deadline as a backstop.
//! [`is_expired`](crate::deadline::is_expired) is OR-ed into [`fuel::is_exhausted`](crate::fuel::is_exhausted),
//! so every place the kernel already degrades on fuel exhaustion — `whnf`
//! returns its input, `is_def_eq` decides syntactically (a conservative
//! `false`), `infer_type` aborts — now ALSO honours the deadline, and the caller
//! reports the declaration as a NAMED resource limit. It never changes a
//! positive or negative verdict; it only moves an otherwise-unbounded
//! declaration into the resource-limit bucket instead of hanging.
//!
//! ## Determinism caveat
//!
//! Unlike fuel (a node count), a wall-clock deadline is machine- and
//! load-dependent: WHICH declarations hit it varies with hardware. That is the
//! price of bounding computations the deterministic meter cannot see, and it is
//! disclosed wherever the time budget is set. Soundness is unaffected — an
//! expired declaration is degraded exactly like a fuel-exhausted one, so it is
//! always a named `unsupported`, never a wrong accept or reject.
//!
//! The default is *no deadline* ([`set_deadline`](crate::deadline::set_deadline)`(None)`): plain kernel users
//! (tests, embedders) are unaffected unless they opt in.

use crate::wall_clock::Instant;
use core::time::Duration;
use std::cell::Cell;

thread_local! {
    /// When the current unit of work started (`None` = no deadline).
    static START: Cell<Option<Instant>> = const { Cell::new(None) };
    /// The wall-clock budget for the current unit of work.
    static BUDGET: Cell<Duration> = const { Cell::new(Duration::ZERO) };
    /// Latched expiry flag (set once the budget is first observed exceeded).
    static EXPIRED: Cell<bool> = const { Cell::new(false) };
    /// Call counter, so the clock is only sampled every `CHECK_MASK + 1` calls.
    static TICK: Cell<u32> = const { Cell::new(0) };
}

/// Sample the clock only every 1024 [`is_expired`](crate::deadline::is_expired) calls. The hot reduction
/// loops call it once per step, so the amortised cost is ~one `Instant::now`
/// per 1024 steps while a runaway loop is still cut off within 1024 steps of
/// the deadline.
const CHECK_MASK: u32 = 0x3FF;

/// Reset the deadline for a new unit of work (typically: one declaration).
///
/// `Some(budget)` arms a wall-clock budget starting now; `None` disarms it.
/// Both clear the expiry latch and the sampling counter.
pub fn set_deadline(budget: Option<Duration>) {
    match budget {
        Some(b) => {
            START.with(|s| s.set(Some(Instant::now())));
            BUDGET.with(|d| d.set(b));
        }
        None => START.with(|s| s.set(None)),
    }
    EXPIRED.with(|e| e.set(false));
    TICK.with(|t| t.set(0));
}

/// Whether the current thread's wall-clock budget has been exceeded since the
/// last [`set_deadline`]. Latches on the first observed expiry. The clock is
/// sampled only periodically (see `CHECK_MASK`) to keep hot-loop cost
/// negligible; with no deadline armed it is a single `Cell` read.
#[inline]
#[must_use]
pub fn is_expired() -> bool {
    if EXPIRED.with(|e| e.get()) {
        return true;
    }
    let tick = TICK.with(|t| {
        let v = t.get().wrapping_add(1);
        t.set(v);
        v
    });
    if tick & CHECK_MASK != 0 {
        return false;
    }
    START.with(|s| {
        if let Some(start) = s.get() {
            if start.elapsed() >= BUDGET.with(|d| d.get()) {
                EXPIRED.with(|e| e.set(true));
                return true;
            }
        }
        false
    })
}

/// Whether the deadline latched at any point since the last [`set_deadline`]
/// (without sampling the clock). Used by callers to attribute a degraded
/// verdict to the time budget.
#[must_use]
pub fn was_expired() -> bool {
    EXPIRED.with(|e| e.get())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_deadline_never_expires() {
        set_deadline(None);
        for _ in 0..10_000 {
            assert!(!is_expired());
        }
        assert!(!was_expired());
    }

    #[test]
    fn zero_budget_expires_and_latches() {
        set_deadline(Some(Duration::ZERO));
        // The clock is sampled every 1024 calls; drive past one sample.
        let mut fired = false;
        for _ in 0..2048 {
            if is_expired() {
                fired = true;
                break;
            }
        }
        assert!(
            fired,
            "a zero budget must expire within one sample interval"
        );
        assert!(was_expired(), "expiry latches");
        // Latch persists and short-circuits without sampling.
        assert!(is_expired());
        // Re-arming clears the latch.
        set_deadline(None);
        assert!(!was_expired());
    }
}
