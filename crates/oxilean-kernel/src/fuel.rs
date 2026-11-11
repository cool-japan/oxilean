//! Deterministic per-declaration resource fuel (C16).
//!
//! One declaration must never be able to OOM the process. This module
//! provides a *deterministic* budget on the work a single declaration may
//! make the kernel do, measured in **subterms the kernel's term builders
//! reach**:
//!
//! * [`Expr::clone`](crate::Expr) charges one unit per call (a clone copies
//!   one node; what the node points to is shared);
//! * the walks that build terms (`subst::instantiate`, `abstract_expr`,
//!   `expr_util::lift_loose_bvars`, `instantiate::instantiate_type_lparams`,
//!   and the plainer `shift_bvars`, `parallel_subst`,
//!   `instantiate::{instantiate_rev, instantiate_many}`) charge one unit per
//!   subterm they reach — whether they rebuild it, share it unchanged or
//!   find it already rebuilt — and the spine builder of the reducer charges
//!   one per application it allocates.
//!
//! So every node the kernel allocates while checking a declaration is
//! charged, and a budget of `n` units bounds both the time spent (a unit is
//! a few tens of nanoseconds) and the memory that can be live at once (`n`
//! nodes, and what the caches hold of them).
//!
//! ## What a unit used to be
//!
//! Through 0.1.4's first performance pass a substitution that SHARED an
//! untouched subterm was charged what rebuilding it node by node would have
//! cost — its size as a tree, a shared subterm once per path to it. That
//! kept verdicts as they had been before terms were shared, and it made the
//! budget a measure of something the kernel no longer did: a proof term of a
//! few thousand shared nodes stands for millions of tree nodes, and 632
//! declarations of Lean core `Init` — 111 of a Mathlib slice — were
//! `unsupported` after a tenth of a second of work. A shared subterm now
//! costs one unit, as it costs one reference count to share.
//!
//! ## Contract
//!
//! * The counter is thread-local; a checking thread calls
//!   [`set_budget`](crate::fuel::set_budget)`(Some(n))` before a declaration and inspects
//!   [`is_exhausted`](crate::fuel::is_exhausted) after.
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
//!   exhausts at the same point — the count depends only on the terms and
//!   the reduction path taken (which subterms of a term are one allocation
//!   is part of the term as it was read), never on wall-clock time or
//!   allocator state.
//!
//! * Reduction limits: the reducer also has limits of its own — a cap on
//!   nesting depth (it protects the native stack; a caller whose stack is
//!   small lowers it with
//!   [`set_reduction_nesting_limit`](crate::fuel::set_reduction_nesting_limit))
//!   and a cap on the length of
//!   one head-reduction chain. A reduction cut short by either is NOT a
//!   normal form, so a failure that follows it is not a verdict: the reducer
//!   records the event with [`note_reduction_limit`](crate::fuel::note_reduction_limit), and a caller that
//!   finds [`reduction_limit_hit`](crate::fuel::reduction_limit_hit) set must not report the failure as a
//!   rejection. Unlike exhaustion this latch degrades nothing; it is only
//!   read when a failure is classified.
//!
//!   The chain cap is also what lets definitional equality give up on a
//!   reduction that is going nowhere and try another route, so it stays on
//!   by default. A caller whose check FAILED after the cap fired can repeat
//!   it with the cap lifted ([`set_reduction_cap`](crate::fuel::set_reduction_cap)`(false)`): the chain is
//!   then bounded by the budget alone — every step charges fuel and polls
//!   the deadline — so a long but cheap chain runs to its normal form. The
//!   cap is only ever lifted while a budget is armed
//!   ([`is_metered`](crate::fuel::is_metered)); with none, nothing else would stop a runaway chain.
//!
//! The default is *unlimited* ([`set_budget`](crate::fuel::set_budget)`(None)`): plain kernel users
//! (tests, embedders) are unaffected unless they opt in.

use std::cell::Cell;

