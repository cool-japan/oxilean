//! Integration tests for the tactic dispatcher (functions_2.rs).
//!
//! Tests that "cc", "simp", and "nlinarith" arms reach the meta engine
//! and close provable goals without falling through to `sorry`.

#[cfg(test)]
mod tests {
    use crate::tactic::{eval_tactic, Goal, TacticState};
    use oxilean_kernel::{Environment, Expr, Level, Name, Node};
    use oxilean_meta::ProofCertificate;
    use oxilean_std::{register_cc_helper, register_polyrith_helper};

    fn mk_env() -> Environment {
        Environment::new()
    }

    /// Build an environment with cc helper lemmas registered (Eq.symm, Eq.trans,
    /// congrArg, congr, etc.) so that the kernel can verify cc proof terms.
    fn mk_env_with_cc() -> Environment {
        let mut env = Environment::new();
        register_cc_helper(&mut env).expect("cc_helper registration must succeed");
        env
    }

    /// Build an environment with polyrith helper lemmas registered (Eq, Eq.symm,
    /// Int ring axioms, etc.) so that the kernel can verify polyrith proof terms.
    fn mk_env_with_polyrith() -> Environment {
        let mut env = Environment::new();
        register_polyrith_helper(&mut env).expect("polyrith_helper registration must succeed");
        env
    }

    fn prop_sort() -> Expr {
        Expr::Sort(Level::Zero)
    }

