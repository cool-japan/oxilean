//! # EmptyMatchRule - Trait Implementations
//!
//! This module contains trait implementations for `EmptyMatchRule`.
//!
//! ## Implemented Traits
//!
//! - `LintRule`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::framework::{
    collect_var_refs, is_camel_case, is_pascal_case, is_snake_case, lint_ids, to_snake_case,
    AutoFix, LintCategory, LintContext, LintDiagnostic, LintId, LintRule, Severity, SourceRange,
};
use oxilean_parse::{Binder, Decl, DoAction, Located, MatchArm, Pattern, Span, SurfaceExpr};

use super::types::EmptyMatchRule;

impl LintRule for EmptyMatchRule {
    fn id(&self) -> LintId {
        lint_ids::empty_match()
    }
    fn name(&self) -> &str {
        "empty match"
    }
    fn description(&self) -> &str {
        "detects match expressions with no arms"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn category(&self) -> LintCategory {
        LintCategory::Correctness
    }
    fn check_expr(&self, ctx: &mut LintContext<'_>, expr: &Located<SurfaceExpr>) {
        if let SurfaceExpr::Match(_, arms) = &expr.value {
            if arms.is_empty() {
                ctx.emit(
                    LintDiagnostic::new(
                        self.id(),
                        Severity::Warning,
                        "match expression has no arms",
                        SourceRange::from_span(&expr.span),
                    )
                    .with_note(
                        "an empty match is only valid for types with no constructors (e.g., Empty)",
                    ),
                );
            }
        }
    }
}
