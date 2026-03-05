//! # UnnecessaryParensRule - Trait Implementations
//!
//! This module contains trait implementations for `UnnecessaryParensRule`.
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

use super::types::UnnecessaryParensRule;

impl LintRule for UnnecessaryParensRule {
    fn id(&self) -> LintId {
        lint_ids::unnecessary_parens()
    }
    fn name(&self) -> &str {
        "unnecessary parentheses"
    }
    fn description(&self) -> &str {
        "detects unnecessary parentheses around expressions"
    }
    fn default_severity(&self) -> Severity {
        Severity::Hint
    }
    fn category(&self) -> LintCategory {
        LintCategory::Style
    }
    fn check_expr(&self, ctx: &mut LintContext<'_>, expr: &Located<SurfaceExpr>) {
        match &expr.value {
            SurfaceExpr::Ann(inner, _outer_ty) => {
                if let SurfaceExpr::Ann(_, _) = &inner.value {
                    ctx.emit(
                        LintDiagnostic::new(
                                self.id(),
                                Severity::Hint,
                                "double type annotation is unnecessary",
                                SourceRange::from_span(&expr.span),
                            )
                            .with_note(
                                "the inner type annotation already constrains the expression;                              remove the outer `(expr : T)`",
                            ),
                    );
                }
            }
            SurfaceExpr::AnonymousCtor(elems) if elems.len() == 1 => {
                if let SurfaceExpr::AnonymousCtor(_) = &elems[0].value {
                    ctx.emit(
                        LintDiagnostic::new(
                            self.id(),
                            Severity::Hint,
                            "nested single-element anonymous constructor is redundant",
                            SourceRange::from_span(&expr.span),
                        )
                        .with_note("use the inner constructor directly"),
                    );
                }
            }
            SurfaceExpr::Proj(base, _field) => {
                if let SurfaceExpr::Ann(_, _) = &base.value {
                    ctx.emit(
                        LintDiagnostic::new(
                                self.id(),
                                Severity::Hint,
                                "type annotation before field projection is usually unnecessary",
                                SourceRange::from_span(&expr.span),
                            )
                            .with_note(
                                "consider removing the type annotation and letting                              type inference resolve the field",
                            ),
                    );
                }
            }
            _ => {}
        }
    }
}
