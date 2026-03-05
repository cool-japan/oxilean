//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    InferError, InferResult, InferTypeAnalysisPass, InferTypeBuilder, InferTypeConfig,
    InferTypeConfigValue, InferTypeCounterMap, InferTypeDiagnostics, InferTypeDiff,
    InferTypeExtConfig2300, InferTypeExtConfigVal2300, InferTypeExtDiag2300, InferTypeExtDiff2300,
    InferTypeExtMap, InferTypeExtPass2300, InferTypeExtPipeline2300, InferTypeExtResult2300,
    InferTypeExtUtil, InferTypePipeline, InferTypeResult, InferTypeStateMachine, InferTypeWindow,
    InferTypeWorkQueue, MetaInferType, TypeAnnotation, TypeInferCache, TypeWithProvenance,
    TypedExpr, TypingStack,
};
use crate::basic::{MVarId, MetaContext, MetavarKind};
use oxilean_kernel::{BinderInfo, ConstantInfo, Expr, Level, Name};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infer_type::*;
    use oxilean_kernel::{Environment, FVarId, Literal};
    fn mk_ctx() -> MetaContext {
        MetaContext::new(Environment::new())
    }
    #[test]
    fn test_infer_sort() {
        let mut infer = MetaInferType::new();
        let mut ctx = mk_ctx();
        let expr = Expr::Sort(Level::zero());
        let ty = infer
            .infer_type(&expr, &mut ctx)
            .expect("ty should be present");
        assert_eq!(ty, Expr::Sort(Level::succ(Level::zero())));
    }
    #[test]
    fn test_infer_lit_nat() {
        let mut infer = MetaInferType::new();
        let mut ctx = mk_ctx();
        let expr = Expr::Lit(Literal::Nat(42));
        let ty = infer
            .infer_type(&expr, &mut ctx)
            .expect("ty should be present");
        assert_eq!(ty, Expr::Const(Name::str("Nat"), vec![]));
    }
    #[test]
    fn test_infer_lit_str() {
        let mut infer = MetaInferType::new();
        let mut ctx = mk_ctx();
        let expr = Expr::Lit(Literal::Str("hello".to_string()));
        let ty = infer
            .infer_type(&expr, &mut ctx)
            .expect("ty should be present");
        assert_eq!(ty, Expr::Const(Name::str("String"), vec![]));
    }
    #[test]
    fn test_infer_fvar() {
        let mut infer = MetaInferType::new();
        let mut ctx = mk_ctx();
        let fvar = ctx.mk_local_decl(
            Name::str("x"),
            Expr::Const(Name::str("Nat"), vec![]),
            BinderInfo::Default,
        );
        let ty = infer
            .infer_type(&Expr::FVar(fvar), &mut ctx)
            .expect("ty should be present");
        assert_eq!(ty, Expr::Const(Name::str("Nat"), vec![]));
    }
    #[test]
    fn test_infer_unknown_fvar() {
        let mut infer = MetaInferType::new();
        let mut ctx = mk_ctx();
        let result = infer.infer_type(&Expr::FVar(FVarId::new(999)), &mut ctx);
        assert!(result.is_err());
    }
    #[test]
    fn test_infer_mvar() {
        let mut infer = MetaInferType::new();
        let mut ctx = mk_ctx();
        let mvar_ty = Expr::Sort(Level::zero());
        let (_, placeholder) = ctx.mk_fresh_expr_mvar(mvar_ty.clone(), MetavarKind::Natural);
        let ty = infer
            .infer_type(&placeholder, &mut ctx)
            .expect("ty should be present");
        assert_eq!(ty, mvar_ty);
    }
    #[test]
    fn test_infer_app() {
        let mut infer = MetaInferType::new();
        let mut ctx = mk_ctx();
        let fn_ty = Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
        );
        let fvar = ctx.mk_local_decl(Name::str("f"), fn_ty, BinderInfo::Default);
        let app = Expr::App(
            Box::new(Expr::FVar(fvar)),
            Box::new(Expr::Lit(Literal::Nat(42))),
        );
        let ty = infer
            .infer_type(&app, &mut ctx)
            .expect("ty should be present");
        assert_eq!(ty, Expr::Const(Name::str("Nat"), vec![]));
    }
    #[test]
    fn test_infer_lambda() {
        let mut infer = MetaInferType::new();
        let mut ctx = mk_ctx();
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
            Box::new(Expr::BVar(0)),
        );
        let ty = infer
            .infer_type(&lam, &mut ctx)
            .expect("ty should be present");
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_infer_let() {
        let mut infer = MetaInferType::new();
        let mut ctx = mk_ctx();
        let expr = Expr::Let(
            Name::str("x"),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
            Box::new(Expr::Lit(Literal::Nat(42))),
            Box::new(Expr::BVar(0)),
        );
        let ty = infer
            .infer_type(&expr, &mut ctx)
            .expect("ty should be present");
        assert_eq!(ty, Expr::Const(Name::str("Nat"), vec![]));
    }
    #[test]
    fn test_get_level() {
        let mut infer = MetaInferType::new();
        let mut ctx = mk_ctx();
        let sort = Expr::Sort(Level::zero());
        let level = infer
            .get_level(&sort, &mut ctx)
            .expect("level should be present");
        assert_eq!(level, Level::zero());
        let sort2 = Expr::Sort(Level::succ(Level::zero()));
        let level2 = infer
            .get_level(&sort2, &mut ctx)
            .expect("level2 should be present");
        assert_eq!(level2, Level::succ(Level::zero()));
    }
    #[test]
    fn test_ensure_pi() {
        let mut infer = MetaInferType::new();
        let mut ctx = mk_ctx();
        let pi = Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
        );
        let (domain, codomain) = infer
            .ensure_pi(&pi, &mut ctx)
            .expect("value should be present");
        assert_eq!(domain, Expr::Const(Name::str("Nat"), vec![]));
        assert_eq!(codomain, Expr::Const(Name::str("Nat"), vec![]));
    }
    #[test]
    fn test_ensure_pi_not_pi() {
        let mut infer = MetaInferType::new();
        let mut ctx = mk_ctx();
        let not_pi = Expr::Lit(Literal::Nat(42));
        let result = infer.ensure_pi(&not_pi, &mut ctx);
        assert!(result.is_err());
    }
    #[test]
    fn test_ensure_sort() {
        let mut infer = MetaInferType::new();
        let mut ctx = mk_ctx();
        let sort = Expr::Sort(Level::zero());
        let level = infer
            .ensure_sort(&sort, &mut ctx)
            .expect("level should be present");
        assert_eq!(level, Level::zero());
    }
}
/// Compute the universe level of a Sort expression.
#[allow(dead_code)]
pub fn sort_level(expr: &Expr) -> Option<&Level> {
    match expr {
        Expr::Sort(l) => Some(l),
        _ => None,
    }
}
/// Check if an expression is Prop (Sort 0).
#[allow(dead_code)]
pub fn is_prop_expr(expr: &Expr) -> bool {
    match expr {
        Expr::Sort(l) => l.is_zero(),
        _ => false,
    }
}
/// Check if an expression is Type 1 (Sort 1).
#[allow(dead_code)]
pub fn is_type1_expr(expr: &Expr) -> bool {
    match expr {
        Expr::Sort(l) => l == &Level::succ(Level::zero()),
        _ => false,
    }
}
/// Maximum depth for type inference.
#[allow(dead_code)]
pub const MAX_INFER_DEPTH: u32 = 512;
/// Infer the type of a literal without any context.
#[allow(dead_code)]
pub fn infer_literal_type(lit: &oxilean_kernel::Literal) -> Expr {
    match lit {
        oxilean_kernel::Literal::Nat(_) => Expr::Const(Name::str("Nat"), vec![]),
        oxilean_kernel::Literal::Str(_) => Expr::Const(Name::str("String"), vec![]),
    }
}
/// Check if two types are compatible for unification purposes.
#[allow(dead_code)]
pub fn types_compatible(ty1: &Expr, ty2: &Expr) -> bool {
    match (ty1, ty2) {
        (Expr::Sort(_), Expr::Sort(_)) => true,
        (Expr::Const(n1, _), Expr::Const(n2, _)) => n1 == n2,
        (Expr::Lit(l1), Expr::Lit(l2)) => {
            matches!(
                (l1, l2),
                (
                    oxilean_kernel::Literal::Nat(_),
                    oxilean_kernel::Literal::Nat(_)
                ) | (
                    oxilean_kernel::Literal::Str(_),
                    oxilean_kernel::Literal::Str(_)
                )
            )
        }
        _ => false,
    }
}
#[cfg(test)]
mod extended_infer_tests {
    use super::*;
    use crate::infer_type::*;
    use oxilean_kernel::Literal;
    #[test]
    fn test_type_infer_cache_empty() {
        let cache = TypeInferCache::new();
        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);
    }
    #[test]
    fn test_type_infer_cache_insert_get() {
        let mut cache = TypeInferCache::new();
        let expr = Expr::Lit(Literal::Nat(42));
        let ty = Expr::Const(Name::str("Nat"), vec![]);
        cache.insert(expr.clone(), ty.clone());
        let result = cache.get(&expr);
        assert!(result.is_some());
        assert_eq!(result.expect("result should be valid"), &ty);
    }
    #[test]
    fn test_type_infer_cache_miss() {
        let mut cache = TypeInferCache::new();
        let expr = Expr::Lit(Literal::Nat(1));
        let result = cache.get(&expr);
        assert!(result.is_none());
        assert_eq!(cache.total_accesses(), 1);
    }
    #[test]
    fn test_type_infer_cache_hit_rate() {
        let mut cache = TypeInferCache::new();
        let expr = Expr::Lit(Literal::Nat(7));
        let ty = Expr::Const(Name::str("Nat"), vec![]);
        cache.insert(expr.clone(), ty);
        let _ = cache.get(&expr);
        let _ = cache.get(&Expr::BVar(0));
        assert!((cache.hit_rate() - 0.5).abs() < 1e-10);
    }
    #[test]
    fn test_type_infer_cache_clear() {
        let mut cache = TypeInferCache::new();
        cache.insert(Expr::BVar(0), Expr::Sort(Level::zero()));
        cache.clear();
        assert!(cache.is_empty());
    }
    #[test]
    fn test_sort_level() {
        let prop = Expr::Sort(Level::zero());
        assert_eq!(sort_level(&prop), Some(&Level::zero()));
        assert_eq!(sort_level(&Expr::BVar(0)), None);
    }
    #[test]
    fn test_is_prop_expr() {
        assert!(is_prop_expr(&Expr::Sort(Level::zero())));
        assert!(!is_prop_expr(&Expr::Sort(Level::succ(Level::zero()))));
        assert!(!is_prop_expr(&Expr::BVar(0)));
    }
    #[test]
    fn test_is_type1_expr() {
        let type1 = Expr::Sort(Level::succ(Level::zero()));
        assert!(is_type1_expr(&type1));
        assert!(!is_type1_expr(&Expr::Sort(Level::zero())));
    }
    #[test]
    fn test_typed_expr_is_proof() {
        let te = TypedExpr::new(
            Expr::Const(Name::str("proof"), vec![]),
            Expr::Sort(Level::zero()),
        );
        assert!(te.is_proof());
        assert!(!te.is_type());
    }
    #[test]
    fn test_typed_expr_is_type() {
        let te = TypedExpr::new(
            Expr::Const(Name::str("Nat"), vec![]),
            Expr::Sort(Level::succ(Level::zero())),
        );
        assert!(te.is_type());
        assert!(!te.is_proof());
    }
    #[test]
    fn test_infer_result_ok() {
        let ty = Expr::Const(Name::str("Nat"), vec![]);
        let r = InferResult::ok(ty.clone());
        assert!(!r.has_warnings());
        assert_eq!(r.steps, 0);
        assert_eq!(r.ty, ty);
    }
    #[test]
    fn test_infer_result_with_warnings() {
        let ty = Expr::Sort(Level::zero());
        let r = InferResult::with_warnings(ty, vec!["warning!".to_string()], 5);
        assert!(r.has_warnings());
        assert_eq!(r.steps, 5);
    }
    #[test]
    fn test_infer_literal_type_nat() {
        let ty = infer_literal_type(&oxilean_kernel::Literal::Nat(0));
        assert_eq!(ty, Expr::Const(Name::str("Nat"), vec![]));
    }
    #[test]
    fn test_infer_literal_type_str() {
        let ty = infer_literal_type(&oxilean_kernel::Literal::Str("hi".to_string()));
        assert_eq!(ty, Expr::Const(Name::str("String"), vec![]));
    }
    #[test]
    fn test_types_compatible_sorts() {
        let p = Expr::Sort(Level::zero());
        let t1 = Expr::Sort(Level::succ(Level::zero()));
        assert!(types_compatible(&p, &t1));
        assert!(types_compatible(&p, &p));
    }
    #[test]
    fn test_types_compatible_consts() {
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let nat2 = Expr::Const(Name::str("Nat"), vec![]);
        let bool_e = Expr::Const(Name::str("Bool"), vec![]);
        assert!(types_compatible(&nat, &nat2));
        assert!(!types_compatible(&nat, &bool_e));
    }
    #[test]
    fn test_type_annotation_is_prop() {
        let ann = TypeAnnotation::new(
            Expr::Const(Name::str("proof"), vec![]),
            Expr::Sort(Level::zero()),
        );
        assert!(ann.is_prop());
        assert!(ann.type_is_sort());
    }
    #[test]
    fn test_typed_expr_into_pair() {
        let e = Expr::Const(Name::str("x"), vec![]);
        let t = Expr::Const(Name::str("Nat"), vec![]);
        let te = TypedExpr::new(e.clone(), t.clone());
        let (re, rt) = te.into_pair();
        assert_eq!(re, e);
        assert_eq!(rt, t);
    }
}
/// Normalize a type by unfolding `Sort(succ(zero))` to a canonical form label.
#[allow(dead_code)]
pub fn normalize_sort_label(expr: &Expr) -> &'static str {
    match expr {
        Expr::Sort(l) if l.is_zero() => "Prop",
        Expr::Sort(l) if l == &Level::succ(Level::zero()) => "Type",
        Expr::Sort(_) => "Sort(n)",
        _ => "non-sort",
    }
}
/// Collect all subexpressions of an expression.
#[allow(dead_code)]
pub fn collect_subexprs(expr: &Expr) -> Vec<Expr> {
    let mut result = vec![expr.clone()];
    match expr {
        Expr::App(f, a) => {
            result.extend(collect_subexprs(f));
            result.extend(collect_subexprs(a));
        }
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            result.extend(collect_subexprs(ty));
            result.extend(collect_subexprs(body));
        }
        Expr::Let(_, ty, val, body) => {
            result.extend(collect_subexprs(ty));
            result.extend(collect_subexprs(val));
            result.extend(collect_subexprs(body));
        }
        Expr::Proj(_, _, e) => result.extend(collect_subexprs(e)),
        _ => {}
    }
    result
}
#[cfg(test)]
mod typing_stack_tests {
    use super::*;
    use crate::infer_type::*;
    #[test]
    fn test_typing_stack_empty() {
        let stack = TypingStack::new();
        assert!(stack.is_empty());
        assert_eq!(stack.depth(), 0);
    }
    #[test]
    fn test_typing_stack_push_pop() {
        let mut stack = TypingStack::new();
        stack.push_frame();
        stack.bind("x".to_string(), Expr::Const(Name::str("Nat"), vec![]));
        assert_eq!(stack.depth(), 1);
        assert_eq!(stack.total_bindings(), 1);
        let frame = stack.pop_frame();
        assert_eq!(frame.len(), 1);
        assert_eq!(stack.depth(), 0);
    }
    #[test]
    fn test_typing_stack_lookup() {
        let mut stack = TypingStack::new();
        stack.push_frame();
        stack.bind("x".to_string(), Expr::Const(Name::str("Nat"), vec![]));
        let ty = stack.lookup("x");
        assert!(ty.is_some());
        assert_eq!(
            ty.expect("ty should be valid"),
            &Expr::Const(Name::str("Nat"), vec![])
        );
    }
    #[test]
    fn test_typing_stack_lookup_missing() {
        let mut stack = TypingStack::new();
        stack.push_frame();
        assert!(stack.lookup("z").is_none());
    }
    #[test]
    fn test_typing_stack_nested_lookup() {
        let mut stack = TypingStack::new();
        stack.push_frame();
        stack.bind("x".to_string(), Expr::Sort(Level::zero()));
        stack.push_frame();
        stack.bind("y".to_string(), Expr::Sort(Level::succ(Level::zero())));
        assert!(stack.lookup("x").is_some());
        assert!(stack.lookup("y").is_some());
        stack.pop_frame();
        assert!(stack.lookup("y").is_none());
        assert!(stack.lookup("x").is_some());
    }
    #[test]
    fn test_infer_error_display() {
        let e = InferError::UnboundBVar(3);
        assert!(e.to_string().contains("3"));
        let e2 = InferError::UnknownConst("Foo".to_string());
        assert!(e2.to_string().contains("Foo"));
    }
    #[test]
    fn test_type_with_provenance() {
        let ty = Expr::Sort(Level::zero());
        let src = Expr::Const(Name::str("p"), vec![]);
        let prov = TypeWithProvenance::new(ty.clone(), src, 2);
        assert!(prov.is_prop());
        assert_eq!(prov.depth, 2);
    }
    #[test]
    fn test_normalize_sort_label_prop() {
        let prop = Expr::Sort(Level::zero());
        assert_eq!(normalize_sort_label(&prop), "Prop");
    }
    #[test]
    fn test_normalize_sort_label_type() {
        let t1 = Expr::Sort(Level::succ(Level::zero()));
        assert_eq!(normalize_sort_label(&t1), "Type");
    }
    #[test]
    fn test_normalize_sort_label_non_sort() {
        let e = Expr::Const(Name::str("Nat"), vec![]);
        assert_eq!(normalize_sort_label(&e), "non-sort");
    }
    #[test]
    fn test_collect_subexprs_leaf() {
        let e = Expr::Lit(oxilean_kernel::Literal::Nat(0));
        let subs = collect_subexprs(&e);
        assert_eq!(subs.len(), 1);
    }
    #[test]
    fn test_collect_subexprs_app() {
        let e = Expr::App(
            Box::new(Expr::Const(Name::str("f"), vec![])),
            Box::new(Expr::Const(Name::str("a"), vec![])),
        );
        let subs = collect_subexprs(&e);
        assert!(subs.len() >= 3);
    }
}
#[cfg(test)]
mod infertype_ext2_tests {
    use super::*;
    use crate::infer_type::*;
    #[test]
    fn test_infertype_ext_util_basic() {
        let mut u = InferTypeExtUtil::new("test");
        u.push(10);
        u.push(20);
        assert_eq!(u.sum(), 30);
        assert_eq!(u.len(), 2);
    }
    #[test]
    fn test_infertype_ext_util_min_max() {
        let mut u = InferTypeExtUtil::new("mm");
        u.push(5);
        u.push(1);
        u.push(9);
        assert_eq!(u.min_val(), Some(1));
        assert_eq!(u.max_val(), Some(9));
    }
    #[test]
    fn test_infertype_ext_util_flags() {
        let mut u = InferTypeExtUtil::new("flags");
        u.set_flag(3);
        assert!(u.has_flag(3));
        assert!(!u.has_flag(2));
    }
    #[test]
    fn test_infertype_ext_util_pop() {
        let mut u = InferTypeExtUtil::new("pop");
        u.push(42);
        assert_eq!(u.pop(), Some(42));
        assert!(u.is_empty());
    }
    #[test]
    fn test_infertype_ext_map_basic() {
        let mut m: InferTypeExtMap<i32> = InferTypeExtMap::new();
        m.insert("key", 42);
        assert_eq!(m.get("key"), Some(&42));
        assert!(m.contains("key"));
        assert!(!m.contains("other"));
    }
    #[test]
    fn test_infertype_ext_map_get_or_default() {
        let mut m: InferTypeExtMap<i32> = InferTypeExtMap::new();
        m.insert("k", 5);
        assert_eq!(m.get_or_default("k"), 5);
        assert_eq!(m.get_or_default("missing"), 0);
    }
    #[test]
    fn test_infertype_ext_map_keys_sorted() {
        let mut m: InferTypeExtMap<i32> = InferTypeExtMap::new();
        m.insert("z", 1);
        m.insert("a", 2);
        m.insert("m", 3);
        let keys = m.keys_sorted();
        assert_eq!(keys[0].as_str(), "a");
        assert_eq!(keys[2].as_str(), "z");
    }
    #[test]
    fn test_infertype_window_mean() {
        let mut w = InferTypeWindow::new(3);
        w.push(1.0);
        w.push(2.0);
        w.push(3.0);
        assert!((w.mean() - 2.0).abs() < 1e-10);
    }
    #[test]
    fn test_infertype_window_evict() {
        let mut w = InferTypeWindow::new(2);
        w.push(10.0);
        w.push(20.0);
        w.push(30.0);
        assert_eq!(w.len(), 2);
        assert!((w.mean() - 25.0).abs() < 1e-10);
    }
    #[test]
    fn test_infertype_window_std_dev() {
        let mut w = InferTypeWindow::new(10);
        for i in 0..10 {
            w.push(i as f64);
        }
        assert!(w.std_dev() > 0.0);
    }
    #[test]
    fn test_infertype_builder_basic() {
        let b = InferTypeBuilder::new("test")
            .add_item("a")
            .add_item("b")
            .set_config("key", "val");
        assert_eq!(b.item_count(), 2);
        assert!(b.has_config("key"));
        assert_eq!(b.get_config("key"), Some("val"));
    }
    #[test]
    fn test_infertype_builder_summary() {
        let b = InferTypeBuilder::new("suite").add_item("x");
        let s = b.build_summary();
        assert!(s.contains("suite"));
    }
    #[test]
    fn test_infertype_state_machine_start() {
        let mut sm = InferTypeStateMachine::new();
        assert!(sm.start());
        assert!(sm.state.is_running());
    }
    #[test]
    fn test_infertype_state_machine_complete() {
        let mut sm = InferTypeStateMachine::new();
        sm.start();
        sm.complete();
        assert!(sm.state.is_terminal());
    }
    #[test]
    fn test_infertype_state_machine_fail() {
        let mut sm = InferTypeStateMachine::new();
        sm.fail("oops");
        assert!(sm.state.is_terminal());
        assert_eq!(sm.state.error_msg(), Some("oops"));
    }
    #[test]
    fn test_infertype_state_machine_no_transition_after_terminal() {
        let mut sm = InferTypeStateMachine::new();
        sm.complete();
        assert!(!sm.start());
    }
    #[test]
    fn test_infertype_work_queue_basic() {
        let mut wq = InferTypeWorkQueue::new(10);
        wq.enqueue("task1".to_string());
        wq.enqueue("task2".to_string());
        assert_eq!(wq.pending_count(), 2);
        let t = wq.dequeue();
        assert_eq!(t, Some("task1".to_string()));
        assert_eq!(wq.processed_count(), 1);
    }
    #[test]
    fn test_infertype_work_queue_capacity() {
        let mut wq = InferTypeWorkQueue::new(2);
        wq.enqueue("a".to_string());
        wq.enqueue("b".to_string());
        assert!(wq.is_full());
        assert!(!wq.enqueue("c".to_string()));
    }
    #[test]
    fn test_infertype_counter_map_basic() {
        let mut cm = InferTypeCounterMap::new();
        cm.increment("apple");
        cm.increment("apple");
        cm.increment("banana");
        assert_eq!(cm.count("apple"), 2);
        assert_eq!(cm.count("banana"), 1);
        assert_eq!(cm.num_unique(), 2);
    }
    #[test]
    fn test_infertype_counter_map_frequency() {
        let mut cm = InferTypeCounterMap::new();
        cm.increment("a");
        cm.increment("a");
        cm.increment("b");
        assert!((cm.frequency("a") - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_infertype_counter_map_most_common() {
        let mut cm = InferTypeCounterMap::new();
        cm.increment("x");
        cm.increment("y");
        cm.increment("x");
        let (k, v) = cm.most_common().expect("most_common should succeed");
        assert_eq!(k.as_str(), "x");
        assert_eq!(v, 2);
    }
}
#[cfg(test)]
mod infertype_analysis_tests {
    use super::*;
    use crate::infer_type::*;
    #[test]
    fn test_infertype_result_ok() {
        let r = InferTypeResult::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_infertype_result_err() {
        let r = InferTypeResult::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_infertype_result_partial() {
        let r = InferTypeResult::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_infertype_result_skipped() {
        let r = InferTypeResult::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_infertype_analysis_pass_run() {
        let mut p = InferTypeAnalysisPass::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_infertype_analysis_pass_empty_input() {
        let mut p = InferTypeAnalysisPass::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_infertype_analysis_pass_success_rate() {
        let mut p = InferTypeAnalysisPass::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_infertype_analysis_pass_disable() {
        let mut p = InferTypeAnalysisPass::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_infertype_pipeline_basic() {
        let mut pipeline = InferTypePipeline::new("main_pipeline");
        pipeline.add_pass(InferTypeAnalysisPass::new("pass1"));
        pipeline.add_pass(InferTypeAnalysisPass::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_infertype_pipeline_disabled_pass() {
        let mut pipeline = InferTypePipeline::new("partial");
        let mut p = InferTypeAnalysisPass::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(InferTypeAnalysisPass::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_infertype_diff_basic() {
        let mut d = InferTypeDiff::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_infertype_diff_summary() {
        let mut d = InferTypeDiff::new();
        d.add("x");
        d.add("y");
        d.remove("z");
        let s = d.summary();
        assert!(s.contains("+2"));
    }
    #[test]
    fn test_infertype_config_set_get() {
        let mut cfg = InferTypeConfig::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_infertype_config_read_only() {
        let mut cfg = InferTypeConfig::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_infertype_config_remove() {
        let mut cfg = InferTypeConfig::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_infertype_diagnostics_basic() {
        let mut diag = InferTypeDiagnostics::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_infertype_diagnostics_max_errors() {
        let mut diag = InferTypeDiagnostics::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_infertype_diagnostics_clear() {
        let mut diag = InferTypeDiagnostics::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_infertype_config_value_types() {
        let b = InferTypeConfigValue::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = InferTypeConfigValue::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = InferTypeConfigValue::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = InferTypeConfigValue::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = InferTypeConfigValue::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
#[cfg(test)]
mod infer_type_ext_tests_2300 {
    use super::*;
    use crate::infer_type::*;
    #[test]
    fn test_infer_type_ext_result_ok_2300() {
        let r = InferTypeExtResult2300::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_infer_type_ext_result_err_2300() {
        let r = InferTypeExtResult2300::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_infer_type_ext_result_partial_2300() {
        let r = InferTypeExtResult2300::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_infer_type_ext_result_skipped_2300() {
        let r = InferTypeExtResult2300::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_infer_type_ext_pass_run_2300() {
        let mut p = InferTypeExtPass2300::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_infer_type_ext_pass_empty_2300() {
        let mut p = InferTypeExtPass2300::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_infer_type_ext_pass_rate_2300() {
        let mut p = InferTypeExtPass2300::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_infer_type_ext_pass_disable_2300() {
        let mut p = InferTypeExtPass2300::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_infer_type_ext_pipeline_basic_2300() {
        let mut pipeline = InferTypeExtPipeline2300::new("main_pipeline");
        pipeline.add_pass(InferTypeExtPass2300::new("pass1"));
        pipeline.add_pass(InferTypeExtPass2300::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_infer_type_ext_pipeline_disabled_2300() {
        let mut pipeline = InferTypeExtPipeline2300::new("partial");
        let mut p = InferTypeExtPass2300::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(InferTypeExtPass2300::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_infer_type_ext_diff_basic_2300() {
        let mut d = InferTypeExtDiff2300::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_infer_type_ext_config_set_get_2300() {
        let mut cfg = InferTypeExtConfig2300::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_infer_type_ext_config_read_only_2300() {
        let mut cfg = InferTypeExtConfig2300::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_infer_type_ext_config_remove_2300() {
        let mut cfg = InferTypeExtConfig2300::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_infer_type_ext_diagnostics_basic_2300() {
        let mut diag = InferTypeExtDiag2300::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_infer_type_ext_diagnostics_max_errors_2300() {
        let mut diag = InferTypeExtDiag2300::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_infer_type_ext_diagnostics_clear_2300() {
        let mut diag = InferTypeExtDiag2300::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_infer_type_ext_config_value_types_2300() {
        let b = InferTypeExtConfigVal2300::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = InferTypeExtConfigVal2300::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = InferTypeExtConfigVal2300::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = InferTypeExtConfigVal2300::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = InferTypeExtConfigVal2300::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
