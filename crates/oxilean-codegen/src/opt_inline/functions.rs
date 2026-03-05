//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::{LcnfArg, LcnfExpr, LcnfFunDecl, LcnfLetValue, LcnfLit, LcnfType, LcnfVarId};
use std::collections::{HashMap, HashSet};

use super::types::{
    CallFrequencyAnalyzer, CallGraph, CallSite, CalleeSizeTable, CloneSpecializer,
    ExtendedInlinePass, ExtendedInlineStats, FreshVarGen, HotPath, InlineAnnotation,
    InlineAnnotationRegistry, InlineBudget, InlineConfig, InlineContextStack, InlineCost,
    InlineDecision, InlineFusionManager, InlineHeuristics, InlineHistory, InlineOrderScheduler,
    InlinePass, InlineProfile, InlineProfitabilityEstimator, InlineReport, InlineTrace,
    InlineTraceEntry, InliningContext, InterproceduralInlinePass, NestingDepthTracker,
    PartialInlineDecision, PartialInlineRegion, RecursiveInlineLimiter, SpeculativeInlineRecord,
    SpeculativeInliner, TarjanScc,
};

/// Estimate the "size" of a function declaration in abstract units.
///
/// Each let-binding contributes 1, each case alternative contributes 2,
/// and the base overhead is 1.
pub fn estimate_size(decl: &LcnfFunDecl) -> u64 {
    1 + count_expr_size(&decl.body)
}
pub(super) fn count_expr_size(expr: &LcnfExpr) -> u64 {
    match expr {
        LcnfExpr::Let { value, body, .. } => {
            let value_cost = match value {
                LcnfLetValue::App(_, args) => 1 + args.len() as u64,
                LcnfLetValue::Ctor(_, _, args) | LcnfLetValue::Reuse(_, _, _, args) => {
                    1 + args.len() as u64
                }
                LcnfLetValue::Proj(..) => 1,
                LcnfLetValue::Reset(..) => 1,
                LcnfLetValue::Lit(_) | LcnfLetValue::Erased | LcnfLetValue::FVar(_) => 1,
            };
            value_cost + count_expr_size(body)
        }
        LcnfExpr::Case { alts, default, .. } => {
            let alt_cost: u64 = alts.iter().map(|a| 2 + count_expr_size(&a.body)).sum();
            let default_cost = default.as_ref().map_or(0, |d| 2 + count_expr_size(d));
            1 + alt_cost + default_cost
        }
        LcnfExpr::Return(_) | LcnfExpr::TailCall(_, _) | LcnfExpr::Unreachable => 1,
    }
}
/// Substitute `params` with `args` throughout `body`.
///
/// Each parameter whose name matches a key in `subst` is replaced by a
/// `Return`-wrapped copy of the argument value.  Because LCNF bodies are
/// in ANF, we wrap argument variables/literals in `let` bindings to
/// preserve the ANF invariant.
pub(super) fn substitute_params(
    body: &LcnfExpr,
    params: &[String],
    args: &[LcnfArg],
    gen: &mut FreshVarGen,
) -> LcnfExpr {
    let mut param_map: HashMap<String, LcnfArg> = HashMap::new();
    for (param, arg) in params.iter().zip(args.iter()) {
        param_map.insert(param.clone(), arg.clone());
    }
    subst_expr(body, &param_map, gen)
}
#[allow(clippy::only_used_in_recursion)]
pub(super) fn subst_expr(
    expr: &LcnfExpr,
    map: &HashMap<String, LcnfArg>,
    gen: &mut FreshVarGen,
) -> LcnfExpr {
    match expr {
        LcnfExpr::Let {
            id,
            name,
            ty,
            value,
            body,
        } => {
            let new_value = subst_let_value(value, map);
            let new_body = subst_expr(body, map, gen);
            LcnfExpr::Let {
                id: *id,
                name: name.clone(),
                ty: ty.clone(),
                value: new_value,
                body: Box::new(new_body),
            }
        }
        LcnfExpr::Case {
            scrutinee,
            scrutinee_ty,
            alts,
            default,
        } => {
            let new_scrutinee = *scrutinee;
            let new_alts = alts
                .iter()
                .map(|alt| crate::lcnf::LcnfAlt {
                    ctor_name: alt.ctor_name.clone(),
                    ctor_tag: alt.ctor_tag,
                    params: alt.params.clone(),
                    body: subst_expr(&alt.body, map, gen),
                })
                .collect();
            let new_default = default.as_ref().map(|d| Box::new(subst_expr(d, map, gen)));
            LcnfExpr::Case {
                scrutinee: new_scrutinee,
                scrutinee_ty: scrutinee_ty.clone(),
                alts: new_alts,
                default: new_default,
            }
        }
        LcnfExpr::Return(arg) => LcnfExpr::Return(subst_arg(arg, map)),
        LcnfExpr::TailCall(func, args) => LcnfExpr::TailCall(
            subst_arg(func, map),
            args.iter().map(|a| subst_arg(a, map)).collect(),
        ),
        LcnfExpr::Unreachable => LcnfExpr::Unreachable,
    }
}
pub(super) fn subst_let_value(
    value: &LcnfLetValue,
    map: &HashMap<String, LcnfArg>,
) -> LcnfLetValue {
    match value {
        LcnfLetValue::App(func, args) => LcnfLetValue::App(
            subst_arg(func, map),
            args.iter().map(|a| subst_arg(a, map)).collect(),
        ),
        LcnfLetValue::Ctor(name, tag, args) => LcnfLetValue::Ctor(
            name.clone(),
            *tag,
            args.iter().map(|a| subst_arg(a, map)).collect(),
        ),
        LcnfLetValue::Reuse(slot, name, tag, args) => LcnfLetValue::Reuse(
            *slot,
            name.clone(),
            *tag,
            args.iter().map(|a| subst_arg(a, map)).collect(),
        ),
        LcnfLetValue::Proj(name, idx, var) => LcnfLetValue::Proj(name.clone(), *idx, *var),
        LcnfLetValue::Reset(var) => LcnfLetValue::Reset(*var),
        LcnfLetValue::Lit(lit) => LcnfLetValue::Lit(lit.clone()),
        LcnfLetValue::Erased => LcnfLetValue::Erased,
        LcnfLetValue::FVar(id) => LcnfLetValue::FVar(*id),
    }
}
pub(super) fn subst_arg(arg: &LcnfArg, map: &HashMap<String, LcnfArg>) -> LcnfArg {
    match arg {
        LcnfArg::Lit(lit) => LcnfArg::Lit(lit.clone()),
        LcnfArg::Var(id) => {
            let name_str = format!("_x{}", id.0);
            if let Some(replacement) = map.get(&name_str) {
                replacement.clone()
            } else {
                LcnfArg::Var(*id)
            }
        }
        LcnfArg::Erased => LcnfArg::Erased,
        LcnfArg::Type(ty) => LcnfArg::Type(ty.clone()),
    }
}
/// Substitute `inlined_body` as the "value" portion of a let, with `continuation`
/// as the next expression.  Because LCNF is in ANF, we simply sequence the
/// inlined body and then the continuation under a fresh result binding.
pub(super) fn splice_inlined(inlined: LcnfExpr, continuation: LcnfExpr) -> LcnfExpr {
    match inlined {
        LcnfExpr::Return(_) => continuation,
        LcnfExpr::Unreachable => LcnfExpr::Unreachable,
        other => sequence_exprs(other, continuation),
    }
}
/// Append `next` after all terminal nodes of `first`.
pub(super) fn sequence_exprs(first: LcnfExpr, next: LcnfExpr) -> LcnfExpr {
    match first {
        LcnfExpr::Let {
            id,
            name,
            ty,
            value,
            body,
        } => LcnfExpr::Let {
            id,
            name,
            ty,
            value,
            body: Box::new(sequence_exprs(*body, next)),
        },
        LcnfExpr::Case {
            scrutinee,
            scrutinee_ty,
            alts,
            default,
        } => LcnfExpr::Case {
            scrutinee,
            scrutinee_ty,
            alts: alts
                .into_iter()
                .map(|alt| crate::lcnf::LcnfAlt {
                    ctor_name: alt.ctor_name,
                    ctor_tag: alt.ctor_tag,
                    params: alt.params,
                    body: sequence_exprs(alt.body, next.clone()),
                })
                .collect(),
            default: default.map(|d| Box::new(sequence_exprs(*d, next.clone()))),
        },
        LcnfExpr::Return(_) => next,
        LcnfExpr::TailCall(_, _) | LcnfExpr::Unreachable => first,
    }
}
/// Convert an [`LcnfArg`] to a [`LcnfLetValue`] suitable for a let-binding.
pub(super) fn arg_to_let_value(arg: &LcnfArg) -> LcnfLetValue {
    match arg {
        LcnfArg::Lit(lit) => LcnfLetValue::Lit(lit.clone()),
        LcnfArg::Var(id) => LcnfLetValue::FVar(*id),
        LcnfArg::Erased => LcnfLetValue::Erased,
        LcnfArg::Type(_) => LcnfLetValue::Erased,
    }
}
/// Run the inlining pass with default configuration and return a report.
pub fn run_inline_pass(decls: &mut [LcnfFunDecl]) -> InlineReport {
    let mut pass = InlinePass::default();
    pass.run(decls);
    pass.report().clone()
}
/// Run the inlining pass with custom configuration and return a report.
pub fn run_inline_pass_with_config(
    decls: &mut [LcnfFunDecl],
    config: InlineConfig,
) -> InlineReport {
    let mut pass = InlinePass::new(config);
    pass.run(decls);
    pass.report().clone()
}
/// Collect all function names called in an expression.
#[allow(dead_code)]
pub fn collect_callees(expr: &LcnfExpr) -> Vec<String> {
    let mut out = Vec::new();
    collect_callees_rec(expr, &mut out);
    out
}
#[allow(dead_code)]
pub(super) fn collect_callees_rec(expr: &LcnfExpr, out: &mut Vec<String>) {
    match expr {
        LcnfExpr::Let { value, body, .. } => {
            if let LcnfLetValue::App(LcnfArg::Lit(LcnfLit::Str(name)), _) = value {
                if !out.contains(name) {
                    out.push(name.clone());
                }
            }
            collect_callees_rec(body, out);
        }
        LcnfExpr::Case { alts, default, .. } => {
            for alt in alts {
                collect_callees_rec(&alt.body, out);
            }
            if let Some(def) = default {
                collect_callees_rec(def, out);
            }
        }
        LcnfExpr::TailCall(LcnfArg::Lit(LcnfLit::Str(name)), _) => {
            if !out.contains(name) {
                out.push(name.clone());
            }
        }
        _ => {}
    }
}
#[allow(dead_code)]
pub(super) fn inline_subst(expr: LcnfExpr, from: LcnfVarId, to: LcnfArg) -> LcnfExpr {
    match expr {
        LcnfExpr::Let {
            id,
            name,
            ty,
            value,
            body,
        } => {
            let value2 = inline_subst_value(value, from, &to);
            let body2 = inline_subst(*body, from, to);
            LcnfExpr::Let {
                id,
                name,
                ty,
                value: value2,
                body: Box::new(body2),
            }
        }
        LcnfExpr::Case {
            scrutinee,
            scrutinee_ty,
            alts,
            default,
        } => {
            let s2 = if scrutinee == from {
                match &to {
                    LcnfArg::Var(v) => *v,
                    _ => scrutinee,
                }
            } else {
                scrutinee
            };
            let alts2 = alts
                .into_iter()
                .map(|alt| crate::lcnf::LcnfAlt {
                    ctor_name: alt.ctor_name,
                    ctor_tag: alt.ctor_tag,
                    params: alt.params,
                    body: inline_subst(alt.body, from, to.clone()),
                })
                .collect();
            let default2 = default.map(|d| Box::new(inline_subst(*d, from, to)));
            LcnfExpr::Case {
                scrutinee: s2,
                scrutinee_ty,
                alts: alts2,
                default: default2,
            }
        }
        LcnfExpr::Return(arg) => LcnfExpr::Return(inline_subst_arg(arg, from, &to)),
        LcnfExpr::TailCall(func, args) => LcnfExpr::TailCall(
            inline_subst_arg(func, from, &to),
            args.into_iter()
                .map(|a| inline_subst_arg(a, from, &to))
                .collect(),
        ),
        LcnfExpr::Unreachable => LcnfExpr::Unreachable,
    }
}
#[allow(dead_code)]
pub(super) fn inline_subst_arg(arg: LcnfArg, from: LcnfVarId, to: &LcnfArg) -> LcnfArg {
    match &arg {
        LcnfArg::Var(id) if *id == from => to.clone(),
        _ => arg,
    }
}
#[allow(dead_code)]
pub(super) fn inline_subst_value(
    value: LcnfLetValue,
    from: LcnfVarId,
    to: &LcnfArg,
) -> LcnfLetValue {
    match value {
        LcnfLetValue::App(func, args) => LcnfLetValue::App(
            inline_subst_arg(func, from, to),
            args.into_iter()
                .map(|a| inline_subst_arg(a, from, to))
                .collect(),
        ),
        LcnfLetValue::Ctor(name, tag, args) => LcnfLetValue::Ctor(
            name,
            tag,
            args.into_iter()
                .map(|a| inline_subst_arg(a, from, to))
                .collect(),
        ),
        LcnfLetValue::Proj(name, idx, var) => {
            let v2 = if var == from {
                match to {
                    LcnfArg::Var(v) => *v,
                    _ => var,
                }
            } else {
                var
            };
            LcnfLetValue::Proj(name, idx, v2)
        }
        LcnfLetValue::FVar(var) => {
            if var == from {
                match to {
                    LcnfArg::Var(v) => LcnfLetValue::FVar(*v),
                    _ => LcnfLetValue::FVar(var),
                }
            } else {
                LcnfLetValue::FVar(var)
            }
        }
        LcnfLetValue::Reset(var) => {
            let v2 = if var == from {
                match to {
                    LcnfArg::Var(v) => *v,
                    _ => var,
                }
            } else {
                var
            };
            LcnfLetValue::Reset(v2)
        }
        LcnfLetValue::Reuse(slot, name, tag, args) => {
            let s2 = if slot == from {
                match to {
                    LcnfArg::Var(v) => *v,
                    _ => slot,
                }
            } else {
                slot
            };
            LcnfLetValue::Reuse(
                s2,
                name,
                tag,
                args.into_iter()
                    .map(|a| inline_subst_arg(a, from, to))
                    .collect(),
            )
        }
        other => other,
    }
}
#[cfg(test)]
mod inline_tests {
    use super::*;
    use crate::lcnf::LcnfFunDecl;
    pub(super) fn make_trivial_decl(name: &str) -> LcnfFunDecl {
        LcnfFunDecl {
            name: name.to_owned(),
            original_name: None,
            params: vec![],
            ret_type: LcnfType::Object,
            body: LcnfExpr::Return(LcnfArg::Erased),
            is_recursive: false,
            is_lifted: false,
            inline_cost: 1,
        }
    }
    pub(super) fn make_calling_decl(from: &str, to: &str) -> LcnfFunDecl {
        LcnfFunDecl {
            name: from.to_owned(),
            original_name: None,
            params: vec![],
            ret_type: LcnfType::Object,
            body: LcnfExpr::TailCall(LcnfArg::Lit(LcnfLit::Str(to.to_owned())), vec![]),
            is_recursive: false,
            is_lifted: false,
            inline_cost: 2,
        }
    }
    #[test]
    pub(super) fn inline_decision_basics() {
        assert!(InlineDecision::Always.should_inline(0));
        assert!(!InlineDecision::Never.should_inline(0));
        assert!(InlineDecision::Heuristic(0.8).should_inline(0));
        assert!(!InlineDecision::Heuristic(0.3).should_inline(0));
        assert!(InlineDecision::OnceOnly.should_inline(0));
        assert!(!InlineDecision::OnceOnly.should_inline(1));
    }
    #[test]
    pub(super) fn inline_cost_net_gain() {
        let cost = InlineCost {
            body_size: 5,
            call_overhead: 4,
            estimated_savings: 3,
        };
        assert_eq!(cost.net_gain(), 2);
        assert!(cost.is_profitable());
    }
    #[test]
    pub(super) fn call_site_benefit() {
        let site = CallSite::new("f", "g", 1, true, false);
        assert_eq!(site.inline_benefit(), 13);
        let rec = CallSite::new("f", "f", 2, false, true);
        assert_eq!(rec.inline_benefit(), 2);
    }
    #[test]
    pub(super) fn inline_profile_hot() {
        let mut p = InlineProfile::new();
        for _ in 0..5 {
            p.record_call("foo");
        }
        assert!(p.is_hot("foo", 5));
        assert!(!p.is_hot("foo", 6));
        assert_eq!(p.top_callees(1)[0].0, "foo");
    }
    #[test]
    pub(super) fn heuristics_decide_tiny_fn() {
        let h = InlineHeuristics::default();
        let decl = make_trivial_decl("f");
        let dec = h.decide(&decl, &InlineProfile::new());
        assert_eq!(dec, InlineDecision::Always);
    }
    #[test]
    pub(super) fn inlining_context_cycle() {
        let mut ctx = InliningContext::new();
        assert!(ctx.push_call("foo"));
        assert!(!ctx.push_call("foo"));
        ctx.pop_call();
        assert!(ctx.push_call("foo"));
    }
    #[test]
    pub(super) fn estimate_size_trivial() {
        let decl = make_trivial_decl("f");
        assert_eq!(estimate_size(&decl), 2);
    }
    #[test]
    pub(super) fn inline_pass_smoke() {
        let mut decls = vec![make_trivial_decl("main"), make_calling_decl("f", "main")];
        let report = run_inline_pass(&mut decls);
        let _ = report.summary();
    }
    #[test]
    pub(super) fn tarjan_no_recursion() {
        let decls = vec![make_trivial_decl("a"), make_trivial_decl("b")];
        let mut scc = TarjanScc::new(&decls);
        scc.compute();
        assert!(!scc.is_recursive("a"));
    }
    #[test]
    pub(super) fn budget_spend() {
        let mut b = InlineBudget::new(100);
        assert!(b.try_spend("f", 40));
        assert!(b.try_spend("f", 40));
        assert!(!b.try_spend("f", 40));
        assert_eq!(b.remaining(), 20);
    }
    #[test]
    pub(super) fn hot_path_trivial() {
        let decl = make_trivial_decl("f");
        assert!(!HotPath::extract(&decl).has_prefix());
    }
    #[test]
    pub(super) fn speculative_inliner_committed() {
        let mut si = SpeculativeInliner::new(0.6);
        si.add(SpeculativeInlineRecord::new("f", "g", 0.8, "Nat"));
        si.add(SpeculativeInlineRecord::new("f", "h", 0.3, "Bool"));
        assert_eq!(si.committed().len(), 1);
    }
    #[test]
    pub(super) fn annotation_registry() {
        let mut reg = InlineAnnotationRegistry::new();
        reg.register("f", InlineAnnotation::AlwaysInline);
        reg.register("g", InlineAnnotation::NeverInline);
        assert_eq!(
            reg.apply("f", InlineDecision::Heuristic(0.2)),
            InlineDecision::Always
        );
        assert_eq!(
            reg.apply("g", InlineDecision::Always),
            InlineDecision::Never
        );
    }
    #[test]
    pub(super) fn callee_size_table() {
        let decls = vec![make_trivial_decl("a"), make_trivial_decl("b")];
        let t = CalleeSizeTable::build(&decls);
        assert_eq!(t.len(), 2);
        assert!(t.size_of("a").is_some());
    }
    #[test]
    pub(super) fn nesting_tracker() {
        let mut t = NestingDepthTracker::new(2);
        assert!(t.push());
        assert!(t.push());
        assert!(!t.push());
        assert_eq!(t.limit_hit_count, 1);
        assert_eq!(t.peak_depth, 2);
        t.pop();
        assert_eq!(t.remaining(), 1);
    }
    #[test]
    pub(super) fn extended_stats_summary() {
        let mut s = ExtendedInlineStats::new();
        s.record_decision(&InlineDecision::Always, true);
        s.record_size_change(100, 50);
        assert_eq!(s.net_size_change(), 50);
        assert!(s.summary().contains("InlineStats"));
    }
    #[test]
    pub(super) fn call_freq_analyzer() {
        let decls = vec![make_calling_decl("f", "g"), make_calling_decl("h", "g")];
        let mut p = CallFrequencyAnalyzer::analyze(&decls);
        assert_eq!(p.call_counts.get("g").copied(), Some(2));
        CallFrequencyAnalyzer::mark_hot(&mut p, 2);
        assert!(p.hot_functions.contains("g"));
    }
    #[test]
    pub(super) fn recursive_limiter() {
        let mut lim = RecursiveInlineLimiter::new(2);
        assert!(lim.try_unroll("f"));
        assert!(lim.try_unroll("f"));
        assert!(!lim.try_unroll("f"));
        lim.pop_unroll("f");
        assert!(lim.try_unroll("f"));
    }
    #[test]
    pub(super) fn extended_pass_init() {
        let decls = vec![make_trivial_decl("main")];
        let mut pass = ExtendedInlinePass::new(InlineConfig::default(), 10000);
        pass.init_scc(&decls);
        assert!(pass.scc.is_some());
        assert_eq!(pass.size_table.len(), 1);
    }
    #[test]
    pub(super) fn call_graph_build_and_query() {
        let decls = vec![make_calling_decl("f", "g"), make_trivial_decl("g")];
        let g = CallGraph::build(&decls);
        assert_eq!(g.num_nodes(), 2);
        assert_eq!(g.in_degree("g"), 1);
        assert!(g.leaf_functions().contains(&"g"));
    }
    #[test]
    pub(super) fn inline_order_bottom_up() {
        let decls = vec![make_calling_decl("f", "g"), make_trivial_decl("g")];
        let g = CallGraph::build(&decls);
        let sched = InlineOrderScheduler::compute(&g);
        let gp = sched.order.iter().position(|n| n == "g");
        let fp = sched.order.iter().position(|n| n == "f");
        if let (Some(gi), Some(fi)) = (gp, fp) {
            assert!(gi < fi);
        }
    }
    #[test]
    pub(super) fn inline_trace_disabled() {
        let mut t = InlineTrace::disabled();
        t.record(InlineTraceEntry::new(0, "a", "b", "always", 1, true));
        assert!(t.is_empty());
    }
    #[test]
    pub(super) fn inline_trace_csv() {
        let mut t = InlineTrace::new();
        t.record(InlineTraceEntry::new(1, "f", "g", "always", 5, true));
        let csv = t.to_csv();
        assert!(csv.contains("f,g,always,5,true"));
    }
    #[test]
    pub(super) fn inline_report_rate() {
        let r = InlineReport {
            total_calls_considered: 10,
            inlined_count: 7,
            skipped_recursive: 1,
            skipped_too_large: 2,
        };
        assert!((r.inline_rate() - 0.7).abs() < 1e-9);
        assert!(r.summary().contains("7/10"));
    }
    #[test]
    pub(super) fn collect_callees_fn() {
        let expr = LcnfExpr::TailCall(LcnfArg::Lit(LcnfLit::Str("foo".to_owned())), vec![]);
        assert_eq!(collect_callees(&expr), vec!["foo".to_owned()]);
    }
}
#[cfg(test)]
mod inline_extended_tests {
    use super::*;
    #[test]
    pub(super) fn inline_context_stack_basic() {
        let mut stack = InlineContextStack::new();
        assert_eq!(stack.depth(), 0);
        stack.push("foo", 0);
        stack.push("bar", 1);
        assert_eq!(stack.depth(), 2);
        assert!(stack.contains("foo"));
        assert!(stack.contains("bar"));
        assert!(!stack.contains("baz"));
        let fp = stack.fingerprint();
        assert!(fp.contains("foo"));
        assert!(fp.contains("bar"));
        let frame = stack.pop().expect("frame should be available to pop");
        assert_eq!(frame.callee, "bar");
        assert_eq!(stack.depth(), 1);
    }
    #[test]
    pub(super) fn partial_inline_decision_will_inline() {
        let d = PartialInlineDecision::full("f", 10);
        assert!(d.will_inline());
        let d2 = PartialInlineDecision::no_inline("g", "too_large");
        assert!(!d2.will_inline());
        let d3 = PartialInlineDecision::prefix("h", 3, 5);
        assert!(d3.will_inline());
        match d3.region {
            PartialInlineRegion::Prefix(n) => assert_eq!(n, 3),
            _ => panic!("expected Prefix"),
        }
    }
    #[test]
    pub(super) fn profitability_estimator_basic() {
        let est = InlineProfitabilityEstimator::new();
        assert!(est.is_profitable(2, 5.0, true));
        assert!(!est.is_profitable(1000, 1.0, false));
    }
    #[test]
    pub(super) fn clone_specializer_records() {
        let mut cs = CloneSpecializer::new();
        let name = cs.record("add", 0, "42");
        assert!(name.starts_with("add_spec_0_42_c"));
        assert_eq!(cs.count(), 1);
    }
    #[test]
    pub(super) fn inline_history_tracking() {
        let mut h = InlineHistory::new();
        assert!(!h.has_seen("f", "g"));
        h.mark_seen("f", "g");
        assert!(h.has_seen("f", "g"));
        assert_eq!(h.count(), 1);
        h.reset();
        assert!(!h.has_seen("f", "g"));
        assert_eq!(h.count(), 0);
    }
    #[test]
    pub(super) fn interprocedural_inline_pass_smoke() {
        let config = InlineConfig::default();
        let mut pass = InterproceduralInlinePass::new(config);
        let mut decls: Vec<LcnfFunDecl> = vec![];
        pass.run(&mut decls);
        let report = pass.report();
        assert!(report.contains("0 functions processed"));
    }
    #[test]
    pub(super) fn inline_fusion_manager_basic() {
        let mut mgr = InlineFusionManager::new();
        let name = mgr.fuse("caller", "f", "g", 15);
        assert!(name.starts_with("caller_fused_f_g_"));
        assert_eq!(mgr.all_records().len(), 1);
        assert_eq!(mgr.total_savings(), 15);
        mgr.fuse("caller", "g", "h", 5);
        assert_eq!(mgr.total_savings(), 20);
    }
    #[test]
    pub(super) fn extended_inline_stats_default() {
        let stats = ExtendedInlineStats::default();
        assert_eq!(stats.total_functions_processed, 0);
        assert!(stats.inlining_order.is_empty());
    }
}
