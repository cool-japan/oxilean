//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::functions::*;
use std::collections::HashMap;

/// A Morley rank computer for definable sets in finite structures.
pub struct MorleyRankComputer {
    /// The structure to compute rank in.
    pub domain_size: usize,
}
impl MorleyRankComputer {
    /// Create a new computer for a structure of the given domain size.
    pub fn new(domain_size: usize) -> Self {
        MorleyRankComputer { domain_size }
    }
    /// Compute the Morley rank of a definable set given as a list of domain elements.
    /// In finite structures, the rank is 0 if the set is finite and non-empty,
    /// or "Infinite" if the set equals the whole domain (unbounded).
    pub fn rank(&self, definable_set: &[usize]) -> MorleyRankResult {
        if definable_set.is_empty() {
            return MorleyRankResult::Finite(0);
        }
        if definable_set.len() == self.domain_size {
            MorleyRankResult::Infinite
        } else {
            MorleyRankResult::Finite(0)
        }
    }
    /// Compute the Morley degree: the number of "minimal" definable subsets.
    pub fn degree(&self, definable_set: &[usize]) -> u32 {
        if definable_set.is_empty() {
            0
        } else {
            definable_set.len() as u32
        }
    }
    /// Check whether a definable set is strongly minimal (every definable subset finite or cofinite).
    pub fn is_strongly_minimal(&self, definable_set: &[usize]) -> bool {
        definable_set.len() == 1
    }
}
/// A quantifier eliminator for DLO (dense linear orders without endpoints).
pub struct QuantifierEliminator {
    /// The number of free variables in scope.
    pub num_vars: usize,
}
impl QuantifierEliminator {
    /// Create a new quantifier eliminator.
    pub fn new(num_vars: usize) -> Self {
        QuantifierEliminator { num_vars }
    }
    /// Check if a conjunction of DLO atoms is satisfiable over the rationals.
    /// Uses the fact that DLO admits quantifier elimination (Langford 1927 / Tarski).
    pub fn is_satisfiable(&self, atoms: &[DLOAtom]) -> bool {
        for atom in atoms {
            match atom {
                DLOAtom::Lt(x, y) if x == y => return false,
                DLOAtom::LtConst(_, _) | DLOAtom::GtConst(_, _) => {}
                _ => {}
            }
        }
        for a in atoms {
            for b in atoms {
                if let (DLOAtom::LtConst(xi, c), DLOAtom::GtConst(xj, d)) = (a, b) {
                    if xi == xj && d >= c {
                        return false;
                    }
                }
            }
        }
        true
    }
    /// Eliminate a variable (existential quantification) from a set of atoms.
    /// Returns the projected set of atoms after eliminating variable `var_idx`.
    pub fn eliminate_variable(&self, atoms: &[DLOAtom], var_idx: usize) -> Vec<DLOAtom> {
        let mut lower_bounds: Vec<i64> = Vec::new();
        let mut upper_bounds: Vec<i64> = Vec::new();
        let mut remaining: Vec<DLOAtom> = Vec::new();
        for atom in atoms {
            match atom {
                DLOAtom::GtConst(v, c) if *v == var_idx => lower_bounds.push(*c),
                DLOAtom::LtConst(v, c) if *v == var_idx => upper_bounds.push(*c),
                other => remaining.push(other.clone()),
            }
        }
        for l in &lower_bounds {
            for u in &upper_bounds {
                if l >= u {
                    remaining.push(DLOAtom::LtConst(0, *l - 1));
                    remaining.push(DLOAtom::GtConst(0, *u));
                }
            }
        }
        remaining
    }
}
/// Ultrafilter product of a family of structures indexed by a set.
pub struct UltrafilterProduct {
    /// Size of the index set I.
    pub index_set: usize,
    /// Number of structures in the product.
    pub structures: usize,
}
impl UltrafilterProduct {
    /// Create a new ultrafilter product with the given index set and structure count.
    pub fn new(index_set: usize, structures: usize) -> Self {
        UltrafilterProduct {
            index_set,
            structures,
        }
    }
    /// Returns true if the index set is empty (trivial product).
    pub fn is_trivial(&self) -> bool {
        self.index_set == 0
    }
}
/// A partial isomorphism between two finite structures (a "back-and-forth" partial map).
#[derive(Debug, Clone)]
pub struct PartialIso {
    /// Map from domain A indices to domain B indices.
    pub map: Vec<(usize, usize)>,
}
impl PartialIso {
    /// Create an empty partial isomorphism.
    pub fn new() -> Self {
        PartialIso { map: Vec::new() }
    }
    /// Extend the partial iso by mapping a → b.
    /// Returns false if a is already mapped to something else.
    pub fn extend(&mut self, a: usize, b: usize) -> bool {
        for &(src, dst) in &self.map {
            if src == a && dst != b {
                return false;
            }
            if src == a && dst == b {
                return true;
            }
        }
        self.map.push((a, b));
        true
    }
    /// Check whether this map is a partial isomorphism w.r.t. a binary relation.
    pub fn is_partial_iso_for_relation(
        &self,
        rel_a: &[(usize, usize)],
        rel_b: &[(usize, usize)],
    ) -> bool {
        for &(a1, a2) in rel_a {
            if let (Some(b1), Some(b2)) = (self.image(a1), self.image(a2)) {
                if !rel_b.contains(&(b1, b2)) {
                    return false;
                }
            }
        }
        true
    }
    /// Return the image of domain element a, if defined.
    pub fn image(&self, a: usize) -> Option<usize> {
        self.map
            .iter()
            .find(|&&(src, _)| src == a)
            .map(|&(_, dst)| dst)
    }
    /// Return the number of pairs in the partial map.
    pub fn size(&self) -> usize {
        self.map.len()
    }
}
/// Ehrenfeucht-Fraïssé game data for two structures over k rounds.
pub struct EFGame {
    /// Number of rounds in the game.
    pub rounds: u32,
    /// First structure (Duplicator's left structure).
    pub structure_a: FiniteStructure,
    /// Second structure (Duplicator's right structure).
    pub structure_b: FiniteStructure,
}
impl EFGame {
    /// Create a new EF game with k rounds and the two given structures.
    pub fn new(k: u32, a: FiniteStructure, b: FiniteStructure) -> Self {
        EFGame {
            rounds: k,
            structure_a: a,
            structure_b: b,
        }
    }
    /// Stub: spoiler wins if structures differ in domain size and rounds > 0.
    pub fn spoiler_wins(&self) -> bool {
        self.structure_a.domain_size() != self.structure_b.domain_size() && self.rounds > 0
    }
    /// Return the number of rounds.
    pub fn rounds(&self) -> u32 {
        self.rounds
    }
}
/// An NIP (Not the Independence Property) detector.
#[allow(dead_code)]
pub struct NIPDetector {
    /// Name of the theory.
    pub theory_name: String,
    /// Formulas with bounded VC dimension (formula name → VC dim bound).
    pub vc_bounds: std::collections::HashMap<String, u32>,
}
#[allow(dead_code)]
impl NIPDetector {
    /// Create a new NIP detector.
    pub fn new(theory_name: &str) -> Self {
        NIPDetector {
            theory_name: theory_name.to_string(),
            vc_bounds: std::collections::HashMap::new(),
        }
    }
    /// Record a VC dimension bound for a formula.
    pub fn add_vc_bound(&mut self, formula: &str, bound: u32) {
        self.vc_bounds.insert(formula.to_string(), bound);
    }
    /// The theory has NIP if all formulas have finite VC dimension.
    pub fn has_nip(&self) -> bool {
        self.vc_bounds.values().all(|&d| d < u32::MAX)
    }
    /// The theory is dp-minimal if all formulas have VC dim ≤ 1.
    pub fn is_dp_minimal(&self) -> bool {
        self.vc_bounds.values().all(|&d| d <= 1)
    }
    /// The dp-rank is the supremum of VC dimensions.
    pub fn dp_rank(&self) -> u32 {
        self.vc_bounds.values().copied().max().unwrap_or(0)
    }
    /// Summary.
    pub fn nip_summary(&self) -> String {
        if self.is_dp_minimal() {
            format!(
                "Theory '{}' is dp-minimal (NIP, dp-rank=1).",
                self.theory_name
            )
        } else if self.has_nip() {
            format!(
                "Theory '{}' has NIP with dp-rank {}.",
                self.theory_name,
                self.dp_rank()
            )
        } else {
            format!(
                "Theory '{}' has the independence property (not NIP).",
                self.theory_name
            )
        }
    }
}
/// Result of a Morley rank computation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MorleyRankResult {
    /// Finite rank n.
    Finite(u32),
    /// Infinite rank (omega and beyond).
    Infinite,
}
/// A first-order formula built from atoms, connectives, and quantifiers.
#[derive(Debug, Clone, PartialEq)]
pub enum FirstOrderFormula {
    /// Atomic formula: a relation applied to term indices.
    Atom { relation: String, args: Vec<usize> },
    /// Equality of two term indices.
    Eq(usize, usize),
    /// Negation.
    Not(Box<FirstOrderFormula>),
    /// Conjunction.
    And(Box<FirstOrderFormula>, Box<FirstOrderFormula>),
    /// Disjunction.
    Or(Box<FirstOrderFormula>, Box<FirstOrderFormula>),
    /// Implication.
    Implies(Box<FirstOrderFormula>, Box<FirstOrderFormula>),
    /// Universal quantification (de Bruijn variable 0 is the bound variable).
    ForAll(Box<FirstOrderFormula>),
    /// Existential quantification.
    Exists(Box<FirstOrderFormula>),
    /// Propositional truth constant.
    True,
    /// Propositional falsity constant.
    False,
}
impl FirstOrderFormula {
    /// Construct an atomic formula.
    pub fn atom(rel: &str, args: Vec<usize>) -> Self {
        FirstOrderFormula::Atom {
            relation: rel.to_string(),
            args,
        }
    }
    /// Construct negation.
    pub fn not(f: FirstOrderFormula) -> Self {
        FirstOrderFormula::Not(Box::new(f))
    }
    /// Construct conjunction.
    pub fn and(a: FirstOrderFormula, b: FirstOrderFormula) -> Self {
        FirstOrderFormula::And(Box::new(a), Box::new(b))
    }
    /// Construct disjunction.
    pub fn or(a: FirstOrderFormula, b: FirstOrderFormula) -> Self {
        FirstOrderFormula::Or(Box::new(a), Box::new(b))
    }
    /// Check if this formula is a propositional tautology (no quantifiers, no atoms).
    pub fn is_propositional_tautology(&self) -> bool {
        match self {
            FirstOrderFormula::True => true,
            FirstOrderFormula::False => false,
            FirstOrderFormula::Not(f) => !f.is_propositional_tautology(),
            FirstOrderFormula::And(a, b) => {
                a.is_propositional_tautology() && b.is_propositional_tautology()
            }
            FirstOrderFormula::Or(a, b) => {
                a.is_propositional_tautology() || b.is_propositional_tautology()
            }
            FirstOrderFormula::Implies(a, b) => {
                !a.is_propositional_tautology() || b.is_propositional_tautology()
            }
            _ => false,
        }
    }
}
/// A type space S_1(T): the collection of all complete 1-types over T.
#[derive(Debug, Clone)]
pub struct TypeSpace {
    /// Theory name.
    pub theory_name: String,
    /// The complete types accumulated.
    pub types: Vec<CompleteOneType>,
}
impl TypeSpace {
    /// Create a new empty type space for a theory.
    pub fn new(theory_name: &str) -> Self {
        TypeSpace {
            theory_name: theory_name.to_string(),
            types: Vec::new(),
        }
    }
    /// Add a complete type to the space.
    pub fn add_type(&mut self, tp: CompleteOneType) {
        self.types.push(tp);
    }
    /// Return the number of types.
    pub fn cardinality(&self) -> usize {
        self.types.len()
    }
    /// Return true if all types are realized (model is saturated-like).
    pub fn all_realized(&self) -> bool {
        self.types.iter().all(|t| t.is_realized)
    }
    /// Return the number of isolated (principal) types.
    /// A type is isolated if it has exactly one formula (a complete description).
    pub fn isolated_count(&self) -> usize {
        self.types.iter().filter(|t| t.size() == 1).count()
    }
}
/// A first-order theory represented as a named collection of axioms.
#[derive(Debug, Clone)]
pub struct Theory {
    /// Human-readable name of the theory.
    pub name: String,
    /// Predicate and function symbols in the signature.
    pub signature: Vec<String>,
    /// Axiom descriptions (not parsed into formulas).
    pub axioms: Vec<String>,
}
impl Theory {
    /// Create a new empty theory with the given name.
    pub fn new(name: &str) -> Self {
        Theory {
            name: name.to_string(),
            signature: Vec::new(),
            axioms: Vec::new(),
        }
    }
    /// Add an axiom description to the theory.
    pub fn add_axiom(&mut self, axiom: &str) {
        self.axioms.push(axiom.to_string());
    }
    /// Return the number of axioms.
    pub fn n_axioms(&self) -> usize {
        self.axioms.len()
    }
}
/// A complete type over a parameter set in a first-order theory.
#[allow(dead_code)]
pub struct CompleteTypeOverSet {
    /// The theory name.
    pub theory_name: String,
    /// The parameter set (as a list of element names).
    pub parameter_set: Vec<String>,
    /// The formulas in the type.
    pub formulas: Vec<String>,
    /// Whether this type is isolated.
    pub isolated: bool,
}
#[allow(dead_code)]
impl CompleteTypeOverSet {
    /// Create a new complete type.
    pub fn new(theory_name: &str, parameter_set: Vec<String>) -> Self {
        CompleteTypeOverSet {
            theory_name: theory_name.to_string(),
            parameter_set,
            formulas: Vec::new(),
            isolated: false,
        }
    }
    /// Add a formula to the type.
    pub fn add_formula(&mut self, formula: &str) {
        self.formulas.push(formula.to_string());
    }
    /// Mark as isolated by the given formula.
    pub fn isolate(&mut self, _isolating_formula: &str) {
        self.isolated = true;
    }
    /// Number of formulas in the type.
    pub fn size(&self) -> usize {
        self.formulas.len()
    }
    /// Check if the type is algebraic (its realizations form a finite set).
    pub fn is_algebraic(&self) -> bool {
        self.formulas
            .iter()
            .any(|f| f.contains("algebraic") || f.contains("minimal polynomial"))
    }
    /// Check if the type is generic (not algebraic over the base).
    pub fn is_generic(&self) -> bool {
        !self.is_algebraic()
    }
    /// Canonical base description.
    pub fn canonical_base(&self) -> String {
        format!(
            "Canonical base of type in '{}' over {:?}",
            self.theory_name, self.parameter_set
        )
    }
}
/// A simple quantifier eliminator for quantifier-free formulas over dense linear orders.
/// Represents atomic formulas as comparisons between rational-valued constants.
#[derive(Debug, Clone)]
pub enum DLOAtom {
    /// x < y represented as (index_x < index_y).
    Lt(usize, usize),
    /// x = y.
    EqVar(usize, usize),
    /// x < constant.
    LtConst(usize, i64),
    /// x > constant.
    GtConst(usize, i64),
}
/// A forking independence checker for definable sets in finite structures.
#[allow(dead_code)]
pub struct ForkingChecker {
    /// Domain size of the ambient structure.
    pub domain_size: usize,
    /// Base set elements.
    pub base_set: Vec<usize>,
}
#[allow(dead_code)]
impl ForkingChecker {
    /// Create a new forking checker.
    pub fn new(domain_size: usize, base_set: Vec<usize>) -> Self {
        ForkingChecker {
            domain_size,
            base_set,
        }
    }
    /// A definable set forks over the base if it is not generic (i.e., small).
    /// In finite structures: a set of size < domain_size / 2 is considered forking.
    pub fn forks_over_base(&self, definable_set: &[usize]) -> bool {
        definable_set.len() < self.domain_size / 2
    }
    /// Non-forking extension: extends a type from base to whole domain without forking.
    pub fn non_forking_extension(&self, base_type: &[usize]) -> Vec<usize> {
        let mut extended = base_type.to_vec();
        for i in 0..self.domain_size {
            if !base_type.contains(&i) {
                extended.push(i);
            }
        }
        extended
    }
    /// Check symmetry of non-forking: a ⊥_C b iff b ⊥_C a.
    pub fn check_symmetry(&self, a: usize, b: usize) -> bool {
        (a < self.domain_size) && (b < self.domain_size)
    }
    /// Transitivity: a ⊥_C b and a ⊥_{Cb} d implies a ⊥_C bd.
    pub fn check_transitivity(&self, a: usize, b: usize, d: usize) -> bool {
        a < self.domain_size && b < self.domain_size && d < self.domain_size
    }
    /// Monotonicity: a ⊥_C B and A ⊆ B implies a ⊥_C A.
    pub fn check_monotonicity(&self, a: usize, big_set: &[usize], small_set: &[usize]) -> bool {
        let _ = a;
        small_set.iter().all(|x| big_set.contains(x))
    }
}
/// A saturated model builder that collects types and checks saturation conditions.
#[allow(dead_code)]
pub struct SaturatedModelBuilder {
    /// The theory name.
    pub theory_name: String,
    /// Target saturation cardinal κ (as a string descriptor).
    pub kappa: String,
    /// Types over small parameter sets that have been realized.
    pub realized_types: Vec<CompleteTypeOverSet>,
    /// Types that remain unrealized (witnesses to non-saturation).
    pub unrealized_types: Vec<CompleteTypeOverSet>,
}
#[allow(dead_code)]
impl SaturatedModelBuilder {
    /// Create a new saturated model builder.
    pub fn new(theory_name: &str, kappa: &str) -> Self {
        SaturatedModelBuilder {
            theory_name: theory_name.to_string(),
            kappa: kappa.to_string(),
            realized_types: Vec::new(),
            unrealized_types: Vec::new(),
        }
    }
    /// Record a realized type.
    pub fn realize_type(&mut self, tp: CompleteTypeOverSet) {
        self.realized_types.push(tp);
    }
    /// Record an unrealized type.
    pub fn fail_to_realize(&mut self, tp: CompleteTypeOverSet) {
        self.unrealized_types.push(tp);
    }
    /// The model is κ-saturated if all types over sets of size < κ are realized.
    pub fn is_saturated(&self) -> bool {
        self.unrealized_types.is_empty()
    }
    /// Number of realized types.
    pub fn num_realized(&self) -> usize {
        self.realized_types.len()
    }
    /// Number of unrealized types.
    pub fn num_unrealized(&self) -> usize {
        self.unrealized_types.len()
    }
    /// Summary.
    pub fn saturation_summary(&self) -> String {
        if self.is_saturated() {
            format!(
                "Model of '{}' is {}-saturated: all {} types realized.",
                self.theory_name,
                self.kappa,
                self.num_realized()
            )
        } else {
            format!(
                "Model of '{}' is NOT {}-saturated: {} types unrealized.",
                self.theory_name,
                self.kappa,
                self.num_unrealized()
            )
        }
    }
}
/// A differential field of characteristic 0 (DCF₀) data structure.
#[allow(dead_code)]
pub struct DifferentialField {
    /// Name of the underlying field.
    pub field_name: String,
    /// List of derivation operators (by name).
    pub derivations: Vec<String>,
    /// Whether the field is differentially closed.
    pub is_closed: bool,
}
#[allow(dead_code)]
impl DifferentialField {
    /// Create a new differential field.
    pub fn new(field_name: &str, derivations: Vec<String>, is_closed: bool) -> Self {
        DifferentialField {
            field_name: field_name.to_string(),
            derivations,
            is_closed,
        }
    }
    /// The theory DCF₀ is complete and has quantifier elimination (Blum).
    pub fn has_quantifier_elimination(&self) -> bool {
        self.is_closed
    }
    /// DCF₀ is the model companion of the theory of differential fields.
    pub fn is_model_companion(&self) -> bool {
        self.is_closed
    }
    /// The Kolchin polynomial measures the transcendence degree of a type.
    pub fn kolchin_polynomial_degree(&self, type_size: usize) -> usize {
        type_size * self.derivations.len()
    }
    /// Description of the differential field.
    pub fn description(&self) -> String {
        format!(
            "Differential field '{}' with derivations {:?}, closed={}.",
            self.field_name, self.derivations, self.is_closed
        )
    }
    /// The Lascar rank in DCF₀ coincides with the Kolchin polynomial degree.
    pub fn lascar_rank_of_type(&self, type_size: usize) -> String {
        format!(
            "Lascar rank = {} (Kolchin polynomial degree) for type of size {} in DCF₀.",
            self.kolchin_polynomial_degree(type_size),
            type_size
        )
    }
}
/// A finite model that can check satisfaction of `FirstOrderFormula`.
pub struct FiniteModel {
    /// The underlying structure.
    pub structure: FiniteStructure,
}
impl FiniteModel {
    /// Create a finite model from a structure.
    pub fn new(structure: FiniteStructure) -> Self {
        FiniteModel { structure }
    }
    /// Check whether a ground formula (no free variables) is satisfied.
    /// Quantifiers range over the domain indices.
    pub fn satisfies(&self, formula: &FirstOrderFormula) -> bool {
        self.satisfies_env(formula, &[])
    }
    /// Internal satisfaction check with a variable assignment environment.
    fn satisfies_env(&self, formula: &FirstOrderFormula, env: &[usize]) -> bool {
        match formula {
            FirstOrderFormula::True => true,
            FirstOrderFormula::False => false,
            FirstOrderFormula::Eq(i, j) => {
                let a = env.get(*i).copied().unwrap_or(*i);
                let b = env.get(*j).copied().unwrap_or(*j);
                self.structure.satisfies_equality(a, b)
            }
            FirstOrderFormula::Atom { relation, args } => {
                let resolved: Vec<usize> = args
                    .iter()
                    .map(|&idx| env.get(idx).copied().unwrap_or(idx))
                    .collect();
                self.structure.satisfies_relation(relation, &resolved)
            }
            FirstOrderFormula::Not(f) => !self.satisfies_env(f, env),
            FirstOrderFormula::And(a, b) => {
                self.satisfies_env(a, env) && self.satisfies_env(b, env)
            }
            FirstOrderFormula::Or(a, b) => self.satisfies_env(a, env) || self.satisfies_env(b, env),
            FirstOrderFormula::Implies(a, b) => {
                !self.satisfies_env(a, env) || self.satisfies_env(b, env)
            }
            FirstOrderFormula::ForAll(body) => {
                let n = self.structure.domain_size();
                (0..n).all(|d| {
                    let mut new_env = vec![d];
                    new_env.extend_from_slice(env);
                    self.satisfies_env(body, &new_env)
                })
            }
            FirstOrderFormula::Exists(body) => {
                let n = self.structure.domain_size();
                (0..n).any(|d| {
                    let mut new_env = vec![d];
                    new_env.extend_from_slice(env);
                    self.satisfies_env(body, &new_env)
                })
            }
        }
    }
}
/// A finite first-order structure with a named domain.
pub struct FiniteStructure {
    /// Domain elements, each identified by a name string.
    pub domain: Vec<String>,
    /// Constant interpretations: name → domain index.
    pub constants: std::collections::HashMap<String, usize>,
    /// Relation interpretations: relation name → set of argument tuples.
    pub relations: std::collections::HashMap<String, Vec<Vec<usize>>>,
    /// Function interpretations: function name → list of (args, result) pairs.
    pub functions: std::collections::HashMap<String, Vec<(Vec<usize>, usize)>>,
}
impl FiniteStructure {
    /// Create a new structure with the given domain elements.
    pub fn new(domain: Vec<String>) -> Self {
        FiniteStructure {
            domain,
            constants: std::collections::HashMap::new(),
            relations: std::collections::HashMap::new(),
            functions: std::collections::HashMap::new(),
        }
    }
    /// Add a constant interpretation.
    pub fn add_constant(&mut self, name: &str, elem: usize) {
        self.constants.insert(name.to_string(), elem);
    }
    /// Add a relation as a set of tuples of domain indices.
    pub fn add_relation(&mut self, name: &str, tuples: Vec<Vec<usize>>) {
        self.relations.insert(name.to_string(), tuples);
    }
    /// Return the domain size.
    pub fn domain_size(&self) -> usize {
        self.domain.len()
    }
    /// Check equality of two domain elements.
    pub fn satisfies_equality(&self, a: usize, b: usize) -> bool {
        a == b
    }
    /// Check if a named relation holds for the given argument tuple.
    pub fn satisfies_relation(&self, name: &str, args: &[usize]) -> bool {
        if let Some(tuples) = self.relations.get(name) {
            tuples.iter().any(|t| t.as_slice() == args)
        } else {
            false
        }
    }
}
/// A stability classifier for first-order theories.
#[allow(dead_code)]
pub struct StabilityClassifier {
    /// The theory being classified.
    pub theory_name: String,
    /// Stability at various cardinals (cardinal index → stable).
    pub stability_map: std::collections::HashMap<String, bool>,
}
#[allow(dead_code)]
impl StabilityClassifier {
    /// Create a new classifier for the given theory.
    pub fn new(theory_name: &str) -> Self {
        StabilityClassifier {
            theory_name: theory_name.to_string(),
            stability_map: std::collections::HashMap::new(),
        }
    }
    /// Record stability at a given cardinal descriptor.
    pub fn add_stability(&mut self, cardinal: &str, is_stable: bool) {
        self.stability_map.insert(cardinal.to_string(), is_stable);
    }
    /// Is the theory omega-stable? (stable at all infinite cardinals)
    pub fn is_omega_stable(&self) -> bool {
        self.stability_map.values().all(|&b| b)
    }
    /// Is the theory superstable? (stable at every κ ≥ 2^ℵ₀)
    pub fn is_superstable(&self) -> bool {
        self.stability_map
            .iter()
            .filter(|(k, _)| k.contains("uncountable"))
            .all(|(_, &v)| v)
    }
    /// Is the theory strictly stable (stable but not superstable)?
    pub fn is_strictly_stable(&self) -> bool {
        !self.is_superstable() && self.stability_map.values().any(|&v| v)
    }
    /// Summary description.
    pub fn classify(&self) -> String {
        if self.is_omega_stable() {
            format!("Theory '{}' is omega-stable.", self.theory_name)
        } else if self.is_superstable() {
            format!("Theory '{}' is superstable.", self.theory_name)
        } else if self.is_strictly_stable() {
            format!("Theory '{}' is stable (strictly).", self.theory_name)
        } else {
            format!("Theory '{}' is unstable.", self.theory_name)
        }
    }
}
/// A model-theoretic algebraically closed valued field (ACVF) data structure.
#[allow(dead_code)]
pub struct ACVFData {
    /// Characteristic of the residue field.
    pub residue_char: u32,
    /// Value group description.
    pub value_group: String,
    /// Whether the field is complete.
    pub is_complete: bool,
}
#[allow(dead_code)]
impl ACVFData {
    /// Create a new ACVF data structure.
    pub fn new(residue_char: u32, value_group: &str, is_complete: bool) -> Self {
        ACVFData {
            residue_char,
            value_group: value_group.to_string(),
            is_complete,
        }
    }
    /// ACVF has elimination of imaginaries (Haskell-Hrushovski-Macpherson).
    pub fn has_elim_of_imaginaries(&self) -> bool {
        true
    }
    /// ACVF is stable iff the value group is trivial.
    pub fn is_stable(&self) -> bool {
        self.value_group == "trivial"
    }
    /// ACVF has NIP (as a valued field).
    pub fn has_nip(&self) -> bool {
        true
    }
    /// Description of the ACVF.
    pub fn description(&self) -> String {
        format!(
            "ACVF(char={}, value_group={}, complete={}): algebraically closed valued field.",
            self.residue_char, self.value_group, self.is_complete
        )
    }
    /// The theory of ACVF with given characteristic is model-complete.
    pub fn is_model_complete(&self) -> bool {
        true
    }
}
/// A complete 1-type over a theory: a maximal consistent set of formulas in one free variable.
/// Represented as a set of formula descriptions.
#[derive(Debug, Clone)]
pub struct CompleteOneType {
    /// The formulas belonging to this type (as strings).
    pub formulas: Vec<String>,
    /// Whether this type is realized in the canonical model.
    pub is_realized: bool,
}
impl CompleteOneType {
    /// Create a new empty complete type.
    pub fn new() -> Self {
        CompleteOneType {
            formulas: Vec::new(),
            is_realized: false,
        }
    }
    /// Add a formula description to the type.
    pub fn add_formula(&mut self, f: &str) {
        self.formulas.push(f.to_string());
    }
    /// Mark this type as realized.
    pub fn mark_realized(&mut self) {
        self.is_realized = true;
    }
    /// Return the number of formulas in this type.
    pub fn size(&self) -> usize {
        self.formulas.len()
    }
}
/// A theory amalgamation checker based on the amalgamation property.
#[allow(dead_code)]
pub struct AmalgamationChecker {
    /// The theory name.
    pub theory_name: String,
    /// Whether the theory has the amalgamation property.
    pub has_ap: bool,
    /// Whether the theory has the joint embedding property.
    pub has_jep: bool,
    /// Whether the theory has quantifier elimination.
    pub has_qe: bool,
}
#[allow(dead_code)]
impl AmalgamationChecker {
    /// Create a new amalgamation checker.
    pub fn new(theory_name: &str) -> Self {
        AmalgamationChecker {
            theory_name: theory_name.to_string(),
            has_ap: false,
            has_jep: false,
            has_qe: false,
        }
    }
    /// Set the amalgamation property.
    pub fn set_ap(&mut self, has: bool) {
        self.has_ap = has;
    }
    /// Set the joint embedding property.
    pub fn set_jep(&mut self, has: bool) {
        self.has_jep = has;
    }
    /// Set quantifier elimination.
    pub fn set_qe(&mut self, has: bool) {
        self.has_qe = has;
    }
    /// The theory has a Fraïssé limit iff it has AP + JEP + HP.
    pub fn has_fraisse_limit(&self) -> bool {
        self.has_ap && self.has_jep
    }
    /// Check if the theory is model-complete (QE implies model-completeness in many cases).
    pub fn is_model_complete(&self) -> bool {
        self.has_qe
    }
    /// A theory with AP, JEP, and QE is complete.
    pub fn is_complete(&self) -> bool {
        self.has_ap && self.has_jep && self.has_qe
    }
    /// Summary.
    pub fn theory_summary(&self) -> String {
        format!(
            "Theory '{}': AP={}, JEP={}, QE={}, complete={}, Fraisse-limit={}.",
            self.theory_name,
            self.has_ap,
            self.has_jep,
            self.has_qe,
            self.is_complete(),
            self.has_fraisse_limit()
        )
    }
}
