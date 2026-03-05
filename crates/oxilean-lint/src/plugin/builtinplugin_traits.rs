//! # BuiltinPlugin - Trait Implementations
//!
//! This module contains trait implementations for `BuiltinPlugin`.
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
use crate::rules::{
    DeadCodeRule, DeprecatedApiRule, MissingDocRule, NamingConventionRule, RedundantPatternRule,
    SimplifiableExprRule, StyleRule, UnreachableCodeRule, UnusedImportRule, UnusedVariableRule,
};

use super::functions::LintPlugin;
use super::types::BuiltinPlugin;

impl Default for BuiltinPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl LintPlugin for BuiltinPlugin {
    fn name(&self) -> &str {
        "builtin"
    }
    fn version(&self) -> &str {
        "1.0.0"
    }
    fn rules(&self) -> Vec<Box<dyn LintRule>> {
        vec![
            Box::new(UnusedVariableRule::new()),
            Box::new(UnusedImportRule::new()),
            Box::new(DeprecatedApiRule::new()),
            Box::new(RedundantPatternRule),
            Box::new(SimplifiableExprRule),
            Box::new(MissingDocRule::new()),
            Box::new(NamingConventionRule::new()),
            Box::new(DeadCodeRule::new()),
            Box::new(UnreachableCodeRule),
            Box::new(StyleRule::new()),
        ]
    }
}
