//! Natural number arithmetic formal proof tests.
//!
//! This module tests that OxiLean can parse and elaborate theorems about Nat
//! arithmetic, including commutativity, associativity, distributivity, and
//! induction-based properties.  All proofs use `sorry` because kernel-level
//! arithmetic reduction is not yet wired to the elaborator.
//!
//! Note: OxiLean's elaborator resolves `n + m` via the `+` operator identifier
//! which requires `HAdd` type-class instances. We therefore use explicit
//! `Nat.add n m` / `Nat.mul n m` function calls to sidestep operator lookup,
//! or express statements in terms of `Nat.succ` where appropriate.

use super::functions::{assert_suite_passes, run_proof_test};
use super::types::ProofTestCase;

// ──────────────────────────────────────────────────────────────────────────────
// § 1. Nat Arithmetic Identities — Axioms (using Nat.add / Nat.mul)
// ──────────────────────────────────────────────────────────────────────────────

static NAT_AXIOMS: &[ProofTestCase] = &[
    ProofTestCase::new(
        "axiom_zero_add",
        "axiom nat_zero_add : forall (n : Nat), Nat.add 0 n = n",
        "Prop",
    ),
    ProofTestCase::new(
        "axiom_add_zero",
        "axiom nat_add_zero : forall (n : Nat), Nat.add n 0 = n",
        "Prop",
    ),
    ProofTestCase::new(
        "axiom_add_comm",
        "axiom nat_add_comm : forall (n m : Nat), Nat.add n m = Nat.add m n",
        "Prop",
    ),
    ProofTestCase::new(
        "axiom_add_assoc",
        "axiom nat_add_assoc : forall (n m k : Nat), Nat.add (Nat.add n m) k = Nat.add n (Nat.add m k)",
        "Prop",
    ),
    ProofTestCase::new(
        "axiom_mul_comm",
        "axiom nat_mul_comm : forall (n m : Nat), Nat.mul n m = Nat.mul m n",
        "Prop",
    ),
    ProofTestCase::new(
        "axiom_mul_one",
        "axiom nat_mul_one : forall (n : Nat), Nat.mul n 1 = n",
        "Prop",
    ),
    ProofTestCase::new(
        "axiom_one_mul",
        "axiom nat_one_mul : forall (n : Nat), Nat.mul 1 n = n",
        "Prop",
    ),
    ProofTestCase::new(
        "axiom_zero_mul",
        "axiom nat_zero_mul : forall (n : Nat), Nat.mul 0 n = 0",
        "Prop",
    ),
    ProofTestCase::new(
        "axiom_mul_zero",
        "axiom nat_mul_zero : forall (n : Nat), Nat.mul n 0 = 0",
        "Prop",
    ),
    ProofTestCase::new(
        "axiom_mul_assoc",
        "axiom nat_mul_assoc : forall (n m k : Nat), Nat.mul (Nat.mul n m) k = Nat.mul n (Nat.mul m k)",
        "Prop",
    ),
    ProofTestCase::new(
        "axiom_succ_ne_zero",
        "axiom nat_succ_ne_zero : forall (n : Nat), ¬ (Nat.succ n = 0)",
        "Prop",
    ),
    ProofTestCase::new(
        "axiom_succ_add",
        "axiom nat_succ_add : forall (n m : Nat), Nat.add (Nat.succ n) m = Nat.succ (Nat.add n m)",
        "Prop",
    ),
    ProofTestCase::new(
        "axiom_add_succ",
        "axiom nat_add_succ : forall (n m : Nat), Nat.add n (Nat.succ m) = Nat.succ (Nat.add n m)",
        "Prop",
    ),
    ProofTestCase::new(
        "axiom_mul_distrib_r",
        "axiom nat_mul_distrib_r : forall (n m k : Nat), Nat.mul (Nat.add n m) k = Nat.add (Nat.mul n k) (Nat.mul m k)",
        "Prop",
    ),
    ProofTestCase::new(
        "axiom_mul_distrib_l",
        "axiom nat_mul_distrib_l : forall (n m k : Nat), Nat.mul n (Nat.add m k) = Nat.add (Nat.mul n m) (Nat.mul n k)",
        "Prop",
    ),
];

#[test]
fn test_nat_axioms() {
    assert_suite_passes("nat_axioms", NAT_AXIOMS);
}

// ──────────────────────────────────────────────────────────────────────────────
// § 2. Nat Theorems with Sorry Proofs
// ──────────────────────────────────────────────────────────────────────────────

