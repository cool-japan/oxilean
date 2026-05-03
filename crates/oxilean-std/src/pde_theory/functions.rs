//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    BurgersRiemann, CurvatureFlowSim, DeGiorgiNashMoser, DispersiveEstimateChecker, HeatSolver1D,
    HomogenizationApprox, KPZEquation, Mesh1D, MinimalSurface, NonlinearSchrodinger,
    ParabolicSolver, PseudodiffOperatorSim, SchauderEstimate, StiffnessMatrix1D,
    StochasticHeatEquation, StrichartzData,
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
pub fn set_of(ty: Expr) -> Expr {
    app(cst("Set"), ty)
}
pub fn fn_ty(dom: Expr, cod: Expr) -> Expr {
    arrow(dom, cod)
}
/// `SobolevSpace : Nat → Nat → Type`
///
/// W^{k,p}(Ω): the Sobolev space of functions whose weak derivatives up to
/// order k lie in L^p(Ω). Here Nat encodes the order k and the exponent p.
pub fn sobolev_space_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `L2Space : Type`
///
/// L²(Ω): square-integrable functions on a domain Ω. This is W^{0,2}(Ω).
pub fn l2_space_ty() -> Expr {
    type0()
}
/// `H1Space : Type`
///
/// H¹(Ω) = W^{1,2}(Ω): Sobolev space of L² functions with L² weak gradient.
pub fn h1_space_ty() -> Expr {
    type0()
}
/// `H10Space : Type`
///
/// H¹₀(Ω): closure of C∞_c(Ω) in H¹(Ω); functions vanishing on ∂Ω.
pub fn h10_space_ty() -> Expr {
    type0()
}
/// `HkSpace : Nat → Type`
///
/// H^k(Ω) = W^{k,2}(Ω) for arbitrary order k.
pub fn hk_space_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `WeakDerivative : (L2Space → L2Space) → Nat → L2Space → Prop`
///
/// D^α u = v in the weak sense: ∀ φ ∈ C∞_c, ∫ u ∂^α φ = (-1)^|α| ∫ v φ.
pub fn weak_derivative_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "V",
        type0(),
        arrow(nat_ty(), arrow(cst("L2Space"), prop())),
    )
}
/// `SobolevNorm : H1Space → Real`
///
/// ‖u‖_{H¹} = (‖u‖²_{L²} + ‖∇u‖²_{L²})^{1/2}.
pub fn sobolev_norm_ty() -> Expr {
    arrow(cst("H1Space"), real_ty())
}
/// `SobolevEmbedding : Nat → Nat → Prop`
///
/// W^{k,p}(Ω) ↪ W^{m,q}(Ω) continuously when the Sobolev exponents satisfy
/// the standard embedding condition.
pub fn sobolev_embedding_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `TraceMap : H1Space → L2Space`
///
/// The boundary trace operator γ: H¹(Ω) → L²(∂Ω), extending the restriction
/// to ∂Ω for smooth functions to all of H¹(Ω).
pub fn trace_map_ty() -> Expr {
    arrow(cst("H1Space"), cst("L2Space"))
}
/// `EllipticOperator : Type`
///
/// A second-order linear differential operator L = -div(A∇·) + b·∇ + c
/// satisfying the ellipticity condition: ∃ λ > 0, ξᵀA(x)ξ ≥ λ|ξ|² a.e.
pub fn elliptic_operator_ty() -> Expr {
    type0()
}
/// `ParabolicOperator : Type`
///
/// An operator of the form ∂_t - L where L is elliptic; governs parabolic PDEs.
pub fn parabolic_operator_ty() -> Expr {
    type0()
}
/// `HyperbolicOperator : Type`
///
/// An operator of the form ∂²_t - c²Δ or more generally ∂²_t - L; governs
/// wave-type equations.
pub fn hyperbolic_operator_ty() -> Expr {
    type0()
}
/// `IsElliptic : EllipticOperator → Prop`
///
/// The uniform ellipticity condition: ∃ 0 < λ ≤ Λ such that
/// λ|ξ|² ≤ ξᵀA(x)ξ ≤ Λ|ξ|² for a.e. x and all ξ ∈ ℝⁿ.
pub fn is_elliptic_ty() -> Expr {
    arrow(cst("EllipticOperator"), prop())
}
/// `BilinearForm : H1Space → H1Space → Real`
///
/// A continuous bilinear form a: V × V → ℝ arising in the weak formulation
/// of an elliptic PDE.
pub fn bilinear_form_ty() -> Expr {
    arrow(cst("H1Space"), arrow(cst("H1Space"), real_ty()))
}
/// `IsCoercive : BilinearForm → Prop`
///
/// Coercivity: ∃ α > 0 such that a(u, u) ≥ α ‖u‖²_V for all u ∈ V.
pub fn is_coercive_ty() -> Expr {
    arrow(cst("BilinearForm"), prop())
}
/// `IsContinuous_bf : BilinearForm → Prop`
///
/// Continuity of the bilinear form: ∃ M > 0, |a(u, v)| ≤ M ‖u‖ ‖v‖.
pub fn is_continuous_bf_ty() -> Expr {
    arrow(cst("BilinearForm"), prop())
}
/// `WeakSolution : EllipticOperator → L2Space → H10Space → Prop`
///
/// u ∈ H¹₀(Ω) is a weak solution of Lu = f if
/// a(u, v) = ⟨f, v⟩ for all v ∈ H¹₀(Ω).
pub fn weak_solution_ty() -> Expr {
    arrow(
        cst("EllipticOperator"),
        arrow(cst("L2Space"), arrow(cst("H10Space"), prop())),
    )
}
/// `WeakSolutionHeat : ParabolicOperator → L2Space → (Real → H10Space) → Prop`
///
/// u ∈ L²(0,T; H¹₀) with ∂_t u ∈ L²(0,T; H⁻¹) is a weak solution of
/// ∂_t u - Δu = f with u(0) = u₀.
pub fn weak_solution_heat_ty() -> Expr {
    arrow(
        cst("ParabolicOperator"),
        arrow(
            cst("L2Space"),
            arrow(fn_ty(real_ty(), cst("H10Space")), prop()),
        ),
    )
}
/// `WeakSolutionWave : HyperbolicOperator → L2Space → H10Space → (Real → H10Space) → Prop`
///
/// Weak solution of the wave equation with initial data (u₀, u₁).
pub fn weak_solution_wave_ty() -> Expr {
    arrow(
        cst("HyperbolicOperator"),
        arrow(
            cst("L2Space"),
            arrow(
                cst("H10Space"),
                arrow(fn_ty(real_ty(), cst("H10Space")), prop()),
            ),
        ),
    )
}
/// `LaxMilgram : ∀ (V : HilbertSpace) (a : BilinearForm V) (f : V → ℝ),
///   IsCoercive a → IsContinuous a → ∃! u : V, ∀ v, a(u, v) = f(v)`
///
/// The Lax-Milgram lemma: given a coercive continuous bilinear form on a
/// Hilbert space V and a bounded linear functional f ∈ V*, there exists a
/// unique u ∈ V solving the variational problem a(u, v) = f(v) for all v.
pub fn lax_milgram_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "V",
        type0(),
        arrow(
            app(cst("IsCoercive"), bvar(0)),
            arrow(app(cst("IsContinuous_bf"), bvar(1)), prop()),
        ),
    )
}
/// `EllipticRegularity : WeakSolution Lu f → f ∈ Hk → u ∈ H(k+2)`
///
/// Interior elliptic regularity: if f ∈ H^k(Ω) and u is a weak solution of
/// Lu = f, then u ∈ H^{k+2}_{loc}(Ω).
pub fn elliptic_regularity_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        nat_ty(),
        arrow(
            app3(
                cst("WeakSolution"),
                cst("EllipticOperator"),
                cst("L2Space"),
                cst("H10Space"),
            ),
            prop(),
        ),
    )
}
/// `MaximumPrinciple : Lu ≥ 0 in Ω → u|_{∂Ω} ≤ 0 → u ≤ 0 in Ω`
///
/// The maximum principle for elliptic operators: if Lu ≥ 0 (sub-solution)
/// and u ≤ 0 on the boundary, then u ≤ 0 in the interior.
pub fn maximum_principle_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "L",
        cst("EllipticOperator"),
        arrow(app(cst("IsElliptic"), bvar(0)), prop()),
    )
}
/// `SobolevEmbeddingThm : W^{1,p}(Ω) ↪ L^q(Ω) for 1/q = 1/p - 1/n`
///
/// The Sobolev embedding theorem: W^{k,p}(Ω) embeds continuously into
/// L^q(Ω) (and into C^{k-n/p} when k > n/p).
pub fn sobolev_embedding_thm_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), prop())))
}
/// `PoincareInequality : ∀ u ∈ H¹₀(Ω), ‖u‖_{L²} ≤ C ‖∇u‖_{L²}`
///
/// The Poincaré inequality on bounded domains: the L² norm of u is controlled
/// by the L² norm of its gradient when u vanishes on the boundary.
pub fn poincare_inequality_ty() -> Expr {
    arrow(cst("H10Space"), prop())
}
/// `HeatEquationExistence : ∀ u₀ ∈ L², f ∈ L²(0,T;H⁻¹),
///   ∃ u ∈ L²(0,T;H¹₀) ∩ C(0,T;L²), weak solution of heat equation`
///
/// Existence and uniqueness of weak solutions to the heat equation.
pub fn heat_equation_existence_ty() -> Expr {
    arrow(
        cst("L2Space"),
        arrow(fn_ty(real_ty(), cst("H10Space")), prop()),
    )
}
/// `WaveEquationExistence : ∀ (u₀, u₁) ∈ H¹₀ × L², ∃ weak solution of wave equation`
///
/// Existence of weak solutions to the wave equation with finite energy.
pub fn wave_equation_existence_ty() -> Expr {
    arrow(cst("H10Space"), arrow(cst("L2Space"), prop()))
}
/// `NavierStokesWeakExistence : ∀ u₀ ∈ L²(div=0), f ∈ L²(0,T;H⁻¹),
///   ∃ u (Leray-Hopf weak solution) of Navier-Stokes`
///
/// The Leray-Hopf existence theorem: for divergence-free initial data and
/// forcing, there exists a global Leray-Hopf weak solution of the
/// incompressible Navier-Stokes equations.
pub fn navier_stokes_weak_existence_ty() -> Expr {
    arrow(
        cst("L2Space"),
        arrow(fn_ty(real_ty(), cst("H10Space")), prop()),
    )
}
/// `RiemannProblemSolution : ConservationLaw → InitialData → Prop`
///
/// The Riemann problem for a scalar conservation law u_t + f(u)_x = 0:
/// entropy solutions exist and are given by shock/rarefaction fans.
pub fn riemann_problem_solution_ty() -> Expr {
    arrow(cst("ConservationLaw"), arrow(cst("InitialData"), prop()))
}
/// `GalerkinApproximation : H10Space → Nat → H10Space`
///
/// The Galerkin approximation u_N ∈ V_N (finite-dimensional subspace of V)
/// solving a(u_N, v_N) = f(v_N) for all v_N ∈ V_N.
pub fn galerkin_approximation_ty() -> Expr {
    arrow(cst("H10Space"), arrow(nat_ty(), cst("H10Space")))
}
/// `CeaLemma : ∀ u exact, u_N Galerkin, ‖u - u_N‖_V ≤ (M/α) inf_{v_N ∈ V_N} ‖u - v_N‖_V`
///
/// Céa's lemma: the Galerkin approximation is quasi-optimal in the energy norm,
/// with quasi-optimality constant M/α (continuity over coercivity constants).
pub fn cea_lemma_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "u",
        cst("H10Space"),
        arrow(
            app2(cst("GalerkinApproximation"), bvar(0), cst("Nat")),
            prop(),
        ),
    )
}
/// `FiniteElementSpace : Nat → Nat → Type`
///
/// P_k finite element space on a mesh of mesh-size h (encoded as Nat):
/// piecewise polynomial functions of degree k.
pub fn finite_element_space_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `AprioriError : ‖u - u_h‖_{H¹} ≤ C h^k |u|_{H^{k+1}}`
///
/// A priori error estimate for finite element methods: the H¹ error between
/// the exact solution u and the finite element approximation u_h is O(h^k).
pub fn apriori_error_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), prop()))
}
/// `InfSupCondition : BilinearForm → Prop`
///
/// The inf-sup (Ladyzhenskaya-Babuška-Brezzi) condition for mixed methods:
/// inf_{q ≠ 0} sup_{v ≠ 0} b(v,q)/(‖v‖ ‖q‖) ≥ β > 0.
pub fn inf_sup_condition_ty() -> Expr {
    arrow(cst("BilinearForm"), prop())
}
/// `ConservationLaw : (Real → Real) → Type`
///
/// A scalar conservation law u_t + f(u)_x = 0 determined by its flux f.
pub fn conservation_law_ty() -> Expr {
    arrow(fn_ty(real_ty(), real_ty()), type0())
}
/// `EntropySolution : ConservationLaw → InitialData → (Real → Real → Real) → Prop`
///
/// An entropy (Kružkov) solution: a weak solution satisfying the entropy
/// inequality for all convex entropies η.
pub fn entropy_solution_ty() -> Expr {
    arrow(
        cst("ConservationLaw"),
        arrow(
            cst("InitialData"),
            arrow(fn_ty(real_ty(), fn_ty(real_ty(), real_ty())), prop()),
        ),
    )
}
/// `KruzkovTheorem : ∀ u₀ ∈ L∞ ∩ BV, ∃! entropy solution of conservation law`
///
/// Kružkov's existence and uniqueness theorem for scalar conservation laws
/// with bounded, BV initial data.
pub fn kruzkov_theorem_ty() -> Expr {
    arrow(cst("L2Space"), prop())
}
/// `RankineHugoniot : shock speed = [f(u)] / \[u\]`
///
/// The Rankine-Hugoniot condition: at a shock discontinuity, the shock speed s
/// satisfies s \[u\] = [f(u)] where [·] denotes the jump across the discontinuity.
pub fn rankine_hugoniot_ty() -> Expr {
    arrow(cst("ConservationLaw"), arrow(real_ty(), prop()))
}
/// `StrichartzEstimate : SolutionSpace → Prop`
///
/// The Strichartz estimate: for a solution u of the linear Schrödinger equation,
/// ‖u‖_{L^q_t L^r_x} ≤ C ‖u₀‖_{L²} where (q, r) is an admissible Strichartz pair.
pub fn strichartz_estimate_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "u",
        cst("SolutionSpace"),
        app2(
            cst("Le"),
            app(cst("StrichartzNorm"), bvar(0)),
            app(cst("InitialDataNorm"), bvar(0)),
        ),
    )
}
/// `AdmissiblePair : Nat → Nat → Prop`
///
/// A pair (q, r) is Strichartz-admissible for the d-dimensional Schrödinger equation
/// when 2/q + d/r = d/2 with q ≥ 2 and (q, r, d) ≠ (2, ∞, 2).
pub fn admissible_pair_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `DispersiveEstimate : Real → Real → Prop`
///
/// The dispersive decay estimate: ‖e^{itΔ} u₀‖_{L^∞} ≤ C |t|^{-d/2} ‖u₀‖_{L¹}.
pub fn dispersive_estimate_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), prop()))
}
/// `KdVSoliton : Real → Real → Real → Real`
///
/// The KdV soliton solution u(x, t) = -2κ² sech²(κ(x - 4κ²t - x₀)).
pub fn kdv_soliton_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), real_ty())))
}
/// `NLSGlobalWellPosedness : H1Space → Prop`
///
/// Global well-posedness for the defocusing cubic NLS in 3D: given initial data
/// u₀ ∈ H¹(ℝ³), there exists a unique global solution u ∈ C(ℝ; H¹).
pub fn nls_gwp_ty() -> Expr {
    arrow(cst("H1Space"), prop())
}
/// `NLSBlowup : H1Space → Real → Prop`
///
/// Blowup for the focusing NLS: the solution with initial data u₀ blows up
/// at time T* < ∞ (‖∇u(t)‖_{L²} → ∞ as t → T*).
pub fn nls_blowup_ty() -> Expr {
    arrow(cst("H1Space"), arrow(real_ty(), prop()))
}
/// `ParaproductDecomposition : L2Space → L2Space → L2Space`
///
/// The Bony paraproduct decomposition: for two functions f, g in L²,
/// fg = T_f g + T_g f + R(f, g) where T, R are the paraproduct and remainder.
pub fn paraproduct_decomposition_ty() -> Expr {
    arrow(cst("L2Space"), arrow(cst("L2Space"), cst("L2Space")))
}
/// `BilinearEstimate : L2Space → L2Space → Real → Prop`
///
/// A bilinear estimate asserting that ‖T(u, v)‖_X ≤ C ‖u‖_Y ‖v‖_Z for
/// suitable function space norms X, Y, Z.
pub fn bilinear_estimate_ty() -> Expr {
    arrow(
        cst("L2Space"),
        arrow(cst("L2Space"), arrow(real_ty(), prop())),
    )
}
/// `LittlewoodPaleyProjection : Nat → L2Space → L2Space`
///
/// The Littlewood-Paley dyadic frequency projection P_N onto frequencies |ξ| ~ N.
pub fn littlewood_paley_projection_ty() -> Expr {
    arrow(nat_ty(), arrow(cst("L2Space"), cst("L2Space")))
}
/// `CalderonZygmundOperator : Type`
///
/// A singular integral operator T with kernel K satisfying the standard estimates
/// |K(x,y)| ≤ C|x-y|^{-d} and |∇_x K| + |∇_y K| ≤ C|x-y|^{-d-1}.
pub fn calderon_zygmund_operator_ty() -> Expr {
    type0()
}
/// `CZBoundedness : CalderonZygmundOperator → Prop`
///
/// L^p boundedness of a Calderon-Zygmund operator: T is bounded on L^p for 1 < p < ∞.
pub fn cz_boundedness_ty() -> Expr {
    arrow(cst("CalderonZygmundOperator"), prop())
}
/// `WeakType11 : CalderonZygmundOperator → Prop`
///
/// Weak (1,1) boundedness: m{x : |Tf(x)| > λ} ≤ C/λ ‖f‖_{L¹}.
pub fn weak_type_11_ty() -> Expr {
    arrow(cst("CalderonZygmundOperator"), prop())
}
/// `HardyLittlewoodMaximal : L2Space → L2Space`
///
/// The Hardy-Littlewood maximal operator Mf(x) = sup_{r>0} (1/|B_r|) ∫_{B_r} |f|.
pub fn hardy_littlewood_maximal_ty() -> Expr {
    arrow(cst("L2Space"), cst("L2Space"))
}
/// `HilbertTransform : L2Space → L2Space`
///
/// The Hilbert transform Hf(x) = p.v. ∫ f(y)/(x-y) dy, the canonical
/// example of a Calderon-Zygmund singular integral.
pub fn hilbert_transform_ty() -> Expr {
    arrow(cst("L2Space"), cst("L2Space"))
}
/// `SymbolClass : Nat → Nat → Type`
///
/// The Hörmander symbol class S^m_{ρ,δ}: symbols a(x,ξ) satisfying
/// |∂^α_ξ ∂^β_x a| ≤ C_{α,β} ⟨ξ⟩^{m - ρ|α| + δ|β|}.
pub fn symbol_class_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `PseudodiffOperator : SymbolClass → Type`
///
/// A pseudodifferential operator Op(a) with symbol a ∈ S^m_{ρ,δ}, defined by
/// Op(a)u(x) = (2π)^{-d} ∫ e^{ix·ξ} a(x,ξ) û(ξ) dξ.
pub fn pseudodiff_operator_ty() -> Expr {
    arrow(cst("SymbolClass"), type0())
}
/// `PseudodiffComposition : PseudodiffOperator → PseudodiffOperator → PseudodiffOperator`
///
/// Composition of pseudodifferential operators: Op(a) ∘ Op(b) = Op(a#b)
/// where a#b is the asymptotic expansion of the composed symbol.
pub fn pseudodiff_composition_ty() -> Expr {
    arrow(
        cst("PseudodiffOperator"),
        arrow(cst("PseudodiffOperator"), cst("PseudodiffOperator")),
    )
}
/// `FourierIntegralOperator : Type`
///
/// A Fourier integral operator (FIO) with phase φ(x,ξ) and amplitude a(x,ξ),
/// generalizing pseudodifferential operators to include propagation effects.
pub fn fourier_integral_operator_ty() -> Expr {
    type0()
}
/// `WavefrontSet : L2Space → Type`
///
/// The wavefront set WF(u) ⊂ T*X \ 0: the set of (x,ξ) pairs where u is
/// not microlocally smooth (not C^∞ in any conic neighborhood of ξ).
pub fn wavefront_set_ty() -> Expr {
    arrow(cst("L2Space"), type0())
}
/// `EllipticRegularityMicro : PseudodiffOperator → L2Space → Prop`
///
/// Microlocal elliptic regularity: if a ∈ S^m is elliptic at (x₀, ξ₀) and
/// Pu = f is microlocally smooth at (x₀, ξ₀), then so is u.
pub fn elliptic_regularity_micro_ty() -> Expr {
    arrow(cst("PseudodiffOperator"), arrow(cst("L2Space"), prop()))
}
/// `PropagationOfSingularities : HyperbolicOperator → L2Space → Prop`
///
/// The Hörmander propagation of singularities theorem: WF(u) \ WF(Pu) is
/// invariant under the Hamilton flow of the principal symbol of P.
pub fn propagation_of_singularities_ty() -> Expr {
    arrow(cst("HyperbolicOperator"), arrow(cst("L2Space"), prop()))
}
/// `LadyzhenskayaProdiSerrin : Real → Real → Prop`
///
/// The Ladyzhenskaya-Prodi-Serrin regularity criterion: if u ∈ L^p(0,T; L^q)
/// with 2/p + 3/q ≤ 1 and q ≥ 3, then u is a strong solution (no blowup).
pub fn ladyzhenskaya_prodi_serrin_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), prop()))
}
/// `CaffarelliKohnNirenberg : L2Space → Prop`
///
/// The Caffarelli-Kohn-Nirenberg partial regularity theorem: the singular set
/// of a Leray-Hopf weak solution has parabolic Hausdorff dimension at most 1.
pub fn caffarelli_kohn_nirenberg_ty() -> Expr {
    arrow(cst("L2Space"), prop())
}
/// `VorticityFormulation : L2Space → L2Space`
///
/// The vorticity ω = ∇ × u of a velocity field u, satisfying the vorticity
/// equation ω_t + (u·∇)ω = (ω·∇)u + νΔω.
pub fn vorticity_formulation_ty() -> Expr {
    arrow(cst("L2Space"), cst("L2Space"))
}
/// `RicciFlowSolution : Type`
///
/// A solution to the Ricci flow ∂_t g_{ij} = -2 R_{ij} on a Riemannian
/// manifold, driving the metric toward constant curvature.
pub fn ricci_flow_solution_ty() -> Expr {
    type0()
}
/// `MeanCurvatureFlowSolution : Type`
///
/// A solution to mean curvature flow: a family of hypersurfaces Σ_t evolving
/// by d/dt x = H·ν where H is the mean curvature and ν is the unit normal.
pub fn mean_curvature_flow_solution_ty() -> Expr {
    type0()
}
/// `ShortTimeExistenceGeomFlow : RicciFlowSolution → Prop`
///
/// Short-time existence for geometric flows: for smooth initial data (compact
/// manifold), there exists T > 0 and a smooth solution on [0, T).
pub fn short_time_existence_geom_flow_ty() -> Expr {
    arrow(cst("RicciFlowSolution"), prop())
}
/// `NeckPinchSingularity : MeanCurvatureFlowSolution → Real → Prop`
///
/// Neck-pinch singularity formation: under mean curvature flow, a cylindrical
/// neck can pinch off in finite time T, with blow-up profile ~(T-t)^{-1/2}.
pub fn neck_pinch_singularity_ty() -> Expr {
    arrow(cst("MeanCurvatureFlowSolution"), arrow(real_ty(), prop()))
}
/// `FreeBoundary : Type`
///
/// A free boundary: an unknown interface Γ(t) = ∂{u > 0} separating regions
/// where different PDE regimes apply (e.g., contact set in obstacle problem).
pub fn free_boundary_ty() -> Expr {
    type0()
}
/// `ObstacleProblem : H10Space → L2Space → FreeBoundary → Prop`
///
/// The obstacle problem: find u ∈ K = {v ∈ H¹₀: v ≥ φ} minimizing the
/// Dirichlet energy; the free boundary ∂{u > φ} has C^{1,α} regularity.
pub fn obstacle_problem_ty() -> Expr {
    arrow(
        cst("H10Space"),
        arrow(cst("L2Space"), arrow(cst("FreeBoundary"), prop())),
    )
}
/// `StefanProblem : FreeBoundary → Real → Prop`
///
/// The Stefan problem for phase transitions: heat equation in each phase with
/// Stefan condition v_n = \[∂_n u\] on the free boundary (latent heat release).
pub fn stefan_problem_ty() -> Expr {
    arrow(cst("FreeBoundary"), arrow(real_ty(), prop()))
}
/// `FreeBoundaryRegularity : FreeBoundary → Prop`
///
/// C^{1,α} regularity of the free boundary in the obstacle problem away from
/// degenerate points; Caffarelli's Alt-Caffarelli-Friedman monotonicity formula.
pub fn free_boundary_regularity_ty() -> Expr {
    arrow(cst("FreeBoundary"), prop())
}
/// `HamiltonJacobiEquation : Type`
///
/// A Hamilton-Jacobi equation: ∂_t u + H(x, ∇u) = 0 or the static version
/// H(x, ∇u) = 0, determined by the Hamiltonian H: Ω × ℝⁿ → ℝ.
pub fn hamilton_jacobi_equation_ty() -> Expr {
    type0()
}
/// `ViscositySolution : HamiltonJacobiEquation → L2Space → Prop`
///
/// A viscosity solution in the sense of Crandall-Lions: u is a sub- and
/// super-solution with all test-function inequalities satisfied.
pub fn viscosity_solution_ty() -> Expr {
    arrow(cst("HamiltonJacobiEquation"), arrow(cst("L2Space"), prop()))
}
/// `CrandallLionsUniqueness : HamiltonJacobiEquation → Prop`
///
/// Uniqueness of viscosity solutions: under appropriate coercivity and
/// continuity of H, the comparison principle gives uniqueness.
pub fn crandall_lions_uniqueness_ty() -> Expr {
    arrow(cst("HamiltonJacobiEquation"), prop())
}
/// `HopfLaxFormula : HamiltonJacobiEquation → Real → Real → Real`
///
/// The Hopf-Lax formula: u(x, t) = min_y { g(y) + t H*(( x-y)/t) }
/// where H* is the Legendre-Fenchel conjugate of H.
pub fn hopf_lax_formula_ty() -> Expr {
    arrow(
        cst("HamiltonJacobiEquation"),
        arrow(real_ty(), arrow(real_ty(), real_ty())),
    )
}
/// `HomogenizedCoefficient : Real → Real`
///
/// The effective/homogenized coefficient A_hom arising from the two-scale limit
/// of oscillating coefficients A(x/ε) in the elliptic equation -div(A(x/ε)∇u_ε) = f.
pub fn homogenized_coefficient_ty() -> Expr {
    arrow(real_ty(), real_ty())
}
/// `TwoScaleConvergence : L2Space → L2Space → Prop`
///
/// Two-scale convergence: u_ε →₂ u₀(x,y) if for all admissible test functions,
/// ∫ u_ε(x) φ(x, x/ε) dx → ∫∫ u₀(x,y) φ(x,y) dx dy.
pub fn two_scale_convergence_ty() -> Expr {
    arrow(cst("L2Space"), arrow(cst("L2Space"), prop()))
}
/// `GammaConvergence : Type`
///
/// Γ-convergence: a variational notion of convergence for functionals F_ε → F_hom
/// ensuring that minimizers u_ε of F_ε converge to minimizers of F_hom.
pub fn gamma_convergence_ty() -> Expr {
    type0()
}
/// `HomogenizationThm : Real → L2Space → Prop`
///
/// The homogenization theorem: as ε → 0, the solution u_ε of the oscillating
/// problem converges weakly in H¹ to the solution of the homogenized equation.
pub fn homogenization_thm_ty() -> Expr {
    arrow(real_ty(), arrow(cst("L2Space"), prop()))
}
/// `ReactionDiffusionSystem : Type`
///
/// A reaction-diffusion system ∂_t u = D Δu + F(u) where D is a diffusion
/// matrix and F: ℝⁿ → ℝⁿ is the reaction term.
pub fn reaction_diffusion_system_ty() -> Expr {
    type0()
}
/// `TuringInstability : ReactionDiffusionSystem → Prop`
///
/// Turing instability: a homogeneous steady state that is stable without diffusion
/// becomes unstable when diffusion is added (diffusion-driven instability).
pub fn turing_instability_ty() -> Expr {
    arrow(cst("ReactionDiffusionSystem"), prop())
}
/// `TravelingWave : ReactionDiffusionSystem → Real → Prop`
///
/// A traveling wave solution u(x,t) = U(x - ct) with wave speed c,
/// connecting two steady states of the reaction-diffusion system.
pub fn traveling_wave_ty() -> Expr {
    arrow(cst("ReactionDiffusionSystem"), arrow(real_ty(), prop()))
}
/// `FisherKPPEquation : Real → Real → Prop`
///
/// The Fisher-KPP equation u_t = Δu + f(u) with f(0)=f(1)=0, f'(0)>0:
/// minimal wave speed c* = 2√(D f'(0)).
pub fn fisher_kpp_equation_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), prop()))
}
/// `VariationalInequality : H10Space → L2Space → Prop`
///
/// A variational inequality: find u ∈ K (closed convex set in V) such that
/// a(u, v-u) ≥ ⟨f, v-u⟩ for all v ∈ K.
pub fn variational_inequality_ty() -> Expr {
    arrow(cst("H10Space"), arrow(cst("L2Space"), prop()))
}
/// `ConvexMinimizer : H10Space → L2Space → H10Space`
///
/// The unique minimizer of a strictly convex coercive functional J(v) = (1/2)a(v,v) - ⟨f,v⟩
/// over a closed convex set K ⊂ V.
pub fn convex_minimizer_ty() -> Expr {
    arrow(cst("H10Space"), arrow(cst("L2Space"), cst("H10Space")))
}
/// `PenaltyMethodConvergence : VariationalInequality → Real → Prop`
///
/// Convergence of the penalty method: as the penalty parameter ε → 0, the
/// penalized solutions u_ε converge strongly in V to the solution u of the VI.
pub fn penalty_method_convergence_ty() -> Expr {
    arrow(cst("VariationalInequality"), arrow(real_ty(), prop()))
}
/// Register all PDE theory axioms into the kernel environment.
pub fn build_pde_theory_env(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("L2Space", type0()),
        ("H1Space", type0()),
        ("H10Space", type0()),
        ("BilinearForm", type0()),
        ("ConservationLaw", type0()),
        ("InitialData", type0()),
        ("SobolevSpace", sobolev_space_ty()),
        ("HkSpace", hk_space_ty()),
        ("WeakDerivative", weak_derivative_ty()),
        ("SobolevNorm", sobolev_norm_ty()),
        ("SobolevEmbedding", sobolev_embedding_ty()),
        ("TraceMap", trace_map_ty()),
        ("EllipticOperator", elliptic_operator_ty()),
        ("ParabolicOperator", parabolic_operator_ty()),
        ("HyperbolicOperator", hyperbolic_operator_ty()),
        ("IsElliptic", is_elliptic_ty()),
        ("IsCoercive", is_coercive_ty()),
        ("IsContinuous_bf", is_continuous_bf_ty()),
        ("WeakSolution", weak_solution_ty()),
        ("WeakSolutionHeat", weak_solution_heat_ty()),
        ("WeakSolutionWave", weak_solution_wave_ty()),
        ("lax_milgram", lax_milgram_ty()),
        ("elliptic_regularity", elliptic_regularity_ty()),
        ("maximum_principle", maximum_principle_ty()),
        ("sobolev_embedding_thm", sobolev_embedding_thm_ty()),
        ("poincare_inequality", poincare_inequality_ty()),
        ("heat_equation_existence", heat_equation_existence_ty()),
        ("wave_equation_existence", wave_equation_existence_ty()),
        (
            "navier_stokes_weak_existence",
            navier_stokes_weak_existence_ty(),
        ),
        ("riemann_problem_solution", riemann_problem_solution_ty()),
        ("GalerkinApproximation", galerkin_approximation_ty()),
        ("cea_lemma", cea_lemma_ty()),
        ("FiniteElementSpace", finite_element_space_ty()),
        ("apriori_error", apriori_error_ty()),
        ("InfSupCondition", inf_sup_condition_ty()),
        ("EntropySolution", entropy_solution_ty()),
        ("kruzkov_theorem", kruzkov_theorem_ty()),
        ("rankine_hugoniot", rankine_hugoniot_ty()),
        ("SolutionSpace", type0()),
        ("StrichartzNorm", arrow(cst("SolutionSpace"), real_ty())),
        ("InitialDataNorm", arrow(cst("SolutionSpace"), real_ty())),
        ("strichartz_estimate", strichartz_estimate_ty()),
        ("AdmissiblePair", admissible_pair_ty()),
        ("dispersive_estimate", dispersive_estimate_ty()),
        ("KdVSoliton", kdv_soliton_ty()),
        ("nls_gwp", nls_gwp_ty()),
        ("nls_blowup", nls_blowup_ty()),
        ("paraproduct_decomposition", paraproduct_decomposition_ty()),
        ("bilinear_estimate", bilinear_estimate_ty()),
        (
            "LittlewoodPaleyProjection",
            littlewood_paley_projection_ty(),
        ),
        ("CalderonZygmundOperator", calderon_zygmund_operator_ty()),
        ("cz_boundedness", cz_boundedness_ty()),
        ("WeakType11", weak_type_11_ty()),
        ("HardyLittlewoodMaximal", hardy_littlewood_maximal_ty()),
        ("HilbertTransform", hilbert_transform_ty()),
        ("SymbolClass", symbol_class_ty()),
        ("PseudodiffOperator", pseudodiff_operator_ty()),
        ("pseudodiff_composition", pseudodiff_composition_ty()),
        ("FourierIntegralOperator", fourier_integral_operator_ty()),
        ("WavefrontSet", wavefront_set_ty()),
        ("elliptic_regularity_micro", elliptic_regularity_micro_ty()),
        (
            "propagation_of_singularities",
            propagation_of_singularities_ty(),
        ),
        (
            "ladyzhenskaya_prodi_serrin",
            ladyzhenskaya_prodi_serrin_ty(),
        ),
        ("caffarelli_kohn_nirenberg", caffarelli_kohn_nirenberg_ty()),
        ("VorticityFormulation", vorticity_formulation_ty()),
        ("RicciFlowSolution", ricci_flow_solution_ty()),
        (
            "MeanCurvatureFlowSolution",
            mean_curvature_flow_solution_ty(),
        ),
        (
            "short_time_existence_geom_flow",
            short_time_existence_geom_flow_ty(),
        ),
        ("neck_pinch_singularity", neck_pinch_singularity_ty()),
        ("FreeBoundary", free_boundary_ty()),
        ("obstacle_problem", obstacle_problem_ty()),
        ("stefan_problem", stefan_problem_ty()),
        ("free_boundary_regularity", free_boundary_regularity_ty()),
        ("HamiltonJacobiEquation", hamilton_jacobi_equation_ty()),
        ("viscosity_solution", viscosity_solution_ty()),
        ("crandall_lions_uniqueness", crandall_lions_uniqueness_ty()),
        ("hopf_lax_formula", hopf_lax_formula_ty()),
        ("HomogenizedCoefficient", homogenized_coefficient_ty()),
        ("two_scale_convergence", two_scale_convergence_ty()),
        ("GammaConvergence", gamma_convergence_ty()),
        ("homogenization_thm", homogenization_thm_ty()),
        ("ReactionDiffusionSystem", reaction_diffusion_system_ty()),
        ("turing_instability", turing_instability_ty()),
        ("traveling_wave", traveling_wave_ty()),
        ("fisher_kpp_equation", fisher_kpp_equation_ty()),
        ("VariationalInequality", variational_inequality_ty()),
        ("ConvexMinimizer", convex_minimizer_ty()),
        (
            "penalty_method_convergence",
            penalty_method_convergence_ty(),
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
/// Compute the discrete H¹ semi-norm |u|_{H¹} = ‖u'‖_{L²} on a 1D mesh
/// using finite differences.
pub fn discrete_h1_seminorm(u: &[f64], h: f64) -> f64 {
    if u.len() < 2 {
        return 0.0;
    }
    let sum_sq: f64 = u
        .windows(2)
        .map(|w| {
            let du = (w[1] - w[0]) / h;
            du * du * h
        })
        .sum();
    sum_sq.sqrt()
}
/// Compute the discrete L² norm ‖u‖_{L²} on a 1D mesh using the trapezoidal rule.
pub fn discrete_l2_norm(u: &[f64], h: f64) -> f64 {
    if u.is_empty() {
        return 0.0;
    }
    let n = u.len();
    let sum_sq: f64 = (0..n)
        .map(|i| {
            let w = if i == 0 || i == n - 1 { 0.5 } else { 1.0 };
            w * u[i] * u[i] * h
        })
        .sum();
    sum_sq.sqrt()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_build_pde_theory_env_populates() {
        let mut env = Environment::new();
        build_pde_theory_env(&mut env);
        assert!(env.get(&Name::str("lax_milgram")).is_some());
        assert!(env.get(&Name::str("SobolevSpace")).is_some());
        assert!(env.get(&Name::str("EllipticOperator")).is_some());
        assert!(env
            .get(&Name::str("navier_stokes_weak_existence"))
            .is_some());
        assert!(env.get(&Name::str("poincare_inequality")).is_some());
    }
    #[test]
    fn test_mesh1d_uniform() {
        let mesh = Mesh1D::uniform(0.0, 1.0, 4);
        assert_eq!(mesh.nodes.len(), 5);
        assert!((mesh.nodes[0] - 0.0).abs() < 1e-12);
        assert!((mesh.nodes[4] - 1.0).abs() < 1e-12);
        assert!((mesh.mesh_size() - 0.25).abs() < 1e-12);
        assert_eq!(mesh.num_interior(), 3);
    }
    #[test]
    fn test_stiffness_matrix_assemble() {
        let mesh = Mesh1D::uniform(0.0, 1.0, 3);
        let k = StiffnessMatrix1D::assemble(&mesh);
        assert_eq!(k.dim, 2);
        let h = mesh.mesh_size();
        assert!((k.data[0][0] - 2.0 / h).abs() < 1e-10);
        assert!((k.data[0][1] + 1.0 / h).abs() < 1e-10);
    }
    #[test]
    fn test_stiffness_matrix_solve() {
        let n = 20;
        let mesh = Mesh1D::uniform(0.0, 1.0, n + 1);
        let k = StiffnessMatrix1D::assemble(&mesh);
        let h = mesh.mesh_size();
        let rhs = vec![h; k.dim];
        let u = k.solve_tridiagonal(&rhs);
        let mid = u.len() / 2;
        let x_mid = mesh.nodes[1 + mid];
        let exact = x_mid * (1.0 - x_mid) / 2.0;
        assert!(
            (u[mid] - exact).abs() < 0.01,
            "FEM solution differs: got {}, expected {}",
            u[mid],
            exact
        );
    }
    #[test]
    fn test_heat_solver_decay() {
        let alpha = 1.0;
        let n = 50;
        let mesh = Mesh1D::uniform(0.0, 1.0, n + 1);
        let dt = 0.5 * mesh.mesh_size().powi(2) / alpha;
        let mut solver = HeatSolver1D::new(alpha, mesh, |x| (std::f64::consts::PI * x).sin());
        let t_end = 0.1;
        solver.advance_to(t_end, dt);
        let decay = (-std::f64::consts::PI * std::f64::consts::PI * alpha * t_end).exp();
        let norm = solver.l2_norm();
        let initial_norm = 1.0 / std::f64::consts::SQRT_2;
        let expected = initial_norm * decay;
        assert!(
            (norm - expected).abs() < 0.05,
            "Heat decay: got {}, expected ~{}",
            norm,
            expected
        );
    }
    #[test]
    fn test_burgers_shock() {
        let rp = BurgersRiemann::new(2.0, 0.0);
        assert!((rp.shock_speed() - 1.0).abs() < 1e-12);
        assert!((rp.eval(-0.5, 1.0) - 2.0).abs() < 1e-12);
        assert!((rp.eval(1.5, 1.0) - 0.0).abs() < 1e-12);
    }
    #[test]
    fn test_burgers_rarefaction() {
        let rp = BurgersRiemann::new(0.0, 1.0);
        assert!((rp.eval(0.5, 1.0) - 0.5).abs() < 1e-12);
        assert!((rp.eval(-0.1, 1.0) - 0.0).abs() < 1e-12);
        assert!((rp.eval(1.5, 1.0) - 1.0).abs() < 1e-12);
    }
    #[test]
    fn test_discrete_norms() {
        let n = 100;
        let h = 1.0 / n as f64;
        let u: Vec<f64> = (0..=n).map(|i| i as f64 * h).collect();
        let l2 = discrete_l2_norm(&u, h);
        assert!((l2 - 1.0 / 3.0_f64.sqrt()).abs() < 0.01, "L2 norm: {}", l2);
        let h1 = discrete_h1_seminorm(&u, h);
        assert!((h1 - 1.0).abs() < 0.01, "H1 seminorm: {}", h1);
    }
    #[test]
    fn test_axiom_types_are_props_or_types() {
        let ty = lax_milgram_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
        let ty2 = sobolev_space_ty();
        assert!(matches!(ty2, Expr::Pi(_, _, _, _)));
        let ty3 = poincare_inequality_ty();
        assert!(matches!(ty3, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_new_axioms_registered() {
        let mut env = Environment::new();
        build_pde_theory_env(&mut env);
        assert!(env.get(&Name::str("strichartz_estimate")).is_some());
        assert!(env.get(&Name::str("AdmissiblePair")).is_some());
        assert!(env.get(&Name::str("dispersive_estimate")).is_some());
        assert!(env.get(&Name::str("nls_gwp")).is_some());
        assert!(env.get(&Name::str("nls_blowup")).is_some());
        assert!(env.get(&Name::str("paraproduct_decomposition")).is_some());
        assert!(env.get(&Name::str("bilinear_estimate")).is_some());
        assert!(env.get(&Name::str("LittlewoodPaleyProjection")).is_some());
        assert!(env.get(&Name::str("CalderonZygmundOperator")).is_some());
        assert!(env.get(&Name::str("cz_boundedness")).is_some());
        assert!(env.get(&Name::str("WeakType11")).is_some());
        assert!(env.get(&Name::str("HilbertTransform")).is_some());
        assert!(env.get(&Name::str("SymbolClass")).is_some());
        assert!(env.get(&Name::str("PseudodiffOperator")).is_some());
        assert!(env.get(&Name::str("FourierIntegralOperator")).is_some());
        assert!(env.get(&Name::str("WavefrontSet")).is_some());
        assert!(env
            .get(&Name::str("propagation_of_singularities"))
            .is_some());
        assert!(env.get(&Name::str("ladyzhenskaya_prodi_serrin")).is_some());
        assert!(env.get(&Name::str("caffarelli_kohn_nirenberg")).is_some());
        assert!(env.get(&Name::str("RicciFlowSolution")).is_some());
        assert!(env.get(&Name::str("MeanCurvatureFlowSolution")).is_some());
        assert!(env.get(&Name::str("neck_pinch_singularity")).is_some());
        assert!(env.get(&Name::str("FreeBoundary")).is_some());
        assert!(env.get(&Name::str("obstacle_problem")).is_some());
        assert!(env.get(&Name::str("stefan_problem")).is_some());
        assert!(env.get(&Name::str("free_boundary_regularity")).is_some());
        assert!(env.get(&Name::str("HamiltonJacobiEquation")).is_some());
        assert!(env.get(&Name::str("viscosity_solution")).is_some());
        assert!(env.get(&Name::str("crandall_lions_uniqueness")).is_some());
        assert!(env.get(&Name::str("hopf_lax_formula")).is_some());
        assert!(env.get(&Name::str("HomogenizedCoefficient")).is_some());
        assert!(env.get(&Name::str("two_scale_convergence")).is_some());
        assert!(env.get(&Name::str("GammaConvergence")).is_some());
        assert!(env.get(&Name::str("homogenization_thm")).is_some());
        assert!(env.get(&Name::str("ReactionDiffusionSystem")).is_some());
        assert!(env.get(&Name::str("turing_instability")).is_some());
        assert!(env.get(&Name::str("traveling_wave")).is_some());
        assert!(env.get(&Name::str("fisher_kpp_equation")).is_some());
        assert!(env.get(&Name::str("VariationalInequality")).is_some());
        assert!(env.get(&Name::str("ConvexMinimizer")).is_some());
        assert!(env.get(&Name::str("penalty_method_convergence")).is_some());
    }
    #[test]
    fn test_new_axiom_expr_shapes() {
        assert!(matches!(strichartz_estimate_ty(), Expr::Pi(_, _, _, _)));
        assert!(matches!(viscosity_solution_ty(), Expr::Pi(_, _, _, _)));
        assert!(matches!(turing_instability_ty(), Expr::Pi(_, _, _, _)));
        assert!(matches!(calderon_zygmund_operator_ty(), Expr::Sort(_)));
        assert!(matches!(free_boundary_ty(), Expr::Sort(_)));
        assert!(matches!(ricci_flow_solution_ty(), Expr::Sort(_)));
    }
    #[test]
    fn test_pseudodiff_operator_identity() {
        let op = PseudodiffOperatorSim::new(0.0, 8);
        let u: Vec<f64> = (0..8).map(|i| (i as f64).sin()).collect();
        let v = op.apply(&u);
        for (a, b) in u.iter().zip(v.iter()) {
            assert!(
                (a - b).abs() < 1e-9,
                "Expected identity, got diff {}",
                a - b
            );
        }
    }
    #[test]
    fn test_pseudodiff_operator_norm_bound() {
        let op = PseudodiffOperatorSim::new(1.0, 16);
        let bound = op.l2_operator_norm_bound(16);
        assert!(bound > 1.0, "Norm bound should be > 1 for order 1");
    }
    #[test]
    fn test_dispersive_l2_conservation() {
        let checker = DispersiveEstimateChecker::new(16, 0.01);
        let u_re: Vec<f64> = (0..16)
            .map(|i| (i as f64 / 16.0 * 2.0 * std::f64::consts::PI).cos())
            .collect();
        let u_im: Vec<f64> = vec![0.0; 16];
        assert!(
            checker.check_l2_conservation(&u_re, &u_im, 1e-8),
            "L² should be conserved by Schrödinger propagator"
        );
    }
    #[test]
    fn test_parabolic_solver_decay() {
        let alpha = 1.0;
        let n = 50;
        let mesh = Mesh1D::uniform(0.0, 1.0, n + 1);
        let dt = 0.01;
        let mut solver = ParabolicSolver::new(alpha, mesh, |x| (std::f64::consts::PI * x).sin());
        let t_end = 0.1;
        solver.advance_to(t_end, dt);
        let decay = (-std::f64::consts::PI * std::f64::consts::PI * alpha * t_end).exp();
        let norm = solver.l2_norm();
        let initial_norm = 1.0 / std::f64::consts::SQRT_2;
        let expected = initial_norm * decay;
        assert!(
            (norm - expected).abs() < 0.05,
            "Implicit Euler heat decay: got {}, expected ~{}",
            norm,
            expected
        );
    }
    #[test]
    fn test_curvature_flow_circle_shrinks() {
        let mut sim = CurvatureFlowSim::circle(0.0, 0.0, 1.0, 32);
        let area_init = sim.area();
        sim.advance_to(0.1, 0.001);
        let area_final = sim.area();
        assert!(
            area_final < area_init,
            "Circle area should shrink under MCF: {} → {}",
            area_init,
            area_final
        );
    }
    #[test]
    fn test_homogenization_approx_constant_coeff() {
        let hom = HomogenizationApprox::new(1000);
        let c = 3.0;
        let a_hom = hom.compute(|_y| c);
        assert!(
            (a_hom - c).abs() < 0.01,
            "Constant coefficient: got {}, expected {}",
            a_hom,
            c
        );
    }
    #[test]
    fn test_homogenization_two_phase() {
        let hom = HomogenizationApprox::new(1000);
        let (a1, a2, theta) = (1.0, 4.0, 0.5);
        let a_hom = hom.compute(|y| if y < theta { a1 } else { a2 });
        let exact = HomogenizationApprox::two_phase_exact(a1, a2, theta);
        assert!(
            (a_hom - exact).abs() < 0.01,
            "Two-phase homogenization: got {}, expected {}",
            a_hom,
            exact
        );
    }
}
#[cfg(test)]
mod tests_pde_ext {
    use super::*;
    #[test]
    fn test_stochastic_heat() {
        let spde = StochasticHeatEquation::new(1.0, 0.5, 1);
        assert!(spde.is_well_posed_l2());
        let energy = spde.energy_at_time(1.0, 2.0);
        assert!(energy > 2.0);
        let reg = spde.mild_solution_regularity();
        assert!(reg.contains("L2"));
    }
    #[test]
    fn test_kpz_equation() {
        let kpz = KPZEquation::new(1.0, 0.5);
        let (chi, beta, z) = kpz.kpz_exponents();
        assert!((chi - 0.5).abs() < 1e-10);
        assert!((beta - 1.0 / 3.0).abs() < 1e-10);
        assert!((z - 1.5).abs() < 1e-10);
        assert!(kpz.is_kpz_universality_class());
        let hc = kpz.hopf_cole_transform();
        assert!(hc.contains("Hopf-Cole"));
    }
    #[test]
    fn test_minimal_surface() {
        let cat = MinimalSurface::catenoid();
        assert_eq!(cat.mean_curvature(), 0.0);
        assert_eq!(cat.ambient_dimension, 3);
        let plane = MinimalSurface::plane(4);
        let simons = plane.simons_gap_theorem();
        assert!(simons.contains("hyperplane"));
    }
    #[test]
    fn test_de_giorgi_nash_moser() {
        let dgnm = DeGiorgiNashMoser::new(0.5, 2.0, 3);
        assert!(dgnm.holder_exponent > 0.0);
        assert!(dgnm.ellipticity_ratio() == 4.0);
        let harnack = dgnm.harnack_inequality_constant();
        assert!(harnack > 1.0);
    }
    #[test]
    fn test_schauder_estimate() {
        let est = SchauderEstimate::second_order(0.3);
        assert_eq!(est.operator_order, 2);
        assert!((est.holder_solution - 2.3).abs() < 1e-10);
        let desc = est.interior_estimate();
        assert!(desc.contains("Schauder"));
    }
    #[test]
    fn test_strichartz() {
        let schr = StrichartzData::schrodinger(3);
        let desc = schr.strichartz_estimate_description();
        assert!(desc.contains("Strichartz"));
        assert!(schr.is_energy_critical());
        let wave = StrichartzData::wave(2);
        assert!(!wave.is_energy_critical());
    }
    #[test]
    fn test_nonlinear_schrodinger() {
        let nls = NonlinearSchrodinger::new(2.0, 3);
        assert!(nls.global_well_posedness_h1());
        let desc = nls.scattering_theory_description();
        assert!(desc.contains("NLS"));
    }
}
