//! Property-based tests for the OxiLean kernel.
//!
//! These tests verify core invariants of the kernel using `proptest`:
//!
//! 1. **Substitution identity**: `instantiate(abstract_expr(e, fvar), fvar_expr) == e`
//!    for expressions that contain no loose bound variables.
//! 2. **Lift/lower bvars**: `shift_down(lift_bvars(e, n), n) == e` for closed expressions.
//! 3. **Level normalization idempotency**: `normalize(normalize(l)) == normalize(l)`.
//! 4. **WHNF idempotency**: `whnf(whnf(e)) == whnf(e)`.
//! 5. **Definitional equality reflexivity**: `def_eq(e, e)` is always true.
//! 6. **Name round-trip**: Display then re-parse yields the same `Name`.

#![allow(clippy::all)]

use oxilean_kernel::instantiate::has_loose_bvars;
use oxilean_kernel::level::normalize as normalize_level;
use oxilean_kernel::{
    abstract_expr, instantiate, is_def_eq_simple, whnf, BinderInfo, Expr, FVarId, Level, Literal,
    Name,
};

use proptest::prelude::*;

// ── Utility: shift BVars down (inverse of lift_bvars) ────────────────────────

/// Shift all free (loose) bound-variable indices down by `n`.
///
/// This is the inverse of `lift_bvars(expr, n)` and is only defined for
/// expressions where every loose BVar index is >= n.
fn lower_bvars(expr: &Expr, n: u32) -> Expr {
    lower_bvars_at(expr, n, 0)
}

fn lower_bvars_at(expr: &Expr, n: u32, depth: u32) -> Expr {
    match expr {
        Expr::BVar(i) => {
            if *i >= depth {
                // This is a loose bvar; shift it down by n.
                Expr::BVar(i.saturating_sub(n))
            } else {
                expr.clone()
            }
        }
        Expr::Sort(_) | Expr::FVar(_) | Expr::Const(_, _) | Expr::Lit(_) => expr.clone(),
        Expr::App(f, a) => Expr::App(
            Box::new(lower_bvars_at(f, n, depth)),
            Box::new(lower_bvars_at(a, n, depth)),
        ),
        Expr::Lam(bi, name, dom, body) => Expr::Lam(
            *bi,
            name.clone(),
            Box::new(lower_bvars_at(dom, n, depth)),
            Box::new(lower_bvars_at(body, n, depth + 1)),
        ),
        Expr::Pi(bi, name, dom, body) => Expr::Pi(
            *bi,
            name.clone(),
            Box::new(lower_bvars_at(dom, n, depth)),
            Box::new(lower_bvars_at(body, n, depth + 1)),
        ),
        Expr::Let(name, ty, val, body) => Expr::Let(
            name.clone(),
            Box::new(lower_bvars_at(ty, n, depth)),
            Box::new(lower_bvars_at(val, n, depth)),
            Box::new(lower_bvars_at(body, n, depth + 1)),
        ),
        Expr::Proj(name, idx, e) => {
            Expr::Proj(name.clone(), *idx, Box::new(lower_bvars_at(e, n, depth)))
        }
    }
}

// ── Arbitrary strategies ──────────────────────────────────────────────────────

/// Generate a simple, valid `Name` (string components only, no numeric-only root).
fn arb_name() -> impl Strategy<Value = Name> {
    prop_oneof![
        Just(Name::str("x")),
        Just(Name::str("y")),
        Just(Name::str("z")),
        Just(Name::str("Nat")),
        Just(Name::str("Bool")),
        Just(Name::str("Nat").append_str("succ")),
        Just(Name::str("Nat").append_str("zero")),
        Just(Name::str("List").append_str("nil")),
        Just(Name::str("f")),
        Just(Name::str("g")),
    ]
}

/// Generate arbitrary `Name` values including numeric components (for round-trip tests).
///
/// Note: `Name::Anonymous` is excluded because its `Display` representation is `"_"`,
/// but `Name::from_str("_")` produces `Name::Str(Anonymous, "_")` rather than
/// `Name::Anonymous` — the round-trip is not defined for the anonymous root.
fn arb_name_for_roundtrip() -> impl Strategy<Value = Name> {
    prop_oneof![
        // pure string names — must not start with digit (to avoid ambiguity with Num)
        "[a-z][a-z0-9]{0,4}".prop_map(|s| Name::str(s)),
        // two-component string names
        ("[a-z][a-z0-9]{0,3}", "[a-z][a-z0-9]{0,3}")
            .prop_map(|(a, b)| { Name::str(a).append_str(b) }),
        // string + numeric suffix — from_str parses trailing numbers as Num components
        ("[a-z][a-z0-9]{0,3}", 0u64..100u64).prop_map(|(s, n)| { Name::str(s).append_num(n) }),
    ]
}

