//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::framework::{
    LintCategory, LintContext, LintDiagnostic, LintId, LintRule, Severity, SourceRange,
};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use super::types::{
    BuiltinPlugin, CompatibilityChecker, DependencyResolver, ExampleCustomPlugin, HotReloadManager,
    HotReloadState, PluginAuditLog, PluginCapabilitySet, PluginCatalog, PluginCompatibilityMatrix,
    PluginConfig, PluginDependency, PluginDependencyGraph, PluginDiscovery, PluginDiscoveryConfig,
    PluginEvent, PluginEventLog, PluginHealthMonitor, PluginManifest, PluginPriority,
    PluginRateLimiter, PluginRegistry, PluginRegistrySnapshot, PluginResourceUsage,
    PluginRunResult, PluginRunStats, PluginRunner, PluginSandboxConfig, PluginTags,
    PluginTestHarness, PluginVersion, PrioritizedPluginRegistry, ProjectPluginPolicy, SemVer,
};

/// A lint plugin contributes a set of lint rules to the engine.
///
/// Implement this trait to create a custom lint plugin.
pub trait LintPlugin: Send + Sync {
    /// The unique name of the plugin.
    fn name(&self) -> &str;
    /// The semantic version of the plugin.
    fn version(&self) -> &str;
    /// Return all lint rules provided by this plugin.
    fn rules(&self) -> Vec<Box<dyn LintRule>>;
}
/// Load a plugin from a manifest.
///
/// Currently returns an `ExampleCustomPlugin` when the manifest name matches,
/// or a `BuiltinPlugin` otherwise.  In a production system this would
/// dynamically load a shared library.
pub fn load_plugin_from_manifest(manifest: &PluginManifest) -> Box<dyn LintPlugin> {
    match manifest.name.as_str() {
        "example_custom" => Box::new(ExampleCustomPlugin::new()),
        _ => Box::new(BuiltinPlugin::new()),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_plugin_registry() {
        let mut registry = PluginRegistry::new();
        assert!(registry.is_empty());
        registry.register(Box::new(BuiltinPlugin::new()));
        assert_eq!(registry.len(), 1);
        assert!(registry.contains("builtin"));
        registry.register(Box::new(ExampleCustomPlugin::new()));
        assert_eq!(registry.len(), 2);
        assert!(registry.contains("example_custom"));
        let removed = registry.unregister("builtin");
        assert!(removed);
        assert_eq!(registry.len(), 1);
        assert!(!registry.contains("builtin"));
        let not_found = registry.unregister("nonexistent");
        assert!(!not_found);
    }
    #[test]
    fn test_builtin_plugin() {
        let plugin = BuiltinPlugin::new();
        assert_eq!(plugin.name(), "builtin");
        assert_eq!(plugin.version(), "1.0.0");
        let rules = plugin.rules();
        assert!(!rules.is_empty());
        let rule_names: Vec<&str> = rules.iter().map(|r| r.name()).collect();
        assert!(rule_names.contains(&"unused variable"));
        assert!(rule_names.contains(&"unused import"));
    }
    #[test]
    fn test_custom_plugin_rules() {
        let plugin = ExampleCustomPlugin::new();
        assert_eq!(plugin.name(), "example_custom");
        assert_eq!(plugin.version(), "0.1.1");
        let rules = plugin.rules();
        assert_eq!(rules.len(), 2);
        let names: Vec<&str> = rules.iter().map(|r| r.name()).collect();
        assert!(names.contains(&"no sorry"));
        assert!(names.contains(&"prefer omega"));
        let ids: Vec<String> = rules.iter().map(|r| r.id().as_str().to_string()).collect();
        assert!(ids.contains(&"no_sorry".to_string()));
        assert!(ids.contains(&"prefer_omega".to_string()));
    }
    #[test]
    fn test_plugin_manifest() {
        let manifest = PluginManifest::new(
            "example_custom",
            "0.1.1",
            "Example plugin",
            vec!["no_sorry".to_string(), "prefer_omega".to_string()],
        );
        assert_eq!(manifest.name, "example_custom");
        assert_eq!(manifest.version, "0.1.1");
        assert_eq!(manifest.rules.len(), 2);
        let text = manifest.to_text();
        assert!(text.contains("example_custom"));
        assert!(text.contains("0.1.1"));
        assert!(text.contains("no_sorry"));
        let loaded = load_plugin_from_manifest(&manifest);
        assert_eq!(loaded.name(), "example_custom");
        let rules = loaded.rules();
        assert_eq!(rules.len(), 2);
    }
    #[test]
    fn test_semver_parse() {
        let v = SemVer::parse("1.2.3").expect("parse should succeed");
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 2);
        assert_eq!(v.patch, 3);
        assert_eq!(v.to_string(), "1.2.3");
        assert!(SemVer::parse("bad").is_none());
        assert!(SemVer::parse("1.2").is_none());
        assert!(SemVer::parse("1.2.3.4").is_none());
    }
    #[test]
    fn test_semver_compatibility() {
        let v100 = SemVer::new(1, 0, 0);
        let v110 = SemVer::new(1, 1, 0);
        let v200 = SemVer::new(2, 0, 0);
        assert!(v110.is_compatible_with(&v100));
        assert!(!v100.is_compatible_with(&v110));
        assert!(!v200.is_compatible_with(&v100));
    }
    #[test]
    fn test_semver_bump() {
        let v = SemVer::new(1, 2, 3);
        assert_eq!(v.bump_patch(), SemVer::new(1, 2, 4));
        assert_eq!(v.bump_minor(), SemVer::new(1, 3, 0));
        assert_eq!(v.bump_major(), SemVer::new(2, 0, 0));
    }
    #[test]
    fn test_semver_pre_release() {
        assert!(SemVer::new(0, 5, 0).is_pre_release());
        assert!(!SemVer::new(1, 0, 0).is_pre_release());
    }
    #[test]
    fn test_compatibility_checker() {
        let checker =
            CompatibilityChecker::new(SemVer::new(1, 0, 0)).with_max(SemVer::new(1, 9, 9));
        assert!(checker.is_engine_compatible(&SemVer::new(1, 5, 0)));
        assert!(!checker.is_engine_compatible(&SemVer::new(2, 0, 0)));
        assert!(!checker.is_engine_compatible(&SemVer::new(0, 9, 0)));
        let desc = checker.requirement_description();
        assert!(desc.contains("1.0.0"));
    }
    #[test]
    fn test_dependency_resolver() {
        let deps = vec![
            PluginDependency::required("plugin_a", SemVer::new(1, 0, 0)),
            PluginDependency::optional("plugin_b", SemVer::new(0, 1, 0)),
        ];
        let mut available = HashMap::new();
        available.insert("plugin_a".to_string(), SemVer::new(1, 2, 0));
        let results = DependencyResolver::resolve(&deps, &available);
        assert!(DependencyResolver::is_ok(&results));
        let results_empty = DependencyResolver::resolve(&deps, &HashMap::new());
        assert!(!DependencyResolver::is_ok(&results_empty));
    }
    #[test]
    fn test_plugin_config() {
        let mut cfg = PluginConfig::new();
        cfg.set("max_errors", "50");
        cfg.set("strict", "true");
        cfg.set("timeout", "5000");
        assert_eq!(cfg.get("max_errors"), Some("50"));
        assert_eq!(cfg.get_bool("strict"), Some(true));
        assert_eq!(cfg.get_usize("timeout"), Some(5000));
        assert_eq!(cfg.get("missing"), None);
        assert_eq!(cfg.len(), 3);
        cfg.remove("strict");
        assert_eq!(cfg.len(), 2);
    }
    #[test]
    fn test_sandbox_config() {
        let perm = PluginSandboxConfig::permissive();
        assert!(perm.allow_fs_read);
        let strict = PluginSandboxConfig::strict();
        assert!(!strict.allow_fs_read);
        assert!(strict.exceeds_diag_limit(600));
        assert!(!strict.exceeds_diag_limit(100));
    }
    #[test]
    fn test_resource_usage() {
        let mut usage = PluginResourceUsage::new();
        usage.cpu_ms = 100;
        usage.diagnostics_emitted = 10;
        let sandbox = PluginSandboxConfig::strict();
        assert!(!usage.exceeds_limits(&sandbox));
        usage.cpu_ms = 100_000;
        assert!(usage.exceeds_limits(&sandbox));
    }
    #[test]
    fn test_project_plugin_policy() {
        let mut policy = ProjectPluginPolicy::all_enabled();
        assert!(policy.is_allowed("any_plugin"));
        policy.enable("plugin_a");
        assert!(policy.is_allowed("plugin_a"));
        assert!(!policy.is_allowed("plugin_b"));
        policy.disable("plugin_a");
        assert!(!policy.is_allowed("plugin_a"));
    }
    #[test]
    fn test_hot_reload_state() {
        let mut state = HotReloadState::new(true);
        state.record("my_plugin", 100);
        assert!(!state.has_changed("my_plugin", 100));
        assert!(state.has_changed("my_plugin", 200));
        assert!(state.has_changed("unknown_plugin", 0));
    }
    #[test]
    fn test_hot_reload_manager() {
        let mut mgr = HotReloadManager::new(true);
        let mut stamps = HashMap::new();
        stamps.insert("plugin_a".to_string(), 1u64);
        stamps.insert("plugin_b".to_string(), 1u64);
        let stale = mgr.check_and_reload(&stamps);
        assert_eq!(stale.len(), 2);
        assert_eq!(mgr.reload_count("plugin_a"), 1);
        let stale2 = mgr.check_and_reload(&stamps);
        assert_eq!(stale2.len(), 0);
        stamps.insert("plugin_a".to_string(), 2u64);
        let stale3 = mgr.check_and_reload(&stamps);
        assert_eq!(stale3.len(), 1);
        assert_eq!(mgr.reload_count("plugin_a"), 2);
    }
    #[test]
    fn test_plugin_test_harness() {
        let mut harness = PluginTestHarness::new();
        let plugin = ExampleCustomPlugin::new();
        harness.assert_name_non_empty(&plugin);
        harness.assert_version_parseable(&plugin);
        harness.assert_has_rules(&plugin);
        harness.assert_has_rule(&plugin, "no_sorry");
        harness.assert_has_rule(&plugin, "prefer_omega");
        harness.assert_has_rule(&plugin, "nonexistent");
        assert_eq!(harness.failure_count(), 1);
        assert!(!harness.all_passed());
        let summary = harness.summary();
        assert!(summary.contains("FAIL"));
    }
    #[test]
    fn test_plugin_registry_snapshot() {
        let mut registry = PluginRegistry::new();
        registry.register(Box::new(BuiltinPlugin::new()));
        let snap1 = PluginRegistrySnapshot::take(&registry, 1);
        registry.register(Box::new(ExampleCustomPlugin::new()));
        let snap2 = PluginRegistrySnapshot::take(&registry, 2);
        assert!(snap1.differs_from(&snap2));
        assert_eq!(snap1.plugin_names.len(), 1);
        assert_eq!(snap2.plugin_names.len(), 2);
    }
    #[test]
    fn test_plugin_catalog() {
        let mut catalog = PluginCatalog::new();
        let m = PluginManifest::new(
            "my_plugin",
            "1.0.0",
            "A great plugin",
            vec!["rule_a".to_string()],
        );
        catalog.add(m);
        assert_eq!(catalog.len(), 1);
        let found = catalog.get("my_plugin").expect("key should exist");
        assert_eq!(found.name, "my_plugin");
        let results = catalog.search("great");
        assert_eq!(results.len(), 1);
        catalog.remove("my_plugin");
        assert!(catalog.is_empty());
    }
    #[test]
    fn test_plugin_event_log() {
        let mut log = PluginEventLog::new();
        log.push(PluginEvent::Registered("plugin_a".to_string()));
        log.push(PluginEvent::Disabled("plugin_a".to_string()));
        log.push(PluginEvent::Registered("plugin_b".to_string()));
        let registered = log.count_where(|e| matches!(e, PluginEvent::Registered(_)));
        assert_eq!(registered, 2);
        let disabled = log.filter(|e| matches!(e, PluginEvent::Disabled(_)));
        assert_eq!(disabled.len(), 1);
        log.clear();
        assert_eq!(log.events().len(), 0);
    }
    #[test]
    fn test_prioritized_registry() {
        let mut reg = PrioritizedPluginRegistry::new();
        reg.add(PluginPriority::Low, Box::new(ExampleCustomPlugin::new()));
        reg.add(PluginPriority::High, Box::new(BuiltinPlugin::new()));
        reg.add(PluginPriority::Normal, Box::new(ExampleCustomPlugin::new()));
        let names = reg.names_ordered();
        assert_eq!(names.len(), 3);
        assert_eq!(names[0], "builtin");
    }
    #[test]
    fn test_plugin_tags() {
        let mut tags = PluginTags::new();
        tags.add("security");
        tags.add("correctness");
        assert!(tags.has("security"));
        assert!(!tags.has("style"));
        let mut other = PluginTags::from_iter(["style", "security"]);
        assert!(tags.overlaps(&other));
        other.remove("security");
        other.remove("style");
        assert!(!tags.overlaps(&other));
        assert_eq!(tags.len(), 2);
    }
    #[test]
    fn test_plugin_runner() {
        let policy = ProjectPluginPolicy::all_enabled();
        let runner = PluginRunner::new(policy);
        let plugin = BuiltinPlugin::new();
        let result = runner.run_plugin(&plugin);
        assert!(!result.aborted);
        assert_eq!(result.plugin_name, "builtin");
    }
    #[test]
    fn test_plugin_runner_disabled() {
        let mut policy = ProjectPluginPolicy::all_enabled();
        policy.disable("builtin");
        let runner = PluginRunner::new(policy);
        let plugin = BuiltinPlugin::new();
        let result = runner.run_plugin(&plugin);
        assert!(result.aborted);
        assert!(result.status.contains("disabled"));
    }
    #[test]
    fn test_discovery_config() {
        let cfg = PluginDiscoveryConfig::new()
            .add_dir("/tmp/plugins")
            .with_recursive(true);
        assert!(cfg.recursive);
        assert_eq!(cfg.search_dirs.len(), 2);
    }
    #[test]
    fn test_plugin_discovery() {
        let cfg = PluginDiscoveryConfig::new();
        let discovery = PluginDiscovery::new(cfg);
        let found = discovery.discover();
        assert!(!found.is_empty());
        let toml_paths = PluginDiscovery::filter_by_extension(&found, "toml");
        assert!(!toml_paths.is_empty());
    }
    #[test]
    fn test_plugin_run_result() {
        let ok = PluginRunResult::success("my_plugin", 5);
        assert!(!ok.aborted);
        assert_eq!(ok.diagnostic_count, 5);
        let aborted = PluginRunResult::aborted("my_plugin", "timeout");
        assert!(aborted.aborted);
        assert!(aborted.status.contains("timeout"));
    }
}
#[cfg(test)]
mod plugin_extension_tests {
    use super::*;
    #[test]
    fn plugin_rate_limiter_allows_within_limit() {
        let mut limiter = PluginRateLimiter::new(3);
        assert!(limiter.allow(0));
        assert!(limiter.allow(0));
        assert!(limiter.allow(0));
        assert!(!limiter.allow(0));
    }
    #[test]
    fn plugin_rate_limiter_resets_on_new_tick() {
        let mut limiter = PluginRateLimiter::new(2);
        limiter.allow(0);
        limiter.allow(0);
        assert!(limiter.allow(1));
    }
    #[test]
    fn plugin_dependency_graph_no_cycle() {
        let mut graph = PluginDependencyGraph::new();
        graph.add_dependency("A", "B");
        graph.add_dependency("B", "C");
        assert!(!graph.has_cycle("A"));
    }
    #[test]
    fn plugin_dependency_graph_cycle_detected() {
        let mut graph = PluginDependencyGraph::new();
        graph.add_dependency("A", "B");
        graph.add_dependency("B", "A");
        assert!(graph.has_cycle("A"));
    }
    #[test]
    fn plugin_dependency_graph_dependencies_of() {
        let mut graph = PluginDependencyGraph::new();
        graph.add_dependency("A", "B");
        graph.add_dependency("A", "C");
        let deps = graph.dependencies_of("A");
        assert!(deps.contains(&"B"));
        assert!(deps.contains(&"C"));
    }
    #[test]
    fn plugin_health_monitor_records_failures() {
        let mut monitor = PluginHealthMonitor::new(3);
        assert!(monitor.is_healthy("plugin_a"));
        monitor.record_failure("plugin_a");
        monitor.record_failure("plugin_a");
        assert!(monitor.is_healthy("plugin_a"));
        let should_disable = monitor.record_failure("plugin_a");
        assert!(should_disable);
        assert!(!monitor.is_healthy("plugin_a"));
    }
    #[test]
    fn plugin_health_monitor_reset() {
        let mut monitor = PluginHealthMonitor::new(2);
        monitor.record_failure("p");
        monitor.record_failure("p");
        monitor.reset("p");
        assert!(monitor.is_healthy("p"));
    }
    #[test]
    fn plugin_audit_log_basic() {
        let mut log = PluginAuditLog::new();
        log.log("plugin_a", "loaded", true);
        log.log("plugin_b", "run_failed", false);
        assert_eq!(log.total(), 2);
        let failures = log.failures();
        assert_eq!(failures.len(), 1);
        assert_eq!(failures[0].plugin_name, "plugin_b");
    }
}
/// Negotiate the minimum required API version between a host and a plugin.
///
/// Returns `Ok(negotiated)` where `negotiated` is the higher of the two
/// versions when they share the same major version, or `Err` on mismatch.
#[allow(dead_code)]
pub fn negotiate_api_version(
    host: PluginVersion,
    plugin: PluginVersion,
) -> Result<PluginVersion, String> {
    if host.major != plugin.major {
        return Err(format!(
            "major version mismatch: host={} plugin={}",
            host.major, plugin.major
        ));
    }
    Ok(host.max(plugin))
}
/// Optional lifecycle hooks a plugin may implement beyond the core trait.
///
/// The host engine calls these at defined points in the lint run.
#[allow(dead_code)]
pub trait PluginLifecycleHooks {
    /// Called once before the lint run begins.
    fn on_run_start(&mut self) {}
    /// Called once after the lint run completes.
    fn on_run_end(&mut self) {}
    /// Called when a new source file is about to be analysed.
    fn on_file_start(&mut self, _path: &Path) {}
    /// Called when analysis of a source file has finished.
    fn on_file_end(&mut self, _path: &Path, _diagnostic_count: usize) {}
    /// Called when the plugin is about to be unloaded.
    fn on_shutdown(&mut self) {}
    /// Called when configuration is reloaded at runtime.
    fn on_config_reload(&mut self, _new_config: &HashMap<String, String>) {}
}
#[cfg(test)]
mod capability_tests {
    use super::*;
    #[test]
    fn capability_set_add_and_has() {
        let mut caps = PluginCapabilitySet::new();
        caps.add("incremental").add("cross_module");
        assert!(caps.has("incremental"));
        assert!(caps.has("cross_module"));
        assert!(!caps.has("gpu"));
        assert_eq!(caps.len(), 2);
    }
    #[test]
    fn capability_set_remove() {
        let mut caps = PluginCapabilitySet::new();
        caps.add("a");
        assert!(caps.remove("a"));
        assert!(!caps.has("a"));
        assert!(caps.is_empty());
    }
    #[test]
    fn capability_set_merge() {
        let mut a = PluginCapabilitySet::new();
        a.add("x");
        let mut b = PluginCapabilitySet::new();
        b.add("y");
        a.merge(&b);
        assert!(a.has("x"));
        assert!(a.has("y"));
    }
    #[test]
    fn capability_set_intersection() {
        let mut a = PluginCapabilitySet::new();
        a.add("x").add("z");
        let mut b = PluginCapabilitySet::new();
        b.add("x").add("y");
        let inter = a.intersection(&b);
        assert!(inter.has("x"));
        assert!(!inter.has("y"));
        assert!(!inter.has("z"));
    }
    #[test]
    fn compatibility_matrix_incompatible() {
        let mut matrix = PluginCompatibilityMatrix::new();
        matrix.declare_incompatible("alpha", "beta");
        assert!(matrix.are_incompatible("alpha", "beta"));
        assert!(matrix.are_incompatible("beta", "alpha"));
        assert!(!matrix.are_incompatible("alpha", "gamma"));
    }
    #[test]
    fn compatibility_matrix_validate() {
        let mut matrix = PluginCompatibilityMatrix::new();
        matrix.declare_incompatible("a", "b");
        let conflicts = matrix.validate(&["a", "b", "c"]);
        assert_eq!(conflicts.len(), 1);
        let no_conflicts = matrix.validate(&["a", "c"]);
        assert!(no_conflicts.is_empty());
    }
    #[test]
    fn plugin_version_parse_and_compare() {
        let v1 = PluginVersion::parse("1.2.3").expect("parse should succeed");
        let v2 = PluginVersion::parse("1.3.0").expect("parse should succeed");
        assert!(v2 > v1);
        assert_eq!(v1.to_string_repr(), "1.2.3");
    }
    #[test]
    fn plugin_version_compatibility() {
        let host = PluginVersion::new(2, 1, 0);
        let plugin = PluginVersion::new(2, 0, 5);
        assert!(host.is_compatible_with(&plugin));
        let incompat = PluginVersion::new(3, 0, 0);
        assert!(!host.is_compatible_with(&incompat));
    }
    #[test]
    fn negotiate_api_version_ok() {
        let host = PluginVersion::new(1, 5, 0);
        let plugin = PluginVersion::new(1, 3, 2);
        let neg = negotiate_api_version(host, plugin).expect("version negotiation should succeed");
        assert_eq!(neg, host);
    }
    #[test]
    fn negotiate_api_version_mismatch() {
        let host = PluginVersion::new(1, 0, 0);
        let plugin = PluginVersion::new(2, 0, 0);
        assert!(negotiate_api_version(host, plugin).is_err());
    }
    #[test]
    fn plugin_run_stats_merge() {
        let mut s1 = PluginRunStats::new();
        s1.diagnostics_emitted = 3;
        s1.files_visited = 2;
        let mut s2 = PluginRunStats::new();
        s2.diagnostics_emitted = 7;
        s2.files_visited = 5;
        s1.merge(&s2);
        assert_eq!(s1.diagnostics_emitted, 10);
        assert_eq!(s1.files_visited, 7);
    }
    #[test]
    fn plugin_run_stats_avg() {
        let mut s = PluginRunStats::new();
        s.diagnostics_emitted = 10;
        s.files_visited = 4;
        assert!((s.avg_diagnostics_per_file() - 2.5).abs() < 1e-9);
    }
    #[test]
    fn plugin_run_stats_zero_files() {
        let s = PluginRunStats::new();
        assert_eq!(s.avg_diagnostics_per_file(), 0.0);
    }
}
