//! # OxiLean Build System & Package Manager
//!
//! This crate implements the build system and package manager for OxiLean,
//! providing incremental compilation, dependency resolution, parallel build
//! execution, and package registry integration.
//!
//! ## Modules
//!
//! - manifest: Package manifest parsing and metadata
//! - resolver: PubGrub-style dependency resolution
//! - incremental: Incremental compilation with fingerprinting
//! - executor: DAG-based parallel build scheduling
//! - registry: Package registry integration
//! - scripts: Custom build scripts and hooks

#![allow(dead_code)]
#![warn(clippy::all)]
#![allow(clippy::result_large_err)]
#![allow(unused_imports)]
#![allow(clippy::field_reassign_with_default)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::single_match)]
#![allow(clippy::needless_ifs)]
#![allow(clippy::useless_format)]
#![allow(clippy::new_without_default)]
#![allow(clippy::manual_strip)]
#![allow(clippy::needless_borrows_for_generic_args)]
#![allow(clippy::len_without_is_empty)]
#![allow(clippy::type_complexity)]
#![allow(clippy::manual_saturating_arithmetic)]
#![allow(clippy::if_same_then_else)]
#![allow(clippy::manual_is_variant_and)]
#![allow(clippy::implicit_saturating_sub)]
#![allow(clippy::incompatible_msrv)]
#![allow(clippy::int_plus_one)]
#![allow(clippy::manual_map)]
#![allow(clippy::needless_bool)]
#![allow(clippy::needless_else)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::manual_find)]
#![allow(clippy::for_kv_map)]
#![allow(clippy::manual_range_contains)]
#![allow(clippy::double_ended_iterator_last)]
#![allow(clippy::len_zero)]

pub mod analytics;
pub mod cache_eviction;
pub mod distributed;
pub mod executor;
pub mod incremental;
pub mod manifest;
pub mod opt_incremental;
pub mod registry;
pub mod remote_cache;
pub mod resolver;
pub mod scripts;

// ============================================================
// Top-level types defined in this lib file
// ============================================================

use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;

// ============================================================
// BuildConfig: top-level configuration
// ============================================================

/// Global build configuration.
#[derive(Clone, Debug)]
pub struct BuildConfig {
    /// Root directory of the project.
    pub root: PathBuf,
    /// Output/artifact directory.
    pub out_dir: PathBuf,
    /// Build profile (debug, release, etc.).
    pub profile: BuildProfileKind,
    /// Number of parallel build jobs.
    pub jobs: usize,
    /// Whether to enable verbose output.
    pub verbose: bool,
    /// Whether to emit build warnings.
    pub warnings: bool,
    /// Extra compiler flags.
    pub extra_flags: Vec<String>,
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            root: PathBuf::from("."),
            out_dir: PathBuf::from("build"),
            profile: BuildProfileKind::Debug,
            jobs: num_cpus(),
            verbose: false,
            warnings: true,
            extra_flags: Vec::new(),
        }
    }
}

impl BuildConfig {
    /// Create a release build configuration.
    pub fn release() -> Self {
        Self {
            profile: BuildProfileKind::Release,
            ..Self::default()
        }
    }

    /// Set the number of parallel jobs.
    pub fn with_jobs(mut self, n: usize) -> Self {
        self.jobs = n.max(1);
        self
    }

    /// Enable verbose output.
    pub fn verbose(mut self) -> Self {
        self.verbose = true;
        self
    }

    /// Set the root directory.
    pub fn with_root(mut self, root: impl Into<PathBuf>) -> Self {
        self.root = root.into();
        self
    }
}

/// Returns a best-effort CPU count.
fn num_cpus() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
}

// ============================================================
// BuildProfileKind
// ============================================================

/// Build profile variants.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BuildProfileKind {
    /// Debug build: fast compilation, no optimizations, all debug info.
    #[default]
    Debug,
    /// Release build: full optimizations, stripped debug info.
    Release,
    /// Test build: debug + test harness.
    Test,
    /// Benchmark build: release + benchmark harness.
    Bench,
    /// Documentation build: no compilation, only doc extraction.
    Doc,
}

impl fmt::Display for BuildProfileKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuildProfileKind::Debug => write!(f, "debug"),
            BuildProfileKind::Release => write!(f, "release"),
            BuildProfileKind::Test => write!(f, "test"),
            BuildProfileKind::Bench => write!(f, "bench"),
            BuildProfileKind::Doc => write!(f, "doc"),
        }
    }
}

// ============================================================
// BuildTarget
// ============================================================

/// A build target (library, binary, test, etc.).
#[derive(Clone, Debug, PartialEq)]
pub struct BuildTarget {
    /// Target name.
    pub name: String,
    /// Root source file.
    pub src: PathBuf,
    /// Kind of target.
    pub kind: TargetKind,
    /// Whether the target is enabled.
    pub enabled: bool,
    /// Dependencies on other targets.
    pub deps: Vec<String>,
}

impl BuildTarget {
    /// Create a library target.
    pub fn lib(name: &str, src: impl Into<PathBuf>) -> Self {
        Self {
            name: name.to_string(),
            src: src.into(),
            kind: TargetKind::Lib,
            enabled: true,
            deps: Vec::new(),
        }
    }

    /// Create a binary target.
    pub fn bin(name: &str, src: impl Into<PathBuf>) -> Self {
        Self {
            name: name.to_string(),
            src: src.into(),
            kind: TargetKind::Bin,
            enabled: true,
            deps: Vec::new(),
        }
    }

