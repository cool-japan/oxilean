//! # SecurityIssue - Trait Implementations
//!
//! This module contains trait implementations for `SecurityIssue`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::SecurityIssue;

impl std::fmt::Display for SecurityIssue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SecurityIssue::UncheckedInput => write!(f, "unchecked_input"),
            SecurityIssue::ExposedPrivateData => write!(f, "exposed_private_data"),
            SecurityIssue::WeakAssumption => write!(f, "weak_assumption"),
            SecurityIssue::CircularProof => write!(f, "circular_proof"),
            SecurityIssue::UnsoundAxiom => write!(f, "unsound_axiom"),
            SecurityIssue::ClassicalChoice => write!(f, "classical_choice"),
            SecurityIssue::PropExtMisuse => write!(f, "prop_ext_misuse"),
            SecurityIssue::DangerousFfi => write!(f, "dangerous_ffi"),
            SecurityIssue::UnverifiedExternal => write!(f, "unverified_external"),
        }
    }
}

