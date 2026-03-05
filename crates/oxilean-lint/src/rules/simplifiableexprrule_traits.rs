//! # SimplifiableExprRule - Trait Implementations
//!
//! This module contains trait implementations for `SimplifiableExprRule`.
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

use super::types::SimplifiableExprRule;

impl LintRule for SimplifiableExprRule {
    fn id(&self) -> LintId {
        lint_ids::simplifiable_expr()
    }
    fn name(&self) -> &str {
        "simplifiable expression"
    }
    fn description(&self) -> &str {
        "detects expressions that can be simplified"
    }
    fn default_severity(&self) -> Severity {
        Severity::Hint
    }
    fn category(&self) -> LintCategory {
        LintCategory::Complexity
    }
    fn check_expr(&self, ctx: &mut LintContext<'_>, expr: &Located<SurfaceExpr>) {
        self.check_simplifiable(ctx, expr);
    }
    fn check_decl(&self, ctx: &mut LintContext<'_>, decl: &Located<Decl>) {
        match &decl.value {
            Decl::Definition { val, .. } => {
                self.walk_expr(ctx, val);
            }
            Decl::Theorem { proof, .. } => {
                self.walk_expr(ctx, proof);
            }
            _ => {}
        }
    }
}