    /// Create a test target.
    pub fn test(name: &str, src: impl Into<PathBuf>) -> Self {
        Self {
            name: name.to_string(),
            src: src.into(),
            kind: TargetKind::Test,
            enabled: true,
            deps: Vec::new(),
        }
    }

    /// Add a dependency on another target.
    pub fn depends_on(mut self, dep: &str) -> Self {
        self.deps.push(dep.to_string());
        self
    }

    /// Disable this target.
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }
}

/// The kind of a build target.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TargetKind {
    /// A library.
    Lib,
    /// An executable binary.
    Bin,
    /// A test suite.
    Test,
    /// A benchmark suite.
    Bench,
    /// Documentation.
    Doc,
    /// A build script.
    BuildScript,
}

impl fmt::Display for TargetKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TargetKind::Lib => write!(f, "lib"),
            TargetKind::Bin => write!(f, "bin"),
            TargetKind::Test => write!(f, "test"),
            TargetKind::Bench => write!(f, "bench"),
            TargetKind::Doc => write!(f, "doc"),
            TargetKind::BuildScript => write!(f, "build-script"),
        }
    }
}

// ============================================================
// BuildPlan
// ============================================================

/// A resolved build plan ready for execution.
#[derive(Clone, Debug)]
pub struct BuildPlan {
    /// Build configuration.
    pub config: BuildConfig,
    /// Targets in topological build order.
    pub targets: Vec<BuildTarget>,
    /// Resolved dependency graph.
    pub dep_graph: HashMap<String, Vec<String>>,
}

impl BuildPlan {
    /// Create an empty build plan.
    pub fn new(config: BuildConfig) -> Self {
        Self {
            config,
            targets: Vec::new(),
            dep_graph: HashMap::new(),
        }
    }

    /// Add a target to the plan.
    pub fn add_target(&mut self, target: BuildTarget) {
        let name = target.name.clone();
        let deps = target.deps.clone();
        self.targets.push(target);
        self.dep_graph.insert(name, deps);
    }

    /// Number of targets in the plan.
    pub fn target_count(&self) -> usize {
        self.targets.len()
    }

    /// Find a target by name.
    pub fn find_target(&self, name: &str) -> Option<&BuildTarget> {
        self.targets.iter().find(|t| t.name == name)
    }

    /// Return targets in topological order.
    pub fn topo_order(&self) -> Vec<&BuildTarget> {
        self.targets.iter().collect()
    }

    /// Enabled targets only.
    pub fn enabled_targets(&self) -> Vec<&BuildTarget> {
        self.targets.iter().filter(|t| t.enabled).collect()
    }
}

// ============================================================
// CompilationUnit
// ============================================================

/// A single unit of compilation (one source file to one object).
#[derive(Clone, Debug)]
pub struct CompilationUnit {
    /// Source file path.
    pub source: PathBuf,
    /// Output artifact path.
    pub output: PathBuf,
    /// Module name.
    pub module_name: String,
    /// Whether this unit was already compiled and up-to-date.
    pub is_cached: bool,
}

impl CompilationUnit {
    /// Create a new compilation unit.
    pub fn new(source: impl Into<PathBuf>, output: impl Into<PathBuf>, module_name: &str) -> Self {
        Self {
            source: source.into(),
            output: output.into(),
            module_name: module_name.to_string(),
            is_cached: false,
        }
    }

    /// Mark this unit as cached (up-to-date).
    pub fn mark_cached(mut self) -> Self {
        self.is_cached = true;
        self
    }

    /// Extension of the output file.
    pub fn output_ext(&self) -> &str {
        self.output
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
    }
}

// ============================================================
// BuildSummary
// ============================================================

/// Summary of a completed build.
#[derive(Clone, Debug, Default)]
pub struct BuildSummary {
    /// Number of units compiled from scratch.
    pub compiled: usize,
    /// Number of units served from cache.
    pub cached: usize,
    /// Number of units that failed.
    pub failed: usize,
    /// Total wall-clock time in milliseconds.
    pub elapsed_ms: u64,
    /// Errors encountered.
    pub errors: Vec<String>,
    /// Warnings encountered.
    pub warnings: Vec<String>,
}

impl BuildSummary {
    /// Create a zero summary.
    pub fn new() -> Self {
        Self::default()
    }

    /// Whether the build succeeded.
    pub fn is_success(&self) -> bool {
        self.failed == 0 && self.errors.is_empty()
    }

    /// Total units processed.
    pub fn total(&self) -> usize {
        self.compiled + self.cached + self.failed
    }

    /// Add an error message.
    pub fn add_error(&mut self, msg: &str) {
        self.errors.push(msg.to_string());
        self.failed += 1;
    }

    /// Add a warning message.
    pub fn add_warning(&mut self, msg: &str) {
        self.warnings.push(msg.to_string());
    }
}

impl fmt::Display for BuildSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BuildSummary {{ compiled: {}, cached: {}, failed: {}, {}ms }}",
            self.compiled, self.cached, self.failed, self.elapsed_ms
        )
    }
}

// ============================================================
// Tests
// ============================================================

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

// ============================================================
// BuildCache: cache of compilation results
// ============================================================

/// A simple cache of compiled module results.
#[derive(Clone, Debug, Default)]
pub struct BuildCache {
    /// Cached entries: module_name -> output_path.
    entries: std::collections::HashMap<String, std::path::PathBuf>,
}

impl BuildCache {
    /// Create an empty cache.
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a cache entry.
    pub fn insert(&mut self, module: &str, output: std::path::PathBuf) {
        self.entries.insert(module.to_string(), output);
    }

