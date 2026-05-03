//! # Process Algebra — Type Definitions
//!
//! Core types for CCS, CSP, labeled transition systems, bisimulation,
//! Hennessy-Milner logic, and behavioral equivalences.

use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;

// ─── Actions and Labels ───────────────────────────────────────────────────────

/// An **action** in a process algebra: either a visible communication label,
/// its co-name (complement), or the silent internal action τ.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Action {
    /// Visible input action on channel `name`.
    Input(String),
    /// Visible output action on channel `name` (co-name of input).
    Output(String),
    /// Silent/internal action τ (tau).
    Tau,
}

impl Action {
    /// Create an input action.
    pub fn input(name: &str) -> Self {
        Action::Input(name.to_string())
    }

    /// Create an output action.
    pub fn output(name: &str) -> Self {
        Action::Output(name.to_string())
    }

    /// Is this the silent action τ?
    pub fn is_tau(&self) -> bool {
        matches!(self, Action::Tau)
    }

    /// Is this action visible (not τ)?
    pub fn is_visible(&self) -> bool {
        !self.is_tau()
    }

    /// The complementary action (input ↔ output; τ → τ).
    pub fn complement(&self) -> Action {
        match self {
            Action::Input(s) => Action::Output(s.clone()),
            Action::Output(s) => Action::Input(s.clone()),
            Action::Tau => Action::Tau,
        }
    }

    /// Get the channel name if visible.
    pub fn channel(&self) -> Option<&str> {
        match self {
            Action::Input(s) | Action::Output(s) => Some(s.as_str()),
            Action::Tau => None,
        }
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Action::Input(s) => write!(f, "{}", s),
            Action::Output(s) => write!(f, "{}̄", s),
            Action::Tau => write!(f, "τ"),
        }
    }
}

// ─── CCS Process Terms ────────────────────────────────────────────────────────

/// A **CCS process term** `P`:
///
/// ```text
/// P ::= 0            -- nil: does nothing
///     | α.P          -- prefix: perform α then behave as P
///     | P + Q        -- choice: behave as P or Q (nondeterministic)
///     | P | Q        -- parallel: P and Q run concurrently
///     | P \ L        -- restriction: hide channels in set L
///     | P\[f\]         -- relabeling: rename actions by function f
///     | X            -- process variable (recursion)
///     | μX.P         -- recursive definition: X = P (fixpoint)
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CcsProcess {
    /// `0` — nil process (deadlock/termination).
    Nil,
    /// `α.P` — action prefix.
    Prefix(Action, Box<CcsProcess>),
    /// `P + Q` — nondeterministic choice.
    Choice(Box<CcsProcess>, Box<CcsProcess>),
    /// `P | Q` — parallel composition.
    Parallel(Box<CcsProcess>, Box<CcsProcess>),
    /// `P \ L` — restriction (hide labels in L ⊆ Act).
    Restriction(Box<CcsProcess>, HashSet<String>),
    /// `P\[f\]` — relabeling (rename by function).
    Relabeling(Box<CcsProcess>, HashMap<String, String>),
    /// `X` — process variable.
    Var(String),
    /// `μX.P` — recursive process definition.
    Rec(String, Box<CcsProcess>),
}

impl CcsProcess {
    /// Construct an `α.P` prefix.
    pub fn prefix(a: Action, p: CcsProcess) -> Self {
        CcsProcess::Prefix(a, Box::new(p))
    }

    /// Construct `P + Q`.
    pub fn choice(p: CcsProcess, q: CcsProcess) -> Self {
        CcsProcess::Choice(Box::new(p), Box::new(q))
    }

    /// Construct `P | Q`.
    pub fn parallel(p: CcsProcess, q: CcsProcess) -> Self {
        CcsProcess::Parallel(Box::new(p), Box::new(q))
    }

    /// Construct `P \ L`.
    pub fn restrict(p: CcsProcess, labels: HashSet<String>) -> Self {
        CcsProcess::Restriction(Box::new(p), labels)
    }

    /// Construct `P\[f\]`.
    pub fn relabel(p: CcsProcess, map: HashMap<String, String>) -> Self {
        CcsProcess::Relabeling(Box::new(p), map)
    }

    /// Construct `μX.P`.
    pub fn rec(var: &str, p: CcsProcess) -> Self {
        CcsProcess::Rec(var.to_string(), Box::new(p))
    }

