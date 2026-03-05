//! # InstanceError - Trait Implementations
//!
//! This module contains trait implementations for `InstanceError`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::InstanceError;
use std::fmt;

impl std::fmt::Display for InstanceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstanceError::NotFound { class } => {
                write!(f, "no instance for class '{:?}'", class)
            }
            InstanceError::Ambiguous { class, candidates } => {
                write!(
                    f,
                    "ambiguous instances for class '{:?}': {:?}",
                    class, candidates
                )
            }
            InstanceError::MaxDepthExceeded { depth } => {
                write!(f, "instance search exceeded max depth {}", depth)
            }
            InstanceError::CircularDependency { chain } => {
                write!(f, "circular instance dependency: {:?}", chain)
            }
            InstanceError::UnresolvableSubgoal { instance, subgoal } => {
                write!(
                    f,
                    "instance '{}' has unresolvable subgoal '{}'",
                    instance, subgoal
                )
            }
        }
    }
}