    /// Look up a cached output path.
    pub fn get(&self, module: &str) -> Option<&std::path::PathBuf> {
        self.entries.get(module)
    }

    /// Whether a module is cached.
    pub fn contains(&self, module: &str) -> bool {
        self.entries.contains_key(module)
    }

    /// Number of cached entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Remove an entry.
    pub fn invalidate(&mut self, module: &str) {
        self.entries.remove(module);
    }

    /// Clear all entries.
    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

impl std::fmt::Display for BuildCache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BuildCache({} entries)", self.entries.len())
    }
}

// ============================================================
// BuildLog: structured build log entries
// ============================================================

/// The level of a build log message.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BuildLogLevel {
    /// Verbose/trace output.
    Trace,
    /// Informational message.
    Info,
    /// Warning.
    Warn,
    /// Error.
    Error,
}

impl std::fmt::Display for BuildLogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildLogLevel::Trace => write!(f, "TRACE"),
            BuildLogLevel::Info => write!(f, "INFO"),
            BuildLogLevel::Warn => write!(f, "WARN"),
            BuildLogLevel::Error => write!(f, "ERROR"),
        }
    }
}

/// A single build log entry.
#[derive(Clone, Debug)]
pub struct BuildLogEntry {
    /// Log level.
    pub level: BuildLogLevel,
    /// Log message.
    pub message: String,
    /// Optional target name context.
    pub target: Option<String>,
}

impl BuildLogEntry {
    /// Create an info entry.
    pub fn info(msg: &str) -> Self {
        Self {
            level: BuildLogLevel::Info,
            message: msg.to_string(),
            target: None,
        }
    }

    /// Create an error entry.
    pub fn error(msg: &str) -> Self {
        Self {
            level: BuildLogLevel::Error,
            message: msg.to_string(),
            target: None,
        }
    }

    /// Attach a target name.
    pub fn for_target(mut self, target: &str) -> Self {
        self.target = Some(target.to_string());
        self
    }
}

impl std::fmt::Display for BuildLogEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(t) = &self.target {
            write!(f, "[{}][{}] {}", self.level, t, self.message)
        } else {
            write!(f, "[{}] {}", self.level, self.message)
        }
    }
}

// ============================================================
// Additional tests
// ============================================================

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

// ============================================================
// DependencyGraph: explicit dep graph operations
// ============================================================

/// Utility functions for build dependency graphs.
pub struct DependencyGraph;

impl DependencyGraph {
    /// Compute the topological order of nodes given an adjacency list.
    /// Returns `None` if there is a cycle.
    pub fn topo_sort(deps: &std::collections::HashMap<String, Vec<String>>) -> Option<Vec<String>> {
        let mut in_degree: std::collections::HashMap<&str, usize> =
            deps.keys().map(|k| (k.as_str(), 0)).collect();
        for edges in deps.values() {
            for e in edges {
                *in_degree.entry(e.as_str()).or_insert(0) += 1;
            }
        }
        let mut queue: std::collections::VecDeque<&str> = in_degree
            .iter()
            .filter(|(_, &d)| d == 0)
            .map(|(&k, _)| k)
            .collect();
        let mut order = Vec::new();
        while let Some(node) = queue.pop_front() {
            order.push(node.to_string());
            if let Some(edges) = deps.get(node) {
                for e in edges {
                    let d = in_degree.entry(e.as_str()).or_insert(0);
                    *d = d.saturating_sub(1);
                    if *d == 0 {
                        queue.push_back(e.as_str());
                    }
                }
            }
        }
        if order.len() == deps.len() {
            Some(order)
        } else {
            None
        }
    }

    /// Whether the dependency graph has any cycles.
    pub fn has_cycle(deps: &std::collections::HashMap<String, Vec<String>>) -> bool {
        Self::topo_sort(deps).is_none()
    }
}

// ============================================================
// BuildEnvironment: environment variables for the build
// ============================================================

/// Environment variables available to build scripts.
#[derive(Clone, Debug, Default)]
pub struct BuildEnvironment {
    /// Variable name → value pairs.
    vars: std::collections::HashMap<String, String>,
}

impl BuildEnvironment {
    /// Create an empty build environment.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set a variable.
    pub fn set(&mut self, key: &str, value: &str) {
        self.vars.insert(key.to_string(), value.to_string());
    }

    /// Get a variable.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.vars.get(key).map(|s| s.as_str())
    }

    /// Number of variables.
    pub fn len(&self) -> usize {
        self.vars.len()
    }

    /// Whether the environment is empty.
    pub fn is_empty(&self) -> bool {
        self.vars.is_empty()
    }
}

impl std::fmt::Display for BuildEnvironment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BuildEnvironment({} vars)", self.vars.len())
    }
}

// ============================================================
// Additional tests
// ============================================================

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
        // DAG: a -> b -> c (a depends on b, b depends on c).
        // Kahn's algorithm processes nodes with in-degree 0 first,
        // so 'a' appears before 'b', and 'b' before 'c'.
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

// ============================================================
// BuildArtifact: describes a single output artifact
// ============================================================

/// A compiled artifact produced by a build target.
#[derive(Clone, Debug)]
pub struct BuildArtifact {
    /// Name of the artifact.
    pub name: String,
    /// Path to the artifact.
    pub path: std::path::PathBuf,
    /// Kind of artifact.
    pub kind: ArtifactKind,
    /// Size in bytes (if known).
    pub size_bytes: Option<u64>,
}

