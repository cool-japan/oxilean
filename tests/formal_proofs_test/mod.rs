//! Formal proof type-checking test suite for OxiLean.
//!
//! This suite verifies that OxiLean can successfully parse and type-check
//! (elaborate) a collection of non-trivial formal proofs covering:
//!
//! - **Propositional and predicate logic** (`tests_logic`): And/Or introduction
//!   and elimination, modus ponens, hypothetical syllogism, De Morgan's laws,
//!   double negation, excluded middle, distribution laws.
//!
//! - **Nat arithmetic** (`tests_nat`): `zero_add`, `add_zero`, `add_comm`,
//!   `add_assoc`, `mul_comm`, `mul_distrib`, `succ_ne_zero`, induction shapes.
//!
//! - **Universe polymorphism** (`tests_polymorphism`): Identity function,
//!   function composition, flip, Church encodings, funext/propext axioms, and
//!   higher-order function statements.
//!
//! ## Test Infrastructure
//!
//! The module-level infrastructure (`types`, `functions`) provides:
//!
//! - `ProofTestCase` — a named (source, expected_type) triple
//! - `run_proof_test` — runs parse + elaborate and returns a `ProofOutcome`
//! - `assert_suite_passes` — asserts that every case in a slice passes
//! - `assert_suite_passes_at_least` — asserts a minimum pass count

mod functions;
mod tests_logic;
mod tests_nat;
mod tests_polymorphism;
mod types;
