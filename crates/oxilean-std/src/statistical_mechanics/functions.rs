//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    CanonicalEnsemble, CorrelationFunction, CriticalExponentTable, Ensemble,
    GrandCanonicalEnsemble, IdealGas, IsingModel, IsingModel1D, LandauFreeEnergy, MeanFieldIsing,
    RenormalizationGroup, VanDerWaalsGas, VirialGas,
};

pub fn app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
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
pub fn int_ty() -> Expr {
    cst("Int")
}
/// Microstate type: an individual configuration of the system
pub fn microstate_ty() -> Expr {
    type0()
}
/// Partition function type: Z = Σ exp(-βE_i)
pub fn partition_function_ty() -> Expr {
    arrow(arrow(type0(), real_ty()), real_ty())
}
/// Boltzmann distribution type: p_i = exp(-βE_i)/Z
pub fn boltzmann_distribution_ty() -> Expr {
    arrow(type0(), arrow(type0(), real_ty()))
}
/// Thermodynamic entropy type: S = -k_B Σ p_i log p_i
pub fn thermodynamic_entropy_ty() -> Expr {
    arrow(arrow(type0(), real_ty()), real_ty())
}
/// Helmholtz free energy type: F = -k_B T log Z
pub fn free_energy_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), real_ty()))
}
/// Boltzmann H-theorem: entropy is non-decreasing in time
pub fn boltzmann_h_theorem_ty() -> Expr {
    arrow(type0(), prop())
}
/// Equipartition theorem: each quadratic degree of freedom contributes ½ k_B T
pub fn equipartition_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), real_ty()))
}
/// Liouville's theorem: phase space volume is preserved under Hamiltonian flow
pub fn liouville_theorem_ty() -> Expr {
    arrow(type0(), prop())
}
/// Fluctuation theorem (detailed balance): related to second law at microscale
pub fn fluctuation_theorem_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// Microcanonical entropy: S = k_B log Ω(E) where Ω is the density of states
/// Type: Nat → Real (number of microstates → entropy value)
pub fn microcanonical_entropy_ty() -> Expr {
    arrow(nat_ty(), real_ty())
}
/// Density of states: Ω(E) — number of microstates with energy E
/// Type: Real → Nat
pub fn density_of_states_ty() -> Expr {
    arrow(real_ty(), nat_ty())
}
/// Microcanonical temperature: 1/T = ∂S/∂E at constant V,N
/// Type: (Real → Real) → Real → Real  (entropy function, energy → temperature)
pub fn microcanonical_temperature_ty() -> Expr {
    arrow(arrow(real_ty(), real_ty()), arrow(real_ty(), real_ty()))
}
/// Canonical free energy: F = -k_B T ln Z(β)
/// Type: Real → Real → Real  (β, Z → F)
pub fn canonical_free_energy_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), real_ty()))
}
/// Canonical partition function with continuous spectrum: Z = ∫ ρ(E) exp(-βE) dE
/// Type: (Real → Real) → Real → Real  (density of states, β → Z)
pub fn canonical_partition_continuous_ty() -> Expr {
    arrow(arrow(real_ty(), real_ty()), arrow(real_ty(), real_ty()))
}
/// Grand potential: Ω = -k_B T ln Ξ where Ξ is the grand partition function
/// Type: Real → Real → Real  (T, Ξ → grand potential)
pub fn grand_potential_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), real_ty()))
}
/// Grand partition function: Ξ = Σ_{N,i} exp(-β(E_i - μN))
/// Type: Real → Real → Real  (β, μ → Ξ)
pub fn grand_partition_function_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), real_ty()))
}
/// Average particle number in grand canonical: ⟨N⟩ = -∂Ω/∂μ
/// Type: (Real → Real) → Real → Real  (grand potential fn, μ → ⟨N⟩)
pub fn grand_canonical_mean_number_ty() -> Expr {
    arrow(arrow(real_ty(), real_ty()), arrow(real_ty(), real_ty()))
}
/// First-order phase transition predicate: discontinuous order parameter
/// Type: Type → Prop
pub fn first_order_phase_transition_ty() -> Expr {
    arrow(type0(), prop())
}
/// Second-order (continuous) phase transition predicate
/// Type: Type → Prop
pub fn second_order_phase_transition_ty() -> Expr {
    arrow(type0(), prop())
}
/// Critical exponent α (heat capacity divergence): C ~ |t|^{-α}
/// Type: Real (just the exponent value as a real number axiom)
pub fn critical_exponent_alpha_ty() -> Expr {
    real_ty()
}
/// Critical exponent β (order parameter): m ~ |t|^β
pub fn critical_exponent_beta_ty() -> Expr {
    real_ty()
}
/// Critical exponent γ (susceptibility): χ ~ |t|^{-γ}
pub fn critical_exponent_gamma_ty() -> Expr {
    real_ty()
}
/// Critical exponent δ (critical isotherm): h ~ m^δ at T=T_c
pub fn critical_exponent_delta_ty() -> Expr {
    real_ty()
}
/// Critical exponent ν (correlation length): ξ ~ |t|^{-ν}
pub fn critical_exponent_nu_ty() -> Expr {
    real_ty()
}
/// Critical exponent η (anomalous dimension of correlations)
pub fn critical_exponent_eta_ty() -> Expr {
    real_ty()
}
/// Scaling hypothesis: free energy near critical point obeys generalized homogeneity
/// Type: Prop
pub fn scaling_hypothesis_ty() -> Expr {
    prop()
}
/// Widom scaling relation: γ = β(δ - 1)
/// Type: Prop
pub fn widom_scaling_relation_ty() -> Expr {
    prop()
}
/// Rushbrooke scaling relation: α + 2β + γ = 2
/// Type: Prop
pub fn rushbrooke_scaling_relation_ty() -> Expr {
    prop()
}
/// 1D Ising exact partition function: Z_N = (2 cosh(βJ))^N in zero field
/// Type: Nat → Real → Real  (N sites, β → Z)
pub fn ising_1d_partition_function_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), real_ty()))
}
/// 1D Ising free energy per site: f = -k_BT ln(2 cosh(βJ))
/// Type: Real → Real  (β → f)
pub fn ising_1d_free_energy_ty() -> Expr {
    arrow(real_ty(), real_ty())
}
/// 2D Ising critical temperature: k_B T_c / J = 2 / ln(1+√2) (Onsager)
/// Type: Real
pub fn ising_2d_critical_temp_ty() -> Expr {
    real_ty()
}
/// Onsager exact solution for 2D Ising free energy
/// Type: Real → Real  (β → f)
pub fn onsager_free_energy_ty() -> Expr {
    arrow(real_ty(), real_ty())
}
/// Weiss molecular field: effective field h_eff = h + zJm (z=coordination number)
/// Type: Real → Real → Real → Real  (h, z*J, m → h_eff)
pub fn weiss_molecular_field_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), real_ty())))
}
/// Mean field self-consistency equation: m = tanh(β(h + zJm))
/// Type: Real → Real → Real → Real → Real  (β, h, z, J → m_selfconsistent)
pub fn mean_field_self_consistency_ty() -> Expr {
    arrow(
        real_ty(),
        arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), real_ty()))),
    )
}
/// Mean field critical temperature: k_B T_c = z J
/// Type: Real → Real  (z*J → T_c)
pub fn mean_field_critical_temp_ty() -> Expr {
    arrow(real_ty(), real_ty())
}
/// Landau free energy density: f(m) = a(T-T_c)m² + bm⁴ + ... (second-order)
/// Type: Real → Real → Real  (m, t=(T-Tc)/Tc → f)
pub fn landau_free_energy_density_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), real_ty()))
}
/// Landau order parameter below T_c: m = ±√(-a t / 2b)
/// Type: Real → Real → Real  (t, ratio a/b → m)
pub fn landau_order_parameter_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), real_ty()))
}
/// Ginzburg-Landau gradient term: |∇ψ|² in free energy functional
/// Type: (Real → Real) → Real  (order parameter field → gradient energy)
pub fn ginzburg_landau_gradient_ty() -> Expr {
    arrow(arrow(real_ty(), real_ty()), real_ty())
}
/// RG flow: transformation on coupling constants under coarse-graining
/// Type: (Real → Real) → Real → Real  (coupling space, scale → new coupling)
pub fn rg_flow_ty() -> Expr {
    arrow(arrow(real_ty(), real_ty()), arrow(real_ty(), real_ty()))
}
/// Fixed point of RG transformation: g* such that R(g*) = g*
/// Type: (Real → Real) → Real → Prop
pub fn rg_fixed_point_ty() -> Expr {
    arrow(arrow(real_ty(), real_ty()), arrow(real_ty(), prop()))
}
/// Universality class: set of systems sharing the same critical exponents
/// Type: Type → Type → Prop  (two systems are in the same universality class)
pub fn same_universality_class_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// Renormalization group relevance: operator dimension determines relevance
/// Type: Real → Bool  (scaling dimension → relevant/irrelevant)
pub fn rg_relevance_ty() -> Expr {
    arrow(real_ty(), bool_ty())
}
/// Fluctuation-dissipation theorem: Im χ(ω) = (πω/k_BT) S(ω)
/// Type: Prop
pub fn fluctuation_dissipation_theorem_ty() -> Expr {
    prop()
}
/// Kubo formula: linear response function as time-integral of correlator
/// Type: (Real → Real) → Real → Real  (correlation function, time → response)
pub fn kubo_formula_ty() -> Expr {
    arrow(arrow(real_ty(), real_ty()), arrow(real_ty(), real_ty()))
}
/// Susceptibility: χ = ∂⟨m⟩/∂h at constant T
/// Type: Type → Real  (ensemble → susceptibility)
pub fn susceptibility_ty() -> Expr {
    arrow(type0(), real_ty())
}
/// Onsager reciprocal relations: L_{ij} = L_{ji} for transport coefficients
/// Type: Prop
pub fn onsager_reciprocal_relations_ty() -> Expr {
    prop()
}
/// Density matrix: ρ = Σ p_i |ψ_i⟩⟨ψ_i| (mixed state)
/// Type: Type → Type  (Hilbert space → density matrix type)
pub fn density_matrix_ty() -> Expr {
    arrow(type0(), type0())
}
/// Von Neumann entropy: S = -k_B Tr(ρ ln ρ)
/// Type: Type → Real  (density matrix → entropy)
pub fn von_neumann_entropy_ty() -> Expr {
    arrow(type0(), real_ty())
}
/// Quantum partition function: Z = Tr(exp(-β H))
/// Type: Type → Real → Real  (Hilbert space, β → Z)
pub fn quantum_partition_function_ty() -> Expr {
    arrow(type0(), arrow(real_ty(), real_ty()))
}
/// Bose-Einstein distribution: n_i = 1/(exp(β(ε_i - μ)) - 1)
/// Type: Real → Real → Real  (β(ε-μ) value → occupation number)
pub fn bose_einstein_distribution_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), real_ty()))
}
/// Bose-Einstein condensate predicate: macroscopic ground-state occupation
/// Type: Type → Real → Prop  (system, T → BEC occurs)
pub fn bose_einstein_condensation_ty() -> Expr {
    arrow(type0(), arrow(real_ty(), prop()))
}
/// BEC critical temperature: T_c = (2πℏ²/m k_B) (n/ζ(3/2))^(2/3)
/// Type: Real → Real → Real  (density n, mass m → T_c)
pub fn bec_critical_temperature_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), real_ty()))
}
/// Fermi-Dirac distribution: n_i = 1/(exp(β(ε_i - μ)) + 1)
/// Type: Real → Real → Real  (β(ε-μ) → occupation number)
pub fn fermi_dirac_distribution_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), real_ty()))
}
/// Fermi energy: energy of highest occupied state at T=0
/// Type: Real → Real → Real  (density n, mass m → E_F)
pub fn fermi_energy_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), real_ty()))
}
/// Fermi surface: constant-energy surface in k-space at E = E_F
/// Type: Type → Type  (fermionic system → Fermi surface)
pub fn fermi_surface_ty() -> Expr {
    arrow(type0(), type0())
}
/// Sommerfeld expansion: low-T expansion of Fermi integrals
/// Type: (Real → Real) → Real → Real → Real  (integrand, E_F, T → integral)
pub fn sommerfeld_expansion_ty() -> Expr {
    arrow(
        arrow(real_ty(), real_ty()),
        arrow(real_ty(), arrow(real_ty(), real_ty())),
    )
}
/// Thermodynamic limit: free energy per particle converges as N → ∞
/// Type: (Nat → Real) → Prop  (sequence of free energies → limit exists)
pub fn thermodynamic_limit_ty() -> Expr {
    arrow(arrow(nat_ty(), real_ty()), prop())
}
/// Lee-Yang theorem: zeros of partition function lie on the unit circle (for Ising)
/// Type: Prop
pub fn lee_yang_theorem_ty() -> Expr {
    prop()
}
/// Lee-Yang zero: complex z with Z(z) = 0 (edge singularity → phase transition)
/// Type: Real → Real → Prop  (Re z, Im z → is a Lee-Yang zero)
pub fn lee_yang_zero_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), prop()))
}
/// Boltzmann transport equation: ∂f/∂t + v·∇f + F·∇_v f = (∂f/∂t)_coll
/// Type: (Real → Real) → Prop  (distribution function f → satisfies BTE)
pub fn boltzmann_transport_equation_ty() -> Expr {
    arrow(arrow(real_ty(), real_ty()), prop())
}
/// Collision integral: (∂f/∂t)_coll for a scattering kernel
/// Type: (Real → Real) → (Real → Real → Real) → Real → Real
///       (distribution, scattering kernel, momentum → collision rate)
pub fn collision_integral_ty() -> Expr {
    arrow(
        arrow(real_ty(), real_ty()),
        arrow(
            arrow(real_ty(), arrow(real_ty(), real_ty())),
            arrow(real_ty(), real_ty()),
        ),
    )
}
/// Relaxation time approximation: (∂f/∂t)_coll ≈ -(f - f_0)/τ
/// Type: Real → (Real → Real) → (Real → Real) → Real → Real
///       (τ, f, f_0, v → collision term)
pub fn relaxation_time_approx_ty() -> Expr {
    arrow(
        real_ty(),
        arrow(
            arrow(real_ty(), real_ty()),
            arrow(arrow(real_ty(), real_ty()), arrow(real_ty(), real_ty())),
        ),
    )
}
/// Entropy production rate: σ = -dH/dt ≥ 0 (H-theorem consequence)
/// Type: (Real → Real) → Real  (distribution function → σ)
pub fn entropy_production_rate_ty() -> Expr {
    arrow(arrow(real_ty(), real_ty()), real_ty())
}
/// Register all statistical mechanics axioms into the environment.
pub fn build_statistical_mechanics_env(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("Microstate", microstate_ty()),
        ("PartitionFunction", partition_function_ty()),
        ("BoltzmannDist", boltzmann_distribution_ty()),
        ("ThermodynamicEntropy", thermodynamic_entropy_ty()),
        ("HelmholtzFreeEnergy", free_energy_ty()),
        ("BoltzmannConstant", real_ty()),
        ("AvogadroNumber", real_ty()),
        ("PlanckConstant", real_ty()),
        ("SpeedOfLight", real_ty()),
        ("CanonicalEnsemble", arrow(real_ty(), type0())),
        ("MicrocanonicalEnsemble", arrow(real_ty(), type0())),
        (
            "GrandCanonicalEnsemble",
            arrow(real_ty(), arrow(real_ty(), type0())),
        ),
        ("MeanEnergy", arrow(type0(), real_ty())),
        ("HeatCapacity", arrow(type0(), real_ty())),
        ("ChemicalPotential", arrow(type0(), real_ty())),
        ("Pressure", arrow(type0(), real_ty())),
        ("Temperature", arrow(type0(), real_ty())),
        ("boltzmann_h_theorem", boltzmann_h_theorem_ty()),
        ("equipartition", equipartition_ty()),
        ("liouville_theorem", liouville_theorem_ty()),
        ("fluctuation_theorem", fluctuation_theorem_ty()),
        ("IsEquilibrium", arrow(type0(), prop())),
        ("SatisfiesDetailedBalance", arrow(type0(), prop())),
        ("IsErgodic", arrow(type0(), prop())),
        ("GibbsParadox", prop()),
        (
            "MaxwellBoltzmannDist",
            arrow(real_ty(), arrow(real_ty(), real_ty())),
        ),
        (
            "FermiDiracDist",
            arrow(real_ty(), arrow(real_ty(), real_ty())),
        ),
        (
            "BoseEinsteinDist",
            arrow(real_ty(), arrow(real_ty(), real_ty())),
        ),
        ("IdealGasLaw", arrow(real_ty(), arrow(real_ty(), prop()))),
        ("IsingHamiltonian", arrow(type0(), real_ty())),
        ("CriticalTemperature", arrow(real_ty(), real_ty())),
        ("OrderParameter", arrow(type0(), real_ty())),
        ("MicrocanonicalEntropy", microcanonical_entropy_ty()),
        ("DensityOfStates", density_of_states_ty()),
        ("MicrocanonicalTemperature", microcanonical_temperature_ty()),
        ("CanonicalFreeEnergy", canonical_free_energy_ty()),
        (
            "CanonicalPartitionContinuous",
            canonical_partition_continuous_ty(),
        ),
        ("GrandPotential", grand_potential_ty()),
        ("GrandPartitionFunction", grand_partition_function_ty()),
        ("GrandCanonicalMeanNumber", grand_canonical_mean_number_ty()),
        (
            "FirstOrderPhaseTransition",
            first_order_phase_transition_ty(),
        ),
        (
            "SecondOrderPhaseTransition",
            second_order_phase_transition_ty(),
        ),
        ("CriticalExponentAlpha", critical_exponent_alpha_ty()),
        ("CriticalExponentBeta", critical_exponent_beta_ty()),
        ("CriticalExponentGamma", critical_exponent_gamma_ty()),
        ("CriticalExponentDelta", critical_exponent_delta_ty()),
        ("CriticalExponentNu", critical_exponent_nu_ty()),
        ("CriticalExponentEta", critical_exponent_eta_ty()),
        ("ScalingHypothesis", scaling_hypothesis_ty()),
        ("WidomScalingRelation", widom_scaling_relation_ty()),
        (
            "RushbrookeScalingRelation",
            rushbrooke_scaling_relation_ty(),
        ),
        ("Ising1DPartitionFunction", ising_1d_partition_function_ty()),
        ("Ising1DFreeEnergy", ising_1d_free_energy_ty()),
        ("Ising2DCriticalTemp", ising_2d_critical_temp_ty()),
        ("OnsagerFreeEnergy", onsager_free_energy_ty()),
        ("WeissMolecularField", weiss_molecular_field_ty()),
        ("MeanFieldSelfConsistency", mean_field_self_consistency_ty()),
        ("MeanFieldCriticalTemp", mean_field_critical_temp_ty()),
        ("LandauFreeEnergyDensity", landau_free_energy_density_ty()),
        ("LandauOrderParameter", landau_order_parameter_ty()),
        ("GinzburgLandauGradient", ginzburg_landau_gradient_ty()),
        ("RGFlow", rg_flow_ty()),
        ("RGFixedPoint", rg_fixed_point_ty()),
        ("SameUniversalityClass", same_universality_class_ty()),
        ("RGRelevance", rg_relevance_ty()),
        (
            "FluctuationDissipationTheorem",
            fluctuation_dissipation_theorem_ty(),
        ),
        ("KuboFormula", kubo_formula_ty()),
        ("Susceptibility", susceptibility_ty()),
        (
            "OnsagerReciprocalRelations",
            onsager_reciprocal_relations_ty(),
        ),
        ("DensityMatrix", density_matrix_ty()),
        ("VonNeumannEntropy", von_neumann_entropy_ty()),
        ("QuantumPartitionFunction", quantum_partition_function_ty()),
        ("BoseEinsteinDistribution", bose_einstein_distribution_ty()),
        ("BoseEinsteinCondensation", bose_einstein_condensation_ty()),
        ("BECCriticalTemperature", bec_critical_temperature_ty()),
        ("FermiDiracDistribution", fermi_dirac_distribution_ty()),
        ("FermiEnergy", fermi_energy_ty()),
        ("FermiSurface", fermi_surface_ty()),
        ("SommerfeldExpansion", sommerfeld_expansion_ty()),
        ("ThermodynamicLimit", thermodynamic_limit_ty()),
        ("LeeYangTheorem", lee_yang_theorem_ty()),
        ("LeeYangZero", lee_yang_zero_ty()),
        (
            "BoltzmannTransportEquation",
            boltzmann_transport_equation_ty(),
        ),
        ("CollisionIntegral", collision_integral_ty()),
        ("RelaxationTimeApprox", relaxation_time_approx_ty()),
        ("EntropyProductionRate", entropy_production_rate_ty()),
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
/// Boltzmann constant k_B in J/K
pub const BOLTZMANN_K: f64 = 1.380649e-23;
/// Avogadro's number in mol^-1
pub const AVOGADRO: f64 = 6.02214076e23;
/// Planck constant h in J·s
pub const PLANCK_H: f64 = 6.62607015e-34;
/// RG fixed-point stability: eigenvalues of the linearized RG around g*
/// Type: (Real → Real) → Real → Real  (RG map, coupling → eigenvalue)
pub fn rg_stability_eigenvalue_ty() -> Expr {
    arrow(arrow(real_ty(), real_ty()), arrow(real_ty(), real_ty()))
}
/// Scaling field: linear combination of coupling constants transforming simply under RG
/// Type: Real → Real → Real  (coupling constant, scale factor → scaling field)
pub fn rg_scaling_field_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), real_ty()))
}
/// RG beta function: β(g) = dg/d(ln μ) — running of coupling with scale
/// Type: (Real → Real) → Real  (coupling function → beta function value)
pub fn rg_beta_function_ty() -> Expr {
    arrow(arrow(real_ty(), real_ty()), real_ty())
}
/// Anomalous dimension: η = -d(ln Z)/d(ln μ) where Z is the field renormalization
/// Type: Real → Real  (coupling → anomalous dimension)
pub fn sm_ext_anomalous_dimension_ty() -> Expr {
    arrow(real_ty(), real_ty())
}
/// Hyperscaling relation: dν = 2 - α (valid below upper critical dimension)
/// Type: Prop
pub fn hyperscaling_relation_ty() -> Expr {
    prop()
}
/// Fisher scaling relation: γ = ν(2 - η)
/// Type: Prop
pub fn fisher_scaling_relation_ty() -> Expr {
    prop()
}
/// Josephson scaling relation: dν = 2 - α (same as hyperscaling)
/// Type: Prop
pub fn josephson_scaling_relation_ty() -> Expr {
    prop()
}
/// Upper critical dimension: d_c above which mean-field exponents hold
/// Type: Real  (dimension value)
pub fn upper_critical_dimension_ty() -> Expr {
    real_ty()
}
/// Epsilon expansion: perturbative RG in ε = d_c - d
/// Type: Real → Real  (ε → correction to exponent)
pub fn epsilon_expansion_ty() -> Expr {
    arrow(real_ty(), real_ty())
}
/// Conformal symmetry generator: central charge c in Virasoro algebra
/// Type: Real  (central charge c)
pub fn virasoro_central_charge_ty() -> Expr {
    real_ty()
}
/// Virasoro algebra: \[L_m, L_n\] = (m-n) L_{m+n} + c/12 m(m²-1) δ_{m+n,0}
/// Type: Prop
pub fn virasoro_algebra_ty() -> Expr {
    prop()
}
/// Conformal weight: (h, h̄) labeling a primary field
/// Type: Real → Real → Type  (h, h-bar → primary field type)
pub fn conformal_weight_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), type0()))
}
/// Operator product expansion: OPE coefficients C_{ij}^k
/// Type: Real → Real → Real → Real  (conformal weight i, j, k → OPE coefficient)
pub fn ope_coefficient_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), real_ty())))
}
/// Kac determinant: formula for null states in Verma module
/// Type: Real → Real → Real  (central charge, conformal weight → Kac determinant)
pub fn kac_determinant_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), real_ty()))
}
/// Minimal model: (p,q) minimal CFT with central charge c = 1 - 6(p-q)²/(pq)
/// Type: Nat → Nat → Real  (p, q → central charge)
pub fn minimal_model_central_charge_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), real_ty()))
}
/// Transfer matrix: 2×2 matrix encoding the 1D Ising Boltzmann weights
/// Type: Real → Real → Type  (β*J, β*h → transfer matrix type)
pub fn transfer_matrix_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), type0()))
}
/// Onsager exact free energy: f = -k_BT\[ln(2) + (1/2π)∫ ln(cosh²2βJ - sinh2βJ cos k) dk\]
/// Type: Real → Real  (β → free energy per site)
pub fn onsager_exact_free_energy_ty() -> Expr {
    arrow(real_ty(), real_ty())
}
/// Onsager magnetization: m = (1 - sinh^{-4}(2βJ))^{1/8}
/// Type: Real → Real  (β → magnetization)
pub fn onsager_magnetization_ty() -> Expr {
    arrow(real_ty(), real_ty())
}
/// Yang-Lee edge singularity: singularity in complex h plane at imaginary field
/// Type: Real → Prop  (β → edge singularity exists)
pub fn yang_lee_edge_singularity_ty() -> Expr {
    arrow(real_ty(), prop())
}
/// Lee-Yang circle theorem: partition function zeros on unit circle |z| = 1
/// Type: (Real → Real) → Prop  (Hamiltonian → zeros on unit circle)
pub fn lee_yang_circle_theorem_ty() -> Expr {
    arrow(arrow(real_ty(), real_ty()), prop())
}
/// Peierls argument: phase transition in 2D Ising at low temperature
/// Type: Real → Prop  (β → ordered phase exists)
pub fn peierls_argument_ty() -> Expr {
    arrow(real_ty(), prop())
}
/// Griffiths inequalities: correlation functions are non-negative
/// Type: Prop
pub fn griffiths_inequalities_ty() -> Expr {
    prop()
}
/// FKG inequality: monotone events are positively correlated
/// Type: Prop
pub fn fkg_inequality_ty() -> Expr {
    prop()
}
/// Pirogov-Sinai theory: rigorous phase diagram for low-temperature models
/// Type: Type → Prop  (lattice model → PS theory applies)
pub fn pirogov_sinai_theory_ty() -> Expr {
    arrow(type0(), prop())
}
/// Contour model: excitations described as contours above ground state
/// Type: Type → Type  (lattice model → contour type)
pub fn contour_model_ty() -> Expr {
    arrow(type0(), type0())
}
/// Ground state: configuration minimizing the Hamiltonian
/// Type: Type → Type  (configuration space → ground state type)
pub fn ground_state_ty() -> Expr {
    arrow(type0(), type0())
}
/// Mayer f-function: f(r) = exp(-βu(r)) - 1 for pair potential u(r)
/// Type: (Real → Real) → Real → Real  (pair potential, r → Mayer f)
pub fn mayer_f_function_ty() -> Expr {
    arrow(arrow(real_ty(), real_ty()), arrow(real_ty(), real_ty()))
}
/// Cluster integral: b_n = (1/n!) ∫ ∏ f(r_{ij}) dr₁...drₙ
/// Type: Nat → (Real → Real) → Real  (cluster size, pair potential → cluster integral)
pub fn cluster_integral_ty() -> Expr {
    arrow(nat_ty(), arrow(arrow(real_ty(), real_ty()), real_ty()))
}
/// Virial expansion: P/k_BT = ρ + B₂(T)ρ² + B₃(T)ρ³ + ...
/// Type: Real → Real → Real  (density ρ, temperature T → pressure/k_BT)
pub fn virial_expansion_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), real_ty()))
}
/// Second virial coefficient: B₂(T) = -½ ∫ f(r) 4πr² dr
/// Type: (Real → Real) → Real → Real  (pair potential, T → B₂)
pub fn second_virial_coefficient_ty() -> Expr {
    arrow(arrow(real_ty(), real_ty()), arrow(real_ty(), real_ty()))
}
/// Van der Waals equation: (P + a/V²)(V - b) = N k_B T
/// Type: Real → Real → Real → Real → Real  (P, V, N, T → VdW residual)
pub fn van_der_waals_equation_ty() -> Expr {
    arrow(
        real_ty(),
        arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), real_ty()))),
    )
}
/// Van der Waals critical point: T_c = 8a/(27Rb), P_c = a/(27b²), V_c = 3b
/// Type: Real → Real → Real  (a, b → critical temperature)
pub fn van_der_waals_critical_temp_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), real_ty()))
}
/// Spontaneous symmetry breaking predicate: ground state breaks symmetry of Hamiltonian
/// Type: Type → Type → Prop  (Hamiltonian symmetry group, ground state → SSB)
pub fn spontaneous_symmetry_breaking_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// Goldstone's theorem: SSB of continuous symmetry implies massless Goldstone bosons
/// Type: Prop
pub fn goldstone_theorem_ty() -> Expr {
    prop()
}
/// Number of Goldstone bosons: = dimension of broken symmetry generators
/// Type: Nat → Nat → Nat  (dim of symmetry group, dim of residual group → # Goldstone)
pub fn goldstone_boson_count_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), nat_ty()))
}
/// Mermin-Wagner theorem: no spontaneous breaking of continuous symmetry in d≤2
/// Type: Prop
pub fn mermin_wagner_theorem_ty() -> Expr {
    prop()
}
/// Higgs mechanism: Goldstone boson absorbed by gauge boson to give it mass
/// Type: Type → Type → Prop  (gauge field, Higgs field → mechanism applies)
pub fn higgs_mechanism_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// Maxwell relation: (∂T/∂V)_S = -(∂P/∂S)_V
/// Type: Prop
pub fn maxwell_relation_tv_ty() -> Expr {
    prop()
}
/// Maxwell relation: (∂T/∂P)_S = (∂V/∂S)_P
/// Type: Prop
pub fn maxwell_relation_tp_ty() -> Expr {
    prop()
}
/// Maxwell relation: (∂S/∂V)_T = (∂P/∂T)_V
/// Type: Prop
pub fn maxwell_relation_sv_ty() -> Expr {
    prop()
}
/// Clausius-Clapeyron equation: dP/dT = L/(T ΔV) for phase boundary slope
/// Type: Real → Real → Real → Real  (L, T, ΔV → dP/dT)
pub fn clausius_clapeyron_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), real_ty())))
}
/// Gibbs-Duhem relation: SdT - VdP + Ndμ = 0
/// Type: Prop
pub fn gibbs_duhem_relation_ty() -> Expr {
    prop()
}
/// Wigner distribution function: quasi-probability distribution in phase space
/// Type: (Real → Real → Real) → Real → Real → Real  (wave function, x, p → W(x,p))
pub fn wigner_distribution_ty() -> Expr {
    arrow(
        arrow(real_ty(), arrow(real_ty(), real_ty())),
        arrow(real_ty(), arrow(real_ty(), real_ty())),
    )
}
/// Husimi Q function: coherent-state phase space representation
/// Type: Type → Real → Real → Real  (density matrix, q, p → Q(q,p))
pub fn husimi_q_function_ty() -> Expr {
    arrow(type0(), arrow(real_ty(), arrow(real_ty(), real_ty())))
}
/// Quantum ergodicity: energy eigenstates equidistribute on energy shell
/// Type: Type → Prop  (quantum system → quantum ergodicity holds)
pub fn quantum_ergodicity_ty() -> Expr {
    arrow(type0(), prop())
}
/// Eigenstate thermalization hypothesis (ETH): expectation values in energy eigenstates
/// agree with microcanonical ensemble predictions
/// Type: Type → Prop  (system → ETH holds)
pub fn eigenstate_thermalization_hypothesis_ty() -> Expr {
    arrow(type0(), prop())
}
/// Fokker-Planck equation: ∂P/∂t = -∂(FP)/∂x + D ∂²P/∂x²
/// Type: (Real → Real → Real) → Prop  (probability density P(x,t) → satisfies FP)
pub fn fokker_planck_equation_ty() -> Expr {
    arrow(arrow(real_ty(), arrow(real_ty(), real_ty())), prop())
}
/// Langevin equation: m ẍ = -γẋ + F(x) + ξ(t) (ξ = white noise)
/// Type: (Real → Real) → Real → Prop  (force function, friction γ → Langevin system)
pub fn langevin_equation_ty() -> Expr {
    arrow(arrow(real_ty(), real_ty()), arrow(real_ty(), prop()))
}
/// Detailed balance: π(x) T(x→y) = π(y) T(y→x) (stationarity condition)
/// Type: (Real → Real) → (Real → Real → Real) → Prop
///       (stationary distribution, transition kernel → detailed balance)
pub fn detailed_balance_ty() -> Expr {
    arrow(
        arrow(real_ty(), real_ty()),
        arrow(arrow(real_ty(), arrow(real_ty(), real_ty())), prop()),
    )
}
/// Green-Kubo relations: transport coefficient from correlation function integral
/// Type: (Real → Real) → Real  (correlation function → transport coefficient)
pub fn green_kubo_relation_ty() -> Expr {
    arrow(arrow(real_ty(), real_ty()), real_ty())
}
/// Register all extended statistical mechanics axioms into the environment.
pub fn register_statistical_mechanics_extended(env: &mut Environment) -> Result<(), String> {
    let axioms: &[(&str, Expr)] = &[
        ("RGStabilityEigenvalue", rg_stability_eigenvalue_ty()),
        ("RGScalingField", rg_scaling_field_ty()),
        ("RGBetaFunction", rg_beta_function_ty()),
        ("AnomalousDimension", sm_ext_anomalous_dimension_ty()),
        ("HyperscalingRelation", hyperscaling_relation_ty()),
        ("FisherScalingRelation", fisher_scaling_relation_ty()),
        ("JosephsonScalingRelation", josephson_scaling_relation_ty()),
        ("UpperCriticalDimension", upper_critical_dimension_ty()),
        ("EpsilonExpansion", epsilon_expansion_ty()),
        ("VirasoroCentralCharge", virasoro_central_charge_ty()),
        ("VirasoroAlgebra", virasoro_algebra_ty()),
        ("ConformalWeight", conformal_weight_ty()),
        ("OPECoefficient", ope_coefficient_ty()),
        ("KacDeterminant", kac_determinant_ty()),
        (
            "MinimalModelCentralCharge",
            minimal_model_central_charge_ty(),
        ),
        ("TransferMatrix", transfer_matrix_ty()),
        ("OnsagerExactFreeEnergy", onsager_exact_free_energy_ty()),
        ("OnsagerMagnetization", onsager_magnetization_ty()),
        ("YangLeeEdgeSingularity", yang_lee_edge_singularity_ty()),
        ("LeeYangCircleTheorem", lee_yang_circle_theorem_ty()),
        ("PeierlsArgument", peierls_argument_ty()),
        ("GriffithsInequalities", griffiths_inequalities_ty()),
        ("FKGInequality", fkg_inequality_ty()),
        ("PirogovSinaiTheory", pirogov_sinai_theory_ty()),
        ("ContourModel", contour_model_ty()),
        ("GroundState", ground_state_ty()),
        ("MayerFFunction", mayer_f_function_ty()),
        ("ClusterIntegral", cluster_integral_ty()),
        ("VirialExpansion", virial_expansion_ty()),
        ("SecondVirialCoefficient", second_virial_coefficient_ty()),
        ("VanDerWaalsEquation", van_der_waals_equation_ty()),
        ("VanDerWaalsCriticalTemp", van_der_waals_critical_temp_ty()),
        (
            "SpontaneousSymmetryBreaking",
            spontaneous_symmetry_breaking_ty(),
        ),
        ("GoldstoneTheorem", goldstone_theorem_ty()),
        ("GoldstoneBosonCount", goldstone_boson_count_ty()),
        ("MerminWagnerTheorem", mermin_wagner_theorem_ty()),
        ("HiggsMechanism", higgs_mechanism_ty()),
        ("MaxwellRelationTV", maxwell_relation_tv_ty()),
        ("MaxwellRelationTP", maxwell_relation_tp_ty()),
        ("MaxwellRelationSV", maxwell_relation_sv_ty()),
        ("ClausiusClapeyron", clausius_clapeyron_ty()),
        ("GibbsDuhemRelation", gibbs_duhem_relation_ty()),
        ("WignerDistribution", wigner_distribution_ty()),
        ("HusimiQFunction", husimi_q_function_ty()),
        ("QuantumErgodicity", quantum_ergodicity_ty()),
        (
            "EigenstateThermalizationHypothesis",
            eigenstate_thermalization_hypothesis_ty(),
        ),
        ("FokkerPlanckEquation", fokker_planck_equation_ty()),
        ("LangevinEquation", langevin_equation_ty()),
        ("DetailedBalance", detailed_balance_ty()),
        ("GreenKuboRelation", green_kubo_relation_ty()),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .map_err(|e| format!("Failed to add {name}: {e:?}"))?;
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    /// Relative tolerance for floating-point comparisons
    fn rel_close(a: f64, b: f64, tol: f64) -> bool {
        if b.abs() < 1e-300 {
            a.abs() < tol
        } else {
            ((a - b) / b).abs() < tol
        }
    }
    #[test]
    fn test_ensemble_partition_function() {
        let eps = BOLTZMANN_K * 300.0;
        let temp = 300.0;
        let ens = Ensemble::new(vec![0.0, eps], temp);
        let beta = 1.0 / (BOLTZMANN_K * temp);
        let z_expected = 1.0 + (-beta * eps).exp();
        assert!(
            rel_close(ens.partition_function(), z_expected, 1e-10),
            "Z = 1 + exp(-βε): expected {z_expected}, got {}",
            ens.partition_function()
        );
    }
    #[test]
    fn test_boltzmann_probability() {
        let eps = BOLTZMANN_K * 300.0;
        let ens = Ensemble::new(vec![0.0, eps], 300.0);
        let p0 = ens.probability(0);
        let p1 = ens.probability(1);
        assert!(p0 > p1, "ground state more probable: p0={p0}, p1={p1}");
        assert!((p0 + p1 - 1.0).abs() < 1e-12, "probabilities sum to 1");
    }
    #[test]
    fn test_ensemble_mean_energy() {
        let eps = BOLTZMANN_K * 1000.0;
        let ens = Ensemble::new(vec![0.0, eps], 1.0);
        let mean_e = ens.mean_energy();
        assert!(
            mean_e < eps * 1e-10,
            "at very low T, mean energy ≈ 0, got {mean_e}"
        );
    }
    #[test]
    fn test_ensemble_entropy_positive() {
        let eps = BOLTZMANN_K * 300.0;
        let ens = Ensemble::new(vec![0.0, eps], 300.0);
        let s = ens.entropy();
        assert!(s >= 0.0, "entropy is non-negative, got {s}");
    }
    #[test]
    fn test_ideal_gas_pressure() {
        let n = 1_000_000u64;
        let t = 300.0;
        let v = 1e-3;
        let gas = IdealGas::new(n, t, v);
        let p = gas.pressure();
        let p_expected = (n as f64) * BOLTZMANN_K * t / v;
        assert!(
            rel_close(p, p_expected, 1e-10),
            "PV = Nk_BT: expected {p_expected}, got {p}"
        );
    }
    #[test]
    fn test_ideal_gas_mean_energy() {
        let gas = IdealGas::new(1, 300.0, 1.0);
        let e = gas.mean_kinetic_energy();
        let expected = 1.5 * BOLTZMANN_K * 300.0;
        assert!(
            rel_close(e, expected, 1e-10),
            "⟨E⟩ = (3/2)k_BT: expected {expected}, got {e}"
        );
    }
    #[test]
    fn test_ising_model_energy() {
        let mut model = IsingModel {
            spins: vec![vec![1, 1], vec![1, 1]],
            j_coupling: 1.0,
            temperature: 1.0,
        };
        let e = model.energy();
        assert!(
            e < 0.0,
            "ferromagnetic Ising should have negative energy, got {e}"
        );
        model.spins[0][0] = -1;
        let e2 = model.energy();
        assert!(
            e2 > e,
            "flipping one spin in FM increases energy: e={e}, e2={e2}"
        );
    }
    #[test]
    fn test_ensemble_free_energy() {
        let eps = BOLTZMANN_K * 1.0;
        let temp = 1e6;
        let ens = Ensemble::new(vec![0.0, eps], temp);
        let f = ens.free_energy();
        let f_expected = -BOLTZMANN_K * temp * 2.0_f64.ln();
        assert!(
            rel_close(f, f_expected, 1e-3),
            "F ≈ -k_BT ln 2 at high T: expected {f_expected}, got {f}"
        );
    }
    #[test]
    fn test_ising_1d_zero_field_partition() {
        let j = BOLTZMANN_K * 100.0;
        let temp = 300.0;
        let n = 10;
        let model = IsingModel1D::new(n, j, 0.0, temp);
        let z_exact = model.zero_field_partition_function();
        let z_transfer = model.partition_function();
        assert!(
            rel_close(z_exact, z_transfer, 0.01),
            "1D Ising Z: exact={z_exact}, transfer={z_transfer}"
        );
    }
    #[test]
    fn test_ising_1d_free_energy_per_site() {
        let j = BOLTZMANN_K * 100.0;
        let temp = 300.0;
        let model = IsingModel1D::new(100, j, 0.0, temp);
        let f = model.free_energy_per_site();
        let b = 1.0 / (BOLTZMANN_K * temp);
        let f_expected = -BOLTZMANN_K * temp * (2.0 * (b * j).cosh()).ln();
        assert!(
            rel_close(f, f_expected, 1e-4),
            "1D Ising f/site: expected {f_expected}, got {f}"
        );
    }
    #[test]
    fn test_mean_field_critical_temperature() {
        let j = BOLTZMANN_K * 100.0;
        let model = MeanFieldIsing::new(j, 0.0, 4.0, 400.0);
        let tc = model.critical_temperature();
        let tc_expected = 4.0 * j / BOLTZMANN_K;
        assert!(
            rel_close(tc, tc_expected, 1e-10),
            "MF T_c = zJ/k_B: expected {tc_expected}, got {tc}"
        );
    }
    #[test]
    fn test_mean_field_above_tc_paramagnetic() {
        let j = BOLTZMANN_K * 100.0;
        let tc = 4.0 * j / BOLTZMANN_K;
        let model = MeanFieldIsing::new(j, 0.0, 4.0, tc * 1.5);
        let solutions = model.find_all_solutions();
        for &m in &solutions {
            assert!(m.abs() < 0.01, "Above T_c, m should be ~0, got {m}");
        }
    }
    #[test]
    fn test_mean_field_below_tc_symmetry_breaking() {
        let j = BOLTZMANN_K * 100.0;
        let tc = 4.0 * j / BOLTZMANN_K;
        let model = MeanFieldIsing::new(j, 0.0, 4.0, tc * 0.5);
        let solutions = model.find_all_solutions();
        let has_positive = solutions.iter().any(|&m| m > 0.1);
        let has_negative = solutions.iter().any(|&m| m < -0.1);
        assert!(
            has_positive && has_negative,
            "Below T_c, expect ±m solutions, got: {solutions:?}"
        );
    }
    #[test]
    fn test_landau_free_energy_minimum() {
        let lf = LandauFreeEnergy::new_second_order(1.0, 1.0, 0.0);
        let t = -0.5;
        let m_eq = lf.equilibrium_order_parameter(t);
        let m_expected = 0.5_f64;
        assert!(
            rel_close(m_eq.abs(), m_expected, 0.01),
            "Landau m_eq={m_eq}, expected ±{m_expected}"
        );
    }
    #[test]
    fn test_landau_free_energy_above_tc() {
        let lf = LandauFreeEnergy::new_second_order(1.0, 1.0, 0.0);
        let m_eq = lf.equilibrium_order_parameter(0.5);
        assert!(
            m_eq.abs() < 0.01,
            "Above T_c, Landau m_eq should be ~0, got {m_eq}"
        );
    }
    #[test]
    fn test_critical_exponent_table_widom() {
        let table = CriticalExponentTable::standard();
        for entry in &table.entries {
            assert!(
                entry.check_widom(),
                "Widom relation fails for {}: γ={}, β(δ-1)={}",
                entry.name,
                entry.gamma,
                entry.beta * (entry.delta - 1.0)
            );
        }
    }
    #[test]
    fn test_critical_exponent_table_rushbrooke() {
        let table = CriticalExponentTable::standard();
        for entry in &table.entries {
            assert!(
                entry.check_rushbrooke(),
                "Rushbrooke relation fails for {}: α+2β+γ={}",
                entry.name,
                entry.alpha + 2.0 * entry.beta + entry.gamma
            );
        }
    }
    #[test]
    fn test_canonical_ensemble_energy_variance() {
        let eps = BOLTZMANN_K * 300.0;
        let ce = CanonicalEnsemble::new(vec![0.0, eps], 300.0);
        let var = ce.energy_variance();
        assert!(
            var >= 0.0,
            "Energy variance must be non-negative, got {var}"
        );
    }
    #[test]
    fn test_canonical_ensemble_probabilities_sum_to_one() {
        let energies: Vec<f64> = (0..5).map(|i| (i as f64) * BOLTZMANN_K * 100.0).collect();
        let ce = CanonicalEnsemble::new(energies, 300.0);
        let sum: f64 = ce.probabilities().iter().sum();
        assert!(
            (sum - 1.0).abs() < 1e-12,
            "Probabilities must sum to 1, got {sum}"
        );
    }
    #[test]
    fn test_axiom_registration() {
        use oxilean_kernel::Environment;
        let mut env = Environment::new();
        build_statistical_mechanics_env(&mut env);
        let new_names = [
            "MicrocanonicalEntropy",
            "GrandPotential",
            "FirstOrderPhaseTransition",
            "CriticalExponentAlpha",
            "Ising1DPartitionFunction",
            "WeissMolecularField",
            "LandauFreeEnergyDensity",
            "RGFlow",
            "FluctuationDissipationTheorem",
            "DensityMatrix",
            "BoseEinsteinDistribution",
            "FermiDiracDistribution",
            "ThermodynamicLimit",
            "LeeYangTheorem",
            "BoltzmannTransportEquation",
        ];
        for name in &new_names {
            assert!(
                env.get(&Name::str(*name)).is_some(),
                "Axiom '{name}' not found in environment"
            );
        }
    }
    #[test]
    fn test_extended_axiom_registration() {
        use oxilean_kernel::Environment;
        let mut env = Environment::new();
        register_statistical_mechanics_extended(&mut env).expect("Environment::new should succeed");
        let extended_names = [
            "RGStabilityEigenvalue",
            "HyperscalingRelation",
            "VirasoroCentralCharge",
            "VirasoroAlgebra",
            "ConformalWeight",
            "TransferMatrix",
            "OnsagerExactFreeEnergy",
            "PeierlsArgument",
            "GriffithsInequalities",
            "MayerFFunction",
            "VirialExpansion",
            "VanDerWaalsEquation",
            "SpontaneousSymmetryBreaking",
            "GoldstoneTheorem",
            "MerminWagnerTheorem",
            "HiggsMechanism",
            "FokkerPlanckEquation",
            "LangevinEquation",
            "GreenKuboRelation",
        ];
        for name in &extended_names {
            assert!(
                env.get(&Name::str(*name)).is_some(),
                "Extended axiom '{name}' not found"
            );
        }
    }
    #[test]
    fn test_virial_gas_pressure() {
        let b2 = VirialGas::hard_sphere_b2(3e-10);
        let gas = VirialGas::new(b2, 0.0, 300.0);
        let rho = 1e25_f64;
        let p = gas.pressure(rho);
        let p_ideal = BOLTZMANN_K * 300.0 * rho;
        assert!(
            p > p_ideal,
            "virial gas pressure should exceed ideal: p={p}, p_ideal={p_ideal}"
        );
    }
    #[test]
    fn test_van_der_waals_critical_point() {
        let a = 1.355e-48_f64;
        let b = 3.2e-29_f64;
        let gas = VanDerWaalsGas::new(a, b, 150.0);
        let tc = gas.critical_temperature();
        let tc_expected = 8.0 * a / (27.0 * BOLTZMANN_K * b);
        assert!(
            ((tc - tc_expected) / tc_expected).abs() < 1e-10,
            "VdW T_c={tc}, expected {tc_expected}"
        );
        assert!(
            (VanDerWaalsGas::critical_compressibility() - 0.375).abs() < 1e-10,
            "Z_c should be 0.375"
        );
    }
    #[test]
    fn test_van_der_waals_pressure() {
        let a = 1.0e-48_f64;
        let b = 3.0e-29_f64;
        let gas = VanDerWaalsGas::new(a, b, 300.0);
        let v = 1e-27_f64;
        let p = gas.pressure(v);
        assert!(
            p.is_finite() && p > 0.0,
            "VdW pressure should be positive and finite, got {p}"
        );
    }
    #[test]
    fn test_rg_fixed_point_trivial() {
        let rg = RenormalizationGroup::new(1.0, 2.0);
        let (g_star, converged) = rg.find_fixed_point(&|g: f64| g / 2.0, 1e-12, 1000);
        assert!(converged, "RG should converge");
        assert!(g_star.abs() < 1e-6, "Fixed point at 0, got {g_star}");
    }
    #[test]
    fn test_rg_fixed_point_nontrivial() {
        let rg = RenormalizationGroup::new(0.8, 2.0);
        let (g_star, converged) = rg.find_fixed_point(&|g: f64| 2.0 * g - g * g, 1e-8, 2000);
        assert!(converged, "RG should converge near g=1");
        assert!(
            (g_star - 1.0).abs() < 0.01,
            "Fixed point near 1, got {g_star}"
        );
    }
    #[test]
    fn test_grand_canonical_fermi_dirac() {
        let energy_levels = vec![0.0, 1e-21, 2e-21, 3e-21, 4e-21];
        let mu = 2.5e-21_f64;
        let temp = 1.0;
        let gc = GrandCanonicalEnsemble::new(energy_levels, temp, mu, true);
        let n0 = gc.mean_occupation(0);
        let n4 = gc.mean_occupation(4);
        assert!(n0 > 0.99, "Below μ: n should be ~1, got {n0}");
        assert!(n4 < 0.01, "Above μ: n should be ~0, got {n4}");
    }
    #[test]
    fn test_correlation_function_variance() {
        let samples = vec![1.0, -1.0, 1.0, -1.0, 1.0, -1.0];
        let cf = CorrelationFunction::new(samples);
        let var = cf.variance();
        assert!(var > 0.9, "Variance should be ~1, got {var}");
        let c0 = cf.connected_correlator(0);
        assert!(
            (c0 - var).abs() < 1e-10,
            "C(0) should equal variance, got c0={c0}, var={var}"
        );
    }
    #[test]
    fn test_correlation_function_mean() {
        let samples = vec![2.0, 2.0, 2.0, 2.0];
        let cf = CorrelationFunction::new(samples);
        assert!((cf.mean() - 2.0).abs() < 1e-10, "Mean should be 2.0");
        assert!(
            cf.variance().abs() < 1e-10,
            "Constant series has zero variance"
        );
    }
}
