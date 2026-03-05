//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    CompatIssue, CompatLevel, FieldVisibility, IssueSeverity, Lean4Attribute, Lean4CompatChecker,
    Lean4CompatMatrix, Lean4CompatReport, Lean4Constructor, Lean4DocstringExtractor,
    Lean4ElabError, Lean4ErrorKind, Lean4Feature, Lean4FieldDescriptor, Lean4ImportResolver,
    Lean4InductiveDescriptor, Lean4KeywordCategory, Lean4KeywordClassifier, Lean4NameConverter,
    Lean4NamespaceTracker, Lean4OpenCommand, Lean4Option, Lean4OptionConfig, Lean4PositionMapper,
    Lean4SectionManager, Lean4StructureDescriptor, Lean4SyntaxAdapter, Lean4SyntaxDiff,
    Lean4SyntaxVersion, Lean4TermRewriter, Lean4Token, Lean4TokenKind, Lean4TypeAnnotation,
    Lean4UniverseNormalizer, ScopeKind,
};
use oxilean_kernel::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lean4_compat::*;
    #[test]
    fn test_compat_matrix_new() {
        let matrix = Lean4CompatMatrix::new();
        assert_eq!(matrix.entries.len(), Lean4Feature::all().len());
    }
    #[test]
    fn test_compat_level_check() {
        let matrix = Lean4CompatMatrix::new();
        assert_eq!(
            matrix.compat_level(&Lean4Feature::TacticMode),
            CompatLevel::Full
        );
        assert!(matrix.partially_supported(&Lean4Feature::MacroExpansion));
        assert!(!matrix.partially_supported(&Lean4Feature::TacticMode));
        assert!(matrix.is_supported(&Lean4Feature::DoNotation));
    }
    #[test]
    fn test_unsupported_features() {
        let matrix = Lean4CompatMatrix::new();
        let unsupported = matrix.unsupported_features();
        assert!(unsupported
            .iter()
            .any(|f| **f == Lean4Feature::MetaProgramming));
    }
    #[test]
    fn test_adapt_arrow() {
        let src = "def foo : Nat := bar => baz";
        let adapted = Lean4SyntaxAdapter::adapt_arrow_syntax(src);
        assert!(adapted.contains("->"));
        assert!(!adapted.contains("=>"));
    }
    #[test]
    fn test_adapt_lambda() {
        let src = "fun x => x + 1";
        let adapted = Lean4SyntaxAdapter::adapt_lambda(src);
        assert!(adapted.contains("->"));
        assert!(!adapted.contains("=>"));
    }
    #[test]
    fn test_adapt_match() {
        let src = "match n with | 0 => zero | k => succ k";
        let adapted = Lean4SyntaxAdapter::adapt_match_syntax(src);
        assert!(!adapted.contains("=>"));
        assert!(adapted.contains("->"));
    }
    #[test]
    fn test_adapt_all() {
        let src = "fun x => match x with | 0 => zero | k => succ k";
        let adapted = Lean4SyntaxAdapter::adapt_all(src);
        assert!(!adapted.contains("=>"));
    }
    #[test]
    fn test_name_converter() {
        assert_eq!(Lean4NameConverter::to_oxilean_name("Nat.add"), "Nat.add");
        assert_eq!(Lean4NameConverter::from_oxilean_name("Nat.add"), "Nat.add");
        assert!(Lean4NameConverter::is_valid_oxilean_name("foo"));
        assert!(Lean4NameConverter::is_valid_oxilean_name("Nat.add"));
        assert!(Lean4NameConverter::is_valid_oxilean_name("foo'"));
        assert!(!Lean4NameConverter::is_valid_oxilean_name(""));
        assert!(!Lean4NameConverter::is_valid_oxilean_name("1foo"));
    }
}
#[cfg(test)]
mod extra_tests {
    use super::*;
    use crate::lean4_compat::*;
    #[test]
    fn test_lean4_feature_labels_all() {
        assert_eq!(Lean4Feature::DoNotation.label(), "do-notation");
        assert_eq!(Lean4Feature::TacticMode.label(), "tactic-mode");
        assert_eq!(
            Lean4Feature::UniversePolymorphism.label(),
            "universe-polymorphism"
        );
        assert_eq!(Lean4Feature::MutualRecursion.label(), "mutual-recursion");
        assert_eq!(Lean4Feature::WhereBindings.label(), "where-bindings");
        assert_eq!(Lean4Feature::Notation.label(), "notation");
        assert_eq!(Lean4Feature::PatternMatching.label(), "pattern-matching");
        assert_eq!(Lean4Feature::MetaProgramming.label(), "meta-programming");
    }
    #[test]
    fn test_lean4_feature_all_count() {
        assert_eq!(Lean4Feature::all().len(), 12);
    }
    #[test]
    fn test_lean4_feature_all_labels_nonempty() {
        for feat in Lean4Feature::all() {
            assert!(!feat.label().is_empty(), "label empty for {:?}", feat);
        }
    }
    #[test]
    fn test_lean4_feature_all_unique_labels() {
        let labels: Vec<&str> = Lean4Feature::all().iter().map(|f| f.label()).collect();
        let unique: std::collections::HashSet<&str> = labels.iter().copied().collect();
        assert_eq!(labels.len(), unique.len());
    }
    #[test]
    fn test_lean4_feature_eq_and_clone() {
        let f1 = Lean4Feature::TacticMode;
        let f2 = f1.clone();
        assert_eq!(f1, f2);
        assert_ne!(f1, Lean4Feature::DoNotation);
    }
    #[test]
    fn test_compat_matrix_full_supported() {
        let matrix = Lean4CompatMatrix::new();
        let full = matrix.full_supported_features();
        assert!(full.iter().any(|f| **f == Lean4Feature::DoNotation));
        assert!(full.iter().any(|f| **f == Lean4Feature::TacticMode));
        assert!(!full.iter().any(|f| **f == Lean4Feature::MacroExpansion));
    }
    #[test]
    fn test_compat_matrix_report_contains_all_features() {
        let matrix = Lean4CompatMatrix::new();
        let report = matrix.report();
        assert!(report.contains("do-notation"));
        assert!(report.contains("tactic-mode"));
        assert!(report.contains("Full"));
        assert!(report.contains("Partial"));
        assert!(report.contains("Stub"));
    }
    #[test]
    fn test_compat_level_is_any_support_all_variants() {
        assert!(CompatLevel::Full.is_any_support());
        assert!(CompatLevel::Partial("x".to_string()).is_any_support());
        assert!(!CompatLevel::Stub.is_any_support());
        assert!(!CompatLevel::Unsupported.is_any_support());
    }
    #[test]
    fn test_compat_level_clone_and_eq() {
        let cl = CompatLevel::Partial("msg".to_string());
        let cl2 = cl.clone();
        assert_eq!(cl, cl2);
    }
    #[test]
    fn test_adapt_do_notation() {
        let src = "x \u{2190} foo";
        let adapted = Lean4SyntaxAdapter::adapt_do_notation(src);
        assert!(adapted.contains("<-"));
        assert!(!adapted.contains('\u{2190}'));
    }
    #[test]
    fn test_adapt_where_clause() {
        let src = "def f := g where; h := 1";
        let adapted = Lean4SyntaxAdapter::adapt_where_clause(src);
        assert!(!adapted.contains("where;"));
        assert!(adapted.contains("where"));
    }
    #[test]
    fn test_adapt_match_multiple_arms() {
        let src = "match n with | 0 => zero | 1 => one | k => other k";
        let adapted = Lean4SyntaxAdapter::adapt_match_syntax(src);
        assert!(!adapted.contains("=>"));
        assert_eq!(adapted.matches("->").count(), 3);
    }
    #[test]
    fn test_adapt_all_composes() {
        let src = "x \u{2190} a; match x with | 0 => zero | _ => one";
        let adapted = Lean4SyntaxAdapter::adapt_all(src);
        assert!(!adapted.contains("=>"));
        assert!(!adapted.contains('\u{2190}'));
    }
    #[test]
    fn test_is_valid_oxilean_name_edge_cases() {
        assert!(Lean4NameConverter::is_valid_oxilean_name("_x"));
        assert!(Lean4NameConverter::is_valid_oxilean_name("h'"));
        assert!(Lean4NameConverter::is_valid_oxilean_name("Nat.succ"));
        assert!(!Lean4NameConverter::is_valid_oxilean_name("3foo"));
        assert!(!Lean4NameConverter::is_valid_oxilean_name("foo-bar"));
        assert!(!Lean4NameConverter::is_valid_oxilean_name("foo bar"));
        assert!(!Lean4NameConverter::is_valid_oxilean_name(".foo"));
    }
    #[test]
    fn test_name_roundtrip() {
        for name in &["Nat.add", "List.map", "foo", "Bar.baz"] {
            let to = Lean4NameConverter::to_oxilean_name(name);
            let back = Lean4NameConverter::from_oxilean_name(&to);
            assert_eq!(&back, name);
        }
    }
    #[test]
    fn test_default_matrix_has_all_features() {
        let m = Lean4CompatMatrix::default();
        assert_eq!(m.entries.len(), Lean4Feature::all().len());
    }
    #[test]
    fn test_compat_matrix_all_features_covered() {
        let matrix = Lean4CompatMatrix::new();
        for feat in Lean4Feature::all() {
            let level = matrix.compat_level(&feat);
            let _ = level.is_any_support();
        }
    }
    #[test]
    fn test_compat_matrix_partially_supported_count_positive() {
        let matrix = Lean4CompatMatrix::new();
        let count = Lean4Feature::all()
            .iter()
            .filter(|f| matrix.partially_supported(f))
            .count();
        assert!(count > 0);
    }
    #[test]
    fn test_adapt_do_notation_multiple_binds() {
        let src = "x \u{2190} a; y \u{2190} b; return (x, y)";
        let adapted = Lean4SyntaxAdapter::adapt_do_notation(src);
        assert_eq!(adapted.matches("<-").count(), 2);
    }
}
/// Parse a `@[attr1, attr2]` string into a list of attributes.
#[allow(dead_code)]
pub fn parse_lean4_attributes(src: &str) -> Vec<Lean4Attribute> {
    let src = src.trim();
    if !src.starts_with("@[") || !src.ends_with(']') {
        return Vec::new();
    }
    let inner = &src[2..src.len() - 1];
    inner
        .split(',')
        .map(|part| {
            let part = part.trim();
            let mut tokens = part.split_whitespace();
            let name = tokens.next().unwrap_or("").to_string();
            let args: Vec<String> = tokens.map(|s| s.to_string()).collect();
            Lean4Attribute { name, args }
        })
        .filter(|a| !a.name.is_empty())
        .collect()
}
/// Build the canonical set of compatibility reports.
#[allow(dead_code)]
pub fn build_compat_reports() -> Vec<Lean4CompatReport> {
    vec![
        Lean4CompatReport::new("do-notation", CompatLevel::Full)
            .with_supported("Full monadic do-notation with ← binds and return.")
            .with_workaround("Use <- instead of the Unicode left-arrow when needed."),
        Lean4CompatReport::new(
            "macro-expansion",
            CompatLevel::Partial("hygienic macros not yet supported".to_string()),
        )
        .with_supported("Basic macro_rules with simple patterns.")
        .with_gap("Hygienic macro variable capture is not implemented.")
        .with_gap("Recursive macros are limited.")
        .with_workaround("Use def/theorem instead of macro for complex cases."),
        Lean4CompatReport::new(
            "auto-bound-implicits",
            CompatLevel::Partial("single-level auto-bind only".to_string()),
        )
        .with_supported("Single-level implicit variable auto-binding.")
        .with_gap("Nested auto-bound implicits in tactic blocks.")
        .with_workaround("Declare variables explicitly with `variable (x : T)`."),
        Lean4CompatReport::new(
            "structure-inheritance",
            CompatLevel::Partial("single extends only".to_string()),
        )
        .with_supported("Single-parent structure inheritance via `extends`.")
        .with_gap("Multiple inheritance with diamond resolution.")
        .with_workaround("Use composition instead of multiple extends."),
        Lean4CompatReport::new("declaration-attributes", CompatLevel::Full)
            .with_supported("All standard declaration attributes (@[simp], @[instance], etc.)."),
        Lean4CompatReport::new(
            "universe-polymorphism",
            CompatLevel::Partial("universe variables erased during elaboration".to_string()),
        )
        .with_supported("Universe variable declarations are parsed.")
        .with_gap("Universe constraints are not verified.")
        .with_gap("Universe-polymorphic definitions may not instantiate correctly.")
        .with_workaround("Use explicit Type levels (Type 0, Type 1) instead of Sort u."),
        Lean4CompatReport::new("tactic-mode", CompatLevel::Full)
            .with_supported("Full tactic mode with by blocks."),
        Lean4CompatReport::new("meta-programming", CompatLevel::Stub)
            .with_gap("Lean.Elab.Tactic APIs not available.")
            .with_gap("Quote/unquote and macro monad not implemented.")
            .with_workaround("Use external tools for code generation."),
        Lean4CompatReport::new("pattern-matching", CompatLevel::Full)
            .with_supported("Full dependent pattern matching in def and theorem."),
        Lean4CompatReport::new("mutual-recursion", CompatLevel::Full)
            .with_supported("Mutual recursive definitions via `mutual` blocks."),
        Lean4CompatReport::new(
            "where-bindings",
            CompatLevel::Partial("only top-level where supported".to_string()),
        )
        .with_supported("Top-level `where` local definitions.")
        .with_gap("Nested where blocks inside tactics.")
        .with_workaround("Lift nested where definitions to top-level helpers."),
        Lean4CompatReport::new(
            "notation",
            CompatLevel::Partial("basic fixity notation only".to_string()),
        )
        .with_supported("Basic infix/prefix/postfix notation declarations.")
        .with_gap("Mixfix notation with multiple holes.")
        .with_gap("Category-based syntax extensions.")
        .with_workaround("Use def + infixl/infixr for simple binary operators."),
    ]
}
/// Known syntax changes between Lean 4 versions relevant to OxiLean.
#[allow(dead_code)]
pub fn known_syntax_diffs() -> Vec<Lean4SyntaxDiff> {
    vec![
        Lean4SyntaxDiff::new(
            "`=>` replaced by `->` in match arms in OxiLean",
            Lean4SyntaxVersion::v4_0_0(),
            false,
        ),
        Lean4SyntaxDiff::new(
            "`fun x => body` becomes `fun x -> body` in OxiLean",
            Lean4SyntaxVersion::v4_0_0(),
            false,
        ),
        Lean4SyntaxDiff::new(
            "Universe annotations `.{u}` are stripped",
            Lean4SyntaxVersion::v4_0_0(),
            true,
        ),
        Lean4SyntaxDiff::new(
            "`where;` trailing semicolon removed",
            Lean4SyntaxVersion::v4_3_0(),
            true,
        ),
        Lean4SyntaxDiff::new(
            "`←` in do-blocks replaced by `<-`",
            Lean4SyntaxVersion::v4_0_0(),
            true,
        ),
    ]
}
#[cfg(test)]
mod compat_expansion_tests {
    use super::*;
    use crate::lean4_compat::*;
    #[test]
    fn test_type_annotation_brackets() {
        let ann = Lean4TypeAnnotation::Implicit;
        let (l, r) = ann.brackets();
        assert_eq!(l, "{");
        assert_eq!(r, "}");
    }
    #[test]
    fn test_type_annotation_is_implicit() {
        assert!(Lean4TypeAnnotation::Implicit.is_implicit());
        assert!(Lean4TypeAnnotation::Instance.is_implicit());
        assert!(Lean4TypeAnnotation::StrictImplicit.is_implicit());
        assert!(!Lean4TypeAnnotation::Ascription.is_implicit());
        assert!(!Lean4TypeAnnotation::AutoParam.is_implicit());
    }
    #[test]
    fn test_type_annotation_labels_nonempty() {
        for ann in Lean4TypeAnnotation::all() {
            assert!(!ann.label().is_empty(), "label empty for {:?}", ann);
        }
    }
    #[test]
    fn test_type_annotation_all_count() {
        assert_eq!(Lean4TypeAnnotation::all().len(), 6);
    }
    #[test]
    fn test_keyword_classifier_declaration() {
        assert_eq!(
            Lean4KeywordClassifier::classify("def"),
            Lean4KeywordCategory::Declaration
        );
        assert_eq!(
            Lean4KeywordClassifier::classify("theorem"),
            Lean4KeywordCategory::Declaration
        );
    }
    #[test]
    fn test_keyword_classifier_tactic() {
        assert_eq!(
            Lean4KeywordClassifier::classify("simp"),
            Lean4KeywordCategory::Tactic
        );
        assert_eq!(
            Lean4KeywordClassifier::classify("ring"),
            Lean4KeywordCategory::Tactic
        );
    }
    #[test]
    fn test_keyword_classifier_not_keyword() {
        assert_eq!(
            Lean4KeywordClassifier::classify("myFunc"),
            Lean4KeywordCategory::NotKeyword
        );
    }
    #[test]
    fn test_keyword_classifier_is_keyword() {
        assert!(Lean4KeywordClassifier::is_keyword("def"));
        assert!(Lean4KeywordClassifier::is_keyword("intro"));
        assert!(!Lean4KeywordClassifier::is_keyword("foo"));
    }
    #[test]
    fn test_declaration_keywords_nonempty() {
        assert!(!Lean4KeywordClassifier::declaration_keywords().is_empty());
    }
    #[test]
    fn test_tactic_keywords_nonempty() {
        assert!(!Lean4KeywordClassifier::tactic_keywords().is_empty());
    }
    #[test]
    fn test_namespace_keywords_include_import() {
        assert!(Lean4KeywordClassifier::namespace_keywords().contains(&"import"));
    }
    #[test]
    fn test_syntax_version_ordering() {
        let v400 = Lean4SyntaxVersion::v4_0_0();
        let v430 = Lean4SyntaxVersion::v4_3_0();
        let v460 = Lean4SyntaxVersion::v4_6_0();
        assert!(v400 < v430);
        assert!(v430 < v460);
        assert!(v460.is_at_least(&v400));
        assert!(!v400.is_at_least(&v460));
    }
    #[test]
    fn test_syntax_version_to_string() {
        assert_eq!(Lean4SyntaxVersion::v4_0_0().to_string(), "4.0.0");
        assert_eq!(Lean4SyntaxVersion::v4_6_0().to_string(), "4.6.0");
    }
    #[test]
    fn test_attribute_new() {
        let a = Lean4Attribute::new("simp");
        assert_eq!(a.name, "simp");
        assert!(a.args.is_empty());
        assert!(a.is_simp());
        assert!(!a.is_instance());
    }
    #[test]
    fn test_attribute_format_no_args() {
        let a = Lean4Attribute::new("simp");
        assert_eq!(a.format(), "@[simp]");
    }
    #[test]
    fn test_attribute_format_with_args() {
        let a = Lean4Attribute::with_args("simp", vec!["Nat.add_comm"]);
        assert_eq!(a.format(), "@[simp Nat.add_comm]");
    }
    #[test]
    fn test_attribute_is_reducibility() {
        assert!(Lean4Attribute::new("reducible").is_reducibility());
        assert!(Lean4Attribute::new("irreducible").is_reducibility());
        assert!(!Lean4Attribute::new("simp").is_reducibility());
    }
    #[test]
    fn test_parse_lean4_attributes_empty() {
        let attrs = parse_lean4_attributes("");
        assert!(attrs.is_empty());
    }
    #[test]
    fn test_parse_lean4_attributes_single() {
        let attrs = parse_lean4_attributes("@[simp]");
        assert_eq!(attrs.len(), 1);
        assert_eq!(attrs[0].name, "simp");
    }
    #[test]
    fn test_parse_lean4_attributes_multiple() {
        let attrs = parse_lean4_attributes("@[simp, instance, inline]");
        assert_eq!(attrs.len(), 3);
        assert_eq!(attrs[0].name, "simp");
        assert_eq!(attrs[1].name, "instance");
        assert_eq!(attrs[2].name, "inline");
    }
    #[test]
    fn test_strip_universe_annotations_basic() {
        let src = "List.{u} Nat.{v, w}";
        let out = Lean4UniverseNormalizer::strip_universe_annotations(src);
        assert!(!out.contains(".{"));
        assert!(out.contains("List"));
        assert!(out.contains("Nat"));
    }
    #[test]
    fn test_normalize_sort_star() {
        let src = "Sort* Type*";
        let out = Lean4UniverseNormalizer::normalize_sort_star(src);
        assert_eq!(out, "Type Type");
    }
    #[test]
    fn test_strip_universe_decls() {
        let src = "universe u v\ndef foo := 1";
        let out = Lean4UniverseNormalizer::strip_universe_decls(src);
        assert!(!out.contains("universe"));
        assert!(out.contains("def foo"));
    }
    #[test]
    fn test_normalize_all_universe() {
        let src = "universe u\nList.{u} (Sort* -> Type*)";
        let out = Lean4UniverseNormalizer::normalize_all(src);
        assert!(!out.contains("universe"));
        assert!(!out.contains(".{"));
        assert!(!out.contains("Sort*"));
    }
    #[test]
    fn test_extract_leading_docstring_found() {
        let src = "/-- This is a doc. -/ def foo := 1";
        let result = Lean4DocstringExtractor::extract_leading_docstring(src);
        assert!(result.is_some());
        let (doc, rest) = result.expect("test operation should succeed");
        assert_eq!(doc, "This is a doc.");
        assert!(rest.contains("def foo"));
    }
    #[test]
    fn test_extract_leading_docstring_none() {
        let src = "def foo := 1";
        assert!(Lean4DocstringExtractor::extract_leading_docstring(src).is_none());
    }
    #[test]
    fn test_extract_all_docstrings() {
        let src = "/-- First -/ def a := 1\n/-- Second -/ def b := 2";
        let docs = Lean4DocstringExtractor::extract_all_docstrings(src);
        assert_eq!(docs.len(), 2);
        assert_eq!(docs[0].1, "First");
        assert_eq!(docs[1].1, "Second");
    }
    #[test]
    fn test_strip_docstrings() {
        let src = "/-- doc -/ def foo := 1";
        let stripped = Lean4DocstringExtractor::strip_docstrings(src);
        assert!(!stripped.contains("/--"));
        assert!(stripped.contains("def foo"));
    }
    #[test]
    fn test_namespace_tracker_empty() {
        let t = Lean4NamespaceTracker::new();
        assert!(t.is_root());
        assert_eq!(t.current(), "");
        assert_eq!(t.depth(), 0);
    }
    #[test]
    fn test_namespace_tracker_push_pop() {
        let mut t = Lean4NamespaceTracker::new();
        t.push("Nat");
        assert_eq!(t.current(), "Nat");
        t.push("Basic");
        assert_eq!(t.current(), "Nat.Basic");
        assert_eq!(t.depth(), 2);
        t.pop();
        assert_eq!(t.current(), "Nat");
    }
    #[test]
    fn test_namespace_tracker_resolve() {
        let mut t = Lean4NamespaceTracker::new();
        t.push("Nat");
        assert_eq!(t.resolve("add"), "Nat.add");
    }
    #[test]
    fn test_namespace_tracker_resolve_root() {
        let t = Lean4NamespaceTracker::new();
        assert_eq!(t.resolve("foo"), "foo");
    }
    #[test]
    fn test_token_kind_labels_nonempty() {
        let kinds = vec![
            Lean4TokenKind::Ident,
            Lean4TokenKind::Keyword,
            Lean4TokenKind::Arrow,
            Lean4TokenKind::Eof,
        ];
        for k in &kinds {
            assert!(!k.label().is_empty());
        }
    }
    #[test]
    fn test_token_kind_can_start_expr() {
        assert!(Lean4TokenKind::Ident.can_start_expr());
        assert!(Lean4TokenKind::IntLit.can_start_expr());
        assert!(Lean4TokenKind::LParen.can_start_expr());
        assert!(!Lean4TokenKind::Comma.can_start_expr());
        assert!(!Lean4TokenKind::Eof.can_start_expr());
    }
    #[test]
    fn test_token_is_ident() {
        let t = Lean4Token::new(Lean4TokenKind::Ident, "foo", 0, 1, 0);
        assert!(t.is_ident());
        assert!(!t.is_keyword());
        assert!(!t.is_eof());
    }
    #[test]
    fn test_token_is_eof() {
        let t = Lean4Token::new(Lean4TokenKind::Eof, "", 10, 2, 0);
        assert!(t.is_eof());
        assert!(!t.is_ident());
    }
    #[test]
    fn test_compat_report_new() {
        let r = Lean4CompatReport::new("tactic-mode", CompatLevel::Full);
        assert!(r.is_full());
        assert!(!r.has_gaps());
    }
    #[test]
    fn test_compat_report_with_gaps() {
        let r = Lean4CompatReport::new(
            "macro-expansion",
            CompatLevel::Partial("partial".to_string()),
        )
        .with_gap("Hygienic macros not supported.");
        assert!(r.has_gaps());
        assert!(!r.is_full());
    }
    #[test]
    fn test_compat_report_to_markdown_contains_feature() {
        let r = Lean4CompatReport::new("notation", CompatLevel::Full)
            .with_supported("Basic notation works.");
        let md = r.to_markdown();
        assert!(md.contains("notation"));
        assert!(md.contains("Full"));
        assert!(md.contains("Basic notation works."));
    }
    #[test]
    fn test_build_compat_reports_nonempty() {
        let reports = build_compat_reports();
        assert!(!reports.is_empty());
        assert!(reports.iter().any(|r| r.feature == "do-notation"));
    }
    #[test]
    fn test_field_descriptor_format() {
        let f = Lean4FieldDescriptor::new("val", "Nat");
        assert_eq!(f.format(), "val : Nat");
    }
    #[test]
    fn test_field_descriptor_with_default() {
        let f = Lean4FieldDescriptor::new("size", "Nat").with_default("0");
        assert!(f.format().contains(":= 0"));
    }
    #[test]
    fn test_field_descriptor_private() {
        let f = Lean4FieldDescriptor::new("x", "Int").private();
        assert!(matches!(f.visibility, FieldVisibility::Private));
        assert!(f.format().contains("private"));
    }
    #[test]
    fn test_structure_descriptor_format() {
        let s = Lean4StructureDescriptor::new("MyStruct")
            .add_field(Lean4FieldDescriptor::new("x", "Nat"))
            .add_field(Lean4FieldDescriptor::new("y", "Int"));
        let fmt = s.format();
        assert!(fmt.contains("structure MyStruct"));
        assert!(fmt.contains("x : Nat"));
        assert!(fmt.contains("y : Int"));
    }
    #[test]
    fn test_structure_descriptor_as_class() {
        let s = Lean4StructureDescriptor::new("Monoid").as_class();
        assert!(s.is_class);
        assert!(s.format().contains("class"));
    }
    #[test]
    fn test_structure_descriptor_own_field_count() {
        let s = Lean4StructureDescriptor::new("S")
            .add_field(Lean4FieldDescriptor::new("a", "Nat"))
            .add_field(Lean4FieldDescriptor::new("b", "Nat").inherited());
        assert_eq!(s.own_field_count(), 1);
    }
    #[test]
    fn test_constructor_format_no_args() {
        let c = Lean4Constructor::new("zero");
        assert_eq!(c.format(), "| zero");
        assert_eq!(c.arity(), 0);
    }
    #[test]
    fn test_constructor_format_with_args() {
        let c = Lean4Constructor::new("succ").with_arg("Nat");
        assert!(c.format().contains("succ"));
        assert_eq!(c.arity(), 1);
    }
    #[test]
    fn test_inductive_descriptor_format() {
        let ind = Lean4InductiveDescriptor::new("Bool", "Prop")
            .with_constructor(Lean4Constructor::new("true"))
            .with_constructor(Lean4Constructor::new("false"));
        let fmt = ind.format();
        assert!(fmt.contains("inductive Bool"));
        assert!(fmt.contains("| true"));
        assert!(fmt.contains("| false"));
        assert_eq!(ind.constructor_count(), 2);
    }
    #[test]
    fn test_inductive_descriptor_with_params() {
        let ind = Lean4InductiveDescriptor::new("List", "Type").with_param("α", "Type");
        let fmt = ind.format();
        assert!(fmt.contains("(α : Type)"));
    }
    #[test]
    fn test_lean4_option_new() {
        let opt = Lean4Option::new("pp.all", false, "Print all.");
        assert!(!opt.is_enabled());
        assert!(opt.is_default());
    }
    #[test]
    fn test_lean4_option_set() {
        let opt = Lean4Option::new("pp.all", false, "Print all.").set(true);
        assert!(opt.is_enabled());
        assert!(!opt.is_default());
    }
    #[test]
    fn test_lean4_option_format() {
        let opt = Lean4Option::new("pp.unicode", true, "Use Unicode.").set(false);
        assert_eq!(opt.format_set_option(), "set_option pp.unicode false");
    }
    #[test]
    fn test_option_config_defaults() {
        let config = Lean4OptionConfig::defaults();
        assert!(!config.is_empty());
        assert!(config.get("pp.all").is_some());
        assert!(config.get("pp.unicode").is_some());
    }
    #[test]
    fn test_option_config_set_value() {
        let mut config = Lean4OptionConfig::defaults();
        config.set_value("pp.all", true);
        assert!(config.get("pp.all").expect("key should exist").is_enabled());
    }
    #[test]
    fn test_option_config_format_non_defaults() {
        let mut config = Lean4OptionConfig::defaults();
        config.set_value("pp.all", true);
        let out = config.format_non_defaults();
        assert!(out.contains("pp.all"));
    }
    #[test]
    fn test_error_kind_labels() {
        assert_eq!(Lean4ErrorKind::TypeMismatch.label(), "type-mismatch");
        assert_eq!(Lean4ErrorKind::UnknownIdent.label(), "unknown-identifier");
        assert_eq!(Lean4ErrorKind::TacticFailure.label(), "tactic-failure");
    }
    #[test]
    fn test_error_kind_is_recoverable() {
        assert!(Lean4ErrorKind::TacticFailure.is_recoverable());
        assert!(Lean4ErrorKind::UnsupportedFeature.is_recoverable());
        assert!(!Lean4ErrorKind::TypeMismatch.is_recoverable());
        assert!(!Lean4ErrorKind::SyntaxError.is_recoverable());
    }
    #[test]
    fn test_elab_error_format() {
        let err = Lean4ElabError::new(Lean4ErrorKind::TypeMismatch, "Expected Nat, got Int")
            .at(5, 10)
            .with_hint("Cast using Int.toNat.");
        let fmt = err.format();
        assert!(fmt.contains("type-mismatch"));
        assert!(fmt.contains("Expected Nat, got Int"));
        assert!(fmt.contains("5:10"));
        assert!(fmt.contains("Cast using Int.toNat."));
    }
    #[test]
    fn test_elab_error_is_recoverable() {
        let err = Lean4ElabError::new(Lean4ErrorKind::TacticFailure, "simp failed");
        assert!(err.is_recoverable());
    }
    #[test]
    fn test_module_to_path() {
        assert_eq!(
            Lean4ImportResolver::module_to_path("Mathlib.Data.Nat.Basic"),
            "Mathlib/Data/Nat/Basic.lean"
        );
    }
    #[test]
    fn test_parse_imports() {
        let src = "import Mathlib.Data.Nat\nimport Lean.Elab\ndef foo := 1";
        let imports = Lean4ImportResolver::parse_imports(src);
        assert_eq!(imports.len(), 2);
        assert!(imports.contains(&"Mathlib.Data.Nat".to_string()));
    }
    #[test]
    fn test_resolver_resolve() {
        let r = Lean4ImportResolver::new(vec!["/lib"]);
        let path = r.resolve("Foo.Bar");
        assert!(path.is_some());
        assert!(path
            .expect("test operation should succeed")
            .contains("Foo/Bar.lean"));
    }
    #[test]
    fn test_resolver_root_count() {
        let mut r = Lean4ImportResolver::new(vec!["/a", "/b"]);
        assert_eq!(r.root_count(), 2);
        r.add_root("/c");
        assert_eq!(r.root_count(), 3);
    }
    #[test]
    fn test_open_command_full() {
        let cmd = Lean4OpenCommand::full("Nat");
        assert_eq!(cmd.format(), "open Nat");
        assert_eq!(cmd.resolve("add"), Some("Nat.add".to_string()));
    }
    #[test]
    fn test_open_command_partial() {
        let cmd = Lean4OpenCommand::partial("Nat", vec!["add", "mul"]);
        assert_eq!(cmd.resolve("add"), Some("Nat.add".to_string()));
        assert_eq!(cmd.resolve("div"), None);
    }
    #[test]
    fn test_open_command_scoped() {
        let cmd = Lean4OpenCommand::scoped("Mathlib");
        assert!(cmd.scoped);
        assert!(cmd.format().contains("scoped"));
    }
    #[test]
    fn test_syntax_diff_backward_compat() {
        let diff = Lean4SyntaxDiff::new("test change", Lean4SyntaxVersion::v4_0_0(), true);
        assert!(diff.is_backward_compat());
    }
    #[test]
    fn test_known_syntax_diffs_nonempty() {
        let diffs = known_syntax_diffs();
        assert!(!diffs.is_empty());
    }
    #[test]
    fn test_term_rewriter_single_rule() {
        let rw = Lean4TermRewriter::new().add_rule("foo", "bar");
        assert_eq!(rw.rewrite("foo baz foo"), "bar baz bar");
    }
    #[test]
    fn test_term_rewriter_multiple_rules() {
        let rw = Lean4TermRewriter::new()
            .add_rule(" => ", " -> ")
            .add_rule("←", "<-");
        let src = "fun x => x ← y";
        let out = rw.rewrite(src);
        assert!(out.contains("->"));
        assert!(out.contains("<-"));
    }
    #[test]
    fn test_term_rewriter_standard() {
        let rw = Lean4TermRewriter::standard();
        assert!(rw.rule_count() > 0);
        let out = rw.rewrite("fun x => x");
        assert!(out.contains("->"));
    }
    #[test]
    fn test_position_mapper_single_line() {
        let src = "hello world";
        let mapper = Lean4PositionMapper::new(src);
        assert_eq!(mapper.line_count(), 1);
        assert_eq!(mapper.offset_to_line_col(0), (1, 1));
        assert_eq!(mapper.offset_to_line_col(6), (1, 7));
    }
    #[test]
    fn test_position_mapper_multi_line() {
        let src = "line1\nline2\nline3";
        let mapper = Lean4PositionMapper::new(src);
        assert_eq!(mapper.line_count(), 3);
        assert_eq!(mapper.offset_to_line_col(6), (2, 1));
    }
    #[test]
    fn test_position_mapper_roundtrip() {
        let src = "abc\ndefg\nhi";
        let mapper = Lean4PositionMapper::new(src);
        let (line, col) = mapper.offset_to_line_col(5);
        let offset = mapper.line_col_to_offset(line, col);
        assert_eq!(offset, 5);
    }
    #[test]
    fn test_compat_checker_fat_arrow_error() {
        let src = "fun x => x";
        let issues = Lean4CompatChecker::check(src);
        assert!(Lean4CompatChecker::has_errors(&issues));
        let errors = Lean4CompatChecker::filter_by_severity(&issues, IssueSeverity::Error);
        assert!(!errors.is_empty());
    }
    #[test]
    fn test_compat_checker_universe_warning() {
        let src = "List.{u} Nat";
        let issues = Lean4CompatChecker::check(src);
        let warnings = Lean4CompatChecker::filter_by_severity(&issues, IssueSeverity::Warning);
        assert!(!warnings.is_empty());
    }
    #[test]
    fn test_compat_checker_clean_source() {
        let src = "def foo : Nat := 1";
        let issues = Lean4CompatChecker::check(src);
        assert!(!Lean4CompatChecker::has_errors(&issues));
    }
    #[test]
    fn test_compat_issue_format() {
        let issue = CompatIssue::new(3, "test message", IssueSeverity::Warning);
        let fmt = issue.format();
        assert!(fmt.contains("warning"));
        assert!(fmt.contains("3"));
        assert!(fmt.contains("test message"));
    }
    #[test]
    fn test_strip_check_commands() {
        let src = "#check Nat\n#print Nat\ndef foo := 1";
        let out = Lean4SyntaxAdapter::strip_check_commands(src);
        assert!(!out.contains("#check"));
        assert!(!out.contains("#print"));
        assert!(out.contains("def foo"));
    }
    #[test]
    fn test_expand_by_exact() {
        let src = "theorem t : True := by exact trivial";
        let out = Lean4SyntaxAdapter::expand_by_exact(src);
        assert!(out.contains("trivial"));
    }
    #[test]
    fn test_strip_variable_commands() {
        let src = "variable (n : Nat)\ndef foo := n";
        let out = Lean4SyntaxAdapter::strip_variable_commands(src);
        assert!(!out.contains("variable"));
        assert!(out.contains("def foo"));
    }
    #[test]
    fn test_camel_to_snake() {
        assert_eq!(Lean4NameConverter::camel_to_snake("MyFoo"), "my_foo");
        assert_eq!(Lean4NameConverter::camel_to_snake("natAdd"), "nat_add");
        assert_eq!(Lean4NameConverter::camel_to_snake("abc"), "abc");
    }
    #[test]
    fn test_snake_to_camel() {
        assert_eq!(Lean4NameConverter::snake_to_camel("my_foo"), "MyFoo");
        assert_eq!(Lean4NameConverter::snake_to_camel("nat_add"), "NatAdd");
        assert_eq!(Lean4NameConverter::snake_to_camel("abc"), "Abc");
    }
    #[test]
    fn test_strip_namespace() {
        assert_eq!(Lean4NameConverter::strip_namespace("Nat.add"), "add");
        assert_eq!(Lean4NameConverter::strip_namespace("foo"), "foo");
    }
    #[test]
    fn test_namespace_of() {
        assert_eq!(Lean4NameConverter::namespace_of("Nat.add"), "Nat");
        assert_eq!(Lean4NameConverter::namespace_of("foo"), "");
    }
    #[test]
    fn test_same_namespace() {
        assert!(Lean4NameConverter::same_namespace("Nat.add", "Nat.mul"));
        assert!(!Lean4NameConverter::same_namespace("Nat.add", "List.map"));
    }
    #[test]
    fn test_relative_name() {
        assert_eq!(Lean4NameConverter::relative_name("Nat.add", "Nat"), "add");
        assert_eq!(
            Lean4NameConverter::relative_name("Nat.add", "List"),
            "Nat.add"
        );
        assert_eq!(Lean4NameConverter::relative_name("foo", ""), "foo");
    }
    #[test]
    fn test_matrix_count_full() {
        let m = Lean4CompatMatrix::new();
        assert!(m.count_full() > 0);
    }
    #[test]
    fn test_matrix_count_partial() {
        let m = Lean4CompatMatrix::new();
        assert!(m.count_partial() > 0);
    }
    #[test]
    fn test_matrix_count_stub() {
        let m = Lean4CompatMatrix::new();
        assert!(m.count_stub() > 0);
    }
    #[test]
    fn test_matrix_summary_line_contains_counts() {
        let m = Lean4CompatMatrix::new();
        let s = m.summary_line();
        assert!(s.contains("Full:"));
        assert!(s.contains("Partial:"));
        assert!(s.contains("Stub:"));
        assert!(s.contains("Unsupported:"));
    }
    #[test]
    fn test_matrix_set_level() {
        let mut m = Lean4CompatMatrix::new();
        m.set_level(Lean4Feature::MetaProgramming, CompatLevel::Full);
        assert_eq!(
            m.compat_level(&Lean4Feature::MetaProgramming),
            CompatLevel::Full
        );
    }
    #[test]
    fn test_matrix_iter() {
        let m = Lean4CompatMatrix::new();
        let count = m.iter().count();
        assert_eq!(count, Lean4Feature::all().len());
    }
    #[test]
    fn test_feature_descriptions_nonempty() {
        for f in Lean4Feature::all() {
            assert!(!f.description().is_empty(), "description empty for {:?}", f);
        }
    }
    #[test]
    fn test_feature_is_core() {
        assert!(Lean4Feature::TacticMode.is_core());
        assert!(Lean4Feature::PatternMatching.is_core());
        assert!(!Lean4Feature::DoNotation.is_core());
    }
    #[test]
    fn test_feature_affects_surface_syntax() {
        assert!(Lean4Feature::DoNotation.affects_surface_syntax());
        assert!(Lean4Feature::Notation.affects_surface_syntax());
        assert!(!Lean4Feature::TacticMode.affects_surface_syntax());
    }
    #[test]
    fn test_section_manager_empty() {
        let m = Lean4SectionManager::new();
        assert_eq!(m.depth(), 0);
        assert_eq!(m.current_namespace(), "");
        assert!(m.visible_variables().is_empty());
    }
    #[test]
    fn test_section_manager_enter_namespace() {
        let mut m = Lean4SectionManager::new();
        m.enter_namespace("Nat");
        assert_eq!(m.current_namespace(), "Nat");
        assert_eq!(m.depth(), 1);
    }
    #[test]
    fn test_section_manager_enter_section() {
        let mut m = Lean4SectionManager::new();
        m.enter_section("MySection");
        assert_eq!(m.depth(), 1);
        assert_eq!(m.current_namespace(), "");
    }
    #[test]
    fn test_section_manager_exit() {
        let mut m = Lean4SectionManager::new();
        m.enter_namespace("Nat");
        let exited = m.exit();
        assert!(exited.is_some());
        assert_eq!(exited.expect("test operation should succeed").1, "Nat");
        assert_eq!(m.depth(), 0);
    }
    #[test]
    fn test_section_manager_add_variable() {
        let mut m = Lean4SectionManager::new();
        m.enter_section("S");
        m.add_variable("n", "Nat");
        m.add_variable("h", "n > 0");
        let vars = m.visible_variables();
        assert_eq!(vars.len(), 2);
        assert_eq!(vars[0].0, "n");
    }
    #[test]
    fn test_section_manager_nested_namespace() {
        let mut m = Lean4SectionManager::new();
        m.enter_namespace("Data");
        m.enter_namespace("Nat");
        assert_eq!(m.current_namespace(), "Data.Nat");
        m.exit();
        assert_eq!(m.current_namespace(), "Data");
    }
    #[test]
    fn test_scope_kind_keyword() {
        assert_eq!(ScopeKind::Namespace.keyword(), "namespace");
        assert_eq!(ScopeKind::Section.keyword(), "section");
    }
    #[test]
    fn test_field_visibility_as_str() {
        assert_eq!(FieldVisibility::Public.as_str(), "public");
        assert_eq!(FieldVisibility::Protected.as_str(), "protected");
        assert_eq!(FieldVisibility::Private.as_str(), "private");
    }
}
