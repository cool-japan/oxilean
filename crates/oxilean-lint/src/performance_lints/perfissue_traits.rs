//! # PerfIssue - Trait Implementations
//!
//! This module contains trait implementations for `PerfIssue`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::PerfIssue;

impl std::fmt::Display for PerfIssue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PerfIssue::SlowSimpLemma => write!(f, "slow_simp_lemma"),
            PerfIssue::LargeProofTerm => write!(f, "large_proof_term"),
            PerfIssue::DeepRecursion => write!(f, "deep_recursion"),
            PerfIssue::ExpensiveInstanceSearch => write!(f, "expensive_instance_search"),
            PerfIssue::UnboundedSearch => write!(f, "unbounded_search"),
            PerfIssue::InefficientPattern => write!(f, "inefficient_pattern"),
            PerfIssue::ExcessiveMetavars => write!(f, "excessive_metavars"),
            PerfIssue::LargeInductiveType => write!(f, "large_inductive_type"),
        }
    }
}

