//! Differential + exhaustive tests for universe-level definitional equality.
//!
//! Ported from the level audit's reference-evaluator harness
//! (`docs/audit/2026-07-12-verify-gap/audit-levels.md`, §5/§8). The oracle is a
//! ground evaluator (`eval`) that computes a level's value under a concrete
//! assignment of the parameters {u, v, w} to naturals. Against it we assert, on
//! a finite space, the two properties the audit demanded:
//!
//! * SOUNDNESS  — the kernel never accepts a false fact:
//!   `is_equivalent  ==> forall-assignment equal`,
//!   `is_leq/is_geq  ==> forall-assignment ordered`,
//!   `normalize`     preserves semantics and is idempotent.
//! * COMPLETENESS — over the finite assignment space the kernel accepts *every*
//!   true fact: `forall-assignment equal ==> is_equivalent`, and likewise for
//!   `is_leq`. This is the property the pre-Wave-2 checker failed (it was sound
//!   but too strict); the case-split `leq` makes it hold here.
//!
//! Plus directed regressions for every divergence point in the audit's D1..D10
//! catalogue and the end-to-end `def Endo.{u}` repro through `check_declaration`.

#![allow(clippy::all)]

use oxilean_kernel::check::check_declaration;
use oxilean_kernel::level::{is_equivalent, is_geq, is_leq, mk_imax, mk_max, normalize};
use oxilean_kernel::Node;
use oxilean_kernel::{
    BinderInfo, Declaration, Environment, Expr, Level, LevelView, Name, ReducibilityHint,
};

use proptest::prelude::*;

// ── Reference evaluator (the oracle) ─────────────────────────────────────────

fn p(s: &str) -> Level {
    Level::param(Name::str(s))
}

/// Evaluate a level under a concrete assignment of u, v, w to naturals.
///
/// This is the ground semantics of universe levels (matches
/// `universe::level_to_nat`): `imax(a, b)` is `0` when `b` evaluates to `0`,
/// else `max(a, b)`.
fn eval(l: &Level, u: u64, v: u64, w: u64) -> u64 {
    match l.view() {
        LevelView::Zero => 0,
        LevelView::Succ(i) => eval(i, u, v, w) + 1,
        LevelView::Max(a, b) => eval(a, u, v, w).max(eval(b, u, v, w)),
        LevelView::IMax(a, b) => {
            let bb = eval(b, u, v, w);
            if bb == 0 {
                0
            } else {
                eval(a, u, v, w).max(bb)
            }
        }
        LevelView::Param(n) => match n.to_string().as_str() {
            "u" => u,
            "v" => v,
            _ => w,
        },
        // MVars do not occur in these tests.
        LevelView::MVar(_) => 0,
    }
}

/// The largest explicit numeral offset appearing anywhere in `l`.
fn max_const(l: &Level) -> u64 {
    match l.view() {
        LevelView::Zero => 0,
        LevelView::Succ(i) => max_const(i) + 1,
        LevelView::Max(a, b) | LevelView::IMax(a, b) => max_const(a).max(max_const(b)),
        LevelView::Param(_) | LevelView::MVar(_) => 0,
    }
}

fn collect_params(l: &Level, acc: &mut std::collections::BTreeSet<String>) {
    match l.view() {
        LevelView::Param(n) => {
            acc.insert(n.to_string());
        }
        LevelView::Succ(i) => collect_params(i, acc),
        LevelView::Max(a, b) | LevelView::IMax(a, b) => {
            collect_params(a, acc);
            collect_params(b, acc);
        }
        LevelView::Zero | LevelView::MVar(_) => {}
    }
}

/// Adaptive, *provably-sufficient* sampling bound for the universal relations
/// `l1 == l2` and `l1 <= l2` over ℕ^params.
///
/// `eval(l, ρ)` is a max-plus / imax expression: piecewise it is a `max` of
/// terms `param_i + c` (and `0`), with the impredicative `imax` introducing a
/// breakpoint at `param = 0`. The truth of `f R g` (`R` in {`==`, `<=`}) over
/// all of ℕ^k is decided by the sign of `g - f`, a piecewise-linear function
/// whose breakpoints lie where two constituent linear pieces cross, i.e. where
/// `param_i - param_j` equals a difference of the integer offsets (bounded by
/// `maxconst`) or where some `param` crosses `0`. A strict chain of such
/// constraints across `k` distinct params can force a param as high as
/// `maxconst + k`; sampling each param over `0..=(maxconst + k + 1)` therefore
/// visits every critical region **and** one step past the last breakpoint,
/// exposing any unbounded-param divergence. Hence the finite oracle is an exact
/// decision procedure and the completeness assertions are not ceiling artifacts.
fn oracle_bound(l1: &Level, l2: &Level) -> u64 {
    let mut params = std::collections::BTreeSet::new();
    collect_params(l1, &mut params);
    collect_params(l2, &mut params);
    max_const(l1).max(max_const(l2)) + params.len() as u64 + 1
}

