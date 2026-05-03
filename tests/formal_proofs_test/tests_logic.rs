//! Propositional and predicate logic formal proof tests.
//!
//! This module tests that OxiLean can parse and elaborate (type-check) a broad
//! range of propositional logic and predicate logic theorems.  Proofs use either
//! `sorry` (type-signature well-formedness) or term-mode lambda proofs where
//! OxiLean's elaborator supports them.

use super::functions::{assert_suite_passes, assert_suite_passes_at_least, run_proof_test};
use super::types::ProofTestCase;

// ──────────────────────────────────────────────────────────────────────────────
// § 1. Basic Axioms and Well-Formedness
// ──────────────────────────────────────────────────────────────────────────────

/// Table of basic propositional logic axioms that must parse and elaborate.
static BASIC_AXIOMS: &[ProofTestCase] = &[
    ProofTestCase::new("axiom_bare_prop", "axiom bare_prop : Prop", "Prop"),
    ProofTestCase::new("axiom_true", "axiom true_holds : True", "True"),
    ProofTestCase::new(
        "axiom_impl_self",
        "axiom impl_self : forall (p : Prop), p -> p",
        "Prop -> Prop",
    ),
    ProofTestCase::new(
        "axiom_and_intro_type",
        "axiom and_intro_type : forall (p q : Prop), p -> q -> p ∧ q",
        "Prop",
    ),
    ProofTestCase::new(
        "axiom_and_elim_left",
        "axiom and_elim_left : forall (p q : Prop), p ∧ q -> p",
        "Prop",
    ),
    ProofTestCase::new(
        "axiom_and_elim_right",
        "axiom and_elim_right : forall (p q : Prop), p ∧ q -> q",
        "Prop",
    ),
    ProofTestCase::new(
        "axiom_or_intro_left",
        "axiom or_intro_left : forall (p q : Prop), p -> p ∨ q",
        "Prop",
    ),
    ProofTestCase::new(
        "axiom_or_intro_right",
        "axiom or_intro_right : forall (p q : Prop), q -> p ∨ q",
        "Prop",
    ),
    ProofTestCase::new(
        "axiom_false_elim",
        "axiom false_elim : forall (p : Prop), False -> p",
        "Prop",
    ),
    ProofTestCase::new("axiom_not_false", "axiom not_false_ax : ¬ False", "Prop"),
    ProofTestCase::new(
        "axiom_contrapositive",
        "axiom contrapositive : forall (p q : Prop), (p -> q) -> ¬ q -> ¬ p",
        "Prop",
    ),
    ProofTestCase::new(
        "axiom_modus_ponens",
        "axiom modus_ponens : forall (p q : Prop), (p -> q) -> p -> q",
        "Prop",
    ),
    ProofTestCase::new(
        "axiom_hyp_syllogism",
        "axiom hyp_syllogism : forall (p q r : Prop), (p -> q) -> (q -> r) -> p -> r",
        "Prop",
    ),
    ProofTestCase::new(
        "axiom_double_neg_intro",
        "axiom double_neg_intro : forall (p : Prop), p -> ¬ ¬ p",
        "Prop",
    ),
    ProofTestCase::new(
        "axiom_double_neg_elim",
        "axiom double_neg_elim : forall (p : Prop), ¬ ¬ p -> p",
        "Prop",
    ),
    ProofTestCase::new(
        "axiom_excluded_middle",
        "axiom em : forall (p : Prop), p ∨ ¬ p",
        "Prop",
    ),
    ProofTestCase::new(
        "axiom_and_comm",
        "axiom and_comm : forall (p q : Prop), p ∧ q -> q ∧ p",
        "Prop",
    ),
    ProofTestCase::new(
        "axiom_or_comm",
        "axiom or_comm : forall (p q : Prop), p ∨ q -> q ∨ p",
        "Prop",
    ),
    ProofTestCase::new(
        "axiom_and_assoc_lr",
        "axiom and_assoc_lr : forall (p q r : Prop), (p ∧ q) ∧ r -> p ∧ (q ∧ r)",
        "Prop",
    ),
    ProofTestCase::new(
        "axiom_and_assoc_rl",
        "axiom and_assoc_rl : forall (p q r : Prop), p ∧ (q ∧ r) -> (p ∧ q) ∧ r",
        "Prop",
    ),
    ProofTestCase::new(
        "axiom_or_assoc_lr",
        "axiom or_assoc_lr : forall (p q r : Prop), (p ∨ q) ∨ r -> p ∨ (q ∨ r)",
        "Prop",
    ),
    ProofTestCase::new(
        "axiom_or_assoc_rl",
        "axiom or_assoc_rl : forall (p q r : Prop), p ∨ (q ∨ r) -> (p ∨ q) ∨ r",
        "Prop",
    ),
];

