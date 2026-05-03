//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::{BinderInfo, Expr, FVarId, Name};
use std::collections::HashMap;

use super::types::{
    ConfigNode, Context, ContextChain, ContextDiff, ContextEntry, ContextStats, DecisionNode,
    Either2, FlatSubstitution, FocusStack, FreshNameSeq, HypContext, LabelSet, NameGenerator,
    NonEmptyVec, PathBuf, RewriteRule, RewriteRuleSet, ScopedContext, SimpleDag, SlidingSum,
    SmallMap, SparseVec, StackCalc, StatSummary, Stopwatch, StringPool, TokenBucket, TransformStat,
    TransitiveClosure, VersionedRecord, WindowIterator, WriteOnce,
};

/// Abstract a single free variable in an expression, replacing it with BVar(0).
pub(super) fn abstract_fvar(expr: Expr, fvar: FVarId) -> Expr {
    abstract_fvar_at(expr, fvar, 0)
}
/// Abstract a free variable, replacing with BVar(depth).
pub(super) fn abstract_fvar_at(expr: Expr, fvar: FVarId, depth: u32) -> Expr {
    match expr {
        Expr::FVar(id) if id == fvar => Expr::BVar(depth),
        Expr::App(f, a) => Expr::App(
            Box::new(abstract_fvar_at(*f, fvar, depth)),
            Box::new(abstract_fvar_at(*a, fvar, depth)),
        ),
        Expr::Lam(bi, n, ty, body) => Expr::Lam(
            bi,
            n,
            Box::new(abstract_fvar_at(*ty, fvar, depth)),
            Box::new(abstract_fvar_at(*body, fvar, depth + 1)),
        ),
        Expr::Pi(bi, n, ty, body) => Expr::Pi(
            bi,
            n,
            Box::new(abstract_fvar_at(*ty, fvar, depth)),
            Box::new(abstract_fvar_at(*body, fvar, depth + 1)),
        ),
        Expr::Let(n, ty, val, body) => Expr::Let(
            n,
            Box::new(abstract_fvar_at(*ty, fvar, depth)),
            Box::new(abstract_fvar_at(*val, fvar, depth)),
            Box::new(abstract_fvar_at(*body, fvar, depth + 1)),
        ),
        _ => expr,
    }
}
/// Helper: abstract earlier fvars in a type expression for telescope construction.
///
/// When building `λ x₁ x₂ x₃, body`, the type annotation τᵢ of each binder xᵢ
/// may reference the outer binders x₁…xᵢ₋₁.  This function replaces each such
/// free variable with the appropriate de Bruijn index so that the resulting
/// telescope is well-scoped.
///
/// Concretely, if `current_fvar` is at index `k` in `fvars`, then fvars[0..k]
/// are outer binders and must be replaced with BVar(k-1-i) for fvar at index i.
pub(super) fn abstract_fvars_in_type(ty: Expr, fvars: &[FVarId], current_fvar: FVarId) -> Expr {
    let current_idx = match fvars.iter().position(|&f| f == current_fvar) {
        Some(idx) => idx,
        None => return ty,
    };
    if current_idx == 0 {
        return ty;
    }
    let mut result = ty;
    for (i, &fvar) in fvars[..current_idx].iter().enumerate() {
        let depth = (current_idx - 1 - i) as u32;
        result = abstract_fvar_at(result, fvar, depth);
    }
    result
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Level, Literal};
    #[test]
    fn test_context_create() {
        let ctx = Context::new();
        assert_eq!(ctx.num_locals(), 0);
        assert!(ctx.is_empty());
    }
    #[test]
    fn test_push_local() {
        let mut ctx = Context::new();
        let ty = Expr::Sort(Level::zero());
        let fvar = ctx.push_local(Name::str("x"), ty, None);
        assert_eq!(ctx.num_locals(), 1);
        assert!(!ctx.is_empty());
        let local = ctx.get_local(fvar).expect("local should be present");
        assert_eq!(local.name, Name::str("x"));
    }
    #[test]
    fn test_pop_local() {
        let mut ctx = Context::new();
        let ty = Expr::Sort(Level::zero());
        ctx.push_local(Name::str("x"), ty.clone(), None);
        ctx.push_local(Name::str("y"), ty, None);
        assert_eq!(ctx.num_locals(), 2);
        let popped = ctx.pop_local().expect("popped should be present");
        assert_eq!(popped.name, Name::str("y"));
        assert_eq!(ctx.num_locals(), 1);
    }
    #[test]
    fn test_find_local() {
        let mut ctx = Context::new();
        let ty = Expr::Sort(Level::zero());
        ctx.push_local(Name::str("x"), ty.clone(), None);
        ctx.push_local(Name::str("y"), ty, None);
        let local = ctx
            .find_local(&Name::str("x"))
            .expect("local should be present");
        assert_eq!(local.name, Name::str("x"));
    }
    #[test]
    fn test_find_local_not_found() {
        let ctx = Context::new();
        assert!(ctx.find_local(&Name::str("z")).is_none());
    }
    #[test]
    fn test_local_with_value() {
        let mut ctx = Context::new();
        let ty = Expr::Sort(Level::zero());
        let val = Expr::Lit(Literal::Nat(42));
        let fvar = ctx.push_local(Name::str("x"), ty, Some(val.clone()));
        let local = ctx.get_local(fvar).expect("local should be present");
        assert!(local.val.is_some());
        assert_eq!(local.val.as_ref().expect("as_ref should succeed"), &val);
    }
    #[test]
    fn test_clear() {
        let mut ctx = Context::new();
        let ty = Expr::Sort(Level::zero());
        ctx.push_local(Name::str("x"), ty, None);
        ctx.clear();
        assert_eq!(ctx.num_locals(), 0);
        assert!(ctx.is_empty());
    }
    #[test]
    fn test_mk_local_decl() {
        let mut ctx = Context::new();
        let ty = Expr::Sort(Level::zero());
        let fvar_expr = ctx.mk_local_decl(Name::str("x"), BinderInfo::Default, ty);
        assert!(matches!(fvar_expr, Expr::FVar(_)));
        assert_eq!(ctx.num_locals(), 1);
    }
    #[test]
    fn test_mk_let_decl() {
        let mut ctx = Context::new();
        let ty = Expr::Sort(Level::zero());
        let val = Expr::Lit(Literal::Nat(42));
        let fvar_expr = ctx.mk_let_decl(Name::str("x"), ty, val);
        if let Expr::FVar(id) = fvar_expr {
            assert!(ctx.is_let(id));
        } else {
            panic!("Expected FVar");
        }
    }
    #[test]
    fn test_save_restore() {
        let mut ctx = Context::new();
        let ty = Expr::Sort(Level::zero());
        ctx.push_local(Name::str("x"), ty.clone(), None);
        let snap = ctx.save();
        ctx.push_local(Name::str("y"), ty.clone(), None);
        ctx.push_local(Name::str("z"), ty, None);
        assert_eq!(ctx.num_locals(), 3);
        ctx.restore(&snap);
        assert_eq!(ctx.num_locals(), 1);
    }
    #[test]
    fn test_get_fvars() {
        let mut ctx = Context::new();
        let ty = Expr::Sort(Level::zero());
        ctx.push_local(Name::str("x"), ty.clone(), None);
        ctx.push_local(Name::str("y"), ty, None);
        let fvars = ctx.get_fvars();
        assert_eq!(fvars.len(), 2);
    }
    #[test]
    fn test_name_generator() {
        let mut gen = NameGenerator::new("x");
        let n1 = gen.next();
        let n2 = gen.next();
        assert_ne!(n1, n2);
        assert_eq!(n1, Name::str("x_0"));
        assert_eq!(n2, Name::str("x_1"));
    }
    #[test]
    fn test_with_local() {
        let mut ctx = Context::new();
        let ty = Expr::Sort(Level::zero());
        let fvar_inside = ctx.with_local(Name::str("temp"), ty, |ctx, fvar| {
            assert_eq!(ctx.num_locals(), 1);
            assert!(ctx.get_local(fvar).is_some());
            fvar
        });
        assert_eq!(ctx.num_locals(), 0);
        assert!(ctx.get_local(fvar_inside).is_none());
    }
    #[test]
    fn test_binder_info_preserved() {
        let mut ctx = Context::new();
        let ty = Expr::Sort(Level::zero());
        let fvar = ctx.push_local_with_binder(Name::str("x"), BinderInfo::Implicit, ty, None);
        let local = ctx.get_local(fvar).expect("local should be present");
        assert_eq!(local.binder_info, BinderInfo::Implicit);
    }
    #[test]
    fn test_abstract_fvar() {
        let fvar_id = FVarId(42);
        let expr = Expr::App(
            Box::new(Expr::Const(Name::str("f"), vec![])),
            Box::new(Expr::FVar(fvar_id)),
        );
        let abstracted = abstract_fvar(expr, fvar_id);
        let expected = Expr::App(
            Box::new(Expr::Const(Name::str("f"), vec![])),
            Box::new(Expr::BVar(0)),
        );
        assert_eq!(abstracted, expected);
    }
}
/// Fresh name generation utilities.
#[allow(dead_code)]
pub mod name_gen {
    use super::Name;
    /// Generate a fresh name based on a base and a counter.
    pub fn fresh(base: &str, counter: u64) -> Name {
        Name::str(format!("{}_{}", base, counter))
    }
    /// Generate a Greek letter name for type variables.
    pub fn greek(idx: usize) -> Name {
        const GREEK: &[&str] = &[
            "alpha", "beta", "gamma", "delta", "epsilon", "zeta", "eta", "theta", "iota", "kappa",
            "lambda", "mu", "nu", "xi", "pi", "rho", "sigma", "tau", "upsilon", "phi", "chi",
            "psi", "omega",
        ];
        if idx < GREEK.len() {
            Name::str(GREEK[idx])
        } else {
            Name::str(format!("alpha{}", idx))
        }
    }
    /// Generate a metavariable-style name.
    pub fn mvar(idx: u64) -> Name {
        Name::str(format!("?m{}", idx))
    }
    /// Generate a local hypothesis name.
    pub fn hyp(idx: usize) -> Name {
        Name::str(format!("h{}", idx))
    }
}
#[cfg(test)]
mod extended_ctx_tests {
    use super::*;
    use crate::{Level, Literal};
    #[test]
    fn test_context_entry_local() {
        let e = ContextEntry::local(Name::str("x"), Expr::Sort(Level::zero()));
        assert!(!e.is_let());
        assert!(!e.is_implicit());
    }
    #[test]
    fn test_context_entry_implicit() {
        let e = ContextEntry::implicit(Name::str("alpha"), Expr::Sort(Level::zero()));
        assert!(e.is_implicit());
        assert!(!e.is_let());
    }
    #[test]
    fn test_context_entry_let() {
        let e = ContextEntry::let_binding(
            Name::str("x"),
            Expr::Sort(Level::zero()),
            Expr::Lit(Literal::Nat(1)),
        );
        assert!(e.is_let());
    }
    #[test]
    fn test_context_chain_empty() {
        let chain = ContextChain::new();
        assert!(chain.is_empty());
        assert_eq!(chain.len(), 0);
    }
    #[test]
    fn test_context_chain_push_pop() {
        let mut chain = ContextChain::new();
        chain.push(ContextEntry::local(
            Name::str("x"),
            Expr::Sort(Level::zero()),
        ));
        chain.push(ContextEntry::implicit(
            Name::str("y"),
            Expr::Sort(Level::zero()),
        ));
        assert_eq!(chain.len(), 2);
        assert_eq!(chain.num_implicit(), 1);
        assert_eq!(chain.num_lets(), 0);
        let popped = chain.pop().expect("collection should not be empty");
        assert_eq!(popped.name, Name::str("y"));
    }
    #[test]
    fn test_context_chain_find() {
        let mut chain = ContextChain::new();
        chain.push(ContextEntry::local(
            Name::str("a"),
            Expr::Sort(Level::zero()),
        ));
        chain.push(ContextEntry::local(
            Name::str("b"),
            Expr::Sort(Level::zero()),
        ));
        assert!(chain.find(&Name::str("a")).is_some());
        assert!(chain.find(&Name::str("c")).is_none());
    }
    #[test]
    fn test_context_chain_from_context() {
        let mut ctx = Context::new();
        ctx.push_local(Name::str("x"), Expr::Sort(Level::zero()), None);
        ctx.push_local(
            Name::str("y"),
            Expr::Sort(Level::zero()),
            Some(Expr::Lit(Literal::Nat(0))),
        );
        let chain = ContextChain::from_context(&ctx);
        assert_eq!(chain.len(), 2);
        assert_eq!(chain.num_lets(), 1);
    }
    #[test]
    fn test_context_stats() {
        let mut ctx = Context::new();
        ctx.push_local(Name::str("x"), Expr::Sort(Level::zero()), None);
        ctx.push_local_with_binder(
            Name::str("y"),
            BinderInfo::Implicit,
            Expr::Sort(Level::zero()),
            None,
        );
        let stats = ContextStats::from_context(&ctx);
        assert_eq!(stats.num_locals, 2);
        assert_eq!(stats.num_implicit, 1);
        assert_eq!(stats.num_lets, 0);
    }
    #[test]
    fn test_context_diff_compute() {
        let mut old_ctx = Context::new();
        old_ctx.push_local(Name::str("x"), Expr::Sort(Level::zero()), None);
        let mut new_ctx = Context::new();
        new_ctx.push_local(Name::str("x"), Expr::Sort(Level::zero()), None);
        new_ctx.push_local(Name::str("y"), Expr::Sort(Level::zero()), None);
        let diff = ContextDiff::compute(&old_ctx, &new_ctx);
        assert!(diff.added.contains(&Name::str("y")));
        assert!(diff.removed.is_empty());
        assert!(!diff.is_empty());
    }
    #[test]
    fn test_context_diff_empty() {
        let ctx = Context::new();
        let diff = ContextDiff::compute(&ctx, &ctx);
        assert!(diff.is_empty());
    }
    #[test]
    fn test_name_gen_fresh() {
        let n = name_gen::fresh("x", 5);
        assert_eq!(n, Name::str("x_5"));
    }
    #[test]
    fn test_name_gen_greek() {
        let alpha = name_gen::greek(0);
        let beta = name_gen::greek(1);
        assert_ne!(alpha, beta);
        let overflow = name_gen::greek(100);
        assert_eq!(overflow, Name::str("alpha100"));
    }
    #[test]
    fn test_name_gen_mvar() {
        let m = name_gen::mvar(3);
        assert_eq!(m, Name::str("?m3"));
    }
    #[test]
    fn test_name_gen_hyp() {
        let h = name_gen::hyp(0);
        assert_eq!(h, Name::str("h0"));
    }
    #[test]
    fn test_context_with_local_cleanup() {
        let mut ctx = Context::new();
        let ty = Expr::Sort(Level::zero());
        let saw_local = ctx.with_local(Name::str("temp"), ty, |ctx, _fvar| {
            assert_eq!(ctx.num_locals(), 1);
            true
        });
        assert!(saw_local);
        assert_eq!(ctx.num_locals(), 0);
    }
    #[test]
    fn test_context_mk_lambda() {
        let mut ctx = Context::new();
        let ty = Expr::Sort(Level::zero());
        let fvar = ctx.push_local(Name::str("x"), ty, None);
        let body = Expr::FVar(fvar);
        let lam = ctx.mk_lambda(&[fvar], body);
        assert!(matches!(lam, Expr::Lam(_, _, _, _)));
    }
    #[test]
    fn test_context_mk_pi() {
        let mut ctx = Context::new();
        let ty = Expr::Sort(Level::zero());
        let fvar = ctx.push_local(Name::str("x"), ty, None);
        let body = Expr::FVar(fvar);
        let pi = ctx.mk_pi(&[fvar], body);
        assert!(matches!(pi, Expr::Pi(_, _, _, _)));
    }
}
/// Rename a free variable in an expression.
///
/// All occurrences of `Expr::FVar(old)` are replaced with `Expr::FVar(new)`.
#[allow(dead_code)]
pub fn rename_fvar(expr: Expr, old: FVarId, new: FVarId) -> Expr {
    match expr {
        Expr::FVar(id) if id == old => Expr::FVar(new),
        Expr::App(f, a) => Expr::App(
            Box::new(rename_fvar(*f, old, new)),
            Box::new(rename_fvar(*a, old, new)),
        ),
        Expr::Lam(bi, n, ty, body) => Expr::Lam(
            bi,
            n,
            Box::new(rename_fvar(*ty, old, new)),
            Box::new(rename_fvar(*body, old, new)),
        ),
        Expr::Pi(bi, n, ty, body) => Expr::Pi(
            bi,
            n,
            Box::new(rename_fvar(*ty, old, new)),
            Box::new(rename_fvar(*body, old, new)),
        ),
        Expr::Let(n, ty, val, body) => Expr::Let(
            n,
            Box::new(rename_fvar(*ty, old, new)),
            Box::new(rename_fvar(*val, old, new)),
            Box::new(rename_fvar(*body, old, new)),
        ),
        other => other,
    }
}
/// Collect all free variable IDs in an expression.
#[allow(dead_code)]
pub fn collect_fvars(expr: &Expr) -> Vec<FVarId> {
    let mut result = Vec::new();
    collect_fvars_helper(expr, &mut result);
    result
}
pub(super) fn collect_fvars_helper(expr: &Expr, acc: &mut Vec<FVarId>) {
    match expr {
        Expr::FVar(id) if !acc.contains(id) => {
            acc.push(*id);
        }
        Expr::FVar(_) => {}
        Expr::App(f, a) => {
            collect_fvars_helper(f, acc);
            collect_fvars_helper(a, acc);
        }
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            collect_fvars_helper(ty, acc);
            collect_fvars_helper(body, acc);
        }
        Expr::Let(_, ty, val, body) => {
            collect_fvars_helper(ty, acc);
            collect_fvars_helper(val, acc);
            collect_fvars_helper(body, acc);
        }
        _ => {}
    }
}
/// Count the number of occurrences of a free variable in an expression.
#[allow(dead_code)]
pub fn count_fvar(expr: &Expr, fvar: FVarId) -> usize {
    match expr {
        Expr::FVar(id) if *id == fvar => 1,
        Expr::FVar(_) => 0,
        Expr::App(f, a) => count_fvar(f, fvar) + count_fvar(a, fvar),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            count_fvar(ty, fvar) + count_fvar(body, fvar)
        }
        Expr::Let(_, ty, val, body) => {
            count_fvar(ty, fvar) + count_fvar(val, fvar) + count_fvar(body, fvar)
        }
        _ => 0,
    }
}
/// Check whether a free variable occurs in an expression.
#[allow(dead_code)]
pub fn fvar_occurs(expr: &Expr, fvar: FVarId) -> bool {
    count_fvar(expr, fvar) > 0
}
#[cfg(test)]
mod scoped_ctx_tests {
    use super::*;
    use crate::Level;
    #[test]
    fn test_rename_fvar() {
        let old = FVarId(0);
        let new = FVarId(1);
        let expr = Expr::App(
            Box::new(Expr::Const(Name::str("f"), vec![])),
            Box::new(Expr::FVar(old)),
        );
        let renamed = rename_fvar(expr, old, new);
        if let Expr::App(_, a) = renamed {
            assert_eq!(*a, Expr::FVar(new));
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_collect_fvars() {
        let e1 = Expr::FVar(FVarId(0));
        let e2 = Expr::FVar(FVarId(1));
        let expr = Expr::App(Box::new(e1), Box::new(e2));
        let fvars = collect_fvars(&expr);
        assert_eq!(fvars.len(), 2);
    }
    #[test]
    fn test_collect_fvars_dedup() {
        let fv = FVarId(5);
        let expr = Expr::App(Box::new(Expr::FVar(fv)), Box::new(Expr::FVar(fv)));
        let fvars = collect_fvars(&expr);
        assert_eq!(fvars.len(), 1);
    }
    #[test]
    fn test_count_fvar() {
        let fv = FVarId(3);
        let expr = Expr::App(Box::new(Expr::FVar(fv)), Box::new(Expr::FVar(fv)));
        assert_eq!(count_fvar(&expr, fv), 2);
        assert_eq!(count_fvar(&expr, FVarId(99)), 0);
    }
    #[test]
    fn test_fvar_occurs() {
        let fv = FVarId(7);
        let expr = Expr::FVar(fv);
        assert!(fvar_occurs(&expr, fv));
        assert!(!fvar_occurs(&expr, FVarId(8)));
    }
    #[test]
    fn test_scoped_context_push_pop() {
        let mut ctx = ScopedContext::new();
        ctx.push_scope();
        ctx.add_local(Name::str("x"), Expr::Sort(Level::zero()));
        assert_eq!(ctx.num_locals(), 1);
        ctx.pop_scope();
        assert_eq!(ctx.num_locals(), 0);
    }
    #[test]
    fn test_scoped_context_depth() {
        let mut ctx = ScopedContext::new();
        assert_eq!(ctx.scope_depth(), 0);
        ctx.push_scope();
        assert_eq!(ctx.scope_depth(), 1);
        ctx.push_scope();
        assert_eq!(ctx.scope_depth(), 2);
        ctx.pop_scope();
        assert_eq!(ctx.scope_depth(), 1);
    }
    #[test]
    fn test_hyp_context_add_find() {
        let mut ctx = HypContext::new();
        let ty = Expr::Sort(Level::zero());
        let fvar = ctx.add_hyp(Name::str("h"), ty);
        let found = ctx.find_hyp(&Name::str("h"));
        assert_eq!(found, Some(fvar));
        assert_eq!(ctx.num_hyps(), 1);
    }
    #[test]
    fn test_hyp_context_not_found() {
        let ctx = HypContext::new();
        assert!(ctx.find_hyp(&Name::str("missing")).is_none());
    }
    #[test]
    fn test_hyp_context_hyp_type() {
        let mut ctx = HypContext::new();
        let ty = Expr::Sort(Level::zero());
        let fvar = ctx.add_hyp(Name::str("h"), ty.clone());
        assert_eq!(ctx.hyp_type(fvar), Some(&ty));
    }
    #[test]
    fn test_hyp_context_remove_last() {
        let mut ctx = HypContext::new();
        let ty = Expr::Sort(Level::zero());
        ctx.add_hyp(Name::str("h1"), ty.clone());
        ctx.add_hyp(Name::str("h2"), ty);
        ctx.remove_last_hyp();
        assert_eq!(ctx.num_hyps(), 1);
        assert!(ctx.find_hyp(&Name::str("h2")).is_none());
    }
    #[test]
    fn test_fresh_name_seq() {
        let mut seq = FreshNameSeq::new("x");
        let n1 = seq.next();
        let n2 = seq.next();
        assert_ne!(n1, n2);
        assert_eq!(seq.count(), 2);
    }
    #[test]
    fn test_fresh_name_seq_reserve() {
        let mut seq = FreshNameSeq::new("h");
        seq.reserve("h");
        let n = seq.next();
        assert_ne!(n, Name::str("h"));
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
