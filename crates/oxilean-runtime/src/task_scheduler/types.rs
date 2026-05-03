//! Concurrent task scheduler types: metrics, adaptive load balancing, work stealing.

use std::collections::HashMap;

/// Unique task identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TaskId(pub u64);

impl TaskId {
    /// Create a new TaskId from a raw value.
    pub fn new(id: u64) -> Self {
        TaskId(id)
    }

    /// Return the raw identifier.
    pub fn raw(self) -> u64 {
        self.0
    }
}

impl std::fmt::Display for TaskId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "task#{}", self.0)
    }
}

/// Task execution priority level.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum TaskPriority {
    /// Highest priority — must run immediately.
    Critical = 0,
    /// High priority — runs before normal work.
    High = 1,
    /// Default priority.
    Normal = 2,
    /// Lower priority — deferred when system is busy.
    Low = 3,
    /// Runs only when workers are otherwise idle.
    Background = 4,
}

impl TaskPriority {
    /// Numeric priority level (lower = more urgent).
    pub fn level(self) -> u8 {
        self as u8
    }

    /// Convert from a raw level, clamping unknown values to `Background`.
    pub fn from_level(level: u8) -> Self {
        match level {
            0 => TaskPriority::Critical,
            1 => TaskPriority::High,
            2 => TaskPriority::Normal,
            3 => TaskPriority::Low,
            _ => TaskPriority::Background,
        }
    }
}

impl std::fmt::Display for TaskPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            TaskPriority::Critical => "critical",
            TaskPriority::High => "high",
            TaskPriority::Normal => "normal",
            TaskPriority::Low => "low",
            TaskPriority::Background => "background",
        };
        write!(f, "{}", s)
    }
}

/// Current lifecycle state of a task.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TaskState {
    /// Queued, waiting to be dispatched.
    Pending,
    /// Actively executing on the given worker.
    Running {
        /// Index of the worker executing this task.
        worker: usize,
    },
    /// Finished successfully.
    Completed,
    /// Finished with an error.
    Failed(String),
    /// Removed from the queue before execution.
    Cancelled,
}

impl TaskState {
    /// Whether the task is in a terminal state (completed, failed, or cancelled).
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            TaskState::Completed | TaskState::Failed(_) | TaskState::Cancelled
        )
    }

    /// Whether the task is still eligible to run.
    pub fn is_pending(&self) -> bool {
        matches!(self, TaskState::Pending)
    }

    /// Whether the task is actively executing.
    pub fn is_running(&self) -> bool {
        matches!(self, TaskState::Running { .. })
    }
}

impl std::fmt::Display for TaskState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskState::Pending => write!(f, "pending"),
            TaskState::Running { worker } => write!(f, "running(worker={})", worker),
            TaskState::Completed => write!(f, "completed"),
            TaskState::Failed(msg) => write!(f, "failed({})", msg),
            TaskState::Cancelled => write!(f, "cancelled"),
        }
    }
}

/// A schedulable unit of work.
#[derive(Clone, Debug)]
pub struct Task {
    /// Unique task identifier.
    pub id: TaskId,
    /// Scheduling priority.
    pub priority: TaskPriority,
    /// Current state of the task.
    pub state: TaskState,
    /// Nanosecond timestamp when the task was enqueued.
    pub enqueued_at: u64,
    /// Nanosecond timestamp when the task started executing, if at all.
    pub started_at: Option<u64>,
    /// Nanosecond timestamp when the task reached a terminal state.
    pub completed_at: Option<u64>,
}

impl Task {
    /// Create a new pending task.
    pub fn new(id: TaskId, priority: TaskPriority, enqueued_at: u64) -> Self {
        Task {
            id,
            priority,
            state: TaskState::Pending,
            enqueued_at,
            started_at: None,
            completed_at: None,
        }
    }

    /// Latency from enqueue to completion in nanoseconds.
    pub fn latency_ns(&self) -> Option<u64> {
        self.completed_at
            .map(|c| c.saturating_sub(self.enqueued_at))
    }

    /// Queuing delay (time between enqueue and start) in nanoseconds.
    pub fn queue_delay_ns(&self) -> Option<u64> {
        self.started_at.map(|s| s.saturating_sub(self.enqueued_at))
    }

    /// Execution duration (start to completion) in nanoseconds.
    pub fn execution_ns(&self) -> Option<u64> {
        match (self.started_at, self.completed_at) {
            (Some(s), Some(c)) => Some(c.saturating_sub(s)),
            _ => None,
        }
    }
}

