//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::manifest::{BuildProfile, OptLevel, Version};
use oxilean_parse::parse_source_file;
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Unique identifier for a build step.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct StepId(pub u64);
impl StepId {
    /// Create a new step ID.
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}
/// A single build step in the DAG.
#[derive(Clone, Debug)]
pub struct BuildStep {
    /// Unique step ID.
    pub id: StepId,
    /// Display name.
    pub name: String,
    /// Step kind.
    pub kind: StepKind,
    /// Module or target being built.
    pub target: String,
    /// Input files.
    pub inputs: Vec<PathBuf>,
    /// Output files.
    pub outputs: Vec<PathBuf>,
    /// Step IDs that must complete before this step.
    pub dependencies: Vec<StepId>,
    /// Estimated duration in milliseconds (for scheduling).
    pub estimated_ms: u64,
    /// Environment variables for this step.
    pub env: HashMap<String, String>,
}
impl BuildStep {
    /// Create a new build step.
    pub fn new(id: StepId, name: &str, kind: StepKind, target: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            kind,
            target: target.to_string(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            dependencies: Vec::new(),
            estimated_ms: 100,
            env: HashMap::new(),
        }
    }
    /// Add an input file.
    pub fn add_input(mut self, path: &Path) -> Self {
        self.inputs.push(path.to_path_buf());
        self
    }
    /// Add an output file.
    pub fn add_output(mut self, path: &Path) -> Self {
        self.outputs.push(path.to_path_buf());
        self
    }
    /// Add a dependency on another step.
    pub fn depends_on(mut self, step: StepId) -> Self {
        self.dependencies.push(step);
        self
    }
    /// Set the estimated duration.
    pub fn with_estimate(mut self, ms: u64) -> Self {
        self.estimated_ms = ms;
        self
    }
    /// Set an environment variable.
    pub fn set_env(mut self, key: &str, value: &str) -> Self {
        self.env.insert(key.to_string(), value.to_string());
        self
    }
}
/// Helper for constructing a build DAG from module information.
pub struct BuildPlanBuilder {
    dag: BuildDag,
    module_parse_steps: HashMap<String, StepId>,
    module_elab_steps: HashMap<String, StepId>,
    module_compile_steps: HashMap<String, StepId>,
}
impl BuildPlanBuilder {
    /// Create a new plan builder.
    pub fn new() -> Self {
        Self {
            dag: BuildDag::new(),
            module_parse_steps: HashMap::new(),
            module_elab_steps: HashMap::new(),
            module_compile_steps: HashMap::new(),
        }
    }
    /// Add a module to the build plan.
    pub fn add_module(&mut self, module_path: &str, source_path: &Path, dependencies: &[String]) {
        let parse_id = self.dag.next_step_id();
        let parse_step = BuildStep::new(
            parse_id.clone(),
            &format!("parse {}", module_path),
            StepKind::Parse,
            module_path,
        )
        .add_input(source_path);
        self.dag.add_step(parse_step);
        self.module_parse_steps
            .insert(module_path.to_string(), parse_id.clone());
        let elab_id = self.dag.next_step_id();
        let mut elab_step = BuildStep::new(
            elab_id.clone(),
            &format!("elaborate {}", module_path),
            StepKind::Elaborate,
            module_path,
        )
        .depends_on(parse_id);
        for dep in dependencies {
            if let Some(dep_elab_id) = self.module_elab_steps.get(dep) {
                elab_step = elab_step.depends_on(dep_elab_id.clone());
            }
        }
        self.dag.add_step(elab_step);
        self.module_elab_steps
            .insert(module_path.to_string(), elab_id.clone());
        let compile_id = self.dag.next_step_id();
        let compile_step = BuildStep::new(
            compile_id.clone(),
            &format!("compile {}", module_path),
            StepKind::Compile,
            module_path,
        )
        .depends_on(elab_id);
        self.dag.add_step(compile_step);
        self.module_compile_steps
            .insert(module_path.to_string(), compile_id);
    }
    /// Add a link step that depends on all compile steps.
    pub fn add_link_step(&mut self, output_name: &str) -> StepId {
        let link_id = self.dag.next_step_id();
        let mut link_step = BuildStep::new(
            link_id.clone(),
            &format!("link {}", output_name),
            StepKind::Link,
            output_name,
        );
        for compile_id in self.module_compile_steps.values() {
            link_step = link_step.depends_on(compile_id.clone());
        }
        self.dag.add_step(link_step);
        link_id
    }
    /// Add a documentation generation step.
    pub fn add_doc_step(&mut self, module_path: &str) -> StepId {
        let doc_id = self.dag.next_step_id();
        let mut doc_step = BuildStep::new(
            doc_id.clone(),
            &format!("document {}", module_path),
            StepKind::Document,
            module_path,
        );
        if let Some(elab_id) = self.module_elab_steps.get(module_path) {
            doc_step = doc_step.depends_on(elab_id.clone());
        }
        self.dag.add_step(doc_step);
        doc_id
    }
    /// Finalize and return the build DAG.
    pub fn build(self) -> BuildDag {
        self.dag
    }
}
/// Manages a pool of worker slots for the local executor.
pub struct ExecutorWorkerPool {
    pub max_workers: usize,
    active: std::collections::HashSet<u64>,
}
impl ExecutorWorkerPool {
    /// Create a pool.
    pub fn new(max_workers: usize) -> Self {
        Self {
            max_workers,
            active: std::collections::HashSet::new(),
        }
    }
    /// Whether we can accept more work.
    pub fn has_capacity(&self) -> bool {
        self.active.len() < self.max_workers
    }
    /// Assign a step to a worker slot.
    pub fn assign(&mut self, step_id: u64) -> bool {
        if self.has_capacity() {
            self.active.insert(step_id);
            true
        } else {
            false
        }
    }
    /// Release a worker slot.
    pub fn release(&mut self, step_id: u64) {
        self.active.remove(&step_id);
    }
    /// Number of active workers.
    pub fn active_count(&self) -> usize {
        self.active.len()
    }
    /// Utilization fraction.
    pub fn utilization(&self) -> f64 {
        if self.max_workers == 0 {
            0.0
        } else {
            self.active.len() as f64 / self.max_workers as f64
        }
    }
}
/// Final report from a build execution.
#[derive(Clone, Debug, Default)]
pub struct BuildExecutionReport {
    /// Steps that succeeded.
    pub succeeded: Vec<u64>,
    /// Steps that failed.
    pub failed: Vec<u64>,
    /// Steps that were skipped.
    pub skipped: Vec<u64>,
    /// Total wall-clock time in milliseconds.
    pub wall_ms: u64,
}
impl BuildExecutionReport {
    /// Create an empty report.
    pub fn new() -> Self {
        Self::default()
    }
    /// Whether the build succeeded (no failures).
    pub fn is_success(&self) -> bool {
        self.failed.is_empty()
    }
    /// Total steps tracked.
    pub fn total(&self) -> usize {
        self.succeeded.len() + self.failed.len() + self.skipped.len()
    }
    /// Success rate.
    pub fn success_rate(&self) -> f64 {
        let done = self.succeeded.len() + self.failed.len();
        if done == 0 {
            1.0
        } else {
            self.succeeded.len() as f64 / done as f64
        }
    }
}
/// Collects build outputs.
pub struct OutputCollector {
    /// All outputs.
    outputs: Vec<BuildOutput>,
}
impl OutputCollector {
    /// Create a new collector.
    pub fn new() -> Self {
        Self {
            outputs: Vec::new(),
        }
    }
    /// Add an output.
    pub fn add(&mut self, output: BuildOutput) {
        self.outputs.push(output);
    }
    /// Get all outputs.
    pub fn all(&self) -> &[BuildOutput] {
        &self.outputs
    }
    /// Get outputs by kind.
    pub fn by_kind(&self, kind: &OutputKind) -> Vec<&BuildOutput> {
        self.outputs.iter().filter(|o| &o.kind == kind).collect()
    }
    /// Get total output size.
    pub fn total_size(&self) -> u64 {
        self.outputs.iter().map(|o| o.size).sum()
    }
    /// Get the output count.
    pub fn count(&self) -> usize {
        self.outputs.len()
    }
}
/// Statistics collected by the build executor.
#[derive(Clone, Debug, Default)]
pub struct ExecutorStats {
    /// Total steps submitted.
    pub steps_submitted: u64,
    /// Steps completed successfully.
    pub steps_succeeded: u64,
    /// Steps that failed.
    pub steps_failed: u64,
    /// Steps skipped (cached).
    pub steps_skipped: u64,
    /// Total CPU time consumed in milliseconds.
    pub cpu_ms: u64,
    /// Total wall-clock time in milliseconds.
    pub wall_ms: u64,
    /// Peak concurrent workers.
    pub peak_concurrency: u64,
}
impl ExecutorStats {
    /// Create zeroed stats.
    pub fn new() -> Self {
        Self::default()
    }
    /// Step success rate.
    pub fn success_rate(&self) -> f64 {
        let done = self.steps_succeeded + self.steps_failed;
        if done == 0 {
            1.0
        } else {
            self.steps_succeeded as f64 / done as f64
        }
    }
    /// CPU efficiency (cpu_ms / (wall_ms * peak_concurrency)).
    pub fn cpu_efficiency(&self) -> f64 {
        let denom = self.wall_ms * self.peak_concurrency.max(1);
        if denom == 0 {
            0.0
        } else {
            self.cpu_ms as f64 / denom as f64
        }
    }
    /// Summary string.
    pub fn summary(&self) -> String {
        format!(
            "submitted={} succeeded={} failed={} skipped={} wall={}ms",
            self.steps_submitted,
            self.steps_succeeded,
            self.steps_failed,
            self.steps_skipped,
            self.wall_ms,
        )
    }
}
/// Represents a build output (artifact path + metadata).
#[derive(Clone, Debug)]
pub struct BuildOutput {
    /// Output file path.
    pub path: PathBuf,
    /// The step that produced this output.
    pub produced_by: StepId,
    /// Output kind.
    pub kind: OutputKind,
    /// Size in bytes.
    pub size: u64,
}
impl BuildOutput {
    /// Create a new build output.
    pub fn new(path: &Path, produced_by: StepId, kind: OutputKind) -> Self {
        Self {
            path: path.to_path_buf(),
            produced_by,
            kind,
            size: 0,
        }
    }
    /// Set the size.
    pub fn with_size(mut self, size: u64) -> Self {
        self.size = size;
        self
    }
}
/// Error type for build execution.
#[derive(Clone, Debug)]
pub enum ExecutorError {
    /// Cyclic dependency in the build DAG.
    CyclicDependency,
    /// Empty build DAG.
    EmptyDag,
    /// A build step failed.
    StepFailed {
        /// Step that failed.
        step_id: StepId,
        /// Error message.
        error: String,
    },
    /// Multiple steps failed.
    MultipleFailures(Vec<(StepId, String)>),
    /// Invalid configuration.
    InvalidConfig(String),
    /// IO error.
    IoError(String),
    /// Cancelled by user.
    Cancelled,
}
/// Run-time configuration for the build executor.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ExecutorRunConfig {
    /// Maximum parallel workers.
    pub max_workers: usize,
    /// Sandbox isolation config.
    pub sandbox: SandboxConfig,
    /// Whether to stop on first failure.
    pub fail_fast: bool,
    /// Whether to print verbose step output.
    pub verbose: bool,
}
#[allow(dead_code)]
impl ExecutorRunConfig {
    /// Create a config for CI (fail-fast, verbose).
    pub fn ci() -> Self {
        Self {
            fail_fast: true,
            verbose: true,
            ..Self::default()
        }
    }
    /// Enable fail-fast mode.
    pub fn with_fail_fast(mut self) -> Self {
        self.fail_fast = true;
        self
    }
    /// Set max workers.
    pub fn with_workers(mut self, n: usize) -> Self {
        self.max_workers = n.max(1);
        self
    }
}
/// A stub handle to a spawned build process.
#[derive(Debug)]
pub struct ProcessHandle {
    /// The step ID this process is running.
    pub step_id: u64,
    /// Whether the process has completed.
    pub finished: bool,
    /// Exit code (0 = success).
    pub exit_code: i32,
}
impl ProcessHandle {
    /// Create a running handle.
    pub fn spawned(step_id: u64) -> Self {
        Self {
            step_id,
            finished: false,
            exit_code: 0,
        }
    }
    /// Simulate completion with the given exit code.
    pub fn complete(&mut self, exit_code: i32) {
        self.finished = true;
        self.exit_code = exit_code;
    }
    /// Whether the process succeeded.
    pub fn succeeded(&self) -> bool {
        self.finished && self.exit_code == 0
    }
}
/// Configuration for the build executor.
#[derive(Clone, Debug)]
pub struct ExecutorConfig {
    /// Number of parallel jobs.
    pub parallelism: usize,
    /// Whether to stop on first error.
    pub fail_fast: bool,
    /// Build profile to use.
    pub profile: BuildProfile,
    /// Output directory.
    pub output_dir: PathBuf,
    /// Whether to display progress.
    pub show_progress: bool,
    /// Whether to display verbose output.
    pub verbose: bool,
    /// Maximum time per step (None = unlimited).
    pub step_timeout: Option<Duration>,
    /// Package version being built.
    pub package_version: Version,
}
/// The directed acyclic graph of build steps.
#[derive(Clone, Debug)]
pub struct BuildDag {
    /// All steps indexed by ID.
    steps: BTreeMap<StepId, BuildStep>,
    /// Forward edges: step -> steps that depend on it.
    dependents: HashMap<StepId, Vec<StepId>>,
    /// Next step ID to assign.
    next_id: u64,
}
impl BuildDag {
    /// Create a new empty DAG.
    pub fn new() -> Self {
        Self {
            steps: BTreeMap::new(),
            dependents: HashMap::new(),
            next_id: 0,
        }
    }
    /// Allocate a new step ID.
    pub fn next_step_id(&mut self) -> StepId {
        let id = StepId::new(self.next_id);
        self.next_id += 1;
        id
    }
    /// Add a step to the DAG.
    pub fn add_step(&mut self, step: BuildStep) {
        let step_id = step.id.clone();
        for dep in &step.dependencies {
            self.dependents
                .entry(dep.clone())
                .or_default()
                .push(step_id.clone());
        }
        self.steps.insert(step_id, step);
    }
    /// Get a step by ID.
    pub fn get_step(&self, id: &StepId) -> Option<&BuildStep> {
        self.steps.get(id)
    }
    /// Get all steps.
    pub fn all_steps(&self) -> Vec<&BuildStep> {
        self.steps.values().collect()
    }
    /// Get the number of steps.
    pub fn step_count(&self) -> usize {
        self.steps.len()
    }
    /// Get steps that have no dependencies (roots of the DAG).
    pub fn root_steps(&self) -> Vec<StepId> {
        self.steps
            .values()
            .filter(|s| s.dependencies.is_empty())
            .map(|s| s.id.clone())
            .collect()
    }
    /// Get steps that depend on a given step.
    pub fn dependents_of(&self, id: &StepId) -> Vec<StepId> {
        self.dependents.get(id).cloned().unwrap_or_default()
    }
    /// Compute topological order of all steps.
    pub fn topological_order(&self) -> Result<Vec<StepId>, ExecutorError> {
        let mut in_degree: HashMap<StepId, usize> = HashMap::new();
        for (id, step) in &self.steps {
            in_degree.entry(id.clone()).or_insert(0);
            for _dep in &step.dependencies {}
            in_degree.insert(id.clone(), step.dependencies.len());
        }
        let mut queue: VecDeque<StepId> = VecDeque::new();
        for (id, deg) in &in_degree {
            if *deg == 0 {
                queue.push_back(id.clone());
            }
        }
        let mut result = Vec::new();
        while let Some(id) = queue.pop_front() {
            result.push(id.clone());
            for dep_id in self.dependents_of(&id) {
                if let Some(deg) = in_degree.get_mut(&dep_id) {
                    *deg -= 1;
                    if *deg == 0 {
                        queue.push_back(dep_id);
                    }
                }
            }
        }
        if result.len() != self.steps.len() {
            return Err(ExecutorError::CyclicDependency);
        }
        Ok(result)
    }
    /// Compute the critical path (longest chain).
    pub fn critical_path(&self) -> Result<(Vec<StepId>, u64), ExecutorError> {
        let topo = self.topological_order()?;
        let mut longest_to: HashMap<StepId, u64> = HashMap::new();
        let mut predecessor: HashMap<StepId, Option<StepId>> = HashMap::new();
        for id in &topo {
            let step = &self.steps[id];
            let mut max_prev = 0u64;
            let mut best_pred = None;
            for dep in &step.dependencies {
                let dep_time = longest_to.get(dep).copied().unwrap_or(0);
                if dep_time > max_prev {
                    max_prev = dep_time;
                    best_pred = Some(dep.clone());
                }
            }
            longest_to.insert(id.clone(), max_prev + step.estimated_ms);
            predecessor.insert(id.clone(), best_pred);
        }
        let last = topo
            .iter()
            .max_by_key(|id| longest_to.get(*id).copied().unwrap_or(0))
            .cloned()
            .ok_or(ExecutorError::EmptyDag)?;
        let total = longest_to[&last];
        let mut path = Vec::new();
        let mut current = Some(last);
        while let Some(id) = current {
            path.push(id.clone());
            current = predecessor.get(&id).and_then(|p| p.clone());
        }
        path.reverse();
        Ok((path, total))
    }
    /// Estimate total build time with given parallelism.
    pub fn estimate_total_time(&self, parallelism: usize) -> Result<u64, ExecutorError> {
        if parallelism == 0 {
            return Err(ExecutorError::InvalidConfig(
                "parallelism must be > 0".to_string(),
            ));
        }
        let total_work: u64 = self.steps.values().map(|s| s.estimated_ms).sum();
        let (_path, critical_time) = self.critical_path()?;
        Ok(critical_time.max(total_work / parallelism as u64))
    }
}
/// Progress state for the entire build.
#[derive(Clone, Debug)]
pub struct BuildProgress {
    /// Total number of steps.
    pub total_steps: usize,
    /// Number of completed steps.
    pub completed_steps: usize,
    /// Number of failed steps.
    pub failed_steps: usize,
    /// Number of steps currently running.
    pub running_steps: usize,
    /// Number of steps waiting to run.
    pub pending_steps: usize,
    /// Steps currently in progress.
    pub current_steps: Vec<StepId>,
    /// Start time of the build.
    pub start_time: Option<Instant>,
    /// Elapsed time.
    pub elapsed: Duration,
    /// Estimated remaining time.
    pub estimated_remaining: Duration,
}
impl BuildProgress {
    /// Create a new progress tracker.
    pub fn new(total_steps: usize) -> Self {
        Self {
            total_steps,
            completed_steps: 0,
            failed_steps: 0,
            running_steps: 0,
            pending_steps: total_steps,
            current_steps: Vec::new(),
            start_time: None,
            elapsed: Duration::ZERO,
            estimated_remaining: Duration::ZERO,
        }
    }
    /// Start tracking.
    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }
    /// Update elapsed time.
    pub fn update_elapsed(&mut self) {
        if let Some(start) = self.start_time {
            self.elapsed = start.elapsed();
            if self.completed_steps > 0 {
                let per_step = self.elapsed.as_millis() / self.completed_steps as u128;
                let remaining = self.pending_steps as u128 * per_step;
                self.estimated_remaining = Duration::from_millis(remaining as u64);
            }
        }
    }
    /// Get progress as a fraction (0.0 to 1.0).
    pub fn fraction(&self) -> f64 {
        if self.total_steps == 0 {
            1.0
        } else {
            self.completed_steps as f64 / self.total_steps as f64
        }
    }
    /// Get progress as a percentage (0 to 100).
    pub fn percentage(&self) -> u32 {
        (self.fraction() * 100.0) as u32
    }
    /// Check if the build is complete.
    pub fn is_complete(&self) -> bool {
        self.completed_steps + self.failed_steps >= self.total_steps
    }
    /// Format a progress bar string.
    pub fn progress_bar(&self, width: usize) -> String {
        let filled = (self.fraction() * width as f64) as usize;
        let empty = width.saturating_sub(filled);
        format!(
            "[{}{}] {}/{} ({}%)",
            "#".repeat(filled),
            ".".repeat(empty),
            self.completed_steps,
            self.total_steps,
            self.percentage()
        )
    }
}
/// A log entry from the build process.
#[derive(Clone, Debug)]
pub struct BuildLogEntry {
    /// Step that produced this entry.
    pub step_id: Option<StepId>,
    /// Log level.
    pub level: LogLevel,
    /// Message.
    pub message: String,
    /// Timestamp (relative to build start).
    pub timestamp: Duration,
}
/// Collects build log entries.
pub struct BuildLog {
    /// All entries.
    entries: Vec<BuildLogEntry>,
    /// Minimum level to record.
    min_level: LogLevel,
}
impl BuildLog {
    /// Create a new build log.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            min_level: LogLevel::Info,
        }
    }
    /// Set the minimum log level.
    pub fn set_min_level(&mut self, level: LogLevel) {
        self.min_level = level;
    }
    fn level_ordinal(level: &LogLevel) -> u8 {
        match level {
            LogLevel::Debug => 0,
            LogLevel::Info => 1,
            LogLevel::Warning => 2,
            LogLevel::Error => 3,
        }
    }
    /// Add a log entry.
    pub fn log(
        &mut self,
        step_id: Option<StepId>,
        level: LogLevel,
        message: &str,
        timestamp: Duration,
    ) {
        if Self::level_ordinal(&level) >= Self::level_ordinal(&self.min_level) {
            self.entries.push(BuildLogEntry {
                step_id,
                level,
                message: message.to_string(),
                timestamp,
            });
        }
    }
    /// Get all entries.
    pub fn entries(&self) -> &[BuildLogEntry] {
        &self.entries
    }
    /// Get entries for a specific step.
    pub fn entries_for_step(&self, step_id: &StepId) -> Vec<&BuildLogEntry> {
        self.entries
            .iter()
            .filter(|e| e.step_id.as_ref() == Some(step_id))
            .collect()
    }
    /// Get all warnings.
    pub fn warnings(&self) -> Vec<&BuildLogEntry> {
        self.entries
            .iter()
            .filter(|e| e.level == LogLevel::Warning)
            .collect()
    }
    /// Get all errors.
    pub fn errors(&self) -> Vec<&BuildLogEntry> {
        self.entries
            .iter()
            .filter(|e| e.level == LogLevel::Error)
            .collect()
    }
    /// Get the total number of entries.
    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }
    /// Clear all entries.
    pub fn clear(&mut self) {
        self.entries.clear();
    }
}
/// The build executor that runs the DAG.
pub struct BuildExecutor {
    /// The build DAG.
    dag: BuildDag,
    /// Configuration.
    config: ExecutorConfig,
    /// Results of completed steps.
    results: Arc<Mutex<Vec<StepResult>>>,
    /// Progress tracker.
    progress: Arc<Mutex<BuildProgress>>,
    /// Whether the build was cancelled.
    cancelled: Arc<Mutex<bool>>,
}
impl BuildExecutor {
    /// Create a new executor.
    pub fn new(dag: BuildDag, config: ExecutorConfig) -> Self {
        let total = dag.step_count();
        Self {
            dag,
            config,
            results: Arc::new(Mutex::new(Vec::new())),
            progress: Arc::new(Mutex::new(BuildProgress::new(total))),
            cancelled: Arc::new(Mutex::new(false)),
        }
    }
    /// Execute the build (single-threaded simulation).
    pub fn execute(&mut self) -> Result<BuildReport, ExecutorError> {
        let topo = self.dag.topological_order()?;
        let start = Instant::now();
        {
            let mut progress = self.progress.lock().expect("mutex should not be poisoned");
            progress.start();
        }
        let mut completed: HashSet<StepId> = HashSet::new();
        let mut failures: Vec<(StepId, String)> = Vec::new();
        for step_id in &topo {
            if *self.cancelled.lock().expect("mutex should not be poisoned") {
                return Err(ExecutorError::Cancelled);
            }
            let step = self.dag.get_step(step_id).ok_or(ExecutorError::EmptyDag)?;
            let deps_satisfied = step.dependencies.iter().all(|d| completed.contains(d));
            if !deps_satisfied {
                if self.config.fail_fast {
                    break;
                }
                continue;
            }
            {
                let mut progress = self.progress.lock().expect("mutex should not be poisoned");
                progress.running_steps += 1;
                progress.pending_steps = progress.pending_steps.saturating_sub(1);
                progress.current_steps.push(step_id.clone());
            }
            let step_start = Instant::now();
            let result = self.execute_step(step);
            let duration = step_start.elapsed();
            let step_result = match result {
                Ok(()) => StepResult::success(step_id.clone(), duration),
                Err(msg) => StepResult::failure(step_id.clone(), duration, &msg),
            };
            let success = step_result.success;
            {
                let mut results = self.results.lock().expect("mutex should not be poisoned");
                results.push(step_result);
            }
            {
                let mut progress = self.progress.lock().expect("mutex should not be poisoned");
                progress.running_steps = progress.running_steps.saturating_sub(1);
                progress.current_steps.retain(|id| id != step_id);
                if success {
                    progress.completed_steps += 1;
                } else {
                    progress.failed_steps += 1;
                }
                progress.update_elapsed();
            }
            if success {
                completed.insert(step_id.clone());
            } else {
                failures.push((step_id.clone(), "step failed".to_string()));
                if self.config.fail_fast {
                    break;
                }
            }
        }
        let total_duration = start.elapsed();
        let results = self
            .results
            .lock()
            .expect("mutex should not be poisoned")
            .clone();
        let report = BuildReport {
            total_steps: self.dag.step_count(),
            completed_steps: completed.len(),
            failed_steps: failures.len(),
            skipped_steps: self.dag.step_count() - completed.len() - failures.len(),
            total_duration,
            step_results: results,
            success: failures.is_empty(),
            profile_name: self.config.profile.name.clone(),
            opt_level: self.config.profile.opt_level.clone(),
        };
        if !failures.is_empty() && self.config.fail_fast {}
        Ok(report)
    }
    fn execute_step(&self, step: &BuildStep) -> Result<(), String> {
        match &step.kind {
            StepKind::Parse => self.execute_parse(step),
            StepKind::Elaborate => self.execute_elaborate(step),
            StepKind::Compile => self.execute_compile(step),
            StepKind::Link => self.execute_link(step),
            StepKind::Document => self.execute_document(step),
            StepKind::Test => self.execute_test(step),
            StepKind::Script(name) => self.execute_script(name, step),
            StepKind::CopyArtifact => self.execute_copy_artifact(step),
        }
    }
    /// Parse all input source files using the OxiLean lexer and parser.
    ///
    /// Returns `Ok(())` even when some input files do not exist on disk
    /// (the file set may be constructed before the source tree is fully
    /// materialised).  Parse errors *within* an existing file are collected
    /// and, in verbose mode, printed to stderr; they do not fail the step so
    /// that the executor can continue building unrelated modules.
    fn execute_parse(&self, step: &BuildStep) -> Result<(), String> {
        if step.inputs.is_empty() {
            return Err("no input files for parse step".to_string());
        }
        for input in &step.inputs {
            let source = match fs::read_to_string(input) {
                Ok(s) => s,
                Err(_) => continue,
            };
            let mut errors: Vec<String> = Vec::new();
            let _count = parse_source_file(&source, Some(&mut errors)).unwrap_or(0);
            if self.config.verbose {
                for e in &errors {
                    eprintln!("[parse] {}: {}", input.display(), e);
                }
            }
        }
        Ok(())
    }
    /// Elaborate (type-check) a module.
    ///
    /// Verifies that each input file either exists on disk or was produced by
    /// an earlier build step.  Full type-checking is performed by the
    /// `oxilean-elab` pipeline; this step validates that inputs are present
    /// and syntactically well-formed before handing off to the elaborator.
    fn execute_elaborate(&self, step: &BuildStep) -> Result<(), String> {
        for input in &step.inputs {
            if !input.exists() {
                continue;
            }
            let source = fs::read_to_string(input)
                .map_err(|e| format!("cannot read {}: {}", input.display(), e))?;
            let mut errors: Vec<String> = Vec::new();
            let _count = parse_source_file(&source, Some(&mut errors)).unwrap_or(0);
            if !errors.is_empty() {
                for e in &errors {
                    eprintln!("[elaborate] syntax error in {}: {}", input.display(), e);
                }
                return Err(format!(
                    "{} syntax error(s) in {}",
                    errors.len(),
                    input.display()
                ));
            } else if self.config.verbose {
                eprintln!("[elaborate] ok: {}", input.display());
            }
        }
        Ok(())
    }
    /// Compile a module to an intermediate representation.
    ///
    /// Checks that the elaborated source is readable, then creates
    /// placeholder artifact files in the configured output directory so that
    /// downstream link and copy steps can depend on them.
    fn execute_compile(&self, step: &BuildStep) -> Result<(), String> {
        for input in &step.inputs {
            if input.exists() {
                fs::read(input)
                    .map_err(|e| format!("cannot read input {}: {}", input.display(), e))?;
            }
        }
        for output in &step.outputs {
            if let Some(parent) = output.parent() {
                if !parent.as_os_str().is_empty() && !parent.exists() {
                    fs::create_dir_all(parent).map_err(|e| {
                        format!("cannot create output dir {}: {}", parent.display(), e)
                    })?;
                }
            }
            fs::write(
                output,
                format!(
                    "# compiled artifact: {}
",
                    step.target
                ),
            )
            .map_err(|e| format!("cannot write artifact {}: {}", output.display(), e))?;
        }
        Ok(())
    }
    /// Link compiled modules into a final binary or library artifact.
    ///
    /// Verifies all compiled inputs are present and writes the linked
    /// output artifact.
    fn execute_link(&self, step: &BuildStep) -> Result<(), String> {
        for input in &step.inputs {
            if input.exists() {
                fs::read(input)
                    .map_err(|e| format!("cannot read linked input {}: {}", input.display(), e))?;
            }
        }
        for output in &step.outputs {
            if let Some(parent) = output.parent() {
                if !parent.as_os_str().is_empty() && !parent.exists() {
                    fs::create_dir_all(parent).map_err(|e| {
                        format!("cannot create output dir {}: {}", parent.display(), e)
                    })?;
                }
            }
            fs::write(
                output,
                format!(
                    "# linked artifact: {}
",
                    step.target
                ),
            )
            .map_err(|e| format!("cannot write linked artifact {}: {}", output.display(), e))?;
        }
        Ok(())
    }
    /// Generate documentation for a module.
    ///
    /// Reads each source file, collects doc-comment lines, and writes a
    /// Markdown summary to each configured output path.
    fn execute_document(&self, step: &BuildStep) -> Result<(), String> {
        let mut doc_lines: Vec<String> = Vec::new();
        doc_lines.push(format!("# Module: {}", step.target));
        doc_lines.push(String::new());
        let mut total_decls = 0usize;
        for input in &step.inputs {
            if !input.exists() {
                continue;
            }
            let source = fs::read_to_string(input)
                .map_err(|e| format!("cannot read {}: {}", input.display(), e))?;
            let mut errors: Vec<String> = Vec::new();
            let decl_count = parse_source_file(&source, Some(&mut errors)).unwrap_or(0);
            total_decls += decl_count;
            let mut in_block_comment = false;
            for line in source.lines() {
                let trimmed = line.trim();
                if in_block_comment {
                    if trimmed.contains("-/") {
                        in_block_comment = false;
                        let content = trimmed.split("-/").next().unwrap_or("").trim();
                        if !content.is_empty() {
                            doc_lines.push(content.to_string());
                        }
                    } else {
                        doc_lines.push(trimmed.to_string());
                    }
                } else if trimmed.starts_with("/-!") {
                    in_block_comment = true;
                    let content = trimmed.trim_start_matches("/-!").trim();
                    if !content.is_empty() {
                        doc_lines.push(content.to_string());
                    }
                } else if trimmed.starts_with("--!") {
                    let content = trimmed.trim_start_matches("--!").trim();
                    doc_lines.push(content.to_string());
                }
            }
        }
        if total_decls > 0 {
            doc_lines.push(String::new());
            doc_lines.push("---".to_string());
            doc_lines.push(format!("*{} declarations documented.*", total_decls));
        }
        for output in &step.outputs {
            if let Some(parent) = output.parent() {
                if !parent.as_os_str().is_empty() && !parent.exists() {
                    fs::create_dir_all(parent).map_err(|e| {
                        format!("cannot create doc dir {}: {}", parent.display(), e)
                    })?;
                }
            }
            fs::write(output, doc_lines.join("\n"))
                .map_err(|e| format!("cannot write doc {}: {}", output.display(), e))?;
            if self.config.verbose {
                eprintln!("[document] wrote {} to {}", step.target, output.display());
            }
        }
        Ok(())
    }
    /// Run tests defined in source files.
    ///
    /// Reads each input file, counts theorem/def declarations (as a proxy for
    /// testable items), and returns success when parsing succeeds.  In verbose
    /// mode the declaration count is printed to stderr.
    fn execute_test(&self, step: &BuildStep) -> Result<(), String> {
        let mut total_decls = 0usize;
        for input in &step.inputs {
            if !input.exists() {
                continue;
            }
            let source = fs::read_to_string(input)
                .map_err(|e| format!("cannot read test file {}: {}", input.display(), e))?;
            let mut errors: Vec<String> = Vec::new();
            let count = parse_source_file(&source, Some(&mut errors)).unwrap_or(0);
            total_decls += count;
            if !errors.is_empty() {
                for e in &errors {
                    eprintln!("[test] parse error in {}: {}", input.display(), e);
                }
                return Err(format!(
                    "{} parse error(s) in test file {}",
                    errors.len(),
                    input.display()
                ));
            } else if self.config.verbose {
                eprintln!("[test] {} declarations in {}", count, input.display());
            }
        }
        if self.config.verbose && total_decls > 0 {
            eprintln!(
                "[test] {} total declarations for target {}",
                total_decls, step.target
            );
        }
        Ok(())
    }
    /// Execute a named custom build script step.
    fn execute_script(&self, name: &str, step: &BuildStep) -> Result<(), String> {
        if self.config.verbose {
            eprintln!(
                "[script] executing script '{}' for target {}",
                name, step.target
            );
        }
        for input in &step.inputs {
            if input.exists() {
                fs::metadata(input).map_err(|e| {
                    format!("cannot access script input {}: {}", input.display(), e)
                })?;
            }
        }
        Ok(())
    }
    /// Copy artifact files to their destination paths.
    fn execute_copy_artifact(&self, step: &BuildStep) -> Result<(), String> {
        if step.inputs.len() != step.outputs.len() && !step.outputs.is_empty() {
            for src in &step.inputs {
                if !src.exists() {
                    continue;
                }
                for dst in &step.outputs {
                    if let Some(parent) = dst.parent() {
                        if !parent.as_os_str().is_empty() && !parent.exists() {
                            fs::create_dir_all(parent).map_err(|e| {
                                format!("cannot create dir {}: {}", parent.display(), e)
                            })?;
                        }
                    }
                    fs::copy(src, dst).map_err(|e| {
                        format!("cannot copy {} -> {}: {}", src.display(), dst.display(), e)
                    })?;
                }
            }
        } else {
            for (src, dst) in step.inputs.iter().zip(step.outputs.iter()) {
                if !src.exists() {
                    continue;
                }
                if let Some(parent) = dst.parent() {
                    if !parent.as_os_str().is_empty() && !parent.exists() {
                        fs::create_dir_all(parent).map_err(|e| {
                            format!("cannot create dir {}: {}", parent.display(), e)
                        })?;
                    }
                }
                fs::copy(src, dst).map_err(|e| {
                    format!("cannot copy {} -> {}: {}", src.display(), dst.display(), e)
                })?;
            }
        }
        Ok(())
    }
    /// Cancel the build.
    pub fn cancel(&self) {
        *self.cancelled.lock().expect("mutex should not be poisoned") = true;
    }
    /// Get current progress.
    pub fn current_progress(&self) -> BuildProgress {
        self.progress
            .lock()
            .expect("mutex should not be poisoned")
            .clone()
    }
}
/// State of a step in the scheduler.
#[derive(Clone, Debug, PartialEq, Eq)]
enum SchedulerState {
    /// Waiting for dependencies.
    Blocked,
    /// Ready to execute.
    Ready,
    /// Currently running.
    Running,
    /// Completed.
    Done,
    /// Failed.
    Failed,
}
/// The kind of build output.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OutputKind {
    /// A compiled object file.
    CompiledObject,
    /// A linked library or executable.
    LinkedBinary,
    /// A documentation file.
    Documentation,
    /// A test result file.
    TestResult,
    /// A generated source file.
    GeneratedSource,
}
/// Tracks timing information for build steps.
pub struct BuildTimer {
    /// Start times for steps.
    start_times: HashMap<StepId, Instant>,
    /// Durations for completed steps.
    durations: HashMap<StepId, Duration>,
    /// Overall build start time.
    build_start: Option<Instant>,
}
impl BuildTimer {
    /// Create a new timer.
    pub fn new() -> Self {
        Self {
            start_times: HashMap::new(),
            durations: HashMap::new(),
            build_start: None,
        }
    }
    /// Start the overall build timer.
    pub fn start_build(&mut self) {
        self.build_start = Some(Instant::now());
    }
    /// Start timing a step.
    pub fn start_step(&mut self, step_id: StepId) {
        self.start_times.insert(step_id, Instant::now());
    }
    /// End timing a step.
    pub fn end_step(&mut self, step_id: &StepId) -> Duration {
        if let Some(start) = self.start_times.remove(step_id) {
            let duration = start.elapsed();
            self.durations.insert(step_id.clone(), duration);
            duration
        } else {
            Duration::ZERO
        }
    }
    /// Get the duration of a completed step.
    pub fn step_duration(&self, step_id: &StepId) -> Option<Duration> {
        self.durations.get(step_id).copied()
    }
    /// Get the total elapsed build time.
    pub fn elapsed(&self) -> Duration {
        self.build_start
            .map(|s| s.elapsed())
            .unwrap_or(Duration::ZERO)
    }
    /// Get the total step time (sum of all step durations).
    pub fn total_step_time(&self) -> Duration {
        self.durations.values().sum()
    }
    /// Get the slowest step.
    pub fn slowest_step(&self) -> Option<(&StepId, Duration)> {
        self.durations
            .iter()
            .max_by_key(|(_, d)| *d)
            .map(|(id, d)| (id, *d))
    }
    /// Get the number of timed steps.
    pub fn timed_step_count(&self) -> usize {
        self.durations.len()
    }
}
/// A parallel build scheduler.
pub struct ParallelScheduler {
    /// States of each step.
    states: HashMap<StepId, SchedulerState>,
    /// The build DAG.
    dag: BuildDag,
    /// Maximum parallelism.
    max_parallel: usize,
    /// Currently running steps.
    running: HashSet<StepId>,
}
impl ParallelScheduler {
    /// Create a new scheduler.
    pub fn new(dag: BuildDag, max_parallel: usize) -> Self {
        let mut states = HashMap::new();
        let roots = dag.root_steps();
        for step in dag.all_steps() {
            if roots.contains(&step.id) {
                states.insert(step.id.clone(), SchedulerState::Ready);
            } else {
                states.insert(step.id.clone(), SchedulerState::Blocked);
            }
        }
        Self {
            states,
            dag,
            max_parallel: max_parallel.max(1),
            running: HashSet::new(),
        }
    }
    /// Get the next batch of steps to execute.
    pub fn next_batch(&self) -> Vec<StepId> {
        let available_slots = self.max_parallel.saturating_sub(self.running.len());
        self.states
            .iter()
            .filter(|(_, state)| **state == SchedulerState::Ready)
            .map(|(id, _)| id.clone())
            .take(available_slots)
            .collect()
    }
    /// Mark a step as started.
    pub fn start_step(&mut self, id: &StepId) {
        self.states.insert(id.clone(), SchedulerState::Running);
        self.running.insert(id.clone());
    }
    /// Mark a step as completed and unblock dependents.
    pub fn complete_step(&mut self, id: &StepId) {
        self.states.insert(id.clone(), SchedulerState::Done);
        self.running.remove(id);
        for dep_id in self.dag.dependents_of(id) {
            if let Some(step) = self.dag.get_step(&dep_id) {
                let all_deps_done = step
                    .dependencies
                    .iter()
                    .all(|d| self.states.get(d) == Some(&SchedulerState::Done));
                if all_deps_done {
                    self.states.insert(dep_id, SchedulerState::Ready);
                }
            }
        }
    }
    /// Mark a step as failed.
    pub fn fail_step(&mut self, id: &StepId) {
        self.states.insert(id.clone(), SchedulerState::Failed);
        self.running.remove(id);
    }
    /// Check if all steps are complete.
    pub fn is_complete(&self) -> bool {
        self.states
            .values()
            .all(|s| *s == SchedulerState::Done || *s == SchedulerState::Failed)
    }
    /// Get the count of steps in each state.
    pub fn state_counts(&self) -> HashMap<&'static str, usize> {
        let mut counts = HashMap::new();
        for state in self.states.values() {
            let key = match state {
                SchedulerState::Blocked => "blocked",
                SchedulerState::Ready => "ready",
                SchedulerState::Running => "running",
                SchedulerState::Done => "done",
                SchedulerState::Failed => "failed",
            };
            *counts.entry(key).or_insert(0) += 1;
        }
        counts
    }
}
/// Priority queue for build steps (highest priority first).
pub struct StepPriorityQueue {
    /// (priority, step_id) pairs.
    heap: std::collections::BinaryHeap<(u32, u64)>,
}
impl StepPriorityQueue {
    /// Create an empty queue.
    pub fn new() -> Self {
        Self {
            heap: std::collections::BinaryHeap::new(),
        }
    }
    /// Enqueue a step.
    pub fn push(&mut self, step_id: u64, priority: u32) {
        self.heap.push((priority, step_id));
    }
    /// Dequeue the highest-priority step.
    pub fn pop(&mut self) -> Option<u64> {
        self.heap.pop().map(|(_, id)| id)
    }
    /// Number of queued steps.
    pub fn len(&self) -> usize {
        self.heap.len()
    }
    /// Whether empty.
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }
}
/// The result of executing a build step.
#[derive(Clone, Debug)]
pub struct StepResult {
    /// The step that was executed.
    pub step_id: StepId,
    /// Whether the step succeeded.
    pub success: bool,
    /// Duration of execution.
    pub duration: Duration,
    /// Standard output.
    pub stdout: String,
    /// Standard error.
    pub stderr: String,
    /// Exit code (if applicable).
    pub exit_code: Option<i32>,
    /// Warnings produced.
    pub warnings: Vec<String>,
    /// Errors produced.
    pub errors: Vec<String>,
}
impl StepResult {
    /// Create a successful result.
    pub fn success(step_id: StepId, duration: Duration) -> Self {
        Self {
            step_id,
            success: true,
            duration,
            stdout: String::new(),
            stderr: String::new(),
            exit_code: Some(0),
            warnings: Vec::new(),
            errors: Vec::new(),
        }
    }
    /// Create a failed result.
    pub fn failure(step_id: StepId, duration: Duration, error: &str) -> Self {
        Self {
            step_id,
            success: false,
            duration,
            stdout: String::new(),
            stderr: error.to_string(),
            exit_code: Some(1),
            warnings: Vec::new(),
            errors: vec![error.to_string()],
        }
    }
}
/// The kind of build step.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StepKind {
    /// Parse a source file.
    Parse,
    /// Elaborate (type-check) a module.
    Elaborate,
    /// Compile a module to an intermediate representation.
    Compile,
    /// Link compiled modules.
    Link,
    /// Generate documentation.
    Document,
    /// Run tests.
    Test,
    /// Execute a custom build script.
    Script(String),
    /// Copy artifact to output directory.
    CopyArtifact,
}
/// The final report after a build.
#[derive(Clone, Debug)]
pub struct BuildReport {
    /// Total number of steps.
    pub total_steps: usize,
    /// Number of completed steps.
    pub completed_steps: usize,
    /// Number of failed steps.
    pub failed_steps: usize,
    /// Number of skipped steps.
    pub skipped_steps: usize,
    /// Total build duration.
    pub total_duration: Duration,
    /// Results for each step.
    pub step_results: Vec<StepResult>,
    /// Whether the build succeeded.
    pub success: bool,
    /// Profile used.
    pub profile_name: String,
    /// Optimization level used.
    pub opt_level: OptLevel,
}
impl BuildReport {
    /// Format a summary of the build.
    pub fn summary(&self) -> String {
        let status = if self.success { "SUCCEEDED" } else { "FAILED" };
        format!(
            "Build {} in {:.2}s ({} steps: {} completed, {} failed, {} skipped) [profile: {}]",
            status,
            self.total_duration.as_secs_f64(),
            self.total_steps,
            self.completed_steps,
            self.failed_steps,
            self.skipped_steps,
            self.profile_name,
        )
    }
    /// Get all errors from the build.
    pub fn all_errors(&self) -> Vec<&str> {
        self.step_results
            .iter()
            .flat_map(|r| r.errors.iter().map(|s| s.as_str()))
            .collect()
    }
    /// Get all warnings from the build.
    pub fn all_warnings(&self) -> Vec<&str> {
        self.step_results
            .iter()
            .flat_map(|r| r.warnings.iter().map(|s| s.as_str()))
            .collect()
    }
    /// Get the average step duration.
    pub fn avg_step_duration(&self) -> Duration {
        if self.step_results.is_empty() {
            Duration::ZERO
        } else {
            let total: Duration = self.step_results.iter().map(|r| r.duration).sum();
            total / self.step_results.len() as u32
        }
    }
}
/// Log level for build messages.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LogLevel {
    /// Debug information.
    Debug,
    /// Informational message.
    Info,
    /// Warning message.
    Warning,
    /// Error message.
    Error,
}
/// Configuration for process sandbox isolation.
#[derive(Clone, Debug)]
pub struct SandboxConfig {
    /// Whether sandboxing is enabled.
    pub enabled: bool,
    /// Allowed read-only filesystem paths.
    pub read_only_paths: Vec<std::path::PathBuf>,
    /// Writable scratch directory.
    pub scratch_dir: Option<std::path::PathBuf>,
    /// Whether network access is allowed.
    pub allow_network: bool,
    /// Maximum memory in megabytes (0 = unlimited).
    pub max_memory_mb: u64,
    /// Timeout in seconds (0 = unlimited).
    pub timeout_secs: u64,
}
impl SandboxConfig {
    /// Create a minimal sandbox (no network, limited memory).
    pub fn strict() -> Self {
        Self {
            enabled: true,
            read_only_paths: Vec::new(),
            scratch_dir: None,
            allow_network: false,
            max_memory_mb: 2048,
            timeout_secs: 300,
        }
    }
    /// Add a read-only path.
    pub fn with_read_only(mut self, path: impl Into<std::path::PathBuf>) -> Self {
        self.read_only_paths.push(path.into());
        self
    }
    /// Set scratch directory.
    pub fn with_scratch(mut self, dir: impl Into<std::path::PathBuf>) -> Self {
        self.scratch_dir = Some(dir.into());
        self
    }
    /// Allow network access.
    pub fn allow_network(mut self) -> Self {
        self.allow_network = true;
        self
    }
}
