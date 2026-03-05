//! # DeadCodeRule - Trait Implementations
//!
//! This module contains trait implementations for `DeadCodeRule`.
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

use super::types::DeadCodeRule;

impl LintRule for DeadCodeRule {
    fn id(&self) -> LintId {
        lint_ids::dead_code()
    }
    fn name(&self) -> &str {
        "dead code"
    }
    fn description(&self) -> &str {
        "detects declarations that are never referenced"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn category(&self) -> LintCategory {
        LintCategory::Correctness
    }
    fn check_decl(&self, ctx: &mut LintContext<'_>, decl: &Located<Decl>) {
        match &decl.value {
            Decl::Definition { name, val, ty, .. } => {
                ctx.register_var_decl(name, SourceRange::from_span(&decl.span));
                if let Some(ty_expr) = ty {
                    let refs = collect_var_refs(&ty_expr.value);
                    for r in refs {
                        ctx.register_var_use(&r, SourceRange::from_span(&ty_expr.span));
                    }
                }
                let refs = collect_var_refs(&val.value);
                for r in refs {
                    ctx.register_var_use(&r, SourceRange::from_span(&val.span));
                }
            }
            Decl::Theorem {
                name, ty, proof, ..
            } => {
                ctx.register_var_decl(name, SourceRange::from_span(&decl.span));
                let refs = collect_var_refs(&ty.value);
                for r in refs {
                    ctx.register_var_use(&r, SourceRange::from_span(&ty.span));
                }
                let refs = collect_var_refs(&proof.value);
                for r in refs {
                    ctx.register_var_use(&r, SourceRange::from_span(&proof.span));
                }
            }
            _ => {}
        }
    }
    fn finalize(&self, ctx: &mut LintContext<'_>) {
        let unused: Vec<(String, SourceRange)> = ctx
            .unused_variables()
            .iter()
            .map(|(name, range)| (name.to_string(), (*range).clone()))
            .collect();
        for (name, range) in unused {
            if self.excluded.contains(&name) {
                continue;
            }
            if name == "main" {
                continue;
            }
            ctx.emit(
                LintDiagnostic::new(
                    self.id(),
                    Severity::Warning,
                    format!("declaration `{}` is never used", name),
                    range,
                )
                .with_note("consider removing this declaration or marking it with `_`"),
            );
        }
    }
}