/// Generate a `Level` up to bounded depth (no MVars — those complicate normalization).
fn arb_level(max_depth: u32) -> impl Strategy<Value = Level> {
    arb_level_impl(max_depth)
}

fn arb_level_impl(depth: u32) -> impl Strategy<Value = Level> {
    if depth == 0 {
        prop_oneof![Just(Level::Zero), arb_name().prop_map(Level::Param),].boxed()
    } else {
        prop_oneof![
            Just(Level::Zero),
            arb_name().prop_map(Level::Param),
            // Succ
            arb_level_impl(depth - 1).prop_map(|l| Level::Succ(Box::new(l))),
            // Max
            (arb_level_impl(depth - 1), arb_level_impl(depth - 1))
                .prop_map(|(l1, l2)| Level::Max(Box::new(l1), Box::new(l2))),
            // IMax
            (arb_level_impl(depth - 1), arb_level_impl(depth - 1))
                .prop_map(|(l1, l2)| Level::IMax(Box::new(l1), Box::new(l2))),
        ]
        .boxed()
    }
}

/// Generate a well-formed, **closed** `Expr` (no loose BVars, no FVars) up to
/// the given tree depth.  These are safe to use in all substitution and WHNF
/// tests without pre-condition filtering.
fn arb_closed_expr(max_depth: u32) -> impl Strategy<Value = Expr> {
    arb_closed_expr_impl(max_depth, 0)
}

fn arb_closed_expr_impl(depth: u32, binder_depth: u32) -> impl Strategy<Value = Expr> {
    if depth == 0 {
        // Atoms only — safe regardless of binder depth.
        prop_oneof![
            arb_level(2).prop_map(Expr::Sort),
            arb_name().prop_map(|n| Expr::Const(n, vec![])),
            (0u64..=1000u64).prop_map(|n| Expr::Lit(Literal::Nat(n))),
        ]
        .boxed()
    } else {
        prop_oneof![
            // Atomic expressions
            arb_level(2).prop_map(Expr::Sort),
            arb_name().prop_map(|n| Expr::Const(n, vec![])),
            (0u64..=1000u64).prop_map(|n| Expr::Lit(Literal::Nat(n))),
            // Application: f applied to a (both closed)
            (
                arb_closed_expr_impl(depth - 1, binder_depth),
                arb_closed_expr_impl(depth - 1, binder_depth)
            )
                .prop_map(|(f, a)| Expr::App(Box::new(f), Box::new(a))),
            // Lambda: Lam binder — body has binder_depth + 1, but we generate
            // body as a closed expression (BVar(0) inside = bound by this lam).
            // We skip BVar inside to keep it truly closed externally.
            (
                arb_name(),
                arb_closed_expr_impl(depth - 1, binder_depth),
                arb_closed_expr_impl(depth - 1, binder_depth + 1)
            )
                .prop_map(|(name, ty, body)| {
                    Expr::Lam(BinderInfo::Default, name, Box::new(ty), Box::new(body))
                }),
            // Pi: same pattern
            (
                arb_name(),
                arb_closed_expr_impl(depth - 1, binder_depth),
                arb_closed_expr_impl(depth - 1, binder_depth + 1)
            )
                .prop_map(|(name, ty, body)| {
                    Expr::Pi(BinderInfo::Default, name, Box::new(ty), Box::new(body))
                }),
        ]
        .boxed()
    }
}

/// Generate an `Expr` that may contain the specific `FVarId(999)` as a free
/// variable but has no loose BVars.  Used for the substitution identity test.
fn arb_fvar_expr(max_depth: u32, fvar: FVarId) -> impl Strategy<Value = Expr> {
    arb_fvar_expr_impl(max_depth, fvar)
}

fn arb_fvar_expr_impl(depth: u32, fvar: FVarId) -> impl Strategy<Value = Expr> {
    if depth == 0 {
        prop_oneof![
            arb_level(2).prop_map(Expr::Sort),
            arb_name().prop_map(|n| Expr::Const(n, vec![])),
            (0u64..=100u64).prop_map(|n| Expr::Lit(Literal::Nat(n))),
            // The specific FVar we care about
            Just(Expr::FVar(fvar)),
        ]
        .boxed()
    } else {
        prop_oneof![
            arb_level(2).prop_map(Expr::Sort),
            arb_name().prop_map(|n| Expr::Const(n, vec![])),
            (0u64..=100u64).prop_map(|n| Expr::Lit(Literal::Nat(n))),
            Just(Expr::FVar(fvar)),
            // App
            (
                arb_fvar_expr_impl(depth - 1, fvar),
                arb_fvar_expr_impl(depth - 1, fvar)
            )
                .prop_map(|(f, a)| Expr::App(Box::new(f), Box::new(a))),
            // Lam — generate body without embedding fvar inside the binder body,
            // because abstract_expr would shift BVars inside, breaking the
            // identity for expressions that contain BVar(0) bound by THIS lambda.
            // We only put fvar in the *type* position.
            (
                arb_name(),
                arb_fvar_expr_impl(depth - 1, fvar),
                // body uses atoms only so no loose bvars are introduced
                arb_level(1).prop_map(Expr::Sort),
            )
                .prop_map(|(name, ty, body)| {
                    Expr::Lam(BinderInfo::Default, name, Box::new(ty), Box::new(body))
                }),
            // Pi — same constraint
            (
                arb_name(),
                arb_fvar_expr_impl(depth - 1, fvar),
                arb_level(1).prop_map(Expr::Sort),
            )
                .prop_map(|(name, ty, body)| {
                    Expr::Pi(BinderInfo::Default, name, Box::new(ty), Box::new(body))
                }),
        ]
        .boxed()
    }
}

