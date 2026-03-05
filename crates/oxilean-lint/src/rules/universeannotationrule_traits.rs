//! # UniverseAnnotationRule - Trait Implementations
//!
//! This module contains trait implementations for `UniverseAnnotationRule`.
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
use std::collections::{HashMap, HashSet};

use super::types::UniverseAnnotationRule;

impl LintRule for UniverseAnnotationRule {
    fn id(&self) -> LintId {
        LintId::new("universe_annotation")
    }
    fn name(&self) -> &str {
        "universe annotation"
    }
    fn description(&self) -> &str {
        "checks for potentially incorrect universe annotations"
    }
    fn default_severity(&self) -> Severity {
        Severity::Info
    }
    fn category(&self) -> LintCategory {
        LintCategory::Correctness
    }
    fn check_decl(&self, ctx: &mut LintContext<'_>, decl: &Located<Decl>) {
        let univ_params = match &decl.value {
            Decl::Definition { univ_params, .. } => univ_params,
            Decl::Theorem { univ_params, .. } => univ_params,
            Decl::Inductive { univ_params, .. } => univ_params,
            _ => return,
        };
        if univ_params.len() > 5 {
            ctx.emit(
                LintDiagnostic::new(
                    self.id(),
                    Severity::Info,
                    format!(
                        "declaration has {} universe parameters, which is unusual",
                        univ_params.len()
                    ),
                    SourceRange::from_span(&decl.span),
                )
                .with_note("most declarations have at most 2-3 universe parameters"),
            );
        }
        let mut seen = HashSet::new();
        for param in univ_params {
            if !seen.insert(param) {
                ctx.emit(LintDiagnostic::new(
                    self.id(),
                    Severity::Error,
                    format!("duplicate universe parameter `{}`", param),
                    SourceRange::from_span(&decl.span),
                ));
            }
        }
    }
}
