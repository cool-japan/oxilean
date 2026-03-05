//! # ProofSystem - Trait Implementations
//!
//! This module contains trait implementations for `ProofSystem`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ProofSystem;
use std::fmt;

impl std::fmt::Display for ProofSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            ProofSystem::Resolution => "Resolution",
            ProofSystem::Frege => "Frege",
            ProofSystem::ExtendedFrege => "Extended Frege",
            ProofSystem::HalfFrege => "Half-Frege",
            ProofSystem::CuttingPlanes => "Cutting Planes",
            ProofSystem::Nullstellensatz => "Nullstellensatz",
            ProofSystem::SOS => "Sum-of-Squares",
            ProofSystem::IPS => "IPS",
        };
        write!(f, "{}", name)
    }
}
