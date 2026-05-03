//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::reduce::TransparencyMode;
use crate::{Environment, Expr, Reducer};

use super::types::{
    BitSet64, BucketCounter, Coercion, CoercionTable, ConfigNode, ConvResult, ConversionChecker,
    DecisionNode, Either2, Fixture, FlatSubstitution, FocusStack, LabelSet, MinHeap, NonEmptyVec,
    PathBuf, PrefixCounter, RewriteRule, RewriteRuleSet, SimpleDag, SlidingSum, SmallMap,
    SparseVec, StackCalc, StatSummary, Stopwatch, StringPool, TokenBucket, TransformStat,
    TransitiveClosure, VersionedRecord, WindowIterator, WriteOnce,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Level, Literal, Name};
    #[test]
    fn test_convertible_identical() {
        let mut checker = ConversionChecker::new();
        let e = Expr::Lit(Literal::Nat(42));
        assert!(checker.is_convertible(&e, &e));
    }
    #[test]
    fn test_convertible_different_lits() {
        let mut checker = ConversionChecker::new();
        let e1 = Expr::Lit(Literal::Nat(1));
        let e2 = Expr::Lit(Literal::Nat(2));
        assert!(!checker.is_convertible(&e1, &e2));
    }
    #[test]
    fn test_convertible_sorts() {
        let mut checker = ConversionChecker::new();
        let s1 = Expr::Sort(Level::zero());
        let s2 = Expr::Sort(Level::zero());
        assert!(checker.is_convertible(&s1, &s2));
    }
    #[test]
    fn test_convertible_apps() {
        let mut checker = ConversionChecker::new();
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::Lit(Literal::Nat(1));
        let app1 = Expr::App(Box::new(f.clone()), Box::new(a.clone()));
        let app2 = Expr::App(Box::new(f), Box::new(a));
        assert!(checker.is_convertible(&app1, &app2));
    }
    #[test]
    fn test_transparency_modes() {
        let checker = ConversionChecker::with_transparency(TransparencyMode::All);
        assert_eq!(checker.transparency(), TransparencyMode::All);
        let checker2 = ConversionChecker::with_transparency(TransparencyMode::None);
        assert_eq!(checker2.transparency(), TransparencyMode::None);
    }
    #[test]
    fn test_sort_equivalence() {
        let mut checker = ConversionChecker::new();
        let s1 = Expr::Sort(crate::Level::max(
            crate::Level::param(Name::str("u")),
            crate::Level::param(Name::str("v")),
        ));
        let s2 = Expr::Sort(crate::Level::max(
            crate::Level::param(Name::str("v")),
            crate::Level::param(Name::str("u")),
        ));
        assert!(checker.is_convertible(&s1, &s2));
    }
    #[test]
    fn test_convertible_in_env() {
        let mut checker = ConversionChecker::new();
        let mut env = Environment::new();
        env.add(crate::Declaration::Definition {
            name: Name::str("answer"),
            univ_params: vec![],
            ty: Expr::Const(Name::str("Nat"), vec![]),
            val: Expr::Lit(Literal::Nat(42)),
            hint: crate::ReducibilityHint::Regular(1),
        })
        .expect("value should be present");
        let answer = Expr::Const(Name::str("answer"), vec![]);
        let forty_two = Expr::Lit(Literal::Nat(42));
        assert!(checker.is_convertible_in_env(&answer, &forty_two, &env));
    }
    #[test]
    fn test_proj_convertible() {
        let mut checker = ConversionChecker::new();
        let e = Expr::BVar(0);
        let p1 = Expr::Proj(Name::str("Prod"), 0, Box::new(e.clone()));
        let p2 = Expr::Proj(Name::str("Prod"), 0, Box::new(e));
        assert!(checker.is_convertible(&p1, &p2));
    }
}
pub(super) fn coercion_head_matches(source_ty: &crate::Expr, name: &crate::Name) -> bool {
    let mut e = source_ty;
    while let crate::Expr::App(f, _) = e {
        e = f;
    }
    matches!(e, crate ::Expr::Const(n, _) if n == name)
}
/// Perform conversion checking with diagnostics.
pub fn check_conversion(e1: &Expr, e2: &Expr, max_steps: usize) -> ConvResult {
    let mut checker = ConversionChecker::new();
    checker.set_max_steps(max_steps);
    if checker.is_convertible(e1, e2) {
        ConvResult::Equal
    } else {
        ConvResult::NotEqual
    }
}
/// Check if two expressions are definitionally equal with a specified transparency mode.
pub fn def_eq_with_mode(e1: &Expr, e2: &Expr, env: &Environment, mode: TransparencyMode) -> bool {
    let mut checker = ConversionChecker::with_transparency(mode);
    checker.is_convertible_in_env(e1, e2, env)
}
/// Check if two level expressions are definitionally equivalent.
pub fn levels_def_eq(l1: &crate::Level, l2: &crate::Level) -> bool {
    crate::level::is_equivalent(l1, l2)
}
/// Collect pairs of subexpressions that differ between two expressions.
pub fn conversion_diff(e1: &Expr, e2: &Expr) -> Vec<(Expr, Expr)> {
    let mut diffs = Vec::new();
    collect_diffs(e1, e2, &mut diffs);
    diffs
}
pub(super) fn collect_diffs(e1: &Expr, e2: &Expr, diffs: &mut Vec<(Expr, Expr)>) {
    if e1 == e2 {
        return;
    }
    match (e1, e2) {
        (Expr::App(f1, a1), Expr::App(f2, a2)) => {
            collect_diffs(f1, f2, diffs);
            collect_diffs(a1, a2, diffs);
        }
        (Expr::Lam(_, _, ty1, b1), Expr::Lam(_, _, ty2, b2))
        | (Expr::Pi(_, _, ty1, b1), Expr::Pi(_, _, ty2, b2)) => {
            collect_diffs(ty1, ty2, diffs);
            collect_diffs(b1, b2, diffs);
        }
        _ => {
            diffs.push((e1.clone(), e2.clone()));
        }
    }
}
/// Compute a heuristic convertibility score between 0.0 and 1.0.
pub fn convertibility_score(e1: &Expr, e2: &Expr) -> f64 {
    if e1 == e2 {
        return 1.0;
    }
    match (e1, e2) {
        (Expr::App(f1, a1), Expr::App(f2, a2)) => {
            (convertibility_score(f1, f2) + convertibility_score(a1, a2)) / 2.0
        }
        (Expr::Lam(_, _, ty1, b1), Expr::Lam(_, _, ty2, b2))
        | (Expr::Pi(_, _, ty1, b1), Expr::Pi(_, _, ty2, b2)) => {
            (convertibility_score(ty1, ty2) + convertibility_score(b1, b2)) / 2.0
        }
        (Expr::Const(n1, _), Expr::Const(n2, _)) if n1 == n2 => 0.9,
        (Expr::Const(_, _), Expr::Const(_, _)) => 0.0,
        (Expr::BVar(i1), Expr::BVar(i2)) if i1 == i2 => 1.0,
        (Expr::BVar(_), Expr::BVar(_)) => 0.0,
        (Expr::Sort(_), Expr::Sort(_)) => 0.8,
        _ => 0.0,
    }
}
/// Check whether two expressions have the same outermost constructor/head.
pub fn same_head(e1: &Expr, e2: &Expr) -> bool {
    match (e1, e2) {
        (Expr::BVar(i1), Expr::BVar(i2)) => i1 == i2,
        (Expr::FVar(f1), Expr::FVar(f2)) => f1 == f2,
        (Expr::Sort(_), Expr::Sort(_)) => true,
        (Expr::Lit(_), Expr::Lit(_)) => true,
        (Expr::Const(n1, _), Expr::Const(n2, _)) => n1 == n2,
        (Expr::App(_, _), Expr::App(_, _)) => true,
        (Expr::Lam(_, _, _, _), Expr::Lam(_, _, _, _)) => true,
        (Expr::Pi(_, _, _, _), Expr::Pi(_, _, _, _)) => true,
        (Expr::Let(_, _, _, _), Expr::Let(_, _, _, _)) => true,
        (Expr::Proj(n1, i1, _), Expr::Proj(n2, i2, _)) => n1 == n2 && i1 == i2,
        _ => false,
    }
}
/// Fast syntactic equality check without reduction.
pub fn syntactic_eq(e1: &Expr, e2: &Expr) -> bool {
    e1 == e2
}
/// Check whether an expression is transparency-neutral.
pub fn is_transparency_neutral(expr: &Expr) -> bool {
    match expr {
        Expr::BVar(_) | Expr::FVar(_) | Expr::Sort(_) | Expr::Lit(_) => true,
        Expr::Const(_, _) => false,
        Expr::App(f, a) => is_transparency_neutral(f) && is_transparency_neutral(a),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            is_transparency_neutral(ty) && is_transparency_neutral(body)
        }
        Expr::Let(_, ty, val, body) => {
            is_transparency_neutral(ty)
                && is_transparency_neutral(val)
                && is_transparency_neutral(body)
        }
        Expr::Proj(_, _, e) => is_transparency_neutral(e),
    }
}
#[cfg(test)]
mod extended_tests {
    use super::*;
    use crate::{Level, Literal, Name};
    #[test]
    fn test_coercion_table_empty() {
        let table = CoercionTable::new();
        assert!(table.is_empty());
    }
    #[test]
    fn test_coercion_table_register_and_find() {
        let mut table = CoercionTable::new();
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let int = Expr::Const(Name::str("Int"), vec![]);
        table.register(Coercion {
            name: Name::str("Nat.toInt"),
            source: nat,
            target: int,
            priority: 100,
        });
        let found = table.find(&Name::str("Nat"));
        assert_eq!(found.len(), 1);
    }
    #[test]
    fn test_conv_result_is_equal() {
        assert!(ConvResult::Equal.is_equal());
        assert!(!ConvResult::NotEqual.is_equal());
        assert!(!ConvResult::Timeout { steps: 5 }.is_equal());
    }
    #[test]
    fn test_check_conversion_equal() {
        let e = Expr::Lit(Literal::Nat(42));
        assert!(check_conversion(&e, &e, 1000).is_equal());
    }
    #[test]
    fn test_conversion_diff_same() {
        let e = Expr::BVar(0);
        assert!(conversion_diff(&e, &e).is_empty());
    }
    #[test]
    fn test_conversion_diff_different() {
        let e1 = Expr::Const(Name::str("A"), vec![]);
        let e2 = Expr::Const(Name::str("B"), vec![]);
        assert_eq!(conversion_diff(&e1, &e2).len(), 1);
    }
    #[test]
    fn test_levels_def_eq() {
        let l1 = Level::zero();
        let l2 = Level::zero();
        assert!(levels_def_eq(&l1, &l2));
    }
    #[test]
    fn test_convertibility_score_same() {
        let e = Expr::Const(Name::str("Nat"), vec![]);
        assert!((convertibility_score(&e, &e) - 1.0).abs() < 1e-9);
    }
    #[test]
    fn test_same_head_sorts() {
        let s1 = Expr::Sort(Level::zero());
        let s2 = Expr::Sort(Level::succ(Level::zero()));
        assert!(same_head(&s1, &s2));
    }
    #[test]
    fn test_is_transparency_neutral_lit() {
        assert!(is_transparency_neutral(&Expr::Lit(Literal::Nat(0))));
    }
    #[test]
    fn test_is_transparency_neutral_const() {
        assert!(!is_transparency_neutral(&Expr::Const(
            Name::str("foo"),
            vec![]
        )));
    }
    #[test]
    fn test_def_eq_with_mode() {
        let env = Environment::new();
        let e = Expr::Lit(Literal::Nat(42));
        assert!(def_eq_with_mode(&e, &e, &env, TransparencyMode::Default));
    }
}
/// Bounded depth search for convertibility.
pub fn bounded_conversion(e1: &Expr, e2: &Expr, max_depth: usize) -> ConvResult {
    if syntactic_eq(e1, e2) {
        return ConvResult::Equal;
    }
    let mut checker = ConversionChecker::new();
    for depth in 1..=max_depth {
        checker.set_max_steps(depth * 100);
        if checker.is_convertible(e1, e2) {
            return ConvResult::Equal;
        }
    }
    ConvResult::NotEqual
}
/// Check eta-equality: f and lambda x. f x are eta-equal.
pub fn is_eta_equal(f: &Expr, g: &Expr) -> bool {
    if let Expr::Lam(_, _, _, body) = g {
        if let Expr::App(head, arg) = body.as_ref() {
            if matches!(arg.as_ref(), Expr::BVar(0)) {
                return structurally_equal_shifted(f, head, 1);
            }
        }
    }
    if let Expr::Lam(_, _, _, body) = f {
        if let Expr::App(head, arg) = body.as_ref() {
            if matches!(arg.as_ref(), Expr::BVar(0)) {
                return structurally_equal_shifted(g, head, 1);
            }
        }
    }
    false
}
pub(super) fn structurally_equal_shifted(e1: &Expr, e2: &Expr, shift: u32) -> bool {
    match (e1, e2) {
        (Expr::BVar(i1), Expr::BVar(i2)) => i1 == &(*i2 + shift),
        (Expr::Const(n1, ls1), Expr::Const(n2, ls2)) => n1 == n2 && ls1 == ls2,
        (Expr::FVar(f1), Expr::FVar(f2)) => f1 == f2,
        (Expr::App(f1, a1), Expr::App(f2, a2)) => {
            structurally_equal_shifted(f1, f2, shift) && structurally_equal_shifted(a1, a2, shift)
        }
        (Expr::Lam(_, _, ty1, b1), Expr::Lam(_, _, ty2, b2))
        | (Expr::Pi(_, _, ty1, b1), Expr::Pi(_, _, ty2, b2)) => {
            structurally_equal_shifted(ty1, ty2, shift) && structurally_equal_shifted(b1, b2, shift)
        }
        _ => e1 == e2,
    }
}
#[cfg(test)]
mod bounded_tests {
    use super::*;
    use crate::{Literal, Name};
    #[test]
    fn test_syntactic_eq() {
        let e = Expr::Lit(Literal::Nat(42));
        assert!(syntactic_eq(&e, &e));
        assert!(!syntactic_eq(&e, &Expr::Lit(Literal::Nat(43))));
    }
    #[test]
    fn test_bounded_conversion_equal() {
        let e = Expr::Lit(Literal::Nat(1));
        assert!(bounded_conversion(&e, &e, 5).is_equal());
    }
    #[test]
    fn test_bounded_conversion_not_equal() {
        let e1 = Expr::Lit(Literal::Nat(1));
        let e2 = Expr::Lit(Literal::Nat(2));
        assert!(!bounded_conversion(&e1, &e2, 5).is_equal());
    }
    #[test]
    fn test_same_head_different() {
        let s = Expr::Sort(crate::Level::zero());
        let c = Expr::Const(Name::str("X"), vec![]);
        assert!(!same_head(&s, &c));
    }
    #[test]
    fn test_same_head_lits() {
        let l1 = Expr::Lit(Literal::Nat(1));
        let l2 = Expr::Lit(Literal::Nat(2));
        assert!(same_head(&l1, &l2));
    }
    #[test]
    fn test_conv_result_is_definitive() {
        assert!(ConvResult::Equal.is_definitive());
        assert!(ConvResult::NotEqual.is_definitive());
        assert!(!ConvResult::Timeout { steps: 0 }.is_definitive());
    }
    #[test]
    fn test_coercion_priority_order() {
        let mut table = CoercionTable::new();
        let src = Expr::Const(crate::Name::str("A"), vec![]);
        let tgt = Expr::Const(crate::Name::str("B"), vec![]);
        table.register(Coercion {
            name: crate::Name::str("low"),
            source: src.clone(),
            target: tgt.clone(),
            priority: 10,
        });
        table.register(Coercion {
            name: crate::Name::str("high"),
            source: src.clone(),
            target: tgt.clone(),
            priority: 100,
        });
        let found = table.find(&crate::Name::str("A"));
        assert_eq!(found.len(), 2);
        assert!(found[0].priority >= found[1].priority);
    }
    #[test]
    fn test_convertibility_score_diff_consts() {
        let e1 = Expr::Const(crate::Name::str("A"), vec![]);
        let e2 = Expr::Const(crate::Name::str("B"), vec![]);
        assert_eq!(convertibility_score(&e1, &e2), 0.0);
    }
    #[test]
    fn test_conversion_diff_app() {
        let f = Expr::Const(crate::Name::str("f"), vec![]);
        let a1 = Expr::BVar(0);
        let a2 = Expr::BVar(1);
        let app1 = Expr::App(Box::new(f.clone()), Box::new(a1));
        let app2 = Expr::App(Box::new(f), Box::new(a2));
        let diffs = conversion_diff(&app1, &app2);
        assert_eq!(diffs.len(), 1);
    }
}
/// Check if an expression is in "atomic" form (no reducible subexpressions).
///
/// Atomic expressions are: BVar, FVar, Sort, Lit, Const.
pub fn is_atomic(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::BVar(_) | Expr::FVar(_) | Expr::Sort(_) | Expr::Lit(_) | Expr::Const(_, _)
    )
}
/// Count the number of distinct free variables in an expression.
pub fn count_free_vars(expr: &Expr) -> usize {
    let mut seen = std::collections::HashSet::new();
    count_fvars_impl(expr, &mut seen);
    seen.len()
}
pub(super) fn count_fvars_impl(expr: &Expr, seen: &mut std::collections::HashSet<crate::FVarId>) {
    match expr {
        Expr::FVar(id) => {
            seen.insert(*id);
        }
        Expr::App(f, a) => {
            count_fvars_impl(f, seen);
            count_fvars_impl(a, seen);
        }
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            count_fvars_impl(ty, seen);
            count_fvars_impl(body, seen);
        }
        Expr::Let(_, ty, val, body) => {
            count_fvars_impl(ty, seen);
            count_fvars_impl(val, seen);
            count_fvars_impl(body, seen);
        }
        Expr::Proj(_, _, e) => count_fvars_impl(e, seen),
        _ => {}
    }
}
#[cfg(test)]
mod atomic_tests {
    use super::*;
    use crate::{FVarId, Literal, Name};
    #[test]
    fn test_is_atomic_bvar() {
        assert!(is_atomic(&Expr::BVar(0)));
    }
    #[test]
    fn test_is_atomic_app() {
        let app = Expr::App(Box::new(Expr::BVar(0)), Box::new(Expr::BVar(1)));
        assert!(!is_atomic(&app));
    }
    #[test]
    fn test_count_free_vars_none() {
        assert_eq!(count_free_vars(&Expr::BVar(0)), 0);
    }
    #[test]
    fn test_count_free_vars_one() {
        let e = Expr::FVar(FVarId(1));
        assert_eq!(count_free_vars(&e), 1);
    }
    #[test]
    fn test_count_free_vars_dedup() {
        let fv = Expr::FVar(FVarId(1));
        let app = Expr::App(Box::new(fv.clone()), Box::new(fv));
        assert_eq!(count_free_vars(&app), 1);
    }
    #[test]
    fn test_is_eta_equal_basic() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let lam = Expr::Lam(
            crate::BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Sort(crate::Level::zero())),
            Box::new(Expr::App(Box::new(Expr::BVar(1)), Box::new(Expr::BVar(0)))),
        );
        let _ = is_eta_equal(&f, &lam);
    }
    #[test]
    fn test_is_atomic_lit() {
        assert!(is_atomic(&Expr::Lit(Literal::Nat(42))));
    }
    #[test]
    fn test_is_atomic_const() {
        assert!(is_atomic(&Expr::Const(Name::str("Nat"), vec![])));
    }
}
/// Check if an expression has no subterms at all (is completely atomic).
pub fn is_leaf(expr: &Expr) -> bool {
    is_atomic(expr)
}
/// Check if two expressions are syntactically equal (alias for syntactic_eq).
pub fn exprs_equal(e1: &Expr, e2: &Expr) -> bool {
    syntactic_eq(e1, e2)
}
/// Check if an expression structurally "contains" another as a subterm.
#[allow(dead_code)]
pub fn contains_subterm(haystack: &Expr, needle: &Expr) -> bool {
    if haystack == needle {
        return true;
    }
    match haystack {
        Expr::App(f, a) => contains_subterm(f, needle) || contains_subterm(a, needle),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            contains_subterm(ty, needle) || contains_subterm(body, needle)
        }
        Expr::Let(_, ty, val, body) => {
            contains_subterm(ty, needle)
                || contains_subterm(val, needle)
                || contains_subterm(body, needle)
        }
        Expr::Proj(_, _, e) => contains_subterm(e, needle),
        _ => false,
    }
}
/// Compute the "edit distance" between two expressions as a rough
/// measure of how different they are structurally.
///
/// Returns 0 for identical expressions.
#[allow(dead_code)]
pub fn structural_distance(e1: &Expr, e2: &Expr) -> usize {
    if e1 == e2 {
        return 0;
    }
    match (e1, e2) {
        (Expr::App(f1, a1), Expr::App(f2, a2)) => {
            structural_distance(f1, f2) + structural_distance(a1, a2)
        }
        (Expr::Lam(_, _, ty1, b1), Expr::Lam(_, _, ty2, b2))
        | (Expr::Pi(_, _, ty1, b1), Expr::Pi(_, _, ty2, b2)) => {
            structural_distance(ty1, ty2) + structural_distance(b1, b2)
        }
        (Expr::Let(_, ty1, v1, b1), Expr::Let(_, ty2, v2, b2)) => {
            structural_distance(ty1, ty2)
                + structural_distance(v1, v2)
                + structural_distance(b1, b2)
        }
        _ => 1,
    }
}
/// Check if two expressions differ only in binder names (alpha-equivalent
/// at the surface level, without substitution).
#[allow(dead_code)]
pub fn alpha_similar(e1: &Expr, e2: &Expr) -> bool {
    match (e1, e2) {
        (Expr::BVar(i1), Expr::BVar(i2)) => i1 == i2,
        (Expr::FVar(f1), Expr::FVar(f2)) => f1 == f2,
        (Expr::Const(n1, ls1), Expr::Const(n2, ls2)) => n1 == n2 && ls1 == ls2,
        (Expr::Sort(l1), Expr::Sort(l2)) => crate::level::is_equivalent(l1, l2),
        (Expr::Lit(l1), Expr::Lit(l2)) => l1 == l2,
        (Expr::App(f1, a1), Expr::App(f2, a2)) => alpha_similar(f1, f2) && alpha_similar(a1, a2),
        (Expr::Lam(bi1, _, ty1, b1), Expr::Lam(bi2, _, ty2, b2)) => {
            bi1 == bi2 && alpha_similar(ty1, ty2) && alpha_similar(b1, b2)
        }
        (Expr::Pi(bi1, _, ty1, b1), Expr::Pi(bi2, _, ty2, b2)) => {
            bi1 == bi2 && alpha_similar(ty1, ty2) && alpha_similar(b1, b2)
        }
        (Expr::Let(_, ty1, v1, b1), Expr::Let(_, ty2, v2, b2)) => {
            alpha_similar(ty1, ty2) && alpha_similar(v1, v2) && alpha_similar(b1, b2)
        }
        (Expr::Proj(n1, i1, e1), Expr::Proj(n2, i2, e2)) => {
            n1 == n2 && i1 == i2 && alpha_similar(e1, e2)
        }
        _ => false,
    }
}
/// Depth of an expression tree.
#[allow(dead_code)]
pub fn expr_depth(expr: &Expr) -> usize {
    match expr {
        Expr::BVar(_) | Expr::FVar(_) | Expr::Sort(_) | Expr::Const(_, _) | Expr::Lit(_) => 0,
        Expr::App(f, a) => 1 + expr_depth(f).max(expr_depth(a)),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            1 + expr_depth(ty).max(expr_depth(body))
        }
        Expr::Let(_, ty, val, body) => {
            1 + expr_depth(ty).max(expr_depth(val)).max(expr_depth(body))
        }
        Expr::Proj(_, _, e) => 1 + expr_depth(e),
    }
}
/// Count total number of nodes in an expression tree.
#[allow(dead_code)]
pub fn expr_size(expr: &Expr) -> usize {
    match expr {
        Expr::BVar(_) | Expr::FVar(_) | Expr::Sort(_) | Expr::Const(_, _) | Expr::Lit(_) => 1,
        Expr::App(f, a) => 1 + expr_size(f) + expr_size(a),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => 1 + expr_size(ty) + expr_size(body),
        Expr::Let(_, ty, val, body) => 1 + expr_size(ty) + expr_size(val) + expr_size(body),
        Expr::Proj(_, _, e) => 1 + expr_size(e),
    }
}
/// Check whether an expression is in normal form: no beta-redexes at top level.
#[allow(dead_code)]
pub fn is_beta_normal(expr: &Expr) -> bool {
    match expr {
        Expr::App(f, _) => !matches!(f.as_ref(), Expr::Lam(_, _, _, _)),
        _ => true,
    }
}
#[cfg(test)]
mod extra_conv_tests {
    use super::*;
    use crate::{Literal, Name};
    #[test]
    fn test_contains_subterm_self() {
        let e = Expr::Const(Name::str("Nat"), vec![]);
        assert!(contains_subterm(&e, &e));
    }
    #[test]
    fn test_contains_subterm_inside_app() {
        let needle = Expr::BVar(0);
        let e = Expr::App(
            Box::new(Expr::Const(Name::str("f"), vec![])),
            Box::new(needle.clone()),
        );
        assert!(contains_subterm(&e, &needle));
    }
    #[test]
    fn test_structural_distance_same() {
        let e = Expr::Lit(Literal::Nat(1));
        assert_eq!(structural_distance(&e, &e), 0);
    }
    #[test]
    fn test_structural_distance_diff() {
        let e1 = Expr::Const(Name::str("A"), vec![]);
        let e2 = Expr::Const(Name::str("B"), vec![]);
        assert_eq!(structural_distance(&e1, &e2), 1);
    }
    #[test]
    fn test_alpha_similar_same() {
        let e = Expr::BVar(0);
        assert!(alpha_similar(&e, &e));
    }
    #[test]
    fn test_expr_depth_leaf() {
        assert_eq!(expr_depth(&Expr::BVar(0)), 0);
    }
    #[test]
    fn test_expr_depth_app() {
        let app = Expr::App(Box::new(Expr::BVar(0)), Box::new(Expr::BVar(1)));
        assert_eq!(expr_depth(&app), 1);
    }
    #[test]
    fn test_expr_size_leaf() {
        assert_eq!(expr_size(&Expr::BVar(0)), 1);
    }
    #[test]
    fn test_expr_size_app() {
        let app = Expr::App(Box::new(Expr::BVar(0)), Box::new(Expr::BVar(1)));
        assert_eq!(expr_size(&app), 3);
    }
    #[test]
    fn test_is_beta_normal_const() {
        assert!(is_beta_normal(&Expr::Const(Name::str("f"), vec![])));
    }
    #[test]
    fn test_is_beta_normal_app_not_lam() {
        let app = Expr::App(
            Box::new(Expr::Const(Name::str("f"), vec![])),
            Box::new(Expr::BVar(0)),
        );
        assert!(is_beta_normal(&app));
    }
    #[test]
    fn test_is_beta_normal_redex() {
        use crate::{BinderInfo, Level};
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(Expr::BVar(0)),
        );
        let redex = Expr::App(Box::new(lam), Box::new(Expr::BVar(0)));
        assert!(!is_beta_normal(&redex));
    }
    #[test]
    fn test_coercion_table_clear() {
        let mut table = CoercionTable::new();
        table.register(Coercion {
            name: Name::str("f"),
            source: Expr::Const(Name::str("A"), vec![]),
            target: Expr::Const(Name::str("B"), vec![]),
            priority: 1,
        });
        assert_eq!(table.len(), 1);
        table.clear();
        assert!(table.is_empty());
    }
    #[test]
    fn test_alpha_similar_app() {
        let e1 = Expr::App(Box::new(Expr::BVar(0)), Box::new(Expr::BVar(1)));
        let e2 = Expr::App(Box::new(Expr::BVar(0)), Box::new(Expr::BVar(1)));
        assert!(alpha_similar(&e1, &e2));
    }
    #[test]
    fn test_structural_distance_nested() {
        let e1 = Expr::App(Box::new(Expr::BVar(0)), Box::new(Expr::BVar(1)));
        let e2 = Expr::App(Box::new(Expr::BVar(0)), Box::new(Expr::BVar(2)));
        assert_eq!(structural_distance(&e1, &e2), 1);
    }
}
#[cfg(test)]
mod tests_padding_infra {
    use super::*;
    #[test]
    fn test_stat_summary() {
        let mut ss = StatSummary::new();
        ss.record(10.0);
        ss.record(20.0);
        ss.record(30.0);
        assert_eq!(ss.count(), 3);
        assert!((ss.mean().expect("mean should succeed") - 20.0).abs() < 1e-9);
        assert_eq!(ss.min().expect("min should succeed") as i64, 10);
        assert_eq!(ss.max().expect("max should succeed") as i64, 30);
    }
    #[test]
    fn test_transform_stat() {
        let mut ts = TransformStat::new();
        ts.record_before(100.0);
        ts.record_after(80.0);
        let ratio = ts.mean_ratio().expect("ratio should be present");
        assert!((ratio - 0.8).abs() < 1e-9);
    }
    #[test]
    fn test_small_map() {
        let mut m: SmallMap<u32, &str> = SmallMap::new();
        m.insert(3, "three");
        m.insert(1, "one");
        m.insert(2, "two");
        assert_eq!(m.get(&2), Some(&"two"));
        assert_eq!(m.len(), 3);
        let keys = m.keys();
        assert_eq!(*keys[0], 1);
        assert_eq!(*keys[2], 3);
    }
    #[test]
    fn test_label_set() {
        let mut ls = LabelSet::new();
        ls.add("foo");
        ls.add("bar");
        ls.add("foo");
        assert_eq!(ls.count(), 2);
        assert!(ls.has("bar"));
        assert!(!ls.has("baz"));
    }
    #[test]
    fn test_config_node() {
        let mut root = ConfigNode::section("root");
        let child = ConfigNode::leaf("key", "value");
        root.add_child(child);
        assert_eq!(root.num_children(), 1);
    }
    #[test]
    fn test_versioned_record() {
        let mut vr = VersionedRecord::new(0u32);
        vr.update(1);
        vr.update(2);
        assert_eq!(*vr.current(), 2);
        assert_eq!(vr.version(), 2);
        assert!(vr.has_history());
        assert_eq!(*vr.at_version(0).expect("value should be present"), 0);
    }
    #[test]
    fn test_simple_dag() {
        let mut dag = SimpleDag::new(4);
        dag.add_edge(0, 1);
        dag.add_edge(1, 2);
        dag.add_edge(2, 3);
        assert!(dag.can_reach(0, 3));
        assert!(!dag.can_reach(3, 0));
        let order = dag.topological_sort().expect("order should be present");
        assert_eq!(order, vec![0, 1, 2, 3]);
    }
    #[test]
    fn test_focus_stack() {
        let mut fs: FocusStack<&str> = FocusStack::new();
        fs.focus("a");
        fs.focus("b");
        assert_eq!(fs.current(), Some(&"b"));
        assert_eq!(fs.depth(), 2);
        fs.blur();
        assert_eq!(fs.current(), Some(&"a"));
    }
}
#[cfg(test)]
mod tests_extra_iterators {
    use super::*;
    #[test]
    fn test_window_iterator() {
        let data = vec![1u32, 2, 3, 4, 5];
        let windows: Vec<_> = WindowIterator::new(&data, 3).collect();
        assert_eq!(windows.len(), 3);
        assert_eq!(windows[0], &[1, 2, 3]);
        assert_eq!(windows[2], &[3, 4, 5]);
    }
    #[test]
    fn test_non_empty_vec() {
        let mut nev = NonEmptyVec::singleton(10u32);
        nev.push(20);
        nev.push(30);
        assert_eq!(nev.len(), 3);
        assert_eq!(*nev.first(), 10);
        assert_eq!(*nev.last(), 30);
    }
}
#[cfg(test)]
mod tests_padding2 {
    use super::*;
    #[test]
    fn test_sliding_sum() {
        let mut ss = SlidingSum::new(3);
        ss.push(1.0);
        ss.push(2.0);
        ss.push(3.0);
        assert!((ss.sum() - 6.0).abs() < 1e-9);
        ss.push(4.0);
        assert!((ss.sum() - 9.0).abs() < 1e-9);
        assert_eq!(ss.count(), 3);
    }
    #[test]
    fn test_path_buf() {
        let mut pb = PathBuf::new();
        pb.push("src");
        pb.push("main");
        assert_eq!(pb.as_str(), "src/main");
        assert_eq!(pb.depth(), 2);
        pb.pop();
        assert_eq!(pb.as_str(), "src");
    }
    #[test]
    fn test_string_pool() {
        let mut pool = StringPool::new();
        let s = pool.take();
        assert!(s.is_empty());
        pool.give("hello".to_string());
        let s2 = pool.take();
        assert!(s2.is_empty());
        assert_eq!(pool.free_count(), 0);
    }
    #[test]
    fn test_transitive_closure() {
        let mut tc = TransitiveClosure::new(4);
        tc.add_edge(0, 1);
        tc.add_edge(1, 2);
        tc.add_edge(2, 3);
        assert!(tc.can_reach(0, 3));
        assert!(!tc.can_reach(3, 0));
        let r = tc.reachable_from(0);
        assert_eq!(r.len(), 4);
    }
    #[test]
    fn test_token_bucket() {
        let mut tb = TokenBucket::new(100, 10);
        assert_eq!(tb.available(), 100);
        assert!(tb.try_consume(50));
        assert_eq!(tb.available(), 50);
        assert!(!tb.try_consume(60));
        assert_eq!(tb.capacity(), 100);
    }
    #[test]
    fn test_rewrite_rule_set() {
        let mut rrs = RewriteRuleSet::new();
        rrs.add(RewriteRule::unconditional(
            "beta",
            "App(Lam(x, b), v)",
            "b[x:=v]",
        ));
        rrs.add(RewriteRule::conditional("comm", "a + b", "b + a"));
        assert_eq!(rrs.len(), 2);
        assert_eq!(rrs.unconditional_rules().len(), 1);
        assert_eq!(rrs.conditional_rules().len(), 1);
        assert!(rrs.get("beta").is_some());
        let disp = rrs
            .get("beta")
            .expect("element at \'beta\' should exist")
            .display();
        assert!(disp.contains("→"));
    }
}
#[cfg(test)]
mod tests_padding3 {
    use super::*;
    #[test]
    fn test_decision_node() {
        let tree = DecisionNode::Branch {
            key: "x".into(),
            val: "1".into(),
            yes_branch: Box::new(DecisionNode::Leaf("yes".into())),
            no_branch: Box::new(DecisionNode::Leaf("no".into())),
        };
        let mut ctx = std::collections::HashMap::new();
        ctx.insert("x".into(), "1".into());
        assert_eq!(tree.evaluate(&ctx), "yes");
        ctx.insert("x".into(), "2".into());
        assert_eq!(tree.evaluate(&ctx), "no");
        assert_eq!(tree.depth(), 1);
    }
    #[test]
    fn test_flat_substitution() {
        let mut sub = FlatSubstitution::new();
        sub.add("foo", "bar");
        sub.add("baz", "qux");
        assert_eq!(sub.apply("foo and baz"), "bar and qux");
        assert_eq!(sub.len(), 2);
    }
    #[test]
    fn test_stopwatch() {
        let mut sw = Stopwatch::start();
        sw.split();
        sw.split();
        assert_eq!(sw.num_splits(), 2);
        assert!(sw.elapsed_ms() >= 0.0);
        for &s in sw.splits() {
            assert!(s >= 0.0);
        }
    }
    #[test]
    fn test_either2() {
        let e: Either2<i32, &str> = Either2::First(42);
        assert!(e.is_first());
        let mapped = e.map_first(|x| x * 2);
        assert_eq!(mapped.first(), Some(84));
        let e2: Either2<i32, &str> = Either2::Second("hello");
        assert!(e2.is_second());
        assert_eq!(e2.second(), Some("hello"));
    }
    #[test]
    fn test_write_once() {
        let wo: WriteOnce<u32> = WriteOnce::new();
        assert!(!wo.is_written());
        assert!(wo.write(42));
        assert!(!wo.write(99));
        assert_eq!(wo.read(), Some(42));
    }
    #[test]
    fn test_sparse_vec() {
        let mut sv: SparseVec<i32> = SparseVec::new(100);
        sv.set(5, 10);
        sv.set(50, 20);
        assert_eq!(*sv.get(5), 10);
        assert_eq!(*sv.get(50), 20);
        assert_eq!(*sv.get(0), 0);
        assert_eq!(sv.nnz(), 2);
        sv.set(5, 0);
        assert_eq!(sv.nnz(), 1);
    }
    #[test]
    fn test_stack_calc() {
        let mut calc = StackCalc::new();
        calc.push(3);
        calc.push(4);
        calc.add();
        assert_eq!(calc.peek(), Some(7));
        calc.push(2);
        calc.mul();
        assert_eq!(calc.peek(), Some(14));
    }
}
#[cfg(test)]
mod tests_final_padding {
    use super::*;
    #[test]
    fn test_min_heap() {
        let mut h = MinHeap::new();
        h.push(5u32);
        h.push(1u32);
        h.push(3u32);
        assert_eq!(h.peek(), Some(&1));
        assert_eq!(h.pop(), Some(1));
        assert_eq!(h.pop(), Some(3));
        assert_eq!(h.pop(), Some(5));
        assert!(h.is_empty());
    }
    #[test]
    fn test_prefix_counter() {
        let mut pc = PrefixCounter::new();
        pc.record("hello");
        pc.record("help");
        pc.record("world");
        assert_eq!(pc.count_with_prefix("hel"), 2);
        assert_eq!(pc.count_with_prefix("wor"), 1);
        assert_eq!(pc.count_with_prefix("xyz"), 0);
    }
    #[test]
    fn test_fixture() {
        let mut f = Fixture::new();
        f.set("key1", "val1");
        f.set("key2", "val2");
        assert_eq!(f.get("key1"), Some("val1"));
        assert_eq!(f.get("key3"), None);
        assert_eq!(f.len(), 2);
    }
}
#[cfg(test)]
mod tests_tiny_padding {
    use super::*;
    #[test]
    fn test_bitset64() {
        let mut bs = BitSet64::new();
        bs.insert(0);
        bs.insert(63);
        assert!(bs.contains(0));
        assert!(bs.contains(63));
        assert!(!bs.contains(1));
        assert_eq!(bs.len(), 2);
        bs.remove(0);
        assert!(!bs.contains(0));
    }
    #[test]
    fn test_bucket_counter() {
        let mut bc: BucketCounter<4> = BucketCounter::new();
        bc.inc(0);
        bc.inc(0);
        bc.inc(1);
        assert_eq!(bc.get(0), 2);
        assert_eq!(bc.total(), 3);
        assert_eq!(bc.argmax(), 0);
    }
}
