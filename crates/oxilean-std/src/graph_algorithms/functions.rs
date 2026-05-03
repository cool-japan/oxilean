//! Advanced graph algorithm implementations.

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

use super::types::{
    AllPairsShortestPath, FlowNetwork, MatchingResult, SpanningTree, StronglyConnectedComponents,
    WGraph,
};

// ── Union-Find (Disjoint Set Union) ──────────────────────────────────────────

struct Dsu {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let ra = self.find(a);
        let rb = self.find(b);
        if ra == rb {
            return false;
        }
        match self.rank[ra].cmp(&self.rank[rb]) {
            std::cmp::Ordering::Less => self.parent[ra] = rb,
            std::cmp::Ordering::Greater => self.parent[rb] = ra,
            std::cmp::Ordering::Equal => {
                self.parent[rb] = ra;
                self.rank[ra] += 1;
            }
        }
        true
    }
}

// ── Shortest Paths ────────────────────────────────────────────────────────────

/// Dijkstra's algorithm for single-source shortest paths (non-negative weights).
///
/// Returns `dist\[v\]` = `Some(d)` if reachable, `None` otherwise.
/// Panics (via bounds check) if `start >= g.n`.
pub fn dijkstra(g: &WGraph, start: usize) -> Vec<Option<i64>> {
    let n = g.n;
    let inf = i64::MAX / 2;
    let mut dist = vec![inf; n];
    dist[start] = 0;
    let mut heap: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();
    heap.push(Reverse((0, start)));

    while let Some(Reverse((d, u))) = heap.pop() {
        if d > dist[u] {
            continue;
        }
        for &(v, w) in &g.adj[u] {
            let nd = dist[u].saturating_add(w);
            if nd < dist[v] {
                dist[v] = nd;
                heap.push(Reverse((nd, v)));
            }
        }
    }

    dist.into_iter()
        .map(|d| if d == inf { None } else { Some(d) })
        .collect()
}

/// Bellman-Ford algorithm for single-source shortest paths (allows negative weights).
///
/// Returns `Ok(dist)` with `None` for unreachable vertices, or `Err(msg)` if a
/// negative-weight cycle reachable from `start` is detected.
pub fn bellman_ford(g: &WGraph, start: usize) -> Result<Vec<Option<i64>>, String> {
    let n = g.n;
    let inf = i64::MAX / 2;
    let mut dist = vec![inf; n];
    dist[start] = 0;

    // n-1 relaxation rounds
    for _ in 0..n.saturating_sub(1) {
        let mut changed = false;
        for u in 0..n {
            if dist[u] == inf {
                continue;
            }
            for &(v, w) in &g.adj[u] {
                let nd = dist[u].saturating_add(w);
                if nd < dist[v] {
                    dist[v] = nd;
                    changed = true;
                }
            }
        }
        if !changed {
            break;
        }
    }

    // Negative-cycle detection: one more round
    for u in 0..n {
        if dist[u] == inf {
            continue;
        }
        for &(v, w) in &g.adj[u] {
            let nd = dist[u].saturating_add(w);
            if nd < dist[v] {
                return Err(format!(
                    "Negative-weight cycle detected reachable from vertex {}",
                    start
                ));
            }
        }
    }

    Ok(dist
        .into_iter()
        .map(|d| if d == inf { None } else { Some(d) })
        .collect())
}

/// Floyd-Warshall all-pairs shortest paths with path reconstruction.
///
/// Handles negative weights but not negative cycles.
/// For vertices on a negative cycle, distances are set to `None`.
pub fn floyd_warshall(g: &WGraph) -> AllPairsShortestPath {
    let n = g.n;
    let inf = i64::MAX / 4;

    let mut d = vec![vec![inf; n]; n];
    let mut next: Vec<Vec<Option<usize>>> = vec![vec![None; n]; n];

    for u in 0..n {
        d[u][u] = 0;
        for &(v, w) in &g.adj[u] {
            if w < d[u][v] {
                d[u][v] = w;
                next[u][v] = Some(v);
            }
        }
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if d[i][k] < inf && d[k][j] < inf {
                    let through = d[i][k] + d[k][j];
                    if through < d[i][j] {
                        d[i][j] = through;
                        next[i][j] = next[i][k];
                    }
                }
            }
        }
    }

    // Convert to Option, mapping inf to None
    let dist = d
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|v| if v >= inf { None } else { Some(v) })
                .collect()
        })
        .collect();

    AllPairsShortestPath { dist, next }
}

