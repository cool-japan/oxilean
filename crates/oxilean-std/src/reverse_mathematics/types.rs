//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

/// A coloring of pairs from {0..n} with k colors, used for Ramsey experiments.
#[derive(Debug, Clone)]
pub struct RamseyColoringFinder {
    /// Number of vertices.
    pub n: usize,
    /// Number of colors.
    pub k: u32,
    /// coloring[i][j] = color of the pair {i, j}, for i < j.
    pub coloring: Vec<Vec<u32>>,
}
impl RamseyColoringFinder {
    /// Create a new all-zero coloring on n vertices.
    pub fn new_uniform(n: usize, k: u32) -> Self {
        let coloring = vec![vec![0u32; n]; n];
        Self { n, k, coloring }
    }
    /// Set the color of pair {i, j} (i < j enforced by sorting).
    pub fn set_color(&mut self, i: usize, j: usize, color: u32) {
        let (lo, hi) = if i < j { (i, j) } else { (j, i) };
        if hi < self.n && color < self.k {
            self.coloring[lo][hi] = color;
        }
    }
    /// Get the color of pair {i, j}.
    pub fn get_color(&self, i: usize, j: usize) -> u32 {
        let (lo, hi) = if i < j { (i, j) } else { (j, i) };
        self.coloring
            .get(lo)
            .and_then(|r| r.get(hi))
            .copied()
            .unwrap_or(0)
    }
    /// Find the largest monochromatic clique for a specific color.
    pub fn monochromatic_clique(&self, color: u32) -> Vec<usize> {
        let mut best: Vec<usize> = vec![];
        for start in 0..self.n {
            let mut clique = vec![start];
            for v in (start + 1)..self.n {
                if clique.iter().all(|&u| self.get_color(u, v) == color) {
                    clique.push(v);
                }
            }
            if clique.len() > best.len() {
                best = clique;
            }
        }
        best
    }
    /// Return the largest monochromatic clique across all colors.
    pub fn best_monochromatic_clique(&self) -> (u32, Vec<usize>) {
        let mut best_color = 0u32;
        let mut best_clique: Vec<usize> = vec![];
        for c in 0..self.k {
            let clique = self.monochromatic_clique(c);
            if clique.len() > best_clique.len() {
                best_color = c;
                best_clique = clique;
            }
        }
        (best_color, best_clique)
    }
}
/// Omega-models and their properties.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OmegaModel {
    /// Name of the model (e.g., "ω-model of ACA₀").
    pub name: String,
    /// Which subsystem this is a model of.
    pub satisfies: String,
    /// Whether this is a minimal omega-model.
    pub is_minimal: bool,
    /// Description.
    pub description: String,
}
impl OmegaModel {
    #[allow(dead_code)]
    pub fn new(
        name: impl Into<String>,
        satisfies: impl Into<String>,
        is_minimal: bool,
        description: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            satisfies: satisfies.into(),
            is_minimal,
            description: description.into(),
        }
    }
    #[allow(dead_code)]
    pub fn standard_aca0() -> Self {
        Self::new(
            "M_0",
            "ACA₀",
            true,
            "The minimal ω-model of ACA₀: all sets arithmetically definable in N",
        )
    }
    #[allow(dead_code)]
    pub fn rec_sets() -> Self {
        Self::new(
            "REC",
            "RCA₀",
            true,
            "The minimal ω-model of RCA₀: all recursive (computable) sets",
        )
    }
    #[allow(dead_code)]
    pub fn description_str(&self) -> String {
        format!(
            "OmegaModel {{ name: {}, satisfies: {}, minimal: {}, desc: {} }}",
            self.name, self.satisfies, self.is_minimal, self.description
        )
    }
}
/// Summary statistics about the Big Five systems.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BigFiveStats {
    /// Number of "classical" theorems provable in each system.
    pub rca0_count: usize,
    pub wkl0_count: usize,
    pub aca0_count: usize,
    pub atr0_count: usize,
    pub pi11ca0_count: usize,
}
impl BigFiveStats {
    #[allow(dead_code)]
    pub fn from_scoreboard(sb: &RMScoreboard) -> Self {
        Self {
            rca0_count: sb.count_in("RCA₀"),
            wkl0_count: sb.count_in("WKL₀"),
            aca0_count: sb.count_in("ACA₀"),
            atr0_count: sb.count_in("ATR₀"),
            pi11ca0_count: sb.count_in("Π¹₁-CA₀"),
        }
    }
    #[allow(dead_code)]
    pub fn total(&self) -> usize {
        self.rca0_count + self.wkl0_count + self.aca0_count + self.atr0_count + self.pi11ca0_count
    }
    #[allow(dead_code)]
    pub fn display(&self) -> String {
        format!(
            "RCA₀:{} WKL₀:{} ACA₀:{} ATR₀:{} Π¹₁-CA₀:{} total:{}",
            self.rca0_count,
            self.wkl0_count,
            self.aca0_count,
            self.atr0_count,
            self.pi11ca0_count,
            self.total()
        )
    }
}
/// The Big Five subsystems of second-order arithmetic, ordered by strength.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BigFiveSystem {
    /// Recursive Comprehension Axiom (weakest).
    RCA0,
    /// Weak König's Lemma.
    WKL0,
    /// Arithmetical Comprehension Axiom.
    ACA0,
    /// Arithmetical Transfinite Recursion.
    ATR0,
    /// Π¹_1 Comprehension Axiom (strongest).
    Pi11CA0,
}
impl BigFiveSystem {
    /// Human-readable name of the system.
    pub fn name(&self) -> &'static str {
        match self {
            BigFiveSystem::RCA0 => "RCA₀",
            BigFiveSystem::WKL0 => "WKL₀",
            BigFiveSystem::ACA0 => "ACA₀",
            BigFiveSystem::ATR0 => "ATR₀",
            BigFiveSystem::Pi11CA0 => "Π¹₁-CA₀",
        }
    }
    /// Is this system stronger than or equal to the other?
    pub fn at_least_as_strong_as(&self, other: &BigFiveSystem) -> bool {
        self >= other
    }
    /// The corresponding proof-theoretic ordinal (Bachmann–Howard notation as string).
    pub fn proof_theoretic_ordinal(&self) -> &'static str {
        match self {
            BigFiveSystem::RCA0 => "ω^ω",
            BigFiveSystem::WKL0 => "ω^ω",
            BigFiveSystem::ACA0 => "ε₀",
            BigFiveSystem::ATR0 => "Γ₀",
            BigFiveSystem::Pi11CA0 => "ψ(Ω_ω)",
        }
    }
    /// Returns `true` if WKL₀ is Π¹_1-conservative over this system.
    /// This holds precisely for RCA₀.
    pub fn wkl0_is_conservative_over(&self) -> bool {
        matches!(self, BigFiveSystem::RCA0)
    }
}
/// A conservation result: `stronger` is `formula_class`-conservative over `weaker`.
#[derive(Debug, Clone)]
pub struct ConservationResult {
    /// The stronger system (e.g. WKL₀).
    pub stronger: BigFiveSystem,
    /// The weaker system (e.g. RCA₀).
    pub weaker: BigFiveSystem,
    /// The class of formulas for which conservation holds.
    pub formula_class: &'static str,
    /// Authors and year of result.
    pub reference: &'static str,
}
impl ConservationResult {
    /// WKL₀ is Π¹_1-conservative over RCA₀ (Friedman 1976, Simpson).
    pub fn wkl0_over_rca0() -> Self {
        Self {
            stronger: BigFiveSystem::WKL0,
            weaker: BigFiveSystem::RCA0,
            formula_class: "Π¹_1",
            reference: "Friedman 1976; Simpson 1999",
        }
    }
    /// ACA₀ is conservative over PA for first-order sentences (well-known).
    pub fn aca0_over_pa() -> Self {
        Self {
            stronger: BigFiveSystem::ACA0,
            weaker: BigFiveSystem::RCA0,
            formula_class: "First-order arithmetic",
            reference: "Friedman 1976; Simpson Ch. III",
        }
    }
    /// ATR₀ is Π¹_2-conservative over ACA₀.
    pub fn atr0_over_aca0() -> Self {
        Self {
            stronger: BigFiveSystem::ATR0,
            weaker: BigFiveSystem::ACA0,
            formula_class: "Π¹_2",
            reference: "Friedman 1976",
        }
    }
    /// Verify the direction: stronger ≥ weaker in the Big Five ordering.
    pub fn is_valid_direction(&self) -> bool {
        self.stronger >= self.weaker
    }
}
/// Represents a Π¹₁ sentence and its known proof-theoretic strength.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Pi11Sentence {
    /// Name of the statement.
    pub name: String,
    /// The statement in informal mathematics.
    pub statement: String,
    /// Known ordinal proof-theoretic strength.
    pub ordinal_strength: Option<String>,
}
impl Pi11Sentence {
    #[allow(dead_code)]
    pub fn new(
        name: impl Into<String>,
        statement: impl Into<String>,
        ordinal_strength: Option<impl Into<String>>,
    ) -> Self {
        Self {
            name: name.into(),
            statement: statement.into(),
            ordinal_strength: ordinal_strength.map(Into::into),
        }
    }
    #[allow(dead_code)]
    pub fn display(&self) -> String {
        let ord = self.ordinal_strength.as_deref().unwrap_or("unknown");
        format!("{}: {} [strength: {}]", self.name, self.statement, ord)
    }
}
/// A combinatorial principle in the RM zoo.
#[derive(Debug, Clone)]
pub struct RMPrinciple {
    /// Short name (e.g. "RT²_2").
    pub name: &'static str,
    /// Long description.
    pub description: &'static str,
    /// Known RM strength.
    pub strength: RMStrength,
    /// Year first studied.
    pub year: u32,
}
impl RMPrinciple {
    /// Return true if this principle is equivalent (over RCA₀) to the given system.
    pub fn equivalent_to(&self, system: &BigFiveSystem) -> bool {
        match (&self.strength, system) {
            (RMStrength::WKL0, BigFiveSystem::WKL0) => true,
            (RMStrength::ACA0, BigFiveSystem::ACA0) => true,
            (RMStrength::ATR0, BigFiveSystem::ATR0) => true,
            (RMStrength::Pi11CA0, BigFiveSystem::Pi11CA0) => true,
            (RMStrength::RCA0, BigFiveSystem::RCA0) => true,
            _ => false,
        }
    }
    /// Ramsey's theorem RT²_2: pairs, 2 colors.
    pub fn rt22() -> Self {
        Self {
            name: "RT²_2",
            description: "Ramsey's theorem for pairs and 2 colors",
            strength: RMStrength::BetweenWKL0AndACA0,
            year: 1995,
        }
    }
    /// Stable Ramsey's theorem SRT²_2.
    pub fn srt22() -> Self {
        Self {
            name: "SRT²_2",
            description: "Stable Ramsey's theorem for pairs and 2 colors",
            strength: RMStrength::BetweenRCA0AndWKL0,
            year: 2001,
        }
    }
    /// Chain-Antichain principle CAC (Dilworth).
    pub fn cac() -> Self {
        Self {
            name: "CAC",
            description: "Every infinite partial order has an infinite chain or antichain",
            strength: RMStrength::BetweenWKL0AndACA0,
            year: 2007,
        }
    }
    /// Ascending/Descending Sequence ADS.
    pub fn ads() -> Self {
        Self {
            name: "ADS",
            description: "Every infinite linear order has an infinite ascending or descending seq",
            strength: RMStrength::BetweenRCA0AndWKL0,
            year: 2007,
        }
    }
    /// Hindman's theorem.
    pub fn hindman() -> Self {
        Self {
            name: "HT",
            description: "Hindman's finite sums theorem",
            strength: RMStrength::BetweenACA0AndATR0,
            year: 1974,
        }
    }
    /// Bolzano–Weierstrass theorem.
    pub fn bolzano_weierstrass() -> Self {
        Self {
            name: "BW",
            description: "Every bounded sequence of reals has a convergent subsequence",
            strength: RMStrength::ACA0,
            year: 1985,
        }
    }
    /// Brouwer's fixed-point theorem.
    pub fn brouwer() -> Self {
        Self {
            name: "BFP",
            description: "Brouwer's fixed-point theorem for the closed disk",
            strength: RMStrength::WKL0,
            year: 1998,
        }
    }
}
/// Bishop-style constructive principles and their RM strengths.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ConstructivePrinciple {
    /// Name of the principle.
    pub name: String,
    /// The classical theorem it corresponds to.
    pub classical_counterpart: String,
    /// Whether this is constructively provable (Bishop-style).
    pub constructively_provable: bool,
    /// Required axiom for constructive proof (if any).
    pub required_axiom: Option<String>,
}
impl ConstructivePrinciple {
    #[allow(dead_code)]
    pub fn new(
        name: impl Into<String>,
        classical: impl Into<String>,
        constructive: bool,
        axiom: Option<impl Into<String>>,
    ) -> Self {
        Self {
            name: name.into(),
            classical_counterpart: classical.into(),
            constructively_provable: constructive,
            required_axiom: axiom.map(Into::into),
        }
    }
    #[allow(dead_code)]
    pub fn display(&self) -> String {
        let ax = self.required_axiom.as_deref().unwrap_or("none");
        format!(
            "Principle '{}' (classical: '{}') constructive={}, axiom={}",
            self.name, self.classical_counterpart, self.constructively_provable, ax
        )
    }
}
/// Represents a formal proof system in reverse mathematics.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProofSystem {
    /// Primitive Recursive Arithmetic (PRA).
    PRA,
    /// Elementary Recursive Arithmetic (ECA).
    ECA,
    /// Robinson Arithmetic (Q).
    RobinsonQ,
    /// Peano Arithmetic (PA).
    PeanoPA,
    /// Second-Order Arithmetic (Z₂).
    Z2,
    /// A custom-named system.
    Custom(String),
}
impl ProofSystem {
    #[allow(dead_code)]
    pub fn name(&self) -> String {
        match self {
            Self::PRA => "PRA".to_owned(),
            Self::ECA => "ECA".to_owned(),
            Self::RobinsonQ => "Q".to_owned(),
            Self::PeanoPA => "PA".to_owned(),
            Self::Z2 => "Z₂".to_owned(),
            Self::Custom(s) => s.clone(),
        }
    }
    /// Returns `true` if `other` is provably a conservative extension of `self`.
    #[allow(dead_code)]
    pub fn is_conservative_over(&self, other: &ProofSystem) -> bool {
        use ProofSystem::*;
        matches!(
            (self, other),
            (Z2, PeanoPA)
                | (PeanoPA, RobinsonQ)
                | (Z2, RobinsonQ)
                | (PeanoPA, ECA)
                | (Z2, ECA)
                | (PeanoPA, PRA)
                | (Z2, PRA)
        )
    }
    /// Returns the set of systems stronger than (or equal to) `self`.
    #[allow(dead_code)]
    pub fn stronger_systems(&self) -> Vec<ProofSystem> {
        use ProofSystem::*;
        match self {
            PRA => vec![PRA, ECA, RobinsonQ, PeanoPA, Z2],
            ECA => vec![ECA, RobinsonQ, PeanoPA, Z2],
            RobinsonQ => vec![RobinsonQ, PeanoPA, Z2],
            PeanoPA => vec![PeanoPA, Z2],
            Z2 => vec![Z2],
            Custom(_) => vec![self.clone()],
        }
    }
}
/// A finite approximation to an infinite binary tree (König / WKL₀).
/// Nodes are stored as bit-strings of bounded length.
#[derive(Debug, Clone)]
pub struct WeakKonigTree {
    /// Maximum depth of stored nodes.
    pub max_depth: u32,
    /// The set of nodes (as `u64` bit-strings of length ≤ max_depth).
    pub nodes: Vec<u64>,
    /// The length of each node bit-string (parallel to `nodes`).
    pub depths: Vec<u32>,
}
impl WeakKonigTree {
    /// Create a complete binary tree of depth `d`.
    pub fn complete(d: u32) -> Self {
        let mut nodes = Vec::new();
        let mut depths = Vec::new();
        for depth in 0..=d {
            for bits in 0u64..(1u64 << depth) {
                nodes.push(bits);
                depths.push(depth);
            }
        }
        Self {
            max_depth: d,
            nodes,
            depths,
        }
    }
    /// Return the number of nodes at depth `d`.
    pub fn count_at_depth(&self, d: u32) -> usize {
        self.depths.iter().filter(|&&depth| depth == d).count()
    }
    /// Return true if the tree is infinite (has nodes at every depth 0..=max_depth).
    pub fn is_infinite(&self) -> bool {
        (0..=self.max_depth).all(|d| self.count_at_depth(d) > 0)
    }
    /// Greedily extract an infinite path (always extend with bit 0 if possible, else 1).
    /// Returns the sequence of bits representing the path.
    pub fn greedy_path(&self) -> Vec<u8> {
        let mut path = Vec::new();
        let mut prefix: u64 = 0;
        for depth in 1..=self.max_depth {
            let zero_child = prefix;
            let one_child = prefix | (1u64 << (depth - 1));
            let has_zero = self
                .nodes
                .iter()
                .zip(self.depths.iter())
                .any(|(&n, &d)| d == depth && n == zero_child);
            let has_one = self
                .nodes
                .iter()
                .zip(self.depths.iter())
                .any(|(&n, &d)| d == depth && n == one_child);
            if has_zero {
                prefix = zero_child;
                path.push(0u8);
            } else if has_one {
                prefix = one_child;
                path.push(1u8);
            } else {
                break;
            }
        }
        path
    }
}
/// A tagged statement together with the RCA₀ axiom kind it falls under.
#[derive(Debug, Clone)]
pub struct RMA0System {
    /// Human-readable description of the statement.
    pub statement: &'static str,
    /// The axiom kind.
    pub kind: RCA0AxiomKind,
    /// Whether this instance is valid (for illustrative purposes).
    pub is_valid: bool,
}
impl RMA0System {
    /// Verify that this is a valid RCA₀ axiom instance.
    pub fn verify(&self) -> bool {
        self.is_valid
    }
    /// Return a human-readable summary.
    pub fn summary(&self) -> String {
        format!(
            "[{:?}] {} — {}",
            self.kind,
            self.statement,
            if self.is_valid { "VALID" } else { "INVALID" }
        )
    }
    /// Produce the standard Σ⁰_1-comprehension instance for a given predicate name.
    pub fn sigma01_comprehension_for(predicate: &'static str) -> Self {
        Self {
            statement: predicate,
            kind: RCA0AxiomKind::Sigma01Comprehension,
            is_valid: true,
        }
    }
}
/// One level in the RM hierarchy.
#[derive(Debug, Clone)]
pub struct RMHierarchyEntry {
    /// The Big Five system.
    pub system: BigFiveSystem,
    /// Representative equivalent principles (names only).
    pub equivalents: Vec<&'static str>,
    /// Key conservation result for this level.
    pub conservation_note: &'static str,
}
/// A theorem statement together with the minimal subsystem that proves it.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RMTheorem {
    /// Name of the theorem.
    pub name: String,
    /// Informal statement.
    pub statement: String,
    /// The minimal Big-Five system that proves the theorem.
    pub minimal_system: String,
    /// Known equivalences with other theorems over RCA₀.
    pub equivalences: Vec<String>,
    /// Is the theorem provable in RCA₀ itself (not just equivalent over it)?
    pub provable_in_rca0: bool,
}
impl RMTheorem {
    /// Construct a new RM theorem record.
    #[allow(dead_code)]
    pub fn new(
        name: impl Into<String>,
        statement: impl Into<String>,
        minimal_system: impl Into<String>,
        equivalences: Vec<impl Into<String>>,
        provable_in_rca0: bool,
    ) -> Self {
        Self {
            name: name.into(),
            statement: statement.into(),
            minimal_system: minimal_system.into(),
            equivalences: equivalences.into_iter().map(Into::into).collect(),
            provable_in_rca0,
        }
    }
    /// Returns a formatted summary.
    #[allow(dead_code)]
    pub fn summary(&self) -> String {
        let equiv_str = if self.equivalences.is_empty() {
            "none known".to_owned()
        } else {
            self.equivalences.join(", ")
        };
        format!(
            "[{}] Minimal system: {}. Equivalences: {}. Provable in RCA₀: {}.",
            self.name, self.minimal_system, equiv_str, self.provable_in_rca0
        )
    }
}
/// Enumeration of formal independence results in set theory.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct IndependenceResult {
    /// Name of the statement.
    pub statement: String,
    /// Base theory (e.g., "ZFC").
    pub base_theory: String,
    /// True = independent; False = provable or disprovable.
    pub is_independent: bool,
    /// Notes on the independence result.
    pub notes: String,
}
impl IndependenceResult {
    #[allow(dead_code)]
    pub fn new(
        statement: impl Into<String>,
        base: impl Into<String>,
        ind: bool,
        notes: impl Into<String>,
    ) -> Self {
        Self {
            statement: statement.into(),
            base_theory: base.into(),
            is_independent: ind,
            notes: notes.into(),
        }
    }
    #[allow(dead_code)]
    pub fn display(&self) -> String {
        let kind = if self.is_independent {
            "INDEPENDENT"
        } else {
            "PROVABLE"
        };
        format!(
            "[{}] {} over {} ({})",
            kind, self.statement, self.base_theory, self.notes
        )
    }
}
/// A Turing machine oracle that answers membership queries for a set X ⊆ ℕ.
/// Used to model oracle-relative computability in the RM hierarchy.
#[derive(Debug, Clone)]
pub struct ComputableFunction {
    /// A description / name for this function (e.g. "Halting problem oracle").
    pub name: &'static str,
    /// Degree information: `Some(k)` means the function has degree 0^(k) (k-th jump).
    pub jump_level: Option<u32>,
    /// The function table for inputs 0..table.len() (partial; `None` = diverges).
    pub table: Vec<Option<u64>>,
}
impl ComputableFunction {
    /// Create the characteristic function of the set {n | n < bound} (computable).
    pub fn indicator_below(bound: u64) -> Self {
        let table = (0u64..bound + 4)
            .map(|n| Some(if n < bound { 1 } else { 0 }))
            .collect();
        Self {
            name: "indicator_below",
            jump_level: Some(0),
            table,
        }
    }
    /// Evaluate at input n, returning `None` if the function diverges.
    pub fn eval(&self, n: usize) -> Option<u64> {
        self.table.get(n).copied().flatten()
    }
    /// Return true if this function is total on 0..n.
    pub fn is_total_up_to(&self, n: usize) -> bool {
        (0..n).all(|i| self.table.get(i).copied().flatten().is_some())
    }
    /// Simulate the n-th step of a Turing machine oracle computation (stub).
    /// In a full implementation this would run a UTM; here we return the table value.
    pub fn oracle_query(&self, input: usize, _steps: u32) -> Option<u64> {
        self.eval(input)
    }
}
/// The Friedman-style reverse mathematics scoreboard.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct RMScoreboard {
    pub theorems: Vec<(String, String)>,
}
impl RMScoreboard {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn record(&mut self, theorem: impl Into<String>, system: impl Into<String>) {
        self.theorems.push((theorem.into(), system.into()));
    }
    #[allow(dead_code)]
    pub fn theorems_in(&self, system: &str) -> Vec<&str> {
        self.theorems
            .iter()
            .filter(|(_, s)| s == system)
            .map(|(t, _)| t.as_str())
            .collect()
    }
    #[allow(dead_code)]
    pub fn count_in(&self, system: &str) -> usize {
        self.theorems_in(system).len()
    }
    #[allow(dead_code)]
    pub fn standard() -> Self {
        let mut sb = Self::new();
        for thm in standard_rm_theorems() {
            sb.record(thm.name.clone(), thm.minimal_system.clone());
        }
        sb
    }
}
/// A fragment axiom verifier for RCA₀.
/// Checks whether a given closed formula (represented as a tag) is an instance
/// of one of the axiom schemes of RCA₀.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RCA0AxiomKind {
    /// Σ⁰_1 comprehension: {n | φ(n)} exists for Σ⁰_1 φ.
    Sigma01Comprehension,
    /// Σ⁰_1 induction: φ(0) ∧ ∀n(φ(n)→φ(n+1)) → ∀n φ(n) for Σ⁰_1 φ.
    Sigma01Induction,
    /// Basic arithmetic axioms (successor, addition, multiplication).
    BasicArithmetic,
    /// Recursive definition (primitive recursion).
    PrimitiveRecursion,
}
/// Display the RM hierarchy: Big Five systems with proof-theoretic ordinals
/// and key equivalent principles.
#[derive(Debug, Clone)]
pub struct RMHierarchy {
    entries: Vec<RMHierarchyEntry>,
}
impl RMHierarchy {
    /// Build the standard Big Five hierarchy with known equivalents.
    pub fn standard() -> Self {
        Self {
            entries: vec![
                RMHierarchyEntry {
                    system: BigFiveSystem::RCA0,
                    equivalents: vec![
                        "Σ⁰₁-comprehension",
                        "Σ⁰₁-induction",
                        "Primitive recursion",
                        "Low basis theorem",
                        "Σ⁰₁-bounding",
                    ],
                    conservation_note: "Base system; WKL₀ is Π¹₁-conservative over RCA₀",
                },
                RMHierarchyEntry {
                    system: BigFiveSystem::WKL0,
                    equivalents: vec![
                        "König's lemma (binary trees)",
                        "Brouwer's fixed-point theorem",
                        "Hahn-Banach theorem",
                        "Gödel completeness theorem",
                        "Maximal ideal theorem",
                        "Jordan curve theorem",
                    ],
                    conservation_note: "Π¹₁-conservative over RCA₀ (Friedman 1976)",
                },
                RMHierarchyEntry {
                    system: BigFiveSystem::ACA0,
                    equivalents: vec![
                        "Bolzano-Weierstrass theorem",
                        "Arithmetical comprehension",
                        "Sequential completeness of ℝ",
                        "Comparability of well-orderings",
                        "König's lemma (finitely branching)",
                        "CAC (chain-antichain)",
                        "Omitting types theorem",
                    ],
                    conservation_note: "Conservative over PA for first-order sentences",
                },
                RMHierarchyEntry {
                    system: BigFiveSystem::ATR0,
                    equivalents: vec![
                        "Open determinacy",
                        "Bar induction",
                        "Comparison of well-orderings",
                        "Hausdorff scattered characterisation",
                        "Ulm's theorem",
                        "Perfect set theorem",
                        "Σ¹₁ separation",
                    ],
                    conservation_note: "Π¹₂-conservative over ACA₀",
                },
                RMHierarchyEntry {
                    system: BigFiveSystem::Pi11CA0,
                    equivalents: vec![
                        "Π¹₁-comprehension",
                        "Σ¹₁-determinacy",
                        "Hyperarithmetic theorem",
                        "Every Π¹₁ set is a union of ω₁-many Borel sets",
                        "Cantor-Bendixson theorem (full)",
                    ],
                    conservation_note: "Strongest of the Big Five",
                },
            ],
        }
    }
    /// Print the hierarchy to a String (suitable for display/logging).
    pub fn display(&self) -> String {
        let mut out = String::from("=== Reverse Mathematics Hierarchy ===\n");
        for entry in &self.entries {
            out.push_str(&format!(
                "\n[{}] (ordinal: {})\n",
                entry.system.name(),
                entry.system.proof_theoretic_ordinal()
            ));
            out.push_str(&format!("  Conservation: {}\n", entry.conservation_note));
            out.push_str("  Equivalents:\n");
            for eq in &entry.equivalents {
                out.push_str(&format!("    - {}\n", eq));
            }
        }
        out
    }
    /// Return the entry for the given system, if present.
    pub fn entry_for(&self, system: &BigFiveSystem) -> Option<&RMHierarchyEntry> {
        self.entries.iter().find(|e| &e.system == system)
    }
}
/// Strength classification for a combinatorial principle in the RM zoo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RMStrength {
    /// Provable in RCA₀.
    RCA0,
    /// Strictly between RCA₀ and WKL₀.
    BetweenRCA0AndWKL0,
    /// Equivalent to WKL₀ over RCA₀.
    WKL0,
    /// Strictly between WKL₀ and ACA₀.
    BetweenWKL0AndACA0,
    /// Equivalent to ACA₀ over RCA₀.
    ACA0,
    /// Strictly between ACA₀ and ATR₀.
    BetweenACA0AndATR0,
    /// Equivalent to ATR₀ over RCA₀.
    ATR0,
    /// Equivalent to Π¹_1-CA₀ over RCA₀.
    Pi11CA0,
    /// Strictly above all Big Five (requires additional set-existence).
    AbovePi11CA0,
    /// Strength not yet determined.
    Unknown,
}
impl RMStrength {
    /// Return the closest Big Five system from below (or None if below RCA₀).
    pub fn lower_bound_system(&self) -> Option<BigFiveSystem> {
        match self {
            RMStrength::RCA0 => Some(BigFiveSystem::RCA0),
            RMStrength::BetweenRCA0AndWKL0 => Some(BigFiveSystem::RCA0),
            RMStrength::WKL0 => Some(BigFiveSystem::WKL0),
            RMStrength::BetweenWKL0AndACA0 => Some(BigFiveSystem::WKL0),
            RMStrength::ACA0 => Some(BigFiveSystem::ACA0),
            RMStrength::BetweenACA0AndATR0 => Some(BigFiveSystem::ACA0),
            RMStrength::ATR0 => Some(BigFiveSystem::ATR0),
            RMStrength::Pi11CA0 | RMStrength::AbovePi11CA0 => Some(BigFiveSystem::Pi11CA0),
            RMStrength::Unknown => None,
        }
    }
}
