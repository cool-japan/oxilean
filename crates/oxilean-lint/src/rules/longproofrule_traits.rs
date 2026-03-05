//! # LongProofRule - Trait Implementations
//!
//! This module contains trait implementations for `LongProofRule`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `LintRule`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::framework::{
    collect_var_refs, is_camel_case, is_pascal_case, is_snake_case, lint_ids, to_snake_case,
    AutoFix, LintCategory, LintContext, LintDiagnostic, LintId, LintRule, Severity, SourceRange,
};
use oxilean_parse::{Binder, Decl, DoAction, Located, MatchArm, Pattern, Span, SurfaceExpr};

use super::types::LongProofRule;

impl Default for LongProofRule {
    fn default() -> Self {
        Self::new()
    }
}

impl LintRule for LongProofRule {
    fn id(&self) -> LintId {
        lint_ids::long_proof()
    }
    fn name(&self) -> &str {
        "long proof"
    }
    fn description(&self) -> &str {
        "warns when a proof block exceeds the recommended tactic-line count"
    }
    fn default_severity(&self) -> Severity {
        Severity::Info
    }
    fn category(&self) -> LintCategory {
        LintCategory::Complexity
    }
    fn check_decl(&self, ctx: &mut LintContext<'_>, decl: &Located<Decl>) {
        let (name, proof) = match &decl.value {
            Decl::Theorem { name, proof, .. } => (name.as_str(), proof),
            _ => return,
        };
        let count = self.count_tactic_lines(&proof.value);
        if count > self.max_tactic_lines {
            ctx.emit(
                LintDiagnostic::new(
                        self.id(),
                        Severity::Info,
                        format!(
                            "proof of `{}` has {} tactic steps (max recommended: {})",
                            name, count, self.max_tactic_lines
                        ),
                        SourceRange::from_span(&decl.span),
                    )
                    .with_note(
                        "consider splitting into smaller helper lemmas using `have` or auxiliary theorems",
                    ),
            );
        }
    }
}
