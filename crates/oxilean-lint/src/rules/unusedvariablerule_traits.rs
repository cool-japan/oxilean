//! # UnusedVariableRule - Trait Implementations
//!
//! This module contains trait implementations for `UnusedVariableRule`.
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

use super::types::UnusedVariableRule;

impl LintRule for UnusedVariableRule {
    fn id(&self) -> LintId {
        lint_ids::unused_variable()
    }
    fn name(&self) -> &str {
        "unused variable"
    }
    fn description(&self) -> &str {
        "detects variables that are declared but never used"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn category(&self) -> LintCategory {
        LintCategory::Correctness
    }
    fn check_decl(&self, ctx: &mut LintContext<'_>, decl: &Located<Decl>) {
        match &decl.value {
            Decl::Definition { name: _, val, .. } => {
                self.check_expr_for_unused(ctx, val);
            }
            Decl::Theorem { proof, .. } => {
                self.check_expr_for_unused(ctx, proof);
            }
            _ => {}
        }
    }
    fn check_expr(&self, ctx: &mut LintContext<'_>, expr: &Located<SurfaceExpr>) {
        self.check_expr_for_unused(ctx, expr);
    }
}
