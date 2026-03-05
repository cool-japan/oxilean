//! # UnusedImportRule - Trait Implementations
//!
//! This module contains trait implementations for `UnusedImportRule`.
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

use super::types::UnusedImportRule;

impl LintRule for UnusedImportRule {
    fn id(&self) -> LintId {
        lint_ids::unused_import()
    }
    fn name(&self) -> &str {
        "unused import"
    }
    fn description(&self) -> &str {
        "detects imports that are not referenced in the file"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn category(&self) -> LintCategory {
        LintCategory::Correctness
    }
    fn check_decl(&self, ctx: &mut LintContext<'_>, decl: &Located<Decl>) {
        if let Decl::Import { path } = &decl.value {
            ctx.register_import(path.clone(), SourceRange::from_span(&decl.span));
        }
        if let Decl::Definition { val, .. } | Decl::Theorem { proof: val, .. } = &decl.value {
            let refs = collect_var_refs(&val.value);
            for name in &refs {
                if let Some(parts) = name.split('.').next() {
                    ctx.mark_import_used(&[parts.to_string()]);
                }
            }
        }
    }
    fn finalize(&self, ctx: &mut LintContext<'_>) {
        let unused: Vec<(String, SourceRange)> = ctx
            .unused_imports()
            .iter()
            .map(|imp| (imp.path.join("."), imp.range.clone()))
            .collect();
        for (path_str, range) in unused {
            let fix = AutoFix::deletion(
                format!("remove unused import `{}`", path_str),
                range.clone(),
            );
            ctx.emit(
                LintDiagnostic::new(
                    self.id(),
                    Severity::Warning,
                    format!("unused import `{}`", path_str),
                    range,
                )
                .with_fix(fix),
            );
        }
    }
}
