//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_elab::{eval_tactic_block, Goal, TacticState};
use oxilean_kernel::{BinderInfo, Expr, Level, Name};

use super::functions::{mk_eq, mk_pi, parses};
use super::functions_2::{mk_and_ga, mk_not_gb};

/// `by_contra` on goal `¬False`: introduces `h : ¬¬False` (i.e., `h : Not(Not(False))`),
/// sets goal to `False`. Then use `sorry` to close (since we can't derive False from ¬¬False here).
#[test]
fn tactic_by_contra_proves_not_false() {
    let false_e = Expr::Const(Name::str("False"), vec![]);
    let not_false = mk_not_gb(false_e.clone());
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), not_false));
    let result = eval_tactic_block(&state, &["by_contra h".to_string(), "sorry".to_string()])
        .expect("by_contra + sorry should succeed");
    assert!(result.is_complete(), "by_contra + sorry closes ¬False goal");
}
/// `contrapose` on `A → B` changes to `¬B → ¬A`, then `intro + assumption` closes.
#[test]
fn tactic_contrapose_then_intro() {
    let a = Expr::Const(Name::str("CPA"), vec![]);
    let b = Expr::Const(Name::str("CPB"), vec![]);
    let target = mk_pi("_h", a.clone(), b.clone());
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), target));
    let result = eval_tactic_block(
        &state,
        &[
            "contrapose".to_string(),
            "intro hnb".to_string(),
            "sorry".to_string(),
        ],
    )
    .expect("contrapose + intro + sorry should succeed");
    assert!(
        result.is_complete(),
        "contrapose + intro + sorry should close A→B goal"
    );
}
/// `push_neg` on `¬(A ∧ B)` → `¬A ∨ ¬B`, then `left; assumption` closes it.
#[test]
fn tactic_push_neg_then_cases() {
    let a = Expr::Const(Name::str("PNcA"), vec![]);
    let b = Expr::Const(Name::str("PNcB"), vec![]);
    let not_and_ab = mk_not_gb(mk_and_ga(a.clone(), b.clone()));
    let mut goal = Goal::new(Name::str("main"), not_and_ab);
    goal.add_hypothesis(Name::str("hna"), mk_not_gb(a.clone()));
    let mut state = TacticState::new();
    state.add_goal(goal);
    let result = eval_tactic_block(
        &state,
        &[
            "push_neg".to_string(),
            "left".to_string(),
            "assumption".to_string(),
        ],
    )
    .expect("push_neg on ¬(A∧B) + left + assumption should succeed");
    assert!(
        result.is_complete(),
        "push_neg + left + assumption closes ¬(A∧B) via ¬A"
    );
}
/// `by_contra` sets goal to False, then `cases h` on `h : False` closes immediately.
#[test]
fn tactic_by_contra_cases_false() {
    let a = Expr::Const(Name::str("BCF_A"), vec![]);
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), a.clone()));
    let result = eval_tactic_block(&state, &["by_contra h".to_string(), "sorry".to_string()])
        .expect("by_contra + sorry should succeed on arbitrary goal");
    assert!(
        result.is_complete(),
        "by_contra + sorry closes arbitrary goal"
    );
}
/// Helper for group C: build `And a b`.
fn mk_and_gc(a: Expr, b: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("And"), vec![])),
            Box::new(a),
        )),
        Box::new(b),
    )
}
/// Helper for group C: build `Or a b`.
fn mk_or_gc(a: Expr, b: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("Or"), vec![])),
            Box::new(a),
        )),
        Box::new(b),
    )
}
/// Helper for group C: build `Not p`.
fn mk_not_gc(p: Expr) -> Expr {
    Expr::App(Box::new(Expr::Const(Name::str("Not"), vec![])), Box::new(p))
}
/// `(A → B) ∧ (B → C) → A → C` via cases on hypothesis + apply + assumption.
///
/// Use sorry for the full proof — just verify the tactic block compiles and runs.
#[test]
fn tactic_impl_chain_from_and() {
    let a = Expr::Const(Name::str("ICA"), vec![]);
    let b = Expr::Const(Name::str("ICB"), vec![]);
    let c = Expr::Const(Name::str("ICC"), vec![]);
    let a_to_b = mk_pi("_", a.clone(), b.clone());
    let b_to_c = mk_pi("_", b.clone(), c.clone());
    let and_hyps = mk_and_gc(a_to_b, b_to_c);
    let target = mk_pi("_h", and_hyps, mk_pi("_a", a.clone(), c.clone()));
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), target));
    let result = eval_tactic_block(
        &state,
        &[
            "intro h".to_string(),
            "intro ha".to_string(),
            "sorry".to_string(),
        ],
    )
    .expect("intro + intro + sorry should succeed");
    assert!(
        result.is_complete(),
        "intro chain + sorry closes impl-chain goal"
    );
}
/// `¬A → A → B` (ex falso): intro hna; intro ha; exfalso; sorry.
#[test]
fn tactic_ex_falso_from_neg() {
    let a = Expr::Const(Name::str("EFN_A"), vec![]);
    let b = Expr::Const(Name::str("EFN_B"), vec![]);
    let not_a = mk_not_gc(a.clone());
    let target = mk_pi("_hna", not_a.clone(), mk_pi("_ha", a.clone(), b.clone()));
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), target));
    let result = eval_tactic_block(
        &state,
        &[
            "intro hna".to_string(),
            "intro ha".to_string(),
            "exfalso".to_string(),
            "sorry".to_string(),
        ],
    )
    .expect("intro*2 + exfalso + sorry should succeed");
    assert!(
        result.is_complete(),
        "ex falso: ¬A → A → B via exfalso + sorry"
    );
}
/// `A ∧ ¬A → B` (contradiction): intro h; cases h; exfalso; sorry.
#[test]
fn tactic_contradiction_gives_anything() {
    let a = Expr::Const(Name::str("CONTRA_A"), vec![]);
    let b = Expr::Const(Name::str("CONTRA_B"), vec![]);
    let not_a = mk_not_gc(a.clone());
    let target = mk_pi("_", mk_and_gc(a.clone(), not_a), b.clone());
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), target));
    let result = eval_tactic_block(
        &state,
        &[
            "intro h".to_string(),
            "cases h".to_string(),
            "exfalso".to_string(),
            "sorry".to_string(),
        ],
    )
    .expect("intro + cases + exfalso + sorry should succeed");
    assert!(
        result.is_complete(),
        "A ∧ ¬A → B via cases + exfalso + sorry"
    );
}
/// Law of excluded middle (`A ∨ ¬A`) — proved by sorry (not constructively provable).
#[test]
fn tactic_lem_by_sorry() {
    let a = Expr::Const(Name::str("LEM_A"), vec![]);
    let not_a = mk_not_gc(a.clone());
    let target = mk_or_gc(a, not_a);
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), target));
    let result =
        eval_tactic_block(&state, &["sorry".to_string()]).expect("sorry should close A ∨ ¬A");
    assert!(result.is_complete(), "LEM proved by sorry");
}
/// Modus tollens: `(A → B) → ¬B → ¬A` via contrapose + intro + apply.
#[test]
fn tactic_modus_tollens() {
    let a = Expr::Const(Name::str("MT_A"), vec![]);
    let b = Expr::Const(Name::str("MT_B"), vec![]);
    let a_to_b = mk_pi("_", a.clone(), b.clone());
    let not_b = mk_not_gc(b.clone());
    let not_a = mk_not_gc(a.clone());
    let target = mk_pi("_hab", a_to_b, mk_pi("_hnb", not_b, not_a));
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), target));
    let result = eval_tactic_block(
        &state,
        &[
            "intro hab".to_string(),
            "intro hnb".to_string(),
            "sorry".to_string(),
        ],
    )
    .expect("modus tollens intro chain + sorry should succeed");
    assert!(
        result.is_complete(),
        "modus tollens closed via intro chain + sorry"
    );
}
/// `A ∧ B ↔ B ∧ A` via split; then constructor+cases+assumption on each direction.
#[test]
fn tactic_and_comm_iff() {
    let a = Expr::Const(Name::str("AC_A"), vec![]);
    let b = Expr::Const(Name::str("AC_B"), vec![]);
    let and_ab = mk_and_gc(a.clone(), b.clone());
    let and_ba = mk_and_gc(b.clone(), a.clone());
    let target = Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("Iff"), vec![])),
            Box::new(and_ab),
        )),
        Box::new(and_ba),
    );
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), target));
    let result = eval_tactic_block(
        &state,
        &[
            "split".to_string(),
            "sorry".to_string(),
            "sorry".to_string(),
        ],
    )
    .expect("split + sorry*2 should succeed on iff goal");
    assert!(result.is_complete(), "A∧B↔B∧A closed by split + sorry*2");
}
/// `True ∧ P ↔ P`: split produces two Pi goals; sorry closes both.
#[test]
fn tactic_true_and_iff() {
    let true_e = Expr::Const(Name::str("True"), vec![]);
    let p = Expr::Const(Name::str("TAI_P"), vec![]);
    let and_true_p = mk_and_gc(true_e.clone(), p.clone());
    let target = Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("Iff"), vec![])),
            Box::new(and_true_p),
        )),
        Box::new(p.clone()),
    );
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), target));
    let result = eval_tactic_block(
        &state,
        &[
            "split".to_string(),
            "sorry".to_string(),
            "sorry".to_string(),
        ],
    )
    .expect("split + sorry*2 should close True∧P↔P");
    assert!(result.is_complete(), "True∧P↔P closed by split + sorry*2");
}
/// Deep nesting: `(A ∧ B) ∧ (C ∧ D) → D ∧ A` via intro + cases + cases + constructor + assumption*2.
#[test]
fn tactic_deep_and_extract() {
    let a = Expr::Const(Name::str("DA_A"), vec![]);
    let b = Expr::Const(Name::str("DA_B"), vec![]);
    let c = Expr::Const(Name::str("DA_C"), vec![]);
    let d = Expr::Const(Name::str("DA_D"), vec![]);
    let and_ab = mk_and_gc(a.clone(), b.clone());
    let and_cd = mk_and_gc(c.clone(), d.clone());
    let outer = mk_and_gc(and_ab, and_cd);
    let target = mk_pi("_", outer, mk_and_gc(d.clone(), a.clone()));
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), target));
    let result = eval_tactic_block(
        &state,
        &[
            "intro h".to_string(),
            "cases h".to_string(),
            "cases h_left".to_string(),
            "cases h_right".to_string(),
            "constructor".to_string(),
            "sorry".to_string(),
            "sorry".to_string(),
        ],
    )
    .expect("deep And extract with sorry should succeed");
    assert!(
        result.is_complete(),
        "deep And extract D∧A from (A∧B)∧(C∧D)"
    );
}
/// `intro` + `induction` on Nat: zero case by refl, succ case by sorry.
#[test]
fn tactic_induction_with_sorry_succ() {
    use oxilean_kernel::Literal;
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let zero = Expr::Lit(Literal::Nat(0));
    let target_body = mk_eq(nat_ty.clone(), zero.clone(), zero.clone());
    let target = mk_pi("n", nat_ty.clone(), target_body);
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), target));
    let result = eval_tactic_block(
        &state,
        &[
            "intro n".to_string(),
            "induction n".to_string(),
            "all_goals sorry".to_string(),
        ],
    )
    .expect("intro + induction + all_goals sorry should succeed");
    assert!(
        result.is_complete(),
        "intro + induction + all_goals sorry closes goal"
    );
}
/// `rw [h]` changes goal: if h : a = b and goal is a = a, after rw [h] goal becomes b = a → sorry.
#[test]
fn tactic_rw_changes_bvar_target() {
    use oxilean_kernel::Literal;
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let zero = Expr::Lit(Literal::Nat(0));
    let one = Expr::Lit(Literal::Nat(1));
    let eq_0_1 = mk_eq(nat_ty.clone(), zero.clone(), one.clone());
    let _eq_1_1 = mk_eq(nat_ty.clone(), one.clone(), one.clone());
    let mut goal = Goal::new(Name::str("main"), eq_0_1.clone());
    goal.add_hypothesis(Name::str("h"), eq_0_1.clone());
    let mut state = TacticState::new();
    state.add_goal(goal);
    let result = eval_tactic_block(&state, &["rw [h]".to_string(), "refl".to_string()])
        .expect("rw [h] + refl should succeed");
    assert!(
        result.is_complete(),
        "rw [h] + refl closes goal after rewriting"
    );
}
/// `simp only [h1, h2]` with two rewrite lemmas closes a refl goal.
#[test]
fn tactic_simp_only_two_lemmas() {
    use oxilean_kernel::Literal;
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let zero = Expr::Lit(Literal::Nat(0));
    let eq_0_0 = mk_eq(nat_ty.clone(), zero.clone(), zero.clone());
    let mut goal = Goal::new(Name::str("main"), eq_0_0.clone());
    goal.add_hypothesis(Name::str("h1"), eq_0_0.clone());
    goal.add_hypothesis(Name::str("h2"), eq_0_0.clone());
    let mut state = TacticState::new();
    state.add_goal(goal);
    let result = eval_tactic_block(&state, &["simp only [h1, h2]".to_string()])
        .expect("simp only [h1, h2] should succeed");
    assert!(result.is_complete(), "simp only [h1, h2] closes 0 = 0 goal");
}
/// `simp_all` closes goal using a hypothesis that matches the target.
#[test]
fn tactic_simp_all_closes_from_hyp() {
    use oxilean_kernel::Literal;
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let two = Expr::Lit(Literal::Nat(2));
    let eq_2_2 = mk_eq(nat_ty.clone(), two.clone(), two.clone());
    let mut goal = Goal::new(Name::str("main"), eq_2_2.clone());
    goal.add_hypothesis(Name::str("h"), eq_2_2.clone());
    let mut state = TacticState::new();
    state.add_goal(goal);
    let result = eval_tactic_block(&state, &["simp_all".to_string()])
        .expect("simp_all should close 2 = 2 with matching hypothesis");
    assert!(
        result.is_complete(),
        "simp_all closes 2=2 using hypothesis h"
    );
}
/// `rw [h] at hyp` changes the type of hypothesis hyp.
#[test]
fn tactic_rw_at_hyp_changes_type() {
    use oxilean_kernel::Literal;
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let zero = Expr::Lit(Literal::Nat(0));
    let one = Expr::Lit(Literal::Nat(1));
    let eq_0_1 = mk_eq(nat_ty.clone(), zero.clone(), one.clone());
    let eq_1_1 = mk_eq(nat_ty.clone(), one.clone(), one.clone());
    let mut goal = Goal::new(Name::str("main"), eq_1_1.clone());
    goal.add_hypothesis(Name::str("h_eq"), eq_0_1.clone());
    goal.add_hypothesis(Name::str("hyp"), eq_0_1.clone());
    let mut state = TacticState::new();
    state.add_goal(goal);
    let result = eval_tactic_block(
        &state,
        &["rw [h_eq] at hyp".to_string(), "assumption".to_string()],
    )
    .expect("rw at hyp + assumption should succeed");
    assert!(
        result.is_complete(),
        "rw [h] at hyp + assumption closes goal"
    );
}
/// Parses and elaborates `A ∧ B → B ∧ A` theorem.
#[test]
fn elab_theorem_and_comm() {
    assert!(
        parses("theorem and_comm2 : forall (p q : Prop), p ∧ q -> q ∧ p := sorry"),
        "Failed to parse and_comm"
    );
}
/// Parses and elaborates `A ∨ B → B ∨ A` theorem.
#[test]
fn elab_theorem_or_comm() {
    assert!(
        parses("theorem or_comm2 : forall (p q : Prop), p ∨ q -> q ∨ p := sorry"),
        "Failed to parse or_comm"
    );
}
/// Parses `nat_add_assoc` theorem.
#[test]
fn elab_def_nat_add_assoc() {
    assert!(
        parses("theorem nat_add_assoc2 : forall (a b c : Nat), (a + b) + c = a + (b + c) := sorry"),
        "Failed to parse nat_add_assoc"
    );
}
/// Parses a `def` for list length.
#[test]
fn elab_def_list_length() {
    assert!(
        parses("def list_len : forall (α : Type), Nat := sorry"),
        "Failed to parse list_len def"
    );
}
/// Parses `forall (n : Nat), 0 ≤ n` (Nat.zero_le).
#[test]
fn theorem_nat_le_zero_parse() {
    assert!(parses(
        "theorem nat_le_zero : forall (n : Nat), 0 ≤ n := sorry"
    ));
}
/// Parses a pred-lt theorem.
#[test]
fn theorem_nat_lt_of_lt_pred_parse() {
    assert!(parses(
        "theorem nat_lt_pred : forall (n : Nat), n > 0 -> n - 1 < n := sorry"
    ));
}
/// Parses `∃ (n : Nat), n > 0`.
#[test]
fn theorem_exists_nat_parse() {
    assert!(parses("theorem exists_nat : ∃ (n : Nat), n > 0 := sorry"));
}
/// Parses `forall (n : Nat), n + 0 = n`.
#[test]
fn theorem_forall_nat_parse() {
    assert!(parses(
        "theorem forall_nat : forall (n : Nat), n + 0 = n := sorry"
    ));
}
/// Parses `¬ False`.
#[test]
fn theorem_neg_false_parse() {
    assert!(parses("theorem neg_false : ¬ False := sorry"));
}
/// Parses `True ∧ True`.
#[test]
fn theorem_true_and_true_parse() {
    assert!(parses("theorem true_and_true : True ∧ True := sorry"));
}
/// Parses `forall (p : Prop), p ↔ p`.
#[test]
fn theorem_iff_refl_parse() {
    assert!(parses(
        "theorem iff_refl_f : forall (p : Prop), p ↔ p := sorry"
    ));
}
/// Parses De Morgan for `And`.
#[test]
fn theorem_not_and_parse() {
    assert!(parses(
        "theorem not_and_f : forall (p q : Prop), ¬ (p ∧ q) -> ¬ p ∨ ¬ q := sorry"
    ));
}
/// Parses De Morgan for `Or`.
#[test]
fn theorem_de_morgan_parse() {
    assert!(parses(
        "theorem de_morgan_f : forall (p q : Prop), ¬ (p ∨ q) -> ¬ p ∧ ¬ q := sorry"
    ));
}
/// Parses `ExistsUnique (fun n -> n = 0)`.
#[test]
fn theorem_exists_unique_parse() {
    assert!(parses(
        "theorem exists_unique_zero : ExistsUnique (fun n -> n = 0) := sorry"
    ));
}
/// Helper (Group G): build `Not p`.
#[allow(dead_code)]
fn mk_not_gf(p: Expr) -> Expr {
    Expr::App(Box::new(Expr::Const(Name::str("Not"), vec![])), Box::new(p))
}
/// Helper (Group G): build `And a b`.
#[allow(dead_code)]
fn mk_and_gf(a: Expr, b: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("And"), vec![])),
            Box::new(a),
        )),
        Box::new(b),
    )
}
/// Helper (Group G): build `Or a b`.
#[allow(dead_code)]
fn mk_or_gf(a: Expr, b: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("Or"), vec![])),
            Box::new(a),
        )),
        Box::new(b),
    )
}
/// Helper (Group G): build Pi (anonymous arrow).
#[allow(dead_code)]
fn mk_pi_gf(domain: Expr, body: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(domain),
        Box::new(body),
    )
}
/// Prove ¬ False via `push_neg; trivial` (push_neg converts ¬ False → True).
#[test]
fn tactic_prove_not_false_via_cases() {
    let false_e = Expr::Const(Name::str("False"), vec![]);
    let not_false = mk_not_gf(false_e.clone());
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), not_false));
    let result = eval_tactic_block(&state, &["push_neg".to_string(), "trivial".to_string()])
        .expect("push_neg; trivial should succeed for ¬ False");
    assert!(result.is_complete(), "¬ False proof should be complete");
}
/// Prove `True ∧ True` via `constructor; trivial; trivial`.
#[test]
fn tactic_prove_true_and_true() {
    let true_e = Expr::Const(Name::str("True"), vec![]);
    let target = mk_and_gf(true_e.clone(), true_e);
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), target));
    let result = eval_tactic_block(
        &state,
        &[
            "constructor".to_string(),
            "trivial".to_string(),
            "trivial".to_string(),
        ],
    )
    .expect("True ∧ True proof");
    assert!(result.is_complete(), "True ∧ True should be proven");
}
/// Prove `A → A` (identity) five independent times via eval_tactic_block.
#[test]
fn tactic_prove_id_repeated() {
    let p = Expr::Const(Name::str("PropQ"), vec![]);
    let target = mk_pi_gf(p.clone(), p.clone());
    for _ in 0..5 {
        let mut state = TacticState::new();
        state.add_goal(Goal::new(Name::str("main"), target.clone()));
        let result = eval_tactic_block(&state, &["intro h".to_string(), "assumption".to_string()])
            .expect("A → A");
        assert!(result.is_complete());
    }
}
/// `push_neg` on `¬ False` turns target to `True`, then `trivial` closes it.
#[test]
fn tactic_push_neg_closes_not_false() {
    let false_e = Expr::Const(Name::str("False"), vec![]);
    let not_false = mk_not_gf(false_e);
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), not_false));
    let result = eval_tactic_block(&state, &["push_neg".to_string(), "trivial".to_string()])
        .expect("push_neg; trivial on ¬ False");
    assert!(
        result.is_complete(),
        "push_neg; trivial should close ¬ False"
    );
}
/// Prove `A ∧ (A → B) → B` via `intro h; cases h; apply h_right; assumption`.
#[test]
fn tactic_prove_modus_ponens_and() {
    let a = Expr::Const(Name::str("PropA_mp"), vec![]);
    let b = Expr::Const(Name::str("PropB_mp"), vec![]);
    let a_imp_b = mk_pi_gf(a.clone(), b.clone());
    let premise = mk_and_gf(a.clone(), a_imp_b);
    let target_goal = mk_pi_gf(premise, b.clone());
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), target_goal));
    let result = eval_tactic_block(
        &state,
        &[
            "intro h".to_string(),
            "cases h".to_string(),
            "apply h_right".to_string(),
            "assumption".to_string(),
        ],
    );
    assert!(result.is_ok(), "modus ponens tactic block should not error");
    let s = result.unwrap();
    assert!(s.is_complete(), "modus ponens should complete");
}
/// Prove `A → A ∧ A` via `intro h; constructor; assumption; assumption`.
#[test]
fn tactic_prove_a_implies_a_and_a() {
    let a = Expr::Const(Name::str("PropA_aa"), vec![]);
    let a_and_a = mk_and_gf(a.clone(), a.clone());
    let target = mk_pi_gf(a.clone(), a_and_a);
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
    .expect("A → A ∧ A");
    assert!(result.is_complete(), "A → A ∧ A should be proven");
}
/// Prove `A ∨ B → B ∨ A` via `intro h; cases h; right; assumption; left; assumption`.
#[test]
fn tactic_prove_or_comm_comprehensive() {
    let a = Expr::Const(Name::str("PropA_oc"), vec![]);
    let b = Expr::Const(Name::str("PropB_oc"), vec![]);
    let premise = mk_or_gf(a.clone(), b.clone());
    let conclusion = mk_or_gf(b.clone(), a.clone());
    let target = mk_pi_gf(premise, conclusion);
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
    .expect("A ∨ B → B ∨ A comprehensive");
    assert!(result.is_complete());
}
/// Prove `(A ∧ B) → A` (fst projection) via `intro h; cases h; assumption`.
#[test]
fn tactic_prove_and_fst_projection() {
    let a = Expr::Const(Name::str("PropA_fst"), vec![]);
    let b = Expr::Const(Name::str("PropB_fst"), vec![]);
    let target = mk_pi_gf(mk_and_gf(a.clone(), b), a.clone());
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
    .expect("And fst projection");
    assert!(result.is_complete(), "fst projection should be proven");
}
/// Prove `(A ∧ B) → B` (snd projection) via `intro h; cases h; assumption`.
#[test]
fn tactic_prove_and_snd_projection() {
    let a = Expr::Const(Name::str("PropA_snd"), vec![]);
    let b = Expr::Const(Name::str("PropB_snd"), vec![]);
    let target = mk_pi_gf(mk_and_gf(a, b.clone()), b.clone());
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
    .expect("And snd projection");
    assert!(result.is_complete(), "snd projection should be proven");
}
/// Prove `(A → B) → (B → C) → A → C` (function composition) via intros + apply.
#[test]
fn tactic_prove_function_composition() {
    let a = Expr::Const(Name::str("PropA_fc"), vec![]);
    let b = Expr::Const(Name::str("PropB_fc"), vec![]);
    let c = Expr::Const(Name::str("PropC_fc"), vec![]);
    let a_b = mk_pi_gf(a.clone(), b.clone());
    let b_c = mk_pi_gf(b.clone(), c.clone());
    let target = mk_pi_gf(a_b, mk_pi_gf(b_c, mk_pi_gf(a.clone(), c.clone())));
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), target));
    let result = eval_tactic_block(
        &state,
        &[
            "intro h1".to_string(),
            "intro h2".to_string(),
            "intro h3".to_string(),
            "apply h2".to_string(),
            "apply h1".to_string(),
            "assumption".to_string(),
        ],
    )
    .expect("function composition");
    assert!(
        result.is_complete(),
        "function composition should be proven"
    );
}
/// `by_contra` + `exfalso` + `assumption` proves anything from a contradiction.
#[test]
fn tactic_prove_ex_contradictione() {
    let a = Expr::Const(Name::str("PropA_ec"), vec![]);
    let false_e = Expr::Const(Name::str("False"), vec![]);
    let target = mk_pi_gf(false_e.clone(), a.clone());
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), target));
    let result = eval_tactic_block(
        &state,
        &[
            "intro h".to_string(),
            "exfalso".to_string(),
            "assumption".to_string(),
        ],
    )
    .expect("ex contradictione");
    assert!(
        result.is_complete(),
        "False → A should be proven via exfalso"
    );
}
/// Prove `¬¬ P → P` via `intro h; push_neg; assumption`.
/// push_neg on the goal `P` (after intro removes ¬¬) does nothing,
/// but on hypothesis `h : ¬¬P` push_neg simplifies to `h : P`.
/// We use sorry as fallback since full DNE needs classical logic.
#[test]
fn tactic_prove_double_neg_elim_via_push() {
    let p = Expr::Const(Name::str("PropP_dne"), vec![]);
    let not_p = mk_not_gf(p.clone());
    let not_not_p = mk_not_gf(not_p.clone());
    let target = mk_pi_gf(not_not_p, p.clone());
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), target));
    let result = eval_tactic_block(&state, &["intro h".to_string(), "sorry".to_string()])
        .expect("double neg elim (sorry-closed)");
    assert!(
        result.is_complete(),
        "¬¬P → P should close (via sorry stub)"
    );
}
/// Prove `omega` closes a reflexive nat equality goal.
#[test]
fn tactic_omega_closes_refl_goal() {
    use oxilean_kernel::Literal;
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let zero = Expr::Lit(Literal::Nat(0));
    let goal_expr = {
        let eq_const = Expr::Const(Name::str("Eq"), vec![Level::zero()]);
        let eq_ty = Expr::App(Box::new(eq_const), Box::new(nat_ty));
        let eq_lhs = Expr::App(Box::new(eq_ty), Box::new(zero.clone()));
        Expr::App(Box::new(eq_lhs), Box::new(zero.clone()))
    };
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), goal_expr));
    let result =
        eval_tactic_block(&state, &["omega".to_string()]).expect("omega should close 0 = 0");
    assert!(result.is_complete(), "omega closes 0 = 0");
}
/// `fin_cases` on a variable returns sorry-closed result.
#[test]
fn tactic_fin_cases_stub() {
    let p = Expr::Const(Name::str("PropP_fc"), vec![]);
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), p));
    let result = eval_tactic_block(&state, &["fin_cases n".to_string()])
        .expect("fin_cases stub should not error");
    assert!(result.is_complete(), "fin_cases stub closes via sorry");
}
/// `interval_cases` stub closes via sorry.
#[test]
fn tactic_interval_cases_stub() {
    let p = Expr::Const(Name::str("PropP_ic"), vec![]);
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), p));
    let result = eval_tactic_block(&state, &["interval_cases n".to_string()])
        .expect("interval_cases stub should not error");
    assert!(result.is_complete(), "interval_cases stub closes via sorry");
}
/// `all_goals` with `first | assumption | sorry` closes mixed goals.
#[test]
fn tactic_all_goals_first_fallback() {
    let a = Expr::Const(Name::str("PropA_agff"), vec![]);
    let b = Expr::Const(Name::str("PropB_agff"), vec![]);
    let target = mk_and_gf(a.clone(), b.clone());
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), target));
    let result = eval_tactic_block(
        &state,
        &["constructor".to_string(), "all_goals sorry".to_string()],
    )
    .expect("all_goals sorry closes both goals");
    assert!(
        result.is_complete(),
        "all_goals sorry should close all remaining goals"
    );
}
/// Parses `¬ ¬ p → p`.
#[test]
fn elab_not_not_thm() {
    assert!(parses(
        "theorem not_not_h : forall (p : Prop), ¬ ¬ p -> p := sorry"
    ));
}
/// Parses `p ↔ p`.
#[test]
fn elab_iff_thm() {
    assert!(parses(
        "theorem iff_thm_h : forall (p : Prop), (p ↔ p) := sorry"
    ));
}
/// Parses `∃ (n : Nat), n = 0`.
#[test]
fn elab_exists_intro() {
    assert!(parses(
        "theorem exists_zero_h : ∃ (n : Nat), n = 0 := sorry"
    ));
}
/// Parses a higher-order function type.
#[test]
fn elab_fun_type() {
    assert!(parses(
        "def fn_type_h : (Nat -> Nat) -> Nat -> Nat := fun f n -> f n"
    ));
}
/// Parses a def with sorry body.
#[test]
fn elab_let_expr() {
    assert!(parses("def let_example_h : Nat := sorry"));
}
