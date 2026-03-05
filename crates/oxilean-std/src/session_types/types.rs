//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use std::collections::{HashMap, HashSet, VecDeque};

/// A global session type (multiparty protocol specification).
#[derive(Debug, Clone)]
pub enum GType {
    /// `p → q : T; G` — p sends T to q, continues as G.
    Comm {
        /// Sender role.
        sender: Role,
        /// Receiver role.
        receiver: Role,
        /// Communicated type.
        msg_ty: BaseType,
        /// Continuation.
        cont: Box<GType>,
    },
    /// `p → q { branch₁ : G₁, branch₂ : G₂ }` — p selects a branch to q.
    Choice {
        /// The role that makes the choice.
        selector: Role,
        /// The role that receives the choice.
        receiver: Role,
        /// Named branches (label → continuation).
        branches: HashMap<String, GType>,
    },
    /// Protocol end.
    End,
    /// Recursive global type: `μX.G`.
    Rec(String, Box<GType>),
    /// Recursion variable.
    Var(String),
}
impl GType {
    /// Collect all roles participating in this global type.
    pub fn participants(&self) -> HashSet<Role> {
        let mut roles = HashSet::new();
        self.collect_roles(&mut roles);
        roles
    }
    fn collect_roles(&self, roles: &mut HashSet<Role>) {
        match self {
            GType::Comm {
                sender,
                receiver,
                cont,
                ..
            } => {
                roles.insert(sender.clone());
                roles.insert(receiver.clone());
                cont.collect_roles(roles);
            }
            GType::Choice {
                selector,
                receiver,
                branches,
            } => {
                roles.insert(selector.clone());
                roles.insert(receiver.clone());
                for cont in branches.values() {
                    cont.collect_roles(roles);
                }
            }
            GType::End | GType::Var(_) => {}
            GType::Rec(_, body) => body.collect_roles(roles),
        }
    }
    /// Project the global type onto a specific role.
    pub fn project(&self, role: &Role) -> LType {
        match self {
            GType::Comm {
                sender,
                receiver,
                msg_ty,
                cont,
            } => {
                let cont_proj = cont.project(role);
                if sender == role {
                    LType::Send(receiver.clone(), msg_ty.clone(), Box::new(cont_proj))
                } else if receiver == role {
                    LType::Recv(sender.clone(), msg_ty.clone(), Box::new(cont_proj))
                } else {
                    cont_proj
                }
            }
            GType::Choice {
                selector,
                receiver,
                branches,
            } => {
                if selector == role {
                    let mut proj_branches: Vec<(String, LType)> = branches
                        .iter()
                        .map(|(lbl, g)| (lbl.clone(), g.project(role)))
                        .collect();
                    proj_branches.sort_by(|a, b| a.0.cmp(&b.0));
                    LType::IChoice(receiver.clone(), proj_branches)
                } else if receiver == role {
                    let mut proj_branches: Vec<(String, LType)> = branches
                        .iter()
                        .map(|(lbl, g)| (lbl.clone(), g.project(role)))
                        .collect();
                    proj_branches.sort_by(|a, b| a.0.cmp(&b.0));
                    LType::EChoice(selector.clone(), proj_branches)
                } else {
                    let projs: Vec<LType> = branches.values().map(|g| g.project(role)).collect();
                    Self::merge_all(projs)
                }
            }
            GType::End => LType::End,
            GType::Rec(x, body) => LType::Rec(x.clone(), Box::new(body.project(role))),
            GType::Var(x) => LType::Var(x.clone()),
        }
    }
    fn merge_all(types: Vec<LType>) -> LType {
        types
            .into_iter()
            .reduce(|a, b| if a == b { a } else { b })
            .unwrap_or(LType::End)
    }
}
/// An asynchronous session endpoint that buffers outgoing messages.
///
/// Unlike `SessionEndpoint`, sends do not block: they enqueue into a local
/// outbox. The peer drains the outbox into its inbox when it is ready to
/// receive, modelling asynchronous / buffered channels.
pub struct AsyncSessionEndpoint {
    /// The remaining (un-consumed) session type.
    pub remaining: SType,
    /// Outgoing messages waiting to be delivered.
    outbox: VecDeque<Message>,
    /// Incoming messages ready to be received.
    inbox: VecDeque<Message>,
}
impl AsyncSessionEndpoint {
    /// Create a new async endpoint with the given session type.
    pub fn new(stype: SType) -> Self {
        AsyncSessionEndpoint {
            remaining: stype,
            outbox: VecDeque::new(),
            inbox: VecDeque::new(),
        }
    }
    /// Asynchronously send a message (enqueue without blocking).
    pub fn async_send(&mut self, msg: Message) -> Result<(), String> {
        match &self.remaining.clone() {
            SType::Send(_, cont) => {
                self.remaining = *cont.clone();
                self.outbox.push_back(msg);
                Ok(())
            }
            other => Err(format!("AsyncSend: expected Send, got {}", other)),
        }
    }
    /// Deliver all outgoing messages into the peer's inbox.
    /// Returns the number of messages flushed.
    pub fn flush_to(&mut self, peer: &mut AsyncSessionEndpoint) -> usize {
        let count = self.outbox.len();
        while let Some(msg) = self.outbox.pop_front() {
            peer.inbox.push_back(msg);
        }
        count
    }
    /// Receive a message from the inbox (must have been delivered by the peer).
    pub fn async_recv(&mut self) -> Result<Message, String> {
        match &self.remaining.clone() {
            SType::Recv(_, cont) => {
                if let Some(msg) = self.inbox.pop_front() {
                    self.remaining = *cont.clone();
                    Ok(msg)
                } else {
                    Err("AsyncRecv: inbox empty — message not yet delivered".to_string())
                }
            }
            other => Err(format!("AsyncRecv: expected Recv, got {}", other)),
        }
    }
    /// Number of messages waiting in the outbox (not yet delivered).
    pub fn outbox_len(&self) -> usize {
        self.outbox.len()
    }
    /// Number of messages waiting in the inbox (delivered but not yet consumed).
    pub fn inbox_len(&self) -> usize {
        self.inbox.len()
    }
}
/// A role in a multiparty session.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Role(pub String);
impl Role {
    /// Create a new role.
    pub fn new(name: impl Into<String>) -> Self {
        Role(name.into())
    }
}
/// A builder for constructing session types fluently.
pub struct ProtocolBuilder {
    current: SType,
}
impl ProtocolBuilder {
    /// Start with the `End` session.
    pub fn end() -> Self {
        ProtocolBuilder {
            current: SType::End,
        }
    }
    /// Prepend a send operation.
    pub fn then_send(self, ty: BaseType) -> Self {
        ProtocolBuilder {
            current: SType::Send(Box::new(ty), Box::new(self.current)),
        }
    }
    /// Prepend a receive operation.
    pub fn then_recv(self, ty: BaseType) -> Self {
        ProtocolBuilder {
            current: SType::Recv(Box::new(ty), Box::new(self.current)),
        }
    }
    /// Finalize and return the session type.
    pub fn build(self) -> SType {
        self.current
    }
}
/// A binary session type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SType {
    /// `!T.S` — send T then continue as S.
    Send(Box<BaseType>, Box<SType>),
    /// `?T.S` — receive T then continue as S.
    Recv(Box<BaseType>, Box<SType>),
    /// Session termination.
    End,
    /// Internal choice: `S₁ ⊕ S₂`.
    Choice(Box<SType>, Box<SType>),
    /// External choice: `S₁ & S₂`.
    Branch(Box<SType>, Box<SType>),
    /// Recursive session type: `μX.S`.
    Rec(String, Box<SType>),
    /// Session type variable.
    Var(String),
}
impl SType {
    /// Compute the dual session type.
    pub fn dual(&self) -> SType {
        match self {
            SType::Send(t, s) => SType::Recv(t.clone(), Box::new(s.dual())),
            SType::Recv(t, s) => SType::Send(t.clone(), Box::new(s.dual())),
            SType::End => SType::End,
            SType::Choice(s1, s2) => SType::Branch(Box::new(s1.dual()), Box::new(s2.dual())),
            SType::Branch(s1, s2) => SType::Choice(Box::new(s1.dual()), Box::new(s2.dual())),
            SType::Rec(x, s) => SType::Rec(x.clone(), Box::new(s.dual())),
            SType::Var(x) => SType::Var(x.clone()),
        }
    }
    /// Unfold a recursive session type one step.
    pub fn unfold(&self) -> SType {
        match self {
            SType::Rec(x, body) => {
                let mut body = (**body).clone();
                body.subst_var(x, self);
                body
            }
            other => other.clone(),
        }
    }
    /// Substitute variable `x` with `replacement` in `self`.
    fn subst_var(&mut self, x: &str, replacement: &SType) {
        match self {
            SType::Send(_, s) | SType::Recv(_, s) => s.subst_var(x, replacement),
            SType::End => {}
            SType::Choice(s1, s2) | SType::Branch(s1, s2) => {
                s1.subst_var(x, replacement);
                s2.subst_var(x, replacement);
            }
            SType::Rec(y, s) => {
                if y != x {
                    s.subst_var(x, replacement);
                }
            }
            SType::Var(y) => {
                if y == x {
                    *self = replacement.clone();
                }
            }
        }
    }
    /// Check if this session type is `End`.
    pub fn is_end(&self) -> bool {
        matches!(self, SType::End)
    }
    /// Check if this is a send type.
    pub fn is_send(&self) -> bool {
        matches!(self, SType::Send(_, _))
    }
    /// Check if this is a receive type.
    pub fn is_recv(&self) -> bool {
        matches!(self, SType::Recv(_, _))
    }
}
/// A base type that can be communicated over a session.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BaseType {
    /// Natural numbers.
    Nat,
    /// Booleans.
    Bool,
    /// Strings.
    Str,
    /// Unit type.
    Unit,
    /// A named type.
    Named(String),
    /// Pair type.
    Pair(Box<BaseType>, Box<BaseType>),
    /// Sum type.
    Sum(Box<BaseType>, Box<BaseType>),
}
/// A Markov-chain protocol simulator: makes probabilistic choices at each
/// `Choice` node according to supplied weights.
pub struct ProbSessionScheduler {
    /// The per-choice weights.  Key = choice depth (simple approximation).
    branches: Vec<ProbBranch>,
}
impl ProbSessionScheduler {
    /// Create a new scheduler with the given branches.
    pub fn new(branches: Vec<ProbBranch>) -> Self {
        ProbSessionScheduler { branches }
    }
    /// Normalise the weights to proper probabilities that sum to 1.
    pub fn probabilities(&self) -> Vec<f64> {
        let total: f64 = self.branches.iter().map(|b| b.weight).sum();
        if total == 0.0 {
            return vec![0.0; self.branches.len()];
        }
        self.branches.iter().map(|b| b.weight / total).collect()
    }
    /// Sample a branch index deterministically by taking the one with the
    /// highest weight (useful for testing without a random source).
    pub fn greedy_choice(&self) -> Option<usize> {
        self.branches
            .iter()
            .enumerate()
            .max_by(|a, b| {
                a.1.weight
                    .partial_cmp(&b.1.weight)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(idx, _)| idx)
    }
    /// Expected number of rounds: sum of (prob × cost) for each branch,
    /// where cost is approximated as 1 for non-End, 0 for End.
    pub fn expected_rounds(&self) -> f64 {
        let probs = self.probabilities();
        probs
            .iter()
            .zip(self.branches.iter())
            .map(|(p, b)| {
                let cost = if b.cont == SType::End { 0.0 } else { 1.0 };
                p * cost
            })
            .sum()
    }
}
/// A message that can be sent or received on a session channel.
#[derive(Debug, Clone)]
pub enum Message {
    /// A natural number.
    Nat(u64),
    /// A boolean.
    Bool(bool),
    /// A string.
    Str(String),
    /// Unit.
    Unit,
    /// A left injection (choice/sum).
    Left,
    /// A right injection (choice/sum).
    Right,
}
/// Behavioral subtype checker for session types.
///
/// Implements the standard coinductive subtype relation:
/// - `Send T S₁ <: Send T S₂` when `S₁ <: S₂` (covariant send)
/// - `Recv T S₁ <: Recv T S₂` when `S₁ <: S₂` (covariant receive)
/// - `Choice S₁ S₂ <: Choice T₁ T₂` when `S₁ <: T₁` and `S₂ <: T₂`
/// - `Branch S₁ S₂ <: Branch T₁ T₂` when `S₁ <: T₁` and `S₂ <: T₂`
/// - `End <: End`
pub struct SessionSubtypeChecker {
    /// Pairs already decided (for coinductive checking of recursive types).
    decided: HashSet<(String, String)>,
}
impl SessionSubtypeChecker {
    /// Create a new subtype checker.
    pub fn new() -> Self {
        SessionSubtypeChecker {
            decided: HashSet::new(),
        }
    }
    /// Check whether `sub <: sup` holds under the behavioral subtype relation.
    pub fn is_subtype(&mut self, sub: &SType, sup: &SType) -> bool {
        let key = (format!("{}", sub), format!("{}", sup));
        if self.decided.contains(&key) {
            return true;
        }
        self.decided.insert(key);
        match (sub, sup) {
            (SType::End, SType::End) => true,
            (SType::Send(t1, s1), SType::Send(t2, s2)) => t1 == t2 && self.is_subtype(s1, s2),
            (SType::Recv(t1, s1), SType::Recv(t2, s2)) => t1 == t2 && self.is_subtype(s1, s2),
            (SType::Choice(l1, r1), SType::Choice(l2, r2)) => {
                self.is_subtype(l1, l2) && self.is_subtype(r1, r2)
            }
            (SType::Branch(l1, r1), SType::Branch(l2, r2)) => {
                self.is_subtype(l1, l2) && self.is_subtype(r1, r2)
            }
            (SType::Rec(_, _), _) => self.is_subtype(&sub.unfold(), sup),
            (_, SType::Rec(_, _)) => self.is_subtype(sub, &sup.unfold()),
            _ => false,
        }
    }
}
/// An action step extracted from a global type during choreography execution.
#[derive(Debug, Clone)]
pub enum ChoreographyStep {
    /// `sender` sends a message of type `msg_ty` to `receiver`.
    Comm {
        /// The sending role.
        sender: String,
        /// The receiving role.
        receiver: String,
        /// A description of the communicated type.
        msg_ty: String,
    },
    /// A choice is made by `selector`, communicated to `receiver`.
    Choice {
        /// The role making the selection.
        selector: String,
        /// The role receiving the label.
        receiver: String,
        /// The selected branch label.
        branch: String,
    },
    /// The protocol has ended.
    End,
}
/// A choreography engine that steps through a `GType` and emits actions.
pub struct ChoreographyEngine {
    /// Sequence of steps emitted so far.
    pub trace: Vec<ChoreographyStep>,
}
impl ChoreographyEngine {
    /// Create a new engine with an empty trace.
    pub fn new() -> Self {
        ChoreographyEngine { trace: vec![] }
    }
    /// Execute a global type to completion, recording all communication steps.
    /// Returns `Err` if a recursion variable is encountered without binding
    /// (unfolding must be done before calling this method).
    pub fn execute(&mut self, gtype: &GType) -> Result<(), String> {
        match gtype {
            GType::Comm {
                sender,
                receiver,
                msg_ty,
                cont,
            } => {
                self.trace.push(ChoreographyStep::Comm {
                    sender: sender.0.clone(),
                    receiver: receiver.0.clone(),
                    msg_ty: format!("{}", msg_ty),
                });
                self.execute(cont)
            }
            GType::Choice {
                selector,
                receiver,
                branches,
            } => {
                let mut sorted: Vec<(&String, &GType)> = branches.iter().collect();
                sorted.sort_by_key(|(k, _)| k.as_str());
                if let Some((label, cont)) = sorted.first() {
                    self.trace.push(ChoreographyStep::Choice {
                        selector: selector.0.clone(),
                        receiver: receiver.0.clone(),
                        branch: (*label).clone(),
                    });
                    self.execute(cont)
                } else {
                    Err("GType::Choice has no branches".to_string())
                }
            }
            GType::End => {
                self.trace.push(ChoreographyStep::End);
                Ok(())
            }
            GType::Rec(_, body) => self.execute(body),
            GType::Var(x) => Err(format!("Unresolved recursion variable: {}", x)),
        }
    }
    /// Return the number of communication actions (excluding the final End).
    pub fn comm_count(&self) -> usize {
        self.trace
            .iter()
            .filter(|s| !matches!(s, ChoreographyStep::End))
            .count()
    }
}
/// A session channel endpoint, consuming the session type as it is used.
///
/// Note: In Rust we cannot enforce linearity at the type level without a
/// linear type system; this struct instead tracks usage dynamically.
pub struct SessionEndpoint {
    /// The remaining session type (what is yet to be done).
    pub remaining: SType,
    /// The communication buffer (incoming messages).
    buffer: VecDeque<Message>,
    /// Whether the session has been closed.
    closed: bool,
}
impl SessionEndpoint {
    /// Create a new session endpoint with the given initial session type.
    pub fn new(stype: SType) -> Self {
        SessionEndpoint {
            remaining: stype,
            buffer: VecDeque::new(),
            closed: false,
        }
    }
    /// Send a message, advancing the session type.
    pub fn send(&mut self, msg: Message) -> Result<(), String> {
        match &self.remaining.clone() {
            SType::Send(_, continuation) => {
                self.remaining = *continuation.clone();
                self.buffer.push_back(msg);
                Ok(())
            }
            other => Err(format!("Expected Send, got {}", other)),
        }
    }
    /// Receive a message from the buffer, advancing the session type.
    pub fn recv(&mut self) -> Result<Message, String> {
        match &self.remaining.clone() {
            SType::Recv(_, continuation) => {
                if let Some(msg) = self.buffer.pop_front() {
                    self.remaining = *continuation.clone();
                    Ok(msg)
                } else {
                    Err("No message available".to_string())
                }
            }
            other => Err(format!("Expected Recv, got {}", other)),
        }
    }
    /// Select the left branch of an internal choice.
    pub fn select_left(&mut self) -> Result<(), String> {
        match &self.remaining.clone() {
            SType::Choice(left, _) => {
                self.remaining = *left.clone();
                Ok(())
            }
            other => Err(format!("Expected Choice, got {}", other)),
        }
    }
    /// Select the right branch of an internal choice.
    pub fn select_right(&mut self) -> Result<(), String> {
        match &self.remaining.clone() {
            SType::Choice(_, right) => {
                self.remaining = *right.clone();
                Ok(())
            }
            other => Err(format!("Expected Choice, got {}", other)),
        }
    }
    /// Close the session.
    pub fn close(&mut self) -> Result<(), String> {
        if self.remaining == SType::End {
            self.closed = true;
            Ok(())
        } else {
            Err(format!("Expected End, got {}", self.remaining))
        }
    }
    /// Check if the session is complete.
    pub fn is_complete(&self) -> bool {
        self.closed
    }
}
/// A probabilistic session choice, picking a branch by weighted probability.
///
/// Each branch has a label, a weight (non-negative), and a continuation type.
/// The scheduler normalises weights to obtain probabilities and samples a branch.
#[allow(clippy::too_many_arguments)]
pub struct ProbBranch {
    /// Human-readable label for the branch.
    pub label: String,
    /// Relative weight (unnormalised); must be positive.
    pub weight: f64,
    /// Continuation session type if this branch is chosen.
    pub cont: SType,
}
/// A simple graph-based deadlock freedom checker.
///
/// Represents the communication graph of a system of sessions and checks
/// for cycles (which would indicate potential deadlocks).
pub struct DeadlockChecker {
    /// Edges: (channel_name, role_a, role_b) — role_a waits for role_b on channel.
    wait_edges: Vec<(String, String, String)>,
}
impl DeadlockChecker {
    /// Create a new deadlock checker.
    pub fn new() -> Self {
        DeadlockChecker { wait_edges: vec![] }
    }
    /// Add a wait edge: `waiter` waits for `provider` on `channel`.
    pub fn add_wait(
        &mut self,
        channel: impl Into<String>,
        waiter: impl Into<String>,
        provider: impl Into<String>,
    ) {
        self.wait_edges
            .push((channel.into(), waiter.into(), provider.into()));
    }
    /// Check if the wait graph is cycle-free (deadlock-free).
    pub fn is_deadlock_free(&self) -> bool {
        let mut adj: HashMap<&str, Vec<&str>> = HashMap::new();
        for (_, waiter, provider) in &self.wait_edges {
            adj.entry(waiter.as_str())
                .or_default()
                .push(provider.as_str());
        }
        let mut visited: HashSet<&str> = HashSet::new();
        let mut in_stack: HashSet<&str> = HashSet::new();
        let nodes: Vec<&str> = adj.keys().copied().collect();
        for &node in &nodes {
            if !visited.contains(node) && Self::has_cycle(node, &adj, &mut visited, &mut in_stack) {
                return false;
            }
        }
        true
    }
    fn has_cycle<'a>(
        node: &'a str,
        adj: &HashMap<&'a str, Vec<&'a str>>,
        visited: &mut HashSet<&'a str>,
        in_stack: &mut HashSet<&'a str>,
    ) -> bool {
        visited.insert(node);
        in_stack.insert(node);
        if let Some(neighbors) = adj.get(node) {
            for &nb in neighbors {
                if !visited.contains(nb) {
                    if Self::has_cycle(nb, adj, visited, in_stack) {
                        return true;
                    }
                } else if in_stack.contains(nb) {
                    return true;
                }
            }
        }
        in_stack.remove(node);
        false
    }
}
/// A local session type (from one role's perspective).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LType {
    /// `!q(T).L` — send T to role q.
    Send(Role, BaseType, Box<LType>),
    /// `?p(T).L` — receive T from role p.
    Recv(Role, BaseType, Box<LType>),
    /// Internal choice: select one of the labeled continuations.
    IChoice(Role, Vec<(String, LType)>),
    /// External choice: offer labeled continuations to a role.
    EChoice(Role, Vec<(String, LType)>),
    /// Session end.
    End,
    /// Recursive local type.
    Rec(String, Box<LType>),
    /// Type variable.
    Var(String),
}
/// A single session operation (for type checking).
#[derive(Debug, Clone)]
pub enum SessionOp {
    /// Send a value of the given type.
    Send(BaseType),
    /// Receive a value of the given type.
    Recv(BaseType),
    /// Select the left branch.
    SelectLeft,
    /// Select the right branch.
    SelectRight,
    /// Close the session.
    Close,
}
impl SessionOp {
    /// Check this operation against the current session type, returning the continuation type.
    pub fn check_step(&self, stype: SType) -> Result<SType, String> {
        match (self, &stype) {
            (SessionOp::Send(t), SType::Send(expected, cont)) => {
                if t == expected.as_ref() {
                    Ok(*cont.clone())
                } else {
                    Err(format!(
                        "Type mismatch: sent {:?} but expected {:?}",
                        t, expected
                    ))
                }
            }
            (SessionOp::Recv(t), SType::Recv(expected, cont)) => {
                if t == expected.as_ref() {
                    Ok(*cont.clone())
                } else {
                    Err(format!(
                        "Type mismatch: recv {:?} but expected {:?}",
                        t, expected
                    ))
                }
            }
            (SessionOp::SelectLeft, SType::Choice(left, _)) => Ok(*left.clone()),
            (SessionOp::SelectRight, SType::Choice(_, right)) => Ok(*right.clone()),
            (SessionOp::Close, SType::End) => Ok(SType::End),
            _ => Err(format!(
                "Operation {:?} incompatible with session type {}",
                self, stype
            )),
        }
    }
}
/// The result of a runtime monitor check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MonitorResult {
    /// The communication matched the expected type.
    Ok,
    /// A cast was inserted (type did not match static annotation).
    CastInserted(String),
    /// A hard failure: the communication violates the protocol irreparably.
    Failure(String),
}
/// A simple session type checker for binary sessions.
pub struct SessionChecker {
    /// Mapping from channel names to their session types.
    channels: HashMap<String, SType>,
}
impl SessionChecker {
    /// Create a new session type checker.
    pub fn new() -> Self {
        SessionChecker {
            channels: HashMap::new(),
        }
    }
    /// Register a channel with its session type.
    pub fn register_channel(&mut self, name: impl Into<String>, stype: SType) {
        self.channels.insert(name.into(), stype);
    }
    /// Check that a use pattern (sequence of operations) is consistent with
    /// the declared session type of a channel.
    pub fn check_usage(&self, channel: &str, ops: &[SessionOp]) -> Result<SType, String> {
        let stype = self
            .channels
            .get(channel)
            .ok_or_else(|| format!("Unknown channel: {}", channel))?;
        let mut current = stype.clone();
        for op in ops {
            current = op.check_step(current)?;
        }
        Ok(current)
    }
}
/// A runtime monitor for gradual session types.
///
/// Tracks the statically known portion of the session type and performs
/// dynamic checks for the parts annotated with the dynamic type `?`.
pub struct GradualSessionMonitor {
    /// The expected session type (may contain `Var("?")` for dynamic parts).
    expected: SType,
    /// Violations logged during monitoring.
    pub violations: Vec<String>,
    /// Casts inserted during monitoring.
    pub casts: Vec<String>,
}
impl GradualSessionMonitor {
    /// Create a monitor for the given expected session type.
    pub fn new(expected: SType) -> Self {
        GradualSessionMonitor {
            expected,
            violations: vec![],
            casts: vec![],
        }
    }
    /// Check a send operation against the expected type.
    /// Returns `Ok` if compatible, `CastInserted` if a cast bridges a mismatch,
    /// or `Failure` if irreparably incompatible.
    pub fn check_send(&mut self, actual_ty: &BaseType) -> MonitorResult {
        match self.expected.clone() {
            SType::Send(expected_ty, cont) => {
                self.expected = *cont;
                if actual_ty == expected_ty.as_ref() {
                    MonitorResult::Ok
                } else {
                    let msg = format!("cast {:?} → {:?}", actual_ty, expected_ty);
                    self.casts.push(msg.clone());
                    MonitorResult::CastInserted(msg)
                }
            }
            SType::Var(ref s) if s == "?" => {
                let msg = format!("dynamic send {:?}", actual_ty);
                self.casts.push(msg.clone());
                MonitorResult::CastInserted(msg)
            }
            other => {
                let msg = format!("expected Send, got {}", other);
                self.violations.push(msg.clone());
                MonitorResult::Failure(msg)
            }
        }
    }
    /// Check a receive operation against the expected type.
    pub fn check_recv(&mut self, actual_ty: &BaseType) -> MonitorResult {
        match self.expected.clone() {
            SType::Recv(expected_ty, cont) => {
                self.expected = *cont;
                if actual_ty == expected_ty.as_ref() {
                    MonitorResult::Ok
                } else {
                    let msg = format!("cast {:?} → {:?}", actual_ty, expected_ty);
                    self.casts.push(msg.clone());
                    MonitorResult::CastInserted(msg)
                }
            }
            SType::Var(ref s) if s == "?" => {
                let msg = format!("dynamic recv {:?}", actual_ty);
                self.casts.push(msg.clone());
                MonitorResult::CastInserted(msg)
            }
            other => {
                let msg = format!("expected Recv, got {}", other);
                self.violations.push(msg.clone());
                MonitorResult::Failure(msg)
            }
        }
    }
    /// Return true iff no violations have been recorded.
    pub fn is_safe(&self) -> bool {
        self.violations.is_empty()
    }
}
