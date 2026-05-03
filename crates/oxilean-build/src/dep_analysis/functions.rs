//! Functions for dependency graph analysis.

use std::collections::{HashMap, HashSet, VecDeque};

use super::types::{ComponentInfo, DepCycle, DepGraph, DepKind, DepNode, DepStats};

// ── DepGraph construction ────────────────────────────────────────────────────

impl DepGraph {
    /// Create an empty dependency graph.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a new node.  Returns the id assigned to the node.
    pub fn add_node(
        &mut self,
        name: &str,
        path: &str,
        size_bytes: u64,
        last_modified: u64,
    ) -> usize {
        let id = self.nodes.len();
        self.name_index.insert(name.to_string(), id);
        self.nodes.push(DepNode {
            id,
            name: name.to_string(),
            path: path.to_string(),
            size_bytes,
            last_modified,
        });
        id
    }

    /// Add a directed edge `from → to` with the given kind.
    /// Silently ignores the edge when either id is out of range.
    pub fn add_edge(&mut self, from: usize, to: usize, kind: DepKind) {
        if from < self.nodes.len() && to < self.nodes.len() {
            self.edges.push(super::types::DepEdge { from, to, kind });
        }
    }

    /// Look up a node by name.
    pub fn node_by_name(&self, name: &str) -> Option<&DepNode> {
        self.name_index.get(name).and_then(|&id| self.nodes.get(id))
    }

    // ── internal helpers ────────────────────────────────────────────────────

    /// Build forward adjacency list (from → [to]).
    pub(super) fn adjacency(&self) -> Vec<Vec<usize>> {
        let n = self.nodes.len();
        let mut adj = vec![Vec::new(); n];
        for e in &self.edges {
            adj[e.from].push(e.to);
        }
        adj
    }

    /// Build reverse adjacency list (to → [from]).
    pub(super) fn reverse_adjacency(&self) -> Vec<Vec<usize>> {
        let n = self.nodes.len();
        let mut radj = vec![Vec::new(); n];
        for e in &self.edges {
            radj[e.to].push(e.from);
        }
        radj
    }

    /// Check whether edge (from, to) is a self-loop.
    pub(super) fn has_self_loop(&self, node: usize) -> bool {
        self.edges.iter().any(|e| e.from == node && e.to == node)
    }
}

// ── Public analysis functions ────────────────────────────────────────────────

/// Compute aggregate statistics for the given dependency graph.
pub fn compute_stats(graph: &DepGraph) -> DepStats {
    let n = graph.nodes.len();
    if n == 0 {
        return DepStats {
            node_count: 0,
            edge_count: 0,
            max_depth: 0,
            avg_fan_in: 0.0,
            avg_fan_out: 0.0,
            isolated_count: 0,
        };
    }

    let adj = graph.adjacency();
    let radj = graph.reverse_adjacency();

    let mut in_degree = vec![0usize; n];
    let mut out_degree = vec![0usize; n];
    for i in 0..n {
        out_degree[i] = adj[i].len();
        in_degree[i] = radj[i].len();
    }

    let isolated_count = (0..n)
        .filter(|&i| in_degree[i] == 0 && out_degree[i] == 0)
        .count();

    let avg_fan_in = in_degree.iter().sum::<usize>() as f64 / n as f64;
    let avg_fan_out = out_degree.iter().sum::<usize>() as f64 / n as f64;

    let max_depth = compute_max_depth(graph, &adj);

    DepStats {
        node_count: n,
        edge_count: graph.edges.len(),
        max_depth,
        avg_fan_in,
        avg_fan_out,
        isolated_count,
    }
}

/// Compute the longest path length in the graph using iterative DFS with memoisation.
/// Works correctly on DAGs; on cyclic graphs it returns the longest *simple* path found
/// during DFS (approximation — exact longest path in cyclic graph is NP-hard).
fn compute_max_depth(graph: &DepGraph, adj: &[Vec<usize>]) -> usize {
    let n = graph.nodes.len();
    if n == 0 {
        return 0;
    }

    // Memo table: None = not computed, Some(d) = longest outgoing path from node.
    let mut memo: Vec<Option<usize>> = vec![None; n];
    let mut visiting: Vec<bool> = vec![false; n];

    fn dfs(
        v: usize,
        adj: &[Vec<usize>],
        memo: &mut Vec<Option<usize>>,
        visiting: &mut Vec<bool>,
    ) -> usize {
        if let Some(d) = memo[v] {
            return d;
        }
        if visiting[v] {
            // Cycle detected — return 0 to avoid infinite recursion.
            return 0;
        }
        visiting[v] = true;
        let depth = adj[v]
            .iter()
            .map(|&w| 1 + dfs(w, adj, memo, visiting))
            .max()
            .unwrap_or(0);
        visiting[v] = false;
        memo[v] = Some(depth);
        depth
    }

    (0..n)
        .map(|v| dfs(v, adj, &mut memo, &mut visiting))
        .max()
        .unwrap_or(0)
}

