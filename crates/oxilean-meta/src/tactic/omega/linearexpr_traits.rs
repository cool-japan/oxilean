//! # LinearExpr - Trait Implementations
//!
//! This module contains trait implementations for `LinearExpr`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::LinearExpr;
use std::fmt;

impl fmt::Display for LinearExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.terms.is_empty() {
            return write!(f, "{}", self.constant);
        }
        let mut first = true;
        if self.constant != 0 {
            write!(f, "{}", self.constant)?;
            first = false;
        }
        for (name, coeff) in &self.terms {
            if !first {
                if *coeff > 0 {
                    write!(f, " + ")?;
                } else {
                    write!(f, " - ")?;
                }
            }
            let abs_coeff = coeff.unsigned_abs();
            if abs_coeff == 1 {
                write!(f, "{name}")?;
            } else {
                write!(f, "{abs_coeff}·{name}")?;
            }
            first = false;
        }
        Ok(())
    }
}
