//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};
use std::collections::{HashMap, HashSet, VecDeque};

use super::types::{
    CommunityPartition, DiGraph, EpidemicState, ErdosRenyiModel, FlowNetwork, Graph,
    HeterogeneousMeanField, HitsResult, LinkPredictionMethod, LinkPredictionScore, Network,
    NetworkReliability, NetworkRobustness, PageRank, PageRankConfig, SIRTimeSeries, SISTimeSeries,
    SirModel, SpectralGraphData, StochasticBlockModel, TemporalGraph,
};

pub fn app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
pub fn app2(f: Expr, a: Expr, b: Expr) -> Expr {
    app(app(f, a), b)
}
pub fn app3(f: Expr, a: Expr, b: Expr, c: Expr) -> Expr {
    app(app2(f, a, b), c)
}
pub fn cst(s: &str) -> Expr {
    Expr::Const(Name::str(s), vec![])
}
pub fn prop() -> Expr {
    Expr::Sort(Level::zero())
}
pub fn type0() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
pub fn pi(bi: BinderInfo, name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Pi(bi, Name::str(name), Box::new(dom), Box::new(body))
}
pub fn arrow(a: Expr, b: Expr) -> Expr {
    pi(BinderInfo::Default, "_", a, b)
}
pub fn bvar(n: u32) -> Expr {
    Expr::BVar(n)
}
pub fn nat_ty() -> Expr {
    cst("Nat")
}
pub fn real_ty() -> Expr {
    cst("Real")
}
pub fn bool_ty() -> Expr {
    cst("Bool")
}
pub fn list_ty(elem: Expr) -> Expr {
    app(cst("List"), elem)
}
pub fn pair_ty(a: Expr, b: Expr) -> Expr {
    app2(cst("Prod"), a, b)
}
/// `Graph : Nat → Prop` — a graph on n vertices
pub fn graph_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `Subgraph : Prop → Prop → Prop` — H is a subgraph of G
pub fn subgraph_ty() -> Expr {
    arrow(prop(), arrow(prop(), prop()))
}
/// `Degree : Prop → Nat → Nat` — degree of vertex v in graph G
pub fn degree_ty() -> Expr {
    arrow(prop(), arrow(nat_ty(), nat_ty()))
}
/// `Connected : Prop → Prop` — graph G is connected
pub fn connected_ty() -> Expr {
    arrow(prop(), prop())
}
/// `GiantComponent : Prop → Real → Prop`
/// The giant component of G has relative size s.
pub fn giant_component_ty() -> Expr {
    arrow(prop(), arrow(real_ty(), prop()))
}
/// `ClusteringCoefficient : Prop → Real` — local clustering coefficient
pub fn clustering_coefficient_ty() -> Expr {
    arrow(prop(), real_ty())
}
/// `AveragePathLength : Prop → Real` — average shortest-path length
pub fn average_path_length_ty() -> Expr {
    arrow(prop(), real_ty())
}
/// `SmallWorldNetwork : Prop → Prop`
/// A network is small-world if it has high clustering and low average path length.
pub fn small_world_ty() -> Expr {
    arrow(prop(), prop())
}
/// `ScaleFreeNetwork : Prop → Prop`
/// A network is scale-free if its degree distribution follows a power law.
pub fn scale_free_ty() -> Expr {
    arrow(prop(), prop())
}
/// `BarabasiAlbertModel : Nat → Nat → Prop`
/// The BA model with initial m₀ nodes and attachment parameter m.
pub fn barabasi_albert_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `Modularity : Prop → Real`
/// The modularity Q of a network partition.
pub fn modularity_ty() -> Expr {
    arrow(prop(), real_ty())
}
/// `PageRank : Prop → Nat → Real`
/// The PageRank score of vertex v in graph G.
pub fn pagerank_ty() -> Expr {
    arrow(prop(), arrow(nat_ty(), real_ty()))
}
/// `HubScore : Prop → Nat → Real` — HITS hub score
pub fn hub_score_ty() -> Expr {
    arrow(prop(), arrow(nat_ty(), real_ty()))
}
/// `AuthorityScore : Prop → Nat → Real` — HITS authority score
pub fn authority_score_ty() -> Expr {
    arrow(prop(), arrow(nat_ty(), real_ty()))
}
/// `BetweennessCentrality : Prop → Nat → Real`
pub fn betweenness_centrality_ty() -> Expr {
    arrow(prop(), arrow(nat_ty(), real_ty()))
}
/// `ClosenessCentrality : Prop → Nat → Real`
pub fn closeness_centrality_ty() -> Expr {
    arrow(prop(), arrow(nat_ty(), real_ty()))
}
/// `EigenvectorCentrality : Prop → Nat → Real`
pub fn eigenvector_centrality_ty() -> Expr {
    arrow(prop(), arrow(nat_ty(), real_ty()))
}
/// `SIRModel : Real → Real → Prop`
/// Susceptible-Infected-Recovered model with infection rate β and recovery rate γ.
pub fn sir_model_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), prop()))
}
/// `SISModel : Real → Real → Prop`
/// Susceptible-Infected-Susceptible model.
pub fn sis_model_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), prop()))
}
/// `BasicReproductionNumber : Prop → Real`
/// R₀ = β/γ for SIR; epidemic if R₀ > 1.
pub fn basic_reproduction_number_ty() -> Expr {
    arrow(prop(), real_ty())
}
/// `PercolationThreshold : Prop → Real`
/// Critical bond/site percolation probability p_c.
pub fn percolation_threshold_ty() -> Expr {
    arrow(prop(), real_ty())
}
/// `NetworkRobustness : Prop → Real → Prop`
/// Network G remains connected after fraction f of nodes are removed.
pub fn network_robustness_ty() -> Expr {
    arrow(prop(), arrow(real_ty(), prop()))
}
/// `IndependentCascade : Prop → Real → Nat → Real`
/// Expected cascade size from seed node v with propagation probability p.
pub fn independent_cascade_ty() -> Expr {
    arrow(prop(), arrow(real_ty(), arrow(nat_ty(), real_ty())))
}
/// `StructuralHole : Prop → Nat → Prop`
/// Vertex v occupies a structural hole in G (Burt's constraint is low).
pub fn structural_hole_ty() -> Expr {
    arrow(prop(), arrow(nat_ty(), prop()))
}
/// `WattsStrogatzSmallWorld`: rewired lattices are small-world networks.
pub fn watts_strogatz_theorem_ty() -> Expr {
    arrow(prop(), arrow(real_ty(), prop()))
}
/// `BarabasiAlbertPowerLaw`: BA model produces scale-free degree distribution P(k) ~ k^{-3}.
pub fn ba_power_law_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `EpidemicThreshold`: SIR epidemic grows iff R₀ > 1.
pub fn epidemic_threshold_ty() -> Expr {
    arrow(prop(), arrow(real_ty(), prop()))
}
/// `PageRankConvergence`: PageRank iteration converges for any damping factor d ∈ (0,1).
pub fn pagerank_convergence_ty() -> Expr {
    arrow(prop(), arrow(real_ty(), prop()))
}
/// `ModularityNPHard`: Maximizing modularity is NP-hard.
pub fn modularity_nphard_ty() -> Expr {
    prop()
}
/// `PercolationPhaseTransition`: giant component emerges at p_c.
pub fn percolation_phase_transition_ty() -> Expr {
    arrow(prop(), prop())
}
/// `ScaleFreeRobustness`: scale-free networks are robust to random failures but fragile to targeted attacks.
pub fn scale_free_robustness_ty() -> Expr {
    arrow(prop(), prop())
}
/// Build the network theory environment: register all axioms as opaque constants.
pub fn build_network_theory_env(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("Graph", graph_ty()),
        ("Subgraph", subgraph_ty()),
        ("Degree", degree_ty()),
        ("Connected", connected_ty()),
        ("GiantComponent", giant_component_ty()),
        ("ClusteringCoefficient", clustering_coefficient_ty()),
        ("AveragePathLength", average_path_length_ty()),
        ("SmallWorldNetwork", small_world_ty()),
        ("ScaleFreeNetwork", scale_free_ty()),
        ("BarabasiAlbertModel", barabasi_albert_ty()),
        ("Modularity", modularity_ty()),
        ("PageRank", pagerank_ty()),
        ("HubScore", hub_score_ty()),
        ("AuthorityScore", authority_score_ty()),
        ("BetweennessCentrality", betweenness_centrality_ty()),
        ("ClosenessCentrality", closeness_centrality_ty()),
        ("EigenvectorCentrality", eigenvector_centrality_ty()),
        ("SIRModel", sir_model_ty()),
        ("SISModel", sis_model_ty()),
        ("BasicReproductionNumber", basic_reproduction_number_ty()),
        ("PercolationThreshold", percolation_threshold_ty()),
        ("NetworkRobustness", network_robustness_ty()),
        ("IndependentCascade", independent_cascade_ty()),
        ("StructuralHole", structural_hole_ty()),
        ("watts_strogatz_theorem", watts_strogatz_theorem_ty()),
        ("ba_power_law", ba_power_law_ty()),
        ("epidemic_threshold", epidemic_threshold_ty()),
        ("pagerank_convergence", pagerank_convergence_ty()),
        ("modularity_nphard", modularity_nphard_ty()),
        (
            "percolation_phase_transition",
            percolation_phase_transition_ty(),
        ),
        ("scale_free_robustness_theorem", scale_free_robustness_ty()),
        (
            "CommunityPartition",
            arrow(prop(), list_ty(list_ty(nat_ty()))),
        ),
        (
            "AdjacencyMatrix",
            arrow(nat_ty(), list_ty(list_ty(bool_ty()))),
        ),
        ("DegreeDistribution", arrow(prop(), list_ty(real_ty()))),
        (
            "ShortestPathMatrix",
            arrow(prop(), list_ty(list_ty(nat_ty()))),
        ),
        (
            "NetworkFlow",
            arrow(prop(), arrow(nat_ty(), arrow(nat_ty(), real_ty()))),
        ),
        ("Homophily", arrow(prop(), real_ty())),
        ("TriadicClosure", arrow(prop(), real_ty())),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
}
/// Compute PageRank scores for a directed graph.
///
/// Uses the power iteration method. Dangling nodes (out-degree 0) distribute
/// their rank uniformly to all nodes.
pub fn pagerank(g: &DiGraph, cfg: &PageRankConfig) -> Vec<f64> {
    let n = g.n;
    if n == 0 {
        return vec![];
    }
    let mut rank = vec![1.0 / n as f64; n];
    let teleport = (1.0 - cfg.damping) / n as f64;
    for _ in 0..cfg.max_iter {
        let mut new_rank = vec![teleport; n];
        let dangling: f64 = (0..n)
            .filter(|&v| g.out_degree(v) == 0)
            .map(|v| rank[v])
            .sum();
        let dangling_contrib = cfg.damping * dangling / n as f64;
        for v in 0..n {
            new_rank[v] += dangling_contrib;
            for &u in &g.in_adj[v] {
                new_rank[v] += cfg.damping * rank[u] / g.out_degree(u) as f64;
            }
        }
        let delta: f64 = rank.iter().zip(&new_rank).map(|(a, b)| (a - b).abs()).sum();
        rank = new_rank;
        if delta < cfg.tol {
            break;
        }
    }
    rank
}
/// Compute HITS hub and authority scores using power iteration.
pub fn hits(g: &DiGraph, max_iter: usize) -> HitsResult {
    let n = g.n;
    let mut hub = vec![1.0_f64; n];
    let mut auth = vec![1.0_f64; n];
    for _ in 0..max_iter {
        let mut new_auth = vec![0.0_f64; n];
        for v in 0..n {
            for &u in &g.in_adj[v] {
                new_auth[v] += hub[u];
            }
        }
        let auth_norm: f64 = new_auth.iter().map(|x| x * x).sum::<f64>().sqrt();
        if auth_norm > 1e-15 {
            for x in &mut new_auth {
                *x /= auth_norm;
            }
        }
        let mut new_hub = vec![0.0_f64; n];
        for v in 0..n {
            for &w in &g.out_adj[v] {
                new_hub[v] += new_auth[w];
            }
        }
        let hub_norm: f64 = new_hub.iter().map(|x| x * x).sum::<f64>().sqrt();
        if hub_norm > 1e-15 {
            for x in &mut new_hub {
                *x /= hub_norm;
            }
        }
        auth = new_auth;
        hub = new_hub;
    }
    HitsResult {
        hub,
        authority: auth,
    }
}
/// Compute degree centrality for all vertices (normalized by n-1).
pub fn degree_centrality(g: &Graph) -> Vec<f64> {
    let n = g.n;
    if n <= 1 {
        return vec![0.0; n];
    }
    (0..n)
        .map(|v| g.degree(v) as f64 / (n - 1) as f64)
        .collect()
}
/// Compute closeness centrality for all vertices.
///
/// C(v) = (n-1) / sum_u d(v,u), considering only reachable nodes.
pub fn closeness_centrality(g: &Graph) -> Vec<f64> {
    let n = g.n;
    (0..n)
        .map(|v| {
            let dist = g.bfs_distances(v);
            let (total, count) = dist.iter().fold((0usize, 0usize), |(s, c), &d| {
                if d != usize::MAX && d > 0 {
                    (s + d, c + 1)
                } else {
                    (s, c)
                }
            });
            if total == 0 {
                0.0
            } else {
                count as f64 / total as f64
            }
        })
        .collect()
}
/// Compute betweenness centrality for all vertices using Brandes' algorithm.
///
/// Returns unnormalized betweenness (divide by (n-1)(n-2)/2 for undirected normalization).
pub fn betweenness_centrality(g: &Graph) -> Vec<f64> {
    let n = g.n;
    let mut cb = vec![0.0_f64; n];
    for s in 0..n {
        let mut stack = Vec::new();
        let mut pred: Vec<Vec<usize>> = vec![Vec::new(); n];
        let mut sigma = vec![0.0_f64; n];
        let mut dist = vec![-1i64; n];
        sigma[s] = 1.0;
        dist[s] = 0;
        let mut queue = VecDeque::new();
        queue.push_back(s);
        while let Some(v) = queue.pop_front() {
            stack.push(v);
            for &w in &g.adj[v] {
                if dist[w] < 0 {
                    queue.push_back(w);
                    dist[w] = dist[v] + 1;
                }
                if dist[w] == dist[v] + 1 {
                    sigma[w] += sigma[v];
                    pred[w].push(v);
                }
            }
        }
        let mut delta = vec![0.0_f64; n];
        while let Some(w) = stack.pop() {
            for &v in &pred[w] {
                delta[v] += (sigma[v] / sigma[w]) * (1.0 + delta[w]);
            }
            if w != s {
                cb[w] += delta[w];
            }
        }
    }
    cb
}
/// Approximate eigenvector centrality via power iteration.
pub fn eigenvector_centrality(g: &Graph, max_iter: usize) -> Vec<f64> {
    let n = g.n;
    if n == 0 {
        return vec![];
    }
    let mut x = vec![1.0_f64 / n as f64; n];
    for _ in 0..max_iter {
        let mut xnew = vec![0.0_f64; n];
        for v in 0..n {
            for &w in &g.adj[v] {
                xnew[v] += x[w];
            }
        }
        let norm: f64 = xnew.iter().cloned().fold(0.0_f64, f64::max);
        if norm > 1e-15 {
            for xv in &mut xnew {
                *xv /= norm;
            }
        }
        x = xnew;
    }
    x
}
/// Greedy Louvain-style community detection (single-pass greedy modularity maximization).
///
/// Each vertex starts in its own community. We repeatedly move each vertex to the
/// neighbor community that maximizes the modularity gain, until no improvement is found.
pub fn louvain_communities(g: &Graph) -> CommunityPartition {
    let n = g.n;
    let mut label: Vec<usize> = (0..n).collect();
    let m = g.edge_count();
    if m == 0 {
        return CommunityPartition {
            label,
            n_communities: n,
        };
    }
    let mut improved = true;
    while improved {
        improved = false;
        for v in 0..n {
            let current_comm = label[v];
            let neighbor_comms: HashSet<usize> = g.adj[v].iter().map(|&w| label[w]).collect();
            let mut best_comm = current_comm;
            let mut best_gain = 0.0_f64;
            for &nc in &neighbor_comms {
                if nc == current_comm {
                    continue;
                }
                let edges_to_nc: usize = g.adj[v].iter().filter(|&&w| label[w] == nc).count();
                let edges_to_cur: usize = g.adj[v]
                    .iter()
                    .filter(|&&w| label[w] == current_comm && w != v)
                    .count();
                let gain = edges_to_nc as f64 - edges_to_cur as f64;
                if gain > best_gain {
                    best_gain = gain;
                    best_comm = nc;
                }
            }
            if best_comm != current_comm {
                label[v] = best_comm;
                improved = true;
            }
        }
    }
    let mut remap = HashMap::new();
    let mut next_id = 0usize;
    let mut relabeled = vec![0usize; n];
    for v in 0..n {
        let c = label[v];
        let entry = remap.entry(c).or_insert_with(|| {
            let id = next_id;
            next_id += 1;
            id
        });
        relabeled[v] = *entry;
    }
    CommunityPartition {
        label: relabeled,
        n_communities: next_id,
    }
}
/// Generate a Barabási-Albert scale-free network.
///
/// Start with a fully connected seed of `m0` nodes. Each new node attaches
/// `m` edges via preferential attachment: probability proportional to degree.
#[allow(clippy::too_many_arguments)]
pub fn barabasi_albert(total_nodes: usize, m0: usize, m: usize) -> Graph {
    let m0 = m0.max(2);
    let m = m.min(m0);
    let mut g = Graph::new(total_nodes.max(m0));
    for i in 0..m0 {
        for j in (i + 1)..m0 {
            g.add_edge(i, j);
        }
    }
    if total_nodes <= m0 {
        return g;
    }
    let mut stubs: Vec<usize> = Vec::new();
    for v in 0..m0 {
        let deg = g.degree(v).max(1);
        for _ in 0..deg {
            stubs.push(v);
        }
    }
    for new_node in m0..total_nodes {
        let mut targets = HashSet::new();
        let stub_len = stubs.len();
        let step = if stub_len > m { stub_len / m } else { 1 };
        let mut idx = new_node % stub_len.max(1);
        let mut attempts = 0;
        while targets.len() < m && attempts < stub_len * 2 {
            let candidate = stubs[idx % stub_len];
            if candidate != new_node {
                targets.insert(candidate);
            }
            idx = (idx + step) % stub_len.max(1);
            attempts += 1;
        }
        for &t in &targets {
            g.add_edge(new_node, t);
            stubs.push(new_node);
            stubs.push(t);
        }
    }
    g
}
/// Bond percolation on a graph: keep each edge independently with probability p.
///
/// Returns the fraction of nodes in the largest connected component.
pub fn bond_percolation(g: &Graph, p: f64, seed: u64) -> f64 {
    let n = g.n;
    if n == 0 {
        return 0.0;
    }
    let mut rng_state = seed;
    let mut percolated = Graph::new(n);
    let lcg_next = |s: u64| -> (u64, f64) {
        let s2 = s
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let f = (s2 >> 11) as f64 / (1u64 << 53) as f64;
        (s2, f)
    };
    for u in 0..n {
        for &v in &g.adj[u] {
            if u < v {
                let (ns, f) = lcg_next(rng_state);
                rng_state = ns;
                if f < p {
                    percolated.add_edge(u, v);
                }
            }
        }
    }
    percolated.largest_component_size() as f64 / n as f64
}
/// Site percolation on a graph: keep each vertex independently with probability p.
///
/// Returns the fraction of nodes in the largest connected component.
pub fn site_percolation(g: &Graph, p: f64, seed: u64) -> f64 {
    let n = g.n;
    if n == 0 {
        return 0.0;
    }
    let mut rng_state = seed;
    let lcg_next = |s: u64| -> (u64, f64) {
        let s2 = s
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let f = (s2 >> 11) as f64 / (1u64 << 53) as f64;
        (s2, f)
    };
    let mut active = vec![false; n];
    for v in 0..n {
        let (ns, f) = lcg_next(rng_state);
        rng_state = ns;
        active[v] = f < p;
    }
    let mut visited = vec![false; n];
    let mut max_comp = 0;
    for start in 0..n {
        if !active[start] || visited[start] {
            continue;
        }
        let mut comp = 0;
        let mut queue = VecDeque::new();
        visited[start] = true;
        queue.push_back(start);
        while let Some(v) = queue.pop_front() {
            comp += 1;
            for &w in &g.adj[v] {
                if active[w] && !visited[w] {
                    visited[w] = true;
                    queue.push_back(w);
                }
            }
        }
        max_comp = max_comp.max(comp);
    }
    max_comp as f64 / n as f64
}
/// Run a discrete-time SIR simulation on graph `g`.
///
/// `beta` = infection probability per S-I contact per step.
/// `gamma` = recovery probability per I node per step.
/// `initial_infected` = set of initially infected nodes.
/// Returns time series of (S, I, R) fractions.
pub fn sir_simulation(
    g: &Graph,
    beta: f64,
    gamma: f64,
    initial_infected: &[usize],
    max_steps: usize,
    seed: u64,
) -> SIRTimeSeries {
    let n = g.n;
    let mut state = vec![EpidemicState::Susceptible; n];
    for &v in initial_infected {
        if v < n {
            state[v] = EpidemicState::Infected;
        }
    }
    let mut rng_state = seed;
    let lcg_next = |s: u64| -> (u64, f64) {
        let s2 = s
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let f = (s2 >> 11) as f64 / (1u64 << 53) as f64;
        (s2, f)
    };
    let mut series = Vec::new();
    let count_states = |s: &Vec<EpidemicState>| -> (f64, f64, f64) {
        let sus = s
            .iter()
            .filter(|&&x| x == EpidemicState::Susceptible)
            .count();
        let inf = s.iter().filter(|&&x| x == EpidemicState::Infected).count();
        let rec = s.iter().filter(|&&x| x == EpidemicState::Recovered).count();
        let nf = n as f64;
        (sus as f64 / nf, inf as f64 / nf, rec as f64 / nf)
    };
    series.push(count_states(&state));
    for _ in 0..max_steps {
        let mut new_state = state.clone();
        for v in 0..n {
            match state[v] {
                EpidemicState::Susceptible => {
                    let infected_neighbors = g.adj[v]
                        .iter()
                        .filter(|&&w| state[w] == EpidemicState::Infected)
                        .count();
                    let prob_infect = 1.0 - (1.0 - beta).powi(infected_neighbors as i32);
                    let (ns, f) = lcg_next(rng_state);
                    rng_state = ns;
                    if f < prob_infect {
                        new_state[v] = EpidemicState::Infected;
                    }
                }
                EpidemicState::Infected => {
                    let (ns, f) = lcg_next(rng_state);
                    rng_state = ns;
                    if f < gamma {
                        new_state[v] = EpidemicState::Recovered;
                    }
                }
                EpidemicState::Recovered => {}
            }
        }
        state = new_state;
        series.push(count_states(&state));
        if state.iter().all(|&s| s != EpidemicState::Infected) {
            break;
        }
    }
    SIRTimeSeries { series }
}
/// Run a discrete-time SIS simulation on graph `g`.
pub fn sis_simulation(
    g: &Graph,
    beta: f64,
    gamma: f64,
    initial_infected: &[usize],
    max_steps: usize,
    seed: u64,
) -> SISTimeSeries {
    let n = g.n;
    let mut infected = vec![false; n];
    for &v in initial_infected {
        if v < n {
            infected[v] = true;
        }
    }
    let mut rng_state = seed;
    let lcg_next = |s: u64| -> (u64, f64) {
        let s2 = s
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let f = (s2 >> 11) as f64 / (1u64 << 53) as f64;
        (s2, f)
    };
    let mut series = Vec::new();
    let count = |inf: &Vec<bool>| -> (f64, f64) {
        let i = inf.iter().filter(|&&b| b).count() as f64 / n as f64;
        (1.0 - i, i)
    };
    series.push(count(&infected));
    for _ in 0..max_steps {
        let mut new_inf = infected.clone();
        for v in 0..n {
            if infected[v] {
                let (ns, f) = lcg_next(rng_state);
                rng_state = ns;
                if f < gamma {
                    new_inf[v] = false;
                }
            } else {
                let inf_nbrs = g.adj[v].iter().filter(|&&w| infected[w]).count();
                let prob = 1.0 - (1.0 - beta).powi(inf_nbrs as i32);
                let (ns, f) = lcg_next(rng_state);
                rng_state = ns;
                if f < prob {
                    new_inf[v] = true;
                }
            }
        }
        infected = new_inf;
        series.push(count(&infected));
    }
    let endemic_level = series.last().map(|&(_, i)| i).unwrap_or(0.0);
    SISTimeSeries {
        series,
        endemic_level,
    }
}
/// Compute R₀ = beta * average_degree / gamma for a network.
pub fn basic_reproduction_number(g: &Graph, beta: f64, gamma: f64) -> f64 {
    if g.n == 0 || gamma < 1e-15 {
        return 0.0;
    }
    let avg_deg = (0..g.n).map(|v| g.degree(v)).sum::<usize>() as f64 / g.n as f64;
    beta * avg_deg / gamma
}
/// Simulate random node removal and return the fraction of the giant component
/// at each removal step (returns a Vec of (fraction_removed, giant_fraction)).
pub fn random_attack_resilience(g: &Graph) -> Vec<(f64, f64)> {
    let n = g.n;
    let mut result = Vec::new();
    let mut active = vec![true; n];
    let mut current = g.clone();
    for step in 0..n {
        let frac_removed = step as f64 / n as f64;
        let gc = current.largest_component_size() as f64 / n as f64;
        result.push((frac_removed, gc));
        for &w in &g.adj[step] {
            current.adj[step].remove(&w);
            current.adj[w].remove(&step);
        }
        active[step] = false;
    }
    result
}
/// Simulate targeted attack (highest-degree first) and return resilience curve.
pub fn targeted_attack_resilience(g: &Graph) -> Vec<(f64, f64)> {
    let n = g.n;
    let mut result = Vec::new();
    let mut current = g.clone();
    let mut removal_order: Vec<usize> = (0..n).collect();
    removal_order.sort_by(|&a, &b| g.degree(b).cmp(&g.degree(a)));
    for (step, &node) in removal_order.iter().enumerate() {
        let frac_removed = step as f64 / n as f64;
        let gc = current.largest_component_size() as f64 / n as f64;
        result.push((frac_removed, gc));
        for &w in &g.adj[node] {
            current.adj[node].remove(&w);
            current.adj[w].remove(&node);
        }
    }
    result
}
/// Run the Independent Cascade (IC) model on a directed graph.
///
/// Starting from `seeds`, each newly infected node `u` tries to infect each
/// susceptible neighbor `v` with probability `p`. Returns the final cascade size.
pub fn independent_cascade(g: &DiGraph, seeds: &[usize], p: f64, seed_rng: u64) -> usize {
    let n = g.n;
    let mut active = vec![false; n];
    let mut newly_active = Vec::new();
    for &s in seeds {
        if s < n && !active[s] {
            active[s] = true;
            newly_active.push(s);
        }
    }
    let mut rng_state = seed_rng;
    let lcg_next = |s: u64| -> (u64, f64) {
        let s2 = s
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let f = (s2 >> 11) as f64 / (1u64 << 53) as f64;
        (s2, f)
    };
    let mut queue: VecDeque<usize> = newly_active.into_iter().collect();
    let mut total = active.iter().filter(|&&b| b).count();
    while let Some(u) = queue.pop_front() {
        for &v in &g.out_adj[u] {
            if !active[v] {
                let (ns, f) = lcg_next(rng_state);
                rng_state = ns;
                if f < p {
                    active[v] = true;
                    total += 1;
                    queue.push_back(v);
                }
            }
        }
    }
    total
}
/// Compute the triadic closure rate: fraction of open triangles that are closed.
///
/// Returns a value in [0,1]; 1 means every path of length 2 is also connected.
pub fn triadic_closure_rate(g: &Graph) -> f64 {
    let mut closed = 0usize;
    let mut open = 0usize;
    for v in 0..g.n {
        let nbrs: Vec<usize> = g.adj[v].iter().cloned().collect();
        let k = nbrs.len();
        for i in 0..k {
            for j in (i + 1)..k {
                open += 1;
                if g.adj[nbrs[i]].contains(&nbrs[j]) {
                    closed += 1;
                }
            }
        }
    }
    if open == 0 {
        0.0
    } else {
        closed as f64 / open as f64
    }
}
/// Compute assortativity (degree-degree Pearson correlation coefficient).
pub fn degree_assortativity(g: &Graph) -> f64 {
    let m = g.edge_count();
    if m == 0 {
        return 0.0;
    }
    let mut sum_ji = 0.0_f64;
    let mut sum_sq = 0.0_f64;
    let mut sum_j_plus_i = 0.0_f64;
    for u in 0..g.n {
        for &v in &g.adj[u] {
            if u < v {
                let j = g.degree(u) as f64;
                let i = g.degree(v) as f64;
                sum_ji += j * i;
                sum_sq += (j * j + i * i) / 2.0;
                sum_j_plus_i += (j + i) / 2.0;
            }
        }
    }
    let mf = m as f64;
    let num = sum_ji / mf - (sum_j_plus_i / mf).powi(2);
    let den = sum_sq / mf - (sum_j_plus_i / mf).powi(2);
    if den.abs() < 1e-15 {
        0.0
    } else {
        num / den
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn triangle_graph() -> Graph {
        let mut g = Graph::new(3);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(0, 2);
        g
    }
    fn path_graph(n: usize) -> Graph {
        let mut g = Graph::new(n);
        for i in 0..(n - 1) {
            g.add_edge(i, i + 1);
        }
        g
    }
    fn star_graph(n: usize) -> Graph {
        let mut g = Graph::new(n);
        for i in 1..n {
            g.add_edge(0, i);
        }
        g
    }
    #[test]
    fn test_graph_basic() {
        let g = triangle_graph();
        assert_eq!(g.n, 3);
        assert_eq!(g.edge_count(), 3);
        assert_eq!(g.degree(0), 2);
        assert!(g.is_connected());
        let p = path_graph(4);
        assert!(p.is_connected());
        assert_eq!(p.edge_count(), 3);
        let mut disc = Graph::new(4);
        disc.add_edge(0, 1);
        assert!(!disc.is_connected());
    }
    #[test]
    fn test_clustering_coefficient() {
        let g = triangle_graph();
        for v in 0..3 {
            let cc = g.local_clustering(v);
            assert!(
                (cc - 1.0).abs() < 1e-10,
                "Triangle vertex {v} clustering should be 1"
            );
        }
        let s = star_graph(5);
        assert!(
            (s.local_clustering(0)).abs() < 1e-10,
            "Star hub clustering should be 0"
        );
    }
    #[test]
    fn test_average_path_length() {
        let p = path_graph(3);
        let apl = p.average_path_length();
        assert!(
            (apl - 4.0 / 3.0).abs() < 1e-10,
            "APL of P3 should be 4/3, got {apl}"
        );
        let g = triangle_graph();
        let apl2 = g.average_path_length();
        assert!((apl2 - 1.0).abs() < 1e-10, "APL of triangle should be 1");
    }
    #[test]
    fn test_pagerank() {
        let mut g = DiGraph::new(3);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 0);
        let cfg = PageRankConfig::default();
        let pr = pagerank(&g, &cfg);
        assert_eq!(pr.len(), 3);
        let total: f64 = pr.iter().sum();
        assert!(
            (total - 1.0).abs() < 1e-6,
            "PageRank sum should be ~1, got {total}"
        );
        assert!(
            (pr[0] - pr[1]).abs() < 1e-6,
            "Cycle nodes should have equal PageRank"
        );
        assert!(
            (pr[1] - pr[2]).abs() < 1e-6,
            "Cycle nodes should have equal PageRank"
        );
    }
    #[test]
    fn test_betweenness_centrality() {
        let p = path_graph(5);
        let bc = betweenness_centrality(&p);
        assert_eq!(bc.len(), 5);
        let max_node = bc
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).expect("max_by should succeed"))
            .expect("max_by should succeed")
            .0;
        assert_eq!(
            max_node, 2,
            "Middle node should have highest betweenness in path P5"
        );
    }
    #[test]
    fn test_community_modularity() {
        let mut g = Graph::new(6);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(0, 2);
        g.add_edge(3, 4);
        g.add_edge(4, 5);
        g.add_edge(3, 5);
        g.add_edge(2, 3);
        let partition = CommunityPartition {
            label: vec![0, 0, 0, 1, 1, 1],
            n_communities: 2,
        };
        let q = partition.modularity(&g);
        assert!(
            q > 0.0,
            "Modularity of two-triangle partition should be positive, got {q}"
        );
    }
    #[test]
    fn test_sir_epidemic() {
        let mut g = Graph::new(5);
        for i in 0..5 {
            for j in (i + 1)..5 {
                g.add_edge(i, j);
            }
        }
        let result = sir_simulation(&g, 0.8, 0.1, &[0], 50, 42);
        assert!(!result.series.is_empty(), "SIR should produce time series");
        let (s0, i0, r0) = result.series[0];
        assert!((s0 - 0.8).abs() < 1e-10, "Initially 4/5 susceptible");
        assert!((i0 - 0.2).abs() < 1e-10, "Initially 1/5 infected");
        assert!((r0).abs() < 1e-10, "Initially 0 recovered");
        let r0_val = basic_reproduction_number(&g, 0.8, 0.1);
        assert!(r0_val > 1.0, "R0 should be > 1 for epidemic, got {r0_val}");
    }
    #[test]
    fn test_ba_model() {
        let g = barabasi_albert(50, 3, 2);
        assert_eq!(g.n, 50);
        assert!(g.is_connected(), "BA model should produce connected graph");
        let avg_deg = (0..50).map(|v| g.degree(v)).sum::<usize>() as f64 / 50.0;
        let max_deg = (0..50).map(|v| g.degree(v)).max().unwrap_or(0);
        assert!(max_deg as f64 > avg_deg, "BA model should have hub nodes");
    }
    #[test]
    fn test_build_env() {
        let mut env = Environment::new();
        build_network_theory_env(&mut env);
        assert!(env.get(&Name::str("Graph")).is_some());
        assert!(env.get(&Name::str("PageRank")).is_some());
        assert!(env.get(&Name::str("SIRModel")).is_some());
        assert!(env.get(&Name::str("Modularity")).is_some());
    }
}
/// Build the network theory environment (public alias for build_network_theory_env).
pub fn build_env(env: &mut Environment) {
    build_network_theory_env(env);
}
/// Network centrality measures.
#[allow(dead_code)]
pub fn centrality_measures() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Degree centrality", "C_D(v) = deg(v) / (n-1)"),
        (
            "Betweenness centrality",
            "C_B(v) = sum sigma_st(v) / sigma_st",
        ),
        ("Closeness centrality", "C_C(v) = (n-1) / sum d(v,u)"),
        ("Eigenvector centrality", "C_E(v): v = lambda^-1 A v"),
        ("Katz centrality", "C_K(v) = sum_k alpha^k (A^k 1)_v"),
        ("PageRank", "PR(v) = (1-d)/n + d * sum PR(u)/deg(u)"),
        ("Harmonic centrality", "C_H(v) = sum 1/d(v,u) for u != v"),
        (
            "Percolation centrality",
            "Combines betweenness and state information",
        ),
    ]
}
#[cfg(test)]
mod network_ext_tests {
    use super::*;
    #[test]
    fn test_temporal_graph() {
        let mut tg = TemporalGraph::new(4, 10.0);
        tg.add_edge(0, 1, 1.0, 3.0);
        tg.add_edge(1, 2, 2.0, 2.0);
        assert_eq!(tg.num_edges_at(2.0), 2);
        assert!(tg.has_temporal_path(0, 2));
    }
    #[test]
    fn test_erdos_renyi() {
        let g = ErdosRenyiModel::new(100, 0.1);
        let exp_deg = g.expected_degree();
        assert!((exp_deg - 9.9).abs() < 0.2);
        assert!(g.is_above_connectivity_threshold());
    }
    #[test]
    fn test_stochastic_block_model() {
        let sbm = StochasticBlockModel::new(vec![50, 50], 0.1, 0.01);
        assert_eq!(sbm.total_nodes(), 100);
    }
    #[test]
    fn test_spectral_graph() {
        let sg = SpectralGraphData::new("K4", 4, 4.0, 4.0);
        assert!(sg.is_connected());
        assert!(sg.expander_quality() > 0.0);
    }
    #[test]
    fn test_centrality_nonempty() {
        let c = centrality_measures();
        assert!(!c.is_empty());
    }
}
#[cfg(test)]
mod flow_tests {
    use super::*;
    #[test]
    fn test_flow_network() {
        let mut fn_ = FlowNetwork::new(4, 0, 3);
        fn_.add_edge(0, 1, 10.0);
        fn_.add_edge(0, 2, 5.0);
        assert!((fn_.max_capacity_from_source() - 15.0).abs() < 1e-10);
        assert!(!fn_.mfmc_theorem().is_empty());
    }
    #[test]
    fn test_network_reliability() {
        let nr = NetworkReliability::new(5, 6, 0.9);
        let p = nr.all_edges_work_probability();
        assert!(p > 0.0 && p < 1.0);
    }
}
#[cfg(test)]
mod epidemic_tests {
    use super::*;
    #[test]
    fn test_sir_model() {
        let sir = SirModel::new(0.3, 0.1, 1.0, 1000.0);
        assert!((sir.basic_reproduction_number() - 3.0).abs() < 1e-10);
        assert!(sir.will_epidemic_occur());
    }
    #[test]
    fn test_heterogeneous_mf() {
        let n_max = 20usize;
        let lambda = 3.0_f64;
        let mut dist = Vec::with_capacity(n_max);
        let mut factorial = 1u64;
        for k in 0..n_max {
            if k > 0 {
                factorial *= k as u64;
            }
            let pk = (-lambda).exp() * lambda.powi(k as i32) / factorial as f64;
            dist.push(pk);
        }
        let hmf = HeterogeneousMeanField::new(dist, 0.3, 0.1);
        assert!((hmf.mean_degree() - 3.0).abs() < 0.1);
    }
}
#[cfg(test)]
mod link_prediction_tests {
    use super::*;
    #[test]
    fn test_link_prediction() {
        let lp = LinkPredictionScore::new(LinkPredictionMethod::CommonNeighbors, 1, 2, 3.0);
        assert_eq!(lp.pair, (1, 2));
        assert!(!lp.description().is_empty());
    }
}
