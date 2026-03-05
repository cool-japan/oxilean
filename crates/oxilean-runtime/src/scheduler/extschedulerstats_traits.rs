//! # ExtSchedulerStats - Trait Implementations
//!
//! This module contains trait implementations for `ExtSchedulerStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ExtSchedulerStats;
use std::fmt;

impl fmt::Display for ExtSchedulerStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "ExtSchedulerStats:")?;
        writeln!(f, "  Created:    {}", self.tasks_created)?;
        writeln!(f, "  Completed:  {}", self.tasks_completed)?;
        writeln!(f, "  Cancelled:  {}", self.tasks_cancelled)?;
        writeln!(f, "  Stolen:     {}", self.tasks_stolen)?;
        writeln!(f, "  Utilization:{:.1}%", self.utilization() * 100.0)?;
        writeln!(f, "  Avg latency:{:.1} ticks", self.avg_latency())?;
        writeln!(f, "  Max latency:{} ticks", self.max_latency_ticks)?;
        writeln!(f, "  Violations: {}", self.latency_violations)
    }
}
