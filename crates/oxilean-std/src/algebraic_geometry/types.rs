//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::functions::*;
use std::collections::HashMap;

/// Divisor on an algebraic variety.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Divisor {
    pub components: Vec<(String, i32)>,
    pub is_effective: bool,
    pub is_prime: bool,
}
#[allow(dead_code)]
impl Divisor {
    /// Zero divisor.
    pub fn zero() -> Self {
        Self {
            components: Vec::new(),
            is_effective: true,
            is_prime: false,
        }
    }
    /// Prime divisor.
    pub fn prime(name: &str) -> Self {
        Self {
            components: vec![(name.to_string(), 1)],
            is_effective: true,
            is_prime: true,
        }
    }
    /// Total degree.
    pub fn degree(&self) -> i32 {
        self.components.iter().map(|(_, c)| c).sum()
    }
    /// Linear equivalence description.
    pub fn linear_equiv_description(&self) -> String {
        format!("Div class of degree {}", self.degree())
    }
}
/// Chern class data.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ChernClass {
    pub vector_bundle: String,
    pub rank: usize,
    pub classes: Vec<String>,
}
#[allow(dead_code)]
impl ChernClass {
    /// Chern classes of a rank r bundle.
    pub fn new(bundle: &str, rank: usize) -> Self {
        let classes = (0..=rank).map(|i| format!("c_{}({})", i, bundle)).collect();
        Self {
            vector_bundle: bundle.to_string(),
            rank,
            classes,
        }
    }
    /// Total Chern class.
    pub fn total_chern_class(&self) -> String {
        self.classes.join(" + ")
    }
    /// Grothendieck-Riemann-Roch theorem applies.
    pub fn grr_applies(&self) -> bool {
        true
    }
}
/// An affine variety V(I) ⊆ A^n defined by polynomial equations.
///
/// Represented by a list of polynomial equations as strings (for display/testing),
/// together with a set of sample points for membership testing.
#[derive(Debug, Clone)]
pub struct AffineVariety {
    /// Dimension of the ambient affine space.
    pub ambient_dim: usize,
    /// Defining polynomial equations (as strings for display).
    pub equations: Vec<String>,
}
impl AffineVariety {
    /// Create a new affine variety in A^n defined by given polynomial equations.
    pub fn new(ambient_dim: usize, equations: Vec<String>) -> Self {
        Self {
            ambient_dim,
            equations,
        }
    }
    /// The empty variety in A^n (defined by 1 = 0).
    pub fn empty(n: usize) -> Self {
        Self::new(n, vec!["1".to_string()])
    }
    /// The whole affine space A^n (no equations).
    pub fn affine_space(n: usize) -> Self {
        Self::new(n, vec![])
    }
    /// Number of defining equations.
    pub fn num_equations(&self) -> usize {
        self.equations.len()
    }
    /// Codimension estimate: min(n, number of equations).
    pub fn codimension_estimate(&self) -> usize {
        self.equations.len().min(self.ambient_dim)
    }
    /// Dimension estimate (ambient dim minus codimension estimate).
    pub fn dimension_estimate(&self) -> usize {
        self.ambient_dim.saturating_sub(self.codimension_estimate())
    }
    /// Test if the variety is the empty variety (has equation "1").
    pub fn is_empty_variety(&self) -> bool {
        self.equations.iter().any(|eq| eq.trim() == "1")
    }
    /// Test if the variety is the whole affine space (no equations).
    pub fn is_full_space(&self) -> bool {
        self.equations.is_empty()
    }
}
/// A point in projective space P^n represented by homogeneous coordinates.
///
/// The coordinates are stored as a vector of `i64`; the actual point is the
/// equivalence class \[x₀ : x₁ : … : xₙ\] where not all xᵢ are zero.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectivePoint {
    /// Homogeneous coordinates \[x₀ : x₁ : … : xₙ\].
    pub coords: Vec<i64>,
}
impl ProjectivePoint {
    /// Create a new projective point from homogeneous coordinates.
    /// Returns `None` if all coordinates are zero.
    pub fn new(coords: Vec<i64>) -> Option<Self> {
        if coords.iter().all(|&c| c == 0) {
            None
        } else {
            Some(Self { coords })
        }
    }
    /// The projective dimension (length of coords minus one).
    pub fn dim(&self) -> usize {
        self.coords.len().saturating_sub(1)
    }
    /// Normalize by dividing by the GCD of the coordinates (keeping the first nonzero positive).
    pub fn normalize(&self) -> Self {
        let g = self
            .coords
            .iter()
            .fold(0i64, |acc, &x| gcd(acc.abs(), x.abs()));
        if g == 0 {
            return self.clone();
        }
        let sign = self
            .coords
            .iter()
            .find(|&&c| c != 0)
            .map(|&c| if c < 0 { -1i64 } else { 1i64 })
            .unwrap_or(1);
        Self {
            coords: self.coords.iter().map(|&c| sign * c / g).collect(),
        }
    }
    /// Check if two projective points represent the same element of P^n.
    pub fn equiv(&self, other: &ProjectivePoint) -> bool {
        if self.coords.len() != other.coords.len() {
            return false;
        }
        let n = self.coords.len();
        for i in 0..n {
            for j in 0..n {
                if self.coords[i] * other.coords[j] != self.coords[j] * other.coords[i] {
                    return false;
                }
            }
        }
        true
    }
}
/// A divisor class on a curve of genus g, represented by its degree.
///
/// The Riemann-Roch theorem operates on divisor classes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DivisorClass {
    /// The degree of the divisor class.
    pub degree: i64,
    /// The genus of the underlying curve.
    pub genus: i64,
    /// A label for identification.
    pub label: String,
}
impl DivisorClass {
    /// Create a new divisor class.
    pub fn new(degree: i64, genus: i64, label: impl Into<String>) -> Self {
        Self {
            degree,
            genus,
            label: label.into(),
        }
    }
    /// The zero divisor class (degree 0).
    pub fn zero(genus: i64) -> Self {
        Self::new(0, genus, "O")
    }
    /// The canonical divisor class has degree 2g - 2.
    pub fn canonical(genus: i64) -> Self {
        Self::new(2 * genus - 2, genus, "K")
    }
    /// Add two divisor classes (sum of degrees).
    pub fn add(&self, other: &DivisorClass) -> Self {
        assert_eq!(self.genus, other.genus, "genus must match");
        Self::new(
            self.degree + other.degree,
            self.genus,
            format!("{}+{}", self.label, other.label),
        )
    }
    /// Negate a divisor class.
    pub fn neg(&self) -> Self {
        Self::new(-self.degree, self.genus, format!("-{}", self.label))
    }
    /// Subtract two divisor classes.
    pub fn sub(&self, other: &DivisorClass) -> Self {
        self.add(&other.neg())
    }
    /// Check if this divisor class is effective (degree ≥ 0).
    pub fn is_effective(&self) -> bool {
        self.degree >= 0
    }
    /// Check if this is the canonical class.
    pub fn is_canonical(&self) -> bool {
        self.degree == 2 * self.genus - 2
    }
}
/// Projective variety data.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ProjectiveVariety {
    pub name: String,
    pub ambient_dim: usize,
    pub degree: usize,
    pub is_smooth: bool,
    pub is_irreducible: bool,
}
#[allow(dead_code)]
impl ProjectiveVariety {
    /// Projective space P^n.
    pub fn projective_space(n: usize) -> Self {
        Self {
            name: format!("P^{}", n),
            ambient_dim: n,
            degree: 1,
            is_smooth: true,
            is_irreducible: true,
        }
    }
    /// Smooth hypersurface of degree d in P^n.
    pub fn hypersurface(n: usize, d: usize) -> Self {
        Self {
            name: format!("V_{}(P^{})", d, n),
            ambient_dim: n,
            degree: d,
            is_smooth: true,
            is_irreducible: true,
        }
    }
    /// Dimension (ambient - 1 for hypersurface, etc.).
    pub fn expected_dim(&self) -> usize {
        if self.ambient_dim > 0 {
            self.ambient_dim - 1
        } else {
            0
        }
    }
    /// Bezout's theorem: intersection number bound.
    pub fn bezout_bound(&self, other_degree: usize) -> usize {
        self.degree * other_degree
    }
}
/// An affine scheme represented by the name of its coordinate ring.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AffineScheme {
    /// Name of the coordinate ring (e.g., "k\[x, y\]" for affine n-space).
    pub ring: String,
}
impl AffineScheme {
    /// Create a new affine scheme from a ring name.
    pub fn new(ring: impl Into<String>) -> Self {
        Self { ring: ring.into() }
    }
    /// The spectrum of the integers — Spec ℤ.
    pub fn spec_z() -> Self {
        Self::new("Int")
    }
    /// The spectrum of a polynomial ring k\[x₁, …, xₙ\] — affine n-space.
    pub fn affine_n_space(base_ring: &str, n: usize) -> Self {
        let vars: Vec<String> = (1..=n).map(|i| format!("x{}", i)).collect();
        Self::new(format!("{}[{}]", base_ring, vars.join(", ")))
    }
    /// Dimension of the affine scheme (Krull dimension of the ring).
    pub fn krull_dim_estimate(&self) -> Option<usize> {
        if self.ring.contains('[') {
            let inner = self.ring.split('[').nth(1)?;
            let vars = inner.trim_end_matches(']');
            Some(vars.split(',').count())
        } else if self.ring == "Int" || self.ring == "Z" {
            Some(1)
        } else {
            None
        }
    }
}
/// A sheaf on a topological space, represented by sections over open sets.
///
/// `T` is the type of sections (typically a ring or module).
#[derive(Debug, Clone)]
pub struct Sheaf<T: Clone> {
    /// Sections over open sets, keyed by a string label for the open set.
    sections: std::collections::HashMap<String, T>,
    /// Restriction maps: (source_open, target_open) → index into restriction functions.
    restriction_labels: Vec<(String, String)>,
}
impl<T: Clone> Sheaf<T> {
    /// Create a new empty sheaf.
    pub fn new() -> Self {
        Self {
            sections: std::collections::HashMap::new(),
            restriction_labels: Vec::new(),
        }
    }
    /// Add a section over an open set.
    pub fn add_section(&mut self, open_set: impl Into<String>, section: T) {
        self.sections.insert(open_set.into(), section);
    }
    /// Get the section over a given open set.
    pub fn section(&self, open_set: &str) -> Option<&T> {
        self.sections.get(open_set)
    }
    /// Record a restriction map (source open ⊇ target open).
    pub fn add_restriction(&mut self, source: impl Into<String>, target: impl Into<String>) {
        self.restriction_labels.push((source.into(), target.into()));
    }
    /// Number of open sets with sections.
    pub fn num_sections(&self) -> usize {
        self.sections.len()
    }
}
/// Projective space P^n.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectiveSpace {
    /// The dimension n of the projective space P^n.
    pub dim: usize,
}
impl ProjectiveSpace {
    /// Create n-dimensional projective space.
    pub fn new(dim: usize) -> Self {
        Self { dim }
    }
    /// The projective line P^1.
    pub fn projective_line() -> Self {
        Self::new(1)
    }
    /// The projective plane P^2.
    pub fn projective_plane() -> Self {
        Self::new(2)
    }
    /// Number of homogeneous coordinates (dim + 1).
    pub fn num_coordinates(&self) -> usize {
        self.dim + 1
    }
    /// Euler characteristic of P^n: χ(P^n) = n + 1.
    pub fn euler_characteristic(&self) -> i64 {
        (self.dim as i64) + 1
    }
    /// The Betti numbers of P^n: b_{2k} = 1 for 0 ≤ k ≤ n, all others 0.
    pub fn betti_numbers(&self) -> Vec<u64> {
        let mut betti = vec![0u64; 2 * self.dim + 1];
        for k in 0..=self.dim {
            betti[2 * k] = 1;
        }
        betti
    }
}
/// A morphism of schemes, identified by source and target names.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Morphism {
    /// The source scheme of the morphism.
    pub source: String,
    /// The target scheme of the morphism.
    pub target: String,
}
impl Morphism {
    /// Create a new morphism.
    pub fn new(source: impl Into<String>, target: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            target: target.into(),
        }
    }
    /// The identity morphism on a scheme.
    pub fn identity(scheme: impl Into<String>) -> Self {
        let s = scheme.into();
        Self {
            source: s.clone(),
            target: s,
        }
    }
    /// Check if this is the identity morphism.
    pub fn is_identity(&self) -> bool {
        self.source == self.target
    }
    /// Compose two morphisms: `self` followed by `other` (other ∘ self).
    /// Returns `None` if the target of `self` does not match the source of `other`.
    pub fn compose(&self, other: &Morphism) -> Option<Morphism> {
        if self.target == other.source {
            Some(Morphism::new(self.source.clone(), other.target.clone()))
        } else {
            None
        }
    }
}
/// A point on an elliptic curve over a prime field F_p in short Weierstrass form
/// y² = x³ + ax + b.
///
/// The point can be either an affine point (x, y) or the point at infinity O.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EllipticCurvePoint {
    /// The point at infinity O (identity element of the group).
    Infinity,
    /// An affine point (x, y) on the curve.
    Affine { x: i64, y: i64 },
}
/// Moduli problem data.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ModuliProblem {
    pub objects: String,
    pub equivalence: String,
    pub coarse_moduli_space: Option<String>,
    pub has_fine_moduli: bool,
}
#[allow(dead_code)]
impl ModuliProblem {
    /// Moduli of elliptic curves.
    pub fn elliptic_curves() -> Self {
        Self {
            objects: "elliptic curves over k".to_string(),
            equivalence: "isomorphism".to_string(),
            coarse_moduli_space: Some("A^1_j".to_string()),
            has_fine_moduli: false,
        }
    }
    /// Moduli of vector bundles.
    pub fn vector_bundles(n: usize, degree: i32) -> Self {
        Self {
            objects: format!("rank-{} bundles of degree {}", n, degree),
            equivalence: "S-equivalence".to_string(),
            coarse_moduli_space: Some(format!("M({},{})", n, degree)),
            has_fine_moduli: false,
        }
    }
    /// Does a fine moduli space exist?
    pub fn fine_moduli_description(&self) -> String {
        if self.has_fine_moduli {
            format!("Fine moduli space exists for {}", self.objects)
        } else {
            format!("Only coarse moduli exists for {}", self.objects)
        }
    }
}
/// An elliptic curve y² = x³ + ax + b over F_p (for prime p).
#[derive(Debug, Clone)]
pub struct EllipticCurveF {
    /// Coefficient a in y² = x³ + ax + b.
    pub a: i64,
    /// Coefficient b in y² = x³ + ax + b.
    pub b: i64,
    /// Prime modulus p (the field F_p).
    pub p: i64,
}
impl EllipticCurveF {
    /// Create a new elliptic curve y² = x³ + ax + b over F_p.
    /// Does not check the discriminant condition.
    pub fn new(a: i64, b: i64, p: i64) -> Self {
        Self { a, b, p }
    }
    /// Check if the curve is non-singular (discriminant Δ = -16(4a³ + 27b²) ≠ 0 mod p).
    pub fn is_nonsingular(&self) -> bool {
        let disc = mod_reduce(
            4 * pow_mod(self.a, 3, self.p) + 27 * pow_mod(self.b, 2, self.p),
            self.p,
        );
        disc != 0
    }
    /// Check if a given affine point lies on the curve (y² ≡ x³ + ax + b mod p).
    pub fn on_curve(&self, x: i64, y: i64) -> bool {
        let lhs = mod_reduce(y * y, self.p);
        let rhs = mod_reduce(pow_mod(x, 3, self.p) + self.a * x + self.b, self.p);
        lhs == rhs
    }
    /// Enumerate all affine points on the curve over F_p.
    pub fn points(&self) -> Vec<EllipticCurvePoint> {
        let mut pts = vec![EllipticCurvePoint::Infinity];
        for x in 0..self.p {
            for y in 0..self.p {
                if self.on_curve(x, y) {
                    pts.push(EllipticCurvePoint::Affine { x, y });
                }
            }
        }
        pts
    }
    /// The number of points on the curve over F_p (including the point at infinity).
    pub fn point_count(&self) -> usize {
        self.points().len()
    }
    /// Add two points P and Q on the curve (elliptic curve group law over F_p).
    pub fn add(&self, p_pt: &EllipticCurvePoint, q_pt: &EllipticCurvePoint) -> EllipticCurvePoint {
        match (p_pt, q_pt) {
            (EllipticCurvePoint::Infinity, q) => q.clone(),
            (p, EllipticCurvePoint::Infinity) => p.clone(),
            (
                EllipticCurvePoint::Affine { x: x1, y: y1 },
                EllipticCurvePoint::Affine { x: x2, y: y2 },
            ) => {
                if x1 == x2 && mod_reduce(y1 + y2, self.p) == 0 {
                    return EllipticCurvePoint::Infinity;
                }
                let lambda = if x1 == x2 && y1 == y2 {
                    let num = mod_reduce(3 * pow_mod(*x1, 2, self.p) + self.a, self.p);
                    let den = mod_reduce(2 * y1, self.p);
                    mod_mul(num, mod_inv(den, self.p), self.p)
                } else {
                    let num = mod_reduce(y2 - y1, self.p);
                    let den = mod_reduce(x2 - x1, self.p);
                    mod_mul(num, mod_inv(den, self.p), self.p)
                };
                let x3 = mod_reduce(pow_mod(lambda, 2, self.p) - x1 - x2, self.p);
                let y3 = mod_reduce(lambda * (x1 - x3) - y1, self.p);
                EllipticCurvePoint::Affine { x: x3, y: y3 }
            }
        }
    }
    /// Scalar multiplication \[n\]P on the elliptic curve using double-and-add.
    pub fn scalar_mul(&self, n: u64, pt: &EllipticCurvePoint) -> EllipticCurvePoint {
        let mut result = EllipticCurvePoint::Infinity;
        let mut addend = pt.clone();
        let mut k = n;
        while k > 0 {
            if k & 1 == 1 {
                result = self.add(&result, &addend);
            }
            addend = self.add(&addend, &addend);
            k >>= 1;
        }
        result
    }
    /// Compute the order of a point P (the smallest n > 0 such that \[n\]P = O).
    /// Returns `None` if the order exceeds `max_order`.
    pub fn point_order(&self, pt: &EllipticCurvePoint, max_order: u64) -> Option<u64> {
        if matches!(pt, EllipticCurvePoint::Infinity) {
            return Some(1);
        }
        let mut current = pt.clone();
        for n in 1..=max_order {
            current = self.add(&current, pt);
            if matches!(current, EllipticCurvePoint::Infinity) {
                return Some(n + 1);
            }
        }
        None
    }
}
/// Affine scheme data.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AffineSchemeData {
    pub ring: String,
    pub is_integral: bool,
    pub is_noetherian: bool,
    pub dimension: Option<usize>,
}
#[allow(dead_code)]
impl AffineSchemeData {
    /// Spec of polynomial ring k\[x1,...,xn\].
    pub fn affine_space(n: usize) -> Self {
        Self {
            ring: format!("k[x1,...,x{}]", n),
            is_integral: true,
            is_noetherian: true,
            dimension: Some(n),
        }
    }
    /// Spec of a field.
    pub fn point() -> Self {
        Self {
            ring: "k".to_string(),
            is_integral: true,
            is_noetherian: true,
            dimension: Some(0),
        }
    }
    /// Is this a variety (integral + finite type over k)?
    pub fn is_variety(&self) -> bool {
        self.is_integral && self.is_noetherian
    }
}
