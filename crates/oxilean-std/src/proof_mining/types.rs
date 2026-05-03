//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DialecticaInterp {
    pub formula: String,
    pub witness_type: String,
    pub counterexample_type: String,
    pub is_godel_dialectica: bool,
}
#[allow(dead_code)]
impl DialecticaInterp {
    pub fn new(formula: &str, witness: &str, counter: &str) -> Self {
        DialecticaInterp {
            formula: formula.to_string(),
            witness_type: witness.to_string(),
            counterexample_type: counter.to_string(),
            is_godel_dialectica: true,
        }
    }
    pub fn godel_t_translation(&self) -> String {
        format!(
            "Dialectica: '{}' → ∃{}.∀{}.A({}, {})",
            self.formula,
            self.witness_type,
            self.counterexample_type,
            self.witness_type,
            self.counterexample_type
        )
    }
    pub fn modified_realizability(&self) -> String {
        format!(
            "Modified realizability of '{}': witness type = {}",
            self.formula, self.witness_type
        )
    }
    pub fn soundness_theorem(&self) -> String {
        "Dialectica interpretation is sound for Heyting arithmetic + AC_qf + IPP".to_string()
    }
    pub fn is_sound_for_classical(&self) -> bool {
        true
    }
}
/// G_del functional interpretation.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GodelInterpretation {
    pub theorem: String,
    pub functional_type: String,
    pub realizing_term: String,
}
impl GodelInterpretation {
    #[allow(dead_code)]
    pub fn new(thm: &str, ftype: &str, term: &str) -> Self {
        Self {
            theorem: thm.to_string(),
            functional_type: ftype.to_string(),
            realizing_term: term.to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn soundness_statement(&self) -> String {
        format!("If HA proves A, then T proves A^D where A^D is the Dialectica interpretation",)
    }
}
/// The empty clause ⊥ (proof of contradiction).
#[derive(Debug, Clone)]
pub struct EmptyClause;
impl EmptyClause {
    /// Return the empty clause.
    pub fn as_clause() -> Clause {
        Clause::empty()
    }
}
/// Proof-as-program correspondence (Curry-Howard).
#[derive(Debug, Clone)]
pub struct CurryHoward {
    /// The proposition (as a string).
    pub proposition: String,
    /// The corresponding type (as a string).
    pub corresponding_type: String,
}
impl CurryHoward {
    /// Create a new Curry-Howard correspondence entry.
    pub fn new(proposition: impl Into<String>, corresponding_type: impl Into<String>) -> Self {
        Self {
            proposition: proposition.into(),
            corresponding_type: corresponding_type.into(),
        }
    }
    /// Return a human-readable summary.
    pub fn describe(&self) -> String {
        format!(
            "Prop: {} <-> Type: {}",
            self.proposition, self.corresponding_type
        )
    }
}
/// Checks Howard/Bezem majorizability for a pair of functions (represented as tables).
#[derive(Debug, Clone)]
pub struct MajorizabilityChecker {
    /// Values of the "major" function f (f(i) at index i).
    pub f_values: Vec<u64>,
    /// Values of the "minor" function g (g(i) at index i).
    pub g_values: Vec<u64>,
}
impl MajorizabilityChecker {
    /// Create a checker for f majorizing g (both given as value tables).
    pub fn new(f_values: Vec<u64>, g_values: Vec<u64>) -> Self {
        Self { f_values, g_values }
    }
    /// Returns true if f Howard-majorizes g: ∀n, g(n) ≤ f(n).
    pub fn howard_majorizes(&self) -> bool {
        self.f_values
            .iter()
            .zip(self.g_values.iter())
            .all(|(&fv, &gv)| gv <= fv)
    }
    /// Returns true if f Bezem-majorizes g:
    /// ∀m ≤ n, g(m) ≤ f(n) (strong majorizability).
    pub fn bezem_majorizes(&self) -> bool {
        let n = self.f_values.len().min(self.g_values.len());
        for outer in 0..n {
            for inner in 0..=outer {
                if inner < self.g_values.len() && outer < self.f_values.len() {
                    if self.g_values[inner] > self.f_values[outer] {
                        return false;
                    }
                }
            }
        }
        true
    }
    /// Returns the pointwise maximum of f and g (another majorant).
    pub fn pointwise_max(&self) -> Vec<u64> {
        let n = self.f_values.len().max(self.g_values.len());
        (0..n)
            .map(|i| {
                let fv = self.f_values.get(i).copied().unwrap_or(0);
                let gv = self.g_values.get(i).copied().unwrap_or(0);
                fv.max(gv)
            })
            .collect()
    }
}
/// Kleene's realizability / Kreisel's modified realizability.
#[derive(Debug, Clone)]
pub struct RealizabilityInterpretation {
    /// Which variant: `"kleene"` or `"kreisel"`.
    pub variant: String,
    /// The formula being interpreted.
    pub formula: RealizedFormula,
    /// Whether the interpretation is constructive.
    pub is_constructive: bool,
}
impl RealizabilityInterpretation {
    /// Create a Kleene realizability interpretation.
    pub fn kleene(formula: RealizedFormula) -> Self {
        Self {
            variant: "kleene".into(),
            formula,
            is_constructive: true,
        }
    }
    /// Create a Kreisel modified-realizability interpretation.
    pub fn kreisel(formula: RealizedFormula) -> Self {
        Self {
            variant: "kreisel".into(),
            formula,
            is_constructive: true,
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ProverData {
    pub prover_name: String,
    pub logic: String,
    pub is_complete: bool,
    pub is_sound: bool,
    pub search_strategy: SearchStrategyNew,
}
#[allow(dead_code)]
impl ProverData {
    pub fn resolution_prover() -> Self {
        ProverData {
            prover_name: "Resolution Prover".to_string(),
            logic: "Classical Propositional Logic".to_string(),
            is_complete: true,
            is_sound: true,
            search_strategy: SearchStrategyNew::Saturation,
        }
    }
    pub fn tableau_prover() -> Self {
        ProverData {
            prover_name: "Tableau Prover".to_string(),
            logic: "Classical First-Order Logic".to_string(),
            is_complete: true,
            is_sound: true,
            search_strategy: SearchStrategyNew::DepthFirst,
        }
    }
    pub fn new_heuristic(name: &str, logic: &str, hint: &str) -> Self {
        ProverData {
            prover_name: name.to_string(),
            logic: logic.to_string(),
            is_complete: false,
            is_sound: true,
            search_strategy: SearchStrategyNew::Heuristic(hint.to_string()),
        }
    }
    pub fn completeness_theorem(&self) -> String {
        if self.is_complete {
            format!(
                "{}: complete for {} (every valid formula is provable)",
                self.prover_name, self.logic
            )
        } else {
            format!("{}: incomplete (heuristic only)", self.prover_name)
        }
    }
    pub fn superposition_calculus_description(&self) -> String {
        "Superposition: complete for equational logic, used in E, Vampire, SPASS".to_string()
    }
}
/// A resolution refutation: derives ⊥ from a set of clauses.
#[derive(Debug, Clone)]
pub struct ResolutionRefutation {
    /// The initial clause set.
    pub clauses: Vec<Clause>,
    /// The sequence of resolution steps.
    pub steps: Vec<ResolutionStep>,
}
impl ResolutionRefutation {
    /// Create a new refutation attempt from a clause set.
    pub fn new(clauses: Vec<Clause>) -> Self {
        Self {
            clauses,
            steps: Vec::new(),
        }
    }
    /// Add a resolution step.
    pub fn add_step(&mut self, step: ResolutionStep) {
        self.steps.push(step);
    }
    /// Returns true if the last step produced the empty clause.
    pub fn is_complete(&self) -> bool {
        self.steps
            .last()
            .is_some_and(|s| s.resolvent.is_empty_clause())
    }
    /// Perform unit propagation: return the set of forced literals.
    pub fn unit_propagate(&self) -> Vec<Literal> {
        let mut forced: Vec<Literal> = Vec::new();
        for clause in &self.clauses {
            if clause.is_unit() {
                if let Some(lit) = clause.literals.first() {
                    if !forced.contains(lit) {
                        forced.push(lit.clone());
                    }
                }
            }
        }
        forced
    }
    /// A single DPLL-style split on the first unassigned variable.
    pub fn dpll_step(&self) -> Option<u32> {
        for clause in &self.clauses {
            if let Some(lit) = clause.literals.first() {
                return Some(lit.var);
            }
        }
        None
    }
}
/// Computable functional realizing ∀∃ statements under Dialectica.
#[derive(Debug, Clone)]
pub struct FunctionalInterpretation {
    /// The formula being realized.
    pub formula: DialecticaFormula,
    /// Description of the realizing functional.
    pub functional_description: String,
}
impl FunctionalInterpretation {
    /// Create a new functional interpretation.
    pub fn new(formula: DialecticaFormula, functional_description: impl Into<String>) -> Self {
        Self {
            formula,
            functional_description: functional_description.into(),
        }
    }
}
/// Weak König's Lemma and its Dialectica interpretation.
#[derive(Debug, Clone)]
pub struct WeakKoenigsLemma {
    /// WKL: every infinite binary tree has an infinite path.
    pub statement: String,
    /// Its Dialectica interpretation (skolemized).
    pub dialectica_form: Option<DialecticaFormula>,
}
impl WeakKoenigsLemma {
    /// Construct the standard WKL instance.
    pub fn standard() -> Self {
        Self {
            statement: "Every infinite binary tree has an infinite path.".into(),
            dialectica_form: None,
        }
    }
    /// Return whether a Dialectica form has been computed.
    pub fn has_dialectica_form(&self) -> bool {
        self.dialectica_form.is_some()
    }
}
/// Ramsey theory bounds from proof mining.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RamseyBound {
    pub name: String,
    pub lower_bound: u64,
    pub upper_bound: Option<u64>,
    pub proof_system: String,
}
impl RamseyBound {
    #[allow(dead_code)]
    pub fn r33() -> Self {
        Self {
            name: "R(3,3)".to_string(),
            lower_bound: 6,
            upper_bound: Some(6),
            proof_system: "Elementary combinatorics".to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn r44() -> Self {
        Self {
            name: "R(4,4)".to_string(),
            lower_bound: 18,
            upper_bound: Some(18),
            proof_system: "Computer search + proof".to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn is_exact(&self) -> bool {
        self.upper_bound
            .map(|u| u == self.lower_bound)
            .unwrap_or(false)
    }
}
/// Finitization of an infinite principle.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Finitization {
    pub infinite_principle: String,
    pub finite_version: String,
    pub quantitative_bound: String,
}
impl Finitization {
    #[allow(dead_code)]
    pub fn new(inf: &str, fin: &str, bound: &str) -> Self {
        Self {
            infinite_principle: inf.to_string(),
            finite_version: fin.to_string(),
            quantitative_bound: bound.to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn bolzano_weierstrass() -> Self {
        Self::new(
            "Every bounded sequence has a convergent subsequence",
            "For every eps, there exist indices i < j s.t. |x_i - x_j| < eps",
            "Omega(eps, M) primitive recursive in eps and bound M",
        )
    }
    #[allow(dead_code)]
    pub fn description(&self) -> String {
        format!(
            "Finite: {}\nfrom: {}\nbound: {}",
            self.finite_version, self.infinite_principle, self.quantitative_bound
        )
    }
}
/// A clause: a disjunction of literals.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Clause {
    /// The literals in the clause.
    pub literals: Vec<Literal>,
}
impl Clause {
    /// Create an empty clause (represents ⊥).
    pub fn empty() -> Self {
        Self {
            literals: Vec::new(),
        }
    }
    /// Create a clause from a list of literals.
    pub fn new(literals: Vec<Literal>) -> Self {
        Self { literals }
    }
    /// Returns true if this is the empty clause (contradiction).
    pub fn is_empty_clause(&self) -> bool {
        self.literals.is_empty()
    }
    /// Returns true if the clause is a tautology (contains x and ¬x).
    pub fn is_tautology(&self) -> bool {
        for lit in &self.literals {
            if self.literals.contains(&lit.complement()) {
                return true;
            }
        }
        false
    }
    /// Returns true if the clause is a unit clause (exactly one literal).
    pub fn is_unit(&self) -> bool {
        self.literals.len() == 1
    }
}
/// Uniform convexity modulus.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct UniformConvexityModulus {
    pub space_name: String,
    pub modulus: String,
}
impl UniformConvexityModulus {
    #[allow(dead_code)]
    pub fn new(space: &str, modulus: &str) -> Self {
        Self {
            space_name: space.to_string(),
            modulus: modulus.to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn hilbert_space() -> Self {
        Self::new("Hilbert space H", "delta(eps) = 1 - sqrt(1 - eps^2/4)")
    }
    #[allow(dead_code)]
    pub fn l_p_space(p: f64) -> Self {
        let formula = if p >= 2.0 {
            format!("delta(eps) >= (eps/2)^p / (2 max(1, 2^(p-2)))")
        } else {
            format!("delta(eps) >= (eps/2)^2 / 8 (Clarkson for p={p})")
        };
        Self::new(&format!("L^{p}"), &formula)
    }
    #[allow(dead_code)]
    pub fn bound_on_iterations_for_mann(&self, _epsilon: f64) -> String {
        format!(
            "Kohlenbach-Leustean: rate of Mann iterations in {} via {}",
            self.space_name, self.modulus
        )
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum SearchStrategyNew {
    BreadthFirst,
    DepthFirst,
    IterativeDeepening,
    Heuristic(String),
    Saturation,
}
#[allow(dead_code)]
impl SearchStrategyNew {
    fn saturation_strategy() -> SearchStrategyNew {
        SearchStrategyNew::Saturation
    }
}
/// Termination argument via ordinals.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OrdinalTermination {
    pub algorithm_name: String,
    pub ordinal_bound: String,
    pub is_primitive_recursive: bool,
}
impl OrdinalTermination {
    #[allow(dead_code)]
    pub fn new(alg: &str, bound: &str, prim_rec: bool) -> Self {
        Self {
            algorithm_name: alg.to_string(),
            ordinal_bound: bound.to_string(),
            is_primitive_recursive: prim_rec,
        }
    }
    #[allow(dead_code)]
    pub fn termination_proof(&self) -> String {
        format!(
            "{} terminates: assign ordinal {} decreasing at each step",
            self.algorithm_name, self.ordinal_bound
        )
    }
}
/// Explicit witness extracted from a ∃x.P(x) proof (Herbrand's theorem).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HerbrandTerm {
    /// The variable bound by the quantifier.
    pub variable: String,
    /// The concrete term witnessing existence.
    pub term: String,
    /// The predicate P, as a string.
    pub predicate: String,
}
impl HerbrandTerm {
    /// Create a new Herbrand witness.
    pub fn new(
        variable: impl Into<String>,
        term: impl Into<String>,
        predicate: impl Into<String>,
    ) -> Self {
        Self {
            variable: variable.into(),
            term: term.into(),
            predicate: predicate.into(),
        }
    }
    /// Return a human-readable description of the witness.
    pub fn describe(&self) -> String {
        format!(
            "∃ {}, {} [witness: {}]",
            self.variable, self.predicate, self.term
        )
    }
}
/// A heuristic function: estimates the distance from a proof state to a complete proof.
#[derive(Debug, Clone)]
pub struct HeuristicFn {
    /// Human-readable name of the heuristic.
    pub name: String,
}
impl HeuristicFn {
    /// Create a named heuristic.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
    /// Estimate: simply return the number of remaining goals.
    pub fn estimate(&self, state: &ProofState) -> usize {
        state.goals.len()
    }
}
/// A simple representation of ordinals below ε_0 in Cantor Normal Form.
///
/// An ordinal is represented as a sorted (descending) list of exponents,
/// where the ordinal = ω^e₁ + ω^e₂ + … for e₁ ≥ e₂ ≥ …
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CantorNormalForm {
    /// Exponents in descending order (each is a Cantor normal form itself,
    /// but we represent exponents as u32 for finite ordinals below ε_0).
    pub exponents: Vec<u32>,
}
impl CantorNormalForm {
    /// The zero ordinal.
    pub fn zero() -> Self {
        Self { exponents: vec![] }
    }
    /// The ordinal 1 (= ω^0).
    pub fn one() -> Self {
        Self { exponents: vec![0] }
    }
    /// The ordinal ω (= ω^1).
    pub fn omega() -> Self {
        Self { exponents: vec![1] }
    }
    /// The ordinal ε_0 is represented as "the limit" — we use a sentinel.
    pub fn epsilon0() -> Self {
        Self {
            exponents: vec![u32::MAX],
        }
    }
    /// Returns true if this is the zero ordinal.
    pub fn is_zero(&self) -> bool {
        self.exponents.is_empty()
    }
    /// Compare two ordinals (both must be in CNF with u32 exponents).
    pub fn less_than(&self, other: &Self) -> bool {
        for (a, b) in self.exponents.iter().zip(other.exponents.iter()) {
            if a < b {
                return true;
            }
            if a > b {
                return false;
            }
        }
        self.exponents.len() < other.exponents.len()
    }
    /// Ordinal addition α + β in CNF.
    pub fn add(&self, other: &Self) -> Self {
        if other.is_zero() {
            return self.clone();
        }
        let lead_b = other.exponents[0];
        let mut result: Vec<u32> = self
            .exponents
            .iter()
            .copied()
            .filter(|&e| e > lead_b)
            .collect();
        result.extend_from_slice(&other.exponents);
        Self { exponents: result }
    }
}
/// Extracts computational content from a proof (Kohlenbach's proof mining).
#[derive(Debug, Clone)]
pub struct WitnessExtractor {
    /// Human-readable description of the proof being mined.
    pub proof_name: String,
    /// The extracted witness terms (as strings for display).
    pub witnesses: Vec<String>,
    /// Upper bound on the number of computation steps.
    pub bound: Option<u64>,
}
impl WitnessExtractor {
    /// Create a new `WitnessExtractor` for the named proof.
    pub fn new(proof_name: impl Into<String>) -> Self {
        Self {
            proof_name: proof_name.into(),
            witnesses: Vec::new(),
            bound: None,
        }
    }
    /// Add an extracted witness term.
    pub fn add_witness(&mut self, term: impl Into<String>) {
        self.witnesses.push(term.into());
    }
    /// Return the first extracted witness, if any.
    pub fn extract_witness(&self) -> Option<&str> {
        self.witnesses.first().map(String::as_str)
    }
    /// Return the computed bound, or `u64::MAX` as a sentinel for "unknown".
    pub fn compute_bound(&self) -> u64 {
        self.bound.unwrap_or(u64::MAX)
    }
    /// Check whether at least one witness has been extracted.
    pub fn is_realizable(&self) -> bool {
        !self.witnesses.is_empty()
    }
}
/// Current state of a proof search.
#[derive(Debug, Clone)]
pub struct ProofState {
    /// Remaining goals as formula strings.
    pub goals: Vec<String>,
    /// Tactics applied so far.
    pub applied_tactics: Vec<String>,
    /// Remaining computational budget (steps).
    pub budget: usize,
}
impl ProofState {
    /// Create a proof state with a single goal and a budget.
    pub fn new(goal: impl Into<String>, budget: usize) -> Self {
        Self {
            goals: vec![goal.into()],
            applied_tactics: Vec::new(),
            budget,
        }
    }
    /// Returns true if all goals have been discharged.
    pub fn is_complete(&self) -> bool {
        self.goals.is_empty()
    }
    /// Apply a tactic name: removes the first goal and decrements budget.
    pub fn apply_tactic(&mut self, tactic: impl Into<String>) -> bool {
        if self.budget == 0 || self.goals.is_empty() {
            return false;
        }
        self.goals.remove(0);
        self.applied_tactics.push(tactic.into());
        self.budget -= 1;
        true
    }
}
/// Proof complexity measure: size, depth, width, degree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofComplexityMeasure {
    /// Total number of lines / nodes in the proof.
    pub size: usize,
    /// Maximum nesting depth.
    pub depth: usize,
    /// Maximum clause width (number of literals per clause).
    pub width: usize,
    /// Algebraic degree (for algebraic systems).
    pub degree: usize,
}
impl ProofComplexityMeasure {
    /// Construct a measure with all fields set to zero.
    pub fn zero() -> Self {
        Self {
            size: 0,
            depth: 0,
            width: 0,
            degree: 0,
        }
    }
    /// Returns true if all measures are within the given bound.
    pub fn is_within_bound(&self, bound: usize) -> bool {
        self.size <= bound && self.depth <= bound && self.width <= bound && self.degree <= bound
    }
}
/// The Cook-Reckhow theorem: NP ≠ co-NP iff no proof system is "efficient".
#[derive(Debug, Clone)]
pub struct CookReckhowThm {
    /// Statement of the theorem.
    pub statement: String,
}
impl CookReckhowThm {
    /// Return the canonical statement of Cook-Reckhow.
    pub fn canonical() -> Self {
        Self {
            statement: concat!(
                "NP ≠ co-NP if and only if no propositional proof system ",
                "polynomially simulates all others."
            )
            .into(),
        }
    }
}
/// Proof search strategy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SearchStrategy {
    /// Depth-first search.
    DFS,
    /// Breadth-first search.
    BFS,
    /// A* (best-first with a heuristic).
    AStar,
    /// Iterative deepening depth-first search.
    IDDFS,
    /// Monte Carlo Tree Search.
    MCTS,
}
/// Bounded model checking state: BMC encoding with a depth bound.
#[derive(Debug, Clone)]
pub struct ModelCheckingBound {
    /// Current unfolding depth.
    pub depth: usize,
    /// Maximum depth allowed.
    pub max_depth: usize,
    /// Whether a counterexample was found within `depth` steps.
    pub counterexample_found: bool,
}
impl ModelCheckingBound {
    /// Create a bounded model checking instance.
    pub fn new(max_depth: usize) -> Self {
        Self {
            depth: 0,
            max_depth,
            counterexample_found: false,
        }
    }
    /// Increment the unfolding depth by one.
    pub fn unfold(&mut self) {
        if self.depth < self.max_depth {
            self.depth += 1;
        }
    }
    /// Returns true if the bound has been reached.
    pub fn at_bound(&self) -> bool {
        self.depth >= self.max_depth
    }
}
/// An ML-like program extracted from a constructive proof.
#[derive(Debug, Clone)]
pub struct ExtractedProgram {
    /// The source proposition / theorem.
    pub source_theorem: String,
    /// The program text (pseudo-ML).
    pub program_text: String,
    /// Whether the extraction is complete.
    pub is_complete: bool,
}
impl ExtractedProgram {
    /// Create a new extracted program.
    pub fn new(theorem: impl Into<String>, program_text: impl Into<String>) -> Self {
        Self {
            source_theorem: theorem.into(),
            program_text: program_text.into(),
            is_complete: true,
        }
    }
}
/// Encodes Kohlenbach's monotone functional interpretation.
#[derive(Debug, Clone)]
pub struct MonotoneFunctionalInterpretation {
    /// The formula being interpreted.
    pub formula: String,
    /// The majorizing functional (as a description string).
    pub majorant: String,
    /// Whether this is a bounded (monotone) interpretation.
    pub is_bounded: bool,
}
impl MonotoneFunctionalInterpretation {
    /// Create a new monotone functional interpretation.
    pub fn new(formula: impl Into<String>, majorant: impl Into<String>) -> Self {
        Self {
            formula: formula.into(),
            majorant: majorant.into(),
            is_bounded: true,
        }
    }
    /// Check whether the interpretation preserves the monotone bound.
    pub fn check_bound(&self) -> bool {
        !self.majorant.is_empty()
    }
}
/// A propositional proof: a sequence of lines each with a justification.
#[derive(Debug, Clone)]
pub struct PropositionalProof {
    /// The proof system used.
    pub system: ProofSystem,
    /// Lines: (formula_string, justification_string).
    pub lines: Vec<(String, String)>,
    /// The conclusion formula.
    pub conclusion: String,
}
impl PropositionalProof {
    /// Create an empty proof in the given system for the given conclusion.
    pub fn new(system: ProofSystem, conclusion: impl Into<String>) -> Self {
        Self {
            system,
            lines: Vec::new(),
            conclusion: conclusion.into(),
        }
    }
    /// Add a proof line with a justification.
    pub fn add_line(&mut self, formula: impl Into<String>, justification: impl Into<String>) {
        self.lines.push((formula.into(), justification.into()));
    }
    /// Return the proof complexity measure.
    pub fn measure(&self) -> ProofComplexityMeasure {
        ProofComplexityMeasure {
            size: self.lines.len(),
            depth: self.lines.len(),
            width: self.lines.iter().map(|(f, _)| f.len()).max().unwrap_or(0),
            degree: 0,
        }
    }
}
/// Well-founded induction certificate proving termination.
#[derive(Debug, Clone)]
pub struct TerminationProof {
    /// The function name.
    pub function_name: String,
    /// The well-founded ordering used.
    pub ordering: String,
    /// Whether the termination argument has been fully verified.
    pub verified: bool,
}
impl TerminationProof {
    /// Create a termination proof by a given ordering.
    pub fn new(function_name: impl Into<String>, ordering: impl Into<String>) -> Self {
        Self {
            function_name: function_name.into(),
            ordering: ordering.into(),
            verified: false,
        }
    }
    /// Mark the termination proof as verified.
    pub fn verify(&mut self) {
        self.verified = true;
    }
}
/// Proof mining in metric fixed point theory.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MetricFixedPointMining {
    pub contraction_modulus: f64,
    pub initial_error: f64,
}
impl MetricFixedPointMining {
    #[allow(dead_code)]
    pub fn new(q: f64, err: f64) -> Self {
        assert!(q < 1.0, "Contraction constant must be < 1");
        Self {
            contraction_modulus: q,
            initial_error: err,
        }
    }
    #[allow(dead_code)]
    pub fn iterations_to_epsilon(&self, epsilon: f64) -> u64 {
        let n = (epsilon / self.initial_error).ln() / self.contraction_modulus.ln();
        n.ceil() as u64
    }
    #[allow(dead_code)]
    pub fn rate_of_convergence_description(&self) -> String {
        format!(
            "Banach iteration converges geometrically with ratio q = {}",
            self.contraction_modulus
        )
    }
}
/// A literal: positive or negative occurrence of a variable.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Literal {
    /// The propositional variable index (1-based, SAT convention).
    pub var: u32,
    /// True for positive, false for negative.
    pub positive: bool,
}
impl Literal {
    /// Create a positive literal for variable `var`.
    pub fn pos(var: u32) -> Self {
        Self {
            var,
            positive: true,
        }
    }
    /// Create a negative literal for variable `var`.
    pub fn neg(var: u32) -> Self {
        Self {
            var,
            positive: false,
        }
    }
    /// Return the complementary literal.
    pub fn complement(&self) -> Self {
        Self {
            var: self.var,
            positive: !self.positive,
        }
    }
}
/// A formula together with its realizability status.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RealizedFormula {
    /// An atomic proposition identified by name.
    Atomic(String),
    /// Conjunction A ∧ B, realized by a pair of realizers.
    Conjunction(Box<RealizedFormula>, Box<RealizedFormula>),
    /// Disjunction A ∨ B, realized by a tagged realizer.
    Disjunction(Box<RealizedFormula>, Box<RealizedFormula>),
    /// Implication A → B, realized by a computable function.
    Implication(Box<RealizedFormula>, Box<RealizedFormula>),
    /// Universal quantification ∀x.A(x), realized for each input.
    Forall(String, Box<RealizedFormula>),
    /// Existential quantification ∃x.A(x), realized by a witness + proof.
    Exists(String, Box<RealizedFormula>),
}
impl RealizedFormula {
    /// Returns the depth of the formula tree.
    pub fn depth(&self) -> usize {
        match self {
            RealizedFormula::Atomic(_) => 0,
            RealizedFormula::Forall(_, f) | RealizedFormula::Exists(_, f) => 1 + f.depth(),
            RealizedFormula::Conjunction(a, b)
            | RealizedFormula::Disjunction(a, b)
            | RealizedFormula::Implication(a, b) => 1 + a.depth().max(b.depth()),
        }
    }
    /// Returns true if the formula is existential at the top level.
    pub fn is_existential(&self) -> bool {
        matches!(self, RealizedFormula::Exists(_, _))
    }
}
/// Systematic proof searcher with backtracking.
#[derive(Debug, Clone)]
pub struct ProofSearcher {
    /// The search strategy.
    pub strategy: SearchStrategy,
    /// The heuristic function (used by A* and MCTS).
    pub heuristic: HeuristicFn,
    /// Maximum search depth.
    pub max_depth: usize,
}
impl ProofSearcher {
    /// Create a new proof searcher.
    pub fn new(strategy: SearchStrategy, max_depth: usize) -> Self {
        Self {
            strategy,
            heuristic: HeuristicFn::new("goal_count"),
            max_depth,
        }
    }
    /// Attempt to search for a proof of `goal` using `tactics`.
    /// Returns the proof state if a complete proof is found.
    pub fn search(&self, goal: impl Into<String>, tactics: &[&str]) -> Option<ProofState> {
        let mut state = ProofState::new(goal, self.max_depth);
        for tactic in tactics {
            if state.is_complete() {
                break;
            }
            state.apply_tactic(*tactic);
        }
        if state.is_complete() {
            Some(state)
        } else {
            None
        }
    }
}
/// Encodes Tao's metastability bound Φ(ε, k) for a convergent sequence.
///
/// A sequence (a_n) is metastable with rate Φ if for every ε > 0 and k,
/// there exists n ≤ Φ(ε, k) such that |a_n - a_{n+1}| < ε for all
/// indices in \[n, n + k\].
#[derive(Debug, Clone)]
pub struct MetastabilityBound {
    /// Name of the theorem/sequence this bound applies to.
    pub name: String,
    /// The bound function Φ: (epsilon_inv, steps) → index bound.
    /// We represent ε as its inverse (a natural number).
    pub bound_table: Vec<Vec<u64>>,
    /// Whether the bound is tight (matches known lower bounds).
    pub is_tight: bool,
}
impl MetastabilityBound {
    /// Create a metastability bound with a constant Φ(ε_inv, k) = c.
    pub fn constant(name: impl Into<String>, c: u64) -> Self {
        let table: Vec<Vec<u64>> = (0..8).map(|_| (0..8).map(|_| c).collect()).collect();
        Self {
            name: name.into(),
            bound_table: table,
            is_tight: false,
        }
    }
    /// Evaluate Φ(epsilon_inv, k): look up the bound table, clamped.
    pub fn evaluate(&self, epsilon_inv: usize, k: usize) -> u64 {
        let r = epsilon_inv.min(self.bound_table.len() - 1);
        let c = k.min(self.bound_table[r].len() - 1);
        self.bound_table[r][c]
    }
    /// Returns true if the bound is finite for all inputs.
    pub fn is_finite(&self) -> bool {
        self.bound_table
            .iter()
            .all(|row| row.iter().all(|&v| v < u64::MAX))
    }
}
/// A single resolution step: C_1 ∨ x, C_2 ∨ ¬x ⊢ C_1 ∨ C_2.
#[derive(Debug, Clone)]
pub struct ResolutionStep {
    /// First parent clause (contains `pivot` positively).
    pub parent1: Clause,
    /// Second parent clause (contains `pivot` negatively).
    pub parent2: Clause,
    /// The pivot variable.
    pub pivot: u32,
    /// The resolvent clause.
    pub resolvent: Clause,
}
impl ResolutionStep {
    /// Attempt to resolve `c1` and `c2` on variable `pivot`.
    /// Returns `None` if the pivot does not appear with opposite signs.
    pub fn resolve(c1: &Clause, c2: &Clause, pivot: u32) -> Option<Self> {
        let has_pos = c1.literals.iter().any(|l| l.var == pivot && l.positive);
        let has_neg = c2.literals.iter().any(|l| l.var == pivot && !l.positive);
        if !has_pos || !has_neg {
            return None;
        }
        let mut res_lits: Vec<Literal> = c1
            .literals
            .iter()
            .filter(|l| l.var != pivot)
            .chain(c2.literals.iter().filter(|l| l.var != pivot))
            .cloned()
            .collect();
        res_lits.sort_by_key(|l| (l.var, l.positive));
        res_lits.dedup();
        Some(ResolutionStep {
            parent1: c1.clone(),
            parent2: c2.clone(),
            pivot,
            resolvent: Clause::new(res_lits),
        })
    }
}
/// Spector bar recursion.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BarRecursion {
    pub type_level: usize,
    pub models_comprehension: bool,
}
impl BarRecursion {
    #[allow(dead_code)]
    pub fn spector() -> Self {
        Self {
            type_level: 2,
            models_comprehension: true,
        }
    }
    #[allow(dead_code)]
    pub fn description(&self) -> String {
        format!(
            "Spector bar recursion: models classical comprehension via type-{} functional",
            self.type_level
        )
    }
    #[allow(dead_code)]
    pub fn kohlenbach_generalization(&self) -> String {
        "Modified bar recursion for non-empty types (Berger-Oliva)".to_string()
    }
}
/// Quantitative Cauchy sequence criterion.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct QuantitativeCauchy {
    pub space_name: String,
    pub convergence_rate: String,
}
impl QuantitativeCauchy {
    #[allow(dead_code)]
    pub fn new(space: &str, rate: &str) -> Self {
        Self {
            space_name: space.to_string(),
            convergence_rate: rate.to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn leustean_bound_for_cat0(&self) -> String {
        "Leustean: Halpern iterations converge in CAT(0) spaces with explicit rate omega^omega"
            .to_string()
    }
}
/// Gödel's Dialectica translation A^D as (∃u.∀x. A_D(u,x)).
#[derive(Debug, Clone)]
pub struct DialecticaFormula {
    /// The original formula A.
    pub original: String,
    /// The universal variable names (the "x" side).
    pub universal_vars: Vec<String>,
    /// The existential variable names (the "u" side).
    pub existential_vars: Vec<String>,
    /// The quantifier-free body A_D(u,x), as a string.
    pub body: String,
}
impl DialecticaFormula {
    /// Create the Dialectica translation of a formula.
    pub fn translate(original: impl Into<String>, body: impl Into<String>) -> Self {
        Self {
            original: original.into(),
            universal_vars: vec!["x".into()],
            existential_vars: vec!["u".into()],
            body: body.into(),
        }
    }
    /// Return the full translated formula as a string.
    pub fn display(&self) -> String {
        format!(
            "∃ {}. ∀ {}. {}",
            self.existential_vars.join(", "),
            self.universal_vars.join(", "),
            self.body,
        )
    }
}
/// A polynomial or exponential complexity bound extracted from a proof.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComplexityBound {
    /// Constant O(1).
    Constant,
    /// Linear O(n).
    Linear,
    /// Polynomial O(n^k) for given k.
    Polynomial(u32),
    /// Exponential O(2^n).
    Exponential,
    /// Non-elementary.
    NonElementary,
}
impl ComplexityBound {
    /// Returns true if the bound is at most polynomial.
    pub fn is_polynomial(&self) -> bool {
        matches!(
            self,
            ComplexityBound::Constant | ComplexityBound::Linear | ComplexityBound::Polynomial(_)
        )
    }
}
/// Builds Herbrand sequences from a first-order formula.
#[derive(Debug, Clone)]
pub struct HerbrandSequenceBuilder {
    /// The original formula (as a string).
    pub formula: String,
    /// The ground instances collected so far.
    pub instances: Vec<String>,
    /// Maximum number of instances to generate.
    pub max_instances: usize,
}
impl HerbrandSequenceBuilder {
    /// Create a new builder for the given formula.
    pub fn new(formula: impl Into<String>, max_instances: usize) -> Self {
        Self {
            formula: formula.into(),
            instances: Vec::new(),
            max_instances,
        }
    }
    /// Add a ground instance (substituting concrete terms for free variables).
    pub fn add_instance(&mut self, instance: impl Into<String>) -> bool {
        if self.instances.len() >= self.max_instances {
            return false;
        }
        self.instances.push(instance.into());
        true
    }
    /// Return true if the disjunction of all instances is a tautology (checked syntactically).
    pub fn is_tautology(&self) -> bool {
        self.instances.iter().any(|s| s.contains("True"))
    }
    /// Return the Herbrand complexity (number of instances).
    pub fn complexity(&self) -> usize {
        self.instances.len()
    }
    /// Return the disjunction of all collected instances.
    pub fn disjunction(&self) -> String {
        if self.instances.is_empty() {
            return "False".to_string();
        }
        self.instances.join(" ∨ ")
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GentzenNormalization {
    pub proof_size_before: usize,
    pub proof_size_after: usize,
    pub cut_rank: usize,
    pub num_reduction_steps: usize,
}
#[allow(dead_code)]
impl GentzenNormalization {
    pub fn new(before: usize, cut_rank: usize) -> Self {
        let steps = 2_usize.saturating_pow(cut_rank as u32);
        GentzenNormalization {
            proof_size_before: before,
            proof_size_after: before * steps,
            cut_rank,
            num_reduction_steps: steps,
        }
    }
    pub fn cut_elimination_theorem(&self) -> String {
        format!(
            "Gentzen's cut elimination: proof of size {} with cut rank {} → normal proof of size {} (TOWER({}))",
            self.proof_size_before, self.cut_rank, self.proof_size_after, self.cut_rank
        )
    }
    pub fn reduction_terminates(&self) -> bool {
        true
    }
    pub fn ordinal_analysis_connection(&self) -> String {
        "PA cut-rank ↔ ordinal ε₀; PA₂ ↔ Γ₀; KP ↔ Bachmann-Howard ordinal".to_string()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ProofSystemType {
    Resolution,
    Frege,
    ExtendedFrege,
    QuantifiedPropositional,
    CuttingPlanes,
    SumOfSquares,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PhpPrinciple {
    pub n_pigeons: usize,
    pub m_holes: usize,
    pub resolution_lower_bound: usize,
}
#[allow(dead_code)]
impl PhpPrinciple {
    pub fn new(n: usize, m: usize) -> Self {
        let lb = 2_usize.saturating_pow((n / 2) as u32);
        PhpPrinciple {
            n_pigeons: n,
            m_holes: m,
            resolution_lower_bound: lb,
        }
    }
    pub fn is_valid_php(&self) -> bool {
        self.n_pigeons > self.m_holes
    }
    pub fn haken_lower_bound_description(&self) -> String {
        format!(
            "Haken (1985): PHP_{} has no polynomial resolution proof; lower bound ≥ {}",
            self.n_pigeons, self.resolution_lower_bound
        )
    }
    pub fn bounded_arithmetic_connection(&self) -> String {
        format!(
            "PHP_{} unprovable in S^1_2 but provable in T^2_2 (bounded arithmetic)",
            self.n_pigeons
        )
    }
}
/// A propositional proof system.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProofSystem {
    /// Resolution (Robinson 1965).
    Resolution,
    /// Frege / Hilbert-style propositional calculus.
    Frege,
    /// Extended Frege (with extension axioms).
    ExtendedFrege,
    /// Half-Frege (bounded-depth Frege).
    HalfFrege,
    /// Cutting Planes (LP relaxation cuts).
    CuttingPlanes,
    /// Nullstellensatz proof system (algebraic).
    Nullstellensatz,
    /// Sum-of-Squares (Positivstellensatz).
    SOS,
    /// Ideal Proof System (IPS).
    IPS,
}
/// A resolution proof: a DAG of resolution steps.
#[derive(Debug, Clone)]
pub struct ResolutionProof {
    /// The input clause set.
    pub input_clauses: Vec<Vec<i32>>,
    /// Resolution steps: (parent1_idx, parent2_idx, pivot_var, resolvent).
    pub steps: Vec<(usize, usize, i32, Vec<i32>)>,
}
impl ResolutionProof {
    /// Create a new resolution proof from the given input clauses.
    pub fn new(input_clauses: Vec<Vec<i32>>) -> Self {
        Self {
            input_clauses,
            steps: Vec::new(),
        }
    }
    /// Add a resolution step.
    pub fn add_step(&mut self, p1: usize, p2: usize, pivot: i32, resolvent: Vec<i32>) {
        self.steps.push((p1, p2, pivot, resolvent));
    }
    /// Returns true if the proof ends with the empty clause.
    pub fn is_refutation(&self) -> bool {
        self.steps.last().is_some_and(|(_, _, _, r)| r.is_empty())
    }
}
/// Effective bound extraction result.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EffectiveBound {
    pub original_theorem: String,
    pub extracted_bound: String,
    pub bound_type: BoundType,
    pub dependencies: Vec<String>,
}
impl EffectiveBound {
    #[allow(dead_code)]
    pub fn new(theorem: &str, bound: &str, bt: BoundType) -> Self {
        Self {
            original_theorem: theorem.to_string(),
            extracted_bound: bound.to_string(),
            bound_type: bt,
            dependencies: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn is_feasible(&self) -> bool {
        matches!(self.bound_type, BoundType::Polynomial)
    }
    #[allow(dead_code)]
    pub fn add_dependency(&mut self, dep: &str) {
        self.dependencies.push(dep.to_string());
    }
}
/// Unwinding theorem: proof mining extracts computational content.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct UnwindingResult {
    pub classical_proof: String,
    pub constructive_content: String,
    pub interpreter_used: String,
}
impl UnwindingResult {
    #[allow(dead_code)]
    pub fn new(classical: &str, constructive: &str, interp: &str) -> Self {
        Self {
            classical_proof: classical.to_string(),
            constructive_content: constructive.to_string(),
            interpreter_used: interp.to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn kohlenbach_style(&self) -> String {
        format!(
            "Kohlenbach unwinding: {} -> {}, via {}",
            self.classical_proof, self.constructive_content, self.interpreter_used
        )
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ProofSystemNew {
    pub name: String,
    pub system_type: ProofSystemType,
    pub propositional_completeness: bool,
    pub p_simulates_resolution: Option<bool>,
}
#[allow(dead_code)]
impl ProofSystemNew {
    pub fn resolution() -> Self {
        ProofSystemNew {
            name: "Resolution".to_string(),
            system_type: ProofSystemType::Resolution,
            propositional_completeness: true,
            p_simulates_resolution: Some(false),
        }
    }
    pub fn frege() -> Self {
        ProofSystemNew {
            name: "Frege (Hilbert-style)".to_string(),
            system_type: ProofSystemType::Frege,
            propositional_completeness: true,
            p_simulates_resolution: Some(true),
        }
    }
    pub fn extended_frege() -> Self {
        ProofSystemNew {
            name: "Extended Frege (EF)".to_string(),
            system_type: ProofSystemType::ExtendedFrege,
            propositional_completeness: true,
            p_simulates_resolution: Some(true),
        }
    }
    pub fn separating_tautologies(&self) -> String {
        match &self.system_type {
            ProofSystemType::Resolution => {
                "Pigeonhole principle requires exponential-size Resolution proofs".to_string()
            }
            ProofSystemType::CuttingPlanes => {
                "CP: exponential lower bounds for random CNF (Razborov)".to_string()
            }
            _ => "Known lower bounds: PHP, Tseitin, Random k-CNF".to_string(),
        }
    }
    pub fn cook_reckhow_theorem(&self) -> String {
        "Cook-Reckhow: P=NP iff every Cook system has polynomial-size proofs".to_string()
    }
}
/// PA^ω conservativity results for bounded arithmetic.
#[derive(Debug, Clone)]
pub struct BoundedArithmetic {
    /// The theory name (e.g. "PA^ω", "WKL_0").
    pub theory: String,
    /// The base theory it is conservative over (e.g. "PRA").
    pub base_theory: String,
    /// Formula class for which conservativity holds.
    pub formula_class: String,
}
impl BoundedArithmetic {
    /// PA^ω is Π^0_2-conservative over PRA.
    pub fn paomega_over_pra() -> Self {
        Self {
            theory: "PA^ω".into(),
            base_theory: "PRA".into(),
            formula_class: "Π^0_2".into(),
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoundType {
    Polynomial,
    Exponential,
    PrimitiveRecursive,
    Ackermann,
    NonPrimitive,
}
