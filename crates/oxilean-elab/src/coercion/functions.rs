//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Expr, Level, Name};

use super::types::{
    Coercion, CoercionAppStats, CoercionCache, CoercionEventKind, CoercionEventLog, CoercionGraph,
    CoercionGraphRegistry, CoercionInferenceHint, CoercionKind, CoercionNormalizer, CoercionPath,
    CoercionPrettyPrinter, CoercionRegistry, CoercionScope, CoercionScopeStack, CoercionStats,
    CoercionTypeClass, CoercionValidationError, CoercionValidator, FunctionCoercion,
    NormalizationStrategy, SortCoercionDecl, SortCoercionDirection, TypeClassCoercionRegistry,
};

/// Try to insert a coercion so that `expr : actual_ty` becomes `expected_ty`.
///
/// Returns `Some(coerced_expr)` on success, `None` if no coercion exists.
#[allow(dead_code)]
pub fn try_insert_coercion(
    registry: &CoercionRegistry,
    expr: Expr,
    actual_ty: &Expr,
    expected_ty: &Expr,
) -> Option<Expr> {
    if actual_ty == expected_ty {
        return Some(expr);
    }
    if let Some(coercion) = registry.find_coercion(actual_ty, expected_ty) {
        return Some(Expr::App(
            Box::new(Expr::Const(coercion.coerce.clone(), vec![])),
            Box::new(expr),
        ));
    }
    if let Some(path) = registry.find_coercion_chain(actual_ty, expected_ty) {
        return Some(registry.apply_coercion_chain(expr, &path));
    }
    if let Some(coerced) = coerce_to_sort(registry, expr.clone(), actual_ty, expected_ty) {
        return Some(coerced);
    }
    if let Some(coerced) = coerce_to_function(registry, expr.clone(), actual_ty, expected_ty) {
        return Some(coerced);
    }
    None
}
/// Coerce an expression to a sort when the expected type is a Sort.
///
/// E.g. when `Bool` appears where `Prop` is expected.
#[allow(dead_code)]
pub fn coerce_to_sort(
    registry: &CoercionRegistry,
    expr: Expr,
    actual_ty: &Expr,
    expected_ty: &Expr,
) -> Option<Expr> {
    if !expected_ty.is_sort() {
        return None;
    }
    registry.find_coercion(actual_ty, expected_ty).map(|c| {
        Expr::App(
            Box::new(Expr::Const(c.coerce.clone(), vec![])),
            Box::new(expr),
        )
    })
}
/// Coerce an expression to a function type when the expected type is Pi.
///
/// If the target is `Pi _ : A, B` and the source is not a Pi, look for
/// a coercion from the source type to a function type.
#[allow(dead_code)]
pub fn coerce_to_function(
    registry: &CoercionRegistry,
    expr: Expr,
    actual_ty: &Expr,
    expected_ty: &Expr,
) -> Option<Expr> {
    if !expected_ty.is_pi() || actual_ty.is_pi() {
        return None;
    }
    for c in registry.all_coercions() {
        if &c.from == actual_ty && c.to.is_pi() {
            return Some(Expr::App(
                Box::new(Expr::Const(c.coerce.clone(), vec![])),
                Box::new(expr),
            ));
        }
    }
    None
}
/// Apply a coercion to lift an expression from one sort level to another.
#[allow(dead_code)]
pub fn lift_sort(expr: Expr, from_level: &Level, to_level: &Level) -> Option<Expr> {
    if from_level == to_level {
        return Some(expr);
    }
    Some(Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("sortLift"), vec![])),
            Box::new(Expr::Sort(from_level.clone())),
        )),
        Box::new(expr),
    ))
}
/// Create a coercion expression `fun_name arg`.
#[allow(dead_code)]
pub fn mk_coercion_app(fun_name: Name, arg: Expr) -> Expr {
    Expr::App(Box::new(Expr::Const(fun_name, vec![])), Box::new(arg))
}
/// Create a coercion that wraps in a function type.
#[allow(dead_code)]
pub fn mk_fun_coercion(param_name: Name, param_ty: Expr, body: Expr) -> Expr {
    Expr::Lam(
        BinderInfo::Default,
        param_name,
        Box::new(param_ty),
        Box::new(body),
    )
}
/// Check if two expressions represent the same type head (ignoring arguments).
#[allow(dead_code)]
pub fn same_type_head(a: &Expr, b: &Expr) -> bool {
    match (a, b) {
        (Expr::Const(n1, _), Expr::Const(n2, _)) => n1 == n2,
        (Expr::Sort(l1), Expr::Sort(l2)) => l1 == l2,
        _ => false,
    }
}
/// Collect the head constant name from an expression (peeling off App nodes).
#[allow(dead_code)]
pub fn head_const_name(e: &Expr) -> Option<&Name> {
    match e {
        Expr::Const(n, _) => Some(n),
        Expr::App(f, _) => head_const_name(f),
        _ => None,
    }
}
/// Collect all argument expressions from a chain of applications.
#[allow(dead_code)]
pub fn collect_app_args(e: &Expr) -> (&Expr, Vec<&Expr>) {
    let mut args = Vec::new();
    let mut cur = e;
    while let Expr::App(f, a) = cur {
        args.push(a.as_ref());
        cur = f.as_ref();
    }
    args.reverse();
    (cur, args)
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::coercion::*;
    use oxilean_kernel::Literal;
    fn nat_expr() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    fn int_expr() -> Expr {
        Expr::Const(Name::str("Int"), vec![])
    }
    fn rat_expr() -> Expr {
        Expr::Const(Name::str("Rat"), vec![])
    }
    fn string_expr() -> Expr {
        Expr::Const(Name::str("String"), vec![])
    }
    fn bool_expr() -> Expr {
        Expr::Const(Name::str("Bool"), vec![])
    }
    fn prop_expr() -> Expr {
        Expr::Sort(Level::zero())
    }
    fn mk_test_coercion(from: Expr, to: Expr, name: &str) -> Coercion {
        Coercion {
            from,
            to,
            coerce: Name::str(name),
            kind: CoercionKind::FunCoercion,
            priority: 100,
            is_instance: false,
        }
    }
    #[test]
    fn test_registry_create() {
        let registry = CoercionRegistry::new();
        assert_eq!(registry.all_coercions().len(), 0);
    }
    #[test]
    fn test_register_coercion() {
        let mut registry = CoercionRegistry::new();
        registry.register(mk_test_coercion(nat_expr(), int_expr(), "Nat.toInt"));
        assert_eq!(registry.all_coercions().len(), 1);
        assert!(registry.find_coercion(&nat_expr(), &int_expr()).is_some());
    }
    #[test]
    fn test_find_coercion_none() {
        let registry = CoercionRegistry::new();
        assert!(registry
            .find_coercion(&nat_expr(), &string_expr())
            .is_none());
    }
    #[test]
    fn test_apply_coercion() {
        let mut registry = CoercionRegistry::new();
        registry.register(mk_test_coercion(nat_expr(), int_expr(), "Nat.toInt"));
        let expr = Expr::Lit(Literal::Nat(42));
        let result = registry.apply_coercion(expr.clone(), &nat_expr(), &int_expr());
        assert!(result.is_some());
        if let Some(Expr::App(f, a)) = result {
            assert!(matches!(*f, Expr::Const(_, _)));
            assert_eq!(*a, expr);
        } else {
            panic!("Expected App");
        }
    }
    #[test]
    fn test_coercion_kind_eq() {
        assert_eq!(CoercionKind::FunCoercion, CoercionKind::FunCoercion);
        assert_ne!(CoercionKind::FunCoercion, CoercionKind::SortCoercion);
    }
    #[test]
    fn test_coercion_kind_compound() {
        let ck = CoercionKind::CompoundCoercion(vec![Name::str("a"), Name::str("b")]);
        if let CoercionKind::CompoundCoercion(names) = &ck {
            assert_eq!(names.len(), 2);
        } else {
            panic!("Expected CompoundCoercion");
        }
    }
    #[test]
    fn test_coercion_priority() {
        let c = Coercion {
            from: nat_expr(),
            to: int_expr(),
            coerce: Name::str("Nat.toInt"),
            kind: CoercionKind::FunCoercion,
            priority: 50,
            is_instance: true,
        };
        assert_eq!(c.priority, 50);
        assert!(c.is_instance);
    }
    #[test]
    fn test_coercion_path_single() {
        let c = mk_test_coercion(nat_expr(), int_expr(), "Nat.toInt");
        let path = CoercionPath::single(c);
        assert_eq!(path.len(), 1);
        assert!(!path.is_empty());
        assert_eq!(path.total_cost, 100);
    }
    #[test]
    fn test_coercion_path_multi() {
        let c1 = mk_test_coercion(nat_expr(), int_expr(), "Nat.toInt");
        let c2 = mk_test_coercion(int_expr(), rat_expr(), "Rat.ofInt");
        let path = CoercionPath::from_steps(vec![c1, c2]);
        assert_eq!(path.len(), 2);
        assert_eq!(path.total_cost, 200);
    }
    #[test]
    fn test_find_coercion_chain_direct() {
        let mut registry = CoercionRegistry::new();
        registry.register(mk_test_coercion(nat_expr(), int_expr(), "Nat.toInt"));
        let chain = registry.find_coercion_chain(&nat_expr(), &int_expr());
        assert!(chain.is_some());
        assert_eq!(chain.expect("test operation should succeed").len(), 1);
    }
    #[test]
    fn test_find_coercion_chain_two_steps() {
        let mut registry = CoercionRegistry::new();
        registry.register(mk_test_coercion(nat_expr(), int_expr(), "Nat.toInt"));
        registry.register(mk_test_coercion(int_expr(), rat_expr(), "Rat.ofInt"));
        let chain = registry.find_coercion_chain(&nat_expr(), &rat_expr());
        assert!(chain.is_some());
        let path = chain.expect("test operation should succeed");
        assert_eq!(path.len(), 2);
        assert_eq!(path.steps[0].coerce, Name::str("Nat.toInt"));
        assert_eq!(path.steps[1].coerce, Name::str("Rat.ofInt"));
    }
    #[test]
    fn test_find_coercion_chain_none() {
        let mut registry = CoercionRegistry::new();
        registry.register(mk_test_coercion(nat_expr(), int_expr(), "Nat.toInt"));
        let chain = registry.find_coercion_chain(&nat_expr(), &string_expr());
        assert!(chain.is_none());
    }
    #[test]
    fn test_find_coercion_chain_no_cycle() {
        let mut registry = CoercionRegistry::new();
        let a = Expr::Const(Name::str("A"), vec![]);
        let b = Expr::Const(Name::str("B"), vec![]);
        let c = Expr::Const(Name::str("C"), vec![]);
        registry.register(mk_test_coercion(a.clone(), b.clone(), "a_to_b"));
        registry.register(mk_test_coercion(b.clone(), c.clone(), "b_to_c"));
        registry.register(mk_test_coercion(c.clone(), a.clone(), "c_to_a"));
        let chain = registry.find_coercion_chain(&a, &c);
        assert!(chain.is_some());
        assert_eq!(chain.expect("test operation should succeed").len(), 2);
    }
    #[test]
    fn test_apply_coercion_chain() {
        let mut registry = CoercionRegistry::new();
        registry.register(mk_test_coercion(nat_expr(), int_expr(), "Nat.toInt"));
        registry.register(mk_test_coercion(int_expr(), rat_expr(), "Rat.ofInt"));
        let chain = registry
            .find_coercion_chain(&nat_expr(), &rat_expr())
            .expect("test operation should succeed");
        let expr = Expr::Lit(Literal::Nat(5));
        let result = registry.apply_coercion_chain(expr, &chain);
        if let Expr::App(f, inner) = &result {
            assert!(matches!(f.as_ref(), Expr::Const(n, _) if * n == Name::str("Rat.ofInt")));
            if let Expr::App(f2, a2) = inner.as_ref() {
                assert!(matches!(f2.as_ref(), Expr::Const(n, _) if * n ==
                    Name::str("Nat.toInt")));
                assert_eq!(*a2.as_ref(), Expr::Lit(Literal::Nat(5)));
            } else {
                panic!("Expected inner App");
            }
        } else {
            panic!("Expected outer App");
        }
    }
    #[test]
    fn test_register_function_coercion() {
        let mut registry = CoercionRegistry::new();
        registry.register_function_coercion(
            Name::str("Nat"),
            Name::str("Int"),
            Name::str("Nat.toInt"),
            50,
        );
        assert_eq!(registry.all_coercions().len(), 1);
        let c = &registry.all_coercions()[0];
        assert_eq!(c.kind, CoercionKind::FunCoercion);
        assert_eq!(c.priority, 50);
    }
    #[test]
    fn test_register_sort_coercion() {
        let mut registry = CoercionRegistry::new();
        registry.register_sort_coercion(Level::zero(), Level::succ(Level::zero()));
        assert_eq!(registry.all_coercions().len(), 1);
        let c = &registry.all_coercions()[0];
        assert_eq!(c.kind, CoercionKind::SortCoercion);
    }
    #[test]
    fn test_unregister_coercion() {
        let mut registry = CoercionRegistry::new();
        registry.register(mk_test_coercion(nat_expr(), int_expr(), "Nat.toInt"));
        assert_eq!(registry.all_coercions().len(), 1);
        let removed = registry.unregister_coercion(&nat_expr(), &int_expr());
        assert!(removed);
        assert_eq!(registry.all_coercions().len(), 0);
    }
    #[test]
    fn test_unregister_coercion_not_found() {
        let mut registry = CoercionRegistry::new();
        let removed = registry.unregister_coercion(&nat_expr(), &int_expr());
        assert!(!removed);
    }
    #[test]
    fn test_has_coercion_direct() {
        let mut registry = CoercionRegistry::new();
        registry.register(mk_test_coercion(nat_expr(), int_expr(), "Nat.toInt"));
        assert!(registry.has_coercion(&nat_expr(), &int_expr()));
        assert!(!registry.has_coercion(&int_expr(), &nat_expr()));
    }
    #[test]
    fn test_has_coercion_transitive() {
        let mut registry = CoercionRegistry::new();
        registry.register(mk_test_coercion(nat_expr(), int_expr(), "Nat.toInt"));
        registry.register(mk_test_coercion(int_expr(), rat_expr(), "Rat.ofInt"));
        assert!(registry.has_coercion(&nat_expr(), &rat_expr()));
    }
    #[test]
    fn test_coercion_graph() {
        let mut registry = CoercionRegistry::new();
        registry.register(mk_test_coercion(nat_expr(), int_expr(), "Nat.toInt"));
        registry.register(mk_test_coercion(int_expr(), rat_expr(), "Rat.ofInt"));
        let graph = registry.coercion_graph();
        assert_eq!(graph.len(), 2);
    }
    #[test]
    fn test_register_builtins() {
        let mut registry = CoercionRegistry::new();
        registry.register_builtins();
        assert!(registry.all_coercions().len() >= 3);
        assert!(registry.find_coercion(&nat_expr(), &int_expr()).is_some());
        assert!(registry.find_coercion(&bool_expr(), &prop_expr()).is_some());
    }
    #[test]
    fn test_builtins_nat_to_rat_chain() {
        let mut registry = CoercionRegistry::new();
        registry.register_builtins();
        let chain = registry.find_coercion_chain(&nat_expr(), &rat_expr());
        assert!(chain.is_some());
    }
    #[test]
    fn test_try_insert_coercion_same_type() {
        let registry = CoercionRegistry::new();
        let expr = Expr::Lit(Literal::Nat(1));
        let result = try_insert_coercion(&registry, expr.clone(), &nat_expr(), &nat_expr());
        assert_eq!(result, Some(expr));
    }
    #[test]
    fn test_try_insert_coercion_direct() {
        let mut registry = CoercionRegistry::new();
        registry.register(mk_test_coercion(nat_expr(), int_expr(), "Nat.toInt"));
        let expr = Expr::Lit(Literal::Nat(7));
        let result = try_insert_coercion(&registry, expr, &nat_expr(), &int_expr());
        assert!(result.is_some());
    }
    #[test]
    fn test_try_insert_coercion_chain() {
        let mut registry = CoercionRegistry::new();
        registry.register(mk_test_coercion(nat_expr(), int_expr(), "Nat.toInt"));
        registry.register(mk_test_coercion(int_expr(), rat_expr(), "Rat.ofInt"));
        let expr = Expr::Lit(Literal::Nat(3));
        let result = try_insert_coercion(&registry, expr, &nat_expr(), &rat_expr());
        assert!(result.is_some());
    }
    #[test]
    fn test_try_insert_coercion_none() {
        let registry = CoercionRegistry::new();
        let expr = Expr::Lit(Literal::Nat(1));
        let result = try_insert_coercion(&registry, expr, &nat_expr(), &string_expr());
        assert!(result.is_none());
    }
    #[test]
    fn test_coerce_to_sort() {
        let mut registry = CoercionRegistry::new();
        registry.register(mk_test_coercion(bool_expr(), prop_expr(), "Bool.toProp"));
        let expr = Expr::Const(Name::str("true"), vec![]);
        let result = coerce_to_sort(&registry, expr, &bool_expr(), &prop_expr());
        assert!(result.is_some());
    }
    #[test]
    fn test_coerce_to_sort_not_sort() {
        let registry = CoercionRegistry::new();
        let expr = Expr::Lit(Literal::Nat(1));
        let result = coerce_to_sort(&registry, expr, &nat_expr(), &int_expr());
        assert!(result.is_none());
    }
    #[test]
    fn test_coerce_to_function() {
        let mut registry = CoercionRegistry::new();
        let pi_ty = Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(nat_expr()),
            Box::new(nat_expr()),
        );
        registry.register(Coercion {
            from: Expr::Const(Name::str("MyFunctor"), vec![]),
            to: pi_ty.clone(),
            coerce: Name::str("MyFunctor.toFun"),
            kind: CoercionKind::FunCoercion,
            priority: 100,
            is_instance: false,
        });
        let expr = Expr::Const(Name::str("myFunctor"), vec![]);
        let src_ty = Expr::Const(Name::str("MyFunctor"), vec![]);
        let result = coerce_to_function(&registry, expr, &src_ty, &pi_ty);
        assert!(result.is_some());
    }
    #[test]
    fn test_coerce_to_function_not_pi() {
        let registry = CoercionRegistry::new();
        let expr = Expr::Lit(Literal::Nat(1));
        let result = coerce_to_function(&registry, expr, &nat_expr(), &int_expr());
        assert!(result.is_none());
    }
    #[test]
    fn test_same_type_head() {
        assert!(same_type_head(&nat_expr(), &nat_expr()));
        assert!(!same_type_head(&nat_expr(), &int_expr()));
        assert!(same_type_head(&prop_expr(), &prop_expr()));
    }
    #[test]
    fn test_head_const_name() {
        let e = Expr::App(
            Box::new(Expr::Const(Name::str("List"), vec![])),
            Box::new(nat_expr()),
        );
        assert_eq!(head_const_name(&e), Some(&Name::str("List")));
        assert_eq!(head_const_name(&nat_expr()), Some(&Name::str("Nat")));
        assert_eq!(head_const_name(&Expr::Lit(Literal::Nat(0))), None);
    }
    #[test]
    fn test_collect_app_args() {
        let e = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("f"), vec![])),
                Box::new(Expr::Lit(Literal::Nat(1))),
            )),
            Box::new(Expr::Lit(Literal::Nat(2))),
        );
        let (head, args) = collect_app_args(&e);
        assert!(matches!(head, Expr::Const(n, _) if * n == Name::str("f")));
        assert_eq!(args.len(), 2);
    }
    #[test]
    fn test_mk_coercion_app() {
        let result = mk_coercion_app(Name::str("f"), Expr::Lit(Literal::Nat(1)));
        assert!(matches!(result, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_fun_coercion() {
        let result = mk_fun_coercion(Name::str("x"), nat_expr(), Expr::BVar(0));
        assert!(matches!(result, Expr::Lam(_, _, _, _)));
    }
    #[test]
    fn test_lift_sort_same() {
        let expr = Expr::Const(Name::str("x"), vec![]);
        let result = lift_sort(expr.clone(), &Level::zero(), &Level::zero());
        assert_eq!(result, Some(expr));
    }
    #[test]
    fn test_lift_sort_different() {
        let expr = Expr::Const(Name::str("x"), vec![]);
        let result = lift_sort(expr, &Level::zero(), &Level::succ(Level::zero()));
        assert!(result.is_some());
    }
    #[test]
    fn test_default_registry() {
        let registry = CoercionRegistry::default();
        assert_eq!(registry.all_coercions().len(), 0);
    }
}
/// Check if two expressions represent the same coercion source/target type (by head constant).
pub fn same_coercion_head(a: &Expr, b: &Expr) -> bool {
    fn head(e: &Expr) -> Option<&Name> {
        match e {
            Expr::Const(n, _) => Some(n),
            Expr::App(f, _) => head(f),
            _ => None,
        }
    }
    match (head(a), head(b)) {
        (Some(na), Some(nb)) => na == nb,
        _ => false,
    }
}
/// Check if a coercion is trivial (coercing to the same type).
pub fn is_trivial_coercion(coercion: &Coercion) -> bool {
    coercion.from == coercion.to
}
#[cfg(test)]
mod coercion_extra_tests {
    use super::*;
    use crate::coercion::*;
    fn nat_expr() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    #[test]
    fn test_coercion_stats_record() {
        let mut s = CoercionStats::new();
        s.record_inserted();
        s.record_failed();
        s.record_chained();
        s.record_sort();
        assert_eq!(s.inserted, 3);
        assert_eq!(s.failed, 1);
        assert_eq!(s.chained, 1);
        assert_eq!(s.sort_coercions, 1);
    }
    #[test]
    fn test_coercion_stats_total_attempts() {
        let mut s = CoercionStats::new();
        s.record_inserted();
        s.record_inserted();
        s.record_failed();
        assert_eq!(s.total_attempts(), 3);
    }
    #[test]
    fn test_coercion_stats_success_rate() {
        let mut s = CoercionStats::new();
        s.record_inserted();
        s.record_inserted();
        s.record_failed();
        let rate = s.success_rate();
        assert!((rate - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_coercion_stats_empty_rate() {
        let s = CoercionStats::new();
        assert_eq!(s.success_rate(), 1.0);
    }
    #[test]
    fn test_coercion_stats_display() {
        let s = CoercionStats {
            inserted: 5,
            failed: 2,
            chained: 1,
            sort_coercions: 0,
        };
        let txt = format!("{}", s);
        assert!(txt.contains("inserted: 5"));
        assert!(txt.contains("failed: 2"));
    }
    #[test]
    fn test_same_coercion_head_same() {
        let a = nat_expr();
        let b = nat_expr();
        assert!(same_coercion_head(&a, &b));
    }
    #[test]
    fn test_same_coercion_head_different() {
        let a = nat_expr();
        let b = Expr::Const(Name::str("Bool"), vec![]);
        assert!(!same_coercion_head(&a, &b));
    }
    #[test]
    fn test_same_coercion_head_app() {
        let fa = Expr::App(
            Box::new(Expr::Const(Name::str("List"), vec![])),
            Box::new(nat_expr()),
        );
        let fb = Expr::App(
            Box::new(Expr::Const(Name::str("List"), vec![])),
            Box::new(Expr::Const(Name::str("Bool"), vec![])),
        );
        assert!(same_coercion_head(&fa, &fb));
    }
    #[test]
    fn test_is_trivial_coercion() {
        let c = Coercion {
            from: nat_expr(),
            to: nat_expr(),
            coerce: Name::str("id"),
            kind: CoercionKind::UserCoercion,
            priority: 0,
            is_instance: false,
        };
        assert!(is_trivial_coercion(&c));
    }
    #[test]
    fn test_is_not_trivial_coercion() {
        let c = Coercion {
            from: nat_expr(),
            to: Expr::Const(Name::str("Int"), vec![]),
            coerce: Name::str("Int.ofNat"),
            kind: CoercionKind::UserCoercion,
            priority: 0,
            is_instance: false,
        };
        assert!(!is_trivial_coercion(&c));
    }
}
/// Compute a key string for a type expression (for graph lookup).
pub fn coercion_type_key(ty: &Expr) -> String {
    match ty {
        Expr::Const(name, _) => format!("{:?}", name),
        Expr::App(f, _) => format!("App({:?})", f),
        _ => format!("{:?}", ty),
    }
}
/// Composes a coercion path into a single application expression.
#[allow(dead_code)]
pub fn compose_coercion_path(path: &CoercionPath, arg: Expr) -> Expr {
    let mut result = arg;
    for step in &path.steps {
        result = Expr::App(
            Box::new(Expr::Const(step.coerce.clone(), vec![])),
            Box::new(result),
        );
    }
    result
}
#[cfg(test)]
mod extended_coercion_tests {
    use super::*;
    use crate::coercion::*;
    fn nat_expr() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    fn int_expr() -> Expr {
        Expr::Const(Name::str("Int"), vec![])
    }
    fn bool_expr() -> Expr {
        Expr::Const(Name::str("Bool"), vec![])
    }
    fn make_coercion(from: Expr, to: Expr, name: &str, priority: u32) -> Coercion {
        Coercion {
            from,
            to,
            coerce: Name::str(name),
            kind: CoercionKind::UserCoercion,
            priority,
            is_instance: false,
        }
    }
    #[test]
    fn test_coercion_graph_add_find() {
        let mut graph = CoercionGraph::new();
        let c = make_coercion(nat_expr(), int_expr(), "Int.ofNat", 0);
        graph.add_coercion(c);
        assert_eq!(graph.edge_count(), 1);
        assert_eq!(graph.node_count(), 1);
        let edges = graph.coercions_from(&nat_expr());
        assert_eq!(edges.len(), 1);
        assert_eq!(edges[0].coerce, Name::str("Int.ofNat"));
    }
    #[test]
    fn test_coercion_registry_direct() {
        let mut reg = CoercionGraphRegistry::new();
        reg.register(make_coercion(nat_expr(), int_expr(), "Int.ofNat", 5));
        reg.register(make_coercion(int_expr(), bool_expr(), "Bool.ofInt", 3));
        assert_eq!(reg.len(), 2);
        let direct = reg.find_direct(&nat_expr(), &int_expr());
        assert!(direct.is_some());
        assert_eq!(
            direct.expect("coercion should succeed").coerce,
            Name::str("Int.ofNat")
        );
        let none = reg.find_direct(&nat_expr(), &bool_expr());
        assert!(none.is_none());
    }
    #[test]
    fn test_coercion_registry_path() {
        let mut reg = CoercionGraphRegistry::new();
        reg.register(make_coercion(nat_expr(), int_expr(), "Int.ofNat", 5));
        reg.register(make_coercion(int_expr(), bool_expr(), "Bool.ofInt", 3));
        let direct_path = reg.find_path(&nat_expr(), &int_expr(), 5);
        assert!(direct_path.is_some());
        assert_eq!(direct_path.expect("test operation should succeed").len(), 1);
        let multi_path = reg.find_path(&nat_expr(), &bool_expr(), 5);
        assert!(multi_path.is_some());
        assert_eq!(multi_path.expect("test operation should succeed").len(), 2);
    }
    #[test]
    fn test_coercion_registry_user_and_instance() {
        let mut reg = CoercionGraphRegistry::new();
        let mut instance_coerce = make_coercion(nat_expr(), int_expr(), "inst_coerce", 0);
        instance_coerce.is_instance = true;
        reg.register(instance_coerce);
        reg.register(make_coercion(int_expr(), bool_expr(), "user_coerce", 0));
        assert_eq!(reg.user_coercions().len(), 1);
        assert_eq!(reg.instance_coercions().len(), 1);
    }
    #[test]
    fn test_coercion_cache_basic() {
        let mut cache = CoercionCache::new();
        assert!(cache.is_empty());
        cache.store(&nat_expr(), &int_expr(), None);
        assert_eq!(cache.len(), 1);
        let result = cache.get(&nat_expr(), &int_expr());
        assert!(result.is_some());
        assert!(result.expect("test operation should succeed").is_none());
        assert_eq!(cache.hits(), 1);
        assert_eq!(cache.misses(), 0);
    }
    #[test]
    fn test_coercion_cache_hit_rate() {
        let mut cache = CoercionCache::new();
        cache.store(&nat_expr(), &int_expr(), None);
        cache.get(&nat_expr(), &int_expr());
        cache.get(&nat_expr(), &int_expr());
        cache.get(&int_expr(), &bool_expr());
        assert!((cache.hit_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_coercion_cache_clear() {
        let mut cache = CoercionCache::new();
        cache.store(&nat_expr(), &int_expr(), None);
        cache.clear();
        assert!(cache.is_empty());
        assert_eq!(cache.hits(), 0);
        assert_eq!(cache.misses(), 0);
    }
    #[test]
    fn test_coercion_stats() {
        let mut stats = CoercionAppStats::new();
        stats.record_application(1, false);
        stats.record_application(3, true);
        stats.record_application(1, true);
        stats.record_failure();
        assert_eq!(stats.total_applied, 3);
        assert_eq!(stats.multi_step, 1);
        assert_eq!(stats.cache_hits, 2);
        assert_eq!(stats.failures, 1);
        assert!((stats.success_rate() - 0.75).abs() < 1e-9);
    }
    #[test]
    fn test_compose_coercion_path() {
        let step1 = make_coercion(nat_expr(), int_expr(), "Int.ofNat", 0);
        let path = CoercionPath::single(step1);
        let composed = compose_coercion_path(&path, nat_expr());
        assert!(matches!(composed, Expr::App(_, _)));
    }
    #[test]
    fn test_coercion_graph_no_path() {
        let mut graph = CoercionGraph::new();
        graph.add_coercion(make_coercion(nat_expr(), int_expr(), "c1", 0));
        let path = graph.find_path(&bool_expr(), &nat_expr(), 5);
        assert!(path.is_none());
    }
    #[test]
    fn test_coercion_registry_empty() {
        let reg = CoercionGraphRegistry::new();
        assert!(reg.is_empty());
        assert!(reg.find_direct(&nat_expr(), &int_expr()).is_none());
        assert!(reg.find_path(&nat_expr(), &int_expr(), 5).is_none());
    }
    #[test]
    fn test_coercion_graph_bfs_multi_step() {
        let mut graph = CoercionGraph::new();
        let char_expr = Expr::Const(Name::str("Char"), vec![]);
        graph.add_coercion(make_coercion(nat_expr(), int_expr(), "toInt", 0));
        graph.add_coercion(make_coercion(int_expr(), bool_expr(), "toBool", 0));
        graph.add_coercion(make_coercion(bool_expr(), char_expr.clone(), "toChar", 0));
        let path = graph.find_path(&nat_expr(), &char_expr, 5);
        assert!(path.is_some());
        let p = path.expect("test operation should succeed");
        assert_eq!(p.len(), 3);
    }
}
#[cfg(test)]
mod function_sort_scope_tests {
    use super::*;
    use crate::coercion::*;
    fn nat_expr() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    fn int_expr() -> Expr {
        Expr::Const(Name::str("Int"), vec![])
    }
    #[test]
    fn test_function_coercion() {
        let fc = FunctionCoercion::new(Name::str("fn_coerce")).with_codomain(Coercion {
            from: nat_expr(),
            to: int_expr(),
            coerce: Name::str("Int.ofNat"),
            kind: CoercionKind::FunCoercion,
            priority: 0,
            is_instance: false,
        });
        assert!(!fc.has_domain_coerce());
        assert!(fc.has_codomain_coerce());
        assert_eq!(fc.name, Name::str("fn_coerce"));
    }
    #[test]
    fn test_sort_coercion_decl() {
        let prop_to_type = SortCoercionDecl::prop_to_type(Name::str("Prop.toType"));
        assert!(prop_to_type.is_prop_to_type());
        assert_eq!(prop_to_type.direction, SortCoercionDirection::PropToType);
        let type_to_prop = SortCoercionDecl::type_to_prop(Name::str("Type.toProp"));
        assert!(!type_to_prop.is_prop_to_type());
        assert_eq!(type_to_prop.direction, SortCoercionDirection::TypeToProp);
    }
    #[test]
    fn test_coercion_scope_basic() {
        let mut scope = CoercionScope::new();
        assert!(scope.is_empty());
        scope.add(Coercion {
            from: nat_expr(),
            to: int_expr(),
            coerce: Name::str("c1"),
            kind: CoercionKind::UserCoercion,
            priority: 0,
            is_instance: false,
        });
        assert_eq!(scope.len(), 1);
        assert!(!scope.is_empty());
    }
    #[test]
    fn test_coercion_scope_stack() {
        let mut stack = CoercionScopeStack::new();
        assert!(stack.is_empty());
        stack.push_scope();
        stack.add_to_top(Coercion {
            from: nat_expr(),
            to: int_expr(),
            coerce: Name::str("outer_c"),
            kind: CoercionKind::UserCoercion,
            priority: 0,
            is_instance: false,
        });
        stack.push_scope();
        stack.add_to_top(Coercion {
            from: int_expr(),
            to: nat_expr(),
            coerce: Name::str("inner_c"),
            kind: CoercionKind::UserCoercion,
            priority: 0,
            is_instance: false,
        });
        assert_eq!(stack.depth(), 2);
        let visible = stack.visible_coercions();
        assert_eq!(visible.len(), 2);
        assert_eq!(visible[0].coerce, Name::str("inner_c"));
        let popped = stack.pop_scope().expect("test operation should succeed");
        assert_eq!(popped.len(), 1);
        assert_eq!(stack.depth(), 1);
    }
    #[test]
    fn test_scope_stack_add_when_empty() {
        let mut stack = CoercionScopeStack::new();
        stack.add_to_top(Coercion {
            from: nat_expr(),
            to: int_expr(),
            coerce: Name::str("c"),
            kind: CoercionKind::UserCoercion,
            priority: 0,
            is_instance: false,
        });
        assert!(stack.is_empty());
    }
    #[test]
    fn test_sort_coercion_direction_eq() {
        assert_eq!(
            SortCoercionDirection::PropToType,
            SortCoercionDirection::PropToType
        );
        assert_ne!(
            SortCoercionDirection::PropToType,
            SortCoercionDirection::TypeToProp
        );
    }
    #[test]
    fn test_function_coercion_both() {
        let c1 = Coercion {
            from: nat_expr(),
            to: int_expr(),
            coerce: Name::str("toInt"),
            kind: CoercionKind::FunCoercion,
            priority: 0,
            is_instance: false,
        };
        let c2 = Coercion {
            from: int_expr(),
            to: nat_expr(),
            coerce: Name::str("toNat"),
            kind: CoercionKind::FunCoercion,
            priority: 0,
            is_instance: false,
        };
        let fc = FunctionCoercion::new(Name::str("combined"))
            .with_domain(c1)
            .with_codomain(c2);
        assert!(fc.has_domain_coerce());
        assert!(fc.has_codomain_coerce());
    }
}
#[cfg(test)]
mod extended_tests {
    use super::*;
    use crate::coercion::*;
    fn nat_expr() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    fn int_expr() -> Expr {
        Expr::Const(Name::str("Int"), vec![])
    }
    fn make_coercion(name: &str, priority: u32) -> Coercion {
        Coercion {
            from: nat_expr(),
            to: int_expr(),
            coerce: Name::str(name),
            kind: CoercionKind::FunCoercion,
            priority,
            is_instance: false,
        }
    }
    #[test]
    fn test_validator_valid_coercion() {
        let mut v = CoercionValidator::new();
        let c = make_coercion("toInt", 0);
        v.validate_coercion(&c);
        assert!(!v.has_errors());
    }
    #[test]
    fn test_validator_invalid_priority() {
        let mut v = CoercionValidator::new();
        let c = make_coercion("bad", u32::MAX);
        v.validate_coercion(&c);
        assert!(v.has_errors());
        assert!(matches!(
            v.errors()[0],
            CoercionValidationError::InvalidPriority { .. }
        ));
    }
    #[test]
    fn test_event_log_cache_hit_rate() {
        let mut log = CoercionEventLog::new(100);
        log.log(CoercionEventKind::CacheHit);
        log.log(CoercionEventKind::CacheHit);
        log.log(CoercionEventKind::CacheMiss);
        let rate = log.cache_hit_rate();
        assert!((rate - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_event_log_applied() {
        let mut log = CoercionEventLog::new(100);
        log.log(CoercionEventKind::Applied {
            coerce: Name::str("toInt"),
        });
        log.log(CoercionEventKind::PathFound { length: 2 });
        let applied = log.applied_coercions();
        assert_eq!(applied.len(), 1);
    }
    #[test]
    fn test_event_log_capacity() {
        let mut log = CoercionEventLog::new(3);
        for i in 0..5u64 {
            log.log(CoercionEventKind::Registered {
                coerce: Name::str(format!("c{}", i)),
            });
        }
        assert_eq!(log.events().len(), 3);
    }
    #[test]
    fn test_pretty_printer() {
        let c = make_coercion("toInt", 5);
        let pp = CoercionPrettyPrinter::new();
        let s = pp.print_coercion(&c);
        assert!(s.contains("toInt"));
        assert!(s.contains("priority=5"));
    }
    #[test]
    fn test_typeclass_coercion_registry() {
        let c = make_coercion("toInt", 0);
        let tc = CoercionTypeClass::new(Name::str("Coe"), Name::str("coe"), c);
        let mut reg = TypeClassCoercionRegistry::new();
        reg.register(tc);
        assert_eq!(reg.len(), 1);
        let found = reg.lookup_by_class(&Name::str("Coe"));
        assert_eq!(found.len(), 1);
        let by_method = reg.lookup_by_method(&Name::str("coe"));
        assert!(by_method.is_some());
    }
    #[test]
    fn test_normalizer_none_strategy() {
        let norm = CoercionNormalizer::new(NormalizationStrategy::None);
        assert_eq!(norm.strategy(), NormalizationStrategy::None);
        let e = nat_expr();
        let result = norm.normalize(e.clone());
        assert_eq!(format!("{:?}", result), format!("{:?}", e));
    }
    #[test]
    fn test_inference_hint_forbidden() {
        let hint = CoercionInferenceHint::new()
            .forbid(Name::str("bad_coerce"))
            .max_length(3);
        assert!(hint.is_forbidden(&Name::str("bad_coerce")));
        assert!(!hint.is_forbidden(&Name::str("good_coerce")));
        assert_eq!(hint.max_path_length, Some(3));
    }
    #[test]
    fn test_validator_registry_warnings() {
        let mut v = CoercionValidator::new();
        let mut reg = CoercionRegistry::new();
        reg.register(make_coercion("toInt1", 0));
        reg.register(make_coercion("toInt2", 1));
        v.validate_registry(&reg);
        assert!(!v.warnings().is_empty());
    }
}
