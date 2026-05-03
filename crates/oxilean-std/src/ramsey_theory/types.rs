//! Types for Ramsey theory.

/// A Ramsey number R(r, s): the minimum n such that any 2-coloring of
/// edges of K_n contains a red K_r or blue K_s.
///
/// `value` is `None` when the exact value is unknown.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RamseyNumber {
    /// The clique size parameter r.
    pub r: usize,
    /// The independent set size parameter s.
    pub s: usize,
    /// The exact Ramsey number, if known.
    pub value: Option<usize>,
}

/// An edge-coloring of a complete graph on `num_vertices` vertices.
///
/// Each entry in `edges` is `(u, v, color)` where `0 <= color < num_colors`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Coloring {
    /// Number of vertices in the underlying complete graph.
    pub num_vertices: usize,
    /// Number of colors used (colors are 0-indexed up to num_colors-1).
    pub num_colors: usize,
    /// Edge list: each `(u, v, color)` with `u < v`.
    pub edges: Vec<(usize, usize, usize)>,
}

/// A clique: a complete subgraph induced by a subset of vertices.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Clique {
    /// Vertices comprising the clique.
    pub vertices: Vec<usize>,
    /// Size of the clique (equals `vertices.len()`).
    pub size: usize,
}

/// An independent set: a set of vertices with no two adjacent in the
/// underlying complete graph under a given color.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IndependentSet {
    /// Vertices in the independent set.
    pub vertices: Vec<usize>,
}

/// A witness that a coloring avoids both a monochromatic r-clique in
/// `clique_free_color` and a monochromatic s-independent-set in `indep_free_color`.
#[derive(Clone, Debug)]
pub struct RamseyWitness {
    /// The underlying edge-coloring.
    pub coloring: Coloring,
    /// Color that is free of r-cliques.
    pub clique_free_color: usize,
    /// Color that is free of s-independent sets.
    pub indep_free_color: usize,
}

/// Configuration for a Van der Waerden problem: W(k; r) is the minimum n
/// such that every r-coloring of {1, …, n} contains a monochromatic AP of
/// length k.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VanDerWaerdenConfig {
    /// Universe size n.
    pub n: usize,
    /// Required arithmetic progression length.
    pub k: usize,
    /// Number of colors.
    pub num_colors: usize,
}

/// Configuration for a Hales–Jewett problem: HJ(t, n).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HalesJewettConfig {
    /// Alphabet size t.
    pub t: usize,
    /// Word length n.
    pub n: usize,
}

/// A Schur number S(k): the largest n such that {1, …, n} can be k-colored
/// without a monochromatic solution to x + y = z.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SchurNumber {
    /// Number of colors k.
    pub k: usize,
    /// The Schur number S(k).
    pub value: usize,
}

/// Points configuration for the Happy Ending (Erdős–Szekeres) problem.
#[derive(Clone, Debug)]
pub struct HappyEndingConfig {
    /// Number of points required in the convex position problem.
    pub n: usize,
    /// The point set in the plane.
    pub points: Vec<(f64, f64)>,
}

/// An arithmetic progression: start, step, length.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArithmeticProgression {
    /// First element of the progression (1-indexed).
    pub start: usize,
    /// Common difference.
    pub step: usize,
    /// Number of terms.
    pub length: usize,
}
