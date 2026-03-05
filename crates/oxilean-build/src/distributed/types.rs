//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet, VecDeque};

/// Top-level cluster configuration.
#[derive(Clone, Debug)]
pub struct ClusterConfig {
    /// Cluster name for identification.
    pub name: String,
    /// Maximum total workers.
    pub max_workers: usize,
    /// Whether dynamic worker allocation is allowed.
    pub dynamic_scaling: bool,
    /// Target cluster utilization (0.0–1.0) before scaling.
    pub scale_threshold: f64,
}
impl ClusterConfig {
    /// Whether scaling should be triggered given `utilization`.
    pub fn should_scale_up(&self, utilization: f64) -> bool {
        self.dynamic_scaling && utilization >= self.scale_threshold
    }
}
/// Represents a remote build worker
#[derive(Debug, Clone)]
pub struct RemoteWorker {
    pub id: String,
    pub address: String,
    pub capacity: u32,
    pub current_jobs: u32,
    pub available: bool,
}
impl RemoteWorker {
    pub fn new(id: &str, address: &str, capacity: u32) -> Self {
        RemoteWorker {
            id: id.to_string(),
            address: address.to_string(),
            capacity,
            current_jobs: 0,
            available: true,
        }
    }
    pub fn is_available(&self) -> bool {
        self.available && self.current_jobs < self.capacity
    }
    /// current_jobs / capacity (0.0 if capacity == 0)
    pub fn utilization(&self) -> f64 {
        if self.capacity == 0 {
            0.0
        } else {
            self.current_jobs as f64 / self.capacity as f64
        }
    }
}
/// A pool of remote workers with load-balanced assignment.
pub struct WorkerPool {
    workers: Vec<RemoteWorker>,
    /// Round-robin cursor for basic scheduling.
    next_idx: usize,
}
impl WorkerPool {
    /// Create an empty pool.
    pub fn new() -> Self {
        Self {
            workers: Vec::new(),
            next_idx: 0,
        }
    }
    /// Add a worker to the pool.
    pub fn add(&mut self, worker: RemoteWorker) {
        self.workers.push(worker);
    }
    /// Number of workers.
    pub fn len(&self) -> usize {
        self.workers.len()
    }
    /// Whether the pool is empty.
    pub fn is_empty(&self) -> bool {
        self.workers.is_empty()
    }
    /// Count available workers.
    pub fn available_count(&self) -> usize {
        self.workers.iter().filter(|w| w.is_available()).count()
    }
    /// Return the least-loaded available worker index, if any.
    pub fn least_loaded_idx(&self) -> Option<usize> {
        self.workers
            .iter()
            .enumerate()
            .filter(|(_, w)| w.is_available())
            .min_by(|(_, a), (_, b)| {
                a.utilization()
                    .partial_cmp(&b.utilization())
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(i, _)| i)
    }
    /// Round-robin pick (ignores availability — caller must check).
    pub fn round_robin_idx(&mut self) -> Option<usize> {
        if self.workers.is_empty() {
            return None;
        }
        let idx = self.next_idx % self.workers.len();
        self.next_idx += 1;
        Some(idx)
    }
    /// Mark worker at `idx` as starting a job.
    pub fn start_job(&mut self, idx: usize) {
        if let Some(w) = self.workers.get_mut(idx) {
            if w.current_jobs < w.capacity {
                w.current_jobs += 1;
            }
        }
    }
    /// Mark worker at `idx` as finishing a job.
    pub fn finish_job(&mut self, idx: usize) {
        if let Some(w) = self.workers.get_mut(idx) {
            if w.current_jobs > 0 {
                w.current_jobs -= 1;
            }
        }
    }
    /// Average utilization across all workers.
    pub fn avg_utilization(&self) -> f64 {
        if self.workers.is_empty() {
            return 0.0;
        }
        let sum: f64 = self.workers.iter().map(|w| w.utilization()).sum();
        sum / self.workers.len() as f64
    }
    /// Maximum utilization across all workers.
    pub fn max_utilization(&self) -> f64 {
        self.workers
            .iter()
            .map(|w| w.utilization())
            .fold(0.0_f64, f64::max)
    }
    /// Mark worker `id` as unavailable.
    pub fn disable_worker(&mut self, id: &str) {
        if let Some(w) = self.workers.iter_mut().find(|w| w.id == id) {
            w.available = false;
        }
    }
    /// Mark worker `id` as available again.
    pub fn enable_worker(&mut self, id: &str) {
        if let Some(w) = self.workers.iter_mut().find(|w| w.id == id) {
            w.available = true;
        }
    }
}
/// Fault-tolerance configuration for distributed builds.
#[derive(Clone, Debug)]
pub struct FaultTolerance {
    /// Number of times to retry a failed task.
    pub max_retries: u32,
    /// Whether to re-submit a task to a different worker on failure.
    pub reroute_on_failure: bool,
    /// Whether to use speculative execution (submit duplicate task to another worker).
    pub speculative_execution: bool,
    /// Timeout per task in seconds.
    pub task_timeout_secs: u64,
}
impl FaultTolerance {
    /// Sensible defaults.
    pub fn default_ft() -> Self {
        Self {
            max_retries: 2,
            reroute_on_failure: true,
            speculative_execution: false,
            task_timeout_secs: 120,
        }
    }
    /// Whether a task at attempt `n` (0-indexed) can be retried.
    pub fn can_retry(&self, n: u32) -> bool {
        n < self.max_retries
    }
}
/// Manages speculative duplicate task submissions for latency reduction.
pub struct SpeculativeExecutor {
    /// task_id → set of workers it was sent to.
    sent_to: std::collections::HashMap<String, std::collections::HashSet<String>>,
    /// task_id → first-to-finish worker.
    winner: std::collections::HashMap<String, String>,
}
impl SpeculativeExecutor {
    /// Create an empty executor.
    pub fn new() -> Self {
        Self {
            sent_to: std::collections::HashMap::new(),
            winner: std::collections::HashMap::new(),
        }
    }
    /// Register that `task_id` was sent to `worker_id`.
    pub fn register_send(&mut self, task_id: &str, worker_id: &str) {
        self.sent_to
            .entry(task_id.to_string())
            .or_default()
            .insert(worker_id.to_string());
    }
    /// Record that `worker_id` finished `task_id` first.
    /// Returns `true` if this is the first finish (winner).
    pub fn record_finish(&mut self, task_id: &str, worker_id: &str) -> bool {
        if self.winner.contains_key(task_id) {
            return false;
        }
        self.winner
            .insert(task_id.to_string(), worker_id.to_string());
        true
    }
    /// Workers to cancel for `task_id` (all except winner).
    pub fn workers_to_cancel(&self, task_id: &str) -> Vec<String> {
        let winner = self.winner.get(task_id).cloned().unwrap_or_default();
        self.sent_to
            .get(task_id)
            .map(|set| set.iter().filter(|&w| w != &winner).cloned().collect())
            .unwrap_or_default()
    }
    /// Whether `task_id` has been completed.
    pub fn is_done(&self, task_id: &str) -> bool {
        self.winner.contains_key(task_id)
    }
}
/// Remote cache for build artifacts
#[derive(Debug)]
pub struct RemoteCache {
    pub endpoint: String,
    pub enabled: bool,
    pub max_size_mb: u64,
    pub current_size_mb: u64,
    pub hit_count: u64,
    pub miss_count: u64,
    store: std::collections::HashMap<String, Vec<u8>>,
}
impl RemoteCache {
    pub fn new(endpoint: &str, max_size_mb: u64) -> Self {
        RemoteCache {
            endpoint: endpoint.to_string(),
            enabled: true,
            max_size_mb,
            current_size_mb: 0,
            hit_count: 0,
            miss_count: 0,
            store: std::collections::HashMap::new(),
        }
    }
    /// Cache hit rate (hits / (hits + misses)); 0.0 if no requests yet
    pub fn hit_rate(&self) -> f64 {
        let total = self.hit_count + self.miss_count;
        if total == 0 {
            0.0
        } else {
            self.hit_count as f64 / total as f64
        }
    }
    pub fn is_full(&self) -> bool {
        self.current_size_mb >= self.max_size_mb
    }
    /// Percentage of max_size_mb currently used
    pub fn usage_percent(&self) -> f64 {
        if self.max_size_mb == 0 {
            0.0
        } else {
            (self.current_size_mb as f64 / self.max_size_mb as f64) * 100.0
        }
    }
    /// Stub: look up key; returns data if present, None otherwise
    pub fn try_get(&mut self, key: &str) -> Option<Vec<u8>> {
        if !self.enabled {
            self.miss_count += 1;
            return None;
        }
        match self.store.get(key).cloned() {
            Some(data) => {
                self.hit_count += 1;
                Some(data)
            }
            None => {
                self.miss_count += 1;
                None
            }
        }
    }
    /// Stub: store data under key
    pub fn put(&mut self, key: &str, data: Vec<u8>) {
        if !self.enabled || self.is_full() {
            return;
        }
        let size_mb = (data.len() as u64).div_ceil(1_048_576);
        self.current_size_mb += size_mb;
        self.store.insert(key.to_string(), data);
    }
}
/// Full configuration for a distributed build session.
#[derive(Clone, Debug)]
pub struct DistributedBuildConfig {
    /// Scheduling strategy.
    pub scheduler: JobSchedulerKind,
    /// Fault-tolerance settings.
    pub ft: FaultTolerance,
    /// Whether to enable the distributed remote cache.
    pub use_remote_cache: bool,
    /// Maximum concurrent jobs globally.
    pub max_global_jobs: usize,
    /// Whether to print verbose scheduler decisions.
    pub verbose: bool,
}
impl DistributedBuildConfig {
    /// Enable verbose output.
    pub fn with_verbose(mut self) -> Self {
        self.verbose = true;
        self
    }
    /// Set the scheduling strategy.
    pub fn with_scheduler(mut self, kind: JobSchedulerKind) -> Self {
        self.scheduler = kind;
        self
    }
}
/// A batch of distributed tasks to execute together.
#[derive(Clone, Debug)]
pub struct JobBatch {
    /// Batch ID.
    pub id: String,
    /// Tasks in this batch.
    pub tasks: Vec<DistributedTask>,
    /// Minimum workers required for this batch.
    pub min_workers: usize,
}
impl JobBatch {
    /// Create a new batch.
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            tasks: Vec::new(),
            min_workers: 1,
        }
    }
    /// Add a task to the batch.
    pub fn add_task(&mut self, task: DistributedTask) {
        self.tasks.push(task);
    }
    /// Number of tasks.
    pub fn len(&self) -> usize {
        self.tasks.len()
    }
    /// Whether empty.
    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }
    /// Total priority across all tasks.
    pub fn total_priority(&self) -> u32 {
        self.tasks.iter().map(|t| t.priority).sum()
    }
}
/// Configuration for the distributed build cache layer.
#[derive(Clone, Debug)]
pub struct DistributedCacheConfig {
    /// Whether to use the distributed cache.
    pub enabled: bool,
    /// Remote cache endpoint URL.
    pub endpoint: String,
    /// Maximum artifact size in megabytes.
    pub max_artifact_mb: u64,
    /// Cache read timeout in seconds.
    pub read_timeout_secs: u64,
    /// Cache write timeout in seconds.
    pub write_timeout_secs: u64,
}
impl DistributedCacheConfig {
    /// Create with a specific endpoint.
    pub fn with_endpoint(endpoint: &str) -> Self {
        Self {
            endpoint: endpoint.to_string(),
            ..Self::default()
        }
    }
    /// Whether the config is usable (enabled + non-empty endpoint).
    pub fn is_usable(&self) -> bool {
        self.enabled && !self.endpoint.is_empty()
    }
}
/// Tracks the capacity and current load of each worker in a matrix format.
pub struct WorkerCapacityMatrix {
    /// worker_id → (capacity, current_jobs).
    data: std::collections::HashMap<String, (u32, u32)>,
}
impl WorkerCapacityMatrix {
    /// Create an empty matrix.
    pub fn new() -> Self {
        Self {
            data: std::collections::HashMap::new(),
        }
    }
    /// Add or update a worker entry.
    pub fn upsert(&mut self, worker_id: &str, capacity: u32, current_jobs: u32) {
        self.data
            .insert(worker_id.to_string(), (capacity, current_jobs));
    }
    /// Utilization for a worker (0.0–1.0).
    pub fn utilization(&self, worker_id: &str) -> f64 {
        match self.data.get(worker_id) {
            Some(&(cap, cur)) if cap > 0 => cur as f64 / cap as f64,
            _ => 0.0,
        }
    }
    /// Worker with the lowest utilization.
    pub fn least_loaded(&self) -> Option<&str> {
        self.data
            .iter()
            .min_by(|(_, (ca, ja)), (_, (cb, jb))| {
                let ua = *ja as f64 / (*ca).max(1) as f64;
                let ub = *jb as f64 / (*cb).max(1) as f64;
                ua.partial_cmp(&ub).unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(id, _)| id.as_str())
    }
    /// Total capacity across all workers.
    pub fn total_capacity(&self) -> u32 {
        self.data.values().map(|(c, _)| c).sum()
    }
    /// Total current jobs across all workers.
    pub fn total_jobs(&self) -> u32 {
        self.data.values().map(|(_, j)| j).sum()
    }
}
/// Priority queue for distributed tasks (highest priority first).
pub struct TaskPriorityQueue {
    heap: std::collections::BinaryHeap<(u32, DistributedTask)>,
}
impl TaskPriorityQueue {
    /// Create an empty queue.
    pub fn new() -> Self {
        Self {
            heap: std::collections::BinaryHeap::new(),
        }
    }
    /// Push a task.
    pub fn push(&mut self, task: DistributedTask) {
        let pri = task.priority;
        self.heap.push((pri, task));
    }
    /// Pop the highest-priority task.
    pub fn pop(&mut self) -> Option<DistributedTask> {
        self.heap.pop().map(|(_, t)| t)
    }
    /// Number of queued tasks.
    pub fn len(&self) -> usize {
        self.heap.len()
    }
    /// Whether empty.
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }
}
/// Health summary for a single worker node.
#[derive(Clone, Debug)]
pub struct NodeHealthReport {
    /// Worker ID.
    pub worker_id: String,
    /// Whether the node appears healthy.
    pub healthy: bool,
    /// CPU usage estimate (0.0–1.0).
    pub cpu_usage: f64,
    /// Memory usage estimate (0.0–1.0).
    pub mem_usage: f64,
    /// Network latency estimate in milliseconds.
    pub latency_ms: u64,
}
impl NodeHealthReport {
    /// Create a healthy report with all fields.
    pub fn healthy(worker_id: &str, cpu: f64, mem: f64, latency_ms: u64) -> Self {
        Self {
            worker_id: worker_id.to_string(),
            healthy: true,
            cpu_usage: cpu.clamp(0.0, 1.0),
            mem_usage: mem.clamp(0.0, 1.0),
            latency_ms,
        }
    }
    /// Create an unhealthy (dead) report.
    pub fn dead(worker_id: &str) -> Self {
        Self {
            worker_id: worker_id.to_string(),
            healthy: false,
            cpu_usage: 0.0,
            mem_usage: 0.0,
            latency_ms: u64::MAX,
        }
    }
    /// Whether the node is considered overloaded (CPU or mem > 90%).
    pub fn is_overloaded(&self) -> bool {
        self.cpu_usage > 0.9 || self.mem_usage > 0.9
    }
}
/// A plan for distributing a set of tasks across a cluster.
pub struct DistributedBuildPlan {
    /// Ordered batches of task IDs (each batch can run in parallel).
    pub batches: Vec<Vec<String>>,
}
impl DistributedBuildPlan {
    /// Create an empty plan.
    pub fn new() -> Self {
        Self {
            batches: Vec::new(),
        }
    }
    /// Add a batch.
    pub fn add_batch(&mut self, task_ids: Vec<String>) {
        if !task_ids.is_empty() {
            self.batches.push(task_ids);
        }
    }
    /// Total number of tasks across all batches.
    pub fn total_tasks(&self) -> usize {
        self.batches.iter().map(|b| b.len()).sum()
    }
    /// Number of batches.
    pub fn batch_count(&self) -> usize {
        self.batches.len()
    }
    /// Maximum batch size.
    pub fn max_batch_size(&self) -> usize {
        self.batches.iter().map(|b| b.len()).max().unwrap_or(0)
    }
}
/// Tracks task dependencies to determine which tasks are ready to run.
pub struct TaskDependencyTracker {
    /// task_id → set of dependency task IDs not yet complete.
    pending_deps: std::collections::HashMap<String, std::collections::HashSet<String>>,
    /// task_id → task.
    all_tasks: std::collections::HashMap<String, DistributedTask>,
}
impl TaskDependencyTracker {
    /// Create a tracker from a list of tasks.
    pub fn new(tasks: Vec<DistributedTask>) -> Self {
        let mut pending_deps = std::collections::HashMap::new();
        let mut all_tasks = std::collections::HashMap::new();
        for task in tasks {
            let deps: std::collections::HashSet<String> =
                task.dependencies.iter().cloned().collect();
            pending_deps.insert(task.id.clone(), deps);
            all_tasks.insert(task.id.clone(), task);
        }
        Self {
            pending_deps,
            all_tasks,
        }
    }
    /// Mark `task_id` as complete; returns newly-ready task IDs.
    pub fn mark_complete(&mut self, task_id: &str) -> Vec<String> {
        self.all_tasks.remove(task_id);
        self.pending_deps.remove(task_id);
        let mut ready = Vec::new();
        for (id, deps) in &mut self.pending_deps {
            deps.remove(task_id);
            if deps.is_empty() {
                ready.push(id.clone());
            }
        }
        for id in &ready {
            self.pending_deps.remove(id);
        }
        ready
    }
    /// Tasks whose dependencies are all satisfied (ready to run).
    pub fn ready_tasks(&self) -> Vec<&DistributedTask> {
        self.pending_deps
            .iter()
            .filter(|(_, deps)| deps.is_empty())
            .filter_map(|(id, _)| self.all_tasks.get(id))
            .collect()
    }
    /// Number of tasks not yet completed.
    pub fn remaining(&self) -> usize {
        self.all_tasks.len()
    }
}
/// Statistics collected during a distributed build.
#[derive(Clone, Debug, Default)]
pub struct DistributedBuildStats {
    /// Total tasks submitted.
    pub tasks_submitted: u64,
    /// Tasks completed successfully.
    pub tasks_succeeded: u64,
    /// Tasks that failed.
    pub tasks_failed: u64,
    /// Tasks retried.
    pub tasks_retried: u64,
    /// Total wall-clock time in milliseconds.
    pub wall_ms: u64,
    /// Peak number of concurrent workers used.
    pub peak_workers: u64,
}
impl DistributedBuildStats {
    /// Create zeroed stats.
    pub fn new() -> Self {
        Self::default()
    }
    /// Task success rate.
    pub fn success_rate(&self) -> f64 {
        if self.tasks_submitted == 0 {
            1.0
        } else {
            self.tasks_succeeded as f64 / self.tasks_submitted as f64
        }
    }
    /// Human-readable summary.
    pub fn summary(&self) -> String {
        format!(
            "submitted={} succeeded={} failed={} retried={} wall={}ms",
            self.tasks_submitted,
            self.tasks_succeeded,
            self.tasks_failed,
            self.tasks_retried,
            self.wall_ms,
        )
    }
}
/// Summarizes health across an entire cluster.
#[derive(Clone, Debug, Default)]
pub struct ClusterHealthSummary {
    /// Total nodes in the cluster.
    pub total_nodes: usize,
    /// Healthy nodes.
    pub healthy_nodes: usize,
    /// Overloaded nodes.
    pub overloaded_nodes: usize,
    /// Dead nodes.
    pub dead_nodes: usize,
    /// Average latency across healthy nodes (ms).
    pub avg_latency_ms: f64,
}
impl ClusterHealthSummary {
    /// Build a summary from a list of reports.
    pub fn from_reports(reports: &[NodeHealthReport]) -> Self {
        let total_nodes = reports.len();
        let healthy_nodes = reports.iter().filter(|r| r.healthy).count();
        let dead_nodes = reports.iter().filter(|r| !r.healthy).count();
        let overloaded_nodes = reports.iter().filter(|r| r.is_overloaded()).count();
        let healthy_latencies: Vec<u64> = reports
            .iter()
            .filter(|r| r.healthy)
            .map(|r| r.latency_ms)
            .collect();
        let avg_latency_ms = if healthy_latencies.is_empty() {
            0.0
        } else {
            healthy_latencies.iter().sum::<u64>() as f64 / healthy_latencies.len() as f64
        };
        Self {
            total_nodes,
            healthy_nodes,
            overloaded_nodes,
            dead_nodes,
            avg_latency_ms,
        }
    }
    /// Cluster health percentage.
    pub fn health_pct(&self) -> f64 {
        if self.total_nodes == 0 {
            100.0
        } else {
            (self.healthy_nodes as f64 / self.total_nodes as f64) * 100.0
        }
    }
}
/// Log of all task outcomes in a distributed build session.
pub struct DistributedTaskLog {
    entries: Vec<(String, String, TaskResult)>,
}
impl DistributedTaskLog {
    /// Create an empty log.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
    /// Record an outcome.
    pub fn record(&mut self, task_id: &str, worker_id: &str, result: TaskResult) {
        self.entries
            .push((task_id.to_string(), worker_id.to_string(), result));
    }
    /// Number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Whether empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Count successful tasks.
    pub fn success_count(&self) -> usize {
        self.entries
            .iter()
            .filter(|(_, _, r)| r.is_success())
            .count()
    }
    /// Count failed tasks.
    pub fn failure_count(&self) -> usize {
        self.entries
            .iter()
            .filter(|(_, _, r)| matches!(r, TaskResult::Failure(_)))
            .count()
    }
    /// Success rate.
    pub fn success_rate(&self) -> f64 {
        if self.entries.is_empty() {
            1.0
        } else {
            self.success_count() as f64 / self.entries.len() as f64
        }
    }
}
/// The outcome of a distributed task.
#[derive(Clone, Debug)]
pub enum TaskResult {
    /// Task succeeded; artifact bytes attached.
    Success(Vec<u8>),
    /// Task failed with an error message.
    Failure(String),
    /// Task timed out.
    Timeout,
    /// Task was cancelled.
    Cancelled,
}
impl TaskResult {
    /// Whether the result is a success.
    pub fn is_success(&self) -> bool {
        matches!(self, TaskResult::Success(_))
    }
    /// Short status label.
    pub fn status_label(&self) -> &'static str {
        match self {
            TaskResult::Success(_) => "success",
            TaskResult::Failure(_) => "failure",
            TaskResult::Timeout => "timeout",
            TaskResult::Cancelled => "cancelled",
        }
    }
    /// Error message (if any).
    pub fn error_message(&self) -> Option<&str> {
        if let TaskResult::Failure(msg) = self {
            Some(msg.as_str())
        } else {
            None
        }
    }
}
/// A simple FIFO work-stealing queue (single-deque stub).
pub struct WorkStealingQueue {
    local: std::collections::VecDeque<DistributedTask>,
}
impl WorkStealingQueue {
    /// Create an empty queue.
    pub fn new() -> Self {
        Self {
            local: std::collections::VecDeque::new(),
        }
    }
    /// Push a task locally.
    pub fn push(&mut self, task: DistributedTask) {
        self.local.push_back(task);
    }
    /// Pop from the local end.
    pub fn pop_local(&mut self) -> Option<DistributedTask> {
        self.local.pop_back()
    }
    /// "Steal" from the remote end.
    pub fn steal(&mut self) -> Option<DistributedTask> {
        self.local.pop_front()
    }
    /// Number of tasks.
    pub fn len(&self) -> usize {
        self.local.len()
    }
    /// Whether empty.
    pub fn is_empty(&self) -> bool {
        self.local.is_empty()
    }
}
/// Monitors worker heartbeats to detect failures.
pub struct HeartbeatMonitor {
    /// worker_id → last heartbeat timestamp (seconds).
    last_seen: std::collections::HashMap<String, u64>,
    /// Timeout threshold in seconds.
    pub timeout_secs: u64,
}
impl HeartbeatMonitor {
    /// Create a monitor with the given timeout.
    pub fn new(timeout_secs: u64) -> Self {
        Self {
            last_seen: std::collections::HashMap::new(),
            timeout_secs,
        }
    }
    /// Record a heartbeat from a worker.
    pub fn record(&mut self, worker_id: &str, ts: u64) {
        self.last_seen.insert(worker_id.to_string(), ts);
    }
    /// Check if a worker is considered dead (no heartbeat within timeout).
    pub fn is_dead(&self, worker_id: &str, now_ts: u64) -> bool {
        match self.last_seen.get(worker_id) {
            Some(&last) => now_ts.saturating_sub(last) > self.timeout_secs,
            None => true,
        }
    }
    /// Return IDs of all workers that appear dead.
    pub fn dead_workers(&self, now_ts: u64) -> Vec<&str> {
        self.last_seen
            .iter()
            .filter(|(_, &ts)| now_ts.saturating_sub(ts) > self.timeout_secs)
            .map(|(id, _)| id.as_str())
            .collect()
    }
    /// Number of known workers.
    pub fn worker_count(&self) -> usize {
        self.last_seen.len()
    }
}
/// Tracks retry state for distributed tasks.
pub struct RetryManager {
    /// task_id → attempt count.
    attempts: std::collections::HashMap<String, u32>,
    pub max_retries: u32,
}
impl RetryManager {
    /// Create a retry manager with the given limit.
    pub fn new(max_retries: u32) -> Self {
        Self {
            attempts: std::collections::HashMap::new(),
            max_retries,
        }
    }
    /// Record an attempt for `task_id`. Returns `true` if another retry is allowed.
    pub fn record_attempt(&mut self, task_id: &str) -> bool {
        let count = self.attempts.entry(task_id.to_string()).or_insert(0);
        *count += 1;
        *count <= self.max_retries
    }
    /// Number of attempts made for `task_id`.
    pub fn attempt_count(&self, task_id: &str) -> u32 {
        *self.attempts.get(task_id).unwrap_or(&0)
    }
    /// Whether `task_id` has exceeded its retry budget.
    pub fn exhausted(&self, task_id: &str) -> bool {
        self.attempt_count(task_id) > self.max_retries
    }
    /// Reset state for `task_id`.
    pub fn reset(&mut self, task_id: &str) {
        self.attempts.remove(task_id);
    }
    /// Clear all state.
    pub fn clear(&mut self) {
        self.attempts.clear();
    }
}
/// Per-worker metrics for a distributed build.
#[derive(Clone, Debug, Default)]
pub struct WorkerMetrics {
    /// Worker ID.
    pub worker_id: String,
    /// Total tasks completed.
    pub tasks_completed: u64,
    /// Total tasks failed.
    pub tasks_failed: u64,
    /// Total compute time in milliseconds.
    pub compute_ms: u64,
    /// Number of times this worker was idle (no tasks available).
    pub idle_cycles: u64,
}
impl WorkerMetrics {
    /// Create zeroed metrics for a worker.
    pub fn new(worker_id: &str) -> Self {
        Self {
            worker_id: worker_id.to_string(),
            ..Self::default()
        }
    }
    /// Record a completed task.
    pub fn record_success(&mut self, elapsed_ms: u64) {
        self.tasks_completed += 1;
        self.compute_ms += elapsed_ms;
    }
    /// Record a failed task.
    pub fn record_failure(&mut self) {
        self.tasks_failed += 1;
    }
    /// Success rate.
    pub fn success_rate(&self) -> f64 {
        let total = self.tasks_completed + self.tasks_failed;
        if total == 0 {
            1.0
        } else {
            self.tasks_completed as f64 / total as f64
        }
    }
    /// Average task time in milliseconds.
    pub fn avg_task_ms(&self) -> f64 {
        if self.tasks_completed == 0 {
            0.0
        } else {
            self.compute_ms as f64 / self.tasks_completed as f64
        }
    }
}
/// High-level orchestrator for distributed builds.
pub struct DistributedBuildOrchestrator {
    pub registry: WorkerRegistry,
    pub batches: Vec<JobBatch>,
    pub stats: DistributedBuildStats,
    pub config: DistributedBuildConfig,
}
impl DistributedBuildOrchestrator {
    /// Create a new orchestrator.
    pub fn new(config: DistributedBuildConfig) -> Self {
        Self {
            registry: WorkerRegistry::new(),
            batches: Vec::new(),
            stats: DistributedBuildStats::new(),
            config,
        }
    }
    /// Register a worker.
    pub fn register_worker(&mut self, worker: RemoteWorker) {
        self.registry.register(worker);
    }
    /// Add a batch of work.
    pub fn add_batch(&mut self, batch: JobBatch) {
        self.stats.tasks_submitted += batch.len() as u64;
        self.batches.push(batch);
    }
    /// Simulate execution: mark all tasks as succeeded.
    pub fn run_all(&mut self) {
        let workers: Vec<String> = self
            .registry
            .worker_ids()
            .iter()
            .map(|s| s.to_string())
            .collect();
        let default_worker = "local".to_string();
        let worker_id = workers.first().unwrap_or(&default_worker).clone();
        for batch in &self.batches {
            for _task in &batch.tasks {
                self.registry.record_success(&worker_id, 50);
                self.stats.tasks_succeeded += 1;
            }
        }
    }
    /// Total task count.
    pub fn total_tasks(&self) -> usize {
        self.batches.iter().map(|b| b.len()).sum()
    }
    /// Summary.
    pub fn summary(&self) -> String {
        format!(
            "orchestrator: workers={} batches={} tasks={} {}",
            self.registry.count(),
            self.batches.len(),
            self.total_tasks(),
            self.stats.summary(),
        )
    }
}
/// A full distributed build session, tying together all subsystems.
pub struct DistributedSession {
    pub config: DistributedBuildConfig,
    pub coordinator: DistributedCoordinator,
    pub pool: WorkerPool,
    pub stats: DistributedBuildStats,
    pub task_log: DistributedTaskLog,
    pub heartbeat: HeartbeatMonitor,
}
impl DistributedSession {
    /// Create a session with the given config.
    pub fn new(config: DistributedBuildConfig) -> Self {
        Self {
            heartbeat: HeartbeatMonitor::new(config.ft.task_timeout_secs),
            config,
            coordinator: DistributedCoordinator::new(),
            pool: WorkerPool::new(),
            stats: DistributedBuildStats::new(),
            task_log: DistributedTaskLog::new(),
        }
    }
    /// Register a worker with both the coordinator and the pool.
    pub fn add_worker(&mut self, worker: RemoteWorker) {
        self.coordinator.add_worker(worker.clone());
        self.pool.add(worker);
    }
    /// Submit a task.
    pub fn submit(&mut self, task: DistributedTask) {
        self.stats.tasks_submitted += 1;
        self.coordinator.submit_task(task);
    }
    /// Assign and run the next task (stub: always succeeds).
    pub fn step(&mut self) -> bool {
        if let Some((wid, task)) = self.coordinator.assign_next() {
            self.heartbeat.record(&wid, 0);
            let result = TaskResult::Success(b"artifact".to_vec());
            self.stats.tasks_succeeded += 1;
            self.task_log.record(&task.id, &wid, result);
            self.coordinator.mark_complete(&task.id, &wid);
            true
        } else {
            false
        }
    }
    /// Run all pending tasks in a loop.
    pub fn run_all(&mut self) {
        while self.step() {}
    }
    /// Whether all work is done.
    pub fn is_done(&self) -> bool {
        self.coordinator.pending_count() == 0
    }
}
/// Aggregates partial build results as they arrive from remote workers.
pub struct ResultAggregator {
    /// task_id → artifact bytes.
    results: std::collections::HashMap<String, Vec<u8>>,
    /// IDs of tasks that failed.
    failures: Vec<String>,
    /// IDs of tasks pending.
    pending: std::collections::HashSet<String>,
}
impl ResultAggregator {
    /// Create an empty aggregator; register expected tasks.
    pub fn new(task_ids: &[&str]) -> Self {
        let mut pending = std::collections::HashSet::new();
        for &id in task_ids {
            pending.insert(id.to_string());
        }
        Self {
            results: std::collections::HashMap::new(),
            failures: Vec::new(),
            pending,
        }
    }
    /// Record a successful result.
    pub fn record_success(&mut self, task_id: &str, artifact: Vec<u8>) {
        self.pending.remove(task_id);
        self.results.insert(task_id.to_string(), artifact);
    }
    /// Record a failure.
    pub fn record_failure(&mut self, task_id: &str) {
        self.pending.remove(task_id);
        self.failures.push(task_id.to_string());
    }
    /// Whether all tasks are done (none pending).
    pub fn is_complete(&self) -> bool {
        self.pending.is_empty()
    }
    /// Number of successful results.
    pub fn success_count(&self) -> usize {
        self.results.len()
    }
    /// Number of failures.
    pub fn failure_count(&self) -> usize {
        self.failures.len()
    }
    /// Remaining pending count.
    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }
    /// Retrieve artifact by task ID.
    pub fn get_artifact(&self, task_id: &str) -> Option<&Vec<u8>> {
        self.results.get(task_id)
    }
}
/// Least-loaded strategy: pick the worker with the lowest utilization.
pub struct LeastLoadedStrategy;
/// Estimates network bandwidth between the coordinator and workers.
pub struct NetworkBandwidthEstimator {
    /// worker_id → estimated bandwidth in bytes/second.
    estimates: std::collections::HashMap<String, f64>,
}
impl NetworkBandwidthEstimator {
    /// Create an empty estimator.
    pub fn new() -> Self {
        Self {
            estimates: std::collections::HashMap::new(),
        }
    }
    /// Record an observed transfer: `bytes` transferred in `elapsed_ms` ms.
    pub fn record(&mut self, worker_id: &str, bytes: u64, elapsed_ms: u64) {
        if elapsed_ms == 0 {
            return;
        }
        let bps = bytes as f64 / (elapsed_ms as f64 / 1000.0);
        let entry = self.estimates.entry(worker_id.to_string()).or_insert(bps);
        *entry = 0.7 * (*entry) + 0.3 * bps;
    }
    /// Estimated bandwidth for `worker_id` in bytes/second (None if unknown).
    pub fn estimate(&self, worker_id: &str) -> Option<f64> {
        self.estimates.get(worker_id).copied()
    }
    /// Estimated time in milliseconds to transfer `bytes` to `worker_id`.
    pub fn transfer_time_ms(&self, worker_id: &str, bytes: u64) -> Option<u64> {
        self.estimates.get(worker_id).map(|&bps| {
            if bps <= 0.0 {
                u64::MAX
            } else {
                ((bytes as f64 / bps) * 1000.0) as u64
            }
        })
    }
}
/// A task to distribute to a remote worker
#[derive(Debug, Clone)]
pub struct DistributedTask {
    pub id: String,
    pub file_path: String,
    pub dependencies: Vec<String>,
    pub priority: u32,
}
/// Coordinates work across multiple remote workers
pub struct DistributedCoordinator {
    workers: Vec<RemoteWorker>,
    pending: Vec<DistributedTask>,
    completed: Vec<String>,
}
impl DistributedCoordinator {
    pub fn new() -> Self {
        DistributedCoordinator {
            workers: Vec::new(),
            pending: Vec::new(),
            completed: Vec::new(),
        }
    }
    pub fn add_worker(&mut self, worker: RemoteWorker) {
        self.workers.push(worker);
    }
    pub fn submit_task(&mut self, task: DistributedTask) {
        self.pending.push(task);
    }
    /// Assign the highest-priority pending task to the least-loaded available worker.
    /// Returns `(worker_id, task)` if an assignment was made.
    pub fn assign_next(&mut self) -> Option<(String, DistributedTask)> {
        let worker_idx = self
            .workers
            .iter()
            .enumerate()
            .filter(|(_, w)| w.is_available())
            .min_by(|(_, a), (_, b)| {
                a.utilization()
                    .partial_cmp(&b.utilization())
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(i, _)| i)?;
        if self.pending.is_empty() {
            return None;
        }
        let task_idx = self
            .pending
            .iter()
            .enumerate()
            .max_by_key(|(_, t)| t.priority)
            .map(|(i, _)| i)?;
        let task = self.pending.remove(task_idx);
        self.workers[worker_idx].current_jobs += 1;
        let worker_id = self.workers[worker_idx].id.clone();
        Some((worker_id, task))
    }
    /// Mark a task as complete and decrement the worker's job count.
    pub fn mark_complete(&mut self, task_id: &str, worker_id: &str) {
        self.completed.push(task_id.to_string());
        if let Some(w) = self.workers.iter_mut().find(|w| w.id == worker_id) {
            if w.current_jobs > 0 {
                w.current_jobs -= 1;
            }
        }
    }
    pub fn available_workers(&self) -> Vec<&RemoteWorker> {
        self.workers.iter().filter(|w| w.is_available()).collect()
    }
    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }
    pub fn completed_count(&self) -> usize {
        self.completed.len()
    }
    /// Average utilization across all workers (0.0 if no workers)
    pub fn utilization(&self) -> f64 {
        if self.workers.is_empty() {
            return 0.0;
        }
        let sum: f64 = self.workers.iter().map(|w| w.utilization()).sum();
        sum / self.workers.len() as f64
    }
}
/// Registry of all known workers and their metrics.
pub struct WorkerRegistry {
    workers: std::collections::HashMap<String, RemoteWorker>,
    metrics: std::collections::HashMap<String, WorkerMetrics>,
}
impl WorkerRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self {
            workers: std::collections::HashMap::new(),
            metrics: std::collections::HashMap::new(),
        }
    }
    /// Register a worker.
    pub fn register(&mut self, worker: RemoteWorker) {
        let id = worker.id.clone();
        self.metrics.insert(id.clone(), WorkerMetrics::new(&id));
        self.workers.insert(id, worker);
    }
    /// Unregister a worker.
    pub fn unregister(&mut self, id: &str) {
        self.workers.remove(id);
        self.metrics.remove(id);
    }
    /// Get a worker by ID.
    pub fn get(&self, id: &str) -> Option<&RemoteWorker> {
        self.workers.get(id)
    }
    /// Get metrics for a worker.
    pub fn metrics_for(&self, id: &str) -> Option<&WorkerMetrics> {
        self.metrics.get(id)
    }
    /// Record a task success for a worker.
    pub fn record_success(&mut self, worker_id: &str, elapsed_ms: u64) {
        if let Some(m) = self.metrics.get_mut(worker_id) {
            m.record_success(elapsed_ms);
        }
    }
    /// Record a task failure for a worker.
    pub fn record_failure(&mut self, worker_id: &str) {
        if let Some(m) = self.metrics.get_mut(worker_id) {
            m.record_failure();
        }
    }
    /// Number of registered workers.
    pub fn count(&self) -> usize {
        self.workers.len()
    }
    /// All registered worker IDs.
    pub fn worker_ids(&self) -> Vec<&str> {
        self.workers.keys().map(|k| k.as_str()).collect()
    }
    /// The worker with the highest task count.
    pub fn busiest_worker(&self) -> Option<&str> {
        self.metrics
            .iter()
            .max_by_key(|(_, m)| m.tasks_completed)
            .map(|(id, _)| id.as_str())
    }
}
/// Snapshot of task states across a distributed build.
#[derive(Clone, Debug, Default)]
pub struct TaskStateSnapshot {
    pub pending: usize,
    pub running: usize,
    pub done: usize,
    pub failed: usize,
    pub cancelled: usize,
}
impl TaskStateSnapshot {
    /// Create from counts.
    pub fn new(
        pending: usize,
        running: usize,
        done: usize,
        failed: usize,
        cancelled: usize,
    ) -> Self {
        Self {
            pending,
            running,
            done,
            failed,
            cancelled,
        }
    }
    /// Total tasks in the snapshot.
    pub fn total(&self) -> usize {
        self.pending + self.running + self.done + self.failed + self.cancelled
    }
    /// Whether all tasks are complete (done, failed, or cancelled).
    pub fn all_finished(&self) -> bool {
        self.pending == 0 && self.running == 0
    }
}
/// Random strategy: pick a random available worker (deterministic stub using hash).
pub struct RandomStrategy {
    pub seed: u64,
}
impl RandomStrategy {
    /// Create with a fixed seed.
    pub fn new(seed: u64) -> Self {
        Self { seed }
    }
}
/// Lifecycle state of a distributed task.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DistributedTaskState {
    /// Waiting to be assigned.
    Pending,
    /// Assigned to a worker, executing.
    Running,
    /// Completed successfully.
    Done,
    /// Failed after all retries.
    Failed,
    /// Cancelled by coordinator.
    Cancelled,
}
/// Strategy for distributing tasks to workers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JobSchedulerKind {
    /// Assign to the worker with the lowest current load.
    LeastLoaded,
    /// Simple round-robin among available workers.
    RoundRobin,
    /// Priority-weighted assignment.
    PriorityWeighted,
}
