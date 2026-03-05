//! # DeprecatedTacticRule - Trait Implementations
//!
//! This module contains trait implementations for `DeprecatedTacticRule`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `LintRule`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::framework::{
    collect_var_refs, is_camel_case, is_pascal_case, is_snake_case, lint_ids, to_snake_case,
    AutoFix, LintCategory, LintContext, LintDiagnostic, LintId, LintRule, Severity, SourceRange,
};
use oxilean_parse::{Binder, Decl, DoAction, Located, MatchArm, Pattern, Span, SurfaceExpr};

use super::types::DeprecatedTacticRule;

impl Default for DeprecatedTacticRule {
    fn default() -> Self {
        Self::new()
    }
}

impl LintRule for DeprecatedTacticRule {
    fn id(&self) -> LintId {
        lint_ids::deprecated_tactic()
    }
    fn name(&self) -> &str {
        "deprecated tactic"
    }
    fn description(&self) -> &str {
        "warns when deprecated or discouraged tactics are used in proof blocks"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn category(&self) -> LintCategory {
        LintCategory::Deprecated
    }
    fn check_decl(&self, ctx: &mut LintContext<'_>, decl: &Located<Decl>) {
        let proof = match &decl.value {
            Decl::Theorem { name, proof, .. } => {
                let _ = name;
                proof
            }
            _ => return,
        };
        self.scan_for_deprecated_tactics(ctx, proof, &decl.span);
    }
}
