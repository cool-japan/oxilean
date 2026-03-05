//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
use super::functions::*;

/// MacArthur-Wilson island biogeography model.
///
/// Immigration rate I(S): decreasing function of current species richness S.
/// Extinction rate E(S): increasing function of current species richness S.
/// dS/dt = I(S) − E(S)
#[allow(dead_code)]
pub struct IslandBiogeographyModel {
    /// Maximum immigration rate (when island is empty).
    pub immigration_max: f64,
    /// Maximum extinction rate (at maximum richness).
    pub extinction_max: f64,
    /// Size of the mainland species pool P.
    pub mainland_pool: usize,
}
impl IslandBiogeographyModel {
    /// Create a new island biogeography model.
    pub fn new(immigration_max: f64, extinction_max: f64, mainland_pool: usize) -> Self {
        IslandBiogeographyModel {
            immigration_max,
            extinction_max,
            mainland_pool,
        }
    }
    /// Immigration rate at current richness S.
    /// I(S) = I_max · (1 − S/P): linear decrease as slots fill.
    pub fn immigration_rate(&self, s: f64) -> f64 {
        let p = self.mainland_pool as f64;
        self.immigration_max * (1.0 - s / p).max(0.0)
    }
    /// Extinction rate at current richness S.
    /// E(S) = E_max · S/P: linear increase with richness.
    pub fn extinction_rate(&self, s: f64) -> f64 {
        let p = self.mainland_pool as f64;
        self.extinction_max * s / p
    }
    /// Net change in species richness dS/dt = I(S) − E(S).
    pub fn dsdt(&self, s: f64) -> f64 {
        self.immigration_rate(s) - self.extinction_rate(s)
    }
    /// Equilibrium species richness S* = I_max · P / (I_max + E_max).
    pub fn equilibrium_richness(&self) -> f64 {
        let p = self.mainland_pool as f64;
        self.immigration_max * p / (self.immigration_max + self.extinction_max)
    }
    /// Turnover rate T = I(S*) = E(S*) at equilibrium.
    pub fn turnover_rate(&self) -> f64 {
        let s_star = self.equilibrium_richness();
        self.immigration_rate(s_star)
    }
    /// Simulate species richness trajectory using Euler method.
    pub fn simulate(&self, s0: f64, dt: f64, steps: usize) -> Vec<(f64, f64)> {
        let mut traj = Vec::with_capacity(steps + 1);
        let mut s = s0.max(0.0);
        let mut t = 0.0_f64;
        let p = self.mainland_pool as f64;
        traj.push((t, s));
        for _ in 0..steps {
            let ds = self.dsdt(s);
            s = (s + dt * ds).clamp(0.0, p);
            t += dt;
            traj.push((t, s));
        }
        traj
    }
}
/// Leslie matrix model for age-structured population dynamics.
#[derive(Debug, Clone)]
pub struct LeslieMatrix {
    /// The Leslie matrix (n×n).
    pub matrix: Vec<Vec<f64>>,
    /// Number of age classes.
    pub n_classes: usize,
}
impl LeslieMatrix {
    /// Construct a Leslie matrix from fecundities (first row) and survival probabilities.
    ///
    /// `fecundities[i]` = per-capita offspring produced by age class i.
    /// `survivals[i]` = probability of surviving from age class i to i+1.
    pub fn new(fecundities: Vec<f64>, survivals: Vec<f64>) -> Self {
        let n = fecundities.len();
        let mut matrix = vec![vec![0.0; n]; n];
        for (j, &f) in fecundities.iter().enumerate() {
            matrix[0][j] = f;
        }
        for i in 0..(n - 1) {
            if i < survivals.len() {
                matrix[i + 1][i] = survivals[i];
            }
        }
        LeslieMatrix {
            matrix,
            n_classes: n,
        }
    }
    /// Apply the Leslie matrix: N(t+1) = L · N(t).
    pub fn project(&self, n: &[f64]) -> Vec<f64> {
        let mut result = vec![0.0; self.n_classes];
        for i in 0..self.n_classes {
            let mut sum = 0.0;
            for j in 0..self.n_classes {
                sum += self.matrix[i][j] * n[j];
            }
            result[i] = sum;
        }
        result
    }
    /// Simulate population dynamics for `steps` time periods.
    ///
    /// Returns a vector of population state vectors.
    pub fn simulate(&self, n0: Vec<f64>, steps: usize) -> Vec<Vec<f64>> {
        let mut history = Vec::with_capacity(steps + 1);
        let mut n = n0;
        history.push(n.clone());
        for _ in 0..steps {
            n = self.project(&n);
            history.push(n.clone());
        }
        history
    }
    /// Approximate dominant eigenvalue (finite rate of increase λ₁) using power iteration.
    pub fn dominant_eigenvalue(&self, iterations: usize) -> f64 {
        let n = self.n_classes;
        let mut v: Vec<f64> = vec![1.0; n];
        let mut lambda = 1.0_f64;
        for _ in 0..iterations {
            let w = self.project(&v);
            let norm: f64 = w.iter().map(|x| x * x).sum::<f64>().sqrt();
            if norm < 1e-15 {
                break;
            }
            lambda = norm / v.iter().map(|x| x * x).sum::<f64>().sqrt();
            v = w.iter().map(|x| x / norm).collect();
        }
        lambda
    }
    /// Net reproductive rate R₀ = Σ l_x m_x.
    ///
    /// `l_x` = survivorship to age x (product of survival probs up to age x).
    /// `m_x` = fecundity at age x (first row of Leslie matrix).
    pub fn net_reproductive_rate(&self) -> f64 {
        let fecundities: Vec<f64> = self.matrix[0].clone();
        let mut r0 = 0.0_f64;
        let mut lx = 1.0_f64;
        for x in 0..self.n_classes {
            r0 += lx * fecundities[x];
            if x + 1 < self.n_classes {
                let sx = self.matrix[x + 1][x];
                lx *= sx;
            }
        }
        r0
    }
}
/// Generalised n-species Lotka-Volterra competition model.
///
/// dN_i/dt = r_i·N_i·(1 − Σ_j α_ij·N_j / K_i)
#[derive(Debug, Clone)]
pub struct CompetitionModel {
    /// Number of species.
    pub n: usize,
    /// Interaction (competition) matrix α_ij.
    pub interaction_matrix: Vec<Vec<f64>>,
}
impl CompetitionModel {
    /// Create a competition model with n species.
    pub fn new(n: usize, interaction_matrix: Vec<Vec<f64>>) -> Self {
        assert_eq!(interaction_matrix.len(), n);
        CompetitionModel {
            n,
            interaction_matrix,
        }
    }
    /// Returns the per-capita competition effect on species i from species j.
    pub fn lotka_volterra_competition(&self, i: usize, j: usize) -> f64 {
        self.interaction_matrix[i][j]
    }
    /// Coexistence condition: all species can invade when rare (simplified check).
    /// Returns true if the diagonal (intraspecific) competition > off-diagonal (interspecific).
    pub fn coexistence_condition(&self) -> bool {
        for i in 0..self.n {
            let intra = self.interaction_matrix[i][i];
            for j in 0..self.n {
                if i != j && self.interaction_matrix[i][j] >= intra {
                    return false;
                }
            }
        }
        true
    }
    /// Competitive exclusion: returns true if one species dominates all others.
    pub fn exclusion_principle(&self) -> bool {
        !self.coexistence_condition()
    }
}
/// Compartmental epidemic model variants.
#[derive(Debug, Clone)]
pub enum EpidemicModel {
    /// SIR with transmission rate β and recovery rate γ.
    SIR(f64, f64),
    /// SEIR with β, γ, and rate of progression from exposed σ.
    SEIR(f64, f64, f64),
    /// SIRS with β, γ, and rate of loss of immunity δ.
    SIRS(f64, f64, f64),
}
impl EpidemicModel {
    /// Basic reproduction number R₀.
    /// SIR: β/γ, SEIR: β/γ, SIRS: β/γ.
    pub fn basic_reproduction_number(&self) -> f64 {
        match self {
            EpidemicModel::SIR(beta, gamma) => beta / gamma,
            EpidemicModel::SEIR(beta, gamma, _sigma) => beta / gamma,
            EpidemicModel::SIRS(beta, gamma, _delta) => beta / gamma,
        }
    }
    /// Herd immunity threshold: p_c = 1 − 1/R₀.
    pub fn herd_immunity_threshold(&self) -> f64 {
        let r0 = self.basic_reproduction_number();
        if r0 <= 1.0 {
            0.0
        } else {
            1.0 - 1.0 / r0
        }
    }
    /// Approximate peak infection fraction.
    /// For SIR starting with S₀ ≈ 1: I_peak ≈ 1 − 1/R₀ − ln(R₀)/R₀.
    pub fn peak_infection(&self) -> f64 {
        let r0 = self.basic_reproduction_number();
        if r0 <= 1.0 {
            return 0.0;
        }
        let s_peak = 1.0 / r0;
        1.0 - s_peak - s_peak.ln() / r0
    }
}
/// Multi-species population dynamics model using logistic growth.
///
/// Tracks carrying capacity and intrinsic growth rates for each species.
#[derive(Debug, Clone)]
pub struct PopulationDynamics {
    /// Species names.
    pub species: Vec<String>,
    /// Carrying capacities K_i for each species.
    pub carrying_capacity: Vec<f64>,
    /// Intrinsic growth rates r_i for each species.
    pub growth_rates: Vec<f64>,
}
impl PopulationDynamics {
    /// Create a new population dynamics model.
    pub fn new(species: Vec<String>, carrying_capacity: Vec<f64>, growth_rates: Vec<f64>) -> Self {
        assert_eq!(species.len(), carrying_capacity.len());
        assert_eq!(species.len(), growth_rates.len());
        PopulationDynamics {
            species,
            carrying_capacity,
            growth_rates,
        }
    }
    /// Logistic growth rate: dN/dt = r·N·(1 − N/K).
    pub fn logistic_growth(n: f64, r: f64, k: f64) -> f64 {
        r * n * (1.0 - n / k)
    }
    /// Perform one Euler step: update all species populations.
    pub fn euler_step(&self, populations: &mut Vec<f64>, dt: f64) {
        for i in 0..self.species.len() {
            let dn = Self::logistic_growth(
                populations[i],
                self.growth_rates[i],
                self.carrying_capacity[i],
            );
            populations[i] += dt * dn;
            if populations[i] < 0.0 {
                populations[i] = 0.0;
            }
        }
    }
    /// Compute equilibrium population sizes (= carrying capacity for single species).
    pub fn equilibrium(&self) -> Vec<f64> {
        self.carrying_capacity.clone()
    }
}
/// Adaptive dynamics simulator using the canonical equation.
///
/// Tracks the evolutionary trajectory of a quantitative trait x(t)
/// under selection, following the canonical equation:
///   dx/dt = (1/2) · μ · σ² · n*(x) · D(x)
/// where D(x) = ∂s(y,x)/∂y |_{y=x} is the selection gradient.
#[allow(dead_code)]
pub struct AdaptiveDynamicsSimulator {
    /// Mutation rate μ.
    pub mutation_rate: f64,
    /// Mutational variance σ².
    pub mutation_variance: f64,
    /// Current resident trait value.
    pub trait_value: f64,
    /// Trajectory of (time, trait_value).
    pub trajectory: Vec<(f64, f64)>,
}
impl AdaptiveDynamicsSimulator {
    /// Create a new adaptive dynamics simulator.
    pub fn new(mutation_rate: f64, mutation_variance: f64, initial_trait: f64) -> Self {
        AdaptiveDynamicsSimulator {
            mutation_rate,
            mutation_variance,
            trait_value: initial_trait,
            trajectory: vec![(0.0, initial_trait)],
        }
    }
    /// Selection gradient D(x) = ∂s(y,x)/∂y |_{y=x}.
    /// Uses a quadratic invasion fitness: s(y,x) = -(y-x*)(y-x)/σ_K²
    /// where x* is the optimal trait and σ_K = 1.0 (niche width).
    pub fn selection_gradient(&self, x: f64, x_opt: f64) -> f64 {
        x_opt - x
    }
    /// Resident equilibrium population density n*(x) = K(x).
    /// Carrying capacity K(x) = K₀ · exp(-(x-x_opt)² / (2·σ_K²)).
    pub fn equilibrium_density(&self, x: f64, x_opt: f64, k0: f64, sigma_k: f64) -> f64 {
        k0 * (-(x - x_opt).powi(2) / (2.0 * sigma_k.powi(2))).exp()
    }
    /// Run the canonical equation for `steps` Euler steps of size `dt`.
    pub fn simulate(&mut self, x_opt: f64, k0: f64, sigma_k: f64, dt: f64, steps: usize) {
        let mut x = self.trait_value;
        let mut t = self.trajectory.last().map(|(t, _)| *t).unwrap_or(0.0);
        for _ in 0..steps {
            let n_star = self.equilibrium_density(x, x_opt, k0, sigma_k);
            let d = self.selection_gradient(x, x_opt);
            let dx_dt = 0.5 * self.mutation_rate * self.mutation_variance * n_star * d;
            x += dt * dx_dt;
            t += dt;
            self.trajectory.push((t, x));
        }
        self.trait_value = x;
    }
    /// Check if a branching point has been reached: trait is near x_opt
    /// but selection is disruptive (second derivative of fitness < 0).
    /// Simplified check: returns true if |D(x)| < ε.
    pub fn near_singular_strategy(&self, x_opt: f64, eps: f64) -> bool {
        (self.trait_value - x_opt).abs() < eps
    }
}
/// Two-species competitive Lotka-Volterra model.
///
/// dN₁/dt = r₁·N₁·(1 − (N₁ + α₁₂·N₂)/K₁)
/// dN₂/dt = r₂·N₂·(1 − (α₂₁·N₁ + N₂)/K₂)
#[derive(Debug, Clone)]
pub struct CompetitiveLV2 {
    /// Intrinsic growth rate of species 1.
    pub r1: f64,
    /// Intrinsic growth rate of species 2.
    pub r2: f64,
    /// Carrying capacity of species 1.
    pub k1: f64,
    /// Carrying capacity of species 2.
    pub k2: f64,
    /// Competition coefficient: effect of species 2 on species 1.
    pub alpha12: f64,
    /// Competition coefficient: effect of species 1 on species 2.
    pub alpha21: f64,
}
impl CompetitiveLV2 {
    /// Create a new two-species competitive Lotka-Volterra system.
    #[allow(clippy::too_many_arguments)]
    pub fn new(r1: f64, r2: f64, k1: f64, k2: f64, alpha12: f64, alpha21: f64) -> Self {
        CompetitiveLV2 {
            r1,
            r2,
            k1,
            k2,
            alpha12,
            alpha21,
        }
    }
    /// Right-hand side: returns (dN1/dt, dN2/dt).
    pub fn rhs(&self, n1: f64, n2: f64) -> (f64, f64) {
        let dn1 = self.r1 * n1 * (1.0 - (n1 + self.alpha12 * n2) / self.k1);
        let dn2 = self.r2 * n2 * (1.0 - (self.alpha21 * n1 + n2) / self.k2);
        (dn1, dn2)
    }
    /// Determine outcome according to isocline analysis.
    ///
    /// Returns:
    ///   0 = species 1 wins, 1 = species 2 wins,
    ///   2 = stable coexistence, 3 = unstable equilibrium (priority effects).
    pub fn outcome(&self) -> u8 {
        let sp1_wins_cond1 = self.k1 > self.k2 / self.alpha21;
        let sp1_wins_cond2 = self.k2 < self.k1 / self.alpha12;
        let sp2_wins_cond1 = self.k1 < self.k2 / self.alpha21;
        let sp2_wins_cond2 = self.k2 > self.k1 / self.alpha12;
        if sp1_wins_cond1 && sp1_wins_cond2 {
            0
        } else if sp2_wins_cond1 && sp2_wins_cond2 {
            1
        } else if !sp1_wins_cond1 && !sp2_wins_cond1 {
            2
        } else {
            3
        }
    }
    /// Simulate using RK4, returning (t, N1, N2) triples.
    pub fn simulate_rk4(
        &self,
        n1_0: f64,
        n2_0: f64,
        dt: f64,
        steps: usize,
    ) -> Vec<(f64, f64, f64)> {
        let mut traj = Vec::with_capacity(steps + 1);
        let mut n1 = n1_0;
        let mut n2 = n2_0;
        let mut t = 0.0_f64;
        traj.push((t, n1, n2));
        for _ in 0..steps {
            let (k1a, k1b) = self.rhs(n1, n2);
            let (k2a, k2b) = self.rhs(n1 + 0.5 * dt * k1a, n2 + 0.5 * dt * k1b);
            let (k3a, k3b) = self.rhs(n1 + 0.5 * dt * k2a, n2 + 0.5 * dt * k2b);
            let (k4a, k4b) = self.rhs(n1 + dt * k3a, n2 + dt * k3b);
            n1 += dt / 6.0 * (k1a + 2.0 * k2a + 2.0 * k3a + k4a);
            n2 += dt / 6.0 * (k1b + 2.0 * k2b + 2.0 * k3b + k4b);
            t += dt;
            traj.push((t, n1, n2));
        }
        traj
    }
}
/// Levins (1969) metapopulation model.
///
/// dp/dt = c·p·(1−p) − e·p
#[derive(Debug, Clone)]
pub struct LevinsModel {
    /// Colonization rate.
    pub c: f64,
    /// Local extinction rate.
    pub e: f64,
}
impl LevinsModel {
    /// Create a Levins metapopulation model.
    pub fn new(c: f64, e: f64) -> Self {
        LevinsModel { c, e }
    }
    /// Equilibrium patch occupancy p* = max(0, 1 − e/c).
    pub fn equilibrium(&self) -> f64 {
        if self.c <= self.e {
            0.0
        } else {
            1.0 - self.e / self.c
        }
    }
    /// Returns true if the metapopulation persists (c > e).
    pub fn persists(&self) -> bool {
        self.c > self.e
    }
    /// dp/dt at patch occupancy p.
    pub fn dpdt(&self, p: f64) -> f64 {
        self.c * p * (1.0 - p) - self.e * p
    }
    /// Simulate patch occupancy over time using Euler method.
    pub fn simulate(&self, p0: f64, dt: f64, steps: usize) -> Vec<(f64, f64)> {
        let mut traj = Vec::with_capacity(steps + 1);
        let mut p = p0.clamp(0.0, 1.0);
        let mut t = 0.0_f64;
        traj.push((t, p));
        for _ in 0..steps {
            let dp = self.dpdt(p);
            p = (p + dt * dp).clamp(0.0, 1.0);
            t += dt;
            traj.push((t, p));
        }
        traj
    }
}
/// SIR (Susceptible-Infected-Recovered) epidemic model.
#[derive(Debug, Clone)]
pub struct SIRModel {
    /// Transmission rate β (contacts × probability of transmission per contact).
    pub beta: f64,
    /// Recovery rate γ.
    pub gamma: f64,
    /// Total population size N.
    pub population: f64,
}
impl SIRModel {
    /// Create an SIR model.
    pub fn new(beta: f64, gamma: f64, population: f64) -> Self {
        SIRModel {
            beta,
            gamma,
            population,
        }
    }
    /// Basic reproduction number R₀ = β/γ.
    ///
    /// Since the RHS already normalizes by population (dS = -β·S·I/N, dI = β·S·I/N - γ·I),
    /// the transmission rate β is the per-capita effective rate, and R₀ = β/γ.
    pub fn r0(&self) -> f64 {
        self.beta / self.gamma
    }
    /// Herd immunity threshold: fraction that must be immune to prevent epidemic.
    /// p_c = 1 − 1/R₀.
    pub fn herd_immunity_threshold(&self) -> f64 {
        let r0 = self.r0();
        if r0 <= 1.0 {
            0.0
        } else {
            1.0 - 1.0 / r0
        }
    }
    /// Right-hand sides (dS, dI, dR) at state (S, I, R).
    pub fn rhs(&self, s: f64, i: f64, _r: f64) -> (f64, f64, f64) {
        let ds = -self.beta * s * i / self.population;
        let di = self.beta * s * i / self.population - self.gamma * i;
        let dr = self.gamma * i;
        (ds, di, dr)
    }
    /// Simulate SIR dynamics using RK4.
    ///
    /// Returns Vec of (t, S, I, R).
    pub fn simulate_rk4(
        &self,
        s0: f64,
        i0: f64,
        r0: f64,
        dt: f64,
        steps: usize,
    ) -> Vec<(f64, f64, f64, f64)> {
        let mut traj = Vec::with_capacity(steps + 1);
        let mut s = s0;
        let mut i = i0;
        let mut r = r0;
        let mut t = 0.0_f64;
        traj.push((t, s, i, r));
        for _ in 0..steps {
            let (k1s, k1i, k1r) = self.rhs(s, i, r);
            let (k2s, k2i, k2r) =
                self.rhs(s + 0.5 * dt * k1s, i + 0.5 * dt * k1i, r + 0.5 * dt * k1r);
            let (k3s, k3i, k3r) =
                self.rhs(s + 0.5 * dt * k2s, i + 0.5 * dt * k2i, r + 0.5 * dt * k2r);
            let (k4s, k4i, k4r) = self.rhs(s + dt * k3s, i + dt * k3i, r + dt * k3r);
            s += dt / 6.0 * (k1s + 2.0 * k2s + 2.0 * k3s + k4s);
            i += dt / 6.0 * (k1i + 2.0 * k2i + 2.0 * k3i + k4i);
            r += dt / 6.0 * (k1r + 2.0 * k2r + 2.0 * k3r + k4r);
            t += dt;
            traj.push((t, s, i, r));
        }
        traj
    }
    /// Final size relation: total fraction infected z satisfies z = 1 − exp(−R₀·z).
    ///
    /// Solved iteratively (fixed-point iteration).
    pub fn final_size(&self) -> f64 {
        let r0 = self.r0();
        if r0 <= 1.0 {
            return 0.0;
        }
        let mut z = 0.5_f64;
        for _ in 0..1000 {
            let z_new = 1.0 - (-r0 * z).exp();
            if (z_new - z).abs() < 1e-12 {
                return z_new;
            }
            z = z_new;
        }
        z
    }
}
/// Resource competition model based on Tilman's R* theory.
///
/// Each consumer species has an R* (minimum resource level needed to persist).
#[derive(Debug, Clone)]
pub struct ResourceCompetition {
    /// Resource names.
    pub resources: Vec<String>,
    /// Consumer species names.
    pub consumers: Vec<String>,
    /// R* values: minimum resource requirement of each consumer for each resource.
    /// Indexed as R_star[consumer][resource].
    pub r_star: Vec<f64>,
}
impl ResourceCompetition {
    /// Create a resource competition model.
    pub fn new(resources: Vec<String>, consumers: Vec<String>, r_star: Vec<f64>) -> Self {
        ResourceCompetition {
            resources,
            consumers,
            r_star,
        }
    }
    /// Tilman's R* theory: the species with the lowest R* wins competition.
    /// Returns the index of the winning consumer (lowest R*).
    pub fn tilman_r_star_theory(&self) -> Option<usize> {
        if self.r_star.is_empty() {
            return None;
        }
        let mut min_idx = 0;
        let mut min_val = self.r_star[0];
        for (i, &r) in self.r_star.iter().enumerate() {
            if r < min_val {
                min_val = r;
                min_idx = i;
            }
        }
        Some(min_idx)
    }
    /// Competitive exclusion: only the species with the lowest R* survives.
    /// Returns true if exactly one species dominates.
    pub fn competitive_exclusion(&self) -> bool {
        if self.r_star.len() <= 1 {
            return true;
        }
        let min = self.r_star.iter().cloned().fold(f64::INFINITY, f64::min);
        let count = self
            .r_star
            .iter()
            .filter(|&&r| (r - min).abs() < 1e-10)
            .count();
        count == 1
    }
}
/// Symmetric evolutionary game with payoff matrix.
#[derive(Debug, Clone)]
pub struct EvolutionaryGame {
    /// Payoff matrix A: A[i][j] = payoff to strategy i when playing against strategy j.
    pub payoff_matrix: Vec<Vec<f64>>,
    /// Strategy names.
    pub strategies: Vec<String>,
}
impl EvolutionaryGame {
    /// Create a new evolutionary game.
    pub fn new(payoff_matrix: Vec<Vec<f64>>, strategies: Vec<String>) -> Self {
        EvolutionaryGame {
            payoff_matrix,
            strategies,
        }
    }
    /// Fitness of strategy i given population state x (frequency vector).
    pub fn fitness(&self, strategy: usize, x: &[f64]) -> f64 {
        self.payoff_matrix[strategy]
            .iter()
            .zip(x.iter())
            .map(|(a, xi)| a * xi)
            .sum()
    }
    /// Mean fitness of the population given state x.
    pub fn mean_fitness(&self, x: &[f64]) -> f64 {
        x.iter()
            .enumerate()
            .map(|(i, xi)| xi * self.fitness(i, x))
            .sum()
    }
    /// One step of the replicator equation (Euler step).
    ///
    /// dxᵢ/dt = xᵢ · (fᵢ(x) − f̄(x))
    pub fn replicator_step(&self, x: &[f64], dt: f64) -> Vec<f64> {
        let f_bar = self.mean_fitness(x);
        let n = x.len();
        let mut x_new = vec![0.0; n];
        for i in 0..n {
            let fi = self.fitness(i, x);
            x_new[i] = x[i] + dt * x[i] * (fi - f_bar);
        }
        let total: f64 = x_new.iter().sum();
        if total > 0.0 {
            x_new.iter_mut().for_each(|xi| *xi /= total);
        }
        x_new
    }
    /// Simulate replicator dynamics for `steps` steps.
    pub fn simulate_replicator(&self, x0: Vec<f64>, dt: f64, steps: usize) -> Vec<Vec<f64>> {
        let mut history = Vec::with_capacity(steps + 1);
        let mut x = x0;
        history.push(x.clone());
        for _ in 0..steps {
            x = self.replicator_step(&x, dt);
            history.push(x.clone());
        }
        history
    }
    /// Check if strategy `s` is an ESS.
    ///
    /// σ* = strategy s is ESS if for all j ≠ s:
    ///   A[s][s] > A[j][s], or (A[s][s] = A[j][s] and A[s][j] > A[j][j]).
    pub fn is_ess(&self, s: usize) -> bool {
        let n = self.payoff_matrix.len();
        for j in 0..n {
            if j == s {
                continue;
            }
            let ass = self.payoff_matrix[s][s];
            let ajs = self.payoff_matrix[j][s];
            let asj = self.payoff_matrix[s][j];
            let ajj = self.payoff_matrix[j][j];
            if ass < ajs {
                return false;
            }
            if (ass - ajs).abs() < 1e-12 && asj <= ajj {
                return false;
            }
        }
        true
    }
}
/// Turing pattern analysis for the two-component activator-inhibitor system.
///
/// Linearization at (u*, v*) gives Jacobian:
///   J = [[a, b], [c, d]]
/// with diffusion coefficients D_u, D_v.
///
/// Turing instability occurs when diffusion destabilizes the uniform steady state.
#[derive(Debug, Clone)]
pub struct TuringAnalysis {
    /// Jacobian element (∂f/∂u at steady state).
    pub a: f64,
    /// Jacobian element (∂f/∂v at steady state).
    pub b: f64,
    /// Jacobian element (∂g/∂u at steady state).
    pub c: f64,
    /// Jacobian element (∂g/∂v at steady state).
    pub d: f64,
    /// Diffusion coefficient for activator u.
    pub d_u: f64,
    /// Diffusion coefficient for inhibitor v.
    pub d_v: f64,
}
impl TuringAnalysis {
    /// Create a new Turing analysis configuration.
    pub fn new(a: f64, b: f64, c: f64, d: f64, d_u: f64, d_v: f64) -> Self {
        TuringAnalysis {
            a,
            b,
            c,
            d,
            d_u,
            d_v,
        }
    }
    /// Check if the uniform steady state is stable without diffusion.
    /// Requires: trace(J) < 0 and det(J) > 0.
    pub fn is_stable_without_diffusion(&self) -> bool {
        let trace = self.a + self.d;
        let det = self.a * self.d - self.b * self.c;
        trace < 0.0 && det > 0.0
    }
    /// Dispersion relation: h(k²) = D_u · D_v · k⁴ − (a·D_v + d·D_u)·k² + det(J).
    /// Turing instability occurs for some k² > 0 where h(k²) < 0.
    pub fn dispersion_relation(&self, k_sq: f64) -> f64 {
        self.d_u * self.d_v * k_sq * k_sq - (self.a * self.d_v + self.d * self.d_u) * k_sq
            + (self.a * self.d - self.b * self.c)
    }
    /// Critical wavenumber k*² at which dispersion is minimum.
    /// k*² = (a·D_v + d·D_u) / (2·D_u·D_v).
    pub fn critical_wavenumber_sq(&self) -> f64 {
        (self.a * self.d_v + self.d * self.d_u) / (2.0 * self.d_u * self.d_v)
    }
    /// Check if Turing instability occurs (dispersion < 0 at k*²).
    pub fn has_turing_instability(&self) -> bool {
        if !self.is_stable_without_diffusion() {
            return false;
        }
        let k_sq = self.critical_wavenumber_sq();
        if k_sq <= 0.0 {
            return false;
        }
        self.dispersion_relation(k_sq) < 0.0
    }
    /// Minimum diffusion ratio d = D_v/D_u required for Turing instability.
    pub fn minimum_diffusion_ratio(&self) -> f64 {
        let det = self.a * self.d - self.b * self.c;
        if det <= 0.0 {
            return f64::NAN;
        }
        let lhs_val = 2.0 * det.sqrt();
        if self.a <= 0.0 {
            return f64::NAN;
        }
        let ratio = lhs_val / self.a;
        ratio * ratio
    }
}
/// Eco-evolutionary dynamics tracking phenotype distribution.
#[derive(Debug, Clone)]
pub struct EcoEvolutionaryDynamics {
    /// Vector of phenotype values in the population.
    pub phenotypes: Vec<f64>,
}
impl EcoEvolutionaryDynamics {
    /// Create an eco-evolutionary dynamics model.
    pub fn new(phenotypes: Vec<f64>) -> Self {
        EcoEvolutionaryDynamics { phenotypes }
    }
    /// Adaptive dynamics step: move each phenotype towards the fitness gradient.
    /// Uses simple gradient ascent on mean fitness.
    pub fn adaptive_dynamics(&mut self, fitness_fn: impl Fn(f64, &[f64]) -> f64, step: f64) {
        let pop = self.phenotypes.clone();
        for x in &mut self.phenotypes {
            let f0 = fitness_fn(*x, &pop);
            let f1 = fitness_fn(*x + 1e-6, &pop);
            let grad = (f1 - f0) / 1e-6;
            *x += step * grad;
        }
    }
    /// Evolutionarily stable strategy (ESS): phenotype at which fitness gradient = 0.
    /// Returns average phenotype as approximation.
    pub fn ess(&self) -> f64 {
        if self.phenotypes.is_empty() {
            return 0.0;
        }
        self.phenotypes.iter().sum::<f64>() / self.phenotypes.len() as f64
    }
    /// Returns true if the current mean phenotype is convergence-stable.
    /// Simplified check: variance < threshold.
    pub fn convergence_stable(&self) -> bool {
        if self.phenotypes.len() < 2 {
            return true;
        }
        let mean = self.ess();
        let var = self
            .phenotypes
            .iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<f64>()
            / self.phenotypes.len() as f64;
        var < 0.1
    }
}
/// Classical Lotka-Volterra predator-prey model.
///
/// dN/dt = α·N − β·N·P  (prey)
/// dP/dt = δ·N·P − γ·P  (predator)
#[derive(Debug, Clone)]
pub struct LotkaVolterra {
    /// Prey birth rate.
    pub alpha: f64,
    /// Predation rate.
    pub beta: f64,
    /// Predator death rate.
    pub gamma: f64,
    /// Predator conversion efficiency.
    pub delta: f64,
}
impl LotkaVolterra {
    /// Create a Lotka-Volterra model.
    pub fn new(alpha: f64, beta: f64, gamma: f64, delta: f64) -> Self {
        LotkaVolterra {
            alpha,
            beta,
            gamma,
            delta,
        }
    }
    /// Predator-prey dynamics: returns (dN/dt, dP/dt) at (n, p).
    pub fn predator_prey_dynamics(&self, n: f64, p: f64) -> (f64, f64) {
        let dn = self.alpha * n - self.beta * n * p;
        let dp = self.delta * n * p - self.gamma * p;
        (dn, dp)
    }
    /// Prey isocline: N* = γ/δ, Predator isocline: P* = α/β.
    pub fn isoclines(&self) -> (f64, f64) {
        let n_star = self.gamma / self.delta;
        let p_star = self.alpha / self.beta;
        (n_star, p_star)
    }
    /// Approximate oscillation period using linearisation at equilibrium.
    /// T ≈ 2π / sqrt(α·γ).
    pub fn oscillation_period(&self) -> f64 {
        2.0 * std::f64::consts::PI / (self.alpha * self.gamma).sqrt()
    }
}
/// Levins-style metapopulation model with rescue effect.
///
/// dp/dt = c·p·(1 − p) − e·(1 − p)·p  (with rescue effect)
/// dp/dt = c·p·(1 − p) − e·p           (classic Levins)
#[derive(Debug, Clone)]
pub struct MetapopulationModel {
    /// Number of habitat patches.
    pub patches: usize,
    /// Colonization rate per occupied patch.
    pub colonization: f64,
    /// Local extinction rate per occupied patch.
    pub extinction: f64,
}
impl MetapopulationModel {
    /// Create a metapopulation model.
    pub fn new(patches: usize, colonization: f64, extinction: f64) -> Self {
        MetapopulationModel {
            patches,
            colonization,
            extinction,
        }
    }
    /// Simulate patch occupancy using Levins model (Euler integration).
    /// Returns trajectory as Vec<(t, p)>.
    pub fn levins_model(&self, p0: f64, dt: f64, steps: usize) -> Vec<(f64, f64)> {
        let mut traj = Vec::with_capacity(steps + 1);
        let mut p = p0.clamp(0.0, 1.0);
        let mut t = 0.0_f64;
        traj.push((t, p));
        for _ in 0..steps {
            let dp = self.colonization * p * (1.0 - p) - self.extinction * p;
            p = (p + dt * dp).clamp(0.0, 1.0);
            t += dt;
            traj.push((t, p));
        }
        traj
    }
    /// Equilibrium patch occupancy: p* = max(0, 1 − e/c).
    pub fn equilibrium_occupancy(&self) -> f64 {
        if self.colonization <= self.extinction {
            0.0
        } else {
            1.0 - self.extinction / self.colonization
        }
    }
    /// Rescue effect: reduction in extinction rate due to immigration.
    /// Returns the effective extinction rate e_eff = e·(1 − p*).
    pub fn rescue_effect(&self) -> f64 {
        let p_eq = self.equilibrium_occupancy();
        self.extinction * (1.0 - p_eq)
    }
}
/// Food web model with energy transfer efficiencies.
#[derive(Debug, Clone)]
pub struct FoodWeb {
    /// Species names.
    pub species: Vec<String>,
    /// Energy transfers: (from_species, to_species, efficiency).
    pub energy_transfers: Vec<(usize, usize, f64)>,
}
impl FoodWeb {
    /// Create a food web.
    pub fn new(species: Vec<String>, energy_transfers: Vec<(usize, usize, f64)>) -> Self {
        FoodWeb {
            species,
            energy_transfers,
        }
    }
    /// Connectance = L / (S*(S-1)) where L = number of links, S = number of species.
    pub fn connectance(&self) -> f64 {
        let s = self.species.len();
        if s <= 1 {
            return 0.0;
        }
        self.energy_transfers.len() as f64 / (s * (s - 1)) as f64
    }
    /// Approximate trophic level of each species (1 = primary producer).
    /// Uses shortest path from primary producers (in-degree = 0).
    pub fn trophic_level(&self) -> Vec<f64> {
        let s = self.species.len();
        let mut levels = vec![1.0_f64; s];
        for _ in 0..s {
            let old = levels.clone();
            for &(from, to, _eff) in &self.energy_transfers {
                levels[to] = levels[to].max(old[from] + 1.0);
            }
        }
        levels
    }
    /// Identify keystone species: those whose removal causes disproportionate impact.
    /// Simple heuristic: species with high out-degree relative to in-degree.
    pub fn keystone_species(&self) -> Vec<usize> {
        let s = self.species.len();
        let mut out_degree = vec![0usize; s];
        let mut in_degree = vec![0usize; s];
        for &(from, to, _) in &self.energy_transfers {
            out_degree[from] += 1;
            in_degree[to] += 1;
        }
        let mut keystones = Vec::new();
        for i in 0..s {
            if out_degree[i] > 0 && in_degree[i] == 0 {
                keystones.push(i);
            } else if out_degree[i] as f64 > 2.0 * (in_degree[i] as f64 + 1.0) {
                keystones.push(i);
            }
        }
        keystones
    }
}
/// Leslie matrix population model for age-structured populations.
///
/// The Leslie matrix L has:
///   - Row 0: fecundity values fₓ for age class x
///   - Subdiagonal: survival probabilities Pₓ for age class x
///
/// Population at time t+1: N(t+1) = L · N(t)
#[allow(dead_code)]
pub struct LeslieMatrixModel {
    /// Number of age classes.
    pub age_classes: usize,
    /// Fecundities for each age class (row 0 of Leslie matrix).
    pub fecundity: Vec<f64>,
    /// Survival probabilities for each age class (subdiagonal).
    pub survival: Vec<f64>,
}
impl LeslieMatrixModel {
    /// Create a Leslie matrix model.
    pub fn new(fecundity: Vec<f64>, survival: Vec<f64>) -> Self {
        let age_classes = fecundity.len();
        LeslieMatrixModel {
            age_classes,
            fecundity,
            survival,
        }
    }
    /// Project population vector one time step: N(t+1) = L · N(t).
    pub fn project(&self, n: &[f64]) -> Vec<f64> {
        let k = self.age_classes;
        let mut n_next = vec![0.0; k];
        n_next[0] = self
            .fecundity
            .iter()
            .zip(n.iter())
            .map(|(f, ni)| f * ni)
            .sum();
        for i in 1..k {
            if i - 1 < self.survival.len() && i - 1 < n.len() {
                n_next[i] = self.survival[i - 1] * n[i - 1];
            }
        }
        n_next
    }
    /// Project population for `steps` time steps. Returns trajectory.
    pub fn simulate(&self, n0: Vec<f64>, steps: usize) -> Vec<Vec<f64>> {
        let mut traj = Vec::with_capacity(steps + 1);
        let mut n = n0;
        traj.push(n.clone());
        for _ in 0..steps {
            n = self.project(&n);
            traj.push(n.clone());
        }
        traj
    }
    /// Compute total population size at each time step.
    pub fn total_population(&self, n0: Vec<f64>, steps: usize) -> Vec<f64> {
        self.simulate(n0, steps)
            .into_iter()
            .map(|n| n.iter().sum())
            .collect()
    }
    /// Net reproductive rate R₀ = Σₓ lₓ mₓ (product of cumulative survival × fecundity).
    pub fn net_reproductive_rate(&self) -> f64 {
        let mut r0 = 0.0;
        let mut lx = 1.0;
        for x in 0..self.age_classes {
            r0 += lx * self.fecundity[x];
            if x < self.survival.len() {
                lx *= self.survival[x];
            }
        }
        r0
    }
}
/// Parameters for the classical Lotka-Volterra predator-prey system.
///
/// dN/dt = α·N − β·N·P
/// dP/dt = δ·N·P − γ·P
#[derive(Debug, Clone)]
pub struct LotkaVolterraParams {
    /// Prey birth rate.
    pub alpha: f64,
    /// Predation rate coefficient.
    pub beta: f64,
    /// Predator death rate.
    pub gamma: f64,
    /// Predator reproduction rate per prey consumed.
    pub delta: f64,
}
impl LotkaVolterraParams {
    /// Create a new Lotka-Volterra parameter set.
    pub fn new(alpha: f64, beta: f64, gamma: f64, delta: f64) -> Self {
        LotkaVolterraParams {
            alpha,
            beta,
            gamma,
            delta,
        }
    }
    /// Coexistence equilibrium: (N*, P*) = (γ/δ, α/β).
    pub fn equilibrium(&self) -> (f64, f64) {
        let n_star = self.gamma / self.delta;
        let p_star = self.alpha / self.beta;
        (n_star, p_star)
    }
    /// Evaluate the right-hand sides dN/dt and dP/dt at (N, P).
    pub fn rhs(&self, n: f64, p: f64) -> (f64, f64) {
        let dn = self.alpha * n - self.beta * n * p;
        let dp = self.delta * n * p - self.gamma * p;
        (dn, dp)
    }
    /// First integral (conserved quantity) H(N, P).
    /// H = δ·N − γ·ln(N) + β·P − α·ln(P)
    pub fn first_integral(&self, n: f64, p: f64) -> f64 {
        self.delta * n - self.gamma * n.ln() + self.beta * p - self.alpha * p.ln()
    }
    /// Simulate trajectory using 4th-order Runge-Kutta.
    ///
    /// Returns a vector of (t, N, P) triples.
    pub fn simulate_rk4(&self, n0: f64, p0: f64, dt: f64, steps: usize) -> Vec<(f64, f64, f64)> {
        let mut trajectory = Vec::with_capacity(steps + 1);
        let mut n = n0;
        let mut p = p0;
        let mut t = 0.0_f64;
        trajectory.push((t, n, p));
        for _ in 0..steps {
            let (k1n, k1p) = self.rhs(n, p);
            let (k2n, k2p) = self.rhs(n + 0.5 * dt * k1n, p + 0.5 * dt * k1p);
            let (k3n, k3p) = self.rhs(n + 0.5 * dt * k2n, p + 0.5 * dt * k2p);
            let (k4n, k4p) = self.rhs(n + dt * k3n, p + dt * k3p);
            n += dt / 6.0 * (k1n + 2.0 * k2n + 2.0 * k3n + k4n);
            p += dt / 6.0 * (k1p + 2.0 * k2p + 2.0 * k3p + k4p);
            t += dt;
            trajectory.push((t, n, p));
        }
        trajectory
    }
}
/// SEIR epidemic model with exposed (latent) compartment.
#[derive(Debug, Clone)]
pub struct SEIRModel {
    /// Transmission rate β.
    pub beta: f64,
    /// Progression rate from exposed to infectious σ (= 1 / incubation period).
    pub sigma: f64,
    /// Recovery rate γ.
    pub gamma: f64,
    /// Total population size.
    pub population: f64,
}
impl SEIRModel {
    /// Create a SEIR model.
    pub fn new(beta: f64, sigma: f64, gamma: f64, population: f64) -> Self {
        SEIRModel {
            beta,
            sigma,
            gamma,
            population,
        }
    }
    /// Basic reproduction number R₀ = β / γ (same as SIR for initial population).
    pub fn r0(&self) -> f64 {
        self.beta * self.population / self.gamma
    }
    /// Right-hand sides (dS, dE, dI, dR).
    pub fn rhs(&self, s: f64, e: f64, i: f64, _r: f64) -> (f64, f64, f64, f64) {
        let ds = -self.beta * s * i / self.population;
        let de = self.beta * s * i / self.population - self.sigma * e;
        let di = self.sigma * e - self.gamma * i;
        let dr = self.gamma * i;
        (ds, de, di, dr)
    }
    /// Simulate SEIR dynamics using RK4.
    ///
    /// Returns Vec of (t, S, E, I, R).
    pub fn simulate_rk4(
        &self,
        s0: f64,
        e0: f64,
        i0: f64,
        r0: f64,
        dt: f64,
        steps: usize,
    ) -> Vec<(f64, f64, f64, f64, f64)> {
        let mut traj = Vec::with_capacity(steps + 1);
        let (mut s, mut e, mut i, mut r) = (s0, e0, i0, r0);
        let mut t = 0.0_f64;
        traj.push((t, s, e, i, r));
        for _ in 0..steps {
            let (k1s, k1e, k1i, k1r) = self.rhs(s, e, i, r);
            let (k2s, k2e, k2i, k2r) = self.rhs(
                s + 0.5 * dt * k1s,
                e + 0.5 * dt * k1e,
                i + 0.5 * dt * k1i,
                r + 0.5 * dt * k1r,
            );
            let (k3s, k3e, k3i, k3r) = self.rhs(
                s + 0.5 * dt * k2s,
                e + 0.5 * dt * k2e,
                i + 0.5 * dt * k2i,
                r + 0.5 * dt * k2r,
            );
            let (k4s, k4e, k4i, k4r) =
                self.rhs(s + dt * k3s, e + dt * k3e, i + dt * k3i, r + dt * k3r);
            s += dt / 6.0 * (k1s + 2.0 * k2s + 2.0 * k3s + k4s);
            e += dt / 6.0 * (k1e + 2.0 * k2e + 2.0 * k3e + k4e);
            i += dt / 6.0 * (k1i + 2.0 * k2i + 2.0 * k3i + k4i);
            r += dt / 6.0 * (k1r + 2.0 * k2r + 2.0 * k3r + k4r);
            t += dt;
            traj.push((t, s, e, i, r));
        }
        traj
    }
}
/// Spatial ecology model with a 2D grid and dispersal.
#[derive(Debug, Clone)]
pub struct SpatialEcology {
    /// Grid dimensions (rows, cols).
    pub grid: (usize, usize),
    /// Dispersal rate between adjacent cells.
    pub dispersal_rate: f64,
}
impl SpatialEcology {
    /// Create a spatial ecology model.
    pub fn new(grid: (usize, usize), dispersal_rate: f64) -> Self {
        SpatialEcology {
            grid,
            dispersal_rate,
        }
    }
    /// Check for Turing instability given activator-inhibitor parameters.
    /// Condition: d_v/d_u > 1 (inhibitor must diffuse faster than activator).
    /// Uses the `dispersal_rate` as d_v/d_u ratio for simplicity.
    pub fn turing_instability(&self) -> bool {
        self.dispersal_rate > 1.0
    }
    /// Simulate pattern formation on grid (simplified: returns occupancy map).
    /// Returns a flat vector of length rows*cols with 0.0 or 1.0.
    pub fn pattern_formation(&self, seed: u64) -> Vec<f64> {
        let (rows, cols) = self.grid;
        let mut grid = vec![0.0_f64; rows * cols];
        let mut rng_state = seed;
        for val in grid.iter_mut() {
            rng_state = rng_state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            let r = (rng_state >> 33) as f64 / u32::MAX as f64;
            if self.dispersal_rate > 1.5 {
                *val = if r > 0.5 { 1.0 } else { 0.0 };
            } else {
                *val = r;
            }
        }
        grid
    }
}
/// Hubbell's neutral theory of biodiversity simulator.
///
/// Simulates local community dynamics under ecological drift:
/// births and deaths are random, with immigration from the metacommunity
/// at rate m (per death event).
#[allow(dead_code)]
pub struct NeutralBiodiversityModel {
    /// Local community size J.
    pub local_community_size: usize,
    /// Fundamental biodiversity number θ = 2·J_M·ν.
    pub theta: f64,
    /// Dispersal fraction / immigration rate m.
    pub immigration_rate: f64,
}
impl NeutralBiodiversityModel {
    /// Create a new neutral biodiversity model.
    pub fn new(local_community_size: usize, theta: f64, immigration_rate: f64) -> Self {
        NeutralBiodiversityModel {
            local_community_size,
            theta,
            immigration_rate,
        }
    }
    /// Expected local species richness under the unified neutral theory.
    /// Approximation: E[S] ≈ θ · ln(1 + J / θ).
    pub fn expected_richness(&self) -> f64 {
        let j = self.local_community_size as f64;
        self.theta * (1.0 + j / self.theta).ln()
    }
    /// Predicted relative abundance of the rank-r species (log-series approximation).
    /// P(n) = θ/n · x^n / (-ln(1-x)) where x = J/(J+θ).
    pub fn relative_abundance(&self, n: usize) -> f64 {
        if n == 0 {
            return 0.0;
        }
        let j = self.local_community_size as f64;
        let x = j / (j + self.theta);
        let log_series_norm = -(1.0 - x).ln();
        (self.theta / n as f64) * x.powi(n as i32) / log_series_norm
    }
    /// Alpha diversity: expected number of species in a sample of size k.
    /// Uses Chao1 estimator approximation: S_est = S_obs + n₁²/(2·n₂).
    pub fn alpha_diversity(&self, k: usize) -> f64 {
        let j = self.local_community_size as f64;
        let frac = (k as f64 / j).min(1.0);
        self.expected_richness() * frac.sqrt()
    }
    /// Beta diversity: multiplicative turnover between two communities.
    /// β = γ / α where γ = regional richness, α = local richness.
    pub fn beta_diversity(&self, regional_richness: f64) -> f64 {
        let alpha = self.expected_richness();
        if alpha == 0.0 {
            0.0
        } else {
            regional_richness / alpha
        }
    }
}
/// Extended SEIR (Susceptible-Exposed-Infected-Recovered) epidemic model
/// with waning immunity and demographic turnover.
#[allow(dead_code)]
pub struct SEIRExtended {
    /// Transmission rate β.
    pub beta: f64,
    /// Incubation rate σ (1/σ = mean incubation period).
    pub sigma: f64,
    /// Recovery rate γ.
    pub gamma: f64,
    /// Waning immunity rate ξ (recovered → susceptible).
    pub xi: f64,
    /// Birth/death rate μ (demographic turnover).
    pub mu: f64,
    /// Total population N.
    pub population: f64,
}
impl SEIRExtended {
    /// Create a new extended SEIR model.
    #[allow(clippy::too_many_arguments)]
    pub fn new(beta: f64, sigma: f64, gamma: f64, xi: f64, mu: f64, population: f64) -> Self {
        SEIRExtended {
            beta,
            sigma,
            gamma,
            xi,
            mu,
            population,
        }
    }
    /// Basic reproduction number R₀ = β·σ / ((σ + μ)·(γ + μ)).
    pub fn r0(&self) -> f64 {
        self.beta * self.sigma / ((self.sigma + self.mu) * (self.gamma + self.mu))
    }
    /// Epidemic threshold: disease invades iff R₀ > 1.
    pub fn epidemic_threshold(&self) -> bool {
        self.r0() > 1.0
    }
    /// Right-hand sides (dS, dE, dI, dR) at state (S, E, I, R).
    pub fn rhs(&self, s: f64, e: f64, i: f64, r: f64) -> (f64, f64, f64, f64) {
        let n = self.population;
        let ds = self.mu * n - self.beta * s * i / n - self.mu * s + self.xi * r;
        let de = self.beta * s * i / n - (self.sigma + self.mu) * e;
        let di = self.sigma * e - (self.gamma + self.mu) * i;
        let dr = self.gamma * i - (self.xi + self.mu) * r;
        (ds, de, di, dr)
    }
    /// Simulate SEIR dynamics using RK4. Returns Vec of (t, S, E, I, R).
    pub fn simulate_rk4(
        &self,
        s0: f64,
        e0: f64,
        i0: f64,
        r0: f64,
        dt: f64,
        steps: usize,
    ) -> Vec<(f64, f64, f64, f64, f64)> {
        let mut traj = Vec::with_capacity(steps + 1);
        let mut s = s0;
        let mut e = e0;
        let mut i = i0;
        let mut r = r0;
        let mut t = 0.0_f64;
        traj.push((t, s, e, i, r));
        for _ in 0..steps {
            let (k1s, k1e, k1i, k1r) = self.rhs(s, e, i, r);
            let (k2s, k2e, k2i, k2r) = self.rhs(
                s + 0.5 * dt * k1s,
                e + 0.5 * dt * k1e,
                i + 0.5 * dt * k1i,
                r + 0.5 * dt * k1r,
            );
            let (k3s, k3e, k3i, k3r) = self.rhs(
                s + 0.5 * dt * k2s,
                e + 0.5 * dt * k2e,
                i + 0.5 * dt * k2i,
                r + 0.5 * dt * k2r,
            );
            let (k4s, k4e, k4i, k4r) =
                self.rhs(s + dt * k3s, e + dt * k3e, i + dt * k3i, r + dt * k3r);
            s += dt / 6.0 * (k1s + 2.0 * k2s + 2.0 * k3s + k4s);
            e += dt / 6.0 * (k1e + 2.0 * k2e + 2.0 * k3e + k4e);
            i += dt / 6.0 * (k1i + 2.0 * k2i + 2.0 * k3i + k4i);
            r += dt / 6.0 * (k1r + 2.0 * k2r + 2.0 * k3r + k4r);
            t += dt;
            traj.push((t, s, e, i, r));
        }
        traj
    }
    /// Endemic equilibrium: (S*, E*, I*, R*) when R₀ > 1.
    /// Returns None if R₀ ≤ 1 (disease dies out).
    pub fn endemic_equilibrium(&self) -> Option<(f64, f64, f64, f64)> {
        let r0 = self.r0();
        if r0 <= 1.0 {
            return None;
        }
        let n = self.population;
        let i_star = self.mu * n * (1.0 - 1.0 / r0) / (self.gamma + self.mu);
        let e_star = (self.gamma + self.mu) * i_star / self.sigma;
        let s_star = n / r0;
        let r_star = n - s_star - e_star - i_star;
        Some((s_star, e_star, i_star, r_star))
    }
}
