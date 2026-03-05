//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)


/// The main performance lint pass.
pub struct PerfLintPass {
    config: PerfLintConfig,
}
impl PerfLintPass {
    /// Create a pass with the default configuration.
    pub fn new() -> Self {
        Self {
            config: PerfLintConfig::default(),
        }
    }
    /// Create a pass with a custom configuration.
    pub fn with_config(config: PerfLintConfig) -> Self {
        Self { config }
    }
    /// Warn when the proof term (tokens between `:=` and end of source) is large.
    pub fn check_proof_term_size(&self, source: &str) -> Vec<PerfFinding> {
        let mut findings = Vec::new();
        let token_count = source.split_whitespace().count();
        if token_count > self.config.max_proof_term_size {
            findings
                .push(
                    PerfFinding::new(
                            PerfIssue::LargeProofTerm,
                            PerfSeverity::Warning,
                            "source",
                            &format!(
                                "Source has {} tokens (threshold: {}); proof term may be large",
                                token_count, self.config.max_proof_term_size
                            ),
                        )
                        .with_impact(
                            (token_count as f64
                                / (self.config.max_proof_term_size as f64 * 2.0))
                                .min(1.0),
                        ),
                );
        }
        findings
    }
    /// Warn when `simp` is invoked with a very large lemma list.
    pub fn check_simp_config(&self, source: &str) -> Vec<PerfFinding> {
        let mut findings = Vec::new();
        for (line_idx, line) in source.lines().enumerate() {
            if let Some(start) = line.find("simp [") {
                let after = &line[start..];
                if let Some(end) = after.find(']') {
                    let lemmas = after[..end].matches(',').count() + 1;
                    if lemmas > self.config.max_simp_lemmas {
                        findings
                            .push(
                                PerfFinding::new(
                                        PerfIssue::SlowSimpLemma,
                                        PerfSeverity::Warning,
                                        &format!("line:{}", line_idx + 1),
                                        &format!(
                                            "`simp` called with {} lemmas (threshold: {})", lemmas, self
                                            .config.max_simp_lemmas
                                        ),
                                    )
                                    .with_impact(0.6),
                            );
                    }
                }
            }
        }
        findings
    }
    /// Detect potentially expensive instance searches (many typeclass constraints).
    pub fn check_instance_search(&self, source: &str) -> Vec<PerfFinding> {
        let mut findings = Vec::new();
        for (line_idx, line) in source.lines().enumerate() {
            let instance_args = line.matches('[').count();
            if instance_args >= 5 {
                findings
                    .push(
                        PerfFinding::new(
                                PerfIssue::ExpensiveInstanceSearch,
                                PerfSeverity::Warning,
                                &format!("line:{}", line_idx + 1),
                                &format!(
                                    "Line has {} potential instance arguments; may trigger expensive search",
                                    instance_args
                                ),
                            )
                            .with_impact(0.5),
                    );
            }
        }
        findings
    }
    /// Detect deeply nested recursion patterns.
    pub fn check_recursion_depth(&self, source: &str) -> Vec<PerfFinding> {
        let mut findings = Vec::new();
        let mut max_depth = 0usize;
        let mut depth = 0usize;
        for ch in source.chars() {
            match ch {
                '(' => {
                    depth += 1;
                    max_depth = max_depth.max(depth);
                }
                ')' => {
                    depth = depth.saturating_sub(1);
                }
                _ => {}
            }
        }
        if max_depth > self.config.max_recursion_depth {
            findings
                .push(
                    PerfFinding::new(
                            PerfIssue::DeepRecursion,
                            PerfSeverity::Warning,
                            "source",
                            &format!(
                                "Maximum nesting depth {} exceeds threshold {}", max_depth,
                                self.config.max_recursion_depth
                            ),
                        )
                        .with_impact(
                            (max_depth as f64
                                / (self.config.max_recursion_depth as f64 * 2.0))
                                .min(1.0),
                        ),
                );
        }
        findings
    }
    /// Estimate overall source complexity on a 0.0–1.0 scale.
    pub fn estimate_complexity(&self, source: &str) -> f64 {
        ElabComplexityEstimator::overall_score(source)
    }
    /// Run all performance checks and return combined findings.
    pub fn run_all(&self, source: &str) -> Vec<PerfFinding> {
        let mut findings = Vec::new();
        findings.extend(self.check_proof_term_size(source));
        findings.extend(self.check_simp_config(source));
        findings.extend(self.check_instance_search(source));
        findings.extend(self.check_recursion_depth(source));
        findings
    }
    /// Return findings sorted by estimated impact (descending).
    pub fn prioritized_findings<'a>(
        &self,
        findings: &'a [PerfFinding],
    ) -> Vec<&'a PerfFinding> {
        let mut sorted: Vec<&PerfFinding> = findings.iter().collect();
        sorted
            .sort_by(|a, b| {
                b.estimated_impact
                    .partial_cmp(&a.estimated_impact)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
        sorted
    }
}
/// Categories of performance issues detected by this lint pass.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PerfIssue {
    /// A `simp` lemma that is known to be slow.
    SlowSimpLemma,
    /// A proof term that is unusually large.
    LargeProofTerm,
    /// Recursion depth that may cause a stack overflow.
    DeepRecursion,
    /// Instance search that is likely expensive.
    ExpensiveInstanceSearch,
    /// A proof search with no depth bound.
    UnboundedSearch,
    /// An inefficient match/pattern structure.
    InefficientPattern,
    /// Too many open metavariables.
    ExcessiveMetavars,
    /// An inductive type with an extremely large number of constructors.
    LargeInductiveType,
}
/// A budget that tracks how many expensive operations have been used.
#[allow(dead_code)]
pub struct ComplexityBudget {
    pub max_cost: f64,
    current_cost: f64,
}
impl ComplexityBudget {
    #[allow(dead_code)]
    pub fn new(max_cost: f64) -> Self {
        Self {
            max_cost,
            current_cost: 0.0,
        }
    }
    /// Spend `cost` units.  Returns `false` if the budget is exceeded.
    #[allow(dead_code)]
    pub fn spend(&mut self, cost: f64) -> bool {
        self.current_cost += cost;
        self.current_cost <= self.max_cost
    }
    /// Remaining budget.
    #[allow(dead_code)]
    pub fn remaining(&self) -> f64 {
        (self.max_cost - self.current_cost).max(0.0)
    }
    /// Whether the budget has been exhausted.
    #[allow(dead_code)]
    pub fn is_exhausted(&self) -> bool {
        self.current_cost > self.max_cost
    }
    /// Reset the budget to zero spent.
    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.current_cost = 0.0;
    }
}
/// Counts and analyses `simp` tactic invocations.
#[allow(dead_code)]
pub struct SimpCallCounter;
impl SimpCallCounter {
    /// Return the total number of `simp` calls in `source`.
    #[allow(dead_code)]
    pub fn total_simp_calls(source: &str) -> usize {
        source
            .lines()
            .filter(|l| {
                let t = l.trim();
                t.starts_with("simp") || t.contains(" simp") || t.contains("\tsimp")
            })
            .count()
    }
    /// Return the number of `simp only [...]` calls.
    #[allow(dead_code)]
    pub fn simp_only_calls(source: &str) -> usize {
        source.matches("simp only [").count()
    }
    /// Compute the ratio of `simp only` to total `simp` calls.
    /// Returns `0.0` when there are no simp calls.
    #[allow(dead_code)]
    pub fn simp_only_ratio(source: &str) -> f64 {
        let total = Self::total_simp_calls(source);
        if total == 0 {
            return 0.0;
        }
        let only = Self::simp_only_calls(source);
        only as f64 / total as f64
    }
}
/// Detects heavily-constrained polymorphic definitions that may trigger
/// expensive type-class searches.
#[allow(dead_code)]
pub struct TypeClassComplexityChecker {
    pub max_constraints: usize,
}
impl TypeClassComplexityChecker {
    #[allow(dead_code)]
    pub fn new(max_constraints: usize) -> Self {
        Self { max_constraints }
    }
    /// Count the number of `[TypeClass ...]` constraints on a single line.
    #[allow(dead_code)]
    pub fn count_constraints_on_line(line: &str) -> usize {
        let mut count = 0usize;
        let chars: Vec<char> = line.chars().collect();
        for i in 0..chars.len() {
            if chars[i] == '[' {
                if let Some(&next) = chars.get(i + 1) {
                    if next.is_uppercase() {
                        count += 1;
                    }
                }
            }
        }
        count
    }
    /// Emit findings for definitions with too many type-class constraints.
    #[allow(dead_code)]
    pub fn check(&self, source: &str) -> Vec<PerfFinding> {
        let mut findings = Vec::new();
        for (line_idx, line) in source.lines().enumerate() {
            let count = Self::count_constraints_on_line(line);
            if count > self.max_constraints {
                findings
                    .push(
                        PerfFinding::new(
                                PerfIssue::ExpensiveInstanceSearch,
                                PerfSeverity::Warning,
                                &format!("line:{}", line_idx + 1),
                                &format!(
                                    "{} typeclass constraints on line (threshold: {})", count,
                                    self.max_constraints
                                ),
                            )
                            .with_impact(0.45),
                    );
            }
        }
        findings
    }
}
/// Provides O(n) vs O(1) suggestions for common patterns.
#[allow(dead_code)]
pub struct OComplexityAdvisor;
impl OComplexityAdvisor {
    /// Returns suggestions for patterns that have more efficient alternatives.
    #[allow(dead_code)]
    pub fn suggest(source: &str) -> Vec<String> {
        let mut suggestions = Vec::new();
        let patterns: Vec<(&str, &str)> = vec![
            (".contains(", "Consider using a HashSet for O(1) membership tests"),
            (".position(", "Consider sorting + binary_search for repeated lookups"),
            (".find(", "Consider indexing by key with a HashMap"), ("Vec::new(); for",
            "Consider collecting with .map().collect()"), (".push(",
            "If capacity is known, use Vec::with_capacity"),
        ];
        for (pat, advice) in &patterns {
            if source.contains(pat) {
                suggestions.push(advice.to_string());
            }
        }
        suggestions
    }
}
/// Analyses potential depth of proof search (e.g., `apply` chains).
#[allow(dead_code)]
pub struct ProofSearchDepthAnalyzer {
    pub max_depth: usize,
}
impl ProofSearchDepthAnalyzer {
    #[allow(dead_code)]
    pub fn new(max_depth: usize) -> Self {
        Self { max_depth }
    }
    /// Count `apply` calls in source as a proxy for proof-search depth.
    #[allow(dead_code)]
    pub fn count_apply_calls(source: &str) -> usize {
        source
            .lines()
            .filter(|l| {
                let t = l.trim();
                t.starts_with("apply ") || t.starts_with("exact ")
                    || t.starts_with("refine ")
            })
            .count()
    }
    /// Emit findings when apply call count exceeds threshold.
    #[allow(dead_code)]
    pub fn check(&self, source: &str) -> Vec<PerfFinding> {
        let count = Self::count_apply_calls(source);
        if count > self.max_depth {
            vec![
                PerfFinding::new(PerfIssue::UnboundedSearch, PerfSeverity::Warning,
                "source", &
                format!("{} tactic calls detected (threshold: {}); proof search may be deep",
                count, self.max_depth),).with_impact((count as f64 / (self.max_depth as
                f64 * 2.0)).min(1.0))
            ]
        } else {
            Vec::new()
        }
    }
}
/// Warns about inductive types with an unusually large number of constructors.
#[allow(dead_code)]
pub struct InductiveSizeChecker {
    pub max_constructors: usize,
}
impl InductiveSizeChecker {
    #[allow(dead_code)]
    pub fn new(max_constructors: usize) -> Self {
        Self { max_constructors }
    }
    /// Count the constructors of the first `inductive` block in `source`.
    /// Constructors are lines starting with `| `.
    #[allow(dead_code)]
    pub fn count_constructors(source: &str) -> usize {
        let mut in_inductive = false;
        let mut count = 0usize;
        for line in source.lines() {
            let t = line.trim();
            if t.starts_with("inductive ") {
                in_inductive = true;
            }
            if in_inductive && t.starts_with("| ") {
                count += 1;
            }
            if in_inductive && !t.is_empty() && !t.starts_with("|")
                && !t.starts_with("inductive ") && count > 0
            {
                break;
            }
        }
        count
    }
    /// Emit a finding if the constructor count exceeds the threshold.
    #[allow(dead_code)]
    pub fn check(&self, source: &str) -> Vec<PerfFinding> {
        let count = Self::count_constructors(source);
        if count > self.max_constructors {
            vec![
                PerfFinding::new(PerfIssue::LargeInductiveType, PerfSeverity::Warning,
                "source", &
                format!("Inductive type has {} constructors (threshold: {}); may be slow to elaborate",
                count, self.max_constructors),).with_impact((count as f64 / 200.0)
                .min(1.0))
            ]
        } else {
            Vec::new()
        }
    }
}
/// Checks whether `simp` lemmas are in a canonical normal form (naively).
#[allow(dead_code)]
pub struct SimpNormalFormChecker;
impl SimpNormalFormChecker {
    /// Return `true` when the lemma (a string like `h : A = B`) looks like it
    /// is already in normal form: the LHS is shorter than or equal to the RHS.
    #[allow(dead_code)]
    pub fn is_normal_form(lemma: &str) -> bool {
        if let Some(eq_pos) = lemma.find('=') {
            let lhs = lemma[..eq_pos].trim();
            let rhs = lemma[eq_pos + 1..].trim();
            lhs.len() <= rhs.len()
        } else {
            true
        }
    }
    /// Check every `simp` lemma in `lemma_list` and return the problematic ones.
    #[allow(dead_code)]
    pub fn find_non_normal_form_lemmas(lemma_list: &[&str]) -> Vec<String> {
        lemma_list
            .iter()
            .filter(|l| !Self::is_normal_form(l))
            .map(|l| l.to_string())
            .collect()
    }
}
/// Detects heap-allocation-heavy patterns that may slow down elaboration.
#[allow(dead_code)]
pub struct AllocationAnalyzer;
impl AllocationAnalyzer {
    /// Count potential allocation sites: `Vec::new`, `Box::new`, `String::new`,
    /// `HashMap::new`, `BTreeMap::new`.
    #[allow(dead_code)]
    pub fn count_allocations(source: &str) -> usize {
        let alloc_patterns = [
            "Vec::new",
            "Box::new",
            "String::new",
            "String::from",
            "HashMap::new",
            "BTreeMap::new",
            "HashSet::new",
            "BTreeSet::new",
            "Arc::new",
            "Rc::new",
            "to_string()",
            "to_owned()",
            ".clone()",
        ];
        alloc_patterns.iter().map(|p| source.matches(p).count()).sum()
    }
    /// Returns a list of `(line_number, pattern)` for each allocation site found.
    #[allow(dead_code)]
    pub fn find_allocation_sites(source: &str) -> Vec<(usize, String)> {
        let alloc_patterns = [
            "Vec::new",
            "Box::new",
            "String::new",
            "HashMap::new",
            ".clone()",
            "to_owned()",
        ];
        let mut sites = Vec::new();
        for (line_idx, line) in source.lines().enumerate() {
            for pat in &alloc_patterns {
                if line.contains(pat) {
                    sites.push((line_idx + 1, pat.to_string()));
                }
            }
        }
        sites
    }
    /// Emit `PerfFinding`s for all discovered allocation sites.
    #[allow(dead_code)]
    pub fn emit_findings(source: &str) -> Vec<PerfFinding> {
        Self::find_allocation_sites(source)
            .into_iter()
            .map(|(line, pat)| {
                PerfFinding::new(
                        PerfIssue::InefficientPattern,
                        PerfSeverity::Suggestion,
                        &format!("line:{}", line),
                        &format!("Heap allocation pattern `{}` detected", pat),
                    )
                    .with_impact(0.2)
            })
            .collect()
    }
}
/// Configuration thresholds for the performance lint pass.
#[derive(Clone, Debug)]
pub struct PerfLintConfig {
    /// Maximum proof-term token count before warning.
    pub max_proof_term_size: usize,
    /// Maximum recursion depth before warning.
    pub max_recursion_depth: usize,
    /// Maximum number of `simp` lemmas before warning.
    pub max_simp_lemmas: usize,
    /// Whether to warn when classical reasoning is used (can slow instance search).
    pub warn_on_classical: bool,
}
impl PerfLintConfig {
    /// Default thresholds.
    pub fn default() -> Self {
        Self {
            max_proof_term_size: 1000,
            max_recursion_depth: 50,
            max_simp_lemmas: 100,
            warn_on_classical: false,
        }
    }
}
/// Tracks performance findings across multiple analysis runs.
#[allow(dead_code)]
pub struct PerfTrend {
    snapshots: Vec<(String, Vec<PerfFinding>)>,
}
impl PerfTrend {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { snapshots: Vec::new() }
    }
    /// Record a new snapshot labelled `label`.
    #[allow(dead_code)]
    pub fn record(&mut self, label: &str, findings: Vec<PerfFinding>) {
        self.snapshots.push((label.to_string(), findings));
    }
    /// Return the number of snapshots recorded.
    #[allow(dead_code)]
    pub fn snapshot_count(&self) -> usize {
        self.snapshots.len()
    }
    /// Total number of findings in the latest snapshot.
    #[allow(dead_code)]
    pub fn latest_finding_count(&self) -> usize {
        self.snapshots.last().map(|(_, v)| v.len()).unwrap_or(0)
    }
    /// Returns `true` if the latest snapshot has fewer findings than the previous one.
    #[allow(dead_code)]
    pub fn is_improving(&self) -> bool {
        if self.snapshots.len() < 2 {
            return false;
        }
        let prev = self.snapshots[self.snapshots.len() - 2].1.len();
        let latest = self.snapshots[self.snapshots.len() - 1].1.len();
        latest < prev
    }
}
/// Detects potential non-tail recursion patterns (a proxy for stack usage).
#[allow(dead_code)]
pub struct TailCallDetector;
impl TailCallDetector {
    /// Returns line numbers where a function appears to call itself non-tail
    /// (heuristic: recursive call inside a binary operator expression).
    #[allow(dead_code)]
    pub fn find_non_tail_recursive_calls(source: &str) -> Vec<usize> {
        let mut results = Vec::new();
        for (line_idx, line) in source.lines().enumerate() {
            let has_operator = line.contains(" + ") || line.contains(" - ")
                || line.contains(" * ") || line.contains(" && ")
                || line.contains(" || ");
            let words: Vec<&str> = line.split_whitespace().collect();
            let mut word_counts: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
            for w in &words {
                let clean = w.trim_matches(|c: char| !c.is_alphanumeric() && c != '_');
                if clean.len() > 2 {
                    *word_counts.entry(clean).or_insert(0) += 1;
                }
            }
            let has_repeated_fn = word_counts.values().any(|&c| c >= 2);
            if has_operator && has_repeated_fn {
                results.push(line_idx + 1);
            }
        }
        results
    }
    /// Emit `PerfFinding`s for suspected non-tail recursive calls.
    #[allow(dead_code)]
    pub fn emit_findings(source: &str) -> Vec<PerfFinding> {
        Self::find_non_tail_recursive_calls(source)
            .into_iter()
            .map(|line| {
                PerfFinding::new(
                        PerfIssue::DeepRecursion,
                        PerfSeverity::Suggestion,
                        &format!("line:{}", line),
                        "Non-tail recursive call pattern detected; consider accumulator style",
                    )
                    .with_impact(0.3)
            })
            .collect()
    }
}
/// Detects patterns that could be vectorised (SIMD-friendly).
#[allow(dead_code)]
pub struct VectorisationOpportunityDetector;
impl VectorisationOpportunityDetector {
    /// Look for element-wise loops over numeric arrays.
    #[allow(dead_code)]
    pub fn find_opportunities(source: &str) -> Vec<(usize, String)> {
        let mut results = Vec::new();
        let mut in_loop = false;
        let mut loop_line = 0usize;
        for (line_idx, line) in source.lines().enumerate() {
            let t = line.trim();
            if t.starts_with("for ") && (t.contains(" in ") || t.contains(" := ")) {
                in_loop = true;
                loop_line = line_idx + 1;
            }
            if in_loop {
                let has_arith = t.contains('+') || t.contains('*') || t.contains('-');
                let has_index = t.contains('[') && t.contains(']');
                if has_arith && has_index {
                    results.push((loop_line, "Element-wise numeric loop".to_string()));
                    in_loop = false;
                }
                if t.contains('}') || t.is_empty() {
                    in_loop = false;
                }
            }
        }
        results
    }
    /// Emit performance findings for vectorisation opportunities.
    #[allow(dead_code)]
    pub fn emit_findings(source: &str) -> Vec<PerfFinding> {
        Self::find_opportunities(source)
            .into_iter()
            .map(|(line, desc)| {
                PerfFinding::new(
                        PerfIssue::InefficientPattern,
                        PerfSeverity::Suggestion,
                        &format!("line:{}", line),
                        &format!(
                            "{}: consider using SIMD intrinsics or parallel iterators",
                            desc
                        ),
                    )
                    .with_impact(0.35)
            })
            .collect()
    }
}
/// Detects lines where the same sub-expression appears multiple times
/// (very rough textual heuristic).
#[allow(dead_code)]
pub struct RedundantComputationDetector {
    /// Minimum token length to consider for duplication.
    pub min_token_len: usize,
}
impl RedundantComputationDetector {
    #[allow(dead_code)]
    pub fn new(min_token_len: usize) -> Self {
        Self { min_token_len }
    }
    /// Check a single line for repeated tokens.
    #[allow(dead_code)]
    pub fn check_line(line: &str, min_len: usize) -> Vec<String> {
        let mut seen: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
        let tokens: Vec<&str> = line.split_whitespace().collect();
        for tok in &tokens {
            if tok.len() >= min_len {
                *seen.entry(tok).or_insert(0) += 1;
            }
        }
        seen.into_iter()
            .filter(|(_, count)| *count >= 2)
            .map(|(tok, _)| tok.to_string())
            .collect()
    }
    /// Emit findings for lines with redundant tokens.
    #[allow(dead_code)]
    pub fn emit_findings(&self, source: &str) -> Vec<PerfFinding> {
        let mut findings = Vec::new();
        for (line_idx, line) in source.lines().enumerate() {
            let repeated = Self::check_line(line, self.min_token_len);
            for tok in repeated {
                findings
                    .push(
                        PerfFinding::new(
                                PerfIssue::InefficientPattern,
                                PerfSeverity::Suggestion,
                                &format!("line:{}", line_idx + 1),
                                &format!(
                                    "Token `{}` appears multiple times; consider let-binding",
                                    tok
                                ),
                            )
                            .with_impact(0.15),
                    );
            }
        }
        findings
    }
}
/// Provides suggestions for making pattern matches more efficient.
#[allow(dead_code)]
pub struct PatternMatchOptimizer;
impl PatternMatchOptimizer {
    /// Detect pattern matches that might benefit from ordering (most-frequent first).
    ///
    /// Heuristic: count the number of arms in a `match`/`cases` block.
    #[allow(dead_code)]
    pub fn count_match_arms(source: &str) -> usize {
        source
            .lines()
            .filter(|l| {
                let t = l.trim();
                t.starts_with("| ") && !t.starts_with("| --")
            })
            .count()
    }
    /// Emit a finding if the number of arms exceeds the threshold.
    #[allow(dead_code)]
    pub fn check_match_size(source: &str, max_arms: usize) -> Vec<PerfFinding> {
        let arms = Self::count_match_arms(source);
        if arms > max_arms {
            vec![
                PerfFinding::new(PerfIssue::InefficientPattern, PerfSeverity::Suggestion,
                "source", &
                format!("Pattern match has {} arms (threshold: {}); consider using a HashMap for dispatch",
                arms, max_arms),).with_impact(0.2)
            ]
        } else {
            Vec::new()
        }
    }
}
/// A simple simulated elaboration time profiler.
#[allow(dead_code)]
pub struct ElabTimeProfiler {
    records: Vec<ElabTimeRecord>,
}
impl ElabTimeProfiler {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { records: Vec::new() }
    }
    /// Record a labelled duration in milliseconds.
    #[allow(dead_code)]
    pub fn record(&mut self, label: &str, duration_ms: u64) {
        self.records.push(ElabTimeRecord::new(label, duration_ms));
    }
    /// Total recorded time in milliseconds.
    #[allow(dead_code)]
    pub fn total_ms(&self) -> u64 {
        self.records.iter().map(|r| r.duration_ms).sum()
    }
    /// Return the slowest record.
    #[allow(dead_code)]
    pub fn slowest(&self) -> Option<&ElabTimeRecord> {
        self.records.iter().max_by_key(|r| r.duration_ms)
    }
    /// Return records sorted by duration descending.
    #[allow(dead_code)]
    pub fn sorted_by_duration(&self) -> Vec<&ElabTimeRecord> {
        let mut sorted: Vec<&ElabTimeRecord> = self.records.iter().collect();
        sorted.sort_by(|a, b| b.duration_ms.cmp(&a.duration_ms));
        sorted
    }
    /// Return average duration in milliseconds.
    #[allow(dead_code)]
    pub fn average_ms(&self) -> f64 {
        if self.records.is_empty() {
            return 0.0;
        }
        self.total_ms() as f64 / self.records.len() as f64
    }
}
/// Detects unnecessary eta-expansion (`fun x -> f x` instead of `f`).
#[allow(dead_code)]
pub struct EtaExpansionDetector;
impl EtaExpansionDetector {
    /// Very rough heuristic: look for `fun [ident] -> [expr] [ident]` pattern.
    #[allow(dead_code)]
    pub fn find_eta_expansions(source: &str) -> Vec<(usize, String)> {
        let mut results = Vec::new();
        for (line_idx, line) in source.lines().enumerate() {
            let t = line.trim();
            if (t.starts_with("fun ") && t.contains("->"))
                || (t.starts_with("fun ") && t.contains("=>"))
            {
                let after_fun = &t["fun ".len()..];
                if let Some(space_pos) = after_fun.find(' ') {
                    let var_name = &after_fun[..space_pos];
                    if !var_name.is_empty() && t.ends_with(var_name) {
                        results
                            .push((
                                line_idx + 1,
                                format!("fun {} -> ... {}", var_name, var_name),
                            ));
                    }
                }
            }
        }
        results
    }
    /// Emit suggestions for eta-reducible expressions.
    #[allow(dead_code)]
    pub fn emit_findings(source: &str) -> Vec<PerfFinding> {
        Self::find_eta_expansions(source)
            .into_iter()
            .map(|(line, desc)| {
                PerfFinding::new(
                        PerfIssue::InefficientPattern,
                        PerfSeverity::Suggestion,
                        &format!("line:{}", line),
                        &format!(
                            "Eta-expansion `{}` can be simplified by eta-reduction", desc
                        ),
                    )
                    .with_impact(0.08)
            })
            .collect()
    }
}
/// A simple in-process benchmark harness for measuring lint pass overhead.
#[allow(dead_code)]
pub struct BenchmarkHarness {
    pub iterations: u32,
}
impl BenchmarkHarness {
    #[allow(dead_code)]
    pub fn new(iterations: u32) -> Self {
        Self { iterations }
    }
    /// Run `f` `iterations` times and return the average count of findings.
    #[allow(dead_code)]
    pub fn run_avg<F>(&self, f: F) -> f64
    where
        F: Fn() -> Vec<PerfFinding>,
    {
        let total: usize = (0..self.iterations).map(|_| f().len()).sum();
        total as f64 / self.iterations as f64
    }
}
/// A single performance finding.
#[derive(Clone, Debug)]
pub struct PerfFinding {
    /// The kind of performance issue.
    pub issue: PerfIssue,
    /// Severity of the finding.
    pub severity: PerfSeverity,
    /// Human-readable location string.
    pub location: String,
    /// Description of the issue.
    pub message: String,
    /// Estimated performance impact on a 0.0–1.0 scale.
    pub estimated_impact: f64,
}
impl PerfFinding {
    /// Create a new finding with zero estimated impact.
    pub fn new(issue: PerfIssue, severity: PerfSeverity, loc: &str, msg: &str) -> Self {
        Self {
            issue,
            severity,
            location: loc.to_string(),
            message: msg.to_string(),
            estimated_impact: 0.0,
        }
    }
    /// Set the estimated impact and return `self`.
    pub fn with_impact(mut self, impact: f64) -> Self {
        self.estimated_impact = impact.clamp(0.0, 1.0);
        self
    }
    /// Returns `true` when the severity is `Blocker`.
    pub fn is_blocker(&self) -> bool {
        self.severity == PerfSeverity::Blocker
    }
}
/// Scores the complexity introduced by polymorphic definitions.
#[allow(dead_code)]
pub struct PolymorphismComplexityScorer;
impl PolymorphismComplexityScorer {
    /// Count the number of universe-polymorphic level variables in `source`.
    #[allow(dead_code)]
    pub fn count_universe_variables(source: &str) -> usize {
        source.matches(".{").count()
    }
    /// Count the number of type-level variables (like `{α : Type}`, `{β : Sort}`).
    #[allow(dead_code)]
    pub fn count_type_variables(source: &str) -> usize {
        source.matches(": Type").count() + source.matches(": Sort").count()
            + source.matches(": Prop").count()
    }
    /// Overall polymorphism score in [0, 1].
    #[allow(dead_code)]
    pub fn score(source: &str) -> f64 {
        let univ = Self::count_universe_variables(source) as f64;
        let type_vars = Self::count_type_variables(source) as f64;
        ((univ / 20.0) + (type_vars / 30.0)).min(1.0)
    }
}
/// Detects computations that are repeated inside loops and could be hoisted.
#[allow(dead_code)]
pub struct LoopHoistingAnalyzer;
impl LoopHoistingAnalyzer {
    /// Check for function calls or allocations that appear inside loop bodies.
    ///
    /// Heuristic: any line between `for`/`while`/`loop` and the matching `}`
    /// that contains an expensive call.
    #[allow(dead_code)]
    pub fn find_hoistable(source: &str) -> Vec<(usize, String)> {
        let expensive = [
            ".sort()",
            ".sort_by(",
            ".contains(",
            "Vec::new",
            "String::new",
            ".len()",
        ];
        let mut results = Vec::new();
        let mut in_loop = false;
        let mut depth = 0usize;
        for (line_idx, line) in source.lines().enumerate() {
            let trimmed = line.trim();
            if trimmed.starts_with("for ") || trimmed.starts_with("while ")
                || trimmed == "loop {"
            {
                in_loop = true;
            }
            if in_loop {
                for ch in trimmed.chars() {
                    match ch {
                        '{' => depth += 1,
                        '}' => {
                            depth = depth.saturating_sub(1);
                            if depth == 0 {
                                in_loop = false;
                            }
                        }
                        _ => {}
                    }
                }
                for pat in &expensive {
                    if trimmed.contains(pat) {
                        results.push((line_idx + 1, pat.to_string()));
                    }
                }
            }
        }
        results
    }
    /// Convert findings to `PerfFinding`s.
    #[allow(dead_code)]
    pub fn emit_findings(source: &str) -> Vec<PerfFinding> {
        Self::find_hoistable(source)
            .into_iter()
            .map(|(line, pat)| {
                PerfFinding::new(
                        PerfIssue::InefficientPattern,
                        PerfSeverity::Warning,
                        &format!("line:{}", line),
                        &format!("Potentially hoistable call `{}` inside loop", pat),
                    )
                    .with_impact(0.4)
            })
            .collect()
    }
}
/// Tracks and warns about excessive metavariable usage.
#[allow(dead_code)]
pub struct MetavarTracker {
    pub max_metavars: usize,
}
impl MetavarTracker {
    #[allow(dead_code)]
    pub fn new(max_metavars: usize) -> Self {
        Self { max_metavars }
    }
    /// Count metavariable placeholders (`?_`, `?x`, `_`) in `source`.
    #[allow(dead_code)]
    pub fn count_metavars(source: &str) -> usize {
        source.matches("?_").count() + source.matches("?m").count()
            + source.matches("?n").count()
            + source.split_whitespace().filter(|t| *t == "_").count()
    }
    /// Emit a warning if metavar count exceeds the threshold.
    #[allow(dead_code)]
    pub fn check(&self, source: &str) -> Vec<PerfFinding> {
        let count = Self::count_metavars(source);
        if count > self.max_metavars {
            vec![
                PerfFinding::new(PerfIssue::ExcessiveMetavars, PerfSeverity::Warning,
                "source", &
                format!("{} metavariables detected (threshold: {}); elaboration may be slow",
                count, self.max_metavars),).with_impact((count as f64 / 100.0).min(1.0))
            ]
        } else {
            Vec::new()
        }
    }
}
/// Detects unnecessary `.clone()` calls that could be eliminated.
#[allow(dead_code)]
pub struct CloneDetector {
    /// Minimum number of clones per source before warning.
    pub threshold: usize,
}
impl CloneDetector {
    #[allow(dead_code)]
    pub fn new(threshold: usize) -> Self {
        Self { threshold }
    }
    /// Count `.clone()` calls in `source`.
    #[allow(dead_code)]
    pub fn count_clones(source: &str) -> usize {
        source.matches(".clone()").count()
    }
    /// Return findings if the clone count exceeds the threshold.
    #[allow(dead_code)]
    pub fn check(&self, source: &str) -> Vec<PerfFinding> {
        let count = Self::count_clones(source);
        if count >= self.threshold {
            vec![
                PerfFinding::new(PerfIssue::InefficientPattern, PerfSeverity::Warning,
                "source", &
                format!("{} `.clone()` calls detected (threshold: {}); consider borrowing",
                count, self.threshold),).with_impact((count as f64 / 50.0).min(1.0))
            ]
        } else {
            Vec::new()
        }
    }
}
/// Estimates memory usage from patterns in source text.
#[allow(dead_code)]
pub struct MemoryUsageEstimator;
impl MemoryUsageEstimator {
    /// Estimate bytes allocated per `Vec::new()` call (rough average of 24 bytes
    /// for the Vec header on 64-bit systems).
    #[allow(dead_code)]
    pub const VEC_HEADER_BYTES: usize = 24;
    /// Estimate bytes for a `String::new()` header.
    #[allow(dead_code)]
    pub const STRING_HEADER_BYTES: usize = 24;
    /// Rough estimate of total heap bytes based on allocation patterns.
    #[allow(dead_code)]
    pub fn estimate_bytes(source: &str) -> usize {
        let vecs = source.matches("Vec::new").count();
        let strings = source.matches("String::new").count()
            + source.matches("String::from").count();
        let boxes = source.matches("Box::new").count();
        let hashmaps = source.matches("HashMap::new").count()
            + source.matches("BTreeMap::new").count();
        vecs * Self::VEC_HEADER_BYTES + strings * Self::STRING_HEADER_BYTES + boxes * 8
            + hashmaps * 64
    }
    /// Return a human-readable estimate string.
    #[allow(dead_code)]
    pub fn estimate_readable(source: &str) -> String {
        let bytes = Self::estimate_bytes(source);
        if bytes >= 1024 * 1024 {
            format!("{:.2} MB", bytes as f64 / (1024.0 * 1024.0))
        } else if bytes >= 1024 {
            format!("{:.2} KB", bytes as f64 / 1024.0)
        } else {
            format!("{} B", bytes)
        }
    }
}
/// A single elaboration timing record.
#[allow(dead_code)]
pub struct ElabTimeRecord {
    pub label: String,
    pub duration_ms: u64,
}
impl ElabTimeRecord {
    #[allow(dead_code)]
    pub fn new(label: &str, duration_ms: u64) -> Self {
        Self {
            label: label.to_string(),
            duration_ms,
        }
    }
}
/// Detects proof branches that can never be taken (very rough heuristic).
#[allow(dead_code)]
pub struct DeadProofBranchDetector;
impl DeadProofBranchDetector {
    /// Look for `| False => ...` or `| absurd ...` patterns that are unreachable
    /// heuristics.
    #[allow(dead_code)]
    pub fn find_dead_branches(source: &str) -> Vec<usize> {
        let mut results = Vec::new();
        let dead_patterns = [
            "| False =>",
            "| False ->",
            "absurd rfl",
            "absurd (by exact",
        ];
        for (line_idx, line) in source.lines().enumerate() {
            for pat in &dead_patterns {
                if line.contains(pat) {
                    results.push(line_idx + 1);
                }
            }
        }
        results
    }
    /// Emit findings for dead branches.
    #[allow(dead_code)]
    pub fn emit_findings(source: &str) -> Vec<PerfFinding> {
        Self::find_dead_branches(source)
            .into_iter()
            .map(|line| {
                PerfFinding::new(
                        PerfIssue::InefficientPattern,
                        PerfSeverity::Suggestion,
                        &format!("line:{}", line),
                        "Potentially dead proof branch detected; consider simplifying",
                    )
                    .with_impact(0.1)
            })
            .collect()
    }
}
/// Runs every performance analyzer and returns a combined `PerfReport`.
#[allow(dead_code)]
pub struct FullPerfAnalyzer {
    pub pass: PerfLintPass,
    pub clone_threshold: usize,
    pub max_metavars: usize,
    pub max_constructors: usize,
    pub redundant_min_len: usize,
}
impl FullPerfAnalyzer {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            pass: PerfLintPass::new(),
            clone_threshold: 5,
            max_metavars: 10,
            max_constructors: 30,
            redundant_min_len: 5,
        }
    }
    /// Run the full analysis and produce a `PerfReport`.
    #[allow(dead_code)]
    pub fn analyze(&self, source: &str) -> PerfReport {
        let mut findings = self.pass.run_all(source);
        findings.extend(AllocationAnalyzer::emit_findings(source));
        findings.extend(LoopHoistingAnalyzer::emit_findings(source));
        findings.extend(CacheUnfriendlyAccessDetector::emit_findings(source));
        let clone_detector = CloneDetector::new(self.clone_threshold);
        findings.extend(clone_detector.check(source));
        let metavar_tracker = MetavarTracker::new(self.max_metavars);
        findings.extend(metavar_tracker.check(source));
        let inductive_checker = InductiveSizeChecker::new(self.max_constructors);
        findings.extend(inductive_checker.check(source));
        let redundant_detector = RedundantComputationDetector::new(
            self.redundant_min_len,
        );
        findings.extend(redundant_detector.emit_findings(source));
        let complexity = self.pass.estimate_complexity(source);
        PerfReport::from_findings(findings, complexity)
    }
}
/// A named threshold value with a description.
#[allow(dead_code)]
pub struct PerformanceThreshold {
    pub name: String,
    pub value: f64,
    pub description: String,
}
impl PerformanceThreshold {
    #[allow(dead_code)]
    pub fn new(name: &str, value: f64, description: &str) -> Self {
        Self {
            name: name.to_string(),
            value,
            description: description.to_string(),
        }
    }
    /// Returns `true` when `measured` exceeds this threshold.
    #[allow(dead_code)]
    pub fn is_exceeded(&self, measured: f64) -> bool {
        measured > self.value
    }
}
/// Severity of a performance finding.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PerfSeverity {
    /// Build will not terminate in reasonable time.
    Blocker,
    /// Noticeably slows elaboration.
    Warning,
    /// Minor improvement available.
    Suggestion,
}
/// Detects arithmetic expressions with only constant operands that could be
/// precomputed.
#[allow(dead_code)]
pub struct ConstantFoldingChecker;
impl ConstantFoldingChecker {
    /// Check whether a token is a numeric literal.
    #[allow(dead_code)]
    fn is_numeric(token: &str) -> bool {
        token.parse::<f64>().is_ok()
    }
    /// Find lines where a simple constant arithmetic expression is detectable.
    #[allow(dead_code)]
    pub fn find_foldable_expressions(source: &str) -> Vec<(usize, String)> {
        let mut results = Vec::new();
        for (line_idx, line) in source.lines().enumerate() {
            let words: Vec<&str> = line.split_whitespace().collect();
            for win in words.windows(3) {
                if Self::is_numeric(win[0])
                    && (win[1] == "+" || win[1] == "-" || win[1] == "*" || win[1] == "/")
                    && Self::is_numeric(win[2])
                {
                    results
                        .push((
                            line_idx + 1,
                            format!("{} {} {}", win[0], win[1], win[2]),
                        ));
                }
            }
        }
        results
    }
    /// Emit findings for constant expressions that can be folded.
    #[allow(dead_code)]
    pub fn emit_findings(source: &str) -> Vec<PerfFinding> {
        Self::find_foldable_expressions(source)
            .into_iter()
            .map(|(line, expr)| {
                PerfFinding::new(
                        PerfIssue::InefficientPattern,
                        PerfSeverity::Suggestion,
                        &format!("line:{}", line),
                        &format!("Constant expression `{}` can be precomputed", expr),
                    )
                    .with_impact(0.05)
            })
            .collect()
    }
}
/// Estimates elaboration complexity from raw source text.
pub struct ElabComplexityEstimator;
impl ElabComplexityEstimator {
    /// Create a new estimator.
    pub fn new() -> Self {
        Self
    }
    /// Count the number of function application sites (spaces between tokens,
    /// used as a cheap proxy).
    pub fn count_applications(source: &str) -> usize {
        let mut count = 0usize;
        let words: Vec<&str> = source.split_whitespace().collect();
        for pair in words.windows(2) {
            let a = pair[0];
            let b = pair[1];
            let a_ident = a.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '.');
            let b_ident = b.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '.');
            if a_ident && b_ident {
                count += 1;
            }
        }
        count
    }
    /// Count the number of binders (`fun`, `forall`, `∀`, `λ`, `\`).
    pub fn count_binders(source: &str) -> usize {
        let binder_keywords = ["fun ", "forall ", "∀ ", "λ ", "\\ "];
        binder_keywords.iter().map(|kw| source.matches(kw).count()).sum()
    }
    /// Estimate the cost of unification based on the number of metavariable
    /// placeholders (`?_`, `_`) in the source.
    pub fn estimate_unification_cost(source: &str) -> f64 {
        let holes = source.matches("?_").count() + source.matches(" _ ").count();
        (holes as f64 * 0.05).min(1.0)
    }
    /// Combined score in [0.0, 1.0] representing overall elaboration complexity.
    pub fn overall_score(source: &str) -> f64 {
        let apps = Self::count_applications(source) as f64;
        let binders = Self::count_binders(source) as f64;
        let unif = Self::estimate_unification_cost(source);
        let app_score = (apps / 500.0).min(1.0);
        let binder_score = (binders / 50.0).min(1.0);
        ((app_score + binder_score + unif) / 3.0).min(1.0)
    }
}
/// Aggregated performance report for a source file.
#[allow(dead_code)]
pub struct PerfReport {
    pub findings: Vec<PerfFinding>,
    pub complexity_score: f64,
    pub blocker_count: usize,
    pub warning_count: usize,
    pub suggestion_count: usize,
}
impl PerfReport {
    /// Build a report from a list of findings.
    #[allow(dead_code)]
    pub fn from_findings(findings: Vec<PerfFinding>, complexity_score: f64) -> Self {
        let blocker_count = findings
            .iter()
            .filter(|f| f.severity == PerfSeverity::Blocker)
            .count();
        let warning_count = findings
            .iter()
            .filter(|f| f.severity == PerfSeverity::Warning)
            .count();
        let suggestion_count = findings
            .iter()
            .filter(|f| f.severity == PerfSeverity::Suggestion)
            .count();
        Self {
            findings,
            complexity_score,
            blocker_count,
            warning_count,
            suggestion_count,
        }
    }
    /// Return true when there are no findings.
    #[allow(dead_code)]
    pub fn is_clean(&self) -> bool {
        self.findings.is_empty()
    }
    /// Overall score: 1.0 = perfect, 0.0 = very bad.
    #[allow(dead_code)]
    pub fn health_score(&self) -> f64 {
        let penalty = self.blocker_count as f64 * 0.4 + self.warning_count as f64 * 0.1
            + self.suggestion_count as f64 * 0.02;
        (1.0 - penalty - self.complexity_score * 0.3).clamp(0.0, 1.0)
    }
}
/// Detects potentially cache-unfriendly access patterns.
#[allow(dead_code)]
pub struct CacheUnfriendlyAccessDetector;
impl CacheUnfriendlyAccessDetector {
    /// Look for random-access patterns like `v[i]` where `i` is not a simple
    /// loop counter.
    #[allow(dead_code)]
    pub fn find_random_access_patterns(source: &str) -> Vec<(usize, String)> {
        let mut results = Vec::new();
        for (line_idx, line) in source.lines().enumerate() {
            if let Some(start) = line.find('[') {
                let after = &line[start + 1..];
                if let Some(end) = after.find(']') {
                    let index_expr = after[..end].trim();
                    let is_simple = index_expr.len() <= 2
                        || index_expr.chars().all(|c| c.is_alphanumeric() || c == '_');
                    if !is_simple {
                        results
                            .push((
                                line_idx + 1,
                                format!("Index expression `[{}]`", index_expr),
                            ));
                    }
                }
            }
        }
        results
    }
    /// Emit findings for cache-unfriendly patterns.
    #[allow(dead_code)]
    pub fn emit_findings(source: &str) -> Vec<PerfFinding> {
        Self::find_random_access_patterns(source)
            .into_iter()
            .map(|(line, desc)| {
                PerfFinding::new(
                        PerfIssue::InefficientPattern,
                        PerfSeverity::Suggestion,
                        &format!("line:{}", line),
                        &format!("{} may be cache-unfriendly", desc),
                    )
                    .with_impact(0.25)
            })
            .collect()
    }
}
/// A hint for where to add profiling instrumentation.
#[allow(dead_code)]
pub struct ProfilingHint {
    pub location: String,
    pub reason: String,
}
impl ProfilingHint {
    #[allow(dead_code)]
    pub fn new(location: &str, reason: &str) -> Self {
        Self {
            location: location.to_string(),
            reason: reason.to_string(),
        }
    }
    /// Generate profiling hints from a list of `PerfFinding`s.
    #[allow(dead_code)]
    pub fn from_findings(findings: &[PerfFinding]) -> Vec<Self> {
        findings
            .iter()
            .filter(|f| f.severity >= PerfSeverity::Warning)
            .map(|f| Self::new(&f.location, &f.message))
            .collect()
    }
}
