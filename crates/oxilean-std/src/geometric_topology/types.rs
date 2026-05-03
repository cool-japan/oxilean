//! Types for geometric topology: 3-manifolds, knot surgery, JSJ decomposition.

/// A 3-manifold with basic topological invariants.
#[derive(Debug, Clone, PartialEq)]
pub struct Manifold3 {
    /// Human-readable name, e.g. "S^3", "T^3", "lens space L(5,2)".
    pub name: String,
    /// Genus of the manifold (if applicable, e.g. for surface bundles).
    pub genus: Option<u32>,
    /// Whether the manifold is orientable.
    pub is_orientable: bool,
    /// Whether the manifold has no boundary.
    pub is_closed: bool,
    /// Number of boundary components (0 for closed manifolds).
    pub boundary_components: u32,
}

impl Manifold3 {
    /// Construct a new 3-manifold.
    pub fn new(
        name: impl Into<String>,
        genus: Option<u32>,
        is_orientable: bool,
        is_closed: bool,
        boundary_components: u32,
    ) -> Self {
        Self {
            name: name.into(),
            genus,
            is_orientable,
            is_closed,
            boundary_components,
        }
    }

    /// The 3-sphere S^3.
    pub fn s3() -> Self {
        Self::new("S^3", Some(0), true, true, 0)
    }

    /// The 3-torus T^3.
    pub fn t3() -> Self {
        Self::new("T^3", Some(3), true, true, 0)
    }

    /// Real projective space RP^3.
    pub fn rp3() -> Self {
        Self::new("RP^3", None, false, true, 0)
    }

    /// Solid torus (D^2 x S^1).
    pub fn solid_torus() -> Self {
        Self::new("D^2 x S^1", Some(1), true, false, 1)
    }
}

/// Heegaard splitting of a 3-manifold.
///
/// Every closed, orientable 3-manifold M decomposes as M = H1 ∪_Σ H2 where
/// H1 and H2 are handlebodies glued along a surface Σ_g of genus g.
#[derive(Debug, Clone, PartialEq)]
pub struct HeegaardSplitting {
    /// Genus of the splitting surface.
    pub genus: usize,
    /// Name of the underlying manifold.
    pub manifold: String,
}

impl HeegaardSplitting {
    /// Construct a Heegaard splitting.
    pub fn new(genus: usize, manifold: impl Into<String>) -> Self {
        Self {
            genus,
            manifold: manifold.into(),
        }
    }
}

/// Dehn surgery along a knot with rational slope p/q.
///
/// Given a knot K in S^3, (p/q)-surgery removes a tubular neighbourhood of K
/// and reglues a solid torus so that the meridian maps to p\[μ\] + q\[λ\].
#[derive(Debug, Clone, PartialEq)]
pub struct DehnSurgery {
    /// Name of the ambient manifold (usually "S^3").
    pub manifold: String,
    /// Name of the knot, e.g. "trefoil", "figure-eight".
    pub knot: String,
    /// Surgery slope (p, q) representing the fraction p/q.
    pub slope: (i64, i64),
}

impl DehnSurgery {
    /// Construct a Dehn surgery specification.
    pub fn new(manifold: impl Into<String>, knot: impl Into<String>, slope: (i64, i64)) -> Self {
        Self {
            manifold: manifold.into(),
            knot: knot.into(),
            slope,
        }
    }
}

/// JSJ (Jaco–Shalen–Johannsen) decomposition of a 3-manifold.
///
/// Every compact, orientable, irreducible 3-manifold with incompressible
/// boundary admits a canonical decomposition along embedded tori into
/// Seifert-fibered and hyperbolic pieces.
#[derive(Debug, Clone)]
pub struct JSJDecomposition {
    /// List of geometric pieces.
    pub pieces: Vec<ManifoldPiece>,
    /// Gluing data: (index_i, index_j, gluing_matrix).
    pub gluing: Vec<(usize, usize, Matrix2x2)>,
}

impl JSJDecomposition {
    /// Construct a JSJ decomposition.
    pub fn new(pieces: Vec<ManifoldPiece>, gluing: Vec<(usize, usize, Matrix2x2)>) -> Self {
        Self { pieces, gluing }
    }