/// Detect all dependency cycles using Tarjan's SCC algorithm, returning cycles
/// (SCCs of size > 1 or self-loops).
pub fn find_cycles(graph: &DepGraph) -> Vec<DepCycle> {
    let sccs = tarjan_scc(graph);
    let mut cycles = Vec::new();
    for scc in &sccs {
        if scc.len() > 1 {
            cycles.push(DepCycle { nodes: scc.clone() });
        } else if scc.len() == 1 && graph.has_self_loop(scc[0]) {
            cycles.push(DepCycle { nodes: scc.clone() });
        }
    }
    cycles
}

/// Tarjan's iterative strongly-connected-components algorithm.
/// Returns SCCs in reverse topological order of the condensation DAG.
fn tarjan_scc(graph: &DepGraph) -> Vec<Vec<usize>> {
    let n = graph.nodes.len();
    let adj = graph.adjacency();

    let mut index_counter = 0usize;
    let mut stack: Vec<usize> = Vec::new();
    let mut on_stack = vec![false; n];
    let mut index = vec![usize::MAX; n];
    let mut lowlink = vec![0usize; n];
    let mut sccs: Vec<Vec<usize>> = Vec::new();

    // Iterative Tarjan using an explicit call-stack frame.
    #[derive(Debug)]
    struct Frame {
        v: usize,
        // Index into adj[v] — which successor we process next.
        child_idx: usize,
    }

    for start in 0..n {
        if index[start] != usize::MAX {
            continue;
        }

        let mut call_stack: Vec<Frame> = vec![Frame {
            v: start,
            child_idx: 0,
        }];

        // Assign index and push to Tarjan stack before entering recursion body.
        index[start] = index_counter;
        lowlink[start] = index_counter;
        index_counter += 1;
        stack.push(start);
        on_stack[start] = true;

        while let Some(frame) = call_stack.last_mut() {
            let v = frame.v;
            if frame.child_idx < adj[v].len() {
                let w = adj[v][frame.child_idx];
                frame.child_idx += 1;

                if index[w] == usize::MAX {
                    // Tree edge — recurse into w.
                    index[w] = index_counter;
                    lowlink[w] = index_counter;
                    index_counter += 1;
                    stack.push(w);
                    on_stack[w] = true;
                    call_stack.push(Frame { v: w, child_idx: 0 });
                } else if on_stack[w] {
                    // Back edge — update lowlink.
                    if lowlink[w] < lowlink[v] {
                        lowlink[v] = lowlink[w];
                    }
                }
            } else {
                // Finished processing v's children — pop the frame.
                call_stack.pop();

                if let Some(parent_frame) = call_stack.last() {
                    let p = parent_frame.v;
                    if lowlink[v] < lowlink[p] {
                        lowlink[p] = lowlink[v];
                    }
                }

                // Root of an SCC?
                if lowlink[v] == index[v] {
                    let mut scc = Vec::new();
                    loop {
                        let w = stack.pop().unwrap_or(v);
                        on_stack[w] = false;
                        scc.push(w);
                        if w == v {
                            break;
                        }
                    }
                    sccs.push(scc);
                }
            }
        }
    }

    sccs
}

/// Return a topological ordering of the nodes (Kahn's BFS algorithm).
/// Returns `Err(cycle)` when the graph contains a cycle.
pub fn topological_order(graph: &DepGraph) -> Result<Vec<usize>, DepCycle> {
    let n = graph.nodes.len();
    let adj = graph.adjacency();

    let mut in_degree = vec![0usize; n];
    for e in &graph.edges {
        in_degree[e.to] += 1;
    }

    let mut queue: VecDeque<usize> = (0..n).filter(|&i| in_degree[i] == 0).collect();
    let mut order = Vec::with_capacity(n);

    while let Some(v) = queue.pop_front() {
        order.push(v);
        for &w in &adj[v] {
            in_degree[w] -= 1;
            if in_degree[w] == 0 {
                queue.push_back(w);
            }
        }
    }

    if order.len() == n {
        Ok(order)
    } else {
        // At least one cycle exists — report one.
        let cycle_nodes: Vec<usize> = (0..n).filter(|&i| in_degree[i] > 0).collect();
        Err(DepCycle { nodes: cycle_nodes })
    }
}

