//! # BashHereDoc - Trait Implementations
//!
//! This module contains trait implementations for `BashHereDoc`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::BashHereDoc;
use std::fmt;

impl fmt::Display for BashHereDoc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let delim_str = if self.no_expand {
            format!("'{}'", self.delimiter)
        } else {
            self.delimiter.clone()
        };
        let op = if self.strip_tabs { "<<-" } else { "<<" };
        writeln!(f, "{}{}", op, delim_str)?;
        for line in &self.content {
            writeln!(f, "{}", line)?;
        }
        write!(f, "{}", self.delimiter)
    }
}
