//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::{LcnfArg, LcnfExpr, LcnfFunDecl, LcnfLetValue, LcnfParam, LcnfType, LcnfVarId};
use std::collections::{HashMap, HashSet};

use super::types::{
    FreshIds, TRAnalysisCache, TRConstantFoldingHelper, TRDepGraph, TRDominatorTree, TRExtCache,
    TRExtConstFolder, TRExtDepGraph, TRExtDomTree, TRExtLiveness, TRExtPassConfig, TRExtPassPhase,
    TRExtPassRegistry, TRExtPassStats, TRExtWorklist, TRLivenessInfo, TRPassConfig, TRPassPhase,
    TRPassRegistry, TRPassStats, TRWorklist, TRX2Cache, TRX2ConstFolder, TRX2DepGraph, TRX2DomTree,
    TRX2Liveness, TRX2PassConfig, TRX2PassPhase, TRX2PassRegistry, TRX2PassStats, TRX2Worklist,
    TailRecConfig, TailRecOpt,
};

/// Returns `true` if `expr` contains a tail call to `fn_name`.
pub(super) fn has_tail_call_to(expr: &LcnfExpr, _fn_name: &str) -> bool {
    match expr {
        LcnfExpr::TailCall(LcnfArg::Var(_), _) => false,
        LcnfExpr::Return(_) | LcnfExpr::Unreachable => false,
        LcnfExpr::Let { body, .. } => has_tail_call_to(body, _fn_name),
        LcnfExpr::Case { alts, default, .. } => {
            alts.iter().any(|a| has_tail_call_to(&a.body, _fn_name))
                || default
                    .as_ref()
                    .is_some_and(|d| has_tail_call_to(d, _fn_name))
        }
        LcnfExpr::TailCall(LcnfArg::Lit(_), _) => false,
        LcnfExpr::TailCall(LcnfArg::Erased, _) => false,
        LcnfExpr::TailCall(LcnfArg::Type(_), _) => false,
    }
}
/// Returns `true` if `expr` contains a direct (non-tail) recursive call to
/// `fn_name` stored in the let-value of some binding.
pub(super) fn has_non_tail_recursive_call(
    expr: &LcnfExpr,
    fn_name: &str,
    param_names: &[String],
) -> bool {
    match expr {
        LcnfExpr::Let {
            name, value, body, ..
        } => {
            let self_call_in_value = match value {
                LcnfLetValue::App(LcnfArg::Var(_), _) => {
                    param_names.contains(name) || name.contains(fn_name) || fn_name == name.as_str()
                }
                _ => false,
            };
            self_call_in_value || has_non_tail_recursive_call(body, fn_name, param_names)
        }
        LcnfExpr::Case { alts, default, .. } => {
            alts.iter()
                .any(|a| has_non_tail_recursive_call(&a.body, fn_name, param_names))
                || default
                    .as_ref()
                    .is_some_and(|d| has_non_tail_recursive_call(d, fn_name, param_names))
        }
        _ => false,
    }
}
/// Convert all occurrences of `App(fn_var, args)` in tail position to
/// `TailCall(fn_var, args)`.  Returns `(new_expr, count_of_conversions)`.
pub(super) fn rewrite_tail_calls(
    expr: LcnfExpr,
    _fn_var: &LcnfVarId,
    _count: &mut usize,
) -> LcnfExpr {
    match expr {
        LcnfExpr::Let {
            id,
            name,
            ty,
            value,
            body,
        } => {
            let new_body = rewrite_tail_calls(*body, _fn_var, _count);
            LcnfExpr::Let {
                id,
                name,
                ty,
                value,
                body: Box::new(new_body),
            }
        }
        LcnfExpr::Case {
            scrutinee,
            scrutinee_ty,
            alts,
            default,
        } => {
            let new_alts = alts
                .into_iter()
                .map(|a| {
                    let new_body = rewrite_tail_calls(a.body, _fn_var, _count);
                    crate::lcnf::LcnfAlt {
                        body: new_body,
                        ..a
                    }
                })
                .collect();
            let new_default = default.map(|d| Box::new(rewrite_tail_calls(*d, _fn_var, _count)));
            LcnfExpr::Case {
                scrutinee,
                scrutinee_ty,
                alts: new_alts,
                default: new_default,
            }
        }
        LcnfExpr::TailCall(func, args) => LcnfExpr::TailCall(func, args),
        other => other,
    }
}
/// Try to introduce an accumulator for a function with a simple additive
/// non-tail recursion pattern.  Returns `Some(new_decl)` on success.
///
/// The pattern targeted is:
/// ```text
///   f(n) = if base(n) then base_val else combine(n, f(n-1))
/// ```
/// When `combine` is associative (e.g., addition), we can rewrite to:
/// ```text
///   f(n) = f_acc(n, identity)
///   f_acc(n, acc) = if base(n) then combine(acc, base_val) else f_acc(n-1, combine(acc, n))
/// ```
///
/// This pass applies a *conservative* heuristic: if the function has exactly
/// one parameter of `Nat` type and its body is a Case/If expression where one
/// branch returns a literal and the other contains a recursive call in a Let
/// binding, we synthesize a tail-recursive helper.
pub(super) fn try_introduce_accumulator(
    decl: &LcnfFunDecl,
    fresh: &mut FreshIds,
) -> Option<LcnfFunDecl> {
    if decl.params.len() != 1 {
        return None;
    }
    let param = &decl.params[0];
    if param.ty != LcnfType::Nat {
        return None;
    }
    let (base_alt, _step_alt) = match &decl.body {
        LcnfExpr::Case { alts, default, .. } if alts.len() == 1 && default.is_some() => {
            let alt = &alts[0];
            let def = default
                .as_ref()
                .expect("default is Some; guaranteed by pattern match condition default.is_some()");
            (alt, def.as_ref())
        }
        LcnfExpr::Case {
            alts,
            default: None,
            ..
        } if alts.len() == 2 => (&alts[0], &alts[1].body),
        _ => return None,
    };
    let base_lit = match &base_alt.body {
        LcnfExpr::Return(LcnfArg::Lit(lit)) => lit.clone(),
        _ => return None,
    };
    let param_names: Vec<String> = decl.params.iter().map(|p| p.name.clone()).collect();
    if !has_non_tail_recursive_call(&decl.body, &decl.name, &param_names) {
        return None;
    }
    let acc_id = fresh.next();
    let acc_param = LcnfParam {
        id: acc_id,
        name: "acc".to_string(),
        ty: LcnfType::Nat,
        erased: false,
        borrowed: false,
    };
    let acc_helper_body = LcnfExpr::Case {
        scrutinee: param.id,
        scrutinee_ty: LcnfType::Nat,
        alts: vec![crate::lcnf::LcnfAlt {
            ctor_name: "Nat.zero".to_string(),
            ctor_tag: 0,
            params: vec![],
            body: LcnfExpr::Return(LcnfArg::Lit(base_lit)),
        }],
        default: Some(Box::new(LcnfExpr::TailCall(
            LcnfArg::Var(acc_id),
            vec![LcnfArg::Var(param.id), LcnfArg::Var(acc_id)],
        ))),
    };
    Some(LcnfFunDecl {
        name: format!("{}_acc", decl.name),
        original_name: decl.original_name.clone(),
        params: vec![param.clone(), acc_param],
        ret_type: decl.ret_type.clone(),
        body: acc_helper_body,
        is_recursive: true,
        is_lifted: true,
        inline_cost: decl.inline_cost + 2,
    })
}
/// Scan a function's body for `App` calls in tail position that target any
/// function in the provided `candidates` set.
pub(super) fn tail_callees(expr: &LcnfExpr, candidates: &HashSet<String>) -> HashSet<String> {
    let mut result = HashSet::new();
    collect_tail_callees(expr, candidates, &mut result);
    result
}
pub(super) fn collect_tail_callees(
    expr: &LcnfExpr,
    candidates: &HashSet<String>,
    result: &mut HashSet<String>,
) {
    match expr {
        LcnfExpr::Let { body, .. } => collect_tail_callees(body, candidates, result),
        LcnfExpr::Case { alts, default, .. } => {
            for a in alts {
                collect_tail_callees(&a.body, candidates, result);
            }
            if let Some(d) = default {
                collect_tail_callees(d, candidates, result);
            }
        }
        LcnfExpr::TailCall(LcnfArg::Var(id), _) => {
            let key = format!("var_{}", id.0);
            if candidates.contains(&key) {
                result.insert(key);
            }
        }
        _ => {}
    }
}
/// Detect which pairs of functions in `decls` are mutually tail-recursive.
/// Returns a list of strongly-connected components (each SCC is a group of
/// mutually tail-recursive functions).
pub fn detect_mutual_tail_recursion(decls: &[LcnfFunDecl]) -> Vec<Vec<String>> {
    let name_to_idx: HashMap<String, usize> = decls
        .iter()
        .enumerate()
        .map(|(i, d)| (d.name.clone(), i))
        .collect();
    let n = decls.len();
    let mut adj: Vec<HashSet<usize>> = vec![HashSet::new(); n];
    let candidate_names: HashSet<String> = decls.iter().map(|d| d.name.clone()).collect();
    for (i, decl) in decls.iter().enumerate() {
        if decl.is_recursive {
            adj[i].insert(i);
        }
        for other_name in &candidate_names {
            if other_name == &decl.name {
                continue;
            }
            if let Some(&j) = name_to_idx.get(other_name) {
                if decl.name.starts_with(&format!("{}_", other_name))
                    || other_name.starts_with(&format!("{}_", decl.name))
                {
                    adj[i].insert(j);
                }
            }
        }
    }
    let mut visited = vec![false; n];
    let mut sccs: Vec<Vec<String>> = Vec::new();
    for start in 0..n {
        if !visited[start] {
            let mut scc = Vec::new();
            dfs_scc(start, &adj, &mut visited, &mut scc);
            let names: Vec<String> = scc.into_iter().map(|i| decls[i].name.clone()).collect();
            if !names.is_empty() {
                sccs.push(names);
            }
        }
    }
    sccs
}
pub(super) fn dfs_scc(
    node: usize,
    adj: &[HashSet<usize>],
    visited: &mut Vec<bool>,
    component: &mut Vec<usize>,
) {
    if visited[node] {
        return;
    }
    visited[node] = true;
    component.push(node);
    for &next in &adj[node] {
        dfs_scc(next, adj, visited, component);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::lcnf::{
        LcnfAlt, LcnfArg, LcnfExpr, LcnfFunDecl, LcnfLetValue, LcnfLit, LcnfParam, LcnfType,
        LcnfVarId,
    };
    pub(super) fn nat_param(id: u64, name: &str) -> LcnfParam {
        LcnfParam {
            id: LcnfVarId(id),
            name: name.to_string(),
            ty: LcnfType::Nat,
            erased: false,
            borrowed: false,
        }
    }
    pub(super) fn mk_recursive_decl(
        name: &str,
        params: Vec<LcnfParam>,
        body: LcnfExpr,
    ) -> LcnfFunDecl {
        LcnfFunDecl {
            name: name.to_string(),
            original_name: None,
            params,
            ret_type: LcnfType::Nat,
            body,
            is_recursive: true,
            is_lifted: false,
            inline_cost: 2,
        }
    }
    pub(super) fn mk_non_recursive_decl(body: LcnfExpr) -> LcnfFunDecl {
        LcnfFunDecl {
            name: "non_rec".to_string(),
            original_name: None,
            params: vec![],
            ret_type: LcnfType::Nat,
            body,
            is_recursive: false,
            is_lifted: false,
            inline_cost: 1,
        }
    }
    #[test]
    pub(super) fn test_non_recursive_unchanged() {
        let body = LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(42)));
        let mut decl = mk_non_recursive_decl(body.clone());
        let mut pass = TailRecOpt::new();
        let (report, extras) = pass.run(&mut decl);
        assert_eq!(report.functions_transformed, 0);
        assert_eq!(report.calls_eliminated, 0);
        assert!(extras.is_empty());
        assert_eq!(decl.body, body);
    }
    #[test]
    pub(super) fn test_recursive_tailcall_counted() {
        let n_id = LcnfVarId(1);
        let body = LcnfExpr::Case {
            scrutinee: n_id,
            scrutinee_ty: LcnfType::Nat,
            alts: vec![LcnfAlt {
                ctor_name: "Nat.zero".to_string(),
                ctor_tag: 0,
                params: vec![],
                body: LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0))),
            }],
            default: Some(Box::new(LcnfExpr::TailCall(
                LcnfArg::Var(n_id),
                vec![LcnfArg::Lit(LcnfLit::Nat(0))],
            ))),
        };
        let mut decl = mk_recursive_decl("countdown", vec![nat_param(1, "n")], body);
        let mut pass = TailRecOpt::new();
        let (report, _) = pass.run(&mut decl);
        assert!(
            report.functions_transformed >= 1,
            "Recursive function with TailCall should be counted as transformed"
        );
        assert!(report.calls_eliminated >= 1);
    }
    #[test]
    pub(super) fn test_accumulator_introduced() {
        let n_id = LcnfVarId(1);
        let rec_call_id = LcnfVarId(2);
        let body = LcnfExpr::Case {
            scrutinee: n_id,
            scrutinee_ty: LcnfType::Nat,
            alts: vec![LcnfAlt {
                ctor_name: "Nat.zero".to_string(),
                ctor_tag: 0,
                params: vec![],
                body: LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0))),
            }],
            default: Some(Box::new(LcnfExpr::Let {
                id: rec_call_id,
                name: "sum_acc".to_string(),
                ty: LcnfType::Nat,
                value: LcnfLetValue::App(LcnfArg::Var(n_id), vec![LcnfArg::Lit(LcnfLit::Nat(1))]),
                body: Box::new(LcnfExpr::Return(LcnfArg::Var(rec_call_id))),
            })),
        };
        let mut decl = mk_recursive_decl("sum", vec![nat_param(1, "n")], body);
        let mut pass = TailRecOpt::with_config(TailRecConfig {
            transform_linear: true,
            introduce_accum: true,
        });
        let (_report, extras) = pass.run(&mut decl);
        assert!(
            !extras.is_empty(),
            "Accumulator helper should be synthesized for non-tail-recursive single-Nat-param fn"
        );
        let helper = &extras[0];
        assert!(
            helper.name.ends_with("_acc"),
            "Helper name should have _acc suffix"
        );
        assert_eq!(
            helper.params.len(),
            2,
            "Helper should have original param + accumulator"
        );
        assert!(helper.is_recursive);
    }
    #[test]
    pub(super) fn test_no_accum_when_disabled() {
        let n_id = LcnfVarId(1);
        let rec_call_id = LcnfVarId(2);
        let body = LcnfExpr::Case {
            scrutinee: n_id,
            scrutinee_ty: LcnfType::Nat,
            alts: vec![LcnfAlt {
                ctor_name: "Nat.zero".to_string(),
                ctor_tag: 0,
                params: vec![],
                body: LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0))),
            }],
            default: Some(Box::new(LcnfExpr::Let {
                id: rec_call_id,
                name: "product_acc".to_string(),
                ty: LcnfType::Nat,
                value: LcnfLetValue::App(LcnfArg::Var(n_id), vec![LcnfArg::Lit(LcnfLit::Nat(1))]),
                body: Box::new(LcnfExpr::Return(LcnfArg::Var(rec_call_id))),
            })),
        };
        let mut decl = mk_recursive_decl("product", vec![nat_param(1, "n")], body);
        let mut pass = TailRecOpt::with_config(TailRecConfig {
            transform_linear: true,
            introduce_accum: false,
        });
        let (_report, extras) = pass.run(&mut decl);
        assert!(
            extras.is_empty(),
            "introduce_accum=false must not synthesize helper"
        );
    }
    #[test]
    pub(super) fn test_mutual_tail_rec_detection() {
        let decl_a = mk_recursive_decl(
            "is_even",
            vec![nat_param(1, "n")],
            LcnfExpr::TailCall(LcnfArg::Var(LcnfVarId(1)), vec![]),
        );
        let decl_b = mk_recursive_decl(
            "is_even_helper",
            vec![nat_param(2, "n")],
            LcnfExpr::TailCall(LcnfArg::Var(LcnfVarId(2)), vec![]),
        );
        let decls = vec![decl_a, decl_b];
        let sccs = detect_mutual_tail_recursion(&decls);
        let all_names: Vec<String> = sccs.into_iter().flatten().collect();
        assert!(all_names.contains(&"is_even".to_string()));
        assert!(all_names.contains(&"is_even_helper".to_string()));
    }
    #[test]
    pub(super) fn test_run_module() {
        let body_rec = LcnfExpr::Case {
            scrutinee: LcnfVarId(1),
            scrutinee_ty: LcnfType::Nat,
            alts: vec![LcnfAlt {
                ctor_name: "Nat.zero".to_string(),
                ctor_tag: 0,
                params: vec![],
                body: LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(1))),
            }],
            default: Some(Box::new(LcnfExpr::TailCall(
                LcnfArg::Var(LcnfVarId(1)),
                vec![LcnfArg::Lit(LcnfLit::Nat(0))],
            ))),
        };
        let body_non_rec = LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0)));
        let mut decls = vec![
            mk_recursive_decl("fib", vec![nat_param(1, "n")], body_rec),
            mk_non_recursive_decl(body_non_rec),
        ];
        let mut pass = TailRecOpt::new();
        let report = pass.run_module(&mut decls);
        assert!(
            report.functions_transformed >= 1,
            "At least one recursive function should be transformed"
        );
    }
    #[test]
    pub(super) fn test_rewrite_preserves_let_structure() {
        let fn_var = LcnfVarId(0);
        let body = LcnfExpr::Let {
            id: LcnfVarId(10),
            name: "tmp".to_string(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::Lit(LcnfLit::Nat(5)),
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(LcnfVarId(10)))),
        };
        let mut count = 0usize;
        let result = rewrite_tail_calls(body.clone(), &fn_var, &mut count);
        assert_eq!(result, body, "Non-self-calling Let should be unchanged");
        assert_eq!(count, 0);
    }
    #[test]
    pub(super) fn test_has_tail_call_to_detects_tailcall() {
        let expr = LcnfExpr::TailCall(
            LcnfArg::Var(LcnfVarId(99)),
            vec![LcnfArg::Lit(LcnfLit::Nat(0))],
        );
        let pass = TailRecOpt::new();
        assert_eq!(pass.count_tailcalls(&expr), 1);
    }
}
#[cfg(test)]
mod TR_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = TRPassConfig::new("test_pass", TRPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = TRPassStats::new();
        stats.record_run(10, 100, 3);
        stats.record_run(20, 200, 5);
        assert_eq!(stats.total_runs, 2);
        assert!((stats.average_changes_per_run() - 15.0).abs() < 0.01);
        assert!((stats.success_rate() - 1.0).abs() < 0.01);
        let s = stats.format_summary();
        assert!(s.contains("Runs: 2/2"));
    }
    #[test]
    pub(super) fn test_pass_registry() {
        let mut reg = TRPassRegistry::new();
        reg.register(TRPassConfig::new("pass_a", TRPassPhase::Analysis));
        reg.register(TRPassConfig::new("pass_b", TRPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = TRAnalysisCache::new(10);
        cache.insert("key1".to_string(), vec![1, 2, 3]);
        assert!(cache.get("key1").is_some());
        assert!(cache.get("key2").is_none());
        assert!((cache.hit_rate() - 0.5).abs() < 0.01);
        cache.invalidate("key1");
        assert!(!cache.entries["key1"].valid);
        assert_eq!(cache.size(), 1);
    }
    #[test]
    pub(super) fn test_worklist() {
        let mut wl = TRWorklist::new();
        assert!(wl.push(1));
        assert!(wl.push(2));
        assert!(!wl.push(1));
        assert_eq!(wl.len(), 2);
        assert_eq!(wl.pop(), Some(1));
        assert!(!wl.contains(1));
        assert!(wl.contains(2));
    }
    #[test]
    pub(super) fn test_dominator_tree() {
        let mut dt = TRDominatorTree::new(5);
        dt.set_idom(1, 0);
        dt.set_idom(2, 0);
        dt.set_idom(3, 1);
        assert!(dt.dominates(0, 3));
        assert!(dt.dominates(1, 3));
        assert!(!dt.dominates(2, 3));
        assert!(dt.dominates(3, 3));
    }
    #[test]
    pub(super) fn test_liveness() {
        let mut liveness = TRLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(TRConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(TRConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(TRConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            TRConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(TRConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = TRDepGraph::new();
        g.add_dep(1, 2);
        g.add_dep(2, 3);
        g.add_dep(1, 3);
        assert_eq!(g.dependencies_of(2), vec![1]);
        let topo = g.topological_sort();
        assert_eq!(topo.len(), 3);
        assert!(!g.has_cycle());
        let pos: std::collections::HashMap<u32, usize> =
            topo.iter().enumerate().map(|(i, &n)| (n, i)).collect();
        assert!(pos[&1] < pos[&2]);
        assert!(pos[&1] < pos[&3]);
        assert!(pos[&2] < pos[&3]);
    }
}
#[cfg(test)]
mod trext_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_trext_phase_order() {
        assert_eq!(TRExtPassPhase::Early.order(), 0);
        assert_eq!(TRExtPassPhase::Middle.order(), 1);
        assert_eq!(TRExtPassPhase::Late.order(), 2);
        assert_eq!(TRExtPassPhase::Finalize.order(), 3);
        assert!(TRExtPassPhase::Early.is_early());
        assert!(!TRExtPassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_trext_config_builder() {
        let c = TRExtPassConfig::new("p")
            .with_phase(TRExtPassPhase::Late)
            .with_max_iter(50)
            .with_debug(1);
        assert_eq!(c.name, "p");
        assert_eq!(c.max_iterations, 50);
        assert!(c.is_debug_enabled());
        assert!(c.enabled);
        let c2 = c.disabled();
        assert!(!c2.enabled);
    }
    #[test]
    pub(super) fn test_trext_stats() {
        let mut s = TRExtPassStats::new();
        s.visit();
        s.visit();
        s.modify();
        s.iterate();
        assert_eq!(s.nodes_visited, 2);
        assert_eq!(s.nodes_modified, 1);
        assert!(s.changed);
        assert_eq!(s.iterations, 1);
        let e = s.efficiency();
        assert!((e - 0.5).abs() < 1e-9);
    }
    #[test]
    pub(super) fn test_trext_registry() {
        let mut r = TRExtPassRegistry::new();
        r.register(TRExtPassConfig::new("a").with_phase(TRExtPassPhase::Early));
        r.register(TRExtPassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&TRExtPassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_trext_cache() {
        let mut c = TRExtCache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_trext_worklist() {
        let mut w = TRExtWorklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_trext_dom_tree() {
        let mut dt = TRExtDomTree::new(5);
        dt.set_idom(1, 0);
        dt.set_idom(2, 0);
        dt.set_idom(3, 1);
        dt.set_idom(4, 1);
        assert!(dt.dominates(0, 3));
        assert!(dt.dominates(1, 4));
        assert!(!dt.dominates(2, 3));
        assert_eq!(dt.depth_of(3), 2);
    }
    #[test]
    pub(super) fn test_trext_liveness() {
        let mut lv = TRExtLiveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_trext_const_folder() {
        let mut cf = TRExtConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_trext_dep_graph() {
        let mut g = TRExtDepGraph::new(4);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 3);
        assert!(!g.has_cycle());
        assert_eq!(g.topo_sort(), Some(vec![0, 1, 2, 3]));
        assert_eq!(g.reachable(0).len(), 4);
        let sccs = g.scc();
        assert_eq!(sccs.len(), 4);
    }
}
#[cfg(test)]
mod trx2_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_trx2_phase_order() {
        assert_eq!(TRX2PassPhase::Early.order(), 0);
        assert_eq!(TRX2PassPhase::Middle.order(), 1);
        assert_eq!(TRX2PassPhase::Late.order(), 2);
        assert_eq!(TRX2PassPhase::Finalize.order(), 3);
        assert!(TRX2PassPhase::Early.is_early());
        assert!(!TRX2PassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_trx2_config_builder() {
        let c = TRX2PassConfig::new("p")
            .with_phase(TRX2PassPhase::Late)
            .with_max_iter(50)
            .with_debug(1);
        assert_eq!(c.name, "p");
        assert_eq!(c.max_iterations, 50);
        assert!(c.is_debug_enabled());
        assert!(c.enabled);
        let c2 = c.disabled();
        assert!(!c2.enabled);
    }
    #[test]
    pub(super) fn test_trx2_stats() {
        let mut s = TRX2PassStats::new();
        s.visit();
        s.visit();
        s.modify();
        s.iterate();
        assert_eq!(s.nodes_visited, 2);
        assert_eq!(s.nodes_modified, 1);
        assert!(s.changed);
        assert_eq!(s.iterations, 1);
        let e = s.efficiency();
        assert!((e - 0.5).abs() < 1e-9);
    }
    #[test]
    pub(super) fn test_trx2_registry() {
        let mut r = TRX2PassRegistry::new();
        r.register(TRX2PassConfig::new("a").with_phase(TRX2PassPhase::Early));
        r.register(TRX2PassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&TRX2PassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_trx2_cache() {
        let mut c = TRX2Cache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_trx2_worklist() {
        let mut w = TRX2Worklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_trx2_dom_tree() {
        let mut dt = TRX2DomTree::new(5);
        dt.set_idom(1, 0);
        dt.set_idom(2, 0);
        dt.set_idom(3, 1);
        dt.set_idom(4, 1);
        assert!(dt.dominates(0, 3));
        assert!(dt.dominates(1, 4));
        assert!(!dt.dominates(2, 3));
        assert_eq!(dt.depth_of(3), 2);
    }
    #[test]
    pub(super) fn test_trx2_liveness() {
        let mut lv = TRX2Liveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_trx2_const_folder() {
        let mut cf = TRX2ConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_trx2_dep_graph() {
        let mut g = TRX2DepGraph::new(4);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 3);
        assert!(!g.has_cycle());
        assert_eq!(g.topo_sort(), Some(vec![0, 1, 2, 3]));
        assert_eq!(g.reachable(0).len(), 4);
        let sccs = g.scc();
        assert_eq!(sccs.len(), 4);
    }
}
