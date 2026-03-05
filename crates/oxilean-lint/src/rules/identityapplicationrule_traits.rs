//! # IdentityApplicationRule - Trait Implementations
//!
//! This module contains trait implementations for `IdentityApplicationRule`.
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

use super::types::IdentityApplicationRule;

impl LintRule for IdentityApplicationRule {
    fn id(&self) -> LintId {
        lint_ids::identity_application()
    }
    fn name(&self) -> &str {
        "identity application"
    }
    fn description(&self) -> &str {
        "detects applications of identity functions"
    }
    fn default_severity(&self) -> Severity {
        Severity::Hint
    }
    fn category(&self) -> LintCategory {
        LintCategory::Complexity
    }
    fn check_expr(&self, ctx: &mut LintContext<'_>, expr: &Located<SurfaceExpr>) {
        if let SurfaceExpr::App(f, _arg) = &expr.value {
            if let SurfaceExpr::Var(name) = &f.value {
                if name == "id" || name == "Function.id" {
                    ctx.emit(
                        LintDiagnostic::new(
                            self.id(),
                            Severity::Hint,
                            "application of identity function can be removed",
                            SourceRange::from_span(&expr.span),
                        )
                        .with_note("replace `id x` with `x`"),
                    );
                }
            }
        }
    }
}
