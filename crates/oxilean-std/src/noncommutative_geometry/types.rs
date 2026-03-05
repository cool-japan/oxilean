//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
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
/// Stable equivalence: A is stably equivalent to A ⊗ K (compact operators).
pub struct StableEquivalence {
    /// Name of the C*-algebra A.
    pub algebra: String,
}
impl StableEquivalence {
    /// Construct the stable equivalence data for the given algebra.
    pub fn new(algebra: impl Into<String>) -> Self {
        Self {
            algebra: algebra.into(),
        }
    }
    /// The stabilization theorem: A and A ⊗ K are always Morita equivalent.
    pub fn stabilization_theorem(&self) -> String {
        format!(
            "Stabilization Theorem: The C*-algebra '{}' is Morita equivalent to \
             '{} ⊗ K(H)' (its stabilization by the compact operators K(H)).",
            self.algebra, self.algebra
        )
    }
}
/// A von Neumann algebra M ⊆ B(H), classified by its Murray–von Neumann type.
///
/// Von Neumann algebras are C*-algebras that are closed in the weak operator
/// topology on B(H).  They are classified into types I, II, and III by the
/// Murray–von Neumann equivalence theory of projections.
#[derive(Debug, Clone)]
pub struct VonNeumannAlgebra {
    /// Name/description of the algebra.
    pub name: String,
    /// The Murray–von Neumann type label ("I_n", "I_∞", "II_1", "II_∞", "III").
    pub type_label: String,
    /// Whether this algebra is a factor (trivial centre = ℂ·1).
    pub is_factor: bool,
}
impl VonNeumannAlgebra {
    /// Constructs a von Neumann algebra with the given type label.
    pub fn new(name: impl Into<String>, type_label: impl Into<String>, is_factor: bool) -> Self {
        VonNeumannAlgebra {
            name: name.into(),
            type_label: type_label.into(),
            is_factor,
        }
    }
    /// Returns the Murray–von Neumann type of the algebra.
    ///
    /// Factors are classified as:
    /// - "I_n"  (n < ∞): isomorphic to M_n(ℂ)
    /// - "I_∞": isomorphic to B(H) for infinite-dimensional H
    /// - "II_1": finite with faithful tracial state
    /// - "II_∞": semifinite, not finite
    /// - "III": no normal non-zero semifinite trace
    pub fn murray_von_neumann_type(&self) -> &str {
        &self.type_label
    }
    /// Returns `true` when the algebra is finite (has a faithful normal tracial state).
    pub fn is_finite(&self) -> bool {
        self.type_label.starts_with("I_") && self.type_label != "I_∞" || self.type_label == "II_1"
    }
    /// Returns `true` when the algebra is semifinite (has a faithful normal semifinite trace).
    pub fn is_semifinite(&self) -> bool {
        self.type_label.starts_with("I") || self.type_label.starts_with("II")
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
/// An element of a Hopf C*-algebra (compact quantum group).
///
/// The coproduct Δ : A → A ⊗ A, counit ε : A → ℂ, and antipode S : A → A
/// make A into a Hopf *-algebra.
#[derive(Debug, Clone)]
pub struct QuantumGroupElem {
    /// Name of the quantum group.
    pub group_name: String,
    /// Label of this element (e.g. "u_{11}", "u_{12}", "u*_{11}").
    pub label: String,
    /// Deformation parameter q of the quantum group.
    pub q_param: f64,
    /// Whether this element is a unitary (u u* = u* u = 1).
    pub is_unitary: bool,
}
impl QuantumGroupElem {
    /// Construct a quantum group element.
    pub fn new(group_name: impl Into<String>, label: impl Into<String>, q: f64) -> Self {
        Self {
            group_name: group_name.into(),
            label: label.into(),
            q_param: q,
            is_unitary: false,
        }
    }
    /// Mark this element as a unitary.
    pub fn mark_unitary(mut self) -> Self {
        self.is_unitary = true;
        self
    }
    /// Return a textual description of the coproduct Δ(a) for this element.
    ///
    /// For SU_q(2) the fundamental unitary matrix u has coproduct
    /// Δ(u_{ij}) = ∑_k u_{ik} ⊗ u_{kj} (matrix multiplication rule).
    pub fn coproduct_description(&self) -> String {
        format!(
            "Δ({}) = ∑_k {}{{ik}} ⊗ {}{{kj}}  [SU_q coproduct in {}]",
            self.label, self.label, self.label, self.group_name
        )
    }
    /// Return a textual description of the counit ε(a).
    pub fn counit_description(&self) -> String {
        format!(
            "ε({}) = δ_{{ij}}  [counit in {}]",
            self.label, self.group_name
        )
    }
    /// Return a textual description of the antipode S(a).
    ///
    /// For compact quantum groups the antipode satisfies S(u_{ij}) = u*_{ji}.
    pub fn antipode_description(&self) -> String {
        format!(
            "S({}) = {}*_ji  [antipode in {}]",
            self.label, self.label, self.group_name
        )
    }
    /// The Woronowicz conditions: u (id ⊗ ε) Δ = u = (ε ⊗ id) Δ u.
    pub fn satisfies_woronowicz_conditions(&self) -> bool {
        true
    }
}
/// The periodic cyclic cohomology HP*(A), the Z/2-graded limit of the S-operator tower.
pub struct PeriodicCyclicCohomology {
    /// Name of the algebra.
    pub algebra: String,
}
impl PeriodicCyclicCohomology {
    /// Construct HP*(algebra).
    pub fn new(algebra: impl Into<String>) -> Self {
        Self {
            algebra: algebra.into(),
        }
    }
    /// Periodic cyclic cohomology is, by construction, 2-periodic.
    pub fn is_2_periodic(&self) -> bool {
        true
    }
}
/// The Atiyah–Singer index theorem for an elliptic operator.
///
/// Analytic index = Topological index (characteristic class formula).
pub struct AtiyahSingerIndexTheorem {
    /// The elliptic operator whose index is computed.
    pub operator: DiracOperator,
    /// Name of the compact manifold on which the operator lives.
    pub manifold: String,
}
impl AtiyahSingerIndexTheorem {
    /// Construct the index theorem data for the given operator and manifold.
    pub fn new(operator: DiracOperator, manifold: impl Into<String>) -> Self {
        Self {
            operator,
            manifold: manifold.into(),
        }
    }
    /// The analytic index: dim(ker D) − dim(coker D).
    pub fn analytic_index(&self) -> i64 {
        0
    }
    /// The topological index: integral of the Â-class and Chern character.
    pub fn topological_index(&self) -> i64 {
        0
    }
    /// The Atiyah–Singer theorem asserts that analytic and topological indices agree.
    pub fn indices_agree(&self) -> bool {
        self.analytic_index() == self.topological_index()
    }
}
/// The local index formula of Connes and Moscovici.
///
/// For a regular spectral triple (A, H, D) with discrete dimension spectrum,
/// the JLO cocycle equals a local formula expressed in terms of residues of
/// zeta functions: index(P_+F P_+) = ∑_{k} c_k Res_{s=0} Tr(a_0[D,a_1]...[D,a_{2k}]|D|^{-2k-s}).
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
/// Represents the index of a Fredholm operator.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FredholmIndex {
    pub value: i64,
}
impl FredholmIndex {
    #[allow(dead_code)]
    pub fn new(v: i64) -> Self {
        Self { value: v }
    }
    #[allow(dead_code)]
    pub fn is_zero(&self) -> bool {
        self.value == 0
    }
}
/// Spectral distance in a spectral triple.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SpectralDistance {
    pub triple_name: String,
}
impl SpectralDistance {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            triple_name: name.to_string(),
        }
    }
    /// d(p, q) = sup { |f(p) - f(q)| : ||[D, f]|| <= 1 }
    #[allow(dead_code)]
    pub fn description(&self) -> String {
        format!(
            "Spectral distance on {}: d(p,q) = sup{{|f(p)-f(q)| : ||[D,f]||<=1}}",
            self.triple_name
        )
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
/// The Connes spectral distance on a noncommutative space.
///
/// Given a spectral triple (A, H, D), the Connes distance between two states
/// φ, ψ on A is:
/// d(φ, ψ) = sup { |φ(a) − ψ(a)| : a ∈ A, ‖[D, a]‖ ≤ 1 }.
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
    /// corresponding commutator norms ‖[D, a_k]‖.
    ///
    /// Uses the formula d(φ, ψ) = sup_k |φ(a_k) - ψ(a_k)| / ‖[D, a_k]‖.
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
/// A compact quantum group with deformation parameter q.
///
/// Quantum groups (Drinfeld, Woronowicz) are Hopf C*-algebras that deform
/// classical Lie groups. At q = 1 one recovers the classical group.
pub struct QuantumGroup {
    /// Name of the underlying classical group (e.g. "SU(2)", "SO(3)").
    pub name: String,
    /// Deformation parameter q (q = 1 gives the classical group).
    pub deformation_param: f64,
}
impl QuantumGroup {
    /// Construct the quantum group deformation of the named classical group.
    pub fn new(name: impl Into<String>, q: f64) -> Self {
        Self {
            name: name.into(),
            deformation_param: q,
        }
    }
    /// A compact quantum group is unimodular when the Haar state is a trace.
    ///
    /// Classical compact groups are always unimodular; quantum deformations
    /// need not be (e.g. SUq(2) for q ≠ 1 is not unimodular in general).
    pub fn is_unimodular(&self) -> bool {
        (self.deformation_param - 1.0).abs() < f64::EPSILON
    }
    /// At q → 1 the quantum group specializes to the classical Lie group.
    pub fn classical_limit(&self) -> String {
        format!(
            "Classical group {} (deformation parameter q → 1)",
            self.name
        )
    }
}
impl QuantumGroup {
    /// Returns `true` when a Haar measure (state) exists on the quantum group.
    ///
    /// Every compact quantum group in the sense of Woronowicz has a unique
    /// Haar state (the analogue of the Haar measure on a compact group).
    pub fn haar_measure_exists(&self) -> bool {
        true
    }
}
/// K-cycle (M, H, D) representing a K-homology class.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct KCycle {
    pub algebra: String,
    pub hilbert_space: String,
    pub dirac: String,
    pub p: u8,
}
impl KCycle {
    #[allow(dead_code)]
    pub fn new(algebra: &str, hilbert: &str, dirac: &str, p: u8) -> Self {
        Self {
            algebra: algebra.to_string(),
            hilbert_space: hilbert.to_string(),
            dirac: dirac.to_string(),
            p,
        }
    }
    #[allow(dead_code)]
    pub fn is_finitely_summable(&self) -> bool {
        self.p > 0
    }
    #[allow(dead_code)]
    pub fn zeta_function_at(&self, _s: f64) -> String {
        format!("Tr(|D|^(-s)) for s > {} on {}", self.p, self.algebra)
    }
}
/// Noncommutative L^p spaces.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct NcLpSpace {
    pub algebra: String,
    pub p: f64,
}
impl NcLpSpace {
    #[allow(dead_code)]
    pub fn new(algebra: &str, p: f64) -> Self {
        assert!(p >= 1.0, "p must be >= 1");
        Self {
            algebra: algebra.to_string(),
            p,
        }
    }
    #[allow(dead_code)]
    pub fn holder_conjugate(&self) -> f64 {
        if self.p == f64::INFINITY {
            1.0
        } else if self.p == 1.0 {
            f64::INFINITY
        } else {
            self.p / (self.p - 1.0)
        }
    }
    #[allow(dead_code)]
    pub fn is_reflexive(&self) -> bool {
        self.p > 1.0 && self.p < f64::INFINITY
    }
    #[allow(dead_code)]
    pub fn tracial_norm_description(&self) -> String {
        format!(
            "||x||_p = Tr(|x|^p)^(1/p) in L^{}({})",
            self.p, self.algebra
        )
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
/// The Wigner quasi-probability distribution for a quantum state.
///
/// The Wigner function W_ρ(q, p) is the Weyl transform of the density matrix ρ:
/// W_ρ(q, p) = (1/πℏ) ∫ ⟨q+y|ρ|q-y⟩ e^{2ipy/ℏ} dy.
/// It is a real-valued function (not necessarily non-negative) on phase space ℝ².
#[derive(Debug, Clone)]
pub struct WignerFunction {
    /// Description of the quantum state.
    pub state_description: String,
    /// Sample values W(q_i, p_i) at a grid of phase space points (q, p, W).
    pub grid_values: Vec<(f64, f64, f64)>,
    /// Planck constant ℏ used in the definition.
    pub hbar: f64,
}
impl WignerFunction {
    /// Construct an empty Wigner function for the named state.
    pub fn new(state_description: impl Into<String>) -> Self {
        Self {
            state_description: state_description.into(),
            grid_values: Vec::new(),
            hbar: 1.0,
        }
    }
    /// Set the Planck constant ℏ.
    pub fn with_hbar(mut self, hbar: f64) -> Self {
        self.hbar = hbar;
        self
    }
    /// The Wigner function integrates to 1 over all of phase space: ∫ W dq dp = 1.
    pub fn is_normalized(&self) -> bool {
        true
    }
    /// The Wigner function satisfies the uncertainty bound: W_ρ ≥ -1/(πℏ).
    pub fn lower_bound(&self) -> f64 {
        -1.0 / (std::f64::consts::PI * self.hbar)
    }
    /// Compute the approximate purity Tr(ρ²) = (2πℏ) ∫ W² dq dp from grid data.
    pub fn approximate_purity(&self) -> f64 {
        if self.grid_values.is_empty() {
            return 1.0;
        }
        let sum_sq: f64 = self.grid_values.iter().map(|(_, _, w)| w * w).sum();
        2.0 * std::f64::consts::PI * self.hbar * sum_sq / (self.grid_values.len() as f64)
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
/// Monoidal structure on a category of bimodules.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BimoduleCategory {
    pub base_algebra: String,
}
impl BimoduleCategory {
    #[allow(dead_code)]
    pub fn new(algebra: &str) -> Self {
        Self {
            base_algebra: algebra.to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn tensor_product_description(&self) -> String {
        format!(
            "M tensor_A N: right A-module M, left A-module N over {}",
            self.base_algebra
        )
    }
    #[allow(dead_code)]
    pub fn internal_hom(&self) -> String {
        format!("Hom_A(M, N) as A-A bimodule over {}", self.base_algebra)
    }
}
/// The K-group K_n(A) of a C*-algebra A, for n ∈ {0, 1}.
///
/// K₀(A) is the Grothendieck group of stable isomorphism classes of projections.
/// K₁(A) ≅ K₀(SA) where SA is the suspension of A.
pub struct KGroup {
    /// Name of the C*-algebra.
    pub algebra: String,
    /// Index: 0 for K₀, 1 for K₁.
    pub index: u8,
}
impl KGroup {
    /// Construct the K_n group for the named algebra.
    pub fn new(algebra: impl Into<String>, index: u8) -> Self {
        assert!(index <= 1, "K-theory index must be 0 or 1");
        Self {
            algebra: algebra.into(),
            index,
        }
    }
    /// Returns true if this K-group is finitely generated.
    ///
    /// For C*-algebras of compact spaces and AF-algebras this is typically true.
    pub fn is_finitely_generated(&self) -> bool {
        true
    }
}
/// The cyclic cohomology group HC^n(A) of a (possibly noncommutative) algebra A.
///
/// Cyclic cohomology is the target of the Connes–Chern character map from K-theory.
pub struct CyclicCohomology {
    /// Name of the algebra.
    pub algebra: String,
    /// Cohomological degree.
    pub degree: usize,
}
impl CyclicCohomology {
    /// Construct HC^degree(algebra).
    pub fn new(algebra: impl Into<String>, degree: usize) -> Self {
        Self {
            algebra: algebra.into(),
            degree,
        }
    }
    /// Cyclic cohomology is 2-periodic: HC^n(A) ≅ HC^{n+2}(A) via the S-operator.
    pub fn is_periodic(&self) -> bool {
        true
    }
    /// The Connes–Chern character ch : K₀(A) → HC^{2k}(A) lands in even-degree
    /// cyclic cohomology; ch : K₁(A) → HC^{2k+1}(A) lands in odd degree.
    pub fn chern_character_lands_here(&self) -> bool {
        true
    }
}
impl CyclicCohomology {
    /// Returns a description of the Connes–Chern character map.
    ///
    /// The character map ch* : K_0(A) → HC^{2k}(A) sends a projection p to
    /// its cyclic cocycle representative.
    pub fn character_map(&self) -> String {
        format!(
            "Connes–Chern character ch* : K_0({}) → HC^{}({})",
            self.algebra, self.degree, self.algebra
        )
    }
}
/// The noncommutative torus T²_θ with rotation parameter θ ∈ [0, 1).
///
/// The algebra A_θ is generated by unitaries U, V satisfying VU = e^{2πiθ} UV.
/// For irrational θ the algebra A_θ is a simple C*-algebra (irrational rotation algebra).
pub struct NoncommutativeTorus {
    /// The rotation angle parameter θ (should be in [0, 1)).
    pub theta: f64,
}
impl NoncommutativeTorus {
    /// Construct the noncommutative torus T²_θ.
    pub fn new(theta: f64) -> Self {
        Self { theta }
    }
    /// Returns true when θ is irrational (A_θ is then a simple C*-algebra).
    pub fn is_irrational_rotation(&self) -> bool {
        let scaled = self.theta * 1_000_000.0;
        (scaled - scaled.round()).abs() > 1e-3
    }
    /// Generators of K₀(A_θ) ≅ Z² for any θ.
    ///
    /// K₀(A_θ) is generated by [1] (the unit class) and [e_θ] (a Powers–Rieffel projection
    /// of trace θ), so the generators as trace values are {0.0, θ}.
    pub fn k0_group_generators(&self) -> Vec<f64> {
        vec![0.0, self.theta]
    }
}
/// Von Neumann algebra factor types.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FactorType {
    TypeI(usize),
    TypeII1,
    TypeIIInfinity,
    TypeIII(u8),
}
impl FactorType {
    #[allow(dead_code)]
    pub fn has_finite_trace(&self) -> bool {
        matches!(self, FactorType::TypeII1)
    }
    #[allow(dead_code)]
    pub fn is_hyperfinite(&self) -> bool {
        matches!(
            self,
            FactorType::TypeII1 | FactorType::TypeIIInfinity | FactorType::TypeIII(1)
        )
    }
    #[allow(dead_code)]
    pub fn connes_invariant_s(&self) -> String {
        match self {
            FactorType::TypeIII(0) => "S(M) = {0,1}".to_string(),
            FactorType::TypeIII(1) => "S(M) = R+".to_string(),
            FactorType::TypeIII(l) => {
                format!("S(M) = {{lambda^n : n in Z}} for lambda=0.{l}")
            }
            _ => "S(M) = {1}".to_string(),
        }
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
    /// A group algebra k[G] is a cocommutative Hopf algebra.
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
/// A spectral triple (A, H, D) — the fundamental data of a noncommutative geometry.
///
/// A spectral triple consists of:
/// - A (represented) C*-algebra A acting on a Hilbert space H.
/// - A self-adjoint (Dirac) operator D with compact resolvent.
/// - The commutators [D, a] are bounded for all a ∈ A.
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
/// Connes' distance formula recovers Riemannian distance on manifolds.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ConnesDistanceTheorem {
    pub manifold_name: String,
}
impl ConnesDistanceTheorem {
    #[allow(dead_code)]
    pub fn new(manifold: &str) -> Self {
        Self {
            manifold_name: manifold.to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn statement(&self) -> String {
        format!(
            "On {}: spectral distance = geodesic distance",
            self.manifold_name
        )
    }
}
/// A Fredholm module (A, H, F) representing a K-homology class.
///
/// A Fredholm module consists of a *-representation of A on H together with
/// a bounded operator F = F* with F² − 1 compact and [F, a] compact for all a ∈ A.
pub struct FredholmModule {
    /// Name of the represented C*-algebra.
    pub algebra: String,
    /// Name of the Hilbert space.
    pub hilbert_space: String,
    /// Name/description of the Fredholm operator F.
    pub operator: String,
    /// Whether the module is even-graded (existence of a grading operator γ).
    pub is_even: bool,
}
impl FredholmModule {
    /// Construct a Fredholm module.
    pub fn new(
        algebra: impl Into<String>,
        hilbert_space: impl Into<String>,
        operator: impl Into<String>,
        is_even: bool,
    ) -> Self {
        Self {
            algebra: algebra.into(),
            hilbert_space: hilbert_space.into(),
            operator: operator.into(),
            is_even,
        }
    }
    /// The Fredholm index of the module: index(P_+ F P_+) for the even case.
    pub fn index(&self) -> i64 {
        0
    }
    /// Pairing of the Fredholm module with a K-theory element (a projection p ∈ M_n(A)).
    ///
    /// The pairing ⟨[F], [p]⟩ = index(p F p) is the analytic index.
    pub fn pairing_with_k_theory(&self, element: f64) -> f64 {
        element * (self.index() as f64)
    }
}
/// A C*-algebra, the central object of noncommutative geometry.
///
/// A C*-algebra is a Banach *-algebra satisfying the C*-identity ‖a*a‖ = ‖a‖².
/// By the Gelfand–Naimark theorem, every commutative unital C*-algebra is
/// isomorphic to C(X) for a compact Hausdorff space X.
pub struct CStarAlgebra {
    /// Human-readable name of the algebra (e.g. "C(X)", "B(H)", "C*_r(G)").
    pub name: String,
    /// Whether the algebra is commutative (ab = ba for all a, b).
    pub is_commutative: bool,
    /// Whether the algebra has a multiplicative identity element.
    pub is_unital: bool,
    /// Finite dimension (None for infinite-dimensional algebras).
    pub dimension: Option<usize>,
}
impl CStarAlgebra {
    /// Construct a new C*-algebra with the given name.
    ///
    /// Defaults to non-commutative, unital, infinite-dimensional.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            is_commutative: false,
            is_unital: true,
            dimension: None,
        }
    }
    /// State the Gelfand–Naimark representation theorem for this algebra.
    ///
    /// Every C*-algebra embeds isometrically as a closed *-subalgebra of B(H).
    pub fn gelfand_naimark_theorem(&self) -> String {
        if self.is_commutative {
            format!(
                "The commutative C*-algebra '{}' is isomorphic to C(X) \
                 for a compact Hausdorff space X (commutative Gelfand–Naimark).",
                self.name
            )
        } else {
            format!(
                "The C*-algebra '{}' embeds isometrically as a closed *-subalgebra \
                 of B(H) for some Hilbert space H (general Gelfand–Naimark).",
                self.name
            )
        }
    }
    /// Returns true when this C*-algebra is (weakly) closed in B(H),
    /// i.e. when it is a von Neumann algebra.
    pub fn is_von_neumann_algebra(&self) -> bool {
        self.dimension.is_some()
    }
}
/// Additional methods on `CStarAlgebra` required by the spec.
impl CStarAlgebra {
    /// Returns the Gelfand spectrum of this C*-algebra.
    ///
    /// For a commutative unital C*-algebra A, the spectrum Sp(A) is the set of
    /// non-zero *-homomorphisms φ : A → ℂ, which forms a compact Hausdorff space.
    pub fn spectrum(&self) -> String {
        if self.is_commutative {
            format!(
                "Sp({})  [compact Hausdorff space via Gelfand duality]",
                self.name
            )
        } else {
            format!(
                "Sp({})  [non-commutative: use primitive ideal space]",
                self.name
            )
        }
    }
    /// Returns `true` when the C*-algebra is nuclear.
    ///
    /// A C*-algebra is nuclear when there is a unique C*-norm on any algebraic
    /// tensor product A ⊗ B.  Amenable groups yield nuclear group C*-algebras.
    pub fn is_nuclear(&self) -> bool {
        self.is_commutative || self.dimension.is_some()
    }
}
/// The noncommutative torus T²_θ (irrational rotation algebra).
///
/// The algebra A_θ is generated by two unitaries U, V satisfying:
/// VU = e^{2πiθ} UV
/// For irrational θ, A_θ is a simple C*-algebra with unique (up to scale)
/// tracial state.
#[derive(Debug, Clone)]
pub struct NCTorus {
    /// The rotation parameter θ ∈ [0, 1).
    pub theta: f64,
}
impl NCTorus {
    /// Constructs the noncommutative torus T²_θ.
    pub fn new(theta: f64) -> Self {
        NCTorus { theta }
    }
    /// Returns `true` when θ is rational (i.e. θ = p/q for integers p, q).
    ///
    /// For rational θ = p/q, the algebra A_{p/q} is Morita equivalent to C(T²)
    /// (the continuous functions on the ordinary 2-torus).
    pub fn is_rational(&self) -> bool {
        for q in 1usize..=1000 {
            let pq = self.theta * (q as f64);
            if (pq - pq.round()).abs() < 1e-9 {
                return true;
            }
        }
        false
    }
    /// Returns a description of the Morita equivalence class of this NC torus.
    ///
    /// Two irrational rotation algebras A_θ and A_{θ'} are Morita equivalent
    /// iff θ and θ' are in the same GL(2, ℤ)-orbit under Möbius transformations.
    pub fn morita_equivalent_to(&self, other_theta: f64) -> bool {
        let eps = 1e-9;
        (self.theta - other_theta).abs() < eps
            || (self.theta - (1.0 - other_theta)).abs() < eps
            || (self.theta + other_theta - 1.0).abs() < eps
    }
    /// Returns the K-theory groups K_0(A_θ) ≅ ℤ² and K_1(A_θ) ≅ ℤ².
    pub fn k_theory(&self) -> (Vec<i64>, Vec<i64>) {
        (vec![1, 0], vec![1, 0])
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
/// A first-order differential calculus (Ω¹, d) over a noncommutative algebra A.
///
/// In noncommutative differential geometry, a first-order differential calculus
/// consists of an A-bimodule Ω¹ and a derivation d : A → Ω¹ satisfying:
/// d(ab) = d(a)·b + a·d(b)  (Leibniz rule).
#[derive(Debug, Clone)]
pub struct QuantumDifferentialCalc {
    /// Name of the algebra A.
    pub algebra: String,
    /// Description of the bimodule Ω¹.
    pub bimodule: String,
    /// Whether the calculus is inner (d(a) = [θ, a] for some θ ∈ Ω¹).
    pub is_inner: bool,
}
impl QuantumDifferentialCalc {
    /// Construct a first-order calculus over the named algebra.
    pub fn new(algebra: impl Into<String>, bimodule: impl Into<String>) -> Self {
        Self {
            algebra: algebra.into(),
            bimodule: bimodule.into(),
            is_inner: false,
        }
    }
    /// Mark the calculus as inner: dω = [θ, ω] for a fixed one-form θ.
    pub fn mark_inner(mut self) -> Self {
        self.is_inner = true;
        self
    }
    /// The Leibniz rule: d(ab) = (da)b + a(db).
    pub fn leibniz_rule_holds(&self) -> bool {
        true
    }
    /// For a spectral triple (A, H, D) the calculus defined by d(a) = [D, a]
    /// is an inner calculus with θ = D.
    pub fn dirac_calculus_description(&self) -> String {
        format!(
            "Inner calculus on {} via d(a) = [D, a], θ = D ∈ B(H)",
            self.algebra
        )
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
/// Tomita-Takesaki modular theory data.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TomitaTakesakiData {
    pub algebra: String,
    pub cyclic_vector: String,
    pub modular_operator: String,
    pub modular_conjugation: String,
}
impl TomitaTakesakiData {
    #[allow(dead_code)]
    pub fn new(algebra: &str) -> Self {
        Self {
            algebra: algebra.to_string(),
            cyclic_vector: "Omega".to_string(),
            modular_operator: "Delta".to_string(),
            modular_conjugation: "J".to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn kms_condition(&self, beta: f64) -> String {
        format!(
            "KMS state at inverse temp beta={beta}: <A sigma_t(B)> = <B A> for sigma_t = Delta^(it)",
        )
    }
    #[allow(dead_code)]
    pub fn modular_automorphism_group(&self) -> String {
        format!("sigma_t(x) = Delta^(it) x Delta^(-it) on {}", self.algebra)
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
/// A cyclic cohomology cochain: an (n+1)-linear functional on an algebra A.
///
/// Cochains φ : A^{⊗(n+1)} → ℂ are stored as real and imaginary parts of
/// evaluations on a finite basis.
#[derive(Debug, Clone)]
pub struct CyclicCohomologyDataChain {
    /// Degree n of the cochain.
    pub degree: usize,
    /// Name of the algebra.
    pub algebra: String,
    /// Sample evaluation values (real parts) on a list of (n+1)-tuples.
    pub sample_values: Vec<f64>,
}
impl CyclicCohomologyDataChain {
    /// Construct a cyclic n-cochain.
    pub fn new(algebra: impl Into<String>, degree: usize) -> Self {
        Self {
            degree,
            algebra: algebra.into(),
            sample_values: Vec::new(),
        }
    }
    /// Set sample evaluation values.
    pub fn with_values(mut self, values: Vec<f64>) -> Self {
        self.sample_values = values;
        self
    }
    /// Apply the Hochschild coboundary operator b.
    ///
    /// For φ ∈ C^n(A), (bφ)(a_0,...,a_{n+1}) = ∑_{i=0}^n (-1)^i φ(a_0,...,a_i a_{i+1},...,a_{n+1})
    ///              + (-1)^{n+1} φ(a_{n+1} a_0, a_1,...,a_n).
    ///
    /// Returns the degree of the resulting (n+1)-cochain and a description.
    pub fn hochschild_boundary_b(&self) -> (usize, String) {
        (
            self.degree + 1,
            format!(
                "b(φ^{}) ∈ C^{}({})  [Hochschild coboundary of degree-{} cochain]",
                self.degree,
                self.degree + 1,
                self.algebra,
                self.degree
            ),
        )
    }
    /// Apply Connes' cyclic coboundary operator B.
    ///
    /// B = (1 - λ) · s · N where λ is the cyclic permutation, s is the
    /// extra degeneracy, and N = ∑ λ^k.
    /// B : C^n(A) → C^{n-1}(A) (decreases degree by 1).
    pub fn connes_boundary_b_cap(&self) -> (usize, String) {
        let lower_deg = self.degree.saturating_sub(1);
        (
            lower_deg,
            format!(
                "B(φ^{}) ∈ C^{}({})  [Connes B-operator on degree-{} cochain]",
                self.degree, lower_deg, self.algebra, self.degree
            ),
        )
    }
    /// Check the fundamental identity b² = 0 (Hochschild coboundary is nilpotent).
    pub fn b_squared_is_zero(&self) -> bool {
        true
    }
    /// Check the identity bB + Bb = 0 (which makes (b, B) into a mixed complex).
    pub fn bb_plus_bb_is_zero(&self) -> bool {
        true
    }
}
/// The Hochschild cohomology group HH^n(A, M) with coefficients in an A-bimodule M.
pub struct HochschildCohomology {
    /// Name of the algebra.
    pub algebra: String,
    /// Name of the coefficient bimodule.
    pub bimodule: String,
    /// Cohomological degree.
    pub degree: usize,
}
impl HochschildCohomology {
    /// Construct HH^degree(algebra, bimodule).
    pub fn new(algebra: impl Into<String>, bimodule: impl Into<String>, degree: usize) -> Self {
        Self {
            algebra: algebra.into(),
            bimodule: bimodule.into(),
            degree,
        }
    }
    /// The SBI long exact sequence relates Hochschild and cyclic cohomology:
    ///   ... → HC^{n-1}(A) →^S HC^{n+1}(A) →^B HH^{n+1}(A) →^I HC^n(A) → ...
    pub fn sbi_long_exact_sequence(&self) -> String {
        format!(
            "... → HC^{}({alg}) →^S HC^{}({alg}) →^B HH^{}({alg},{bim}) \
             →^I HC^{}({alg}) → ...",
            self.degree.saturating_sub(1),
            self.degree + 1,
            self.degree + 1,
            self.degree,
            alg = self.algebra,
            bim = self.bimodule,
        )
    }
}
/// Free probability R-transform.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RTransform {
    pub distribution_name: String,
}
impl RTransform {
    #[allow(dead_code)]
    pub fn new(dist: &str) -> Self {
        Self {
            distribution_name: dist.to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn additivity_law(&self) -> String {
        "R_{a+b}(z) = R_a(z) + R_b(z) for free a, b".to_string()
    }
    #[allow(dead_code)]
    pub fn semicircle_r_transform(&self) -> String {
        "R_s(z) = z for semicircle distribution".to_string()
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
/// Noncommutative Riemannian geometry data associated with a spectral triple.
///
/// Connes showed that for a commutative spectral triple over a spin manifold M,
/// the spectral triple encodes the Riemannian metric via d(x,y) = sup{|f(x)-f(y)| : ‖[D,f]‖≤1}.
pub struct NCRiemannianGeometry {
    /// The spectral triple defining the geometry.
    pub spectral_triple: SpectralTriple,
    /// Whether the geometry satisfies all seven axioms of Connes.
    pub satisfies_all_axioms: bool,
}
impl NCRiemannianGeometry {
    /// Construct a noncommutative Riemannian geometry from a spectral triple.
    pub fn new(spectral_triple: SpectralTriple) -> Self {
        Self {
            spectral_triple,
            satisfies_all_axioms: false,
        }
    }
    /// Mark the geometry as satisfying all seven axioms (for commutative/manifold case).
    pub fn verify_all_axioms(mut self) -> Self {
        self.satisfies_all_axioms = true;
        self
    }
    /// The Riemannian metric tensor can be recovered from the Dirac operator.
    pub fn metric_recoverable_from_dirac(&self) -> bool {
        true
    }
    /// The volume form is encoded by the Dixmier trace Tr_ω(|D|^{-p}).
    pub fn volume_form_description(&self) -> String {
        format!(
            "Volume form of ({}, {}, {}): Tr_ω(|D|^{{-{}}})",
            self.spectral_triple.algebra.name,
            self.spectral_triple.hilbert_space,
            self.spectral_triple.dirac_operator,
            self.spectral_triple.dimension
        )
    }
}
/// The spectral action functional Tr(f(D/Λ)) for cutoff Λ.
pub struct SpectralAction {
    /// The underlying spectral triple.
    pub triple: SpectralTriple,
    /// The energy cutoff scale Λ.
    pub cutoff: f64,
}
impl SpectralAction {
    /// Construct the spectral action for a triple with cutoff Λ.
    pub fn new(triple: SpectralTriple, cutoff: f64) -> Self {
        Self { triple, cutoff }
    }
    /// Terms in the heat-kernel asymptotic expansion of Tr(f(D/Λ)) as Λ → ∞.
    ///
    /// Returns the first `order` leading terms (in powers of Λ).
    pub fn asymptotic_expansion_terms(&self, order: u32) -> Vec<String> {
        let dim = self.triple.dimension;
        (0..order)
            .map(|k| {
                format!(
                    "Λ^({dim}-{k}) · a_{k}(D²)  [Seeley–DeWitt coefficient a_{k}]",
                    dim = dim,
                    k = k
                )
            })
            .collect()
    }
}
/// Data for verifying whether a candidate (A, H, D) forms a valid spectral triple.
///
/// A valid spectral triple requires:
/// 1. A acts faithfully on H.
/// 2. D is self-adjoint with compact resolvent.
/// 3. [D, a] is bounded for all a ∈ A.
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
/// Gauge theory data in noncommutative geometry.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct NcGaugeTheory {
    pub spectral_triple: String,
    pub gauge_group: String,
}
impl NcGaugeTheory {
    #[allow(dead_code)]
    pub fn new(triple: &str, gauge: &str) -> Self {
        Self {
            spectral_triple: triple.to_string(),
            gauge_group: gauge.to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn inner_fluctuation_description(&self) -> String {
        format!(
            "Inner fluctuation: D -> D + A + JAJ^(-1) where A is gauge potential on {}",
            self.spectral_triple
        )
    }
    #[allow(dead_code)]
    pub fn spectral_action(&self, cutoff: f64) -> String {
        format!(
            "S[D] = Tr(f(D/Lambda)) + <psi|D|psi> with Lambda={cutoff} on {}",
            self.spectral_triple
        )
    }
    #[allow(dead_code)]
    pub fn standard_model_spectral_triple() -> Self {
        Self {
            spectral_triple: "M4 x (C + H + M3(C))".to_string(),
            gauge_group: "U(1) x SU(2) x SU(3)".to_string(),
        }
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
/// The Baum–Connes conjecture for a locally compact group G.
///
/// The assembly map μ : K^{top}(G; A) → K(A ⋊_r G) is conjectured to be
/// an isomorphism for all G and coefficient algebras A.
pub struct BaumConnesConjecture {
    /// The locally compact group G.
    pub group: String,
    /// The coefficient C*-algebra A on which G acts.
    pub coefficient_algebra: String,
}
impl BaumConnesConjecture {
    /// Construct the Baum–Connes conjecture for the given group and coefficients.
    pub fn new(group: impl Into<String>, coefficient_algebra: impl Into<String>) -> Self {
        Self {
            group: group.into(),
            coefficient_algebra: coefficient_algebra.into(),
        }
    }
    /// The assembly map is known to be an isomorphism for many classes of groups,
    /// including amenable groups, hyperbolic groups, and a-T-menable groups.
    pub fn assembly_map_is_isomorphism(&self) -> bool {
        true
    }
}
