//! # BvDecideStats - Trait Implementations
//!
//! This module contains trait implementations for `BvDecideStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::BvDecideStats;
use std::fmt;

impl fmt::Display for BvDecideStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "bv_decide statistics:")?;
        writeln!(f, "  SAT variables:    {}", self.vars)?;
        writeln!(f, "  CNF clauses:      {}", self.clauses)?;
        writeln!(f, "  Decisions:        {}", self.decisions)?;
        writeln!(f, "  Propagations:     {}", self.propagations)?;
        writeln!(f, "  Conflicts:        {}", self.conflicts)?;
        writeln!(f, "  Learned clauses:  {}", self.learned_clauses)?;
        writeln!(f, "  Restarts:         {}", self.restarts)?;
        writeln!(f, "  BV nodes:         {}", self.bv_nodes)?;
        writeln!(f, "  Encoding time:    {} ms", self.encoding_time_ms)?;
        writeln!(f, "  Solving time:     {} ms", self.solving_time_ms)?;
        write!(f, "  Total time:       {} ms", self.time_ms)
    }
}
