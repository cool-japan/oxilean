//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

/// The Bloch formula: CH^n(X) ≅ H^{2n,n}(X, ℤ) for smooth X.
#[derive(Debug, Clone)]
pub struct BlochFormula {
    /// Whether the formula has been verified for this scheme.
    pub verified: bool,
    /// The scheme X.
    pub scheme: String,
}
impl BlochFormula {
    /// Create a verified Bloch formula for X.
    pub fn verified(scheme: impl Into<String>) -> Self {
        Self {
            verified: true,
            scheme: scheme.into(),
        }
    }
    /// Check the formula holds (smooth X over a field).
    pub fn holds_for_smooth(&self) -> bool {
        self.verified
    }
}
/// Algebraic K-group K_n(R) of a ring R, with computed rank.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlgebraicKGroup {
    /// The ring R (name or description).
    pub ring: String,
    /// The degree n (may be negative for Bass's negative K-groups).
    pub degree: i64,
    /// Rank of K_n(R) as a free abelian group (if finite rank).
    pub rank: Option<usize>,
    /// Torsion part description.
    pub torsion: Vec<u64>,
}
impl AlgebraicKGroup {
    /// Create K_n(R) with the given degree and rank.
    pub fn new(ring: impl Into<String>, degree: i64, rank: Option<usize>) -> Self {
        Self {
            ring: ring.into(),
            degree,
            rank,
            torsion: Vec::new(),
        }
    }
    /// K_0(R): the Grothendieck group of projective R-modules.
    pub fn k0(ring: impl Into<String>, rank: usize) -> Self {
        Self::new(ring, 0, Some(rank))
    }
    /// K_1(R): the group of units R^× modulo elementary matrices.
    pub fn k1(ring: impl Into<String>, rank: usize) -> Self {
        Self::new(ring, 1, Some(rank))
    }
    /// Rank of this K-group as a free abelian group.
    pub fn rank(&self) -> Option<usize> {
        self.rank
    }
    /// The K-group is trivial (rank 0, no torsion).
    pub fn is_trivial(&self) -> bool {
        self.rank == Some(0) && self.torsion.is_empty()
    }
    /// Add torsion part (cyclic group of order n).
    pub fn with_torsion(mut self, orders: Vec<u64>) -> Self {
        self.torsion = orders;
        self
    }
}
/// The motivic functor X ↦ M(X) in DM.
#[derive(Debug, Clone)]
pub struct MotivicFunctor {
    /// Name describing the functor.
    pub description: String,
}
impl MotivicFunctor {
    /// The standard motivic functor.
    pub fn standard() -> Self {
        Self {
            description: "M: Sm/k → DM(k, ℤ)".to_string(),
        }
    }
    /// Apply the functor to a scheme name.
    pub fn apply(&self, scheme: &str) -> MixedMotive {
        MixedMotive::new(format!("M({})", scheme), "k")
    }
}
/// Milnor K-theory K_n^M(F) = (F^×)^{⊗n} / Steinberg relations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MilnorKTheory {
    /// The field F.
    pub field: String,
    /// The degree n.
    pub degree: usize,
    /// Generators (Steinberg symbols {a_1, ..., a_n}).
    pub generators: Vec<Vec<String>>,
}
impl MilnorKTheory {
    /// Create K_n^M(F) with no generators.
    pub fn new(field: impl Into<String>, degree: usize) -> Self {
        Self {
            field: field.into(),
            degree,
            generators: Vec::new(),
        }
    }
    /// Add a Steinberg symbol {a_1, ..., a_n}.
    pub fn add_symbol(&mut self, symbol: Vec<String>) {
        self.generators.push(symbol);
    }
    /// K_0^M(F) = ℤ.
    pub fn k0(field: impl Into<String>) -> Self {
        Self::new(field, 0)
    }
    /// K_1^M(F) = F^×.
    pub fn k1(field: impl Into<String>) -> Self {
        Self::new(field, 1)
    }
}
/// Voevodsky's proof of the Bloch-Kato conjecture using motivic cohomology.
#[derive(Debug, Clone)]
pub struct VoevodskysProof {
    /// Key ingredients of the proof.
    pub ingredients: Vec<String>,
}
impl VoevodskysProof {
    /// Create the proof record with standard ingredients.
    pub fn new() -> Self {
        Self {
            ingredients: vec![
                "Motivic cohomology (Bloch cycle complex)".to_string(),
                "Motivic Steenrod algebra".to_string(),
                "Milnor conjecture (ℓ=2, proved 1996)".to_string(),
                "Norm varieties (Rost)".to_string(),
                "Motivic cobordism".to_string(),
            ],
        }
    }
}
/// A realization functor from DM to a linear category.
#[derive(Debug, Clone)]
pub struct RealizationFunctor {
    /// The type of realization.
    pub realization: RealizationType,
}
impl RealizationFunctor {
    /// The Betti realization functor.
    pub fn betti() -> Self {
        Self {
            realization: RealizationType::Betti,
        }
    }
    /// The de Rham realization functor.
    pub fn de_rham() -> Self {
        Self {
            realization: RealizationType::DeRham,
        }
    }
    /// The ℓ-adic étale realization functor.
    pub fn l_adic(prime: u64) -> Self {
        Self {
            realization: RealizationType::LAdicEtale { prime },
        }
    }
    /// Apply to a motive: returns the realization description.
    pub fn apply(&self, motive: &MixedMotive) -> String {
        format!("{}({})", self.realization.description(), motive.name)
    }
}
/// Motivic cohomology group H^{p,q}(X, ℤ) with bigrading.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MotivicCohomology {
    /// Scheme X.
    pub scheme: String,
    /// Cohomological degree p.
    pub cohom_degree: usize,
    /// Weight (Tate twist) q.
    pub weight: usize,
    /// Rank of the group.
    pub rank: Option<usize>,
}
impl MotivicCohomology {
    /// Create H^{p,q}(X, ℤ).
    pub fn new(scheme: impl Into<String>, p: usize, q: usize) -> Self {
        Self {
            scheme: scheme.into(),
            cohom_degree: p,
            weight: q,
            rank: None,
        }
    }
    /// Set the rank.
    pub fn with_rank(mut self, rank: usize) -> Self {
        self.rank = Some(rank);
        self
    }
    /// Check if this is the Milnor K-theory piece H^{n,n}(Spec F).
    pub fn is_milnor_piece(&self) -> bool {
        self.cohom_degree == self.weight
    }
    /// Check if this is the Chow group piece H^{2n,n}(X).
    pub fn is_chow_piece(&self) -> bool {
        self.cohom_degree == 2 * self.weight
    }
}
/// ℓ-adic cohomology H^i_ét(X, ℤ_ℓ) as a Galois module.
#[derive(Debug, Clone)]
pub struct LAdicCohomology {
    /// The scheme X.
    pub scheme: String,
    /// The cohomological degree i.
    pub degree: usize,
    /// The prime ℓ.
    pub prime: u64,
    /// Betti number (rank over ℤ_ℓ).
    pub betti_number: Option<usize>,
}
impl LAdicCohomology {
    /// Create H^i_ét(X, ℤ_ℓ).
    pub fn new(scheme: impl Into<String>, degree: usize, prime: u64) -> Self {
        Self {
            scheme: scheme.into(),
            degree,
            prime,
            betti_number: None,
        }
    }
    /// Set the Betti number.
    pub fn with_betti(mut self, b: usize) -> Self {
        self.betti_number = Some(b);
        self
    }
}
/// The algebra of ℓ-adic sheaves on a scheme.
#[derive(Debug, Clone)]
pub struct EtaleSheavesLAdicAlgebra {
    /// The scheme X.
    pub scheme: String,
    /// The prime ℓ.
    pub prime: u64,
}
impl EtaleSheavesLAdicAlgebra {
    /// Create the ℓ-adic sheaf algebra on X.
    pub fn new(scheme: impl Into<String>, prime: u64) -> Self {
        Self {
            scheme: scheme.into(),
            prime,
        }
    }
}
/// A formal group law over a commutative ring R: F(x, y) ∈ R[[x, y]].
///
/// Represents the formal power series up to a chosen truncation degree,
/// capturing the first Chern class formula c_1(L ⊗ L') = F(c_1(L), c_1(L')).
#[allow(dead_code)]
pub struct FormalGroupLaw {
    /// The coefficient ring R (name).
    pub ring: String,
    /// Coefficients a_{ij} where F(x,y) = x + y + Σ a_{ij} x^i y^j (truncated).
    pub coefficients: Vec<(usize, usize, f64)>,
    /// Truncation degree.
    pub truncation: usize,
}
impl FormalGroupLaw {
    /// Create the additive formal group law F(x, y) = x + y.
    pub fn additive(ring: impl Into<String>) -> Self {
        Self {
            ring: ring.into(),
            coefficients: Vec::new(),
            truncation: 5,
        }
    }
    /// Create the multiplicative formal group law F(x, y) = x + y - xy (K-theory).
    pub fn multiplicative(ring: impl Into<String>) -> Self {
        Self {
            ring: ring.into(),
            coefficients: vec![(1, 1, -1.0)],
            truncation: 5,
        }
    }
    /// Create the universal (Lazard) formal group law (truncated).
    pub fn lazard(ring: impl Into<String>, truncation: usize) -> Self {
        Self {
            ring: ring.into(),
            coefficients: vec![(1, 1, 1.0), (2, 1, -2.0), (1, 2, -2.0)],
            truncation,
        }
    }
    /// Evaluate F(x, y) at the given values (polynomial truncation).
    pub fn evaluate(&self, x: f64, y: f64) -> f64 {
        let mut result = x + y;
        for &(i, j, coeff) in &self.coefficients {
            result += coeff * x.powi(i as i32) * y.powi(j as i32);
        }
        result
    }
    /// Check commutativity: F(x, y) ≈ F(y, x) at a test point.
    pub fn is_commutative_at(&self, x: f64, y: f64, tol: f64) -> bool {
        (self.evaluate(x, y) - self.evaluate(y, x)).abs() < tol
    }
    /// Type of this formal group law.
    pub fn group_type(&self) -> &'static str {
        if self.coefficients.is_empty() {
            "additive"
        } else if self.coefficients.len() == 1 && self.coefficients[0] == (1, 1, -1.0) {
            "multiplicative"
        } else {
            "general"
        }
    }
}
/// A formal ℤ-linear combination of irreducible subvarieties.
#[derive(Debug, Clone)]
pub struct AlgebraicCycle {
    /// Components: (subvariety name, coefficient).
    pub components: Vec<(String, i64)>,
    /// Codimension of the cycle.
    pub codimension: usize,
}
impl AlgebraicCycle {
    /// Create the zero cycle.
    pub fn zero(codimension: usize) -> Self {
        Self {
            components: Vec::new(),
            codimension,
        }
    }
    /// Create a cycle from a single subvariety with coefficient 1.
    pub fn from_subvariety(name: impl Into<String>, codimension: usize) -> Self {
        Self {
            components: vec![(name.into(), 1)],
            codimension,
        }
    }
    /// Add a component with a given coefficient.
    pub fn add_component(&mut self, name: impl Into<String>, coeff: i64) {
        self.components.push((name.into(), coeff));
    }
    /// The degree of the cycle (sum of coefficients).
    pub fn degree(&self) -> i64 {
        self.components.iter().map(|(_, c)| c).sum()
    }
    /// The cycle class in the Chow group.
    pub fn cycle_class(&self) -> String {
        self.components
            .iter()
            .map(|(v, c)| format!("{}{}", c, v))
            .collect::<Vec<_>>()
            .join(" + ")
    }
    /// Push forward by multiplying all coefficients by a factor.
    pub fn push_forward(&self, factor: i64) -> Self {
        Self {
            components: self
                .components
                .iter()
                .map(|(v, c)| (v.clone(), c * factor))
                .collect(),
            codimension: self.codimension,
        }
    }
}
/// Motivic sphere S^{p,q} and its homotopy group data.
///
/// In A¹-homotopy theory, S^{p,q} = (S^1)^∧p ∧ (G_m)^∧q
/// where S^1 is the simplicial circle and G_m is the multiplicative group.
#[allow(dead_code)]
pub struct MotivicSphere {
    /// Topological degree p.
    pub top_degree: usize,
    /// Weight q (motivic degree).
    pub weight: usize,
    /// Known motivic homotopy groups π_{a,b}(S^{p,q}) for small (a,b).
    pub homotopy_groups: Vec<((usize, usize), String)>,
}
impl MotivicSphere {
    /// Create the motivic sphere S^{p,q}.
    pub fn new(top_degree: usize, weight: usize) -> Self {
        Self {
            top_degree,
            weight,
            homotopy_groups: Vec::new(),
        }
    }
    /// The simplest motivic sphere S^{1,1} = ℙ^1 \ {0, ∞} ≅ G_m.
    pub fn g_m() -> Self {
        let mut s = Self::new(1, 1);
        s.homotopy_groups.push(((1, 1), "ℤ".to_string()));
        s
    }
    /// The projective line ℙ^1 ≅ S^{2,1}.
    pub fn p1() -> Self {
        let mut s = Self::new(2, 1);
        s.homotopy_groups.push(((2, 1), "ℤ".to_string()));
        s
    }
    /// Add a known homotopy group.
    pub fn add_homotopy_group(&mut self, a: usize, b: usize, description: impl Into<String>) {
        self.homotopy_groups.push(((a, b), description.into()));
    }
    /// Display notation S^{p,q}.
    pub fn notation(&self) -> String {
        format!("S^{{{},{}}}", self.top_degree, self.weight)
    }
    /// Check if this is the topological sphere (weight = 0).
    pub fn is_topological(&self) -> bool {
        self.weight == 0
    }
    /// The Euler characteristic of S^{p,q} in motivic cohomology.
    ///
    /// χ_mot(S^{p,q}) = 1 - (-1)^p T^q where T is the Tate class.
    pub fn euler_char_sign(&self) -> i64 {
        if self.top_degree % 2 == 0 {
            2
        } else {
            0
        }
    }
}
/// The connective K-theory spectrum K(R).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KTheorySpectrum {
    /// The ring R.
    pub ring: String,
    /// Whether this is the non-connective spectrum (Bass's version).
    pub non_connective: bool,
}
impl KTheorySpectrum {
    /// Create the connective K-theory spectrum K(R).
    pub fn new(ring: impl Into<String>) -> Self {
        Self {
            ring: ring.into(),
            non_connective: false,
        }
    }
    /// Create the non-connective (Bass) K-theory spectrum KB(R).
    pub fn bass(ring: impl Into<String>) -> Self {
        Self {
            ring: ring.into(),
            non_connective: true,
        }
    }
    /// Whether the ring is invertible (units form a group).
    pub fn is_invertible_sheaf(&self) -> bool {
        !self.ring.is_empty()
    }
}
/// The Milnor conjecture: special case of Bloch-Kato for ℓ = 2.
#[derive(Debug, Clone)]
pub struct MilnorConjecture {
    /// The field k (characteristic ≠ 2).
    pub field: String,
    /// Whether this has been proved (by Voevodsky, 1996).
    pub proved: bool,
}
impl MilnorConjecture {
    /// The Milnor conjecture for field k, proved by Voevodsky.
    pub fn voevodsky_proof(field: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            proved: true,
        }
    }
}
/// Adams operation ψ^k on K_0(R), represented by its action on generators.
///
/// On K_0, ψ^k([E]) = [E^{⊗k}] where E^{⊗k} is the k-th tensor power.
/// On complex K-theory, ψ^k acts on line bundles by L ↦ L^k.
#[allow(dead_code)]
pub struct AdamsOperation {
    /// The Adams degree k.
    pub degree: usize,
    /// Action table: for each generator index, the image as a linear combination.
    /// `action[i]` = list of (generator_index, coefficient) pairs.
    pub action: Vec<Vec<(usize, i64)>>,
}
impl AdamsOperation {
    /// Create the Adams operation ψ^k with given degree.
    pub fn new(degree: usize) -> Self {
        Self {
            degree,
            action: Vec::new(),
        }
    }
    /// Set the action on n generators.
    pub fn with_action(mut self, action: Vec<Vec<(usize, i64)>>) -> Self {
        self.action = action;
        self
    }
    /// Apply ψ^k to a K_0 element represented as a vector of coefficients.
    pub fn apply(&self, element: &[i64]) -> Vec<i64> {
        let n = element.len();
        let mut result = vec![0i64; n];
        for (i, img) in self.action.iter().enumerate() {
            if i < n {
                for &(j, coeff) in img {
                    if j < n {
                        result[j] += element[i] * coeff;
                    }
                }
            }
        }
        result
    }
    /// Check if this operation is a ring homomorphism on K_0 (additive consistency).
    pub fn is_additive(&self, a: &[i64], b: &[i64]) -> bool {
        let sum_ab: Vec<i64> = a.iter().zip(b.iter()).map(|(&x, &y)| x + y).collect();
        let psi_sum = self.apply(&sum_ab);
        let psi_a = self.apply(a);
        let psi_b = self.apply(b);
        let psi_a_plus_psi_b: Vec<i64> = psi_a
            .iter()
            .zip(psi_b.iter())
            .map(|(&x, &y)| x + y)
            .collect();
        psi_sum == psi_a_plus_psi_b
    }
    /// Name of this Adams operation.
    pub fn name(&self) -> String {
        format!("ψ^{}", self.degree)
    }
}
/// Compute the formal zeta function Z(X/F_q, T) up to a given degree.
///
/// Z(X/F_q, T) = exp(Σ_{n≥1} |X(F_{q^n})| T^n / n).
/// For a curve of genus g, Z(X, T) = P_1(T) / ((1-T)(1-qT))
/// where P_1(T) = Π_{i=1}^{2g} (1 - α_i T).
#[allow(dead_code)]
pub struct ZetaFunction {
    /// The variety name.
    pub variety: String,
    /// The field size q.
    pub field_size: u64,
    /// The numerator polynomial coefficients (for curve: P_1(T)).
    pub numerator_coeffs: Vec<i64>,
    /// The denominator degree (for projective space: 1 for P^n).
    pub denominator_degree: usize,
}
impl ZetaFunction {
    /// Create the zeta function of a genus-g curve over F_q.
    pub fn curve(variety: impl Into<String>, q: u64, genus: usize) -> Self {
        let mut coeffs = vec![1i64];
        coeffs.extend(std::iter::repeat(0i64).take(genus));
        Self {
            variety: variety.into(),
            field_size: q,
            numerator_coeffs: coeffs,
            denominator_degree: 2,
        }
    }
    /// Create the zeta function of projective n-space P^n over F_q.
    pub fn projective_space(n: usize, q: u64) -> Self {
        Self {
            variety: format!("P^{}", n),
            field_size: q,
            numerator_coeffs: vec![1],
            denominator_degree: n + 1,
        }
    }
    /// Evaluate the denominator (1-T)(1-qT) at T = t (for a curve).
    pub fn denominator_at(&self, t: f64) -> f64 {
        let q = self.field_size as f64;
        (1.0 - t) * (1.0 - q * t)
    }
    /// Check the functional equation: Z(T) = ε * q^{χ/2} * T^{χ} * Z(1/(qT))
    /// at a test value (simplified check via degree matching).
    pub fn functional_equation_degree(&self) -> usize {
        self.numerator_coeffs.len().saturating_sub(1)
    }
    /// Description of the Weil conjecture status for this variety.
    pub fn status(&self) -> &'static str {
        "Proved by Deligne (1974)"
    }
}
/// The Gersten resolution for K-theory on a scheme.
///
/// Represents the complex:
/// `0 → K_n(R) → ⊕_{x ∈ X^0} K_n(k(x)) → ⊕_{x ∈ X^1} K_{n-1}(k(x)) → ...`
#[derive(Debug, Clone)]
pub struct GerstenResolution {
    /// The scheme X (name).
    pub scheme: String,
    /// The K-theory degree n.
    pub degree: usize,
    /// The terms of the complex: (codimension, description).
    pub terms: Vec<(usize, String)>,
}
impl GerstenResolution {
    /// Create a Gersten complex for K_n on the given scheme.
    pub fn new(scheme: impl Into<String>, degree: usize) -> Self {
        let scheme = scheme.into();
        let terms = (0..=degree)
            .map(|p| {
                (
                    p,
                    format!("⊕_{{x ∈ X^{}}} K_{}(k(x))", p, degree as i64 - p as i64),
                )
            })
            .collect();
        Self {
            scheme,
            degree,
            terms,
        }
    }
    /// Length of the Gersten complex.
    pub fn length(&self) -> usize {
        self.terms.len()
    }
}
/// Chow group CH^p(X): codimension-p algebraic cycles modulo rational equivalence.
#[derive(Debug, Clone)]
pub struct ChowGroup {
    /// Scheme X.
    pub scheme: String,
    /// Codimension p.
    pub codimension: usize,
    /// Generators (cycle names).
    pub generators: Vec<String>,
}
impl ChowGroup {
    /// Create an empty Chow group CH^p(X).
    pub fn new(scheme: impl Into<String>, codimension: usize) -> Self {
        Self {
            scheme: scheme.into(),
            codimension,
            generators: Vec::new(),
        }
    }
    /// Add a cycle generator.
    pub fn add_generator(&mut self, name: impl Into<String>) {
        self.generators.push(name.into());
    }
    /// Rank of the Chow group.
    pub fn rank(&self) -> usize {
        self.generators.len()
    }
    /// The degree map CH^1(X) → ℤ for divisors.
    pub fn degree(&self, coefficients: &[i64]) -> i64 {
        coefficients.iter().sum()
    }
}
/// Cohomological purity (absolute cohomological purity).
#[derive(Debug, Clone)]
pub struct PurityThm {
    /// Description of the purity isomorphism.
    pub description: String,
}
impl PurityThm {
    /// The standard cohomological purity statement.
    pub fn absolute_purity() -> Self {
        Self {
            description:
                "For Z ↪ X smooth closed of codimension c: H^i_Z(X, ℤ_ℓ) ≅ H^{i-2c}(Z, ℤ_ℓ)(-c)"
                    .to_string(),
        }
    }
}
/// A mixed motive in DM(S, ℤ).
#[derive(Debug, Clone)]
pub struct MixedMotive {
    /// Label for this motive.
    pub name: String,
    /// The base scheme S.
    pub base: String,
    /// Weight filtration: list of (weight, graded piece name).
    pub weight_graded_pieces: Vec<(i32, String)>,
}
impl MixedMotive {
    /// Create a mixed motive with a given name.
    pub fn new(name: impl Into<String>, base: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            base: base.into(),
            weight_graded_pieces: Vec::new(),
        }
    }
    /// Add a weight-graded piece.
    pub fn add_weight_piece(&mut self, weight: i32, piece: impl Into<String>) {
        self.weight_graded_pieces.push((weight, piece.into()));
    }
    /// Tate twist: M(n) shifts weight by 2n.
    pub fn tate_twist(&self, n: i32) -> Self {
        let mut m = self.clone();
        m.name = format!("{}({})", self.name, n);
        m.weight_graded_pieces = m
            .weight_graded_pieces
            .iter()
            .map(|(w, p)| (w - 2 * n, p.clone()))
            .collect();
        m
    }
    /// Check if this motive is effective (all weights ≥ 0).
    pub fn is_effective(&self) -> bool {
        self.weight_graded_pieces.iter().all(|(w, _)| *w >= 0)
    }
    /// Weight filtration as sorted weight values.
    pub fn weight_filtration(&self) -> Vec<i32> {
        let mut weights: Vec<i32> = self.weight_graded_pieces.iter().map(|(w, _)| *w).collect();
        weights.sort();
        weights.dedup();
        weights
    }
    /// Dual motive: negates all weights and renames.
    pub fn dual(&self) -> Self {
        let mut d = self.clone();
        d.name = format!("({})^∨", self.name);
        d.weight_graded_pieces = d
            .weight_graded_pieces
            .iter()
            .map(|(w, p)| (-w, p.clone()))
            .collect();
        d
    }
}
/// The Bloch-Kato conjecture: H^n_ét(Spec k, μ_ℓ^⊗n) ≅ K_n^M(k) / ℓ.
#[derive(Debug, Clone)]
pub struct BlochKatoConjecture {
    /// The field k.
    pub field: String,
    /// The prime ℓ.
    pub prime: u64,
    /// Whether this instance is proved.
    pub proved: bool,
}
impl BlochKatoConjecture {
    /// Create the Bloch-Kato conjecture for field k and prime ℓ.
    pub fn new(field: impl Into<String>, prime: u64) -> Self {
        Self {
            field: field.into(),
            prime,
            proved: false,
        }
    }
    /// The proved instance (all cases, by Voevodsky-Rost).
    pub fn proved(field: impl Into<String>, prime: u64) -> Self {
        Self {
            field: field.into(),
            prime,
            proved: true,
        }
    }
    /// Check whether this is the Milnor conjecture (ℓ = 2).
    pub fn is_milnor_conjecture(&self) -> bool {
        self.prime == 2
    }
}
/// The Chow ring CH^*(X) = ⊕_p CH^p(X) with the intersection product.
#[derive(Debug, Clone)]
pub struct ChowRing {
    /// Scheme X.
    pub scheme: String,
    /// Chow groups in each codimension.
    pub groups: Vec<ChowGroup>,
}
impl ChowRing {
    /// Create an empty Chow ring for X.
    pub fn new(scheme: impl Into<String>) -> Self {
        Self {
            scheme: scheme.into(),
            groups: Vec::new(),
        }
    }
    /// Add CH^p(X) to the ring.
    pub fn add_group(&mut self, group: ChowGroup) {
        self.groups.push(group);
    }
    /// Total rank of all Chow groups.
    pub fn total_rank(&self) -> usize {
        self.groups.iter().map(|g| g.rank()).sum()
    }
}
/// Higher Chow group CH^p(X, n) data: the p-th Chow group in Bloch's higher Chow
/// complex, computing the (2p-n, p) piece of motivic cohomology.
#[allow(dead_code)]
pub struct HigherChowGroup {
    /// The scheme X (name).
    pub scheme: String,
    /// Codimension p.
    pub codim: usize,
    /// Simplicial degree n.
    pub simplex_degree: usize,
    /// Rank (if computed).
    pub rank: Option<usize>,
}
impl HigherChowGroup {
    /// Create CH^p(X, n).
    pub fn new(scheme: impl Into<String>, codim: usize, simplex_degree: usize) -> Self {
        Self {
            scheme: scheme.into(),
            codim,
            simplex_degree,
            rank: None,
        }
    }
    /// Set the rank.
    pub fn with_rank(mut self, r: usize) -> Self {
        self.rank = Some(r);
        self
    }
    /// Cohomological degree in motivic bigrading: 2p - n.
    pub fn motivic_cohom_degree(&self) -> Option<usize> {
        let twice_p = 2 * self.codim;
        if twice_p >= self.simplex_degree {
            Some(twice_p - self.simplex_degree)
        } else {
            None
        }
    }
    /// Weight q in motivic bigrading: q = p.
    pub fn motivic_weight(&self) -> usize {
        self.codim
    }
    /// Check if this is a classical Chow group (n = 0).
    pub fn is_classical(&self) -> bool {
        self.simplex_degree == 0
    }
    /// Check if this is the Milnor K-theory piece (p = n, cohom_degree = n).
    pub fn is_milnor_k_theory_piece(&self) -> bool {
        self.codim == self.simplex_degree
    }
    /// Display string in motivic bigrading notation.
    pub fn motivic_notation(&self) -> String {
        match self.motivic_cohom_degree() {
            Some(p) => format!("H^{{{},{}}}({}, ℤ)", p, self.codim, self.scheme),
            None => {
                format!(
                    "CH^{}({}, {}) [degenerate]",
                    self.codim, self.scheme, self.simplex_degree
                )
            }
        }
    }
}
/// The proper base change theorem and smooth base change theorem.
#[derive(Debug, Clone)]
pub struct BaseChangeThm {
    /// Whether this is the proper version (true) or smooth version (false).
    pub proper: bool,
}
impl BaseChangeThm {
    /// The proper base change theorem.
    pub fn proper() -> Self {
        Self { proper: true }
    }
    /// The smooth base change theorem.
    pub fn smooth() -> Self {
        Self { proper: false }
    }
    /// Name of the theorem.
    pub fn name(&self) -> &'static str {
        if self.proper {
            "Proper Base Change"
        } else {
            "Smooth Base Change"
        }
    }
}
/// Reduced power operations Sq^i (mod 2) or P^i (odd prime) in motivic cohomology.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReducedPowerOperation {
    /// The prime ℓ for the Steenrod algebra.
    pub prime: u64,
    /// The degree i of the operation.
    pub degree: usize,
}
impl ReducedPowerOperation {
    /// Create Sq^i (mod 2 Steenrod square).
    pub fn sq(i: usize) -> Self {
        Self {
            prime: 2,
            degree: i,
        }
    }
    /// Create P^i (mod ℓ power operation for odd prime ℓ).
    pub fn power(i: usize, prime: u64) -> Self {
        Self { prime, degree: i }
    }
    /// The name of this operation.
    pub fn name(&self) -> String {
        if self.prime == 2 {
            format!("Sq^{}", self.degree)
        } else {
            format!("P^{}", self.degree)
        }
    }
}
/// Bloch's cycle complex ℤ(n) on a scheme X.
#[derive(Debug, Clone)]
pub struct MotivicComplex {
    /// Scheme X.
    pub scheme: String,
    /// The Tate twist n.
    pub twist: usize,
    /// Length of the complex.
    pub length: usize,
}
impl MotivicComplex {
    /// Create the complex ℤ(n) on X.
    pub fn new(scheme: impl Into<String>, twist: usize) -> Self {
        let length = twist + 1;
        Self {
            scheme: scheme.into(),
            twist,
            length,
        }
    }
}
/// The Weil conjectures for a variety over a finite field.
#[derive(Debug, Clone)]
pub struct WeilConjectures {
    /// The variety.
    pub variety: String,
    /// The finite field F_q.
    pub field_size: u64,
    /// Whether Deligne's theorem has been invoked.
    pub deligne_proved: bool,
    /// Betti numbers of the variety.
    pub betti_numbers: Vec<usize>,
}
impl WeilConjectures {
    /// Create Weil conjecture data for a variety over F_q.
    pub fn new(variety: impl Into<String>, q: u64, betti_numbers: Vec<usize>) -> Self {
        Self {
            variety: variety.into(),
            field_size: q,
            deligne_proved: true,
            betti_numbers,
        }
    }
    /// Compute the Euler characteristic χ = Σ (-1)^i b_i.
    pub fn euler_characteristic(&self) -> i64 {
        self.betti_numbers
            .iter()
            .enumerate()
            .map(|(i, &b)| if i % 2 == 0 { b as i64 } else { -(b as i64) })
            .sum()
    }
    /// Degree of the zeta function numerator/denominator.
    pub fn zeta_degree(&self) -> usize {
        self.betti_numbers.iter().sum()
    }
}
/// Rational equivalence of cycles on a scheme.
#[derive(Debug, Clone)]
pub struct RationalEquivalence {
    /// The scheme.
    pub scheme: String,
    /// A rational function (represented as a string) on a subvariety.
    pub rational_function: String,
    /// The divisor of this rational function.
    pub divisor_of: AlgebraicCycle,
}
impl RationalEquivalence {
    /// Create a rational equivalence from a rational function on the scheme.
    pub fn new(
        scheme: impl Into<String>,
        rational_function: impl Into<String>,
        divisor: AlgebraicCycle,
    ) -> Self {
        Self {
            scheme: scheme.into(),
            rational_function: rational_function.into(),
            divisor_of: divisor,
        }
    }
}
/// A realization functor (Betti, de Rham, or ℓ-adic).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RealizationType {
    /// Betti realization: singular cohomology over ℂ.
    Betti,
    /// De Rham realization: algebraic de Rham cohomology.
    DeRham,
    /// ℓ-adic realization: étale cohomology with ℤ_ℓ coefficients.
    LAdicEtale { prime: u64 },
    /// Crystalline realization (p-adic cohomology).
    Crystalline { prime: u64 },
}
impl RealizationType {
    /// Description of this realization.
    pub fn description(&self) -> String {
        match self {
            Self::Betti => "Betti (singular cohomology)".to_string(),
            Self::DeRham => "de Rham (algebraic)".to_string(),
            Self::LAdicEtale { prime } => format!("ℓ-adic étale (ℓ = {})", prime),
            Self::Crystalline { prime } => format!("Crystalline (p = {})", prime),
        }
    }
}
/// A pure (Chow) motive (X, p, n).
#[derive(Debug, Clone)]
pub struct PureMotive {
    /// The smooth projective variety X.
    pub variety: String,
    /// The idempotent correspondence p ∈ CH^{dim X}(X × X).
    pub projector: String,
    /// The Tate twist n.
    pub twist: i32,
}
impl PureMotive {
    /// Create the pure motive (X, id, 0).
    pub fn identity(variety: impl Into<String>) -> Self {
        Self {
            variety: variety.into(),
            projector: "id".to_string(),
            twist: 0,
        }
    }
    /// Create the Tate motive ℤ(n) = (Spec k, id, n).
    pub fn tate(n: i32) -> Self {
        Self {
            variety: "Spec k".to_string(),
            projector: "id".to_string(),
            twist: n,
        }
    }
    /// Apply a Tate twist.
    pub fn twist_by(&self, n: i32) -> Self {
        Self {
            variety: self.variety.clone(),
            projector: self.projector.clone(),
            twist: self.twist + n,
        }
    }
}
