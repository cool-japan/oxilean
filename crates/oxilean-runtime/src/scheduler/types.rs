//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::object::RtObject;
use std::collections::{HashMap, VecDeque};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

/// A message sent to an actor.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ActorMessage {
    /// Sender actor.
    pub from: ActorId,
    /// Recipient actor.
    pub to: ActorId,
    /// The message payload.
    pub payload: RtObject,
    /// Sequence number.
    pub seq: u64,
}
#[allow(dead_code)]
impl ActorMessage {
    /// Create a new message.
    pub fn new(from: ActorId, to: ActorId, payload: RtObject, seq: u64) -> Self {
        ActorMessage {
            from,
            to,
            payload,
            seq,
        }
    }
}
/// Simple round-robin token for fair scheduling.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RoundRobinToken {
    slots: usize,
    current: usize,
}
#[allow(dead_code)]
impl RoundRobinToken {
    pub fn new(slots: usize) -> Self {
        assert!(slots > 0, "slots must be > 0");
        Self { slots, current: 0 }
    }
    /// Advance to the next slot and return it.
    pub fn next(&mut self) -> usize {
        let slot = self.current;
        self.current = (self.current + 1) % self.slots;
        slot
    }
    /// Peek at the current slot without advancing.
    pub fn peek(&self) -> usize {
        self.current
    }
    /// Reset back to slot 0.
    pub fn reset(&mut self) {
        self.current = 0;
    }
}
/// A deterministic harness for testing the scheduler without real threads.
#[allow(dead_code)]
pub struct SchedulerTestHarness {
    /// Tasks submitted.
    pub tasks: Vec<(TaskId, RtObject)>,
    /// Execution order recorded.
    pub execution_order: Vec<TaskId>,
    /// Results produced.
    pub results: HashMap<TaskId, RtObject>,
    /// Next task id.
    next_id: u64,
}
#[allow(dead_code)]
impl SchedulerTestHarness {
    /// Create a new harness.
    pub fn new() -> Self {
        SchedulerTestHarness {
            tasks: Vec::new(),
            execution_order: Vec::new(),
            results: HashMap::new(),
            next_id: 0,
        }
    }
    /// Submit a task.
    pub fn submit(&mut self, action: RtObject) -> TaskId {
        let id = TaskId::new(self.next_id);
        self.next_id += 1;
        self.tasks.push((id, action));
        id
    }
    /// Run all submitted tasks with a given handler.
    pub fn run_all<F: FnMut(&RtObject) -> RtObject>(&mut self, mut f: F) {
        let tasks = std::mem::take(&mut self.tasks);
        for (id, action) in tasks {
            let result = f(&action);
            self.execution_order.push(id);
            self.results.insert(id, result);
        }
    }
    /// Get the result of a task.
    pub fn get_result(&self, id: TaskId) -> Option<&RtObject> {
        self.results.get(&id)
    }
    /// Number of tasks completed.
    pub fn completed(&self) -> usize {
        self.results.len()
    }
    /// Reset the harness.
    pub fn reset(&mut self) {
        self.tasks.clear();
        self.execution_order.clear();
        self.results.clear();
        self.next_id = 0;
    }
}
/// Affinity specification for a task.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TaskAffinity {
    /// No affinity — any worker can run this task.
    Any,
    /// Pinned to a specific worker index.
    Worker(usize),
    /// Prefer a worker but allow stealing.
    Prefer(usize),
    /// Run only on the main thread (worker 0).
    MainThread,
}
#[allow(dead_code)]
impl TaskAffinity {
    /// Check if a given worker satisfies this affinity.
    pub fn allows(&self, worker: usize) -> bool {
        match self {
            TaskAffinity::Any => true,
            TaskAffinity::Worker(w) => *w == worker,
            TaskAffinity::Prefer(w) => *w == worker,
            TaskAffinity::MainThread => worker == 0,
        }
    }
    /// Whether this is a hard pin (only one worker may run it).
    pub fn is_pinned(&self) -> bool {
        matches!(self, TaskAffinity::Worker(_) | TaskAffinity::MainThread)
    }
    /// Whether stealing is allowed for this affinity.
    pub fn allows_steal(&self) -> bool {
        matches!(self, TaskAffinity::Any | TaskAffinity::Prefer(_))
    }
}
/// A task queue that maintains separate buckets per priority level.
#[allow(dead_code)]
pub struct PriorityTaskQueue {
    /// Buckets indexed by priority value (0..=4).
    buckets: [VecDeque<TaskId>; 5],
    /// Total tasks across all buckets.
    total: usize,
}
#[allow(dead_code)]
impl PriorityTaskQueue {
    /// Create an empty priority queue.
    pub fn new() -> Self {
        PriorityTaskQueue {
            buckets: [
                VecDeque::new(),
                VecDeque::new(),
                VecDeque::new(),
                VecDeque::new(),
                VecDeque::new(),
            ],
            total: 0,
        }
    }
    /// Push a task with a given priority.
    pub fn push(&mut self, id: TaskId, priority: TaskPriority) {
        self.buckets[priority.value() as usize].push_back(id);
        self.total += 1;
    }
    /// Pop the highest-priority pending task.
    pub fn pop(&mut self) -> Option<(TaskId, TaskPriority)> {
        for level in (0..5).rev() {
            if let Some(id) = self.buckets[level].pop_front() {
                self.total -= 1;
                return Some((id, TaskPriority::from_u8(level as u8)));
            }
        }
        None
    }
    /// Total number of tasks.
    pub fn len(&self) -> usize {
        self.total
    }
    /// Whether the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.total == 0
    }
    /// Number of tasks at a given priority.
    pub fn count_at(&self, priority: TaskPriority) -> usize {
        self.buckets[priority.value() as usize].len()
    }
    /// Clear all tasks.
    pub fn clear(&mut self) {
        for bucket in &mut self.buckets {
            bucket.clear();
        }
        self.total = 0;
    }
}
/// A task represents a unit of work to be executed.
#[derive(Clone, Debug)]
pub struct Task {
    /// Unique task identifier.
    pub id: TaskId,
    /// Human-readable name (for debugging).
    pub name: Option<String>,
    /// Priority.
    pub priority: TaskPriority,
    /// Current state.
    pub state: TaskState,
    /// The closure to execute (represented as an RtObject).
    pub action: RtObject,
    /// Dependencies (tasks that must complete before this one).
    pub dependencies: Vec<TaskId>,
    /// Tasks that depend on this one (notified on completion).
    pub dependents: Vec<TaskId>,
    /// Creation timestamp (nanoseconds).
    pub created_at: u64,
    /// Completion timestamp (nanoseconds).
    pub completed_at: Option<u64>,
}
impl Task {
    /// Create a new task.
    pub fn new(id: TaskId, action: RtObject) -> Self {
        Task {
            id,
            name: None,
            priority: TaskPriority::Normal,
            state: TaskState::Created,
            action,
            dependencies: Vec::new(),
            dependents: Vec::new(),
            created_at: 0,
            completed_at: None,
        }
    }
    /// Create a named task.
    pub fn named(id: TaskId, name: String, action: RtObject) -> Self {
        Task {
            id,
            name: Some(name),
            priority: TaskPriority::Normal,
            state: TaskState::Created,
            action,
            dependencies: Vec::new(),
            dependents: Vec::new(),
            created_at: 0,
            completed_at: None,
        }
    }
    /// Set the priority.
    pub fn with_priority(mut self, priority: TaskPriority) -> Self {
        self.priority = priority;
        self
    }
    /// Add a dependency.
    pub fn depends_on(mut self, dep: TaskId) -> Self {
        self.dependencies.push(dep);
        self
    }
    /// Check if all dependencies are satisfied.
    pub fn dependencies_satisfied(&self, completed: &[TaskId]) -> bool {
        self.dependencies.iter().all(|dep| completed.contains(dep))
    }
    /// Mark as completed.
    pub fn complete(&mut self, result: RtObject) {
        self.state = TaskState::Completed { result };
    }
    /// Mark as failed.
    pub fn fail(&mut self, error: String) {
        self.state = TaskState::Failed { error };
    }
    /// Mark as cancelled.
    pub fn cancel(&mut self) {
        self.state = TaskState::Cancelled;
    }
    /// Get the result if completed.
    pub fn result(&self) -> Option<&RtObject> {
        if let TaskState::Completed { ref result } = self.state {
            Some(result)
        } else {
            None
        }
    }
    /// Get the error if failed.
    pub fn error(&self) -> Option<&str> {
        if let TaskState::Failed { ref error } = self.state {
            Some(error)
        } else {
            None
        }
    }
}
/// A unique actor identifier.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ActorId(pub u64);
#[allow(dead_code)]
impl ActorId {
    /// Create a new actor ID.
    pub fn new(id: u64) -> Self {
        ActorId(id)
    }
    /// Raw value.
    pub fn raw(self) -> u64 {
        self.0
    }
}
/// Represents a worker in the scheduler.
#[derive(Debug)]
pub struct Worker {
    /// Worker ID.
    pub id: usize,
    /// The worker's local task deque.
    pub deque: WorkStealingDeque,
    /// Number of tasks completed by this worker.
    pub tasks_completed: u64,
    /// Number of tasks stolen from this worker.
    pub tasks_stolen_from: u64,
    /// Number of tasks stolen by this worker.
    pub tasks_stolen: u64,
    /// Whether this worker is currently idle.
    pub idle: bool,
    /// Current task being executed (if any).
    pub current_task: Option<TaskId>,
}
impl Worker {
    /// Create a new worker.
    pub fn new(id: usize, deque_capacity: usize) -> Self {
        Worker {
            id,
            deque: WorkStealingDeque::new(deque_capacity),
            tasks_completed: 0,
            tasks_stolen_from: 0,
            tasks_stolen: 0,
            idle: true,
            current_task: None,
        }
    }
    /// Push a task to this worker's deque.
    pub fn push_task(&mut self, task_id: TaskId) -> bool {
        self.deque.push(task_id)
    }
    /// Pop a task from this worker's deque.
    pub fn pop_task(&mut self) -> Option<TaskId> {
        self.deque.pop()
    }
    /// Start executing a task.
    pub fn start_task(&mut self, task_id: TaskId) {
        self.current_task = Some(task_id);
        self.idle = false;
    }
    /// Finish executing the current task.
    pub fn finish_task(&mut self) {
        self.current_task = None;
        self.idle = true;
        self.tasks_completed += 1;
    }
    /// Load of this worker (number of queued + current tasks).
    pub fn load(&self) -> usize {
        self.deque.len() + if self.current_task.is_some() { 1 } else { 0 }
    }
}
/// Load balancing strategies.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoadBalanceStrategy {
    /// Round-robin assignment.
    RoundRobin,
    /// Assign to the least loaded worker.
    LeastLoaded,
    /// Random assignment.
    Random,
    /// Work stealing (tasks start local, idle workers steal).
    WorkStealing,
}
/// Worker statistics summary.
#[derive(Clone, Debug)]
pub struct WorkerStats {
    /// Worker ID.
    pub id: usize,
    /// Tasks completed.
    pub tasks_completed: u64,
    /// Tasks stolen by this worker.
    pub tasks_stolen: u64,
    /// Tasks stolen from this worker.
    pub tasks_stolen_from: u64,
    /// Current queue length.
    pub queue_length: usize,
    /// Whether idle.
    pub idle: bool,
}
/// A handle for requesting a yield from outside the task.
#[allow(dead_code)]
pub struct YieldHandle {
    requested: Arc<AtomicBool>,
}
#[allow(dead_code)]
impl YieldHandle {
    /// Request the task to yield at its next safe point.
    pub fn request(&self) {
        self.requested.store(true, Ordering::Release);
    }
    /// Check if a yield is pending.
    pub fn is_pending(&self) -> bool {
        self.requested.load(Ordering::Acquire)
    }
}
/// Extended scheduler statistics.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct ExtSchedulerStats {
    /// Total tasks created.
    pub tasks_created: u64,
    /// Total tasks completed.
    pub tasks_completed: u64,
    /// Total tasks cancelled.
    pub tasks_cancelled: u64,
    /// Total tasks stolen from other workers.
    pub tasks_stolen: u64,
    /// Total worker-idle samples.
    pub idle_samples: u64,
    /// Total worker-busy samples.
    pub busy_samples: u64,
    /// Cumulative task latency in "ticks".
    pub total_latency_ticks: u64,
    /// Maximum observed task latency.
    pub max_latency_ticks: u64,
    /// Total tasks exceeding latency threshold.
    pub latency_violations: u64,
    /// Latency violation threshold.
    pub latency_threshold_ticks: u64,
}
#[allow(dead_code)]
impl ExtSchedulerStats {
    /// Create a new stats instance.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record task creation.
    pub fn record_created(&mut self) {
        self.tasks_created += 1;
    }
    /// Record task completion with latency.
    pub fn record_completed(&mut self, latency_ticks: u64) {
        self.tasks_completed += 1;
        self.total_latency_ticks += latency_ticks;
        if latency_ticks > self.max_latency_ticks {
            self.max_latency_ticks = latency_ticks;
        }
        if self.latency_threshold_ticks > 0 && latency_ticks > self.latency_threshold_ticks {
            self.latency_violations += 1;
        }
    }
    /// Record a task cancellation.
    pub fn record_cancelled(&mut self) {
        self.tasks_cancelled += 1;
    }
    /// Record a work-steal event.
    pub fn record_steal(&mut self) {
        self.tasks_stolen += 1;
    }
    /// Record worker state samples.
    pub fn record_sample(&mut self, busy: bool) {
        if busy {
            self.busy_samples += 1;
        } else {
            self.idle_samples += 1;
        }
    }
    /// Worker utilization (0.0 – 1.0).
    pub fn utilization(&self) -> f64 {
        let total = self.busy_samples + self.idle_samples;
        if total == 0 {
            return 0.0;
        }
        self.busy_samples as f64 / total as f64
    }
    /// Average latency in ticks.
    pub fn avg_latency(&self) -> f64 {
        if self.tasks_completed == 0 {
            return 0.0;
        }
        self.total_latency_ticks as f64 / self.tasks_completed as f64
    }
    /// Merge with another stats instance.
    pub fn merge(&mut self, other: &ExtSchedulerStats) {
        self.tasks_created += other.tasks_created;
        self.tasks_completed += other.tasks_completed;
        self.tasks_cancelled += other.tasks_cancelled;
        self.tasks_stolen += other.tasks_stolen;
        self.idle_samples += other.idle_samples;
        self.busy_samples += other.busy_samples;
        self.total_latency_ticks += other.total_latency_ticks;
        if other.max_latency_ticks > self.max_latency_ticks {
            self.max_latency_ticks = other.max_latency_ticks;
        }
        self.latency_violations += other.latency_violations;
    }
    /// Reset all statistics.
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}
/// Priority level for task scheduling.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum TaskPriority {
    /// Background (lowest): GC, cleanup.
    Background = 0,
    /// Low priority (background tasks).
    Low = 1,
    /// Normal priority (default).
    #[default]
    Normal = 2,
    /// High priority (user-facing tasks).
    High = 3,
    /// Critical priority (system tasks).
    Critical = 4,
}
impl TaskPriority {
    /// Create from a numeric value.
    pub fn from_u8(v: u8) -> Self {
        match v {
            0 => TaskPriority::Background,
            1 => TaskPriority::Low,
            2 => TaskPriority::Normal,
            3 => TaskPriority::High,
            _ => TaskPriority::Critical,
        }
    }
    /// Numeric value of the priority.
    pub fn value(self) -> u8 {
        self as u8
    }
    /// Whether this priority is above Normal.
    pub fn is_high(self) -> bool {
        self >= TaskPriority::High
    }
    /// Whether this is a background task.
    pub fn is_background(self) -> bool {
        self == TaskPriority::Background
    }
}
/// Tracks backpressure between producers and consumers.
#[allow(dead_code)]
pub struct BackpressureController {
    /// Maximum queue depth before producer is throttled.
    pub high_watermark: usize,
    /// Queue depth at which throttling is released.
    pub low_watermark: usize,
    /// Current queue depth.
    pub current_depth: usize,
    /// Whether the producer is currently throttled.
    throttled: bool,
    /// Total times throttling was applied.
    pub throttle_events: u64,
}
#[allow(dead_code)]
impl BackpressureController {
    /// Create a backpressure controller.
    pub fn new(high_watermark: usize, low_watermark: usize) -> Self {
        BackpressureController {
            high_watermark,
            low_watermark,
            current_depth: 0,
            throttled: false,
            throttle_events: 0,
        }
    }
    /// Record that one item was enqueued.
    pub fn enqueue(&mut self) {
        self.current_depth += 1;
        if self.current_depth >= self.high_watermark && !self.throttled {
            self.throttled = true;
            self.throttle_events += 1;
        }
    }
    /// Record that one item was dequeued.
    pub fn dequeue(&mut self) {
        if self.current_depth > 0 {
            self.current_depth -= 1;
        }
        if self.current_depth <= self.low_watermark {
            self.throttled = false;
        }
    }
    /// Whether the producer should be throttled.
    pub fn is_throttled(&self) -> bool {
        self.throttled
    }
    /// Fill ratio (0.0 = empty, 1.0 = at high watermark).
    pub fn fill_ratio(&self) -> f64 {
        if self.high_watermark == 0 {
            return 1.0;
        }
        (self.current_depth as f64 / self.high_watermark as f64).min(1.0)
    }
    /// Reset depth and throttle state.
    pub fn reset(&mut self) {
        self.current_depth = 0;
        self.throttled = false;
    }
}
/// Parallel evaluation primitives.
pub struct ParallelEval;
impl ParallelEval {
    /// Evaluate multiple independent tasks in parallel and return results.
    pub fn par_map(scheduler: &mut Scheduler, actions: Vec<RtObject>) -> Vec<TaskId> {
        actions
            .into_iter()
            .map(|action| scheduler.spawn(action))
            .collect()
    }
    /// Spawn two tasks and combine their results.
    pub fn par_pair(
        scheduler: &mut Scheduler,
        action_a: RtObject,
        action_b: RtObject,
    ) -> (TaskId, TaskId) {
        let a = scheduler.spawn(action_a);
        let b = scheduler.spawn(action_b);
        (a, b)
    }
    /// Create a task that depends on the completion of all given tasks.
    pub fn when_all(
        scheduler: &mut Scheduler,
        deps: Vec<TaskId>,
        continuation: RtObject,
    ) -> TaskId {
        scheduler.spawn_with_deps(continuation, deps)
    }
    /// Create a barrier: spawn a continuation after all deps complete.
    pub fn barrier(
        scheduler: &mut Scheduler,
        dep_actions: Vec<RtObject>,
        continuation: RtObject,
    ) -> (Vec<TaskId>, TaskId) {
        let dep_ids: Vec<TaskId> = dep_actions
            .into_iter()
            .map(|action| scheduler.spawn(action))
            .collect();
        let barrier_id = scheduler.spawn_with_deps(continuation, dep_ids.clone());
        (dep_ids, barrier_id)
    }
}
/// Profiling record for a single task.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct TaskProfile {
    /// Task id.
    pub id: TaskId,
    /// Task name (if any).
    pub name: Option<String>,
    /// Tick at creation.
    pub created_at: u64,
    /// Tick at start of execution.
    pub started_at: Option<u64>,
    /// Tick at completion.
    pub completed_at: Option<u64>,
    /// Number of times the task yielded.
    pub yield_count: u32,
    /// Number of times the task was stolen.
    pub steal_count: u32,
    /// Worker that ultimately completed the task.
    pub completed_by: Option<usize>,
}
#[allow(dead_code)]
impl TaskProfile {
    /// Create a new profile at the given creation tick.
    pub fn new(id: TaskId, created_at: u64) -> Self {
        TaskProfile {
            id,
            name: None,
            created_at,
            started_at: None,
            completed_at: None,
            yield_count: 0,
            steal_count: 0,
            completed_by: None,
        }
    }
    /// Record the start of execution.
    pub fn start(&mut self, tick: u64) {
        self.started_at = Some(tick);
    }
    /// Record the completion of the task.
    pub fn complete(&mut self, tick: u64, worker: usize) {
        self.completed_at = Some(tick);
        self.completed_by = Some(worker);
    }
    /// Queuing latency (time from creation to start).
    pub fn queue_latency(&self) -> Option<u64> {
        self.started_at.map(|s| s - self.created_at)
    }
    /// Execution time (time from start to completion).
    pub fn execution_time(&self) -> Option<u64> {
        match (self.started_at, self.completed_at) {
            (Some(s), Some(c)) => Some(c - s),
            _ => None,
        }
    }
    /// Total latency (creation to completion).
    pub fn total_latency(&self) -> Option<u64> {
        self.completed_at.map(|c| c - self.created_at)
    }
}
/// Load balancer for distributing tasks.
pub struct LoadBalancer {
    /// Strategy to use.
    strategy: LoadBalanceStrategy,
    /// Round-robin counter.
    rr_counter: usize,
    /// Number of workers.
    num_workers: usize,
}
impl LoadBalancer {
    /// Create a new load balancer.
    pub fn new(strategy: LoadBalanceStrategy, num_workers: usize) -> Self {
        LoadBalancer {
            strategy,
            rr_counter: 0,
            num_workers,
        }
    }
    /// Select a worker for a task.
    pub fn select_worker(&mut self, worker_loads: &[usize]) -> usize {
        match self.strategy {
            LoadBalanceStrategy::RoundRobin => {
                let worker = self.rr_counter % self.num_workers;
                self.rr_counter += 1;
                worker
            }
            LoadBalanceStrategy::LeastLoaded => worker_loads
                .iter()
                .enumerate()
                .min_by_key(|(_, load)| *load)
                .map(|(i, _)| i)
                .unwrap_or(0),
            LoadBalanceStrategy::Random => {
                self.rr_counter = self
                    .rr_counter
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1);
                (self.rr_counter >> 16) % self.num_workers
            }
            LoadBalanceStrategy::WorkStealing => 0,
        }
    }
}
/// Thread-safe shared state for the scheduler.
///
/// In a real multi-threaded implementation, this would be accessed
/// by multiple worker threads.
pub struct SharedState {
    /// Whether the scheduler should stop.
    pub shutdown: Arc<AtomicBool>,
    /// Global task counter.
    pub task_counter: Arc<AtomicU64>,
    /// Shared global queue.
    pub global_queue: Arc<Mutex<VecDeque<TaskId>>>,
    /// Shared task results.
    pub results: Arc<Mutex<HashMap<TaskId, RtObject>>>,
}
impl SharedState {
    /// Create new shared state.
    pub fn new() -> Self {
        SharedState {
            shutdown: Arc::new(AtomicBool::new(false)),
            task_counter: Arc::new(AtomicU64::new(0)),
            global_queue: Arc::new(Mutex::new(VecDeque::new())),
            results: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    /// Request shutdown.
    pub fn request_shutdown(&self) {
        self.shutdown.store(true, Ordering::Release);
    }
    /// Check if shutdown was requested.
    pub fn should_shutdown(&self) -> bool {
        self.shutdown.load(Ordering::Acquire)
    }
    /// Generate a new task ID.
    pub fn next_task_id(&self) -> TaskId {
        let id = self.task_counter.fetch_add(1, Ordering::Relaxed);
        TaskId::new(id)
    }
    /// Push a task to the global queue.
    pub fn push_task(&self, task_id: TaskId) {
        if let Ok(mut queue) = self.global_queue.lock() {
            queue.push_back(task_id);
        }
    }
    /// Pop a task from the global queue.
    pub fn pop_task(&self) -> Option<TaskId> {
        self.global_queue.lock().ok()?.pop_front()
    }
    /// Store a task result.
    pub fn store_result(&self, task_id: TaskId, result: RtObject) {
        if let Ok(mut results) = self.results.lock() {
            results.insert(task_id, result);
        }
    }
    /// Get a task result.
    pub fn get_result(&self, task_id: TaskId) -> Option<RtObject> {
        self.results.lock().ok()?.get(&task_id).cloned()
    }
}
/// Simulates preemptive scheduling by tracking time slices.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct PreemptionSimulator {
    /// Time slice in ticks per task before forced preemption.
    pub time_slice: u64,
    /// Current tick within the slice for the active task.
    pub ticks_used: u64,
    /// Total preemptions performed.
    pub preemptions: u64,
    /// Currently active task.
    pub active_task: Option<TaskId>,
}
#[allow(dead_code)]
impl PreemptionSimulator {
    /// Create a simulator with given time slice.
    pub fn new(time_slice: u64) -> Self {
        PreemptionSimulator {
            time_slice,
            ticks_used: 0,
            preemptions: 0,
            active_task: None,
        }
    }
    /// Assign the active task.
    pub fn set_active(&mut self, id: TaskId) {
        self.active_task = Some(id);
        self.ticks_used = 0;
    }
    /// Tick the clock. Returns `true` if the task should be preempted.
    pub fn tick(&mut self) -> bool {
        self.ticks_used += 1;
        if self.ticks_used >= self.time_slice {
            self.preemptions += 1;
            self.ticks_used = 0;
            self.active_task = None;
            true
        } else {
            false
        }
    }
    /// Remaining ticks in the current slice.
    pub fn remaining(&self) -> u64 {
        self.time_slice.saturating_sub(self.ticks_used)
    }
}
/// Configuration for the task scheduler.
#[derive(Clone, Debug)]
pub struct SchedulerConfig {
    /// Number of workers.
    pub num_workers: usize,
    /// Capacity of each worker's deque.
    pub deque_capacity: usize,
    /// Maximum number of tasks that can be active.
    pub max_tasks: usize,
    /// Whether work stealing is enabled.
    pub work_stealing: bool,
    /// Steal batch size (number of tasks to steal at once).
    pub steal_batch_size: usize,
    /// Whether to use priority scheduling.
    pub priority_scheduling: bool,
    /// Maximum number of retries for failed tasks.
    pub max_retries: u32,
}
impl SchedulerConfig {
    /// Create default configuration.
    pub fn new() -> Self {
        SchedulerConfig {
            num_workers: 4,
            deque_capacity: 1024,
            max_tasks: 100_000,
            work_stealing: true,
            steal_batch_size: 4,
            priority_scheduling: true,
            max_retries: 3,
        }
    }
    /// Create configuration for single-threaded execution.
    pub fn single_threaded() -> Self {
        SchedulerConfig {
            num_workers: 1,
            deque_capacity: 1024,
            max_tasks: 100_000,
            work_stealing: false,
            steal_batch_size: 1,
            priority_scheduling: false,
            max_retries: 0,
        }
    }
    /// Set the number of workers.
    pub fn with_workers(mut self, n: usize) -> Self {
        self.num_workers = n.max(1);
        self
    }
    /// Set the deque capacity.
    pub fn with_deque_capacity(mut self, cap: usize) -> Self {
        self.deque_capacity = cap;
        self
    }
    /// Set the maximum number of tasks.
    pub fn with_max_tasks(mut self, max: usize) -> Self {
        self.max_tasks = max;
        self
    }
    /// Enable or disable work stealing.
    pub fn with_work_stealing(mut self, enabled: bool) -> Self {
        self.work_stealing = enabled;
        self
    }
}
/// The main task scheduler.
///
/// Manages workers, task queues, and coordinates execution.
pub struct Scheduler {
    /// Configuration.
    config: SchedulerConfig,
    /// All workers.
    pub(super) workers: Vec<Worker>,
    /// All tasks, indexed by ID.
    tasks: HashMap<TaskId, Task>,
    /// Global task queue (for tasks not assigned to a worker).
    pub(super) global_queue: VecDeque<TaskId>,
    /// Completed task IDs.
    pub(super) completed: Vec<TaskId>,
    /// Next task ID.
    next_task_id: u64,
    /// Whether the scheduler is running.
    running: bool,
    /// Statistics.
    stats: SchedulerStats,
}
impl Scheduler {
    /// Create a new scheduler with the given configuration.
    pub fn new(config: SchedulerConfig) -> Self {
        let workers: Vec<Worker> = (0..config.num_workers)
            .map(|id| Worker::new(id, config.deque_capacity))
            .collect();
        Scheduler {
            config,
            workers,
            tasks: HashMap::new(),
            global_queue: VecDeque::new(),
            completed: Vec::new(),
            next_task_id: 0,
            running: false,
            stats: SchedulerStats::default(),
        }
    }
    /// Create a single-threaded scheduler.
    pub fn single_threaded() -> Self {
        Scheduler::new(SchedulerConfig::single_threaded())
    }
    /// Spawn a new task and return its ID.
    pub fn spawn(&mut self, action: RtObject) -> TaskId {
        let id = TaskId::new(self.next_task_id);
        self.next_task_id += 1;
        let task = Task::new(id, action);
        self.tasks.insert(id, task);
        self.global_queue.push_back(id);
        self.stats.tasks_created += 1;
        let active = self.active_task_count() as u64;
        if active > self.stats.peak_active_tasks {
            self.stats.peak_active_tasks = active;
        }
        id
    }
    /// Spawn a named task.
    pub fn spawn_named(&mut self, name: String, action: RtObject) -> TaskId {
        let id = TaskId::new(self.next_task_id);
        self.next_task_id += 1;
        let task = Task::named(id, name, action);
        self.tasks.insert(id, task);
        self.global_queue.push_back(id);
        self.stats.tasks_created += 1;
        id
    }
    /// Spawn a task with a priority.
    pub fn spawn_with_priority(&mut self, action: RtObject, priority: TaskPriority) -> TaskId {
        let id = TaskId::new(self.next_task_id);
        self.next_task_id += 1;
        let task = Task::new(id, action).with_priority(priority);
        self.tasks.insert(id, task);
        if self.config.priority_scheduling && priority >= TaskPriority::High {
            self.global_queue.push_front(id);
        } else {
            self.global_queue.push_back(id);
        }
        self.stats.tasks_created += 1;
        id
    }
    /// Spawn a task with dependencies.
    pub fn spawn_with_deps(&mut self, action: RtObject, deps: Vec<TaskId>) -> TaskId {
        let id = TaskId::new(self.next_task_id);
        self.next_task_id += 1;
        let mut task = Task::new(id, action);
        task.dependencies = deps.clone();
        let all_satisfied = deps.iter().all(|dep| self.completed.contains(dep));
        if all_satisfied {
            task.state = TaskState::Created;
            self.global_queue.push_back(id);
        } else {
            task.state = TaskState::Suspended {
                waiting_on: deps.clone(),
            };
        }
        for dep in &deps {
            if let Some(dep_task) = self.tasks.get_mut(dep) {
                dep_task.dependents.push(id);
            }
        }
        self.tasks.insert(id, task);
        self.stats.tasks_created += 1;
        id
    }
    /// Get a task by ID.
    pub fn get_task(&self, id: TaskId) -> Option<&Task> {
        self.tasks.get(&id)
    }
    /// Get a task mutably by ID.
    pub fn get_task_mut(&mut self, id: TaskId) -> Option<&mut Task> {
        self.tasks.get_mut(&id)
    }
    /// Cancel a task.
    pub fn cancel(&mut self, id: TaskId) -> bool {
        if let Some(task) = self.tasks.get_mut(&id) {
            if !task.state.is_terminal() {
                task.cancel();
                self.stats.tasks_cancelled += 1;
                return true;
            }
        }
        false
    }
    /// Check if a task has completed.
    pub fn is_complete(&self, id: TaskId) -> bool {
        self.tasks
            .get(&id)
            .map(|t| t.state.is_terminal())
            .unwrap_or(false)
    }
    /// Get the result of a completed task.
    pub fn get_result(&self, id: TaskId) -> Option<&RtObject> {
        self.tasks.get(&id).and_then(|t| t.result())
    }
    /// Run one scheduling step.
    ///
    /// This distributes tasks from the global queue to workers,
    /// performs work stealing, and returns the next task to execute.
    pub fn schedule_step(&mut self) -> Option<(usize, TaskId)> {
        self.stats.scheduling_rounds += 1;
        while let Some(task_id) = self.global_queue.pop_front() {
            let target_worker = self.find_least_loaded_worker();
            if !self.workers[target_worker].push_task(task_id) {
                self.global_queue.push_front(task_id);
                break;
            }
            if let Some(task) = self.tasks.get_mut(&task_id) {
                task.state = TaskState::Queued;
            }
        }
        for worker_id in 0..self.workers.len() {
            if let Some(task_id) = self.workers[worker_id].pop_task() {
                self.workers[worker_id].start_task(task_id);
                if let Some(task) = self.tasks.get_mut(&task_id) {
                    task.state = TaskState::Running { worker_id };
                }
                return Some((worker_id, task_id));
            }
        }
        if self.config.work_stealing {
            if let Some((worker_id, task_id)) = self.try_steal() {
                self.workers[worker_id].start_task(task_id);
                if let Some(task) = self.tasks.get_mut(&task_id) {
                    task.state = TaskState::Running { worker_id };
                }
                return Some((worker_id, task_id));
            }
        }
        self.stats.idle_cycles += 1;
        None
    }
    /// Complete a task with a result.
    pub fn complete_task(&mut self, task_id: TaskId, result: RtObject) {
        let dependents = self
            .tasks
            .get(&task_id)
            .map(|t| t.dependents.clone())
            .unwrap_or_default();
        if let Some(task) = self.tasks.get_mut(&task_id) {
            task.complete(result);
            self.completed.push(task_id);
            self.stats.tasks_completed += 1;
        }
        for worker in &mut self.workers {
            if worker.current_task == Some(task_id) {
                worker.finish_task();
                break;
            }
        }
        for dep_id in &dependents {
            self.try_wake_task(*dep_id);
        }
    }
    /// Fail a task with an error.
    pub fn fail_task(&mut self, task_id: TaskId, error: String) {
        if let Some(task) = self.tasks.get_mut(&task_id) {
            task.fail(error);
            self.stats.tasks_failed += 1;
        }
        for worker in &mut self.workers {
            if worker.current_task == Some(task_id) {
                worker.finish_task();
                break;
            }
        }
    }
    /// Try to wake a suspended task.
    fn try_wake_task(&mut self, task_id: TaskId) {
        let should_wake = if let Some(task) = self.tasks.get(&task_id) {
            if let TaskState::Suspended { ref waiting_on } = task.state {
                waiting_on.iter().all(|dep| self.completed.contains(dep))
            } else {
                false
            }
        } else {
            false
        };
        if should_wake {
            if let Some(task) = self.tasks.get_mut(&task_id) {
                task.state = TaskState::Queued;
            }
            self.global_queue.push_back(task_id);
        }
    }
    /// Find the worker with the least load.
    fn find_least_loaded_worker(&self) -> usize {
        self.workers
            .iter()
            .enumerate()
            .min_by_key(|(_, w)| w.load())
            .map(|(i, _)| i)
            .unwrap_or(0)
    }
    /// Try to steal a task from another worker.
    fn try_steal(&mut self) -> Option<(usize, TaskId)> {
        self.stats.steal_attempts += 1;
        let idle_worker = self.workers.iter().position(|w| w.idle)?;
        let busy_worker = self
            .workers
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != idle_worker)
            .max_by_key(|(_, w)| w.deque.len())?
            .0;
        if self.workers[busy_worker].deque.is_empty() {
            return None;
        }
        let stolen = self.workers[busy_worker].deque.steal()?;
        self.workers[busy_worker].tasks_stolen_from += 1;
        self.workers[idle_worker].tasks_stolen += 1;
        self.stats.total_steals += 1;
        Some((idle_worker, stolen))
    }
    /// Number of active (non-terminal) tasks.
    pub fn active_task_count(&self) -> usize {
        self.tasks
            .values()
            .filter(|t| !t.state.is_terminal())
            .count()
    }
    /// Number of completed tasks.
    pub fn completed_count(&self) -> usize {
        self.completed.len()
    }
    /// Number of workers.
    pub fn num_workers(&self) -> usize {
        self.workers.len()
    }
    /// Get worker statistics.
    pub fn worker_stats(&self) -> Vec<WorkerStats> {
        self.workers
            .iter()
            .map(|w| WorkerStats {
                id: w.id,
                tasks_completed: w.tasks_completed,
                tasks_stolen: w.tasks_stolen,
                tasks_stolen_from: w.tasks_stolen_from,
                queue_length: w.deque.len(),
                idle: w.idle,
            })
            .collect()
    }
    /// Get the scheduler statistics.
    pub fn stats(&self) -> &SchedulerStats {
        &self.stats
    }
    /// Get the configuration.
    pub fn config(&self) -> &SchedulerConfig {
        &self.config
    }
    /// Check if the scheduler is running.
    pub fn is_running(&self) -> bool {
        self.running
    }
    /// Start the scheduler.
    pub fn start(&mut self) {
        self.running = true;
    }
    /// Stop the scheduler.
    pub fn stop(&mut self) {
        self.running = false;
    }
    /// Reset the scheduler.
    pub fn reset(&mut self) {
        self.tasks.clear();
        self.global_queue.clear();
        self.completed.clear();
        self.next_task_id = 0;
        self.stats = SchedulerStats::default();
        for worker in &mut self.workers {
            worker.deque.clear();
            worker.tasks_completed = 0;
            worker.tasks_stolen = 0;
            worker.tasks_stolen_from = 0;
            worker.idle = true;
            worker.current_task = None;
        }
    }
    /// Run all tasks to completion (single-threaded simulation).
    pub fn run_all(&mut self, mut executor: impl FnMut(&Task) -> Result<RtObject, String>) {
        self.start();
        while self.active_task_count() > 0 {
            if let Some((_worker_id, task_id)) = self.schedule_step() {
                let result = {
                    let task = self
                        .tasks
                        .get(&task_id)
                        .expect("task_id returned by schedule_step must exist in the tasks map");
                    executor(task)
                };
                match result {
                    Ok(value) => self.complete_task(task_id, value),
                    Err(error) => self.fail_task(task_id, error),
                }
            } else {
                let has_suspended = self.tasks.values().any(|t| t.state.is_suspended());
                if has_suspended && self.global_queue.is_empty() {
                    let suspended: Vec<TaskId> = self
                        .tasks
                        .iter()
                        .filter(|(_, t)| t.state.is_suspended())
                        .map(|(id, _)| *id)
                        .collect();
                    for id in suspended {
                        self.fail_task(id, "deadlock detected".to_string());
                    }
                }
                break;
            }
        }
        self.stop();
    }
}
/// A work-stealing deque for a single worker.
///
/// The owner pushes/pops from the bottom; thieves steal from the top.
/// This implementation uses a simple VecDeque protected by a mutex
/// (a production implementation would use a lock-free deque).
pub struct WorkStealingDeque {
    /// The deque storage.
    pub(super) deque: VecDeque<TaskId>,
    /// Maximum capacity.
    pub(super) capacity: usize,
}
impl WorkStealingDeque {
    /// Create a new empty deque.
    pub fn new(capacity: usize) -> Self {
        WorkStealingDeque {
            deque: VecDeque::with_capacity(capacity),
            capacity,
        }
    }
    /// Push a task to the bottom (owner's end).
    pub fn push(&mut self, task_id: TaskId) -> bool {
        if self.deque.len() >= self.capacity {
            return false;
        }
        self.deque.push_back(task_id);
        true
    }
    /// Pop a task from the bottom (owner's end).
    pub fn pop(&mut self) -> Option<TaskId> {
        self.deque.pop_back()
    }
    /// Steal a task from the top (thief's end).
    pub fn steal(&mut self) -> Option<TaskId> {
        self.deque.pop_front()
    }
    /// Number of tasks in the deque.
    pub fn len(&self) -> usize {
        self.deque.len()
    }
    /// Check if the deque is empty.
    pub fn is_empty(&self) -> bool {
        self.deque.is_empty()
    }
    /// Check if the deque is full.
    pub fn is_full(&self) -> bool {
        self.deque.len() >= self.capacity
    }
    /// Clear all tasks.
    pub fn clear(&mut self) {
        self.deque.clear();
    }
    /// Peek at the bottom task without removing it.
    pub fn peek(&self) -> Option<&TaskId> {
        self.deque.back()
    }
    /// Steal up to n tasks.
    pub fn steal_batch(&mut self, n: usize) -> Vec<TaskId> {
        let count = n.min(self.deque.len() / 2).max(1).min(self.deque.len());
        let mut stolen = Vec::with_capacity(count);
        for _ in 0..count {
            if let Some(task_id) = self.deque.pop_front() {
                stolen.push(task_id);
            } else {
                break;
            }
        }
        stolen
    }
}
/// Statistics for the scheduler.
#[derive(Clone, Debug, Default)]
pub struct SchedulerStats {
    /// Total tasks created.
    pub tasks_created: u64,
    /// Total tasks completed.
    pub tasks_completed: u64,
    /// Total tasks failed.
    pub tasks_failed: u64,
    /// Total tasks cancelled.
    pub tasks_cancelled: u64,
    /// Total steals performed.
    pub total_steals: u64,
    /// Total steal attempts (including failures).
    pub steal_attempts: u64,
    /// Total idle cycles.
    pub idle_cycles: u64,
    /// Peak number of active tasks.
    pub peak_active_tasks: u64,
    /// Total scheduling rounds.
    pub scheduling_rounds: u64,
}
/// An actor mailbox (a queue of messages).
#[allow(dead_code)]
pub struct ActorMailbox {
    /// The actor's ID.
    pub id: ActorId,
    /// Pending messages.
    messages: VecDeque<ActorMessage>,
    /// Total messages received.
    pub total_received: u64,
    /// Total messages processed.
    pub total_processed: u64,
}
#[allow(dead_code)]
impl ActorMailbox {
    /// Create a new mailbox.
    pub fn new(id: ActorId) -> Self {
        ActorMailbox {
            id,
            messages: VecDeque::new(),
            total_received: 0,
            total_processed: 0,
        }
    }
    /// Enqueue a message.
    pub fn send(&mut self, msg: ActorMessage) {
        self.messages.push_back(msg);
        self.total_received += 1;
    }
    /// Dequeue the next message.
    pub fn receive(&mut self) -> Option<ActorMessage> {
        let msg = self.messages.pop_front();
        if msg.is_some() {
            self.total_processed += 1;
        }
        msg
    }
    /// Number of pending messages.
    pub fn pending(&self) -> usize {
        self.messages.len()
    }
    /// Whether the mailbox is empty.
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }
}
/// A unique identifier for a task.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TaskId(pub u64);
impl TaskId {
    /// Create a new task ID.
    pub fn new(id: u64) -> Self {
        TaskId(id)
    }
    /// Get the raw ID value.
    pub fn raw(self) -> u64 {
        self.0
    }
}
/// The state of a task in its lifecycle.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TaskState {
    /// Task has been created but not yet scheduled.
    Created,
    /// Task is in a queue waiting to be executed.
    Queued,
    /// Task is currently running on a worker.
    Running {
        /// Which worker is executing this task.
        worker_id: usize,
    },
    /// Task is suspended (waiting for a dependency).
    Suspended {
        /// Task IDs this task is waiting on.
        waiting_on: Vec<TaskId>,
    },
    /// Task has completed successfully.
    Completed {
        /// The result value.
        result: RtObject,
    },
    /// Task has failed with an error.
    Failed {
        /// The error message.
        error: String,
    },
    /// Task has been cancelled.
    Cancelled,
}
impl TaskState {
    /// Check if the task is in a terminal state.
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            TaskState::Completed { .. } | TaskState::Failed { .. } | TaskState::Cancelled
        )
    }
    /// Check if the task is runnable.
    pub fn is_runnable(&self) -> bool {
        matches!(self, TaskState::Created | TaskState::Queued)
    }
    /// Check if the task is running.
    pub fn is_running(&self) -> bool {
        matches!(self, TaskState::Running { .. })
    }
    /// Check if the task is suspended.
    pub fn is_suspended(&self) -> bool {
        matches!(self, TaskState::Suspended { .. })
    }
}
/// A cooperative yield mechanism for long-running tasks.
#[allow(dead_code)]
pub struct YieldPoint {
    /// Whether a yield has been requested.
    requested: Arc<AtomicBool>,
    /// How many times this yield point has been checked.
    pub check_count: u64,
    /// How many times the task actually yielded.
    pub yield_count: u64,
    /// Instructions between checks.
    pub check_interval: u64,
}
#[allow(dead_code)]
impl YieldPoint {
    /// Create a new yield point.
    pub fn new() -> Self {
        YieldPoint {
            requested: Arc::new(AtomicBool::new(false)),
            check_count: 0,
            yield_count: 0,
            check_interval: 100,
        }
    }
    /// Create with a custom check interval.
    pub fn with_interval(check_interval: u64) -> Self {
        YieldPoint {
            requested: Arc::new(AtomicBool::new(false)),
            check_count: 0,
            yield_count: 0,
            check_interval,
        }
    }
    /// Request a yield (called from scheduler).
    pub fn request_yield(&self) {
        self.requested.store(true, Ordering::Release);
    }
    /// Clear the yield request (called after yielding).
    pub fn clear_request(&self) {
        self.requested.store(false, Ordering::Release);
    }
    /// Check if a yield should happen. Returns true if the task should yield.
    pub fn should_yield(&mut self) -> bool {
        self.check_count += 1;
        if self.requested.load(Ordering::Acquire) {
            self.yield_count += 1;
            self.clear_request();
            true
        } else {
            false
        }
    }
    /// Get a handle that the scheduler can use to request a yield.
    pub fn handle(&self) -> YieldHandle {
        YieldHandle {
            requested: Arc::clone(&self.requested),
        }
    }
}
