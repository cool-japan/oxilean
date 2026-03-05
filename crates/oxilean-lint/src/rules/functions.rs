//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::framework::{
    collect_var_refs, is_camel_case, is_pascal_case, is_snake_case, lint_ids, to_snake_case,
    AutoFix, LintCategory, LintContext, LintDiagnostic, LintId, LintRule, Severity, SourceRange,
};
use oxilean_parse::{Binder, Decl, DoAction, Located, MatchArm, Pattern, Span, SurfaceExpr};

use super::types::{
    ComplexExprRule, DeadCodeRule, DeprecatedApiRule, DeprecatedTacticRule, DoubleNegationRule,
    EmptyMatchRule, IdentityApplicationRule, InconsistentBinderRule, LargePatternMatchRule,
    LongProofRule, MissingDocRule, MissingDocstringRule, NamingConventionRule,
    RedundantAssumptionRule, RedundantPatternRule, RedundantTypeAnnotationRule,
    SimplifiableExprRule, SorryInProofRule, StyleRule, SuspiciousShadowRule,
    UniverseAnnotationRule, UnnecessaryParensRule, UnreachableCodeRule, UnusedHypothesisRule,
    UnusedImportRule, UnusedVariableRule, UnusedWhereRule,
};

/// Create a lint registry with all default rules.
pub fn default_rules() -> Vec<Box<dyn LintRule>> {
    vec![
        Box::new(UnusedVariableRule::new()),
        Box::new(UnusedImportRule::new()),
        Box::new(DeprecatedApiRule::new()),
        Box::new(RedundantPatternRule::new()),
        Box::new(SimplifiableExprRule::new()),
        Box::new(MissingDocRule::new()),
        Box::new(NamingConventionRule::new()),
        Box::new(DeadCodeRule::new()),
        Box::new(UnreachableCodeRule::new()),
        Box::new(StyleRule::new()),
        Box::new(RedundantTypeAnnotationRule::new()),
        Box::new(EmptyMatchRule::new()),
        Box::new(DoubleNegationRule::new()),
        Box::new(UnnecessaryParensRule::new()),
        Box::new(UnusedWhereRule::new()),
        Box::new(IdentityApplicationRule::new()),
        Box::new(InconsistentBinderRule::new()),
        Box::new(LargePatternMatchRule::new()),
        Box::new(SuspiciousShadowRule::new()),
        Box::new(ComplexExprRule::new()),
        Box::new(UniverseAnnotationRule::new()),
        Box::new(SorryInProofRule::new()),
        Box::new(UnusedHypothesisRule::new()),
        Box::new(RedundantAssumptionRule::new()),
        Box::new(DeprecatedTacticRule::new()),
        Box::new(LongProofRule::new()),
        Box::new(MissingDocstringRule::new()),
    ]
}
/// Create a default lint registry with all rules registered.
pub fn default_registry() -> crate::framework::LintRegistry {
    let mut registry = crate::framework::LintRegistry::new();
    for rule in default_rules() {
        registry.register(rule);
    }
    registry
}
#[cfg(test)]
mod tests {
    use super::*;
    fn _make_located<T>(value: T) -> Located<T> {
        Located {
            value,
            span: Span {
                start: 0,
                end: 10,
                line: 1,
                column: 1,
            },
        }
    }
    #[test]
    fn test_unused_variable_rule_id() {
        let rule = UnusedVariableRule::new();
        assert_eq!(rule.id(), lint_ids::unused_variable());
        assert_eq!(rule.name(), "unused variable");
    }
    #[test]
    fn test_naming_convention_checks() {
        let rule = NamingConventionRule::new();
        assert_eq!(rule.id(), lint_ids::naming_convention());
        assert_eq!(rule.category(), LintCategory::Style);
    }
    #[test]
    fn test_style_rule_defaults() {
        let rule = StyleRule::new();
        assert_eq!(rule.max_line_length, 100);
        assert!(rule.disallow_tabs);
        assert!(rule.check_trailing_whitespace);
    }
    #[test]
    fn test_default_rules_count() {
        let rules = default_rules();
        assert!(
            rules.len() >= 20,
            "expected at least 20 rules, got {}",
            rules.len()
        );
    }
    #[test]
    fn test_default_registry() {
        let registry = default_registry();
        assert!(!registry.is_empty());
        assert!(registry.get(&lint_ids::unused_variable()).is_some());
        assert!(registry.get(&lint_ids::style()).is_some());
    }
    #[test]
    fn test_complex_expr_depth() {
        let rule = ComplexExprRule::new();
        let simple = SurfaceExpr::Var("x".to_string());
        assert_eq!(rule.measure_depth(&simple), 0);
    }
    #[test]
    fn test_sorry_in_proof_contains_sorry() {
        assert!(SorryInProofRule::contains_sorry(&SurfaceExpr::Var(
            "sorry".to_string()
        )));
        assert!(!SorryInProofRule::contains_sorry(&SurfaceExpr::Var(
            "rfl".to_string()
        )));
    }
    #[test]
    fn test_sorry_in_proof_nested() {
        use oxilean_parse::Span;
        let dummy_span = || Span {
            start: 0,
            end: 0,
            line: 1,
            column: 1,
        };
        let sorry_var = SurfaceExpr::Var("sorry".to_string());
        let ok_var = SurfaceExpr::Var("rfl".to_string());
        let located_sorry = Box::new(Located {
            value: sorry_var,
            span: dummy_span(),
        });
        let located_ok = || {
            Box::new(Located {
                value: ok_var.clone(),
                span: dummy_span(),
            })
        };
        let app_with_sorry = SurfaceExpr::App(located_ok(), located_sorry);
        assert!(SorryInProofRule::contains_sorry(&app_with_sorry));
        let app_clean = SurfaceExpr::App(
            located_ok(),
            Box::new(Located {
                value: ok_var,
                span: dummy_span(),
            }),
        );
        assert!(!SorryInProofRule::contains_sorry(&app_clean));
    }
    #[test]
    fn test_sorry_in_proof_rule_metadata() {
        let rule = SorryInProofRule::new();
        assert_eq!(rule.id(), lint_ids::sorry_in_proof());
        assert_eq!(rule.name(), "sorry in proof");
        assert_eq!(rule.category(), LintCategory::Correctness);
        assert_eq!(rule.default_severity(), Severity::Warning);
    }
    #[test]
    fn test_unused_hypothesis_rule_metadata() {
        let rule = UnusedHypothesisRule::new();
        assert_eq!(rule.id(), lint_ids::unused_hypothesis());
        assert_eq!(rule.name(), "unused hypothesis");
        assert_eq!(rule.category(), LintCategory::Correctness);
        assert_eq!(rule.default_severity(), Severity::Warning);
    }
    #[test]
    fn test_redundant_assumption_rule_metadata() {
        let rule = RedundantAssumptionRule::new();
        assert_eq!(rule.id(), lint_ids::redundant_assumption());
        assert_eq!(rule.name(), "redundant assumption");
        assert_eq!(rule.category(), LintCategory::Correctness);
        assert_eq!(rule.default_severity(), Severity::Warning);
    }
    #[test]
    fn test_deprecated_tactic_rule_metadata() {
        let rule = DeprecatedTacticRule::new();
        assert_eq!(rule.id(), lint_ids::deprecated_tactic());
        assert_eq!(rule.name(), "deprecated tactic");
        assert_eq!(rule.category(), LintCategory::Deprecated);
        assert_eq!(rule.default_severity(), Severity::Warning);
        assert!(rule.deprecated.contains_key("norm_cast"));
        assert!(rule.deprecated.contains_key("finish"));
        assert!(rule.deprecated.contains_key("blast"));
    }
    #[test]
    fn test_long_proof_rule_metadata() {
        let rule = LongProofRule::new();
        assert_eq!(rule.id(), lint_ids::long_proof());
        assert_eq!(rule.name(), "long proof");
        assert_eq!(rule.category(), LintCategory::Complexity);
        assert_eq!(rule.default_severity(), Severity::Info);
        assert_eq!(rule.max_tactic_lines, 50);
        let custom = LongProofRule::with_threshold(20);
        assert_eq!(custom.max_tactic_lines, 20);
    }
    #[test]
    fn test_long_proof_count_tactic_lines() {
        let rule = LongProofRule::new();
        let var = SurfaceExpr::Var("x".to_string());
        assert_eq!(rule.count_tactic_lines(&var), 0);
        let tactics: Vec<Located<String>> = (0..3)
            .map(|i| Located {
                value: format!("tac_{}", i),
                span: Span {
                    start: 0,
                    end: 0,
                    line: 1,
                    column: 1,
                },
            })
            .collect();
        let by_tac = SurfaceExpr::ByTactic(tactics);
        assert_eq!(rule.count_tactic_lines(&by_tac), 3);
    }
    #[test]
    fn test_missing_docstring_rule_metadata() {
        let rule = MissingDocstringRule::new();
        assert_eq!(rule.id(), lint_ids::missing_docstring());
        assert_eq!(rule.name(), "missing docstring");
        assert_eq!(rule.category(), LintCategory::Documentation);
        assert_eq!(rule.default_severity(), Severity::Info);
    }
    #[test]
    fn test_missing_docstring_has_doc_comment() {
        let source_with_doc = "--! Adds two numbers\ndef add := 0\n";
        let def_offset = source_with_doc
            .find("def")
            .expect("substring should be found");
        assert!(MissingDocstringRule::has_doc_comment_before(
            source_with_doc,
            def_offset
        ));
        let source_slash_doc = "/-- A lemma\ntheorem foo := sorry\n";
        let thm_offset = source_slash_doc
            .find("theorem")
            .expect("substring should be found");
        assert!(MissingDocstringRule::has_doc_comment_before(
            source_slash_doc,
            thm_offset
        ));
        let source_no_doc = "\ndef bar := 0\n";
        let bar_offset = source_no_doc
            .find("def")
            .expect("substring should be found");
        assert!(!MissingDocstringRule::has_doc_comment_before(
            source_no_doc,
            bar_offset
        ));
    }
    #[test]
    fn test_default_rules_includes_new_rules() {
        let rules = default_rules();
        let ids: Vec<LintId> = rules.iter().map(|r| r.id()).collect();
        assert!(ids.contains(&lint_ids::unused_hypothesis()));
        assert!(ids.contains(&lint_ids::redundant_assumption()));
        assert!(ids.contains(&lint_ids::deprecated_tactic()));
        assert!(ids.contains(&lint_ids::long_proof()));
        assert!(ids.contains(&lint_ids::missing_docstring()));
        assert!(
            rules.len() >= 27,
            "expected at least 27 rules, got {}",
            rules.len()
        );
    }
    #[test]
    fn test_redundant_assumption_collect_have_names() {
        let inner_have = SurfaceExpr::Have(
            "h2".to_string(),
            Box::new(Located {
                value: SurfaceExpr::Var("Nat".to_string()),
                span: Span {
                    start: 0,
                    end: 0,
                    line: 1,
                    column: 1,
                },
            }),
            Box::new(Located {
                value: SurfaceExpr::Var("zero".to_string()),
                span: Span {
                    start: 0,
                    end: 0,
                    line: 1,
                    column: 1,
                },
            }),
            Box::new(Located {
                value: SurfaceExpr::Var("h2".to_string()),
                span: Span {
                    start: 0,
                    end: 0,
                    line: 1,
                    column: 1,
                },
            }),
        );
        let mut names = Vec::new();
        RedundantAssumptionRule::collect_have_names(&inner_have, &mut names);
        assert!(names.contains(&"h2".to_string()));
    }
}
