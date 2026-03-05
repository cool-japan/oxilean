//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    ClassicalParticle, ElectromagneticField, FockSpaceVec, GaugeField, GeodesicEquation,
    HamiltonianSystem, IntegratorMethod, KdVSoliton, SchrodingerPropagator, SymplecticIntegrator,
};

pub fn app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
pub fn app2(f: Expr, a: Expr, b: Expr) -> Expr {
    app(app(f, a), b)
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
pub fn type1() -> Expr {
    Expr::Sort(Level::succ(Level::succ(Level::zero())))
}
pub fn bvar(n: u32) -> Expr {
    Expr::BVar(n)
}
pub fn real_ty() -> Expr {
    cst("Real")
}
pub fn vec_ty() -> Expr {
    app(cst("Vec"), real_ty())
}
/// Lagrangian type: L : TM → ℝ (map from the tangent bundle to reals).
/// Lagrangian : (M : Type) → Type
pub fn lagrangian_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "M",
        type0(),
        arrow(app(cst("TangentBundle"), cst("M")), real_ty()),
    )
}
/// Hamiltonian type: H : T*M → ℝ (map from the cotangent bundle to reals).
/// Hamiltonian : (M : Type) → Type
pub fn hamiltonian_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "M",
        type0(),
        arrow(app(cst("CotangentBundle"), cst("M")), real_ty()),
    )
}
/// Phase space type: symplectic manifold (M, ω).
/// PhaseSpace : (M : Type) → Type
pub fn phase_space_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "M",
        type0(),
        arrow(app(cst("SymplecticForm"), cst("M")), type0()),
    )
}
/// Euler-Lagrange equations type: equations of motion for the Lagrangian.
/// EulerLagrange : (M : Type) → Lagrangian M → Prop
pub fn euler_lagrange_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "M",
        type0(),
        arrow(app(cst("Lagrangian"), cst("M")), prop()),
    )
}
/// Maxwell equations type: EM field equations dF = 0, d★F = J.
/// MaxwellEquations : (F : DifferentialForm) → (J : CurrentForm) → Prop
pub fn maxwell_equations_ty() -> Expr {
    arrow(
        app2(cst("DifferentialForm"), cst("2"), cst("Spacetime")),
        arrow(app(cst("CurrentForm"), cst("Spacetime")), prop()),
    )
}
/// Einstein field equations type: G_μν = 8π T_μν.
/// EinsteinEquations : (g : Metric) → (T : StressEnergyTensor) → Prop
pub fn einstein_equations_ty() -> Expr {
    arrow(
        app(cst("LorentzianMetric"), cst("Spacetime")),
        arrow(app(cst("StressEnergyTensor"), cst("Spacetime")), prop()),
    )
}
/// Noether's theorem: every continuous symmetry yields a conserved quantity.
/// NoetherTheorem : ∀ (M : Type) (L : Lagrangian M) (sym : Symmetry L),
///   ∃ J : ConservedCurrent, dJ = 0
pub fn noether_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "M",
        type0(),
        pi(
            BinderInfo::Default,
            "L",
            app(cst("Lagrangian"), cst("M")),
            arrow(
                app2(cst("Symmetry"), cst("L"), cst("M")),
                app(cst("HasConservedCurrent"), cst("L")),
            ),
        ),
    )
}
/// Liouville's theorem: Hamiltonian flow preserves the phase space volume.
/// LiouvilleTheorem : ∀ (M : Type) (H : Hamiltonian M),
///   PreservesVolume (HamiltonianFlow H)
pub fn liouville_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "M",
        type0(),
        pi(
            BinderInfo::Default,
            "H",
            app(cst("Hamiltonian"), cst("M")),
            app(
                cst("PreservesVolume"),
                app(cst("HamiltonianFlow"), cst("H")),
            ),
        ),
    )
}
/// Hamilton's principle: the physical path makes the action stationary.
/// HamiltonsPrinciple : ∀ (M : Type) (L : Lagrangian M) (path : Path M),
///   PhysicalPath path ↔ StationaryAction L path
pub fn hamiltons_principle_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "M",
        type0(),
        pi(
            BinderInfo::Default,
            "L",
            app(cst("Lagrangian"), cst("M")),
            pi(
                BinderInfo::Default,
                "path",
                app(cst("Path"), cst("M")),
                app2(
                    cst("Iff"),
                    app(cst("PhysicalPath"), cst("path")),
                    app2(cst("StationaryAction"), cst("L"), cst("path")),
                ),
            ),
        ),
    )
}
/// Gauss's law: the flux of E through a closed surface equals Q/ε₀.
/// GaussLaw : ∀ (surface : ClosedSurface) (E : VectorField),
///   SurfaceIntegral E surface = TotalCharge / ε₀
pub fn gauss_law_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "surface",
        cst("ClosedSurface"),
        pi(
            BinderInfo::Default,
            "E",
            cst("VectorField"),
            arrow(
                prop(),
                app2(
                    cst("Eq"),
                    app2(cst("SurfaceIntegral"), cst("E"), cst("surface")),
                    app2(cst("Div"), cst("TotalCharge"), cst("Permittivity")),
                ),
            ),
        ),
    )
}
/// Energy-momentum conservation: ∇_μ T^{μν} = 0.
/// EnergyMomentumConservation : ∀ (T : StressEnergyTensor), DivFree T
pub fn energy_momentum_conservation_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "T",
        app(cst("StressEnergyTensor"), cst("Spacetime")),
        app(cst("DivFree"), cst("T")),
    )
}
/// Poisson bracket: {f, g} = ∂f/∂q · ∂g/∂p − ∂f/∂p · ∂g/∂q.
/// PoissonBracket : (M : Type) → (f g : CotangentBundle M → Real) → Real
pub fn poisson_bracket_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "M",
        type0(),
        arrow(
            arrow(app(cst("CotangentBundle"), cst("M")), real_ty()),
            arrow(
                arrow(app(cst("CotangentBundle"), cst("M")), real_ty()),
                real_ty(),
            ),
        ),
    )
}
/// Poisson bracket is antisymmetric: {f,g} = −{g,f}.
/// PoissonAntisymmetry : ∀ M f g, PoissonBracket M f g = −(PoissonBracket M g f)
pub fn poisson_antisymmetry_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "M",
        type0(),
        pi(
            BinderInfo::Default,
            "f",
            arrow(app(cst("CotangentBundle"), cst("M")), real_ty()),
            pi(
                BinderInfo::Default,
                "g",
                arrow(app(cst("CotangentBundle"), cst("M")), real_ty()),
                app2(
                    cst("Eq"),
                    app2(app(cst("PoissonBracket"), cst("M")), cst("f"), cst("g")),
                    app(
                        cst("Neg"),
                        app2(app(cst("PoissonBracket"), cst("M")), cst("g"), cst("f")),
                    ),
                ),
            ),
        ),
    )
}
/// Jacobi identity for Poisson brackets:
/// {f, {g, h}} + {g, {h, f}} + {h, {f, g}} = 0.
/// PoissonJacobiIdentity : ∀ M f g h, ... = 0
pub fn poisson_jacobi_identity_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "M",
        type0(),
        pi(
            BinderInfo::Default,
            "f",
            arrow(app(cst("CotangentBundle"), cst("M")), real_ty()),
            pi(
                BinderInfo::Default,
                "g",
                arrow(app(cst("CotangentBundle"), cst("M")), real_ty()),
                pi(
                    BinderInfo::Default,
                    "h",
                    arrow(app(cst("CotangentBundle"), cst("M")), real_ty()),
                    app(cst("PoissonJacobiHolds"), cst("M")),
                ),
            ),
        ),
    )
}
/// Symplectic structure: a closed non-degenerate 2-form ω on a manifold M.
/// SymplecticStructure : (M : Type) → Type
pub fn symplectic_structure_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "M",
        type0(),
        arrow(app2(cst("DifferentialForm"), cst("2"), cst("M")), prop()),
    )
}
/// Darboux's theorem: every symplectic manifold is locally canonical.
/// DarbouxTheorem : ∀ M ω, SymplecticStructure M ω → HasCanonicalCoordinates M ω
pub fn darboux_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "M",
        type0(),
        pi(
            BinderInfo::Default,
            "omega",
            app2(cst("DifferentialForm"), cst("2"), cst("M")),
            arrow(
                app2(cst("SymplecticStructure"), cst("M"), cst("omega")),
                app2(cst("HasCanonicalCoordinates"), cst("M"), cst("omega")),
            ),
        ),
    )
}
/// Schrödinger equation type: iħ ∂ψ/∂t = H ψ.
/// SchrodingerEquation : (H : HilbertSpace) → (psi : StateVector H) → Prop
pub fn schrodinger_equation_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "H",
        cst("HilbertSpace"),
        arrow(app(cst("StateVector"), cst("H")), prop()),
    )
}
/// Uncertainty principle (Robertson-Heisenberg):
/// ΔA · ΔB ≥ ½|⟨[A,B]⟩|.
/// UncertaintyPrinciple : ∀ (H : HilbertSpace) (A B : Observable H),
///   UncertaintyBound A B
pub fn uncertainty_principle_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "H",
        cst("HilbertSpace"),
        pi(
            BinderInfo::Default,
            "A",
            app(cst("Observable"), cst("H")),
            pi(
                BinderInfo::Default,
                "B",
                app(cst("Observable"), cst("H")),
                app2(cst("UncertaintyBound"), cst("A"), cst("B")),
            ),
        ),
    )
}
/// Spectral theorem for self-adjoint operators on a Hilbert space.
/// SpectralTheorem : ∀ (H : HilbertSpace) (A : SelfAdjointOp H),
///   HasSpectralDecomposition A
pub fn spectral_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "H",
        cst("HilbertSpace"),
        pi(
            BinderInfo::Default,
            "A",
            app(cst("SelfAdjointOp"), cst("H")),
            app(cst("HasSpectralDecomposition"), cst("A")),
        ),
    )
}
/// Born rule: the probability of measurement outcome λ is |⟨λ|ψ⟩|².
/// BornRule : ∀ H (psi : StateVector H) (obs : Observable H),
///   MeasurementProbability obs psi = NormSquared (Projection obs psi)
pub fn born_rule_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "H",
        cst("HilbertSpace"),
        pi(
            BinderInfo::Default,
            "psi",
            app(cst("StateVector"), cst("H")),
            pi(
                BinderInfo::Default,
                "obs",
                app(cst("Observable"), cst("H")),
                app2(
                    cst("Eq"),
                    app2(cst("MeasurementProbability"), cst("obs"), cst("psi")),
                    app(
                        cst("NormSquared"),
                        app2(cst("Projection"), cst("obs"), cst("psi")),
                    ),
                ),
            ),
        ),
    )
}
/// Path integral formulation: ⟨xf|xi⟩ = ∫ Dq exp(iS[q]/ħ).
/// PathIntegral : (M : Type) → (L : Lagrangian M) → (xi xf : M) → Complex
pub fn path_integral_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "M",
        type0(),
        pi(
            BinderInfo::Default,
            "L",
            app(cst("Lagrangian"), cst("M")),
            arrow(cst("M"), arrow(cst("M"), cst("Complex"))),
        ),
    )
}
/// Free scalar field action: S[φ] = ∫ (½(∂φ)² − ½m²φ²) d⁴x.
/// FreeScalarFieldAction : (m : Real) → (phi : ScalarField Spacetime) → Real
pub fn free_scalar_field_action_ty() -> Expr {
    arrow(
        real_ty(),
        arrow(app(cst("ScalarField"), cst("Spacetime")), real_ty()),
    )
}
/// Fock space: the Hilbert space of arbitrary-particle-number states.
/// FockSpace : (H1 : HilbertSpace) → HilbertSpace
pub fn fock_space_ty() -> Expr {
    arrow(cst("HilbertSpace"), cst("HilbertSpace"))
}
/// Creation operator a†: FockSpace H1 → FockSpace H1.
/// CreationOp : (H1 : HilbertSpace) → Operator (FockSpace H1)
pub fn creation_operator_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "H1",
        cst("HilbertSpace"),
        app(cst("Operator"), app(cst("FockSpace"), cst("H1"))),
    )
}
/// Annihilation operator a: FockSpace H1 → FockSpace H1.
/// AnnihilationOp : (H1 : HilbertSpace) → Operator (FockSpace H1)
pub fn annihilation_operator_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "H1",
        cst("HilbertSpace"),
        app(cst("Operator"), app(cst("FockSpace"), cst("H1"))),
    )
}
/// Canonical commutation relations: [a, a†] = 1.
/// CanonicalCommutationRelation : ∀ H1, CommutatorEq (AnnihilationOp H1) (CreationOp H1) Identity
pub fn canonical_commutation_relation_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "H1",
        cst("HilbertSpace"),
        app2(
            app(cst("CommutatorEq"), app(cst("FockSpace"), cst("H1"))),
            app(cst("AnnihilationOp"), cst("H1")),
            app(cst("CreationOp"), cst("H1")),
        ),
    )
}
/// Normal ordering: :O: places all creation operators to the left.
/// NormalOrdering : (H1 : HilbertSpace) → Operator (FockSpace H1) → Operator (FockSpace H1)
pub fn normal_ordering_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "H1",
        cst("HilbertSpace"),
        arrow(
            app(cst("Operator"), app(cst("FockSpace"), cst("H1"))),
            app(cst("Operator"), app(cst("FockSpace"), cst("H1"))),
        ),
    )
}
/// Wick's theorem: time-ordered product = normal-ordered product + contractions.
/// WicksTheorem : ∀ H1 (ops : List (Operator (FockSpace H1))),
///   TimeOrderedProduct ops = NormalOrderedPlusContractions ops
pub fn wicks_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "H1",
        cst("HilbertSpace"),
        pi(
            BinderInfo::Default,
            "ops",
            app(
                cst("List"),
                app(cst("Operator"), app(cst("FockSpace"), cst("H1"))),
            ),
            app(cst("WickExpansionHolds"), cst("ops")),
        ),
    )
}
/// Yang-Mills action: S[A] = −¼ ∫ tr(F ∧ ★F).
/// YangMillsAction : (G : LieGroup) → (A : Connection G Spacetime) → Real
pub fn yang_mills_action_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        cst("LieGroup"),
        arrow(
            app2(cst("Connection"), cst("G"), cst("Spacetime")),
            real_ty(),
        ),
    )
}
/// Yang-Mills equations: D ★ F = J (gauge covariant divergence-free field).
/// YangMillsEquations : (G : LieGroup) → (A : Connection G Spacetime) → Prop
pub fn yang_mills_equations_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        cst("LieGroup"),
        arrow(app2(cst("Connection"), cst("G"), cst("Spacetime")), prop()),
    )
}
/// Chern-Simons action: S[A] = (k/4π) ∫ tr(A ∧ dA + ⅔ A ∧ A ∧ A).
/// ChernSimonsAction : (G : LieGroup) (M : 3Manifold) (k : Int) → (A : Connection G M) → Real
pub fn chern_simons_action_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        cst("LieGroup"),
        pi(
            BinderInfo::Default,
            "M",
            cst("ThreeManifold"),
            arrow(app2(cst("Connection"), cst("G"), cst("M")), real_ty()),
        ),
    )
}
/// Geodesic equation: d²x^μ/dτ² + Γ^μ_{νρ} (dx^ν/dτ)(dx^ρ/dτ) = 0.
/// GeodesicEquations : (g : LorentzianMetric Spacetime) → (gamma : Curve Spacetime) → Prop
pub fn geodesic_equations_ty() -> Expr {
    arrow(
        app(cst("LorentzianMetric"), cst("Spacetime")),
        arrow(app(cst("Curve"), cst("Spacetime")), prop()),
    )
}
/// Schwarzschild metric: static spherically symmetric vacuum solution.
/// SchwarzschildMetric : (M_bh : Real) → LorentzianMetric Spacetime
pub fn schwarzschild_metric_ty() -> Expr {
    arrow(real_ty(), app(cst("LorentzianMetric"), cst("Spacetime")))
}
/// Hawking temperature: T_H = ħc³ / (8πGMk_B).
/// HawkingTemperature : (M_bh : Real) → Real
pub fn hawking_temperature_ty() -> Expr {
    arrow(real_ty(), real_ty())
}
/// Hawking radiation: black holes emit thermal radiation at T_H.
/// HawkingRadiation : ∀ (M_bh : Real), IsBlackBodySpectrum (HawkingTemperature M_bh)
pub fn hawking_radiation_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "M_bh",
        real_ty(),
        app(
            cst("IsBlackBodySpectrum"),
            app(cst("HawkingTemperature"), cst("M_bh")),
        ),
    )
}
/// Polyakov (worldsheet) action: S = −T/2 ∫ √|h| h^{ab} ∂_a X · ∂_b X d²σ.
/// PolyakovAction : (T_str : Real) → (X : WorldsheetMap) → Real
pub fn polyakov_action_ty() -> Expr {
    arrow(real_ty(), arrow(cst("WorldsheetMap"), real_ty()))
}
/// Virasoro algebra: central extension of the Witt algebra; [L_m, L_n] = (m−n)L_{m+n} + c/12(m³−m)δ.
/// VirasoroAlgebra : (c : Real) → LieAlgebra
pub fn virasoro_algebra_ty() -> Expr {
    arrow(real_ty(), cst("LieAlgebra"))
}
/// Conformal Ward identity in a 2D CFT.
/// ConformalWardIdentity : (c : Real) → (T : StressEnergyTensor CFT2) → Prop
pub fn conformal_ward_identity_ty() -> Expr {
    arrow(
        real_ty(),
        arrow(app(cst("StressEnergyTensor"), cst("CFT2")), prop()),
    )
}
/// Topological quantum field theory (Atiyah axioms):
/// TQFT : (n : Nat) → (Z : Functor (nCobord n) HilbertSpace) → Prop
pub fn tqft_axioms_ty() -> Expr {
    arrow(
        cst("Nat"),
        arrow(
            app2(
                cst("Functor"),
                app(cst("nCobord"), cst("Nat")),
                cst("HilbertSpace"),
            ),
            prop(),
        ),
    )
}
/// Wess-Zumino-Witten model action at level k.
/// WZWAction : (G : LieGroup) (k : Int) → (g : SmoothMap Sigma G) → Real
pub fn wzw_action_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        cst("LieGroup"),
        arrow(
            cst("Int"),
            arrow(app2(cst("SmoothMap"), cst("Sigma"), cst("G")), real_ty()),
        ),
    )
}
/// Lax pair representation: isospectral deformation dL/dt = [M, L].
/// LaxPair : (L M : MatrixField) → Prop
pub fn lax_pair_ty() -> Expr {
    arrow(cst("MatrixField"), arrow(cst("MatrixField"), prop()))
}
/// Inverse scattering transform: maps KdV initial data to scattering data.
/// InverseScatteringTransform : (u0 : ScalarField Real) → ScatteringData
pub fn inverse_scattering_ty() -> Expr {
    arrow(app(cst("ScalarField"), real_ty()), cst("ScatteringData"))
}
/// KdV equation: ∂u/∂t + 6u ∂u/∂x + ∂³u/∂x³ = 0.
/// KdVEquation : (u : ScalarField (Real × Real)) → Prop
pub fn kdv_equation_ty() -> Expr {
    arrow(
        app(cst("ScalarField"), app2(cst("Prod"), real_ty(), real_ty())),
        prop(),
    )
}
/// KdV one-soliton solution: u(x,t) = −½κ² sech²(½κ(x − κ²t − x₀)).
/// KdVOneSoliton : (kappa x0 : Real) → ScalarField (Real × Real)
pub fn kdv_one_soliton_ty() -> Expr {
    arrow(
        real_ty(),
        arrow(
            real_ty(),
            app(cst("ScalarField"), app2(cst("Prod"), real_ty(), real_ty())),
        ),
    )
}
/// Conservation laws of KdV: infinitely many conserved quantities.
/// KdVConservationLaws : ∀ (u : ScalarField (Real × Real)),
///   KdVEquation u → ∀ n : Nat, HasConservedQuantity u n
pub fn kdv_conservation_laws_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "u",
        app(cst("ScalarField"), app2(cst("Prod"), real_ty(), real_ty())),
        arrow(
            app(cst("KdVEquation"), cst("u")),
            pi(
                BinderInfo::Default,
                "n",
                cst("Nat"),
                app2(cst("HasConservedQuantity"), cst("u"), cst("n")),
            ),
        ),
    )
}
/// Populate an `Environment` with mathematical physics axioms and theorems.
pub fn build_mathematical_physics_env() -> Environment {
    let mut env = Environment::new();
    let axioms: &[(&str, Expr)] = &[
        ("Lagrangian", lagrangian_ty()),
        ("Hamiltonian", hamiltonian_ty()),
        ("PhaseSpace", phase_space_ty()),
        ("EulerLagrange", euler_lagrange_ty()),
        ("MaxwellEquations", maxwell_equations_ty()),
        ("EinsteinEquations", einstein_equations_ty()),
        ("NoetherTheorem", noether_theorem_ty()),
        ("LiouvilleTheorem", liouville_theorem_ty()),
        ("HamiltonsPrinciple", hamiltons_principle_ty()),
        ("GaussLaw", gauss_law_ty()),
        (
            "EnergyMomentumConservation",
            energy_momentum_conservation_ty(),
        ),
        ("PoissonBracket", poisson_bracket_ty()),
        ("PoissonAntisymmetry", poisson_antisymmetry_ty()),
        ("PoissonJacobiIdentity", poisson_jacobi_identity_ty()),
        ("SymplecticStructure", symplectic_structure_ty()),
        ("DarbouxTheorem", darboux_theorem_ty()),
        ("SchrodingerEquation", schrodinger_equation_ty()),
        ("UncertaintyPrinciple", uncertainty_principle_ty()),
        ("SpectralTheoremQM", spectral_theorem_ty()),
        ("BornRule", born_rule_ty()),
        ("PathIntegral", path_integral_ty()),
        ("FreeScalarFieldAction", free_scalar_field_action_ty()),
        ("FockSpace", fock_space_ty()),
        ("CreationOp", creation_operator_ty()),
        ("AnnihilationOp", annihilation_operator_ty()),
        (
            "CanonicalCommutationRelation",
            canonical_commutation_relation_ty(),
        ),
        ("NormalOrdering", normal_ordering_ty()),
        ("WicksTheorem", wicks_theorem_ty()),
        ("YangMillsAction", yang_mills_action_ty()),
        ("YangMillsEquations", yang_mills_equations_ty()),
        ("ChernSimonsAction", chern_simons_action_ty()),
        ("GeodesicEquations", geodesic_equations_ty()),
        ("SchwarzschildMetric", schwarzschild_metric_ty()),
        ("HawkingTemperature", hawking_temperature_ty()),
        ("HawkingRadiation", hawking_radiation_ty()),
        ("PolyakovAction", polyakov_action_ty()),
        ("VirasoroAlgebra", virasoro_algebra_ty()),
        ("ConformalWardIdentity", conformal_ward_identity_ty()),
        ("TQFTAxioms", tqft_axioms_ty()),
        ("WZWAction", wzw_action_ty()),
        ("LaxPair", lax_pair_ty()),
        ("InverseScatteringTransform", inverse_scattering_ty()),
        ("KdVEquation", kdv_equation_ty()),
        ("KdVOneSoliton", kdv_one_soliton_ty()),
        ("KdVConservationLaws", kdv_conservation_laws_ty()),
        ("TangentBundle", arrow(type0(), type0())),
        ("CotangentBundle", arrow(type0(), type0())),
        ("SymplecticForm", arrow(type0(), type0())),
        ("DifferentialForm", arrow(type0(), arrow(type0(), type0()))),
        ("CurrentForm", arrow(type0(), type0())),
        ("LorentzianMetric", arrow(type0(), type0())),
        ("StressEnergyTensor", arrow(type0(), type0())),
        ("Spacetime", type0()),
        ("Symmetry", arrow(type0(), arrow(type0(), prop()))),
        ("HasConservedCurrent", arrow(type0(), prop())),
        ("HamiltonianFlow", arrow(type0(), type0())),
        ("PreservesVolume", arrow(type0(), prop())),
        ("Path", arrow(type0(), type0())),
        ("PhysicalPath", arrow(type0(), prop())),
        ("StationaryAction", arrow(type0(), arrow(type0(), prop()))),
        ("ClosedSurface", type0()),
        ("VectorField", type0()),
        ("SurfaceIntegral", arrow(type0(), arrow(type0(), real_ty()))),
        ("TotalCharge", real_ty()),
        ("Permittivity", real_ty()),
        ("DivFree", arrow(type0(), prop())),
        ("Iff", arrow(prop(), arrow(prop(), prop()))),
        ("Eq", arrow(type0(), arrow(type0(), prop()))),
        ("Div", arrow(real_ty(), arrow(real_ty(), real_ty()))),
        ("PoissonJacobiHolds", arrow(type0(), prop())),
        (
            "HasCanonicalCoordinates",
            arrow(type0(), arrow(type0(), prop())),
        ),
        ("HilbertSpace", type0()),
        ("StateVector", arrow(type0(), type0())),
        ("Observable", arrow(type0(), type0())),
        ("UncertaintyBound", arrow(type0(), arrow(type0(), prop()))),
        ("SelfAdjointOp", arrow(type0(), type0())),
        ("HasSpectralDecomposition", arrow(type0(), prop())),
        (
            "MeasurementProbability",
            arrow(type0(), arrow(type0(), real_ty())),
        ),
        ("NormSquared", arrow(type0(), real_ty())),
        ("Projection", arrow(type0(), arrow(type0(), type0()))),
        ("Complex", type0()),
        ("ScalarField", arrow(type0(), type0())),
        ("Operator", arrow(type0(), type0())),
        (
            "CommutatorEq",
            arrow(type0(), arrow(type0(), arrow(type0(), prop()))),
        ),
        ("TimeOrderedProduct", arrow(type0(), type0())),
        ("WickExpansionHolds", arrow(type0(), prop())),
        ("List", arrow(type0(), type0())),
        ("LieGroup", type0()),
        ("Connection", arrow(type0(), arrow(type0(), type0()))),
        ("ThreeManifold", type0()),
        ("Curve", arrow(type0(), type0())),
        ("IsBlackBodySpectrum", arrow(type0(), prop())),
        ("WorldsheetMap", type0()),
        ("LieAlgebra", type0()),
        ("CFT2", type0()),
        ("Nat", type0()),
        ("Functor", arrow(type0(), arrow(type0(), type0()))),
        ("nCobord", arrow(type0(), type0())),
        ("Int", type0()),
        ("Sigma", type0()),
        ("SmoothMap", arrow(type0(), arrow(type0(), type0()))),
        ("MatrixField", type0()),
        ("ScatteringData", type0()),
        ("Prod", arrow(type0(), arrow(type0(), type0()))),
        (
            "HasConservedQuantity",
            arrow(type0(), arrow(type0(), prop())),
        ),
        ("Neg", arrow(real_ty(), real_ty())),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    env
}
/// Physical constants.
pub const EPSILON_0: f64 = 8.854_187_817e-12;
pub const MU_0: f64 = 1.256_637_061e-6;
/// Compute the cross product of two 3-vectors.
pub fn cross3(a: &[f64], b: &[f64]) -> Vec<f64> {
    vec![
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
/// WightmanField: a quantum field satisfying the Wightman axioms.
/// WightmanField : (n : Nat) → Type
pub fn mp_ext_wightman_field_ty() -> Expr {
    arrow(cst("Nat"), type0())
}
/// WightmanVacuum: existence of a unique Poincaré-invariant vacuum state.
/// WightmanVacuum : (H : HilbertSpace) → Prop
pub fn mp_ext_wightman_vacuum_ty() -> Expr {
    arrow(cst("HilbertSpace"), prop())
}
/// WightmanCovariance: Poincaré covariance of Wightman fields.
/// WightmanCovariance : (phi : WightmanField n) → Prop
pub fn mp_ext_wightman_covariance_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        cst("Nat"),
        arrow(app(cst("WightmanField"), cst("n")), prop()),
    )
}
/// WightmanLocality: fields at spacelike separation commute (or anti-commute).
/// WightmanLocality : (phi psi : WightmanField n) → Prop
pub fn mp_ext_wightman_locality_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        cst("Nat"),
        arrow(
            app(cst("WightmanField"), cst("n")),
            arrow(app(cst("WightmanField"), cst("n")), prop()),
        ),
    )
}
/// WightmanSpectrumCondition: spectrum of the energy-momentum operator is in the forward cone.
/// WightmanSpectrumCondition : (H : HilbertSpace) → (U : PoincareRep H) → Prop
pub fn mp_ext_wightman_spectrum_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "H",
        cst("HilbertSpace"),
        arrow(app(cst("PoincareRep"), cst("H")), prop()),
    )
}
/// AlgebraicQFT: a net of algebras satisfying the Haag-Kastler axioms.
/// AlgebraicQFT : (Spacetime : Type) → Type 1
pub fn mp_ext_algebraic_qft_ty() -> Expr {
    pi(BinderInfo::Default, "M", type0(), type1())
}
/// HaagKastlerIsotony: if O₁ ⊂ O₂ then A(O₁) ⊂ A(O₂).
/// HKIsotony : (net : AlgebraicQFT M) → Prop
pub fn mp_ext_hk_isotony_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "M",
        type0(),
        arrow(app(cst("AlgebraicQFT"), bvar(0)), prop()),
    )
}
/// HaagKastlerLocality: algebras of spacelike-separated regions commute.
/// HKLocality : (net : AlgebraicQFT M) → Prop
pub fn mp_ext_hk_locality_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "M",
        type0(),
        arrow(app(cst("AlgebraicQFT"), bvar(0)), prop()),
    )
}
/// HaagKastlerCovariance: Poincaré covariance of the net.
/// HKCovariance : (net : AlgebraicQFT M) → Prop
pub fn mp_ext_hk_covariance_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "M",
        type0(),
        arrow(app(cst("AlgebraicQFT"), bvar(0)), prop()),
    )
}
/// HaagDuality: local algebras equal the commutant of spacelike complement.
/// HaagDuality : (net : AlgebraicQFT M) → Prop
pub fn mp_ext_haag_duality_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "M",
        type0(),
        arrow(app(cst("AlgebraicQFT"), bvar(0)), prop()),
    )
}
/// PCTTheorem: PCT symmetry holds in any local Wightman QFT.
/// PCTTheorem : (phi : WightmanField n) → HasPCTSymmetry phi
pub fn mp_ext_pct_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        cst("Nat"),
        pi(
            BinderInfo::Default,
            "phi",
            app(cst("WightmanField"), bvar(0)),
            app(cst("HasPCTSymmetry"), bvar(0)),
        ),
    )
}
/// SpinStatisticsTheorem: integer spin ↔ Bose statistics; half-integer ↔ Fermi.
/// SpinStatistics : (phi : WightmanField n) → SpinStatisticsRelationHolds phi
pub fn mp_ext_spin_statistics_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        cst("Nat"),
        pi(
            BinderInfo::Default,
            "phi",
            app(cst("WightmanField"), bvar(0)),
            app(cst("SpinStatisticsRelationHolds"), bvar(0)),
        ),
    )
}
/// AtiyahTQFT: a TQFT satisfying all of Atiyah's axioms.
/// AtiyahTQFT : (n : Nat) → Type 1
pub fn mp_ext_atiyah_tqft_ty() -> Expr {
    arrow(cst("Nat"), type1())
}
/// TQFTInvolutivity: Z(M̄) = Z(M)† (orientation reversal = adjoint).
/// TQFTInvolutivity : (Z : AtiyahTQFT n) → Prop
pub fn mp_ext_tqft_involutivity_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        cst("Nat"),
        arrow(app(cst("AtiyahTQFT"), bvar(0)), prop()),
    )
}
/// TQFTMultiplicativity: Z(M₁ ⊔ M₂) = Z(M₁) ⊗ Z(M₂).
/// TQFTMultiplicativity : (Z : AtiyahTQFT n) → Prop
pub fn mp_ext_tqft_multiplicativity_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        cst("Nat"),
        arrow(app(cst("AtiyahTQFT"), bvar(0)), prop()),
    )
}
/// TQFTGluing: partition functions compose under gluing of manifolds.
/// TQFTGluing : (Z : AtiyahTQFT n) → Prop
pub fn mp_ext_tqft_gluing_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        cst("Nat"),
        arrow(app(cst("AtiyahTQFT"), bvar(0)), prop()),
    )
}
/// SegalCFT: a Segal-style conformal field theory (functor from Riemann surfaces).
/// SegalCFT : Type 1
pub fn mp_ext_segal_cft_ty() -> Expr {
    type1()
}
/// ConformalBootstrap: OPE coefficients determine the CFT.
/// ConformalBootstrap : (CFT : SegalCFT) → Prop
pub fn mp_ext_conformal_bootstrap_ty() -> Expr {
    arrow(cst("SegalCFT"), prop())
}
/// OPECoefficients: the operator product expansion data of a CFT.
/// OPECoefficients : (CFT : SegalCFT) → Type
pub fn mp_ext_ope_coefficients_ty() -> Expr {
    arrow(cst("SegalCFT"), type0())
}
/// VertexOperatorAlgebra: a VOA with state-operator correspondence.
/// VOA : Type 1
pub fn mp_ext_voa_ty() -> Expr {
    type1()
}
/// VOAVacuum: the vacuum vector in the VOA.
/// VOAVacuum : (V : VOA) → V.carrier
pub fn mp_ext_voa_vacuum_ty() -> Expr {
    arrow(cst("VOA"), type0())
}
/// VOATranslation: the translation operator T in the VOA.
/// VOATranslation : (V : VOA) → Endomorphism V
pub fn mp_ext_voa_translation_ty() -> Expr {
    arrow(cst("VOA"), app(cst("Endomorphism"), cst("VOA")))
}
/// VOAStateFieldCorrespondence: the state-field map Y : V → End(V)[[z, z⁻¹]].
/// VOAStateField : (V : VOA) → StateFieldMap V
pub fn mp_ext_voa_state_field_ty() -> Expr {
    arrow(cst("VOA"), app(cst("StateFieldMap"), cst("VOA")))
}
/// VOAJacobiIdentity: the Jacobi identity for vertex operators.
/// VOAJacobi : (V : VOA) → Prop
pub fn mp_ext_voa_jacobi_ty() -> Expr {
    arrow(cst("VOA"), prop())
}
/// BVComplex: the Batalin-Vilkovisky complex for a gauge theory.
/// BVComplex : (S : ActionFunctional) → Type
pub fn mp_ext_bv_complex_ty() -> Expr {
    arrow(cst("ActionFunctional"), type0())
}
/// BVMasterEquation: the classical master equation {S, S} = 0.
/// BVMasterEquation : (S : ActionFunctional) → Prop
pub fn mp_ext_bv_master_equation_ty() -> Expr {
    arrow(cst("ActionFunctional"), prop())
}
/// BRSTOperator: the BRST charge Q with Q² = 0.
/// BRSTOperator : (S : ActionFunctional) → Endomorphism (BVComplex S)
pub fn mp_ext_brst_operator_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "S",
        cst("ActionFunctional"),
        app(cst("Endomorphism"), app(cst("BVComplex"), bvar(0))),
    )
}
/// BRSTCohomology: the BRST cohomology groups of a gauge theory.
/// BRSTCohomology : (S : ActionFunctional) → (n : Nat) → Type
pub fn mp_ext_brst_cohomology_ty() -> Expr {
    arrow(cst("ActionFunctional"), arrow(cst("Nat"), type0()))
}
/// BRSTNilpotent: Q² = 0.
/// BRSTNilpotent : (S : ActionFunctional) → Prop
pub fn mp_ext_brst_nilpotent_ty() -> Expr {
    arrow(cst("ActionFunctional"), prop())
}
/// SupersymmetryAlgebra: the SUSY algebra {Q_α, Q†_β} = 2σ^μ_{αβ} P_μ.
/// SupersymmetryAlgebra : (d : Nat) → LieSuperAlgebra
pub fn mp_ext_susy_algebra_ty() -> Expr {
    arrow(cst("Nat"), cst("LieSuperAlgebra"))
}
/// SuperchargeQ: the supercharge operator Q in the SUSY algebra.
/// SuperchargeQ : (d : Nat) → Operator (SupersymmetryRep d)
pub fn mp_ext_supercharge_q_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "d",
        cst("Nat"),
        app(cst("Operator"), app(cst("SupersymmetryRep"), bvar(0))),
    )
}
/// SUSYAlgebraRelations: the anticommutation relations of the SUSY algebra.
/// SUSYAlgebraRelations : (d : Nat) → Prop
pub fn mp_ext_susy_relations_ty() -> Expr {
    arrow(cst("Nat"), prop())
}
/// Supermanifold: a manifold with both bosonic and fermionic coordinates.
/// Supermanifold : (p q : Nat) → Type
pub fn mp_ext_supermanifold_ty() -> Expr {
    arrow(cst("Nat"), arrow(cst("Nat"), type0()))
}
/// SuperDifferentialForm: a differential form on a supermanifold.
/// SuperDifferentialForm : (M : Supermanifold p q) → (k : Nat) → Type
pub fn mp_ext_super_diff_form_ty() -> Expr {
    arrow(type0(), arrow(cst("Nat"), type0()))
}
/// BerezinIntegral: integration over fermionic variables.
/// BerezinIntegral : (f : SuperFunction) → Real
pub fn mp_ext_berezin_integral_ty() -> Expr {
    arrow(cst("SuperFunction"), real_ty())
}
/// MirrorPair: a pair of Calabi-Yau manifolds related by mirror symmetry.
/// MirrorPair : (X Y : CalabiYauManifold) → Prop
pub fn mp_ext_mirror_pair_ty() -> Expr {
    arrow(
        cst("CalabiYauManifold"),
        arrow(cst("CalabiYauManifold"), prop()),
    )
}
/// SYZFibration: a special Lagrangian torus fibration (SYZ conjecture).
/// SYZFibration : (X : CalabiYauManifold) → Prop
pub fn mp_ext_syz_fibration_ty() -> Expr {
    arrow(cst("CalabiYauManifold"), prop())
}
/// MirrorSymmetryIsomorphism: Hᵖ'q(X) ≅ Hᵠ(Y, ∧ᵖ TY) under mirror symmetry.
/// MirrorIsomorphism : (X Y : CalabiYauManifold) → MirrorPair X Y → Prop
pub fn mp_ext_mirror_isomorphism_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("CalabiYauManifold"),
        pi(
            BinderInfo::Default,
            "Y",
            cst("CalabiYauManifold"),
            arrow(app2(cst("MirrorPair"), bvar(1), bvar(0)), prop()),
        ),
    )
}
/// HomologicalMirrorSymmetry: Fuk(X) ≅ Db(Coh(Y)) (Kontsevich conjecture).
/// HomologicalMirrorSymmetry : (X Y : CalabiYauManifold) → Prop
pub fn mp_ext_homological_mirror_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("CalabiYauManifold"),
        pi(BinderInfo::Default, "Y", cst("CalabiYauManifold"), prop()),
    )
}
/// DonaldsonInvariant: polynomial invariants of smooth 4-manifolds.
/// DonaldsonInvariant : (M : FourManifold) → (k : Nat) → Int
pub fn mp_ext_donaldson_invariant_ty() -> Expr {
    arrow(cst("FourManifold"), arrow(cst("Nat"), cst("Int")))
}
/// AntiSelfDualConnection: instanton (ASD) connection on a 4-manifold.
/// AntiSelfDualConnection : (G : LieGroup) → (M : FourManifold) → Type
pub fn mp_ext_asd_connection_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        cst("LieGroup"),
        arrow(cst("FourManifold"), type0()),
    )
}
/// InstantonEquations: F⁺ = 0 (self-dual part of curvature vanishes).
/// InstantonEquations : (G : LieGroup) → (M : FourManifold) →
///   (A : Connection G M) → Prop
pub fn mp_ext_instanton_equations_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        cst("LieGroup"),
        pi(
            BinderInfo::Default,
            "M",
            cst("FourManifold"),
            arrow(app2(cst("Connection"), bvar(1), bvar(0)), prop()),
        ),
    )
}
/// SeibergWittenEquations: the SW monopole equations on a 4-manifold.
/// SeibergWittenEquations : (M : FourManifold) → (A : SpinCConnection M) →
///   (psi : Spinor M) → Prop
pub fn mp_ext_sw_equations_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "M",
        cst("FourManifold"),
        arrow(
            app(cst("SpinCConnection"), bvar(0)),
            arrow(app(cst("Spinor"), bvar(1)), prop()),
        ),
    )
}
/// SeibergWittenInvariant: SW invariants of smooth 4-manifolds.
/// SWInvariant : (M : FourManifold) → Int
pub fn mp_ext_sw_invariant_ty() -> Expr {
    arrow(cst("FourManifold"), cst("Int"))
}
/// WittenEquivalence: Donaldson and SW theories are equivalent (Witten 1994).
/// WittenEquivalence : (M : FourManifold) → Prop
pub fn mp_ext_witten_equivalence_ty() -> Expr {
    arrow(cst("FourManifold"), prop())
}
/// ChernSimonsPartitionFunction: the CS partition function as a knot invariant.
/// CSPartitionFunction : (G : LieGroup) → (M : ThreeManifold) → (k : Int) → Complex
pub fn mp_ext_cs_partition_function_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        cst("LieGroup"),
        pi(
            BinderInfo::Default,
            "M",
            cst("ThreeManifold"),
            arrow(cst("Int"), cst("Complex")),
        ),
    )
}
/// WilsonLoop: the expectation value of a Wilson loop in CS theory.
/// WilsonLoop : (G : LieGroup) → (M : ThreeManifold) → (gamma : Knot M) → Complex
pub fn mp_ext_wilson_loop_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        cst("LieGroup"),
        pi(
            BinderInfo::Default,
            "M",
            cst("ThreeManifold"),
            arrow(app(cst("Knot"), bvar(0)), cst("Complex")),
        ),
    )
}
/// JonesPolynomial: the Jones polynomial obtained from CS theory (Witten).
/// JonesPolynomial : (L : Link) → LaurentPolynomial
pub fn mp_ext_jones_polynomial_ty() -> Expr {
    arrow(cst("Link"), cst("LaurentPolynomial"))
}
/// YangMillsExistence: existence of a mass gap in 4D Yang-Mills (Millennium problem).
/// YangMillsMassGap : (G : LieGroup) → Prop
pub fn mp_ext_yang_mills_mass_gap_ty() -> Expr {
    arrow(cst("LieGroup"), prop())
}
/// YangMillsInstantonNumber: the instanton number (second Chern class).
/// InstantonNumber : (G : LieGroup) → (M : FourManifold) →
///   (A : Connection G M) → Int
pub fn mp_ext_instanton_number_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        cst("LieGroup"),
        pi(
            BinderInfo::Default,
            "M",
            cst("FourManifold"),
            arrow(app2(cst("Connection"), bvar(1), bvar(0)), cst("Int")),
        ),
    )
}
/// BogomolnyBound: the lower bound on YM action by topological charge.
/// BogomolnyBound : (G : LieGroup) → (M : FourManifold) →
///   (A : Connection G M) → Prop
pub fn mp_ext_bogomolny_bound_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        cst("LieGroup"),
        pi(
            BinderInfo::Default,
            "M",
            cst("FourManifold"),
            arrow(app2(cst("Connection"), bvar(1), bvar(0)), prop()),
        ),
    )
}
/// Register all extended mathematical physics axioms.
pub fn register_mathematical_physics_extended(env: &mut Environment) -> Result<(), String> {
    let axioms: &[(&str, Expr)] = &[
        ("WightmanField", mp_ext_wightman_field_ty()),
        ("WightmanVacuum", mp_ext_wightman_vacuum_ty()),
        ("WightmanCovariance", mp_ext_wightman_covariance_ty()),
        ("WightmanLocality", mp_ext_wightman_locality_ty()),
        ("WightmanSpectrumCondition", mp_ext_wightman_spectrum_ty()),
        ("AlgebraicQFT", mp_ext_algebraic_qft_ty()),
        ("HKIsotony", mp_ext_hk_isotony_ty()),
        ("HKLocality", mp_ext_hk_locality_ty()),
        ("HKCovariance", mp_ext_hk_covariance_ty()),
        ("HaagDuality", mp_ext_haag_duality_ty()),
        ("PCTTheorem", mp_ext_pct_theorem_ty()),
        ("SpinStatisticsTheorem", mp_ext_spin_statistics_ty()),
        ("AtiyahTQFT", mp_ext_atiyah_tqft_ty()),
        ("TQFTInvolutivity", mp_ext_tqft_involutivity_ty()),
        ("TQFTMultiplicativity", mp_ext_tqft_multiplicativity_ty()),
        ("TQFTGluing", mp_ext_tqft_gluing_ty()),
        ("SegalCFT", mp_ext_segal_cft_ty()),
        ("ConformalBootstrap", mp_ext_conformal_bootstrap_ty()),
        ("OPECoefficients", mp_ext_ope_coefficients_ty()),
        ("VOA", mp_ext_voa_ty()),
        ("VOAVacuum", mp_ext_voa_vacuum_ty()),
        ("VOATranslation", mp_ext_voa_translation_ty()),
        ("VOAStateField", mp_ext_voa_state_field_ty()),
        ("VOAJacobi", mp_ext_voa_jacobi_ty()),
        ("BVComplex", mp_ext_bv_complex_ty()),
        ("BVMasterEquation", mp_ext_bv_master_equation_ty()),
        ("BRSTOperator", mp_ext_brst_operator_ty()),
        ("BRSTCohomology", mp_ext_brst_cohomology_ty()),
        ("BRSTNilpotent", mp_ext_brst_nilpotent_ty()),
        ("SupersymmetryAlgebra", mp_ext_susy_algebra_ty()),
        ("SuperchargeQ", mp_ext_supercharge_q_ty()),
        ("SUSYAlgebraRelations", mp_ext_susy_relations_ty()),
        ("Supermanifold", mp_ext_supermanifold_ty()),
        ("SuperDifferentialForm", mp_ext_super_diff_form_ty()),
        ("BerezinIntegral", mp_ext_berezin_integral_ty()),
        ("CalabiYauManifold", type0()),
        ("MirrorPair", mp_ext_mirror_pair_ty()),
        ("SYZFibration", mp_ext_syz_fibration_ty()),
        ("MirrorIsomorphism", mp_ext_mirror_isomorphism_ty()),
        ("HomologicalMirrorSymmetry", mp_ext_homological_mirror_ty()),
        ("FourManifold", type0()),
        ("DonaldsonInvariant", mp_ext_donaldson_invariant_ty()),
        ("AntiSelfDualConnection", mp_ext_asd_connection_ty()),
        ("InstantonEquations", mp_ext_instanton_equations_ty()),
        ("SeibergWittenEquations", mp_ext_sw_equations_ty()),
        ("SWInvariant", mp_ext_sw_invariant_ty()),
        ("WittenEquivalence", mp_ext_witten_equivalence_ty()),
        ("CSPartitionFunction", mp_ext_cs_partition_function_ty()),
        ("WilsonLoop", mp_ext_wilson_loop_ty()),
        ("JonesPolynomial", mp_ext_jones_polynomial_ty()),
        ("YangMillsMassGap", mp_ext_yang_mills_mass_gap_ty()),
        ("InstantonNumber", mp_ext_instanton_number_ty()),
        ("BogomolnyBound", mp_ext_bogomolny_bound_ty()),
        ("PoincareRep", arrow(cst("HilbertSpace"), type0())),
        ("HasPCTSymmetry", arrow(type0(), prop())),
        ("SpinStatisticsRelationHolds", arrow(type0(), prop())),
        ("LieSuperAlgebra", type0()),
        ("SupersymmetryRep", arrow(cst("Nat"), type0())),
        ("SuperFunction", type0()),
        ("StateFieldMap", arrow(type0(), type0())),
        ("Endomorphism", arrow(type0(), type0())),
        ("ActionFunctional", type0()),
        ("SpinCConnection", arrow(type0(), type0())),
        ("Spinor", arrow(type0(), type0())),
        ("Knot", arrow(type0(), type0())),
        ("Link", type0()),
        ("LaurentPolynomial", type0()),
        ("Complex", type0()),
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
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_classical_particle_kinetic_energy() {
        let mut p = ClassicalParticle::new(2.0, 3);
        p.set_velocity(vec![3.0, 0.0, 0.0]);
        assert!((p.kinetic_energy() - 9.0).abs() < 1e-12);
    }
    #[test]
    fn test_classical_particle_momentum() {
        let mut p = ClassicalParticle::new(3.0, 2);
        p.set_velocity(vec![2.0, -1.0]);
        let mom = p.momentum();
        assert!((mom[0] - 6.0).abs() < 1e-12);
        assert!((mom[1] - (-3.0)).abs() < 1e-12);
    }
    #[test]
    fn test_symplectic_integrator_euler() {
        let integrator = SymplecticIntegrator {
            dt: 0.01,
            method: IntegratorMethod::Euler,
        };
        let grad_h = |q: &[f64], p: &[f64]| -> (Vec<f64>, Vec<f64>) { (p.to_vec(), q.to_vec()) };
        let (q_new, p_new) = integrator.step(&[0.0], &[1.0], &grad_h);
        assert!((q_new[0] - 0.01).abs() < 1e-12);
        assert!((p_new[0] - 1.0).abs() < 1e-12);
    }
    #[test]
    fn test_symplectic_integrator_leapfrog_energy_near_conservation() {
        let integrator = SymplecticIntegrator {
            dt: 0.01,
            method: IntegratorMethod::Leapfrog,
        };
        let grad_h = |q: &[f64], p: &[f64]| -> (Vec<f64>, Vec<f64>) { (p.to_vec(), q.to_vec()) };
        let mut q = vec![1.0_f64];
        let mut p = vec![0.0_f64];
        let e0 = q[0] * q[0] / 2.0 + p[0] * p[0] / 2.0;
        for _ in 0..1000 {
            let (qn, pn) = integrator.step(&q, &p, &grad_h);
            q = qn;
            p = pn;
        }
        let e1 = q[0] * q[0] / 2.0 + p[0] * p[0] / 2.0;
        assert!(
            (e1 - e0).abs() < 1e-3,
            "Energy drift too large: {}",
            (e1 - e0).abs()
        );
    }
    #[test]
    fn test_electromagnetic_field_energy_density() {
        let mut field = ElectromagneticField::new();
        field.e = vec![1.0, 0.0, 0.0];
        field.b = vec![0.0, 0.0, 0.0];
        let u = field.energy_density();
        let expected = EPSILON_0 / 2.0;
        assert!((u - expected).abs() < 1e-25);
    }
    #[test]
    fn test_electromagnetic_field_lorentz_force() {
        let mut field = ElectromagneticField::new();
        field.e = vec![1.0, 0.0, 0.0];
        field.b = vec![0.0, 0.0, 1.0];
        let f = field.lorentz_force(1.0, &[0.0, 1.0, 0.0]);
        assert!((f[0] - 2.0).abs() < 1e-12);
        assert!(f[1].abs() < 1e-12);
        assert!(f[2].abs() < 1e-12);
    }
    #[test]
    fn test_electromagnetic_field_poynting_vector() {
        let mut field = ElectromagneticField::new();
        field.e = vec![1.0, 0.0, 0.0];
        field.b = vec![0.0, 1.0, 0.0];
        let s = field.poynting_vector();
        assert!(s[0].abs() < 1e-12);
        assert!(s[1].abs() < 1e-12);
        assert!((s[2] - 1.0 / MU_0).abs() < 1.0);
        assert!(s[2] > 0.0);
    }
    #[test]
    fn test_geodesic_equation_flat() {
        let geo = GeodesicEquation::new(4);
        assert_eq!(geo.christoffel_symbol(0, 1, 2), 0.0);
        assert_eq!(geo.geodesic_deviation(), 0.0);
    }
    #[test]
    fn test_build_mathematical_physics_env() {
        let env = build_mathematical_physics_env();
        let _ = env;
    }
    #[test]
    fn test_hamiltonian_system_sho_energy() {
        let mut sys = HamiltonianSystem::new(vec![1.0], vec![0.0], 0.01);
        let grad_h = |q: &[f64], p: &[f64]| -> (Vec<f64>, Vec<f64>) { (p.to_vec(), q.to_vec()) };
        let e0 = sys.q[0] * sys.q[0] / 2.0 + sys.p[0] * sys.p[0] / 2.0;
        let (qs, ps) = sys.run(500, &grad_h);
        let q_fin = qs.last().expect("last should succeed")[0];
        let p_fin = ps.last().expect("last should succeed")[0];
        let e1 = q_fin * q_fin / 2.0 + p_fin * p_fin / 2.0;
        assert!((e1 - e0).abs() < 1e-2, "Energy drift: {}", (e1 - e0).abs());
    }
    #[test]
    fn test_hamiltonian_system_kinetic_energy() {
        let sys = HamiltonianSystem::new(vec![0.0], vec![2.0], 0.1);
        let ke = sys.kinetic_energy(&[1.0]);
        assert!((ke - 2.0).abs() < 1e-12);
    }
    #[test]
    fn test_schrodinger_propagator_norm_conservation() {
        let n = 32;
        let dx = 0.1;
        let prop = SchrodingerPropagator::new(n, dx, 1.0, 0.001, vec![0.0; n]);
        let mut psi_re: Vec<f64> = (0..n)
            .map(|i| (-(((i as f64) - 16.0) * dx).powi(2) / 2.0).exp())
            .collect();
        let mut psi_im = vec![0.0f64; n];
        let norm0 = prop.norm_squared(&psi_re, &psi_im);
        let scale = norm0.sqrt();
        psi_re.iter_mut().for_each(|x| *x /= scale);
        psi_im.iter_mut().for_each(|x| *x /= scale);
        for _ in 0..100 {
            let (r, i) = prop.step(&psi_re, &psi_im);
            psi_re = r;
            psi_im = i;
        }
        let norm1 = prop.norm_squared(&psi_re, &psi_im);
        assert!((norm1 - 1.0).abs() < 0.1, "Norm drift: {}", norm1);
    }
    #[test]
    fn test_fock_space_vec_vacuum() {
        let vac = FockSpaceVec::vacuum(4);
        assert_eq!(vac.total_number(), 0);
        assert_eq!(vac.number_op(0), 0);
    }
    #[test]
    fn test_fock_space_creation_annihilation() {
        let vac = FockSpaceVec::vacuum(3);
        let (state1, norm1) = vac
            .apply_creation(1)
            .expect("apply_creation should succeed");
        assert_eq!(state1.occupations[1], 1);
        assert!((norm1 - 1.0).abs() < 1e-12);
        let (state2, norm2) = state1
            .apply_creation(1)
            .expect("apply_creation should succeed");
        assert_eq!(state2.occupations[1], 2);
        assert!((norm2 - 2.0_f64.sqrt()).abs() < 1e-12);
        let (state3, norm3) = state2
            .apply_annihilation(1)
            .expect("apply_annihilation should succeed");
        assert_eq!(state3.occupations[1], 1);
        assert!((norm3 - 2.0_f64.sqrt()).abs() < 1e-12);
        assert!(vac.apply_annihilation(2).is_none());
    }
    #[test]
    fn test_fock_space_inner_product() {
        let v1 = FockSpaceVec {
            occupations: vec![1, 0, 2],
        };
        let v2 = FockSpaceVec {
            occupations: vec![1, 0, 2],
        };
        let v3 = FockSpaceVec {
            occupations: vec![0, 1, 2],
        };
        assert_eq!(v1.inner_product(&v2), 1.0);
        assert_eq!(v1.inner_product(&v3), 0.0);
    }
    #[test]
    fn test_gauge_field_cold_start_zero_action() {
        let gf = GaugeField::new_cold(4, 1.0);
        assert!((gf.action()).abs() < 1e-12);
    }
    #[test]
    fn test_gauge_field_polyakov_loop() {
        let gf = GaugeField::new_cold(4, 1.0);
        assert!((gf.polyakov_loop(0) - 1.0).abs() < 1e-12);
    }
    #[test]
    fn test_kdv_soliton_eval() {
        let sol = KdVSoliton::new(2.0, 0.0);
        let u = sol.eval(0.0, 0.0);
        assert!((u - (-2.0)).abs() < 1e-12, "u = {}", u);
    }
    #[test]
    fn test_kdv_soliton_speed_amplitude() {
        let sol = KdVSoliton::new(3.0, 1.0);
        assert!((sol.speed() - 9.0).abs() < 1e-12);
        assert!((sol.amplitude() - (-4.5)).abs() < 1e-12);
    }
    #[test]
    fn test_kdv_soliton_grid() {
        let sol = KdVSoliton::new(1.0, 0.0);
        let xs = vec![-10.0, 0.0, 10.0];
        let vals = sol.eval_grid(&xs, 0.0);
        assert!(vals[0].abs() < 1e-3);
        assert!(vals[2].abs() < 1e-3);
        assert!(vals[1] < vals[0]);
    }
}