    /// Substitute process `sub` for variable `var`.
    pub fn substitute(&self, var: &str, sub: &CcsProcess) -> CcsProcess {
        match self {
            CcsProcess::Nil => CcsProcess::Nil,
            CcsProcess::Prefix(a, p) => {
                CcsProcess::Prefix(a.clone(), Box::new(p.substitute(var, sub)))
            }
            CcsProcess::Choice(p, q) => CcsProcess::Choice(
                Box::new(p.substitute(var, sub)),
                Box::new(q.substitute(var, sub)),
            ),
            CcsProcess::Parallel(p, q) => CcsProcess::Parallel(
                Box::new(p.substitute(var, sub)),
                Box::new(q.substitute(var, sub)),
            ),
            CcsProcess::Restriction(p, l) => {
                CcsProcess::Restriction(Box::new(p.substitute(var, sub)), l.clone())
            }
            CcsProcess::Relabeling(p, f) => {
                CcsProcess::Relabeling(Box::new(p.substitute(var, sub)), f.clone())
            }
            CcsProcess::Var(x) => {
                if x == var {
                    sub.clone()
                } else {
                    CcsProcess::Var(x.clone())
                }
            }
            CcsProcess::Rec(x, p) => {
                if x == var {
                    CcsProcess::Rec(x.clone(), p.clone())
                } else {
                    CcsProcess::Rec(x.clone(), Box::new(p.substitute(var, sub)))
                }
            }
        }
    }

    /// Unfold one level of a recursive process `μX.P → P\[μX.P/X\]`.
    pub fn unfold(&self) -> CcsProcess {
        match self {
            CcsProcess::Rec(x, p) => {
                let rec = self.clone();
                p.substitute(x, &rec)
            }
            _ => self.clone(),
        }
    }
}

impl fmt::Display for CcsProcess {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CcsProcess::Nil => write!(f, "0"),
            CcsProcess::Prefix(a, p) => write!(f, "{}.{}", a, p),
            CcsProcess::Choice(p, q) => write!(f, "({} + {})", p, q),
            CcsProcess::Parallel(p, q) => write!(f, "({} | {})", p, q),
            CcsProcess::Restriction(p, l) => {
                let mut labels: Vec<&str> = l.iter().map(|s| s.as_str()).collect();
                labels.sort();
                write!(f, "({} \\ {{{}}})", p, labels.join(","))
            }
            CcsProcess::Relabeling(p, m) => {
                let mut pairs: Vec<(&str, &str)> =
                    m.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
                pairs.sort();
                let map_str: Vec<String> =
                    pairs.iter().map(|(k, v)| format!("{}/{}", v, k)).collect();
                write!(f, "{}[{}]", p, map_str.join(","))
            }
            CcsProcess::Var(x) => write!(f, "{}", x),
            CcsProcess::Rec(x, p) => write!(f, "μ{}.{}", x, p),
        }
    }
}

// ─── CSP Process Terms ────────────────────────────────────────────────────────

/// A **CSP (Communicating Sequential Processes)** term.
///
/// CSP extends CCS with synchronous parallel (must synchronize on shared events),
/// sequential composition, interrupt, and richer termination.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CspProcess {
    /// `STOP` — deadlock.
    Stop,
    /// `SKIP` — successful termination.
    Skip,
    /// `a → P` — prefix (event a then P).
    Prefix(String, Box<CspProcess>),
    /// `P □ Q` — external choice (environment chooses).
    ExternalChoice(Box<CspProcess>, Box<CspProcess>),
    /// `P ⊓ Q` — internal choice (process chooses).
    InternalChoice(Box<CspProcess>, Box<CspProcess>),
    /// `P ‖_A Q` — alphabetized parallel (synchronize on A).
    AlphaParallel(Box<CspProcess>, HashSet<String>, Box<CspProcess>),
    /// `P ; Q` — sequential composition (P then Q).
    Sequential(Box<CspProcess>, Box<CspProcess>),
    /// `P △ Q` — interrupt (Q can interrupt P at any time).
    Interrupt(Box<CspProcess>, Box<CspProcess>),
    /// `P \ A` — hiding (internalize events in A).
    Hide(Box<CspProcess>, HashSet<String>),
    /// Process reference by name (for recursion).
    Ref(String),
}

impl fmt::Display for CspProcess {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CspProcess::Stop => write!(f, "STOP"),
            CspProcess::Skip => write!(f, "SKIP"),
            CspProcess::Prefix(a, p) => write!(f, "{} → {}", a, p),
            CspProcess::ExternalChoice(p, q) => write!(f, "({} □ {})", p, q),
            CspProcess::InternalChoice(p, q) => write!(f, "({} ⊓ {})", p, q),
            CspProcess::AlphaParallel(p, a, q) => {
                let mut events: Vec<&str> = a.iter().map(|s| s.as_str()).collect();
                events.sort();
                write!(f, "({} ‖_{{{}}} {})", p, events.join(","), q)
            }
            CspProcess::Sequential(p, q) => write!(f, "({} ; {})", p, q),
            CspProcess::Interrupt(p, q) => write!(f, "({} △ {})", p, q),
            CspProcess::Hide(p, a) => {
                let mut events: Vec<&str> = a.iter().map(|s| s.as_str()).collect();
                events.sort();
                write!(f, "({} \\ {{{}}})", p, events.join(","))
            }
            CspProcess::Ref(name) => write!(f, "{}", name),
        }
    }
}

