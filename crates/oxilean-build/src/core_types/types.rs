//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashMap;
use std::path::PathBuf;

use super::functions::BuildPlugin;

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
/// A notification event emitted during the build process.
#[derive(Clone, Debug)]
pub enum BuildNotification {
    /// Build started.
    Started,
    /// A target began compiling.
    TargetStarted(String),
    /// A target finished compiling.
    TargetFinished(String, bool),
    /// Build completed.
    Completed(bool),
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
/// A simple cache of compiled module results.
#[derive(Clone, Debug, Default)]
pub struct BuildCache {
    /// Cached entries: module_name -> output_path.
    pub entries: std::collections::HashMap<String, std::path::PathBuf>,
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
/// Environment variables available to build scripts.
#[derive(Clone, Debug, Default)]
pub struct BuildEnvironment {
    /// Variable name → value pairs.
    pub vars: std::collections::HashMap<String, String>,
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
