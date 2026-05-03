//! Types for dependency graph analysis.

use std::collections::HashMap;

/// A node in the dependency graph, representing a single build unit (file, module, package).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DepNode {
    /// Unique numeric identifier within the graph.
    pub id: usize,
    /// Human-readable name (e.g., module name or file stem).
    pub name: String,
    /// File-system path to the source unit.
    pub path: String,
    /// Size of the source file in bytes.
    pub size_bytes: u64,
    /// Last-modified timestamp as Unix milliseconds.
    pub last_modified: u64,
}

/// A directed edge between two nodes in the dependency graph.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DepEdge {
    /// Source node id (the dependant).
    pub from: usize,
    /// Target node id (the dependency).
    pub to: usize,
    /// Semantic kind of this dependency relationship.
    pub kind: DepKind,
}

/// Semantic classification of a dependency edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DepKind {
    /// A direct import / `open` statement.
    Import,
    /// A namespace / section opening that exposes names.
    OpenNamespace,
    /// Struct / class inheritance.
    Inheritance,
    /// Type-class instance declaration.
    Instance,
    /// Axiom or sorry — unsafe assumption.
    Axiom,
}

impl std::fmt::Display for DepKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DepKind::Import => write!(f, "import"),
            DepKind::OpenNamespace => write!(f, "open_namespace"),
            DepKind::Inheritance => write!(f, "inheritance"),
            DepKind::Instance => write!(f, "instance"),
            DepKind::Axiom => write!(f, "axiom"),
        }
    }
}

/// The full dependency graph.
#[derive(Debug, Clone, Default)]
pub struct DepGraph {
    /// All nodes, keyed by their sequential position (index == node.id when added in order).
    pub nodes: Vec<DepNode>,
    /// All directed edges.
    pub edges: Vec<DepEdge>,
    /// Internal lookup: name → node id.
    pub(super) name_index: HashMap<String, usize>,
}

/// A detected dependency cycle (set of node ids forming the cycle).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DepCycle {
    /// Ordered list of node ids that constitute the cycle.
    pub nodes: Vec<usize>,
}

impl std::fmt::Display for DepCycle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ids: Vec<String> = self.nodes.iter().map(|n| n.to_string()).collect();
        write!(f, "Cycle({})", ids.join(" -> "))
    }
}

/// Aggregate statistics about a dependency graph.
#[derive(Debug, Clone)]
pub struct DepStats {
    /// Total number of nodes.
    pub node_count: usize,
    /// Total number of edges.
    pub edge_count: usize,
    /// Longest path from any root to any leaf (in edge hops).
    pub max_depth: usize,
    /// Average number of incoming edges per node.
    pub avg_fan_in: f64,
    /// Average number of outgoing edges per node.
    pub avg_fan_out: f64,
    /// Number of nodes with no incoming and no outgoing edges.
    pub isolated_count: usize,
}

/// Information about one strongly connected component.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentInfo {
    /// Sequential SCC id (0-based, in reverse topological order of SCCs).
    pub id: usize,
    /// Node ids belonging to this SCC.
    pub nodes: Vec<usize>,
    /// `true` when the SCC contains exactly one node with no self-loop.
    pub is_acyclic: bool,
}
