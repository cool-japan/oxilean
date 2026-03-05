//! # RedundantTypeAnnotationRule - Trait Implementations
//!
//! This module contains trait implementations for `RedundantTypeAnnotationRule`.
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

use super::types::RedundantTypeAnnotationRule;

impl LintRule for RedundantTypeAnnotationRule {
    fn id(&self) -> LintId {
        lint_ids::redundant_type_annotation()
    }
    fn name(&self) -> &str {
        "redundant type annotation"
    }
    fn description(&self) -> &str {
        "detects type annotations that could be inferred"
    }
    fn default_severity(&self) -> Severity {
        Severity::Hint
    }
    fn category(&self) -> LintCategory {
        LintCategory::Style
    }
    fn check_expr(&self, ctx: &mut LintContext<'_>, expr: &Located<SurfaceExpr>) {
        if let SurfaceExpr::Ann(inner, ty) = &expr.value {
            if let SurfaceExpr::Lit(_) = &inner.value {
                if let SurfaceExpr::Var(ty_name) = &ty.value {
                    if ty_name == "Nat" || ty_name == "String" {
                        ctx.emit(
                            LintDiagnostic::new(
                                self.id(),
                                Severity::Hint,
                                format!("type annotation `{}` is redundant for a literal", ty_name),
                                SourceRange::from_span(&expr.span),
                            )
                            .with_note("the type can be inferred from the literal"),
                        );
                    }
                }
            }
        }
    }
}
