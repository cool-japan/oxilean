//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    AnfConverter, EraseConfig, ErasedAst, ErasedBetaReducer, ErasedBitOps, ErasedCallSite,
    ErasedClosureEnv, ErasedCodegen, ErasedConstantPool, ErasedDCE, ErasedDecl, ErasedEnv,
    ErasedExpr, ErasedExprExt, ErasedFlatApp, ErasedHeapObj, ErasedInliner, ErasedInterpreter,
    ErasedLetChain, ErasedLiveness, ErasedMatchArm, ErasedModule, ErasedNormalizer,
    ErasedOptimizer, ErasedPattern, ErasedPrinter, ErasedReachability, ErasedRenamer, ErasedScope,
    ErasedSizeBound, ErasedStack, ErasedSubstMap, ErasedTupleOps, ErasedTypeMap, ErasedValue,
    ErasureContext, ErasurePass, ErasureStats, TypeEraser,
};

/// Substitute `TypeErased` for `BVar(target)` in an erased expression.
///
/// De Bruijn shifting is not implemented here for simplicity; this is
/// a best-effort helper used during optimisation.
pub(super) fn subst_bvar(expr: ErasedExpr, target: u32, replacement: &ErasedExpr) -> ErasedExpr {
    match expr {
        ErasedExpr::BVar(i) if i == target => replacement.clone(),
        ErasedExpr::Lam(body) => {
            ErasedExpr::Lam(Box::new(subst_bvar(*body, target + 1, replacement)))
        }
        ErasedExpr::App(f, arg) => ErasedExpr::App(
            Box::new(subst_bvar(*f, target, replacement)),
            Box::new(subst_bvar(*arg, target, replacement)),
        ),
        ErasedExpr::Let(val, body) => ErasedExpr::Let(
            Box::new(subst_bvar(*val, target, replacement)),
            Box::new(subst_bvar(*body, target + 1, replacement)),
        ),
        other => other,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_erase_config_default() {
        let cfg = EraseConfig::default();
        assert!(!cfg.keep_props);
        assert!(!cfg.inline_defs);
    }
    #[test]
    fn test_erase_sort() {
        let eraser = TypeEraser::new();
        assert_eq!(eraser.erase_sort(), ErasedExpr::TypeErased);
    }
    #[test]
    fn test_erase_pi() {
        let eraser = TypeEraser::new();
        assert_eq!(eraser.erase_pi(), ErasedExpr::TypeErased);
    }
    #[test]
    fn test_erase_lam() {
        let eraser = TypeEraser::new();
        let body = ErasedExpr::BVar(0);
        let lam = eraser.erase_lam_body(body.clone());
        assert_eq!(lam, ErasedExpr::Lam(Box::new(body)));
    }
    #[test]
    fn test_erase_app() {
        let f = ErasedExpr::Var("f".to_string());
        let arg = ErasedExpr::Lit(42);
        let app = TypeEraser::erase_app(f.clone(), arg.clone());
        assert_eq!(app, ErasedExpr::App(Box::new(f), Box::new(arg)));
    }
    #[test]
    fn test_erase_lit() {
        assert_eq!(TypeEraser::erase_lit(0), ErasedExpr::Lit(0));
        assert_eq!(TypeEraser::erase_lit(999), ErasedExpr::Lit(999));
    }
    #[test]
    fn test_erasure_stats_ratio() {
        let mut stats = ErasureStats::new();
        assert_eq!(stats.ratio_erased(), 0.0);
        stats.add_sort();
        stats.add_pi();
        stats.add_term();
        stats.add_term();
        assert!((stats.ratio_erased() - 0.5).abs() < 1e-10);
    }
    #[test]
    fn test_optimize_type_erased() {
        let eraser = TypeEraser::new();
        let app = ErasedExpr::App(
            Box::new(ErasedExpr::TypeErased),
            Box::new(ErasedExpr::Lit(1)),
        );
        assert_eq!(eraser.optimize(app), ErasedExpr::TypeErased);
        let lam_body = ErasedExpr::BVar(0);
        let lam = ErasedExpr::Lam(Box::new(lam_body));
        let app2 = ErasedExpr::App(Box::new(lam), Box::new(ErasedExpr::TypeErased));
        assert_eq!(eraser.optimize(app2), ErasedExpr::TypeErased);
        assert_eq!(eraser.optimize(ErasedExpr::Lit(7)), ErasedExpr::Lit(7));
    }
}
/// Substitute BVar(0) with `replacement` in `expr`, shifting free variables.
#[allow(dead_code)]
pub fn subst_bvar0(expr: ErasedExprExt, replacement: ErasedExprExt) -> ErasedExprExt {
    subst_bvar_rec(expr, 0, &replacement, 0)
}
pub(super) fn subst_bvar_rec(
    expr: ErasedExprExt,
    target: u32,
    replacement: &ErasedExprExt,
    depth: u32,
) -> ErasedExprExt {
    match expr {
        ErasedExprExt::BVar(i) => {
            if i == target + depth {
                shift_up(replacement.clone(), depth)
            } else if i > target + depth {
                ErasedExprExt::BVar(i - 1)
            } else {
                ErasedExprExt::BVar(i)
            }
        }
        ErasedExprExt::Lam(b) => {
            ErasedExprExt::Lam(Box::new(subst_bvar_rec(*b, target, replacement, depth + 1)))
        }
        ErasedExprExt::App(f, x) => ErasedExprExt::App(
            Box::new(subst_bvar_rec(*f, target, replacement, depth)),
            Box::new(subst_bvar_rec(*x, target, replacement, depth)),
        ),
        ErasedExprExt::Let(v, b) => ErasedExprExt::Let(
            Box::new(subst_bvar_rec(*v, target, replacement, depth)),
            Box::new(subst_bvar_rec(*b, target, replacement, depth + 1)),
        ),
        other => other,
    }
}
pub(super) fn shift_up(expr: ErasedExprExt, amount: u32) -> ErasedExprExt {
    if amount == 0 {
        return expr;
    }
    match expr {
        ErasedExprExt::BVar(i) => ErasedExprExt::BVar(i + amount),
        ErasedExprExt::Lam(b) => ErasedExprExt::Lam(Box::new(shift_up(*b, amount))),
        ErasedExprExt::App(f, x) => ErasedExprExt::App(
            Box::new(shift_up(*f, amount)),
            Box::new(shift_up(*x, amount)),
        ),
        ErasedExprExt::Let(v, b) => ErasedExprExt::Let(
            Box::new(shift_up(*v, amount)),
            Box::new(shift_up(*b, amount)),
        ),
        other => other,
    }
}
/// Counts free variables in an erased expression.
#[allow(dead_code)]
pub fn count_free_vars(expr: &ErasedExprExt) -> usize {
    count_fv_rec(expr, 0)
}
fn count_fv_rec(expr: &ErasedExprExt, depth: u32) -> usize {
    match expr {
        ErasedExprExt::BVar(i) => {
            if *i >= depth {
                1
            } else {
                0
            }
        }
        ErasedExprExt::FVar(_) => 1,
        ErasedExprExt::Lam(b) => count_fv_rec(b, depth + 1),
        ErasedExprExt::App(f, x) => count_fv_rec(f, depth) + count_fv_rec(x, depth),
        ErasedExprExt::Let(v, b) => count_fv_rec(v, depth) + count_fv_rec(b, depth + 1),
        _ => 0,
    }
}
/// Collects all constant references in an expression.
#[allow(dead_code)]
pub fn collect_consts(expr: &ErasedExprExt) -> Vec<String> {
    let mut consts = Vec::new();
    collect_consts_rec(expr, &mut consts);
    consts
}
pub(super) fn collect_consts_rec(expr: &ErasedExprExt, out: &mut Vec<String>) {
    match expr {
        ErasedExprExt::Const(name) => {
            if !out.contains(name) {
                out.push(name.clone());
            }
        }
        ErasedExprExt::Lam(b) => collect_consts_rec(b, out),
        ErasedExprExt::App(f, x) => {
            collect_consts_rec(f, out);
            collect_consts_rec(x, out);
        }
        ErasedExprExt::Let(v, b) => {
            collect_consts_rec(v, out);
            collect_consts_rec(b, out);
        }
        _ => {}
    }
}
#[cfg(test)]
mod tests_erased_expr {
    use super::*;
    #[test]
    fn test_erased_expr_size() {
        let e = ErasedExprExt::App(
            Box::new(ErasedExprExt::Lam(Box::new(ErasedExprExt::BVar(0)))),
            Box::new(ErasedExprExt::Lit(42)),
        );
        assert_eq!(e.size(), 4);
    }
    #[test]
    fn test_erased_expr_predicates() {
        assert!(ErasedExprExt::Lit(0).is_lit());
        assert!(ErasedExprExt::Lam(Box::new(ErasedExprExt::Unit)).is_lam());
        assert!(ErasedExprExt::TypeErased.is_type_erased());
    }
    #[test]
    fn test_subst_bvar0() {
        let body = ErasedExprExt::BVar(0);
        let result = subst_bvar0(body, ErasedExprExt::Lit(42));
        assert_eq!(result, ErasedExprExt::Lit(42));
    }
    #[test]
    fn test_beta_reduce_lam_app() {
        let lam = ErasedExprExt::Lam(Box::new(ErasedExprExt::BVar(0)));
        let app = ErasedExprExt::App(Box::new(lam), Box::new(ErasedExprExt::Lit(7)));
        let mut reducer = ErasedBetaReducer::new(100);
        let result = reducer.step(app);
        assert_eq!(result, ErasedExprExt::Lit(7));
        assert_eq!(reducer.steps, 1);
    }
    #[test]
    fn test_count_free_vars() {
        let e1 = ErasedExprExt::Lam(Box::new(ErasedExprExt::BVar(0)));
        assert_eq!(count_free_vars(&e1), 0);
        let e2 = ErasedExprExt::BVar(0);
        assert_eq!(count_free_vars(&e2), 1);
    }
    #[test]
    fn test_collect_consts() {
        let e = ErasedExprExt::App(
            Box::new(ErasedExprExt::Const("Nat.add".to_string())),
            Box::new(ErasedExprExt::App(
                Box::new(ErasedExprExt::Const("Nat.add".to_string())),
                Box::new(ErasedExprExt::Lit(1)),
            )),
        );
        let consts = collect_consts(&e);
        assert_eq!(consts, vec!["Nat.add".to_string()]);
    }
    #[test]
    fn test_erased_inliner() {
        let mut inliner = ErasedInliner::new();
        inliner.register("id", ErasedExprExt::Lam(Box::new(ErasedExprExt::BVar(0))));
        let e = ErasedExprExt::App(
            Box::new(ErasedExprExt::Const("id".to_string())),
            Box::new(ErasedExprExt::Lit(99)),
        );
        let result = inliner.inline(e);
        assert_eq!(inliner.inlined, 1);
        assert!(result.is_app());
    }
    #[test]
    fn test_erased_dce() {
        let e = ErasedExprExt::App(
            Box::new(ErasedExprExt::Lit(5)),
            Box::new(ErasedExprExt::TypeErased),
        );
        let mut dce = ErasedDCE::new();
        let result = dce.elim(e);
        assert_eq!(result, ErasedExprExt::Lit(5));
        assert_eq!(dce.eliminated, 1);
    }
    #[test]
    fn test_erased_optimizer() {
        let mut opt = ErasedOptimizer::new(100);
        let lam_body = ErasedExprExt::BVar(0);
        let lam = ErasedExprExt::App(
            Box::new(ErasedExprExt::Lam(Box::new(lam_body))),
            Box::new(ErasedExprExt::TypeErased),
        );
        let result = opt.optimize(lam);
        assert_eq!(result, ErasedExprExt::TypeErased);
    }
}
/// Pretty prints an erased expression to a string.
#[allow(dead_code)]
pub fn pretty_print_erased(expr: &ErasedExprExt) -> String {
    match expr {
        ErasedExprExt::BVar(i) => format!("#{}", i),
        ErasedExprExt::FVar(name) => name.clone(),
        ErasedExprExt::Lit(n) => n.to_string(),
        ErasedExprExt::CtorTag(t) => format!("ctor({})", t),
        ErasedExprExt::Lam(b) => format!("(λ. {})", pretty_print_erased(b)),
        ErasedExprExt::App(f, x) => {
            format!("({} {})", pretty_print_erased(f), pretty_print_erased(x))
        }
        ErasedExprExt::Const(name) => name.clone(),
        ErasedExprExt::Let(v, b) => {
            format!(
                "(let {} in {})",
                pretty_print_erased(v),
                pretty_print_erased(b)
            )
        }
        ErasedExprExt::TypeErased => "_".to_string(),
        ErasedExprExt::Unit => "()".to_string(),
    }
}
/// Computes the lambda depth of an expression.
#[allow(dead_code)]
pub fn lambda_depth(expr: &ErasedExprExt) -> u32 {
    match expr {
        ErasedExprExt::Lam(b) => 1 + lambda_depth(b),
        ErasedExprExt::App(f, x) => lambda_depth(f).max(lambda_depth(x)),
        ErasedExprExt::Let(v, b) => lambda_depth(v).max(lambda_depth(b)),
        _ => 0,
    }
}
/// Count the number of App nodes.
#[allow(dead_code)]
pub fn count_apps(expr: &ErasedExprExt) -> usize {
    match expr {
        ErasedExprExt::App(f, x) => 1 + count_apps(f) + count_apps(x),
        ErasedExprExt::Lam(b) => count_apps(b),
        ErasedExprExt::Let(v, b) => count_apps(v) + count_apps(b),
        _ => 0,
    }
}
/// Return whether BVar(0) appears free in expr at the given depth.
#[allow(dead_code)]
pub fn has_free_bvar(expr: &ErasedExprExt, depth: u32) -> bool {
    match expr {
        ErasedExprExt::BVar(i) => *i == depth,
        ErasedExprExt::Lam(b) => has_free_bvar(b, depth + 1),
        ErasedExprExt::App(f, x) => has_free_bvar(f, depth) || has_free_bvar(x, depth),
        ErasedExprExt::Let(v, b) => has_free_bvar(v, depth) || has_free_bvar(b, depth + 1),
        _ => false,
    }
}
pub(super) fn is_atom(expr: &ErasedExprExt) -> bool {
    matches!(
        expr,
        ErasedExprExt::BVar(_)
            | ErasedExprExt::FVar(_)
            | ErasedExprExt::Lit(_)
            | ErasedExprExt::Const(_)
            | ErasedExprExt::Unit
            | ErasedExprExt::TypeErased
    )
}
#[cfg(test)]
mod tests_erasure_extended {
    use super::*;
    #[test]
    fn test_erased_pattern_depth() {
        let p = ErasedPattern::Ctor(
            0,
            vec![
                ErasedPattern::Ctor(1, vec![ErasedPattern::Wildcard]),
                ErasedPattern::Lit(0),
            ],
        );
        assert_eq!(p.depth(), 3);
    }
    #[test]
    fn test_erased_pattern_irrefutable() {
        assert!(ErasedPattern::Wildcard.is_irrefutable());
        assert!(ErasedPattern::Var("x".to_string()).is_irrefutable());
        assert!(!ErasedPattern::Lit(0).is_irrefutable());
    }
    #[test]
    fn test_erased_match_arm() {
        let arm = ErasedMatchArm::new(ErasedPattern::Wildcard, ErasedExprExt::Unit);
        assert!(arm.is_catch_all());
        let arm2 = ErasedMatchArm::new(ErasedPattern::Lit(0), ErasedExprExt::Lit(0));
        assert!(!arm2.is_catch_all());
    }
    #[test]
    fn test_erased_module() {
        let mut m = ErasedModule::new("Nat");
        m.add(ErasedDecl::Def {
            name: "zero".to_string(),
            body: ErasedExprExt::Lit(0),
        });
        m.add(ErasedDecl::Axiom {
            name: "propext".to_string(),
        });
        m.add(ErasedDecl::Inductive {
            name: "Nat".to_string(),
            ctor_count: 2,
        });
        assert_eq!(m.len(), 3);
        assert_eq!(
            m.find("zero").expect("value should be present").name(),
            "zero"
        );
        assert_eq!(m.function_names(), vec!["zero"]);
    }
    #[test]
    fn test_erasure_context() {
        let mut ctx = ErasureContext::new();
        ctx.push(true);
        ctx.push(false);
        assert!(!ctx.is_type_at(0));
        assert!(ctx.is_type_at(1));
        assert_eq!(ctx.depth(), 2);
    }
    #[test]
    fn test_erased_scope() {
        let mut scope = ErasedScope::new();
        scope.bind("x", ErasedValue::Int(42));
        scope.bind("y", ErasedValue::Int(7));
        assert!(scope.lookup("x").is_some());
        let cp = scope.save();
        scope.bind("z", ErasedValue::Unit);
        assert_eq!(scope.depth(), 3);
        scope.restore(cp);
        assert_eq!(scope.depth(), 2);
        assert!(scope.lookup("z").is_none());
    }
    #[test]
    fn test_erased_type_map() {
        let mut m = ErasedTypeMap::new();
        m.insert(1, ErasedExprExt::Lit(42));
        m.insert(2, ErasedExprExt::Unit);
        assert_eq!(m.get(1), Some(&ErasedExprExt::Lit(42)));
        assert_eq!(m.get(99), None);
        m.insert(1, ErasedExprExt::Lit(99));
        assert_eq!(m.get(1), Some(&ErasedExprExt::Lit(99)));
    }
    #[test]
    fn test_pretty_print_erased() {
        let e = ErasedExprExt::App(
            Box::new(ErasedExprExt::Const("f".to_string())),
            Box::new(ErasedExprExt::Lit(5)),
        );
        assert_eq!(pretty_print_erased(&e), "(f 5)");
        let lam = ErasedExprExt::Lam(Box::new(ErasedExprExt::BVar(0)));
        assert_eq!(pretty_print_erased(&lam), "(λ. #0)");
    }
    #[test]
    fn test_lambda_depth() {
        let e = ErasedExprExt::Lam(Box::new(ErasedExprExt::Lam(Box::new(ErasedExprExt::BVar(
            0,
        )))));
        assert_eq!(lambda_depth(&e), 2);
    }
    #[test]
    fn test_count_apps() {
        let e = ErasedExprExt::App(
            Box::new(ErasedExprExt::App(
                Box::new(ErasedExprExt::Const("f".to_string())),
                Box::new(ErasedExprExt::Lit(1)),
            )),
            Box::new(ErasedExprExt::Lit(2)),
        );
        assert_eq!(count_apps(&e), 2);
    }
    #[test]
    fn test_has_free_bvar() {
        let e = ErasedExprExt::BVar(0);
        assert!(has_free_bvar(&e, 0));
        assert!(!has_free_bvar(&e, 1));
        let lam = ErasedExprExt::Lam(Box::new(ErasedExprExt::BVar(0)));
        assert!(!has_free_bvar(&lam, 0));
    }
    #[test]
    fn test_anf_converter() {
        let mut conv = AnfConverter::new();
        let e = ErasedExprExt::App(
            Box::new(ErasedExprExt::App(
                Box::new(ErasedExprExt::Const("f".to_string())),
                Box::new(ErasedExprExt::Lit(1)),
            )),
            Box::new(ErasedExprExt::Lit(2)),
        );
        let anf = conv.convert(e);
        assert!(
            conv.let_count > 0
                || matches!(anf, ErasedExprExt::Let(_, _) | ErasedExprExt::App(_, _))
        );
    }
    #[test]
    fn test_erased_reachability() {
        let mut ra = ErasedReachability::new();
        ra.add_root("main");
        ra.mark_reachable("helper");
        assert!(ra.is_reachable("main"));
        assert!(ra.is_reachable("helper"));
        assert!(!ra.is_reachable("dead_fn"));
        assert_eq!(ra.reachable_count(), 2);
    }
    #[test]
    fn test_erased_codegen() {
        let mut cg = ErasedCodegen::new();
        let mut m = ErasedModule::new("Test");
        m.add(ErasedDecl::Def {
            name: "id".to_string(),
            body: ErasedExprExt::Lam(Box::new(ErasedExprExt::BVar(0))),
        });
        cg.gen_module(&m);
        assert!(cg.output.contains("id"));
        assert!(cg.output.contains("fun x"));
    }
}
#[cfg(test)]
mod tests_erasure_extended2 {
    use super::*;
    #[test]
    fn test_erased_interpreter_lit() {
        let mut interp = ErasedInterpreter::new(100);
        let result = interp.eval(ErasedExprExt::Lit(42));
        assert_eq!(result, Some(ErasedValue::Int(42)));
    }
    #[test]
    fn test_erased_interpreter_app_lam() {
        let mut interp = ErasedInterpreter::new(100);
        let lam = ErasedExprExt::Lam(Box::new(ErasedExprExt::BVar(0)));
        let app = ErasedExprExt::App(Box::new(lam), Box::new(ErasedExprExt::Lit(7)));
        let result = interp.eval(app);
        assert_eq!(result, Some(ErasedValue::Int(7)));
    }
    #[test]
    fn test_erased_heap_obj() {
        let lit = ErasedHeapObj::Lit(42);
        assert!(!lit.is_ctor());
        let ctor = ErasedHeapObj::Ctor {
            tag: 1,
            fields: vec![],
        };
        assert!(ctor.is_ctor());
        assert_eq!(ctor.ctor_tag(), Some(1));
        let closure = ErasedHeapObj::Closure {
            arity: 1,
            fn_ptr: 0,
            num_caps: 0,
        };
        assert!(closure.is_closure());
        let thunk = ErasedHeapObj::Thunk { code: 999 };
        assert!(thunk.is_thunk());
    }
    #[test]
    fn test_erased_renamer() {
        let mut r = ErasedRenamer::new(vec![("old_fn".to_string(), "new_fn".to_string())]);
        let e = ErasedExprExt::Const("old_fn".to_string());
        let result = r.rename(e);
        assert_eq!(result, ErasedExprExt::Const("new_fn".to_string()));
        assert_eq!(r.renames, 1);
    }
    #[test]
    fn test_erasure_pass_pipeline() {
        let mut pass = ErasurePass::new("test_pass");
        let lam_body = ErasedExprExt::BVar(0);
        let app2 = ErasedExprExt::App(
            Box::new(ErasedExprExt::Lam(Box::new(lam_body))),
            Box::new(ErasedExprExt::TypeErased),
        );
        assert_eq!(pass.run(app2), ErasedExprExt::TypeErased);
        let plain = ErasedExprExt::Lit(7);
        assert_eq!(pass.run(plain), ErasedExprExt::Lit(7));
    }
    #[test]
    fn test_size_bound_checker() {
        let checker = ErasedSizeBound::new(10);
        let small = ErasedExprExt::Lit(1);
        assert!(checker.check(&small));
        assert_eq!(checker.size_of(&small), 1);
        let big = ErasedExprExt::App(
            Box::new(ErasedExprExt::Lam(Box::new(ErasedExprExt::App(
                Box::new(ErasedExprExt::BVar(0)),
                Box::new(ErasedExprExt::Lit(1)),
            )))),
            Box::new(ErasedExprExt::App(
                Box::new(ErasedExprExt::Const("f".to_string())),
                Box::new(ErasedExprExt::App(
                    Box::new(ErasedExprExt::Lit(2)),
                    Box::new(ErasedExprExt::Lit(3)),
                )),
            )),
        );
        let size = checker.size_of(&big);
        assert!(size > 5);
    }
    #[test]
    fn test_erased_printer() {
        let mut printer = ErasedPrinter::new();
        printer.print(&ErasedExprExt::Lit(42));
        assert!(printer.result().contains("42"));
        printer.clear();
        assert!(printer.result().is_empty());
    }
    #[test]
    fn test_erased_tuple_ops() {
        let pair = ErasedTupleOps::make_pair(ErasedExprExt::Lit(1), ErasedExprExt::Lit(2));
        let fst = ErasedTupleOps::fst(pair.clone());
        assert!(matches!(fst, ErasedExprExt::App(_, _)));
        let snd = ErasedTupleOps::snd(pair);
        assert!(matches!(snd, ErasedExprExt::App(_, _)));
        let triple = ErasedTupleOps::n_tuple(vec![
            ErasedExprExt::Lit(1),
            ErasedExprExt::Lit(2),
            ErasedExprExt::Lit(3),
        ]);
        assert!(matches!(triple, ErasedExprExt::App(_, _)));
    }
}
/// Flatten left-associated App chains into (head, [arg1, arg2, ...]).
#[allow(dead_code)]
pub fn flatten_apps(expr: ErasedExpr) -> (ErasedExpr, Vec<ErasedExpr>) {
    let mut args = Vec::new();
    let mut cur = expr;
    loop {
        match cur {
            ErasedExpr::App(f, x) => {
                args.push(*x);
                cur = *f;
            }
            other => {
                args.reverse();
                return (other, args);
            }
        }
    }
}
/// Build a left-associated App chain from (head, [arg1, arg2, ...]).
#[allow(dead_code)]
pub fn build_apps(head: ErasedExpr, args: Vec<ErasedExpr>) -> ErasedExpr {
    args.into_iter()
        .fold(head, |f, x| ErasedExpr::App(Box::new(f), Box::new(x)))
}
#[cfg(test)]
mod tests_erasure_normalizer {
    use super::*;
    #[test]
    fn test_erased_normalizer_whnf() {
        let mut norm = ErasedNormalizer::new(100);
        let app = ErasedExpr::App(
            Box::new(ErasedExpr::Lam(Box::new(ErasedExpr::BVar(0)))),
            Box::new(ErasedExpr::Lit(5)),
        );
        let result = norm.whnf(app);
        assert_eq!(result, ErasedExpr::Lit(5));
        assert_eq!(norm.beta_steps, 1);
    }
    #[test]
    fn test_erased_normalizer_const_fold() {
        let mut norm = ErasedNormalizer::new(100);
        let add_3_4 = ErasedExpr::App(
            Box::new(ErasedExpr::App(
                Box::new(ErasedExpr::Const("Nat.add".to_string())),
                Box::new(ErasedExpr::Lit(3)),
            )),
            Box::new(ErasedExpr::Lit(4)),
        );
        let result = norm.const_fold_add(add_3_4);
        assert_eq!(result, ErasedExpr::Lit(7));
        assert_eq!(norm.const_folds, 1);
    }
    #[test]
    fn test_erased_stack() {
        let mut stack = ErasedStack::new();
        stack.push(ErasedExpr::Lit(1));
        stack.push(ErasedExpr::Lit(2));
        assert_eq!(stack.depth(), 2);
        assert_eq!(stack.pop(), Some(ErasedExpr::Lit(2)));
        assert_eq!(stack.peek(), Some(&ErasedExpr::Lit(1)));
    }
    #[test]
    fn test_erased_env() {
        let mut env = ErasedEnv::new();
        env.bind("x", ErasedExpr::Lit(42));
        env.bind("y", ErasedExpr::Lit(7));
        assert_eq!(env.get("x"), Some(&ErasedExpr::Lit(42)));
        assert_eq!(env.get("z"), None);
    }
    #[test]
    fn test_flatten_build_apps() {
        let expr = ErasedExpr::App(
            Box::new(ErasedExpr::App(
                Box::new(ErasedExpr::Const("f".to_string())),
                Box::new(ErasedExpr::Lit(1)),
            )),
            Box::new(ErasedExpr::Lit(2)),
        );
        let (head, args) = flatten_apps(expr);
        assert_eq!(head, ErasedExpr::Const("f".to_string()));
        assert_eq!(args.len(), 2);
        assert_eq!(args[0], ErasedExpr::Lit(1));
        assert_eq!(args[1], ErasedExpr::Lit(2));
        let rebuilt = build_apps(head, args);
        if let ErasedExpr::App(outer, arg2) = rebuilt {
            assert_eq!(*arg2, ErasedExpr::Lit(2));
            if let ErasedExpr::App(inner_f, arg1) = *outer {
                assert_eq!(*inner_f, ErasedExpr::Const("f".to_string()));
                assert_eq!(*arg1, ErasedExpr::Lit(1));
            }
        }
    }
    #[test]
    fn test_erased_bit_ops() {
        assert_eq!(ErasedBitOps::fold_and(&[0xFF, 0x0F, 0x3F]), 0x0F);
        assert_eq!(ErasedBitOps::fold_or(&[0x01, 0x02, 0x04]), 0x07);
        assert_eq!(ErasedBitOps::fold_binop(&[1, 2, 3, 4], 0, |a, b| a + b), 10);
    }
}
/// Count lambda depth in an ErasedExpr.
#[allow(dead_code)]
pub(super) fn erased_expr_depth(expr: &ErasedExpr) -> u32 {
    match expr {
        ErasedExpr::Lam(b) => 1 + erased_expr_depth(b),
        ErasedExpr::App(f, _) => erased_expr_depth(f),
        _ => 0,
    }
}
/// Count apps in an ErasedExpr.
#[allow(dead_code)]
pub(super) fn erased_expr_apps(expr: &ErasedExpr) -> usize {
    match expr {
        ErasedExpr::App(f, x) => 1 + erased_expr_apps(f) + erased_expr_apps(x),
        ErasedExpr::Lam(b) => erased_expr_apps(b),
        ErasedExpr::Let(v, b) => erased_expr_apps(v) + erased_expr_apps(b),
        _ => 0,
    }
}
/// A generic transformation pass over erased expressions.
#[allow(dead_code)]
pub trait ErasedPass {
    /// Name of the pass, for diagnostic output.
    fn name(&self) -> &str;
    /// Run the pass on `expr`, returning the (possibly transformed) result.
    fn run(&mut self, expr: ErasedExpr) -> ErasedExpr;
    /// Run the pass on a slice of declarations.
    fn run_on_module(&mut self, _decls: &mut Vec<ErasedDecl>) {}
}
#[cfg(test)]
mod tests_erasure_extra {
    use super::*;
    #[test]
    fn test_erased_let_chain_empty() {
        let body = ErasedExprExt::Unit;
        let chain = ErasedLetChain::new(body);
        assert!(chain.is_empty());
        assert_eq!(chain.len(), 0);
    }
    #[test]
    fn test_erased_let_chain_push_and_into() {
        let body = ErasedExprExt::BVar(0);
        let mut chain = ErasedLetChain::new(body);
        chain.push("x", ErasedExprExt::Lit(42));
        assert_eq!(chain.len(), 1);
        let expr = chain.into_expr();
        match expr {
            ErasedExprExt::Let(rhs, _body2) => {
                assert!(matches!(*rhs, ErasedExprExt::Lit(42)));
            }
            _ => panic!("expected Let"),
        }
    }
    #[test]
    fn test_closure_env_lookup() {
        let mut env = ErasedClosureEnv::new();
        env.capture("x", ErasedExprExt::Lit(10));
        env.capture("y", ErasedExprExt::Lit(20));
        assert!(env.lookup("x").is_some());
        assert!(env.lookup("z").is_none());
        assert_eq!(env.size(), 2);
    }
    #[test]
    fn test_call_site_self_tail() {
        let cs = ErasedCallSite::new("foo", 2, true);
        assert!(cs.is_self_tail("foo"));
        assert!(!cs.is_self_tail("bar"));
    }
    #[test]
    fn test_liveness_merge() {
        let mut a = ErasedLiveness::new();
        a.mark_live(0);
        a.mark_live(2);
        let mut b = ErasedLiveness::new();
        b.mark_live(1);
        b.mark_live(2);
        a.merge(&b);
        assert!(a.is_live(0));
        assert!(a.is_live(1));
        assert!(a.is_live(2));
        assert_eq!(a.count(), 3);
        assert_eq!(a.max_live(), Some(2));
    }
    #[test]
    fn test_constant_pool_intern() {
        let mut pool = ErasedConstantPool::new();
        let idx1 = pool.intern(99);
        let idx2 = pool.intern(99);
        assert_eq!(idx1, idx2);
        let idx3 = pool.intern(100);
        assert_ne!(idx1, idx3);
        assert_eq!(pool.get(idx1), Some(99));
        assert_eq!(pool.len(), 2);
    }
    #[test]
    fn test_flat_app_round_trip() {
        let expr = ErasedExpr::App(
            Box::new(ErasedExpr::App(
                Box::new(ErasedExpr::Const("f".into())),
                Box::new(ErasedExpr::Lit(1)),
            )),
            Box::new(ErasedExpr::Lit(2)),
        );
        let flat = ErasedFlatApp::from_expr(expr);
        assert_eq!(flat.arity(), 2);
        let rebuilt = flat.into_expr();
        assert!(matches!(rebuilt, ErasedExpr::App(_, _)));
    }
    #[test]
    fn test_subst_map() {
        let mut m = ErasedSubstMap::new();
        m.insert(0, ErasedExpr::Lit(5));
        assert!(m.get(0).is_some());
        assert!(m.get(1).is_none());
        assert_eq!(m.len(), 1);
    }
    #[test]
    fn test_erased_ast_size() {
        let expr = ErasedExpr::App(
            Box::new(ErasedExpr::Const("g".into())),
            Box::new(ErasedExpr::Lit(7)),
        );
        let ast = ErasedAst::new(expr, "test");
        assert!(ast.size() >= 1);
    }
}