// ─── Labeled Transition System ────────────────────────────────────────────────

/// A **state** in a labeled transition system (LTS).
pub type State = usize;

/// A **transition** `(source, action, target)` in an LTS.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Transition {
    /// Source state.
    pub source: State,
    /// Action label.
    pub action: Action,
    /// Target state.
    pub target: State,
}

impl Transition {
    /// Create a transition.
    pub fn new(source: State, action: Action, target: State) -> Self {
        Transition {
            source,
            action,
            target,
        }
    }
}

impl fmt::Display for Transition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} --{}--> {}", self.source, self.action, self.target)
    }
}

/// A **Labeled Transition System** (LTS): a graph where edges are labeled with actions.
///
/// Formally `(S, Act, →, s₀)` where:
/// - `S` is a finite set of states
/// - `Act` is a set of actions (including τ)
/// - `→ ⊆ S × Act × S` is the transition relation
/// - `s₀ ∈ S` is the initial state
#[derive(Debug, Clone)]
pub struct Lts {
    /// Number of states (states are indices 0..n-1).
    pub num_states: usize,
    /// Initial state.
    pub initial: State,
    /// Transition relation.
    pub transitions: Vec<Transition>,
}

impl Lts {
    /// Create an LTS.
    pub fn new(num_states: usize, initial: State, transitions: Vec<Transition>) -> Self {
        Lts {
            num_states,
            initial,
            transitions,
        }
    }

    /// Get all transitions from a state.
    pub fn transitions_from(&self, s: State) -> Vec<&Transition> {
        self.transitions.iter().filter(|t| t.source == s).collect()
    }

    /// Get all transitions with a given action from a state.
    pub fn transitions_by_action(&self, s: State, a: &Action) -> Vec<State> {
        self.transitions
            .iter()
            .filter(|t| t.source == s && &t.action == a)
            .map(|t| t.target)
            .collect()
    }

    /// Compute the **weak transition** `⇒ₐ`: zero or more τ steps, then action `a`,
    /// then zero or more τ steps.
    pub fn weak_transitions(&self, s: State, a: &Action) -> HashSet<State> {
        if a.is_tau() {
            // Weak τ transition = τ-closure
            let mut reachable = self.tau_closure(s);
            reachable.insert(s);
            reachable
        } else {
            // s ⇒ₐ t: s →τ*→ s' →ₐ s'' →τ*→ t
            let pre_states = {
                let mut set = self.tau_closure(s);
                set.insert(s);
                set
            };
            let mut result = HashSet::new();
            for pre in pre_states {
                for &mid in &self.transitions_by_action(pre, a) {
                    let post = {
                        let mut set = self.tau_closure(mid);
                        set.insert(mid);
                        set
                    };
                    result.extend(post);
                }
            }
            result
        }
    }

    /// Compute the τ-closure of a state: all states reachable via τ transitions.
    pub fn tau_closure(&self, s: State) -> HashSet<State> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(s);
        while let Some(cur) = queue.pop_front() {
            for &next in &self.transitions_by_action(cur, &Action::Tau) {
                if visited.insert(next) {
                    queue.push_back(next);
                }
            }
        }
        visited
    }

    /// Get all actions that appear in the LTS.
    pub fn alphabet(&self) -> HashSet<Action> {
        self.transitions.iter().map(|t| t.action.clone()).collect()
    }

    /// Get all visible actions (non-τ).
    pub fn visible_alphabet(&self) -> HashSet<Action> {
        self.alphabet()
            .into_iter()
            .filter(|a| a.is_visible())
            .collect()
    }
}

// ─── Bisimulation ─────────────────────────────────────────────────────────────

/// A **bisimulation relation** `R ⊆ S × S` satisfying:
/// For all `(p, q) ∈ R`:
/// 1. If `p →ₐ p'`, then ∃ `q'` with `q →ₐ q'` and `(p', q') ∈ R`
/// 2. If `q →ₐ q'`, then ∃ `p'` with `p →ₐ p'` and `(p', q') ∈ R`
///
/// Bisimilarity `~` is the largest bisimulation.
#[derive(Debug, Clone)]
pub struct BisimulationRelation {
    /// Pairs `(s, t)` in the relation.
    pub pairs: HashSet<(State, State)>,
}

