//! # EllipticCurve - Trait Implementations
//!
//! This module contains trait implementations for `EllipticCurve`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::EllipticCurve;
use std::fmt;

impl std::fmt::Display for EllipticCurve {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sign_b = if self.b >= 0 { "+" } else { "-" };
        write!(
            f,
            "y² = x³ {} {} x {} {} over {}",
            if self.a >= 0 { "+" } else { "-" },
            self.a.unsigned_abs(),
            sign_b,
            self.b.unsigned_abs(),
            self.field
        )
    }
}
