// distributed_rpc/types.rs — Simulated distributed RPC framework types

use std::collections::HashMap;

/// Identifies a node in the simulated cluster
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NodeId(pub u32);

impl std::fmt::Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node({})", self.0)
    }
}

/// A named service registered on a node
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ServiceName(pub String);

impl ServiceName {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }
}

impl std::fmt::Display for ServiceName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Outgoing RPC call
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RpcRequest {
    pub id: u64,
    pub from: NodeId,
    pub to: NodeId,
    pub service: ServiceName,
    pub method: String,
    pub args: Vec<String>,
}

/// The outcome of an RPC invocation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RpcResult {
    /// Successful call, carries the string-encoded return value
    Ok(String),
    /// Call failed with an application-level error
    Err(String),
    /// No reply arrived within the simulated deadline
    Timeout,
    /// Target node is not reachable
    NodeDown,
}

/// Response envelope matching a prior `RpcRequest`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RpcResponse {
    pub request_id: u64,
    pub from: NodeId,
    pub to: NodeId,
    pub result: RpcResult,
}

/// Maps `(NodeId, ServiceName)` → list of exposed method names
#[derive(Debug, Default, Clone)]
pub struct ServiceRegistry {
    pub services: HashMap<(NodeId, ServiceName), Vec<String>>,
}

impl ServiceRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, node: NodeId, service: ServiceName, methods: Vec<String>) {
        self.services.insert((node, service), methods);
    }

    /// Return all nodes that expose `service`
    pub fn nodes_for(&self, service: &ServiceName) -> Vec<NodeId> {
        self.services
            .keys()
            .filter_map(|(n, s)| if s == service { Some(*n) } else { None })
            .collect()
    }

    pub fn has_method(&self, node: NodeId, service: &ServiceName, method: &str) -> bool {
        self.services
            .get(&(node, service.clone()))
            .map_or(false, |methods| methods.iter().any(|m| m == method))
    }
}

/// Circuit-breaker state
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CircuitState {
    /// Normal operation — calls pass through
    Closed,
    /// Circuit tripped — calls are rejected immediately
    Open,
    /// Probe phase — one call is allowed through to test recovery
    HalfOpen,
}

/// Per-service or per-node circuit breaker
#[derive(Debug, Clone)]
pub struct CircuitBreaker {
    /// Consecutive failures since last success
    pub failures: u32,
    /// Number of failures before tripping to `Open`
    pub threshold: u32,
    pub state: CircuitState,
    /// Logical timestamp of the most recent attempt (caller-managed clock)
    pub last_attempt: u64,
    /// How many time units the breaker stays `Open` before moving to `HalfOpen`
    pub open_duration: u64,
}

impl CircuitBreaker {
    pub fn new(threshold: u32, open_duration: u64) -> Self {
        Self {
            failures: 0,
            threshold,
            state: CircuitState::Closed,
            last_attempt: 0,
            open_duration,
        }
    }
}

/// The simulated RPC network
#[derive(Debug)]
pub struct RpcNetwork {
    /// All participating node identifiers
    pub nodes: Vec<NodeId>,
    /// Service registry
    pub registry: ServiceRegistry,
    /// Simulated one-way latency in ms between node pairs
    pub latency_ms: HashMap<(NodeId, NodeId), u64>,
    /// Full log of all (request, response) pairs handled
    pub message_log: Vec<(RpcRequest, RpcResponse)>,
    /// Set of nodes that are currently unreachable
    pub(super) downed_nodes: std::collections::HashSet<NodeId>,
    /// Monotonically increasing request counter
    pub(super) next_req_id: u64,
    /// Simulated wall-clock in ms (advanced per call by latency)
    pub(super) clock_ms: u64,
}

impl RpcNetwork {
    pub fn new(nodes: Vec<NodeId>) -> Self {
        Self {
            nodes,
            registry: ServiceRegistry::new(),
            latency_ms: HashMap::new(),
            message_log: Vec::new(),
            downed_nodes: std::collections::HashSet::new(),
            next_req_id: 0,
            clock_ms: 0,
        }
    }
}