/// True iff `l1` and `l2` evaluate equally under every assignment in
/// `0..=oracle_bound` (exact for the universal relation — see [`max_const`]).
fn sem_eq(l1: &Level, l2: &Level) -> bool {
    let hi = oracle_bound(l1, l2);
    for u in 0..=hi {
        for v in 0..=hi {
            for w in 0..=hi {
                if eval(l1, u, v, w) != eval(l2, u, v, w) {
                    return false;
                }
            }
        }
    }
    true
}

/// True iff `l1 >= l2` under every assignment in `0..=oracle_bound`.
fn sem_geq(l1: &Level, l2: &Level) -> bool {
    let hi = oracle_bound(l1, l2);
    for u in 0..=hi {
        for v in 0..=hi {
            for w in 0..=hi {
                if eval(l1, u, v, w) < eval(l2, u, v, w) {
                    return false;
                }
            }
        }
    }
    true
}

// ── Exhaustive small-term enumeration ────────────────────────────────────────

/// Enumerate all level terms up to `depth` over `{0, 1, u, v}` and the binary
/// constructors `max`/`imax` plus unary `succ`. Deduplicated by structure.
fn enumerate(depth: u32) -> Vec<Level> {
    let atoms: Vec<Level> = vec![Level::zero(), Level::succ(Level::zero()), p("u"), p("v")];
    let mut cur = atoms.clone();
    for _ in 0..depth {
        let prev = cur.clone();
        let mut next = prev.clone();
        for a in &prev {
            next.push(Level::succ(a.clone()));
        }
        for a in &prev {
            for b in &atoms {
                next.push(Level::max(a.clone(), b.clone()));
                next.push(Level::max(b.clone(), a.clone()));
                next.push(Level::imax(a.clone(), b.clone()));
                next.push(Level::imax(b.clone(), a.clone()));
            }
        }
        next.sort_by_key(|l| format!("{:?}", l));
        next.dedup();
        cur = next;
    }
    cur
}

#[test]
fn exhaustive_normalize_preserves_semantics_and_idempotent() {
    let all = enumerate(2);
    for l in &all {
        let n = normalize(l);
        assert!(
            sem_eq(l, &n),
            "normalize changed semantics: {:?} -> {:?}",
            l,
            n
        );
        let nn = normalize(&n);
        assert_eq!(n, nn, "normalize not idempotent on {:?}", l);
    }
}

#[test]
fn exhaustive_is_leq_sound_and_complete() {
    // Cap the pair count to keep the test fast; depth-2 enumeration is already
    // several hundred terms, and n^2 over a 350-term cap is ~120k pairs.
    let all = enumerate(2);
    let n = all.len().min(350);
    let mut unsound = 0usize;
    let mut incomplete = 0usize;
    let mut ex_unsound: Vec<String> = vec![];
    let mut ex_incomplete: Vec<String> = vec![];
    for i in 0..n {
        for j in 0..n {
            let (l1, l2) = (&all[i], &all[j]);
            let kernel = is_leq(l1, l2);
            let truth = sem_geq(l2, l1); // l1 <= l2 iff l2 >= l1
            if kernel && !truth {
                unsound += 1;
                if ex_unsound.len() < 8 {
                    ex_unsound.push(format!("{:?} <= {:?}", l1, l2));
                }
            }
            if !kernel && truth {
                incomplete += 1;
                if ex_incomplete.len() < 8 {
                    ex_incomplete.push(format!("{:?} <= {:?}", l1, l2));
                }
            }
        }
    }
    assert_eq!(unsound, 0, "is_leq UNSOUND on: {:#?}", ex_unsound);
    assert_eq!(
        incomplete, 0,
        "is_leq INCOMPLETE on {} pairs, e.g.: {:#?}",
        incomplete, ex_incomplete
    );
}