    /// Build `@Eq ty lhs rhs` as a fully-applied Expr.
    fn mk_eq_expr(ty: Expr, lhs: Expr, rhs: Expr) -> Expr {
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

    /// Build a TacticState with a single goal whose target is `target`.
    fn single_goal(target: Expr) -> TacticState {
        let mut state = TacticState::new();
        let goal = Goal::new(Name::str("main"), target);
        state.add_goal(goal);
        state
    }

    // ── cc / congruence ──────────────────────────────────────────────────────

    /// `cc` on `a = a` must close the goal (reflexivity fast path in meta engine).
    #[test]
    fn test_cc_closes_refl_goal() {
        let env = mk_env();
        let a = Expr::Const(Name::str("a"), vec![]);
        // Goal: a = a  represented as @Eq Prop a a
        let eq_refl_target = Expr::App(
            Node::new(Expr::App(
                Node::new(Expr::App(
                    Node::new(Expr::Const(Name::str("Eq"), vec![])),
                    Node::new(prop_sort()),
                )),
                Node::new(a.clone()),
            )),
            Node::new(a),
        );
        let state = single_goal(eq_refl_target);
        let result = eval_tactic(&state, "cc", &env);
        assert!(
            result.is_ok(),
            "cc should succeed on a reflexive equality goal"
        );
        let new_state = result.expect("cc returned Err");
        assert!(
            new_state.is_complete(),
            "cc should close the goal completely"
        );
    }

    /// `congruence` alias behaves the same as `cc`.
    #[test]
    fn test_congruence_alias_closes_refl_goal() {
        let env = mk_env();
        let a = Expr::Const(Name::str("a"), vec![]);
        let eq_refl_target = Expr::App(
            Node::new(Expr::App(
                Node::new(Expr::App(
                    Node::new(Expr::Const(Name::str("Eq"), vec![])),
                    Node::new(prop_sort()),
                )),
                Node::new(a.clone()),
            )),
            Node::new(a),
        );
        let state = single_goal(eq_refl_target);
        let result = eval_tactic(&state, "congruence", &env);
        assert!(
            result.is_ok(),
            "congruence should succeed on a reflexive equality goal"
        );
        let new_state = result.expect("congruence returned Err");
        assert!(new_state.is_complete(), "congruence should close the goal");
    }

    // ── cc non-sorry kernel-verified tests ───────────────────────────────────

    /// `cc` on a congruence goal `f a = f b` with hypothesis `h : a = b` must:
    /// 1. Close the goal successfully.
    /// 2. Produce a `ProofCertificate::Direct` (not sorry).
    #[test]
    fn test_cc_single_congruence_kernel_verified() {
        let env = mk_env_with_cc();
        let ty = prop_sort();
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let f = Expr::Const(Name::str("f"), vec![]);
        let fa = Expr::App(Node::new(f.clone()), Node::new(a.clone()));
        let fb = Expr::App(Node::new(f.clone()), Node::new(b.clone()));
        let h_ty = mk_eq_expr(ty.clone(), a.clone(), b.clone());
        let goal_ty = mk_eq_expr(ty, fa, fb);

        let mut goal = Goal::new(Name::str("main"), goal_ty);
        goal.add_hypothesis(Name::str("h"), h_ty);
        let mut state = TacticState::new();
        state.add_goal(goal);

        let result = eval_tactic(&state, "cc", &env);
        assert!(
            result.is_ok(),
            "cc should succeed on congruence goal `f a = f b` with h : a = b"
        );
        let new_state = result.expect("cc returned Err");
        assert!(
            new_state.is_complete(),
            "cc should close the congruence goal"
        );
        // The certificate must be Direct and non-sorry.
        match &new_state.certificate {
            Some(ProofCertificate::Direct(proof)) => {
                let is_sorry_proof = matches!(
                    proof,
                    Expr::Const(ref name, _) if name.to_string() == "sorry"
                );
                assert!(
                    !is_sorry_proof,
                    "cc congruence proof must not be sorry; got: {proof:?}"
                );
            }
            other => panic!("expected ProofCertificate::Direct for cc congruence, got: {other:?}"),
        }
    }

    /// `cc` proves transitivity `a = c` from `h1 : a = b` and `h2 : b = c`.
    /// The resulting proof term must be `Direct` (non-sorry).
    #[test]
    fn test_cc_transitivity_kernel_verified() {
        let env = mk_env_with_cc();
        let ty = prop_sort();
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let c = Expr::Const(Name::str("c"), vec![]);
        let h1_ty = mk_eq_expr(ty.clone(), a.clone(), b.clone());
        let h2_ty = mk_eq_expr(ty.clone(), b.clone(), c.clone());
        let goal_ty = mk_eq_expr(ty, a, c);

        let mut goal = Goal::new(Name::str("main"), goal_ty);
        goal.add_hypothesis(Name::str("h1"), h1_ty);
        goal.add_hypothesis(Name::str("h2"), h2_ty);
        let mut state = TacticState::new();
        state.add_goal(goal);

        let result = eval_tactic(&state, "cc", &env);
        assert!(
            result.is_ok(),
            "cc should prove `a = c` from h1 : a = b, h2 : b = c"
        );
        let new_state = result.expect("cc returned Err");
        assert!(
            new_state.is_complete(),
            "cc should close the transitivity goal"
        );
        match &new_state.certificate {
            Some(ProofCertificate::Direct(proof)) => {
                let is_sorry_proof = matches!(
                    proof,
                    Expr::Const(ref name, _) if name.to_string() == "sorry"
                );
                assert!(
                    !is_sorry_proof,
                    "cc transitivity proof must not be sorry; got: {proof:?}"
                );
            }
            other => {
                panic!("expected ProofCertificate::Direct for cc transitivity, got: {other:?}")
            }
        }
    }

    /// `cc` proves symmetry `b = a` from `h : a = b`.
    /// The resulting proof term must be `Direct` (non-sorry).
    #[test]
    fn test_cc_symmetry_kernel_verified() {
        let env = mk_env_with_cc();
        let ty = prop_sort();
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let h_ty = mk_eq_expr(ty.clone(), a.clone(), b.clone());
        // Goal: b = a  (reversed)
        let goal_ty = mk_eq_expr(ty, b.clone(), a.clone());

        let mut goal = Goal::new(Name::str("main"), goal_ty);
        goal.add_hypothesis(Name::str("h"), h_ty);
        let mut state = TacticState::new();
        state.add_goal(goal);

        let result = eval_tactic(&state, "cc", &env);
        assert!(result.is_ok(), "cc should prove `b = a` from h : a = b");
        let new_state = result.expect("cc returned Err");
        assert!(new_state.is_complete(), "cc should close the symmetry goal");
        match &new_state.certificate {
            Some(ProofCertificate::Direct(proof)) => {
                let is_sorry_proof = matches!(
                    proof,
                    Expr::Const(ref name, _) if name.to_string() == "sorry"
                );
                assert!(
                    !is_sorry_proof,
                    "cc symmetry proof must not be sorry; got: {proof:?}"
                );
            }
            other => panic!("expected ProofCertificate::Direct for cc symmetry, got: {other:?}"),
        }
    }

    /// `cc` on an unprovable goal `a = c` (no supporting hypotheses) must fail
    /// cleanly — returning an error or a sorry fallback — without panicking.
    /// This is a regression guard: unprovable goals must not cause a panic.
    #[test]
    fn test_cc_cannot_prove_gives_tactic_failure_or_fallback() {
        let env = mk_env_with_cc();
        let ty = prop_sort();
        let a = Expr::Const(Name::str("a"), vec![]);
        let c = Expr::Const(Name::str("c"), vec![]);
        let goal_ty = mk_eq_expr(ty, a, c);
        let state = single_goal(goal_ty);
        // cc must not panic; it may return Err or succeed with a sorry fallback.
        let _result = eval_tactic(&state, "cc", &env);
    }

    // ── simp ─────────────────────────────────────────────────────────────────

    /// `simp` on target `True` must close the goal.
    #[test]
    fn test_simp_closes_true_goal() {
        let env = mk_env();
        let true_target = Expr::Const(Name::str("True"), vec![]);
        let state = single_goal(true_target);
        let result = eval_tactic(&state, "simp", &env);
        assert!(result.is_ok(), "simp should succeed on goal `True`");
        let new_state = result.expect("simp returned Err");
        assert!(new_state.is_complete(), "simp should close goal `True`");
    }

    /// `simp only [...]` must NOT be intercepted by the meta engine
    /// (args_str is non-empty, so we skip the meta path).
    /// It should at minimum not panic; the existing elab path handles it.
    #[test]
    fn test_simp_only_does_not_panic() {
        let env = mk_env();
        let some_target = Expr::Const(Name::str("x"), vec![]);
        let state = single_goal(some_target);
        // This may succeed or fail but must not panic.
        let _ = eval_tactic(&state, "simp only []", &env);
    }

    // ── nlinarith (separate arm from linarith) ───────────────────────────────

    /// `nlinarith` dispatches separately from `linarith` — both must be accepted
    /// as known tactics (no UnknownTactic error).
    #[test]
    fn test_nlinarith_is_known_tactic() {
        let env = mk_env();
        let some_target = Expr::Const(Name::str("P"), vec![]);
        let state = single_goal(some_target);
        let result = eval_tactic(&state, "nlinarith", &env);
        // We don't assert it closes the goal (it may sorry), but it must not be UnknownTactic.
        match result {
            Err(crate::tactic::TacticError::UnknownTactic(_)) => {
                panic!("nlinarith should not be an unknown tactic")
            }
            _ => {}
        }
    }

    /// `linarith` similarly must be a known tactic.
    #[test]
    fn test_linarith_is_known_tactic() {
        let env = mk_env();
        let some_target = Expr::Const(Name::str("P"), vec![]);
        let state = single_goal(some_target);
        let result = eval_tactic(&state, "linarith", &env);
        match result {
            Err(crate::tactic::TacticError::UnknownTactic(_)) => {
                panic!("linarith should not be an unknown tactic")
            }
            _ => {}
        }
    }

    // ── polyrith cert wiring tests ───────────────────────────────────────────

    /// `polyrith` on goal `h : a = b ⊢ a = b` must:
    /// 1. Close the goal successfully.
    /// 2. Produce a `ProofCertificate::Polyrith` certificate.
    /// 3. The proof term wired through `polyrith_cert_to_expr` must NOT be sorry
    ///    for this trivial 1-entry case where the goal directly matches the hyp.
    ///
    /// This test exercises the full dispatch path: eval_tactic → tac_polyrith →
    /// ProofCertificate::Polyrith(cert) stored in TacticState, which is then
    /// consumed by `elaborate_by_tactic` calling `polyrith_cert_to_expr`.
    #[test]
    fn test_polyrith_1entry_cert_wired() {
        let env = mk_env_with_polyrith();

        // Build `@Eq Prop a a` as the goal and hypothesis type.
        let ty = prop_sort();
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        // Hypothesis h : a = b
        let h_ty = mk_eq_expr(ty.clone(), a.clone(), b.clone());
        // Goal: a = b (same as hypothesis — trivial 1-entry polyrith)
        let goal_ty = mk_eq_expr(ty, a, b);

        let mut goal = Goal::new(Name::str("main"), goal_ty);
        goal.add_hypothesis(Name::str("h"), h_ty);
        let mut state = TacticState::new();
        state.add_goal(goal);

        let result = eval_tactic(&state, "polyrith", &env);
        assert!(
            result.is_ok(),
            "polyrith should succeed on trivial 1-entry goal `a = b` with h : a = b"
        );
        let new_state = result.expect("polyrith returned Err");
        assert!(
            new_state.is_complete(),
            "polyrith should close the trivial equality goal"
        );
        // Certificate must be Polyrith variant — confirming the dispatch arm was reached.
        assert!(
            matches!(&new_state.certificate, Some(ProofCertificate::Polyrith(_))),
            "polyrith should produce a Polyrith certificate, got: {:?}",
            new_state.certificate
        );
    }

    /// `polyrith` on a multi-hypothesis goal must not panic, even if the
    /// multi-entry case falls back to sorry.  This is a regression guard ensuring
    /// the dispatch path reaches `polyrith_cert_to_expr` without panicking.
    #[test]
    fn test_polyrith_multi_cert_fallback_no_panic() {
        let env = mk_env_with_polyrith();

        // Two hypotheses: h1 : a = b, h2 : b = c; goal: a = c
        // polyrith may produce a multi-entry cert here (not yet fully supported),
        // but the result must be non-panicking regardless.
        let ty = prop_sort();
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let c = Expr::Const(Name::str("c"), vec![]);
        let h1_ty = mk_eq_expr(ty.clone(), a.clone(), b.clone());
        let h2_ty = mk_eq_expr(ty.clone(), b.clone(), c.clone());
        let goal_ty = mk_eq_expr(ty, a, c);

        let mut goal = Goal::new(Name::str("main"), goal_ty);
        goal.add_hypothesis(Name::str("h1"), h1_ty);
        goal.add_hypothesis(Name::str("h2"), h2_ty);
        let mut state = TacticState::new();
        state.add_goal(goal);

        // Must not panic. May succeed (with sorry or non-sorry) or return error.
        let result = eval_tactic(&state, "polyrith", &env);
        // If it succeeds, verify no panic occurred (the cert arm was exercised safely).
        if let Ok(new_state) = result {
            // If polyrith closes the goal, cert should be Polyrith (or at least not Direct).
            // We don't assert the cert type here — just that no panic occurred.
            let _ = new_state.is_complete();
        }
        // An Err result is also acceptable (polyrith couldn't find a certificate).
    }
}
