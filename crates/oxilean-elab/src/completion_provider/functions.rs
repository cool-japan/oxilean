//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    AnnotatedCompletionItem, BoostMiddleware, CompletionAnnotation, CompletionCache,
    CompletionCacheEntry, CompletionContext, CompletionDocumentation, CompletionHistogram,
    CompletionItem, CompletionKind, CompletionPipeline, CompletionProvider, CompletionRanker,
    CompletionScore, CompletionSession, CompletionStatistics, CompletionTrigger, DeduplicateStage,
    EnrichedCompletionContext, FilterChain, FuzzyMatcher, FuzzyWeights, KindFilter,
    LabelContainsFilter, LimitMiddleware, MinScoreFilter, SnippetCompletions, SnippetLibrary,
    SnippetTemplate, SortByScoreStage, TruncateStage,
};
use oxilean_kernel::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::completion_provider::*;
    #[test]
    fn test_completion_item_new_defaults() {
        let item = CompletionItem::new("simp", CompletionKind::Tactic);
        assert_eq!(item.label, "simp");
        assert!(matches!(item.kind, CompletionKind::Tactic));
        assert_eq!(item.insert_text, "simp");
        assert_eq!(item.score, 1.0);
        assert!(item.detail.is_none());
    }
    #[test]
    fn test_completion_item_builder() {
        let item = CompletionItem::new("ring", CompletionKind::Tactic)
            .with_detail("ring tactic")
            .with_insert("ring")
            .with_score(5.0);
        assert_eq!(item.detail.expect("tactic should succeed"), "ring tactic");
        assert_eq!(item.insert_text, "ring");
        assert!((item.score - 5.0).abs() < f32::EPSILON);
    }
    #[test]
    fn test_matches_prefix() {
        let item = CompletionItem::new("simp_all", CompletionKind::Tactic);
        assert!(item.matches_prefix("simp"));
        assert!(item.matches_prefix("simp_all"));
        assert!(!item.matches_prefix("ring"));
        assert!(!item.matches_prefix("simp_allx"));
    }
    #[test]
    fn test_completion_context_new() {
        let ctx = CompletionContext::new("sim", 3, 10);
        assert_eq!(ctx.prefix, "sim");
        assert_eq!(ctx.line, 3);
        assert_eq!(ctx.col, 10);
        assert!(!ctx.is_in_tactic_block);
        assert!(!ctx.is_in_type_position);
    }
    #[test]
    fn test_provider_complete_basic() {
        let mut p = CompletionProvider::new();
        p.add_item(CompletionItem::new("simp", CompletionKind::Tactic));
        p.add_item(CompletionItem::new("simp_all", CompletionKind::Tactic));
        p.add_item(CompletionItem::new("ring", CompletionKind::Tactic));
        let ctx = CompletionContext::new("sim", 0, 0);
        let results = p.complete(&ctx);
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_provider_complete_sorted_by_score() {
        let mut p = CompletionProvider::new();
        p.add_item(CompletionItem::new("apply", CompletionKind::Tactic).with_score(1.0));
        p.add_item(CompletionItem::new("assumption", CompletionKind::Tactic).with_score(3.0));
        p.add_item(CompletionItem::new("all_goals", CompletionKind::Tactic).with_score(2.0));
        let ctx = CompletionContext::new("a", 0, 0);
        let sorted = p.complete_sorted(&ctx);
        assert_eq!(sorted.len(), 3);
        assert_eq!(sorted[0].label, "assumption");
        assert_eq!(sorted[1].label, "all_goals");
        assert_eq!(sorted[2].label, "apply");
    }
    #[test]
    fn test_snippet_insert_text() {
        let t = SnippetCompletions::theorem_snippet();
        assert!(t.insert_text.contains("theorem"));
        assert!(t.insert_text.contains("sorry"));
        let d = SnippetCompletions::def_snippet();
        assert!(d.insert_text.contains("def"));
        let m = SnippetCompletions::match_snippet();
        assert!(m.insert_text.contains("match"));
        let f = SnippetCompletions::fun_snippet();
        assert!(f.insert_text.contains("fun"));
    }
    #[test]
    fn test_default_provider_has_tactics_and_types() {
        let p = CompletionProvider::default_provider();
        let ctx = CompletionContext::new("si", 0, 0);
        let results = p.complete(&ctx);
        let labels: Vec<&str> = results.iter().map(|r| r.label.as_str()).collect();
        assert!(labels.contains(&"simp"));
        assert!(labels.contains(&"simp_all"));
        assert!(labels.contains(&"simp_only"));
        let ctx2 = CompletionContext::new("Na", 0, 0);
        let results2 = p.complete(&ctx2);
        let labels2: Vec<&str> = results2.iter().map(|r| r.label.as_str()).collect();
        assert!(labels2.contains(&"Nat"));
    }
    #[test]
    fn test_tactic_block_filter() {
        let p = CompletionProvider::default_provider();
        let mut ctx = CompletionContext::new("", 0, 0);
        ctx.is_in_tactic_block = true;
        let results = p.complete(&ctx);
        for item in results {
            assert!(matches!(
                item.kind,
                CompletionKind::Tactic
                    | CompletionKind::Keyword
                    | CompletionKind::Snippet
                    | CompletionKind::Variable
                    | CompletionKind::Function
                    | CompletionKind::Theorem
            ));
        }
    }
}
#[cfg(test)]
mod extra_tests {
    use super::*;
    use crate::completion_provider::*;
    #[test]
    fn test_completion_kind_as_str_all_variants() {
        assert_eq!(CompletionKind::Function.as_str(), "function");
        assert_eq!(CompletionKind::Theorem.as_str(), "theorem");
        assert_eq!(CompletionKind::Type.as_str(), "type");
        assert_eq!(CompletionKind::Tactic.as_str(), "tactic");
        assert_eq!(CompletionKind::Variable.as_str(), "variable");
        assert_eq!(CompletionKind::Constructor.as_str(), "constructor");
        assert_eq!(CompletionKind::Field.as_str(), "field");
        assert_eq!(CompletionKind::Keyword.as_str(), "keyword");
        assert_eq!(CompletionKind::Snippet.as_str(), "snippet");
    }
    #[test]
    fn test_completion_kind_eq_and_clone() {
        let k = CompletionKind::Tactic;
        let k2 = k.clone();
        assert_eq!(k, k2);
        assert_ne!(k, CompletionKind::Keyword);
    }
    #[test]
    fn test_completion_item_sort_key_defaults_to_label() {
        let item = CompletionItem::new("myFunc", CompletionKind::Function);
        assert_eq!(item.sort_key, "myFunc");
        assert!(item.documentation.is_none());
    }
    #[test]
    fn test_matches_prefix_empty_always_true() {
        let item = CompletionItem::new("anything", CompletionKind::Tactic);
        assert!(item.matches_prefix(""));
    }
    #[test]
    fn test_matches_prefix_case_sensitive() {
        let item = CompletionItem::new("Simp", CompletionKind::Tactic);
        assert!(!item.matches_prefix("simp"));
        assert!(item.matches_prefix("Si"));
    }
    #[test]
    fn test_completion_context_flags() {
        let mut ctx = CompletionContext::new("si", 5, 10);
        ctx.is_in_tactic_block = true;
        ctx.is_in_type_position = true;
        assert!(ctx.is_in_tactic_block);
        assert!(ctx.is_in_type_position);
        assert_eq!(ctx.line, 5);
        assert_eq!(ctx.col, 10);
    }
    #[test]
    fn test_provider_no_match_returns_empty() {
        let p = CompletionProvider::default_provider();
        let ctx = CompletionContext::new("ZZNOMATCH", 0, 0);
        assert!(p.complete(&ctx).is_empty());
    }
    #[test]
    fn test_provider_sorted_same_score_alpha_order() {
        let mut p = CompletionProvider::new();
        p.add_item(CompletionItem::new("c_item", CompletionKind::Tactic).with_score(2.0));
        p.add_item(CompletionItem::new("a_item", CompletionKind::Tactic).with_score(2.0));
        p.add_item(CompletionItem::new("b_item", CompletionKind::Tactic).with_score(2.0));
        let ctx = CompletionContext::new("", 0, 0);
        let sorted = p.complete_sorted(&ctx);
        assert_eq!(sorted[0].label, "a_item");
        assert_eq!(sorted[1].label, "b_item");
        assert_eq!(sorted[2].label, "c_item");
    }
    #[test]
    fn test_provider_tactic_block_excludes_type() {
        let mut p = CompletionProvider::new();
        p.add_item(CompletionItem::new("Nat", CompletionKind::Type));
        p.add_item(CompletionItem::new("simp", CompletionKind::Tactic));
        let mut ctx = CompletionContext::new("", 0, 0);
        ctx.is_in_tactic_block = true;
        let results = p.complete(&ctx);
        assert!(!results
            .iter()
            .any(|r| matches!(r.kind, CompletionKind::Type)));
        assert!(results
            .iter()
            .any(|r| matches!(r.kind, CompletionKind::Tactic)));
    }
    #[test]
    fn test_provider_tactic_block_includes_function_theorem() {
        let mut p = CompletionProvider::new();
        p.add_item(CompletionItem::new("myFn", CompletionKind::Function));
        p.add_item(CompletionItem::new("myThm", CompletionKind::Theorem));
        let mut ctx = CompletionContext::new("", 0, 0);
        ctx.is_in_tactic_block = true;
        let results = p.complete(&ctx);
        assert!(results.iter().any(|r| r.label == "myFn"));
        assert!(results.iter().any(|r| r.label == "myThm"));
    }
    #[test]
    fn test_provider_default_is_empty() {
        let p = CompletionProvider::default();
        let ctx = CompletionContext::new("", 0, 0);
        assert!(p.complete(&ctx).is_empty());
    }
    #[test]
    fn test_register_tactic_completions_scores() {
        let mut p = CompletionProvider::new();
        p.register_tactic_completions();
        let ctx = CompletionContext::new("intro", 0, 0);
        let results = p.complete(&ctx);
        for item in &results {
            assert!((item.score - 2.0).abs() < f32::EPSILON);
        }
    }
    #[test]
    fn test_register_keyword_completions_scores() {
        let mut p = CompletionProvider::new();
        p.register_keyword_completions();
        let ctx = CompletionContext::new("def", 0, 0);
        let results = p.complete(&ctx);
        for item in &results {
            assert!((item.score - 1.5).abs() < f32::EPSILON);
        }
    }
    #[test]
    fn test_register_type_completions_scores() {
        let mut p = CompletionProvider::new();
        p.register_type_completions();
        let ctx = CompletionContext::new("Nat", 0, 0);
        let results = p.complete(&ctx);
        for item in &results {
            assert!((item.score - 1.2).abs() < f32::EPSILON);
        }
    }
    #[test]
    fn test_snippet_kinds() {
        assert!(matches!(
            SnippetCompletions::theorem_snippet().kind,
            CompletionKind::Snippet
        ));
        assert!(matches!(
            SnippetCompletions::def_snippet().kind,
            CompletionKind::Snippet
        ));
        assert!(matches!(
            SnippetCompletions::match_snippet().kind,
            CompletionKind::Snippet
        ));
        assert!(matches!(
            SnippetCompletions::fun_snippet().kind,
            CompletionKind::Snippet
        ));
    }
    #[test]
    fn test_snippet_scores() {
        assert!((SnippetCompletions::theorem_snippet().score - 3.0).abs() < f32::EPSILON);
        assert!((SnippetCompletions::def_snippet().score - 3.0).abs() < f32::EPSILON);
        assert!((SnippetCompletions::match_snippet().score - 2.5).abs() < f32::EPSILON);
        assert!((SnippetCompletions::fun_snippet().score - 2.5).abs() < f32::EPSILON);
    }
    #[test]
    fn test_snippet_labels() {
        assert_eq!(SnippetCompletions::theorem_snippet().label, "theorem!");
        assert_eq!(SnippetCompletions::def_snippet().label, "def!");
        assert_eq!(SnippetCompletions::match_snippet().label, "match!");
        assert_eq!(SnippetCompletions::fun_snippet().label, "fun!");
    }
    #[test]
    fn test_default_provider_type_completions_present() {
        let p = CompletionProvider::default_provider();
        let ctx = CompletionContext::new("Bool", 0, 0);
        let results = p.complete(&ctx);
        assert!(results.iter().any(|r| r.label == "Bool"));
    }
}
/// Return true if the character is a word separator for fuzzy matching.
pub fn is_separator(c: char) -> bool {
    matches!(c, '_' | '.' | ':' | ' ' | '-' | '/' | '\\')
}
#[cfg(test)]
mod fuzzy_and_session_tests {
    use super::*;
    use crate::completion_provider::*;
    #[test]
    fn test_fuzzy_matcher_exact() {
        let matcher = FuzzyMatcher::new();
        let result = matcher.matches("simp", "simp");
        assert!(result.matched);
        assert!(result.score > 0.0);
    }
    #[test]
    fn test_fuzzy_matcher_prefix() {
        let matcher = FuzzyMatcher::new();
        let result = matcher.matches("si", "simp");
        assert!(result.matched);
    }
    #[test]
    fn test_fuzzy_matcher_non_contiguous() {
        let matcher = FuzzyMatcher::new();
        let result = matcher.matches("sp", "simp");
        assert!(result.matched);
        assert!(result.match_positions.contains(&3));
    }
    #[test]
    fn test_fuzzy_matcher_no_match() {
        let matcher = FuzzyMatcher::new();
        let result = matcher.matches("xyz", "simp");
        assert!(!result.matched);
    }
    #[test]
    fn test_fuzzy_matcher_empty_query() {
        let matcher = FuzzyMatcher::new();
        let result = matcher.matches("", "anything");
        assert!(result.matched);
    }
    #[test]
    fn test_fuzzy_matcher_case_insensitive() {
        let matcher = FuzzyMatcher::new();
        let result = matcher.matches("SIMP", "simp_all");
        assert!(result.matched);
    }
    #[test]
    fn test_fuzzy_matcher_case_sensitive_no_match() {
        let matcher = FuzzyMatcher::case_sensitive();
        let result = matcher.matches("SIMP", "simp_all");
        assert!(!result.matched);
    }
    #[test]
    fn test_fuzzy_score_and_sort() {
        let matcher = FuzzyMatcher::new();
        let candidates = ["simp_all", "simple", "simp", "sieve"];
        let sorted = matcher.score_and_sort("simp", &candidates);
        assert!(!sorted.is_empty());
        assert_eq!(sorted[0].0, "simp");
    }
    #[test]
    fn test_completion_session_basic() {
        let mut session = CompletionSession::new(5);
        assert!(session.is_empty());
        session.accept("simp");
        session.accept("ring");
        session.accept("omega");
        assert_eq!(session.len(), 3);
        assert_eq!(session.recent_labels()[0], "omega");
        assert!(session.recency_bonus("omega") > session.recency_bonus("simp"));
        assert_eq!(session.recency_bonus("unknown"), 0.0);
    }
    #[test]
    fn test_completion_session_dedup() {
        let mut session = CompletionSession::new(5);
        session.accept("simp");
        session.accept("ring");
        session.accept("simp");
        assert_eq!(session.len(), 2);
        assert_eq!(session.recent_labels()[0], "simp");
    }
    #[test]
    fn test_completion_session_max_recent() {
        let mut session = CompletionSession::new(3);
        session.accept("a");
        session.accept("b");
        session.accept("c");
        session.accept("d");
        assert_eq!(session.len(), 3);
        assert!(!session.recent_labels().contains(&"a".to_string()));
    }
    #[test]
    fn test_snippet_template_placeholder_count() {
        let t = SnippetTemplate::new(
            "id",
            "trig",
            "theorem ${1:name} : ${2:type} := by\n  ${3:sorry}",
            "desc",
        );
        assert_eq!(t.placeholder_count(), 3);
    }
    #[test]
    fn test_snippet_template_to_completion_item() {
        let t = SnippetTemplate::new("id", "trigger", "template text", "desc");
        let item = t.to_completion_item();
        assert_eq!(item.label, "trigger");
        assert!(matches!(item.kind, CompletionKind::Snippet));
        assert_eq!(item.insert_text, "template text");
        assert_eq!(item.detail.as_deref(), Some("desc"));
    }
    #[test]
    fn test_snippet_library_default() {
        let lib = SnippetLibrary::default_library();
        assert!(!lib.is_empty());
        let thm = lib.matching_trigger("thm");
        assert!(!thm.is_empty());
        let items = lib.to_completion_items();
        assert_eq!(items.len(), lib.len());
    }
    #[test]
    fn test_snippet_library_no_match() {
        let lib = SnippetLibrary::default_library();
        let no_match = lib.matching_trigger("zzzz");
        assert!(no_match.is_empty());
    }
    #[test]
    fn test_is_separator() {
        assert!(is_separator('_'));
        assert!(is_separator('.'));
        assert!(is_separator(':'));
        assert!(!is_separator('a'));
        assert!(!is_separator('Z'));
    }
    #[test]
    fn test_completion_score_total() {
        let score = CompletionScore {
            fuzzy_score: 3.5,
            kind_score: 1.2,
            recency_bonus: 0.5,
            user_boost: 0.0,
        };
        assert!((score.total() - 5.2).abs() < 1e-5);
    }
    #[test]
    fn test_fuzzy_matcher_score_completions() {
        let matcher = FuzzyMatcher::new();
        let items = vec![
            CompletionItem::new("simp", CompletionKind::Tactic).with_score(2.0),
            CompletionItem::new("simp_all", CompletionKind::Tactic).with_score(2.0),
            CompletionItem::new("ring", CompletionKind::Tactic).with_score(2.0),
        ];
        let scored = matcher.score_completions("si", &items);
        assert_eq!(scored.len(), 2);
        assert!(scored[0].1 >= scored[1].1);
    }
    #[test]
    fn test_fuzzy_weights_default() {
        let w = FuzzyWeights::default();
        assert!(w.prefix_bonus > 0.0);
        assert!(w.word_start_bonus > 0.0);
        assert!(w.consecutive_bonus > 0.0);
        assert!(w.gap_penalty < 0.0);
    }
}
/// A predicate that filters completion items.
pub trait CompletionFilter: Send + Sync {
    /// Return true if the given item should be included.
    fn accepts(&self, item: &CompletionItem, ctx: &CompletionContext) -> bool;
    /// Name of this filter.
    fn filter_name(&self) -> &str;
}
#[cfg(test)]
mod filter_and_ranker_tests {
    use super::*;
    use crate::completion_provider::*;
    #[test]
    fn test_kind_filter() {
        let filter = KindFilter::tactics_only();
        let ctx = CompletionContext::new("", 0, 0);
        let tactic_item = CompletionItem::new("simp", CompletionKind::Tactic);
        let type_item = CompletionItem::new("Nat", CompletionKind::Type);
        assert!(filter.accepts(&tactic_item, &ctx));
        assert!(!filter.accepts(&type_item, &ctx));
        assert_eq!(filter.filter_name(), "kind_filter");
    }
    #[test]
    fn test_min_score_filter() {
        let filter = MinScoreFilter::new(2.0);
        let ctx = CompletionContext::new("", 0, 0);
        let low = CompletionItem::new("x", CompletionKind::Variable).with_score(1.0);
        let high = CompletionItem::new("y", CompletionKind::Variable).with_score(3.0);
        let exact = CompletionItem::new("z", CompletionKind::Variable).with_score(2.0);
        assert!(!filter.accepts(&low, &ctx));
        assert!(filter.accepts(&high, &ctx));
        assert!(filter.accepts(&exact, &ctx));
        assert_eq!(filter.filter_name(), "min_score_filter");
    }
    #[test]
    fn test_label_contains_filter() {
        let filter = LabelContainsFilter::new("_all");
        let ctx = CompletionContext::new("", 0, 0);
        let match_item = CompletionItem::new("simp_all", CompletionKind::Tactic);
        let no_match = CompletionItem::new("simp", CompletionKind::Tactic);
        assert!(filter.accepts(&match_item, &ctx));
        assert!(!filter.accepts(&no_match, &ctx));
        assert_eq!(filter.filter_name(), "label_contains_filter");
    }
    #[test]
    fn test_filter_chain_empty() {
        let chain = FilterChain::new();
        let ctx = CompletionContext::new("", 0, 0);
        let item = CompletionItem::new("x", CompletionKind::Variable);
        assert!(chain.accepts(&item, &ctx));
    }
    #[test]
    fn test_filter_chain_combined() {
        let mut chain = FilterChain::new();
        chain.add(KindFilter::tactics_only());
        chain.add(MinScoreFilter::new(2.0));
        let ctx = CompletionContext::new("", 0, 0);
        let items = vec![
            CompletionItem::new("simp", CompletionKind::Tactic).with_score(2.5),
            CompletionItem::new("ring", CompletionKind::Tactic).with_score(1.0),
            CompletionItem::new("Nat", CompletionKind::Type).with_score(3.0),
        ];
        let accepted = chain.apply(&items, &ctx);
        assert_eq!(accepted.len(), 1);
        assert_eq!(accepted[0].label, "simp");
        let names = chain.filter_names();
        assert!(names.contains(&"kind_filter"));
        assert!(names.contains(&"min_score_filter"));
    }
    #[test]
    fn test_completion_ranker_basic() {
        let ranker = CompletionRanker::new();
        let items = vec![
            CompletionItem::new("simp", CompletionKind::Tactic).with_score(2.0),
            CompletionItem::new("simp_all", CompletionKind::Tactic).with_score(2.0),
            CompletionItem::new("ring", CompletionKind::Tactic).with_score(2.0),
        ];
        let ranked = ranker.rank("si", &items);
        assert_eq!(ranked.len(), 2);
        assert!(ranked[0].1 >= ranked[1].1);
    }
    #[test]
    fn test_completion_ranker_with_recency() {
        let mut ranker = CompletionRanker::new();
        ranker.record_acceptance("ring");
        let items = vec![
            CompletionItem::new("ring", CompletionKind::Tactic).with_score(1.0),
            CompletionItem::new("rw", CompletionKind::Tactic).with_score(1.0),
        ];
        let ranked = ranker.rank("r", &items);
        assert_eq!(ranked.len(), 2);
        assert_eq!(ranked[0].0.label, "ring");
    }
    #[test]
    fn test_completion_ranker_no_match() {
        let ranker = CompletionRanker::new();
        let items = vec![CompletionItem::new("simp", CompletionKind::Tactic)];
        let ranked = ranker.rank("zz", &items);
        assert!(ranked.is_empty());
    }
    #[test]
    fn test_kinds_filter_multiple_kinds() {
        let filter = KindFilter::new(vec![CompletionKind::Tactic, CompletionKind::Keyword]);
        let ctx = CompletionContext::new("", 0, 0);
        let t = CompletionItem::new("simp", CompletionKind::Tactic);
        let k = CompletionItem::new("def", CompletionKind::Keyword);
        let ty = CompletionItem::new("Nat", CompletionKind::Type);
        assert!(filter.accepts(&t, &ctx));
        assert!(filter.accepts(&k, &ctx));
        assert!(!filter.accepts(&ty, &ctx));
    }
}
#[cfg(test)]
mod documentation_tests {
    use super::*;
    use crate::completion_provider::*;
    #[test]
    fn test_completion_documentation_basic() {
        let doc = CompletionDocumentation::summary("closes ring goals");
        assert_eq!(doc.summary, "closes ring goals");
        assert!(doc.description.is_none());
        assert!(doc.example.is_none());
        assert!(doc.see_also.is_empty());
    }
    #[test]
    fn test_completion_documentation_full() {
        let mut doc = CompletionDocumentation::summary("ring tactic")
            .with_description("Solves ring-arithmetic equality goals.")
            .with_example("ring");
        doc.add_see_also("linarith");
        doc.add_see_also("omega");
        let md = doc.to_markdown();
        assert!(md.contains("ring tactic"));
        assert!(md.contains("ring-arithmetic"));
        assert!(md.contains("linarith"));
        assert!(md.contains("omega"));
        assert!(md.contains("```"));
    }
    #[test]
    fn test_completion_documentation_default() {
        let doc = CompletionDocumentation::default();
        assert!(doc.summary.is_empty());
        let md = doc.to_markdown();
        assert!(md.contains("**"));
    }
}
#[cfg(test)]
mod cache_and_trigger_tests {
    use super::*;
    use crate::completion_provider::*;
    #[test]
    fn test_completion_cache_basic() {
        let mut cache = CompletionCache::new(5);
        assert!(cache.is_empty());
        cache.put("si", vec!["simp".to_string(), "simp_all".to_string()]);
        assert_eq!(cache.len(), 1);
        assert_eq!(cache.valid_len(), 1);
        let entry = cache.get("si").expect("key should exist");
        assert_eq!(entry.item_labels.len(), 2);
        assert!(cache.get("ring").is_none());
    }
    #[test]
    fn test_completion_cache_update() {
        let mut cache = CompletionCache::new(5);
        cache.put("si", vec!["simp".to_string()]);
        cache.put("si", vec!["simp".to_string(), "simp_all".to_string()]);
        assert_eq!(cache.len(), 1);
        let entry = cache.get("si").expect("key should exist");
        assert_eq!(entry.item_labels.len(), 2);
    }
    #[test]
    fn test_completion_cache_invalidate_prefix() {
        let mut cache = CompletionCache::new(10);
        cache.put("si", vec!["simp".to_string()]);
        cache.put("sim", vec!["simp".to_string()]);
        cache.put("ring", vec!["ring".to_string()]);
        cache.invalidate_prefix("si");
        assert_eq!(cache.valid_len(), 1);
        assert!(cache.get("si").is_none());
        assert!(cache.get("sim").is_none());
        assert!(cache.get("ring").is_some());
    }
    #[test]
    fn test_completion_cache_eviction() {
        let mut cache = CompletionCache::new(3);
        cache.put("a", vec!["a".to_string()]);
        cache.put("b", vec!["b".to_string()]);
        cache.put("c", vec!["c".to_string()]);
        cache.put("d", vec!["d".to_string()]);
        assert_eq!(cache.len(), 3);
        assert!(cache.get("a").is_none());
        assert!(cache.get("d").is_some());
    }
    #[test]
    fn test_completion_cache_clear() {
        let mut cache = CompletionCache::new(5);
        cache.put("x", vec!["x".to_string()]);
        cache.put("y", vec!["y".to_string()]);
        cache.clear();
        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);
    }
    #[test]
    fn test_completion_trigger_variants() {
        let t1 = CompletionTrigger::Invoked;
        assert!(t1.is_invoked());
        assert!(t1.trigger_char().is_none());
        let t2 = CompletionTrigger::TriggerCharacter('.');
        assert!(!t2.is_invoked());
        assert_eq!(t2.trigger_char(), Some('.'));
        let t3 = CompletionTrigger::TriggerForIncompleteCompletions;
        assert!(!t3.is_invoked());
        assert!(t3.trigger_char().is_none());
    }
    #[test]
    fn test_enriched_completion_context() {
        let base = CompletionContext::new("si", 3, 7);
        let enriched = EnrichedCompletionContext::from_base(base)
            .with_trigger(CompletionTrigger::TriggerCharacter('.'))
            .with_surrounding("let x :=");
        assert_eq!(enriched.base.prefix, "si");
        assert_eq!(enriched.trigger.trigger_char(), Some('.'));
        assert_eq!(enriched.surrounding.as_deref(), Some("let x :="));
    }
    #[test]
    fn test_completion_trigger_eq() {
        assert_eq!(CompletionTrigger::Invoked, CompletionTrigger::Invoked);
        assert_ne!(
            CompletionTrigger::Invoked,
            CompletionTrigger::TriggerCharacter('.')
        );
    }
    #[test]
    fn test_cache_entry_invalidate() {
        let mut entry = CompletionCacheEntry::new("prefix", vec!["item".to_string()]);
        assert!(entry.valid);
        entry.invalidate();
        assert!(!entry.valid);
    }
}
/// A hook applied before or after completion computation.
pub trait CompletionMiddleware: Send + Sync {
    /// Name of this middleware.
    fn middleware_name(&self) -> &str;
    /// Pre-process the context before querying.
    fn pre_complete(&self, _ctx: &mut CompletionContext) {}
    /// Post-process the results after querying.
    fn post_complete(&self, _ctx: &CompletionContext, _items: &mut Vec<CompletionItem>) {}
}
#[cfg(test)]
mod stats_and_middleware_tests {
    use super::*;
    use crate::completion_provider::*;
    #[test]
    fn test_completion_statistics_basic() {
        let mut stats = CompletionStatistics::new();
        stats.record_request();
        stats.record_request();
        stats.record_acceptance("simp");
        stats.record_acceptance("ring");
        stats.record_acceptance("simp");
        assert_eq!(stats.total_requests, 2);
        assert_eq!(stats.total_accepted, 3);
        assert!((stats.acceptance_rate() - 1.5).abs() < 1e-9);
        assert_eq!(stats.distinct_accepted(), 2);
        let top = stats.top_accepted(1);
        assert_eq!(top.len(), 1);
        assert_eq!(top[0].0, "simp");
        assert_eq!(top[0].1, 2);
    }
    #[test]
    fn test_completion_statistics_empty() {
        let stats = CompletionStatistics::new();
        assert_eq!(stats.acceptance_rate(), 0.0);
        assert_eq!(stats.distinct_accepted(), 0);
        assert!(stats.top_accepted(5).is_empty());
    }
    #[test]
    fn test_limit_middleware() {
        let mw = LimitMiddleware::new(2);
        let ctx = CompletionContext::new("", 0, 0);
        let mut items = vec![
            CompletionItem::new("a", CompletionKind::Tactic),
            CompletionItem::new("b", CompletionKind::Tactic),
            CompletionItem::new("c", CompletionKind::Tactic),
        ];
        mw.post_complete(&ctx, &mut items);
        assert_eq!(items.len(), 2);
        assert_eq!(mw.middleware_name(), "limit_middleware");
    }
    #[test]
    fn test_boost_middleware() {
        let mw = BoostMiddleware::new("simp", 5.0);
        let ctx = CompletionContext::new("", 0, 0);
        let mut items = vec![
            CompletionItem::new("simp_all", CompletionKind::Tactic).with_score(1.0),
            CompletionItem::new("ring", CompletionKind::Tactic).with_score(1.0),
        ];
        mw.post_complete(&ctx, &mut items);
        assert!((items[0].score - 6.0).abs() < f32::EPSILON);
        assert!((items[1].score - 1.0).abs() < f32::EPSILON);
        assert_eq!(mw.middleware_name(), "boost_middleware");
    }
    #[test]
    fn test_middleware_pre_complete_noop() {
        let mw = LimitMiddleware::new(10);
        let mut ctx = CompletionContext::new("test", 0, 0);
        mw.pre_complete(&mut ctx);
        assert_eq!(ctx.prefix, "test");
    }
    #[test]
    fn test_statistics_top_accepted_sorting() {
        let mut stats = CompletionStatistics::new();
        for _ in 0..3 {
            stats.record_acceptance("c");
        }
        for _ in 0..5 {
            stats.record_acceptance("a");
        }
        for _ in 0..1 {
            stats.record_acceptance("b");
        }
        let top = stats.top_accepted(3);
        assert_eq!(top[0].0, "a");
        assert_eq!(top[1].0, "c");
        assert_eq!(top[2].0, "b");
    }
}
#[cfg(test)]
mod annotation_tests {
    use super::*;
    use crate::completion_provider::*;
    #[test]
    fn test_completion_annotation_builtin() {
        let ann = CompletionAnnotation::builtin();
        assert!(!ann.user_defined);
        assert!(!ann.deprecated);
        assert!(ann.module_path.is_none());
    }
    #[test]
    fn test_completion_annotation_user() {
        let ann = CompletionAnnotation::user()
            .deprecated()
            .with_module("MyLib")
            .with_type("Nat -> Nat");
        assert!(ann.user_defined);
        assert!(ann.deprecated);
        assert_eq!(ann.module_path.as_deref(), Some("MyLib"));
        assert_eq!(ann.type_signature.as_deref(), Some("Nat -> Nat"));
    }
    #[test]
    fn test_annotated_completion_item() {
        let item = CompletionItem::new("myFn", CompletionKind::Function);
        let ann = CompletionAnnotation::user().with_type("Nat -> Bool");
        let annotated = AnnotatedCompletionItem::new(item, ann);
        assert_eq!(annotated.label(), "myFn");
        assert!(!annotated.is_deprecated());
        assert_eq!(annotated.type_signature(), Some("Nat -> Bool"));
    }
    #[test]
    fn test_annotated_item_deprecated() {
        let item = CompletionItem::new("oldFn", CompletionKind::Function);
        let ann = CompletionAnnotation::builtin().deprecated();
        let annotated = AnnotatedCompletionItem::new(item, ann);
        assert!(annotated.is_deprecated());
    }
    #[test]
    fn test_completion_annotation_default() {
        let ann = CompletionAnnotation::default();
        assert!(!ann.deprecated);
        assert!(!ann.user_defined);
        assert!(ann.module_path.is_none());
        assert!(ann.type_signature.is_none());
    }
    #[test]
    fn test_completion_cache_entry_new() {
        let entry = CompletionCacheEntry::new("prefix", vec!["item1".to_string()]);
        assert_eq!(entry.prefix, "prefix");
        assert!(entry.valid);
        assert_eq!(entry.item_labels.len(), 1);
    }
    #[test]
    fn test_filter_chain_names_all() {
        let mut chain = FilterChain::new();
        chain.add(KindFilter::tactics_only());
        chain.add(MinScoreFilter::new(1.0));
        chain.add(LabelContainsFilter::new("si"));
        let names = chain.filter_names();
        assert_eq!(names.len(), 3);
        assert!(names.contains(&"kind_filter"));
        assert!(names.contains(&"min_score_filter"));
        assert!(names.contains(&"label_contains_filter"));
    }
    #[test]
    fn test_completion_ranker_sort_stability_same_score() {
        let ranker = CompletionRanker::new();
        let items = vec![
            CompletionItem::new("cc_item", CompletionKind::Tactic).with_score(1.0),
            CompletionItem::new("aa_item", CompletionKind::Tactic).with_score(1.0),
            CompletionItem::new("bb_item", CompletionKind::Tactic).with_score(1.0),
        ];
        let ranked = ranker.rank("", &items);
        assert_eq!(ranked[0].0.label, "aa_item");
        assert_eq!(ranked[1].0.label, "bb_item");
        assert_eq!(ranked[2].0.label, "cc_item");
    }
}
#[allow(dead_code)]
pub trait CompletionStage: Send + Sync {
    fn name(&self) -> &'static str;
    fn process(&self, ctx: &CompletionContext, items: Vec<CompletionItem>) -> Vec<CompletionItem>;
}
#[cfg(test)]
mod pipeline_tests {
    use super::*;
    use crate::completion_provider::*;
    #[test]
    fn test_deduplicate_stage() {
        let stage = DeduplicateStage;
        let ctx = CompletionContext::new("", 0, 0);
        let items = vec![
            CompletionItem::new("foo", CompletionKind::Function),
            CompletionItem::new("foo", CompletionKind::Function),
            CompletionItem::new("bar", CompletionKind::Function),
        ];
        let result = stage.process(&ctx, items);
        assert_eq!(result.len(), 2);
    }
    #[test]
    fn test_sort_by_score_stage() {
        let stage = SortByScoreStage;
        let ctx = CompletionContext::new("", 0, 0);
        let items = vec![
            CompletionItem::new("low", CompletionKind::Function).with_score(0.1),
            CompletionItem::new("high", CompletionKind::Function).with_score(0.9),
            CompletionItem::new("mid", CompletionKind::Function).with_score(0.5),
        ];
        let result = stage.process(&ctx, items);
        assert_eq!(result[0].label, "high");
        assert_eq!(result[2].label, "low");
    }
    #[test]
    fn test_truncate_stage() {
        let stage = TruncateStage::new(2);
        let ctx = CompletionContext::new("", 0, 0);
        let items = vec![
            CompletionItem::new("a", CompletionKind::Function),
            CompletionItem::new("b", CompletionKind::Function),
            CompletionItem::new("c", CompletionKind::Function),
        ];
        let result = stage.process(&ctx, items);
        assert_eq!(result.len(), 2);
    }
    #[test]
    fn test_pipeline_stages() {
        let pipeline = CompletionPipeline::new()
            .add_stage(DeduplicateStage)
            .add_stage(SortByScoreStage)
            .add_stage(TruncateStage::new(5));
        assert_eq!(
            pipeline.stage_names(),
            vec!["deduplicate", "sort_by_score", "truncate"]
        );
    }
    #[test]
    fn test_histogram() {
        let mut hist = CompletionHistogram::new();
        let items = vec![
            CompletionItem::new("a", CompletionKind::Function),
            CompletionItem::new("b", CompletionKind::Tactic),
            CompletionItem::new("c", CompletionKind::Function),
        ];
        hist.record_all(&items);
        assert_eq!(hist.count(&CompletionKind::Function), 2);
        assert_eq!(hist.count(&CompletionKind::Tactic), 1);
        assert_eq!(hist.total(), 3);
        let top = hist.top_kinds(1);
        assert_eq!(top.len(), 1);
    }
}
