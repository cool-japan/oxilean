//! # NodeStatus - Trait Implementations
//!
//! This module contains trait implementations for `NodeStatus`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::NodeStatus;

impl std::fmt::Display for NodeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeStatus::Open => write!(f, "open"),
            NodeStatus::Expanded => write!(f, "expanded"),
            NodeStatus::Solved => write!(f, "solved"),
            NodeStatus::Failed => write!(f, "failed"),
            NodeStatus::Pruned => write!(f, "pruned"),
        }
    }
}
