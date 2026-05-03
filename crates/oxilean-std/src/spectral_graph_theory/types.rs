//! Types for spectral graph theory.

/// Adjacency matrix representation of an undirected graph with `n` vertices.
///
/// `data\[i\]\[j\]` is the weight of edge (i, j); 0.0 means no edge.
#[derive(Clone, Debug, PartialEq)]
pub struct AdjMatrix {
    /// Edge weights; `data\[i\]\[j\]` = weight of edge between i and j.
    pub data: Vec<Vec<f64>>,
    /// Number of vertices.
    pub n: usize,
}

impl AdjMatrix {
    /// Construct an n×n zero adjacency matrix.
    pub fn new(n: usize) -> Self {
        Self {
            data: vec![vec![0.0; n]; n],
            n,
        }
    }

    /// Add an undirected edge (i, j) with weight w.
    pub fn add_edge(&mut self, i: usize, j: usize, w: f64) {
        self.data[i][j] = w;
        self.data[j][i] = w;
    }
}

/// Laplacian matrix L = D − A, where D is the degree matrix.
#[derive(Clone, Debug, PartialEq)]
pub struct LaplacianMatrix {
    /// Matrix entries.
    pub data: Vec<Vec<f64>>,
    /// Number of vertices.
    pub n: usize,
}

/// Spectrum of a real symmetric matrix: eigenvalues (sorted ascending) and
/// corresponding eigenvectors.
#[derive(Clone, Debug)]
pub struct GraphSpectrum {
    /// Eigenvalues sorted in non-decreasing order.
    pub eigenvalues: Vec<f64>,
    /// Corresponding eigenvectors; `eigenvectors\[k\]` is the k-th eigenvector.
    pub eigenvectors: Vec<Vec<f64>>,
}

/// Spectral gap between the first (λ₁) and second (λ₂) eigenvalues of the
/// normalised Laplacian (or adjacency matrix depending on context).
#[derive(Clone, Debug)]
pub struct SpectralGap {
    /// First (smallest non-negative) eigenvalue λ₁.
    pub lambda1: f64,
    /// Second eigenvalue λ₂.
    pub lambda2: f64,
    /// gap = λ₂ − λ₁.
    pub gap: f64,
}

/// Certificate that a graph is an expander.
#[derive(Clone, Debug)]
pub struct Expander {
    /// Number of vertices.
    pub n: usize,
    /// Regular degree (each vertex has this many neighbours).
    pub degree: usize,
    /// Spectral gap of the normalised adjacency operator.
    pub spectral_gap: f64,
}

/// Random-walk matrix W = D⁻¹A.
///
/// `data\[i\]\[j\]` is the probability of moving from vertex i to vertex j in one
/// step of the lazy random walk.
#[derive(Clone, Debug)]
pub struct RandomWalkMatrix {
    /// Transition probabilities.
    pub data: Vec<Vec<f64>>,
    /// Number of vertices.
    pub n: usize,
}

/// Mixing time of a random walk: the number of steps required to reach within
/// total-variation distance `epsilon` of the stationary distribution.
#[derive(Clone, Debug)]
pub struct MixingTime {
    /// Target total-variation distance.
    pub epsilon: f64,
    /// Number of steps required.
    pub steps: usize,
}
