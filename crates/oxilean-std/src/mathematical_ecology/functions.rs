//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
#![allow(clippy::items_after_test_module)]

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    CompetitiveLV2, EvolutionaryGame, LeslieMatrix, LevinsModel, LotkaVolterraParams, SEIRModel,
    SIRModel, TuringAnalysis,
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
pub fn fn_ty(dom: Expr, cod: Expr) -> Expr {
    arrow(dom, cod)
}
pub fn vec_real() -> Expr {
    list_ty(real_ty())
}
/// `LotkaVolterraSystem : Real → Real → Real → Real → Prop`
///
/// Classical two-species predator-prey system with parameters (α, β, γ, δ):
///   dN/dt = αN − βNP
///   dP/dt = δNP − γP
pub fn lotka_volterra_ty() -> Expr {
    arrow(
        real_ty(),
        arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), prop()))),
    )
}
/// `PredatorPreyEquilibrium : Real → Real → Real → Real → Prop`
///
/// The coexistence equilibrium (N*, P*) = (γ/δ, α/β).
pub fn predator_prey_equilibrium_ty() -> Expr {
    arrow(
        real_ty(),
        arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), prop()))),
    )
}
/// `LotkaVolterraConservation : ∀ α β γ δ, LV(α,β,γ,δ) → ∃ H, dH/dt = 0`
///
/// Conservation of the Lotka-Volterra first integral
///   H(N,P) = δN − γ ln N + βP − α ln P.
pub fn lotka_volterra_conservation_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "alpha",
        real_ty(),
        pi(
            BinderInfo::Default,
            "beta",
            real_ty(),
            arrow(
                app3(
                    app(cst("LotkaVolterraSystem"), bvar(1)),
                    bvar(0),
                    real_ty(),
                    real_ty(),
                ),
                prop(),
            ),
        ),
    )
}
/// `CompetitiveLV : Nat → Vec Real → Vec (Vec Real) → Prop`
///
/// n-species competitive Lotka-Volterra: dNᵢ/dt = rᵢ Nᵢ (1 − Σⱼ aᵢⱼ Nⱼ / Kᵢ).
pub fn competitive_lv_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(vec_real(), arrow(list_ty(vec_real()), prop())),
    )
}
/// `CompetitiveExclusion : Prop`
///
/// Gause's competitive exclusion principle: at most n species can stably coexist
/// in n-dimensional niche space.
pub fn competitive_exclusion_ty() -> Expr {
    prop()
}
/// `NicheOverlap : Real → Real → Real`
///
/// Niche overlap coefficient between two species (0 = no overlap, 1 = identical niches).
pub fn niche_overlap_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), real_ty()))
}
/// `LevinsMetapopulation : Real → Real → Prop`
///
/// Levins (1969) patch occupancy model:
///   dp/dt = c·p·(1−p) − e·p
/// where p = fraction of occupied patches, c = colonization, e = extinction rate.
pub fn levins_metapopulation_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), prop()))
}
/// `MetapopulationEquilibrium : Real → Real → Real`
///
/// Equilibrium patch occupancy p* = 1 − e/c (provided c > e).
pub fn metapopulation_equilibrium_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), real_ty()))
}
/// `MetapopulationPersistence : Real → Real → Prop`
///
/// Metapopulation persists iff c > e (i.e., p* > 0).
pub fn metapopulation_persistence_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), prop()))
}
/// `ReactionDiffusionSystem : (Real → Real → Real) → (Real → Real → Real) → Real → Real → Prop`
///
/// Two-component reaction-diffusion:
///   ∂u/∂t = D_u Δu + f(u,v)
///   ∂v/∂t = D_v Δv + g(u,v)
pub fn reaction_diffusion_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), fn_ty(real_ty(), real_ty())),
        arrow(
            fn_ty(real_ty(), fn_ty(real_ty(), real_ty())),
            arrow(real_ty(), arrow(real_ty(), prop())),
        ),
    )
}
/// `TuringInstability : Real → Real → Prop`
///
/// Turing (1952) diffusion-driven instability: a uniform steady state that is
/// stable without diffusion becomes unstable with diffusion when d = D_v/D_u > d_critical.
pub fn turing_instability_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), prop()))
}
/// `TuringWavenumber : Real → Real → Real`
///
/// Critical wavenumber k* at which Turing instability first appears.
pub fn turing_wavenumber_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), real_ty()))
}
/// `SIRModel : Real → Real → Real → Prop`
///
/// Kermack-McKendrick SIR model with transmission rate β and recovery rate γ:
///   dS/dt = −β S I
///   dI/dt =  β S I − γ I
///   dR/dt =  γ I
pub fn sir_model_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), prop())))
}
/// `SEIRModel : Real → Real → Real → Real → Prop`
///
/// SEIR model adding exposed (latent) compartment with rate σ:
///   dE/dt = β S I − σ E
///   dI/dt = σ E − γ I
pub fn seir_model_ty() -> Expr {
    arrow(
        real_ty(),
        arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), prop()))),
    )
}
/// `SEIRSModel : Real → Real → Real → Real → Real → Prop`
///
/// SEIRS model with waning immunity at rate ξ (recovered → susceptible).
pub fn seirs_model_ty() -> Expr {
    arrow(
        real_ty(),
        arrow(
            real_ty(),
            arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), prop()))),
        ),
    )
}
/// `BasicReproductionNumber : Real → Real → Real`
///
/// R₀ = β S₀ / γ for the SIR model. Epidemic occurs iff R₀ > 1.
pub fn basic_reproduction_number_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), real_ty()))
}
/// `HerdImmunityThreshold : Real → Real`
///
/// Herd immunity threshold p_c = 1 − 1/R₀.
pub fn herd_immunity_threshold_ty() -> Expr {
    arrow(real_ty(), real_ty())
}
/// `LeslieMatrix : Nat → Prop`
///
/// Leslie (1945) age-structured matrix model: N(t+1) = L · N(t),
/// where L has fertilities in first row and survival probabilities on subdiagonal.
pub fn leslie_matrix_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `StableAgeDistribution : Nat → Vec Real`
///
/// Stable age distribution (right eigenvector corresponding to dominant eigenvalue λ₁).
pub fn stable_age_distribution_ty() -> Expr {
    arrow(nat_ty(), vec_real())
}
/// `NetReproductiveRate : Vec Real → Vec Real → Real`
///
/// Net reproductive rate R₀ = Σₓ lₓ mₓ (survivorship × fecundity).
pub fn net_reproductive_rate_ty() -> Expr {
    arrow(vec_real(), arrow(vec_real(), real_ty()))
}
/// `EulerLotkaEquation : Real → Vec Real → Vec Real → Prop`
///
/// Euler-Lotka characteristic equation: 1 = Σₓ e^{−rx} lₓ mₓ,
/// solved for r = intrinsic rate of natural increase.
pub fn euler_lotka_equation_ty() -> Expr {
    arrow(real_ty(), arrow(vec_real(), arrow(vec_real(), prop())))
}
/// `EvolutionaryGame : Nat → Prop`
///
/// An evolutionary game on n strategies with a payoff matrix.
pub fn evolutionary_game_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `EvolutionarilyStableStrategy : Vec Real → Prop`
///
/// ESS condition: σ* is an ESS if for all σ ≠ σ*,
///   W(σ*, σ*) > W(σ, σ*), or \[W(σ*, σ*) = W(σ, σ*) and W(σ*, σ) > W(σ, σ)\].
pub fn ess_ty() -> Expr {
    arrow(vec_real(), prop())
}
/// `ReplicatorDynamics : Nat → Vec Real → Vec Real`
///
/// Replicator equation: dxᵢ/dt = xᵢ \[fᵢ(x) − f̄(x)\] where f̄ = Σⱼ xⱼ fⱼ.
pub fn replicator_dynamics_ty() -> Expr {
    arrow(nat_ty(), arrow(vec_real(), vec_real()))
}
/// `InvasionFitness : Real → Real → Real`
///
/// s(y, x) = per-capita growth rate of rare mutant y in resident x community.
pub fn invasion_fitness_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), real_ty()))
}
/// `EvolutionarilySingularStrategy : Real → Prop`
///
/// x* is evolutionarily singular: ∂s/∂y|_{y=x*,x=x*} = 0.
pub fn evolutionarily_singular_strategy_ty() -> Expr {
    arrow(real_ty(), prop())
}
/// `ConvergenceStability : Real → Prop`
///
/// x* is convergence stable (CSS): selection gradient pushes residents towards x*.
pub fn convergence_stability_ty() -> Expr {
    arrow(real_ty(), prop())
}
/// `PairwiseInvasibilityPlot : (Real → Real → Real) → Prop`
///
/// PIP construction: map (y, x) ↦ sign(s(y,x)).
pub fn pairwise_invasibility_plot_ty() -> Expr {
    arrow(fn_ty(real_ty(), fn_ty(real_ty(), real_ty())), prop())
}
/// `PoincareBendixson : Prop`
///
/// Poincaré-Bendixson theorem: bounded orbits in 2D have periodic or equilibrium limit sets.
pub fn poincare_bendixson_ty() -> Expr {
    prop()
}
/// `PerronFrobenius : Nat → Prop`
///
/// Perron-Frobenius theorem: positive matrices have a unique dominant real eigenvalue.
pub fn perron_frobenius_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `ESSNashEquilibrium : Vec Real → Prop`
///
/// Every ESS is a Nash equilibrium (but not vice versa).
pub fn ess_nash_equilibrium_ty() -> Expr {
    arrow(vec_real(), prop())
}
/// `CompetitiveExclusionPrinciple : Nat → Prop`
///
/// In a system with n limiting resources, at most n species can coexist at equilibrium.
pub fn competitive_exclusion_principle_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// Build the mathematical ecology environment: register all axioms as opaque constants.
pub fn build_mathematical_ecology_env(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("LotkaVolterraSystem", lotka_volterra_ty()),
        ("PredatorPreyEquilibrium", predator_prey_equilibrium_ty()),
        ("LVConservation", lotka_volterra_conservation_ty()),
        ("CompetitiveLV", competitive_lv_ty()),
        ("CompetitiveExclusion", competitive_exclusion_ty()),
        ("NicheOverlap", niche_overlap_ty()),
        ("LevinsMetapopulation", levins_metapopulation_ty()),
        ("MetapopulationEquilibrium", metapopulation_equilibrium_ty()),
        ("MetapopulationPersistence", metapopulation_persistence_ty()),
        ("ReactionDiffusionSystem", reaction_diffusion_ty()),
        ("TuringInstability", turing_instability_ty()),
        ("TuringWavenumber", turing_wavenumber_ty()),
        ("SIRModel", sir_model_ty()),
        ("SEIRModel", seir_model_ty()),
        ("SEIRSModel", seirs_model_ty()),
        ("BasicReproductionNumber", basic_reproduction_number_ty()),
        ("HerdImmunityThreshold", herd_immunity_threshold_ty()),
        ("LeslieMatrix", leslie_matrix_ty()),
        ("StableAgeDistribution", stable_age_distribution_ty()),
        ("NetReproductiveRate", net_reproductive_rate_ty()),
        ("EulerLotkaEquation", euler_lotka_equation_ty()),
        ("EvolutionaryGame", evolutionary_game_ty()),
        ("ESS", ess_ty()),
        ("ReplicatorDynamics", replicator_dynamics_ty()),
        ("InvasionFitness", invasion_fitness_ty()),
        (
            "EvolSingularStrategy",
            evolutionarily_singular_strategy_ty(),
        ),
        ("ConvergenceStability", convergence_stability_ty()),
        ("PairwiseInvasibilityPlot", pairwise_invasibility_plot_ty()),
        ("PoincareBendixson", poincare_bendixson_ty()),
        ("PerronFrobenius", perron_frobenius_ty()),
        ("ESSNashEquilibrium", ess_nash_equilibrium_ty()),
        (
            "CompetitiveExclusionPrinciple",
            competitive_exclusion_principle_ty(),
        ),
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
/// Create the classical Hawk-Dove evolutionary game.
///
/// V = resource value, C = cost of injury.
/// Hawk-Hawk: (V-C)/2, Hawk-Dove: V, Dove-Hawk: 0, Dove-Dove: V/2.
pub fn hawk_dove_game(v: f64, c: f64) -> EvolutionaryGame {
    let payoff = vec![vec![(v - c) / 2.0, v], vec![0.0, v / 2.0]];
    EvolutionaryGame::new(payoff, vec!["Hawk".to_string(), "Dove".to_string()])
}
/// ESS frequency of Hawks in the Hawk-Dove game: p* = V/C (if V < C).
pub fn hawk_dove_ess_frequency(v: f64, c: f64) -> f64 {
    if v >= c {
        1.0
    } else {
        v / c
    }
}
/// Compute the Lotka-Volterra invasion fitness of a mutant (strategy y) in a
/// monomorphic resident population at strategy x, using a logistic competition model.
///
/// s(y, x) = r(y) · (1 − α(y,x) · N*(x) / K(y))
/// where N*(x) = K(x) (logistic equilibrium of resident),
/// K(z) = carrying capacity function,
/// α(y,x) = competition kernel.
pub fn invasion_fitness_logistic(
    y: f64,
    x: f64,
    r_fn: impl Fn(f64) -> f64,
    k_fn: impl Fn(f64) -> f64,
    alpha_fn: impl Fn(f64, f64) -> f64,
) -> f64 {
    let n_star_x = k_fn(x);
    let ky = k_fn(y);
    let ry = r_fn(y);
    let alpha_yx = alpha_fn(y, x);
    ry * (1.0 - alpha_yx * n_star_x / ky)
}
/// Gaussian competition kernel: α(y,x) = exp(−(y−x)²/(2σ²)).
pub fn gaussian_competition_kernel(y: f64, x: f64, sigma: f64) -> f64 {
    let diff = (y - x) / sigma;
    (-0.5 * diff * diff).exp()
}
/// Gaussian carrying capacity: K(z) = K_max · exp(−z²/(2σ_K²)).
pub fn gaussian_carrying_capacity(z: f64, k_max: f64, sigma_k: f64) -> f64 {
    let ratio = z / sigma_k;
    k_max * (-0.5 * ratio * ratio).exp()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lotka_volterra_equilibrium() {
        let lv = LotkaVolterraParams::new(1.0, 0.5, 0.5, 0.25);
        let (n_star, p_star) = lv.equilibrium();
        assert!((n_star - 2.0).abs() < 1e-10, "N* should be 2");
        assert!((p_star - 2.0).abs() < 1e-10, "P* should be 2");
    }
    #[test]
    fn test_lotka_volterra_conservation() {
        let lv = LotkaVolterraParams::new(1.0, 0.1, 0.4, 0.1);
        let (n_star, p_star) = lv.equilibrium();
        let n0 = n_star * 1.1;
        let p0 = p_star * 0.9;
        let h0 = lv.first_integral(n0, p0);
        let traj = lv.simulate_rk4(n0, p0, 0.01, 500);
        for &(_, n, p) in &traj {
            let h = lv.first_integral(n, p);
            assert!(
                (h - h0).abs() < 1e-4,
                "First integral not conserved: H={h}, H0={h0}"
            );
        }
    }
    #[test]
    fn test_sir_r0_and_herd_immunity() {
        let sir = SIRModel::new(0.3, 0.1, 1000.0);
        let r0 = sir.r0();
        assert!((r0 - 3.0).abs() < 1e-10, "R0 should be 3, got {r0}");
        let pit = sir.herd_immunity_threshold();
        assert!(
            (pit - 2.0 / 3.0).abs() < 1e-10,
            "HIT should be 2/3, got {pit}"
        );
    }
    #[test]
    fn test_sir_mass_conservation() {
        let sir = SIRModel::new(0.3, 0.1, 1000.0);
        let traj = sir.simulate_rk4(990.0, 10.0, 0.0, 0.1, 500);
        for &(_, s, i, r) in &traj {
            let total = s + i + r;
            assert!(
                (total - 1000.0).abs() < 0.01,
                "Population not conserved: {total}"
            );
        }
    }
    #[test]
    fn test_levins_metapopulation() {
        let lm = LevinsModel::new(0.3, 0.1);
        assert!(lm.persists(), "Metapopulation should persist when c > e");
        let peq = lm.equilibrium();
        assert!(
            (peq - (1.0 - 0.1 / 0.3)).abs() < 1e-10,
            "Equilibrium p* wrong: {peq}"
        );
        let lm2 = LevinsModel::new(0.05, 0.1);
        assert!(!lm2.persists(), "Should go extinct when e > c");
        assert!((lm2.equilibrium() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_leslie_matrix_net_reproductive_rate() {
        let leslie = LeslieMatrix::new(vec![0.0, 2.0, 1.0], vec![0.8, 0.5]);
        let r0 = leslie.net_reproductive_rate();
        assert!(
            (r0 - 2.0).abs() < 1e-10,
            "Net reproductive rate should be 2.0, got {r0}"
        );
    }
    #[test]
    fn test_hawk_dove_ess() {
        let hd = hawk_dove_game(2.0, 6.0);
        let p_ess = hawk_dove_ess_frequency(2.0, 6.0);
        assert!(
            (p_ess - 1.0 / 3.0).abs() < 1e-10,
            "Hawk-Dove ESS frequency should be 1/3"
        );
        assert!(!hd.is_ess(0), "Pure Hawk is not ESS when V < C");
        assert!(!hd.is_ess(1), "Pure Dove is not ESS when V > 0");
    }
    #[test]
    fn test_turing_instability() {
        // a=1, b=-2, c=2, d=-3: trace=-2<0, det=1>0 → stable without diffusion
        // With D_u=1, D_v=10: critical k²=0.35, dispersion(0.35)<0 → Turing instability
        let ta = TuringAnalysis::new(1.0, -2.0, 2.0, -3.0, 1.0, 10.0);
        assert!(
            ta.is_stable_without_diffusion(),
            "Should be stable without diffusion"
        );
        assert!(
            ta.has_turing_instability(),
            "Should have Turing instability"
        );
        // With D_v=1.5: critical k²<0 → no instability
        let ta_nodiff = TuringAnalysis::new(1.0, -2.0, 2.0, -3.0, 1.0, 1.5);
        assert!(
            !ta_nodiff.has_turing_instability(),
            "Should NOT have Turing instability"
        );
    }
    #[test]
    fn test_competitive_lv_outcome() {
        let clv = CompetitiveLV2::new(1.0, 1.0, 200.0, 100.0, 0.5, 0.8);
        assert_eq!(clv.outcome(), 0, "Species 1 should win");
        let clv2 = CompetitiveLV2::new(1.0, 1.0, 100.0, 200.0, 0.8, 0.5);
        assert_eq!(clv2.outcome(), 1, "Species 2 should win");
    }
}
/// `LotkaVolterraGlobalExistence : Real → Real → Real → Real → Prop`
/// Kolmogorov's theorem: the Lotka-Volterra ODE has a global positive solution.
pub fn me_ext_lv_global_existence_ty() -> Expr {
    arrow(
        real_ty(),
        arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), prop()))),
    )
}
/// `KolmogorovLVTheorem : Nat → Prop`
/// Kolmogorov's generalization: n-species LV systems have a positive invariant set.
pub fn me_ext_kolmogorov_lv_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `LVPeriodicOrbit : Real → Real → Real → Real → Prop`
/// Lotka-Volterra systems exhibit closed orbits (periodic solutions) around the equilibrium.
pub fn me_ext_lv_periodic_orbit_ty() -> Expr {
    arrow(
        real_ty(),
        arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), prop()))),
    )
}
/// `SpeciesAreaRelationship : Real → Real → Prop`
/// S = c · A^z where S = species richness, A = area, c, z = constants.
pub fn me_ext_species_area_relationship_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), prop()))
}
/// `SARExponent : Real → Prop`
/// The SAR exponent z ≈ 0.25 for islands (Preston's canonical distribution).
pub fn me_ext_sar_exponent_ty() -> Expr {
    arrow(real_ty(), prop())
}
/// `SpeciesRichnessLog : Real → Real → Real`
/// Log-linear form: log S = log c + z · log A.
pub fn me_ext_species_richness_log_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), real_ty()))
}
/// `IslandBiogeography : Real → Real → Real → Prop`
/// MacArthur-Wilson (1967) equilibrium theory: dS/dt = I(S) − E(S).
pub fn me_ext_island_biogeography_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), prop())))
}
/// `IslandEquilibriumRichness : Real → Real → Real`
/// Equilibrium species richness S* from immigration I and extinction E rates.
pub fn me_ext_island_equilibrium_richness_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), real_ty()))
}
/// `IslandTurnoverRate : Real → Real → Real`
/// Species turnover rate T = I(S*) = E(S*) at equilibrium.
pub fn me_ext_island_turnover_rate_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), real_ty()))
}
/// `HubbellNeutralTheory : Nat → Real → Prop`
/// Hubbell's (2001) neutral theory: J metacommunity individuals with drift rate ν.
pub fn me_ext_hubbell_neutral_theory_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), prop()))
}
/// `NeutralFundamentalBiodiversityNumber : Real → Real → Real`
/// θ = 2·J·ν (fundamental biodiversity number).
pub fn me_ext_neutral_biodiversity_number_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), real_ty()))
}
/// `NeutralRelativeAbundance : Real → Vec Real`
/// Predicted log-series distribution of relative species abundances under neutrality.
pub fn me_ext_neutral_relative_abundance_ty() -> Expr {
    arrow(real_ty(), list_ty(real_ty()))
}
/// `ESSExistence : Nat → Prop`
/// Every finite symmetric game has at least one ESS or a mixed Nash equilibrium.
pub fn me_ext_ess_existence_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `HawkDoveESS : Real → Real → Real`
/// Hawk-Dove game: ESS frequency of Hawks = V/C (value / cost).
pub fn me_ext_hawk_dove_ess_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), real_ty()))
}
/// `PrisonersDilemmaESS : Real → Real → Prop`
/// In iterated prisoner's dilemma, tit-for-tat can be an ESS with discount factor w.
pub fn me_ext_prisoners_dilemma_ess_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), prop()))
}
/// `ESSStabilityCondition : Vec Real → Prop`
/// Second-order ESS stability: the Jacobian of replicator dynamics has all negative eigenvalues.
pub fn me_ext_ess_stability_condition_ty() -> Expr {
    arrow(list_ty(real_ty()), prop())
}
/// `SelectionGradient : Real → Real`
/// D(x) = ∂s(y,x)/∂y |_{y=x}: direction of evolutionary change.
pub fn me_ext_selection_gradient_ty() -> Expr {
    arrow(real_ty(), real_ty())
}
/// `BranchingPoint : Real → Prop`
/// x* is an evolutionary branching point: singular strategy that is convergence stable but
/// not evolutionarily stable (disruptive selection).
pub fn me_ext_branching_point_ty() -> Expr {
    arrow(real_ty(), prop())
}
/// `MutualInvasibility : Real → Real → Prop`
/// Two resident strategies x1, x2 can mutually invade each other's communities.
pub fn me_ext_mutual_invasibility_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), prop()))
}
/// `AdaptiveDynamicsODE : Real → Real → Prop`
/// Canonical equation of adaptive dynamics: dx/dt = (1/2) μ σ² n*(x) D(x).
pub fn me_ext_adaptive_dynamics_ode_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), prop()))
}
/// `CoevolutionaryDynamics : Nat → Vec Real → Prop`
/// Multi-species coevolutionary adaptive dynamics for n interacting species.
pub fn me_ext_coevolutionary_dynamics_ty() -> Expr {
    arrow(nat_ty(), arrow(list_ty(real_ty()), prop()))
}
/// `MetacommunityPatchDynamics : Nat → Real → Real → Prop`
/// Patch dynamics model with n patches, colonization c and extinction e.
pub fn me_ext_metacommunity_patch_dynamics_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), arrow(real_ty(), prop())))
}
/// `SourceSinkDynamics : Real → Real → Prop`
/// Source-sink model: source patches (λ > 1) rescue sink patches (λ < 1).
pub fn me_ext_source_sink_dynamics_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), prop()))
}
/// `RescueEffect : Real → Real → Prop`
/// Rescue effect: immigration from sources reduces local extinction probability.
pub fn me_ext_rescue_effect_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), prop()))
}
/// `MetacommunityNeutral : Nat → Real → Prop`
/// Neutral metacommunity with J local individuals and dispersal fraction m.
pub fn me_ext_metacommunity_neutral_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), prop()))
}
/// `RegionalSpeciesPool : Nat → Nat → Prop`
/// Regional species pool of S species feeding J local communities.
pub fn me_ext_regional_species_pool_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `LeslieMatrixEigenvalue : Nat → Real → Prop`
/// Dominant eigenvalue λ₁ of Leslie matrix determines population growth rate.
pub fn me_ext_leslie_matrix_eigenvalue_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), prop()))
}
/// `LeslieMatrixStableStage : Nat → Vec Real → Prop`
/// Stable stage distribution (right eigenvector of Leslie matrix).
pub fn me_ext_leslie_stable_stage_ty() -> Expr {
    arrow(nat_ty(), arrow(list_ty(real_ty()), prop()))
}
/// `ReproductiveValue : Nat → Vec Real → Prop`
/// Reproductive values (left eigenvector of Leslie matrix).
pub fn me_ext_reproductive_value_ty() -> Expr {
    arrow(nat_ty(), arrow(list_ty(real_ty()), prop()))
}
/// `SensitivityAnalysis : Nat → Vec Real → Vec Real → Prop`
/// Sensitivity of λ₁ to changes in Leslie matrix entries.
pub fn me_ext_sensitivity_analysis_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(list_ty(real_ty()), arrow(list_ty(real_ty()), prop())),
    )
}
/// `ElasticityAnalysis : Nat → Vec Real → Vec Real → Prop`
/// Elasticity = proportional sensitivity = (aᵢⱼ/λ₁)(∂λ₁/∂aᵢⱼ).
pub fn me_ext_elasticity_analysis_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(list_ty(real_ty()), arrow(list_ty(real_ty()), prop())),
    )
}
/// `MayStabilityComplexity : Nat → Real → Real → Prop`
/// May (1972): a random community with n species, connectance C, interaction strength σ
/// is stable iff σ · √(n·C) < 1.
pub fn me_ext_may_stability_complexity_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), arrow(real_ty(), prop())))
}
/// `RandomCommunityMatrix : Nat → Real → Real → Prop`
/// Random community matrix with n species, connectance C, and variance σ².
pub fn me_ext_random_community_matrix_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), arrow(real_ty(), prop())))
}
/// `CommunityStabilityCriteria : Nat → Real → Prop`
/// Stability criteria for a community with n species and mean interaction strength s.
pub fn me_ext_community_stability_criteria_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), prop()))
}
/// `IlkahanMetapopulation : Real → Real → Real → Prop`
/// Metapopulation with rescue effect: dp/dt = c·p·(1−p) − e·p·(1−p).
pub fn me_ext_ilkahan_metapopulation_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), prop())))
}
/// `PropagulePressure : Nat → Real → Prop`
/// Propagule pressure: probability that n propagules colonize a new patch.
pub fn me_ext_propagule_pressure_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), prop()))
}
/// Register all extended mathematical ecology axioms into the environment.
pub fn register_mathematical_ecology_extended(env: &mut Environment) -> Result<(), String> {
    let axioms: &[(&str, Expr)] = &[
        (
            "LotkaVolterraGlobalExistence",
            me_ext_lv_global_existence_ty(),
        ),
        ("KolmogorovLVTheorem", me_ext_kolmogorov_lv_ty()),
        ("LVPeriodicOrbit", me_ext_lv_periodic_orbit_ty()),
        (
            "SpeciesAreaRelationship",
            me_ext_species_area_relationship_ty(),
        ),
        ("SARExponent", me_ext_sar_exponent_ty()),
        ("SpeciesRichnessLog", me_ext_species_richness_log_ty()),
        ("IslandBiogeography", me_ext_island_biogeography_ty()),
        (
            "IslandEquilibriumRichness",
            me_ext_island_equilibrium_richness_ty(),
        ),
        ("IslandTurnoverRate", me_ext_island_turnover_rate_ty()),
        ("HubbellNeutralTheory", me_ext_hubbell_neutral_theory_ty()),
        (
            "NeutralBiodiversityNumber",
            me_ext_neutral_biodiversity_number_ty(),
        ),
        (
            "NeutralRelativeAbundance",
            me_ext_neutral_relative_abundance_ty(),
        ),
        ("ESSExistence", me_ext_ess_existence_ty()),
        ("HawkDoveESS", me_ext_hawk_dove_ess_ty()),
        ("PrisonersDilemmaESS", me_ext_prisoners_dilemma_ess_ty()),
        ("ESSStabilityCondition", me_ext_ess_stability_condition_ty()),
        ("SelectionGradient", me_ext_selection_gradient_ty()),
        ("BranchingPoint", me_ext_branching_point_ty()),
        ("MutualInvasibility", me_ext_mutual_invasibility_ty()),
        ("AdaptiveDynamicsODE", me_ext_adaptive_dynamics_ode_ty()),
        (
            "CoevolutionaryDynamics",
            me_ext_coevolutionary_dynamics_ty(),
        ),
        (
            "MetacommunityPatchDynamics",
            me_ext_metacommunity_patch_dynamics_ty(),
        ),
        ("SourceSinkDynamics", me_ext_source_sink_dynamics_ty()),
        ("RescueEffect", me_ext_rescue_effect_ty()),
        ("MetacommunityNeutral", me_ext_metacommunity_neutral_ty()),
        ("RegionalSpeciesPool", me_ext_regional_species_pool_ty()),
        (
            "LeslieMatrixEigenvalue",
            me_ext_leslie_matrix_eigenvalue_ty(),
        ),
        ("LeslieStableStage", me_ext_leslie_stable_stage_ty()),
        ("ReproductiveValue", me_ext_reproductive_value_ty()),
        ("SensitivityAnalysis", me_ext_sensitivity_analysis_ty()),
        ("ElasticityAnalysis", me_ext_elasticity_analysis_ty()),
        (
            "MayStabilityComplexity",
            me_ext_may_stability_complexity_ty(),
        ),
        ("RandomCommunityMatrix", me_ext_random_community_matrix_ty()),
        (
            "CommunityStabilityCriteria",
            me_ext_community_stability_criteria_ty(),
        ),
        ("IlkahanMetapopulation", me_ext_ilkahan_metapopulation_ty()),
        ("PropagulePressure", me_ext_propagule_pressure_ty()),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .map_err(|e| format!("Failed to register {}: {:?}", name, e))?;
    }
    Ok(())
}
