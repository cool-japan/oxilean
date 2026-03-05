//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use std::collections::{HashMap, HashSet};

use super::types::{
    BorrowInfo, InPlaceUpdate, LayoutInfo, LifetimeAnalyzer, ORAnalysisCache,
    ORConstantFoldingHelper, ORDepGraph, ORDominatorTree, ORLivenessInfo, ORPassConfig,
    ORPassPhase, ORPassRegistry, ORPassStats, ORWorklist, Ownership, RcElimInfo, RcElimKind,
    RcElimReason, ReuseAllocLog, ReuseAnalyzer, ReuseConfig, ReuseMemSizeClass, ReuseOpportunity,
    ReuseStats,
};

/// Compute layout information from constructor arguments
pub(super) fn compute_layout(ctor_name: &str, ctor_tag: u32, arg_types: &[LcnfType]) -> LayoutInfo {
    let mut obj_fields = 0;
    let mut scalar_fields = 0;
    for ty in arg_types {
        match ty {
            LcnfType::Object | LcnfType::Ctor(_, _) | LcnfType::LcnfString => {
                obj_fields += 1;
            }
            LcnfType::Nat | LcnfType::Unit | LcnfType::Var(_) => {
                scalar_fields += 1;
            }
            LcnfType::Erased | LcnfType::Irrelevant => {}
            LcnfType::Fun(_, _) => {
                obj_fields += 1;
            }
        }
    }
    LayoutInfo::new(ctor_name, ctor_tag, obj_fields, scalar_fields)
}
/// Detect opportunities for in-place updates (destructive updates)
///
/// An in-place update is possible when:
/// 1. The original value has unique ownership
/// 2. The new value is the same constructor with some fields changed
/// 3. The original value is not used after the update
pub(super) fn find_in_place_updates(
    expr: &LcnfExpr,
    unique_vars: &HashSet<LcnfVarId>,
) -> Vec<InPlaceUpdate> {
    let mut updates = Vec::new();
    find_in_place_inner(expr, unique_vars, &mut updates);
    updates
}
pub(super) fn find_in_place_inner(
    expr: &LcnfExpr,
    unique_vars: &HashSet<LcnfVarId>,
    updates: &mut Vec<InPlaceUpdate>,
) {
    match expr {
        LcnfExpr::Case {
            scrutinee, alts, ..
        } => {
            if unique_vars.contains(scrutinee) {
                for alt in alts {
                    if let Some(update) =
                        detect_ctor_update(*scrutinee, &alt.ctor_name, &alt.params, &alt.body)
                    {
                        updates.push(update);
                    }
                    find_in_place_inner(&alt.body, unique_vars, updates);
                }
            }
        }
        LcnfExpr::Let { body, .. } => {
            find_in_place_inner(body, unique_vars, updates);
        }
        _ => {}
    }
}
/// Detect if a case alt body constructs the same constructor with minor changes
pub(super) fn detect_ctor_update(
    scrutinee: LcnfVarId,
    ctor_name: &str,
    fields: &[LcnfParam],
    body: &LcnfExpr,
) -> Option<InPlaceUpdate> {
    if let LcnfExpr::Let {
        id,
        value: LcnfLetValue::Ctor(result_ctor, _, args),
        ..
    } = body
    {
        if result_ctor == ctor_name {
            let mut changed = Vec::new();
            let field_ids: Vec<LcnfVarId> = fields.iter().map(|f| f.id).collect();
            for (i, arg) in args.iter().enumerate() {
                if i < field_ids.len() {
                    match arg {
                        LcnfArg::Var(v) if *v == field_ids[i] => {}
                        _ => {
                            changed.push((i, arg.clone()));
                        }
                    }
                }
            }
            if !changed.is_empty() && changed.len() < args.len() {
                return Some(InPlaceUpdate {
                    source: scrutinee,
                    result: *id,
                    changed_fields: changed,
                });
            }
        }
    }
    None
}
/// Main entry point: optimize reuse in a module
pub fn optimize_reuse(module: &mut LcnfModule, config: &ReuseConfig) {
    let mut analyzer = ReuseAnalyzer::new(config.clone());
    for decl in &mut module.fun_decls {
        analyzer.analyze_decl(decl);
        if config.enable_reset_reuse {
            analyzer.apply_reuse(&mut decl.body);
        }
        if config.enable_borrow {
            analyzer.apply_borrows(decl);
        }
    }
}
/// Analyze ownership for a single declaration
pub fn analyze_ownership(
    decl: &LcnfFunDecl,
    config: &ReuseConfig,
) -> HashMap<LcnfVarId, BorrowInfo> {
    let mut analyzer = ReuseAnalyzer::new(config.clone());
    analyzer.analyze_decl(decl);
    analyzer.borrow_info
}
/// Check if a variable has unique ownership in a given context
pub fn is_uniquely_owned(var: LcnfVarId, borrow_info: &HashMap<LcnfVarId, BorrowInfo>) -> bool {
    borrow_info
        .get(&var)
        .is_some_and(|info| info.ownership == Ownership::Unique)
}
#[cfg(test)]
mod tests {
    use super::*;
    pub(super) fn make_var(n: u64) -> LcnfVarId {
        LcnfVarId(n)
    }
    pub(super) fn make_param(n: u64, name: &str) -> LcnfParam {
        LcnfParam {
            id: LcnfVarId(n),
            name: name.to_string(),
            ty: LcnfType::Nat,
            erased: false,
            borrowed: false,
        }
    }
    pub(super) fn make_simple_let(id: u64, value: LcnfLetValue, body: LcnfExpr) -> LcnfExpr {
        LcnfExpr::Let {
            id: LcnfVarId(id),
            name: format!("x{}", id),
            ty: LcnfType::Nat,
            value,
            body: Box::new(body),
        }
    }
    pub(super) fn make_decl(name: &str, params: Vec<LcnfParam>, body: LcnfExpr) -> LcnfFunDecl {
        LcnfFunDecl {
            name: name.to_string(),
            original_name: None,
            params,
            ret_type: LcnfType::Nat,
            body,
            is_recursive: false,
            is_lifted: false,
            inline_cost: 1,
        }
    }
    #[test]
    pub(super) fn test_config_default() {
        let config = ReuseConfig::default();
        assert!(config.enable_reset_reuse);
        assert!(config.enable_borrow);
        assert!(config.enable_rc_elim);
        assert!(config.enable_in_place);
    }
    #[test]
    pub(super) fn test_ownership_merge() {
        assert_eq!(
            Ownership::Unique.merge(Ownership::Unique),
            Ownership::Unique
        );
        assert_eq!(
            Ownership::Unique.merge(Ownership::Shared),
            Ownership::Shared
        );
        assert_eq!(
            Ownership::Borrowed.merge(Ownership::Borrowed),
            Ownership::Borrowed
        );
        assert_eq!(
            Ownership::Unknown.merge(Ownership::Unique),
            Ownership::Unknown
        );
    }
    #[test]
    pub(super) fn test_borrow_info_new() {
        let info = BorrowInfo::new(make_var(0));
        assert!(!info.can_borrow);
        assert_eq!(info.ownership, Ownership::Unknown);
        assert!(!info.escapes);
    }
    #[test]
    pub(super) fn test_borrow_info_with_ownership() {
        let info = BorrowInfo::with_ownership(make_var(0), Ownership::Unique);
        assert_eq!(info.ownership, Ownership::Unique);
        assert!(!info.can_borrow);
        let info = BorrowInfo::with_ownership(make_var(1), Ownership::Borrowed);
        assert_eq!(info.ownership, Ownership::Borrowed);
        assert!(info.can_borrow);
    }
    #[test]
    pub(super) fn test_layout_info_compatible() {
        let l1 = LayoutInfo::new("Cons", 0, 2, 0);
        let l2 = LayoutInfo::new("Cons", 0, 2, 0);
        assert!(l1.is_compatible_with(&l2));
        let l3 = LayoutInfo::new("Nil", 1, 0, 0);
        assert!(!l1.is_compatible_with(&l3));
    }
    #[test]
    pub(super) fn test_layout_info_fits_in() {
        let small = LayoutInfo::new("Nil", 0, 0, 0);
        let large = LayoutInfo::new("Cons", 1, 2, 1);
        assert!(small.fits_in(&large));
        assert!(!large.fits_in(&small));
    }
    #[test]
    pub(super) fn test_compute_layout() {
        let layout = compute_layout("Cons", 0, &[LcnfType::Object, LcnfType::Object]);
        assert_eq!(layout.obj_fields, 2);
        assert_eq!(layout.scalar_fields, 0);
        assert_eq!(layout.total_words, 3);
        let layout2 = compute_layout("Pair", 0, &[LcnfType::Nat, LcnfType::Object]);
        assert_eq!(layout2.obj_fields, 1);
        assert_eq!(layout2.scalar_fields, 1);
    }
    #[test]
    pub(super) fn test_compute_layout_erased() {
        let layout = compute_layout("Box", 0, &[LcnfType::Object, LcnfType::Erased]);
        assert_eq!(layout.obj_fields, 1);
        assert_eq!(layout.scalar_fields, 0);
    }
    #[test]
    pub(super) fn test_analyzer_use_counts() {
        let body = make_simple_let(
            1,
            LcnfLetValue::FVar(make_var(0)),
            make_simple_let(
                2,
                LcnfLetValue::FVar(make_var(0)),
                LcnfExpr::Return(LcnfArg::Var(make_var(2))),
            ),
        );
        let mut analyzer = ReuseAnalyzer::new(ReuseConfig::default());
        analyzer.compute_use_counts(&body);
        assert_eq!(
            analyzer.use_counts.get(&make_var(0)).copied().unwrap_or(0),
            2
        );
        assert_eq!(
            analyzer.use_counts.get(&make_var(2)).copied().unwrap_or(0),
            1
        );
    }
    #[test]
    pub(super) fn test_analyzer_ownership_inference() {
        let body = make_simple_let(
            1,
            LcnfLetValue::Ctor(
                "Pair".to_string(),
                0,
                vec![LcnfArg::Var(make_var(0)), LcnfArg::Lit(LcnfLit::Nat(42))],
            ),
            LcnfExpr::Return(LcnfArg::Var(make_var(1))),
        );
        let decl = make_decl("test", vec![make_param(0, "x")], body);
        let mut analyzer = ReuseAnalyzer::new(ReuseConfig::default());
        analyzer.analyze_decl(&decl);
        if let Some(info) = analyzer.borrow_info.get(&make_var(1)) {
            assert_eq!(info.ownership, Ownership::Unique);
        }
    }
    #[test]
    pub(super) fn test_does_escape_return() {
        let body = LcnfExpr::Return(LcnfArg::Var(make_var(1)));
        let analyzer = ReuseAnalyzer::new(ReuseConfig::default());
        assert!(analyzer.does_escape(make_var(1), &body));
        assert!(!analyzer.does_escape(make_var(2), &body));
    }
    #[test]
    pub(super) fn test_does_escape_tailcall() {
        let body = LcnfExpr::TailCall(LcnfArg::Var(make_var(10)), vec![LcnfArg::Var(make_var(1))]);
        let analyzer = ReuseAnalyzer::new(ReuseConfig::default());
        assert!(analyzer.does_escape(make_var(1), &body));
        assert!(!analyzer.does_escape(make_var(2), &body));
    }
    #[test]
    pub(super) fn test_does_escape_ctor() {
        let body = make_simple_let(
            5,
            LcnfLetValue::Ctor("Pair".to_string(), 0, vec![LcnfArg::Var(make_var(1))]),
            LcnfExpr::Return(LcnfArg::Var(make_var(5))),
        );
        let analyzer = ReuseAnalyzer::new(ReuseConfig::default());
        assert!(analyzer.does_escape(make_var(1), &body));
    }
    #[test]
    pub(super) fn test_borrow_inference() {
        let body = make_simple_let(
            1,
            LcnfLetValue::Proj("Point".to_string(), 0, make_var(0)),
            LcnfExpr::Return(LcnfArg::Var(make_var(1))),
        );
        let decl = make_decl("get_x", vec![make_param(0, "point")], body);
        let mut analyzer = ReuseAnalyzer::new(ReuseConfig::default());
        analyzer.analyze_decl(&decl);
        if let Some(info) = analyzer.borrow_info.get(&make_var(0)) {
            assert!(info.can_borrow);
        }
    }
    #[test]
    pub(super) fn test_reuse_opportunity_detection() {
        let body = LcnfExpr::Case {
            scrutinee: make_var(0),
            scrutinee_ty: LcnfType::Object,
            alts: vec![LcnfAlt {
                ctor_name: "Cons".to_string(),
                ctor_tag: 0,
                params: vec![make_param(1, "h"), make_param(2, "t")],
                body: make_simple_let(
                    3,
                    LcnfLetValue::Ctor(
                        "Cons".to_string(),
                        0,
                        vec![LcnfArg::Lit(LcnfLit::Nat(99)), LcnfArg::Var(make_var(2))],
                    ),
                    LcnfExpr::Return(LcnfArg::Var(make_var(3))),
                ),
            }],
            default: None,
        };
        let decl = make_decl("map_head", vec![make_param(0, "list")], body);
        let mut analyzer = ReuseAnalyzer::new(ReuseConfig::default());
        analyzer.borrow_info.insert(
            make_var(0),
            BorrowInfo::with_ownership(make_var(0), Ownership::Unique),
        );
        analyzer.find_reuse_opportunities(&decl.body);
        assert_eq!(analyzer.reuse_opportunities.len(), 1);
        let opp = &analyzer.reuse_opportunities[0];
        assert_eq!(opp.dealloc_var, make_var(0));
        assert_eq!(opp.alloc_var, make_var(3));
        assert_eq!(opp.dealloc_ctor, "Cons");
        assert_eq!(opp.alloc_ctor, "Cons");
    }
    #[test]
    pub(super) fn test_rc_elim_unique() {
        let body = make_simple_let(
            1,
            LcnfLetValue::Lit(LcnfLit::Nat(42)),
            LcnfExpr::Return(LcnfArg::Var(make_var(1))),
        );
        let decl = make_decl("test", vec![], body);
        let mut analyzer = ReuseAnalyzer::new(ReuseConfig::default());
        analyzer.analyze_decl(&decl);
        assert!(analyzer
            .rc_eliminations
            .iter()
            .any(|e| e.var == make_var(1)));
    }
    #[test]
    pub(super) fn test_optimize_reuse_empty() {
        let mut module = LcnfModule::default();
        let config = ReuseConfig::default();
        optimize_reuse(&mut module, &config);
        assert!(module.fun_decls.is_empty());
    }
    #[test]
    pub(super) fn test_optimize_reuse_simple() {
        let body = make_simple_let(
            1,
            LcnfLetValue::Lit(LcnfLit::Nat(42)),
            LcnfExpr::Return(LcnfArg::Var(make_var(1))),
        );
        let decl = make_decl("test", vec![make_param(0, "x")], body);
        let mut module = LcnfModule {
            fun_decls: vec![decl],
            extern_decls: vec![],
            name: "test".to_string(),
            metadata: LcnfModuleMetadata::default(),
        };
        let config = ReuseConfig::default();
        optimize_reuse(&mut module, &config);
        assert_eq!(module.fun_decls.len(), 1);
    }
    #[test]
    pub(super) fn test_analyze_ownership_api() {
        let body = make_simple_let(
            1,
            LcnfLetValue::Ctor("Box".to_string(), 0, vec![LcnfArg::Var(make_var(0))]),
            LcnfExpr::Return(LcnfArg::Var(make_var(1))),
        );
        let decl = make_decl("wrap", vec![make_param(0, "x")], body);
        let config = ReuseConfig::default();
        let info = analyze_ownership(&decl, &config);
        assert!(info.contains_key(&make_var(1)));
    }
    #[test]
    pub(super) fn test_is_uniquely_owned() {
        let mut info = HashMap::new();
        info.insert(
            make_var(0),
            BorrowInfo::with_ownership(make_var(0), Ownership::Unique),
        );
        info.insert(
            make_var(1),
            BorrowInfo::with_ownership(make_var(1), Ownership::Shared),
        );
        assert!(is_uniquely_owned(make_var(0), &info));
        assert!(!is_uniquely_owned(make_var(1), &info));
        assert!(!is_uniquely_owned(make_var(2), &info));
    }
    #[test]
    pub(super) fn test_lifetime_analyzer() {
        let mut la = LifetimeAnalyzer::new();
        la.push_scope();
        la.define_var(make_var(0));
        la.borrow_var(make_var(0));
        assert!(la.is_borrow_safe(make_var(0)));
        assert!(!la.is_borrow_safe(make_var(1)));
        la.pop_scope();
    }
    #[test]
    pub(super) fn test_lifetime_analyzer_nested() {
        let mut la = LifetimeAnalyzer::new();
        la.push_scope();
        la.define_var(make_var(0));
        la.push_scope();
        la.define_var(make_var(1));
        assert!(la.is_borrow_safe(make_var(0)));
        assert!(la.is_borrow_safe(make_var(1)));
        la.pop_scope();
        la.pop_scope();
    }
    #[test]
    pub(super) fn test_in_place_update_detection() {
        let body = LcnfExpr::Case {
            scrutinee: make_var(0),
            scrutinee_ty: LcnfType::Object,
            alts: vec![LcnfAlt {
                ctor_name: "Pair".to_string(),
                ctor_tag: 0,
                params: vec![make_param(1, "fst"), make_param(2, "snd")],
                body: make_simple_let(
                    3,
                    LcnfLetValue::Ctor(
                        "Pair".to_string(),
                        0,
                        vec![LcnfArg::Var(make_var(1)), LcnfArg::Lit(LcnfLit::Nat(999))],
                    ),
                    LcnfExpr::Return(LcnfArg::Var(make_var(3))),
                ),
            }],
            default: None,
        };
        let mut unique_vars = HashSet::new();
        unique_vars.insert(make_var(0));
        let updates = find_in_place_updates(&body, &unique_vars);
        assert_eq!(updates.len(), 1);
        assert_eq!(updates[0].source, make_var(0));
        assert_eq!(updates[0].result, make_var(3));
        assert_eq!(updates[0].changed_fields.len(), 1);
        assert_eq!(updates[0].changed_fields[0].0, 1);
    }
    #[test]
    pub(super) fn test_stats_default() {
        let stats = ReuseStats::default();
        assert_eq!(stats.reuse_pairs, 0);
        assert_eq!(stats.borrows_inferred, 0);
        assert_eq!(stats.rc_ops_eliminated, 0);
    }
    #[test]
    pub(super) fn test_rc_elim_kinds() {
        let elim = RcElimInfo {
            var: make_var(0),
            kind: RcElimKind::SkipInc,
            reason: RcElimReason::Borrowed,
        };
        assert_eq!(elim.kind, RcElimKind::SkipInc);
        let elim2 = RcElimInfo {
            var: make_var(1),
            kind: RcElimKind::CancelPair,
            reason: RcElimReason::CancelledPair,
        };
        assert_eq!(elim2.kind, RcElimKind::CancelPair);
    }
    #[test]
    pub(super) fn test_reuse_opportunity_struct() {
        let opp = ReuseOpportunity {
            dealloc_var: make_var(0),
            alloc_var: make_var(1),
            dealloc_ctor: "Old".to_string(),
            dealloc_tag: 0,
            alloc_ctor: "New".to_string(),
            alloc_tag: 1,
            layout_compatible: true,
            estimated_savings: 24,
        };
        assert!(opp.layout_compatible);
        assert_eq!(opp.estimated_savings, 24);
    }
    #[test]
    pub(super) fn test_analyzer_accessors() {
        let analyzer = ReuseAnalyzer::new(ReuseConfig::default());
        assert!(analyzer.reuse_opportunities().is_empty());
        assert!(analyzer.rc_eliminations().is_empty());
        assert!(analyzer.get_borrow_info(&make_var(0)).is_none());
        assert_eq!(analyzer.stats().reuse_pairs, 0);
    }
    #[test]
    pub(super) fn test_multiple_uses_shared_ownership() {
        let body = make_simple_let(
            1,
            LcnfLetValue::FVar(make_var(0)),
            make_simple_let(
                2,
                LcnfLetValue::FVar(make_var(0)),
                make_simple_let(
                    3,
                    LcnfLetValue::FVar(make_var(0)),
                    LcnfExpr::Return(LcnfArg::Var(make_var(3))),
                ),
            ),
        );
        let decl = make_decl("multi_use", vec![make_param(0, "x")], body);
        let mut analyzer = ReuseAnalyzer::new(ReuseConfig::default());
        analyzer.analyze_decl(&decl);
        if let Some(info) = analyzer.borrow_info.get(&make_var(0)) {
            assert_eq!(info.ownership, Ownership::Shared);
        }
    }
    #[test]
    pub(super) fn test_projection_ownership() {
        let body = make_simple_let(
            1,
            LcnfLetValue::Proj("Point".to_string(), 0, make_var(0)),
            LcnfExpr::Return(LcnfArg::Var(make_var(1))),
        );
        let decl = make_decl("proj_test", vec![make_param(0, "point")], body);
        let mut analyzer = ReuseAnalyzer::new(ReuseConfig::default());
        analyzer.analyze_decl(&decl);
        if let Some(info) = analyzer.borrow_info.get(&make_var(1)) {
            assert_eq!(info.ownership, Ownership::Borrowed);
        }
    }
    #[test]
    pub(super) fn test_lifetime_analyze_expr() {
        let body = make_simple_let(
            1,
            LcnfLetValue::Lit(LcnfLit::Nat(42)),
            LcnfExpr::Case {
                scrutinee: make_var(1),
                scrutinee_ty: LcnfType::Nat,
                alts: vec![LcnfAlt {
                    ctor_name: "Zero".to_string(),
                    ctor_tag: 0,
                    params: vec![],
                    body: LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0))),
                }],
                default: Some(Box::new(LcnfExpr::Return(LcnfArg::Var(make_var(1))))),
            },
        );
        let mut la = LifetimeAnalyzer::new();
        la.push_scope();
        la.analyze(&body);
        la.pop_scope();
    }
}
#[allow(dead_code)]
pub fn classify_size(bytes: u64) -> ReuseMemSizeClass {
    match bytes {
        0..=8 => ReuseMemSizeClass::Tiny,
        9..=64 => ReuseMemSizeClass::Small,
        65..=512 => ReuseMemSizeClass::Medium,
        513..=4096 => ReuseMemSizeClass::Large,
        _ => ReuseMemSizeClass::Huge,
    }
}
/// Reuse pass version
#[allow(dead_code)]
pub const REUSE_PASS_VERSION: &str = "1.0.0";
/// Reuse analysis version
#[allow(dead_code)]
pub const REUSE_ANALYSIS_VERSION: &str = "1.0.0";
/// Reuse max live range distance
#[allow(dead_code)]
pub const REUSE_MAX_DISTANCE: usize = 10_000;
/// Reuse analysis version
#[allow(dead_code)]
pub const REUSE_VERSION_STR: &str = "reuse-analysis-1.0.0";
#[cfg(test)]
mod OR_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = ORPassConfig::new("test_pass", ORPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = ORPassStats::new();
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
        let mut reg = ORPassRegistry::new();
        reg.register(ORPassConfig::new("pass_a", ORPassPhase::Analysis));
        reg.register(ORPassConfig::new("pass_b", ORPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = ORAnalysisCache::new(10);
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
        let mut wl = ORWorklist::new();
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
        let mut dt = ORDominatorTree::new(5);
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
        let mut liveness = ORLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(ORConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(ORConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(ORConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            ORConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(ORConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = ORDepGraph::new();
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
/// Reuse analysis version info
#[allow(dead_code)]
pub const REUSE_BACKEND_VERSION: &str = "1.0.0";
/// Reuse analysis emit function
#[allow(dead_code)]
pub fn reuse_emit_decision_log(log: &ReuseAllocLog) -> String {
    let mut out = String::from("// Reuse allocation log:\n");
    for r in &log.records {
        out.push_str(&format!("//   {}\n", r));
    }
    out
}
