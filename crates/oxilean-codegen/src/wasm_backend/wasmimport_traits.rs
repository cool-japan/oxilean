//! # WasmImport - Trait Implementations
//!
//! This module contains trait implementations for `WasmImport`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::WasmImport;
use std::fmt;

impl fmt::Display for WasmImport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"  (import "{}" "{}" {})"#,
            self.module, self.name, self.kind
        )
    }
}
