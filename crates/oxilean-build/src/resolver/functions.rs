//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::manifest::{Dependency, DependencySource, Manifest, Version, VersionConstraint};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};

use super::types::{
    BacktrackingResolver, ConflictAnalyzer, ConflictCause, DepEdge, DependencyAudit,
    DependencyGraph, FeatureUnifier, GitPinned, GitResolver, LockfileGenerator,
    MemoryPackageSource, PackageId, PackageSummary, PathDepResolver, ResolutionCache,
    ResolutionPlan, ResolutionStats, ResolutionSummary, ResolveError, Resolver, VersionRange,
    VersionSelectionStrategy, VersionSet,
};

/// Interface for querying available packages.
pub trait PackageSource {
    /// List all available versions of a package.
    fn available_versions(&self, name: &str) -> Vec<PackageSummary>;
    /// Get a specific version's summary.
    fn get_summary(&self, name: &str, version: &Version) -> Option<PackageSummary>;
}
/// Resolve dependencies for a manifest using a given package source.
pub fn resolve_dependencies<S: PackageSource>(
    manifest: &Manifest,
    source: &S,
) -> Result<DependencyGraph, ResolveError> {
    let mut resolver = Resolver::new(source);
    resolver.resolve(manifest)
}
/// Resolve dependencies using only local path dependencies (no registry).
pub fn resolve_local_dependencies(manifest: &Manifest) -> Result<DependencyGraph, ResolveError> {
    let root = PackageId::root(&manifest.name, manifest.version.clone());
    let mut graph = DependencyGraph::new(root.clone());
    for dep in manifest.dependencies.values() {
        if let DependencySource::Path { ref path } = dep.source {
            let dep_id = PackageId::new(
                &dep.name,
                Version::new(0, 0, 0),
                &path.display().to_string(),
            );
            graph.add_package(dep_id.clone());
            graph.add_edge(DepEdge {
                from: root.clone(),
                to: dep_id,
                constraint: dep.version.clone(),
                optional: dep.optional,
                features: dep.features.clone(),
            });
        }
    }
    Ok(graph)
}
/// Perform an audit on the dependency graph.
pub fn audit_dependencies(graph: &DependencyGraph) -> Vec<DependencyAudit> {
    let mut audits = Vec::new();
    let root_deps: HashSet<String> = graph
        .edges
        .get(&graph.root)
        .map(|edges| edges.iter().map(|e| e.to.name.clone()).collect())
        .unwrap_or_default();
    let mut depths: HashMap<String, usize> = HashMap::new();
    let mut queue: VecDeque<(PackageId, usize)> = VecDeque::new();
    queue.push_back((graph.root.clone(), 0));
    let mut visited: HashSet<String> = HashSet::new();
    while let Some((pkg, depth)) = queue.pop_front() {
        if !visited.insert(pkg.name.clone()) {
            continue;
        }
        depths.insert(pkg.name.clone(), depth);
        if let Some(edges) = graph.edges.get(&pkg) {
            for edge in edges {
                queue.push_back((edge.to.clone(), depth + 1));
            }
        }
    }
    for pkg in graph.packages.values() {
        if pkg == &graph.root {
            continue;
        }
        let dependent_count = graph
            .reverse_edges
            .get(pkg)
            .map(|deps| deps.len())
            .unwrap_or(0);
        let depth = depths.get(&pkg.name).copied().unwrap_or(0);
        audits.push(DependencyAudit {
            name: pkg.name.clone(),
            version: pkg.version.clone(),
            dependent_count,
            is_direct: root_deps.contains(&pkg.name),
            depth,
            license: None,
        });
    }
    audits.sort_by(|a, b| a.name.cmp(&b.name));
    audits
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_version_range_contains() {
        let range = VersionRange::between(Version::new(1, 0, 0), Version::new(2, 0, 0));
        assert!(range.contains(&Version::new(1, 0, 0)));
        assert!(range.contains(&Version::new(1, 5, 0)));
        assert!(!range.contains(&Version::new(2, 0, 0)));
        assert!(!range.contains(&Version::new(0, 9, 0)));
    }
    #[test]
    fn test_version_range_intersect() {
        let r1 = VersionRange::between(Version::new(1, 0, 0), Version::new(3, 0, 0));
        let r2 = VersionRange::between(Version::new(2, 0, 0), Version::new(4, 0, 0));
        let intersection = r1.intersect(&r2).expect("test operation should succeed");
        assert_eq!(intersection.lower, Some(Version::new(2, 0, 0)));
        assert_eq!(intersection.upper, Some(Version::new(3, 0, 0)));
    }
    #[test]
    fn test_version_range_no_overlap() {
        let r1 = VersionRange::between(Version::new(1, 0, 0), Version::new(2, 0, 0));
        let r2 = VersionRange::between(Version::new(3, 0, 0), Version::new(4, 0, 0));
        assert!(r1.intersect(&r2).is_none());
    }
    #[test]
    fn test_dependency_graph_topo_sort() {
        let root = PackageId::root("root", Version::new(1, 0, 0));
        let mut graph = DependencyGraph::new(root.clone());
        let a = PackageId::new("a", Version::new(1, 0, 0), "reg");
        let b = PackageId::new("b", Version::new(1, 0, 0), "reg");
        graph.add_package(a.clone());
        graph.add_package(b.clone());
        graph.add_edge(DepEdge {
            from: root.clone(),
            to: a.clone(),
            constraint: VersionConstraint::Any,
            optional: false,
            features: Vec::new(),
        });
        graph.add_edge(DepEdge {
            from: root,
            to: b.clone(),
            constraint: VersionConstraint::Any,
            optional: false,
            features: Vec::new(),
        });
        graph.add_edge(DepEdge {
            from: a,
            to: b,
            constraint: VersionConstraint::Any,
            optional: false,
            features: Vec::new(),
        });
        let sorted = graph
            .topological_sort()
            .expect("test operation should succeed");
        assert_eq!(sorted.len(), 3);
    }
    #[test]
    fn test_resolve_local_deps() {
        let mut manifest = Manifest::new("test", Version::new(0, 1, 0));
        manifest.add_dependency(Dependency::path(
            "local-dep",
            std::path::Path::new("../local-dep"),
        ));
        let graph = resolve_local_dependencies(&manifest).expect("resolution should succeed");
        assert!(graph.contains("local-dep"));
    }
    #[test]
    fn test_memory_source_resolve() {
        let mut source = MemoryPackageSource::new();
        source.add_summary(PackageSummary {
            name: "foo".to_string(),
            version: Version::new(1, 0, 0),
            dependencies: Vec::new(),
            features: BTreeSet::new(),
            yanked: false,
        });
        let mut manifest = Manifest::new("test", Version::new(0, 1, 0));
        manifest.add_dependency(Dependency::new(
            "foo",
            VersionConstraint::Caret(Version::new(1, 0, 0)),
        ));
        let graph = resolve_dependencies(&manifest, &source).expect("resolution should succeed");
        assert!(graph.contains("foo"));
    }
    #[test]
    fn test_transitive_deps() {
        let root = PackageId::root("root", Version::new(1, 0, 0));
        let mut graph = DependencyGraph::new(root.clone());
        let a = PackageId::new("a", Version::new(1, 0, 0), "reg");
        let b = PackageId::new("b", Version::new(1, 0, 0), "reg");
        graph.add_package(a.clone());
        graph.add_package(b.clone());
        graph.add_edge(DepEdge {
            from: root.clone(),
            to: a.clone(),
            constraint: VersionConstraint::Any,
            optional: false,
            features: Vec::new(),
        });
        graph.add_edge(DepEdge {
            from: a,
            to: b,
            constraint: VersionConstraint::Any,
            optional: false,
            features: Vec::new(),
        });
        let transitive = graph.transitive_deps(&root);
        assert_eq!(transitive.len(), 2);
    }
    #[test]
    fn test_version_set_contains() {
        let vs = VersionSet::from_range(VersionRange::between(
            Version::new(1, 0, 0),
            Version::new(2, 0, 0),
        ));
        assert!(vs.contains(&Version::new(1, 5, 0)));
        assert!(!vs.contains(&Version::new(2, 0, 0)));
        assert!(!vs.contains(&Version::new(0, 9, 0)));
    }
    #[test]
    fn test_version_set_intersect() {
        let vs1 = VersionSet::from_range(VersionRange::between(
            Version::new(1, 0, 0),
            Version::new(3, 0, 0),
        ));
        let vs2 = VersionSet::from_range(VersionRange::between(
            Version::new(2, 0, 0),
            Version::new(4, 0, 0),
        ));
        let intersection = vs1.intersect(&vs2);
        assert!(intersection.contains(&Version::new(2, 5, 0)));
        assert!(!intersection.contains(&Version::new(1, 5, 0)));
        assert!(!intersection.contains(&Version::new(3, 5, 0)));
    }
    #[test]
    fn test_version_set_union() {
        let vs1 = VersionSet::from_range(VersionRange::between(
            Version::new(1, 0, 0),
            Version::new(2, 0, 0),
        ));
        let vs2 = VersionSet::from_range(VersionRange::between(
            Version::new(3, 0, 0),
            Version::new(4, 0, 0),
        ));
        let union = vs1.union(&vs2);
        assert!(union.contains(&Version::new(1, 5, 0)));
        assert!(union.contains(&Version::new(3, 5, 0)));
        assert!(!union.contains(&Version::new(2, 5, 0)));
    }
    #[test]
    fn test_version_set_best_match() {
        let vs = VersionSet::from_range(VersionRange::between(
            Version::new(1, 0, 0),
            Version::new(2, 0, 0),
        ));
        let candidates = vec![
            Version::new(0, 9, 0),
            Version::new(1, 0, 0),
            Version::new(1, 5, 0),
            Version::new(1, 9, 0),
            Version::new(2, 0, 0),
        ];
        let best = vs.best_match(&candidates);
        assert_eq!(best, Some(&Version::new(1, 9, 0)));
    }
    #[test]
    fn test_resolution_summary() {
        let root = PackageId::root("root", Version::new(1, 0, 0));
        let mut graph = DependencyGraph::new(root.clone());
        let a = PackageId::new("a", Version::new(1, 0, 0), "reg");
        let b = PackageId::new("b", Version::new(1, 0, 0), "reg");
        graph.add_package(a.clone());
        graph.add_package(b.clone());
        graph.add_edge(DepEdge {
            from: root.clone(),
            to: a.clone(),
            constraint: VersionConstraint::Any,
            optional: false,
            features: Vec::new(),
        });
        graph.add_edge(DepEdge {
            from: a,
            to: b,
            constraint: VersionConstraint::Any,
            optional: false,
            features: Vec::new(),
        });
        let summary = ResolutionSummary::from_graph(&graph);
        assert_eq!(summary.total_packages, 3);
        assert_eq!(summary.direct_deps, 1);
        assert_eq!(summary.max_depth, 2);
    }
    #[test]
    fn test_dependency_audit() {
        let root = PackageId::root("root", Version::new(1, 0, 0));
        let mut graph = DependencyGraph::new(root.clone());
        let a = PackageId::new("a", Version::new(1, 0, 0), "reg");
        let b = PackageId::new("b", Version::new(1, 0, 0), "reg");
        graph.add_package(a.clone());
        graph.add_package(b.clone());
        graph.add_edge(DepEdge {
            from: root.clone(),
            to: a.clone(),
            constraint: VersionConstraint::Any,
            optional: false,
            features: Vec::new(),
        });
        graph.add_edge(DepEdge {
            from: a,
            to: b,
            constraint: VersionConstraint::Any,
            optional: false,
            features: Vec::new(),
        });
        let audits = audit_dependencies(&graph);
        assert_eq!(audits.len(), 2);
        let a_audit = audits
            .iter()
            .find(|a| a.name == "a")
            .expect("test operation should succeed");
        assert!(a_audit.is_direct);
        let b_audit = audits
            .iter()
            .find(|a| a.name == "b")
            .expect("test operation should succeed");
        assert!(!b_audit.is_direct);
    }
    #[test]
    fn test_conflict_analyzer() {
        let error = ResolveError::new(ConflictCause::NoMatchingVersion {
            package: "missing-pkg".to_string(),
            constraint: VersionConstraint::Caret(Version::new(5, 0, 0)),
        });
        let mut analyzer = ConflictAnalyzer::new();
        let explanations = analyzer.analyze(&error);
        assert!(!explanations.is_empty());
        assert!(explanations[0].contains("missing-pkg"));
    }
    #[test]
    fn test_resolution_plan() {
        let root = PackageId::root("root", Version::new(1, 0, 0));
        let mut graph = DependencyGraph::new(root.clone());
        let a = PackageId::new("a", Version::new(1, 0, 0), "default");
        graph.add_package(a.clone());
        graph.add_edge(DepEdge {
            from: root,
            to: a,
            constraint: VersionConstraint::Any,
            optional: false,
            features: Vec::new(),
        });
        let plan = ResolutionPlan::from_graph(&graph);
        assert_eq!(plan.steps.len(), 1);
    }
    #[test]
    fn test_package_id_display() {
        let id = PackageId::new("foo", Version::new(1, 2, 3), "registry");
        let display = format!("{}", id);
        assert!(display.contains("foo"));
        assert!(display.contains("1.2.3"));
        assert!(display.contains("registry"));
    }
    #[test]
    fn test_graph_edge_count() {
        let root = PackageId::root("root", Version::new(1, 0, 0));
        let mut graph = DependencyGraph::new(root.clone());
        let a = PackageId::new("a", Version::new(1, 0, 0), "reg");
        graph.add_package(a.clone());
        graph.add_edge(DepEdge {
            from: root,
            to: a,
            constraint: VersionConstraint::Any,
            optional: false,
            features: Vec::new(),
        });
        assert_eq!(graph.edge_count(), 1);
        assert_eq!(graph.package_count(), 2);
    }
}
#[cfg(test)]
mod extended_resolver_tests {
    use super::*;
    use crate::manifest::Version;
    #[test]
    fn test_version_selection_maximal() {
        let strategy = VersionSelectionStrategy::Maximal;
        let candidates = vec![
            Version::new(1, 0, 0),
            Version::new(1, 2, 0),
            Version::new(1, 1, 0),
        ];
        let selected = strategy
            .select(&candidates)
            .expect("test operation should succeed");
        assert_eq!(*selected, Version::new(1, 2, 0));
    }
    #[test]
    fn test_version_selection_minimal() {
        let strategy = VersionSelectionStrategy::Minimal;
        let candidates = vec![
            Version::new(1, 0, 0),
            Version::new(1, 2, 0),
            Version::new(1, 1, 0),
        ];
        let selected = strategy
            .select(&candidates)
            .expect("test operation should succeed");
        assert_eq!(*selected, Version::new(1, 0, 0));
    }
    #[test]
    fn test_version_selection_pinned() {
        let strategy = VersionSelectionStrategy::Pinned(Version::new(1, 1, 0));
        let candidates = vec![
            Version::new(1, 0, 0),
            Version::new(1, 1, 0),
            Version::new(1, 2, 0),
        ];
        let selected = strategy
            .select(&candidates)
            .expect("test operation should succeed");
        assert_eq!(*selected, Version::new(1, 1, 0));
    }
    #[test]
    fn test_version_selection_empty() {
        let strategy = VersionSelectionStrategy::Maximal;
        assert!(strategy.select(&[]).is_none());
    }
    #[test]
    fn test_feature_unifier_basic() {
        let mut unifier = FeatureUnifier::new();
        unifier.add_features("serde", &["derive".to_string(), "std".to_string()]);
        unifier.add_features("serde", &["std".to_string(), "alloc".to_string()]);
        let features = unifier.get("serde").expect("key should exist");
        assert!(features.contains("derive"));
        assert!(features.contains("std"));
        assert!(features.contains("alloc"));
        assert_eq!(features.len(), 3);
    }
    #[test]
    fn test_feature_unifier_merge() {
        let mut a = FeatureUnifier::new();
        a.add_features("serde", &["derive".to_string()]);
        let mut b = FeatureUnifier::new();
        b.add_features("serde", &["std".to_string()]);
        b.add_features("tokio", &["full".to_string()]);
        a.merge(&b);
        assert!(a.has_feature("serde", "derive"));
        assert!(a.has_feature("serde", "std"));
        assert!(a.has_feature("tokio", "full"));
    }
    #[test]
    fn test_resolution_stats_avg_candidates() {
        let mut stats = ResolutionStats::zero();
        stats.resolved_packages = 4;
        stats.candidates_evaluated = 12;
        assert!((stats.avg_candidates_per_package() - 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_backtracking_resolver_simple() {
        let mut resolver = BacktrackingResolver::new(VersionSelectionStrategy::Maximal);
        resolver.register("foo", vec![Version::new(1, 0, 0), Version::new(1, 1, 0)]);
        resolver.register("bar", vec![Version::new(2, 0, 0), Version::new(2, 1, 0)]);
        let reqs = vec![
            (
                "foo".to_string(),
                VersionConstraint::Caret(Version::new(1, 0, 0)),
            ),
            ("bar".to_string(), VersionConstraint::Any),
        ];
        let result = resolver.resolve(&reqs);
        assert!(result.is_ok());
        let resolved = result.expect("resolution should succeed");
        assert_eq!(resolved["foo"], Version::new(1, 1, 0));
        assert_eq!(resolved["bar"], Version::new(2, 1, 0));
    }
    #[test]
    fn test_backtracking_resolver_no_match() {
        let mut resolver = BacktrackingResolver::new(VersionSelectionStrategy::Maximal);
        resolver.register("foo", vec![Version::new(1, 0, 0)]);
        let reqs = vec![(
            "foo".to_string(),
            VersionConstraint::Exact(Version::new(2, 0, 0)),
        )];
        assert!(resolver.resolve(&reqs).is_err());
    }
    #[test]
    fn test_git_pinned_valid_commit() {
        let pinned = GitPinned::new("https://github.com/foo/bar", "abc1234").with_reference("main");
        assert!(pinned.is_valid_commit());
        assert_eq!(pinned.short_commit(), "abc1234");
    }
    #[test]
    fn test_git_pinned_invalid_commit() {
        let pinned = GitPinned::new("https://example.com/repo", "xyz!");
        assert!(!pinned.is_valid_commit());
    }
    #[test]
    fn test_git_resolver() {
        let mut resolver = GitResolver::new();
        resolver.register(
            "https://github.com/foo/bar",
            "main",
            "aabbccdd1122334455667788990011223344556677",
        );
        let pinned = resolver.resolve("https://github.com/foo/bar", Some("main"));
        assert!(pinned.is_some());
        let p = pinned.expect("test operation should succeed");
        assert_eq!(p.short_commit(), "aabbccd");
    }
    #[test]
    fn test_path_dep_resolver() {
        let mut resolver = PathDepResolver::new("/workspace");
        resolver.register("crates/kernel", "oxilean-kernel", Version::new(0, 1, 0));
        let resolved = resolver.resolve(std::path::Path::new("crates/kernel"));
        assert!(resolved.is_some());
        let r = resolved.expect("resolution should succeed");
        assert_eq!(r.name, "oxilean-kernel");
        assert_eq!(r.version, Version::new(0, 1, 0));
    }
    #[test]
    fn test_lockfile_generator() {
        let mut gen = LockfileGenerator::new(VersionSelectionStrategy::Maximal);
        gen.register_versions("serde", vec![Version::new(1, 0, 0), Version::new(1, 2, 0)]);
        let mut manifest = crate::manifest::Manifest::new("test", Version::new(0, 1, 0));
        manifest.add_dependency(crate::manifest::Dependency::registry(
            "serde",
            VersionConstraint::Caret(Version::new(1, 0, 0)),
        ));
        let lockfile = gen.generate(&manifest);
        assert!(lockfile.is_ok());
        let lf = lockfile.expect("lockfile generation should succeed");
        let serialized = lf.serialize();
        assert!(serialized.contains("serde"));
    }
    #[test]
    fn test_resolution_cache() {
        let mut cache = ResolutionCache::new();
        cache.insert("serde", Version::new(1, 2, 0));
        assert!(cache.get("serde").is_some());
        assert!(cache.get("nonexistent").is_none());
        assert!((cache.hit_ratio() - 0.5).abs() < 1e-9);
    }
    #[test]
    fn test_resolution_cache_invalidate() {
        let mut cache = ResolutionCache::new();
        cache.insert("tokio", Version::new(1, 0, 0));
        assert!(cache.invalidate("tokio"));
        assert!(!cache.invalidate("tokio"));
        assert_eq!(cache.size(), 0);
    }
    #[test]
    fn test_version_selection_strategy_display() {
        assert_eq!(format!("{}", VersionSelectionStrategy::Maximal), "maximal");
        assert_eq!(format!("{}", VersionSelectionStrategy::Minimal), "minimal");
    }
}
/// Returns the resolver subsystem version string.
pub fn resolver_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
/// Returns the maximum supported dependency depth for cycle detection.
pub fn max_dependency_depth() -> usize {
    256
}
#[cfg(test)]
mod resolver_version_tests {
    use super::*;
    #[test]
    fn resolver_version_nonempty() {
        assert!(!resolver_version().is_empty());
    }
    #[test]
    fn max_depth_positive() {
        assert!(max_dependency_depth() > 0);
    }
}
/// Returns whether the resolver supports pre-release versions.
pub fn resolver_supports_pre_release() -> bool {
    true
}
/// Returns whether the resolver supports version ranges.
pub fn resolver_supports_ranges() -> bool {
    true
}
#[cfg(test)]
mod resolver_feature_tests {
    use super::*;
    #[test]
    fn pre_release_supported() {
        assert!(resolver_supports_pre_release());
    }
    #[test]
    fn ranges_supported() {
        assert!(resolver_supports_ranges());
    }
}
