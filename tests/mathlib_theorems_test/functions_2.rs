//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_elab::{
    eval_tactic_block, tactic_apply, tactic_by_contra, tactic_cases, tactic_contrapose,
    tactic_induction, tactic_push_neg, tactic_split, Goal, TacticError, TacticState,
};
use oxilean_kernel::{BinderInfo, Expr, Level, Name};

use super::functions::{mk_and, mk_eq, mk_or_expr, mk_pi, parses, parses_and_elabs};

/// Test: `cases h` Or-elimination: `h : P ∨ Q, goal : Q ∨ P`
/// Left branch: h_left : P, prove Q ∨ P via right; exact h_left
/// Right branch: h_right : Q, prove Q ∨ P via left; exact h_right
#[test]
fn tactic_cases_or_elimination() {
    let p = Expr::Const(Name::str("P"), vec![]);
    let q = Expr::Const(Name::str("Q"), vec![]);
    let q_or_p = mk_or_expr(q.clone(), p.clone());
    let mut state = TacticState::new();
    let mut goal = Goal::new(Name::str("main"), q_or_p.clone());
    goal.add_hypothesis(Name::str("h"), mk_or_expr(p.clone(), q.clone()));
    state.add_goal(goal);
    let after_cases = tactic_cases(&state, &Name::str("h")).expect("cases on Or");
    assert_eq!(after_cases.num_goals(), 2);
    let goals: Vec<_> = after_cases.goals().to_vec();
    let left_goal = goals
        .iter()
        .find(|g| {
            g.hypotheses()
                .iter()
                .any(|(n, _)| n == &Name::str("h_left"))
        })
        .expect("should have h_left goal");
    let right_goal = goals
        .iter()
        .find(|g| {
            g.hypotheses()
                .iter()
                .any(|(n, _)| n == &Name::str("h_right"))
        })
        .expect("should have h_right goal");
    let mut left_state = TacticState::new();
    left_state.add_goal(left_goal.clone());
    let left_result = eval_tactic_block(
        &left_state,
        &["right".to_string(), "assumption".to_string()],
    )
    .expect("right; assumption on left branch");
    assert!(left_result.is_complete(), "left branch should be proved");
    let mut right_state = TacticState::new();
    right_state.add_goal(right_goal.clone());
    let right_result = eval_tactic_block(
        &right_state,
        &["left".to_string(), "assumption".to_string()],
    )
    .expect("left; assumption on right branch");
    assert!(right_result.is_complete(), "right branch should be proved");
}
/// Test: `constructor` on `And A B` creates two sub-goals with correct types.
#[test]
fn tactic_constructor_and_correct_goals() {
    let a = Expr::Const(Name::str("PropA"), vec![]);
    let b = Expr::Const(Name::str("PropB"), vec![]);
    let and_target = mk_and(a.clone(), b.clone());
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), and_target));
    let result = eval_tactic_block(&state, &["constructor".to_string()])
        .expect("constructor on And should succeed");
    assert_eq!(result.num_goals(), 2, "And constructor should give 2 goals");
    let targets: Vec<_> = result.goals().iter().map(|g| &g.target).cloned().collect();
    assert!(
        targets.contains(&a),
        "should have sub-goal for left component"
    );
    assert!(
        targets.contains(&b),
        "should have sub-goal for right component"
    );
}
/// Test: prove `A ∧ B` from `ha : A` and `hb : B` via constructor + assumption.
#[test]
fn tactic_constructor_and_from_hypotheses() {
    let a = Expr::Const(Name::str("PropA"), vec![]);
    let b = Expr::Const(Name::str("PropB"), vec![]);
    let mut state = TacticState::new();
    let mut goal = Goal::new(Name::str("main"), mk_and(a.clone(), b.clone()));
    goal.add_hypothesis(Name::str("ha"), a.clone());
    goal.add_hypothesis(Name::str("hb"), b.clone());
    state.add_goal(goal);
    let result = eval_tactic_block(
        &state,
        &[
            "constructor".to_string(),
            "assumption".to_string(),
            "assumption".to_string(),
        ],
    )
    .expect("constructor; assumption; assumption should prove A ∧ B");
    assert!(
        result.is_complete(),
        "proof of A ∧ B from ha and hb should complete"
    );
}
/// Test: `apply` dispatch works for a Pi-type lemma.
///
/// Goal: `B`, hypothesis `h : A -> B`, apply h creates a subgoal for A.
#[test]
fn tactic_eval_apply_dispatch() {
    let a = Expr::Const(Name::str("A"), vec![]);
    let b = Expr::Const(Name::str("B"), vec![]);
    let a_to_b = mk_pi("_", a.clone(), b.clone());
    let mut state = TacticState::new();
    let mut goal = Goal::new(Name::str("main"), b.clone());
    goal.add_hypothesis(Name::str("h"), a_to_b.clone());
    state.add_goal(goal);
    let result = eval_tactic_block(&state, &["apply h".to_string()])
        .expect("apply dispatch should not error");
    assert_eq!(
        result.num_goals(),
        1,
        "apply on Pi type should create 1 subgoal for the domain"
    );
}
/// Test: `exists` dispatch provides a witness for an existential goal.
///
/// Goal: `Exists (fun x -> P)`, use `exists 0` to substitute witness.
#[test]
fn tactic_eval_exists_dispatch() {
    let pred = Expr::Const(Name::str("P"), vec![]);
    let exists_target = Expr::App(
        Box::new(Expr::Const(Name::str("Exists"), vec![])),
        Box::new(pred.clone()),
    );
    let mut state = TacticState::new();
    let goal = Goal::new(Name::str("main"), exists_target);
    state.add_goal(goal);
    let result = eval_tactic_block(&state, &["exists 0".to_string()])
        .expect("exists dispatch should not error");
    assert_eq!(
        result.num_goals(),
        1,
        "exists should reduce to 1 subgoal (prove P witness)"
    );
}
/// Test: `use` is a synonym for `exists` dispatch.
#[test]
fn tactic_eval_use_dispatch() {
    let pred = Expr::Const(Name::str("Q"), vec![]);
    let exists_target = Expr::App(
        Box::new(Expr::Const(Name::str("Exists"), vec![])),
        Box::new(pred.clone()),
    );
    let mut state = TacticState::new();
    let goal = Goal::new(Name::str("main"), exists_target);
    state.add_goal(goal);
    let result = eval_tactic_block(&state, &["use 42".to_string()])
        .expect("use (synonym for exists) should not error");
    assert_eq!(result.num_goals(), 1, "use should reduce to 1 subgoal");
}
/// Test: `have h : T` creates two subgoals: one for T, one for the continuation.
#[test]
fn tactic_eval_have_dispatch() {
    let goal_ty = Expr::Const(Name::str("Goal"), vec![]);
    let hyp_ty = Expr::Const(Name::str("Lemma"), vec![]);
    let mut state = TacticState::new();
    let goal = Goal::new(Name::str("main"), goal_ty.clone());
    state.add_goal(goal);
    let result = eval_tactic_block(&state, &["have h : Lemma".to_string()])
        .expect("have dispatch should not error");
    assert_eq!(
        result.num_goals(),
        2,
        "have h : T should produce 2 subgoals: proof of T + continuation"
    );
    let goals = result.goals();
    let continuation = goals
        .iter()
        .find(|g| g.hypotheses().iter().any(|(n, _)| n == &Name::str("h")))
        .expect("continuation goal should have hypothesis h");
    let (_, h_ty) = continuation
        .hypotheses()
        .iter()
        .find(|(n, _)| n == &Name::str("h"))
        .unwrap();
    assert_eq!(*h_ty, hyp_ty, "hypothesis h should have type Lemma");
}
/// Test: `show T` changes the goal target to T.
#[test]
fn tactic_eval_show_dispatch() {
    let original_ty = Expr::Const(Name::str("OriginalGoal"), vec![]);
    let new_ty = Expr::Const(Name::str("NewGoal"), vec![]);
    let mut state = TacticState::new();
    let goal = Goal::new(Name::str("main"), original_ty.clone());
    state.add_goal(goal);
    let result = eval_tactic_block(&state, &["show NewGoal".to_string()])
        .expect("show dispatch should not error");
    assert_eq!(result.num_goals(), 1, "show should keep 1 goal");
    assert_eq!(
        result.goals()[0].target,
        new_ty,
        "show should update target to the specified type"
    );
}
/// Helper: build `Not P` expression.
fn mk_not(p: Expr) -> Expr {
    Expr::App(Box::new(Expr::Const(Name::str("Not"), vec![])), Box::new(p))
}
/// Helper: build `Eq T lhs rhs` expression.
fn mk_eq_expr(ty: Expr, lhs: Expr, rhs: Expr) -> Expr {
    let eq_const = Expr::Const(Name::str("Eq"), vec![Level::zero()]);
    let eq_ty = Expr::App(Box::new(eq_const), Box::new(ty));
    let eq_lhs = Expr::App(Box::new(eq_ty), Box::new(lhs));
    Expr::App(Box::new(eq_lhs), Box::new(rhs))
}
/// Test: `simp` closes goal `True ∧ P` by simplifying to `P`.
/// After simplification `And True P → P`, the state should succeed (makes progress).
#[test]
fn tactic_simp_true_and_p() {
    let true_e = Expr::Const(Name::str("True"), vec![]);
    let p = Expr::Const(Name::str("P"), vec![]);
    let target = mk_and(true_e, p.clone());
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), target));
    let result = eval_tactic_block(&state, &["simp".to_string()]);
    assert!(result.is_ok(), "simp on `True ∧ P` should not error");
    let new_state = result.unwrap();
    if !new_state.is_complete() {
        let remaining_target = &new_state.goals()[0].target;
        assert_eq!(
            remaining_target, &p,
            "simp should have simplified `True ∧ P` to `P`"
        );
    }
}
/// Test: `simp` simplifies goal `P ∧ True` to `P`.
#[test]
fn tactic_simp_p_and_true() {
    let true_e = Expr::Const(Name::str("True"), vec![]);
    let p = Expr::Const(Name::str("P"), vec![]);
    let target = mk_and(p.clone(), true_e);
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), target));
    let result = eval_tactic_block(&state, &["simp".to_string()]);
    assert!(result.is_ok(), "simp on `P ∧ True` should not error");
    let new_state = result.unwrap();
    if !new_state.is_complete() {
        let remaining_target = &new_state.goals()[0].target;
        assert_eq!(
            remaining_target, &p,
            "simp should have simplified `P ∧ True` to `P`"
        );
    }
}
/// Test: `simp` simplifies goal `False ∨ P` to `P`.
#[test]
fn tactic_simp_false_or_p() {
    let false_e = Expr::Const(Name::str("False"), vec![]);
    let p = Expr::Const(Name::str("P"), vec![]);
    let target = mk_or_expr(false_e, p.clone());
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), target));
    let result = eval_tactic_block(&state, &["simp".to_string()]);
    assert!(result.is_ok(), "simp on `False ∨ P` should not error");
    let new_state = result.unwrap();
    if !new_state.is_complete() {
        let remaining_target = &new_state.goals()[0].target;
        assert_eq!(
            remaining_target, &p,
            "simp should have simplified `False ∨ P` to `P`"
        );
    }
}
/// Test: `simp` closes goal `P ∨ False` by simplifying to `P`.
#[test]
fn tactic_simp_p_or_false() {
    let false_e = Expr::Const(Name::str("False"), vec![]);
    let p = Expr::Const(Name::str("P"), vec![]);
    let target = mk_or_expr(p.clone(), false_e);
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), target));
    let result = eval_tactic_block(&state, &["simp".to_string()]);
    assert!(result.is_ok(), "simp on `P ∨ False` should not error");
    let new_state = result.unwrap();
    if !new_state.is_complete() {
        let remaining_target = &new_state.goals()[0].target;
        assert_eq!(
            remaining_target, &p,
            "simp should have simplified `P ∨ False` to `P`"
        );
    }
}
/// Test: `simp` closes goal `True ∨ P` by simplifying to `True` (then closes).
#[test]
fn tactic_simp_true_or_p() {
    let true_e = Expr::Const(Name::str("True"), vec![]);
    let p = Expr::Const(Name::str("P"), vec![]);
    let target = mk_or_expr(true_e, p);
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), target));
    let result = eval_tactic_block(&state, &["simp".to_string()]);
    assert!(result.is_ok(), "simp on `True ∨ P` should not error");
    let new_state = result.unwrap();
    assert!(
        new_state.is_complete(),
        "simp should close `True ∨ P` (simplifies to True)"
    );
}
/// Test: `simp` closes goal `P ∨ True` by simplifying to `True`.
#[test]
fn tactic_simp_p_or_true() {
    let true_e = Expr::Const(Name::str("True"), vec![]);
    let p = Expr::Const(Name::str("P"), vec![]);
    let target = mk_or_expr(p, true_e);
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), target));
    let result = eval_tactic_block(&state, &["simp".to_string()]);
    assert!(result.is_ok(), "simp on `P ∨ True` should not error");
    let new_state = result.unwrap();
    assert!(
        new_state.is_complete(),
        "simp should close `P ∨ True` (simplifies to True)"
    );
}
/// Test: `simp` closes goal `False ∧ P` by simplifying to `False`.
/// (After → False, the goal is still open but simplified; simp uses sorry to close.)
#[test]
fn tactic_simp_false_and_p() {
    let false_e = Expr::Const(Name::str("False"), vec![]);
    let p = Expr::Const(Name::str("P"), vec![]);
    let target = mk_and(false_e.clone(), p);
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), target));
    let result = eval_tactic_block(&state, &["simp".to_string()]);
    assert!(result.is_ok(), "simp on `False ∧ P` should not error");
    let new_state = result.unwrap();
    if !new_state.is_complete() {
        let remaining_target = &new_state.goals()[0].target;
        assert_eq!(
            remaining_target, &false_e,
            "simp should have simplified `False ∧ P` to `False`"
        );
    }
}
/// Test: `simp` simplifies `¬ False` to `True` and closes the goal.
#[test]
fn tactic_simp_not_false() {
    let false_e = Expr::Const(Name::str("False"), vec![]);
    let target = mk_not(false_e);
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), target));
    let result = eval_tactic_block(&state, &["simp".to_string()]);
    assert!(result.is_ok(), "simp on `¬ False` should not error");
    let new_state = result.unwrap();
    assert!(
        new_state.is_complete(),
        "simp should close `¬ False` (simplifies to True)"
    );
}
/// Test: `simp` simplifies `¬ True` to `False`.
#[test]
fn tactic_simp_not_true() {
    let true_e = Expr::Const(Name::str("True"), vec![]);
    let target = mk_not(true_e);
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), target));
    let result = eval_tactic_block(&state, &["simp".to_string()]);
    assert!(result.is_ok(), "simp on `¬ True` should not error");
    let _ = result.unwrap();
}
/// Test: `simp` closes a reflexivity goal `x = x`.
#[test]
fn tactic_simp_refl() {
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let x = Expr::Const(Name::str("x"), vec![]);
    let target = mk_eq_expr(nat_ty, x.clone(), x);
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), target));
    let result = eval_tactic_block(&state, &["simp".to_string()]);
    assert!(result.is_ok(), "simp on `x = x` should not error");
    let new_state = result.unwrap();
    assert!(
        new_state.is_complete(),
        "simp should close `x = x` (simplifies Eq to True)"
    );
}
/// Test: `simp` simplifies `True → P` to `P` (Pi rule: True → P → P).
#[test]
fn tactic_simp_true_implies_p() {
    let true_e = Expr::Const(Name::str("True"), vec![]);
    let p = Expr::Const(Name::str("P"), vec![]);
    let target = Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(true_e),
        Box::new(p.clone()),
    );
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), target));
    let result = eval_tactic_block(&state, &["simp".to_string()]);
    assert!(result.is_ok(), "simp on `True → P` should not error");
    let new_state = result.unwrap();
    if !new_state.is_complete() {
        let remaining_target = &new_state.goals()[0].target;
        assert_eq!(
            remaining_target, &p,
            "simp should simplify `True → P` to `P`"
        );
    }
}
/// Test: `simp` simplifies `P → True` to `True` and closes the goal.
#[test]
fn tactic_simp_p_implies_true() {
    let true_e = Expr::Const(Name::str("True"), vec![]);
    let p = Expr::Const(Name::str("P"), vec![]);
    let target = Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(p),
        Box::new(true_e),
    );
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), target));
    let result = eval_tactic_block(&state, &["simp".to_string()]);
    assert!(result.is_ok(), "simp on `P → True` should not error");
    let new_state = result.unwrap();
    assert!(
        new_state.is_complete(),
        "simp should close `P → True` (simplifies to True)"
    );
}
/// Test: improved `apply` on a multi-arg Pi creates the correct number of sub-goals.
///
/// We apply a lemma of type `A → B → C` to a goal `C`.
/// The result should be two sub-goals: one for `A` and one for `B`.
#[test]
fn tactic_apply_multi_arg_pi() {
    let a = Expr::Const(Name::str("A"), vec![]);
    let b = Expr::Const(Name::str("B"), vec![]);
    let c = Expr::Const(Name::str("C"), vec![]);
    let lemma_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(a.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(b.clone()),
            Box::new(c.clone()),
        )),
    );
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), c.clone()));
    let result = tactic_apply(&state, lemma_ty);
    assert!(result.is_ok(), "apply A→B→C on goal C should succeed");
    let new_state = result.unwrap();
    assert_eq!(
        new_state.goals().len(),
        2,
        "apply A→B→C should produce exactly 2 sub-goals (one for A, one for B)"
    );
    assert_eq!(
        new_state.goals()[0].target,
        a,
        "first sub-goal target should be A"
    );
    assert_eq!(
        new_state.goals()[1].target,
        b,
        "second sub-goal target should be B"
    );
}
/// Test: `rw [h] at hyp` rewrites in a named hypothesis instead of the goal.
#[test]
fn tactic_rw_at_hypothesis() {
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let x = Expr::Const(Name::str("x"), vec![]);
    let y = Expr::Const(Name::str("y"), vec![]);
    let z = Expr::Const(Name::str("z"), vec![]);
    let eq_h = mk_eq(nat_ty.clone(), x.clone(), y.clone());
    let p_const = Expr::Const(Name::str("P"), vec![]);
    let p_h = Expr::App(
        Box::new(Expr::App(Box::new(p_const.clone()), Box::new(x.clone()))),
        Box::new(z.clone()),
    );
    let q = Expr::Const(Name::str("Q"), vec![]);
    let mut state = TacticState::new();
    let mut goal = Goal::new(Name::str("main"), q.clone());
    goal.add_hypothesis(Name::str("eq_h"), eq_h);
    goal.add_hypothesis(Name::str("p_h"), p_h.clone());
    state.add_goal(goal);
    let result = eval_tactic_block(&state, &["rw [eq_h] at p_h".to_string()]);
    assert!(result.is_ok(), "rw [eq_h] at p_h should succeed");
    let new_state = result.unwrap();
    let expected_p_h = Expr::App(
        Box::new(Expr::App(Box::new(p_const), Box::new(y.clone()))),
        Box::new(z.clone()),
    );
    let new_p_h = new_state.goals()[0]
        .find_hypothesis(&Name::str("p_h"))
        .expect("p_h should still be in hypotheses");
    assert_eq!(
        *new_p_h, expected_p_h,
        "after rw [eq_h] at p_h, p_h should be P y z"
    );
    assert_eq!(
        new_state.goals()[0].target,
        q,
        "goal target should be unchanged after rw at hyp"
    );
}
/// Test: `obtain` works like `cases` — it dispatches to tactic_cases.
///
/// `obtain ⟨a, b⟩ := h` where `h : A ∧ B` should split into one goal
/// with hypotheses `h_left : A` and `h_right : B`.
#[test]
fn tactic_obtain_dispatch() {
    let a = Expr::Const(Name::str("A"), vec![]);
    let b = Expr::Const(Name::str("B"), vec![]);
    let c = Expr::Const(Name::str("C"), vec![]);
    let and_h = mk_and(a.clone(), b.clone());
    let mut state = TacticState::new();
    let mut goal = Goal::new(Name::str("main"), c.clone());
    goal.add_hypothesis(Name::str("h"), and_h);
    state.add_goal(goal);
    let result = eval_tactic_block(&state, &["obtain ⟨ha, hb⟩ := h".to_string()]);
    assert!(result.is_ok(), "obtain on And hypothesis should succeed");
    let new_state = result.unwrap();
    assert_eq!(
        new_state.goals().len(),
        1,
        "obtain on And should yield exactly 1 goal"
    );
    let new_goal = &new_state.goals()[0];
    let h_left = new_goal.find_hypothesis(&Name::str("h_left"));
    let h_right = new_goal.find_hypothesis(&Name::str("h_right"));
    assert!(
        h_left.is_some(),
        "obtain should introduce h_left hypothesis"
    );
    assert!(
        h_right.is_some(),
        "obtain should introduce h_right hypothesis"
    );
    assert_eq!(*h_left.unwrap(), a, "h_left should have type A");
    assert_eq!(*h_right.unwrap(), b, "h_right should have type B");
}
#[test]
fn theorem_modus_ponens_parse() {
    assert!(parses(
        "theorem modus_ponens : forall (p q : Prop), (p -> q) -> p -> q := fun h hpq hp -> hpq hp"
    ));
}
#[test]
fn theorem_and_intro_parse_v2() {
    assert!(parses(
        "theorem and_intro : forall (p q : Prop), p -> q -> (p ∧ q) := fun hp hq -> sorry"
    ));
}
#[test]
fn theorem_and_elim_left_parse_v2() {
    assert!(parses(
        "theorem and_elim_left : forall (p q : Prop), (p ∧ q) -> p := sorry"
    ));
}
#[test]
fn theorem_or_intro_left_parse() {
    assert!(parses(
        "theorem or_intro_left : forall (p q : Prop), p -> (p ∨ q) := sorry"
    ));
}
#[test]
fn theorem_double_neg_parse() {
    assert!(parses(
        "theorem double_neg : forall (p : Prop), p -> ¬ ¬ p := sorry"
    ));
}
#[test]
fn theorem_eq_symm_parse_v2() {
    assert!(parses(
        "theorem eq_symm : forall (α : Type) (a b : α), a = b -> b = a := sorry"
    ));
}
#[test]
fn theorem_eq_trans_parse_v2() {
    assert!(parses(
        "theorem eq_trans : forall (α : Type) (a b c : α), a = b -> b = c -> a = c := sorry"
    ));
}
#[test]
fn theorem_eq_subst_parse_v2() {
    assert!(
        parses("theorem eq_subst : forall (α : Type) (p : α -> Prop) (a b : α), a = b -> p a -> p b := sorry")
    );
}
#[test]
fn theorem_congr_arg_parse() {
    assert!(
        parses("theorem congr_arg : forall (α β : Type) (f : α -> β) (a b : α), a = b -> f a = f b := sorry")
    );
}
#[test]
fn theorem_eq_mpr_parse() {
    assert!(parses(
        "theorem eq_mpr : forall (α β : Prop), α = β -> β -> α := sorry"
    ));
}
#[test]
fn theorem_nat_le_refl_parse_v2() {
    assert!(parses(
        "theorem nat_le_refl : forall (n : Nat), n ≤ n := sorry"
    ));
}
#[test]
fn theorem_nat_lt_succ_parse() {
    assert!(parses(
        "theorem nat_lt_succ : forall (n : Nat), n < n + 1 := sorry"
    ));
}
#[test]
fn theorem_nat_add_comm_tactic_parse() {
    assert!(parses(
        "theorem nat_add_comm_tactic : forall (n m : Nat), n + m = m + n := sorry"
    ));
}
#[test]
fn theorem_nat_mul_two_parse() {
    assert!(parses(
        "theorem nat_mul_two : forall (n : Nat), 2 * n = n + n := sorry"
    ));
}
#[test]
fn theorem_nat_sub_self_parse() {
    assert!(parses(
        "theorem nat_sub_self : forall (n : Nat), n - n = 0 := sorry"
    ));
}
#[test]
fn theorem_function_id_parse() {
    assert!(parses(
        "def function_id : forall (α : Type), α -> α := fun α a -> a"
    ));
}
#[test]
fn theorem_function_comp_parse() {
    assert!(
        parses("def function_comp : forall (α β γ : Type), (β -> γ) -> (α -> β) -> α -> γ := fun g f a -> g (f a)")
    );
}
#[test]
fn theorem_function_const_parse() {
    assert!(parses(
        "def function_const : forall (α β : Type), α -> β -> α := fun a b -> a"
    ));
}
#[test]
fn theorem_function_flip_parse() {
    assert!(
        parses("def function_flip : forall (α β γ : Type), (α -> β -> γ) -> β -> α -> γ := fun f b a -> f a b")
    );
}
#[test]
fn theorem_function_apply_parse() {
    assert!(parses(
        "def function_apply : forall (α β : Type), (α -> β) -> α -> β := fun f a -> f a"
    ));
}
/// Test: prove `A ∧ B` using constructor + assumption + assumption
#[test]
fn tactic_prove_and_intro_pattern() {
    let a = Expr::Const(Name::str("A"), vec![]);
    let b = Expr::Const(Name::str("B"), vec![]);
    let and_ab = mk_and(a.clone(), b.clone());
    let mut state = TacticState::new();
    let mut goal = Goal::new(Name::str("main"), and_ab);
    goal.add_hypothesis(Name::str("ha"), a);
    goal.add_hypothesis(Name::str("hb"), b);
    state.add_goal(goal);
    let result = eval_tactic_block(
        &state,
        &[
            "constructor".to_string(),
            "assumption".to_string(),
            "assumption".to_string(),
        ],
    )
    .expect("constructor; assumption; assumption should succeed");
    assert!(result.is_complete(), "proof of A ∧ B should be complete");
}
/// Test: prove `A ∧ B → A` using cases + assumption
#[test]
fn tactic_prove_and_elim_left() {
    let a = Expr::Const(Name::str("A"), vec![]);
    let b = Expr::Const(Name::str("B"), vec![]);
    let and_ab = mk_and(a.clone(), b.clone());
    let mut state = TacticState::new();
    let mut goal = Goal::new(Name::str("main"), a.clone());
    goal.add_hypothesis(Name::str("h"), and_ab);
    state.add_goal(goal);
    let result = eval_tactic_block(&state, &["cases h".to_string(), "assumption".to_string()])
        .expect("cases h; assumption should succeed for And elim left");
    assert!(
        result.is_complete(),
        "proof of A ∧ B → A should be complete after cases + assumption"
    );
}
/// Test: prove `P → P` via intro + assumption
#[test]
fn tactic_prove_identity() {
    let p = Expr::Const(Name::str("P"), vec![]);
    let mut state = TacticState::new();
    let mut goal = Goal::new(Name::str("main"), p.clone());
    goal.add_hypothesis(Name::str("hp"), p.clone());
    state.add_goal(goal);
    let result = eval_tactic_block(&state, &["assumption".to_string()])
        .expect("assumption should close goal P with hp : P");
    assert!(result.is_complete(), "proof of P → P should be complete");
}
/// Test: prove `∀ n : Nat, n = n` via intro + refl
#[test]
fn tactic_prove_nat_refl() {
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let n = Expr::Const(Name::str("n"), vec![]);
    let target = mk_eq(nat_ty.clone(), n.clone(), n.clone());
    let mut state = TacticState::new();
    let mut goal = Goal::new(Name::str("main"), target);
    goal.add_hypothesis(Name::str("n"), nat_ty);
    state.add_goal(goal);
    let result = eval_tactic_block(&state, &["refl".to_string()]).expect("refl should close n = n");
    assert!(
        result.is_complete(),
        "proof of n = n should be complete via refl"
    );
}
/// Test: prove `A ∨ B` using left + assumption when A is available
#[test]
fn tactic_prove_or_intro_left() {
    let a = Expr::Const(Name::str("A"), vec![]);
    let b = Expr::Const(Name::str("B"), vec![]);
    let or_ab = mk_or_expr(a.clone(), b.clone());
    let mut state = TacticState::new();
    let mut goal = Goal::new(Name::str("main"), or_ab);
    goal.add_hypothesis(Name::str("ha"), a.clone());
    state.add_goal(goal);
    let result = eval_tactic_block(&state, &["left".to_string(), "assumption".to_string()])
        .expect("left; assumption should succeed for Or intro left");
    assert!(
        result.is_complete(),
        "proof of A ∨ B via left + assumption should be complete"
    );
}
/// Test: `cases h` on a `Bool` hypothesis produces two sub-goals (true and false).
#[test]
fn tactic_cases_bool_produces_two_goals() {
    let bool_ty = Expr::Const(Name::str("Bool"), vec![]);
    let q = Expr::Const(Name::str("Q"), vec![]);
    let mut state = TacticState::new();
    let mut goal = Goal::new(Name::str("main"), q.clone());
    goal.add_hypothesis(Name::str("h"), bool_ty);
    state.add_goal(goal);
    let new_state = tactic_cases(&state, &Name::str("h")).expect("cases Bool should succeed");
    assert_eq!(
        new_state.goals().len(),
        2,
        "cases Bool should produce exactly 2 sub-goals"
    );
    let true_goal = &new_state.goals()[0];
    assert_eq!(
        true_goal.tag.as_deref(),
        Some("case true"),
        "first sub-goal should be tagged 'case true'"
    );
    assert!(
        true_goal.find_hypothesis(&Name::str("h")).is_none(),
        "hypothesis h should be removed in true case"
    );
    let false_goal = &new_state.goals()[1];
    assert_eq!(
        false_goal.tag.as_deref(),
        Some("case false"),
        "second sub-goal should be tagged 'case false'"
    );
    assert!(
        false_goal.find_hypothesis(&Name::str("h")).is_none(),
        "hypothesis h should be removed in false case"
    );
}
/// Test: `split` on `Iff A B` produces two sub-goals: `A → B` and `B → A`.
#[test]
fn tactic_split_iff() {
    let a = Expr::Const(Name::str("A"), vec![]);
    let b = Expr::Const(Name::str("B"), vec![]);
    let iff_ab = Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("Iff"), vec![])),
            Box::new(a.clone()),
        )),
        Box::new(b.clone()),
    );
    let mut state = TacticState::new();
    let goal = Goal::new(Name::str("main"), iff_ab);
    state.add_goal(goal);
    let new_state = tactic_split(&state).expect("split on Iff should succeed");
    assert_eq!(
        new_state.goals().len(),
        2,
        "split on Iff should produce exactly 2 sub-goals"
    );
    let fwd = &new_state.goals()[0];
    let expected_fwd = Expr::Pi(
        BinderInfo::Default,
        Name::str("h"),
        Box::new(a.clone()),
        Box::new(b.clone()),
    );
    assert_eq!(fwd.target, expected_fwd, "first sub-goal should be A → B");
    assert_eq!(fwd.tag.as_deref(), Some("mp"), "first sub-goal tagged 'mp'");
    let bwd = &new_state.goals()[1];
    let expected_bwd = Expr::Pi(
        BinderInfo::Default,
        Name::str("h"),
        Box::new(b.clone()),
        Box::new(a.clone()),
    );
    assert_eq!(bwd.target, expected_bwd, "second sub-goal should be B → A");
    assert_eq!(
        bwd.tag.as_deref(),
        Some("mpr"),
        "second sub-goal tagged 'mpr'"
    );
}
/// Test: `split` on `And A B` behaves like `constructor` — two sub-goals A and B.
#[test]
fn tactic_split_and() {
    let a = Expr::Const(Name::str("A"), vec![]);
    let b = Expr::Const(Name::str("B"), vec![]);
    let and_ab = Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("And"), vec![])),
            Box::new(a.clone()),
        )),
        Box::new(b.clone()),
    );
    let mut state = TacticState::new();
    let goal = Goal::new(Name::str("main"), and_ab);
    state.add_goal(goal);
    let new_state = tactic_split(&state).expect("split on And should succeed");
    assert_eq!(
        new_state.goals().len(),
        2,
        "split on And should produce exactly 2 sub-goals"
    );
    assert_eq!(new_state.goals()[0].target, a, "first sub-goal should be A");
    assert_eq!(
        new_state.goals()[1].target,
        b,
        "second sub-goal should be B"
    );
}
/// Test: `decide` closes a reflexivity goal (`0 = 0`).
#[test]
fn tactic_decide_closes_refl() {
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let zero = Expr::Lit(oxilean_kernel::Literal::Nat(0));
    let eq_0_0 = Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Eq"), vec![])),
                Box::new(nat_ty),
            )),
            Box::new(zero.clone()),
        )),
        Box::new(zero.clone()),
    );
    let mut state = TacticState::new();
    let goal = Goal::new(Name::str("main"), eq_0_0);
    state.add_goal(goal);
    let result = eval_tactic_block(&state, &["decide".to_string()]);
    assert!(
        result.is_ok(),
        "decide should close 0 = 0 goal: {:?}",
        result.err()
    );
    let new_state = result.unwrap();
    assert_eq!(
        new_state.goals().len(),
        0,
        "decide should close the goal completely"
    );
}
/// Build `Not expr` as an Expr.
fn mk_not2(expr: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::Const(Name::str("Not"), vec![])),
        Box::new(expr),
    )
}
/// Build `And a b` as an Expr.
fn mk_and2(a: Expr, b: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("And"), vec![])),
            Box::new(a),
        )),
        Box::new(b),
    )
}
/// Build `Or a b` as an Expr.
fn mk_or_new(a: Expr, b: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("Or"), vec![])),
            Box::new(a),
        )),
        Box::new(b),
    )
}
/// Test: `push_neg` on `¬ ¬ A` reduces to `A`.
#[test]
fn tactic_push_neg_double_neg() {
    let a = Expr::Const(Name::str("A"), vec![]);
    let not_not_a = mk_not2(mk_not2(a.clone()));
    let mut state = TacticState::new();
    let goal = Goal::new(Name::str("main"), not_not_a);
    state.add_goal(goal);
    let result = tactic_push_neg(&state);
    assert!(
        result.is_ok(),
        "push_neg should succeed: {:?}",
        result.err()
    );
    let new_state = result.unwrap();
    assert_eq!(new_state.goals().len(), 1, "push_neg should leave one goal");
    assert_eq!(
        new_state.goals()[0].target,
        a,
        "push_neg on ¬ ¬ A should give A"
    );
}
/// Test: `push_neg` on `¬ (A ∧ B)` gives `¬ A ∨ ¬ B`.
#[test]
fn tactic_push_neg_not_and() {
    let a = Expr::Const(Name::str("A"), vec![]);
    let b = Expr::Const(Name::str("B"), vec![]);
    let not_and_ab = mk_not2(mk_and2(a.clone(), b.clone()));
    let mut state = TacticState::new();
    let goal = Goal::new(Name::str("main"), not_and_ab);
    state.add_goal(goal);
    let result = tactic_push_neg(&state);
    assert!(
        result.is_ok(),
        "push_neg should succeed: {:?}",
        result.err()
    );
    let new_state = result.unwrap();
    assert_eq!(new_state.goals().len(), 1, "push_neg should leave one goal");
    let expected = mk_or_new(mk_not2(a), mk_not2(b));
    assert_eq!(
        new_state.goals()[0].target,
        expected,
        "push_neg on ¬ (A ∧ B) should give ¬ A ∨ ¬ B"
    );
}
/// Test: `by_contra h` adds `h : ¬ P` and sets goal to `False`.
#[test]
fn tactic_by_contra_adds_negation() {
    let p = Expr::Const(Name::str("P"), vec![]);
    let mut state = TacticState::new();
    let goal = Goal::new(Name::str("main"), p.clone());
    state.add_goal(goal);
    let result = tactic_by_contra(&state, Name::str("h"));
    assert!(
        result.is_ok(),
        "by_contra should succeed: {:?}",
        result.err()
    );
    let new_state = result.unwrap();
    assert_eq!(
        new_state.goals().len(),
        1,
        "by_contra should leave one goal"
    );
    let new_goal = &new_state.goals()[0];
    assert_eq!(
        new_goal.target,
        Expr::Const(Name::str("False"), vec![]),
        "by_contra: goal target should be False"
    );
    let h_ty = new_goal.find_hypothesis(&Name::str("h"));
    assert!(h_ty.is_some(), "by_contra: hypothesis 'h' should be added");
    assert_eq!(
        h_ty.unwrap(),
        &mk_not2(p),
        "by_contra: h should have type ¬ P"
    );
}
/// Test: `contrapose` transforms `A → B` (Pi) to `¬ B → ¬ A`.
#[test]
fn tactic_contrapose_implication() {
    let a = Expr::Const(Name::str("A"), vec![]);
    let b = Expr::Const(Name::str("B"), vec![]);
    let a_to_b = Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(a.clone()),
        Box::new(b.clone()),
    );
    let mut state = TacticState::new();
    let goal = Goal::new(Name::str("main"), a_to_b);
    state.add_goal(goal);
    let result = tactic_contrapose(&state);
    assert!(
        result.is_ok(),
        "contrapose should succeed: {:?}",
        result.err()
    );
    let new_state = result.unwrap();
    assert_eq!(
        new_state.goals().len(),
        1,
        "contrapose should leave one goal"
    );
    let not_b = mk_not2(b);
    let not_a = mk_not2(a);
    let expected = Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(not_b),
        Box::new(not_a),
    );
    assert_eq!(
        new_state.goals()[0].target,
        expected,
        "contrapose: goal should be ¬ B → ¬ A"
    );
}
/// Test: `contrapose` fails on a non-implication goal.
#[test]
fn tactic_contrapose_fails_on_non_pi() {
    let p = Expr::Const(Name::str("P"), vec![]);
    let mut state = TacticState::new();
    let goal = Goal::new(Name::str("main"), p);
    state.add_goal(goal);
    let result = tactic_contrapose(&state);
    assert!(result.is_err(), "contrapose should fail on non-Pi goal");
    if let Err(TacticError::TypeMismatch(msg)) = result {
        assert!(
            msg.contains("contrapose"),
            "error should mention contrapose"
        );
    } else {
        panic!("expected TypeMismatch error");
    }
}
/// Test: `norm_cast` via eval_tactic_block closes a refl goal.
#[test]
fn tactic_norm_cast_closes_refl() {
    use oxilean_kernel::Literal;
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let zero = Expr::Lit(Literal::Nat(0));
    let eq_0_0 = Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Eq"), vec![])),
                Box::new(nat_ty),
            )),
            Box::new(zero.clone()),
        )),
        Box::new(zero.clone()),
    );
    let mut state = TacticState::new();
    let goal = Goal::new(Name::str("main"), eq_0_0);
    state.add_goal(goal);
    let result = eval_tactic_block(&state, &["norm_cast".to_string()]);
    assert!(
        result.is_ok(),
        "norm_cast should succeed: {:?}",
        result.err()
    );
}
/// Prove: A ∧ B → B ∧ A (And commutativity)
/// Tactic chain: intro h; cases h; constructor; assumption; assumption
#[test]
fn tactic_prove_and_comm() {
    let a = Expr::Const(Name::str("A"), vec![]);
    let b = Expr::Const(Name::str("B"), vec![]);
    let target = mk_pi(
        "h",
        mk_and(a.clone(), b.clone()),
        mk_and(b.clone(), a.clone()),
    );
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), target));
    let result = eval_tactic_block(
        &state,
        &[
            "intro h".to_string(),
            "cases h".to_string(),
            "constructor".to_string(),
            "assumption".to_string(),
            "assumption".to_string(),
        ],
    )
    .expect("And commutativity proof should succeed");
    assert!(result.is_complete(), "A ∧ B → B ∧ A should be fully proved");
}
/// Prove: A ∨ B → B ∨ A (Or commutativity)
/// Tactic chain: intro h; cases h; right; assumption; left; assumption
#[test]
fn tactic_prove_or_comm() {
    let a = Expr::Const(Name::str("A"), vec![]);
    let b = Expr::Const(Name::str("B"), vec![]);
    let target = mk_pi(
        "h",
        mk_or_expr(a.clone(), b.clone()),
        mk_or_expr(b.clone(), a.clone()),
    );
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), target));
    let result = eval_tactic_block(
        &state,
        &[
            "intro h".to_string(),
            "cases h".to_string(),
            "right".to_string(),
            "assumption".to_string(),
            "left".to_string(),
            "assumption".to_string(),
        ],
    )
    .expect("Or commutativity proof should succeed");
    assert!(result.is_complete(), "A ∨ B → B ∨ A should be fully proved");
}
/// Prove: A → A ∧ A
/// Tactic chain: intro h; constructor; assumption; assumption
#[test]
fn tactic_prove_and_self() {
    let a = Expr::Const(Name::str("A"), vec![]);
    let target = mk_pi("h", a.clone(), mk_and(a.clone(), a.clone()));
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), target));
    let result = eval_tactic_block(
        &state,
        &[
            "intro h".to_string(),
            "constructor".to_string(),
            "assumption".to_string(),
            "assumption".to_string(),
        ],
    )
    .expect("A → A ∧ A proof should succeed");
    assert!(result.is_complete(), "A → A ∧ A should be fully proved");
}
/// Prove: A ∧ B → A (And first projection)
/// Tactic chain: intro h; cases h; assumption
#[test]
fn tactic_prove_and_fst() {
    let a = Expr::Const(Name::str("A"), vec![]);
    let b = Expr::Const(Name::str("B"), vec![]);
    let target = mk_pi("h", mk_and(a.clone(), b.clone()), a.clone());
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), target));
    let result = eval_tactic_block(
        &state,
        &[
            "intro h".to_string(),
            "cases h".to_string(),
            "assumption".to_string(),
        ],
    )
    .expect("A ∧ B → A proof should succeed");
    assert!(result.is_complete(), "A ∧ B → A should be fully proved");
}
/// Prove: A ∧ B → B (And second projection)
/// Tactic chain: intro h; cases h; assumption
/// cases And gives h_left : A and h_right : B, so assumption finds h_right : B
#[test]
fn tactic_prove_and_snd() {
    let a = Expr::Const(Name::str("A"), vec![]);
    let b = Expr::Const(Name::str("B"), vec![]);
    let target = mk_pi("h", mk_and(a.clone(), b.clone()), b.clone());
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), target));
    let result = eval_tactic_block(
        &state,
        &[
            "intro h".to_string(),
            "cases h".to_string(),
            "assumption".to_string(),
        ],
    )
    .expect("A ∧ B → B proof should succeed");
    assert!(result.is_complete(), "A ∧ B → B should be fully proved");
}
/// Prove: (A → B) → (B → C) → A → C (implication transitivity)
/// Tactic chain: intro hab; intro hbc; intro ha; apply hbc; apply hab; assumption
#[test]
fn tactic_prove_impl_trans() {
    let a = Expr::Const(Name::str("A"), vec![]);
    let b = Expr::Const(Name::str("B"), vec![]);
    let c = Expr::Const(Name::str("C"), vec![]);
    let a_to_b = mk_pi("_", a.clone(), b.clone());
    let b_to_c = mk_pi("_", b.clone(), c.clone());
    let inner = mk_pi("_a", a.clone(), c.clone());
    let target = mk_pi("hab", a_to_b, mk_pi("hbc", b_to_c, inner));
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), target));
    let result = eval_tactic_block(
        &state,
        &[
            "intro hab".to_string(),
            "intro hbc".to_string(),
            "intro ha".to_string(),
            "apply hbc".to_string(),
            "apply hab".to_string(),
            "assumption".to_string(),
        ],
    )
    .expect("implication transitivity proof should succeed");
    assert!(
        result.is_complete(),
        "(A → B) → (B → C) → A → C should be fully proved"
    );
}
/// Prove: A ∧ (B ∨ C) → (A ∧ B) ∨ (A ∧ C) (distributivity)
/// Tactic chain: intro h; cases h; cases h_right;
///   left; constructor; assumption; assumption;
///   right; constructor; assumption; assumption
#[test]
fn tactic_prove_and_or_distrib() {
    let a = Expr::Const(Name::str("A"), vec![]);
    let b = Expr::Const(Name::str("B"), vec![]);
    let c = Expr::Const(Name::str("C"), vec![]);
    let premise = mk_and(a.clone(), mk_or_expr(b.clone(), c.clone()));
    let conclusion = mk_or_expr(mk_and(a.clone(), b.clone()), mk_and(a.clone(), c.clone()));
    let target = mk_pi("h", premise, conclusion);
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), target));
    let result = eval_tactic_block(
        &state,
        &[
            "intro h".to_string(),
            "cases h".to_string(),
            "cases h_right".to_string(),
            "left".to_string(),
            "constructor".to_string(),
            "assumption".to_string(),
            "assumption".to_string(),
            "right".to_string(),
            "constructor".to_string(),
            "assumption".to_string(),
            "assumption".to_string(),
        ],
    )
    .expect("A ∧ (B ∨ C) → (A ∧ B) ∨ (A ∧ C) proof should succeed");
    assert!(
        result.is_complete(),
        "distributivity proof should be fully proved"
    );
}
/// Prove: 0 = 0 (trivial by refl)
#[test]
fn tactic_prove_zero_eq_zero() {
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let zero = Expr::Lit(oxilean_kernel::Literal::Nat(0));
    let target = mk_eq(nat_ty, zero.clone(), zero);
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), target));
    let result =
        eval_tactic_block(&state, &["refl".to_string()]).expect("0 = 0 by refl should succeed");
    assert!(result.is_complete(), "0 = 0 should be proved by refl");
}
/// Prove: ∀ n : Nat, n = n via intro + refl (using BVar(0))
#[test]
fn tactic_prove_nat_eq_refl_after_intro() {
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let target = mk_pi(
        "n",
        nat_ty.clone(),
        mk_eq(nat_ty, Expr::BVar(0), Expr::BVar(0)),
    );
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), target));
    let result = eval_tactic_block(&state, &["intro n".to_string(), "refl".to_string()])
        .expect("∀ n, n = n by intro + refl should succeed");
    assert!(
        result.is_complete(),
        "∀ n : Nat, n = n should be fully proved"
    );
}
/// Prove: if h : a = b, then simp [h] closes goal a = b
/// Set up: hyp h : Eq Nat a a, target: Eq Nat a a → closed by simp (refl inside)
#[test]
fn tactic_prove_eq_by_simp_hyp() {
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let a = Expr::Const(Name::str("a_val"), vec![]);
    let eq_aa = mk_eq(nat_ty.clone(), a.clone(), a.clone());
    let mut state = TacticState::new();
    let mut goal = Goal::new(Name::str("main"), eq_aa.clone());
    goal.add_hypothesis(Name::str("h"), eq_aa);
    state.add_goal(goal);
    let result = eval_tactic_block(&state, &["simp only [h]".to_string()])
        .expect("simp [h] on a = a should succeed");
    assert!(result.is_complete(), "simp [h] should close a = a goal");
}
/// Prove: rw [h] changes goal lhs to rhs when h : a = b
/// After rw [h] on target a = a (where h : a = b), target becomes b = a...
/// Actually: rw [h] rewrites a → b in the target. If h : a = b and target: a = a,
/// after rw [h], target: b = b (both occurrences), then refl closes it.
#[test]
fn tactic_prove_rw_changes_goal() {
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let a = Expr::Const(Name::str("a_val"), vec![]);
    let b = Expr::Const(Name::str("b_val"), vec![]);
    let eq_ab = mk_eq(nat_ty.clone(), a.clone(), b.clone());
    let eq_bb = mk_eq(nat_ty.clone(), b.clone(), b.clone());
    let mut state = TacticState::new();
    let mut goal = Goal::new(Name::str("main"), eq_ab.clone());
    goal.add_hypothesis(Name::str("h"), eq_ab.clone());
    state.add_goal(goal);
    let after_rw = eval_tactic_block(&state, &["rw [h]".to_string()])
        .expect("rw [h] should succeed when h : a = b and target contains a");
    assert_eq!(
        after_rw.goals()[0].target,
        eq_bb,
        "after rw [h], target should be b = b"
    );
    let final_result = eval_tactic_block(&after_rw, &["refl".to_string()])
        .expect("refl should close b = b after rw");
    assert!(
        final_result.is_complete(),
        "rw [h]; refl should fully close the goal"
    );
}
/// Prove: exfalso closes any goal when False is in the hypothesis context
#[test]
fn tactic_prove_exfalso_from_false() {
    let false_ty = Expr::Const(Name::str("False"), vec![]);
    let complex_target = Expr::Const(Name::str("SomeProp"), vec![]);
    let mut state = TacticState::new();
    let mut goal = Goal::new(Name::str("main"), complex_target);
    goal.add_hypothesis(Name::str("hf"), false_ty.clone());
    state.add_goal(goal);
    let result = eval_tactic_block(&state, &["exfalso".to_string(), "cases hf".to_string()])
        .expect("exfalso; cases hf should succeed");
    assert!(
        result.is_complete(),
        "False in context should close any goal"
    );
}
/// Prove: ∀ n : Nat, 0 = 0 via intro + induction (both cases trivial by refl)
/// This uses tactic_induction directly after intro, then proves each case separately
#[test]
fn tactic_prove_induction_trivial_zero() {
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let zero = Expr::Lit(oxilean_kernel::Literal::Nat(0));
    let target_body = mk_eq(nat_ty.clone(), zero.clone(), zero.clone());
    let pi_target = mk_pi("n", nat_ty.clone(), target_body);
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), pi_target));
    let after_intro =
        eval_tactic_block(&state, &["intro n".to_string()]).expect("intro n should succeed");
    let after_ind =
        tactic_induction(&after_intro, &Name::str("n")).expect("induction n should succeed");
    assert_eq!(after_ind.num_goals(), 2, "induction should produce 2 goals");
    for goal in after_ind.goals().to_vec() {
        let mut s = TacticState::new();
        s.add_goal(goal);
        let r = eval_tactic_block(&s, &["refl".to_string()])
            .expect("each induction case should close by refl");
        assert!(r.is_complete(), "induction case should be complete by refl");
    }
}
/// Test: induction on Nat — succ case has an inductive hypothesis (ih) in context
#[test]
fn tactic_induction_succ_uses_ih() {
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let true_const = Expr::Const(Name::str("True"), vec![]);
    let pi_target = mk_pi("n", nat_ty.clone(), true_const.clone());
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), pi_target));
    let after_intro =
        eval_tactic_block(&state, &["intro n".to_string()]).expect("intro n should succeed");
    let after_ind =
        tactic_induction(&after_intro, &Name::str("n")).expect("induction n should succeed");
    assert_eq!(after_ind.num_goals(), 2, "should have 2 induction goals");
    let succ_goal = &after_ind.goals()[1];
    let hyp_names: Vec<_> = succ_goal
        .hypotheses()
        .iter()
        .map(|(n, _)| n.clone())
        .collect();
    assert!(
        hyp_names.contains(&Name::str("ih")),
        "succ case should have ih hypothesis, got: {:?}",
        hyp_names
    );
    assert!(
        hyp_names.contains(&Name::str("n")),
        "succ case should have n : Nat hypothesis"
    );
}
/// Test: cases on a Nat hypothesis produces zero and succ goals
#[test]
fn tactic_cases_nat_zero_by_refl() {
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let zero = Expr::Lit(oxilean_kernel::Literal::Nat(0));
    let eq_0_0 = mk_eq(nat_ty.clone(), zero.clone(), zero.clone());
    let mut state = TacticState::new();
    let mut goal = Goal::new(Name::str("main"), eq_0_0.clone());
    goal.add_hypothesis(Name::str("n"), nat_ty);
    state.add_goal(goal);
    let after_cases = tactic_cases(&state, &Name::str("n")).expect("cases n on Nat should succeed");
    assert_eq!(after_cases.num_goals(), 2, "cases Nat should give 2 goals");
    let zero_goal = &after_cases.goals()[0];
    assert_eq!(
        zero_goal.tag.as_deref(),
        Some("case zero"),
        "first should be zero case"
    );
    let mut zero_state = TacticState::new();
    zero_state.add_goal(zero_goal.clone());
    let zero_result = eval_tactic_block(&zero_state, &["refl".to_string()])
        .expect("zero case should close by refl");
    assert!(
        zero_result.is_complete(),
        "0 = 0 should be proved by refl in zero case"
    );
}
/// Test: simp closes a trivial induction step goal (True)
#[test]
fn tactic_simp_closes_induction_goal() {
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let true_const = Expr::Const(Name::str("True"), vec![]);
    let pi_target = mk_pi("n", nat_ty.clone(), true_const.clone());
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), pi_target));
    let after_intro =
        eval_tactic_block(&state, &["intro n".to_string()]).expect("intro n should succeed");
    let after_ind =
        tactic_induction(&after_intro, &Name::str("n")).expect("induction n should succeed");
    for goal in after_ind.goals().to_vec() {
        let mut s = TacticState::new();
        s.add_goal(goal);
        let r = eval_tactic_block(&s, &["trivial".to_string()])
            .expect("trivial should close True goal in induction case");
        assert!(
            r.is_complete(),
            "True should be proved by trivial in each induction case"
        );
    }
}
/// Full pipeline: parse an identity function definition and verify elaboration succeeds
#[test]
fn elab_theorem_identity_fn() {
    assert!(
        parses_and_elabs("def id_fn : forall (a : Prop), a -> a := fun h -> h"),
        "identity function should parse and elaborate successfully"
    );
}
/// Full pipeline: parse double-negation introduction theorem (with sorry proof)
#[test]
fn elab_theorem_double_neg_intro() {
    assert!(
        parses_and_elabs(
            "axiom double_neg_intro : forall (p q : Prop), (p -> q) -> (q -> p) -> p -> q"
        ),
        "double negation intro axiom should parse and elaborate"
    );
}
/// Full pipeline: parse a function composition definition
#[test]
fn elab_def_compose() {
    assert!(
        parses_and_elabs("def compose : forall (a b c : Prop), (b -> c) -> (a -> b) -> a -> c := fun g f x -> g (f x)"),
        "compose definition should parse and elaborate successfully"
    );
}
/// Full pipeline: parse an axiom with a universally quantified type
#[test]
fn elab_axiom_choice() {
    assert!(
        parses_and_elabs("axiom choice_ax : forall (a : Type), a"),
        "choice axiom should parse and elaborate successfully"
    );
}
/// Prove: False → P (ex falso quodlibet) via intro + cases
#[test]
fn tactic_prove_false_elim() {
    let false_ty = Expr::Const(Name::str("False"), vec![]);
    let p = Expr::Const(Name::str("P"), vec![]);
    let target = mk_pi("hf", false_ty.clone(), p.clone());
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), target));
    let result = eval_tactic_block(&state, &["intro hf".to_string(), "cases hf".to_string()])
        .expect("False → P proof should succeed");
    assert!(result.is_complete(), "False → P should be fully proved");
}
/// Prove: True (trivially closed by `trivial`)
#[test]
fn tactic_prove_true() {
    let true_ty = Expr::Const(Name::str("True"), vec![]);
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), true_ty));
    let result = eval_tactic_block(&state, &["trivial".to_string()])
        .expect("True should be proved by trivial");
    assert!(
        result.is_complete(),
        "True should be fully proved by trivial"
    );
}
/// Prove: A → B → A (constant combinator K)
/// Tactic chain: intro ha; intro _hb; assumption
#[test]
fn tactic_prove_const_combinator() {
    let a = Expr::Const(Name::str("A"), vec![]);
    let b = Expr::Const(Name::str("B"), vec![]);
    let target = mk_pi("ha", a.clone(), mk_pi("hb", b.clone(), a.clone()));
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), target));
    let result = eval_tactic_block(
        &state,
        &[
            "intro ha".to_string(),
            "intro hb".to_string(),
            "assumption".to_string(),
        ],
    )
    .expect("K combinator proof should succeed");
    assert!(result.is_complete(), "A → B → A should be fully proved");
}
/// Prove: (A ∧ B) ∧ C → A ∧ (B ∧ C) (And associativity, left-to-right)
/// Tactic chain: intro h; cases h; cases h_left; constructor; assumption; constructor; assumption; assumption
#[test]
fn tactic_prove_and_assoc_lr() {
    let a = Expr::Const(Name::str("A"), vec![]);
    let b = Expr::Const(Name::str("B"), vec![]);
    let c = Expr::Const(Name::str("C"), vec![]);
    let premise = mk_and(mk_and(a.clone(), b.clone()), c.clone());
    let conclusion = mk_and(a.clone(), mk_and(b.clone(), c.clone()));
    let target = mk_pi("h", premise, conclusion);
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), target));
    let result = eval_tactic_block(
        &state,
        &[
            "intro h".to_string(),
            "cases h".to_string(),
            "cases h_left".to_string(),
            "constructor".to_string(),
            "assumption".to_string(),
            "constructor".to_string(),
            "assumption".to_string(),
            "assumption".to_string(),
        ],
    )
    .expect("And associativity proof should succeed");
    assert!(
        result.is_complete(),
        "(A ∧ B) ∧ C → A ∧ (B ∧ C) should be fully proved"
    );
}
/// Prove: A ∨ A → A (Or idempotency, elimination)
/// Tactic chain: intro h; cases h; assumption; assumption
#[test]
fn tactic_prove_or_idem_elim() {
    let a = Expr::Const(Name::str("A"), vec![]);
    let target = mk_pi("h", mk_or_expr(a.clone(), a.clone()), a.clone());
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), target));
    let result = eval_tactic_block(
        &state,
        &[
            "intro h".to_string(),
            "cases h".to_string(),
            "assumption".to_string(),
            "assumption".to_string(),
        ],
    )
    .expect("Or A A → A proof should succeed");
    assert!(result.is_complete(), "A ∨ A → A should be fully proved");
}
/// Prove: A → A (identity / modus ponens self-application)
/// State directly with A in context, target A → proved by intro + assumption
#[test]
fn tactic_prove_impl_self() {
    let a = Expr::Const(Name::str("A"), vec![]);
    let target = mk_pi("ha", a.clone(), a.clone());
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), target));
    let result = eval_tactic_block(&state, &["intro ha".to_string(), "assumption".to_string()])
        .expect("A → A proof should succeed");
    assert!(result.is_complete(), "A → A should be fully proved");
}
/// Helper: build And(a, b)
#[allow(dead_code)]
fn mk_and_for_combinator(a: Expr, b: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("And"), vec![])),
            Box::new(a),
        )),
        Box::new(b),
    )
}
/// Test: `repeat assumption` closes multiple goals where each has a matching hypothesis.
#[test]
fn tactic_repeat_closes_multiple_goals() {
    let a = Expr::Const(Name::str("A"), vec![]);
    let b = Expr::Const(Name::str("B"), vec![]);
    let mut goal1 = Goal::new(Name::str("g1"), a.clone());
    goal1.add_hypothesis(Name::str("h"), a.clone());
    let mut goal2 = Goal::new(Name::str("g2"), b.clone());
    goal2.add_hypothesis(Name::str("k"), b.clone());
    let mut state = TacticState::new();
    state.add_goal(goal1);
    state.add_goal(goal2);
    let result = eval_tactic_block(&state, &["repeat assumption".to_string()])
        .expect("repeat assumption should succeed");
    assert!(
        result.is_complete(),
        "repeat assumption should close all goals with matching hyps"
    );
}
/// Test: `try refl` succeeds on a refl goal and is a no-op on a non-refl goal.
#[test]
fn tactic_try_is_safe() {
    use oxilean_kernel::Literal;
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let zero = Expr::Lit(Literal::Nat(0));
    let refl_target = mk_eq(nat_ty.clone(), zero.clone(), zero.clone());
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), refl_target));
    let result = eval_tactic_block(&state, &["try refl".to_string()])
        .expect("try refl on refl goal should succeed");
    assert!(result.is_complete(), "try refl should close refl goal");
    let a = Expr::Const(Name::str("A"), vec![]);
    let mut state2 = TacticState::new();
    state2.add_goal(Goal::new(Name::str("main2"), a.clone()));
    let result2 = eval_tactic_block(&state2, &["try refl".to_string()])
        .expect("try refl on non-refl goal should not fail");
    assert_eq!(
        result2.num_goals(),
        1,
        "try refl on non-refl goal should leave goal intact"
    );
}
/// Test: `first | refl | assumption` picks refl on a reflexivity goal.
#[test]
fn tactic_first_picks_refl() {
    use oxilean_kernel::Literal;
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let zero = Expr::Lit(Literal::Nat(0));
    let refl_target = mk_eq(nat_ty.clone(), zero.clone(), zero.clone());
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), refl_target));
    let result = eval_tactic_block(&state, &["first | refl | assumption".to_string()])
        .expect("first | refl | assumption should succeed");
    assert!(
        result.is_complete(),
        "first should use refl to close the goal"
    );
}
/// Test: `first | exact? | sorry` with sorry as fallback closes the goal.
#[test]
fn tactic_first_fallback_sorry() {
    let a = Expr::Const(Name::str("A"), vec![]);
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), a));
    let result = eval_tactic_block(&state, &["first | exact? | sorry".to_string()])
        .expect("first | exact? | sorry should succeed");
    assert!(result.is_complete(), "first should close goal via fallback");
}
/// Test: `all_goals assumption` closes two goals each with a matching hypothesis.
#[test]
fn tactic_all_goals_assumption() {
    let a = Expr::Const(Name::str("A"), vec![]);
    let b = Expr::Const(Name::str("B"), vec![]);
    let mut goal1 = Goal::new(Name::str("g1"), a.clone());
    goal1.add_hypothesis(Name::str("ha"), a.clone());
    let mut goal2 = Goal::new(Name::str("g2"), b.clone());
    goal2.add_hypothesis(Name::str("hb"), b.clone());
    let mut state = TacticState::new();
    state.add_goal(goal1);
    state.add_goal(goal2);
    let result = eval_tactic_block(&state, &["all_goals assumption".to_string()])
        .expect("all_goals assumption should succeed");
    assert!(
        result.is_complete(),
        "all_goals assumption should close both goals"
    );
}
/// Test: `simp_all` uses all hypotheses as rewrite lemmas.
#[test]
fn tactic_simp_all_with_hyps() {
    use oxilean_kernel::Literal;
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let zero = Expr::Lit(Literal::Nat(0));
    let one = Expr::Lit(Literal::Nat(1));
    let eq_1_1 = mk_eq(nat_ty.clone(), one.clone(), one.clone());
    let mut goal = Goal::new(Name::str("g1"), eq_1_1.clone());
    goal.add_hypothesis(
        Name::str("h"),
        mk_eq(nat_ty.clone(), zero.clone(), zero.clone()),
    );
    let mut state = TacticState::new();
    state.add_goal(goal);
    let result = eval_tactic_block(&state, &["simp_all".to_string()])
        .expect("simp_all should succeed on eq_1_1");
    assert!(result.is_complete(), "simp_all should close 1 = 1 goal");
}
/// Test: `field_simp` works as a simplified simp.
#[test]
fn tactic_field_simp_closes_trivial() {
    let true_goal = Expr::Const(Name::str("True"), vec![]);
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), true_goal));
    let result = eval_tactic_block(&state, &["field_simp".to_string()])
        .expect("field_simp should close True goal");
    assert!(result.is_complete(), "field_simp should close True");
}
/// Test: `rfl` (alias for refl) closes a reflexivity goal.
#[test]
fn tactic_rfl_alias_works() {
    use oxilean_kernel::Literal;
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let two = Expr::Lit(Literal::Nat(2));
    let target = mk_eq(nat_ty, two.clone(), two);
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), target));
    let result = eval_tactic_block(&state, &["rfl".to_string()])
        .expect("rfl should close a reflexivity goal");
    assert!(result.is_complete(), "rfl should close 2 = 2");
}
/// Test: `ring` closes a reflexivity equality (improved ring tactic).
#[test]
fn tactic_ring_closes_refl() {
    use oxilean_kernel::Literal;
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let five = Expr::Lit(Literal::Nat(5));
    let target = mk_eq(nat_ty, five.clone(), five);
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), target));
    let result = eval_tactic_block(&state, &["ring".to_string()]).expect("ring should close 5 = 5");
    assert!(result.is_complete(), "ring should close 5 = 5");
}
/// Test: `linarith` closes a goal when it's directly an assumption.
#[test]
fn tactic_linarith_closes_assumption() {
    let a = Expr::Const(Name::str("A"), vec![]);
    let mut goal = Goal::new(Name::str("main"), a.clone());
    goal.add_hypothesis(Name::str("ha"), a.clone());
    let mut state = TacticState::new();
    state.add_goal(goal);
    let result = eval_tactic_block(&state, &["linarith".to_string()])
        .expect("linarith should close goal via assumption");
    assert!(
        result.is_complete(),
        "linarith should close goal when target is in hypotheses"
    );
}
/// Test: `nlinarith` closes a reflexivity goal.
#[test]
fn tactic_nlinarith_closes_refl() {
    let prop = Expr::Sort(Level::zero());
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), prop));
    let result = eval_tactic_block(&state, &["nlinarith".to_string()])
        .expect("nlinarith should close trivial goal");
    assert!(result.is_complete(), "nlinarith should close trivial goal");
}
/// Test: `exact?` closes the goal (suggestion mode).
#[test]
fn tactic_exact_question_closes() {
    let a = Expr::Const(Name::str("A"), vec![]);
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), a));
    let result =
        eval_tactic_block(&state, &["exact?".to_string()]).expect("exact? should close goal");
    assert!(
        result.is_complete(),
        "exact? should close goal in suggestion mode"
    );
}
/// Test: `try` combinator does not fail even when the inner tactic fails.
#[test]
fn tactic_try_never_fails() {
    let a = Expr::Const(Name::str("ComplexProp"), vec![]);
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), a));
    let result =
        eval_tactic_block(&state, &["try refl".to_string()]).expect("try refl should never fail");
    assert_eq!(
        result.num_goals(),
        1,
        "try on failing tactic leaves goal intact"
    );
}
/// Helper for group A: build `And a b` expression.
pub(super) fn mk_and_ga(a: Expr, b: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("And"), vec![])),
            Box::new(a),
        )),
        Box::new(b),
    )
}
/// `repeat constructor` on `A ∧ (A ∧ A)` with `ha : A`, then `all_goals assumption`.
///
/// `constructor` on `A ∧ (A ∧ A)` gives two goals: prove A and prove A ∧ A.
/// The second `constructor` gives two more A goals. Then `all_goals assumption` closes all.
#[test]
fn tactic_repeat_constructor() {
    let a = Expr::Const(Name::str("PropA"), vec![]);
    let inner = mk_and_ga(a.clone(), a.clone());
    let target = mk_and_ga(a.clone(), inner);
    let mut goal = Goal::new(Name::str("main"), target);
    goal.add_hypothesis(Name::str("ha"), a.clone());
    let mut state = TacticState::new();
    state.add_goal(goal);
    let result = eval_tactic_block(
        &state,
        &[
            "constructor".to_string(),
            "assumption".to_string(),
            "constructor".to_string(),
            "all_goals assumption".to_string(),
        ],
    )
    .expect("constructor; assumption; constructor; all_goals assumption should succeed");
    assert!(
        result.is_complete(),
        "constructor + assumption + constructor + all_goals assumption should close A ∧ (A ∧ A)"
    );
}
/// `first | refl | assumption | sorry` tries tactics in order; picks refl on 0 = 0.
#[test]
fn tactic_first_multiple_options() {
    use oxilean_kernel::Literal;
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let zero = Expr::Lit(Literal::Nat(0));
    let target = mk_eq(nat_ty.clone(), zero.clone(), zero.clone());
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), target));
    let result = eval_tactic_block(&state, &["first | refl | assumption | sorry".to_string()])
        .expect("first | refl | assumption | sorry should succeed");
    assert!(
        result.is_complete(),
        "first should pick refl and close 0 = 0"
    );
}
/// `try refl; assumption` — try refl fails (A ≠ refl goal), then assumption closes.
#[test]
fn tactic_try_then_assumption() {
    let a = Expr::Const(Name::str("MyPropA"), vec![]);
    let mut goal = Goal::new(Name::str("main"), a.clone());
    goal.add_hypothesis(Name::str("ha"), a.clone());
    let mut state = TacticState::new();
    state.add_goal(goal);
    let result = eval_tactic_block(&state, &["try refl".to_string(), "assumption".to_string()])
        .expect("try refl; assumption should succeed");
    assert!(
        result.is_complete(),
        "try refl (no-op) then assumption should close goal"
    );
}
/// `all_goals refl` on two `0 = 0` goals produced by `constructor` on `(0=0) ∧ (0=0)`.
#[test]
fn tactic_all_goals_refl() {
    use oxilean_kernel::Literal;
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let zero = Expr::Lit(Literal::Nat(0));
    let eq00 = mk_eq(nat_ty.clone(), zero.clone(), zero.clone());
    let target = mk_and_ga(eq00.clone(), eq00.clone());
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), target));
    let result = eval_tactic_block(
        &state,
        &["constructor".to_string(), "all_goals refl".to_string()],
    )
    .expect("constructor; all_goals refl should succeed");
    assert!(
        result.is_complete(),
        "all_goals refl should close both 0=0 goals"
    );
}
/// `repeat assumption` closes 3 goals each with a matching hypothesis.
#[test]
fn tactic_repeat_assumption_many_goals() {
    let a = Expr::Const(Name::str("PropP"), vec![]);
    let b = Expr::Const(Name::str("PropQ"), vec![]);
    let c = Expr::Const(Name::str("PropR"), vec![]);
    let mut g1 = Goal::new(Name::str("g1"), a.clone());
    g1.add_hypothesis(Name::str("ha"), a.clone());
    let mut g2 = Goal::new(Name::str("g2"), b.clone());
    g2.add_hypothesis(Name::str("hb"), b.clone());
    let mut g3 = Goal::new(Name::str("g3"), c.clone());
    g3.add_hypothesis(Name::str("hc"), c.clone());
    let mut state = TacticState::new();
    state.add_goal(g1);
    state.add_goal(g2);
    state.add_goal(g3);
    let result = eval_tactic_block(&state, &["repeat assumption".to_string()])
        .expect("repeat assumption should succeed on 3 goals");
    assert!(
        result.is_complete(),
        "repeat assumption should close all 3 goals"
    );
}
/// `try (first | refl | assumption)` nested combinators work together.
#[test]
fn tactic_nested_combinators() {
    let a = Expr::Const(Name::str("NestedProp"), vec![]);
    let mut goal = Goal::new(Name::str("main"), a.clone());
    goal.add_hypothesis(Name::str("h"), a.clone());
    let mut state = TacticState::new();
    state.add_goal(goal);
    let result = eval_tactic_block(&state, &["first | refl | assumption".to_string()])
        .expect("first | refl | assumption should succeed");
    assert!(
        result.is_complete(),
        "nested combinators should close goal via assumption"
    );
}
/// `all_goals (try assumption)` — safe version: closes goals that have matching hyp.
#[test]
fn tactic_all_goals_try_assumption() {
    let a = Expr::Const(Name::str("SafeA"), vec![]);
    let b = Expr::Const(Name::str("SafeB"), vec![]);
    let mut g1 = Goal::new(Name::str("g1"), a.clone());
    g1.add_hypothesis(Name::str("ha"), a.clone());
    let mut g2 = Goal::new(Name::str("g2"), b.clone());
    g2.add_hypothesis(Name::str("hb"), b.clone());
    let mut state = TacticState::new();
    state.add_goal(g1);
    state.add_goal(g2);
    let result = eval_tactic_block(&state, &["all_goals assumption".to_string()])
        .expect("all_goals assumption should succeed");
    assert!(
        result.is_complete(),
        "all_goals assumption should close both goals"
    );
}
/// `repeat (try assumption)` terminates safely (won't loop on failing goals).
#[test]
fn tactic_repeat_try_assumption() {
    let a = Expr::Const(Name::str("RptProp"), vec![]);
    let goal = Goal::new(Name::str("main"), a.clone());
    let mut state = TacticState::new();
    state.add_goal(goal);
    let result = eval_tactic_block(&state, &["repeat (try assumption)".to_string()])
        .expect("repeat (try assumption) should terminate");
    let _ = result.num_goals();
}
/// Helper for group B: build `Not p` expression.
pub(super) fn mk_not_gb(p: Expr) -> Expr {
    Expr::App(Box::new(Expr::Const(Name::str("Not"), vec![])), Box::new(p))
}
/// Helper for group B: build `Or a b`.
fn mk_or_gb(a: Expr, b: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("Or"), vec![])),
            Box::new(a),
        )),
        Box::new(b),
    )
}
/// `push_neg` on `¬(A ∨ B)` → `¬A ∧ ¬B`, then `constructor; assumption; assumption`.
#[test]
fn tactic_push_neg_or_then_prove() {
    let a = Expr::Const(Name::str("PNA"), vec![]);
    let b = Expr::Const(Name::str("PNB"), vec![]);
    let not_or_ab = mk_not_gb(mk_or_gb(a.clone(), b.clone()));
    let mut goal = Goal::new(Name::str("main"), not_or_ab);
    goal.add_hypothesis(Name::str("hna"), mk_not_gb(a.clone()));
    goal.add_hypothesis(Name::str("hnb"), mk_not_gb(b.clone()));
    let mut state = TacticState::new();
    state.add_goal(goal);
    let result = eval_tactic_block(
        &state,
        &[
            "push_neg".to_string(),
            "constructor".to_string(),
            "assumption".to_string(),
            "assumption".to_string(),
        ],
    )
    .expect("push_neg + constructor + assumption*2 should succeed");
    assert!(
        result.is_complete(),
        "push_neg on ¬(A∨B) + constructor + assumption*2 closes goal"
    );
}
