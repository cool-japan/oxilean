//! # Deferred - Trait Implementations
//!
//! This module contains trait implementations for `Deferred`.
//!
//! ## Implemented Traits
//!
//! - `Debug`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::Deferred;
use std::fmt;

impl<A: 'static + std::fmt::Debug> std::fmt::Debug for Deferred<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Deferred(<fn>)")
    }
}
