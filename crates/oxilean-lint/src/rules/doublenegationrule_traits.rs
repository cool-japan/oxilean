//! # DoubleNegationRule - Trait Implementations
//!
//! This module contains trait implementations for `DoubleNegationRule`.
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

use super::types::DoubleNegationRule;

impl LintRule for DoubleNegationRule {
    fn id(&self) -> LintId {
        lint_ids::double_negation()
    }
    fn name(&self) -> &str {
        "double negation"
    }
    fn description(&self) -> &str {
        "detects double negation that can be simplified"
    }
    fn default_severity(&self) -> Severity {
        Severity::Hint
    }
    fn category(&self) -> LintCategory {
        LintCategory::Complexity
    }
    fn check_expr(&self, ctx: &mut LintContext<'_>, expr: &Located<SurfaceExpr>) {
        if let SurfaceExpr::App(f, arg) = &expr.value {
            if let SurfaceExpr::Var(name) = &f.value {
                if name == "not" || name == "Not" {
                    if let SurfaceExpr::App(inner_f, _inner_arg) = &arg.value {
                        if let SurfaceExpr::Var(inner_name) = &inner_f.value {
                            if inner_name == "not" || inner_name == "Not" {
                                ctx.emit(
                                    LintDiagnostic::new(
                                        self.id(),
                                        Severity::Hint,
                                        "double negation can be simplified",
                                        SourceRange::from_span(&expr.span),
                                    )
                                    .with_note(
                                        "use `Decidable.decide` or apply `not_not` to simplify",
                                    ),
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}
