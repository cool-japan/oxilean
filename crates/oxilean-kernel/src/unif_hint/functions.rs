//! Functions for the unification hint system.
//!
//! The primary entry point for the def_eq integration is
//! `try_unif_hints`, which queries a `UnifHintDB` and verifies
//! hypothesis obligations.

use crate::{Environment, Expr};

use super::types::{match_expr_pattern, PatternSubst, UnifHint, UnifHintDB};

// ── Hypothesis verification ───────────────────────────────────────────────────

/// Check whether all hypotheses of `hint` are satisfied under `subst`,
/// using `def_eq_fn` as the callback for definitional equality.
///
/// `def_eq_fn(t, s)` should return `true` iff `t ≡ s` definitionally.
///
/// This function does **not** recursively call into `UnifHintDB`; that is
/// handled by the caller (typically `DefEqChecker`).
pub fn check_hint_hypotheses<F>(hint: &UnifHint, subst: &PatternSubst, mut def_eq_fn: F) -> bool
where
    F: FnMut(&Expr, &Expr) -> bool,
{
    for (_hyp_name, (lhs_hyp, rhs_hyp)) in &hint.hypotheses {
        let lhs_inst = subst.apply(lhs_hyp);
        let rhs_inst = subst.apply(rhs_hyp);
        if !def_eq_fn(&lhs_inst, &rhs_inst) {
            return false;
        }
    }
    true
}

/// Try to discharge a stuck definitional equality `(t, s)` via unification
/// hints.
///
/// # Arguments
///
/// * `db`         — The hint database to query.
/// * `t`          — Left-hand side (already in WHNF).
/// * `s`          — Right-hand side (already in WHNF).
/// * `def_eq_fn`  — A closure that checks definitional equality (used for
///   hypothesis verification).
///
/// Returns `true` if some hint fires (all its hypotheses are satisfied),
/// `false` if no hint applies.
pub fn try_unif_hints<F>(db: &UnifHintDB, t: &Expr, s: &Expr, def_eq_fn: F) -> bool
where
    F: FnMut(&Expr, &Expr) -> bool + Clone,
{
    if db.is_empty() {
        return false;
    }
    let candidates = db.find_hints(t, s);
    for (hint, subst, _swapped) in candidates {
        if check_hint_hypotheses(hint, &subst, def_eq_fn.clone()) {
            return true;
        }
    }
    false
}

/// Build a simple unconditional hint from two expressions.
///
/// Both `lhs` and `rhs` are stored verbatim; any `Const` node whose name
/// starts with `?` will be treated as a pattern variable during matching.
pub fn make_hint(lhs: Expr, rhs: Expr) -> UnifHint {
    UnifHint::new(lhs, rhs)
}

/// Attempt one-sided pattern matching: match `pattern` against `target`.
///
/// Returns `Some(PatternSubst)` on success, `None` on failure.
pub fn one_sided_match(pattern: &Expr, target: &Expr) -> Option<PatternSubst> {
    let mut subst = PatternSubst::new();
    if match_expr_pattern(pattern, target, &mut subst) {
        Some(subst)
    } else {
        None
    }
}

