//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
use super::functions::*;

/// Verifies the symmetric Lovász Local Lemma condition.
///
/// Given n bad events, each with probability at most p, and each
/// depending on at most d others, LLL guarantees a positive probability
/// avoidance if p * (d + 1) ≤ 1 (symmetric version, ep(d+1) ≤ 1 in general).
#[derive(Debug, Clone)]
pub struct LovaszLocalLemma {
    /// Number of bad events.
    pub num_events: usize,
    /// Upper bound on the probability of each event (as 1/denom).
    pub prob_numerator: u64,
    pub prob_denominator: u64,
    /// Maximum dependency degree.
    pub max_degree: usize,
}
impl LovaszLocalLemma {
    /// Create a new LLL instance.
    pub fn new(
        num_events: usize,
        prob_numerator: u64,
        prob_denominator: u64,
        max_degree: usize,
    ) -> Self {
        Self {
            num_events,
            prob_numerator,
            prob_denominator,
            max_degree,
        }
    }
    /// Check the symmetric LLL condition: p * (d + 1) ≤ 1/4.
    ///
    /// Returns true if the condition holds (4 * p * (d+1) ≤ 1).
    pub fn symmetric_condition_holds(&self) -> bool {
        let lhs = 4u64
            .saturating_mul(self.prob_numerator)
            .saturating_mul((self.max_degree + 1) as u64);
        lhs <= self.prob_denominator
    }
    /// Check the asymmetric LLL condition: p * e * (d + 1) ≤ 1.
    ///
    /// Uses e ≈ 271828/100000 ≈ 2.71828.
    pub fn asymmetric_condition_holds(&self) -> bool {
        let numerator = 271828u64
            .saturating_mul(self.prob_numerator)
            .saturating_mul((self.max_degree + 1) as u64);
        let denominator = self.prob_denominator.saturating_mul(100000);
        numerator <= denominator
    }
    /// Return the slack: how far the condition is from the boundary.
    pub fn slack(&self) -> i64 {
        let lhs = 4i64 * self.prob_numerator as i64 * (self.max_degree as i64 + 1);
        self.prob_denominator as i64 - lhs
    }
}
/// A polynomial over i64 stored as coefficient vector: `poly\[i\]` = coefficient of x^i.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Poly(pub Vec<i64>);
impl Poly {
    /// Zero polynomial.
    pub fn zero() -> Self {
        Poly(vec![])
    }
    /// Constant polynomial.
    pub fn constant(c: i64) -> Self {
        if c == 0 {
            Poly::zero()
        } else {
            Poly(vec![c])
        }
    }
    /// Degree (returns 0 for the zero polynomial).
    pub fn degree(&self) -> usize {
        self.0.len().saturating_sub(1)
    }
    /// Evaluate at x using Horner's method.
    pub fn eval(&self, x: i64) -> i64 {
        self.0.iter().rev().fold(0i64, |acc, &c| acc * x + c)
    }
    /// Trim trailing zero coefficients.
    pub fn trim(mut self) -> Self {
        while self.0.last() == Some(&0) {
            self.0.pop();
        }
        self
    }
    /// Add two polynomials.
    pub fn add(&self, other: &Self) -> Self {
        let len = self.0.len().max(other.0.len());
        let mut result = vec![0i64; len];
        for (i, &c) in self.0.iter().enumerate() {
            result[i] += c;
        }
        for (i, &c) in other.0.iter().enumerate() {
            result[i] += c;
        }
        Poly(result).trim()
    }
    /// Multiply two polynomials (naive O(n²)).
    pub fn mul(&self, other: &Self) -> Self {
        if self.0.is_empty() || other.0.is_empty() {
            return Poly::zero();
        }
        let len = self.0.len() + other.0.len() - 1;
        let mut result = vec![0i64; len];
        for (i, &a) in self.0.iter().enumerate() {
            for (j, &b) in other.0.iter().enumerate() {
                result[i + j] += a * b;
            }
        }
        Poly(result).trim()
    }
    /// Compute the first `n+1` terms of the OGF product (Cauchy convolution).
    pub fn conv_truncated(a: &[i64], b: &[i64], n: usize) -> Vec<i64> {
        let mut c = vec![0i64; n + 1];
        for i in 0..=n {
            for j in 0..=i {
                if j < a.len() && (i - j) < b.len() {
                    c[i] += a[j] * b[i - j];
                }
            }
        }
        c
    }
}
/// Computes Turán densities and exact Turán numbers for small graphs.
#[derive(Debug, Clone)]
pub struct TuranDensityComputer {
    /// The forbidden graph (number of vertices in K_{r+1}).
    pub forbidden_clique: usize,
    /// Precomputed upper bounds for various n.
    pub bounds: Vec<(usize, u64)>,
}
impl TuranDensityComputer {
    /// Create a Turán density computer forbidding K_{r+1}.
    pub fn new(r: usize) -> Self {
        Self {
            forbidden_clique: r + 1,
            bounds: Vec::new(),
        }
    }
    /// Compute ex(n, K_{r+1}) = number of edges in the Turán graph T(n,r).
    ///
    /// T(n,r) has n vertices partitioned into r equal parts with all edges between parts.
    /// |E(T(n,r))| = (r-1)/(2r) * n² (asymptotically).
    pub fn turan_edges(&self, n: usize) -> u64 {
        let r = self.forbidden_clique - 1;
        if r == 0 {
            return 0;
        }
        let q = n / r;
        let rem = n % r;
        let total_edges = (n as u64 * (n as u64 - 1)) / 2;
        let intra: u64 = (0..r)
            .map(|i| {
                let part_size = if i < rem { q + 1 } else { q } as u64;
                part_size * (part_size - 1) / 2
            })
            .sum();
        total_edges - intra
    }
    /// Compute the Turán density π(K_{r+1}) = (r-1)/r.
    ///
    /// Returns the numerator and denominator.
    pub fn density(&self) -> (usize, usize) {
        let r = self.forbidden_clique - 1;
        if r <= 1 {
            return (0, 1);
        }
        let g = gcd_usize(r - 1, r);
        ((r - 1) / g, r / g)
    }
    /// Cache a precomputed bound ex(n, K_{r+1}) = bound.
    pub fn cache_bound(&mut self, n: usize, bound: u64) {
        self.bounds.push((n, bound));
    }
}
/// Solves the matroid intersection problem for two partition matroids.
///
/// A partition matroid is defined by a partition of the ground set E into
/// blocks B_1, …, B_k, with rank bounds r_1, …, r_k.
#[derive(Debug, Clone)]
pub struct MatroidIntersectionSolver {
    /// Ground set size |E|.
    pub ground_set_size: usize,
    /// Block assignments: block\[i\] = block index for element i.
    pub block_assignment_1: Vec<usize>,
    /// Rank bounds for matroid 1 (indexed by block).
    pub rank_bounds_1: Vec<usize>,
    /// Block assignments for matroid 2.
    pub block_assignment_2: Vec<usize>,
    /// Rank bounds for matroid 2.
    pub rank_bounds_2: Vec<usize>,
}
impl MatroidIntersectionSolver {
    /// Create a new solver for two partition matroids on ground set {0..n-1}.
    pub fn new(
        n: usize,
        block_assignment_1: Vec<usize>,
        rank_bounds_1: Vec<usize>,
        block_assignment_2: Vec<usize>,
        rank_bounds_2: Vec<usize>,
    ) -> Self {
        Self {
            ground_set_size: n,
            block_assignment_1,
            rank_bounds_1,
            block_assignment_2,
            rank_bounds_2,
        }
    }
    /// Check if a set (represented as a bitmask) is independent in matroid 1.
    pub fn is_independent_1(&self, mask: u64) -> bool {
        let mut counts = vec![0usize; self.rank_bounds_1.len()];
        for i in 0..self.ground_set_size {
            if mask & (1 << i) != 0 {
                let b = self.block_assignment_1[i];
                if b < counts.len() {
                    counts[b] += 1;
                    if counts[b] > self.rank_bounds_1[b] {
                        return false;
                    }
                }
            }
        }
        true
    }
    /// Check if a set is independent in matroid 2.
    pub fn is_independent_2(&self, mask: u64) -> bool {
        let mut counts = vec![0usize; self.rank_bounds_2.len()];
        for i in 0..self.ground_set_size {
            if mask & (1 << i) != 0 {
                let b = self.block_assignment_2[i];
                if b < counts.len() {
                    counts[b] += 1;
                    if counts[b] > self.rank_bounds_2[b] {
                        return false;
                    }
                }
            }
        }
        true
    }
    /// Find the maximum common independent set by brute force (for small |E|).
    ///
    /// Returns (max_size, mask) where mask encodes the optimal set.
    pub fn solve_brute_force(&self) -> (usize, u64) {
        let n = self.ground_set_size.min(20);
        let mut best_size = 0;
        let mut best_mask = 0u64;
        for mask in 0u64..(1 << n) {
            if self.is_independent_1(mask) && self.is_independent_2(mask) {
                let size = mask.count_ones() as usize;
                if size > best_size {
                    best_size = size;
                    best_mask = mask;
                }
            }
        }
        (best_size, best_mask)
    }
}
/// Represents the cycle index polynomial Z(G; x₁, …, xₙ) of a permutation group.
///
/// The cycle index is stored as a list of monomials, where each monomial
/// is represented as a vector of exponents (e\[i\] = exponent of x_{i+1}).
#[derive(Debug, Clone)]
pub struct CycleIndexPolynomial {
    /// Group name (for display).
    pub group_name: String,
    /// Each term: (coefficient, exponent_vector).
    /// The polynomial is (1/|G|) * ∑ coeff * ∏ x_i^{e_i}.
    pub terms: Vec<(u64, Vec<u32>)>,
    /// Order of the group |G|.
    pub group_order: u64,
}
impl CycleIndexPolynomial {
    /// Create a new cycle index for a named group.
    pub fn new(group_name: impl Into<String>, group_order: u64) -> Self {
        Self {
            group_name: group_name.into(),
            terms: Vec::new(),
            group_order,
        }
    }
    /// Add a term to the cycle index.
    pub fn add_term(&mut self, coeff: u64, exponents: Vec<u32>) {
        self.terms.push((coeff, exponents));
    }
    /// Evaluate Z(G; 1, 1, …, 1) = |G| / |G| = 1 (should return 1).
    pub fn eval_all_ones(&self) -> u64 {
        let total: u64 = self.terms.iter().map(|(c, _)| c).sum();
        total / self.group_order.max(1)
    }
    /// Apply Burnside's lemma: the number of distinct colorings with k colors
    /// is Z(G; k, k, …, k).
    ///
    /// Each term `(coeff, exps)` contributes `coeff * ∏_i k^{exps\[i\]}` = `coeff * k^{∑ exps\[i\]}`.
    pub fn burnside_count(&self, k: u64) -> u64 {
        let total: u64 = self
            .terms
            .iter()
            .map(|(coeff, exps)| {
                let sum_exps: u32 = exps.iter().sum();
                let product = k.pow(sum_exps);
                coeff * product
            })
            .sum();
        total / self.group_order.max(1)
    }
    /// The cycle index for C_n (cyclic group of order n).
    pub fn cyclic(n: u32) -> Self {
        let mut cip = Self::new(format!("C_{}", n), n as u64);
        for d in 1..=(n as u64) {
            if n as u64 % d == 0 {
                let phi_d = euler_phi(d);
                let exp_pos = (d as usize).saturating_sub(1);
                let count = n as u64 / d;
                let mut exps = vec![0u32; n as usize];
                if exp_pos < exps.len() {
                    exps[exp_pos] = count as u32;
                }
                cip.add_term(phi_d, exps);
            }
        }
        cip
    }
}