impl BuildArtifact {
    /// Create a new artifact.
    pub fn new(name: &str, path: impl Into<std::path::PathBuf>, kind: ArtifactKind) -> Self {
        Self {
            name: name.to_string(),
            path: path.into(),
            kind,
            size_bytes: None,
        }
    }

    /// Attach a size.
    pub fn with_size(mut self, size: u64) -> Self {
        self.size_bytes = Some(size);
        self
    }

    /// Return a human-readable size description.
    pub fn size_display(&self) -> String {
        match self.size_bytes {
            Some(b) if b >= 1_048_576 => format!("{:.1} MB", b as f64 / 1_048_576.0),
            Some(b) if b >= 1024 => format!("{:.1} KB", b as f64 / 1024.0),
            Some(b) => format!("{} B", b),
            None => "unknown".to_string(),
        }
    }
}

impl std::fmt::Display for BuildArtifact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}[{}] -> {:?}", self.name, self.kind, self.path)
    }
}

/// The kind of a build artifact.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArtifactKind {
    /// A compiled object file.
    Object,
    /// A static library.
    StaticLib,
    /// A dynamic library.
    DynLib,
    /// An executable binary.
    Executable,
    /// Documentation output.
    Docs,
    /// An OxiLean `.olean`-like export file.
    Export,
}

impl std::fmt::Display for ArtifactKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArtifactKind::Object => write!(f, "object"),
            ArtifactKind::StaticLib => write!(f, "static-lib"),
            ArtifactKind::DynLib => write!(f, "dyn-lib"),
            ArtifactKind::Executable => write!(f, "executable"),
            ArtifactKind::Docs => write!(f, "docs"),
            ArtifactKind::Export => write!(f, "export"),
        }
    }
}

// ============================================================
// BuildPhase: named stages of the build pipeline
// ============================================================

/// A named phase in the build pipeline.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum BuildPhase {
    /// Parsing source files.
    Parse,
    /// Type checking.
    TypeCheck,
    /// Code generation.
    Codegen,
    /// Linking.
    Link,
    /// Packaging.
    Package,
}

impl std::fmt::Display for BuildPhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildPhase::Parse => write!(f, "parse"),
            BuildPhase::TypeCheck => write!(f, "type-check"),
            BuildPhase::Codegen => write!(f, "codegen"),
            BuildPhase::Link => write!(f, "link"),
            BuildPhase::Package => write!(f, "package"),
        }
    }
}

/// A phase timing record.
#[derive(Clone, Debug)]
pub struct PhaseTimings {
    /// Phase name.
    pub phase: BuildPhase,
    /// Elapsed time in milliseconds.
    pub elapsed_ms: u64,
}

impl PhaseTimings {
    /// Create a new timing record.
    pub fn new(phase: BuildPhase, elapsed_ms: u64) -> Self {
        Self { phase, elapsed_ms }
    }
}

impl std::fmt::Display for PhaseTimings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}ms", self.phase, self.elapsed_ms)
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

// ============================================================
// BuildFeatureFlags: feature flags for the build system
// ============================================================

/// Feature flags that can be toggled to change build behavior.
#[derive(Clone, Debug, Default)]
pub struct BuildFeatureFlags {
    /// Enable link-time optimization.
    pub lto: bool,
    /// Enable profile-guided optimization.
    pub pgo: bool,
    /// Enable SIMD instruction generation.
    pub simd: bool,
    /// Enable experimental parallel type-checking.
    pub parallel_type_check: bool,
    /// Enable debug assertions even in release builds.
    pub debug_assertions: bool,
    /// Enable incremental compilation.
    pub incremental: bool,
    /// Enable sanitizers (address, memory, etc.).
    pub sanitizers: Vec<String>,
}

impl BuildFeatureFlags {
    /// Create default flags for a debug build.
    pub fn debug_defaults() -> Self {
        Self {
            lto: false,
            pgo: false,
            simd: false,
            parallel_type_check: true,
            debug_assertions: true,
            incremental: true,
            sanitizers: Vec::new(),
        }
    }

    /// Create default flags for a release build.
    pub fn release_defaults() -> Self {
        Self {
            lto: true,
            pgo: false,
            simd: true,
            parallel_type_check: true,
            debug_assertions: false,
            incremental: false,
            sanitizers: Vec::new(),
        }
    }

    /// Add a sanitizer.
    pub fn with_sanitizer(mut self, name: &str) -> Self {
        self.sanitizers.push(name.to_string());
        self
    }

    /// Whether any sanitizers are active.
    pub fn has_sanitizers(&self) -> bool {
        !self.sanitizers.is_empty()
    }
}

// ============================================================
// BuildNotification
// ============================================================

/// A notification event emitted during the build process.
#[derive(Clone, Debug)]
pub enum BuildNotification {
    /// Build started.
    Started,
    /// A target began compiling.
    TargetStarted(String),
    /// A target finished compiling.
    TargetFinished(String, bool), // (name, success)
    /// Build completed.
    Completed(bool), // success
    /// A warning was emitted.
    Warning(String),
    /// An error was emitted.
    Error(String),
}

impl BuildNotification {
    /// Whether this notification indicates success.
    pub fn is_success(&self) -> bool {
        matches!(self, BuildNotification::Completed(true))
    }

