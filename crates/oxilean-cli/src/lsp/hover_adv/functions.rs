//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lsp::{
    analyze_document, Document, Hover, MarkupContent, MarkupKind, Position, Range, SymbolKind,
};
use oxilean_kernel::{Environment, Name};

use super::types::{
    AdvHoverProvider, DocumentationDb, GoalInfo, HoverBreadcrumb, HoverCache, HoverChain,
    HoverContentBuilder, HoverContext, HoverDocumentationCache, HoverFormatOptions, HoverInfo,
    HoverMarkdownRenderer, HoverPrecisionLevel, HoverRangeExtractor, HoverSpan, HoverStats,
    HoverTypeFormatter, HoveredKind, HypothesisInfo, KeywordDocs, LatexHintDb, StaticHoverProvider,
    StructuredHover, TacticDoc, TacticDocRegistry, TypeDisplayStyle, UnifiedHoverProvider,
};

/// Keyword hover documentation.
pub const KEYWORD_HOVER_DOCS: &[(&str, &str)] = &[
    (
        "def",
        "**def** -- Define a new function or value.\n\n```lean\ndef name : Type := value\n```\n\nUse `def` for non-recursive or structurally recursive definitions.",
    ),
    (
        "theorem",
        "**theorem** -- State and prove a proposition.\n\n```lean\ntheorem name : Prop := proof\ntheorem name : Prop := by tactic\n```\n\nThe body is erased at runtime (proof irrelevance).",
    ),
    (
        "lemma",
        "**lemma** -- Alias for `theorem`.\n\nConventionally used for smaller auxiliary results.",
    ),
    (
        "axiom",
        "**axiom** -- Postulate a type without proof.\n\n```lean\naxiom name : Type\n```\n\nWarning: axioms can introduce inconsistency.",
    ),
    (
        "inductive",
        "**inductive** -- Define an inductive data type.\n\n```lean\ninductive Name where\n  | ctor : ... -> Name\n```",
    ),
    ("structure", "**structure** -- Define a record type with named fields."),
    ("class", "**class** -- Define a type class for ad-hoc polymorphism."),
    ("instance", "**instance** -- Provide a type class instance."),
    (
        "fun",
        "**fun** -- Lambda abstraction (anonymous function).\n\n```lean\nfun x => x + 1\n```",
    ),
    ("forall", "**forall** -- Universal quantification / dependent function type."),
    (
        "match",
        "**match** -- Pattern matching.\n\n```lean\nmatch x with\n| pattern => result\n```",
    ),
    ("let", "**let** -- Local binding.\n\n```lean\nlet x := value\nin body\n```"),
    ("if", "**if** -- Conditional expression.\n\n```lean\nif cond then a else b\n```"),
    ("do", "**do** -- Do-notation for monadic code."),
    (
        "by",
        "**by** -- Enter tactic mode to construct a proof.\n\n```lean\ntheorem p : 1 + 1 = 2 := by rfl\n```",
    ),
    (
        "sorry",
        "**sorry** -- Placeholder for incomplete proofs (axiom).\n\nMarks the proof obligation as admitted (unsound).",
    ),
    ("where", "**where** -- Introduce local definitions after a declaration."),
    ("have", "**have** -- Introduce a local hypothesis."),
    ("show", "**show** -- Annotate the expected type of an expression."),
    ("Prop", "**Prop** -- The type of propositions (`Sort 0`)."),
    ("Type", "**Type** -- The type of types (`Sort 1`)."),
    ("Sort", "**Sort** -- A universe level."),
    ("namespace", "**namespace** -- Open a namespace for declarations."),
    ("section", "**section** -- Begin a section for local variables."),
    ("open", "**open** -- Open a namespace to use its names unqualified."),
    ("import", "**import** -- Import definitions from another module."),
    ("variable", "**variable** -- Declare a section variable."),
    ("end", "**end** -- Close a `namespace` or `section` block."),
];
/// Tactic hover documentation.
pub const TACTIC_HOVER_DOCS: &[(&str, &str)] = &[
    (
        "intro",
        "Introduce one hypothesis from the goal into the context.\n\n```lean\nintro h\n```",
    ),
    ("intros", "Introduce all leading hypotheses."),
    ("apply", "Apply a lemma or hypothesis to the current goal."),
    ("exact", "Close the goal with an exact proof term."),
    ("rfl", "Close the goal by reflexivity (`a = a`)."),
    (
        "rw",
        "Rewrite the goal using an equation.\n\n```lean\nrw [h]\nrw [<- h]  -- rewrite backwards\n```",
    ),
    ("simp", "Simplify using the simp lemma set."),
    ("cases", "Case analysis on an inductive value."),
    ("induction", "Induction on a term."),
    ("constructor", "Apply a constructor to split a goal."),
    ("assumption", "Close the goal if it matches a hypothesis."),
    ("contradiction", "Close the goal by finding a contradiction."),
    ("sorry", "Admit the current goal (unsound)."),
    ("have", "Introduce an intermediate lemma in tactic mode."),
    ("calc", "Begin a calculational proof."),
    ("ring", "Prove equalities in commutative (semi)rings."),
    ("omega", "Solve linear arithmetic goals over Nat/Int."),
    ("linarith", "Linear arithmetic decision procedure."),
    ("norm_num", "Normalize numeric expressions and close numeric goals."),
    ("decide", "Decide a decidable proposition by computation."),
    ("trivial", "Solve trivial goals."),
    ("left", "Choose the left disjunct of an `Or` goal."),
    ("right", "Choose the right disjunct of an `Or` goal."),
];
#[cfg(test)]
mod tests {
    use super::*;
    fn make_doc(content: &str) -> Document {
        Document::new("file:///test.lean", 1, content)
    }
    #[test]
    fn test_hover_content_builder() {
        let mut builder = HoverContentBuilder::new();
        builder.add_signature("def", "foo", "Nat -> Nat");
        builder.add_documentation("A function that does something.");
        let content = builder.build();
        assert!(content.contains("def foo : Nat -> Nat"));
        assert!(content.contains("A function that does something."));
    }
    #[test]
    fn test_hover_content_builder_empty() {
        let builder = HoverContentBuilder::new();
        assert!(builder.is_empty());
    }
    #[test]
    fn test_hover_content_builder_goal_state() {
        let mut builder = HoverContentBuilder::new();
        let goals = vec![GoalInfo {
            hypotheses: vec![HypothesisInfo {
                name: "h".to_string(),
                ty: "P".to_string(),
            }],
            target: "Q".to_string(),
        }];
        builder.add_goal_state(&goals);
        let content = builder.build();
        assert!(content.contains("Goals"));
        assert!(content.contains("h : P"));
    }
    #[test]
    fn test_adv_hover_keyword() {
        let env = Environment::new();
        let provider = AdvHoverProvider::new(&env);
        let doc = make_doc("def foo := 1");
        let hover = provider.hover_at(&doc, &Position::new(0, 1));
        assert!(hover.is_some());
    }
    #[test]
    fn test_adv_hover_literal() {
        let env = Environment::new();
        let provider = AdvHoverProvider::new(&env);
        let doc = make_doc("42");
        let hover = provider.hover_at(&doc, &Position::new(0, 1));
        assert!(hover.is_some());
    }
    #[test]
    fn test_documentation_db() {
        let db = DocumentationDb::new();
        assert!(db.get("Nat.add").is_some());
        assert!(db.get("nonexistent").is_none());
        assert!(!db.is_empty());
    }
    #[test]
    fn test_documentation_db_insert() {
        let mut db = DocumentationDb::new();
        let old_len = db.len();
        db.insert("custom.func", "Custom documentation");
        assert_eq!(db.len(), old_len + 1);
        assert_eq!(db.get("custom.func"), Some("Custom documentation"));
    }
    #[test]
    fn test_latex_hint_db() {
        let db = LatexHintDb::new();
        assert_eq!(db.get("Nat"), Some("\\mathbb{N}"));
        assert_eq!(db.get("And"), Some("P \\land Q"));
        assert!(!db.is_empty());
    }
    #[test]
    fn test_latex_type_to_latex() {
        let db = LatexHintDb::new();
        let result = db.type_to_latex("Nat -> Prop");
        assert!(result.contains("\\to"));
    }
    #[test]
    fn test_hover_cache() {
        let mut cache = HoverCache::new(100);
        assert!(cache.is_empty());
        cache.store("file:///a.lean", 0, 5, 1, None);
        assert_eq!(cache.len(), 1);
        assert!(cache.get("file:///a.lean", 0, 5, 1).is_some());
        assert!(cache.get("file:///a.lean", 0, 5, 2).is_none());
        cache.invalidate("file:///a.lean");
        assert!(cache.is_empty());
    }
    #[test]
    fn test_hover_with_goals() {
        let env = Environment::new();
        let provider = AdvHoverProvider::new(&env);
        let doc = make_doc("sorry");
        let goals = vec![GoalInfo {
            hypotheses: Vec::new(),
            target: "True".to_string(),
        }];
        let hover = provider.hover_with_goals(&doc, &Position::new(0, 2), &goals);
        assert!(hover.is_some());
    }
    #[test]
    fn test_hover_section_ordering() {
        let mut builder = HoverContentBuilder::new();
        builder.add_documentation("Doc comes second");
        builder.add_signature("def", "f", "Nat");
        let content = builder.build();
        let sig_pos = content.find("def f");
        let doc_pos = content.find("Doc comes second");
        assert!(sig_pos < doc_pos);
    }
}
#[cfg(test)]
mod hover_extra_tests {
    use super::*;
    fn make_doc(src: &str) -> Document {
        Document::new("test://test.lean", 0, src)
    }
    #[test]
    fn test_keyword_docs_is_keyword() {
        let docs = KeywordDocs::new();
        assert!(docs.is_keyword("def"));
        assert!(docs.is_keyword("theorem"));
        assert!(!docs.is_keyword("notakeyword"));
    }
    #[test]
    fn test_keyword_docs_get() {
        let docs = KeywordDocs::new();
        assert!(docs.get("axiom").is_some());
        assert!(docs.get("xyz").is_none());
    }
    #[test]
    fn test_keyword_docs_len() {
        let docs = KeywordDocs::new();
        assert!(docs.len() > 10);
    }
    #[test]
    fn test_tactic_doc_to_markdown() {
        let td = TacticDoc::new("intro", "Introduce a hypothesis.").with_example("intro h");
        let md = td.to_markdown();
        assert!(md.contains("intro"));
        assert!(md.contains("```lean"));
    }
    #[test]
    fn test_tactic_doc_registry_get() {
        let reg = TacticDocRegistry::new();
        assert!(reg.get("simp").is_some());
        assert!(reg.get("nonexistent").is_none());
    }
    #[test]
    fn test_tactic_doc_registry_len() {
        let reg = TacticDocRegistry::new();
        assert!(reg.len() > 5);
    }
    #[test]
    fn test_tactic_doc_registry_names() {
        let reg = TacticDocRegistry::new();
        let names = reg.tactic_names();
        assert!(names.contains(&"intro"));
        assert!(names.contains(&"apply"));
        assert!(names.contains(&"simp"));
    }
    #[test]
    fn test_hover_span_is_point() {
        let s = HoverSpan::new("file:///a.lean", 3, 5, 3, 5);
        assert!(s.is_point());
        let s2 = HoverSpan::new("file:///a.lean", 3, 5, 3, 10);
        assert!(!s2.is_point());
    }
    #[test]
    fn test_hover_span_line_span() {
        let s = HoverSpan::new("file:///a.lean", 2, 0, 5, 10);
        assert_eq!(s.line_span(), 3);
    }
    #[test]
    fn test_hovered_kind_display() {
        assert_eq!(format!("{}", HoveredKind::Tactic), "tactic");
        assert_eq!(format!("{}", HoveredKind::Keyword), "keyword");
        assert_eq!(format!("{}", HoveredKind::Declaration), "declaration");
    }
    #[test]
    fn test_structured_hover_to_hover() {
        let sh = StructuredHover::new("Hello **world**", HoveredKind::Keyword);
        let h = sh.to_hover();
        assert!(h.contents.value.contains("Hello"));
    }
    #[test]
    fn test_unified_hover_provider_keyword() {
        let env = Environment::new();
        let provider = UnifiedHoverProvider::new(&env);
        let h = provider.keyword_hover("def");
        assert!(h.is_some());
        assert!(h
            .expect("test operation should succeed")
            .contents
            .value
            .contains("def"));
    }
    #[test]
    fn test_unified_hover_provider_tactic() {
        let env = Environment::new();
        let provider = UnifiedHoverProvider::new(&env);
        let h = provider.tactic_hover("simp");
        assert!(h.is_some());
    }
    #[test]
    fn test_unified_hover_provider_unknown_keyword() {
        let env = Environment::new();
        let provider = UnifiedHoverProvider::new(&env);
        let h = provider.keyword_hover("notakeyword");
        assert!(h.is_none());
    }
    #[test]
    fn test_unified_hover_invalidate() {
        let env = Environment::new();
        let mut provider = UnifiedHoverProvider::new(&env);
        let doc = make_doc("def foo := 1");
        let _ = provider.hover(&doc, &Position::new(0, 1));
        provider.invalidate("test://test.lean");
        assert!(provider.cache.is_empty());
    }
    #[test]
    fn test_structured_hover_with_span() {
        let span = HoverSpan::new("file:///a.lean", 1, 0, 1, 5);
        let sh = StructuredHover::new("content", HoveredKind::Declaration).with_span(span.clone());
        assert_eq!(
            sh.span
                .as_ref()
                .expect("type conversion should succeed")
                .start_line,
            1
        );
    }
}
/// Trait for hover information providers.
#[allow(dead_code)]
pub trait HoverInfoProvider: Send + Sync {
    /// Return hover info for a name.
    fn hover_for(&self, name: &str) -> Option<HoverInfo>;
}
/// Return the hover_adv module version.
#[allow(dead_code)]
pub fn hover_adv_version() -> &'static str {
    "0.1.1"
}
#[cfg(test)]
mod hover_adv_extra_tests {
    use super::*;
    #[test]
    fn test_hover_doc_cache() {
        let mut cache = HoverDocumentationCache::new(10);
        cache.store(
            "foo".to_string(),
            "Nat -> Nat".to_string(),
            "A function.".to_string(),
        );
        let entry = cache.get("foo");
        assert!(entry.is_some());
        assert_eq!(
            entry.expect("test operation should succeed").type_str,
            "Nat -> Nat"
        );
    }
    #[test]
    fn test_hover_markdown_renderer() {
        let renderer = HoverMarkdownRenderer::new();
        let md = renderer.render_entry(
            "foo",
            "Nat -> Nat",
            Some("Increments x."),
            Some("def foo : Nat -> Nat := fun x -> x + 1"),
            &["foo 3  -- 4"],
        );
        assert!(md.contains("```lean"));
        assert!(md.contains("foo"));
        assert!(md.contains("Increments x."));
        assert!(md.contains("**Source:**"));
        assert!(md.contains("**Example:**"));
    }
    #[test]
    fn test_hover_markdown_renderer_no_source() {
        let mut renderer = HoverMarkdownRenderer::new();
        renderer.include_source = false;
        let md = renderer.render_source("def foo := 1");
        assert!(md.is_empty());
    }
    #[test]
    fn test_hover_info_builder() {
        let info = HoverInfo::new("Nat.add", "Nat -> Nat -> Nat")
            .with_documentation("Natural number addition")
            .with_definition("file:///Nat.lean", 42)
            .with_tag("builtin");
        assert_eq!(info.name, "Nat.add");
        assert!(info.documentation.is_some());
        assert_eq!(info.definition_line, Some(42));
        assert!(info.tags.contains(&"builtin".to_string()));
    }
    #[test]
    fn test_hover_range_extractor() {
        let line = "theorem foo_bar : Nat := 0";
        let result = HoverRangeExtractor::extract_word_at(line, 8);
        assert!(result.is_some());
        let (start, _end, word) = result.expect("test operation should succeed");
        assert_eq!(word, "foo_bar");
        assert_eq!(start, 8);
    }
    #[test]
    fn test_hover_range_extractor_not_on_ident() {
        let line = "theorem foo : Nat";
        let result = HoverRangeExtractor::extract_word_at(line, 7);
        assert!(result.is_none());
    }
    #[test]
    fn test_hover_range_extractor_qualified() {
        let line = "def x := Nat.add 1 2";
        let result = HoverRangeExtractor::extract_qualified_at(line, 9);
        assert!(result.is_some());
        let (_, _, word) = result.expect("test operation should succeed");
        assert_eq!(word, "Nat.add");
    }
    #[test]
    fn test_hover_type_formatter() {
        let formatter = HoverTypeFormatter::new();
        let short = formatter.format("Nat -> Nat");
        assert_eq!(short, "Nat -> Nat");
        let long_type =
            "A -> B -> C -> D -> E -> F -> G -> H -> I -> J -> K -> L -> M -> N -> O -> P";
        let formatted = formatter.format(long_type);
        assert!(formatted.len() >= long_type.len() || formatted.contains('\n'));
    }
    #[test]
    fn test_hover_stats() {
        let mut stats = HoverStats::default();
        stats.record_hit();
        stats.record_miss(500);
        assert_eq!(stats.total_requests, 2);
        assert!((stats.hit_rate() - 50.0).abs() < 1.0);
        assert!((stats.avg_latency_us - 500.0).abs() < 1.0);
    }
    #[test]
    fn test_hover_adv_version() {
        assert!(!hover_adv_version().is_empty());
    }
}
/// Render hover info at a given precision level.
#[allow(dead_code)]
pub fn render_hover_at_precision(info: &HoverInfo, level: HoverPrecisionLevel) -> String {
    let renderer = HoverMarkdownRenderer::new();
    match level {
        HoverPrecisionLevel::Minimal => info.name.clone(),
        HoverPrecisionLevel::TypeOnly => renderer.render_type(&info.name, &info.type_str),
        HoverPrecisionLevel::Full => renderer.render_entry(
            &info.name,
            &info.type_str,
            info.documentation.as_deref(),
            None,
            &[],
        ),
        HoverPrecisionLevel::Verbose => renderer.render_entry(
            &info.name,
            &info.type_str,
            info.documentation.as_deref(),
            None,
            &[],
        ),
    }
}
/// Return list of features in hover_adv module.
#[allow(dead_code)]
pub fn hover_adv_features() -> Vec<&'static str> {
    vec![
        "cache",
        "markdown-renderer",
        "provider-trait",
        "range-extractor",
        "type-formatter",
        "stats",
        "chain",
        "static-provider",
        "precision-levels",
    ]
}
#[cfg(test)]
mod hover_chain_tests {
    use super::*;
    #[test]
    fn test_static_hover_provider() {
        let provider = StaticHoverProvider::builtin();
        let info = provider.hover_for("Nat");
        assert!(info.is_some());
        let info = info.expect("test operation should succeed");
        assert_eq!(info.type_str, "Type");
        assert!(info.documentation.is_some());
    }
    #[test]
    fn test_static_hover_provider_miss() {
        let provider = StaticHoverProvider::builtin();
        assert!(provider.hover_for("MyCustomType").is_none());
    }
    #[test]
    fn test_hover_chain() {
        let mut chain = HoverChain::new();
        chain.add(Box::new(StaticHoverProvider::builtin()));
        let info = chain.hover_for("Bool");
        assert!(info.is_some());
        let missing = chain.hover_for("MissingSymbol");
        assert!(missing.is_none());
    }
    #[test]
    fn test_render_at_precision() {
        let info = HoverInfo::new("foo", "Nat -> Nat").with_documentation("Does stuff");
        let minimal = render_hover_at_precision(&info, HoverPrecisionLevel::Minimal);
        assert_eq!(minimal, "foo");
        let type_only = render_hover_at_precision(&info, HoverPrecisionLevel::TypeOnly);
        assert!(type_only.contains("foo"));
        assert!(type_only.contains("Nat -> Nat"));
        let full = render_hover_at_precision(&info, HoverPrecisionLevel::Full);
        assert!(full.contains("Does stuff"));
    }
    #[test]
    fn test_hover_context() {
        let ctx = HoverContext::new("file:///a.lean", 5, 10);
        assert_eq!(ctx.line, 5);
        assert_eq!(ctx.character, 10);
    }
    #[test]
    fn test_hover_adv_features() {
        let features = hover_adv_features();
        assert!(features.contains(&"cache"));
        assert!(features.contains(&"chain"));
    }
}
/// Render a breadcrumb path as markdown.
#[allow(dead_code)]
pub fn render_breadcrumbs(breadcrumbs: &[HoverBreadcrumb]) -> String {
    breadcrumbs
        .iter()
        .map(|b| b.to_markdown())
        .collect::<Vec<_>>()
        .join(" > ")
}
/// Return the total features in hover_adv.
#[allow(dead_code)]
pub fn hover_adv_feature_count() -> usize {
    hover_adv_features().len()
}
#[cfg(test)]
mod breadcrumb_tests {
    use super::*;
    #[test]
    fn test_hover_breadcrumb_with_target() {
        let crumb = HoverBreadcrumb::with_target("Nat.add", "file:///Nat.lean", 42);
        let md = crumb.to_markdown();
        assert!(md.contains("Nat.add"));
        assert!(md.contains("#L42"));
    }
    #[test]
    fn test_hover_breadcrumb_plain() {
        let crumb = HoverBreadcrumb::plain("Mathlib");
        let md = crumb.to_markdown();
        assert_eq!(md, "Mathlib");
    }
    #[test]
    fn test_render_breadcrumbs() {
        let crumbs = vec![
            HoverBreadcrumb::plain("Mathlib"),
            HoverBreadcrumb::plain("Algebra"),
            HoverBreadcrumb::with_target("Group", "file:///Group.lean", 1),
        ];
        let rendered = render_breadcrumbs(&crumbs);
        assert!(rendered.contains(" > "));
        assert!(rendered.contains("Mathlib"));
        assert!(rendered.contains("Group"));
    }
    #[test]
    fn test_hover_adv_feature_count() {
        assert!(hover_adv_feature_count() > 0);
    }
}
#[cfg(test)]
mod format_options_tests {
    use super::*;
    #[test]
    fn test_hover_format_options_default() {
        let opts = HoverFormatOptions::default();
        assert!(opts.show_types);
        assert!(opts.show_docs);
        assert_eq!(opts.type_display_style, TypeDisplayStyle::Full);
    }
    #[test]
    fn test_hover_format_options_verbose() {
        let opts = HoverFormatOptions::verbose();
        assert!(opts.show_examples);
        assert_eq!(opts.max_doc_lines, 50);
    }
    #[test]
    fn test_hover_format_options_minimal() {
        let opts = HoverFormatOptions::minimal();
        assert!(!opts.show_docs);
        assert_eq!(opts.type_display_style, TypeDisplayStyle::Abbreviated);
    }
}
