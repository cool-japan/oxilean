//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    FailureReason, InstanceEntry, InstanceSynthesizer, SearchStats, SynthDiagnostics,
    SynthInstanceAnalysisPass, SynthInstanceConfig, SynthInstanceConfigValue,
    SynthInstanceDiagnostics, SynthInstanceDiff, SynthInstanceExtConfig1500,
    SynthInstanceExtConfigVal1500, SynthInstanceExtDiag1500, SynthInstanceExtDiff1500,
    SynthInstanceExtPass1500, SynthInstanceExtPipeline1500, SynthInstanceExtResult1500,
    SynthInstancePipeline, SynthInstanceResult, SynthResult,
};
use crate::basic::{MVarId, MetaContext};
use oxilean_kernel::{BinderInfo, Expr, Level, Name};

/// Priority level for instances (0-65535, lower = higher priority).
pub type InstancePriority = u32;
/// Default priority for instances.
pub const DEFAULT_PRIORITY: InstancePriority = 1000;
/// Maximum instance priority.
pub const MAX_PRIORITY: InstancePriority = 65535;
/// Extract the class name from a type class application.
///
/// For `Add Nat`, returns `Add`.
/// For `Monad IO`, returns `Monad`.
pub(super) fn extract_class_name(ty: &Expr) -> Name {
    let mut e = ty;
    while let Expr::App(f, _) = e {
        e = f;
    }
    match e {
        Expr::Const(name, _) => name.clone(),
        _ => Name::Anonymous,
    }
}
/// Check if two goals are structurally similar (for loop detection).
pub(super) fn goals_structurally_similar(goal1: &Expr, goal2: &Expr) -> bool {
    match (goal1, goal2) {
        (Expr::App(f1, a1), Expr::App(f2, a2)) => {
            goals_structurally_similar(f1, f2) && goals_structurally_similar(a1, a2)
        }
        (Expr::Const(n1, _), Expr::Const(n2, _)) => n1 == n2,
        (Expr::FVar(id1), Expr::FVar(id2)) => id1 == id2,
        _ => false,
    }
}
/// Collect unassigned metavariable IDs from an expression.
pub(super) fn collect_unassigned_mvars(expr: &Expr, ctx: &MetaContext) -> Vec<MVarId> {
    let mut result = Vec::new();
    collect_unassigned_mvars_impl(expr, ctx, &mut result);
    result.sort_by_key(|id| id.0);
    result.dedup();
    result
}
pub(super) fn collect_unassigned_mvars_impl(
    expr: &Expr,
    ctx: &MetaContext,
    result: &mut Vec<MVarId>,
) {
    if let Some(id) = MetaContext::is_mvar_expr(expr) {
        if !ctx.is_mvar_assigned(id) {
            result.push(id);
        }
        return;
    }
    match expr {
        Expr::App(f, a) => {
            collect_unassigned_mvars_impl(f, ctx, result);
            collect_unassigned_mvars_impl(a, ctx, result);
        }
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            collect_unassigned_mvars_impl(ty, ctx, result);
            collect_unassigned_mvars_impl(body, ctx, result);
        }
        Expr::Let(_, ty, val, body) => {
            collect_unassigned_mvars_impl(ty, ctx, result);
            collect_unassigned_mvars_impl(val, ctx, result);
            collect_unassigned_mvars_impl(body, ctx, result);
        }
        Expr::Proj(_, _, e) => {
            collect_unassigned_mvars_impl(e, ctx, result);
        }
        _ => {}
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::synth_instance::*;
    use oxilean_kernel::Environment;
    fn mk_ctx() -> MetaContext {
        MetaContext::new(Environment::new())
    }
    #[test]
    fn test_create_synthesizer() {
        let synth = InstanceSynthesizer::new();
        assert_eq!(synth.num_instances(), 0);
    }
    #[test]
    fn test_add_instance() {
        let mut synth = InstanceSynthesizer::new();
        let entry = InstanceEntry {
            name: Name::str("Add.instNat"),
            ty: Expr::App(
                Box::new(Expr::Const(Name::str("Add"), vec![])),
                Box::new(Expr::Const(Name::str("Nat"), vec![])),
            ),
            priority: DEFAULT_PRIORITY,
            is_local: false,
            preferred: false,
        };
        synth.add_instance(entry);
        assert_eq!(synth.num_instances(), 1);
        let instances = synth.get_instances(&Name::str("Add"));
        assert_eq!(instances.len(), 1);
    }
    #[test]
    fn test_get_instances_empty() {
        let synth = InstanceSynthesizer::new();
        let instances = synth.get_instances(&Name::str("Add"));
        assert!(instances.is_empty());
    }
    #[test]
    fn test_extract_class_name() {
        let ty = Expr::App(
            Box::new(Expr::Const(Name::str("Add"), vec![])),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
        );
        assert_eq!(extract_class_name(&ty), Name::str("Add"));
        let simple = Expr::Const(Name::str("Inhabited"), vec![]);
        assert_eq!(extract_class_name(&simple), Name::str("Inhabited"));
    }
    #[test]
    fn test_synthesis_failure() {
        let mut synth = InstanceSynthesizer::new();
        let mut ctx = mk_ctx();
        let goal = Expr::App(
            Box::new(Expr::Const(Name::str("Add"), vec![])),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
        );
        let result = synth.synthesize(&goal, &mut ctx);
        assert!(!result.is_success());
    }
    #[test]
    fn test_synthesis_simple() {
        let mut synth = InstanceSynthesizer::new();
        let mut ctx = mk_ctx();
        let add_nat_ty = Expr::App(
            Box::new(Expr::Const(Name::str("Add"), vec![])),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
        );
        synth.add_instance(InstanceEntry {
            name: Name::str("instAddNat"),
            ty: add_nat_ty.clone(),
            priority: DEFAULT_PRIORITY,
            is_local: false,
            preferred: false,
        });
        let result = synth.synthesize(&add_nat_ty, &mut ctx);
        assert!(result.is_success());
    }
    #[test]
    fn test_priority_ordering() {
        let mut synth = InstanceSynthesizer::new();
        synth.add_instance(InstanceEntry {
            name: Name::str("low_priority"),
            ty: Expr::Const(Name::str("C"), vec![]),
            priority: 2000,
            is_local: false,
            preferred: false,
        });
        synth.add_instance(InstanceEntry {
            name: Name::str("high_priority"),
            ty: Expr::Const(Name::str("C"), vec![]),
            priority: 500,
            is_local: false,
            preferred: false,
        });
        let sorted =
            synth.get_sorted_candidates(&Name::str("C"), &Expr::Const(Name::str("C"), vec![]));
        assert_eq!(sorted.len(), 2);
        assert_eq!(sorted[0].name, Name::str("high_priority"));
    }
    #[test]
    fn test_priority_with_preference() {
        let mut synth = InstanceSynthesizer::new();
        synth.add_instance(InstanceEntry {
            name: Name::str("preferred_high"),
            ty: Expr::Const(Name::str("C"), vec![]),
            priority: 1000,
            is_local: false,
            preferred: true,
        });
        synth.add_instance(InstanceEntry {
            name: Name::str("not_preferred_low"),
            ty: Expr::Const(Name::str("C"), vec![]),
            priority: 500,
            is_local: false,
            preferred: false,
        });
        let sorted =
            synth.get_sorted_candidates(&Name::str("C"), &Expr::Const(Name::str("C"), vec![]));
        assert_eq!(sorted[0].priority, 500);
    }
    #[test]
    fn test_local_instance_priority() {
        let mut synth = InstanceSynthesizer::new();
        synth.add_instance(InstanceEntry {
            name: Name::str("global"),
            ty: Expr::Const(Name::str("C"), vec![]),
            priority: 1000,
            is_local: false,
            preferred: false,
        });
        synth.add_instance(InstanceEntry {
            name: Name::str("local"),
            ty: Expr::Const(Name::str("C"), vec![]),
            priority: 1000,
            is_local: true,
            preferred: false,
        });
        let sorted =
            synth.get_sorted_candidates(&Name::str("C"), &Expr::Const(Name::str("C"), vec![]));
        assert!(sorted[0].is_local);
    }
    #[test]
    fn test_clear_cache() {
        let mut synth = InstanceSynthesizer::new();
        synth.cache.insert(
            Expr::Const(Name::str("test"), vec![]),
            Expr::Const(Name::str("result"), vec![]),
        );
        assert_eq!(synth.cache.len(), 1);
        synth.clear_cache();
        assert!(synth.cache.is_empty());
    }
    #[test]
    fn test_cache_success() {
        let mut synth = InstanceSynthesizer::new();
        let mut ctx = mk_ctx();
        let goal = Expr::Const(Name::str("Inhabited"), vec![]);
        synth.add_instance(InstanceEntry {
            name: Name::str("instInhabited"),
            ty: goal.clone(),
            priority: DEFAULT_PRIORITY,
            is_local: false,
            preferred: false,
        });
        let result1 = synth.synthesize(&goal, &mut ctx);
        assert!(result1.is_success());
        assert_eq!(synth.current_stats.cache_hits, 0);
        let result2 = synth.synthesize(&goal, &mut ctx);
        assert!(result2.is_success());
        assert_eq!(synth.current_stats.cache_hits, 1);
    }
    #[test]
    fn test_set_limits() {
        let mut synth = InstanceSynthesizer::new();
        synth.set_max_depth(64);
        assert_eq!(synth.max_depth, 64);
        synth.set_max_heartbeats(50_000);
        assert_eq!(synth.max_heartbeats, 50_000);
    }
    #[test]
    fn test_heartbeat_tracking() {
        let mut synth = InstanceSynthesizer::new();
        let mut ctx = mk_ctx();
        let goal = Expr::App(
            Box::new(Expr::Const(Name::str("Add"), vec![])),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
        );
        synth.add_instance(InstanceEntry {
            name: Name::str("inst1"),
            ty: goal.clone(),
            priority: 1000,
            is_local: false,
            preferred: false,
        });
        synth.synthesize(&goal, &mut ctx);
        assert!(synth.last_heartbeats() > 0);
    }
    #[test]
    fn test_instance_entry_builder() {
        let entry = InstanceEntry::new(Name::str("test"), Expr::Const(Name::str("C"), vec![]))
            .with_priority(500)
            .with_local(true)
            .with_preferred(true);
        assert_eq!(entry.priority, 500);
        assert!(entry.is_local);
        assert!(entry.preferred);
    }
    #[test]
    fn test_diagnostics_no_instances() {
        let mut synth = InstanceSynthesizer::new();
        let mut ctx = mk_ctx();
        let goal = Expr::Const(Name::str("Unknown"), vec![]);
        synth.synthesize(&goal, &mut ctx);
        let diag = synth.last_diagnostics();
        assert!(diag.is_some());
        if let Some(d) = diag {
            matches!(d.failure_reason, FailureReason::NoInstances);
        }
    }
    #[test]
    fn test_rank_candidates() {
        let mut synth = InstanceSynthesizer::new();
        synth.add_instance(InstanceEntry {
            name: Name::str("first"),
            ty: Expr::Const(Name::str("C"), vec![]),
            priority: 500,
            is_local: false,
            preferred: false,
        });
        synth.add_instance(InstanceEntry {
            name: Name::str("second"),
            ty: Expr::Const(Name::str("C"), vec![]),
            priority: 1000,
            is_local: false,
            preferred: false,
        });
        let goal = Expr::Const(Name::str("C"), vec![]);
        let ranks = synth.rank_candidates(&goal);
        assert_eq!(ranks.len(), 2);
        assert_eq!(ranks[0].0, Name::str("first"));
        assert_eq!(ranks[0].1, 0);
        assert_eq!(ranks[1].0, Name::str("second"));
        assert_eq!(ranks[1].1, 1);
    }
    #[test]
    fn test_overlap_detection() {
        let mut synth = InstanceSynthesizer::new();
        synth.add_instance(InstanceEntry {
            name: Name::str("inst1"),
            ty: Expr::Const(Name::str("C"), vec![]),
            priority: 1000,
            is_local: false,
            preferred: false,
        });
        synth.add_instance(InstanceEntry {
            name: Name::str("inst2"),
            ty: Expr::Const(Name::str("C"), vec![]),
            priority: 1000,
            is_local: false,
            preferred: false,
        });
        let has_overlap = synth.check_overlap(&Name::str("C"));
        assert!(has_overlap);
    }
    #[test]
    fn test_no_overlap_different_priority() {
        let mut synth = InstanceSynthesizer::new();
        synth.add_instance(InstanceEntry {
            name: Name::str("inst1"),
            ty: Expr::Const(Name::str("C"), vec![]),
            priority: 500,
            is_local: false,
            preferred: false,
        });
        synth.add_instance(InstanceEntry {
            name: Name::str("inst2"),
            ty: Expr::Const(Name::str("C"), vec![]),
            priority: 1000,
            is_local: false,
            preferred: false,
        });
        let has_overlap = synth.check_overlap(&Name::str("C"));
        assert!(!has_overlap);
    }
    #[test]
    fn test_search_stats() {
        let mut synth = InstanceSynthesizer::new();
        let mut ctx = mk_ctx();
        let goal = Expr::Const(Name::str("Test"), vec![]);
        synth.add_instance(InstanceEntry {
            name: Name::str("inst1"),
            ty: goal.clone(),
            priority: 1000,
            is_local: false,
            preferred: false,
        });
        synth.synthesize(&goal, &mut ctx);
        let stats = synth.current_stats();
        assert!(stats.instances_examined > 0);
    }
    #[test]
    fn test_instance_entry_default_priority() {
        let entry = InstanceEntry::new(Name::str("test"), Expr::Const(Name::str("C"), vec![]));
        assert_eq!(entry.priority, DEFAULT_PRIORITY);
    }
    #[test]
    fn test_instance_entry_not_local_by_default() {
        let entry = InstanceEntry::new(Name::str("test"), Expr::Const(Name::str("C"), vec![]));
        assert!(!entry.is_local);
    }
    #[test]
    fn test_instance_entry_not_preferred_by_default() {
        let entry = InstanceEntry::new(Name::str("test"), Expr::Const(Name::str("C"), vec![]));
        assert!(!entry.preferred);
    }
    #[test]
    fn test_goals_structurally_similar() {
        let goal1 = Expr::Const(Name::str("C"), vec![]);
        let goal2 = Expr::Const(Name::str("C"), vec![]);
        assert!(goals_structurally_similar(&goal1, &goal2));
        let goal3 = Expr::Const(Name::str("D"), vec![]);
        assert!(!goals_structurally_similar(&goal1, &goal3));
    }
    #[test]
    fn test_goals_structurally_similar_apps() {
        let goal1 = Expr::App(
            Box::new(Expr::Const(Name::str("Add"), vec![])),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
        );
        let goal2 = Expr::App(
            Box::new(Expr::Const(Name::str("Add"), vec![])),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
        );
        assert!(goals_structurally_similar(&goal1, &goal2));
    }
    #[test]
    fn test_synth_result_success() {
        let result = SynthResult::Success(Expr::Const(Name::str("result"), vec![]));
        assert!(result.is_success());
        assert!(result.expr().is_some());
    }
    #[test]
    fn test_synth_result_failure() {
        let result: SynthResult = SynthResult::Failure;
        assert!(!result.is_success());
        assert!(result.expr().is_none());
    }
    #[test]
    fn test_synth_result_stuck() {
        let result = SynthResult::Stuck(MVarId::new(42));
        assert!(!result.is_success());
        assert!(result.expr().is_none());
    }
    #[test]
    fn test_multiple_instances_same_class() {
        let mut synth = InstanceSynthesizer::new();
        synth.add_instance(InstanceEntry {
            name: Name::str("inst1"),
            ty: Expr::Const(Name::str("C"), vec![]),
            priority: 1000,
            is_local: false,
            preferred: false,
        });
        synth.add_instance(InstanceEntry {
            name: Name::str("inst2"),
            ty: Expr::Const(Name::str("C"), vec![]),
            priority: 1500,
            is_local: false,
            preferred: false,
        });
        synth.add_instance(InstanceEntry {
            name: Name::str("inst3"),
            ty: Expr::Const(Name::str("C"), vec![]),
            priority: 500,
            is_local: false,
            preferred: false,
        });
        let instances = synth.get_instances(&Name::str("C"));
        assert_eq!(instances.len(), 3);
        let sorted =
            synth.get_sorted_candidates(&Name::str("C"), &Expr::Const(Name::str("C"), vec![]));
        assert_eq!(sorted[0].priority, 500);
        assert_eq!(sorted[1].priority, 1000);
        assert_eq!(sorted[2].priority, 1500);
    }
    #[test]
    fn test_different_classes_independent() {
        let mut synth = InstanceSynthesizer::new();
        synth.add_instance(InstanceEntry {
            name: Name::str("instA"),
            ty: Expr::Const(Name::str("ClassA"), vec![]),
            priority: 1000,
            is_local: false,
            preferred: false,
        });
        synth.add_instance(InstanceEntry {
            name: Name::str("instB"),
            ty: Expr::Const(Name::str("ClassB"), vec![]),
            priority: 1000,
            is_local: false,
            preferred: false,
        });
        assert_eq!(synth.get_instances(&Name::str("ClassA")).len(), 1);
        assert_eq!(synth.get_instances(&Name::str("ClassB")).len(), 1);
        assert_eq!(synth.get_instances(&Name::str("ClassC")).len(), 0);
    }
    #[test]
    fn test_default_synthesizer() {
        let synth: InstanceSynthesizer = Default::default();
        assert_eq!(synth.num_instances(), 0);
        assert_eq!(synth.max_depth, 32);
    }
    #[test]
    fn test_failure_reasons() {
        let _reasons = [
            FailureReason::NoInstances,
            FailureReason::UnificationFailed,
            FailureReason::LoopDetected,
            FailureReason::MaxDepthExceeded,
            FailureReason::Timeout,
            FailureReason::PostponedConstraints,
        ];
        assert!(_reasons.len() == 6);
    }
    #[test]
    fn test_instance_entry_builder_chaining() {
        let entry = InstanceEntry::new(Name::str("test"), Expr::Const(Name::str("C"), vec![]))
            .with_priority(100)
            .with_local(true)
            .with_preferred(true);
        assert_eq!(entry.priority, 100);
        assert!(entry.is_local);
        assert!(entry.preferred);
    }
    #[test]
    fn test_multiple_instances_different_priorities() {
        let mut synth = InstanceSynthesizer::new();
        synth.add_instance(
            InstanceEntry::new(Name::str("i1"), Expr::Const(Name::str("C"), vec![]))
                .with_priority(100),
        );
        synth.add_instance(
            InstanceEntry::new(Name::str("i2"), Expr::Const(Name::str("C"), vec![]))
                .with_priority(50),
        );
        synth.add_instance(
            InstanceEntry::new(Name::str("i3"), Expr::Const(Name::str("C"), vec![]))
                .with_priority(200),
        );
        let sorted =
            synth.get_sorted_candidates(&Name::str("C"), &Expr::Const(Name::str("C"), vec![]));
        assert_eq!(sorted[0].priority, 50);
        assert_eq!(sorted[1].priority, 100);
        assert_eq!(sorted[2].priority, 200);
    }
    #[test]
    fn test_search_stats_increment() {
        let mut stats = SearchStats::default();
        assert_eq!(stats.instances_examined, 0);
        stats.instances_examined = 5;
        assert_eq!(stats.instances_examined, 5);
    }
    #[test]
    fn test_synth_diagnostics_creation() {
        let diag = SynthDiagnostics {
            failure_reason: FailureReason::NoInstances,
            stats: SearchStats::default(),
            tried_candidates: vec![],
        };
        matches!(diag.failure_reason, FailureReason::NoInstances);
    }
    #[test]
    fn test_instance_entry_default_values() {
        let entry = InstanceEntry::new(Name::str("test"), Expr::Const(Name::str("C"), vec![]));
        assert_eq!(entry.priority, DEFAULT_PRIORITY);
        assert!(!entry.is_local);
        assert!(!entry.preferred);
    }
    #[test]
    fn test_get_sorted_candidates_empty() {
        let synth = InstanceSynthesizer::new();
        let sorted = synth.get_sorted_candidates(
            &Name::str("Unknown"),
            &Expr::Const(Name::str("Unknown"), vec![]),
        );
        assert!(sorted.is_empty());
    }
    #[test]
    fn test_get_instances_by_class_multiple_classes() {
        let mut synth = InstanceSynthesizer::new();
        synth.add_instance(InstanceEntry::new(
            Name::str("a1"),
            Expr::Const(Name::str("A"), vec![]),
        ));
        synth.add_instance(InstanceEntry::new(
            Name::str("b1"),
            Expr::Const(Name::str("B"), vec![]),
        ));
        synth.add_instance(InstanceEntry::new(
            Name::str("a2"),
            Expr::Const(Name::str("A"), vec![]),
        ));
        assert_eq!(synth.get_instances(&Name::str("A")).len(), 2);
        assert_eq!(synth.get_instances(&Name::str("B")).len(), 1);
        assert_eq!(synth.get_instances(&Name::str("C")).len(), 0);
    }
    #[test]
    fn test_priority_boundary_values() {
        let mut synth = InstanceSynthesizer::new();
        synth.add_instance(
            InstanceEntry::new(Name::str("min"), Expr::Const(Name::str("C"), vec![]))
                .with_priority(0),
        );
        synth.add_instance(
            InstanceEntry::new(Name::str("max"), Expr::Const(Name::str("C"), vec![]))
                .with_priority(MAX_PRIORITY),
        );
        let sorted =
            synth.get_sorted_candidates(&Name::str("C"), &Expr::Const(Name::str("C"), vec![]));
        assert_eq!(sorted[0].name, Name::str("min"));
        assert_eq!(sorted[1].name, Name::str("max"));
    }
    #[test]
    fn test_local_vs_global_instances() {
        let mut synth = InstanceSynthesizer::new();
        synth.add_instance(InstanceEntry::new(
            Name::str("global"),
            Expr::Const(Name::str("C"), vec![]),
        ));
        synth.add_local_instance(InstanceEntry::new(
            Name::str("local"),
            Expr::Const(Name::str("C"), vec![]),
        ));
        let instances = synth.get_instances(&Name::str("C"));
        assert_eq!(instances.len(), 2);
        let local_count = instances.iter().filter(|i| i.is_local).count();
        assert_eq!(local_count, 1);
    }
    #[test]
    fn test_cache_insertion_and_retrieval() {
        let mut synth = InstanceSynthesizer::new();
        let expr = Expr::Const(Name::str("test"), vec![]);
        let result = Expr::Const(Name::str("result"), vec![]);
        synth.cache.insert(expr.clone(), result.clone());
        assert!(synth.cache.contains_key(&expr));
        assert_eq!(
            synth
                .cache
                .get(&expr)
                .expect("element at &expr should exist"),
            &result
        );
    }
    #[test]
    fn test_failure_cache_insertion() {
        let mut synth = InstanceSynthesizer::new();
        let expr = Expr::Const(Name::str("test"), vec![]);
        synth
            .failure_cache
            .insert(expr.clone(), FailureReason::NoInstances);
        assert!(synth.failure_cache.contains_key(&expr));
    }
    #[test]
    fn test_clear_both_caches() {
        let mut synth = InstanceSynthesizer::new();
        synth.cache.insert(
            Expr::Const(Name::str("a"), vec![]),
            Expr::Const(Name::str("b"), vec![]),
        );
        synth.failure_cache.insert(
            Expr::Const(Name::str("c"), vec![]),
            FailureReason::NoInstances,
        );
        assert!(!synth.cache.is_empty());
        assert!(!synth.failure_cache.is_empty());
        synth.clear_cache();
        assert!(synth.cache.is_empty());
        assert!(synth.failure_cache.is_empty());
    }
    #[test]
    fn test_synthesizer_max_depth_configuration() {
        let mut synth = InstanceSynthesizer::new();
        assert_eq!(synth.max_depth, 32);
        synth.set_max_depth(128);
        assert_eq!(synth.max_depth, 128);
        synth.set_max_depth(1);
        assert_eq!(synth.max_depth, 1);
    }
    #[test]
    fn test_synthesizer_heartbeat_configuration() {
        let mut synth = InstanceSynthesizer::new();
        assert_eq!(synth.max_heartbeats, 20_000);
        synth.set_max_heartbeats(100_000);
        assert_eq!(synth.max_heartbeats, 100_000);
        synth.set_max_heartbeats(1);
        assert_eq!(synth.max_heartbeats, 1);
    }
    #[test]
    fn test_trail_operations() {
        let mut synth = InstanceSynthesizer::new();
        assert!(synth.trail.is_empty());
        let expr = Expr::Const(Name::str("test"), vec![]);
        synth.trail.push(expr.clone());
        assert_eq!(synth.trail.len(), 1);
        synth.trail.pop();
        assert!(synth.trail.is_empty());
    }
    #[test]
    fn test_choice_points_initialization() {
        let synth = InstanceSynthesizer::new();
        assert!(synth.choice_points.is_empty());
    }
    #[test]
    fn test_resolution_nodes_initialization() {
        let synth = InstanceSynthesizer::new();
        assert!(synth.resolution_nodes.is_empty());
    }
    #[test]
    fn test_current_stats_initialization() {
        let synth = InstanceSynthesizer::new();
        let stats = synth.current_stats();
        assert_eq!(stats.instances_examined, 0);
        assert_eq!(stats.cache_hits, 0);
    }
    #[test]
    fn test_instance_count_grows() {
        let mut synth = InstanceSynthesizer::new();
        assert_eq!(synth.num_instances(), 0);
        for i in 0..5 {
            synth.add_instance(InstanceEntry::new(
                Name::str(format!("inst{}", i)),
                Expr::Const(Name::str("C"), vec![]),
            ));
        }
        assert_eq!(synth.num_instances(), 5);
    }
    #[test]
    fn test_preferred_flag_sorting() {
        let mut synth = InstanceSynthesizer::new();
        synth.add_instance(
            InstanceEntry::new(Name::str("not_pref"), Expr::Const(Name::str("C"), vec![]))
                .with_priority(100)
                .with_preferred(false),
        );
        synth.add_instance(
            InstanceEntry::new(Name::str("pref"), Expr::Const(Name::str("C"), vec![]))
                .with_priority(100)
                .with_preferred(true),
        );
        let sorted =
            synth.get_sorted_candidates(&Name::str("C"), &Expr::Const(Name::str("C"), vec![]));
        assert_eq!(sorted[0].name, Name::str("pref"));
    }
    #[test]
    fn test_multiple_classes_isolated() {
        let mut synth = InstanceSynthesizer::new();
        synth.add_instance(InstanceEntry::new(
            Name::str("monoid"),
            Expr::Const(Name::str("Monoid"), vec![]),
        ));
        synth.add_instance(InstanceEntry::new(
            Name::str("group"),
            Expr::Const(Name::str("Group"), vec![]),
        ));
        synth.add_instance(InstanceEntry::new(
            Name::str("ring"),
            Expr::Const(Name::str("Ring"), vec![]),
        ));
        assert_eq!(synth.get_instances(&Name::str("Monoid")).len(), 1);
        assert_eq!(synth.get_instances(&Name::str("Group")).len(), 1);
        assert_eq!(synth.get_instances(&Name::str("Ring")).len(), 1);
        assert_eq!(synth.num_instances(), 3);
    }
    #[test]
    fn test_overlap_check_false_with_different_priorities() {
        let mut synth = InstanceSynthesizer::new();
        synth.add_instance(
            InstanceEntry::new(Name::str("i1"), Expr::Const(Name::str("C"), vec![]))
                .with_priority(100),
        );
        synth.add_instance(
            InstanceEntry::new(Name::str("i2"), Expr::Const(Name::str("C"), vec![]))
                .with_priority(200),
        );
        assert!(!synth.check_overlap(&Name::str("C")));
    }
    #[test]
    fn test_overlap_check_with_single_instance() {
        let mut synth = InstanceSynthesizer::new();
        synth.add_instance(InstanceEntry::new(
            Name::str("only"),
            Expr::Const(Name::str("C"), vec![]),
        ));
        assert!(!synth.check_overlap(&Name::str("C")));
    }
    #[test]
    fn test_overlap_check_empty_class() {
        let synth = InstanceSynthesizer::new();
        assert!(!synth.check_overlap(&Name::str("Unknown")));
    }
    #[test]
    fn test_rank_candidates_ordering() {
        let mut synth = InstanceSynthesizer::new();
        synth.add_instance(
            InstanceEntry::new(Name::str("a"), Expr::Const(Name::str("C"), vec![]))
                .with_priority(50),
        );
        synth.add_instance(
            InstanceEntry::new(Name::str("b"), Expr::Const(Name::str("C"), vec![]))
                .with_priority(100),
        );
        synth.add_instance(
            InstanceEntry::new(Name::str("c"), Expr::Const(Name::str("C"), vec![]))
                .with_priority(150),
        );
        let goal = Expr::Const(Name::str("C"), vec![]);
        let ranks = synth.rank_candidates(&goal);
        assert_eq!(ranks[0].1, 0);
        assert_eq!(ranks[1].1, 1);
        assert_eq!(ranks[2].1, 2);
    }
    #[test]
    fn test_rank_candidates_empty() {
        let synth = InstanceSynthesizer::new();
        let goal = Expr::Const(Name::str("C"), vec![]);
        let ranks = synth.rank_candidates(&goal);
        assert!(ranks.is_empty());
    }
    #[test]
    fn test_extract_complex_class_name() {
        let ty = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Functor"), vec![])),
                Box::new(Expr::Const(Name::str("m"), vec![])),
            )),
            Box::new(Expr::Const(Name::str("n"), vec![])),
        );
        assert_eq!(extract_class_name(&ty), Name::str("Functor"));
    }
    #[test]
    fn test_goals_structurally_similar_nested() {
        let g1 = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("A"), vec![])),
                Box::new(Expr::Const(Name::str("B"), vec![])),
            )),
            Box::new(Expr::Const(Name::str("C"), vec![])),
        );
        let g2 = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("A"), vec![])),
                Box::new(Expr::Const(Name::str("B"), vec![])),
            )),
            Box::new(Expr::Const(Name::str("C"), vec![])),
        );
        assert!(goals_structurally_similar(&g1, &g2));
    }
    #[test]
    fn test_goals_structurally_dissimilar_nested() {
        let g1 = Expr::App(
            Box::new(Expr::Const(Name::str("A"), vec![])),
            Box::new(Expr::Const(Name::str("B"), vec![])),
        );
        let g2 = Expr::App(
            Box::new(Expr::Const(Name::str("A"), vec![])),
            Box::new(Expr::Const(Name::str("C"), vec![])),
        );
        assert!(!goals_structurally_similar(&g1, &g2));
    }
    #[test]
    fn test_collect_unassigned_mvars_empty() {
        let ctx = mk_ctx();
        let expr = Expr::Const(Name::str("test"), vec![]);
        let mvars = collect_unassigned_mvars(&expr, &ctx);
        assert!(mvars.is_empty());
    }
    #[test]
    fn test_synth_result_match_patterns() {
        let success = SynthResult::Success(Expr::Const(Name::str("ok"), vec![]));
        let failure = SynthResult::Failure;
        assert!(matches!(success, SynthResult::Success(_)));
        assert!(matches!(failure, SynthResult::Failure));
    }
    #[test]
    fn test_default_priority_constant() {
        assert_eq!(DEFAULT_PRIORITY, 1000);
    }
    #[test]
    fn test_max_priority_constant() {
        const _: () = assert!(MAX_PRIORITY >= DEFAULT_PRIORITY);
    }
    #[test]
    fn test_instance_entry_name_preservation() {
        let name = Name::str("test_instance");
        let entry = InstanceEntry::new(name.clone(), Expr::Const(Name::str("C"), vec![]));
        assert_eq!(entry.name, name);
    }
    #[test]
    fn test_instance_entry_type_preservation() {
        let ty = Expr::Const(Name::str("MyClass"), vec![]);
        let entry = InstanceEntry::new(Name::str("test"), ty.clone());
        assert_eq!(entry.ty, ty);
    }
    #[test]
    fn test_search_stats_clone() {
        let stats = SearchStats {
            instances_examined: 42,
            ..SearchStats::default()
        };
        let cloned = stats.clone();
        assert_eq!(cloned.instances_examined, 42);
    }
    #[test]
    fn test_synth_diagnostics_clone() {
        let diag = SynthDiagnostics {
            failure_reason: FailureReason::Timeout,
            stats: SearchStats::default(),
            tried_candidates: vec![(Name::str("test"), FailureReason::NoInstances)],
        };
        let cloned = diag.clone();
        matches!(cloned.failure_reason, FailureReason::Timeout);
    }
}
#[cfg(test)]
mod synthinstance_analysis_tests {
    use super::*;
    use crate::synth_instance::*;
    #[test]
    fn test_synthinstance_result_ok() {
        let r = SynthInstanceResult::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_synthinstance_result_err() {
        let r = SynthInstanceResult::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_synthinstance_result_partial() {
        let r = SynthInstanceResult::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_synthinstance_result_skipped() {
        let r = SynthInstanceResult::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_synthinstance_analysis_pass_run() {
        let mut p = SynthInstanceAnalysisPass::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_synthinstance_analysis_pass_empty_input() {
        let mut p = SynthInstanceAnalysisPass::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_synthinstance_analysis_pass_success_rate() {
        let mut p = SynthInstanceAnalysisPass::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_synthinstance_analysis_pass_disable() {
        let mut p = SynthInstanceAnalysisPass::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_synthinstance_pipeline_basic() {
        let mut pipeline = SynthInstancePipeline::new("main_pipeline");
        pipeline.add_pass(SynthInstanceAnalysisPass::new("pass1"));
        pipeline.add_pass(SynthInstanceAnalysisPass::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_synthinstance_pipeline_disabled_pass() {
        let mut pipeline = SynthInstancePipeline::new("partial");
        let mut p = SynthInstanceAnalysisPass::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(SynthInstanceAnalysisPass::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_synthinstance_diff_basic() {
        let mut d = SynthInstanceDiff::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_synthinstance_diff_summary() {
        let mut d = SynthInstanceDiff::new();
        d.add("x");
        d.add("y");
        d.remove("z");
        let s = d.summary();
        assert!(s.contains("+2"));
    }
    #[test]
    fn test_synthinstance_config_set_get() {
        let mut cfg = SynthInstanceConfig::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_synthinstance_config_read_only() {
        let mut cfg = SynthInstanceConfig::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_synthinstance_config_remove() {
        let mut cfg = SynthInstanceConfig::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_synthinstance_diagnostics_basic() {
        let mut diag = SynthInstanceDiagnostics::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_synthinstance_diagnostics_max_errors() {
        let mut diag = SynthInstanceDiagnostics::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_synthinstance_diagnostics_clear() {
        let mut diag = SynthInstanceDiagnostics::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_synthinstance_config_value_types() {
        let b = SynthInstanceConfigValue::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = SynthInstanceConfigValue::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = SynthInstanceConfigValue::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = SynthInstanceConfigValue::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = SynthInstanceConfigValue::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
#[cfg(test)]
mod synth_instance_ext_tests_1500 {
    use super::*;
    use crate::synth_instance::*;
    #[test]
    fn test_synth_instance_ext_result_ok_1500() {
        let r = SynthInstanceExtResult1500::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_synth_instance_ext_result_err_1500() {
        let r = SynthInstanceExtResult1500::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_synth_instance_ext_result_partial_1500() {
        let r = SynthInstanceExtResult1500::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_synth_instance_ext_result_skipped_1500() {
        let r = SynthInstanceExtResult1500::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_synth_instance_ext_pass_run_1500() {
        let mut p = SynthInstanceExtPass1500::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_synth_instance_ext_pass_empty_1500() {
        let mut p = SynthInstanceExtPass1500::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_synth_instance_ext_pass_rate_1500() {
        let mut p = SynthInstanceExtPass1500::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_synth_instance_ext_pass_disable_1500() {
        let mut p = SynthInstanceExtPass1500::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_synth_instance_ext_pipeline_basic_1500() {
        let mut pipeline = SynthInstanceExtPipeline1500::new("main_pipeline");
        pipeline.add_pass(SynthInstanceExtPass1500::new("pass1"));
        pipeline.add_pass(SynthInstanceExtPass1500::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_synth_instance_ext_pipeline_disabled_1500() {
        let mut pipeline = SynthInstanceExtPipeline1500::new("partial");
        let mut p = SynthInstanceExtPass1500::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(SynthInstanceExtPass1500::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_synth_instance_ext_diff_basic_1500() {
        let mut d = SynthInstanceExtDiff1500::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_synth_instance_ext_config_set_get_1500() {
        let mut cfg = SynthInstanceExtConfig1500::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_synth_instance_ext_config_read_only_1500() {
        let mut cfg = SynthInstanceExtConfig1500::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_synth_instance_ext_config_remove_1500() {
        let mut cfg = SynthInstanceExtConfig1500::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_synth_instance_ext_diagnostics_basic_1500() {
        let mut diag = SynthInstanceExtDiag1500::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_synth_instance_ext_diagnostics_max_errors_1500() {
        let mut diag = SynthInstanceExtDiag1500::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_synth_instance_ext_diagnostics_clear_1500() {
        let mut diag = SynthInstanceExtDiag1500::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_synth_instance_ext_config_value_types_1500() {
        let b = SynthInstanceExtConfigVal1500::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = SynthInstanceExtConfigVal1500::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = SynthInstanceExtConfigVal1500::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = SynthInstanceExtConfigVal1500::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = SynthInstanceExtConfigVal1500::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