thread_local! {
    /// Remaining fuel, in units (see the module documentation). `u64::MAX`
    /// means unlimited.
    static FUEL: Cell<u64> = const { Cell::new(u64::MAX) };
    /// Latched exhaustion flag (set when the budget crosses zero).
    static EXHAUSTED: Cell<bool> = const { Cell::new(false) };
    /// Total units charged since the last [`set_budget`](crate::fuel::set_budget) (for reporting).
    static USED: Cell<u64> = const { Cell::new(0) };
    /// Latched: the reducer cut a reduction short at one of its own limits
    /// (nesting depth or chain length) since the last
    /// [`set_budget`](crate::fuel::set_budget).
    static REDUCTION_LIMIT_HIT: Cell<bool> = const { Cell::new(false) };
    /// Whether the reducer applies its own cap to the length of a
    /// head-reduction chain (the default). See
    /// [`set_reduction_cap`](crate::fuel::set_reduction_cap).
    static REDUCTION_CAP: Cell<bool> = const { Cell::new(true) };
    /// How deep a term may be on this thread (`u32::MAX` = no limit). See
    /// [`set_term_depth_limit`](crate::fuel::set_term_depth_limit).
    static TERM_DEPTH_LIMIT: Cell<u32> = const { Cell::new(u32::MAX) };
    /// Latched: a term deeper than that limit was built, or offered for
    /// checking, since the last [`set_budget`](crate::fuel::set_budget).
    static TERM_DEPTH_HIT: Cell<bool> = const { Cell::new(false) };
    /// The reductions nested inside one another on this thread, and how
    /// many there may be. See
    /// [`set_reduction_nesting_limit`](crate::fuel::set_reduction_nesting_limit).
    static NESTING: Nesting = const {
        Nesting {
            limit: Cell::new(u32::MAX),
            current: Cell::new(0),
        }
    };
}

/// One thread's count of nested reductions (one thread-local, so that
/// entering a reduction is one lookup).
struct Nesting {
    /// How many reductions may be nested inside one another (`u32::MAX` =
    /// no limit of this kind).
    limit: Cell<u32>,
    /// How many are nested inside one another right now.
    current: Cell<u32>,
}

/// Reset the fuel for a new unit of work (typically: one declaration).
///
/// `Some(n)` sets a budget of `n` units; `None` is unlimited. Both
/// clear the exhaustion latch, the reduction-limit and term-depth latches
/// and the usage counter, and put the reducer's chain cap back on.
pub fn set_budget(budget: Option<u64>) {
    FUEL.with(|f| f.set(budget.unwrap_or(u64::MAX)));
    EXHAUSTED.with(|e| e.set(false));
    USED.with(|u| u.set(0));
    REDUCTION_LIMIT_HIT.with(|l| l.set(false));
    TERM_DEPTH_HIT.with(|h| h.set(false));
    REDUCTION_CAP.with(|c| c.set(true));
}

/// Whether a budget bounds the current unit of work: a finite fuel budget
/// (spent or not) or an armed wall-clock [`deadline`](crate::deadline).
///
/// With a budget armed, a head-reduction chain ends even without the
/// reducer's own cap — each step charges fuel and polls the deadline. With
/// none, only that cap ends it.
#[inline]
#[must_use]
pub fn is_metered() -> bool {
    FUEL.with(|f| f.get() != u64::MAX) || crate::deadline::is_armed()
}

/// Turn the reducer's cap on the length of a head-reduction chain off
/// (`false`) or back on (`true`, the default; [`set_budget`] restores it).
///
/// Lift the cap to REPEAT a check that failed after the cap had fired
/// ([`reduction_limit_hit`]): a chain the cap cut short may be a long but
/// cheap computation the proof depends on, and the budget is enough to bound
/// it. The request is honoured only while a budget is armed
/// ([`is_metered`]) — see [`reduction_cap_applies`].
#[inline]
pub fn set_reduction_cap(enabled: bool) {
    REDUCTION_CAP.with(|c| c.set(enabled));
}

/// Whether the reducer must apply its own cap to the length of a
/// head-reduction chain right now: always, unless the cap was lifted with
/// [`set_reduction_cap`]`(false)` AND a budget is armed to bound the chain in
/// its place.
#[inline]
#[must_use]
pub fn reduction_cap_applies() -> bool {
    REDUCTION_CAP.with(|c| c.get()) || !is_metered()
}

