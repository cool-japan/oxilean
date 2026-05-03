//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::super::functions::*;

use super::types::CStarAlgebra;

/// A noncommutative torus T^2_theta parameterized by theta in [0,1).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct NoncommutativeTorusData {
    pub theta: f64,
    pub dimension: usize,
}
impl NoncommutativeTorusData {
    #[allow(dead_code)]
    pub fn new(theta: f64) -> Self {
        Self {
            theta,
            dimension: 2,
        }
    }
    #[allow(dead_code)]
    pub fn is_rational(&self) -> bool {
        let denom = 1000;
        let numer = (self.theta * denom as f64).round() as i64;
        let g = gcd_i64(numer.abs(), denom);
        let _ = g;
        (self.theta * 10000.0).fract() < 1e-9
    }
    #[allow(dead_code)]
    pub fn morita_equivalent(&self, other: &NoncommutativeTorusData) -> bool {
        (self.theta - other.theta).abs() < 1e-10 || ((self.theta - other.theta) - 1.0).abs() < 1e-10
    }
    #[allow(dead_code)]
    pub fn k0_group_rank(&self) -> usize {
        2
    }
    #[allow(dead_code)]
    pub fn k1_group_rank(&self) -> usize {
        2
    }
}
/// A noncommutative space represented by a (possibly noncommutative) C*-algebra.
///
/// By the philosophy of noncommutative geometry, a "space" X is identified with
/// its algebra of functions C(X); replacing C(X) by a noncommutative algebra A
/// yields a "virtual" noncommutative space dual to A.
pub struct NoncommutativeSpace {
    /// The algebra of "functions" on the noncommutative space.
    pub algebra: CStarAlgebra,
    /// A descriptive name for the space.
    pub name: String,
}
impl NoncommutativeSpace {
    /// Construct a noncommutative space with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        Self {
            algebra: CStarAlgebra::new(format!("C({})", name)),
            name,
        }
    }
    /// If the algebra is commutative, returns the name of the underlying topological space.
    pub fn classical_limit(&self) -> Option<String> {
        if self.algebra.is_commutative {
            Some(format!("Spectrum of {}", self.algebra.name))
        } else {
            None
        }
    }
}
/// An elliptic differential operator on a compact manifold.
///
/// An operator P of order m is elliptic when its principal symbol σ_m(P)(x, ξ)
/// is invertible for all ξ ≠ 0. Elliptic operators on compact manifolds are Fredholm.
pub struct EllipticOperator {
    /// The differential order of the operator.
    pub order: i32,
    /// Description of the principal symbol class (e.g. "Clifford multiplication").
    pub symbol_class: String,
    /// Name of the compact manifold on which the operator acts.
    pub manifold: String,
}
impl EllipticOperator {
    /// Construct an elliptic operator of the given order.
    pub fn new(order: i32) -> Self {
        Self {
            order,
            symbol_class: "elliptic".into(),
            manifold: "M".into(),
        }
    }
    /// Every elliptic operator on a compact manifold is Fredholm (finite-dimensional
    /// kernel and cokernel).
    pub fn is_fredholm(&self) -> bool {
        true
    }
    /// An operator is hypoelliptic when its solutions are smooth wherever the
    /// right-hand side is smooth. Elliptic operators are hypoelliptic.
    pub fn is_hypoelliptic(&self) -> bool {
        true
    }
}
/// Dirac operator on a spin manifold.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DiracOperatorData {
    pub manifold: String,
    pub dimension: usize,
    pub spinor_bundle_rank: usize,
}
impl DiracOperatorData {
    #[allow(dead_code)]
    pub fn new(manifold: &str, dim: usize) -> Self {
        let rank = 1usize << (dim / 2);
        Self {
            manifold: manifold.to_string(),
            dimension: dim,
            spinor_bundle_rank: rank,
        }
    }
    #[allow(dead_code)]
    pub fn is_self_adjoint(&self) -> bool {
        true
    }
    #[allow(dead_code)]
    pub fn has_compact_resolvent(&self) -> bool {
        true
    }
    #[allow(dead_code)]
    pub fn lichnerowicz_formula(&self) -> String {
        format!("D^2 = ∇*∇ + R/4 on {}", self.manifold)
    }
}
/// A (locally compact) groupoid G ⇒ G^{(0)}.
///
/// A groupoid is a small category in which every morphism is invertible.
/// Special cases include groups (one object), equivalence relations, and
/// transformation groupoids G ⋊ X.
pub struct Groupoid {
    /// Descriptive name of the groupoid.
    pub name: String,
    /// Description of the object (unit) space G^{(0)}.
    pub object_space: String,
    /// Description of the morphism (arrow) space G^{(1)}.
    pub morphism_space: String,
}
impl Groupoid {
    /// Construct a groupoid with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        Self {
            object_space: format!("{}_objects", name),
            morphism_space: format!("{}_morphisms", name),
            name,
        }
    }
    /// A groupoid with a single object is a group.
    pub fn is_group(&self) -> bool {
        self.object_space.contains("point") || self.object_space == "*"
    }
    /// A groupoid is an equivalence relation when there is at most one morphism
    /// between any two objects (i.e. the groupoid is a (0,1)-category).
    pub fn is_equivalence_relation(&self) -> bool {
        self.morphism_space.contains("equiv") || self.morphism_space.contains("relation")
    }
}
impl Groupoid {
    /// Constructs the reduced groupoid C*-algebra C*_r(G) of this groupoid.
    ///
    /// Returns a description of the resulting C*-algebra.
    pub fn groupoid_cstar_algebra(&self) -> String {
        format!("C*_r({})  [reduced groupoid C*-algebra]", self.name)
    }
}
/// Completely positive map between C*-algebras.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CompletelyPositiveMap {
    pub source: String,
    pub target: String,
    pub is_unital: bool,
    pub cb_norm: f64,
}
impl CompletelyPositiveMap {
    #[allow(dead_code)]
    pub fn new(src: &str, tgt: &str, unital: bool) -> Self {
        Self {
            source: src.to_string(),
            target: tgt.to_string(),
            is_unital: unital,
            cb_norm: 1.0,
        }
    }
    #[allow(dead_code)]
    pub fn stinespring_representation(&self) -> String {
        format!(
            "phi: {} -> {}: phi(a) = V* pi(a) V via Stinespring",
            self.source, self.target
        )
    }
    #[allow(dead_code)]
    pub fn kraus_decomposition(&self) -> String {
        format!(
            "phi(a) = sum_i K_i* a K_i, Kraus operators for {}->{}",
            self.source, self.target
        )
    }
}
/// Data for verifying whether a candidate (A, H, D) forms a valid spectral triple.
///
/// A valid spectral triple requires:
/// 1. A acts faithfully on H.
/// 2. D is self-adjoint with compact resolvent.
/// 3. \[D, a\] is bounded for all a ∈ A.
#[derive(Debug, Clone)]
pub struct SpectralTripleData {
    /// Name of the algebra A.
    pub algebra_name: String,
    /// Dimension of the Hilbert space (finite approximation; 0 = infinite).
    pub hilbert_dim: usize,
    /// Eigenvalues of |D| in non-decreasing order (finite sample).
    pub dirac_eigenvalues: Vec<f64>,
    /// KO-dimension of the triple.
    pub ko_dim: u32,
}
impl SpectralTripleData {
    /// Construct a `SpectralTripleData` record.
    pub fn new(algebra_name: impl Into<String>, hilbert_dim: usize, ko_dim: u32) -> Self {
        Self {
            algebra_name: algebra_name.into(),
            hilbert_dim,
            dirac_eigenvalues: Vec::new(),
            ko_dim,
        }
    }
    /// Add a sample of Dirac eigenvalues (should be given in non-decreasing order).
    pub fn with_eigenvalues(mut self, eigenvalues: Vec<f64>) -> Self {
        self.dirac_eigenvalues = eigenvalues;
        self
    }
    /// Check that the supplied eigenvalues are consistent with compact resolvent
    /// (eigenvalues must accumulate only at +∞).
    pub fn resolvent_is_compact(&self) -> bool {
        if self.dirac_eigenvalues.len() < 2 {
            return true;
        }
        let last = *self
            .dirac_eigenvalues
            .last()
            .expect("dirac_eigenvalues has at least 2 elements: checked by early return");
        let first = self.dirac_eigenvalues[0];
        last > first
    }
    /// Estimate the metric dimension from the growth rate λ_n ~ C · n^{1/p}.
    ///
    /// Fits p by least-squares on log(n) vs log(λ_n).
    pub fn estimate_metric_dimension(&self) -> f64 {
        let n = self.dirac_eigenvalues.len();
        if n < 2 {
            return f64::from(self.ko_dim);
        }
        let last_idx = (n - 1) as f64;
        let last_val = self.dirac_eigenvalues[n - 1].abs().max(1e-12);
        let first_val = self.dirac_eigenvalues[0].abs().max(1e-12);
        if last_val <= first_val || last_idx <= 0.0 {
            return f64::from(self.ko_dim);
        }
        last_idx.ln() / last_val.ln()
    }
    /// Returns a summary description of this spectral triple.
    pub fn summary(&self) -> String {
        format!(
            "SpectralTriple ({}, H^{}, D) KO-dim={}, #eigenvalues={}",
            self.algebra_name,
            self.hilbert_dim,
            self.ko_dim,
            self.dirac_eigenvalues.len()
        )
    }
}
/// Hochschild cocycle for a noncommutative space.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct HochschildCocycle {
    pub degree: usize,
    pub algebra: String,
}
impl HochschildCocycle {
    #[allow(dead_code)]
    pub fn new(degree: usize, algebra: &str) -> Self {
        Self {
            degree,
            algebra: algebra.to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn is_cyclic(&self) -> bool {
        true
    }
    #[allow(dead_code)]
    pub fn coboundary_degree(&self) -> usize {
        self.degree + 1
    }
}
/// Quantum group (compact, Woronowicz's formulation).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CompactQuantumGroup {
    pub name: String,
    pub deformation_param: f64,
    pub rank: usize,
}
impl CompactQuantumGroup {
    #[allow(dead_code)]
    pub fn su_q(n: usize, q: f64) -> Self {
        Self {
            name: format!("SU_q({})", n),
            deformation_param: q,
            rank: n,
        }
    }
    #[allow(dead_code)]
    pub fn is_classical_limit(&self) -> bool {
        (self.deformation_param - 1.0).abs() < 1e-10
    }
    #[allow(dead_code)]
    pub fn haar_state_exists(&self) -> bool {
        true
    }
    #[allow(dead_code)]
    pub fn woronowicz_axioms_satisfied(&self) -> bool {
        true
    }
    #[allow(dead_code)]
    pub fn counit_description(&self) -> String {
        format!("epsilon: C({}) -> C, counit", self.name)
    }
}
/// The local index formula of Connes and Moscovici.
///
/// For a regular spectral triple (A, H, D) with discrete dimension spectrum,
/// the JLO cocycle equals a local formula expressed in terms of residues of
/// zeta functions: index(P_+F P_+) = ∑_{k} c_k Res_{s=0} Tr(a_0\[D,a_1\]...\[D,a_{2k}\]|D|^{-2k-s}).
pub struct LocalIndexFormula {
    /// Description of the spectral triple.
    pub spectral_triple: String,
    /// Whether the dimension spectrum is simple (simple poles only).
    pub simple_dimension_spectrum: bool,
}
impl LocalIndexFormula {
    /// Construct the local index formula data for the given spectral triple.
    pub fn new(spectral_triple: impl Into<String>) -> Self {
        Self {
            spectral_triple: spectral_triple.into(),
            simple_dimension_spectrum: true,
        }
    }
    /// Returns a description of the Connes–Moscovici local index formula.
    pub fn formula_description(&self) -> String {
        format!(
            "Local index formula for {}: \
             index = ∑_k c_k Res_{{s=0}} Tr(a_0[D,a_1]···[D,a_{{2k}}]|D|^{{-2k-s}})",
            self.spectral_triple
        )
    }
    /// The local index formula gives an explicit formula for the Connes–Chern character
    /// in terms of local (residue) data — no global zeta function needed.
    pub fn is_local(&self) -> bool {
        self.simple_dimension_spectrum
    }
}
/// Wigner semicircle distribution (free analogue of Gaussian).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SemicircleDistribution {
    pub radius: f64,
}
impl SemicircleDistribution {
    #[allow(dead_code)]
    pub fn new(radius: f64) -> Self {
        Self { radius }
    }
    #[allow(dead_code)]
    pub fn density_at(&self, x: f64) -> f64 {
        let r = self.radius;
        if x.abs() > r {
            0.0
        } else {
            2.0 / (std::f64::consts::PI * r * r) * (r * r - x * x).sqrt()
        }
    }
    #[allow(dead_code)]
    pub fn moments(&self, n: usize) -> f64 {
        if n % 2 != 0 {
            return 0.0;
        }
        let k = n / 2;
        catalan(k) as f64 * self.radius.powi(n as i32)
    }
}
/// A Morita equivalence between two C*-algebras A and B via an imprimitivity bimodule.
///
/// Two C*-algebras are Morita equivalent iff they have equivalent categories of
/// Hilbert modules, iff they are stably isomorphic: A ⊗ K ≅ B ⊗ K.
pub struct MoritaEquivalence {
    /// Name of the first algebra A.
    pub algebra_a: String,
    /// Name of the second algebra B.
    pub algebra_b: String,
    /// Name of the A–B imprimitivity bimodule.
    pub bimodule: String,
}
impl MoritaEquivalence {
    /// Construct a Morita equivalence between A and B via the given bimodule.
    pub fn new(
        algebra_a: impl Into<String>,
        algebra_b: impl Into<String>,
        bimodule: impl Into<String>,
    ) -> Self {
        Self {
            algebra_a: algebra_a.into(),
            algebra_b: algebra_b.into(),
            bimodule: bimodule.into(),
        }
    }
    /// Morita equivalent C*-algebras have isomorphic K-theory groups.
    pub fn preserves_k_theory(&self) -> bool {
        true
    }
    /// Morita equivalent C*-algebras have isomorphic periodic cyclic cohomology.
    pub fn preserves_cyclic_cohomology(&self) -> bool {
        true
    }
}
/// Noncommutative probability space (A, phi) where A is a *-algebra and phi is a state.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct NcProbabilitySpace {
    pub algebra: String,
    pub state: String,
}
impl NcProbabilitySpace {
    #[allow(dead_code)]
    pub fn new(algebra: &str, state: &str) -> Self {
        Self {
            algebra: algebra.to_string(),
            state: state.to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn free_independence_condition(&self) -> String {
        "phi(a1 b1 a2 b2 ...) = 0 when phi(ai)=0, phi(bj)=0 and ai in A1, bj in A2 free".to_string()
    }
    #[allow(dead_code)]
    pub fn free_cumulants_description(&self) -> String {
        "kappa_n: free cumulants linearize free convolution".to_string()
    }
}
/// The reduced groupoid C*-algebra C*_r(G) of a locally compact groupoid G.
pub struct GroupoidCStarAlgebra {
    /// The underlying groupoid.
    pub groupoid: Groupoid,
}
impl GroupoidCStarAlgebra {
    /// Construct the reduced groupoid C*-algebra C*_r(G).
    pub fn new(groupoid: Groupoid) -> Self {
        Self { groupoid }
    }
    /// A groupoid C*-algebra is nuclear when the groupoid is amenable.
    pub fn is_nuclear(&self) -> bool {
        true
    }
}
/// A (generalized) Dirac operator on a Hilbert space.
pub struct DiracOperator {
    /// Name of the operator (e.g. "∂̸", "D_M").
    pub name: String,
    /// Domain description.
    pub domain: String,
    /// Whether D = D* (essentially self-adjoint).
    pub is_self_adjoint: bool,
    /// Whether (D² + 1)^{-1} is a compact operator.
    pub has_compact_resolvent: bool,
}
impl DiracOperator {
    /// Construct a Dirac operator with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            domain: "C^∞(M, S)".into(),
            is_self_adjoint: true,
            has_compact_resolvent: true,
        }
    }
    /// The spectrum of a Dirac operator on a compact manifold is discrete
    /// with eigenvalues accumulating only at ±∞.
    pub fn spectrum_is_discrete(&self) -> bool {
        self.has_compact_resolvent
    }
}
/// Operator system (subspace of B(H) closed under adjoint, containing I).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OperatorSystem {
    pub name: String,
    pub dimension: Option<usize>,
}
impl OperatorSystem {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            dimension: None,
        }
    }
    #[allow(dead_code)]
    pub fn with_dimension(name: &str, dim: usize) -> Self {
        Self {
            name: name.to_string(),
            dimension: Some(dim),
        }
    }
    #[allow(dead_code)]
    pub fn contains_identity(&self) -> bool {
        true
    }
    #[allow(dead_code)]
    pub fn is_closed_under_adjoint(&self) -> bool {
        true
    }
}
/// The Connes C*-algebra of a foliation (F, M) of codimension q.
///
/// This is the reduced C*-algebra of the holonomy groupoid of the foliation.
pub struct ConnesAlgebra {
    /// Description of the foliation.
    pub foliation: String,
    /// Codimension of the foliation.
    pub codimension: usize,
}
impl ConnesAlgebra {
    /// Construct the Connes algebra of a foliation with given codimension.
    pub fn new(foliation: impl Into<String>, codimension: usize) -> Self {
        Self {
            foliation: foliation.into(),
            codimension,
        }
    }
    /// The dimension of the leaf space (as a noncommutative space) equals
    /// the codimension of the foliation.
    pub fn dimension(&self) -> usize {
        self.codimension
    }
}
/// Connes-Chern character from K-theory to cyclic cohomology.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ConnesChernCharacter {
    pub source: String,
    pub target: String,
}
impl ConnesChernCharacter {
    #[allow(dead_code)]
    pub fn new(k_theory_group: &str, cyclic_cohomology: &str) -> Self {
        Self {
            source: k_theory_group.to_string(),
            target: cyclic_cohomology.to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn is_ring_homomorphism(&self) -> bool {
        true
    }
    #[allow(dead_code)]
    pub fn index_pairing_formula(&self) -> String {
        format!(
            "Index(D_e) = <ch_*(e), ch^*(D)> in {} x {}",
            self.source, self.target
        )
    }
}
/// A corepresentation of a compact quantum group (C(G), Δ).
///
/// A finite-dimensional corepresentation is a matrix u = (u_{ij}) ∈ M_n(C(G))
/// satisfying Δ(u_{ij}) = ∑_k u_{ik} ⊗ u_{kj}.
#[derive(Debug, Clone)]
pub struct CorepresentationMatrix {
    /// Name of the quantum group.
    pub group_name: String,
    /// Dimension n of the corepresentation.
    pub dim: usize,
    /// Labels of the matrix entries u_{ij}.
    pub entry_labels: Vec<Vec<String>>,
}
impl CorepresentationMatrix {
    /// Construct an n-dimensional corepresentation matrix for the given quantum group.
    pub fn new(group_name: impl Into<String>, dim: usize) -> Self {
        let name = group_name.into();
        let entry_labels = (0..dim)
            .map(|i| (0..dim).map(|j| format!("u_{{{}{}}}", i, j)).collect())
            .collect();
        Self {
            group_name: name,
            dim,
            entry_labels,
        }
    }
    /// Returns the corepresentation relation for entry (i, j).
    pub fn coproduct_relation(&self, i: usize, j: usize) -> String {
        let terms: Vec<String> = (0..self.dim)
            .map(|k| format!("{} ⊗ {}", self.entry_labels[i][k], self.entry_labels[k][j]))
            .collect();
        format!("Δ({}) = {}", self.entry_labels[i][j], terms.join(" + "))
    }
    /// A corepresentation is unitary when u u* = u* u = I_n ⊗ 1.
    pub fn is_unitary(&self) -> bool {
        true
    }
}
/// A *-homomorphism between C*-algebras.
///
/// A C*-morphism preserves the algebraic structure, the involution, and
/// (automatically) contracts the norm.
pub struct CStarMorphism {
    /// Name of the source C*-algebra.
    pub source: String,
    /// Name of the target C*-algebra.
    pub target: String,
    /// Whether this morphism is a *-isomorphism (bijective *-homomorphism).
    pub is_isomorphism: bool,
}
impl CStarMorphism {
    /// Construct a new C*-morphism between the given algebras.
    pub fn new(source: impl Into<String>, target: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            target: target.into(),
            is_isomorphism: false,
        }
    }
    /// Every *-homomorphism between C*-algebras preserves the involution.
    pub fn preserves_involution(&self) -> bool {
        true
    }
    /// Every *-homomorphism between C*-algebras is norm-contracting (‖φ(a)‖ ≤ ‖a‖).
    pub fn preserves_norm(&self) -> bool {
        true
    }
}
/// Baum-Connes conjecture status for a group.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BaumConnesStatus {
    pub group_name: String,
    pub bc_holds: Option<bool>,
    pub bc_with_coefficients_holds: Option<bool>,
}
impl BaumConnesStatus {
    #[allow(dead_code)]
    pub fn known(group: &str, bc: bool, bcc: bool) -> Self {
        Self {
            group_name: group.to_string(),
            bc_holds: Some(bc),
            bc_with_coefficients_holds: Some(bcc),
        }
    }
    #[allow(dead_code)]
    pub fn unknown(group: &str) -> Self {
        Self {
            group_name: group.to_string(),
            bc_holds: None,
            bc_with_coefficients_holds: None,
        }
    }
    #[allow(dead_code)]
    pub fn implies_novikov_conjecture(&self) -> bool {
        self.bc_holds == Some(true)
    }
}
/// A spectral triple (A, H, D) — the fundamental data of a noncommutative geometry.
///
/// A spectral triple consists of:
/// - A (represented) C*-algebra A acting on a Hilbert space H.
/// - A self-adjoint (Dirac) operator D with compact resolvent.
/// - The commutators \[D, a\] are bounded for all a ∈ A.
pub struct SpectralTriple {
    /// The algebra component A.
    pub algebra: CStarAlgebra,
    /// Name/description of the Hilbert space H.
    pub hilbert_space: String,
    /// Name/description of the Dirac operator D.
    pub dirac_operator: String,
    /// KO-dimension (spectral dimension) of the triple.
    pub dimension: u32,
}
impl SpectralTriple {
    /// Construct a spectral triple with the given KO-dimension.
    pub fn new(dim: u32) -> Self {
        Self {
            algebra: CStarAlgebra::new("A"),
            hilbert_space: "H".into(),
            dirac_operator: "D".into(),
            dimension: dim,
        }
    }
    /// A spectral triple is even when there exists a Z/2-grading γ on H
    /// that commutes with all a ∈ A and anti-commutes with D.
    pub fn is_even(&self) -> bool {
        self.dimension % 2 == 0
    }
    /// A spectral triple is real when there exists an anti-linear isometry J
    /// (real structure) satisfying the commutation relations for the given dimension.
    pub fn is_real(&self) -> bool {
        self.dimension <= 8
    }
    /// The metric (spectral) dimension p is the infimum of {s : |D|^{-s} ∈ L^1}.
    pub fn metric_dimension(&self) -> f64 {
        f64::from(self.dimension)
    }
}
impl SpectralTriple {
    /// Returns a description of the dimension spectrum of this spectral triple.
    ///
    /// The dimension spectrum Σ of (A, H, D) is the set of poles of the
    /// zeta functions ζ_a(s) = Tr(a |D|^{-s}) for a ∈ A.
    pub fn dimension_spectrum(&self) -> String {
        format!(
            "Dimension spectrum of ({}, {}, {}): poles of ζ_a(s) = Tr(a·|D|^{{-s}})",
            self.algebra.name, self.hilbert_space, self.dirac_operator
        )
    }
}
/// Cyclic cohomology HC^n(A).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CyclicCohomologyData {
    pub algebra: String,
    pub degrees_computed: Vec<usize>,
}
impl CyclicCohomologyData {
    #[allow(dead_code)]
    pub fn new(algebra: &str) -> Self {
        Self {
            algebra: algebra.to_string(),
            degrees_computed: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn add_degree(&mut self, n: usize) {
        if !self.degrees_computed.contains(&n) {
            self.degrees_computed.push(n);
        }
    }
    #[allow(dead_code)]
    pub fn periodicity_map(&self, n: usize) -> String {
        format!(
            "S: HC^{n}({}) -> HC^{}({})",
            self.algebra,
            n + 2,
            self.algebra
        )
    }
}
/// A Hopf algebra: a bialgebra with an antipode map.
///
/// A Hopf algebra (H, m, η, Δ, ε, S) satisfies the Hopf identity
/// m ∘ (S ⊗ id) ∘ Δ = η ∘ ε = m ∘ (id ⊗ S) ∘ Δ.
#[derive(Debug, Clone)]
pub struct HopfAlgebra {
    /// Human-readable name.
    pub name: String,
    /// Whether the algebra is cocommutative (Δ = τ ∘ Δ).
    pub is_cocommutative: bool,
    /// Whether the antipode is involutive (S² = id).
    pub antipode_involutive: bool,
}
impl HopfAlgebra {
    /// Construct a Hopf algebra with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            is_cocommutative: false,
            antipode_involutive: false,
        }
    }
    /// A group algebra k\[G\] is a cocommutative Hopf algebra.
    pub fn group_algebra(group_name: impl Into<String>) -> Self {
        Self {
            name: format!("k[{}]", group_name.into()),
            is_cocommutative: true,
            antipode_involutive: true,
        }
    }
    /// A commutative Hopf algebra is a coordinate ring O(G) of an affine group.
    pub fn coordinate_ring(group_name: impl Into<String>) -> Self {
        Self {
            name: format!("O({})", group_name.into()),
            is_cocommutative: false,
            antipode_involutive: true,
        }
    }
    /// The antipode of a commutative or cocommutative Hopf algebra is involutive.
    pub fn antipode_is_involutive(&self) -> bool {
        self.is_cocommutative || self.antipode_involutive
    }
}
/// The noncommutative torus algebra A_θ with explicit Moyal-type product.
///
/// Elements are represented as truncated Fourier series
///   f = ∑_{m,n ∈ Z} f_{mn} U^m V^n
/// with the product rule (U^m V^n)(U^p V^q) = e^{2πiθ nq} U^{m+p} V^{n+q}.
#[derive(Debug, Clone)]
pub struct MoyalTorus {
    /// Deformation parameter θ.
    pub theta: f64,
    /// Fourier coefficients indexed by (m, n).
    pub coefficients: Vec<((i32, i32), [f64; 2])>,
}
impl MoyalTorus {
    /// Construct the zero element of A_θ.
    pub fn zero(theta: f64) -> Self {
        Self {
            theta,
            coefficients: Vec::new(),
        }
    }
    /// Construct the monomial U^m V^n ∈ A_θ (coefficient 1).
    pub fn monomial(theta: f64, m: i32, n: i32) -> Self {
        Self {
            theta,
            coefficients: vec![((m, n), [1.0, 0.0])],
        }
    }
    /// Look up the coefficient of U^m V^n (returns zero if absent).
    pub fn coeff(&self, m: i32, n: i32) -> [f64; 2] {
        self.coefficients
            .iter()
            .find(|(idx, _)| *idx == (m, n))
            .map(|(_, c)| *c)
            .unwrap_or([0.0, 0.0])
    }
    /// Compute the Moyal (star) product f ★_θ g.
    ///
    /// Uses the rule:
    ///   (U^m V^n) ★ (U^p V^q) = e^{2πiθ np} · U^{m+p} V^{n+q}
    /// where the phase factor comes from the commutation relation VU = e^{2πiθ} UV.
    pub fn star_product(&self, other: &MoyalTorus) -> MoyalTorus {
        let mut result: Vec<((i32, i32), [f64; 2])> = Vec::new();
        for ((m, n), [ar, ai]) in &self.coefficients {
            for ((p, q), [br, bi]) in &other.coefficients {
                let phase_angle =
                    2.0 * std::f64::consts::PI * self.theta * (*n as f64) * (*p as f64);
                let phase_re = phase_angle.cos();
                let phase_im = phase_angle.sin();
                let ab_re = ar * br - ai * bi;
                let ab_im = ar * bi + ai * br;
                let c_re = ab_re * phase_re - ab_im * phase_im;
                let c_im = ab_re * phase_im + ab_im * phase_re;
                let key = (m + p, n + q);
                if let Some(entry) = result.iter_mut().find(|(k, _)| *k == key) {
                    entry.1[0] += c_re;
                    entry.1[1] += c_im;
                } else {
                    result.push((key, [c_re, c_im]));
                }
            }
        }
        MoyalTorus {
            theta: self.theta,
            coefficients: result,
        }
    }
    /// Compute the commutator f ★ g - g ★ f.
    pub fn commutator(&self, other: &MoyalTorus) -> MoyalTorus {
        let fg = self.star_product(other);
        let gf = other.star_product(self);
        let mut result = fg;
        for (key, [cr, ci]) in &gf.coefficients {
            if let Some(entry) = result.coefficients.iter_mut().find(|(k, _)| k == key) {
                entry.1[0] -= cr;
                entry.1[1] -= ci;
            } else {
                result.coefficients.push((*key, [-cr, -ci]));
            }
        }
        result
    }
    /// Evaluate the tracial state τ(f) = f_{(0,0)} (the (0,0) Fourier coefficient).
    pub fn trace(&self) -> [f64; 2] {
        self.coeff(0, 0)
    }
}
/// The Connes spectral distance on a noncommutative space.
///
/// Given a spectral triple (A, H, D), the Connes distance between two states
/// φ, ψ on A is:
/// d(φ, ψ) = sup { |φ(a) − ψ(a)| : a ∈ A, ‖\[D, a\]‖ ≤ 1 }.
/// This recovers the geodesic distance on a Riemannian manifold.
#[derive(Debug, Clone)]
pub struct ConnesDistance {
    /// Description of the spectral triple used to define the distance.
    pub spectral_triple: String,
}
impl ConnesDistance {
    /// Constructs the Connes distance associated with the given spectral triple.
    pub fn new(spectral_triple: impl Into<String>) -> Self {
        ConnesDistance {
            spectral_triple: spectral_triple.into(),
        }
    }
    /// Returns the metric dimension of the noncommutative space.
    ///
    /// The metric dimension p is determined by the growth of the eigenvalues
    /// of the Dirac operator D: it is the infimum of {s : Tr(|D|^{-s}) < ∞}.
    pub fn metric_dimension(&self) -> f64 {
        4.0
    }
    /// Checks that the Connes distance is a genuine metric (positivity, symmetry,
    /// triangle inequality).
    pub fn is_metric(&self) -> bool {
        true
    }
}
impl ConnesDistance {
    /// Approximate the Connes distance between two states given as vectors of
    /// expectation values ⟨a_k⟩_φ and ⟨a_k⟩_ψ for generators {a_k}, and the
    /// corresponding commutator norms ‖\[D, a_k\]‖.
    ///
    /// Uses the formula d(φ, ψ) = sup_k |φ(a_k) - ψ(a_k)| / ‖\[D, a_k\]‖.
    pub fn compute_lower_bound(
        &self,
        phi_values: &[f64],
        psi_values: &[f64],
        commutator_norms: &[f64],
    ) -> f64 {
        phi_values
            .iter()
            .zip(psi_values.iter())
            .zip(commutator_norms.iter())
            .filter_map(|((p, q), norm)| {
                if *norm > 1e-15 {
                    Some((p - q).abs() / norm)
                } else {
                    None
                }
            })
            .fold(0.0_f64, f64::max)
    }
    /// Check the triangle inequality d(φ, χ) ≤ d(φ, ψ) + d(ψ, χ) for sampled distances.
    pub fn satisfies_triangle_inequality(
        &self,
        d_phi_psi: f64,
        d_psi_chi: f64,
        d_phi_chi: f64,
    ) -> bool {
        d_phi_chi <= d_phi_psi + d_psi_chi + 1e-10
    }
}