// ── Property 1: Substitution identity ────────────────────────────────────────

proptest! {
    /// For any expression `e` that has no loose BVars, and for any FVar `fvar`,
    /// the round-trip `instantiate(abstract_expr(e, fvar), fvar_expr) == e`
    /// holds provided `fvar_expr` itself has no loose BVars.
    ///
    /// The key constraint is that `e` must not contain loose BVars, because
    /// `abstract_expr` increments ALL BVar indices unconditionally.
    #[test]
    fn prop_substitution_identity(
        e in arb_fvar_expr(3, FVarId::new(999)),
        fvar_id in 900u64..=999u64,
    ) {
        let fvar = FVarId::new(fvar_id);
        // The expression must not have loose BVars for the identity to hold.
        // (abstract_expr shifts ALL bvars up by 1, so if there are existing
        // loose bvars they would shift incorrectly.)
        prop_assume!(!has_loose_bvars(&e));

        // Perform the round-trip.
        let abstracted = abstract_expr(&e, fvar);
        let fvar_expr = Expr::FVar(fvar);
        let reinstated = instantiate(&abstracted, &fvar_expr);

        // The reinstated expression must equal the original.
        prop_assert_eq!(reinstated, e,
            "instantiate(abstract_expr(e, fvar), fvar) should equal e for closed expressions");
    }
}

// ── Property 2: Lift / lower BVars ───────────────────────────────────────────

proptest! {
    /// For any closed expression `e` (no loose BVars) and lift amount `n`,
    /// `lower_bvars(lift_bvars(e, n), n) == e`.
    ///
    /// `lift_bvars` is the identity on expressions with no loose BVars at
    /// depth 0, but in general it shifts all free BVars up by n.  Since we
    /// start with a closed expression (no loose bvars at depth 0), lifting
    /// and then lowering is the identity.
    #[test]
    fn prop_lift_lower_bvars_identity(
        e in arb_closed_expr(3),
        n in 0u32..=5u32,
    ) {
        prop_assume!(!has_loose_bvars(&e));

        let lifted = oxilean_kernel::instantiate::lift_bvars(&e, n);
        let lowered = lower_bvars(&lifted, n);

        prop_assert_eq!(lowered, e,
            "lower_bvars(lift_bvars(e, n), n) should equal e for closed expressions");
    }
}

// ── Property 3: Level normalization idempotency ───────────────────────────────

proptest! {
    /// For any level `l`, `normalize(normalize(l)) == normalize(l)`.
    ///
    /// Normalization is idempotent: applying it twice yields the same result.
    #[test]
    fn prop_level_normalize_idempotent(l in arb_level(4)) {
        let once = normalize_level(&l);
        let twice = normalize_level(&once);
        prop_assert_eq!(twice, once,
            "normalize(normalize(l)) should equal normalize(l)");
    }
}

// ── Property 4: WHNF idempotency ─────────────────────────────────────────────

proptest! {
    /// For any expression `e`, `whnf(whnf(e)) == whnf(e)`.
    ///
    /// WHNF reduction is a fixed point: an expression already in WHNF is
    /// unchanged by a further WHNF step.
    #[test]
    fn prop_whnf_idempotent(e in arb_closed_expr(3)) {
        let once = whnf(&e);
        let twice = whnf(&once);
        prop_assert_eq!(twice, once,
            "whnf(whnf(e)) should equal whnf(e)");
    }
}

// ── Property 5: Definitional equality reflexivity ────────────────────────────

proptest! {
    /// For any expression `e`, `is_def_eq(e, e)` is always true.
    ///
    /// Definitional equality is reflexive: every expression is definitionally
    /// equal to itself.
    #[test]
    fn prop_def_eq_reflexive(e in arb_closed_expr(3)) {
        let result = is_def_eq_simple(&e, &e);
        prop_assert!(result,
            "is_def_eq_simple(e, e) should be true for any expression e");
    }
}

