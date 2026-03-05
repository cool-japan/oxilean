//! # ClosureStats - Trait Implementations
//!
//! This module contains trait implementations for `ClosureStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{Closure, ClosureStats};
use std::fmt;

impl fmt::Display for ClosureStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Closure Statistics:")?;
        writeln!(f, "  Closures created:   {}", self.closures_created)?;
        writeln!(f, "  PAPs created:       {}", self.paps_created)?;
        writeln!(f, "  Exact calls:        {}", self.exact_calls)?;
        writeln!(f, "  Under-applications: {}", self.under_applications)?;
        writeln!(f, "  Over-applications:  {}", self.over_applications)?;
        writeln!(f, "  Tail calls:         {}", self.tail_calls)?;
        writeln!(f, "  Direct calls:       {}", self.direct_calls)?;
        writeln!(f, "  Built-in calls:     {}", self.builtin_calls)?;
        writeln!(f, "  Peak stack depth:   {}", self.peak_stack_depth)
    }
}