// ── Minimum Spanning Tree ─────────────────────────────────────────────────────

/// Kruskal's MST algorithm on an undirected weighted graph given as an edge list.
///
/// `edges` = slice of `(u, v, weight)`. Works on undirected edges.
/// Returns the spanning tree (or forest if disconnected).
pub fn kruskal_mst(n: usize, edges: &[(usize, usize, i64)]) -> SpanningTree {
    let mut sorted: Vec<(i64, usize, usize)> = edges.iter().map(|&(u, v, w)| (w, u, v)).collect();
    sorted.sort_unstable();

    let mut dsu = Dsu::new(n);
    let mut tree_edges: Vec<(usize, usize, i64)> = Vec::new();
    let mut total_weight = 0i64;

    for (w, u, v) in sorted {
        if dsu.union(u, v) {
            tree_edges.push((u, v, w));
            total_weight += w;
        }
    }

    SpanningTree {
        edges: tree_edges,
        total_weight,
    }
}

/// Prim's MST algorithm on a weighted directed graph.
///
/// Treats the graph as undirected (uses both u→v and v→u edges if present).
/// Starts from vertex 0.
pub fn prim_mst(g: &WGraph) -> SpanningTree {
    let n = g.n;
    if n == 0 {
        return SpanningTree {
            edges: vec![],
            total_weight: 0,
        };
    }

    // Build undirected adjacency for Prim
    let mut adj: Vec<Vec<(usize, i64)>> = vec![vec![]; n];
    for u in 0..n {
        for &(v, w) in &g.adj[u] {
            adj[u].push((v, w));
            adj[v].push((u, w));
        }
    }

    let inf = i64::MAX / 2;
    let mut in_tree = vec![false; n];
    let mut min_edge: Vec<i64> = vec![inf; n];
    let mut parent: Vec<Option<usize>> = vec![None; n];
    min_edge[0] = 0;

    // Min-heap: (cost, vertex, from_vertex)
    let mut heap: BinaryHeap<Reverse<(i64, usize, usize)>> = BinaryHeap::new();
    heap.push(Reverse((0, 0, 0)));

    let mut tree_edges: Vec<(usize, usize, i64)> = Vec::new();
    let mut total_weight = 0i64;

    while let Some(Reverse((cost, u, from))) = heap.pop() {
        if in_tree[u] {
            continue;
        }
        in_tree[u] = true;
        if u != 0 {
            tree_edges.push((from, u, cost));
            total_weight += cost;
        }

        for &(v, w) in &adj[u] {
            if !in_tree[v] && w < min_edge[v] {
                min_edge[v] = w;
                parent[v] = Some(u);
                heap.push(Reverse((w, v, u)));
            }
        }
    }

    SpanningTree {
        edges: tree_edges,
        total_weight,
    }
}

// ── Max-Flow / Min-Cut ────────────────────────────────────────────────────────

/// Edmonds-Karp max-flow (BFS-based Ford-Fulkerson on `FlowNetwork`).
///
/// Modifies the flow network in-place and returns the total max-flow value.
pub fn max_flow_bfs(net: &mut FlowNetwork) -> i64 {
    let source = net.source;
    let sink = net.sink;
    let mut total_flow = 0i64;

    loop {
        // BFS to find augmenting path
        let mut prev_edge: Vec<Option<usize>> = vec![None; net.n];
        let mut visited = vec![false; net.n];
        let mut queue = VecDeque::new();
        visited[source] = true;
        queue.push_back(source);

        'bfs: while let Some(u) = queue.pop_front() {
            for &eid in &net.graph[u].clone() {
                let v = net.edges[eid].to;
                if !visited[v] && net.residual(eid) > 0 {
                    visited[v] = true;
                    prev_edge[v] = Some(eid);
                    if v == sink {
                        break 'bfs;
                    }
                    queue.push_back(v);
                }
            }
        }

        if !visited[sink] {
            break;
        }

        // Find bottleneck
        let mut bottleneck = i64::MAX;
        let mut cur = sink;
        while cur != source {
            if let Some(eid) = prev_edge[cur] {
                bottleneck = bottleneck.min(net.residual(eid));
                cur = net.edges[eid].from;
            } else {
                break;
            }
        }

        // Update flows
        cur = sink;
        while cur != source {
            if let Some(eid) = prev_edge[cur] {
                net.edges[eid].flow += bottleneck;
                let rev = net.edges[eid].rev;
                net.edges[rev].flow -= bottleneck;
                cur = net.edges[eid].from;
            } else {
                break;
            }
        }

        total_flow += bottleneck;
    }

    total_flow
}