// ── Property 6: Name round-trip ──────────────────────────────────────────────

proptest! {
    /// Any `Name` can be displayed and re-parsed to the same `Name`.
    ///
    /// `Name::from_str(name.to_string())` should yield back the original name,
    /// provided the name contains no empty string components.
    #[test]
    fn prop_name_roundtrip(name in arb_name_for_roundtrip()) {
        let displayed = name.to_string();
        let reparsed = Name::from_str(&displayed);
        prop_assert_eq!(reparsed, name,
            "Name::from_str(name.to_string()) should equal name");
    }
}

// ── Property 6b: Name round-trip for well-known names ────────────────────────

proptest! {
    /// Named constants built from `Name::str` and `append_str` round-trip through display.
    #[test]
    fn prop_name_str_roundtrip(
        head in "[a-z][a-z0-9]{0,7}",
        tail in prop::option::of("[a-z][a-z0-9]{0,7}"),
    ) {
        let name = match tail {
            None => Name::str(&head),
            Some(t) => Name::str(&head).append_str(t),
        };
        let displayed = name.to_string();
        let reparsed = Name::from_str(&displayed);
        prop_assert_eq!(reparsed, name,
            "String-component names must round-trip through to_string/from_str");
    }
}

// ── Bonus: Level equivalence symmetry ────────────────────────────────────────

proptest! {
    /// `normalize(l)` strips all MVars and params by canonicalizing structure.
    /// As a weaker check: normalization of a literal numeric level
    /// equals the same literal numeric level (since it is already normal).
    #[test]
    fn prop_numeric_level_normalize_fixed(n in 0u32..=8u32) {
        // Build Level::Succ^n(Zero).
        let mut l = Level::Zero;
        for _ in 0..n {
            l = Level::Succ(Box::new(l));
        }
        let normed = normalize_level(&l);
        // A chain of Succs over Zero is already in normal form.
        prop_assert_eq!(normed, l,
            "Succ^n(Zero) is already in normal form and should be unchanged by normalize");
    }
}

// ── Bonus: WHNF of atoms ─────────────────────────────────────────────────────

proptest! {
    /// Atomic expressions (Sort, Const, Lit) are already in WHNF.
    #[test]
    fn prop_whnf_atoms_are_fixed_points(l in arb_level(2)) {
        // Sort(l) is always in WHNF.
        let sort = Expr::Sort(l);
        let whnf_sort = whnf(&sort);
        prop_assert_eq!(whnf_sort, sort.clone(),
            "Sort is always a WHNF fixed point");
    }
}

proptest! {
    /// Literal expressions are already in WHNF.
    #[test]
    fn prop_whnf_lit_fixed_points(n in 0u64..=u64::MAX) {
        let lit = Expr::Lit(Literal::Nat(n));
        let whnf_lit = whnf(&lit);
        prop_assert_eq!(whnf_lit, lit,
            "Literal::Nat is always a WHNF fixed point");
    }
}

// ── Bonus: def_eq symmetry on atomic terms ────────────────────────────────────

proptest! {
    /// Definitional equality of distinct numeric literals is false.
    #[test]
    fn prop_def_eq_distinct_lits_false(
        n in 0u64..=999u64,
        m in 1000u64..=1999u64,
    ) {
        let e1 = Expr::Lit(Literal::Nat(n));
        let e2 = Expr::Lit(Literal::Nat(m));
        let result = is_def_eq_simple(&e1, &e2);
        prop_assert!(!result,
            "Distinct literal naturals {} and {} should not be definitionally equal", n, m);
    }
}

// ── Bonus: abstract/instantiate on FVar-free expressions ─────────────────────

proptest! {
    /// If an expression `e` does NOT contain `fvar`, then
    /// `instantiate(abstract_expr(e, fvar), anything) == e`
    /// because abstract_expr introduces BVar(0) only where fvar appeared.
    /// If fvar doesn't appear, abstract_expr only shifts other BVars.
    ///
    /// This is the *stronger* case: no fvar occurrence, no loose bvars.
    #[test]
    fn prop_instantiate_abstract_no_fvar(
        e in arb_closed_expr(3),
        replacement in arb_closed_expr(2),
    ) {
        let fvar = FVarId::new(42_000);
        // e has no FVars (generated by arb_closed_expr) and no loose BVars.
        prop_assume!(!has_loose_bvars(&e));

        let abstracted = abstract_expr(&e, fvar);
        // Since fvar never appears, BVar(0) never appears in abstracted.
        // Instantiate replaces BVar(0) with replacement — harmless since it's absent.
        let result = instantiate(&abstracted, &replacement);

        prop_assert_eq!(result, e,
            "When fvar doesn't occur in e, instantiate(abstract_expr(e, fvar), _) == e");
    }
}
