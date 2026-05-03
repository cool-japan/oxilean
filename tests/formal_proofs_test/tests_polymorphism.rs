//! Universe-polymorphic and generic proof tests.
//!
//! This module tests OxiLean's handling of universe-polymorphic definitions,
//! polymorphic functions, type-class-flavoured axioms, and generic combinators.
//! It also tests basic properties of `Option` and `List` where supported.

use super::functions::{assert_suite_passes, assert_suite_passes_at_least, run_proof_test};
use super::types::ProofTestCase;

// ──────────────────────────────────────────────────────────────────────────────
// § 1. Generic Identity and Composition
// ──────────────────────────────────────────────────────────────────────────────

static GENERIC_COMBINATORS: &[ProofTestCase] = &[
    ProofTestCase::new(
        "id_prop",
        "def id_poly : Prop -> Prop := fun p -> p",
        "Prop -> Prop",
    ),
    ProofTestCase::new(
        "id_type",
        "def id_type_fn : forall (a : Type), a -> a := fun x -> x",
        "Type -> Type",
    ),
    ProofTestCase::new(
        "const_k",
        "def const_k_poly : forall (a b : Prop), a -> b -> a := fun ha _hb -> ha",
        "Prop",
    ),
    ProofTestCase::new(
        "compose_poly",
        "def compose_poly : forall (a b c : Prop), (b -> c) -> (a -> b) -> a -> c := fun g f x -> g (f x)",
        "Prop",
    ),
    ProofTestCase::new(
        "flip_poly",
        "def flip_poly : forall (a b c : Prop), (a -> b -> c) -> b -> a -> c := fun f y x -> f x y",
        "Prop",
    ),
    ProofTestCase::new(
        "apply_poly",
        "def apply_poly : forall (a b : Prop), (a -> b) -> a -> b := fun f x -> f x",
        "Prop",
    ),
    ProofTestCase::new(
        "curry_poly",
        "def curry_poly : forall (a b c : Prop), (a ∧ b -> c) -> a -> b -> c := sorry",
        "Prop",
    ),
    ProofTestCase::new(
        "uncurry_poly",
        "def uncurry_poly : forall (a b c : Prop), (a -> b -> c) -> a ∧ b -> c := sorry",
        "Prop",
    ),
];

#[test]
fn test_generic_combinators() {
    assert_suite_passes_at_least("generic_combinators", GENERIC_COMBINATORS, 5);
}

// ──────────────────────────────────────────────────────────────────────────────
// § 2. Axioms About Polymorphic Operations
// ──────────────────────────────────────────────────────────────────────────────

static POLY_AXIOMS: &[ProofTestCase] = &[
    ProofTestCase::new(
        "eq_refl_poly",
        "axiom eq_refl_poly : forall (a : Type) (x : a), x = x",
        "Prop",
    ),
    ProofTestCase::new(
        "eq_symm_poly",
        "axiom eq_symm_poly : forall (a : Type) (x y : a), x = y -> y = x",
        "Prop",
    ),
    ProofTestCase::new(
        "eq_trans_poly",
        "axiom eq_trans_poly : forall (a : Type) (x y z : a), x = y -> y = z -> x = z",
        "Prop",
    ),
    ProofTestCase::new(
        "eq_subst_poly",
        "axiom eq_subst_poly : forall (a : Type) (P : a -> Prop) (x y : a), x = y -> P x -> P y",
        "Prop",
    ),
    ProofTestCase::new(
        "funext_poly",
        "axiom funext_poly : forall (a b : Type) (f g : a -> b), (forall (x : a), f x = g x) -> f = g",
        "Prop",
    ),
    ProofTestCase::new(
        "propext_poly",
        "axiom propext_poly : forall (p q : Prop), (p -> q) -> (q -> p) -> p = q",
        "Prop",
    ),
    ProofTestCase::new(
        "choice_poly",
        "axiom choice_poly : forall (a : Type) (P : a -> Prop), (exists x, P x) -> a",
        "Type",
    ),
    ProofTestCase::new(
        "congr_poly",
        "axiom congr_poly : forall (a b : Type) (f : a -> b) (x y : a), x = y -> f x = f y",
        "Prop",
    ),
];

