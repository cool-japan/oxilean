//! # ConversionStats - Trait Implementations
//!
//! This module contains trait implementations for `ConversionStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::ConversionStats;
use std::fmt;

impl fmt::Display for ConversionStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Conversion Statistics:")?;
        writeln!(f, "  Expressions visited:   {}", self.exprs_visited)?;
        writeln!(
            f,
            "  Let bindings:          {}",
            self.let_bindings_generated
        )?;
        writeln!(f, "  Lambdas lifted:        {}", self.lambdas_lifted)?;
        writeln!(f, "  Proofs erased:         {}", self.proofs_erased)?;
        writeln!(f, "  Types erased:          {}", self.types_erased)?;
        writeln!(f, "  Closures converted:    {}", self.closures_converted)?;
        writeln!(f, "  Max depth:             {}", self.max_depth)?;
        writeln!(f, "  Tail calls detected:   {}", self.tail_calls_detected)?;
        writeln!(f, "  Fresh vars allocated:  {}", self.fresh_vars_allocated)?;
        writeln!(f, "  Free var computations: {}", self.free_var_computations)
    }
}
