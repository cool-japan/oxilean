//! Types for advanced graph algorithms.

/// A weighted directed graph with `n` vertices (0-indexed).
#[derive(Clone, Debug)]
pub struct WGraph {
    /// Number of vertices.
    pub n: usize,
    /// Adjacency list: `adj\[u\]` = list of `(v, weight)`.
    pub adj: Vec<Vec<(usize, i64)>>,
}

impl WGraph {
    /// Construct an empty weighted directed graph with `n` vertices.
    pub fn new(n: usize) -> Self {
        Self {
            n,
            adj: vec![vec![]; n],
        }
    }

    /// Add directed edge u → v with integer weight.
    pub fn add_edge(&mut self, u: usize, v: usize, w: i64) {
        self.adj[u].push((v, w));
    }
}

/// A single edge in a flow network (internal bookkeeping).
#[derive(Clone, Debug)]
pub struct FlowEdge {
    /// Source vertex of this edge.
    pub from: usize,
    /// Target vertex of this edge.
    pub to: usize,
    /// Maximum capacity.
    pub cap: i64,
    /// Current flow.
    pub flow: i64,
    /// Index of the reverse edge in the network's edge list.
    pub rev: usize,
}

/// Flow value and capacity pair (utility wrapper, not used internally in network).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Flow {
    /// Maximum capacity on this channel.
    pub capacity: i64,
    /// Current flow value.
    pub flow: i64,
}

/// A flow network for max-flow computation.
///
/// Edges are stored in a flat list; each edge at index `i` has its reverse at `edges\[i\].rev`.
/// Residual capacity of edge `i` = `edges\[i\].cap - edges\[i\].flow`.
#[derive(Clone, Debug)]
pub struct FlowNetwork {
    /// Number of vertices.
    pub n: usize,
    /// Source vertex index.
    pub source: usize,
    /// Sink vertex index.
    pub sink: usize,
    /// All edges (including reverse edges).
    pub edges: Vec<FlowEdge>,
    /// Graph adjacency for edge indices: `graph\[u\]` = list of edge indices out of `u`.
    pub graph: Vec<Vec<usize>>,
}

impl FlowNetwork {
    /// Create a new flow network with `n` vertices, given source and sink.
    pub fn new(n: usize, source: usize, sink: usize) -> Self {
        Self {
            n,
            source,
            sink,
            edges: Vec::new(),
            graph: vec![vec![]; n],
        }
    }

    /// Add a directed edge from `u` to `v` with capacity `cap`.
    /// Automatically adds a reverse edge with capacity 0.
    pub fn add_edge(&mut self, u: usize, v: usize, cap: i64) {
        let fwd_idx = self.edges.len();
        let bwd_idx = fwd_idx + 1;
        self.edges.push(FlowEdge {
            from: u,
            to: v,
            cap,
            flow: 0,
            rev: bwd_idx,
        });
        self.edges.push(FlowEdge {
            from: v,
            to: u,
            cap: 0,
            flow: 0,
            rev: fwd_idx,
        });
        self.graph[u].push(fwd_idx);
        self.graph[v].push(bwd_idx);
    }

    /// Residual capacity of edge at index `eid`.
    pub fn residual(&self, eid: usize) -> i64 {
        self.edges[eid].cap - self.edges[eid].flow
    }
}

/// Result of bipartite matching.
#[derive(Clone, Debug)]
pub struct MatchingResult {
    /// `matching[left_vertex]` = Some(right_vertex) if matched, else None.
    pub matching: Vec<Option<usize>>,
    /// Total number of matched pairs.
    pub size: usize,
}

/// A spanning tree (or forest) encoded as an edge list with total weight.
#[derive(Clone, Debug)]
pub struct SpanningTree {
    /// Edges `(u, v, weight)` in the spanning tree.
    pub edges: Vec<(usize, usize, i64)>,
    /// Sum of edge weights.
    pub total_weight: i64,
}

/// All-pairs shortest path result (Floyd-Warshall output).
#[derive(Clone, Debug)]
pub struct AllPairsShortestPath {
    /// `dist\[i\]\[j\]` = shortest distance from i to j, or `None` if unreachable / negative cycle.
    pub dist: Vec<Vec<Option<i64>>>,
    /// `next\[i\]\[j\]` = next vertex on the shortest path i → j, for path reconstruction.
    pub next: Vec<Vec<Option<usize>>>,
}

/// Strongly connected components result (Tarjan's algorithm).
#[derive(Clone, Debug)]
pub struct StronglyConnectedComponents {
    /// List of SCCs; each SCC is a list of vertex indices.
    /// SCCs are returned in reverse topological order of the condensation DAG.
    pub components: Vec<Vec<usize>>,
    /// `component_of\[v\]` = index into `components` for vertex `v`.
    pub component_of: Vec<usize>,
}
