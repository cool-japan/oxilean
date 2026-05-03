//! Types for hybrid dynamical systems.

/// Current state of a hybrid automaton: a discrete mode index plus a
/// continuous (real-valued) component.
#[derive(Debug, Clone, PartialEq)]
pub struct HybridState {
    /// Index into `HybridAutomaton::modes`.
    pub mode: usize,
    /// Continuous state vector.
    pub continuous: Vec<f64>,
}

impl HybridState {
    /// Construct a new hybrid state.
    pub fn new(mode: usize, continuous: Vec<f64>) -> Self {
        Self { mode, continuous }
    }
}

/// Guard condition that must hold for a discrete transition to fire.
#[derive(Debug, Clone)]
pub enum GuardCondition {
    /// Transition fires unconditionally.
    Always,
    /// Fires when `coeffs · state <= bound`.
    LinearInequality { coeffs: Vec<f64>, bound: f64 },
    /// Arbitrary (named) nonlinear guard; treated as always-false during
    /// numeric simulation unless specialised.
    NonLinear(String),
}

/// Reset map applied to the continuous state after a discrete transition.
#[derive(Debug, Clone)]
pub enum ResetMap {
    /// Continuous state is unchanged.
    Identity,
    /// New state = `matrix * old_state`.
    Linear { matrix: Vec<Vec<f64>> },
    /// New state is set to a constant vector.
    Constant { values: Vec<f64> },
}

/// A single discrete transition of the hybrid automaton.
#[derive(Debug, Clone)]
pub struct DiscreteTransition {
    pub from_mode: usize,
    pub to_mode: usize,
    pub guard: GuardCondition,
    pub reset: ResetMap,
}

/// Describes the ODE flow type in one mode.
#[derive(Debug, Clone)]
pub enum FlowType {
    /// ẋ = A x + b  (linear / affine with zero offset)
    Linear {
        a_matrix: Vec<Vec<f64>>,
        b_vector: Vec<f64>,
    },
    /// ẋ = A x + B u + c  (fully affine)
    Affine {
        a: Vec<Vec<f64>>,
        b: Vec<Vec<f64>>,
        c: Vec<f64>,
    },
    /// ẋ = 0  (stationary)
    Zero,
}

/// ODE dynamics assigned to a particular mode.
#[derive(Debug, Clone)]
pub struct ContinuousDynamics {
    pub mode: usize,
    pub flow: FlowType,
}

/// Invariant (staying condition) for a mode: the continuous state must
/// satisfy `condition` while the automaton remains in `mode`.
#[derive(Debug, Clone)]
pub struct Invariant {
    pub mode: usize,
    pub condition: GuardCondition,
}

/// The hybrid automaton itself.
#[derive(Debug, Clone)]
pub struct HybridAutomaton {
    /// Human-readable mode names (indexed by mode id).
    pub modes: Vec<String>,
    /// All possible discrete transitions.
    pub transitions: Vec<DiscreteTransition>,
    /// ODE dynamics, one entry per mode (may be fewer – missing modes use Zero).
    pub dynamics: Vec<ContinuousDynamics>,
    pub initial_mode: usize,
    pub initial_state: Vec<f64>,
}

impl HybridAutomaton {
    /// Construct an automaton with given modes and initial conditions.
    pub fn new(
        modes: Vec<String>,
        transitions: Vec<DiscreteTransition>,
        dynamics: Vec<ContinuousDynamics>,
        initial_mode: usize,
        initial_state: Vec<f64>,
    ) -> Self {
        Self {
            modes,
            transitions,
            dynamics,
            initial_mode,
            initial_state,
        }
    }

    /// Number of discrete modes.
    pub fn num_modes(&self) -> usize {
        self.modes.len()
    }
}

/// A recorded execution trace of the hybrid automaton.
#[derive(Debug, Clone)]
pub struct ExecutionTrace {
    /// Sequence of hybrid states visited.
    pub states: Vec<HybridState>,
    /// Simulation time at each state.
    pub times: Vec<f64>,
    /// `transitions\[i\]` = index into the transition list used at step i,
    /// or `None` if step i was a continuous (Euler) step.
    pub transitions: Vec<Option<usize>>,
}

impl ExecutionTrace {
    /// Construct an empty trace.
    pub fn empty() -> Self {
        Self {
            states: Vec::new(),
            times: Vec::new(),
            transitions: Vec::new(),
        }
    }

    /// Total number of discrete jumps recorded.
    pub fn jump_count(&self) -> usize {
        self.transitions.iter().filter(|t| t.is_some()).count()
    }

    /// Total simulated time span.
    pub fn total_time(&self) -> f64 {
        match (self.times.first(), self.times.last()) {
            (Some(&t0), Some(&t1)) => t1 - t0,
            _ => 0.0,
        }
    }
}

/// A safety property to be verified against an execution trace.
#[derive(Debug, Clone)]
pub enum SafetyProperty {
    /// The automaton must never visit any of the listed modes.
    ReachabilityAvoidance { forbidden_modes: Vec<usize> },
    /// Dimension `dim` of the continuous state must stay in `[lower, upper]`.
    StateBound { dim: usize, lower: f64, upper: f64 },
    /// The automaton must not spend more than `max_duration` continuous time
    /// in mode `mode` during the trace.
    ModeTime { mode: usize, max_duration: f64 },
}
