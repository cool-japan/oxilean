//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{Declaration, ReducibilityHint};
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum BuildEvent {
    Started { file: String },
    Succeeded { file: String, duration_ms: u64 },
    Failed { file: String, error: String },
    Skipped { file: String, reason: String },
    Warning { file: String, message: String },
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BuildCacheEntry {
    pub source_hash: u64,
    pub output_path: std::path::PathBuf,
    pub built_at: std::time::SystemTime,
}
#[allow(dead_code)]
pub struct BuildFilter {
    include_patterns: Vec<String>,
    exclude_patterns: Vec<String>,
}
#[allow(dead_code)]
impl BuildFilter {
    pub fn new() -> Self {
        Self {
            include_patterns: Vec::new(),
            exclude_patterns: Vec::new(),
        }
    }
    pub fn include(mut self, pattern: &str) -> Self {
        self.include_patterns.push(pattern.to_string());
        self
    }
    pub fn exclude(mut self, pattern: &str) -> Self {
        self.exclude_patterns.push(pattern.to_string());
        self
    }
    pub fn matches(&self, path: &str) -> bool {
        let excluded = self
            .exclude_patterns
            .iter()
            .any(|p| path.contains(p.as_str()));
        if excluded {
            return false;
        }
        if self.include_patterns.is_empty() {
            return true;
        }
        self.include_patterns
            .iter()
            .any(|p| path.contains(p.as_str()))
    }
}
/// The build executor drives the compilation pipeline.
#[derive(Debug)]
pub struct BuildExecutor {
    /// Build configuration.
    config: BuildConfig,
    /// Build dependency graph.
    graph: BuildGraph,
    /// Accumulated results.
    results: Vec<BuildStepResult>,
    /// When the build started.
    start_time: Instant,
}
impl BuildExecutor {
    /// Create a new executor for the given configuration and graph.
    pub fn new(config: BuildConfig, graph: BuildGraph) -> Self {
        Self {
            config,
            graph,
            results: Vec::new(),
            start_time: Instant::now(),
        }
    }
    /// Run the full build, returning a report.
    pub fn execute_build(&mut self) -> BuildReport {
        self.start_time = Instant::now();
        self.results.clear();
        let order = match self.graph.topological_order() {
            Ok(o) => o,
            Err(msg) => {
                self.results.push(BuildStepResult {
                    module: "<graph>".to_string(),
                    status: BuildStatus::Failure,
                    duration: Duration::ZERO,
                    diagnostics: vec![msg],
                });
                return self.make_report();
            }
        };
        for module in &order {
            let result = self.execute_step(module);
            self.results.push(result);
        }
        self.make_report()
    }
    /// Compile a single module through the OxiLean pipeline.
    pub fn execute_step(&self, module_name: &str) -> BuildStepResult {
        let step_start = Instant::now();
        let node = match self.graph.get_node(module_name) {
            Some(n) => n,
            None => {
                return BuildStepResult {
                    module: module_name.to_string(),
                    status: BuildStatus::Failure,
                    duration: step_start.elapsed(),
                    diagnostics: vec![format!("module '{}' not found in graph", module_name)],
                };
            }
        };
        if !node.is_stale && !self.config.force_rebuild {
            return BuildStepResult {
                module: module_name.to_string(),
                status: BuildStatus::Skipped,
                duration: step_start.elapsed(),
                diagnostics: Vec::new(),
            };
        }
        if !self.config.force_rebuild {
            if let Some(cached) = self.check_cache(module_name, node.hash) {
                return cached;
            }
        }
        let source = match std::fs::read_to_string(&node.source_path) {
            Ok(s) => s,
            Err(e) => {
                return BuildStepResult {
                    module: module_name.to_string(),
                    status: BuildStatus::Failure,
                    duration: step_start.elapsed(),
                    diagnostics: vec![format!("cannot read {}: {}", node.source_path.display(), e)],
                };
            }
        };
        let mut diagnostics = Vec::new();
        let mut lexer = oxilean_parse::Lexer::new(&source);
        let tokens = lexer.tokenize();
        if tokens.is_empty() {
            diagnostics.push("warning: source file produced no tokens".to_string());
        }
        let mut parser = oxilean_parse::Parser::new(tokens);
        let mut decls = Vec::new();
        loop {
            match parser.parse_decl() {
                Ok(d) => decls.push(d),
                Err(e) => {
                    let msg = e.to_string();
                    if msg.contains("end of file") {
                        break;
                    }
                    return BuildStepResult {
                        module: module_name.to_string(),
                        status: BuildStatus::Failure,
                        duration: step_start.elapsed(),
                        diagnostics: vec![format!("parse error: {}", msg)],
                    };
                }
            }
        }
        let mut env = oxilean_kernel::Environment::new();
        for surface_decl in &decls {
            match oxilean_elab::elaborate_decl(&env, &surface_decl.value) {
                Ok(pending) => {
                    use oxilean_elab::PendingDecl;
                    use oxilean_kernel::{Declaration, ReducibilityHint};
                    let kernel_decl = match pending {
                        PendingDecl::Definition { name, ty, val, .. } => Declaration::Definition {
                            name,
                            univ_params: vec![],
                            ty,
                            val,
                            hint: ReducibilityHint::Regular(0),
                        },
                        PendingDecl::Theorem {
                            name, ty, proof, ..
                        } => Declaration::Theorem {
                            name,
                            univ_params: vec![],
                            ty,
                            val: proof,
                        },
                        PendingDecl::Axiom { name, ty, .. } => Declaration::Axiom {
                            name,
                            univ_params: vec![],
                            ty,
                        },
                        PendingDecl::Inductive { name, ty, .. } => Declaration::Axiom {
                            name,
                            univ_params: vec![],
                            ty,
                        },
                        PendingDecl::Opaque { name, ty, val } => Declaration::Opaque {
                            name,
                            univ_params: vec![],
                            ty,
                            val,
                        },
                    };
                    if let Err(e) = oxilean_kernel::check_declaration(&mut env, kernel_decl) {
                        return BuildStepResult {
                            module: module_name.to_string(),
                            status: BuildStatus::Failure,
                            duration: step_start.elapsed(),
                            diagnostics: vec![format!("type error: {}", e)],
                        };
                    }
                }
                Err(e) => {
                    return BuildStepResult {
                        module: module_name.to_string(),
                        status: BuildStatus::Failure,
                        duration: step_start.elapsed(),
                        diagnostics: vec![format!("elaboration error: {:?}", e)],
                    };
                }
            }
        }
        if self.config.verbose {
            diagnostics.push(format!(
                "checked {} declaration(s) in {}",
                decls.len(),
                module_name,
            ));
        }
        let result = BuildStepResult {
            module: module_name.to_string(),
            status: BuildStatus::Success,
            duration: step_start.elapsed(),
            diagnostics,
        };
        self.write_cache(module_name, node.hash);
        result
    }
    /// Try to load a cached result for the given module + hash.
    fn check_cache(&self, module_name: &str, hash: u64) -> Option<BuildStepResult> {
        let cache_path = self.config.cache_dir.join(format!("{}.cache", module_name));
        if let Ok(data) = std::fs::read_to_string(&cache_path) {
            if let Ok(cached_hash) = data.trim().parse::<u64>() {
                if cached_hash == hash {
                    return Some(BuildStepResult {
                        module: module_name.to_string(),
                        status: BuildStatus::Cached,
                        duration: Duration::ZERO,
                        diagnostics: Vec::new(),
                    });
                }
            }
        }
        None
    }
    /// Write a cache entry for the given module.
    fn write_cache(&self, module_name: &str, hash: u64) {
        let cache_path = self.config.cache_dir.join(format!("{}.cache", module_name));
        if let Some(parent) = cache_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let _ = std::fs::write(&cache_path, hash.to_string());
    }
    /// Build a report from accumulated results.
    fn make_report(&self) -> BuildReport {
        let mut succeeded = 0usize;
        let mut failed = 0usize;
        let mut cached = 0usize;
        for r in &self.results {
            match r.status {
                BuildStatus::Success => succeeded += 1,
                BuildStatus::Failure => failed += 1,
                BuildStatus::Cached => cached += 1,
                BuildStatus::Skipped => {}
            }
        }
        BuildReport {
            total_modules: self.graph.node_count(),
            succeeded,
            failed,
            cached,
            elapsed: self.start_time.elapsed(),
            steps: self.results.clone(),
        }
    }
}
/// Optimisation level.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum OptLevel {
    /// No optimisations, full debug info.
    Debug,
    /// Full optimisations.
    Release,
    /// Optimise for binary size.
    Size,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DependencyVersion {
    pub name: String,
    pub version: String,
    pub hash: String,
}
/// Kind of build target (for BuildTargetConfig).
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BuildTargetKind {
    Library,
    Binary,
    Test,
    Benchmark,
}
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct BuildMetrics {
    pub compiled: usize,
    pub skipped: usize,
    pub failed: usize,
    pub total_duration_ms: u64,
}
#[allow(dead_code)]
impl BuildMetrics {
    pub fn record_compiled(&mut self, duration_ms: u64) {
        self.compiled += 1;
        self.total_duration_ms += duration_ms;
    }
    pub fn record_skipped(&mut self) {
        self.skipped += 1;
    }
    pub fn record_failed(&mut self) {
        self.failed += 1;
    }
    pub fn success_rate(&self) -> f64 {
        let total = self.compiled + self.failed;
        if total == 0 {
            return 1.0;
        }
        self.compiled as f64 / total as f64
    }
    pub fn avg_duration_ms(&self) -> f64 {
        if self.compiled == 0 {
            return 0.0;
        }
        self.total_duration_ms as f64 / self.compiled as f64
    }
    pub fn to_summary(&self) -> String {
        format!(
            "Compiled: {} Skipped: {} Failed: {} Success: {:.1}%",
            self.compiled,
            self.skipped,
            self.failed,
            self.success_rate() * 100.0
        )
    }
}
/// A record of a past build.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct BuildHistoryEntry {
    pub timestamp_secs: u64,
    pub success: bool,
    pub duration_ms: u64,
    pub stage_reached: String,
    pub error_count: usize,
    pub warning_count: usize,
}
/// Result of compiling a single module.
#[derive(Clone, Debug)]
pub struct BuildStepResult {
    /// Module name.
    pub module: String,
    /// Final status.
    pub status: BuildStatus,
    /// Wall-clock time for this step.
    pub duration: Duration,
    /// Diagnostic messages (errors, warnings).
    pub diagnostics: Vec<String>,
}
impl BuildStepResult {
    /// Return true if the step succeeded (including cached).
    pub fn is_ok(&self) -> bool {
        matches!(
            self.status,
            BuildStatus::Success | BuildStatus::Cached | BuildStatus::Skipped
        )
    }
}
/// Manages compiled artifacts on disk.
#[derive(Clone, Debug)]
pub struct ArtifactStore {
    /// Root directory where artifacts are stored.
    root_dir: PathBuf,
}
impl ArtifactStore {
    /// Create a new artifact store at the given directory.
    pub fn new(root_dir: impl Into<PathBuf>) -> Self {
        Self {
            root_dir: root_dir.into(),
        }
    }
    /// Compute the path on disk for a given module name.
    pub fn artifact_path(&self, module_name: &str) -> PathBuf {
        let sanitized = module_name.replace('.', "/");
        self.root_dir.join(format!("{}.olean", sanitized))
    }
    /// Store compiled data for a module.
    pub fn store_artifact(&self, module_name: &str, data: &[u8]) -> Result<(), String> {
        let path = self.artifact_path(module_name);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("cannot create artifact dir: {}", e))?;
        }
        std::fs::write(&path, data)
            .map_err(|e| format!("cannot write artifact '{}': {}", path.display(), e))
    }
    /// Load compiled data for a module.
    pub fn load_artifact(&self, module_name: &str) -> Result<Vec<u8>, String> {
        let path = self.artifact_path(module_name);
        std::fs::read(&path)
            .map_err(|e| format!("cannot read artifact '{}': {}", path.display(), e))
    }
    /// Invalidate (remove) the artifact for a module.
    pub fn invalidate_artifact(&self, module_name: &str) -> Result<(), String> {
        let path = self.artifact_path(module_name);
        if path.exists() {
            std::fs::remove_file(&path)
                .map_err(|e| format!("cannot remove artifact '{}': {}", path.display(), e))?;
        }
        Ok(())
    }
    /// Remove all artifacts.
    pub fn clean_artifacts(&self) -> Result<usize, String> {
        if !self.root_dir.exists() {
            return Ok(0);
        }
        let mut count = 0usize;
        let entries = std::fs::read_dir(&self.root_dir)
            .map_err(|e| format!("cannot read artifact dir: {}", e))?;
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("olean")
                && std::fs::remove_file(&path).is_ok()
            {
                count += 1;
            }
        }
        Ok(count)
    }
    /// Return the total size of all artifacts in bytes.
    pub fn total_size(&self) -> u64 {
        if !self.root_dir.exists() {
            return 0;
        }
        let mut total = 0u64;
        if let Ok(entries) = std::fs::read_dir(&self.root_dir) {
            for entry in entries.flatten() {
                if let Ok(meta) = entry.metadata() {
                    total += meta.len();
                }
            }
        }
        total
    }
}
/// A stage in the build pipeline.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct BuildStage {
    pub name: String,
    pub description: String,
    pub required: bool,
    pub parallel: bool,
}
impl BuildStage {
    /// Create a required sequential stage.
    #[allow(dead_code)]
    pub fn required(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            required: true,
            parallel: false,
        }
    }
    /// Create an optional parallel stage.
    #[allow(dead_code)]
    pub fn optional_parallel(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            required: false,
            parallel: true,
        }
    }
}
/// A named build target with explicit source/output config.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct BuildTargetConfig {
    pub name: String,
    pub sources: Vec<String>,
    pub dependencies: Vec<String>,
    pub output: String,
    pub build_type: BuildTargetKind,
}
impl BuildTargetConfig {
    /// Create a library target.
    #[allow(dead_code)]
    pub fn library(
        name: impl Into<String>,
        sources: Vec<String>,
        output: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            sources,
            dependencies: vec![],
            output: output.into(),
            build_type: BuildTargetKind::Library,
        }
    }
    /// Create a binary target.
    #[allow(dead_code)]
    pub fn binary(
        name: impl Into<String>,
        sources: Vec<String>,
        output: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            sources,
            dependencies: vec![],
            output: output.into(),
            build_type: BuildTargetKind::Binary,
        }
    }
    /// Add a dependency.
    #[allow(dead_code)]
    pub fn with_dependency(mut self, dep: impl Into<String>) -> Self {
        self.dependencies.push(dep.into());
        self
    }
}
/// A validation error.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct BuildValidationError {
    pub field: String,
    pub message: String,
}
#[allow(dead_code)]
pub struct ArtifactRegistry {
    artifacts: Vec<BuildArtifact>,
}
#[allow(dead_code)]
impl ArtifactRegistry {
    pub fn new() -> Self {
        Self {
            artifacts: Vec::new(),
        }
    }
    pub fn add(&mut self, artifact: BuildArtifact) {
        self.artifacts.push(artifact);
    }
    pub fn by_kind(&self, kind: &ArtifactKind) -> Vec<&BuildArtifact> {
        self.artifacts.iter().filter(|a| &a.kind == kind).collect()
    }
    pub fn total_size(&self) -> u64 {
        self.artifacts.iter().map(|a| a.size_bytes).sum()
    }
    pub fn count(&self) -> usize {
        self.artifacts.len()
    }
    pub fn find_by_source(&self, source: &str) -> Option<&BuildArtifact> {
        self.artifacts.iter().find(|a| a.source == source)
    }
}
/// History of past builds.
#[allow(dead_code)]
pub struct BuildHistory {
    pub(crate) entries: Vec<BuildHistoryEntry>,
    max_entries: usize,
}
impl BuildHistory {
    /// Create a new history store.
    #[allow(dead_code)]
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: vec![],
            max_entries,
        }
    }
    /// Record a build.
    #[allow(dead_code)]
    pub fn record(&mut self, entry: BuildHistoryEntry) {
        if self.entries.len() >= self.max_entries {
            self.entries.remove(0);
        }
        self.entries.push(entry);
    }
    /// Return the last N entries.
    #[allow(dead_code)]
    pub fn last_n(&self, n: usize) -> Vec<&BuildHistoryEntry> {
        self.entries.iter().rev().take(n).collect()
    }
    /// Return the success rate.
    #[allow(dead_code)]
    pub fn success_rate(&self) -> f64 {
        if self.entries.is_empty() {
            return 0.0;
        }
        let success = self.entries.iter().filter(|e| e.success).count();
        100.0 * success as f64 / self.entries.len() as f64
    }
}
#[allow(dead_code)]
pub struct BuildEventRecorder {
    events: Vec<(std::time::Instant, BuildEvent)>,
    max_events: usize,
}
#[allow(dead_code)]
impl BuildEventRecorder {
    pub fn new(max_events: usize) -> Self {
        Self {
            events: Vec::new(),
            max_events,
        }
    }
    pub fn record(&mut self, event: BuildEvent) {
        if self.events.len() >= self.max_events {
            self.events.remove(0);
        }
        self.events.push((std::time::Instant::now(), event));
    }
    pub fn failures(&self) -> Vec<&BuildEvent> {
        self.events
            .iter()
            .filter(|(_, e)| matches!(e, BuildEvent::Failed { .. }))
            .map(|(_, e)| e)
            .collect()
    }
    pub fn successes(&self) -> Vec<&BuildEvent> {
        self.events
            .iter()
            .filter(|(_, e)| matches!(e, BuildEvent::Succeeded { .. }))
            .map(|(_, e)| e)
            .collect()
    }
    pub fn event_count(&self) -> usize {
        self.events.len()
    }
}
/// Configuration for a build session.
#[derive(Clone, Debug)]
pub struct BuildConfig {
    /// Root directory of the project.
    pub project_root: PathBuf,
    /// Directory for build outputs.
    pub output_dir: PathBuf,
    /// Maximum number of parallel compilation units.
    pub parallelism: usize,
    /// Enable verbose logging.
    pub verbose: bool,
    /// Force a full rebuild regardless of cache state.
    pub force_rebuild: bool,
    /// Directory for build caches.
    pub cache_dir: PathBuf,
    /// The kind of build to perform.
    pub target: BuildTarget,
    /// Optimisation level.
    pub opt_level: OptLevel,
}
impl BuildConfig {
    /// Create a build configuration rooted at the given project directory.
    pub fn for_project(root: impl Into<PathBuf>) -> Self {
        let root = root.into();
        let output_dir = root.join("build");
        let cache_dir = output_dir.join("cache");
        Self {
            project_root: root,
            output_dir,
            cache_dir,
            ..Default::default()
        }
    }
    /// Return the effective optimisation level taking the target into account.
    pub fn effective_opt_level(&self) -> OptLevel {
        match self.target {
            BuildTarget::Release | BuildTarget::Bench => OptLevel::Release,
            _ => self.opt_level,
        }
    }
    /// Check whether the configuration requests parallel builds.
    pub fn is_parallel(&self) -> bool {
        self.parallelism > 1
    }
    /// Validate the configuration, returning an error message if invalid.
    pub fn validate(&self) -> Result<(), String> {
        if self.parallelism == 0 {
            return Err("parallelism must be at least 1".to_string());
        }
        Ok(())
    }
}
#[allow(dead_code)]
pub struct BuildLockfile {
    pub dependencies: Vec<DependencyVersion>,
    pub generated_at: String,
}
#[allow(dead_code)]
impl BuildLockfile {
    pub fn new() -> Self {
        Self {
            dependencies: Vec::new(),
            generated_at: "2026-02-28".to_string(),
        }
    }
    pub fn add_dep(&mut self, name: &str, version: &str, hash: &str) {
        self.dependencies.push(DependencyVersion {
            name: name.to_string(),
            version: version.to_string(),
            hash: hash.to_string(),
        });
    }
    pub fn find(&self, name: &str) -> Option<&DependencyVersion> {
        self.dependencies.iter().find(|d| d.name == name)
    }
    pub fn to_toml(&self) -> String {
        let mut out = format!("# Generated at {}\n\n", self.generated_at);
        for dep in &self.dependencies {
            out.push_str(&format!(
                "[[dependency]]\nname = \"{}\"\nversion = \"{}\"\nhash = \"{}\"\n\n",
                dep.name, dep.version, dep.hash
            ));
        }
        out
    }
}
/// Summary report for a build run.
#[derive(Clone, Debug)]
pub struct BuildReport {
    /// Total number of modules in the build graph.
    pub total_modules: usize,
    /// Number of modules that compiled successfully.
    pub succeeded: usize,
    /// Number of modules that failed to compile.
    pub failed: usize,
    /// Number of modules served from cache.
    pub cached: usize,
    /// Total wall-clock time.
    pub elapsed: Duration,
    /// Per-module results.
    pub steps: Vec<BuildStepResult>,
}
impl BuildReport {
    /// Return true if there were no failures.
    pub fn is_success(&self) -> bool {
        self.failed == 0
    }
    /// Return the list of failed modules.
    pub fn failed_modules(&self) -> Vec<&str> {
        self.steps
            .iter()
            .filter(|s| s.status == BuildStatus::Failure)
            .map(|s| s.module.as_str())
            .collect()
    }
    /// Collect all diagnostic messages.
    pub fn all_diagnostics(&self) -> Vec<&str> {
        self.steps
            .iter()
            .flat_map(|s| s.diagnostics.iter().map(|d| d.as_str()))
            .collect()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BuildLogEntry {
    pub level: LogLevel,
    pub message: String,
    pub source: Option<String>,
    pub timestamp: std::time::Instant,
}
#[allow(dead_code)]
pub struct BuildLog {
    entries: Vec<BuildLogEntry>,
    max_entries: usize,
}
#[allow(dead_code)]
impl BuildLog {
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: Vec::new(),
            max_entries,
        }
    }
    pub fn log(&mut self, level: LogLevel, message: &str, source: Option<&str>) {
        if self.entries.len() >= self.max_entries {
            self.entries.remove(0);
        }
        self.entries.push(BuildLogEntry {
            level,
            message: message.to_string(),
            source: source.map(|s| s.to_string()),
            timestamp: std::time::Instant::now(),
        });
    }
    pub fn info(&mut self, msg: &str) {
        self.log(LogLevel::Info, msg, None);
    }
    pub fn warn(&mut self, msg: &str) {
        self.log(LogLevel::Warning, msg, None);
    }
    pub fn error(&mut self, msg: &str) {
        self.log(LogLevel::Error, msg, None);
    }
    pub fn errors(&self) -> Vec<&BuildLogEntry> {
        self.entries
            .iter()
            .filter(|e| e.level == LogLevel::Error)
            .collect()
    }
    pub fn warnings(&self) -> Vec<&BuildLogEntry> {
        self.entries
            .iter()
            .filter(|e| e.level == LogLevel::Warning)
            .collect()
    }
    pub fn last_n(&self, n: usize) -> &[BuildLogEntry] {
        let start = self.entries.len().saturating_sub(n);
        &self.entries[start..]
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BuildArtifact {
    pub source: String,
    pub output: std::path::PathBuf,
    pub kind: ArtifactKind,
    pub size_bytes: u64,
}
/// A single node in the build dependency graph.
#[derive(Clone, Debug)]
pub struct BuildNode {
    /// Fully qualified module name (e.g. `Mathlib.Algebra.Group`).
    pub module_name: String,
    /// Path to the source file.
    pub source_path: PathBuf,
    /// Module names this node directly depends on.
    pub dependencies: Vec<String>,
    /// Path where the compiled output will be written.
    pub output_path: PathBuf,
    /// Whether this node needs to be rebuilt.
    pub is_stale: bool,
    /// Content hash of the source file.
    pub hash: u64,
}
impl BuildNode {
    /// Create a new build node.
    pub fn new(module_name: String, source_path: PathBuf, output_path: PathBuf) -> Self {
        Self {
            module_name,
            source_path,
            dependencies: Vec::new(),
            output_path,
            is_stale: true,
            hash: 0,
        }
    }
    /// Add a dependency on another module.
    pub fn add_dependency(&mut self, dep: String) {
        if !self.dependencies.contains(&dep) {
            self.dependencies.push(dep);
        }
    }
    /// Return the number of direct dependencies.
    pub fn dependency_count(&self) -> usize {
        self.dependencies.len()
    }
}
#[allow(dead_code)]
pub struct BuildCache {
    entries: std::collections::HashMap<String, BuildCacheEntry>,
}
#[allow(dead_code)]
impl BuildCache {
    pub fn new() -> Self {
        Self {
            entries: std::collections::HashMap::new(),
        }
    }
    pub fn insert(&mut self, source: &str, entry: BuildCacheEntry) {
        self.entries.insert(source.to_string(), entry);
    }
    pub fn get(&self, source: &str) -> Option<&BuildCacheEntry> {
        self.entries.get(source)
    }
    pub fn is_stale(&self, source: &str, current_hash: u64) -> bool {
        match self.get(source) {
            None => true,
            Some(e) => e.source_hash != current_hash,
        }
    }
    pub fn invalidate(&mut self, source: &str) {
        self.entries.remove(source);
    }
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    pub fn size(&self) -> usize {
        self.entries.len()
    }
}
/// A multi-stage build pipeline.
#[allow(dead_code)]
pub struct BuildPipeline {
    stages: Vec<BuildStage>,
}
impl BuildPipeline {
    /// Create a default OxiLean build pipeline.
    #[allow(dead_code)]
    pub fn default_oxilean() -> Self {
        Self {
            stages: vec![
                BuildStage::required("parse", "Parse all source files"),
                BuildStage::required("elaborate", "Elaborate declarations"),
                BuildStage::required("typecheck", "Type-check elaborated terms"),
                BuildStage::optional_parallel("codegen", "Generate code"),
                BuildStage::optional_parallel("optimize", "Optimize output"),
                BuildStage::required("link", "Link artifacts"),
            ],
        }
    }
    /// Return stage names.
    #[allow(dead_code)]
    pub fn stage_names(&self) -> Vec<&str> {
        self.stages.iter().map(|s| s.name.as_str()).collect()
    }
    /// Return required stages.
    #[allow(dead_code)]
    pub fn required_stages(&self) -> Vec<&BuildStage> {
        self.stages.iter().filter(|s| s.required).collect()
    }
}
/// Severity of a build diagnostic.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BuildDiagnosticSeverity {
    Error,
    Warning,
    Note,
}
#[allow(dead_code)]
pub struct ParallelBuildPlan {
    pub waves: Vec<Vec<String>>,
}
#[allow(dead_code)]
impl ParallelBuildPlan {
    pub fn from_deps(deps: &std::collections::HashMap<String, Vec<String>>) -> Self {
        let mut in_degree: std::collections::HashMap<&str, usize> =
            std::collections::HashMap::new();
        for (node, _) in deps {
            in_degree.entry(node.as_str()).or_insert(0);
        }
        for deps_list in deps.values() {
            for dep in deps_list {
                *in_degree.entry(dep.as_str()).or_insert(0) += 1;
            }
        }
        let mut waves = Vec::new();
        while !in_degree.is_empty() {
            let wave: Vec<String> = in_degree
                .iter()
                .filter(|(_, &d)| d == 0)
                .map(|(n, _)| n.to_string())
                .collect();
            if wave.is_empty() {
                break;
            }
            for node in &wave {
                in_degree.remove(node.as_str());
                if let Some(dependents) = deps.get(node.as_str()) {
                    for dep in dependents {
                        if let Some(d) = in_degree.get_mut(dep.as_str()) {
                            *d = d.saturating_sub(1);
                        }
                    }
                }
            }
            waves.push(wave);
        }
        Self { waves }
    }
    pub fn wave_count(&self) -> usize {
        self.waves.len()
    }
    pub fn total_nodes(&self) -> usize {
        self.waves.iter().map(|w| w.len()).sum()
    }
}
/// Build target kind.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BuildTarget {
    /// Type-check only (no code generation).
    Check,
    /// Full build with debug info.
    Build,
    /// Optimised release build.
    Release,
    /// Build and run tests.
    Test,
    /// Build and run benchmarks.
    Bench,
    /// Generate documentation.
    Doc,
}
/// Status of a single build step.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BuildStatus {
    /// Compilation succeeded.
    Success,
    /// Compilation failed.
    Failure,
    /// Result was served from cache.
    Cached,
    /// Step was skipped (e.g. not stale).
    Skipped,
}
/// Directed acyclic graph of build nodes.
#[derive(Clone, Debug)]
pub struct BuildGraph {
    /// All nodes keyed by module name.
    nodes: HashMap<String, BuildNode>,
    /// Adjacency list: module -> modules that depend on it.
    reverse_deps: HashMap<String, Vec<String>>,
}
impl BuildGraph {
    /// Create an empty build graph.
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            reverse_deps: HashMap::new(),
        }
    }
    /// Add a node to the graph.
    pub fn add_node(&mut self, node: BuildNode) {
        let name = node.module_name.clone();
        for dep in &node.dependencies {
            self.reverse_deps
                .entry(dep.clone())
                .or_default()
                .push(name.clone());
        }
        self.nodes.insert(name, node);
    }
    /// Retrieve a node by module name.
    pub fn get_node(&self, name: &str) -> Option<&BuildNode> {
        self.nodes.get(name)
    }
    /// Retrieve a mutable reference to a node.
    pub fn get_node_mut(&mut self, name: &str) -> Option<&mut BuildNode> {
        self.nodes.get_mut(name)
    }
    /// Return all node names.
    pub fn module_names(&self) -> Vec<String> {
        self.nodes.keys().cloned().collect()
    }
    /// Return the number of nodes.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
    /// Return true if the graph contains a node with the given name.
    pub fn contains(&self, name: &str) -> bool {
        self.nodes.contains_key(name)
    }
    /// Compute staleness for each node by comparing content hashes and
    /// checking whether any dependency is stale.
    pub fn compute_staleness(&mut self, cache: &HashMap<String, u64>) {
        let names: Vec<String> = self.nodes.keys().cloned().collect();
        for name in &names {
            if let Some(node) = self.nodes.get_mut(name) {
                match cache.get(name) {
                    Some(&cached_hash) if cached_hash == node.hash => {
                        node.is_stale = false;
                    }
                    _ => {
                        node.is_stale = true;
                    }
                }
            }
        }
        let mut changed = true;
        while changed {
            changed = false;
            for name in &names {
                let stale = {
                    let node = &self.nodes[name];
                    if node.is_stale {
                        continue;
                    }
                    node.dependencies
                        .iter()
                        .any(|d| self.nodes.get(d).is_some_and(|n| n.is_stale))
                };
                if stale {
                    self.nodes
                        .get_mut(name)
                        .expect("name exists in nodes: iterating over self.nodes keys")
                        .is_stale = true;
                    changed = true;
                }
            }
        }
    }
    /// Return a topological ordering of the modules that is safe for
    /// parallel compilation (all dependencies of a module appear before it).
    pub fn topological_order(&self) -> Result<Vec<String>, String> {
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        for (name, node) in &self.nodes {
            in_degree.entry(name.clone()).or_insert(0);
            for dep in &node.dependencies {
                if self.nodes.contains_key(dep) {
                    *in_degree.entry(name.clone()).or_insert(0) += 1;
                }
            }
        }
        let mut queue: VecDeque<String> = in_degree
            .iter()
            .filter(|(_, &d)| d == 0)
            .map(|(n, _)| n.clone())
            .collect();
        let mut order = Vec::with_capacity(self.nodes.len());
        while let Some(name) = queue.pop_front() {
            order.push(name.clone());
            if let Some(dependents) = self.reverse_deps.get(&name) {
                for dep in dependents {
                    if let Some(d) = in_degree.get_mut(dep) {
                        *d = d.saturating_sub(1);
                        if *d == 0 {
                            queue.push_back(dep.clone());
                        }
                    }
                }
            }
        }
        if order.len() != self.nodes.len() {
            return Err("cyclic dependency detected in build graph".to_string());
        }
        Ok(order)
    }
    /// Return the set of modules that need to be rebuilt when the given
    /// module changes (transitive dependents).
    pub fn affected_nodes(&self, changed: &str) -> HashSet<String> {
        let mut affected = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(changed.to_string());
        while let Some(name) = queue.pop_front() {
            if affected.contains(&name) {
                continue;
            }
            affected.insert(name.clone());
            if let Some(dependents) = self.reverse_deps.get(&name) {
                for dep in dependents {
                    if !affected.contains(dep) {
                        queue.push_back(dep.clone());
                    }
                }
            }
        }
        affected
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum ArtifactKind {
    ObjectFile,
    Binary,
    WasmModule,
    DocumentationFile,
    TestBinary,
}
#[allow(dead_code)]
pub struct BuildSummaryReport {
    pub files_compiled: usize,
    pub files_skipped: usize,
    pub files_failed: usize,
    pub total_duration_ms: u64,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}
#[allow(dead_code)]
impl BuildSummaryReport {
    pub fn new() -> Self {
        Self {
            files_compiled: 0,
            files_skipped: 0,
            files_failed: 0,
            total_duration_ms: 0,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }
    pub fn is_success(&self) -> bool {
        self.files_failed == 0
    }
    pub fn to_string(&self) -> String {
        format!(
            "Build: {} compiled, {} skipped, {} failed in {}ms. {} error(s), {} warning(s).",
            self.files_compiled,
            self.files_skipped,
            self.files_failed,
            self.total_duration_ms,
            self.errors.len(),
            self.warnings.len()
        )
    }
}
/// Validates build configuration for correctness.
#[allow(dead_code)]
pub struct BuildConfigValidator;
impl BuildConfigValidator {
    /// Validate a build environment.
    #[allow(dead_code)]
    pub fn validate_env(env: &BuildEnvironment) -> Vec<BuildValidationError> {
        let mut errors = vec![];
        if env.jobs == 0 {
            errors.push(BuildValidationError {
                field: "jobs".to_string(),
                message: "jobs must be >= 1".to_string(),
            });
        }
        if env.optimization_level > 3 {
            errors.push(BuildValidationError {
                field: "optimization_level".to_string(),
                message: "optimization_level must be 0-3".to_string(),
            });
        }
        if env.output_dir.is_empty() {
            errors.push(BuildValidationError {
                field: "output_dir".to_string(),
                message: "output_dir must not be empty".to_string(),
            });
        }
        errors
    }
}
/// Environment configuration for building.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct BuildEnvironment {
    pub oxilean_path: String,
    pub stdlib_path: String,
    pub output_dir: String,
    pub cache_dir: String,
    pub jobs: usize,
    pub optimization_level: u8,
    pub debug_info: bool,
    pub warnings_as_errors: bool,
}
impl BuildEnvironment {
    /// Create a development environment.
    #[allow(dead_code)]
    pub fn development() -> Self {
        Self {
            oxilean_path: "/usr/local/oxilean".to_string(),
            stdlib_path: "/usr/local/oxilean/lib".to_string(),
            output_dir: "target/debug".to_string(),
            cache_dir: ".oxilean_cache".to_string(),
            jobs: 4,
            optimization_level: 0,
            debug_info: true,
            warnings_as_errors: false,
        }
    }
    /// Create a release environment.
    #[allow(dead_code)]
    pub fn release() -> Self {
        Self {
            oxilean_path: "/usr/local/oxilean".to_string(),
            stdlib_path: "/usr/local/oxilean/lib".to_string(),
            output_dir: "target/release".to_string(),
            cache_dir: ".oxilean_cache".to_string(),
            jobs: num_cpus_estimate(),
            optimization_level: 3,
            debug_info: false,
            warnings_as_errors: true,
        }
    }
}
/// A diagnostic emitted during the build process.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct BuildDiagnostic {
    pub stage: String,
    pub file: Option<String>,
    pub line: Option<u32>,
    pub column: Option<u32>,
    pub message: String,
    pub severity: BuildDiagnosticSeverity,
}
impl BuildDiagnostic {
    /// Create an error diagnostic.
    #[allow(dead_code)]
    pub fn error(stage: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            stage: stage.into(),
            file: None,
            line: None,
            column: None,
            message: message.into(),
            severity: BuildDiagnosticSeverity::Error,
        }
    }
    /// Create a warning.
    #[allow(dead_code)]
    pub fn warning(stage: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            stage: stage.into(),
            file: None,
            line: None,
            column: None,
            message: message.into(),
            severity: BuildDiagnosticSeverity::Warning,
        }
    }
    /// Add a file location.
    #[allow(dead_code)]
    pub fn at_location(mut self, file: impl Into<String>, line: u32, col: u32) -> Self {
        self.file = Some(file.into());
        self.line = Some(line);
        self.column = Some(col);
        self
    }
    /// Format as a human-readable string.
    #[allow(dead_code)]
    pub fn format(&self) -> String {
        let sev = match self.severity {
            BuildDiagnosticSeverity::Error => "error",
            BuildDiagnosticSeverity::Warning => "warning",
            BuildDiagnosticSeverity::Note => "note",
        };
        if let (Some(file), Some(line)) = (&self.file, self.line) {
            format!(
                "[{}] {}:{}:{}: [{}] {}",
                self.stage,
                file,
                line,
                self.column.unwrap_or(0),
                sev,
                self.message
            )
        } else {
            format!("[{}] [{}] {}", self.stage, sev, self.message)
        }
    }
}
#[allow(dead_code)]
pub struct BuildProgress {
    pub total: usize,
    pub done: usize,
    pub failed: usize,
    pub start: std::time::Instant,
}
#[allow(dead_code)]
impl BuildProgress {
    pub fn new(total: usize) -> Self {
        Self {
            total,
            done: 0,
            failed: 0,
            start: std::time::Instant::now(),
        }
    }
    pub fn complete_one(&mut self) {
        self.done += 1;
    }
    pub fn fail_one(&mut self) {
        self.failed += 1;
    }
    pub fn pct(&self) -> f64 {
        if self.total == 0 {
            return 100.0;
        }
        (self.done + self.failed) as f64 / self.total as f64 * 100.0
    }
    pub fn elapsed_secs(&self) -> f64 {
        self.start.elapsed().as_secs_f64()
    }
    pub fn eta_secs(&self) -> f64 {
        let done = self.done + self.failed;
        if done == 0 {
            return f64::INFINITY;
        }
        let rate = done as f64 / self.elapsed_secs();
        (self.total - done) as f64 / rate
    }
    pub fn status_line(&self) -> String {
        format!(
            "[{}/{} ({:.0}%)] {} failed | {:.1}s elapsed | ETA: {:.0}s",
            self.done + self.failed,
            self.total,
            self.pct(),
            self.failed,
            self.elapsed_secs(),
            self.eta_secs().min(99999.0)
        )
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}
#[allow(dead_code)]
impl LogLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Debug => "debug",
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct BuildProfile {
    pub lex_ms: f64,
    pub parse_ms: f64,
    pub elab_ms: f64,
    pub typecheck_ms: f64,
    pub codegen_ms: f64,
}
#[allow(dead_code)]
impl BuildProfile {
    pub fn total_ms(&self) -> f64 {
        self.lex_ms + self.parse_ms + self.elab_ms + self.typecheck_ms + self.codegen_ms
    }
    pub fn dominant_stage(&self) -> &'static str {
        let stages = [
            ("lex", self.lex_ms),
            ("parse", self.parse_ms),
            ("elab", self.elab_ms),
            ("typecheck", self.typecheck_ms),
            ("codegen", self.codegen_ms),
        ];
        stages
            .iter()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(s, _)| *s)
            .unwrap_or("none")
    }
    pub fn add(&mut self, other: &BuildProfile) {
        self.lex_ms += other.lex_ms;
        self.parse_ms += other.parse_ms;
        self.elab_ms += other.elab_ms;
        self.typecheck_ms += other.typecheck_ms;
        self.codegen_ms += other.codegen_ms;
    }
}
