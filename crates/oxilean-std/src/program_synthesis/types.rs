//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};
use std::collections::HashMap;

use super::functions::Oracle;
use super::functions::*;

/// A refinement type used in liquid type synthesis.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RefinementType {
    /// The base type (e.g., "Int", "Bool").
    pub base: String,
    /// The refinement predicate as a string (e.g., "v > 0").
    pub predicate: String,
}
#[allow(dead_code)]
impl RefinementType {
    /// Construct a new refinement type.
    ///
    /// ```
    /// use oxilean_std::program_synthesis::RefinementType;
    /// let rt = RefinementType::new("Int", "v >= 0");
    /// assert_eq!(rt.base, "Int");
    /// ```
    pub fn new(base: impl Into<String>, predicate: impl Into<String>) -> Self {
        Self {
            base: base.into(),
            predicate: predicate.into(),
        }
    }
    /// Check whether the predicate is trivially true (empty string).
    pub fn is_trivial(&self) -> bool {
        self.predicate.trim().is_empty()
    }
    /// Return a strengthened type by conjoining an extra predicate.
    pub fn strengthen(&self, extra: impl Into<String>) -> Self {
        let new_pred = format!("({}) && ({})", self.predicate, extra.into());
        Self {
            base: self.base.clone(),
            predicate: new_pred,
        }
    }
    /// Return a weakened type by disjoining an extra predicate.
    pub fn weaken(&self, extra: impl Into<String>) -> Self {
        let new_pred = format!("({}) || ({})", self.predicate, extra.into());
        Self {
            base: self.base.clone(),
            predicate: new_pred,
        }
    }
}
/// A constraint-based synthesis engine that encodes the problem as SMT.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ConstraintSynthEngine {
    /// Hard constraints that the synthesised program must satisfy.
    pub hard_constraints: Vec<String>,
    /// Soft constraints (ranked by weight).
    pub soft_constraints: Vec<(String, u32)>,
    /// The program template (sketch).
    pub template: Option<String>,
}
#[allow(dead_code)]
impl ConstraintSynthEngine {
    /// Create a new empty constraint synthesis engine.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a hard constraint.
    pub fn add_hard(&mut self, constraint: impl Into<String>) {
        self.hard_constraints.push(constraint.into());
    }
    /// Add a soft constraint with a weight.
    pub fn add_soft(&mut self, constraint: impl Into<String>, weight: u32) {
        self.soft_constraints.push((constraint.into(), weight));
    }
    /// Set the program template (sketch with holes).
    pub fn set_template(&mut self, template: impl Into<String>) {
        self.template = Some(template.into());
    }
    /// Count total constraints.
    pub fn num_constraints(&self) -> usize {
        self.hard_constraints.len() + self.soft_constraints.len()
    }
    /// Encode as a pseudo-Boolean optimisation problem (placeholder).
    pub fn encode_pbo(&self) -> String {
        let hard: Vec<String> = self
            .hard_constraints
            .iter()
            .map(|c| format!("HARD: {}", c))
            .collect();
        let soft: Vec<String> = self
            .soft_constraints
            .iter()
            .map(|(c, w)| format!("SOFT[{}]: {}", w, c))
            .collect();
        [hard, soft].concat().join("\n")
    }
    /// Attempt to synthesise a program satisfying all hard constraints.
    /// Returns the filled template or `None` if unsatisfiable.
    pub fn solve(&self) -> Option<String> {
        if self.hard_constraints.is_empty() {
            self.template.clone()
        } else {
            None
        }
    }
}
/// A logical specification for a synthesis problem.
///
/// Specifications range from purely logical (pre/post conditions) to
/// example-based (input/output pairs) to grammar-constrained.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Spec {
    /// A logical predicate `P(x, y)` relating inputs `x` to output `y`.
    Logic(String),
    /// A finite set of concrete input/output examples.
    Examples(Vec<(Vec<String>, String)>),
    /// A context-free grammar restricting the syntactic form of solutions.
    Grammar(CFG),
    /// Conjunction of multiple specifications.
    Conjunction(Box<Spec>, Box<Spec>),
    /// Disjunction: any one of the specs suffices.
    Disjunction(Box<Spec>, Box<Spec>),
}
impl Spec {
    /// Build a logic spec from a predicate string.
    ///
    /// ```
    /// use oxilean_std::program_synthesis::Spec;
    /// let s = Spec::logic("output = input * 2");
    /// assert!(matches!(s, Spec::Logic(_)));
    /// ```
    pub fn logic(pred: impl Into<String>) -> Self {
        Spec::Logic(pred.into())
    }
    /// Build a spec from a list of (inputs, output) example pairs.
    ///
    /// ```
    /// use oxilean_std::program_synthesis::Spec;
    /// let s = Spec::from_examples(vec![(vec!["0".into()], "0".into()),
    ///                                   (vec!["1".into()], "2".into())]);
    /// assert!(matches!(s, Spec::Examples(_)));
    /// ```
    pub fn from_examples(ex: Vec<(Vec<String>, String)>) -> Self {
        Spec::Examples(ex)
    }
    /// Return the number of constraints (examples or logical clauses) in this spec.
    pub fn constraint_count(&self) -> usize {
        match self {
            Spec::Logic(_) => 1,
            Spec::Examples(ex) => ex.len(),
            Spec::Grammar(_) => 1,
            Spec::Conjunction(a, b) => a.constraint_count() + b.constraint_count(),
            Spec::Disjunction(a, b) => a.constraint_count().max(b.constraint_count()),
        }
    }
}
/// The CEGIS synthesis loop state.
#[derive(Debug, Clone)]
pub struct CegisState {
    /// The specification to satisfy.
    pub spec: Spec,
    /// Counterexamples accumulated across iterations.
    pub counterexamples: Vec<Vec<String>>,
    /// Current candidate (if any).
    pub candidate: Option<Candidate>,
    /// Number of CEGIS iterations performed.
    pub iterations: usize,
    /// Maximum allowed iterations before giving up.
    pub max_iterations: usize,
}
impl CegisState {
    /// Initialise a CEGIS loop for the given spec.
    ///
    /// ```
    /// use oxilean_std::program_synthesis::{CegisState, Spec};
    /// let state = CegisState::new(Spec::logic("y = x + 1"), 100);
    /// assert_eq!(state.iterations, 0);
    /// ```
    pub fn new(spec: Spec, max_iterations: usize) -> Self {
        Self {
            spec,
            counterexamples: Vec::new(),
            candidate: None,
            iterations: 0,
            max_iterations,
        }
    }
    /// Propose a candidate (synthesiser step).
    pub fn propose(&mut self, candidate: Candidate) {
        self.candidate = Some(candidate);
        self.iterations += 1;
    }
    /// Record a counterexample returned by the verifier.
    pub fn add_counterexample(&mut self, ce: Vec<String>) {
        self.counterexamples.push(ce);
        self.candidate = None;
    }
    /// Check whether the synthesis loop has terminated successfully.
    pub fn is_solved(&self) -> bool {
        self.candidate.is_some()
    }
    /// Check whether the loop has exceeded its iteration budget.
    pub fn is_exhausted(&self) -> bool {
        self.iterations >= self.max_iterations
    }
}
/// A Manna–Waldinger synthesis goal: (precondition, postcondition, output variable).
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MWSynthGoal {
    /// The precondition predicate.
    pub pre: String,
    /// The postcondition predicate relating inputs and output.
    pub post: String,
    /// The output variable name.
    pub output_var: String,
    /// Sub-goals generated by applying a rule.
    pub subgoals: Vec<MWSynthGoal>,
}
#[allow(dead_code)]
impl MWSynthGoal {
    /// Create a leaf synthesis goal.
    ///
    /// ```
    /// use oxilean_std::program_synthesis::MWSynthGoal;
    /// let g = MWSynthGoal::leaf("true", "y = x + 1", "y");
    /// assert!(g.subgoals.is_empty());
    /// ```
    pub fn leaf(
        pre: impl Into<String>,
        post: impl Into<String>,
        output_var: impl Into<String>,
    ) -> Self {
        Self {
            pre: pre.into(),
            post: post.into(),
            output_var: output_var.into(),
            subgoals: Vec::new(),
        }
    }
    /// Create a goal with sub-goals (internal node).
    pub fn with_subgoals(
        pre: impl Into<String>,
        post: impl Into<String>,
        output_var: impl Into<String>,
        subgoals: Vec<MWSynthGoal>,
    ) -> Self {
        Self {
            pre: pre.into(),
            post: post.into(),
            output_var: output_var.into(),
            subgoals,
        }
    }
    /// Depth of the goal tree.
    pub fn depth(&self) -> usize {
        if self.subgoals.is_empty() {
            0
        } else {
            1 + self.subgoals.iter().map(|sg| sg.depth()).max().unwrap_or(0)
        }
    }
    /// Check whether this is an atomic (leaf) goal.
    pub fn is_leaf(&self) -> bool {
        self.subgoals.is_empty()
    }
}
/// An input/output example for programming by example.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IOExample {
    /// Input values (as strings for generality).
    pub inputs: Vec<String>,
    /// Expected output.
    pub output: String,
}
impl IOExample {
    /// Create a new I/O example.
    pub fn new(inputs: Vec<String>, output: impl Into<String>) -> Self {
        Self {
            inputs,
            output: output.into(),
        }
    }
}
/// An enumerative SyGuS solver that exhaustively searches the grammar.
#[derive(Debug, Clone)]
pub struct EnumerativeSolver {
    /// Depth limit for grammar expansion.
    pub depth_limit: usize,
    /// Number of programs enumerated so far.
    pub enumerated: usize,
}
impl EnumerativeSolver {
    /// Create a new enumerative solver with the given depth limit.
    pub fn new(depth_limit: usize) -> Self {
        Self {
            depth_limit,
            enumerated: 0,
        }
    }
    /// Enumerate all programs up to `depth` from `start` symbol.
    pub fn enumerate(&mut self, grammar: &CFG, depth: usize) -> Vec<String> {
        if depth == 0 {
            let terms: Vec<String> = grammar.terminals.clone();
            self.enumerated += terms.len();
            terms
        } else {
            let mut result = Vec::new();
            for prod in &grammar.productions.clone() {
                if prod.rhs.iter().all(|s| grammar.terminals.contains(s)) {
                    let term = prod.rhs.join(" ");
                    result.push(term);
                    self.enumerated += 1;
                }
            }
            result
        }
    }
    /// Attempt to solve a SyGuS problem (simplified placeholder).
    pub fn solve(&mut self, _problem: &SyGuSProblem) -> SyGuSResult {
        SyGuSResult::Timeout
    }
}
/// A hole in a program sketch, parameterised by expected type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hole {
    /// Unique identifier for this hole.
    pub id: usize,
    /// Expected type of the expression filling this hole.
    pub expected_type: String,
    /// Optional constraint on the filler.
    pub constraint: Option<String>,
}
impl Hole {
    /// Create a new unconstrained hole.
    pub fn new(id: usize, expected_type: impl Into<String>) -> Self {
        Self {
            id,
            expected_type: expected_type.into(),
            constraint: None,
        }
    }
    /// Create a constrained hole.
    pub fn constrained(
        id: usize,
        expected_type: impl Into<String>,
        constraint: impl Into<String>,
    ) -> Self {
        Self {
            id,
            expected_type: expected_type.into(),
            constraint: Some(constraint.into()),
        }
    }
}
/// A finite-table oracle backed by a lookup table.
#[derive(Debug, Clone, Default)]
pub struct TableOracle {
    /// The lookup table.
    pub table: HashMap<Vec<String>, String>,
}
impl TableOracle {
    /// Build a new table oracle.
    pub fn new() -> Self {
        Self::default()
    }
    /// Insert an entry.
    pub fn insert(&mut self, input: Vec<String>, output: impl Into<String>) {
        self.table.insert(input, output.into());
    }
}
/// A program sketch: a program template with "holes" to be filled.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ProgramSketch {
    /// The template string with {hole_N} placeholders.
    pub template: String,
    /// The number of holes.
    pub num_holes: usize,
    /// The type constraints for each hole (simplified).
    pub hole_types: Vec<String>,
}
#[allow(dead_code)]
impl ProgramSketch {
    /// Create a new program sketch.
    pub fn new(template: &str, num_holes: usize) -> Self {
        ProgramSketch {
            template: template.to_string(),
            num_holes,
            hole_types: vec!["Any".to_string(); num_holes],
        }
    }
    /// Set the type of hole i.
    pub fn set_hole_type(&mut self, i: usize, ty: &str) {
        if i < self.hole_types.len() {
            self.hole_types[i] = ty.to_string();
        }
    }
    /// Fill holes with given completions to get a concrete program.
    pub fn fill(&self, completions: &[&str]) -> String {
        let mut result = self.template.clone();
        for (i, completion) in completions.iter().enumerate() {
            result = result.replace(&format!("{{hole_{}}}", i), completion);
        }
        result
    }
    /// Check if all holes are filled in a given program string.
    pub fn is_complete(&self, program: &str) -> bool {
        !(0..self.num_holes).any(|i| program.contains(&format!("{{hole_{}}}", i)))
    }
    /// Generate all sketches of programs up to depth `d` with `k` holes.
    /// Returns simplified count: (operations^holes).
    pub fn sketch_space_size(&self, num_operations: u64) -> u64 {
        num_operations.saturating_pow(self.num_holes as u32)
    }
}
/// A version space: the set of programs consistent with all examples seen.
#[derive(Debug, Clone)]
pub struct VersionSpace {
    /// Candidate programs still consistent with all examples.
    pub candidates: Vec<String>,
    /// Examples used to prune the version space.
    pub examples: Vec<IOExample>,
}
impl VersionSpace {
    /// Create a version space from an initial candidate set.
    pub fn new(candidates: Vec<String>) -> Self {
        Self {
            candidates,
            examples: Vec::new(),
        }
    }
    /// Refine the version space with a new example.
    pub fn refine(&mut self, example: IOExample) {
        self.examples.push(example);
    }
    /// Return the unique candidate if the version space is a singleton.
    pub fn unique_solution(&self) -> Option<&str> {
        if self.candidates.len() == 1 {
            Some(&self.candidates[0])
        } else {
            None
        }
    }
    /// Number of consistent candidates.
    pub fn size(&self) -> usize {
        self.candidates.len()
    }
}
/// A sketch-based synthesiser that fills holes using constraint solving.
#[derive(Debug, Clone)]
pub struct SketchSolver {
    /// Maximum number of candidate values tried per hole.
    pub candidates_per_hole: usize,
}
impl SketchSolver {
    /// Create a new sketch solver.
    pub fn new(candidates_per_hole: usize) -> Self {
        Self {
            candidates_per_hole,
        }
    }
    /// Attempt to complete a sketch given a specification.
    ///
    /// Returns the completed program or `None` if no solution is found.
    pub fn complete(&self, sketch: &Sketch, _spec: &Spec) -> Option<String> {
        if sketch.is_complete() {
            return Some(sketch.source.clone());
        }
        None
    }
}
/// An inductive loop invariant synthesiser using Houdini-style fixpoint.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct HoudiniInvariantSynth {
    /// Candidate invariant predicates to try.
    pub candidates: Vec<String>,
    /// Iterations of the Houdini fixpoint computation.
    pub iterations: usize,
}
#[allow(dead_code)]
impl HoudiniInvariantSynth {
    /// Create a Houdini synthesiser with an initial set of candidates.
    ///
    /// ```
    /// use oxilean_std::program_synthesis::HoudiniInvariantSynth;
    /// let h = HoudiniInvariantSynth::new(vec!["x >= 0".into(), "x <= n".into()]);
    /// assert_eq!(h.candidates.len(), 2);
    /// ```
    pub fn new(candidates: Vec<String>) -> Self {
        Self {
            candidates,
            iterations: 0,
        }
    }
    /// Add a candidate invariant predicate.
    pub fn add_candidate(&mut self, pred: impl Into<String>) {
        self.candidates.push(pred.into());
    }
    /// Simulate one Houdini iteration: remove candidates falsified by pre-image.
    ///
    /// In a real implementation, each candidate would be checked against the
    /// loop body; here we just count the iteration.
    pub fn step(&mut self) -> usize {
        self.iterations += 1;
        self.candidates.len()
    }
    /// Return the current invariant candidate set.
    pub fn current_invariants(&self) -> &[String] {
        &self.candidates
    }
    /// Check whether a fixpoint has been reached (placeholder heuristic).
    pub fn is_fixpoint(&self) -> bool {
        self.iterations > 0
    }
}
/// A component library used in component-based synthesis.
#[derive(Debug, Clone, Default)]
pub struct ComponentLibrary {
    /// All available components.
    pub components: Vec<Component>,
}
impl ComponentLibrary {
    /// Create an empty library.
    pub fn new() -> Self {
        Self::default()
    }
    /// Register a component.
    pub fn register(&mut self, comp: Component) {
        self.components.push(comp);
    }
    /// Find components whose output sort matches `target`.
    pub fn candidates_for(&self, target: &str) -> Vec<&Component> {
        self.components
            .iter()
            .filter(|c| c.output_matches(target))
            .collect()
    }
}
/// The result of a CEGIS verification query.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerifierResult {
    /// The candidate program satisfies the specification for all inputs.
    Correct,
    /// A counterexample was found: the given input violates the spec.
    Counterexample(Vec<String>),
    /// Verification was inconclusive (timeout or undecidable fragment).
    Unknown,
}
/// A deductive synthesis derivation tree node.
#[derive(Debug, Clone)]
pub struct DerivationNode {
    /// The synthesis goal at this node: (precondition, postcondition).
    pub goal: (String, String),
    /// The rule applied to discharge this goal.
    pub rule: Option<DeductiveRule>,
    /// Child derivations.
    pub children: Vec<DerivationNode>,
    /// The synthesised program fragment (leaf nodes only).
    pub program: Option<String>,
}
impl DerivationNode {
    /// Create a leaf node with a concrete program.
    pub fn leaf(
        pre: impl Into<String>,
        post: impl Into<String>,
        program: impl Into<String>,
    ) -> Self {
        Self {
            goal: (pre.into(), post.into()),
            rule: None,
            children: Vec::new(),
            program: Some(program.into()),
        }
    }
    /// Create an internal node with a rule and sub-goals.
    pub fn internal(
        pre: impl Into<String>,
        post: impl Into<String>,
        rule: DeductiveRule,
        children: Vec<DerivationNode>,
    ) -> Self {
        Self {
            goal: (pre.into(), post.into()),
            rule: Some(rule),
            children,
            program: None,
        }
    }
    /// Extract the full synthesised program by recursively assembling sub-trees.
    pub fn extract_program(&self) -> String {
        if let Some(ref p) = self.program {
            return p.clone();
        }
        if let Some(ref rule) = self.rule {
            match rule.name.as_str() {
                "seq" => {
                    let parts: Vec<String> =
                        self.children.iter().map(|c| c.extract_program()).collect();
                    parts.join("; ")
                }
                "if-then-else" => {
                    let cond = self
                        .children
                        .first()
                        .map(|c| c.extract_program())
                        .unwrap_or_default();
                    let then_b = self
                        .children
                        .get(1)
                        .map(|c| c.extract_program())
                        .unwrap_or_default();
                    let else_b = self
                        .children
                        .get(2)
                        .map(|c| c.extract_program())
                        .unwrap_or_default();
                    format!("if {} then {} else {}", cond, then_b, else_b)
                }
                _ => self
                    .children
                    .iter()
                    .map(|c| c.extract_program())
                    .collect::<Vec<_>>()
                    .join(" "),
            }
        } else {
            String::new()
        }
    }
    /// Depth of the derivation tree.
    pub fn depth(&self) -> usize {
        if self.children.is_empty() {
            0
        } else {
            1 + self.children.iter().map(|c| c.depth()).max().unwrap_or(0)
        }
    }
}
/// A program sketch: a partial program with holes to fill.
#[derive(Debug, Clone)]
pub struct Sketch {
    /// The sketch source text (holes written as `??<id>`).
    pub source: String,
    /// The holes present in this sketch.
    pub holes: Vec<Hole>,
}
impl Sketch {
    /// Create a new sketch from source text and holes.
    pub fn new(source: impl Into<String>, holes: Vec<Hole>) -> Self {
        Self {
            source: source.into(),
            holes,
        }
    }
    /// Count the number of holes.
    pub fn num_holes(&self) -> usize {
        self.holes.len()
    }
    /// Fill a hole with the given expression, returning a new sketch.
    pub fn fill_hole(&self, hole_id: usize, expr: &str) -> Sketch {
        let new_source = self.source.replace(&format!("??{}", hole_id), expr);
        let new_holes = self
            .holes
            .iter()
            .filter(|h| h.id != hole_id)
            .cloned()
            .collect();
        Sketch::new(new_source, new_holes)
    }
    /// Check if the sketch is complete (no holes remaining).
    pub fn is_complete(&self) -> bool {
        self.holes.is_empty()
    }
}
/// FOIL (First Order Inductive Learner) algorithm scaffolding.
#[allow(dead_code)]
pub struct FoilLearner {
    /// Maximum clause depth.
    pub max_depth: usize,
    /// Minimum information gain threshold.
    pub min_gain: f64,
}
#[allow(dead_code)]
impl FoilLearner {
    /// Create a new FOIL learner.
    pub fn new(max_depth: usize, min_gain: f64) -> Self {
        FoilLearner {
            max_depth,
            min_gain,
        }
    }
    /// FOIL information gain for a literal L.
    /// Gain(L) = t * (log2(p1/(p1+n1)) - log2(p0/(p0+n0)))
    /// where p0/n0 are positive/negative before adding L, p1/n1 after.
    pub fn foil_gain(&self, t: f64, p0: f64, n0: f64, p1: f64, n1: f64) -> f64 {
        if p0 + n0 == 0.0 || p1 + n1 == 0.0 {
            return 0.0;
        }
        let before = (p0 / (p0 + n0)).log2();
        let after = (p1 / (p1 + n1)).log2();
        t * (after - before)
    }
    /// Greedy covering algorithm: add clauses until all positives are covered.
    /// Returns the number of clauses needed (simplified).
    pub fn covering_clauses_needed(&self, num_positives: usize) -> usize {
        let mut remaining = num_positives;
        let mut clauses = 0;
        while remaining > 0 {
            remaining /= 2;
            clauses += 1;
            if clauses > self.max_depth {
                break;
            }
        }
        clauses
    }
}
/// A recursive program structure for inductive synthesis.
#[derive(Debug, Clone)]
pub enum FuncProgram {
    /// A literal (constant) value.
    Lit(String),
    /// A variable reference.
    Var(String),
    /// Lambda abstraction.
    Lam(String, Box<FuncProgram>),
    /// Application.
    App(Box<FuncProgram>, Box<FuncProgram>),
    /// Pattern match on a list.
    ListCase {
        scrutinee: Box<FuncProgram>,
        nil_branch: Box<FuncProgram>,
        cons_head: String,
        cons_tail: String,
        cons_branch: Box<FuncProgram>,
    },
    /// Recursive call (for structurally recursive programs).
    Rec(Box<FuncProgram>),
}
impl FuncProgram {
    /// Pretty-print the program.
    pub fn pretty(&self) -> String {
        match self {
            FuncProgram::Lit(v) => v.clone(),
            FuncProgram::Var(v) => v.clone(),
            FuncProgram::Lam(x, body) => format!("fun {} -> {}", x, body.pretty()),
            FuncProgram::App(f, a) => format!("({} {})", f.pretty(), a.pretty()),
            FuncProgram::ListCase {
                scrutinee,
                nil_branch,
                cons_head,
                cons_tail,
                cons_branch,
            } => {
                format!(
                    "match {} with | [] -> {} | {}::{} -> {}",
                    scrutinee.pretty(),
                    nil_branch.pretty(),
                    cons_head,
                    cons_tail,
                    cons_branch.pretty()
                )
            }
            FuncProgram::Rec(p) => format!("rec({})", p.pretty()),
        }
    }
    /// Count AST nodes.
    pub fn size(&self) -> usize {
        match self {
            FuncProgram::Lit(_) | FuncProgram::Var(_) => 1,
            FuncProgram::Lam(_, b) => 1 + b.size(),
            FuncProgram::App(f, a) => 1 + f.size() + a.size(),
            FuncProgram::ListCase {
                scrutinee,
                nil_branch,
                cons_branch,
                ..
            } => 1 + scrutinee.size() + nil_branch.size() + cons_branch.size(),
            FuncProgram::Rec(p) => 1 + p.size(),
        }
    }
}
/// Oracle-guided synthesis loop.
#[derive(Debug, Clone)]
pub struct OracleSynthLoop {
    /// Queries made so far.
    pub queries: Vec<Vec<String>>,
    /// Answers received.
    pub answers: Vec<Option<String>>,
}
impl OracleSynthLoop {
    /// Create a new oracle-guided synthesis loop.
    pub fn new() -> Self {
        Self {
            queries: Vec::new(),
            answers: Vec::new(),
        }
    }
    /// Make an oracle query and record the answer.
    pub fn query(&mut self, oracle: &dyn Oracle, input: Vec<String>) -> Option<String> {
        let ans = oracle.query(&input);
        self.queries.push(input);
        self.answers.push(ans.clone());
        ans
    }
    /// Number of oracle queries made.
    pub fn num_queries(&self) -> usize {
        self.queries.len()
    }
}
/// Represents a learning problem in Inductive Logic Programming.
/// ILP learns logic programs (Horn clauses) from examples and background knowledge.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ILPProblem {
    /// Positive examples (facts that should be entailed).
    pub positive_examples: Vec<String>,
    /// Negative examples (facts that should NOT be entailed).
    pub negative_examples: Vec<String>,
    /// Background knowledge (predefined facts and rules).
    pub background: Vec<String>,
    /// Target predicate to learn.
    pub target: String,
}
#[allow(dead_code)]
impl ILPProblem {
    /// Create a new ILP problem.
    pub fn new(target: &str) -> Self {
        ILPProblem {
            positive_examples: vec![],
            negative_examples: vec![],
            background: vec![],
            target: target.to_string(),
        }
    }
    /// Add a positive example.
    pub fn add_positive(&mut self, example: &str) {
        self.positive_examples.push(example.to_string());
    }
    /// Add a negative example.
    pub fn add_negative(&mut self, example: &str) {
        self.negative_examples.push(example.to_string());
    }
    /// Add background knowledge.
    pub fn add_background(&mut self, fact: &str) {
        self.background.push(fact.to_string());
    }
    /// Count the total number of training examples.
    pub fn total_examples(&self) -> usize {
        self.positive_examples.len() + self.negative_examples.len()
    }
    /// Accuracy of a candidate hypothesis on this problem.
    /// The hypothesis is a list of clauses (strings).
    /// Simplified: count how many positive examples are in the hypothesis "model".
    pub fn accuracy(&self, hypothesis: &[String]) -> f64 {
        if self.total_examples() == 0 {
            return 1.0;
        }
        let _hyp_set: std::collections::HashSet<&str> =
            hypothesis.iter().map(|s| s.as_str()).collect();
        let covered = self
            .positive_examples
            .iter()
            .filter(|e| hypothesis.iter().any(|h| h.contains(e.as_str())))
            .count();
        covered as f64 / self.total_examples() as f64
    }
}
/// Oracle-guided inductive synthesis (OGIS): uses a teacher oracle.
#[allow(dead_code)]
pub struct OGISSynthesizer {
    /// Maximum number of oracle queries.
    pub max_queries: usize,
    /// Counter-examples collected from oracle.
    pub counter_examples: Vec<(Vec<i64>, i64)>,
}
#[allow(dead_code)]
impl OGISSynthesizer {
    /// Create a new OGIS synthesizer.
    pub fn new(max_queries: usize) -> Self {
        OGISSynthesizer {
            max_queries,
            counter_examples: vec![],
        }
    }
    /// Add a counter-example (input, expected output) from the oracle.
    pub fn add_counter_example(&mut self, input: Vec<i64>, output: i64) {
        self.counter_examples.push((input, output));
    }
    /// Check consistency: does the current hypothesis agree with all counter-examples?
    /// Simplified: just checks if the count of known CEs matches.
    pub fn is_consistent(&self, _hypothesis: &str) -> bool {
        self.counter_examples.len() <= self.max_queries
    }
    /// CEGIS (counterexample-guided inductive synthesis) iteration count.
    /// Returns the number of refinements done.
    pub fn cegis_iterations(&self) -> usize {
        self.counter_examples.len()
    }
    /// Convergence criterion: no more counter-examples means synthesis succeeded.
    pub fn has_converged(&self) -> bool {
        self.counter_examples.is_empty() || self.counter_examples.len() >= self.max_queries
    }
}
/// A production rule in a context-free grammar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Production {
    /// Left-hand non-terminal symbol.
    pub lhs: String,
    /// Right-hand side symbols (terminals and non-terminals).
    pub rhs: Vec<String>,
}
impl Production {
    /// Create a new production rule.
    pub fn new(lhs: impl Into<String>, rhs: Vec<String>) -> Self {
        Self {
            lhs: lhs.into(),
            rhs,
        }
    }
    /// Check whether this is an ε-production (empty right-hand side).
    pub fn is_epsilon(&self) -> bool {
        self.rhs.is_empty()
    }
}
/// A type-directed synthesis context: typed components available.
#[derive(Debug, Clone, Default)]
pub struct SynthContext {
    /// Named components with their types.
    pub components: HashMap<String, SynthType>,
}
impl SynthContext {
    /// Create an empty synthesis context.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a component with the given type.
    pub fn add(&mut self, name: impl Into<String>, ty: SynthType) {
        self.components.insert(name.into(), ty);
    }
    /// Find all components that match the goal type (syntactically).
    pub fn matching(&self, goal: &SynthType) -> Vec<(&str, &SynthType)> {
        self.components
            .iter()
            .filter(|(_, t)| *t == goal)
            .map(|(n, t)| (n.as_str(), t))
            .collect()
    }
}
/// A simple type language for type-directed synthesis.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SynthType {
    /// Base type (e.g., "Nat", "Bool").
    Base(String),
    /// Function type A → B.
    Arrow(Box<SynthType>, Box<SynthType>),
    /// Product type A × B.
    Product(Box<SynthType>, Box<SynthType>),
    /// Sum type A + B.
    Sum(Box<SynthType>, Box<SynthType>),
    /// Type variable.
    Var(String),
    /// Universally quantified type ∀ α. T.
    Forall(String, Box<SynthType>),
    /// The unit type.
    Unit,
}
impl SynthType {
    /// Construct an arrow type.
    pub fn arrow(a: SynthType, b: SynthType) -> Self {
        SynthType::Arrow(Box::new(a), Box::new(b))
    }
    /// Construct a product type.
    pub fn product(a: SynthType, b: SynthType) -> Self {
        SynthType::Product(Box::new(a), Box::new(b))
    }
    /// Construct a sum type.
    pub fn sum(a: SynthType, b: SynthType) -> Self {
        SynthType::Sum(Box::new(a), Box::new(b))
    }
    /// Check whether `name` appears free in this type.
    pub fn free_vars(&self) -> Vec<String> {
        match self {
            SynthType::Base(_) | SynthType::Unit => vec![],
            SynthType::Var(n) => vec![n.clone()],
            SynthType::Arrow(a, b) | SynthType::Product(a, b) | SynthType::Sum(a, b) => {
                let mut v = a.free_vars();
                v.extend(b.free_vars());
                v.sort();
                v.dedup();
                v
            }
            SynthType::Forall(bound, body) => body
                .free_vars()
                .into_iter()
                .filter(|n| n != bound)
                .collect(),
        }
    }
}
/// A rule in a deductive synthesis calculus.
///
/// Rules transform goal triples `(pre, post, type)` into sub-goals.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeductiveRule {
    /// Name of the rule (e.g., "seq", "if-then-else", "while").
    pub name: String,
    /// Number of sub-goals this rule produces.
    pub arity: usize,
    /// Informal description.
    pub description: String,
}
impl DeductiveRule {
    /// Build a new deductive rule.
    pub fn new(name: impl Into<String>, arity: usize, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            arity,
            description: description.into(),
        }
    }
}
/// A candidate program in CEGIS, represented as a string expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Candidate {
    /// The program text.
    pub program: String,
    /// The set of inputs on which this candidate has been verified so far.
    pub verified_inputs: Vec<Vec<String>>,
}
impl Candidate {
    /// Create a new candidate from a program string.
    pub fn new(program: impl Into<String>) -> Self {
        Self {
            program: program.into(),
            verified_inputs: Vec::new(),
        }
    }
    /// Simulate evaluation on the given input (placeholder).
    pub fn evaluate(&self, _input: &[String]) -> String {
        "?".into()
    }
}
/// A library component with pre/post-conditions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Component {
    /// Name of the component (e.g., function name).
    pub name: String,
    /// Input sorts.
    pub input_sorts: Vec<String>,
    /// Output sort.
    pub output_sort: String,
    /// Formal precondition.
    pub precondition: String,
    /// Formal postcondition.
    pub postcondition: String,
}
impl Component {
    /// Build a new library component.
    pub fn new(
        name: impl Into<String>,
        input_sorts: Vec<String>,
        output_sort: impl Into<String>,
        precondition: impl Into<String>,
        postcondition: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            input_sorts,
            output_sort: output_sort.into(),
            precondition: precondition.into(),
            postcondition: postcondition.into(),
        }
    }
    /// Check whether the component's output sort matches the target.
    pub fn output_matches(&self, target_sort: &str) -> bool {
        self.output_sort == target_sort
    }
}
/// A higher-order program template for functional synthesis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FuncTemplate {
    /// `map f xs` – apply f to each element.
    Map,
    /// `filter p xs` – keep elements satisfying p.
    Filter,
    /// `fold f init xs` – left fold.
    Fold,
    /// `unfold seed step` – anamorphism.
    Unfold,
    /// Custom combinator with a name.
    Custom(String),
}
impl FuncTemplate {
    /// Return the arity of this template.
    pub fn arity(&self) -> usize {
        match self {
            FuncTemplate::Map | FuncTemplate::Filter | FuncTemplate::Unfold => 2,
            FuncTemplate::Fold => 3,
            FuncTemplate::Custom(_) => 0,
        }
    }
}
/// A bottom-up enumerative synthesiser for functional programs.
#[derive(Debug, Clone)]
pub struct BottomUpSynth {
    /// Maximum program size (AST nodes).
    pub max_size: usize,
    /// Available variable names.
    pub variables: Vec<String>,
    /// Available literal values.
    pub literals: Vec<String>,
}
impl BottomUpSynth {
    /// Create a new bottom-up synthesiser.
    pub fn new(max_size: usize, variables: Vec<String>, literals: Vec<String>) -> Self {
        Self {
            max_size,
            variables,
            literals,
        }
    }
    /// Enumerate all programs of exactly `size` AST nodes.
    pub fn enumerate_size(&self, size: usize) -> Vec<FuncProgram> {
        if size == 1 {
            let mut progs: Vec<FuncProgram> = self
                .literals
                .iter()
                .map(|l| FuncProgram::Lit(l.clone()))
                .collect();
            progs.extend(self.variables.iter().map(|v| FuncProgram::Var(v.clone())));
            progs
        } else {
            let mut progs = Vec::new();
            for f_size in 1..size {
                let a_size = size - 1 - f_size;
                if a_size == 0 {
                    continue;
                }
                let f_progs = self.enumerate_size(f_size);
                let a_progs = self.enumerate_size(a_size);
                for f in &f_progs {
                    for a in &a_progs {
                        progs.push(FuncProgram::App(Box::new(f.clone()), Box::new(a.clone())));
                    }
                }
            }
            progs
        }
    }
}
/// A context-free grammar used as a syntax guide for synthesis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CFG {
    /// Start symbol.
    pub start: String,
    /// All production rules.
    pub productions: Vec<Production>,
    /// Non-terminal symbols.
    pub non_terminals: Vec<String>,
    /// Terminal symbols.
    pub terminals: Vec<String>,
}
impl CFG {
    /// Build a minimal CFG with one production from `start`.
    pub fn new(start: impl Into<String>) -> Self {
        let s = start.into();
        Self {
            start: s.clone(),
            productions: Vec::new(),
            non_terminals: vec![s],
            terminals: Vec::new(),
        }
    }
    /// Add a production rule.
    pub fn add_production(&mut self, lhs: impl Into<String>, rhs: Vec<String>) {
        let lhs_s = lhs.into();
        if !self.non_terminals.contains(&lhs_s) {
            self.non_terminals.push(lhs_s.clone());
        }
        self.productions.push(Production::new(lhs_s, rhs));
    }
    /// Add a terminal symbol.
    pub fn add_terminal(&mut self, t: impl Into<String>) {
        let t_s = t.into();
        if !self.terminals.contains(&t_s) {
            self.terminals.push(t_s);
        }
    }
    /// Count productions for a given non-terminal.
    pub fn productions_for(&self, nt: &str) -> Vec<&Production> {
        self.productions.iter().filter(|p| p.lhs == nt).collect()
    }
}
/// A superoptimiser that searches for the shortest equivalent program.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Superoptimiser {
    /// Instruction set available to the target program.
    pub instruction_set: Vec<String>,
    /// Maximum program length to enumerate.
    pub max_length: usize,
    /// Number of programs tested.
    pub tested: usize,
}
#[allow(dead_code)]
impl Superoptimiser {
    /// Create a new superoptimiser.
    ///
    /// ```
    /// use oxilean_std::program_synthesis::Superoptimiser;
    /// let so = Superoptimiser::new(vec!["add".into(), "mul".into()], 4);
    /// assert_eq!(so.max_length, 4);
    /// ```
    pub fn new(instruction_set: Vec<String>, max_length: usize) -> Self {
        Self {
            instruction_set,
            max_length,
            tested: 0,
        }
    }
    /// Enumerate all programs of the given length.
    pub fn enumerate_length(&self, length: usize) -> Vec<Vec<String>> {
        if length == 0 {
            return vec![vec![]];
        }
        let mut result = Vec::new();
        for shorter in self.enumerate_length(length - 1) {
            for instr in &self.instruction_set {
                let mut prog = shorter.clone();
                prog.push(instr.clone());
                result.push(prog);
            }
        }
        result
    }
    /// Test a candidate program against a reference (placeholder).
    pub fn test_equivalent(&mut self, _candidate: &[String], _reference: &[String]) -> bool {
        self.tested += 1;
        false
    }
    /// Attempt to find the shortest equivalent program of length ≤ max_length.
    pub fn optimise(&mut self, reference: &[String]) -> Option<Vec<String>> {
        for length in 0..=self.max_length {
            for candidate in self.enumerate_length(length) {
                if self.test_equivalent(&candidate, reference) {
                    return Some(candidate);
                }
            }
        }
        None
    }
}
/// The FlashFill-style synthesis algorithm for string transformations.
#[derive(Debug, Clone)]
pub struct FlashFillSynth {
    /// Atomic string operators available.
    pub operators: Vec<String>,
}
impl FlashFillSynth {
    /// Create a FlashFill synthesiser with default string operators.
    pub fn new() -> Self {
        Self {
            operators: vec![
                "Concat".into(),
                "Substr".into(),
                "GetToken".into(),
                "Trim".into(),
                "ToUpper".into(),
                "ToLower".into(),
            ],
        }
    }
    /// Synthesise a string transformation program from examples.
    pub fn synthesise(&self, examples: &[IOExample]) -> Option<String> {
        if examples.is_empty() {
            return None;
        }
        let identity = examples
            .iter()
            .all(|ex| ex.inputs.len() == 1 && ex.inputs[0] == ex.output);
        if identity {
            Some("fun x -> x".into())
        } else {
            None
        }
    }
}
/// A SyGuS solver result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyGuSResult {
    /// A solution expression (satisfies the grammar and the constraint).
    Solution(String),
    /// No solution exists within the grammar.
    Infeasible,
    /// Solver timed out.
    Timeout,
}
/// A SyGuS problem instance.
#[derive(Debug, Clone)]
pub struct SyGuSProblem {
    /// Name of the function to synthesise.
    pub function_name: String,
    /// Argument names and their sorts.
    pub arguments: Vec<(String, String)>,
    /// Return sort.
    pub return_sort: String,
    /// Grammar constraining syntactic form.
    pub grammar: CFG,
    /// Logical constraint (correctness spec).
    pub constraint: String,
}
impl SyGuSProblem {
    /// Build a SyGuS problem.
    pub fn new(
        function_name: impl Into<String>,
        arguments: Vec<(String, String)>,
        return_sort: impl Into<String>,
        grammar: CFG,
        constraint: impl Into<String>,
    ) -> Self {
        Self {
            function_name: function_name.into(),
            arguments,
            return_sort: return_sort.into(),
            grammar,
            constraint: constraint.into(),
        }
    }
    /// Return the arity (number of arguments).
    pub fn arity(&self) -> usize {
        self.arguments.len()
    }
}
/// A component-based synthesis query.
#[derive(Debug, Clone)]
pub struct ComponentSynthQuery {
    /// The target sort to synthesise.
    pub target_sort: String,
    /// The logical specification.
    pub spec: String,
    /// Available inputs (name, sort).
    pub inputs: Vec<(String, String)>,
}
impl ComponentSynthQuery {
    /// Build a synthesis query.
    pub fn new(
        target_sort: impl Into<String>,
        spec: impl Into<String>,
        inputs: Vec<(String, String)>,
    ) -> Self {
        Self {
            target_sort: target_sort.into(),
            spec: spec.into(),
            inputs,
        }
    }
}
/// Type-directed synthesiser: Djinn/Agsy-style proof search.
#[derive(Debug, Clone)]
pub struct TypeDirectedSynth {
    /// Depth limit for proof search.
    pub depth_limit: usize,
    /// Number of synthesis attempts made.
    pub attempts: usize,
}
impl TypeDirectedSynth {
    /// Create a new synthesiser with a depth limit.
    pub fn new(depth_limit: usize) -> Self {
        Self {
            depth_limit,
            attempts: 0,
        }
    }
    /// Attempt to synthesise a term of `goal_type` from `ctx`.
    ///
    /// Returns a program string on success.
    pub fn synthesise(
        &mut self,
        ctx: &SynthContext,
        goal_type: &SynthType,
        depth: usize,
    ) -> Option<String> {
        self.attempts += 1;
        if depth > self.depth_limit {
            return None;
        }
        let matches = ctx.matching(goal_type);
        if let Some((name, _)) = matches.first() {
            return Some(name.to_string());
        }
        match goal_type {
            SynthType::Unit => Some("()".into()),
            SynthType::Arrow(dom, cod) => {
                let arg_name = format!("x{}", depth);
                let mut new_ctx = ctx.clone();
                new_ctx.add(arg_name.clone(), *dom.clone());
                let body = self.synthesise(&new_ctx, cod, depth + 1)?;
                Some(format!("fun {} -> {}", arg_name, body))
            }
            SynthType::Product(a, b) => {
                let pa = self.synthesise(ctx, a, depth + 1)?;
                let pb = self.synthesise(ctx, b, depth + 1)?;
                Some(format!("({}, {})", pa, pb))
            }
            _ => None,
        }
    }
}
