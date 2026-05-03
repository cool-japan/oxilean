//! Concurrent task scheduler with adaptive load balancing and work-stealing metrics.
//!
//! Provides an `AdaptiveScheduler` that tracks task lifecycle, collects
//! per-worker statistics, and switches load-balance policy dynamically when
//! worker imbalance is detected.

pub mod functions;
pub mod types;

pub use functions::{imbalance_ratio, suggest_worker_count};
pub use types::{
    AdaptiveScheduler, LoadBalancePolicy, SchedulerMetrics, Task, TaskId, TaskPriority, TaskState,
    WorkerStats,
};