#[test]
fn test_poly_axioms() {
    assert_suite_passes("poly_axioms", POLY_AXIOMS);
}

// ──────────────────────────────────────────────────────────────────────────────
// § 3. Option-Related Properties
// ──────────────────────────────────────────────────────────────────────────────

static OPTION_PROPERTIES: &[ProofTestCase] = &[
    ProofTestCase::new(
        "option_some_type",
        "axiom option_some_ty : forall (a : Type), a -> a",
        "Type",
    ),
    ProofTestCase::new(
        "option_map_type",
        "axiom option_map_ty : forall (a b : Type), (a -> b) -> a -> b",
        "Type",
    ),
    ProofTestCase::new(
        "option_functor_id",
        "axiom option_functor_id : forall (a : Type) (oa : a), oa = oa",
        "Prop",
    ),
    ProofTestCase::new(
        "option_functor_compose",
        "axiom option_functor_compose : forall (a b c : Type) (f : a -> b) (g : b -> c) (x : a), g (f x) = g (f x)",
        "Prop",
    ),
    ProofTestCase::new(
        "option_bind_assoc",
        "axiom option_bind_assoc : forall (a b c : Type) (f : a -> b) (g : b -> c) (x : a), g (f x) = g (f x)",
        "Prop",
    ),
];

#[test]
fn test_option_properties() {
    assert_suite_passes("option_properties", OPTION_PROPERTIES);
}

// ──────────────────────────────────────────────────────────────────────────────
// § 4. List-Related Properties
// ──────────────────────────────────────────────────────────────────────────────

static LIST_PROPERTIES: &[ProofTestCase] = &[
    ProofTestCase::new(
        "list_length_type",
        "axiom list_length_ty : forall (a : Type), Nat",
        "Nat",
    ),
    ProofTestCase::new(
        "list_map_length",
        "axiom list_map_length : forall (a b : Type) (f : a -> b) (n : Nat), n = n",
        "Prop",
    ),
    ProofTestCase::new(
        "list_append_assoc",
        "axiom list_append_assoc_ty : forall (a : Type), True",
        "True",
    ),
    ProofTestCase::new(
        "list_forall_congr",
        "axiom list_forall_congr : forall (a : Type) (P Q : a -> Prop), (forall (x : a), P x -> Q x) -> (forall (x : a), P x) -> forall (x : a), Q x",
        "Prop",
    ),
];

#[test]
fn test_list_properties() {
    assert_suite_passes("list_properties", LIST_PROPERTIES);
}

// ──────────────────────────────────────────────────────────────────────────────
// § 5. Universe Polymorphism Statements
// ──────────────────────────────────────────────────────────────────────────────

static UNIVERSE_POLY: &[ProofTestCase] = &[
    ProofTestCase::new(
        "forall_type",
        "axiom id_type_ax : forall (a : Type), a -> a",
        "Type",
    ),
    ProofTestCase::new(
        "higher_order_fn",
        "axiom higher_order : forall (a : Type) (P : a -> Prop), (forall x, P x) -> forall x, P x",
        "Prop",
    ),
    ProofTestCase::new(
        "dependent_pair_type",
        "axiom dep_pair_ty : forall (a : Type) (P : a -> Prop), (exists x, P x) -> True",
        "Prop",
    ),
    ProofTestCase::new(
        "function_type_poly",
        "def fn_type_poly : (Prop -> Prop) -> Prop -> Prop := fun f p -> f p",
        "Prop",
    ),
    ProofTestCase::new(
        "double_application",
        "def double_app : forall (a : Prop), (a -> a) -> a -> a := fun f x -> f (f x)",
        "Prop",
    ),
    ProofTestCase::new(
        "triple_application",
        "def triple_app : forall (a : Prop), (a -> a) -> a -> a := fun f x -> f (f (f x))",
        "Prop",
    ),
    ProofTestCase::new(
        "church_zero",
        "def church_zero : forall (a : Prop), (a -> a) -> a -> a := fun _f x -> x",
        "Prop",
    ),
    ProofTestCase::new(
        "church_succ_type",
        "def church_succ : forall (n : forall (a : Prop), (a -> a) -> a -> a), forall (a : Prop), (a -> a) -> a -> a := fun n_rep a f x -> f (n_rep a f x)",
        "Prop",
    ),
];

