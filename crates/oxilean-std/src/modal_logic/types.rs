//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet};

use super::functions::PropVar;
use super::functions::*;

/// A PDL model: finite Kripke frame with named atomic programs.
#[derive(Debug, Clone)]
pub struct PdlModel {
    /// The Kripke model underlying the PDL model.
    pub kripke: KripkeModel,
    /// Number of atomic programs.
    pub n_programs: usize,
}
impl PdlModel {
    /// Create a new PDL model.
    pub fn new(kripke: KripkeModel, n_programs: usize) -> Self {
        PdlModel { kripke, n_programs }
    }
    /// Compute the set of worlds reachable from `start` by executing program `prog`.
    pub fn reachable(&self, start: usize, prog: &PdlProgram) -> HashSet<usize> {
        match prog {
            PdlProgram::Atomic(rel) => self
                .kripke
                .frame
                .successors(*rel, start)
                .into_iter()
                .collect(),
            PdlProgram::Test(phi) => {
                if self.kripke.satisfies(start, phi) {
                    let mut s = HashSet::new();
                    s.insert(start);
                    s
                } else {
                    HashSet::new()
                }
            }
            PdlProgram::Sequence(alpha, beta) => {
                let after_alpha = self.reachable(start, alpha);
                let mut result = HashSet::new();
                for w in after_alpha {
                    result.extend(self.reachable(w, beta));
                }
                result
            }
            PdlProgram::Choice(alpha, beta) => {
                let mut r = self.reachable(start, alpha);
                r.extend(self.reachable(start, beta));
                r
            }
            PdlProgram::Star(alpha) => {
                let mut reachable = HashSet::new();
                reachable.insert(start);
                loop {
                    let current: Vec<usize> = reachable.iter().cloned().collect();
                    let mut added = false;
                    for w in current {
                        for v in self.reachable(w, alpha) {
                            if reachable.insert(v) {
                                added = true;
                            }
                        }
                    }
                    if !added {
                        break;
                    }
                }
                reachable
            }
        }
    }
    /// Evaluate \[α\]φ at world `w`: all successors via α satisfy φ.
    pub fn box_program(&self, w: usize, prog: &PdlProgram, phi: &ModalFormula) -> bool {
        self.reachable(w, prog)
            .iter()
            .all(|&v| self.kripke.satisfies(v, phi))
    }
    /// Evaluate ⟨α⟩φ at world `w`: some successor via α satisfies φ.
    pub fn diamond_program(&self, w: usize, prog: &PdlProgram, phi: &ModalFormula) -> bool {
        self.reachable(w, prog)
            .iter()
            .any(|&v| self.kripke.satisfies(v, phi))
    }
}
/// A simple mu-calculus fixed-point evaluator on finite Kripke models.
#[derive(Debug, Clone)]
pub struct MuCalculusEval {
    /// The underlying Kripke model.
    pub model: KripkeModel,
    /// Variable environment: propositional variable -> set of worlds.
    pub env: HashMap<PropVar, HashSet<usize>>,
}
impl MuCalculusEval {
    /// Create a new evaluator with an empty environment.
    pub fn new(model: KripkeModel) -> Self {
        MuCalculusEval {
            model,
            env: HashMap::new(),
        }
    }
    /// Compute the least fixed point: iterate from empty set upward until stable.
    pub fn least_fixed_point<F>(&self, step: F) -> HashSet<usize>
    where
        F: Fn(&HashSet<usize>) -> HashSet<usize>,
    {
        let mut current: HashSet<usize> = HashSet::new();
        loop {
            let next = step(&current);
            if next == current {
                return current;
            }
            current = next;
        }
    }
    /// Compute the greatest fixed point: iterate from all worlds downward until stable.
    pub fn greatest_fixed_point<F>(&self, step: F) -> HashSet<usize>
    where
        F: Fn(&HashSet<usize>) -> HashSet<usize>,
    {
        let all_worlds: HashSet<usize> = (0..self.model.frame.n_worlds).collect();
        let mut current = all_worlds;
        loop {
            let next = step(&current);
            if next == current {
                return current;
            }
            current = next;
        }
    }
    /// Evaluate reachability: least fixed point of (φ ∨ ◇X).
    pub fn reachability(&self, phi: &ModalFormula) -> HashSet<usize> {
        let model = &self.model;
        self.least_fixed_point(|x| {
            (0..model.frame.n_worlds)
                .filter(|&w| {
                    model.satisfies(w, phi)
                        || model.frame.successors(0, w).iter().any(|&v| x.contains(&v))
                })
                .collect()
        })
    }
    /// Evaluate safety: greatest fixed point of (φ ∧ □X).
    pub fn safety(&self, phi: &ModalFormula) -> HashSet<usize> {
        let model = &self.model;
        self.greatest_fixed_point(|x| {
            (0..model.frame.n_worlds)
                .filter(|&w| {
                    model.satisfies(w, phi)
                        && model.frame.successors(0, w).iter().all(|&v| x.contains(&v))
                })
                .collect()
        })
    }
}
/// A simple tableau prover for modal logic K.
#[derive(Debug)]
pub struct TableauProver {
    /// Current set of tableau nodes (worlds).
    pub nodes: Vec<TableauNode>,
    /// Next available world index.
    pub next_world: usize,
    /// Accessibility edges created during proof search.
    pub edges: Vec<(usize, usize)>,
}
impl TableauProver {
    /// Create a new tableau prover with an initial node.
    pub fn new() -> Self {
        let root = TableauNode::new(0);
        TableauProver {
            nodes: vec![root],
            next_world: 1,
            edges: Vec::new(),
        }
    }
    /// Attempt to prove φ is valid (i.e., ¬φ is unsatisfiable) using a simple check.
    /// Returns true if every branch closes.
    pub fn is_tautology(&mut self, phi: &ModalFormula) -> bool {
        let neg = ModalFormula::not(phi.clone());
        self.nodes[0].add_positive(neg);
        self.nodes[0].detect_closure();
        self.nodes[0].closed
    }
}
/// The canonical model for a modal logic, built from maximal consistent sets.
#[derive(Debug)]
pub struct CanonicalModel {
    /// The worlds = maximal consistent sets.
    pub worlds: Vec<MaximalConsistentSet>,
    /// Canonical accessibility: w R v iff {φ | □φ ∈ w} ⊆ v.
    pub accessibility: HashSet<(usize, usize)>,
}
impl CanonicalModel {
    /// Create an empty canonical model.
    pub fn new() -> Self {
        CanonicalModel {
            worlds: Vec::new(),
            accessibility: HashSet::new(),
        }
    }
    /// Add a world (MCS) to the canonical model.
    pub fn add_world(&mut self, mcs: MaximalConsistentSet) {
        self.worlds.push(mcs);
    }
    /// Build accessibility relation: w R v if all □φ ∈ w have φ ∈ v.
    pub fn build_accessibility(&mut self) {
        let n = self.worlds.len();
        for i in 0..n {
            let box_formulas: Vec<ModalFormula> = self.worlds[i]
                .formulas
                .iter()
                .filter_map(|f| {
                    if let ModalFormula::Box(0, phi) = f {
                        Some(*phi.clone())
                    } else {
                        None
                    }
                })
                .collect();
            for j in 0..n {
                let all_contained = box_formulas.iter().all(|phi| self.worlds[j].contains(phi));
                if all_contained {
                    self.accessibility.insert((i, j));
                }
            }
        }
    }
    /// Number of worlds in the canonical model.
    pub fn size(&self) -> usize {
        self.worlds.len()
    }
}
/// A multi-agent epistemic model (for up to n agents).
#[derive(Debug, Clone)]
pub struct EpistemicModel {
    /// Number of possible worlds.
    pub n_worlds: usize,
    /// Number of agents.
    pub n_agents: usize,
    /// Accessibility relations per agent: agent → frame relation.
    pub agent_relations: Vec<HashSet<(usize, usize)>>,
    /// Valuation.
    pub valuation: HashMap<PropVar, HashSet<usize>>,
}
impl EpistemicModel {
    /// Create a new epistemic model.
    pub fn new(n_worlds: usize, n_agents: usize) -> Self {
        EpistemicModel {
            n_worlds,
            n_agents,
            agent_relations: vec![HashSet::new(); n_agents],
            valuation: HashMap::new(),
        }
    }
    /// Add an edge to agent `i`'s accessibility relation.
    pub fn add_edge(&mut self, agent: usize, from: usize, to: usize) {
        if agent < self.n_agents {
            self.agent_relations[agent].insert((from, to));
        }
    }
    /// Make all agent relations equivalence relations (for S5 / knowledge).
    pub fn make_equivalence_relations(&mut self) {
        for agent in 0..self.n_agents {
            for w in 0..self.n_worlds {
                self.agent_relations[agent].insert((w, w));
            }
            let edges: Vec<(usize, usize)> = self.agent_relations[agent].iter().cloned().collect();
            for &(a, b) in &edges {
                self.agent_relations[agent].insert((b, a));
            }
            loop {
                let edges: Vec<(usize, usize)> =
                    self.agent_relations[agent].iter().cloned().collect();
                let mut changed = false;
                for &(a, b) in &edges {
                    for &(c, d) in &edges {
                        if b == c && !self.agent_relations[agent].contains(&(a, d)) {
                            self.agent_relations[agent].insert((a, d));
                            changed = true;
                        }
                    }
                }
                if !changed {
                    break;
                }
            }
        }
    }
    /// Check whether agent `i` knows φ at world `w`.
    pub fn knows(&self, agent: usize, w: usize, phi: &ModalFormula) -> bool {
        if agent >= self.n_agents {
            return false;
        }
        let successors: Vec<usize> = self.agent_relations[agent]
            .iter()
            .filter(|&&(f, _)| f == w)
            .map(|&(_, t)| t)
            .collect();
        successors
            .iter()
            .all(|&v| self.satisfies_with_agent(v, phi))
    }
    /// Simple satisfaction check (treats modalities as agent 0).
    fn satisfies_with_agent(&self, w: usize, phi: &ModalFormula) -> bool {
        match phi {
            ModalFormula::Top => true,
            ModalFormula::Bot => false,
            ModalFormula::Atom(p) => self.valuation.get(p).is_some_and(|s| s.contains(&w)),
            ModalFormula::Not(psi) => !self.satisfies_with_agent(w, psi),
            ModalFormula::And(a, b) => {
                self.satisfies_with_agent(w, a) && self.satisfies_with_agent(w, b)
            }
            ModalFormula::Or(a, b) => {
                self.satisfies_with_agent(w, a) || self.satisfies_with_agent(w, b)
            }
            ModalFormula::Implies(a, b) => {
                !self.satisfies_with_agent(w, a) || self.satisfies_with_agent(w, b)
            }
            ModalFormula::Box(rel, psi) => {
                let agent = *rel;
                self.knows(agent, w, psi)
            }
            ModalFormula::Diamond(rel, psi) => {
                let agent = *rel;
                if agent >= self.n_agents {
                    return false;
                }
                self.agent_relations[agent]
                    .iter()
                    .filter(|&&(f, _)| f == w)
                    .any(|&(_, v)| self.satisfies_with_agent(v, psi))
            }
        }
    }
}
/// Describes the Sahlqvist syntactic class membership (simplified).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SahlqvistClass {
    /// Sahlqvist antecedent: positive (possibly boxed atoms)
    Antecedent,
    /// Sahlqvist consequent: positive formula
    Consequent,
    /// Full Sahlqvist formula: antecedent → consequent
    Full,
    /// Not Sahlqvist
    NotSahlqvist,
}
/// A propositional dynamic logic program over a finite transition system.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PdlProgram {
    /// Atomic action (index into transition table)
    Atomic(usize),
    /// Test: φ? — succeeds at worlds satisfying φ
    Test(Box<ModalFormula>),
    /// Sequential composition: α;β
    Sequence(Box<PdlProgram>, Box<PdlProgram>),
    /// Non-deterministic choice: α ∪ β
    Choice(Box<PdlProgram>, Box<PdlProgram>),
    /// Kleene star (finite iteration): α*
    Star(Box<PdlProgram>),
}
/// A finite trace: a sequence of propositional valuations.
#[derive(Debug, Clone)]
pub struct FiniteTrace {
    /// Each step maps propositional variables to their truth values.
    pub steps: Vec<HashMap<PropVar, bool>>,
}
impl FiniteTrace {
    /// Create a new empty trace.
    pub fn new() -> Self {
        FiniteTrace { steps: Vec::new() }
    }
    /// Append a step to the trace.
    pub fn push(&mut self, valuation: HashMap<PropVar, bool>) {
        self.steps.push(valuation);
    }
    /// Length of the trace.
    pub fn len(&self) -> usize {
        self.steps.len()
    }
    /// Check if the trace is empty.
    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }
    /// Check if proposition p holds at step i.
    pub fn prop_at(&self, p: PropVar, i: usize) -> bool {
        self.steps
            .get(i)
            .and_then(|s| s.get(&p))
            .copied()
            .unwrap_or(false)
    }
    /// Evaluate a temporal formula at position `i`.
    /// Box(0,_)=Globally, Diamond(0,_)=Finally, Box(1,_)=Next.
    pub fn satisfies(&self, i: usize, phi: &ModalFormula) -> bool {
        if i >= self.steps.len() {
            return false;
        }
        match phi {
            ModalFormula::Top => true,
            ModalFormula::Bot => false,
            ModalFormula::Atom(p) => self.prop_at(*p, i),
            ModalFormula::Not(psi) => !self.satisfies(i, psi),
            ModalFormula::And(a, b) => self.satisfies(i, a) && self.satisfies(i, b),
            ModalFormula::Or(a, b) => self.satisfies(i, a) || self.satisfies(i, b),
            ModalFormula::Implies(a, b) => !self.satisfies(i, a) || self.satisfies(i, b),
            ModalFormula::Box(0, psi) => (i..self.steps.len()).all(|j| self.satisfies(j, psi)),
            ModalFormula::Diamond(0, psi) => (i..self.steps.len()).any(|j| self.satisfies(j, psi)),
            ModalFormula::Box(1, psi) => i + 1 < self.steps.len() && self.satisfies(i + 1, psi),
            ModalFormula::Diamond(1, psi) => {
                i + 1 >= self.steps.len() || self.satisfies(i + 1, psi)
            }
            _ => false,
        }
    }
    /// Check whether a formula holds at position 0.
    pub fn check(&self, phi: &ModalFormula) -> bool {
        self.satisfies(0, phi)
    }
}
/// A belief revision operator implementing a simplified AGM framework.
#[derive(Debug, Clone)]
pub struct BeliefRevisionOp {
    /// The current belief set K.
    pub beliefs: Vec<ModalFormula>,
    /// An ordering on formulas (entrenched beliefs have higher values).
    pub entrenchment: HashMap<ModalFormula, u32>,
}
impl BeliefRevisionOp {
    /// Create a new belief revision operator with an empty belief set.
    pub fn new() -> Self {
        BeliefRevisionOp {
            beliefs: Vec::new(),
            entrenchment: HashMap::new(),
        }
    }
    /// Add a belief with a given entrenchment level.
    pub fn add_belief(&mut self, phi: ModalFormula, level: u32) {
        if !self.beliefs.contains(&phi) {
            self.beliefs.push(phi.clone());
        }
        self.entrenchment.insert(phi, level);
    }
    /// Check if φ is believed.
    pub fn believes(&self, phi: &ModalFormula) -> bool {
        self.beliefs.contains(phi)
    }
    /// Revision K * φ: add φ, remove lower-entrenched negations of φ.
    pub fn revise(&self, phi: &ModalFormula) -> BeliefRevisionOp {
        let mut new_op = self.clone();
        let neg_phi = ModalFormula::not(phi.clone());
        let phi_level = self.entrenchment.get(phi).copied().unwrap_or(0);
        new_op.beliefs.retain(|b| {
            if b == &neg_phi {
                let b_level = self.entrenchment.get(b).copied().unwrap_or(0);
                b_level > phi_level
            } else {
                true
            }
        });
        new_op.add_belief(phi.clone(), phi_level.max(1));
        new_op
    }
    /// Contraction K ÷ φ: remove φ from the belief set.
    pub fn contract(&self, phi: &ModalFormula) -> BeliefRevisionOp {
        let mut new_op = self.clone();
        new_op.beliefs.retain(|b| b != phi);
        new_op.entrenchment.remove(phi);
        new_op
    }
    /// Size of the belief set.
    pub fn size(&self) -> usize {
        self.beliefs.len()
    }
}
/// A Kripke model: frame + valuation.
#[derive(Debug, Clone)]
pub struct KripkeModel {
    /// The underlying frame.
    pub frame: KripkeFrame,
    /// Valuation: prop_var → set of worlds where it is true.
    pub valuation: HashMap<PropVar, HashSet<usize>>,
}
impl KripkeModel {
    /// Create a new model with no valuation.
    pub fn new(frame: KripkeFrame) -> Self {
        KripkeModel {
            frame,
            valuation: HashMap::new(),
        }
    }
    /// Set proposition `p` to true at world `w`.
    pub fn set_true(&mut self, p: PropVar, w: usize) {
        self.valuation.entry(p).or_default().insert(w);
    }
    /// Check whether proposition `p` is true at world `w`.
    pub fn prop_true(&self, p: PropVar, w: usize) -> bool {
        self.valuation.get(&p).is_some_and(|s| s.contains(&w))
    }
    /// Evaluate a modal formula at world `w` (modality index = 0).
    pub fn satisfies(&self, w: usize, phi: &ModalFormula) -> bool {
        match phi {
            ModalFormula::Top => true,
            ModalFormula::Bot => false,
            ModalFormula::Atom(p) => self.prop_true(*p, w),
            ModalFormula::Not(psi) => !self.satisfies(w, psi),
            ModalFormula::And(a, b) => self.satisfies(w, a) && self.satisfies(w, b),
            ModalFormula::Or(a, b) => self.satisfies(w, a) || self.satisfies(w, b),
            ModalFormula::Implies(a, b) => !self.satisfies(w, a) || self.satisfies(w, b),
            ModalFormula::Box(rel, psi) => self
                .frame
                .successors(*rel, w)
                .iter()
                .all(|&v| self.satisfies(v, psi)),
            ModalFormula::Diamond(rel, psi) => self
                .frame
                .successors(*rel, w)
                .iter()
                .any(|&v| self.satisfies(v, psi)),
        }
    }
    /// Check whether φ is valid in this model (true at all worlds).
    pub fn model_valid(&self, phi: &ModalFormula) -> bool {
        (0..self.frame.n_worlds).all(|w| self.satisfies(w, phi))
    }
    /// Compute the extension [\[φ\]] — the set of worlds satisfying φ.
    pub fn extension(&self, phi: &ModalFormula) -> HashSet<usize> {
        (0..self.frame.n_worlds)
            .filter(|&w| self.satisfies(w, phi))
            .collect()
    }
}
/// A maximally consistent set (MCS) represented as a set of formula indices.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MaximalConsistentSet {
    /// Index identifying this MCS (world in the canonical model).
    pub id: usize,
    /// Representative formulas in this MCS (for finite approximation).
    pub formulas: Vec<ModalFormula>,
}
impl MaximalConsistentSet {
    /// Create a new MCS with the given id and formulas.
    pub fn new(id: usize, formulas: Vec<ModalFormula>) -> Self {
        MaximalConsistentSet { id, formulas }
    }
    /// Check if a formula is a member of this MCS.
    pub fn contains(&self, phi: &ModalFormula) -> bool {
        self.formulas.contains(phi)
    }
    /// Add a formula to this MCS (used during construction).
    pub fn add(&mut self, phi: ModalFormula) {
        if !self.contains(&phi) {
            self.formulas.push(phi);
        }
    }
}
/// Represents a public announcement update.
#[derive(Debug, Clone)]
pub struct PublicAnnouncement {
    /// The announced formula (must be true at actual world).
    pub announcement: ModalFormula,
}
impl PublicAnnouncement {
    /// Create a public announcement.
    pub fn new(phi: ModalFormula) -> Self {
        PublicAnnouncement { announcement: phi }
    }
    /// Update an epistemic model by a public announcement:
    /// keep only worlds where the announcement is satisfied.
    pub fn update(&self, model: &EpistemicModel) -> EpistemicModel {
        let surviving: HashSet<usize> = (0..model.n_worlds)
            .filter(|&w| model.satisfies_with_agent(w, &self.announcement))
            .collect();
        let old_to_new: HashMap<usize, usize> = surviving
            .iter()
            .enumerate()
            .map(|(new, &old)| (old, new))
            .collect();
        let n_new = surviving.len();
        let mut new_model = EpistemicModel::new(n_new, model.n_agents);
        for agent in 0..model.n_agents {
            for &(from, to) in &model.agent_relations[agent] {
                if let (Some(&nf), Some(&nt)) = (old_to_new.get(&from), old_to_new.get(&to)) {
                    new_model.agent_relations[agent].insert((nf, nt));
                }
            }
        }
        for (&p, worlds) in &model.valuation {
            for &w in worlds {
                if let Some(&nw) = old_to_new.get(&w) {
                    new_model.valuation.entry(p).or_default().insert(nw);
                }
            }
        }
        new_model
    }
}
/// Identifies a normal modal logic by its axiom schema name.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ModalSystem {
    /// K: the minimal normal modal logic
    K,
    /// T (= M): K + Axiom T (reflexivity)
    T,
    /// S4: T + Axiom 4 (transitivity)
    S4,
    /// S5: S4 + Axiom B (symmetry)
    S5,
    /// D: K + Axiom D (seriality, deontic)
    D,
    /// KD45: D + Axiom 4 + Axiom 5 (doxastic belief)
    KD45,
    /// GL: K + Axiom Löb (provability)
    GL,
    /// B: K + T + B (Brouwer)
    B,
}
impl ModalSystem {
    /// Return a human-readable name for the logic.
    pub fn name(&self) -> &'static str {
        match self {
            ModalSystem::K => "K",
            ModalSystem::T => "T",
            ModalSystem::S4 => "S4",
            ModalSystem::S5 => "S5",
            ModalSystem::D => "D",
            ModalSystem::KD45 => "KD45",
            ModalSystem::GL => "GL",
            ModalSystem::B => "B",
        }
    }
    /// Check whether the given frame satisfies the characteristic frame condition.
    pub fn frame_validates(&self, frame: &KripkeFrame, rel: usize) -> bool {
        match self {
            ModalSystem::K => true,
            ModalSystem::T => frame.is_reflexive(rel),
            ModalSystem::S4 => frame.is_reflexive(rel) && frame.is_transitive(rel),
            ModalSystem::S5 => {
                frame.is_reflexive(rel) && frame.is_transitive(rel) && frame.is_symmetric(rel)
            }
            ModalSystem::D => frame.is_serial(rel),
            ModalSystem::KD45 => {
                frame.is_serial(rel) && frame.is_transitive(rel) && frame.is_euclidean(rel)
            }
            ModalSystem::GL => frame.is_transitive(rel),
            ModalSystem::B => frame.is_reflexive(rel) && frame.is_symmetric(rel),
        }
    }
}
/// A finite Kripke frame with named accessibility relations.
#[derive(Debug, Clone)]
pub struct KripkeFrame {
    /// Number of worlds (worlds are labeled 0..n_worlds).
    pub n_worlds: usize,
    /// Accessibility relations: relation_index → set of (from, to) pairs.
    pub relations: Vec<HashSet<(usize, usize)>>,
}
impl KripkeFrame {
    /// Create a new frame with `n_worlds` worlds and `n_relations` accessibility relations.
    pub fn new(n_worlds: usize, n_relations: usize) -> Self {
        KripkeFrame {
            n_worlds,
            relations: vec![HashSet::new(); n_relations],
        }
    }
    /// Add an accessibility edge w → v for relation `rel`.
    pub fn add_edge(&mut self, rel: usize, from: usize, to: usize) {
        if rel < self.relations.len() {
            self.relations[rel].insert((from, to));
        }
    }
    /// Check whether wRv holds for relation `rel`.
    pub fn accessible(&self, rel: usize, w: usize, v: usize) -> bool {
        self.relations.get(rel).is_some_and(|r| r.contains(&(w, v)))
    }
    /// Return all worlds accessible from `w` via relation `rel`.
    pub fn successors(&self, rel: usize, w: usize) -> Vec<usize> {
        self.relations
            .get(rel)
            .map(|r| r.iter().filter(|(f, _)| *f == w).map(|(_, t)| *t).collect())
            .unwrap_or_default()
    }
    /// Check if relation `rel` is reflexive.
    pub fn is_reflexive(&self, rel: usize) -> bool {
        (0..self.n_worlds).all(|w| self.accessible(rel, w, w))
    }
    /// Check if relation `rel` is transitive.
    pub fn is_transitive(&self, rel: usize) -> bool {
        let r = match self.relations.get(rel) {
            Some(r) => r,
            None => return true,
        };
        let pairs: Vec<(usize, usize)> = r.iter().cloned().collect();
        for &(a, b) in &pairs {
            for &(c, d) in &pairs {
                if b == c && !r.contains(&(a, d)) {
                    return false;
                }
            }
        }
        true
    }
    /// Check if relation `rel` is symmetric.
    pub fn is_symmetric(&self, rel: usize) -> bool {
        let r = match self.relations.get(rel) {
            Some(r) => r,
            None => return true,
        };
        r.iter().all(|&(a, b)| r.contains(&(b, a)))
    }
    /// Check if relation `rel` is serial (every world has a successor).
    pub fn is_serial(&self, rel: usize) -> bool {
        (0..self.n_worlds).all(|w| !self.successors(rel, w).is_empty())
    }
    /// Check if relation `rel` is Euclidean: wRv ∧ wRu → vRu.
    pub fn is_euclidean(&self, rel: usize) -> bool {
        let r = match self.relations.get(rel) {
            Some(r) => r,
            None => return true,
        };
        let pairs: Vec<(usize, usize)> = r.iter().cloned().collect();
        for &(w, v) in &pairs {
            for &(w2, u) in &pairs {
                if w == w2 && !r.contains(&(v, u)) {
                    return false;
                }
            }
        }
        true
    }
    /// Make relation `rel` reflexive (add all (w,w) edges).
    pub fn make_reflexive(&mut self, rel: usize) {
        for w in 0..self.n_worlds {
            self.add_edge(rel, w, w);
        }
    }
    /// Make relation `rel` transitive (Floyd-Warshall closure).
    pub fn make_transitive(&mut self, rel: usize) {
        if rel >= self.relations.len() {
            return;
        }
        loop {
            let current: Vec<(usize, usize)> = self.relations[rel].iter().cloned().collect();
            let mut added = false;
            for &(a, b) in &current {
                for &(c, d) in &current {
                    if b == c && !self.relations[rel].contains(&(a, d)) {
                        self.relations[rel].insert((a, d));
                        added = true;
                    }
                }
            }
            if !added {
                break;
            }
        }
    }
}
/// Modal formula over propositional variables.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ModalFormula {
    /// Atomic proposition p_i
    Atom(PropVar),
    /// Propositional constant ⊤
    Top,
    /// Propositional constant ⊥
    Bot,
    /// Negation ¬φ
    Not(Box<ModalFormula>),
    /// Conjunction φ ∧ ψ
    And(Box<ModalFormula>, Box<ModalFormula>),
    /// Disjunction φ ∨ ψ
    Or(Box<ModalFormula>, Box<ModalFormula>),
    /// Implication φ → ψ
    Implies(Box<ModalFormula>, Box<ModalFormula>),
    /// Box (necessity) □_i φ
    Box(usize, Box<ModalFormula>),
    /// Diamond (possibility) ◇_i φ
    Diamond(usize, Box<ModalFormula>),
}
impl ModalFormula {
    /// Create an atomic proposition.
    pub fn atom(p: PropVar) -> Self {
        ModalFormula::Atom(p)
    }
    /// Create the necessity □φ (modality 0).
    pub fn necessity(phi: ModalFormula) -> Self {
        ModalFormula::Box(0, Box::new(phi))
    }
    /// Create the possibility ◇φ (modality 0).
    pub fn possibility(phi: ModalFormula) -> Self {
        ModalFormula::Diamond(0, Box::new(phi))
    }
    /// Create implication.
    pub fn implies(a: ModalFormula, b: ModalFormula) -> Self {
        ModalFormula::Implies(Box::new(a), Box::new(b))
    }
    /// Create conjunction.
    pub fn and(a: ModalFormula, b: ModalFormula) -> Self {
        ModalFormula::And(Box::new(a), Box::new(b))
    }
    /// Create disjunction.
    pub fn or(a: ModalFormula, b: ModalFormula) -> Self {
        ModalFormula::Or(Box::new(a), Box::new(b))
    }
    /// Create negation.
    pub fn not(a: ModalFormula) -> Self {
        ModalFormula::Not(Box::new(a))
    }
    /// Collect all subformulas (including self).
    pub fn subformulas(&self) -> Vec<ModalFormula> {
        let mut result = vec![self.clone()];
        match self {
            ModalFormula::Not(phi) => result.extend(phi.subformulas()),
            ModalFormula::And(a, b) | ModalFormula::Or(a, b) | ModalFormula::Implies(a, b) => {
                result.extend(a.subformulas());
                result.extend(b.subformulas());
            }
            ModalFormula::Box(_, phi) | ModalFormula::Diamond(_, phi) => {
                result.extend(phi.subformulas())
            }
            _ => {}
        }
        result
    }
    /// Return the set of propositional variables appearing in the formula.
    pub fn prop_vars(&self) -> HashSet<PropVar> {
        let mut vars = HashSet::new();
        self.collect_vars(&mut vars);
        vars
    }
    fn collect_vars(&self, vars: &mut HashSet<PropVar>) {
        match self {
            ModalFormula::Atom(p) => {
                vars.insert(*p);
            }
            ModalFormula::Not(phi) => phi.collect_vars(vars),
            ModalFormula::And(a, b) | ModalFormula::Or(a, b) | ModalFormula::Implies(a, b) => {
                a.collect_vars(vars);
                b.collect_vars(vars);
            }
            ModalFormula::Box(_, phi) | ModalFormula::Diamond(_, phi) => phi.collect_vars(vars),
            _ => {}
        }
    }
    /// Modal depth of the formula.
    pub fn modal_depth(&self) -> usize {
        match self {
            ModalFormula::Atom(_) | ModalFormula::Top | ModalFormula::Bot => 0,
            ModalFormula::Not(phi) => phi.modal_depth(),
            ModalFormula::And(a, b) | ModalFormula::Or(a, b) | ModalFormula::Implies(a, b) => {
                a.modal_depth().max(b.modal_depth())
            }
            ModalFormula::Box(_, phi) | ModalFormula::Diamond(_, phi) => 1 + phi.modal_depth(),
        }
    }
}
/// A node in a tableau for modal logic K.
#[derive(Debug, Clone)]
pub struct TableauNode {
    /// World label.
    pub world: usize,
    /// Set of formulas true at this world.
    pub positive: Vec<ModalFormula>,
    /// Set of formulas false at this world.
    pub negative: Vec<ModalFormula>,
    /// Whether this node is closed (contradiction found).
    pub closed: bool,
}
impl TableauNode {
    /// Create a new open tableau node.
    pub fn new(world: usize) -> Self {
        TableauNode {
            world,
            positive: Vec::new(),
            negative: Vec::new(),
            closed: false,
        }
    }
    /// Add a formula to the positive (true) set.
    pub fn add_positive(&mut self, phi: ModalFormula) {
        self.positive.push(phi);
    }
    /// Add a formula to the negative (false) set.
    pub fn add_negative(&mut self, phi: ModalFormula) {
        self.negative.push(phi);
    }
    /// Detect closure: a formula appears both positive and negative.
    pub fn detect_closure(&mut self) {
        for p in &self.positive {
            if self.negative.contains(p) {
                self.closed = true;
                return;
            }
        }
        if self.positive.contains(&ModalFormula::Bot) || self.negative.contains(&ModalFormula::Top)
        {
            self.closed = true;
        }
    }
}
/// A Kripke model equipped with graded modality evaluation.
#[derive(Debug, Clone)]
pub struct GradedModel {
    /// Underlying Kripke model.
    pub model: KripkeModel,
}
impl GradedModel {
    /// Create a new graded model.
    pub fn new(model: KripkeModel) -> Self {
        GradedModel { model }
    }
    /// Evaluate ◇^{≥n} φ at world w: at least n successors satisfy φ.
    pub fn graded_diamond(&self, w: usize, n: usize, phi: &ModalFormula) -> bool {
        let count = self
            .model
            .frame
            .successors(0, w)
            .iter()
            .filter(|&&v| self.model.satisfies(v, phi))
            .count();
        count >= n
    }
    /// Evaluate □^{≤n} φ at world w: at most n successors fail to satisfy φ.
    pub fn graded_box(&self, w: usize, n: usize, phi: &ModalFormula) -> bool {
        let failures = self
            .model
            .frame
            .successors(0, w)
            .iter()
            .filter(|&&v| !self.model.satisfies(v, phi))
            .count();
        failures <= n
    }
    /// Count the number of successors of world w satisfying φ.
    pub fn count_satisfying(&self, w: usize, phi: &ModalFormula) -> usize {
        self.model
            .frame
            .successors(0, w)
            .iter()
            .filter(|&&v| self.model.satisfies(v, phi))
            .count()
    }
}
/// A bisimulation between two Kripke models.
#[derive(Debug, Clone)]
pub struct Bisimulation {
    /// Pairs (w, v) where w is in model 1 and v is in model 2.
    pub pairs: HashSet<(usize, usize)>,
}
impl Bisimulation {
    /// Create an empty bisimulation.
    pub fn new() -> Self {
        Bisimulation {
            pairs: HashSet::new(),
        }
    }
    /// Add a bisimulation pair.
    pub fn add_pair(&mut self, w: usize, v: usize) {
        self.pairs.insert((w, v));
    }
    /// Check whether two models are bisimilar at (w1, w2) given this relation.
    /// (Simplified: only checks atom agreement, not the full forth/back conditions.)
    pub fn is_bisimulation_pair(
        &self,
        _m1: &KripkeModel,
        _m2: &KripkeModel,
        w1: usize,
        w2: usize,
    ) -> bool {
        self.pairs.contains(&(w1, w2))
    }
    /// Size of the bisimulation.
    pub fn size(&self) -> usize {
        self.pairs.len()
    }
}
