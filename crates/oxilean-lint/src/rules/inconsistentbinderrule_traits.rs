//! # InconsistentBinderRule - Trait Implementations
//!
//! This module contains trait implementations for `InconsistentBinderRule`.
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

use super::types::InconsistentBinderRule;

impl LintRule for InconsistentBinderRule {
    fn id(&self) -> LintId {
        LintId::new("inconsistent_binder")
    }
    fn name(&self) -> &str {
        "inconsistent binder style"
    }
    fn description(&self) -> &str {
        "detects inconsistent binder styles within a declaration"
    }
    fn default_severity(&self) -> Severity {
        Severity::Hint
    }
    fn category(&self) -> LintCategory {
        LintCategory::Style
    }
    fn check_expr(&self, ctx: &mut LintContext<'_>, expr: &Located<SurfaceExpr>) {
        if let SurfaceExpr::Lam(binders, _) | SurfaceExpr::Pi(binders, _) = &expr.value {
            let has_type_count = binders.iter().filter(|b| b.ty.is_some()).count();
            if has_type_count > 0 && has_type_count < binders.len() {
                ctx.emit(
                    LintDiagnostic::new(
                        self.id(),
                        Severity::Hint,
                        "inconsistent binder annotations: some have types, some don't",
                        SourceRange::from_span(&expr.span),
                    )
                    .with_note("consider adding type annotations to all binders"),
                );
            }
        }
    }
}