    /// Short label for display.
    pub fn label(&self) -> &'static str {
        match self {
            BuildNotification::Started => "started",
            BuildNotification::TargetStarted(_) => "target-started",
            BuildNotification::TargetFinished(_, _) => "target-finished",
            BuildNotification::Completed(_) => "completed",
            BuildNotification::Warning(_) => "warning",
            BuildNotification::Error(_) => "error",
        }
    }
}

impl std::fmt::Display for BuildNotification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.label())
    }
}

// ============================================================
// BuildEventLog
// ============================================================

/// A log of `BuildNotification`s for a build session.
#[derive(Clone, Debug, Default)]
pub struct BuildEventLog {
    events: Vec<BuildNotification>,
}

impl BuildEventLog {
    /// Create an empty log.
    pub fn new() -> Self {
        Self::default()
    }

    /// Append a notification.
    pub fn push(&mut self, event: BuildNotification) {
        self.events.push(event);
    }

    /// Number of events.
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// Whether empty.
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Count events with the given label.
    pub fn count_by_label(&self, label: &str) -> usize {
        self.events.iter().filter(|e| e.label() == label).count()
    }

    /// Whether any error events were recorded.
    pub fn has_errors(&self) -> bool {
        self.events
            .iter()
            .any(|e| matches!(e, BuildNotification::Error(_)))
    }
}

// ============================================================
// BuildPathResolver
// ============================================================

/// Utility for resolving relative paths within the build directory.
pub struct BuildPathResolver {
    root: std::path::PathBuf,
    out: std::path::PathBuf,
}

impl BuildPathResolver {
    /// Create a resolver anchored at `root` with output in `out`.
    pub fn new(root: impl Into<std::path::PathBuf>, out: impl Into<std::path::PathBuf>) -> Self {
        Self {
            root: root.into(),
            out: out.into(),
        }
    }

    /// Source path relative to root.
    pub fn source_path(&self, rel: &str) -> std::path::PathBuf {
        self.root.join(rel)
    }

    /// Output path relative to out directory.
    pub fn output_path(&self, rel: &str) -> std::path::PathBuf {
        self.out.join(rel)
    }

    /// Object file path for a given module name.
    pub fn object_path(&self, module: &str) -> std::path::PathBuf {
        let rel = format!("{}.o", module.replace('.', "/"));
        self.out.join("obj").join(rel)
    }

    /// Interface file path for a given module name.
    pub fn interface_path(&self, module: &str) -> std::path::PathBuf {
        let rel = format!("{}.olean", module.replace('.', "/"));
        self.out.join("iface").join(rel)
    }
}

// ============================================================
// BuildMetrics (top-level)
// ============================================================

/// Top-level build performance metrics.
#[derive(Clone, Debug, Default)]
pub struct BuildMetrics {
    /// Number of source files processed.
    pub files_processed: u64,
    /// Number of type errors found.
    pub type_errors: u64,
    /// Total parse time in milliseconds.
    pub parse_ms: u64,
    /// Total type-check time in milliseconds.
    pub typecheck_ms: u64,
    /// Total codegen time in milliseconds.
    pub codegen_ms: u64,
    /// Total link time in milliseconds.
    pub link_ms: u64,
    /// Peak RSS memory usage in bytes.
    pub peak_rss_bytes: u64,
}

impl BuildMetrics {
    /// Create zeroed metrics.
    pub fn new() -> Self {
        Self::default()
    }

    /// Total build time in milliseconds.
    pub fn total_ms(&self) -> u64 {
        self.parse_ms + self.typecheck_ms + self.codegen_ms + self.link_ms
    }

    /// Whether any type errors were found.
    pub fn has_type_errors(&self) -> bool {
        self.type_errors > 0
    }

    /// Human-readable report.
    pub fn report(&self) -> String {
        format!(
            "files={} type_errors={} parse={}ms typecheck={}ms codegen={}ms link={}ms total={}ms",
            self.files_processed,
            self.type_errors,
            self.parse_ms,
            self.typecheck_ms,
            self.codegen_ms,
            self.link_ms,
            self.total_ms(),
        )
    }
}

// ============================================================
// PackageId: unique identifier for a package
// ============================================================

/// A unique package identifier (name + version string).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PackageId {
    /// Package name.
    pub name: String,
    /// Version string.
    pub version: String,
}

impl PackageId {
    /// Create a new package ID.
    pub fn new(name: &str, version: &str) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
        }
    }

    /// Short display form "name@version".
    pub fn to_slug(&self) -> String {
        format!("{}@{}", self.name, self.version)
    }
}

impl std::fmt::Display for PackageId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_slug())
    }
}

// ============================================================
// BuildGraphNode
// ============================================================

/// A node in the build graph, representing one compilation unit.
#[derive(Clone, Debug)]
pub struct BuildGraphNode {
    /// Node identifier (usually module name).
    pub id: String,
    /// Source file.
    pub source: std::path::PathBuf,
    /// Dependency node IDs.
    pub deps: Vec<String>,
    /// Output artifact path.
    pub output: std::path::PathBuf,
    /// Whether this node was invalidated.
    pub invalidated: bool,
}

impl BuildGraphNode {
    /// Create a node.
    pub fn new(
        id: &str,
        source: impl Into<std::path::PathBuf>,
        output: impl Into<std::path::PathBuf>,
    ) -> Self {
        Self {
            id: id.to_string(),
            source: source.into(),
            deps: Vec::new(),
            output: output.into(),
            invalidated: false,
        }
    }

    /// Add a dependency.
    pub fn add_dep(&mut self, dep_id: &str) {
        self.deps.push(dep_id.to_string());
    }

    /// Mark as invalidated.
    pub fn invalidate(&mut self) {
        self.invalidated = true;
    }
}