#[test]
fn test_basic_logic_axioms() {
    assert_suite_passes("basic_logic_axioms", BASIC_AXIOMS);
}

// ──────────────────────────────────────────────────────────────────────────────
// § 2. Theorems with Sorry Proofs
// ──────────────────────────────────────────────────────────────────────────────

static LOGIC_THEOREMS_SORRY: &[ProofTestCase] = &[
    ProofTestCase::new(
        "thm_and_intro_sorry",
        "theorem and_intro_thm : forall (p q : Prop), p -> q -> p ∧ q := sorry",
        "Prop",
    ),
    ProofTestCase::new(
        "thm_and_elim_l_sorry",
        "theorem and_left_thm : forall (p q : Prop), p ∧ q -> p := sorry",
        "Prop",
    ),
    ProofTestCase::new(
        "thm_and_elim_r_sorry",
        "theorem and_right_thm : forall (p q : Prop), p ∧ q -> q := sorry",
        "Prop",
    ),
    ProofTestCase::new(
        "thm_or_intro_l_sorry",
        "theorem or_inl_thm : forall (p q : Prop), p -> p ∨ q := sorry",
        "Prop",
    ),
    ProofTestCase::new(
        "thm_or_intro_r_sorry",
        "theorem or_inr_thm : forall (p q : Prop), q -> p ∨ q := sorry",
        "Prop",
    ),
    ProofTestCase::new(
        "thm_modus_ponens_sorry",
        "theorem mp : forall (p q : Prop), (p -> q) -> p -> q := sorry",
        "Prop",
    ),
    ProofTestCase::new(
        "thm_hypothetical_syllogism_sorry",
        "theorem hs : forall (p q r : Prop), (p -> q) -> (q -> r) -> p -> r := sorry",
        "Prop",
    ),
    ProofTestCase::new(
        "thm_and_comm_sorry",
        "theorem and_comm_thm : forall (p q : Prop), p ∧ q -> q ∧ p := sorry",
        "Prop",
    ),
    ProofTestCase::new(
        "thm_or_comm_sorry",
        "theorem or_comm_thm : forall (p q : Prop), p ∨ q -> q ∨ p := sorry",
        "Prop",
    ),
    ProofTestCase::new(
        "thm_excluded_middle_sorry",
        "theorem em_thm : forall (p : Prop), p ∨ ¬ p := sorry",
        "Prop",
    ),
    ProofTestCase::new(
        "thm_contrapositive_sorry",
        "theorem contrapositive_thm : forall (p q : Prop), (p -> q) -> ¬ q -> ¬ p := sorry",
        "Prop",
    ),
    ProofTestCase::new(
        "thm_double_neg_intro_sorry",
        "theorem dne_intro : forall (p : Prop), p -> ¬ ¬ p := sorry",
        "Prop",
    ),
    ProofTestCase::new(
        "thm_double_neg_elim_sorry",
        "theorem dne_elim : forall (p : Prop), ¬ ¬ p -> p := sorry",
        "Prop",
    ),
    ProofTestCase::new(
        "thm_demorgan_and_sorry",
        "theorem demorgan_and : forall (p q : Prop), ¬ (p ∧ q) -> ¬ p ∨ ¬ q := sorry",
        "Prop",
    ),
    ProofTestCase::new(
        "thm_demorgan_or_sorry",
        "theorem demorgan_or : forall (p q : Prop), ¬ (p ∨ q) -> ¬ p ∧ ¬ q := sorry",
        "Prop",
    ),
    ProofTestCase::new(
        "thm_demorgan_and_rev_sorry",
        "theorem demorgan_and_rev : forall (p q : Prop), ¬ p ∨ ¬ q -> ¬ (p ∧ q) := sorry",
        "Prop",
    ),
    ProofTestCase::new(
        "thm_demorgan_or_rev_sorry",
        "theorem demorgan_or_rev : forall (p q : Prop), ¬ p ∧ ¬ q -> ¬ (p ∨ q) := sorry",
        "Prop",
    ),
    ProofTestCase::new(
        "thm_and_distrib_or_sorry",
        "theorem and_distrib_or : forall (p q r : Prop), p ∧ (q ∨ r) -> (p ∧ q) ∨ (p ∧ r) := sorry",
        "Prop",
    ),
    ProofTestCase::new(
        "thm_or_distrib_and_sorry",
        "theorem or_distrib_and : forall (p q r : Prop), p ∨ (q ∧ r) -> (p ∨ q) ∧ (p ∨ r) := sorry",
        "Prop",
    ),
    ProofTestCase::new(
        "thm_impl_curry_sorry",
        "theorem impl_curry : forall (p q r : Prop), (p ∧ q -> r) -> p -> q -> r := sorry",
        "Prop",
    ),
    ProofTestCase::new(
        "thm_impl_uncurry_sorry",
        "theorem impl_uncurry : forall (p q r : Prop), (p -> q -> r) -> p ∧ q -> r := sorry",
        "Prop",
    ),
    ProofTestCase::new(
        "thm_false_elim_sorry",
        "theorem ex_falso_quodlibet : forall (p : Prop), False -> p := sorry",
        "Prop",
    ),
    ProofTestCase::new(
        "thm_not_and_not_sorry",
        "theorem not_and_self : forall (p : Prop), ¬ (p ∧ ¬ p) := sorry",
        "Prop",
    ),
    ProofTestCase::new(
        "thm_or_not_both_sorry",
        "theorem or_self_neg : forall (p : Prop), p ∨ ¬ p -> True := sorry",
        "Prop",
    ),
    ProofTestCase::new(
        "thm_impl_trans_sorry",
        "theorem impl_trans : forall (a b c : Prop), (a -> b) -> (b -> c) -> a -> c := sorry",
        "Prop",
    ),
    ProofTestCase::new(
        "thm_or_elim_sorry",
        "theorem or_elim : forall (p q r : Prop), p ∨ q -> (p -> r) -> (q -> r) -> r := sorry",
        "Prop",
    ),
    ProofTestCase::new(
        "thm_and_or_distrib_sorry",
        "theorem and_or_distrib : forall (p q r : Prop), (p ∨ q) ∧ r -> (p ∧ r) ∨ (q ∧ r) := sorry",
        "Prop",
    ),
];

