//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashMap;
use std::path::PathBuf;

use super::types::{
    ArtifactKind, BuildArtifact, BuildCache, BuildCompiler, BuildConfig, BuildEnvironment,
    BuildEventLog, BuildFeatureFlags, BuildGraph, BuildGraphNode, BuildLogEntry, BuildLogLevel,
    BuildMetrics, BuildNotification, BuildOutputFilter, BuildPathResolver, BuildPhase, BuildPlan,
    BuildProfileKind, BuildSession, BuildStats, BuildSummary, BuildSystemCapabilities,
    BuildSystemError, BuildTarget, CompilationUnit, DependencyGraph, NoopPlugin, OxileanVersion,
    PackageId, PhaseTimings, PluginRegistry, TargetKind, WorkspaceInfo,
};

/// Returns a best-effort CPU count.
pub(super) fn num_cpus() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_build_config_default() {
        let cfg = BuildConfig::default();
        assert_eq!(cfg.profile, BuildProfileKind::Debug);
        assert!(cfg.jobs >= 1);
        assert!(!cfg.verbose);
    }
    #[test]
    fn test_build_config_release() {
        let cfg = BuildConfig::release();
        assert_eq!(cfg.profile, BuildProfileKind::Release);
    }
    #[test]
    fn test_build_config_with_jobs() {
        let cfg = BuildConfig::default().with_jobs(8);
        assert_eq!(cfg.jobs, 8);
    }
    #[test]
    fn test_build_config_with_jobs_min_one() {
        let cfg = BuildConfig::default().with_jobs(0);
        assert_eq!(cfg.jobs, 1);
    }
    #[test]
    fn test_build_profile_display() {
        assert_eq!(format!("{}", BuildProfileKind::Debug), "debug");
        assert_eq!(format!("{}", BuildProfileKind::Release), "release");
        assert_eq!(format!("{}", BuildProfileKind::Test), "test");
    }
    #[test]
    fn test_target_kind_display() {
        assert_eq!(format!("{}", TargetKind::Lib), "lib");
        assert_eq!(format!("{}", TargetKind::Bin), "bin");
    }
    #[test]
    fn test_build_target_lib() {
        let t = BuildTarget::lib("mylib", "src/lib.ox");
        assert_eq!(t.name, "mylib");
        assert_eq!(t.kind, TargetKind::Lib);
        assert!(t.enabled);
    }
    #[test]
    fn test_build_target_bin() {
        let t = BuildTarget::bin("mybinary", "src/main.ox");
        assert_eq!(t.kind, TargetKind::Bin);
    }
    #[test]
    fn test_build_target_test() {
        let t = BuildTarget::test("mytest", "tests/test.ox");
        assert_eq!(t.kind, TargetKind::Test);
    }
    #[test]
    fn test_build_target_disabled() {
        let t = BuildTarget::lib("lib", "src/lib.ox").disabled();
        assert!(!t.enabled);
    }
    #[test]
    fn test_build_target_depends_on() {
        let t = BuildTarget::bin("app", "src/main.ox")
            .depends_on("core")
            .depends_on("util");
        assert_eq!(t.deps.len(), 2);
        assert!(t.deps.contains(&"core".to_string()));
    }
    #[test]
    fn test_build_plan_add_target() {
        let mut plan = BuildPlan::new(BuildConfig::default());
        plan.add_target(BuildTarget::lib("lib", "src/lib.ox"));
        assert_eq!(plan.target_count(), 1);
    }
    #[test]
    fn test_build_plan_find_target() {
        let mut plan = BuildPlan::new(BuildConfig::default());
        plan.add_target(BuildTarget::lib("mylib", "src/lib.ox"));
        assert!(plan.find_target("mylib").is_some());
        assert!(plan.find_target("missing").is_none());
    }
    #[test]
    fn test_build_plan_enabled_targets() {
        let mut plan = BuildPlan::new(BuildConfig::default());
        plan.add_target(BuildTarget::lib("lib1", "src/lib1.ox"));
        plan.add_target(BuildTarget::lib("lib2", "src/lib2.ox").disabled());
        let enabled = plan.enabled_targets();
        assert_eq!(enabled.len(), 1);
    }
    #[test]
    fn test_compilation_unit_new() {
        let unit = CompilationUnit::new("src/foo.ox", "build/foo.o", "Foo");
        assert_eq!(unit.module_name, "Foo");
        assert!(!unit.is_cached);
    }
    #[test]
    fn test_compilation_unit_mark_cached() {
        let unit = CompilationUnit::new("src/foo.ox", "build/foo.o", "Foo").mark_cached();
        assert!(unit.is_cached);
    }
    #[test]
    fn test_build_summary_default() {
        let s = BuildSummary::new();
        assert!(s.is_success());
        assert_eq!(s.total(), 0);
    }
    #[test]
    fn test_build_summary_add_error() {
        let mut s = BuildSummary::new();
        s.add_error("compilation failed");
        assert!(!s.is_success());
        assert_eq!(s.failed, 1);
    }
    #[test]
    fn test_build_summary_add_warning() {
        let mut s = BuildSummary::new();
        s.add_warning("unused variable");
        assert!(s.is_success());
        assert_eq!(s.warnings.len(), 1);
    }
    #[test]
    fn test_build_summary_display() {
        let s = BuildSummary {
            compiled: 5,
            cached: 3,
            failed: 0,
            elapsed_ms: 1200,
            ..BuildSummary::default()
        };
        let text = format!("{}", s);
        assert!(text.contains("compiled: 5"));
        assert!(text.contains("cached: 3"));
        assert!(text.contains("1200ms"));
    }
    #[test]
    fn test_build_summary_total() {
        let s = BuildSummary {
            compiled: 4,
            cached: 2,
            failed: 1,
            ..BuildSummary::default()
        };
        assert_eq!(s.total(), 7);
    }
    #[test]
    fn test_build_config_verbose() {
        let cfg = BuildConfig::default().verbose();
        assert!(cfg.verbose);
    }
}
#[cfg(test)]
mod extra_tests {
    use super::*;
    #[test]
    fn test_build_cache_insert_and_get() {
        let mut c = BuildCache::new();
        c.insert("Foo", std::path::PathBuf::from("build/Foo.o"));
        assert!(c.contains("Foo"));
        assert_eq!(
            c.get("Foo").expect("key should exist"),
            &std::path::PathBuf::from("build/Foo.o")
        );
    }
    #[test]
    fn test_build_cache_invalidate() {
        let mut c = BuildCache::new();
        c.insert("Bar", std::path::PathBuf::from("build/Bar.o"));
        c.invalidate("Bar");
        assert!(!c.contains("Bar"));
    }
    #[test]
    fn test_build_cache_clear() {
        let mut c = BuildCache::new();
        c.insert("X", std::path::PathBuf::from("build/X.o"));
        c.clear();
        assert!(c.is_empty());
    }
    #[test]
    fn test_build_cache_display() {
        let c = BuildCache::new();
        let s = format!("{}", c);
        assert!(s.contains("BuildCache"));
    }
    #[test]
    fn test_build_log_level_display() {
        assert_eq!(format!("{}", BuildLogLevel::Info), "INFO");
        assert_eq!(format!("{}", BuildLogLevel::Error), "ERROR");
    }
    #[test]
    fn test_build_log_entry_info() {
        let e = BuildLogEntry::info("compiled foo");
        assert_eq!(e.level, BuildLogLevel::Info);
        assert!(e.target.is_none());
    }
    #[test]
    fn test_build_log_entry_for_target() {
        let e = BuildLogEntry::info("compiled").for_target("mylib");
        assert_eq!(e.target.as_deref(), Some("mylib"));
    }
    #[test]
    fn test_build_log_entry_display_with_target() {
        let e = BuildLogEntry::error("fail").for_target("core");
        let s = format!("{}", e);
        assert!(s.contains("core"));
        assert!(s.contains("fail"));
    }
    #[test]
    fn test_build_log_entry_display_no_target() {
        let e = BuildLogEntry::info("starting build");
        let s = format!("{}", e);
        assert!(s.contains("INFO"));
    }
}
#[cfg(test)]
mod dep_tests {
    use super::*;
    fn simple_dag() -> std::collections::HashMap<String, Vec<String>> {
        let mut m = std::collections::HashMap::new();
        m.insert("a".to_string(), vec!["b".to_string()]);
        m.insert("b".to_string(), vec!["c".to_string()]);
        m.insert("c".to_string(), vec![]);
        m
    }
    #[test]
    fn test_topo_sort_acyclic() {
        let dag = simple_dag();
        let order = DependencyGraph::topo_sort(&dag);
        assert!(order.is_some());
        let o = order.expect("test operation should succeed");
        let ai = o
            .iter()
            .position(|s| s == "a")
            .expect("test operation should succeed");
        let bi = o
            .iter()
            .position(|s| s == "b")
            .expect("test operation should succeed");
        let ci = o
            .iter()
            .position(|s| s == "c")
            .expect("test operation should succeed");
        assert!(
            ai < bi && bi < ci,
            "expected topological ordering a < b < c, got {:?}",
            o
        );
    }
    #[test]
    fn test_has_cycle_no_cycle() {
        let dag = simple_dag();
        assert!(!DependencyGraph::has_cycle(&dag));
    }
    #[test]
    fn test_has_cycle_with_cycle() {
        let mut dag = std::collections::HashMap::new();
        dag.insert("a".to_string(), vec!["b".to_string()]);
        dag.insert("b".to_string(), vec!["a".to_string()]);
        assert!(DependencyGraph::has_cycle(&dag));
    }
    #[test]
    fn test_build_environment_set_get() {
        let mut env = BuildEnvironment::new();
        env.set("OXILEAN_ROOT", "/opt/oxilean");
        assert_eq!(env.get("OXILEAN_ROOT"), Some("/opt/oxilean"));
    }
    #[test]
    fn test_build_environment_missing_key() {
        let env = BuildEnvironment::new();
        assert_eq!(env.get("MISSING"), None);
    }
    #[test]
    fn test_build_environment_len() {
        let mut env = BuildEnvironment::new();
        env.set("A", "1");
        env.set("B", "2");
        assert_eq!(env.len(), 2);
    }
    #[test]
    fn test_build_environment_display() {
        let env = BuildEnvironment::new();
        let s = format!("{}", env);
        assert!(s.contains("BuildEnvironment"));
    }
    #[test]
    fn test_topo_sort_empty() {
        let dag = std::collections::HashMap::new();
        let order = DependencyGraph::topo_sort(&dag);
        assert_eq!(order, Some(vec![]));
    }
}
#[cfg(test)]
mod artifact_tests {
    use super::*;
    #[test]
    fn test_artifact_new() {
        let a = BuildArtifact::new("mylib.a", "build/mylib.a", ArtifactKind::StaticLib);
        assert_eq!(a.name, "mylib.a");
        assert_eq!(a.kind, ArtifactKind::StaticLib);
        assert!(a.size_bytes.is_none());
    }
    #[test]
    fn test_artifact_with_size_bytes() {
        let a = BuildArtifact::new("foo", "build/foo", ArtifactKind::Executable).with_size(512);
        assert_eq!(a.size_display(), "512 B");
    }
    #[test]
    fn test_artifact_with_size_kb() {
        let a = BuildArtifact::new("foo", "f", ArtifactKind::Object).with_size(2048);
        assert!(a.size_display().contains("KB"));
    }
    #[test]
    fn test_artifact_with_size_mb() {
        let a = BuildArtifact::new("foo", "f", ArtifactKind::DynLib).with_size(2_097_152);
        assert!(a.size_display().contains("MB"));
    }
    #[test]
    fn test_artifact_display() {
        let a = BuildArtifact::new("lib", "build/lib.a", ArtifactKind::StaticLib);
        let s = format!("{}", a);
        assert!(s.contains("lib"));
    }
    #[test]
    fn test_artifact_kind_display() {
        assert_eq!(format!("{}", ArtifactKind::Object), "object");
        assert_eq!(format!("{}", ArtifactKind::Export), "export");
    }
    #[test]
    fn test_build_phase_ordering() {
        assert!(BuildPhase::Parse < BuildPhase::TypeCheck);
        assert!(BuildPhase::TypeCheck < BuildPhase::Codegen);
        assert!(BuildPhase::Codegen < BuildPhase::Link);
    }
    #[test]
    fn test_phase_timings_display() {
        let t = PhaseTimings::new(BuildPhase::Parse, 150);
        let s = format!("{}", t);
        assert!(s.contains("parse"));
        assert!(s.contains("150ms"));
    }
    #[test]
    fn test_artifact_size_unknown() {
        let a = BuildArtifact::new("x", "x", ArtifactKind::Docs);
        assert_eq!(a.size_display(), "unknown");
    }
    #[test]
    fn test_build_phase_display() {
        assert_eq!(format!("{}", BuildPhase::TypeCheck), "type-check");
        assert_eq!(format!("{}", BuildPhase::Package), "package");
    }
}
#[cfg(test)]
mod lib_extra_tests {
    use super::*;
    #[test]
    fn feature_flags_debug_defaults() {
        let flags = BuildFeatureFlags::debug_defaults();
        assert!(flags.debug_assertions);
        assert!(flags.incremental);
        assert!(!flags.lto);
    }
    #[test]
    fn feature_flags_release_defaults() {
        let flags = BuildFeatureFlags::release_defaults();
        assert!(flags.lto);
        assert!(flags.simd);
        assert!(!flags.debug_assertions);
    }
    #[test]
    fn feature_flags_sanitizer() {
        let flags = BuildFeatureFlags::debug_defaults().with_sanitizer("address");
        assert!(flags.has_sanitizers());
        assert_eq!(flags.sanitizers.len(), 1);
    }
    #[test]
    fn build_notification_label() {
        assert_eq!(BuildNotification::Started.label(), "started");
        assert_eq!(BuildNotification::Error("e".to_string()).label(), "error");
        assert!(BuildNotification::Completed(true).is_success());
        assert!(!BuildNotification::Completed(false).is_success());
    }
    #[test]
    fn build_notification_display() {
        let n = BuildNotification::Warning("unused var".to_string());
        assert!(format!("{}", n).contains("warning"));
    }
    #[test]
    fn build_event_log_push_and_count() {
        let mut log = BuildEventLog::new();
        log.push(BuildNotification::Started);
        log.push(BuildNotification::Error("fail".to_string()));
        assert_eq!(log.len(), 2);
        assert!(log.has_errors());
        assert_eq!(log.count_by_label("error"), 1);
    }
    #[test]
    fn build_path_resolver_paths() {
        let res = BuildPathResolver::new("/project", "/project/build");
        let src = res.source_path("src/Main.lean");
        assert!(src
            .to_str()
            .expect("conversion should succeed")
            .contains("src/Main.lean"));
        let obj = res.object_path("Mathlib.Data.Nat");
        assert!(obj
            .to_str()
            .expect("conversion should succeed")
            .contains(".o"));
    }
    #[test]
    fn build_metrics_total_ms() {
        let m = BuildMetrics {
            parse_ms: 100,
            typecheck_ms: 200,
            codegen_ms: 50,
            link_ms: 25,
            ..Default::default()
        };
        assert_eq!(m.total_ms(), 375);
    }
    #[test]
    fn build_metrics_report_nonempty() {
        let m = BuildMetrics::new();
        assert!(!m.report().is_empty());
    }
    #[test]
    fn package_id_slug() {
        let id = PackageId::new("oxilean-core", "0.1.1");
        assert_eq!(id.to_slug(), "oxilean-core@0.1.1");
        assert_eq!(format!("{}", id), "oxilean-core@0.1.1");
    }
    #[test]
    fn build_graph_node_add_dep() {
        let mut node = BuildGraphNode::new("ModA", "src/A.lean", "build/A.o");
        node.add_dep("ModB");
        assert_eq!(node.deps.len(), 1);
    }
    #[test]
    fn build_graph_node_invalidate() {
        let mut node = BuildGraphNode::new("X", "X.lean", "X.o");
        assert!(!node.invalidated);
        node.invalidate();
        assert!(node.invalidated);
    }
    #[test]
    fn build_graph_topo_order() {
        let mut graph = BuildGraph::new();
        let mut a = BuildGraphNode::new("A", "A.lean", "A.o");
        a.add_dep("B");
        graph.add_node(a);
        graph.add_node(BuildGraphNode::new("B", "B.lean", "B.o"));
        let order = graph.topo_order();
        assert_eq!(order.len(), 2);
    }
    #[test]
    fn build_graph_invalidated_nodes() {
        let mut graph = BuildGraph::new();
        let mut node = BuildGraphNode::new("X", "X.lean", "X.o");
        node.invalidate();
        graph.add_node(node);
        graph.add_node(BuildGraphNode::new("Y", "Y.lean", "Y.o"));
        assert_eq!(graph.invalidated_nodes().len(), 1);
    }
    #[test]
    fn build_stats_success_rate() {
        let s = BuildStats {
            targets_built: 8,
            targets_skipped: 1,
            targets_failed: 1,
            ..Default::default()
        };
        assert!((s.success_rate() - 0.8).abs() < 1e-9);
    }
    #[test]
    fn build_stats_is_clean() {
        let mut s = BuildStats::new();
        assert!(s.is_clean());
        s.targets_failed = 1;
        assert!(!s.is_clean());
    }
    #[test]
    fn build_stats_summary_nonempty() {
        let s = BuildStats::new();
        assert!(!s.summary().is_empty());
    }
}
#[cfg(test)]
mod session_tests {
    use super::*;
    #[test]
    fn workspace_info_members() {
        let mut ws = WorkspaceInfo::new("oxilean", "/opt/oxilean");
        ws.add_member("oxilean-core");
        ws.add_member("oxilean-meta");
        assert_eq!(ws.member_count(), 2);
        assert!(ws.is_member("oxilean-core"));
        assert!(!ws.is_member("missing"));
    }
    #[test]
    fn workspace_info_display() {
        let ws = WorkspaceInfo::new("my-workspace", "/ws");
        let s = format!("{}", ws);
        assert!(s.contains("Workspace(my-workspace"));
    }
    #[test]
    fn build_session_record_built() {
        let cfg = BuildConfig::default();
        let mut session = BuildSession::start(cfg);
        session.record_built("mylib");
        assert_eq!(session.stats.targets_built, 1);
        assert!(session.events.count_by_label("target-finished") >= 1);
    }
    #[test]
    fn build_session_record_failed() {
        let cfg = BuildConfig::default();
        let mut session = BuildSession::start(cfg);
        session.record_failed("bad-lib", "type error");
        assert_eq!(session.stats.targets_failed, 1);
        assert!(session.events.has_errors());
        assert!(!session.succeeded());
    }
    #[test]
    fn build_session_advance_phase() {
        let cfg = BuildConfig::default();
        let mut session = BuildSession::start(cfg);
        assert_eq!(session.phase, BuildPhase::Parse);
        session.advance_phase();
        assert_eq!(session.phase, BuildPhase::TypeCheck);
        session.advance_phase();
        assert_eq!(session.phase, BuildPhase::Codegen);
    }
    #[test]
    fn build_session_finish_success() {
        let cfg = BuildConfig::default();
        let mut session = BuildSession::start(cfg);
        session.record_built("lib");
        session.finish(1500);
        assert_eq!(session.stats.wall_time_ms, 1500);
        assert!(session.succeeded());
    }
    #[test]
    fn build_session_finish_failure() {
        let cfg = BuildConfig::default();
        let mut session = BuildSession::start(cfg);
        session.record_failed("lib", "oops");
        session.finish(500);
        assert!(!session.succeeded());
    }
}
/// A hook that can be called at various build lifecycle points.
pub trait BuildPlugin: std::fmt::Debug {
    /// Name of the plugin.
    fn name(&self) -> &str;
    /// Called before any targets are built.
    fn on_build_start(&self, _config: &BuildConfig) {}
    /// Called after all targets are built.
    fn on_build_finish(&self, _summary: &BuildSummary) {}
    /// Called when a target starts building.
    fn on_target_start(&self, _target: &BuildTarget) {}
    /// Called when a target finishes building.
    fn on_target_finish(&self, _target: &BuildTarget, _success: bool) {}
}
/// Convenience type alias for build system results.
pub type BuildResult<T> = Result<T, BuildSystemError>;
#[cfg(test)]
mod plugin_tests {
    use super::*;
    #[test]
    fn noop_plugin_name() {
        let p = NoopPlugin::new("formatter");
        assert_eq!(p.name(), "formatter");
    }
    #[test]
    fn plugin_registry_register_and_fire() {
        let mut reg = PluginRegistry::new();
        reg.register(Box::new(NoopPlugin::new("p1")));
        reg.register(Box::new(NoopPlugin::new("p2")));
        assert_eq!(reg.len(), 2);
        let cfg = BuildConfig::default();
        reg.fire_build_start(&cfg);
        let summary = BuildSummary::new();
        reg.fire_build_finish(&summary);
    }
    #[test]
    fn oxilean_version_display() {
        let v = OxileanVersion::new(0, 1, 1);
        assert_eq!(format!("{}", v), "0.1.1");
    }
    #[test]
    fn oxilean_version_pre_release() {
        let v = OxileanVersion::pre(1, 0, 0, "alpha.1");
        assert!(v.is_pre_release());
        assert_eq!(format!("{}", v), "1.0.0-alpha.1");
    }
    #[test]
    fn oxilean_version_is_at_least() {
        let v = OxileanVersion::new(0, 2, 0);
        assert!(v.is_at_least(0, 1, 0));
        assert!(v.is_at_least(0, 2, 0));
        assert!(!v.is_at_least(0, 3, 0));
    }
    #[test]
    fn build_system_error_display() {
        let e = BuildSystemError::InvalidConfig("missing field".to_string());
        assert!(format!("{}", e).contains("InvalidConfig"));
        let e2 = BuildSystemError::DependencyCycle(vec!["A".to_string(), "B".to_string()]);
        assert!(format!("{}", e2).contains("A -> B"));
    }
    #[test]
    fn build_result_ok_err() {
        let ok: BuildResult<u32> = Ok(42);
        assert!(ok.is_ok());
        let err: BuildResult<u32> = Err(BuildSystemError::Io("disk full".to_string()));
        assert!(err.is_err());
    }
}
/// Returns the current build system API version.
pub fn build_system_api_version() -> OxileanVersion {
    OxileanVersion::new(0, 1, 1)
}
#[cfg(test)]
mod api_version_test {
    use super::*;
    #[test]
    fn api_version_nonempty() {
        let v = build_system_api_version();
        assert!(format!("{}", v).len() > 0);
    }
}
#[cfg(test)]
mod filter_compiler_tests {
    use super::*;
    #[test]
    fn output_filter_should_show() {
        let filter = BuildOutputFilter::new()
            .suppress("note:")
            .min_level(BuildLogLevel::Warn);
        assert!(!filter.should_show(BuildLogLevel::Info, "just info"));
        assert!(filter.should_show(BuildLogLevel::Warn, "this is a warning"));
        assert!(!filter.should_show(BuildLogLevel::Info, "note: unused import"));
    }
    #[test]
    fn output_filter_highlight() {
        let filter = BuildOutputFilter::new().highlight("error[");
        assert!(filter.should_highlight("error[E0001]: something"));
        assert!(!filter.should_highlight("just a warning"));
    }
    #[test]
    fn build_compiler_command_line() {
        let mut compiler = BuildCompiler::new("/usr/bin/oxileanc");
        compiler.add_flag("--opt");
        let cmd = compiler.command_line("src/Main.lean", "build/Main.o");
        assert!(cmd.contains(&"--opt".to_string()));
        assert!(cmd.contains(&"-o".to_string()));
        assert!(cmd.contains(&"src/Main.lean".to_string()));
    }
    #[test]
    fn build_compiler_env() {
        let mut compiler = BuildCompiler::new("/usr/bin/oxileanc");
        compiler.set_env("LEAN_PATH", "/lean/lib");
        assert_eq!(
            compiler.env.get("LEAN_PATH").map(|s| s.as_str()),
            Some("/lean/lib")
        );
    }
}
/// Utility: compute a simple build fingerprint from a list of file paths.
pub fn fingerprint_file_list(files: &[&str]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for &file in files {
        for b in file.bytes() {
            h ^= b as u64;
            h = h.wrapping_mul(0x100000001b3);
        }
        h ^= b'/' as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}
#[cfg(test)]
mod fingerprint_test {
    use super::*;
    #[test]
    fn fingerprint_file_list_deterministic() {
        let h1 = fingerprint_file_list(&["a.lean", "b.lean"]);
        let h2 = fingerprint_file_list(&["a.lean", "b.lean"]);
        assert_eq!(h1, h2);
    }
    #[test]
    fn fingerprint_file_list_different_inputs() {
        let h1 = fingerprint_file_list(&["a.lean"]);
        let h2 = fingerprint_file_list(&["b.lean"]);
        assert_ne!(h1, h2);
    }
}
#[cfg(test)]
mod capabilities_tests {
    use super::*;
    #[test]
    fn full_capabilities() {
        let caps = BuildSystemCapabilities::full();
        assert!(caps.incremental && caps.distributed && caps.parallel);
        assert_eq!(caps.max_jobs, 256);
    }
    #[test]
    fn minimal_capabilities() {
        let caps = BuildSystemCapabilities::minimal();
        assert!(!caps.incremental);
        assert_eq!(caps.max_jobs, 1);
    }
    #[test]
    fn capabilities_display() {
        let caps = BuildSystemCapabilities::full();
        let s = format!("{}", caps);
        assert!(s.contains("Capabilities["));
    }
}
