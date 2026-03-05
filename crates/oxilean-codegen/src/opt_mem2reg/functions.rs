//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use std::collections::{HashMap, HashSet};

use super::types::{
    BindingInfo, BindingKind, DominanceFrontier, M2RAnalysisCache, M2RConstantFoldingHelper,
    M2RDepGraph, M2RDominatorTree, M2RExtCache, M2RExtConstFolder, M2RExtDepGraph, M2RExtDomTree,
    M2RExtLiveness, M2RExtPassConfig, M2RExtPassPhase, M2RExtPassRegistry, M2RExtPassStats,
    M2RExtWorklist, M2RLivenessInfo, M2RPassConfig, M2RPassPhase, M2RPassRegistry, M2RPassStats,
    M2RWorklist, M2RX2Cache, M2RX2ConstFolder, M2RX2DepGraph, M2RX2DomTree, M2RX2Liveness,
    M2RX2PassConfig, M2RX2PassPhase, M2RX2PassRegistry, M2RX2PassStats, M2RX2Worklist, Mem2Reg,
    Mem2RegConfig,
};

/// Count how many times each variable is referenced in `expr`.
pub(super) fn count_uses(expr: &LcnfExpr, counts: &mut HashMap<LcnfVarId, usize>) {
    match expr {
        LcnfExpr::Let {
            id, value, body, ..
        } => {
            count_uses_in_value(value, counts);
            count_uses(body, counts);
            let _ = id;
        }
        LcnfExpr::Case {
            scrutinee,
            alts,
            default,
            ..
        } => {
            *counts.entry(*scrutinee).or_insert(0) += 1;
            for alt in alts {
                count_uses(&alt.body, counts);
            }
            if let Some(def) = default {
                count_uses(def, counts);
            }
        }
        LcnfExpr::Return(arg) => count_uses_in_arg(arg, counts),
        LcnfExpr::TailCall(func, args) => {
            count_uses_in_arg(func, counts);
            for a in args {
                count_uses_in_arg(a, counts);
            }
        }
        LcnfExpr::Unreachable => {}
    }
}
pub(super) fn count_uses_in_value(value: &LcnfLetValue, counts: &mut HashMap<LcnfVarId, usize>) {
    match value {
        LcnfLetValue::App(func, args) => {
            count_uses_in_arg(func, counts);
            for a in args {
                count_uses_in_arg(a, counts);
            }
        }
        LcnfLetValue::Proj(_, _, var) => {
            *counts.entry(*var).or_insert(0) += 1;
        }
        LcnfLetValue::Ctor(_, _, args) => {
            for a in args {
                count_uses_in_arg(a, counts);
            }
        }
        LcnfLetValue::Lit(_) => {}
        LcnfLetValue::Erased => {}
        LcnfLetValue::FVar(var) => {
            *counts.entry(*var).or_insert(0) += 1;
        }
        LcnfLetValue::Reset(var) => {
            *counts.entry(*var).or_insert(0) += 1;
        }
        LcnfLetValue::Reuse(slot, _, _, args) => {
            *counts.entry(*slot).or_insert(0) += 1;
            for a in args {
                count_uses_in_arg(a, counts);
            }
        }
    }
}
pub(super) fn count_uses_in_arg(arg: &LcnfArg, counts: &mut HashMap<LcnfVarId, usize>) {
    if let LcnfArg::Var(id) = arg {
        *counts.entry(*id).or_insert(0) += 1;
    }
}
/// Collect all variables defined anywhere in an expression.
pub(super) fn collect_defined(expr: &LcnfExpr, defined: &mut HashSet<LcnfVarId>) {
    match expr {
        LcnfExpr::Let {
            id, value, body, ..
        } => {
            defined.insert(*id);
            collect_defined_in_value(value, defined);
            collect_defined(body, defined);
        }
        LcnfExpr::Case { alts, default, .. } => {
            for alt in alts {
                for p in &alt.params {
                    defined.insert(p.id);
                }
                collect_defined(&alt.body, defined);
            }
            if let Some(def) = default {
                collect_defined(def, defined);
            }
        }
        LcnfExpr::Return(_) | LcnfExpr::Unreachable | LcnfExpr::TailCall(_, _) => {}
    }
}
pub(super) fn collect_defined_in_value(value: &LcnfLetValue, defined: &mut HashSet<LcnfVarId>) {
    let _ = (value, defined);
}
/// Compute dominance frontiers by finding variables that appear defined in
/// some branches of a `Case` but also used outside the case.
pub(super) fn compute_dominance_frontier(expr: &LcnfExpr) -> DominanceFrontier {
    let mut frontier = DominanceFrontier::default();
    compute_frontier_rec(expr, &mut frontier);
    frontier
}
pub(super) fn compute_frontier_rec(expr: &LcnfExpr, frontier: &mut DominanceFrontier) {
    match expr {
        LcnfExpr::Let { value: _, body, .. } => {
            compute_frontier_rec(body, frontier);
        }
        LcnfExpr::Case { alts, default, .. } => {
            let mut branch_defs: Vec<HashSet<LcnfVarId>> = Vec::new();
            for alt in alts {
                let mut defs = HashSet::new();
                for p in &alt.params {
                    defs.insert(p.id);
                }
                collect_defined(&alt.body, &mut defs);
                compute_frontier_rec(&alt.body, frontier);
                branch_defs.push(defs);
            }
            if let Some(def) = default {
                let mut defs = HashSet::new();
                collect_defined(def, &mut defs);
                compute_frontier_rec(def, frontier);
                branch_defs.push(defs);
            }
            if branch_defs.len() > 1 {
                let all_defs: HashSet<LcnfVarId> = branch_defs.iter().flatten().cloned().collect();
                for var in all_defs {
                    let defined_in_all = branch_defs.iter().all(|d| d.contains(&var));
                    if !defined_in_all {
                        frontier.join_vars.insert(var);
                    }
                }
            }
        }
        LcnfExpr::Return(_) | LcnfExpr::Unreachable | LcnfExpr::TailCall(_, _) => {}
    }
}
/// Substitute all occurrences of `from` with `to` in `expr`.
pub(super) fn substitute_var(expr: LcnfExpr, from: LcnfVarId, to: LcnfArg) -> LcnfExpr {
    match expr {
        LcnfExpr::Let {
            id,
            name,
            ty,
            value,
            body,
        } => {
            let value2 = subst_in_value(value, from, &to);
            let body2 = substitute_var(*body, from, to);
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
            let scrutinee2 = if scrutinee == from {
                match &to {
                    LcnfArg::Var(v) => *v,
                    _ => scrutinee,
                }
            } else {
                scrutinee
            };
            let alts2 = alts
                .into_iter()
                .map(|alt| LcnfAlt {
                    ctor_name: alt.ctor_name,
                    ctor_tag: alt.ctor_tag,
                    params: alt.params,
                    body: substitute_var(alt.body, from, to.clone()),
                })
                .collect();
            let default2 = default.map(|d| Box::new(substitute_var(*d, from, to)));
            LcnfExpr::Case {
                scrutinee: scrutinee2,
                scrutinee_ty,
                alts: alts2,
                default: default2,
            }
        }
        LcnfExpr::Return(arg) => LcnfExpr::Return(subst_in_arg(arg, from, &to)),
        LcnfExpr::TailCall(func, args) => {
            let func2 = subst_in_arg(func, from, &to);
            let args2 = args
                .into_iter()
                .map(|a| subst_in_arg(a, from, &to))
                .collect();
            LcnfExpr::TailCall(func2, args2)
        }
        LcnfExpr::Unreachable => LcnfExpr::Unreachable,
    }
}
pub(super) fn subst_in_arg(arg: LcnfArg, from: LcnfVarId, to: &LcnfArg) -> LcnfArg {
    match &arg {
        LcnfArg::Var(id) if *id == from => to.clone(),
        _ => arg,
    }
}
pub(super) fn subst_in_value(value: LcnfLetValue, from: LcnfVarId, to: &LcnfArg) -> LcnfLetValue {
    match value {
        LcnfLetValue::App(func, args) => {
            let func2 = subst_in_arg(func, from, to);
            let args2 = args
                .into_iter()
                .map(|a| subst_in_arg(a, from, to))
                .collect();
            LcnfLetValue::App(func2, args2)
        }
        LcnfLetValue::Proj(name, idx, var) => {
            let var2 = if var == from {
                match to {
                    LcnfArg::Var(v) => *v,
                    _ => var,
                }
            } else {
                var
            };
            LcnfLetValue::Proj(name, idx, var2)
        }
        LcnfLetValue::Ctor(name, tag, args) => {
            let args2 = args
                .into_iter()
                .map(|a| subst_in_arg(a, from, to))
                .collect();
            LcnfLetValue::Ctor(name, tag, args2)
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
            let var2 = if var == from {
                match to {
                    LcnfArg::Var(v) => *v,
                    _ => var,
                }
            } else {
                var
            };
            LcnfLetValue::Reset(var2)
        }
        LcnfLetValue::Reuse(slot, name, tag, args) => {
            let slot2 = if slot == from {
                match to {
                    LcnfArg::Var(v) => *v,
                    _ => slot,
                }
            } else {
                slot
            };
            let args2 = args
                .into_iter()
                .map(|a| subst_in_arg(a, from, to))
                .collect();
            LcnfLetValue::Reuse(slot2, name, tag, args2)
        }
        other => other,
    }
}
/// Returns `true` if a let-value is promotable (pure, no memory side effects).
pub(super) fn is_promotable(value: &LcnfLetValue) -> bool {
    match value {
        LcnfLetValue::Lit(_) => true,
        LcnfLetValue::FVar(_) => true,
        LcnfLetValue::Erased => true,
        LcnfLetValue::Ctor(_, _, _) => true,
        LcnfLetValue::Proj(_, _, _) => true,
        LcnfLetValue::App(_, _) => false,
        LcnfLetValue::Reset(_) => false,
        LcnfLetValue::Reuse(_, _, _, _) => false,
    }
}
/// Returns `true` if the value is a "trivial" scalar (literal or single var
/// alias), the cheapest class to inline at every use site.
pub(super) fn is_trivial(value: &LcnfLetValue) -> bool {
    matches!(
        value,
        LcnfLetValue::Lit(_) | LcnfLetValue::FVar(_) | LcnfLetValue::Erased
    )
}
/// Convert a promotable `LcnfLetValue` into an `LcnfArg` for substitution,
/// returning `None` for values that have no direct `LcnfArg` representation.
pub(super) fn let_value_to_arg(value: &LcnfLetValue) -> Option<LcnfArg> {
    match value {
        LcnfLetValue::Lit(l) => Some(LcnfArg::Lit(l.clone())),
        LcnfLetValue::FVar(v) => Some(LcnfArg::Var(*v)),
        LcnfLetValue::Erased => Some(LcnfArg::Erased),
        _ => None,
    }
}
/// Collect binding information recursively.
pub(super) fn collect_binding_info(
    expr: &LcnfExpr,
    bindings: &mut HashMap<LcnfVarId, BindingInfo>,
    use_counts: &HashMap<LcnfVarId, usize>,
    frontier: &DominanceFrontier,
    depth: usize,
) {
    match expr {
        LcnfExpr::Let {
            id,
            ty,
            value,
            body,
            ..
        } => {
            let kind = if frontier.join_vars.contains(id) {
                BindingKind::MayJoin
            } else {
                match value {
                    LcnfLetValue::Reset(_) | LcnfLetValue::Reuse(_, _, _, _) => {
                        BindingKind::MemoryOp
                    }
                    _ => BindingKind::Immutable,
                }
            };
            let use_count = *use_counts.get(id).unwrap_or(&0);
            bindings.insert(
                *id,
                BindingInfo {
                    kind,
                    value: value.clone(),
                    ty: ty.clone(),
                    use_count,
                    depth,
                },
            );
            collect_binding_info(body, bindings, use_counts, frontier, depth + 1);
        }
        LcnfExpr::Case { alts, default, .. } => {
            for alt in alts {
                collect_binding_info(&alt.body, bindings, use_counts, frontier, depth + 1);
            }
            if let Some(def) = default {
                collect_binding_info(def, bindings, use_counts, frontier, depth + 1);
            }
        }
        LcnfExpr::Return(_) | LcnfExpr::Unreachable | LcnfExpr::TailCall(_, _) => {}
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::lcnf::{
        LcnfExpr, LcnfFunDecl, LcnfLetValue, LcnfLit, LcnfParam, LcnfType, LcnfVarId,
    };
    pub(super) fn make_decl(body: LcnfExpr) -> LcnfFunDecl {
        LcnfFunDecl {
            name: "test_fn".to_string(),
            original_name: None,
            params: vec![],
            ret_type: LcnfType::Nat,
            body,
            is_recursive: false,
            is_lifted: false,
            inline_cost: 1,
        }
    }
    /// `let x = 42; return x`  →  `return 42`
    #[test]
    pub(super) fn test_promote_literal_binding() {
        let body = LcnfExpr::Let {
            id: LcnfVarId(0),
            name: "x".to_string(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::Lit(LcnfLit::Nat(42)),
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(LcnfVarId(0)))),
        };
        let mut decl = make_decl(body);
        let mut pass = Mem2Reg::new(Mem2RegConfig::default());
        pass.run(&mut decl);
        assert_eq!(decl.body, LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(42))));
        assert_eq!(pass.report().bindings_promoted, 1);
    }
    /// `let x = fvar(y); return x`  →  `return y`
    #[test]
    pub(super) fn test_promote_fvar_binding() {
        let body = LcnfExpr::Let {
            id: LcnfVarId(1),
            name: "x".to_string(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::FVar(LcnfVarId(0)),
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(LcnfVarId(1)))),
        };
        let mut decl = make_decl(body);
        let mut pass = Mem2Reg::new(Mem2RegConfig::default());
        pass.run(&mut decl);
        assert_eq!(decl.body, LcnfExpr::Return(LcnfArg::Var(LcnfVarId(0))));
        assert_eq!(pass.report().bindings_promoted, 1);
    }
    /// Memory ops (Reset) must NOT be promoted.
    #[test]
    pub(super) fn test_no_promote_memory_op() {
        let body = LcnfExpr::Let {
            id: LcnfVarId(1),
            name: "slot".to_string(),
            ty: LcnfType::Object,
            value: LcnfLetValue::Reset(LcnfVarId(0)),
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(LcnfVarId(1)))),
        };
        let mut decl = make_decl(body);
        let mut pass = Mem2Reg::new(Mem2RegConfig::default());
        pass.run(&mut decl);
        assert!(matches!(decl.body, LcnfExpr::Let { .. }));
        assert_eq!(pass.report().bindings_promoted, 0);
    }
    /// App bindings are NOT promotable (may have side effects).
    #[test]
    pub(super) fn test_no_promote_app() {
        let body = LcnfExpr::Let {
            id: LcnfVarId(1),
            name: "r".to_string(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::App(LcnfArg::Var(LcnfVarId(0)), vec![]),
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(LcnfVarId(1)))),
        };
        let mut decl = make_decl(body);
        let mut pass = Mem2Reg::new(Mem2RegConfig::default());
        pass.run(&mut decl);
        assert!(matches!(decl.body, LcnfExpr::Let { .. }));
        assert_eq!(pass.report().bindings_promoted, 0);
    }
    /// Conservative mode: erased bindings are still trivial and should be promoted.
    #[test]
    pub(super) fn test_conservative_promotes_trivial() {
        let body = LcnfExpr::Let {
            id: LcnfVarId(0),
            name: "e".to_string(),
            ty: LcnfType::Erased,
            value: LcnfLetValue::Erased,
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(LcnfVarId(0)))),
        };
        let mut decl = make_decl(body);
        let mut pass = Mem2Reg::new(Mem2RegConfig {
            conservative: true,
            ..Default::default()
        });
        pass.run(&mut decl);
        assert_eq!(decl.body, LcnfExpr::Return(LcnfArg::Erased));
        assert_eq!(pass.report().bindings_promoted, 1);
    }
    /// Chain: `let x = 1; let y = x; return y`  →  `return 1`
    #[test]
    pub(super) fn test_promote_chain() {
        let body = LcnfExpr::Let {
            id: LcnfVarId(0),
            name: "x".to_string(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::Lit(LcnfLit::Nat(1)),
            body: Box::new(LcnfExpr::Let {
                id: LcnfVarId(1),
                name: "y".to_string(),
                ty: LcnfType::Nat,
                value: LcnfLetValue::FVar(LcnfVarId(0)),
                body: Box::new(LcnfExpr::Return(LcnfArg::Var(LcnfVarId(1)))),
            }),
        };
        let mut decl = make_decl(body);
        let mut pass = Mem2Reg::new(Mem2RegConfig::default());
        pass.run(&mut decl);
        assert_eq!(decl.body, LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(1))));
        assert_eq!(pass.report().bindings_promoted, 2);
    }
    /// Dominance frontier computation: variables defined only in one branch
    /// are reported as join candidates.
    #[test]
    pub(super) fn test_dominance_frontier() {
        let case_expr = LcnfExpr::Case {
            scrutinee: LcnfVarId(0),
            scrutinee_ty: LcnfType::Object,
            alts: vec![LcnfAlt {
                ctor_name: "A".to_string(),
                ctor_tag: 0,
                params: vec![LcnfParam {
                    id: LcnfVarId(1),
                    name: "p".to_string(),
                    ty: LcnfType::Nat,
                    erased: false,
                    borrowed: false,
                }],
                body: LcnfExpr::Return(LcnfArg::Var(LcnfVarId(1))),
            }],
            default: Some(Box::new(LcnfExpr::Return(LcnfArg::Var(LcnfVarId(0))))),
        };
        let frontier = compute_dominance_frontier(&case_expr);
        assert!(frontier.join_vars.contains(&LcnfVarId(1)));
    }
    /// `Mem2Reg::default_pass()` constructor smoke test.
    #[test]
    pub(super) fn test_default_pass_smoke() {
        let body = LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0)));
        let mut decl = make_decl(body);
        let mut pass = Mem2Reg::default_pass();
        pass.run(&mut decl);
        assert_eq!(pass.report().bindings_promoted, 0);
    }
}
#[cfg(test)]
mod M2R_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = M2RPassConfig::new("test_pass", M2RPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = M2RPassStats::new();
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
        let mut reg = M2RPassRegistry::new();
        reg.register(M2RPassConfig::new("pass_a", M2RPassPhase::Analysis));
        reg.register(M2RPassConfig::new("pass_b", M2RPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = M2RAnalysisCache::new(10);
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
        let mut wl = M2RWorklist::new();
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
        let mut dt = M2RDominatorTree::new(5);
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
        let mut liveness = M2RLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(M2RConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(M2RConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(M2RConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            M2RConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(M2RConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = M2RDepGraph::new();
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
mod m2rext_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_m2rext_phase_order() {
        assert_eq!(M2RExtPassPhase::Early.order(), 0);
        assert_eq!(M2RExtPassPhase::Middle.order(), 1);
        assert_eq!(M2RExtPassPhase::Late.order(), 2);
        assert_eq!(M2RExtPassPhase::Finalize.order(), 3);
        assert!(M2RExtPassPhase::Early.is_early());
        assert!(!M2RExtPassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_m2rext_config_builder() {
        let c = M2RExtPassConfig::new("p")
            .with_phase(M2RExtPassPhase::Late)
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
    pub(super) fn test_m2rext_stats() {
        let mut s = M2RExtPassStats::new();
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
    pub(super) fn test_m2rext_registry() {
        let mut r = M2RExtPassRegistry::new();
        r.register(M2RExtPassConfig::new("a").with_phase(M2RExtPassPhase::Early));
        r.register(M2RExtPassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&M2RExtPassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_m2rext_cache() {
        let mut c = M2RExtCache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_m2rext_worklist() {
        let mut w = M2RExtWorklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_m2rext_dom_tree() {
        let mut dt = M2RExtDomTree::new(5);
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
    pub(super) fn test_m2rext_liveness() {
        let mut lv = M2RExtLiveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_m2rext_const_folder() {
        let mut cf = M2RExtConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_m2rext_dep_graph() {
        let mut g = M2RExtDepGraph::new(4);
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
mod m2rx2_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_m2rx2_phase_order() {
        assert_eq!(M2RX2PassPhase::Early.order(), 0);
        assert_eq!(M2RX2PassPhase::Middle.order(), 1);
        assert_eq!(M2RX2PassPhase::Late.order(), 2);
        assert_eq!(M2RX2PassPhase::Finalize.order(), 3);
        assert!(M2RX2PassPhase::Early.is_early());
        assert!(!M2RX2PassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_m2rx2_config_builder() {
        let c = M2RX2PassConfig::new("p")
            .with_phase(M2RX2PassPhase::Late)
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
    pub(super) fn test_m2rx2_stats() {
        let mut s = M2RX2PassStats::new();
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
    pub(super) fn test_m2rx2_registry() {
        let mut r = M2RX2PassRegistry::new();
        r.register(M2RX2PassConfig::new("a").with_phase(M2RX2PassPhase::Early));
        r.register(M2RX2PassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&M2RX2PassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_m2rx2_cache() {
        let mut c = M2RX2Cache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_m2rx2_worklist() {
        let mut w = M2RX2Worklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_m2rx2_dom_tree() {
        let mut dt = M2RX2DomTree::new(5);
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
    pub(super) fn test_m2rx2_liveness() {
        let mut lv = M2RX2Liveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_m2rx2_const_folder() {
        let mut cf = M2RX2ConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_m2rx2_dep_graph() {
        let mut g = M2RX2DepGraph::new(4);
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
