//! Adaptive task scheduler implementation and tests.

use std::collections::HashMap;

use super::types::{
    AdaptiveScheduler, LoadBalancePolicy, SchedulerMetrics, Task, TaskId, TaskPriority, TaskState,
    WorkerStats,
};

impl AdaptiveScheduler {
    /// Create a scheduler with `num_workers` workers and the given policy.
    pub fn new(num_workers: usize, policy: LoadBalancePolicy) -> Self {
        let num_workers = num_workers.max(1);
        let workers: Vec<WorkerStats> = (0..num_workers).map(WorkerStats::new).collect();
        let mut metrics = SchedulerMetrics::default();
        metrics.workers = workers.clone();

        AdaptiveScheduler {
            policy,
            workers,
            metrics,
            tasks: HashMap::new(),
            next_task_id: 0,
            clock: 0,
            rr_cursor: 0,
            total_latency_ns: 0,
            latency_sample_count: 0,
        }
    }

    /// Advance the internal simulated clock by `delta_ns` nanoseconds.
    pub fn advance_clock(&mut self, delta_ns: u64) {
        self.clock = self.clock.saturating_add(delta_ns);
    }

    /// Submit a task with the given priority; returns its `TaskId`.
    pub fn submit(&mut self, priority: TaskPriority) -> TaskId {
        let id = TaskId::new(self.next_task_id);
        self.next_task_id += 1;
        let task = Task::new(id, priority, self.clock);
        self.tasks.insert(id, task);
        self.metrics.total_tasks += 1;
        id
    }

    /// Mark a task as completed by the specified worker.
    ///
    /// Updates worker stats and refreshes rolling metrics.
    pub fn complete(&mut self, id: TaskId, worker: usize) {
        if let Some(task) = self.tasks.get_mut(&id) {
            if task.state.is_terminal() {
                return;
            }
            if task.started_at.is_none() {
                task.started_at = Some(self.clock);
            }
            task.completed_at = Some(self.clock);
            task.state = TaskState::Completed;
            self.metrics.completed += 1;

            if let Some(latency) = task.latency_ns() {
                self.total_latency_ns = self.total_latency_ns.saturating_add(latency);
                self.latency_sample_count += 1;
                self.metrics.avg_latency_ns = self.total_latency_ns / self.latency_sample_count;
            }
        }

        if let Some(w) = self.workers.get_mut(worker) {
            w.tasks_completed += 1;
        }
        self.sync_worker_metrics();
        self.update_throughput();
    }

    /// Mark a task as failed with a reason string.
    pub fn fail(&mut self, id: TaskId, reason: &str) {
        if let Some(task) = self.tasks.get_mut(&id) {
            if task.state.is_terminal() {
                return;
            }
            if task.started_at.is_none() {
                task.started_at = Some(self.clock);
            }
            task.completed_at = Some(self.clock);
            task.state = TaskState::Failed(reason.to_owned());
            self.metrics.failed += 1;
        }
        self.sync_worker_metrics();
    }

    /// Cancel a pending task by id.
    ///
    /// Returns `true` if the task was pending and was cancelled.
    pub fn cancel(&mut self, id: TaskId) -> bool {
        if let Some(task) = self.tasks.get_mut(&id) {
            if task.state.is_pending() {
                task.state = TaskState::Cancelled;
                task.completed_at = Some(self.clock);
                return true;
            }
        }
        false
    }