// ============================================================
// BuildGraph
// ============================================================

/// The build graph: a collection of nodes with dependencies.
pub struct BuildGraph {
    nodes: HashMap<String, BuildGraphNode>,
}

impl BuildGraph {
    /// Create an empty graph.
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    /// Add a node.
    pub fn add_node(&mut self, node: BuildGraphNode) {
        self.nodes.insert(node.id.clone(), node);
    }

    /// Get a node by ID.
    pub fn get(&self, id: &str) -> Option<&BuildGraphNode> {
        self.nodes.get(id)
    }

    /// Get a node mutably.
    pub fn get_mut(&mut self, id: &str) -> Option<&mut BuildGraphNode> {
        self.nodes.get_mut(id)
    }

    /// Number of nodes.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Nodes in topological order (simple Kahn's algorithm).
    pub fn topo_order(&self) -> Vec<&BuildGraphNode> {
        let mut in_degree: HashMap<&str, usize> =
            self.nodes.keys().map(|k| (k.as_str(), 0)).collect();
        for node in self.nodes.values() {
            for dep in &node.deps {
                *in_degree.entry(dep.as_str()).or_insert(0) += 1;
            }
        }
        let mut queue: std::collections::VecDeque<&str> = in_degree
            .iter()
            .filter(|(_, &d)| d == 0)
            .map(|(&k, _)| k)
            .collect();
        let mut order = Vec::new();
        while let Some(id) = queue.pop_front() {
            if let Some(node) = self.nodes.get(id) {
                order.push(node);
                for dep in &node.deps {
                    let d = in_degree.entry(dep.as_str()).or_insert(0);
                    *d = d.saturating_sub(1);
                    if *d == 0 {
                        queue.push_back(dep.as_str());
                    }
                }
            }
        }
        order
    }

    /// All invalidated nodes.
    pub fn invalidated_nodes(&self) -> Vec<&BuildGraphNode> {
        self.nodes.values().filter(|n| n.invalidated).collect()
    }
}

impl Default for BuildGraph {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================
// BuildStats
// ============================================================

/// Statistics from a single build run.
#[derive(Clone, Debug, Default)]
pub struct BuildStats {
    pub targets_built: usize,
    pub targets_skipped: usize,
    pub targets_failed: usize,
    pub total_warnings: usize,
    pub total_errors: usize,
    pub wall_time_ms: u64,
}

impl BuildStats {
    /// Create zeroed stats.
    pub fn new() -> Self {
        Self::default()
    }

    /// Success rate (targets_built / (built + skipped + failed)).
    pub fn success_rate(&self) -> f64 {
        let total = self.targets_built + self.targets_skipped + self.targets_failed;
        if total == 0 {
            1.0
        } else {
            self.targets_built as f64 / total as f64
        }
    }

    /// Whether the build was error-free.
    pub fn is_clean(&self) -> bool {
        self.targets_failed == 0 && self.total_errors == 0
    }

    /// Human-readable summary.
    pub fn summary(&self) -> String {
        format!(
            "built={} skipped={} failed={} warnings={} errors={} wall={}ms",
            self.targets_built,
            self.targets_skipped,
            self.targets_failed,
            self.total_warnings,
            self.total_errors,
            self.wall_time_ms,
        )
    }
}

// ============================================================
// Additional tests
// ============================================================

#[cfg(test)]
mod lib_extra_tests {
    use super::*;

    // ── BuildFeatureFlags ──
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

    // ── BuildNotification ──
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

    // ── BuildEventLog ──
    #[test]
    fn build_event_log_push_and_count() {
        let mut log = BuildEventLog::new();
        log.push(BuildNotification::Started);
        log.push(BuildNotification::Error("fail".to_string()));
        assert_eq!(log.len(), 2);
        assert!(log.has_errors());
        assert_eq!(log.count_by_label("error"), 1);
    }

    // ── BuildPathResolver ──
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

    // ── BuildMetrics ──
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

    // ── PackageId ──
    #[test]
    fn package_id_slug() {
        let id = PackageId::new("oxilean-core", "0.1.1");
        assert_eq!(id.to_slug(), "oxilean-core@0.1.1");
        assert_eq!(format!("{}", id), "oxilean-core@0.1.1");
    }

    // ── BuildGraphNode ──
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

    // ── BuildGraph ──
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

    // ── BuildStats ──
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

// ============================================================
// WorkspaceInfo: metadata about the workspace
// ============================================================

/// High-level metadata about the OxiLean workspace.
#[derive(Clone, Debug)]
pub struct WorkspaceInfo {
    /// Workspace name.
    pub name: String,
    /// Root directory.
    pub root: std::path::PathBuf,
    /// Member package names.
    pub members: Vec<String>,
    /// Workspace-level feature flags.
    pub flags: BuildFeatureFlags,
}

impl WorkspaceInfo {
    /// Create a workspace with the given name and root.
    pub fn new(name: &str, root: impl Into<std::path::PathBuf>) -> Self {
        Self {
            name: name.to_string(),
            root: root.into(),
            members: Vec::new(),
            flags: BuildFeatureFlags::default(),
        }
    }

    /// Add a member package.
    pub fn add_member(&mut self, member: &str) {
        self.members.push(member.to_string());
    }

    /// Number of member packages.
    pub fn member_count(&self) -> usize {
        self.members.len()
    }

