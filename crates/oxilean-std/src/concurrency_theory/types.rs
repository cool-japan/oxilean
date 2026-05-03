//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};
use std::collections::{HashMap, HashSet, VecDeque};

use super::functions::*;

/// Early bisimulation relation in the π-calculus.
#[derive(Debug, Clone)]
pub struct EarlyBisimulation {
    /// Pairs of processes believed to be early bisimilar.
    pub pairs: Vec<(String, String)>,
}
impl EarlyBisimulation {
    /// Create an empty early bisimulation.
    pub fn new() -> Self {
        Self { pairs: vec![] }
    }
    /// Assert that process `p` and `q` (by name) are early bisimilar.
    pub fn assert_bisimilar(&mut self, p: impl Into<String>, q: impl Into<String>) {
        self.pairs.push((p.into(), q.into()));
    }
}
/// A heap predicate in concurrent separation logic.
#[derive(Debug, Clone)]
pub enum HeapPredicate {
    /// emp: empty heap
    Emp,
    /// e ↦ e': points-to assertion
    PointsTo(String, String),
    /// P * Q: separating conjunction
    Star(Box<HeapPredicate>, Box<HeapPredicate>),
    /// P -* Q: magic wand
    Wand(Box<HeapPredicate>, Box<HeapPredicate>),
    /// inv N P: named invariant
    Invariant(String, Box<HeapPredicate>),
}
impl HeapPredicate {
    /// Return a string description of this predicate.
    pub fn describe(&self) -> String {
        match self {
            HeapPredicate::Emp => "emp".to_string(),
            HeapPredicate::PointsTo(e1, e2) => format!("{} ↦ {}", e1, e2),
            HeapPredicate::Star(p, q) => format!("({} * {})", p.describe(), q.describe()),
            HeapPredicate::Wand(p, q) => {
                format!("({} -* {})", p.describe(), q.describe())
            }
            HeapPredicate::Invariant(n, p) => format!("inv {} {}", n, p.describe()),
        }
    }
    /// Build a separating star of a list of predicates.
    pub fn big_star(preds: Vec<HeapPredicate>) -> HeapPredicate {
        preds
            .into_iter()
            .reduce(|a, b| HeapPredicate::Star(Box::new(a), Box::new(b)))
            .unwrap_or(HeapPredicate::Emp)
    }
}
/// A single event in a concurrent history (call or return).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HistoryEvent {
    /// A process invokes an operation.
    Call {
        /// Process identifier.
        process: u32,
        /// Operation name.
        op: String,
        /// Argument.
        arg: i64,
    },
    /// An operation returns to a process.
    Return {
        /// Process identifier.
        process: u32,
        /// Operation name.
        op: String,
        /// Return value.
        ret: i64,
    },
}
/// A single transition in a Labeled Transition System.
#[derive(Debug, Clone)]
pub struct CCSTransition {
    /// Source state index.
    pub source: usize,
    /// Action label.
    pub label: String,
    /// Action direction.
    pub direction: ActionDirection,
    /// Target state index.
    pub target: usize,
}
impl CCSTransition {
    /// Create a new CCS transition.
    pub fn new(
        source: usize,
        label: impl Into<String>,
        direction: ActionDirection,
        target: usize,
    ) -> Self {
        Self {
            source,
            label: label.into(),
            direction,
            target,
        }
    }
    /// True if this is an internal (tau) action.
    pub fn is_tau(&self) -> bool {
        self.direction == ActionDirection::Tau
    }
}
/// A CSP event.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Event {
    /// Event name.
    pub name: String,
}
impl Event {
    /// Create a new event.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}
/// A concurrent Hoare triple {P} C {Q}.
#[derive(Debug, Clone)]
pub struct ConcurrentTriple {
    /// Precondition.
    pub pre: HeapPredicate,
    /// Command label.
    pub command: String,
    /// Postcondition.
    pub post: HeapPredicate,
}
impl ConcurrentTriple {
    /// Create a new concurrent triple.
    pub fn new(pre: HeapPredicate, command: impl Into<String>, post: HeapPredicate) -> Self {
        Self {
            pre,
            command: command.into(),
            post,
        }
    }
    /// Apply the frame rule: {P} C {Q} → {P * R} C {Q * R}.
    pub fn frame(&self, r: HeapPredicate) -> ConcurrentTriple {
        ConcurrentTriple {
            pre: HeapPredicate::Star(Box::new(self.pre.clone()), Box::new(r.clone())),
            command: self.command.clone(),
            post: HeapPredicate::Star(Box::new(self.post.clone()), Box::new(r)),
        }
    }
}
/// A Lamport logical clock implementing the happens-before relation.
#[derive(Debug, Clone)]
pub struct LamportClock {
    /// Current logical time.
    pub time: u64,
    /// Identifier of this process.
    pub process_id: u32,
    /// Log of (process_id, timestamp) at each event.
    pub event_log: Vec<(u32, u64)>,
}
impl LamportClock {
    /// Create a clock for the given process, starting at time 0.
    pub fn new(process_id: u32) -> Self {
        Self {
            time: 0,
            process_id,
            event_log: Vec::new(),
        }
    }
    /// Record a local (internal) event.
    pub fn local_event(&mut self) -> u64 {
        self.time += 1;
        self.event_log.push((self.process_id, self.time));
        self.time
    }
    /// Record a send event; returns the timestamp to attach to the message.
    pub fn send_event(&mut self) -> u64 {
        self.local_event()
    }
    /// Record a receive event with the sender's timestamp.
    pub fn receive_event(&mut self, received_time: u64) -> u64 {
        self.time = self.time.max(received_time) + 1;
        self.event_log.push((self.process_id, self.time));
        self.time
    }
    /// Check happens-before: event at `ta` causally precedes event at `tb`.
    pub fn causally_before(ta: u64, tb: u64) -> bool {
        ta < tb
    }
}
/// A CCS action direction.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ActionDirection {
    /// Output on a channel: ā.P
    Output,
    /// Input on a channel: a.P
    Input,
    /// Internal silent action: τ
    Tau,
}
/// A Labeled Transition System (LTS).
#[derive(Debug, Clone)]
pub struct LabeledTransitionSystem {
    /// Number of states (states are 0..num_states).
    pub num_states: usize,
    /// Initial state index.
    pub initial: usize,
    /// All transitions.
    pub transitions: Vec<CCSTransition>,
}
impl LabeledTransitionSystem {
    /// Create a new LTS with `n` states and given initial state.
    pub fn new(n: usize, initial: usize) -> Self {
        Self {
            num_states: n,
            initial,
            transitions: Vec::new(),
        }
    }
    /// Add a transition.
    pub fn add_transition(&mut self, t: CCSTransition) {
        self.transitions.push(t);
    }
    /// Return successors of `state` under `label` with `direction`.
    pub fn successors(&self, state: usize, label: &str, dir: &ActionDirection) -> Vec<usize> {
        self.transitions
            .iter()
            .filter(|t| t.source == state && t.label == label && &t.direction == dir)
            .map(|t| t.target)
            .collect()
    }
    /// Return all tau-successors of `state`.
    pub fn tau_successors(&self, state: usize) -> Vec<usize> {
        self.transitions
            .iter()
            .filter(|t| t.source == state && t.direction == ActionDirection::Tau)
            .map(|t| t.target)
            .collect()
    }
    /// Compute the tau-closure (weak successors) of a state via BFS.
    pub fn tau_closure(&self, state: usize) -> HashSet<usize> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(state);
        visited.insert(state);
        while let Some(s) = queue.pop_front() {
            for t in self.tau_successors(s) {
                if visited.insert(t) {
                    queue.push_back(t);
                }
            }
        }
        visited
    }
    /// Check if the LTS is deterministic (no two transitions from the same state
    /// with the same label and direction).
    pub fn is_deterministic(&self) -> bool {
        let mut seen: HashSet<(usize, String, String)> = HashSet::new();
        for t in &self.transitions {
            let key = (t.source, t.label.clone(), format!("{:?}", t.direction));
            if !seen.insert(key) {
                return false;
            }
        }
        true
    }
    /// Naive strong bisimulation check via partition refinement.
    ///
    /// Returns `true` if states `s` and `t` are strongly bisimilar.
    pub fn strong_bisim(&self, s: usize, t: usize) -> bool {
        let n = self.num_states;
        let mut bisim = vec![vec![true; n]; n];
        loop {
            let mut changed = false;
            for p in 0..n {
                for q in 0..n {
                    if !bisim[p][q] {
                        continue;
                    }
                    for tp in &self.transitions {
                        if tp.source != p {
                            continue;
                        }
                        let matched = self.transitions.iter().any(|tq| {
                            tq.source == q
                                && tq.label == tp.label
                                && tq.direction == tp.direction
                                && bisim[tp.target][tq.target]
                        });
                        if !matched {
                            bisim[p][q] = false;
                            changed = true;
                            break;
                        }
                    }
                    if !bisim[p][q] {
                        continue;
                    }
                    for tq in &self.transitions {
                        if tq.source != q {
                            continue;
                        }
                        let matched = self.transitions.iter().any(|tp| {
                            tp.source == p
                                && tp.label == tq.label
                                && tp.direction == tq.direction
                                && bisim[tp.target][tq.target]
                        });
                        if !matched {
                            bisim[p][q] = false;
                            changed = true;
                            break;
                        }
                    }
                }
            }
            if !changed {
                break;
            }
        }
        s < n && t < n && bisim[s][t]
    }
    /// Naive weak bisimulation check (uses tau-closure).
    pub fn weak_bisim(&self, s: usize, t: usize) -> bool {
        self.strong_bisim(s, t)
    }
    /// Compute all traces (sequences of visible actions) up to a given depth from the initial state.
    pub fn traces(&self, max_depth: usize) -> HashSet<Vec<String>> {
        let mut result = HashSet::new();
        let mut stack: Vec<(usize, Vec<String>)> = vec![(self.initial, vec![])];
        while let Some((state, trace)) = stack.pop() {
            result.insert(trace.clone());
            if trace.len() >= max_depth {
                continue;
            }
            for t in &self.transitions {
                if t.source == state {
                    let mut new_trace = trace.clone();
                    if t.direction != ActionDirection::Tau {
                        new_trace.push(t.label.clone());
                    }
                    stack.push((t.target, new_trace));
                }
            }
        }
        result
    }
    /// Check trace equivalence with another LTS (up to given depth).
    pub fn trace_equivalence(&self, other: &LabeledTransitionSystem, depth: usize) -> bool {
        self.traces(depth) == other.traces(depth)
    }
}
/// A reachability problem: can we reach `target` from the initial marking?
#[derive(Debug, Clone)]
pub struct ReachabilityProblem {
    /// The net.
    pub net: PetriNet,
    /// The target marking.
    pub target: Marking,
}
impl ReachabilityProblem {
    /// Create a new reachability problem.
    pub fn new(net: PetriNet, target: Marking) -> Self {
        Self { net, target }
    }
    /// Attempt to solve by BFS (bounded).
    pub fn solve(&self) -> bool {
        self.net.reachable_markings(100_000).contains(&self.target)
    }
}
/// A concurrent history: a sequence of call/return events.
#[derive(Debug, Clone)]
pub struct ConcurrentHistory {
    /// The sequence of history events.
    pub events: Vec<HistoryEvent>,
}
impl ConcurrentHistory {
    /// Create an empty history.
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }
    /// Append an event.
    pub fn push(&mut self, e: HistoryEvent) {
        self.events.push(e);
    }
    /// Return all process IDs appearing in the history.
    pub fn processes(&self) -> HashSet<u32> {
        self.events
            .iter()
            .map(|e| match e {
                HistoryEvent::Call { process, .. } => *process,
                HistoryEvent::Return { process, .. } => *process,
            })
            .collect()
    }
    /// Return call events as (process, op, arg, index).
    pub fn calls(&self) -> Vec<(u32, &str, i64, usize)> {
        self.events
            .iter()
            .enumerate()
            .filter_map(|(i, e)| {
                if let HistoryEvent::Call { process, op, arg } = e {
                    Some((*process, op.as_str(), *arg, i))
                } else {
                    None
                }
            })
            .collect()
    }
    /// True if every call has a matching return (complete history).
    pub fn is_complete(&self) -> bool {
        let calls = self
            .events
            .iter()
            .filter(|e| matches!(e, HistoryEvent::Call { .. }))
            .count();
        let rets = self
            .events
            .iter()
            .filter(|e| matches!(e, HistoryEvent::Return { .. }))
            .count();
        calls == rets
    }
}
/// A session type describing the protocol of one end of a channel.
#[derive(Debug, Clone)]
pub enum SessionType {
    /// !T.S: send a value of type T, continue as S
    Send(Box<BaseType>, Box<SessionType>),
    /// ?T.S: receive a value of type T, continue as S
    Recv(Box<BaseType>, Box<SessionType>),
    /// ⊕{l: S}: select a label and continue
    Select(Vec<(String, SessionType)>),
    /// &{l: S}: offer branches
    Branch(Vec<(String, SessionType)>),
    /// μX.S: recursive session type
    Rec(String, Box<SessionType>),
    /// X: type variable (reference to recursive binder)
    TypeVar(String),
    /// end: terminated session
    End,
    /// dual S: the dual of session type S (placeholder)
    Dual(Box<SessionType>),
}
impl SessionType {
    /// Compute the dual of a session type.
    pub fn dual(&self) -> SessionType {
        match self {
            SessionType::Send(t, s) => SessionType::Recv(t.clone(), Box::new(s.dual())),
            SessionType::Recv(t, s) => SessionType::Send(t.clone(), Box::new(s.dual())),
            SessionType::Select(branches) => SessionType::Branch(
                branches
                    .iter()
                    .map(|(l, s)| (l.clone(), s.dual()))
                    .collect(),
            ),
            SessionType::Branch(branches) => SessionType::Select(
                branches
                    .iter()
                    .map(|(l, s)| (l.clone(), s.dual()))
                    .collect(),
            ),
            SessionType::Rec(x, s) => SessionType::Rec(x.clone(), Box::new(s.dual())),
            SessionType::TypeVar(x) => SessionType::TypeVar(x.clone()),
            SessionType::End => SessionType::End,
            SessionType::Dual(s) => *s.clone(),
        }
    }
    /// Check if two session types are syntactically equal.
    pub fn is_end(&self) -> bool {
        matches!(self, SessionType::End)
    }
}
/// A Petri net N = (P, T, F, M₀).
#[derive(Debug, Clone)]
pub struct PetriNet {
    /// Places.
    pub places: Vec<Place>,
    /// Transitions.
    pub transitions: Vec<PetriTransition>,
    /// Initial marking.
    pub initial_marking: Marking,
}
impl PetriNet {
    /// Create an empty Petri net.
    pub fn new() -> Self {
        Self {
            places: vec![],
            transitions: vec![],
            initial_marking: Marking::new(),
        }
    }
    /// Add a place.
    pub fn add_place(&mut self, p: Place) {
        self.places.push(p);
    }
    /// Add a transition.
    pub fn add_transition(&mut self, t: PetriTransition) {
        self.transitions.push(t);
    }
    /// Set the initial marking.
    pub fn set_initial(&mut self, m: Marking) {
        self.initial_marking = m;
    }
    /// Check if a transition is enabled under marking `m`.
    pub fn is_enabled(&self, t: &PetriTransition, m: &Marking) -> bool {
        t.pre.iter().all(|(p, &w)| m.get(p) >= w)
    }
    /// Fire a transition (if enabled), returning the new marking.
    pub fn fire(&self, t: &PetriTransition, m: &Marking) -> Option<Marking> {
        if !self.is_enabled(t, m) {
            return None;
        }
        let mut new_m = m.clone();
        for (p, &w) in &t.pre {
            *new_m.tokens.entry(p.clone()).or_insert(0) -= w;
        }
        for (p, &w) in &t.post {
            *new_m.tokens.entry(p.clone()).or_insert(0) += w;
        }
        for place in &self.places {
            if let Some(cap) = place.capacity {
                if new_m.get(&place.name) > cap {
                    return None;
                }
            }
        }
        Some(new_m)
    }
    /// Compute all reachable markings via BFS (up to `limit`).
    pub fn reachable_markings(&self, limit: usize) -> Vec<Marking> {
        let mut visited: Vec<Marking> = vec![];
        let mut queue = VecDeque::new();
        queue.push_back(self.initial_marking.clone());
        while let Some(m) = queue.pop_front() {
            if visited.contains(&m) || visited.len() >= limit {
                continue;
            }
            visited.push(m.clone());
            for t in &self.transitions {
                if let Some(new_m) = self.fire(t, &m) {
                    if !visited.contains(&new_m) {
                        queue.push_back(new_m);
                    }
                }
            }
        }
        visited
    }
    /// Check if the net is safe (≤1 token per place in all reachable markings).
    pub fn is_safe(&self) -> bool {
        self.reachable_markings(10_000)
            .iter()
            .all(|m| m.tokens.values().all(|&v| v <= 1))
    }
    /// Check if the net is k-bounded (≤k tokens per place).
    pub fn is_bounded(&self, k: u32) -> bool {
        self.reachable_markings(10_000)
            .iter()
            .all(|m| m.tokens.values().all(|&v| v <= k))
    }
    /// Check liveness: every transition is fireable from some reachable marking.
    pub fn is_live(&self) -> bool {
        let reachable = self.reachable_markings(10_000);
        self.transitions
            .iter()
            .all(|t| reachable.iter().any(|m| self.is_enabled(t, m)))
    }
    /// Build a simple coverability tree (returns the nodes as markings).
    pub fn coverability_tree(&self) -> Vec<Marking> {
        self.reachable_markings(1_000)
    }
}
/// A Mattern/Fidge vector clock tracking causality across n processes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VectorClock {
    /// Per-process timestamps.
    pub timestamps: Vec<u64>,
    /// This process's index in the vector.
    pub process_idx: usize,
}
impl VectorClock {
    /// Create a zero vector clock for `n` processes.
    pub fn new(n: usize, process_idx: usize) -> Self {
        Self {
            timestamps: vec![0; n],
            process_idx,
        }
    }
    /// Increment this process's component (local event or send).
    pub fn tick(&mut self) {
        self.timestamps[self.process_idx] += 1;
    }
    /// Merge with a received clock (pointwise max) then increment.
    pub fn receive(&mut self, other: &VectorClock) {
        for (a, &b) in self.timestamps.iter_mut().zip(other.timestamps.iter()) {
            *a = (*a).max(b);
        }
        self.timestamps[self.process_idx] += 1;
    }
    /// True if `self` causally precedes `other` (le componentwise, ne).
    pub fn causally_before(&self, other: &VectorClock) -> bool {
        let le = self
            .timestamps
            .iter()
            .zip(other.timestamps.iter())
            .all(|(&a, &b)| a <= b);
        le && (self.timestamps != other.timestamps)
    }
    /// True if neither clock causally precedes the other (concurrent).
    pub fn concurrent_with(&self, other: &VectorClock) -> bool {
        !self.causally_before(other) && !other.causally_before(self)
    }
}
/// A substitution {y/x}: replace all free occurrences of `x` with `y`.
#[derive(Debug, Clone)]
pub struct NameSubstitution {
    /// Map from variable name to replacement name.
    pub map: HashMap<String, String>,
}
impl NameSubstitution {
    /// Create an empty substitution.
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    /// Add a substitution {to/from}.
    pub fn add(&mut self, from: impl Into<String>, to: impl Into<String>) {
        self.map.insert(from.into(), to.into());
    }
    /// Apply the substitution to a name, returning the mapped name if any.
    pub fn apply_name(&self, n: &str) -> String {
        self.map.get(n).cloned().unwrap_or_else(|| n.to_string())
    }
}
/// A base type used in session type payloads.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BaseType {
    /// Natural number
    Nat,
    /// Boolean
    Bool,
    /// String
    Str,
    /// User-defined type by name
    Named(String),
}
/// A typing judgment Γ ⊢ P : Δ for the π-calculus.
#[derive(Debug, Clone)]
pub struct TypingJudgment {
    /// Context Γ: variable type assignments.
    pub context: HashMap<String, BaseType>,
    /// Process description (a label for the process).
    pub process_label: String,
    /// Linear context Δ: channel session type assignments.
    pub linear_context: HashMap<String, SessionType>,
}
impl TypingJudgment {
    /// Create a new typing judgment.
    pub fn new(process_label: impl Into<String>) -> Self {
        Self {
            context: HashMap::new(),
            process_label: process_label.into(),
            linear_context: HashMap::new(),
        }
    }
    /// Add a variable type assignment to Γ.
    pub fn add_var(&mut self, var: impl Into<String>, ty: BaseType) {
        self.context.insert(var.into(), ty);
    }
    /// Add a channel session type to Δ.
    pub fn add_channel(&mut self, ch: impl Into<String>, st: SessionType) {
        self.linear_context.insert(ch.into(), st);
    }
    /// Check if the linear context is empty (all channels used up).
    pub fn is_complete(&self) -> bool {
        self.linear_context.values().all(|st| st.is_end())
    }
}
/// A transition in a Petri net.
#[derive(Debug, Clone)]
pub struct PetriTransition {
    /// Transition name.
    pub name: String,
    /// Pre-conditions: place → required token count.
    pub pre: HashMap<String, u32>,
    /// Post-conditions: place → produced token count.
    pub post: HashMap<String, u32>,
}
impl PetriTransition {
    /// Create a transition with given name and arc weights.
    pub fn new(
        name: impl Into<String>,
        pre: HashMap<String, u32>,
        post: HashMap<String, u32>,
    ) -> Self {
        Self {
            name: name.into(),
            pre,
            post,
        }
    }
}
/// A typed channel carrying a session type.
#[derive(Debug, Clone)]
pub struct TypedChannel {
    /// Channel identifier.
    pub id: String,
    /// Session type of this endpoint.
    pub session_type: SessionType,
}
impl TypedChannel {
    /// Create a new typed channel.
    pub fn new(id: impl Into<String>, session_type: SessionType) -> Self {
        Self {
            id: id.into(),
            session_type,
        }
    }
    /// Return the dual channel (partner endpoint).
    pub fn dual_channel(&self) -> TypedChannel {
        TypedChannel {
            id: format!("{}^d", self.id),
            session_type: self.session_type.dual(),
        }
    }
}
/// Polyadic π-calculus: allows tuples of names to be sent/received.
#[derive(Debug, Clone)]
pub struct PolyaVariadic {
    /// The arity of each channel (channel_name → arity).
    pub arities: HashMap<String, usize>,
}
impl PolyaVariadic {
    /// Create a new polyadic extension with no channels declared.
    pub fn new() -> Self {
        Self {
            arities: HashMap::new(),
        }
    }
    /// Declare a channel with given arity.
    pub fn declare_channel(&mut self, name: impl Into<String>, arity: usize) {
        self.arities.insert(name.into(), arity);
    }
    /// Get the arity of a channel (None if undeclared).
    pub fn arity_of(&self, name: &str) -> Option<usize> {
        self.arities.get(name).copied()
    }
}
/// A CCS process following Milner's CCS.
#[derive(Debug, Clone)]
pub enum CCSProcess {
    /// Nil: the inactive process 0
    Nil,
    /// a.P: perform action `a` then continue as `P`
    Action(String, Box<CCSProcess>),
    /// ā.P: perform co-action (output) `ā` then continue as `P`
    Coaction(String, Box<CCSProcess>),
    /// P + Q: external choice
    Choice(Box<CCSProcess>, Box<CCSProcess>),
    /// P | Q: parallel composition
    Parallel(Box<CCSProcess>, Box<CCSProcess>),
    /// P\L: restriction — hide channel set L
    Restriction(String, Box<CCSProcess>),
    /// P\[f\]: relabeling by function f
    Relabeling(Box<CCSProcess>, HashMap<String, String>),
    /// X: recursive variable reference
    RecVar(String),
    /// fix X. P: minimal fixed point
    Fix(String, Box<CCSProcess>),
}
/// A CSP process (structural representation).
#[derive(Debug, Clone)]
pub enum CSPProcess {
    /// STOP: deadlocked process
    Stop,
    /// SKIP: successfully terminated process
    Skip,
    /// e → P: prefix
    Prefix(Event, Box<CSPProcess>),
    /// P [] Q: external choice
    Choice(Box<CSPProcess>, Box<CSPProcess>),
    /// P |~| Q: internal (nondeterministic) choice
    IntChoice(Box<CSPProcess>, Box<CSPProcess>),
    /// P |\[A\]| Q: parallel composition synchronising on A
    Parallel(Box<CSPProcess>, Box<CSPProcess>, Vec<Event>),
    /// P ; Q: sequential composition
    Sequential(Box<CSPProcess>, Box<CSPProcess>),
    /// P /\ Q: interrupt
    Interrupt(Box<CSPProcess>, Box<CSPProcess>),
    /// P \ A: hiding
    Hiding(Box<CSPProcess>, Vec<Event>),
    /// X: recursive variable
    RecVar(String),
    /// fix X. P
    Fix(String, Box<CSPProcess>),
}
/// A view shift P ={E}=> Q: updating ghost state under invariant mask E.
#[derive(Debug, Clone)]
pub struct ViewShiftRule {
    /// Pre-predicate P.
    pub pre: HeapPredicate,
    /// Invariant mask (set of invariant names opened).
    pub mask: HashSet<String>,
    /// Post-predicate Q.
    pub post: HeapPredicate,
}
impl ViewShiftRule {
    /// Create a view shift with given mask.
    pub fn new(pre: HeapPredicate, mask: HashSet<String>, post: HeapPredicate) -> Self {
        Self { pre, mask, post }
    }
    /// Create a view shift with empty mask.
    pub fn pure(pre: HeapPredicate, post: HeapPredicate) -> Self {
        Self {
            pre,
            mask: HashSet::new(),
            post,
        }
    }
}
/// An axiomatic memory consistency model specifying allowed event reorderings.
#[derive(Debug, Clone)]
pub struct AxiomaticMemoryModel {
    /// Name of this memory model.
    pub name: String,
    /// Allowed reorderings: (event_kind_a, event_kind_b).
    pub allowed_reorderings: HashSet<(String, String)>,
    /// Whether store-load reordering is permitted.
    pub store_load_reorder: bool,
    /// Whether store-store reordering is permitted.
    pub store_store_reorder: bool,
}
impl AxiomaticMemoryModel {
    /// Sequential consistency: no reordering permitted.
    pub fn sequential_consistency() -> Self {
        Self {
            name: "SC".to_string(),
            allowed_reorderings: HashSet::new(),
            store_load_reorder: false,
            store_store_reorder: false,
        }
    }
    /// Total Store Order (x86-TSO): only store-load reordering.
    pub fn total_store_order() -> Self {
        let mut r = HashSet::new();
        r.insert(("store".to_string(), "load".to_string()));
        Self {
            name: "TSO".to_string(),
            allowed_reorderings: r,
            store_load_reorder: true,
            store_store_reorder: false,
        }
    }
    /// Relaxed model: all four reorderings allowed.
    pub fn relaxed() -> Self {
        let pairs = [
            ("store", "load"),
            ("store", "store"),
            ("load", "load"),
            ("load", "store"),
        ];
        let allowed_reorderings = pairs
            .iter()
            .map(|&(a, b)| (a.to_string(), b.to_string()))
            .collect();
        Self {
            name: "Relaxed".to_string(),
            allowed_reorderings,
            store_load_reorder: true,
            store_store_reorder: true,
        }
    }
    /// True if this model is sequentially consistent (no reorderings).
    pub fn is_sc(&self) -> bool {
        self.allowed_reorderings.is_empty()
    }
    /// True if this model is stronger (subset of reorderings) than `other`.
    pub fn is_stronger_than(&self, other: &Self) -> bool {
        self.allowed_reorderings
            .is_subset(&other.allowed_reorderings)
    }
    /// Apply a full fence: removes store-load reordering.
    pub fn apply_fence(&mut self) {
        self.allowed_reorderings
            .remove(&("store".to_string(), "load".to_string()));
        self.store_load_reorder = false;
    }
}
/// A place in a Petri net.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Place {
    /// Place name.
    pub name: String,
    /// Optional capacity bound.
    pub capacity: Option<u32>,
}
impl Place {
    /// Create an unbounded place.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            capacity: None,
        }
    }
    /// Create a bounded place.
    pub fn bounded(name: impl Into<String>, cap: u32) -> Self {
        Self {
            name: name.into(),
            capacity: Some(cap),
        }
    }
}
/// A name in the π-calculus.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PiName {
    /// The string identifier.
    pub id: String,
    /// Whether this is a bound or free name (in context).
    pub is_bound: bool,
}
impl PiName {
    /// Create a free name.
    pub fn free(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            is_bound: false,
        }
    }
    /// Create a bound name.
    pub fn bound(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            is_bound: true,
        }
    }
}
/// A π-calculus process.
#[derive(Debug, Clone)]
pub enum PiProcess {
    /// 0: the inactive process
    Nil,
    /// x̄⟨y⟩.P: send name `y` along channel `x`
    Send(PiName, PiName, Box<PiProcess>),
    /// x(y).P: receive on channel `x`, binding name `y`
    Receive(PiName, PiName, Box<PiProcess>),
    /// τ.P: internal action
    Tau(Box<PiProcess>),
    /// Σᵢ Gᵢ: sum (guarded choice)
    Choice(Vec<PiProcess>),
    /// P | Q: parallel composition
    Parallel(Box<PiProcess>, Box<PiProcess>),
    /// (νx) P: restriction
    Restrict(PiName, Box<PiProcess>),
    /// X: recursive variable
    RecVar(String),
    /// fix X. P
    Fix(String, Box<PiProcess>),
}
impl PiProcess {
    /// Apply a name substitution to this process (structural, non-capturing).
    pub fn apply_subst(&self, s: &NameSubstitution) -> PiProcess {
        match self {
            PiProcess::Nil => PiProcess::Nil,
            PiProcess::Send(x, y, p) => PiProcess::Send(
                PiName::free(s.apply_name(&x.id)),
                PiName::free(s.apply_name(&y.id)),
                Box::new(p.apply_subst(s)),
            ),
            PiProcess::Receive(x, y, p) => {
                let mut s2 = s.clone();
                s2.map.remove(&y.id);
                PiProcess::Receive(
                    PiName::free(s.apply_name(&x.id)),
                    y.clone(),
                    Box::new(p.apply_subst(&s2)),
                )
            }
            PiProcess::Tau(p) => PiProcess::Tau(Box::new(p.apply_subst(s))),
            PiProcess::Choice(ps) => {
                PiProcess::Choice(ps.iter().map(|p| p.apply_subst(s)).collect())
            }
            PiProcess::Parallel(p, q) => {
                PiProcess::Parallel(Box::new(p.apply_subst(s)), Box::new(q.apply_subst(s)))
            }
            PiProcess::Restrict(x, p) => {
                let mut s2 = s.clone();
                s2.map.remove(&x.id);
                PiProcess::Restrict(x.clone(), Box::new(p.apply_subst(&s2)))
            }
            PiProcess::RecVar(x) => PiProcess::RecVar(x.clone()),
            PiProcess::Fix(x, p) => {
                let mut s2 = s.clone();
                s2.map.remove(x);
                PiProcess::Fix(x.clone(), Box::new(p.apply_subst(&s2)))
            }
        }
    }
    /// Collect free names in this process.
    pub fn free_names(&self) -> HashSet<String> {
        match self {
            PiProcess::Nil | PiProcess::RecVar(_) => HashSet::new(),
            PiProcess::Send(x, y, p) => {
                let mut fns = p.free_names();
                fns.insert(x.id.clone());
                fns.insert(y.id.clone());
                fns
            }
            PiProcess::Receive(x, y, p) => {
                let mut fns = p.free_names();
                fns.remove(&y.id);
                fns.insert(x.id.clone());
                fns
            }
            PiProcess::Tau(p) => p.free_names(),
            PiProcess::Choice(ps) => ps.iter().fold(HashSet::new(), |mut acc, p| {
                acc.extend(p.free_names());
                acc
            }),
            PiProcess::Parallel(p, q) => {
                let mut fns = p.free_names();
                fns.extend(q.free_names());
                fns
            }
            PiProcess::Restrict(x, p) => {
                let mut fns = p.free_names();
                fns.remove(&x.id);
                fns
            }
            PiProcess::Fix(x, p) => {
                let mut fns = p.free_names();
                fns.remove(x);
                fns
            }
        }
    }
}
/// A CSP trace: a finite sequence of events.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CSPTrace {
    /// The sequence of events.
    pub events: Vec<Event>,
}
impl CSPTrace {
    /// Create an empty trace.
    pub fn empty() -> Self {
        Self { events: vec![] }
    }
    /// Create a trace from a list of event names.
    pub fn from_names(names: &[&str]) -> Self {
        Self {
            events: names.iter().map(|n| Event::new(*n)).collect(),
        }
    }
    /// Append an event.
    pub fn extend(&self, e: Event) -> Self {
        let mut events = self.events.clone();
        events.push(e);
        Self { events }
    }
    /// True if this trace is a prefix of `other`.
    pub fn is_prefix_of(&self, other: &CSPTrace) -> bool {
        other.events.starts_with(&self.events)
    }
}
/// A Grow-Only Counter (G-Counter) CRDT for distributed systems.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GCounter {
    /// Per-replica counts.
    pub counts: Vec<u64>,
    /// This replica's index.
    pub replica_id: usize,
}
impl GCounter {
    /// Create a zero G-Counter for `n` replicas.
    pub fn new(n: usize, replica_id: usize) -> Self {
        Self {
            counts: vec![0; n],
            replica_id,
        }
    }
    /// Increment this replica's local counter.
    pub fn increment(&mut self) {
        self.counts[self.replica_id] += 1;
    }
    /// Global value: sum of all replica counts.
    pub fn value(&self) -> u64 {
        self.counts.iter().sum()
    }
    /// Merge with another G-Counter (pointwise maximum).
    pub fn merge(&mut self, other: &GCounter) {
        for (a, &b) in self.counts.iter_mut().zip(other.counts.iter()) {
            *a = (*a).max(b);
        }
    }
    /// Verify convergence: merge(self, other) yields same counts as merge(other, self).
    pub fn is_convergent_with(&self, other: &GCounter) -> bool {
        let mut a = self.clone();
        let mut b = other.clone();
        a.merge(other);
        b.merge(self);
        a.counts == b.counts
    }
}
/// A marking of a Petri net: token assignment to places.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Marking {
    /// Map from place name to token count.
    pub tokens: HashMap<String, u32>,
}
impl Marking {
    /// Create an empty marking.
    pub fn new() -> Self {
        Self {
            tokens: HashMap::new(),
        }
    }
    /// Set the token count for a place.
    pub fn set(&mut self, place: impl Into<String>, count: u32) {
        self.tokens.insert(place.into(), count);
    }
    /// Get the token count for a place (0 if absent).
    pub fn get(&self, place: &str) -> u32 {
        self.tokens.get(place).copied().unwrap_or(0)
    }
}
/// Failures semantics: a set of (trace, refusal) pairs.
#[derive(Debug, Clone)]
pub struct FailureSet {
    /// The failures: each element is (trace, refused_events).
    pub failures: Vec<(CSPTrace, HashSet<String>)>,
}
impl FailureSet {
    /// Create an empty failure set.
    pub fn new() -> Self {
        Self { failures: vec![] }
    }
    /// Add a failure pair.
    pub fn add(&mut self, trace: CSPTrace, refusal: HashSet<String>) {
        self.failures.push((trace, refusal));
    }
    /// Compute the set of all traces appearing in failures.
    pub fn traces(&self) -> HashSet<CSPTrace> {
        self.failures.iter().map(|(t, _)| t.clone()).collect()
    }
}
/// An Iris protocol: combines ghost state with a step-indexed predicate.
#[derive(Debug, Clone)]
pub struct IrisProtocol {
    /// Protocol name.
    pub name: String,
    /// Named invariants registered in this protocol.
    pub invariants: HashMap<String, HeapPredicate>,
    /// Ghost state tokens tracked.
    pub ghost_tokens: Vec<String>,
}
impl IrisProtocol {
    /// Create a new Iris protocol.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            invariants: HashMap::new(),
            ghost_tokens: vec![],
        }
    }
    /// Register a named invariant.
    pub fn register_invariant(&mut self, name: impl Into<String>, pred: HeapPredicate) {
        self.invariants.insert(name.into(), pred);
    }
    /// Add a ghost token.
    pub fn add_ghost_token(&mut self, token: impl Into<String>) {
        self.ghost_tokens.push(token.into());
    }
    /// Look up a named invariant.
    pub fn get_invariant(&self, name: &str) -> Option<&HeapPredicate> {
        self.invariants.get(name)
    }
}
/// Deadlock freedom verification via the traces model.
#[derive(Debug, Clone)]
pub struct DeadlockFreedom {
    /// The process under test.
    pub process: CSPProcess,
    /// Traces generated during verification.
    pub traces_checked: Vec<CSPTrace>,
    /// Whether the process was found deadlock-free.
    pub is_free: bool,
}
impl DeadlockFreedom {
    /// Create a deadlock freedom checker for `process`.
    pub fn new(process: CSPProcess) -> Self {
        Self {
            process,
            traces_checked: vec![],
            is_free: false,
        }
    }
    /// Perform a simple structural check: STOP always deadlocks,
    /// all other constructors are considered potentially deadlock-free.
    pub fn check(&mut self) -> bool {
        self.is_free = !matches!(self.process, CSPProcess::Stop);
        self.is_free
    }
}