#[test]
fn exhaustive_is_equivalent_sound_and_complete() {
    let all = enumerate(2);
    let n = all.len().min(350);
    let mut unsound = 0usize;
    let mut incomplete = 0usize;
    let mut ex_unsound: Vec<String> = vec![];
    let mut ex_incomplete: Vec<String> = vec![];
    for i in 0..n {
        for j in 0..n {
            let (l1, l2) = (&all[i], &all[j]);
            let kernel = is_equivalent(l1, l2);
            let truth = sem_eq(l1, l2);
            if kernel && !truth {
                unsound += 1;
                if ex_unsound.len() < 8 {
                    ex_unsound.push(format!("{:?} ~ {:?}", l1, l2));
                }
            }
            if !kernel && truth {
                incomplete += 1;
                if ex_incomplete.len() < 8 {
                    ex_incomplete.push(format!("{:?} ~ {:?}", l1, l2));
                }
            }
        }
    }
    assert_eq!(unsound, 0, "is_equivalent UNSOUND on: {:#?}", ex_unsound);
    assert_eq!(
        incomplete, 0,
        "is_equivalent INCOMPLETE on {} pairs, e.g.: {:#?}",
        incomplete, ex_incomplete
    );
}

#[test]
fn is_equivalent_is_leq_both_ways_consistent() {
    let all = enumerate(2);
    let n = all.len().min(300);
    for i in 0..n {
        for j in 0..n {
            let (l1, l2) = (&all[i], &all[j]);
            let equiv = is_equivalent(l1, l2);
            let both = is_leq(l1, l2) && is_leq(l2, l1);
            assert_eq!(
                equiv, both,
                "is_equivalent inconsistent with leq-both-ways: {:?} ~ {:?}",
                l1, l2
            );
        }
    }
}

// ── Directed regressions: audit divergence catalogue D1..D10 ─────────────────

fn u() -> Level {
    p("u")
}
fn v() -> Level {
    p("v")
}
fn w() -> Level {
    p("w")
}
fn s(l: Level) -> Level {
    Level::succ(l)
}

/// D1 (CRITICAL): imax(u, u) = u.
#[test]
fn d1_imax_uu_equiv_u() {
    let l = Level::imax(u(), u());
    assert!(sem_eq(&l, &u()));
    assert!(is_equivalent(&l, &u()));
    assert_eq!(normalize(&l), normalize(&u()));
    assert_eq!(normalize(&l), u());
}

/// D2: max(0, u) = u.
#[test]
fn d2_max_zero_u_equiv_u() {
    let l = Level::max(Level::zero(), u());
    assert!(is_equivalent(&l, &u()));
    assert_eq!(normalize(&l), u());
}

/// D3: max(1, succ u) = succ u.
#[test]
fn d3_max_one_succ_u_equiv_succ_u() {
    let l = Level::max(s(Level::zero()), s(u()));
    let r = s(u());
    assert!(sem_eq(&l, &r));
    assert!(is_equivalent(&l, &r));
    assert_eq!(normalize(&l), normalize(&r));
}

/// D4: succ(max(u,v)) = max(succ u, succ v) (semantically true; must be equiv).
#[test]
fn d4_succ_max_distributes() {
    let l = s(Level::max(u(), v()));
    let r = Level::max(s(u()), s(v()));
    assert!(sem_eq(&l, &r));
    assert!(is_equivalent(&l, &r));
}

/// D5: imax(u, imax(v,w)) = max(imax(u,w), imax(v,w)).
#[test]
fn d5_imax_nested_imax() {
    let l = Level::imax(u(), Level::imax(v(), w()));
    let r = Level::max(Level::imax(u(), w()), Level::imax(v(), w()));
    assert!(sem_eq(&l, &r));
    assert!(is_equivalent(&l, &r));
}

/// D6: imax(u, max(v,w)) = max(imax(u,v), imax(u,w)).
#[test]
fn d6_imax_of_max() {
    let l = Level::imax(u(), Level::max(v(), w()));
    let r = Level::max(Level::imax(u(), v()), Level::imax(u(), w()));
    assert!(sem_eq(&l, &r));
    assert!(is_equivalent(&l, &r));
}

