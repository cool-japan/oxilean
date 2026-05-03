//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use std::collections::{HashMap, HashSet};

/// Decision table for rough set analysis.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DecisionTableExt {
    pub n_objects: usize,
    pub condition_attrs: Vec<String>,
    pub decision_attr: String,
    pub data: Vec<Vec<u32>>,
    pub decisions: Vec<u32>,
}
#[allow(dead_code)]
impl DecisionTableExt {
    pub fn new(n: usize, cond: Vec<&str>, dec: &str) -> Self {
        DecisionTableExt {
            n_objects: n,
            condition_attrs: cond.iter().map(|s| s.to_string()).collect(),
            decision_attr: dec.to_string(),
            data: vec![vec![0; cond.len()]; n],
            decisions: vec![0; n],
        }
    }
    pub fn set_row(&mut self, obj: usize, attrs: Vec<u32>, decision: u32) {
        self.data[obj] = attrs;
        self.decisions[obj] = decision;
    }
    pub fn n_attrs(&self) -> usize {
        self.condition_attrs.len()
    }
    /// Indiscernibility relation: objects i, j are equivalent w.r.t. attribute subset.
    pub fn indiscernible(&self, i: usize, j: usize, attrs: &[usize]) -> bool {
        attrs.iter().all(|&a| self.data[i][a] == self.data[j][a])
    }
    /// Equivalence classes under attribute subset.
    pub fn equivalence_classes(&self, attrs: &[usize]) -> Vec<Vec<usize>> {
        let mut classes: Vec<Vec<usize>> = Vec::new();
        let mut assigned = vec![false; self.n_objects];
        for i in 0..self.n_objects {
            if assigned[i] {
                continue;
            }
            let mut cls = vec![i];
            assigned[i] = true;
            for j in (i + 1)..self.n_objects {
                if !assigned[j] && self.indiscernible(i, j, attrs) {
                    cls.push(j);
                    assigned[j] = true;
                }
            }
            classes.push(cls);
        }
        classes
    }
    /// Positive region of decision w.r.t. condition attributes.
    pub fn positive_region(&self, cond_attrs: &[usize]) -> Vec<usize> {
        let classes = self.equivalence_classes(cond_attrs);
        let mut pos = Vec::new();
        for cls in &classes {
            let first_dec = self.decisions[cls[0]];
            if cls.iter().all(|&o| self.decisions[o] == first_dec) {
                pos.extend_from_slice(cls);
            }
        }
        pos.sort();
        pos
    }
    /// Accuracy of approximation: |lower| / |upper|.
    pub fn accuracy(&self, concept: &[usize], cond_attrs: &[usize]) -> f64 {
        let classes = self.equivalence_classes(cond_attrs);
        let concept_set: std::collections::HashSet<usize> = concept.iter().cloned().collect();
        let lower: usize = classes
            .iter()
            .filter(|cls| cls.iter().all(|o| concept_set.contains(o)))
            .map(|cls| cls.len())
            .sum();
        let upper: usize = classes
            .iter()
            .filter(|cls| cls.iter().any(|o| concept_set.contains(o)))
            .map(|cls| cls.len())
            .sum();
        if upper == 0 {
            1.0
        } else {
            lower as f64 / upper as f64
        }
    }
}
/// Variable precision rough set (VPRS) with threshold u.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct VPRS {
    pub precision: f64,
}
#[allow(dead_code)]
impl VPRS {
    pub fn new(u: f64) -> Self {
        assert!(u >= 0.0 && u < 0.5);
        VPRS { precision: u }
    }
    pub fn standard() -> Self {
        VPRS::new(0.0)
    }
    pub fn relative_positive_region(&self, class_in_concept: usize, class_size: usize) -> bool {
        if class_size == 0 {
            return false;
        }
        let ratio = class_in_concept as f64 / class_size as f64;
        1.0 - ratio <= self.precision
    }
}
/// Fuzzy rough set membership function using Łukasiewicz t-norm.
pub struct RoughFuzzyMembership {
    /// Fuzzy membership of each object in the target set (values in \[0,1\]).
    pub membership: Vec<f64>,
    /// Similarity matrix: sim\[i\]\[j\] ∈ \[0,1\].
    pub similarity: Vec<Vec<f64>>,
}
impl RoughFuzzyMembership {
    pub fn new(membership: Vec<f64>, similarity: Vec<Vec<f64>>) -> Self {
        RoughFuzzyMembership {
            membership,
            similarity,
        }
    }
    /// Łukasiewicz t-norm: T(a, b) = max(0, a + b - 1).
    pub fn t_norm_lukasiewicz(a: f64, b: f64) -> f64 {
        (a + b - 1.0).max(0.0)
    }
    /// Łukasiewicz t-conorm: S(a, b) = min(1, a + b).
    pub fn t_conorm_lukasiewicz(a: f64, b: f64) -> f64 {
        (a + b).min(1.0)
    }
    /// Fuzzy lower approximation of the target using Łukasiewicz t-norm.
    /// μ_lower(x) = inf_y { max(1 - sim(x,y), μ(y)) }  (implication-based)
    pub fn fuzzy_lower(&self, x: usize) -> f64 {
        let n = self.membership.len();
        (0..n)
            .map(|y| (1.0 - self.similarity[x][y]).max(self.membership[y]))
            .fold(f64::INFINITY, f64::min)
    }
    /// Fuzzy upper approximation: μ_upper(x) = sup_y { T(sim(x,y), μ(y)) }.
    pub fn fuzzy_upper(&self, x: usize) -> f64 {
        let n = self.membership.len();
        (0..n)
            .map(|y| Self::t_norm_lukasiewicz(self.similarity[x][y], self.membership[y]))
            .fold(0.0_f64, f64::max)
    }
    /// Compute rough fuzzy boundary degree for object x.
    pub fn boundary_degree(&self, x: usize) -> f64 {
        let upper = self.fuzzy_upper(x);
        let lower = self.fuzzy_lower(x);
        upper - lower
    }
}
/// A helper that computes dependency metrics for decision-system analysis.
pub struct DecisionSystemReducer {
    pub table: DecisionTable,
}
impl DecisionSystemReducer {
    pub fn new(table: DecisionTable) -> Self {
        DecisionSystemReducer { table }
    }
    /// Find the minimal reduct that achieves the full dependency degree.
    /// Returns the first reduct found (shortest first).
    pub fn minimal_reduct(&self) -> Vec<usize> {
        let mut all = self.table.all_reducts();
        all.sort_by_key(|r| r.len());
        all.into_iter().next().unwrap_or_default()
    }
    /// Compute attribute ranking by significance (descending).
    pub fn ranked_attributes(&self) -> Vec<(usize, f64)> {
        let cond = self.table.condition_attrs();
        let mut sigs: Vec<(usize, f64)> = cond
            .iter()
            .map(|&a| (a, attribute_significance(&self.table, a)))
            .collect();
        sigs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        sigs
    }
    /// Greedy reduct: add attributes greedily by significance until full dep.
    pub fn greedy_reduct(&self) -> Vec<usize> {
        let full_dep = self.table.dependency_degree();
        let ranked = self.ranked_attributes();
        let mut selected = Vec::new();
        for (attr, _) in ranked {
            selected.push(attr);
            let dep = self
                .table
                .info
                .quality_of_approximation(&selected, &[self.table.decision_attr]);
            if (dep - full_dep).abs() < 1e-9 {
                break;
            }
        }
        selected
    }
}
/// Extended VPRS wrapper with inclusion measure computation.
pub struct VariablePrecisionApprox {
    pub precision: f64,
    pub info: InformationSystem,
}
impl VariablePrecisionApprox {
    pub fn new(info: InformationSystem, precision: f64) -> Self {
        assert!((0.0..0.5).contains(&precision));
        VariablePrecisionApprox { precision, info }
    }
    /// Inclusion measure c(X, Y) = 1 - |X \ Y| / |X|.
    pub fn inclusion_measure(x: &HashSet<usize>, y: &HashSet<usize>) -> f64 {
        if x.is_empty() {
            return 1.0;
        }
        let diff = x.difference(y).count();
        1.0 - (diff as f64 / x.len() as f64)
    }
    /// u-lower approximation of X: classes whose inclusion measure ≥ 1-l.
    pub fn u_lower(&self, attrs: &[usize], target: &HashSet<usize>) -> HashSet<usize> {
        let threshold = 1.0 - self.precision;
        let classes = self.info.indiscernibility_classes(attrs);
        let mut result = HashSet::new();
        for cls in &classes {
            if Self::inclusion_measure(cls, target) >= threshold {
                result.extend(cls.iter().copied());
            }
        }
        result
    }
    /// u-upper approximation of X: classes with non-zero inclusion in X.
    pub fn u_upper(&self, attrs: &[usize], target: &HashSet<usize>) -> HashSet<usize> {
        let classes = self.info.indiscernibility_classes(attrs);
        let mut result = HashSet::new();
        for cls in &classes {
            if cls.intersection(target).next().is_some() {
                result.extend(cls.iter().copied());
            }
        }
        result
    }
}
/// Reduct: a minimal subset of attributes preserving discernibility.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AttributeReduct {
    pub reduct_attrs: Vec<usize>,
    pub n_total_attrs: usize,
}
#[allow(dead_code)]
impl AttributeReduct {
    pub fn new(reduct: Vec<usize>, total: usize) -> Self {
        AttributeReduct {
            reduct_attrs: reduct,
            n_total_attrs: total,
        }
    }
    pub fn reduction_ratio(&self) -> f64 {
        1.0 - (self.reduct_attrs.len() as f64 / self.n_total_attrs as f64)
    }
    pub fn is_minimal(&self) -> bool {
        !self.reduct_attrs.is_empty()
    }
}
/// A full discernibility matrix with helper methods.
pub struct DiscernibilityMatrix {
    pub n_objects: usize,
    /// matrix\[i\]\[j\] = set of attributes distinguishing i from j (decision-wise).
    pub matrix: Vec<Vec<HashSet<usize>>>,
}
impl DiscernibilityMatrix {
    /// Build the discernibility matrix from a decision table.
    pub fn build(dt: &DecisionTable) -> Self {
        let matrix = discernibility_matrix(dt);
        DiscernibilityMatrix {
            n_objects: dt.info.n_objects,
            matrix,
        }
    }
    /// Return the discernibility set for pair (i, j).
    pub fn get(&self, i: usize, j: usize) -> &HashSet<usize> {
        &self.matrix[i][j]
    }
    /// Compute the discernibility function (conjunction of disjunctions).
    /// Returns each non-empty discernibility set (one disjunct per pair).
    pub fn discernibility_function(&self) -> Vec<HashSet<usize>> {
        let mut clauses = Vec::new();
        for i in 0..self.n_objects {
            for j in (i + 1)..self.n_objects {
                if !self.matrix[i][j].is_empty() {
                    clauses.push(self.matrix[i][j].clone());
                }
            }
        }
        clauses
    }
    /// Check if a set of attributes is a hitting set (intersects every clause).
    pub fn is_hitting_set(&self, attrs: &HashSet<usize>) -> bool {
        self.discernibility_function()
            .iter()
            .all(|clause| clause.intersection(attrs).next().is_some())
    }
}
/// Rough truth values: a proposition can be certainly true, possibly true,
/// certainly false, or uncertain (rough).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoughTruth {
    CertainlyTrue,
    PossiblyTrue,
    CertainlyFalse,
    Uncertain,
}
impl RoughTruth {
    /// Determine the rough truth value of "x ∈ X" given lower and upper approximations.
    pub fn from_membership(x: usize, lower: &HashSet<usize>, upper: &HashSet<usize>) -> Self {
        let in_lower = lower.contains(&x);
        let in_upper = upper.contains(&x);
        match (in_lower, in_upper) {
            (true, true) => RoughTruth::CertainlyTrue,
            (false, true) => RoughTruth::Uncertain,
            (false, false) => RoughTruth::CertainlyFalse,
            (true, false) => unreachable!("lower ⊆ upper"),
        }
    }
    /// Rough conjunction.
    pub fn and(self, other: RoughTruth) -> RoughTruth {
        use RoughTruth::*;
        match (self, other) {
            (CertainlyTrue, CertainlyTrue) => CertainlyTrue,
            (CertainlyFalse, _) | (_, CertainlyFalse) => CertainlyFalse,
            _ => Uncertain,
        }
    }
    /// Rough disjunction.
    pub fn or(self, other: RoughTruth) -> RoughTruth {
        use RoughTruth::*;
        match (self, other) {
            (CertainlyTrue, _) | (_, CertainlyTrue) => CertainlyTrue,
            (CertainlyFalse, CertainlyFalse) => CertainlyFalse,
            _ => Uncertain,
        }
    }
    /// Rough negation.
    pub fn not(self) -> RoughTruth {
        use RoughTruth::*;
        match self {
            CertainlyTrue => CertainlyFalse,
            CertainlyFalse => CertainlyTrue,
            PossiblyTrue => Uncertain,
            Uncertain => Uncertain,
        }
    }
}
/// A covering of the universe: a collection of sets that covers all objects.
#[derive(Debug, Clone)]
pub struct CoveringSystem {
    pub n_objects: usize,
    pub coverings: Vec<HashSet<usize>>,
}
impl CoveringSystem {
    pub fn new(n_objects: usize) -> Self {
        CoveringSystem {
            n_objects,
            coverings: Vec::new(),
        }
    }
    pub fn add_cover(&mut self, cover: HashSet<usize>) {
        self.coverings.push(cover);
    }
    /// Check if this is indeed a covering (every object is in at least one cover).
    pub fn is_valid_covering(&self) -> bool {
        (0..self.n_objects).all(|x| self.coverings.iter().any(|c| c.contains(&x)))
    }
    /// Minimal description of x: minimal covers containing x.
    pub fn minimal_description(&self, x: usize) -> Vec<&HashSet<usize>> {
        let containing: Vec<&HashSet<usize>> =
            self.coverings.iter().filter(|c| c.contains(&x)).collect();
        containing
            .iter()
            .filter(|&&c| {
                !containing
                    .iter()
                    .any(|&d| !std::ptr::eq(c, d) && d.is_subset(c) && d.contains(&x))
            })
            .copied()
            .collect()
    }
    /// Covering-based lower approximation: { x | N(x) ⊆ X }.
    pub fn lower_approximation(&self, target: &HashSet<usize>) -> HashSet<usize> {
        (0..self.n_objects)
            .filter(|&x| {
                self.minimal_description(x)
                    .iter()
                    .all(|md| md.is_subset(target))
            })
            .collect()
    }
    /// Covering-based upper approximation: { x | ∃ cover C containing x s.t. C ∩ X ≠ ∅ }.
    pub fn upper_approximation(&self, target: &HashSet<usize>) -> HashSet<usize> {
        (0..self.n_objects)
            .filter(|&x| {
                self.coverings
                    .iter()
                    .filter(|c| c.contains(&x))
                    .any(|c| c.intersection(target).next().is_some())
            })
            .collect()
    }
}
/// Bayesian (decision-theoretic) rough set approximation.
/// Uses thresholds α ∈ (0.5, 1] and β ∈ [0, 0.5) for three-way decisions.
#[derive(Debug, Clone)]
pub struct BayesianRoughSet {
    pub alpha: f64,
    pub beta: f64,
}
impl BayesianRoughSet {
    pub fn new(alpha: f64, beta: f64) -> Self {
        assert!(alpha > 0.5 && alpha <= 1.0);
        assert!((0.0..0.5).contains(&beta));
        BayesianRoughSet { alpha, beta }
    }
    /// Compute conditional probability P(X | \[x\]_B) for object x.
    pub fn conditional_prob(cls: &HashSet<usize>, target: &HashSet<usize>) -> f64 {
        if cls.is_empty() {
            return 0.0;
        }
        cls.intersection(target).count() as f64 / cls.len() as f64
    }
    /// Three-way decision: Positive, Boundary, or Negative region.
    pub fn classify(
        &self,
        info: &InformationSystem,
        attrs: &[usize],
        target: &HashSet<usize>,
    ) -> (HashSet<usize>, HashSet<usize>, HashSet<usize>) {
        let classes = info.indiscernibility_classes(attrs);
        let mut pos = HashSet::new();
        let mut neg = HashSet::new();
        let mut bnd = HashSet::new();
        for cls in &classes {
            let p = Self::conditional_prob(cls, target);
            if p >= self.alpha {
                pos.extend(cls.iter().copied());
            } else if p < self.beta {
                neg.extend(cls.iter().copied());
            } else {
                bnd.extend(cls.iter().copied());
            }
        }
        (pos, bnd, neg)
    }
}
/// Variable precision rough set approximation with precision l (0 ≤ l < 0.5).
/// Lower_l(X) = { o | rel_overlap(\[o\]_B, X) ≤ l } where rel_overlap = |\[o\]∩X^c|/|\[o\]|.
#[derive(Debug, Clone)]
pub struct VPRSApproximation {
    pub precision: f64,
}
impl VPRSApproximation {
    pub fn new(precision: f64) -> Self {
        assert!((0.0..0.5).contains(&precision));
        VPRSApproximation { precision }
    }
    /// Compute the l-lower approximation.
    pub fn lower_approximation(
        &self,
        info: &InformationSystem,
        attrs: &[usize],
        target: &HashSet<usize>,
    ) -> HashSet<usize> {
        let classes = info.indiscernibility_classes(attrs);
        let mut result = HashSet::new();
        for cls in &classes {
            let complement_overlap = cls.difference(target).count();
            let relative_overlap = complement_overlap as f64 / cls.len() as f64;
            if relative_overlap <= self.precision {
                for &obj in cls {
                    result.insert(obj);
                }
            }
        }
        result
    }
    /// Compute the l-upper approximation.
    pub fn upper_approximation(
        &self,
        info: &InformationSystem,
        attrs: &[usize],
        target: &HashSet<usize>,
    ) -> HashSet<usize> {
        let classes = info.indiscernibility_classes(attrs);
        let mut result = HashSet::new();
        for cls in &classes {
            let overlap = cls.intersection(target).count();
            let relative_overlap = 1.0 - (overlap as f64 / cls.len() as f64);
            if relative_overlap <= self.precision {
                for &obj in cls {
                    result.insert(obj);
                }
            }
        }
        result
    }
}
/// A granule: a basic unit of knowledge (a subset of objects).
#[derive(Debug, Clone)]
pub struct Granule {
    pub objects: HashSet<usize>,
    pub label: String,
}
impl Granule {
    pub fn new(label: impl Into<String>, objects: HashSet<usize>) -> Self {
        Granule {
            objects,
            label: label.into(),
        }
    }
    pub fn size(&self) -> usize {
        self.objects.len()
    }
    pub fn intersects(&self, other: &Granule) -> bool {
        self.objects.intersection(&other.objects).next().is_some()
    }
    pub fn is_subgranule_of(&self, other: &Granule) -> bool {
        self.objects.is_subset(&other.objects)
    }
}
/// A neighborhood system: for each object, a set of neighbors.
#[derive(Debug, Clone)]
pub struct NeighborhoodSystem {
    pub n_objects: usize,
    /// neighborhoods\[x\] = set of objects in the neighborhood of x.
    pub neighborhoods: Vec<HashSet<usize>>,
}
impl NeighborhoodSystem {
    pub fn new(n_objects: usize) -> Self {
        NeighborhoodSystem {
            n_objects,
            neighborhoods: vec![HashSet::new(); n_objects],
        }
    }
    /// Set the neighborhood of object x.
    pub fn set_neighborhood(&mut self, x: usize, neighbors: HashSet<usize>) {
        self.neighborhoods[x] = neighbors;
    }
    /// Add object y to the neighborhood of x.
    pub fn add_neighbor(&mut self, x: usize, y: usize) {
        self.neighborhoods[x].insert(y);
    }
    /// Build from a similarity relation: x and y are neighbors if sim(x,y) ≥ threshold.
    pub fn from_similarity(sim: &[Vec<f64>], threshold: f64) -> Self {
        let n = sim.len();
        let mut ns = NeighborhoodSystem::new(n);
        for x in 0..n {
            for y in 0..n {
                if sim[x][y] >= threshold {
                    ns.add_neighbor(x, y);
                }
            }
        }
        ns
    }
    /// Lower approximation using neighborhood system.
    pub fn lower_approximation(&self, target: &HashSet<usize>) -> HashSet<usize> {
        (0..self.n_objects)
            .filter(|&x| self.neighborhoods[x].is_subset(target))
            .collect()
    }
    /// Upper approximation using neighborhood system.
    pub fn upper_approximation(&self, target: &HashSet<usize>) -> HashSet<usize> {
        (0..self.n_objects)
            .filter(|&x| self.neighborhoods[x].intersection(target).next().is_some())
            .collect()
    }
    /// Check if the neighborhood system is reflexive (x ∈ N(x) for all x).
    pub fn is_reflexive(&self) -> bool {
        (0..self.n_objects).all(|x| self.neighborhoods[x].contains(&x))
    }
    /// Check if the system is symmetric (y ∈ N(x) ↔ x ∈ N(y)).
    pub fn is_symmetric(&self) -> bool {
        for x in 0..self.n_objects {
            for &y in &self.neighborhoods[x] {
                if !self.neighborhoods[y].contains(&x) {
                    return false;
                }
            }
        }
        true
    }
    /// Check transitivity: y ∈ N(x) and z ∈ N(y) → z ∈ N(x).
    pub fn is_transitive(&self) -> bool {
        for x in 0..self.n_objects {
            let nx = self.neighborhoods[x].clone();
            for y in &nx {
                for &z in &self.neighborhoods[*y] {
                    if !self.neighborhoods[x].contains(&z) {
                        return false;
                    }
                }
            }
        }
        true
    }
}
/// A dominance-based approximation space.
/// Attribute values are ordinal (higher = better for gain-type attributes).
#[derive(Debug, Clone)]
pub struct DominanceRoughSet {
    pub info: InformationSystem,
    /// Gain-type attributes (higher value is preferred).
    pub gain_attrs: Vec<usize>,
    /// Cost-type attributes (lower value is preferred).
    pub cost_attrs: Vec<usize>,
}
impl DominanceRoughSet {
    pub fn new(info: InformationSystem, gain_attrs: Vec<usize>, cost_attrs: Vec<usize>) -> Self {
        DominanceRoughSet {
            info,
            gain_attrs,
            cost_attrs,
        }
    }
    /// Check if object x dominates object y w.r.t. the given attribute sets.
    /// x ≥_D y if for all gain attrs a: val(x,a) ≥ val(y,a) and
    ///           for all cost attrs a: val(x,a) ≤ val(y,a).
    pub fn dominates(&self, x: usize, y: usize) -> bool {
        self.gain_attrs
            .iter()
            .all(|&a| self.info.get(x, a) >= self.info.get(y, a))
            && self
                .cost_attrs
                .iter()
                .all(|&a| self.info.get(x, a) <= self.info.get(y, a))
    }
    /// Dominance class of object x: D^+(x) = { y | x dominates y }.
    pub fn dominance_class_up(&self, x: usize) -> HashSet<usize> {
        (0..self.info.n_objects)
            .filter(|&y| self.dominates(x, y))
            .collect()
    }
    /// Dominated class of object x: D^-(x) = { y | y dominates x }.
    pub fn dominance_class_down(&self, x: usize) -> HashSet<usize> {
        (0..self.info.n_objects)
            .filter(|&y| self.dominates(y, x))
            .collect()
    }
    /// DRSA lower approximation of an upward union of decision classes Cl_t^≥.
    pub fn lower_approx_upward_union(&self, target: &HashSet<usize>) -> HashSet<usize> {
        let mut result = HashSet::new();
        for x in 0..self.info.n_objects {
            let dp = self.dominance_class_up(x);
            if dp.is_subset(target) {
                result.insert(x);
            }
        }
        result
    }
    /// DRSA upper approximation of an upward union of decision classes Cl_t^≥.
    pub fn upper_approx_upward_union(&self, target: &HashSet<usize>) -> HashSet<usize> {
        let mut result = HashSet::new();
        for x in 0..self.info.n_objects {
            let dm = self.dominance_class_down(x);
            if dm.intersection(target).next().is_some() {
                result.insert(x);
            }
        }
        result
    }
}
/// A granule structure: a collection of granules at a given level of granularity.
#[derive(Debug, Clone)]
pub struct GranuleStructure {
    pub granules: Vec<Granule>,
    pub universe_size: usize,
}
impl GranuleStructure {
    pub fn new(universe_size: usize) -> Self {
        GranuleStructure {
            granules: Vec::new(),
            universe_size,
        }
    }
    pub fn add_granule(&mut self, granule: Granule) {
        self.granules.push(granule);
    }
    /// Check if the granules form a partition.
    pub fn is_partition(&self) -> bool {
        let mut counts = vec![0usize; self.universe_size];
        for g in &self.granules {
            for &o in &g.objects {
                if o < self.universe_size {
                    counts[o] += 1;
                }
            }
        }
        counts.iter().all(|&c| c == 1)
    }
    /// Find the granule containing object x (first match).
    pub fn find_granule(&self, x: usize) -> Option<&Granule> {
        self.granules.iter().find(|g| g.objects.contains(&x))
    }
    /// Compute lower approximation from the granule structure.
    pub fn lower_approximation(&self, target: &HashSet<usize>) -> HashSet<usize> {
        let mut result = HashSet::new();
        for g in &self.granules {
            if g.objects.is_subset(target) {
                result.extend(g.objects.iter().copied());
            }
        }
        result
    }
    /// Compute upper approximation from the granule structure.
    pub fn upper_approximation(&self, target: &HashSet<usize>) -> HashSet<usize> {
        let mut result = HashSet::new();
        for g in &self.granules {
            if g.objects.intersection(target).next().is_some() {
                result.extend(g.objects.iter().copied());
            }
        }
        result
    }
}
/// An information system (approximation space).
/// Objects are 0..n_objects, attributes are 0..n_attributes.
/// Each cell `table[obj][attr]` holds the attribute value.
#[derive(Debug, Clone)]
pub struct InformationSystem {
    pub n_objects: usize,
    pub n_attributes: usize,
    /// Attribute values as integers (categorical).
    pub table: Vec<Vec<u32>>,
    /// Optional attribute names.
    pub attr_names: Vec<String>,
    /// Optional object names.
    pub obj_names: Vec<String>,
}
impl InformationSystem {
    /// Create a new empty information system.
    pub fn new(n_objects: usize, n_attributes: usize) -> Self {
        InformationSystem {
            n_objects,
            n_attributes,
            table: vec![vec![0u32; n_attributes]; n_objects],
            attr_names: (0..n_attributes).map(|i| format!("a{i}")).collect(),
            obj_names: (0..n_objects).map(|i| format!("x{i}")).collect(),
        }
    }
    /// Set the value of attribute `attr` for object `obj`.
    pub fn set(&mut self, obj: usize, attr: usize, val: u32) {
        self.table[obj][attr] = val;
    }
    /// Get the value of attribute `attr` for object `obj`.
    pub fn get(&self, obj: usize, attr: usize) -> u32 {
        self.table[obj][attr]
    }
    /// Compute the indiscernibility relation for a set of attributes.
    /// Returns equivalence classes as sets of object indices.
    pub fn indiscernibility_classes(&self, attrs: &[usize]) -> Vec<HashSet<usize>> {
        let mut visited = vec![false; self.n_objects];
        let mut classes = Vec::new();
        for i in 0..self.n_objects {
            if visited[i] {
                continue;
            }
            let mut cls = HashSet::new();
            cls.insert(i);
            for j in (i + 1)..self.n_objects {
                if attrs.iter().all(|&a| self.table[i][a] == self.table[j][a]) {
                    cls.insert(j);
                    visited[j] = true;
                }
            }
            visited[i] = true;
            classes.push(cls);
        }
        classes
    }
    /// Lower approximation of a target set X w.r.t. attributes.
    /// Lower(X) = { o | \[o\]_B ⊆ X }
    pub fn lower_approximation(&self, attrs: &[usize], target: &HashSet<usize>) -> HashSet<usize> {
        let classes = self.indiscernibility_classes(attrs);
        let mut result = HashSet::new();
        for cls in &classes {
            if cls.is_subset(target) {
                for &obj in cls {
                    result.insert(obj);
                }
            }
        }
        result
    }
    /// Upper approximation of a target set X w.r.t. attributes.
    /// Upper(X) = { o | \[o\]_B ∩ X ≠ ∅ }
    pub fn upper_approximation(&self, attrs: &[usize], target: &HashSet<usize>) -> HashSet<usize> {
        let classes = self.indiscernibility_classes(attrs);
        let mut result = HashSet::new();
        for cls in &classes {
            if cls.intersection(target).next().is_some() {
                for &obj in cls {
                    result.insert(obj);
                }
            }
        }
        result
    }
    /// Boundary region: upper approximation \ lower approximation.
    pub fn boundary_region(&self, attrs: &[usize], target: &HashSet<usize>) -> HashSet<usize> {
        let lower = self.lower_approximation(attrs, target);
        let upper = self.upper_approximation(attrs, target);
        upper.difference(&lower).copied().collect()
    }
    /// Accuracy of approximation: |Lower(X)| / |Upper(X)|.
    pub fn accuracy(&self, attrs: &[usize], target: &HashSet<usize>) -> f64 {
        let lower = self.lower_approximation(attrs, target);
        let upper = self.upper_approximation(attrs, target);
        if upper.is_empty() {
            1.0
        } else {
            lower.len() as f64 / upper.len() as f64
        }
    }
    /// Quality of approximation (dependency degree): fraction of objects
    /// that can be classified.
    pub fn quality_of_approximation(
        &self,
        condition_attrs: &[usize],
        decision_attrs: &[usize],
    ) -> f64 {
        let decision_classes = self.indiscernibility_classes(decision_attrs);
        let classified: HashSet<usize> = decision_classes
            .iter()
            .flat_map(|cls| self.lower_approximation(condition_attrs, cls))
            .collect();
        classified.len() as f64 / self.n_objects as f64
    }
}
/// Multi-granulation rough set: uses multiple equivalence relations.
#[derive(Debug, Clone)]
pub struct MultiGranulationRoughSet {
    pub info: InformationSystem,
    pub granulations: Vec<Vec<usize>>,
}
impl MultiGranulationRoughSet {
    pub fn new(info: InformationSystem, granulations: Vec<Vec<usize>>) -> Self {
        MultiGranulationRoughSet { info, granulations }
    }
    /// Optimistic multi-granulation lower approximation:
    /// ∑_i Lower_i(X) (union of lower approximations).
    pub fn optimistic_lower(&self, target: &HashSet<usize>) -> HashSet<usize> {
        let mut result = HashSet::new();
        for attrs in &self.granulations {
            let lower = self.info.lower_approximation(attrs, target);
            result.extend(lower);
        }
        result
    }
    /// Pessimistic multi-granulation lower approximation:
    /// ∏_i Lower_i(X) (intersection of lower approximations).
    pub fn pessimistic_lower(&self, target: &HashSet<usize>) -> HashSet<usize> {
        let lowers: Vec<HashSet<usize>> = self
            .granulations
            .iter()
            .map(|attrs| self.info.lower_approximation(attrs, target))
            .collect();
        if lowers.is_empty() {
            return HashSet::new();
        }
        let first = lowers[0].clone();
        lowers.iter().skip(1).fold(first, |acc, lower| {
            acc.intersection(lower).copied().collect()
        })
    }
    /// Optimistic upper approximation: intersection of upper approximations.
    pub fn optimistic_upper(&self, target: &HashSet<usize>) -> HashSet<usize> {
        let uppers: Vec<HashSet<usize>> = self
            .granulations
            .iter()
            .map(|attrs| self.info.upper_approximation(attrs, target))
            .collect();
        if uppers.is_empty() {
            return HashSet::new();
        }
        let first = uppers[0].clone();
        uppers.iter().skip(1).fold(first, |acc, upper| {
            acc.intersection(upper).copied().collect()
        })
    }
    /// Pessimistic upper approximation: union of upper approximations.
    pub fn pessimistic_upper(&self, target: &HashSet<usize>) -> HashSet<usize> {
        let mut result = HashSet::new();
        for attrs in &self.granulations {
            let upper = self.info.upper_approximation(attrs, target);
            result.extend(upper);
        }
        result
    }
}
/// Discernibility matrix entry.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DiscernibilityMatrixExt {
    pub n: usize,
    pub entries: Vec<Vec<Vec<usize>>>,
}
#[allow(dead_code)]
impl DiscernibilityMatrixExt {
    pub fn from_table(table: &DecisionTableExt) -> Self {
        let n = table.n_objects;
        let all_attrs: Vec<usize> = (0..table.n_attrs()).collect();
        let mut entries = vec![vec![Vec::new(); n]; n];
        for i in 0..n {
            for j in (i + 1)..n {
                if table.decisions[i] != table.decisions[j] {
                    let disc: Vec<usize> = all_attrs
                        .iter()
                        .filter(|&&a| table.data[i][a] != table.data[j][a])
                        .cloned()
                        .collect();
                    entries[i][j] = disc.clone();
                    entries[j][i] = disc;
                }
            }
        }
        DiscernibilityMatrixExt { n, entries }
    }
    pub fn get(&self, i: usize, j: usize) -> &[usize] {
        &self.entries[i][j]
    }
    pub fn is_consistent(&self) -> bool {
        for i in 0..self.n {
            for j in (i + 1)..self.n {
                if !self.entries[i][j].is_empty() {
                    return true;
                }
            }
        }
        false
    }
}
/// A decision table: an information system with a designated decision attribute.
#[derive(Debug, Clone)]
pub struct DecisionTable {
    pub info: InformationSystem,
    /// Index of the decision attribute.
    pub decision_attr: usize,
}
impl DecisionTable {
    pub fn new(info: InformationSystem, decision_attr: usize) -> Self {
        DecisionTable {
            info,
            decision_attr,
        }
    }
    /// Condition attributes (all attributes except the decision attribute).
    pub fn condition_attrs(&self) -> Vec<usize> {
        (0..self.info.n_attributes)
            .filter(|&a| a != self.decision_attr)
            .collect()
    }
    /// Dependency degree γ(C, D): quality of approximation of D by C.
    pub fn dependency_degree(&self) -> f64 {
        let cond = self.condition_attrs();
        self.info
            .quality_of_approximation(&cond, &[self.decision_attr])
    }
    /// Check if a subset of condition attributes is a reduct.
    /// B is a reduct if γ(B, D) = γ(C, D) and no proper subset of B has this.
    pub fn is_reduct(&self, attrs: &[usize]) -> bool {
        let full_dep = self.dependency_degree();
        let dep = self
            .info
            .quality_of_approximation(attrs, &[self.decision_attr]);
        if (dep - full_dep).abs() > 1e-9 {
            return false;
        }
        for i in 0..attrs.len() {
            let reduced: Vec<usize> = attrs
                .iter()
                .enumerate()
                .filter(|(j, _)| *j != i)
                .map(|(_, &a)| a)
                .collect();
            let dep_reduced = self
                .info
                .quality_of_approximation(&reduced, &[self.decision_attr]);
            if (dep_reduced - full_dep).abs() < 1e-9 {
                return false;
            }
        }
        true
    }
    /// Compute all reducts by exhaustive search (exponential, small tables only).
    pub fn all_reducts(&self) -> Vec<Vec<usize>> {
        let cond = self.condition_attrs();
        let n = cond.len();
        let full_dep = self.dependency_degree();
        let mut reducts = Vec::new();
        for mask in 1u64..(1u64 << n) {
            let subset: Vec<usize> = (0..n)
                .filter(|&i| (mask >> i) & 1 == 1)
                .map(|i| cond[i])
                .collect();
            let dep = self
                .info
                .quality_of_approximation(&subset, &[self.decision_attr]);
            if (dep - full_dep).abs() < 1e-9 && self.is_reduct(&subset) {
                reducts.push(subset);
            }
        }
        reducts
    }
    /// Core: intersection of all reducts.
    pub fn core(&self) -> Vec<usize> {
        let reducts = self.all_reducts();
        if reducts.is_empty() {
            return vec![];
        }
        let first: HashSet<usize> = reducts[0].iter().copied().collect();
        let core: HashSet<usize> = reducts.iter().skip(1).fold(first, |acc, r| {
            let rset: HashSet<usize> = r.iter().copied().collect();
            acc.intersection(&rset).copied().collect()
        });
        let mut v: Vec<usize> = core.into_iter().collect();
        v.sort_unstable();
        v
    }
    /// Compute decision rules from the table.
    /// Returns a vector of (condition_map, decision_value) pairs.
    pub fn extract_rules(&self) -> Vec<(HashMap<usize, u32>, u32)> {
        let cond = self.condition_attrs();
        let mut rules = Vec::new();
        let classes = self.info.indiscernibility_classes(&cond);
        for cls in &classes {
            let obj = *cls
                .iter()
                .next()
                .expect("cls is a non-empty indiscernibility class");
            let cond_map: HashMap<usize, u32> =
                cond.iter().map(|&a| (a, self.info.get(obj, a))).collect();
            let decision = self.info.get(obj, self.decision_attr);
            rules.push((cond_map, decision));
        }
        rules
    }
}
