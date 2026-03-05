//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::basic::{MVarId, MetaContext, MetavarKind};
use crate::tactic::state::{TacticError, TacticResult, TacticState};
use oxilean_kernel::{Expr, Level, Name};

use super::types::{
    CacheLookup, LemmaCandidate, LemmaEntry, LemmaIndex, LibrarySearchConfig, ScoredEntry,
    ScoringCriteria, SearchCache, SearchResult, SearchState, SimpleLemmaIndex, TypeSignature,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tactic::library_search::*;
    use oxilean_kernel::{
        AxiomVal, BinderInfo, ConstantInfo, ConstantVal, Declaration, Environment, Level,
        ReducibilityHint,
    };
    fn mk_env() -> Environment {
        Environment::new()
    }
    fn mk_ctx() -> MetaContext {
        MetaContext::new(mk_env())
    }
    fn mk_nat() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    fn mk_bool() -> Expr {
        Expr::Const(Name::str("Bool"), vec![])
    }
    fn mk_prop() -> Expr {
        Expr::Sort(Level::zero())
    }
    fn mk_true() -> Expr {
        Expr::Const(Name::str("True"), vec![])
    }
    fn mk_eq(alpha: Expr, lhs: Expr, rhs: Expr) -> Expr {
        Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Eq"), vec![Level::zero()])),
                    Box::new(alpha),
                )),
                Box::new(lhs),
            )),
            Box::new(rhs),
        )
    }
    fn mk_arrow(domain: Expr, codomain: Expr) -> Expr {
        Expr::Pi(
            BinderInfo::Default,
            Name::Anonymous,
            Box::new(domain),
            Box::new(codomain),
        )
    }
    #[test]
    fn test_default_config() {
        let config = LibrarySearchConfig::default();
        assert_eq!(config.max_candidates, 256);
        assert_eq!(config.max_depth, 4);
        assert_eq!(config.timeout_ms, 5000);
        assert!(config.include_local);
        assert!(!config.suggest_only);
        assert!(!config.allow_subgoals);
    }
    #[test]
    fn test_exact_mode_config() {
        let config = LibrarySearchConfig::exact_mode();
        assert!(!config.allow_subgoals);
        assert_eq!(config.max_remaining_goals, 0);
    }
    #[test]
    fn test_apply_mode_config() {
        let config = LibrarySearchConfig::apply_mode();
        assert!(config.allow_subgoals);
        assert_eq!(config.max_remaining_goals, 5);
        assert_eq!(config.max_candidates, 512);
    }
    #[test]
    fn test_scoring_criteria_default() {
        let criteria = ScoringCriteria::default();
        assert_eq!(criteria.specificity, 0.0);
        assert_eq!(criteria.remaining_goals, 0);
        assert_eq!(criteria.edit_distance, 0);
        assert!(!criteria.is_local);
    }
    #[test]
    fn test_score_computation() {
        let config = LibrarySearchConfig::default();
        let c1 = ScoringCriteria {
            specificity: 1.0,
            remaining_goals: 0,
            edit_distance: 0,
            is_local: false,
            num_universe_params: 0,
            num_synth_args: 0,
            total_args: 0,
        };
        let s1 = c1.score(&config);
        let c2 = ScoringCriteria {
            specificity: 0.5,
            remaining_goals: 2,
            edit_distance: 1,
            is_local: false,
            num_universe_params: 0,
            num_synth_args: 0,
            total_args: 0,
        };
        let s2 = c2.score(&config);
        assert!(
            s1 > s2,
            "higher specificity and fewer goals should score higher"
        );
    }
    #[test]
    fn test_local_bonus() {
        let config = LibrarySearchConfig::default();
        let local_criteria = ScoringCriteria {
            specificity: 0.5,
            is_local: true,
            ..ScoringCriteria::default()
        };
        let non_local_criteria = ScoringCriteria {
            specificity: 0.5,
            is_local: false,
            ..ScoringCriteria::default()
        };
        assert!(local_criteria.score(&config) > non_local_criteria.score(&config));
    }
    #[test]
    fn test_synth_bonus() {
        let config = LibrarySearchConfig::default();
        let full_synth = ScoringCriteria {
            num_synth_args: 3,
            total_args: 3,
            ..ScoringCriteria::default()
        };
        let no_synth = ScoringCriteria {
            num_synth_args: 0,
            total_args: 3,
            ..ScoringCriteria::default()
        };
        assert!(full_synth.score(&config) > no_synth.score(&config));
    }
    #[test]
    fn test_lemma_candidate_new() {
        let c = LemmaCandidate::new(Name::str("foo"), mk_nat());
        assert_eq!(c.name, Name::str("foo"));
        assert!(c.score >= 0.0, "score should be non-negative");
        assert!(c.applied_args.is_empty());
        assert_eq!(c.remaining_goals, 0);
        assert!(c.proof.is_none());
        assert!(!c.suggestion.is_empty());
    }
    #[test]
    fn test_search_result_is_found() {
        let found = SearchResult::Found(mk_nat(), "exact Nat".into());
        assert!(found.is_found());
        assert!(!found.is_timed_out());
        let not_found = SearchResult::NotFound;
        assert!(!not_found.is_found());
        let timed = SearchResult::TimedOut;
        assert!(timed.is_timed_out());
        assert!(!timed.is_found());
    }
    #[test]
    fn test_lemma_index_new() {
        let index = LemmaIndex::new();
        assert!(index.is_empty());
        assert_eq!(index.len(), 0);
    }
    #[test]
    fn test_lemma_index_insert_and_lookup() {
        let mut index = LemmaIndex::new();
        let ty = mk_arrow(mk_nat(), mk_bool());
        index.insert(Name::str("f"), ty, 0, false);
        assert_eq!(index.len(), 1);
        let results = index.lookup(&mk_bool());
        assert!(!results.is_empty());
    }
    #[test]
    fn test_lemma_index_no_duplicates() {
        let mut index = LemmaIndex::new();
        let ty = mk_nat();
        index.insert(Name::str("f"), ty.clone(), 0, false);
        index.insert(Name::str("f"), ty, 0, false);
        assert_eq!(index.len(), 1);
    }
    #[test]
    fn test_lemma_index_clear() {
        let mut index = LemmaIndex::new();
        index.insert(Name::str("a"), mk_nat(), 0, false);
        index.insert(Name::str("b"), mk_bool(), 0, false);
        assert_eq!(index.len(), 2);
        index.clear();
        assert!(index.is_empty());
    }
    #[test]
    fn test_lemma_index_all_entries() {
        let mut index = LemmaIndex::new();
        index.insert(Name::str("a"), mk_nat(), 0, false);
        index.insert(Name::str("b"), mk_bool(), 0, true);
        let all = index.all_entries();
        assert_eq!(all.len(), 2);
    }
    #[test]
    fn test_lemma_index_from_empty_env() {
        let ctx = mk_ctx();
        let index = LemmaIndex::from_environment(&ctx);
        assert!(index.is_empty());
    }
    #[test]
    fn test_strip_leading_pis_no_pi() {
        let e = mk_nat();
        let result = strip_leading_pis(&e);
        assert_eq!(result, mk_nat());
    }
    #[test]
    fn test_strip_leading_pis_one_pi() {
        let e = mk_arrow(mk_nat(), mk_bool());
        let result = strip_leading_pis(&e);
        assert_eq!(result, mk_bool());
    }
    #[test]
    fn test_strip_leading_pis_nested() {
        let e = mk_arrow(mk_nat(), mk_arrow(mk_bool(), mk_prop()));
        let result = strip_leading_pis(&e);
        assert_eq!(result, mk_prop());
    }
    #[test]
    fn test_count_leading_pis() {
        assert_eq!(count_leading_pis(&mk_nat()), 0);
        assert_eq!(count_leading_pis(&mk_arrow(mk_nat(), mk_bool())), 1);
        assert_eq!(
            count_leading_pis(&mk_arrow(mk_nat(), mk_arrow(mk_bool(), mk_prop()))),
            2
        );
    }
    #[test]
    fn test_specificity_const() {
        let spec = compute_specificity(&mk_nat());
        assert!(spec > 0.0);
    }
    #[test]
    fn test_specificity_bvar() {
        let spec = compute_specificity(&Expr::BVar(0));
        assert_eq!(spec, 0.0);
    }
    #[test]
    fn test_edit_distance_same() {
        let d = compute_edit_distance(&mk_nat(), &mk_nat());
        assert_eq!(d, 0);
    }
    #[test]
    fn test_edit_distance_different() {
        let d = compute_edit_distance(&mk_nat(), &mk_bool());
        assert!(d > 0);
    }
    #[test]
    fn test_edit_distance_app() {
        let e1 = Expr::App(Box::new(mk_nat()), Box::new(mk_bool()));
        let e2 = Expr::App(Box::new(mk_nat()), Box::new(mk_nat()));
        let d = compute_edit_distance(&e1, &e2);
        assert!(d > 0);
    }
    #[test]
    fn test_substitute_bvar0_hit() {
        let body = Expr::BVar(0);
        let repl = mk_nat();
        assert_eq!(substitute_bvar0(&body, &repl), mk_nat());
    }
    #[test]
    fn test_substitute_bvar0_miss() {
        let body = Expr::BVar(1);
        let repl = mk_nat();
        assert_eq!(substitute_bvar0(&body, &repl), Expr::BVar(0));
    }
    #[test]
    fn test_substitute_bvar0_in_app() {
        let body = Expr::App(Box::new(Expr::BVar(0)), Box::new(Expr::BVar(1)));
        let repl = mk_nat();
        let result = substitute_bvar0(&body, &repl);
        assert_eq!(
            result,
            Expr::App(Box::new(mk_nat()), Box::new(Expr::BVar(0)))
        );
    }
    #[test]
    fn test_substitute_bvar0_in_lam() {
        let body = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::BVar(0)),
            Box::new(Expr::BVar(1)),
        );
        let repl = mk_nat();
        let result = substitute_bvar0(&body, &repl);
        assert!(matches!(result, Expr::Lam(_, _, _, _)));
    }
    #[test]
    fn test_substitute_bvars_empty() {
        let e = Expr::BVar(0);
        let result = substitute_bvars_with_exprs(&e, &[]);
        assert_eq!(result, Expr::BVar(0));
    }
    #[test]
    fn test_freshen_no_params() {
        let mut ctx = mk_ctx();
        let e = mk_nat();
        let result = freshen_universe_params(&e, 0, &mut ctx);
        assert_eq!(result, mk_nat());
    }
    #[test]
    fn test_freshen_with_params() {
        let mut ctx = mk_ctx();
        let e = Expr::Sort(Level::Param(Name::str("u")));
        let result = freshen_universe_params(&e, 1, &mut ctx);
        assert!(matches!(result, Expr::Sort(_)));
        assert!(!matches!(result, Expr::Sort(Level::Param(_))));
    }
    #[test]
    fn test_open_pis_no_pi() {
        let mut ctx = mk_ctx();
        let ty = mk_nat();
        let (applied, mvars, conclusion) = open_pis_as_mvars(&Name::str("c"), &ty, &mut ctx, 8);
        assert!(mvars.is_empty());
        assert_eq!(conclusion, mk_nat());
        assert!(matches!(applied, Expr::Const(_, _)));
    }
    #[test]
    fn test_open_pis_one_pi() {
        let mut ctx = mk_ctx();
        let ty = mk_arrow(mk_nat(), mk_bool());
        let (applied, mvars, conclusion) = open_pis_as_mvars(&Name::str("f"), &ty, &mut ctx, 8);
        assert_eq!(mvars.len(), 1);
        assert_eq!(conclusion, mk_bool());
        assert!(matches!(applied, Expr::App(_, _)));
    }
    #[test]
    fn test_open_pis_max_limit() {
        let mut ctx = mk_ctx();
        let ty = mk_arrow(mk_nat(), mk_arrow(mk_nat(), mk_arrow(mk_nat(), mk_bool())));
        let (_applied, mvars, _conclusion) = open_pis_as_mvars(&Name::str("g"), &ty, &mut ctx, 2);
        assert_eq!(mvars.len(), 2);
    }
    #[test]
    fn test_synth_trivial_true() {
        let result = try_synth_trivial(&mk_true());
        assert!(result.is_some());
    }
    #[test]
    fn test_synth_trivial_refl() {
        let eq = mk_eq(mk_nat(), mk_nat(), mk_nat());
        let result = try_synth_trivial(&eq);
        assert!(result.is_some());
    }
    #[test]
    fn test_synth_trivial_non_trivial() {
        let result = try_synth_trivial(&mk_nat());
        assert!(result.is_none());
    }
    #[test]
    fn test_try_refl_equal() {
        let eq = mk_eq(mk_nat(), mk_bool(), mk_bool());
        let result = try_refl_proof(&eq);
        assert!(result.is_some());
    }
    #[test]
    fn test_try_refl_not_equal() {
        let eq = mk_eq(mk_nat(), mk_nat(), mk_bool());
        let result = try_refl_proof(&eq);
        assert!(result.is_none());
    }
    #[test]
    fn test_format_const() {
        assert_eq!(format_expr_short(&mk_nat()), "Nat");
    }
    #[test]
    fn test_format_bvar() {
        assert_eq!(format_expr_short(&Expr::BVar(0)), "?_0");
    }
    #[test]
    fn test_format_lit() {
        assert_eq!(
            format_expr_short(&Expr::Lit(oxilean_kernel::Literal::Nat(42))),
            "42"
        );
    }
    #[test]
    fn test_format_app() {
        let e = Expr::App(Box::new(mk_nat()), Box::new(mk_bool()));
        assert_eq!(format_expr_short(&e), "(Nat Bool)");
    }
    #[test]
    fn test_format_lam() {
        let e = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(mk_nat()),
            Box::new(Expr::BVar(0)),
        );
        assert_eq!(format_expr_short(&e), "(fun x => ...)");
    }
    #[test]
    fn test_format_pi() {
        let e = mk_arrow(mk_nat(), mk_bool());
        assert_eq!(format_expr_short(&e), "(_ -> _)");
    }
    #[test]
    fn test_suggestion_exact_no_args() {
        let s = build_suggestion(&Name::str("Nat.zero"), &[], true);
        assert_eq!(s, "exact Nat.zero");
    }
    #[test]
    fn test_suggestion_apply_with_args() {
        let args = vec![mk_nat(), mk_bool()];
        let s = build_suggestion(&Name::str("foo"), &args, false);
        assert!(s.starts_with("apply @foo"));
        assert!(s.contains("Nat"));
        assert!(s.contains("Bool"));
    }
    #[test]
    fn test_search_cache_new() {
        let cache = SearchCache::new();
        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);
    }
    #[test]
    fn test_search_cache_failure() {
        let mut cache = SearchCache::new();
        cache.record_failure(42);
        assert!(!cache.is_empty());
        let r = cache.lookup(42);
        assert!(matches!(r, Some(CacheLookup::Failed)));
    }
    #[test]
    fn test_search_cache_success() {
        let mut cache = SearchCache::new();
        let candidates = vec![LemmaCandidate::new(Name::str("a"), mk_nat())];
        cache.record_success(99, candidates);
        let r = cache.lookup(99);
        assert!(matches!(r, Some(CacheLookup::Found(_))));
    }
    #[test]
    fn test_search_cache_miss() {
        let cache = SearchCache::new();
        assert!(cache.lookup(0).is_none());
    }
    #[test]
    fn test_search_cache_clear() {
        let mut cache = SearchCache::new();
        cache.record_failure(1);
        cache.record_failure(2);
        cache.clear();
        assert!(cache.is_empty());
    }
    #[test]
    fn test_hash_expr_deterministic() {
        let e = mk_nat();
        assert_eq!(hash_expr(&e), hash_expr(&e));
    }
    #[test]
    fn test_hash_expr_different() {
        assert_ne!(hash_expr(&mk_nat()), hash_expr(&mk_bool()));
    }
    #[test]
    fn test_expr_size_atom() {
        assert_eq!(expr_size(&mk_nat()), 1);
    }
    #[test]
    fn test_expr_size_app() {
        let e = Expr::App(Box::new(mk_nat()), Box::new(mk_bool()));
        assert_eq!(expr_size(&e), 3);
    }
    #[test]
    fn test_expr_size_pi() {
        let e = mk_arrow(mk_nat(), mk_bool());
        assert_eq!(expr_size(&e), 3);
    }
    #[test]
    fn test_expr_depth_atom() {
        assert_eq!(expr_depth(&mk_nat()), 0);
    }
    #[test]
    fn test_expr_depth_app() {
        let e = Expr::App(Box::new(mk_nat()), Box::new(mk_bool()));
        assert_eq!(expr_depth(&e), 1);
    }
    #[test]
    fn test_name_last_component() {
        assert_eq!(name_last_component(&Name::str("foo")), "foo");
        assert_eq!(
            name_last_component(&Name::str("Nat").append_str("add")),
            "add"
        );
        assert_eq!(name_last_component(&Name::Anonymous), "_");
    }
    #[test]
    fn test_names_are_siblings() {
        let a = Name::str("Nat").append_str("add");
        let b = Name::str("Nat").append_str("mul");
        assert!(names_are_siblings(&a, &b));
        let c = Name::str("Int").append_str("add");
        assert!(!names_are_siblings(&a, &c));
    }
    #[test]
    fn test_name_parent() {
        let n = Name::str("Nat").append_str("add");
        assert_eq!(name_parent(&n), &Name::str("Nat"));
        assert_eq!(name_parent(&Name::Anonymous), &Name::Anonymous);
    }
    #[test]
    fn test_is_search_candidate_axiom() {
        use oxilean_kernel::declaration::{AxiomVal, ConstantVal};
        use oxilean_kernel::ConstantInfo;
        let ci = ConstantInfo::Axiom(AxiomVal {
            common: ConstantVal {
                name: Name::str("my_axiom"),
                level_params: vec![],
                ty: mk_prop(),
            },
            is_unsafe: false,
        });
        assert!(is_search_candidate(&Name::str("my_axiom"), &ci));
    }
    #[test]
    fn test_is_search_candidate_skip_rec() {
        let ci = ConstantInfo::Axiom(AxiomVal {
            common: ConstantVal {
                name: Name::str("Nat.rec"),
                level_params: vec![],
                ty: mk_prop(),
            },
            is_unsafe: false,
        });
        assert!(!is_search_candidate(&Name::str("Nat.rec"), &ci));
    }
    #[test]
    fn test_is_search_candidate_skip_internal() {
        let ci = ConstantInfo::Axiom(AxiomVal {
            common: ConstantVal {
                name: Name::str("_private"),
                level_params: vec![],
                ty: mk_prop(),
            },
            is_unsafe: false,
        });
        assert!(!is_search_candidate(&Name::str("_private"), &ci));
    }
    #[test]
    fn test_is_proposition_shaped_sort0() {
        assert!(is_proposition_shaped(&mk_prop()));
    }
    #[test]
    fn test_is_proposition_shaped_eq() {
        let eq = mk_eq(mk_nat(), mk_nat(), mk_nat());
        assert!(is_proposition_shaped(&eq));
    }
    #[test]
    fn test_is_proposition_shaped_nat() {
        assert!(!is_proposition_shaped(&mk_nat()));
    }
    #[test]
    fn test_collect_constants_simple() {
        let e = mk_nat();
        let cs = collect_constants(&e);
        assert_eq!(cs, vec![Name::str("Nat")]);
    }
    #[test]
    fn test_collect_constants_app() {
        let e = Expr::App(Box::new(mk_nat()), Box::new(mk_bool()));
        let cs = collect_constants(&e);
        assert!(cs.contains(&Name::str("Nat")));
        assert!(cs.contains(&Name::str("Bool")));
    }
    #[test]
    fn test_collect_constants_no_dup() {
        let e = Expr::App(Box::new(mk_nat()), Box::new(mk_nat()));
        let cs = collect_constants(&e);
        assert_eq!(cs.len(), 1);
    }
    #[test]
    fn test_collect_fvar_ids_none() {
        let e = mk_nat();
        assert!(collect_fvar_ids(&e).is_empty());
    }
    #[test]
    fn test_collect_fvar_ids_some() {
        use oxilean_kernel::FVarId;
        let e = Expr::App(
            Box::new(Expr::FVar(FVarId::new(10))),
            Box::new(Expr::FVar(FVarId::new(20))),
        );
        let ids = collect_fvar_ids(&e);
        assert_eq!(ids.len(), 2);
        assert!(ids.contains(&10));
        assert!(ids.contains(&20));
    }
    #[test]
    fn test_decompose_goal_const() {
        let binding = mk_nat();
        let (head, args) = decompose_goal(&binding);
        assert_eq!(head, Some(Name::str("Nat")));
        assert!(args.is_empty());
    }
    #[test]
    fn test_decompose_goal_app() {
        let e = Expr::App(Box::new(mk_nat()), Box::new(mk_bool()));
        let (head, args) = decompose_goal(&e);
        assert_eq!(head, Some(Name::str("Nat")));
        assert_eq!(args.len(), 1);
    }
    #[test]
    fn test_search_state_budget() {
        let config = LibrarySearchConfig {
            max_candidates: 2,
            timeout_ms: 0,
            ..LibrarySearchConfig::default()
        };
        let mut ss = SearchState::new(config);
        assert!(!ss.is_budget_exhausted());
        ss.candidates_tried = 2;
        assert!(ss.is_budget_exhausted());
    }
    #[test]
    fn test_search_state_failed_cache() {
        let config = LibrarySearchConfig::default();
        let mut ss = SearchState::new(config);
        let name = Name::str("foo");
        assert!(!ss.already_failed(&name));
        ss.mark_failed(&name);
        assert!(ss.already_failed(&name));
    }
    #[test]
    fn test_search_state_record_result() {
        let config = LibrarySearchConfig {
            max_results: 2,
            ..LibrarySearchConfig::default()
        };
        let mut ss = SearchState::new(config);
        let c1 = LemmaCandidate {
            score: 1.0,
            ..LemmaCandidate::new(Name::str("a"), mk_nat())
        };
        let c2 = LemmaCandidate {
            score: 3.0,
            ..LemmaCandidate::new(Name::str("b"), mk_nat())
        };
        let c3 = LemmaCandidate {
            score: 2.0,
            ..LemmaCandidate::new(Name::str("c"), mk_nat())
        };
        ss.record_result(c1);
        ss.record_result(c2);
        ss.record_result(c3);
        assert_eq!(ss.results.len(), 2);
        assert_eq!(ss.results[0].name, Name::str("b"));
        assert_eq!(ss.results[1].name, Name::str("c"));
    }
    #[test]
    fn test_library_search_no_goals() {
        let mut ctx = mk_ctx();
        let mut state = TacticState::new(vec![]);
        let result = tac_library_search(&mut state, &mut ctx);
        assert!(result.is_err());
    }
    #[test]
    fn test_library_search_empty_env() {
        let mut ctx = mk_ctx();
        let goal_ty = mk_nat();
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let result = tac_library_search(&mut state, &mut ctx);
        assert!(result.is_err());
    }
    #[test]
    fn test_exact_question_empty_env() {
        let mut ctx = mk_ctx();
        let goal_ty = mk_nat();
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let result = tac_exact_question(&mut state, &mut ctx);
        assert!(result.is_err());
    }
    #[test]
    fn test_scored_entry_ordering() {
        let entry = LemmaEntry {
            name: Name::str("x"),
            ty: mk_nat(),
            specificity: 1.0,
            num_univ_params: 0,
            is_local: false,
        };
        let a = ScoredEntry {
            entry: entry.clone(),
            priority: 5.0,
            depth: 0,
        };
        let b = ScoredEntry {
            entry,
            priority: 3.0,
            depth: 0,
        };
        assert!(a < b);
    }
    #[test]
    fn test_filter_index() {
        let mut index = LemmaIndex::new();
        index.insert(Name::str("keep_me"), mk_nat(), 0, false);
        index.insert(Name::str("drop_me"), mk_bool(), 0, false);
        let filtered = filter_index(&index, |n| format!("{}", n).starts_with("keep"));
        assert_eq!(filtered.len(), 1);
    }
    #[test]
    fn test_batch_search_empty() {
        let mut ctx = mk_ctx();
        let index = LemmaIndex::new();
        let config = LibrarySearchConfig::default();
        let results = batch_search(&[mk_nat(), mk_bool()], &index, &mut ctx, &config);
        assert_eq!(results.len(), 2);
        for r in &results {
            assert!(matches!(r, SearchResult::NotFound));
        }
    }
    #[test]
    fn test_find_lemmas_about_empty() {
        let index = LemmaIndex::new();
        let results = find_lemmas_about(&Name::str("Nat"), &index);
        assert!(results.is_empty());
    }
    #[test]
    fn test_match_conclusion_same() {
        let mut ctx = mk_ctx();
        let goal = mk_nat();
        let conclusion = mk_nat();
        let result = match_conclusion(&conclusion, &goal, &mut ctx);
        assert!(result.is_some());
    }
    #[test]
    fn test_match_conclusion_different() {
        let mut ctx = mk_ctx();
        let goal = mk_nat();
        let conclusion = mk_bool();
        let result = match_conclusion(&conclusion, &goal, &mut ctx);
        assert!(result.is_none());
    }
    #[test]
    fn test_library_search_finds_exact_match() {
        use oxilean_kernel::env::Declaration;
        use oxilean_kernel::reduce::ReducibilityHint;
        let mut env = mk_env();
        let _ = env.add(Declaration::Definition {
            name: Name::str("my_nat_val"),
            univ_params: vec![],
            ty: mk_nat(),
            val: Expr::Const(Name::str("Nat.zero"), vec![]),
            hint: ReducibilityHint::Regular(100),
        });
        let mut ctx = MetaContext::new(env);
        let goal_ty = mk_nat();
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let result = tac_library_search(&mut state, &mut ctx);
        assert!(result.is_ok(), "Expected search to succeed: {:?}", result);
        assert!(state.is_done());
    }
    #[test]
    fn test_library_search_suggest_only() {
        let mut env = mk_env();
        let _ = env.add(Declaration::Definition {
            name: Name::str("my_bool_val"),
            univ_params: vec![],
            ty: mk_bool(),
            val: Expr::Const(Name::str("Bool.true"), vec![]),
            hint: ReducibilityHint::Regular(100),
        });
        let mut ctx = MetaContext::new(env);
        let goal_ty = mk_bool();
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let config = LibrarySearchConfig {
            suggest_only: true,
            ..LibrarySearchConfig::default()
        };
        let result = tac_library_search_with_config(&mut state, &mut ctx, config);
        assert!(result.is_err());
        let err_msg = format!("{}", result.unwrap_err());
        assert!(err_msg.contains("Try this"));
    }
    #[test]
    fn test_library_search_with_arrow_type() {
        let mut env = mk_env();
        let f_ty = mk_arrow(mk_nat(), mk_bool());
        let _ = env.add(Declaration::Definition {
            name: Name::str("f"),
            univ_params: vec![],
            ty: f_ty,
            val: Expr::Lam(
                BinderInfo::Default,
                Name::str("n"),
                Box::new(mk_nat()),
                Box::new(Expr::Const(Name::str("Bool.true"), vec![])),
            ),
            hint: ReducibilityHint::Regular(100),
        });
        let mut ctx = MetaContext::new(env);
        let goal_ty = mk_bool();
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let config = LibrarySearchConfig::apply_mode();
        let result = tac_library_search_with_config(&mut state, &mut ctx, config);
        let _ = result;
    }
    #[test]
    fn test_empty_args_suggestion() {
        let s = build_suggestion(&Name::str("rfl"), &[], true);
        assert_eq!(s, "exact rfl");
    }
    #[test]
    fn test_edit_distance_deep() {
        let deep = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::App(Box::new(mk_nat()), Box::new(mk_nat()))),
                Box::new(mk_nat()),
            )),
            Box::new(mk_nat()),
        );
        let d = compute_edit_distance(&deep, &mk_nat());
        assert!(d > 0);
    }
    #[test]
    fn test_specificity_app() {
        let e = Expr::App(
            Box::new(Expr::Const(Name::str("List"), vec![])),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
        );
        let spec = compute_specificity(&e);
        assert!(spec > 0.5);
    }
    #[test]
    fn test_format_proj() {
        let e = Expr::Proj(Name::str("Prod"), 0, Box::new(mk_nat()));
        let s = format_expr_short(&e);
        assert_eq!(s, "Prod.0");
    }
    #[test]
    fn test_format_let() {
        let e = Expr::Let(
            Name::str("x"),
            Box::new(mk_nat()),
            Box::new(mk_nat()),
            Box::new(Expr::BVar(0)),
        );
        let s = format_expr_short(&e);
        assert_eq!(s, "(let x := ...)");
    }
    #[test]
    fn test_format_sort_prop() {
        assert_eq!(format_expr_short(&mk_prop()), "Prop");
    }
    #[test]
    fn test_format_sort_type() {
        let e = Expr::Sort(Level::succ(Level::zero()));
        assert_eq!(format_expr_short(&e), "Sort _");
    }
    #[test]
    fn test_type_signature_from_str_simple() {
        let sig = TypeSignature::parse_type("Nat");
        assert_eq!(sig.head, "Nat");
        assert!(sig.args.is_empty());
        assert!(!sig.is_prop);
    }
    #[test]
    fn test_type_signature_from_str_applied() {
        let sig = TypeSignature::parse_type("List Nat");
        assert_eq!(sig.head, "List");
        assert_eq!(sig.args.len(), 1);
        assert_eq!(sig.args[0].head, "Nat");
    }
    #[test]
    fn test_type_signature_from_str_arrow() {
        let sig = TypeSignature::parse_type("Nat -> Bool");
        assert_eq!(sig.head, "->");
        assert_eq!(sig.args.len(), 2);
        assert_eq!(sig.args[0].head, "Nat");
        assert_eq!(sig.args[1].head, "Bool");
    }
    #[test]
    fn test_type_signature_matches_wildcard() {
        let wildcard = TypeSignature::parse_type("_");
        let nat_sig = TypeSignature::parse_type("Nat");
        assert!(wildcard.matches(&nat_sig));
    }
    #[test]
    fn test_type_signature_matches_exact() {
        let nat = TypeSignature::parse_type("Nat");
        let nat2 = TypeSignature::parse_type("Nat");
        let bool_sig = TypeSignature::parse_type("Bool");
        assert!(nat.matches(&nat2));
        assert!(!nat.matches(&bool_sig));
    }
    #[test]
    fn test_simple_lemma_index_add_and_search() {
        let mut idx = SimpleLemmaIndex::new();
        idx.add_lemma("Nat.add_comm", "Eq Nat _ _");
        idx.add_lemma("List.nil_append", "Eq List _ _");
        let query = TypeSignature::parse_type("Eq Nat _ _");
        let results = idx.search_by_type(&query);
        assert!(results.contains(&"Nat.add_comm"));
        assert!(!results.contains(&"List.nil_append"));
    }
    #[test]
    fn test_simple_lemma_index_search_by_name() {
        let mut idx = SimpleLemmaIndex::new();
        idx.add_lemma("Nat.add_comm", "Eq Nat _ _");
        idx.add_lemma("Nat.mul_comm", "Eq Nat _ _");
        idx.add_lemma("List.append_nil", "Eq List _ _");
        let results = idx.search_by_name("Nat.");
        assert_eq!(results.len(), 2);
        assert!(results.contains(&"Nat.add_comm"));
        assert!(results.contains(&"Nat.mul_comm"));
    }
    #[test]
    fn test_run_library_search_advanced() {
        let results = run_library_search_advanced("Eq Nat _ _");
        assert!(!results.is_empty());
        for r in &results {
            assert!(!r.is_empty());
        }
    }
}