/// D7: imax(v, imax(v,u)) = imax(v,u) and imax(imax(u,v), v) = imax(u,v).
#[test]
fn d7_imax_absorption() {
    let a = Level::imax(v(), Level::imax(v(), u()));
    let b = Level::imax(v(), u());
    assert!(sem_eq(&a, &b));
    assert!(is_equivalent(&a, &b));

    let c = Level::imax(Level::imax(u(), v()), v());
    let d = Level::imax(u(), v());
    assert!(sem_eq(&c, &d));
    assert!(is_equivalent(&c, &d));
}

/// D8: is_geq(succ u, 1) and is_geq(u+2, 2).
#[test]
fn d8_geq_succ_ge_numeral() {
    assert!(sem_geq(&s(u()), &s(Level::zero())));
    assert!(is_geq(&s(u()), &s(Level::zero())));

    let up2 = s(s(u()));
    let two = s(s(Level::zero()));
    assert!(sem_geq(&up2, &two));
    assert!(is_geq(&up2, &two));
}

/// D9: is_geq(succ(imax(u,v)), succ v).
#[test]
fn d9_geq_succ_imax() {
    let l = s(Level::imax(u(), v()));
    let r = s(v());
    assert!(sem_geq(&l, &r));
    assert!(is_geq(&l, &r));
}

/// D10 companion: nested succ offsets and param-only comparisons.
#[test]
fn nested_succ_offsets_and_param_only() {
    // max(u, v) >= u, >= v, and >= imax(u, v).
    assert!(is_geq(&Level::max(u(), v()), &u()));
    assert!(is_geq(&Level::max(u(), v()), &v()));
    assert!(is_geq(&Level::max(u(), v()), &Level::imax(u(), v())));
    // param-only strictness: u is neither >= nor <= v (independent params).
    assert!(!is_geq(&u(), &v()));
    assert!(!is_leq(&u(), &v()));
    assert!(!is_equivalent(&u(), &v()));
    // deep succ offsets on a shared base.
    let deep = |n: u32| {
        let mut l = u();
        for _ in 0..n {
            l = s(l);
        }
        l
    };
    assert!(is_geq(&deep(5), &deep(2)));
    assert!(!is_geq(&deep(2), &deep(5)));
    assert!(is_leq(&deep(2), &deep(5)));
}

/// Directed regressions for the audit's nested-imax divergence points, checked
/// through both `is_equivalent` and the oracle.
#[test]
fn nested_imax_shapes() {
    // imax(u, imax(v, w)) — the D5 shape, plus its reflexive `<=`.
    let a = Level::imax(u(), Level::imax(v(), w()));
    assert!(is_leq(&a, &a) && is_geq(&a, &a));
    // imax(u, max(v, w)) — the D6 shape.
    let b = Level::imax(u(), Level::max(v(), w()));
    assert!(is_equivalent(
        &b,
        &Level::max(Level::imax(u(), v()), Level::imax(u(), w()))
    ));
    // Deeply left-nested imax over params must still be decided against itself.
    let c = Level::imax(Level::imax(Level::imax(u(), v()), w()), u());
    assert!(is_equivalent(&c, &c));
}

/// Termination / non-blowup: the parameter case-split substitutes `p := succ p`
/// (which grows the term); a chain of imax-with-param nodes must still terminate
/// promptly. This asserts the recursion is well-founded on an adversarial shape.
#[test]
fn param_case_split_terminates_on_deep_imax_chain() {
    // imax(u, imax(u, imax(u, ... imax(u, v))))  — 12 deep.
    let mut l = v();
    for _ in 0..12 {
        l = Level::imax(u(), l);
    }
    // Comparing against itself and against a fresh copy exercises the full
    // case-split machinery; the assertion is really that this returns at all.
    assert!(is_leq(&l, &l));
    let same = l.clone();
    assert!(is_equivalent(&l, &same));
    // And a genuinely different chain is handled too.
    let mut r = w();
    for _ in 0..12 {
        r = Level::imax(u(), r);
    }
    // l ends in v, r ends in w — not equivalent (differ when v != w).
    assert_eq!(is_equivalent(&l, &r), sem_eq(&l, &r));
}