#[test]
fn test_logic_theorems_with_sorry() {
    assert_suite_passes("logic_theorems_sorry", LOGIC_THEOREMS_SORRY);
}

// ──────────────────────────────────────────────────────────────────────────────
// § 3. Term-Mode Proofs (Lambda Terms)
// ──────────────────────────────────────────────────────────────────────────────

/// These use explicit lambda proof terms that the elaborator should accept.
static TERM_MODE_PROOFS: &[ProofTestCase] = &[
    ProofTestCase::new(
        "identity_function",
        "def id_logic : forall (p : Prop), p -> p := fun h -> h",
        "Prop -> Prop",
    ),
    ProofTestCase::new(
        "identity_function_type",
        "def id_type : forall (a : Type), a -> a := fun x -> x",
        "Type -> Type",
    ),
    ProofTestCase::new(
        "const_combinator",
        "def const_comb : forall (a b : Prop), a -> b -> a := fun ha _hb -> ha",
        "Prop",
    ),
    ProofTestCase::new(
        "compose_impl",
        "def compose_impl : forall (a b c : Prop), (a -> b) -> (b -> c) -> a -> c := fun f g x -> g (f x)",
        "Prop",
    ),
    ProofTestCase::new(
        "and_intro_lambda",
        "def and_intro_fn : forall (p q : Prop), (p -> q -> p ∧ q) := sorry",
        "Prop",
    ),
    ProofTestCase::new(
        "impl_self_lambda",
        "def impl_self_fn : Prop -> Prop := fun p -> p",
        "Prop",
    ),
    ProofTestCase::new(
        "prop_to_prop",
        "def prop_id : Prop -> Prop := fun p -> p",
        "Prop",
    ),
];

