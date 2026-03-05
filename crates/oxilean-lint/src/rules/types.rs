//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::framework::{
    collect_var_refs, is_camel_case, is_pascal_case, is_snake_case, lint_ids, to_snake_case,
    AutoFix, LintCategory, LintContext, LintDiagnostic, LintId, LintRule, Severity, SourceRange,
};
use oxilean_parse::{Binder, Decl, DoAction, Located, MatchArm, Pattern, Span, SurfaceExpr};
use std::collections::{HashMap, HashSet};

/// Detects explicit universe annotations that may be incorrect.
#[derive(Default)]
pub struct UniverseAnnotationRule;
impl UniverseAnnotationRule {
    /// Create a new instance.
    pub fn new() -> Self {
        Self
    }
}
/// Entry for a deprecated API.
struct DeprecatedEntry {
    /// Replacement suggestion.
    replacement: Option<String>,
    /// Deprecation message.
    message: String,
    /// Since version.
    since: Option<String>,
}
/// Detects code that can never be reached.
///
/// Examples:
/// - Code after a `return` in do-notation
/// - Dead branches in if/match with known conditions
/// - Absurd patterns in match
#[derive(Default)]
pub struct UnreachableCodeRule;
impl UnreachableCodeRule {
    /// Create a new instance.
    pub fn new() -> Self {
        Self
    }
    fn check_do_actions(
        &self,
        ctx: &mut LintContext<'_>,
        actions: &[oxilean_parse::DoAction],
        span: &Span,
    ) {
        let mut seen_return = false;
        for action in actions {
            if seen_return {
                ctx.emit(LintDiagnostic::new(
                    self.id(),
                    Severity::Warning,
                    "unreachable code after `return`",
                    SourceRange::from_span(span),
                ));
                break;
            }
            if let oxilean_parse::DoAction::Return(_) = action {
                seen_return = true;
            }
        }
    }
    pub(super) fn check_unreachable_branches(
        &self,
        ctx: &mut LintContext<'_>,
        expr: &Located<SurfaceExpr>,
    ) {
        match &expr.value {
            SurfaceExpr::If(cond, _then, _else) => {
                if let SurfaceExpr::Lit(oxilean_parse::Literal::Nat(0)) = &cond.value {
                    ctx.emit(
                        LintDiagnostic::new(
                            self.id(),
                            Severity::Warning,
                            "condition is always falsy (0), `else` branch is always taken",
                            SourceRange::from_span(&expr.span),
                        )
                        .with_note("the `then` branch is unreachable"),
                    );
                }
            }
            SurfaceExpr::Match(_, arms) => {
                let mut found_wildcard = false;
                for arm in arms {
                    if found_wildcard {
                        ctx.emit(LintDiagnostic::new(
                            self.id(),
                            Severity::Warning,
                            "unreachable match arm after catch-all pattern",
                            SourceRange::from_span(&arm.pattern.span),
                        ));
                    }
                    if matches!(arm.pattern.value, Pattern::Wild) && arm.guard.is_none() {
                        found_wildcard = true;
                    }
                }
            }
            SurfaceExpr::Do(actions) => {
                self.check_do_actions(ctx, actions, &expr.span);
            }
            _ => {}
        }
    }
}
impl UnreachableCodeRule {
    pub(super) fn walk_for_unreachable(
        &self,
        ctx: &mut LintContext<'_>,
        expr: &Located<SurfaceExpr>,
    ) {
        self.check_unreachable_branches(ctx, expr);
        match &expr.value {
            SurfaceExpr::App(f, arg) => {
                self.walk_for_unreachable(ctx, f);
                self.walk_for_unreachable(ctx, arg);
            }
            SurfaceExpr::Lam(_, body) | SurfaceExpr::Pi(_, body) => {
                self.walk_for_unreachable(ctx, body);
            }
            SurfaceExpr::Let(_, ty, val, body) => {
                if let Some(t) = ty {
                    self.walk_for_unreachable(ctx, t);
                }
                self.walk_for_unreachable(ctx, val);
                self.walk_for_unreachable(ctx, body);
            }
            SurfaceExpr::If(c, t, e) => {
                self.walk_for_unreachable(ctx, c);
                self.walk_for_unreachable(ctx, t);
                self.walk_for_unreachable(ctx, e);
            }
            SurfaceExpr::Match(s, arms) => {
                self.walk_for_unreachable(ctx, s);
                for arm in arms {
                    self.walk_for_unreachable(ctx, &arm.rhs);
                }
            }
            _ => {}
        }
    }
}
/// Detects overly complex expressions (deep nesting).
pub struct ComplexExprRule {
    /// Maximum nesting depth.
    pub max_depth: usize,
}
#[allow(clippy::new_without_default)]
impl ComplexExprRule {
    /// Create a new instance.
    pub fn new() -> Self {
        Self { max_depth: 10 }
    }
    pub(super) fn measure_depth(&self, expr: &SurfaceExpr) -> usize {
        match expr {
            SurfaceExpr::App(f, arg) => {
                1 + self
                    .measure_depth(&f.value)
                    .max(self.measure_depth(&arg.value))
            }
            SurfaceExpr::Lam(_, body) | SurfaceExpr::Pi(_, body) => {
                1 + self.measure_depth(&body.value)
            }
            SurfaceExpr::Let(_, _, val, body) => {
                1 + self
                    .measure_depth(&val.value)
                    .max(self.measure_depth(&body.value))
            }
            SurfaceExpr::If(c, t, e) => {
                1 + self
                    .measure_depth(&c.value)
                    .max(self.measure_depth(&t.value))
                    .max(self.measure_depth(&e.value))
            }
            SurfaceExpr::Match(s, arms) => {
                let arm_max = arms
                    .iter()
                    .map(|a| self.measure_depth(&a.rhs.value))
                    .max()
                    .unwrap_or(0);
                1 + self.measure_depth(&s.value).max(arm_max)
            }
            _ => 0,
        }
    }
}
/// Enforces code style rules.
///
/// Checks:
/// - Trailing whitespace
/// - Inconsistent indentation (tabs vs spaces)
/// - Long lines (>100 characters)
/// - Missing final newline
/// - Multiple blank lines
/// - Spaces around operators
pub struct StyleRule {
    /// Maximum line length.
    pub max_line_length: usize,
    /// Whether to disallow tabs.
    pub disallow_tabs: bool,
    /// Whether to check trailing whitespace.
    pub check_trailing_whitespace: bool,
    /// Whether to require final newline.
    pub require_final_newline: bool,
    /// Maximum consecutive blank lines.
    pub max_blank_lines: usize,
}
#[allow(clippy::new_without_default)]
impl StyleRule {
    /// Create with default settings.
    pub fn new() -> Self {
        Self {
            max_line_length: 100,
            disallow_tabs: true,
            check_trailing_whitespace: true,
            require_final_newline: true,
            max_blank_lines: 2,
        }
    }
    /// Create a relaxed version.
    pub fn relaxed() -> Self {
        Self {
            max_line_length: 120,
            disallow_tabs: false,
            check_trailing_whitespace: false,
            require_final_newline: false,
            max_blank_lines: 5,
        }
    }
}
/// Naming style convention.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NamingStyle {
    /// PascalCase (e.g., `MyType`)
    PascalCase,
    /// snake_case (e.g., `my_function`)
    SnakeCase,
    /// camelCase (e.g., `myFunction`)
    CamelCase,
    /// Any style is accepted.
    Any,
}
/// Detects empty match expressions.
#[derive(Default)]
pub struct EmptyMatchRule;
impl EmptyMatchRule {
    /// Create a new instance.
    pub fn new() -> Self {
        Self
    }
}
/// Detects redundant type annotations that can be inferred.
#[derive(Default)]
pub struct RedundantTypeAnnotationRule;
impl RedundantTypeAnnotationRule {
    /// Create a new instance.
    pub fn new() -> Self {
        Self
    }
}
/// Enforces naming conventions.
///
/// Conventions:
/// - Type names (inductives, structures, classes): PascalCase
/// - Definitions and theorems: snake_case or camelCase
/// - Variables: snake_case
/// - Namespaces: PascalCase
pub struct NamingConventionRule {
    /// Custom allowed names (exempt from checks).
    pub allowed_names: HashSet<String>,
    /// Convention for types.
    pub type_convention: NamingStyle,
    /// Convention for definitions.
    pub def_convention: NamingStyle,
    /// Convention for variables.
    pub var_convention: NamingStyle,
}
#[allow(clippy::new_without_default)]
impl NamingConventionRule {
    /// Create with default conventions (Lean 4-like).
    pub fn new() -> Self {
        Self {
            allowed_names: HashSet::new(),
            type_convention: NamingStyle::PascalCase,
            def_convention: NamingStyle::Any,
            var_convention: NamingStyle::Any,
        }
    }
    /// Add an allowed name that bypasses convention checks.
    pub fn allow_name(&mut self, name: String) {
        self.allowed_names.insert(name);
    }
    pub(super) fn check_name(
        &self,
        ctx: &mut LintContext<'_>,
        name: &str,
        style: NamingStyle,
        kind: &str,
        span: &Span,
    ) {
        if name.starts_with('_') || name.is_empty() || self.allowed_names.contains(name) {
            return;
        }
        let valid = match style {
            NamingStyle::PascalCase => is_pascal_case(name),
            NamingStyle::SnakeCase => is_snake_case(name),
            NamingStyle::CamelCase => is_camel_case(name),
            NamingStyle::Any => true,
        };
        if !valid {
            let expected = match style {
                NamingStyle::PascalCase => "PascalCase",
                NamingStyle::SnakeCase => "snake_case",
                NamingStyle::CamelCase => "camelCase",
                NamingStyle::Any => unreachable!(),
            };
            let range = SourceRange::from_span(span);
            let mut diag = LintDiagnostic::new(
                self.id(),
                Severity::Warning,
                format!(
                    "{} `{}` does not follow {} convention",
                    kind, name, expected
                ),
                range.clone(),
            );
            let suggested = match style {
                NamingStyle::SnakeCase => to_snake_case(name),
                NamingStyle::PascalCase => crate::framework::to_pascal_case(name),
                _ => name.to_string(),
            };
            if suggested != name {
                diag = diag.with_fix(AutoFix::replacement(
                    format!("rename to `{}`", suggested),
                    range,
                    suggested,
                ));
            }
            ctx.emit(diag);
        }
    }
}
/// Detects expressions that can be simplified.
///
/// Examples:
/// - `if true then a else b` -> `a`
/// - `if false then a else b` -> `b`
/// - `fun x => f x` -> `f` (eta reduction)
/// - `not (not p)` -> `p` (double negation)
/// - `x + 0` or `0 + x` -> `x`
/// - `x * 1` or `1 * x` -> `x`
#[derive(Default)]
pub struct SimplifiableExprRule;
impl SimplifiableExprRule {
    /// Create a new instance.
    pub fn new() -> Self {
        Self
    }
    pub(super) fn check_simplifiable(
        &self,
        ctx: &mut LintContext<'_>,
        expr: &Located<SurfaceExpr>,
    ) {
        match &expr.value {
            SurfaceExpr::If(cond, _then, _else_) => {
                if let SurfaceExpr::Var(name) = &cond.value {
                    if name == "true" || name == "True" {
                        ctx.emit(
                            LintDiagnostic::new(
                                self.id(),
                                Severity::Warning,
                                "condition is always true, `if` can be simplified",
                                SourceRange::from_span(&expr.span),
                            )
                            .with_note("replace with the `then` branch directly"),
                        );
                    } else if name == "false" || name == "False" {
                        ctx.emit(
                            LintDiagnostic::new(
                                self.id(),
                                Severity::Warning,
                                "condition is always false, `if` can be simplified",
                                SourceRange::from_span(&expr.span),
                            )
                            .with_note("replace with the `else` branch directly"),
                        );
                    }
                }
            }
            SurfaceExpr::Lam(binders, body) => {
                if binders.len() == 1 {
                    if let SurfaceExpr::App(f, arg) = &body.value {
                        if let SurfaceExpr::Var(arg_name) = &arg.value {
                            if arg_name == &binders[0].name {
                                let refs = collect_var_refs(&f.value);
                                if !refs.contains(&binders[0].name) {
                                    ctx.emit(
                                        LintDiagnostic::new(
                                            self.id(),
                                            Severity::Hint,
                                            "lambda can be eta-reduced",
                                            SourceRange::from_span(&expr.span),
                                        )
                                        .with_note("consider replacing `fun x => f x` with `f`"),
                                    );
                                }
                            }
                        }
                    }
                }
            }
            SurfaceExpr::App(f, _arg) => {
                if let SurfaceExpr::Var(name) = &f.value {
                    if name == "id" {
                        ctx.emit(
                            LintDiagnostic::new(
                                self.id(),
                                Severity::Hint,
                                "application of `id` can be removed",
                                SourceRange::from_span(&expr.span),
                            )
                            .with_note("replace `id x` with `x`"),
                        );
                    }
                }
            }
            SurfaceExpr::Let(name, _, _, body) => {
                if name == "_" {
                    ctx.emit(
                        LintDiagnostic::new(
                            self.id(),
                            Severity::Hint,
                            "let binding to `_` can potentially be removed",
                            SourceRange::from_span(&expr.span),
                        )
                        .with_note("the bound value is discarded"),
                    );
                }
                self.check_simplifiable(ctx, body);
            }
            _ => {}
        }
    }
}
impl SimplifiableExprRule {
    pub(super) fn walk_expr(&self, ctx: &mut LintContext<'_>, expr: &Located<SurfaceExpr>) {
        self.check_simplifiable(ctx, expr);
        match &expr.value {
            SurfaceExpr::App(f, arg) => {
                self.walk_expr(ctx, f);
                self.walk_expr(ctx, arg);
            }
            SurfaceExpr::Lam(_, body) | SurfaceExpr::Pi(_, body) => {
                self.walk_expr(ctx, body);
            }
            SurfaceExpr::Let(_, ty, val, body) => {
                if let Some(ty_expr) = ty {
                    self.walk_expr(ctx, ty_expr);
                }
                self.walk_expr(ctx, val);
                self.walk_expr(ctx, body);
            }
            SurfaceExpr::If(c, t, e) => {
                self.walk_expr(ctx, c);
                self.walk_expr(ctx, t);
                self.walk_expr(ctx, e);
            }
            SurfaceExpr::Match(s, arms) => {
                self.walk_expr(ctx, s);
                for arm in arms {
                    self.walk_expr(ctx, &arm.rhs);
                }
            }
            _ => {}
        }
    }
}
/// Detects double negation patterns like `not (not p)`.
#[derive(Default)]
pub struct DoubleNegationRule;
impl DoubleNegationRule {
    /// Create a new instance.
    pub fn new() -> Self {
        Self
    }
}
/// Detects hypotheses introduced with `have` or lambda binders in proofs that are
/// never referenced in the proof body.
///
/// Hypotheses prefixed with `_` are exempt from this check.
#[derive(Default)]
pub struct UnusedHypothesisRule;
impl UnusedHypothesisRule {
    /// Create a new instance.
    pub fn new() -> Self {
        Self
    }
    /// Check a `have h : T := val; body` expression.
    pub(super) fn check_have_in_expr(
        &self,
        ctx: &mut LintContext<'_>,
        expr: &Located<SurfaceExpr>,
        span: &Span,
    ) {
        match &expr.value {
            SurfaceExpr::Have(name, _ty, _val, body) => {
                if !name.starts_with('_') && name != "_" {
                    let refs = collect_var_refs(&body.value);
                    if !refs.contains(name) {
                        ctx.emit(
                            LintDiagnostic::new(
                                    self.id(),
                                    Severity::Warning,
                                    format!(
                                        "hypothesis `{}` is introduced but never used in the proof body",
                                        name
                                    ),
                                    SourceRange::from_span(span),
                                )
                                .with_note(
                                    "if this hypothesis is intentionally unused, prefix its name with `_`",
                                ),
                        );
                    }
                }
                self.check_have_in_expr(ctx, _val, span);
                self.check_have_in_expr(ctx, body, span);
            }
            SurfaceExpr::App(f, arg) => {
                self.check_have_in_expr(ctx, f, span);
                self.check_have_in_expr(ctx, arg, span);
            }
            SurfaceExpr::Lam(_, body) | SurfaceExpr::Pi(_, body) => {
                self.check_have_in_expr(ctx, body, span);
            }
            SurfaceExpr::Let(_, ty, val, body) => {
                if let Some(t) = ty {
                    self.check_have_in_expr(ctx, t, span);
                }
                self.check_have_in_expr(ctx, val, span);
                self.check_have_in_expr(ctx, body, span);
            }
            SurfaceExpr::If(c, t, e) => {
                self.check_have_in_expr(ctx, c, span);
                self.check_have_in_expr(ctx, t, span);
                self.check_have_in_expr(ctx, e, span);
            }
            SurfaceExpr::Match(scrut, arms) => {
                self.check_have_in_expr(ctx, scrut, span);
                for arm in arms {
                    self.check_have_in_expr(ctx, &arm.rhs, span);
                }
            }
            _ => {}
        }
    }
}
/// Detects `have h : T := h` where a hypothesis is immediately re-introduced
/// with the same name, or `have h : T := assumption` patterns where the type
/// already exists as a hypothesis (trivially redundant).
///
/// Also detects duplicate `have` bindings with identical names in the same scope.
#[derive(Default)]
pub struct RedundantAssumptionRule;
impl RedundantAssumptionRule {
    /// Create a new instance.
    pub fn new() -> Self {
        Self
    }
    /// Collect all `have` names introduced in an expression (non-recursive into have bodies).
    pub(crate) fn collect_have_names(expr: &SurfaceExpr, names: &mut Vec<String>) {
        match expr {
            SurfaceExpr::Have(name, _, val, body) => {
                names.push(name.clone());
                Self::collect_have_names(&val.value, names);
                Self::collect_have_names(&body.value, names);
            }
            SurfaceExpr::App(f, arg) => {
                Self::collect_have_names(&f.value, names);
                Self::collect_have_names(&arg.value, names);
            }
            SurfaceExpr::Lam(_, body) | SurfaceExpr::Pi(_, body) => {
                Self::collect_have_names(&body.value, names);
            }
            SurfaceExpr::Let(_, _, val, body) => {
                Self::collect_have_names(&val.value, names);
                Self::collect_have_names(&body.value, names);
            }
            _ => {}
        }
    }
    pub(super) fn check_expr_for_redundancy(
        &self,
        ctx: &mut LintContext<'_>,
        expr: &Located<SurfaceExpr>,
        span: &Span,
    ) {
        match &expr.value {
            SurfaceExpr::Have(name, _ty, val, body) => {
                if let SurfaceExpr::Var(val_name) = &val.value {
                    if val_name == name {
                        ctx.emit(
                            LintDiagnostic::new(
                                self.id(),
                                Severity::Warning,
                                format!("hypothesis `{}` is redundantly assigned to itself", name),
                                SourceRange::from_span(span),
                            )
                            .with_note("remove this `have` binding"),
                        );
                    }
                }
                let mut body_names = Vec::new();
                Self::collect_have_names(&body.value, &mut body_names);
                if body_names.contains(name) {
                    ctx.emit(
                        LintDiagnostic::new(
                            self.id(),
                            Severity::Info,
                            format!(
                                "hypothesis name `{}` is shadowed by a later `have` binding",
                                name
                            ),
                            SourceRange::from_span(span),
                        )
                        .with_note("consider using distinct names for each hypothesis"),
                    );
                }
                self.check_expr_for_redundancy(ctx, val, span);
                self.check_expr_for_redundancy(ctx, body, span);
            }
            SurfaceExpr::App(f, arg) => {
                self.check_expr_for_redundancy(ctx, f, span);
                self.check_expr_for_redundancy(ctx, arg, span);
            }
            SurfaceExpr::Lam(_, body) | SurfaceExpr::Pi(_, body) => {
                self.check_expr_for_redundancy(ctx, body, span);
            }
            SurfaceExpr::Let(_, _, val, body) => {
                self.check_expr_for_redundancy(ctx, val, span);
                self.check_expr_for_redundancy(ctx, body, span);
            }
            _ => {}
        }
    }
}
/// Detects suspicious variable shadowing.
#[derive(Default)]
pub struct SuspiciousShadowRule;
impl SuspiciousShadowRule {
    /// Create a new instance.
    pub fn new() -> Self {
        Self
    }
}
/// Detects applications of identity-like functions.
#[derive(Default)]
pub struct IdentityApplicationRule;
impl IdentityApplicationRule {
    /// Create a new instance.
    pub fn new() -> Self {
        Self
    }
}
/// Warns when a proof block contains more tactic steps than a configured threshold.
///
/// Very long proof blocks are hard to read and maintain. Consider breaking them
/// into smaller lemmas using `have` or extracting helper theorems.
pub struct LongProofRule {
    /// Maximum number of tactic lines before warning.
    pub max_tactic_lines: usize,
}
impl LongProofRule {
    /// Create a new instance with the default threshold of 50 tactic lines.
    pub fn new() -> Self {
        Self {
            max_tactic_lines: 50,
        }
    }
    /// Create a new instance with a custom threshold.
    pub fn with_threshold(max_tactic_lines: usize) -> Self {
        Self { max_tactic_lines }
    }
    /// Count the total number of tactic lines in an expression tree.
    pub(super) fn count_tactic_lines(&self, expr: &SurfaceExpr) -> usize {
        match expr {
            SurfaceExpr::ByTactic(tactics) => tactics.len(),
            SurfaceExpr::App(f, arg) => {
                self.count_tactic_lines(&f.value) + self.count_tactic_lines(&arg.value)
            }
            SurfaceExpr::Lam(_, body) | SurfaceExpr::Pi(_, body) => {
                self.count_tactic_lines(&body.value)
            }
            SurfaceExpr::Let(_, ty, val, body) => {
                ty.as_ref()
                    .map(|t| self.count_tactic_lines(&t.value))
                    .unwrap_or(0)
                    + self.count_tactic_lines(&val.value)
                    + self.count_tactic_lines(&body.value)
            }
            SurfaceExpr::Have(_, ty, val, body) => {
                self.count_tactic_lines(&ty.value)
                    + self.count_tactic_lines(&val.value)
                    + self.count_tactic_lines(&body.value)
            }
            SurfaceExpr::If(c, t, e) => {
                self.count_tactic_lines(&c.value)
                    + self.count_tactic_lines(&t.value)
                    + self.count_tactic_lines(&e.value)
            }
            SurfaceExpr::Match(scrut, arms) => {
                self.count_tactic_lines(&scrut.value)
                    + arms
                        .iter()
                        .map(|a| self.count_tactic_lines(&a.rhs.value))
                        .sum::<usize>()
            }
            _ => 0,
        }
    }
}
/// Detects inconsistent binder styles in a single declaration.
#[derive(Default)]
pub struct InconsistentBinderRule;
impl InconsistentBinderRule {
    /// Create a new instance.
    pub fn new() -> Self {
        Self
    }
}
/// Warns when public declarations (`theorem`, `def`, `inductive`, `axiom`) lack
/// a doc-comment.
///
/// Doc-comments in OxiLean are written with `--!` or `/-- ... -/` before the
/// declaration. This rule checks for their presence via the `LintContext`
/// source text immediately preceding the declaration span.
///
/// Unlike `MissingDocRule` (which checks based on visibility metadata), this
/// rule is purely syntactic: it looks at the raw source text above the
/// declaration for any doc-comment marker.
#[derive(Default)]
pub struct MissingDocstringRule;
impl MissingDocstringRule {
    /// Create a new instance.
    pub fn new() -> Self {
        Self
    }
    /// Return `true` if the source text immediately before `start` (within the
    /// same file) ends with a doc-comment line.
    pub(super) fn has_doc_comment_before(source: &str, start: usize) -> bool {
        let before = &source[..start.min(source.len())];
        for line in before.lines().rev() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            return trimmed.starts_with("--!")
                || trimmed.starts_with("/--")
                || trimmed.starts_with("---")
                || trimmed.starts_with("/-!");
        }
        false
    }
}
/// Detects theorems and definitions that use `sorry` as a proof placeholder.
///
/// A `sorry` in a proof is an unsound axiom that admits any goal without
/// verification. It should only appear in work-in-progress code.
#[derive(Default)]
pub struct SorryInProofRule;
impl SorryInProofRule {
    /// Create a new instance.
    pub fn new() -> Self {
        Self
    }
    /// Recursively scan an expression for any occurrence of `sorry`.
    pub(super) fn contains_sorry(expr: &SurfaceExpr) -> bool {
        match expr {
            SurfaceExpr::Var(name) => name == "sorry",
            SurfaceExpr::ByTactic(tactics) => tactics.iter().any(|t| t.value.trim() == "sorry"),
            SurfaceExpr::App(f, arg) => {
                Self::contains_sorry(&f.value) || Self::contains_sorry(&arg.value)
            }
            SurfaceExpr::Lam(_, body) | SurfaceExpr::Pi(_, body) => {
                Self::contains_sorry(&body.value)
            }
            SurfaceExpr::Let(_, ty, val, body) => {
                ty.as_ref()
                    .map(|t| Self::contains_sorry(&t.value))
                    .unwrap_or(false)
                    || Self::contains_sorry(&val.value)
                    || Self::contains_sorry(&body.value)
            }
            SurfaceExpr::Ann(e, ty) => {
                Self::contains_sorry(&e.value) || Self::contains_sorry(&ty.value)
            }
            SurfaceExpr::If(c, t, e) => {
                Self::contains_sorry(&c.value)
                    || Self::contains_sorry(&t.value)
                    || Self::contains_sorry(&e.value)
            }
            SurfaceExpr::Match(scrut, arms) => {
                Self::contains_sorry(&scrut.value)
                    || arms.iter().any(|a| Self::contains_sorry(&a.rhs.value))
            }
            SurfaceExpr::Have(_, ty, val, body) => {
                Self::contains_sorry(&ty.value)
                    || Self::contains_sorry(&val.value)
                    || Self::contains_sorry(&body.value)
            }
            SurfaceExpr::Suffices(_, ty, body) => {
                Self::contains_sorry(&ty.value) || Self::contains_sorry(&body.value)
            }
            SurfaceExpr::Show(ty, e) => {
                Self::contains_sorry(&ty.value) || Self::contains_sorry(&e.value)
            }
            SurfaceExpr::NamedArg(f, _, arg) => {
                Self::contains_sorry(&f.value) || Self::contains_sorry(&arg.value)
            }
            SurfaceExpr::AnonymousCtor(args) | SurfaceExpr::ListLit(args) => {
                args.iter().any(|a| Self::contains_sorry(&a.value))
            }
            SurfaceExpr::Tuple(elems) => elems.iter().any(|e| Self::contains_sorry(&e.value)),
            SurfaceExpr::Return(e) => Self::contains_sorry(&e.value),
            SurfaceExpr::Proj(e, _) => Self::contains_sorry(&e.value),
            SurfaceExpr::Do(actions) => actions.iter().any(|a| match a {
                DoAction::Bind(_, e) | DoAction::Expr(e) | DoAction::Return(e) => {
                    Self::contains_sorry(&e.value)
                }
                DoAction::Let(_, e) => Self::contains_sorry(&e.value),
                DoAction::LetTyped(_, ty, e) => {
                    Self::contains_sorry(&ty.value) || Self::contains_sorry(&e.value)
                }
            }),
            SurfaceExpr::Calc(steps) => steps.iter().any(|s| Self::contains_sorry(&s.proof.value)),
            SurfaceExpr::Range(lo, hi) => {
                lo.as_ref()
                    .map(|e| Self::contains_sorry(&e.value))
                    .unwrap_or(false)
                    || hi
                        .as_ref()
                        .map(|e| Self::contains_sorry(&e.value))
                        .unwrap_or(false)
            }
            SurfaceExpr::StringInterp(_)
            | SurfaceExpr::Lit(_)
            | SurfaceExpr::Sort(_)
            | SurfaceExpr::Hole => false,
        }
    }
}
/// Detects unnecessary parentheses.
#[derive(Default)]
pub struct UnnecessaryParensRule;
impl UnnecessaryParensRule {
    /// Create a new instance.
    pub fn new() -> Self {
        Self
    }
}
/// Warns when deprecated or discouraged tactics are used in proof blocks.
///
/// Deprecated tactics in OxiLean:
/// - `exact_mod_cast` → prefer `exact` with explicit cast
/// - `norm_cast` → use `push_cast` / `pull_cast` instead
/// - `finish` → deprecated proof search tactic
/// - `blast` → removed tactic, use `simp` + `decide`
/// - `tauto` → prefer `omega` or `decide` for propositional goals
pub struct DeprecatedTacticRule {
    /// Map from deprecated tactic name to suggested replacement.
    pub(crate) deprecated: HashMap<&'static str, &'static str>,
}
impl DeprecatedTacticRule {
    /// Create a new instance with the default set of deprecated tactics.
    pub fn new() -> Self {
        let mut deprecated = HashMap::new();
        deprecated.insert(
            "exact_mod_cast",
            "use `exact` with an explicit cast instead",
        );
        deprecated.insert("norm_cast", "use `push_cast` or `pull_cast` instead");
        deprecated.insert(
            "finish",
            "`finish` is deprecated — try `simp` + `assumption`",
        );
        deprecated.insert("blast", "`blast` is removed — use `simp` + `decide`");
        deprecated.insert(
            "tauto",
            "prefer `omega` or `decide` for propositional goals",
        );
        deprecated.insert("ring_exp", "use `ring` instead");
        deprecated.insert("Abel", "use `ring` or `group` instead");
        Self { deprecated }
    }
}
impl DeprecatedTacticRule {
    pub(super) fn scan_for_deprecated_tactics(
        &self,
        ctx: &mut LintContext<'_>,
        expr: &Located<SurfaceExpr>,
        span: &Span,
    ) {
        match &expr.value {
            SurfaceExpr::ByTactic(tactics) => {
                for tactic in tactics {
                    let tactic_str = tactic.value.trim();
                    let tactic_name = tactic_str.split_whitespace().next().unwrap_or("");
                    if let Some(advice) = self.deprecated.get(tactic_name) {
                        ctx.emit(
                            LintDiagnostic::new(
                                self.id(),
                                Severity::Warning,
                                format!("deprecated tactic `{}` used", tactic_name),
                                SourceRange::from_span(span),
                            )
                            .with_note(*advice),
                        );
                    }
                }
            }
            SurfaceExpr::App(f, arg) => {
                self.scan_for_deprecated_tactics(ctx, f, span);
                self.scan_for_deprecated_tactics(ctx, arg, span);
            }
            SurfaceExpr::Lam(_, body) | SurfaceExpr::Pi(_, body) => {
                self.scan_for_deprecated_tactics(ctx, body, span);
            }
            SurfaceExpr::Let(_, ty, val, body) => {
                if let Some(t) = ty {
                    self.scan_for_deprecated_tactics(ctx, t, span);
                }
                self.scan_for_deprecated_tactics(ctx, val, span);
                self.scan_for_deprecated_tactics(ctx, body, span);
            }
            SurfaceExpr::Have(_, ty, val, body) => {
                self.scan_for_deprecated_tactics(ctx, ty, span);
                self.scan_for_deprecated_tactics(ctx, val, span);
                self.scan_for_deprecated_tactics(ctx, body, span);
            }
            SurfaceExpr::If(c, t, e) => {
                self.scan_for_deprecated_tactics(ctx, c, span);
                self.scan_for_deprecated_tactics(ctx, t, span);
                self.scan_for_deprecated_tactics(ctx, e, span);
            }
            SurfaceExpr::Match(scrut, arms) => {
                self.scan_for_deprecated_tactics(ctx, scrut, span);
                for arm in arms {
                    self.scan_for_deprecated_tactics(ctx, &arm.rhs, span);
                }
            }
            _ => {}
        }
    }
}
/// Detects public declarations that lack documentation.
pub struct MissingDocRule {
    /// Whether to require docs on theorems.
    pub require_theorem_docs: bool,
    /// Whether to require docs on definitions.
    pub require_def_docs: bool,
    /// Whether to require docs on inductive types.
    pub require_inductive_docs: bool,
    /// Whether to require docs on structures.
    pub require_structure_docs: bool,
}
#[allow(clippy::new_without_default)]
impl MissingDocRule {
    /// Create with default settings (require docs on all public decls).
    pub fn new() -> Self {
        Self {
            require_theorem_docs: true,
            require_def_docs: true,
            require_inductive_docs: true,
            require_structure_docs: true,
        }
    }
    /// Create a relaxed version that only checks definitions.
    pub fn defs_only() -> Self {
        Self {
            require_theorem_docs: false,
            require_def_docs: true,
            require_inductive_docs: false,
            require_structure_docs: false,
        }
    }
    pub(super) fn has_doc_comment(&self, source: &str, span: &Span) -> bool {
        if span.start == 0 {
            return false;
        }
        let before = &source[..span.start.min(source.len())];
        let trimmed = before.trim_end();
        trimmed.ends_with("-/") || trimmed.ends_with("--/")
    }
}
/// Detects redundant patterns in match expressions.
///
/// Examples:
/// - A match with only a wildcard arm
/// - Match with identical arms that could be collapsed
/// - Match on a constructor with exactly one variant
#[derive(Default)]
pub struct RedundantPatternRule;
impl RedundantPatternRule {
    /// Create a new instance.
    pub fn new() -> Self {
        Self
    }
    pub(super) fn check_match_arms(
        &self,
        ctx: &mut LintContext<'_>,
        arms: &[MatchArm],
        span: &Span,
    ) {
        if arms.len() == 1 {
            if let Pattern::Wild = arms[0].pattern.value {
                ctx.emit(
                    LintDiagnostic::new(
                        self.id(),
                        Severity::Warning,
                        "match expression with a single wildcard arm can be simplified",
                        SourceRange::from_span(span),
                    )
                    .with_note("consider removing the match and using the body directly"),
                );
            }
        }
        if arms.len() >= 2 {
            let first_body = format!("{:?}", arms[0].rhs.value);
            let all_same = arms
                .iter()
                .all(|a| format!("{:?}", a.rhs.value) == first_body);
            if all_same {
                ctx.emit(
                    LintDiagnostic::new(
                        self.id(),
                        Severity::Warning,
                        "all match arms have the same body",
                        SourceRange::from_span(span),
                    )
                    .with_note("consider replacing with a single expression"),
                );
            }
        }
        let mut seen_wildcard = false;
        for arm in arms {
            if let Pattern::Wild = arm.pattern.value {
                if seen_wildcard {
                    ctx.emit(LintDiagnostic::new(
                        self.id(),
                        Severity::Warning,
                        "unreachable pattern after wildcard",
                        SourceRange::from_span(&arm.pattern.span),
                    ));
                }
                seen_wildcard = true;
            }
        }
        for arm in arms {
            if let Pattern::Var(name) = &arm.pattern.value {
                let refs = collect_var_refs(&arm.rhs.value);
                if !refs.contains(name) {
                    ctx.emit(
                        LintDiagnostic::new(
                            self.id(),
                            Severity::Hint,
                            format!("pattern variable `{}` is not used in the arm body", name),
                            SourceRange::from_span(&arm.pattern.span),
                        )
                        .with_fix(AutoFix::replacement(
                            "replace with wildcard `_`",
                            SourceRange::from_span(&arm.pattern.span),
                            "_".to_string(),
                        )),
                    );
                }
            }
        }
    }
}
impl RedundantPatternRule {
    pub(super) fn walk_expr_for_matches(
        &self,
        ctx: &mut LintContext<'_>,
        expr: &Located<SurfaceExpr>,
    ) {
        match &expr.value {
            SurfaceExpr::Match(scrut, arms) => {
                self.check_match_arms(ctx, arms, &expr.span);
                self.walk_expr_for_matches(ctx, scrut);
                for arm in arms {
                    self.walk_expr_for_matches(ctx, &arm.rhs);
                }
            }
            SurfaceExpr::App(f, arg) => {
                self.walk_expr_for_matches(ctx, f);
                self.walk_expr_for_matches(ctx, arg);
            }
            SurfaceExpr::Lam(_, body) | SurfaceExpr::Pi(_, body) => {
                self.walk_expr_for_matches(ctx, body);
            }
            SurfaceExpr::Let(_, ty, val, body) => {
                if let Some(ty_expr) = ty {
                    self.walk_expr_for_matches(ctx, ty_expr);
                }
                self.walk_expr_for_matches(ctx, val);
                self.walk_expr_for_matches(ctx, body);
            }
            SurfaceExpr::If(c, t, e) => {
                self.walk_expr_for_matches(ctx, c);
                self.walk_expr_for_matches(ctx, t);
                self.walk_expr_for_matches(ctx, e);
            }
            _ => {}
        }
    }
}
/// Detects variables that are declared but never used.
///
/// Variables prefixed with `_` are exempt from this check.
#[derive(Default)]
pub struct UnusedVariableRule;
impl UnusedVariableRule {
    /// Create a new instance of the unused variable rule.
    pub fn new() -> Self {
        Self
    }
    /// Collect bindings from binders and track usage in body.
    fn check_binders_in_body(
        &self,
        ctx: &mut LintContext<'_>,
        binders: &[Binder],
        body: &Located<SurfaceExpr>,
        decl_span: &Span,
    ) {
        let refs = collect_var_refs(&body.value);
        for binder in binders {
            if binder.name.starts_with('_') || binder.name == "_" {
                continue;
            }
            if !refs.contains(&binder.name) {
                let range = SourceRange::from_span(decl_span);
                let fix = AutoFix::replacement(
                    format!("prefix unused variable `{}` with `_`", binder.name),
                    range.clone(),
                    format!("_{}", binder.name),
                );
                ctx.emit(
                    LintDiagnostic::new(
                        self.id(),
                        Severity::Warning,
                        format!("unused variable `{}`", binder.name),
                        range,
                    )
                    .with_fix(fix)
                    .with_note("if this is intentional, prefix the name with `_`"),
                );
            }
        }
    }
}
impl UnusedVariableRule {
    pub(super) fn check_expr_for_unused(
        &self,
        ctx: &mut LintContext<'_>,
        expr: &Located<SurfaceExpr>,
    ) {
        match &expr.value {
            SurfaceExpr::Lam(binders, body) => {
                self.check_binders_in_body(ctx, binders, body, &expr.span);
            }
            SurfaceExpr::Let(name, _ty, _val, body) => {
                if !name.starts_with('_') && name != "_" {
                    let refs = collect_var_refs(&body.value);
                    if !refs.contains(name) {
                        let range = SourceRange::from_span(&expr.span);
                        ctx.emit(
                            LintDiagnostic::new(
                                self.id(),
                                Severity::Warning,
                                format!("unused let binding `{}`", name),
                                range,
                            )
                            .with_note("if this is intentional, prefix the name with `_`"),
                        );
                    }
                }
            }
            SurfaceExpr::Have(name, _ty, _proof, body) => {
                if !name.starts_with('_') && name != "_" {
                    let refs = collect_var_refs(&body.value);
                    if !refs.contains(name) {
                        let range = SourceRange::from_span(&expr.span);
                        ctx.emit(LintDiagnostic::new(
                            self.id(),
                            Severity::Warning,
                            format!("unused `have` binding `{}`", name),
                            range,
                        ));
                    }
                }
            }
            _ => {}
        }
    }
}
/// Detects unused where clause definitions.
#[derive(Default)]
pub struct UnusedWhereRule;
impl UnusedWhereRule {
    /// Create a new instance.
    pub fn new() -> Self {
        Self
    }
}
/// Detects pattern matches with many arms.
pub struct LargePatternMatchRule {
    /// Maximum allowed match arms.
    pub max_arms: usize,
}
#[allow(clippy::new_without_default)]
impl LargePatternMatchRule {
    /// Create a new instance.
    pub fn new() -> Self {
        Self { max_arms: 15 }
    }
}
/// Detects usage of deprecated definitions.
pub struct DeprecatedApiRule {
    /// Map of deprecated name -> replacement suggestion.
    deprecated: HashMap<String, DeprecatedEntry>,
}
#[allow(clippy::new_without_default)]
impl DeprecatedApiRule {
    /// Create with default deprecated API list.
    pub fn new() -> Self {
        let mut deprecated = HashMap::new();
        deprecated.insert(
            "Nat.has_zero".to_string(),
            DeprecatedEntry {
                replacement: Some("instOfNatNat".to_string()),
                message: "Nat.has_zero is deprecated".to_string(),
                since: Some("0.2.0".to_string()),
            },
        );
        deprecated.insert(
            "List.nil".to_string(),
            DeprecatedEntry {
                replacement: Some("[]".to_string()),
                message: "use list literal syntax instead".to_string(),
                since: Some("0.1.0".to_string()),
            },
        );
        deprecated.insert(
            "Classical.propDecidable".to_string(),
            DeprecatedEntry {
                replacement: Some("Classical.dec".to_string()),
                message: "propDecidable has been renamed".to_string(),
                since: Some("0.2.0".to_string()),
            },
        );
        deprecated.insert(
            "Decidable.decide".to_string(),
            DeprecatedEntry {
                replacement: Some("decide".to_string()),
                message: "use top-level decide instead".to_string(),
                since: Some("0.2.0".to_string()),
            },
        );
        deprecated.insert(
            "Option.isSome".to_string(),
            DeprecatedEntry {
                replacement: Some("Option.isSome'".to_string()),
                message: "Option.isSome has been replaced".to_string(),
                since: Some("0.3.0".to_string()),
            },
        );
        Self { deprecated }
    }
    /// Add a custom deprecated entry.
    pub fn add_deprecated(&mut self, name: String, replacement: Option<String>, message: String) {
        self.deprecated.insert(
            name,
            DeprecatedEntry {
                replacement,
                message,
                since: None,
            },
        );
    }
    pub(super) fn check_name_in_expr(
        &self,
        ctx: &mut LintContext<'_>,
        expr: &Located<SurfaceExpr>,
    ) {
        match &expr.value {
            SurfaceExpr::Var(name) => {
                if let Some(entry) = self.deprecated.get(name) {
                    let range = SourceRange::from_span(&expr.span);
                    let mut diag = LintDiagnostic::new(
                        self.id(),
                        Severity::Warning,
                        &entry.message,
                        range.clone(),
                    );
                    if let Some(ref since) = entry.since {
                        diag = diag.with_note(format!("deprecated since version {}", since));
                    }
                    if let Some(ref replacement) = entry.replacement {
                        diag = diag.with_fix(AutoFix::replacement(
                            format!("replace with `{}`", replacement),
                            range,
                            replacement.clone(),
                        ));
                    }
                    ctx.emit(diag);
                }
            }
            SurfaceExpr::App(f, arg) => {
                self.check_name_in_expr(ctx, f);
                self.check_name_in_expr(ctx, arg);
            }
            SurfaceExpr::Lam(_, body) | SurfaceExpr::Pi(_, body) => {
                self.check_name_in_expr(ctx, body);
            }
            SurfaceExpr::Let(_, ty, val, body) => {
                if let Some(ty_expr) = ty {
                    self.check_name_in_expr(ctx, ty_expr);
                }
                self.check_name_in_expr(ctx, val);
                self.check_name_in_expr(ctx, body);
            }
            _ => {}
        }
    }
}
/// Detects declarations that are never referenced.
///
/// Tracks definitions and their usage across the module.
#[derive(Default)]
pub struct DeadCodeRule {
    /// Names that are declared.
    declared: HashMap<String, SourceRange>,
    /// Names that are used.
    used: HashSet<String>,
    /// Names to exclude from dead code analysis.
    pub(super) excluded: HashSet<String>,
}
impl DeadCodeRule {
    /// Create a new instance.
    pub fn new() -> Self {
        Self {
            declared: HashMap::new(),
            used: HashSet::new(),
            excluded: HashSet::new(),
        }
    }
    /// Exclude a name from dead code analysis.
    pub fn exclude(&mut self, name: String) {
        self.excluded.insert(name);
    }
    fn collect_refs_from_expr(&self, expr: &SurfaceExpr, used: &mut HashSet<String>) {
        let refs = collect_var_refs(expr);
        used.extend(refs);
    }
}
/// Detects imports that are never referenced.
#[derive(Default)]
pub struct UnusedImportRule;
impl UnusedImportRule {
    /// Create a new instance.
    pub fn new() -> Self {
        Self
    }
}
