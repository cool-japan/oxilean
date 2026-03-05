//! # NoSorryRule - Trait Implementations
//!
//! This module contains trait implementations for `NoSorryRule`.
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

use super::types::NoSorryRule;

impl LintRule for NoSorryRule {
    fn id(&self) -> LintId {
        LintId::new("no_sorry")
    }
    fn name(&self) -> &str {
        "no sorry"
    }
    fn description(&self) -> &str {
        "flags the use of `sorry` which admits goals without proof"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn category(&self) -> LintCategory {
        LintCategory::Correctness
    }
    fn check_decl(&self, ctx: &mut LintContext<'_>, decl: &Located<Decl>) {
        let src = format!("{:?}", decl.value);
        if src.contains("sorry") || src.contains("Sorry") {
            ctx.emit(LintDiagnostic::new(
                self.id(),
                Severity::Warning,
                "proof uses `sorry` — goal is admitted without verification".to_string(),
                SourceRange::from_span(&decl.span),
            ));
        }
    }
    fn check_expr(&self, ctx: &mut LintContext<'_>, expr: &Located<SurfaceExpr>) {
        let src = format!("{:?}", expr.value);
        if src.contains("sorry") || src.contains("Sorry") {
            ctx.emit(LintDiagnostic::new(
                self.id(),
                Severity::Warning,
                "expression contains `sorry` — goal is admitted without verification".to_string(),
                SourceRange::from_span(&expr.span),
            ));
        }
    }
}
