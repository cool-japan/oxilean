//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet};

/// Standard interval widening operator.
pub struct IntervalWidening;
impl IntervalWidening {
    /// Apply interval widening: a ▽ b.
    pub fn apply(a: &IntervalDomain, b: &IntervalDomain) -> IntervalDomain {
        a.widen(b)
    }
}
/// An abstract state: a map from variable name to interval abstract value.
#[derive(Debug, Clone)]
pub struct AbstractState {
    /// Variable name → interval abstract value
    pub vars: HashMap<String, IntervalDomain>,
}
impl AbstractState {
    /// The bottom state (unreachable program point).
    pub fn bottom() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }
    /// Look up the abstract value of a variable (top if unknown).
    pub fn get(&self, var: &str) -> IntervalDomain {
        self.vars
            .get(var)
            .cloned()
            .unwrap_or_else(IntervalDomain::top)
    }
    /// Set the abstract value of a variable.
    pub fn set(&mut self, var: impl Into<String>, val: IntervalDomain) {
        self.vars.insert(var.into(), val);
    }
    /// Join two abstract states pointwise.
    pub fn join(&self, other: &Self) -> Self {
        let mut result = self.clone();
        for (k, v) in &other.vars {
            let joined = result.get(k).join(v);
            result.vars.insert(k.clone(), joined);
        }
        result
    }
    /// Widening of two abstract states pointwise.
    pub fn widen(&self, other: &Self) -> Self {
        let mut result = self.clone();
        for (k, v) in &other.vars {
            let widened = result.get(k).widen(v);
            result.vars.insert(k.clone(), widened);
        }
        result
    }
}
/// A lightweight polyhedral domain element: a set of linear inequalities.
pub struct PolyhedralDomain {
    /// Number of variables
    pub n: usize,
    /// Constraints: each is (coefficients, rhs) meaning sum(a_i * x_i) ≤ b
    pub constraints: Vec<(Vec<f64>, f64)>,
}
impl PolyhedralDomain {
    /// Create the top polyhedron (no constraints).
    pub fn top(n: usize) -> Self {
        Self {
            n,
            constraints: vec![],
        }
    }
    /// Add a constraint: coeffs · x ≤ rhs.
    pub fn add_constraint(&mut self, coeffs: Vec<f64>, rhs: f64) {
        if coeffs.len() == self.n {
            self.constraints.push((coeffs, rhs));
        }
    }
    /// Check if a point satisfies all constraints.
    pub fn contains(&self, point: &[f64]) -> bool {
        if point.len() != self.n {
            return false;
        }
        self.constraints.iter().all(|(coeffs, rhs)| {
            let sum: f64 = coeffs.iter().zip(point.iter()).map(|(a, x)| a * x).sum();
            sum <= *rhs
        })
    }
    /// Number of constraints.
    pub fn num_constraints(&self) -> usize {
        self.constraints.len()
    }
}
/// The abstract value for the parity domain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParityDomain {
    /// No concrete values (unreachable)
    Bottom,
    /// Even integers only
    Even,
    /// Odd integers only
    Odd,
    /// All integers (no parity information)
    Top,
}
impl ParityDomain {
    /// Join of two parity abstract values.
    pub fn join(&self, other: &Self) -> Self {
        use ParityDomain::*;
        match (self, other) {
            (Bottom, x) | (x, Bottom) => x.clone(),
            (Top, _) | (_, Top) => Top,
            (a, b) if a == b => a.clone(),
            _ => Top,
        }
    }
    /// Abstract semantics of addition for parity.
    pub fn add(&self, other: &Self) -> Self {
        use ParityDomain::*;
        match (self, other) {
            (Bottom, _) | (_, Bottom) => Bottom,
            (Top, _) | (_, Top) => Top,
            (Even, Even) => Even,
            (Odd, Odd) => Even,
            (Even, Odd) | (Odd, Even) => Odd,
        }
    }
    /// Abstract semantics of multiplication for parity.
    pub fn mul(&self, other: &Self) -> Self {
        use ParityDomain::*;
        match (self, other) {
            (Bottom, _) | (_, Bottom) => Bottom,
            (Even, _) | (_, Even) => Even,
            (Odd, Odd) => Odd,
            _ => Top,
        }
    }
}
/// Abstract element for a probabilistic program: an interval over the
/// probability of being in each of k abstract states (partition of the state space).
///
/// Represents distributions where each partition cell has a probability
/// in [lo_i, hi_i] with Σ lo_i ≤ 1 ≤ Σ hi_i.
#[allow(dead_code)]
pub struct ProbabilisticAbstractDomain {
    /// Number of abstract partition cells
    pub k: usize,
    /// Lower bound on probability of each cell
    pub prob_lower: Vec<f64>,
    /// Upper bound on probability of each cell
    pub prob_upper: Vec<f64>,
}
#[allow(dead_code)]
impl ProbabilisticAbstractDomain {
    /// Create a uniform distribution abstraction over k cells.
    pub fn uniform(k: usize) -> Self {
        let p = 1.0 / k as f64;
        Self {
            k,
            prob_lower: vec![p; k],
            prob_upper: vec![p; k],
        }
    }
    /// Create the top element (no probability information: [0, 1] per cell).
    pub fn top(k: usize) -> Self {
        Self {
            k,
            prob_lower: vec![0.0; k],
            prob_upper: vec![1.0; k],
        }
    }
    /// Create a Dirac distribution concentrated on cell i.
    pub fn dirac(k: usize, i: usize) -> Self {
        let mut lower = vec![0.0; k];
        let mut upper = vec![0.0; k];
        lower[i] = 1.0;
        upper[i] = 1.0;
        Self {
            k,
            prob_lower: lower,
            prob_upper: upper,
        }
    }
    /// Join: componentwise interval join (least upper bound).
    pub fn join(&self, other: &Self) -> Self {
        assert_eq!(self.k, other.k);
        Self {
            k: self.k,
            prob_lower: self
                .prob_lower
                .iter()
                .zip(&other.prob_lower)
                .map(|(a, b)| a.min(*b))
                .collect(),
            prob_upper: self
                .prob_upper
                .iter()
                .zip(&other.prob_upper)
                .map(|(a, b)| a.max(*b))
                .collect(),
        }
    }
    /// Compute abstract expected value of a bounded function f: f[i] = value on cell i.
    /// Returns [lower, upper] interval on E[f].
    pub fn abstract_expectation(&self, f: &[f64]) -> (f64, f64) {
        assert_eq!(f.len(), self.k);
        let lower_e: f64 = self
            .prob_lower
            .iter()
            .zip(f.iter())
            .map(|(p, fi)| p * fi)
            .sum();
        let upper_e: f64 = self
            .prob_upper
            .iter()
            .zip(f.iter())
            .map(|(p, fi)| p * fi)
            .sum();
        (lower_e, upper_e)
    }
    /// Check if the abstract element is sound (lower ≤ upper per cell,
    /// and lower bounds sum ≤ 1, upper bounds sum ≥ 1 where meaningful).
    pub fn is_sound(&self) -> bool {
        let lo_ok = self
            .prob_lower
            .iter()
            .zip(&self.prob_upper)
            .all(|(l, u)| l <= u && *l >= 0.0 && *u <= 1.0);
        let sum_lo: f64 = self.prob_lower.iter().sum();
        let sum_hi: f64 = self.prob_upper.iter().sum();
        lo_ok && sum_lo <= 1.0 + 1e-9 && sum_hi >= 1.0 - 1e-9
    }
}
/// A single-neuron abstract element in the DeepPoly domain.
///
/// Each neuron y_i has a concrete value bounded by a linear expression:
///   lb_coeffs · x + lb_bias ≤ y_i ≤ ub_coeffs · x + ub_bias
/// where x are the (input) neurons from a preceding layer.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DeepPolyNeuron {
    /// Coefficients of the lower linear bound (over preceding layer neurons)
    pub lb_coeffs: Vec<f64>,
    /// Constant of the lower linear bound
    pub lb_bias: f64,
    /// Coefficients of the upper linear bound
    pub ub_coeffs: Vec<f64>,
    /// Constant of the upper linear bound
    pub ub_bias: f64,
    /// Concrete lower bound (after back-substitution)
    pub concrete_lb: f64,
    /// Concrete upper bound (after back-substitution)
    pub concrete_ub: f64,
}
#[allow(dead_code)]
impl DeepPolyNeuron {
    /// Create a constant neuron (no uncertainty).
    pub fn constant(val: f64, input_dim: usize) -> Self {
        Self {
            lb_coeffs: vec![0.0; input_dim],
            lb_bias: val,
            ub_coeffs: vec![0.0; input_dim],
            ub_bias: val,
            concrete_lb: val,
            concrete_ub: val,
        }
    }
    /// Apply abstract ReLU: case analysis on the concrete bounds.
    pub fn abstract_relu(&self) -> Self {
        let input_dim = self.lb_coeffs.len();
        if self.concrete_ub <= 0.0 {
            DeepPolyNeuron::constant(0.0, input_dim)
        } else if self.concrete_lb >= 0.0 {
            self.clone()
        } else {
            let slope = self.concrete_ub / (self.concrete_ub - self.concrete_lb);
            let ub_coeffs: Vec<f64> = self.ub_coeffs.iter().map(|c| slope * c).collect();
            let ub_bias = slope * (self.ub_bias - self.concrete_lb);
            DeepPolyNeuron {
                lb_coeffs: vec![0.0; input_dim],
                lb_bias: 0.0,
                ub_coeffs,
                ub_bias,
                concrete_lb: 0.0,
                concrete_ub: slope * (self.concrete_ub - self.concrete_lb),
            }
        }
    }
    /// Check if a concrete value is within the abstract bounds.
    pub fn contains_concrete(&self, val: f64) -> bool {
        val >= self.concrete_lb && val <= self.concrete_ub
    }
}
/// Estimated loop bound: an upper bound on iteration count.
pub struct LoopBound {
    /// Estimated maximum iterations
    pub bound: Option<u64>,
}
impl LoopBound {
    /// Unknown bound.
    pub fn unknown() -> Self {
        Self { bound: None }
    }
    /// Known finite bound.
    pub fn finite(n: u64) -> Self {
        Self { bound: Some(n) }
    }
    /// Estimate loop bound from interval [0, n]: bound is n+1 iterations.
    pub fn from_interval(interval: &IntervalDomain) -> Self {
        match &interval.upper {
            Bound::Finite(n) if *n >= 0 => Self::finite(*n as u64 + 1),
            _ => Self::unknown(),
        }
    }
    /// Check if the loop is known to terminate.
    pub fn terminates(&self) -> bool {
        self.bound.is_some()
    }
}
/// A bound value: either a finite integer or ±∞.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Bound {
    /// Negative infinity
    NegInf,
    /// A finite integer value
    Finite(i64),
    /// Positive infinity
    PosInf,
}
impl Bound {
    /// Add two bounds (with infinity arithmetic).
    pub fn add(&self, other: &Self) -> Self {
        match (self, other) {
            (Bound::NegInf, Bound::PosInf) | (Bound::PosInf, Bound::NegInf) => Bound::NegInf,
            (Bound::NegInf, _) | (_, Bound::NegInf) => Bound::NegInf,
            (Bound::PosInf, _) | (_, Bound::PosInf) => Bound::PosInf,
            (Bound::Finite(a), Bound::Finite(b)) => Bound::Finite(a.saturating_add(*b)),
        }
    }
    /// Negate a bound.
    pub fn neg(&self) -> Self {
        match self {
            Bound::NegInf => Bound::PosInf,
            Bound::PosInf => Bound::NegInf,
            Bound::Finite(n) => Bound::Finite(-n),
        }
    }
}
/// Taint analysis: tracks tainted (user-controlled) values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaintValue {
    /// Value is clean (no user input)
    Clean,
    /// Value may be tainted
    Tainted,
    /// Unreachable (bottom)
    Bottom,
}
/// The abstraction function: smallest interval containing a concrete set.
pub struct AbstractionFunction {
    gc: GaloisConnection,
}
impl AbstractionFunction {
    /// Create from the standard interval Galois connection.
    pub fn new() -> Self {
        Self {
            gc: GaloisConnection::interval_galois(),
        }
    }
    /// Compute α(S): best abstract value for a set of integers.
    pub fn apply(&self, vals: &[i64]) -> IntervalDomain {
        self.gc.abstract_of(vals)
    }
}
/// The abstract value for the sign domain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignDomain {
    /// No concrete values (unreachable)
    Bottom,
    /// Strictly negative integers (< 0)
    Neg,
    /// Zero only
    Zero,
    /// Strictly positive integers (> 0)
    Pos,
    /// Non-negative integers (≥ 0)
    NonNeg,
    /// Non-positive integers (≤ 0)
    NonPos,
    /// All integers (no information)
    Top,
}
impl SignDomain {
    /// Join (least upper bound ⊔) of two sign abstract values.
    pub fn join(&self, other: &Self) -> Self {
        use SignDomain::*;
        match (self, other) {
            (Bottom, x) | (x, Bottom) => x.clone(),
            (Top, _) | (_, Top) => Top,
            (a, b) if a == b => a.clone(),
            (Zero, Pos) | (Pos, Zero) => NonNeg,
            (Zero, Neg) | (Neg, Zero) => NonPos,
            (NonNeg, Neg) | (Neg, NonNeg) => Top,
            (NonPos, Pos) | (Pos, NonPos) => Top,
            (Neg, Pos) | (Pos, Neg) => Top,
            (NonNeg, NonPos) | (NonPos, NonNeg) => Top,
            _ => Top,
        }
    }
    /// Widening for sign domain (same as join since it's a finite lattice).
    pub fn widen(&self, other: &Self) -> Self {
        self.join(other)
    }
    /// Abstract semantics of addition.
    pub fn add(&self, other: &Self) -> Self {
        use SignDomain::*;
        match (self, other) {
            (Bottom, _) | (_, Bottom) => Bottom,
            (Top, _) | (_, Top) => Top,
            (Zero, x) | (x, Zero) => x.clone(),
            (Pos, Pos) => Pos,
            (Neg, Neg) => Neg,
            (NonNeg, NonNeg) => NonNeg,
            (NonPos, NonPos) => NonPos,
            (Pos, NonNeg) | (NonNeg, Pos) => Pos,
            (Neg, NonPos) | (NonPos, Neg) => Neg,
            _ => Top,
        }
    }
    /// Abstract semantics of negation.
    pub fn neg(&self) -> Self {
        use SignDomain::*;
        match self {
            Bottom => Bottom,
            Neg => Pos,
            Zero => Zero,
            Pos => Neg,
            NonNeg => NonPos,
            NonPos => NonNeg,
            Top => Top,
        }
    }
}
/// The octagon domain for n variables: stores bounds on ±xi ± xj.
/// Encoded as a difference-bound matrix (DBM) for simplicity.
pub struct OctagonDomain {
    /// Number of variables
    pub n: usize,
    /// DBM entries: entry [2i][2j] = bound on xi - xj, etc.
    pub matrix: Vec<Vec<Option<i64>>>,
}
impl OctagonDomain {
    /// Create a top octagon (no constraints) for n variables.
    pub fn top(n: usize) -> Self {
        let size = 2 * n;
        Self {
            n,
            matrix: vec![vec![None; size]; size],
        }
    }
    /// Create a bottom octagon (all bounds = -∞, contradictory).
    pub fn bottom(n: usize) -> Self {
        let size = 2 * n;
        let mut matrix = vec![vec![None; size]; size];
        for i in 0..size {
            matrix[i][i] = Some(-1);
        }
        Self { n, matrix }
    }
    /// Add a unary constraint xi ≤ c.
    pub fn add_upper_bound(&mut self, i: usize, c: i64) {
        if i < self.n {
            let pos = 2 * i;
            let neg = 2 * i + 1;
            let cur = self.matrix[pos][neg];
            let val = 2 * c;
            self.matrix[pos][neg] = Some(match cur {
                None => val,
                Some(v) => v.min(val),
            });
        }
    }
    /// Check if the octagon is satisfiable (no negative-weight cycles).
    pub fn is_satisfiable(&self) -> bool {
        for i in 0..2 * self.n {
            if let Some(v) = self.matrix[i][i] {
                if v < 0 {
                    return false;
                }
            }
        }
        true
    }
}
/// Fixpoint computation via Kleene iteration with widening.
pub struct FixpointComputation {
    /// Maximum number of iterations before forcing termination
    pub max_iter: usize,
    /// Widening delay
    pub widen_delay: usize,
}
impl FixpointComputation {
    /// Create a fixpoint computation with standard settings.
    pub fn new() -> Self {
        Self {
            max_iter: 1000,
            widen_delay: 3,
        }
    }
    /// Compute the least fixpoint of f starting from init, using widening.
    pub fn compute<F>(&self, f: F, init: AbstractState) -> AbstractState
    where
        F: Fn(&AbstractState) -> AbstractState,
    {
        let mut current = init;
        let mut delay = DelayedWidening::new(self.widen_delay);
        for _ in 0..self.max_iter {
            let next = f(&current);
            let widened = AbstractState {
                vars: {
                    let mut vars = current.vars.clone();
                    for (k, v) in &next.vars {
                        let cur_v = current.get(k);
                        let w = delay.apply(&cur_v, v);
                        vars.insert(k.clone(), w);
                    }
                    vars
                },
            };
            if widened
                .vars
                .iter()
                .all(|(k, v)| current.vars.get(k) == Some(v))
                && current.vars.len() == widened.vars.len()
            {
                return widened;
            }
            current = widened;
        }
        current
    }
}
/// The concretization function: set of integers represented by an abstract value.
pub struct ConcretizationFunction {
    gc: GaloisConnection,
}
impl ConcretizationFunction {
    /// Create from the standard interval Galois connection.
    pub fn new() -> Self {
        Self {
            gc: GaloisConnection::interval_galois(),
        }
    }
    /// Compute γ(a): all concrete integers represented (None if infinite).
    pub fn apply(&self, a: &IntervalDomain) -> Option<Vec<i64>> {
        self.gc.concretize(a)
    }
}
/// Taint analysis state: map from variable to taint status.
pub struct TaintAnalysis {
    /// Variable → taint status
    pub taint: HashMap<String, TaintValue>,
}
impl TaintAnalysis {
    /// Create empty taint analysis state.
    pub fn new() -> Self {
        Self {
            taint: HashMap::new(),
        }
    }
    /// Mark a variable as a taint source.
    pub fn add_source(&mut self, var: impl Into<String>) {
        self.taint.insert(var.into(), TaintValue::Tainted);
    }
    /// Check if a variable is tainted.
    pub fn is_tainted(&self, var: &str) -> bool {
        matches!(self.taint.get(var), Some(TaintValue::Tainted))
    }
    /// Propagate taint: result of operation is tainted if any input is.
    pub fn propagate(&mut self, result: impl Into<String>, inputs: &[&str]) {
        let tainted = inputs.iter().any(|v| self.is_tainted(v));
        let val = if tainted {
            TaintValue::Tainted
        } else {
            TaintValue::Clean
        };
        self.taint.insert(result.into(), val);
    }
}
/// The interval domain: an abstract value [lower, upper].
/// Represents the set of integers n with lower ≤ n ≤ upper.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntervalDomain {
    /// Lower bound (inclusive), or NegInf
    pub lower: Bound,
    /// Upper bound (inclusive), or PosInf
    pub upper: Bound,
    /// Whether this is the bottom element (empty set)
    pub is_bottom: bool,
}
impl IntervalDomain {
    /// Bottom element (empty set, unreachable).
    pub fn bottom() -> Self {
        Self {
            lower: Bound::PosInf,
            upper: Bound::NegInf,
            is_bottom: true,
        }
    }
    /// Top element (all integers).
    pub fn top() -> Self {
        Self {
            lower: Bound::NegInf,
            upper: Bound::PosInf,
            is_bottom: false,
        }
    }
    /// A constant interval [n, n].
    pub fn constant(n: i64) -> Self {
        Self {
            lower: Bound::Finite(n),
            upper: Bound::Finite(n),
            is_bottom: false,
        }
    }
    /// An interval [l, u].
    pub fn new(lower: Bound, upper: Bound) -> Self {
        if lower > upper {
            Self::bottom()
        } else {
            Self {
                lower,
                upper,
                is_bottom: false,
            }
        }
    }
    /// Join (least upper bound): smallest interval containing both.
    pub fn join(&self, other: &Self) -> Self {
        if self.is_bottom {
            return other.clone();
        }
        if other.is_bottom {
            return self.clone();
        }
        Self::new(
            self.lower.clone().min(other.lower.clone()),
            self.upper.clone().max(other.upper.clone()),
        )
    }
    /// Meet (greatest lower bound): intersection of two intervals.
    pub fn meet(&self, other: &Self) -> Self {
        if self.is_bottom || other.is_bottom {
            return Self::bottom();
        }
        Self::new(
            self.lower.clone().max(other.lower.clone()),
            self.upper.clone().min(other.upper.clone()),
        )
    }
    /// Widening: accelerate convergence by jumping to ±∞.
    pub fn widen(&self, other: &Self) -> Self {
        if self.is_bottom {
            return other.clone();
        }
        if other.is_bottom {
            return self.clone();
        }
        let new_lower = if other.lower < self.lower {
            Bound::NegInf
        } else {
            self.lower.clone()
        };
        let new_upper = if other.upper > self.upper {
            Bound::PosInf
        } else {
            self.upper.clone()
        };
        Self::new(new_lower, new_upper)
    }
    /// Narrowing: refine using constraints from other.
    pub fn narrow(&self, other: &Self) -> Self {
        if self.is_bottom || other.is_bottom {
            return Self::bottom();
        }
        let new_lower = if self.lower == Bound::NegInf {
            other.lower.clone()
        } else {
            self.lower.clone()
        };
        let new_upper = if self.upper == Bound::PosInf {
            other.upper.clone()
        } else {
            self.upper.clone()
        };
        Self::new(new_lower, new_upper)
    }
    /// Abstract addition: [a,b] + [c,d] = [a+c, b+d].
    pub fn add(&self, other: &Self) -> Self {
        if self.is_bottom || other.is_bottom {
            return Self::bottom();
        }
        Self::new(self.lower.add(&other.lower), self.upper.add(&other.upper))
    }
    /// Check if a concrete integer is in the interval.
    pub fn contains(&self, n: i64) -> bool {
        if self.is_bottom {
            return false;
        }
        let lo_ok = match &self.lower {
            Bound::NegInf => true,
            Bound::Finite(l) => *l <= n,
            Bound::PosInf => false,
        };
        let hi_ok = match &self.upper {
            Bound::NegInf => false,
            Bound::Finite(u) => n <= *u,
            Bound::PosInf => true,
        };
        lo_ok && hi_ok
    }
}
/// An abstract transformer: a function from abstract states to abstract states.
pub struct AbstractTransformer {
    /// The transformation function
    pub transform: Box<dyn Fn(&AbstractState) -> AbstractState>,
}
impl AbstractTransformer {
    /// Create an abstract transformer from a closure.
    pub fn new<F: Fn(&AbstractState) -> AbstractState + 'static>(f: F) -> Self {
        Self {
            transform: Box::new(f),
        }
    }
    /// Apply the transformer to an abstract state.
    pub fn apply(&self, state: &AbstractState) -> AbstractState {
        (self.transform)(state)
    }
    /// Compose two transformers: first apply self, then other.
    pub fn compose(self, other: Self) -> Self {
        Self::new(move |s| other.apply(&self.apply(s)))
    }
}
/// Direction of a dataflow analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnalysisDirection {
    /// Forward: propagate from entry to exit
    Forward,
    /// Backward: propagate from exit to entry
    Backward,
}
/// Null pointer analysis: may/must nullness at each program point.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NullValue {
    /// Definitely null
    MustNull,
    /// Definitely non-null
    NonNull,
    /// May or may not be null
    MayNull,
    /// Unreachable (bottom)
    Bottom,
}
/// A zonotope: x = center + Σ_i ε_i * generators[i], |ε_i| ≤ 1.
///
/// Zonotopes are closed under affine maps and provide compact relational
/// over-approximations. Each generator g_i encodes a direction of uncertainty.
#[allow(dead_code)]
pub struct ZonotopeDomain {
    /// Number of dimensions
    pub dim: usize,
    /// Center point
    pub center: Vec<f64>,
    /// Generators (each has `dim` components)
    pub generators: Vec<Vec<f64>>,
}
#[allow(dead_code)]
impl ZonotopeDomain {
    /// Create a point zonotope (no uncertainty).
    pub fn point(center: Vec<f64>) -> Self {
        let dim = center.len();
        Self {
            dim,
            center,
            generators: vec![],
        }
    }
    /// Create an interval zonotope from lower and upper bounds.
    /// center = (lo + hi) / 2, one generator per dimension = (hi - lo) / 2.
    pub fn from_intervals(lower: &[f64], upper: &[f64]) -> Self {
        assert_eq!(lower.len(), upper.len());
        let dim = lower.len();
        let center: Vec<f64> = lower
            .iter()
            .zip(upper.iter())
            .map(|(l, h)| (l + h) / 2.0)
            .collect();
        let generators: Vec<Vec<f64>> = (0..dim)
            .map(|i| {
                let mut g = vec![0.0; dim];
                g[i] = (upper[i] - lower[i]) / 2.0;
                g
            })
            .collect();
        Self {
            dim,
            center,
            generators,
        }
    }
    /// Compute the interval over-approximation of the zonotope.
    /// Returns (lower, upper) bounds per dimension.
    pub fn to_intervals(&self) -> (Vec<f64>, Vec<f64>) {
        let mut lower = self.center.clone();
        let mut upper = self.center.clone();
        for g in &self.generators {
            for (i, gi) in g.iter().enumerate() {
                let abs_g = gi.abs();
                lower[i] -= abs_g;
                upper[i] += abs_g;
            }
        }
        (lower, upper)
    }
    /// Apply an affine map: y = A * x + b (exact in the zonotope domain).
    /// `matrix` is (out_dim x self.dim), `bias` has length out_dim.
    pub fn affine_map(&self, matrix: &[Vec<f64>], bias: &[f64]) -> Self {
        let out_dim = matrix.len();
        let new_center: Vec<f64> = (0..out_dim)
            .map(|i| {
                let dot: f64 = matrix[i]
                    .iter()
                    .zip(self.center.iter())
                    .map(|(a, c)| a * c)
                    .sum();
                dot + bias[i]
            })
            .collect();
        let new_generators: Vec<Vec<f64>> = self
            .generators
            .iter()
            .map(|g| {
                (0..out_dim)
                    .map(|i| matrix[i].iter().zip(g.iter()).map(|(a, gi)| a * gi).sum())
                    .collect()
            })
            .collect();
        Self {
            dim: out_dim,
            center: new_center,
            generators: new_generators,
        }
    }
    /// Join two zonotopes: take their interval hull as a new zonotope.
    pub fn join(&self, other: &Self) -> Self {
        assert_eq!(self.dim, other.dim);
        let (l1, u1) = self.to_intervals();
        let (l2, u2) = other.to_intervals();
        let lower: Vec<f64> = l1.iter().zip(l2.iter()).map(|(a, b)| a.min(*b)).collect();
        let upper: Vec<f64> = u1.iter().zip(u2.iter()).map(|(a, b)| a.max(*b)).collect();
        Self::from_intervals(&lower, &upper)
    }
    /// Check if a point is (conservatively) in the zonotope
    /// by checking against the interval over-approximation.
    pub fn may_contain(&self, point: &[f64]) -> bool {
        if point.len() != self.dim {
            return false;
        }
        let (lower, upper) = self.to_intervals();
        lower
            .iter()
            .zip(upper.iter())
            .zip(point.iter())
            .all(|((l, u), p)| *l <= *p && *p <= *u)
    }
}
/// Array bounds analysis: interval analysis for array indices.
pub struct ArrayBoundsAnalysis {
    /// Variable → index interval
    pub index_bounds: HashMap<String, IntervalDomain>,
    /// Array name → size (as interval)
    pub array_sizes: HashMap<String, IntervalDomain>,
}
impl ArrayBoundsAnalysis {
    /// Create a new empty array bounds analysis.
    pub fn new() -> Self {
        Self {
            index_bounds: HashMap::new(),
            array_sizes: HashMap::new(),
        }
    }
    /// Check if access array[idx_var] is provably safe.
    pub fn is_safe_access(&self, array: &str, idx_var: &str) -> bool {
        let idx = self
            .index_bounds
            .get(idx_var)
            .cloned()
            .unwrap_or_else(IntervalDomain::top);
        let sz = match self.array_sizes.get(array) {
            Some(s) => s.clone(),
            None => return false,
        };
        if idx.is_bottom {
            return true;
        }
        let lo_ok = match &idx.lower {
            Bound::Finite(l) => *l >= 0,
            _ => false,
        };
        let hi_ok = match (&idx.upper, &sz.lower) {
            (Bound::Finite(hi), Bound::Finite(sz_lo)) => *hi < *sz_lo,
            _ => false,
        };
        lo_ok && hi_ok
    }
}
/// Delayed widening: apply regular join for first k steps, then widen.
pub struct DelayedWidening {
    /// Number of steps before switching to widening
    pub delay: usize,
    /// Current step counter
    pub step: usize,
}
impl DelayedWidening {
    /// Create a delayed widening operator with given delay.
    pub fn new(delay: usize) -> Self {
        Self { delay, step: 0 }
    }
    /// Apply: join for step < delay, widening otherwise.
    pub fn apply(&mut self, a: &IntervalDomain, b: &IntervalDomain) -> IntervalDomain {
        self.step += 1;
        if self.step <= self.delay {
            a.join(b)
        } else {
            a.widen(b)
        }
    }
    /// Reset the step counter.
    pub fn reset(&mut self) {
        self.step = 0;
    }
}
/// A Galois connection (α, γ) between concrete sets (Vec<i64>) and intervals.
pub struct GaloisConnection {
    /// α: ℘(ℤ) → IntervalDomain (abstraction)
    pub alpha: Box<dyn Fn(&[i64]) -> IntervalDomain>,
    /// γ: IntervalDomain → Vec<i64> (concretization, approximate for ∞)
    pub gamma: Box<dyn Fn(&IntervalDomain) -> Option<Vec<i64>>>,
}
impl GaloisConnection {
    /// Build the standard Galois connection for the interval domain.
    pub fn interval_galois() -> Self {
        Self {
            alpha: Box::new(|vals: &[i64]| {
                if vals.is_empty() {
                    return IntervalDomain::bottom();
                }
                let lo = vals
                    .iter()
                    .copied()
                    .min()
                    .expect("vals is non-empty: checked by early return");
                let hi = vals
                    .iter()
                    .copied()
                    .max()
                    .expect("vals is non-empty: checked by early return");
                IntervalDomain::new(Bound::Finite(lo), Bound::Finite(hi))
            }),
            gamma: Box::new(|interval: &IntervalDomain| {
                if interval.is_bottom {
                    return Some(vec![]);
                }
                match (&interval.lower, &interval.upper) {
                    (Bound::Finite(l), Bound::Finite(u)) if u - l <= 1000 => {
                        Some((*l..=*u).collect())
                    }
                    _ => None,
                }
            }),
        }
    }
    /// Compute the abstraction of a concrete set.
    pub fn abstract_of(&self, vals: &[i64]) -> IntervalDomain {
        (self.alpha)(vals)
    }
    /// Compute (a finite approximation of) the concretization.
    pub fn concretize(&self, a: &IntervalDomain) -> Option<Vec<i64>> {
        (self.gamma)(a)
    }
}
/// Reachability analysis: computes which program points are reachable.
pub struct ReachabilityAnalysis {
    /// Set of reachable labels
    pub reachable: std::collections::HashSet<usize>,
}
impl ReachabilityAnalysis {
    /// Create a new empty reachability analysis.
    pub fn new() -> Self {
        Self {
            reachable: std::collections::HashSet::new(),
        }
    }
    /// Mark a program point as reachable.
    pub fn mark_reachable(&mut self, label: usize) {
        self.reachable.insert(label);
    }
    /// Check if a program point is reachable.
    pub fn is_reachable(&self, label: usize) -> bool {
        self.reachable.contains(&label)
    }
}
/// An assume-guarantee contract for a program component.
///
/// Represents (A, G): if pre-state satisfies A, then post-state satisfies G.
/// Enables compositional verification without analyzing full systems.
#[allow(dead_code)]
pub struct AssumeGuaranteeContract {
    /// The assumption: abstract pre-condition (lower bound on what env provides)
    pub assumption: AbstractState,
    /// The guarantee: abstract post-condition (upper bound on what component delivers)
    pub guarantee: AbstractState,
    /// Human-readable name of the component
    pub component_name: String,
}
#[allow(dead_code)]
impl AssumeGuaranteeContract {
    /// Create a new contract with given assumption and guarantee.
    pub fn new(
        component_name: impl Into<String>,
        assumption: AbstractState,
        guarantee: AbstractState,
    ) -> Self {
        Self {
            assumption,
            guarantee,
            component_name: component_name.into(),
        }
    }
    /// Compose two contracts: component B's assumption must be implied by A's guarantee.
    /// Returns the composed contract if compatible, None if incompatible.
    pub fn compose(&self, next: &Self) -> Option<Self> {
        let compatible = next.assumption.vars.iter().all(|(var, needed)| {
            if let Some(provided) = self.guarantee.vars.get(var) {
                match (
                    &provided.lower,
                    &needed.lower,
                    &provided.upper,
                    &needed.upper,
                ) {
                    (
                        Bound::Finite(pl),
                        Bound::Finite(nl),
                        Bound::Finite(pu),
                        Bound::Finite(nu),
                    ) => pl >= nl && pu <= nu,
                    _ => true,
                }
            } else {
                true
            }
        });
        if compatible {
            Some(Self::new(
                format!("{};{}", self.component_name, next.component_name),
                self.assumption.clone(),
                next.guarantee.clone(),
            ))
        } else {
            None
        }
    }
    /// Check if a given (pre, post) pair satisfies this contract.
    pub fn check(&self, pre: &AbstractState, post: &AbstractState) -> bool {
        let pre_ok = self.assumption.vars.iter().all(|(var, assumed)| {
            let actual = pre.get(var);
            match (&actual.lower, &assumed.lower, &actual.upper, &assumed.upper) {
                (Bound::Finite(al), Bound::Finite(asl), Bound::Finite(au), Bound::Finite(asu)) => {
                    al >= asl && au <= asu
                }
                _ => !actual.is_bottom,
            }
        });
        let post_ok = self.guarantee.vars.iter().all(|(var, guaranteed)| {
            let actual = post.get(var);
            match (
                &actual.lower,
                &guaranteed.lower,
                &actual.upper,
                &guaranteed.upper,
            ) {
                (Bound::Finite(al), Bound::Finite(gl), Bound::Finite(au), Bound::Finite(gu)) => {
                    al >= gl && au <= gu
                }
                _ => !actual.is_bottom,
            }
        });
        pre_ok && post_ok
    }
}
/// A full DeepPoly abstract element for one layer.
#[allow(dead_code)]
pub struct DeepPolyLayer {
    /// Neurons in this layer
    pub neurons: Vec<DeepPolyNeuron>,
}
#[allow(dead_code)]
impl DeepPolyLayer {
    /// Create from interval bounds on neurons (input layer).
    pub fn input_layer(lower: &[f64], upper: &[f64]) -> Self {
        let dim = lower.len();
        let neurons = (0..dim)
            .map(|i| {
                let mut lb_coeffs = vec![0.0; dim];
                let mut ub_coeffs = vec![0.0; dim];
                lb_coeffs[i] = 1.0;
                ub_coeffs[i] = 1.0;
                DeepPolyNeuron {
                    lb_coeffs,
                    lb_bias: 0.0,
                    ub_coeffs,
                    ub_bias: 0.0,
                    concrete_lb: lower[i],
                    concrete_ub: upper[i],
                }
            })
            .collect();
        Self { neurons }
    }
    /// Apply an affine layer W * x + b to produce a new abstract layer.
    pub fn affine(&self, weights: &[Vec<f64>], bias: &[f64]) -> Self {
        let in_dim = self.neurons.len();
        let out_dim = weights.len();
        let neurons = (0..out_dim)
            .map(|j| {
                let mut concrete_lb = bias[j];
                let mut concrete_ub = bias[j];
                for (i, neuron) in self.neurons.iter().enumerate() {
                    let w = weights[j][i];
                    if w >= 0.0 {
                        concrete_lb += w * neuron.concrete_lb;
                        concrete_ub += w * neuron.concrete_ub;
                    } else {
                        concrete_lb += w * neuron.concrete_ub;
                        concrete_ub += w * neuron.concrete_lb;
                    }
                }
                let lb_coeffs = vec![0.0; in_dim];
                let ub_coeffs = vec![0.0; in_dim];
                DeepPolyNeuron {
                    lb_coeffs,
                    lb_bias: concrete_lb,
                    ub_coeffs,
                    ub_bias: concrete_ub,
                    concrete_lb,
                    concrete_ub,
                }
            })
            .collect();
        Self { neurons }
    }
    /// Apply abstract ReLU to all neurons.
    pub fn relu(&self) -> Self {
        Self {
            neurons: self.neurons.iter().map(|n| n.abstract_relu()).collect(),
        }
    }
    /// Get the concrete lower bound on all neurons.
    pub fn concrete_lower(&self) -> Vec<f64> {
        self.neurons.iter().map(|n| n.concrete_lb).collect()
    }
    /// Get the concrete upper bound on all neurons.
    pub fn concrete_upper(&self) -> Vec<f64> {
        self.neurons.iter().map(|n| n.concrete_ub).collect()
    }
}
/// A template polyhedron: { x | C * x ≤ d } where C (the template) is fixed.
///
/// Fixing C allows efficient join, meet, and widening without full
/// Fourier–Motzkin elimination. Precision is controlled by the choice of C.
#[allow(dead_code)]
pub struct TemplatePolyhedronDomain {
    /// Number of variables
    pub dim: usize,
    /// Template matrix C: each row is a constraint direction (len = dim)
    pub template: Vec<Vec<f64>>,
    /// RHS bounds d: C * x ≤ d
    pub bounds: Vec<f64>,
    /// Whether this is the bottom (infeasible) element
    pub is_bottom: bool,
}
#[allow(dead_code)]
impl TemplatePolyhedronDomain {
    /// Create the top element (no constraints: all bounds = +∞).
    pub fn top(dim: usize, template: Vec<Vec<f64>>) -> Self {
        let k = template.len();
        Self {
            dim,
            template,
            bounds: vec![f64::INFINITY; k],
            is_bottom: false,
        }
    }
    /// Create the bottom element (infeasible: one bound = -∞).
    pub fn bottom(dim: usize, template: Vec<Vec<f64>>) -> Self {
        let k = template.len();
        Self {
            dim,
            template,
            bounds: vec![f64::NEG_INFINITY; k],
            is_bottom: true,
        }
    }
    /// Create from a point: d_i = c_i · point (tight bounds).
    pub fn from_point(dim: usize, template: Vec<Vec<f64>>, point: &[f64]) -> Self {
        assert_eq!(point.len(), dim);
        let bounds: Vec<f64> = template
            .iter()
            .map(|row| row.iter().zip(point.iter()).map(|(ci, xi)| ci * xi).sum())
            .collect();
        Self {
            dim,
            template,
            bounds,
            is_bottom: false,
        }
    }
    /// Join: take componentwise max of bounds (least upper bound).
    pub fn join(&self, other: &Self) -> Self {
        assert_eq!(self.template.len(), other.template.len());
        if self.is_bottom {
            return other.clone();
        }
        if other.is_bottom {
            return self.clone();
        }
        let bounds: Vec<f64> = self
            .bounds
            .iter()
            .zip(other.bounds.iter())
            .map(|(a, b)| a.max(*b))
            .collect();
        Self {
            dim: self.dim,
            template: self.template.clone(),
            bounds,
            is_bottom: false,
        }
    }
    /// Meet: take componentwise min of bounds (greatest lower bound).
    pub fn meet(&self, other: &Self) -> Self {
        assert_eq!(self.template.len(), other.template.len());
        if self.is_bottom || other.is_bottom {
            return Self::bottom(self.dim, self.template.clone());
        }
        let bounds: Vec<f64> = self
            .bounds
            .iter()
            .zip(other.bounds.iter())
            .map(|(a, b)| a.min(*b))
            .collect();
        Self {
            dim: self.dim,
            template: self.template.clone(),
            bounds,
            is_bottom: false,
        }
    }
    /// Widening: keep bound if stable, otherwise set to +∞.
    pub fn widen(&self, next: &Self) -> Self {
        assert_eq!(self.bounds.len(), next.bounds.len());
        if self.is_bottom {
            return next.clone();
        }
        let bounds: Vec<f64> = self
            .bounds
            .iter()
            .zip(next.bounds.iter())
            .map(|(a, b)| if *b <= *a { *a } else { f64::INFINITY })
            .collect();
        Self {
            dim: self.dim,
            template: self.template.clone(),
            bounds,
            is_bottom: false,
        }
    }
    /// Check if a point satisfies all template constraints.
    pub fn contains(&self, point: &[f64]) -> bool {
        if self.is_bottom || point.len() != self.dim {
            return false;
        }
        self.template
            .iter()
            .zip(self.bounds.iter())
            .all(|(row, &d)| {
                let lhs: f64 = row.iter().zip(point.iter()).map(|(c, x)| c * x).sum();
                lhs <= d
            })
    }
}
/// A dataflow analysis: associates abstract values to program points.
pub struct DataflowAnalysis {
    /// Whether this is a forward or backward analysis
    pub direction: AnalysisDirection,
    /// Abstract values at each program point (indexed by label)
    pub values: HashMap<usize, AbstractState>,
}
impl DataflowAnalysis {
    /// Create a new (initially bottom) forward dataflow analysis.
    pub fn new_forward() -> Self {
        Self {
            direction: AnalysisDirection::Forward,
            values: HashMap::new(),
        }
    }
    /// Create a new (initially bottom) backward dataflow analysis.
    pub fn new_backward() -> Self {
        Self {
            direction: AnalysisDirection::Backward,
            values: HashMap::new(),
        }
    }
    /// Get the abstract value at a program point.
    pub fn at(&self, label: usize) -> AbstractState {
        self.values
            .get(&label)
            .cloned()
            .unwrap_or_else(AbstractState::bottom)
    }
    /// Set the abstract value at a program point.
    pub fn set_at(&mut self, label: usize, state: AbstractState) {
        self.values.insert(label, state);
    }
}
/// Null pointer analysis state: map from variable to null status.
pub struct NullPointerAnalysis {
    /// Variable → nullness
    pub nullness: HashMap<String, NullValue>,
}
impl NullPointerAnalysis {
    /// Create empty null pointer analysis state.
    pub fn new() -> Self {
        Self {
            nullness: HashMap::new(),
        }
    }
    /// Check if a variable may be null.
    pub fn may_be_null(&self, var: &str) -> bool {
        matches!(
            self.nullness.get(var),
            Some(NullValue::MayNull) | Some(NullValue::MustNull) | None
        )
    }
    /// Check if a variable must be null.
    pub fn must_be_null(&self, var: &str) -> bool {
        matches!(self.nullness.get(var), Some(NullValue::MustNull))
    }
}
