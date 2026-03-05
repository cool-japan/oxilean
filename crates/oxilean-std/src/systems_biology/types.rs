//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
/// Toggle switch ODE model (Gardner et al. 2000).
///
/// du/dt = α₁ / (1 + v^β) - u
/// dv/dt = α₂ / (1 + u^γ) - v
#[derive(Debug, Clone)]
pub struct ToggleSwitchOde {
    pub alpha1: f64,
    pub alpha2: f64,
    pub beta: f64,
    pub gamma: f64,
}
impl ToggleSwitchOde {
    /// Compute derivatives [du/dt, dv/dt] at state [u, v].
    pub fn derivatives(&self, state: &[f64]) -> [f64; 2] {
        let u = state[0];
        let v = state[1];
        let dudt = self.alpha1 * hill_repression(v, self.beta, 1.0) - u;
        let dvdt = self.alpha2 * hill_repression(u, self.gamma, 1.0) - v;
        [dudt, dvdt]
    }
    /// Integrate using 4th-order Runge-Kutta.
    pub fn integrate(&self, initial: [f64; 2], dt: f64, steps: usize) -> Vec<[f64; 2]> {
        let mut traj = Vec::with_capacity(steps + 1);
        let mut state = initial;
        traj.push(state);
        for _ in 0..steps {
            let k1 = self.derivatives(&state);
            let s2 = [state[0] + 0.5 * dt * k1[0], state[1] + 0.5 * dt * k1[1]];
            let k2 = self.derivatives(&s2);
            let s3 = [state[0] + 0.5 * dt * k2[0], state[1] + 0.5 * dt * k2[1]];
            let k3 = self.derivatives(&s3);
            let s4 = [state[0] + dt * k3[0], state[1] + dt * k3[1]];
            let k4 = self.derivatives(&s4);
            state[0] += dt / 6.0 * (k1[0] + 2.0 * k2[0] + 2.0 * k3[0] + k4[0]);
            state[1] += dt / 6.0 * (k1[1] + 2.0 * k2[1] + 2.0 * k3[1] + k4[1]);
            traj.push(state);
        }
        traj
    }
}
/// Result of a Gillespie simulation run.
#[derive(Debug, Clone)]
pub struct GillespieTrajectory {
    pub times: Vec<f64>,
    pub states: Vec<Vec<i64>>,
}
/// A simple flux balance analysis problem.
///
/// Maximise c·v subject to N·v = 0, lb ≤ v ≤ ub.
#[derive(Debug, Clone)]
pub struct FBAModel {
    pub stoich: Vec<Vec<f64>>,
    pub lower_bounds: Vec<f64>,
    pub upper_bounds: Vec<f64>,
    pub objective: Vec<f64>,
}
impl FBAModel {
    /// Create an FBA model.
    pub fn new(
        stoich: Vec<Vec<f64>>,
        lower_bounds: Vec<f64>,
        upper_bounds: Vec<f64>,
        objective: Vec<f64>,
    ) -> Self {
        FBAModel {
            stoich,
            lower_bounds,
            upper_bounds,
            objective,
        }
    }
    /// Check steady-state feasibility: |N·v|_∞ < tol for a given flux vector.
    pub fn check_steady_state(&self, v: &[f64], tol: f64) -> bool {
        for row in &self.stoich {
            let dot: f64 = row.iter().zip(v.iter()).map(|(n, vi)| n * vi).sum();
            if dot.abs() > tol {
                return false;
            }
        }
        true
    }
    /// Compute the objective value c·v.
    pub fn objective_value(&self, v: &[f64]) -> f64 {
        self.objective
            .iter()
            .zip(v.iter())
            .map(|(ci, vi)| ci * vi)
            .sum()
    }
    /// Check that flux vector satisfies bound constraints.
    pub fn check_bounds(&self, v: &[f64]) -> bool {
        v.iter()
            .enumerate()
            .all(|(i, &vi)| vi >= self.lower_bounds[i] && vi <= self.upper_bounds[i])
    }
}
/// State of the SIR model: susceptible, infected, recovered counts.
#[derive(Debug, Clone, Copy)]
pub struct SIRState {
    pub s: f64,
    pub i: f64,
    pub r: f64,
}
/// A Boolean gene regulatory network backed by a truth-table representation.
#[derive(Debug, Clone)]
pub struct BooleanGeneNetwork {
    /// Number of genes/nodes.
    pub n_genes: usize,
    /// Adjacency matrix: `regulation[i][j] = Some(true)` means gene j activates gene i,
    /// `Some(false)` means repression, `None` means no edge.
    pub regulation: Vec<Vec<Option<bool>>>,
    /// Threshold for each gene: gene i is ON iff
    /// (sum of active activators) - (sum of active repressors) >= threshold[i].
    pub threshold: Vec<i32>,
}
impl BooleanGeneNetwork {
    /// Create a new Boolean gene network.
    pub fn new(n_genes: usize) -> Self {
        BooleanGeneNetwork {
            n_genes,
            regulation: vec![vec![None; n_genes]; n_genes],
            threshold: vec![1; n_genes],
        }
    }
    /// Add a regulatory edge: gene `from` activates/represses gene `to`.
    pub fn add_edge(&mut self, from: usize, to: usize, activates: bool) {
        if from < self.n_genes && to < self.n_genes {
            self.regulation[to][from] = Some(activates);
        }
    }
    /// Set the activation threshold for gene `gene_idx`.
    pub fn set_threshold(&mut self, gene_idx: usize, t: i32) {
        if gene_idx < self.n_genes {
            self.threshold[gene_idx] = t;
        }
    }
    /// Synchronously update all genes.
    pub fn update(&self, state: &[bool]) -> Vec<bool> {
        (0..self.n_genes)
            .map(|i| {
                let score: i32 = (0..self.n_genes)
                    .filter_map(|j| {
                        self.regulation[i][j].map(|activates| {
                            if state[j] {
                                if activates {
                                    1i32
                                } else {
                                    -1i32
                                }
                            } else {
                                0i32
                            }
                        })
                    })
                    .sum();
                score >= self.threshold[i]
            })
            .collect()
    }
    /// Find all attractors by exhaustive enumeration (feasible for n_genes ≤ 16).
    pub fn find_attractors(&self) -> Vec<AttractorInfo> {
        if self.n_genes > 16 {
            return vec![];
        }
        let n_states = 1usize << self.n_genes;
        let successor: Vec<usize> = (0..n_states)
            .map(|s| {
                let state: Vec<bool> = (0..self.n_genes).map(|i| (s >> i) & 1 == 1).collect();
                let next = self.update(&state);
                next.iter()
                    .enumerate()
                    .fold(0usize, |acc, (i, &b)| acc | ((b as usize) << i))
            })
            .collect();
        let mut in_attractor = vec![false; n_states];
        let mut attractor_id = vec![usize::MAX; n_states];
        let mut next_id = 0usize;
        let mut attractors: Vec<AttractorInfo> = Vec::new();
        for start in 0..n_states {
            if attractor_id[start] != usize::MAX {
                continue;
            }
            let mut path = Vec::new();
            let mut cur = start;
            loop {
                if attractor_id[cur] != usize::MAX {
                    let aid = attractor_id[cur];
                    for &s in &path {
                        attractor_id[s] = aid;
                    }
                    break;
                }
                if let Some(pos) = path.iter().position(|&x| x == cur) {
                    let cycle: Vec<usize> = path[pos..].to_vec();
                    let states: Vec<Vec<bool>> = cycle
                        .iter()
                        .map(|&s| (0..self.n_genes).map(|i| (s >> i) & 1 == 1).collect())
                        .collect();
                    let period = cycle.len();
                    attractors.push(AttractorInfo {
                        states,
                        period,
                        attractor_id: next_id,
                    });
                    for &s in &cycle {
                        attractor_id[s] = next_id;
                        in_attractor[s] = true;
                    }
                    let aid = next_id;
                    for &s in &path[..pos] {
                        attractor_id[s] = aid;
                    }
                    next_id += 1;
                    break;
                }
                path.push(cur);
                cur = successor[cur];
            }
        }
        attractors
    }
    /// Compute the basin of attraction sizes.
    pub fn basin_sizes(&self) -> Vec<usize> {
        if self.n_genes > 20 {
            return vec![];
        }
        let n_states = 1usize << self.n_genes;
        let attractors = self.find_attractors();
        let n_attr = attractors.len();
        if n_attr == 0 {
            return vec![];
        }
        let mut state_to_attr: Vec<Option<usize>> = vec![None; n_states];
        for (ai, attr) in attractors.iter().enumerate() {
            for states in &attr.states {
                let idx = states
                    .iter()
                    .enumerate()
                    .fold(0usize, |acc, (i, &b)| acc | ((b as usize) << i));
                state_to_attr[idx] = Some(ai);
            }
        }
        let mut basin = vec![0usize; n_attr];
        for s in 0..n_states {
            if let Some(ai) = state_to_attr[s] {
                basin[ai] += 1;
            }
        }
        basin
    }
}
/// A biochemical Petri net.
#[derive(Debug, Clone)]
pub struct PetriNet {
    pub places: Vec<String>,
    pub transitions: Vec<PetriTransition>,
    pub marking: Vec<usize>,
}
impl PetriNet {
    /// Create a new Petri net with initial marking.
    pub fn new(places: Vec<String>, marking: Vec<usize>) -> Self {
        PetriNet {
            places,
            transitions: Vec::new(),
            marking,
        }
    }
    /// Add a transition.
    pub fn add_transition(&mut self, t: PetriTransition) {
        self.transitions.push(t);
    }
    /// Check if a transition is enabled in the current marking.
    pub fn is_enabled(&self, t_idx: usize) -> bool {
        let t = &self.transitions[t_idx];
        if t.pre.len() != self.marking.len() {
            return false;
        }
        t.pre
            .iter()
            .zip(self.marking.iter())
            .all(|(&req, &avail)| avail >= req)
    }
    /// Fire a transition (if enabled) and update the marking.
    ///
    /// Returns true if the transition fired.
    pub fn fire(&mut self, t_idx: usize) -> bool {
        if !self.is_enabled(t_idx) {
            return false;
        }
        let pre = self.transitions[t_idx].pre.clone();
        let post = self.transitions[t_idx].post.clone();
        for (i, &req) in pre.iter().enumerate() {
            if i < self.marking.len() {
                self.marking[i] -= req;
            }
        }
        for (i, &add) in post.iter().enumerate() {
            if i < self.marking.len() {
                self.marking[i] += add;
            }
        }
        true
    }
    /// Check if a target marking is reachable by BFS (small nets only).
    pub fn is_reachable(&self, target: &[usize], max_steps: usize) -> bool {
        let mut frontier = std::collections::HashSet::new();
        frontier.insert(self.marking.clone());
        let mut visited = frontier.clone();
        for _ in 0..max_steps {
            let mut next_frontier = std::collections::HashSet::new();
            for marking in &frontier {
                if marking == target {
                    return true;
                }
                let mut net_copy = self.clone();
                net_copy.marking = marking.clone();
                for ti in 0..self.transitions.len() {
                    if net_copy.is_enabled(ti) {
                        let mut fired = net_copy.clone();
                        fired.fire(ti);
                        if !visited.contains(&fired.marking) {
                            visited.insert(fired.marking.clone());
                            next_frontier.insert(fired.marking.clone());
                        }
                    }
                }
            }
            if next_frontier.is_empty() {
                break;
            }
            frontier = next_frontier;
        }
        frontier.iter().any(|m| m == target)
    }
}
/// A Boolean regulatory network with synchronous update.
pub struct BooleanNetwork {
    pub n_nodes: usize,
    /// Update function for each node: f_i(state) → bool
    pub update_fns: Vec<Box<dyn Fn(&[bool]) -> bool + Send + Sync>>,
}
impl BooleanNetwork {
    /// Synchronously update all nodes.
    pub fn update(&self, state: &[bool]) -> Vec<bool> {
        self.update_fns.iter().map(|f| f(state)).collect()
    }
    /// Find all attractors by exhaustive simulation (only feasible for small n).
    pub fn find_attractors(&self) -> Vec<Vec<bool>> {
        if self.n_nodes > 16 {
            return vec![];
        }
        let n_states = 1usize << self.n_nodes;
        let mut attractor_states = Vec::new();
        for start in 0..n_states {
            let initial: Vec<bool> = (0..self.n_nodes).map(|i| (start >> i) & 1 == 1).collect();
            let mut visited = std::collections::HashSet::new();
            let mut current = initial.clone();
            loop {
                if !visited.insert(
                    current
                        .iter()
                        .enumerate()
                        .fold(0u64, |acc, (i, &b)| acc | ((b as u64) << i)),
                ) {
                    if !attractor_states.contains(&current) {
                        attractor_states.push(current.clone());
                    }
                    break;
                }
                current = self.update(&current);
            }
        }
        attractor_states
    }
    /// Compute the state transition graph.
    pub fn transition_graph(&self) -> Vec<(u64, u64)> {
        if self.n_nodes > 20 {
            return vec![];
        }
        let n_states = 1u64 << self.n_nodes;
        (0..n_states)
            .map(|s| {
                let state: Vec<bool> = (0..self.n_nodes).map(|i| (s >> i) & 1 == 1).collect();
                let next = self.update(&state);
                let next_idx = next
                    .iter()
                    .enumerate()
                    .fold(0u64, |acc, (i, &b)| acc | ((b as u64) << i));
                (s, next_idx)
            })
            .collect()
    }
}
/// Configuration for a Gillespie stochastic simulation.
#[derive(Debug, Clone)]
pub struct GillespieAlgorithm {
    pub network: ReactionNetwork,
    pub t_max: f64,
    pub max_steps: usize,
    pub rng_seed: u64,
}
impl GillespieAlgorithm {
    /// Create a new Gillespie simulation configuration.
    pub fn new(network: ReactionNetwork, t_max: f64, max_steps: usize, rng_seed: u64) -> Self {
        GillespieAlgorithm {
            network,
            t_max,
            max_steps,
            rng_seed,
        }
    }
    /// Run the simulation from `initial_state`.
    pub fn run(&self, initial_state: Vec<i64>) -> GillespieTrajectory {
        gillespie_ssa(
            &self.network,
            initial_state,
            self.t_max,
            self.max_steps,
            self.rng_seed,
        )
    }
    /// Estimate the mean species count at time `t` via `n_runs` independent trajectories.
    pub fn estimate_mean(&self, initial_state: Vec<i64>, t: f64, n_runs: usize) -> Vec<f64> {
        let n_species = initial_state.len();
        let mut sums = vec![0.0f64; n_species];
        let mut count = 0usize;
        for run in 0..n_runs {
            let seed = self
                .rng_seed
                .wrapping_add((run as u64).wrapping_mul(6364136223846793005));
            let traj = gillespie_ssa(
                &self.network,
                initial_state.clone(),
                t,
                self.max_steps,
                seed,
            );
            if let Some(last_state) = traj.states.last() {
                for (i, &x) in last_state.iter().enumerate() {
                    if i < n_species {
                        sums[i] += x as f64;
                    }
                }
                count += 1;
            }
        }
        if count > 0 {
            sums.iter().map(|&s| s / count as f64).collect()
        } else {
            vec![0.0; n_species]
        }
    }
    /// Compute empirical probability distribution over final states.
    pub fn empirical_distribution(
        &self,
        initial_state: Vec<i64>,
        n_runs: usize,
    ) -> std::collections::HashMap<Vec<i64>, f64> {
        let mut counts: std::collections::HashMap<Vec<i64>, usize> =
            std::collections::HashMap::new();
        for run in 0..n_runs {
            let seed = self
                .rng_seed
                .wrapping_add((run as u64).wrapping_mul(2862933555777941757));
            let traj = gillespie_ssa(
                &self.network,
                initial_state.clone(),
                self.t_max,
                self.max_steps,
                seed,
            );
            if let Some(last_state) = traj.states.last() {
                *counts.entry(last_state.clone()).or_insert(0) += 1;
            }
        }
        let total = counts.values().sum::<usize>() as f64;
        counts
            .into_iter()
            .map(|(state, c)| (state, c as f64 / total))
            .collect()
    }
}
/// Information about a detected attractor.
#[derive(Debug, Clone)]
pub struct AttractorInfo {
    /// Sequence of Boolean states in the attractor cycle.
    pub states: Vec<Vec<bool>>,
    /// Period of the cycle (1 = fixed point).
    pub period: usize,
    /// Internal attractor identifier.
    pub attractor_id: usize,
}
/// Michaelis-Menten enzyme kinetics parameters and solver.
#[derive(Debug, Clone)]
pub struct MichaelisMentenKinetics {
    /// Maximum reaction velocity.
    pub v_max: f64,
    /// Michaelis constant (substrate concentration at half-maximal velocity).
    pub km: f64,
}
impl MichaelisMentenKinetics {
    /// Create a new MM kinetics instance.
    pub fn new(v_max: f64, km: f64) -> Self {
        MichaelisMentenKinetics { v_max, km }
    }
    /// Compute reaction velocity v = Vmax * S / (Km + S).
    pub fn velocity(&self, substrate: f64) -> f64 {
        if self.km <= 0.0 || substrate < 0.0 {
            return 0.0;
        }
        self.v_max * substrate / (self.km + substrate)
    }
    /// Solve for steady-state substrate concentration given production rate p:
    /// p = Vmax * S / (Km + S)  →  S = p * Km / (Vmax - p)
    ///
    /// Returns `None` if no physical solution exists (p ≥ Vmax).
    pub fn steady_state_substrate(&self, production_rate: f64) -> Option<f64> {
        if production_rate <= 0.0 {
            return Some(0.0);
        }
        if production_rate >= self.v_max {
            return None;
        }
        let s_ss = production_rate * self.km / (self.v_max - production_rate);
        Some(s_ss.max(0.0))
    }
    /// Compute the Hill coefficient (cooperativity) for a sigmoidal variant.
    ///
    /// Given two substrate levels s10 and s90 at 10% and 90% of Vmax,
    /// estimates the Hill coefficient n ≈ log(81) / log(s90 / s10).
    pub fn estimate_hill_coefficient(s10: f64, s90: f64) -> f64 {
        if s10 <= 0.0 || s90 <= s10 {
            return 1.0;
        }
        81f64.ln() / (s90 / s10).ln()
    }
    /// Lineweaver-Burk (double reciprocal) linearization.
    ///
    /// Returns (1/V, 1/S) for a given substrate concentration.
    pub fn lineweaver_burk(&self, substrate: f64) -> Option<(f64, f64)> {
        let v = self.velocity(substrate);
        if v <= 0.0 || substrate <= 0.0 {
            return None;
        }
        Some((1.0 / v, 1.0 / substrate))
    }
}
/// A 2×2 Jacobian matrix for stability analysis of planar systems.
#[derive(Debug, Clone, Copy)]
pub struct Jacobian2x2 {
    pub j00: f64,
    pub j01: f64,
    pub j10: f64,
    pub j11: f64,
}
impl Jacobian2x2 {
    /// Trace and determinant.
    pub fn trace(&self) -> f64 {
        self.j00 + self.j11
    }
    pub fn det(&self) -> f64 {
        self.j00 * self.j11 - self.j01 * self.j10
    }
    /// Compute eigenvalues of the 2×2 Jacobian.
    pub fn eigenvalues(&self) -> Eigenvalues2x2 {
        let tr = self.trace();
        let det = self.det();
        let discriminant = tr * tr - 4.0 * det;
        if discriminant >= 0.0 {
            let sq = discriminant.sqrt();
            Eigenvalues2x2::Real((tr + sq) / 2.0, (tr - sq) / 2.0)
        } else {
            Eigenvalues2x2::Complex {
                real: tr / 2.0,
                imag: (-discriminant).sqrt() / 2.0,
            }
        }
    }
    /// Classify the equilibrium stability.
    pub fn classify(&self) -> Stability {
        let tr = self.trace();
        let det = self.det();
        if det < 0.0 {
            return Stability::SaddlePoint;
        }
        let discriminant = tr * tr - 4.0 * det;
        if discriminant < 0.0 {
            if tr < -1e-12 {
                Stability::StableFocus
            } else if tr > 1e-12 {
                Stability::UnstableFocus
            } else {
                Stability::Center
            }
        } else {
            if tr < -1e-12 {
                Stability::StableNode
            } else if tr > 1e-12 {
                Stability::UnstableNode
            } else {
                Stability::Center
            }
        }
    }
}
/// Stability classification for an equilibrium.
#[derive(Debug, Clone, PartialEq)]
pub enum Stability {
    StableNode,
    UnstableNode,
    SaddlePoint,
    StableFocus,
    UnstableFocus,
    Center,
}
/// Parameters for the Lotka-Volterra predator-prey model.
#[derive(Debug, Clone)]
pub struct LotkaVolterraSimulation {
    /// Prey birth rate α.
    pub alpha: f64,
    /// Predation rate β.
    pub beta: f64,
    /// Predator death rate γ.
    pub gamma: f64,
    /// Predator reproduction efficiency δ.
    pub delta: f64,
}
impl LotkaVolterraSimulation {
    /// Create a new Lotka-Volterra model.
    pub fn new(alpha: f64, beta: f64, gamma: f64, delta: f64) -> Self {
        LotkaVolterraSimulation {
            alpha,
            beta,
            gamma,
            delta,
        }
    }
    /// Compute derivatives [dx/dt, dy/dt] (prey x, predator y).
    pub fn derivatives(&self, prey: f64, pred: f64) -> (f64, f64) {
        let dxdt = self.alpha * prey - self.beta * prey * pred;
        let dydt = self.delta * prey * pred - self.gamma * pred;
        (dxdt, dydt)
    }
    /// Conserved quantity (Lyapunov function) V = δx - γ ln x + βy - α ln y.
    pub fn conserved_quantity(&self, prey: f64, pred: f64) -> f64 {
        if prey <= 0.0 || pred <= 0.0 {
            return f64::INFINITY;
        }
        self.delta * prey - self.gamma * prey.ln() + self.beta * pred - self.alpha * pred.ln()
    }
    /// Integrate using 4th-order Runge-Kutta.
    pub fn simulate(&self, prey0: f64, pred0: f64, dt: f64, steps: usize) -> Vec<(f64, f64)> {
        let mut traj = Vec::with_capacity(steps + 1);
        let mut x = prey0;
        let mut y = pred0;
        traj.push((x, y));
        for _ in 0..steps {
            let (k1x, k1y) = self.derivatives(x, y);
            let (k2x, k2y) = self.derivatives(x + 0.5 * dt * k1x, y + 0.5 * dt * k1y);
            let (k3x, k3y) = self.derivatives(x + 0.5 * dt * k2x, y + 0.5 * dt * k2y);
            let (k4x, k4y) = self.derivatives(x + dt * k3x, y + dt * k3y);
            x += dt / 6.0 * (k1x + 2.0 * k2x + 2.0 * k3x + k4x);
            y += dt / 6.0 * (k1y + 2.0 * k2y + 2.0 * k3y + k4y);
            x = x.max(0.0);
            y = y.max(0.0);
            traj.push((x, y));
        }
        traj
    }
    /// Non-trivial equilibrium: (γ/δ, α/β).
    pub fn coexistence_equilibrium(&self) -> (f64, f64) {
        (self.gamma / self.delta, self.alpha / self.beta)
    }
}
/// State and probability pair for the CME.
#[derive(Debug, Clone)]
pub struct CmeState {
    pub counts: Vec<i64>,
    pub probability: f64,
}
/// A reaction network.
#[derive(Debug, Clone)]
pub struct ReactionNetwork {
    pub species: Vec<String>,
    pub reactions: Vec<ChemReaction>,
}
impl ReactionNetwork {
    /// Create a new empty reaction network.
    pub fn new(species: Vec<String>) -> Self {
        ReactionNetwork {
            species,
            reactions: Vec::new(),
        }
    }
    /// Add a reaction to the network.
    pub fn add_reaction(&mut self, rxn: ChemReaction) {
        self.reactions.push(rxn);
    }
    /// Build the stoichiometric matrix N (species × reactions).
    ///
    /// N[i][j] = net stoichiometric coefficient of species i in reaction j.
    pub fn stoichiometric_matrix(&self) -> Vec<Vec<i32>> {
        let m = self.species.len();
        let n = self.reactions.len();
        let mut matrix = vec![vec![0i32; n]; m];
        for (j, rxn) in self.reactions.iter().enumerate() {
            for &(si, coeff) in &rxn.reactants {
                if si < m {
                    matrix[si][j] -= coeff;
                }
            }
            for &(si, coeff) in &rxn.products {
                if si < m {
                    matrix[si][j] += coeff;
                }
            }
        }
        matrix
    }
    /// Compute propensity functions for a given state.
    ///
    /// Uses mass-action kinetics: a_j(x) = k_j * ∏ C(x_i, v_ij).
    pub fn propensities(&self, state: &[i64]) -> Vec<f64> {
        self.reactions
            .iter()
            .map(|rxn| {
                let mut prop = rxn.rate_constant;
                for &(si, stoich) in &rxn.reactants {
                    if si < state.len() {
                        let x = state[si];
                        for k in 0..stoich {
                            if x - k as i64 <= 0 {
                                prop = 0.0;
                                break;
                            }
                            prop *= (x - k as i64) as f64;
                        }
                        let mut factorial = 1i64;
                        for k in 1..=stoich as i64 {
                            factorial *= k;
                        }
                        prop /= factorial as f64;
                    }
                }
                prop.max(0.0)
            })
            .collect()
    }
}
/// A Petri net transition.
#[derive(Debug, Clone)]
pub struct PetriTransition {
    pub name: String,
    pub pre: Vec<usize>,
    pub post: Vec<usize>,
}
/// A chemical reaction with stoichiometry and rate constant.
#[derive(Debug, Clone)]
pub struct ChemReaction {
    pub name: String,
    pub reactants: Vec<(usize, i32)>,
    pub products: Vec<(usize, i32)>,
    pub rate_constant: f64,
}
/// Parameters for the SIR epidemic model.
#[derive(Debug, Clone)]
pub struct SIREpidemicModel {
    /// Transmission rate β (contact rate × probability of transmission).
    pub beta: f64,
    /// Recovery rate γ (1/infectious_period).
    pub gamma: f64,
    /// Total population size N.
    pub population: f64,
}
impl SIREpidemicModel {
    /// Create a new SIR model.
    pub fn new(beta: f64, gamma: f64, population: f64) -> Self {
        SIREpidemicModel {
            beta,
            gamma,
            population,
        }
    }
    /// Basic reproduction number R₀ = β/γ.
    pub fn r0(&self) -> f64 {
        self.beta / self.gamma
    }
    /// Compute the time derivative [dS/dt, dI/dt, dR/dt].
    pub fn derivatives(&self, state: SIRState) -> SIRState {
        let n = self.population;
        let dsdt = -self.beta * state.s * state.i / n;
        let didt = self.beta * state.s * state.i / n - self.gamma * state.i;
        let drdt = self.gamma * state.i;
        SIRState {
            s: dsdt,
            i: didt,
            r: drdt,
        }
    }
    /// Integrate using 4th-order Runge-Kutta and return trajectory.
    pub fn simulate(&self, initial: SIRState, dt: f64, steps: usize) -> Vec<SIRState> {
        let mut traj = Vec::with_capacity(steps + 1);
        let mut state = initial;
        traj.push(state);
        for _ in 0..steps {
            let k1 = self.derivatives(state);
            let s2 = SIRState {
                s: state.s + 0.5 * dt * k1.s,
                i: state.i + 0.5 * dt * k1.i,
                r: state.r + 0.5 * dt * k1.r,
            };
            let k2 = self.derivatives(s2);
            let s3 = SIRState {
                s: state.s + 0.5 * dt * k2.s,
                i: state.i + 0.5 * dt * k2.i,
                r: state.r + 0.5 * dt * k2.r,
            };
            let k3 = self.derivatives(s3);
            let s4 = SIRState {
                s: state.s + dt * k3.s,
                i: state.i + dt * k3.i,
                r: state.r + dt * k3.r,
            };
            let k4 = self.derivatives(s4);
            state = SIRState {
                s: state.s + dt / 6.0 * (k1.s + 2.0 * k2.s + 2.0 * k3.s + k4.s),
                i: state.i + dt / 6.0 * (k1.i + 2.0 * k2.i + 2.0 * k3.i + k4.i),
                r: state.r + dt / 6.0 * (k1.r + 2.0 * k2.r + 2.0 * k3.r + k4.r),
            };
            state.s = state.s.max(0.0);
            state.i = state.i.max(0.0);
            state.r = state.r.max(0.0);
            traj.push(state);
        }
        traj
    }
    /// Find peak infection time step.
    pub fn peak_infected_step(&self, traj: &[SIRState]) -> usize {
        traj.iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.i.partial_cmp(&b.i).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i)
            .unwrap_or(0)
    }
}
/// Eigenvalue result for a 2×2 matrix.
#[derive(Debug, Clone)]
pub enum Eigenvalues2x2 {
    Real(f64, f64),
    Complex { real: f64, imag: f64 },
}
