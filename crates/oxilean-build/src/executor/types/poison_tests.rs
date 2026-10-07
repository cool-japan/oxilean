//! Behaviour of `BuildExecutor` around its private locks, including after a
//! thread panicked while holding the cancellation flag's or the results
//! log's.

use super::*;
use std::thread;

fn single_step_executor() -> BuildExecutor {
    let mut dag = BuildDag::new();
    let id = dag.next_step_id();
    dag.add_step(
        BuildStep::new(id, "parse A", StepKind::Parse, "A").add_input(Path::new("a.lean")),
    );
    BuildExecutor::new(dag, ExecutorConfig::default())
}

/// Poison `lock` by panicking in another thread while that thread holds it.
fn poison<T: Send + 'static>(lock: &Arc<Mutex<T>>) {
    let lock = Arc::clone(lock);
    let joined = thread::spawn(move || {
        let _guard = lock.lock();
        panic!("poisoning the lock on purpose");
    })
    .join();
    assert!(joined.is_err(), "the poisoning thread must have panicked");
}

#[test]
fn cancel_stops_the_next_execute() {
    let mut executor = single_step_executor();
    executor.cancel();
    assert!(matches!(executor.execute(), Err(ExecutorError::Cancelled)));
}

#[test]
fn progress_snapshot_follows_execute() {
    let mut executor = single_step_executor();
    let before = executor.current_progress();
    assert_eq!(before.total_steps, 1);
    assert_eq!(before.completed_steps, 0);
    assert_eq!(before.pending_steps, 1);
    let report = executor.execute().expect("build operation should succeed");
    assert!(report.success);
    let after = executor.current_progress();
    assert_eq!(after.completed_steps, 1);
    assert_eq!(after.failed_steps, 0);
    assert_eq!(after.running_steps, 0);
    assert!(after.current_steps.is_empty());
}

#[test]
fn poisoned_cancel_flag_is_still_read_and_written() {
    let mut executor = single_step_executor();
    poison(&executor.cancelled);
    assert!(executor.cancelled.is_poisoned());
    executor.cancel();
    assert!(matches!(executor.execute(), Err(ExecutorError::Cancelled)));
}

#[test]
fn poisoned_cancel_flag_does_not_stop_an_uncancelled_build() {
    let mut executor = single_step_executor();
    poison(&executor.cancelled);
    let report = executor.execute().expect("build operation should succeed");
    assert!(report.success);
    assert_eq!(report.completed_steps, 1);
}

#[test]
fn poisoned_results_log_is_still_appended_and_cloned() {
    let mut executor = single_step_executor();
    poison(&executor.results);
    assert!(executor.results.is_poisoned());
    let report = executor.execute().expect("build operation should succeed");
    assert!(report.success);
    assert_eq!(report.step_results.len(), 1);
    assert!(report.step_results[0].success);
}
