//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::Name;
use oxilean_parse::Decl;
use std::collections::{HashMap, HashSet, VecDeque};

/// A simplified work-stealing deque that allows a "thief" to steal from
/// the back and an "owner" to push/pop from the front.
///
/// This is a purely sequential simulation for use in single-threaded
/// elaboration scheduling; a real implementation would use atomic operations.
#[derive(Debug)]
pub struct WorkStealDeque<T> {
    pub items: VecDeque<T>,
}
impl<T> WorkStealDeque<T> {
    /// Create an empty deque.
    pub fn new() -> Self {
        Self::default()
    }
    /// Push a task onto the owner's end (front).
    pub fn push(&mut self, item: T) {
        self.items.push_front(item);
    }
    /// Pop a task from the owner's end (front).
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop_front()
    }
    /// Steal a task from the thief's end (back).
    pub fn steal(&mut self) -> Option<T> {
        self.items.pop_back()
    }
    /// Return the number of tasks in the deque.
    pub fn len(&self) -> usize {
        self.items.len()
    }
    /// Return `true` if the deque is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}
/// Batch execution result.
#[derive(Clone, Debug)]
pub struct BatchExecutionResult {
    /// Batch ID
    pub batch_id: usize,
    /// Results in this batch
    pub results: Vec<ElabResult>,
    /// Errors in this batch
    pub errors: Vec<(TaskId, String)>,
    /// Batch execution time
    pub batch_time_ms: u128,
}
impl BatchExecutionResult {
    /// Check if batch was successful.
    pub fn is_successful(&self) -> bool {
        self.errors.is_empty()
    }
    /// Get success rate.
    pub fn success_rate(&self) -> f64 {
        let total = (self.results.len() + self.errors.len()) as f64;
        if total == 0.0 {
            1.0
        } else {
            self.results.len() as f64 / total
        }
    }
}
/// Priority levels for elaboration tasks.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TaskPriority {
    /// Lowest priority — background tasks.
    Low = 0,
    /// Normal priority — default.
    Normal = 1,
    /// High priority — critical path tasks.
    High = 2,
    /// Urgent — must execute next.
    Urgent = 3,
}
/// Schedules and executes elaboration tasks.
pub struct ParallelScheduler {
    /// Task graph
    pub graph: TaskGraph,
    /// Configuration
    pub config: SchedulerConfig,
    /// Execution results
    pub results: Vec<ElabResult>,
    /// Failed results
    pub failures: Vec<(TaskId, String)>,
}
impl ParallelScheduler {
    /// Create a new scheduler with default config.
    pub fn new() -> Self {
        ParallelScheduler {
            graph: TaskGraph::new(),
            config: SchedulerConfig::new(),
            results: Vec::new(),
            failures: Vec::new(),
        }
    }
    /// Create a new scheduler with custom config.
    pub fn with_config(config: SchedulerConfig) -> Self {
        ParallelScheduler {
            graph: TaskGraph::new(),
            config,
            results: Vec::new(),
            failures: Vec::new(),
        }
    }
    /// Get the wavefront of ready tasks.
    pub fn wavefront(&self) -> Vec<TaskId> {
        self.graph.ready_tasks()
    }
    /// Execute all tasks sequentially.
    pub fn execute_sequential<F>(&mut self, mut executor: F) -> Result<Vec<ElabResult>, TaskError>
    where
        F: FnMut(TaskId) -> Result<ElabResult, String>,
    {
        let order = self.graph.topological_order()?;
        for task_id in order {
            self.graph.start_task(task_id)?;
            match executor(task_id) {
                Ok(result) => {
                    self.graph.complete_task(task_id)?;
                    self.results.push(result);
                }
                Err(reason) => {
                    self.graph.fail_task(task_id, reason.clone())?;
                    self.failures.push((task_id, reason));
                }
            }
        }
        Ok(self.results.clone())
    }
    /// Execute tasks in wavefront order (simulated parallelism).
    pub fn execute_wavefront<F>(&mut self, mut executor: F) -> Result<Vec<ElabResult>, TaskError>
    where
        F: FnMut(TaskId) -> Result<ElabResult, String>,
    {
        loop {
            let wavefront = self.wavefront();
            if wavefront.is_empty() {
                break;
            }
            for task_id in wavefront {
                self.graph.start_task(task_id)?;
                match executor(task_id) {
                    Ok(result) => {
                        self.graph.complete_task(task_id)?;
                        self.results.push(result);
                    }
                    Err(reason) => {
                        self.graph.fail_task(task_id, reason.clone())?;
                        self.failures.push((task_id, reason));
                    }
                }
            }
        }
        Ok(self.results.clone())
    }
    /// Get execution summary.
    pub fn summary(&self) -> SchedulerSummary {
        let total = self.graph.total_count();
        let completed = self.graph.completed_count();
        let failed = self.graph.failed_count();
        let total_time_ms: u128 = self.results.iter().map(|r| r.elapsed_ms).sum();
        SchedulerSummary {
            total,
            completed,
            failed,
            total_time_ms,
        }
    }
}
/// Stub types for Rayon integration.
///
/// In a real implementation these would forward to `rayon::scope` / `rayon::join`.
/// Here we provide sequential fallbacks so the elaborator compiles without the
/// `rayon` feature.
/// Configuration for the parallel elaboration backend.
#[derive(Clone, Debug)]
pub struct ParallelElabConfig {
    /// Number of worker threads (0 = use number of CPUs).
    pub num_threads: usize,
    /// Maximum task-graph size before falling back to sequential.
    pub max_parallel_tasks: usize,
    /// Whether to enable work-stealing.
    pub work_stealing: bool,
    /// Whether to collect per-task timing.
    pub collect_timing: bool,
}
impl ParallelElabConfig {
    /// Create a default config.
    pub fn new() -> Self {
        Self::default()
    }
    /// Use a fixed number of threads.
    pub fn with_threads(mut self, n: usize) -> Self {
        self.num_threads = n;
        self
    }
    /// Disable work-stealing.
    pub fn no_work_stealing(mut self) -> Self {
        self.work_stealing = false;
        self
    }
    /// Enable per-task timing collection.
    pub fn with_timing(mut self) -> Self {
        self.collect_timing = true;
        self
    }
    /// Effective number of worker threads.
    pub fn effective_threads(&self) -> usize {
        if self.num_threads == 0 {
            4
        } else {
            self.num_threads
        }
    }
}
/// Statistics for a parallel elaboration run.
#[derive(Clone, Debug, Default)]
pub struct ParallelElabStats {
    /// Total tasks executed.
    pub tasks_executed: u64,
    /// Tasks that were stolen (work-stealing events).
    pub tasks_stolen: u64,
    /// Total wall-clock time in milliseconds.
    pub wall_time_ms: u64,
    /// Total CPU time across all threads in milliseconds.
    pub cpu_time_ms: u64,
    /// Number of batches executed.
    pub batches: u64,
}
impl ParallelElabStats {
    /// Create zeroed stats.
    pub fn new() -> Self {
        Self::default()
    }
    /// Speedup ratio (CPU time / wall time).  Values > 1 indicate parallelism.
    pub fn speedup(&self) -> f64 {
        if self.wall_time_ms == 0 {
            1.0
        } else {
            self.cpu_time_ms as f64 / self.wall_time_ms as f64
        }
    }
    /// Work-stealing rate (stolen / executed).
    pub fn steal_rate(&self) -> f64 {
        if self.tasks_executed == 0 {
            0.0
        } else {
            self.tasks_stolen as f64 / self.tasks_executed as f64
        }
    }
    /// Merge another stats object into this one.
    pub fn merge(&mut self, other: &ParallelElabStats) {
        self.tasks_executed += other.tasks_executed;
        self.tasks_stolen += other.tasks_stolen;
        self.wall_time_ms += other.wall_time_ms;
        self.cpu_time_ms += other.cpu_time_ms;
        self.batches += other.batches;
    }
    /// One-line summary.
    pub fn summary(&self) -> String {
        format!(
            "tasks={} stolen={} wall={}ms cpu={}ms batches={} speedup={:.2}",
            self.tasks_executed,
            self.tasks_stolen,
            self.wall_time_ms,
            self.cpu_time_ms,
            self.batches,
            self.speedup()
        )
    }
}
/// A task with an associated priority.
#[derive(Clone, Debug)]
pub struct PrioritizedTask {
    /// The task identifier.
    pub id: TaskId,
    /// The priority level.
    pub priority: TaskPriority,
}
impl PrioritizedTask {
    /// Create a normal-priority task.
    pub fn normal(id: TaskId) -> Self {
        Self {
            id,
            priority: TaskPriority::Normal,
        }
    }
    /// Create a high-priority task.
    pub fn high(id: TaskId) -> Self {
        Self {
            id,
            priority: TaskPriority::High,
        }
    }
    /// Create an urgent task.
    pub fn urgent(id: TaskId) -> Self {
        Self {
            id,
            priority: TaskPriority::Urgent,
        }
    }
}
/// Statistics about task execution.
#[derive(Clone, Debug)]
pub struct ExecutionStats {
    /// Average task execution time in milliseconds
    pub avg_time_ms: u128,
    /// Minimum task execution time
    pub min_time_ms: u128,
    /// Maximum task execution time
    pub max_time_ms: u128,
    /// Total parallelism factor
    pub parallelism_factor: f64,
}
impl ExecutionStats {
    /// Calculate execution statistics from results.
    pub fn from_results(results: &[ElabResult]) -> Option<Self> {
        if results.is_empty() {
            return None;
        }
        let mut total_time = 0u128;
        let mut min_time = u128::MAX;
        let mut max_time = 0u128;
        for result in results {
            total_time += result.elapsed_ms;
            min_time = min_time.min(result.elapsed_ms);
            max_time = max_time.max(result.elapsed_ms);
        }
        let avg_time_ms = total_time / results.len() as u128;
        let parallelism_factor = if max_time > 0 {
            total_time as f64 / max_time as f64
        } else {
            0.0
        };
        Some(ExecutionStats {
            avg_time_ms,
            min_time_ms: min_time,
            max_time_ms: max_time,
            parallelism_factor,
        })
    }
}
/// Tracks progress of elaboration.
pub struct ProgressTracker {
    /// Number of completed tasks
    pub completed: usize,
    /// Total number of tasks
    pub total: usize,
    /// Number of failed tasks
    pub failed: usize,
}
impl ProgressTracker {
    /// Create a new progress tracker.
    pub fn new(total: usize) -> Self {
        ProgressTracker {
            completed: 0,
            total,
            failed: 0,
        }
    }
    /// Mark a task as completed.
    pub fn mark_completed(&mut self) {
        if self.completed < self.total {
            self.completed += 1;
        }
    }
    /// Mark a task as failed.
    pub fn mark_failed(&mut self) {
        if self.failed < self.total {
            self.failed += 1;
        }
    }
    /// Get progress percentage (0-100).
    pub fn progress_pct(&self) -> u32 {
        if self.total == 0 {
            return 100;
        }
        ((self.completed as u32 * 100) / self.total as u32).min(100)
    }
    /// Format progress as a string.
    pub fn format_progress(&self) -> String {
        let pct = self.progress_pct();
        let bar_len = 30;
        let filled = (bar_len * pct as usize) / 100;
        let bar = format!("[{}{}]", "=".repeat(filled), " ".repeat(bar_len - filled));
        format!(
            "{} {}/{}  {}%  ({} failed)",
            bar, self.completed, self.total, pct, self.failed
        )
    }
    /// Reset the tracker.
    pub fn reset(&mut self) {
        self.completed = 0;
        self.failed = 0;
    }
}
/// A priority queue for elaboration tasks.
///
/// Tasks are dequeued in priority order (highest first), with FIFO ordering
/// within the same priority level.
#[derive(Debug, Default)]
pub struct TaskPriorityQueue {
    buckets: HashMap<u8, VecDeque<TaskId>>,
}
impl TaskPriorityQueue {
    /// Create an empty queue.
    pub fn new() -> Self {
        Self::default()
    }
    /// Enqueue a task at the given priority.
    pub fn enqueue(&mut self, id: TaskId, priority: TaskPriority) {
        self.buckets
            .entry(priority as u8)
            .or_default()
            .push_back(id);
    }
    /// Dequeue the highest-priority task.
    pub fn dequeue(&mut self) -> Option<(TaskId, TaskPriority)> {
        for &prio in &[
            TaskPriority::Urgent as u8,
            TaskPriority::High as u8,
            TaskPriority::Normal as u8,
            TaskPriority::Low as u8,
        ] {
            if let Some(bucket) = self.buckets.get_mut(&prio) {
                if let Some(id) = bucket.pop_front() {
                    let p = match prio {
                        3 => TaskPriority::Urgent,
                        2 => TaskPriority::High,
                        1 => TaskPriority::Normal,
                        _ => TaskPriority::Low,
                    };
                    return Some((id, p));
                }
            }
        }
        None
    }
    /// Total number of tasks in all buckets.
    pub fn len(&self) -> usize {
        self.buckets.values().map(|b| b.len()).sum()
    }
    /// Return `true` if no tasks are queued.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
/// The outcome of a single parallel elaboration task.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum ParallelTaskOutcome {
    /// Task completed successfully.
    Success { task_id: TaskId, duration_ns: u64 },
    /// Task failed with an error message.
    Failure { task_id: TaskId, reason: String },
    /// Task was skipped (e.g., dependency failed).
    Skipped { task_id: TaskId },
}
#[allow(dead_code)]
impl ParallelTaskOutcome {
    /// Return true if the task succeeded.
    pub fn is_success(&self) -> bool {
        matches!(self, ParallelTaskOutcome::Success { .. })
    }
    /// Return the task ID.
    pub fn task_id(&self) -> TaskId {
        match self {
            ParallelTaskOutcome::Success { task_id, .. } => *task_id,
            ParallelTaskOutcome::Failure { task_id, .. } => *task_id,
            ParallelTaskOutcome::Skipped { task_id } => *task_id,
        }
    }
    /// Return the duration in nanoseconds, if available.
    pub fn duration_ns(&self) -> Option<u64> {
        match self {
            ParallelTaskOutcome::Success { duration_ns, .. } => Some(*duration_ns),
            _ => None,
        }
    }
}
/// Result of a completed elaboration task.
#[derive(Clone, Debug)]
pub struct ElabResult {
    /// Task ID
    pub task_id: TaskId,
    /// Declaration name
    pub decl_name: Name,
    /// Elapsed time in milliseconds
    pub elapsed_ms: u128,
    /// Elaboration output
    pub output: ElabOutput,
}
impl ElabResult {
    /// Create a new elaboration result.
    pub fn new(task_id: TaskId, decl_name: Name, elapsed_ms: u128, output: ElabOutput) -> Self {
        ElabResult {
            task_id,
            decl_name,
            elapsed_ms,
            output,
        }
    }
}
/// Aggregated results from a parallel elaboration batch.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct ParallelBatchResult {
    outcomes: Vec<ParallelTaskOutcome>,
}
#[allow(dead_code)]
impl ParallelBatchResult {
    /// Create an empty batch result.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add an outcome.
    pub fn push(&mut self, outcome: ParallelTaskOutcome) {
        self.outcomes.push(outcome);
    }
    /// Return the number of successful tasks.
    pub fn success_count(&self) -> usize {
        self.outcomes.iter().filter(|o| o.is_success()).count()
    }
    /// Return the number of failed tasks.
    pub fn failure_count(&self) -> usize {
        self.outcomes
            .iter()
            .filter(|o| matches!(o, ParallelTaskOutcome::Failure { .. }))
            .count()
    }
    /// Return the number of skipped tasks.
    pub fn skipped_count(&self) -> usize {
        self.outcomes
            .iter()
            .filter(|o| matches!(o, ParallelTaskOutcome::Skipped { .. }))
            .count()
    }
    /// Return the success rate in [0, 1].
    pub fn success_rate(&self) -> f64 {
        let n = self.outcomes.len();
        if n == 0 {
            return 1.0;
        }
        self.success_count() as f64 / n as f64
    }
    /// Return average duration of successful tasks in nanoseconds.
    pub fn avg_success_ns(&self) -> f64 {
        let durations: Vec<u64> = self
            .outcomes
            .iter()
            .filter_map(|o| o.duration_ns())
            .collect();
        if durations.is_empty() {
            return 0.0;
        }
        durations.iter().sum::<u64>() as f64 / durations.len() as f64
    }
    /// Return all failed task IDs with their error reasons.
    pub fn failures(&self) -> Vec<(TaskId, &str)> {
        self.outcomes
            .iter()
            .filter_map(|o| {
                if let ParallelTaskOutcome::Failure { task_id, reason } = o {
                    Some((*task_id, reason.as_str()))
                } else {
                    None
                }
            })
            .collect()
    }
}
/// Configuration for the parallel scheduler.
#[derive(Clone, Debug)]
pub struct SchedulerConfig {
    /// Maximum number of concurrent tasks
    pub max_concurrent: usize,
    /// Enable progress reporting
    pub report_progress: bool,
    /// Timeout per task in milliseconds
    pub task_timeout_ms: u64,
    /// Maximum retries on failure
    pub max_retries: usize,
}
impl SchedulerConfig {
    /// Create a new scheduler configuration with defaults.
    pub fn new() -> Self {
        SchedulerConfig {
            max_concurrent: num_cpus_fallback(),
            report_progress: true,
            task_timeout_ms: 30000,
            max_retries: 0,
        }
    }
    /// Set maximum concurrent tasks.
    pub fn with_max_concurrent(mut self, max: usize) -> Self {
        self.max_concurrent = max.max(1);
        self
    }
    /// Enable or disable progress reporting.
    pub fn with_progress_reporting(mut self, enabled: bool) -> Self {
        self.report_progress = enabled;
        self
    }
    /// Set task timeout.
    pub fn with_timeout(mut self, ms: u64) -> Self {
        self.task_timeout_ms = ms;
        self
    }
    /// Set max retries.
    pub fn with_max_retries(mut self, retries: usize) -> Self {
        self.max_retries = retries;
        self
    }
}
/// Errors that can occur during task processing.
#[derive(Clone, Debug)]
pub enum TaskError {
    /// Task not found in the graph
    TaskNotFound(TaskId),
    /// Cyclic dependency detected
    CyclicDependency(Vec<TaskId>),
    /// Invalid state transition
    InvalidStateTransition {
        /// Current state
        from: TaskStatus,
        /// Attempted state
        to: TaskStatus,
    },
    /// Dependency failed, cannot proceed
    DependencyFailed(TaskId),
    /// Task execution failed
    ExecutionFailed {
        /// Task ID
        task_id: TaskId,
        /// Error message
        reason: String,
    },
}
/// Summary of scheduler execution.
#[derive(Clone, Debug)]
pub struct SchedulerSummary {
    /// Total tasks
    pub total: usize,
    /// Completed tasks
    pub completed: usize,
    /// Failed tasks
    pub failed: usize,
    /// Total execution time in milliseconds
    pub total_time_ms: u128,
}
/// Configuration for dependency analysis.
#[derive(Clone, Debug)]
pub struct DependencyConfig {
    /// Maximum depth to analyze
    pub max_depth: usize,
    /// Include transitive dependencies
    pub transitive: bool,
    /// Detect circular dependencies
    pub detect_cycles: bool,
}
impl DependencyConfig {
    /// Create with default settings.
    pub fn new() -> Self {
        DependencyConfig {
            max_depth: 100,
            transitive: true,
            detect_cycles: true,
        }
    }
}
/// Manages task dependencies and scheduling.
pub struct TaskGraph {
    /// All tasks indexed by ID
    pub(super) tasks: HashMap<TaskId, ElabTask>,
    /// Next available task ID
    next_id: u64,
    /// Completed task IDs
    completed: HashSet<TaskId>,
    /// Failed task IDs
    failed: HashSet<TaskId>,
}
impl TaskGraph {
    /// Create a new empty task graph.
    pub fn new() -> Self {
        TaskGraph {
            tasks: HashMap::new(),
            next_id: 0,
            completed: HashSet::new(),
            failed: HashSet::new(),
        }
    }
    /// Add a new task to the graph.
    pub fn add_task(&mut self, name: String, decl_name: Name) -> TaskId {
        let id = TaskId::new(self.next_id);
        self.next_id += 1;
        let task = ElabTask::new(id, name, decl_name);
        self.tasks.insert(id, task);
        id
    }
    /// Add a dependency between two tasks.
    pub fn add_dependency(
        &mut self,
        dependent: TaskId,
        dependency: TaskId,
    ) -> Result<(), TaskError> {
        if !self.tasks.contains_key(&dependent) {
            return Err(TaskError::TaskNotFound(dependent));
        }
        if !self.tasks.contains_key(&dependency) {
            return Err(TaskError::TaskNotFound(dependency));
        }
        if let Some(task) = self.tasks.get_mut(&dependent) {
            task.add_dependency(dependency);
        }
        Ok(())
    }
    /// Get all tasks that are ready to run.
    pub fn ready_tasks(&self) -> Vec<TaskId> {
        self.tasks
            .values()
            .filter(|task| task.is_ready(&self.completed))
            .map(|task| task.id)
            .collect()
    }
    /// Mark a task as completed.
    pub fn complete_task(&mut self, id: TaskId) -> Result<(), TaskError> {
        if let Some(task) = self.tasks.get_mut(&id) {
            if task.status != TaskStatus::Pending && task.status != TaskStatus::Running {
                return Err(TaskError::InvalidStateTransition {
                    from: task.status.clone(),
                    to: TaskStatus::Completed,
                });
            }
            task.status = TaskStatus::Completed;
            self.completed.insert(id);
            Ok(())
        } else {
            Err(TaskError::TaskNotFound(id))
        }
    }
    /// Mark a task as failed.
    pub fn fail_task(&mut self, id: TaskId, _reason: String) -> Result<(), TaskError> {
        if let Some(task) = self.tasks.get_mut(&id) {
            task.status = TaskStatus::Failed;
            self.failed.insert(id);
            Ok(())
        } else {
            Err(TaskError::TaskNotFound(id))
        }
    }
    /// Mark a task as running.
    pub fn start_task(&mut self, id: TaskId) -> Result<(), TaskError> {
        if let Some(task) = self.tasks.get_mut(&id) {
            if task.status != TaskStatus::Pending {
                return Err(TaskError::InvalidStateTransition {
                    from: task.status.clone(),
                    to: TaskStatus::Running,
                });
            }
            task.status = TaskStatus::Running;
            Ok(())
        } else {
            Err(TaskError::TaskNotFound(id))
        }
    }
    /// Detect cycles using DFS.
    pub fn detect_cycles(&self) -> Option<Vec<TaskId>> {
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        let mut path = Vec::new();
        for task_id in self.tasks.keys() {
            if !visited.contains(task_id) {
                if let Some(cycle) =
                    self.dfs_cycle_check(*task_id, &mut visited, &mut rec_stack, &mut path)
                {
                    return Some(cycle);
                }
            }
        }
        None
    }
    fn dfs_cycle_check(
        &self,
        node: TaskId,
        visited: &mut HashSet<TaskId>,
        rec_stack: &mut HashSet<TaskId>,
        path: &mut Vec<TaskId>,
    ) -> Option<Vec<TaskId>> {
        visited.insert(node);
        rec_stack.insert(node);
        path.push(node);
        if let Some(task) = self.tasks.get(&node) {
            for dep in &task.deps {
                if !visited.contains(dep) {
                    if let Some(cycle) = self.dfs_cycle_check(*dep, visited, rec_stack, path) {
                        return Some(cycle);
                    }
                } else if rec_stack.contains(dep) {
                    let idx = path.iter().position(|&x| x == *dep).unwrap_or(0);
                    return Some(path[idx..].to_vec());
                }
            }
        }
        rec_stack.remove(&node);
        path.pop();
        None
    }
    /// Compute topological ordering of tasks.
    pub fn topological_order(&self) -> Result<Vec<TaskId>, TaskError> {
        if let Some(cycle) = self.detect_cycles() {
            return Err(TaskError::CyclicDependency(cycle));
        }
        let mut in_degree: HashMap<TaskId, usize> = HashMap::new();
        let mut queue: VecDeque<TaskId> = VecDeque::new();
        let mut result = Vec::new();
        let mut graph_reverse: HashMap<TaskId, Vec<TaskId>> = HashMap::new();
        for task_id in self.tasks.keys() {
            in_degree.insert(*task_id, 0);
            graph_reverse.insert(*task_id, Vec::new());
        }
        for task in self.tasks.values() {
            for dep in &task.deps {
                in_degree.entry(task.id).and_modify(|d| *d += 1);
                graph_reverse.entry(*dep).or_default().push(task.id);
            }
        }
        for (task_id, &degree) in &in_degree {
            if degree == 0 {
                queue.push_back(*task_id);
            }
        }
        while let Some(current) = queue.pop_front() {
            result.push(current);
            if let Some(dependents) = graph_reverse.get(&current) {
                for dependent in dependents {
                    let new_degree = in_degree[dependent] - 1;
                    in_degree.insert(*dependent, new_degree);
                    if new_degree == 0 {
                        queue.push_back(*dependent);
                    }
                }
            }
        }
        if result.len() == self.tasks.len() {
            Ok(result)
        } else {
            Err(TaskError::CyclicDependency(Vec::new()))
        }
    }
    /// Compute critical path length for a task.
    pub fn critical_path_length(&self, task_id: TaskId) -> usize {
        if !self.tasks.contains_key(&task_id) {
            return 0;
        }
        let mut memo = HashMap::new();
        self.compute_critical_path(task_id, &mut memo)
    }
    fn compute_critical_path(&self, node: TaskId, memo: &mut HashMap<TaskId, usize>) -> usize {
        if let Some(&length) = memo.get(&node) {
            return length;
        }
        let mut max_depth = 0;
        if let Some(task) = self.tasks.get(&node) {
            for dep in &task.deps {
                let depth = 1 + self.compute_critical_path(*dep, memo);
                max_depth = max_depth.max(depth);
            }
        }
        memo.insert(node, max_depth);
        max_depth
    }
    /// Get a task by ID.
    pub fn get_task(&self, id: TaskId) -> Option<&ElabTask> {
        self.tasks.get(&id)
    }
    /// Get a mutable task by ID.
    pub fn get_task_mut(&mut self, id: TaskId) -> Option<&mut ElabTask> {
        self.tasks.get_mut(&id)
    }
    /// Get all tasks.
    pub fn all_tasks(&self) -> Vec<&ElabTask> {
        self.tasks.values().collect()
    }
    /// Get count of completed tasks.
    pub fn completed_count(&self) -> usize {
        self.completed.len()
    }
    /// Get count of failed tasks.
    pub fn failed_count(&self) -> usize {
        self.failed.len()
    }
    /// Get total task count.
    pub fn total_count(&self) -> usize {
        self.tasks.len()
    }
    /// Clear all state.
    pub fn clear(&mut self) {
        self.tasks.clear();
        self.next_id = 0;
        self.completed.clear();
        self.failed.clear();
    }
}
/// Task dependency information.
#[derive(Clone, Debug)]
pub struct TaskDependencyInfo {
    /// Task ID
    pub task_id: TaskId,
    /// Tasks this task depends on
    pub dependencies: Vec<TaskId>,
    /// Tasks that depend on this task
    pub dependents: Vec<TaskId>,
    /// Depth in dependency tree (0 = root)
    pub depth: usize,
    /// Breadth in dependency tree
    pub breadth: usize,
}
impl TaskDependencyInfo {
    /// Analyze task dependencies.
    pub fn analyze(graph: &TaskGraph, task_id: TaskId) -> Option<Self> {
        let task = graph.get_task(task_id)?;
        let dependencies = task.deps.clone();
        let dependents = graph
            .all_tasks()
            .iter()
            .filter(|t| t.deps.contains(&task_id))
            .map(|t| t.id)
            .collect();
        Some(TaskDependencyInfo {
            task_id,
            dependencies,
            dependents,
            depth: 0,
            breadth: 0,
        })
    }
}
/// Result of elaborating a single declaration.
#[derive(Clone, Debug)]
pub struct ElabOutput {
    /// Name of the elaborated declaration
    pub decl_name: Name,
    /// Elaborated declaration
    pub decl: String,
}
impl ElabOutput {
    /// Create a new elaboration output.
    pub fn new(decl_name: Name, decl: String) -> Self {
        ElabOutput { decl_name, decl }
    }
}
/// Analyzes dependencies between declarations.
pub struct DependencyAnalyzer {
    /// Declarations being analyzed
    pub declarations: Vec<Decl>,
    /// Dependency map
    pub dependencies: HashMap<String, HashSet<String>>,
}
impl DependencyAnalyzer {
    /// Create a new dependency analyzer.
    pub fn new() -> Self {
        DependencyAnalyzer {
            declarations: Vec::new(),
            dependencies: HashMap::new(),
        }
    }
    /// Analyze a set of declarations.
    pub fn analyze_declarations(&mut self, decls: &[Decl]) -> Result<(), String> {
        self.declarations = decls.to_vec();
        for decl in decls {
            let name = self.get_decl_name(decl)?;
            self.dependencies.insert(name, HashSet::new());
        }
        for decl in decls {
            let name = self.get_decl_name(decl)?;
            let deps = self.extract_dependencies(decl)?;
            if let Some(entry) = self.dependencies.get_mut(&name) {
                *entry = deps;
            }
        }
        Ok(())
    }
    /// Extract dependencies from a single declaration.
    ///
    /// Walks the declaration's type/body expressions and collects all `Var`
    /// references that correspond to other known declarations.
    pub fn extract_dependencies(&self, decl: &Decl) -> Result<HashSet<String>, String> {
        let known: HashSet<&str> = self.dependencies.keys().map(|s| s.as_str()).collect();
        let mut refs = HashSet::new();
        collect_decl_refs(decl, &mut refs);
        Ok(refs
            .into_iter()
            .filter(|n| known.contains(n.as_str()))
            .collect())
    }
    /// Build an elaboration plan from analyzed declarations.
    pub fn build_elaboration_plan(&self) -> Result<Vec<Vec<String>>, String> {
        if self.declarations.is_empty() {
            return Ok(Vec::new());
        }
        let mut in_degree = HashMap::new();
        let mut queue = VecDeque::new();
        let mut result = Vec::new();
        for (name, deps) in &self.dependencies {
            in_degree.insert(name.clone(), deps.len());
        }
        for (name, in_deg) in in_degree.iter_mut() {
            if *in_deg == 0 {
                queue.push_back(name.clone());
            }
        }
        while !queue.is_empty() {
            let mut batch = Vec::new();
            let current_size = queue.len();
            for _ in 0..current_size {
                if let Some(current) = queue.pop_front() {
                    batch.push(current.clone());
                    for (name, deps) in self.dependencies.iter() {
                        if deps.contains(&current) {
                            let deg = in_degree
                                .get_mut(name)
                                .expect("name is from dependencies so it exists in in_degree");
                            *deg -= 1;
                            if *deg == 0 {
                                queue.push_back(name.clone());
                            }
                        }
                    }
                }
            }
            if !batch.is_empty() {
                result.push(batch);
            }
        }
        Ok(result)
    }
    fn get_decl_name(&self, decl: &Decl) -> Result<String, String> {
        use oxilean_parse::Decl as D;
        let name = match decl {
            D::Axiom { name, .. } => name.clone(),
            D::Definition { name, .. } => name.clone(),
            D::Theorem { name, .. } => name.clone(),
            D::Inductive { name, .. } => name.clone(),
            D::Structure { name, .. } => name.clone(),
            D::ClassDecl { name, .. } => name.clone(),
            D::InstanceDecl {
                name, class_name, ..
            } => name
                .clone()
                .unwrap_or_else(|| format!("{}_inst", class_name)),
            D::Derive { type_name, .. } => format!("{}_derive", type_name),
            D::Namespace { name, .. } => name.clone(),
            D::SectionDecl { name, .. } => name.clone(),
            D::Mutual { decls } => {
                if let Some(inner) = decls.first() {
                    self.get_decl_name(&inner.value)?
                } else {
                    return Err("empty mutual block".to_string());
                }
            }
            D::NotationDecl { name, .. } => name.clone(),
            D::Import { path } => path.join("."),
            D::Open { path, .. } => path.join("."),
            D::Variable { .. } => "variable".to_string(),
            D::Universe { names } => names
                .first()
                .cloned()
                .unwrap_or_else(|| "universe".to_string()),
            D::Attribute { decl, .. } => self.get_decl_name(&decl.value)?,
            D::HashCmd { cmd, .. } => format!("#{}", cmd),
        };
        Ok(name)
    }
}
/// Unique identifier for elaboration tasks.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TaskId(pub u64);
impl TaskId {
    /// Create a new TaskId from a u64.
    pub fn new(id: u64) -> Self {
        TaskId(id)
    }
    /// Get the inner u64 value.
    pub fn inner(self) -> u64 {
        self.0
    }
}
/// Status of an elaboration task.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TaskStatus {
    /// Task is waiting to be executed
    Pending,
    /// Task is currently being elaborated
    Running,
    /// Task completed successfully
    Completed,
    /// Task failed during elaboration
    Failed,
    /// Task was cancelled
    Cancelled,
}
/// Represents a single elaboration task for a declaration.
#[derive(Clone, Debug)]
pub struct ElabTask {
    /// Unique task identifier
    pub id: TaskId,
    /// Display name for the task
    pub name: String,
    /// Name of the declaration being elaborated
    pub decl_name: Name,
    /// IDs of tasks this task depends on
    pub deps: Vec<TaskId>,
    /// Current status of the task
    pub status: TaskStatus,
    /// Priority for scheduling (higher = higher priority)
    pub priority: i32,
}
impl ElabTask {
    /// Create a new elaboration task.
    pub fn new(id: TaskId, name: String, decl_name: Name) -> Self {
        ElabTask {
            id,
            name,
            decl_name,
            deps: Vec::new(),
            status: TaskStatus::Pending,
            priority: 0,
        }
    }
    /// Add a dependency to this task.
    pub fn add_dependency(&mut self, dep_id: TaskId) {
        if !self.deps.contains(&dep_id) {
            self.deps.push(dep_id);
        }
    }
    /// Check if this task is ready to run (all dependencies completed).
    pub fn is_ready(&self, completed: &HashSet<TaskId>) -> bool {
        self.status == TaskStatus::Pending && self.deps.iter().all(|d| completed.contains(d))
    }
    /// Set the priority for this task.
    pub fn set_priority(&mut self, priority: i32) {
        self.priority = priority;
    }
}
/// A simple multi-producer single-consumer queue for parallel task dispatch.
#[derive(Debug, Default)]
pub struct ParallelTaskQueue {
    /// Tasks waiting to be dispatched.
    pending: VecDeque<TaskId>,
    /// Total tasks ever enqueued.
    total_enqueued: u64,
    /// Total tasks ever dequeued.
    total_dequeued: u64,
}
impl ParallelTaskQueue {
    /// Create an empty queue.
    pub fn new() -> Self {
        Self::default()
    }
    /// Enqueue a task.
    pub fn enqueue(&mut self, id: TaskId) {
        self.pending.push_back(id);
        self.total_enqueued += 1;
    }
    /// Dequeue the next task, if any.
    pub fn dequeue(&mut self) -> Option<TaskId> {
        let item = self.pending.pop_front();
        if item.is_some() {
            self.total_dequeued += 1;
        }
        item
    }
    /// Number of tasks currently pending.
    pub fn pending(&self) -> usize {
        self.pending.len()
    }
    /// Return `true` if no tasks are pending.
    pub fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }
    /// Total tasks enqueued since creation.
    pub fn total_enqueued(&self) -> u64 {
        self.total_enqueued
    }
    /// Total tasks dequeued since creation.
    pub fn total_dequeued(&self) -> u64 {
        self.total_dequeued
    }
    /// Throughput: dequeued / enqueued (in [0, 1]).
    pub fn throughput(&self) -> f64 {
        if self.total_enqueued == 0 {
            1.0
        } else {
            self.total_dequeued as f64 / self.total_enqueued as f64
        }
    }
}
