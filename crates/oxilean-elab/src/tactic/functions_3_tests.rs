// Tests extracted from functions_3.rs

    use super::*;
    use crate::tactic::*;
    use oxilean_kernel::Level;
    fn mk_prop() -> Expr {
        Expr::Sort(Level::zero())
    }
    fn mk_type() -> Expr {
        Expr::Sort(Level::succ(Level::zero()))
    }
    fn mk_pi(name: &str, domain: Expr, body: Expr) -> Expr {
        Expr::Pi(
            BinderInfo::Default,
            Name::str(name),
            Node::new(domain),
            Node::new(body),
        )
    }
    fn mk_eq(ty: Expr, lhs: Expr, rhs: Expr) -> Expr {
        Expr::App(
            Node::new(Expr::App(
                Node::new(Expr::App(
                    Node::new(Expr::Const(Name::str("Eq"), vec![])),
                    Node::new(ty),
                )),
                Node::new(lhs),
            )),
            Node::new(rhs),
        )
    }
    fn mk_or(a: Expr, b: Expr) -> Expr {
        Expr::App(
            Node::new(Expr::App(
                Node::new(Expr::Const(Name::str("Or"), vec![])),
                Node::new(a),
            )),
            Node::new(b),
        )
    }
    fn mk_state_with_goal(goal: Goal) -> TacticState {
        let mut state = TacticState::new();
        state.add_goal(goal);
        state
    }
    #[test]
    fn test_error_display() {
        let e = TacticError::NoGoals;
        assert!(e.to_string().contains("no goals"));
        let e = TacticError::GoalNotFound(Name::str("g1"));
        assert!(e.to_string().contains("g1"));
        let e = TacticError::UnknownTactic("foo".to_string());
        assert!(e.to_string().contains("foo"));
    }
    #[test]
    fn test_goal_create() {
        let goal = Goal::new(Name::str("g1"), Expr::Lit(Literal::nat(42)));
        assert_eq!(goal.name, Name::str("g1"));
        assert_eq!(goal.hypotheses().len(), 0);
        assert!(goal.tag.is_none());
    }
    #[test]
    fn test_goal_add_hypothesis() {
        let mut goal = Goal::new(Name::str("g1"), mk_prop());
        goal.add_hypothesis(Name::str("h"), mk_type());
        assert_eq!(goal.hypotheses().len(), 1);
        assert_eq!(goal.local_ctx.len(), 1);
    }
    #[test]
    fn test_goal_with_hypothesis() {
        let goal = Goal::new(Name::str("g1"), mk_prop());
        let new_goal = goal.with_hypothesis(Name::str("h"), mk_type());
        assert_eq!(new_goal.hypotheses().len(), 1);
        assert_eq!(goal.hypotheses().len(), 0);
    }
    #[test]
    fn test_goal_replace_target() {
        let goal = Goal::new(Name::str("g1"), mk_prop());
        let new_goal = goal.replace_target(mk_type());
        assert_eq!(new_goal.target(), &mk_type());
        assert_ne!(new_goal.mvar_id, goal.mvar_id);
    }
    #[test]
    fn test_goal_has_hypothesis() {
        let mut goal = Goal::new(Name::str("g1"), mk_prop());
        goal.add_hypothesis(Name::str("h"), mk_type());
        assert!(goal.has_hypothesis(&Name::str("h")));
        assert!(!goal.has_hypothesis(&Name::str("x")));
    }
    #[test]
    fn test_goal_find_hypothesis() {
        let mut goal = Goal::new(Name::str("g1"), mk_prop());
        goal.add_hypothesis(Name::str("h"), mk_type());
        assert_eq!(goal.find_hypothesis(&Name::str("h")), Some(&mk_type()));
        assert_eq!(goal.find_hypothesis(&Name::str("x")), None);
    }
    #[test]
    fn test_goal_local_names() {
        let mut goal = Goal::new(Name::str("g1"), mk_prop());
        goal.add_hypothesis(Name::str("h1"), mk_type());
        goal.add_hypothesis(Name::str("h2"), mk_prop());
        let names = goal.local_names();
        assert_eq!(names.len(), 2);
    }
    #[test]
    fn test_tactic_state() {
        let mut state = TacticState::new();
        assert!(state.is_complete());
        let goal = Goal::new(Name::str("g1"), mk_prop());
        state.add_goal(goal);
        assert_eq!(state.num_goals(), 1);
        assert!(!state.is_complete());
        state.solve_goal(&Name::str("g1"));
        assert_eq!(state.num_goals(), 0);
        assert!(state.is_complete());
    }
    #[test]
    fn test_state_focus() {
        let mut state = TacticState::new();
        assert!(state.focus().is_none());
        state.add_goal(Goal::new(Name::str("g1"), mk_prop()));
        state.add_goal(Goal::new(Name::str("g2"), mk_type()));
        assert_eq!(
            state.focus().expect("test operation should succeed").name,
            Name::str("g1")
        );
    }
    #[test]
    fn test_state_rotate() {
        let mut state = TacticState::new();
        state.add_goal(Goal::new(Name::str("g1"), mk_prop()));
        state.add_goal(Goal::new(Name::str("g2"), mk_type()));
        state.add_goal(Goal::new(Name::str("g3"), mk_prop()));
        state.rotate(1);
        assert_eq!(state.goals()[0].name, Name::str("g2"));
        assert_eq!(state.goals()[2].name, Name::str("g1"));
    }
    #[test]
    fn test_state_swap() {
        let mut state = TacticState::new();
        state.add_goal(Goal::new(Name::str("g1"), mk_prop()));
        state.add_goal(Goal::new(Name::str("g2"), mk_type()));
        state.swap();
        assert_eq!(state.goals()[0].name, Name::str("g2"));
        assert_eq!(state.goals()[1].name, Name::str("g1"));
    }
    #[test]
    fn test_state_replace_goal() {
        let mut state = TacticState::new();
        state.add_goal(Goal::new(Name::str("g1"), mk_prop()));
        state.replace_goal(
            &Name::str("g1"),
            vec![
                Goal::new(Name::str("g1a"), mk_prop()),
                Goal::new(Name::str("g1b"), mk_type()),
            ],
        );
        assert_eq!(state.num_goals(), 2);
        assert_eq!(state.goals()[0].name, Name::str("g1a"));
    }
    #[test]
    fn test_state_save_restore() {
        let mut state = TacticState::new();
        state.add_goal(Goal::new(Name::str("g1"), mk_prop()));
        let saved = state.save_state();
        state.solve_goal(&Name::str("g1"));
        assert!(state.is_complete());
        state.restore_state(saved);
        assert_eq!(state.num_goals(), 1);
    }
    #[test]
    fn test_tactic_intro_pi() {
        let target = mk_pi("x", mk_type(), mk_prop());
        let goal = Goal::new(Name::str("g1"), target);
        let state = mk_state_with_goal(goal);
        let result = tactic_intro(&state, Name::str("x")).expect("tactic should succeed");
        assert_eq!(result.num_goals(), 1);
        let new_goal = &result.goals()[0];
        assert!(new_goal.has_hypothesis(&Name::str("x")));
        assert_eq!(new_goal.target(), &mk_prop());
    }
    #[test]
    fn test_tactic_intro_not_pi() {
        let goal = Goal::new(Name::str("g1"), mk_prop());
        let state = mk_state_with_goal(goal);
        let result = tactic_intro(&state, Name::str("x"));
        assert!(result.is_err());
    }
    #[test]
    fn test_tactic_intro_no_goals() {
        let state = TacticState::new();
        let result = tactic_intro(&state, Name::str("x"));
        assert!(matches!(result, Err(TacticError::NoGoals)));
    }
    #[test]
    fn test_tactic_intros() {
        let target = mk_pi("a", mk_type(), mk_pi("b", mk_type(), mk_prop()));
        let goal = Goal::new(Name::str("g1"), target);
        let state = mk_state_with_goal(goal);
        let result = tactic_intros(&state, &[Name::str("x"), Name::str("y")])
            .expect("tactic should succeed");
        assert_eq!(result.num_goals(), 1);
        let new_goal = &result.goals()[0];
        assert!(new_goal.has_hypothesis(&Name::str("x")));
        assert!(new_goal.has_hypothesis(&Name::str("y")));
    }
    #[test]
    fn test_tactic_exact() {
        let goal = Goal::new(Name::str("g1"), mk_prop());
        let state = mk_state_with_goal(goal);
        let result = tactic_exact(&state, mk_prop()).expect("tactic should succeed");
        assert!(result.is_complete());
    }
    #[test]
    fn test_tactic_exact_no_goals() {
        let state = TacticState::new();
        let result = tactic_exact(&state, mk_prop());
        assert!(result.is_err());
    }
    #[test]
    fn test_tactic_assumption_found() {
        let target = mk_prop();
        let mut goal = Goal::new(Name::str("g1"), target.clone());
        goal.add_hypothesis(Name::str("h"), target);
        let state = mk_state_with_goal(goal);
        let result = tactic_assumption(&state).expect("tactic should succeed");
        assert!(result.is_complete());
    }
    #[test]
    fn test_tactic_assumption_not_found() {
        let mut goal = Goal::new(Name::str("g1"), mk_prop());
        goal.add_hypothesis(Name::str("h"), mk_type());
        let state = mk_state_with_goal(goal);
        let result = tactic_assumption(&state);
        assert!(result.is_err());
    }
    #[test]
    fn test_tactic_refl_eq() {
        let nat_const = Expr::Const(Name::str("Nat"), vec![]);
        let zero = Expr::Lit(Literal::nat(0));
        let target = mk_eq(nat_const, zero.clone(), zero);
        let goal = Goal::new(Name::str("g1"), target);
        let state = mk_state_with_goal(goal);
        let result = tactic_refl(&state).expect("tactic should succeed");
        assert!(result.is_complete());
    }
    #[test]
    fn test_tactic_refl_not_eq() {
        let nat_const = Expr::Const(Name::str("Nat"), vec![]);
        let zero = Expr::Lit(Literal::nat(0));
        let one = Expr::Lit(Literal::nat(1));
        let target = mk_eq(nat_const, zero, one);
        let goal = Goal::new(Name::str("g1"), target);
        let state = mk_state_with_goal(goal);
        let result = tactic_refl(&state);
        assert!(result.is_err());
    }
    #[test]
    fn test_tactic_trivial_refl() {
        let goal = Goal::new(Name::str("g1"), mk_prop());
        let state = mk_state_with_goal(goal);
        let result = tactic_trivial(&state).expect("tactic should succeed");
        assert!(result.is_complete());
    }
    #[test]
    fn test_tactic_trivial_assumption() {
        let target = mk_type();
        let mut goal = Goal::new(Name::str("g1"), target.clone());
        goal.add_hypothesis(Name::str("h"), target);
        let state = mk_state_with_goal(goal);
        let result = tactic_trivial(&state).expect("tactic should succeed");
        assert!(result.is_complete());
    }
    #[test]
    fn test_tactic_trivial_true() {
        let target = Expr::Const(Name::str("True"), vec![]);
        let goal = Goal::new(Name::str("g1"), target);
        let state = mk_state_with_goal(goal);
        let result = tactic_trivial(&state).expect("tactic should succeed");
        assert!(result.is_complete());
    }
    #[test]
    fn test_tactic_sorry() {
        let goal = Goal::new(Name::str("g1"), mk_type());
        let state = mk_state_with_goal(goal);
        let result = tactic_sorry(&state).expect("tactic should succeed");
        assert!(result.is_complete());
    }
    #[test]
    fn test_tactic_apply_matching() {
        let target = mk_prop();
        let goal = Goal::new(Name::str("g1"), target.clone());
        let state = mk_state_with_goal(goal);
        let result = tactic_apply(&state, target).expect("tactic should succeed");
        assert!(result.is_complete());
    }
    #[test]
    fn test_tactic_apply_pi() {
        let pi = mk_pi("x", mk_type(), mk_prop());
        let goal = Goal::new(Name::str("g1"), mk_prop());
        let state = mk_state_with_goal(goal);
        let result = tactic_apply(&state, pi).expect("tactic should succeed");
        assert_eq!(result.num_goals(), 1);
    }
    #[test]
    fn test_tactic_constructor_true() {
        let target = Expr::Const(Name::str("True"), vec![]);
        let goal = Goal::new(Name::str("g1"), target);
        let state = mk_state_with_goal(goal);
        let result = tactic_constructor(&state).expect("tactic should succeed");
        assert!(result.is_complete());
    }
    #[test]
    fn test_tactic_left_or() {
        let a = Expr::Const(Name::str("A"), vec![]);
        let b = Expr::Const(Name::str("B"), vec![]);
        let target = mk_or(a.clone(), b);
        let goal = Goal::new(Name::str("g1"), target);
        let state = mk_state_with_goal(goal);
        let result = tactic_left(&state).expect("tactic should succeed");
        assert_eq!(result.num_goals(), 1);
        assert_eq!(result.goals()[0].target(), &a);
    }
    #[test]
    fn test_tactic_right_or() {
        let a = Expr::Const(Name::str("A"), vec![]);
        let b = Expr::Const(Name::str("B"), vec![]);
        let target = mk_or(a, b.clone());
        let goal = Goal::new(Name::str("g1"), target);
        let state = mk_state_with_goal(goal);
        let result = tactic_right(&state).expect("tactic should succeed");
        assert_eq!(result.num_goals(), 1);
        assert_eq!(result.goals()[0].target(), &b);
    }
    #[test]
    fn test_tactic_exfalso() {
        let goal = Goal::new(Name::str("g1"), mk_type());
        let state = mk_state_with_goal(goal);
        let result = tactic_exfalso(&state).expect("tactic should succeed");
        assert_eq!(result.num_goals(), 1);
        assert_eq!(
            result.goals()[0].target(),
            &Expr::Const(Name::str("False"), vec![])
        );
    }
    #[test]
    fn test_tactic_clear() {
        let mut goal = Goal::new(Name::str("g1"), mk_prop());
        goal.add_hypothesis(Name::str("h"), mk_type());
        let state = mk_state_with_goal(goal);
        let result = tactic_clear(&state, &Name::str("h")).expect("tactic should succeed");
        assert!(!result.goals()[0].has_hypothesis(&Name::str("h")));
    }
    #[test]
    fn test_tactic_clear_not_found() {
        let goal = Goal::new(Name::str("g1"), mk_prop());
        let state = mk_state_with_goal(goal);
        let result = tactic_clear(&state, &Name::str("h"));
        assert!(result.is_err());
    }
    #[test]
    fn test_tactic_rename() {
        let mut goal = Goal::new(Name::str("g1"), mk_prop());
        goal.add_hypothesis(Name::str("old"), mk_type());
        let state = mk_state_with_goal(goal);
        let result = tactic_rename(&state, &Name::str("old"), Name::str("new_name"))
            .expect("tactic should succeed");
        assert!(result.goals()[0].has_hypothesis(&Name::str("new_name")));
        assert!(!result.goals()[0].has_hypothesis(&Name::str("old")));
    }
    #[test]
    fn test_tactic_revert() {
        let mut goal = Goal::new(Name::str("g1"), mk_prop());
        goal.add_hypothesis(Name::str("h"), mk_type());
        let state = mk_state_with_goal(goal);
        let result = tactic_revert(&state, &Name::str("h")).expect("tactic should succeed");
        let new_goal = &result.goals()[0];
        assert!(!new_goal.has_hypothesis(&Name::str("h")));
        assert!(new_goal.target().is_pi());
    }
    #[test]
    fn test_tactic_have_with_proof() {
        let goal = Goal::new(Name::str("g1"), mk_prop());
        let state = mk_state_with_goal(goal);
        let result = tactic_have(&state, Name::str("h"), mk_type(), Some(mk_type()))
            .expect("tactic should succeed");
        assert_eq!(result.num_goals(), 1);
        assert!(result.goals()[0].has_hypothesis(&Name::str("h")));
    }
    #[test]
    fn test_tactic_have_without_proof() {
        let goal = Goal::new(Name::str("g1"), mk_prop());
        let state = mk_state_with_goal(goal);
        let result =
            tactic_have(&state, Name::str("h"), mk_type(), None).expect("tactic should succeed");
        assert_eq!(result.num_goals(), 2);
    }
    #[test]
    fn test_tactic_suffices() {
        let goal = Goal::new(Name::str("g1"), mk_prop());
        let state = mk_state_with_goal(goal);
        let result =
            tactic_suffices(&state, Name::str("h"), mk_type()).expect("tactic should succeed");
        assert_eq!(result.num_goals(), 2);
    }
    #[test]
    fn test_eval_tactic_sorry() {
        let goal = Goal::new(Name::str("g1"), mk_type());
        let state = mk_state_with_goal(goal);
        let env = oxilean_kernel::Environment::new();
        let result = eval_tactic(&state, "sorry", &env).expect("tactic should succeed");
        assert!(result.is_complete());
    }
    #[test]
    fn test_eval_tactic_intro() {
        let target = mk_pi("x", mk_type(), mk_prop());
        let goal = Goal::new(Name::str("g1"), target);
        let state = mk_state_with_goal(goal);
        let env = oxilean_kernel::Environment::new();
        let result = eval_tactic(&state, "intro x", &env).expect("tactic should succeed");
        assert_eq!(result.num_goals(), 1);
    }
    #[test]
    fn test_eval_tactic_unknown() {
        let goal = Goal::new(Name::str("g1"), mk_prop());
        let state = mk_state_with_goal(goal);
        let env = oxilean_kernel::Environment::new();
        let result = eval_tactic(&state, "nonexistent", &env);
        assert!(matches!(result, Err(TacticError::UnknownTactic(_))));
    }
    #[test]
    fn test_eval_tactic_block() {
        let target = mk_pi("x", mk_prop(), mk_prop());
        let mut goal = Goal::new(Name::str("g1"), target);
        goal.tag = Some("test".to_string());
        let state = mk_state_with_goal(goal);
        let tactics = vec!["intro h".to_string(), "sorry".to_string()];
        let env = oxilean_kernel::Environment::new();
        let result = eval_tactic_block(&state, &tactics, &env).expect("tactic should succeed");
        assert!(result.is_complete());
    }
    #[test]
    fn test_eval_tactic_block_empty() {
        let goal = Goal::new(Name::str("g1"), mk_prop());
        let state = mk_state_with_goal(goal);
        let env = oxilean_kernel::Environment::new();
        let result = eval_tactic_block(&state, &[], &env).expect("tactic should succeed");
        assert_eq!(result.num_goals(), 1);
    }
    #[test]
    fn test_tactic_registry() {
        let registry = TacticRegistry::default();
        assert!(registry.get(&Name::str("intro")).is_some());
        assert!(registry.get(&Name::str("apply")).is_some());
        assert!(registry.get(&Name::str("exact")).is_some());
        assert!(registry.get(&Name::str("refl")).is_some());
        assert!(registry.get(&Name::str("sorry")).is_some());
        assert!(registry.get(&Name::str("unknown")).is_none());
    }
    #[test]
    fn test_tactic_registry_all() {
        let registry = TacticRegistry::default();
        let all = registry.all_tactics();
        assert!(all.len() >= 15);
    }
    #[test]
    fn test_tactic_registry_execute() {
        let registry = TacticRegistry::default();
        let goal = Goal::new(Name::str("g1"), mk_prop());
        let state = mk_state_with_goal(goal);
        let env = oxilean_kernel::Environment::new();
        let result = registry
            .execute("sorry", &state, &[], &env)
            .expect("test operation should succeed");
        assert!(result.is_complete());
    }
    #[test]
    fn test_tactic_registry_execute_unknown() {
        let registry = TacticRegistry::default();
        let state = TacticState::new();
        let env = oxilean_kernel::Environment::new();
        let result = registry.execute("nonexistent", &state, &[], &env);
        assert!(result.is_err());
    }
    #[test]
    fn test_tactic_registry_arity() {
        let registry = TacticRegistry::default();
        assert_eq!(registry.arity(&Name::str("refl")), Some(Some(0)));
        assert_eq!(registry.arity(&Name::str("intro")), Some(Some(1)));
        assert_eq!(registry.arity(&Name::str("intros")), Some(None));
    }

    // ─── nlinarith / Positivstellensatz-lite tests ───────────────────────────

    /// Build `LE.le 0 (HPow.hPow x 2)` — i.e., `0 ≤ x^2`.
    ///
    /// This is the canonical goal that linarith cannot handle because `x^2` is
    /// a nonlinear atom, but nlinarith should close via `x^2 ≥ 0`.
    fn mk_sq_ge_zero_goal() -> (TacticState, Expr) {
        // HPow.hPow x 2
        let x = Expr::Const(Name::str("x"), vec![]);
        let two = Expr::Lit(Literal::nat(2));
        let x_sq = Expr::App(
            Node::new(Expr::App(
                Node::new(Expr::Const(Name::str("HPow.hPow"), vec![])),
                Node::new(x),
            )),
            Node::new(two),
        );
        // 0 ≤ x^2  →  LE.le 0 x^2
        let zero = Expr::Lit(Literal::nat(0));
        let target = Expr::App(
            Node::new(Expr::App(
                Node::new(Expr::Const(Name::str("LE.le"), vec![])),
                Node::new(zero),
            )),
            Node::new(x_sq.clone()),
        );
        let goal = Goal::new(Name::str("g1"), target);
        let mut state = TacticState::new();
        state.add_goal(goal);
        (state, x_sq)
    }

    #[test]
    fn test_nlinarith_closes_sq_ge_zero() {
        // nlinarith should close `0 ≤ x^2`.
        let (state, _) = mk_sq_ge_zero_goal();
        let result = try_nlinarith_with_positivstellensatz(&state);
        assert!(
            result.is_ok(),
            "nlinarith should close 0 ≤ x^2, got: {:?}",
            result.err()
        );
        assert!(result.unwrap().is_complete());
    }

    #[test]
    fn test_linarith_cannot_close_sq_ge_zero() {
        // linarith must NOT close `0 ≤ x^2` — the test verifies the split is real.
        let (state, _) = mk_sq_ge_zero_goal();
        let result = try_linarith_with_hyps(&state);
        assert!(
            result.is_err(),
            "linarith must not close 0 ≤ x^2 (nonlinear goal)"
        );
    }

    #[test]
    fn test_nlinarith_augment_increases_constraints() {
        // positivstellensatz_augment should add at least one constraint when
        // a nonlinear atom appears in the base set.
        let x = Expr::Const(Name::str("x"), vec![]);
        let two = Expr::Lit(Literal::nat(2));
        let x_sq = Expr::App(
            Node::new(Expr::App(
                Node::new(Expr::Const(Name::str("HPow.hPow"), vec![])),
                Node::new(x),
            )),
            Node::new(two),
        );
        // Build a base constraint set that contains the x_sq atom.
        // `0 ≤ x^2` parsed: diff = 0 - x^2 = -x^2;  SymLinCon { lhs: -x^2, strict:false }
        // After parse_sym_lin_cons the key for x_sq will be __app_{:?}
        let zero = Expr::Lit(Literal::nat(0));
        let target = Expr::App(
            Node::new(Expr::App(
                Node::new(Expr::Const(Name::str("LE.le"), vec![])),
                Node::new(zero),
            )),
            Node::new(x_sq.clone()),
        );
        let base_cons = parse_sym_lin_cons(&target).expect("should parse LE.le 0 x^2");
        let neg_con = base_cons[0].negate();
        let base = vec![neg_con];

        let mut atoms: Vec<Expr> = Vec::new();
        collect_atom_exprs_from(&target, &mut atoms);

        let augmented = positivstellensatz_augment(&base, &atoms);
        assert!(
            augmented.len() > base.len(),
            "augmentation should add at least one constraint for x^2 atom"
        );
    }

    #[test]
    fn test_nlinarith_empty_state_returns_error() {
        // An empty TacticState has no goals; nlinarith should return Err gracefully.
        let state = TacticState::new();
        let result = try_nlinarith_with_positivstellensatz(&state);
        assert!(
            result.is_err(),
            "nlinarith on empty state should return Err"
        );
    }

    #[test]
    fn test_nlinarith_still_closes_linear_goals() {
        // A linear goal that linarith closes should also be closed by nlinarith.
        // Goal: 0 ≤ 5  →  LE.le 0 5
        let zero = Expr::Lit(Literal::nat(0));
        let five = Expr::Lit(Literal::nat(5));
        let target = Expr::App(
            Node::new(Expr::App(
                Node::new(Expr::Const(Name::str("LE.le"), vec![])),
                Node::new(zero),
            )),
            Node::new(five),
        );
        let goal = Goal::new(Name::str("g1"), target);
        let mut state = TacticState::new();
        state.add_goal(goal);

        // linarith should handle it.
        let lin_result = try_linarith_with_hyps(&state);
        assert!(lin_result.is_ok(), "linarith must close 0 ≤ 5");

        // nlinarith must also handle it (falls through to linear pass).
        // try_nlinarith_with_positivstellensatz tries nlinarith augmentation;
        // if no nonlinear atoms, it returns Err — so we test via the direct nlinarith
        // arm by calling try_linarith_with_hyps, which is embedded in the arm.
        // The arm succeeds before reaching the nlinarith augmentation step.
        let state2 = TacticState::new();
        let mut state2 = state2;
        let goal2 = Goal::new(
            Name::str("g1"),
            Expr::App(
                Node::new(Expr::App(
                    Node::new(Expr::Const(Name::str("LE.le"), vec![])),
                    Node::new(Expr::Lit(Literal::nat(0))),
                )),
                Node::new(Expr::Lit(Literal::nat(5))),
            ),
        );
        state2.add_goal(goal2);
        let env = oxilean_kernel::Environment::new();
        // nlinarith arm in eval_tactic first calls try_linarith_with_hyps; this
        // means the goal closes before sorry — we verify it works end-to-end.
        let result = eval_tactic(&state2, "nlinarith", &env);
        assert!(result.is_ok(), "nlinarith must close linear goal 0 ≤ 5");
        assert!(
            result.unwrap().is_complete(),
            "state must be complete after nlinarith on linear goal"
        );
    }

    #[test]
    fn test_nlinarith_with_nonneg_hypothesis() {
        // Goal: 0 ≤ x^2 with an extra (irrelevant) hypothesis h : 0 ≤ x.
        // nlinarith should still close it via the square nonnegativity.
        let x = Expr::Const(Name::str("x"), vec![]);
        let two = Expr::Lit(Literal::nat(2));
        let x_sq = Expr::App(
            Node::new(Expr::App(
                Node::new(Expr::Const(Name::str("HPow.hPow"), vec![])),
                Node::new(x.clone()),
            )),
            Node::new(two),
        );
        let zero = Expr::Lit(Literal::nat(0));
        let target = Expr::App(
            Node::new(Expr::App(
                Node::new(Expr::Const(Name::str("LE.le"), vec![])),
                Node::new(zero.clone()),
            )),
            Node::new(x_sq),
        );
        // h : 0 ≤ x
        let hyp_ty = Expr::App(
            Node::new(Expr::App(
                Node::new(Expr::Const(Name::str("LE.le"), vec![])),
                Node::new(zero),
            )),
            Node::new(x),
        );
        let mut goal = Goal::new(Name::str("g1"), target);
        goal.add_hypothesis(Name::str("h"), hyp_ty);
        let mut state = TacticState::new();
        state.add_goal(goal);
        let result = try_nlinarith_with_positivstellensatz(&state);
        assert!(
            result.is_ok(),
            "nlinarith should close 0 ≤ x^2 even with extra hypothesis"
        );
    }

    // ─── Tactic error message tests ──────────────────────────────────────────

    /// format_tactic_failure with no goal contains the tactic name and error.
    #[test]
    fn test_format_tactic_failure_no_goal() {
        let msg = format_tactic_failure("omega", &TacticError::NoGoals, None);
        assert!(msg.contains("omega"), "message must mention tactic name");
        assert!(
            msg.contains("no goals"),
            "message must mention error kind"
        );
        assert!(
            !msg.contains("goal:"),
            "no goal label expected when goal is None"
        );
    }

    /// format_tactic_failure with a goal includes the goal in the message.
    #[test]
    fn test_format_tactic_failure_with_goal() {
        let msg = format_tactic_failure(
            "ring",
            &TacticError::TypeMismatch("expected ring".to_string()),
            Some("⊢ 1 = 2"),
        );
        assert!(msg.contains("ring"), "message must mention tactic name");
        assert!(msg.contains("goal"), "message must mention the goal label");
        assert!(msg.contains("⊢ 1 = 2"), "goal must appear verbatim");
    }

    /// TacticError::UnknownTactic Display contains the tactic name.
    #[test]
    fn test_unknown_tactic_display() {
        let e = TacticError::UnknownTactic("foo".to_string());
        let s = e.to_string();
        assert!(s.contains("foo"), "display must include the tactic name");
        assert!(
            s.contains("unknown"),
            "display must include 'unknown' keyword"
        );
    }

    /// TacticError::InvalidArg Display preserves the message.
    #[test]
    fn test_invalid_arg_display_preserves_message() {
        let e = TacticError::InvalidArg("x must be positive".to_string());
        let s = e.to_string();
        assert!(
            s.contains("positive"),
            "display must include the argument message"
        );
    }

    /// TacticError::TypeMismatchDetailed Display includes expected and actual types.
    #[test]
    fn test_type_mismatch_detailed_display() {
        let e = TacticError::TypeMismatchDetailed {
            expected: "Nat".to_string(),
            actual: "Int".to_string(),
            context: "ring tactic".to_string(),
        };
        let s = e.to_string();
        assert!(s.contains("Nat"), "display must include expected type");
        assert!(s.contains("Int"), "display must include actual type");
        assert!(
            s.contains("ring tactic"),
            "display must include context string"
        );
    }

    /// format_tactic_failure with an empty goal string omits the goal line.
    #[test]
    fn test_format_tactic_failure_empty_goal_string() {
        let msg = format_tactic_failure("omega", &TacticError::NoGoals, Some(""));
        assert!(
            !msg.contains("goal:"),
            "empty goal string should not produce a goal line"
        );
    }

    /// TacticError implements std::error::Error.
    #[test]
    fn test_tactic_error_implements_std_error() {
        let e: &dyn std::error::Error = &TacticError::InternalError("oops".to_string());
        assert!(e.to_string().contains("oops"));
    }

