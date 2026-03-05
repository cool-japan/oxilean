//! # MissingDocstringRule - Trait Implementations
//!
//! This module contains trait implementations for `MissingDocstringRule`.
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

use super::types::MissingDocstringRule;

impl LintRule for MissingDocstringRule {
    fn id(&self) -> LintId {
        lint_ids::missing_docstring()
    }
    fn name(&self) -> &str {
        "missing docstring"
    }
    fn description(&self) -> &str {
        "warns on public declarations that lack a doc-comment"
    }
    fn default_severity(&self) -> Severity {
        Severity::Info
    }
    fn category(&self) -> LintCategory {
        LintCategory::Documentation
    }
    fn check_decl(&self, ctx: &mut LintContext<'_>, decl: &Located<Decl>) {
        let name = match &decl.value {
            Decl::Theorem { name, .. }
            | Decl::Definition { name, .. }
            | Decl::Inductive { name, .. }
            | Decl::Axiom { name, .. } => name.as_str(),
            _ => return,
        };
        let start = decl.span.start;
        if !Self::has_doc_comment_before(ctx.source, start) {
            ctx.emit(
                LintDiagnostic::new(
                    self.id(),
                    Severity::Info,
                    format!("public declaration `{}` is missing a doc-comment", name),
                    SourceRange::from_span(&decl.span),
                )
                .with_note("add a `--! ...` or `/-- ... -/` doc-comment before the declaration"),
            );
        }
    }
}
