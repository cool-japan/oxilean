//! Universe-level definitional equality: smart constructors, complete `leq`.
//!
//! This module implements Lean 4's kernel semantics for universe levels
//! (`src/kernel/level.cpp`, `Lean/Level.lean`):
//!
//! * [`mk_max`] / [`mk_imax`] — simplifying smart constructors matching Lean's
//!   `mkLevelMax'` / `mkLevelIMax'` (used by [`crate::level::normalize`] and by
//!   the type checker's Pi inference so re-inferred sorts land in the same
//!   canonical shape the elaborator stored in the export).
//! * `is_leq_core` — the *complete* `leq` decision procedure with the
//!   parameter case-split (`p := 0` and `p := succ p'`) used by
//!   trepplein / nanoda / lean4lean, so `is_geq`/`is_leq`/`is_equivalent` are
//!   sound **and** complete over the finite assignment space.
//!
//! Kernel correctness principle: every rule here is either a Lean-kernel rewrite
//! or a semantics-preserving simplification; when stuck we conservatively fall
//! through to `false` (for `leq`) / keep the node (for the constructors), never
//! producing a wrong answer.

use super::types::{Level, LevelView};
use crate::{Name, NameView};

/// Structural total order on [`Name`], replacing the previous
/// `Name::to_string()` comparison.
///
/// The string comparison was not a well-defined total order: two structurally
/// distinct hierarchical names can render to the same string (e.g.
/// `Str(Str(_, "a"), "b")` vs `Str(_, "a.b")`), which could make `sort_by`
/// observe `Greater` in both directions and panic ("comparison function does
/// not correctly implement a total order"). Comparing the tree structure
/// directly is a genuine total order and avoids per-comparison allocation.
pub(super) fn name_cmp(a: &Name, b: &Name) -> std::cmp::Ordering {
    use std::cmp::Ordering;
    fn tag(n: &Name) -> u8 {
        match n.view() {
            NameView::Anonymous => 0,
            NameView::Str(_, _) => 1,
            NameView::Num(_, _) => 2,
        }
    }
    match (a.view(), b.view()) {
        (NameView::Anonymous, NameView::Anonymous) => Ordering::Equal,
        (NameView::Str(p1, s1), NameView::Str(p2, s2)) => name_cmp(p1, p2).then_with(|| s1.cmp(s2)),
        (NameView::Num(p1, n1), NameView::Num(p2, n2)) => {
            name_cmp(p1, p2).then_with(|| n1.cmp(&n2))
        }
        _ => tag(a).cmp(&tag(b)),
    }
}

/// Simplifying `max` smart constructor (Lean `mkLevelMax'` / kernel `mk_max`).
///
/// Applies the algebraically-sound simplifications Lean's kernel performs when
/// building a `max`:
/// * `max(l, l) = l`
/// * `max(0, l) = l`, `max(l, 0) = l`
/// * numeral subsumption between the two immediate arguments:
///   `max(a, b) = a` when both are numerals and `a >= b` (and symmetrically).
///
/// Deeper canonicalisation (flatten / sort / dedup across a whole `max` tree)
/// is left to [`crate::level::normalize`]; this constructor only performs the
/// local rewrites, so it is cheap enough to call from type inference.
pub fn mk_max(l1: Level, l2: Level) -> Level {
    if l1 == l2 {
        return l1;
    }
    if l1.is_zero() {
        return l2;
    }
    if l2.is_zero() {
        return l1;
    }
    // Numeral subsumption on the two immediate arguments.
    if let (Some(n1), Some(n2)) = (l1.to_nat(), l2.to_nat()) {
        return if n1 >= n2 { l1 } else { l2 };
    }
    Level::max(l1, l2)
}

/// Simplifying `imax` smart constructor (Lean `mkLevelIMax'` / kernel `mk_imax`).
///
/// Rules (matching Lean exactly):
/// * `imax(_, 0) = 0`      (impredicativity: a Pi into `Prop` is a `Prop`)
/// * `imax(0, l) = l`
/// * `imax(l, l) = l`
/// * `imax(l1, l2) = max(l1, l2)` when `l2` is provably non-zero
///   (`is_not_zero`), routed through [`mk_max`] to pick up its simplifications.
///
/// Note the ordering: `imax(_, 0) = 0` fires *before* the `l1 == l2` rule, so
/// `imax(0, 0)` correctly reduces to `0` (via the zero-rhs rule), matching
/// Lean.
pub fn mk_imax(l1: Level, l2: Level) -> Level {
    if l2.is_zero() {
        // imax(_, 0) = 0.
        return Level::zero();
    }
    if l2.is_not_zero() {
        // imax(l1, l2) = max(l1, l2) when l2 is provably >= 1.
        return mk_max(l1, l2);
    }
    if l1.is_zero() {
        // imax(0, l2) = l2.
        return l2;
    }
    if l1 == l2 {
        // imax(l, l) = l.
        return l1;
    }
    Level::imax(l1, l2)
}