/// Per-worker execution statistics.
#[derive(Clone, Debug, Default)]
pub struct WorkerStats {
    /// Worker index.
    pub id: usize,
    /// Number of tasks this worker has completed.
    pub tasks_completed: u64,
    /// Number of tasks stolen from other workers' queues.
    pub tasks_stolen: u64,
    /// Accumulated nanoseconds this worker spent idle.
    pub idle_time_ns: u64,
    /// Accumulated nanoseconds this worker spent executing tasks.
    pub busy_time_ns: u64,
}

impl WorkerStats {
    /// Create stats for a given worker id.
    pub fn new(id: usize) -> Self {
        WorkerStats {
            id,
            ..Default::default()
        }
    }

    /// Utilization ratio: `busy_time / (busy_time + idle_time)`.
    pub fn utilization(&self) -> f64 {
        let total = self.busy_time_ns + self.idle_time_ns;
        if total == 0 {
            return 0.0;
        }
        self.busy_time_ns as f64 / total as f64
    }

    /// Total tasks handled (completed + stolen).
    pub fn total_tasks(&self) -> u64 {
        self.tasks_completed + self.tasks_stolen
    }
}

impl std::fmt::Display for WorkerStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Worker[{}] completed={} stolen={} util={:.2}",
            self.id,
            self.tasks_completed,
            self.tasks_stolen,
            self.utilization()
        )
    }
}

/// Aggregated scheduler-level metrics.
#[derive(Clone, Debug, Default)]
pub struct SchedulerMetrics {
    /// Total tasks ever submitted.
    pub total_tasks: u64,
    /// Total tasks that completed successfully.
    pub completed: u64,
    /// Total tasks that failed.
    pub failed: u64,
    /// Per-worker statistics.
    pub workers: Vec<WorkerStats>,
    /// Rolling throughput estimate (tasks per second).
    pub throughput_per_sec: f64,
    /// Rolling average task latency (nanoseconds).
    pub avg_latency_ns: u64,
}

impl SchedulerMetrics {
    /// Number of tasks still in flight (submitted but not terminal).
    pub fn in_flight(&self) -> u64 {
        self.total_tasks
            .saturating_sub(self.completed + self.failed)
    }

    /// Success rate in the range `[0.0, 1.0]`.
    pub fn success_rate(&self) -> f64 {
        let terminal = self.completed + self.failed;
        if terminal == 0 {
            return 1.0;
        }
        self.completed as f64 / terminal as f64
    }

    /// Find the busiest worker by tasks_completed count.
    pub fn busiest_worker(&self) -> Option<&WorkerStats> {
        self.workers.iter().max_by_key(|w| w.tasks_completed)
    }

    /// Find the least-loaded worker by tasks_completed count.
    pub fn least_loaded_worker(&self) -> Option<&WorkerStats> {
        self.workers.iter().min_by_key(|w| w.tasks_completed)
    }
}

/// Work distribution policy for the adaptive scheduler.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LoadBalancePolicy {
    /// Assign tasks to workers in a rotating order.
    RoundRobin,
    /// Always assign to the worker with the fewest pending tasks.
    LeastLoaded,
    /// Idle workers steal tasks from overloaded peers.
    WorkStealing,
    /// Highest-priority tasks are dispatched first, regardless of worker.
    PriorityFirst,
}

impl std::fmt::Display for LoadBalancePolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            LoadBalancePolicy::RoundRobin => "round-robin",
            LoadBalancePolicy::LeastLoaded => "least-loaded",
            LoadBalancePolicy::WorkStealing => "work-stealing",
            LoadBalancePolicy::PriorityFirst => "priority-first",
        };
        write!(f, "{}", s)
    }
}

/// Adaptive scheduler that tracks task lifecycle and worker metrics.
pub struct AdaptiveScheduler {
    /// Active load-balance policy.
    pub policy: LoadBalancePolicy,
    /// Per-worker statistics (index = worker id).
    pub workers: Vec<WorkerStats>,
    /// Global scheduler metrics.
    pub metrics: SchedulerMetrics,
    /// All submitted tasks, keyed by TaskId.
    pub(super) tasks: HashMap<TaskId, Task>,
    /// Monotonically increasing task id counter.
    pub(super) next_task_id: u64,
    /// Monotonically increasing clock (simulated nanoseconds).
    pub(super) clock: u64,
    /// Round-robin cursor for `RoundRobin` policy.
    pub(super) rr_cursor: usize,
    /// Accumulated latency for computing the rolling average.
    pub(super) total_latency_ns: u64,
    /// Number of completed tasks that contributed to the latency sum.
    pub(super) latency_sample_count: u64,
}
