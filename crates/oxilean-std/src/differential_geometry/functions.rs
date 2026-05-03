//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    CalibrationChecker, ChristoffelSymbols, CurvatureTensor, Curve3D, DifferentialForm,
    DifferentialFormWedge, FirstFundamentalForm, Geodesic, GeodesicIntegrator, HodgeStar,
    HolonomyComputer, LieGroupSO3, LorentzianMetric2D, RandersFinsler, RiemannMetric,
    RiemannianMetric2D, RiemannianMetric3D, Sphere, Torus, WeylTensorComputer,
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
pub fn int_ty() -> Expr {
    cst("Int")
}
/// Smooth manifold type: a topological space with smooth atlas
pub fn smooth_manifold_ty() -> Expr {
    type0()
}
/// Tangent bundle type TM over a manifold M
pub fn tangent_bundle_ty() -> Expr {
    arrow(type0(), type0())
}
/// Riemannian metric type: a smooth positive-definite inner product on TM
pub fn riemannian_metric_ty() -> Expr {
    arrow(type0(), type0())
}
/// Geodesic curve type: length-minimizing (locally) curves on a manifold
pub fn geodesic_ty() -> Expr {
    arrow(type0(), arrow(real_ty(), type0()))
}
/// Riemann curvature tensor R type
pub fn curvature_tensor_ty() -> Expr {
    arrow(type0(), type0())
}
/// Levi-Civita connection type
pub fn connection_ty() -> Expr {
    arrow(type0(), type0())
}
/// Atlas: collection of charts covering a manifold
pub fn atlas_ty() -> Expr {
    arrow(type0(), arrow(nat_ty(), type0()))
}
/// Chart: homeomorphism from open subset of M to R^n
pub fn chart_ty() -> Expr {
    arrow(type0(), arrow(nat_ty(), type0()))
}
/// Transition map between two charts
pub fn transition_map_ty() -> Expr {
    arrow(nat_ty(), arrow(arrow(real_ty(), real_ty()), prop()))
}
/// Cotangent bundle T*M over a manifold M
pub fn cotangent_bundle_ty() -> Expr {
    arrow(type0(), type0())
}
/// Vector bundle over a base manifold
pub fn vector_bundle_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// Differential k-form on a manifold M
pub fn differential_form_ty() -> Expr {
    arrow(nat_ty(), arrow(type0(), type0()))
}
/// Exterior derivative d: Ω^k(M) → Ω^(k+1)(M)
pub fn exterior_derivative_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(
            type0(),
            arrow(
                app2(cst("DifferentialForm"), cst("k"), cst("M")),
                app2(
                    cst("DifferentialForm"),
                    app(cst("Nat.succ"), cst("k")),
                    cst("M"),
                ),
            ),
        ),
    )
}
/// Integration of a top-form over an oriented manifold
pub fn integration_form_ty() -> Expr {
    arrow(type0(), arrow(type0(), real_ty()))
}
/// Stokes' theorem: ∫_M dω = ∫_∂M ω
pub fn stokes_theorem_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// de Rham cohomology group H^k_dR(M)
pub fn de_rham_cohomology_ty() -> Expr {
    arrow(nat_ty(), arrow(type0(), type0()))
}
/// de Rham's theorem: H^k_dR(M) ≅ H^k_sing(M; R)
pub fn de_rham_theorem_ty() -> Expr {
    arrow(nat_ty(), arrow(type0(), prop()))
}
/// Lie group type (smooth group manifold)
pub fn lie_group_ty() -> Expr {
    type0()
}
/// Lie algebra type: tangent space at identity with Lie bracket
pub fn lie_algebra_ty() -> Expr {
    arrow(type0(), type0())
}
/// Exponential map exp: g → G from Lie algebra to Lie group
pub fn lie_exp_map_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// Principal fiber bundle P(M, G)
pub fn principal_bundle_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// Connection 1-form on a principal bundle (g-valued 1-form)
pub fn connection_1form_ty() -> Expr {
    arrow(type0(), type0())
}
/// Curvature 2-form Ω = dA + A ∧ A
pub fn curvature_2form_ty() -> Expr {
    arrow(type0(), type0())
}
/// Bianchi identity: dΩ + \[A, Ω\] = 0
pub fn bianchi_identity_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// Sectional curvature K(σ) of a 2-plane section σ
pub fn sectional_curvature_ty() -> Expr {
    arrow(type0(), arrow(type0(), real_ty()))
}
/// Ricci tensor Ric(X,Y) = trace of R(·,X)Y
pub fn ricci_tensor_ty() -> Expr {
    arrow(type0(), type0())
}
/// Scalar curvature s = trace(Ric)
pub fn scalar_curvature_ty() -> Expr {
    arrow(type0(), arrow(type0(), real_ty()))
}
/// Einstein field equations: G_μν + Λg_μν = 8πG T_μν
pub fn einstein_equations_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// Chern-Weil homomorphism: characteristic classes from curvature
pub fn chern_weil_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// Characteristic class (Chern, Pontryagin, Euler classes)
pub fn characteristic_class_ty() -> Expr {
    arrow(nat_ty(), arrow(type0(), type0()))
}
/// Hodge star operator *: Ω^k(M) → Ω^(n-k)(M)
pub fn hodge_star_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(type0(), type0())))
}
/// Hodge decomposition: ω = dα + δβ + γ (γ harmonic)
pub fn hodge_decomposition_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// Harmonic form: Δω = 0 where Δ = dδ + δd is the Hodge Laplacian
pub fn harmonic_form_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// Symplectic form: closed non-degenerate 2-form ω on an even-dimensional manifold
pub fn symplectic_form_ty() -> Expr {
    arrow(type0(), type0())
}
/// Darboux theorem: every symplectic manifold is locally ω = ∑ dp_i ∧ dq_i
pub fn darboux_theorem_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// Hamiltonian vector field X_H associated to H: M → R
pub fn hamiltonian_vector_field_ty() -> Expr {
    arrow(type0(), arrow(arrow(type0(), real_ty()), type0()))
}
/// Moment map μ: M → g* for a Lie group action
pub fn moment_map_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// Symplectic reduction (Marsden-Weinstein): M//G = μ⁻¹(0)/G
pub fn symplectic_reduction_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// Contact structure: maximally non-integrable hyperplane distribution ξ ⊂ TM
pub fn contact_structure_ty() -> Expr {
    arrow(type0(), type0())
}
/// Contact manifold (M, ξ) with contact form α
pub fn contact_manifold_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// Reeb vector field associated to a contact form α
pub fn reeb_vector_field_ty() -> Expr {
    arrow(type0(), type0())
}
/// Geodesic equation: ∇_γ' γ' = 0
pub fn geodesic_equation_ty() -> Expr {
    arrow(type0(), arrow(arrow(real_ty(), type0()), prop()))
}
/// Jacobi field: solution to the Jacobi equation along a geodesic
pub fn jacobi_field_ty() -> Expr {
    arrow(type0(), arrow(arrow(real_ty(), type0()), type0()))
}
/// Sub-Riemannian metric: metric on a bracket-generating distribution Δ ⊂ TM
pub fn axiom_sub_riemannian_metric_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// Carnot-Carathéodory distance in a sub-Riemannian manifold
pub fn axiom_carnot_caratheodory_dist_ty() -> Expr {
    arrow(type0(), arrow(type0(), arrow(type0(), real_ty())))
}
/// Horizontal curve: a curve tangent to the distribution at every point
pub fn axiom_horizontal_curve_ty() -> Expr {
    arrow(
        type0(),
        arrow(type0(), arrow(arrow(real_ty(), type0()), prop())),
    )
}
/// Chow-Rashevskii theorem: bracket-generating ⟹ any two points connected by horizontal curve
pub fn axiom_chow_rashevskii_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// Finsler metric: smooth norm F: TM → R on each tangent space (generalization of Riemannian)
pub fn axiom_finsler_metric_ty() -> Expr {
    arrow(type0(), type0())
}
/// Finsler geodesic: length-minimizing path under a Finsler metric
pub fn axiom_finsler_geodesic_ty() -> Expr {
    arrow(type0(), arrow(real_ty(), type0()))
}
/// Busemann function: generalized distance function at infinity
pub fn axiom_busemann_function_ty() -> Expr {
    arrow(type0(), arrow(type0(), real_ty()))
}
/// Lorentzian metric: indefinite metric of signature (−,+,+,+) on a 4-manifold
pub fn axiom_lorentzian_metric_ty() -> Expr {
    arrow(type0(), type0())
}
/// Causal future J⁺(p): set of points q reachable from p by future-directed causal curves
pub fn axiom_causal_future_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// Causal past J⁻(p)
pub fn axiom_causal_past_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// Globally hyperbolic spacetime: a strongly causal spacetime with compact causal diamonds
pub fn axiom_globally_hyperbolic_ty() -> Expr {
    arrow(type0(), prop())
}
/// Penrose-Hawking singularity theorem: incomplete geodesics under energy conditions
pub fn axiom_singularity_theorem_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// Spin structure on an oriented Riemannian manifold
pub fn axiom_spin_structure_ty() -> Expr {
    arrow(type0(), type0())
}
/// Spinor bundle S → M associated to a spin structure
pub fn axiom_spinor_bundle_ty() -> Expr {
    arrow(type0(), type0())
}
/// Dirac operator: D : Γ(S) → Γ(S), first-order elliptic self-adjoint operator
pub fn axiom_dirac_operator_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// Lichnerowicz formula: D² = ∇*∇ + R/4 (relating Dirac to curvature)
pub fn axiom_lichnerowicz_formula_ty() -> Expr {
    arrow(type0(), prop())
}
/// Atiyah-Singer index theorem for Dirac operator: ind(D) = Â(M)
pub fn axiom_atiyah_singer_dirac_ty() -> Expr {
    arrow(type0(), prop())
}
/// Conformal metric: g̃ = e^{2f} g related to g by a positive function
pub fn axiom_conformal_metric_ty() -> Expr {
    arrow(
        type0(),
        arrow(arrow(type0(), real_ty()), arrow(type0(), type0())),
    )
}
/// Conformal class \[g\]: equivalence class of metrics differing by positive function
pub fn axiom_conformal_class_ty() -> Expr {
    arrow(type0(), type0())
}
/// Weyl tensor: trace-free part of Riemann tensor, conformally invariant
pub fn axiom_weyl_tensor_ty() -> Expr {
    arrow(type0(), type0())
}
/// Yamabe problem: find constant scalar curvature metric in a conformal class
pub fn axiom_yamabe_problem_ty() -> Expr {
    arrow(type0(), prop())
}
/// Projective differential structure: unparameterized geodesics defining projective class
pub fn axiom_projective_structure_ty() -> Expr {
    arrow(type0(), type0())
}
/// CR structure: Cauchy-Riemann structure, complex distribution on odd-dimensional manifold
pub fn axiom_cr_structure_ty() -> Expr {
    arrow(type0(), type0())
}
/// CR manifold (M, T^{1,0}M) with partial complex structure
pub fn axiom_cr_manifold_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// Cartan geometry modelled on (G, H): (P, ω) with Cartan connection ω
pub fn axiom_cartan_geometry_ty() -> Expr {
    arrow(type0(), arrow(type0(), arrow(type0(), type0())))
}
/// G-structure on M: a reduction of the frame bundle to a subgroup G ⊂ GL(n)
pub fn axiom_g_structure_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// Holonomy group Hol(M, g, p): group of parallel transports around loops at p
pub fn axiom_holonomy_group_ty() -> Expr {
    arrow(type0(), arrow(type0(), arrow(type0(), type0())))
}
/// Berger's classification of irreducible Riemannian holonomy groups
pub fn axiom_berger_classification_ty() -> Expr {
    arrow(type0(), prop())
}
/// Special holonomy: manifold with holonomy contained in a proper subgroup of SO(n)
pub fn axiom_special_holonomy_ty() -> Expr {
    arrow(type0(), prop())
}
/// Calibration φ ∈ Ω^k(M): a closed form with φ|_ξ ≤ vol_ξ for all k-planes ξ
pub fn axiom_calibration_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// Calibrated submanifold: a submanifold N ↪ M where φ|_N = vol_N (area-minimizing)
pub fn axiom_calibrated_submanifold_ty() -> Expr {
    arrow(type0(), arrow(type0(), arrow(type0(), prop())))
}
/// Kähler-Einstein metric: Ric(g) = λ g for some constant λ on a Kähler manifold
pub fn axiom_kahler_einstein_metric_ty() -> Expr {
    arrow(type0(), arrow(real_ty(), prop()))
}
/// Yau's theorem (Calabi conjecture): every Kähler class with c₁=0 has unique Ricci-flat metric
pub fn axiom_yau_theorem_ty() -> Expr {
    arrow(type0(), prop())
}
/// Hermitian metric h on a complex vector bundle E → M
pub fn axiom_hermitian_metric_ty() -> Expr {
    arrow(type0(), type0())
}
/// Chern connection: unique connection on (E, h) compatible with both h and holomorphic structure
pub fn axiom_chern_connection_ty() -> Expr {
    arrow(type0(), type0())
}
/// Bochner technique: vanishing theorem via Weitzenböck identity + positivity of curvature
pub fn axiom_bochner_technique_ty() -> Expr {
    arrow(type0(), arrow(nat_ty(), prop()))
}
/// Kodaira vanishing theorem: H^q(M, K_M ⊗ L) = 0 for q > 0 and L positive line bundle
pub fn axiom_kodaira_vanishing_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// Bishop-Gromov volume comparison theorem
pub fn axiom_bishop_gromov_ty() -> Expr {
    arrow(type0(), arrow(real_ty(), prop()))
}
/// Alexandrov space: metric space with curvature bounded below in comparison sense
pub fn axiom_alexandrov_space_ty() -> Expr {
    arrow(real_ty(), arrow(type0(), prop()))
}
/// Toponogov comparison theorem: triangle comparison in non-negative curvature
pub fn axiom_toponogov_ty() -> Expr {
    arrow(type0(), prop())
}
/// Gromov-Hausdorff convergence of metric spaces
pub fn axiom_gromov_hausdorff_ty() -> Expr {
    arrow(arrow(nat_ty(), type0()), arrow(type0(), prop()))
}
/// Gromov's compactness theorem: sequences of manifolds with bounded curvature and diameter subconverge
pub fn axiom_gromov_compactness_ty() -> Expr {
    arrow(type0(), prop())
}
/// Gauss-Bonnet theorem: ∫ K dA = 2π χ(M) for closed orientable surface M
pub fn gauss_bonnet_ty() -> Expr {
    let m_var = app(cst("SmoothManifold"), cst("Unit"));
    arrow(m_var, prop())
}
/// Hopf-Rinow theorem: complete ↔ geodesically complete
pub fn hopf_rinow_ty() -> Expr {
    let m_var = app(cst("SmoothManifold"), cst("Unit"));
    arrow(m_var, prop())
}
/// Exponential map: exp_p : T_pM → M
pub fn exponential_map_ty() -> Expr {
    arrow(type0(), arrow(real_ty(), real_ty()))
}
/// Parallel transport along curves preserves the Riemannian metric
pub fn parallel_transport_ty() -> Expr {
    arrow(type0(), arrow(arrow(real_ty(), type0()), prop()))
}
/// Register all differential geometry axioms into the environment.
pub fn build_differential_geometry_env(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("SmoothManifold", smooth_manifold_ty()),
        ("TangentBundle", tangent_bundle_ty()),
        ("RiemannianMetric", riemannian_metric_ty()),
        ("Geodesic", geodesic_ty()),
        ("CurvatureTensor", curvature_tensor_ty()),
        ("LeviCivitaConnection", connection_ty()),
        ("Atlas", atlas_ty()),
        ("Chart", chart_ty()),
        ("TransitionMap", transition_map_ty()),
        ("CotangentBundle", cotangent_bundle_ty()),
        ("VectorBundle", vector_bundle_ty()),
        ("DifferentialForm", differential_form_ty()),
        ("ExteriorDerivative", exterior_derivative_ty()),
        ("IntegrationForm", integration_form_ty()),
        ("DeRhamCohomology", de_rham_cohomology_ty()),
        ("LieGroup", lie_group_ty()),
        ("LieAlgebra", lie_algebra_ty()),
        ("LieExpMap", lie_exp_map_ty()),
        ("PrincipalBundle", principal_bundle_ty()),
        ("Connection1Form", connection_1form_ty()),
        ("Curvature2Form", curvature_2form_ty()),
        ("SectionalCurvature", sectional_curvature_ty()),
        ("RicciTensor", ricci_tensor_ty()),
        ("ScalarCurvature", scalar_curvature_ty()),
        ("ChernWeil", chern_weil_ty()),
        ("CharacteristicClass", characteristic_class_ty()),
        ("HodgeStar", hodge_star_ty()),
        ("HarmonicForm", harmonic_form_ty()),
        ("SymplecticForm", symplectic_form_ty()),
        ("HamiltonianVectorField", hamiltonian_vector_field_ty()),
        ("MomentMap", moment_map_ty()),
        ("SymplecticReduction", symplectic_reduction_ty()),
        ("ContactStructure", contact_structure_ty()),
        ("ContactManifold", contact_manifold_ty()),
        ("ReebVectorField", reeb_vector_field_ty()),
        ("JacobiField", jacobi_field_ty()),
        ("GaussianCurvatureIntegral", arrow(type0(), real_ty())),
        ("EulerCharacteristic", arrow(type0(), int_ty())),
        ("TangentSpace", arrow(type0(), arrow(type0(), type0()))),
        (
            "ParallelTransport",
            arrow(type0(), arrow(arrow(real_ty(), type0()), type0())),
        ),
        ("ExponentialMap", arrow(type0(), arrow(real_ty(), type0()))),
        ("SecondFundamentalFormConst", arrow(type0(), type0())),
        ("ShapeOperator", arrow(type0(), type0())),
        ("SphereManifold", arrow(real_ty(), type0())),
        ("TorusManifold", arrow(real_ty(), arrow(real_ty(), type0()))),
        ("PlaneManifold", type0()),
        ("gauss_bonnet", gauss_bonnet_ty()),
        ("hopf_rinow", hopf_rinow_ty()),
        ("exponential_map", exponential_map_ty()),
        ("parallel_transport", parallel_transport_ty()),
        ("stokes_theorem", stokes_theorem_ty()),
        ("de_rham_theorem", de_rham_theorem_ty()),
        ("bianchi_identity", bianchi_identity_ty()),
        ("einstein_equations", einstein_equations_ty()),
        ("hodge_decomposition", hodge_decomposition_ty()),
        ("darboux_theorem", darboux_theorem_ty()),
        ("geodesic_equation", geodesic_equation_ty()),
        ("SubRiemannianMetric", axiom_sub_riemannian_metric_ty()),
        (
            "CarnotCaratheodoryDist",
            axiom_carnot_caratheodory_dist_ty(),
        ),
        ("HorizontalCurve", axiom_horizontal_curve_ty()),
        ("FinslerMetric", axiom_finsler_metric_ty()),
        ("FinslerGeodesic", axiom_finsler_geodesic_ty()),
        ("BusemannFunction", axiom_busemann_function_ty()),
        ("LorentzianMetric", axiom_lorentzian_metric_ty()),
        ("CausalFuture", axiom_causal_future_ty()),
        ("CausalPast", axiom_causal_past_ty()),
        ("GloballyHyperbolic", axiom_globally_hyperbolic_ty()),
        ("SpinStructure", axiom_spin_structure_ty()),
        ("SpinorBundle", axiom_spinor_bundle_ty()),
        ("DiracOperator", axiom_dirac_operator_ty()),
        ("ConformalMetric", axiom_conformal_metric_ty()),
        ("ConformalClass", axiom_conformal_class_ty()),
        ("WeylTensor", axiom_weyl_tensor_ty()),
        ("ProjectiveStructure", axiom_projective_structure_ty()),
        ("CRStructure", axiom_cr_structure_ty()),
        ("CRManifold", axiom_cr_manifold_ty()),
        ("CartanGeometry", axiom_cartan_geometry_ty()),
        ("GStructure", axiom_g_structure_ty()),
        ("HolonomyGroup", axiom_holonomy_group_ty()),
        ("Calibration", axiom_calibration_ty()),
        ("CalibratedSubmanifold", axiom_calibrated_submanifold_ty()),
        ("KahlerEinsteinMetric", axiom_kahler_einstein_metric_ty()),
        ("HermitianMetric", axiom_hermitian_metric_ty()),
        ("ChernConnection", axiom_chern_connection_ty()),
        ("AlexandrovSpace", axiom_alexandrov_space_ty()),
        ("chow_rashevskii", axiom_chow_rashevskii_ty()),
        ("singularity_theorem", axiom_singularity_theorem_ty()),
        ("lichnerowicz_formula", axiom_lichnerowicz_formula_ty()),
        ("atiyah_singer_dirac", axiom_atiyah_singer_dirac_ty()),
        ("yamabe_problem", axiom_yamabe_problem_ty()),
        ("berger_classification", axiom_berger_classification_ty()),
        ("yau_theorem", axiom_yau_theorem_ty()),
        ("bochner_technique", axiom_bochner_technique_ty()),
        ("kodaira_vanishing", axiom_kodaira_vanishing_ty()),
        ("bishop_gromov", axiom_bishop_gromov_ty()),
        ("toponogov", axiom_toponogov_ty()),
        ("gromov_hausdorff", axiom_gromov_hausdorff_ty()),
        ("gromov_compactness", axiom_gromov_compactness_ty()),
        ("IsFlat", arrow(type0(), prop())),
        ("IsPositivelyCurved", arrow(type0(), prop())),
        ("IsNegativelyCurved", arrow(type0(), prop())),
        ("IsMinimalSurface", arrow(type0(), prop())),
        ("IsGeodesicallyComplete", arrow(type0(), prop())),
        ("IsMetricallyComplete", arrow(type0(), prop())),
        ("FirstFundamentalFormConst", arrow(type0(), type0())),
        (
            "GaussianCurvatureAt",
            arrow(type0(), arrow(real_ty(), arrow(real_ty(), real_ty()))),
        ),
        (
            "MeanCurvatureAt",
            arrow(type0(), arrow(real_ty(), arrow(real_ty(), real_ty()))),
        ),
        ("ChernGauss", prop()),
        ("AtiyahSinger", prop()),
        ("Schoenflies", prop()),
        ("NashEmbedding", arrow(type0(), prop())),
        ("Pi", real_ty()),
        ("TwoPi", real_ty()),
        ("IsSymplectic", arrow(type0(), prop())),
        ("IsContactManifold", arrow(type0(), prop())),
        ("IsKahler", arrow(type0(), prop())),
        ("IsEinstein", arrow(type0(), prop())),
        ("IsHarmonic", arrow(type0(), prop())),
        ("WedgeProduct", arrow(type0(), arrow(type0(), type0()))),
        ("DualLieAlgebra", arrow(type0(), type0())),
        ("Distribution", arrow(type0(), arrow(nat_ty(), type0()))),
        ("VectorField", arrow(type0(), type0())),
        (
            "LieBracket",
            arrow(type0(), arrow(type0(), arrow(type0(), type0()))),
        ),
        ("HamiltonianFlow", arrow(type0(), arrow(real_ty(), type0()))),
        (
            "PoissonBracket",
            arrow(
                arrow(type0(), real_ty()),
                arrow(arrow(type0(), real_ty()), arrow(type0(), real_ty())),
            ),
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
/// Dot product of two 3-vectors
pub fn dot3(a: &[f64; 3], b: &[f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
/// Cross product of two 3-vectors
pub fn cross3(a: &[f64; 3], b: &[f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
/// Norm of a 3-vector
pub fn norm3(a: &[f64; 3]) -> f64 {
    dot3(a, a).sqrt()
}
/// Normalize a 3-vector (returns zero vector if near-zero)
pub fn normalize3(a: &[f64; 3]) -> [f64; 3] {
    let n = norm3(a);
    if n < 1e-12 {
        [0.0, 0.0, 0.0]
    } else {
        [a[0] / n, a[1] / n, a[2] / n]
    }
}
/// Subtract two 3-vectors
pub fn sub3(a: &[f64; 3], b: &[f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
/// Add two 3-vectors
pub fn add3(a: &[f64; 3], b: &[f64; 3]) -> [f64; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}
/// Scale a 3-vector
pub fn scale3(a: &[f64; 3], s: f64) -> [f64; 3] {
    [a[0] * s, a[1] * s, a[2] * s]
}
/// Compute the sign of a permutation: +1 if even, -1 if odd.
pub fn permutation_sign(perm: &[usize]) -> f64 {
    let n = perm.len();
    let mut visited = vec![false; n];
    let mut sign = 1.0_f64;
    for i in 0..n {
        if visited[i] {
            continue;
        }
        let mut cycle_len = 0;
        let mut j = i;
        while !visited[j] {
            visited[j] = true;
            j = perm[j];
            cycle_len += 1;
        }
        if cycle_len % 2 == 0 {
            sign *= -1.0;
        }
    }
    sign
}
#[cfg(test)]
mod tests {
    use super::*;
    const EPS: f64 = 1e-9;
    #[test]
    fn test_first_fundamental_sphere() {
        let r = 3.0_f64;
        let sphere = Sphere { radius: r };
        let fff = sphere.first_fundamental_form_equator();
        let r2 = r * r;
        assert!((fff.e - r2).abs() < EPS, "E = R^2");
        assert!(fff.f.abs() < EPS, "F = 0");
        assert!((fff.g - r2).abs() < EPS, "G = R^2");
    }
    #[test]
    fn test_gaussian_curvature_sphere() {
        let r = 2.0_f64;
        let sphere = Sphere { radius: r };
        let fff = sphere.first_fundamental_form_equator();
        let sff = sphere.second_fundamental_form_equator();
        let k = sff.gaussian_curvature(&fff);
        assert!((k - 1.0 / (r * r)).abs() < EPS, "K = 1/R^2, got {k}");
    }
    #[test]
    fn test_mean_curvature_sphere() {
        let r = 2.0_f64;
        let sphere = Sphere { radius: r };
        let fff = sphere.first_fundamental_form_equator();
        let sff = sphere.second_fundamental_form_equator();
        let h = sff.mean_curvature(&fff);
        assert!((h - 1.0 / r).abs() < EPS, "H = 1/R, got {h}");
    }
    #[test]
    fn test_principal_curvatures_sphere() {
        let r = 2.0_f64;
        let sphere = Sphere { radius: r };
        let fff = sphere.first_fundamental_form_equator();
        let sff = sphere.second_fundamental_form_equator();
        let (k1, k2) = sff.principal_curvatures(&fff);
        assert!((k1 - 1.0 / r).abs() < EPS, "κ₁ = 1/R, got {k1}");
        assert!((k2 - 1.0 / r).abs() < EPS, "κ₂ = 1/R, got {k2}");
    }
    #[test]
    fn test_torus_euler_char() {
        let torus = Torus {
            major_radius: 3.0,
            minor_radius: 1.0,
        };
        assert_eq!(torus.euler_characteristic(), 0, "χ(T^2) = 0");
    }
    #[test]
    fn test_sphere_area() {
        let r = 5.0_f64;
        let sphere = Sphere { radius: r };
        let expected = 4.0 * std::f64::consts::PI * r * r;
        assert!((sphere.area() - expected).abs() < 1e-10, "area = 4πR²");
    }
    #[test]
    fn test_curve3d_length() {
        let pts: Vec<[f64; 3]> = (0..=4)
            .map(|i| {
                let t = i as f64 * std::f64::consts::PI / 2.0;
                [t.cos(), t.sin(), 0.0]
            })
            .collect();
        let curve = Curve3D::new(pts);
        let len = curve.length();
        assert!(len > 5.0 && len < 6.5, "approximate arc length, got {len}");
    }
    #[test]
    fn test_first_fundamental_form_positive_definite() {
        let fff = FirstFundamentalForm::new(4.0, 1.0, 4.0);
        assert!(fff.is_positive_definite(), "4*4-1 = 15 > 0");
        let bad = FirstFundamentalForm::new(1.0, 2.0, 1.0);
        assert!(!bad.is_positive_definite(), "1*1 - 4 = -3 < 0");
    }
    #[test]
    fn test_riemannian_metric_2d_euclidean() {
        let g = RiemannianMetric2D::euclidean();
        assert!((g.det() - 1.0).abs() < EPS, "det of Euclidean metric = 1");
        let inv = g.inverse();
        assert!((inv[0][0] - 1.0).abs() < EPS);
        assert!((inv[1][1] - 1.0).abs() < EPS);
        assert!(inv[0][1].abs() < EPS);
    }
    #[test]
    fn test_riemannian_metric_2d_christoffel_flat() {
        let g = RiemannianMetric2D::euclidean();
        let dg = [[[0.0f64; 2]; 2]; 2];
        let gamma = g.christoffel(&dg);
        for k in 0..2 {
            for i in 0..2 {
                for j in 0..2 {
                    assert!(
                        gamma[k][i][j].abs() < EPS,
                        "Γ^{k}_{i}{j} = 0 for flat metric, got {}",
                        gamma[k][i][j]
                    );
                }
            }
        }
    }
    #[test]
    fn test_geodesic_integrator_straight_line() {
        let g = RiemannianMetric2D::euclidean();
        let dg = [[[0.0f64; 2]; 2]; 2];
        let gamma = g.christoffel(&dg);
        let mut integrator = GeodesicIntegrator::new([0.0, 0.0], [1.0, 0.0]);
        let traj = integrator.integrate(&g, &gamma, 0.1, 10);
        assert_eq!(traj.len(), 11);
        assert!(
            (traj[10][0] - 1.0).abs() < 1e-10,
            "x ≈ 1, got {}",
            traj[10][0]
        );
        assert!(traj[10][1].abs() < 1e-10, "y ≈ 0, got {}", traj[10][1]);
    }
    #[test]
    fn test_differential_form_wedge_anticommutativity() {
        let dx = DifferentialFormWedge::basis_1form(2, 0);
        let dy = DifferentialFormWedge::basis_1form(2, 1);
        let dx_dy = dx.wedge(&dy);
        let dy_dx = dy.wedge(&dx);
        assert!(!dx_dy.is_zero(), "dx ∧ dy ≠ 0");
        assert_eq!(dx_dy.terms.len(), 1);
        assert_eq!(dy_dx.terms.len(), 1);
        let coeff_pos = dx_dy.terms[0].0;
        let coeff_neg = dy_dx.terms[0].0;
        assert!((coeff_pos + coeff_neg).abs() < EPS, "dx∧dy = -dy∧dx");
    }
    #[test]
    fn test_differential_form_wedge_nilpotency() {
        let dx = DifferentialFormWedge::basis_1form(3, 0);
        let dx_dx = dx.wedge(&dx);
        assert!(dx_dx.is_zero(), "dx ∧ dx = 0");
    }
    #[test]
    fn test_lie_group_so3_identity() {
        let r = LieGroupSO3::identity();
        assert!(r.is_valid(), "identity is a valid rotation");
        let v = [1.0, 2.0, 3.0];
        let rv = r.apply(&v);
        assert!((rv[0] - v[0]).abs() < EPS);
        assert!((rv[1] - v[1]).abs() < EPS);
        assert!((rv[2] - v[2]).abs() < EPS);
    }
    #[test]
    fn test_lie_group_so3_rodrigues_rotation_z() {
        let omega = [0.0, 0.0, std::f64::consts::FRAC_PI_2];
        let r = LieGroupSO3::from_axis_angle(&omega);
        assert!(r.is_valid(), "rotation is valid");
        let v = [1.0, 0.0, 0.0];
        let rv = r.apply(&v);
        let expected = [0.0, 1.0, 0.0];
        assert!((rv[0] - expected[0]).abs() < 1e-10, "x → 0, got {}", rv[0]);
        assert!((rv[1] - expected[1]).abs() < 1e-10, "y → 1, got {}", rv[1]);
        assert!((rv[2] - expected[2]).abs() < 1e-10, "z → 0, got {}", rv[2]);
    }
    #[test]
    fn test_lie_group_so3_log_map() {
        let omega = [0.5, -0.3, 0.7];
        let r = LieGroupSO3::from_axis_angle(&omega);
        let recovered = r.log_map();
        let diff = norm3(&sub3(&recovered, &omega));
        assert!(diff < 1e-10, "log(exp(ω)) ≈ ω, diff = {diff}");
    }
    #[test]
    fn test_hodge_star_r3_1forms() {
        let hodge = HodgeStar::new(3);
        let dx = DifferentialFormWedge::basis_1form(3, 0);
        let star_dx = hodge.apply(&dx);
        assert_eq!(star_dx.terms.len(), 1, "★dx has one term");
        assert_eq!(star_dx.terms[0].1, vec![1, 2], "★dx = dy∧dz");
        assert!((star_dx.terms[0].0 - 1.0).abs() < EPS, "coefficient 1");
    }
    #[test]
    fn test_hodge_star_double_application() {
        let hodge = HodgeStar::new(3);
        let dx = DifferentialFormWedge::basis_1form(3, 0);
        let star_dx = hodge.apply(&dx);
        let star_star_dx = hodge.apply(&star_dx);
        assert_eq!(star_star_dx.terms.len(), 1);
        assert_eq!(star_star_dx.terms[0].1, vec![0], "★★dx = dx");
        assert!(
            (star_star_dx.terms[0].0 - 1.0).abs() < EPS,
            "coefficient = 1"
        );
    }
    #[test]
    fn test_riemannian_metric_3d_euclidean() {
        let g = RiemannianMetric3D::euclidean();
        assert!((g.det() - 1.0).abs() < EPS, "det = 1");
        assert!((g.volume_element() - 1.0).abs() < EPS, "vol element = 1");
        let inv = g.inverse();
        assert!((inv[0][0] - 1.0).abs() < EPS);
        assert!((inv[1][1] - 1.0).abs() < EPS);
        assert!((inv[2][2] - 1.0).abs() < EPS);
    }
    #[test]
    fn test_riemannian_metric_3d_christoffel_flat() {
        let g = RiemannianMetric3D::euclidean();
        let dg = [[[0.0f64; 3]; 3]; 3];
        let gamma = g.christoffel(&dg);
        for k in 0..3 {
            for i in 0..3 {
                for j in 0..3 {
                    assert!(
                        gamma[k][i][j].abs() < EPS,
                        "Γ^{k}_{i}{j} = 0, got {}",
                        gamma[k][i][j]
                    );
                }
            }
        }
    }
    #[test]
    fn test_build_differential_geometry_env() {
        let mut env = oxilean_kernel::Environment::new();
        build_differential_geometry_env(&mut env);
        let check_names = [
            "Atlas",
            "Chart",
            "TransitionMap",
            "CotangentBundle",
            "VectorBundle",
            "DifferentialForm",
            "ExteriorDerivative",
            "IntegrationForm",
            "DeRhamCohomology",
            "LieGroup",
            "LieAlgebra",
            "LieExpMap",
            "PrincipalBundle",
            "Connection1Form",
            "Curvature2Form",
            "SectionalCurvature",
            "RicciTensor",
            "ScalarCurvature",
            "ChernWeil",
            "CharacteristicClass",
            "HodgeStar",
            "HarmonicForm",
            "SymplecticForm",
            "HamiltonianVectorField",
            "MomentMap",
            "SymplecticReduction",
            "ContactStructure",
            "ContactManifold",
            "ReebVectorField",
            "JacobiField",
            "stokes_theorem",
            "de_rham_theorem",
            "bianchi_identity",
            "einstein_equations",
            "hodge_decomposition",
            "darboux_theorem",
            "geodesic_equation",
        ];
        for name in check_names {
            assert!(
                env.get(&oxilean_kernel::Name::str(name)).is_some(),
                "axiom '{name}' not registered"
            );
        }
    }
    #[test]
    fn test_so3_compose_inverse() {
        let omega = [0.3, 0.5, -0.2];
        let r = LieGroupSO3::from_axis_angle(&omega);
        let r_inv = r.transpose();
        let prod = r.compose(&r_inv);
        let id = LieGroupSO3::identity();
        for i in 0..3 {
            for j in 0..3 {
                assert!(
                    (prod.matrix[i][j] - id.matrix[i][j]).abs() < 1e-10,
                    "R * R^T ≈ I at [{i}][{j}]"
                );
            }
        }
    }
    #[test]
    fn test_lorentzian_metric_2d_signatures() {
        let g = LorentzianMetric2D::minkowski();
        let time_vec = [1.0_f64, 0.0_f64];
        assert!(g.is_timelike(&time_vec), "pure time vector is timelike");
        let space_vec = [0.0_f64, 1.0_f64];
        assert!(g.is_spacelike(&space_vec), "pure space vector is spacelike");
        let null_vec = [1.0_f64, 1.0_f64];
        assert!(g.is_null(&null_vec), "lightlike vector is null");
    }
    #[test]
    fn test_lorentzian_metric_2d_proper_time() {
        let g = LorentzianMetric2D::minkowski();
        let dt_rest = g.proper_time_element(0.0);
        assert!((dt_rest - 1.0).abs() < 1e-10, "at rest: dτ/dt = 1");
        let dt_half = g.proper_time_element(0.5);
        let expected = 0.75_f64.sqrt();
        assert!(
            (dt_half - expected).abs() < 1e-10,
            "half c: dτ/dt = sqrt(3)/2"
        );
    }
    #[test]
    fn test_randers_finsler_norm() {
        let f = RandersFinsler::new(2.0, 0.5).expect("RandersFinsler::new should succeed");
        assert!(
            f.is_strongly_convex(),
            "valid Randers metric is strongly convex"
        );
        assert!((f.norm(1.0) - 2.5).abs() < EPS, "F(1) = 2.5");
        assert!((f.norm(-1.0) - 1.5).abs() < EPS, "F(-1) = 1.5");
    }
    #[test]
    fn test_randers_finsler_invalid() {
        let f = RandersFinsler::new(1.0, 1.5);
        assert!(f.is_none(), "beta >= alpha is invalid");
    }
    #[test]
    fn test_holonomy_flat_space() {
        let gamma = [[[0.0f64; 2]; 2]; 2];
        let computer = HolonomyComputer::new(gamma);
        let angle = computer.holonomy_angle_square_loop(0.1);
        assert!(
            angle.abs() < 1e-10,
            "flat space holonomy angle = 0, got {angle}"
        );
    }
    #[test]
    fn test_calibration_area_form() {
        let checker = CalibrationChecker::area_form_r3();
        let u = [1.0_f64, 0.0, 0.0];
        let v = [0.0_f64, 1.0, 0.0];
        assert!(
            checker.is_calibrated(&u, &v),
            "xy-plane is calibrated by dx∧dy"
        );
        let val = checker.evaluate(&u, &v);
        assert!((val - 1.0).abs() < EPS, "φ(e1,e2) = 1");
    }
    #[test]
    fn test_weyl_vanishes_dim3() {
        let w = WeylTensorComputer::new(3, 0.0);
        assert!(w.weyl_vanishes_in_dim3(), "Weyl tensor vanishes in dim 3");
    }
    #[test]
    fn test_weyl_coefficients_dim4() {
        let w = WeylTensorComputer::new(4, 12.0);
        assert!(
            (w.ricci_coefficient() - (-0.5)).abs() < EPS,
            "Ricci coeff = -0.5"
        );
        assert!(
            (w.metric_coefficient() - 2.0).abs() < EPS,
            "metric coeff = 2.0"
        );
    }
    #[test]
    fn test_build_differential_geometry_env_new_axioms() {
        let mut env = oxilean_kernel::Environment::new();
        build_differential_geometry_env(&mut env);
        let new_axioms = [
            "SubRiemannianMetric",
            "FinslerMetric",
            "LorentzianMetric",
            "CausalFuture",
            "SpinStructure",
            "DiracOperator",
            "WeylTensor",
            "CRStructure",
            "CartanGeometry",
            "HolonomyGroup",
            "KahlerEinsteinMetric",
            "AlexandrovSpace",
            "chow_rashevskii",
            "lichnerowicz_formula",
            "yau_theorem",
            "berger_classification",
            "bishop_gromov",
            "gromov_hausdorff",
        ];
        for name in new_axioms {
            assert!(
                env.get(&oxilean_kernel::Name::str(name)).is_some(),
                "new axiom '{name}' not registered"
            );
        }
    }
}
#[cfg(test)]
mod tests_diffgeo_extra {
    use super::*;
    #[test]
    fn test_riemann_metric() {
        let g = RiemannMetric::euclidean(3);
        let u = vec![1.0, 0.0, 0.0];
        let v = vec![0.0, 1.0, 0.0];
        assert!((g.inner_product(&u, &u) - 1.0).abs() < 1e-9);
        assert!((g.inner_product(&u, &v)).abs() < 1e-9);
        assert!((g.norm(&u) - 1.0).abs() < 1e-9);
    }
    #[test]
    fn test_metric_det_2d() {
        let g = RiemannMetric::euclidean(2);
        assert!((g.det_2d().expect("det_2d should succeed") - 1.0).abs() < 1e-9);
        let mut g2 = RiemannMetric::new(2);
        g2.set_component(0, 0, 4.0);
        g2.set_component(1, 1, 9.0);
        assert!((g2.det_2d().expect("det_2d should succeed") - 36.0).abs() < 1e-9);
    }
    #[test]
    fn test_christoffel_flat() {
        let cs = ChristoffelSymbols::new(3);
        assert!(cs.is_flat());
    }
    #[test]
    fn test_geodesic() {
        let g = Geodesic::new(vec![0.0, 0.0], vec![3.0, 4.0], 10);
        assert!((g.euclidean_length() - 5.0).abs() < 1e-9);
        let mid = g.point_at(0.5);
        assert!((mid[0] - 1.5).abs() < 1e-9);
        assert!((mid[1] - 2.0).abs() < 1e-9);
        let pts = g.sample_points();
        assert_eq!(pts.len(), 11);
    }
    #[test]
    fn test_differential_form() {
        let vol = DifferentialForm::volume_form(3);
        assert!(vol.is_closed);
        assert!(vol.is_top_form());
        assert_eq!(vol.space_dimension(), 1);
        let f2 = DifferentialForm::new(2, 4);
        assert_eq!(f2.space_dimension(), 6);
    }
    #[test]
    fn test_curvature_tensor() {
        let flat = CurvatureTensor::flat(3);
        assert!(flat.is_einstein(0.0));
        assert!((flat.ricci_scalar() - 0.0).abs() < 1e-9);
        let s3 = CurvatureTensor::sphere_unit(3);
        assert!((s3.scalar_curvature - 6.0).abs() < 1e-9);
        assert!(s3.is_einstein(2.0));
    }
}
