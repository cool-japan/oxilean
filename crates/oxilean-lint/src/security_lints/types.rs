//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashMap;

/// Audits visibility modifiers to ensure sensitive items are not over-exposed.
#[allow(dead_code)]
pub struct AccessControlAuditor;
impl AccessControlAuditor {
    /// Find `pub` declarations that look like they might expose sensitive internals.
    #[allow(dead_code)]
    pub fn check_pub_exposure(source: &str) -> Vec<SecurityFinding> {
        let sensitive_keywords = [
            "secret",
            "private",
            "internal",
            "credential",
            "key",
            "token",
            "password",
        ];
        let mut findings = Vec::new();
        for (line_idx, line) in source.lines().enumerate() {
            let t = line.trim();
            if t.starts_with("pub ") || t.starts_with("pub(crate) ") {
                let lower = t.to_lowercase();
                for kw in &sensitive_keywords {
                    if lower.contains(kw) {
                        findings
                            .push(
                                SecurityFinding::new(
                                    SecurityIssue::ExposedPrivateData,
                                    SecuritySeverity::High,
                                    &format!("line:{}", line_idx + 1),
                                    &format!(
                                        "Public declaration containing `{}` may expose sensitive data",
                                        kw
                                    ),
                                ),
                            );
                    }
                }
            }
        }
        findings
    }
}
/// Represents the trust level associated with a piece of proof code.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TrustLevel {
    /// Fully verified; no concerns.
    Verified,
    /// Unverified but reviewed.
    Reviewed,
    /// Untrusted; requires further audit.
    Untrusted,
    /// Contains known vulnerabilities.
    Compromised,
}
/// Aggregated metrics about security findings.
#[allow(dead_code)]
pub struct SecurityMetrics {
    pub total: usize,
    pub by_severity: HashMap<String, usize>,
    pub by_issue: HashMap<String, usize>,
}
impl SecurityMetrics {
    /// Compute metrics from a slice of findings.
    #[allow(dead_code)]
    pub fn compute(findings: &[SecurityFinding]) -> Self {
        let total = findings.len();
        let mut by_severity: HashMap<String, usize> = HashMap::new();
        let mut by_issue: HashMap<String, usize> = HashMap::new();
        for f in findings {
            *by_severity.entry(f.severity.to_string()).or_insert(0) += 1;
            *by_issue.entry(f.issue.to_string()).or_insert(0) += 1;
        }
        Self {
            total,
            by_severity,
            by_issue,
        }
    }
    /// Return the most common issue type.
    #[allow(dead_code)]
    pub fn most_common_issue(&self) -> Option<String> {
        self.by_issue.iter().max_by_key(|(_, &v)| v).map(|(k, _)| k.clone())
    }
}
/// Checks that proof-bearing declarations maintain logical integrity.
#[allow(dead_code)]
pub struct ProofIntegrityChecker;
impl ProofIntegrityChecker {
    /// Verify that every `theorem` declaration has a non-`sorry` proof body.
    #[allow(dead_code)]
    pub fn check_theorems_have_proofs(source: &str) -> Vec<SecurityFinding> {
        let mut findings = Vec::new();
        for (line_idx, line) in source.lines().enumerate() {
            let t = line.trim();
            if (t.starts_with("theorem ") || t.starts_with("lemma ")) && t.contains(":=")
                && t.contains("sorry")
            {
                findings
                    .push(
                        SecurityFinding::new(
                                SecurityIssue::WeakAssumption,
                                SecuritySeverity::High,
                                &format!("line:{}", line_idx + 1),
                                "Theorem/Lemma with `sorry` proof body",
                            )
                            .with_suggestion("Replace `sorry` with a complete proof."),
                    );
            }
        }
        findings
    }
    /// Detect `noncomputable` declarations that might hide soundness issues.
    #[allow(dead_code)]
    pub fn check_noncomputable(source: &str) -> Vec<SecurityFinding> {
        let mut findings = Vec::new();
        for (line_idx, line) in source.lines().enumerate() {
            if line.trim().starts_with("noncomputable ") {
                findings
                    .push(
                        SecurityFinding::new(
                            SecurityIssue::ClassicalChoice,
                            SecuritySeverity::Info,
                            &format!("line:{}", line_idx + 1),
                            "`noncomputable` declaration may rely on classical axioms",
                        ),
                    );
            }
        }
        findings
    }
}
/// Detects patterns that commonly lead to injection vulnerabilities.
#[allow(dead_code)]
pub struct InjectionVulnerabilityDetector;
impl InjectionVulnerabilityDetector {
    /// Patterns that suggest unsafe string interpolation into command/query strings.
    #[allow(dead_code)]
    fn risky_patterns() -> Vec<(&'static str, &'static str)> {
        vec![
            ("format!(\"{}\"",
            "String interpolation into format may be injection-prone"), ("concat!(",
            "String concatenation may allow injection"), ("eval(",
            "eval() call can execute arbitrary code"), ("Command::new(",
            "Shell command construction may allow injection"), ("process::Command",
            "Process spawning may allow command injection"),
        ]
    }
    /// Scan source for injection-prone patterns.
    #[allow(dead_code)]
    pub fn scan(source: &str) -> Vec<SecurityFinding> {
        let mut findings = Vec::new();
        for (line_idx, line) in source.lines().enumerate() {
            for (pattern, description) in Self::risky_patterns() {
                if line.contains(pattern) {
                    findings
                        .push(
                            SecurityFinding::new(
                                    SecurityIssue::UncheckedInput,
                                    SecuritySeverity::High,
                                    &format!("line:{}", line_idx + 1),
                                    description,
                                )
                                .with_suggestion("Validate and sanitise inputs before use."),
                        );
                }
            }
        }
        findings
    }
}
/// Detects potential path traversal vulnerabilities.
#[allow(dead_code)]
pub struct PathTraversalChecker;
impl PathTraversalChecker {
    /// Patterns suggesting unsafe path construction.
    #[allow(dead_code)]
    fn risky_path_patterns() -> Vec<&'static str> {
        vec!["../", "..\\", "Path::new(user", "PathBuf::from(user", "join(user"]
    }
    /// Scan source for path traversal patterns.
    #[allow(dead_code)]
    pub fn scan(source: &str) -> Vec<SecurityFinding> {
        let mut findings = Vec::new();
        for (line_idx, line) in source.lines().enumerate() {
            for pat in Self::risky_path_patterns() {
                if line.contains(pat) {
                    findings
                        .push(
                            SecurityFinding::new(
                                    SecurityIssue::UncheckedInput,
                                    SecuritySeverity::High,
                                    &format!("line:{}", line_idx + 1),
                                    &format!(
                                        "Potential path traversal: `{}` in path construction", pat
                                    ),
                                )
                                .with_suggestion(
                                    "Canonicalize paths and validate that they remain within allowed directories.",
                                ),
                        );
                }
            }
        }
        findings
    }
}
/// Audits `extern crate` declarations.
#[allow(dead_code)]
pub struct ExternCrateAuditor {
    pub allowed_crates: Vec<String>,
}
impl ExternCrateAuditor {
    #[allow(dead_code)]
    pub fn new(allowed_crates: Vec<&str>) -> Self {
        Self {
            allowed_crates: allowed_crates.iter().map(|s| s.to_string()).collect(),
        }
    }
    /// Emit findings for `extern crate` declarations not in the allowlist.
    #[allow(dead_code)]
    pub fn check(&self, source: &str) -> Vec<SecurityFinding> {
        let mut findings = Vec::new();
        for (line_idx, line) in source.lines().enumerate() {
            let t = line.trim();
            if t.starts_with("extern crate ") {
                let crate_name = t["extern crate ".len()..].trim_end_matches(';');
                if !self.allowed_crates.iter().any(|a| a == crate_name) {
                    findings
                        .push(
                            SecurityFinding::new(
                                    SecurityIssue::UnverifiedExternal,
                                    SecuritySeverity::Medium,
                                    &format!("line:{}", line_idx + 1),
                                    &format!("Unapproved extern crate: `{}`", crate_name),
                                )
                                .with_suggestion(
                                    "Add this crate to the approved list after audit.",
                                ),
                        );
                }
            }
        }
        findings
    }
}
/// A compact text summary of a `SecurityReport`.
#[allow(dead_code)]
pub struct SecuritySummary;
impl SecuritySummary {
    /// Generate a one-line summary string for a report.
    #[allow(dead_code)]
    pub fn one_line(report: &SecurityReport) -> String {
        format!(
            "Security: {} findings — {} critical, {} high, {} medium, {} low, {} info | risk {:.2}",
            report.findings.len(), report.critical_count, report.high_count, report
            .medium_count, report.low_count, report.info_count, report.risk_score()
        )
    }
    /// Return a multi-line markdown-style summary.
    #[allow(dead_code)]
    pub fn markdown(report: &SecurityReport) -> String {
        let mut lines = Vec::new();
        lines.push("## Security Report".to_string());
        lines.push(format!("- Total findings: {}", report.findings.len()));
        lines.push(format!("- Critical: {}", report.critical_count));
        lines.push(format!("- High: {}", report.high_count));
        lines.push(format!("- Medium: {}", report.medium_count));
        lines.push(format!("- Low: {}", report.low_count));
        lines.push(format!("- Info: {}", report.info_count));
        lines.push(format!("- Risk score: {:.3}", report.risk_score()));
        lines.join("\n")
    }
}
/// A named security policy with a list of checks to enforce.
#[allow(dead_code)]
pub struct SecurityPolicy {
    pub name: String,
    pub check_sorry: bool,
    pub check_axioms: bool,
    pub check_classical: bool,
    pub check_ffi: bool,
    pub check_injection: bool,
    pub check_credentials: bool,
    pub check_path_traversal: bool,
    pub check_unsafe_api: bool,
    pub min_severity_to_fail: SecuritySeverity,
}
impl SecurityPolicy {
    /// A permissive policy that only fails on Critical findings.
    #[allow(dead_code)]
    pub fn permissive() -> Self {
        Self {
            name: "permissive".to_string(),
            check_sorry: false,
            check_axioms: false,
            check_classical: false,
            check_ffi: false,
            check_injection: true,
            check_credentials: true,
            check_path_traversal: true,
            check_unsafe_api: false,
            min_severity_to_fail: SecuritySeverity::Critical,
        }
    }
    /// A strict policy that fails on High or above.
    #[allow(dead_code)]
    pub fn strict() -> Self {
        Self {
            name: "strict".to_string(),
            check_sorry: true,
            check_axioms: true,
            check_classical: true,
            check_ffi: true,
            check_injection: true,
            check_credentials: true,
            check_path_traversal: true,
            check_unsafe_api: true,
            min_severity_to_fail: SecuritySeverity::High,
        }
    }
    /// Returns `true` if the given report passes the policy.
    #[allow(dead_code)]
    pub fn passes(&self, report: &SecurityReport) -> bool {
        !report.findings.iter().any(|f| f.severity >= self.min_severity_to_fail)
    }
}
/// Configuration for the security lint pass.
#[derive(Clone, Debug)]
pub struct SecurityLintConfig {
    /// Whether to check for Classical.choice usage.
    pub check_classical: bool,
    /// Whether to flag `sorry` occurrences.
    pub check_sorry: bool,
    /// Whether to check for FFI calls.
    pub check_ffi: bool,
    /// Whether to audit axiom declarations.
    pub check_axioms: bool,
    /// Names of declarations where `sorry` is explicitly allowed.
    pub allow_sorry_in: Vec<String>,
}
impl SecurityLintConfig {
    /// Create a default (permissive) configuration.
    pub fn default() -> Self {
        Self {
            check_classical: true,
            check_sorry: true,
            check_ffi: true,
            check_axioms: true,
            allow_sorry_in: Vec::new(),
        }
    }
    /// Create a strict configuration with all checks enabled and no exceptions.
    pub fn strict() -> Self {
        Self {
            check_classical: true,
            check_sorry: true,
            check_ffi: true,
            check_axioms: true,
            allow_sorry_in: Vec::new(),
        }
    }
}
/// Detects patterns that may lead to integer overflow.
#[allow(dead_code)]
pub struct IntegerOverflowDetector;
impl IntegerOverflowDetector {
    /// Patterns that suggest unchecked arithmetic.
    #[allow(dead_code)]
    fn overflow_patterns() -> Vec<(&'static str, &'static str)> {
        vec![
            (" + ", "Addition without overflow check"), (" * ",
            "Multiplication without overflow check"), (" - ",
            "Subtraction without underflow check"), ("as usize",
            "Cast to usize may panic/overflow"), ("as u32", "Cast to u32 may truncate"),
            ("as i32", "Cast to i32 may overflow"),
        ]
    }
    /// Scan source for potential overflow patterns (only in numeric contexts).
    #[allow(dead_code)]
    pub fn scan(source: &str) -> Vec<SecurityFinding> {
        let mut findings = Vec::new();
        for (line_idx, line) in source.lines().enumerate() {
            let has_numeric = line.chars().any(|c| c.is_ascii_digit());
            if !has_numeric {
                continue;
            }
            for (pat, desc) in Self::overflow_patterns() {
                if line.contains(pat) {
                    findings
                        .push(
                            SecurityFinding::new(
                                    SecurityIssue::WeakAssumption,
                                    SecuritySeverity::Low,
                                    &format!("line:{}", line_idx + 1),
                                    desc,
                                )
                                .with_suggestion(
                                    "Use checked arithmetic (checked_add, checked_mul, etc.) or saturating variants.",
                                ),
                        );
                    break;
                }
            }
        }
        findings
    }
}
/// A simple log of security events.
#[allow(dead_code)]
pub struct SecurityAuditLog {
    entries: Vec<SecurityAuditEntry>,
    counter: u64,
}
impl SecurityAuditLog {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            counter: 0,
        }
    }
    /// Log a new security finding.
    #[allow(dead_code)]
    pub fn log(&mut self, finding: &SecurityFinding) -> u64 {
        self.counter += 1;
        let id = self.counter;
        self.entries
            .push(SecurityAuditEntry {
                id,
                severity: finding.severity,
                message: finding.message.clone(),
                resolved: false,
            });
        id
    }
    /// Mark an entry as resolved by ID.
    #[allow(dead_code)]
    pub fn resolve(&mut self, id: u64) -> bool {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.id == id) {
            entry.resolved = true;
            true
        } else {
            false
        }
    }
    /// Return unresolved entries.
    #[allow(dead_code)]
    pub fn unresolved(&self) -> Vec<&SecurityAuditEntry> {
        self.entries.iter().filter(|e| !e.resolved).collect()
    }
    /// Total number of logged entries.
    #[allow(dead_code)]
    pub fn total(&self) -> usize {
        self.entries.len()
    }
}
/// Detects debug/development code that should not be in production.
#[allow(dead_code)]
pub struct ExposedDebugCodeDetector;
impl ExposedDebugCodeDetector {
    /// Patterns that suggest debug code left in production.
    #[allow(dead_code)]
    fn debug_patterns() -> Vec<(&'static str, &'static str)> {
        vec![
            ("println!(", "println! macro should not be in production code"), ("dbg!(",
            "dbg! macro should not be in production code"), ("eprintln!(",
            "eprintln! may leak sensitive info in logs"), ("// TODO: remove",
            "TODO marker for code removal detected"), ("// FIXME:",
            "FIXME marker indicates known issue"), ("// HACK:",
            "HACK marker indicates fragile code"), ("// DEBUG",
            "DEBUG comment in production code"),
        ]
    }
    /// Scan source for debug code patterns.
    #[allow(dead_code)]
    pub fn scan(source: &str) -> Vec<SecurityFinding> {
        let mut findings = Vec::new();
        for (line_idx, line) in source.lines().enumerate() {
            for (pat, desc) in Self::debug_patterns() {
                if line.contains(pat) {
                    findings
                        .push(
                            SecurityFinding::new(
                                SecurityIssue::ExposedPrivateData,
                                SecuritySeverity::Low,
                                &format!("line:{}", line_idx + 1),
                                desc,
                            ),
                        );
                }
            }
        }
        findings
    }
}
/// Helper that locates and counts `sorry` occurrences in source text.
pub struct SorryTracker;
impl SorryTracker {
    /// Create a new tracker.
    pub fn new() -> Self {
        Self
    }
    /// Count the number of `sorry` tokens in `source`.
    pub fn count_sorries(source: &str) -> usize {
        source.split("sorry").count().saturating_sub(1)
    }
    /// Return a list of location strings for each `sorry` occurrence.
    ///
    /// Each entry has the form `"line:<n>"` (1-based).
    pub fn sorry_locations(source: &str) -> Vec<String> {
        let mut locations = Vec::new();
        for (line_idx, line) in source.lines().enumerate() {
            let mut remaining = line;
            while let Some(pos) = remaining.find("sorry") {
                let after = &remaining[pos + "sorry".len()..];
                let before_ok = pos == 0
                    || {
                        let ch = remaining.as_bytes()[pos - 1] as char;
                        !ch.is_alphanumeric() && ch != '_'
                    };
                let after_ok = after
                    .chars()
                    .next()
                    .map(|c| !c.is_alphanumeric() && c != '_')
                    .unwrap_or(true);
                if before_ok && after_ok {
                    locations.push(format!("line:{}", line_idx + 1));
                }
                remaining = &remaining[pos + "sorry".len()..];
            }
        }
        locations
    }
    /// Returns `true` if `sorry` appears inside a proof body
    /// (heuristic: after `:= by` or `:=`).
    pub fn is_sorry_in_proof(source: &str) -> bool {
        let keywords = [":= by", ":="];
        for kw in &keywords {
            if let Some(pos) = source.find(kw) {
                let after = &source[pos..];
                if after.contains("sorry") {
                    return true;
                }
            }
        }
        false
    }
}
/// Audits usage of unsafe Rust APIs.
#[allow(dead_code)]
pub struct UnsafeApiAuditor;
impl UnsafeApiAuditor {
    /// Known unsafe APIs or patterns.
    #[allow(dead_code)]
    fn unsafe_apis() -> Vec<(&'static str, SecuritySeverity, &'static str)> {
        vec![
            ("std::mem::transmute", SecuritySeverity::Critical,
            "transmute can violate type safety"), ("std::mem::forget",
            SecuritySeverity::Medium, "forget can cause memory leaks"), ("ptr::read",
            SecuritySeverity::High, "raw pointer read may access invalid memory"),
            ("ptr::write", SecuritySeverity::High,
            "raw pointer write may corrupt memory"), ("slice::from_raw_parts",
            SecuritySeverity::High, "raw slice construction may be unsound"),
            ("std::hint::unreachable_unchecked", SecuritySeverity::Critical,
            "unreachable_unchecked causes UB if reachable"), ("unsafe {",
            SecuritySeverity::Medium, "unsafe block detected"),
        ]
    }
    /// Scan source for unsafe API usage.
    #[allow(dead_code)]
    pub fn scan(source: &str) -> Vec<SecurityFinding> {
        let mut findings = Vec::new();
        for (line_idx, line) in source.lines().enumerate() {
            for (api, severity, desc) in Self::unsafe_apis() {
                if line.contains(api) {
                    findings
                        .push(
                            SecurityFinding::new(
                                    SecurityIssue::DangerousFfi,
                                    severity,
                                    &format!("line:{}", line_idx + 1),
                                    desc,
                                )
                                .with_suggestion(
                                    "Audit this unsafe code carefully; add a safety comment explaining invariants.",
                                ),
                        );
                }
            }
        }
        findings
    }
}
/// Tracks how many security "credits" have been used by acceptable risk.
#[allow(dead_code)]
pub struct SecurityBudget {
    pub max_risk: f64,
    accumulated_risk: f64,
}
impl SecurityBudget {
    #[allow(dead_code)]
    pub fn new(max_risk: f64) -> Self {
        Self {
            max_risk,
            accumulated_risk: 0.0,
        }
    }
    /// Accept a security finding and accumulate its risk weight.
    /// Returns false if the budget is exceeded.
    #[allow(dead_code)]
    pub fn accept(&mut self, finding: &SecurityFinding) -> bool {
        let weight = match finding.severity {
            SecuritySeverity::Critical => 0.5,
            SecuritySeverity::High => 0.2,
            SecuritySeverity::Medium => 0.1,
            SecuritySeverity::Low => 0.03,
            SecuritySeverity::Info => 0.01,
        };
        self.accumulated_risk += weight;
        self.accumulated_risk <= self.max_risk
    }
    /// Remaining budget.
    #[allow(dead_code)]
    pub fn remaining(&self) -> f64 {
        (self.max_risk - self.accumulated_risk).max(0.0)
    }
    /// Whether budget is exhausted.
    #[allow(dead_code)]
    pub fn is_exhausted(&self) -> bool {
        self.accumulated_risk > self.max_risk
    }
}
/// A single security finding emitted by the security lint pass.
#[derive(Clone, Debug)]
pub struct SecurityFinding {
    /// The kind of security issue.
    pub issue: SecurityIssue,
    /// How severe the issue is.
    pub severity: SecuritySeverity,
    /// Human-readable location string (e.g., "file.ox:42").
    pub location: String,
    /// Description of what was found.
    pub message: String,
    /// Optional remediation suggestion.
    pub suggestion: Option<String>,
}
impl SecurityFinding {
    /// Create a new finding without a suggestion.
    pub fn new(
        issue: SecurityIssue,
        severity: SecuritySeverity,
        loc: &str,
        msg: &str,
    ) -> Self {
        Self {
            issue,
            severity,
            location: loc.to_string(),
            message: msg.to_string(),
            suggestion: None,
        }
    }
    /// Attach a remediation suggestion and return `self`.
    pub fn with_suggestion(mut self, sug: &str) -> Self {
        self.suggestion = Some(sug.to_string());
        self
    }
    /// Returns `true` when the severity is `Critical`.
    pub fn is_critical(&self) -> bool {
        self.severity == SecuritySeverity::Critical
    }
}
/// A collection of named security rules with their descriptions.
#[allow(dead_code)]
pub struct SecurityRuleBook {
    rules: HashMap<String, String>,
}
impl SecurityRuleBook {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let mut rules = HashMap::new();
        rules
            .insert(
                "no_sorry".to_string(),
                "All proofs must be complete — no sorry allowed.".to_string(),
            );
        rules
            .insert(
                "no_unsound_axiom".to_string(),
                "No unsound axioms may be introduced.".to_string(),
            );
        rules
            .insert(
                "no_bare_ffi".to_string(),
                "FFI usage must have a safety justification comment.".to_string(),
            );
        rules
            .insert(
                "no_hardcoded_secrets".to_string(),
                "No credentials may be hard-coded in source.".to_string(),
            );
        rules
            .insert(
                "no_path_traversal".to_string(),
                "Path construction must not use user-controlled segments.".to_string(),
            );
        Self { rules }
    }
    /// Add a custom rule.
    #[allow(dead_code)]
    pub fn add_rule(&mut self, name: &str, description: &str) {
        self.rules.insert(name.to_string(), description.to_string());
    }
    /// Look up a rule description.
    #[allow(dead_code)]
    pub fn get_rule(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(|s| s.as_str())
    }
    /// Return all rule names sorted.
    #[allow(dead_code)]
    pub fn rule_names(&self) -> Vec<&str> {
        let mut names: Vec<&str> = self.rules.keys().map(|s| s.as_str()).collect();
        names.sort();
        names
    }
}
/// Runs all security analyzers and produces a combined `SecurityReport`.
#[allow(dead_code)]
pub struct FullSecurityAnalyzer {
    pub pass: SecurityLintPass,
}
impl FullSecurityAnalyzer {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            pass: SecurityLintPass::new(),
        }
    }
    /// Run the full analysis.
    #[allow(dead_code)]
    pub fn analyze(&self, source: &str) -> SecurityReport {
        let mut findings = self.pass.run_all(source);
        findings.extend(InjectionVulnerabilityDetector::scan(source));
        findings.extend(CredentialLeakDetector::scan(source));
        findings.extend(PathTraversalChecker::scan(source));
        findings.extend(UnsafeApiAuditor::scan(source));
        SecurityReport::from_findings(findings)
    }
}
/// Checks that user-controlled inputs are sanitized before use.
#[allow(dead_code)]
pub struct SanitizerChecker;
impl SanitizerChecker {
    /// Look for user-input variables used directly in string formatting or concatenation
    /// without any sanitization call in between.
    #[allow(dead_code)]
    pub fn check_unsanitized_inputs(source: &str) -> Vec<SecurityFinding> {
        let input_vars = ["user_input", "request_body", "raw_input", "untrusted"];
        let sanitizers = ["sanitize", "escape", "validate", "strip", "clean"];
        let mut findings = Vec::new();
        for (line_idx, line) in source.lines().enumerate() {
            let has_input = input_vars.iter().any(|v| line.contains(v));
            let has_sanitizer = sanitizers.iter().any(|s| line.contains(s));
            if has_input && !has_sanitizer {
                findings
                    .push(
                        SecurityFinding::new(
                                SecurityIssue::UncheckedInput,
                                SecuritySeverity::Medium,
                                &format!("line:{}", line_idx + 1),
                                "User-controlled input used without sanitization",
                            )
                            .with_suggestion("Apply input sanitization before use."),
                    );
            }
        }
        findings
    }
}
/// Detects "verification gaps" — theorems stated but not proved.
#[allow(dead_code)]
pub struct VerificationGapDetector;
impl VerificationGapDetector {
    /// Returns the number of sorry-proved theorems (verification gaps).
    #[allow(dead_code)]
    pub fn count_gaps(source: &str) -> usize {
        source
            .lines()
            .filter(|l| {
                let t = l.trim();
                (t.starts_with("theorem ") || t.starts_with("lemma "))
                    && t.contains("sorry")
            })
            .count()
    }
    /// Returns a gap ratio: 0.0 = all proved, 1.0 = all sorry.
    #[allow(dead_code)]
    pub fn gap_ratio(source: &str) -> f64 {
        let total = source
            .lines()
            .filter(|l| {
                let t = l.trim();
                t.starts_with("theorem ") || t.starts_with("lemma ")
            })
            .count();
        if total == 0 {
            return 0.0;
        }
        let gaps = Self::count_gaps(source);
        gaps as f64 / total as f64
    }
}
/// A single security audit entry.
#[allow(dead_code)]
pub struct SecurityAuditEntry {
    pub id: u64,
    pub severity: SecuritySeverity,
    pub message: String,
    pub resolved: bool,
}
/// Checks the ratio of sorry proofs per declaration.
#[allow(dead_code)]
pub struct SorryDensityChecker {
    pub max_density: f64,
}
impl SorryDensityChecker {
    #[allow(dead_code)]
    pub fn new(max_density: f64) -> Self {
        Self { max_density }
    }
    /// Compute sorry density: sorries / total declarations.
    #[allow(dead_code)]
    pub fn density(source: &str) -> f64 {
        let decls = source
            .lines()
            .filter(|l| {
                let t = l.trim();
                t.starts_with("theorem ") || t.starts_with("lemma ")
                    || t.starts_with("def ") || t.starts_with("axiom ")
            })
            .count();
        if decls == 0 {
            return 0.0;
        }
        let sorries = SorryTracker::count_sorries(source);
        sorries as f64 / decls as f64
    }
    /// Emit a finding if the density exceeds the threshold.
    #[allow(dead_code)]
    pub fn check(&self, source: &str) -> Vec<SecurityFinding> {
        let d = Self::density(source);
        if d > self.max_density {
            vec![
                SecurityFinding::new(SecurityIssue::WeakAssumption,
                SecuritySeverity::Medium, "source", &
                format!("Sorry density {:.2} exceeds threshold {:.2}; proof coverage is low",
                d, self.max_density),)
            ]
        } else {
            Vec::new()
        }
    }
}
/// Audits imported modules for potentially unsafe dependencies.
#[allow(dead_code)]
pub struct DependencyAuditor {
    /// List of import prefixes considered high-risk.
    pub risky_prefixes: Vec<String>,
}
impl DependencyAuditor {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            risky_prefixes: vec![
                "Mathlib.Tactic.Polyrith".to_string(), "Mathlib.Tactic.Polyrith"
                .to_string(), "Unsafe".to_string(), "System.IO".to_string(),
            ],
        }
    }
    /// Add a risky import prefix.
    #[allow(dead_code)]
    pub fn add_risky_prefix(&mut self, prefix: &str) {
        self.risky_prefixes.push(prefix.to_string());
    }
    /// Check import lines for risky prefixes.
    #[allow(dead_code)]
    pub fn check(&self, source: &str) -> Vec<SecurityFinding> {
        let mut findings = Vec::new();
        for (line_idx, line) in source.lines().enumerate() {
            let t = line.trim();
            if t.starts_with("import ") {
                let import_path = &t["import ".len()..];
                for prefix in &self.risky_prefixes {
                    if import_path.starts_with(prefix.as_str()) {
                        findings
                            .push(
                                SecurityFinding::new(
                                        SecurityIssue::UnverifiedExternal,
                                        SecuritySeverity::Medium,
                                        &format!("line:{}", line_idx + 1),
                                        &format!(
                                            "Import of potentially risky module: `{}`", import_path
                                        ),
                                    )
                                    .with_suggestion(
                                        "Audit this import for security implications.",
                                    ),
                            );
                    }
                }
            }
        }
        findings
    }
}
/// Tracks security findings across multiple analysis runs to spot trends.
#[allow(dead_code)]
pub struct SecurityTrendTracker {
    snapshots: Vec<(String, SecurityReport)>,
}
impl SecurityTrendTracker {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { snapshots: Vec::new() }
    }
    /// Record a new security report snapshot under a label.
    #[allow(dead_code)]
    pub fn record(&mut self, label: &str, report: SecurityReport) {
        self.snapshots.push((label.to_string(), report));
    }
    /// Number of snapshots recorded.
    #[allow(dead_code)]
    pub fn snapshot_count(&self) -> usize {
        self.snapshots.len()
    }
    /// Latest risk score or 0.0 if no snapshots.
    #[allow(dead_code)]
    pub fn latest_risk_score(&self) -> f64 {
        self.snapshots.last().map(|(_, r)| r.risk_score()).unwrap_or(0.0)
    }
    /// Returns `true` when the latest snapshot has a lower risk score than the previous.
    #[allow(dead_code)]
    pub fn is_improving(&self) -> bool {
        if self.snapshots.len() < 2 {
            return false;
        }
        let prev = self.snapshots[self.snapshots.len() - 2].1.risk_score();
        let latest = self.snapshots[self.snapshots.len() - 1].1.risk_score();
        latest < prev
    }
}
/// Aggregated security report for a source file.
#[allow(dead_code)]
pub struct SecurityReport {
    pub findings: Vec<SecurityFinding>,
    pub critical_count: usize,
    pub high_count: usize,
    pub medium_count: usize,
    pub low_count: usize,
    pub info_count: usize,
}
impl SecurityReport {
    /// Build a report from a list of findings.
    #[allow(dead_code)]
    pub fn from_findings(findings: Vec<SecurityFinding>) -> Self {
        let critical_count = findings
            .iter()
            .filter(|f| f.severity == SecuritySeverity::Critical)
            .count();
        let high_count = findings
            .iter()
            .filter(|f| f.severity == SecuritySeverity::High)
            .count();
        let medium_count = findings
            .iter()
            .filter(|f| f.severity == SecuritySeverity::Medium)
            .count();
        let low_count = findings
            .iter()
            .filter(|f| f.severity == SecuritySeverity::Low)
            .count();
        let info_count = findings
            .iter()
            .filter(|f| f.severity == SecuritySeverity::Info)
            .count();
        Self {
            findings,
            critical_count,
            high_count,
            medium_count,
            low_count,
            info_count,
        }
    }
    /// True when no findings exist.
    #[allow(dead_code)]
    pub fn is_clean(&self) -> bool {
        self.findings.is_empty()
    }
    /// Overall risk score in [0.0, 1.0]: higher means more risk.
    #[allow(dead_code)]
    pub fn risk_score(&self) -> f64 {
        let score = self.critical_count as f64 * 0.5 + self.high_count as f64 * 0.2
            + self.medium_count as f64 * 0.1 + self.low_count as f64 * 0.03
            + self.info_count as f64 * 0.01;
        score.min(1.0)
    }
    /// Returns `true` when there is at least one Critical finding.
    #[allow(dead_code)]
    pub fn has_blocker(&self) -> bool {
        self.critical_count > 0
    }
    /// Return findings sorted by severity (Critical first).
    #[allow(dead_code)]
    pub fn sorted_by_severity(&self) -> Vec<&SecurityFinding> {
        let mut sorted: Vec<&SecurityFinding> = self.findings.iter().collect();
        sorted.sort_by(|a, b| b.severity.cmp(&a.severity));
        sorted
    }
}
/// Classifies source files by their overall risk level based on findings.
#[allow(dead_code)]
pub struct RiskClassifier;
impl RiskClassifier {
    /// Classify a report into a risk category.
    #[allow(dead_code)]
    pub fn classify(report: &SecurityReport) -> &'static str {
        let score = report.risk_score();
        if score >= 0.8 {
            "critical"
        } else if score >= 0.5 {
            "high"
        } else if score >= 0.2 {
            "medium"
        } else if score > 0.0 {
            "low"
        } else {
            "clean"
        }
    }
}
/// Estimates the entropy of string literals to detect low-entropy "secrets".
#[allow(dead_code)]
pub struct SecretEntropyEstimator;
impl SecretEntropyEstimator {
    /// Calculate Shannon entropy (bits per character) of a string.
    #[allow(dead_code)]
    pub fn entropy(s: &str) -> f64 {
        if s.is_empty() {
            return 0.0;
        }
        let mut freq: HashMap<char, usize> = HashMap::new();
        for ch in s.chars() {
            *freq.entry(ch).or_insert(0) += 1;
        }
        let len = s.len() as f64;
        freq.values()
            .map(|&count| {
                let p = count as f64 / len;
                -p * p.log2()
            })
            .sum()
    }
    /// Return `true` when the entropy suggests a realistic secret (>= 3.5 bits/char).
    #[allow(dead_code)]
    pub fn looks_like_secret(s: &str) -> bool {
        s.len() >= 8 && Self::entropy(s) >= 3.5
    }
    /// Find string literals in source that look like high-entropy secrets.
    #[allow(dead_code)]
    pub fn find_high_entropy_strings(source: &str) -> Vec<(usize, f64)> {
        let mut results = Vec::new();
        for (line_idx, line) in source.lines().enumerate() {
            let mut rest = line;
            while let Some(start) = rest.find('"') {
                let inner_start = start + 1;
                if inner_start >= rest.len() {
                    break;
                }
                let inner = &rest[inner_start..];
                if let Some(end) = inner.find('"') {
                    let literal = &inner[..end];
                    if Self::looks_like_secret(literal) {
                        results.push((line_idx + 1, Self::entropy(literal)));
                    }
                    rest = &rest[inner_start + end + 1..];
                } else {
                    break;
                }
            }
        }
        results
    }
}
/// Simple string-based taint analysis: tracks which variables are "tainted"
/// (received from an external source) and warns when they flow into sensitive sinks.
#[allow(dead_code)]
pub struct TaintAnalyzer {
    /// Names of sources that introduce taint (e.g., "user_input", "env_var").
    pub taint_sources: Vec<String>,
    /// Names of sinks where tainted data should not flow (e.g., "eval", "exec").
    pub taint_sinks: Vec<String>,
}
impl TaintAnalyzer {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            taint_sources: vec![
                "user_input".to_string(), "env_var".to_string(), "read_line".to_string(),
                "stdin".to_string(), "args".to_string(),
            ],
            taint_sinks: vec![
                "eval".to_string(), "exec".to_string(), "system".to_string(), "unsafe "
                .to_string(), "transmute".to_string(),
            ],
        }
    }
    /// Check whether `source` contains both a taint source and a sink on the
    /// same line (very naive heuristic).
    #[allow(dead_code)]
    pub fn check_for_taint_flow(&self, source: &str) -> Vec<SecurityFinding> {
        let mut findings = Vec::new();
        for (line_idx, line) in source.lines().enumerate() {
            let has_source = self
                .taint_sources
                .iter()
                .any(|s| line.contains(s.as_str()));
            let has_sink = self.taint_sinks.iter().any(|s| line.contains(s.as_str()));
            if has_source && has_sink {
                findings
                    .push(
                        SecurityFinding::new(
                            SecurityIssue::UncheckedInput,
                            SecuritySeverity::Critical,
                            &format!("line:{}", line_idx + 1),
                            "Tainted data flows from source to sensitive sink on same line",
                        ),
                    );
            }
        }
        findings
    }
    /// Add a custom taint source.
    #[allow(dead_code)]
    pub fn add_source(&mut self, source_name: &str) {
        self.taint_sources.push(source_name.to_string());
    }
    /// Add a custom taint sink.
    #[allow(dead_code)]
    pub fn add_sink(&mut self, sink_name: &str) {
        self.taint_sinks.push(sink_name.to_string());
    }
}
/// The main security lint pass.
pub struct SecurityLintPass {
    config: SecurityLintConfig,
}
impl SecurityLintPass {
    /// Create a pass with the default configuration.
    pub fn new() -> Self {
        Self {
            config: SecurityLintConfig::default(),
        }
    }
    /// Create a pass with a custom configuration.
    pub fn with_config(config: SecurityLintConfig) -> Self {
        Self { config }
    }
    /// Scan source for `sorry` and emit findings.
    pub fn check_for_sorry(&self, source: &str) -> Vec<SecurityFinding> {
        if !self.config.check_sorry {
            return Vec::new();
        }
        let locations = SorryTracker::sorry_locations(source);
        locations
            .into_iter()
            .map(|loc| {
                SecurityFinding::new(
                        SecurityIssue::WeakAssumption,
                        SecuritySeverity::High,
                        &loc,
                        "`sorry` placeholder found — proof is incomplete",
                    )
                    .with_suggestion("Replace `sorry` with a real proof term.")
            })
            .collect()
    }
    /// Scan for unsafe or unsound axiom declarations.
    pub fn check_for_unsafe_axioms(&self, source: &str) -> Vec<SecurityFinding> {
        if !self.config.check_axioms {
            return Vec::new();
        }
        let mut findings = Vec::new();
        for (line_idx, line) in source.lines().enumerate() {
            let trimmed = line.trim();
            if trimmed.starts_with("axiom ") || trimmed.starts_with("unsafe axiom ") {
                findings
                    .push(
                        SecurityFinding::new(
                                SecurityIssue::UnsoundAxiom,
                                SecuritySeverity::Critical,
                                &format!("line:{}", line_idx + 1),
                                &format!("Axiom declaration found: `{}`", trimmed),
                            )
                            .with_suggestion(
                                "Audit this axiom carefully; unsound axioms break the proof system.",
                            ),
                    );
            }
        }
        findings
    }
    /// Scan for Classical.choice usage.
    pub fn check_for_classical_choice(&self, source: &str) -> Vec<SecurityFinding> {
        if !self.config.check_classical {
            return Vec::new();
        }
        let mut findings = Vec::new();
        let needles = ["Classical.choice", "Classical.em", "propext"];
        for (line_idx, line) in source.lines().enumerate() {
            for needle in &needles {
                if line.contains(needle) {
                    let issue = if *needle == "propext" {
                        SecurityIssue::PropExtMisuse
                    } else {
                        SecurityIssue::ClassicalChoice
                    };
                    findings
                        .push(
                            SecurityFinding::new(
                                    issue,
                                    SecuritySeverity::Medium,
                                    &format!("line:{}", line_idx + 1),
                                    &format!("Classical reasoning `{}` detected", needle),
                                )
                                .with_suggestion(
                                    "Prefer constructive proofs where possible.",
                                ),
                        );
                }
            }
        }
        findings
    }
    /// Detect potential circular imports in a list of module paths.
    pub fn check_for_circular_imports(
        &self,
        imports: &[String],
    ) -> Vec<SecurityFinding> {
        let mut findings = Vec::new();
        let mut seen = std::collections::HashSet::new();
        for import in imports {
            if !seen.insert(import.clone()) {
                findings
                    .push(
                        SecurityFinding::new(
                            SecurityIssue::CircularProof,
                            SecuritySeverity::High,
                            "imports",
                            &format!("Duplicate/circular import detected: `{}`", import),
                        ),
                    );
            }
        }
        findings
    }
    /// Scan for FFI-related keywords.
    pub fn check_ffi_usage(&self, source: &str) -> Vec<SecurityFinding> {
        if !self.config.check_ffi {
            return Vec::new();
        }
        let mut findings = Vec::new();
        let ffi_keywords = ["@[extern", "extern \"C\"", "#[link(", "unsafe extern"];
        for (line_idx, line) in source.lines().enumerate() {
            for kw in &ffi_keywords {
                if line.contains(kw) {
                    findings
                        .push(
                            SecurityFinding::new(
                                    SecurityIssue::DangerousFfi,
                                    SecuritySeverity::High,
                                    &format!("line:{}", line_idx + 1),
                                    &format!("FFI usage `{}` detected", kw),
                                )
                                .with_suggestion(
                                    "Ensure FFI bindings are safe and verified.",
                                ),
                        );
                }
            }
        }
        findings
    }
    /// Run all enabled checks and return combined findings.
    pub fn run_all(&self, source: &str) -> Vec<SecurityFinding> {
        let mut findings = Vec::new();
        findings.extend(self.check_for_sorry(source));
        findings.extend(self.check_for_unsafe_axioms(source));
        findings.extend(self.check_for_classical_choice(source));
        findings.extend(self.check_ffi_usage(source));
        findings
    }
    /// Count findings grouped by severity name.
    pub fn total_by_severity(
        &self,
        findings: &[SecurityFinding],
    ) -> HashMap<String, usize> {
        let mut map: HashMap<String, usize> = HashMap::new();
        for f in findings {
            *map.entry(f.severity.to_string()).or_insert(0) += 1;
        }
        map
    }
}
/// Categories of security issues detected by this lint pass.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum SecurityIssue {
    /// Unchecked external or user input.
    UncheckedInput,
    /// Private data exposed to external callers.
    ExposedPrivateData,
    /// A weak or unjustified assumption.
    WeakAssumption,
    /// Circular proof dependency.
    CircularProof,
    /// Unsound axiom introduced.
    UnsoundAxiom,
    /// Use of Classical.choice without justification.
    ClassicalChoice,
    /// Misuse of propositional extensionality.
    PropExtMisuse,
    /// Unsafe foreign-function interface call.
    DangerousFfi,
    /// External lemma used without verification.
    UnverifiedExternal,
}
/// Detects hard-coded credentials or secrets in source text.
#[allow(dead_code)]
pub struct CredentialLeakDetector;
impl CredentialLeakDetector {
    /// Patterns suggesting hard-coded secrets.
    #[allow(dead_code)]
    fn secret_patterns() -> Vec<(&'static str, &'static str)> {
        vec![
            ("password", "Hard-coded password detected"), ("secret_key",
            "Hard-coded secret key detected"), ("api_key",
            "Hard-coded API key detected"), ("private_key",
            "Hard-coded private key detected"), ("access_token",
            "Hard-coded access token detected"), ("auth_token",
            "Hard-coded auth token detected"), ("BEGIN RSA PRIVATE KEY",
            "RSA private key literal detected"), ("BEGIN EC PRIVATE KEY",
            "EC private key literal detected"),
        ]
    }
    /// Scan `source` for credential leaks.
    #[allow(dead_code)]
    pub fn scan(source: &str) -> Vec<SecurityFinding> {
        let mut findings = Vec::new();
        let lower = source.to_lowercase();
        for (line_idx, (line, lower_line)) in source
            .lines()
            .zip(lower.lines())
            .enumerate()
        {
            for (pat, desc) in Self::secret_patterns() {
                if lower_line.contains(pat) {
                    if line.contains('=') || line.contains('"') || line.contains('\'') {
                        findings
                            .push(
                                SecurityFinding::new(
                                        SecurityIssue::ExposedPrivateData,
                                        SecuritySeverity::Critical,
                                        &format!("line:{}", line_idx + 1),
                                        desc,
                                    )
                                    .with_suggestion(
                                        "Remove hard-coded credentials; use environment variables or secret managers.",
                                    ),
                            );
                    }
                }
            }
        }
        findings
    }
}
/// Severity level for security findings.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SecuritySeverity {
    /// Must be fixed before any merge.
    Critical,
    /// Should be fixed.
    High,
    /// Worth investigating.
    Medium,
    /// Minor concern.
    Low,
    /// Informational only.
    Info,
}
