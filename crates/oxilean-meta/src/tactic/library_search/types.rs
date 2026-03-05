//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use crate::basic::MetaContext;
use crate::discr_tree::DiscrTree;
use oxilean_kernel::{Expr, Name};
use std::collections::{HashMap, HashSet};
use std::time::Instant;

/// A simple string-keyed lemma index for type-directed search.
///
/// This is distinct from the kernel-level `LemmaIndex` (which is backed by
/// `DiscrTree`). `SimpleLemmaIndex` operates on string representations and
/// is suitable for lightweight heuristic search without kernel elaboration.
#[derive(Clone, Debug, Default)]
pub struct SimpleLemmaIndex {
    /// Stored as `(lemma_name, type_signature)`.
    pub entries: Vec<(String, TypeSignature)>,
}
impl SimpleLemmaIndex {
    /// Create a new, empty index.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
    /// Add a lemma by name and type string.
    pub fn add_lemma(&mut self, name: &str, ty_str: &str) {
        if self.entries.iter().any(|(n, _)| n == name) {
            return;
        }
        let sig = TypeSignature::parse_type(ty_str);
        self.entries.push((name.to_string(), sig));
    }
    /// Search for lemmas whose type signature matches the query.
    pub fn search_by_type(&self, query: &TypeSignature) -> Vec<&str> {
        self.entries
            .iter()
            .filter(|(_, sig)| query.matches(sig))
            .map(|(name, _)| name.as_str())
            .collect()
    }
    /// Search for lemmas whose name starts with the given prefix.
    pub fn search_by_name(&self, prefix: &str) -> Vec<&str> {
        self.entries
            .iter()
            .filter(|(name, _)| name.starts_with(prefix))
            .map(|(name, _)| name.as_str())
            .collect()
    }
    /// Combined search: filter by name prefix AND optionally by type signature.
    pub fn search_combined(
        &self,
        name_prefix: &str,
        ty_query: Option<&TypeSignature>,
    ) -> Vec<&str> {
        self.entries
            .iter()
            .filter(|(name, sig)| {
                let name_ok = name_prefix.is_empty() || name.starts_with(name_prefix);
                let type_ok = ty_query.map_or(true, |q| q.matches(sig));
                name_ok && type_ok
            })
            .map(|(name, _)| name.as_str())
            .collect()
    }
    /// Number of indexed lemmas.
    pub fn size(&self) -> usize {
        self.entries.len()
    }
    /// Return a `SimpleLemmaIndex` pre-populated with common Mathlib lemmas.
    pub fn prebuilt() -> Self {
        let mut idx = Self::new();
        idx.add_lemma("Nat.zero_add", "Nat -> Eq Nat _ _");
        idx.add_lemma("Nat.add_zero", "Nat -> Eq Nat _ _");
        idx.add_lemma("Nat.add_comm", "Eq Nat _ _");
        idx.add_lemma("Nat.add_assoc", "Eq Nat _ _");
        idx.add_lemma("Nat.mul_comm", "Eq Nat _ _");
        idx.add_lemma("Nat.mul_zero", "Eq Nat _ _");
        idx.add_lemma("Nat.zero_mul", "Eq Nat _ _");
        idx.add_lemma("Nat.succ_pos", "Nat -> _");
        idx.add_lemma("Nat.le_refl", "Nat -> _");
        idx.add_lemma("Nat.le_trans", "_ -> _ -> _");
        idx.add_lemma("Nat.lt_irrefl", "Nat -> _");
        idx.add_lemma("Nat.pow_zero", "Eq Nat _ _");
        idx.add_lemma("Nat.pow_succ", "Eq Nat _ _");
        idx.add_lemma("Bool.and_true", "Eq Bool _ _");
        idx.add_lemma("Bool.true_and", "Eq Bool _ _");
        idx.add_lemma("Bool.or_false", "Eq Bool _ _");
        idx.add_lemma("Bool.false_or", "Eq Bool _ _");
        idx.add_lemma("And.intro", "_ -> _ -> And _ _");
        idx.add_lemma("And.left", "And _ _ -> _");
        idx.add_lemma("And.right", "And _ _ -> _");
        idx.add_lemma("Or.inl", "_ -> Or _ _");
        idx.add_lemma("Or.inr", "_ -> Or _ _");
        idx.add_lemma("Iff.intro", "_ -> _ -> Iff _ _");
        idx.add_lemma("Iff.mp", "Iff _ _ -> _ -> _");
        idx.add_lemma("Iff.mpr", "Iff _ _ -> _ -> _");
        idx.add_lemma("not_not", "Iff _ _");
        idx.add_lemma("Classical.em", "Or _ _");
        idx.add_lemma("Classical.byContradiction", "_ -> _");
        idx.add_lemma("Eq.refl", "Eq _ _ _");
        idx.add_lemma("Eq.symm", "Eq _ _ _ -> Eq _ _ _");
        idx.add_lemma("Eq.trans", "Eq _ _ _ -> Eq _ _ _ -> Eq _ _ _");
        idx.add_lemma("congrArg", "Eq _ _ _ -> Eq _ _ _");
        idx.add_lemma("List.nil_append", "Eq List _ _");
        idx.add_lemma("List.append_nil", "Eq List _ _");
        idx.add_lemma("List.length_nil", "Eq Nat _ _");
        idx.add_lemma("List.length_cons", "Eq Nat _ _");
        idx.add_lemma("List.map_id", "Eq List _ _");
        idx.add_lemma("List.reverse_reverse", "Eq List _ _");
        idx
    }
}
/// A scored entry for the search priority queue.
#[derive(Clone, Debug)]
pub(super) struct ScoredEntry {
    pub(super) entry: LemmaEntry,
    pub(super) priority: f64,
    pub(super) depth: u32,
}
/// Configuration knobs for the library search procedure.
#[derive(Clone, Debug)]
pub struct LibrarySearchConfig {
    /// Maximum number of candidate lemmas to try before giving up.
    pub max_candidates: usize,
    /// Maximum depth for recursive argument synthesis.
    pub max_depth: u32,
    /// Hard wall-clock timeout in milliseconds (0 = unlimited).
    pub timeout_ms: u64,
    /// Whether to include local hypotheses as candidate lemmas.
    pub include_local: bool,
    /// If true, only suggest a tactic string; do not actually apply it.
    pub suggest_only: bool,
    /// Maximum number of results to return (for `exact?` interactive display).
    pub max_results: usize,
    /// Whether to allow lemmas that leave subgoals (apply? mode).
    pub allow_subgoals: bool,
    /// Maximum number of subgoals a candidate may leave behind.
    pub max_remaining_goals: usize,
    /// Maximum number of arguments to synthesize per candidate.
    pub max_synth_args: usize,
    /// Whether to use the discrimination-tree index (false = brute force).
    pub use_discr_tree: bool,
    /// Minimum score threshold; candidates below this are pruned early.
    pub min_score: f64,
    /// Weight given to specificity when scoring.
    pub specificity_weight: f64,
    /// Weight given to remaining-goal count when scoring.
    pub remaining_goals_weight: f64,
    /// Weight given to edit-distance penalty when scoring.
    pub edit_distance_weight: f64,
}
impl LibrarySearchConfig {
    /// Preset for `exact?`: no subgoals allowed, fast timeout.
    pub fn exact_mode() -> Self {
        Self {
            allow_subgoals: false,
            max_remaining_goals: 0,
            ..Self::default()
        }
    }
    /// Preset for `apply?`: subgoals allowed, slightly more generous limits.
    pub fn apply_mode() -> Self {
        Self {
            allow_subgoals: true,
            max_remaining_goals: 5,
            max_candidates: 512,
            ..Self::default()
        }
    }
}
/// Outcome of a library search attempt.
#[derive(Clone, Debug)]
pub enum SearchResult {
    /// A proof was found; includes the term and a human-readable suggestion.
    Found(Expr, String),
    /// Multiple results found (for interactive display).
    MultipleFound(Vec<LemmaCandidate>),
    /// No matching lemma was found.
    NotFound,
    /// The search timed out before completing.
    TimedOut,
}
impl SearchResult {
    /// Whether the search succeeded.
    pub fn is_found(&self) -> bool {
        matches!(
            self,
            SearchResult::Found(_, _) | SearchResult::MultipleFound(_)
        )
    }
    /// Whether the search timed out.
    pub fn is_timed_out(&self) -> bool {
        matches!(self, SearchResult::TimedOut)
    }
}
/// Internal error type for candidate evaluation.
#[derive(Debug)]
pub(super) enum CandidateError {
    /// Conclusion did not unify with goal.
    UnificationFailed,
    /// Score was below the configured threshold.
    ScoreTooLow,
}
/// A candidate lemma that might close (or partially close) the goal.
#[derive(Clone, Debug)]
pub struct LemmaCandidate {
    /// Fully-qualified name of the lemma / theorem / definition.
    pub name: Name,
    /// The type of the lemma (possibly with universe metavariables).
    pub ty: Expr,
    /// Composite score used for ranking.
    pub score: f64,
    /// The arguments that were successfully applied.
    pub applied_args: Vec<Expr>,
    /// How many goals remain after applying this candidate.
    pub remaining_goals: usize,
    /// The proof term that would close the goal (if fully solved).
    pub proof: Option<Expr>,
    /// Human-readable tactic suggestion string.
    pub suggestion: String,
    /// Detailed scoring breakdown.
    pub criteria: ScoringCriteria,
}
impl LemmaCandidate {
    /// Create a candidate with a meaningful initial score derived from the
    /// lemma type's specificity and a default `LibrarySearchConfig`.
    ///
    /// The score is computed as:
    /// - `specificity × specificity_weight` (from the default config)
    /// - plus a bonus if the lemma type is a proposition
    /// - plus a local-hypothesis bonus if applicable
    ///
    /// `proof` and `applied_args` are left empty because a bare candidate
    /// constructed this way has not yet been unified against a goal.  Use
    /// `try_candidate` (inside `run_search`) to produce a fully evaluated
    /// candidate.
    pub(super) fn new(name: Name, ty: Expr) -> Self {
        let conclusion = strip_leading_pis_local(&ty);
        let specificity = compute_specificity_local(&conclusion);
        let config = LibrarySearchConfig::default();
        let criteria = ScoringCriteria {
            specificity,
            remaining_goals: 0,
            edit_distance: 0,
            is_local: false,
            num_universe_params: 0,
            num_synth_args: 0,
            total_args: count_leading_pis_local(&ty),
        };
        let score = criteria.score(&config);
        let suggestion = format!("exact {}", name);
        Self {
            name,
            ty,
            score,
            applied_args: Vec::new(),
            remaining_goals: 0,
            proof: None,
            suggestion,
            criteria,
        }
    }
    /// Create a candidate from a `LemmaEntry`, with a pre-computed specificity
    /// and an optional `is_local` flag.  The score is computed relative to the
    /// supplied `config`.
    pub fn from_entry(
        name: Name,
        ty: Expr,
        specificity: f64,
        is_local: bool,
        num_univ_params: usize,
        config: &LibrarySearchConfig,
    ) -> Self {
        let total_args = count_leading_pis_local(&ty);
        let criteria = ScoringCriteria {
            specificity,
            remaining_goals: 0,
            edit_distance: 0,
            is_local,
            num_universe_params: num_univ_params,
            num_synth_args: 0,
            total_args,
        };
        let score = criteria.score(config);
        let tactic = if is_local { "exact" } else { "apply" };
        let suggestion = format!("{} {}", tactic, name);
        Self {
            name,
            ty,
            score,
            applied_args: Vec::new(),
            remaining_goals: 0,
            proof: None,
            suggestion,
            criteria,
        }
    }
}
/// Internal bookkeeping for a single search invocation.
pub(super) struct SearchState {
    /// Configuration snapshot.
    pub(super) config: LibrarySearchConfig,
    /// Wall-clock start time.
    pub(super) start: Instant,
    /// Number of candidates tried so far.
    pub(super) candidates_tried: usize,
    /// Cache of names that have already failed (avoid retrying).
    pub(super) failed_cache: HashSet<String>,
    /// Best results found so far (sorted by score descending).
    pub(super) results: Vec<LemmaCandidate>,
}
impl SearchState {
    pub(super) fn new(config: LibrarySearchConfig) -> Self {
        Self {
            config,
            start: Instant::now(),
            candidates_tried: 0,
            failed_cache: HashSet::new(),
            results: Vec::new(),
        }
    }
    /// Check whether the search has exhausted its budget.
    pub(super) fn is_budget_exhausted(&self) -> bool {
        if self.candidates_tried >= self.config.max_candidates {
            return true;
        }
        if self.config.timeout_ms > 0 {
            let elapsed = self.start.elapsed().as_millis() as u64;
            if elapsed >= self.config.timeout_ms {
                return true;
            }
        }
        false
    }
    /// Check whether we have timed out specifically.
    pub(super) fn is_timed_out(&self) -> bool {
        if self.config.timeout_ms > 0 {
            let elapsed = self.start.elapsed().as_millis() as u64;
            return elapsed >= self.config.timeout_ms;
        }
        false
    }
    /// Record a successful candidate.
    pub(super) fn record_result(&mut self, candidate: LemmaCandidate) {
        self.results.push(candidate);
        self.results.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        if self.results.len() > self.config.max_results {
            self.results.truncate(self.config.max_results);
        }
    }
    /// Mark a name as failed so we skip it on subsequent passes.
    pub(super) fn mark_failed(&mut self, name: &Name) {
        self.failed_cache.insert(format!("{}", name));
    }
    /// Whether this name already failed.
    pub(super) fn already_failed(&self, name: &Name) -> bool {
        self.failed_cache.contains(&format!("{}", name))
    }
}
/// A simplified type representation used for structural matching in type-directed search.
///
/// This is a string-based approximation of types, not dependent on the kernel's
/// `Expr` representation, making it suitable for lightweight lemma indexing.
#[derive(Clone, Debug, PartialEq)]
pub struct TypeSignature {
    /// The head type constructor name (e.g., "Nat", "List", "Eq", "->").
    pub head: String,
    /// Type arguments applied to the head constructor.
    pub args: Vec<TypeSignature>,
    /// Whether this signature represents a proposition (lives in Prop).
    pub is_prop: bool,
}
impl TypeSignature {
    /// Parse a simplified type string into a `TypeSignature`.
    ///
    /// Supports:
    /// - Plain identifiers: `"Nat"`, `"Bool"`
    /// - Applied types: `"List Nat"`, `"Eq Nat a b"`
    /// - Arrows: `"Nat -> Bool"` (right-associative)
    /// - Wildcard: `"_"` matches any type
    pub fn parse_type(s: &str) -> Self {
        let s = s.trim();
        if let Some(idx) = find_top_level_arrow(s) {
            let lhs = &s[..idx].trim_end();
            let rhs = &s[idx + 2..].trim_start();
            return TypeSignature {
                head: "->".to_string(),
                args: vec![
                    TypeSignature::parse_type(lhs),
                    TypeSignature::parse_type(rhs),
                ],
                is_prop: false,
            };
        }
        if s.starts_with('(') && s.ends_with(')') {
            let inner = &s[1..s.len() - 1];
            return TypeSignature::parse_type(inner);
        }
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.is_empty() {
            return TypeSignature {
                head: "_".to_string(),
                args: vec![],
                is_prop: false,
            };
        }
        let head = parts[0].to_string();
        let is_prop = matches!(
            head.as_str(),
            "Prop" | "True" | "False" | "And" | "Or" | "Not" | "Iff" | "Eq"
        );
        let args = parts[1..]
            .iter()
            .map(|a| TypeSignature::parse_type(a))
            .collect();
        TypeSignature {
            head,
            args,
            is_prop,
        }
    }
    /// Structural matching with wildcard support.
    ///
    /// A `_` wildcard in `self` matches any sub-signature in `other`.
    pub fn matches(&self, other: &TypeSignature) -> bool {
        if self.head == "_" {
            return true;
        }
        if self.head != other.head {
            return false;
        }
        if self.args.len() != other.args.len() {
            if self.args.is_empty() {
                return true;
            }
            return false;
        }
        self.args
            .iter()
            .zip(other.args.iter())
            .all(|(a, b)| a.matches(b))
    }
}
/// An entry stored inside the discrimination tree.
#[derive(Clone, Debug)]
pub(super) struct LemmaEntry {
    /// Declaration name.
    pub(super) name: Name,
    /// The full type of the declaration.
    pub(super) ty: Expr,
    /// Pre-computed specificity.
    pub(super) specificity: f64,
    /// Number of universe parameters.
    pub(super) num_univ_params: usize,
    /// Whether this entry originates from a local hypothesis.
    pub(super) is_local: bool,
}
/// DiscrTree-backed index for fast lemma lookup by conclusion type.
#[derive(Clone, Debug)]
pub struct LemmaIndex {
    /// The underlying discrimination tree, keyed by conclusion type.
    pub(super) tree: DiscrTree<LemmaEntry>,
    /// Set of names already inserted (avoids duplicates).
    pub(super) inserted: HashSet<String>,
    /// Total number of entries.
    pub(super) count: usize,
}
impl LemmaIndex {
    /// Create a new, empty lemma index.
    pub fn new() -> Self {
        Self {
            tree: DiscrTree::new(),
            inserted: HashSet::new(),
            count: 0,
        }
    }
    /// Number of indexed lemmas.
    pub fn len(&self) -> usize {
        self.count
    }
    /// Whether the index is empty.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
    /// Insert a lemma into the index.
    ///
    /// The conclusion is extracted from the type by stripping leading Pi
    /// binders, and the resulting body is used as the discrimination-tree key.
    pub fn insert(&mut self, name: Name, ty: Expr, num_univ_params: usize, is_local: bool) {
        let name_str = format!("{}", name);
        if self.inserted.contains(&name_str) {
            return;
        }
        let conclusion = strip_leading_pis(&ty);
        let specificity = compute_specificity(&conclusion);
        let entry = LemmaEntry {
            name: name.clone(),
            ty,
            specificity,
            num_univ_params,
            is_local,
        };
        self.tree.insert(&conclusion, entry);
        self.inserted.insert(name_str);
        self.count += 1;
    }
    /// Look up lemmas whose conclusion unifies with `goal_type`.
    pub(super) fn lookup(&self, goal_type: &Expr) -> Vec<&LemmaEntry> {
        self.tree.find(goal_type)
    }
    /// Brute-force: return all entries.
    pub(super) fn all_entries(&self) -> Vec<&LemmaEntry> {
        self.tree.all_values()
    }
    /// Clear the index.
    pub fn clear(&mut self) {
        self.tree.clear();
        self.inserted.clear();
        self.count = 0;
    }
    /// Build an index from every declaration in a `MetaContext`'s environment.
    pub fn from_environment(ctx: &MetaContext) -> Self {
        let mut index = Self::new();
        let env = ctx.env();
        for (name, ci) in env.constant_infos() {
            let ty = ci.ty().clone();
            let num_univ = ci.level_params().len();
            if is_search_candidate(name, ci) {
                index.insert(name.clone(), ty, num_univ, false);
            }
        }
        index
    }
    /// Add local hypotheses from the current MetaContext.
    pub fn add_local_hyps(&mut self, ctx: &MetaContext) {
        for (name, ty) in ctx.get_local_hyps() {
            self.insert(name, ty, 0, true);
        }
    }
}
/// How candidates are ranked.
#[derive(Clone, Debug)]
pub struct ScoringCriteria {
    /// Specificity: ratio of concrete (non-wildcard) keys in the encoded type.
    pub specificity: f64,
    /// Number of remaining goals after applying the candidate.
    pub remaining_goals: usize,
    /// Edit distance between the candidate type and the goal type.
    pub edit_distance: u32,
    /// Whether the candidate is a local hypothesis (bonus).
    pub is_local: bool,
    /// Number of universe parameters that had to be freshened.
    pub num_universe_params: usize,
    /// Number of implicit arguments that were successfully synthesised.
    pub num_synth_args: usize,
    /// Total number of arguments the lemma expects.
    pub total_args: usize,
}
impl ScoringCriteria {
    /// Compute a single scalar score (higher is better).
    pub fn score(&self, config: &LibrarySearchConfig) -> f64 {
        let specificity_term = self.specificity * config.specificity_weight;
        let remaining_penalty = self.remaining_goals as f64 * config.remaining_goals_weight;
        let edit_penalty = self.edit_distance as f64 * config.edit_distance_weight;
        let local_bonus = if self.is_local { 2.0 } else { 0.0 };
        let synth_bonus = if self.total_args > 0 {
            self.num_synth_args as f64 / self.total_args as f64
        } else {
            0.0
        };
        specificity_term + local_bonus + synth_bonus - remaining_penalty - edit_penalty
    }
}
/// A thread-local cache that remembers failed lookups so we can skip them
/// when the same goal type appears again.
#[derive(Clone, Debug, Default)]
pub struct SearchCache {
    /// Failed goal-type hashes.
    pub(super) failed: HashSet<u64>,
    /// Successful results keyed by a hash of the goal type.
    pub(super) success: HashMap<u64, Vec<LemmaCandidate>>,
}
impl SearchCache {
    /// Create an empty cache.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record a failure.
    pub fn record_failure(&mut self, goal_hash: u64) {
        self.failed.insert(goal_hash);
    }
    /// Record success.
    pub fn record_success(&mut self, goal_hash: u64, candidates: Vec<LemmaCandidate>) {
        self.success.insert(goal_hash, candidates);
    }
    /// Look up a cached result.
    pub fn lookup(&self, goal_hash: u64) -> Option<CacheLookup> {
        if self.failed.contains(&goal_hash) {
            return Some(CacheLookup::Failed);
        }
        self.success
            .get(&goal_hash)
            .map(|cs| CacheLookup::Found(cs.clone()))
    }
    /// Clear the cache.
    pub fn clear(&mut self) {
        self.failed.clear();
        self.success.clear();
    }
    /// Number of cached entries.
    pub fn len(&self) -> usize {
        self.failed.len() + self.success.len()
    }
    /// Whether the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.failed.is_empty() && self.success.is_empty()
    }
}
/// A single result from the type-directed search, carrying relevance metadata.
#[derive(Clone, Debug)]
pub struct TypeSearchResult {
    /// Lemma name.
    pub name: String,
    /// Human-readable type string.
    pub type_str: String,
    /// Relevance score (higher is better).
    pub score: f32,
    /// How the lemma is expected to be used: `"exact"`, `"apply"`, `"rewrite"`.
    pub how_used: String,
}
impl TypeSearchResult {
    /// Construct a new search result.
    pub fn new(name: &str, ty: &str, score: f32, how: &str) -> Self {
        Self {
            name: name.to_string(),
            type_str: ty.to_string(),
            score,
            how_used: how.to_string(),
        }
    }
}
/// Outcome of a cache lookup.
#[derive(Clone, Debug)]
pub enum CacheLookup {
    /// Previously recorded failure.
    Failed,
    /// Previously found candidates.
    Found(Vec<LemmaCandidate>),
}
/// High-level type-directed lemma search engine.
///
/// Uses `SimpleLemmaIndex` for lightweight string-based heuristic search.
pub struct TypeDirectedSearch {
    /// The underlying lemma index.
    pub index: SimpleLemmaIndex,
}
impl TypeDirectedSearch {
    /// Create with an empty index.
    pub fn new() -> Self {
        Self {
            index: SimpleLemmaIndex::new(),
        }
    }
    /// Create pre-populated with common lemmas.
    pub fn with_prebuilt() -> Self {
        Self {
            index: SimpleLemmaIndex::prebuilt(),
        }
    }
    /// Find lemmas that could directly close (or help close) a goal of the given type.
    ///
    /// Returns results scored by how well the lemma type matches the goal type.
    pub fn search_for_goal(&self, goal_type: &str) -> Vec<TypeSearchResult> {
        let query = TypeSignature::parse_type(goal_type);
        let mut results = Vec::new();
        for (name, sig) in &self.index.entries {
            if query.matches(sig) {
                let score = score_match(&query, sig, true);
                results.push(TypeSearchResult::new(
                    name,
                    &sig.to_string(),
                    score,
                    "exact",
                ));
            } else if sig.head == "->" && sig.args.len() == 2 && query.matches(&sig.args[1]) {
                let score = score_match(&query, &sig.args[1], false) * 0.8;
                results.push(TypeSearchResult::new(
                    name,
                    &sig.to_string(),
                    score,
                    "apply",
                ));
            }
        }
        self.rank_results(results)
    }
    /// Find lemmas suitable for rewriting — i.e., whose conclusion is an equality.
    pub fn search_for_rewrite(&self, goal_type: &str) -> Vec<TypeSearchResult> {
        let goal_sig = TypeSignature::parse_type(goal_type);
        let eq_query = TypeSignature::parse_type("Eq _ _ _");
        let iff_query = TypeSignature::parse_type("Iff _ _");
        let mut results = Vec::new();
        for (name, sig) in &self.index.entries {
            let conclusion = conclusion_of(sig);
            if eq_query.matches(conclusion) || iff_query.matches(conclusion) {
                let relevance =
                    if conclusion.args.len() >= 3 && goal_sig.head == conclusion.args[1].head {
                        0.9_f32
                    } else {
                        0.5_f32
                    };
                results.push(TypeSearchResult::new(
                    name,
                    &sig.to_string(),
                    relevance,
                    "rewrite",
                ));
            }
        }
        self.rank_results(results)
    }
    /// Sort results by score descending.
    pub fn rank_results(&self, mut results: Vec<TypeSearchResult>) -> Vec<TypeSearchResult> {
        results.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        results
    }
}
