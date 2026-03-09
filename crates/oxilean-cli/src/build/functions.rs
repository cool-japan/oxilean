//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet, VecDeque};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use super::types::{
    ArtifactKind, ArtifactRegistry, ArtifactStore, BuildArtifact, BuildCache, BuildCacheEntry,
    BuildConfig, BuildConfigValidator, BuildDiagnostic, BuildEnvironment, BuildEvent,
    BuildEventRecorder, BuildFilter, BuildGraph, BuildHistory, BuildHistoryEntry, BuildLockfile,
    BuildLog, BuildMetrics, BuildNode, BuildPipeline, BuildProfile, BuildProgress, BuildReport,
    BuildStatus, BuildStepResult, BuildSummaryReport, BuildTarget, BuildTargetConfig,
    BuildTargetKind, OptLevel, ParallelBuildPlan,
};

/// Compute a simple content hash for change detection.
pub fn content_hash(data: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for &byte in data {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x0100_0000_01b3);
    }
    hash
}
/// Build a dependency graph from a set of source files.
///
/// Each file's `import` lines are scanned to determine dependencies.
pub fn build_dependency_graph(sources: &[(String, PathBuf)], output_dir: &Path) -> BuildGraph {
    let mut graph = BuildGraph::new();
    for (module, source_path) in sources {
        let output_path = output_dir.join(format!("{}.olean", module));
        let mut node = BuildNode::new(module.clone(), source_path.clone(), output_path);
        let path_bytes = source_path.to_string_lossy().as_bytes().to_vec();
        node.hash = content_hash(&path_bytes);
        if let Ok(contents) = std::fs::read_to_string(source_path) {
            for line in contents.lines() {
                let trimmed = line.trim();
                if let Some(rest) = trimmed.strip_prefix("import ") {
                    let dep = rest.trim().to_string();
                    node.add_dependency(dep);
                }
            }
            node.hash = content_hash(contents.as_bytes());
        }
        graph.add_node(node);
    }
    graph
}
/// Pretty-print a build report.
pub fn format_build_report(report: &BuildReport) -> String {
    let mut lines = Vec::new();
    lines.push(String::new());
    lines.push("=== Build Report ===".to_string());
    lines.push(format!("Total modules : {}", report.total_modules));
    lines.push(format!("Succeeded     : {}", report.succeeded));
    lines.push(format!("Cached        : {}", report.cached));
    lines.push(format!("Failed        : {}", report.failed));
    lines.push(format!(
        "Elapsed       : {}",
        format_duration(report.elapsed)
    ));
    if !report.steps.is_empty() {
        lines.push(String::new());
        lines.push("--- Steps ---".to_string());
        for step in &report.steps {
            lines.push(format_build_step(step));
        }
    }
    if report.failed > 0 {
        lines.push(String::new());
        lines.push("--- Failures ---".to_string());
        for step in &report.steps {
            if step.status == BuildStatus::Failure {
                lines.push(format!("  {}", step.module));
                for diag in &step.diagnostics {
                    lines.push(format!("    {}", diag));
                }
            }
        }
    }
    lines.push(String::new());
    if report.is_success() {
        lines.push("Build succeeded.".to_string());
    } else {
        lines.push(format!("Build failed ({} error(s)).", report.failed));
    }
    lines.join("\n")
}
/// Format a single build step result.
pub fn format_build_step(step: &BuildStepResult) -> String {
    format!(
        "  [{:>6}] {} ({})",
        step.status,
        step.module,
        format_duration(step.duration),
    )
}
/// Simple text-based progress bar.
pub fn progress_bar(current: usize, total: usize, width: usize) -> String {
    if total == 0 {
        return format!("[{}] 0/0", " ".repeat(width));
    }
    let filled = (current * width) / total;
    let empty = width.saturating_sub(filled);
    format!(
        "[{}{}] {}/{}",
        "#".repeat(filled),
        " ".repeat(empty),
        current,
        total,
    )
}
/// Format a duration in human-readable form.
pub fn format_duration(d: Duration) -> String {
    let total_secs = d.as_secs();
    if total_secs >= 60 {
        let mins = total_secs / 60;
        let secs = total_secs % 60;
        format!("{}m {:02}s", mins, secs)
    } else if total_secs > 0 {
        format!("{}.{:03}s", total_secs, d.subsec_millis())
    } else {
        let ms = d.as_millis();
        if ms > 0 {
            format!("{}ms", ms)
        } else {
            format!("{}us", d.as_micros())
        }
    }
}
/// Format a byte count in human-readable form.
pub fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = 1024 * KB;
    const GB: u64 = 1024 * MB;
    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_build_config_default() {
        let cfg = BuildConfig::default();
        assert_eq!(cfg.target, BuildTarget::Check);
        assert_eq!(cfg.opt_level, OptLevel::Debug);
        assert_eq!(cfg.parallelism, 4);
        assert!(!cfg.verbose);
        assert!(!cfg.force_rebuild);
    }
    #[test]
    fn test_build_config_for_project() {
        let cfg = BuildConfig::for_project("/tmp/my_project");
        assert_eq!(cfg.project_root, PathBuf::from("/tmp/my_project"));
        assert_eq!(cfg.output_dir, PathBuf::from("/tmp/my_project/build"));
        assert_eq!(cfg.cache_dir, PathBuf::from("/tmp/my_project/build/cache"));
    }
    #[test]
    fn test_build_config_effective_opt_level() {
        let mut cfg = BuildConfig::default();
        assert_eq!(cfg.effective_opt_level(), OptLevel::Debug);
        cfg.target = BuildTarget::Release;
        assert_eq!(cfg.effective_opt_level(), OptLevel::Release);
        cfg.target = BuildTarget::Bench;
        assert_eq!(cfg.effective_opt_level(), OptLevel::Release);
    }
    #[test]
    fn test_build_config_validate() {
        let cfg = BuildConfig::default();
        assert!(cfg.validate().is_ok());
        let bad = BuildConfig {
            parallelism: 0,
            ..Default::default()
        };
        assert!(bad.validate().is_err());
    }
    #[test]
    fn test_build_config_is_parallel() {
        let mut cfg = BuildConfig::default();
        assert!(cfg.is_parallel());
        cfg.parallelism = 1;
        assert!(!cfg.is_parallel());
    }
    #[test]
    fn test_build_target_display() {
        assert_eq!(BuildTarget::Check.to_string(), "check");
        assert_eq!(BuildTarget::Release.to_string(), "release");
        assert_eq!(BuildTarget::Doc.to_string(), "doc");
    }
    #[test]
    fn test_opt_level_display() {
        assert_eq!(OptLevel::Debug.to_string(), "debug");
        assert_eq!(OptLevel::Release.to_string(), "release");
        assert_eq!(OptLevel::Size.to_string(), "size");
    }
    #[test]
    fn test_build_node_new() {
        let node = BuildNode::new(
            "Foo.Bar".to_string(),
            PathBuf::from("src/Foo/Bar.lean"),
            PathBuf::from("build/Foo.Bar.olean"),
        );
        assert_eq!(node.module_name, "Foo.Bar");
        assert!(node.dependencies.is_empty());
        assert!(node.is_stale);
    }
    #[test]
    fn test_build_node_add_dependency() {
        let mut node = BuildNode::new(
            "A".to_string(),
            PathBuf::from("A.lean"),
            PathBuf::from("A.olean"),
        );
        node.add_dependency("B".to_string());
        node.add_dependency("C".to_string());
        node.add_dependency("B".to_string());
        assert_eq!(node.dependency_count(), 2);
    }
    #[test]
    fn test_build_graph_empty() {
        let g = BuildGraph::new();
        assert_eq!(g.node_count(), 0);
        assert!(!g.contains("foo"));
    }
    #[test]
    fn test_build_graph_add_and_get() {
        let mut g = BuildGraph::new();
        let n = BuildNode::new("A".into(), "A.lean".into(), "A.olean".into());
        g.add_node(n);
        assert_eq!(g.node_count(), 1);
        assert!(g.contains("A"));
        assert!(g.get_node("A").is_some());
        assert!(g.get_node("B").is_none());
    }
    #[test]
    fn test_topological_order_no_deps() {
        let mut g = BuildGraph::new();
        g.add_node(BuildNode::new(
            "A".into(),
            "A.lean".into(),
            "A.olean".into(),
        ));
        g.add_node(BuildNode::new(
            "B".into(),
            "B.lean".into(),
            "B.olean".into(),
        ));
        let order = g
            .topological_order()
            .expect("test operation should succeed");
        assert_eq!(order.len(), 2);
    }
    #[test]
    fn test_topological_order_with_deps() {
        let mut g = BuildGraph::new();
        let mut b = BuildNode::new("B".into(), "B.lean".into(), "B.olean".into());
        b.add_dependency("A".to_string());
        g.add_node(BuildNode::new(
            "A".into(),
            "A.lean".into(),
            "A.olean".into(),
        ));
        g.add_node(b);
        let order = g
            .topological_order()
            .expect("test operation should succeed");
        let pos_a = order
            .iter()
            .position(|n| n == "A")
            .expect("test operation should succeed");
        let pos_b = order
            .iter()
            .position(|n| n == "B")
            .expect("test operation should succeed");
        assert!(pos_a < pos_b);
    }
    #[test]
    fn test_topological_order_cycle() {
        let mut g = BuildGraph::new();
        let mut a = BuildNode::new("A".into(), "A.lean".into(), "A.olean".into());
        a.add_dependency("B".to_string());
        let mut b = BuildNode::new("B".into(), "B.lean".into(), "B.olean".into());
        b.add_dependency("A".to_string());
        g.add_node(a);
        g.add_node(b);
        assert!(g.topological_order().is_err());
    }
    #[test]
    fn test_affected_nodes() {
        let mut g = BuildGraph::new();
        let mut b = BuildNode::new("B".into(), "B.lean".into(), "B.olean".into());
        b.add_dependency("A".to_string());
        let mut c = BuildNode::new("C".into(), "C.lean".into(), "C.olean".into());
        c.add_dependency("B".to_string());
        g.add_node(BuildNode::new(
            "A".into(),
            "A.lean".into(),
            "A.olean".into(),
        ));
        g.add_node(b);
        g.add_node(c);
        let affected = g.affected_nodes("A");
        assert!(affected.contains("A"));
        assert!(affected.contains("B"));
        assert!(affected.contains("C"));
    }
    #[test]
    fn test_compute_staleness_fresh() {
        let mut g = BuildGraph::new();
        let mut n = BuildNode::new("A".into(), "A.lean".into(), "A.olean".into());
        n.hash = 42;
        g.add_node(n);
        let mut cache = HashMap::new();
        cache.insert("A".to_string(), 42u64);
        g.compute_staleness(&cache);
        assert!(
            !g.get_node("A")
                .expect("test operation should succeed")
                .is_stale
        );
    }
    #[test]
    fn test_compute_staleness_stale() {
        let mut g = BuildGraph::new();
        let mut n = BuildNode::new("A".into(), "A.lean".into(), "A.olean".into());
        n.hash = 99;
        g.add_node(n);
        let mut cache = HashMap::new();
        cache.insert("A".to_string(), 42u64);
        g.compute_staleness(&cache);
        assert!(
            g.get_node("A")
                .expect("test operation should succeed")
                .is_stale
        );
    }
    #[test]
    fn test_content_hash_deterministic() {
        let data = b"hello world";
        let h1 = content_hash(data);
        let h2 = content_hash(data);
        assert_eq!(h1, h2);
    }
    #[test]
    fn test_content_hash_different_data() {
        let h1 = content_hash(b"hello");
        let h2 = content_hash(b"world");
        assert_ne!(h1, h2);
    }
    #[test]
    fn test_build_step_result_is_ok() {
        let ok = BuildStepResult {
            module: "A".into(),
            status: BuildStatus::Success,
            duration: Duration::ZERO,
            diagnostics: vec![],
        };
        assert!(ok.is_ok());
        let fail = BuildStepResult {
            module: "B".into(),
            status: BuildStatus::Failure,
            duration: Duration::ZERO,
            diagnostics: vec!["error".into()],
        };
        assert!(!fail.is_ok());
    }
    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(Duration::from_secs(90)), "1m 30s");
        assert_eq!(format_duration(Duration::from_millis(1500)), "1.500s");
        assert_eq!(format_duration(Duration::from_millis(42)), "42ms");
        assert_eq!(format_duration(Duration::from_micros(100)), "100us");
    }
    #[test]
    fn test_format_size() {
        assert_eq!(format_size(512), "512 B");
        assert_eq!(format_size(2048), "2.00 KB");
        assert_eq!(format_size(1_048_576), "1.00 MB");
        assert_eq!(format_size(2_147_483_648), "2.00 GB");
    }
    #[test]
    fn test_progress_bar() {
        let bar = progress_bar(5, 10, 20);
        assert!(bar.contains("##########"));
        assert!(bar.contains("5/10"));
        let empty_bar = progress_bar(0, 0, 10);
        assert!(empty_bar.contains("0/0"));
    }
    #[test]
    fn test_build_report_is_success() {
        let report = BuildReport {
            total_modules: 2,
            succeeded: 2,
            failed: 0,
            cached: 0,
            elapsed: Duration::from_millis(100),
            steps: vec![],
        };
        assert!(report.is_success());
    }
    #[test]
    fn test_build_report_failed_modules() {
        let report = BuildReport {
            total_modules: 2,
            succeeded: 1,
            failed: 1,
            cached: 0,
            elapsed: Duration::from_millis(100),
            steps: vec![
                BuildStepResult {
                    module: "A".into(),
                    status: BuildStatus::Success,
                    duration: Duration::ZERO,
                    diagnostics: vec![],
                },
                BuildStepResult {
                    module: "B".into(),
                    status: BuildStatus::Failure,
                    duration: Duration::ZERO,
                    diagnostics: vec!["boom".into()],
                },
            ],
        };
        assert!(!report.is_success());
        assert_eq!(report.failed_modules(), vec!["B"]);
    }
    #[test]
    fn test_format_build_report_content() {
        let report = BuildReport {
            total_modules: 1,
            succeeded: 1,
            failed: 0,
            cached: 0,
            elapsed: Duration::from_millis(50),
            steps: vec![BuildStepResult {
                module: "Main".into(),
                status: BuildStatus::Success,
                duration: Duration::from_millis(50),
                diagnostics: vec![],
            }],
        };
        let text = format_build_report(&report);
        assert!(text.contains("Build succeeded"));
        assert!(text.contains("Main"));
    }
    #[test]
    fn test_artifact_path() {
        let store = ArtifactStore::new("/tmp/oxilean_test_artifacts");
        let p = store.artifact_path("Foo.Bar.Baz");
        assert_eq!(
            p,
            PathBuf::from("/tmp/oxilean_test_artifacts/Foo/Bar/Baz.olean")
        );
    }
    #[test]
    fn test_artifact_store_roundtrip() {
        let dir = std::env::temp_dir().join("oxilean_build_test_artifacts");
        let _ = std::fs::remove_dir_all(&dir);
        let store = ArtifactStore::new(&dir);
        store
            .store_artifact("Test.Module", b"data123")
            .expect("test operation should succeed");
        let data = store
            .load_artifact("Test.Module")
            .expect("test operation should succeed");
        assert_eq!(data, b"data123");
        store
            .invalidate_artifact("Test.Module")
            .expect("test operation should succeed");
        assert!(store.load_artifact("Test.Module").is_err());
        let _ = std::fs::remove_dir_all(&dir);
    }
    #[test]
    fn test_build_status_display() {
        assert_eq!(BuildStatus::Success.to_string(), "OK");
        assert_eq!(BuildStatus::Failure.to_string(), "FAIL");
        assert_eq!(BuildStatus::Cached.to_string(), "CACHED");
        assert_eq!(BuildStatus::Skipped.to_string(), "SKIP");
    }
    #[test]
    fn test_build_graph_module_names() {
        let mut g = BuildGraph::new();
        g.add_node(BuildNode::new(
            "X".into(),
            "X.lean".into(),
            "X.olean".into(),
        ));
        g.add_node(BuildNode::new(
            "Y".into(),
            "Y.lean".into(),
            "Y.olean".into(),
        ));
        let names = g.module_names();
        assert_eq!(names.len(), 2);
        assert!(names.contains(&"X".to_string()));
        assert!(names.contains(&"Y".to_string()));
    }
}
#[allow(dead_code)]
pub fn build_fingerprint(content: &str) -> u64 {
    let mut h: u64 = 14695981039346656037;
    for b in content.as_bytes() {
        h = h.wrapping_mul(1099511628211);
        h ^= *b as u64;
    }
    h
}
#[cfg(test)]
mod build_extended_tests {
    use super::*;
    #[test]
    fn test_build_metrics() {
        let mut m = BuildMetrics::default();
        m.record_compiled(100);
        m.record_compiled(200);
        m.record_failed();
        assert_eq!(m.compiled, 2);
        assert_eq!(m.failed, 1);
        assert!((m.success_rate() - 2.0 / 3.0).abs() < 0.01);
        assert!((m.avg_duration_ms() - 150.0).abs() < 0.1);
    }
    #[test]
    fn test_build_profile_dominant() {
        let p = BuildProfile {
            lex_ms: 10.0,
            parse_ms: 5.0,
            elab_ms: 100.0,
            typecheck_ms: 30.0,
            codegen_ms: 20.0,
        };
        assert_eq!(p.dominant_stage(), "elab");
    }
    #[test]
    fn test_build_profile_add() {
        let mut p1 = BuildProfile {
            lex_ms: 10.0,
            parse_ms: 5.0,
            elab_ms: 0.0,
            typecheck_ms: 0.0,
            codegen_ms: 0.0,
        };
        let p2 = BuildProfile {
            lex_ms: 5.0,
            parse_ms: 10.0,
            elab_ms: 0.0,
            typecheck_ms: 0.0,
            codegen_ms: 0.0,
        };
        p1.add(&p2);
        assert!((p1.lex_ms - 15.0).abs() < 0.1);
        assert!((p1.parse_ms - 15.0).abs() < 0.1);
    }
    #[test]
    fn test_build_log() {
        let mut log = BuildLog::new(100);
        log.info("build started");
        log.warn("old API");
        log.error("compile failed");
        assert_eq!(log.errors().len(), 1);
        assert_eq!(log.warnings().len(), 1);
    }
    #[test]
    fn test_build_filter_include_exclude() {
        let f = BuildFilter::new().include("src").exclude("test");
        assert!(f.matches("src/main.lean"));
        assert!(!f.matches("test/spec.lean"));
        assert!(!f.matches("docs/guide.lean"));
    }
    #[test]
    fn test_artifact_registry() {
        let mut reg = ArtifactRegistry::new();
        reg.add(BuildArtifact {
            source: "main.lean".to_string(),
            output: std::path::PathBuf::from("main.o"),
            kind: ArtifactKind::ObjectFile,
            size_bytes: 1024,
        });
        assert_eq!(reg.count(), 1);
        assert_eq!(reg.total_size(), 1024);
        assert!(reg.find_by_source("main.lean").is_some());
    }
    #[test]
    fn test_parallel_build_plan_waves() {
        let mut deps = std::collections::HashMap::new();
        deps.insert("a".to_string(), vec![]);
        deps.insert("b".to_string(), vec!["a".to_string()]);
        deps.insert("c".to_string(), vec!["a".to_string()]);
        let plan = ParallelBuildPlan::from_deps(&deps);
        assert!(plan.wave_count() >= 1);
    }
    #[test]
    fn test_build_cache_is_stale() {
        let mut cache = BuildCache::new();
        cache.insert(
            "a.lean",
            BuildCacheEntry {
                source_hash: 42,
                output_path: std::path::PathBuf::new(),
                built_at: std::time::SystemTime::UNIX_EPOCH,
            },
        );
        assert!(!cache.is_stale("a.lean", 42));
        assert!(cache.is_stale("a.lean", 99));
        assert!(cache.is_stale("missing.lean", 0));
    }
    #[test]
    fn test_build_fingerprint_deterministic() {
        let h1 = build_fingerprint("theorem foo : P := sorry");
        let h2 = build_fingerprint("theorem foo : P := sorry");
        assert_eq!(h1, h2);
    }
    #[test]
    fn test_build_fingerprint_different() {
        let h1 = build_fingerprint("foo");
        let h2 = build_fingerprint("bar");
        assert_ne!(h1, h2);
    }
    #[test]
    fn test_build_progress() {
        let mut p = BuildProgress::new(10);
        p.complete_one();
        p.complete_one();
        p.fail_one();
        assert!((p.pct() - 30.0).abs() < 0.1);
        assert_eq!(p.done, 2);
        assert_eq!(p.failed, 1);
    }
    #[test]
    fn test_build_lockfile() {
        let mut lock = BuildLockfile::new();
        lock.add_dep("std", "0.1.1", "abc123");
        assert!(lock.find("std").is_some());
        let toml = lock.to_toml();
        assert!(toml.contains("std"));
    }
    #[test]
    fn test_build_event_recorder() {
        let mut rec = BuildEventRecorder::new(100);
        rec.record(BuildEvent::Started {
            file: "a.lean".to_string(),
        });
        rec.record(BuildEvent::Succeeded {
            file: "a.lean".to_string(),
            duration_ms: 50,
        });
        rec.record(BuildEvent::Failed {
            file: "b.lean".to_string(),
            error: "err".to_string(),
        });
        assert_eq!(rec.successes().len(), 1);
        assert_eq!(rec.failures().len(), 1);
        assert_eq!(rec.event_count(), 3);
    }
    #[test]
    fn test_build_summary_report() {
        let mut r = BuildSummaryReport::new();
        r.files_compiled = 10;
        r.files_failed = 0;
        assert!(r.is_success());
        r.files_failed = 1;
        assert!(!r.is_success());
    }
}
/// Estimate the number of CPUs available.
#[allow(dead_code)]
pub fn num_cpus_estimate() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
}
/// Return the build module version.
#[allow(dead_code)]
pub fn build_module_version() -> &'static str {
    "0.1.1"
}
#[cfg(test)]
mod build_extra_tests {
    use super::*;
    #[test]
    fn test_build_pipeline() {
        let pipeline = BuildPipeline::default_oxilean();
        let names = pipeline.stage_names();
        assert!(names.contains(&"parse"));
        assert!(names.contains(&"typecheck"));
        let required = pipeline.required_stages();
        assert!(required.iter().all(|s| s.required));
    }
    #[test]
    fn test_build_target_library() {
        let target = BuildTargetConfig::library(
            "oxilean_std",
            vec!["src/lib.lean".to_string()],
            "target/libstd.olean",
        );
        assert_eq!(target.build_type, BuildTargetKind::Library);
        assert!(target.dependencies.is_empty());
    }
    #[test]
    fn test_build_target_with_dependency() {
        let target =
            BuildTargetConfig::binary("main", vec![], "target/main").with_dependency("oxilean_std");
        assert_eq!(target.dependencies.len(), 1);
    }
    #[test]
    fn test_build_environment_development() {
        let env = BuildEnvironment::development();
        assert_eq!(env.optimization_level, 0);
        assert!(env.debug_info);
        assert!(!env.warnings_as_errors);
    }
    #[test]
    fn test_build_environment_release() {
        let env = BuildEnvironment::release();
        assert_eq!(env.optimization_level, 3);
        assert!(!env.debug_info);
        assert!(env.warnings_as_errors);
    }
    #[test]
    fn test_build_diagnostic_error() {
        let diag = BuildDiagnostic::error("typecheck", "type mismatch").at_location(
            "src/main.lean",
            10,
            5,
        );
        let formatted = diag.format();
        assert!(formatted.contains("[error]"));
        assert!(formatted.contains("type mismatch"));
        assert!(formatted.contains("src/main.lean"));
    }
    #[test]
    fn test_build_diagnostic_warning() {
        let diag = BuildDiagnostic::warning("parse", "ambiguous syntax");
        let formatted = diag.format();
        assert!(formatted.contains("[warning]"));
    }
    #[test]
    fn test_build_module_version() {
        assert!(!build_module_version().is_empty());
    }
    #[test]
    fn test_num_cpus_estimate() {
        let cpus = num_cpus_estimate();
        assert!(cpus >= 1);
    }
}
/// Return the feature set for the build module.
#[allow(dead_code)]
pub fn build_features() -> Vec<&'static str> {
    vec![
        "dependency-graph",
        "topological-sort",
        "artifact-registry",
        "cache",
        "parallel-plan",
        "progress",
        "lockfile",
        "events",
        "metrics",
        "pipeline",
        "targets",
        "environment",
        "diagnostics",
        "validator",
    ]
}
#[cfg(test)]
mod build_validator_tests {
    use super::*;
    #[test]
    fn test_build_validator_valid() {
        let env = BuildEnvironment::development();
        let errors = BuildConfigValidator::validate_env(&env);
        assert!(errors.is_empty());
    }
    #[test]
    fn test_build_validator_invalid_jobs() {
        let mut env = BuildEnvironment::development();
        env.jobs = 0;
        let errors = BuildConfigValidator::validate_env(&env);
        assert!(errors.iter().any(|e| e.field == "jobs"));
    }
    #[test]
    fn test_build_features() {
        let features = build_features();
        assert!(features.contains(&"dependency-graph"));
        assert!(features.contains(&"cache"));
        assert!(features.len() >= 10);
    }
}
/// No-op placeholder.
#[allow(dead_code)]
pub fn build_noop() {}
#[cfg(test)]
mod build_history_tests {
    use super::*;
    #[test]
    fn test_build_history() {
        let mut history = BuildHistory::new(10);
        history.record(BuildHistoryEntry {
            timestamp_secs: 1000,
            success: true,
            duration_ms: 200,
            stage_reached: "link".to_string(),
            error_count: 0,
            warning_count: 2,
        });
        history.record(BuildHistoryEntry {
            timestamp_secs: 2000,
            success: false,
            duration_ms: 100,
            stage_reached: "typecheck".to_string(),
            error_count: 3,
            warning_count: 0,
        });
        assert_eq!(history.entries.len(), 2);
        assert!((history.success_rate() - 50.0).abs() < 0.01);
        let last = history.last_n(1);
        assert_eq!(last.len(), 1);
        assert!(!last[0].success);
    }
}