impl BisimulationRelation {
    /// Empty bisimulation.
    pub fn empty() -> Self {
        BisimulationRelation {
            pairs: HashSet::new(),
        }
    }

    /// Add a pair.
    pub fn add(&mut self, s: State, t: State) {
        self.pairs.insert((s, t));
    }

    /// Check if `(s, t)` is in the relation.
    pub fn contains(&self, s: State, t: State) -> bool {
        self.pairs.contains(&(s, t))
    }

    /// Size of the relation.
    pub fn size(&self) -> usize {
        self.pairs.len()
    }
}

// ─── Trace Semantics ──────────────────────────────────────────────────────────

/// A **trace** is a finite sequence of visible actions.
pub type Trace = Vec<Action>;

/// The **traces** of a process: all finite sequences of visible actions it can perform.
#[derive(Debug, Clone)]
pub struct TraceSet {
    /// The set of traces.
    pub traces: HashSet<Vec<Action>>,
}

impl TraceSet {
    /// Empty trace set (contains only the empty trace).
    pub fn new() -> Self {
        let mut traces = HashSet::new();
        traces.insert(vec![]); // empty trace always included
        TraceSet { traces }
    }

    /// Check if a trace is in the set.
    pub fn contains(&self, trace: &[Action]) -> bool {
        self.traces.contains(trace)
    }

    /// Number of traces.
    pub fn len(&self) -> usize {
        self.traces.len()
    }

    /// Whether the trace set contains no traces at all (not even the empty trace).
    pub fn is_empty(&self) -> bool {
        self.traces.is_empty()
    }

    /// Is empty (only has the empty trace).
    pub fn is_trivial(&self) -> bool {
        self.traces.len() == 1 && self.traces.contains(&vec![])
    }
}

// ─── Failures Semantics ───────────────────────────────────────────────────────

/// A **failure** is a pair `(s, X)` where `s` is a trace and `X` is a **refusal set**
/// (the set of events the process can refuse after performing `s`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Failure {
    /// The trace performed.
    pub trace: Vec<Action>,
    /// The set of actions refused after the trace (sorted for determinism).
    pub refusal: HashSet<Action>,
}

impl Failure {
    /// Create a failure pair.
    pub fn new(trace: Vec<Action>, refusal: HashSet<Action>) -> Self {
        Failure { trace, refusal }
    }
}

/// The **failures model** of a process: a set of (trace, refusal) pairs.
#[derive(Debug, Clone)]
pub struct FailuresModel {
    /// Failures (stored as Vec since Failure is not Hash-able due to HashSet field).
    pub failures: Vec<Failure>,
    /// Traces (for convenience).
    pub traces: TraceSet,
}

impl FailuresModel {
    /// Create an empty failures model.
    pub fn new() -> Self {
        FailuresModel {
            failures: Vec::new(),
            traces: TraceSet::new(),
        }
    }
}

// ─── Hennessy-Milner Logic ────────────────────────────────────────────────────

/// **Hennessy-Milner Logic (HML)** formula for describing process properties.
///
/// ```text
/// φ ::= tt                -- true (always satisfied)
///     | ff                -- false (never satisfied)
///     | φ ∧ ψ             -- conjunction
///     | φ ∨ ψ             -- disjunction
///     | ¬φ                -- negation
///     | ⟨α⟩φ              -- diamond: can do α and then satisfy φ
///     | \[α\]φ              -- box: every α-transition leads to φ
/// ```
///
/// For finitely-branching image-finite processes, HML characterizes bisimilarity:
/// `P ~ Q` iff `P` and `Q` satisfy the same HML formulas.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HmlFormula {
    /// `tt` — true.
    True,
    /// `ff` — false.
    False,
    /// `φ ∧ ψ` — conjunction.
    And(Box<HmlFormula>, Box<HmlFormula>),
    /// `φ ∨ ψ` — disjunction.
    Or(Box<HmlFormula>, Box<HmlFormula>),
    /// `¬φ` — negation.
    Not(Box<HmlFormula>),
    /// `⟨α⟩φ` — diamond modality: can perform α and then satisfy φ.
    Diamond(Action, Box<HmlFormula>),
    /// `[α]φ` — box modality: all α-transitions lead to states satisfying φ.
    Box_(Action, Box<HmlFormula>),
}