    /// Simulate stealing a task from worker `from` and giving it to worker `to`.
    ///
    /// Picks the lowest-priority pending task nominally owned by `from` (by
    /// scanning the task table for tasks that could be re-assigned) and
    /// reassigns it to `to`.  Returns the `TaskId` of the stolen task, or
    /// `None` if no stealable task was found.
    pub fn steal(&mut self, from: usize, to: usize) -> Option<TaskId> {
        // Find a pending task we attribute to `from` worker (workers complete
        // in order, so we steal the highest-priority pending task from `from`'s
        // nominal share).
        let num_workers = self.workers.len();
        if from >= num_workers || to >= num_workers || from == to {
            return None;
        }

        // Identify pending tasks whose nominal worker (by round-robin modulo)
        // maps to `from`.
        let stolen_id = self
            .tasks
            .iter()
            .filter(|(_, t)| t.state.is_pending())
            .filter(|(id, _)| (id.raw() as usize) % num_workers == from)
            .min_by_key(|(_, t)| t.priority)
            .map(|(id, _)| *id);

        if let Some(id) = stolen_id {
            if let Some(task) = self.tasks.get_mut(&id) {
                task.state = TaskState::Running { worker: to };
                task.started_at = Some(self.clock);
            }
            if let Some(w) = self.workers.get_mut(to) {
                w.tasks_stolen += 1;
            }
            self.sync_worker_metrics();
        }

        stolen_id
    }

    /// Perform adaptive rebalancing: migrate tasks from overloaded workers to underloaded ones.
    ///
    /// This implements a threshold-based policy: if a worker has completed more
    /// than `2 * avg` tasks, it is considered overloaded; workers with fewer
    /// than `avg / 2` completions are candidates to receive stolen tasks.
    pub fn rebalance(&mut self) {
        if self.workers.is_empty() {
            return;
        }

        let total: u64 = self.workers.iter().map(|w| w.tasks_completed).sum();
        let avg = total / self.workers.len() as u64;

        if avg == 0 {
            return;
        }

        let overloaded: Vec<usize> = self
            .workers
            .iter()
            .enumerate()
            .filter(|(_, w)| w.tasks_completed > avg.saturating_mul(2))
            .map(|(i, _)| i)
            .collect();

        let underloaded: Vec<usize> = self
            .workers
            .iter()
            .enumerate()
            .filter(|(_, w)| w.tasks_completed < avg / 2)
            .map(|(i, _)| i)
            .collect();

        // For each overloaded/underloaded pair, attempt a steal.
        let pairs: Vec<(usize, usize)> = overloaded.into_iter().zip(underloaded).collect();

        for (from, to) in pairs {
            self.steal(from, to);
        }

        // Switch policy to WorkStealing if imbalance is severe.
        let ratio = imbalance_ratio(&self.metrics);
        if ratio > 2.0 {
            self.policy = LoadBalancePolicy::WorkStealing;
        }

        self.sync_worker_metrics();
    }

    /// Return a reference to the current scheduler metrics.
    pub fn metrics(&self) -> &SchedulerMetrics {
        &self.metrics
    }

    /// Look up a task by id.
    pub fn get_task(&self, id: TaskId) -> Option<&Task> {
        self.tasks.get(&id)
    }

    /// Number of workers in this scheduler.
    pub fn num_workers(&self) -> usize {
        self.workers.len()
    }

    /// Assign a pending task to a worker according to the current policy,
    /// transitioning it to `Running`.  Returns the assigned worker index,
    /// or `None` if no pending tasks exist.
    pub fn dispatch_next(&mut self) -> Option<(TaskId, usize)> {
        // Pick next pending task according to policy.
        let task_id = match self.policy {
            LoadBalancePolicy::PriorityFirst => self
                .tasks
                .iter()
                .filter(|(_, t)| t.state.is_pending())
                .min_by_key(|(_, t)| t.priority)
                .map(|(id, _)| *id),
            _ => self
                .tasks
                .iter()
                .filter(|(_, t)| t.state.is_pending())
                .min_by_key(|(id, _)| id.raw())
                .map(|(id, _)| *id),
        }?;

        let worker = self.pick_worker();
        if let Some(task) = self.tasks.get_mut(&task_id) {
            task.state = TaskState::Running { worker };
            task.started_at = Some(self.clock);
        }
        self.sync_worker_metrics();
        Some((task_id, worker))
    }

    // --- private helpers ---