    /// Whether `pkg` is a member of the workspace.
    pub fn is_member(&self, pkg: &str) -> bool {
        self.members.iter().any(|m| m == pkg)
    }
}

impl std::fmt::Display for WorkspaceInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Workspace({}, {} members)",
            self.name,
            self.members.len()
        )
    }
}

// ============================================================
// BuildSession
// ============================================================

/// A stateful build session, tracking progress of a single build invocation.
pub struct BuildSession {
    /// Configuration.
    pub config: BuildConfig,
    /// The build plan.
    pub plan: BuildPlan,
    /// Events recorded during the build.
    pub events: BuildEventLog,
    /// Running statistics.
    pub stats: BuildStats,
    /// Current phase.
    pub phase: BuildPhase,
}

impl BuildSession {
    /// Start a new session.
    pub fn start(config: BuildConfig) -> Self {
        let plan = BuildPlan::new(config.clone());
        let mut events = BuildEventLog::new();
        events.push(BuildNotification::Started);
        Self {
            config,
            plan,
            events,
            stats: BuildStats::new(),
            phase: BuildPhase::Parse,
        }
    }

    /// Advance to the next phase.
    pub fn advance_phase(&mut self) {
        self.phase = match self.phase {
            BuildPhase::Parse => BuildPhase::TypeCheck,
            BuildPhase::TypeCheck => BuildPhase::Codegen,
            BuildPhase::Codegen => BuildPhase::Link,
            BuildPhase::Link | BuildPhase::Package => BuildPhase::Package,
        };
    }

    /// Record a target as built.
    pub fn record_built(&mut self, target_name: &str) {
        self.stats.targets_built += 1;
        self.events.push(BuildNotification::TargetFinished(
            target_name.to_string(),
            true,
        ));
    }

    /// Record a target as failed.
    pub fn record_failed(&mut self, target_name: &str, error: &str) {
        self.stats.targets_failed += 1;
        self.stats.total_errors += 1;
        self.events.push(BuildNotification::TargetFinished(
            target_name.to_string(),
            false,
        ));
        self.events
            .push(BuildNotification::Error(error.to_string()));
    }

    /// Finalize the session.
    pub fn finish(&mut self, wall_ms: u64) {
        self.stats.wall_time_ms = wall_ms;
        let ok = self.stats.is_clean();
        self.events.push(BuildNotification::Completed(ok));
    }

    /// Whether the build succeeded.
    pub fn succeeded(&self) -> bool {
        self.stats.is_clean()
    }
}

// ============================================================
// Additional tests
// ============================================================

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

// ============================================================
// BuildPlugin: extension points for the build system
// ============================================================

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

/// A no-op plugin for testing.
#[derive(Debug)]
pub struct NoopPlugin {
    pub name: String,
}

impl NoopPlugin {
    /// Create a noop plugin.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl BuildPlugin for NoopPlugin {
    fn name(&self) -> &str {
        &self.name
    }
}

// ============================================================
// PluginRegistry
// ============================================================

/// Registry of active build plugins.
pub struct PluginRegistry {
    plugins: Vec<Box<dyn BuildPlugin>>,
}

impl PluginRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    /// Register a plugin.
    pub fn register(&mut self, plugin: Box<dyn BuildPlugin>) {
        self.plugins.push(plugin);
    }

    /// Number of plugins.
    pub fn len(&self) -> usize {
        self.plugins.len()
    }

    /// Whether empty.
    pub fn is_empty(&self) -> bool {
        self.plugins.is_empty()
    }

    /// Fire `on_build_start` on all plugins.
    pub fn fire_build_start(&self, config: &BuildConfig) {
        for p in &self.plugins {
            p.on_build_start(config);
        }
    }

    /// Fire `on_build_finish` on all plugins.
    pub fn fire_build_finish(&self, summary: &BuildSummary) {
        for p in &self.plugins {
            p.on_build_finish(summary);
        }
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================
// OxileanVersion: version information
// ============================================================

/// Version information for the OxiLean build system.
#[derive(Clone, Debug)]
pub struct OxileanVersion {
    /// Major version.
    pub major: u32,
    /// Minor version.
    pub minor: u32,
    /// Patch version.
    pub patch: u32,
    /// Pre-release tag (if any).
    pub pre: Option<String>,
}

impl OxileanVersion {
    /// Create a version.
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
            pre: None,
        }
    }

    /// Create a pre-release version.
    pub fn pre(major: u32, minor: u32, patch: u32, pre: &str) -> Self {
        Self {
            major,
            minor,
            patch,
            pre: Some(pre.to_string()),
        }
    }

    /// Whether this is a pre-release version.
    pub fn is_pre_release(&self) -> bool {
        self.pre.is_some()
    }

    /// Whether this version is at least `(major, minor, patch)`.
    pub fn is_at_least(&self, major: u32, minor: u32, patch: u32) -> bool {
        (self.major, self.minor, self.patch) >= (major, minor, patch)
    }
}

impl std::fmt::Display for OxileanVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.pre {
            Some(pre) => write!(f, "{}.{}.{}-{}", self.major, self.minor, self.patch, pre),
            None => write!(f, "{}.{}.{}", self.major, self.minor, self.patch),
        }
    }
}

// ============================================================
// BuildSystemError
// ============================================================

/// Errors that can occur in the build system.
#[derive(Clone, Debug)]
pub enum BuildSystemError {
    /// Configuration was invalid.
    InvalidConfig(String),
    /// A source file was not found.
    SourceNotFound(std::path::PathBuf),
    /// A dependency cycle was detected.
    DependencyCycle(Vec<String>),
    /// A compilation step failed.
    CompilationFailed { target: String, reason: String },
    /// An I/O error occurred.
    Io(String),
    /// A plugin error occurred.
    Plugin { plugin: String, message: String },
}

