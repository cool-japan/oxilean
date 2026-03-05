//! # ComplexExprRule - Trait Implementations
//!
//! This module contains trait implementations for `ComplexExprRule`.
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

use super::types::ComplexExprRule;

impl LintRule for ComplexExprRule {
    fn id(&self) -> LintId {
        LintId::new("complex_expr")
    }
    fn name(&self) -> &str {
        "complex expression"
    }
    fn description(&self) -> &str {
        "detects overly complex expressions with deep nesting"
    }
    fn default_severity(&self) -> Severity {
        Severity::Info
    }
    fn category(&self) -> LintCategory {
        LintCategory::Complexity
    }
    fn check_decl(&self, ctx: &mut LintContext<'_>, decl: &Located<Decl>) {
        let expr = match &decl.value {
            Decl::Definition { val, .. } => Some(val),
            Decl::Theorem { proof, .. } => Some(proof),
            _ => None,
        };
        if let Some(expr) = expr {
            let depth = self.measure_depth(&expr.value);
            if depth > self.max_depth {
                ctx.emit(
                    LintDiagnostic::new(
                        self.id(),
                        Severity::Info,
                        format!(
                            "expression nesting depth is {} (max recommended: {})",
                            depth, self.max_depth
                        ),
                        SourceRange::from_span(&decl.span),
                    )
                    .with_note("consider breaking this into smaller helper definitions"),
                );
            }
        }
    }
}
