//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use std::collections::{HashMap, HashSet, VecDeque};

/// Network reliability polynomial.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct NetworkReliability {
    pub num_nodes: usize,
    pub num_edges: usize,
    pub edge_reliability: f64,
}
impl NetworkReliability {
    #[allow(dead_code)]
    pub fn new(n: usize, m: usize, p: f64) -> Self {
        Self {
            num_nodes: n,
            num_edges: m,
            edge_reliability: p,
        }
    }
    #[allow(dead_code)]
    pub fn all_edges_work_probability(&self) -> f64 {
        self.edge_reliability.powi(self.num_edges as i32)
    }
    #[allow(dead_code)]
    pub fn no_edge_fails_probability(&self) -> f64 {
        self.all_edges_work_probability()
    }
}
/// Barabási–Albert preferential attachment model specification.
#[derive(Debug, Clone)]
pub struct BarabasiAlbert {
    /// Initial number of nodes (seed network size).
    pub m0: usize,
    /// Edges added per new node.
    pub m: usize,
    /// Total number of nodes.
    pub n: usize,
}
impl BarabasiAlbert {
    /// Create a new BA model specification.
    pub fn new(m0: usize, m: usize, n: usize) -> Self {
        BarabasiAlbert { m0, m, n }
    }
    /// Analytical power-law degree distribution P(k) ∝ k^{-γ} for k = 1..=n.
    pub fn degree_distribution(&self) -> Vec<(usize, f64)> {
        let gamma = self.power_law_exponent();
        (1..=self.n).map(|k| (k, (k as f64).powf(-gamma))).collect()
    }
    /// Power-law exponent for the BA model (γ = 3).
    pub fn power_law_exponent(&self) -> f64 {
        3.0
    }
}
/// Flow network with capacities.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FlowNetwork {
    pub num_nodes: usize,
    pub edges: Vec<(usize, usize, f64)>,
    pub source: usize,
    pub sink: usize,
}
impl FlowNetwork {
    #[allow(dead_code)]
    pub fn new(n: usize, source: usize, sink: usize) -> Self {
        Self {
            num_nodes: n,
            edges: Vec::new(),
            source,
            sink,
        }
    }
    #[allow(dead_code)]
    pub fn add_edge(&mut self, from: usize, to: usize, cap: f64) {
        self.edges.push((from, to, cap));
    }
    #[allow(dead_code)]
    pub fn max_capacity_from_source(&self) -> f64 {
        self.edges
            .iter()
            .filter(|(f, _, _)| *f == self.source)
            .map(|(_, _, c)| *c)
            .sum()
    }
    #[allow(dead_code)]
    pub fn mfmc_theorem(&self) -> String {
        "Max-flow min-cut theorem: max flow value = min cut capacity".to_string()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinkPredictionMethod {
    CommonNeighbors,
    JaccardSimilarity,
    AdamicAdar,
    ResourceAllocation,
    Katz,
    SimRank,
}
/// SIR epidemic simulation result at each timestep.
pub struct SIRTimeSeries {
    /// (S, I, R) fractions at each timestep.
    pub series: Vec<(f64, f64, f64)>,
}
/// Stochastic block model (community structure).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct StochasticBlockModel {
    pub num_communities: usize,
    pub community_sizes: Vec<usize>,
    pub p_in: f64,
    pub p_out: f64,
}
impl StochasticBlockModel {
    #[allow(dead_code)]
    pub fn new(sizes: Vec<usize>, p_in: f64, p_out: f64) -> Self {
        let k = sizes.len();
        Self {
            num_communities: k,
            community_sizes: sizes,
            p_in,
            p_out,
        }
    }
    #[allow(dead_code)]
    pub fn total_nodes(&self) -> usize {
        self.community_sizes.iter().sum()
    }
    #[allow(dead_code)]
    pub fn detectability_threshold(&self) -> bool {
        let n = self.total_nodes() as f64;
        let k = self.num_communities as f64;
        let snr = (self.p_in - self.p_out) * (self.p_in - self.p_out)
            / (self.p_in + (k - 1.0) * self.p_out)
            * n;
        snr > k * k
    }
}
/// PageRank computation struct.
#[derive(Debug, Clone)]
pub struct PageRank {
    /// Damping factor d ∈ (0, 1), typically 0.85.
    pub damping: f64,
    /// Number of nodes.
    pub nodes: usize,
}
impl PageRank {
    /// Create a new PageRank instance.
    pub fn new(damping: f64, nodes: usize) -> Self {
        PageRank { damping, nodes }
    }
    /// Compute PageRank scores for a directed graph.
    pub fn rank(&self, g: &DiGraph) -> Vec<f64> {
        pagerank(
            g,
            &PageRankConfig {
                damping: self.damping,
                ..Default::default()
            },
        )
    }
    /// Upper bound on convergence error after `t` iterations: ε ≤ d^t.
    pub fn convergence_bound(&self, iterations: u32) -> f64 {
        self.damping.powi(iterations as i32)
    }
}
/// A directed graph represented as an adjacency list.
#[derive(Clone)]
pub struct DiGraph {
    /// Number of vertices.
    pub n: usize,
    /// `out_adj\[v\]` = set of vertices that v points to.
    pub out_adj: Vec<HashSet<usize>>,
    /// `in_adj\[v\]` = set of vertices that point to v.
    pub in_adj: Vec<HashSet<usize>>,
}
impl DiGraph {
    /// Create an empty directed graph with `n` vertices.
    pub fn new(n: usize) -> Self {
        DiGraph {
            n,
            out_adj: vec![HashSet::new(); n],
            in_adj: vec![HashSet::new(); n],
        }
    }
    /// Add a directed edge from u to v.
    pub fn add_edge(&mut self, u: usize, v: usize) {
        if u < self.n && v < self.n && u != v {
            self.out_adj[u].insert(v);
            self.in_adj[v].insert(u);
        }
    }
    /// Out-degree of vertex v.
    pub fn out_degree(&self, v: usize) -> usize {
        self.out_adj[v].len()
    }
    /// In-degree of vertex v.
    pub fn in_degree(&self, v: usize) -> usize {
        self.in_adj[v].len()
    }
}
/// Watts–Strogatz small-world model specification.
#[derive(Debug, Clone)]
pub struct WattsStrogatz {
    /// Number of nodes.
    pub n: usize,
    /// Mean degree k.
    pub k: usize,
    /// Rewiring probability β ∈ \[0, 1\].
    pub beta: f64,
}
impl WattsStrogatz {
    /// Create a new WS model specification.
    pub fn new(n: usize, k: usize, beta: f64) -> Self {
        WattsStrogatz { n, k, beta }
    }
    /// Build a WS graph (ring lattice, then rewiring) and return it as a `Graph`.
    pub fn rewire_edges(&self) -> Graph {
        let n = self.n;
        let k = self.k;
        let beta = self.beta;
        let mut g = Graph::new(n);
        for i in 0..n {
            for j in 1..=(k / 2) {
                let nb = (i + j) % n;
                g.add_edge(i, nb);
            }
        }
        let mut rng_state: u64 = 12345;
        let lcg = |s: u64| -> (u64, f64) {
            let s2 = s
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            let f = (s2 >> 11) as f64 / (1u64 << 53) as f64;
            (s2, f)
        };
        if beta > 0.0 {
            for i in 0..n {
                for j in 1..=(k / 2) {
                    let nb = (i + j) % n;
                    let (ns, f) = lcg(rng_state);
                    rng_state = ns;
                    if f < beta {
                        g.adj[i].remove(&nb);
                        g.adj[nb].remove(&i);
                        let (ns2, f2) = lcg(rng_state);
                        rng_state = ns2;
                        let new_nb = (f2 * n as f64) as usize % n;
                        if new_nb != i {
                            g.add_edge(i, new_nb);
                        }
                    }
                }
            }
        }
        g
    }
    /// Compute small-world indicators: (clustering coefficient, average path length).
    pub fn small_world_property(&self) -> (f64, f64) {
        let g = self.rewire_edges();
        let cc = g.global_clustering();
        let apl = g.average_path_length();
        (cc, apl)
    }
}
/// Heterogeneous mean-field approximation.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct HeterogeneousMeanField {
    pub degree_distribution: Vec<f64>,
    pub beta: f64,
    pub gamma: f64,
}
impl HeterogeneousMeanField {
    #[allow(dead_code)]
    pub fn new(degree_dist: Vec<f64>, beta: f64, gamma: f64) -> Self {
        Self {
            degree_distribution: degree_dist,
            beta,
            gamma,
        }
    }
    #[allow(dead_code)]
    pub fn mean_degree(&self) -> f64 {
        self.degree_distribution
            .iter()
            .enumerate()
            .map(|(k, &pk)| k as f64 * pk)
            .sum()
    }
    #[allow(dead_code)]
    pub fn mean_degree_squared(&self) -> f64 {
        self.degree_distribution
            .iter()
            .enumerate()
            .map(|(k, &pk)| (k as f64) * (k as f64) * pk)
            .sum()
    }
    #[allow(dead_code)]
    pub fn epidemic_threshold(&self) -> f64 {
        let k = self.mean_degree();
        let k2 = self.mean_degree_squared();
        if k2 > 0.0 {
            self.gamma * k / (self.beta * k2)
        } else {
            f64::INFINITY
        }
    }
}
/// Result of HITS algorithm.
pub struct HitsResult {
    /// Hub scores.
    pub hub: Vec<f64>,
    /// Authority scores.
    pub authority: Vec<f64>,
}
/// Temporal graph.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TemporalGraph {
    pub num_nodes: usize,
    pub edges: Vec<TemporalEdge>,
    pub time_horizon: f64,
}
impl TemporalGraph {
    #[allow(dead_code)]
    pub fn new(n: usize, horizon: f64) -> Self {
        Self {
            num_nodes: n,
            edges: Vec::new(),
            time_horizon: horizon,
        }
    }
    #[allow(dead_code)]
    pub fn add_edge(&mut self, from: usize, to: usize, t: f64, dur: f64) {
        self.edges.push(TemporalEdge::new(from, to, t, dur));
    }
    #[allow(dead_code)]
    pub fn edges_active_at(&self, t: f64) -> Vec<&TemporalEdge> {
        self.edges.iter().filter(|e| e.is_active_at(t)).collect()
    }
    #[allow(dead_code)]
    pub fn num_edges_at(&self, t: f64) -> usize {
        self.edges_active_at(t).len()
    }
    #[allow(dead_code)]
    pub fn has_temporal_path(&self, source: usize, target: usize) -> bool {
        if source == target {
            return true;
        }
        let mut reachable = std::collections::HashSet::new();
        reachable.insert((source, 0.0f64.to_bits()));
        for edge in &self.edges {
            if reachable.iter().any(|&(node, t_bits)| {
                let t = f64::from_bits(t_bits);
                node == edge.from && t <= edge.timestamp
            }) {
                reachable.insert((edge.to, edge.timestamp.to_bits()));
            }
        }
        reachable.iter().any(|&(node, _)| node == target)
    }
}
/// Spectral graph theory: Laplacian eigenvalues.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SpectralGraphData {
    pub graph_name: String,
    pub num_nodes: usize,
    pub algebraic_connectivity: f64,
    pub spectral_gap: f64,
}
impl SpectralGraphData {
    #[allow(dead_code)]
    pub fn new(name: &str, n: usize, alg_conn: f64, gap: f64) -> Self {
        Self {
            graph_name: name.to_string(),
            num_nodes: n,
            algebraic_connectivity: alg_conn,
            spectral_gap: gap,
        }
    }
    #[allow(dead_code)]
    pub fn is_connected(&self) -> bool {
        self.algebraic_connectivity > 0.0
    }
    #[allow(dead_code)]
    pub fn cheeger_inequality_description(&self) -> String {
        format!(
            "Cheeger: h_G^2/2 <= lambda_2 <= 2 h_G for {} (h_G = edge expansion)",
            self.graph_name
        )
    }
    #[allow(dead_code)]
    pub fn expander_quality(&self) -> f64 {
        self.spectral_gap / 2.0
    }
}
/// Temporal network: edges have timestamps.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TemporalEdge {
    pub from: usize,
    pub to: usize,
    pub timestamp: f64,
    pub duration: f64,
}
impl TemporalEdge {
    #[allow(dead_code)]
    pub fn new(from: usize, to: usize, t: f64, dur: f64) -> Self {
        Self {
            from,
            to,
            timestamp: t,
            duration: dur,
        }
    }
    #[allow(dead_code)]
    pub fn is_active_at(&self, t: f64) -> bool {
        t >= self.timestamp && t < self.timestamp + self.duration
    }
}
/// A weighted, undirected network (separate from the `Graph` adjacency-list struct).
#[derive(Debug, Clone)]
pub struct Network {
    /// Number of nodes.
    pub nodes: usize,
    /// Edges as (u, v, weight).
    pub edges: Vec<(usize, usize, f64)>,
}
impl Network {
    /// Create an empty network with `nodes` nodes.
    pub fn new_network(nodes: usize) -> Self {
        Network {
            nodes,
            edges: Vec::new(),
        }
    }
    /// Add a weighted edge (u, v, weight).
    pub fn add_weighted_edge(&mut self, u: usize, v: usize, w: f64) {
        self.edges.push((u, v, w));
    }
    /// Check if the network is connected (ignores weights).
    pub fn is_connected(&self) -> bool {
        if self.nodes == 0 {
            return true;
        }
        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); self.nodes];
        for &(u, v, _) in &self.edges {
            if u < self.nodes && v < self.nodes {
                adj[u].push(v);
                adj[v].push(u);
            }
        }
        let mut visited = vec![false; self.nodes];
        let mut queue = VecDeque::new();
        queue.push_back(0usize);
        visited[0] = true;
        while let Some(node) = queue.pop_front() {
            for &nb in &adj[node] {
                if !visited[nb] {
                    visited[nb] = true;
                    queue.push_back(nb);
                }
            }
        }
        visited.iter().all(|&v| v)
    }
    /// BFS-based diameter (longest shortest path, unweighted).
    pub fn diameter(&self) -> usize {
        if self.nodes == 0 {
            return 0;
        }
        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); self.nodes];
        for &(u, v, _) in &self.edges {
            if u < self.nodes && v < self.nodes {
                adj[u].push(v);
                adj[v].push(u);
            }
        }
        let mut diam = 0usize;
        for src in 0..self.nodes {
            let mut dist = vec![usize::MAX; self.nodes];
            dist[src] = 0;
            let mut q = VecDeque::new();
            q.push_back(src);
            while let Some(u) = q.pop_front() {
                for &v in &adj[u] {
                    if dist[v] == usize::MAX {
                        dist[v] = dist[u] + 1;
                        if dist[v] > diam {
                            diam = dist[v];
                        }
                        q.push_back(v);
                    }
                }
            }
        }
        diam
    }
    /// Average shortest path length (unweighted BFS over reachable pairs).
    pub fn avg_path_length(&self) -> f64 {
        if self.nodes <= 1 {
            return 0.0;
        }
        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); self.nodes];
        for &(u, v, _) in &self.edges {
            if u < self.nodes && v < self.nodes {
                adj[u].push(v);
                adj[v].push(u);
            }
        }
        let mut total = 0usize;
        let mut count = 0usize;
        for src in 0..self.nodes {
            let mut dist = vec![usize::MAX; self.nodes];
            dist[src] = 0;
            let mut q = VecDeque::new();
            q.push_back(src);
            while let Some(u) = q.pop_front() {
                for &v in &adj[u] {
                    if dist[v] == usize::MAX {
                        dist[v] = dist[u] + 1;
                        total += dist[v];
                        count += 1;
                        q.push_back(v);
                    }
                }
            }
        }
        if count == 0 {
            0.0
        } else {
            total as f64 / count as f64
        }
    }
}
/// Cascading failure propagation model.
#[derive(Debug, Clone)]
pub struct CascadingFailure {
    /// Load threshold: a node fails when ≥ threshold fraction of its neighbors have failed.
    pub threshold: f64,
    /// Initially failed nodes.
    pub initial_failures: Vec<usize>,
}
impl CascadingFailure {
    /// Create a new cascading failure model.
    pub fn new(threshold: f64, initial_failures: Vec<usize>) -> Self {
        CascadingFailure {
            threshold,
            initial_failures,
        }
    }
    /// Propagate failures through the network.
    pub fn propagate(&self, g: &Graph) -> Vec<usize> {
        let n = g.n;
        let mut failed = vec![false; n];
        for &f in &self.initial_failures {
            if f < n {
                failed[f] = true;
            }
        }
        let mut changed = true;
        while changed {
            changed = false;
            for v in 0..n {
                if failed[v] {
                    continue;
                }
                let deg = g.degree(v);
                if deg == 0 {
                    continue;
                }
                let failed_nb = g.adj[v].iter().filter(|&&u| failed[u]).count();
                if failed_nb as f64 / deg as f64 >= self.threshold {
                    failed[v] = true;
                    changed = true;
                }
            }
        }
        (0..n).filter(|&v| failed[v]).collect()
    }
    /// Final failure size as fraction of total nodes.
    pub fn final_failure_size(&self, g: &Graph) -> f64 {
        let failed = self.propagate(g);
        if g.n == 0 {
            0.0
        } else {
            failed.len() as f64 / g.n as f64
        }
    }
}
/// Link prediction score.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LinkPredictionScore {
    pub method: LinkPredictionMethod,
    pub pair: (usize, usize),
    pub score: f64,
}
impl LinkPredictionScore {
    #[allow(dead_code)]
    pub fn new(method: LinkPredictionMethod, u: usize, v: usize, score: f64) -> Self {
        Self {
            method,
            pair: (u, v),
            score,
        }
    }
    #[allow(dead_code)]
    pub fn description(&self) -> String {
        let method_name = match &self.method {
            LinkPredictionMethod::CommonNeighbors => "Common Neighbors: |N(u) intersect N(v)|",
            LinkPredictionMethod::JaccardSimilarity => {
                "Jaccard: |N(u) intersect N(v)| / |N(u) union N(v)|"
            }
            LinkPredictionMethod::AdamicAdar => {
                "Adamic-Adar: sum 1/log(deg(w)) for w in common neighbors"
            }
            LinkPredictionMethod::ResourceAllocation => "Resource Allocation: sum 1/deg(w)",
            LinkPredictionMethod::Katz => "Katz: sum beta^l * paths_l(u,v)",
            LinkPredictionMethod::SimRank => "SimRank: recursive similarity of in-neighbors",
        };
        format!(
            "{} for ({}, {}): {}",
            method_name, self.pair.0, self.pair.1, self.score
        )
    }
}
/// PageRank parameters and computation.
pub struct PageRankConfig {
    /// Damping factor (typically 0.85).
    pub damping: f64,
    /// Maximum number of iterations.
    pub max_iter: usize,
    /// Convergence tolerance.
    pub tol: f64,
}
/// Represents a community partition.
pub struct CommunityPartition {
    /// `label\[v\]` = community id of vertex v.
    pub label: Vec<usize>,
    /// Number of communities.
    pub n_communities: usize,
}
impl CommunityPartition {
    /// Compute modularity Q of this partition on graph `g`.
    ///
    /// Q = (1/2m) * Σ_{ij} \[A_{ij} - k_i k_j / (2m)\] δ(c_i, c_j)
    pub fn modularity(&self, g: &Graph) -> f64 {
        let m = g.edge_count();
        if m == 0 {
            return 0.0;
        }
        let two_m = 2.0 * m as f64;
        let mut q = 0.0;
        for u in 0..g.n {
            for v in 0..g.n {
                if self.label[u] == self.label[v] {
                    let a_uv = if g.adj[u].contains(&v) { 1.0 } else { 0.0 };
                    q += a_uv - (g.degree(u) * g.degree(v)) as f64 / two_m;
                }
            }
        }
        q / two_m
    }
}
/// SIS epidemic: recovered nodes become susceptible again.
pub struct SISTimeSeries {
    /// (S, I) fractions at each timestep.
    pub series: Vec<(f64, f64)>,
    /// Steady-state infection fraction estimate.
    pub endemic_level: f64,
}
/// Network robustness analysis via percolation.
#[derive(Debug, Clone)]
pub struct NetworkRobustness {
    /// Fraction of nodes removed (for targeted/random attack).
    pub fraction_removed: f64,
}
impl NetworkRobustness {
    /// Create a new robustness analysis instance.
    pub fn new(fraction_removed: f64) -> Self {
        NetworkRobustness { fraction_removed }
    }
    /// Estimate the percolation threshold p_c via binary search over site percolation.
    pub fn percolation_threshold(&self, g: &Graph) -> f64 {
        let mut lo = 0.0f64;
        let mut hi = 1.0f64;
        for _ in 0..20 {
            let mid = (lo + hi) / 2.0;
            let gc = site_percolation(g, mid, 42);
            if gc > 0.5 {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        (lo + hi) / 2.0
    }
    /// Fraction of nodes in the giant component after removing `fraction_removed`.
    pub fn giant_component_fraction(&self, g: &Graph) -> f64 {
        site_percolation(g, 1.0 - self.fraction_removed, 42)
    }
}
/// Community detection configuration.
#[derive(Debug, Clone)]
pub struct CommunityDetection {
    /// Algorithm name (e.g. "louvain").
    pub algorithm: String,
    /// Expected number of communities.
    pub num_communities: usize,
}
impl CommunityDetection {
    /// Create a new community detection instance.
    pub fn new(algorithm: impl Into<String>, num_communities: usize) -> Self {
        CommunityDetection {
            algorithm: algorithm.into(),
            num_communities,
        }
    }
    /// Compute the modularity of a community partition on `g`.
    pub fn modularity(&self, g: &Graph, partition: &CommunityPartition) -> f64 {
        partition.modularity(g)
    }
    /// Perform one Louvain step and return the resulting partition.
    pub fn louvain_step(&self, g: &Graph) -> CommunityPartition {
        louvain_communities(g)
    }
}
/// Centrality measure selector.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CentralityMeasure {
    /// Fraction of edges incident to the node (normalized).
    Degree,
    /// Fraction of shortest paths passing through the node.
    Betweenness,
    /// Inverse of average shortest path distance to all nodes.
    Closeness,
    /// Centrality based on neighbors' centrality (eigenvector).
    Eigenvector,
}
impl CentralityMeasure {
    /// Compute centrality scores for all nodes in `g`.
    pub fn compute_for(&self, g: &Graph) -> Vec<f64> {
        match self {
            CentralityMeasure::Degree => degree_centrality(g),
            CentralityMeasure::Betweenness => betweenness_centrality(g),
            CentralityMeasure::Closeness => closeness_centrality(g),
            CentralityMeasure::Eigenvector => eigenvector_centrality(g, 100),
        }
    }
}
/// An undirected graph represented as an adjacency list.
#[derive(Clone)]
pub struct Graph {
    /// Number of vertices (0..n).
    pub n: usize,
    /// Adjacency list: `adj\[v\]` = set of neighbors of v.
    pub adj: Vec<HashSet<usize>>,
}
impl Graph {
    /// Create an empty graph with `n` vertices.
    pub fn new(n: usize) -> Self {
        Graph {
            n,
            adj: vec![HashSet::new(); n],
        }
    }
    /// Add an undirected edge between u and v.
    pub fn add_edge(&mut self, u: usize, v: usize) {
        if u != v && u < self.n && v < self.n {
            self.adj[u].insert(v);
            self.adj[v].insert(u);
        }
    }
    /// Return the degree of vertex v.
    pub fn degree(&self, v: usize) -> usize {
        self.adj[v].len()
    }
    /// Return the total number of edges.
    pub fn edge_count(&self) -> usize {
        self.adj.iter().map(|a| a.len()).sum::<usize>() / 2
    }
    /// Return true if the graph is connected (using BFS from vertex 0).
    pub fn is_connected(&self) -> bool {
        if self.n == 0 {
            return true;
        }
        let mut visited = vec![false; self.n];
        let mut queue = VecDeque::new();
        visited[0] = true;
        queue.push_back(0);
        while let Some(v) = queue.pop_front() {
            for &w in &self.adj[v] {
                if !visited[w] {
                    visited[w] = true;
                    queue.push_back(w);
                }
            }
        }
        visited.iter().all(|&b| b)
    }
    /// BFS shortest path lengths from source `s`. Returns `usize::MAX` for unreachable nodes.
    pub fn bfs_distances(&self, s: usize) -> Vec<usize> {
        let mut dist = vec![usize::MAX; self.n];
        dist[s] = 0;
        let mut queue = VecDeque::new();
        queue.push_back(s);
        while let Some(v) = queue.pop_front() {
            for &w in &self.adj[v] {
                if dist[w] == usize::MAX {
                    dist[w] = dist[v] + 1;
                    queue.push_back(w);
                }
            }
        }
        dist
    }
    /// Compute the average shortest-path length (over all reachable pairs).
    pub fn average_path_length(&self) -> f64 {
        let mut total = 0usize;
        let mut count = 0usize;
        for s in 0..self.n {
            let dist = self.bfs_distances(s);
            for d in dist {
                if d != usize::MAX && d > 0 {
                    total += d;
                    count += 1;
                }
            }
        }
        if count == 0 {
            0.0
        } else {
            total as f64 / count as f64
        }
    }
    /// Compute the local clustering coefficient of vertex v.
    ///
    /// C(v) = (number of edges among neighbors of v) / (k*(k-1)/2),
    /// where k = degree(v). Returns 0 if k < 2.
    pub fn local_clustering(&self, v: usize) -> f64 {
        let neighbors: Vec<usize> = self.adj[v].iter().cloned().collect();
        let k = neighbors.len();
        if k < 2 {
            return 0.0;
        }
        let mut edges = 0usize;
        for i in 0..k {
            for j in (i + 1)..k {
                if self.adj[neighbors[i]].contains(&neighbors[j]) {
                    edges += 1;
                }
            }
        }
        2.0 * edges as f64 / (k * (k - 1)) as f64
    }
    /// Compute the global clustering coefficient (average over all vertices).
    pub fn global_clustering(&self) -> f64 {
        if self.n == 0 {
            return 0.0;
        }
        let total: f64 = (0..self.n).map(|v| self.local_clustering(v)).sum();
        total / self.n as f64
    }
    /// Compute the degree distribution: returns a map from degree → count.
    pub fn degree_distribution(&self) -> HashMap<usize, usize> {
        let mut dist = HashMap::new();
        for v in 0..self.n {
            *dist.entry(self.degree(v)).or_insert(0) += 1;
        }
        dist
    }
    /// Compute the connected components using BFS. Returns a Vec of component node lists.
    pub fn connected_components(&self) -> Vec<Vec<usize>> {
        let mut visited = vec![false; self.n];
        let mut components = Vec::new();
        for start in 0..self.n {
            if !visited[start] {
                let mut comp = Vec::new();
                let mut queue = VecDeque::new();
                visited[start] = true;
                queue.push_back(start);
                while let Some(v) = queue.pop_front() {
                    comp.push(v);
                    for &w in &self.adj[v] {
                        if !visited[w] {
                            visited[w] = true;
                            queue.push_back(w);
                        }
                    }
                }
                components.push(comp);
            }
        }
        components
    }
    /// Return the size of the largest connected component.
    pub fn largest_component_size(&self) -> usize {
        self.connected_components()
            .iter()
            .map(|c| c.len())
            .max()
            .unwrap_or(0)
    }
}
/// Topology model for a network.
#[derive(Debug, Clone)]
pub enum NetworkTopology {
    /// Erdős–Rényi random graph with edge probability p.
    Random(f64),
    /// Barabási–Albert scale-free model with degree-distribution exponent γ.
    ScaleFree(f64),
    /// Watts–Strogatz small-world model: (rewiring probability β, mean degree k).
    SmallWorld(f64, f64),
    /// Regular lattice where every node has the same degree.
    Regular(u32),
}
impl NetworkTopology {
    /// Expected average degree for the topology model.
    pub fn average_degree(&self) -> f64 {
        match self {
            NetworkTopology::Random(p) => *p,
            NetworkTopology::ScaleFree(gamma) => {
                if *gamma > 2.0 {
                    2.0 / (gamma - 2.0)
                } else {
                    f64::INFINITY
                }
            }
            NetworkTopology::SmallWorld(_beta, k) => *k,
            NetworkTopology::Regular(d) => *d as f64,
        }
    }
    /// Expected clustering coefficient for the topology model.
    pub fn clustering_coefficient(&self) -> f64 {
        match self {
            NetworkTopology::Random(p) => *p,
            NetworkTopology::ScaleFree(_) => 0.0,
            NetworkTopology::SmallWorld(_beta, k) => {
                let kv = *k;
                if kv >= 2.0 {
                    3.0 * (kv - 2.0) / (4.0 * (kv - 1.0))
                } else {
                    0.0
                }
            }
            NetworkTopology::Regular(d) => {
                let d = *d as f64;
                if d >= 2.0 {
                    3.0 * (d - 2.0) / (4.0 * (d - 1.0))
                } else {
                    0.0
                }
            }
        }
    }
}
/// State of nodes in an epidemic model.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EpidemicState {
    Susceptible,
    Infected,
    Recovered,
}
/// Random graph model (Erdos-Renyi G(n,p)).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ErdosRenyiModel {
    pub n: usize,
    pub p: f64,
}
impl ErdosRenyiModel {
    #[allow(dead_code)]
    pub fn new(n: usize, p: f64) -> Self {
        Self { n, p }
    }
    #[allow(dead_code)]
    pub fn expected_edges(&self) -> f64 {
        let n = self.n as f64;
        n * (n - 1.0) / 2.0 * self.p
    }
    #[allow(dead_code)]
    pub fn expected_degree(&self) -> f64 {
        (self.n as f64 - 1.0) * self.p
    }
    #[allow(dead_code)]
    pub fn connectivity_threshold(&self) -> f64 {
        (self.n as f64).ln() / self.n as f64
    }
    #[allow(dead_code)]
    pub fn is_above_connectivity_threshold(&self) -> bool {
        self.p > self.connectivity_threshold()
    }
    #[allow(dead_code)]
    pub fn giant_component_threshold(&self) -> f64 {
        1.0 / self.n as f64
    }
}
/// Network motif: a small subgraph pattern.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct NetworkMotif {
    pub name: String,
    pub num_nodes: usize,
    pub num_edges: usize,
    pub is_directed: bool,
}
impl NetworkMotif {
    #[allow(dead_code)]
    pub fn feed_forward_loop() -> Self {
        Self {
            name: "Feed-forward loop".to_string(),
            num_nodes: 3,
            num_edges: 3,
            is_directed: true,
        }
    }
    #[allow(dead_code)]
    pub fn bi_fan() -> Self {
        Self {
            name: "Bi-fan".to_string(),
            num_nodes: 4,
            num_edges: 4,
            is_directed: true,
        }
    }
    #[allow(dead_code)]
    pub fn triangle() -> Self {
        Self {
            name: "Triangle/3-clique".to_string(),
            num_nodes: 3,
            num_edges: 3,
            is_directed: false,
        }
    }
    #[allow(dead_code)]
    pub fn z_score_description(&self) -> String {
        format!(
            "Z-score of {} motif: (count - mean_random) / std_random",
            self.name
        )
    }
}
/// SIR epidemic model parameters.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SirModel {
    pub beta: f64,
    pub gamma: f64,
    pub initial_infected: f64,
    pub population: f64,
}
impl SirModel {
    #[allow(dead_code)]
    pub fn new(beta: f64, gamma: f64, i0: f64, n: f64) -> Self {
        Self {
            beta,
            gamma,
            initial_infected: i0,
            population: n,
        }
    }
    #[allow(dead_code)]
    pub fn basic_reproduction_number(&self) -> f64 {
        self.beta / self.gamma
    }
    #[allow(dead_code)]
    pub fn epidemic_threshold(&self) -> f64 {
        1.0 / self.basic_reproduction_number()
    }
    #[allow(dead_code)]
    pub fn will_epidemic_occur(&self) -> bool {
        self.basic_reproduction_number() > 1.0
    }
    #[allow(dead_code)]
    pub fn final_size_equation(&self) -> String {
        "R_inf = 1 - exp(-R0 * R_inf) (transcendental equation for final epidemic size)".to_string()
    }
}