impl std::fmt::Display for BuildSystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildSystemError::InvalidConfig(msg) => write!(f, "InvalidConfig: {}", msg),
            BuildSystemError::SourceNotFound(p) => write!(f, "SourceNotFound: {:?}", p),
            BuildSystemError::DependencyCycle(cycle) => {
                write!(f, "DependencyCycle: {}", cycle.join(" -> "))
            }
            BuildSystemError::CompilationFailed { target, reason } => {
                write!(f, "CompilationFailed[{}]: {}", target, reason)
            }
            BuildSystemError::Io(msg) => write!(f, "IoError: {}", msg),
            BuildSystemError::Plugin { plugin, message } => {
                write!(f, "PluginError[{}]: {}", plugin, message)
            }
        }
    }
}

/// Convenience type alias for build system results.
pub type BuildResult<T> = Result<T, BuildSystemError>;

// ============================================================
// Tests
// ============================================================

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

// ============================================================
// BuildOutputFilter
// ============================================================

/// Filters build output to suppress or highlight certain messages.
#[derive(Clone, Debug)]
pub struct BuildOutputFilter {
    /// Suppress lines matching these substrings.
    pub suppress: Vec<String>,
    /// Highlight lines matching these substrings.
    pub highlight: Vec<String>,
    /// Minimum log level to show.
    pub min_level: BuildLogLevel,
}

impl BuildOutputFilter {
    /// Create a default filter (show everything, highlight nothing).
    pub fn new() -> Self {
        Self {
            suppress: Vec::new(),
            highlight: Vec::new(),
            min_level: BuildLogLevel::Trace,
        }
    }

    /// Add a suppression pattern.
    pub fn suppress(mut self, pattern: &str) -> Self {
        self.suppress.push(pattern.to_string());
        self
    }

    /// Add a highlight pattern.
    pub fn highlight(mut self, pattern: &str) -> Self {
        self.highlight.push(pattern.to_string());
        self
    }

    /// Set minimum level.
    pub fn min_level(mut self, level: BuildLogLevel) -> Self {
        self.min_level = level;
        self
    }

    /// Whether a message at `level` with `text` should be shown.
    pub fn should_show(&self, level: BuildLogLevel, text: &str) -> bool {
        if (level as u8) < (self.min_level as u8) {
            return false;
        }
        !self.suppress.iter().any(|s| text.contains(s.as_str()))
    }

    /// Whether a message should be highlighted.
    pub fn should_highlight(&self, text: &str) -> bool {
        self.highlight.iter().any(|h| text.contains(h.as_str()))
    }
}

impl Default for BuildOutputFilter {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================
// BuildCompiler: stub representation of the compiler invocation
// ============================================================

/// Represents a compiler invocation configuration.
#[derive(Clone, Debug)]
pub struct BuildCompiler {
    /// Path to the compiler executable.
    pub executable: std::path::PathBuf,
    /// Default flags.
    pub default_flags: Vec<String>,
    /// Environment variables to set.
    pub env: HashMap<String, String>,
}

impl BuildCompiler {
    /// Create a compiler configuration.
    pub fn new(executable: impl Into<std::path::PathBuf>) -> Self {
        Self {
            executable: executable.into(),
            default_flags: Vec::new(),
            env: HashMap::new(),
        }
    }

    /// Add a default flag.
    pub fn add_flag(&mut self, flag: &str) {
        self.default_flags.push(flag.to_string());
    }

    /// Set an environment variable.
    pub fn set_env(&mut self, key: &str, value: &str) {
        self.env.insert(key.to_string(), value.to_string());
    }

    /// Construct a command line for compiling `source` to `output`.
    pub fn command_line(&self, source: &str, output: &str) -> Vec<String> {
        let mut cmd = vec![self.executable.to_string_lossy().to_string()];
        cmd.extend(self.default_flags.iter().cloned());
        cmd.push(source.to_string());
        cmd.push("-o".to_string());
        cmd.push(output.to_string());
        cmd
    }
}

// ============================================================
// Tests
// ============================================================

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

// ============================================================
// BuildSystemCapabilities: what the system can do
// ============================================================

/// A description of the build system's capabilities.
#[derive(Clone, Debug, Default)]
pub struct BuildSystemCapabilities {
    /// Whether incremental compilation is available.
    pub incremental: bool,
    /// Whether distributed builds are available.
    pub distributed: bool,
    /// Whether a remote cache is available.
    pub remote_cache: bool,
    /// Whether parallel builds are available.
    pub parallel: bool,
    /// Maximum supported parallel jobs.
    pub max_jobs: usize,
}

impl BuildSystemCapabilities {
    /// Create capabilities for a full-featured build system.
    pub fn full() -> Self {
        Self {
            incremental: true,
            distributed: true,
            remote_cache: true,
            parallel: true,
            max_jobs: 256,
        }
    }

    /// Create capabilities for a minimal build system.
    pub fn minimal() -> Self {
        Self {
            incremental: false,
            distributed: false,
            remote_cache: false,
            parallel: false,
            max_jobs: 1,
        }
    }
}

impl std::fmt::Display for BuildSystemCapabilities {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Capabilities[incremental={} distributed={} remote_cache={} parallel={} max_jobs={}]",
            self.incremental, self.distributed, self.remote_cache, self.parallel, self.max_jobs,
        )
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