static NAT_THEOREMS_SORRY: &[ProofTestCase] = &[
    ProofTestCase::new(
        "thm_zero_add",
        "theorem zero_add_thm : forall (n : Nat), Nat.add 0 n = n := sorry",
        "Prop",
    ),
    ProofTestCase::new(
        "thm_add_zero",
        "theorem add_zero_thm : forall (n : Nat), Nat.add n 0 = n := sorry",
        "Prop",
    ),
    ProofTestCase::new(
        "thm_add_comm",
        "theorem add_comm_thm : forall (n m : Nat), Nat.add n m = Nat.add m n := sorry",
        "Prop",
    ),
    ProofTestCase::new(
        "thm_add_assoc",
        "theorem add_assoc_thm : forall (n m k : Nat), Nat.add (Nat.add n m) k = Nat.add n (Nat.add m k) := sorry",
        "Prop",
    ),
    ProofTestCase::new(
        "thm_mul_comm",
        "theorem mul_comm_thm : forall (n m : Nat), Nat.mul n m = Nat.mul m n := sorry",
        "Prop",
    ),
    ProofTestCase::new(
        "thm_mul_one",
        "theorem mul_one_thm : forall (n : Nat), Nat.mul n 1 = n := sorry",
        "Prop",
    ),
    ProofTestCase::new(
        "thm_succ_ne_zero",
        "theorem succ_ne_zero_thm : forall (n : Nat), ¬ (Nat.succ n = 0) := sorry",
        "Prop",
    ),
    ProofTestCase::new(
        "thm_mul_distrib_r",
        "theorem mul_distrib_r_thm : forall (n m k : Nat), Nat.mul (Nat.add n m) k = Nat.add (Nat.mul n k) (Nat.mul m k) := sorry",
        "Prop",
    ),
    ProofTestCase::new(
        "thm_mul_distrib_l",
        "theorem mul_distrib_l_thm : forall (n m k : Nat), Nat.mul n (Nat.add m k) = Nat.add (Nat.mul n m) (Nat.mul n k) := sorry",
        "Prop",
    ),
    ProofTestCase::new(
        "thm_add_left_cancel",
        "theorem add_left_cancel : forall (n m k : Nat), Nat.add n m = Nat.add n k -> m = k := sorry",
        "Prop",
    ),
    ProofTestCase::new(
        "thm_add_right_cancel",
        "theorem add_right_cancel : forall (n m k : Nat), Nat.add m n = Nat.add k n -> m = k := sorry",
        "Prop",
    ),
    ProofTestCase::new(
        "thm_nat_zero_or_succ",
        "theorem nat_zero_or_succ : forall (n : Nat), n = 0 ∨ exists (m : Nat), n = Nat.succ m := sorry",
        "Prop",
    ),
    ProofTestCase::new(
        "thm_nat_double_add",
        "theorem nat_double_add : forall (n : Nat), Nat.add n n = Nat.mul 2 n := sorry",
        "Prop",
    ),
    ProofTestCase::new(
        "thm_nat_succ_add",
        "theorem nat_succ_add : forall (n m : Nat), Nat.add (Nat.succ n) m = Nat.succ (Nat.add n m) := sorry",
        "Prop",
    ),
    ProofTestCase::new(
        "thm_nat_add_succ",
        "theorem nat_add_succ : forall (n m : Nat), Nat.add n (Nat.succ m) = Nat.succ (Nat.add n m) := sorry",
        "Prop",
    ),
    ProofTestCase::new(
        "thm_nat_zero_add_eq",
        "theorem nat_zero_add_eq : forall (n : Nat), Nat.add 0 n = n := sorry",
        "Prop",
    ),
    ProofTestCase::new(
        "thm_nat_add_zero_eq",
        "theorem nat_add_zero_eq : forall (n : Nat), Nat.add n 0 = n := sorry",
        "Prop",
    ),
    ProofTestCase::new(
        "thm_nat_zero_mul",
        "theorem nat_zero_mul_eq : forall (n : Nat), Nat.mul 0 n = 0 := sorry",
        "Prop",
    ),
    ProofTestCase::new(
        "thm_nat_mul_zero",
        "theorem nat_mul_zero_eq : forall (n : Nat), Nat.mul n 0 = 0 := sorry",
        "Prop",
    ),
    ProofTestCase::new(
        "thm_nat_one_mul",
        "theorem nat_one_mul_eq : forall (n : Nat), Nat.mul 1 n = n := sorry",
        "Prop",
    ),
];

#[test]
fn test_nat_theorems_sorry() {
    assert_suite_passes("nat_theorems_sorry", NAT_THEOREMS_SORRY);
}

// ──────────────────────────────────────────────────────────────────────────────
// § 3. Induction-Based Statement Shapes
// ──────────────────────────────────────────────────────────────────────────────