/// Limit how many reductions may be nested inside one another on this
/// thread (`None`, the default: no limit of this kind; the reducer's own
/// depth guard still applies). The setting is the thread's: [`set_budget`]
/// leaves it.
///
/// A reduction nested inside another — the major premise of a recursor, the
/// structure of a projection, an argument of a literal operation — holds
/// native stack until it is done, and how many fit is a matter of the
/// caller's stack, not of the term. A caller that cannot give the kernel a
/// deep stack sets this to what its stack holds (`oxilean-verify` runs on a
/// thread of 512 MiB and sets none; its WebAssembly build, on a browser's
/// megabyte, sets 256).
///
/// A reduction that would nest deeper is not started: the term comes back
/// as it is — sound, but not a normal form — and the event is recorded
/// ([`note_reduction_limit`]), so a failure that follows is reported as a
/// resource limit, never as a rejection.
#[inline]
pub fn set_reduction_nesting_limit(limit: Option<u32>) {
    NESTING.with(|n| n.limit.set(limit.unwrap_or(u32::MAX)));
}

/// The limit set by [`set_reduction_nesting_limit`] (`None` = none).
#[inline]
#[must_use]
pub fn reduction_nesting_limit() -> Option<u32> {
    let limit = NESTING.with(|n| n.limit.get());
    (limit != u32::MAX).then_some(limit)
}

/// One reduction in progress on this thread, counted against the limit of
/// [`set_reduction_nesting_limit`] from [`NestedReduction::enter`] until it
/// is dropped — also when the reduction is left by unwinding.
pub(crate) struct NestedReduction(());

impl NestedReduction {
    /// Enter one more nested reduction. `None` — and nothing entered — when
    /// that would exceed the limit; the caller then returns its term
    /// unreduced and records the event ([`note_reduction_limit`]).
    #[inline]
    #[must_use]
    pub(crate) fn enter() -> Option<Self> {
        NESTING.with(|n| {
            let current = n.current.get();
            if current >= n.limit.get() {
                return None;
            }
            n.current.set(current + 1);
            Some(Self(()))
        })
    }
}

impl Drop for NestedReduction {
    #[inline]
    fn drop(&mut self) {
        NESTING.with(|n| n.current.set(n.current.get().saturating_sub(1)));
    }
}

/// Whether this build of the kernel tracks the depth of its terms, which is
/// what makes [`set_term_depth_limit`] effective: the crate's `term-depth`
/// feature.
pub const TERM_DEPTH_TRACKED: bool = cfg!(feature = "term-depth");

/// Limit how deep a term may be on this thread (`None`, the default: no
/// limit). The depth of a term is the number of nodes on the longest path
/// from it down to a leaf. The setting is the thread's: [`set_budget`]
/// leaves it.
///
/// The kernel walks a term by recursion — to substitute into it, to infer
/// its type, to compare it with another, and to free it — so the deepest
/// term alive decides how much native stack those walks need, as the
/// nesting of reductions does for the reducer
/// ([`set_reduction_nesting_limit`]). A term nested thousands deep need not
/// be in the input: an argument that a recursion adds to at every step is
/// one after thousands of steps. A caller that cannot give the kernel a deep
/// stack sets this to what its stack holds (the WebAssembly build of
/// `oxilean-verify` sets 512).
///
/// Building a deeper term spends the budget: the exhaustion latch is set
/// ([`is_exhausted`]), so reduction stops where it is, type inference aborts
/// at its next step, and the failure that follows is reported as a resource
/// limit — this one, by name ([`term_depth_limit_hit`]) — never as a
/// rejection. A declaration whose own type or value is deeper than the limit
/// is refused before any of it is walked. The substitution in flight when
/// the limit is crossed completes exactly (terms are never truncated), so a
/// term up to twice the limit deep can exist for a moment; a caller sizing
/// the limit to its stack leaves room for that.
///
/// Effective only where the depth of terms is tracked
/// ([`TERM_DEPTH_TRACKED`]); in a build without the feature the limit is
/// kept and never reached.
#[inline]
pub fn set_term_depth_limit(limit: Option<u32>) {
    TERM_DEPTH_LIMIT.with(|l| l.set(limit.unwrap_or(u32::MAX)));
}

