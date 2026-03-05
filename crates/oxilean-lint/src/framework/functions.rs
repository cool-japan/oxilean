//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_parse::{Decl, Located, Span, SurfaceExpr};
use std::collections::{HashMap, HashSet};

use super::types::{
    AnnotationKind, AnnotationParser, AutoFix, LintCategory, LintConfig, LintContext,
    LintDiagnostic, LintFileStats, LintId, LintPassDependencies, LintPassScheduler, LintSummary,
    Severity, SourceRange, SuppressionParser,
};

/// Trait for implementing lint rules.
///
/// Each lint rule inspects AST nodes and emits diagnostics.
pub trait LintRule: Send + Sync {
    /// Unique identifier for this lint.
    fn id(&self) -> LintId;
    /// Human-readable name.
    fn name(&self) -> &str;
    /// Description of what this lint checks.
    fn description(&self) -> &str;
    /// Default severity for this lint.
    fn default_severity(&self) -> Severity;
    /// Category of this lint.
    fn category(&self) -> LintCategory {
        LintCategory::Correctness
    }
    /// Check a declaration.
    fn check_decl(&self, _ctx: &mut LintContext<'_>, _decl: &Located<Decl>) {}
    /// Check an expression.
    fn check_expr(&self, _ctx: &mut LintContext<'_>, _expr: &Located<SurfaceExpr>) {}
    /// Finalize lint analysis (called after all decls/exprs are visited).
    fn finalize(&self, _ctx: &mut LintContext<'_>) {}
}
/// Visitor trait for walking the AST.
///
/// Provides default implementations that walk into child nodes.
pub trait AstVisitor {
    /// Visit a declaration.
    fn visit_decl(&mut self, decl: &Located<Decl>) {
        self.walk_decl(decl);
    }
    /// Visit an expression.
    fn visit_expr(&mut self, expr: &Located<SurfaceExpr>) {
        self.walk_expr(expr);
    }
    /// Walk into child declarations.
    fn walk_decl(&mut self, decl: &Located<Decl>) {
        match &decl.value {
            Decl::Definition { val, ty, .. } => {
                if let Some(ty_expr) = ty {
                    self.visit_expr(ty_expr);
                }
                self.visit_expr(val);
            }
            Decl::Theorem { ty, proof, .. } => {
                self.visit_expr(ty);
                self.visit_expr(proof);
            }
            Decl::Axiom { ty, .. } => {
                self.visit_expr(ty);
            }
            Decl::Namespace { decls, .. } => {
                for d in decls {
                    self.visit_decl(d);
                }
            }
            Decl::SectionDecl { decls, .. } => {
                for d in decls {
                    self.visit_decl(d);
                }
            }
            Decl::Mutual { decls } => {
                for d in decls {
                    self.visit_decl(d);
                }
            }
            _ => {}
        }
    }
    /// Walk into child expressions.
    fn walk_expr(&mut self, expr: &Located<SurfaceExpr>) {
        match &expr.value {
            SurfaceExpr::App(f, arg) => {
                self.visit_expr(f);
                self.visit_expr(arg);
            }
            SurfaceExpr::Lam(_, body) => {
                self.visit_expr(body);
            }
            SurfaceExpr::Pi(_, body) => {
                self.visit_expr(body);
            }
            SurfaceExpr::Let(_, ty, val, body) => {
                if let Some(ty_expr) = ty {
                    self.visit_expr(ty_expr);
                }
                self.visit_expr(val);
                self.visit_expr(body);
            }
            SurfaceExpr::Ann(e, t) => {
                self.visit_expr(e);
                self.visit_expr(t);
            }
            SurfaceExpr::If(cond, then_e, else_e) => {
                self.visit_expr(cond);
                self.visit_expr(then_e);
                self.visit_expr(else_e);
            }
            SurfaceExpr::Match(scrutinee, arms) => {
                self.visit_expr(scrutinee);
                for arm in arms {
                    self.visit_expr(&arm.rhs);
                }
            }
            SurfaceExpr::Have(_, ty, proof, body) => {
                self.visit_expr(ty);
                self.visit_expr(proof);
                self.visit_expr(body);
            }
            SurfaceExpr::Proj(e, _) => {
                self.visit_expr(e);
            }
            SurfaceExpr::NamedArg(f, _, e) => {
                self.visit_expr(f);
                self.visit_expr(e);
            }
            SurfaceExpr::ListLit(elems) => {
                for elem in elems {
                    self.visit_expr(elem);
                }
            }
            SurfaceExpr::Tuple(elems) => {
                for elem in elems {
                    self.visit_expr(elem);
                }
            }
            SurfaceExpr::AnonymousCtor(fields) => {
                for field in fields {
                    self.visit_expr(field);
                }
            }
            SurfaceExpr::Suffices(_, ty, body) => {
                self.visit_expr(ty);
                self.visit_expr(body);
            }
            SurfaceExpr::Show(ty, e) => {
                self.visit_expr(ty);
                self.visit_expr(e);
            }
            SurfaceExpr::Return(e) => {
                self.visit_expr(e);
            }
            _ => {}
        }
    }
}
/// Analyze names used in an expression and collect variable references.
pub fn collect_var_refs(expr: &SurfaceExpr) -> HashSet<String> {
    let mut refs = HashSet::new();
    collect_var_refs_inner(expr, &mut refs);
    refs
}
fn collect_var_refs_inner(expr: &SurfaceExpr, refs: &mut HashSet<String>) {
    match expr {
        SurfaceExpr::Var(name) => {
            refs.insert(name.clone());
        }
        SurfaceExpr::App(f, arg) => {
            collect_var_refs_inner(&f.value, refs);
            collect_var_refs_inner(&arg.value, refs);
        }
        SurfaceExpr::Lam(binders, body) => {
            for binder in binders {
                if let Some(ty) = &binder.ty {
                    collect_var_refs_inner(&ty.value, refs);
                }
            }
            collect_var_refs_inner(&body.value, refs);
        }
        SurfaceExpr::Pi(binders, body) => {
            for binder in binders {
                if let Some(ty) = &binder.ty {
                    collect_var_refs_inner(&ty.value, refs);
                }
            }
            collect_var_refs_inner(&body.value, refs);
        }
        SurfaceExpr::Let(_, ty, val, body) => {
            if let Some(ty_expr) = ty {
                collect_var_refs_inner(&ty_expr.value, refs);
            }
            collect_var_refs_inner(&val.value, refs);
            collect_var_refs_inner(&body.value, refs);
        }
        SurfaceExpr::Ann(e, t) => {
            collect_var_refs_inner(&e.value, refs);
            collect_var_refs_inner(&t.value, refs);
        }
        SurfaceExpr::If(c, t, e) => {
            collect_var_refs_inner(&c.value, refs);
            collect_var_refs_inner(&t.value, refs);
            collect_var_refs_inner(&e.value, refs);
        }
        SurfaceExpr::Match(scrut, arms) => {
            collect_var_refs_inner(&scrut.value, refs);
            for arm in arms {
                collect_var_refs_inner(&arm.rhs.value, refs);
            }
        }
        SurfaceExpr::Have(_, ty, proof, body) => {
            collect_var_refs_inner(&ty.value, refs);
            collect_var_refs_inner(&proof.value, refs);
            collect_var_refs_inner(&body.value, refs);
        }
        SurfaceExpr::Proj(e, _) => {
            collect_var_refs_inner(&e.value, refs);
        }
        SurfaceExpr::Return(e) => {
            collect_var_refs_inner(&e.value, refs);
        }
        SurfaceExpr::ListLit(elems)
        | SurfaceExpr::Tuple(elems)
        | SurfaceExpr::AnonymousCtor(elems) => {
            for elem in elems {
                collect_var_refs_inner(&elem.value, refs);
            }
        }
        _ => {}
    }
}
/// Collect all variable bindings introduced in an expression.
pub fn collect_binder_names(expr: &SurfaceExpr) -> Vec<String> {
    let mut names = Vec::new();
    collect_binder_names_inner(expr, &mut names);
    names
}
fn collect_binder_names_inner(expr: &SurfaceExpr, names: &mut Vec<String>) {
    match expr {
        SurfaceExpr::Lam(binders, body) => {
            for binder in binders {
                names.push(binder.name.clone());
            }
            collect_binder_names_inner(&body.value, names);
        }
        SurfaceExpr::Pi(binders, body) => {
            for binder in binders {
                names.push(binder.name.clone());
            }
            collect_binder_names_inner(&body.value, names);
        }
        SurfaceExpr::Let(name, _, _, body) => {
            names.push(name.clone());
            collect_binder_names_inner(&body.value, names);
        }
        _ => {}
    }
}
/// Check if a name follows snake_case convention.
pub fn is_snake_case(name: &str) -> bool {
    if name.is_empty() || name.starts_with('_') {
        return true;
    }
    name.chars()
        .all(|c| c.is_lowercase() || c.is_ascii_digit() || c == '_')
}
/// Check if a name follows PascalCase convention.
pub fn is_pascal_case(name: &str) -> bool {
    if name.is_empty() {
        return true;
    }
    let first = name.chars().next().expect("name is non-empty");
    first.is_uppercase() && !name.contains('_')
}
/// Check if a name follows camelCase convention.
pub fn is_camel_case(name: &str) -> bool {
    if name.is_empty() {
        return true;
    }
    let first = name.chars().next().expect("name is non-empty");
    first.is_lowercase() && !name.contains('_')
}
/// Convert a name to snake_case.
pub fn to_snake_case(name: &str) -> String {
    let mut result = String::new();
    for (i, ch) in name.chars().enumerate() {
        if ch.is_uppercase() {
            if i > 0 {
                result.push('_');
            }
            for lower in ch.to_lowercase() {
                result.push(lower);
            }
        } else {
            result.push(ch);
        }
    }
    result
}
/// Convert a name to PascalCase.
pub fn to_pascal_case(name: &str) -> String {
    name.split('_')
        .filter(|s| !s.is_empty())
        .map(|s| {
            let mut chars = s.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => c.to_uppercase().to_string() + &chars.as_str().to_lowercase(),
            }
        })
        .collect()
}
/// Convert a byte offset to (line, column).
pub fn offset_to_line_col(source: &str, offset: usize) -> (usize, usize) {
    let clamped = offset.min(source.len());
    let mut line = 0;
    let mut col = 0;
    for (i, ch) in source.chars().enumerate() {
        if i >= clamped {
            break;
        }
        if ch == '\n' {
            line += 1;
            col = 0;
        } else {
            col += 1;
        }
    }
    (line, col)
}
/// Well-known lint identifiers.
pub mod lint_ids {
    use super::LintId;
    /// Unused variable lint.
    pub fn unused_variable() -> LintId {
        LintId::new("unused_variable")
    }
    /// Unused import lint.
    pub fn unused_import() -> LintId {
        LintId::new("unused_import")
    }
    /// Deprecated API lint.
    pub fn deprecated_api() -> LintId {
        LintId::new("deprecated_api")
    }
    /// Redundant pattern lint.
    pub fn redundant_pattern() -> LintId {
        LintId::new("redundant_pattern")
    }
    /// Simplifiable expression lint.
    pub fn simplifiable_expr() -> LintId {
        LintId::new("simplifiable_expr")
    }
    /// Missing documentation lint.
    pub fn missing_doc() -> LintId {
        LintId::new("missing_doc")
    }
    /// Naming convention lint.
    pub fn naming_convention() -> LintId {
        LintId::new("naming_convention")
    }
    /// Dead code lint.
    pub fn dead_code() -> LintId {
        LintId::new("dead_code")
    }
    /// Unreachable code lint.
    pub fn unreachable_code() -> LintId {
        LintId::new("unreachable_code")
    }
    /// Style lint.
    pub fn style() -> LintId {
        LintId::new("style")
    }
    /// Unused where clause lint.
    pub fn unused_where() -> LintId {
        LintId::new("unused_where")
    }
    /// Redundant type annotation lint.
    pub fn redundant_type_annotation() -> LintId {
        LintId::new("redundant_type_annotation")
    }
    /// Empty match lint.
    pub fn empty_match() -> LintId {
        LintId::new("empty_match")
    }
    /// Double negation lint.
    pub fn double_negation() -> LintId {
        LintId::new("double_negation")
    }
    /// Identity function application lint.
    pub fn identity_application() -> LintId {
        LintId::new("identity_application")
    }
    /// Unnecessary parentheses lint.
    pub fn unnecessary_parens() -> LintId {
        LintId::new("unnecessary_parens")
    }
    /// Trailing whitespace lint.
    pub fn trailing_whitespace() -> LintId {
        LintId::new("trailing_whitespace")
    }
    /// Inconsistent indentation lint.
    pub fn inconsistent_indentation() -> LintId {
        LintId::new("inconsistent_indentation")
    }
    /// Long line lint.
    pub fn long_line() -> LintId {
        LintId::new("long_line")
    }
    /// Missing final newline lint.
    pub fn missing_final_newline() -> LintId {
        LintId::new("missing_final_newline")
    }
    /// Sorry in proof lint.
    pub fn sorry_in_proof() -> LintId {
        LintId::new("sorry_in_proof")
    }
    /// Unused hypothesis in proof lint.
    pub fn unused_hypothesis() -> LintId {
        LintId::new("unused_hypothesis")
    }
    /// Redundant assumption lint.
    pub fn redundant_assumption() -> LintId {
        LintId::new("redundant_assumption")
    }
    /// Deprecated tactic lint.
    pub fn deprecated_tactic() -> LintId {
        LintId::new("deprecated_tactic")
    }
    /// Long proof block lint.
    pub fn long_proof() -> LintId {
        LintId::new("long_proof")
    }
    /// Missing docstring lint.
    pub fn missing_docstring() -> LintId {
        LintId::new("missing_docstring")
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lint_id_matches_pattern() {
        let id = LintId::new("unused_variable");
        assert!(id.matches_pattern("unused_*"));
        assert!(id.matches_pattern("*variable"));
        assert!(id.matches_pattern("*"));
        assert!(id.matches_pattern("unused_variable"));
        assert!(!id.matches_pattern("style_*"));
    }
    #[test]
    fn test_severity_ordering() {
        assert!(Severity::Error < Severity::Warning);
        assert!(Severity::Warning < Severity::Info);
        assert!(Severity::Info < Severity::Hint);
    }
    #[test]
    fn test_source_range_operations() {
        let r1 = SourceRange::new(10, 20);
        let r2 = SourceRange::new(15, 25);
        assert!(r1.overlaps(&r2));
        let merged = r1.merge(&r2);
        assert_eq!(merged.start, 10);
        assert_eq!(merged.end, 25);
    }
    #[test]
    fn test_auto_fix_apply() {
        let source = "hello world";
        let fix = AutoFix::replacement("fix", SourceRange::new(6, 11), "rust".to_string());
        assert_eq!(fix.apply(source), "hello rust");
    }
    #[test]
    fn test_naming_conventions() {
        assert!(is_snake_case("hello_world"));
        assert!(!is_snake_case("helloWorld"));
        assert!(is_pascal_case("HelloWorld"));
        assert!(!is_pascal_case("hello_world"));
        assert_eq!(to_snake_case("HelloWorld"), "hello_world");
        assert_eq!(to_pascal_case("hello_world"), "HelloWorld");
    }
    #[test]
    fn test_suppression_parser() {
        let comment = "@[nolint unused_variable]";
        let sup =
            SuppressionParser::parse_comment(comment, 0).expect("parse_comment should succeed");
        assert!(sup.suppresses(&LintId::new("unused_variable")));
        assert!(!sup.suppresses(&LintId::new("style")));
    }
    #[test]
    fn test_offset_to_line_col() {
        let source = "line1\nline2\nline3";
        assert_eq!(offset_to_line_col(source, 0), (0, 0));
        assert_eq!(offset_to_line_col(source, 6), (1, 0));
        assert_eq!(offset_to_line_col(source, 12), (2, 0));
    }
    #[test]
    fn test_lint_config_enabled() {
        let mut config = LintConfig::default();
        let lint = LintId::new("test_lint");
        assert!(config.is_enabled(&lint));
        config.disable(lint.clone());
        assert!(!config.is_enabled(&lint));
        config.enable(lint.clone());
        assert!(config.is_enabled(&lint));
    }
    #[test]
    fn test_lint_summary() {
        let diags = vec![
            LintDiagnostic::new(
                LintId::new("test"),
                Severity::Error,
                "test error",
                SourceRange::new(0, 5),
            ),
            LintDiagnostic::new(
                LintId::new("test"),
                Severity::Warning,
                "test warning",
                SourceRange::new(10, 15),
            ),
        ];
        let summary = LintSummary::from_diagnostics(&diags);
        assert_eq!(summary.total, 2);
        assert_eq!(summary.errors, 1);
        assert_eq!(summary.warnings, 1);
        assert!(!summary.passed());
    }
}
#[cfg(test)]
mod framework_extension_tests {
    use super::*;
    #[test]
    fn lint_pass_scheduler_enabled_passes() {
        let mut scheduler = LintPassScheduler::new();
        scheduler.add_pass("style");
        scheduler.add_pass("security");
        scheduler.add_pass("performance");
        scheduler.disable_pass("security");
        let enabled = scheduler.enabled_passes();
        assert_eq!(enabled.len(), 2);
        assert!(!enabled.contains(&"security"));
        assert_eq!(scheduler.total_scheduled(), 3);
    }
    #[test]
    fn lint_pass_scheduler_empty() {
        let scheduler = LintPassScheduler::new();
        assert!(scheduler.enabled_passes().is_empty());
    }
    #[test]
    fn annotation_parser_ignore_line() {
        let source = "-- oxilean-ignore: unused_import, dead_code\ntheorem foo : True := trivial";
        let anns = AnnotationParser::parse(source);
        assert_eq!(anns.len(), 1);
        assert_eq!(anns[0].kind, AnnotationKind::IgnoreLine);
        assert_eq!(anns[0].lint_ids.len(), 2);
        assert!(anns[0].lint_ids.contains(&"unused_import".to_string()));
    }
    #[test]
    fn annotation_parser_disable_next_line() {
        let source = "-- oxilean-disable-next-line: naming_convention\nlet MyVar = 1";
        let anns = AnnotationParser::parse(source);
        assert_eq!(anns.len(), 1);
        assert_eq!(anns[0].kind, AnnotationKind::DisableNextLine);
    }
    #[test]
    fn annotation_parser_enable_disable_all() {
        let source = "-- oxilean-disable-all\ncode\n-- oxilean-enable-all";
        let anns = AnnotationParser::parse(source);
        assert_eq!(anns.len(), 2);
        assert_eq!(anns[0].kind, AnnotationKind::DisableAll);
        assert_eq!(anns[1].kind, AnnotationKind::EnableAll);
    }
    #[test]
    fn annotation_parser_count_suppressions() {
        let source = "-- oxilean-ignore: a\n-- oxilean-ignore: b\nnormal line";
        assert_eq!(AnnotationParser::count_suppressions(source), 2);
    }
    #[test]
    fn lint_file_stats_density() {
        let mut stats = LintFileStats::new("foo.ox", 100);
        stats.diagnostic_count = 5;
        assert!((stats.diagnostic_density() - 0.05).abs() < 1e-9);
    }
    #[test]
    fn lint_file_stats_suppression_rate() {
        let mut stats = LintFileStats::new("foo.ox", 100);
        stats.diagnostic_count = 3;
        stats.suppressed_count = 1;
        let rate = stats.suppression_rate();
        assert!((rate - 0.25).abs() < 1e-9);
    }
    #[test]
    fn lint_file_stats_zero_lines() {
        let stats = LintFileStats::new("empty.ox", 0);
        assert_eq!(stats.diagnostic_density(), 0.0);
        assert_eq!(stats.suppression_rate(), 0.0);
    }
}
#[cfg(test)]
mod pass_dep_tests {
    use super::*;
    #[test]
    fn lint_pass_deps_require_and_conflict() {
        let deps = LintPassDependencies::new()
            .require("type_check")
            .conflict("legacy_pass");
        assert!(deps.needs("type_check"));
        assert!(!deps.needs("other"));
        assert!(deps.conflicts_with("legacy_pass"));
        assert!(!deps.conflicts_with("type_check"));
    }
    #[test]
    fn lint_pass_deps_is_empty() {
        let deps = LintPassDependencies::new();
        assert!(deps.is_empty());
        let deps2 = deps.require("something");
        assert!(!deps2.is_empty());
    }
}
