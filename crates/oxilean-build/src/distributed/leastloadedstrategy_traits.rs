//! # LeastLoadedStrategy - Trait Implementations
//!
//! This module contains trait implementations for `LeastLoadedStrategy`.
//!
//! ## Implemented Traits
//!
//! - `LoadBalancingStrategy`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::LoadBalancingStrategy;
use super::types::{LeastLoadedStrategy, RemoteWorker};

impl LoadBalancingStrategy for LeastLoadedStrategy {
    fn select(&self, workers: &[RemoteWorker]) -> Option<usize> {
        workers
            .iter()
            .enumerate()
            .filter(|(_, w)| w.is_available())
            .min_by(|(_, a), (_, b)| {
                a.utilization()
                    .partial_cmp(&b.utilization())
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(i, _)| i)
    }
}