    /// Pick a worker index according to the current policy.
    fn pick_worker(&mut self) -> usize {
        match self.policy {
            LoadBalancePolicy::RoundRobin => {
                let w = self.rr_cursor % self.workers.len();
                self.rr_cursor += 1;
                w
            }
            LoadBalancePolicy::LeastLoaded | LoadBalancePolicy::WorkStealing => self
                .workers
                .iter()
                .enumerate()
                .min_by_key(|(_, w)| w.tasks_completed)
                .map(|(i, _)| i)
                .unwrap_or(0),
            LoadBalancePolicy::PriorityFirst => {
                // Least-loaded for priority-based dispatch.
                self.workers
                    .iter()
                    .enumerate()
                    .min_by_key(|(_, w)| w.tasks_completed)
                    .map(|(i, _)| i)
                    .unwrap_or(0)
            }
        }
    }

    /// Sync `metrics.workers` with the authoritative `self.workers` vec.
    fn sync_worker_metrics(&mut self) {
        self.metrics.workers = self.workers.clone();
    }

    /// Recompute throughput estimate based on total completions and current clock.
    fn update_throughput(&mut self) {
        if self.clock == 0 {
            return;
        }
        let secs = self.clock as f64 / 1_000_000_000.0;
        self.metrics.throughput_per_sec = self.metrics.completed as f64 / secs;
    }
}

/// Suggest an optimal worker count based on current scheduler metrics.
///
/// Heuristic:
/// - If the success rate is below 90%, increase workers by 25%.
/// - If imbalance ratio > 3, add workers proportionally.
/// - If all workers are idle (no completed tasks), return the current count.
/// - Otherwise clamp to the range `[1, 512]`.
pub fn suggest_worker_count(current_metrics: &SchedulerMetrics) -> usize {
    let n = current_metrics.workers.len().max(1);
    let ratio = imbalance_ratio(current_metrics);

    let mut suggestion = n;

    if current_metrics.success_rate() < 0.9 {
        // Add 25% more capacity.
        suggestion = (n as f64 * 1.25).ceil() as usize;
    }

    if ratio > 3.0 {
        // Scale proportionally to the imbalance.
        let scale = (ratio / 2.0).ceil() as usize;
        suggestion = suggestion.max(n + scale);
    }

    suggestion.clamp(1, 512)
}