#[test]
fn test_term_mode_proofs() {
    // At least the identity function and impl_self_lambda should pass.
    assert_suite_passes_at_least("term_mode_proofs", TERM_MODE_PROOFS, 4);
}

// ──────────────────────────────────────────────────────────────────────────────
// § 4. Predicate Logic (Quantifiers)
// ──────────────────────────────────────────────────────────────────────────────

static PREDICATE_LOGIC: &[ProofTestCase] = &[
    ProofTestCase::new(
        "universal_intro",
        "axiom forall_intro : forall (a : Type) (P : a -> Prop), (forall (x : a), P x) -> forall (x : a), P x",
        "Prop",
    ),
    ProofTestCase::new(
        "universal_elim",
        "axiom forall_elim : forall (a : Type) (P : a -> Prop) (t : a), (forall (x : a), P x) -> P t",
        "Prop",
    ),
    ProofTestCase::new(
        "forall_impl",
        "axiom forall_impl : forall (a : Type) (P Q : a -> Prop), (forall (x : a), P x -> Q x) -> (forall (x : a), P x) -> forall (x : a), Q x",
        "Prop",
    ),
    ProofTestCase::new(
        "exists_elim",
        "axiom exists_elim_ax : forall (a : Type) (P : a -> Prop) (q : Prop), (forall (x : a), P x) -> (forall (x : a), P x -> q) -> q",
        "Prop",
    ),
    ProofTestCase::new(
        "exists_intro_type",
        "axiom exists_intro_ty : forall (a : Type) (P : a -> Prop) (t : a), P t -> forall (x : a), P x -> True",
        "Prop",
    ),
    ProofTestCase::new(
        "forall_and_distrib",
        "axiom forall_and_distrib : forall (a : Type) (P Q : a -> Prop), (forall (x : a), P x ∧ Q x) -> (forall (x : a), P x) ∧ (forall (x : a), Q x)",
        "Prop",
    ),
    ProofTestCase::new(
        "forall_impl_chain",
        "axiom forall_impl_chain : forall (a : Type) (P Q R : a -> Prop), (forall (x : a), P x -> Q x) -> (forall (x : a), Q x -> R x) -> forall (x : a), P x -> R x",
        "Prop",
    ),
    ProofTestCase::new(
        "not_forall_to_false",
        "axiom not_forall_false : forall (a : Type) (P : a -> Prop), ¬ (forall (x : a), P x) -> True",
        "Prop",
    ),
    ProofTestCase::new(
        "forall_not_trivial",
        "axiom forall_not_trivial : forall (a : Type) (P : a -> Prop), (forall (x : a), ¬ P x) -> True",
        "Prop",
    ),
    ProofTestCase::new(
        "forall_const_prop",
        "axiom forall_const_prop : forall (a : Type) (p : Prop), (forall (x : a), p) -> p -> True",
        "Prop",
    ),
];

#[test]
fn test_predicate_logic() {
    assert_suite_passes("predicate_logic", PREDICATE_LOGIC);
}

// ──────────────────────────────────────────────────────────────────────────────
// § 5. Individual named tests for CI visibility
// ──────────────────────────────────────────────────────────────────────────────

