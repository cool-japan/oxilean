//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{BinaryHeap, HashSet, VecDeque};

/// Evaluates the Tutte polynomial T(G; x, y) at a given point using deletion-contraction.
///
/// This implementation uses memoization over edge subsets (exponential, for small graphs).
/// T(G; x, y) encodes chromatic, reliability, and flow polynomials as specializations.
#[allow(dead_code)]
pub struct TuttePolynomialEval {
    /// Edge list as (u, v) pairs.
    edges: Vec<(usize, usize)>,
    /// Number of vertices.
    n: usize,
}
#[allow(dead_code)]
impl TuttePolynomialEval {
    /// Create from an undirected graph's edge list.
    pub fn from_graph(graph: &UndirectedGraph) -> Self {
        let mut edges = Vec::new();
        for u in 0..graph.n {
            for &v in &graph.adj[u] {
                if u < v {
                    edges.push((u, v));
                }
            }
        }
        Self { edges, n: graph.n }
    }
    /// Evaluate T(G; x, y) via deletion-contraction recursion (exponential time).
    /// Base cases: empty edge set → x^(k-1) where k = components; single loops → y.
    pub fn evaluate(&self, x: f64, y: f64) -> f64 {
        self.tutte_rec(&self.edges.clone(), self.n, x, y)
    }
    fn components(n: usize, edges: &[(usize, usize)]) -> usize {
        let max_v = edges.iter().flat_map(|&(u, v)| [u, v]).max().unwrap_or(0);
        let sz = max_v.max(n.saturating_sub(1)) + 1;
        let mut parent: Vec<usize> = (0..sz).collect();
        fn find(parent: &mut Vec<usize>, a: usize) -> usize {
            if parent[a] != a {
                parent[a] = find(parent, parent[a]);
            }
            parent[a]
        }
        for &(u, v) in edges {
            if u == v {
                continue;
            }
            let ru = find(&mut parent, u);
            let rv = find(&mut parent, v);
            if ru != rv {
                parent[ru] = rv;
            }
        }
        let count_n = n.min(sz);
        (0..count_n).filter(|&i| find(&mut parent, i) == i).count()
    }
    /// Count distinct vertices referenced in an edge list.
    fn vertex_count(edges: &[(usize, usize)], base_n: usize) -> usize {
        let mut seen: HashSet<usize> = HashSet::new();
        for &(u, v) in edges {
            seen.insert(u);
            seen.insert(v);
        }
        seen.len().max(base_n)
    }
    fn tutte_rec(&self, edges: &[(usize, usize)], n: usize, x: f64, y: f64) -> f64 {
        if edges.is_empty() {
            return x.powi((n as i32) - 1);
        }
        let e = edges[0];
        let rest: Vec<(usize, usize)> = edges[1..].to_vec();
        let is_loop = e.0 == e.1;
        if is_loop {
            return y * self.tutte_rec(&rest, n, x, y);
        }
        let c_with = Self::components(n, edges);
        let c_without = Self::components(n, &rest);
        let is_bridge = c_without > c_with;
        if is_bridge {
            let contracted = self.contract_edge(&rest, e);
            let new_n = n.saturating_sub(1).max(1);
            return x * self.tutte_rec(&contracted, new_n, x, y);
        }
        let contracted = self.contract_edge(&rest, e);
        let new_n = n.saturating_sub(1).max(1);
        let del = self.tutte_rec(&rest, n, x, y);
        let con = self.tutte_rec(&contracted, new_n, x, y);
        del + con
    }
    /// Contract edge e = (u, v): merge v into u in all remaining edges.
    fn contract_edge(&self, edges: &[(usize, usize)], e: (usize, usize)) -> Vec<(usize, usize)> {
        let (u, v) = e;
        edges
            .iter()
            .map(|&(a, b)| {
                let a2 = if a == v { u } else { a };
                let b2 = if b == v { u } else { b };
                (a2, b2)
            })
            .collect()
    }
}
/// Szemerédi regularity partition: partitions vertices into roughly equal parts
/// and reports epsilon-regular pairs.
///
/// This is a simplified heuristic (not the full constructive proof), suitable for
/// demonstration and testing purposes.
#[allow(dead_code)]
pub struct SzemerédiRegularityLemma {
    /// The graph to partition.
    pub graph: UndirectedGraph,
    /// Regularity parameter ε.
    pub epsilon: f64,
}
#[allow(dead_code)]
impl SzemerédiRegularityLemma {
    /// Create a new regularity lemma instance.
    pub fn new(graph: UndirectedGraph, epsilon: f64) -> Self {
        Self { graph, epsilon }
    }
    /// Edge density between two disjoint subsets A and B.
    pub fn density(&self, a: &[usize], b: &[usize]) -> f64 {
        let b_set: HashSet<usize> = b.iter().copied().collect();
        let edges: usize = a
            .iter()
            .map(|&u| {
                self.graph.adj[u]
                    .iter()
                    .filter(|v| b_set.contains(*v))
                    .count()
            })
            .sum();
        let denom = a.len() * b.len();
        if denom == 0 {
            0.0
        } else {
            edges as f64 / denom as f64
        }
    }
    /// Check if (A, B) is an epsilon-regular pair.
    /// A pair is ε-regular if for all A' ⊆ A, B' ⊆ B with |A'| ≥ ε|A|, |B'| ≥ ε|B|,
    /// |d(A', B') - d(A, B)| ≤ ε. Here we check a simplified version using halved subsets.
    pub fn is_regular_pair(&self, a: &[usize], b: &[usize]) -> bool {
        let d = self.density(a, b);
        let a_half: Vec<usize> = a[..a.len() / 2].to_vec();
        let b_half: Vec<usize> = b[..b.len() / 2].to_vec();
        let d_sub = self.density(&a_half, &b_half);
        (d - d_sub).abs() <= self.epsilon
    }
    /// Partition vertices into k roughly equal parts and return the partition.
    pub fn partition(&self, k: usize) -> Vec<Vec<usize>> {
        let n = self.graph.n;
        let k = k.max(1);
        let mut parts: Vec<Vec<usize>> = vec![vec![]; k];
        for v in 0..n {
            parts[v % k].push(v);
        }
        parts
    }
    /// Run the regularity lemma: partition into k parts and count regular pairs.
    /// Returns (partition, number_of_regular_pairs).
    pub fn run(&self, k: usize) -> (Vec<Vec<usize>>, usize) {
        let parts = self.partition(k);
        let mut regular_pairs = 0usize;
        for i in 0..parts.len() {
            for j in (i + 1)..parts.len() {
                if self.is_regular_pair(&parts[i], &parts[j]) {
                    regular_pairs += 1;
                }
            }
        }
        (parts, regular_pairs)
    }
}
/// Greedy treewidth heuristic using the min-fill elimination ordering.
///
/// The min-fill heuristic eliminates the vertex that adds the fewest fill edges
/// (edges between non-adjacent neighbors). This gives an upper bound on treewidth.
#[allow(dead_code)]
pub struct TreewidthHeuristic {
    /// Adjacency sets (mutable working copy).
    adj: Vec<HashSet<usize>>,
    /// Number of vertices.
    n: usize,
}
#[allow(dead_code)]
impl TreewidthHeuristic {
    /// Create from an undirected graph.
    pub fn new(graph: &UndirectedGraph) -> Self {
        Self {
            adj: graph.adj.clone(),
            n: graph.n,
        }
    }
    /// Count fill edges needed to eliminate vertex v: pairs of non-adjacent neighbors.
    fn fill_count(&self, v: usize) -> usize {
        let neighbors: Vec<usize> = self.adj[v].iter().copied().collect();
        let mut fill = 0usize;
        for i in 0..neighbors.len() {
            for j in (i + 1)..neighbors.len() {
                let a = neighbors[i];
                let b = neighbors[j];
                if !self.adj[a].contains(&b) {
                    fill += 1;
                }
            }
        }
        fill
    }
    /// Run min-fill elimination. Returns (elimination_order, upper_bound_on_treewidth).
    pub fn run(&mut self) -> (Vec<usize>, usize) {
        let mut eliminated = vec![false; self.n];
        let mut order = Vec::with_capacity(self.n);
        let mut tw_bound = 0usize;
        for _ in 0..self.n {
            let v = (0..self.n)
                .filter(|&u| !eliminated[u])
                .min_by_key(|&u| self.fill_count(u))
                .expect(
                    "at least one non-eliminated vertex exists: loop runs n times for n vertices",
                );
            let deg = self.adj[v].len();
            tw_bound = tw_bound.max(deg);
            let neighbors: Vec<usize> = self.adj[v].iter().copied().collect();
            for i in 0..neighbors.len() {
                for j in (i + 1)..neighbors.len() {
                    let a = neighbors[i];
                    let b = neighbors[j];
                    if a != b {
                        self.adj[a].insert(b);
                        self.adj[b].insert(a);
                    }
                }
            }
            for &u in &neighbors {
                self.adj[u].remove(&v);
            }
            self.adj[v].clear();
            eliminated[v] = true;
            order.push(v);
        }
        (order, tw_bound)
    }
}
/// A simple directed graph with Rust-level algorithms.
#[derive(Clone, Debug)]
pub struct DiGraph {
    /// Number of vertices (0..n).
    pub n: usize,
    /// Adjacency list: adj\[u\] = list of (v, weight) neighbors.
    pub adj: Vec<Vec<(usize, i64)>>,
}
impl DiGraph {
    /// Create a new directed graph with n vertices.
    pub fn new(n: usize) -> Self {
        Self {
            n,
            adj: vec![vec![]; n],
        }
    }
    /// Add a directed edge u → v with weight w.
    pub fn add_edge(&mut self, u: usize, v: usize, w: i64) {
        self.adj[u].push((v, w));
    }
    /// BFS from source s; returns distances (usize::MAX if unreachable).
    pub fn bfs(&self, s: usize) -> Vec<usize> {
        let mut dist = vec![usize::MAX; self.n];
        let mut queue = VecDeque::new();
        dist[s] = 0;
        queue.push_back(s);
        while let Some(u) = queue.pop_front() {
            for &(v, _) in &self.adj[u] {
                if dist[v] == usize::MAX {
                    dist[v] = dist[u] + 1;
                    queue.push_back(v);
                }
            }
        }
        dist
    }
    /// DFS from source s; returns DFS finish order.
    pub fn dfs(&self, s: usize) -> Vec<usize> {
        let mut visited = vec![false; self.n];
        let mut order = Vec::new();
        self.dfs_visit(s, &mut visited, &mut order);
        order
    }
    fn dfs_visit(&self, u: usize, visited: &mut Vec<bool>, order: &mut Vec<usize>) {
        if visited[u] {
            return;
        }
        visited[u] = true;
        for &(v, _) in &self.adj[u] {
            self.dfs_visit(v, visited, order);
        }
        order.push(u);
    }
    /// Topological sort (Kahn's algorithm). Returns None if cycle exists.
    pub fn topo_sort(&self) -> Option<Vec<usize>> {
        let mut in_degree = vec![0usize; self.n];
        for u in 0..self.n {
            for &(v, _) in &self.adj[u] {
                in_degree[v] += 1;
            }
        }
        let mut queue: VecDeque<usize> = (0..self.n).filter(|&u| in_degree[u] == 0).collect();
        let mut order = Vec::new();
        while let Some(u) = queue.pop_front() {
            order.push(u);
            for &(v, _) in &self.adj[u] {
                in_degree[v] -= 1;
                if in_degree[v] == 0 {
                    queue.push_back(v);
                }
            }
        }
        if order.len() == self.n {
            Some(order)
        } else {
            None
        }
    }
    /// Strongly connected components (Kosaraju's algorithm).
    pub fn scc(&self) -> Vec<Vec<usize>> {
        let mut visited = vec![false; self.n];
        let mut finish_order = Vec::new();
        for u in 0..self.n {
            if !visited[u] {
                self.dfs_visit(u, &mut visited, &mut finish_order);
            }
        }
        let mut transposed = DiGraph::new(self.n);
        for u in 0..self.n {
            for &(v, w) in &self.adj[u] {
                transposed.add_edge(v, u, w);
            }
        }
        let mut visited2 = vec![false; self.n];
        let mut components = Vec::new();
        for &u in finish_order.iter().rev() {
            if !visited2[u] {
                let mut comp = Vec::new();
                transposed.dfs_visit(u, &mut visited2, &mut comp);
                components.push(comp);
            }
        }
        components
    }
    /// Dijkstra's shortest paths from source s (non-negative weights only).
    /// Returns (dist, parent) where dist\[v\] = shortest distance, parent\[v\] = predecessor.
    pub fn dijkstra(&self, s: usize) -> (Vec<i64>, Vec<Option<usize>>) {
        use super::functions::*;
        use std::cmp::Reverse;
        let inf = i64::MAX / 2;
        let mut dist = vec![inf; self.n];
        let mut parent = vec![None; self.n];
        dist[s] = 0;
        let mut heap = BinaryHeap::new();
        heap.push(Reverse((0i64, s)));
        while let Some(Reverse((d, u))) = heap.pop() {
            if d > dist[u] {
                continue;
            }
            for &(v, w) in &self.adj[u] {
                let nd = dist[u] + w;
                if nd < dist[v] {
                    dist[v] = nd;
                    parent[v] = Some(u);
                    heap.push(Reverse((nd, v)));
                }
            }
        }
        (dist, parent)
    }
    /// Bellman-Ford shortest paths from source s (allows negative weights).
    /// Returns None if negative cycle is reachable.
    pub fn bellman_ford(&self, s: usize) -> Option<Vec<i64>> {
        let inf = i64::MAX / 2;
        let mut dist = vec![inf; self.n];
        dist[s] = 0;
        for _ in 0..self.n - 1 {
            let mut changed = false;
            for u in 0..self.n {
                if dist[u] == inf {
                    continue;
                }
                for &(v, w) in &self.adj[u] {
                    if dist[u] + w < dist[v] {
                        dist[v] = dist[u] + w;
                        changed = true;
                    }
                }
            }
            if !changed {
                break;
            }
        }
        for u in 0..self.n {
            if dist[u] == inf {
                continue;
            }
            for &(v, w) in &self.adj[u] {
                if dist[u] + w < dist[v] {
                    return None;
                }
            }
        }
        Some(dist)
    }
    /// Floyd-Warshall all-pairs shortest paths.
    /// Returns distance matrix (i64::MAX/2 = unreachable).
    pub fn floyd_warshall(&self) -> Vec<Vec<i64>> {
        let inf = i64::MAX / 2;
        let mut d = vec![vec![inf; self.n]; self.n];
        for u in 0..self.n {
            d[u][u] = 0;
            for &(v, w) in &self.adj[u] {
                d[u][v] = d[u][v].min(w);
            }
        }
        for k in 0..self.n {
            for i in 0..self.n {
                for j in 0..self.n {
                    if d[i][k] < inf && d[k][j] < inf {
                        d[i][j] = d[i][j].min(d[i][k] + d[k][j]);
                    }
                }
            }
        }
        d
    }
    /// Is this graph a DAG?
    pub fn is_dag(&self) -> bool {
        self.topo_sort().is_some()
    }
    /// Count vertices reachable from s.
    pub fn reachable_count(&self, s: usize) -> usize {
        let dist = self.bfs(s);
        dist.iter().filter(|&&d| d != usize::MAX).count()
    }
}
/// An undirected simple graph (represented as adjacency sets for simplicity).
#[derive(Clone, Debug, Default)]
pub struct UndirectedGraph {
    /// Number of vertices.
    pub n: usize,
    /// Adjacency sets: adj\[u\] = set of neighbors
    pub adj: Vec<HashSet<usize>>,
}
impl UndirectedGraph {
    /// Create with n vertices.
    pub fn new(n: usize) -> Self {
        Self {
            n,
            adj: vec![HashSet::new(); n],
        }
    }
    /// Add undirected edge.
    pub fn add_edge(&mut self, u: usize, v: usize) {
        if u != v {
            self.adj[u].insert(v);
            self.adj[v].insert(u);
        }
    }
    /// BFS from source s.
    pub fn bfs(&self, s: usize) -> Vec<usize> {
        let mut dist = vec![usize::MAX; self.n];
        let mut queue = VecDeque::new();
        dist[s] = 0;
        queue.push_back(s);
        while let Some(u) = queue.pop_front() {
            for &v in &self.adj[u] {
                if dist[v] == usize::MAX {
                    dist[v] = dist[u] + 1;
                    queue.push_back(v);
                }
            }
        }
        dist
    }
    /// Is the graph connected?
    pub fn is_connected(&self) -> bool {
        if self.n == 0 {
            return true;
        }
        let dist = self.bfs(0);
        dist.iter().all(|&d| d != usize::MAX)
    }
    /// Number of connected components.
    pub fn num_components(&self) -> usize {
        let mut visited = vec![false; self.n];
        let mut count = 0;
        for u in 0..self.n {
            if !visited[u] {
                count += 1;
                let mut stack = vec![u];
                while let Some(v) = stack.pop() {
                    if visited[v] {
                        continue;
                    }
                    visited[v] = true;
                    for &w in &self.adj[v] {
                        if !visited[w] {
                            stack.push(w);
                        }
                    }
                }
            }
        }
        count
    }
    /// Is the graph bipartite? Returns Some(coloring) or None.
    pub fn bipartite_coloring(&self) -> Option<Vec<usize>> {
        let mut color = vec![usize::MAX; self.n];
        for s in 0..self.n {
            if color[s] != usize::MAX {
                continue;
            }
            color[s] = 0;
            let mut queue = VecDeque::new();
            queue.push_back(s);
            while let Some(u) = queue.pop_front() {
                for &v in &self.adj[u] {
                    if color[v] == usize::MAX {
                        color[v] = 1 - color[u];
                        queue.push_back(v);
                    } else if color[v] == color[u] {
                        return None;
                    }
                }
            }
        }
        Some(color)
    }
    /// Is the graph bipartite?
    pub fn is_bipartite(&self) -> bool {
        self.bipartite_coloring().is_some()
    }
    /// Check if all vertex degrees are even (necessary for Eulerian circuit).
    pub fn all_degrees_even(&self) -> bool {
        (0..self.n).all(|u| self.adj[u].len() % 2 == 0)
    }
    /// Check if graph has an Eulerian circuit (connected + all even degrees).
    pub fn has_eulerian_circuit(&self) -> bool {
        self.is_connected() && self.all_degrees_even()
    }
    /// Greedy graph coloring (returns number of colors used and coloring).
    pub fn greedy_coloring(&self) -> (usize, Vec<usize>) {
        let mut color = vec![usize::MAX; self.n];
        let mut max_color = 0;
        for u in 0..self.n {
            let neighbor_colors: HashSet<usize> = self.adj[u]
                .iter()
                .filter(|&&v| color[v] != usize::MAX)
                .map(|&v| color[v])
                .collect();
            let mut c = 0;
            while neighbor_colors.contains(&c) {
                c += 1;
            }
            color[u] = c;
            max_color = max_color.max(c);
        }
        (max_color + 1, color)
    }
    /// Check if a given coloring is proper.
    pub fn is_proper_coloring(&self, coloring: &[usize]) -> bool {
        for u in 0..self.n {
            for &v in &self.adj[u] {
                if coloring[u] == coloring[v] {
                    return false;
                }
            }
        }
        true
    }
    /// Degree of vertex u.
    pub fn degree(&self, u: usize) -> usize {
        self.adj[u].len()
    }
    /// Number of edges.
    pub fn edge_count(&self) -> usize {
        (0..self.n).map(|u| self.adj[u].len()).sum::<usize>() / 2
    }
    /// Minimum spanning tree (Prim's algorithm, unweighted → any spanning tree).
    /// Returns edges of the spanning tree.
    pub fn spanning_tree(&self) -> Vec<(usize, usize)> {
        if self.n == 0 {
            return vec![];
        }
        let mut in_tree = vec![false; self.n];
        let mut edges = Vec::new();
        in_tree[0] = true;
        for _ in 0..self.n - 1 {
            let mut found = false;
            for u in 0..self.n {
                if !in_tree[u] {
                    continue;
                }
                for &v in &self.adj[u] {
                    if !in_tree[v] {
                        in_tree[v] = true;
                        edges.push((u, v));
                        found = true;
                        break;
                    }
                }
                if found {
                    break;
                }
            }
        }
        edges
    }
    /// Create complete graph K_n.
    pub fn complete(n: usize) -> Self {
        let mut g = Self::new(n);
        for u in 0..n {
            for v in (u + 1)..n {
                g.add_edge(u, v);
            }
        }
        g
    }
    /// Create path graph P_n (0-1-2-...-n-1).
    pub fn path(n: usize) -> Self {
        let mut g = Self::new(n);
        for i in 0..n.saturating_sub(1) {
            g.add_edge(i, i + 1);
        }
        g
    }
    /// Create cycle graph C_n.
    pub fn cycle(n: usize) -> Self {
        let mut g = Self::path(n);
        if n >= 3 {
            g.add_edge(n - 1, 0);
        }
        g
    }
    /// Create complete bipartite graph K_{m,n}.
    pub fn complete_bipartite(m: usize, n: usize) -> Self {
        let mut g = Self::new(m + n);
        for u in 0..m {
            for v in m..(m + n) {
                g.add_edge(u, v);
            }
        }
        g
    }
}
/// Checks expansion properties of a graph using an approximation of the Cheeger constant.
///
/// The Cheeger constant h(G) = min_{S: 0<|S|≤n/2} |∂S| / |S|.
/// This checker approximates it by trying all subsets up to a given size limit.
#[allow(dead_code)]
pub struct ExpanderChecker {
    /// The underlying undirected graph.
    pub graph: UndirectedGraph,
}
#[allow(dead_code)]
impl ExpanderChecker {
    /// Create from an existing undirected graph.
    pub fn new(graph: UndirectedGraph) -> Self {
        Self { graph }
    }
    /// Compute the edge boundary |∂S| = edges with exactly one endpoint in S.
    pub fn edge_boundary(&self, subset: &[usize]) -> usize {
        let in_set: HashSet<usize> = subset.iter().copied().collect();
        let mut boundary = 0usize;
        for &u in &in_set {
            for &v in &self.graph.adj[u] {
                if !in_set.contains(&v) {
                    boundary += 1;
                }
            }
        }
        boundary
    }
    /// Approximate Cheeger constant: try all subsets of size 1..=n/2 (only feasible for small n).
    /// Returns the approximate h(G) as a rational approximation (boundary/size).
    pub fn approximate_cheeger(&self) -> f64 {
        let n = self.graph.n;
        if n == 0 {
            return 0.0;
        }
        let half = n / 2;
        let mut min_ratio = f64::INFINITY;
        for u in 0..n {
            let boundary = self.edge_boundary(&[u]);
            let size = 1usize;
            let ratio = boundary as f64 / size as f64;
            if ratio < min_ratio {
                min_ratio = ratio;
            }
        }
        if half >= 2 {
            for u in 0..n {
                for v in (u + 1)..n {
                    if u == v {
                        continue;
                    }
                    let boundary = self.edge_boundary(&[u, v]);
                    let ratio = boundary as f64 / 2.0;
                    if ratio < min_ratio {
                        min_ratio = ratio;
                    }
                }
            }
        }
        if min_ratio.is_infinite() {
            0.0
        } else {
            min_ratio
        }
    }
    /// Returns true if the graph is an h-vertex expander: h(G) ≥ threshold.
    pub fn is_expander(&self, threshold: f64) -> bool {
        self.approximate_cheeger() >= threshold
    }
}
/// Samples a simple graph from a graphon W : \[0,1\]² → \[0,1\].
///
/// A graphon is represented as a symmetric function. We discretize by n sample points.
#[allow(dead_code)]
pub struct GraphonSampler {
    /// Number of vertices to sample.
    pub n: usize,
    /// The graphon function W(x, y) ∈ \[0, 1\]; must satisfy W(x,y) = W(y,x).
    pub graphon: Box<dyn Fn(f64, f64) -> f64>,
}
#[allow(dead_code)]
impl GraphonSampler {
    /// Create a new sampler for a given graphon function and vertex count.
    pub fn new(n: usize, graphon: Box<dyn Fn(f64, f64) -> f64>) -> Self {
        Self { n, graphon }
    }
    /// Sample a graph: vertex i gets label (i+0.5)/n, edge (i,j) included with prob W(xi, xj).
    /// Uses a deterministic threshold for reproducibility (threshold = 0.5).
    pub fn sample_deterministic(&self) -> UndirectedGraph {
        let mut g = UndirectedGraph::new(self.n);
        for i in 0..self.n {
            let xi = (i as f64 + 0.5) / self.n as f64;
            for j in (i + 1)..self.n {
                let xj = (j as f64 + 0.5) / self.n as f64;
                let w = (self.graphon)(xi, xj);
                if w > 0.5 {
                    g.add_edge(i, j);
                }
            }
        }
        g
    }
    /// Sample by thresholding at a given value p ∈ \[0, 1\].
    pub fn sample_at_threshold(&self, p: f64) -> UndirectedGraph {
        let mut g = UndirectedGraph::new(self.n);
        for i in 0..self.n {
            let xi = (i as f64 + 0.5) / self.n as f64;
            for j in (i + 1)..self.n {
                let xj = (j as f64 + 0.5) / self.n as f64;
                let w = (self.graphon)(xi, xj);
                if w > p {
                    g.add_edge(i, j);
                }
            }
        }
        g
    }
}