// ── Pinned completeness regressions: param-dependent imax under a right max ──
//
// Both found by `prop_is_leq_complete`. The disjunctive max-right rule of
// `is_leq_core` used to commit to one disjunct for all assignments even when
// a param-dependent `imax` survived inside the max operands, where the
// correct disjunct depends on that param — the fix case-splits on a
// zero-ness-critical param first (`find_split_param` in `level/order.rs`).

/// `succ(max(max(0,v),0)) <= max(2, imax(succ v, imax(0, v)))`, i.e.
/// semantically `v+1 <= max(2, ...)`: needs the LEFT disjunct at `v = 0`
/// (`1 <= 2`) and the RIGHT one at `v >= 1` (`v+1 <= v+1`). The inner imax's
/// rhs is a bare `Param`.
#[test]
fn leq_complete_imax_bare_param_rhs_under_right_max() {
    let v = || Level::param(Name::str("v"));
    let l1 = Level::succ(Level::max(Level::max(Level::zero(), v()), Level::zero()));
    let l2 = Level::max(
        Level::succ(Level::succ(Level::zero())),
        Level::imax(Level::succ(v()), Level::imax(Level::zero(), v())),
    );
    assert!(sem_geq(&l2, &l1), "oracle: l1 <= l2 must hold pointwise");
    assert!(
        is_leq(&l1, &l2),
        "is_leq must decide succ v <= max(2, imax(succ v, imax(0, v)))"
    );
}

/// `succ u <= max(imax(succ u, max(u, u)), 2)`: the blocked imax's rhs is a
/// `Max` of params (NOT a bare param), so the split trigger must follow the
/// zero-ness decision path (`Max` both operands, `IMax` rhs), not just look
/// for bare-`Param` rhses.
#[test]
fn leq_complete_imax_max_param_rhs_under_right_max() {
    let u = || Level::param(Name::str("u"));
    let l1 = Level::succ(u());
    let l2 = Level::max(
        Level::imax(Level::succ(u()), Level::max(u(), u())),
        Level::succ(Level::succ(Level::zero())),
    );
    assert!(sem_geq(&l2, &l1), "oracle: l1 <= l2 must hold pointwise");
    assert!(
        is_leq(&l1, &l2),
        "is_leq must decide succ u <= max(imax(succ u, max(u, u)), 2)"
    );
}

// ── Smart constructors ───────────────────────────────────────────────────────

#[test]
fn mk_imax_matches_lean_rules() {
    // imax(_, 0) = 0
    assert_eq!(mk_imax(u(), Level::zero()), Level::zero());
    assert_eq!(mk_imax(Level::zero(), Level::zero()), Level::zero());
    // imax(0, v) = v
    assert_eq!(mk_imax(Level::zero(), v()), v());
    // imax(u, u) = u
    assert_eq!(mk_imax(u(), u()), u());
    // imax(u, succ v) = max(u, succ v) (rhs provably non-zero)
    assert!(is_equivalent(
        &mk_imax(u(), s(v())),
        &Level::max(u(), s(v()))
    ));
    // stuck: imax(u, v) stays imax
    assert_eq!(mk_imax(u(), v()), Level::imax(u(), v()));
}

#[test]
fn mk_max_matches_lean_rules() {
    assert_eq!(mk_max(u(), u()), u());
    assert_eq!(mk_max(Level::zero(), u()), u());
    assert_eq!(mk_max(u(), Level::zero()), u());
    // numeral subsumption
    assert_eq!(
        mk_max(s(Level::zero()), s(s(Level::zero()))),
        s(s(Level::zero()))
    );
    assert_eq!(
        mk_max(s(s(Level::zero())), s(Level::zero())),
        s(s(Level::zero()))
    );
    // stuck
    assert_eq!(mk_max(u(), v()), Level::max(u(), v()));
}

#[test]
fn mk_constructors_preserve_semantics() {
    let all = enumerate(1);
    for a in &all {
        for b in &all {
            let m = mk_max(a.clone(), b.clone());
            assert!(
                sem_eq(&m, &Level::max(a.clone(), b.clone())),
                "mk_max changed semantics: {:?} {:?}",
                a,
                b
            );
            let im = mk_imax(a.clone(), b.clone());
            assert!(
                sem_eq(&im, &Level::imax(a.clone(), b.clone())),
                "mk_imax changed semantics: {:?} {:?}",
                a,
                b
            );
        }
    }
}

// ── End-to-end: def Endo.{u} : Sort u -> Sort u := fun a => a -> a ────────────

