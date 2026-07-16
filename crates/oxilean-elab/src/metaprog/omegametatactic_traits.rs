//! # OmegaMetaTactic - Trait Implementations
//!
//! This module contains trait implementations for `OmegaMetaTactic`.
//!
//! ## Implemented Traits
//!
//! - `UserTactic`
//!
//! The implementation uses a real Omega test (Pugh's algorithm) for deciding
//! Presburger arithmetic goals.  Parsing, the Omega test algorithm, and helper
//! utilities live in the sibling `omega_engine` module.

use super::functions::UserTactic;
use super::omega_engine::{run_omega_tactic, OmegaTacticResult};
use super::types::{OmegaMetaTactic, UserTacticResult};

impl UserTactic for OmegaMetaTactic {
    fn name(&self) -> &str {
        "omega"
    }

    fn run(&self, goal_target: &str, hypotheses: &[(String, String)]) -> UserTacticResult {
        match run_omega_tactic(goal_target, hypotheses) {
            OmegaTacticResult::Proved => UserTacticResult::Solved,
            OmegaTacticResult::Failed(msg) => UserTacticResult::Failed(msg),
        }
    }

    fn description(&self) -> &str {
        "Closes linear arithmetic goals over integers using Pugh's Omega test (Presburger arithmetic)"
    }
}

#[cfg(test)]
mod tactic_tests {
    use super::*;

    fn omega() -> OmegaMetaTactic {
        OmegaMetaTactic
    }

    /// omega_trivial_true: proves `2 <= 3`.
    #[test]
    fn omega_trivial_true() {
        assert!(
            matches!(omega().run("2 <= 3", &[]), UserTacticResult::Solved),
            "2 <= 3 should be solved by omega"
        );
    }

    /// omega_variable: proves `x + 1 > x`.
    #[test]
    fn omega_variable() {
        assert!(
            matches!(omega().run("x + 1 > x", &[]), UserTacticResult::Solved),
            "x + 1 > x should be solved by omega"
        );
    }

    /// omega_ground_false: fails on `3 < 2`.
    #[test]
    fn omega_ground_false() {
        assert!(
            matches!(omega().run("3 < 2", &[]), UserTacticResult::Failed(_)),
            "3 < 2 should NOT be solved by omega"
        );
    }

    /// omega_conjunction: proves `a >= 0 ∧ b >= 0 → a + b >= 0`.
    #[test]
    fn omega_conjunction() {
        assert!(
            matches!(
                omega().run("a >= 0 ∧ b >= 0 → a + b >= 0", &[]),
                UserTacticResult::Solved
            ),
            "a >= 0 ∧ b >= 0 → a + b >= 0 should be solved by omega"
        );
    }

    /// omega_unsat_system: detects that hypotheses `{x > 0, x < 0}` are contradictory.
    #[test]
    fn omega_unsat_system() {
        let hyps = vec![
            ("h1".to_string(), "x > 0".to_string()),
            ("h2".to_string(), "x < 0".to_string()),
        ];
        // Any goal should be provable when hypotheses are contradictory.
        assert!(
            matches!(omega().run("1 <= 0", &hyps), UserTacticResult::Solved),
            "contradictory hypotheses x > 0 ∧ x < 0 should make any goal provable"
        );
    }

    /// Goals that lack arithmetic operators still fail gracefully.
    #[test]
    fn omega_non_arithmetic_goal_fails() {
        let result = omega().run("True", &[]);
        assert!(
            matches!(result, UserTacticResult::Failed(_)),
            "omega should fail on non-arithmetic goals"
        );
    }

    /// Tautology: x + 2 >= x + 1.
    #[test]
    fn omega_shift_tautology() {
        assert!(
            matches!(omega().run("x + 2 >= x + 1", &[]), UserTacticResult::Solved),
            "x + 2 >= x + 1 is always true"
        );
    }

    /// Implication provable: x >= 1 -> x >= 0.
    #[test]
    fn omega_implication_provable() {
        assert!(
            matches!(
                omega().run("x >= 1 -> x >= 0", &[]),
                UserTacticResult::Solved
            ),
            "x >= 1 -> x >= 0 is provable"
        );
    }

    /// Implication not provable: x >= 0 -> x >= 1.
    #[test]
    fn omega_implication_not_provable() {
        assert!(
            matches!(
                omega().run("x >= 0 -> x >= 1", &[]),
                UserTacticResult::Failed(_)
            ),
            "x >= 0 -> x >= 1 is not provable"
        );
    }
}
