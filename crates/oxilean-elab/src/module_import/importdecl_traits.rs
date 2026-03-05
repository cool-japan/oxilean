//! # ImportDecl - Trait Implementations
//!
//! This module contains trait implementations for `ImportDecl`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ImportDecl;
use std::fmt;

impl fmt::Display for ImportDecl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_public {
            write!(f, "export ")?;
        } else {
            write!(f, "import ")?;
        }
        write!(f, "{}", self.path)?;
        if let Some(ref names) = self.selective {
            let names_str: Vec<String> = names.iter().map(|n| format!("{}", n)).collect();
            write!(f, " ({})", names_str.join(", "))?;
        }
        if let Some(ref hidden) = self.hiding {
            let hidden_str: Vec<String> = hidden.iter().map(|n| format!("{}", n)).collect();
            write!(f, " hiding ({})", hidden_str.join(", "))?;
        }
        for (old, new) in &self.renamed {
            write!(f, " renaming ({} -> {})", old, new)?;
        }
        Ok(())
    }
}
