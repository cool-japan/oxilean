//! Types for convex analysis.

/// Description of a convex set.
#[derive(Debug, Clone)]
pub enum SetDescription {
    /// Half-space { x : <normal, x> <= offset }.
    HalfSpace { normal: Vec<f64>, offset: f64 },
    /// Polytope defined by linear inequalities Ax <= b.
    /// Each entry is (row of A, corresponding b_i).
    Polytope { inequalities: Vec<(Vec<f64>, f64)> },
    /// Euclidean ball { x : ||x - center|| <= radius }.
    Ball { center: Vec<f64>, radius: f64 },
    /// Convex hull of given vertices.
    Simplex { vertices: Vec<Vec<f64>> },
    /// Intersection of a list of sets.
    Intersection(Vec<Box<SetDescription>>),
}

/// A convex set in R^d.
#[derive(Debug, Clone)]
pub struct ConvexSet {
    /// Ambient dimension.
    pub dimension: usize,
    /// Geometric description.
    pub description: SetDescription,
}

impl ConvexSet {
    /// Construct a new convex set.
    pub fn new(dimension: usize, description: SetDescription) -> Self {
        Self {
            dimension,
            description,
        }
    }
}

/// The kind / parametrisation of a convex function f : R^n -> R.
#[derive(Debug, Clone)]
pub enum FunctionKind {
    /// Quadratic: f(x) = (1/2) x^T Q x + b^T x + c.
    Quadratic {
        q: Vec<Vec<f64>>,
        b: Vec<f64>,
        c: f64,
    },
    /// Linear: f(x) = a^T x + b.
    Linear { a: Vec<f64>, b: f64 },
    /// p-norm: f(x) = ||x||_p.
    Norm { p: f64 },
    /// Indicator of the domain convex set (0 inside, +inf outside).
    Indicator,
    /// Max-affine function: f(x) = max_i (a_i^T x + b_i).
    MaxAffine { pieces: Vec<(Vec<f64>, f64)> },
}

/// A convex function with an explicit domain.
#[derive(Debug, Clone)]
pub struct ConvexFunction {
    /// Domain over which the function is defined (and finite).
    pub domain: ConvexSet,
    /// Functional form.
    pub kind: FunctionKind,
}

impl ConvexFunction {
    /// Construct a new convex function.
    pub fn new(domain: ConvexSet, kind: FunctionKind) -> Self {
        Self { domain, kind }
    }
}

/// Result of a subgradient computation.
#[derive(Debug, Clone)]
pub struct SubgradientResult {
    /// Point at which the subgradient was computed.
    pub point: Vec<f64>,
    /// A subgradient g such that f(y) >= f(x) + <g, y-x> for all y.
    pub subgradient: Vec<f64>,
    /// Function value f(x).
    pub value: f64,
}

/// Result of a projection onto a convex set.
#[derive(Debug, Clone)]
pub struct ProjectionResult {
    /// The projected point.
    pub projected: Vec<f64>,
    /// Euclidean distance from the original point to the projection.
    pub distance: f64,
}

/// A separating hyperplane { x : <normal, x> = offset }.
#[derive(Debug, Clone)]
pub struct SeparatingHyperplane {
    /// Outward normal (unit or unnormalised).
    pub normal: Vec<f64>,
    /// Scalar offset so that the hyperplane is { x : <normal,x> = offset }.
    pub offset: f64,
}

/// Fenchel conjugate f*(y) = sup_x { <y, x> - f(x) }.
#[derive(Debug, Clone)]
pub struct ConjugateFunction {
    /// The original function kind (kept for reference).
    pub original_kind: FunctionKind,
}

/// Proximal operator prox_{t f}(v) = argmin_x { f(x) + (1/2t)||x - v||^2 }.
#[derive(Debug, Clone)]
pub struct ProximalOperator {
    /// The function whose proximal map is represented.
    pub function: ConvexFunction,
    /// Step size t > 0.
    pub step_size: f64,
}

impl ProximalOperator {
    /// Construct a new proximal operator.
    pub fn new(function: ConvexFunction, step_size: f64) -> Self {
        Self {
            function,
            step_size,
        }
    }
}

/// Result of a convex optimisation run.
#[derive(Debug, Clone)]
pub struct OptimizationResult {
    /// Best iterate found.
    pub optimal_point: Vec<f64>,
    /// Objective value at the best iterate.
    pub optimal_value: f64,
    /// Number of iterations performed.
    pub iterations: usize,
    /// Whether the algorithm declared convergence.
    pub converged: bool,
}
