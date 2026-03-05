//! # SuspiciousShadowRule - Trait Implementations
//!
//! This module contains trait implementations for `SuspiciousShadowRule`.
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

use super::types::SuspiciousShadowRule;

impl LintRule for SuspiciousShadowRule {
    fn id(&self) -> LintId {
        LintId::new("suspicious_shadow")
    }
    fn name(&self) -> &str {
        "suspicious shadow"
    }
    fn description(&self) -> &str {
        "detects suspicious variable shadowing"
    }
    fn default_severity(&self) -> Severity {
        Severity::Info
    }
    fn category(&self) -> LintCategory {
        LintCategory::Correctness
    }
    fn check_expr(&self, ctx: &mut LintContext<'_>, expr: &Located<SurfaceExpr>) {
        if let SurfaceExpr::Let(name, _, _, body) = &expr.value {
            if name.starts_with('_') {
                return;
            }
            if let SurfaceExpr::Let(inner_name, _, _, _) = &body.value {
                if inner_name == name {
                    ctx.emit(
                        LintDiagnostic::new(
                            self.id(),
                            Severity::Info,
                            format!(
                                "variable `{}` is shadowed immediately by another `let`",
                                name
                            ),
                            SourceRange::from_span(&expr.span),
                        )
                        .with_note("this may indicate a copy-paste error"),
                    );
                }
            }
        }
    }
}
