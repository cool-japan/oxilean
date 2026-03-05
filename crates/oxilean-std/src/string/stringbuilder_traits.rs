//! # StringBuilder - Trait Implementations
//!
//! This module contains trait implementations for `StringBuilder`.
//!
//! ## Implemented Traits
//!
//! - `Write`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::StringBuilder;
use std::fmt;

impl std::fmt::Write for StringBuilder {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.buf.push_str(s);
        Ok(())
    }
}
