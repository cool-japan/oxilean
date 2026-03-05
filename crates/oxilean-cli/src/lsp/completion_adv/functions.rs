//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lsp::{
    analyze_document, CompletionItem, CompletionItemKind, Document, MarkupContent, Position,
};
use oxilean_kernel::Environment;
use std::collections::HashSet;

use super::types::{
    AdvCompletionStats, AdvancedCompletionItem, CompletionCache, CompletionFormatter,
    CompletionItemTag, CompletionList, CompletionPipeline, CompletionRegistry,
    CompletionResolveRequest, CompletionResult, CompletionTrigger, ContextAnalyzer,
    ContextCategory, ImportCompleter, ImportCompletionProvider, KeywordCompletionProvider,
    LimitStep, LspCompletionContext, ModuleInfo, RemoveDeprecatedStep, SnippetCompletionProvider,
    SnippetContext, SnippetDefinition, SnippetEntry, SnippetProvider, TacticCategory,
    TacticCompleter, TacticEntry, TypeFilter, UnifiedCompletionEngine,
};

/// Top-level keywords.
pub const TOP_LEVEL_KEYWORDS: &[&str] = &[
    "def",
    "theorem",
    "lemma",
    "axiom",
    "inductive",
    "structure",
    "class",
    "instance",
    "namespace",
    "section",
    "end",
    "variable",
    "open",
    "import",
    "export",
    "set_option",
    "attribute",
    "#check",
    "#eval",
    "#print",
];
pub fn is_ident_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_' || b == b'.' || b == b'\''
}
/// Standard tactic entries.
pub fn standard_tactics() -> Vec<TacticEntry> {
    vec![
        TacticEntry { name : "intro", description : "Introduce a hypothesis", snippet :
        Some("intro ${1:h}"), category : TacticCategory::IntroElim, common : true },
        TacticEntry { name : "intros", description : "Introduce all hypotheses", snippet
        : None, category : TacticCategory::IntroElim, common : true }, TacticEntry { name
        : "apply", description : "Apply a lemma to the goal", snippet :
        Some("apply ${1:lemma}"), category : TacticCategory::IntroElim, common : true },
        TacticEntry { name : "exact", description : "Provide an exact proof term",
        snippet : Some("exact ${1:term}"), category : TacticCategory::Closing, common :
        true }, TacticEntry { name : "rfl", description : "Close by reflexivity", snippet
        : None, category : TacticCategory::Closing, common : true }, TacticEntry { name :
        "rw", description : "Rewrite using an equation", snippet : Some("rw [${1:h}]"),
        category : TacticCategory::Rewrite, common : true }, TacticEntry { name : "simp",
        description : "Simplify using simp lemmas", snippet : None, category :
        TacticCategory::Simplification, common : true }, TacticEntry { name :
        "simp only", description : "Simplify with specific lemmas", snippet :
        Some("simp only [${1:lemma}]"), category : TacticCategory::Simplification, common
        : true }, TacticEntry { name : "cases", description : "Case analysis on a term",
        snippet : Some("cases ${1:h}"), category : TacticCategory::CaseAnalysis, common :
        true }, TacticEntry { name : "induction", description : "Induction on a term",
        snippet :
        Some("induction ${1:n} with\n| ${2:base} => ${3:sorry}\n| ${4:step} ${5:ih} => ${0:sorry}"),
        category : TacticCategory::CaseAnalysis, common : true }, TacticEntry { name :
        "constructor", description : "Apply a constructor", snippet : None, category :
        TacticCategory::IntroElim, common : true }, TacticEntry { name : "assumption",
        description : "Close by assumption", snippet : None, category :
        TacticCategory::Closing, common : true }, TacticEntry { name : "contradiction",
        description : "Close by contradiction", snippet : None, category :
        TacticCategory::Closing, common : false }, TacticEntry { name : "have",
        description : "Introduce an intermediate lemma", snippet :
        Some("have ${1:h} : ${2:Type} := by\n  ${0:sorry}"), category :
        TacticCategory::Structural, common : true }, TacticEntry { name : "let",
        description : "Introduce a local definition", snippet :
        Some("let ${1:x} := ${2:value}"), category : TacticCategory::Structural, common :
        false }, TacticEntry { name : "show", description : "Annotate the goal type",
        snippet : Some("show ${1:Type}"), category : TacticCategory::Structural, common :
        false }, TacticEntry { name : "calc", description : "Calculational proof",
        snippet : Some("calc ${1:lhs}\n    _ = ${2:rhs} := by ${0:sorry}"), category :
        TacticCategory::Structural, common : false }, TacticEntry { name : "ring",
        description : "Prove ring equalities", snippet : None, category :
        TacticCategory::Automation, common : true }, TacticEntry { name : "omega",
        description : "Linear arithmetic over Nat/Int", snippet : None, category :
        TacticCategory::Automation, common : true }, TacticEntry { name : "linarith",
        description : "Linear arithmetic", snippet : None, category :
        TacticCategory::Automation, common : false }, TacticEntry { name : "norm_num",
        description : "Normalize numeric expressions", snippet : None, category :
        TacticCategory::Automation, common : true }, TacticEntry { name : "decide",
        description : "Decide a decidable prop", snippet : None, category :
        TacticCategory::Automation, common : false }, TacticEntry { name : "trivial",
        description : "Solve trivial goals", snippet : None, category :
        TacticCategory::Closing, common : false }, TacticEntry { name : "left",
        description : "Choose left disjunct", snippet : None, category :
        TacticCategory::IntroElim, common : true }, TacticEntry { name : "right",
        description : "Choose right disjunct", snippet : None, category :
        TacticCategory::IntroElim, common : true }, TacticEntry { name : "obtain",
        description : "Destructure a hypothesis", snippet :
        Some("obtain \\<${1:h1}, ${2:h2}\\> := ${0:h}"), category :
        TacticCategory::IntroElim, common : false }, TacticEntry { name : "ext",
        description : "Apply extensionality", snippet : None, category :
        TacticCategory::Advanced, common : false }, TacticEntry { name : "funext",
        description : "Function extensionality", snippet : Some("funext ${1:x}"),
        category : TacticCategory::Advanced, common : false }, TacticEntry { name :
        "congr", description : "Apply congruence", snippet : None, category :
        TacticCategory::Advanced, common : false }, TacticEntry { name : "specialize",
        description : "Specialize a hypothesis", snippet :
        Some("specialize ${1:h} ${2:arg}"), category : TacticCategory::Structural, common
        : false }, TacticEntry { name : "revert", description :
        "Move hypothesis to goal", snippet : Some("revert ${1:h}"), category :
        TacticCategory::Structural, common : false }, TacticEntry { name : "clear",
        description : "Remove a hypothesis", snippet : Some("clear ${1:h}"), category :
        TacticCategory::Structural, common : false }, TacticEntry { name : "sorry",
        description : "Admit goal (unsound)", snippet : None, category :
        TacticCategory::Closing, common : true }, TacticEntry { name : "rcases",
        description : "Recursive case split", snippet :
        Some("rcases ${1:h} with ${0:pattern}"), category : TacticCategory::CaseAnalysis,
        common : false }, TacticEntry { name : "use", description :
        "Provide existential witness", snippet : Some("use ${1:witness}"), category :
        TacticCategory::IntroElim, common : true }, TacticEntry { name : "exists",
        description : "Provide existential witness (alias)", snippet :
        Some("exists ${1:witness}"), category : TacticCategory::IntroElim, common : false
        }, TacticEntry { name : "split", description : "Split a conjunction goal",
        snippet : None, category : TacticCategory::IntroElim, common : true },
        TacticEntry { name : "exfalso", description : "Reduce goal to False", snippet :
        None, category : TacticCategory::Closing, common : false }, TacticEntry { name :
        "by_contra", description : "Proof by contradiction", snippet :
        Some("by_contra ${1:h}"), category : TacticCategory::Advanced, common : false },
        TacticEntry { name : "push_neg", description : "Push negations inward", snippet :
        None, category : TacticCategory::Rewrite, common : false }, TacticEntry { name :
        "norm_cast", description : "Normalize casts", snippet : None, category :
        TacticCategory::Simplification, common : false }, TacticEntry { name :
        "field_simp", description : "Simplify field expressions", snippet : None,
        category : TacticCategory::Simplification, common : false }, TacticEntry { name :
        "nlinarith", description : "Nonlinear arithmetic", snippet : None, category :
        TacticCategory::Automation, common : false }, TacticEntry { name : "positivity",
        description : "Prove positivity goals", snippet : None, category :
        TacticCategory::Automation, common : false }, TacticEntry { name : "gcongr",
        description : "Generalized congruence", snippet : None, category :
        TacticCategory::Advanced, common : false },
    ]
}
/// Standard module entries.
pub fn standard_modules() -> Vec<ModuleInfo> {
    vec![
        ModuleInfo {
            path: "Init".to_string(),
            description: "Core initialization module".to_string(),
            is_std: true,
        },
        ModuleInfo {
            path: "Init.Prelude".to_string(),
            description: "Basic prelude types".to_string(),
            is_std: true,
        },
        ModuleInfo {
            path: "Init.Core".to_string(),
            description: "Core type definitions".to_string(),
            is_std: true,
        },
        ModuleInfo {
            path: "Init.Data".to_string(),
            description: "Data structure definitions".to_string(),
            is_std: true,
        },
        ModuleInfo {
            path: "Init.Data.Nat".to_string(),
            description: "Natural number operations".to_string(),
            is_std: true,
        },
        ModuleInfo {
            path: "Init.Data.Int".to_string(),
            description: "Integer operations".to_string(),
            is_std: true,
        },
        ModuleInfo {
            path: "Init.Data.List".to_string(),
            description: "List operations".to_string(),
            is_std: true,
        },
        ModuleInfo {
            path: "Init.Data.Array".to_string(),
            description: "Array operations".to_string(),
            is_std: true,
        },
        ModuleInfo {
            path: "Init.Data.String".to_string(),
            description: "String operations".to_string(),
            is_std: true,
        },
        ModuleInfo {
            path: "Init.Tactics".to_string(),
            description: "Core tactics".to_string(),
            is_std: true,
        },
        ModuleInfo {
            path: "Std".to_string(),
            description: "Standard library".to_string(),
            is_std: true,
        },
        ModuleInfo {
            path: "Std.Data".to_string(),
            description: "Standard data structures".to_string(),
            is_std: true,
        },
        ModuleInfo {
            path: "Std.Data.HashMap".to_string(),
            description: "Hash map".to_string(),
            is_std: true,
        },
        ModuleInfo {
            path: "Std.Data.HashSet".to_string(),
            description: "Hash set".to_string(),
            is_std: true,
        },
        ModuleInfo {
            path: "Std.Data.RBMap".to_string(),
            description: "Red-black tree map".to_string(),
            is_std: true,
        },
        ModuleInfo {
            path: "Std.Logic".to_string(),
            description: "Logic lemmas".to_string(),
            is_std: true,
        },
        ModuleInfo {
            path: "Std.Tactic".to_string(),
            description: "Extended tactics".to_string(),
            is_std: true,
        },
        ModuleInfo {
            path: "Mathlib".to_string(),
            description: "Mathlib library".to_string(),
            is_std: false,
        },
        ModuleInfo {
            path: "Mathlib.Tactic".to_string(),
            description: "Mathlib tactics".to_string(),
            is_std: false,
        },
        ModuleInfo {
            path: "Mathlib.Data.Nat.Basic".to_string(),
            description: "Nat lemmas".to_string(),
            is_std: false,
        },
        ModuleInfo {
            path: "Mathlib.Data.Int.Basic".to_string(),
            description: "Int lemmas".to_string(),
            is_std: false,
        },
        ModuleInfo {
            path: "Mathlib.Data.List.Basic".to_string(),
            description: "List lemmas".to_string(),
            is_std: false,
        },
        ModuleInfo {
            path: "Mathlib.Algebra.Group.Basic".to_string(),
            description: "Group theory basics".to_string(),
            is_std: false,
        },
        ModuleInfo {
            path: "Mathlib.Algebra.Ring.Basic".to_string(),
            description: "Ring theory basics".to_string(),
            is_std: false,
        },
        ModuleInfo {
            path: "Mathlib.Order.Basic".to_string(),
            description: "Order theory basics".to_string(),
            is_std: false,
        },
        ModuleInfo {
            path: "Mathlib.Topology.Basic".to_string(),
            description: "Topology basics".to_string(),
            is_std: false,
        },
        ModuleInfo {
            path: "Mathlib.Analysis.Basic".to_string(),
            description: "Analysis basics".to_string(),
            is_std: false,
        },
    ]
}
/// Standard snippet entries.
pub fn standard_snippets() -> Vec<SnippetEntry> {
    vec![
        SnippetEntry { label : "def", template :
        "def ${1:name} : ${2:Type} := ${0:sorry}", description : "Definition", context :
        SnippetContext::TopLevel }, SnippetEntry { label : "theorem", template :
        "theorem ${1:name} : ${2:Prop} := by\n  ${0:sorry}", description :
        "Theorem with tactic proof", context : SnippetContext::TopLevel }, SnippetEntry {
        label : "lemma", template : "lemma ${1:name} : ${2:Prop} := by\n  ${0:sorry}",
        description : "Lemma with tactic proof", context : SnippetContext::TopLevel },
        SnippetEntry { label : "inductive", template :
        "inductive ${1:Name} where\n  | ${2:ctor} : ${0:Name}", description :
        "Inductive type", context : SnippetContext::TopLevel }, SnippetEntry { label :
        "structure", template : "structure ${1:Name} where\n  ${2:field} : ${0:Type}",
        description : "Structure/record type", context : SnippetContext::TopLevel },
        SnippetEntry { label : "class", template :
        "class ${1:Name} (${2:a} : Type) where\n  ${3:method} : ${0:Type}", description :
        "Type class", context : SnippetContext::TopLevel }, SnippetEntry { label :
        "instance", template :
        "instance : ${1:Class} ${2:Type} where\n  ${3:method} := ${0:sorry}", description
        : "Type class instance", context : SnippetContext::TopLevel }, SnippetEntry {
        label : "namespace", template : "namespace ${1:Name}\n\n${0}\n\nend ${1:Name}",
        description : "Namespace block", context : SnippetContext::TopLevel },
        SnippetEntry { label : "section", template :
        "section ${1:Name}\n\n${0}\n\nend ${1:Name}", description : "Section block",
        context : SnippetContext::TopLevel }, SnippetEntry { label : "match", template :
        "match ${1:x} with\n| ${2:pattern} => ${0:sorry}", description : "Pattern match",
        context : SnippetContext::Expression }, SnippetEntry { label : "if", template :
        "if ${1:cond} then ${2:a} else ${0:b}", description : "Conditional", context :
        SnippetContext::Expression }, SnippetEntry { label : "fun", template :
        "fun ${1:x} => ${0:body}", description : "Lambda function", context :
        SnippetContext::Expression }, SnippetEntry { label : "let", template :
        "let ${1:x} := ${2:value}\n${0}", description : "Local binding", context :
        SnippetContext::Expression }, SnippetEntry { label : "do", template :
        "do\n  ${0}", description : "Do-notation block", context :
        SnippetContext::Expression }, SnippetEntry { label : "have", template :
        "have ${1:h} : ${2:Prop} := by\n  ${0:sorry}", description : "Have expression",
        context : SnippetContext::Tactic }, SnippetEntry { label : "calc", template :
        "calc ${1:lhs}\n    _ = ${2:rhs} := by ${0:sorry}", description :
        "Calculational proof", context : SnippetContext::Tactic }, SnippetEntry { label :
        "induction with", template :
        "induction ${1:n} with\n| ${2:zero} => ${3:sorry}\n| ${4:succ} ${5:ih} => ${0:sorry}",
        description : "Induction with cases", context : SnippetContext::Tactic },
        SnippetEntry { label : "cases with", template :
        "cases ${1:h} with\n| ${2:case1} => ${3:sorry}\n| ${4:case2} => ${0:sorry}",
        description : "Cases with branches", context : SnippetContext::Tactic },
    ]
}
/// Get keyword completions matching a prefix.
pub fn keyword_completions(prefix: &str) -> Vec<CompletionItem> {
    let keywords = [
        "def",
        "theorem",
        "lemma",
        "axiom",
        "inductive",
        "structure",
        "class",
        "instance",
        "namespace",
        "section",
        "end",
        "variable",
        "open",
        "import",
        "export",
        "where",
        "let",
        "in",
        "fun",
        "forall",
        "match",
        "with",
        "if",
        "then",
        "else",
        "do",
        "have",
        "show",
        "by",
        "Prop",
        "Type",
        "Sort",
        "sorry",
    ];
    let mut items = Vec::new();
    for kw in &keywords {
        if prefix.is_empty() || kw.starts_with(prefix) {
            items.push(CompletionItem::keyword(kw));
        }
    }
    items
}
#[cfg(test)]
mod tests {
    use super::*;
    fn make_doc(content: &str) -> Document {
        Document::new("file:///test.lean", 1, content)
    }
    #[test]
    fn test_context_analyzer_top_level() {
        let env = Environment::new();
        let doc = make_doc("def ");
        let analyzer = ContextAnalyzer::new(&doc, &env);
        let ctx = analyzer.analyze(&Position::new(0, 4));
        assert_eq!(ctx.category, ContextCategory::TopLevel);
    }
    #[test]
    fn test_context_analyzer_import() {
        let env = Environment::new();
        let doc = make_doc("import ");
        let analyzer = ContextAnalyzer::new(&doc, &env);
        let ctx = analyzer.analyze(&Position::new(0, 7));
        assert_eq!(ctx.category, ContextCategory::Import);
    }
    #[test]
    fn test_tactic_completer() {
        let completer = TacticCompleter::new();
        let items = completer.complete("intr");
        assert!(!items.is_empty());
        assert!(items.iter().any(|i| i.label == "intro"));
    }
    #[test]
    fn test_tactic_completer_empty_prefix() {
        let completer = TacticCompleter::new();
        let items = completer.complete("");
        assert!(items.len() > 10);
    }
    #[test]
    fn test_import_completer() {
        let completer = ImportCompleter::new();
        let items = completer.complete("Init");
        assert!(!items.is_empty());
        assert!(items.iter().any(|i| i.label.starts_with("Init")));
    }
    #[test]
    fn test_import_completer_submodule() {
        let completer = ImportCompleter::new();
        let items = completer.complete_submodule("Init");
        assert!(!items.is_empty());
    }
    #[test]
    fn test_snippet_provider_top_level() {
        let provider = SnippetProvider::new();
        let items = provider.complete("def", &ContextCategory::TopLevel);
        assert!(!items.is_empty());
    }
    #[test]
    fn test_snippet_provider_tactic() {
        let provider = SnippetProvider::new();
        let items = provider.complete("", &ContextCategory::Tactic);
        assert!(!items.is_empty());
    }
    #[test]
    fn test_type_filter() {
        let env = Environment::new();
        let filter = TypeFilter::new(&env);
        assert!(filter.type_matches("Nat", "Nat"));
        assert!(filter.type_matches("Nat -> Nat", "Nat"));
        assert!(!filter.type_matches("String", "Nat"));
    }
    #[test]
    fn test_unified_engine_top_level() {
        let env = Environment::new();
        let engine = UnifiedCompletionEngine::new(&env);
        let doc = make_doc("");
        let result = engine.complete_at(&doc, &Position::new(0, 0));
        assert!(!result.items.is_empty());
    }
    #[test]
    fn test_unified_engine_tactic() {
        let env = Environment::new();
        let engine = UnifiedCompletionEngine::new(&env);
        let doc = make_doc("theorem p : True := by\n  ");
        let _result = engine.complete_at(&doc, &Position::new(1, 2));
    }
    #[test]
    fn test_completion_result() {
        let result = CompletionResult {
            items: Vec::new(),
            is_incomplete: false,
            context: ContextCategory::TopLevel,
        };
        assert!(!result.is_incomplete);
        assert!(result.items.is_empty());
    }
    #[test]
    fn test_context_category_type_annotation() {
        let env = Environment::new();
        let doc = make_doc("def x : ");
        let analyzer = ContextAnalyzer::new(&doc, &env);
        let ctx = analyzer.analyze(&Position::new(0, 8));
        assert_eq!(ctx.category, ContextCategory::TypeAnnotation);
    }
    #[test]
    fn test_attribute_completions() {
        let env = Environment::new();
        let engine = UnifiedCompletionEngine::new(&env);
        let items = engine.attribute_completions("sim");
        assert!(items.iter().any(|i| i.label == "simp"));
    }
}
/// Trait for providing completions in a specific context.
#[allow(dead_code)]
pub trait CompletionProvider: Send + Sync {
    /// Return the context types this provider handles.
    fn triggers(&self) -> &[char];
    /// Provide completions for a document at a position.
    fn completions(
        &self,
        uri: &str,
        line: u32,
        character: u32,
        context: &LspCompletionContext,
    ) -> CompletionList;
    /// Resolve a specific completion item.
    fn resolve(&self, request: &CompletionResolveRequest) -> AdvancedCompletionItem {
        AdvancedCompletionItem::new(request.item_label.clone())
    }
}
/// Return the completion_adv module version.
#[allow(dead_code)]
pub fn completion_adv_version() -> &'static str {
    "0.1.0"
}
#[cfg(test)]
mod adv_completion_tests {
    use super::*;
    #[test]
    fn test_completion_trigger() {
        let ctx = LspCompletionContext::triggered_by('.');
        assert_eq!(ctx.trigger_character, Some('.'));
        assert_eq!(ctx.trigger_kind, CompletionTrigger::Character('.'));
    }
    #[test]
    fn test_advanced_completion_item() {
        let item = AdvancedCompletionItem::new("theorem")
            .with_kind(14)
            .with_detail("Lean keyword")
            .preselected();
        assert_eq!(item.label, "theorem");
        assert_eq!(item.kind, 14);
        assert!(item.preselect);
    }
    #[test]
    fn test_completion_list_sort() {
        let mut list = CompletionList::complete(vec![
            AdvancedCompletionItem::new("zebra"),
            AdvancedCompletionItem::new("apple"),
            AdvancedCompletionItem::new("mango"),
        ]);
        list.sort();
        assert_eq!(list.items[0].label, "apple");
        assert_eq!(list.items[2].label, "zebra");
    }
    #[test]
    fn test_completion_list_filter() {
        let list = CompletionList::complete(vec![
            AdvancedCompletionItem::new("theorem"),
            AdvancedCompletionItem::new("theory"),
            AdvancedCompletionItem::new("apply"),
        ]);
        let filtered = list.filter_by_prefix("the");
        assert_eq!(filtered.items.len(), 2);
    }
    #[test]
    fn test_keyword_provider() {
        let provider = KeywordCompletionProvider::new();
        let ctx = LspCompletionContext::invoked();
        let list = provider.completions("file:///test.lean", 0, 0, &ctx);
        assert!(!list.items.is_empty());
        assert!(list.items.iter().any(|i| i.label == "theorem"));
    }
    #[test]
    fn test_snippet_provider() {
        let provider = SnippetCompletionProvider::new();
        let ctx = LspCompletionContext::invoked();
        let list = provider.completions("file:///test.lean", 0, 0, &ctx);
        assert!(!list.items.is_empty());
        assert!(list.items.iter().any(|i| i.insert_text_format == 2));
    }
    #[test]
    fn test_completion_registry() {
        let registry = CompletionRegistry::with_defaults();
        let ctx = LspCompletionContext::invoked();
        let list = registry.completions("file:///test.lean", 0, 0, &ctx);
        assert!(!list.items.is_empty());
        assert!(list.items.iter().any(|i| i.label == "theorem"));
    }
    #[test]
    fn test_completion_stats() {
        let mut stats = AdvCompletionStats::default();
        stats.record_request(10);
        stats.record_request(20);
        stats.record_resolve();
        assert_eq!(stats.total_requests, 2);
        assert_eq!(stats.total_resolve_requests, 1);
        assert!((stats.avg_items_per_request - 15.0).abs() < 0.01);
    }
    #[test]
    fn test_snippet_definition() {
        let snip = SnippetDefinition::new("thm", "theorem ${1:name} := ${0}", "Theorem");
        assert_eq!(snip.prefix, "thm");
        assert!(snip.body.contains("${1:name}"));
    }
    #[test]
    fn test_deprecated_item() {
        let item = AdvancedCompletionItem::new("old_func").deprecated();
        assert!(item.deprecated);
        assert!(item.tags.contains(&CompletionItemTag::Deprecated));
    }
    #[test]
    fn test_completion_adv_version() {
        assert!(!completion_adv_version().is_empty());
    }
}
#[cfg(test)]
mod cache_format_tests {
    use super::*;
    #[test]
    fn test_completion_cache_basic() {
        let mut cache = CompletionCache::new(10);
        let list = CompletionList::complete(vec![AdvancedCompletionItem::new("theorem")]);
        cache.store("file:///a.lean", 5, 10, list.clone());
        let found = cache.get("file:///a.lean", 5, 10);
        assert!(found.is_some());
        assert_eq!(found.expect("test operation should succeed").items.len(), 1);
    }
    #[test]
    fn test_completion_cache_miss() {
        let cache = CompletionCache::new(10);
        let found = cache.get("file:///nonexistent.lean", 0, 0);
        assert!(found.is_none());
    }
    #[test]
    fn test_completion_cache_invalidate() {
        let mut cache = CompletionCache::new(10);
        let list = CompletionList::complete(vec![]);
        cache.store("file:///a.lean", 0, 0, list);
        cache.invalidate_uri("file:///a.lean");
        assert!(cache.get("file:///a.lean", 0, 0).is_none());
    }
    #[test]
    fn test_formatter_terminal() {
        let item = AdvancedCompletionItem::new("theorem").with_detail("Lean keyword");
        let formatted = CompletionFormatter::format_for_terminal(&item);
        assert!(formatted.contains("theorem"));
        assert!(formatted.contains("Lean keyword"));
    }
    #[test]
    fn test_formatter_terminal_deprecated() {
        let item = AdvancedCompletionItem::new("old_func").deprecated();
        let formatted = CompletionFormatter::format_for_terminal(&item);
        assert!(formatted.contains("[DEPRECATED]"));
    }
    #[test]
    fn test_formatter_json() {
        let item = AdvancedCompletionItem::new("theorem")
            .with_kind(14)
            .with_detail("Keyword");
        let json = CompletionFormatter::format_for_json(&item);
        assert!(json.contains("\"label\":\"theorem\""));
        assert!(json.contains("\"kind\":14"));
    }
    #[test]
    fn test_formatter_list_to_json() {
        let list = CompletionList::complete(vec![
            AdvancedCompletionItem::new("def"),
            AdvancedCompletionItem::new("theorem"),
        ]);
        let json = CompletionFormatter::list_to_json(&list);
        assert!(json.contains("\"isIncomplete\":false"));
        assert!(json.contains("\"items\":["));
    }
    #[test]
    fn test_completion_resolve_request() {
        let req = CompletionResolveRequest::new("theorem", None);
        assert_eq!(req.item_label, "theorem");
        assert!(req.item_data.is_none());
    }
}
/// Return feature list for completion_adv.
#[allow(dead_code)]
pub fn completion_adv_features() -> Vec<&'static str> {
    vec![
        "keywords",
        "snippets",
        "imports",
        "caching",
        "formatting",
        "registry",
        "stats",
        "lsp-protocol",
    ]
}
#[cfg(test)]
mod import_provider_tests {
    use super::*;
    #[test]
    fn test_import_provider() {
        let mut provider = ImportCompletionProvider::new(vec!["Mathlib.Algebra".to_string()]);
        provider.add_module("Mathlib.Data.Nat".to_string());
        let ctx = LspCompletionContext::invoked();
        let list = provider.completions("file:///a.lean", 0, 0, &ctx);
        assert_eq!(list.items.len(), 2);
    }
    #[test]
    fn test_completion_adv_features() {
        let features = completion_adv_features();
        assert!(features.contains(&"keywords"));
        assert!(features.contains(&"snippets"));
    }
}
/// A pipeline step for processing completions.
#[allow(dead_code)]
pub trait CompletionPipelineStep: Send + Sync {
    /// Process a completion list (filter, enrich, etc.).
    fn process(&self, list: CompletionList) -> CompletionList;
}
#[cfg(test)]
mod pipeline_tests {
    use super::*;
    #[test]
    fn test_limit_step() {
        let items: Vec<AdvancedCompletionItem> = (0..20)
            .map(|i| AdvancedCompletionItem::new(format!("item{}", i)))
            .collect();
        let list = CompletionList::complete(items);
        let step = LimitStep { max_items: 5 };
        let result = step.process(list);
        assert_eq!(result.items.len(), 5);
        assert!(result.is_incomplete);
    }
    #[test]
    fn test_remove_deprecated_step() {
        let list = CompletionList::complete(vec![
            AdvancedCompletionItem::new("good"),
            AdvancedCompletionItem::new("bad").deprecated(),
        ]);
        let step = RemoveDeprecatedStep;
        let result = step.process(list);
        assert_eq!(result.items.len(), 1);
        assert_eq!(result.items[0].label, "good");
    }
    #[test]
    fn test_pipeline_combine() {
        let items: Vec<AdvancedCompletionItem> = (0..10)
            .map(|i| AdvancedCompletionItem::new(format!("item{}", i)))
            .collect();
        let list = CompletionList::complete(items);
        let pipeline = CompletionPipeline::new()
            .add_step(Box::new(LimitStep { max_items: 5 }))
            .add_step(Box::new(RemoveDeprecatedStep));
        let result = pipeline.run(list);
        assert_eq!(result.items.len(), 5);
    }
}