    /// Total number of geometric pieces.
    pub fn num_pieces(&self) -> usize {
        self.pieces.len()
    }
}

/// A piece in the JSJ decomposition.
#[derive(Debug, Clone)]
pub enum ManifoldPiece {
    /// A Seifert-fibered space.
    Seifert(SeifertFibered),
    /// A hyperbolic piece.
    Hyperbolic(HyperbolicPiece),
    /// An I-bundle over a surface.
    IBundle,
}

/// A Seifert-fibered space.
///
/// Described by a base orbifold and a list of exceptional fibers (a_i, b_i)
/// where the i-th exceptional fiber has Seifert invariant (a_i, b_i) with gcd(a_i,b_i)=1.
#[derive(Debug, Clone, PartialEq)]
pub struct SeifertFibered {
    /// Description of the base orbifold, e.g. "S^2", "D^2", "T^2".
    pub base_orbifold: String,
    /// List of exceptional fiber invariants (a_i, b_i).
    pub exceptional_fibers: Vec<(i64, i64)>,
}

impl SeifertFibered {
    /// Construct a Seifert-fibered space.
    pub fn new(base_orbifold: impl Into<String>, exceptional_fibers: Vec<(i64, i64)>) -> Self {
        Self {
            base_orbifold: base_orbifold.into(),
            exceptional_fibers,
        }
    }

    /// Seifert fibration over S^2 with three exceptional fibers — lens space model.
    pub fn lens_space_seifert(p: i64, q: i64) -> Self {
        Self::new("S^2", vec![(p, q), (1, 0)])
    }
}

/// A hyperbolic 3-manifold piece.
#[derive(Debug, Clone, PartialEq)]
pub struct HyperbolicPiece {
    /// Hyperbolic volume (always > 0 for genuine hyperbolic pieces).
    pub volume: f64,
    /// Number of cusps (boundary tori).
    pub cusps: u32,
}

impl HyperbolicPiece {
    /// Construct a hyperbolic piece.
    pub fn new(volume: f64, cusps: u32) -> Self {
        Self { volume, cusps }
    }

    /// The figure-eight knot complement — smallest known hyperbolic 3-manifold.
    pub fn figure_eight_complement() -> Self {
        Self::new(2.029_883_212_819_307_5, 1)
    }
}

/// An integer 2×2 matrix, used as a gluing matrix in SL(2,Z).
///
/// Represents the change-of-basis matrix for boundary tori in JSJ gluing.
#[derive(Debug, Clone, PartialEq)]
pub struct Matrix2x2 {
    /// Top-left entry.
    pub a: i64,
    /// Top-right entry.
    pub b: i64,
    /// Bottom-left entry.
    pub c: i64,
    /// Bottom-right entry.
    pub d: i64,
}

impl Matrix2x2 {
    /// Construct a 2×2 integer matrix.
    pub fn new(a: i64, b: i64, c: i64, d: i64) -> Self {
        Self { a, b, c, d }
    }

    /// Identity matrix.
    pub fn identity() -> Self {
        Self::new(1, 0, 0, 1)
    }

    /// Determinant.
    pub fn det(&self) -> i64 {
        self.a * self.d - self.b * self.c
    }

    /// Standard Dehn surgery gluing matrix for slope (p,q).
    ///
    /// Constructs [\[p, -t\],\[q, s\]] where bezout gives p*s + q*t = gcd(p,q)=1,
    /// so det = p*s - (-t)*q = p*s + q*t = 1.
    pub fn surgery_matrix(p: i64, q: i64) -> Self {
        // bezout_coefficients(p,q) returns (s,t) with p*s + q*t = gcd(p,q)
        let (s, t) = bezout_coefficients(p, q);
        // det = p*s - (-t)*q = p*s + q*t = 1
        Self::new(p, -t, q, s)
    }
}

/// Extended Euclidean algorithm returning (s, t) with a*s + b*t = gcd(a,b).
pub(super) fn bezout_coefficients(a: i64, b: i64) -> (i64, i64) {
    if b == 0 {
        return (1, 0);
    }
    let (s1, t1) = bezout_coefficients(b, a % b);
    (t1, s1 - (a / b) * t1)
}