/// These tests verify that induction-flavoured statement shapes type-check.
static INDUCTION_STATEMENTS: &[ProofTestCase] = &[
    ProofTestCase::new(
        "ind_zero_base",
        "axiom nat_ind_zero_base : forall (P : Nat -> Prop), P 0 -> (forall (n : Nat), P n -> P (Nat.succ n)) -> forall (n : Nat), P n",
        "Prop",
    ),
    ProofTestCase::new(
        "ind_add_zero_ind",
        "theorem add_zero_ind : forall (P : Nat -> Prop), P 0 -> (forall (n : Nat), P n -> P (Nat.add n 0)) -> P 0 := sorry",
        "Prop",
    ),
    ProofTestCase::new(
        "ind_induction_principle",
        "axiom nat_induction : forall (P : Nat -> Prop), P 0 -> (forall (k : Nat), P k -> P (Nat.succ k)) -> forall (n : Nat), P n",
        "Prop",
    ),
    ProofTestCase::new(
        "ind_strong_induction",
        "axiom nat_strong_ind : forall (P : Nat -> Prop), (forall (n : Nat), (forall (m : Nat), m = Nat.zero -> P m) -> P n) -> forall (n : Nat), P n",
        "Prop",
    ),
    ProofTestCase::new(
        "ind_well_founded_type",
        "axiom nat_well_founded_ty : forall (P : Nat -> Prop), (forall (n : Nat), P (Nat.succ n) -> P n) -> P 0 -> forall (n : Nat), P n",
        "Prop",
    ),
];

#[test]
fn test_induction_statements() {
    assert_suite_passes("induction_statements", INDUCTION_STATEMENTS);
}

// ──────────────────────────────────────────────────────────────────────────────
// § 4. Individual Named Tests
// ──────────────────────────────────────────────────────────────────────────────

#[test]
fn nat_zero_add_parses_and_elabs() {
    let case = ProofTestCase::new(
        "zero_add",
        "theorem zero_add : forall (n : Nat), Nat.add 0 n = n := sorry",
        "Prop",
    );
    let out = run_proof_test(&case);
    assert!(out.success(), "zero_add should type-check: {:?}", out.error);
}

#[test]
fn nat_add_zero_parses_and_elabs() {
    let case = ProofTestCase::new(
        "add_zero",
        "theorem add_zero : forall (n : Nat), Nat.add n 0 = n := sorry",
        "Prop",
    );
    let out = run_proof_test(&case);
    assert!(out.success(), "add_zero should type-check: {:?}", out.error);
}

#[test]
fn nat_add_comm_parses_and_elabs() {
    let case = ProofTestCase::new(
        "add_comm",
        "theorem add_comm : forall (n m : Nat), Nat.add n m = Nat.add m n := sorry",
        "Prop",
    );
    let out = run_proof_test(&case);
    assert!(out.success(), "add_comm should type-check: {:?}", out.error);
}

#[test]
fn nat_add_assoc_parses_and_elabs() {
    let case = ProofTestCase::new(
        "add_assoc",
        "theorem add_assoc : forall (n m k : Nat), Nat.add (Nat.add n m) k = Nat.add n (Nat.add m k) := sorry",
        "Prop",
    );
    let out = run_proof_test(&case);
    assert!(
        out.success(),
        "add_assoc should type-check: {:?}",
        out.error
    );
}

#[test]
fn nat_mul_comm_parses_and_elabs() {
    let case = ProofTestCase::new(
        "mul_comm",
        "theorem mul_comm : forall (n m : Nat), Nat.mul n m = Nat.mul m n := sorry",
        "Prop",
    );
    let out = run_proof_test(&case);
    assert!(out.success(), "mul_comm should type-check: {:?}", out.error);
}

#[test]
fn nat_mul_distrib_parses_and_elabs() {
    let case = ProofTestCase::new(
        "mul_distrib",
        "theorem mul_distrib : forall (n m k : Nat), Nat.mul n (Nat.add m k) = Nat.add (Nat.mul n m) (Nat.mul n k) := sorry",
        "Prop",
    );
    let out = run_proof_test(&case);
    assert!(
        out.success(),
        "mul_distrib should type-check: {:?}",
        out.error
    );
}

#[test]
fn nat_succ_ne_zero_parses_and_elabs() {
    let case = ProofTestCase::new(
        "succ_ne_zero",
        "theorem succ_ne_zero : forall (n : Nat), ¬ (Nat.succ n = 0) := sorry",
        "Prop",
    );
    let out = run_proof_test(&case);
    assert!(
        out.success(),
        "succ_ne_zero should type-check: {:?}",
        out.error
    );
}

#[test]
fn nat_induction_principle_parses() {
    let case = ProofTestCase::new(
        "induction_axiom",
        "axiom nat_ind_ax : forall (P : Nat -> Prop), P 0 -> (forall (n : Nat), P n -> P (Nat.succ n)) -> forall (n : Nat), P n",
        "Prop",
    );
    let out = run_proof_test(&case);
    assert!(
        out.success(),
        "induction principle should type-check: {:?}",
        out.error
    );
}
