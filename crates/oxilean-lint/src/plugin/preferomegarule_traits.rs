//! # PreferOmegaRule - Trait Implementations
//!
//! This module contains trait implementations for `PreferOmegaRule`.
//!
//! ## Implemented Traits
//!
//! - `LintRule`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::framework::{
    LintCategory, LintContext, LintDiagnostic, LintId, LintRule, Severity, SourceRange,
};
use oxilean_parse::{Decl, Located, SurfaceExpr};

use super::types::PreferOmegaRule;

impl LintRule for PreferOmegaRule {
    fn id(&self) -> LintId {
        LintId::new("prefer_omega")
    }
    fn name(&self) -> &str {
        "prefer omega"
    }
    fn description(&self) -> &str {
        "suggests `omega` tactic for linear arithmetic goals instead of manual proofs"
    }
    fn default_severity(&self) -> Severity {
        Severity::Hint
    }
    fn category(&self) -> LintCategory {
        LintCategory::Style
    }
    fn check_decl(&self, ctx: &mut LintContext<'_>, decl: &Located<Decl>) {
        let src = format!("{:?}", decl.value);
        if src.contains("linarith") {
            ctx.emit(LintDiagnostic::new(
                self.id(),
                Severity::Hint,
                "consider using `omega` instead of `linarith` for linear arithmetic".to_string(),
                SourceRange::from_span(&decl.span),
            ));
        }
    }
}
