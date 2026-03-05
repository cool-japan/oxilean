//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::functions::*;
use super::functions::{BOLTZMANN_K, PLANCK_H};

/// Discrete renormalization group iteration for 1D models.
///
/// Iterates the recursion relation g_{n+1} = R(g_n) for a coupling constant g.
/// Can be used to find fixed points and analyze RG flows.
#[allow(dead_code)]
pub struct RenormalizationGroup {
    /// Initial coupling constant g₀
    pub initial_coupling: f64,
    /// Scale factor b at each RG step (usually b = 2)
    pub scale_factor: f64,
}
impl RenormalizationGroup {
    /// Create an RG solver with initial coupling and scale factor.
    pub fn new(initial_coupling: f64, scale_factor: f64) -> Self {
        Self {
            initial_coupling,
            scale_factor,
        }
    }
    /// Iterate the RG recursion R(g) for n steps.
    ///
    /// Returns the trajectory [g₀, g₁, ..., gₙ].
    pub fn iterate<F>(&self, rg_map: F, n_steps: usize) -> Vec<f64>
    where
        F: Fn(f64) -> f64,
    {
        let mut trajectory = vec![self.initial_coupling];
        let mut g = self.initial_coupling;
        for _ in 0..n_steps {
            g = rg_map(g);
            trajectory.push(g);
        }
        trajectory
    }
    /// Find a fixed point using Newton's method: g* = R(g*) ⟺ R(g) - g = 0.
    ///
    /// Returns (g_star, converged).
    pub fn find_fixed_point<F>(&self, rg_map: &F, tol: f64, max_iter: usize) -> (f64, bool)
    where
        F: Fn(f64) -> f64,
    {
        let mut g = self.initial_coupling;
        for _ in 0..max_iter {
            let g_new = rg_map(g);
            let residual = g_new - g;
            if residual.abs() < tol {
                return (g_new, true);
            }
            g = g + 0.5 * residual;
        }
        let converged = (rg_map(g) - g).abs() < tol;
        (g, converged)
    }
    /// Classify fixed point as stable (attractive) or unstable (repulsive).
    ///
    /// Computes |dR/dg| at g*. Returns Some(true) if stable, Some(false) if unstable.
    pub fn is_stable_fixed_point<F>(&self, rg_map: &F, g_star: f64) -> Option<bool>
    where
        F: Fn(f64) -> f64,
    {
        let dg = g_star.abs() * 1e-6 + 1e-10;
        if dg == 0.0 {
            return None;
        }
        let deriv = (rg_map(g_star + dg) - rg_map(g_star - dg)) / (2.0 * dg);
        Some(deriv.abs() < 1.0)
    }
    /// Compute the RG eigenvalue (scaling exponent) at a fixed point.
    pub fn scaling_exponent<F>(&self, rg_map: &F, g_star: f64) -> f64
    where
        F: Fn(f64) -> f64,
    {
        let dg = g_star.abs() * 1e-6 + 1e-10;
        let deriv = (rg_map(g_star + dg) - rg_map(g_star - dg)) / (2.0 * dg);
        if self.scale_factor > 1.0 && deriv.abs() > 0.0 {
            deriv.abs().ln() / self.scale_factor.ln()
        } else {
            0.0
        }
    }
}
/// Grand canonical ensemble simulation.
///
/// Models a system with variable particle number coupled to a reservoir
/// at temperature T and chemical potential μ.
#[allow(dead_code)]
pub struct GrandCanonicalEnsemble {
    /// Single-particle energy levels ε_k
    pub energy_levels: Vec<f64>,
    /// Temperature T [K]
    pub temperature: f64,
    /// Chemical potential μ [J]
    pub chemical_potential: f64,
    /// Statistics: true = Fermi-Dirac, false = Bose-Einstein
    pub is_fermionic: bool,
}
impl GrandCanonicalEnsemble {
    /// Create a grand canonical ensemble.
    pub fn new(energy_levels: Vec<f64>, temperature: f64, mu: f64, fermionic: bool) -> Self {
        Self {
            energy_levels,
            temperature,
            chemical_potential: mu,
            is_fermionic: fermionic,
        }
    }
    /// Inverse temperature β = 1/(k_B T)
    pub fn beta(&self) -> f64 {
        1.0 / (BOLTZMANN_K * self.temperature)
    }
    /// Mean occupation number for level k:
    /// n_k = 1/(exp(β(ε_k - μ)) ∓ 1)  where − is FD, + is BE
    pub fn mean_occupation(&self, k: usize) -> f64 {
        let x = self.beta() * (self.energy_levels[k] - self.chemical_potential);
        if self.is_fermionic {
            1.0 / (x.exp() + 1.0)
        } else {
            if x <= 0.0 {
                f64::INFINITY
            } else {
                1.0 / (x.exp() - 1.0)
            }
        }
    }
    /// Grand potential: Ω = ∓k_BT Σ_k ln(1 ± exp(β(μ - ε_k)))
    pub fn grand_potential(&self) -> f64 {
        let b = self.beta();
        let sum: f64 = self
            .energy_levels
            .iter()
            .map(|&eps| {
                let x = b * (self.chemical_potential - eps);
                if self.is_fermionic {
                    (1.0 + x.exp()).ln()
                } else if x < -700.0 {
                    0.0
                } else {
                    -(1.0 - x.exp()).abs().ln()
                }
            })
            .sum();
        -BOLTZMANN_K * self.temperature * sum
    }
    /// Mean total particle number: ⟨N⟩ = Σ_k ⟨n_k⟩
    pub fn mean_particle_number(&self) -> f64 {
        (0..self.energy_levels.len())
            .filter_map(|k| {
                let n = self.mean_occupation(k);
                if n.is_finite() {
                    Some(n)
                } else {
                    None
                }
            })
            .sum()
    }
    /// Mean total energy: ⟨E⟩ = Σ_k ε_k ⟨n_k⟩
    pub fn mean_energy(&self) -> f64 {
        self.energy_levels
            .iter()
            .enumerate()
            .filter_map(|(k, &eps)| {
                let n = self.mean_occupation(k);
                if n.is_finite() {
                    Some(eps * n)
                } else {
                    None
                }
            })
            .sum()
    }
}
/// Computes equal-time connected correlation functions from a discrete distribution.
#[allow(dead_code)]
pub struct CorrelationFunction {
    /// Sample values of the order parameter
    pub samples: Vec<f64>,
}
impl CorrelationFunction {
    /// Create from a list of samples.
    pub fn new(samples: Vec<f64>) -> Self {
        Self { samples }
    }
    /// Sample mean: ⟨φ⟩ = (1/N) Σ φ_i
    pub fn mean(&self) -> f64 {
        if self.samples.is_empty() {
            return 0.0;
        }
        self.samples.iter().sum::<f64>() / self.samples.len() as f64
    }
    /// Variance: ⟨φ²⟩ - ⟨φ⟩²
    pub fn variance(&self) -> f64 {
        if self.samples.is_empty() {
            return 0.0;
        }
        let m = self.mean();
        let m2 = self.samples.iter().map(|&x| x * x).sum::<f64>() / self.samples.len() as f64;
        m2 - m * m
    }
    /// Connected two-point correlator at lag τ (discrete): C(τ) = ⟨φ(0)φ(τ)⟩ - ⟨φ⟩²
    pub fn connected_correlator(&self, lag: usize) -> f64 {
        let n = self.samples.len();
        if lag >= n {
            return 0.0;
        }
        let m = self.mean();
        let count = (n - lag) as f64;
        let corr: f64 = (0..n - lag)
            .map(|i| self.samples[i] * self.samples[i + lag])
            .sum::<f64>()
            / count;
        corr - m * m
    }
    /// Autocorrelation time τ_int = ½ + Σ_{τ=1}^{∞} C(τ)/C(0)
    pub fn integrated_autocorrelation_time(&self, max_lag: usize) -> f64 {
        let c0 = self.connected_correlator(0);
        if c0.abs() < 1e-300 {
            return 0.5;
        }
        let sum: f64 = (1..max_lag.min(self.samples.len()))
            .map(|tau| self.connected_correlator(tau) / c0)
            .sum();
        0.5 + sum
    }
    /// Susceptibility proportional to N * variance (for periodic systems)
    pub fn susceptibility(&self) -> f64 {
        self.variance() * self.samples.len() as f64
    }
}
/// A statistical ensemble in the canonical ensemble
pub struct Ensemble {
    pub energies: Vec<f64>,
    pub temperature: f64,
    pub degeneracies: Vec<u32>,
}
impl Ensemble {
    /// Create an ensemble with unit degeneracies
    pub fn new(energies: Vec<f64>, temperature: f64) -> Self {
        let n = energies.len();
        Self {
            energies,
            temperature,
            degeneracies: vec![1; n],
        }
    }
    /// Create an ensemble with explicit degeneracies
    pub fn with_degeneracies(energies: Vec<f64>, degeneracies: Vec<u32>, temperature: f64) -> Self {
        Self {
            energies,
            temperature,
            degeneracies,
        }
    }
    /// Inverse temperature β = 1/(k_B T)
    pub fn beta(&self) -> f64 {
        1.0 / (BOLTZMANN_K * self.temperature)
    }
    /// Partition function Z = Σ g_i * exp(-β E_i)
    pub fn partition_function(&self) -> f64 {
        let beta = self.beta();
        self.energies
            .iter()
            .zip(self.degeneracies.iter())
            .map(|(&e, &g)| (g as f64) * (-beta * e).exp())
            .sum()
    }
    /// Boltzmann factor for a given energy: exp(-β E)
    pub fn boltzmann_factor(&self, energy: f64) -> f64 {
        (-self.beta() * energy).exp()
    }
    /// Probability of state at index i: g_i exp(-β E_i) / Z
    pub fn probability(&self, state_idx: usize) -> f64 {
        let z = self.partition_function();
        if z == 0.0 {
            return 0.0;
        }
        let e = self.energies[state_idx];
        let g = self.degeneracies[state_idx] as f64;
        g * self.boltzmann_factor(e) / z
    }
    /// Mean energy ⟨E⟩ = Σ E_i p_i
    pub fn mean_energy(&self) -> f64 {
        self.energies
            .iter()
            .enumerate()
            .map(|(i, &e)| e * self.probability(i))
            .sum()
    }
    /// Gibbs entropy S = -k_B Σ p_i log p_i
    pub fn entropy(&self) -> f64 {
        let s: f64 = self
            .energies
            .iter()
            .enumerate()
            .filter_map(|(i, _)| {
                let p = self.probability(i);
                if p > 1e-300 {
                    Some(-p * p.ln())
                } else {
                    None
                }
            })
            .sum();
        BOLTZMANN_K * s
    }
    /// Helmholtz free energy F = -k_B T log Z
    pub fn free_energy(&self) -> f64 {
        let z = self.partition_function();
        if z <= 0.0 {
            return f64::INFINITY;
        }
        -BOLTZMANN_K * self.temperature * z.ln()
    }
    /// Numerical heat capacity C_v = d⟨E⟩/dT using finite differences
    pub fn heat_capacity(&self) -> f64 {
        let dt = self.temperature * 1e-4;
        let e_high = {
            let e_high = Ensemble::with_degeneracies(
                self.energies.clone(),
                self.degeneracies.clone(),
                self.temperature + dt,
            );
            e_high.mean_energy()
        };
        let e_low = {
            let e_low = Ensemble::with_degeneracies(
                self.energies.clone(),
                self.degeneracies.clone(),
                self.temperature - dt,
            );
            e_low.mean_energy()
        };
        (e_high - e_low) / (2.0 * dt)
    }
    /// Index of the state with the highest probability (ground state at low T)
    pub fn max_prob_state(&self) -> usize {
        self.energies
            .iter()
            .enumerate()
            .map(|(i, _)| (i, self.probability(i)))
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i)
            .unwrap_or(0)
    }
}
/// Mean-field Ising model self-consistent solver.
///
/// Self-consistency equation: m = tanh(β (h + z J m))
/// where z is the coordination number (e.g., z=4 for 2D square lattice).
pub struct MeanFieldIsing {
    /// Exchange coupling J
    pub j_coupling: f64,
    /// External field h
    pub h_field: f64,
    /// Coordination number z
    pub coordination: f64,
    /// Temperature T (Kelvin)
    pub temperature: f64,
}
impl MeanFieldIsing {
    pub fn new(j: f64, h: f64, z: f64, temp: f64) -> Self {
        Self {
            j_coupling: j,
            h_field: h,
            coordination: z,
            temperature: temp,
        }
    }
    /// Mean field critical temperature T_c = z J / k_B
    pub fn critical_temperature(&self) -> f64 {
        self.coordination * self.j_coupling / BOLTZMANN_K
    }
    /// Reduced temperature t = (T - T_c) / T_c
    pub fn reduced_temperature(&self) -> f64 {
        let tc = self.critical_temperature();
        (self.temperature - tc) / tc
    }
    /// Self-consistency equation residual: F(m) = m - tanh(β(h + zJm))
    fn residual(&self, m: f64) -> f64 {
        let b = 1.0 / (BOLTZMANN_K * self.temperature);
        let arg = b * (self.h_field + self.coordination * self.j_coupling * m);
        m - arg.tanh()
    }
    /// Solve for the self-consistent magnetization using simple iteration.
    ///
    /// Returns (m, converged) — converged is true if |F(m)| < tol.
    pub fn solve(&self, initial_m: f64, max_iter: usize, tol: f64) -> (f64, bool) {
        let mut m = initial_m;
        for _ in 0..max_iter {
            let b = 1.0 / (BOLTZMANN_K * self.temperature);
            let arg = b * (self.h_field + self.coordination * self.j_coupling * m);
            let m_new = arg.tanh();
            if (m_new - m).abs() < tol {
                return (m_new, true);
            }
            m = 0.8 * m + 0.2 * m_new;
        }
        let converged = self.residual(m).abs() < tol;
        (m, converged)
    }
    /// Find all stable solutions (handles symmetry breaking below T_c)
    ///
    /// Returns a Vec of converged magnetization values.
    pub fn find_all_solutions(&self) -> Vec<f64> {
        let seeds = [-0.999, -0.5, 0.0, 0.5, 0.999];
        let mut solutions: Vec<f64> = Vec::new();
        for &seed in &seeds {
            let (m, converged) = self.solve(seed, 2000, 1e-10);
            if converged {
                let is_new = solutions
                    .iter()
                    .all(|&existing| (existing - m).abs() > 1e-6);
                if is_new {
                    solutions.push(m);
                }
            }
        }
        solutions.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        solutions
    }
    /// Mean field free energy density: f(m) = -zJm²/2 - hm + k_BT * entropy_term
    pub fn free_energy_density(&self, m: f64) -> f64 {
        let b = 1.0 / (BOLTZMANN_K * self.temperature);
        let interaction = -0.5 * self.coordination * self.j_coupling * m * m;
        let field_term = -self.h_field * m;
        let h_eff = self.h_field + self.coordination * self.j_coupling * m;
        let entropy_term = if (b * h_eff).abs() < 700.0 {
            -(b * h_eff).cosh().ln() / b
        } else {
            -(b * h_eff).abs() / b
        };
        interaction + field_term + entropy_term
    }
}
/// Record of critical exponents for a universality class
#[derive(Debug, Clone)]
pub struct CriticalExponents {
    /// Name of the universality class
    pub name: &'static str,
    /// Spatial dimension
    pub dimension: u8,
    /// Heat capacity exponent: C ~ |t|^{-α}
    pub alpha: f64,
    /// Order parameter exponent: m ~ |t|^β
    pub beta: f64,
    /// Susceptibility exponent: χ ~ |t|^{-γ}
    pub gamma: f64,
    /// Critical isotherm exponent: h ~ m^δ
    pub delta: f64,
    /// Correlation length exponent: ξ ~ |t|^{-ν}
    pub nu: f64,
    /// Anomalous dimension
    pub eta: f64,
}
impl CriticalExponents {
    /// Check Widom scaling relation: γ = β(δ - 1)
    pub fn check_widom(&self) -> bool {
        let lhs = self.gamma;
        let rhs = self.beta * (self.delta - 1.0);
        (lhs - rhs).abs() < 0.01
    }
    /// Check Rushbrooke scaling relation: α + 2β + γ = 2
    pub fn check_rushbrooke(&self) -> bool {
        let sum = self.alpha + 2.0 * self.beta + self.gamma;
        (sum - 2.0).abs() < 0.01
    }
    /// Check Fisher scaling relation: γ = ν(2 - η)
    pub fn check_fisher(&self) -> bool {
        let lhs = self.gamma;
        let rhs = self.nu * (2.0 - self.eta);
        (lhs - rhs).abs() < 0.05
    }
}
/// Landau free energy functional for second-order phase transitions.
///
/// f(m, t) = a t m² + b m⁴ + c m⁶ + h m
/// where t = (T - T_c) / T_c is the reduced temperature.
pub struct LandauFreeEnergy {
    /// Coefficient of m² term (typically > 0)
    pub a: f64,
    /// Coefficient of m⁴ term (must be > 0 for stability)
    pub b: f64,
    /// Coefficient of m⁶ term (optional higher-order stabilization)
    pub c: f64,
    /// External field coupling
    pub h: f64,
}
impl LandauFreeEnergy {
    /// Standard second-order transition: f = a t m² + b m⁴ + h m
    pub fn new_second_order(a: f64, b: f64, h: f64) -> Self {
        Self { a, b, c: 0.0, h }
    }
    /// Tricritical point model: f = a t m² + c m⁶ + h m  (b = 0)
    pub fn new_tricritical(a: f64, c: f64, h: f64) -> Self {
        Self { a, b: 0.0, c, h }
    }
    /// Free energy density at order parameter m and reduced temperature t
    pub fn eval(&self, m: f64, t: f64) -> f64 {
        self.a * t * m * m + self.b * m * m * m * m + self.c * m.powf(6.0) + self.h * m
    }
    /// Derivative ∂f/∂m
    pub fn deriv(&self, m: f64, t: f64) -> f64 {
        2.0 * self.a * t * m + 4.0 * self.b * m * m * m + 6.0 * self.c * m.powf(5.0) + self.h
    }
    /// Second derivative ∂²f/∂m²
    pub fn deriv2(&self, m: f64, t: f64) -> f64 {
        2.0 * self.a * t + 12.0 * self.b * m * m + 30.0 * self.c * m.powf(4.0)
    }
    /// Minimize free energy using Newton's method (or bisection fallback).
    ///
    /// Returns (m_min, f_min, converged).
    pub fn minimize(&self, t: f64, initial_m: f64, tol: f64, max_iter: usize) -> (f64, f64, bool) {
        let mut m = initial_m;
        for _ in 0..max_iter {
            let f1 = self.deriv(m, t);
            let f2 = self.deriv2(m, t);
            if f2.abs() < 1e-300 {
                break;
            }
            let step = -f1 / f2;
            let step = step.clamp(-0.5, 0.5);
            m += step;
            if step.abs() < tol {
                return (m, self.eval(m, t), true);
            }
        }
        (m, self.eval(m, t), self.deriv(m, t).abs() < tol * 100.0)
    }
    /// Equilibrium order parameter at reduced temperature t (h=0 assumed).
    ///
    /// Below T_c (t < 0): m ≈ sqrt(-at / 2b)  for b > 0 case.
    pub fn equilibrium_order_parameter(&self, t: f64) -> f64 {
        if t >= 0.0 || self.b <= 0.0 {
            let (m, _, _) = self.minimize(t, 0.01, 1e-10, 1000);
            return m;
        }
        let m_mf = (-self.a * t / (2.0 * self.b)).sqrt();
        let (m_pos, f_pos, _) = self.minimize(t, m_mf, 1e-10, 1000);
        let (m_neg, f_neg, _) = self.minimize(t, -m_mf, 1e-10, 1000);
        if f_pos <= f_neg {
            m_pos
        } else {
            m_neg
        }
    }
    /// Spontaneous magnetization as a function of reduced temperature t in [-1, 0]
    pub fn spontaneous_magnetization_curve(&self, n_points: usize) -> Vec<(f64, f64)> {
        (0..n_points)
            .map(|i| {
                let t = -1.0 + (i as f64) / (n_points as f64);
                let m = self.equilibrium_order_parameter(t);
                (t, m.abs())
            })
            .collect()
    }
}
/// Canonical ensemble simulation for a system with discrete energy levels.
///
/// Wraps `Ensemble` with additional derived quantities.
pub struct CanonicalEnsemble {
    inner: Ensemble,
}
impl CanonicalEnsemble {
    /// Create from energy levels and temperature (in Kelvin)
    pub fn new(energies: Vec<f64>, temperature: f64) -> Self {
        Self {
            inner: Ensemble::new(energies, temperature),
        }
    }
    /// Create with explicit degeneracies
    pub fn with_degeneracies(energies: Vec<f64>, degeneracies: Vec<u32>, temperature: f64) -> Self {
        Self {
            inner: Ensemble::with_degeneracies(energies, degeneracies, temperature),
        }
    }
    /// Partition function Z(β)
    pub fn partition_function(&self) -> f64 {
        self.inner.partition_function()
    }
    /// Helmholtz free energy F = -k_B T ln Z
    pub fn free_energy(&self) -> f64 {
        self.inner.free_energy()
    }
    /// Mean energy ⟨E⟩
    pub fn mean_energy(&self) -> f64 {
        self.inner.mean_energy()
    }
    /// Entropy S = (⟨E⟩ - F) / T
    pub fn entropy(&self) -> f64 {
        (self.inner.mean_energy() - self.inner.free_energy()) / self.inner.temperature
    }
    /// Heat capacity C_v = d⟨E⟩/dT
    pub fn heat_capacity(&self) -> f64 {
        self.inner.heat_capacity()
    }
    /// Variance of energy: Var(E) = ⟨E²⟩ - ⟨E⟩²
    pub fn energy_variance(&self) -> f64 {
        let mean_e = self.inner.mean_energy();
        let mean_e2: f64 = self
            .inner
            .energies
            .iter()
            .enumerate()
            .map(|(i, &e)| e * e * self.inner.probability(i))
            .sum();
        mean_e2 - mean_e * mean_e
    }
    /// Probability distribution over energy levels
    pub fn probabilities(&self) -> Vec<f64> {
        (0..self.inner.energies.len())
            .map(|i| self.inner.probability(i))
            .collect()
    }
    /// Relative entropy (KL divergence) from uniform distribution: D(p||q)
    pub fn kl_from_uniform(&self) -> f64 {
        let n = self.inner.energies.len() as f64;
        if n == 0.0 {
            return 0.0;
        }
        (0..self.inner.energies.len())
            .filter_map(|i| {
                let p = self.inner.probability(i);
                if p > 1e-300 {
                    Some(p * (p * n).ln())
                } else {
                    None
                }
            })
            .sum()
    }
}
/// Van der Waals gas: (P + a n²/V²)(V - nb) = nRT.
///
/// Uses SI units with k_B (per-particle form):
///   (P + a/v²)(v - b) = k_B T   where v = V/N is volume per particle.
#[allow(dead_code)]
pub struct VanDerWaalsGas {
    /// Attractive interaction parameter a [J·m³]
    pub a: f64,
    /// Excluded volume parameter b [m³]
    pub b: f64,
    /// Temperature T [K]
    pub temperature: f64,
}
impl VanDerWaalsGas {
    /// Create a van der Waals gas.
    pub fn new(a: f64, b: f64, temperature: f64) -> Self {
        Self { a, b, temperature }
    }
    /// Pressure at specific volume v = V/N per particle.
    ///
    /// P = k_B T / (v - b) - a / v²
    pub fn pressure(&self, v: f64) -> f64 {
        if v <= self.b {
            return f64::INFINITY;
        }
        BOLTZMANN_K * self.temperature / (v - self.b) - self.a / (v * v)
    }
    /// Critical temperature: T_c = 8a / (27 k_B b)
    pub fn critical_temperature(&self) -> f64 {
        8.0 * self.a / (27.0 * BOLTZMANN_K * self.b)
    }
    /// Critical pressure: P_c = a / (27 b²)
    pub fn critical_pressure(&self) -> f64 {
        self.a / (27.0 * self.b * self.b)
    }
    /// Critical volume per particle: v_c = 3b
    pub fn critical_volume(&self) -> f64 {
        3.0 * self.b
    }
    /// Reduced temperature: T_r = T / T_c
    pub fn reduced_temperature(&self) -> f64 {
        self.temperature / self.critical_temperature()
    }
    /// Check if the system is above the critical temperature
    pub fn is_supercritical(&self) -> bool {
        self.temperature >= self.critical_temperature()
    }
    /// Compressibility factor at the critical point: Z_c = P_c v_c / (k_B T_c) = 3/8
    pub fn critical_compressibility() -> f64 {
        3.0 / 8.0
    }
}
/// Equation of state using the virial expansion up to third virial coefficient.
///
/// P/(ρk_BT) = 1 + B₂(T) ρ + B₃(T) ρ² + ...
#[allow(dead_code)]
pub struct VirialGas {
    /// Second virial coefficient B₂(T) [m³]
    pub b2: f64,
    /// Third virial coefficient B₃(T) [m⁶]
    pub b3: f64,
    /// Temperature T [K]
    pub temperature: f64,
}
impl VirialGas {
    /// Create a virial gas model with explicit B₂ and B₃.
    pub fn new(b2: f64, b3: f64, temperature: f64) -> Self {
        Self {
            b2,
            b3,
            temperature,
        }
    }
    /// Pressure from virial expansion: P = ρ k_B T (1 + B₂ρ + B₃ρ²)
    pub fn pressure(&self, density: f64) -> f64 {
        BOLTZMANN_K
            * self.temperature
            * density
            * (1.0 + self.b2 * density + self.b3 * density * density)
    }
    /// Compressibility factor Z = Pv/(k_BT) = 1 + B₂/v + B₃/v² (v = 1/ρ)
    pub fn compressibility_factor(&self, density: f64) -> f64 {
        1.0 + self.b2 * density + self.b3 * density * density
    }
    /// Hard-sphere second virial coefficient: B₂ = (2/3)π σ³
    pub fn hard_sphere_b2(sigma: f64) -> f64 {
        (2.0 / 3.0) * std::f64::consts::PI * sigma * sigma * sigma
    }
    /// Boyle temperature: temperature where B₂(T) = 0
    /// For Lennard-Jones gas this is approximately T_B ≈ 3.4 ε/k_B
    pub fn is_above_boyle_temperature(&self) -> bool {
        self.b2 >= 0.0
    }
}
/// 2D Ising model on an n×n grid with periodic boundary conditions
pub struct IsingModel {
    pub spins: Vec<Vec<i8>>,
    pub j_coupling: f64,
    pub temperature: f64,
}
impl IsingModel {
    /// Create an n×n Ising model with a deterministic initial spin pattern
    pub fn new(n: usize, j: f64, temp: f64) -> Self {
        let mut rng_state: u64 = 12345;
        let spins = (0..n)
            .map(|_| {
                (0..n)
                    .map(|_| {
                        let r = Self::lcg_next(&mut rng_state);
                        if r < 0.5 {
                            1i8
                        } else {
                            -1i8
                        }
                    })
                    .collect()
            })
            .collect();
        Self {
            spins,
            j_coupling: j,
            temperature: temp,
        }
    }
    /// Total energy E = -J Σ s_i s_j (nearest neighbors, periodic BC)
    pub fn energy(&self) -> f64 {
        let n = self.spins.len();
        let mut e = 0.0;
        for i in 0..n {
            for j in 0..n {
                let s = self.spins[i][j] as f64;
                let s_right = self.spins[i][(j + 1) % n] as f64;
                let s_down = self.spins[(i + 1) % n][j] as f64;
                e -= self.j_coupling * s * (s_right + s_down);
            }
        }
        e
    }
    /// Magnetization per site |Σ s_i| / N²
    pub fn magnetization(&self) -> f64 {
        let n = self.spins.len();
        let total: i64 = self
            .spins
            .iter()
            .flat_map(|row| row.iter())
            .map(|&s| s as i64)
            .sum();
        (total.abs() as f64) / ((n * n) as f64)
    }
    /// One Metropolis-Hastings step: propose flipping a random spin
    pub fn metropolis_step(&mut self, rng: &mut u64) {
        let n = self.spins.len();
        let r1 = Self::lcg_next(rng);
        let r2 = Self::lcg_next(rng);
        let r3 = Self::lcg_next(rng);
        let i = (r1 * n as f64) as usize % n;
        let j = (r2 * n as f64) as usize % n;
        let s = self.spins[i][j] as f64;
        let neighbors: f64 = [
            self.spins[(i + n - 1) % n][j] as f64,
            self.spins[(i + 1) % n][j] as f64,
            self.spins[i][(j + n - 1) % n] as f64,
            self.spins[i][(j + 1) % n] as f64,
        ]
        .iter()
        .sum();
        let delta_e = 2.0 * self.j_coupling * s * neighbors;
        let accept = if delta_e <= 0.0 {
            true
        } else {
            let prob = (-delta_e / (BOLTZMANN_K * self.temperature)).exp();
            r3 < prob
        };
        if accept {
            self.spins[i][j] = -self.spins[i][j];
        }
    }
    /// Linear congruential generator returning a float in [0, 1)
    fn lcg_next(state: &mut u64) -> f64 {
        *state = state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (*state >> 33) as f64 / (u32::MAX as f64 + 1.0)
    }
}
/// Table of critical exponents for common universality classes
pub struct CriticalExponentTable {
    pub entries: Vec<CriticalExponents>,
}
impl CriticalExponentTable {
    /// Build the standard table of universality classes
    pub fn standard() -> Self {
        Self {
            entries: vec![
                CriticalExponents {
                    name: "2D Ising",
                    dimension: 2,
                    alpha: 0.0,
                    beta: 0.125,
                    gamma: 1.75,
                    delta: 15.0,
                    nu: 1.0,
                    eta: 0.25,
                },
                CriticalExponents {
                    name: "3D Ising",
                    dimension: 3,
                    alpha: 0.110,
                    beta: 0.326,
                    gamma: 1.237,
                    delta: 4.789,
                    nu: 0.630,
                    eta: 0.036,
                },
                CriticalExponents {
                    name: "3D XY",
                    dimension: 3,
                    alpha: -0.013,
                    beta: 0.346,
                    gamma: 1.316,
                    delta: 4.780,
                    nu: 0.671,
                    eta: 0.038,
                },
                CriticalExponents {
                    name: "3D Heisenberg",
                    dimension: 3,
                    alpha: -0.122,
                    beta: 0.365,
                    gamma: 1.386,
                    delta: 4.803,
                    nu: 0.707,
                    eta: 0.033,
                },
                CriticalExponents {
                    name: "Mean Field",
                    dimension: 4,
                    alpha: 0.0,
                    beta: 0.5,
                    gamma: 1.0,
                    delta: 3.0,
                    nu: 0.5,
                    eta: 0.0,
                },
                CriticalExponents {
                    name: "2D Potts q=3",
                    dimension: 2,
                    alpha: 0.333,
                    beta: 0.111,
                    gamma: 1.444,
                    delta: 14.0,
                    nu: 0.833,
                    eta: 0.148,
                },
            ],
        }
    }
    /// Find a universality class by name
    pub fn find(&self, name: &str) -> Option<&CriticalExponents> {
        self.entries.iter().find(|e| e.name == name)
    }
    /// Validate all scaling relations in the table
    pub fn validate_scaling_relations(&self) -> Vec<(&'static str, bool, bool, bool)> {
        self.entries
            .iter()
            .map(|e| {
                (
                    e.name,
                    e.check_widom(),
                    e.check_rushbrooke(),
                    e.check_fisher(),
                )
            })
            .collect()
    }
}
/// Classical ideal gas in a box
pub struct IdealGas {
    pub n_particles: u64,
    pub temperature: f64,
    pub volume: f64,
}
impl IdealGas {
    pub fn new(n: u64, t: f64, v: f64) -> Self {
        Self {
            n_particles: n,
            temperature: t,
            volume: v,
        }
    }
    /// Pressure P = N k_B T / V
    pub fn pressure(&self) -> f64 {
        (self.n_particles as f64) * BOLTZMANN_K * self.temperature / self.volume
    }
    /// Mean kinetic energy per particle ⟨E⟩ = (3/2) k_B T
    pub fn mean_kinetic_energy(&self) -> f64 {
        1.5 * BOLTZMANN_K * self.temperature
    }
    /// RMS speed sqrt(3 k_B T / m) for particles of mass m (kg)
    pub fn rms_speed(&self, mass: f64) -> f64 {
        (3.0 * BOLTZMANN_K * self.temperature / mass).sqrt()
    }
    /// Sackur-Tetrode entropy approximation (monatomic ideal gas)
    ///
    /// S/N k_B ≈ ln(V/N * (4π m ⟨E⟩ / (3 N h²))^(3/2)) + 5/2
    /// Uses m = proton mass (1.67e-27 kg) as default
    pub fn entropy(&self) -> f64 {
        let n = self.n_particles as f64;
        let m = 1.6726e-27_f64;
        let mean_e = self.mean_kinetic_energy();
        let lambda_arg = 4.0 * std::f64::consts::PI * m * mean_e / (3.0 * n * PLANCK_H * PLANCK_H);
        if lambda_arg <= 0.0 {
            return 0.0;
        }
        BOLTZMANN_K * n * ((self.volume / n) * lambda_arg.powf(1.5) + 2.5)
    }
}
/// 1D Ising model: exact solution via transfer matrix
///
/// Hamiltonian: H = -J Σ s_i s_{i+1} - h Σ s_i  (periodic BC)
pub struct IsingModel1D {
    /// Number of sites
    pub n_sites: usize,
    /// Exchange coupling J (in units of energy)
    pub j_coupling: f64,
    /// External magnetic field h (in units of energy)
    pub h_field: f64,
    /// Temperature T (in Kelvin; uses dimensionless β = 1/(k_B T))
    pub temperature: f64,
}
impl IsingModel1D {
    /// Create a 1D Ising model
    pub fn new(n: usize, j: f64, h: f64, temp: f64) -> Self {
        Self {
            n_sites: n,
            j_coupling: j,
            h_field: h,
            temperature: temp,
        }
    }
    /// Inverse temperature β = 1/(k_B T)
    pub fn beta(&self) -> f64 {
        1.0 / (BOLTZMANN_K * self.temperature)
    }
    /// Transfer matrix eigenvalues λ± for the 1D Ising model.
    ///
    /// The 2×2 transfer matrix T with elements T_{s,s'} = exp(βJ s s' + βh(s+s')/2)
    /// has eigenvalues:
    ///   λ± = exp(βJ) [cosh(βh) ± sqrt(sinh²(βh) + exp(-4βJ))]
    pub fn eigenvalues(&self) -> (f64, f64) {
        let b = self.beta();
        let bj = b * self.j_coupling;
        let bh = b * self.h_field;
        let exp_bj = bj.exp();
        let cosh_bh = bh.cosh();
        let sinh_bh = bh.sinh();
        let disc = sinh_bh * sinh_bh + (-4.0 * bj).exp();
        let sqrt_disc = disc.sqrt();
        let lambda_plus = exp_bj * (cosh_bh + sqrt_disc);
        let lambda_minus = exp_bj * (cosh_bh - sqrt_disc);
        (lambda_plus, lambda_minus)
    }
    /// Exact partition function Z = λ+^N + λ-^N  (periodic BC)
    pub fn partition_function(&self) -> f64 {
        let (lp, lm) = self.eigenvalues();
        let n = self.n_sites as f64;
        lp.powf(n) + lm.powf(n)
    }
    /// Free energy per site f = -k_B T / N * ln Z
    pub fn free_energy_per_site(&self) -> f64 {
        let z = self.partition_function();
        if z <= 0.0 {
            return f64::INFINITY;
        }
        -BOLTZMANN_K * self.temperature * z.ln() / (self.n_sites as f64)
    }
    /// Zero-field (h=0) partition function: Z = (2 cosh(βJ))^N
    pub fn zero_field_partition_function(&self) -> f64 {
        let b = self.beta();
        let bj = b * self.j_coupling;
        (2.0 * bj.cosh()).powf(self.n_sites as f64)
    }
    /// Magnetization per site ⟨m⟩ = (1/N β) ∂ln Z/∂h  (numerical)
    pub fn magnetization_per_site(&self) -> f64 {
        let dh = self.j_coupling.abs() * 1e-6 + 1e-30;
        let z_plus = IsingModel1D::new(
            self.n_sites,
            self.j_coupling,
            self.h_field + dh,
            self.temperature,
        )
        .partition_function();
        let z_minus = IsingModel1D::new(
            self.n_sites,
            self.j_coupling,
            self.h_field - dh,
            self.temperature,
        )
        .partition_function();
        let z = self.partition_function();
        if z <= 0.0 {
            return 0.0;
        }
        (z_plus - z_minus) / (2.0 * dh * self.beta() * z)
    }
    /// Susceptibility per site χ = ∂⟨m⟩/∂h  (numerical second derivative)
    pub fn susceptibility_per_site(&self) -> f64 {
        let dh = self.j_coupling.abs() * 1e-4 + 1e-30;
        let m_plus = IsingModel1D::new(
            self.n_sites,
            self.j_coupling,
            self.h_field + dh,
            self.temperature,
        )
        .magnetization_per_site();
        let m_minus = IsingModel1D::new(
            self.n_sites,
            self.j_coupling,
            self.h_field - dh,
            self.temperature,
        )
        .magnetization_per_site();
        (m_plus - m_minus) / (2.0 * dh)
    }
}
