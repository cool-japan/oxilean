//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use std::collections::HashMap;

/// A combinatorial knot diagram: collection of crossings.
#[derive(Debug, Clone)]
pub struct KnotDiagram {
    /// Human-readable name, e.g. "trefoil".
    pub name: String,
    /// Number of components (1 = knot, ≥2 = link).
    pub components: usize,
    /// Ordered list of crossings.
    pub crossings: Vec<CrossingData>,
}
impl KnotDiagram {
    /// Create a new diagram.
    pub fn new(name: impl Into<String>, components: usize) -> Self {
        Self {
            name: name.into(),
            components,
            crossings: Vec::new(),
        }
    }
    /// Add a crossing.
    pub fn add_crossing(&mut self, c: CrossingData) {
        self.crossings.push(c);
    }
    /// Writhe: algebraic sum of all crossing signs.
    pub fn writhe(&self) -> i32 {
        self.crossings.iter().map(|c| c.sign.value()).sum()
    }
    /// Crossing number of this diagram (upper bound on the knot crossing number).
    pub fn diagram_crossing_number(&self) -> usize {
        self.crossings.len()
    }
    /// The unknot has a standard diagram with 0 crossings.
    pub fn unknot() -> Self {
        Self::new("unknot", 1)
    }
    /// The trefoil knot (left-handed) — 3 negative crossings.
    pub fn left_trefoil() -> Self {
        let mut d = Self::new("3_1 (left trefoil)", 1);
        for i in 1..=3 {
            d.add_crossing(CrossingData::new(
                i,
                CrossingSignVal::Negative,
                [2 * i - 2, 2 * i - 1, 2 * i, (2 * i + 1) % 6],
            ));
        }
        d
    }
    /// The figure-eight knot — two positive and two negative crossings.
    pub fn figure_eight() -> Self {
        let mut d = Self::new("4_1 (figure-eight)", 1);
        let signs = [
            CrossingSignVal::Positive,
            CrossingSignVal::Negative,
            CrossingSignVal::Positive,
            CrossingSignVal::Negative,
        ];
        for (i, &s) in signs.iter().enumerate() {
            let l = i + 1;
            d.add_crossing(CrossingData::new(
                l,
                s,
                [2 * i, 2 * i + 1, (2 * i + 2) % 8, (2 * i + 3) % 8],
            ));
        }
        d
    }
    /// Hopf link — two components with 2 positive crossings.
    pub fn hopf_link() -> Self {
        let mut d = Self::new("Hopf link", 2);
        for i in 1..=2 {
            d.add_crossing(CrossingData::new(
                i,
                CrossingSignVal::Positive,
                [2 * i - 2, 2 * i - 1, 2 * i % 4, (2 * i + 1) % 4],
            ));
        }
        d
    }
}
/// A Lorenz knot specified by its symbolic sequence on the Lorenz template.
#[derive(Debug, Clone)]
pub struct LorenzKnotData {
    /// The symbolic itinerary: 'L' (left branch) or 'R' (right branch).
    pub symbol_sequence: String,
    /// Period of the orbit.
    pub period: usize,
}
impl LorenzKnotData {
    /// Construct a Lorenz knot from a symbolic word.
    pub fn new(seq: impl Into<String>) -> Self {
        let s: String = seq.into();
        let period = s.len();
        Self {
            symbol_sequence: s,
            period,
        }
    }
    /// The torus knot T(2,3) appears as the Lorenz knot "LLR".
    pub fn trefoil_lorenz() -> Self {
        Self::new("LLR")
    }
    /// T(2,5) appears as "LLRRR".
    pub fn torus_knot_t25() -> Self {
        Self::new("LLRRR")
    }
    /// Number of L symbols in the sequence.
    pub fn left_count(&self) -> usize {
        self.symbol_sequence.chars().filter(|&c| c == 'L').count()
    }
    /// Number of R symbols in the sequence.
    pub fn right_count(&self) -> usize {
        self.symbol_sequence.chars().filter(|&c| c == 'R').count()
    }
    /// Lorenz knots are always fibered: returns true.
    pub fn is_fibered(&self) -> bool {
        true
    }
    /// Lorenz knots are always positive: returns true.
    pub fn is_positive(&self) -> bool {
        true
    }
}
/// Alexander polynomial represented by integer coefficients.
/// coefficients[i] is the coefficient of t^i (starting from the lowest power).
#[derive(Debug, Clone)]
pub struct AlexanderPolynomial {
    pub coefficients: Vec<i64>,
}
impl AlexanderPolynomial {
    pub fn new(coefficients: Vec<i64>) -> Self {
        AlexanderPolynomial { coefficients }
    }
    /// Evaluate the polynomial at t.
    pub fn evaluate_at(&self, t: f64) -> f64 {
        self.coefficients
            .iter()
            .enumerate()
            .map(|(i, &c)| c as f64 * t.powi(i as i32))
            .sum()
    }
    /// Determinant = |Δ(-1)|.
    pub fn determinant(&self) -> i64 {
        let val = self.evaluate_at(-1.0).round() as i64;
        val.abs()
    }
    /// Signature: number of sign changes in the sequence of Fourier-like values (simplified).
    pub fn signature(&self) -> i32 {
        let mut sign_changes = 0i32;
        let mut prev_nonzero: Option<i64> = None;
        for &c in &self.coefficients {
            if c != 0 {
                if let Some(prev) = prev_nonzero {
                    if (prev > 0) != (c > 0) {
                        sign_changes += 1;
                    }
                }
                prev_nonzero = Some(c);
            }
        }
        sign_changes
    }
}
/// Jones polynomial (symbolic, stored as degree and variable name).
#[derive(Debug, Clone)]
pub struct JonesPolynomial {
    pub variable: String,
    pub degree: i32,
}
impl JonesPolynomial {
    pub fn new(variable: impl Into<String>, degree: i32) -> Self {
        JonesPolynomial {
            variable: variable.into(),
            degree,
        }
    }
    /// Evaluate at a specific value of q (simplified: treat as q^degree).
    pub fn evaluate_at(&self, q: f64) -> f64 {
        q.powi(self.degree)
    }
    /// The Jones polynomial of the mirror image is obtained by q → q^{-1}.
    pub fn mirror_image_jones(&self) -> Self {
        JonesPolynomial {
            variable: self.variable.clone(),
            degree: -self.degree,
        }
    }
}
/// A knot cobordism between two knots.
#[derive(Debug, Clone)]
pub struct KnotCobordism {
    pub genus: u32,
    pub is_concordance: bool,
}
impl KnotCobordism {
    pub fn new(genus: u32, is_concordance: bool) -> Self {
        KnotCobordism {
            genus,
            is_concordance,
        }
    }
    /// A knot is slice if it bounds a smooth disk in B^4 (genus=0 concordance).
    pub fn is_slice(&self) -> bool {
        self.genus == 0 && self.is_concordance
    }
    /// The 4-genus (smooth slice genus).
    pub fn four_genus(&self) -> u32 {
        self.genus
    }
}
/// Seifert matrix computer.
///
/// Represents and computes invariants from a Seifert matrix V.
#[derive(Debug, Clone)]
pub struct SeifertMatrixComputer {
    /// The Seifert matrix V (stored as row-major Vec<Vec<i64>>).
    pub matrix: Vec<Vec<i64>>,
    /// Size of the matrix (2g × 2g for a genus-g surface).
    pub size: usize,
}
impl SeifertMatrixComputer {
    /// Create a Seifert matrix computer from a 2D matrix.
    pub fn new(matrix: Vec<Vec<i64>>) -> Self {
        let size = matrix.len();
        SeifertMatrixComputer { matrix, size }
    }
    /// Standard Seifert matrix for the trefoil knot:
    /// V = [[-1, 0], [1, -1]].
    pub fn trefoil() -> Self {
        Self::new(vec![vec![-1, 0], vec![1, -1]])
    }
    /// Standard Seifert matrix for the figure-eight knot:
    /// V = [[-1, 1], [0, 1]].
    pub fn figure_eight() -> Self {
        Self::new(vec![vec![-1, 1], vec![0, 1]])
    }
    /// Compute V + Vᵀ (the symmetrized matrix used for signature and determinant).
    pub fn symmetrized(&self) -> Vec<Vec<i64>> {
        let n = self.size;
        let mut sym = vec![vec![0i64; n]; n];
        for i in 0..n {
            for j in 0..n {
                sym[i][j] = self.matrix[i][j] + self.matrix[j][i];
            }
        }
        sym
    }
    /// Determinant of the 2×2 symmetrized matrix (for genus-1 Seifert surfaces).
    pub fn determinant_2x2(&self) -> i64 {
        if self.size != 2 {
            return 0;
        }
        let s = self.symmetrized();
        s[0][0] * s[1][1] - s[0][1] * s[1][0]
    }
    /// Knot determinant: det(K) = |det(V + Vᵀ)| for genus-1 surfaces (2×2 case).
    pub fn knot_determinant(&self) -> u64 {
        self.determinant_2x2().unsigned_abs()
    }
    /// Signature σ(K) = number of positive eigenvalues minus number of negative.
    /// For 2×2: sign of trace and determinant determine the signature.
    pub fn signature_2x2(&self) -> i32 {
        if self.size != 2 {
            return 0;
        }
        let s = self.symmetrized();
        let tr = s[0][0] + s[1][1];
        let det = s[0][0] * s[1][1] - s[0][1] * s[1][0];
        if det > 0 && tr > 0 {
            2
        } else if det > 0 && tr < 0 {
            -2
        } else {
            0
        }
    }
    /// Alexander polynomial at t = -1 (= knot determinant, normalized).
    /// det(t V - Vᵀ) at t=-1 for 2×2.
    pub fn alexander_at_minus_one(&self) -> i64 {
        if self.size != 2 {
            return 1;
        }
        self.determinant_2x2().abs()
    }
}
/// A knot specified by its crossing number, alternating status, and fibered status.
#[derive(Debug, Clone)]
pub struct Knot {
    pub crossing_number: u32,
    pub is_alternating: bool,
    pub is_fibered: bool,
}
impl Knot {
    pub fn new(crossing_number: u32, is_alternating: bool, is_fibered: bool) -> Self {
        Knot {
            crossing_number,
            is_alternating,
            is_fibered,
        }
    }
    /// Seifert genus: for alternating knots, genus = (crossing_number + 1) / 2 (simplified).
    pub fn genus(&self) -> u32 {
        if self.crossing_number == 0 {
            return 0;
        }
        (self.crossing_number + 1) / 2
    }
    /// Unknotting number (simplified estimate).
    pub fn unknotting_number(&self) -> u32 {
        self.crossing_number / 2
    }
    /// A knot is a torus knot T(p,q) if it is fibered and alternating (simplified heuristic).
    pub fn is_torus_knot(&self) -> bool {
        self.is_fibered && self.is_alternating
    }
}
/// Classical knot invariants representable as polynomials.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KnotInvariant {
    Jones,
    Alexander,
    HOMFLY,
    Conway,
    Arf,
}
impl KnotInvariant {
    /// Degree of the polynomial invariant (approximate upper bound).
    pub fn polynomial_degree(&self) -> i32 {
        match self {
            KnotInvariant::Jones => 3,
            KnotInvariant::Alexander => 2,
            KnotInvariant::HOMFLY => 4,
            KnotInvariant::Conway => 2,
            KnotInvariant::Arf => 0,
        }
    }
    /// Whether this invariant can detect the unknot.
    pub fn detects_unknot(&self) -> bool {
        match self {
            KnotInvariant::Alexander => false,
            KnotInvariant::Jones => true,
            KnotInvariant::HOMFLY => true,
            KnotInvariant::Conway => false,
            KnotInvariant::Arf => false,
        }
    }
}
/// A Seifert surface for a knot/link.
#[derive(Debug, Clone)]
pub struct SeifertSurface {
    pub genus: u32,
    pub euler_char: i32,
}
impl SeifertSurface {
    pub fn new(genus: u32, euler_char: i32) -> Self {
        SeifertSurface { genus, euler_char }
    }
    /// A simplified Seifert matrix (returns dimensions as a string for illustration).
    pub fn seifert_matrix(&self) -> String {
        let size = 2 * self.genus as usize;
        format!(
            "{}×{} Seifert matrix for genus-{} surface",
            size, size, self.genus
        )
    }
    /// A Seifert surface is a fiber surface iff it has minimal genus and is connected.
    pub fn is_fiber_surface(&self) -> bool {
        self.euler_char == 1 - 2 * self.genus as i32
    }
}
/// Sign of a crossing in a knot diagram.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrossingSignVal {
    /// A right-handed (positive) crossing: contributes +1 to writhe.
    Positive,
    /// A left-handed (negative) crossing: contributes -1 to writhe.
    Negative,
}
impl CrossingSignVal {
    /// Integer value of the crossing sign.
    pub fn value(self) -> i32 {
        match self {
            CrossingSignVal::Positive => 1,
            CrossingSignVal::Negative => -1,
        }
    }
    /// Reverse the sign.
    pub fn negate(self) -> Self {
        match self {
            CrossingSignVal::Positive => CrossingSignVal::Negative,
            CrossingSignVal::Negative => CrossingSignVal::Positive,
        }
    }
}
/// A single crossing in a knot diagram.
#[derive(Debug, Clone)]
pub struct CrossingData {
    /// Numerical label (1-indexed in standard usage).
    pub label: usize,
    /// Sign (+1 or -1).
    pub sign: CrossingSignVal,
    /// The four arc indices at this crossing: [over_in, over_out, under_in, under_out].
    pub arcs: [usize; 4],
}
impl CrossingData {
    /// Construct a new crossing.
    pub fn new(label: usize, sign: CrossingSignVal, arcs: [usize; 4]) -> Self {
        Self { label, sign, arcs }
    }
}
/// A rational tangle specified by its continued fraction.
/// The fraction p/q classifies the tangle up to isotopy.
#[derive(Debug, Clone)]
pub struct RationalTangle {
    pub numerator: i64,
    pub denominator: i64,
}
impl RationalTangle {
    /// The zero tangle (horizontal strands).
    pub fn zero() -> Self {
        Self {
            numerator: 0,
            denominator: 1,
        }
    }
    /// The infinity tangle (vertical strands).
    pub fn infinity() -> Self {
        Self {
            numerator: 1,
            denominator: 0,
        }
    }
    /// Integer tangle n (n twists).
    pub fn integer(n: i64) -> Self {
        Self {
            numerator: n,
            denominator: 1,
        }
    }
    /// Add two rational tangles: n(T1 + T2) = n(T1) + n(T2).
    pub fn add(&self, other: &Self) -> Self {
        let num = self.numerator * other.denominator + other.numerator * self.denominator;
        let den = self.denominator * other.denominator;
        let g = gcd(num.unsigned_abs(), den.unsigned_abs());
        if g == 0 {
            Self {
                numerator: num,
                denominator: den,
            }
        } else {
            Self {
                numerator: num / g as i64,
                denominator: den / g as i64,
            }
        }
    }
    /// Numerator closure of the tangle: N(T) is a knot/link with fraction p/q.
    pub fn numerator_closure_component_count(&self) -> usize {
        gcd(
            self.numerator.unsigned_abs(),
            self.denominator.unsigned_abs(),
        ) as usize
    }
}
/// A Laurent polynomial in one variable with integer coefficients.
/// Represented as (min_exp, coefficients) where coefficients[k] is the
/// coefficient of t^{min_exp + k}.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaurentPoly {
    /// The lowest (possibly negative) exponent present.
    pub min_exp: i32,
    /// Coefficients in ascending order of exponent.
    pub coeffs: Vec<i64>,
}
impl LaurentPoly {
    /// Zero polynomial.
    pub fn zero() -> Self {
        Self {
            min_exp: 0,
            coeffs: vec![0],
        }
    }
    /// Monomial c * t^e.
    pub fn monomial(c: i64, e: i32) -> Self {
        Self {
            min_exp: e,
            coeffs: vec![c],
        }
    }
    /// Evaluate at t = 1 (the "augmentation").
    pub fn eval_at_one(&self) -> i64 {
        self.coeffs.iter().sum()
    }
    /// Degree span (max_exp - min_exp).
    pub fn span(&self) -> usize {
        if self.coeffs.is_empty() {
            0
        } else {
            self.coeffs.len() - 1
        }
    }
    /// Coefficient of t^e (0 if out of range).
    pub fn coeff(&self, e: i32) -> i64 {
        let idx = e - self.min_exp;
        if idx < 0 || idx as usize >= self.coeffs.len() {
            0
        } else {
            self.coeffs[idx as usize]
        }
    }
    /// Add two Laurent polynomials.
    pub fn add(&self, other: &Self) -> Self {
        let lo = self.min_exp.min(other.min_exp);
        let hi = (self.min_exp + self.coeffs.len() as i32 - 1)
            .max(other.min_exp + other.coeffs.len() as i32 - 1);
        if hi < lo {
            return Self::zero();
        }
        let len = (hi - lo + 1) as usize;
        let mut c = vec![0i64; len];
        for (k, &v) in self.coeffs.iter().enumerate() {
            let idx = (self.min_exp + k as i32 - lo) as usize;
            c[idx] += v;
        }
        for (k, &v) in other.coeffs.iter().enumerate() {
            let idx = (other.min_exp + k as i32 - lo) as usize;
            c[idx] += v;
        }
        Self {
            min_exp: lo,
            coeffs: c,
        }
    }
}
/// A braid word in B_n.
#[derive(Debug, Clone)]
pub struct BraidWord {
    /// Number of strands.
    pub strands: usize,
    /// Sequence of Artin generators.
    pub word: Vec<BraidGen>,
}
impl BraidWord {
    /// Create the identity braid on n strands.
    pub fn identity(strands: usize) -> Self {
        Self {
            strands,
            word: Vec::new(),
        }
    }
    /// Algebraic length (sum of signs of generators).
    pub fn algebraic_length(&self) -> i32 {
        self.word
            .iter()
            .map(|g| if g.positive { 1 } else { -1 })
            .sum()
    }
    /// Word length.
    pub fn word_length(&self) -> usize {
        self.word.len()
    }
    /// Whether this is a positive braid (all σ_i, no inverses).
    pub fn is_positive(&self) -> bool {
        self.word.iter().all(|g| g.positive)
    }
    /// Append a generator.
    pub fn push(&mut self, gen: BraidGen) {
        self.word.push(gen);
    }
    /// Standard trefoil braid: σ_1 σ_1 σ_1 in B_2 (closed to the left trefoil).
    pub fn trefoil_braid() -> Self {
        let mut b = Self::identity(2);
        b.push(BraidGen::sigma(1));
        b.push(BraidGen::sigma(1));
        b.push(BraidGen::sigma(1));
        b
    }
    /// Hopf link braid: σ_1 σ_1 in B_2.
    pub fn hopf_braid() -> Self {
        let mut b = Self::identity(2);
        b.push(BraidGen::sigma(1));
        b.push(BraidGen::sigma(1));
        b
    }
    /// Torus knot T(p, q) braid: (σ_1 ⋯ σ_{q-1})^p in B_q.
    pub fn torus_knot_braid(p: usize, q: usize) -> Self {
        assert!(q >= 2, "q must be at least 2");
        let mut b = Self::identity(q);
        for _ in 0..p {
            for i in 1..q {
                b.push(BraidGen::sigma(i));
            }
        }
        b
    }
}
/// Khovanov homology of a knot, stored as a list of (homological_degree, q_degree, rank).
#[derive(Debug, Clone)]
pub struct KhovanovHomology {
    pub knot: String,
    pub bigraded: Vec<(i32, i32, i32)>,
}
impl KhovanovHomology {
    pub fn new(knot: impl Into<String>, bigraded: Vec<(i32, i32, i32)>) -> Self {
        KhovanovHomology {
            knot: knot.into(),
            bigraded,
        }
    }
    /// The graded Euler characteristic of Kh should equal the Jones polynomial.
    pub fn euler_char_is_jones(&self) -> bool {
        true
    }
    /// Total rank = sum of all ranks.
    pub fn rank(&self) -> i32 {
        self.bigraded.iter().map(|&(_, _, r)| r).sum()
    }
}
/// HOMFLY-PT polynomial in two variables (v, z).
///
/// Stored as a map from (v-exponent, z-exponent) → coefficient.
#[derive(Debug, Clone)]
pub struct HOMFLYPolynomial {
    /// Coefficients indexed by (v_exp, z_exp).
    pub coeffs: std::collections::HashMap<(i32, i32), i64>,
    /// Knot name (for display).
    pub knot_name: String,
}
impl HOMFLYPolynomial {
    /// Create a new (zero) HOMFLY polynomial.
    pub fn zero(knot_name: impl Into<String>) -> Self {
        HOMFLYPolynomial {
            coeffs: std::collections::HashMap::new(),
            knot_name: knot_name.into(),
        }
    }
    /// Set the coefficient of v^a z^b.
    pub fn set(&mut self, v_exp: i32, z_exp: i32, coeff: i64) {
        if coeff == 0 {
            self.coeffs.remove(&(v_exp, z_exp));
        } else {
            self.coeffs.insert((v_exp, z_exp), coeff);
        }
    }
    /// Get the coefficient of v^a z^b.
    pub fn get(&self, v_exp: i32, z_exp: i32) -> i64 {
        self.coeffs.get(&(v_exp, z_exp)).copied().unwrap_or(0)
    }
    /// HOMFLY polynomial of the unknot: P = 1.
    pub fn unknot() -> Self {
        let mut p = Self::zero("unknot");
        p.set(0, 0, 1);
        p
    }
    /// HOMFLY polynomial of the left trefoil:
    /// P = -v⁻⁴ + 2v⁻² + v⁻²z².
    pub fn left_trefoil() -> Self {
        let mut p = Self::zero("3_1 left trefoil");
        p.set(-4, 0, -1);
        p.set(-2, 0, 2);
        p.set(-2, 2, 1);
        p
    }
    /// HOMFLY polynomial of the right trefoil:
    /// P = -v⁴ + 2v² + v²z².
    pub fn right_trefoil() -> Self {
        let mut p = Self::zero("3_1 right trefoil");
        p.set(4, 0, -1);
        p.set(2, 0, 2);
        p.set(2, 2, 1);
        p
    }
    /// HOMFLY polynomial of the figure-eight knot (amphichiral):
    /// P = v⁻² - 1 + v² + z².
    pub fn figure_eight() -> Self {
        let mut p = Self::zero("4_1 figure-eight");
        p.set(-2, 0, 1);
        p.set(0, 0, -1);
        p.set(2, 0, 1);
        p.set(0, 2, 1);
        p
    }
    /// Evaluate at v = q^a and z = q^b - q^{-b} (Jones specialization: v=q, z=q^{1/2}-q^{-1/2}).
    /// Here we just compute the polynomial value at floating-point (v_val, z_val).
    pub fn evaluate(&self, v_val: f64, z_val: f64) -> f64 {
        let mut result = 0.0;
        for (&(v_exp, z_exp), &coeff) in &self.coeffs {
            result += coeff as f64 * v_val.powi(v_exp) * z_val.powi(z_exp);
        }
        result
    }
    /// Check amphichirality: P(v, z) = P(v⁻¹, z) (true for figure-eight, false for trefoils).
    pub fn is_amphichiral(&self) -> bool {
        let mut reflected = HOMFLYPolynomial::zero(&self.knot_name);
        for (&(v_exp, z_exp), &coeff) in &self.coeffs {
            *reflected.coeffs.entry((-v_exp, z_exp)).or_insert(0) += coeff;
        }
        reflected.coeffs == self.coeffs
    }
    /// Number of terms (monomials with nonzero coefficient).
    pub fn num_terms(&self) -> usize {
        self.coeffs.len()
    }
}
/// Kauffman bracket calculator.
///
/// Computes the Kauffman bracket of a diagram represented by its
/// crossings as a sequence of signs.
#[derive(Debug, Clone)]
pub struct KauffmanBracketCalc {
    /// The variable A (usually a formal variable; here a real for evaluation).
    pub a_val: f64,
}
impl KauffmanBracketCalc {
    /// Create a calculator with the given value for A.
    pub fn new(a_val: f64) -> Self {
        KauffmanBracketCalc { a_val }
    }
    /// Loop value: δ = -A² - A⁻².
    pub fn loop_value(&self) -> f64 {
        -(self.a_val * self.a_val) - 1.0 / (self.a_val * self.a_val)
    }
    /// Kauffman bracket for the unknot (1 loop): ⟨○⟩ = 1.
    pub fn bracket_unknot(&self) -> f64 {
        1.0
    }
    /// Kauffman bracket for a diagram with n extra disjoint loops:
    /// ⟨D ∪ n×○⟩ = δⁿ ⟨D⟩.
    pub fn bracket_with_loops(&self, diagram_val: f64, extra_loops: u32) -> f64 {
        diagram_val * self.loop_value().powi(extra_loops as i32)
    }
    /// Compute bracket of a positive crossing via skein:
    /// ⟨X₊⟩ = A ⟨D₀⟩ + A⁻¹ ⟨D∞⟩.
    pub fn positive_crossing(&self, d0: f64, d_inf: f64) -> f64 {
        self.a_val * d0 + d_inf / self.a_val
    }
    /// Compute bracket of a negative crossing via skein:
    /// ⟨X₋⟩ = A⁻¹ ⟨D₀⟩ + A ⟨D∞⟩.
    pub fn negative_crossing(&self, d0: f64, d_inf: f64) -> f64 {
        d0 / self.a_val + self.a_val * d_inf
    }
    /// Unnormalized Kauffman bracket for the trefoil (3 positive crossings).
    /// The exact value of ⟨3_1⟩ evaluated at variable A.
    /// Result: -A^5 - A^{-3} + A^{-7} (for right trefoil).
    pub fn right_trefoil_bracket(&self) -> f64 {
        let a = self.a_val;
        -a.powi(5) - a.powi(-3) + a.powi(-7)
    }
    /// Jones polynomial specialization: set A = -q^{-1/4}, so t = q = A^{-4}.
    /// V_K(t) = (-A³)^{-w} ⟨K⟩ where w = writhe.
    pub fn jones_from_bracket(&self, bracket_val: f64, writhe: i32) -> f64 {
        let normalization = (-self.a_val.powi(3)).powi(-writhe);
        normalization * bracket_val
    }
}
/// Hyperbolic volume estimator for knot complements.
///
/// Provides reference volumes and estimates for common knots.
#[derive(Debug, Clone)]
pub struct HyperbolicVolumeEstimator {
    /// Look-up table: (knot_name, volume).
    pub table: Vec<(String, f64)>,
}
impl HyperbolicVolumeEstimator {
    /// Create an estimator with a standard table.
    pub fn new() -> Self {
        HyperbolicVolumeEstimator {
            table: vec![
                ("4_1".into(), 2.0298832128),
                ("5_2".into(), 2.8281220883),
                ("6_1".into(), 3.1639612690),
                ("6_2".into(), 4.4002091222),
                ("6_3".into(), 5.6931184087),
                ("7_4".into(), 5.1375856880),
                ("8_18".into(), 7.3277247359),
                ("trefoil".into(), 0.0),
            ],
        }
    }
    /// Look up the hyperbolic volume by knot name.
    /// Returns None for torus knots (volume = 0) or unknown knots.
    pub fn volume(&self, knot_name: &str) -> Option<f64> {
        for (name, vol) in &self.table {
            if name == knot_name {
                return if *vol > 0.0 { Some(*vol) } else { None };
            }
        }
        None
    }
    /// Estimate volume for a knot with n crossings using the Lackenby bound:
    /// vol(K) ≥ (t(K)-2) * v_8 / 2 where v_8 = 3.6638... is the regular ideal octahedron volume.
    pub fn lackenby_bound(&self, crossing_number: u32) -> f64 {
        let v_oct = 3.6638623767_f64;
        if crossing_number < 3 {
            return 0.0;
        }
        ((crossing_number as f64) - 2.0) * v_oct / 2.0
    }
    /// Dehn filling volume: after (p,q)-filling, volume decreases monotonically to 0.
    /// Approximation: vol(K(p/q)) ≈ vol(K) * (1 - 1/p²) for large p.
    pub fn dehn_filling_volume(&self, knot_name: &str, p: i64) -> f64 {
        let base = self.volume(knot_name).unwrap_or(0.0);
        if p == 0 {
            return 0.0;
        }
        base * (1.0 - 1.0 / (p * p) as f64).max(0.0)
    }
    /// Figure-eight knot is the simplest hyperbolic knot: vol = 2.029...
    pub fn figure_eight_volume(&self) -> f64 {
        2.0298832128
    }
    /// Volume conjecture (Kashaev): lim_{N→∞} (2π/N) ln |J_N(K; e^{2πi/N})| = vol(K).
    /// Returns the conjectured limit value (stored reference volume).
    pub fn volume_conjecture_limit(&self, knot_name: &str) -> Option<f64> {
        self.volume(knot_name)
    }
    /// Check if a knot is hyperbolic (has a positive volume in the table).
    pub fn is_hyperbolic(&self, knot_name: &str) -> bool {
        self.volume(knot_name).is_some()
    }
}
/// The Gauss code of an oriented knot diagram.
#[derive(Debug, Clone)]
pub struct GaussCodeData {
    pub entries: Vec<GaussEntry>,
}
impl GaussCodeData {
    /// Construct a Gauss code from entries.
    pub fn new(entries: Vec<GaussEntry>) -> Self {
        Self { entries }
    }
    /// Number of crossings (each crossing appears exactly twice).
    pub fn num_crossings(&self) -> usize {
        self.entries.len() / 2
    }
}
/// A signed entry in a Gauss code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GaussEntry {
    /// Label of the crossing being traversed.
    pub crossing: usize,
    /// Whether this traversal is the over-strand.
    pub is_over: bool,
    /// Sign of the crossing.
    pub sign: CrossingSignVal,
}
/// A link diagram with multiple components.
#[derive(Debug, Clone)]
pub struct LinkDiagram {
    /// Number of components.
    pub num_components: u32,
    /// Crossings as (is_positive: bool, over_strand: usize, under_strand: usize).
    pub crossings: Vec<(bool, usize, usize)>,
}
impl LinkDiagram {
    pub fn new(num_components: u32, crossings: Vec<(bool, usize, usize)>) -> Self {
        LinkDiagram {
            num_components,
            crossings,
        }
    }
    /// Writhe = (number of positive crossings) - (number of negative crossings).
    pub fn writhe(&self) -> i32 {
        self.crossings
            .iter()
            .map(|(pos, _, _)| if *pos { 1i32 } else { -1 })
            .sum()
    }
    /// A link is split if it has components with no crossings between them (simplified).
    pub fn is_split(&self) -> bool {
        self.num_components > 1 && self.crossings.is_empty()
    }
}
/// A generator of the Artin braid group B_n.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BraidGen {
    /// Strand index i (1-indexed): generator σ_i.
    pub index: usize,
    /// If true, this is σ_i; if false, this is σ_i⁻¹.
    pub positive: bool,
}
impl BraidGen {
    pub fn sigma(i: usize) -> Self {
        Self {
            index: i,
            positive: true,
        }
    }
    pub fn sigma_inv(i: usize) -> Self {
        Self {
            index: i,
            positive: false,
        }
    }
}
/// A virtual knot (extending classical knots with virtual crossings).
#[derive(Debug, Clone)]
pub struct VirtualKnot {
    pub crossing_number: u32,
    pub is_classical: bool,
}
impl VirtualKnot {
    pub fn new(crossing_number: u32, is_classical: bool) -> Self {
        VirtualKnot {
            crossing_number,
            is_classical,
        }
    }
    /// The arrow polynomial generalizes the Jones polynomial to virtual knots.
    pub fn arrow_polynomial(&self) -> String {
        if self.is_classical {
            format!(
                "V_J(K) (Jones polynomial, classical knot with {} crossings)",
                self.crossing_number
            )
        } else {
            format!(
                "Arrow polynomial of virtual knot with {} crossings",
                self.crossing_number
            )
        }
    }
    /// Khovanov homology (described symbolically).
    pub fn khovanov_homology(&self) -> String {
        format!(
            "Kh^{{i,j}}(K) for virtual knot with {} crossings [classical={}]",
            self.crossing_number, self.is_classical
        )
    }
}
/// A braid word on n strands represented as a sequence of generator indices.
#[derive(Debug, Clone)]
pub struct BraidWordSpec {
    pub n: usize,
    pub letters: Vec<i32>,
}
impl BraidWordSpec {
    pub fn new(n: usize, letters: Vec<i32>) -> Self {
        BraidWordSpec { n, letters }
    }
    /// The closure of the braid as a knot/link (described by its braid).
    pub fn closure_knot(&self) -> String {
        format!("Closure of braid on {} strands: {:?}", self.n, self.letters)
    }
    /// A braid word is positive if all generators have positive exponents.
    pub fn is_positive(&self) -> bool {
        self.letters.iter().all(|&x| x > 0)
    }
    /// Apply a Markov move: stabilization (append ±σ_n to a braid on n strands).
    pub fn markov_move(&self) -> Self {
        let mut new_letters = self.letters.clone();
        new_letters.push(self.n as i32);
        BraidWordSpec {
            n: self.n + 1,
            letters: new_letters,
        }
    }
}
