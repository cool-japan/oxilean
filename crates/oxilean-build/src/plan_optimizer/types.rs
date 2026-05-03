//! Types for the Build Plan Optimizer.

// ============================================================
// BuildNode
// ============================================================

/// A single node in the build dependency DAG.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildNode {
    /// Unique numeric identifier for this node (0-indexed).
    pub id: usize,
    /// Human-readable name (e.g. crate name or source file path).
    pub name: String,
    /// Estimated compilation/build cost in milliseconds.
    pub estimated_cost_ms: u64,
    /// IDs of nodes that must complete before this node can start.
    pub dependencies: Vec<usize>,
}

impl BuildNode {
    /// Construct a new `BuildNode`.
    pub fn new(
        id: usize,
        name: impl Into<String>,
        estimated_cost_ms: u64,
        dependencies: Vec<usize>,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            estimated_cost_ms,
            dependencies,
        }
    }
}

// ============================================================
// BuildPlan
// ============================================================

/// A directed acyclic graph (DAG) of build targets.
///
/// `nodes` contains every node in the graph.  `edges` is a secondary
/// representation of the dependency relationship as `(from, to)` pairs
/// meaning "node `from` must finish before node `to` can start" — i.e. the
/// same information already captured by [`BuildNode::dependencies`] but
/// stored explicitly so callers can pass either or both representations.
#[derive(Debug, Clone)]
pub struct BuildPlan {
    /// All nodes in the build graph.
    pub nodes: Vec<BuildNode>,
    /// Explicit dependency edges `(dependency_id, dependent_id)`.
    /// These are unioned with the edges implied by
    /// [`BuildNode::dependencies`] before any analysis is performed.
    pub edges: Vec<(usize, usize)>,
}

impl BuildPlan {
    /// Create a plan from nodes alone.  Edges are derived entirely from
    /// [`BuildNode::dependencies`].
    pub fn from_nodes(nodes: Vec<BuildNode>) -> Self {
        Self {
            nodes,
            edges: Vec::new(),
        }
    }

    /// Create a plan with an explicit edge list in addition to the
    /// dependency information on each node.
    pub fn new(nodes: Vec<BuildNode>, edges: Vec<(usize, usize)>) -> Self {
        Self { nodes, edges }
    }

    /// Return the number of nodes.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
}

// ============================================================
// BuildSchedule
// ============================================================

/// The result of [`schedule`](super::functions::schedule).
///
/// `lanes[i]` contains the ordered list of node IDs assigned to worker `i`.
/// Within each lane nodes must be executed sequentially in the given order;
/// different lanes may execute concurrently.
#[derive(Debug, Clone)]
pub struct BuildSchedule {
    /// Worker lanes.  `lanes[worker_index]` is the ordered sequence of node
    /// IDs that worker executes.
    pub lanes: Vec<Vec<usize>>,
    /// The IDs of nodes on the critical path (longest weighted path through
    /// the DAG), in topological order.
    pub critical_path: Vec<usize>,
    /// Lower-bound estimate of the total build time in milliseconds, assuming
    /// `lanes.len()` workers each operating at full speed.
    pub estimated_makespan_ms: u64,
}

impl BuildSchedule {
    /// Number of workers (lanes).
    pub fn num_workers(&self) -> usize {
        self.lanes.len()
    }

    /// Total number of nodes scheduled across all lanes.
    pub fn total_nodes(&self) -> usize {
        self.lanes.iter().map(|l| l.len()).sum()
    }
}