#[test]
fn endo_typechecks_end_to_end() {
    let ul = Level::param(Name::str("u"));
    let sort_u = Expr::Sort(ul.clone());
    // declared type: (a : Sort u) -> Sort u
    let declared_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("a"),
        Node::new(sort_u.clone()),
        Node::new(sort_u.clone()),
    );
    // value: fun (a : Sort u) => (_ : a) -> a
    let val = Expr::Lam(
        BinderInfo::Default,
        Name::str("a"),
        Node::new(sort_u.clone()),
        Node::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Node::new(Expr::BVar(0)),
            Node::new(Expr::BVar(1)),
        )),
    );
    let decl = Declaration::Definition {
        name: Name::str("Endo"),
        univ_params: vec![Name::str("u")],
        ty: declared_ty,
        val,
        hint: ReducibilityHint::Regular(1),
    };
    let mut env = Environment::new();
    check_declaration(&mut env, decl)
        .expect("def Endo.{u} : Sort u -> Sort u := fun a => a -> a must typecheck");
    assert!(env.contains(&Name::str("Endo")));
}

// ── Proptests: soundness on random levels of larger depth ────────────────────

fn arb_param() -> impl Strategy<Value = Name> {
    prop_oneof![
        Just(Name::str("u")),
        Just(Name::str("v")),
        Just(Name::str("w")),
    ]
}

fn arb_level_impl(depth: u32) -> BoxedStrategy<Level> {
    if depth == 0 {
        prop_oneof![
            Just(Level::zero()),
            Just(Level::succ(Level::zero())),
            arb_param().prop_map(Level::param),
        ]
        .boxed()
    } else {
        prop_oneof![
            Just(Level::zero()),
            arb_param().prop_map(Level::param),
            arb_level_impl(depth - 1).prop_map(|l| Level::succ(l)),
            (arb_level_impl(depth - 1), arb_level_impl(depth - 1))
                .prop_map(|(a, b)| Level::max(a, b)),
            (arb_level_impl(depth - 1), arb_level_impl(depth - 1))
                .prop_map(|(a, b)| Level::imax(a, b)),
        ]
        .boxed()
    }
}

fn arb_level() -> impl Strategy<Value = Level> {
    arb_level_impl(3)
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(2000))]

    /// normalize preserves semantics under every assignment in 0..=HI.
    #[test]
    fn prop_normalize_preserves_semantics(l in arb_level()) {
        let n = normalize(&l);
        prop_assert!(sem_eq(&l, &n), "normalize changed semantics: {:?} -> {:?}", l, n);
    }

    /// normalize is idempotent.
    #[test]
    fn prop_normalize_idempotent(l in arb_level()) {
        let n = normalize(&l);
        prop_assert_eq!(normalize(&n), n.clone(), "normalize not idempotent on {:?}", l);
    }

    /// is_leq is SOUND: if the kernel says l1 <= l2, the oracle agrees.
    #[test]
    fn prop_is_leq_sound(l1 in arb_level(), l2 in arb_level()) {
        if is_leq(&l1, &l2) {
            prop_assert!(sem_geq(&l2, &l1),
                "is_leq unsound: {:?} <= {:?}", l1, l2);
        }
    }

    /// is_leq is COMPLETE on this space: if the oracle says l1 <= l2, so does
    /// the kernel.
    #[test]
    fn prop_is_leq_complete(l1 in arb_level(), l2 in arb_level()) {
        if sem_geq(&l2, &l1) {
            prop_assert!(is_leq(&l1, &l2),
                "is_leq incomplete: {:?} <= {:?}", l1, l2);
        }
    }

    /// is_equivalent agrees with the oracle in both directions.
    #[test]
    fn prop_is_equivalent_iff_sem_eq(l1 in arb_level(), l2 in arb_level()) {
        prop_assert_eq!(is_equivalent(&l1, &l2), sem_eq(&l1, &l2),
            "is_equivalent disagrees with oracle: {:?} ~ {:?}", l1, l2);
    }

    /// is_equivalent is exactly leq-both-ways.
    #[test]
    fn prop_is_equivalent_consistent(l1 in arb_level(), l2 in arb_level()) {
        let both = is_leq(&l1, &l2) && is_leq(&l2, &l1);
        prop_assert_eq!(is_equivalent(&l1, &l2), both);
    }
}
