//! # RedundantAssumptionRule - Trait Implementations
//!
//! This module contains trait implementations for `RedundantAssumptionRule`.
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

use super::types::RedundantAssumptionRule;

impl LintRule for RedundantAssumptionRule {
    fn id(&self) -> LintId {
        lint_ids::redundant_assumption()
    }
    fn name(&self) -> &str {
        "redundant assumption"
    }
    fn description(&self) -> &str {
        "detects redundant `have` bindings where a hypothesis is assigned to itself or shadowed"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn category(&self) -> LintCategory {
        LintCategory::Correctness
    }
    fn check_decl(&self, ctx: &mut LintContext<'_>, decl: &Located<Decl>) {
        let proof = match &decl.value {
            Decl::Theorem { proof, .. } => proof,
            _ => return,
        };
        self.check_expr_for_redundancy(ctx, proof, &decl.span);
    }
}
