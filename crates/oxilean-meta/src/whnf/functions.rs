//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    HeadForm, MetaWhnf, WhnfAnalysisPass, WhnfAnalysisResult, WhnfConfig, WhnfConfigStore,
    WhnfConfigValue, WhnfDiagnostics, WhnfDiff, WhnfExtConfig1700, WhnfExtConfigVal1700,
    WhnfExtDiag1700, WhnfExtDiff1700, WhnfExtPass1700, WhnfExtPipeline1700, WhnfExtResult1700,
    WhnfMetaBuilder, WhnfMetaCounterMap, WhnfMetaExtMap, WhnfMetaExtUtil, WhnfMetaStateMachine,
    WhnfMetaWindow, WhnfMetaWorkQueue, WhnfPipeline, WhnfResult, WhnfStats,
};
use crate::basic::{MVarId, MetaContext};
use oxilean_kernel::{
    reduce::TransparencyMode, ConstantInfo, Environment, Expr, Level, Name, Reducer,
};

/// Collect the head and arguments of a nested application.
pub(super) fn collect_app(expr: &Expr) -> (&Expr, Vec<Expr>) {
    let mut args = Vec::new();
    let mut e = expr;
    while let Expr::App(f, a) = e {
        args.push(a.as_ref().clone());
        e = f;
    }
    args.reverse();
    (e, args)
}
/// Rebuild a nested application from head and arguments.
pub(super) fn rebuild_app(head: &Expr, args: &[Expr]) -> Expr {
    let mut result = head.clone();
    for arg in args {
        result = Expr::App(Box::new(result), Box::new(arg.clone()));
    }
    result
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::whnf::*;
    use oxilean_kernel::{BinderInfo, Literal};
    fn mk_env() -> Environment {
        Environment::new()
    }
    #[test]
    fn test_whnf_sort() {
        let mut whnf = MetaWhnf::new();
        let ctx = MetaContext::new(mk_env());
        let expr = Expr::Sort(Level::zero());
        let result = whnf.whnf(&expr, &ctx);
        assert!(!result.is_stuck());
        assert_eq!(result.expr(), &expr);
    }
    #[test]
    fn test_whnf_lit() {
        let mut whnf = MetaWhnf::new();
        let ctx = MetaContext::new(mk_env());
        let expr = Expr::Lit(Literal::Nat(42));
        let result = whnf.whnf(&expr, &ctx);
        assert_eq!(result.expr(), &expr);
    }
    #[test]
    fn test_whnf_beta() {
        let mut whnf = MetaWhnf::new();
        let ctx = MetaContext::new(mk_env());
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(Expr::BVar(0)),
        );
        let arg = Expr::Lit(Literal::Nat(42));
        let app = Expr::App(Box::new(lam), Box::new(arg.clone()));
        let result = whnf.whnf(&app, &ctx);
        assert_eq!(result.expr(), &arg);
    }
    #[test]
    fn test_whnf_mvar_stuck() {
        let mut whnf = MetaWhnf::new();
        let mut ctx = MetaContext::new(mk_env());
        let ty = Expr::Sort(Level::zero());
        let (_id, placeholder) = ctx.mk_fresh_expr_mvar(ty, crate::MetavarKind::Natural);
        let result = whnf.whnf(&placeholder, &ctx);
        assert!(result.is_stuck());
    }
    #[test]
    fn test_whnf_mvar_assigned() {
        let mut whnf = MetaWhnf::new();
        let mut ctx = MetaContext::new(mk_env());
        let ty = Expr::Sort(Level::zero());
        let (id, placeholder) = ctx.mk_fresh_expr_mvar(ty, crate::MetavarKind::Natural);
        let val = Expr::Lit(Literal::Nat(42));
        ctx.assign_mvar(id, val.clone());
        let result = whnf.whnf(&placeholder, &ctx);
        assert!(!result.is_stuck());
        assert_eq!(result.expr(), &val);
    }
    #[test]
    fn test_whnf_let() {
        let mut whnf = MetaWhnf::new();
        let ctx = MetaContext::new(mk_env());
        let expr = Expr::Let(
            Name::str("x"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(Expr::Lit(Literal::Nat(42))),
            Box::new(Expr::BVar(0)),
        );
        let result = whnf.whnf(&expr, &ctx);
        assert_eq!(result.expr(), &Expr::Lit(Literal::Nat(42)));
    }
    #[test]
    fn test_whnf_transparency() {
        let whnf = MetaWhnf::with_transparency(TransparencyMode::None);
        assert_eq!(whnf.transparency(), TransparencyMode::None);
    }
    #[test]
    fn test_collect_app() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::Lit(Literal::Nat(1));
        let b = Expr::Lit(Literal::Nat(2));
        let app = Expr::App(
            Box::new(Expr::App(Box::new(f.clone()), Box::new(a.clone()))),
            Box::new(b.clone()),
        );
        let (head, args) = collect_app(&app);
        assert_eq!(head, &f);
        assert_eq!(args.len(), 2);
        assert_eq!(args[0], a);
        assert_eq!(args[1], b);
    }
    #[test]
    fn test_rebuild_app() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::Lit(Literal::Nat(1));
        let b = Expr::Lit(Literal::Nat(2));
        let rebuilt = rebuild_app(&f, &[a.clone(), b.clone()]);
        let expected = Expr::App(Box::new(Expr::App(Box::new(f), Box::new(a))), Box::new(b));
        assert_eq!(rebuilt, expected);
    }
    #[test]
    fn test_at_transparency() {
        let mut whnf = MetaWhnf::new();
        assert_eq!(whnf.transparency(), TransparencyMode::Default);
        whnf.at_transparency(TransparencyMode::All, |w| {
            assert_eq!(w.transparency(), TransparencyMode::All);
        });
        assert_eq!(whnf.transparency(), TransparencyMode::Default);
    }
    #[test]
    fn test_whnf_cache_clear() {
        let mut whnf = MetaWhnf::new();
        let ctx = MetaContext::new(mk_env());
        let expr = Expr::Lit(Literal::Nat(42));
        let _ = whnf.whnf(&expr, &ctx);
        assert!(!whnf.cache.is_empty());
        whnf.clear_cache();
        assert!(whnf.cache.is_empty());
    }
}
/// Classify the head form of an expression.
///
/// Useful for pattern matching in tactics without needing the full WHNF engine.
pub fn classify_head_form(expr: &Expr) -> HeadForm {
    match expr {
        Expr::Sort(_) => HeadForm::Sort,
        Expr::Lam(_, _, _, _) => HeadForm::Lam,
        Expr::Pi(_, _, _, _) => HeadForm::Pi,
        Expr::Lit(_) => HeadForm::Lit,
        Expr::FVar(_) => HeadForm::FVar,
        Expr::App(_, _) => {
            let mut e = expr;
            while let Expr::App(f, _) = e {
                e = f;
            }
            match e {
                Expr::Const(n, _) => HeadForm::App(n.clone()),
                _ => HeadForm::AppNonConst,
            }
        }
        _ => HeadForm::Neutral,
    }
}
/// Repeatedly compute WHNF until a fixed point is reached or a limit is hit.
///
/// Used for debugging and testing that the WHNF engine is idempotent.
pub fn iterate_whnf(
    whnf: &mut MetaWhnf,
    expr: &Expr,
    ctx: &crate::basic::MetaContext,
    max_steps: usize,
) -> (WhnfResult, usize) {
    let mut current = expr.clone();
    let mut steps = 0;
    for _ in 0..max_steps {
        let result = whnf.whnf(&current, ctx);
        steps += 1;
        if result.expr() == &current || result.is_stuck() {
            return (result, steps);
        }
        current = result.expr().clone();
    }
    (WhnfResult::Reduced(current), steps)
}
/// Compare two expressions by reducing both to WHNF and checking structural equality.
///
/// This is a cheap approximation of definitional equality that does not
/// invoke the full unification procedure.
pub fn whnf_eq(whnf: &mut MetaWhnf, e1: &Expr, e2: &Expr, ctx: &crate::basic::MetaContext) -> bool {
    if e1 == e2 {
        return true;
    }
    let r1 = whnf.whnf(e1, ctx);
    let r2 = whnf.whnf(e2, ctx);
    r1.expr() == r2.expr()
}
/// Get the name of a transparency mode for display.
pub fn transparency_name(mode: oxilean_kernel::reduce::TransparencyMode) -> &'static str {
    use oxilean_kernel::reduce::TransparencyMode;
    match mode {
        TransparencyMode::None => "none",
        TransparencyMode::Reducible => "reducible",
        TransparencyMode::Instances => "instances",
        TransparencyMode::Default => "default",
        TransparencyMode::All => "all",
    }
}
/// Check if one transparency mode is stricter than another.
///
/// A mode is stricter if it unfolds fewer definitions.
pub fn is_stricter(
    a: oxilean_kernel::reduce::TransparencyMode,
    b: oxilean_kernel::reduce::TransparencyMode,
) -> bool {
    let rank = |m: TransparencyMode| -> u8 {
        match m {
            TransparencyMode::None => 0,
            TransparencyMode::Reducible => 1,
            TransparencyMode::Instances => 2,
            TransparencyMode::Default => 3,
            TransparencyMode::All => 4,
        }
    };
    rank(a) < rank(b)
}
/// Decompose a WHNF expression into its head function and argument list.
///
/// For non-application expressions the argument list will be empty.
pub fn decompose_whnf(expr: &Expr) -> (&Expr, Vec<&Expr>) {
    let mut args = Vec::new();
    let mut e = expr;
    while let Expr::App(f, a) = e {
        args.push(a.as_ref());
        e = f;
    }
    args.reverse();
    (e, args)
}
/// Check if a WHNF expression is a specific constant applied to some arguments.
pub fn is_whnf_app_of(expr: &Expr, name: &oxilean_kernel::Name) -> bool {
    let (head, _) = decompose_whnf(expr);
    matches!(head, Expr::Const(n, _) if n == name)
}
/// Get the number of arguments in a WHNF application.
pub fn whnf_app_arity(expr: &Expr) -> usize {
    let mut count = 0;
    let mut e = expr;
    while let Expr::App(f, _) = e {
        count += 1;
        e = f;
    }
    count
}
/// Reduce a list of expressions to WHNF in bulk.
///
/// Returns a vector of `WhnfResult`s in the same order as the input.
pub fn batch_whnf(
    whnf: &mut MetaWhnf,
    exprs: &[Expr],
    ctx: &crate::basic::MetaContext,
) -> Vec<WhnfResult> {
    exprs.iter().map(|e| whnf.whnf(e, ctx)).collect()
}
/// Check if any expression in a batch is stuck on a metavariable.
pub fn any_stuck(results: &[WhnfResult]) -> bool {
    results.iter().any(|r| r.is_stuck())
}
/// Get all metavar IDs that results are stuck on.
pub fn stuck_mvars(results: &[WhnfResult]) -> Vec<MVarId> {
    results
        .iter()
        .filter_map(|r| match r {
            WhnfResult::Stuck(_, id) => Some(*id),
            _ => None,
        })
        .collect()
}
#[cfg(test)]
mod whnf_extra_tests {
    use super::*;
    use crate::whnf::*;
    use oxilean_kernel::{BinderInfo, Literal};
    fn mk_env() -> oxilean_kernel::Environment {
        oxilean_kernel::Environment::new()
    }
    #[test]
    fn test_classify_head_form_sort() {
        let e = Expr::Sort(Level::zero());
        assert_eq!(classify_head_form(&e), HeadForm::Sort);
    }
    #[test]
    fn test_classify_head_form_lam() {
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(Expr::BVar(0)),
        );
        assert_eq!(classify_head_form(&lam), HeadForm::Lam);
    }
    #[test]
    fn test_classify_head_form_pi() {
        let pi = Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(Expr::Sort(Level::zero())),
        );
        assert_eq!(classify_head_form(&pi), HeadForm::Pi);
    }
    #[test]
    fn test_classify_head_form_app_const() {
        let app = Expr::App(
            Box::new(Expr::Const(Name::str("f"), vec![])),
            Box::new(Expr::Lit(Literal::Nat(0))),
        );
        assert_eq!(classify_head_form(&app), HeadForm::App(Name::str("f")));
    }
    #[test]
    fn test_decompose_whnf() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::Lit(Literal::Nat(1));
        let b = Expr::Lit(Literal::Nat(2));
        let app = Expr::App(
            Box::new(Expr::App(Box::new(f.clone()), Box::new(a.clone()))),
            Box::new(b.clone()),
        );
        let (head, args) = decompose_whnf(&app);
        assert_eq!(head, &f);
        assert_eq!(args.len(), 2);
    }
    #[test]
    fn test_is_whnf_app_of() {
        let name = Name::str("Nat.succ");
        let app = Expr::App(
            Box::new(Expr::Const(name.clone(), vec![])),
            Box::new(Expr::Lit(Literal::Nat(0))),
        );
        assert!(is_whnf_app_of(&app, &name));
        assert!(!is_whnf_app_of(&app, &Name::str("other")));
    }
    #[test]
    fn test_whnf_app_arity() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::Lit(Literal::Nat(0));
        let b = Expr::Lit(Literal::Nat(1));
        let app = Expr::App(
            Box::new(Expr::App(Box::new(f.clone()), Box::new(a))),
            Box::new(b),
        );
        assert_eq!(whnf_app_arity(&app), 2);
        assert_eq!(whnf_app_arity(&f), 0);
    }
    #[test]
    fn test_batch_whnf() {
        let mut whnf = MetaWhnf::new();
        let ctx = crate::basic::MetaContext::new(mk_env());
        let exprs = vec![Expr::Sort(Level::zero()), Expr::Lit(Literal::Nat(1))];
        let results = batch_whnf(&mut whnf, &exprs, &ctx);
        assert_eq!(results.len(), 2);
        assert!(!any_stuck(&results));
    }
    #[test]
    fn test_any_stuck_false() {
        let results = vec![
            WhnfResult::Reduced(Expr::Lit(Literal::Nat(0))),
            WhnfResult::Reduced(Expr::Lit(Literal::Nat(1))),
        ];
        assert!(!any_stuck(&results));
    }
    #[test]
    fn test_any_stuck_true() {
        let id = MVarId::new(1);
        let results = vec![
            WhnfResult::Reduced(Expr::Lit(Literal::Nat(0))),
            WhnfResult::Stuck(Expr::Lit(Literal::Nat(1)), id),
        ];
        assert!(any_stuck(&results));
    }
    #[test]
    fn test_stuck_mvars() {
        let id1 = MVarId::new(1);
        let id2 = MVarId::new(2);
        let results = vec![
            WhnfResult::Stuck(Expr::Lit(Literal::Nat(0)), id1),
            WhnfResult::Reduced(Expr::Lit(Literal::Nat(1))),
            WhnfResult::Stuck(Expr::Lit(Literal::Nat(2)), id2),
        ];
        let mvars = stuck_mvars(&results);
        assert_eq!(mvars, vec![id1, id2]);
    }
    #[test]
    fn test_transparency_name() {
        assert_eq!(transparency_name(TransparencyMode::None), "none");
        assert_eq!(transparency_name(TransparencyMode::All), "all");
        assert_eq!(transparency_name(TransparencyMode::Default), "default");
    }
    #[test]
    fn test_is_stricter() {
        assert!(is_stricter(TransparencyMode::None, TransparencyMode::All));
        assert!(is_stricter(
            TransparencyMode::Reducible,
            TransparencyMode::Default
        ));
        assert!(!is_stricter(TransparencyMode::All, TransparencyMode::None));
        assert!(!is_stricter(
            TransparencyMode::Default,
            TransparencyMode::Default
        ));
    }
    #[test]
    fn test_whnf_eq_same() {
        let mut whnf = MetaWhnf::new();
        let ctx = crate::basic::MetaContext::new(mk_env());
        let e = Expr::Lit(Literal::Nat(42));
        assert!(whnf_eq(&mut whnf, &e, &e, &ctx));
    }
    #[test]
    fn test_whnf_eq_beta() {
        let mut whnf = MetaWhnf::new();
        let ctx = crate::basic::MetaContext::new(mk_env());
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(Expr::BVar(0)),
        );
        let arg = Expr::Lit(Literal::Nat(42));
        let app = Expr::App(Box::new(lam), Box::new(arg.clone()));
        assert!(whnf_eq(&mut whnf, &app, &arg, &ctx));
    }
    #[test]
    fn test_iterate_whnf_already_normal() {
        let mut whnf = MetaWhnf::new();
        let ctx = crate::basic::MetaContext::new(mk_env());
        let e = Expr::Lit(Literal::Nat(0));
        let (result, steps) = iterate_whnf(&mut whnf, &e, &ctx, 10);
        assert!(!result.is_stuck());
        assert_eq!(steps, 1);
    }
}
/// Check if an expression is already in WHNF (no further head reductions possible).
pub fn is_whnf(expr: &Expr) -> bool {
    match expr {
        Expr::Sort(_)
        | Expr::Const(..)
        | Expr::Lam(..)
        | Expr::Pi(..)
        | Expr::Lit(_)
        | Expr::BVar(_)
        | Expr::FVar(_) => true,
        Expr::App(f, _) => is_whnf_app_head(f),
        Expr::Let(..) => false,
        Expr::Proj(..) => false,
    }
}
pub(super) fn is_whnf_app_head(e: &Expr) -> bool {
    match e {
        Expr::Lam(..) => false,
        Expr::App(f, _) => is_whnf_app_head(f),
        _ => true,
    }
}
#[cfg(test)]
mod whnf_extended_tests {
    use super::*;
    use crate::whnf::*;
    use oxilean_kernel::{Expr, Level, Literal};
    #[test]
    fn test_whnf_config_default() {
        let cfg = WhnfConfig::default();
        assert!(cfg.unfold_let);
        assert!(cfg.reduce_iota);
        assert_eq!(cfg.max_beta_steps, 1024);
    }
    #[test]
    fn test_whnf_config_conservative() {
        let cfg = WhnfConfig::conservative();
        assert!(!cfg.reduce_iota);
        assert_eq!(cfg.max_beta_steps, 128);
    }
    #[test]
    fn test_whnf_config_aggressive() {
        let cfg = WhnfConfig::aggressive();
        assert_eq!(cfg.max_beta_steps, 16384);
    }
    #[test]
    fn test_whnf_stats_record() {
        let mut s = WhnfStats::new();
        s.record_reduction();
        s.record_reduction();
        s.record_cache_hit();
        s.record_stuck();
        assert_eq!(s.reductions, 2);
        assert_eq!(s.cache_hits, 1);
        assert_eq!(s.stuck_count, 1);
    }
    #[test]
    fn test_whnf_stats_cache_hit_rate() {
        let mut s = WhnfStats::new();
        s.record_reduction();
        s.record_cache_hit();
        let rate = s.cache_hit_rate();
        assert!((rate - 0.5).abs() < 1e-9);
    }
    #[test]
    fn test_whnf_stats_display() {
        let s = WhnfStats {
            reductions: 5,
            cache_hits: 3,
            stuck_count: 1,
        };
        let txt = format!("{}", s);
        assert!(txt.contains("reductions: 5"));
    }
    #[test]
    fn test_is_whnf_sort() {
        assert!(is_whnf(&Expr::Sort(Level::zero())));
    }
    #[test]
    fn test_is_whnf_lit() {
        assert!(is_whnf(&Expr::Lit(Literal::Nat(42))));
    }
    #[test]
    fn test_is_whnf_app_of_const() {
        let e = Expr::App(
            Box::new(Expr::Const(oxilean_kernel::Name::str("f"), vec![])),
            Box::new(Expr::BVar(0)),
        );
        assert!(is_whnf(&e));
    }
    #[test]
    fn test_is_whnf_lam_head_app_not_whnf() {
        let lam = Expr::Lam(
            oxilean_kernel::BinderInfo::Default,
            oxilean_kernel::Name::str("x"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(Expr::BVar(0)),
        );
        let app = Expr::App(Box::new(lam), Box::new(Expr::BVar(0)));
        assert!(!is_whnf(&app));
    }
}
#[cfg(test)]
mod whnfmeta_ext2_tests {
    use super::*;
    use crate::whnf::*;
    #[test]
    fn test_whnfmeta_ext_util_basic() {
        let mut u = WhnfMetaExtUtil::new("test");
        u.push(10);
        u.push(20);
        assert_eq!(u.sum(), 30);
        assert_eq!(u.len(), 2);
    }
    #[test]
    fn test_whnfmeta_ext_util_min_max() {
        let mut u = WhnfMetaExtUtil::new("mm");
        u.push(5);
        u.push(1);
        u.push(9);
        assert_eq!(u.min_val(), Some(1));
        assert_eq!(u.max_val(), Some(9));
    }
    #[test]
    fn test_whnfmeta_ext_util_flags() {
        let mut u = WhnfMetaExtUtil::new("flags");
        u.set_flag(3);
        assert!(u.has_flag(3));
        assert!(!u.has_flag(2));
    }
    #[test]
    fn test_whnfmeta_ext_util_pop() {
        let mut u = WhnfMetaExtUtil::new("pop");
        u.push(42);
        assert_eq!(u.pop(), Some(42));
        assert!(u.is_empty());
    }
    #[test]
    fn test_whnfmeta_ext_map_basic() {
        let mut m: WhnfMetaExtMap<i32> = WhnfMetaExtMap::new();
        m.insert("key", 42);
        assert_eq!(m.get("key"), Some(&42));
        assert!(m.contains("key"));
        assert!(!m.contains("other"));
    }
    #[test]
    fn test_whnfmeta_ext_map_get_or_default() {
        let mut m: WhnfMetaExtMap<i32> = WhnfMetaExtMap::new();
        m.insert("k", 5);
        assert_eq!(m.get_or_default("k"), 5);
        assert_eq!(m.get_or_default("missing"), 0);
    }
    #[test]
    fn test_whnfmeta_ext_map_keys_sorted() {
        let mut m: WhnfMetaExtMap<i32> = WhnfMetaExtMap::new();
        m.insert("z", 1);
        m.insert("a", 2);
        m.insert("m", 3);
        let keys = m.keys_sorted();
        assert_eq!(keys[0].as_str(), "a");
        assert_eq!(keys[2].as_str(), "z");
    }
    #[test]
    fn test_whnfmeta_window_mean() {
        let mut w = WhnfMetaWindow::new(3);
        w.push(1.0);
        w.push(2.0);
        w.push(3.0);
        assert!((w.mean() - 2.0).abs() < 1e-10);
    }
    #[test]
    fn test_whnfmeta_window_evict() {
        let mut w = WhnfMetaWindow::new(2);
        w.push(10.0);
        w.push(20.0);
        w.push(30.0);
        assert_eq!(w.len(), 2);
        assert!((w.mean() - 25.0).abs() < 1e-10);
    }
    #[test]
    fn test_whnfmeta_window_std_dev() {
        let mut w = WhnfMetaWindow::new(10);
        for i in 0..10 {
            w.push(i as f64);
        }
        assert!(w.std_dev() > 0.0);
    }
    #[test]
    fn test_whnfmeta_builder_basic() {
        let b = WhnfMetaBuilder::new("test")
            .add_item("a")
            .add_item("b")
            .set_config("key", "val");
        assert_eq!(b.item_count(), 2);
        assert!(b.has_config("key"));
        assert_eq!(b.get_config("key"), Some("val"));
    }
    #[test]
    fn test_whnfmeta_builder_summary() {
        let b = WhnfMetaBuilder::new("suite").add_item("x");
        let s = b.build_summary();
        assert!(s.contains("suite"));
    }
    #[test]
    fn test_whnfmeta_state_machine_start() {
        let mut sm = WhnfMetaStateMachine::new();
        assert!(sm.start());
        assert!(sm.state.is_running());
    }
    #[test]
    fn test_whnfmeta_state_machine_complete() {
        let mut sm = WhnfMetaStateMachine::new();
        sm.start();
        sm.complete();
        assert!(sm.state.is_terminal());
    }
    #[test]
    fn test_whnfmeta_state_machine_fail() {
        let mut sm = WhnfMetaStateMachine::new();
        sm.fail("oops");
        assert!(sm.state.is_terminal());
        assert_eq!(sm.state.error_msg(), Some("oops"));
    }
    #[test]
    fn test_whnfmeta_state_machine_no_transition_after_terminal() {
        let mut sm = WhnfMetaStateMachine::new();
        sm.complete();
        assert!(!sm.start());
    }
    #[test]
    fn test_whnfmeta_work_queue_basic() {
        let mut wq = WhnfMetaWorkQueue::new(10);
        wq.enqueue("task1".to_string());
        wq.enqueue("task2".to_string());
        assert_eq!(wq.pending_count(), 2);
        let t = wq.dequeue();
        assert_eq!(t, Some("task1".to_string()));
        assert_eq!(wq.processed_count(), 1);
    }
    #[test]
    fn test_whnfmeta_work_queue_capacity() {
        let mut wq = WhnfMetaWorkQueue::new(2);
        wq.enqueue("a".to_string());
        wq.enqueue("b".to_string());
        assert!(wq.is_full());
        assert!(!wq.enqueue("c".to_string()));
    }
    #[test]
    fn test_whnfmeta_counter_map_basic() {
        let mut cm = WhnfMetaCounterMap::new();
        cm.increment("apple");
        cm.increment("apple");
        cm.increment("banana");
        assert_eq!(cm.count("apple"), 2);
        assert_eq!(cm.count("banana"), 1);
        assert_eq!(cm.num_unique(), 2);
    }
    #[test]
    fn test_whnfmeta_counter_map_frequency() {
        let mut cm = WhnfMetaCounterMap::new();
        cm.increment("a");
        cm.increment("a");
        cm.increment("b");
        assert!((cm.frequency("a") - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_whnfmeta_counter_map_most_common() {
        let mut cm = WhnfMetaCounterMap::new();
        cm.increment("x");
        cm.increment("y");
        cm.increment("x");
        let (k, v) = cm.most_common().expect("most_common should succeed");
        assert_eq!(k.as_str(), "x");
        assert_eq!(v, 2);
    }
}
#[cfg(test)]
mod whnf_analysis_tests {
    use super::*;
    use crate::whnf::*;
    #[test]
    fn test_whnf_result_ok() {
        let r = WhnfAnalysisResult::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_whnf_result_err() {
        let r = WhnfAnalysisResult::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_whnf_result_partial() {
        let r = WhnfAnalysisResult::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_whnf_result_skipped() {
        let r = WhnfAnalysisResult::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_whnf_analysis_pass_run() {
        let mut p = WhnfAnalysisPass::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_whnf_analysis_pass_empty_input() {
        let mut p = WhnfAnalysisPass::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_whnf_analysis_pass_success_rate() {
        let mut p = WhnfAnalysisPass::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_whnf_analysis_pass_disable() {
        let mut p = WhnfAnalysisPass::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_whnf_pipeline_basic() {
        let mut pipeline = WhnfPipeline::new("main_pipeline");
        pipeline.add_pass(WhnfAnalysisPass::new("pass1"));
        pipeline.add_pass(WhnfAnalysisPass::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_whnf_pipeline_disabled_pass() {
        let mut pipeline = WhnfPipeline::new("partial");
        let mut p = WhnfAnalysisPass::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(WhnfAnalysisPass::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_whnf_diff_basic() {
        let mut d = WhnfDiff::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_whnf_diff_summary() {
        let mut d = WhnfDiff::new();
        d.add("x");
        d.add("y");
        d.remove("z");
        let s = d.summary();
        assert!(s.contains("+2"));
    }
    #[test]
    fn test_whnf_config_set_get() {
        let mut cfg = WhnfConfigStore::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_whnf_config_read_only() {
        let mut cfg = WhnfConfigStore::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_whnf_config_remove() {
        let mut cfg = WhnfConfigStore::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_whnf_diagnostics_basic() {
        let mut diag = WhnfDiagnostics::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_whnf_diagnostics_max_errors() {
        let mut diag = WhnfDiagnostics::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_whnf_diagnostics_clear() {
        let mut diag = WhnfDiagnostics::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_whnf_config_value_types() {
        let b = WhnfConfigValue::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = WhnfConfigValue::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = WhnfConfigValue::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = WhnfConfigValue::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = WhnfConfigValue::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
#[cfg(test)]
mod whnf_ext_tests_1700 {
    use super::*;
    use crate::whnf::*;
    #[test]
    fn test_whnf_ext_result_ok_1700() {
        let r = WhnfExtResult1700::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_whnf_ext_result_err_1700() {
        let r = WhnfExtResult1700::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_whnf_ext_result_partial_1700() {
        let r = WhnfExtResult1700::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_whnf_ext_result_skipped_1700() {
        let r = WhnfExtResult1700::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_whnf_ext_pass_run_1700() {
        let mut p = WhnfExtPass1700::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_whnf_ext_pass_empty_1700() {
        let mut p = WhnfExtPass1700::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_whnf_ext_pass_rate_1700() {
        let mut p = WhnfExtPass1700::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_whnf_ext_pass_disable_1700() {
        let mut p = WhnfExtPass1700::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_whnf_ext_pipeline_basic_1700() {
        let mut pipeline = WhnfExtPipeline1700::new("main_pipeline");
        pipeline.add_pass(WhnfExtPass1700::new("pass1"));
        pipeline.add_pass(WhnfExtPass1700::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_whnf_ext_pipeline_disabled_1700() {
        let mut pipeline = WhnfExtPipeline1700::new("partial");
        let mut p = WhnfExtPass1700::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(WhnfExtPass1700::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_whnf_ext_diff_basic_1700() {
        let mut d = WhnfExtDiff1700::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_whnf_ext_config_set_get_1700() {
        let mut cfg = WhnfExtConfig1700::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_whnf_ext_config_read_only_1700() {
        let mut cfg = WhnfExtConfig1700::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_whnf_ext_config_remove_1700() {
        let mut cfg = WhnfExtConfig1700::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_whnf_ext_diagnostics_basic_1700() {
        let mut diag = WhnfExtDiag1700::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_whnf_ext_diagnostics_max_errors_1700() {
        let mut diag = WhnfExtDiag1700::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_whnf_ext_diagnostics_clear_1700() {
        let mut diag = WhnfExtDiag1700::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_whnf_ext_config_value_types_1700() {
        let b = WhnfExtConfigVal1700::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = WhnfExtConfigVal1700::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = WhnfExtConfigVal1700::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = WhnfExtConfigVal1700::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = WhnfExtConfigVal1700::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
