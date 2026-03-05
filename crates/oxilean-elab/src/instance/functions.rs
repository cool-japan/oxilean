//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{Expr, Name};

use super::types::{
    CacheKey, DiamondResolutionStrategy, InstanceCache, InstanceChain, InstanceChainCache,
    InstanceDecl, InstanceError, InstanceGraph, InstanceMatcher, InstancePriorityQueue,
    InstanceRegistryStats, InstanceReport, InstanceResolutionTrace, InstanceResolver,
    InstanceScope, InstanceScopeStack, InstanceSearchState, InstanceSnapshot, InstanceSynthesizer,
    LocalInstanceScope, MatchOutcome, PriorityGroup, ResolutionResult, SearchPath, SearchStats,
    SynthConfig, SynthResult, TracedResolver, TypeclassInstance,
};

/// Default priority assigned to instances when none is specified.
pub const DEFAULT_PRIORITY: u32 = 100;
/// Priority given to local (let-bound) instances.
pub const LOCAL_INSTANCE_PRIORITY: u32 = 0;
/// Priority given to globally imported instances.
pub const GLOBAL_INSTANCE_PRIORITY: u32 = 100;
/// Priority for derived instances (e.g. `derive`).
pub const DERIVED_INSTANCE_PRIORITY: u32 = 200;
/// Compare two instance declarations by priority (lower = higher priority).
///
/// Returns `std::cmp::Ordering::Less` when `a` has higher priority than `b`.
pub fn compare_priority(a: &InstanceDecl, b: &InstanceDecl) -> std::cmp::Ordering {
    a.priority.cmp(&b.priority)
}
/// Select the highest-priority instance from a non-empty list.
///
/// Returns `None` if `instances` is empty.
pub fn select_best(instances: &[InstanceDecl]) -> Option<&InstanceDecl> {
    instances.iter().min_by(|a, b| compare_priority(a, b))
}
/// Partition instances into those with the minimum priority and the rest.
///
/// Used to detect ambiguity: if the best partition has more than one entry,
/// the resolution is ambiguous.
pub fn partition_by_best_priority(
    instances: &[InstanceDecl],
) -> (&[InstanceDecl], &[InstanceDecl]) {
    if instances.is_empty() {
        return (&[], &[]);
    }
    let best_priority = instances
        .iter()
        .map(|i| i.priority)
        .min()
        .unwrap_or(u32::MAX);
    let boundary = instances
        .iter()
        .position(|i| i.priority != best_priority)
        .unwrap_or(instances.len());
    (&instances[..boundary], &instances[boundary..])
}
/// Check whether `candidate` structurally matches `query`.
///
/// This is a conservative syntactic check; a real implementation would
/// use full unification.  Returning `true` when uncertain is safe
/// (over-approximates the candidate set).
pub fn structural_match(candidate: &Expr, query: &Expr) -> bool {
    match (candidate, query) {
        (Expr::Const(n1, _), Expr::Const(n2, _)) => n1 == n2,
        (Expr::App(f1, _), Expr::App(f2, _)) => structural_match(f1, f2),
        _ => true,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::instance::*;
    use oxilean_kernel::Literal;
    fn make_inst(name: &str, class: &str, ty: Expr, priority: u32) -> InstanceDecl {
        InstanceDecl {
            name: Name::str(name),
            class: Name::str(class),
            ty,
            priority,
        }
    }
    fn nat_ty() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    fn bool_ty() -> Expr {
        Expr::Const(Name::str("Bool"), vec![])
    }
    #[test]
    fn test_resolver_create() {
        let resolver = InstanceResolver::new();
        assert_eq!(resolver.max_depth(), 10);
        assert_eq!(resolver.total_registered(), 0);
        assert_eq!(resolver.class_count(), 0);
    }
    #[test]
    fn test_register_instance() {
        let mut resolver = InstanceResolver::new();
        resolver.register(make_inst("Eq.Nat", "Eq", nat_ty(), DEFAULT_PRIORITY));
        assert_eq!(resolver.get_instances(&Name::str("Eq")).len(), 1);
        assert_eq!(resolver.total_registered(), 1);
    }
    #[test]
    fn test_find_instance() {
        let mut resolver = InstanceResolver::new();
        resolver.register(make_inst("Eq.Nat", "Eq", nat_ty(), DEFAULT_PRIORITY));
        let found = resolver.find_instance(&Name::str("Eq"), &nat_ty());
        assert!(found.is_some());
    }
    #[test]
    fn test_find_instance_none() {
        let mut resolver = InstanceResolver::new();
        let ty = Expr::Lit(Literal::Nat(42));
        let found = resolver.find_instance(&Name::str("Unknown"), &ty);
        assert!(found.is_none());
    }
    #[test]
    fn test_resolve_found() {
        let mut resolver = InstanceResolver::new();
        resolver.register(make_inst("Eq.Nat", "Eq", nat_ty(), DEFAULT_PRIORITY));
        let result = resolver.resolve(&Name::str("Eq"), &nat_ty());
        assert!(matches!(result, ResolutionResult::Found(_)));
    }
    #[test]
    fn test_resolve_not_found() {
        let mut resolver = InstanceResolver::new();
        let result = resolver.resolve(&Name::str("Eq"), &nat_ty());
        assert_eq!(result, ResolutionResult::NotFound);
    }
    #[test]
    fn test_priority_ordering() {
        let a = make_inst("A", "C", nat_ty(), 10);
        let b = make_inst("B", "C", nat_ty(), 20);
        assert_eq!(compare_priority(&a, &b), std::cmp::Ordering::Less);
    }
    #[test]
    fn test_select_best() {
        let instances = vec![
            make_inst("A", "C", nat_ty(), 20),
            make_inst("B", "C", nat_ty(), 5),
        ];
        let best = select_best(&instances).expect("instance resolution should succeed");
        assert_eq!(best.priority, 5);
    }
    #[test]
    fn test_select_best_empty() {
        let instances: Vec<InstanceDecl> = vec![];
        assert!(select_best(&instances).is_none());
    }
    #[test]
    fn test_local_scope() {
        let mut scope = LocalInstanceScope::new();
        assert!(scope.is_empty());
        scope.push_layer();
        let added = scope.add_to_current(make_inst("I", "C", nat_ty(), 0));
        assert!(added);
        assert_eq!(scope.visible_instances().len(), 1);
        let popped = scope.pop_layer();
        assert_eq!(popped.len(), 1);
        assert!(scope.is_empty());
    }
    #[test]
    fn test_local_scope_nested() {
        let mut scope = LocalInstanceScope::new();
        scope.push_layer();
        scope.add_to_current(make_inst("I1", "C", nat_ty(), 0));
        scope.push_layer();
        scope.add_to_current(make_inst("I2", "C", bool_ty(), 0));
        assert_eq!(scope.visible_instances().len(), 2);
        assert_eq!(scope.depth(), 2);
        scope.pop_layer();
        assert_eq!(scope.visible_instances().len(), 1);
    }
    #[test]
    fn test_cache_key() {
        let k1 = CacheKey::new(&Name::str("Eq"), &nat_ty());
        let k2 = CacheKey::new(&Name::str("Eq"), &nat_ty());
        assert_eq!(k1, k2);
    }
    #[test]
    fn test_cache_hit_miss() {
        let mut cache = InstanceCache::new();
        let key = CacheKey::new(&Name::str("Eq"), &nat_ty());
        assert!(cache.get(&key).is_none());
        assert_eq!(cache.misses(), 1);
        cache.insert(key.clone(), Name::str("Eq.Nat"));
        cache.get(&key);
        assert_eq!(cache.hits(), 1);
    }
    #[test]
    fn test_stats_recording() {
        let mut stats = SearchStats::new();
        stats.record_success(3);
        stats.record_failure();
        assert_eq!(stats.total_queries(), 2);
        assert!((stats.success_rate() - 0.5).abs() < 1e-9);
    }
    #[test]
    fn test_traced_resolver() {
        let mut traced = TracedResolver::new();
        traced.register(make_inst("Eq.Nat", "Eq", nat_ty(), DEFAULT_PRIORITY));
        traced.resolve(&Name::str("Eq"), &nat_ty());
        assert_eq!(traced.stats.successes, 1);
        traced.resolve(&Name::str("Ord"), &nat_ty());
        assert_eq!(traced.stats.failures, 1);
    }
    #[test]
    fn test_register_many() {
        let mut resolver = InstanceResolver::new();
        let instances = vec![
            make_inst("A", "C", nat_ty(), 10),
            make_inst("B", "C", bool_ty(), 20),
        ];
        resolver.register_many(instances);
        assert_eq!(resolver.total_registered(), 2);
    }
    #[test]
    fn test_has_instances_for() {
        let mut resolver = InstanceResolver::new();
        assert!(!resolver.has_instances_for(&Name::str("Eq")));
        resolver.register(make_inst("Eq.Nat", "Eq", nat_ty(), DEFAULT_PRIORITY));
        assert!(resolver.has_instances_for(&Name::str("Eq")));
    }
    #[test]
    fn test_clear_class() {
        let mut resolver = InstanceResolver::new();
        resolver.register(make_inst("Eq.Nat", "Eq", nat_ty(), DEFAULT_PRIORITY));
        resolver.clear_class(&Name::str("Eq"));
        assert!(!resolver.has_instances_for(&Name::str("Eq")));
    }
    #[test]
    fn test_resolve_or_error_ok() {
        let mut resolver = InstanceResolver::new();
        resolver.register(make_inst("Eq.Nat", "Eq", nat_ty(), DEFAULT_PRIORITY));
        let result = resolver.resolve_or_error(&Name::str("Eq"), &nat_ty());
        assert!(result.is_ok());
    }
    #[test]
    fn test_resolve_or_error_not_found() {
        let mut resolver = InstanceResolver::new();
        let result = resolver.resolve_or_error(&Name::str("Eq"), &nat_ty());
        assert!(matches!(result, Err(InstanceError::NotFound { .. })));
    }
}
/// Group instances by priority.
pub fn group_by_priority(instances: &[InstanceDecl]) -> Vec<PriorityGroup> {
    let mut groups: Vec<PriorityGroup> = Vec::new();
    for inst in instances {
        if let Some(g) = groups.iter_mut().find(|g| g.priority == inst.priority) {
            g.add(inst.clone());
        } else {
            let mut g = PriorityGroup::new(inst.priority);
            g.add(inst.clone());
            groups.push(g);
        }
    }
    groups.sort_by_key(|g| g.priority);
    groups
}
/// Check if two instance declarations could conflict (same class, overlapping types).
pub fn could_conflict(a: &InstanceDecl, b: &InstanceDecl) -> bool {
    a.class == b.class && a.ty == b.ty
}
/// Return instances for a class sorted by priority (ascending — lower = higher priority).
pub fn sorted_instances(instances: &[InstanceDecl]) -> Vec<InstanceDecl> {
    let mut result = instances.to_vec();
    result.sort_by_key(|i| i.priority);
    result
}
/// Validate that all instances in a list have non-overlapping types at the same priority.
pub fn validate_instances(instances: &[InstanceDecl]) -> Vec<String> {
    let mut warnings = Vec::new();
    for i in 0..instances.len() {
        for j in (i + 1)..instances.len() {
            if could_conflict(&instances[i], &instances[j]) {
                warnings.push(format!(
                    "Instances '{}' and '{}' may conflict for class '{}'",
                    instances[i].name, instances[j].name, instances[i].class
                ));
            }
        }
    }
    warnings
}
#[cfg(test)]
mod instance_extended_tests {
    use super::*;
    use crate::instance::*;
    fn nat_ty() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    fn bool_ty() -> Expr {
        Expr::Const(Name::str("Bool"), vec![])
    }
    fn make_inst(name: &str, class: &str, ty: Expr, priority: u32) -> InstanceDecl {
        InstanceDecl {
            name: Name::str(name),
            class: Name::str(class),
            ty,
            priority,
        }
    }
    #[test]
    fn test_instance_snapshot() {
        let snap = InstanceSnapshot::new(
            Name::str("Eq"),
            vec![make_inst("Eq.Nat", "Eq", nat_ty(), 100)],
            5,
        );
        assert_eq!(snap.candidate_count(), 1);
        assert_eq!(snap.cache_size, 5);
    }
    #[test]
    fn test_search_path_push_has_class() {
        let mut path = SearchPath::new();
        path.push(Name::str("Eq"), Name::str("Eq.Nat"));
        assert!(path.has_class(&Name::str("Eq")));
        assert!(!path.has_class(&Name::str("Ord")));
    }
    #[test]
    fn test_search_path_len() {
        let mut path = SearchPath::new();
        assert_eq!(path.len(), 0);
        path.push(Name::str("A"), Name::str("a"));
        path.push(Name::str("B"), Name::str("b"));
        assert_eq!(path.len(), 2);
    }
    #[test]
    fn test_search_path_clear() {
        let mut path = SearchPath::new();
        path.push(Name::str("X"), Name::str("x"));
        path.clear();
        assert!(path.is_empty());
    }
    #[test]
    fn test_priority_group_unambiguous() {
        let mut g = PriorityGroup::new(100);
        g.add(make_inst("I1", "C", nat_ty(), 100));
        assert!(g.is_unambiguous());
        g.add(make_inst("I2", "C", bool_ty(), 100));
        assert!(!g.is_unambiguous());
    }
    #[test]
    fn test_group_by_priority() {
        let instances = vec![
            make_inst("A", "C", nat_ty(), 10),
            make_inst("B", "C", bool_ty(), 20),
            make_inst("C", "C", nat_ty(), 10),
        ];
        let groups = group_by_priority(&instances);
        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0].priority, 10);
        assert_eq!(groups[0].len(), 2);
    }
    #[test]
    fn test_could_conflict_same() {
        let a = make_inst("A", "C", nat_ty(), 10);
        let b = make_inst("B", "C", nat_ty(), 20);
        assert!(could_conflict(&a, &b));
    }
    #[test]
    fn test_could_conflict_different_class() {
        let a = make_inst("A", "C1", nat_ty(), 10);
        let b = make_inst("B", "C2", nat_ty(), 10);
        assert!(!could_conflict(&a, &b));
    }
    #[test]
    fn test_sorted_instances() {
        let instances = vec![
            make_inst("A", "C", nat_ty(), 30),
            make_inst("B", "C", bool_ty(), 10),
            make_inst("D", "C", nat_ty(), 20),
        ];
        let sorted = sorted_instances(&instances);
        assert_eq!(sorted[0].priority, 10);
        assert_eq!(sorted[2].priority, 30);
    }
    #[test]
    fn test_validate_instances_no_conflict() {
        let instances = vec![
            make_inst("A", "C", nat_ty(), 10),
            make_inst("B", "C", bool_ty(), 20),
        ];
        let warnings = validate_instances(&instances);
        assert!(warnings.is_empty());
    }
    #[test]
    fn test_validate_instances_conflict() {
        let instances = vec![
            make_inst("A", "C", nat_ty(), 10),
            make_inst("B", "C", nat_ty(), 10),
        ];
        let warnings = validate_instances(&instances);
        assert!(!warnings.is_empty());
    }
    #[test]
    fn test_search_path_get() {
        let mut path = SearchPath::new();
        path.push(Name::str("Eq"), Name::str("Eq.Nat"));
        let step = path.get(0).expect("key should exist");
        assert_eq!(step.0, Name::str("Eq"));
        assert_eq!(step.1, Name::str("Eq.Nat"));
    }
    #[test]
    fn test_instance_snapshot_class() {
        let snap = InstanceSnapshot::new(Name::str("Ord"), vec![], 0);
        assert_eq!(snap.class, Name::str("Ord"));
        assert_eq!(snap.candidate_count(), 0);
    }
}
#[cfg(test)]
mod extended_tests {
    use super::*;
    use crate::instance::*;
    fn make_name(s: &str) -> Name {
        Name::str(s)
    }
    fn make_decl(name: &str, class: &str, priority: u32) -> InstanceDecl {
        InstanceDecl {
            name: make_name(name),
            class: make_name(class),
            ty: oxilean_kernel::Expr::BVar(0),
            priority,
        }
    }
    #[test]
    fn test_typeclass_instance_basic() {
        let decl = make_decl("MyInst", "MyClass", 10);
        let inst = TypeclassInstance::new(decl.clone());
        assert_eq!(inst.priority(), 10);
        assert!(!inst.is_default);
        assert!(!inst.is_local);
    }
    #[test]
    fn test_typeclass_instance_default_local() {
        let decl = make_decl("inst", "cls", 5);
        let inst = TypeclassInstance::new(decl).default_instance().local();
        assert!(inst.is_default);
        assert!(inst.is_local);
    }
    #[test]
    fn test_typeclass_instance_dependencies() {
        let decl = make_decl("inst", "cls", 5);
        let mut inst = TypeclassInstance::new(decl);
        inst.add_dependency(make_name("dep1"));
        inst.add_dependency(make_name("dep2"));
        assert_eq!(inst.dependencies.len(), 2);
    }
    #[test]
    fn test_instance_priority_queue_push_pop() {
        let mut queue = InstancePriorityQueue::new();
        assert!(queue.is_empty());
        queue.push(TypeclassInstance::new(make_decl("inst_high", "cls", 1)));
        queue.push(TypeclassInstance::new(make_decl("inst_low", "cls", 100)));
        queue.push(TypeclassInstance::new(make_decl("inst_mid", "cls", 50)));
        assert_eq!(queue.len(), 3);
        let best = queue.pop_best().expect("test operation should succeed");
        assert_eq!(best.priority(), 1);
        let next = queue.pop_best().expect("test operation should succeed");
        assert_eq!(next.priority(), 50);
    }
    #[test]
    fn test_instance_priority_queue_best_candidates() {
        let mut queue = InstancePriorityQueue::new();
        queue.push(TypeclassInstance::new(make_decl("a", "cls", 5)));
        queue.push(TypeclassInstance::new(make_decl("b", "cls", 5)));
        queue.push(TypeclassInstance::new(make_decl("c", "cls", 10)));
        let best = queue.best_candidates();
        assert_eq!(best.len(), 2);
        assert!(best.iter().all(|c| c.priority() == 5));
    }
    #[test]
    fn test_instance_priority_queue_clear() {
        let mut queue = InstancePriorityQueue::new();
        queue.push(TypeclassInstance::new(make_decl("inst", "cls", 5)));
        queue.clear();
        assert!(queue.is_empty());
    }
    #[test]
    fn test_instance_chain_basic() {
        let chain = InstanceChain::single(make_name("inst1"), 5);
        assert_eq!(chain.len(), 1);
        assert_eq!(chain.total_cost, 5);
        assert!(!chain.has_default);
        let chain2 = chain.extend(make_name("inst2"), 3, true);
        assert_eq!(chain2.len(), 2);
        assert_eq!(chain2.total_cost, 8);
        assert!(chain2.has_default);
        let leaf = chain2.leaf().expect("test operation should succeed");
        assert_eq!(*leaf, make_name("inst2"));
    }
    #[test]
    fn test_instance_chain_empty() {
        let chain = InstanceChain::empty();
        assert!(chain.is_empty());
        assert!(chain.leaf().is_none());
    }
    #[test]
    fn test_instance_graph_edges() {
        let mut graph = InstanceGraph::new();
        graph.add_edge(make_name("A"), make_name("B"));
        graph.add_edge(make_name("A"), make_name("C"));
        graph.add_edge(make_name("B"), make_name("C"));
        assert!(graph.has_edge(&make_name("A"), &make_name("B")));
        assert!(!graph.has_edge(&make_name("B"), &make_name("A")));
        assert_eq!(graph.dependencies(&make_name("A")).len(), 2);
        assert_eq!(graph.dependencies(&make_name("Z")).len(), 0);
    }
    #[test]
    fn test_instance_graph_no_cycle() {
        let mut graph = InstanceGraph::new();
        graph.add_edge(make_name("A"), make_name("B"));
        graph.add_edge(make_name("B"), make_name("C"));
        assert!(!graph.has_cycle());
    }
    #[test]
    fn test_instance_graph_has_cycle() {
        let mut graph = InstanceGraph::new();
        graph.add_edge(make_name("A"), make_name("B"));
        graph.add_edge(make_name("B"), make_name("C"));
        graph.add_edge(make_name("C"), make_name("A"));
        assert!(graph.has_cycle());
    }
    #[test]
    fn test_instance_search_state() {
        let mut state = InstanceSearchState::new();
        assert_eq!(state.depth, 0);
        assert_eq!(state.nodes_explored, 0);
        state.enter_depth();
        state.enter_depth();
        assert_eq!(state.depth, 2);
        assert_eq!(state.max_depth_reached, 2);
        assert_eq!(state.nodes_explored, 2);
        state.exit_depth();
        assert_eq!(state.depth, 1);
        state.mark_tried(make_name("inst1"));
        assert!(state.already_tried(&make_name("inst1")));
        assert!(!state.already_tried(&make_name("inst2")));
    }
    #[test]
    fn test_instance_search_state_depth_limit() {
        let mut state = InstanceSearchState::new();
        for _ in 0..10 {
            state.enter_depth();
        }
        assert!(state.depth_exceeded(5));
        assert!(!state.depth_exceeded(15));
    }
    #[test]
    fn test_diamond_resolution_strategy_default() {
        let strategy = DiamondResolutionStrategy::default();
        assert_eq!(strategy, DiamondResolutionStrategy::PreferLowestPriority);
    }
    #[test]
    fn test_instance_priority_queue_peek() {
        let mut queue = InstancePriorityQueue::new();
        assert!(queue.peek_best().is_none());
        queue.push(TypeclassInstance::new(make_decl("inst", "cls", 7)));
        let peek = queue.peek_best().expect("test operation should succeed");
        assert_eq!(peek.priority(), 7);
        assert_eq!(queue.len(), 1);
    }
    #[test]
    fn test_instance_graph_nodes() {
        let mut graph = InstanceGraph::new();
        graph.add_edge(make_name("A"), make_name("B"));
        graph.add_edge(make_name("C"), make_name("D"));
        let nodes = graph.nodes();
        assert_eq!(nodes.len(), 2);
    }
}
#[cfg(test)]
mod synth_and_cache_tests {
    use super::*;
    use crate::instance::*;
    fn make_name(s: &str) -> Name {
        Name::str(s)
    }
    #[test]
    fn test_synth_result_success() {
        let chain = InstanceChain::single(make_name("inst1"), 5);
        let result = SynthResult::Success(chain);
        assert!(result.is_success());
        assert!(!result.is_failure());
        assert!(!result.is_ambiguous());
        assert!(result.chain().is_some());
    }
    #[test]
    fn test_synth_result_failure() {
        let err = InstanceError::NotFound {
            class: make_name("MyClass"),
        };
        let result = SynthResult::Failure(err);
        assert!(result.is_failure());
        assert!(result.chain().is_none());
    }
    #[test]
    fn test_synth_result_ambiguous() {
        let c1 = InstanceChain::single(make_name("i1"), 5);
        let c2 = InstanceChain::single(make_name("i2"), 5);
        let result = SynthResult::Ambiguous(vec![c1, c2]);
        assert!(result.is_ambiguous());
        assert!(result.chain().is_none());
    }
    #[test]
    fn test_synth_config_default() {
        let config = SynthConfig::default();
        assert_eq!(config.max_depth, 32);
        assert!(config.allow_defaults);
        assert_eq!(
            config.diamond_strategy,
            DiamondResolutionStrategy::PreferLowestPriority
        );
    }
    #[test]
    fn test_instance_synthesizer_new() {
        let synth = InstanceSynthesizer::new();
        assert_eq!(synth.config().max_depth, 32);
        assert!(!synth.has_circular_deps());
    }
    #[test]
    fn test_instance_synthesizer_add_deps() {
        let mut synth = InstanceSynthesizer::new();
        synth.add_dependency(make_name("A"), make_name("B"));
        synth.add_dependency(make_name("B"), make_name("C"));
        assert!(!synth.has_circular_deps());
        synth.add_dependency(make_name("C"), make_name("A"));
        assert!(synth.has_circular_deps());
    }
    #[test]
    fn test_instance_cache_store_lookup() {
        let mut cache = InstanceChainCache::new();
        assert!(cache.is_empty());
        let chain = InstanceChain::single(make_name("inst"), 10);
        cache.store("MyClass.Nat", chain);
        let found = cache.lookup("MyClass.Nat");
        assert!(found.is_some());
        assert_eq!(found.expect("test operation should succeed").total_cost, 10);
        assert_eq!(cache.total_hits(), 1);
        let not_found = cache.lookup("OtherClass.Bool");
        assert!(not_found.is_none());
    }
    #[test]
    fn test_instance_cache_clear() {
        let mut cache = InstanceChainCache::new();
        cache.store("key", InstanceChain::single(make_name("i"), 1));
        cache.clear();
        assert!(cache.is_empty());
        assert_eq!(cache.total_hits(), 0);
    }
    #[test]
    fn test_instance_cache_hit_count() {
        let mut cache = InstanceChainCache::new();
        cache.store("key", InstanceChain::single(make_name("i"), 1));
        cache.lookup("key");
        cache.lookup("key");
        cache.lookup("key");
        assert_eq!(cache.total_hits(), 3);
    }
}
#[cfg(test)]
mod stats_tests {
    use super::*;
    use crate::instance::*;
    #[test]
    fn test_instance_registry_stats() {
        let mut stats = InstanceRegistryStats::new();
        stats.record_success(3);
        stats.record_success(5);
        stats.record_failure();
        stats.record_ambiguous();
        assert_eq!(stats.total(), 4);
        assert_eq!(stats.successes, 2);
        assert!((stats.success_rate() - 0.5).abs() < 1e-9);
        assert!((stats.average_depth() - 4.0).abs() < 1e-9);
    }
    #[test]
    fn test_instance_registry_stats_empty() {
        let stats = InstanceRegistryStats::new();
        assert_eq!(stats.success_rate(), 0.0);
        assert_eq!(stats.average_depth(), 0.0);
        assert_eq!(stats.total(), 0);
    }
    #[test]
    fn test_priority_queue_all_candidates() {
        let mut queue = InstancePriorityQueue::new();
        for i in 0..5u32 {
            let decl = InstanceDecl {
                name: oxilean_kernel::Name::str(format!("inst{}", i)),
                class: oxilean_kernel::Name::str("C"),
                ty: oxilean_kernel::Expr::BVar(0),
                priority: i * 10,
            };
            queue.push(TypeclassInstance::new(decl));
        }
        let all = queue.all_candidates();
        assert_eq!(all.len(), 5);
        for i in 0..4 {
            assert!(all[i].priority() <= all[i + 1].priority());
        }
    }
    #[test]
    fn test_synth_config_custom() {
        let config = SynthConfig {
            max_depth: 10,
            max_instances: 100,
            allow_defaults: false,
            diamond_strategy: DiamondResolutionStrategy::Strict,
        };
        let synth = InstanceSynthesizer::with_config(config);
        assert_eq!(synth.config().max_depth, 10);
        assert!(!synth.config().allow_defaults);
        assert_eq!(
            synth.config().diamond_strategy,
            DiamondResolutionStrategy::Strict
        );
    }
}
#[cfg(test)]
mod scope_tests {
    use super::*;
    use crate::instance::*;
    fn make_name(s: &str) -> Name {
        Name::str(s)
    }
    fn make_inst(name: &str, class: &str, priority: u32) -> TypeclassInstance {
        TypeclassInstance::new(InstanceDecl {
            name: make_name(name),
            class: make_name(class),
            ty: oxilean_kernel::Expr::BVar(0),
            priority,
        })
    }
    #[test]
    fn test_instance_scope_basic() {
        let mut scope = InstanceScope::new();
        assert!(scope.is_active());
        assert!(scope.is_empty());
        scope.add(make_inst("inst1", "MyClass", 5));
        scope.add(make_inst("inst2", "OtherClass", 3));
        assert_eq!(scope.len(), 2);
        let for_class = scope.instances_for_class(&make_name("MyClass"));
        assert_eq!(for_class.len(), 1);
        assert_eq!(for_class[0].name(), &make_name("inst1"));
    }
    #[test]
    fn test_instance_scope_deactivate() {
        let mut scope = InstanceScope::new();
        scope.add(make_inst("inst", "cls", 5));
        scope.deactivate();
        assert!(!scope.is_active());
        let result = scope.instances_for_class(&make_name("cls"));
        assert!(result.is_empty());
    }
    #[test]
    fn test_instance_scope_stack() {
        let mut stack = InstanceScopeStack::new();
        assert!(stack.is_empty());
        stack.push_scope();
        stack.add_to_top(make_inst("inst1", "MyClass", 5));
        stack.push_scope();
        stack.add_to_top(make_inst("inst2", "MyClass", 3));
        assert_eq!(stack.depth(), 2);
        let instances = stack.local_instances_for_class(&make_name("MyClass"));
        assert_eq!(instances.len(), 2);
        assert_eq!(instances[0].name(), &make_name("inst2"));
        assert_eq!(instances[1].name(), &make_name("inst1"));
        let _ = stack.pop_scope();
        assert_eq!(stack.depth(), 1);
        let after_pop = stack.local_instances_for_class(&make_name("MyClass"));
        assert_eq!(after_pop.len(), 1);
    }
    #[test]
    fn test_instance_scope_stack_add_to_top_when_empty() {
        let mut stack = InstanceScopeStack::new();
        stack.add_to_top(make_inst("inst", "cls", 5));
        assert!(stack.is_empty());
    }
    #[test]
    fn test_instance_scope_stack_pop_empty() {
        let mut stack = InstanceScopeStack::new();
        let result = stack.pop_scope();
        assert!(result.is_none());
    }
}
#[cfg(test)]
mod matcher_tests {
    use super::*;
    use crate::instance::*;
    fn make_name(s: &str) -> Name {
        Name::str(s)
    }
    fn make_inst(name: &str, class: &str) -> TypeclassInstance {
        TypeclassInstance::new(InstanceDecl {
            name: make_name(name),
            class: make_name(class),
            ty: oxilean_kernel::Expr::BVar(0),
            priority: 0,
        })
    }
    #[test]
    fn test_instance_matcher_basic() {
        let matcher = InstanceMatcher::new();
        let class1 = make_name("MyClass");
        let class2 = make_name("OtherClass");
        assert_eq!(matcher.match_class(&class1, &class1), MatchOutcome::Match);
        assert_eq!(matcher.match_class(&class1, &class2), MatchOutcome::NoMatch);
    }
    #[test]
    fn test_instance_matcher_filter() {
        let matcher = InstanceMatcher::new();
        let instances = vec![
            make_inst("inst1", "ClassA"),
            make_inst("inst2", "ClassA"),
            make_inst("inst3", "ClassB"),
        ];
        let filtered = matcher.filter_candidates(&make_name("ClassA"), &instances);
        assert_eq!(filtered.len(), 2);
        let empty = matcher.filter_candidates(&make_name("ClassC"), &instances);
        assert!(empty.is_empty());
    }
    #[test]
    fn test_match_outcome_variants() {
        assert_eq!(MatchOutcome::Match, MatchOutcome::Match);
        assert_ne!(MatchOutcome::Match, MatchOutcome::NoMatch);
        assert_ne!(MatchOutcome::Deferred, MatchOutcome::NoMatch);
    }
}
#[cfg(test)]
mod trace_tests {
    use super::*;
    use crate::instance::*;
    fn make_name(s: &str) -> Name {
        Name::str(s)
    }
    #[test]
    fn test_instance_resolution_trace() {
        let mut trace = InstanceResolutionTrace::enabled();
        assert!(trace.is_empty());
        trace.log(make_name("MyClass"), make_name("inst1"), "success", 0);
        trace.log(make_name("MyClass"), make_name("inst2"), "failure", 0);
        trace.log(make_name("OtherClass"), make_name("inst3"), "success", 1);
        assert_eq!(trace.len(), 3);
        let for_my_class = trace.entries_for_class(&make_name("MyClass"));
        assert_eq!(for_my_class.len(), 2);
        trace.clear();
        assert!(trace.is_empty());
    }
    #[test]
    fn test_trace_disabled() {
        let mut trace = InstanceResolutionTrace::new();
        assert!(!trace.enabled);
        trace.log(make_name("C"), make_name("i"), "attempt", 0);
        assert!(trace.is_empty());
    }
    #[test]
    fn test_trace_enable_disable() {
        let mut trace = InstanceResolutionTrace::new();
        trace.enable();
        trace.log(make_name("C"), make_name("i"), "attempt", 0);
        assert_eq!(trace.len(), 1);
        trace.disable();
        trace.log(make_name("C"), make_name("i2"), "attempt", 0);
        assert_eq!(trace.len(), 1);
    }
}
#[cfg(test)]
mod report_tests {
    use super::*;
    use crate::instance::*;
    fn make_name(s: &str) -> Name {
        Name::str(s)
    }
    #[test]
    fn test_instance_report_success() {
        let chain = InstanceChain::single(make_name("inst"), 5);
        let report = InstanceReport::success(make_name("MyClass"), chain, 3);
        assert!(report.is_success());
        assert!(report.error.is_none());
        assert_eq!(report.instances_explored, 3);
    }
    #[test]
    fn test_instance_report_failure() {
        let err = InstanceError::NotFound {
            class: make_name("MyClass"),
        };
        let report = InstanceReport::failure(make_name("MyClass"), err, 10);
        assert!(!report.is_success());
        assert!(report.chain.is_none());
        assert!(report.error.is_some());
    }
}