/// Decompose `l` into `(base, offset)` with `l = succ^offset(base)`.
fn to_offset(l: &Level) -> (&Level, u32) {
    match l.view() {
        LevelView::Succ(inner) => {
            let (base, k) = to_offset(inner);
            (base, k + 1)
        }
        _ => (l, 0),
    }
}

/// Complete `l1 <= l2` decision procedure with parameter case-split.
///
/// This is the standard trepplein / nanoda / lean4lean `leq` algorithm. It is
/// **sound and complete**: `is_leq_core(a, b, 0)` returns `true` iff
/// `eval(a, ρ) <= eval(b, ρ)` for *every* assignment `ρ` of the parameters to
/// naturals.
///
/// `diff` is a running successor balance: conceptually we are deciding
/// `l1 <= l2 + diff`. Peeling a `succ` off `l1` decrements `diff`; peeling one
/// off `l2` increments it. When we reach a leaf, the relation is decided by the
/// leaf together with `diff`.
///
/// The interesting cases are the `imax` ones whose *second* argument is a
/// `Param p`: `imax(a, p)` is `0` when `p = 0` and `max(a, p)` when `p >= 1`, so
/// we cannot decide it uniformly. We case-split on `p`: substitute `p := 0`
/// (which forces the imax to `0`) and `p := succ p` (which forces it to a
/// `max`), and require the relation in *both* worlds. This is exactly Lean's
/// `Level.isEquiv`/`leq` behaviour.
pub(super) fn is_leq_core(l1: &Level, l2: &Level, diff: i64) -> bool {
    // The `imax` param case-split below branches, so a deeply nested
    // param-dependent universe constraint (Mathlib's `CategoryTheory` corpus
    // has them) can make this recursion blow up exponentially WITHOUT
    // constructing any `Expr`, so the deterministic fuel never catches it and
    // the declaration hangs. The wall-clock deadline is the backstop: once it
    // latches, return the conservative `false` ("cannot prove `l1 <= l2`"),
    // which unwinds the recursion and lets the caller report the declaration as
    // a resource limit — never a wrong accept (a spurious `false` only ever
    // fails an equality/validity check, moving the decl to `unsupported`).
    if crate::deadline::is_expired() {
        return false;
    }
    // Cheap structural short-circuit: identical levels differ only by `diff`.
    if l1 == l2 {
        return diff >= 0;
    }
    // The case ordering below is exactly nanoda / trepplein / lean4lean's
    // `leq_core`. The order is load-bearing: `succ` and left-`max` are stripped
    // first, then the right-`max` disjunction fires ONLY for a `Param`/`Zero`
    // left side, and finally the `imax` param case-split / distribution.
    match (l1.view(), l2.view()) {
        // Decisive zero/param cases (guarded so they only fire when they settle
        // the question; otherwise we fall through to succ-stripping so the other
        // side's offset is accounted for).
        //
        // `0 <= l2 + diff` holds for all assignments when `diff >= 0`
        // (since `l2 >= 0`); when `diff < 0` it depends on `l2`, so fall through.
        (LevelView::Zero, _) if diff >= 0 => true,
        // `l1 <= 0 + diff` with `diff < 0` is impossible (`l1 >= 0`); fall
        // through when `diff >= 0`.
        (_, LevelView::Zero) if diff < 0 => false,
        // Two params: comparable only if identical, and then only if `diff >= 0`.
        (LevelView::Param(a), LevelView::Param(b)) => a == b && diff >= 0,
        // `param <= 0 + diff` is impossible when the param could be arbitrarily
        // large (guard `diff < 0` handled above; here `diff >= 0` but a param can
        // exceed any fixed bound) — so `false`.
        (LevelView::Param(_), LevelView::Zero) => false,
        // `0 <= param + diff`: with `diff >= 0` already returned true above;
        // reaching here means `diff < 0`, and a param can be `0`, so `false`.
        (LevelView::Zero, LevelView::Param(_)) => false,
        // Successor on either side: adjust the balance.
        (LevelView::Succ(a), _) => is_leq_core(a, l2, diff - 1),
        (_, LevelView::Succ(b)) => is_leq_core(l1, b, diff + 1),
        // max on the left: max(a,b) <= l2 iff a <= l2 and b <= l2.
        (LevelView::Max(a, b), _) => is_leq_core(a, l2, diff) && is_leq_core(b, l2, diff),
        // max on the right — ONLY when the left side is a Param, Zero or MVar
        // leaf. (Max-left is handled above; an imax-left must NOT take this
        // disjunctive shortcut, which is incomplete for it, and falls through to
        // the imax arms below.)
        //
        // COMPLETENESS GUARD: the disjunction `l1 <= a or l1 <= b` commits to
        // one disjunct for *all* parameter assignments, but if a
        // param-dependent `imax` survives inside the max operands, the
        // *correct* disjunct can depend on that param (e.g. `succ v <= max(2,
        // imax(succ v, v))` needs the left disjunct at `v = 0` and the right
        // one at `v >= 1`). Case-split on such a param FIRST; once both sides
        // are semantically imax-free, the disjunction is complete for an
        // atomic left side.
        (LevelView::Param(_) | LevelView::Zero | LevelView::MVar(_), LevelView::Max(a, b)) => {
            if let Some(name) = find_split_param(l2) {
                let param = Level::param(name.clone());
                imax_param_split(l1, l2, &param, diff)
            } else {
                is_leq_core(l1, a, diff) || is_leq_core(l1, b, diff)
            }
        }
        // imax whose rhs is statically decidable: collapse it. `imax(_, 0) = 0`
        // and `imax(a, b) = max(a, b)` when `b` is provably non-zero. This is
        // what keeps the `p := succ p` branch of the case-split terminating:
        // after substitution the imax rhs becomes `succ p` (non-zero), which
        // this arm rewrites to a `max`, removing the imax.
        (LevelView::IMax(a, b), _) => {
            let simplified = simplify_imax(a, b);
            match simplified {
                Some(s) => is_leq_core(&s, l2, diff),
                None => imax_param_split(l1, l2, b, diff),
            }
        }
        (_, LevelView::IMax(a, b)) => {
            let simplified = simplify_imax(a, b);
            match simplified {
                Some(s) => is_leq_core(l1, &s, diff),
                None => imax_param_split(l1, l2, b, diff),
            }
        }
        // Remaining leaf/leaf cases (params vs params/mvars): fall back to
        // base+offset comparison. Equal bases compare by offset; distinct bases
        // are incomparable (return false — sound: an unconstrained param could
        // exceed any other).
        _ => {
            let (b1, k1) = to_offset(l1);
            let (b2, k2) = to_offset(l2);
            if b1 == b2 {
                // succ^k1(b) <= succ^k2(b) + diff iff k1 <= k2 + diff.
                (k1 as i64) <= (k2 as i64) + diff
            } else {
                false
            }
        }
    }
}