/// Compute all strongly connected components and wrap them in `ComponentInfo`.
pub fn strongly_connected_components(graph: &DepGraph) -> Vec<ComponentInfo> {
    let raw_sccs = tarjan_scc(graph);
    raw_sccs
        .into_iter()
        .enumerate()
        .map(|(id, nodes)| {
            let is_acyclic = nodes.len() == 1 && !graph.has_self_loop(nodes[0]);
            ComponentInfo {
                id,
                nodes,
                is_acyclic,
            }
        })
        .collect()
}

/// Compute the full transitive closure as an n×n boolean matrix.
/// `result[i][j] == true` means node `j` is reachable from node `i`.
pub fn transitive_closure(graph: &DepGraph) -> Vec<Vec<bool>> {
    let n = graph.nodes.len();
    let adj = graph.adjacency();

    // Initialise with direct edges.
    let mut reach = vec![vec![false; n]; n];
    for i in 0..n {
        reach[i][i] = true;
        for &j in &adj[i] {
            reach[i][j] = true;
        }
    }

    // Warshall (Floyd–Warshall transitive closure variant).
    for k in 0..n {
        for i in 0..n {
            if reach[i][k] {
                // Collect the reachable set from k first to avoid borrow conflicts.
                let reachable_from_k: Vec<usize> = (0..n).filter(|&j| reach[k][j]).collect();
                for j in reachable_from_k {
                    reach[i][j] = true;
                }
            }
        }
    }

    reach
}

/// Return all node ids reachable from `start` (inclusive) via BFS.
pub fn reachable_from(graph: &DepGraph, start: usize) -> Vec<usize> {
    let n = graph.nodes.len();
    if start >= n {
        return Vec::new();
    }
    let adj = graph.adjacency();
    let mut visited = vec![false; n];
    let mut queue = VecDeque::new();
    let mut result = Vec::new();

    visited[start] = true;
    queue.push_back(start);

    while let Some(v) = queue.pop_front() {
        result.push(v);
        for &w in &adj[v] {
            if !visited[w] {
                visited[w] = true;
                queue.push_back(w);
            }
        }
    }

    result
}

/// Return all node ids that transitively depend on `changed` — i.e. everything
/// that needs to be rebuilt when `changed` is modified.
/// Uses reverse-edge BFS (who imports `changed`?).
pub fn impact_of_change(graph: &DepGraph, changed: usize) -> Vec<usize> {
    let n = graph.nodes.len();
    if changed >= n {
        return Vec::new();
    }
    let radj = graph.reverse_adjacency();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut result = Vec::new();

    visited.insert(changed);
    queue.push_back(changed);

    while let Some(v) = queue.pop_front() {
        result.push(v);
        for &w in &radj[v] {
            if visited.insert(w) {
                queue.push_back(w);
            }
        }
    }

    result
}

/// Return the length of the critical (longest) path in the graph in edge hops.
pub fn critical_path_length(graph: &DepGraph) -> usize {
    let adj = graph.adjacency();
    compute_max_depth(graph, &adj)
}

/// Render the graph in Graphviz DOT format.
pub fn format_dep_graph(graph: &DepGraph) -> String {
    let mut out = String::from("digraph dep {\n");
    out.push_str("    rankdir=LR;\n");
    out.push_str("    node [shape=box];\n");

    for node in &graph.nodes {
        out.push_str(&format!(
            "    n{} [label=\"{}\"];\n",
            node.id,
            escape_dot(&node.name)
        ));
    }

    for edge in &graph.edges {
        out.push_str(&format!(
            "    n{} -> n{} [label=\"{}\"];\n",
            edge.from, edge.to, edge.kind
        ));
    }

    out.push_str("}\n");
    out
}

