//! # DeprecatedApiRule - Trait Implementations
//!
//! This module contains trait implementations for `DeprecatedApiRule`.
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

use super::types::DeprecatedApiRule;

impl LintRule for DeprecatedApiRule {
    fn id(&self) -> LintId {
        lint_ids::deprecated_api()
    }
    fn name(&self) -> &str {
        "deprecated API"
    }
    fn description(&self) -> &str {
        "detects usage of deprecated definitions"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn category(&self) -> LintCategory {
        LintCategory::Deprecated
    }
    fn check_decl(&self, ctx: &mut LintContext<'_>, decl: &Located<Decl>) {
        match &decl.value {
            Decl::Definition { val, ty, .. } => {
                if let Some(ty_expr) = ty {
                    self.check_name_in_expr(ctx, ty_expr);
                }
                self.check_name_in_expr(ctx, val);
            }
            Decl::Theorem { ty, proof, .. } => {
                self.check_name_in_expr(ctx, ty);
                self.check_name_in_expr(ctx, proof);
            }
            _ => {}
        }
    }
    fn check_expr(&self, ctx: &mut LintContext<'_>, expr: &Located<SurfaceExpr>) {
        self.check_name_in_expr(ctx, expr);
    }
}