/// Find a parameter to case-split on so that every *param-dependent* `imax`
/// in `l` can eventually be eliminated.
///
/// An `imax(a, b)` is **blocked** when the zero-ness of `b` is syntactically
/// undecided (`!b.is_zero() && !b.is_not_zero()`): the imax is `0` when `b`
/// evaluates to `0` and `max(a, b)` otherwise, so no uniform rewrite exists.
/// For a blocked imax we return a parameter on the *zero-ness decision path*
/// of `b` (see [`first_crit_param`]): substituting `p := succ p` then makes
/// `b.is_not_zero()` true (unblocking the imax), and `p := 0` removes `p`
/// from the term entirely — so the case-split in [`imax_param_split`]
/// terminates (lexicographic measure: distinct params, then blocked imaxes).
///
/// A blocked imax whose rhs has *no* parameter on the decision path is
/// semantically always-zero (every decision-path leaf is `Zero`; a `Succ`
/// leaf would have made `is_not_zero` true), hence constant — it cannot make
/// a disjunct choice assignment-dependent and needs no split.
fn find_split_param(l: &Level) -> Option<&Name> {
    match l.view() {
        LevelView::Zero | LevelView::Param(_) | LevelView::MVar(_) => None,
        LevelView::Succ(a) => find_split_param(a),
        LevelView::Max(a, b) => find_split_param(a).or_else(|| find_split_param(b)),
        LevelView::IMax(a, b) => {
            if !b.is_zero() && !b.is_not_zero() {
                if let Some(n) = first_crit_param(b) {
                    return Some(n);
                }
            }
            find_split_param(a).or_else(|| find_split_param(b))
        }
    }
}

