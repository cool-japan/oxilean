//! # AtiyahHirzebruchSS - Trait Implementations
//!
//! This module contains trait implementations for `AtiyahHirzebruchSS`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::AtiyahHirzebruchSS;
use std::fmt;

impl fmt::Display for AtiyahHirzebruchSS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AHSS({}; {})", self.space, self.spectrum)
    }
}
