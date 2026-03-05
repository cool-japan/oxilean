//! # MissingDocRule - Trait Implementations
//!
//! This module contains trait implementations for `MissingDocRule`.
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

use super::types::MissingDocRule;

impl LintRule for MissingDocRule {
    fn id(&self) -> LintId {
        lint_ids::missing_doc()
    }
    fn name(&self) -> &str {
        "missing documentation"
    }
    fn description(&self) -> &str {
        "detects public declarations without documentation comments"
    }
    fn default_severity(&self) -> Severity {
        Severity::Info
    }
    fn category(&self) -> LintCategory {
        LintCategory::Documentation
    }
    fn check_decl(&self, ctx: &mut LintContext<'_>, decl: &Located<Decl>) {
        let (name, should_check) = match &decl.value {
            Decl::Definition { name, .. } => (name.as_str(), self.require_def_docs),
            Decl::Theorem { name, .. } => (name.as_str(), self.require_theorem_docs),
            Decl::Inductive { name, .. } => (name.as_str(), self.require_inductive_docs),
            Decl::Structure { name, .. } => (name.as_str(), self.require_structure_docs),
            Decl::ClassDecl { name, .. } => (name.as_str(), true),
            _ => return,
        };
        if !should_check {
            return;
        }
        if name.starts_with('_') {
            return;
        }
        if !self.has_doc_comment(ctx.source, &decl.span) {
            ctx.emit(
                LintDiagnostic::new(
                    self.id(),
                    Severity::Info,
                    format!("missing documentation for `{}`", name),
                    SourceRange::from_span(&decl.span),
                )
                .with_note("add a `/-- ... -/` doc comment before the declaration"),
            );
        }
    }
}
