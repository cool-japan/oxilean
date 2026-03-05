//! # RedundantPatternRule - Trait Implementations
//!
//! This module contains trait implementations for `RedundantPatternRule`.
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

use super::types::RedundantPatternRule;

impl LintRule for RedundantPatternRule {
    fn id(&self) -> LintId {
        lint_ids::redundant_pattern()
    }
    fn name(&self) -> &str {
        "redundant pattern"
    }
    fn description(&self) -> &str {
        "detects redundant or unnecessary pattern matching"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn category(&self) -> LintCategory {
        LintCategory::Complexity
    }
    fn check_expr(&self, ctx: &mut LintContext<'_>, expr: &Located<SurfaceExpr>) {
        if let SurfaceExpr::Match(_scrutinee, arms) = &expr.value {
            self.check_match_arms(ctx, arms, &expr.span);
        }
    }
    fn check_decl(&self, ctx: &mut LintContext<'_>, decl: &Located<Decl>) {
        match &decl.value {
            Decl::Definition { val, .. } => {
                self.walk_expr_for_matches(ctx, val);
            }
            Decl::Theorem { proof, .. } => {
                self.walk_expr_for_matches(ctx, proof);
            }
            _ => {}
        }
    }
}
