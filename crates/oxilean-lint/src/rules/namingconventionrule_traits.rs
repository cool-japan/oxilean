//! # NamingConventionRule - Trait Implementations
//!
//! This module contains trait implementations for `NamingConventionRule`.
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

use super::types::{NamingConventionRule, NamingStyle};

impl LintRule for NamingConventionRule {
    fn id(&self) -> LintId {
        lint_ids::naming_convention()
    }
    fn name(&self) -> &str {
        "naming convention"
    }
    fn description(&self) -> &str {
        "enforces naming conventions for types, definitions, and variables"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn category(&self) -> LintCategory {
        LintCategory::Style
    }
    fn check_decl(&self, ctx: &mut LintContext<'_>, decl: &Located<Decl>) {
        match &decl.value {
            Decl::Inductive { name, .. } => {
                self.check_name(
                    ctx,
                    name,
                    self.type_convention,
                    "inductive type",
                    &decl.span,
                );
            }
            Decl::Structure { name, .. } => {
                self.check_name(ctx, name, self.type_convention, "structure", &decl.span);
            }
            Decl::ClassDecl { name, .. } => {
                self.check_name(ctx, name, self.type_convention, "class", &decl.span);
            }
            Decl::Definition { name, .. } => {
                self.check_name(ctx, name, self.def_convention, "definition", &decl.span);
            }
            Decl::Theorem { name, .. } => {
                self.check_name(ctx, name, self.def_convention, "theorem", &decl.span);
            }
            Decl::Namespace { name, .. } => {
                self.check_name(ctx, name, NamingStyle::PascalCase, "namespace", &decl.span);
            }
            _ => {}
        }
    }
}
