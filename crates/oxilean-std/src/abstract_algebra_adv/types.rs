//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};
use std::collections::HashMap;

use super::functions::*;

/// A ℤ-graded ring represented as a map from degree to a list of generators
/// for each homogeneous component.
///
/// Elements are represented as `(degree, coefficient)` pairs where
/// coefficients are integers.
#[derive(Debug, Clone)]
pub struct GradedRing {
    /// Homogeneous components: degree → list of element names/labels
    pub components: HashMap<i64, Vec<String>>,
    /// The base ring name (e.g., "Z", "Q", "k")
    pub base_ring: String,
}
impl GradedRing {
    /// Create a new graded ring over the given base ring.
    pub fn new(base_ring: impl Into<String>) -> Self {
        Self {
            components: HashMap::new(),
            base_ring: base_ring.into(),
        }
    }
    /// Add a homogeneous generator of the given degree.
    pub fn add_generator(&mut self, degree: i64, name: impl Into<String>) {
        self.components.entry(degree).or_default().push(name.into());
    }
    /// Return the rank (number of generators) of the n-th homogeneous component.
    pub fn component_rank(&self, degree: i64) -> usize {
        self.components.get(&degree).map_or(0, Vec::len)
    }
    /// Return the sorted list of degrees that appear in this graded ring.
    pub fn degrees(&self) -> Vec<i64> {
        let mut ds: Vec<i64> = self.components.keys().copied().collect();
        ds.sort_unstable();
        ds
    }
    /// Check whether this graded ring is concentrated in non-negative degrees
    /// (a necessary condition for a standard graded ring).
    pub fn is_standard_graded(&self) -> bool {
        self.components.keys().all(|&d| d >= 0)
    }
}
/// Represents a primary decomposition of an ideal over Z/pZ\[x₁,…,xₙ\].
///
/// Each component is a primary ideal given by its radical (a prime) and
/// the primary ideal itself, represented as lists of generator strings.
#[derive(Debug, Clone)]
pub struct PrimaryDecompositionData {
    /// The prime p for Z/pZ
    pub prime: u64,
    /// The ideal being decomposed (as generator strings)
    pub ideal_gens: Vec<String>,
    /// The primary components: list of (primary_gens, radical_gens) pairs
    pub components: Vec<(Vec<String>, Vec<String>)>,
}
impl PrimaryDecompositionData {
    /// Create a new primary decomposition structure.
    pub fn new(prime: u64, ideal_gens: Vec<String>) -> Self {
        Self {
            prime,
            ideal_gens,
            components: Vec::new(),
        }
    }
    /// Add a primary component (primary ideal gens, radical/associated prime gens).
    pub fn add_component(&mut self, primary_gens: Vec<String>, radical_gens: Vec<String>) {
        self.components.push((primary_gens, radical_gens));
    }
    /// Return the number of primary components.
    pub fn num_components(&self) -> usize {
        self.components.len()
    }
    /// Check whether the decomposition is irredundant: no component's radical
    /// contains another's radical.  (Simplified check by string comparison.)
    pub fn is_irredundant(&self) -> bool {
        let radicals: Vec<&Vec<String>> = self.components.iter().map(|(_, r)| r).collect();
        for (i, ri) in radicals.iter().enumerate() {
            for (j, rj) in radicals.iter().enumerate() {
                if i != j && ri.iter().all(|g| rj.contains(g)) {
                    return false;
                }
            }
        }
        true
    }
}
/// A Hopf algebra represented by its comultiplication table on a basis.
///
/// The basis elements are named strings. Comultiplication Δ(b) is stored as
/// a list of pairs (bᵢ, bⱼ) representing the tensor b_i ⊗ b_j summands.
/// The antipode S is stored as a map from basis element to its image.
#[derive(Debug, Clone)]
pub struct HopfAlgebraData {
    /// The name of the Hopf algebra (e.g., "k\[G\]" for a group algebra)
    pub name: String,
    /// Basis elements
    pub basis: Vec<String>,
    /// Comultiplication table: basis_name → list of (left, right) tensor pairs
    pub comultiplication: HashMap<String, Vec<(String, String)>>,
    /// Antipode table: basis_name → antipode image (as basis element name)
    pub antipode: HashMap<String, String>,
    /// Counit table: basis_name → coefficient (0 or 1 for group-like elements)
    pub counit: HashMap<String, i64>,
}
impl HopfAlgebraData {
    /// Create an empty Hopf algebra with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            basis: Vec::new(),
            comultiplication: HashMap::new(),
            antipode: HashMap::new(),
            counit: HashMap::new(),
        }
    }
    /// Add a group-like element g: Δ(g) = g⊗g, ε(g) = 1, S(g) = g⁻¹.
    pub fn add_group_like(&mut self, elem: impl Into<String>, inverse: impl Into<String>) {
        let e = elem.into();
        let inv = inverse.into();
        self.basis.push(e.clone());
        self.comultiplication
            .insert(e.clone(), vec![(e.clone(), e.clone())]);
        self.counit.insert(e.clone(), 1);
        self.antipode.insert(e.clone(), inv);
    }
    /// Add a primitive element x: Δ(x) = x⊗1 + 1⊗x, ε(x) = 0, S(x) = -x.
    /// Here "1" is the unit element name.
    pub fn add_primitive(&mut self, elem: impl Into<String>, unit: &str) {
        let e = elem.into();
        self.basis.push(e.clone());
        self.comultiplication.insert(
            e.clone(),
            vec![(e.clone(), unit.to_string()), (unit.to_string(), e.clone())],
        );
        self.counit.insert(e.clone(), 0);
        self.antipode.insert(e.clone(), format!("-{}", e));
    }
    /// Verify the counit axiom for a group-like element: ε(g) = 1.
    pub fn check_counit_group_like(&self, elem: &str) -> bool {
        self.counit.get(elem).copied() == Some(1)
    }
    /// Return the dimension (number of basis elements).
    pub fn dimension(&self) -> usize {
        self.basis.len()
    }
    /// Check if the algebra is cocommutative: Δ(b) = τ(Δ(b)) for all basis b.
    /// Here cocommutativity means every tensor pair (x,y) also appears as (y,x).
    pub fn is_cocommutative(&self) -> bool {
        for pairs in self.comultiplication.values() {
            for (l, r) in pairs {
                if !pairs.contains(&(r.clone(), l.clone())) {
                    return false;
                }
            }
        }
        true
    }
}
/// Estimates the Krull dimension of a ring by tracking chains of prime ideals.
///
/// A prime ideal is represented here as a set of generator strings.
/// This is a combinatorial estimator: it finds the longest strict chain
/// in a supplied list of prime ideals (ordered by set inclusion of generators).
#[derive(Debug, Clone)]
pub struct KrullDimEstimator {
    /// The prime ideals of the ring, each given as a list of generators.
    pub primes: Vec<Vec<String>>,
}
impl KrullDimEstimator {
    /// Create an estimator with no primes.
    pub fn new() -> Self {
        Self { primes: Vec::new() }
    }
    /// Add a prime ideal by its generator list.
    pub fn add_prime(&mut self, gens: Vec<String>) {
        self.primes.push(gens);
    }
    /// Check strict inclusion: does prime `a` strictly contain prime `b`?
    fn strictly_contains(a: &[String], b: &[String]) -> bool {
        b.iter().all(|g| a.contains(g)) && a.len() > b.len()
    }
    /// Compute the length of the longest ascending chain p₀ ⊊ p₁ ⊊ … ⊊ pₙ
    /// (so Krull dimension = chain length = n).
    ///
    /// Uses dynamic programming on the DAG of inclusions.
    pub fn estimate_krull_dim(&self) -> usize {
        let n = self.primes.len();
        if n == 0 {
            return 0;
        }
        let mut dp = vec![0usize; n];
        for i in 0..n {
            for j in 0..i {
                if Self::strictly_contains(&self.primes[i], &self.primes[j]) {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
        }
        *dp.iter().max().unwrap_or(&0)
    }
    /// Return all maximal chains as lists of prime indices.
    pub fn maximal_chains(&self) -> Vec<Vec<usize>> {
        let n = self.primes.len();
        let mut chains: Vec<Vec<usize>> = Vec::new();
        for start in 0..n {
            self.build_chains(start, vec![start], &mut chains);
        }
        let snapshot = chains.clone();
        chains.retain(|c| {
            !snapshot
                .iter()
                .any(|other| other.len() > c.len() && other[..c.len()] == c[..])
        });
        chains
    }
    fn build_chains(&self, current: usize, chain: Vec<usize>, result: &mut Vec<Vec<usize>>) {
        let mut extended = false;
        for next in 0..self.primes.len() {
            if !chain.contains(&next)
                && Self::strictly_contains(&self.primes[next], &self.primes[current])
            {
                let mut new_chain = chain.clone();
                new_chain.push(next);
                self.build_chains(next, new_chain, result);
                extended = true;
            }
        }
        if !extended {
            result.push(chain);
        }
    }
}
/// A local ring represented by its maximal ideal generators and residue field.
///
/// Used to check Nakayama-type conditions and compute local invariants.
#[derive(Debug, Clone)]
pub struct LocalRing {
    /// Name of the ring (e.g., "Z_(p)", "k[\[x\]]")
    pub name: String,
    /// Generators of the unique maximal ideal m
    pub maximal_ideal_gens: Vec<String>,
    /// Name of the residue field k = R/m
    pub residue_field: String,
    /// Krull dimension (if known)
    pub krull_dim: Option<u32>,
}
impl LocalRing {
    /// Create a new local ring with given maximal ideal generators.
    pub fn new(
        name: impl Into<String>,
        maximal_ideal_gens: Vec<String>,
        residue_field: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            maximal_ideal_gens,
            residue_field: residue_field.into(),
            krull_dim: None,
        }
    }
    /// Set the Krull dimension.
    pub fn with_krull_dim(mut self, d: u32) -> Self {
        self.krull_dim = Some(d);
        self
    }
    /// The embedding dimension: the minimal number of generators of m.
    /// For a regular local ring, this equals the Krull dimension.
    pub fn embedding_dimension(&self) -> usize {
        self.maximal_ideal_gens.len()
    }
    /// A ring is regular local when embedding dimension equals Krull dimension.
    pub fn is_regular(&self) -> bool {
        match self.krull_dim {
            Some(d) => self.embedding_dimension() == d as usize,
            None => false,
        }
    }
    /// Check whether an element (given by name) lies in the maximal ideal.
    pub fn is_in_maximal_ideal(&self, element: &str) -> bool {
        self.maximal_ideal_gens.iter().any(|g| g == element)
    }
}
/// A graded module M over a graded ring, storing the rank of each homogeneous
/// component. Supports computation of the Poincaré series as a formal polynomial.
#[derive(Debug, Clone)]
pub struct GradedModule {
    /// Map from degree to rank of that homogeneous component.
    pub ranks: HashMap<i64, usize>,
    /// The name/label of this module.
    pub name: String,
}
impl GradedModule {
    /// Create a new graded module with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            ranks: HashMap::new(),
            name: name.into(),
        }
    }
    /// Set the rank of the n-th homogeneous component.
    pub fn set_rank(&mut self, degree: i64, rank: usize) {
        self.ranks.insert(degree, rank);
    }
    /// Compute the Poincaré series as a map `degree → rank` (a formal power
    /// series truncated to the known components).
    pub fn poincare_series(&self) -> Vec<(i64, usize)> {
        let mut terms: Vec<(i64, usize)> = self
            .ranks
            .iter()
            .filter(|(_, &r)| r > 0)
            .map(|(&d, &r)| (d, r))
            .collect();
        terms.sort_by_key(|&(d, _)| d);
        terms
    }
    /// Return the Euler characteristic χ = Σ (-1)ⁿ rankₙ over all known degrees.
    pub fn euler_characteristic(&self) -> i64 {
        self.ranks
            .iter()
            .map(|(&d, &r)| if d % 2 == 0 { r as i64 } else { -(r as i64) })
            .sum()
    }
    /// The total rank (sum of all component ranks).
    pub fn total_rank(&self) -> usize {
        self.ranks.values().sum()
    }
}
/// A Galois extension represented by its degree and Galois group generators.
///
/// The Galois group is encoded as a list of automorphism names and their
/// action on a chosen set of field generators.
#[derive(Debug, Clone)]
pub struct GaloisExtensionData {
    /// The base field (e.g., "Q")
    pub base_field: String,
    /// The extension field (e.g., "Q(sqrt(2), sqrt(3))")
    pub extension_field: String,
    /// Degree \[L:K\]
    pub degree: usize,
    /// Generators of L over K (e.g., \["sqrt(2)", "sqrt(3)"\])
    pub generators: Vec<String>,
    /// Automorphism generators: (name, action on generators)
    /// The action is a list of images of each generator.
    pub automorphisms: Vec<(String, Vec<String>)>,
}
impl GaloisExtensionData {
    /// Create a new Galois extension.
    pub fn new(
        base: impl Into<String>,
        ext: impl Into<String>,
        degree: usize,
        generators: Vec<String>,
    ) -> Self {
        Self {
            base_field: base.into(),
            extension_field: ext.into(),
            degree,
            generators,
            automorphisms: Vec::new(),
        }
    }
    /// Add an automorphism with a name and its action on each generator.
    pub fn add_automorphism(&mut self, name: impl Into<String>, action: Vec<String>) {
        self.automorphisms.push((name.into(), action));
    }
    /// The order of the Galois group (number of automorphisms registered).
    pub fn galois_group_order(&self) -> usize {
        self.automorphisms.len()
    }
    /// Check whether the fundamental theorem holds: |Gal(L/K)| = \[L:K\].
    pub fn satisfies_fundamental_theorem(&self) -> bool {
        self.galois_group_order() == self.degree
    }
    /// Return the fixed field corresponding to a subgroup (by automorphism indices).
    /// Returns the names of generators fixed by all listed automorphisms.
    pub fn fixed_generators(&self, subgroup_indices: &[usize]) -> Vec<String> {
        self.generators
            .iter()
            .enumerate()
            .filter(|&(gen_idx, _)| {
                subgroup_indices.iter().all(|&aut_idx| {
                    if let Some((_, action)) = self.automorphisms.get(aut_idx) {
                        action
                            .get(gen_idx)
                            .map(|img| img == &self.generators[gen_idx])
                            .unwrap_or(false)
                    } else {
                        false
                    }
                })
            })
            .map(|(_, g)| g.clone())
            .collect()
    }
}
/// A differential graded algebra (dgA) with an explicit boundary map on basis elements.
///
/// The chain complex is stored as a map from basis element name to the list of
/// (coefficient, target) pairs representing d(b) = Σ cᵢ · bᵢ.
#[derive(Debug, Clone)]
pub struct DGAlgebra {
    /// Name of this dgA
    pub name: String,
    /// Basis elements with their homological degrees
    pub basis: HashMap<String, i64>,
    /// Boundary map: basis_element → list of (coefficient, target_element)
    pub differential: HashMap<String, Vec<(i64, String)>>,
}
impl DGAlgebra {
    /// Create an empty differential graded algebra.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            basis: HashMap::new(),
            differential: HashMap::new(),
        }
    }
    /// Add a basis element with its degree.
    pub fn add_basis(&mut self, name: impl Into<String>, degree: i64) {
        self.basis.insert(name.into(), degree);
    }
    /// Set the differential of a basis element.
    pub fn set_differential(&mut self, elem: impl Into<String>, image: Vec<(i64, String)>) {
        self.differential.insert(elem.into(), image);
    }
    /// Get the degree of a basis element.
    pub fn degree(&self, elem: &str) -> Option<i64> {
        self.basis.get(elem).copied()
    }
    /// Compute d²(b) for a given basis element b and check it is zero.
    /// Returns true if d²(b) = 0 (the zero boundary condition).
    pub fn check_d_squared_zero(&self, elem: &str) -> bool {
        let Some(first_images) = self.differential.get(elem) else {
            return true;
        };
        let mut result: HashMap<String, i64> = HashMap::new();
        for (coeff1, b1) in first_images {
            if let Some(second_images) = self.differential.get(b1) {
                for (coeff2, b2) in second_images {
                    *result.entry(b2.clone()).or_insert(0) += coeff1 * coeff2;
                }
            }
        }
        result.values().all(|&c| c == 0)
    }
    /// Return all basis elements in homological degree n.
    pub fn basis_in_degree(&self, degree: i64) -> Vec<String> {
        self.basis
            .iter()
            .filter(|(_, &d)| d == degree)
            .map(|(name, _)| name.clone())
            .collect()
    }
    /// Compute the Betti numbers (rank of cohomology in each degree) assuming
    /// the complex is over a field (so boundaries = images of d).
    /// Returns a sorted vector of (degree, betti_number) pairs.
    pub fn betti_numbers(&self) -> Vec<(i64, usize)> {
        let mut by_degree: HashMap<i64, Vec<&str>> = HashMap::new();
        for (name, &deg) in &self.basis {
            by_degree.entry(deg).or_default().push(name.as_str());
        }
        let mut result = Vec::new();
        for (&deg, elems) in &by_degree {
            let total = elems.len();
            let bdry_count = self
                .differential
                .iter()
                .filter(|(src, _)| self.basis.get(*src).copied() == Some(deg - 1))
                .flat_map(|(_, imgs)| imgs.iter().filter(|(c, _)| *c != 0))
                .count();
            let cycles = total.saturating_sub(bdry_count);
            result.push((deg, cycles));
        }
        result.sort_by_key(|&(d, _)| d);
        result
    }
}
/// Represents a Koszul complex K(R; f₁,…,fₙ) for elements f₁,…,fₙ in a ring R.
///
/// The Koszul complex is a differential graded algebra built from the exterior
/// algebra on generators e₁,…,eₙ with d(eᵢ) = fᵢ. It is acyclic iff f₁,…,fₙ
/// is a regular sequence.
#[derive(Debug, Clone)]
pub struct KoszulComplex {
    /// The generators f₁, …, fₙ (as string names)
    pub generators: Vec<String>,
    /// Whether the sequence is known to be regular
    pub is_regular_sequence: Option<bool>,
}
impl KoszulComplex {
    /// Create the Koszul complex for the given sequence of ring elements.
    pub fn new(generators: Vec<String>) -> Self {
        Self {
            generators,
            is_regular_sequence: None,
        }
    }
    /// Set whether the underlying sequence is regular.
    pub fn with_regular_sequence(mut self, val: bool) -> Self {
        self.is_regular_sequence = Some(val);
        self
    }
    /// The length n of the sequence.
    pub fn length(&self) -> usize {
        self.generators.len()
    }
    /// The rank of the p-th Koszul module K_p = ∧^p (Rⁿ): binomial(n, p).
    pub fn rank_at(&self, p: usize) -> u64 {
        let n = self.length();
        if p > n {
            return 0;
        }
        let mut result: u64 = 1;
        for i in 0..p {
            result = result * (n - i) as u64 / (i + 1) as u64;
        }
        result
    }
    /// The Euler characteristic Σ (-1)^p rank(K_p) = (1-1)^n.
    /// For n > 0 this is always 0; for n = 0 it is 1.
    pub fn euler_characteristic(&self) -> i64 {
        let n = self.length();
        if n == 0 {
            return 1;
        }
        0
    }
    /// When the sequence is regular, the Koszul complex is a free resolution
    /// of R/(f₁,…,fₙ). Return the Betti numbers in that case.
    pub fn betti_numbers(&self) -> Vec<(usize, u64)> {
        (0..=self.length()).map(|p| (p, self.rank_at(p))).collect()
    }
    /// Check acyclicity (for testing): returns Some(true) if is_regular_sequence is true.
    pub fn is_acyclic(&self) -> Option<bool> {
        self.is_regular_sequence
    }
}
