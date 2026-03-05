//! # Polynomial - Trait Implementations
//!
//! This module contains trait implementations for `Polynomial`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::Polynomial;
use std::fmt;

impl std::fmt::Display for Polynomial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.terms.is_empty() {
            write!(f, "0")?;
            return Ok(());
        }
        for (idx, t) in self.terms.iter().enumerate() {
            if idx > 0 && t.coeff_num > 0 {
                write!(f, " + ")?;
            } else if t.coeff_num < 0 {
                write!(f, " - ")?;
            }
            let abs_num = t.coeff_num.unsigned_abs();
            if t.coeff_den == 1 {
                if abs_num != 1 || t.monomial.exponents.iter().all(|&e| e == 0) {
                    write!(f, "{}", abs_num)?;
                }
            } else {
                write!(f, "{}/{}", abs_num, t.coeff_den)?;
            }
            if !t.monomial.exponents.iter().all(|&e| e == 0) {
                write!(f, "{}", t.monomial)?;
            }
        }
        Ok(())
    }
}
