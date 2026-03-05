//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};
use std::collections::HashMap;

/// A registry of C*-algebras and von Neumann algebras.
#[derive(Debug, Clone)]
pub struct OperatorAlgebraRegistry {
    /// C*-algebras indexed by name
    pub c_star_algebras: HashMap<String, CStarAlgebraData>,
    /// von Neumann algebras indexed by name
    pub von_neumann_algebras: HashMap<String, (CStarAlgebraData, FactorType)>,
    /// GNS triples indexed by "algebra::state"
    pub gns_triples: HashMap<String, GNSTripleData>,
}
impl OperatorAlgebraRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        OperatorAlgebraRegistry {
            c_star_algebras: HashMap::new(),
            von_neumann_algebras: HashMap::new(),
            gns_triples: HashMap::new(),
        }
    }
    /// Create a registry with standard examples.
    pub fn with_standard_examples() -> Self {
        let mut reg = Self::new();
        reg.c_star_algebras
            .insert("M_2(C)".to_string(), CStarAlgebraData::matrix_algebra(2));
        reg.c_star_algebras
            .insert("M_3(C)".to_string(), CStarAlgebraData::matrix_algebra(3));
        reg.c_star_algebras
            .insert("K(H)".to_string(), CStarAlgebraData::compact_operators());
        reg.c_star_algebras.insert(
            "C(S^1)".to_string(),
            CStarAlgebraData::continuous_functions("S^1", 2),
        );
        reg.c_star_algebras
            .insert("O_2".to_string(), CStarAlgebraData::cuntz_algebra(2));
        reg.c_star_algebras
            .insert("O_3".to_string(), CStarAlgebraData::cuntz_algebra(3));
        reg.von_neumann_algebras.insert(
            "R".to_string(),
            (
                CStarAlgebraData::new("R (hyperfinite II_1)"),
                FactorType::TypeII1,
            ),
        );
        reg.von_neumann_algebras.insert(
            "R_inf".to_string(),
            (
                CStarAlgebraData::new("R_inf (Powers III_1)"),
                FactorType::TypeIII1,
            ),
        );
        let m2 = CStarAlgebraData::matrix_algebra(2);
        let tr = StateData::tracial("tr");
        let gns = GNSTripleData::build(&m2, &tr);
        reg.gns_triples.insert("M_2(C)::tr".to_string(), gns);
        reg
    }
    /// Register a C*-algebra.
    pub fn register_c_star(&mut self, algebra: CStarAlgebraData) {
        self.c_star_algebras.insert(algebra.name.clone(), algebra);
    }
    /// Register a von Neumann algebra with its factor type.
    pub fn register_von_neumann(&mut self, algebra: CStarAlgebraData, ft: FactorType) {
        self.von_neumann_algebras
            .insert(algebra.name.clone(), (algebra, ft));
    }
    /// Perform GNS construction and register the result.
    pub fn gns_construct(&mut self, algebra_name: &str, state: StateData) -> Option<GNSTripleData> {
        let algebra = self.c_star_algebras.get(algebra_name)?;
        let triple = GNSTripleData::build(algebra, &state);
        let key = format!("{}::{}", algebra_name, state.name);
        self.gns_triples.insert(key, triple.clone());
        Some(triple)
    }
    /// Count total registered algebras.
    pub fn total_count(&self) -> usize {
        self.c_star_algebras.len() + self.von_neumann_algebras.len()
    }
}
/// Classification of von Neumann algebra factors.
#[derive(Debug, Clone, PartialEq)]
pub enum FactorType {
    /// Type I_n: B(C^n) for finite n, or B(H) for separable H.
    TypeI(Option<usize>),
    /// Type II_1: finite von Neumann algebras with faithful tracial state.
    TypeII1,
    /// Type II_inf: semifinite von Neumann algebra, not finite.
    TypeIIInfty,
    /// Type III_lambda for 0 < lambda < 1 (Connes' classification).
    TypeIII(f64),
    /// Type III_0: most exotic type.
    TypeIII0,
    /// Type III_1: Powers factors, hyperfinite R_inf.
    TypeIII1,
}
/// Represents the spectrum of an operator (as a set of complex values).
#[derive(Debug, Clone)]
pub struct OperatorSpectrum {
    /// Name of the operator
    pub operator_name: String,
    /// Point spectrum (eigenvalues) as (real, imag) pairs
    pub point_spectrum: Vec<(f64, f64)>,
    /// Spectral radius
    pub spectral_radius: f64,
    /// Whether the operator is normal (N*N = NN*)
    pub is_normal: bool,
    /// Whether the operator is self-adjoint (N* = N)
    pub is_self_adjoint: bool,
    /// Whether the operator is positive (N = T*T)
    pub is_positive: bool,
}
impl OperatorSpectrum {
    /// Create a spectrum for a self-adjoint operator with given eigenvalues.
    pub fn self_adjoint(name: &str, eigenvalues: Vec<f64>) -> Self {
        let radius = eigenvalues
            .iter()
            .cloned()
            .fold(0.0_f64, |m, v| m.max(v.abs()));
        let pts = eigenvalues.into_iter().map(|v| (v, 0.0)).collect();
        OperatorSpectrum {
            operator_name: name.to_string(),
            point_spectrum: pts,
            spectral_radius: radius,
            is_normal: true,
            is_self_adjoint: true,
            is_positive: false,
        }
    }
    /// Create a spectrum for a positive operator with given eigenvalues.
    pub fn positive(name: &str, eigenvalues: Vec<f64>) -> Self {
        let mut spec = Self::self_adjoint(name, eigenvalues);
        spec.is_positive = spec.point_spectrum.iter().all(|(r, _)| *r >= 0.0);
        spec
    }
    /// Check if the spectrum is contained in the unit circle.
    pub fn is_unitary_spectrum(&self) -> bool {
        self.point_spectrum.iter().all(|(r, i)| {
            let modulus_sq = r * r + i * i;
            (modulus_sq - 1.0).abs() < 1e-10
        })
    }
    /// Compute the spectral radius (sup of |lambda|).
    pub fn compute_radius(&self) -> f64 {
        self.point_spectrum
            .iter()
            .map(|(r, i)| (r * r + i * i).sqrt())
            .fold(0.0_f64, f64::max)
    }
    /// Apply continuous functional calculus.
    pub fn apply_function<F>(&self, name: &str, f: F) -> OperatorSpectrum
    where
        F: Fn(f64, f64) -> (f64, f64),
    {
        let new_pts: Vec<(f64, f64)> = self.point_spectrum.iter().map(|&(r, i)| f(r, i)).collect();
        let radius = new_pts
            .iter()
            .map(|(r, i)| (r * r + i * i).sqrt())
            .fold(0.0_f64, f64::max);
        OperatorSpectrum {
            operator_name: name.to_string(),
            point_spectrum: new_pts,
            spectral_radius: radius,
            is_normal: true,
            is_self_adjoint: false,
            is_positive: false,
        }
    }
}
/// Represents an operator space with Ruan's axiom data.
#[derive(Debug, Clone)]
pub struct OperatorSpaceData {
    /// Name of the operator space.
    pub name: String,
    /// Whether this is a column Hilbert space.
    pub is_column_hilbert_space: bool,
    /// Whether it is self-dual (OH_n style).
    pub is_self_dual: bool,
    /// Estimate of the cb-norm.
    pub cb_norm_estimate: f64,
}
impl OperatorSpaceData {
    /// Create the column Hilbert space C_n.
    pub fn column_hilbert(n: usize) -> Self {
        OperatorSpaceData {
            name: format!("C_{}", n),
            is_column_hilbert_space: true,
            is_self_dual: false,
            cb_norm_estimate: 1.0,
        }
    }
    /// Create the row Hilbert space R_n.
    pub fn row_hilbert(n: usize) -> Self {
        OperatorSpaceData {
            name: format!("R_{}", n),
            is_column_hilbert_space: false,
            is_self_dual: false,
            cb_norm_estimate: 1.0,
        }
    }
    /// Create the operator Hilbert space OH_n (self-dual).
    pub fn operator_hilbert(n: usize) -> Self {
        OperatorSpaceData {
            name: format!("OH_{}", n),
            is_column_hilbert_space: false,
            is_self_dual: true,
            cb_norm_estimate: 1.0,
        }
    }
    /// Check the Haagerup tensor product inequality.
    pub fn haagerup_inequality_holds(&self) -> bool {
        self.cb_norm_estimate <= 1.0 + 1e-10
    }
}
/// Haagerup property.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct HaagerupProperty {
    pub group: String,
    pub has_haagerup: bool,
    pub description: String,
}
#[allow(dead_code)]
impl HaagerupProperty {
    /// Amenable groups have the Haagerup property.
    pub fn from_amenable(group: &str) -> Self {
        Self {
            group: group.to_string(),
            has_haagerup: true,
            description: "amenable implies Haagerup".to_string(),
        }
    }
    /// Free groups have the Haagerup property.
    pub fn free_group(n: usize) -> Self {
        Self {
            group: format!("F_{}", n),
            has_haagerup: true,
            description: "free group has Haagerup property (a-T-menable)".to_string(),
        }
    }
    /// Baum-Connes conjecture holds for groups with Haagerup property.
    pub fn baum_connes_holds(&self) -> bool {
        self.has_haagerup
    }
}
/// Crossed product C*-algebra.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CrossedProduct {
    pub algebra: String,
    pub group: String,
    pub action_type: String,
    pub is_full: bool,
}
#[allow(dead_code)]
impl CrossedProduct {
    /// Full crossed product A ⋊ G.
    pub fn full(algebra: &str, group: &str) -> Self {
        Self {
            algebra: algebra.to_string(),
            group: group.to_string(),
            action_type: "automorphic".to_string(),
            is_full: true,
        }
    }
    /// Reduced crossed product A ⋊_r G.
    pub fn reduced(algebra: &str, group: &str) -> Self {
        Self {
            algebra: algebra.to_string(),
            group: group.to_string(),
            action_type: "automorphic".to_string(),
            is_full: false,
        }
    }
    /// For amenable groups, full and reduced coincide.
    pub fn amenable_coincidence(&self, amenable: bool) -> bool {
        amenable
    }
}
/// Finite von Neumann algebra data.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FiniteVonNeumann {
    pub name: String,
    pub trace_name: String,
    pub is_factor: bool,
    pub murray_von_neumann_type: String,
}
#[allow(dead_code)]
impl FiniteVonNeumann {
    /// Type II_1 factor.
    pub fn type_ii1(name: &str) -> Self {
        Self {
            name: name.to_string(),
            trace_name: format!("tau_{}", name),
            is_factor: true,
            murray_von_neumann_type: "II_1".to_string(),
        }
    }
    /// Hyperfinite II_1 factor R.
    pub fn hyperfinite() -> Self {
        Self {
            name: "R".to_string(),
            trace_name: "tau".to_string(),
            is_factor: true,
            murray_von_neumann_type: "II_1".to_string(),
        }
    }
    /// L^2 space construction.
    pub fn l2_space(&self) -> String {
        format!("L²({}, {})", self.name, self.trace_name)
    }
}
/// A finite-dimensional square matrix over R.
#[derive(Debug, Clone, PartialEq)]
pub struct FiniteMatrix {
    /// Number of rows = number of columns.
    pub n: usize,
    /// Row-major storage: entry (i,j) is at index i*n + j.
    pub data: Vec<f64>,
}
impl FiniteMatrix {
    /// Construct the n x n zero matrix.
    pub fn zeros(n: usize) -> Self {
        FiniteMatrix {
            n,
            data: vec![0.0; n * n],
        }
    }
    /// Construct the n x n identity matrix.
    pub fn identity(n: usize) -> Self {
        let mut m = Self::zeros(n);
        for i in 0..n {
            m.data[i * n + i] = 1.0;
        }
        m
    }
    /// Construct a matrix from row-major data.
    pub fn from_data(n: usize, data: Vec<f64>) -> Option<Self> {
        if data.len() == n * n {
            Some(FiniteMatrix { n, data })
        } else {
            None
        }
    }
    /// Get entry (i, j).
    pub fn get(&self, i: usize, j: usize) -> f64 {
        self.data[i * self.n + j]
    }
    /// Set entry (i, j).
    pub fn set(&mut self, i: usize, j: usize, v: f64) {
        self.data[i * self.n + j] = v;
    }
    /// Matrix multiplication: self * other.
    pub fn matmul(&self, other: &Self) -> Option<Self> {
        if self.n != other.n {
            return None;
        }
        let n = self.n;
        let mut result = Self::zeros(n);
        for i in 0..n {
            for j in 0..n {
                let mut s = 0.0_f64;
                for k in 0..n {
                    s += self.get(i, k) * other.get(k, j);
                }
                result.set(i, j, s);
            }
        }
        Some(result)
    }
    /// Trace of the matrix: sum of diagonal entries.
    pub fn trace(&self) -> f64 {
        (0..self.n).map(|i| self.get(i, i)).sum()
    }
    /// Adjoint (conjugate transpose); for real matrices this is the transpose.
    pub fn adjoint(&self) -> Self {
        let n = self.n;
        let mut result = Self::zeros(n);
        for i in 0..n {
            for j in 0..n {
                result.set(i, j, self.get(j, i));
            }
        }
        result
    }
    /// Frobenius norm: sqrt(Tr(A_dagger A)).
    pub fn frobenius_norm(&self) -> f64 {
        self.data.iter().map(|&x| x * x).sum::<f64>().sqrt()
    }
    /// Check if the matrix is self-adjoint (symmetric for real: A = A^T).
    pub fn is_self_adjoint(&self) -> bool {
        let n = self.n;
        for i in 0..n {
            for j in 0..n {
                if (self.get(i, j) - self.get(j, i)).abs() > 1e-10 {
                    return false;
                }
            }
        }
        true
    }
    /// Commutator [A, B] = AB - BA.
    pub fn commutator(&self, other: &Self) -> Option<Self> {
        let ab = self.matmul(other)?;
        let ba = other.matmul(self)?;
        let n = self.n;
        let mut result = Self::zeros(n);
        for idx in 0..n * n {
            result.data[idx] = ab.data[idx] - ba.data[idx];
        }
        Some(result)
    }
    /// Commutator norm estimate ||[A,B]||_F.
    pub fn commutator_norm(&self, other: &Self) -> Option<f64> {
        Some(self.commutator(other)?.frobenius_norm())
    }
    /// Spectral decomposition for a 2x2 self-adjoint matrix.
    /// Returns (lambda1, lambda2, v1, v2) where v1, v2 are unit eigenvectors.
    pub fn spectral_decompose_2x2(&self) -> Option<(f64, f64, [f64; 2], [f64; 2])> {
        if self.n != 2 || !self.is_self_adjoint() {
            return None;
        }
        let a = self.get(0, 0);
        let b = self.get(0, 1);
        let d = self.get(1, 1);
        let tr = a + d;
        let det = a * d - b * b;
        let disc = (tr * tr / 4.0 - det).max(0.0);
        let sq = disc.sqrt();
        let lambda1 = tr / 2.0 - sq;
        let lambda2 = tr / 2.0 + sq;
        let v1 = if b.abs() > 1e-12 {
            let x = lambda1 - d;
            let norm = (x * x + b * b).sqrt();
            [x / norm, b / norm]
        } else if (a - lambda1).abs() < (d - lambda1).abs() {
            [1.0, 0.0]
        } else {
            [0.0, 1.0]
        };
        let v2 = [-v1[1], v1[0]];
        Some((lambda1, lambda2, v1, v2))
    }
}
/// AF algebra (approximately finite-dimensional) data.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AfAlgebra {
    pub name: String,
    pub bratteli_diagram: String,
    pub k0_group: String,
    pub k0_positive_cone: String,
}
#[allow(dead_code)]
impl AfAlgebra {
    /// CAR algebra (canonical anticommutation relations).
    pub fn car() -> Self {
        Self {
            name: "CAR".to_string(),
            bratteli_diagram: "Pascal triangle".to_string(),
            k0_group: "Z[1/2]".to_string(),
            k0_positive_cone: "{x >= 0}".to_string(),
        }
    }
    /// UHF algebra of type n^inf.
    pub fn uhf(n: usize) -> Self {
        Self {
            name: format!("M_{}^inf", n),
            bratteli_diagram: format!("uniform {}", n),
            k0_group: format!("Z[1/{}]", n),
            k0_positive_cone: "{x >= 0}".to_string(),
        }
    }
    /// Elliott's classification: AF algebras classified by their ordered K0 group.
    pub fn elliott_invariant(&self) -> String {
        format!("(K0={}, K0+={})", self.k0_group, self.k0_positive_cone)
    }
}
/// Represents K-theory group data (K0, K1) for a C*-algebra.
#[derive(Debug, Clone)]
pub struct KTheoryData {
    /// Name of the algebra.
    pub algebra_name: String,
    /// K0 group as a finitely-generated abelian group.
    /// Each entry is either 0 (for Z summand) or n > 0 (for Z/nZ summand).
    pub k0_summands: Vec<u64>,
    /// K1 group as a finitely-generated abelian group.
    pub k1_summands: Vec<u64>,
    /// Positive cone generator count in K0.
    pub k0_positive_generators: usize,
    /// The class of the unit [1_A] in K0.
    pub unit_class: i64,
}
impl KTheoryData {
    /// K-theory of C: K0(C) = Z, K1(C) = 0.
    pub fn complex_numbers() -> Self {
        KTheoryData {
            algebra_name: "C".to_string(),
            k0_summands: vec![0],
            k1_summands: vec![],
            k0_positive_generators: 1,
            unit_class: 1,
        }
    }
    /// K-theory of M_n(C): K0 = Z, K1 = 0.
    pub fn matrix_algebra(n: usize) -> Self {
        KTheoryData {
            algebra_name: format!("M_{}(C)", n),
            k0_summands: vec![0],
            k1_summands: vec![],
            k0_positive_generators: 1,
            unit_class: n as i64,
        }
    }
    /// K-theory of C(S^1): K0 = Z, K1 = Z.
    pub fn circle() -> Self {
        KTheoryData {
            algebra_name: "C(S^1)".to_string(),
            k0_summands: vec![0],
            k1_summands: vec![0],
            k0_positive_generators: 1,
            unit_class: 1,
        }
    }
    /// K-theory of the Cuntz algebra O_n: K0 = Z/(n-1)Z, K1 = 0.
    pub fn cuntz_algebra(n: u64) -> Self {
        KTheoryData {
            algebra_name: format!("O_{}", n),
            k0_summands: vec![n - 1],
            k1_summands: vec![],
            k0_positive_generators: 1,
            unit_class: 1,
        }
    }
    /// Check if K0 is torsion-free.
    pub fn k0_torsion_free(&self) -> bool {
        self.k0_summands.iter().all(|&s| s == 0)
    }
    /// Rank of K0 (number of Z summands).
    pub fn k0_rank(&self) -> usize {
        self.k0_summands.iter().filter(|&&s| s == 0).count()
    }
    /// Rank of K1.
    pub fn k1_rank(&self) -> usize {
        self.k1_summands.iter().filter(|&&s| s == 0).count()
    }
    /// Total Betti number: rank(K0) + rank(K1).
    pub fn total_betti(&self) -> usize {
        self.k0_rank() + self.k1_rank()
    }
}
/// Represents modular theory data for a von Neumann algebra.
#[derive(Debug, Clone)]
pub struct ModularTheoryData {
    /// Name of the von Neumann algebra
    pub algebra_name: String,
    /// The associated state
    pub state: StateData,
    /// Modular automorphism group parameters (simplified as time steps)
    pub modular_flow_steps: Vec<f64>,
    /// Connes' Sd invariant (simplified as spectral data)
    pub sd_invariant: Vec<f64>,
    /// Factor type classification
    pub factor_type: FactorType,
}
impl ModularTheoryData {
    /// Create modular theory data for a II_1 factor with tracial state.
    pub fn for_ii1_factor(name: &str) -> Self {
        ModularTheoryData {
            algebra_name: name.to_string(),
            state: StateData::tracial("tr"),
            modular_flow_steps: vec![0.0, 0.5, 1.0],
            sd_invariant: vec![1.0],
            factor_type: FactorType::TypeII1,
        }
    }
    /// Create modular theory for a III_lambda factor.
    pub fn for_iii_lambda_factor(name: &str, lambda: f64) -> Self {
        let beta = if lambda > 0.0 {
            -lambda.ln()
        } else {
            f64::INFINITY
        };
        ModularTheoryData {
            algebra_name: name.to_string(),
            state: StateData::kms("phi", beta),
            modular_flow_steps: vec![0.0, 1.0, 2.0],
            sd_invariant: (0..5).map(|n| lambda.powf(n as f64)).collect(),
            factor_type: FactorType::TypeIII(lambda),
        }
    }
    /// Compute the modular automorphism at time t.
    pub fn modular_automorphism_at(&self, t: f64) -> f64 {
        match self.factor_type {
            FactorType::TypeII1 => 0.0,
            FactorType::TypeIII(lambda) if lambda > 0.0 => {
                let period = -2.0 * std::f64::consts::PI / lambda.ln();
                t % period
            }
            _ => t,
        }
    }
    /// Check if the modular automorphism group is inner.
    pub fn modular_is_inner(&self) -> bool {
        matches!(self.factor_type, FactorType::TypeII1 | FactorType::TypeI(_))
    }
}
/// Represents the GNS triple (H_phi, pi_phi, Omega_phi) arising from a state.
#[derive(Debug, Clone)]
pub struct GNSTripleData {
    /// Name of the original algebra
    pub algebra_name: String,
    /// Name of the state
    pub state_name: String,
    /// Dimension of H_phi (None if infinite)
    pub hilbert_dim: Option<usize>,
    /// Whether the representation is irreducible
    pub is_irreducible: bool,
    /// Whether Omega_phi is cyclic
    pub omega_cyclic: bool,
    /// Whether Omega_phi is separating
    pub omega_separating: bool,
}
impl GNSTripleData {
    /// Build a GNS triple from algebra and state data.
    pub fn build(algebra: &CStarAlgebraData, state: &StateData) -> Self {
        GNSTripleData {
            algebra_name: algebra.name.clone(),
            state_name: state.name.clone(),
            hilbert_dim: algebra.dimension,
            is_irreducible: state.gns_is_irreducible(),
            omega_cyclic: true,
            omega_separating: state.is_faithful,
        }
    }
    /// Check if Tomita-Takesaki theory applies (Omega must be cyclic and separating).
    pub fn tomita_takesaki_applies(&self) -> bool {
        self.omega_cyclic && self.omega_separating
    }
}
/// Operator inequality data.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OperatorInequality {
    pub inequality_type: String,
    pub description: String,
}
#[allow(dead_code)]
impl OperatorInequality {
    /// Kadison-Schwarz inequality for unital CP maps.
    pub fn kadison_schwarz() -> Self {
        Self {
            inequality_type: "Kadison-Schwarz".to_string(),
            description: "phi(a*a) >= phi(a)*phi(a) for unital CP map phi".to_string(),
        }
    }
    /// Powers-Stormer inequality.
    pub fn powers_stormer() -> Self {
        Self {
            inequality_type: "Powers-Stormer".to_string(),
            description: "||rho^{1/2} - sigma^{1/2}||^2 <= ||rho - sigma||_1".to_string(),
        }
    }
    /// Golden-Thompson inequality: Tr(e^{A+B}) <= Tr(e^A e^B).
    pub fn golden_thompson() -> Self {
        Self {
            inequality_type: "Golden-Thompson".to_string(),
            description: "Tr(e^{A+B}) <= Tr(e^A e^B) for Hermitian A,B".to_string(),
        }
    }
}
/// Simulation of a GNS representation for M_n(C) with normalised trace.
#[derive(Debug, Clone)]
pub struct GNSRepresentationSim {
    /// Dimension n (for M_n(C)).
    pub n: usize,
    /// The cyclic vector Omega = I/sqrt(n).
    pub cyclic_vector: FiniteMatrix,
}
impl GNSRepresentationSim {
    /// Create a GNS simulation for M_n(C) with the normalised trace.
    pub fn for_matrix_algebra(n: usize) -> Self {
        let scale = 1.0 / (n as f64).sqrt();
        let mut cv = FiniteMatrix::identity(n);
        for v in cv.data.iter_mut() {
            *v *= scale;
        }
        GNSRepresentationSim {
            n,
            cyclic_vector: cv,
        }
    }
    /// Evaluate the state phi(a) = Tr(a)/n.
    pub fn evaluate_state(&self, a: &FiniteMatrix) -> Option<f64> {
        if a.n != self.n {
            return None;
        }
        Some(a.trace() / self.n as f64)
    }
    /// Apply the representation: pi(a)(b) = ab.
    pub fn apply_rep(&self, a: &FiniteMatrix, b: &FiniteMatrix) -> Option<FiniteMatrix> {
        a.matmul(b)
    }
    /// Check the GNS property: phi(a) = <Omega, pi(a) Omega>_H.
    pub fn verify_gns_property(&self, a: &FiniteMatrix) -> bool {
        if let (Some(pi_a_omega), Some(state_val)) = (
            self.apply_rep(a, &self.cyclic_vector),
            self.evaluate_state(a),
        ) {
            let omega_adj = self.cyclic_vector.adjoint();
            if let Some(product) = omega_adj.matmul(&pi_a_omega) {
                let inner = product.trace();
                (inner - state_val).abs() < 1e-9
            } else {
                false
            }
        } else {
            false
        }
    }
}
/// Represents a C*-algebra with basic structural data.
#[derive(Debug, Clone)]
pub struct CStarAlgebraData {
    /// Name of the algebra
    pub name: String,
    /// Dimension (None if infinite-dimensional)
    pub dimension: Option<usize>,
    /// Whether the algebra is commutative
    pub is_commutative: bool,
    /// Whether the algebra is nuclear
    pub is_nuclear: bool,
    /// Whether the algebra is simple
    pub is_simple: bool,
    /// Whether the algebra is unital
    pub is_unital: bool,
    /// K0 group rank (simplified)
    pub k0_rank: usize,
    /// K1 group rank (simplified)
    pub k1_rank: usize,
}
impl CStarAlgebraData {
    /// Create a new C*-algebra description.
    pub fn new(name: &str) -> Self {
        CStarAlgebraData {
            name: name.to_string(),
            dimension: None,
            is_commutative: false,
            is_nuclear: false,
            is_simple: false,
            is_unital: true,
            k0_rank: 1,
            k1_rank: 0,
        }
    }
    /// Create the full matrix algebra M_n(C).
    pub fn matrix_algebra(n: usize) -> Self {
        CStarAlgebraData {
            name: format!("M_{}(C)", n),
            dimension: Some(n * n),
            is_commutative: n == 1,
            is_nuclear: true,
            is_simple: true,
            is_unital: true,
            k0_rank: 1,
            k1_rank: 0,
        }
    }
    /// Create the commutative C*-algebra C(X) of continuous functions on compact X.
    pub fn continuous_functions(space_name: &str, k0_rank: usize) -> Self {
        CStarAlgebraData {
            name: format!("C({})", space_name),
            dimension: None,
            is_commutative: true,
            is_nuclear: true,
            is_simple: false,
            is_unital: true,
            k0_rank,
            k1_rank: 0,
        }
    }
    /// Create the compact operators K(H) on a Hilbert space H.
    pub fn compact_operators() -> Self {
        CStarAlgebraData {
            name: "K(H)".to_string(),
            dimension: None,
            is_commutative: false,
            is_nuclear: true,
            is_simple: true,
            is_unital: false,
            k0_rank: 1,
            k1_rank: 0,
        }
    }
    /// Create the Cuntz algebra O_n.
    pub fn cuntz_algebra(n: usize) -> Self {
        CStarAlgebraData {
            name: format!("O_{}", n),
            dimension: None,
            is_commutative: false,
            is_nuclear: true,
            is_simple: true,
            is_unital: true,
            k0_rank: 1,
            k1_rank: 0,
        }
    }
    /// Check if this algebra satisfies the UCT (Universal Coefficient Theorem).
    pub fn satisfies_uct(&self) -> bool {
        self.is_nuclear
    }
    /// Check if this algebra is AF (approximately finite-dimensional).
    pub fn is_af(&self) -> bool {
        self.is_nuclear && self.k1_rank == 0
    }
}
/// A Fredholm operator with computed index.
#[derive(Debug, Clone)]
pub struct FredholmData {
    /// Label for this operator.
    pub label: String,
    /// Dimension of the kernel.
    pub kernel_dim: usize,
    /// Dimension of the cokernel.
    pub cokernel_dim: usize,
}
impl FredholmData {
    /// Create a Fredholm operator with known kernel and cokernel dimensions.
    pub fn new(label: &str, kernel_dim: usize, cokernel_dim: usize) -> Self {
        FredholmData {
            label: label.to_string(),
            kernel_dim,
            cokernel_dim,
        }
    }
    /// The Fredholm index: ind(T) = dim(ker T) - dim(coker T).
    pub fn index(&self) -> i64 {
        self.kernel_dim as i64 - self.cokernel_dim as i64
    }
    /// Atkinson's theorem: T is Fredholm iff its image in the Calkin algebra is invertible.
    pub fn calkin_invertible(&self) -> bool {
        true
    }
    /// Index stability under compact perturbations.
    pub fn index_stable_under_compact_perturbation(&self, other: &FredholmData) -> bool {
        self.index() == other.index()
    }
}
/// An element of a C*-algebra with norm and involution data.
#[derive(Debug, Clone)]
pub struct CStarElem {
    /// A human-readable label for this element.
    pub label: String,
    /// The underlying matrix representation (None if abstract).
    pub matrix: Option<FiniteMatrix>,
    /// Precomputed operator norm (or approximation).
    pub norm: f64,
    /// Whether a* = a (self-adjoint).
    pub is_self_adjoint: bool,
    /// Whether a*a = aa* (normal).
    pub is_normal: bool,
    /// Whether a*a = 1 (unitary).
    pub is_unitary: bool,
    /// Whether a^2 = a (projection).
    pub is_projection: bool,
}
impl CStarElem {
    /// Create an abstract element with given label and norm.
    pub fn abstract_elem(label: &str, norm: f64) -> Self {
        CStarElem {
            label: label.to_string(),
            matrix: None,
            norm,
            is_self_adjoint: false,
            is_normal: false,
            is_unitary: false,
            is_projection: false,
        }
    }
    /// Create an element from a finite matrix.
    pub fn from_matrix(label: &str, m: FiniteMatrix) -> Self {
        let norm = m.frobenius_norm();
        let adj = m.adjoint();
        let is_sa = m.is_self_adjoint();
        let is_normal = if let (Some(a_adj_a), Some(a_a_adj)) = (adj.matmul(&m), m.matmul(&adj)) {
            let diff_norm = a_adj_a
                .data
                .iter()
                .zip(a_a_adj.data.iter())
                .map(|(x, y)| (x - y).abs())
                .fold(0.0_f64, f64::max);
            diff_norm < 1e-9
        } else {
            false
        };
        let id = FiniteMatrix::identity(m.n);
        let is_unitary = if let Some(a_adj_a) = adj.matmul(&m) {
            let diff: f64 = a_adj_a
                .data
                .iter()
                .zip(id.data.iter())
                .map(|(x, y)| (x - y).abs())
                .fold(0.0_f64, f64::max);
            diff < 1e-9
        } else {
            false
        };
        let is_proj = if let Some(a2) = m.matmul(&m) {
            let diff: f64 = a2
                .data
                .iter()
                .zip(m.data.iter())
                .map(|(x, y)| (x - y).abs())
                .fold(0.0_f64, f64::max);
            diff < 1e-9
        } else {
            false
        };
        CStarElem {
            label: label.to_string(),
            matrix: Some(m),
            norm,
            is_self_adjoint: is_sa,
            is_normal,
            is_unitary,
            is_projection: is_proj,
        }
    }
    /// Compute the involution a* (adjoint).
    pub fn involution(&self) -> Option<CStarElem> {
        let adj_mat = self.matrix.as_ref()?.adjoint();
        Some(CStarElem::from_matrix(&format!("{}*", self.label), adj_mat))
    }
    /// Verify the C*-identity using Frobenius norm approximation.
    pub fn verify_c_star_identity(&self) -> bool {
        if let Some(m) = &self.matrix {
            let adj = m.adjoint();
            if let Some(adj_a) = adj.matmul(m) {
                let norm_adj_a = adj_a.frobenius_norm();
                let norm_sq = self.norm * self.norm;
                (norm_adj_a - norm_sq).abs() < norm_sq * 0.1 + 1e-10
            } else {
                false
            }
        } else {
            true
        }
    }
}
/// Represents a state (positive linear functional of norm 1) on a C*-algebra.
#[derive(Debug, Clone)]
pub struct StateData {
    /// Name of the state
    pub name: String,
    /// Whether the state is faithful
    pub is_faithful: bool,
    /// Whether the state is tracial (phi(ab) = phi(ba))
    pub is_tracial: bool,
    /// Whether the state is a KMS state
    pub is_kms: bool,
    /// Inverse temperature beta for KMS states
    pub beta: Option<f64>,
    /// Whether this is a pure state
    pub is_pure: bool,
}
impl StateData {
    /// Create a new generic state.
    pub fn new(name: &str) -> Self {
        StateData {
            name: name.to_string(),
            is_faithful: false,
            is_tracial: false,
            is_kms: false,
            beta: None,
            is_pure: false,
        }
    }
    /// Create a tracial state.
    pub fn tracial(name: &str) -> Self {
        StateData {
            name: name.to_string(),
            is_faithful: true,
            is_tracial: true,
            is_kms: true,
            beta: Some(0.0),
            is_pure: false,
        }
    }
    /// Create a KMS state at inverse temperature beta.
    pub fn kms(name: &str, beta: f64) -> Self {
        StateData {
            name: name.to_string(),
            is_faithful: true,
            is_tracial: (beta - 0.0_f64).abs() < 1e-10,
            is_kms: true,
            beta: Some(beta),
            is_pure: false,
        }
    }
    /// Create a pure state (corresponds to irreducible representation).
    pub fn pure(name: &str) -> Self {
        StateData {
            name: name.to_string(),
            is_faithful: false,
            is_tracial: false,
            is_kms: false,
            beta: None,
            is_pure: true,
        }
    }
    /// Check if the GNS representation will be irreducible.
    pub fn gns_is_irreducible(&self) -> bool {
        self.is_pure
    }
    /// Check the KMS condition for the state.
    pub fn check_kms_condition(&self) -> bool {
        self.is_kms && self.beta.is_some()
    }
}
/// Cuntz semigroup data.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CuntzSemigroup {
    pub algebra: String,
    pub is_almost_unperforated: bool,
    pub is_almost_divisible: bool,
}
#[allow(dead_code)]
impl CuntzSemigroup {
    /// For Z-stable algebras, the Cuntz semigroup is well-behaved.
    pub fn z_stable(name: &str) -> Self {
        Self {
            algebra: name.to_string(),
            is_almost_unperforated: true,
            is_almost_divisible: true,
        }
    }
    /// Toms-Winter regularity.
    pub fn toms_winter_regularity(&self) -> bool {
        self.is_almost_unperforated && self.is_almost_divisible
    }
}
/// Spectral triple (noncommutative geometry data).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SpectralTripleData {
    pub algebra: String,
    pub hilbert_space: String,
    pub dirac_operator: String,
    pub is_even: bool,
    pub spectral_dim: f64,
}
#[allow(dead_code)]
impl SpectralTripleData {
    /// Standard spectral triple on n-torus.
    pub fn torus(n: usize) -> Self {
        Self {
            algebra: format!("C(T^{})", n),
            hilbert_space: format!("L²(T^{}, S)", n),
            dirac_operator: format!("D_T^{}", n),
            is_even: n % 2 == 0,
            spectral_dim: n as f64,
        }
    }
    /// Connes' distance formula: d(x,y) = sup { |f(x)-f(y)| : ||[D,f]|| ≤ 1 }.
    pub fn distance_formula(&self) -> String {
        format!(
            "d(x,y) = sup{{|f(x)-f(y)| : ||[{},π(f)]|| ≤ 1}}",
            self.dirac_operator
        )
    }
}
/// Completely positive map data.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CompletelyPositiveMap {
    pub source: String,
    pub target: String,
    pub is_unital: bool,
    pub is_trace_preserving: bool,
}
#[allow(dead_code)]
impl CompletelyPositiveMap {
    /// Unital CP map (a quantum channel without trace preservation requirement).
    pub fn unital(source: &str, target: &str) -> Self {
        Self {
            source: source.to_string(),
            target: target.to_string(),
            is_unital: true,
            is_trace_preserving: false,
        }
    }
    /// Quantum channel: unital and trace preserving.
    pub fn quantum_channel(source: &str, target: &str) -> Self {
        Self {
            source: source.to_string(),
            target: target.to_string(),
            is_unital: true,
            is_trace_preserving: true,
        }
    }
    /// Stinespring dilation theorem applies.
    pub fn stinespring_applies(&self) -> bool {
        self.is_unital || self.is_trace_preserving
    }
    /// Description.
    pub fn description(&self) -> String {
        format!(
            "CP map {} → {}, unital={}, trace-preserving={}",
            self.source, self.target, self.is_unital, self.is_trace_preserving
        )
    }
}
/// Operator system data.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OperatorSystem {
    pub name: String,
    pub dimension: usize,
    pub is_nuclear: bool,
}
#[allow(dead_code)]
impl OperatorSystem {
    /// Finite-dimensional operator system.
    pub fn finite(name: &str, dim: usize) -> Self {
        Self {
            name: name.to_string(),
            dimension: dim,
            is_nuclear: true,
        }
    }
    /// Arveson extension theorem: every CP map on a subspace extends to full C*-algebra.
    pub fn arveson_extension_applies(&self) -> bool {
        true
    }
    /// Completely bounded norm description.
    pub fn cb_norm_description(&self) -> String {
        format!("CB norm on {} (dim {})", self.name, self.dimension)
    }
}
/// Nuclearity data for C*-algebras.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct NuclearityData {
    pub algebra: String,
    pub is_nuclear: bool,
    pub decomposition_rank: Option<usize>,
}
#[allow(dead_code)]
impl NuclearityData {
    /// Nuclear C*-algebra.
    pub fn nuclear(name: &str) -> Self {
        Self {
            algebra: name.to_string(),
            is_nuclear: true,
            decomposition_rank: None,
        }
    }
    /// With finite decomposition rank.
    pub fn with_dr(name: &str, dr: usize) -> Self {
        Self {
            algebra: name.to_string(),
            is_nuclear: true,
            decomposition_rank: Some(dr),
        }
    }
    /// Kirchberg-Phillips: nuclear simple purely infinite C*-algebras classified by K-theory.
    pub fn kirchberg_phillips_applies(&self, is_simple_purely_infinite: bool) -> bool {
        self.is_nuclear && is_simple_purely_infinite
    }
}
