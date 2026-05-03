//! Types for type-directed program synthesis.

/// A synthesis goal: what type to synthesize a term for, with context.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SynthesisGoal {
    /// The target type to synthesize a term of.
    pub target_type: String,
    /// Local context: list of (name, type) pairs available.
    pub context: Vec<(String, String)>,
    /// Maximum depth for recursive synthesis.
    pub depth: usize,
}

/// Strategy to use when synthesizing terms.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SynthesisStrategy {
    /// Try all candidates exhaustively.
    ExhaustiveSearch,
    /// Sample candidates randomly using the given seed.
    RandomSampling { seed: u64 },
    /// Use type-directed heuristics to guide search.
    TypeDirected,
    /// Combine all strategies.
    CombineAll,
}

/// The result of a synthesis attempt.
#[derive(Debug, Clone, PartialEq)]
pub enum SynthesisResult {
    /// A term was found with its synthesized type.
    Found { term: String, type_: String },
    /// No term could be found within the given constraints.
    NotFound,
    /// Synthesis timed out.
    Timeout,
}

/// Configuration for the synthesis engine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SynthesisConfig {
    /// Maximum recursion depth during synthesis.
    pub max_depth: usize,
    /// Maximum number of candidate terms to explore.
    pub max_terms: usize,
    /// Timeout in milliseconds.
    pub timeout_ms: u64,
    /// Which strategy to use.
    pub strategy: SynthesisStrategy,
}

impl Default for SynthesisConfig {
    fn default() -> Self {
        Self {
            max_depth: 5,
            max_terms: 1000,
            timeout_ms: 5000,
            strategy: SynthesisStrategy::TypeDirected,
        }
    }
}

/// Statistics collected during synthesis.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SynthesisStats {
    /// Total number of terms explored.
    pub terms_explored: usize,
    /// Maximum depth actually reached.
    pub depth_reached: usize,
    /// Time taken in milliseconds.
    pub time_ms: u64,
}

/// A candidate term produced during synthesis.
#[derive(Debug, Clone, PartialEq)]
pub struct TermCandidate {
    /// The candidate term (as a string representation).
    pub term: String,
    /// Heuristic score: higher is better.
    pub score: f64,
    /// Where this candidate came from.
    pub source: CandidateSource,
}

/// The source of a candidate term.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CandidateSource {
    /// Came from a hypothesis in the local context.
    FromContext,
    /// Came from applying a constructor.
    FromConstructor,
    /// Came from function application.
    FromApplication,
    /// Came from a lambda abstraction.
    FromLambda,
}