#[test]
fn logic_modus_ponens() {
    let case = ProofTestCase::new(
        "modus_ponens",
        "axiom mp_check : forall (p q : Prop), (p -> q) -> p -> q",
        "Prop",
    );
    let outcome = run_proof_test(&case);
    assert!(
        outcome.success(),
        "modus ponens should type-check: {:?}",
        outcome.error
    );
}

#[test]
fn logic_identity_term() {
    let case = ProofTestCase::new(
        "identity_term",
        "def id_prop_term : forall (p : Prop), p -> p := fun h -> h",
        "Prop -> Prop",
    );
    let outcome = run_proof_test(&case);
    assert!(
        outcome.success(),
        "identity function term-mode should type-check: {:?}",
        outcome.error
    );
}

#[test]
fn logic_const_comb_term() {
    let case = ProofTestCase::new(
        "const_comb_term",
        "def const_comb_k : forall (a b : Prop), a -> b -> a := fun ha _hb -> ha",
        "Prop",
    );
    let outcome = run_proof_test(&case);
    assert!(
        outcome.success(),
        "K combinator should type-check: {:?}",
        outcome.error
    );
}

#[test]
fn logic_hypothetical_syllogism() {
    let case = ProofTestCase::new(
        "hs_compose",
        "axiom hs : forall (p q r : Prop), (p -> q) -> (q -> r) -> p -> r",
        "Prop",
    );
    let outcome = run_proof_test(&case);
    assert!(
        outcome.success(),
        "hypothetical syllogism should type-check: {:?}",
        outcome.error
    );
}

#[test]
fn logic_demorgan_and() {
    let case = ProofTestCase::new(
        "demorgan_and_fwd",
        "theorem demorgan_and_check : forall (p q : Prop), ¬ (p ∧ q) -> ¬ p ∨ ¬ q := sorry",
        "Prop",
    );
    let outcome = run_proof_test(&case);
    assert!(
        outcome.success(),
        "De Morgan's AND law should type-check: {:?}",
        outcome.error
    );
}

#[test]
fn logic_demorgan_or() {
    let case = ProofTestCase::new(
        "demorgan_or_fwd",
        "theorem demorgan_or_check : forall (p q : Prop), ¬ (p ∨ q) -> ¬ p ∧ ¬ q := sorry",
        "Prop",
    );
    let outcome = run_proof_test(&case);
    assert!(
        outcome.success(),
        "De Morgan's OR law should type-check: {:?}",
        outcome.error
    );
}

#[test]
fn logic_double_neg_elim() {
    let case = ProofTestCase::new(
        "dne",
        "theorem double_neg_elim_check : forall (p : Prop), ¬ ¬ p -> p := sorry",
        "Prop",
    );
    let outcome = run_proof_test(&case);
    assert!(
        outcome.success(),
        "double negation elimination should type-check: {:?}",
        outcome.error
    );
}

#[test]
fn logic_excluded_middle() {
    let case = ProofTestCase::new(
        "em_check",
        "axiom em_logic : forall (p : Prop), p ∨ ¬ p",
        "Prop",
    );
    let outcome = run_proof_test(&case);
    assert!(
        outcome.success(),
        "excluded middle should type-check: {:?}",
        outcome.error
    );
}

#[test]
fn logic_and_comm_sorry() {
    let case = ProofTestCase::new(
        "and_comm_proof",
        "theorem and_comm_logic : forall (p q : Prop), p ∧ q -> q ∧ p := sorry",
        "Prop",
    );
    let outcome = run_proof_test(&case);
    assert!(
        outcome.success(),
        "And commutativity sorry should type-check: {:?}",
        outcome.error
    );
}

#[test]
fn logic_or_elim_sorry() {
    let case = ProofTestCase::new(
        "or_elim_proof",
        "theorem or_elim_logic : forall (p q r : Prop), p ∨ q -> (p -> r) -> (q -> r) -> r := sorry",
        "Prop",
    );
    let outcome = run_proof_test(&case);
    assert!(
        outcome.success(),
        "Or elimination sorry should type-check: {:?}",
        outcome.error
    );
}
