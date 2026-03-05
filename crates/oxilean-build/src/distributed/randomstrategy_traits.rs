//! # RandomStrategy - Trait Implementations
//!
//! This module contains trait implementations for `RandomStrategy`.
//!
//! ## Implemented Traits
//!
//! - `LoadBalancingStrategy`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::LoadBalancingStrategy;
use super::types::{RandomStrategy, RemoteWorker};

impl LoadBalancingStrategy for RandomStrategy {
    fn select(&self, workers: &[RemoteWorker]) -> Option<usize> {
        let available: Vec<usize> = workers
            .iter()
            .enumerate()
            .filter(|(_, w)| w.is_available())
            .map(|(i, _)| i)
            .collect();
        if available.is_empty() {
            return None;
        }
        let idx = (self.seed as usize) % available.len();
        Some(available[idx])
    }
}
