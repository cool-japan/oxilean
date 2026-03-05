//! # NodeHealthReport - Trait Implementations
//!
//! This module contains trait implementations for `NodeHealthReport`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::NodeHealthReport;

impl std::fmt::Display for NodeHealthReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Node[{}]: healthy={} cpu={:.1}% mem={:.1}% latency={}ms",
            self.worker_id,
            self.healthy,
            self.cpu_usage * 100.0,
            self.mem_usage * 100.0,
            self.latency_ms,
        )
    }
}
