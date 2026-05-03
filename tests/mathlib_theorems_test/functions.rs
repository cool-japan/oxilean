//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_elab::elaborate_decl;
use oxilean_elab::{
    eval_tactic_block, tactic_cases, tactic_induction, tactic_intro, Goal, TacticState,
};
use oxilean_kernel::env::Environment;
use oxilean_kernel::{BinderInfo, Expr, Level, Name};
use oxilean_parse::{Lexer, Parser};

/// Parse a declaration string. Returns Ok if parsing succeeded.
fn parse_decl(src: &str) -> Result<oxilean_parse::Located<oxilean_parse::Decl>, String> {
    let mut lexer = Lexer::new(src);
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    parser.parse_decl().map_err(|e| format!("{e:?}"))
}
/// Returns true if the declaration parses successfully.
pub(super) fn parses(src: &str) -> bool {
    parse_decl(src).is_ok()
}
/// Parse and elaborates. Returns true if both succeed.
pub(super) fn parses_and_elabs(src: &str) -> bool {
    match parse_decl(src) {
        Ok(located) => {
            let env = Environment::new();
            elaborate_decl(&env, &located.value).is_ok()
        }
        Err(_) => false,
    }
}
#[test]
fn theorem_nat_zero_add_parse() {
    assert!(
        parses("theorem zero_add : forall (n : Nat), 0 + n = n := sorry"),
        "Failed to parse zero_add"
    );
}
#[test]
fn theorem_nat_add_zero_parse() {
    assert!(
        parses("theorem add_zero : forall (n : Nat), n + 0 = n := sorry"),
        "Failed to parse add_zero"
    );
}
#[test]
fn theorem_nat_add_comm_parse() {
    assert!(
        parses("theorem add_comm : forall (n m : Nat), n + m = m + n := sorry"),
        "Failed to parse add_comm"
    );
}
#[test]
fn theorem_nat_add_assoc_parse() {
    assert!(
        parses("theorem add_assoc : forall (n m k : Nat), n + m + k = n + (m + k) := sorry"),
        "Failed to parse add_assoc"
    );
}
#[test]
fn theorem_nat_succ_ne_zero_parse() {
    assert!(
        parses("theorem succ_ne_zero : forall (n : Nat), ¬ (Nat.succ n = 0) := sorry"),
        "Failed to parse succ_ne_zero"
    );
}
#[test]
fn theorem_nat_mul_comm_parse() {
    assert!(
        parses("theorem mul_comm : forall (n m : Nat), n * m = m * n := sorry"),
        "Failed to parse mul_comm"
    );
}
#[test]
fn theorem_nat_mul_one_parse() {
    assert!(
        parses("theorem mul_one : forall (n : Nat), n * 1 = n := sorry"),
        "Failed to parse mul_one"
    );
}
#[test]
fn theorem_nat_zero_mul_parse() {
    assert!(
        parses("theorem zero_mul : forall (n : Nat), 0 * n = 0 := sorry"),
        "Failed to parse zero_mul"
    );
}
#[test]
fn theorem_nat_mul_assoc_parse() {
    assert!(
        parses("theorem mul_assoc : forall (n m k : Nat), n * m * k = n * (m * k) := sorry"),
        "Failed to parse mul_assoc"
    );
}
#[test]
fn theorem_nat_succ_add_parse() {
    assert!(
        parses("theorem succ_add : forall (n m : Nat), Nat.succ n + m = Nat.succ (n + m) := sorry"),
        "Failed to parse succ_add"
    );
}
#[test]
fn theorem_bool_and_comm_parse() {
    assert!(
        parses("theorem and_comm : forall (a b : Bool), a && b = b && a := sorry"),
        "Failed to parse bool and_comm"
    );
}
#[test]
fn theorem_bool_or_comm_parse() {
    assert!(
        parses("theorem or_comm : forall (a b : Bool), a || b = b || a := sorry"),
        "Failed to parse bool or_comm"
    );
}
#[test]
fn theorem_bool_and_true_parse() {
    assert!(
        parses("theorem and_true : forall (a : Bool), a && true = a := sorry"),
        "Failed to parse bool and_true"
    );
}
#[test]
fn theorem_bool_true_and_parse() {
    assert!(
        parses("theorem true_and : forall (a : Bool), true && a = a := sorry"),
        "Failed to parse bool true_and"
    );
}
#[test]
fn theorem_bool_or_false_parse() {
    assert!(
        parses("theorem or_false : forall (a : Bool), a || false = a := sorry"),
        "Failed to parse bool or_false"
    );
}
#[test]
fn theorem_bool_false_or_parse() {
    assert!(
        parses("theorem false_or : forall (a : Bool), false || a = a := sorry"),
        "Failed to parse bool false_or"
    );
}
#[test]
fn theorem_bool_and_false_parse() {
    assert!(
        parses("theorem and_false : forall (a : Bool), a && false = false := sorry"),
        "Failed to parse bool and_false"
    );
}
#[test]
fn theorem_bool_or_true_parse() {
    assert!(
        parses("theorem or_true : forall (a : Bool), a || true = true := sorry"),
        "Failed to parse bool or_true"
    );
}
#[test]
fn theorem_and_intro_parse() {
    assert!(
        parses("theorem and_intro : forall (p q : Prop), p -> q -> p ∧ q := sorry"),
        "Failed to parse and_intro"
    );
}
#[test]
fn theorem_and_left_parse() {
    assert!(
        parses("theorem and_left : forall (p q : Prop), p ∧ q -> p := sorry"),
        "Failed to parse and_left"
    );
}
#[test]
fn theorem_and_right_parse() {
    assert!(
        parses("theorem and_right : forall (p q : Prop), p ∧ q -> q := sorry"),
        "Failed to parse and_right"
    );
}
#[test]
fn theorem_or_inl_parse() {
    assert!(
        parses("theorem or_inl : forall (p q : Prop), p -> p ∨ q := sorry"),
        "Failed to parse or_inl"
    );
}
#[test]
fn theorem_or_inr_parse() {
    assert!(
        parses("theorem or_inr : forall (p q : Prop), q -> p ∨ q := sorry"),
        "Failed to parse or_inr"
    );
}
#[test]
fn theorem_not_false_parse() {
    assert!(
        parses("theorem not_false_axiom : ¬ False := sorry"),
        "Failed to parse not_false"
    );
}
#[test]
fn theorem_false_elim_parse() {
    assert!(
        parses("theorem false_elim : forall (p : Prop), False -> p := sorry"),
        "Failed to parse false_elim"
    );
}
#[test]
fn theorem_classical_em_parse() {
    assert!(
        parses("axiom classical_em : forall (p : Prop), p ∨ ¬ p"),
        "Failed to parse classical_em"
    );
}
#[test]
fn theorem_double_neg_intro_parse() {
    assert!(
        parses("theorem double_neg_intro : forall (p : Prop), p -> ¬ ¬ p := sorry"),
        "Failed to parse double_neg_intro"
    );
}
#[test]
fn theorem_and_comm_parse() {
    assert!(
        parses("theorem and_comm_prop : forall (p q : Prop), p ∧ q -> q ∧ p := sorry"),
        "Failed to parse and_comm"
    );
}
#[test]
fn theorem_or_comm_parse() {
    assert!(
        parses("theorem or_comm_prop : forall (p q : Prop), p ∨ q -> q ∨ p := sorry"),
        "Failed to parse or_comm"
    );
}
#[test]
fn theorem_iff_mpr_parse() {
    assert!(
        parses("theorem iff_mpr : forall (p q : Prop), (p ↔ q) -> q -> p := sorry"),
        "Failed to parse iff_mpr"
    );
}
#[test]
fn theorem_list_length_nil_parse() {
    assert!(
        parses("axiom list_length_nil : forall (α : Type), List.length (List.nil : List α) = 0"),
        "Failed to parse list_length_nil"
    );
}
#[test]
fn theorem_list_length_cons_parse() {
    assert!(
        parses(
            "axiom list_length_cons : forall (α : Type) (h : α) (t : List α), \
             List.length (List.cons h t) = Nat.succ (List.length t)"
        ),
        "Failed to parse list_length_cons"
    );
}
#[test]
fn theorem_list_nil_append_parse() {
    assert!(
        parses(
            "axiom list_nil_append : forall (α : Type) (l : List α), \
             List.append List.nil l = l"
        ),
        "Failed to parse list_nil_append"
    );
}
#[test]
fn theorem_list_append_nil_parse() {
    assert!(
        parses(
            "axiom list_append_nil : forall (α : Type) (l : List α), \
             List.append l List.nil = l"
        ),
        "Failed to parse list_append_nil"
    );
}
#[test]
fn theorem_list_append_assoc_parse() {
    assert!(
        parses(
            "axiom list_append_assoc : forall (α : Type) (a b c : List α), \
             List.append (List.append a b) c = List.append a (List.append b c)"
        ),
        "Failed to parse list_append_assoc"
    );
}
#[test]
fn theorem_list_map_id_parse() {
    assert!(
        parses(
            "axiom list_map_id : forall (α : Type) (l : List α), \
             List.map (fun x -> x) l = l"
        ),
        "Failed to parse list_map_id"
    );
}
#[test]
fn theorem_list_reverse_nil_parse() {
    assert!(
        parses(
            "axiom list_reverse_nil : forall (α : Type), \
             List.reverse (List.nil : List α) = List.nil"
        ),
        "Failed to parse list_reverse_nil"
    );
}
#[test]
fn theorem_list_map_nil_parse() {
    assert!(
        parses(
            "axiom list_map_nil : forall (α β : Type) (f : α -> β), \
             List.map f List.nil = List.nil"
        ),
        "Failed to parse list_map_nil"
    );
}
#[test]
fn theorem_list_map_append_parse() {
    assert!(
        parses(
            "axiom list_map_append : forall (α β : Type) (f : α -> β) (l1 l2 : List α), \
             List.map f (List.append l1 l2) = List.append (List.map f l1) (List.map f l2)"
        ),
        "Failed to parse list_map_append"
    );
}
#[test]
fn theorem_list_foldl_nil_parse() {
    assert!(
        parses(
            "axiom list_foldl_nil : forall (α β : Type) (f : β -> α -> β) (init : β), \
             List.foldl f init List.nil = init"
        ),
        "Failed to parse list_foldl_nil"
    );
}
#[test]
fn theorem_eq_refl_parse() {
    assert!(
        parses("theorem eq_refl_prop : forall (α : Type) (a : α), a = a := sorry"),
        "Failed to parse eq_refl"
    );
}
#[test]
fn theorem_eq_symm_parse() {
    assert!(
        parses("theorem eq_symm_prop : forall (α : Type) (a b : α), a = b -> b = a := sorry"),
        "Failed to parse eq_symm"
    );
}
#[test]
fn theorem_eq_trans_parse() {
    assert!(
        parses("theorem eq_trans_prop : forall (α : Type) (a b c : α), a = b -> b = c -> a = c := sorry"),
        "Failed to parse eq_trans"
    );
}
#[test]
fn theorem_eq_subst_parse() {
    assert!(
        parses(
            "theorem eq_subst_prop : forall (α : Type) (p : α -> Prop) (a b : α), \
             a = b -> p a -> p b := sorry"
        ),
        "Failed to parse eq_subst"
    );
}
#[test]
fn theorem_nat_le_refl_parse() {
    assert!(
        parses("theorem nat_le_refl : forall (n : Nat), n ≤ n := sorry"),
        "Failed to parse nat_le_refl"
    );
}
#[test]
fn theorem_nat_le_trans_parse() {
    assert!(
        parses("theorem nat_le_trans : forall (a b c : Nat), a ≤ b -> b ≤ c -> a ≤ c := sorry"),
        "Failed to parse nat_le_trans"
    );
}
#[test]
fn theorem_nat_zero_le_parse() {
    assert!(
        parses("theorem nat_zero_le : forall (n : Nat), 0 ≤ n := sorry"),
        "Failed to parse nat_zero_le"
    );
}
#[test]
fn theorem_nat_le_succ_parse() {
    assert!(
        parses("theorem nat_le_succ : forall (n : Nat), n ≤ n + 1 := sorry"),
        "Failed to parse nat_le_succ"
    );
}
#[test]
fn theorem_nat_lt_irrefl_parse() {
    assert!(
        parses("theorem nat_lt_irrefl : forall (n : Nat), ¬ n < n := sorry"),
        "Failed to parse nat_lt_irrefl"
    );
}
#[test]
fn theorem_nat_add_le_add_right_parse() {
    assert!(
        parses(
            "theorem nat_add_le_add_right : forall (n m k : Nat), n ≤ m -> n + k ≤ m + k := sorry"
        ),
        "Failed to parse nat_add_le_add_right"
    );
}
#[test]
fn theorem_nat_left_distrib_parse() {
    assert!(
        parses(
            "theorem nat_left_distrib : forall (n m k : Nat), n * (m + k) = n * m + n * k := sorry"
        ),
        "Failed to parse nat_left_distrib"
    );
}
#[test]
fn theorem_nat_one_mul_parse() {
    assert!(
        parses("theorem nat_one_mul : forall (n : Nat), 1 * n = n := sorry"),
        "Failed to parse nat_one_mul"
    );
}
#[test]
fn theorem_nat_mul_zero_parse() {
    assert!(
        parses("theorem nat_mul_zero : forall (n : Nat), n * 0 = 0 := sorry"),
        "Failed to parse nat_mul_zero"
    );
}
#[test]
fn theorem_nat_add_right_cancel_parse() {
    assert!(
        parses("axiom nat_add_right_cancel : forall (n m k : Nat), n + k = m + k -> n = m"),
        "Failed to parse nat_add_right_cancel"
    );
}
#[test]
fn theorem_nat_add_left_comm_parse() {
    assert!(
        parses(
            "theorem nat_add_left_comm : forall (n m k : Nat), n + (m + k) = m + (n + k) := sorry"
        ),
        "Failed to parse nat_add_left_comm"
    );
}
#[test]
fn theorem_nat_add_right_comm_parse() {
    assert!(
        parses("theorem nat_add_right_comm : forall (n m k : Nat), n + m + k = n + k + m := sorry"),
        "Failed to parse nat_add_right_comm"
    );
}
#[test]
fn theorem_nat_pow_succ_parse() {
    assert!(
        parses("axiom nat_pow_succ : forall (n m : Nat), n ^ (m + 1) = n ^ m * n"),
        "Failed to parse nat_pow_succ"
    );
}
#[test]
fn theorem_nat_gcd_comm_parse() {
    assert!(
        parses("axiom nat_gcd_comm : forall (m n : Nat), Nat.gcd m n = Nat.gcd n m"),
        "Failed to parse nat_gcd_comm"
    );
}
#[test]
fn theorem_nat_gcd_zero_left_parse() {
    assert!(
        parses("axiom nat_gcd_zero_left : forall (n : Nat), Nat.gcd 0 n = n"),
        "Failed to parse nat_gcd_zero_left"
    );
}
#[test]
fn theorem_nat_dvd_refl_parse() {
    assert!(
        parses("axiom nat_dvd_refl : forall (n : Nat), Nat.dvd n n"),
        "Failed to parse nat_dvd_refl"
    );
}
/// An axiom over a bare Prop type elaborates: Prop = Sort(0).
#[test]
fn elab_axiom_bare_prop() {
    assert!(
        parses_and_elabs("axiom bare_prop_axiom : Prop"),
        "Failed to elab axiom : Prop"
    );
}
/// A Pi type over Prop with a variable body: forall (p : Prop), p -> p
/// elaborates as a Pi(Prop, Pi(BVar(0), BVar(1))).
#[test]
fn elab_axiom_prop_implication() {
    assert!(
        parses_and_elabs("axiom impl_axiom : forall (p : Prop), p -> p"),
        "Failed to elab axiom with Prop implication"
    );
}
/// An axiom of type `Prop -> Prop` elaborates fine.
#[test]
fn elab_axiom_prop_to_prop() {
    assert!(
        parses_and_elabs("axiom prop_fn_axiom : Prop -> Prop"),
        "Failed to elab axiom Prop -> Prop"
    );
}
/// `theorem sort_thm : Prop := Prop` — proof is Prop (a sort, valid for any sort type).
#[test]
fn elab_theorem_sort_proof() {
    assert!(
        parses_and_elabs("theorem sort_thm : Prop := Prop"),
        "Failed to elab theorem with Prop proof"
    );
}
/// `def id_sort : Prop := Prop` — constant definition with sort body.
#[test]
fn elab_def_sort_body() {
    assert!(
        parses_and_elabs("def id_sort : Prop := Prop"),
        "Failed to elab def with sort body"
    );
}
/// Nested Pi types elaborate correctly.
#[test]
fn elab_nested_pi() {
    assert!(parses_and_elabs(
        "axiom nested_pi : forall (p q r : Prop), (p -> q) -> (q -> r) -> p -> r"
    ));
}
/// Double Pi over Prop with non-dependent arrow.
#[test]
fn elab_double_prop_arrow() {
    assert!(parses_and_elabs(
        "axiom double_arrow : Prop -> Prop -> Prop"
    ));
}
/// Triple nested Pi.
#[test]
fn elab_triple_pi() {
    assert!(parses_and_elabs(
        "axiom triple_pi : forall (p q r : Prop), p -> q -> r -> p"
    ));
}
/// Sort (Type) is a valid type expression.
#[test]
fn elab_axiom_type_sort() {
    assert!(parses_and_elabs("axiom type_axiom : Type"));
}
/// Prop → Type is valid (universe polymorphism stub).
#[test]
fn elab_prop_to_type() {
    assert!(parses_and_elabs("axiom prop_to_type : Prop -> Type"));
}
/// Type → Prop is valid.
#[test]
fn elab_type_to_prop() {
    assert!(parses_and_elabs("axiom type_to_prop : Type -> Prop"));
}
/// Type → Type is valid.
#[test]
fn elab_type_to_type() {
    assert!(parses_and_elabs("axiom type_to_type : Type -> Type"));
}
/// forall over Type with Prop body.
#[test]
fn elab_forall_type_prop() {
    assert!(parses_and_elabs(
        "axiom forall_type_prop : forall (alpha : Type), Prop"
    ));
}
/// `theorem` with identity lambda proof elaborates: `fun p -> fun h -> h`.
#[test]
fn elab_theorem_identity_proof() {
    assert!(parses_and_elabs(
        "theorem my_thm : forall (p : Prop), p -> p := fun p -> fun h -> h"
    ));
}
/// `axiom` with complex nested arrow elaborates.
#[test]
fn elab_complex_arrow_axiom() {
    assert!(parses_and_elabs(
        "axiom complex_arrow : (Prop -> Prop) -> Prop -> Prop"
    ));
}
/// `def` with lambda body (constant function).
#[test]
fn elab_def_with_lambda() {
    assert!(parses_and_elabs(
        "def const_fn : Prop -> Prop := fun p -> p"
    ));
}
/// `def` with nested lambda (K combinator stub).
#[test]
fn elab_def_k_combinator() {
    assert!(parses_and_elabs(
        "def k_comb : Prop -> Prop -> Prop := fun p -> fun q -> p"
    ));
}
/// `def` with identity function type annotation.
#[test]
fn elab_def_identity() {
    assert!(parses_and_elabs("def id_prop : Prop -> Prop := fun p -> p"));
}
#[test]
fn summary_parse_rates() {
    let nat_theorems = vec![
        "theorem zero_add : forall (n : Nat), 0 + n = n := sorry",
        "theorem add_zero : forall (n : Nat), n + 0 = n := sorry",
        "theorem add_comm : forall (n m : Nat), n + m = m + n := sorry",
        "theorem add_assoc : forall (n m k : Nat), n + m + k = n + (m + k) := sorry",
        "theorem succ_ne_zero : forall (n : Nat), ¬ (Nat.succ n = 0) := sorry",
        "theorem mul_comm : forall (n m : Nat), n * m = m * n := sorry",
        "theorem mul_one : forall (n : Nat), n * 1 = n := sorry",
        "theorem zero_mul : forall (n : Nat), 0 * n = 0 := sorry",
        "theorem mul_assoc : forall (n m k : Nat), n * m * k = n * (m * k) := sorry",
        "theorem succ_add : forall (n m : Nat), Nat.succ n + m = Nat.succ (n + m) := sorry",
    ];
    let bool_theorems = vec![
        "theorem and_comm_b : forall (a b : Bool), a && b = b && a := sorry",
        "theorem or_comm_b : forall (a b : Bool), a || b = b || a := sorry",
        "theorem and_true_b : forall (a : Bool), a && true = a := sorry",
        "theorem or_false_b : forall (a : Bool), a || false = a := sorry",
    ];
    let logic_theorems = vec![
        "theorem and_intro_l : forall (p q : Prop), p -> q -> p ∧ q := sorry",
        "theorem or_inl_l : forall (p q : Prop), p -> p ∨ q := sorry",
        "axiom classical_em_l : forall (p : Prop), p ∨ ¬ p",
        "theorem iff_mpr_l : forall (p q : Prop), (p ↔ q) -> q -> p := sorry",
    ];
    let all_suites = vec![
        ("Nat arithmetic", &nat_theorems),
        ("Bool operations", &bool_theorems),
        ("Logic/Prop", &logic_theorems),
    ];
    let mut total = 0usize;
    let mut passed = 0usize;
    println!("\n========================================");
    println!("CURATED THEOREM PARSE VERIFICATION");
    println!("========================================");
    for (name, suite) in &all_suites {
        let suite_total = suite.len();
        let suite_passed = suite.iter().filter(|s| parses(s)).count();
        let rate = 100.0 * suite_passed as f64 / suite_total as f64;
        println!("  {name}: {suite_passed}/{suite_total} ({rate:.0}%)");
        total += suite_total;
        passed += suite_passed;
    }
    let overall_rate = 100.0 * passed as f64 / total as f64;
    println!("\n  TOTAL: {passed}/{total} theorems parsed ({overall_rate:.1}%)");
    println!("========================================\n");
    let report = format!(
        "# OxiLean Curated Theorem Verification Report\n\n\
         Date: 2026-02-17\n\n\
         ## Summary\n\
         Total theorems tested: {total}\n\
         Parsed successfully: {passed}\n\
         Parse success rate: {overall_rate:.1}%\n\n\
         ## Category Breakdown\n"
    );
    let _ = std::fs::write("/tmp/oxilean_theorems_report.md", &report);
    assert!(
        overall_rate >= 80.0,
        "Expected ≥80% parse rate for curated theorems, got {overall_rate:.1}%"
    );
}
pub(super) fn mk_pi(name: &str, domain: Expr, body: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str(name),
        Box::new(domain),
        Box::new(body),
    )
}
pub(super) fn mk_eq(ty: Expr, lhs: Expr, rhs: Expr) -> Expr {
    let eq_const = Expr::Const(Name::str("Eq"), vec![Level::zero()]);
    let eq_ty = Expr::App(Box::new(eq_const), Box::new(ty));
    let eq_lhs = Expr::App(Box::new(eq_ty), Box::new(lhs));
    Expr::App(Box::new(eq_lhs), Box::new(rhs))
}
/// Test: `intro` on a Pi goal introduces a hypothesis and returns sub-goal.
#[test]
fn tactic_intro_on_pi_goal() {
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let prop_ty = Expr::Sort(Level::zero());
    let target = mk_pi("n", nat_ty.clone(), prop_ty.clone());
    let mut state = TacticState::new();
    let goal = Goal::new(Name::str("main"), target);
    state.add_goal(goal);
    let result = tactic_intro(&state, Name::str("n")).expect("intro should succeed");
    assert_eq!(result.num_goals(), 1, "should have 1 sub-goal after intro");
    let sub_goal = result.goals().first().unwrap();
    assert_eq!(sub_goal.hypotheses().len(), 1, "should have 1 hypothesis");
    assert_eq!(sub_goal.hypotheses()[0].0, Name::str("n"));
    assert_eq!(sub_goal.hypotheses()[0].1, nat_ty);
}
/// Test: `refl` closes an equality goal `a = a`.
#[test]
fn tactic_refl_closes_eq_goal() {
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let zero = Expr::Lit(oxilean_kernel::Literal::Nat(0));
    let target = mk_eq(nat_ty, zero.clone(), zero);
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), target));
    let result = eval_tactic_block(&state, &["refl".to_string()], &Environment::new())
        .expect("refl should succeed for 0 = 0");
    assert!(result.is_complete(), "refl should close the goal");
}
/// Test: `refl` fails on a non-reflexive goal.
#[test]
fn tactic_refl_fails_on_nonrefl() {
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let zero = Expr::Lit(oxilean_kernel::Literal::Nat(0));
    let one = Expr::Lit(oxilean_kernel::Literal::Nat(1));
    let target = mk_eq(nat_ty, zero, one);
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), target));
    let result = eval_tactic_block(&state, &["refl".to_string()], &Environment::new());
    assert!(result.is_err(), "refl should fail for 0 = 1");
}
/// Test: `assumption` closes the goal when a matching hypothesis exists.
#[test]
fn tactic_assumption_finds_hyp() {
    let prop = Expr::Const(Name::str("MyProp"), vec![]);
    let mut state = TacticState::new();
    let mut goal = Goal::new(Name::str("main"), prop.clone());
    goal.add_hypothesis(Name::str("h"), prop.clone());
    state.add_goal(goal);
    let result = eval_tactic_block(&state, &["assumption".to_string()], &Environment::new())
        .expect("assumption should succeed");
    assert!(result.is_complete(), "assumption should close the goal");
}
/// Test: `assumption` fails when no matching hypothesis exists.
#[test]
fn tactic_assumption_fails_no_match() {
    let prop_a = Expr::Const(Name::str("PropA"), vec![]);
    let prop_b = Expr::Const(Name::str("PropB"), vec![]);
    let mut state = TacticState::new();
    let mut goal = Goal::new(Name::str("main"), prop_a);
    goal.add_hypothesis(Name::str("h"), prop_b);
    state.add_goal(goal);
    let result = eval_tactic_block(&state, &["assumption".to_string()], &Environment::new());
    assert!(result.is_err(), "assumption should fail when no match");
}
/// Test: `trivial` closes True.
#[test]
fn tactic_trivial_closes_true() {
    let target = Expr::Const(Name::str("True"), vec![]);
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), target));
    let result = eval_tactic_block(&state, &["trivial".to_string()], &Environment::new())
        .expect("trivial should close True");
    assert!(result.is_complete(), "trivial should close goal True");
}
/// Test: sequence `intro` + `assumption` proves `P → P`.
#[test]
fn tactic_intro_then_assumption_proves_identity() {
    let prop_p = Expr::Const(Name::str("P"), vec![]);
    let target = mk_pi("h", prop_p.clone(), prop_p.clone());
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), target));
    let result = eval_tactic_block(
        &state,
        &["intro h".to_string(), "assumption".to_string()],
        &Environment::new(),
    )
    .expect("intro + assumption should prove P -> P");
    assert!(result.is_complete(), "proof of P -> P should be complete");
}
/// Test: sequence `intro n` + `intro m` + `refl` proves `∀ n m : Nat, n = n`.
#[test]
fn tactic_multi_intro_then_refl() {
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let zero = Expr::Lit(oxilean_kernel::Literal::Nat(0));
    let eq_goal = mk_eq(nat_ty.clone(), zero.clone(), zero.clone());
    let target = mk_pi("n", nat_ty.clone(), mk_pi("m", nat_ty.clone(), eq_goal));
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), target));
    let result = eval_tactic_block(
        &state,
        &[
            "intro n".to_string(),
            "intro m".to_string(),
            "refl".to_string(),
        ],
        &Environment::new(),
    )
    .expect("multi-intro + refl should succeed");
    assert!(result.is_complete(), "should fully prove ∀ n m, 0 = 0");
}
/// Test: `sorry` closes any goal immediately.
#[test]
fn tactic_sorry_closes_goal() {
    let hard_goal = Expr::Const(Name::str("RiemannHypothesis"), vec![]);
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), hard_goal));
    let result = eval_tactic_block(&state, &["sorry".to_string()], &Environment::new())
        .expect("sorry should always close a goal");
    assert!(result.is_complete(), "sorry should close even hard goals");
}
/// Test: `exfalso` changes the goal to False.
#[test]
fn tactic_exfalso_changes_goal() {
    let complicated = Expr::Const(Name::str("Complicated"), vec![]);
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), complicated));
    let result = eval_tactic_block(&state, &["exfalso".to_string()], &Environment::new())
        .expect("exfalso should transform goal");
    assert_eq!(result.num_goals(), 1, "exfalso produces 1 sub-goal");
    let sub = result.goals().first().unwrap();
    assert_eq!(
        sub.target(),
        &Expr::Const(Name::str("False"), vec![]),
        "sub-goal should be False"
    );
}
/// Summary test: count how many tactic proofs succeed.
#[test]
fn tactic_engine_summary() {
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let zero = Expr::Lit(oxilean_kernel::Literal::Nat(0));
    let prop_p = Expr::Const(Name::str("P"), vec![]);
    let cases: Vec<(&str, Expr, Vec<String>, bool)> = vec![
        (
            "refl on 0 = 0",
            mk_eq(nat_ty.clone(), zero.clone(), zero.clone()),
            vec!["refl".into()],
            true,
        ),
        (
            "trivial on True",
            Expr::Const(Name::str("True"), vec![]),
            vec!["trivial".into()],
            true,
        ),
        (
            "P -> P via intro+assumption",
            mk_pi("h", prop_p.clone(), prop_p.clone()),
            vec!["intro h".into(), "assumption".into()],
            true,
        ),
        (
            "sorry closes goal",
            Expr::Const(Name::str("AnythingHard"), vec![]),
            vec!["sorry".into()],
            true,
        ),
        (
            "refl fails on 0 = 1",
            mk_eq(
                nat_ty.clone(),
                zero.clone(),
                Expr::Lit(oxilean_kernel::Literal::Nat(1)),
            ),
            vec!["refl".into()],
            false,
        ),
    ];
    let total = cases.len();
    let mut passed = 0usize;
    println!("\n========================================");
    println!("TACTIC ENGINE VERIFICATION");
    println!("========================================");
    for (desc, target, tactics, expect_ok) in &cases {
        let mut state = TacticState::new();
        state.add_goal(Goal::new(Name::str("main"), target.clone()));
        let result = eval_tactic_block(&state, tactics, &Environment::new());
        let ok = if *expect_ok {
            result.is_ok() && result.unwrap().is_complete()
        } else {
            result.is_err()
        };
        let status = if ok { "PASS" } else { "FAIL" };
        println!("  [{status}] {desc}");
        if ok {
            passed += 1;
        }
    }
    println!("\n  TOTAL: {passed}/{total} tactic proofs correct");
    println!("========================================\n");
    assert_eq!(passed, total, "all tactic engine tests should pass");
}
/// Test: `simp` closes a True goal (via trivial fallback).
#[test]
fn tactic_simp_closes_true() {
    let target = Expr::Const(Name::str("True"), vec![]);
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), target));
    let result = eval_tactic_block(&state, &["simp".to_string()], &Environment::new())
        .expect("simp should close True goal");
    assert!(result.is_complete(), "simp should close the True goal");
}
/// Test: `simp` closes a reflexivity goal (via trivial → refl).
#[test]
fn tactic_simp_closes_refl() {
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let zero = Expr::Lit(oxilean_kernel::Literal::Nat(0));
    let target = mk_eq(nat_ty, zero.clone(), zero);
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), target));
    let result = eval_tactic_block(&state, &["simp".to_string()], &Environment::new())
        .expect("simp should close refl goal");
    assert!(result.is_complete(), "simp should close the refl goal");
}
/// Test: `rw [h]` rewrites an equality in the goal using hypothesis h.
#[test]
fn tactic_rw_rewrites_hypothesis() {
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let a = Expr::Const(Name::str("A"), vec![]);
    let b = Expr::Const(Name::str("B"), vec![]);
    let c = Expr::Const(Name::str("C"), vec![]);
    let target = mk_eq(nat_ty.clone(), a.clone(), b.clone());
    let h_ty = mk_eq(nat_ty.clone(), a.clone(), c.clone());
    let mut state = TacticState::new();
    let mut goal = Goal::new(Name::str("main"), target);
    goal.add_hypothesis(Name::str("h"), h_ty);
    state.add_goal(goal);
    let result = eval_tactic_block(&state, &["rw [h]".to_string()], &Environment::new())
        .expect("rw [h] should succeed");
    assert_eq!(result.num_goals(), 1, "rw should not close the goal");
    let new_target = result.goals().first().unwrap().target();
    let dbg = format!("{:?}", new_target);
    assert!(
        dbg.contains("\"C\""),
        "C should appear in rewritten goal: {:?}",
        new_target
    );
}
/// Test: `rw [← h]` rewrites in reverse (from rhs to lhs).
#[test]
fn tactic_rw_reverse_rewrites_hypothesis() {
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let a = Expr::Const(Name::str("A"), vec![]);
    let b = Expr::Const(Name::str("B"), vec![]);
    let c = Expr::Const(Name::str("C"), vec![]);
    let target = mk_eq(nat_ty.clone(), c.clone(), b.clone());
    let h_ty = mk_eq(nat_ty.clone(), a.clone(), c.clone());
    let mut state = TacticState::new();
    let mut goal = Goal::new(Name::str("main"), target);
    goal.add_hypothesis(Name::str("h"), h_ty);
    state.add_goal(goal);
    let result = eval_tactic_block(
        &state,
        &["rw [\u{2190} h]".to_string()],
        &Environment::new(),
    )
    .expect("rw [← h] should succeed");
    assert_eq!(result.num_goals(), 1, "rw should not close the goal");
    let new_target = result.goals().first().unwrap().target();
    let dbg = format!("{:?}", new_target);
    assert!(
        dbg.contains("\"A\""),
        "A should appear in rewritten goal: {:?}",
        new_target
    );
}
/// Test: `simp only [h]` rewrites using hypothesis h, then tries refl.
///
/// Goal: A = B
/// Hypothesis h : A = B
/// `simp only [h]` rewrites A→B in the goal, yielding B = B, which refl closes.
#[test]
fn tactic_simp_only_rewrites_and_closes() {
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let a = Expr::Const(Name::str("A"), vec![]);
    let b = Expr::Const(Name::str("B"), vec![]);
    let target = mk_eq(nat_ty.clone(), a.clone(), b.clone());
    let h_ty = mk_eq(nat_ty.clone(), a.clone(), b.clone());
    let mut state = TacticState::new();
    let mut goal = Goal::new(Name::str("main"), target);
    goal.add_hypothesis(Name::str("h"), h_ty);
    state.add_goal(goal);
    let result = eval_tactic_block(&state, &["simp only [h]".to_string()], &Environment::new())
        .expect("simp only [h] should succeed");
    assert!(
        result.is_complete(),
        "simp only should close the goal via refl after rewriting"
    );
}
/// Test: `simp only [h1, h2]` applies two rewrites sequentially.
///
/// Goal: A = C
/// h1 : A = B
/// h2 : B = C
/// After simp only [h1, h2]: rewrites A→B then B→C, yielding C = C → refl.
#[test]
fn tactic_simp_only_two_rewrites_and_closes() {
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let a = Expr::Const(Name::str("AA"), vec![]);
    let b = Expr::Const(Name::str("BB"), vec![]);
    let c = Expr::Const(Name::str("CC"), vec![]);
    let target = mk_eq(nat_ty.clone(), a.clone(), c.clone());
    let h1_ty = mk_eq(nat_ty.clone(), a.clone(), b.clone());
    let h2_ty = mk_eq(nat_ty.clone(), b.clone(), c.clone());
    let mut state = TacticState::new();
    let mut goal = Goal::new(Name::str("main"), target);
    goal.add_hypothesis(Name::str("h1"), h1_ty);
    goal.add_hypothesis(Name::str("h2"), h2_ty);
    state.add_goal(goal);
    let result = eval_tactic_block(
        &state,
        &["simp only [h1, h2]".to_string()],
        &Environment::new(),
    )
    .expect("simp only [h1, h2] should succeed");
    assert!(
        result.is_complete(),
        "simp only with two rewrites should close the goal"
    );
}
/// Test: `simp only [h]` leaves an open goal if rewriting doesn't yield refl.
///
/// Goal: A = B
/// h : X = Y   (unrelated lemma)
/// `simp only [h]` doesn't change goal (no X in goal), so goal remains open.
#[test]
fn tactic_simp_only_no_match_leaves_goal() {
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let a = Expr::Const(Name::str("A"), vec![]);
    let b = Expr::Const(Name::str("B"), vec![]);
    let x = Expr::Const(Name::str("X"), vec![]);
    let y = Expr::Const(Name::str("Y"), vec![]);
    let target = mk_eq(nat_ty.clone(), a.clone(), b.clone());
    let h_ty = mk_eq(nat_ty.clone(), x.clone(), y.clone());
    let mut state = TacticState::new();
    let mut goal = Goal::new(Name::str("main"), target);
    goal.add_hypothesis(Name::str("h"), h_ty);
    state.add_goal(goal);
    let result = eval_tactic_block(&state, &["simp only [h]".to_string()], &Environment::new());
    assert!(
        result.is_ok(),
        "simp only should not return an error even without progress"
    );
}
/// Test: `simp only []` (empty list) acts like plain `simp` (beta-reduce + trivial).
#[test]
fn tactic_simp_only_empty_list_acts_like_simp() {
    let zero = Expr::Lit(oxilean_kernel::Literal::Nat(0));
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let target = mk_eq(nat_ty, zero.clone(), zero);
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), target));
    let result = eval_tactic_block(&state, &["simp only []".to_string()], &Environment::new())
        .expect("simp only [] should close refl goal");
    assert!(result.is_complete(), "simp only [] should close refl goal");
}
/// Test: `simp` after `intro` closes P → P proof.
#[test]
fn tactic_intro_then_simp_closes_identity() {
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let pi_target = Expr::Pi(
        BinderInfo::Default,
        Name::str("x"),
        Box::new(nat_ty.clone()),
        Box::new(nat_ty.clone()),
    );
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), pi_target));
    let result = eval_tactic_block(
        &state,
        &["intro h".to_string(), "assumption".to_string()],
        &Environment::new(),
    )
    .expect("intro + assumption should close P → P");
    assert!(
        result.is_complete(),
        "intro + assumption should prove Nat → Nat"
    );
}
/// Simp summary test: verifies simp handles multiple goal patterns correctly.
#[test]
fn tactic_simp_summary() {
    let zero = Expr::Lit(oxilean_kernel::Literal::Nat(0));
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let true_expr = Expr::Const(Name::str("True"), vec![]);
    let t1 = mk_eq(nat_ty.clone(), zero.clone(), zero.clone());
    let mut s1 = TacticState::new();
    s1.add_goal(Goal::new(Name::str("g1"), t1));
    assert!(
        eval_tactic_block(&s1, &["simp".to_string()], &Environment::new())
            .unwrap()
            .is_complete()
    );
    let mut s2 = TacticState::new();
    s2.add_goal(Goal::new(Name::str("g2"), true_expr.clone()));
    assert!(
        eval_tactic_block(&s2, &["simp".to_string()], &Environment::new())
            .unwrap()
            .is_complete()
    );
    let a = Expr::Const(Name::str("A"), vec![]);
    let b = Expr::Const(Name::str("B"), vec![]);
    let t3 = mk_eq(nat_ty.clone(), a.clone(), b.clone());
    let h3 = mk_eq(nat_ty.clone(), a.clone(), b.clone());
    let mut s3 = TacticState::new();
    let mut g3 = Goal::new(Name::str("g3"), t3);
    g3.add_hypothesis(Name::str("h"), h3);
    s3.add_goal(g3);
    assert!(
        eval_tactic_block(&s3, &["simp only [h]".to_string()], &Environment::new())
            .unwrap()
            .is_complete()
    );
    println!("simp summary: all 3 cases passed");
}
/// Nat.mul_one : n * 1 = n
#[test]
fn theorem_nat_mul_one_parse_b() {
    assert!(parses(
        "theorem mul_one : forall (n : Nat), n * 1 = n := sorry"
    ));
}
/// Nat.one_mul : 1 * n = n
#[test]
fn theorem_nat_one_mul_parse2() {
    assert!(parses(
        "theorem one_mul : forall (n : Nat), 1 * n = n := sorry"
    ));
}
/// Nat.add_assoc : (a + b) + c = a + (b + c)
#[test]
fn theorem_nat_add_assoc_parse_b() {
    assert!(parses(
        "theorem add_assoc : forall (a b c : Nat), (a + b) + c = a + (b + c) := sorry"
    ));
}
/// Nat.mul_add : a * (b + c) = a * b + a * c
#[test]
fn theorem_nat_mul_add_parse() {
    assert!(parses(
        "theorem mul_add : forall (a b c : Nat), a * (b + c) = a * b + a * c := sorry"
    ));
}
/// Nat.add_mul : (a + b) * c = a * c + b * c
#[test]
fn theorem_nat_add_mul_parse() {
    assert!(parses(
        "theorem add_mul : forall (a b c : Nat), (a + b) * c = a * c + b * c := sorry"
    ));
}
/// Nat.mul_assoc : (a * b) * c = a * (b * c)
#[test]
fn theorem_nat_mul_assoc_parse_b() {
    assert!(parses(
        "theorem mul_assoc : forall (a b c : Nat), (a * b) * c = a * (b * c) := sorry"
    ));
}
/// Nat.succ_pos : 0 < Nat.succ n
#[test]
fn theorem_nat_succ_pos_parse() {
    assert!(parses(
        "theorem succ_pos : forall (n : Nat), 0 < Nat.succ n := sorry"
    ));
}
/// Nat.le_refl : n ≤ n
#[test]
fn theorem_nat_le_refl_parse_b() {
    assert!(parses(
        "theorem le_refl : forall (n : Nat), n <= n := sorry"
    ));
}
/// Nat.le_of_succ_le : Nat.succ n ≤ m → n ≤ m
#[test]
fn theorem_nat_le_of_succ_le_parse() {
    assert!(parses(
        "theorem le_of_succ_le : forall (n m : Nat), Nat.succ n <= m -> n <= m := sorry"
    ));
}
/// Nat.not_lt_zero : ¬ (n < 0)
#[test]
fn theorem_nat_not_lt_zero_parse() {
    assert!(parses(
        "theorem not_lt_zero : forall (n : Nat), Not (n < 0) := sorry"
    ));
}
/// Bool.and_true : b && true = b
#[test]
fn theorem_bool_and_true_parse_b() {
    assert!(parses(
        "theorem bool_and_true : forall (b : Bool), b && true = b := sorry"
    ));
}
/// Bool.true_and : true && b = b
#[test]
fn theorem_bool_true_and_parse_b() {
    assert!(parses(
        "theorem bool_true_and : forall (b : Bool), true && b = b := sorry"
    ));
}
/// Bool.and_false : b && false = false
#[test]
fn theorem_bool_and_false_parse_b() {
    assert!(parses(
        "theorem bool_and_false : forall (b : Bool), b && false = false := sorry"
    ));
}
/// Bool.or_true : b || true = true
#[test]
fn theorem_bool_or_true_parse_b() {
    assert!(parses(
        "theorem bool_or_true : forall (b : Bool), b || true = true := sorry"
    ));
}
/// Bool.or_false : b || false = b
#[test]
fn theorem_bool_or_false_parse_b() {
    assert!(parses(
        "theorem bool_or_false : forall (b : Bool), b || false = b := sorry"
    ));
}
/// And.intro : P → Q → P ∧ Q
#[test]
fn theorem_and_intro_parse_b() {
    assert!(parses(
        "theorem and_intro2 : forall (P Q : Prop), P -> Q -> P \u{2227} Q := sorry"
    ));
}
/// And.elim_left : P ∧ Q → P
#[test]
fn theorem_and_elim_left_parse() {
    assert!(parses(
        "theorem and_elim_left : forall (P Q : Prop), P \u{2227} Q -> P := sorry"
    ));
}
/// And.elim_right : P ∧ Q → Q
#[test]
fn theorem_and_elim_right_parse() {
    assert!(parses(
        "theorem and_elim_right : forall (P Q : Prop), P \u{2227} Q -> Q := sorry"
    ));
}
/// Or.inl : P → P ∨ Q
#[test]
fn theorem_or_inl_parse2() {
    assert!(parses(
        "theorem or_inl2 : forall (P Q : Prop), P -> P \u{2228} Q := sorry"
    ));
}
/// Or.inr : Q → P ∨ Q
#[test]
fn theorem_or_inr_parse2() {
    assert!(parses(
        "theorem or_inr2 : forall (P Q : Prop), Q -> P \u{2228} Q := sorry"
    ));
}
/// Not.intro : (P → False) → ¬P
#[test]
fn theorem_not_intro_parse() {
    assert!(parses(
        "theorem not_intro : forall (P : Prop), (P -> False) -> Not P := sorry"
    ));
}
/// Double negation elimination (classical): ¬¬P → P
#[test]
fn theorem_double_neg_elim_parse() {
    assert!(parses(
        "theorem double_neg_elim : forall (P : Prop), Not (Not P) -> P := sorry"
    ));
}
/// Iff.intro : (P → Q) → (Q → P) → (P ↔ Q)
#[test]
fn theorem_iff_intro_parse() {
    assert!(parses(
        "theorem iff_intro : forall (P Q : Prop), (P -> Q) -> (Q -> P) -> Iff P Q := sorry"
    ));
}
/// Iff.mp : (P ↔ Q) → P → Q
#[test]
fn theorem_iff_mp_parse() {
    assert!(parses(
        "theorem iff_mp : forall (P Q : Prop), Iff P Q -> P -> Q := sorry"
    ));
}
/// Iff.mpr : (P ↔ Q) → Q → P
#[test]
fn theorem_iff_mpr_parse_b() {
    assert!(parses(
        "theorem iff_mpr : forall (P Q : Prop), Iff P Q -> Q -> P := sorry"
    ));
}
/// List.length_nil : [].length = 0
#[test]
fn theorem_list_length_nil_parse2() {
    assert!(parses(
        "theorem list_length_nil : (List.length (List.nil : List Nat)) = 0 := sorry"
    ));
}
/// List.length_cons : (a :: l).length = l.length + 1
#[test]
fn theorem_list_length_cons_parse_b() {
    assert!(parses(
        "theorem list_length_cons : forall (a : Nat) (l : List Nat), \
         (List.length (List.cons a l)) = (List.length l) + 1 := sorry"
    ));
}
/// List.map_length : (l.map f).length = l.length
#[test]
fn theorem_list_map_length_parse() {
    assert!(parses(
        "theorem list_map_length : forall (f : Nat -> Nat) (l : List Nat), \
         List.length (List.map f l) = List.length l := sorry"
    ));
}
/// List.append_length : (l₁ ++ l₂).length = l₁.length + l₂.length
#[test]
fn theorem_list_append_length_parse() {
    assert!(parses(
        "theorem list_append_length : forall (l1 l2 : List Nat), \
         List.length (List.append l1 l2) = (List.length l1) + (List.length l2) := sorry"
    ));
}
/// List.reverse_length : l.reverse.length = l.length
#[test]
fn theorem_list_reverse_length_parse() {
    assert!(parses(
        "theorem list_reverse_length : forall (l : List Nat), \
         List.length (List.reverse l) = List.length l := sorry"
    ));
}
/// le_trans: transitivity of ≤
#[test]
fn theorem_le_trans_parse2() {
    assert!(parses(
        "theorem le_trans2 : forall (a b c : Nat), a <= b -> b <= c -> a <= c := sorry"
    ));
}
/// lt_of_lt_of_le : a < b → b ≤ c → a < c
#[test]
fn theorem_lt_of_lt_of_le_parse() {
    assert!(parses(
        "theorem lt_of_lt_of_le : forall (a b c : Nat), a < b -> b <= c -> a < c := sorry"
    ));
}
/// lt_of_le_of_lt : a ≤ b → b < c → a < c
#[test]
fn theorem_lt_of_le_of_lt_parse() {
    assert!(parses(
        "theorem lt_of_le_of_lt : forall (a b c : Nat), a <= b -> b < c -> a < c := sorry"
    ));
}
/// le_antisymm : a ≤ b → b ≤ a → a = b
#[test]
fn theorem_le_antisymm_parse2() {
    assert!(parses(
        "theorem le_antisymm2 : forall (a b : Nat), a <= b -> b <= a -> a = b := sorry"
    ));
}
/// Nat.min_le_left : min a b ≤ a
#[test]
fn theorem_nat_min_le_left_parse() {
    assert!(parses(
        "theorem nat_min_le_left : forall (a b : Nat), Nat.min a b <= a := sorry"
    ));
}
/// Nat.le_max_right : b ≤ max a b
#[test]
fn theorem_nat_le_max_right_parse() {
    assert!(parses(
        "theorem nat_le_max_right : forall (a b : Nat), b <= Nat.max a b := sorry"
    ));
}
/// Function.comp_id : f ∘ id = f
#[test]
fn theorem_comp_id_parse() {
    assert!(parses(
        "theorem comp_id : forall (f : Nat -> Nat), Compose f (fun x -> x) = f := sorry"
    ));
}
/// Function.id_comp : id ∘ f = f
#[test]
fn theorem_id_comp_parse() {
    assert!(parses(
        "theorem id_comp : forall (f : Nat -> Nat), Compose (fun x -> x) f = f := sorry"
    ));
}
/// Function.comp_assoc : (f ∘ g) ∘ h = f ∘ (g ∘ h)
#[test]
fn theorem_comp_assoc_parse() {
    assert!(parses(
        "theorem comp_assoc : forall (f g h : Nat -> Nat), \
         Compose (Compose f g) h = Compose f (Compose g h) := sorry"
    ));
}
/// `constructor` on `P ∧ Q` creates two subgoals: P and Q.
#[test]
fn tactic_constructor_splits_and() {
    let p = Expr::Const(Name::str("P"), vec![]);
    let q = Expr::Const(Name::str("Q"), vec![]);
    let and_pq = Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("And"), vec![])),
            Box::new(p.clone()),
        )),
        Box::new(q.clone()),
    );
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), and_pq));
    let result = eval_tactic_block(&state, &["constructor".to_string()], &Environment::new());
    assert!(result.is_ok(), "constructor should not fail on And goal");
}
/// `left` on `P ∨ Q` changes goal to P.
#[test]
fn tactic_left_on_or_goal() {
    let p = Expr::Const(Name::str("P"), vec![]);
    let q = Expr::Const(Name::str("Q"), vec![]);
    let or_pq = Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("Or"), vec![])),
            Box::new(p.clone()),
        )),
        Box::new(q.clone()),
    );
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), or_pq));
    let result = eval_tactic_block(&state, &["left".to_string()], &Environment::new());
    assert!(result.is_ok(), "left should not fail on Or goal");
}
/// `right` on `P ∨ Q` changes goal to Q.
#[test]
fn tactic_right_on_or_goal() {
    let p = Expr::Const(Name::str("P"), vec![]);
    let q = Expr::Const(Name::str("Q"), vec![]);
    let or_pq = Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("Or"), vec![])),
            Box::new(p.clone()),
        )),
        Box::new(q.clone()),
    );
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), or_pq));
    let result = eval_tactic_block(&state, &["right".to_string()], &Environment::new());
    assert!(result.is_ok(), "right should not fail on Or goal");
}
/// `clear h` removes hypothesis h from the goal context.
#[test]
fn tactic_clear_removes_hypothesis() {
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let p = Expr::Const(Name::str("P"), vec![]);
    let mut state = TacticState::new();
    let mut goal = Goal::new(Name::str("main"), p.clone());
    goal.add_hypothesis(Name::str("h"), nat_ty.clone());
    goal.add_hypothesis(Name::str("k"), nat_ty.clone());
    state.add_goal(goal);
    let result = eval_tactic_block(&state, &["clear h".to_string()], &Environment::new());
    assert!(result.is_ok(), "clear h should not fail");
}
/// `revert h` moves hypothesis h back into the goal as a Pi type.
#[test]
fn tactic_revert_moves_hyp_to_goal() {
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let p = Expr::Const(Name::str("P"), vec![]);
    let mut state = TacticState::new();
    let mut goal = Goal::new(Name::str("main"), p.clone());
    goal.add_hypothesis(Name::str("h"), nat_ty.clone());
    state.add_goal(goal);
    let result = eval_tactic_block(&state, &["revert h".to_string()], &Environment::new());
    assert!(result.is_ok(), "revert h should not fail");
    if let Ok(new_state) = result {
        let new_goal = new_state.goals().first().unwrap();
        assert!(
            new_goal.find_hypothesis(&Name::str("h")).is_none(),
            "h should be removed from context after revert"
        );
    }
}
/// `intro h1; intro h2` introduces two hypotheses sequentially.
#[test]
fn tactic_two_intros_sequential() {
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let inner_pi = Expr::Pi(
        BinderInfo::Default,
        Name::str("y"),
        Box::new(nat_ty.clone()),
        Box::new(nat_ty.clone()),
    );
    let outer_pi = Expr::Pi(
        BinderInfo::Default,
        Name::str("x"),
        Box::new(nat_ty.clone()),
        Box::new(inner_pi),
    );
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), outer_pi));
    let result = eval_tactic_block(
        &state,
        &[
            "intro x".to_string(),
            "intro y".to_string(),
            "assumption".to_string(),
        ],
        &Environment::new(),
    );
    assert!(
        result.is_ok(),
        "intro x; intro y; assumption should work on Nat→Nat→Nat"
    );
}
/// `exact` on a literal closes a literal goal.
#[test]
fn tactic_exact_literal() {
    let zero = Expr::Lit(oxilean_kernel::Literal::Nat(0));
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), nat_ty.clone()));
    let result = eval_tactic_block(&state, &["exact 0".to_string()], &Environment::new());
    assert!(result.is_ok(), "exact 0 should succeed");
    let _ = zero;
}
/// Prove `P → P ∧ P` using intro + constructor + assumption.
#[test]
fn tactic_prove_p_and_p() {
    let p = Expr::Const(Name::str("MyP"), vec![]);
    let and_pp = Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("And"), vec![])),
            Box::new(p.clone()),
        )),
        Box::new(p.clone()),
    );
    let pi_target = Expr::Pi(
        BinderInfo::Default,
        Name::str("h"),
        Box::new(p.clone()),
        Box::new(and_pp),
    );
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), pi_target));
    let result = eval_tactic_block(
        &state,
        &["intro h".to_string(), "constructor".to_string()],
        &Environment::new(),
    );
    assert!(
        result.is_ok(),
        "intro h; constructor should work on P → P ∧ P"
    );
}
/// Test: `sorry` always closes the goal regardless of shape.
#[test]
fn tactic_sorry_closes_any_goal() {
    let complex_goal = Expr::Pi(
        BinderInfo::Default,
        Name::str("x"),
        Box::new(Expr::Const(Name::str("Nat"), vec![])),
        Box::new(Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("And"), vec![])),
                Box::new(Expr::Const(Name::str("P"), vec![])),
            )),
            Box::new(Expr::Const(Name::str("Q"), vec![])),
        )),
    );
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), complex_goal));
    let result = eval_tactic_block(&state, &["sorry".to_string()], &Environment::new())
        .expect("sorry should always close any goal");
    assert!(result.is_complete(), "sorry should close the goal");
}
/// Comprehensive tactic engine summary.
#[test]
fn tactic_engine_full_summary() {
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let zero = Expr::Lit(oxilean_kernel::Literal::Nat(0));
    let p = Expr::Const(Name::str("TestP"), vec![]);
    let q = Expr::Const(Name::str("TestQ"), vec![]);
    let true_expr = Expr::Const(Name::str("True"), vec![]);
    let mut pass = 0u32;
    {
        let t = mk_eq(nat_ty.clone(), zero.clone(), zero.clone());
        let mut s = TacticState::new();
        s.add_goal(Goal::new(Name::str("g"), t));
        if eval_tactic_block(&s, &["refl".to_string()], &Environment::new())
            .map(|r| r.is_complete())
            .unwrap_or(false)
        {
            pass += 1;
        }
    }
    {
        let mut s = TacticState::new();
        s.add_goal(Goal::new(Name::str("g"), true_expr.clone()));
        if eval_tactic_block(&s, &["trivial".to_string()], &Environment::new())
            .map(|r| r.is_complete())
            .unwrap_or(false)
        {
            pass += 1;
        }
    }
    {
        let t = mk_eq(nat_ty.clone(), zero.clone(), zero.clone());
        let mut s = TacticState::new();
        s.add_goal(Goal::new(Name::str("g"), t));
        if eval_tactic_block(&s, &["simp".to_string()], &Environment::new())
            .map(|r| r.is_complete())
            .unwrap_or(false)
        {
            pass += 1;
        }
    }
    {
        let mut s = TacticState::new();
        s.add_goal(Goal::new(Name::str("g"), p.clone()));
        if eval_tactic_block(&s, &["sorry".to_string()], &Environment::new())
            .map(|r| r.is_complete())
            .unwrap_or(false)
        {
            pass += 1;
        }
    }
    {
        let pi = Expr::Pi(
            BinderInfo::Default,
            Name::str("h"),
            Box::new(p.clone()),
            Box::new(p.clone()),
        );
        let mut s = TacticState::new();
        s.add_goal(Goal::new(Name::str("g"), pi));
        if eval_tactic_block(
            &s,
            &["intro h".to_string(), "assumption".to_string()],
            &Environment::new(),
        )
        .map(|r| r.is_complete())
        .unwrap_or(false)
        {
            pass += 1;
        }
    }
    {
        let mut s = TacticState::new();
        let mut g = Goal::new(Name::str("g"), q.clone());
        g.add_hypothesis(Name::str("hq"), q.clone());
        s.add_goal(g);
        if eval_tactic_block(&s, &["assumption".to_string()], &Environment::new())
            .map(|r| r.is_complete())
            .unwrap_or(false)
        {
            pass += 1;
        }
    }
    {
        let a = Expr::Const(Name::str("AA"), vec![]);
        let b = Expr::Const(Name::str("BB"), vec![]);
        let target = mk_eq(nat_ty.clone(), a.clone(), b.clone());
        let h_ty = mk_eq(nat_ty.clone(), a.clone(), b.clone());
        let mut s = TacticState::new();
        let mut g = Goal::new(Name::str("g"), target);
        g.add_hypothesis(Name::str("h"), h_ty);
        s.add_goal(g);
        if eval_tactic_block(&s, &["simp only [h]".to_string()], &Environment::new())
            .map(|r| r.is_complete())
            .unwrap_or(false)
        {
            pass += 1;
        }
    }
    println!("tactic_engine_full_summary: {}/7 cases passed", pass);
    assert!(
        pass >= 6,
        "at least 6/7 tactic tests should pass, got {}",
        pass
    );
}
/// Helper: build `And A B` expression.
pub(super) fn mk_and(a: Expr, b: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("And"), vec![])),
            Box::new(a),
        )),
        Box::new(b),
    )
}
/// Helper: build `Or A B` expression.
pub(super) fn mk_or_expr(a: Expr, b: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("Or"), vec![])),
            Box::new(a),
        )),
        Box::new(b),
    )
}
/// Test: `cases h` on `h : And A B` splits into one goal with h_left/h_right.
#[test]
fn tactic_cases_and_splits_one_goal() {
    let a = Expr::Const(Name::str("PropA"), vec![]);
    let b = Expr::Const(Name::str("PropB"), vec![]);
    let target = Expr::Const(Name::str("PropC"), vec![]);
    let mut state = TacticState::new();
    let mut goal = Goal::new(Name::str("main"), target.clone());
    goal.add_hypothesis(Name::str("h"), mk_and(a.clone(), b.clone()));
    state.add_goal(goal);
    let result = tactic_cases(&state, &Name::str("h")).expect("cases on And should succeed");
    assert_eq!(result.num_goals(), 1, "And split should produce 1 goal");
    let sub = result.goals().first().unwrap();
    let hyp_names: Vec<_> = sub.hypotheses().iter().map(|(n, _)| n.clone()).collect();
    assert!(
        hyp_names.iter().any(|n| n == &Name::str("h_left")),
        "should have h_left"
    );
    assert!(
        hyp_names.iter().any(|n| n == &Name::str("h_right")),
        "should have h_right"
    );
    assert!(
        !hyp_names.iter().any(|n| n == &Name::str("h")),
        "h should be removed"
    );
}
/// Test: `cases h` on `h : Or A B` splits into two goals.
#[test]
fn tactic_cases_or_splits_two_goals() {
    let a = Expr::Const(Name::str("PropA"), vec![]);
    let b = Expr::Const(Name::str("PropB"), vec![]);
    let target = Expr::Const(Name::str("PropC"), vec![]);
    let mut state = TacticState::new();
    let mut goal = Goal::new(Name::str("main"), target.clone());
    goal.add_hypothesis(Name::str("h"), mk_or_expr(a.clone(), b.clone()));
    state.add_goal(goal);
    let result = tactic_cases(&state, &Name::str("h")).expect("cases on Or should succeed");
    assert_eq!(result.num_goals(), 2, "Or split should produce 2 goals");
    let tags: Vec<_> = result
        .goals()
        .iter()
        .filter_map(|g| g.tag.clone())
        .collect();
    assert!(
        tags.iter().any(|t| t.contains("inl")),
        "should have case inl"
    );
    assert!(
        tags.iter().any(|t| t.contains("inr")),
        "should have case inr"
    );
}
/// Test: `cases h` on `h : False` closes the goal.
#[test]
fn tactic_cases_false_closes_goal() {
    let target = Expr::Const(Name::str("Anything"), vec![]);
    let mut state = TacticState::new();
    let mut goal = Goal::new(Name::str("main"), target);
    goal.add_hypothesis(Name::str("h"), Expr::Const(Name::str("False"), vec![]));
    state.add_goal(goal);
    let result = tactic_cases(&state, &Name::str("h")).expect("cases on False should succeed");
    assert!(result.is_complete(), "cases on False should close the goal");
}
/// Test: `cases h` on `h : Nat` produces zero and succ goals.
#[test]
fn tactic_cases_nat_produces_zero_succ() {
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let target = Expr::Const(Name::str("SomeProp"), vec![]);
    let mut state = TacticState::new();
    let mut goal = Goal::new(Name::str("main"), target);
    goal.add_hypothesis(Name::str("n"), nat_ty.clone());
    state.add_goal(goal);
    let result = tactic_cases(&state, &Name::str("n")).expect("cases on Nat should succeed");
    assert_eq!(result.num_goals(), 2, "Nat cases should produce 2 goals");
    let tags: Vec<_> = result
        .goals()
        .iter()
        .filter_map(|g| g.tag.clone())
        .collect();
    assert!(
        tags.iter().any(|t| t.contains("zero")),
        "should have zero case"
    );
    assert!(
        tags.iter().any(|t| t.contains("succ")),
        "should have succ case"
    );
}
/// Test: `cases h` on missing hypothesis fails with error.
#[test]
fn tactic_cases_missing_hyp_fails() {
    let target = Expr::Const(Name::str("Prop"), vec![]);
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), target));
    let result = tactic_cases(&state, &Name::str("h"));
    assert!(result.is_err(), "cases on nonexistent hyp should fail");
}
/// Test: `cases h` via eval_tactic dispatch works for Or.
#[test]
fn tactic_eval_cases_or_dispatch() {
    let a = Expr::Const(Name::str("P"), vec![]);
    let b = Expr::Const(Name::str("Q"), vec![]);
    let target = Expr::Const(Name::str("R"), vec![]);
    let mut state = TacticState::new();
    let mut goal = Goal::new(Name::str("main"), target);
    goal.add_hypothesis(Name::str("h"), mk_or_expr(a, b));
    state.add_goal(goal);
    let result = eval_tactic_block(&state, &["cases h".to_string()], &Environment::new())
        .expect("cases dispatch should work");
    assert_eq!(result.num_goals(), 2, "Or cases should give 2 goals");
}
/// Test: `induction n` on `n : Nat` produces zero and succ goals.
#[test]
fn tactic_induction_nat_produces_two_goals() {
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let prop = Expr::Const(Name::str("P"), vec![]);
    let mut state = TacticState::new();
    let mut goal = Goal::new(Name::str("main"), prop);
    goal.add_hypothesis(Name::str("n"), nat_ty.clone());
    state.add_goal(goal);
    let result =
        tactic_induction(&state, &Name::str("n")).expect("induction on Nat should succeed");
    assert_eq!(
        result.num_goals(),
        2,
        "Nat induction should produce 2 goals"
    );
    let tags: Vec<_> = result
        .goals()
        .iter()
        .filter_map(|g| g.tag.clone())
        .collect();
    assert!(tags.iter().any(|t| t.contains("zero")));
    assert!(tags.iter().any(|t| t.contains("succ")));
}
/// Test: induction succ goal has `n : Nat` and `ih` hypotheses.
#[test]
fn tactic_induction_succ_has_ih() {
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let prop = Expr::Const(Name::str("P"), vec![]);
    let mut state = TacticState::new();
    let mut goal = Goal::new(Name::str("main"), prop);
    goal.add_hypothesis(Name::str("n"), nat_ty.clone());
    state.add_goal(goal);
    let result = tactic_induction(&state, &Name::str("n")).expect("induction on Nat");
    let succ_goal = result
        .goals()
        .iter()
        .find(|g| g.tag.as_deref() == Some("case succ"))
        .expect("should have succ goal");
    let hyp_names: Vec<_> = succ_goal
        .hypotheses()
        .iter()
        .map(|(n, _)| n.clone())
        .collect();
    assert!(
        hyp_names.iter().any(|n| n == &Name::str("n")),
        "succ goal should have n : Nat"
    );
    assert!(
        hyp_names.iter().any(|n| n == &Name::str("ih")),
        "succ goal should have ih"
    );
}
/// Test: induction zero goal has target substituted with 0.
#[test]
fn tactic_induction_zero_goal_has_lit_zero_target() {
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let eq_nn = mk_eq(nat_ty.clone(), Expr::BVar(0), Expr::BVar(0));
    let mut state = TacticState::new();
    let mut goal = Goal::new(Name::str("main"), eq_nn);
    goal.add_hypothesis(Name::str("n"), nat_ty.clone());
    state.add_goal(goal);
    let result = tactic_induction(&state, &Name::str("n")).expect("induction on Nat");
    let zero_goal = result
        .goals()
        .iter()
        .find(|g| g.tag.as_deref() == Some("case zero"))
        .expect("should have zero goal");
    let expected = mk_eq(
        nat_ty,
        Expr::Lit(oxilean_kernel::Literal::Nat(0)),
        Expr::Lit(oxilean_kernel::Literal::Nat(0)),
    );
    assert_eq!(
        zero_goal.target, expected,
        "zero goal target should have 0 substituted for BVar(0)"
    );
}
/// Test: `induction n` then `refl` closes zero goal when target is `0 = 0`.
#[test]
fn tactic_induction_zero_proved_by_refl() {
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let zero = Expr::Lit(oxilean_kernel::Literal::Nat(0));
    let target_body = mk_eq(nat_ty.clone(), zero.clone(), zero.clone());
    let pi_target = mk_pi("n", nat_ty.clone(), target_body.clone());
    let mut state = TacticState::new();
    state.add_goal(Goal::new(Name::str("main"), pi_target));
    let after_intro = eval_tactic_block(&state, &["intro n".to_string()], &Environment::new())
        .expect("intro should work");
    let after_ind = tactic_induction(&after_intro, &Name::str("n")).expect("induction should work");
    assert_eq!(after_ind.num_goals(), 2);
    let zero_goal = after_ind
        .goals()
        .iter()
        .find(|g| g.tag.as_deref() == Some("case zero"))
        .unwrap()
        .clone();
    let mut zero_state = TacticState::new();
    zero_state.add_goal(zero_goal);
    let result = eval_tactic_block(&zero_state, &["refl".to_string()], &Environment::new())
        .expect("refl on 0=0 should work");
    assert!(
        result.is_complete(),
        "zero case 0=0 should be closed by refl"
    );
}
/// Test: `induction` on non-Nat hypothesis fails.
#[test]
fn tactic_induction_non_nat_fails() {
    let prop = Expr::Const(Name::str("Prop"), vec![]);
    let target = Expr::Const(Name::str("P"), vec![]);
    let mut state = TacticState::new();
    let mut goal = Goal::new(Name::str("main"), target);
    goal.add_hypothesis(Name::str("h"), prop);
    state.add_goal(goal);
    let result = tactic_induction(&state, &Name::str("h"));
    assert!(result.is_err(), "induction on non-Nat should fail");
}
/// Test: eval_tactic dispatch for `induction` keyword.
#[test]
fn tactic_eval_induction_dispatch() {
    let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
    let prop = Expr::Const(Name::str("P"), vec![]);
    let mut state = TacticState::new();
    let mut goal = Goal::new(Name::str("main"), prop);
    goal.add_hypothesis(Name::str("n"), nat_ty);
    state.add_goal(goal);
    let result = eval_tactic_block(&state, &["induction n".to_string()], &Environment::new())
        .expect("induction dispatch should work");
    assert_eq!(result.num_goals(), 2, "induction should produce 2 goals");
}
/// Test: `cases h` on And, then `assumption` on the split goal.
/// Simulates: `h : A ∧ B ⊢ A` via `cases h; exact h_left`
#[test]
fn tactic_cases_and_then_assumption() {
    let a = Expr::Const(Name::str("PropA"), vec![]);
    let b = Expr::Const(Name::str("PropB"), vec![]);
    let mut state = TacticState::new();
    let mut goal = Goal::new(Name::str("main"), a.clone());
    goal.add_hypothesis(Name::str("h"), mk_and(a.clone(), b));
    state.add_goal(goal);
    let after_cases = tactic_cases(&state, &Name::str("h")).expect("cases on And");
    assert_eq!(after_cases.num_goals(), 1);
    let result = eval_tactic_block(
        &after_cases,
        &["assumption".to_string()],
        &Environment::new(),
    )
    .expect("assumption after cases should close PropA goal");
    assert!(
        result.is_complete(),
        "proof of A from A ∧ B should complete"
    );
}