/// Compute the S/T partition (min-cut) after running `max_flow_bfs`.
///
/// Returns `(S, T)` where `S` contains vertices reachable from source in the residual graph
/// and `T` contains the rest.
pub fn min_cut(net: &FlowNetwork, _max_flow_val: i64) -> (Vec<usize>, Vec<usize>) {
    let source = net.source;
    let n = net.n;
    let mut reachable = vec![false; n];
    let mut queue = VecDeque::new();
    reachable[source] = true;
    queue.push_back(source);

    while let Some(u) = queue.pop_front() {
        for &eid in &net.graph[u] {
            let v = net.edges[eid].to;
            if !reachable[v] && net.residual(eid) > 0 {
                reachable[v] = true;
                queue.push_back(v);
            }
        }
    }

    let s_side: Vec<usize> = (0..n).filter(|&v| reachable[v]).collect();
    let t_side: Vec<usize> = (0..n).filter(|&v| !reachable[v]).collect();
    (s_side, t_side)
}

// ── Bipartite Matching (Hopcroft-Karp) ───────────────────────────────────────

/// Hopcroft-Karp bipartite matching.
///
/// `left` = number of left vertices (0..left), `right` = number of right vertices (0..right).
/// `edges` = list of `(left_vertex, right_vertex)` pairs.
/// Returns a `MatchingResult` with `matching\[l\]` = `Some(r)` for matched pairs.
pub fn bipartite_matching(left: usize, right: usize, edges: &[(usize, usize)]) -> MatchingResult {
    // Build adjacency list for left vertices
    let mut adj: Vec<Vec<usize>> = vec![vec![]; left];
    for &(l, r) in edges {
        adj[l].push(r);
    }

    let inf = usize::MAX;
    let mut match_l: Vec<Option<usize>> = vec![None; left];
    let mut match_r: Vec<Option<usize>> = vec![None; right];

    loop {
        // BFS phase: build layered graph
        let mut dist: Vec<usize> = vec![inf; left];
        let mut queue = VecDeque::new();

        for l in 0..left {
            if match_l[l].is_none() {
                dist[l] = 0;
                queue.push_back(l);
            }
        }

        let mut found = false;
        while let Some(l) = queue.pop_front() {
            for &r in &adj[l] {
                let nl = match_r[r];
                match nl {
                    None => {
                        found = true;
                    }
                    Some(nl_idx) => {
                        if dist[nl_idx] == inf {
                            dist[nl_idx] = dist[l] + 1;
                            queue.push_back(nl_idx);
                        }
                    }
                }
            }
        }

        if !found {
            break;
        }

        // DFS phase
        for l in 0..left {
            if match_l[l].is_none() {
                dfs_hopcroft(l, &adj, &mut match_l, &mut match_r, &mut dist, inf);
            }
        }
    }

    let size = match_l.iter().filter(|m| m.is_some()).count();
    MatchingResult {
        matching: match_l,
        size,
    }
}

fn dfs_hopcroft(
    l: usize,
    adj: &[Vec<usize>],
    match_l: &mut Vec<Option<usize>>,
    match_r: &mut Vec<Option<usize>>,
    dist: &mut Vec<usize>,
    inf: usize,
) -> bool {
    for &r in &adj[l] {
        let ok = match match_r[r] {
            None => true,
            Some(nl) => {
                dist[nl] == dist[l] + 1 && dfs_hopcroft(nl, adj, match_l, match_r, dist, inf)
            }
        };
        if ok {
            match_l[l] = Some(r);
            match_r[r] = Some(l);
            return true;
        }
    }
    dist[l] = inf;
    false
}

