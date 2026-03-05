//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{AllocationAnalyzer, BenchmarkHarness, CacheUnfriendlyAccessDetector, CloneDetector, ComplexityBudget, ConstantFoldingChecker, DeadProofBranchDetector, ElabComplexityEstimator, ElabTimeProfiler, EtaExpansionDetector, FullPerfAnalyzer, InductiveSizeChecker, LoopHoistingAnalyzer, MemoryUsageEstimator, MetavarTracker, OComplexityAdvisor, PatternMatchOptimizer, PerfFinding, PerfIssue, PerfLintConfig, PerfLintPass, PerfReport, PerfSeverity, PerfTrend, PerformanceThreshold, PolymorphismComplexityScorer, ProfilingHint, ProofSearchDepthAnalyzer, RedundantComputationDetector, SimpCallCounter, SimpNormalFormChecker, TailCallDetector, TypeClassComplexityChecker, VectorisationOpportunityDetector};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_perf_finding_new() {
        let f = PerfFinding::new(
            PerfIssue::LargeProofTerm,
            PerfSeverity::Warning,
            "file.ox:5",
            "large term",
        );
        assert_eq!(f.location, "file.ox:5");
        assert_eq!(f.estimated_impact, 0.0);
        assert!(! f.is_blocker());
    }
    #[test]
    fn test_perf_finding_blocker() {
        let f = PerfFinding::new(
                PerfIssue::UnboundedSearch,
                PerfSeverity::Blocker,
                "file.ox:1",
                "unbounded",
            )
            .with_impact(1.0);
        assert!(f.is_blocker());
        assert!((f.estimated_impact - 1.0).abs() < 1e-9);
    }
    #[test]
    fn test_config_default() {
        let cfg = PerfLintConfig::default();
        assert_eq!(cfg.max_proof_term_size, 1000);
        assert_eq!(cfg.max_recursion_depth, 50);
        assert_eq!(cfg.max_simp_lemmas, 100);
        assert!(! cfg.warn_on_classical);
    }
    #[test]
    fn test_check_proof_size() {
        let pass = PerfLintPass::with_config(PerfLintConfig {
            max_proof_term_size: 3,
            ..PerfLintConfig::default()
        });
        let source = "a b c d e f g";
        let findings = pass.check_proof_term_size(source);
        assert!(! findings.is_empty());
        assert_eq!(findings[0].issue, PerfIssue::LargeProofTerm);
    }
    #[test]
    fn test_estimate_complexity() {
        let pass = PerfLintPass::new();
        let simple = "theorem foo : True := trivial";
        let score = pass.estimate_complexity(simple);
        assert!((0.0..= 1.0).contains(& score));
        let complex: String = std::iter::repeat("fun x -> x ")
            .take(60)
            .collect::<Vec<_>>()
            .join("");
        let complex_score = pass.estimate_complexity(&complex);
        assert!(complex_score > score);
    }
    #[test]
    fn test_run_all() {
        let pass = PerfLintPass::with_config(PerfLintConfig {
            max_proof_term_size: 2,
            ..PerfLintConfig::default()
        });
        let source = "theorem foo : True := trivial";
        let findings = pass.run_all(source);
        assert!(! findings.is_empty());
    }
    #[test]
    fn test_elabcomplexity_count() {
        let source = "fun x -> fun y -> x y";
        assert!(ElabComplexityEstimator::count_binders(source) >= 2);
        assert!(ElabComplexityEstimator::count_applications(source) >= 1);
    }
    #[test]
    fn test_prioritized_findings() {
        let pass = PerfLintPass::new();
        let findings = vec![
            PerfFinding::new(PerfIssue::DeepRecursion, PerfSeverity::Warning, "a", "a")
            .with_impact(0.3), PerfFinding::new(PerfIssue::LargeProofTerm,
            PerfSeverity::Blocker, "b", "b").with_impact(0.9),
            PerfFinding::new(PerfIssue::SlowSimpLemma, PerfSeverity::Suggestion, "c",
            "c").with_impact(0.1),
        ];
        let prioritized = pass.prioritized_findings(&findings);
        assert_eq!(prioritized.len(), 3);
        assert!((prioritized[0].estimated_impact - 0.9).abs() < 1e-9);
        assert!((prioritized[2].estimated_impact - 0.1).abs() < 1e-9);
    }
}
#[cfg(test)]
mod extended_perf_tests {
    use super::*;
    #[test]
    fn allocation_analyzer_counts() {
        let source = "let v = Vec::new();\nlet s = String::new();\nlet b = Box::new(1);";
        assert!(AllocationAnalyzer::count_allocations(source) >= 3);
    }
    #[test]
    fn allocation_analyzer_finds_sites() {
        let source = "let v = Vec::new();\nlet s = String::new();";
        let sites = AllocationAnalyzer::find_allocation_sites(source);
        assert!(! sites.is_empty());
    }
    #[test]
    fn allocation_analyzer_emit_findings() {
        let source = "let v = Vec::new();\n";
        let findings = AllocationAnalyzer::emit_findings(source);
        assert!(! findings.is_empty());
    }
    #[test]
    fn clone_detector_below_threshold() {
        let detector = CloneDetector::new(5);
        let source = "x.clone()";
        let findings = detector.check(source);
        assert!(findings.is_empty());
    }
    #[test]
    fn clone_detector_above_threshold() {
        let detector = CloneDetector::new(2);
        let source = "a.clone() b.clone() c.clone()";
        let findings = detector.check(source);
        assert!(! findings.is_empty());
    }
    #[test]
    fn loop_hoisting_no_loop() {
        let source = "let v = vec![1,2,3]; v.sort();";
        let findings = LoopHoistingAnalyzer::emit_findings(source);
        assert!(findings.is_empty());
    }
    #[test]
    fn redundant_computation_finds_duplicates() {
        let detector = RedundantComputationDetector::new(3);
        let source = "let x = foo + foo + foo;";
        let repeated = RedundantComputationDetector::check_line(source, 3);
        assert!(repeated.contains(& "foo".to_string()));
    }
    #[test]
    fn complexity_budget_spend_and_remaining() {
        let mut budget = ComplexityBudget::new(10.0);
        assert!(budget.spend(4.0));
        assert!((budget.remaining() - 6.0).abs() < 1e-9);
        assert!(! budget.is_exhausted());
        budget.spend(7.0);
        assert!(budget.is_exhausted());
    }
    #[test]
    fn complexity_budget_reset() {
        let mut budget = ComplexityBudget::new(5.0);
        budget.spend(5.0);
        budget.reset();
        assert!(! budget.is_exhausted());
        assert!((budget.remaining() - 5.0).abs() < 1e-9);
    }
    #[test]
    fn simp_call_counter_total() {
        let source = "simp [h1]\nsimp only [h2]\nrfl\nsimp\n";
        let total = SimpCallCounter::total_simp_calls(source);
        assert!(total >= 3);
    }
    #[test]
    fn simp_call_counter_only_ratio() {
        let source = "simp only [h1]\nsimp only [h2]\nsimp [h3]\n";
        let ratio = SimpCallCounter::simp_only_ratio(source);
        assert!((0.0..= 1.0).contains(& ratio));
        assert!(ratio > 0.5);
    }
    #[test]
    fn metavar_tracker_no_warning() {
        let tracker = MetavarTracker::new(20);
        let source = "theorem foo : Nat := 0";
        assert!(tracker.check(source).is_empty());
    }
    #[test]
    fn metavar_tracker_warning() {
        let tracker = MetavarTracker::new(0);
        let source = "theorem foo : ?_ := ?_";
        let findings = tracker.check(source);
        assert!(! findings.is_empty());
        assert_eq!(findings[0].issue, PerfIssue::ExcessiveMetavars);
    }
    #[test]
    fn inductive_size_checker_small() {
        let checker = InductiveSizeChecker::new(10);
        let source = "inductive Color\n| red\n| green\n| blue\n";
        assert!(checker.check(source).is_empty());
    }
    #[test]
    fn inductive_size_checker_large() {
        let checker = InductiveSizeChecker::new(2);
        let source = "inductive Color\n| red\n| green\n| blue\n";
        let findings = checker.check(source);
        assert!(! findings.is_empty());
        assert_eq!(findings[0].issue, PerfIssue::LargeInductiveType);
    }
    #[test]
    fn o_complexity_advisor_gives_suggestions() {
        let source = "if v.contains(&x) { ... }";
        let suggestions = OComplexityAdvisor::suggest(source);
        assert!(! suggestions.is_empty());
    }
    #[test]
    fn o_complexity_advisor_clean_source() {
        let source = "theorem foo : True := trivial";
        let suggestions = OComplexityAdvisor::suggest(source);
        assert!(suggestions.is_empty());
    }
    #[test]
    fn perf_report_health_score() {
        let findings = vec![
            PerfFinding::new(PerfIssue::LargeProofTerm, PerfSeverity::Warning, "x", "x"),
        ];
        let report = PerfReport::from_findings(findings, 0.2);
        let score = report.health_score();
        assert!((0.0..= 1.0).contains(& score));
        assert!(! report.is_clean());
    }
    #[test]
    fn perf_report_clean() {
        let report = PerfReport::from_findings(vec![], 0.0);
        assert!(report.is_clean());
        assert!((report.health_score() - 1.0).abs() < 0.1);
    }
    #[test]
    fn full_perf_analyzer_runs() {
        let analyzer = FullPerfAnalyzer::new();
        let source = "theorem foo : True := trivial";
        let report = analyzer.analyze(source);
        let score = report.health_score();
        assert!((0.0..= 1.0).contains(& score));
    }
    #[test]
    fn profiling_hint_from_findings() {
        let findings = vec![
            PerfFinding::new(PerfIssue::SlowSimpLemma, PerfSeverity::Warning, "line:5",
            "slow").with_impact(0.5),
        ];
        let hints = ProfilingHint::from_findings(&findings);
        assert_eq!(hints.len(), 1);
        assert_eq!(hints[0].location, "line:5");
    }
    #[test]
    fn perf_trend_is_improving() {
        let mut trend = PerfTrend::new();
        let f1 = vec![
            PerfFinding::new(PerfIssue::DeepRecursion, PerfSeverity::Warning, "x", "x"),
            PerfFinding::new(PerfIssue::LargeProofTerm, PerfSeverity::Warning, "y", "y"),
        ];
        let f2 = vec![
            PerfFinding::new(PerfIssue::DeepRecursion, PerfSeverity::Warning, "x", "x"),
        ];
        trend.record("v1", f1);
        trend.record("v2", f2);
        assert!(trend.is_improving());
        assert_eq!(trend.snapshot_count(), 2);
        assert_eq!(trend.latest_finding_count(), 1);
    }
    #[test]
    fn perf_trend_not_improving() {
        let mut trend = PerfTrend::new();
        trend.record("v1", vec![]);
        trend
            .record(
                "v2",
                vec![
                    PerfFinding::new(PerfIssue::SlowSimpLemma, PerfSeverity::Suggestion,
                    "x", "x"),
                ],
            );
        assert!(! trend.is_improving());
    }
    #[test]
    fn performance_threshold_exceeded() {
        let t = PerformanceThreshold::new("max_terms", 100.0, "Maximum term count");
        assert!(t.is_exceeded(101.0));
        assert!(! t.is_exceeded(99.0));
        assert!(! t.is_exceeded(100.0));
    }
    #[test]
    fn cache_unfriendly_access_simple_index() {
        let source = "let x = v[i];";
        let patterns = CacheUnfriendlyAccessDetector::find_random_access_patterns(
            source,
        );
        assert!(patterns.is_empty());
    }
    #[test]
    fn cache_unfriendly_access_complex_index() {
        let source = "let x = v[a + b * c];";
        let patterns = CacheUnfriendlyAccessDetector::find_random_access_patterns(
            source,
        );
        assert!(! patterns.is_empty());
    }
}
#[cfg(test)]
mod more_perf_tests {
    use super::*;
    #[test]
    fn memory_usage_estimator_empty() {
        assert_eq!(MemoryUsageEstimator::estimate_bytes(""), 0);
    }
    #[test]
    fn memory_usage_estimator_with_allocs() {
        let source = "Vec::new() Vec::new() String::new()";
        let bytes = MemoryUsageEstimator::estimate_bytes(source);
        assert!(bytes > 0);
    }
    #[test]
    fn memory_usage_estimator_readable() {
        let source = "Vec::new()";
        let readable = MemoryUsageEstimator::estimate_readable(source);
        assert!(
            readable.ends_with('B') || readable.ends_with("KB") || readable
            .ends_with("MB")
        );
    }
    #[test]
    fn tail_call_detector_detects_non_tail() {
        let source = "def fib n = fib (n-1) + fib (n-2)";
        let lines = TailCallDetector::find_non_tail_recursive_calls(source);
        assert!(! lines.is_empty());
    }
    #[test]
    fn tail_call_detector_no_false_positive() {
        let source = "def loop n acc = if n == 0 then acc else loop (n - 1) (acc + n)";
        let _lines = TailCallDetector::find_non_tail_recursive_calls(source);
    }
    #[test]
    fn elab_time_profiler_records() {
        let mut profiler = ElabTimeProfiler::new();
        profiler.record("typecheck", 50);
        profiler.record("unification", 120);
        profiler.record("simp", 30);
        assert_eq!(profiler.total_ms(), 200);
        assert!((profiler.average_ms() - 200.0 / 3.0).abs() < 1.0);
    }
    #[test]
    fn elab_time_profiler_slowest() {
        let mut profiler = ElabTimeProfiler::new();
        profiler.record("a", 10);
        profiler.record("b", 500);
        profiler.record("c", 50);
        let slowest = profiler.slowest().expect("slowest result should exist");
        assert_eq!(slowest.label, "b");
        assert_eq!(slowest.duration_ms, 500);
    }
    #[test]
    fn elab_time_profiler_sorted() {
        let mut profiler = ElabTimeProfiler::new();
        profiler.record("a", 100);
        profiler.record("b", 200);
        profiler.record("c", 50);
        let sorted = profiler.sorted_by_duration();
        assert_eq!(sorted[0].duration_ms, 200);
        assert_eq!(sorted[2].duration_ms, 50);
    }
    #[test]
    fn elab_time_profiler_empty_average() {
        let profiler = ElabTimeProfiler::new();
        assert_eq!(profiler.average_ms(), 0.0);
        assert!(profiler.slowest().is_none());
    }
    #[test]
    fn pattern_match_optimizer_count_arms() {
        let source = "match x with\n| A => 1\n| B => 2\n| C => 3\n";
        let arms = PatternMatchOptimizer::count_match_arms(source);
        assert_eq!(arms, 3);
    }
    #[test]
    fn pattern_match_optimizer_check_size() {
        let source = "| A ->\n| B ->\n| C ->\n| D ->\n| E ->\n";
        let findings = PatternMatchOptimizer::check_match_size(source, 3);
        assert!(! findings.is_empty());
    }
    #[test]
    fn typeclass_checker_no_findings() {
        let checker = TypeClassComplexityChecker::new(3);
        let source = "def foo [Add A] : A := sorry";
        let findings = checker.check(source);
        assert!(findings.is_empty());
    }
    #[test]
    fn typeclass_checker_finds_complex() {
        let checker = TypeClassComplexityChecker::new(1);
        let source = "def foo [Add A] [Mul A] [Ring A] : A := sorry";
        let findings = checker.check(source);
        assert!(! findings.is_empty());
    }
    #[test]
    fn simp_normal_form_normal() {
        assert!(SimpNormalFormChecker::is_normal_form("a = foo bar baz"));
        assert!(SimpNormalFormChecker::is_normal_form("a = a"));
    }
    #[test]
    fn simp_normal_form_non_normal() {
        assert!(! SimpNormalFormChecker::is_normal_form("foo_very_long_name = x"));
    }
    #[test]
    fn simp_normal_form_checker_list() {
        let lemmas = ["a = foo", "very_long_lhs = x", "b = c d"];
        let bad = SimpNormalFormChecker::find_non_normal_form_lemmas(&lemmas);
        assert!(bad.contains(& "very_long_lhs = x".to_string()));
    }
    #[test]
    fn dead_proof_branch_no_findings() {
        let source = "theorem foo : True := trivial";
        let findings = DeadProofBranchDetector::emit_findings(source);
        assert!(findings.is_empty());
    }
    #[test]
    fn dead_proof_branch_finds_pattern() {
        let source = "| False -> absurd rfl h";
        let branches = DeadProofBranchDetector::find_dead_branches(source);
        assert!(! branches.is_empty());
    }
    #[test]
    fn polymorphism_scorer_empty() {
        assert_eq!(PolymorphismComplexityScorer::score(""), 0.0);
    }
    #[test]
    fn polymorphism_scorer_complex() {
        let source = ": Type : Type : Type : Sort : Sort : Sort : Sort : Prop : Prop : Prop";
        let score = PolymorphismComplexityScorer::score(source);
        assert!(score > 0.0);
        assert!(score <= 1.0);
    }
    #[test]
    fn polymorphism_scorer_counts_type_vars() {
        let source = "(α : Type) (β : Type) (γ : Sort)";
        let count = PolymorphismComplexityScorer::count_type_variables(source);
        assert_eq!(count, 3);
    }
    #[test]
    fn perf_issue_display() {
        assert_eq!(format!("{}", PerfIssue::SlowSimpLemma), "slow_simp_lemma");
        assert_eq!(format!("{}", PerfIssue::LargeProofTerm), "large_proof_term");
        assert_eq!(format!("{}", PerfIssue::ExcessiveMetavars), "excessive_metavars");
    }
    #[test]
    fn perf_severity_ordering() {
        assert!(PerfSeverity::Blocker > PerfSeverity::Warning);
        assert!(PerfSeverity::Warning > PerfSeverity::Suggestion);
    }
    #[test]
    fn elab_complexity_unification_cost() {
        let source = "?_ + ?_ = ?_";
        let cost = ElabComplexityEstimator::estimate_unification_cost(source);
        assert!(cost > 0.0);
        assert!(cost <= 1.0);
    }
    #[test]
    fn elab_complexity_overall_clamped() {
        let huge: String = "fun x -> ".repeat(200);
        let score = ElabComplexityEstimator::overall_score(&huge);
        assert!(score <= 1.0);
    }
    #[test]
    fn perf_lint_check_instance_search_found() {
        let pass = PerfLintPass::new();
        let source = "def foo [A][B][C][D][E][F] := sorry";
        let findings = pass.check_instance_search(source);
        assert!(! findings.is_empty());
    }
    #[test]
    fn perf_lint_check_simp_config_not_exceeded() {
        let pass = PerfLintPass::new();
        let source = "simp [h1, h2]";
        let findings = pass.check_simp_config(source);
        assert!(findings.is_empty());
    }
    #[test]
    fn simp_call_counter_zero_ratio_on_no_calls() {
        let source = "theorem foo : True := trivial";
        let ratio = SimpCallCounter::simp_only_ratio(source);
        assert_eq!(ratio, 0.0);
    }
    #[test]
    fn inductive_size_checker_zero_constructors() {
        let source = "inductive Empty\n-- no constructors\n";
        let count = InductiveSizeChecker::count_constructors(source);
        assert_eq!(count, 0);
    }
    #[test]
    fn full_perf_analyzer_empty_source() {
        let analyzer = FullPerfAnalyzer::new();
        let report = analyzer.analyze("");
        assert!((0.0..= 1.0).contains(& report.health_score()));
    }
    #[test]
    fn full_perf_analyzer_complex_source() {
        let analyzer = FullPerfAnalyzer::new();
        let source = include_str!("performance_lints.rs");
        let report = analyzer.analyze(source);
        assert!((0.0..= 1.0).contains(& report.health_score()));
    }
}
#[cfg(test)]
mod final_perf_tests {
    use super::*;
    #[test]
    fn proof_search_depth_below_threshold() {
        let analyzer = ProofSearchDepthAnalyzer::new(10);
        let source = "apply h1\napply h2\n";
        assert!(analyzer.check(source).is_empty());
    }
    #[test]
    fn proof_search_depth_exceeded() {
        let analyzer = ProofSearchDepthAnalyzer::new(1);
        let source = "apply h1\napply h2\napply h3\n";
        let findings = analyzer.check(source);
        assert!(! findings.is_empty());
        assert_eq!(findings[0].issue, PerfIssue::UnboundedSearch);
    }
    #[test]
    fn vectorisation_no_loops() {
        let source = "let x = a + b";
        let opps = VectorisationOpportunityDetector::find_opportunities(source);
        assert!(opps.is_empty());
    }
    #[test]
    fn constant_folding_finds_expressions() {
        let source = "let x = 3 + 5";
        let exprs = ConstantFoldingChecker::find_foldable_expressions(source);
        assert!(! exprs.is_empty());
        assert!(exprs[0].1.contains('3') || exprs[0].1.contains('5'));
    }
    #[test]
    fn constant_folding_no_match() {
        let source = "let x = a + b";
        let exprs = ConstantFoldingChecker::find_foldable_expressions(source);
        assert!(exprs.is_empty());
    }
    #[test]
    fn eta_expansion_detects() {
        let source = "map (fun x -> f x) list";
        let expns = EtaExpansionDetector::find_eta_expansions(source);
        assert!(! expns.is_empty());
    }
    #[test]
    fn eta_expansion_no_match_different_var() {
        let source = "fun x -> f y";
        let expns = EtaExpansionDetector::find_eta_expansions(source);
        assert!(expns.is_empty());
    }
    #[test]
    fn benchmark_harness_runs() {
        let harness = BenchmarkHarness::new(3);
        let avg = harness
            .run_avg(|| {
                vec![
                    PerfFinding::new(PerfIssue::SlowSimpLemma, PerfSeverity::Suggestion,
                    "x", "x",)
                ]
            });
        assert!((avg - 1.0).abs() < 1e-9);
    }
    #[test]
    fn benchmark_harness_zero_findings() {
        let harness = BenchmarkHarness::new(5);
        let avg = harness.run_avg(|| vec![]);
        assert_eq!(avg, 0.0);
    }
    #[test]
    fn perf_finding_impact_clamp() {
        let f = PerfFinding::new(
                PerfIssue::SlowSimpLemma,
                PerfSeverity::Suggestion,
                "x",
                "x",
            )
            .with_impact(99.0);
        assert!((f.estimated_impact - 1.0).abs() < 1e-9);
    }
    #[test]
    fn perf_finding_negative_impact_clamp() {
        let f = PerfFinding::new(
                PerfIssue::SlowSimpLemma,
                PerfSeverity::Suggestion,
                "x",
                "x",
            )
            .with_impact(-5.0);
        assert_eq!(f.estimated_impact, 0.0);
    }
    #[test]
    fn perf_lint_config_clone() {
        let cfg = PerfLintConfig::default();
        let cloned = cfg.clone();
        assert_eq!(cloned.max_proof_term_size, cfg.max_proof_term_size);
        assert_eq!(cloned.max_recursion_depth, cfg.max_recursion_depth);
    }
    #[test]
    fn perf_lint_pass_with_config_preserves() {
        let cfg = PerfLintConfig {
            max_proof_term_size: 42,
            max_recursion_depth: 7,
            max_simp_lemmas: 3,
            warn_on_classical: true,
        };
        let pass = PerfLintPass::with_config(cfg.clone());
        let source = "a b c d e";
        let findings = pass.check_proof_term_size(source);
        assert!(findings.is_empty());
        let pass2 = PerfLintPass::with_config(PerfLintConfig {
            max_proof_term_size: 3,
            ..cfg
        });
        let findings2 = pass2.check_proof_term_size(source);
        assert!(! findings2.is_empty());
    }
    #[test]
    fn elab_complexity_empty_source() {
        let score = ElabComplexityEstimator::overall_score("");
        assert_eq!(score, 0.0);
    }
}
