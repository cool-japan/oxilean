//! # SorryInProofRule - Trait Implementations
//!
//! This module contains trait implementations for `SorryInProofRule`.
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

use super::types::SorryInProofRule;

impl LintRule for SorryInProofRule {
    fn id(&self) -> LintId {
        lint_ids::sorry_in_proof()
    }
    fn name(&self) -> &str {
        "sorry in proof"
    }
    fn description(&self) -> &str {
        "detects theorems or definitions that use `sorry` as a proof placeholder"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn category(&self) -> LintCategory {
        LintCategory::Correctness
    }
    fn check_decl(&self, ctx: &mut LintContext<'_>, decl: &Located<Decl>) {
        match &decl.value {
            Decl::Theorem { name, proof, .. } => {
                if Self::contains_sorry(&proof.value) {
                    ctx.emit(
                        LintDiagnostic::new(
                            self.id(),
                            Severity::Warning,
                            format!("theorem `{}` uses `sorry` — proof is incomplete", name),
                            SourceRange::from_span(&decl.span),
                        )
                        .with_note(
                            "replace `sorry` with a real proof before marking this as complete",
                        ),
                    );
                }
            }
            Decl::Definition { name, val, .. } => {
                if Self::contains_sorry(&val.value) {
                    ctx.emit(
                        LintDiagnostic::new(
                            self.id(),
                            Severity::Warning,
                            format!("definition `{}` uses `sorry` — body is incomplete", name),
                            SourceRange::from_span(&decl.span),
                        )
                        .with_note("replace `sorry` with a real implementation"),
                    );
                }
            }
            _ => {}
        }
    }
}