/// The limit set by [`set_term_depth_limit`] (`None` = none).
#[inline]
#[must_use]
pub fn term_depth_limit() -> Option<u32> {
    let limit = TERM_DEPTH_LIMIT.with(|l| l.get());
    (limit != u32::MAX).then_some(limit)
}

/// Whether a term deeper than the limit of [`set_term_depth_limit`] was
/// built, or offered for checking, since the last [`set_budget`]. The
/// budget is then spent ([`is_exhausted`]) for this reason and no other.
#[inline]
#[must_use]
pub fn term_depth_limit_hit() -> bool {
    TERM_DEPTH_HIT.with(|h| h.get())
}

/// Record a term of depth `depth`: past the limit of
/// [`set_term_depth_limit`] it latches [`term_depth_limit_hit`] and spends
/// the budget.
#[cfg(feature = "term-depth")]
#[inline]
pub(crate) fn note_term_depth(depth: u32) {
    if depth > TERM_DEPTH_LIMIT.with(|l| l.get()) {
        TERM_DEPTH_HIT.with(|h| h.set(true));
        EXHAUSTED.with(|e| e.set(true));
    }
}

/// Record that the reducer cut a reduction short at one of its own limits
/// and returned a term that is not a normal form. Latches until the next
/// [`set_budget`].
#[inline]
pub fn note_reduction_limit() {
    REDUCTION_LIMIT_HIT.with(|l| l.set(true));
}

/// Clear the reduction-limit latch without touching the fuel budget — for a
/// caller that starts a new unit of work under a budget it does not own.
#[inline]
pub fn clear_reduction_limit() {
    REDUCTION_LIMIT_HIT.with(|l| l.set(false));
}

