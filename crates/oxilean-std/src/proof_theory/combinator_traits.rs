//! # Combinator - Trait Implementations
//!
//! This module contains trait implementations for `Combinator`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::Combinator;
use std::fmt;

impl std::fmt::Display for Combinator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Combinator::S => write!(f, "S"),
            Combinator::K => write!(f, "K"),
            Combinator::I => write!(f, "I"),
            Combinator::App(func, arg) => write!(f, "({} {})", func, arg),
        }
    }
}
