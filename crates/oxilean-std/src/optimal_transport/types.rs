//! Types for optimal transport.

/// A discrete probability measure: a finite collection of atoms with weights.
///
/// `weights\[i\]` is the mass at location `support\[i\]`.  It is the caller's
/// responsibility to ensure weights are non-negative and sum to one (use
/// `normalize_measure` if needed).
#[derive(Debug, Clone)]
pub struct Measure {
    /// Non-negative weights summing to one.
    pub weights: Vec<f64>,
    /// Atoms; `support\[i\]` is a point in R^d.
    pub support: Vec<Vec<f64>>,
}

impl Measure {
    /// Construct a new discrete measure.
    pub fn new(weights: Vec<f64>, support: Vec<Vec<f64>>) -> Self {
        Self { weights, support }
    }
}

/// Ground metric used to build a cost matrix.
#[derive(Debug, Clone)]
pub enum GroundMetric {
    /// Euclidean distance ||x - y||_2.
    Euclidean,
    /// Squared Euclidean distance ||x - y||_2^2.
    SquaredEuclidean,
    /// L1 (Manhattan) distance ||x - y||_1.
    L1,
    /// User-supplied cost matrix (must be n_source × n_target).
    Custom(Vec<Vec<f64>>),
}

/// Pairwise cost matrix C\[i\]\[j\] = c(source_i, target_j).
#[derive(Debug, Clone)]
pub struct CostMatrix {
    /// Cost entries; `entries\[i\]\[j\]` is the cost of moving mass from source i
    /// to target j.
    pub entries: Vec<Vec<f64>>,
    /// Number of source atoms.
    pub n_source: usize,
    /// Number of target atoms.
    pub n_target: usize,
}

impl CostMatrix {
    /// Construct a new cost matrix.
    pub fn new(entries: Vec<Vec<f64>>, n_source: usize, n_target: usize) -> Self {
        Self {
            entries,
            n_source,
            n_target,
        }
    }
}

/// An optimal (or regularised-optimal) transport coupling γ ∈ R^{n×m}.
///
/// `gamma\[i\]\[j\]` is the amount of mass transported from source i to target j.
#[derive(Debug, Clone)]
pub struct TransportPlan {
    /// Coupling matrix.
    pub gamma: Vec<Vec<f64>>,
    /// Total transport cost <γ, C>.
    pub cost: f64,
}

/// Result of the Sinkhorn–Knopp algorithm.
#[derive(Debug, Clone)]
pub struct SinkhornResult {
    /// The regularised transport plan.
    pub plan: TransportPlan,
    /// Number of iterations performed.
    pub iterations: usize,
    /// Whether the algorithm converged within the given tolerance.
    pub converged: bool,
}

/// Wasserstein-p distance between two measures.
#[derive(Debug, Clone, Copy)]
pub struct WassersteinDistance {
    /// Order p.
    pub p: f64,
    /// W_p distance (not raised to the p-th power).
    pub distance: f64,
}

/// Result of a Wasserstein barycenter computation.
#[derive(Debug, Clone)]
pub struct BarycenterResult {
    /// Weights of the barycenter atoms.
    pub weights: Vec<f64>,
    /// Support of the barycenter measure.
    pub support: Vec<Vec<f64>>,
    /// Number of iterations performed.
    pub iterations: usize,
}

/// Sliced Wasserstein distance (Monte Carlo approximation).
#[derive(Debug, Clone, Copy)]
pub struct SlicedWasserstein {
    /// Approximated sliced Wasserstein distance.
    pub distance: f64,
    /// Number of random projections used.
    pub n_projections: usize,
}

/// Dual solution (Kantorovich potentials) for the OT problem.
#[derive(Debug, Clone)]
pub struct OTDualSolution {
    /// Source potential f (length n_source).
    pub f: Vec<f64>,
    /// Target potential g (length n_target).
    pub g: Vec<f64>,
    /// Dual objective value: <f, a> + <g, b>.
    pub dual_objective: f64,
}
