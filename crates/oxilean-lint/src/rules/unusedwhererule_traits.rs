//! # UnusedWhereRule - Trait Implementations
//!
//! This module contains trait implementations for `UnusedWhereRule`.
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

use super::types::UnusedWhereRule;

impl LintRule for UnusedWhereRule {
    fn id(&self) -> LintId {
        lint_ids::unused_where()
    }
    fn name(&self) -> &str {
        "unused where clause"
    }
    fn description(&self) -> &str {
        "detects where clause definitions that are never used"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn category(&self) -> LintCategory {
        LintCategory::Correctness
    }
    fn check_decl(&self, ctx: &mut LintContext<'_>, decl: &Located<Decl>) {
        match &decl.value {
            Decl::Definition {
                val, where_clauses, ..
            } => {
                if !where_clauses.is_empty() {
                    let refs = collect_var_refs(&val.value);
                    for wc in where_clauses {
                        if !refs.contains(&wc.name) {
                            ctx.emit(
                                LintDiagnostic::new(
                                    self.id(),
                                    Severity::Warning,
                                    format!("where clause `{}` is never used in the body", wc.name),
                                    SourceRange::from_span(&decl.span),
                                )
                                .with_note("consider removing this where clause"),
                            );
                        }
                    }
                }
            }
            Decl::Theorem {
                proof,
                where_clauses,
                ..
            } => {
                if !where_clauses.is_empty() {
                    let refs = collect_var_refs(&proof.value);
                    for wc in where_clauses {
                        if !refs.contains(&wc.name) {
                            ctx.emit(
                                LintDiagnostic::new(
                                    self.id(),
                                    Severity::Warning,
                                    format!(
                                        "where clause `{}` is never used in the proof",
                                        wc.name
                                    ),
                                    SourceRange::from_span(&decl.span),
                                )
                                .with_note("consider removing this where clause"),
                            );
                        }
                    }
                }
            }
            _ => {}
        }
    }
}
