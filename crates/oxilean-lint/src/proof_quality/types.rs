//! Types for proof quality lint rules.

use std::collections::HashMap;

/// Lint rules for proof quality analysis.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ProofQualityRule {
    /// Disallow use of `sorry` placeholder.
    AvoidSorry,
    /// Warn when proof nesting exceeds a given depth.
    DeepNesting { max_depth: usize },
    /// Warn when a proof exceeds a given number of lines.
    LongProof { max_lines: usize },
    /// Warn when the same tactic appears at least `min_count` times.
    RepeatedTactic { min_count: usize },
    /// Warn when a hypothesis is introduced but never referenced.
    UnusedHypothesis,
    /// Warn when a lemma has a trivially short / incomplete proof body.
    TrivialLemma,
    /// Warn when a theorem/def lacks an explicit type annotation.
    MissingTypeAnnotation,
}

impl std::fmt::Display for ProofQualityRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AvoidSorry => write!(f, "avoid_sorry"),
            Self::DeepNesting { max_depth } => write!(f, "deep_nesting({})", max_depth),
            Self::LongProof { max_lines } => write!(f, "long_proof({})", max_lines),
            Self::RepeatedTactic { min_count } => write!(f, "repeated_tactic({})", min_count),
            Self::UnusedHypothesis => write!(f, "unused_hypothesis"),
            Self::TrivialLemma => write!(f, "trivial_lemma"),
            Self::MissingTypeAnnotation => write!(f, "missing_type_annotation"),
        }
    }
}

/// Aggregated proof metrics for a single named proof.
#[derive(Clone, Debug, Default)]
pub struct ProofAnalysis {
    /// File path (or label) containing the proof.
    pub file: String,
    /// Name of the theorem/lemma/definition.
    pub name: String,
    /// Total number of lines in the proof body.
    pub proof_lines: usize,
    /// Number of `sorry` occurrences in the proof.
    pub sorry_count: usize,
    /// Maximum nesting depth observed in the proof.
    pub nesting_depth: usize,
    /// All tactic names found (may contain duplicates).
    pub tactics_used: Vec<String>,
    /// Hypothesis names introduced (e.g. `intro h` → `h`).
    pub hypotheses: Vec<String>,
    /// Hypothesis names that appear in the proof body after introduction.
    pub hypotheses_used: Vec<String>,
}

/// A single proof-quality issue.
#[derive(Clone, Debug)]
pub struct ProofQualityIssue {
    /// The rule that triggered this issue.
    pub rule: ProofQualityRule,
    /// (line, column) location; 1-based.
    pub location: (u32, u32),
    /// Human-readable description.
    pub message: String,
    /// Optional fix suggestion.
    pub suggestion: Option<String>,
}

impl ProofQualityIssue {
    /// Construct a new issue without a suggestion.
    pub fn new(rule: ProofQualityRule, location: (u32, u32), message: impl Into<String>) -> Self {
        Self {
            rule,
            location,
            message: message.into(),
            suggestion: None,
        }
    }

    /// Attach a suggestion string.
    pub fn with_suggestion(mut self, s: impl Into<String>) -> Self {
        self.suggestion = Some(s.into());
        self
    }
}

impl std::fmt::Display for ProofQualityIssue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] {}:{} — {}",
            self.rule, self.location.0, self.location.1, self.message
        )
    }
}

/// Summary report for all proof-quality checks on a source file.
#[derive(Clone, Debug, Default)]
pub struct ProofQualityReport {
    /// All issues found.
    pub issues: Vec<ProofQualityIssue>,
    /// Aggregate quality score in `[0.0, 1.0]` (higher = better).
    pub score: f64,
    /// High-level improvement suggestions derived from the issues.
    pub suggestions: Vec<String>,
}

impl ProofQualityReport {
    /// Return `true` when no issues were found.
    pub fn is_clean(&self) -> bool {
        self.issues.is_empty()
    }

    /// Number of issues with a given rule.
    pub fn count_by_rule(&self, rule: &ProofQualityRule) -> usize {
        self.issues.iter().filter(|i| &i.rule == rule).count()
    }
}

/// Configuration for the proof-quality lint pass.
#[derive(Clone, Debug)]
pub struct ProofQualityConfig {
    /// Rules to enable.
    pub rules: Vec<ProofQualityRule>,
    /// Per-rule weighting used when computing the quality score.
    /// Key = `ProofQualityRule::to_string()`, value = penalty weight.
    pub score_weights: HashMap<String, f64>,
}

impl Default for ProofQualityConfig {
    fn default() -> Self {
        let mut score_weights = HashMap::new();
        score_weights.insert("avoid_sorry".to_owned(), 0.4);
        score_weights.insert("deep_nesting(5)".to_owned(), 0.15);
        score_weights.insert("long_proof(100)".to_owned(), 0.1);
        score_weights.insert("repeated_tactic(3)".to_owned(), 0.1);
        score_weights.insert("unused_hypothesis".to_owned(), 0.1);
        score_weights.insert("trivial_lemma".to_owned(), 0.05);
        score_weights.insert("missing_type_annotation".to_owned(), 0.1);

        Self {
            rules: vec![
                ProofQualityRule::AvoidSorry,
                ProofQualityRule::DeepNesting { max_depth: 5 },
                ProofQualityRule::LongProof { max_lines: 100 },
                ProofQualityRule::RepeatedTactic { min_count: 3 },
                ProofQualityRule::UnusedHypothesis,
                ProofQualityRule::TrivialLemma,
                ProofQualityRule::MissingTypeAnnotation,
            ],
            score_weights,
        }
    }
}