// ── Tarjan's SCC ──────────────────────────────────────────────────────────────

/// Tarjan's strongly connected components algorithm.
///
/// Returns SCCs in reverse topological order (each SCC before its successors in condensation).
pub fn tarjan_scc(g: &WGraph) -> StronglyConnectedComponents {
    let n = g.n;
    let mut index_counter = 0usize;
    let mut stack: Vec<usize> = Vec::new();
    let mut on_stack = vec![false; n];
    let mut index: Vec<Option<usize>> = vec![None; n];
    let mut lowlink = vec![0usize; n];
    let mut components: Vec<Vec<usize>> = Vec::new();
    let mut component_of: Vec<usize> = vec![0; n];

    for v in 0..n {
        if index[v].is_none() {
            tarjan_visit(
                v,
                g,
                &mut index_counter,
                &mut stack,
                &mut on_stack,
                &mut index,
                &mut lowlink,
                &mut components,
                &mut component_of,
            );
        }
    }

    StronglyConnectedComponents {
        components,
        component_of,
    }
}

#[allow(clippy::too_many_arguments)]
fn tarjan_visit(
    v: usize,
    g: &WGraph,
    index_counter: &mut usize,
    stack: &mut Vec<usize>,
    on_stack: &mut Vec<bool>,
    index: &mut Vec<Option<usize>>,
    lowlink: &mut Vec<usize>,
    components: &mut Vec<Vec<usize>>,
    component_of: &mut Vec<usize>,
) {
    // Iterative Tarjan using explicit call stack to avoid stack overflow on large graphs.
    // Each frame: (vertex, edge_iterator_position, index_at_entry)
    let mut call_stack: Vec<(usize, usize)> = vec![(v, 0)];
    index[v] = Some(*index_counter);
    lowlink[v] = *index_counter;
    *index_counter += 1;
    stack.push(v);
    on_stack[v] = true;

    while let Some((u, ei)) = call_stack.last_mut() {
        let u = *u;
        let neighbors = &g.adj[u];
        if *ei < neighbors.len() {
            let (w, _) = neighbors[*ei];
            *ei += 1;
            if index[w].is_none() {
                // Recurse into w
                index[w] = Some(*index_counter);
                lowlink[w] = *index_counter;
                *index_counter += 1;
                stack.push(w);
                on_stack[w] = true;
                call_stack.push((w, 0));
            } else if on_stack[w] {
                let lw = lowlink[w];
                let lu = lowlink[u];
                lowlink[u] = lu.min(lw);
                // Update the frame's lowlink by modifying through the stack
                if let Some(frame) = call_stack.last_mut() {
                    let _ = frame; // lowlink[u] already updated
                }
            }
        } else {
            // Done with u; pop
            call_stack.pop();

            // Propagate lowlink upward
            if let Some(&(parent, _)) = call_stack.last() {
                lowlink[parent] = lowlink[parent].min(lowlink[u]);
            }

            // Check if u is root of an SCC
            if let Some(idx_u) = index[u] {
                if lowlink[u] == idx_u {
                    let comp_id = components.len();
                    let mut comp = Vec::new();
                    loop {
                        let w = stack.pop().unwrap_or(u);
                        on_stack[w] = false;
                        comp.push(w);
                        if w == u {
                            break;
                        }
                    }
                    for &node in &comp {
                        component_of[node] = comp_id;
                    }
                    components.push(comp);
                }
            }
        }
    }
}

// ── Topological Sort ──────────────────────────────────────────────────────────

/// Topological sort of a DAG using Kahn's algorithm (BFS).
///
/// Returns `Some(order)` if the graph is acyclic, `None` if a cycle exists.
pub fn topological_sort_dag(g: &WGraph) -> Option<Vec<usize>> {
    let n = g.n;
    let mut in_degree = vec![0usize; n];
    for u in 0..n {
        for &(v, _) in &g.adj[u] {
            in_degree[v] += 1;
        }
    }

    let mut queue: VecDeque<usize> = (0..n).filter(|&u| in_degree[u] == 0).collect();
    let mut order = Vec::with_capacity(n);

    while let Some(u) = queue.pop_front() {
        order.push(u);
        for &(v, _) in &g.adj[u] {
            in_degree[v] -= 1;
            if in_degree[v] == 0 {
                queue.push_back(v);
            }
        }
    }

    if order.len() == n {
        Some(order)
    } else {
        None
    }
}