/// Compute the max/min task completion ratio across all workers.
///
/// Returns `1.0` when all workers have the same load or there are fewer than
/// two workers.  Higher values indicate more severe imbalance.
pub fn imbalance_ratio(metrics: &SchedulerMetrics) -> f64 {
    if metrics.workers.len() < 2 {
        return 1.0;
    }
    let max_load = metrics
        .workers
        .iter()
        .map(|w| w.tasks_completed)
        .max()
        .unwrap_or(0);
    let min_load = metrics
        .workers
        .iter()
        .map(|w| w.tasks_completed)
        .min()
        .unwrap_or(0);

    if min_load == 0 {
        if max_load == 0 {
            return 1.0;
        }
        // Treat zero-load workers as having load 1 to avoid division by zero.
        return max_load as f64;
    }
    max_load as f64 / min_load as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_scheduler(n: usize) -> AdaptiveScheduler {
        AdaptiveScheduler::new(n, LoadBalancePolicy::RoundRobin)
    }

    // --- TaskId tests ---

    #[test]
    fn test_task_id_display() {
        let id = TaskId::new(7);
        assert_eq!(format!("{}", id), "task#7");
    }

    #[test]
    fn test_task_id_ord() {
        assert!(TaskId::new(1) < TaskId::new(2));
    }

    // --- TaskPriority tests ---

    #[test]
    fn test_task_priority_level() {
        assert_eq!(TaskPriority::Critical.level(), 0);
        assert_eq!(TaskPriority::Background.level(), 4);
    }

    #[test]
    fn test_task_priority_from_level() {
        assert_eq!(TaskPriority::from_level(0), TaskPriority::Critical);
        assert_eq!(TaskPriority::from_level(99), TaskPriority::Background);
    }

    #[test]
    fn test_task_priority_ordering() {
        assert!(TaskPriority::Critical < TaskPriority::Low);
        assert!(TaskPriority::High < TaskPriority::Normal);
    }

    #[test]
    fn test_task_priority_display() {
        assert_eq!(format!("{}", TaskPriority::Normal), "normal");
    }

    // --- TaskState tests ---

    #[test]
    fn test_task_state_is_terminal() {
        assert!(TaskState::Completed.is_terminal());
        assert!(TaskState::Failed("e".into()).is_terminal());
        assert!(TaskState::Cancelled.is_terminal());
        assert!(!TaskState::Pending.is_terminal());
        assert!(!TaskState::Running { worker: 0 }.is_terminal());
    }

    #[test]
    fn test_task_state_is_pending() {
        assert!(TaskState::Pending.is_pending());
        assert!(!TaskState::Completed.is_pending());
    }

    #[test]
    fn test_task_state_is_running() {
        assert!(TaskState::Running { worker: 2 }.is_running());
        assert!(!TaskState::Pending.is_running());
    }

    #[test]
    fn test_task_state_display() {
        assert_eq!(
            format!("{}", TaskState::Running { worker: 1 }),
            "running(worker=1)"
        );
    }

    // --- Task tests ---

    #[test]
    fn test_task_latency_ns() {
        let mut t = Task::new(TaskId::new(0), TaskPriority::Normal, 100);
        t.started_at = Some(150);
        t.completed_at = Some(300);
        assert_eq!(t.latency_ns(), Some(200));
        assert_eq!(t.queue_delay_ns(), Some(50));
        assert_eq!(t.execution_ns(), Some(150));
    }

    #[test]
    fn test_task_no_latency_before_completion() {
        let t = Task::new(TaskId::new(1), TaskPriority::High, 0);
        assert_eq!(t.latency_ns(), None);
        assert_eq!(t.execution_ns(), None);
    }

    // --- WorkerStats tests ---

    #[test]
    fn test_worker_stats_utilization_zero() {
        let w = WorkerStats::new(0);
        assert_eq!(w.utilization(), 0.0);
    }

    #[test]
    fn test_worker_stats_utilization_half() {
        let mut w = WorkerStats::new(0);
        w.busy_time_ns = 50;
        w.idle_time_ns = 50;
        assert!((w.utilization() - 0.5).abs() < 1e-9);
    }

    #[test]
    fn test_worker_stats_total_tasks() {
        let mut w = WorkerStats::new(0);
        w.tasks_completed = 3;
        w.tasks_stolen = 2;
        assert_eq!(w.total_tasks(), 5);
    }

    // --- SchedulerMetrics tests ---

    #[test]
    fn test_metrics_success_rate_all_success() {
        let mut m = SchedulerMetrics::default();
        m.completed = 10;
        assert_eq!(m.success_rate(), 1.0);
    }

    #[test]
    fn test_metrics_success_rate_mixed() {
        let mut m = SchedulerMetrics::default();
        m.completed = 8;
        m.failed = 2;
        assert!((m.success_rate() - 0.8).abs() < 1e-9);
    }

    #[test]
    fn test_metrics_in_flight() {
        let mut m = SchedulerMetrics::default();
        m.total_tasks = 10;
        m.completed = 7;
        m.failed = 1;
        assert_eq!(m.in_flight(), 2);
    }

    // --- AdaptiveScheduler::new tests ---

    #[test]
    fn test_scheduler_new_basic() {
        let s = make_scheduler(4);
        assert_eq!(s.num_workers(), 4);
        assert_eq!(s.metrics().total_tasks, 0);
    }

    #[test]
    fn test_scheduler_new_min_workers() {
        let s = AdaptiveScheduler::new(0, LoadBalancePolicy::LeastLoaded);
        assert_eq!(s.num_workers(), 1);
    }

    // --- submit tests ---

    #[test]
    fn test_submit_increments_total() {
        let mut s = make_scheduler(2);
        let id = s.submit(TaskPriority::Normal);
        assert_eq!(s.metrics().total_tasks, 1);
        assert_eq!(id.raw(), 0);
    }

    #[test]
    fn test_submit_multiple_unique_ids() {
        let mut s = make_scheduler(2);
        let a = s.submit(TaskPriority::High);
        let b = s.submit(TaskPriority::Low);
        assert_ne!(a, b);
    }

    // --- complete tests ---

    #[test]
    fn test_complete_updates_metrics() {
        let mut s = make_scheduler(2);
        let id = s.submit(TaskPriority::Normal);
        s.advance_clock(1_000_000);
        s.complete(id, 0);
        assert_eq!(s.metrics().completed, 1);
        assert_eq!(s.workers[0].tasks_completed, 1);
    }

    #[test]
    fn test_complete_idempotent() {
        let mut s = make_scheduler(2);
        let id = s.submit(TaskPriority::Normal);
        s.complete(id, 0);
        s.complete(id, 1); // second call is a no-op
        assert_eq!(s.metrics().completed, 1);
    }

    #[test]
    fn test_complete_latency_recorded() {
        let mut s = make_scheduler(2);
        let id = s.submit(TaskPriority::Normal);
        s.advance_clock(500_000_000);
        s.complete(id, 0);
        assert!(s.metrics().avg_latency_ns > 0);
    }

    // --- fail tests ---

    #[test]
    fn test_fail_updates_metrics() {
        let mut s = make_scheduler(2);
        let id = s.submit(TaskPriority::High);
        s.fail(id, "timeout");
        assert_eq!(s.metrics().failed, 1);
        let task = s.get_task(id).expect("task must exist");
        assert!(matches!(task.state, TaskState::Failed(_)));
    }

    #[test]
    fn test_fail_idempotent() {
        let mut s = make_scheduler(2);
        let id = s.submit(TaskPriority::High);
        s.fail(id, "e1");
        s.fail(id, "e2"); // no-op
        assert_eq!(s.metrics().failed, 1);
    }

    // --- cancel tests ---

    #[test]
    fn test_cancel_pending_task() {
        let mut s = make_scheduler(2);
        let id = s.submit(TaskPriority::Low);
        assert!(s.cancel(id));
        let task = s.get_task(id).expect("task must exist");
        assert_eq!(task.state, TaskState::Cancelled);
    }

    #[test]
    fn test_cancel_running_task_fails() {
        let mut s = make_scheduler(2);
        let id = s.submit(TaskPriority::Low);
        s.complete(id, 0);
        assert!(!s.cancel(id)); // already completed
    }

    // --- steal tests ---

    #[test]
    fn test_steal_basic() {
        let mut s = AdaptiveScheduler::new(2, LoadBalancePolicy::WorkStealing);
        // Submit two tasks for worker 0 (id 0 % 2 == 0).
        s.submit(TaskPriority::Normal); // id=0, maps to worker 0
        s.submit(TaskPriority::Normal); // id=1, maps to worker 1
                                        // Submit another for worker 0.
        s.submit(TaskPriority::Normal); // id=2, maps to worker 0

        // Steal from worker 0 to worker 1.
        let stolen = s.steal(0, 1);
        // At least one task should be stolen (id 0 or 2 maps to worker 0).
        assert!(stolen.is_some());
        assert_eq!(s.workers[1].tasks_stolen, 1);
    }

    #[test]
    fn test_steal_same_worker_returns_none() {
        let mut s = make_scheduler(3);
        s.submit(TaskPriority::Normal);
        assert_eq!(s.steal(0, 0), None);
    }

    #[test]
    fn test_steal_invalid_worker_returns_none() {
        let mut s = make_scheduler(2);
        assert_eq!(s.steal(5, 0), None);
    }

    // --- dispatch_next tests ---

    #[test]
    fn test_dispatch_next_picks_pending() {
        let mut s = make_scheduler(2);
        let id = s.submit(TaskPriority::Normal);
        let result = s.dispatch_next();
        assert!(result.is_some());
        let (dispatched_id, _) = result.expect("dispatch must return a result");
        assert_eq!(dispatched_id, id);
        let task = s.get_task(id).expect("task must exist");
        assert!(task.state.is_running());
    }

    #[test]
    fn test_dispatch_next_priority_first() {
        let mut s = AdaptiveScheduler::new(2, LoadBalancePolicy::PriorityFirst);
        let _low = s.submit(TaskPriority::Background);
        let high = s.submit(TaskPriority::Critical);
        let result = s.dispatch_next();
        let (dispatched_id, _) = result.expect("dispatch must return a result");
        // Critical task should be dispatched first.
        assert_eq!(dispatched_id, high);
    }

    // --- rebalance tests ---

    #[test]
    fn test_rebalance_no_crash_empty() {
        let mut s = AdaptiveScheduler::new(1, LoadBalancePolicy::WorkStealing);
        s.rebalance(); // should not panic
    }

    #[test]
    fn test_rebalance_switches_policy_on_severe_imbalance() {
        let mut s = AdaptiveScheduler::new(2, LoadBalancePolicy::RoundRobin);
        // Manually create severe imbalance.
        s.workers[0].tasks_completed = 100;
        s.workers[1].tasks_completed = 1;
        s.metrics.workers = s.workers.clone();
        s.rebalance();
        assert_eq!(s.policy, LoadBalancePolicy::WorkStealing);
    }

    // --- suggest_worker_count tests ---

    #[test]
    fn test_suggest_worker_count_balanced() {
        let mut m = SchedulerMetrics::default();
        m.workers = (0..4).map(WorkerStats::new).collect();
        m.completed = 100;
        let n = suggest_worker_count(&m);
        assert!(n >= 4);
    }

    #[test]
    fn test_suggest_worker_count_low_success_rate() {
        let mut m = SchedulerMetrics::default();
        m.workers = (0..4).map(WorkerStats::new).collect();
        m.completed = 5;
        m.failed = 100; // very low success rate
        let n = suggest_worker_count(&m);
        assert!(n >= 4); // should suggest at least current or more
    }

    // --- imbalance_ratio tests ---

    #[test]
    fn test_imbalance_ratio_balanced() {
        let mut m = SchedulerMetrics::default();
        let mut w0 = WorkerStats::new(0);
        let mut w1 = WorkerStats::new(1);
        w0.tasks_completed = 10;
        w1.tasks_completed = 10;
        m.workers = vec![w0, w1];
        assert!((imbalance_ratio(&m) - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_imbalance_ratio_imbalanced() {
        let mut m = SchedulerMetrics::default();
        let mut w0 = WorkerStats::new(0);
        let mut w1 = WorkerStats::new(1);
        w0.tasks_completed = 40;
        w1.tasks_completed = 10;
        m.workers = vec![w0, w1];
        assert!((imbalance_ratio(&m) - 4.0).abs() < 1e-9);
    }

    #[test]
    fn test_imbalance_ratio_single_worker() {
        let mut m = SchedulerMetrics::default();
        m.workers = vec![WorkerStats::new(0)];
        assert_eq!(imbalance_ratio(&m), 1.0);
    }

    #[test]
    fn test_imbalance_ratio_one_idle() {
        let mut m = SchedulerMetrics::default();
        let mut w0 = WorkerStats::new(0);
        w0.tasks_completed = 20;
        m.workers = vec![w0, WorkerStats::new(1)];
        // min_load == 0, so ratio = max_load as f64
        assert!((imbalance_ratio(&m) - 20.0).abs() < 1e-9);
    }

    // --- throughput ---

    #[test]
    fn test_throughput_nonzero_after_completions() {
        let mut s = make_scheduler(2);
        s.submit(TaskPriority::Normal);
        let id = s.submit(TaskPriority::Normal);
        s.advance_clock(1_000_000_000); // 1 second
        s.complete(id, 0);
        assert!(s.metrics().throughput_per_sec > 0.0);
    }

    // --- LoadBalancePolicy display ---

    #[test]
    fn test_policy_display() {
        assert_eq!(format!("{}", LoadBalancePolicy::RoundRobin), "round-robin");
        assert_eq!(
            format!("{}", LoadBalancePolicy::WorkStealing),
            "work-stealing"
        );
    }
}
