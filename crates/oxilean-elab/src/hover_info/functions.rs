//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    ContextualHover, HoverAnnotation, HoverAnnotationKind, HoverCache, HoverDiagnostic,
    HoverDiagnosticCollection, HoverDiagnosticSeverity, HoverDocReference, HoverDocReferenceIndex,
    HoverEnricher, HoverEntry, HoverFormat, HoverFormatter, HoverGoalInfo, HoverHistory,
    HoverHypothesisInfo, HoverIndex, HoverInfo, HoverInfoBuilder, HoverKind, HoverLink,
    HoverLocation, HoverMarkdown, HoverModuleInfo, HoverPerformanceMonitor, HoverProvider,
    HoverRegionMerger, HoverRenderer, HoverRendererConfig, HoverRequestContext, HoverResponse,
    HoverResult, HoverStats, HoverThrottler, HoverTypeSignatureParser, TacticSuggestionEngine,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hover_info::*;
    #[test]
    fn test_hover_result_new() {
        let r = HoverResult::new(HoverKind::Theorem, "Nat.add_comm");
        assert_eq!(r.name, "Nat.add_comm");
        assert!(matches!(r.kind, HoverKind::Theorem));
        assert!(r.type_signature.is_empty());
        assert!(r.documentation.is_empty());
        assert!(r.source_location.is_none());
    }
    #[test]
    fn test_hover_result_builder() {
        let r = HoverResult::new(HoverKind::Definition, "id")
            .with_type("forall (a : Type), a -> a")
            .with_doc("The identity function.")
            .with_location("Prelude.lean:10:0");
        assert_eq!(r.type_signature, "forall (a : Type), a -> a");
        assert_eq!(r.documentation, "The identity function.");
        assert_eq!(
            r.source_location.expect("test operation should succeed"),
            "Prelude.lean:10:0"
        );
    }
    #[test]
    fn test_to_markdown_contains_name() {
        let r = HoverResult::new(HoverKind::Tactic, "simp").with_doc("Simplify the goal.");
        let md = r.to_markdown();
        assert!(md.contains("simp"));
        assert!(md.contains("tactic"));
        assert!(md.contains("Simplify the goal."));
    }
    #[test]
    fn test_to_markdown_code_block() {
        let r = HoverResult::new(HoverKind::Theorem, "Nat.add_comm")
            .with_type("forall (n m : Nat), n + m = m + n");
        let md = r.to_markdown();
        assert!(md.contains("```lean"));
        assert!(md.contains("forall (n m : Nat), n + m = m + n"));
        assert!(md.contains("```"));
    }
    #[test]
    fn test_to_plain_text() {
        let r = HoverResult::new(HoverKind::Axiom, "Classical.em")
            .with_type("forall (p : Prop), p ∨ ¬p");
        let plain = r.to_plain_text();
        assert!(plain.contains("[axiom]"));
        assert!(plain.contains("Classical.em"));
        assert!(plain.contains("forall (p : Prop), p ∨ ¬p"));
    }
    #[test]
    fn test_hover_provider_register_and_lookup() {
        let mut p = HoverProvider::new();
        p.register(
            "foo",
            HoverResult::new(HoverKind::Definition, "foo").with_doc("A function."),
        );
        let r = p.lookup("foo").expect("should find foo");
        assert_eq!(r.name, "foo");
        assert!(p.lookup("bar").is_none());
    }
    #[test]
    fn test_hover_provider_lookup_prefix() {
        let mut p = HoverProvider::new();
        p.register_tactic("intro", "Introduce hypothesis.");
        p.register_tactic("intros", "Introduce multiple hypotheses.");
        p.register_tactic("exact", "Exact proof term.");
        let results = p.lookup_prefix("int");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_default_tactics_populated() {
        let p = HoverProvider::default_tactics();
        assert!(p.lookup("simp").is_some());
        assert!(p.lookup("ring").is_some());
        assert!(p.lookup("linarith").is_some());
        assert!(p.lookup("sorry").is_some());
    }
    #[test]
    fn test_contextual_hover_tactic_and_type() {
        let ch = ContextualHover::new();
        let r = ch.hover_at_word("intro", "", 0, 0);
        assert!(r.is_some());
        assert!(matches!(
            r.expect("test operation should succeed").kind,
            HoverKind::Tactic
        ));
        let r2 = ch.hover_at_word("Nat", "", 0, 0);
        assert!(r2.is_some());
        assert!(ContextualHover::is_tactic_keyword("simp"));
        assert!(!ContextualHover::is_tactic_keyword("Nat"));
        assert!(ContextualHover::is_builtin_type("Nat"));
        assert!(!ContextualHover::is_builtin_type("simp"));
    }
}
#[cfg(test)]
mod extra_tests {
    use super::*;
    use crate::hover_info::*;
    #[test]
    fn test_hover_kind_as_str_all_variants() {
        assert_eq!(HoverKind::Definition.as_str(), "def");
        assert_eq!(HoverKind::Theorem.as_str(), "theorem");
        assert_eq!(HoverKind::Axiom.as_str(), "axiom");
        assert_eq!(HoverKind::LocalVar.as_str(), "variable");
        assert_eq!(HoverKind::Constructor.as_str(), "constructor");
        assert_eq!(HoverKind::Field.as_str(), "field");
        assert_eq!(HoverKind::Tactic.as_str(), "tactic");
        assert_eq!(HoverKind::Universe.as_str(), "universe");
        assert_eq!(HoverKind::Keyword.as_str(), "keyword");
    }
    #[test]
    fn test_hover_kind_eq_and_clone() {
        let k = HoverKind::Theorem;
        let k2 = k.clone();
        assert_eq!(k, k2);
        assert_ne!(k, HoverKind::Definition);
    }
    #[test]
    fn test_hover_result_no_type_no_doc_markdown() {
        let r = HoverResult::new(HoverKind::LocalVar, "x");
        let md = r.to_markdown();
        assert!(md.contains("**variable**"));
        assert!(md.contains("`x`"));
        assert!(!md.contains("```lean"));
    }
    #[test]
    fn test_hover_result_with_location_plain() {
        let r =
            HoverResult::new(HoverKind::Theorem, "foo").with_location("Mathlib/Algebra.lean:42:0");
        let plain = r.to_plain_text();
        assert!(plain.contains("Defined at: Mathlib/Algebra.lean:42:0"));
        let md = r.to_markdown();
        assert!(md.contains("Mathlib/Algebra.lean:42:0"));
    }
    #[test]
    fn test_hover_provider_register_keyword() {
        let mut p = HoverProvider::new();
        p.register_keyword("theorem", "Declares a theorem.");
        let r = p.lookup("theorem").expect("test operation should succeed");
        assert_eq!(r.name, "theorem");
        assert!(matches!(r.kind, HoverKind::Keyword));
    }
    #[test]
    fn test_hover_provider_overwrite() {
        let mut p = HoverProvider::new();
        p.register(
            "foo",
            HoverResult::new(HoverKind::Definition, "foo").with_doc("first"),
        );
        p.register(
            "foo",
            HoverResult::new(HoverKind::Theorem, "foo").with_doc("second"),
        );
        let r = p.lookup("foo").expect("test operation should succeed");
        assert_eq!(r.documentation, "second");
        assert!(matches!(r.kind, HoverKind::Theorem));
    }
    #[test]
    fn test_hover_provider_lookup_prefix_multiple() {
        let mut p = HoverProvider::new();
        p.register_tactic("simp", "simp doc");
        p.register_tactic("simp_all", "simp_all doc");
        p.register_tactic("simp_only", "simp_only doc");
        p.register_tactic("ring", "ring doc");
        let results = p.lookup_prefix("simp");
        assert_eq!(results.len(), 3);
    }
    #[test]
    fn test_hover_provider_lookup_prefix_empty_provider() {
        let p = HoverProvider::new();
        assert!(p.lookup_prefix("anything").is_empty());
    }
    #[test]
    fn test_contextual_hover_unknown_word() {
        let ch = ContextualHover::new();
        assert!(ch.hover_at_word("nonexistent_xyz", "", 0, 0).is_none());
    }
    #[test]
    fn test_contextual_hover_all_builtin_types() {
        let ch = ContextualHover::new();
        for name in &["Nat", "Int", "Bool", "List", "Option", "Prop", "Type"] {
            let r = ch.hover_at_word(name, "", 0, 0);
            assert!(r.is_some(), "Expected hover for built-in type {}", name);
        }
    }
    #[test]
    fn test_contextual_hover_all_core_tactics() {
        let ch = ContextualHover::new();
        for tactic in &["intro", "simp", "ring", "linarith", "omega", "sorry"] {
            let r = ch.hover_at_word(tactic, "", 0, 0);
            assert!(r.is_some(), "Expected hover for tactic {}", tactic);
            assert!(matches!(
                r.expect("test operation should succeed").kind,
                HoverKind::Tactic
            ));
        }
    }
    #[test]
    fn test_is_tactic_keyword_all_known() {
        let tactics = [
            "intro",
            "intros",
            "exact",
            "assumption",
            "apply",
            "refl",
            "rw",
            "simp",
            "simp_all",
            "ring",
            "linarith",
            "omega",
            "norm_num",
            "constructor",
            "left",
            "right",
            "cases",
            "induction",
            "have",
            "show",
            "by_contra",
            "by_contradiction",
            "contrapose",
            "push_neg",
            "exfalso",
            "trivial",
            "sorry",
            "split",
            "clear",
            "revert",
            "exists",
            "use",
            "obtain",
            "repeat",
            "try",
            "first",
            "all_goals",
            "field_simp",
            "norm_cast",
            "push_cast",
            "exact_mod_cast",
            "conv",
            "rename",
        ];
        for tac in &tactics {
            assert!(
                ContextualHover::is_tactic_keyword(tac),
                "{} should be tactic",
                tac
            );
        }
    }
    #[test]
    fn test_is_tactic_keyword_non_tactics() {
        let non_tactics = ["Nat", "Bool", "forall", "fun", "let", "def", "theorem", ""];
        for word in &non_tactics {
            assert!(
                !ContextualHover::is_tactic_keyword(word),
                "{} should not be tactic",
                word
            );
        }
    }
    #[test]
    fn test_is_builtin_type_all_known() {
        let types = [
            "Nat", "Int", "Bool", "String", "Char", "Float", "UInt8", "UInt16", "UInt32", "UInt64",
            "List", "Option", "Result", "Array", "Type", "Prop", "Sort", "True", "False", "And",
            "Or", "Not", "Iff", "Eq", "Ne", "Exists", "Unit", "Empty", "Prod", "Sum", "Fin",
            "Subtype",
        ];
        for ty in &types {
            assert!(
                ContextualHover::is_builtin_type(ty),
                "{} should be builtin type",
                ty
            );
        }
    }
    #[test]
    fn test_is_builtin_type_non_types() {
        let non_types = ["simp", "ring", "apply", "", "myCustomType"];
        for word in &non_types {
            assert!(
                !ContextualHover::is_builtin_type(word),
                "{} should not be builtin type",
                word
            );
        }
    }
    #[test]
    fn test_contextual_hover_default() {
        let ch = ContextualHover::default();
        let r = ch.hover_at_word("ring", "", 0, 0);
        assert!(r.is_some());
    }
    #[test]
    fn test_hover_provider_default_is_empty() {
        let p = HoverProvider::default();
        assert!(p.lookup("simp").is_none());
    }
}
#[cfg(test)]
mod extended_hover_tests {
    use super::*;
    use crate::hover_info::*;
    fn make_info(name: &str, kind: HoverKind) -> HoverInfo {
        HoverInfoBuilder::new(name, kind)
            .type_sig("Nat → Nat")
            .doc("A natural number function.")
            .build()
    }
    #[test]
    fn test_hover_location_contains() {
        let loc = HoverLocation::single_line(5, 3, 10);
        assert!(loc.contains(5, 5));
        assert!(!loc.contains(5, 2));
        assert!(!loc.contains(5, 11));
        assert!(!loc.contains(6, 5));
    }
    #[test]
    fn test_hover_index_query() {
        let mut index = HoverIndex::new();
        let loc = HoverLocation::single_line(3, 0, 5);
        let info = make_info("foo", HoverKind::Definition);
        index.insert(HoverEntry::new(loc, info));
        assert!(index.query(3, 3).is_some());
        assert!(index.query(3, 6).is_none());
        assert!(index.query(4, 0).is_none());
    }
    #[test]
    fn test_hover_cache_lru_eviction() {
        let mut cache = HoverCache::new(2);
        cache.insert(1, 0, make_info("a", HoverKind::Theorem));
        cache.insert(2, 0, make_info("b", HoverKind::Theorem));
        cache.insert(3, 0, make_info("c", HoverKind::Theorem));
        assert!(cache.get(1, 0).is_none());
        assert!(cache.get(2, 0).is_some());
        assert!(cache.get(3, 0).is_some());
    }
    #[test]
    fn test_hover_enricher_tactic() {
        let enricher = HoverEnricher::new();
        let mut info = make_info("simp", HoverKind::Tactic);
        info.doc_string = None;
        enricher.enrich(&mut info);
        assert!(info.doc_string.is_some());
        assert!(info
            .doc_string
            .expect("test operation should succeed")
            .contains("Simplifies"));
    }
    #[test]
    fn test_hover_formatter_plaintext() {
        let fmt = HoverFormatter::new(HoverFormat::PlainText);
        let info = make_info("foo", HoverKind::Definition);
        let out = fmt.format_info(&info);
        assert!(out.contains("def: foo"));
    }
    #[test]
    fn test_hover_formatter_markdown() {
        let fmt = HoverFormatter::new(HoverFormat::Markdown);
        let info = make_info("bar", HoverKind::Theorem);
        let out = fmt.format_info(&info);
        assert!(out.contains("**theorem**"));
        assert!(out.contains("`bar`"));
    }
    #[test]
    fn test_hover_diagnostic_collection() {
        let mut coll = HoverDiagnosticCollection::new();
        let loc = HoverLocation::single_line(1, 0, 5);
        coll.add(HoverDiagnostic::new(
            HoverDiagnosticSeverity::Error,
            "bad",
            loc,
        ));
        coll.add(HoverDiagnostic::new(
            HoverDiagnosticSeverity::Warning,
            "warn",
            loc,
        ));
        assert_eq!(coll.count(), 2);
        assert!(coll.has_errors());
        assert_eq!(coll.errors().len(), 1);
        assert_eq!(coll.at(1, 3).len(), 2);
    }
    #[test]
    fn test_hover_link_markdown() {
        let link = HoverLink::to_definition("foo", "src/foo.rs");
        assert_eq!(link.to_markdown(), "[foo](src/foo.rs)");
    }
    #[test]
    fn test_hover_stats() {
        let mut stats = HoverStats::new();
        stats.record_cache_hit();
        stats.record_cache_hit();
        stats.record_cache_miss();
        assert!((stats.cache_hit_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_hover_info_builder() {
        let info = HoverInfoBuilder::new("myFunc", HoverKind::Definition)
            .type_sig("Nat → Nat → Nat")
            .doc("adds two naturals")
            .module("Std.Nat")
            .build();
        assert_eq!(info.name, "myFunc");
        assert_eq!(info.module_path, Some("Std.Nat".to_string()));
    }
    #[test]
    fn test_hover_markdown_code_block() {
        let s = HoverMarkdown::render_code_block("theorem foo : 1 = 1 := rfl");
        assert!(s.starts_with("```lean\n"));
        assert!(s.ends_with("```"));
    }
}
#[allow(dead_code)]
pub trait HoverProviderTrait: Send + Sync {
    fn provide(&self, ctx: &HoverRequestContext) -> Option<HoverResponse>;
    fn priority(&self) -> i32 {
        0
    }
}
#[cfg(test)]
mod hover_extended_tests2 {
    use super::*;
    use crate::hover_info::*;
    #[test]
    fn test_type_sig_parser_split_arrows() {
        let parts = HoverTypeSignatureParser::split_arrows("Nat -> Int -> Bool");
        assert_eq!(parts, vec!["Nat", "Int", "Bool"]);
    }
    #[test]
    fn test_type_sig_parser_arity() {
        assert_eq!(HoverTypeSignatureParser::arity("Nat -> Int -> Bool"), 2);
        assert_eq!(HoverTypeSignatureParser::arity("Nat"), 0);
    }
    #[test]
    fn test_type_sig_parser_return_type() {
        assert_eq!(HoverTypeSignatureParser::return_type("Nat -> Prop"), "Prop");
    }
    #[test]
    fn test_type_sig_parser_is_prop() {
        assert!(HoverTypeSignatureParser::is_prop("Nat -> Prop"));
        assert!(!HoverTypeSignatureParser::is_prop("Nat -> Nat"));
    }
    #[test]
    fn test_hyp_info_to_hover() {
        let hyp = HoverHypothesisInfo::new("h", "n < 10");
        let info = hyp.to_hover_info();
        assert_eq!(info.kind, HoverKind::LocalVar);
        assert_eq!(info.name, "h");
    }
    #[test]
    fn test_goal_info_render() {
        let goal = HoverGoalInfo::new("P ∧ Q")
            .with_hyp(HoverHypothesisInfo::new("h1", "P"))
            .with_hyp(HoverHypothesisInfo::new("h2", "Q"))
            .with_indices(0, 2);
        let rendered = goal.render_plaintext();
        assert!(rendered.contains("Goal 1/2"));
        assert!(rendered.contains("h1 : P"));
        assert!(rendered.contains("⊢ P ∧ Q"));
    }
    #[test]
    fn test_module_info_export_count() {
        let info = HoverModuleInfo::new("Std.Nat")
            .with_export("succ")
            .with_export("zero")
            .with_export("add");
        assert_eq!(info.export_count(), 3);
        let hi = info.to_hover_info();
        assert_eq!(hi.name, "Std.Nat");
    }
    #[test]
    fn test_hover_response_builder() {
        let loc = HoverLocation::single_line(1, 0, 5);
        let ann = HoverAnnotation::new(HoverAnnotationKind::ImplicitArgument, "implicit α");
        let link = HoverLink::to_docs("docs", "https://example.com");
        let diag = HoverDiagnostic::new(HoverDiagnosticSeverity::Warning, "consider simp", loc);
        let resp = HoverResponse::new("hover content")
            .with_range(loc)
            .with_annotation(ann)
            .with_link(link)
            .with_diagnostic(diag);
        assert!(!resp.is_empty());
        assert_eq!(resp.annotation_count(), 1);
        assert!(!resp.has_errors());
    }
    #[test]
    fn test_hover_throttler() {
        let mut t = HoverThrottler::new(100);
        assert!(t.should_proceed(0));
        assert!(!t.should_proceed(50));
        assert!(t.should_proceed(100));
        assert_eq!(t.request_count(), 2);
    }
    #[test]
    fn test_hover_request_context_format() {
        let ctx = HoverRequestContext::new("file:///foo.lean", 10, 5);
        assert_eq!(ctx.preferred_format(), HoverFormat::Markdown);
        let ctx2 = ctx.without_markdown();
        assert_eq!(ctx2.preferred_format(), HoverFormat::PlainText);
    }
    #[test]
    fn test_hover_annotation_is_implicit() {
        let ann = HoverAnnotation::new(HoverAnnotationKind::InstanceArgument, "inst");
        assert!(ann.is_implicit());
        let ann2 = HoverAnnotation::new(HoverAnnotationKind::CoercedExpr, "coerce");
        assert!(!ann2.is_implicit());
    }
}
#[cfg(test)]
mod hover_extended_tests3 {
    use super::*;
    use crate::hover_info::*;
    #[test]
    fn test_tactic_suggestion_engine() {
        let engine = TacticSuggestionEngine::new();
        let suggestions = engine.suggest("a = b");
        assert!(!suggestions.is_empty());
        assert_eq!(suggestions[0].tactic, "rfl");
    }
    #[test]
    fn test_tactic_suggestion_conjunction() {
        let engine = TacticSuggestionEngine::new();
        let sugs = engine.suggest("P ∧ Q");
        let tactics: Vec<&str> = sugs.iter().map(|s| s.tactic.as_str()).collect();
        assert!(tactics.contains(&"constructor"));
    }
    #[test]
    fn test_hover_history() {
        let mut hist = HoverHistory::new(3);
        hist.push(1, 0);
        hist.push(2, 5);
        hist.push(3, 10);
        hist.push(4, 0);
        assert_eq!(hist.len(), 3);
        assert_eq!(hist.last(), Some((4, 0)));
    }
    #[test]
    fn test_hover_history_unique_lines() {
        let mut hist = HoverHistory::new(10);
        hist.push(1, 0);
        hist.push(1, 5);
        hist.push(2, 0);
        let lines = hist.unique_lines();
        assert_eq!(lines, vec![1, 2]);
    }
    #[test]
    fn test_doc_reference_full_url() {
        let r = HoverDocReference::new("simp", "https://leanprover.github.io/simp")
            .with_section("overview");
        assert_eq!(r.full_url(), "https://leanprover.github.io/simp#overview");
    }
    #[test]
    fn test_doc_reference_index() {
        let mut index = HoverDocReferenceIndex::new();
        index.insert(HoverDocReference::new("omega", "https://example.com/omega"));
        assert!(index.lookup("omega").is_some());
        assert!(index.lookup("ring").is_none());
        assert_eq!(index.len(), 1);
    }
    #[test]
    fn test_hover_renderer_config_compact() {
        let cfg = HoverRendererConfig::new().compact();
        assert!(!cfg.show_module);
        assert!(!cfg.show_source_location);
        assert_eq!(cfg.max_output_chars, 500);
    }
    #[test]
    fn test_hover_renderer_render() {
        let cfg = HoverRendererConfig::new().with_format(HoverFormat::PlainText);
        let renderer = HoverRenderer::new(cfg);
        let info = HoverInfoBuilder::new("foo", HoverKind::Definition)
            .type_sig("Nat -> Nat")
            .build();
        let out = renderer.render(&info);
        assert!(out.contains("foo"));
    }
    #[test]
    fn test_performance_monitor_stats() {
        let mut mon = HoverPerformanceMonitor::new(100);
        for i in 1u64..=10 {
            mon.record(i * 100);
        }
        assert_eq!(mon.sample_count(), 10);
        assert_eq!(mon.min_us(), 100);
        assert_eq!(mon.max_us(), 1000);
        assert!((mon.mean_us() - 550.0).abs() < 1.0);
    }
    #[test]
    fn test_region_merger() {
        let make = |name: &str| -> HoverEntry {
            let loc = HoverLocation::single_line(1, 0, 5);
            let info = HoverInfoBuilder::new(name, HoverKind::Definition).build();
            HoverEntry::new(loc, info)
        };
        let entries = vec![make("foo"), make("foo"), make("bar")];
        let merged = HoverRegionMerger::merge(entries);
        assert_eq!(merged.len(), 2);
    }
}