/// Whether [`note_reduction_limit`] was called since the last [`set_budget`]
/// (or [`clear_reduction_limit`]).
///
/// A failure reached with this set may be an artefact of the truncated
/// reduction rather than a verdict; callers report it as a resource limit,
/// never as a rejection.
#[inline]
#[must_use]
pub fn reduction_limit_hit() -> bool {
    REDUCTION_LIMIT_HIT.with(|l| l.get())
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
    fn reduction_limit_latch_is_independent_of_exhaustion() {
        set_budget(None);
        assert!(!reduction_limit_hit());
        note_reduction_limit();
        assert!(reduction_limit_hit(), "the latch must persist");
        assert!(
            !is_exhausted(),
            "a reduction-limit hit must not degrade the kernel"
        );
        set_budget(Some(10));
        assert!(!reduction_limit_hit(), "set_budget clears the latch");
        note_reduction_limit();
        clear_reduction_limit();
        assert!(!reduction_limit_hit(), "clear_reduction_limit clears it");
        assert!(is_metered(), "clearing the latch leaves the budget armed");
        set_budget(None);
    }

    #[test]
    fn reduction_cap_is_lifted_only_under_a_budget_and_restored_by_set_budget() {
        set_budget(None);
        crate::deadline::set_deadline(None);
        assert!(reduction_cap_applies(), "on by default");
        set_reduction_cap(false);
        assert!(
            reduction_cap_applies(),
            "with no budget armed the cap cannot be lifted"
        );
        set_budget(Some(100));
        assert!(reduction_cap_applies(), "set_budget puts the cap back on");
        set_reduction_cap(false);
        assert!(!reduction_cap_applies(), "lifted under a budget");
        set_reduction_cap(true);
        assert!(reduction_cap_applies());
        set_reduction_cap(false);
        set_budget(None);
        assert!(reduction_cap_applies());
    }

    #[test]
    fn metered_means_a_finite_budget_or_an_armed_deadline() {
        set_budget(None);
        crate::deadline::set_deadline(None);
        assert!(!is_metered());
        set_budget(Some(1));
        assert!(is_metered());
        charge(5);
        assert!(is_metered(), "a spent budget is still a budget");
        set_budget(None);
        assert!(!is_metered());
        crate::deadline::set_deadline(Some(core::time::Duration::from_secs(3600)));
        assert!(is_metered());
        crate::deadline::set_deadline(None);
        assert!(!is_metered());
    }

    #[test]
    fn overshoot_saturates() {
        set_budget(Some(5));
        charge(100);
        assert!(is_exhausted());
        assert_eq!(used(), 100);
        set_budget(None);
    }

    #[test]
    fn nested_reductions_are_counted_against_the_limit_and_released_when_left() {
        assert_eq!(reduction_nesting_limit(), None, "no limit by default");
        set_reduction_nesting_limit(Some(2));
        assert_eq!(reduction_nesting_limit(), Some(2));
        let first = NestedReduction::enter();
        let second = NestedReduction::enter();
        assert!(first.is_some() && second.is_some());
        assert!(
            NestedReduction::enter().is_none(),
            "a third nested reduction is over a limit of two"
        );
        // A refused entry entered nothing.
        assert!(NestedReduction::enter().is_none());
        drop(second);
        let again = NestedReduction::enter();
        assert!(again.is_some(), "leaving one makes room for one");
        drop(again);
        drop(first);
        // A limit of zero admits nothing; no limit admits anything.
        set_reduction_nesting_limit(Some(0));
        assert!(NestedReduction::enter().is_none());
        set_reduction_nesting_limit(None);
        let many: Vec<Option<NestedReduction>> =
            (0..10_000).map(|_| NestedReduction::enter()).collect();
        assert!(many.iter().all(Option::is_some));
        drop(many);
        // Everything was left: a limit of one admits exactly one again.
        set_reduction_nesting_limit(Some(1));
        let only = NestedReduction::enter();
        assert!(only.is_some() && NestedReduction::enter().is_none());
        drop(only);
        set_reduction_nesting_limit(None);
    }

    #[test]
    fn a_nested_reduction_left_by_unwinding_is_not_counted_any_more() {
        set_reduction_nesting_limit(Some(1));
        let unwound = std::panic::catch_unwind(|| {
            let _nested = NestedReduction::enter();
            assert!(NestedReduction::enter().is_none());
            std::panic::resume_unwind(Box::new("unwinding out of a reduction"));
        });
        assert!(unwound.is_err());
        let nested = NestedReduction::enter();
        assert!(nested.is_some(), "the unwound reduction no longer counts");
        drop(nested);
        set_reduction_nesting_limit(None);
    }

    #[test]
    fn the_limits_are_the_threads_and_a_new_budget_leaves_them() {
        set_reduction_nesting_limit(Some(7));
        set_term_depth_limit(Some(9));
        set_budget(Some(100));
        set_budget(None);
        assert_eq!(reduction_nesting_limit(), Some(7));
        assert_eq!(term_depth_limit(), Some(9));
        // Another thread starts with none.
        let other = std::thread::spawn(|| (reduction_nesting_limit(), term_depth_limit()))
            .join()
            .unwrap_or_else(|_| panic!("the other thread panicked"));
        assert_eq!(other, (None, None));
        set_reduction_nesting_limit(None);
        set_term_depth_limit(None);
        assert_eq!(reduction_nesting_limit(), None);
        assert_eq!(term_depth_limit(), None);
    }

    #[cfg(feature = "term-depth")]
    #[test]
    fn a_term_over_the_depth_limit_spends_the_budget_and_is_named() {
        set_budget(Some(1_000));
        // No limit: no depth is over it.
        note_term_depth(u32::MAX);
        assert!(!term_depth_limit_hit() && !is_exhausted());
        set_term_depth_limit(Some(4));
        note_term_depth(4);
        assert!(!term_depth_limit_hit() && !is_exhausted(), "at the limit");
        note_term_depth(5);
        assert!(term_depth_limit_hit(), "over the limit");
        assert!(is_exhausted(), "which spends the budget");
        // The latch is the declaration's: a new budget clears it, and the
        // limit stays.
        set_budget(Some(1_000));
        assert!(!term_depth_limit_hit() && !is_exhausted());
        assert_eq!(term_depth_limit(), Some(4));
        // Running out of fuel is not this limit.
        charge(2_000);
        assert!(is_exhausted() && !term_depth_limit_hit());
        set_budget(None);
        set_term_depth_limit(None);
    }
}