impl HmlFormula {
    /// Construct `⟨α⟩φ`.
    pub fn diamond(a: Action, phi: HmlFormula) -> Self {
        HmlFormula::Diamond(a, Box::new(phi))
    }

    /// Construct `[α]φ`.
    pub fn box_(a: Action, phi: HmlFormula) -> Self {
        HmlFormula::Box_(a, Box::new(phi))
    }

    /// Construct `φ ∧ ψ`.
    pub fn and(phi: HmlFormula, psi: HmlFormula) -> Self {
        HmlFormula::And(Box::new(phi), Box::new(psi))
    }

    /// Construct `φ ∨ ψ`.
    pub fn or(phi: HmlFormula, psi: HmlFormula) -> Self {
        HmlFormula::Or(Box::new(phi), Box::new(psi))
    }

    /// Construct `¬φ`.
    pub fn not(phi: HmlFormula) -> Self {
        HmlFormula::Not(Box::new(phi))
    }
}

impl fmt::Display for HmlFormula {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HmlFormula::True => write!(f, "tt"),
            HmlFormula::False => write!(f, "ff"),
            HmlFormula::And(p, q) => write!(f, "({} ∧ {})", p, q),
            HmlFormula::Or(p, q) => write!(f, "({} ∨ {})", p, q),
            HmlFormula::Not(p) => write!(f, "¬{}", p),
            HmlFormula::Diamond(a, p) => write!(f, "⟨{}⟩{}", a, p),
            HmlFormula::Box_(a, p) => write!(f, "[{}]{}", a, p),
        }
    }
}

// ─── Testing Equivalence ──────────────────────────────────────────────────────

/// A **test** is a CSP-like process that interacts with a process under test
/// and either **succeeds** (✓) or **fails**.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Test {
    /// Successful test.
    Success,
    /// Failing test.
    Fail,
    /// Prefix: offer event `a`, then continue with test `t`.
    Offer(String, Box<Test>),
    /// Choice: accept either event.
    TChoice(Box<Test>, Box<Test>),
}

/// Outcome of running a test against a process.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestOutcome {
    /// Test passed.
    Pass,
    /// Test failed.
    Fail,
    /// Test diverged (infinite τ-sequence).
    Diverge,
}

// ─── Process Equivalences Summary ────────────────────────────────────────────

/// The **preorder lattice** of behavioral equivalences.
///
/// From finest (strongest) to coarsest (weakest):
/// ```text
/// bisimilarity (~)
///   ⊆ 2-bisimilarity
///   ⊆ weak bisimilarity (≈)
///   ⊆ ready simulation
///   ⊆ failures equivalence
///   ⊆ trace equivalence
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BehavioralEquivalence {
    /// **Bisimilarity** (strong bisimulation): finest equivalence.
    StrongBisimilarity,
    /// **Weak bisimilarity**: ignores τ-transitions.
    WeakBisimilarity,
    /// **Branching bisimilarity**: preserves branching structure.
    BranchingBisimilarity,
    /// **Ready simulation**: can match ready sets.
    ReadySimulation,
    /// **Failures equivalence**: same (trace, refusal) pairs.
    FailuresEquivalence,
    /// **Trace equivalence**: same finite traces.
    TraceEquivalence,
}

impl fmt::Display for BehavioralEquivalence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BehavioralEquivalence::StrongBisimilarity => write!(f, "strong bisimilarity (~)"),
            BehavioralEquivalence::WeakBisimilarity => write!(f, "weak bisimilarity (≈)"),
            BehavioralEquivalence::BranchingBisimilarity => write!(f, "branching bisimilarity"),
            BehavioralEquivalence::ReadySimulation => write!(f, "ready simulation"),
            BehavioralEquivalence::FailuresEquivalence => write!(f, "failures equivalence"),
            BehavioralEquivalence::TraceEquivalence => write!(f, "trace equivalence"),
        }
    }
}

// ─── CCS Structural Congruence ────────────────────────────────────────────────

/// The **structural congruence** axioms for CCS:
///
/// - `P + 0 ≡ P`
/// - `P + Q ≡ Q + P`
/// - `(P + Q) + R ≡ P + (Q + R)`
/// - `P | 0 ≡ P`
/// - `P | Q ≡ Q | P`
/// - `(P | Q) | R ≡ P | (Q | R)`
/// - `(μX.P) ≡ P\[μX.P/X\]` (unfolding)
///
/// A process is in **structural normal form** if:
/// - No `P + 0` or `0 + P` sub-terms
/// - No nested restriction/relabeling of the same type
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructuralCongruenceClass {
    /// A canonical representative.
    pub representative: CcsProcess,
    /// Number of syntactic simplifications applied.
    pub reductions: usize,
}
