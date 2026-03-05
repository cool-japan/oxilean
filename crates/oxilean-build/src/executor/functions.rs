//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::manifest::{BuildProfile, OptLevel, Version};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use super::types::{
    BuildDag, BuildExecutionReport, BuildExecutor, BuildLog, BuildOutput, BuildPlanBuilder,
    BuildProgress, BuildReport, BuildStep, BuildTimer, ExecutorConfig, ExecutorError,
    ExecutorRunConfig, ExecutorStats, ExecutorWorkerPool, LogLevel, OutputCollector, OutputKind,
    ParallelScheduler, ProcessHandle, SandboxConfig, StepId, StepKind, StepPriorityQueue,
    StepResult,
};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_build_dag_topo_sort() {
        let mut dag = BuildDag::new();
        let id1 = dag.next_step_id();
        let id2 = dag.next_step_id();
        let id3 = dag.next_step_id();
        dag.add_step(BuildStep::new(id1.clone(), "A", StepKind::Parse, "A"));
        dag.add_step(
            BuildStep::new(id2.clone(), "B", StepKind::Elaborate, "B").depends_on(id1.clone()),
        );
        dag.add_step(
            BuildStep::new(id3.clone(), "C", StepKind::Compile, "C")
                .depends_on(id1.clone())
                .depends_on(id2.clone()),
        );
        let order = dag
            .topological_order()
            .expect("DAG operation should succeed");
        assert_eq!(order.len(), 3);
        assert_eq!(order[0], id1);
    }
    #[test]
    fn test_critical_path() {
        let mut dag = BuildDag::new();
        let id1 = dag.next_step_id();
        let id2 = dag.next_step_id();
        let id3 = dag.next_step_id();
        dag.add_step(BuildStep::new(id1.clone(), "A", StepKind::Parse, "A").with_estimate(100));
        dag.add_step(
            BuildStep::new(id2.clone(), "B", StepKind::Parse, "B")
                .depends_on(id1.clone())
                .with_estimate(200),
        );
        dag.add_step(
            BuildStep::new(id3.clone(), "C", StepKind::Parse, "C")
                .depends_on(id1.clone())
                .with_estimate(50),
        );
        let (path, time) = dag.critical_path().expect("DAG operation should succeed");
        assert_eq!(path.len(), 2);
        assert_eq!(time, 300);
    }
    #[test]
    fn test_executor_simple() {
        let mut dag = BuildDag::new();
        let id1 = dag.next_step_id();
        dag.add_step(
            BuildStep::new(id1, "parse A", StepKind::Parse, "A").add_input(Path::new("a.lean")),
        );
        let config = ExecutorConfig::default();
        let mut executor = BuildExecutor::new(dag, config);
        let report = executor.execute().expect("build operation should succeed");
        assert!(report.success);
        assert_eq!(report.completed_steps, 1);
    }
    #[test]
    fn test_plan_builder() {
        let mut builder = BuildPlanBuilder::new();
        builder.add_module("A", Path::new("a.lean"), &[]);
        builder.add_module("B", Path::new("b.lean"), &["A".to_string()]);
        let _link_id = builder.add_link_step("output");
        let dag = builder.build();
        assert_eq!(dag.step_count(), 7);
    }
    #[test]
    fn test_parallel_scheduler() {
        let mut dag = BuildDag::new();
        let id1 = dag.next_step_id();
        let id2 = dag.next_step_id();
        let id3 = dag.next_step_id();
        dag.add_step(BuildStep::new(id1.clone(), "A", StepKind::Parse, "A"));
        dag.add_step(BuildStep::new(id2.clone(), "B", StepKind::Parse, "B"));
        dag.add_step(
            BuildStep::new(id3.clone(), "C", StepKind::Link, "C")
                .depends_on(id1.clone())
                .depends_on(id2.clone()),
        );
        let mut scheduler = ParallelScheduler::new(dag, 4);
        let batch = scheduler.next_batch();
        assert_eq!(batch.len(), 2);
        scheduler.start_step(&id1);
        scheduler.start_step(&id2);
        let batch = scheduler.next_batch();
        assert!(batch.is_empty());
        scheduler.complete_step(&id1);
        scheduler.complete_step(&id2);
        let batch = scheduler.next_batch();
        assert_eq!(batch.len(), 1);
        assert_eq!(batch[0], id3);
    }
    #[test]
    fn test_progress_tracking() {
        let mut progress = BuildProgress::new(10);
        assert_eq!(progress.percentage(), 0);
        assert!(!progress.is_complete());
        progress.completed_steps = 5;
        assert_eq!(progress.percentage(), 50);
        progress.completed_steps = 10;
        assert!(progress.is_complete());
        assert_eq!(progress.percentage(), 100);
    }
    #[test]
    fn test_progress_bar() {
        let mut progress = BuildProgress::new(4);
        progress.completed_steps = 2;
        let bar = progress.progress_bar(20);
        assert!(bar.contains("2/4"));
        assert!(bar.contains("50%"));
    }
    #[test]
    fn test_build_report_summary() {
        let report = BuildReport {
            total_steps: 10,
            completed_steps: 8,
            failed_steps: 1,
            skipped_steps: 1,
            total_duration: Duration::from_secs(5),
            step_results: Vec::new(),
            success: false,
            profile_name: "debug".to_string(),
            opt_level: OptLevel::None,
        };
        let summary = report.summary();
        assert!(summary.contains("FAILED"));
        assert!(summary.contains("debug"));
    }
    #[test]
    fn test_build_report_success() {
        let report = BuildReport {
            total_steps: 3,
            completed_steps: 3,
            failed_steps: 0,
            skipped_steps: 0,
            total_duration: Duration::from_millis(500),
            step_results: vec![
                StepResult::success(StepId::new(0), Duration::from_millis(100)),
                StepResult::success(StepId::new(1), Duration::from_millis(200)),
                StepResult::success(StepId::new(2), Duration::from_millis(200)),
            ],
            success: true,
            profile_name: "release".to_string(),
            opt_level: OptLevel::Full,
        };
        assert!(report.success);
        let avg = report.avg_step_duration();
        assert!(avg.as_millis() > 0);
    }
    #[test]
    fn test_output_collector() {
        let mut collector = OutputCollector::new();
        collector.add(
            BuildOutput::new(
                Path::new("/tmp/a.o"),
                StepId::new(0),
                OutputKind::CompiledObject,
            )
            .with_size(1024),
        );
        collector.add(
            BuildOutput::new(
                Path::new("/tmp/b.o"),
                StepId::new(1),
                OutputKind::CompiledObject,
            )
            .with_size(2048),
        );
        collector.add(
            BuildOutput::new(
                Path::new("/tmp/docs/index.html"),
                StepId::new(2),
                OutputKind::Documentation,
            )
            .with_size(512),
        );
        assert_eq!(collector.count(), 3);
        assert_eq!(collector.total_size(), 3584);
        assert_eq!(collector.by_kind(&OutputKind::CompiledObject).len(), 2);
        assert_eq!(collector.by_kind(&OutputKind::Documentation).len(), 1);
    }
    #[test]
    fn test_build_log() {
        let mut log = BuildLog::new();
        log.log(
            Some(StepId::new(0)),
            LogLevel::Info,
            "parsing module A",
            Duration::from_millis(100),
        );
        log.log(
            Some(StepId::new(0)),
            LogLevel::Warning,
            "unused import",
            Duration::from_millis(150),
        );
        log.log(
            Some(StepId::new(1)),
            LogLevel::Error,
            "type mismatch",
            Duration::from_millis(200),
        );
        log.log(
            None,
            LogLevel::Debug,
            "debug info",
            Duration::from_millis(50),
        );
        assert_eq!(log.entry_count(), 3);
        assert_eq!(log.warnings().len(), 1);
        assert_eq!(log.errors().len(), 1);
    }
    #[test]
    fn test_build_log_debug_level() {
        let mut log = BuildLog::new();
        log.set_min_level(LogLevel::Debug);
        log.log(None, LogLevel::Debug, "debug msg", Duration::ZERO);
        assert_eq!(log.entry_count(), 1);
    }
    #[test]
    fn test_build_timer() {
        let mut timer = BuildTimer::new();
        timer.start_build();
        let step = StepId::new(0);
        timer.start_step(step.clone());
        std::thread::sleep(Duration::from_millis(5));
        let duration = timer.end_step(&step);
        assert!(duration.as_millis() >= 1);
        assert_eq!(timer.timed_step_count(), 1);
        assert!(timer.step_duration(&step).is_some());
        assert!(timer.elapsed().as_millis() >= 1);
    }
    #[test]
    fn test_step_kind_display() {
        assert_eq!(format!("{}", StepKind::Parse), "parse");
        assert_eq!(format!("{}", StepKind::Elaborate), "elaborate");
        assert_eq!(format!("{}", StepKind::Compile), "compile");
        assert_eq!(format!("{}", StepKind::Link), "link");
        assert_eq!(
            format!("{}", StepKind::Script("gen".to_string())),
            "script:gen"
        );
    }
    #[test]
    fn test_executor_error_display() {
        let err = ExecutorError::StepFailed {
            step_id: StepId::new(42),
            error: "compilation error".to_string(),
        };
        let msg = format!("{}", err);
        assert!(msg.contains("step-42"));
        assert!(msg.contains("compilation error"));
    }
    #[test]
    fn test_scheduler_state_counts() {
        let mut dag = BuildDag::new();
        let id1 = dag.next_step_id();
        let id2 = dag.next_step_id();
        dag.add_step(BuildStep::new(id1.clone(), "A", StepKind::Parse, "A"));
        dag.add_step(
            BuildStep::new(id2.clone(), "B", StepKind::Parse, "B").depends_on(id1.clone()),
        );
        let scheduler = ParallelScheduler::new(dag, 4);
        let counts = scheduler.state_counts();
        assert_eq!(*counts.get("ready").unwrap_or(&0), 1);
        assert_eq!(*counts.get("blocked").unwrap_or(&0), 1);
    }
    #[test]
    fn test_scheduler_completion() {
        let mut dag = BuildDag::new();
        let id1 = dag.next_step_id();
        dag.add_step(BuildStep::new(id1.clone(), "A", StepKind::Parse, "A"));
        let mut scheduler = ParallelScheduler::new(dag, 1);
        assert!(!scheduler.is_complete());
        scheduler.start_step(&id1);
        scheduler.complete_step(&id1);
        assert!(scheduler.is_complete());
    }
    #[test]
    fn test_plan_builder_with_docs() {
        let mut builder = BuildPlanBuilder::new();
        builder.add_module("A", Path::new("a.lean"), &[]);
        let _doc_id = builder.add_doc_step("A");
        let dag = builder.build();
        assert_eq!(dag.step_count(), 4);
    }
    #[test]
    fn test_estimate_total_time() {
        let mut dag = BuildDag::new();
        let id1 = dag.next_step_id();
        let id2 = dag.next_step_id();
        dag.add_step(BuildStep::new(id1.clone(), "A", StepKind::Parse, "A").with_estimate(100));
        dag.add_step(
            BuildStep::new(id2, "B", StepKind::Parse, "B")
                .depends_on(id1)
                .with_estimate(200),
        );
        let estimate = dag
            .estimate_total_time(1)
            .expect("DAG operation should succeed");
        assert_eq!(estimate, 300);
        let estimate_parallel = dag
            .estimate_total_time(4)
            .expect("DAG operation should succeed");
        assert!(estimate_parallel >= 300);
    }
}
#[cfg(test)]
mod executor_extra_tests {
    use super::*;
    #[test]
    fn executor_stats_success_rate() {
        let mut s = ExecutorStats::new();
        s.steps_succeeded = 8;
        s.steps_failed = 2;
        assert!((s.success_rate() - 0.8).abs() < 1e-9);
    }
    #[test]
    fn executor_stats_summary() {
        let s = ExecutorStats::new();
        assert!(!s.summary().is_empty());
    }
    #[test]
    fn sandbox_config_strict() {
        let cfg = SandboxConfig::strict();
        assert!(cfg.enabled);
        assert!(!cfg.allow_network);
    }
    #[test]
    fn sandbox_config_allow_network() {
        let cfg = SandboxConfig::strict().allow_network();
        assert!(cfg.allow_network);
    }
    #[test]
    fn process_handle_complete() {
        let mut h = ProcessHandle::spawned(1);
        assert!(!h.finished);
        h.complete(0);
        assert!(h.succeeded());
        h.complete(1);
        assert!(!h.succeeded());
    }
    #[test]
    fn worker_pool_capacity() {
        let mut pool = ExecutorWorkerPool::new(2);
        assert!(pool.assign(1));
        assert!(pool.assign(2));
        assert!(!pool.assign(3));
        pool.release(1);
        assert!(pool.assign(3));
    }
    #[test]
    fn worker_pool_utilization() {
        let mut pool = ExecutorWorkerPool::new(4);
        pool.assign(1);
        pool.assign(2);
        assert!((pool.utilization() - 0.5).abs() < 1e-9);
    }
    #[test]
    fn step_priority_queue_ordering() {
        let mut q = StepPriorityQueue::new();
        q.push(10, 1);
        q.push(20, 5);
        q.push(30, 3);
        let first = q.pop().expect("collection should not be empty");
        assert_eq!(first, 20);
    }
    #[test]
    fn build_execution_report_is_success() {
        let mut r = BuildExecutionReport::new();
        r.succeeded.push(1);
        r.succeeded.push(2);
        assert!(r.is_success());
        r.failed.push(3);
        assert!(!r.is_success());
    }
    #[test]
    fn build_execution_report_display() {
        let r = BuildExecutionReport::new();
        let s = format!("{}", r);
        assert!(s.contains("Report["));
    }
}
/// Returns the executor subsystem version.
pub fn executor_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
#[cfg(test)]
mod executor_version_test {
    use super::*;
    #[test]
    fn executor_version_nonempty() {
        assert!(!executor_version().is_empty());
    }
}
#[cfg(test)]
mod executor_run_config_tests {
    use super::*;
    #[test]
    fn executor_run_config_ci() {
        let cfg = ExecutorRunConfig::ci();
        assert!(cfg.fail_fast);
        assert!(cfg.verbose);
    }
    #[test]
    fn executor_run_config_with_workers() {
        let cfg = ExecutorRunConfig::default().with_workers(8);
        assert_eq!(cfg.max_workers, 8);
    }
    #[test]
    fn executor_run_config_with_workers_min_one() {
        let cfg = ExecutorRunConfig::default().with_workers(0);
        assert_eq!(cfg.max_workers, 1);
    }
}
/// Returns the default max number of executor workers.
pub fn default_executor_workers() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
}
/// Returns the executor protocol version string.
pub fn executor_protocol_version() -> &'static str {
    "1.0"
}
#[cfg(test)]
mod protocol_test {
    use super::*;
    #[test]
    fn protocol_version_nonempty() {
        assert!(!executor_protocol_version().is_empty());
    }
}
/// Returns whether the executor supports speculative execution.
pub fn supports_speculative_execution() -> bool {
    false
}
/// Returns whether the executor supports priority scheduling.
pub fn supports_priority_scheduling() -> bool {
    true
}
/// Returns the default step timeout in seconds (0 = none).
pub fn default_step_timeout_secs() -> u64 {
    600
}