/// First parameter whose value can decide the zero-ness of `l`.
///
/// Mirrors the recursion of [`Level::is_not_zero`]: `max` is zero iff both
/// operands are, and `imax` is zero iff its *rhs* is — so the decision paths
/// follow `Max` into both operands and `IMax` into the rhs only. A `Param`
/// leaf on such a path controls the outcome; `Succ` (never zero) and `Zero`
/// leaves are constants.
fn first_crit_param(l: &Level) -> Option<&Name> {
    match l.view() {
        LevelView::Param(n) => Some(n),
        LevelView::Zero | LevelView::Succ(_) | LevelView::MVar(_) => None,
        LevelView::Max(a, b) => first_crit_param(a).or_else(|| first_crit_param(b)),
        LevelView::IMax(_, b) => first_crit_param(b),
    }
}

/// Perform the `imax`-rhs-is-`Param` case-split for `l1 <= l2` at balance
/// `diff`. `param` is the boxed `Param` (the imax's second argument).
fn imax_param_split(l1: &Level, l2: &Level, param: &Level, diff: i64) -> bool {
    let name = match param.view() {
        LevelView::Param(n) => n.clone(),
        _ => return false,
    };
    // p := 0 collapses the imax(s) to their impredicative-zero value.
    let zero_l1 = subst_param(l1, &name, &Level::zero());
    let zero_l2 = subst_param(l2, &name, &Level::zero());
    // p := succ p forces the imax(s) into `max` form.
    let succ = Level::succ(Level::param(name.clone()));
    let succ_l1 = subst_param(l1, &name, &succ);
    let succ_l2 = subst_param(l2, &name, &succ);
    is_leq_core(&zero_l1, &zero_l2, diff) && is_leq_core(&succ_l1, &succ_l2, diff)
}

/// Statically simplify `imax(a, b)` when its behaviour is determined without a
/// case-split, returning an equivalent level with the outer `imax` removed.
///
/// Returns `None` exactly when the rhs is a bare `Param` (which requires the
/// case-split) or a bare `MVar` (which the kernel path never sees, and which we
/// leave to the conservative base+offset fallback).
///
/// * `b == 0`            -> `0`                              (impredicativity)
/// * `b` provably `!= 0` -> `max(a, b)`                      (imax collapses)
/// * `b = max(c, d)`     -> `max(imax(a, c), imax(a, d))`    (distribute)
/// * `b = imax(c, d)`    -> `max(imax(a, d), imax(c, d))`    (distribute; equal
///   because both sides are `0` when `d = 0` and `max(a, max(c, d))` when
///   `d != 0`)
fn simplify_imax(a: &Level, b: &Level) -> Option<Level> {
    if b.is_zero() {
        return Some(Level::zero());
    }
    if b.is_not_zero() {
        return Some(Level::max(a.clone(), b.clone()));
    }
    match b.view() {
        LevelView::Max(c, d) => Some(Level::max(
            Level::imax(a.clone(), c.clone()),
            Level::imax(a.clone(), d.clone()),
        )),
        LevelView::IMax(c, d) => Some(Level::max(
            Level::imax(a.clone(), d.clone()),
            Level::imax(c.clone(), d.clone()),
        )),
        // Bare Param -> needs case-split; bare MVar -> conservative fallback.
        _ => None,
    }
}

/// Substitute `Param(name)` with `value` everywhere in `l`.
fn subst_param(l: &Level, name: &Name, value: &Level) -> Level {
    match l.view() {
        LevelView::Param(n) if n == name => value.clone(),
        LevelView::Param(_) | LevelView::Zero | LevelView::MVar(_) => l.clone(),
        LevelView::Succ(inner) => Level::succ(subst_param(inner, name, value)),
        LevelView::Max(a, b) => {
            Level::max(subst_param(a, name, value), subst_param(b, name, value))
        }
        LevelView::IMax(a, b) => {
            Level::imax(subst_param(a, name, value), subst_param(b, name, value))
        }
    }
}
