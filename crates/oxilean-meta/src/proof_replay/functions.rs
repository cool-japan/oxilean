//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    CalcEntry, Constraint, ConstraintExpr, MetaExpr, ProofCompressor, ProofReplayAnalysisPass,
    ProofReplayConfig, ProofReplayConfigValue, ProofReplayDiagnostics, ProofReplayDiff,
    ProofReplayExtConfig2600, ProofReplayExtConfigVal2600, ProofReplayExtDiag2600,
    ProofReplayExtDiff2600, ProofReplayExtPass2600, ProofReplayExtPipeline2600,
    ProofReplayExtResult2600, ProofReplayPipeline, ProofReplayResult, ProofReplayer, ProofScript,
    ProofSerializer, ProofStep, ReplayError, ReplayState,
};
use oxilean_kernel::Name;
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

/// Extract the binder type from a ∀-goal string.
///
/// Given a goal like `∀ x : T, Body` or `∀ (x : T), Body`, returns `T`.
/// Falls back to `"?"` if the type cannot be parsed.
pub(super) fn extract_binder_type(goal: &str, _var_name: &str) -> String {
    let rest = goal
        .trim_start_matches('∀')
        .trim_start_matches("forall")
        .trim_start_matches('(')
        .trim();
    if let Some(colon_pos) = rest.find(':') {
        let after_colon = rest[colon_pos + 1..].trim();
        let type_end = after_colon.find([',', ')']).unwrap_or(after_colon.len());
        after_colon[..type_end].trim().to_string()
    } else {
        "?".to_string()
    }
}
/// Split an arrow type `A -> B` or `A → B` into `(A, B)`.
///
/// Handles simple non-nested arrows. For nested types this is approximate.
pub(super) fn split_arrow(goal: &str) -> (String, String) {
    let arrow = if goal.contains("->") { "->" } else { "→" };
    if let Some(pos) = goal.find(arrow) {
        let antecedent = goal[..pos].trim().to_string();
        let consequent = goal[pos + arrow.len()..].trim().to_string();
        (antecedent, consequent)
    } else {
        ("?".to_string(), goal.to_string())
    }
}
/// Return true if a constraint expression is trivially satisfied and can be dropped.
pub(super) fn is_trivially_true(expr: &ConstraintExpr) -> bool {
    match expr {
        ConstraintExpr::Eq(lhs, rhs) => lhs == rhs,
        ConstraintExpr::Lit(s) => s == "true",
        _ => false,
    }
}
/// Collect the hypothesis/variable names that a step *defines* (produces).
pub(super) fn step_defines(step: &ProofStep) -> Vec<String> {
    match step {
        ProofStep::Intro { name } => {
            vec![name
                .as_ref()
                .map(|n| n.to_string())
                .unwrap_or_else(|| "x".to_string())]
        }
        ProofStep::Have { name, .. } => {
            vec![name
                .as_ref()
                .map(|n| n.to_string())
                .unwrap_or_else(|| "h".to_string())]
        }
        ProofStep::Sequence { steps } => steps.iter().flat_map(step_defines).collect(),
        _ => vec![],
    }
}
/// Collect the hypothesis/variable names that a step *uses* (reads).
pub(super) fn step_uses(step: &ProofStep) -> Vec<String> {
    match step {
        ProofStep::Apply { term } | ProofStep::Exact { term } => vec![term.clone()],
        ProofStep::Rewrite { lemma, .. } => vec![lemma.clone()],
        ProofStep::Cases { term } | ProofStep::Induction { term } => vec![term.clone()],
        ProofStep::Clear { name } | ProofStep::Subst { var: name } => {
            vec![name.to_string()]
        }
        ProofStep::Simp { lemmas, .. } => lemmas.clone(),
        ProofStep::Sequence { steps } => steps.iter().flat_map(step_uses).collect(),
        ProofStep::First { alternatives } => alternatives.iter().flat_map(step_uses).collect(),
        _ => vec![],
    }
}
pub(super) const METAVAR_THRESHOLD: u64 = 1_000_000;
#[cfg(test)]
mod tests {
    use super::*;
    use crate::proof_replay::*;
    #[test]
    fn test_proof_script_creation() {
        let script = ProofScript::new("True".to_string(), vec![]);
        assert_eq!(script.goal(), "True");
        assert!(script.hypotheses().is_empty());
    }
    #[test]
    fn test_add_single_step() {
        let mut script = ProofScript::new("True".to_string(), vec![]);
        script.add_step(ProofStep::Trivial);
        assert_eq!(script.steps().len(), 1);
    }
    #[test]
    fn test_add_multiple_steps() {
        let mut script = ProofScript::new("a → b".to_string(), vec![]);
        script.add_steps(vec![ProofStep::Intro { name: None }, ProofStep::Trivial]);
        assert_eq!(script.steps().len(), 2);
    }
    #[test]
    fn test_calc_entry_creation() {
        let entry = CalcEntry {
            rel: "=".to_string(),
            lhs: "x + y".to_string(),
            rhs: "y + x".to_string(),
            proof: "add_comm".to_string(),
        };
        assert_eq!(entry.rel, "=");
    }
    #[test]
    fn test_replay_error_display() {
        let err = ReplayError::InvalidStructure("test".to_string());
        assert!(format!("{}", err).contains("Invalid structure"));
    }
    #[test]
    fn test_replay_state_creation() {
        let state = ReplayState::new("True".to_string(), vec![]);
        assert_eq!(state.goal(), "True");
        assert!(!state.is_complete());
    }
    #[test]
    fn test_replay_state_with_hypotheses() {
        let hyps = vec![(Name::str("h"), "P".to_string())];
        let state = ReplayState::new("Q".to_string(), hyps);
        assert!(state.hypotheses().contains_key("h"));
    }
    #[test]
    fn test_proof_replayer_creation() {
        let replayer = ProofReplayer::new();
        let replayer2 = ProofReplayer::with_history(500);
        assert_eq!(replayer.max_history, 1000);
        assert_eq!(replayer2.max_history, 500);
    }
    #[test]
    fn test_replayer_default() {
        let _replayer = ProofReplayer::default();
    }
    #[test]
    fn test_exact_tactic() {
        let _script = ProofScript::new("True".to_string(), vec![]);
        let replayer = ProofReplayer::new();
        let mut state = ReplayState::new("True".to_string(), vec![]);
        replayer
            .replay_step(
                &ProofStep::Exact {
                    term: "trivial".to_string(),
                },
                &mut state,
            )
            .expect("value should be present");
        assert!(state.is_complete());
    }
    #[test]
    fn test_intro_valid_goal_forall() {
        let replayer = ProofReplayer::new();
        let mut state = ReplayState::new("∀x, P x".to_string(), vec![]);
        replayer
            .replay_step(
                &ProofStep::Intro {
                    name: Some(Name::str("x")),
                },
                &mut state,
            )
            .expect("value should be present");
        assert!(state.hypotheses().contains_key("x"));
    }
    #[test]
    fn test_intro_valid_goal_arrow() {
        let replayer = ProofReplayer::new();
        let mut state = ReplayState::new("A -> B".to_string(), vec![]);
        replayer
            .replay_step(
                &ProofStep::Intro {
                    name: Some(Name::str("h")),
                },
                &mut state,
            )
            .expect("value should be present");
        assert!(state.hypotheses().contains_key("h"));
        assert_eq!(state.goal(), "B");
    }
    #[test]
    fn test_intro_invalid_goal() {
        let replayer = ProofReplayer::new();
        let mut state = ReplayState::new("True".to_string(), vec![]);
        let result = replayer.replay_step(&ProofStep::Intro { name: None }, &mut state);
        assert!(result.is_err());
    }
    #[test]
    fn test_clear_hypothesis() {
        let hyps = vec![(Name::str("h"), "P".to_string())];
        let replayer = ProofReplayer::new();
        let mut state = ReplayState::new("Q".to_string(), hyps);
        replayer
            .replay_step(
                &ProofStep::Clear {
                    name: Name::str("h"),
                },
                &mut state,
            )
            .expect("value should be present");
        assert!(!state.hypotheses().contains_key("h"));
    }
    #[test]
    fn test_clear_nonexistent_hypothesis() {
        let replayer = ProofReplayer::new();
        let mut state = ReplayState::new("Q".to_string(), vec![]);
        let result = replayer.replay_step(
            &ProofStep::Clear {
                name: Name::str("h"),
            },
            &mut state,
        );
        assert!(result.is_err());
    }
    #[test]
    fn test_have_tactic() {
        let replayer = ProofReplayer::new();
        let mut state = ReplayState::new("P → Q".to_string(), vec![]);
        replayer
            .replay_step(
                &ProofStep::Have {
                    name: Some(Name::str("h")),
                    ty: "P".to_string(),
                    proof: "assumption".to_string(),
                },
                &mut state,
            )
            .expect("value should be present");
        assert!(state.hypotheses().contains_key("h"));
    }
    #[test]
    fn test_apply_tactic() {
        let replayer = ProofReplayer::new();
        let mut state = ReplayState::new("P → Q".to_string(), vec![]);
        replayer
            .replay_step(
                &ProofStep::Apply {
                    term: "modus_ponens".to_string(),
                },
                &mut state,
            )
            .expect("value should be present");
    }
    #[test]
    fn test_apply_known_hypothesis() {
        let hyps = vec![(Name::str("h"), "A -> B".to_string())];
        let replayer = ProofReplayer::new();
        let mut state = ReplayState::new("B".to_string(), hyps);
        replayer
            .replay_step(
                &ProofStep::Apply {
                    term: "h".to_string(),
                },
                &mut state,
            )
            .expect("value should be present");
        assert_eq!(state.goal(), "A");
    }
    #[test]
    fn test_trivial_completes_proof() {
        let replayer = ProofReplayer::new();
        let mut state = ReplayState::new("True".to_string(), vec![]);
        replayer
            .replay_step(&ProofStep::Trivial, &mut state)
            .expect("value should be present");
        assert!(state.is_complete());
    }
    #[test]
    fn test_omega_completes_proof() {
        let replayer = ProofReplayer::new();
        let mut state = ReplayState::new("1 < 2".to_string(), vec![]);
        replayer
            .replay_step(&ProofStep::Omega, &mut state)
            .expect("value should be present");
        assert!(state.is_complete());
    }
    #[test]
    fn test_sequence_tactic() {
        let replayer = ProofReplayer::new();
        let mut state = ReplayState::new("a → b".to_string(), vec![]);
        replayer
            .replay_step(
                &ProofStep::Sequence {
                    steps: vec![ProofStep::Intro { name: None }, ProofStep::Trivial],
                },
                &mut state,
            )
            .ok();
    }
    #[test]
    fn test_first_tactic_success() {
        let replayer = ProofReplayer::new();
        let mut state = ReplayState::new("True".to_string(), vec![]);
        replayer
            .replay_step(
                &ProofStep::First {
                    alternatives: vec![ProofStep::Trivial],
                },
                &mut state,
            )
            .expect("value should be present");
        assert!(state.is_complete());
    }
    #[test]
    fn test_validate_structure_valid() {
        let script = ProofScript::new("True".to_string(), vec![]);
        assert!(script.validate_structure().is_ok());
    }
    #[test]
    fn test_validate_empty_first() {
        let mut script = ProofScript::new("True".to_string(), vec![]);
        script.add_step(ProofStep::First {
            alternatives: vec![],
        });
        assert!(script.validate_structure().is_err());
    }
    #[test]
    fn test_serialize_deserialize() {
        let script = ProofScript::new("True".to_string(), vec![(Name::str("h"), "P".to_string())]);
        let serialized = ProofSerializer::serialize(&script).expect("serialized should be present");
        let deserialized =
            ProofSerializer::deserialize(&serialized).expect("deserialized should be present");
        assert_eq!(deserialized.goal(), "True");
        assert_eq!(deserialized.hypotheses().len(), 1);
    }
    #[test]
    fn test_deserialize_bad_magic() {
        let bad_data = b"WRONG";
        let result = ProofSerializer::deserialize(bad_data);
        assert!(result.is_err());
    }
    #[test]
    fn test_compress_proof() {
        let mut script = ProofScript::new("True".to_string(), vec![]);
        script.add_steps(vec![ProofStep::Trivial, ProofStep::Trivial]);
        ProofCompressor::compress(&mut script).expect("value should be present");
    }
    #[test]
    fn test_merge_sequences() {
        let steps = vec![
            ProofStep::Sequence {
                steps: vec![ProofStep::Trivial],
            },
            ProofStep::Sequence {
                steps: vec![ProofStep::Exact {
                    term: "proof".to_string(),
                }],
            },
        ];
        let merged = ProofCompressor::merge_sequences(&steps);
        assert!(!merged.is_empty());
    }
    #[test]
    fn test_calc_steps() {
        let mut script = ProofScript::new("x = x".to_string(), vec![]);
        script.add_step(ProofStep::Calc {
            entries: vec![CalcEntry {
                rel: "=".to_string(),
                lhs: "x".to_string(),
                rhs: "x".to_string(),
                proof: "rfl".to_string(),
            }],
        });
        let replayer = ProofReplayer::new();
        let mut state = ReplayState::new("x = x".to_string(), vec![]);
        replayer
            .replay_step(&script.steps()[0], &mut state)
            .expect("value should be present");
    }
    #[test]
    fn test_simp_tactic() {
        let replayer = ProofReplayer::new();
        let mut state = ReplayState::new("x + 0 = x".to_string(), vec![]);
        replayer
            .replay_step(
                &ProofStep::Simp {
                    lemmas: vec!["add_zero".to_string()],
                    use_default: false,
                },
                &mut state,
            )
            .expect("value should be present");
    }
    #[test]
    fn test_rewrite_tactic() {
        let replayer = ProofReplayer::new();
        let mut state = ReplayState::new("x + y = y + x".to_string(), vec![]);
        replayer
            .replay_step(
                &ProofStep::Rewrite {
                    lemma: "add_comm".to_string(),
                    location: None,
                },
                &mut state,
            )
            .expect("value should be present");
    }
    #[test]
    fn test_cases_tactic() {
        let hyps = vec![(Name::str("h"), "P ∨ Q".to_string())];
        let replayer = ProofReplayer::new();
        let mut state = ReplayState::new("R".to_string(), hyps);
        replayer
            .replay_step(
                &ProofStep::Cases {
                    term: "h".to_string(),
                },
                &mut state,
            )
            .expect("value should be present");
    }
    #[test]
    fn test_induction_tactic() {
        let hyps = vec![(Name::str("n"), "Nat".to_string())];
        let replayer = ProofReplayer::new();
        let mut state = ReplayState::new("P n".to_string(), hyps);
        replayer
            .replay_step(
                &ProofStep::Induction {
                    term: "n".to_string(),
                },
                &mut state,
            )
            .expect("value should be present");
    }
    #[test]
    fn test_constraint_expr_substitute_var() {
        let expr = ConstraintExpr::Var("x".to_string());
        let val = ConstraintExpr::Lit("42".to_string());
        let result = expr.substitute("x", &val);
        assert_eq!(result, ConstraintExpr::Lit("42".to_string()));
    }
    #[test]
    fn test_constraint_expr_substitute_no_match() {
        let expr = ConstraintExpr::Var("y".to_string());
        let val = ConstraintExpr::Lit("42".to_string());
        let result = expr.substitute("x", &val);
        assert_eq!(result, ConstraintExpr::Var("y".to_string()));
    }
    #[test]
    fn test_constraint_expr_substitute_in_eq() {
        let expr = ConstraintExpr::Eq(
            Box::new(ConstraintExpr::Var("x".to_string())),
            Box::new(ConstraintExpr::Var("x".to_string())),
        );
        let val = ConstraintExpr::Lit("5".to_string());
        let result = expr.substitute("x", &val);
        let expected = ConstraintExpr::Eq(
            Box::new(ConstraintExpr::Lit("5".to_string())),
            Box::new(ConstraintExpr::Lit("5".to_string())),
        );
        assert_eq!(result, expected);
    }
    #[test]
    fn test_apply_assignment_removes_trivial() {
        let constraints = vec![Constraint::new(ConstraintExpr::Eq(
            Box::new(ConstraintExpr::Var("x".to_string())),
            Box::new(ConstraintExpr::Var("x".to_string())),
        ))];
        let val = ConstraintExpr::Lit("5".to_string());
        let result = ProofCompressor::apply_assignment("x", &val, constraints);
        assert!(result.is_empty());
    }
    #[test]
    fn test_apply_assignment_non_trivial_retained() {
        let constraints = vec![Constraint::new(ConstraintExpr::Eq(
            Box::new(ConstraintExpr::Var("x".to_string())),
            Box::new(ConstraintExpr::Var("y".to_string())),
        ))];
        let val = ConstraintExpr::Lit("5".to_string());
        let result = ProofCompressor::apply_assignment("x", &val, constraints);
        assert_eq!(result.len(), 1);
    }
    #[test]
    fn test_apply_assignment_with_label() {
        let constraints = vec![Constraint::labelled(
            ConstraintExpr::Var("x".to_string()),
            "my_label",
        )];
        let val = ConstraintExpr::Lit("true".to_string());
        let result = ProofCompressor::apply_assignment("x", &val, constraints);
        assert!(result.is_empty());
    }
    #[test]
    fn test_is_step_live_goal_closer() {
        let steps = vec![ProofStep::Exact {
            term: "h".to_string(),
        }];
        let live = ProofCompressor::eliminate_dead_steps(&steps).expect("live should be present");
        assert_eq!(live.len(), 1);
    }
    #[test]
    fn test_is_step_live_dead_have() {
        let steps = vec![
            ProofStep::Have {
                name: Some(Name::str("h2")),
                ty: "P".to_string(),
                proof: "sorry".to_string(),
            },
            ProofStep::Trivial,
        ];
        let live = ProofCompressor::eliminate_dead_steps(&steps).expect("live should be present");
        assert!(live.iter().any(|s| matches!(s, ProofStep::Trivial)));
        assert!(!live
            .iter()
            .any(|s| matches!(s, ProofStep::Have { name, .. } if name
            .as_ref().map(| n | n.to_string()).as_deref() == Some("h2"))));
    }
    #[test]
    fn test_is_step_live_used_have() {
        let steps = vec![
            ProofStep::Have {
                name: Some(Name::str("h")),
                ty: "P".to_string(),
                proof: "sorry".to_string(),
            },
            ProofStep::Exact {
                term: "h".to_string(),
            },
        ];
        let live = ProofCompressor::eliminate_dead_steps(&steps).expect("live should be present");
        assert_eq!(live.len(), 2);
    }
    #[test]
    fn test_meta_expr_apply_assignments_var_below_threshold() {
        let expr = MetaExpr::FVar(500_000);
        let assignments = HashMap::new();
        let result = expr.apply_assignments(&assignments);
        assert_eq!(result, MetaExpr::FVar(500_000));
    }
    #[test]
    fn test_meta_expr_apply_assignments_metavar_resolved() {
        let mut assignments = HashMap::new();
        assignments.insert(1_000_001, MetaExpr::Const("Nat".to_string()));
        let expr = MetaExpr::FVar(1_000_001);
        let result = expr.apply_assignments(&assignments);
        assert_eq!(result, MetaExpr::Const("Nat".to_string()));
    }
    #[test]
    fn test_meta_expr_apply_assignments_metavar_unresolved() {
        let assignments = HashMap::new();
        let expr = MetaExpr::FVar(1_000_042);
        let result = expr.apply_assignments(&assignments);
        assert_eq!(result, MetaExpr::FVar(1_000_042));
    }
    #[test]
    fn test_meta_expr_apply_assignments_app_recurse() {
        let mut assignments = HashMap::new();
        assignments.insert(1_000_001u64, MetaExpr::Const("Nat".to_string()));
        let expr = MetaExpr::App(
            Box::new(MetaExpr::Const("List".to_string())),
            Box::new(MetaExpr::FVar(1_000_001)),
        );
        let result = expr.apply_assignments(&assignments);
        let expected = MetaExpr::App(
            Box::new(MetaExpr::Const("List".to_string())),
            Box::new(MetaExpr::Const("Nat".to_string())),
        );
        assert_eq!(result, expected);
    }
    #[test]
    fn test_meta_expr_apply_assignments_chained() {
        let mut assignments = HashMap::new();
        assignments.insert(1_000_001u64, MetaExpr::FVar(1_000_002));
        assignments.insert(1_000_002u64, MetaExpr::Const("Nat".to_string()));
        let expr = MetaExpr::FVar(1_000_001);
        let result = expr.apply_assignments(&assignments);
        assert_eq!(result, MetaExpr::Const("Nat".to_string()));
    }
    #[test]
    fn test_meta_expr_apply_assignments_lam() {
        let mut assignments = HashMap::new();
        assignments.insert(1_000_001u64, MetaExpr::Const("Int".to_string()));
        let expr = MetaExpr::Lam(
            "x".to_string(),
            Box::new(MetaExpr::FVar(1_000_001)),
            Box::new(MetaExpr::Const("Prop".to_string())),
        );
        let result = expr.apply_assignments(&assignments);
        let expected = MetaExpr::Lam(
            "x".to_string(),
            Box::new(MetaExpr::Const("Int".to_string())),
            Box::new(MetaExpr::Const("Prop".to_string())),
        );
        assert_eq!(result, expected);
    }
    #[test]
    fn test_meta_expr_has_unresolved_metavars() {
        let assignments: HashMap<u64, MetaExpr> = HashMap::new();
        let expr = MetaExpr::FVar(1_000_001);
        assert!(expr.has_unresolved_metavars(&assignments));
    }
    #[test]
    fn test_meta_expr_no_unresolved_metavars() {
        let mut assignments = HashMap::new();
        assignments.insert(1_000_001u64, MetaExpr::Const("Nat".to_string()));
        let expr = MetaExpr::FVar(1_000_001);
        assert!(!expr.has_unresolved_metavars(&assignments));
    }
    #[test]
    fn test_constraint_expr_free_vars() {
        let expr = ConstraintExpr::And(
            Box::new(ConstraintExpr::Var("x".to_string())),
            Box::new(ConstraintExpr::Var("y".to_string())),
        );
        let vars = expr.free_vars();
        assert!(vars.contains("x"));
        assert!(vars.contains("y"));
        assert_eq!(vars.len(), 2);
    }
    #[test]
    fn test_split_arrow() {
        let (a, b) = split_arrow("A -> B");
        assert_eq!(a, "A");
        assert_eq!(b, "B");
    }
    #[test]
    fn test_extract_binder_type() {
        let ty = extract_binder_type("∀ x : Nat, P x", "x");
        assert_eq!(ty, "Nat");
    }
}
#[cfg(test)]
mod proofreplay_analysis_tests {
    use super::*;
    use crate::proof_replay::*;
    #[test]
    fn test_proofreplay_result_ok() {
        let r = ProofReplayResult::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_proofreplay_result_err() {
        let r = ProofReplayResult::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_proofreplay_result_partial() {
        let r = ProofReplayResult::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_proofreplay_result_skipped() {
        let r = ProofReplayResult::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_proofreplay_analysis_pass_run() {
        let mut p = ProofReplayAnalysisPass::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_proofreplay_analysis_pass_empty_input() {
        let mut p = ProofReplayAnalysisPass::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_proofreplay_analysis_pass_success_rate() {
        let mut p = ProofReplayAnalysisPass::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_proofreplay_analysis_pass_disable() {
        let mut p = ProofReplayAnalysisPass::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_proofreplay_pipeline_basic() {
        let mut pipeline = ProofReplayPipeline::new("main_pipeline");
        pipeline.add_pass(ProofReplayAnalysisPass::new("pass1"));
        pipeline.add_pass(ProofReplayAnalysisPass::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_proofreplay_pipeline_disabled_pass() {
        let mut pipeline = ProofReplayPipeline::new("partial");
        let mut p = ProofReplayAnalysisPass::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(ProofReplayAnalysisPass::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_proofreplay_diff_basic() {
        let mut d = ProofReplayDiff::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_proofreplay_diff_summary() {
        let mut d = ProofReplayDiff::new();
        d.add("x");
        d.add("y");
        d.remove("z");
        let s = d.summary();
        assert!(s.contains("+2"));
    }
    #[test]
    fn test_proofreplay_config_set_get() {
        let mut cfg = ProofReplayConfig::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_proofreplay_config_read_only() {
        let mut cfg = ProofReplayConfig::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_proofreplay_config_remove() {
        let mut cfg = ProofReplayConfig::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_proofreplay_diagnostics_basic() {
        let mut diag = ProofReplayDiagnostics::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_proofreplay_diagnostics_max_errors() {
        let mut diag = ProofReplayDiagnostics::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_proofreplay_diagnostics_clear() {
        let mut diag = ProofReplayDiagnostics::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_proofreplay_config_value_types() {
        let b = ProofReplayConfigValue::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = ProofReplayConfigValue::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = ProofReplayConfigValue::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = ProofReplayConfigValue::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = ProofReplayConfigValue::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
#[cfg(test)]
mod proof_replay_ext_tests_2600 {
    use super::*;
    use crate::proof_replay::*;
    #[test]
    fn test_proof_replay_ext_result_ok_2600() {
        let r = ProofReplayExtResult2600::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_proof_replay_ext_result_err_2600() {
        let r = ProofReplayExtResult2600::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_proof_replay_ext_result_partial_2600() {
        let r = ProofReplayExtResult2600::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_proof_replay_ext_result_skipped_2600() {
        let r = ProofReplayExtResult2600::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_proof_replay_ext_pass_run_2600() {
        let mut p = ProofReplayExtPass2600::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_proof_replay_ext_pass_empty_2600() {
        let mut p = ProofReplayExtPass2600::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_proof_replay_ext_pass_rate_2600() {
        let mut p = ProofReplayExtPass2600::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_proof_replay_ext_pass_disable_2600() {
        let mut p = ProofReplayExtPass2600::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_proof_replay_ext_pipeline_basic_2600() {
        let mut pipeline = ProofReplayExtPipeline2600::new("main_pipeline");
        pipeline.add_pass(ProofReplayExtPass2600::new("pass1"));
        pipeline.add_pass(ProofReplayExtPass2600::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_proof_replay_ext_pipeline_disabled_2600() {
        let mut pipeline = ProofReplayExtPipeline2600::new("partial");
        let mut p = ProofReplayExtPass2600::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(ProofReplayExtPass2600::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_proof_replay_ext_diff_basic_2600() {
        let mut d = ProofReplayExtDiff2600::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_proof_replay_ext_config_set_get_2600() {
        let mut cfg = ProofReplayExtConfig2600::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_proof_replay_ext_config_read_only_2600() {
        let mut cfg = ProofReplayExtConfig2600::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_proof_replay_ext_config_remove_2600() {
        let mut cfg = ProofReplayExtConfig2600::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_proof_replay_ext_diagnostics_basic_2600() {
        let mut diag = ProofReplayExtDiag2600::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_proof_replay_ext_diagnostics_max_errors_2600() {
        let mut diag = ProofReplayExtDiag2600::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_proof_replay_ext_diagnostics_clear_2600() {
        let mut diag = ProofReplayExtDiag2600::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_proof_replay_ext_config_value_types_2600() {
        let b = ProofReplayExtConfigVal2600::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = ProofReplayExtConfigVal2600::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = ProofReplayExtConfigVal2600::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = ProofReplayExtConfigVal2600::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = ProofReplayExtConfigVal2600::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
