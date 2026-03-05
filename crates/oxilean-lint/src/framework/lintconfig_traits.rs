//! # LintConfig - Trait Implementations
//!
//! This module contains trait implementations for `LintConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet};

use super::types::{LintConfig, Severity};

impl Default for LintConfig {
    fn default() -> Self {
        Self {
            enabled: HashSet::new(),
            disabled: HashSet::new(),
            severity_overrides: HashMap::new(),
            max_diagnostics: 500,
            suggest_fixes: true,
            suppression_patterns: Vec::new(),
            report_dead_code: true,
            enforce_naming: true,
            enforce_style: true,
            enforce_docs: false,
            min_severity: Severity::Hint,
        }
    }
}
