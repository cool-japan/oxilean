//! Tests for `ProofNavigator::is_complete` before and after a step.

use super::super::types::{ProofNavigator, ProofStep};
use oxilean_elab::{Goal, TacticState};
use oxilean_kernel::{Expr, Name};

fn open_state() -> TacticState {
    let mut state = TacticState::new();
    state.add_goal(Goal::new(
        Name::str("g"),
        Expr::Const(Name::str("True"), vec![]),
    ));
    state
}

#[test]
fn completion_follows_the_last_step_or_the_initial_state() {
    let open = ProofNavigator::new(open_state());
    assert!(!open.is_complete());
    let closed = ProofNavigator::new(TacticState::new());
    assert!(closed.is_complete());
    let mut navigator = ProofNavigator::new(open_state());
    navigator.add_step(ProofStep {
        tactic: "trivial".to_string(),
        state_before: open_state(),
        state_after: TacticState::new(),
    });
    assert!(navigator.is_complete());
    navigator.add_step(ProofStep {
        tactic: "skip".to_string(),
        state_before: TacticState::new(),
        state_after: open_state(),
    });
    assert!(!navigator.is_complete());
}
