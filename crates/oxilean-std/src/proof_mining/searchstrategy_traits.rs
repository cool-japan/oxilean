//! # SearchStrategy - Trait Implementations
//!
//! This module contains trait implementations for `SearchStrategy`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::SearchStrategy;
use std::fmt;

impl std::fmt::Display for SearchStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SearchStrategy::DFS => write!(f, "DFS"),
            SearchStrategy::BFS => write!(f, "BFS"),
            SearchStrategy::AStar => write!(f, "A*"),
            SearchStrategy::IDDFS => write!(f, "IDDFS"),
            SearchStrategy::MCTS => write!(f, "MCTS"),
        }
    }
}
