//! # ExampleCustomPlugin - Trait Implementations
//!
//! This module contains trait implementations for `ExampleCustomPlugin`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `LintPlugin`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::framework::{
    LintCategory, LintContext, LintDiagnostic, LintId, LintRule, Severity, SourceRange,
};

use super::functions::LintPlugin;
use super::types::{ExampleCustomPlugin, NoSorryRule, PreferOmegaRule};

impl Default for ExampleCustomPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl LintPlugin for ExampleCustomPlugin {
    fn name(&self) -> &str {
        "example_custom"
    }
    fn version(&self) -> &str {
        "0.1.1"
    }
    fn rules(&self) -> Vec<Box<dyn LintRule>> {
        vec![Box::new(NoSorryRule), Box::new(PreferOmegaRule)]
    }
}
