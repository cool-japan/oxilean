//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::functions::*;
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EffectiveBorelSet {
    pub name: String,
    pub lightface_class: LightfaceClass,
    pub boldface_class: BoldfaceClass,
    pub is_recursive: bool,
    pub degree_of_unsolvability: Option<String>,
}
#[allow(dead_code)]
impl EffectiveBorelSet {
    pub fn recursive_set(name: &str) -> Self {
        EffectiveBorelSet {
            name: name.to_string(),
            lightface_class: LightfaceClass::Delta01,
            boldface_class: BoldfaceClass::BoldDelta02,
            is_recursive: true,
            degree_of_unsolvability: Some("0".to_string()),
        }
    }
    pub fn re_set(name: &str) -> Self {
        EffectiveBorelSet {
            name: name.to_string(),
            lightface_class: LightfaceClass::Sigma01,
            boldface_class: BoldfaceClass::BoldSigma01,
            is_recursive: false,
            degree_of_unsolvability: Some("0'".to_string()),
        }
    }
    pub fn lightface_boldface_correspondence(&self) -> String {
        "Lightface ↔ Boldface via oracle: Σ^0_n = Σ^0_n(ω) with oracle for ∅^(n)".to_string()
    }
    pub fn moschovakis_theorem(&self) -> String {
        "Moschovakis: lightface Σ^1_1 sets are exactly Souslin sets definable from recursive ordinals"
            .to_string()
    }
    pub fn characterization(&self) -> String {
        match &self.lightface_class {
            LightfaceClass::Delta01 => format!("{}: recursive set (Δ^0_1)", self.name),
            LightfaceClass::Sigma01 => format!("{}: r.e. set (Σ^0_1)", self.name),
            LightfaceClass::Sigma11 => format!("{}: analytic (Σ^1_1)", self.name),
            LightfaceClass::Pi11 => format!("{}: co-analytic (Π^1_1)", self.name),
            _ => format!("{}: higher descriptive set", self.name),
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ForcingPoset {
    pub name: String,
    pub forcing_extension: String,
    pub collapses_cardinals: bool,
    pub preserves_cardinals: bool,
    pub adds_real: bool,
}
#[allow(dead_code)]
impl ForcingPoset {
    pub fn cohen_forcing() -> Self {
        ForcingPoset {
            name: "Cohen forcing Fn(ω,2)".to_string(),
            forcing_extension: "V[G] adds Cohen real".to_string(),
            collapses_cardinals: false,
            preserves_cardinals: true,
            adds_real: true,
        }
    }
    pub fn random_forcing() -> Self {
        ForcingPoset {
            name: "Random forcing (measure algebra)".to_string(),
            forcing_extension: "V[G] adds random real".to_string(),
            collapses_cardinals: false,
            preserves_cardinals: true,
            adds_real: true,
        }
    }
    pub fn collapsing_forcing(target: &str) -> Self {
        ForcingPoset {
            name: format!("Levy collapse Col(ω, {})", target),
            forcing_extension: format!("V[G]: {} = ω_1^V[G]", target),
            collapses_cardinals: true,
            preserves_cardinals: false,
            adds_real: true,
        }
    }
    pub fn independence_of_ch(&self) -> String {
        "Cohen: ZFC + ¬CH consistent (Cohen forcing). Gödel: ZFC + CH consistent (L)".to_string()
    }
    pub fn generic_absoluteness(&self) -> String {
        "Shoenfield absoluteness: Σ^1_2 facts are forcing-absolute".to_string()
    }
}
/// Scott analysis rank.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ScottRank {
    pub structure: String,
    pub rank: usize,
}
#[allow(dead_code)]
impl ScottRank {
    /// Scott rank of a countable structure.
    pub fn new(structure: &str, rank: usize) -> Self {
        Self {
            structure: structure.to_string(),
            rank,
        }
    }
    /// Scott's isomorphism theorem: countable structures are determined by their Scott sentence.
    pub fn scott_sentence_description(&self) -> String {
        format!(
            "Scott sentence for {} has quantifier rank {}",
            self.structure, self.rank
        )
    }
}
/// An infinite two-player game with a winning set.
#[derive(Debug, Clone)]
pub struct InfiniteGame {
    pub player: String,
    pub winning_set: String,
}
impl InfiniteGame {
    pub fn new(player: impl Into<String>, winning_set: impl Into<String>) -> Self {
        InfiniteGame {
            player: player.into(),
            winning_set: winning_set.into(),
        }
    }
    /// Returns whether this game is determined (one player has a winning strategy).
    pub fn is_determined(&self) -> bool {
        true
    }
    /// Borel determinacy: every Borel game is determined.
    pub fn borel_determinacy(&self) -> bool {
        true
    }
}
/// A level in the projective hierarchy.
#[derive(Debug, Clone)]
pub struct ProjectiveHierarchy {
    pub level: u32,
}
impl ProjectiveHierarchy {
    pub fn new(level: u32) -> Self {
        ProjectiveHierarchy { level }
    }
    /// Σ¹₁ sets are analytic (continuous images of Borel sets).
    pub fn sigma_1_1_is_analytic(&self) -> bool {
        self.level == 1
    }
    /// Π¹₁ sets are co-analytic (complements of analytic sets).
    pub fn pi_1_1_is_coanalytic(&self) -> bool {
        self.level == 1
    }
    /// Determinacy at this projective level (requires large cardinals for level ≥ 2).
    pub fn determinacy(&self) -> bool {
        self.level <= 1
    }
}
/// Computes Wadge degree ordering for sets in a finite name-indexed universe.
#[derive(Debug, Clone)]
pub struct WadgeDegreesComputer {
    /// Named sets with their assigned Wadge rank (lower = simpler).
    pub degrees: std::collections::HashMap<String, u64>,
}
impl WadgeDegreesComputer {
    /// Create a new Wadge degrees computer.
    pub fn new() -> Self {
        Self {
            degrees: std::collections::HashMap::new(),
        }
    }
    /// Assign a Wadge rank to a named set.
    pub fn assign(&mut self, name: impl Into<String>, rank: u64) {
        self.degrees.insert(name.into(), rank);
    }
    /// Check whether A ≤_W B (A has rank ≤ B's rank).
    pub fn wadge_le(&self, a: &str, b: &str) -> bool {
        match (self.degrees.get(a), self.degrees.get(b)) {
            (Some(&ra), Some(&rb)) => ra <= rb,
            _ => false,
        }
    }
    /// Check whether A and B are Wadge equivalent (same rank).
    pub fn wadge_equiv(&self, a: &str, b: &str) -> bool {
        match (self.degrees.get(a), self.degrees.get(b)) {
            (Some(&ra), Some(&rb)) => ra == rb,
            _ => false,
        }
    }
    /// Under AD, the Wadge order is well-founded: every nonempty set has a minimal element.
    /// Returns the name of the set with minimal Wadge rank.
    pub fn minimal(&self) -> Option<&str> {
        self.degrees
            .iter()
            .min_by_key(|(_, &v)| v)
            .map(|(k, _)| k.as_str())
    }
    /// Returns all sets Wadge-reducible to the given set (with rank ≤ its rank).
    pub fn reducible_to(&self, target: &str) -> Vec<&str> {
        if let Some(&rb) = self.degrees.get(target) {
            self.degrees
                .iter()
                .filter(|(_, &ra)| ra <= rb)
                .map(|(k, _)| k.as_str())
                .collect()
        } else {
            Vec::new()
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct WellfoundedRelation {
    pub name: String,
    pub order_type: String,
    pub is_linear: bool,
    pub rank_function: String,
}
#[allow(dead_code)]
impl WellfoundedRelation {
    pub fn natural_numbers() -> Self {
        WellfoundedRelation {
            name: "(ℕ, <)".to_string(),
            order_type: "ω".to_string(),
            is_linear: true,
            rank_function: "rank(n) = n".to_string(),
        }
    }
    pub fn tree_ordering(name: &str) -> Self {
        WellfoundedRelation {
            name: name.to_string(),
            order_type: "ordinal".to_string(),
            is_linear: false,
            rank_function: "rank(t) = sup{rank(s)+1 : s <_T t}".to_string(),
        }
    }
    pub fn kleene_brouwer_ordering(&self) -> String {
        "KB ordering: linear well-ordering of a tree iff tree is well-founded".to_string()
    }
    pub fn rank_pi11_characterization(&self) -> String {
        "Π^1_1 set has well-founded rank function iff it is co-analytic (Kunen-Martin)".to_string()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ProperForcingAxiom {
    pub implies_ma: bool,
    pub implies_not_ch: bool,
    pub semi_proper_version: bool,
    pub consistency_strength_description: String,
}
#[allow(dead_code)]
impl ProperForcingAxiom {
    pub fn pfa() -> Self {
        ProperForcingAxiom {
            implies_ma: true,
            implies_not_ch: true,
            semi_proper_version: false,
            consistency_strength_description:
                "PFA consistent rel. supercompact cardinal (Baumgartner)".to_string(),
        }
    }
    pub fn spfa() -> Self {
        ProperForcingAxiom {
            implies_ma: true,
            implies_not_ch: true,
            semi_proper_version: true,
            consistency_strength_description: "SPFA consistent rel. supercompact".to_string(),
        }
    }
    pub fn woodin_provable_consequences(&self) -> String {
        "Woodin: PFA implies all sets of reals in L(R) are determined (AD^{L(R)})".to_string()
    }
}
/// Baire category data.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BaireCategory {
    pub space: String,
    pub is_meager: bool,
    pub is_comeager: bool,
}
#[allow(dead_code)]
impl BaireCategory {
    /// Meager (first category) set.
    pub fn meager(space: &str) -> Self {
        Self {
            space: space.to_string(),
            is_meager: true,
            is_comeager: false,
        }
    }
    /// Comeager (residual) set.
    pub fn comeager(space: &str) -> Self {
        Self {
            space: space.to_string(),
            is_meager: false,
            is_comeager: true,
        }
    }
    /// In a Baire space, a meager set has empty interior.
    pub fn baire_space_implication(&self, is_baire_space: bool) -> bool {
        !is_baire_space || !self.is_meager
    }
}
/// Projective hierarchy level.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct ProjectiveLevelData {
    pub sigma_n: usize,
    pub pi_n: usize,
    pub delta_n: usize,
}
#[allow(dead_code)]
impl ProjectiveLevelData {
    /// Level n of the projective hierarchy.
    pub fn level(n: usize) -> Self {
        Self {
            sigma_n: n,
            pi_n: n,
            delta_n: n,
        }
    }
    /// Sigma^1_1 is the analytic level.
    pub fn is_analytic(&self) -> bool {
        self.sigma_n == 1
    }
    /// Boldface classes are closed under continuous preimages.
    pub fn closed_under_continuous_preimage(&self) -> bool {
        true
    }
}
/// A perfect set: closed with no isolated points.
#[derive(Debug, Clone)]
pub struct PerfectSet {
    pub polish_space: String,
}
impl PerfectSet {
    pub fn new(polish_space: impl Into<String>) -> Self {
        PerfectSet {
            polish_space: polish_space.into(),
        }
    }
    /// Every nonempty perfect set in a Polish space has continuum-many points.
    pub fn has_continuum_many_points(&self) -> bool {
        true
    }
    /// Cantor-Bendixson: every closed set = perfect set ∪ countable scattered set.
    pub fn cantor_bendixson(&self) -> (bool, bool) {
        (true, true)
    }
    /// Perfect set theorem: every uncountable analytic set contains a perfect subset.
    pub fn perfect_set_thm(&self) -> bool {
        true
    }
}
/// Level in the projective hierarchy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectiveLevel {
    /// Σ¹_n.
    Sigma(u32),
    /// Π¹_n.
    Pi(u32),
    /// Δ¹_n.
    Delta(u32),
}
impl ProjectiveLevel {
    /// Dual class.
    pub fn dual(&self) -> ProjectiveLevel {
        match self {
            ProjectiveLevel::Sigma(n) => ProjectiveLevel::Pi(*n),
            ProjectiveLevel::Pi(n) => ProjectiveLevel::Sigma(*n),
            ProjectiveLevel::Delta(n) => ProjectiveLevel::Delta(*n),
        }
    }
    /// Is this the analytic level (Σ¹_1)?
    pub fn is_analytic(&self) -> bool {
        matches!(self, ProjectiveLevel::Sigma(1))
    }
    /// Is this the co-analytic level (Π¹_1)?
    pub fn is_coanalytic(&self) -> bool {
        matches!(self, ProjectiveLevel::Pi(1))
    }
    /// Is this the Borel = Δ¹_1 level?
    pub fn is_borel(&self) -> bool {
        matches!(self, ProjectiveLevel::Delta(1))
    }
}
/// Checks set membership in a finite approximation of the Borel hierarchy.
#[derive(Debug, Clone)]
pub struct BorelHierarchyChecker {
    /// The level in the hierarchy we are tracking.
    pub level: u32,
    /// Whether the class is Σ (true) or Π (false).
    pub is_sigma: bool,
    /// Named sets registered at this level.
    pub sets: Vec<String>,
}
impl BorelHierarchyChecker {
    /// Create a new checker at the given Σ or Π level.
    pub fn new(level: u32, is_sigma: bool) -> Self {
        Self {
            level,
            is_sigma,
            sets: Vec::new(),
        }
    }
    /// Register a set name as belonging to this class.
    pub fn register(&mut self, name: impl Into<String>) {
        self.sets.push(name.into());
    }
    /// Returns true if `name` is registered in this class.
    pub fn contains(&self, name: &str) -> bool {
        self.sets.iter().any(|s| s == name)
    }
    /// The dual class (Σ ↔ Π).
    pub fn dual(&self) -> BorelHierarchyChecker {
        BorelHierarchyChecker {
            level: self.level,
            is_sigma: !self.is_sigma,
            sets: Vec::new(),
        }
    }
    /// The successor class (Σ_n → Π_n → Σ_{n+1}).
    pub fn successor(&self) -> BorelHierarchyChecker {
        if self.is_sigma {
            BorelHierarchyChecker::new(self.level, false)
        } else {
            BorelHierarchyChecker::new(self.level + 1, true)
        }
    }
    /// Return the name of this class (e.g., "Σ⁰_2").
    pub fn class_name(&self) -> String {
        let tag = if self.is_sigma { "Σ" } else { "Π" };
        format!("{}⁰_{}", tag, self.level)
    }
    /// Every Σ class is closed under countable unions.
    pub fn closed_under_unions(&self) -> bool {
        self.is_sigma
    }
    /// Every Π class is closed under countable intersections.
    pub fn closed_under_intersections(&self) -> bool {
        !self.is_sigma
    }
    /// Both classes are closed under finite intersections/unions at the same level.
    pub fn closed_under_finite_boolean_ops(&self) -> bool {
        true
    }
}
/// Tree on a product space (descriptive set theory tool).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DescriptiveTree {
    pub alphabet: Vec<String>,
    pub is_pruned: bool,
    pub is_well_founded: bool,
}
#[allow(dead_code)]
impl DescriptiveTree {
    /// Pruned tree (no dead ends).
    pub fn pruned(alphabet: Vec<String>) -> Self {
        Self {
            alphabet,
            is_pruned: true,
            is_well_founded: false,
        }
    }
    /// Well-founded tree (no infinite branches = corresponds to bounded rank).
    pub fn well_founded(alphabet: Vec<String>) -> Self {
        Self {
            alphabet,
            is_pruned: false,
            is_well_founded: true,
        }
    }
    /// A tree is ill-founded iff it has an infinite branch (König's lemma for finitely branching).
    pub fn has_infinite_branch(&self) -> bool {
        !self.is_well_founded
    }
    /// Kleene-Brouwer ordering makes a well-founded tree into a well-order.
    pub fn kleene_brouwer_applies(&self) -> bool {
        self.is_well_founded
    }
}
/// The Wadge hierarchy of topological reducibility.
#[derive(Debug, Clone)]
pub struct WadgeHierarchy;
impl WadgeHierarchy {
    pub fn new() -> Self {
        WadgeHierarchy
    }
    /// Wadge reducibility: A ≤_W B if A = f⁻¹(B) for some continuous f.
    pub fn wadge_reducibility(&self) -> bool {
        true
    }
    /// Martin-Steel theorem: under AD, the Wadge hierarchy is well-founded and semi-linear.
    pub fn martin_steel_theorem(&self) -> bool {
        true
    }
}
/// The axiomatic strength of a determinacy principle.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum DetermincyStrength {
    /// Open determinacy (provable in ZFC, or even ATR₀).
    Open,
    /// Clopen determinacy.
    Clopen,
    /// Borel determinacy (Martin, ZFC).
    Borel,
    /// Analytic determinacy (requires measurable cardinal).
    Analytic,
    /// Projective determinacy (PD, requires infinitely many Woodin cardinals).
    Projective,
    /// Full axiom of determinacy (AD, inconsistent with AC, consistent with large cardinals).
    Full,
}
impl DetermincyStrength {
    /// Whether this principle is provable in ordinary ZFC.
    pub fn provable_in_zfc(&self) -> bool {
        matches!(
            self,
            DetermincyStrength::Open | DetermincyStrength::Clopen | DetermincyStrength::Borel
        )
    }
    /// Whether this principle requires a large cardinal hypothesis.
    pub fn requires_large_cardinal(&self) -> bool {
        matches!(
            self,
            DetermincyStrength::Analytic
                | DetermincyStrength::Projective
                | DetermincyStrength::Full
        )
    }
}
/// Borel hierarchy class at a given level.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BorelHierarchy {
    pub level: u32,
    pub is_sigma: bool,
}
impl BorelHierarchy {
    pub fn sigma(level: u32) -> Self {
        BorelHierarchy {
            level,
            is_sigma: true,
        }
    }
    pub fn pi(level: u32) -> Self {
        BorelHierarchy {
            level,
            is_sigma: false,
        }
    }
    /// Σ classes are closed under countable unions.
    pub fn closed_under_unions(&self) -> bool {
        self.is_sigma
    }
    /// Π classes are closed under countable intersections.
    pub fn closed_under_intersections(&self) -> bool {
        !self.is_sigma
    }
    /// Both Σ and Π classes are closed under complements at the same level only if Δ.
    pub fn closed_under_complements(&self) -> bool {
        false
    }
    /// Return the dual class.
    pub fn dual(&self) -> BorelHierarchy {
        BorelHierarchy {
            level: self.level,
            is_sigma: !self.is_sigma,
        }
    }
}
/// A Polish space (separable, completely metrizable topological space).
#[derive(Debug, Clone)]
pub struct PolishSpace {
    pub is_separable: bool,
    pub is_completely_metrizable: bool,
}
impl PolishSpace {
    pub fn new(is_separable: bool, is_completely_metrizable: bool) -> Self {
        PolishSpace {
            is_separable,
            is_completely_metrizable,
        }
    }
    /// Returns true if the space is Polish.
    pub fn is_polish(&self) -> bool {
        self.is_separable && self.is_completely_metrizable
    }
    /// Returns whether this Polish space is zero-dimensional.
    /// (Formal: all Polish spaces with a countable basis of clopen sets are zero-dimensional.)
    pub fn is_zero_dimensional(&self) -> bool {
        self.is_polish()
    }
    /// Topological classification: returns a string describing the space type.
    pub fn topological_classification(&self) -> &'static str {
        if !self.is_separable {
            "non-separable"
        } else if !self.is_completely_metrizable {
            "separable but not completely metrizable"
        } else {
            "Polish"
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MartinsAxiom {
    pub cardinal: String,
    pub ccc_forcing: bool,
    pub consequences: Vec<String>,
}
#[allow(dead_code)]
impl MartinsAxiom {
    pub fn ma_not_ch() -> Self {
        MartinsAxiom {
            cardinal: "ℵ_1 ≤ κ < 2^ℵ_0".to_string(),
            ccc_forcing: true,
            consequences: vec![
                "2^ℵ_0 > ℵ_1 (¬CH)".to_string(),
                "Every ccc poset with 2^ℵ_0 antichains has generic filter".to_string(),
                "All Aronszajn trees are special".to_string(),
                "SH (Souslin Hypothesis) holds".to_string(),
            ],
        }
    }
    pub fn consequence_count(&self) -> usize {
        self.consequences.len()
    }
    pub fn consistency(&self) -> String {
        "MA + ¬CH is consistent relative to ZFC (Solovay-Tennenbaum, 1971)".to_string()
    }
    pub fn ccc_property_description(&self) -> String {
        "CCC (countable chain condition): every antichain is countable".to_string()
    }
}
/// Solves finite approximations of infinite two-player zero-sum games.
/// Uses backwards induction on a finite game tree.
#[derive(Debug, Clone)]
pub struct DeterminacyGameSolver {
    /// Payoff function: game state index → optional winner (true = Player I wins).
    pub payoffs: Vec<Option<bool>>,
    /// Children of each game node (moves available).
    pub children: Vec<Vec<usize>>,
    /// Number of nodes in the game tree.
    pub num_nodes: usize,
    /// Whose turn is it at each node (true = Player I).
    pub player_turn: Vec<bool>,
}
impl DeterminacyGameSolver {
    /// Create a new game tree with `n` nodes.
    pub fn new(n: usize) -> Self {
        Self {
            payoffs: vec![None; n],
            children: vec![Vec::new(); n],
            num_nodes: n,
            player_turn: vec![true; n],
        }
    }
    /// Set the payoff at a terminal node.
    pub fn set_payoff(&mut self, node: usize, player_i_wins: bool) {
        if node < self.num_nodes {
            self.payoffs[node] = Some(player_i_wins);
        }
    }
    /// Add a child move from `parent` to `child`.
    pub fn add_move(&mut self, parent: usize, child: usize) {
        if parent < self.num_nodes && child < self.num_nodes {
            self.children[parent].push(child);
        }
    }
    /// Set whose turn it is at a node (true = Player I moves).
    pub fn set_player(&mut self, node: usize, player_i: bool) {
        if node < self.num_nodes {
            self.player_turn[node] = player_i;
        }
    }
    /// Solve the game by backwards induction.
    /// Returns a vector: for each node, the winner (true = Player I) if determined.
    pub fn solve(&self) -> Vec<Option<bool>> {
        let mut result = self.payoffs.clone();
        let mut changed = true;
        while changed {
            changed = false;
            for node in 0..self.num_nodes {
                if result[node].is_some() {
                    continue;
                }
                let ch = &self.children[node];
                if ch.is_empty() {
                    continue;
                }
                let child_vals: Vec<Option<bool>> = ch.iter().map(|&c| result[c]).collect();
                if child_vals.iter().all(|v| v.is_none()) {
                    continue;
                }
                let player_i_moves = self.player_turn[node];
                let winning_val = if player_i_moves {
                    child_vals.contains(&Some(true))
                } else {
                    !child_vals.contains(&Some(true))
                };
                if child_vals.iter().all(|v| v.is_some()) || winning_val {
                    result[node] = Some(winning_val);
                    changed = true;
                }
            }
        }
        result
    }
    /// Is the game at the root node determined?
    pub fn is_determined(&self) -> bool {
        self.solve()[0].is_some()
    }
    /// Who wins with optimal play from the root? Returns None if not determined yet.
    pub fn winner(&self) -> Option<bool> {
        self.solve()[0]
    }
}
/// Wadge reducibility data.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct WadgeDegree {
    pub set_name: String,
    pub level: usize,
    pub is_selfdual: bool,
}
#[allow(dead_code)]
impl WadgeDegree {
    /// Create a Wadge degree.
    pub fn new(name: &str, level: usize, selfdual: bool) -> Self {
        Self {
            set_name: name.to_string(),
            level,
            is_selfdual: selfdual,
        }
    }
    /// Non-selfdual pairs have a natural pairing.
    pub fn has_complement_pair(&self) -> bool {
        !self.is_selfdual
    }
    /// Wadge's lemma: any two Baire-measurable sets are Wadge comparable or complementary.
    pub fn wadge_lemma_description() -> &'static str {
        "Wadge lemma: for Baire-measurable A, B: either A <=_W B or B <=_W complement(A)"
    }
}
/// Represents an orbit equivalence relation induced by a group action on a finite set.
#[derive(Debug, Clone)]
pub struct OrbitEquivalenceRelation {
    /// Number of elements in the base set X.
    pub num_elements: usize,
    /// Union-Find parent array for the equivalence classes.
    parent: Vec<usize>,
    /// Rank array for union-by-rank.
    rank: Vec<usize>,
}
impl OrbitEquivalenceRelation {
    /// Create a new orbit equivalence relation on `n` elements (initially discrete).
    pub fn new(n: usize) -> Self {
        Self {
            num_elements: n,
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }
    /// Find the representative of the equivalence class of `x`.
    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }
    /// Merge the equivalence classes of `x` and `y` (apply group action).
    pub fn union(&mut self, x: usize, y: usize) {
        let rx = self.find(x);
        let ry = self.find(y);
        if rx == ry {
            return;
        }
        if self.rank[rx] < self.rank[ry] {
            self.parent[rx] = ry;
        } else if self.rank[rx] > self.rank[ry] {
            self.parent[ry] = rx;
        } else {
            self.parent[ry] = rx;
            self.rank[rx] += 1;
        }
    }
    /// Returns true if `x` and `y` are in the same orbit.
    pub fn same_orbit(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
    /// Count the number of orbits.
    pub fn num_orbits(&mut self) -> usize {
        let mut roots = std::collections::HashSet::new();
        for i in 0..self.num_elements {
            roots.insert(self.find(i));
        }
        roots.len()
    }
    /// Get all orbits as a list of equivalence classes.
    pub fn orbits(&mut self) -> Vec<Vec<usize>> {
        let mut map: std::collections::HashMap<usize, Vec<usize>> =
            std::collections::HashMap::new();
        for i in 0..self.num_elements {
            let r = self.find(i);
            map.entry(r).or_default().push(i);
        }
        map.into_values().collect()
    }
    /// Check whether the relation is smooth (finitely many orbits).
    pub fn is_smooth(&mut self) -> bool {
        self.num_orbits() <= self.num_elements
    }
    /// Check whether the relation is hyperfinite (formal stub — always true for finite sets).
    pub fn is_hyperfinite(&self) -> bool {
        true
    }
}
/// A named example of a Polish space with key properties.
#[derive(Debug, Clone)]
pub struct PolishSpaceExample {
    /// Descriptive name.
    pub name: &'static str,
    /// Is the space zero-dimensional?
    pub zero_dimensional: bool,
    /// Is the space compact?
    pub compact: bool,
    /// Is the space locally compact?
    pub locally_compact: bool,
    /// Is the space the universal Polish space (homeomorphic to Baire space)?
    pub is_baire_space: bool,
}
impl PolishSpaceExample {
    /// The real line ℝ.
    pub fn real_line() -> Self {
        Self {
            name: "Real line (ℝ)",
            zero_dimensional: false,
            compact: false,
            locally_compact: true,
            is_baire_space: false,
        }
    }
    /// Baire space ℕ^ℕ.
    pub fn baire_space() -> Self {
        Self {
            name: "Baire space (ℕ^ℕ)",
            zero_dimensional: true,
            compact: false,
            locally_compact: false,
            is_baire_space: true,
        }
    }
    /// Cantor space 2^ℕ.
    pub fn cantor_space() -> Self {
        Self {
            name: "Cantor space (2^ℕ)",
            zero_dimensional: true,
            compact: true,
            locally_compact: true,
            is_baire_space: false,
        }
    }
    /// Hilbert cube [0,1]^ℕ.
    pub fn hilbert_cube() -> Self {
        Self {
            name: "Hilbert cube ([0,1]^ℕ)",
            zero_dimensional: false,
            compact: true,
            locally_compact: false,
            is_baire_space: false,
        }
    }
}
/// A finite topological space represented by its point set and isolated-point
/// detector (for illustrative purposes — infinite spaces need ordinal machinery).
#[derive(Debug, Clone)]
pub struct FiniteTopSpace {
    /// Number of points.
    pub size: usize,
    /// For each point index, whether the point is isolated.
    pub isolated: Vec<bool>,
}
impl FiniteTopSpace {
    /// Create a new finite topological space; no point is isolated by default.
    pub fn new(size: usize) -> Self {
        Self {
            size,
            isolated: vec![false; size],
        }
    }
    /// Mark point `i` as isolated.
    pub fn mark_isolated(&mut self, i: usize) {
        if i < self.size {
            self.isolated[i] = true;
        }
    }
    /// Apply one step of the Cantor–Bendixson derivative: remove isolated points.
    pub fn cb_derivative(&self) -> FiniteTopSpace {
        let new_size = self.isolated.iter().filter(|&&b| !b).count();
        FiniteTopSpace {
            size: new_size,
            isolated: vec![false; new_size],
        }
    }
    /// Count of non-isolated (perfect candidate) points.
    pub fn non_isolated_count(&self) -> usize {
        self.isolated.iter().filter(|&&b| !b).count()
    }
    /// Is the space scattered? (all points eventually isolated under iteration)
    pub fn is_scattered(&self) -> bool {
        let mut current = self.clone();
        loop {
            if current.size == 0 {
                return true;
            }
            let next = current.cb_derivative();
            if next.size == current.size {
                return false;
            }
            current = next;
        }
    }
    /// Compute the CB-rank as the number of derivative steps until the space empties.
    pub fn cb_rank(&self) -> u32 {
        let mut current = self.clone();
        let mut rank: u32 = 0;
        loop {
            if current.size == 0 {
                return rank;
            }
            let next = current.cb_derivative();
            if next.size == current.size {
                return u32::MAX;
            }
            current = next;
            rank += 1;
        }
    }
}
/// A Lipschitz function with constant k.
#[derive(Debug, Clone)]
pub struct LipschitzFunction {
    pub k: f64,
    pub domain: String,
}
impl LipschitzFunction {
    pub fn new(k: f64, domain: impl Into<String>) -> Self {
        LipschitzFunction {
            k,
            domain: domain.into(),
        }
    }
    /// Every Lipschitz function is uniformly continuous.
    pub fn is_uniformly_continuous(&self) -> bool {
        self.k >= 0.0
    }
    /// Every Lipschitz function is (Borel/Lebesgue) measurable.
    pub fn is_measurable(&self) -> bool {
        true
    }
    /// Rademacher's theorem: Lipschitz functions are differentiable a.e.
    pub fn rademacher_theorem(&self) -> bool {
        self.k < f64::INFINITY
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum BoldfaceClass {
    BoldSigma01,
    BoldPi01,
    BoldDelta02,
    BoldSigma11,
    BoldPi11,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LargeCardinal {
    pub name: String,
    pub consistency_strength: String,
    pub cardinal_property: String,
    pub is_measurable: bool,
    pub is_strong: bool,
}
#[allow(dead_code)]
impl LargeCardinal {
    pub fn inaccessible(alpha: usize) -> Self {
        LargeCardinal {
            name: format!("ℵ_{}", alpha),
            consistency_strength: "ZFC + Inaccessible".to_string(),
            cardinal_property: "regular and strong limit".to_string(),
            is_measurable: false,
            is_strong: false,
        }
    }
    pub fn measurable_cardinal() -> Self {
        LargeCardinal {
            name: "κ (measurable)".to_string(),
            consistency_strength: "Con(ZFC) < Con(ZFC + Measurable)".to_string(),
            cardinal_property: "admits non-principal κ-complete ultrafilter".to_string(),
            is_measurable: true,
            is_strong: false,
        }
    }
    pub fn woodin_cardinal() -> Self {
        LargeCardinal {
            name: "δ (Woodin)".to_string(),
            consistency_strength: "Con(ZFC + ∞ Woodin) ↔ PD".to_string(),
            cardinal_property: "Woodin cardinal (Martin-Steel)".to_string(),
            is_measurable: false,
            is_strong: true,
        }
    }
    pub fn projective_determinacy_connection(&self) -> String {
        if self.name.contains("Woodin") {
            "Woodin (1988): PD is equivalent to existence of ∞ many Woodin cardinals".to_string()
        } else if self.is_measurable {
            "Measurable cardinal implies analytic determinacy (Martin)".to_string()
        } else {
            "Connection to determinacy depends on size of cardinal".to_string()
        }
    }
    pub fn silver_indiscernibles(&self) -> String {
        if self.is_measurable {
            "Existence of 0# (sharp): if V ≠ L then there is a measurable cardinal (Silver)"
                .to_string()
        } else {
            "No sharp: L is close to V".to_string()
        }
    }
}
/// Level in the Borel hierarchy (finite approximation).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum BorelLevel {
    /// Σ⁰_α class at the given rank.
    Sigma(u32),
    /// Π⁰_α class at the given rank.
    Pi(u32),
    /// Δ⁰_α (ambiguous) class.
    Delta(u32),
}
impl BorelLevel {
    /// Return the dual class (Σ ↔ Π, Δ fixed).
    pub fn dual(&self) -> BorelLevel {
        match self {
            BorelLevel::Sigma(n) => BorelLevel::Pi(*n),
            BorelLevel::Pi(n) => BorelLevel::Sigma(*n),
            BorelLevel::Delta(n) => BorelLevel::Delta(*n),
        }
    }
    /// Return the successor level (Σ_n ↦ Π_{n+1}, Π_n ↦ Σ_{n+1}).
    pub fn successor(&self) -> BorelLevel {
        match self {
            BorelLevel::Sigma(n) => BorelLevel::Pi(*n),
            BorelLevel::Pi(n) => BorelLevel::Sigma(*n + 1),
            BorelLevel::Delta(n) => BorelLevel::Sigma(*n + 1),
        }
    }
    /// Is this level the open sets (Σ⁰_1)?
    pub fn is_open(&self) -> bool {
        matches!(self, BorelLevel::Sigma(1))
    }
    /// Is this level the closed sets (Π⁰_1)?
    pub fn is_closed(&self) -> bool {
        matches!(self, BorelLevel::Pi(1))
    }
    /// Rank integer (ignoring Σ/Π/Δ tag).
    pub fn rank(&self) -> u32 {
        match self {
            BorelLevel::Sigma(n) | BorelLevel::Pi(n) | BorelLevel::Delta(n) => *n,
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum LightfaceClass {
    Sigma01,
    Pi01,
    Delta01,
    Sigma11,
    Pi11,
    Delta11,
    SigmaN(usize),
    PiN(usize),
}
/// A universally measurable set.
#[derive(Debug, Clone)]
pub struct UniversallyMeasurable {
    pub set: String,
}
impl UniversallyMeasurable {
    pub fn new(set: impl Into<String>) -> Self {
        UniversallyMeasurable { set: set.into() }
    }
    /// Checks (formally) whether this set is universally measurable.
    pub fn is_universally_measurable(&self) -> bool {
        true
    }
    /// Inner regularity: approximation from inside by closed sets.
    pub fn inner_regularity(&self) -> bool {
        true
    }
}
/// An analytic (Σ¹₁) set in a Polish space.
#[derive(Debug, Clone)]
pub struct AnalyticSet {
    pub polish_space: String,
}
impl AnalyticSet {
    pub fn new(polish_space: impl Into<String>) -> Self {
        AnalyticSet {
            polish_space: polish_space.into(),
        }
    }
    /// Every analytic set is a continuous image of a Borel set (in a Polish space).
    pub fn is_continuous_image_of_borel(&self) -> bool {
        true
    }
    /// A set is Souslin if it can be obtained by the Souslin operation from closed sets.
    pub fn is_souslin(&self) -> bool {
        true
    }
    /// Luzin separation: two disjoint analytic sets can be separated by a Borel set.
    pub fn luzin_separation(&self, other: &AnalyticSet) -> bool {
        self.polish_space == other.polish_space
    }
}
/// Analytic set data.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AnalyticSetData {
    pub name: String,
    pub is_borel: bool,
    pub is_co_analytic: bool,
    pub description: String,
}
#[allow(dead_code)]
impl AnalyticSetData {
    /// Create an analytic (Sigma^1_1) set.
    pub fn new(name: &str, is_borel: bool) -> Self {
        Self {
            name: name.to_string(),
            is_borel,
            is_co_analytic: false,
            description: "Sigma^1_1 set".to_string(),
        }
    }
    /// Souslin's theorem: a set is Borel iff it's both analytic and co-analytic.
    pub fn souslin_borel(&self) -> bool {
        self.is_borel == (self.is_co_analytic && self.is_borel)
    }
    /// Separation theorem: any two disjoint analytic sets can be separated by a Borel set.
    pub fn separation_description() -> &'static str {
        "Disjoint analytic sets are Borel-separated"
    }
}
