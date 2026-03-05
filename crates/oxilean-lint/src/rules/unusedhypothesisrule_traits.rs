//! # UnusedHypothesisRule - Trait Implementations
//!
//! This module contains trait implementations for `UnusedHypothesisRule`.
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

use super::types::UnusedHypothesisRule;

impl LintRule for UnusedHypothesisRule {
    fn id(&self) -> LintId {
        lint_ids::unused_hypothesis()
    }
    fn name(&self) -> &str {
        "unused hypothesis"
    }
    fn description(&self) -> &str {
        "detects hypotheses in proofs that are introduced but never used"
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
        self.check_have_in_expr(ctx, proof, &decl.span);
    }
}
