//! # Poly - Trait Implementations
//!
//! This module contains trait implementations for `Poly`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::Poly;

impl std::fmt::Display for Poly {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let t = self.trim();
        if t.coeffs.is_empty() {
            return write!(f, "0");
        }
        let mut parts = Vec::new();
        for (i, &c) in t.coeffs.iter().enumerate().rev() {
            if c == 0 {
                continue;
            }
            let term = if i == 0 {
                format!("{}", c)
            } else if i == 1 {
                if c == 1 {
                    "x".to_string()
                } else {
                    format!("{}x", c)
                }
            } else if c == 1 {
                format!("x^{}", i)
            } else {
                format!("{}x^{}", c, i)
            };
            parts.push(term);
        }
        write!(f, "{}", parts.join(" + "))
    }
}
