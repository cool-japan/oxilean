//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    BooleanGeneNetwork, BooleanNetwork, ChemReaction, CmeState, FBAModel, GillespieAlgorithm,
    GillespieTrajectory, Jacobian2x2, LotkaVolterraSimulation, MichaelisMentenKinetics, PetriNet,
    PetriTransition, ReactionNetwork, SIREpidemicModel, SIRState, Stability, ToggleSwitchOde,
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
pub fn nat_ty() -> Expr {
    cst("Nat")
}
pub fn real_ty() -> Expr {
    cst("Real")
}
pub fn bool_ty() -> Expr {
    cst("Bool")
}
pub fn list_ty(a: Expr) -> Expr {
    app(cst("List"), a)
}
pub fn vec_ty(a: Expr) -> Expr {
    app(cst("Vec"), a)
}
/// `Species : Type` — chemical species in a reaction network.
pub fn species_ty() -> Expr {
    type0()
}
/// `Reaction : Type` — a chemical reaction (reactants + products + rate).
pub fn reaction_ty() -> Expr {
    type0()
}
/// `ReactionNetwork : Type` — a set of species and reactions.
pub fn reaction_network_ty() -> Expr {
    type0()
}
/// `StoichiometricMatrix : Type` — integer matrix N (species × reactions).
pub fn stoich_matrix_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), cst("Int")))
}
/// `FluxVector : Type` — vector of reaction fluxes v.
pub fn flux_vector_ty() -> Expr {
    arrow(nat_ty(), real_ty())
}
/// `ConcentrationVector : Type` — state vector of species concentrations.
pub fn concentration_vector_ty() -> Expr {
    arrow(nat_ty(), real_ty())
}
/// `RateFunction : Type` — function mapping concentrations to reaction rates.
pub fn rate_function_ty() -> Expr {
    arrow(concentration_vector_ty(), real_ty())
}
/// `Equilibrium : Type` — steady-state concentration vector.
pub fn equilibrium_ty() -> Expr {
    concentration_vector_ty()
}
/// `BooleanNetwork : Type` — a Boolean regulatory network.
pub fn boolean_network_ty() -> Expr {
    type0()
}
/// `PetriNet : Type` — a Petri net for biochemical processes.
pub fn petri_net_ty() -> Expr {
    type0()
}
/// Conservation law: N·v = 0 for every flux vector in the null space.
///
/// `stoich_conservation : ∀ (N : StoichiometricMatrix) (v : FluxVector),
///     InNullSpace N v → N·v = 0`
pub fn stoich_conservation_ty() -> Expr {
    let n_ty = stoich_matrix_ty();
    let v_ty = flux_vector_ty();
    let in_null = app2(cst("InNullSpace"), cst("N"), cst("v"));
    let dot_zero = app2(
        cst("Eq"),
        app2(cst("MatVecMul"), cst("N"), cst("v")),
        cst("ZeroVec"),
    );
    arrow(n_ty, arrow(v_ty, arrow(in_null, dot_zero)))
}
/// Deficiency zero theorem: a weakly reversible network with deficiency zero
/// has a unique positive equilibrium in each stoichiometric compatibility class.
///
/// `deficiency_zero_thm : ∀ (rn : ReactionNetwork),
///     WeaklyReversible rn → Deficiency rn = 0 → HasUniquePositiveEquilibrium rn`
pub fn deficiency_zero_thm_ty() -> Expr {
    let rn_ty = reaction_network_ty();
    let wrev = app(cst("WeaklyReversible"), cst("rn"));
    let def_zero = app2(
        cst("Eq"),
        app(cst("Deficiency"), cst("rn")),
        cst("Nat.zero"),
    );
    let concl = app(cst("HasUniquePositiveEquilibrium"), cst("rn"));
    arrow(rn_ty, arrow(wrev, arrow(def_zero, concl)))
}
/// Gillespie SSA correctness: the SSA samples from the exact CME solution.
///
/// `gillespie_exact : ∀ (rn : ReactionNetwork) (t : Real),
///     GillespieDistribution rn t = CMESolution rn t`
pub fn gillespie_exact_ty() -> Expr {
    let rn_ty = reaction_network_ty();
    let lhs = app2(cst("GillespieDistribution"), cst("rn"), cst("t"));
    let rhs = app2(cst("CMESolution"), cst("rn"), cst("t"));
    let concl = app2(cst("Eq"), lhs, rhs);
    arrow(rn_ty, arrow(real_ty(), concl))
}
/// Hill kinetics: activation is monotone increasing in signal.
///
/// `hill_monotone : ∀ (n : Nat) (K s1 s2 : Real),
///     s1 ≤ s2 → hill_activation n K s1 ≤ hill_activation n K s2`
pub fn hill_monotone_ty() -> Expr {
    let leq_s = app2(cst("Le"), cst("s1"), cst("s2"));
    let concl = app2(
        cst("Le"),
        app3(cst("hill_activation"), cst("n"), cst("K"), cst("s1")),
        app3(cst("hill_activation"), cst("n"), cst("K"), cst("s2")),
    );
    arrow(
        nat_ty(),
        arrow(
            real_ty(),
            arrow(real_ty(), arrow(real_ty(), arrow(leq_s, concl))),
        ),
    )
}
/// Flux Balance Analysis: optimal flux satisfies the steady-state constraint.
///
/// `fba_steady_state : ∀ (N : StoichiometricMatrix) (v_opt : FluxVector),
///     FBAOptimal N v_opt → N·v_opt = 0`
pub fn fba_steady_state_ty() -> Expr {
    let n_ty = stoich_matrix_ty();
    let v_ty = flux_vector_ty();
    let opt = app2(cst("FBAOptimal"), cst("N"), cst("v_opt"));
    let nv_zero = app2(
        cst("Eq"),
        app2(cst("MatVecMul"), cst("N"), cst("v_opt")),
        cst("ZeroVec"),
    );
    arrow(n_ty, arrow(v_ty, arrow(opt, nv_zero)))
}
/// Hopf bifurcation: crossing imaginary axis implies periodic orbit.
///
/// `hopf_bifurcation : ∀ (f : OdeSystem) (μ₀ : Real),
///     CrossesImaginaryAxis f μ₀ → ExistsPeriodicOrbit f μ₀`
pub fn hopf_bifurcation_ty() -> Expr {
    let ode_ty = arrow(concentration_vector_ty(), concentration_vector_ty());
    let cross = app2(cst("CrossesImaginaryAxis"), cst("f"), cst("mu0"));
    let concl = app2(cst("ExistsPeriodicOrbit"), cst("f"), cst("mu0"));
    arrow(ode_ty, arrow(real_ty(), arrow(cross, concl)))
}
/// Gene regulatory network: Boolean update is deterministic.
///
/// `grn_boolean_deterministic : ∀ (bn : BooleanNetwork) (s : BoolState),
///     IsFunction (UpdateRule bn) s`
pub fn grn_boolean_deterministic_ty() -> Expr {
    let bn_ty = boolean_network_ty();
    let s_ty = list_ty(bool_ty());
    let concl = app2(
        cst("IsFunction"),
        app(cst("UpdateRule"), cst("bn")),
        cst("s"),
    );
    arrow(bn_ty, arrow(s_ty, concl))
}
/// Protein-protein interaction: symmetry of undirected PPI graph.
///
/// `ppi_symmetry : ∀ (G : PPIGraph) (u v : Protein),
///     Interacts G u v → Interacts G v u`
pub fn ppi_symmetry_ty() -> Expr {
    let g_ty = cst("PPIGraph");
    let p_ty = cst("Protein");
    let interacts_uv = app3(cst("Interacts"), cst("G"), cst("u"), cst("v"));
    let interacts_vu = app3(cst("Interacts"), cst("G"), cst("v"), cst("u"));
    arrow(
        g_ty,
        arrow(p_ty.clone(), arrow(p_ty, arrow(interacts_uv, interacts_vu))),
    )
}
/// Metabolic network: mass balance at steady state N·v = 0.
///
/// `metabolic_mass_balance : ∀ (N : StoichiometricMatrix) (v : FluxVector),
///     SteadyState N v → MatVecMul N v = ZeroVec`
pub fn metabolic_mass_balance_ty() -> Expr {
    let n_ty = stoich_matrix_ty();
    let v_ty = flux_vector_ty();
    let ss = app2(cst("SteadyState"), cst("N"), cst("v"));
    let balance = app2(
        cst("Eq"),
        app2(cst("MatVecMul"), cst("N"), cst("v")),
        cst("ZeroVec"),
    );
    arrow(n_ty, arrow(v_ty, arrow(ss, balance)))
}
/// Michaelis-Menten kinetics: rate saturates at Vmax.
///
/// `mm_saturation : ∀ (Vmax Km s : Real),
///     MichaelisMentenRate Vmax Km s ≤ Vmax`
pub fn mm_saturation_ty() -> Expr {
    let rate = app3(cst("MichaelisMentenRate"), cst("Vmax"), cst("Km"), cst("s"));
    let concl = app2(cst("Le"), rate, cst("Vmax"));
    arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), concl)))
}
/// Hill equation: cooperativity amplifies response.
///
/// `hill_cooperativity : ∀ (n1 n2 K s : Real),
///     n1 ≤ n2 → hill_activation n1 K s ≤ hill_activation n2 K s`
pub fn hill_cooperativity_ty() -> Expr {
    let leq_n = app2(cst("Le"), cst("n1"), cst("n2"));
    let lhs = app3(cst("hill_activation"), cst("n1"), cst("K"), cst("s"));
    let rhs = app3(cst("hill_activation"), cst("n2"), cst("K"), cst("s"));
    let concl = app2(cst("Le"), lhs, rhs);
    arrow(
        real_ty(),
        arrow(
            real_ty(),
            arrow(real_ty(), arrow(real_ty(), arrow(leq_n, concl))),
        ),
    )
}
/// Lotka-Volterra: orbits are closed in the conservative case.
///
/// `lv_conservative_orbits : ∀ (α β γ δ : Real),
///     LVConservative α β γ δ → ExistsConservedQuantity (LVSystem α β γ δ)`
pub fn lv_conservative_orbits_ty() -> Expr {
    let lv_cons = app(
        app(
            app(app(cst("LVConservative"), cst("alpha")), cst("beta")),
            cst("gamma"),
        ),
        cst("delta"),
    );
    let system = app(
        app(
            app(app(cst("LVSystem"), cst("alpha")), cst("beta")),
            cst("gamma"),
        ),
        cst("delta"),
    );
    let concl = app(cst("ExistsConservedQuantity"), system);
    arrow(
        real_ty(),
        arrow(
            real_ty(),
            arrow(real_ty(), arrow(real_ty(), arrow(lv_cons, concl))),
        ),
    )
}
/// Competitive exclusion: two species competing for one resource cannot coexist.
///
/// `competitive_exclusion : ∀ (rn : ReactionNetwork),
///     TwoSpeciesOneResource rn → ExcludesWeaker rn`
pub fn competitive_exclusion_ty() -> Expr {
    let rn_ty = reaction_network_ty();
    let tsor = app(cst("TwoSpeciesOneResource"), cst("rn"));
    let concl = app(cst("ExcludesWeaker"), cst("rn"));
    arrow(rn_ty, arrow(tsor, concl))
}
/// SIR epidemic: basic reproduction number R0 > 1 implies epidemic.
///
/// `sir_epidemic_threshold : ∀ (β γ N : Real),
///     R0SIR β γ N > 1 → EpidemicOccurs (SIRModel β γ N)`
pub fn sir_epidemic_threshold_ty() -> Expr {
    let r0 = app3(cst("R0SIR"), cst("beta"), cst("gamma"), cst("N"));
    let threshold = app2(cst("Gt"), r0, cst("Real.one"));
    let model = app3(cst("SIRModel"), cst("beta"), cst("gamma"), cst("N"));
    let concl = app(cst("EpidemicOccurs"), model);
    arrow(
        real_ty(),
        arrow(real_ty(), arrow(real_ty(), arrow(threshold, concl))),
    )
}
/// SEIR epidemic: exposed class smooths outbreak curve.
///
/// `seir_peak_delay : ∀ (m : SEIRModel),
///     PeakTime (SEIRInfected m) ≥ PeakTime (SIRInfected (ProjectSEIR m))`
pub fn seir_peak_delay_ty() -> Expr {
    let m_ty = cst("SEIRModel");
    let lhs = app(cst("PeakTime"), app(cst("SEIRInfected"), cst("m")));
    let rhs = app(
        cst("PeakTime"),
        app(cst("SIRInfected"), app(cst("ProjectSEIR"), cst("m"))),
    );
    let concl = app2(cst("Ge"), lhs, rhs);
    arrow(m_ty, concl)
}
/// Saddle-node bifurcation: equilibria annihilate pairwise.
///
/// `saddle_node_bif : ∀ (f : OdeSystem) (μ : Real),
///     SaddleNodeCondition f μ → TwoEquilibriaCollide f μ`
pub fn saddle_node_bif_ty() -> Expr {
    let ode_ty = arrow(concentration_vector_ty(), concentration_vector_ty());
    let sn = app2(cst("SaddleNodeCondition"), cst("f"), cst("mu"));
    let concl = app2(cst("TwoEquilibriaCollide"), cst("f"), cst("mu"));
    arrow(ode_ty, arrow(real_ty(), arrow(sn, concl)))
}
/// Turing instability: diffusion-driven instability in activator-inhibitor systems.
///
/// `turing_instability : ∀ (D_a D_i κ : Real),
///     TuringCondition D_a D_i κ → DiffusionDrivenInstability D_a D_i κ`
pub fn turing_instability_ty() -> Expr {
    let tc = app3(cst("TuringCondition"), cst("Da"), cst("Di"), cst("kappa"));
    let concl = app3(
        cst("DiffusionDrivenInstability"),
        cst("Da"),
        cst("Di"),
        cst("kappa"),
    );
    arrow(
        real_ty(),
        arrow(real_ty(), arrow(real_ty(), arrow(tc, concl))),
    )
}
/// Cell cycle: G1/S checkpoint prevents premature replication.
///
/// `g1s_checkpoint : ∀ (cell : CellState),
///     G1SCheckpointActive cell → Not (EnteringS cell)`
pub fn g1s_checkpoint_ty() -> Expr {
    let cell_ty = cst("CellState");
    let checkpoint = app(cst("G1SCheckpointActive"), cst("cell"));
    let concl = app(cst("Not"), app(cst("EnteringS"), cst("cell")));
    arrow(cell_ty, arrow(checkpoint, concl))
}
/// Goodwin oscillator: negative feedback with time delay generates oscillations.
///
/// `goodwin_oscillation : ∀ (n : Nat) (K : Real),
///     n ≥ 8 → GoodwinOscillates n K`
pub fn goodwin_oscillation_ty() -> Expr {
    let n_large = app2(cst("Ge"), cst("n"), app(cst("Nat.ofNat"), cst("8")));
    let concl = app2(cst("GoodwinOscillates"), cst("n"), cst("K"));
    arrow(nat_ty(), arrow(real_ty(), arrow(n_large, concl)))
}
/// Brusselator: chemical oscillator exists for suitable parameters.
///
/// `brusselator_oscillation : ∀ (A B : Real),
///     B > 1 + A * A → BrusselatorOscillates A B`
pub fn brusselator_oscillation_ty() -> Expr {
    let rhs = app2(
        cst("Add"),
        cst("Real.one"),
        app2(cst("Mul"), cst("A"), cst("A")),
    );
    let threshold = app2(cst("Gt"), cst("B"), rhs);
    let concl = app2(cst("BrusselatorOscillates"), cst("A"), cst("B"));
    arrow(real_ty(), arrow(real_ty(), arrow(threshold, concl)))
}
/// Stochastic gene expression: mRNA distribution is Poisson in constitutive model.
///
/// `mrna_poisson : ∀ (λ : Real) (n : Nat),
///     ConstitutiveExpression λ → MRNADistribution λ n = PoissonPMF λ n`
pub fn mrna_poisson_ty() -> Expr {
    let const_expr = app(cst("ConstitutiveExpression"), cst("lambda"));
    let lhs = app2(cst("MRNADistribution"), cst("lambda"), cst("n"));
    let rhs = app2(cst("PoissonPMF"), cst("lambda"), cst("n"));
    let concl = app2(cst("Eq"), lhs, rhs);
    arrow(real_ty(), arrow(nat_ty(), arrow(const_expr, concl)))
}
/// Chemical master equation: probability is conserved.
///
/// `cme_probability_conservation : ∀ (rn : ReactionNetwork) (t : Real),
///     SumAllStates (CMESolution rn t) = Real.one`
pub fn cme_probability_conservation_ty() -> Expr {
    let rn_ty = reaction_network_ty();
    let solution = app2(cst("CMESolution"), cst("rn"), cst("t"));
    let concl = app2(
        cst("Eq"),
        app(cst("SumAllStates"), solution),
        cst("Real.one"),
    );
    arrow(rn_ty, arrow(real_ty(), concl))
}
/// Gillespie next-event distribution: waiting time is exponential.
///
/// `gillespie_waiting_time : ∀ (rn : ReactionNetwork) (x : State),
///     WaitingTimeDistribution rn x = Exponential (TotalPropensity rn x)`
pub fn gillespie_waiting_time_ty() -> Expr {
    let rn_ty = reaction_network_ty();
    let state_ty = list_ty(nat_ty());
    let lhs = app2(cst("WaitingTimeDistribution"), cst("rn"), cst("x"));
    let rhs = app(
        cst("Exponential"),
        app2(cst("TotalPropensity"), cst("rn"), cst("x")),
    );
    let concl = app2(cst("Eq"), lhs, rhs);
    arrow(rn_ty, arrow(state_ty, concl))
}
/// Feedforward loop motif: coherent type-1 delays response to signal removal.
///
/// `ffl_coherent_delay : ∀ (net : SignalingNetwork),
///     CoherentType1FFL net → DelaysOffResponse net`
pub fn ffl_coherent_delay_ty() -> Expr {
    let net_ty = cst("SignalingNetwork");
    let c1ffl = app(cst("CoherentType1FFL"), cst("net"));
    let concl = app(cst("DelaysOffResponse"), cst("net"));
    arrow(net_ty, arrow(c1ffl, concl))
}
/// Negative feedback loop: reduces steady-state gain.
///
/// `neg_feedback_attenuation : ∀ (net : SignalingNetwork),
///     HasNegativeFeedback net → SteadyStateGain net < OpenLoopGain net`
pub fn neg_feedback_attenuation_ty() -> Expr {
    let net_ty = cst("SignalingNetwork");
    let nfb = app(cst("HasNegativeFeedback"), cst("net"));
    let concl = app2(
        cst("Lt"),
        app(cst("SteadyStateGain"), cst("net")),
        app(cst("OpenLoopGain"), cst("net")),
    );
    arrow(net_ty, arrow(nfb, concl))
}
/// Robustness: network output is insensitive to parameter perturbations.
///
/// `network_robustness : ∀ (net : SignalingNetwork) (ε : Real),
///     IsRobust net ε → ∀ (δp : ParameterPerturbation),
///         SmallPerturbation δp ε → SmallOutputChange net δp ε`
pub fn network_robustness_ty() -> Expr {
    let net_ty = cst("SignalingNetwork");
    let robust = app2(cst("IsRobust"), cst("net"), cst("eps"));
    let small_pert = app2(cst("SmallPerturbation"), cst("delta_p"), cst("eps"));
    let small_out = app3(
        cst("SmallOutputChange"),
        cst("net"),
        cst("delta_p"),
        cst("eps"),
    );
    let inner = arrow(cst("ParameterPerturbation"), arrow(small_pert, small_out));
    arrow(net_ty, arrow(real_ty(), arrow(robust, inner)))
}
/// Toggle switch: bistability requires sufficient nonlinearity.
///
/// `toggle_bistability : ∀ (α1 α2 β γ : Real),
///     β ≥ 2 → γ ≥ 2 → ToggledSwitchIsBistable α1 α2 β γ`
pub fn toggle_bistability_ty() -> Expr {
    let beta_large = app2(cst("Ge"), cst("beta"), app(cst("Nat.ofNat"), cst("2")));
    let gamma_large = app2(cst("Ge"), cst("gamma"), app(cst("Nat.ofNat"), cst("2")));
    let concl = app(
        app(
            app(
                app(cst("ToggledSwitchIsBistable"), cst("alpha1")),
                cst("alpha2"),
            ),
            cst("beta"),
        ),
        cst("gamma"),
    );
    arrow(
        real_ty(),
        arrow(
            real_ty(),
            arrow(
                real_ty(),
                arrow(real_ty(), arrow(beta_large, arrow(gamma_large, concl))),
            ),
        ),
    )
}
/// Repressilator: three-gene negative feedback creates oscillations.
///
/// `repressilator_oscillation : ∀ (n : Nat) (α K : Real),
///     n ≥ 2 → RepressilatorOscillates n α K`
pub fn repressilator_oscillation_ty() -> Expr {
    let n_large = app2(cst("Ge"), cst("n"), app(cst("Nat.ofNat"), cst("2")));
    let concl = app3(
        cst("RepressilatorOscillates"),
        cst("n"),
        cst("alpha"),
        cst("K"),
    );
    arrow(
        nat_ty(),
        arrow(real_ty(), arrow(real_ty(), arrow(n_large, concl))),
    )
}
/// Signaling pathway ODE: trajectories remain in positive orthant.
///
/// `signaling_positive_orthant : ∀ (f : SignalingODE) (x0 : ConcentrationVector),
///     PositiveInitial x0 → PositiveTrajectory f x0`
pub fn signaling_positive_orthant_ty() -> Expr {
    let ode_ty = arrow(concentration_vector_ty(), concentration_vector_ty());
    let pos_init = app(cst("PositiveInitial"), cst("x0"));
    let concl = app2(cst("PositiveTrajectory"), cst("f"), cst("x0"));
    arrow(
        ode_ty,
        arrow(concentration_vector_ty(), arrow(pos_init, concl)),
    )
}
/// Pattern formation: Turing pattern wavelength scales with diffusion ratio.
///
/// `turing_wavelength : ∀ (D_a D_i : Real),
///     D_i > D_a → TuringWavelength D_a D_i = OptimalWavelength D_a D_i`
pub fn turing_wavelength_ty() -> Expr {
    let ratio_cond = app2(cst("Gt"), cst("Di"), cst("Da"));
    let lhs = app2(cst("TuringWavelength"), cst("Da"), cst("Di"));
    let rhs = app2(cst("OptimalWavelength"), cst("Da"), cst("Di"));
    let concl = app2(cst("Eq"), lhs, rhs);
    arrow(real_ty(), arrow(real_ty(), arrow(ratio_cond, concl)))
}
/// Excitability: FitzHugh-Nagumo system has a threshold for spiking.
///
/// `fitzhugh_nagumo_excitability : ∀ (I : Real),
///     I > FNThreshold → FNSpikes I`
pub fn fitzhugh_nagumo_excitability_ty() -> Expr {
    let threshold_cond = app2(cst("Gt"), cst("I"), cst("FNThreshold"));
    let concl = app(cst("FNSpikes"), cst("I"));
    arrow(real_ty(), arrow(threshold_cond, concl))
}
/// Enzyme catalysis: Michaelis-Menten equation from quasi-steady-state approximation.
///
/// `mm_qssa_derivation : ∀ (E S ES P : Species) (k1 km1 k2 : Real),
///     QSSAValid E S ES P k1 km1 k2 →
///     ProductionRate E S ES P k1 km1 k2 = MichaelisMentenRate (k2 * E0) (km1 + k2) / k1 S`
pub fn mm_qssa_derivation_ty() -> Expr {
    let qssa = app(
        app(
            app(
                app(
                    app(app(app(cst("QSSAValid"), cst("E")), cst("S")), cst("ES")),
                    cst("P"),
                ),
                cst("k1"),
            ),
            cst("km1"),
        ),
        cst("k2"),
    );
    let lhs = app(
        app(
            app(
                app(
                    app(
                        app(app(cst("ProductionRate"), cst("E")), cst("S")),
                        cst("ES"),
                    ),
                    cst("P"),
                ),
                cst("k1"),
            ),
            cst("km1"),
        ),
        cst("k2"),
    );
    let rhs = app(
        app(
            app(
                cst("MichaelisMentenApprox"),
                app2(cst("Mul"), cst("k2"), cst("E0")),
            ),
            app2(cst("Add"), cst("km1"), cst("k2")),
        ),
        cst("S"),
    );
    let concl = app2(cst("Eq"), lhs, rhs);
    arrow(
        cst("Species"),
        arrow(
            cst("Species"),
            arrow(
                cst("Species"),
                arrow(
                    cst("Species"),
                    arrow(
                        real_ty(),
                        arrow(real_ty(), arrow(real_ty(), arrow(qssa, concl))),
                    ),
                ),
            ),
        ),
    )
}
/// Adiabatic elimination: fast variables track slow manifold.
///
/// `adiabatic_elimination : ∀ (f : OdeSystem) (ε : Real),
///     HasTimescaleSeparation f ε →
///     SlowManifoldApproximation f ε`
pub fn adiabatic_elimination_ty() -> Expr {
    let ode_ty = arrow(concentration_vector_ty(), concentration_vector_ty());
    let sep = app2(cst("HasTimescaleSeparation"), cst("f"), cst("eps"));
    let concl = app2(cst("SlowManifoldApproximation"), cst("f"), cst("eps"));
    arrow(ode_ty, arrow(real_ty(), arrow(sep, concl)))
}
/// Noise in gene expression: intrinsic noise scales as 1/mean.
///
/// `gene_expression_noise : ∀ (λ : Real),
///     λ > 0 → IntrinsicNoiseCVSq λ = Real.inv λ`
pub fn gene_expression_noise_ty() -> Expr {
    let pos = app2(cst("Gt"), cst("lambda"), cst("Real.zero"));
    let lhs = app(cst("IntrinsicNoiseCVSq"), cst("lambda"));
    let rhs = app(cst("Real.inv"), cst("lambda"));
    let concl = app2(cst("Eq"), lhs, rhs);
    arrow(real_ty(), arrow(pos, concl))
}
/// Register systems biology axioms and theorems in the OxiLean kernel environment.
pub fn build_systems_biology_env(env: &mut Environment) -> Result<(), Box<dyn std::error::Error>> {
    let axioms: &[(&str, Expr)] = &[
        ("Species", species_ty()),
        ("Reaction", reaction_ty()),
        ("ReactionNetwork", reaction_network_ty()),
        ("StoichiometricMatrix", stoich_matrix_ty()),
        ("FluxVector", flux_vector_ty()),
        ("ConcentrationVector", concentration_vector_ty()),
        ("RateFunction", rate_function_ty()),
        ("BooleanNetwork", boolean_network_ty()),
        ("PetriNet", petri_net_ty()),
        ("PPIGraph", type0()),
        ("Protein", type0()),
        ("SEIRModel", type0()),
        ("CellState", type0()),
        ("SignalingNetwork", type0()),
        ("ParameterPerturbation", type0()),
        (
            "InNullSpace",
            arrow(stoich_matrix_ty(), arrow(flux_vector_ty(), prop())),
        ),
        ("WeaklyReversible", arrow(reaction_network_ty(), prop())),
        (
            "HasUniquePositiveEquilibrium",
            arrow(reaction_network_ty(), prop()),
        ),
        (
            "FBAOptimal",
            arrow(stoich_matrix_ty(), arrow(flux_vector_ty(), prop())),
        ),
        (
            "CrossesImaginaryAxis",
            arrow(
                arrow(concentration_vector_ty(), concentration_vector_ty()),
                arrow(real_ty(), prop()),
            ),
        ),
        (
            "ExistsPeriodicOrbit",
            arrow(
                arrow(concentration_vector_ty(), concentration_vector_ty()),
                arrow(real_ty(), prop()),
            ),
        ),
        (
            "IsFunction",
            arrow(type0(), arrow(list_ty(bool_ty()), prop())),
        ),
        ("UpdateRule", arrow(boolean_network_ty(), type0())),
        (
            "Interacts",
            arrow(
                cst("PPIGraph"),
                arrow(cst("Protein"), arrow(cst("Protein"), prop())),
            ),
        ),
        (
            "SteadyState",
            arrow(stoich_matrix_ty(), arrow(flux_vector_ty(), prop())),
        ),
        ("ConstitutiveExpression", arrow(real_ty(), prop())),
        (
            "LVConservative",
            arrow(
                real_ty(),
                arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), prop()))),
            ),
        ),
        (
            "TwoSpeciesOneResource",
            arrow(reaction_network_ty(), prop()),
        ),
        ("ExcludesWeaker", arrow(reaction_network_ty(), prop())),
        ("EpidemicOccurs", arrow(type0(), prop())),
        (
            "SaddleNodeCondition",
            arrow(
                arrow(concentration_vector_ty(), concentration_vector_ty()),
                arrow(real_ty(), prop()),
            ),
        ),
        (
            "TwoEquilibriaCollide",
            arrow(
                arrow(concentration_vector_ty(), concentration_vector_ty()),
                arrow(real_ty(), prop()),
            ),
        ),
        (
            "TuringCondition",
            arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), prop()))),
        ),
        (
            "DiffusionDrivenInstability",
            arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), prop()))),
        ),
        ("G1SCheckpointActive", arrow(cst("CellState"), prop())),
        ("EnteringS", arrow(cst("CellState"), prop())),
        (
            "GoodwinOscillates",
            arrow(nat_ty(), arrow(real_ty(), prop())),
        ),
        (
            "BrusselatorOscillates",
            arrow(real_ty(), arrow(real_ty(), prop())),
        ),
        ("CoherentType1FFL", arrow(cst("SignalingNetwork"), prop())),
        ("DelaysOffResponse", arrow(cst("SignalingNetwork"), prop())),
        (
            "HasNegativeFeedback",
            arrow(cst("SignalingNetwork"), prop()),
        ),
        (
            "IsRobust",
            arrow(cst("SignalingNetwork"), arrow(real_ty(), prop())),
        ),
        (
            "SmallPerturbation",
            arrow(cst("ParameterPerturbation"), arrow(real_ty(), prop())),
        ),
        (
            "SmallOutputChange",
            arrow(
                cst("SignalingNetwork"),
                arrow(cst("ParameterPerturbation"), arrow(real_ty(), prop())),
            ),
        ),
        ("PositiveInitial", arrow(concentration_vector_ty(), prop())),
        (
            "PositiveTrajectory",
            arrow(
                arrow(concentration_vector_ty(), concentration_vector_ty()),
                arrow(concentration_vector_ty(), prop()),
            ),
        ),
        (
            "QSSAValid",
            arrow(
                cst("Species"),
                arrow(
                    cst("Species"),
                    arrow(
                        cst("Species"),
                        arrow(
                            cst("Species"),
                            arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), prop()))),
                        ),
                    ),
                ),
            ),
        ),
        (
            "HasTimescaleSeparation",
            arrow(
                arrow(concentration_vector_ty(), concentration_vector_ty()),
                arrow(real_ty(), prop()),
            ),
        ),
        (
            "SlowManifoldApproximation",
            arrow(
                arrow(concentration_vector_ty(), concentration_vector_ty()),
                arrow(real_ty(), prop()),
            ),
        ),
        ("Deficiency", arrow(reaction_network_ty(), nat_ty())),
        (
            "hill_activation",
            arrow(nat_ty(), arrow(real_ty(), arrow(real_ty(), real_ty()))),
        ),
        (
            "GillespieDistribution",
            arrow(reaction_network_ty(), arrow(real_ty(), type0())),
        ),
        (
            "CMESolution",
            arrow(reaction_network_ty(), arrow(real_ty(), type0())),
        ),
        (
            "MichaelisMentenRate",
            arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), real_ty()))),
        ),
        (
            "MichaelisMentenApprox",
            arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), real_ty()))),
        ),
        (
            "R0SIR",
            arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), real_ty()))),
        ),
        (
            "SIRModel",
            arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), type0()))),
        ),
        (
            "SEIRInfected",
            arrow(cst("SEIRModel"), arrow(real_ty(), real_ty())),
        ),
        ("SIRInfected", arrow(type0(), arrow(real_ty(), real_ty()))),
        ("ProjectSEIR", arrow(cst("SEIRModel"), type0())),
        ("PeakTime", arrow(arrow(real_ty(), real_ty()), real_ty())),
        (
            "LVSystem",
            arrow(
                real_ty(),
                arrow(
                    real_ty(),
                    arrow(
                        real_ty(),
                        arrow(
                            real_ty(),
                            arrow(concentration_vector_ty(), concentration_vector_ty()),
                        ),
                    ),
                ),
            ),
        ),
        (
            "ExistsConservedQuantity",
            arrow(
                arrow(concentration_vector_ty(), concentration_vector_ty()),
                prop(),
            ),
        ),
        (
            "MRNADistribution",
            arrow(real_ty(), arrow(nat_ty(), real_ty())),
        ),
        ("PoissonPMF", arrow(real_ty(), arrow(nat_ty(), real_ty()))),
        ("SumAllStates", arrow(type0(), real_ty())),
        (
            "WaitingTimeDistribution",
            arrow(reaction_network_ty(), arrow(list_ty(nat_ty()), type0())),
        ),
        (
            "TotalPropensity",
            arrow(reaction_network_ty(), arrow(list_ty(nat_ty()), real_ty())),
        ),
        ("Exponential", arrow(real_ty(), type0())),
        ("SteadyStateGain", arrow(cst("SignalingNetwork"), real_ty())),
        ("OpenLoopGain", arrow(cst("SignalingNetwork"), real_ty())),
        (
            "ToggledSwitchIsBistable",
            arrow(
                real_ty(),
                arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), prop()))),
            ),
        ),
        (
            "RepressilatorOscillates",
            arrow(nat_ty(), arrow(real_ty(), arrow(real_ty(), prop()))),
        ),
        (
            "TuringWavelength",
            arrow(real_ty(), arrow(real_ty(), real_ty())),
        ),
        (
            "OptimalWavelength",
            arrow(real_ty(), arrow(real_ty(), real_ty())),
        ),
        ("FNThreshold", real_ty()),
        ("FNSpikes", arrow(real_ty(), prop())),
        (
            "ProductionRate",
            arrow(
                cst("Species"),
                arrow(
                    cst("Species"),
                    arrow(
                        cst("Species"),
                        arrow(
                            cst("Species"),
                            arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), real_ty()))),
                        ),
                    ),
                ),
            ),
        ),
        ("IntrinsicNoiseCVSq", arrow(real_ty(), real_ty())),
        ("E0", real_ty()),
        ("stoich_conservation", stoich_conservation_ty()),
        ("deficiency_zero_thm", deficiency_zero_thm_ty()),
        ("gillespie_exact", gillespie_exact_ty()),
        ("hill_monotone", hill_monotone_ty()),
        ("fba_steady_state", fba_steady_state_ty()),
        ("hopf_bifurcation", hopf_bifurcation_ty()),
        ("grn_boolean_deterministic", grn_boolean_deterministic_ty()),
        ("ppi_symmetry", ppi_symmetry_ty()),
        ("metabolic_mass_balance", metabolic_mass_balance_ty()),
        ("mm_saturation", mm_saturation_ty()),
        ("hill_cooperativity", hill_cooperativity_ty()),
        ("lv_conservative_orbits", lv_conservative_orbits_ty()),
        ("competitive_exclusion", competitive_exclusion_ty()),
        ("sir_epidemic_threshold", sir_epidemic_threshold_ty()),
        ("seir_peak_delay", seir_peak_delay_ty()),
        ("saddle_node_bif", saddle_node_bif_ty()),
        ("turing_instability", turing_instability_ty()),
        ("g1s_checkpoint", g1s_checkpoint_ty()),
        ("goodwin_oscillation", goodwin_oscillation_ty()),
        ("brusselator_oscillation", brusselator_oscillation_ty()),
        ("mrna_poisson", mrna_poisson_ty()),
        (
            "cme_probability_conservation",
            cme_probability_conservation_ty(),
        ),
        ("gillespie_waiting_time", gillespie_waiting_time_ty()),
        ("ffl_coherent_delay", ffl_coherent_delay_ty()),
        ("neg_feedback_attenuation", neg_feedback_attenuation_ty()),
        ("network_robustness", network_robustness_ty()),
        ("toggle_bistability", toggle_bistability_ty()),
        ("repressilator_oscillation", repressilator_oscillation_ty()),
        (
            "signaling_positive_orthant",
            signaling_positive_orthant_ty(),
        ),
        ("turing_wavelength", turing_wavelength_ty()),
        (
            "fitzhugh_nagumo_excitability",
            fitzhugh_nagumo_excitability_ty(),
        ),
        ("mm_qssa_derivation", mm_qssa_derivation_ty()),
        ("adiabatic_elimination", adiabatic_elimination_ty()),
        ("gene_expression_noise", gene_expression_noise_ty()),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    Ok(())
}
/// Gillespie exact stochastic simulation algorithm.
///
/// Simulates the reaction network up to time `t_max` starting from `initial_state`.
pub fn gillespie_ssa(
    network: &ReactionNetwork,
    initial_state: Vec<i64>,
    t_max: f64,
    max_steps: usize,
    rng_seed: u64,
) -> GillespieTrajectory {
    let mut times = vec![0.0f64];
    let mut states = vec![initial_state.clone()];
    let mut state = initial_state;
    let mut t = 0.0f64;
    let mut rng = rng_seed;
    let lcg_next = |r: &mut u64| -> f64 {
        *r = r
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let high = (*r >> 33) as f64;
        high / (u32::MAX as f64)
    };
    for _ in 0..max_steps {
        let props = network.propensities(&state);
        let total_prop: f64 = props.iter().sum();
        if total_prop <= 0.0 {
            break;
        }
        let u1 = lcg_next(&mut rng).max(1e-15);
        let tau = -u1.ln() / total_prop;
        t += tau;
        if t > t_max {
            break;
        }
        let u2 = lcg_next(&mut rng);
        let mut cumsum = 0.0;
        let mut chosen = 0usize;
        for (j, &p) in props.iter().enumerate() {
            cumsum += p;
            if u2 * total_prop <= cumsum {
                chosen = j;
                break;
            }
        }
        let rxn = &network.reactions[chosen];
        for &(si, stoich) in &rxn.reactants {
            if si < state.len() {
                state[si] -= stoich as i64;
            }
        }
        for &(si, stoich) in &rxn.products {
            if si < state.len() {
                state[si] += stoich as i64;
            }
        }
        times.push(t);
        states.push(state.clone());
    }
    GillespieTrajectory { times, states }
}
/// Hill activation function: f(s) = s^n / (K^n + s^n).
pub fn hill_activation(s: f64, n: f64, k: f64) -> f64 {
    if k <= 0.0 || s < 0.0 {
        return 0.0;
    }
    let sn = s.powf(n);
    let kn = k.powf(n);
    sn / (kn + sn)
}
/// Hill repression function: f(s) = K^n / (K^n + s^n).
pub fn hill_repression(s: f64, n: f64, k: f64) -> f64 {
    1.0 - hill_activation(s, n, k)
}
/// Check the Hopf bifurcation condition at a parameter value.
///
/// Hopf: trace of J = 0, det of J > 0, and d(trace)/d(μ) ≠ 0.
pub fn is_hopf_candidate(jac: &Jacobian2x2, d_trace_dmu: f64) -> bool {
    let tr = jac.trace();
    let det = jac.det();
    tr.abs() < 1e-8 && det > 0.0 && d_trace_dmu.abs() > 1e-12
}
/// Check the saddle-node bifurcation condition.
///
/// Saddle-node: det of J = 0, trace ≠ 0.
pub fn is_saddle_node_candidate(jac: &Jacobian2x2) -> bool {
    jac.det().abs() < 1e-8 && jac.trace().abs() > 1e-8
}
/// Propagate the CME for one time step using the Euler method.
///
/// Returns the updated probability distribution.
pub fn cme_euler_step(
    distribution: &[CmeState],
    network: &ReactionNetwork,
    dt: f64,
) -> Vec<CmeState> {
    let mut prob_map: std::collections::HashMap<Vec<i64>, f64> = distribution
        .iter()
        .map(|s| (s.counts.clone(), s.probability))
        .collect();
    let mut delta: std::collections::HashMap<Vec<i64>, f64> = std::collections::HashMap::new();
    for state in distribution {
        let props = network.propensities(&state.counts);
        let total: f64 = props.iter().sum();
        *delta.entry(state.counts.clone()).or_insert(0.0) -= total * state.probability * dt;
        for (j, rxn) in network.reactions.iter().enumerate() {
            let mut pred_counts = state.counts.clone();
            let mut valid = true;
            for &(si, stoich) in &rxn.products {
                if si < pred_counts.len() {
                    if pred_counts[si] < stoich as i64 {
                        valid = false;
                        break;
                    }
                    pred_counts[si] -= stoich as i64;
                }
            }
            for &(si, stoich) in &rxn.reactants {
                if si < pred_counts.len() {
                    pred_counts[si] += stoich as i64;
                }
            }
            if valid {
                if let Some(&pred_prob) = prob_map.get(&pred_counts) {
                    let pred_props = network.propensities(&pred_counts);
                    let a_j = if j < pred_props.len() {
                        pred_props[j]
                    } else {
                        0.0
                    };
                    *delta.entry(state.counts.clone()).or_insert(0.0) += a_j * pred_prob * dt;
                }
            }
        }
    }
    for (counts, dp) in &delta {
        let entry = prob_map.entry(counts.clone()).or_insert(0.0);
        *entry += dp;
        if *entry < 0.0 {
            *entry = 0.0;
        }
    }
    prob_map
        .into_iter()
        .filter(|(_, p)| *p > 1e-20)
        .map(|(counts, probability)| CmeState {
            counts,
            probability,
        })
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_stoichiometric_matrix_birth_death() {
        let mut net = ReactionNetwork::new(vec!["A".into()]);
        net.add_reaction(ChemReaction {
            name: "birth".into(),
            reactants: vec![],
            products: vec![(0, 1)],
            rate_constant: 1.0,
        });
        net.add_reaction(ChemReaction {
            name: "death".into(),
            reactants: vec![(0, 1)],
            products: vec![],
            rate_constant: 0.1,
        });
        let n = net.stoichiometric_matrix();
        assert_eq!(n[0][0], 1);
        assert_eq!(n[0][1], -1);
    }
    #[test]
    fn test_gillespie_trajectory_length() {
        let mut net = ReactionNetwork::new(vec!["A".into()]);
        net.add_reaction(ChemReaction {
            name: "birth".into(),
            reactants: vec![],
            products: vec![(0, 1)],
            rate_constant: 1.0,
        });
        net.add_reaction(ChemReaction {
            name: "death".into(),
            reactants: vec![(0, 1)],
            products: vec![],
            rate_constant: 0.5,
        });
        let traj = gillespie_ssa(&net, vec![0], 10.0, 10000, 42);
        assert!(!traj.times.is_empty());
        assert_eq!(traj.times.len(), traj.states.len());
        assert!(
            *traj.times.last().expect("last should succeed") <= 10.0,
            "last time must be ≤ t_max"
        );
    }
    #[test]
    fn test_toggle_switch_ode_converge() {
        let ode = ToggleSwitchOde {
            alpha1: 3.0,
            alpha2: 3.0,
            beta: 2.0,
            gamma: 2.0,
        };
        let traj = ode.integrate([0.1, 2.5], 0.05, 200);
        assert_eq!(traj.len(), 201);
        assert!(traj.last().expect("last should succeed")[0] >= 0.0);
        assert!(traj.last().expect("last should succeed")[1] >= 0.0);
    }
    #[test]
    fn test_jacobian_stable_node() {
        let jac = Jacobian2x2 {
            j00: -2.0,
            j01: 0.0,
            j10: 0.0,
            j11: -3.0,
        };
        assert_eq!(jac.classify(), Stability::StableNode);
        assert!(jac.trace() < 0.0);
        assert!(jac.det() > 0.0);
    }
    #[test]
    fn test_jacobian_saddle_point() {
        let jac = Jacobian2x2 {
            j00: 1.0,
            j01: 0.0,
            j10: 0.0,
            j11: -1.0,
        };
        assert_eq!(jac.classify(), Stability::SaddlePoint);
    }
    #[test]
    fn test_petri_net_fire() {
        let mut net = PetriNet::new(vec!["A".into(), "B".into()], vec![2, 0]);
        net.add_transition(PetriTransition {
            name: "convert".into(),
            pre: vec![1, 0],
            post: vec![0, 1],
        });
        assert!(net.is_enabled(0));
        net.fire(0);
        assert_eq!(net.marking[0], 1);
        assert_eq!(net.marking[1], 1);
    }
    #[test]
    fn test_fba_check_steady_state() {
        let stoich = vec![vec![-1.0], vec![1.0]];
        let model = FBAModel::new(stoich, vec![0.0], vec![10.0], vec![1.0]);
        assert!(!model.check_steady_state(&[1.0], 1e-8));
        assert!(model.check_steady_state(&[0.0], 1e-8));
    }
    #[test]
    fn test_hill_activation_bounds() {
        let low = hill_activation(0.0, 2.0, 1.0);
        let high = hill_activation(1e6, 2.0, 1.0);
        assert!(low < 0.01, "hill_activation(0) should be ≈ 0");
        assert!(high > 0.99, "hill_activation(∞) should be ≈ 1");
    }
    #[test]
    fn test_build_systems_biology_env() {
        let mut env = Environment::new();
        build_systems_biology_env(&mut env).expect("build should succeed");
    }
    #[test]
    fn test_sir_r0_above_one_leads_to_epidemic() {
        let model = SIREpidemicModel::new(0.3, 0.1, 1000.0);
        assert!(model.r0() > 1.0, "R0 should be 3.0 > 1");
        let initial = SIRState {
            s: 999.0,
            i: 1.0,
            r: 0.0,
        };
        let traj = model.simulate(initial, 0.1, 300);
        let peak = model.peak_infected_step(&traj);
        assert!(traj[peak].i > initial.i, "Epidemic should grow when R0 > 1");
    }
    #[test]
    fn test_sir_population_conservation() {
        let model = SIREpidemicModel::new(0.3, 0.1, 1000.0);
        let initial = SIRState {
            s: 999.0,
            i: 1.0,
            r: 0.0,
        };
        let traj = model.simulate(initial, 0.1, 200);
        for state in &traj {
            let total = state.s + state.i + state.r;
            assert!(
                (total - 1000.0).abs() < 1.0,
                "Population should be conserved: got {total}"
            );
        }
    }
    #[test]
    fn test_lotka_volterra_oscillations() {
        let lv = LotkaVolterraSimulation::new(1.0, 0.1, 0.1, 0.01);
        let traj = lv.simulate(80.0, 20.0, 0.01, 5000);
        assert!(traj.iter().all(|(x, y)| *x >= 0.0 && *y >= 0.0));
        let v0 = lv.conserved_quantity(80.0, 20.0);
        let v_final = lv.conserved_quantity(
            traj.last().expect("conserved_quantity should succeed").0,
            traj.last().expect("last should succeed").1,
        );
        assert!(
            (v0 - v_final).abs() < 1.0,
            "Conserved quantity should be stable"
        );
    }
    #[test]
    fn test_lotka_volterra_coexistence_equilibrium() {
        let lv = LotkaVolterraSimulation::new(1.0, 0.1, 0.5, 0.02);
        let (x_eq, y_eq) = lv.coexistence_equilibrium();
        let (dx, dy) = lv.derivatives(x_eq, y_eq);
        assert!(dx.abs() < 1e-10, "dx/dt at equilibrium should be 0");
        assert!(dy.abs() < 1e-10, "dy/dt at equilibrium should be 0");
    }
    #[test]
    fn test_michaelis_menten_velocity_bounds() {
        let mm = MichaelisMentenKinetics::new(10.0, 2.0);
        assert_eq!(mm.velocity(0.0), 0.0);
        let v_km = mm.velocity(mm.km);
        assert!((v_km - mm.v_max / 2.0).abs() < 1e-10);
        let v_large = mm.velocity(1e9);
        assert!(v_large < mm.v_max);
        assert!(v_large > 0.99 * mm.v_max);
    }
    #[test]
    fn test_michaelis_menten_steady_state() {
        let mm = MichaelisMentenKinetics::new(10.0, 2.0);
        let p = 5.0;
        let s_ss = mm
            .steady_state_substrate(p)
            .expect("steady state should exist");
        let v = mm.velocity(s_ss);
        assert!(
            (v - p).abs() < 1e-8,
            "Steady-state rate {v} should equal production {p}"
        );
        assert!(mm.steady_state_substrate(10.0).is_none());
        assert!(mm.steady_state_substrate(11.0).is_none());
    }
    #[test]
    fn test_boolean_gene_network_fixed_point() {
        let mut net = BooleanGeneNetwork::new(2);
        net.add_edge(0, 1, false);
        net.add_edge(1, 0, false);
        net.set_threshold(0, -1);
        net.set_threshold(1, -1);
        let s = vec![true, false];
        let next = net.update(&s);
        assert_eq!(next.len(), 2);
    }
    #[test]
    fn test_boolean_gene_network_attractors() {
        let mut net = BooleanGeneNetwork::new(1);
        net.add_edge(0, 0, true);
        net.set_threshold(0, 1);
        let attractors = net.find_attractors();
        assert_eq!(attractors.len(), 2);
        for a in &attractors {
            assert_eq!(a.period, 1, "Should be fixed points");
        }
    }
    #[test]
    fn test_gillespie_algorithm_struct() {
        let mut net = ReactionNetwork::new(vec!["A".into()]);
        net.add_reaction(ChemReaction {
            name: "production".into(),
            reactants: vec![],
            products: vec![(0, 1)],
            rate_constant: 2.0,
        });
        net.add_reaction(ChemReaction {
            name: "degradation".into(),
            reactants: vec![(0, 1)],
            products: vec![],
            rate_constant: 0.5,
        });
        let algo = GillespieAlgorithm::new(net, 20.0, 50000, 123);
        let traj = algo.run(vec![0]);
        assert!(!traj.times.is_empty());
        let means = algo.estimate_mean(vec![0], 20.0, 50);
        assert!(!means.is_empty());
        assert!(
            means[0] > 0.5 && means[0] < 20.0,
            "Mean A ≈ 4, got {}",
            means[0]
        );
    }
    #[test]
    fn test_new_axioms_registered() {
        let mut env = Environment::new();
        build_systems_biology_env(&mut env).expect("build should succeed");
        let check = [
            "sir_epidemic_threshold",
            "turing_instability",
            "g1s_checkpoint",
            "goodwin_oscillation",
            "repressilator_oscillation",
            "gene_expression_noise",
            "mm_saturation",
            "lv_conservative_orbits",
        ];
        for name in check {
            assert!(
                env.get(&oxilean_kernel::Name::str(name)).is_some(),
                "Axiom '{name}' should be registered"
            );
        }
    }
}