/// Escape a string for use inside a DOT label.
fn escape_dot(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── Helpers ──────────────────────────────────────────────────────────────

    fn simple_dag() -> DepGraph {
        // A → B → D
        // A → C → D
        let mut g = DepGraph::new();
        let a = g.add_node("A", "/a.lean", 100, 0);
        let b = g.add_node("B", "/b.lean", 200, 0);
        let c = g.add_node("C", "/c.lean", 150, 0);
        let d = g.add_node("D", "/d.lean", 80, 0);
        g.add_edge(a, b, DepKind::Import);
        g.add_edge(a, c, DepKind::Import);
        g.add_edge(b, d, DepKind::Import);
        g.add_edge(c, d, DepKind::Import);
        g
    }

    fn cyclic_graph() -> DepGraph {
        // A → B → C → A  (cycle), D isolated
        let mut g = DepGraph::new();
        let a = g.add_node("A", "/a.lean", 10, 0);
        let b = g.add_node("B", "/b.lean", 10, 0);
        let c = g.add_node("C", "/c.lean", 10, 0);
        let _d = g.add_node("D", "/d.lean", 10, 0);
        g.add_edge(a, b, DepKind::Import);
        g.add_edge(b, c, DepKind::Import);
        g.add_edge(c, a, DepKind::Import);
        g
    }

    // ── DepGraph construction ─────────────────────────────────────────────

    #[test]
    fn test_new_graph_is_empty() {
        let g = DepGraph::new();
        assert!(g.nodes.is_empty());
        assert!(g.edges.is_empty());
    }

    #[test]
    fn test_add_node_assigns_sequential_ids() {
        let mut g = DepGraph::new();
        let id0 = g.add_node("Foo", "/foo.lean", 0, 0);
        let id1 = g.add_node("Bar", "/bar.lean", 0, 0);
        assert_eq!(id0, 0);
        assert_eq!(id1, 1);
        assert_eq!(g.nodes.len(), 2);
    }

    #[test]
    fn test_node_by_name_found() {
        let mut g = DepGraph::new();
        g.add_node("Alpha", "/alpha.lean", 42, 1000);
        let node = g.node_by_name("Alpha");
        assert!(node.is_some());
        assert_eq!(
            node.unwrap_or_else(|| panic!("node missing")).size_bytes,
            42
        );
    }

    #[test]
    fn test_node_by_name_not_found() {
        let g = DepGraph::new();
        assert!(g.node_by_name("Missing").is_none());
    }

    #[test]
    fn test_add_edge_out_of_range_ignored() {
        let mut g = DepGraph::new();
        g.add_node("A", "/a.lean", 0, 0);
        g.add_edge(0, 99, DepKind::Import); // 99 is invalid
        assert!(g.edges.is_empty());
    }

    // ── compute_stats ─────────────────────────────────────────────────────

    #[test]
    fn test_stats_empty_graph() {
        let g = DepGraph::new();
        let s = compute_stats(&g);
        assert_eq!(s.node_count, 0);
        assert_eq!(s.edge_count, 0);
        assert_eq!(s.max_depth, 0);
    }

    #[test]
    fn test_stats_dag() {
        let g = simple_dag();
        let s = compute_stats(&g);
        assert_eq!(s.node_count, 4);
        assert_eq!(s.edge_count, 4);
        assert_eq!(s.max_depth, 2);
        assert_eq!(s.isolated_count, 0);
    }

    #[test]
    fn test_stats_isolated_node() {
        let mut g = DepGraph::new();
        g.add_node("Lone", "/lone.lean", 0, 0);
        let s = compute_stats(&g);
        assert_eq!(s.isolated_count, 1);
        assert_eq!(s.avg_fan_in, 0.0);
        assert_eq!(s.avg_fan_out, 0.0);
    }

    #[test]
    fn test_stats_avg_fan() {
        let g = simple_dag();
        let s = compute_stats(&g);
        // 4 edges, 4 nodes → avg fan-out = 1.0, avg fan-in = 1.0
        assert!((s.avg_fan_out - 1.0).abs() < 1e-9);
        assert!((s.avg_fan_in - 1.0).abs() < 1e-9);
    }

    // ── topological_order ─────────────────────────────────────────────────

    #[test]
    fn test_topological_order_dag() {
        let g = simple_dag();
        let order = topological_order(&g).expect("DAG should have topological order");
        assert_eq!(order.len(), 4);
        // A (0) must come before B (1) and C (2); D (3) must come last.
        let pos: HashMap<usize, usize> = order.iter().enumerate().map(|(i, &v)| (v, i)).collect();
        assert!(pos[&0] < pos[&1]);
        assert!(pos[&0] < pos[&2]);
        assert!(pos[&1] < pos[&3]);
        assert!(pos[&2] < pos[&3]);
    }

    #[test]
    fn test_topological_order_cycle_returns_err() {
        let g = cyclic_graph();
        assert!(topological_order(&g).is_err());
    }

    #[test]
    fn test_topological_order_single_node() {
        let mut g = DepGraph::new();
        g.add_node("Solo", "/solo.lean", 0, 0);
        let order = topological_order(&g).expect("single node is acyclic");
        assert_eq!(order, vec![0]);
    }

    // ── find_cycles ───────────────────────────────────────────────────────

    #[test]
    fn test_find_cycles_dag_no_cycles() {
        let g = simple_dag();
        assert!(find_cycles(&g).is_empty());
    }

    #[test]
    fn test_find_cycles_detects_cycle() {
        let g = cyclic_graph();
        let cycles = find_cycles(&g);
        assert!(!cycles.is_empty());
        // The cycle must involve A, B, C (ids 0,1,2).
        let all_nodes: HashSet<usize> = cycles
            .iter()
            .flat_map(|c| c.nodes.iter().copied())
            .collect();
        assert!(all_nodes.contains(&0));
        assert!(all_nodes.contains(&1));
        assert!(all_nodes.contains(&2));
    }

    #[test]
    fn test_find_cycles_self_loop() {
        let mut g = DepGraph::new();
        g.add_node("Self", "/self.lean", 0, 0);
        g.add_edge(0, 0, DepKind::Import);
        let cycles = find_cycles(&g);
        assert!(!cycles.is_empty());
        assert_eq!(cycles[0].nodes, vec![0]);
    }

    // ── strongly_connected_components ────────────────────────────────────

    #[test]
    fn test_scc_dag_all_trivial() {
        let g = simple_dag();
        let sccs = strongly_connected_components(&g);
        // Each SCC should contain exactly one node (acyclic).
        for scc in &sccs {
            assert!(scc.is_acyclic, "Expected all SCCs to be trivial in a DAG");
        }
    }

    #[test]
    fn test_scc_cycle_has_nontrivial_component() {
        let g = cyclic_graph();
        let sccs = strongly_connected_components(&g);
        let non_trivial: Vec<_> = sccs.iter().filter(|s| !s.is_acyclic).collect();
        assert!(!non_trivial.is_empty());
        assert_eq!(non_trivial[0].nodes.len(), 3); // A, B, C form one SCC
    }

    // ── transitive_closure ───────────────────────────────────────────────

    #[test]
    fn test_transitive_closure_dag() {
        let g = simple_dag(); // A→B→D, A→C→D
        let tc = transitive_closure(&g);
        // A (0) can reach all.
        assert!(tc[0][1]); // A→B
        assert!(tc[0][2]); // A→C
        assert!(tc[0][3]); // A→D (transitively)
                           // D (3) cannot reach anything except itself.
        assert!(!tc[3][0]);
        assert!(!tc[3][1]);
        assert!(!tc[3][2]);
        assert!(tc[3][3]); // reflexive
    }

    #[test]
    fn test_transitive_closure_reflexive() {
        let mut g = DepGraph::new();
        g.add_node("X", "/x.lean", 0, 0);
        let tc = transitive_closure(&g);
        assert!(tc[0][0]);
    }

    // ── reachable_from ───────────────────────────────────────────────────

    #[test]
    fn test_reachable_from_includes_start() {
        let g = simple_dag();
        let reachable = reachable_from(&g, 0);
        assert!(reachable.contains(&0));
    }

    #[test]
    fn test_reachable_from_complete_from_root() {
        let g = simple_dag();
        let reachable = reachable_from(&g, 0);
        assert_eq!(reachable.len(), 4); // all nodes reachable from A
    }

    #[test]
    fn test_reachable_from_leaf_only_itself() {
        let g = simple_dag();
        let reachable = reachable_from(&g, 3); // D has no outgoing edges
        assert_eq!(reachable, vec![3]);
    }

    #[test]
    fn test_reachable_from_out_of_range() {
        let g = simple_dag();
        let reachable = reachable_from(&g, 999);
        assert!(reachable.is_empty());
    }

    // ── impact_of_change ─────────────────────────────────────────────────

    #[test]
    fn test_impact_of_change_leaf() {
        // A→B→D, A→C→D means B and C import D, and A imports B and C.
        // Changing D means everything that transitively imports D must rebuild:
        // D itself, B (imports D), C (imports D), A (imports B and C).
        let g = simple_dag();
        let impact = impact_of_change(&g, 3);
        let impact_set: std::collections::HashSet<usize> = impact.into_iter().collect();
        // All four nodes are affected: D itself, plus B, C, A through transitive dependants.
        assert!(impact_set.contains(&3)); // D
        assert!(impact_set.contains(&1)); // B imports D
        assert!(impact_set.contains(&2)); // C imports D
        assert!(impact_set.contains(&0)); // A imports B and C
    }

    #[test]
    fn test_impact_of_change_intermediate() {
        let g = simple_dag();
        // Changing B (id=1) affects B and A.
        let impact = impact_of_change(&g, 1);
        let impact_set: HashSet<usize> = impact.into_iter().collect();
        assert!(impact_set.contains(&1));
        assert!(impact_set.contains(&0)); // A imports B
    }

    #[test]
    fn test_impact_of_change_out_of_range() {
        let g = simple_dag();
        assert!(impact_of_change(&g, 999).is_empty());
    }

    // ── critical_path_length ─────────────────────────────────────────────

    #[test]
    fn test_critical_path_dag() {
        let g = simple_dag();
        assert_eq!(critical_path_length(&g), 2);
    }

    #[test]
    fn test_critical_path_empty() {
        let g = DepGraph::new();
        assert_eq!(critical_path_length(&g), 0);
    }

    #[test]
    fn test_critical_path_single_edge() {
        let mut g = DepGraph::new();
        g.add_node("A", "/a.lean", 0, 0);
        g.add_node("B", "/b.lean", 0, 0);
        g.add_edge(0, 1, DepKind::Import);
        assert_eq!(critical_path_length(&g), 1);
    }

    // ── format_dep_graph ─────────────────────────────────────────────────

    #[test]
    fn test_format_dep_graph_contains_digraph() {
        let g = simple_dag();
        let dot = format_dep_graph(&g);
        assert!(dot.contains("digraph dep"));
    }

    #[test]
    fn test_format_dep_graph_contains_nodes() {
        let g = simple_dag();
        let dot = format_dep_graph(&g);
        assert!(dot.contains("n0"));
        assert!(dot.contains("n3"));
        assert!(dot.contains('A'));
        assert!(dot.contains('D'));
    }

    #[test]
    fn test_format_dep_graph_contains_edges() {
        let g = simple_dag();
        let dot = format_dep_graph(&g);
        assert!(dot.contains("->"));
        assert!(dot.contains("import"));
    }

    #[test]
    fn test_format_dep_graph_empty() {
        let g = DepGraph::new();
        let dot = format_dep_graph(&g);
        assert!(dot.contains("digraph dep"));
        assert!(!dot.contains("->"));
    }

    // ── DepKind display ───────────────────────────────────────────────────

    #[test]
    fn test_dep_kind_display() {
        assert_eq!(DepKind::Import.to_string(), "import");
        assert_eq!(DepKind::OpenNamespace.to_string(), "open_namespace");
        assert_eq!(DepKind::Inheritance.to_string(), "inheritance");
        assert_eq!(DepKind::Instance.to_string(), "instance");
        assert_eq!(DepKind::Axiom.to_string(), "axiom");
    }

    // ── DepCycle display ──────────────────────────────────────────────────

    #[test]
    fn test_dep_cycle_display() {
        let c = DepCycle {
            nodes: vec![0, 1, 2],
        };
        let s = c.to_string();
        assert!(s.contains("Cycle"));
        assert!(s.contains('0'));
        assert!(s.contains('2'));
    }

    // ── deeper chain ─────────────────────────────────────────────────────

    #[test]
    fn test_critical_path_long_chain() {
        // 0→1→2→3→4  (length 4)
        let mut g = DepGraph::new();
        for i in 0..5usize {
            g.add_node(&i.to_string(), "/x.lean", 0, 0);
        }
        for i in 0..4usize {
            g.add_edge(i, i + 1, DepKind::Import);
        }
        assert_eq!(critical_path_length(&g), 4);
    }

    #[test]
    fn test_topological_order_chain() {
        let mut g = DepGraph::new();
        for i in 0..5usize {
            g.add_node(&i.to_string(), "/x.lean", 0, 0);
        }
        for i in 0..4usize {
            g.add_edge(i, i + 1, DepKind::Import);
        }
        let order = topological_order(&g).expect("chain is a DAG");
        for i in 0..4 {
            assert!(order.iter().position(|&x| x == i) < order.iter().position(|&x| x == i + 1));
        }
    }
}
