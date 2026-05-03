//! # TacticRegistry - Trait Implementations
//!
//! This module contains trait implementations for `TacticRegistry`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Expr, Literal, Name};
use std::fmt;

use super::types::{Tactic, TacticRegistry};

impl Default for TacticRegistry {
    fn default() -> Self {
        let mut registry = Self::new();
        let tactics_data = [
            ("intro", "Introduce a hypothesis or lambda", Some(1)),
            ("intros", "Introduce multiple hypotheses", None),
            ("apply", "Apply a function or theorem", Some(1)),
            ("exact", "Provide an exact proof term", Some(1)),
            ("assumption", "Find a matching hypothesis", Some(0)),
            ("refl", "Prove by reflexivity", Some(0)),
            ("trivial", "Try refl, assumption, True.intro", Some(0)),
            (
                "constructor",
                "Apply the constructor of the target",
                Some(0),
            ),
            ("split", "Split Iff or And goal", Some(0)),
            ("left", "Apply Or.inl", Some(0)),
            ("right", "Apply Or.inr", Some(0)),
            ("exists", "Provide an existential witness", Some(1)),
            ("exfalso", "Change target to False", Some(0)),
            ("clear", "Remove a hypothesis", Some(1)),
            ("rename", "Rename a hypothesis", Some(2)),
            ("revert", "Move hypothesis back into target", Some(1)),
            ("have", "Introduce a new hypothesis with proof", None),
            ("suffices", "Assert a sufficient condition", None),
            ("cases", "Case split on a hypothesis", Some(1)),
            (
                "induction",
                "Structural induction on a Nat hypothesis",
                Some(1),
            ),
            ("sorry", "Close goal unsafely", Some(0)),
            ("push_neg", "Push negations inward", Some(0)),
            ("by_contra", "Proof by contradiction", Some(0)),
            ("by_contradiction", "Proof by contradiction", Some(0)),
            ("contrapose", "Prove by contrapositive", Some(0)),
            ("norm_cast", "Normalize casts", Some(0)),
            ("exact_mod_cast", "Exact with cast normalization", Some(0)),
            ("push_cast", "Push casts inward", Some(0)),
            ("field_simp", "Field simplification", Some(0)),
            ("rfl", "Reflexivity (alias for refl)", Some(0)),
            ("ring", "Ring normalization", Some(0)),
            ("linarith", "Linear arithmetic", Some(0)),
            ("nlinarith", "Nonlinear arithmetic", Some(0)),
            ("simp_all", "Simp with all hypotheses", Some(0)),
            ("aesop", "Automated proof search", Some(0)),
            ("tauto", "Propositional tautology prover", Some(0)),
            ("symm", "Symmetry of equality", Some(0)),
            ("symmetry", "Symmetry of equality", Some(0)),
            ("trans", "Transitivity step", None),
            ("congr", "Congruence closure", Some(0)),
            ("gcongr", "Generalized congruence", Some(0)),
            ("subst", "Substitute equality hypothesis", None),
            (
                "specialize",
                "Instantiate universally-quantified hypothesis",
                None,
            ),
            ("norm_num", "Numeric normalization", Some(0)),
            ("decide", "Evaluate decidable proposition", Some(0)),
            ("fin_cases", "Finite case analysis", Some(0)),
            ("interval_cases", "Interval case analysis", Some(0)),
            ("omega", "Linear arithmetic (Omega)", Some(0)),
            ("repeat", "Repeat tactic until failure", None),
            ("first", "Try alternatives in order", None),
            ("try", "Try tactic, ignore failure", None),
            ("all_goals", "Apply tactic to all goals", None),
            ("exact?", "Exact with suggestion", Some(0)),
            ("apply?", "Apply with suggestion", Some(0)),
            ("simp?", "Simp with suggestion", Some(0)),
            ("rw?", "Rewrite with suggestion", Some(0)),
            ("continuity", "Prove continuity of a function", Some(0)),
            (
                "measurability",
                "Prove measurability of a function",
                Some(0),
            ),
            ("mono", "Monotonicity", Some(0)),
            ("grind", "E-matching congruence closure", Some(0)),
            ("bv_decide", "Bit-vector decision procedure", Some(0)),
            ("slim_check", "Random counterexample finder", Some(0)),
            ("simp_rw", "Rewrite then simplify", Some(1)),
            (
                "convert",
                "Change goal to defeq form with subgoals",
                Some(1),
            ),
            ("abel", "Abelian group normalization", Some(0)),
            ("group", "Group normalization", Some(0)),
        ];
        for (name, desc, arity) in tactics_data {
            registry.register_with_arity(
                Tactic {
                    name: Name::str(name),
                    description: desc.to_string(),
                },
                arity,
            );
        }
        registry
    }
}
