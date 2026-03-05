//! # LargePatternMatchRule - Trait Implementations
//!
//! This module contains trait implementations for `LargePatternMatchRule`.
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

use super::types::LargePatternMatchRule;

impl LintRule for LargePatternMatchRule {
    fn id(&self) -> LintId {
        LintId::new("large_pattern_match")
    }
    fn name(&self) -> &str {
        "large pattern match"
    }
    fn description(&self) -> &str {
        "detects match expressions with too many arms"
    }
    fn default_severity(&self) -> Severity {
        Severity::Info
    }
    fn category(&self) -> LintCategory {
        LintCategory::Complexity
    }
    fn check_expr(&self, ctx: &mut LintContext<'_>, expr: &Located<SurfaceExpr>) {
        if let SurfaceExpr::Match(_, arms) = &expr.value {
            if arms.len() > self.max_arms {
                ctx.emit(
                    LintDiagnostic::new(
                        self.id(),
                        Severity::Info,
                        format!(
                            "match expression has {} arms (max recommended: {})",
                            arms.len(),
                            self.max_arms
                        ),
                        SourceRange::from_span(&expr.span),
                    )
                    .with_note("consider refactoring into helper functions"),
                );
            }
        }
    }
}
