//! # Monomial - Trait Implementations
//!
//! This module contains trait implementations for `Monomial`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::Monomial;
use std::fmt;

impl std::fmt::Display for Monomial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.exponents.iter().all(|&e| e == 0) {
            write!(f, "1")?;
            return Ok(());
        }
        for (i, &e) in self.exponents.iter().enumerate() {
            if e > 0 {
                write!(f, "x_{}", i + 1)?;
                if e > 1 {
                    write!(f, "^{}", e)?;
                }
            }
        }
        Ok(())
    }
}
