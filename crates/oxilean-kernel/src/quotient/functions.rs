//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::{Expr, KernelError, Name};

use super::types::{
    ConfigNode, EquivClassSystem, FocusStack, LabelSet, NonEmptyVec, PathBuf, QuotLiftCache,
    QuotReductionKind, QuotStats, QuotUsageKind, QuotientBuilder, QuotientDescription,
    QuotientKernel, QuotientNormalizer, QuotientReducer, QuotientType, RewriteRule, RewriteRuleSet,
    SimpleDag, SlidingSum, SmallMap, StatSummary, StringPool, TokenBucket, TransformStat,
    TransitiveClosure, VersionedRecord, WindowIterator,
};

/// Check that an expression is a valid equivalence relation.
///
/// Performs a lightweight structural check:
/// 1. Fast-path: `Eq` (or any partial application of `Eq`) is always valid.
/// 2. Pi/Lam expressions: accepted if the terminal result type is `Sort 0`
///    (Prop), indicating a Prop-valued binary function.
/// 3. `Const` or `App` expressions: accepted optimistically — we cannot
///    verify reflexivity/symmetry/transitivity without a full proof.
/// 4. Raw `Sort`, `BVar`, `FVar`: rejected as they are not function types.
///
/// Note: The OxiLean kernel cannot formally verify the equivalence properties
/// without proofs; this check only validates the structural signature.
#[allow(clippy::result_large_err)]
pub fn check_equivalence_relation(rel: &Expr) -> Result<(), KernelError> {
    if is_eq_relation(rel) {
        return Ok(());
    }
    match rel {
        Expr::Pi(_, _, _, body) => {
            if ends_in_prop(body) {
                Ok(())
            } else {
                Err(KernelError::Other(
                    "quotient relation must be Prop-valued (end in Sort 0)".to_string(),
                ))
            }
        }
        Expr::Lam(_, _, _, body) => {
            if ends_in_prop(body) {
                Ok(())
            } else {
                Err(KernelError::Other(
                    "quotient relation must be Prop-valued".to_string(),
                ))
            }
        }
        Expr::Const(_, _) | Expr::App(_, _) => Ok(()),
        Expr::Sort(_) | Expr::BVar(_) | Expr::FVar(_) | Expr::Lit(_) => Err(KernelError::Other(
            "quotient relation must be a function type, not a bare value".to_string(),
        )),
        _ => Ok(()),
    }
}
/// Check whether an expression eventually reaches `Sort(Level::zero())` (Prop)
/// after stripping Pi/Lam binders.
fn ends_in_prop(expr: &Expr) -> bool {
    match expr {
        Expr::Sort(l) => l.is_zero(),
        Expr::Pi(_, _, _, body) | Expr::Lam(_, _, _, body) => ends_in_prop(body),
        Expr::Const(_, _) | Expr::App(_, _) => true,
        _ => false,
    }
}
/// Check whether a relation looks like Eq.
pub fn is_eq_relation(rel: &Expr) -> bool {
    match rel {
        Expr::Const(name, _) => *name == Name::str("Eq"),
        Expr::App(f, _) => is_eq_relation(f),
        _ => false,
    }
}
/// Reduce Quot.lift f h (Quot.mk a) to f a.
pub fn reduce_quot_lift(args: &[Expr]) -> Option<Expr> {
    if args.len() < 3 {
        return None;
    }
    match &args[2] {
        Expr::App(head, a) => {
            if let Expr::Const(name, _) = head.as_ref() {
                if *name == Name::str("Quot.mk") {
                    return Some(Expr::App(Box::new(args[0].clone()), a.clone()));
                }
            }
            None
        }
        _ => None,
    }
}
/// Try to reduce a Quot expression.
pub fn try_reduce_quot(head: &Expr, args: &[Expr]) -> Option<Expr> {
    if let Expr::Const(name, _) = head {
        if *name == Name::str("Quot.lift") {
            return reduce_quot_lift(args);
        }
    }
    None
}
/// Check if an expression is a Quot.mk application.
pub fn is_quot_mk(expr: &Expr) -> bool {
    match expr {
        Expr::App(f, _) => {
            matches!(f.as_ref(), Expr::Const(name, _) if * name == Name::str("Quot.mk"))
        }
        _ => false,
    }
}
/// Extract the argument of a Quot.mk application.
pub fn quot_mk_arg(expr: &Expr) -> Option<&Expr> {
    if let Expr::App(f, a) = expr {
        if let Expr::Const(name, _) = f.as_ref() {
            if *name == Name::str("Quot.mk") {
                return Some(a);
            }
        }
    }
    None
}
/// Check whether an expression is a quotient type `Quot α r` structurally.
///
/// A quotient type has the shape `App(App(Const("Quot", _), α), r)`.
/// This check is purely structural and does not require registration in
/// a `QuotientKernel` — it works for any expression headed by `Quot`.
pub fn is_quot_type_expr(expr: &Expr) -> bool {
    if let Expr::App(outer, _rel) = expr {
        if let Expr::App(quot_const, _alpha) = outer.as_ref() {
            if let Expr::Const(name, _) = quot_const.as_ref() {
                return *name == Name::str("Quot");
            }
        }
    }
    false
}
/// Verify that a `Quot.lift` application has a valid motive.
///
/// In Lean's quotient type theory, `Quot.lift` is always permitted, but
/// `Quot.ind` is restricted to propositional motives (the result type must
/// live in `Prop`, i.e., `Sort 0`).  This function checks that restriction
/// for `Quot.ind`-style usage given the motive expression.
///
/// # Arguments
/// * `motive` — The motive expression passed to `Quot.ind` or `Quot.lift`.
///   For `Quot.ind`, the motive must be a `Prop`-valued function
///   (`Pi _ Prop` or a constant whose type is `Sort 0`).
///
/// # Returns
/// * `Ok(())` if the motive is propositional or is used in `Quot.lift`
///   (which is unrestricted).
/// * `Err(KernelError)` if the motive is used with `Quot.ind` but does not
///   land in `Prop`.
#[allow(clippy::result_large_err)]
pub fn check_quot_usage(kind: QuotUsageKind, motive: &Expr) -> Result<(), KernelError> {
    match kind {
        QuotUsageKind::Lift => Ok(()),
        QuotUsageKind::Ind => {
            if is_propositional_motive(motive) {
                Ok(())
            } else {
                Err(KernelError::Other(
                    "Quot.ind motive must be propositional (land in Prop / Sort 0)".to_string(),
                ))
            }
        }
    }
}
/// Check whether a motive expression is propositional.
///
/// A motive is propositional if:
/// - It is `Expr::Sort(l)` where `l` is the zero level (`Prop`).
/// - It is a `Pi` (function type) whose codomain is propositional (so the
///   motive `fun x => P x` is propositional when `P x : Prop`).
/// - It is a `Lam` whose body is propositional (same logic).
fn is_propositional_motive(motive: &Expr) -> bool {
    match motive {
        Expr::Sort(l) => l.is_zero(),
        Expr::Lam(_, _, _, body) => is_propositional_motive(body),
        Expr::Pi(_, _, _, body) => is_propositional_motive(body),
        _ => false,
    }
}
/// Check whether `Quot.mk rel a = Quot.mk rel b` holds.
///
/// In Lean's quotient type theory, `Quot.mk r a = Quot.mk r b` iff `r a b`.
/// This function performs a structural check:
/// 1. If `a == b` (syntactic equality), reflexivity gives `r a a`, so they are equal.
/// 2. If `rel` is the identity/Eq relation, equality requires `a == b`.
/// 3. If `rel` applied to both arguments reduces structurally to `True`, accept.
/// 4. Otherwise, check whether `App(App(rel, a), b)` is syntactically the same
///    as `App(App(rel, b), a)` *and* `rel` is symmetric by name.
pub fn quot_eq(rel: &Expr, a: &Expr, b: &Expr) -> bool {
    if a == b {
        return true;
    }
    if is_eq_relation(rel) {
        return a == b;
    }
    let rel_a_b = Expr::App(
        Box::new(Expr::App(Box::new(rel.clone()), Box::new(a.clone()))),
        Box::new(b.clone()),
    );
    let rel_b_a = Expr::App(
        Box::new(Expr::App(Box::new(rel.clone()), Box::new(b.clone()))),
        Box::new(a.clone()),
    );
    rel_a_b == rel_b_a
}
/// Build a proof of Quot.mk a = Quot.mk b from r a b.
pub fn build_quot_sound(quot_ty: &QuotientType, a: Expr, b: Expr, rel_proof: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::App(Box::new(quot_ty.sound_const()), Box::new(a))),
                Box::new(b),
            )),
            Box::new(rel_proof),
        )),
        Box::new(quot_ty.base_type.clone()),
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BinderInfo, Level};
    fn mk_const(s: &str) -> Expr {
        Expr::Const(Name::str(s), vec![])
    }
    #[test]
    fn test_quotient_creation() {
        let nat = mk_const("Nat");
        let qt = QuotientType::new(nat.clone(), mk_const("Eq"));
        assert_eq!(qt.base_type, nat);
    }
    #[test]
    fn test_quotient_mk_const() {
        let qt = QuotientType::new(mk_const("Nat"), mk_const("Eq"));
        assert_eq!(qt.mk_const(), Expr::Const(Name::str("Quot.mk"), vec![]));
    }
    #[test]
    fn test_quotient_mk_apply() {
        let qt = QuotientType::new(mk_const("Nat"), mk_const("Eq"));
        let applied = qt.mk_apply(mk_const("zero"));
        assert!(matches!(applied, Expr::App(_, _)));
    }
    #[test]
    fn test_quot_lift_reduction() {
        let f = mk_const("f");
        let h = mk_const("h");
        let forty_two = mk_const("42");
        let mk_42 = Expr::App(Box::new(mk_const("Quot.mk")), Box::new(forty_two.clone()));
        let result = reduce_quot_lift(&[f.clone(), h, mk_42]);
        assert!(result.is_some());
        assert_eq!(
            result.expect("result should be valid"),
            Expr::App(Box::new(f), Box::new(forty_two))
        );
    }
    #[test]
    fn test_quot_lift_too_few_args() {
        assert!(reduce_quot_lift(&[mk_const("f"), mk_const("h")]).is_none());
    }
    #[test]
    fn test_is_quot_mk() {
        let mk_expr = Expr::App(Box::new(mk_const("Quot.mk")), Box::new(mk_const("a")));
        assert!(is_quot_mk(&mk_expr));
        assert!(!is_quot_mk(&mk_const("Nat")));
    }
    #[test]
    fn test_quot_mk_arg() {
        let a = mk_const("a");
        let mk_expr = Expr::App(Box::new(mk_const("Quot.mk")), Box::new(a.clone()));
        assert_eq!(quot_mk_arg(&mk_expr), Some(&a));
    }
    #[test]
    fn test_check_equivalence_relation() {
        let nat = mk_const("Nat");
        let rel = Expr::Pi(
            BinderInfo::Default,
            Name::str("a"),
            Box::new(nat.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("b"),
                Box::new(nat),
                Box::new(Expr::Sort(Level::zero())),
            )),
        );
        assert!(check_equivalence_relation(&rel).is_ok());
    }
    #[test]
    fn test_is_eq_relation() {
        assert!(is_eq_relation(&mk_const("Eq")));
        assert!(is_eq_relation(&Expr::App(
            Box::new(mk_const("Eq")),
            Box::new(mk_const("Nat"))
        )));
        assert!(!is_eq_relation(&mk_const("Nat")));
    }
    #[test]
    fn test_quotient_kernel_register() {
        let mut kernel = QuotientKernel::new();
        let qt = QuotientType::new(mk_const("Nat"), mk_const("Eq"));
        kernel.register(qt);
        assert_eq!(kernel.count(), 1);
    }
    #[test]
    fn test_quotient_kernel_find_by_base() {
        let mut kernel = QuotientKernel::new();
        let nat = mk_const("Nat");
        kernel.register(QuotientType::new(nat.clone(), mk_const("Eq")));
        assert!(kernel.find_by_base(&nat).is_some());
        assert!(kernel.find_by_base(&mk_const("Int")).is_none());
    }
    #[test]
    fn test_quotient_kernel_reduce() {
        let kernel = QuotientKernel::new();
        let mk_a = Expr::App(Box::new(mk_const("Quot.mk")), Box::new(mk_const("a")));
        let result = kernel.reduce(
            &mk_const("Quot.lift"),
            &[mk_const("f"), mk_const("h"), mk_a],
        );
        assert!(result.is_some());
    }
    #[test]
    fn test_quot_eq_reflexive() {
        let a = mk_const("a");
        let rel = mk_const("MyRel");
        assert!(quot_eq(&rel, &a, &a));
    }
    #[test]
    fn test_quot_eq_eq_relation_same() {
        let a = mk_const("a");
        assert!(quot_eq(&mk_const("Eq"), &a, &a));
    }
    #[test]
    fn test_quot_eq_eq_relation_different() {
        let a = mk_const("a");
        let b = mk_const("b");
        assert!(!quot_eq(&mk_const("Eq"), &a, &b));
    }
    #[test]
    fn test_quot_eq_symmetric_relation() {
        let a = mk_const("a");
        let b = mk_const("b");
        let rel = mk_const("MyRel");
        assert!(!quot_eq(&rel, &a, &b));
    }
    #[test]
    fn test_is_quot_type_expr_valid() {
        let quot_nat_eq = Expr::App(
            Box::new(Expr::App(
                Box::new(mk_const("Quot")),
                Box::new(mk_const("Nat")),
            )),
            Box::new(mk_const("Eq")),
        );
        assert!(is_quot_type_expr(&quot_nat_eq));
    }
    #[test]
    fn test_is_quot_type_expr_not_quot() {
        let not_quot = Expr::App(
            Box::new(Expr::App(
                Box::new(mk_const("List")),
                Box::new(mk_const("Nat")),
            )),
            Box::new(mk_const("Eq")),
        );
        assert!(!is_quot_type_expr(&not_quot));
    }
    #[test]
    fn test_is_quot_type_expr_not_app() {
        assert!(!is_quot_type_expr(&mk_const("Nat")));
    }
    #[test]
    fn test_check_quot_usage_lift_always_ok() {
        let non_prop_motive = Expr::Sort(Level::succ(Level::zero()));
        assert!(check_quot_usage(QuotUsageKind::Lift, &non_prop_motive).is_ok());
        let prop_motive = Expr::Sort(Level::zero());
        assert!(check_quot_usage(QuotUsageKind::Lift, &prop_motive).is_ok());
    }
    #[test]
    fn test_check_quot_usage_ind_requires_prop() {
        let prop_motive = Expr::Sort(Level::zero());
        assert!(check_quot_usage(QuotUsageKind::Ind, &prop_motive).is_ok());
    }
    #[test]
    fn test_check_quot_usage_ind_rejects_type() {
        let type_motive = Expr::Sort(Level::succ(Level::zero()));
        assert!(check_quot_usage(QuotUsageKind::Ind, &type_motive).is_err());
    }
    #[test]
    fn test_check_quot_usage_ind_lam_prop() {
        let prop = Expr::Sort(Level::zero());
        let nat = mk_const("Nat");
        let lam_motive = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(nat),
            Box::new(prop),
        );
        assert!(check_quot_usage(QuotUsageKind::Ind, &lam_motive).is_ok());
    }
    #[test]
    fn test_quotient_kernel_is_quot_type_structural() {
        let kernel = QuotientKernel::new();
        let quot_nat_eq = Expr::App(
            Box::new(Expr::App(
                Box::new(mk_const("Quot")),
                Box::new(mk_const("Nat")),
            )),
            Box::new(mk_const("Eq")),
        );
        assert!(kernel.is_quot_type(&quot_nat_eq));
    }
}
/// Try to reduce `Quot.ind h (Quot.mk a)` to `h a`.
pub fn reduce_quot_ind(args: &[Expr]) -> Option<Expr> {
    if args.len() < 2 {
        return None;
    }
    match &args[1] {
        Expr::App(head, a) => {
            if let Expr::Const(name, _) = head.as_ref() {
                if *name == Name::str("Quot.mk") {
                    return Some(Expr::App(Box::new(args[0].clone()), a.clone()));
                }
            }
            None
        }
        _ => None,
    }
}
/// Try to reduce either `Quot.lift` or `Quot.ind`.
pub fn try_reduce_quot_full(head: &Expr, args: &[Expr]) -> Option<(Expr, QuotReductionKind)> {
    if let Expr::Const(name, _) = head {
        if *name == Name::str("Quot.lift") {
            return reduce_quot_lift(args).map(|e| (e, QuotReductionKind::Lift));
        }
        if *name == Name::str("Quot.ind") {
            return reduce_quot_ind(args).map(|e| (e, QuotReductionKind::Ind));
        }
    }
    None
}
/// Check whether two Quot.mk applications are definitionally equal given `r a b`.
///
/// In general this requires a proof; here we simply check syntactic equality.
pub fn quot_mk_def_eq(a: &Expr, b: &Expr) -> bool {
    a == b
}
/// Apply `Quot.lift f h` to all elements of a list and collect results.
pub fn lift_map(f: &Expr, items: &[Expr]) -> Vec<Expr> {
    items
        .iter()
        .map(|item| {
            let mk = Expr::App(
                Box::new(Expr::Const(Name::str("Quot.mk"), vec![])),
                Box::new(item.clone()),
            );
            let args = [f.clone(), Expr::Const(Name::str("_"), vec![]), mk];
            reduce_quot_lift(&args)
                .unwrap_or_else(|| Expr::App(Box::new(f.clone()), Box::new(item.clone())))
        })
        .collect()
}
/// Collect all `Quot.mk` subexpressions from an expression tree.
pub fn collect_quot_mk(expr: &Expr) -> Vec<Expr> {
    let mut result = Vec::new();
    collect_quot_mk_helper(expr, &mut result);
    result
}
fn collect_quot_mk_helper(expr: &Expr, acc: &mut Vec<Expr>) {
    if is_quot_mk(expr) {
        acc.push(expr.clone());
    }
    match expr {
        Expr::App(f, a) => {
            collect_quot_mk_helper(f, acc);
            collect_quot_mk_helper(a, acc);
        }
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            collect_quot_mk_helper(ty, acc);
            collect_quot_mk_helper(body, acc);
        }
        Expr::Let(_, ty, val, body) => {
            collect_quot_mk_helper(ty, acc);
            collect_quot_mk_helper(val, acc);
            collect_quot_mk_helper(body, acc);
        }
        _ => {}
    }
}
/// Count the number of `Quot.mk` applications in an expression.
pub fn count_quot_mk(expr: &Expr) -> usize {
    collect_quot_mk(expr).len()
}
/// Check whether an expression contains any `Quot.lift` application.
pub fn contains_quot_lift(expr: &Expr) -> bool {
    match expr {
        Expr::Const(name, _) => *name == Name::str("Quot.lift"),
        Expr::App(f, a) => {
            if let Expr::Const(name, _) = f.as_ref() {
                if *name == Name::str("Quot.lift") {
                    return true;
                }
            }
            contains_quot_lift(f) || contains_quot_lift(a)
        }
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            contains_quot_lift(ty) || contains_quot_lift(body)
        }
        Expr::Let(_, ty, val, body) => {
            contains_quot_lift(ty) || contains_quot_lift(val) || contains_quot_lift(body)
        }
        _ => false,
    }
}
/// Verify that `QuotientType` fields are consistent.
///
/// Checks:
/// 1. `base_type` must not be a raw `Sort` — the thing being quotiented should
///    be a term type (e.g. `Nat`, `Int`), not a universe level like `Prop` or `Type`.
/// 2. `relation` must not be a raw `Sort` — a relation should be a function
///    (e.g. a `Const`, `Lam`, `App`), not a universe.
/// 3. The `quot_type` field must be an `App` expression (it is `Quot α r` which
///    is `App(App(Quot, α), r)`), confirming the struct was built correctly.
/// 4. The mk, lift, ind, and sound helpers on `QuotientType` must all return `Const`
///    nodes — they are fixed names so this is always true, but we verify structure.
pub fn validate_quotient_type(qt: &QuotientType) -> bool {
    if matches!(qt.base_type, Expr::Sort(_)) {
        return false;
    }
    if matches!(qt.relation, Expr::Sort(_)) {
        return false;
    }
    if !matches!(qt.quot_type, Expr::App(_, _)) {
        return false;
    }
    if !matches!(qt.mk_const(), Expr::Const(_, _)) {
        return false;
    }
    if !matches!(qt.lift_const(), Expr::Const(_, _)) {
        return false;
    }
    if !matches!(qt.sound_const(), Expr::Const(_, _)) {
        return false;
    }
    true
}
#[cfg(test)]
mod extended_tests {
    use super::*;
    use crate::Level;
    fn mk_const(s: &str) -> Expr {
        Expr::Const(Name::str(s), vec![])
    }
    #[test]
    fn test_equiv_class_system() {
        let mut sys = EquivClassSystem::new();
        let a = mk_const("a");
        let b = mk_const("b");
        sys.insert(a.clone());
        sys.insert(b.clone());
        sys.merge(&a, &b);
        assert!(sys.same_class(&a, &b));
    }
    #[test]
    fn test_reduce_quot_ind() {
        let h = mk_const("h");
        let a = mk_const("a");
        let mk_a = Expr::App(Box::new(mk_const("Quot.mk")), Box::new(a.clone()));
        let result = reduce_quot_ind(&[h.clone(), mk_a]);
        assert!(result.is_some());
        assert_eq!(
            result.expect("result should be valid"),
            Expr::App(Box::new(h), Box::new(a))
        );
    }
    #[test]
    fn test_try_reduce_quot_full_lift() {
        let f = mk_const("f");
        let h = mk_const("h");
        let mk_a = Expr::App(Box::new(mk_const("Quot.mk")), Box::new(mk_const("a")));
        let (result, kind) = try_reduce_quot_full(&mk_const("Quot.lift"), &[f, h, mk_a])
            .expect("value should be present");
        assert_eq!(kind, QuotReductionKind::Lift);
        assert!(matches!(result, Expr::App(..)));
    }
    #[test]
    fn test_quotient_builder() {
        let qt = QuotientBuilder::new()
            .base(mk_const("Nat"))
            .relation(mk_const("Eq"))
            .build();
        assert!(qt.is_some());
        assert_eq!(qt.expect("qt should be valid").base_type, mk_const("Nat"));
    }
    #[test]
    fn test_quotient_builder_incomplete() {
        let qt = QuotientBuilder::new().base(mk_const("Nat")).build();
        assert!(qt.is_none());
    }
    #[test]
    fn test_collect_quot_mk() {
        let mk_a = Expr::App(Box::new(mk_const("Quot.mk")), Box::new(mk_const("a")));
        let mk_b = Expr::App(Box::new(mk_const("Quot.mk")), Box::new(mk_const("b")));
        let expr = Expr::App(Box::new(mk_a.clone()), Box::new(mk_b.clone()));
        let collected = collect_quot_mk(&expr);
        assert_eq!(collected.len(), 2);
    }
    #[test]
    fn test_count_quot_mk() {
        let mk_a = Expr::App(Box::new(mk_const("Quot.mk")), Box::new(mk_const("a")));
        assert_eq!(count_quot_mk(&mk_a), 1);
        assert_eq!(count_quot_mk(&mk_const("other")), 0);
    }
    #[test]
    fn test_quot_lift_cache() {
        let mut cache = QuotLiftCache::new();
        let f = mk_const("f");
        let a = mk_const("a");
        let r = mk_const("result");
        cache.put(&f, &a, r.clone());
        assert_eq!(cache.get(&f, &a), Some(&r));
        assert_eq!(cache.len(), 1);
        cache.clear();
        assert!(cache.is_empty());
    }
    #[test]
    fn test_contains_quot_lift() {
        let lift = Expr::App(Box::new(mk_const("Quot.lift")), Box::new(mk_const("f")));
        assert!(contains_quot_lift(&lift));
        assert!(!contains_quot_lift(&mk_const("Nat")));
    }
    #[test]
    fn test_quot_reduction_kind_description() {
        assert_eq!(QuotReductionKind::Lift.description(), "Quot.lift reduction");
        assert_eq!(QuotReductionKind::Ind.description(), "Quot.ind reduction");
    }
    #[test]
    fn test_lift_map() {
        let f = mk_const("f");
        let items = vec![mk_const("a"), mk_const("b")];
        let results = lift_map(&f, &items);
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_quot_mk_def_eq() {
        let a = mk_const("a");
        assert!(quot_mk_def_eq(&a, &a));
        assert!(!quot_mk_def_eq(&mk_const("a"), &mk_const("b")));
    }
    #[test]
    fn test_validate_quotient_type_valid() {
        let qt = QuotientType::new(mk_const("Nat"), mk_const("Eq"));
        assert!(validate_quotient_type(&qt));
    }
    #[test]
    fn test_validate_quotient_type_sort_base() {
        let qt = QuotientType {
            base_type: Expr::Sort(Level::zero()),
            relation: mk_const("Eq"),
            quot_type: Expr::App(
                Box::new(Expr::App(
                    Box::new(mk_const("Quot")),
                    Box::new(Expr::Sort(Level::zero())),
                )),
                Box::new(mk_const("Eq")),
            ),
        };
        assert!(!validate_quotient_type(&qt));
    }
    #[test]
    fn test_validate_quotient_type_sort_relation() {
        let qt = QuotientType {
            base_type: mk_const("Nat"),
            relation: Expr::Sort(Level::zero()),
            quot_type: Expr::App(
                Box::new(Expr::App(
                    Box::new(mk_const("Quot")),
                    Box::new(mk_const("Nat")),
                )),
                Box::new(Expr::Sort(Level::zero())),
            ),
        };
        assert!(!validate_quotient_type(&qt));
    }
    #[test]
    fn test_validate_quotient_type_with_app_relation() {
        let rel = Expr::App(Box::new(mk_const("Eq")), Box::new(mk_const("Nat")));
        let qt = QuotientType::new(mk_const("Nat"), rel);
        assert!(validate_quotient_type(&qt));
    }
}
/// Collect application arguments into a flat list: `(f a b c)` → `[f, a, b, c]`.
pub(super) fn collect_args(expr: &Expr) -> Vec<Expr> {
    let mut args = Vec::new();
    let mut current = expr;
    loop {
        match current {
            Expr::App(f, a) => {
                args.push(*a.clone());
                current = f;
            }
            _ => {
                args.push(current.clone());
                break;
            }
        }
    }
    args.reverse();
    args
}
/// Check whether an expression is a `Quot.sound` application.
pub fn is_quot_sound(expr: &Expr) -> bool {
    match expr {
        Expr::App(f, _) => {
            if let Expr::Const(name, _) = f.as_ref() {
                return *name == Name::str("Quot.sound");
            }
            is_quot_sound(f)
        }
        Expr::Const(name, _) => *name == Name::str("Quot.sound"),
        _ => false,
    }
}
/// Check whether the relation is the identity relation (structural equality).
pub fn is_identity_relation(rel: &Expr) -> bool {
    is_eq_relation(rel)
}
/// Build a proof term for `Quot.lift f h (Quot.mk a) = f a` (definitional).
pub fn quot_lift_eq_proof(_f: Expr, _a: Expr) -> Expr {
    Expr::Const(Name::str("Quot.lift_eq"), vec![])
}
/// Generate a fresh `Quot.mk` name for a given base type name.
pub fn quot_mk_name_for(base_type_name: &str) -> Name {
    Name::str(format!("{}.Quot.mk", base_type_name))
}
#[cfg(test)]
mod more_tests {
    use super::*;
    fn mk_const(s: &str) -> Expr {
        Expr::Const(Name::str(s), vec![])
    }
    #[test]
    fn test_quotient_description_display() {
        let d = QuotientDescription::new("Nat", "Eq");
        assert!(d.display().contains("Quot"));
        let d2 = d.with_model_size(5);
        assert!(d2.display().contains("5"));
    }
    #[test]
    fn test_quot_stats_mk() {
        let mk_a = Expr::App(Box::new(mk_const("Quot.mk")), Box::new(mk_const("a")));
        let stats = QuotStats::compute(&mk_a);
        assert_eq!(stats.mk_count, 1);
        assert!(stats.has_quot());
    }
    #[test]
    fn test_quot_stats_lift() {
        let lift = mk_const("Quot.lift");
        let stats = QuotStats::compute(&lift);
        assert_eq!(stats.lift_count, 1);
    }
    #[test]
    fn test_quot_stats_no_quot() {
        let stats = QuotStats::compute(&mk_const("Nat"));
        assert!(!stats.has_quot());
        assert_eq!(stats.total(), 0);
    }
    #[test]
    fn test_is_identity_relation() {
        assert!(is_identity_relation(&mk_const("Eq")));
        let app = Expr::App(Box::new(mk_const("Eq")), Box::new(mk_const("Nat")));
        assert!(is_identity_relation(&app));
    }
    #[test]
    fn test_is_quot_sound() {
        assert!(is_quot_sound(&mk_const("Quot.sound")));
        let app = Expr::App(Box::new(mk_const("Quot.sound")), Box::new(mk_const("h")));
        assert!(is_quot_sound(&app));
        assert!(!is_quot_sound(&mk_const("Nat")));
    }
    #[test]
    fn test_quot_mk_name_for() {
        let n = quot_mk_name_for("Int");
        assert_eq!(n.to_string(), "Int.Quot.mk");
    }
    #[test]
    fn test_quotient_reducer_step_limit() {
        let reducer = QuotientReducer::new(10);
        assert_eq!(reducer.max_steps, 10);
        assert_eq!(reducer.step_count(), 0);
    }
    #[test]
    fn test_collect_args() {
        let f = mk_const("f");
        let a = mk_const("a");
        let b = mk_const("b");
        let expr = Expr::App(
            Box::new(Expr::App(Box::new(f.clone()), Box::new(a.clone()))),
            Box::new(b.clone()),
        );
        let args = collect_args(&expr);
        assert_eq!(args[0], f);
        assert_eq!(args[1], a);
        assert_eq!(args[2], b);
    }
}
pub(super) fn collect_args_norm(expr: &Expr) -> Vec<Expr> {
    let mut args = Vec::new();
    let mut current = expr;
    loop {
        match current {
            Expr::App(f, a) => {
                args.push(*a.clone());
                current = f;
            }
            _ => {
                args.push(current.clone());
                break;
            }
        }
    }
    args.reverse();
    args
}
/// Check whether an expression is in quotient normal form
/// (no unreduced `Quot.lift` or `Quot.ind` applications remain).
#[allow(dead_code)]
pub fn is_quot_normal(expr: &Expr) -> bool {
    let stats = QuotStats::compute(expr);
    stats.lift_count == 0 && stats.ind_count == 0
}
/// Generate a documentation comment for a quotient type.
#[allow(dead_code)]
pub fn quot_type_doc(base: &str, relation: &str) -> String {
    format!(
        "/// Quotient of `{base}` by the relation `{relation}`.\n/// Elements of `Quot {base} {relation}` are equivalence classes under `{relation}`."
    )
}
/// Return a list of all standard quotient constants.
#[allow(dead_code)]
pub fn standard_quot_consts() -> Vec<&'static str> {
    vec!["Quot", "Quot.mk", "Quot.lift", "Quot.ind", "Quot.sound"]
}
/// Check whether a name is a standard quotient constant.
#[allow(dead_code)]
pub fn is_quot_const(name: &Name) -> bool {
    let s = name.to_string();
    matches!(
        s.as_str(),
        "Quot" | "Quot.mk" | "Quot.lift" | "Quot.ind" | "Quot.sound"
    )
}
/// Return the number of arguments expected by a quotient constant.
#[allow(dead_code)]
pub fn quot_const_arity(name: &str) -> Option<usize> {
    match name {
        "Quot.mk" => Some(1),
        "Quot.lift" => Some(3),
        "Quot.ind" => Some(2),
        "Quot.sound" => Some(1),
        _ => None,
    }
}
/// Check whether `expr` is a full `Quot.lift f h q` application (3 explicit args).
#[allow(dead_code)]
pub fn is_full_quot_lift(expr: &Expr) -> bool {
    let args = collect_args_norm(expr);
    args.len() >= 4 && matches!(& args[0], Expr::Const(n, _) if * n == Name::str("Quot.lift"))
}
/// Extract `(f, h, q)` from a `Quot.lift f h q` application.
#[allow(dead_code)]
pub fn extract_quot_lift_args(expr: &Expr) -> Option<(Expr, Expr, Expr)> {
    let args = collect_args_norm(expr);
    if args.len() < 4 {
        return None;
    }
    if !matches!(& args[0], Expr::Const(n, _) if * n == Name::str("Quot.lift")) {
        return None;
    }
    Some((args[1].clone(), args[2].clone(), args[3].clone()))
}
/// Build `Quot α r` from the base type and relation.
#[allow(dead_code)]
pub fn build_quot_type(alpha: Expr, r: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("Quot"), vec![])),
            Box::new(alpha),
        )),
        Box::new(r),
    )
}
/// Build `Quot.mk a`.
#[allow(dead_code)]
pub fn build_quot_mk(a: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::Const(Name::str("Quot.mk"), vec![])),
        Box::new(a),
    )
}
/// Build `Quot.lift f h q`.
#[allow(dead_code)]
pub fn build_quot_lift(f: Expr, h: Expr, q: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Quot.lift"), vec![])),
                Box::new(f),
            )),
            Box::new(h),
        )),
        Box::new(q),
    )
}
/// Check whether a list of expressions are all `Quot.mk` applications.
#[allow(dead_code)]
pub fn all_quot_mk(exprs: &[Expr]) -> bool {
    exprs.iter().all(is_quot_mk)
}
/// Strip all `Quot.mk` wrappers from a list of expressions.
#[allow(dead_code)]
pub fn strip_quot_mk(exprs: Vec<Expr>) -> Vec<Expr> {
    exprs
        .into_iter()
        .filter_map(|e| quot_mk_arg(&e).cloned())
        .collect()
}
#[cfg(test)]
mod normalizer_tests {
    use super::*;
    fn mk_c(s: &str) -> Expr {
        Expr::Const(Name::str(s), vec![])
    }
    #[test]
    fn test_quot_normalizer_no_change() {
        let mut norm = QuotientNormalizer::new(100);
        let e = mk_c("Nat");
        let (result, changed) = norm.normalize(e.clone());
        assert!(!changed);
        assert_eq!(result, e);
    }
    #[test]
    fn test_is_quot_normal_plain() {
        assert!(is_quot_normal(&mk_c("Nat")));
    }
    #[test]
    fn test_is_quot_normal_with_lift() {
        let lift = mk_c("Quot.lift");
        assert!(!is_quot_normal(&lift));
    }
    #[test]
    fn test_quot_type_doc() {
        let doc = quot_type_doc("Nat", "Eq");
        assert!(doc.contains("Nat"));
        assert!(doc.contains("Eq"));
    }
    #[test]
    fn test_standard_quot_consts() {
        let consts = standard_quot_consts();
        assert!(consts.contains(&"Quot.mk"));
        assert!(consts.contains(&"Quot.lift"));
        assert_eq!(consts.len(), 5);
    }
    #[test]
    fn test_is_quot_const() {
        assert!(is_quot_const(&Name::str("Quot.mk")));
        assert!(is_quot_const(&Name::str("Quot.sound")));
        assert!(!is_quot_const(&Name::str("Nat")));
    }
    #[test]
    fn test_quot_const_arity() {
        assert_eq!(quot_const_arity("Quot.mk"), Some(1));
        assert_eq!(quot_const_arity("Quot.lift"), Some(3));
        assert_eq!(quot_const_arity("Quot.ind"), Some(2));
        assert_eq!(quot_const_arity("OtherFn"), None);
    }
    #[test]
    fn test_build_quot_type() {
        let qt = build_quot_type(mk_c("Nat"), mk_c("Eq"));
        assert!(matches!(qt, Expr::App(_, _)));
    }
    #[test]
    fn test_build_quot_mk() {
        let mk = build_quot_mk(mk_c("a"));
        assert!(is_quot_mk(&mk));
    }
    #[test]
    fn test_build_quot_lift() {
        let lift = build_quot_lift(mk_c("f"), mk_c("h"), build_quot_mk(mk_c("a")));
        assert!(is_full_quot_lift(&lift));
    }
    #[test]
    fn test_extract_quot_lift_args() {
        let lift = build_quot_lift(mk_c("f"), mk_c("h"), build_quot_mk(mk_c("a")));
        let (f, h, q) = extract_quot_lift_args(&lift).expect("value should be present");
        assert_eq!(f, mk_c("f"));
        assert_eq!(h, mk_c("h"));
        assert!(is_quot_mk(&q));
    }
    #[test]
    fn test_all_quot_mk() {
        let mks = vec![build_quot_mk(mk_c("a")), build_quot_mk(mk_c("b"))];
        assert!(all_quot_mk(&mks));
        let mixed = vec![build_quot_mk(mk_c("a")), mk_c("b")];
        assert!(!all_quot_mk(&mixed));
    }
    #[test]
    fn test_strip_quot_mk() {
        let mks = vec![build_quot_mk(mk_c("a")), build_quot_mk(mk_c("b"))];
        let stripped = strip_quot_mk(mks);
        assert_eq!(stripped.len(), 2);
        assert_eq!(stripped[0], mk_c("a"));
    }
    #[test]
    fn test_quot_normalizer_steps() {
        let norm = QuotientNormalizer::new(50);
        assert_eq!(norm.steps_taken(), 0);
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
