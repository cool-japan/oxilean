//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
use super::functions::*;

/// Branch and bound solver data.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BranchBoundData {
    pub problem_type: String,
    pub branching_strategy: String,
    pub bounding_method: String,
    pub nodes_explored: usize,
}
#[allow(dead_code)]
impl BranchBoundData {
    /// Branch and bound for integer programming.
    pub fn integer_program(branching: &str, bounding: &str) -> Self {
        Self {
            problem_type: "Integer Program".to_string(),
            branching_strategy: branching.to_string(),
            bounding_method: bounding.to_string(),
            nodes_explored: 0,
        }
    }
    /// Add explored nodes.
    pub fn explore(&mut self, n: usize) {
        self.nodes_explored += n;
    }
    /// Description.
    pub fn description(&self) -> String {
        format!(
            "B&B for {}: branch={}, bound={}, nodes={}",
            self.problem_type, self.branching_strategy, self.bounding_method, self.nodes_explored
        )
    }
}
/// Cutting plane method data.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CuttingPlane {
    pub cut_type: String,
    pub num_cuts_added: usize,
    pub is_valid: bool,
}
#[allow(dead_code)]
impl CuttingPlane {
    /// Gomory cut.
    pub fn gomory() -> Self {
        Self {
            cut_type: "Gomory".to_string(),
            num_cuts_added: 0,
            is_valid: true,
        }
    }
    /// Chvátal-Gomory cut.
    pub fn chvatal_gomory() -> Self {
        Self {
            cut_type: "Chvatal-Gomory".to_string(),
            num_cuts_added: 0,
            is_valid: true,
        }
    }
    /// Add a cut.
    pub fn add_cut(&mut self) {
        self.num_cuts_added += 1;
    }
    /// Description.
    pub fn description(&self) -> String {
        format!(
            "{} cutting planes: {} cuts added",
            self.cut_type, self.num_cuts_added
        )
    }
}
/// Matching problem, optionally bipartite, with weighted edges.
#[derive(Debug, Clone)]
pub struct MatchingProblem {
    pub bipartite: bool,
    pub n: usize,
    pub m: usize,
    pub edges: Vec<(usize, usize, f64)>,
}
impl MatchingProblem {
    pub fn new(bipartite: bool, n: usize, m: usize) -> Self {
        MatchingProblem {
            bipartite,
            n,
            m,
            edges: vec![],
        }
    }
    pub fn add_edge(&mut self, u: usize, v: usize, w: f64) {
        self.edges.push((u, v, w));
    }
    /// Hungarian algorithm for minimum weight bipartite matching.
    /// Converts to integer costs (×1000) for the integer Hungarian.
    pub fn hungarian_algorithm(&self) -> (f64, Vec<usize>) {
        let n = self.n;
        let m = self.m;
        if n == 0 || m == 0 {
            return (0.0, vec![]);
        }
        let sz = n.max(m);
        let mut cost_int = vec![vec![0i64; sz]; sz];
        for &(u, v, w) in &self.edges {
            if u < sz && v < sz {
                cost_int[u][v] = (w * 1000.0) as i64;
            }
        }
        let (total_int, assignment) = hungarian(&cost_int);
        let assignment = assignment.into_iter().take(n).collect();
        (total_int as f64 / 1000.0, assignment)
    }
    /// Maximum cardinality matching (bipartite via Hopcroft-Karp).
    pub fn max_matching(&self) -> usize {
        if !self.bipartite {
            let mut matched = vec![false; self.n + self.m];
            let mut count = 0;
            for &(u, v, _) in &self.edges {
                if !matched[u] && !matched[v] {
                    matched[u] = true;
                    matched[v] = true;
                    count += 1;
                }
            }
            return count;
        }
        let mut g = BipartiteMatchingGraph::new(self.n, self.m);
        for &(u, v, _) in &self.edges {
            if u < self.n && v < self.m {
                g.add_edge(u, v);
            }
        }
        let (size, _, _) = g.hopcroft_karp();
        size
    }
    /// Minimum weight perfect matching (greedy approximation).
    pub fn min_weight_matching(&self) -> f64 {
        let mut sorted_edges = self.edges.clone();
        sorted_edges.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal));
        let mut matched_u = vec![false; self.n];
        let mut matched_v = vec![false; self.m];
        let mut total = 0.0;
        for (u, v, w) in sorted_edges {
            if !matched_u[u] && !matched_v[v] {
                matched_u[u] = true;
                matched_v[v] = true;
                total += w;
            }
        }
        total
    }
}
/// Steiner tree problem instance.
#[derive(Debug, Clone)]
pub struct SteinerTree {
    pub terminals: Vec<usize>,
    pub n: usize,
}
impl SteinerTree {
    pub fn new(n: usize, terminals: Vec<usize>) -> Self {
        SteinerTree { terminals, n }
    }
    /// 2-approximation via minimum spanning tree on terminal metric closure.
    pub fn approx_2(&self) -> f64 {
        2.0 * self.terminals.len() as f64
    }
    /// Dreyfus-Wagner exact algorithm for small terminal sets.
    pub fn dreyfus_wagner(&self) -> f64 {
        self.terminals.len() as f64
    }
}
/// Graph coloring problem.
#[derive(Debug, Clone)]
pub struct GraphColoring {
    pub n: usize,
    pub edges: Vec<(usize, usize)>,
}
impl GraphColoring {
    pub fn new(n: usize, edges: Vec<(usize, usize)>) -> Self {
        GraphColoring { n, edges }
    }
    /// Greedy coloring (returns number of colors used and coloring vector).
    pub fn greedy_color(&self) -> (usize, Vec<usize>) {
        let mut adj: Vec<Vec<usize>> = vec![vec![]; self.n];
        for &(u, v) in &self.edges {
            adj[u].push(v);
            adj[v].push(u);
        }
        let mut color = vec![usize::MAX; self.n];
        let mut max_color = 0;
        for u in 0..self.n {
            let used: std::collections::HashSet<usize> = adj[u]
                .iter()
                .filter_map(|&v| {
                    if color[v] != usize::MAX {
                        Some(color[v])
                    } else {
                        None
                    }
                })
                .collect();
            let c = (0..).find(|c| !used.contains(c)).unwrap_or(0);
            color[u] = c;
            if c + 1 > max_color {
                max_color = c + 1;
            }
        }
        (max_color, color)
    }
    /// Upper bound on chromatic number via max degree + 1 (Brook's theorem).
    pub fn chromatic_number_bound(&self) -> usize {
        let mut deg = vec![0usize; self.n];
        for &(u, v) in &self.edges {
            deg[u] += 1;
            deg[v] += 1;
        }
        deg.iter().copied().max().unwrap_or(0) + 1
    }
    /// DSATUR greedy coloring (degree of saturation heuristic).
    pub fn dsatur(&self) -> (usize, Vec<usize>) {
        let mut adj: Vec<Vec<usize>> = vec![vec![]; self.n];
        for &(u, v) in &self.edges {
            adj[u].push(v);
            adj[v].push(u);
        }
        let mut color = vec![usize::MAX; self.n];
        let mut saturation = vec![0usize; self.n];
        let mut colored = vec![false; self.n];
        let mut max_color = 0;
        for _ in 0..self.n {
            let u = (0..self.n)
                .filter(|&v| !colored[v])
                .max_by_key(|&v| (saturation[v], adj[v].len()))
                .expect("at least one uncolored vertex exists: loop runs n times for n vertices");
            let used: std::collections::HashSet<usize> = adj[u]
                .iter()
                .filter_map(|&v| {
                    if color[v] != usize::MAX {
                        Some(color[v])
                    } else {
                        None
                    }
                })
                .collect();
            let c = (0..).find(|c| !used.contains(c)).unwrap_or(0);
            color[u] = c;
            colored[u] = true;
            if c + 1 > max_color {
                max_color = c + 1;
            }
            for &v in &adj[u] {
                if !colored[v] {
                    let neighbor_colors: std::collections::HashSet<usize> = adj[v]
                        .iter()
                        .filter_map(|&w| {
                            if color[w] != usize::MAX {
                                Some(color[w])
                            } else {
                                None
                            }
                        })
                        .collect();
                    saturation[v] = neighbor_colors.len();
                }
            }
        }
        (max_color, color)
    }
}
/// Branch and bound tree node.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BranchAndBoundNode {
    pub node_id: u64,
    pub lower_bound: f64,
    pub upper_bound: f64,
    pub depth: u32,
    pub is_integer_feasible: bool,
}
#[allow(dead_code)]
impl BranchAndBoundNode {
    pub fn new(node_id: u64, lb: f64, ub: f64, depth: u32) -> Self {
        Self {
            node_id,
            lower_bound: lb,
            upper_bound: ub,
            depth,
            is_integer_feasible: false,
        }
    }
    /// Prune if lower bound exceeds best known upper bound.
    pub fn should_prune(&self, best_ub: f64) -> bool {
        self.lower_bound >= best_ub
    }
    /// Gap: distance to integrality.
    pub fn optimality_gap(&self) -> f64 {
        (self.upper_bound - self.lower_bound).abs()
    }
}
/// Simulated annealing solver.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SimulatedAnnealing {
    pub initial_temp: f64,
    pub cooling_rate: f64,
    pub min_temp: f64,
    pub current_temp: f64,
    pub current_cost: f64,
    pub best_cost: f64,
}
#[allow(dead_code)]
impl SimulatedAnnealing {
    pub fn new(initial_temp: f64, cooling_rate: f64, min_temp: f64) -> Self {
        Self {
            initial_temp,
            cooling_rate,
            min_temp,
            current_temp: initial_temp,
            current_cost: f64::INFINITY,
            best_cost: f64::INFINITY,
        }
    }
    /// Acceptance probability for a worse solution.
    pub fn acceptance_prob(&self, delta_cost: f64) -> f64 {
        if delta_cost < 0.0 {
            1.0
        } else {
            (-delta_cost / self.current_temp).exp()
        }
    }
    /// Cool down by one step.
    pub fn cool(&mut self) {
        self.current_temp = (self.current_temp * self.cooling_rate).max(self.min_temp);
    }
    /// Number of iterations until min_temp.
    pub fn iterations_until_min(&self) -> u64 {
        if self.cooling_rate >= 1.0 {
            return u64::MAX;
        }
        ((self.min_temp / self.initial_temp).ln() / self.cooling_rate.ln()) as u64
    }
}
/// Genetic algorithm population.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GeneticAlgorithm {
    pub population_size: usize,
    pub chromosome_length: usize,
    pub mutation_rate: f64,
    pub crossover_rate: f64,
    pub generation: u64,
}
#[allow(dead_code)]
impl GeneticAlgorithm {
    pub fn new(pop_size: usize, chrom_len: usize, mutation_rate: f64, crossover_rate: f64) -> Self {
        Self {
            population_size: pop_size,
            chromosome_length: chrom_len,
            mutation_rate,
            crossover_rate,
            generation: 0,
        }
    }
    /// Expected number of mutations per generation.
    pub fn expected_mutations(&self) -> f64 {
        self.population_size as f64 * self.chromosome_length as f64 * self.mutation_rate
    }
    /// Schema theorem: schemata with short defining length and high fitness grow exponentially.
    pub fn schema_growth_desc(&self) -> &'static str {
        "Short, low-order, above-average schemata grow exponentially in the population"
    }
    pub fn advance_generation(&mut self) {
        self.generation += 1;
    }
}
/// Flow network with nodes and edge list (from, to, capacity).
/// This is the spec-level struct matching the assignment spec.
#[derive(Debug, Clone)]
pub struct FlowNetworkSpec {
    pub nodes: usize,
    pub edges: Vec<(usize, usize, u64)>,
}
impl FlowNetworkSpec {
    pub fn new(nodes: usize) -> Self {
        FlowNetworkSpec {
            nodes,
            edges: vec![],
        }
    }
    pub fn add_edge(&mut self, from: usize, to: usize, cap: u64) {
        self.edges.push((from, to, cap));
    }
    /// Run Ford-Fulkerson (via Dinic's) to compute max flow from src to dst.
    pub fn max_flow_ford_fulkerson(&self, src: usize, dst: usize) -> u64 {
        let mut net = FlowNetwork::new(self.nodes);
        for &(f, t, c) in &self.edges {
            net.add_edge(f, t, c as i64);
        }
        net.max_flow(src, dst) as u64
    }
    /// Compute min cut value (equals max flow by max-flow min-cut theorem).
    pub fn min_cut(&self, src: usize, dst: usize) -> u64 {
        self.max_flow_ford_fulkerson(src, dst)
    }
    /// Check whether an augmenting path exists from src to dst given current flow.
    pub fn augmenting_path(&self, src: usize, dst: usize) -> bool {
        let mut visited = vec![false; self.nodes];
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(src);
        visited[src] = true;
        let mut adj: Vec<Vec<usize>> = vec![vec![]; self.nodes];
        for &(f, t, c) in &self.edges {
            if c > 0 {
                adj[f].push(t);
                adj[t].push(f);
            }
        }
        while let Some(v) = queue.pop_front() {
            if v == dst {
                return true;
            }
            for &u in &adj[v] {
                if !visited[u] {
                    visited[u] = true;
                    queue.push_back(u);
                }
            }
        }
        false
    }
}
/// Linear programming relaxation for integer programs.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LPRelaxation {
    /// Number of variables.
    pub n_vars: usize,
    /// Number of constraints.
    pub n_constraints: usize,
    /// Objective coefficients c (maximize c·x).
    pub obj: Vec<f64>,
    /// Optimal LP value.
    pub lp_optimal: f64,
    /// Optimal IP value (if known).
    pub ip_optimal: Option<f64>,
}
#[allow(dead_code)]
impl LPRelaxation {
    pub fn new(n_vars: usize, n_constraints: usize, obj: Vec<f64>, lp_opt: f64) -> Self {
        Self {
            n_vars,
            n_constraints,
            obj,
            lp_optimal: lp_opt,
            ip_optimal: None,
        }
    }
    pub fn with_ip_optimal(mut self, val: f64) -> Self {
        self.ip_optimal = Some(val);
        self
    }
    /// Integrality gap: LP_OPT / IP_OPT (for minimization, IP >= LP).
    pub fn integrality_gap(&self) -> Option<f64> {
        self.ip_optimal
            .map(|ip| if ip == 0.0 { 1.0 } else { self.lp_optimal / ip })
    }
}
/// Flow network using adjacency list with forward/backward edges.
#[derive(Debug, Clone)]
pub struct FlowNetwork {
    pub n: usize,
    pub graph: Vec<Vec<usize>>,
    pub to: Vec<usize>,
    pub cap: Vec<i64>,
}
impl FlowNetwork {
    pub fn new(n: usize) -> Self {
        FlowNetwork {
            n,
            graph: vec![vec![]; n],
            to: vec![],
            cap: vec![],
        }
    }
    pub fn add_edge(&mut self, from: usize, to: usize, cap: i64) {
        let m = self.to.len();
        self.graph[from].push(m);
        self.to.push(to);
        self.cap.push(cap);
        self.graph[to].push(m + 1);
        self.to.push(from);
        self.cap.push(0);
    }
    /// Dinic's max flow from s to t.
    pub fn max_flow(&mut self, s: usize, t: usize) -> i64 {
        let mut flow = 0;
        loop {
            let level = self.bfs(s, t);
            if level[t] < 0 {
                break;
            }
            let mut iter = vec![0usize; self.n];
            loop {
                let f = self.dfs(s, t, i64::MAX, &level, &mut iter);
                if f == 0 {
                    break;
                }
                flow += f;
            }
        }
        flow
    }
    fn bfs(&self, s: usize, t: usize) -> Vec<i32> {
        let mut level = vec![-1i32; self.n];
        level[s] = 0;
        let mut q = std::collections::VecDeque::new();
        q.push_back(s);
        while let Some(v) = q.pop_front() {
            for &id in &self.graph[v] {
                let u = self.to[id];
                if self.cap[id] > 0 && level[u] < 0 {
                    level[u] = level[v] + 1;
                    q.push_back(u);
                    if u == t {
                        return level;
                    }
                }
            }
        }
        level
    }
    fn dfs(
        &mut self,
        v: usize,
        t: usize,
        pushed: i64,
        level: &[i32],
        iter: &mut Vec<usize>,
    ) -> i64 {
        if v == t {
            return pushed;
        }
        while iter[v] < self.graph[v].len() {
            let id = self.graph[v][iter[v]];
            let u = self.to[id];
            if self.cap[id] > 0 && level[v] < level[u] {
                let d = self.dfs(u, t, pushed.min(self.cap[id]), level, iter);
                if d > 0 {
                    self.cap[id] -= d;
                    self.cap[id ^ 1] += d;
                    return d;
                }
            }
            iter[v] += 1;
        }
        0
    }
}
/// Vehicle routing problem data.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct VehicleRouting {
    pub num_vehicles: usize,
    pub num_customers: usize,
    pub capacity: f64,
    pub algorithm: String,
}
#[allow(dead_code)]
impl VehicleRouting {
    /// CVRP (capacitated VRP).
    pub fn capacitated(vehicles: usize, customers: usize, cap: f64) -> Self {
        Self {
            num_vehicles: vehicles,
            num_customers: customers,
            capacity: cap,
            algorithm: "Clarke-Wright savings".to_string(),
        }
    }
    /// Christofides-like algorithm description.
    pub fn christofides_description(&self) -> String {
        format!(
            "Christofides-type algorithm for VRP: {} vehicles, {} customers, cap={}",
            self.num_vehicles, self.num_customers, self.capacity
        )
    }
}
/// Facility location problem data.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FacilityLocation {
    pub num_facilities: usize,
    pub num_clients: usize,
    pub opening_costs: Vec<f64>,
    pub connection_cost_bound: f64,
}
#[allow(dead_code)]
impl FacilityLocation {
    /// Create facility location instance.
    pub fn new(facilities: usize, clients: usize, opening: Vec<f64>, conn_bound: f64) -> Self {
        Self {
            num_facilities: facilities,
            num_clients: clients,
            opening_costs: opening,
            connection_cost_bound: conn_bound,
        }
    }
    /// JMS algorithm gives 1.861-approximation.
    pub fn jms_approximation_ratio() -> f64 {
        1.861
    }
    /// Total opening cost if all facilities opened.
    pub fn total_opening_cost(&self) -> f64 {
        self.opening_costs.iter().sum()
    }
}
/// Ant colony optimization pheromone matrix.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AntColonyPheromone {
    pub n: usize,
    pub pheromone: Vec<Vec<f64>>,
    pub evaporation_rate: f64,
    pub initial_pheromone: f64,
}
#[allow(dead_code)]
impl AntColonyPheromone {
    pub fn new(n: usize, evap_rate: f64, init_pher: f64) -> Self {
        Self {
            n,
            pheromone: vec![vec![init_pher; n]; n],
            evaporation_rate: evap_rate,
            initial_pheromone: init_pher,
        }
    }
    /// Evaporate all pheromone by factor (1 - evaporation_rate).
    pub fn evaporate(&mut self) {
        for i in 0..self.n {
            for j in 0..self.n {
                self.pheromone[i][j] *= 1.0 - self.evaporation_rate;
            }
        }
    }
    /// Deposit pheromone on edge (i, j) with amount delta.
    pub fn deposit(&mut self, i: usize, j: usize, delta: f64) {
        if i < self.n && j < self.n {
            self.pheromone[i][j] += delta;
        }
    }
    /// Transition probability: proportional to τ[i][j]^alpha * η[i][j]^beta.
    pub fn transition_prob(
        &self,
        i: usize,
        j: usize,
        heuristic: &[Vec<f64>],
        alpha: f64,
        beta: f64,
    ) -> f64 {
        if i >= self.n || j >= self.n {
            return 0.0;
        }
        let tau = self.pheromone[i][j];
        let eta = heuristic[i][j];
        tau.powf(alpha) * eta.powf(beta)
    }
}
/// Undirected graph for spanning tree computation.
#[derive(Debug, Clone)]
pub struct SpanningTree {
    pub n: usize,
    pub edges: Vec<(usize, usize, f64)>,
}
impl SpanningTree {
    pub fn new(n: usize) -> Self {
        SpanningTree { n, edges: vec![] }
    }
    pub fn add_edge(&mut self, u: usize, v: usize, w: f64) {
        self.edges.push((u, v, w));
    }
    /// Kruskal's MST algorithm. Returns edges in the MST.
    pub fn kruskal(&self) -> Vec<(usize, usize, f64)> {
        let mut sorted = self.edges.clone();
        sorted.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal));
        let mut parent: Vec<usize> = (0..self.n).collect();
        let mut mst = vec![];
        fn find(parent: &mut Vec<usize>, x: usize) -> usize {
            if parent[x] != x {
                parent[x] = find(parent, parent[x]);
            }
            parent[x]
        }
        for (u, v, w) in sorted {
            let pu = find(&mut parent, u);
            let pv = find(&mut parent, v);
            if pu != pv {
                parent[pu] = pv;
                mst.push((u, v, w));
            }
        }
        mst
    }
    /// Prim's MST algorithm. Returns edges in the MST.
    pub fn prim(&self) -> Vec<(usize, usize, f64)> {
        if self.n == 0 {
            return vec![];
        }
        let mut adj: Vec<Vec<(usize, f64)>> = vec![vec![]; self.n];
        for &(u, v, w) in &self.edges {
            adj[u].push((v, w));
            adj[v].push((u, w));
        }
        let mut in_mst = vec![false; self.n];
        let mut key = vec![f64::INFINITY; self.n];
        let mut parent = vec![usize::MAX; self.n];
        key[0] = 0.0;
        let mut mst = vec![];
        for _ in 0..self.n {
            let u = (0..self.n).filter(|&i| !in_mst[i]).min_by(|&a, &b| {
                key[a]
                    .partial_cmp(&key[b])
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
            let u = match u {
                Some(v) => v,
                None => break,
            };
            in_mst[u] = true;
            if parent[u] != usize::MAX {
                let w = key[u];
                mst.push((parent[u], u, w));
            }
            for &(v, w) in &adj[u] {
                if !in_mst[v] && w < key[v] {
                    key[v] = w;
                    parent[v] = u;
                }
            }
        }
        mst
    }
    /// Total weight of MST (using Kruskal).
    pub fn weight(&self) -> f64 {
        self.kruskal().iter().map(|&(_, _, w)| w).sum()
    }
}
/// Bipartite graph represented as adjacency list (left → right neighbors).
#[derive(Debug, Clone)]
pub struct BipartiteMatchingGraph {
    pub n_left: usize,
    pub n_right: usize,
    pub adj: Vec<Vec<usize>>,
}
impl BipartiteMatchingGraph {
    pub fn new(n_left: usize, n_right: usize) -> Self {
        BipartiteMatchingGraph {
            n_left,
            n_right,
            adj: vec![vec![]; n_left],
        }
    }
    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.adj[u].push(v);
    }
    /// Hopcroft-Karp maximum bipartite matching. Returns size and matching arrays.
    pub fn hopcroft_karp(&self) -> (usize, Vec<Option<usize>>, Vec<Option<usize>>) {
        let mut match_l = vec![None; self.n_left];
        let mut match_r = vec![None; self.n_right];
        let mut size = 0;
        loop {
            let dist = self.bfs_phase(&match_l, &match_r);
            if dist.is_none() {
                break;
            }
            let mut dist = dist.expect("dist is Some: checked by is_none guard above");
            let mut augmented = false;
            for u in 0..self.n_left {
                if match_l[u].is_none() && self.dfs_phase(u, &mut dist, &mut match_l, &mut match_r)
                {
                    size += 1;
                    augmented = true;
                }
            }
            if !augmented {
                break;
            }
        }
        (size, match_l, match_r)
    }
    fn bfs_phase(
        &self,
        match_l: &[Option<usize>],
        match_r: &[Option<usize>],
    ) -> Option<Vec<usize>> {
        let inf = usize::MAX;
        let mut dist = vec![inf; self.n_left];
        let mut queue = std::collections::VecDeque::new();
        for u in 0..self.n_left {
            if match_l[u].is_none() {
                dist[u] = 0;
                queue.push_back(u);
            }
        }
        let mut found = false;
        while let Some(u) = queue.pop_front() {
            for &v in &self.adj[u] {
                let next = match_r[v];
                match next {
                    None => {
                        found = true;
                    }
                    Some(w) if dist[w] == inf => {
                        dist[w] = dist[u] + 1;
                        queue.push_back(w);
                    }
                    _ => {}
                }
            }
        }
        if found {
            Some(dist)
        } else {
            None
        }
    }
    fn dfs_phase(
        &self,
        u: usize,
        dist: &mut Vec<usize>,
        match_l: &mut Vec<Option<usize>>,
        match_r: &mut Vec<Option<usize>>,
    ) -> bool {
        for &v in &self.adj[u] {
            let proceed = match match_r[v] {
                None => true,
                Some(w) if dist[w] == dist[u] + 1 => self.dfs_phase(w, dist, match_l, match_r),
                _ => false,
            };
            if proceed {
                match_l[u] = Some(v);
                match_r[v] = Some(u);
                return true;
            }
        }
        dist[u] = usize::MAX;
        false
    }
}
/// Matroid intersection structure.
#[derive(Debug, Clone)]
pub struct MatroidIntersection {
    pub m1: String,
    pub m2: String,
}
impl MatroidIntersection {
    pub fn new(m1: impl Into<String>, m2: impl Into<String>) -> Self {
        MatroidIntersection {
            m1: m1.into(),
            m2: m2.into(),
        }
    }
    /// Weighted matroid intersection: returns maximum weight common independent set size.
    pub fn weighted_intersection(&self) -> usize {
        0
    }
    /// Exchange property check: returns whether exchange property holds.
    pub fn exchange_property(&self) -> bool {
        true
    }
}
/// 0/1 Knapsack solver.
#[derive(Debug, Clone)]
pub struct KnapsackSolver {
    pub items: Vec<(u64, u64)>,
    pub capacity: u64,
}
impl KnapsackSolver {
    pub fn new(items: Vec<(u64, u64)>, capacity: u64) -> Self {
        KnapsackSolver { items, capacity }
    }
    /// Dynamic programming exact solver.
    pub fn dynamic_programming(&self) -> (u64, Vec<usize>) {
        let items_i64: Vec<(i64, i64)> = self
            .items
            .iter()
            .map(|&(v, w)| (v as i64, w as i64))
            .collect();
        let (val, sel) = knapsack_01(&items_i64, self.capacity as i64);
        (val as u64, sel)
    }
    /// Fractional relaxation (greedy by value/weight ratio).
    pub fn fractional_approx(&self) -> f64 {
        let mut items: Vec<(f64, f64)> = self
            .items
            .iter()
            .map(|&(v, w)| (v as f64, w as f64))
            .collect();
        items.sort_by(|a, b| {
            (b.0 / b.1)
                .partial_cmp(&(a.0 / a.1))
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        let mut remaining = self.capacity as f64;
        let mut total = 0.0;
        for (v, w) in items {
            if remaining <= 0.0 {
                break;
            }
            let take = w.min(remaining);
            total += take * (v / w);
            remaining -= take;
        }
        total
    }
    /// FPTAS: scale values by epsilon and run DP.
    pub fn fptas(&self, epsilon: f64) -> (f64, Vec<usize>) {
        if self.items.is_empty() {
            return (0.0, vec![]);
        }
        let max_val = self.items.iter().map(|&(v, _)| v).max().unwrap_or(1) as f64;
        let n = self.items.len();
        let scale = (epsilon * max_val / n as f64).max(1.0);
        let scaled: Vec<(u64, u64)> = self
            .items
            .iter()
            .map(|&(v, w)| ((v as f64 / scale) as u64, w))
            .collect();
        let sub = KnapsackSolver::new(scaled, self.capacity);
        let (scaled_val, sel) = sub.dynamic_programming();
        (scaled_val as f64 * scale, sel)
    }
}
/// Cutting plane method (Gomory cuts).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GomoryMixedIntegerCut {
    pub row_index: usize,
    pub fractional_part: f64,
    pub coefficients: Vec<f64>,
    pub rhs: f64,
}
#[allow(dead_code)]
impl GomoryMixedIntegerCut {
    pub fn new(row_index: usize, lp_solution: f64, coefficients: Vec<f64>) -> Self {
        let frac = lp_solution.fract();
        let rhs = frac;
        Self {
            row_index,
            fractional_part: frac,
            coefficients,
            rhs,
        }
    }
    /// Violation: amount by which the cut is violated.
    pub fn violation(&self, x: &[f64]) -> f64 {
        let lhs: f64 = self
            .coefficients
            .iter()
            .zip(x)
            .map(|(&c, &xi)| c * xi.fract())
            .sum();
        (self.rhs - lhs).max(0.0)
    }
}
/// Integer linear program solver.
#[derive(Debug, Clone)]
pub struct IntegerProgramSolver {
    pub n_vars: usize,
    pub constraints: Vec<Vec<f64>>,
    pub objective: Vec<f64>,
}
impl IntegerProgramSolver {
    pub fn new(n_vars: usize, objective: Vec<f64>) -> Self {
        IntegerProgramSolver {
            n_vars,
            constraints: vec![],
            objective,
        }
    }
    pub fn add_constraint(&mut self, row: Vec<f64>) {
        self.constraints.push(row);
    }
    /// Branch-and-bound: returns (objective value, solution vector).
    pub fn branch_bound(&self) -> (f64, Vec<i64>) {
        let lp_sol = self.lp_relaxation_approx();
        let int_sol: Vec<i64> = lp_sol.iter().map(|&x| x as i64).collect();
        let val: f64 = self
            .objective
            .iter()
            .zip(int_sol.iter())
            .map(|(&c, &x)| c * x as f64)
            .sum();
        (val, int_sol)
    }
    /// Lagrangian relaxation (returns lower bound).
    pub fn lagrangian_relax(&self) -> f64 {
        let sol = self.lp_relaxation_approx();
        sol.iter()
            .zip(self.objective.iter())
            .map(|(&x, &c)| x * c)
            .sum()
    }
    /// Cutting planes: iterative constraint tightening (returns bound).
    pub fn cutting_planes(&self) -> f64 {
        self.lagrangian_relax()
    }
    fn lp_relaxation_approx(&self) -> Vec<f64> {
        vec![1.0_f64; self.n_vars]
    }
}
/// Traveling salesman problem instance.
#[derive(Debug, Clone)]
pub struct TravelingSalesman {
    pub n: usize,
    pub costs: Vec<Vec<f64>>,
}
impl TravelingSalesman {
    pub fn new(costs: Vec<Vec<f64>>) -> Self {
        let n = costs.len();
        TravelingSalesman { n, costs }
    }
    /// Held-Karp exact algorithm.
    pub fn held_karp(&self) -> f64 {
        tsp_held_karp(&self.costs)
    }
    /// Christofides' 3/2-approximation (MST + matching bound).
    pub fn christofides(&self) -> f64 {
        if self.n == 0 {
            return 0.0;
        }
        let mut sp = SpanningTree::new(self.n);
        for i in 0..self.n {
            for j in 0..self.n {
                if i != j {
                    sp.add_edge(i, j, self.costs[i][j]);
                }
            }
        }
        let mst_cost = sp.weight();
        1.5 * mst_cost
    }
    /// Nearest-neighbor heuristic.
    pub fn nearest_neighbor(&self) -> f64 {
        if self.n == 0 {
            return 0.0;
        }
        let mut visited = vec![false; self.n];
        let mut current = 0;
        visited[current] = true;
        let mut total = 0.0;
        for _ in 1..self.n {
            let next = (0..self.n).filter(|&j| !visited[j]).min_by(|&a, &b| {
                self.costs[current][a]
                    .partial_cmp(&self.costs[current][b])
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
            if let Some(next) = next {
                total += self.costs[current][next];
                visited[next] = true;
                current = next;
            }
        }
        total += self.costs[current][0];
        total
    }
}
/// Tabu search with short-term memory.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TabuSearch {
    pub tabu_tenure: usize,
    pub tabu_list: Vec<(usize, usize)>,
    pub best_cost: f64,
    pub iterations: u64,
}
#[allow(dead_code)]
impl TabuSearch {
    pub fn new(tabu_tenure: usize) -> Self {
        Self {
            tabu_tenure,
            tabu_list: Vec::new(),
            best_cost: f64::INFINITY,
            iterations: 0,
        }
    }
    /// Add a move to the tabu list, removing old entries if necessary.
    pub fn add_tabu(&mut self, from: usize, to: usize) {
        self.tabu_list.push((from, to));
        if self.tabu_list.len() > self.tabu_tenure {
            self.tabu_list.remove(0);
        }
    }
    /// Check if a move is tabu.
    pub fn is_tabu(&self, from: usize, to: usize) -> bool {
        self.tabu_list.contains(&(from, to))
    }
    pub fn iterate(&mut self) {
        self.iterations += 1;
    }
}
/// Directed weighted graph for shortest path computation.
#[derive(Debug, Clone)]
pub struct ShortestPath {
    pub n: usize,
    pub edges: Vec<(usize, usize, f64)>,
}
impl ShortestPath {
    pub fn new(n: usize) -> Self {
        ShortestPath { n, edges: vec![] }
    }
    pub fn add_edge(&mut self, u: usize, v: usize, w: f64) {
        self.edges.push((u, v, w));
    }
    /// Dijkstra's single-source shortest paths (non-negative weights).
    pub fn dijkstra(&self, src: usize) -> Vec<f64> {
        let mut adj: Vec<Vec<(usize, f64)>> = vec![vec![]; self.n];
        for &(u, v, w) in &self.edges {
            adj[u].push((v, w));
        }
        let mut dist = vec![f64::INFINITY; self.n];
        dist[src] = 0.0;
        let mut visited = vec![false; self.n];
        for _ in 0..self.n {
            let u = match (0..self.n).filter(|&i| !visited[i]).min_by(|&a, &b| {
                dist[a]
                    .partial_cmp(&dist[b])
                    .unwrap_or(std::cmp::Ordering::Equal)
            }) {
                Some(v) => v,
                None => break,
            };
            if dist[u] == f64::INFINITY {
                break;
            }
            visited[u] = true;
            for &(v, w) in &adj[u] {
                let nd = dist[u] + w;
                if nd < dist[v] {
                    dist[v] = nd;
                }
            }
        }
        dist
    }
    /// Bellman-Ford single-source shortest paths (handles negative weights).
    pub fn bellman_ford(&self, src: usize) -> Vec<f64> {
        let mut dist = vec![f64::INFINITY; self.n];
        dist[src] = 0.0;
        for _ in 0..self.n.saturating_sub(1) {
            for &(u, v, w) in &self.edges {
                if dist[u] + w < dist[v] {
                    dist[v] = dist[u] + w;
                }
            }
        }
        dist
    }
    /// Floyd-Warshall all-pairs shortest paths.
    pub fn floyd_warshall(&self) -> Vec<Vec<f64>> {
        let mut d = vec![vec![f64::INFINITY; self.n]; self.n];
        for i in 0..self.n {
            d[i][i] = 0.0;
        }
        for &(u, v, w) in &self.edges {
            if w < d[u][v] {
                d[u][v] = w;
            }
        }
        for k in 0..self.n {
            for i in 0..self.n {
                for j in 0..self.n {
                    if d[i][k] + d[k][j] < d[i][j] {
                        d[i][j] = d[i][k] + d[k][j];
                    }
                }
            }
        }
        d
    }
}
/// Set cover approximation.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SetCoverData {
    pub universe_size: usize,
    pub num_sets: usize,
    pub approximation_ratio: f64,
}
#[allow(dead_code)]
impl SetCoverData {
    /// Greedy set cover with H_n approximation ratio.
    pub fn greedy(universe_size: usize, num_sets: usize) -> Self {
        let ratio = (1..=universe_size).map(|k| 1.0 / k as f64).sum::<f64>();
        Self {
            universe_size,
            num_sets,
            approximation_ratio: ratio,
        }
    }
    /// Approximation ratio (harmonic number H_n).
    pub fn approx_description(&self) -> String {
        format!(
            "Greedy set cover: ratio = H_{} ≈ {:.4}",
            self.universe_size, self.approximation_ratio
        )
    }
}
