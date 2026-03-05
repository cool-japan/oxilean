//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::time::{Duration, Instant};

/// Tracks tasks that have timeouts and detects expiry.
pub struct TimeoutManager {
    /// item_id → (started_at, timeout)
    active: HashMap<u64, (Instant, Duration)>,
}
impl TimeoutManager {
    /// Create a new timeout manager.
    pub fn new() -> Self {
        Self { active: HashMap::new() }
    }
    /// Register a task with its start time and timeout duration.
    pub fn register(&mut self, item_id: u64, timeout: Duration) {
        self.active.insert(item_id, (Instant::now(), timeout));
    }
    /// Remove a task from tracking (completed or cancelled).
    pub fn deregister(&mut self, item_id: u64) {
        self.active.remove(&item_id);
    }
    /// Return all item_ids whose timeout has expired.
    pub fn expired(&self) -> Vec<u64> {
        self.active
            .iter()
            .filter(|(_, (started, timeout))| started.elapsed() >= *timeout)
            .map(|(&id, _)| id)
            .collect()
    }
    /// Number of tasks currently tracked.
    pub fn tracked_count(&self) -> usize {
        self.active.len()
    }
    /// Remaining time for an item (None if not tracked or already expired).
    pub fn remaining(&self, item_id: u64) -> Option<Duration> {
        self.active
            .get(&item_id)
            .and_then(|(started, timeout)| { timeout.checked_sub(started.elapsed()) })
    }
}
/// Detects cycles in the task dependency graph (deadlock detection).
pub struct DeadlockDetector {
    /// adjacency list: item_id → set of dependency item_ids
    deps: HashMap<u64, HashSet<u64>>,
}
impl DeadlockDetector {
    /// Create a new detector.
    pub fn new() -> Self {
        Self { deps: HashMap::new() }
    }
    /// Register item dependencies.
    pub fn register(&mut self, item_id: u64, dep_ids: &[u64]) {
        let entry = self.deps.entry(item_id).or_default();
        for &d in dep_ids {
            entry.insert(d);
        }
        for &d in dep_ids {
            self.deps.entry(d).or_default();
        }
    }
    /// Register all items from a slice.
    pub fn register_items(&mut self, items: &[WorkItem]) {
        for item in items {
            self.register(item.id, &item.deps);
        }
    }
    /// Detect a cycle using DFS. Returns a cycle path if one is found.
    pub fn detect_cycle(&self) -> Option<Vec<u64>> {
        let mut visited: HashSet<u64> = HashSet::new();
        let mut stack: HashSet<u64> = HashSet::new();
        let mut path: Vec<u64> = Vec::new();
        for &id in self.deps.keys() {
            if !visited.contains(&id) {
                if let Some(cycle) = self
                    .dfs_cycle(id, &mut visited, &mut stack, &mut path)
                {
                    return Some(cycle);
                }
            }
        }
        None
    }
    fn dfs_cycle(
        &self,
        node: u64,
        visited: &mut HashSet<u64>,
        stack: &mut HashSet<u64>,
        path: &mut Vec<u64>,
    ) -> Option<Vec<u64>> {
        visited.insert(node);
        stack.insert(node);
        path.push(node);
        if let Some(neighbors) = self.deps.get(&node) {
            for &next in neighbors {
                if !visited.contains(&next) {
                    if let Some(cycle) = self.dfs_cycle(next, visited, stack, path) {
                        return Some(cycle);
                    }
                } else if stack.contains(&next) {
                    let start = path.iter().position(|&x| x == next).unwrap_or(0);
                    let mut cycle = path[start..].to_vec();
                    cycle.push(next);
                    return Some(cycle);
                }
            }
        }
        stack.remove(&node);
        path.pop();
        None
    }
    /// Check whether the dependency graph is a valid DAG (no cycles).
    pub fn is_acyclic(&self) -> bool {
        self.detect_cycle().is_none()
    }
    /// Topological sort of registered items (Kahn's algorithm).
    ///
    /// Returns `Err` with a cycle path if a cycle is detected.
    pub fn topological_sort(&self) -> Result<Vec<u64>, Vec<u64>> {
        let mut in_deg: HashMap<u64, usize> = self
            .deps
            .keys()
            .map(|&id| (id, 0))
            .collect();
        for (_, deps) in &self.deps {
            for &d in deps {
                *in_deg.entry(d).or_insert(0) += 0;
            }
        }
        for (&id, deps) in &self.deps {
            let _ = id;
            for &d in deps {
                let _ = d;
            }
        }
        let mut in_deg2: HashMap<u64, usize> = self
            .deps
            .keys()
            .map(|&id| (id, 0))
            .collect();
        for (&id, deps) in &self.deps {
            in_deg2.insert(id, deps.len());
        }
        let mut queue: VecDeque<u64> = in_deg2
            .iter()
            .filter(|(_, &d)| d == 0)
            .map(|(&id, _)| id)
            .collect();
        let mut children: HashMap<u64, Vec<u64>> = self
            .deps
            .keys()
            .map(|&id| (id, vec![]))
            .collect();
        for (&id, deps) in &self.deps {
            for &d in deps {
                children.entry(d).or_default().push(id);
            }
        }
        let mut result = Vec::new();
        while let Some(id) = queue.pop_front() {
            result.push(id);
            if let Some(succs) = children.get(&id) {
                for &s in succs {
                    if let Some(d) = in_deg2.get_mut(&s) {
                        *d = d.saturating_sub(1);
                        if *d == 0 {
                            queue.push_back(s);
                        }
                    }
                }
            }
        }
        if result.len() == self.deps.len() {
            Ok(result)
        } else {
            Err(self.detect_cycle().unwrap_or_default())
        }
    }
}
/// The current state of a worker.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WorkerState {
    /// Not executing anything.
    Idle,
    /// Executing a work item with the given id.
    Busy(u64),
    /// Has encountered a failure.
    Failed,
}
/// The algorithm used to pick the next task from the ready queue.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SchedulingStrategy {
    /// Highest priority first; ties broken by lowest id.
    PriorityFirst,
    /// Shortest job first (lowest estimated cost).
    ShortestJobFirst,
    /// Longest job first (highest estimated cost) — minimizes tail latency.
    LongestJobFirst,
    /// Critical-path first (items on the longest path run first).
    CriticalPath,
    /// Round-robin across workers.
    RoundRobin,
}
/// An event emitted by the scheduler during a run.
#[derive(Clone, Debug)]
pub enum SchedulerEvent {
    /// A task was submitted.
    TaskSubmitted { item_id: u64, name: String },
    /// A task was assigned to a worker.
    TaskStarted { item_id: u64, worker_id: usize, name: String },
    /// A task completed successfully.
    TaskSucceeded { item_id: u64, worker_id: usize, elapsed_ms: u64 },
    /// A task failed.
    TaskFailed { item_id: u64, exit_code: i32, elapsed_ms: u64 },
    /// A task was cancelled.
    TaskCancelled { item_id: u64 },
    /// A task timed out.
    TaskTimedOut { item_id: u64, timeout_ms: u64 },
    /// All tasks have completed.
    AllDone { total_ms: u64 },
}
/// A scheduler that dynamically adjusts its worker count based on observed
/// throughput and system load.
pub struct AdaptiveScheduler {
    inner: ParallelScheduler,
    min_workers: usize,
    max_workers: usize,
    /// History of throughput samples (items/tick).
    throughput_history: VecDeque<f64>,
    /// Number of samples to keep.
    history_capacity: usize,
    /// Current logical worker count (may be lower than max).
    current_workers: usize,
}
impl AdaptiveScheduler {
    /// Create a new adaptive scheduler.
    pub fn new(min_workers: usize, max_workers: usize) -> Self {
        let start = min_workers.max(1);
        Self {
            inner: ParallelScheduler::new(start),
            min_workers: start,
            max_workers: max_workers.max(start),
            throughput_history: VecDeque::new(),
            history_capacity: 16,
            current_workers: start,
        }
    }
    /// Submit a work item.
    pub fn submit(&mut self, item: WorkItem) {
        self.inner.submit(item);
    }
    /// Record a throughput sample (items completed per unit time).
    pub fn record_throughput(&mut self, items_per_tick: f64) {
        if self.throughput_history.len() >= self.history_capacity {
            self.throughput_history.pop_front();
        }
        self.throughput_history.push_back(items_per_tick);
    }
    /// Average of recent throughput samples.
    pub fn avg_throughput(&self) -> f64 {
        if self.throughput_history.is_empty() {
            return 0.0;
        }
        let sum: f64 = self.throughput_history.iter().sum();
        sum / self.throughput_history.len() as f64
    }
    /// Suggest an adjusted worker count based on throughput trend.
    ///
    /// Increases workers if throughput is growing, decreases if declining.
    pub fn suggest_worker_count(&self) -> usize {
        if self.throughput_history.len() < 2 {
            return self.current_workers;
        }
        let recent = self.throughput_history.back().copied().unwrap_or(0.0);
        let older = self.throughput_history.front().copied().unwrap_or(0.0);
        if recent > older * 1.1 {
            (self.current_workers + 1).min(self.max_workers)
        } else if recent < older * 0.9 && self.current_workers > self.min_workers {
            (self.current_workers - 1).max(self.min_workers)
        } else {
            self.current_workers
        }
    }
    /// Apply the suggested worker count adjustment.
    pub fn adapt(&mut self) {
        self.current_workers = self.suggest_worker_count();
    }
    /// Current effective worker count.
    pub fn current_workers(&self) -> usize {
        self.current_workers
    }
    /// Schedule the next task.
    pub fn schedule_next(&mut self) -> Option<(usize, u64)> {
        self.inner.schedule_next()
    }
    /// Mark a task complete.
    pub fn complete_item(&mut self, worker_id: usize, item_id: u64) {
        self.inner.complete_item(worker_id, item_id);
    }
    /// Whether all tasks are done.
    pub fn is_complete(&self) -> bool {
        self.inner.is_complete()
    }
    /// Scheduler statistics.
    pub fn stats(&self) -> SchedulerStats {
        self.inner.stats()
    }
}
/// Computes the critical path through a dependency DAG.
pub struct CriticalPathScheduler;
impl CriticalPathScheduler {
    /// Create a new critical-path scheduler.
    pub fn new() -> Self {
        Self
    }
    /// Compute the IDs of items on the critical (longest) path.
    ///
    /// Uses a longest-path DP on the DAG (requires acyclicity).
    pub fn compute_critical_path(&self, items: &[WorkItem]) -> Vec<u64> {
        if items.is_empty() {
            return Vec::new();
        }
        let id_to_item: HashMap<u64, &WorkItem> = items
            .iter()
            .map(|i| (i.id, i))
            .collect();
        let mut dp: HashMap<u64, (u64, Option<u64>)> = HashMap::new();
        let mut in_deg: HashMap<u64, usize> = items
            .iter()
            .map(|i| (i.id, i.deps.len()))
            .collect();
        let mut queue: VecDeque<u64> = in_deg
            .iter()
            .filter(|(_, &d)| d == 0)
            .map(|(id, _)| *id)
            .collect();
        let mut children: HashMap<u64, Vec<u64>> = items
            .iter()
            .map(|i| (i.id, vec![]))
            .collect();
        for item in items {
            for &dep in &item.deps {
                children.entry(dep).or_default().push(item.id);
            }
        }
        while let Some(id) = queue.pop_front() {
            let item = match id_to_item.get(&id) {
                Some(i) => *i,
                None => continue,
            };
            let best_pred_finish = item
                .deps
                .iter()
                .filter_map(|d| dp.get(d).map(|(f, _)| (*f, *d)))
                .max_by_key(|(f, _)| *f);
            let (pred_finish, pred_id) = match best_pred_finish {
                Some((f, pid)) => (f, Some(pid)),
                None => (0, None),
            };
            dp.insert(id, (pred_finish + item.estimated_cost, pred_id));
            if let Some(succs) = children.get(&id) {
                for &s in succs {
                    if let Some(d) = in_deg.get_mut(&s) {
                        *d = d.saturating_sub(1);
                        if *d == 0 {
                            queue.push_back(s);
                        }
                    }
                }
            }
        }
        let Some((&end_id, _)) = dp.iter().max_by_key(|(_, (f, _))| *f) else {
            return Vec::new();
        };
        let mut path = Vec::new();
        let mut cur = end_id;
        loop {
            path.push(cur);
            match dp.get(&cur).and_then(|(_, p)| *p) {
                Some(pred) => cur = pred,
                None => break,
            }
        }
        path.reverse();
        path
    }
    /// Estimate total wall-clock time for a set of items with `num_workers` workers.
    pub fn estimated_total_time(&self, items: &[WorkItem], num_workers: usize) -> u64 {
        if items.is_empty() || num_workers == 0 {
            return 0;
        }
        let total_cost: u64 = items.iter().map(|i| i.estimated_cost).sum();
        let critical_path_cost: u64 = self
            .compute_critical_path(items)
            .iter()
            .filter_map(|id| items.iter().find(|i| i.id == *id))
            .map(|i| i.estimated_cost)
            .sum();
        let parallel_estimate = (total_cost + num_workers as u64 - 1)
            / num_workers as u64;
        critical_path_cost.max(parallel_estimate)
    }
}
/// Priority level for a work item.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    /// Background priority (lowest).
    Background,
    /// Low priority.
    Low,
    /// Normal priority.
    Normal,
    /// High priority.
    High,
    /// Critical priority (highest).
    Critical,
}
/// An ordered log of scheduler events.
pub struct EventLog {
    events: Vec<(Instant, SchedulerEvent)>,
}
impl EventLog {
    /// Create an empty event log.
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }
    /// Append an event with the current timestamp.
    pub fn push(&mut self, event: SchedulerEvent) {
        self.events.push((Instant::now(), event));
    }
    /// Number of recorded events.
    pub fn len(&self) -> usize {
        self.events.len()
    }
    /// Whether the log is empty.
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
    /// Iterate over events (without timestamps).
    pub fn iter_events(&self) -> impl Iterator<Item = &SchedulerEvent> {
        self.events.iter().map(|(_, e)| e)
    }
    /// All events matching a given kind (by string prefix).
    pub fn filter_by_kind(&self, kind: &str) -> Vec<&SchedulerEvent> {
        self.events
            .iter()
            .filter(|(_, e)| format!("{}", e).starts_with(kind))
            .map(|(_, e)| e)
            .collect()
    }
    /// Summarise: how many of each event type.
    pub fn summary(&self) -> BTreeMap<String, usize> {
        let mut map = BTreeMap::new();
        for (_, event) in &self.events {
            let key = match event {
                SchedulerEvent::TaskSubmitted { .. } => "submitted",
                SchedulerEvent::TaskStarted { .. } => "started",
                SchedulerEvent::TaskSucceeded { .. } => "succeeded",
                SchedulerEvent::TaskFailed { .. } => "failed",
                SchedulerEvent::TaskCancelled { .. } => "cancelled",
                SchedulerEvent::TaskTimedOut { .. } => "timed_out",
                SchedulerEvent::AllDone { .. } => "all_done",
            };
            *map.entry(key.to_string()).or_insert(0) += 1;
        }
        map
    }
}
/// Statistics from the parallel scheduler.
#[derive(Clone, Debug)]
pub struct SchedulerStats {
    /// Total number of submitted items.
    pub total_items: usize,
    /// Number of completed items.
    pub completed: usize,
    /// Average time an item waited before being scheduled (ms).
    pub avg_wait_time_ms: f64,
    /// Ratio of time workers were busy vs total available time.
    pub parallelism_ratio: f64,
}
/// A min-heap priority queue for work items (lowest cost = highest urgency).
pub struct MinCostQueue {
    heap: Vec<WorkItem>,
}
impl MinCostQueue {
    /// Create an empty queue.
    pub fn new() -> Self {
        Self { heap: Vec::new() }
    }
    /// Insert an item.
    pub fn push(&mut self, item: WorkItem) {
        self.heap.push(item);
        self.sift_up(self.heap.len() - 1);
    }
    /// Remove and return the lowest-cost item.
    pub fn pop_min(&mut self) -> Option<WorkItem> {
        if self.heap.is_empty() {
            return None;
        }
        let last = self.heap.len() - 1;
        self.heap.swap(0, last);
        let item = self.heap.pop();
        if !self.heap.is_empty() {
            self.sift_down(0);
        }
        item
    }
    /// Peek at the minimum-cost item without removing it.
    pub fn peek_min(&self) -> Option<&WorkItem> {
        self.heap.first()
    }
    /// Number of items.
    pub fn len(&self) -> usize {
        self.heap.len()
    }
    /// Whether empty.
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }
    fn sift_up(&mut self, mut i: usize) {
        while i > 0 {
            let parent = (i - 1) / 2;
            if self.heap[i].estimated_cost < self.heap[parent].estimated_cost {
                self.heap.swap(i, parent);
                i = parent;
            } else {
                break;
            }
        }
    }
    fn sift_down(&mut self, mut i: usize) {
        let n = self.heap.len();
        loop {
            let left = 2 * i + 1;
            let right = 2 * i + 2;
            let mut smallest = i;
            if left < n
                && self.heap[left].estimated_cost < self.heap[smallest].estimated_cost
            {
                smallest = left;
            }
            if right < n
                && self.heap[right].estimated_cost < self.heap[smallest].estimated_cost
            {
                smallest = right;
            }
            if smallest == i {
                break;
            }
            self.heap.swap(i, smallest);
            i = smallest;
        }
    }
}
/// A worker that executes work items.
#[derive(Clone, Debug)]
pub struct Worker {
    /// Worker index.
    pub id: usize,
    /// Current state.
    pub state: WorkerState,
    /// Number of items completed.
    pub completed: usize,
    /// Sum of costs of completed items.
    pub total_cost: u64,
}
impl Worker {
    /// Create a new idle worker.
    pub fn new(id: usize) -> Self {
        Self {
            id,
            state: WorkerState::Idle,
            completed: 0,
            total_cost: 0,
        }
    }
    /// Assign a work item to this worker.
    pub fn assign(&mut self, item_id: u64) {
        self.state = WorkerState::Busy(item_id);
    }
    /// Mark the current item as complete with the given cost.
    pub fn complete(&mut self, cost: u64) {
        self.completed += 1;
        self.total_cost += cost;
        self.state = WorkerState::Idle;
    }
    /// Whether the worker is idle and ready to accept work.
    pub fn is_idle(&self) -> bool {
        self.state == WorkerState::Idle
    }
}
/// A lightweight dependency graph for scheduler-internal use.
pub struct SchedulerDepGraph {
    /// item_id → set of dependency item_ids
    deps: HashMap<u64, HashSet<u64>>,
    /// item_id → set of dependent (successor) item_ids
    succs: HashMap<u64, HashSet<u64>>,
    /// item_id → estimated cost
    costs: HashMap<u64, u64>,
}
impl SchedulerDepGraph {
    /// Create a new empty dependency graph.
    pub fn new() -> Self {
        Self {
            deps: HashMap::new(),
            succs: HashMap::new(),
            costs: HashMap::new(),
        }
    }
    /// Add an item with its cost.
    pub fn add_item(&mut self, id: u64, cost: u64) {
        self.deps.entry(id).or_default();
        self.succs.entry(id).or_default();
        self.costs.insert(id, cost);
    }
    /// Add a dependency edge: `from` depends on `to`.
    pub fn add_dep(&mut self, from: u64, to: u64) {
        self.deps.entry(from).or_default().insert(to);
        self.succs.entry(to).or_default().insert(from);
        self.deps.entry(to).or_default();
        self.succs.entry(from).or_default();
    }
    /// Build from a slice of WorkItems.
    pub fn from_items(items: &[WorkItem]) -> Self {
        let mut g = Self::new();
        for item in items {
            g.add_item(item.id, item.estimated_cost);
        }
        for item in items {
            for &dep in &item.deps {
                g.add_dep(item.id, dep);
            }
        }
        g
    }
    /// Items with no unsatisfied dependencies (sources in the DAG).
    pub fn sources(&self) -> Vec<u64> {
        self.deps.iter().filter(|(_, deps)| deps.is_empty()).map(|(&id, _)| id).collect()
    }
    /// Items with no successors (sinks in the DAG).
    pub fn sinks(&self) -> Vec<u64> {
        self.succs
            .iter()
            .filter(|(_, succs)| succs.is_empty())
            .map(|(&id, _)| id)
            .collect()
    }
    /// Compute the longest path (critical path) cost.
    pub fn critical_path_cost(&self) -> u64 {
        let mut dp: HashMap<u64, u64> = HashMap::new();
        let topo = self.topological_order();
        for &id in &topo {
            let own = *self.costs.get(&id).unwrap_or(&1);
            let best_pred = self
                .deps
                .get(&id)
                .map(|deps| {
                    deps.iter().filter_map(|d| dp.get(d)).copied().max().unwrap_or(0)
                })
                .unwrap_or(0);
            dp.insert(id, best_pred + own);
        }
        dp.values().copied().max().unwrap_or(0)
    }
    fn topological_order(&self) -> Vec<u64> {
        let mut in_deg: HashMap<u64, usize> = self
            .deps
            .iter()
            .map(|(&id, deps)| (id, deps.len()))
            .collect();
        let mut queue: VecDeque<u64> = in_deg
            .iter()
            .filter(|(_, &d)| d == 0)
            .map(|(&id, _)| id)
            .collect();
        let mut order = Vec::new();
        while let Some(id) = queue.pop_front() {
            order.push(id);
            if let Some(succs) = self.succs.get(&id) {
                for &s in succs {
                    if let Some(d) = in_deg.get_mut(&s) {
                        *d = d.saturating_sub(1);
                        if *d == 0 {
                            queue.push_back(s);
                        }
                    }
                }
            }
        }
        order
    }
    /// Number of nodes.
    pub fn node_count(&self) -> usize {
        self.deps.len()
    }
    /// Number of edges.
    pub fn edge_count(&self) -> usize {
        self.deps.values().map(|s| s.len()).sum()
    }
}
/// A priority-based parallel scheduler for work items.
pub struct ParallelScheduler {
    workers: Vec<Worker>,
    pending: Vec<WorkItem>,
    completed_ids: HashSet<u64>,
    in_progress: HashMap<u64, usize>,
    total_submitted: usize,
    total_completed: usize,
}
impl ParallelScheduler {
    /// Create a new scheduler with the given number of workers.
    pub fn new(num_workers: usize) -> Self {
        let workers = (0..num_workers.max(1)).map(Worker::new).collect();
        Self {
            workers,
            pending: Vec::new(),
            completed_ids: HashSet::new(),
            in_progress: HashMap::new(),
            total_submitted: 0,
            total_completed: 0,
        }
    }
    /// Submit a work item for scheduling.
    pub fn submit(&mut self, item: WorkItem) {
        self.total_submitted += 1;
        self.pending.push(item);
    }
    /// Schedule the next ready item to an idle worker.
    ///
    /// Returns `(worker_id, item_id)` if an assignment was made.
    pub fn schedule_next(&mut self) -> Option<(usize, u64)> {
        let worker_idx = self.workers.iter().position(|w| w.is_idle())?;
        let ready = self.ready_item_indices();
        if ready.is_empty() {
            return None;
        }
        let best_idx = ready
            .into_iter()
            .max_by(|&a, &b| {
                let ia = &self.pending[a];
                let ib = &self.pending[b];
                ia.priority.cmp(&ib.priority).then(ib.id.cmp(&ia.id))
            })?;
        let item = self.pending.remove(best_idx);
        let item_id = item.id;
        let cost = item.estimated_cost;
        self.workers[worker_idx].assign(item_id);
        self.in_progress.insert(item_id, worker_idx);
        self.pending_costs.insert(item_id, cost);
        Some((worker_idx, item_id))
    }
    /// A helper map to store costs of items currently in progress.
    #[doc(hidden)]
    fn pending_costs_mut(&mut self) -> &mut HashMap<u64, u64> {
        &mut self.pending_costs
    }
    /// Mark a work item as completed by a worker.
    pub fn complete_item(&mut self, worker_id: usize, item_id: u64) {
        if let Some(w) = self.workers.get_mut(worker_id) {
            let cost = self.pending_costs.remove(&item_id).unwrap_or(1);
            w.complete(cost);
        }
        self.in_progress.remove(&item_id);
        self.completed_ids.insert(item_id);
        self.total_completed += 1;
    }
    /// Items in the pending queue whose dependencies are all satisfied.
    pub fn ready_items(&self) -> Vec<&WorkItem> {
        self.pending
            .iter()
            .filter(|item| item.deps.iter().all(|d| self.completed_ids.contains(d)))
            .collect()
    }
    /// Whether all submitted items are complete.
    pub fn is_complete(&self) -> bool {
        self.pending.is_empty() && self.in_progress.is_empty()
    }
    /// Return a snapshot of scheduling statistics.
    pub fn stats(&self) -> SchedulerStats {
        let parallelism_ratio = if self.workers.is_empty() {
            0.0
        } else {
            let busy = self.workers.iter().filter(|w| !w.is_idle()).count();
            busy as f64 / self.workers.len() as f64
        };
        SchedulerStats {
            total_items: self.total_submitted,
            completed: self.total_completed,
            avg_wait_time_ms: 0.0,
            parallelism_ratio,
        }
    }
    fn ready_item_indices(&self) -> Vec<usize> {
        self.pending
            .iter()
            .enumerate()
            .filter(|(_, item)| item.deps.iter().all(|d| self.completed_ids.contains(d)))
            .map(|(i, _)| i)
            .collect()
    }
}
/// CPU and memory resource limits for a worker pool.
#[derive(Clone, Debug)]
pub struct ResourceConstraints {
    /// Maximum number of concurrent workers.
    pub max_workers: usize,
    /// Maximum memory usage in bytes (0 = unlimited).
    pub max_memory_bytes: u64,
    /// Maximum CPU cores allowed (0 = unlimited).
    pub max_cpu_cores: usize,
    /// Minimum free memory required before scheduling a new task (bytes).
    pub min_free_memory_bytes: u64,
}
impl ResourceConstraints {
    /// Create default resource constraints for the given worker count.
    pub fn new(max_workers: usize) -> Self {
        Self {
            max_workers,
            max_memory_bytes: 0,
            max_cpu_cores: 0,
            min_free_memory_bytes: 0,
        }
    }
    /// Set a memory limit.
    pub fn with_memory_limit(mut self, bytes: u64) -> Self {
        self.max_memory_bytes = bytes;
        self
    }
    /// Set a CPU core limit.
    pub fn with_cpu_limit(mut self, cores: usize) -> Self {
        self.max_cpu_cores = cores;
        self
    }
    /// Set a minimum free-memory threshold.
    pub fn with_min_free_memory(mut self, bytes: u64) -> Self {
        self.min_free_memory_bytes = bytes;
        self
    }
    /// Effective worker count: capped by cpu_cores if set.
    pub fn effective_workers(&self) -> usize {
        if self.max_cpu_cores > 0 {
            self.max_workers.min(self.max_cpu_cores)
        } else {
            self.max_workers
        }
    }
}
/// A unit of work to be scheduled.
#[derive(Clone, Debug)]
pub struct WorkItem {
    /// Unique identifier.
    pub id: u64,
    /// Human-readable name.
    pub name: String,
    /// Scheduling priority.
    pub priority: Priority,
    /// Estimated execution cost (arbitrary units, e.g., milliseconds).
    pub estimated_cost: u64,
    /// IDs of work items that must complete before this one.
    pub deps: Vec<u64>,
}
impl WorkItem {
    /// Create a new work item with default priority and cost.
    pub fn new(id: u64, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            priority: Priority::Normal,
            estimated_cost: 1,
            deps: Vec::new(),
        }
    }
    /// Set the priority (builder pattern).
    pub fn with_priority(mut self, p: Priority) -> Self {
        self.priority = p;
        self
    }
    /// Set the estimated cost (builder pattern).
    pub fn with_cost(mut self, cost: u64) -> Self {
        self.estimated_cost = cost;
        self
    }
    /// Add a dependency on another item id.
    pub fn add_dep(&mut self, dep_id: u64) {
        if !self.deps.contains(&dep_id) {
            self.deps.push(dep_id);
        }
    }
}
/// A queue that supports work stealing.
pub struct WorkStealingQueue {
    own: VecDeque<WorkItem>,
}
impl WorkStealingQueue {
    /// Create an empty work-stealing queue.
    pub fn new() -> Self {
        Self { own: VecDeque::new() }
    }
    /// Push a work item onto this queue.
    pub fn push(&mut self, item: WorkItem) {
        self.own.push_back(item);
    }
    /// Pop from the front of this worker's own queue (FIFO).
    pub fn pop(&mut self) -> Option<WorkItem> {
        self.own.pop_front()
    }
    /// Steal from the back of this queue (simulates stealing from another worker).
    pub fn steal(&mut self) -> Option<WorkItem> {
        self.own.pop_back()
    }
    /// Number of items in the queue.
    pub fn len(&self) -> usize {
        self.own.len()
    }
    /// Whether the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.own.is_empty()
    }
}
/// Collects utilization samples during a scheduling run.
pub struct UtilizationCollector {
    start: Instant,
    idle_ticks: u64,
    busy_ticks: u64,
    peak_concurrency: usize,
    items_processed: usize,
    num_workers: usize,
}
impl UtilizationCollector {
    /// Begin collecting with the given worker count.
    pub fn new(num_workers: usize) -> Self {
        Self {
            start: Instant::now(),
            idle_ticks: 0,
            busy_ticks: 0,
            peak_concurrency: 0,
            items_processed: 0,
            num_workers,
        }
    }
    /// Record a scheduling tick. `running` is the number of currently running tasks.
    pub fn tick(&mut self, running: usize) {
        let idle = self.num_workers.saturating_sub(running);
        self.busy_ticks += running as u64;
        self.idle_ticks += idle as u64;
        if running > self.peak_concurrency {
            self.peak_concurrency = running;
        }
    }
    /// Record a task completion.
    pub fn item_done(&mut self) {
        self.items_processed += 1;
    }
    /// Finalize and return the utilization report.
    pub fn finalize(self) -> SchedulerUtilization {
        SchedulerUtilization {
            wall_time: self.start.elapsed(),
            total_busy_time: Duration::from_millis(self.busy_ticks),
            idle_ticks: self.idle_ticks,
            busy_ticks: self.busy_ticks,
            peak_concurrency: self.peak_concurrency,
            items_processed: self.items_processed,
        }
    }
}
/// A scheduler that respects resource constraints before dispatching tasks.
pub struct ResourceAwareScheduler {
    constraints: ResourceConstraints,
    inner: ParallelScheduler,
    current_memory_usage: u64,
    /// Estimated memory cost per task (bytes).
    task_memory_cost: u64,
}
impl ResourceAwareScheduler {
    /// Create a new resource-aware scheduler.
    pub fn new(constraints: ResourceConstraints) -> Self {
        let workers = constraints.effective_workers();
        Self {
            inner: ParallelScheduler::new(workers),
            constraints,
            current_memory_usage: 0,
            task_memory_cost: 64 * 1024 * 1024,
        }
    }
    /// Override the default per-task memory cost.
    pub fn with_task_memory_cost(mut self, bytes: u64) -> Self {
        self.task_memory_cost = bytes;
        self
    }
    /// Submit a work item.
    pub fn submit(&mut self, item: WorkItem) {
        self.inner.submit(item);
    }
    /// Try to schedule the next task, respecting memory limits.
    ///
    /// Returns `None` if no task is ready or memory is insufficient.
    pub fn schedule_next(&mut self) -> Option<(usize, u64)> {
        if self.constraints.max_memory_bytes > 0 {
            let projected = self.current_memory_usage + self.task_memory_cost;
            if projected > self.constraints.max_memory_bytes {
                return None;
            }
        }
        let result = self.inner.schedule_next()?;
        self.current_memory_usage += self.task_memory_cost;
        Some(result)
    }
    /// Mark a task as complete, releasing its memory.
    pub fn complete_item(&mut self, worker_id: usize, item_id: u64) {
        self.inner.complete_item(worker_id, item_id);
        self.current_memory_usage = self
            .current_memory_usage
            .saturating_sub(self.task_memory_cost);
    }
    /// Whether all tasks have completed.
    pub fn is_complete(&self) -> bool {
        self.inner.is_complete()
    }
    /// Current memory usage estimate.
    pub fn current_memory_usage(&self) -> u64 {
        self.current_memory_usage
    }
    /// Scheduler statistics from the inner scheduler.
    pub fn stats(&self) -> SchedulerStats {
        self.inner.stats()
    }
}
/// A managed pool of workers with load-balancing support.
pub struct WorkerPool {
    workers: Vec<Worker>,
    /// Map from item_id to assigned worker index.
    assignments: HashMap<u64, usize>,
}
impl WorkerPool {
    /// Create a pool with the given number of workers.
    pub fn new(n: usize) -> Self {
        let workers = (0..n.max(1)).map(Worker::new).collect();
        Self {
            workers,
            assignments: HashMap::new(),
        }
    }
    /// Number of workers in the pool.
    pub fn size(&self) -> usize {
        self.workers.len()
    }
    /// Find an idle worker, preferring the one with the least total work done.
    pub fn least_loaded_idle(&self) -> Option<usize> {
        self.workers
            .iter()
            .enumerate()
            .filter(|(_, w)| w.is_idle())
            .min_by_key(|(_, w)| w.total_cost)
            .map(|(i, _)| i)
    }
    /// Assign item to a specific worker.
    pub fn assign(&mut self, worker_idx: usize, item_id: u64) {
        if let Some(w) = self.workers.get_mut(worker_idx) {
            w.assign(item_id);
            self.assignments.insert(item_id, worker_idx);
        }
    }
    /// Complete an item on its assigned worker.
    pub fn complete(&mut self, item_id: u64, cost: u64) -> Option<usize> {
        let worker_idx = self.assignments.remove(&item_id)?;
        if let Some(w) = self.workers.get_mut(worker_idx) {
            w.complete(cost);
        }
        Some(worker_idx)
    }
    /// Number of idle workers.
    pub fn idle_count(&self) -> usize {
        self.workers.iter().filter(|w| w.is_idle()).count()
    }
    /// Number of busy workers.
    pub fn busy_count(&self) -> usize {
        self.workers.iter().filter(|w| !w.is_idle()).count()
    }
    /// Total items completed across all workers.
    pub fn total_completed(&self) -> usize {
        self.workers.iter().map(|w| w.completed).sum()
    }
    /// Total cost across all workers.
    pub fn total_cost(&self) -> u64 {
        self.workers.iter().map(|w| w.total_cost).sum()
    }
    /// Per-worker utilization as a fraction of the most-loaded worker.
    pub fn relative_utilization(&self) -> Vec<f64> {
        let max_cost = self.workers.iter().map(|w| w.total_cost).max().unwrap_or(1);
        self.workers
            .iter()
            .map(|w| {
                if max_cost == 0 { 0.0 } else { w.total_cost as f64 / max_cost as f64 }
            })
            .collect()
    }
    /// Get a reference to a worker by index.
    pub fn get(&self, idx: usize) -> Option<&Worker> {
        self.workers.get(idx)
    }
    /// Iterate over all workers.
    pub fn iter(&self) -> impl Iterator<Item = &Worker> {
        self.workers.iter()
    }
}
/// Configuration for a parallel scheduler.
#[derive(Clone, Debug)]
pub struct SchedulerConfig {
    /// Number of workers.
    pub num_workers: usize,
    /// Whether to enable work-stealing.
    pub work_stealing: bool,
    /// Task timeout (None = unlimited).
    pub default_timeout: Option<Duration>,
    /// Maximum retry attempts on task failure.
    pub max_retries: u32,
    /// Whether to stop all tasks on first failure.
    pub fail_fast: bool,
    /// Whether to emit detailed progress events.
    pub verbose: bool,
    /// Scheduling strategy.
    pub strategy: SchedulingStrategy,
}
impl SchedulerConfig {
    /// Create a default configuration.
    pub fn new(num_workers: usize) -> Self {
        Self {
            num_workers,
            work_stealing: true,
            default_timeout: None,
            max_retries: 0,
            fail_fast: false,
            verbose: false,
            strategy: SchedulingStrategy::PriorityFirst,
        }
    }
    /// Enable fail-fast mode.
    pub fn with_fail_fast(mut self) -> Self {
        self.fail_fast = true;
        self
    }
    /// Enable verbose progress reporting.
    pub fn with_verbose(mut self) -> Self {
        self.verbose = true;
        self
    }
    /// Set a default timeout.
    pub fn with_timeout(mut self, d: Duration) -> Self {
        self.default_timeout = Some(d);
        self
    }
    /// Set max retries.
    pub fn with_max_retries(mut self, n: u32) -> Self {
        self.max_retries = n;
        self
    }
    /// Set the scheduling strategy.
    pub fn with_strategy(mut self, s: SchedulingStrategy) -> Self {
        self.strategy = s;
        self
    }
}
/// Tracks overall build progress.
#[derive(Clone, Debug)]
pub struct ProgressTracker {
    records: HashMap<u64, TaskRecord>,
    total: usize,
}
impl ProgressTracker {
    /// Create a new progress tracker.
    pub fn new() -> Self {
        Self {
            records: HashMap::new(),
            total: 0,
        }
    }
    /// Register a task.
    pub fn register(&mut self, mut record: TaskRecord) {
        record.mark_enqueued();
        self.total += 1;
        self.records.insert(record.item_id, record);
    }
    /// Mark a task as started by the given worker.
    pub fn start(&mut self, item_id: u64, worker_id: usize) {
        if let Some(r) = self.records.get_mut(&item_id) {
            r.mark_started(worker_id);
        }
    }
    /// Mark a task as succeeded.
    pub fn succeed(&mut self, item_id: u64) {
        if let Some(r) = self.records.get_mut(&item_id) {
            r.mark_succeeded();
        }
    }
    /// Mark a task as failed.
    pub fn fail(&mut self, item_id: u64, exit_code: i32) {
        if let Some(r) = self.records.get_mut(&item_id) {
            r.mark_failed(exit_code);
        }
    }
    /// Mark a task as cancelled.
    pub fn cancel(&mut self, item_id: u64) {
        if let Some(r) = self.records.get_mut(&item_id) {
            r.mark_cancelled();
        }
    }
    /// Mark a task as skipped.
    pub fn skip(&mut self, item_id: u64) {
        if let Some(r) = self.records.get_mut(&item_id) {
            r.mark_skipped();
        }
    }
    /// Return a reference to a task record by id.
    pub fn get(&self, item_id: u64) -> Option<&TaskRecord> {
        self.records.get(&item_id)
    }
    /// Number of tasks in each terminal state.
    pub fn count_by_status(&self) -> HashMap<String, usize> {
        let mut map: HashMap<String, usize> = HashMap::new();
        for r in self.records.values() {
            *map.entry(format!("{}", r.status)).or_insert(0) += 1;
        }
        map
    }
    /// Fraction of tasks that have reached a terminal state.
    pub fn completion_fraction(&self) -> f64 {
        if self.total == 0 {
            return 1.0;
        }
        let done = self.records.values().filter(|r| r.status.is_terminal()).count();
        done as f64 / self.total as f64
    }
    /// True when every registered task has reached a terminal state.
    pub fn is_all_done(&self) -> bool {
        self.records.values().all(|r| r.status.is_terminal())
    }
    /// All records whose status is failed or timed-out.
    pub fn failures(&self) -> Vec<&TaskRecord> {
        self.records
            .values()
            .filter(|r| {
                matches!(r.status, TaskStatus::Failed { .. } | TaskStatus::TimedOut)
            })
            .collect()
    }
    /// Average wait duration across all tasks that have started.
    pub fn avg_wait_duration(&self) -> Option<Duration> {
        let waits: Vec<Duration> = self
            .records
            .values()
            .filter_map(|r| r.wait_duration())
            .collect();
        if waits.is_empty() {
            None
        } else {
            let total: Duration = waits.iter().sum();
            Some(total / waits.len() as u32)
        }
    }
    /// Average execution duration across all tasks that have finished.
    pub fn avg_execution_duration(&self) -> Option<Duration> {
        let execs: Vec<Duration> = self
            .records
            .values()
            .filter_map(|r| r.execution_duration())
            .collect();
        if execs.is_empty() {
            None
        } else {
            let total: Duration = execs.iter().sum();
            Some(total / execs.len() as u32)
        }
    }
}
/// Schedules work in task batches.
pub struct BatchScheduler {
    inner: ParallelScheduler,
    submitted_batches: Vec<TaskBatch>,
}
impl BatchScheduler {
    /// Create a new batch scheduler.
    pub fn new(num_workers: usize) -> Self {
        Self {
            inner: ParallelScheduler::new(num_workers),
            submitted_batches: Vec::new(),
        }
    }
    /// Submit an entire batch of work items.
    pub fn submit_batch(&mut self, batch: TaskBatch) {
        let items = batch.clone().into_chained_items();
        self.submitted_batches.push(batch);
        for item in items {
            self.inner.submit(item);
        }
    }
    /// Schedule the next available task.
    pub fn schedule_next(&mut self) -> Option<(usize, u64)> {
        self.inner.schedule_next()
    }
    /// Mark a task complete.
    pub fn complete_item(&mut self, worker_id: usize, item_id: u64) {
        self.inner.complete_item(worker_id, item_id);
    }
    /// Whether all batches have been fully processed.
    pub fn is_complete(&self) -> bool {
        self.inner.is_complete()
    }
    /// Statistics.
    pub fn stats(&self) -> SchedulerStats {
        self.inner.stats()
    }
    /// Number of submitted batches.
    pub fn batch_count(&self) -> usize {
        self.submitted_batches.len()
    }
    /// Total work item count across all submitted batches.
    pub fn total_item_count(&self) -> usize {
        self.submitted_batches.iter().map(|b| b.len()).sum()
    }
}
/// Detailed utilization metrics collected over a scheduling run.
#[derive(Clone, Debug)]
pub struct SchedulerUtilization {
    /// Wall-clock time the scheduler was active.
    pub wall_time: Duration,
    /// Total accumulated worker-busy-time.
    pub total_busy_time: Duration,
    /// Number of idle cycles (ticks where a worker was idle).
    pub idle_ticks: u64,
    /// Number of busy cycles.
    pub busy_ticks: u64,
    /// Peak number of concurrent tasks.
    pub peak_concurrency: usize,
    /// Total items processed.
    pub items_processed: usize,
}
impl SchedulerUtilization {
    /// CPU utilization ratio (0.0–1.0).
    pub fn cpu_utilization(&self) -> f64 {
        let total_ticks = self.idle_ticks + self.busy_ticks;
        if total_ticks == 0 { 0.0 } else { self.busy_ticks as f64 / total_ticks as f64 }
    }
    /// Throughput: items per second (requires wall_time > 0).
    pub fn throughput_items_per_sec(&self) -> f64 {
        let secs = self.wall_time.as_secs_f64();
        if secs == 0.0 { 0.0 } else { self.items_processed as f64 / secs }
    }
}
/// A max-heap priority queue for work items (highest priority = first served).
pub struct MaxPriorityQueue {
    heap: Vec<WorkItem>,
}
impl MaxPriorityQueue {
    /// Create an empty queue.
    pub fn new() -> Self {
        Self { heap: Vec::new() }
    }
    /// Insert an item.
    pub fn push(&mut self, item: WorkItem) {
        self.heap.push(item);
        self.sift_up(self.heap.len() - 1);
    }
    /// Remove and return the highest-priority item.
    pub fn pop_max(&mut self) -> Option<WorkItem> {
        if self.heap.is_empty() {
            return None;
        }
        let last = self.heap.len() - 1;
        self.heap.swap(0, last);
        let item = self.heap.pop();
        if !self.heap.is_empty() {
            self.sift_down(0);
        }
        item
    }
    /// Peek at the highest-priority item.
    pub fn peek_max(&self) -> Option<&WorkItem> {
        self.heap.first()
    }
    /// Number of items.
    pub fn len(&self) -> usize {
        self.heap.len()
    }
    /// Whether empty.
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }
    fn sift_up(&mut self, mut i: usize) {
        while i > 0 {
            let parent = (i - 1) / 2;
            if self.heap[i].priority > self.heap[parent].priority {
                self.heap.swap(i, parent);
                i = parent;
            } else {
                break;
            }
        }
    }
    fn sift_down(&mut self, mut i: usize) {
        let n = self.heap.len();
        loop {
            let left = 2 * i + 1;
            let right = 2 * i + 2;
            let mut largest = i;
            if left < n && self.heap[left].priority > self.heap[largest].priority {
                largest = left;
            }
            if right < n && self.heap[right].priority > self.heap[largest].priority {
                largest = right;
            }
            if largest == i {
                break;
            }
            self.heap.swap(i, largest);
            i = largest;
        }
    }
}
/// Detailed execution status of a task.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TaskStatus {
    /// Task is waiting for dependencies to complete.
    Pending,
    /// Task is ready to run (all deps met).
    Ready,
    /// Task is currently running on a worker.
    Running { worker_id: usize },
    /// Task completed successfully.
    Succeeded,
    /// Task failed with an exit code.
    Failed { exit_code: i32 },
    /// Task was cancelled before starting.
    Cancelled,
    /// Task exceeded its timeout.
    TimedOut,
    /// Task was skipped (condition not met).
    Skipped,
}
impl TaskStatus {
    /// Returns true if this is a terminal state.
    pub fn is_terminal(&self) -> bool {
        matches!(
            self, Self::Succeeded | Self::Failed { .. } | Self::Cancelled |
            Self::TimedOut | Self::Skipped
        )
    }
    /// Returns true if this task completed successfully.
    pub fn is_success(&self) -> bool {
        *self == Self::Succeeded
    }
    /// Returns true if this task needs to be executed.
    pub fn is_runnable(&self) -> bool {
        *self == Self::Ready
    }
}
/// A group of work items that should be treated as a logical unit.
#[derive(Clone, Debug)]
pub struct TaskBatch {
    /// Batch identifier.
    pub id: u64,
    /// Human-readable batch name.
    pub name: String,
    /// Items belonging to this batch.
    pub items: Vec<WorkItem>,
    /// Whether items within the batch can run in parallel.
    pub parallel: bool,
}
impl TaskBatch {
    /// Create a new batch.
    pub fn new(id: u64, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            items: Vec::new(),
            parallel: true,
        }
    }
    /// Add an item to the batch.
    pub fn add_item(mut self, item: WorkItem) -> Self {
        self.items.push(item);
        self
    }
    /// Disable parallel execution within the batch.
    pub fn sequential(mut self) -> Self {
        self.parallel = false;
        self
    }
    /// Total estimated cost of all items in the batch.
    pub fn total_cost(&self) -> u64 {
        self.items.iter().map(|i| i.estimated_cost).sum()
    }
    /// Number of items.
    pub fn len(&self) -> usize {
        self.items.len()
    }
    /// Whether the batch is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    /// Convert a sequential batch into a chain of dependent items.
    ///
    /// Each item (after the first) depends on the previous one.
    pub fn into_chained_items(mut self) -> Vec<WorkItem> {
        if self.parallel || self.items.len() <= 1 {
            return self.items;
        }
        let mut result = Vec::with_capacity(self.items.len());
        let mut prev_id: Option<u64> = None;
        for mut item in self.items.drain(..) {
            if let Some(pid) = prev_id {
                item.add_dep(pid);
            }
            prev_id = Some(item.id);
            result.push(item);
        }
        result
    }
}
/// Full lifecycle record for a tracked task.
#[derive(Clone, Debug)]
pub struct TaskRecord {
    /// The work item id.
    pub item_id: u64,
    /// Human-readable task name.
    pub name: String,
    /// Current status.
    pub status: TaskStatus,
    /// Time the task was enqueued.
    pub enqueued_at: Option<Instant>,
    /// Time the task started running.
    pub started_at: Option<Instant>,
    /// Time the task finished.
    pub finished_at: Option<Instant>,
    /// Optional timeout duration.
    pub timeout: Option<Duration>,
    /// Number of retry attempts made.
    pub retry_count: u32,
    /// Maximum allowed retries.
    pub max_retries: u32,
}
impl TaskRecord {
    /// Create a new pending task record.
    pub fn new(item_id: u64, name: &str) -> Self {
        Self {
            item_id,
            name: name.to_string(),
            status: TaskStatus::Pending,
            enqueued_at: None,
            started_at: None,
            finished_at: None,
            timeout: None,
            retry_count: 0,
            max_retries: 0,
        }
    }
    /// Set maximum retries for this task.
    pub fn with_max_retries(mut self, n: u32) -> Self {
        self.max_retries = n;
        self
    }
    /// Set a timeout for this task.
    pub fn with_timeout(mut self, d: Duration) -> Self {
        self.timeout = Some(d);
        self
    }
    /// Mark as enqueued at the current time.
    pub fn mark_enqueued(&mut self) {
        self.enqueued_at = Some(Instant::now());
        self.status = TaskStatus::Ready;
    }
    /// Mark as started by the given worker.
    pub fn mark_started(&mut self, worker_id: usize) {
        self.started_at = Some(Instant::now());
        self.status = TaskStatus::Running { worker_id };
    }
    /// Mark as succeeded.
    pub fn mark_succeeded(&mut self) {
        self.finished_at = Some(Instant::now());
        self.status = TaskStatus::Succeeded;
    }
    /// Mark as failed with an exit code.
    pub fn mark_failed(&mut self, exit_code: i32) {
        self.finished_at = Some(Instant::now());
        self.status = TaskStatus::Failed { exit_code };
    }
    /// Mark as timed out.
    pub fn mark_timed_out(&mut self) {
        self.finished_at = Some(Instant::now());
        self.status = TaskStatus::TimedOut;
    }
    /// Mark as cancelled.
    pub fn mark_cancelled(&mut self) {
        self.finished_at = Some(Instant::now());
        self.status = TaskStatus::Cancelled;
    }
    /// Mark as skipped.
    pub fn mark_skipped(&mut self) {
        self.finished_at = Some(Instant::now());
        self.status = TaskStatus::Skipped;
    }
    /// Wall-clock wait time (enqueued → started).
    pub fn wait_duration(&self) -> Option<Duration> {
        match (self.enqueued_at, self.started_at) {
            (Some(e), Some(s)) => s.checked_duration_since(e),
            _ => None,
        }
    }
    /// Wall-clock execution time (started → finished).
    pub fn execution_duration(&self) -> Option<Duration> {
        match (self.started_at, self.finished_at) {
            (Some(s), Some(f)) => f.checked_duration_since(s),
            _ => None,
        }
    }
    /// Whether this task can be retried.
    pub fn can_retry(&self) -> bool {
        self.retry_count < self.max_retries
    }
    /// Increment retry counter and reset to Ready.
    pub fn retry(&mut self) {
        self.retry_count += 1;
        self.started_at = None;
        self.finished_at = None;
        self.status = TaskStatus::Ready;
    }
    /// Check if the task has exceeded its timeout (if started).
    pub fn is_timed_out(&self) -> bool {
        if let (Some(started), Some(timeout)) = (self.started_at, self.timeout) {
            started.elapsed() > timeout
        } else {
            false
        }
    }
}
/// A fully-featured scheduler with configurable strategy, retries, and
/// progress tracking.
pub struct FullScheduler {
    config: SchedulerConfig,
    pending: Vec<WorkItem>,
    completed_ids: HashSet<u64>,
    in_progress: HashMap<u64, usize>,
    pending_costs: HashMap<u64, u64>,
    pool: WorkerPool,
    progress: ProgressTracker,
    total_submitted: usize,
    total_completed: usize,
    failed_ids: HashSet<u64>,
    round_robin_next: usize,
}
impl FullScheduler {
    /// Create a new full scheduler with the given configuration.
    pub fn new(config: SchedulerConfig) -> Self {
        let pool = WorkerPool::new(config.num_workers);
        Self {
            config,
            pending: Vec::new(),
            completed_ids: HashSet::new(),
            in_progress: HashMap::new(),
            pending_costs: HashMap::new(),
            pool,
            progress: ProgressTracker::new(),
            total_submitted: 0,
            total_completed: 0,
            failed_ids: HashSet::new(),
            round_robin_next: 0,
        }
    }
    /// Submit a work item for scheduling.
    pub fn submit(&mut self, item: WorkItem) {
        let record = TaskRecord::new(item.id, &item.name);
        self.progress.register(record);
        self.total_submitted += 1;
        self.pending.push(item);
    }
    /// Schedule the next ready item according to the configured strategy.
    pub fn schedule_next(&mut self) -> Option<(usize, u64)> {
        if self.config.fail_fast && !self.failed_ids.is_empty() {
            return None;
        }
        let worker_idx = match self.config.strategy {
            SchedulingStrategy::RoundRobin => self.round_robin_worker()?,
            _ => self.pool.least_loaded_idle()?,
        };
        let ready_indices = self.ready_item_indices();
        if ready_indices.is_empty() {
            return None;
        }
        let best_idx = self.pick_best(&ready_indices);
        let item = self.pending.remove(best_idx);
        let item_id = item.id;
        let cost = item.estimated_cost;
        self.pool.assign(worker_idx, item_id);
        self.in_progress.insert(item_id, worker_idx);
        self.pending_costs.insert(item_id, cost);
        self.progress.start(item_id, worker_idx);
        Some((worker_idx, item_id))
    }
    /// Mark a task as succeeded.
    pub fn complete_item(&mut self, worker_id: usize, item_id: u64) {
        let cost = self.pending_costs.remove(&item_id).unwrap_or(1);
        self.pool.complete(item_id, cost);
        self.in_progress.remove(&item_id);
        self.completed_ids.insert(item_id);
        self.total_completed += 1;
        self.progress.succeed(item_id);
        let _ = worker_id;
    }
    /// Mark a task as failed (with exit code).
    pub fn fail_item(&mut self, item_id: u64, exit_code: i32) {
        let cost = self.pending_costs.remove(&item_id).unwrap_or(1);
        self.pool.complete(item_id, cost);
        self.in_progress.remove(&item_id);
        self.failed_ids.insert(item_id);
        self.total_completed += 1;
        self.progress.fail(item_id, exit_code);
    }
    /// Cancel a pending or in-progress task.
    pub fn cancel_item(&mut self, item_id: u64) {
        self.pending.retain(|i| i.id != item_id);
        if self.in_progress.contains_key(&item_id) {
            self.pool.complete(item_id, 0);
            self.in_progress.remove(&item_id);
            self.pending_costs.remove(&item_id);
        }
        self.progress.cancel(item_id);
    }
    /// Whether all submitted items have finished (success or failure).
    pub fn is_complete(&self) -> bool {
        self.pending.is_empty() && self.in_progress.is_empty()
    }
    /// Whether any tasks have failed.
    pub fn has_failures(&self) -> bool {
        !self.failed_ids.is_empty()
    }
    /// Access the progress tracker.
    pub fn progress(&self) -> &ProgressTracker {
        &self.progress
    }
    /// Worker pool reference.
    pub fn pool(&self) -> &WorkerPool {
        &self.pool
    }
    /// Build a SchedulerStats snapshot.
    pub fn stats(&self) -> SchedulerStats {
        let busy = self.pool.busy_count();
        let total_workers = self.pool.size();
        let parallelism_ratio = if total_workers == 0 {
            0.0
        } else {
            busy as f64 / total_workers as f64
        };
        SchedulerStats {
            total_items: self.total_submitted,
            completed: self.total_completed,
            avg_wait_time_ms: self
                .progress
                .avg_wait_duration()
                .map(|d| d.as_secs_f64() * 1000.0)
                .unwrap_or(0.0),
            parallelism_ratio,
        }
    }
    fn ready_item_indices(&self) -> Vec<usize> {
        self.pending
            .iter()
            .enumerate()
            .filter(|(_, item)| {
                item.deps.iter().all(|d| self.completed_ids.contains(d))
            })
            .map(|(i, _)| i)
            .collect()
    }
    fn pick_best(&self, indices: &[usize]) -> usize {
        match self.config.strategy {
            SchedulingStrategy::ShortestJobFirst => {
                *indices
                    .iter()
                    .min_by_key(|&&i| self.pending[i].estimated_cost)
                    .unwrap_or(&indices[0])
            }
            SchedulingStrategy::LongestJobFirst => {
                *indices
                    .iter()
                    .max_by_key(|&&i| self.pending[i].estimated_cost)
                    .unwrap_or(&indices[0])
            }
            _ => {
                *indices
                    .iter()
                    .max_by(|&&a, &&b| {
                        let ia = &self.pending[a];
                        let ib = &self.pending[b];
                        ia.priority.cmp(&ib.priority).then(ib.id.cmp(&ia.id))
                    })
                    .unwrap_or(&indices[0])
            }
        }
    }
    fn round_robin_worker(&mut self) -> Option<usize> {
        let n = self.pool.size();
        for _ in 0..n {
            let idx = self.round_robin_next % n;
            self.round_robin_next += 1;
            if self.pool.get(idx).map(|w| w.is_idle()).unwrap_or(false) {
                return Some(idx);
            }
        }
        None
    }
}
