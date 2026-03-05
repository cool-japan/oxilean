//! # UnreachableCodeRule - Trait Implementations
//!
//! This module contains trait implementations for `UnreachableCodeRule`.
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

use super::types::UnreachableCodeRule;

impl LintRule for UnreachableCodeRule {
    fn id(&self) -> LintId {
        lint_ids::unreachable_code()
    }
    fn name(&self) -> &str {
        "unreachable code"
    }
    fn description(&self) -> &str {
        "detects code that can never be reached"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn category(&self) -> LintCategory {
        LintCategory::Correctness
    }
    fn check_expr(&self, ctx: &mut LintContext<'_>, expr: &Located<SurfaceExpr>) {
        self.check_unreachable_branches(ctx, expr);
    }
    fn check_decl(&self, ctx: &mut LintContext<'_>, decl: &Located<Decl>) {
        match &decl.value {
            Decl::Definition { val, .. } => {
                self.walk_for_unreachable(ctx, val);
            }
            Decl::Theorem { proof, .. } => {
                self.walk_for_unreachable(ctx, proof);
            }
            _ => {}
        }
    }
}
