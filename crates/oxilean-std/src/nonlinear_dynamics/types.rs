//! Types for nonlinear dynamical systems.

/// A nonlinear dynamical system of a given `dimension`.
#[derive(Clone, Debug)]
pub struct DynamicalSystem {
    /// State-space dimension.
    pub dimension: usize,
    /// The concrete type / parameters of the system.
    pub system_type: SystemType,
}

/// Specifies whether a system evolves in continuous or discrete time, or
/// as a Hamiltonian system.
#[derive(Clone, Debug)]
pub enum SystemType {
    /// Continuous-time ODE ẋ = f(x), represented by truncated polynomial
    /// coefficients (one row per dimension).
    Continuous {
        /// Polynomial ODE coefficients (one vector per state variable).
        ode_coefficients: Vec<Vec<f64>>,
    },
    /// Discrete-time map x_{n+1} = F(x_n).
    Discrete {
        /// The specific map type and parameters.
        map_type: MapType,
    },
    /// Hamiltonian system ẋ = ∂H/∂p, ṗ = -∂H/∂q.
    HamiltonianSystem {
        /// Coefficients of the Hamiltonian polynomial.
        hamiltonian_coeffs: Vec<f64>,
    },
}

/// Specific discrete-time maps with named parameter sets.
#[derive(Clone, Debug, PartialEq)]
pub enum MapType {
    /// The logistic map x_{n+1} = r·x_n·(1 − x_n).
    Logistic {
        /// Growth parameter r ∈ \[0, 4\].
        r: f64,
    },
    /// The Hénon map (x, y) → (1 − a·x² + y, b·x).
    Henon {
        /// Quadratic coefficient a.
        a: f64,
        /// Dissipation coefficient b.
        b: f64,
    },
    /// Chirikov standard map (p, q) → (p + k·sin(q), q + p + k·sin(q)) mod 2π.
    Standard {
        /// Nonlinearity parameter k.
        k: f64,
    },
    /// The tent map x_{n+1} = μ·min(x, 1-x).
    Tent {
        /// Slope parameter μ ∈ \[0, 2\].
        mu: f64,
    },
    /// The Bernoulli shift map x_{n+1} = m·x mod 1.
    Bernoulli {
        /// Integer multiplier m >= 2.
        m: usize,
    },
}

/// A time-parametrized trajectory in phase space.
#[derive(Clone, Debug)]
pub struct Trajectory {
    /// State vectors at each time step; `points\[i\]` is the state at `times\[i\]`.
    pub points: Vec<Vec<f64>>,
    /// Time values corresponding to each state point.
    pub times: Vec<f64>,
}

/// A fixed point of a dynamical system, together with its linear stability
/// classification.
#[derive(Clone, Debug)]
pub struct FixedPoint {
    /// Location in phase space.
    pub location: Vec<f64>,
    /// Stability type determined by linearization.
    pub stability: Stability,
    /// Eigenvalues of the Jacobian at the fixed point.
    pub eigenvalues: Vec<f64>,
}

/// Stability classification for a fixed point.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Stability {
    /// All eigenvalues have negative real part (stable node / focus).
    StableNode,
    /// All eigenvalues have positive real part (unstable node / focus).
    UnstableNode,
    /// Mixed-sign eigenvalues (hyperbolic saddle point).
    SaddlePoint,
    /// Purely imaginary eigenvalues (center, undamped oscillations).
    Center,
    /// Complex eigenvalues, negative real part (stable spiral focus).
    StableFocus,
    /// Complex eigenvalues, positive real part (unstable spiral focus).
    UnstableFocus,
    /// Cannot be determined from available data.
    Unknown,
}

/// The Lyapunov exponent spectrum of a dynamical system.
#[derive(Clone, Debug)]
pub struct LyapunovExponents {
    /// Lyapunov exponents in non-increasing order.
    pub exponents: Vec<f64>,
    /// Dimension of the system state space.
    pub system_dimension: usize,
}

/// Asymptotic attractor type of a dynamical system.
#[derive(Clone, Debug)]
pub enum AttractorType {
    /// Stable equilibrium (fixed point attractor).
    FixedPoint,
    /// Periodic orbit (limit cycle) with the given period.
    LimitCycle {
        /// Temporal period.
        period: f64,
    },
    /// Quasi-periodic motion on a torus with the given fundamental frequencies.
    Torus {
        /// Incommensurate frequencies.
        frequencies: Vec<f64>,
    },
    /// Strange (chaotic) attractor with fractal dimension.
    Strange {
        /// Fractal (Hausdorff/Lyapunov) dimension.
        dimension: f64,
    },
    /// Repelling set (source in phase space).
    Repeller,
}

/// A bifurcation point: the parameter value at which the system's qualitative
/// behavior changes.
#[derive(Clone, Debug)]
pub struct BifurcationPoint {
    /// The value of the bifurcation parameter.
    pub parameter_value: f64,
    /// The type of bifurcation occurring.
    pub bifurcation_type: BifurcationType,
}

/// Standard local bifurcation types.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BifurcationType {
    /// Saddle-node (fold) bifurcation: two fixed points collide and annihilate.
    SaddleNode,
    /// Transcritical bifurcation: two fixed points exchange stability.
    Transcritical,
    /// Pitchfork bifurcation: one fixed point splits into three.
    Pitchfork,
    /// Hopf bifurcation: fixed point loses stability to a limit cycle.
    HopfBifurcation,
    /// Period-doubling (flip) bifurcation: period of a limit cycle doubles.
    PeriodDoubling,
}

/// Result of intersecting a trajectory with a Poincaré section hyperplane.
#[derive(Clone, Debug)]
pub struct PoincareSectionResult {
    /// Points on the Poincaré section (one per crossing event).
    pub points: Vec<Vec<f64>>,
    /// Time intervals between consecutive crossings (return times).
    pub recurrence_times: Vec<f64>,
}

/// Fractal dimension of an attractor computed by a specific method.
#[derive(Clone, Debug)]
pub struct FractalDimension {
    /// Computed fractal dimension value.
    pub value: f64,
    /// The numerical method used.
    pub method: DimensionMethod,
}

/// Method used to estimate the fractal dimension.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DimensionMethod {
    /// Box-counting (Minkowski–Bouligand) dimension.
    BoxCounting,
    /// Correlation dimension (Grassberger–Procaccia).
    CorrelationDimension,
    /// Lyapunov (Kaplan–Yorke) dimension.
    LyapunovDimension,
}
