//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    CheckpointManager, GoalDiff, ProofContext, ProofTrace, StateExtConfig1800,
    StateExtConfigVal1800, StateExtDiag1800, StateExtDiff1800, StateExtPass1800,
    StateExtPipeline1800, StateExtResult1800, TacStateBuilder, TacStateCounterMap, TacStateExtMap,
    TacStateExtUtil, TacStateStateMachine, TacStateWindow, TacStateWorkQueue, TacticCheckpoint,
    TacticError, TacticState, TacticStateAnalysisPass, TacticStateConfig, TacticStateConfigValue,
    TacticStateDiagnostics, TacticStateDiff, TacticStatePipeline, TacticStateResult, TacticStats,
    TacticStep,
};
use crate::basic::{MVarId, MetaContext};

/// Result type for tactic operations.
pub type TacticResult<T> = Result<T, TacticError>;
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tactic::state::*;
    fn id(n: u64) -> MVarId {
        MVarId(n)
    }
    #[test]
    fn test_create_state() {
        let state = TacticState::new(vec![id(0), id(1), id(2)]);
        assert_eq!(state.num_goals(), 3);
        assert!(!state.is_done());
    }
    #[test]
    fn test_single_goal() {
        let state = TacticState::single(id(42));
        assert_eq!(state.num_goals(), 1);
        assert_eq!(
            state.current_goal().expect("current_goal should succeed"),
            id(42)
        );
    }
    #[test]
    fn test_empty_state() {
        let state = TacticState::new(vec![]);
        assert!(state.is_done());
        assert!(state.current_goal().is_err());
    }
    #[test]
    fn test_focus() {
        let mut state = TacticState::new(vec![id(10), id(20), id(30)]);
        assert_eq!(
            state.current_goal().expect("current_goal should succeed"),
            id(10)
        );
        state.focus(2).expect("value should be present");
        assert_eq!(
            state.current_goal().expect("current_goal should succeed"),
            id(30)
        );
        assert!(state.focus(5).is_err());
    }
    #[test]
    fn test_replace_goal() {
        let mut state = TacticState::new(vec![id(10), id(20), id(30)]);
        state.replace_goal(vec![id(100), id(101)]);
        assert_eq!(state.num_goals(), 4);
        assert_eq!(state.all_goals(), &[id(100), id(101), id(20), id(30)]);
    }
    #[test]
    fn test_rotate() {
        let mut state = TacticState::new(vec![id(10), id(20), id(30)]);
        state.rotate();
        assert_eq!(state.all_goals(), &[id(20), id(30), id(10)]);
    }
    #[test]
    fn test_push_goals() {
        let mut state = TacticState::new(vec![id(10)]);
        state.push_goals(vec![id(20), id(30)]);
        assert_eq!(state.num_goals(), 3);
        assert_eq!(state.all_goals(), &[id(10), id(20), id(30)]);
    }
    #[test]
    fn test_tag_goal() {
        let mut state = TacticState::new(vec![id(10)]);
        state
            .tag_goal("main".to_string())
            .expect("serialization should succeed");
    }
    #[test]
    fn test_save_restore() {
        let mut state = TacticState::new(vec![id(10), id(20), id(30)]);
        state.save();
        state.replace_goal(vec![id(100)]);
        assert_eq!(state.num_goals(), 3);
        state.restore().expect("restore should succeed");
        assert_eq!(state.num_goals(), 3);
        assert_eq!(state.all_goals(), &[id(10), id(20), id(30)]);
    }
    #[test]
    fn test_restore_no_saved() {
        let mut state = TacticState::new(vec![id(10)]);
        assert!(state.restore().is_err());
    }
}
/// Returns all goals in the tactic state as a slice.
#[allow(dead_code)]
pub fn all_goals(state: &TacticState) -> &[MVarId] {
    state.all_goals()
}
/// Returns the first goal in the state, or `None` if there are no goals.
#[allow(dead_code)]
pub fn first_goal(state: &TacticState) -> Option<MVarId> {
    state.all_goals().first().copied()
}
/// Check that the tactic state is internally consistent.
///
/// Consistency here means: no goal ID is duplicated in the goal list.
#[allow(dead_code)]
pub fn is_consistent(state: &TacticState) -> bool {
    let goals = state.all_goals();
    let unique: std::collections::HashSet<u64> = goals.iter().map(|g| g.0).collect();
    unique.len() == goals.len()
}
/// Try to apply a tactic function; if it fails, restore the original state.
///
/// Returns `Ok(())` if the tactic succeeded, `Err(msg)` otherwise.
#[allow(dead_code)]
pub fn try_tactic<F>(state: &mut TacticState, f: F) -> Result<(), String>
where
    F: FnOnce(&mut TacticState) -> Result<(), String>,
{
    state.save();
    match f(state) {
        Ok(()) => Ok(()),
        Err(msg) => {
            let _ = state.restore();
            Err(msg)
        }
    }
}
/// Assert that the tactic state has exactly `expected` goals.
///
/// Returns `Ok(())` if so, `Err` with a descriptive message otherwise.
#[allow(dead_code)]
pub fn assert_goal_count(state: &TacticState, expected: usize) -> Result<(), String> {
    let actual = state.num_goals();
    if actual == expected {
        Ok(())
    } else {
        Err(format!("Expected {} goals, but found {}", expected, actual))
    }
}
/// Repeat the given tactic up to `max_times` until it fails or the state has no goals.
///
/// Returns the number of times the tactic was successfully applied.
#[allow(dead_code)]
pub fn repeat_tactic<F>(state: &mut TacticState, max_times: usize, mut f: F) -> usize
where
    F: FnMut(&mut TacticState) -> Result<(), String>,
{
    let mut count = 0;
    for _ in 0..max_times {
        if state.is_done() {
            break;
        }
        if f(state).is_err() {
            break;
        }
        count += 1;
    }
    count
}
/// A type alias for errors returned by tactic chain operations.
#[allow(dead_code)]
pub type TacticChainError = String;
/// Apply a sequence of named tactic functions in order, stopping on first failure.
///
/// Returns `Ok(())` if all tactics succeed, `Err(name)` with the name of the failing tactic.
#[allow(dead_code)]
pub fn chain_tactics<'a, F>(
    state: &mut TacticState,
    tactics: &[(&'a str, F)],
) -> Result<(), (&'a str, TacticChainError)>
where
    F: Fn(&mut TacticState) -> Result<(), String>,
{
    for (name, tac) in tactics {
        tac(state).map_err(|e| (*name, e))?;
    }
    Ok(())
}
/// Returns a human-readable summary of the tactic state.
///
/// Shows number of goals and the MVarId of each.
#[allow(dead_code)]
pub fn summarize_state(state: &TacticState) -> String {
    let goals = state.all_goals();
    if goals.is_empty() {
        "No goals (proof complete)".to_string()
    } else {
        let ids: Vec<String> = goals.iter().map(|id| format!("?{}", id.0)).collect();
        format!("{} goal(s): {}", goals.len(), ids.join(", "))
    }
}
#[cfg(test)]
mod extended_state_tests {
    use super::*;
    use crate::tactic::state::*;
    fn id(n: u64) -> MVarId {
        MVarId(n)
    }
    #[test]
    fn test_tactic_stats_record() {
        let mut stats = TacticStats::new();
        stats.record_tactic();
        stats.record_tactic();
        stats.record_goals_closed(1);
        stats.record_goals_opened(2);
        assert_eq!(stats.tactic_count, 2);
        assert_eq!(stats.goals_closed, 1);
        assert_eq!(stats.goals_opened, 2);
    }
    #[test]
    fn test_tactic_step_delta() {
        let step = TacticStep::new("apply", 3, 2);
        assert_eq!(step.delta(), 1);
    }
    #[test]
    fn test_tactic_step_with_message() {
        let step = TacticStep::new("intro", 2, 1).with_message("introduced h");
        assert_eq!(step.message.as_deref(), Some("introduced h"));
    }
    #[test]
    fn test_proof_trace_len() {
        let mut trace = ProofTrace::new();
        assert!(trace.is_empty());
        trace.push(TacticStep::new("intro", 1, 0));
        assert_eq!(trace.len(), 1);
    }
    #[test]
    fn test_proof_trace_total_delta() {
        let mut trace = ProofTrace::new();
        trace.push(TacticStep::new("apply", 3, 2));
        trace.push(TacticStep::new("exact", 2, 0));
        assert_eq!(trace.total_delta(), 3);
    }
    #[test]
    fn test_proof_trace_names() {
        let mut trace = ProofTrace::new();
        trace.push(TacticStep::new("intro", 2, 1));
        trace.push(TacticStep::new("exact", 1, 0));
        assert_eq!(trace.tactic_names(), vec!["intro", "exact"]);
    }
    #[test]
    fn test_proof_context_run_tactic() {
        let state = TacticState::new(vec![id(1), id(2)]);
        let mut ctx = ProofContext::new(state);
        let result = ctx.run_tactic("intro", |s| {
            s.replace_goal(vec![]);
            Ok(())
        });
        assert!(result.is_ok());
        assert_eq!(ctx.stats.tactic_count, 1);
        assert_eq!(ctx.stats.goals_closed, 1);
    }
    #[test]
    fn test_all_goals_helper() {
        let state = TacticState::new(vec![id(10), id(20)]);
        let goals = all_goals(&state);
        assert_eq!(goals.len(), 2);
    }
    #[test]
    fn test_first_goal_some() {
        let state = TacticState::new(vec![id(5), id(6)]);
        assert_eq!(first_goal(&state), Some(id(5)));
    }
    #[test]
    fn test_first_goal_none() {
        let state = TacticState::new(vec![]);
        assert_eq!(first_goal(&state), None);
    }
    #[test]
    fn test_is_consistent_unique() {
        let state = TacticState::new(vec![id(1), id(2), id(3)]);
        assert!(is_consistent(&state));
    }
    #[test]
    fn test_assert_goal_count_ok() {
        let state = TacticState::new(vec![id(1)]);
        assert!(assert_goal_count(&state, 1).is_ok());
    }
    #[test]
    fn test_assert_goal_count_err() {
        let state = TacticState::new(vec![id(1), id(2)]);
        assert!(assert_goal_count(&state, 1).is_err());
    }
    #[test]
    fn test_try_tactic_success() {
        let mut state = TacticState::new(vec![id(1)]);
        let result = try_tactic(&mut state, |s| {
            s.replace_goal(vec![]);
            Ok(())
        });
        assert!(result.is_ok());
        assert_eq!(state.num_goals(), 0);
    }
    #[test]
    fn test_try_tactic_failure_restores() {
        let mut state = TacticState::new(vec![id(1), id(2)]);
        let result = try_tactic(&mut state, |_s| Err("failed".to_string()));
        assert!(result.is_err());
        assert_eq!(state.num_goals(), 2);
    }
    #[test]
    fn test_summarize_state_no_goals() {
        let state = TacticState::new(vec![]);
        let summary = summarize_state(&state);
        assert!(summary.contains("complete"));
    }
    #[test]
    fn test_summarize_state_with_goals() {
        let state = TacticState::new(vec![id(1), id(2)]);
        let summary = summarize_state(&state);
        assert!(summary.contains("2 goal"));
    }
    #[test]
    fn test_repeat_tactic_closes_all() {
        let mut state = TacticState::new(vec![id(1), id(2), id(3)]);
        let count = repeat_tactic(&mut state, 10, |s| {
            if s.num_goals() > 0 {
                s.replace_goal(vec![]);
                Ok(())
            } else {
                Err("done".to_string())
            }
        });
        assert_eq!(count, 3);
        assert!(state.is_done());
    }
}
/// Rotate the focus to the next goal in the tactic state.
///
/// If there is only one goal, this is a no-op.
/// Useful for tactics like `swap` or `rotate_left`.
#[allow(dead_code)]
pub fn rotate_goals_left(state: &mut TacticState) {
    let n = state.num_goals();
    if n <= 1 {
        return;
    }
    let goals = state.all_goals().to_vec();
    let mut rotated = goals[1..].to_vec();
    rotated.push(goals[0]);
    state.replace_goal(rotated[..1].to_vec());
}
/// Returns the index (0-based) of the currently focused goal, if any.
///
/// Since `TacticState` always focuses on the first remaining goal by
/// default, this is typically 0 unless the state has been reordered.
#[allow(dead_code)]
pub fn current_goal_index(state: &TacticState) -> Option<usize> {
    if state.is_done() {
        None
    } else {
        Some(0)
    }
}
/// Check whether there are any remaining goals to prove.
#[allow(dead_code)]
pub fn has_remaining_goals(state: &TacticState) -> bool {
    !state.is_done()
}
/// Returns a list of all goal IDs as `u64` values.
///
/// This is a convenience for logging and debugging.
#[allow(dead_code)]
pub fn goal_ids_as_u64(state: &TacticState) -> Vec<u64> {
    state.all_goals().iter().map(|g| g.0).collect()
}
/// Apply a tactic to every goal in the state in sequence.
///
/// After each application, the next goal becomes the focus.
/// Returns the number of goals successfully closed.
#[allow(dead_code)]
pub fn apply_to_all_goals<F>(state: &mut TacticState, mut f: F) -> usize
where
    F: FnMut(&mut TacticState) -> Result<(), String>,
{
    let initial = state.num_goals();
    let mut closed = 0;
    for _ in 0..initial {
        if state.is_done() {
            break;
        }
        if f(state).is_ok() && state.num_goals() < initial - closed {
            closed += 1;
        }
    }
    closed
}
/// Returns true if the tactic state has exactly one goal.
#[allow(dead_code)]
pub fn has_single_goal(state: &TacticState) -> bool {
    state.num_goals() == 1
}
/// Returns true if the tactic state has more than one goal.
#[allow(dead_code)]
pub fn has_multiple_goals(state: &TacticState) -> bool {
    state.num_goals() > 1
}
#[cfg(test)]
mod extra_state_tests {
    use super::*;
    use crate::tactic::state::*;
    fn id(n: u64) -> MVarId {
        MVarId(n)
    }
    #[test]
    fn test_has_remaining_goals_true() {
        let state = TacticState::new(vec![id(1)]);
        assert!(has_remaining_goals(&state));
    }
    #[test]
    fn test_has_remaining_goals_false() {
        let state = TacticState::new(vec![]);
        assert!(!has_remaining_goals(&state));
    }
    #[test]
    fn test_current_goal_index_some() {
        let state = TacticState::new(vec![id(5)]);
        assert_eq!(current_goal_index(&state), Some(0));
    }
    #[test]
    fn test_current_goal_index_none() {
        let state = TacticState::new(vec![]);
        assert_eq!(current_goal_index(&state), None);
    }
    #[test]
    fn test_goal_ids_as_u64() {
        let state = TacticState::new(vec![id(3), id(7)]);
        let ids = goal_ids_as_u64(&state);
        assert_eq!(ids, vec![3u64, 7u64]);
    }
    #[test]
    fn test_has_single_goal() {
        let state = TacticState::new(vec![id(1)]);
        assert!(has_single_goal(&state));
        assert!(!has_multiple_goals(&state));
    }
    #[test]
    fn test_has_multiple_goals() {
        let state = TacticState::new(vec![id(1), id(2)]);
        assert!(has_multiple_goals(&state));
        assert!(!has_single_goal(&state));
    }
    #[test]
    fn test_apply_to_all_goals_closes_some() {
        let mut state = TacticState::new(vec![id(1), id(2)]);
        let closed = apply_to_all_goals(&mut state, |s| {
            s.replace_goal(vec![]);
            Ok(())
        });
        let _ = closed;
    }
}
#[cfg(test)]
mod goal_diff_tests {
    use super::*;
    use crate::tactic::state::*;
    fn id(n: u64) -> MVarId {
        MVarId(n)
    }
    #[test]
    fn test_goal_diff_no_change() {
        let before = vec![id(1), id(2)];
        let after = vec![id(1), id(2)];
        let diff = GoalDiff::compute(&before, &after);
        assert!(!diff.has_change());
        assert_eq!(diff.net(), 0);
    }
    #[test]
    fn test_goal_diff_closed_one() {
        let before = vec![id(1), id(2)];
        let after = vec![id(2)];
        let diff = GoalDiff::compute(&before, &after);
        assert_eq!(diff.num_closed(), 1);
        assert_eq!(diff.num_opened(), 0);
        assert_eq!(diff.net(), 1);
    }
    #[test]
    fn test_goal_diff_opened_two() {
        let before = vec![id(1)];
        let after = vec![id(2), id(3)];
        let diff = GoalDiff::compute(&before, &after);
        assert_eq!(diff.num_opened(), 2);
        assert_eq!(diff.num_closed(), 1);
        assert_eq!(diff.net(), -1);
    }
    #[test]
    fn test_goal_diff_display() {
        let before = vec![id(1)];
        let after = vec![id(2), id(3)];
        let diff = GoalDiff::compute(&before, &after);
        let s = format!("{}", diff);
        assert!(s.contains("GoalDiff"));
    }
    #[test]
    fn test_checkpoint_from_state() {
        let state = TacticState::new(vec![id(10), id(20)]);
        let cp = TacticCheckpoint::from_state("main", &state);
        assert_eq!(cp.name, "main");
        assert_eq!(cp.num_goals(), 2);
    }
    #[test]
    fn test_checkpoint_display() {
        let state = TacticState::new(vec![id(1)]);
        let cp = TacticCheckpoint::from_state("step1", &state);
        let s = format!("{}", cp);
        assert!(s.contains("step1"));
        assert!(s.contains("1 goals"));
    }
    #[test]
    fn test_checkpoint_manager_save_load() {
        let state = TacticState::new(vec![id(5)]);
        let mut mgr = CheckpointManager::new();
        mgr.save("alpha", &state);
        let loaded = mgr.load("alpha");
        assert!(loaded.is_some());
        assert_eq!(loaded.expect("loaded should be valid").num_goals(), 1);
    }
    #[test]
    fn test_checkpoint_manager_load_missing() {
        let mgr = CheckpointManager::new();
        assert!(mgr.load("nonexistent").is_none());
    }
    #[test]
    fn test_checkpoint_manager_count() {
        let state = TacticState::new(vec![id(1)]);
        let mut mgr = CheckpointManager::new();
        mgr.save("a", &state);
        mgr.save("b", &state);
        assert_eq!(mgr.count(), 2);
    }
    #[test]
    fn test_checkpoint_manager_clear() {
        let state = TacticState::new(vec![id(1)]);
        let mut mgr = CheckpointManager::new();
        mgr.save("a", &state);
        mgr.clear();
        assert!(mgr.is_empty());
    }
    #[test]
    fn test_checkpoint_manager_names() {
        let state = TacticState::new(vec![id(1)]);
        let mut mgr = CheckpointManager::new();
        mgr.save("first", &state);
        mgr.save("second", &state);
        let names = mgr.names();
        assert!(names.contains(&"first"));
        assert!(names.contains(&"second"));
    }
}
#[cfg(test)]
mod tacstate_ext2_tests {
    use super::*;
    use crate::tactic::state::*;
    #[test]
    fn test_tacstate_ext_util_basic() {
        let mut u = TacStateExtUtil::new("test");
        u.push(10);
        u.push(20);
        assert_eq!(u.sum(), 30);
        assert_eq!(u.len(), 2);
    }
    #[test]
    fn test_tacstate_ext_util_min_max() {
        let mut u = TacStateExtUtil::new("mm");
        u.push(5);
        u.push(1);
        u.push(9);
        assert_eq!(u.min_val(), Some(1));
        assert_eq!(u.max_val(), Some(9));
    }
    #[test]
    fn test_tacstate_ext_util_flags() {
        let mut u = TacStateExtUtil::new("flags");
        u.set_flag(3);
        assert!(u.has_flag(3));
        assert!(!u.has_flag(2));
    }
    #[test]
    fn test_tacstate_ext_util_pop() {
        let mut u = TacStateExtUtil::new("pop");
        u.push(42);
        assert_eq!(u.pop(), Some(42));
        assert!(u.is_empty());
    }
    #[test]
    fn test_tacstate_ext_map_basic() {
        let mut m: TacStateExtMap<i32> = TacStateExtMap::new();
        m.insert("key", 42);
        assert_eq!(m.get("key"), Some(&42));
        assert!(m.contains("key"));
        assert!(!m.contains("other"));
    }
    #[test]
    fn test_tacstate_ext_map_get_or_default() {
        let mut m: TacStateExtMap<i32> = TacStateExtMap::new();
        m.insert("k", 5);
        assert_eq!(m.get_or_default("k"), 5);
        assert_eq!(m.get_or_default("missing"), 0);
    }
    #[test]
    fn test_tacstate_ext_map_keys_sorted() {
        let mut m: TacStateExtMap<i32> = TacStateExtMap::new();
        m.insert("z", 1);
        m.insert("a", 2);
        m.insert("m", 3);
        let keys = m.keys_sorted();
        assert_eq!(keys[0].as_str(), "a");
        assert_eq!(keys[2].as_str(), "z");
    }
    #[test]
    fn test_tacstate_window_mean() {
        let mut w = TacStateWindow::new(3);
        w.push(1.0);
        w.push(2.0);
        w.push(3.0);
        assert!((w.mean() - 2.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacstate_window_evict() {
        let mut w = TacStateWindow::new(2);
        w.push(10.0);
        w.push(20.0);
        w.push(30.0);
        assert_eq!(w.len(), 2);
        assert!((w.mean() - 25.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacstate_window_std_dev() {
        let mut w = TacStateWindow::new(10);
        for i in 0..10 {
            w.push(i as f64);
        }
        assert!(w.std_dev() > 0.0);
    }
    #[test]
    fn test_tacstate_builder_basic() {
        let b = TacStateBuilder::new("test")
            .add_item("a")
            .add_item("b")
            .set_config("key", "val");
        assert_eq!(b.item_count(), 2);
        assert!(b.has_config("key"));
        assert_eq!(b.get_config("key"), Some("val"));
    }
    #[test]
    fn test_tacstate_builder_summary() {
        let b = TacStateBuilder::new("suite").add_item("x");
        let s = b.build_summary();
        assert!(s.contains("suite"));
    }
    #[test]
    fn test_tacstate_state_machine_start() {
        let mut sm = TacStateStateMachine::new();
        assert!(sm.start());
        assert!(sm.state.is_running());
    }
    #[test]
    fn test_tacstate_state_machine_complete() {
        let mut sm = TacStateStateMachine::new();
        sm.start();
        sm.complete();
        assert!(sm.state.is_terminal());
    }
    #[test]
    fn test_tacstate_state_machine_fail() {
        let mut sm = TacStateStateMachine::new();
        sm.fail("oops");
        assert!(sm.state.is_terminal());
        assert_eq!(sm.state.error_msg(), Some("oops"));
    }
    #[test]
    fn test_tacstate_state_machine_no_transition_after_terminal() {
        let mut sm = TacStateStateMachine::new();
        sm.complete();
        assert!(!sm.start());
    }
    #[test]
    fn test_tacstate_work_queue_basic() {
        let mut wq = TacStateWorkQueue::new(10);
        wq.enqueue("task1".to_string());
        wq.enqueue("task2".to_string());
        assert_eq!(wq.pending_count(), 2);
        let t = wq.dequeue();
        assert_eq!(t, Some("task1".to_string()));
        assert_eq!(wq.processed_count(), 1);
    }
    #[test]
    fn test_tacstate_work_queue_capacity() {
        let mut wq = TacStateWorkQueue::new(2);
        wq.enqueue("a".to_string());
        wq.enqueue("b".to_string());
        assert!(wq.is_full());
        assert!(!wq.enqueue("c".to_string()));
    }
    #[test]
    fn test_tacstate_counter_map_basic() {
        let mut cm = TacStateCounterMap::new();
        cm.increment("apple");
        cm.increment("apple");
        cm.increment("banana");
        assert_eq!(cm.count("apple"), 2);
        assert_eq!(cm.count("banana"), 1);
        assert_eq!(cm.num_unique(), 2);
    }
    #[test]
    fn test_tacstate_counter_map_frequency() {
        let mut cm = TacStateCounterMap::new();
        cm.increment("a");
        cm.increment("a");
        cm.increment("b");
        assert!((cm.frequency("a") - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_tacstate_counter_map_most_common() {
        let mut cm = TacStateCounterMap::new();
        cm.increment("x");
        cm.increment("y");
        cm.increment("x");
        let (k, v) = cm.most_common().expect("most_common should succeed");
        assert_eq!(k.as_str(), "x");
        assert_eq!(v, 2);
    }
}
#[cfg(test)]
mod tacticstate_analysis_tests {
    use super::*;
    use crate::tactic::state::*;
    #[test]
    fn test_tacticstate_result_ok() {
        let r = TacticStateResult::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacticstate_result_err() {
        let r = TacticStateResult::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacticstate_result_partial() {
        let r = TacticStateResult::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_tacticstate_result_skipped() {
        let r = TacticStateResult::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_tacticstate_analysis_pass_run() {
        let mut p = TacticStateAnalysisPass::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_tacticstate_analysis_pass_empty_input() {
        let mut p = TacticStateAnalysisPass::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_tacticstate_analysis_pass_success_rate() {
        let mut p = TacticStateAnalysisPass::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_tacticstate_analysis_pass_disable() {
        let mut p = TacticStateAnalysisPass::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_tacticstate_pipeline_basic() {
        let mut pipeline = TacticStatePipeline::new("main_pipeline");
        pipeline.add_pass(TacticStateAnalysisPass::new("pass1"));
        pipeline.add_pass(TacticStateAnalysisPass::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_tacticstate_pipeline_disabled_pass() {
        let mut pipeline = TacticStatePipeline::new("partial");
        let mut p = TacticStateAnalysisPass::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(TacticStateAnalysisPass::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_tacticstate_diff_basic() {
        let mut d = TacticStateDiff::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_tacticstate_diff_summary() {
        let mut d = TacticStateDiff::new();
        d.add("x");
        d.add("y");
        d.remove("z");
        let s = d.summary();
        assert!(s.contains("+2"));
    }
    #[test]
    fn test_tacticstate_config_set_get() {
        let mut cfg = TacticStateConfig::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_tacticstate_config_read_only() {
        let mut cfg = TacticStateConfig::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_tacticstate_config_remove() {
        let mut cfg = TacticStateConfig::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_tacticstate_diagnostics_basic() {
        let mut diag = TacticStateDiagnostics::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_tacticstate_diagnostics_max_errors() {
        let mut diag = TacticStateDiagnostics::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_tacticstate_diagnostics_clear() {
        let mut diag = TacticStateDiagnostics::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_tacticstate_config_value_types() {
        let b = TacticStateConfigValue::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = TacticStateConfigValue::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = TacticStateConfigValue::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = TacticStateConfigValue::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = TacticStateConfigValue::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
#[cfg(test)]
mod state_ext_tests_1800 {
    use super::*;
    use crate::tactic::state::*;
    #[test]
    fn test_state_ext_result_ok_1800() {
        let r = StateExtResult1800::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_state_ext_result_err_1800() {
        let r = StateExtResult1800::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_state_ext_result_partial_1800() {
        let r = StateExtResult1800::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_state_ext_result_skipped_1800() {
        let r = StateExtResult1800::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_state_ext_pass_run_1800() {
        let mut p = StateExtPass1800::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_state_ext_pass_empty_1800() {
        let mut p = StateExtPass1800::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_state_ext_pass_rate_1800() {
        let mut p = StateExtPass1800::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_state_ext_pass_disable_1800() {
        let mut p = StateExtPass1800::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_state_ext_pipeline_basic_1800() {
        let mut pipeline = StateExtPipeline1800::new("main_pipeline");
        pipeline.add_pass(StateExtPass1800::new("pass1"));
        pipeline.add_pass(StateExtPass1800::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_state_ext_pipeline_disabled_1800() {
        let mut pipeline = StateExtPipeline1800::new("partial");
        let mut p = StateExtPass1800::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(StateExtPass1800::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_state_ext_diff_basic_1800() {
        let mut d = StateExtDiff1800::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_state_ext_config_set_get_1800() {
        let mut cfg = StateExtConfig1800::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_state_ext_config_read_only_1800() {
        let mut cfg = StateExtConfig1800::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_state_ext_config_remove_1800() {
        let mut cfg = StateExtConfig1800::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_state_ext_diagnostics_basic_1800() {
        let mut diag = StateExtDiag1800::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_state_ext_diagnostics_max_errors_1800() {
        let mut diag = StateExtDiag1800::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_state_ext_diagnostics_clear_1800() {
        let mut diag = StateExtDiag1800::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_state_ext_config_value_types_1800() {
        let b = StateExtConfigVal1800::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = StateExtConfigVal1800::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = StateExtConfigVal1800::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = StateExtConfigVal1800::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = StateExtConfigVal1800::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
