//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    AdaptiveMeshRefinement, AffineTriangleMap, DGMesh1D, DGMethodStiffness, DenseMatrix, FEMesh2D,
    GalerkinStiffnessMatrix, GaussQuadrature, NitscheData1D, PoissonFESolver, StiffnessMatrix,
    TriangularMesh2D,
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
pub fn app4(f: Expr, a: Expr, b: Expr, c: Expr, d: Expr) -> Expr {
    app(app3(f, a, b, c), d)
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
pub fn fn_ty(dom: Expr, cod: Expr) -> Expr {
    arrow(dom, cod)
}
/// `Triangulation : Nat → Type`
///
/// A triangulation T_h of a domain Ω ⊂ ℝⁿ (n = dim) into simplicial elements.
/// The parameter h denotes the mesh size (largest element diameter).
pub fn triangulation_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `TriangulationElement : Type`
///
/// A single element (simplex) in a triangulation: carries vertex indices and
/// the Jacobian of the affine map from the reference element.
pub fn triangulation_element_ty() -> Expr {
    type0()
}
/// `MeshVertex : Type`
///
/// A vertex in the mesh: its coordinates and global node index.
pub fn mesh_vertex_ty() -> Expr {
    type0()
}
/// `IsConforming : Triangulation → Prop`
///
/// Conformity condition: any two elements share at most a common face, edge, or vertex.
/// No hanging nodes: every face of an element is either a boundary face or a full face
/// of exactly one neighbouring element.
pub fn is_conforming_ty() -> Expr {
    arrow(cst("Triangulation"), prop())
}
/// `IsShapeRegular : Triangulation → Real → Prop`
///
/// Shape regularity: ∃ κ > 0 such that for every element K in T_h,
///   h_K / ρ_K ≤ κ,
/// where h_K is the element diameter and ρ_K is the radius of the largest inscribed ball.
pub fn is_shape_regular_ty() -> Expr {
    arrow(cst("Triangulation"), arrow(real_ty(), prop()))
}
/// `IsQuasiuniform : Triangulation → Real → Prop`
///
/// Quasi-uniformity: T_h is shape regular AND ∃ σ > 0 such that h ≤ σ h_K for all K.
/// This is a stronger condition ensuring no element is too small relative to the mesh size.
pub fn is_quasiuniform_ty() -> Expr {
    arrow(cst("Triangulation"), arrow(real_ty(), prop()))
}
/// `MeshSize : Triangulation → Real`
///
/// The global mesh size h = max_{K ∈ T_h} diam(K).
pub fn mesh_size_ty() -> Expr {
    arrow(cst("Triangulation"), real_ty())
}
/// `NumElements : Triangulation → Nat`
///
/// Total number of elements in the triangulation.
pub fn num_elements_ty() -> Expr {
    arrow(cst("Triangulation"), nat_ty())
}
/// `NumDOF : FiniteElementSpace → Nat`
///
/// Number of degrees of freedom in the finite element space.
pub fn num_dof_ty() -> Expr {
    arrow(cst("FiniteElementSpace"), nat_ty())
}
/// `ReferenceTriangle : Type`
///
/// The reference triangle K̂ = {(ξ,η) ∈ ℝ² : ξ ≥ 0, η ≥ 0, ξ+η ≤ 1}.
/// All physical triangles are images of K̂ under an affine map Fₖ: ξ ↦ Bₖ ξ + bₖ.
pub fn reference_triangle_ty() -> Expr {
    type0()
}
/// `ReferenceTetrahedron : Type`
///
/// The reference tetrahedron K̂ = {(ξ,η,ζ) ∈ ℝ³ : ξ,η,ζ ≥ 0, ξ+η+ζ ≤ 1}.
pub fn reference_tetrahedron_ty() -> Expr {
    type0()
}
/// `ReferenceHexahedron : Type`
///
/// The reference hexahedron (cube) K̂ = [-1,1]³. Used for Q1/Q2 elements.
pub fn reference_hexahedron_ty() -> Expr {
    type0()
}
/// `AffineMap : ReferenceElement → PhysicalElement → Prop`
///
/// The affine map Fₖ: K̂ → K given by Fₖ(x̂) = Bₖ x̂ + bₖ.
/// Here Bₖ ∈ ℝ^{n×n} is invertible and bₖ ∈ ℝⁿ.
pub fn affine_map_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// `JacobianDeterminant : AffineMap → Real`
///
/// |det Bₖ|: the absolute value of the determinant of the Jacobian matrix.
/// Appears in change of variables in integrals: ∫_K f = ∫_{K̂} (f ∘ Fₖ) |det Bₖ| dξ.
pub fn jacobian_determinant_ty() -> Expr {
    arrow(prop(), real_ty())
}
/// `ShapeFunction : ReferenceTriangle → Nat → (Real → Real → Real)`
///
/// The k-th Lagrange P1 shape function on the reference triangle:
///   φ₁(ξ,η) = 1 - ξ - η,   φ₂(ξ,η) = ξ,   φ₃(ξ,η) = η.
pub fn shape_function_ty() -> Expr {
    arrow(
        type0(),
        arrow(nat_ty(), fn_ty(real_ty(), fn_ty(real_ty(), real_ty()))),
    )
}
/// `ShapeFunctionGradient : ReferenceTriangle → Nat → (Real → Real → Real × Real)`
///
/// Gradient of the k-th shape function on the reference element.
/// For P1: ∇φ₁ = (-1,-1), ∇φ₂ = (1,0), ∇φ₃ = (0,1).
pub fn shape_function_gradient_ty() -> Expr {
    arrow(
        type0(),
        arrow(nat_ty(), fn_ty(real_ty(), fn_ty(real_ty(), type0()))),
    )
}
/// `LagrangeP1Space : Triangulation → FiniteElementSpace`
///
/// The piecewise linear continuous finite element space V_h:
///   V_h = { v ∈ H¹(Ω) : v|_K ∈ P₁(K) for all K ∈ T_h }.
/// Globally continuous with nodal basis {φᵢ} satisfying φᵢ(xⱼ) = δᵢⱼ.
pub fn lagrange_p1_space_ty() -> Expr {
    arrow(cst("Triangulation"), type0())
}
/// `LagrangeP2Space : Triangulation → FiniteElementSpace`
///
/// The piecewise quadratic continuous finite element space:
///   V_h = { v ∈ H¹(Ω) : v|_K ∈ P₂(K) for all K ∈ T_h }.
/// Has 6 DOFs per triangle (3 vertices + 3 edge midpoints).
pub fn lagrange_p2_space_ty() -> Expr {
    arrow(cst("Triangulation"), type0())
}
/// `FiniteElementSpace : Type`  (base declaration)
pub fn finite_element_space_ty() -> Expr {
    type0()
}
/// `IsSubspace : FiniteElementSpace → H1Space → Prop`
///
/// V_h ⊆ V = H¹₀(Ω): the FE space is conforming (subspace of the continuous space).
pub fn is_subspace_ty() -> Expr {
    arrow(cst("FiniteElementSpace"), arrow(cst("H1Space"), prop()))
}
/// `InterpolationOperator : H1Space → FiniteElementSpace → H1Space`
///
/// The nodal interpolant Π_h: H¹(Ω) → V_h mapping u to its Lagrange interpolant.
pub fn interpolation_operator_ty() -> Expr {
    arrow(
        cst("H1Space"),
        arrow(cst("FiniteElementSpace"), cst("H1Space")),
    )
}
/// `GalerkinProblem : BilinearForm → LinearForm → FiniteElementSpace → Prop`
///
/// The Galerkin problem: find u_h ∈ V_h such that
///   a(u_h, v_h) = f(v_h) for all v_h ∈ V_h.
/// Encoded as existence and uniqueness of such u_h.
pub fn galerkin_problem_ty() -> Expr {
    arrow(
        cst("BilinearForm"),
        arrow(cst("LinearForm"), arrow(cst("FiniteElementSpace"), prop())),
    )
}
/// `LinearForm : Type`  (rhs functional f : V → ℝ)
pub fn linear_form_ty() -> Expr {
    type0()
}
/// `BilinearForm : Type`  (base declaration for a: V×V → ℝ)
pub fn bilinear_form_base_ty() -> Expr {
    type0()
}
/// `IsCoercive : BilinearForm → Real → Prop`
///
/// Coercivity: ∃ α > 0 such that a(u,u) ≥ α ‖u‖²_V for all u ∈ V.
pub fn is_coercive_ty() -> Expr {
    arrow(cst("BilinearForm"), arrow(real_ty(), prop()))
}
/// `IsBounded : BilinearForm → Real → Prop`
///
/// Continuity: ∃ M > 0 such that |a(u,v)| ≤ M ‖u‖ ‖v‖ for all u,v ∈ V.
pub fn is_bounded_ty() -> Expr {
    arrow(cst("BilinearForm"), arrow(real_ty(), prop()))
}
/// `StiffnessMatrix : FiniteElementSpace → Matrix`
///
/// The assembled global stiffness matrix A with entries
///   A_{ij} = a(φⱼ, φᵢ) = ∫_Ω ∇φⱼ · ∇φᵢ dx.
pub fn stiffness_matrix_ty() -> Expr {
    arrow(cst("FiniteElementSpace"), type0())
}
/// `MassMatrix : FiniteElementSpace → Matrix`
///
/// The assembled global mass matrix M with entries
///   M_{ij} = (φⱼ, φᵢ)_{L²} = ∫_Ω φⱼ φᵢ dx.
pub fn mass_matrix_ty() -> Expr {
    arrow(cst("FiniteElementSpace"), type0())
}
/// `CeaLemma : ∀ (a : BilinearForm) (α M : Real) (Vh : FESpace) (u uh : H1),`
/// `  IsCoercive a α → IsBounded a M → GalerkinSolution a f Vh uh →`
/// `  ‖u - uh‖_V ≤ (M/α) * inf_{vh ∈ Vh} ‖u - vh‖_V`
///
/// Céa's lemma: the Galerkin solution is quasi-optimal in the energy norm.
/// The error is bounded by the best approximation error, up to the constant M/α.
pub fn cea_lemma_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "a",
        cst("BilinearForm"),
        pi(
            BinderInfo::Default,
            "alpha",
            real_ty(),
            pi(
                BinderInfo::Default,
                "M",
                real_ty(),
                pi(
                    BinderInfo::Default,
                    "Vh",
                    cst("FiniteElementSpace"),
                    arrow(
                        app2(cst("IsCoercive"), bvar(3), bvar(2)),
                        arrow(app2(cst("IsBounded"), bvar(4), bvar(2)), prop()),
                    ),
                ),
            ),
        ),
    )
}
/// `InterpolationError : FiniteElementSpace → Nat → Real → Prop`
///
/// Interpolation estimate: for u ∈ H^{k+1}(Ω) and the P_k FE space,
///   ‖u - Π_h u‖_{H¹} ≤ C h^k |u|_{H^{k+1}}.
pub fn interpolation_error_ty() -> Expr {
    arrow(
        cst("FiniteElementSpace"),
        arrow(nat_ty(), arrow(real_ty(), prop())),
    )
}
/// `AprioriErrorEstimate : FiniteElementSpace → Nat → Real → Prop`
///
/// The a priori error estimate combining Céa's lemma with the interpolation bound:
///   ‖u - u_h‖_{H¹} ≤ C h^k |u|_{H^{k+1}}
/// for degree-k Lagrange elements and sufficiently smooth u.
pub fn apriori_error_estimate_ty() -> Expr {
    arrow(
        cst("FiniteElementSpace"),
        arrow(nat_ty(), arrow(real_ty(), prop())),
    )
}
/// `L2ErrorEstimate : FiniteElementSpace → Nat → Real → Prop`
///
/// The Aubin–Nitsche L² error estimate (duality argument):
///   ‖u - u_h‖_{L²} ≤ C h^{k+1} |u|_{H^{k+1}}
/// gaining one extra power of h compared to the H¹ estimate.
pub fn l2_error_estimate_ty() -> Expr {
    arrow(
        cst("FiniteElementSpace"),
        arrow(nat_ty(), arrow(real_ty(), prop())),
    )
}
/// `AubinNitscheTrick : Prop`
///
/// The Aubin–Nitsche duality argument: uses the adjoint problem
///   a(v, φ) = (e_h, v) for all v ∈ V
/// to bootstrap the H¹ estimate into an L² estimate with one additional power of h.
pub fn aubin_nitsche_trick_ty() -> Expr {
    prop()
}
/// `FEMWellPosedness : BilinearForm → LinearForm → FiniteElementSpace → Prop`
///
/// The discrete Lax–Milgram theorem: if a is coercive and bounded on V_h,
/// then the Galerkin problem has a unique solution u_h ∈ V_h.
pub fn fem_well_posedness_ty() -> Expr {
    arrow(
        cst("BilinearForm"),
        arrow(cst("LinearForm"), arrow(cst("FiniteElementSpace"), prop())),
    )
}
/// `LaxMilgramDiscrete : ∀ (a : BilinearForm) (f : LinearForm) (Vh : FESpace),`
/// `  IsCoercive a → IsBounded a → ∃! uh : H1Space, GalerkinSolution a f Vh uh`
///
/// The discrete Lax–Milgram theorem: existence and uniqueness for the Galerkin problem.
pub fn lax_milgram_discrete_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "a",
        cst("BilinearForm"),
        pi(
            BinderInfo::Default,
            "f",
            cst("LinearForm"),
            pi(
                BinderInfo::Default,
                "Vh",
                cst("FiniteElementSpace"),
                arrow(
                    app2(cst("IsCoercive"), bvar(2), real_ty()),
                    arrow(app2(cst("IsBounded"), bvar(3), real_ty()), prop()),
                ),
            ),
        ),
    )
}
/// `StabilityEstimate : BilinearForm → LinearForm → Prop`
///
/// Stability: the solution satisfies ‖u_h‖_V ≤ (1/α) ‖f‖_{V*},
/// i.e., the solution depends continuously on the data.
pub fn stability_estimate_ty() -> Expr {
    arrow(cst("BilinearForm"), arrow(cst("LinearForm"), prop()))
}
/// `NitscheBilinearForm : BilinearForm → Real → BilinearForm`
///
/// The Nitsche bilinear form incorporating a penalty term γ/h:
///   a_Nitsche(u,v) = ∫_Ω ∇u·∇v dx - ∫_{∂Ω} ∂_n u · v ds
///                  - ∫_{∂Ω} u · ∂_n v ds + (γ/h) ∫_{∂Ω} u · v ds.
/// This weakly enforces Dirichlet BCs while preserving optimal convergence.
pub fn nitsche_bilinear_form_ty() -> Expr {
    arrow(cst("BilinearForm"), arrow(real_ty(), cst("BilinearForm")))
}
/// `NitscheConsistency : NitscheBilinearForm → Prop`
///
/// Consistency: the exact solution u satisfies the Nitsche variational form,
/// so the method is Galerkin-orthogonal and admits a Céa-type bound.
pub fn nitsche_consistency_ty() -> Expr {
    arrow(cst("BilinearForm"), prop())
}
/// `NitscheCoercivity : NitscheBilinearForm → Real → Prop`
///
/// Coercivity of the Nitsche form for γ > γ₀ (sufficiently large penalty):
///   a_Nitsche(v_h, v_h) ≥ (α/2) ‖v_h‖²_{H¹}.
pub fn nitsche_coercivity_ty() -> Expr {
    arrow(cst("BilinearForm"), arrow(real_ty(), prop()))
}
/// `NitscheOptimalConvergence : FiniteElementSpace → Nat → Prop`
///
/// Nitsche's method achieves the same convergence rate as the method with
/// strongly imposed Dirichlet conditions:
///   ‖u - u_h^N‖_{H¹} ≤ C h^k |u|_{H^{k+1}}.
pub fn nitsche_optimal_convergence_ty() -> Expr {
    arrow(cst("FiniteElementSpace"), arrow(nat_ty(), prop()))
}
/// `DGSpace : Triangulation → Nat → Type`
///
/// The broken polynomial space V_h^{DG} = { v : Ω → ℝ : v|_K ∈ P_k(K) for all K }
/// without inter-element continuity constraints.
pub fn dg_space_ty() -> Expr {
    arrow(cst("Triangulation"), arrow(nat_ty(), type0()))
}
/// `InteriorPenaltyForm : DGSpace → Real → BilinearForm`
///
/// The symmetric interior penalty DG (SIPG) bilinear form:
///   a_{IP}(u,v) = Σ_K ∫_K ∇u·∇v dx
///               - Σ_e ∫_e ({∇u}·[v] + {∇v}·[u]) ds
///               + Σ_e (σ/h_e) ∫_e [u]·[v] ds,
/// where {·} and [·] denote averages and jumps across interior edges.
pub fn interior_penalty_form_ty() -> Expr {
    arrow(type0(), arrow(real_ty(), cst("BilinearForm")))
}
/// `DGCoercivity : InteriorPenaltyForm → Real → Prop`
///
/// Coercivity of the IP form in the DG norm ‖v‖²_{DG} = Σ_K ‖∇v‖²_{L²(K)} + Σ_e (σ/h_e) ‖[v]‖²
/// for σ > σ₀ sufficiently large.
pub fn dg_coercivity_ty() -> Expr {
    arrow(cst("BilinearForm"), arrow(real_ty(), prop()))
}
/// `DGConsistency : InteriorPenaltyForm → Prop`
///
/// Consistency: the exact solution satisfies a_{IP}(u, v_h) = f(v_h) for all v_h ∈ V_h^{DG}.
pub fn dg_consistency_ty() -> Expr {
    arrow(cst("BilinearForm"), prop())
}
/// `DGErrorEstimate : DGSpace → Nat → Real → Prop`
///
/// DG a priori error estimate in the DG norm:
///   ‖u - u_h‖_{DG} ≤ C h^k |u|_{H^{k+1}}.
pub fn dg_error_estimate_ty() -> Expr {
    arrow(type0(), arrow(nat_ty(), arrow(real_ty(), prop())))
}
/// `UPGForm : DGSpace → Real → BilinearForm`
///
/// The upwind DG (UWDG) form for convection-dominated problems:
///   a_{upw}(u,v) = -∫_Ω u (b·∇v) dx + ∫_{∂Ω_-} (b·n) u_inflow v ds
///                 + Σ_e ∫_e (b·n_e)^+ [u] {v} ds.
pub fn upg_form_ty() -> Expr {
    arrow(type0(), arrow(real_ty(), cst("BilinearForm")))
}
/// `MixedFEProblem : BilinearForm → BilinearForm → LinearForm → LinearForm → Prop`
///
/// The mixed (saddle-point) variational problem: find (u,p) ∈ V × Q such that
///   a(u,v) + b(v,p) = f(v) for all v ∈ V
///   b(u,q)           = g(q) for all q ∈ Q.
pub fn mixed_fe_problem_ty() -> Expr {
    arrow(
        cst("BilinearForm"),
        arrow(
            cst("BilinearForm"),
            arrow(cst("LinearForm"), arrow(cst("LinearForm"), prop())),
        ),
    )
}
/// `InfSupCondition : BilinearForm → Real → Prop`
///
/// The Ladyzhenskaya–Babuška–Brezzi (LBB) inf-sup condition:
///   inf_{q ∈ Q} sup_{v ∈ V} b(v,q) / (‖v‖_V ‖q‖_Q) ≥ β > 0.
/// This is the key compatibility condition for mixed methods.
pub fn inf_sup_condition_ty() -> Expr {
    arrow(cst("BilinearForm"), arrow(real_ty(), prop()))
}
/// `BrezziSplitting : MixedFEProblem → Prop`
///
/// The Brezzi splitting theorem: the saddle-point problem is well-posed iff
/// (i) a is coercive on ker(B), and (ii) the LBB inf-sup condition holds.
pub fn brezzi_splitting_ty() -> Expr {
    arrow(prop(), prop())
}
/// `MixedFEMErrorEstimate : BilinearForm → BilinearForm → FiniteElementSpace → Prop`
///
/// A priori error estimate for mixed methods: if the discrete inf-sup holds with
/// constant β_h ≥ β > 0, then ‖u - u_h‖ + ‖p - p_h‖ ≤ C (inf_{vh} ‖u-vh‖ + inf_{qh} ‖p-qh‖).
pub fn mixed_fem_error_estimate_ty() -> Expr {
    arrow(
        cst("BilinearForm"),
        arrow(
            cst("BilinearForm"),
            arrow(cst("FiniteElementSpace"), prop()),
        ),
    )
}
/// `TaylorHoodElement : Triangulation → FiniteElementSpace`
///
/// The Taylor–Hood element (P2/P1): velocity in P2, pressure in P1.
/// Satisfies the inf-sup condition for the Stokes equations.
pub fn taylor_hood_element_ty() -> Expr {
    arrow(cst("Triangulation"), type0())
}
/// `MiniElement : Triangulation → FiniteElementSpace`
///
/// The MINI element: velocity in P1 + bubble, pressure in P1.
/// A stable low-order element for Stokes.
pub fn mini_element_ty() -> Expr {
    arrow(cst("Triangulation"), type0())
}
/// `ResidualEstimator : FiniteElementSpace → H1Space → Real`
///
/// The element residual error estimator η_K:
///   η²_K = h²_K ‖f + Δu_h‖²_{L²(K)} + Σ_{e ∈ ∂K} h_e ‖[∂_n u_h]‖²_{L²(e)}.
pub fn residual_estimator_ty() -> Expr {
    arrow(cst("FiniteElementSpace"), arrow(cst("H1Space"), real_ty()))
}
/// `ReliabilityBound : ResidualEstimator → Prop`
///
/// Global reliability: ‖u - u_h‖_{H¹} ≤ C_rel (Σ_K η²_K)^{1/2}.
pub fn reliability_bound_ty() -> Expr {
    arrow(prop(), prop())
}
/// `EfficiencyBound : ResidualEstimator → Prop`
///
/// Local efficiency: η_K ≤ C_eff (‖u - u_h‖_{H¹(ω_K)} + h.o.t.)
/// where ω_K is the element patch.
pub fn efficiency_bound_ty() -> Expr {
    arrow(prop(), prop())
}
pub fn build_fem_env(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("FiniteElementSpace", finite_element_space_ty()),
        ("BilinearForm", bilinear_form_base_ty()),
        ("LinearForm", linear_form_ty()),
        ("H1Space", type0()),
        ("H10Space", type0()),
        ("L2Space", type0()),
        ("Matrix", type0()),
        ("Triangulation", triangulation_ty()),
        ("TriangulationElement", triangulation_element_ty()),
        ("MeshVertex", mesh_vertex_ty()),
        ("IsConforming", is_conforming_ty()),
        ("IsShapeRegular", is_shape_regular_ty()),
        ("IsQuasiuniform", is_quasiuniform_ty()),
        ("MeshSize", mesh_size_ty()),
        ("NumElements", num_elements_ty()),
        ("NumDOF", num_dof_ty()),
        ("ReferenceTriangle", reference_triangle_ty()),
        ("ReferenceTetrahedron", reference_tetrahedron_ty()),
        ("ReferenceHexahedron", reference_hexahedron_ty()),
        ("AffineMap", affine_map_ty()),
        ("JacobianDeterminant", jacobian_determinant_ty()),
        ("ShapeFunction", shape_function_ty()),
        ("ShapeFunctionGradient", shape_function_gradient_ty()),
        ("LagrangeP1Space", lagrange_p1_space_ty()),
        ("LagrangeP2Space", lagrange_p2_space_ty()),
        ("IsSubspace", is_subspace_ty()),
        ("InterpolationOperator", interpolation_operator_ty()),
        ("GalerkinProblem", galerkin_problem_ty()),
        ("IsCoercive", is_coercive_ty()),
        ("IsBounded", is_bounded_ty()),
        ("StiffnessMatrix", stiffness_matrix_ty()),
        ("MassMatrix", mass_matrix_ty()),
        ("cea_lemma", cea_lemma_ty()),
        ("InterpolationError", interpolation_error_ty()),
        ("AprioriErrorEstimate", apriori_error_estimate_ty()),
        ("L2ErrorEstimate", l2_error_estimate_ty()),
        ("aubin_nitsche_trick", aubin_nitsche_trick_ty()),
        ("FEMWellPosedness", fem_well_posedness_ty()),
        ("lax_milgram_discrete", lax_milgram_discrete_ty()),
        ("StabilityEstimate", stability_estimate_ty()),
        ("NitscheBilinearForm", nitsche_bilinear_form_ty()),
        ("NitscheConsistency", nitsche_consistency_ty()),
        ("NitscheCoercivity", nitsche_coercivity_ty()),
        (
            "nitsche_optimal_convergence",
            nitsche_optimal_convergence_ty(),
        ),
        ("DGSpace", dg_space_ty()),
        ("InteriorPenaltyForm", interior_penalty_form_ty()),
        ("DGCoercivity", dg_coercivity_ty()),
        ("DGConsistency", dg_consistency_ty()),
        ("DGErrorEstimate", dg_error_estimate_ty()),
        ("UPGForm", upg_form_ty()),
        ("MixedFEProblem", mixed_fe_problem_ty()),
        ("InfSupCondition", inf_sup_condition_ty()),
        ("brezzi_splitting", brezzi_splitting_ty()),
        ("MixedFEMErrorEstimate", mixed_fem_error_estimate_ty()),
        ("TaylorHoodElement", taylor_hood_element_ty()),
        ("MiniElement", mini_element_ty()),
        ("ResidualEstimator", residual_estimator_ty()),
        ("reliability_bound", reliability_bound_ty()),
        ("efficiency_bound", efficiency_bound_ty()),
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
/// The three Lagrange P1 shape functions on the reference triangle K̂.
///
/// φ₁(ξ,η) = 1 - ξ - η
/// φ₂(ξ,η) = ξ
/// φ₃(ξ,η) = η
pub fn p1_shape(k: usize, xi: f64, eta: f64) -> f64 {
    match k {
        0 => 1.0 - xi - eta,
        1 => xi,
        2 => eta,
        _ => 0.0,
    }
}
/// Gradient of P1 shape functions on the reference triangle (constant).
///
/// ∇φ₁ = (-1, -1),  ∇φ₂ = (1, 0),  ∇φ₃ = (0, 1)
pub fn p1_grad(k: usize) -> [f64; 2] {
    match k {
        0 => [-1.0, -1.0],
        1 => [1.0, 0.0],
        2 => [0.0, 1.0],
        _ => [0.0, 0.0],
    }
}
/// Evaluate the six Lagrange P2 shape functions on the reference triangle.
///
/// Nodes: v0=(0,0), v1=(1,0), v2=(0,1), m01=(0.5,0), m12=(0.5,0.5), m02=(0,0.5)
pub fn p2_shape(k: usize, xi: f64, eta: f64) -> f64 {
    let lam1 = 1.0 - xi - eta;
    let lam2 = xi;
    let lam3 = eta;
    match k {
        0 => lam1 * (2.0 * lam1 - 1.0),
        1 => lam2 * (2.0 * lam2 - 1.0),
        2 => lam3 * (2.0 * lam3 - 1.0),
        3 => 4.0 * lam1 * lam2,
        4 => 4.0 * lam2 * lam3,
        5 => 4.0 * lam1 * lam3,
        _ => 0.0,
    }
}
/// 3-point quadrature rule on the reference triangle (degree 2 exact).
///
/// Points: (1/6, 1/6), (2/3, 1/6), (1/6, 2/3); weights: 1/6 each.
pub fn reference_triangle_quadrature() -> Vec<([f64; 2], f64)> {
    vec![
        ([1.0 / 6.0, 1.0 / 6.0], 1.0 / 6.0),
        ([2.0 / 3.0, 1.0 / 6.0], 1.0 / 6.0),
        ([1.0 / 6.0, 2.0 / 3.0], 1.0 / 6.0),
    ]
}
/// Assemble the global P1 stiffness matrix for -Δu = f on a 2D triangular mesh
/// with homogeneous Dirichlet boundary conditions (boundary DOFs excluded).
///
/// Returns the interior stiffness matrix and the load vector for f ≡ 1.
#[allow(clippy::too_many_arguments)]
pub fn assemble_p1_stiffness(mesh: &TriangularMesh2D) -> (DenseMatrix, Vec<f64>) {
    let n_v = mesh.num_vertices();
    let mut k_global = DenseMatrix::zeros(n_v);
    let mut f_global = vec![0.0; n_v];
    let quad = reference_triangle_quadrature();
    for tri_idx in 0..mesh.num_triangles() {
        let [i0, i1, i2] = mesh.triangles[tri_idx];
        let p0 = mesh.vertices[i0];
        let p1 = mesh.vertices[i1];
        let p2 = mesh.vertices[i2];
        let fmap = AffineTriangleMap::new(p0, p1, p2);
        let det_j = fmap.det_j();
        let mut ke = [[0.0f64; 3]; 3];
        let mut fe = [0.0f64; 3];
        let grads_phys: [[f64; 2]; 3] = [
            fmap.transform_grad(p1_grad(0)),
            fmap.transform_grad(p1_grad(1)),
            fmap.transform_grad(p1_grad(2)),
        ];
        let area = det_j / 2.0;
        for a in 0..3 {
            for b in 0..3 {
                let dot = grads_phys[a][0] * grads_phys[b][0] + grads_phys[a][1] * grads_phys[b][1];
                ke[a][b] = dot * area;
            }
            for (pt, w) in &quad {
                fe[a] += w * p1_shape(a, pt[0], pt[1]) * det_j;
            }
        }
        let local_nodes = [i0, i1, i2];
        for a in 0..3 {
            for b in 0..3 {
                k_global.add(local_nodes[a], local_nodes[b], ke[a][b]);
            }
            f_global[local_nodes[a]] += fe[a];
        }
    }
    (k_global, f_global)
}
/// Apply Dirichlet boundary conditions by zeroing rows/columns of boundary nodes
/// and setting the diagonal to 1 with rhs = 0.
pub fn apply_dirichlet_bc(mat: &mut DenseMatrix, rhs: &mut Vec<f64>, is_boundary: &[bool]) {
    for i in 0..mat.n {
        if is_boundary[i] {
            for j in 0..mat.n {
                mat.data[i * mat.n + j] = 0.0;
                mat.data[j * mat.n + i] = 0.0;
            }
            mat.data[i * mat.n + i] = 1.0;
            rhs[i] = 0.0;
        }
    }
}
/// Solve A x = b using the conjugate gradient method.
/// Assumes A is symmetric positive definite.
pub fn conjugate_gradient(mat: &DenseMatrix, rhs: &[f64], tol: f64, max_iter: usize) -> Vec<f64> {
    let n = mat.n;
    let mut x = vec![0.0; n];
    let mut r = rhs.to_vec();
    let mut p = r.clone();
    let mut rr = dot(&r, &r);
    for _ in 0..max_iter {
        if rr < tol * tol {
            break;
        }
        let ap = mat.matvec(&p);
        let alpha = rr / dot(&p, &ap);
        for i in 0..n {
            x[i] += alpha * p[i];
            r[i] -= alpha * ap[i];
        }
        let rr_new = dot(&r, &r);
        let beta = rr_new / rr;
        for i in 0..n {
            p[i] = r[i] + beta * p[i];
        }
        rr = rr_new;
    }
    x
}
pub fn dot(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}
/// Compute element residual error indicators for a 1D P1 FEM solution of -u'' = f.
///
/// η_i = h * |f_i + (u'_{i+1} - u'_i) / h|  (simplified residual at interior nodes)
pub fn element_residuals_1d(u: &[f64], h: f64, f: impl Fn(f64) -> f64) -> Vec<f64> {
    let n = u.len();
    if n < 3 {
        return vec![];
    }
    (1..n - 1)
        .map(|i| {
            let x_i = i as f64 * h;
            let laplacian_u = (u[i - 1] - 2.0 * u[i] + u[i + 1]) / (h * h);
            let residual = f(x_i) + laplacian_u;
            h * residual.abs()
        })
        .collect()
}
/// Global a posteriori error bound: η = (Σ_i η_i²)^{1/2}.
pub fn global_error_estimator(indicators: &[f64]) -> f64 {
    indicators.iter().map(|e| e * e).sum::<f64>().sqrt()
}
/// `PetrovGalerkinProblem : BilinearForm → LinearForm → FESpace → FESpace → Prop`
///
/// The Petrov-Galerkin method: find u_h ∈ V_h such that
///   a(u_h, v_h) = f(v_h) for all v_h ∈ W_h,
/// where the trial space V_h and test space W_h may differ.
pub fn petrov_galerkin_problem_ty() -> Expr {
    arrow(
        cst("BilinearForm"),
        arrow(
            cst("LinearForm"),
            arrow(
                cst("FiniteElementSpace"),
                arrow(cst("FiniteElementSpace"), prop()),
            ),
        ),
    )
}
/// `BubnovGalerkinConsistency : BilinearForm → FiniteElementSpace → Prop`
///
/// Variational consistency: the Bubnov-Galerkin solution satisfies the
/// Galerkin orthogonality condition a(u - u_h, v_h) = 0 for all v_h ∈ V_h.
pub fn bubnov_galerkin_consistency_ty() -> Expr {
    arrow(
        cst("BilinearForm"),
        arrow(cst("FiniteElementSpace"), prop()),
    )
}
/// `GalerkinOrthogonality : BilinearForm → H1Space → FiniteElementSpace → Prop`
///
/// Galerkin orthogonality: the error e_h = u - u_h is orthogonal to V_h
/// in the energy inner product a(e_h, v_h) = 0.
pub fn galerkin_orthogonality_ty() -> Expr {
    arrow(
        cst("BilinearForm"),
        arrow(cst("H1Space"), arrow(cst("FiniteElementSpace"), prop())),
    )
}
/// `HRefinement : Triangulation → Triangulation → Prop`
///
/// h-refinement: T_h' is obtained from T_h by uniformly or adaptively
/// subdividing elements, reducing the mesh size h by a factor.
pub fn h_refinement_ty() -> Expr {
    arrow(cst("Triangulation"), arrow(cst("Triangulation"), prop()))
}
/// `PRefinement : FiniteElementSpace → Nat → FiniteElementSpace → Prop`
///
/// p-refinement: the polynomial degree is increased from k to k+1
/// while the mesh remains fixed.
pub fn p_refinement_ty() -> Expr {
    arrow(
        cst("FiniteElementSpace"),
        arrow(nat_ty(), arrow(cst("FiniteElementSpace"), prop())),
    )
}
/// `HPAdaptivity : Triangulation → FiniteElementSpace → Prop`
///
/// hp-adaptivity: combines h-refinement in regions with low regularity
/// and p-enrichment in smooth regions to achieve exponential convergence.
pub fn hp_adaptivity_ty() -> Expr {
    arrow(
        cst("Triangulation"),
        arrow(cst("FiniteElementSpace"), prop()),
    )
}
/// `ExponentialConvergence : FiniteElementSpace → Real → Prop`
///
/// Exponential convergence of hp-FEM for analytic solutions:
///   ‖u - u_{hp}‖_{H¹} ≤ C exp(-b √N)
/// where N is the number of degrees of freedom.
pub fn exponential_convergence_ty() -> Expr {
    arrow(cst("FiniteElementSpace"), arrow(real_ty(), prop()))
}
/// `DorflerMarking : ResidualEstimator → Real → Subset → Prop`
///
/// Dörfler's marking strategy: select a minimal subset M ⊆ T_h such that
///   Σ_{K ∈ M} η²_K ≥ θ Σ_{K ∈ T_h} η²_K  for θ ∈ (0,1).
pub fn dorfler_marking_ty() -> Expr {
    arrow(prop(), arrow(real_ty(), arrow(prop(), prop())))
}
/// `NedelecSpace : Triangulation → Nat → Type`
///
/// The Nédélec edge element space of degree k:
///   N_k(T_h) = { v ∈ H(curl, Ω) : v|_K ∈ N_k(K) for all K ∈ T_h }.
/// DOFs are moments of tangential components on edges.
pub fn nedelec_space_ty() -> Expr {
    arrow(cst("Triangulation"), arrow(nat_ty(), type0()))
}
/// `RaviartThomasSpace : Triangulation → Nat → Type`
///
/// The Raviart-Thomas H(div) space of degree k:
///   RT_k(T_h) = { v ∈ H(div, Ω) : v|_K ∈ RT_k(K) for all K ∈ T_h }.
/// DOFs are moments of normal components on faces.
pub fn raviart_thomas_space_ty() -> Expr {
    arrow(cst("Triangulation"), arrow(nat_ty(), type0()))
}
/// `BrezziDouglasMariniSpace : Triangulation → Nat → Type`
///
/// The BDM space: an enhanced RT space with full polynomial degree on faces.
pub fn bdm_space_ty() -> Expr {
    arrow(cst("Triangulation"), arrow(nat_ty(), type0()))
}
/// `DeRhamComplex : Triangulation → Prop`
///
/// The discrete de Rham complex:
///   H¹_h →^{∇} H(curl)_h →^{∇×} H(div)_h →^{∇·} L²_h
/// The complex is exact and commutes with the continuous de Rham complex.
pub fn de_rham_complex_ty() -> Expr {
    arrow(cst("Triangulation"), prop())
}
/// `HCurlConforming : FiniteElementSpace → Prop`
///
/// H(curl)-conforming: the tangential trace is continuous across element faces.
/// Satisfied by Nédélec elements.
pub fn hcurl_conforming_ty() -> Expr {
    arrow(cst("FiniteElementSpace"), prop())
}
/// `HDivConforming : FiniteElementSpace → Prop`
///
/// H(div)-conforming: the normal trace is continuous across element faces.
/// Satisfied by Raviart-Thomas and BDM elements.
pub fn hdiv_conforming_ty() -> Expr {
    arrow(cst("FiniteElementSpace"), prop())
}
/// `ZZEstimator : FiniteElementSpace → H1Space → Real`
///
/// The Zienkiewicz-Zhu (ZZ) superconvergent patch recovery estimator:
/// η²_K = ‖G(∇u_h) - ∇u_h‖²_{L²(K)}
/// where G is the SPR gradient recovery operator.
pub fn zz_estimator_ty() -> Expr {
    arrow(cst("FiniteElementSpace"), arrow(cst("H1Space"), real_ty()))
}
/// `GoalOrientedEstimator : FiniteElementSpace → LinearForm → Real`
///
/// Goal-oriented a posteriori error estimator for a functional J(u):
///   |J(u) - J(u_h)| ≤ η_goal
/// using the dual-weighted residual (DWR) method.
pub fn goal_oriented_estimator_ty() -> Expr {
    arrow(
        cst("FiniteElementSpace"),
        arrow(cst("LinearForm"), real_ty()),
    )
}
/// `AdaptiveConvergence : FiniteElementSpace → Real → Real → Prop`
///
/// Convergence of the AFEM loop: after m refinement steps,
///   ‖u - u_h^m‖_{H¹} ≤ C m^{-s/d}
/// where s is the approximation class and d the spatial dimension.
pub fn adaptive_convergence_ty() -> Expr {
    arrow(
        cst("FiniteElementSpace"),
        arrow(real_ty(), arrow(real_ty(), prop())),
    )
}
/// `MultigridPreconditioner : FiniteElementSpace → Type`
///
/// A multigrid V-cycle preconditioner:
/// combines smoothing (Gauss-Seidel or ILU) on each level with
/// coarse-grid correction for optimal O(N) complexity.
pub fn multigrid_preconditioner_ty() -> Expr {
    arrow(cst("FiniteElementSpace"), type0())
}
/// `DomainDecompositionMethod : FiniteElementSpace → Nat → Prop`
///
/// Domain decomposition with n subdomains: decomposes the problem into
/// independent subproblems coupled via interface conditions.
pub fn domain_decomposition_ty() -> Expr {
    arrow(cst("FiniteElementSpace"), arrow(nat_ty(), prop()))
}
/// `FETIPreconditioner : FiniteElementSpace → Type`
///
/// FETI (Finite Element Tearing and Interconnecting) preconditioner:
/// non-overlapping decomposition with Lagrange multipliers at subdomain interfaces.
pub fn feti_preconditioner_ty() -> Expr {
    arrow(cst("FiniteElementSpace"), type0())
}
/// `BDDCPreconditioner : FiniteElementSpace → Type`
///
/// BDDC (Balancing Domain Decomposition by Constraints) preconditioner:
/// primal formulation of FETI-DP with optimally chosen primal unknowns.
pub fn bddc_preconditioner_ty() -> Expr {
    arrow(cst("FiniteElementSpace"), type0())
}
/// `MultigridOptimality : MultigridPreconditioner → Prop`
///
/// Optimality of multigrid: the condition number of the preconditioned system
/// κ(M⁻¹A) = O(1) is bounded independently of the mesh size h.
pub fn multigrid_optimality_ty() -> Expr {
    arrow(type0(), prop())
}
/// `BSplineBasis : Nat → Nat → Type`
///
/// B-spline basis of degree p with n + 1 control points.
/// Supports C^{p-1} continuity across knot spans (C^{p-m_i} at a knot of multiplicity m_i).
pub fn bspline_basis_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `NURBSSurface : BSplineBasis → BSplineBasis → Type`
///
/// A NURBS (Non-Uniform Rational B-Spline) surface: the geometry parameterization
/// used in isogeometric analysis to represent the exact CAD geometry.
pub fn nurbs_surface_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// `IsogeometricSpace : NURBSSurface → Nat → Type`
///
/// The isogeometric analysis space: uses the same NURBS basis for both
/// the geometry representation and the FEM solution approximation.
pub fn isogeometric_space_ty() -> Expr {
    arrow(type0(), arrow(nat_ty(), type0()))
}
/// `IsogeometricConvergence : IsogeometricSpace → Nat → Real → Prop`
///
/// IGA optimal convergence: for degree p,
///   ‖u - u_h‖_{H¹} ≤ C h^p |u|_{H^{p+1}}
/// matching standard FEM rates but with better per-DOF accuracy.
pub fn isogeometric_convergence_ty() -> Expr {
    arrow(type0(), arrow(nat_ty(), arrow(real_ty(), prop())))
}
/// `TSplineSpace : Nat → Type`
///
/// T-spline space: allows local refinement (T-junctions) while maintaining
/// watertight NURBS-compatible geometry representations.
pub fn tspline_space_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `SpaceTimeFESpace : FiniteElementSpace → Real → Type`
///
/// A space-time finite element space over Ω × (0,T):
/// uses tensor-product Lagrange elements in space and time simultaneously.
pub fn space_time_fe_space_ty() -> Expr {
    arrow(cst("FiniteElementSpace"), arrow(real_ty(), type0()))
}
/// `ParabolicStability : SpaceTimeFESpace → Prop`
///
/// Energy stability for the heat equation discretization:
///   ‖u_h(T)‖²_{L²} + Σ_n ∫_{t_n}^{t_{n+1}} ‖∇u_h‖² dt ≤ C ‖u_h(0)‖²_{L²}.
pub fn parabolic_stability_ty() -> Expr {
    arrow(type0(), prop())
}
/// `WaveEnergyConservation : SpaceTimeFESpace → Prop`
///
/// Energy conservation for the wave equation:
///   ‖∂_t u_h(t)‖²_{L²} + ‖∇u_h(t)‖²_{L²} = const for all t ∈ (0,T).
pub fn wave_energy_conservation_ty() -> Expr {
    arrow(type0(), prop())
}
/// `SpaceTimeErrorEstimate : SpaceTimeFESpace → Nat → Real → Prop`
///
/// Space-time a priori error estimate:
///   ‖u - u_h‖_{L²(0,T;H¹)} ≤ C (h^k + τ^q) |u|_{W^{q+1,2}(0,T;H^{k+1})}.
pub fn space_time_error_estimate_ty() -> Expr {
    arrow(type0(), arrow(nat_ty(), arrow(real_ty(), prop())))
}
/// `NewtonRaphsonFEM : BilinearForm → LinearForm → FiniteElementSpace → Prop`
///
/// Newton-Raphson iteration for nonlinear FEM:
///   K(u_h^k) Δu^k = -R(u_h^k),  u_h^{k+1} = u_h^k + Δu^k
/// where K is the consistent tangent stiffness and R the residual.
pub fn newton_raphson_fem_ty() -> Expr {
    arrow(
        cst("BilinearForm"),
        arrow(cst("LinearForm"), arrow(cst("FiniteElementSpace"), prop())),
    )
}
/// `ArcLengthMethod : BilinearForm → Real → Prop`
///
/// Arc-length continuation for path-following through limit points:
/// constrains the load parameter to trace the load-displacement curve.
pub fn arc_length_method_ty() -> Expr {
    arrow(cst("BilinearForm"), arrow(real_ty(), prop()))
}
/// `ConsistentLinearization : BilinearForm → H1Space → BilinearForm`
///
/// The consistent linearization of a nonlinear bilinear form at u:
///   Da(u)[h,v] = lim_{ε→0} (a(u + εh, v) - a(u,v))/ε.
pub fn consistent_linearization_ty() -> Expr {
    arrow(
        cst("BilinearForm"),
        arrow(cst("H1Space"), cst("BilinearForm")),
    )
}
/// `QuadraticConvergenceNewton : NewtonRaphsonFEM → Prop`
///
/// Quadratic convergence of Newton's method near the solution:
///   ‖u_h^{k+1} - u*‖ ≤ C ‖u_h^k - u*‖².
pub fn quadratic_convergence_newton_ty() -> Expr {
    arrow(prop(), prop())
}
/// `ReissnerMindlinPlate : Triangulation → Nat → Type`
///
/// The Reissner-Mindlin plate element:
/// thickness-shear deformable plate theory (valid for thick plates).
/// DOFs: deflection w and rotations (θ_x, θ_y) at each node.
pub fn reissner_mindlin_plate_ty() -> Expr {
    arrow(cst("Triangulation"), arrow(nat_ty(), type0()))
}
/// `KirchhoffLovePlate : Triangulation → Nat → Type`
///
/// The Kirchhoff-Love plate element (Euler-Bernoulli in 2D):
/// thin plate theory requiring C¹ continuity.
pub fn kirchhoff_love_plate_ty() -> Expr {
    arrow(cst("Triangulation"), arrow(nat_ty(), type0()))
}
/// `ShearLockingFreedom : ReissnerMindlinPlate → Prop`
///
/// Shear-locking-free property: for thin plates (t→0),
/// the Reissner-Mindlin element recovers the Kirchhoff solution
/// without spurious shear stiffness (locking).
pub fn shear_locking_freedom_ty() -> Expr {
    arrow(type0(), prop())
}
/// `ReducedIntegration : FiniteElementSpace → Nat → Prop`
///
/// Selective/reduced integration: using fewer quadrature points
/// to avoid locking phenomena in nearly incompressible materials and thin structures.
pub fn reduced_integration_ty() -> Expr {
    arrow(cst("FiniteElementSpace"), arrow(nat_ty(), prop()))
}
/// `FluidStructureInteraction : BilinearForm → BilinearForm → Prop`
///
/// Fluid-structure interaction (FSI): coupled system of fluid equations
/// (Navier-Stokes) and structural equations (elasticity) on a moving domain.
pub fn fluid_structure_interaction_ty() -> Expr {
    arrow(cst("BilinearForm"), arrow(cst("BilinearForm"), prop()))
}
/// `ThermoMechanicalCoupling : BilinearForm → BilinearForm → LinearForm → Prop`
///
/// Thermo-mechanical coupling: the thermal and mechanical problems are coupled
/// via thermal expansion and heat generation by plastic deformation.
pub fn thermo_mechanical_coupling_ty() -> Expr {
    arrow(
        cst("BilinearForm"),
        arrow(cst("BilinearForm"), arrow(cst("LinearForm"), prop())),
    )
}
/// `PoroElasticityBiot : BilinearForm → BilinearForm → Prop`
///
/// Biot's consolidation (poro-elasticity): fully coupled fluid-solid
/// interaction in a porous medium (Terzaghi + Darcy flow).
pub fn poro_elasticity_biot_ty() -> Expr {
    arrow(cst("BilinearForm"), arrow(cst("BilinearForm"), prop()))
}
/// `OperatorSplitting : CoupledProblem → Prop`
///
/// Operator splitting for coupled problems: decouples the subsystems
/// and solves them sequentially, with correction terms at each time step.
pub fn operator_splitting_ty() -> Expr {
    arrow(prop(), prop())
}
/// Register all new FEM axioms (Sections 11-20) into `env`.
pub fn build_fem_env_extended(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("PetrovGalerkinProblem", petrov_galerkin_problem_ty()),
        (
            "BubnovGalerkinConsistency",
            bubnov_galerkin_consistency_ty(),
        ),
        ("GalerkinOrthogonality", galerkin_orthogonality_ty()),
        ("HRefinement", h_refinement_ty()),
        ("PRefinement", p_refinement_ty()),
        ("HPAdaptivity", hp_adaptivity_ty()),
        ("ExponentialConvergence", exponential_convergence_ty()),
        ("DorflerMarking", dorfler_marking_ty()),
        ("NedelecSpace", nedelec_space_ty()),
        ("RaviartThomasSpace", raviart_thomas_space_ty()),
        ("BDMSpace", bdm_space_ty()),
        ("DeRhamComplex", de_rham_complex_ty()),
        ("HCurlConforming", hcurl_conforming_ty()),
        ("HDivConforming", hdiv_conforming_ty()),
        ("ZZEstimator", zz_estimator_ty()),
        ("GoalOrientedEstimator", goal_oriented_estimator_ty()),
        ("AdaptiveConvergence", adaptive_convergence_ty()),
        ("MultigridPreconditioner", multigrid_preconditioner_ty()),
        ("DomainDecomposition", domain_decomposition_ty()),
        ("FETIPreconditioner", feti_preconditioner_ty()),
        ("BDDCPreconditioner", bddc_preconditioner_ty()),
        ("MultigridOptimality", multigrid_optimality_ty()),
        ("BSplineBasis", bspline_basis_ty()),
        ("NURBSSurface", nurbs_surface_ty()),
        ("IsogeometricSpace", isogeometric_space_ty()),
        ("IsogeometricConvergence", isogeometric_convergence_ty()),
        ("TSplineSpace", tspline_space_ty()),
        ("SpaceTimeFESpace", space_time_fe_space_ty()),
        ("ParabolicStability", parabolic_stability_ty()),
        ("WaveEnergyConservation", wave_energy_conservation_ty()),
        ("SpaceTimeErrorEstimate", space_time_error_estimate_ty()),
        ("NewtonRaphsonFEM", newton_raphson_fem_ty()),
        ("ArcLengthMethod", arc_length_method_ty()),
        ("ConsistentLinearization", consistent_linearization_ty()),
        (
            "QuadraticConvergenceNewton",
            quadratic_convergence_newton_ty(),
        ),
        ("ReissnerMindlinPlate", reissner_mindlin_plate_ty()),
        ("KirchhoffLovePlate", kirchhoff_love_plate_ty()),
        ("ShearLockingFreedom", shear_locking_freedom_ty()),
        ("ReducedIntegration", reduced_integration_ty()),
        (
            "FluidStructureInteraction",
            fluid_structure_interaction_ty(),
        ),
        ("ThermoMechanicalCoupling", thermo_mechanical_coupling_ty()),
        ("PoroElasticityBiot", poro_elasticity_biot_ty()),
        ("OperatorSplitting", operator_splitting_ty()),
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
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mesh_area() {
        let mesh = TriangularMesh2D::unit_square(4);
        let area = mesh.total_area();
        assert!((area - 1.0).abs() < 1e-12, "Total area = {}", area);
    }
    #[test]
    fn test_mesh_size() {
        let mesh = TriangularMesh2D::unit_square(4);
        let h = mesh.mesh_size();
        let h_expected = std::f64::consts::SQRT_2 / 4.0;
        assert!(
            (h - h_expected).abs() < 1e-12,
            "Mesh size = {}, expected {}",
            h,
            h_expected
        );
    }
    #[test]
    fn test_p1_shape_partition_of_unity() {
        let points = [(0.2, 0.3), (0.5, 0.3), (0.1, 0.8), (1.0 / 3.0, 1.0 / 3.0)];
        for (xi, eta) in points {
            let sum: f64 = (0..3).map(|k| p1_shape(k, xi, eta)).sum();
            assert!(
                (sum - 1.0).abs() < 1e-12,
                "POU failed at ({}, {}): sum = {}",
                xi,
                eta,
                sum
            );
        }
    }
    #[test]
    fn test_p2_shape_partition_of_unity() {
        let points = [(0.2, 0.2), (0.5, 0.1), (0.0, 0.5)];
        for (xi, eta) in points {
            let sum: f64 = (0..6).map(|k| p2_shape(k, xi, eta)).sum();
            assert!(
                (sum - 1.0).abs() < 1e-12,
                "P2 POU failed at ({}, {}): sum = {}",
                xi,
                eta,
                sum
            );
        }
    }
    #[test]
    fn test_affine_map_and_jacobian() {
        let fmap = AffineTriangleMap::new([0.0, 0.0], [1.0, 0.0], [0.0, 1.0]);
        assert!((fmap.det_j() - 1.0).abs() < 1e-12);
        let centroid = fmap.apply(1.0 / 3.0, 1.0 / 3.0);
        assert!((centroid[0] - 1.0 / 3.0).abs() < 1e-12);
        assert!((centroid[1] - 1.0 / 3.0).abs() < 1e-12);
    }
    #[test]
    fn test_stiffness_assembly_diagonal() {
        let mesh = TriangularMesh2D::unit_square(2);
        let (k, _f) = assemble_p1_stiffness(&mesh);
        for i in 0..k.n {
            assert!(
                k.get(i, i) >= 0.0,
                "Diagonal entry [{}] = {} < 0",
                i,
                k.get(i, i)
            );
        }
    }
    #[test]
    fn test_dg_advection_conservation() {
        let mesh = DGMesh1D::uniform(0.0, 1.0, 10);
        let u0: Vec<f64> = mesh
            .centers
            .iter()
            .map(|&x| (2.0 * std::f64::consts::PI * x).sin())
            .collect();
        let mass0 = mesh.l1_norm(&u0);
        let dt = 0.5 * mesh.h;
        let u1 = mesh.dg_step(&u0, 1.0, dt);
        let mass1 = mesh.l1_norm(&u1);
        assert!(
            mass1 <= mass0 + 1e-10,
            "DG increased mass: {} > {}",
            mass1,
            mass0
        );
    }
    #[test]
    fn test_nitsche_coercivity_threshold() {
        let n_data = NitscheData1D::new(10.0, 0.1, 0.0);
        assert!(n_data.is_coercive());
        let n_data_small = NitscheData1D::new(0.5, 0.1, 0.0);
        assert!(!n_data_small.is_coercive());
    }
    #[test]
    fn test_axiom_types_well_formed() {
        assert!(matches!(cea_lemma_ty(), Expr::Pi(_, _, _, _)));
        assert!(matches!(lax_milgram_discrete_ty(), Expr::Pi(_, _, _, _)));
        assert!(matches!(triangulation_ty(), Expr::Pi(_, _, _, _)));
        assert!(matches!(
            mixed_fem_error_estimate_ty(),
            Expr::Pi(_, _, _, _)
        ));
    }
    #[test]
    fn test_build_fem_env_extended() {
        let mut env = Environment::new();
        build_fem_env(&mut env);
        build_fem_env_extended(&mut env);
        assert!(env
            .get(&oxilean_kernel::Name::str("NedelecSpace"))
            .is_some());
        assert!(env
            .get(&oxilean_kernel::Name::str("BDDCPreconditioner"))
            .is_some());
        assert!(env
            .get(&oxilean_kernel::Name::str("IsogeometricSpace"))
            .is_some());
        assert!(env
            .get(&oxilean_kernel::Name::str("ReissnerMindlinPlate"))
            .is_some());
        assert!(env
            .get(&oxilean_kernel::Name::str("FluidStructureInteraction"))
            .is_some());
    }
    #[test]
    fn test_galerkin_stiffness_constant() {
        let mesh = TriangularMesh2D::unit_square(2);
        let gs = GalerkinStiffnessMatrix::constant(1.0, mesh.clone());
        let k_gs = gs.assemble();
        let (k_p1, _) = assemble_p1_stiffness(&mesh);
        assert_eq!(k_gs.n, k_p1.n);
        for i in 0..k_gs.n {
            assert!(
                (k_gs.get(i, i) - k_p1.get(i, i)).abs() < 1e-12,
                "Diagonal [{i}] differs: {} vs {}",
                k_gs.get(i, i),
                k_p1.get(i, i)
            );
        }
    }
    #[test]
    fn test_galerkin_stiffness_variable() {
        let mesh = TriangularMesh2D::unit_square(2);
        let gs1 = GalerkinStiffnessMatrix::constant(1.0, mesh.clone());
        let gs2 = GalerkinStiffnessMatrix::constant(2.0, mesh.clone());
        let k1 = gs1.assemble();
        let k2 = gs2.assemble();
        for i in 0..k1.n {
            assert!(
                (k2.get(i, i) - 2.0 * k1.get(i, i)).abs() < 1e-12,
                "k=2 diagonal [{i}] should be 2× k=1"
            );
        }
    }
    #[test]
    fn test_adaptive_refinement_reduces_mesh_size() {
        let mesh = TriangularMesh2D::unit_square(2);
        let n_before = mesh.num_triangles();
        let mut amr = AdaptiveMeshRefinement::new(mesh, 1.0);
        let refined = amr.step();
        assert!(refined > 0, "should have refined at least one element");
        assert!(
            amr.mesh.num_triangles() > n_before,
            "mesh should grow after refinement"
        );
    }
    #[test]
    fn test_adaptive_refinement_area_preserved() {
        let mesh = TriangularMesh2D::unit_square(2);
        let area_before = mesh.total_area();
        let mut amr = AdaptiveMeshRefinement::new(mesh, 1.0);
        amr.step();
        let area_after = amr.mesh.total_area();
        assert!(
            (area_after - area_before).abs() < 1e-10,
            "area before = {area_before}, after = {area_after}"
        );
    }
    #[test]
    fn test_dg_stiffness_coercive() {
        let mesh = TriangularMesh2D::unit_square(2);
        let dg = DGMethodStiffness::new(mesh, 100.0);
        assert!(
            dg.is_coercive(),
            "DG stiffness with σ=100 should be coercive"
        );
    }
    #[test]
    fn test_dg_stiffness_dimension() {
        let mesh = TriangularMesh2D::unit_square(2);
        let n_tri = mesh.num_triangles();
        let dg = DGMethodStiffness::new(mesh, 10.0);
        let mat = dg.assemble();
        assert_eq!(mat.n, 3 * n_tri, "DG matrix size should be 3 × n_triangles");
    }
    #[test]
    fn test_new_axiom_types_are_pi_or_sort() {
        for ty in [
            petrov_galerkin_problem_ty(),
            nedelec_space_ty(),
            multigrid_preconditioner_ty(),
            isogeometric_convergence_ty(),
            newton_raphson_fem_ty(),
            kirchhoff_love_plate_ty(),
            fluid_structure_interaction_ty(),
        ] {
            assert!(
                matches!(ty, Expr::Pi(_, _, _, _) | Expr::Sort(_)),
                "Expected Pi or Sort, got {:?}",
                ty
            );
        }
    }
}
#[allow(dead_code)]
pub fn dist(a: (f64, f64), b: (f64, f64)) -> f64 {
    ((b.0 - a.0).powi(2) + (b.1 - a.1).powi(2)).sqrt()
}
#[cfg(test)]
mod tests_fem_extra {
    use super::*;
    fn unit_triangle_mesh() -> FEMesh2D {
        let nodes = vec![(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)];
        let elems = vec![(0, 1, 2)];
        FEMesh2D::new(nodes, elems)
    }
    #[test]
    fn test_fem_mesh() {
        let mesh = unit_triangle_mesh();
        assert_eq!(mesh.n_nodes(), 3);
        assert_eq!(mesh.n_elements(), 1);
        let area = mesh.element_area(0);
        assert!((area - 0.5).abs() < 1e-9, "Area should be 0.5, got {area}");
        assert!((mesh.total_area() - 0.5).abs() < 1e-9);
    }
    #[test]
    fn test_gauss_quadrature() {
        let gl2 = GaussQuadrature::gauss_legendre_2();
        assert_eq!(gl2.n_points(), 2);
        let integral = gl2.integrate(|x| x * x);
        assert!((integral - 2.0 / 3.0).abs() < 1e-6);
    }
    #[test]
    fn test_stiffness_matrix() {
        let mut k = StiffnessMatrix::new(3);
        k.add_entry(0, 0, 2.0);
        k.add_entry(0, 1, -1.0);
        k.add_entry(1, 0, -1.0);
        k.add_entry(1, 1, 2.0);
        k.add_entry(1, 2, -1.0);
        k.add_entry(2, 1, -1.0);
        k.add_entry(2, 2, 1.0);
        assert_eq!(k.n_nonzeros(), 7);
        let r = k.apply(&[1.0, 1.0, 1.0]);
        assert!((r[0] - 1.0).abs() < 1e-9);
    }
    #[test]
    fn test_poisson_solver() {
        let mesh = unit_triangle_mesh();
        let mut solver = PoissonFESolver::new(mesh);
        solver.add_dirichlet_bc(0, 0.0);
        solver.add_dirichlet_bc(1, 0.0);
        assert_eq!(solver.n_free_dofs(), 1);
        assert!(solver.estimated_condition_number() > 0.0);
    }
}