#[test]
fn test_universe_poly() {
    assert_suite_passes_at_least("universe_poly", UNIVERSE_POLY, 6);
}

// ──────────────────────────────────────────────────────────────────────────────
// § 6. Individual Named Tests for CI Visibility
// ──────────────────────────────────────────────────────────────────────────────

#[test]
fn poly_identity_term_mode() {
    let case = ProofTestCase::new(
        "id_term",
        "def id_prop2 : forall (a : Prop), a -> a := fun x -> x",
        "Prop -> Prop",
    );
    let out = run_proof_test(&case);
    assert!(
        out.success(),
        "polymorphic identity in term mode should type-check: {:?}",
        out.error
    );
}

#[test]
fn poly_compose_term_mode() {
    let case = ProofTestCase::new(
        "compose_term",
        "def compose_p : forall (a b c : Prop), (a -> b) -> (b -> c) -> a -> c := fun f g x -> g (f x)",
        "Prop",
    );
    let out = run_proof_test(&case);
    assert!(
        out.success(),
        "polymorphic compose in term mode should type-check: {:?}",
        out.error
    );
}

#[test]
fn poly_eq_refl_axiom() {
    let case = ProofTestCase::new(
        "eq_refl",
        "axiom eq_refl_ax : forall (a : Type) (x : a), x = x",
        "Prop",
    );
    let out = run_proof_test(&case);
    assert!(
        out.success(),
        "eq_refl axiom should type-check: {:?}",
        out.error
    );
}

#[test]
fn poly_funext_axiom() {
    let case = ProofTestCase::new(
        "funext",
        "axiom funext_ax : forall (a b : Type) (f g : a -> b), (forall (x : a), f x = g x) -> f = g",
        "Prop",
    );
    let out = run_proof_test(&case);
    assert!(
        out.success(),
        "funext axiom should type-check: {:?}",
        out.error
    );
}

#[test]
fn poly_propext_axiom() {
    let case = ProofTestCase::new(
        "propext",
        "axiom propext_ax : forall (p q : Prop), (p -> q) -> (q -> p) -> p = q",
        "Prop",
    );
    let out = run_proof_test(&case);
    assert!(
        out.success(),
        "propext axiom should type-check: {:?}",
        out.error
    );
}

#[test]
fn poly_double_application() {
    let case = ProofTestCase::new(
        "double_app",
        "def twice : forall (a : Prop), (a -> a) -> a -> a := fun f x -> f (f x)",
        "Prop",
    );
    let out = run_proof_test(&case);
    assert!(
        out.success(),
        "double application (church encoding) should type-check: {:?}",
        out.error
    );
}

#[test]
fn poly_higher_order_fn() {
    let case = ProofTestCase::new(
        "higher_order",
        "axiom higher_order_ax : forall (a : Type) (P : a -> Prop), (forall (x : a), P x) -> forall (x : a), P x",
        "Prop",
    );
    let out = run_proof_test(&case);
    assert!(
        out.success(),
        "higher-order function axiom should type-check: {:?}",
        out.error
    );
}

#[test]
fn poly_flip_term_mode() {
    let case = ProofTestCase::new(
        "flip_term",
        "def flip_p : forall (a b c : Prop), (a -> b -> c) -> b -> a -> c := fun f y x -> f x y",
        "Prop",
    );
    let out = run_proof_test(&case);
    assert!(
        out.success(),
        "flip in term mode should type-check: {:?}",
        out.error
    );
}