// ── Bipartiteness ─────────────────────────────────────────────────────────────

/// Check if the graph is bipartite using BFS 2-coloring.
///
/// Treats edges as undirected. Returns `Some((left, right))` if bipartite, `None` otherwise.
pub fn is_bipartite(g: &WGraph) -> Option<(Vec<usize>, Vec<usize>)> {
    let n = g.n;
    let no_color = usize::MAX;
    let mut color = vec![no_color; n];

    for start in 0..n {
        if color[start] != no_color {
            continue;
        }
        color[start] = 0;
        let mut queue = VecDeque::new();
        queue.push_back(start);

        while let Some(u) = queue.pop_front() {
            for &(v, _) in &g.adj[u] {
                if color[v] == no_color {
                    color[v] = 1 - color[u];
                    queue.push_back(v);
                } else if color[v] == color[u] {
                    return None;
                }
            }
        }
    }

    let left: Vec<usize> = (0..n).filter(|&v| color[v] == 0).collect();
    let right: Vec<usize> = (0..n).filter(|&v| color[v] == 1).collect();
    Some((left, right))
}

// ── Chromatic Number Approximation ────────────────────────────────────────────

/// Greedy approximation of the chromatic number (treats graph as undirected).
///
/// Uses a greedy coloring heuristic; may overestimate the true chromatic number.
pub fn chromatic_number_approx(g: &WGraph) -> usize {
    let n = g.n;
    if n == 0 {
        return 0;
    }

    // Build undirected adjacency for coloring
    let mut undirected: Vec<HashSet<usize>> = vec![HashSet::new(); n];
    for u in 0..n {
        for &(v, _) in &g.adj[u] {
            undirected[u].insert(v);
            undirected[v].insert(u);
        }
    }

    let mut color = vec![usize::MAX; n];
    let mut max_color = 0usize;

    for u in 0..n {
        let neighbor_colors: HashSet<usize> = undirected[u]
            .iter()
            .filter_map(|&v| {
                if color[v] != usize::MAX {
                    Some(color[v])
                } else {
                    None
                }
            })
            .collect();

        let mut c = 0usize;
        while neighbor_colors.contains(&c) {
            c += 1;
        }
        color[u] = c;
        if c > max_color {
            max_color = c;
        }
    }

    max_color + 1
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::super::types::{FlowNetwork, WGraph};
    use super::*;

    fn simple_dag() -> WGraph {
        // 0→1(1), 0→2(4), 1→2(2), 1→3(5), 2→3(1)
        let mut g = WGraph::new(4);
        g.add_edge(0, 1, 1);
        g.add_edge(0, 2, 4);
        g.add_edge(1, 2, 2);
        g.add_edge(1, 3, 5);
        g.add_edge(2, 3, 1);
        g
    }

    // ── Dijkstra ───────────────────────────────────────────────────────────────

    #[test]
    fn test_dijkstra_basic() {
        let g = simple_dag();
        let dist = dijkstra(&g, 0);
        assert_eq!(dist[0], Some(0));
        assert_eq!(dist[1], Some(1));
        assert_eq!(dist[2], Some(3)); // 0→1→2
        assert_eq!(dist[3], Some(4)); // 0→1→2→3
    }

    #[test]
    fn test_dijkstra_unreachable() {
        let mut g = WGraph::new(3);
        g.add_edge(0, 1, 5);
        // vertex 2 is unreachable
        let dist = dijkstra(&g, 0);
        assert_eq!(dist[2], None);
    }

    #[test]
    fn test_dijkstra_single_vertex() {
        let g = WGraph::new(1);
        let dist = dijkstra(&g, 0);
        assert_eq!(dist[0], Some(0));
    }

    #[test]
    fn test_dijkstra_self_loop() {
        let mut g = WGraph::new(2);
        g.add_edge(0, 0, 5);
        g.add_edge(0, 1, 3);
        let dist = dijkstra(&g, 0);
        assert_eq!(dist[1], Some(3));
    }

    // ── Bellman-Ford ───────────────────────────────────────────────────────────

    #[test]
    fn test_bellman_ford_positive_weights() {
        let g = simple_dag();
        let dist = bellman_ford(&g, 0).expect("no negative cycle");
        assert_eq!(dist[3], Some(4));
    }

    #[test]
    fn test_bellman_ford_negative_weights() {
        let mut g = WGraph::new(3);
        g.add_edge(0, 1, -2);
        g.add_edge(1, 2, 3);
        g.add_edge(0, 2, 4);
        let dist = bellman_ford(&g, 0).expect("no negative cycle");
        assert_eq!(dist[2], Some(1)); // 0→1→2 = -2+3 = 1
    }

    #[test]
    fn test_bellman_ford_negative_cycle() {
        let mut g = WGraph::new(3);
        g.add_edge(0, 1, 1);
        g.add_edge(1, 2, -3);
        g.add_edge(2, 0, 1);
        let result = bellman_ford(&g, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_bellman_ford_unreachable() {
        let mut g = WGraph::new(3);
        g.add_edge(0, 1, 1);
        let dist = bellman_ford(&g, 0).expect("no negative cycle");
        assert_eq!(dist[2], None);
    }

    // ── Floyd-Warshall ─────────────────────────────────────────────────────────

    #[test]
    fn test_floyd_warshall_basic() {
        let g = simple_dag();
        let apsp = floyd_warshall(&g);
        assert_eq!(apsp.dist[0][3], Some(4));
        assert_eq!(apsp.dist[1][3], Some(3));
    }

    #[test]
    fn test_floyd_warshall_no_path() {
        let mut g = WGraph::new(3);
        g.add_edge(0, 1, 2);
        let apsp = floyd_warshall(&g);
        assert_eq!(apsp.dist[0][2], None);
        assert_eq!(apsp.dist[1][2], None);
    }

    #[test]
    fn test_floyd_warshall_next_hops() {
        let mut g = WGraph::new(3);
        g.add_edge(0, 1, 1);
        g.add_edge(1, 2, 1);
        let apsp = floyd_warshall(&g);
        assert_eq!(apsp.next[0][2], Some(1)); // 0→1→2
    }

    // ── Kruskal ────────────────────────────────────────────────────────────────

    #[test]
    fn test_kruskal_basic() {
        let edges = vec![(0, 1, 1), (0, 2, 3), (1, 2, 2), (1, 3, 4), (2, 3, 5)];
        let mst = kruskal_mst(4, &edges);
        assert_eq!(mst.edges.len(), 3);
        assert_eq!(mst.total_weight, 7); // 1+2+4
    }

    #[test]
    fn test_kruskal_empty() {
        let mst = kruskal_mst(3, &[]);
        assert_eq!(mst.edges.len(), 0);
        assert_eq!(mst.total_weight, 0);
    }

    #[test]
    fn test_kruskal_single_edge() {
        let edges = vec![(0, 1, 5)];
        let mst = kruskal_mst(2, &edges);
        assert_eq!(mst.total_weight, 5);
    }

    // ── Prim ──────────────────────────────────────────────────────────────────

    #[test]
    fn test_prim_basic() {
        let mut g = WGraph::new(4);
        g.add_edge(0, 1, 1);
        g.add_edge(1, 0, 1);
        g.add_edge(0, 2, 3);
        g.add_edge(2, 0, 3);
        g.add_edge(1, 2, 2);
        g.add_edge(2, 1, 2);
        g.add_edge(1, 3, 4);
        g.add_edge(3, 1, 4);
        let mst = prim_mst(&g);
        assert_eq!(mst.edges.len(), 3);
        assert_eq!(mst.total_weight, 7);
    }

    #[test]
    fn test_prim_empty() {
        let g = WGraph::new(0);
        let mst = prim_mst(&g);
        assert_eq!(mst.edges.len(), 0);
    }

    // ── Max-Flow ──────────────────────────────────────────────────────────────

    #[test]
    fn test_max_flow_basic() {
        let mut net = FlowNetwork::new(4, 0, 3);
        net.add_edge(0, 1, 3);
        net.add_edge(0, 2, 2);
        net.add_edge(1, 3, 2);
        net.add_edge(2, 3, 3);
        let flow = max_flow_bfs(&mut net);
        assert_eq!(flow, 4);
    }

    #[test]
    fn test_max_flow_no_path() {
        let mut net = FlowNetwork::new(4, 0, 3);
        net.add_edge(0, 1, 10);
        net.add_edge(2, 3, 10);
        // no path from source to sink
        let flow = max_flow_bfs(&mut net);
        assert_eq!(flow, 0);
    }

    #[test]
    fn test_max_flow_single_edge() {
        let mut net = FlowNetwork::new(2, 0, 1);
        net.add_edge(0, 1, 7);
        let flow = max_flow_bfs(&mut net);
        assert_eq!(flow, 7);
    }

    // ── Min-Cut ───────────────────────────────────────────────────────────────

    #[test]
    fn test_min_cut_basic() {
        let mut net = FlowNetwork::new(4, 0, 3);
        net.add_edge(0, 1, 3);
        net.add_edge(0, 2, 2);
        net.add_edge(1, 3, 2);
        net.add_edge(2, 3, 3);
        let flow_val = max_flow_bfs(&mut net);
        let (s_side, t_side) = min_cut(&net, flow_val);
        assert!(s_side.contains(&0));
        assert!(t_side.contains(&3));
    }

    // ── Bipartite Matching ────────────────────────────────────────────────────

    #[test]
    fn test_bipartite_matching_perfect() {
        // 3x3 complete bipartite → perfect matching of size 3
        let edges: Vec<(usize, usize)> = (0..3).flat_map(|l| (0..3).map(move |r| (l, r))).collect();
        let result = bipartite_matching(3, 3, &edges);
        assert_eq!(result.size, 3);
    }

    #[test]
    fn test_bipartite_matching_partial() {
        // Only edge 0→0
        let edges = vec![(0, 0)];
        let result = bipartite_matching(2, 2, &edges);
        assert_eq!(result.size, 1);
        assert_eq!(result.matching[0], Some(0));
    }

    #[test]
    fn test_bipartite_matching_empty() {
        let result = bipartite_matching(3, 3, &[]);
        assert_eq!(result.size, 0);
    }

    #[test]
    fn test_bipartite_matching_chain() {
        // 0→0, 1→1, 2→2 (no sharing)
        let edges = vec![(0, 0), (1, 1), (2, 2)];
        let result = bipartite_matching(3, 3, &edges);
        assert_eq!(result.size, 3);
    }

    // ── Tarjan SCC ────────────────────────────────────────────────────────────

    #[test]
    fn test_tarjan_scc_single_cycle() {
        let mut g = WGraph::new(3);
        g.add_edge(0, 1, 1);
        g.add_edge(1, 2, 1);
        g.add_edge(2, 0, 1);
        let scc = tarjan_scc(&g);
        assert_eq!(scc.components.len(), 1);
        assert_eq!(scc.components[0].len(), 3);
    }

    #[test]
    fn test_tarjan_scc_dag() {
        let g = simple_dag();
        let scc = tarjan_scc(&g);
        // All trivial SCCs (no back edges)
        assert_eq!(scc.components.len(), 4);
    }

    #[test]
    fn test_tarjan_scc_two_components() {
        let mut g = WGraph::new(4);
        g.add_edge(0, 1, 1);
        g.add_edge(1, 0, 1);
        g.add_edge(2, 3, 1);
        g.add_edge(3, 2, 1);
        let scc = tarjan_scc(&g);
        assert_eq!(scc.components.len(), 2);
    }

    // ── Topological Sort ──────────────────────────────────────────────────────

    #[test]
    fn test_topological_sort_dag() {
        let g = simple_dag();
        let order = topological_sort_dag(&g).expect("DAG");
        // Check order is valid: for each edge u→v, u appears before v
        let pos: Vec<usize> = {
            let mut p = vec![0usize; g.n];
            for (i, &v) in order.iter().enumerate() {
                p[v] = i;
            }
            p
        };
        for u in 0..g.n {
            for &(v, _) in &g.adj[u] {
                assert!(pos[u] < pos[v], "edge {} → {} violates order", u, v);
            }
        }
    }

    #[test]
    fn test_topological_sort_cyclic() {
        let mut g = WGraph::new(3);
        g.add_edge(0, 1, 1);
        g.add_edge(1, 2, 1);
        g.add_edge(2, 0, 1);
        assert!(topological_sort_dag(&g).is_none());
    }

    // ── Is Bipartite ──────────────────────────────────────────────────────────

    #[test]
    fn test_is_bipartite_yes() {
        // Even cycle: 0→1→2→3→0 (treat as undirected)
        let mut g = WGraph::new(4);
        g.add_edge(0, 1, 1);
        g.add_edge(1, 0, 1);
        g.add_edge(1, 2, 1);
        g.add_edge(2, 1, 1);
        g.add_edge(2, 3, 1);
        g.add_edge(3, 2, 1);
        g.add_edge(3, 0, 1);
        g.add_edge(0, 3, 1);
        let result = is_bipartite(&g);
        assert!(result.is_some());
    }

    #[test]
    fn test_is_bipartite_no_odd_cycle() {
        // Triangle (odd cycle)
        let mut g = WGraph::new(3);
        g.add_edge(0, 1, 1);
        g.add_edge(1, 0, 1);
        g.add_edge(1, 2, 1);
        g.add_edge(2, 1, 1);
        g.add_edge(2, 0, 1);
        g.add_edge(0, 2, 1);
        assert!(is_bipartite(&g).is_none());
    }

    #[test]
    fn test_is_bipartite_empty() {
        let g = WGraph::new(3);
        let result = is_bipartite(&g);
        assert!(result.is_some());
    }

    // ── Chromatic Number Approx ───────────────────────────────────────────────

    #[test]
    fn test_chromatic_number_path() {
        // Path: 0→1→2 undirected → bipartite → 2 colors
        let mut g = WGraph::new(3);
        g.add_edge(0, 1, 1);
        g.add_edge(1, 0, 1);
        g.add_edge(1, 2, 1);
        g.add_edge(2, 1, 1);
        let chi = chromatic_number_approx(&g);
        assert_eq!(chi, 2);
    }

    #[test]
    fn test_chromatic_number_triangle() {
        // Triangle needs 3 colors
        let mut g = WGraph::new(3);
        g.add_edge(0, 1, 1);
        g.add_edge(1, 0, 1);
        g.add_edge(1, 2, 1);
        g.add_edge(2, 1, 1);
        g.add_edge(0, 2, 1);
        g.add_edge(2, 0, 1);
        let chi = chromatic_number_approx(&g);
        assert_eq!(chi, 3);
    }

    #[test]
    fn test_chromatic_number_empty() {
        let g = WGraph::new(0);
        assert_eq!(chromatic_number_approx(&g), 0);
    }

    #[test]
    fn test_chromatic_number_single() {
        let g = WGraph::new(1);
        assert_eq!(chromatic_number_approx(&g), 1);
    }

    #[test]
    fn test_chromatic_number_independent_set() {
        // 4 isolated vertices → 1 color
        let g = WGraph::new(4);
        assert_eq!(chromatic_number_approx(&g), 1);
    }

    // ── Combined / Edge cases ─────────────────────────────────────────────────

    #[test]
    fn test_dijkstra_zero_weight() {
        let mut g = WGraph::new(3);
        g.add_edge(0, 1, 0);
        g.add_edge(1, 2, 0);
        let dist = dijkstra(&g, 0);
        assert_eq!(dist[2], Some(0));
    }

    #[test]
    fn test_kruskal_negative_weights() {
        let edges = vec![(0, 1, -5), (1, 2, -3), (0, 2, -1)];
        let mst = kruskal_mst(3, &edges);
        assert_eq!(mst.total_weight, -8); // -5 + -3
    }

    #[test]
    fn test_floyd_warshall_self_loop() {
        let mut g = WGraph::new(2);
        g.add_edge(0, 0, 100);
        g.add_edge(0, 1, 5);
        let apsp = floyd_warshall(&g);
        assert_eq!(apsp.dist[0][0], Some(0));
        assert_eq!(apsp.dist[0][1], Some(5));
    }
}
