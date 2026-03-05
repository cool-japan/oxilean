//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{AccessControlAuditor, CredentialLeakDetector, DependencyAuditor, ExposedDebugCodeDetector, ExternCrateAuditor, FullSecurityAnalyzer, InjectionVulnerabilityDetector, IntegerOverflowDetector, PathTraversalChecker, ProofIntegrityChecker, RiskClassifier, SanitizerChecker, SecretEntropyEstimator, SecurityAuditLog, SecurityBudget, SecurityFinding, SecurityIssue, SecurityLintConfig, SecurityLintPass, SecurityMetrics, SecurityPolicy, SecurityReport, SecurityRuleBook, SecuritySeverity, SecuritySummary, SecurityTrendTracker, SorryDensityChecker, SorryTracker, TaintAnalyzer, TrustLevel, UnsafeApiAuditor, VerificationGapDetector};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_security_finding_new() {
        let f = SecurityFinding::new(
            SecurityIssue::WeakAssumption,
            SecuritySeverity::Medium,
            "file.ox:10",
            "test message",
        );
        assert_eq!(f.location, "file.ox:10");
        assert_eq!(f.message, "test message");
        assert!(f.suggestion.is_none());
    }
    #[test]
    fn test_security_finding_critical() {
        let f = SecurityFinding::new(
            SecurityIssue::UnsoundAxiom,
            SecuritySeverity::Critical,
            "file.ox:1",
            "unsound",
        );
        assert!(f.is_critical());
        let not_critical = SecurityFinding::new(
            SecurityIssue::WeakAssumption,
            SecuritySeverity::Low,
            "file.ox:2",
            "weak",
        );
        assert!(! not_critical.is_critical());
    }
    #[test]
    fn test_config_default() {
        let cfg = SecurityLintConfig::default();
        assert!(cfg.check_sorry);
        assert!(cfg.check_classical);
        assert!(cfg.check_ffi);
        assert!(cfg.check_axioms);
        assert!(cfg.allow_sorry_in.is_empty());
    }
    #[test]
    fn test_config_strict() {
        let cfg = SecurityLintConfig::strict();
        assert!(cfg.check_sorry);
        assert!(cfg.check_classical);
        assert!(cfg.check_ffi);
        assert!(cfg.check_axioms);
    }
    #[test]
    fn test_check_for_sorry() {
        let pass = SecurityLintPass::new();
        let source = "theorem foo : True := sorry\ndef bar := sorry";
        let findings = pass.check_for_sorry(source);
        assert_eq!(findings.len(), 2);
        assert!(findings.iter().all(| f | f.severity == SecuritySeverity::High));
    }
    #[test]
    fn test_check_for_axioms() {
        let pass = SecurityLintPass::new();
        let source = "axiom my_axiom : False\ntheorem ok : True := trivial";
        let findings = pass.check_for_unsafe_axioms(source);
        assert_eq!(findings.len(), 1);
        assert!(findings[0].is_critical());
    }
    #[test]
    fn test_sorry_tracker_count() {
        assert_eq!(SorryTracker::count_sorries("sorry sorry sorry"), 3);
        assert_eq!(SorryTracker::count_sorries("no placeholder here"), 0);
        assert_eq!(SorryTracker::count_sorries(""), 0);
    }
    #[test]
    fn test_run_all() {
        let pass = SecurityLintPass::new();
        let source = "axiom bad : False\ntheorem foo : True := sorry";
        let findings = pass.run_all(source);
        assert!(findings.len() >= 2);
        let by_sev = pass.total_by_severity(&findings);
        assert!(by_sev.contains_key("critical") || by_sev.contains_key("high"));
    }
}
#[cfg(test)]
mod security_extended_tests {
    use super::*;
    #[test]
    fn taint_analyzer_finds_flow() {
        let mut analyzer = TaintAnalyzer::new();
        analyzer.add_source("user_data");
        analyzer.add_sink("exec");
        let source = "let result = exec(user_data)";
        let findings = analyzer.check_for_taint_flow(source);
        assert!(! findings.is_empty());
        assert!(findings[0].is_critical());
    }
    #[test]
    fn taint_analyzer_no_flow() {
        let analyzer = TaintAnalyzer::new();
        let source = "let x = user_input\nlet y = safe_function(y)";
        let findings = analyzer.check_for_taint_flow(source);
        assert!(findings.is_empty());
    }
    #[test]
    fn injection_detector_finds_eval() {
        let source = "eval(user_string)";
        let findings = InjectionVulnerabilityDetector::scan(source);
        assert!(! findings.is_empty());
    }
    #[test]
    fn injection_detector_no_findings() {
        let source = "let x = 1 + 2";
        let findings = InjectionVulnerabilityDetector::scan(source);
        assert!(findings.is_empty());
    }
    #[test]
    fn credential_leak_finds_password() {
        let source = "let password = \"my_secret_123\"";
        let findings = CredentialLeakDetector::scan(source);
        assert!(! findings.is_empty());
        assert!(findings[0].is_critical());
    }
    #[test]
    fn credential_leak_no_false_positive() {
        let source = "theorem foo : True := trivial";
        let findings = CredentialLeakDetector::scan(source);
        assert!(findings.is_empty());
    }
    #[test]
    fn path_traversal_finds_dotdot() {
        let source = "let path = \"../etc/passwd\"";
        let findings = PathTraversalChecker::scan(source);
        assert!(! findings.is_empty());
    }
    #[test]
    fn path_traversal_clean() {
        let source = "let path = \"/home/user/file.txt\"";
        let findings = PathTraversalChecker::scan(source);
        assert!(findings.is_empty());
    }
    #[test]
    fn integer_overflow_finds_arithmetic() {
        let source = "let x : u32 = 1000 + 2000";
        let findings = IntegerOverflowDetector::scan(source);
        assert!(! findings.is_empty());
    }
    #[test]
    fn integer_overflow_no_numeric() {
        let source = "let x = a + b";
        let findings = IntegerOverflowDetector::scan(source);
        assert!(findings.is_empty());
    }
    #[test]
    fn unsafe_api_auditor_transmute() {
        let source = "let y = std::mem::transmute::<u32, f32>(x)";
        let findings = UnsafeApiAuditor::scan(source);
        assert!(! findings.is_empty());
        assert!(findings.iter().any(| f | f.is_critical()));
    }
    #[test]
    fn unsafe_api_auditor_clean() {
        let source = "let x = 1 + 2";
        let findings = UnsafeApiAuditor::scan(source);
        assert!(findings.is_empty());
    }
    #[test]
    fn secret_entropy_high_entropy() {
        let s = "aB3xZ9kLmQ2nR7";
        assert!(SecretEntropyEstimator::entropy(s) > 3.0);
        assert!(SecretEntropyEstimator::looks_like_secret(s));
    }
    #[test]
    fn secret_entropy_low_entropy() {
        let s = "aaaaaaaaaa";
        assert_eq!(SecretEntropyEstimator::entropy(s), 0.0);
        assert!(! SecretEntropyEstimator::looks_like_secret(s));
    }
    #[test]
    fn secret_entropy_empty() {
        assert_eq!(SecretEntropyEstimator::entropy(""), 0.0);
    }
    #[test]
    fn security_report_risk_score() {
        let findings = vec![
            SecurityFinding::new(SecurityIssue::UnsoundAxiom, SecuritySeverity::Critical,
            "x", "x"), SecurityFinding::new(SecurityIssue::WeakAssumption,
            SecuritySeverity::High, "y", "y"),
        ];
        let report = SecurityReport::from_findings(findings);
        assert!(report.has_blocker());
        assert!(report.risk_score() > 0.0);
        assert!(! report.is_clean());
    }
    #[test]
    fn security_report_clean() {
        let report = SecurityReport::from_findings(vec![]);
        assert!(report.is_clean());
        assert_eq!(report.risk_score(), 0.0);
        assert!(! report.has_blocker());
    }
    #[test]
    fn security_report_sorted() {
        let findings = vec![
            SecurityFinding::new(SecurityIssue::WeakAssumption, SecuritySeverity::Low,
            "x", "x"), SecurityFinding::new(SecurityIssue::UnsoundAxiom,
            SecuritySeverity::Critical, "y", "y"),
        ];
        let report = SecurityReport::from_findings(findings);
        let sorted = report.sorted_by_severity();
        assert_eq!(sorted[0].severity, SecuritySeverity::Critical);
    }
    #[test]
    fn full_security_analyzer_runs() {
        let analyzer = FullSecurityAnalyzer::new();
        let source = "theorem foo : True := sorry";
        let report = analyzer.analyze(source);
        assert!(! report.is_clean());
    }
    #[test]
    fn security_policy_permissive_passes_high() {
        let policy = SecurityPolicy::permissive();
        let findings = vec![
            SecurityFinding::new(SecurityIssue::WeakAssumption, SecuritySeverity::High,
            "x", "x"),
        ];
        let report = SecurityReport::from_findings(findings);
        assert!(policy.passes(& report));
    }
    #[test]
    fn security_policy_strict_fails_high() {
        let policy = SecurityPolicy::strict();
        let findings = vec![
            SecurityFinding::new(SecurityIssue::WeakAssumption, SecuritySeverity::High,
            "x", "x"),
        ];
        let report = SecurityReport::from_findings(findings);
        assert!(! policy.passes(& report));
    }
    #[test]
    fn security_policy_strict_passes_low() {
        let policy = SecurityPolicy::strict();
        let findings = vec![
            SecurityFinding::new(SecurityIssue::WeakAssumption, SecuritySeverity::Low,
            "x", "x"),
        ];
        let report = SecurityReport::from_findings(findings);
        assert!(policy.passes(& report));
    }
    #[test]
    fn trust_level_ordering() {
        assert!(TrustLevel::Compromised > TrustLevel::Untrusted);
        assert!(TrustLevel::Untrusted > TrustLevel::Reviewed);
        assert!(TrustLevel::Reviewed > TrustLevel::Verified);
    }
    #[test]
    fn trust_level_display() {
        assert_eq!(format!("{}", TrustLevel::Verified), "verified");
        assert_eq!(format!("{}", TrustLevel::Compromised), "compromised");
    }
    #[test]
    fn security_audit_log_basic() {
        let mut log = SecurityAuditLog::new();
        let finding = SecurityFinding::new(
            SecurityIssue::WeakAssumption,
            SecuritySeverity::Medium,
            "x",
            "test",
        );
        let id = log.log(&finding);
        assert_eq!(log.total(), 1);
        assert_eq!(log.unresolved().len(), 1);
        assert!(log.resolve(id));
        assert_eq!(log.unresolved().len(), 0);
    }
    #[test]
    fn security_audit_log_resolve_nonexistent() {
        let mut log = SecurityAuditLog::new();
        assert!(! log.resolve(999));
    }
    #[test]
    fn security_metrics_compute() {
        let findings = vec![
            SecurityFinding::new(SecurityIssue::UnsoundAxiom, SecuritySeverity::Critical,
            "x", "x"), SecurityFinding::new(SecurityIssue::WeakAssumption,
            SecuritySeverity::Critical, "y", "y"),
            SecurityFinding::new(SecurityIssue::CircularProof, SecuritySeverity::High,
            "z", "z"),
        ];
        let metrics = SecurityMetrics::compute(&findings);
        assert_eq!(metrics.total, 3);
        assert_eq!(* metrics.by_severity.get("critical").expect("key should exist"), 2);
        assert_eq!(* metrics.by_severity.get("high").expect("key should exist"), 1);
    }
    #[test]
    fn security_metrics_most_common_issue() {
        let findings = vec![
            SecurityFinding::new(SecurityIssue::WeakAssumption, SecuritySeverity::Low,
            "x", "x"), SecurityFinding::new(SecurityIssue::WeakAssumption,
            SecuritySeverity::Low, "y", "y"),
            SecurityFinding::new(SecurityIssue::UnsoundAxiom, SecuritySeverity::Critical,
            "z", "z"),
        ];
        let metrics = SecurityMetrics::compute(&findings);
        let most_common = metrics.most_common_issue().expect("most common issue should exist");
        assert_eq!(most_common, "weak_assumption");
    }
    #[test]
    fn security_issue_display() {
        assert_eq!(format!("{}", SecurityIssue::UncheckedInput), "unchecked_input");
        assert_eq!(format!("{}", SecurityIssue::UnsoundAxiom), "unsound_axiom");
        assert_eq!(format!("{}", SecurityIssue::DangerousFfi), "dangerous_ffi");
    }
    #[test]
    fn sorry_tracker_is_sorry_in_proof() {
        assert!(SorryTracker::is_sorry_in_proof("theorem foo := sorry"));
        assert!(! SorryTracker::is_sorry_in_proof("-- sorry is a keyword"));
    }
    #[test]
    fn sorry_tracker_word_boundary() {
        let locs = SorryTracker::sorry_locations("let sorryX = 1\nlet x = sorry");
        assert_eq!(locs.len(), 1);
        assert!(locs[0].contains('2'));
    }
    #[test]
    fn security_severity_display() {
        assert_eq!(format!("{}", SecuritySeverity::Critical), "critical");
        assert_eq!(format!("{}", SecuritySeverity::Info), "info");
    }
    #[test]
    fn security_finding_with_suggestion() {
        let f = SecurityFinding::new(
                SecurityIssue::WeakAssumption,
                SecuritySeverity::Low,
                "x",
                "msg",
            )
            .with_suggestion("fix it");
        assert_eq!(f.suggestion.as_deref(), Some("fix it"));
    }
}
#[cfg(test)]
mod final_security_tests {
    use super::*;
    #[test]
    fn proof_integrity_finds_sorry_theorems() {
        let source = "theorem bad : False := sorry\ntheorem good : True := trivial";
        let findings = ProofIntegrityChecker::check_theorems_have_proofs(source);
        assert_eq!(findings.len(), 1);
    }
    #[test]
    fn proof_integrity_noncomputable() {
        let source = "noncomputable def myDef : Nat := Classical.choice ⟨0, rfl⟩";
        let findings = ProofIntegrityChecker::check_noncomputable(source);
        assert!(! findings.is_empty());
        assert_eq!(findings[0].severity, SecuritySeverity::Info);
    }
    #[test]
    fn dependency_auditor_flags_risky() {
        let mut auditor = DependencyAuditor::new();
        auditor.add_risky_prefix("EvilLib");
        let source = "import EvilLib.Dangerous\nimport SafeLib.Good";
        let findings = auditor.check(source);
        assert_eq!(findings.len(), 1);
        assert!(findings[0].message.contains("EvilLib"));
    }
    #[test]
    fn dependency_auditor_clean() {
        let auditor = DependencyAuditor::new();
        let source = "import SafeLib.Core";
        let findings = auditor.check(source);
        assert!(findings.is_empty());
    }
    #[test]
    fn verification_gap_count() {
        let source = "theorem a : True := trivial\ntheorem b : False := sorry\nlemma c : True := sorry";
        let gaps = VerificationGapDetector::count_gaps(source);
        assert_eq!(gaps, 2);
    }
    #[test]
    fn verification_gap_ratio() {
        let source = "theorem a : True := trivial\ntheorem b : False := sorry";
        let ratio = VerificationGapDetector::gap_ratio(source);
        assert!((ratio - 0.5).abs() < 1e-9);
    }
    #[test]
    fn verification_gap_ratio_no_theorems() {
        let source = "def x := 1";
        assert_eq!(VerificationGapDetector::gap_ratio(source), 0.0);
    }
    #[test]
    fn extern_crate_auditor_allowed() {
        let auditor = ExternCrateAuditor::new(vec!["std", "core"]);
        let source = "extern crate std;";
        let findings = auditor.check(source);
        assert!(findings.is_empty());
    }
    #[test]
    fn extern_crate_auditor_disallowed() {
        let auditor = ExternCrateAuditor::new(vec!["std"]);
        let source = "extern crate unknown_crate;";
        let findings = auditor.check(source);
        assert_eq!(findings.len(), 1);
    }
    #[test]
    fn security_summary_one_line() {
        let report = SecurityReport::from_findings(
            vec![
                SecurityFinding::new(SecurityIssue::WeakAssumption,
                SecuritySeverity::High, "x", "x"),
            ],
        );
        let summary = SecuritySummary::one_line(&report);
        assert!(summary.contains("Security:"));
        assert!(summary.contains("high"));
    }
    #[test]
    fn security_summary_markdown() {
        let report = SecurityReport::from_findings(vec![]);
        let md = SecuritySummary::markdown(&report);
        assert!(md.contains("## Security Report"));
        assert!(md.contains("Total findings: 0"));
    }
    #[test]
    fn security_rule_book_lookup() {
        let book = SecurityRuleBook::new();
        assert!(book.get_rule("no_sorry").is_some());
        assert!(book.get_rule("nonexistent").is_none());
    }
    #[test]
    fn security_rule_book_add_rule() {
        let mut book = SecurityRuleBook::new();
        book.add_rule("custom_rule", "No custom violations allowed.");
        assert!(book.get_rule("custom_rule").is_some());
    }
    #[test]
    fn security_rule_book_names_sorted() {
        let book = SecurityRuleBook::new();
        let names = book.rule_names();
        assert!(! names.is_empty());
        for i in 0..names.len().saturating_sub(1) {
            assert!(names[i] <= names[i + 1]);
        }
    }
    #[test]
    fn security_lint_config_strict() {
        let cfg = SecurityLintConfig::strict();
        assert!(cfg.check_sorry);
        assert!(cfg.check_axioms);
        assert!(cfg.check_ffi);
        assert!(cfg.check_classical);
    }
    #[test]
    fn security_lint_pass_disabled_sorry() {
        let pass = SecurityLintPass::with_config(SecurityLintConfig {
            check_sorry: false,
            ..SecurityLintConfig::default()
        });
        let source = "theorem bad : False := sorry";
        let findings = pass.check_for_sorry(source);
        assert!(findings.is_empty());
    }
    #[test]
    fn security_lint_pass_disabled_axioms() {
        let pass = SecurityLintPass::with_config(SecurityLintConfig {
            check_axioms: false,
            ..SecurityLintConfig::default()
        });
        let source = "axiom unsound : False";
        let findings = pass.check_for_unsafe_axioms(source);
        assert!(findings.is_empty());
    }
    #[test]
    fn security_lint_classical_choice_detected() {
        let pass = SecurityLintPass::new();
        let source = "def x := Classical.choice ⟨0, rfl⟩";
        let findings = pass.check_for_classical_choice(source);
        assert!(! findings.is_empty());
    }
    #[test]
    fn security_lint_propext_detected() {
        let pass = SecurityLintPass::new();
        let source = "propext (Iff.intro h1 h2)";
        let findings = pass.check_for_classical_choice(source);
        assert!(! findings.is_empty());
        assert!(findings.iter().any(| f | f.issue == SecurityIssue::PropExtMisuse));
    }
    #[test]
    fn security_lint_circular_imports() {
        let pass = SecurityLintPass::new();
        let imports = vec![
            "Mathlib.Algebra.Ring.Basic".to_string(), "Mathlib.Data.Nat.Basic"
            .to_string(), "Mathlib.Algebra.Ring.Basic".to_string(),
        ];
        let findings = pass.check_for_circular_imports(&imports);
        assert_eq!(findings.len(), 1);
    }
    #[test]
    fn taint_analyzer_custom_source_and_sink() {
        let mut analyzer = TaintAnalyzer::new();
        analyzer.add_source("external_data");
        analyzer.add_sink("dangerous_fn");
        let source = "dangerous_fn(external_data)";
        let findings = analyzer.check_for_taint_flow(source);
        assert!(! findings.is_empty());
    }
    #[test]
    fn secret_entropy_finds_in_source() {
        let source = "let token = \"aB3xZ9kLmQ2nR7pS\"";
        let found = SecretEntropyEstimator::find_high_entropy_strings(source);
        assert!(! found.is_empty());
    }
    #[test]
    fn secret_entropy_low_entropy_not_found() {
        let source = "let s = \"aaaaaaaaaa\"";
        let found = SecretEntropyEstimator::find_high_entropy_strings(source);
        assert!(found.is_empty());
    }
}
#[cfg(test)]
mod security_trend_tests {
    use super::*;
    #[test]
    fn security_trend_tracker_basic() {
        let mut tracker = SecurityTrendTracker::new();
        let r1 = SecurityReport::from_findings(
            vec![
                SecurityFinding::new(SecurityIssue::UnsoundAxiom,
                SecuritySeverity::Critical, "x", "x"),
            ],
        );
        let r2 = SecurityReport::from_findings(vec![]);
        tracker.record("v1", r1);
        tracker.record("v2", r2);
        assert_eq!(tracker.snapshot_count(), 2);
        assert!(tracker.is_improving());
        assert_eq!(tracker.latest_risk_score(), 0.0);
    }
    #[test]
    fn security_trend_tracker_empty() {
        let tracker = SecurityTrendTracker::new();
        assert_eq!(tracker.latest_risk_score(), 0.0);
        assert!(! tracker.is_improving());
    }
    #[test]
    fn sorry_density_checker_below_threshold() {
        let checker = SorryDensityChecker::new(0.5);
        let source = "theorem a : True := trivial\ntheorem b : True := trivial";
        assert!(checker.check(source).is_empty());
    }
    #[test]
    fn sorry_density_checker_above_threshold() {
        let checker = SorryDensityChecker::new(0.0);
        let source = "theorem a : True := sorry";
        let findings = checker.check(source);
        assert!(! findings.is_empty());
    }
    #[test]
    fn sorry_density_no_decls() {
        assert_eq!(SorryDensityChecker::density("-- just a comment"), 0.0);
    }
}
#[cfg(test)]
mod access_control_tests {
    use super::*;
    #[test]
    fn access_control_finds_pub_secret() {
        let source = "pub fn get_secret_key() -> &str { \"key\" }";
        let findings = AccessControlAuditor::check_pub_exposure(source);
        assert!(! findings.is_empty());
    }
    #[test]
    fn access_control_clean() {
        let source = "pub fn compute(x: i32) -> i32 { x + 1 }";
        let findings = AccessControlAuditor::check_pub_exposure(source);
        assert!(findings.is_empty());
    }
    #[test]
    fn sanitizer_checker_unsanitized() {
        let source = "let query = format!(\"SELECT * FROM users WHERE id = {}\", user_input)";
        let findings = SanitizerChecker::check_unsanitized_inputs(source);
        assert!(! findings.is_empty());
    }
    #[test]
    fn sanitizer_checker_sanitized() {
        let source = "let safe = sanitize(user_input)";
        let findings = SanitizerChecker::check_unsanitized_inputs(source);
        assert!(findings.is_empty());
    }
    #[test]
    fn exposed_debug_finds_println() {
        let source = "println!(\"debug value: {}\", x)";
        let findings = ExposedDebugCodeDetector::scan(source);
        assert!(! findings.is_empty());
    }
    #[test]
    fn exposed_debug_clean() {
        let source = "let x = compute(42)";
        let findings = ExposedDebugCodeDetector::scan(source);
        assert!(findings.is_empty());
    }
    #[test]
    fn exposed_debug_finds_todo() {
        let source = "// TODO: remove this before release\nlet x = 1;";
        let findings = ExposedDebugCodeDetector::scan(source);
        assert!(! findings.is_empty());
    }
    #[test]
    fn full_security_analyzer_on_empty() {
        let analyzer = FullSecurityAnalyzer::new();
        let report = analyzer.analyze("");
        assert!(report.is_clean());
    }
    #[test]
    fn security_report_risk_score_clamped() {
        let findings: Vec<SecurityFinding> = (0..10)
            .map(|i| SecurityFinding::new(
                SecurityIssue::UnsoundAxiom,
                SecuritySeverity::Critical,
                &format!("line:{}", i),
                "critical",
            ))
            .collect();
        let report = SecurityReport::from_findings(findings);
        assert!(report.risk_score() <= 1.0);
    }
    #[test]
    fn security_severity_ordering() {
        assert!(SecuritySeverity::Critical > SecuritySeverity::High);
        assert!(SecuritySeverity::High > SecuritySeverity::Medium);
        assert!(SecuritySeverity::Medium > SecuritySeverity::Low);
        assert!(SecuritySeverity::Low > SecuritySeverity::Info);
    }
    #[test]
    fn security_lint_pass_run_all_returns_all() {
        let pass = SecurityLintPass::new();
        let source = "axiom unsound : False\ntheorem bad : False := sorry\n@[extern \"unsafe_fn\"] def foo : Nat := 0";
        let findings = pass.run_all(source);
        assert!(findings.len() >= 2);
    }
    #[test]
    fn security_lint_pass_total_by_severity_empty() {
        let pass = SecurityLintPass::new();
        let map = pass.total_by_severity(&[]);
        assert!(map.is_empty());
    }
}
#[cfg(test)]
mod security_budget_tests {
    use super::*;
    #[test]
    fn security_budget_accept_low_risk() {
        let mut budget = SecurityBudget::new(1.0);
        let f = SecurityFinding::new(
            SecurityIssue::WeakAssumption,
            SecuritySeverity::Low,
            "x",
            "x",
        );
        assert!(budget.accept(& f));
        assert!(! budget.is_exhausted());
    }
    #[test]
    fn security_budget_exceeded_by_critical() {
        let mut budget = SecurityBudget::new(0.3);
        let f = SecurityFinding::new(
            SecurityIssue::UnsoundAxiom,
            SecuritySeverity::Critical,
            "x",
            "x",
        );
        let ok = budget.accept(&f);
        assert!(! ok);
        assert!(budget.is_exhausted());
    }
}
#[cfg(test)]
mod risk_classifier_tests {
    use super::*;
    #[test]
    fn risk_classifier_clean() {
        let report = SecurityReport::from_findings(vec![]);
        assert_eq!(RiskClassifier::classify(& report), "clean");
    }
    #[test]
    fn risk_classifier_critical() {
        let findings: Vec<SecurityFinding> = (0..3)
            .map(|i| SecurityFinding::new(
                SecurityIssue::UnsoundAxiom,
                SecuritySeverity::Critical,
                &format!("line:{}", i),
                "x",
            ))
            .collect();
        let report = SecurityReport::from_findings(findings);
        let class = RiskClassifier::classify(&report);
        assert!(class == "critical" || class == "high");
    }
}
