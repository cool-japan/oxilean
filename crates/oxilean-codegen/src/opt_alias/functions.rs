//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::{LcnfArg, LcnfExpr, LcnfFunDecl, LcnfLetValue, LcnfType, LcnfVarId};
use std::collections::{HashMap, HashSet, VecDeque};

use super::types::{
    AliasAnalysisLevel, AliasConfigExt, AliasFeatureFlags, AliasPass, AliasReport, AliasResult,
    AliasResultExt, AliasSet, AliasStatsExt, AliasVersionInfo, AndersenSolver, LoadStoreForwarder,
    MemoryAccessInfo, PointsToGraph, PointsToNode,
};

/// Returns `true` if two LCNF types are known to be non-aliasing.
///
/// In a type-safe language like Lean, values of different unrelated types
/// cannot alias (unless they are both `Object` or erased).
pub fn tbaa_no_alias(ty_a: &LcnfType, ty_b: &LcnfType) -> bool {
    match (ty_a, ty_b) {
        (a, b) if a == b => false,
        (LcnfType::Erased, _) | (_, LcnfType::Erased) => false,
        (LcnfType::Object, _) | (_, LcnfType::Object) => false,
        (LcnfType::Irrelevant, _) | (_, LcnfType::Irrelevant) => false,
        (LcnfType::Nat, LcnfType::LcnfString) | (LcnfType::LcnfString, LcnfType::Nat) => true,
        (LcnfType::Ctor(a, _), LcnfType::Ctor(b, _)) if a != b => true,
        (LcnfType::Fun(_, _), LcnfType::Nat) | (LcnfType::Nat, LcnfType::Fun(_, _)) => true,
        (LcnfType::Fun(_, _), LcnfType::LcnfString)
        | (LcnfType::LcnfString, LcnfType::Fun(_, _)) => true,
        _ => false,
    }
}
/// Recursively apply load-store forwarding to an expression.
pub(super) fn apply_forwarding_to_expr(expr: &mut LcnfExpr, fwd: &mut LoadStoreForwarder) {
    match expr {
        LcnfExpr::Let { value, body, .. } => {
            if let LcnfLetValue::App(_, _) = value {
                fwd.clear();
            }
            apply_forwarding_to_expr(body, fwd);
        }
        LcnfExpr::Case { alts, default, .. } => {
            let snap = fwd.store_cache.clone();
            for alt in alts.iter_mut() {
                fwd.store_cache = snap.clone();
                apply_forwarding_to_expr(&mut alt.body, fwd);
            }
            if let Some(def) = default {
                fwd.store_cache = snap;
                apply_forwarding_to_expr(def, fwd);
            }
        }
        LcnfExpr::Return(_) | LcnfExpr::TailCall(_, _) | LcnfExpr::Unreachable => {}
    }
}
/// Compute the transitive closure of the copy edges in the points-to graph.
#[allow(clippy::too_many_arguments)]
pub fn transitive_pts_closure(graph: &mut PointsToGraph) {
    let vars: Vec<LcnfVarId> = graph.nodes.keys().copied().collect();
    let mut worklist: VecDeque<LcnfVarId> = vars.into_iter().collect();
    while let Some(v) = worklist.pop_front() {
        let pts: Vec<LcnfVarId> = graph.pts_of(v).clone().into_iter().collect();
        for tgt in pts {
            let pts_tgt: HashSet<LcnfVarId> = graph.pts_of(tgt).clone();
            let pts_v = graph.pts.entry(v).or_default();
            let old_len = pts_v.len();
            pts_v.extend(pts_tgt);
            if pts_v.len() != old_len {
                worklist.push_back(v);
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::lcnf::{LcnfExpr, LcnfFunDecl, LcnfLetValue, LcnfParam, LcnfType, LcnfVarId};
    pub(super) fn v(n: u64) -> LcnfVarId {
        LcnfVarId(n)
    }
    pub(super) fn make_decl(name: &str, body: LcnfExpr) -> LcnfFunDecl {
        LcnfFunDecl {
            name: name.to_string(),
            original_name: None,
            params: vec![],
            ret_type: LcnfType::Nat,
            body,
            is_recursive: false,
            is_lifted: false,
            inline_cost: 1,
        }
    }
    pub(super) fn make_param(n: u64, name: &str) -> LcnfParam {
        LcnfParam {
            id: v(n),
            name: name.to_string(),
            ty: LcnfType::Nat,
            erased: false,
            borrowed: false,
        }
    }
    pub(super) fn ret_var(n: u64) -> LcnfExpr {
        LcnfExpr::Return(LcnfArg::Var(v(n)))
    }
    #[test]
    pub(super) fn test_alias_result_may_alias_must() {
        assert!(AliasResult::MustAlias.may_alias());
    }
    #[test]
    pub(super) fn test_alias_result_may_alias_may() {
        assert!(AliasResult::MayAlias.may_alias());
    }
    #[test]
    pub(super) fn test_alias_result_may_alias_no() {
        assert!(!AliasResult::NoAlias.may_alias());
    }
    #[test]
    pub(super) fn test_alias_result_must_alias() {
        assert!(AliasResult::MustAlias.must_alias());
        assert!(!AliasResult::MayAlias.must_alias());
    }
    #[test]
    pub(super) fn test_alias_result_no_alias() {
        assert!(AliasResult::NoAlias.no_alias());
        assert!(!AliasResult::MustAlias.no_alias());
    }
    #[test]
    pub(super) fn test_alias_result_merge() {
        assert_eq!(
            AliasResult::MustAlias.merge(AliasResult::MustAlias),
            AliasResult::MustAlias
        );
        assert_eq!(
            AliasResult::NoAlias.merge(AliasResult::NoAlias),
            AliasResult::NoAlias
        );
        assert_eq!(
            AliasResult::MustAlias.merge(AliasResult::NoAlias),
            AliasResult::MayAlias
        );
    }
    #[test]
    pub(super) fn test_alias_set_singleton() {
        let s = AliasSet::singleton(v(1));
        assert!(s.contains(v(1)));
        assert_eq!(s.len(), 1);
        assert_eq!(s.representative, Some(v(1)));
    }
    #[test]
    pub(super) fn test_alias_set_insert() {
        let mut s = AliasSet::new();
        s.insert(v(1));
        s.insert(v(2));
        assert!(s.contains(v(1)));
        assert!(s.contains(v(2)));
        assert_eq!(s.len(), 2);
    }
    #[test]
    pub(super) fn test_alias_set_merge() {
        let mut a = AliasSet::singleton(v(1));
        let b = AliasSet::singleton(v(2));
        a.merge_with(&b);
        assert!(a.contains(v(1)));
        assert!(a.contains(v(2)));
    }
    #[test]
    pub(super) fn test_points_to_node_local() {
        let n = PointsToNode::local(v(5), "x");
        assert!(n.is_local());
        assert!(!n.is_heap());
    }
    #[test]
    pub(super) fn test_points_to_node_heap() {
        let n = PointsToNode::heap(v(7), "alloc");
        assert!(n.is_heap());
        assert!(!n.is_local());
    }
    #[test]
    pub(super) fn test_points_to_graph_add_pts() {
        let mut g = PointsToGraph::new();
        g.add_node(PointsToNode::local(v(1), "a"));
        g.add_node(PointsToNode::heap(v(2), "b"));
        g.add_pts(v(1), v(2));
        assert!(g.pts_of(v(1)).contains(&v(2)));
    }
    #[test]
    pub(super) fn test_points_to_graph_intersects() {
        let mut g = PointsToGraph::new();
        g.add_pts(v(1), v(3));
        g.add_pts(v(2), v(3));
        assert!(g.intersects(v(1), v(2)));
    }
    #[test]
    pub(super) fn test_points_to_graph_no_intersect() {
        let mut g = PointsToGraph::new();
        g.add_pts(v(1), v(3));
        g.add_pts(v(2), v(4));
        assert!(!g.intersects(v(1), v(2)));
    }
    #[test]
    pub(super) fn test_andersen_address_of() {
        let mut solver = AndersenSolver::new();
        solver.register_var(PointsToNode::local(v(1), "x"));
        solver.register_var(PointsToNode::heap(v(2), "alloc"));
        solver.add_address_of(v(1), v(2));
        solver.solve();
        assert!(solver.graph.pts_of(v(1)).contains(&v(2)));
    }
    #[test]
    pub(super) fn test_andersen_copy_propagates() {
        let mut solver = AndersenSolver::new();
        solver.register_var(PointsToNode::local(v(1), "x"));
        solver.register_var(PointsToNode::local(v(2), "y"));
        solver.register_var(PointsToNode::heap(v(3), "alloc"));
        solver.add_address_of(v(1), v(3));
        solver.add_copy(v(2), v(1));
        solver.solve();
        assert!(solver.graph.pts_of(v(2)).contains(&v(3)));
    }
    #[test]
    pub(super) fn test_andersen_same_var_must_alias() {
        let mut solver = AndersenSolver::new();
        solver.register_var(PointsToNode::local(v(1), "x"));
        solver.add_address_of(v(1), v(1));
        solver.solve();
        assert_eq!(solver.query(v(1), v(1)), AliasResult::MustAlias);
    }
    #[test]
    pub(super) fn test_andersen_distinct_allocs_no_alias() {
        let mut solver = AndersenSolver::new();
        solver.register_var(PointsToNode::local(v(1), "x"));
        solver.register_var(PointsToNode::local(v(2), "y"));
        solver.add_address_of(v(1), v(1));
        solver.add_address_of(v(2), v(2));
        solver.solve();
        assert_eq!(solver.query(v(1), v(2)), AliasResult::NoAlias);
    }
    #[test]
    pub(super) fn test_tbaa_nat_string_no_alias() {
        assert!(tbaa_no_alias(&LcnfType::Nat, &LcnfType::LcnfString));
    }
    #[test]
    pub(super) fn test_tbaa_same_type_may_alias() {
        assert!(!tbaa_no_alias(&LcnfType::Nat, &LcnfType::Nat));
    }
    #[test]
    pub(super) fn test_tbaa_different_ctors_no_alias() {
        assert!(tbaa_no_alias(
            &LcnfType::Ctor("List".to_string(), vec![]),
            &LcnfType::Ctor("Option".to_string(), vec![])
        ));
    }
    #[test]
    pub(super) fn test_tbaa_object_conservative() {
        assert!(!tbaa_no_alias(&LcnfType::Object, &LcnfType::Nat));
    }
    #[test]
    pub(super) fn test_memory_access_definitely_disjoint() {
        let a = MemoryAccessInfo {
            base: v(1),
            offset: Some(0),
            size: Some(4),
            is_volatile: false,
            access_type: LcnfType::Nat,
            is_write: false,
        };
        let b = MemoryAccessInfo {
            base: v(1),
            offset: Some(8),
            size: Some(4),
            is_volatile: false,
            access_type: LcnfType::Nat,
            is_write: false,
        };
        assert!(a.definitely_disjoint(&b));
    }
    #[test]
    pub(super) fn test_memory_access_overlapping() {
        let a = MemoryAccessInfo {
            base: v(1),
            offset: Some(0),
            size: Some(8),
            is_volatile: false,
            access_type: LcnfType::Nat,
            is_write: false,
        };
        let b = MemoryAccessInfo {
            base: v(1),
            offset: Some(4),
            size: Some(4),
            is_volatile: false,
            access_type: LcnfType::Nat,
            is_write: false,
        };
        assert!(!a.definitely_disjoint(&b));
    }
    #[test]
    pub(super) fn test_alias_pass_basic_aa_same_var() {
        let mut pass = AliasPass::with_level(AliasAnalysisLevel::BasicAA);
        let mut decls = vec![];
        pass.run(&mut decls);
        assert_eq!(pass.query(v(1), v(1)), AliasResult::MustAlias);
    }
    #[test]
    pub(super) fn test_alias_pass_basic_aa_diff_var() {
        let mut pass = AliasPass::with_level(AliasAnalysisLevel::BasicAA);
        let mut decls = vec![];
        pass.run(&mut decls);
        assert_eq!(pass.query(v(1), v(2)), AliasResult::MayAlias);
    }
    #[test]
    pub(super) fn test_alias_pass_no_alias_level() {
        let mut pass = AliasPass::with_level(AliasAnalysisLevel::NoAlias);
        let mut decls = vec![];
        pass.run(&mut decls);
        assert_eq!(pass.query(v(1), v(1)), AliasResult::MayAlias);
    }
    #[test]
    pub(super) fn test_alias_pass_report_counters() {
        let mut pass = AliasPass::with_level(AliasAnalysisLevel::BasicAA);
        let mut decls = vec![];
        pass.run(&mut decls);
        pass.query(v(1), v(1));
        pass.query(v(1), v(2));
        let r = pass.report();
        assert_eq!(r.must_alias, 1);
        assert_eq!(r.may_alias, 1);
        assert_eq!(r.pairs_analyzed, 2);
    }
    #[test]
    pub(super) fn test_alias_pass_run_with_decls() {
        let body = LcnfExpr::Let {
            id: v(10),
            name: "x".to_string(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::Ctor("Nat.zero".to_string(), 0, vec![]),
            body: Box::new(ret_var(10)),
        };
        let decl = make_decl("test_fn", body);
        let mut decls = vec![decl];
        let mut pass = AliasPass::new();
        pass.run(&mut decls);
        assert_eq!(pass.query(v(10), v(10)), AliasResult::MustAlias);
    }
    #[test]
    pub(super) fn test_alias_pass_tbaa_no_alias() {
        let mut pass = AliasPass::with_level(AliasAnalysisLevel::TypeBasedAA);
        pass.type_map.insert(v(1), LcnfType::Nat);
        pass.type_map.insert(v(2), LcnfType::LcnfString);
        let mut decls = vec![];
        pass.run(&mut decls);
        assert_eq!(pass.query(v(1), v(2)), AliasResult::NoAlias);
    }
    #[test]
    pub(super) fn test_alias_pass_cfl_with_params() {
        let body = ret_var(0);
        let mut decl = make_decl("f", body);
        decl.params = vec![make_param(0, "n"), make_param(1, "m")];
        let mut decls = vec![decl];
        let mut pass = AliasPass::new();
        pass.run(&mut decls);
        let r = pass.query(v(0), v(1));
        assert!(matches!(r, AliasResult::NoAlias | AliasResult::MayAlias));
    }
    #[test]
    pub(super) fn test_alias_report_no_alias_ratio() {
        let mut r = AliasReport::default();
        r.pairs_analyzed = 10;
        r.no_alias = 7;
        assert!((r.no_alias_ratio() - 0.7).abs() < 1e-6);
    }
    #[test]
    pub(super) fn test_alias_report_empty() {
        let r = AliasReport::default();
        assert_eq!(r.no_alias_ratio(), 0.0);
    }
    #[test]
    pub(super) fn test_load_store_forwarder_basic() {
        let mut fwd = LoadStoreForwarder::new();
        fwd.record_store(v(1), Some(0), v(5));
        assert_eq!(fwd.forward_load(v(1), Some(0)), Some(v(5)));
    }
    #[test]
    pub(super) fn test_load_store_forwarder_miss() {
        let fwd = LoadStoreForwarder::new();
        assert_eq!(fwd.forward_load(v(1), Some(0)), None);
    }
    #[test]
    pub(super) fn test_load_store_forwarder_invalidate() {
        let mut fwd = LoadStoreForwarder::new();
        fwd.record_store(v(1), Some(0), v(5));
        fwd.invalidate(v(1));
        assert_eq!(fwd.forward_load(v(1), Some(0)), None);
    }
    #[test]
    pub(super) fn test_alias_analysis_level_description() {
        assert!(!AliasAnalysisLevel::CFLAndersen.description().is_empty());
        assert!(!AliasAnalysisLevel::NoAlias.description().is_empty());
    }
    #[test]
    pub(super) fn test_alias_analysis_level_uses_points_to() {
        assert!(AliasAnalysisLevel::CFLAndersen.uses_points_to());
        assert!(AliasAnalysisLevel::CFLSteensgaard.uses_points_to());
        assert!(!AliasAnalysisLevel::BasicAA.uses_points_to());
        assert!(!AliasAnalysisLevel::TypeBasedAA.uses_points_to());
    }
}
/// Alias analysis diagnostic helper
#[allow(dead_code)]
pub fn alias_result_str(r: &AliasResultExt) -> &'static str {
    match r {
        AliasResultExt::NoAlias => "NoAlias",
        AliasResultExt::MayAlias => "MayAlias",
        AliasResultExt::PartialAlias => "PartialAlias",
        AliasResultExt::MustAlias => "MustAlias",
    }
}
/// Alias analysis is_must_alias shorthand
#[allow(dead_code)]
pub fn alias_is_must(r: &AliasResultExt) -> bool {
    *r == AliasResultExt::MustAlias
}
/// Alias analysis is_no_alias shorthand
#[allow(dead_code)]
pub fn alias_is_no(r: &AliasResultExt) -> bool {
    *r == AliasResultExt::NoAlias
}
/// Alias analysis version info default
#[allow(dead_code)]
pub fn alias_default_version() -> AliasVersionInfo {
    AliasVersionInfo::default()
}
/// Alias analysis final code stats helper
#[allow(dead_code)]
pub fn alias_stats_summary(stats: &AliasStatsExt) -> String {
    format!(
        "queries={}, must={:.1}%, no={:.1}%",
        stats.queries_total,
        if stats.queries_total > 0 {
            stats.must_alias_count as f64 / stats.queries_total as f64 * 100.0
        } else {
            0.0
        },
        if stats.queries_total > 0 {
            stats.no_alias_count as f64 / stats.queries_total as f64 * 100.0
        } else {
            0.0
        },
    )
}
/// Default alias feature flags
#[allow(dead_code)]
pub fn alias_default_features() -> AliasFeatureFlags {
    AliasFeatureFlags {
        enable_tbaa: true,
        enable_field_sensitivity: false,
        enable_flow_sensitivity: false,
        enable_context_sensitivity: false,
        enable_escape_analysis: true,
        enable_cfl_reachability: false,
    }
}
/// Alias analysis config from feature flags
#[allow(dead_code)]
pub fn alias_config_from_features(flags: &AliasFeatureFlags) -> AliasConfigExt {
    AliasConfigExt {
        enable_field_sensitivity: flags.enable_field_sensitivity,
        enable_flow_sensitivity: flags.enable_flow_sensitivity,
        enable_context_sensitivity: flags.enable_context_sensitivity,
        track_heap: flags.enable_escape_analysis,
        ..Default::default()
    }
}
/// Print alias config summary
#[allow(dead_code)]
pub fn alias_config_summary(cfg: &AliasConfigExt) -> String {
    format!(
        "AliasConfig {{ level={}, field={}, flow={}, ctx={} }}",
        cfg.level,
        cfg.enable_field_sensitivity,
        cfg.enable_flow_sensitivity,
        cfg.enable_context_sensitivity
    )
}
/// Alias analysis version string
#[allow(dead_code)]
pub const ALIAS_PASS_VERSION: &str = "2.0.0";
/// Alias default max iterations
#[allow(dead_code)]
pub const ALIAS_MAX_ITERS: usize = 100;
/// Andersen pass name
#[allow(dead_code)]
pub const ANDERSEN_PASS: &str = "andersen-points-to";
/// Steensgaard pass name
#[allow(dead_code)]
pub const STEENSGAARD_PASS: &str = "steensgaard-points-to";
/// CFL-Andersen pass name
#[allow(dead_code)]
pub const CFL_ANDERSEN_PASS: &str = "cfl-andersen-points-to";
/// Alias analysis pass name constants
#[allow(dead_code)]
pub const ALIAS_PASS_NAME_BASIC: &str = "basic-aa";
#[allow(dead_code)]
pub const ALIAS_PASS_NAME_TBAA: &str = "type-based-aa";
#[allow(dead_code)]
pub const ALIAS_PASS_NAME_SCOPED: &str = "scoped-noalias";
#[allow(dead_code)]
pub const ALIAS_PASS_NAME_GLOBALS: &str = "globals-aa";
#[allow(dead_code)]
pub const ALIAS_PASS_NAME_CFL: &str = "cfl-steensgaard-aa";
/// Default alias pass for oxilean
#[allow(dead_code)]
pub const OXILEAN_DEFAULT_ALIAS_PASS: &str = "andersen-points-to";
/// Alias analysis pipeline default passes
#[allow(dead_code)]
pub fn alias_default_pipeline_passes() -> Vec<&'static str> {
    vec![
        ALIAS_PASS_NAME_BASIC,
        ALIAS_PASS_NAME_TBAA,
        ALIAS_PASS_NAME_GLOBALS,
    ]
}
/// Alias analysis result ordering (most certain first)
#[allow(dead_code)]
pub fn alias_certainty_order(r: &AliasResultExt) -> u32 {
    match r {
        AliasResultExt::MustAlias => 3,
        AliasResultExt::NoAlias => 2,
        AliasResultExt::PartialAlias => 1,
        AliasResultExt::MayAlias => 0,
    }
}
/// Alias version 2.0 marker constant
#[allow(dead_code)]
pub const ALIAS_V2: bool = true;
