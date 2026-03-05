//! # SchedulerStats - Trait Implementations
//!
//! This module contains trait implementations for `SchedulerStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{Scheduler, SchedulerStats};
use std::fmt;

impl fmt::Display for SchedulerStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Scheduler Statistics:")?;
        writeln!(f, "  Tasks created:     {}", self.tasks_created)?;
        writeln!(f, "  Tasks completed:   {}", self.tasks_completed)?;
        writeln!(f, "  Tasks failed:      {}", self.tasks_failed)?;
        writeln!(f, "  Tasks cancelled:   {}", self.tasks_cancelled)?;
        writeln!(f, "  Total steals:      {}", self.total_steals)?;
        writeln!(f, "  Steal attempts:    {}", self.steal_attempts)?;
        writeln!(f, "  Idle cycles:       {}", self.idle_cycles)?;
        writeln!(f, "  Peak active:       {}", self.peak_active_tasks)?;
        writeln!(f, "  Scheduling rounds: {}", self.scheduling_rounds)
    }
}