/// Look up and apply all matching hints, returning the first one whose
/// hypotheses all pass.
///
/// This is a higher-level wrapper that takes both the database and the
/// environment (unused at the moment but passed through for future use).
pub fn resolve_unif_hints<F>(
    db: &UnifHintDB,
    _env: &Environment,
    t: &Expr,
    s: &Expr,
    def_eq_fn: F,
) -> bool
where
    F: FnMut(&Expr, &Expr) -> bool + Clone,
{
    try_unif_hints(db, t, s, def_eq_fn)
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unif_hint::types::UnifHint;
    use crate::{BinderInfo, Level, Literal, Name};

    fn nat(n: u64) -> Expr {
        Expr::Lit(Literal::Nat(n))
    }

    fn const_name(s: &str) -> Expr {
        Expr::Const(Name::str(s), vec![])
    }

    fn pat_var(s: &str) -> Expr {
        // Pattern variable: Const whose name starts with '?'
        Expr::Const(Name::str(&format!("?{}", s)), vec![])
    }

    // ── UnifHintDB ──────────────────────────────────────────────────────────

    #[test]
    fn test_empty_db_no_hints() {
        let db = UnifHintDB::new();
        assert!(db.is_empty());
        assert_eq!(db.len(), 0);
    }

    #[test]
    fn test_add_hint_increments_len() {
        let mut db = UnifHintDB::new();
        db.add_hint(UnifHint::new(nat(1), nat(1)));
        assert_eq!(db.len(), 1);
        db.add_hint(UnifHint::new(nat(2), nat(2)));
        assert_eq!(db.len(), 2);
    }

    #[test]
    fn test_priority_ordering() {
        let mut db = UnifHintDB::new();
        db.add_hint(UnifHint::new(nat(1), nat(1)).with_priority(5));
        db.add_hint(UnifHint::new(nat(2), nat(2)).with_priority(10));
        db.add_hint(UnifHint::new(nat(3), nat(3)).with_priority(1));
        // Highest priority first
        assert_eq!(db.all_hints()[0].priority, 10);
        assert_eq!(db.all_hints()[1].priority, 5);
        assert_eq!(db.all_hints()[2].priority, 1);
    }

    // ── Pattern matching ────────────────────────────────────────────────────

    #[test]
    fn test_literal_match() {
        let pattern = nat(42);
        let target = nat(42);
        let mut subst = PatternSubst::new();
        assert!(match_expr_pattern(&pattern, &target, &mut subst));
        assert!(subst.is_empty());
    }

    #[test]
    fn test_literal_mismatch() {
        let pattern = nat(42);
        let target = nat(99);
        let mut subst = PatternSubst::new();
        assert!(!match_expr_pattern(&pattern, &target, &mut subst));
    }

    #[test]
    fn test_pattern_variable_binds() {
        let pattern = pat_var("x");
        let target = nat(7);
        let mut subst = PatternSubst::new();
        assert!(match_expr_pattern(&pattern, &target, &mut subst));
        let x_name = Name::str("x");
        assert_eq!(subst.get(&x_name), Some(&nat(7)));
    }

    #[test]
    fn test_pattern_variable_consistent_binding() {
        // Same var used twice — must bind to same value
        let xp = pat_var("x");
        let app_pattern = Expr::App(Box::new(xp.clone()), Box::new(xp.clone()));
        let app_target = Expr::App(Box::new(nat(5)), Box::new(nat(5)));
        let mut subst = PatternSubst::new();
        assert!(match_expr_pattern(&app_pattern, &app_target, &mut subst));
    }

    #[test]
    fn test_pattern_variable_conflicting_binding() {
        // Same var used twice — binds to different values → fail
        let xp = pat_var("x");
        let app_pattern = Expr::App(Box::new(xp.clone()), Box::new(xp.clone()));
        let app_target = Expr::App(Box::new(nat(5)), Box::new(nat(6)));
        let mut subst = PatternSubst::new();
        assert!(!match_expr_pattern(&app_pattern, &app_target, &mut subst));
    }

    #[test]
    fn test_app_match() {
        let f = const_name("f");
        let fp = const_name("f");
        let pattern = Expr::App(Box::new(fp), Box::new(pat_var("x")));
        let target = Expr::App(Box::new(f), Box::new(nat(3)));
        let mut subst = PatternSubst::new();
        assert!(match_expr_pattern(&pattern, &target, &mut subst));
        assert_eq!(subst.get(&Name::str("x")), Some(&nat(3)));
    }

    #[test]
    fn test_pi_match() {
        let pattern = Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(pat_var("A")),
            Box::new(pat_var("B")),
        );
        let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
        let target = Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(nat_ty.clone()),
            Box::new(nat_ty.clone()),
        );
        let mut subst = PatternSubst::new();
        assert!(match_expr_pattern(&pattern, &target, &mut subst));
        assert_eq!(subst.get(&Name::str("A")), Some(&nat_ty));
        assert_eq!(subst.get(&Name::str("B")), Some(&nat_ty));
    }

    // ── find_hints ──────────────────────────────────────────────────────────

    #[test]
    fn test_find_hints_forward() {
        let mut db = UnifHintDB::new();
        // Hint: ?x ≡ ?x  (reflexivity hint)
        let hint = UnifHint::new(pat_var("x"), pat_var("y"));
        db.add_hint(hint);
        let t = nat(10);
        let s = nat(10);
        let results = db.find_hints(&t, &s);
        assert!(!results.is_empty());
    }

    #[test]
    fn test_find_hints_swapped() {
        let mut db = UnifHintDB::new();
        // Hint: Nat ≡ ?T
        let hint = UnifHint::new(Expr::Const(Name::str("Nat"), vec![]), pat_var("T"));
        db.add_hint(hint);
        // Query: ?T vs Nat (swapped)
        let t = nat(0); // not Nat, but let's test with the reverse
        let s = Expr::Const(Name::str("Nat"), vec![]);
        let results = db.find_hints(&s, &t);
        // forward: Nat matches s? yes. ?T matches t? yes.
        assert!(!results.is_empty());
    }

    #[test]
    fn test_find_hints_no_match() {
        let mut db = UnifHintDB::new();
        // Hint: f ?x ≡ g ?x — f and g are different constants
        let f = const_name("f");
        let g = const_name("g");
        let hint = UnifHint::new(
            Expr::App(Box::new(f.clone()), Box::new(pat_var("x"))),
            Expr::App(Box::new(g.clone()), Box::new(pat_var("x"))),
        );
        db.add_hint(hint);
        // Query: h ?x vs g ?x — h ≠ f so no match
        let h_app = Expr::App(Box::new(const_name("h")), Box::new(nat(1)));
        let g_app = Expr::App(Box::new(g.clone()), Box::new(nat(1)));
        let results = db.find_hints(&h_app, &g_app);
        assert!(results.is_empty());
    }

    // ── try_unif_hints ──────────────────────────────────────────────────────

    #[test]
    fn test_try_unif_hints_unconditional_fires() {
        let mut db = UnifHintDB::new();
        // Hint: add 0 ?n ≡ ?n  (zero + n = n)
        let add = const_name("add");
        let zero = nat(0);
        let lhs_pat = Expr::App(
            Box::new(Expr::App(Box::new(add.clone()), Box::new(zero.clone()))),
            Box::new(pat_var("n")),
        );
        let rhs_pat = pat_var("n");
        db.add_hint(UnifHint::new(lhs_pat, rhs_pat));

        // Query: add 0 42 ≡ 42
        let query_lhs = Expr::App(
            Box::new(Expr::App(Box::new(add), Box::new(zero))),
            Box::new(nat(42)),
        );
        let query_rhs = nat(42);

        let fired = try_unif_hints(&db, &query_lhs, &query_rhs, |_t, _s| true);
        assert!(fired);
    }

    #[test]
    fn test_try_unif_hints_conditional_satisfied() {
        let mut db = UnifHintDB::new();
        // Hint with hypothesis: h ?x ≡ ?x  IF  ?x ≡ ?x
        let lhs_pat = Expr::App(Box::new(const_name("h")), Box::new(pat_var("x")));
        let rhs_pat = pat_var("x");
        let hyp_name = Name::str("hyp");
        let hint = UnifHint::with_hypotheses(
            lhs_pat,
            rhs_pat,
            vec![(hyp_name, (pat_var("x"), pat_var("x")))],
        );
        db.add_hint(hint);

        let query_lhs = Expr::App(Box::new(const_name("h")), Box::new(nat(5)));
        let query_rhs = nat(5);

        // def_eq_fn always returns true (hypotheses trivially satisfied)
        let fired = try_unif_hints(&db, &query_lhs, &query_rhs, |_t, _s| true);
        assert!(fired);
    }

    #[test]
    fn test_try_unif_hints_conditional_not_satisfied() {
        let mut db = UnifHintDB::new();
        // Hint: h ?x ≡ ?x  IF  ?x ≡ some_sentinel
        let lhs_pat = Expr::App(Box::new(const_name("h")), Box::new(pat_var("x")));
        let rhs_pat = pat_var("x");
        let sentinel = const_name("sentinel");
        let hint = UnifHint::with_hypotheses(
            lhs_pat,
            rhs_pat,
            vec![(Name::str("guard"), (pat_var("x"), sentinel))],
        );
        db.add_hint(hint);

        let query_lhs = Expr::App(Box::new(const_name("h")), Box::new(nat(5)));
        let query_rhs = nat(5);

        // def_eq_fn always returns false (hypothesis fails)
        let fired = try_unif_hints(&db, &query_lhs, &query_rhs, |_t, _s| false);
        assert!(!fired);
    }

    #[test]
    fn test_empty_db_never_fires() {
        let db = UnifHintDB::new();
        let fired = try_unif_hints(&db, &nat(1), &nat(2), |_t, _s| true);
        assert!(!fired);
    }

    // ── PatternSubst::apply ─────────────────────────────────────────────────

    #[test]
    fn test_subst_apply_const() {
        let mut subst = PatternSubst::new();
        // Bind "n" → Nat 7
        subst.bind(&Name::str("n"), nat(7));
        // Applying to Const("n") should substitute
        let result = subst.apply(&const_name("n"));
        assert_eq!(result, nat(7));
    }

    #[test]
    fn test_subst_apply_app() {
        let mut subst = PatternSubst::new();
        subst.bind(&Name::str("a"), nat(1));
        subst.bind(&Name::str("b"), nat(2));
        let expr = Expr::App(Box::new(const_name("a")), Box::new(const_name("b")));
        let result = subst.apply(&expr);
        assert_eq!(result, Expr::App(Box::new(nat(1)), Box::new(nat(2))));
    }

    // ── remove_named / clear ────────────────────────────────────────────────

    #[test]
    fn test_remove_named() {
        let mut db = UnifHintDB::new();
        db.add_hint(UnifHint::new(nat(1), nat(1)).named(Name::str("h1")));
        db.add_hint(UnifHint::new(nat(2), nat(2)).named(Name::str("h2")));
        assert_eq!(db.len(), 2);
        db.remove_named(&Name::str("h1"));
        assert_eq!(db.len(), 1);
        assert_eq!(
            db.all_hints()[0].name.as_ref().map(|n| n.to_string()),
            Some("h2".to_string())
        );
    }

    #[test]
    fn test_clear() {
        let mut db = UnifHintDB::new();
        db.add_hint(UnifHint::new(nat(1), nat(1)));
        db.add_hint(UnifHint::new(nat(2), nat(2)));
        db.clear();
        assert!(db.is_empty());
    }

    // ── one_sided_match ─────────────────────────────────────────────────────

    #[test]
    fn test_one_sided_match_success() {
        let pattern = Expr::App(Box::new(const_name("f")), Box::new(pat_var("x")));
        let target = Expr::App(Box::new(const_name("f")), Box::new(nat(99)));
        let result = one_sided_match(&pattern, &target);
        assert!(result.is_some());
        let subst = result.expect("one_sided_match should succeed");
        assert_eq!(subst.get(&Name::str("x")), Some(&nat(99)));
    }

    #[test]
    fn test_one_sided_match_failure() {
        let pattern = Expr::App(Box::new(const_name("f")), Box::new(pat_var("x")));
        let target = Expr::App(Box::new(const_name("g")), Box::new(nat(99)));
        assert!(one_sided_match(&pattern, &target).is_none());
    }

    // ── make_hint ───────────────────────────────────────────────────────────

    #[test]
    fn test_make_hint_roundtrip() {
        let h = make_hint(nat(1), nat(2));
        assert_eq!(h.lhs, nat(1));
        assert_eq!(h.rhs, nat(2));
        assert!(h.is_unconditional());
    }
}
