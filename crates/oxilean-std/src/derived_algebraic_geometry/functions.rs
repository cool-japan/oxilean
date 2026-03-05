//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    AInfAlgebra, AtiyahHirzebruchSS, ConvergenceCriteria, CotangentComplex, DAGTStructure,
    DGAlgebra, DGCategory, DeformationFunctor, DerivedBlowup, DerivedCategory, DerivedFunctor,
    DerivedIntersection, DerivedScheme, EInftyRing, ExactTriangle, FormalModuliProb,
    FormalModuliProblem, HochschildComplexSS, KanFibration, LieAlgebraInfty, MaySpectralSequence,
    ModuleSpectrum, NerveFunctor, ObstructionTheory, QuasiCategory, QuasiIsomorphism,
    ShiftedSymplecticStructure, SimplicialObject, SpectralSchemeData, SpectralSequence,
    SquareZeroExtension, StableInftyCategory, TStructure, TanakaDuality, TensorProduct,
    VirtualFundamentalClass,
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
pub fn type1() -> Expr {
    Expr::Sort(Level::succ(Level::succ(Level::zero())))
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
pub fn int_ty() -> Expr {
    cst("Int")
}
pub fn bool_ty() -> Expr {
    cst("Bool")
}
/// `DerivedCategory : Type 1`
///
/// The derived category D(A) of an abelian category A: objects are chain
/// complexes, morphisms are fractions with quasi-isomorphic denominators.
pub fn derived_category_ty() -> Expr {
    type1()
}
/// `TStructure : DerivedCategory → Type`
///
/// A t-structure on a derived category is a pair of subcategories
/// (D^≤0, D^≥0) satisfying truncation axioms.
pub fn t_structure_ty() -> Expr {
    arrow(cst("DerivedCategory"), type0())
}
/// `ExactTriangle : DerivedCategory → Type`
///
/// A distinguished triangle A → B → C → A[1] in a triangulated category.
pub fn exact_triangle_ty() -> Expr {
    arrow(cst("DerivedCategory"), type0())
}
/// `DerivedFunctor : DerivedCategory → DerivedCategory → Type`
///
/// A left or right derived functor between derived categories.
pub fn derived_functor_ty() -> Expr {
    arrow(
        cst("DerivedCategory"),
        arrow(cst("DerivedCategory"), type0()),
    )
}
/// `RotateTriangle : ExactTriangle D → ExactTriangle D`
///
/// Rotating a distinguished triangle: if A→B→C→A[1] is distinguished,
/// so is B→C→A[1]→B[1].
pub fn rotate_triangle_ty() -> Expr {
    arrow(
        app(cst("ExactTriangle"), cst("DerivedCategory")),
        app(cst("ExactTriangle"), cst("DerivedCategory")),
    )
}
/// `OctahedralAxiom : DerivedCategory → Prop`
///
/// The octahedral axiom: given f: X→Y and g: Y→Z, the cones fit into
/// a distinguished triangle.
pub fn octahedral_axiom_ty() -> Expr {
    arrow(cst("DerivedCategory"), prop())
}
/// `TruncationFunctor : TStructure D → Int → DerivedCategory → DerivedCategory`
///
/// Truncation functors τ≤n and τ≥n associated to a t-structure.
pub fn truncation_functor_ty() -> Expr {
    arrow(
        cst("TStructure"),
        arrow(
            int_ty(),
            arrow(cst("DerivedCategory"), cst("DerivedCategory")),
        ),
    )
}
/// `DGCategory : Type 1`
///
/// A differential graded (dg) category: a category enriched over chain
/// complexes of k-modules, with differential d satisfying d²=0 and the
/// Leibniz rule.
pub fn dg_category_ty() -> Expr {
    type1()
}
/// `DGAlgebra : Type`
///
/// A dg-algebra: a graded associative k-algebra A = ⊕ A^n equipped with
/// a degree-+1 differential d: A^n → A^{n+1} satisfying d²=0 and the
/// graded Leibniz rule d(ab) = d(a)b + (-1)^|a| a·d(b).
pub fn dg_algebra_ty() -> Expr {
    type0()
}
/// `AInfAlgebra : Type`
///
/// An A∞-algebra: a graded vector space A with a sequence of multilinear
/// maps m_n: A^⊗n → A of degree 2−n, satisfying the A∞-relations
/// (generalised associativity up to all higher homotopies).
pub fn a_inf_algebra_ty() -> Expr {
    type0()
}
/// `QuasiIsomorphism : DGAlgebra → DGAlgebra → Type`
///
/// A morphism of dg-algebras that induces an isomorphism on cohomology.
pub fn quasi_isomorphism_ty() -> Expr {
    arrow(cst("DGAlgebra"), arrow(cst("DGAlgebra"), type0()))
}
/// `FormallySmoothDGA : DGAlgebra → Prop`
///
/// Formal smoothness for a dg-algebra: the cotangent complex vanishes
/// (Koszul duality perspective — a formally smooth dga is one whose
/// bar-cobar resolution is a quasi-isomorphism).
pub fn formally_smooth_dga_ty() -> Expr {
    arrow(cst("DGAlgebra"), prop())
}
/// `KoszulDual : DGAlgebra → DGAlgebra`
///
/// The Koszul dual of a Koszul algebra A is its bar construction B(A)
/// (or equivalently the Ext-algebra Ext_A(k,k)).
pub fn koszul_dual_ty() -> Expr {
    arrow(cst("DGAlgebra"), cst("DGAlgebra"))
}
/// `DGFunctor : DGCategory → DGCategory → Type`
///
/// A dg-functor: a functor between dg-categories compatible with the
/// dg-enrichment (preserves differentials and compositions).
pub fn dg_functor_ty() -> Expr {
    arrow(cst("DGCategory"), arrow(cst("DGCategory"), type0()))
}
/// `SimplicialObject : (Type → Type 1)`
///
/// A simplicial object in a category C: a contravariant functor Δ^op → C,
/// specified by objects X_n together with face maps d_i: X_n → X_{n-1}
/// (0≤i≤n) and degeneracy maps s_j: X_n → X_{n+1} (0≤j≤n) satisfying
/// the simplicial identities.
pub fn simplicial_object_ty() -> Expr {
    arrow(type0(), type1())
}
/// `NerveFunctor : Cat → SimplicialSet`
///
/// The nerve of a small category C: the simplicial set NC where NC_n is
/// the set of composable n-tuples of morphisms in C.  The nerve is used
/// to build classifying spaces BG.
pub fn nerve_functor_ty() -> Expr {
    arrow(cst("Cat"), cst("SimplicialSet"))
}
/// `KanFibration : SimplicialSet → SimplicialSet → Prop`
///
/// A Kan fibration p: E → B: every horn inclusion Λ^n_k → Δ^n lifts
/// against p (all horns, including outer horns).
pub fn kan_fibration_ty() -> Expr {
    arrow(cst("SimplicialSet"), arrow(cst("SimplicialSet"), prop()))
}
/// `QuasiCategory : SimplicialSet → Prop`
///
/// A quasi-category (weak Kan complex / (∞,1)-category): every inner
/// horn Λ^n_k → X (0 < k < n) has a filler; outer horns need not fill.
pub fn quasi_category_ty() -> Expr {
    arrow(cst("SimplicialSet"), prop())
}
/// `FaceMap : SimplicialObject → Nat → Nat → Type`
///
/// The i-th face map d_i: X_n → X_{n-1} of a simplicial object.
pub fn face_map_ty() -> Expr {
    arrow(
        cst("SimplicialObject"),
        arrow(nat_ty(), arrow(nat_ty(), type0())),
    )
}
/// `DegeneracyMap : SimplicialObject → Nat → Nat → Type`
///
/// The j-th degeneracy map s_j: X_n → X_{n+1} of a simplicial object.
pub fn degeneracy_map_ty() -> Expr {
    arrow(
        cst("SimplicialObject"),
        arrow(nat_ty(), arrow(nat_ty(), type0())),
    )
}
/// `DerivedScheme : Type 1`
///
/// A derived scheme: a pair (X, O_X) where X is a topological space
/// (or ∞-topos) and O_X is a sheaf of simplicial commutative rings
/// (or E∞-rings in the spectral setting), such that each truncation
/// t_n(X, O_X) is an ordinary scheme.
pub fn derived_scheme_ty() -> Expr {
    type1()
}
/// `DerivedStack : Type 1`
///
/// A derived stack: an ∞-functor F: dAff^op → ∞-Gpd from affine derived
/// schemes to ∞-groupoids, satisfying descent with respect to some
/// Grothendieck topology.
pub fn derived_stack_ty() -> Expr {
    type1()
}
/// `CotangentComplex : DerivedScheme → DerivedScheme → Type`
///
/// The (relative) cotangent complex L_{X/Y}: for a morphism f: X → Y
/// of derived schemes, L_{X/Y} is the derived pullback of the sheaf of
/// relative Kähler differentials along the diagonal.
pub fn cotangent_complex_ty() -> Expr {
    arrow(cst("DerivedScheme"), arrow(cst("DerivedScheme"), type0()))
}
/// `ObstructionTheory : DerivedScheme → Type`
///
/// A tangent-obstruction theory on a derived scheme X: a two-term complex
/// T^1 → T^2 controlling deformations (T^1) and obstructions (T^2).
pub fn obstruction_theory_ty() -> Expr {
    arrow(cst("DerivedScheme"), type0())
}
/// `VirtualFundamentalClass : ObstructionTheory X → Type`
///
/// The virtual fundamental class [X]^vir produced by a perfect obstruction
/// theory, living in the Chow group A_*(X).
pub fn virtual_fundamental_class_ty() -> Expr {
    arrow(cst("ObstructionTheory"), type0())
}
/// `PerfectObstructionTheory : DerivedScheme → Prop`
///
/// A perfect obstruction theory is a two-term perfect complex in degrees
/// [-1, 0] together with a map from the cotangent complex.
pub fn perfect_obstruction_theory_ty() -> Expr {
    arrow(cst("DerivedScheme"), prop())
}
/// `SpectralSequence : Type`
///
/// A spectral sequence {E_r^{p,q}, d_r}: a bigraded collection of modules
/// E_r^{p,q} with differentials d_r of bidegree (r, 1-r) satisfying
/// H(E_r, d_r) ≅ E_{r+1}.
pub fn spectral_sequence_ty() -> Expr {
    type0()
}
/// `SpectralPage : SpectralSequence → Nat → Int → Int → Type`
///
/// The (r, p, q)-page E_r^{p,q} of a spectral sequence.
pub fn spectral_page_ty() -> Expr {
    arrow(
        cst("SpectralSequence"),
        arrow(nat_ty(), arrow(int_ty(), arrow(int_ty(), type0()))),
    )
}
/// `HochschildComplexSS : Type`
///
/// The Hochschild-Serre spectral sequence for a group extension
/// 1 → N → G → Q → 1: E_2^{p,q} = H^p(Q; H^q(N; M)) ⇒ H^{p+q}(G; M).
pub fn hochschild_complex_ss_ty() -> Expr {
    type0()
}
/// `AtiyahHirzebruchSS : Type`
///
/// The Atiyah-Hirzebruch spectral sequence:
/// E_2^{p,q} = H^p(X; π_q(E)) ⇒ E^{p+q}(X)
/// computing generalised cohomology E^*(X) from singular cohomology and
/// the coefficients of E.
pub fn atiyah_hirzebruch_ss_ty() -> Expr {
    type0()
}
/// `ConvergesTo : SpectralSequence → Type → Prop`
///
/// Convergence of a spectral sequence to a graded target: E_r degenerates
/// at page r_0 and the filtration quotients are the graded pieces of the
/// abutment.
pub fn converges_to_ty() -> Expr {
    arrow(cst("SpectralSequence"), arrow(type0(), prop()))
}
/// `FrobeniusReciprocity : SpectralSequence → Prop`
///
/// A formal statement that the edge homomorphisms of a Lyndon-Hochschild-Serre
/// spectral sequence satisfy Frobenius reciprocity.
pub fn frobenius_reciprocity_ty() -> Expr {
    arrow(cst("SpectralSequence"), prop())
}
/// `EInftyRing : Type 1`
///
/// A commutative ring spectrum (E∞-ring): a commutative monoid in the
/// symmetric monoidal ∞-category of spectra Sp, with unit S (sphere
/// spectrum) and associative, commutative multiplication up to all
/// coherent higher homotopies.
pub fn e_infty_ring_ty() -> Expr {
    type1()
}
/// `ModuleSpectrum : EInftyRing → Type 1`
///
/// A module spectrum M over an E∞-ring R: an object of the stable
/// ∞-category Mod_R of R-modules in spectra.
pub fn module_spectrum_ty() -> Expr {
    arrow(cst("EInftyRing"), type1())
}
/// `DerivedTensorProduct : EInftyRing → ModuleSpectrum R → ModuleSpectrum R → ModuleSpectrum R`
///
/// The derived (smash) tensor product M ⊗_R^L N of two R-module spectra,
/// computing the correct homotopy-theoretic tensor product.
pub fn derived_tensor_product_ty() -> Expr {
    arrow(
        cst("EInftyRing"),
        arrow(
            cst("ModuleSpectrum"),
            arrow(cst("ModuleSpectrum"), cst("ModuleSpectrum")),
        ),
    )
}
/// `MaySpectralSequence : Type`
///
/// The May spectral sequence computing the homotopy groups of the sphere
/// spectrum via the Adams-May filtration of the Steenrod algebra:
/// E_2 = Ext_{A(p)}(F_p, F_p) ⇒ π_*(S^0)_(p).
pub fn may_spectral_sequence_ty() -> Expr {
    type0()
}
/// `AdamsSpectralSequence : EInftyRing → EInftyRing → Type`
///
/// The Adams spectral sequence E_2^{s,t} = Ext_{E_*(E)}^{s,t}(E_*, E_*(X)) ⇒ π_{t-s}(X)_p^∧
/// for a Landweber-exact homology theory E.
pub fn adams_spectral_sequence_ty() -> Expr {
    arrow(cst("EInftyRing"), arrow(cst("EInftyRing"), type0()))
}
/// `SphereSpectrum : EInftyRing`
///
/// The sphere spectrum S: the initial E∞-ring, the unit for the smash
/// product of spectra.
pub fn sphere_spectrum_ty() -> Expr {
    cst("EInftyRing")
}
/// `FormalModuliProblem : Type 1`
///
/// A formal moduli problem: an ∞-functor X: CAlg^{aug}_k → ∞-Gpd from
/// augmented artinian commutative dg-algebras to ∞-groupoids, satisfying
/// the Schlessinger conditions (preserving finite products up to homotopy).
pub fn formal_moduli_problem_ty() -> Expr {
    type1()
}
/// `LieAlgebraInfty : Type`
///
/// An ∞-Lie algebra (L∞-algebra): a graded vector space L with a sequence
/// of brackets l_n: L^⊗n → L of degree n−2 satisfying the homotopy Jacobi
/// identity (L∞-relations).  Formal moduli problems correspond to L∞-algebras
/// via the Maurer-Cartan functor.
pub fn lie_algebra_infty_ty() -> Expr {
    type0()
}
/// `DeformationFunctor : FormalModuliProblem → CommRing → Type`
///
/// The deformation functor Def_X(A): the ∞-groupoid of deformations of X
/// over an Artinian local ring A (with residue field k).
pub fn deformation_functor_ty() -> Expr {
    arrow(cst("FormalModuliProblem"), arrow(cst("CommRing"), type0()))
}
/// `TanakaDuality : LieAlgebraInfty → FormalModuliProblem → Prop`
///
/// Tanaka duality / Lurie's theorem: the ∞-category of formal moduli
/// problems over k is equivalent to the ∞-category of L∞-algebras over k,
/// via the Maurer-Cartan ∞-groupoid functor MC(−).
pub fn tanaka_duality_ty() -> Expr {
    arrow(
        cst("LieAlgebraInfty"),
        arrow(cst("FormalModuliProblem"), prop()),
    )
}
/// `MaurerCartanSpace : LieAlgebraInfty → Type`
///
/// The Maurer-Cartan space MC(L): the ∞-groupoid of Maurer-Cartan elements
/// of an L∞-algebra L, i.e. solutions to the equation
/// Σ_{n≥1} (1/n!) l_n(x, …, x) = 0.
pub fn maurer_cartan_space_ty() -> Expr {
    arrow(cst("LieAlgebraInfty"), type0())
}
/// `KoszulDualityFMP : FormalModuliProblem → LieAlgebraInfty → Prop`
///
/// Koszul duality between formal moduli problems and dg Lie algebras:
/// if X is a formal moduli problem with tangent L∞-algebra T_X[−1],
/// then the bar construction B(T_X[−1]) is the Koszul dual.
pub fn koszul_duality_fmp_ty() -> Expr {
    arrow(
        cst("FormalModuliProblem"),
        arrow(cst("LieAlgebraInfty"), prop()),
    )
}
/// Populate an [`Environment`] with axioms for the major DAG constructions.
///
/// Registers all major types, functors, and theorems from derived algebraic
/// geometry as kernel-level axioms, so they can be referenced from OxiLean
/// proofs and elaboration.
pub fn build_derived_algebraic_geometry_env(env: &mut Environment) {
    let base_types: &[(&str, Expr)] = &[
        ("DerivedCategory", type1()),
        ("TStructure", t_structure_ty()),
        ("ExactTriangle", exact_triangle_ty()),
        ("DerivedFunctor", derived_functor_ty()),
        ("DGCategory", type1()),
        ("DGAlgebra", type0()),
        ("AInfAlgebra", type0()),
        ("QuasiIsomorphism", quasi_isomorphism_ty()),
        ("SimplicialSet", type1()),
        ("SimplicialObject", arrow(type0(), type1())),
        ("Cat", type1()),
        ("DerivedScheme", type1()),
        ("DerivedStack", type1()),
        ("ObstructionTheory", arrow(cst("DerivedScheme"), type0())),
        ("SpectralSequence", type0()),
        ("EInftyRing", type1()),
        ("ModuleSpectrum", arrow(cst("EInftyRing"), type1())),
        ("FormalModuliProblem", type1()),
        ("LieAlgebraInfty", type0()),
        ("CommRing", type0()),
    ];
    for (name, ty) in base_types {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    let type_axioms: &[(&str, fn() -> Expr)] = &[
        ("derived_category", derived_category_ty),
        ("t_structure", t_structure_ty),
        ("exact_triangle", exact_triangle_ty),
        ("derived_functor", derived_functor_ty),
        ("rotate_triangle", rotate_triangle_ty),
        ("octahedral_axiom", octahedral_axiom_ty),
        ("truncation_functor", truncation_functor_ty),
        ("dg_category", dg_category_ty),
        ("dg_algebra", dg_algebra_ty),
        ("a_inf_algebra", a_inf_algebra_ty),
        ("quasi_isomorphism", quasi_isomorphism_ty),
        ("formally_smooth_dga", formally_smooth_dga_ty),
        ("koszul_dual", koszul_dual_ty),
        ("dg_functor", dg_functor_ty),
        ("simplicial_object", simplicial_object_ty),
        ("nerve_functor", nerve_functor_ty),
        ("kan_fibration", kan_fibration_ty),
        ("quasi_category", quasi_category_ty),
        ("face_map", face_map_ty),
        ("degeneracy_map", degeneracy_map_ty),
        ("derived_scheme", derived_scheme_ty),
        ("derived_stack", derived_stack_ty),
        ("cotangent_complex", cotangent_complex_ty),
        ("obstruction_theory", obstruction_theory_ty),
        ("virtual_fundamental_class", virtual_fundamental_class_ty),
        ("perfect_obstruction_theory", perfect_obstruction_theory_ty),
        ("spectral_sequence", spectral_sequence_ty),
        ("spectral_page", spectral_page_ty),
        ("hochschild_complex_ss", hochschild_complex_ss_ty),
        ("atiyah_hirzebruch_ss", atiyah_hirzebruch_ss_ty),
        ("converges_to", converges_to_ty),
        ("frobenius_reciprocity", frobenius_reciprocity_ty),
        ("e_infty_ring", e_infty_ring_ty),
        ("module_spectrum", module_spectrum_ty),
        ("derived_tensor_product", derived_tensor_product_ty),
        ("may_spectral_sequence", may_spectral_sequence_ty),
        ("adams_spectral_sequence", adams_spectral_sequence_ty),
        ("sphere_spectrum", sphere_spectrum_ty),
        ("formal_moduli_problem", formal_moduli_problem_ty),
        ("lie_algebra_infty", lie_algebra_infty_ty),
        ("deformation_functor", deformation_functor_ty),
        ("tanaka_duality", tanaka_duality_ty),
        ("maurer_cartan_space", maurer_cartan_space_ty),
        ("koszul_duality_fmp", koszul_duality_fmp_ty),
    ];
    for (name, mk_ty) in type_axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: mk_ty(),
        })
        .ok();
    }
}
/// Compute the virtual dimension of a derived scheme given the ranks of T^1 and T^2.
pub fn virtual_dimension(rank_t1: i32, rank_t2: i32) -> i32 {
    rank_t1 - rank_t2
}
/// Return the differential bidegree (r, 1−r) for page r of a spectral sequence.
pub fn differential_bidegree(r: i32) -> (i32, i32) {
    (r, 1 - r)
}
/// Check the simplicial identity d_i ∘ d_j = d_{j+1} ∘ d_i for i ≤ j (index only).
pub fn simplicial_face_identity(i: usize, j: usize) -> bool {
    i <= j
}
/// Check the A∞ relation index: m_1 ∘ m_n + Σ_{j} ±m_{n-1}(id^⊗j ⊗ m_2 ⊗ id^⊗(n-j-1)) + … = 0
/// This returns the number of terms in the A∞ relation for arity n.
pub fn a_inf_relation_terms(n: usize) -> usize {
    n
}
#[cfg(test)]
mod tests {
    use super::*;
    use oxilean_kernel::{Environment, Name};
    #[test]
    fn test_build_derived_algebraic_geometry_env() {
        let mut env = Environment::new();
        build_derived_algebraic_geometry_env(&mut env);
        assert!(env.get(&Name::str("DerivedCategory")).is_some());
        assert!(env.get(&Name::str("DGAlgebra")).is_some());
        assert!(env.get(&Name::str("SimplicialSet")).is_some());
        assert!(env.get(&Name::str("EInftyRing")).is_some());
        assert!(env.get(&Name::str("FormalModuliProblem")).is_some());
        assert!(env.get(&Name::str("cotangent_complex")).is_some());
        assert!(env.get(&Name::str("tanaka_duality")).is_some());
        assert!(env.get(&Name::str("octahedral_axiom")).is_some());
        assert!(env.get(&Name::str("quasi_category")).is_some());
        assert!(env.get(&Name::str("virtual_fundamental_class")).is_some());
    }
    #[test]
    fn test_exact_triangle_rotate() {
        let t = ExactTriangle::new("A", "B", "C");
        assert!(t.is_distinguished_triangle());
        assert!(t.octahedral_axiom_holds());
        let t2 = t.rotate();
        assert_eq!(t2.vertex_a, "B");
        assert_eq!(t2.vertex_b, "C");
        assert_eq!(t2.vertex_c, "A[1]");
        let t3 = t2.rotate();
        assert_eq!(t3.vertex_a, "C");
        assert_eq!(t3.vertex_b, "A[1]");
        assert_eq!(t3.vertex_c, "B[1]");
    }
    #[test]
    fn test_derived_category_display() {
        let db = DerivedCategory::bounded("Coh(X)");
        assert!(db.is_bounded());
        assert!(db.has_t_structure);
        let s = format!("{}", db);
        assert!(s.contains("bounded"));
        let d = DerivedCategory::unbounded("Mod(R)");
        assert!(!d.is_bounded());
        let s2 = format!("{}", d);
        assert!(s2.contains("unbounded"));
    }
    #[test]
    fn test_dg_algebra_de_rham() {
        let omega = DGAlgebra::de_rham("M");
        assert_eq!(omega.grading, "cohomological");
        assert!(omega.differential_squares_zero);
        let s = format!("{}", omega);
        assert!(s.contains("DGA"));
    }
    #[test]
    fn test_a_inf_from_dga() {
        let dga = DGAlgebra::cohomological("A", "k");
        let ainf = AInfAlgebra::from_dga(&dga);
        assert_eq!(ainf.max_composition_order, 2);
        assert!(ainf.relations_verified);
        let s = format!("{}", ainf);
        assert!(s.contains("A∞"));
    }
    #[test]
    fn test_spectral_sequence_serre() {
        let ss = SpectralSequence::serre("F", "E", "B");
        assert_eq!(ss.start_page, 2);
        assert_eq!(ss.differential_bidegree, (2, -1));
        assert_eq!(ss.convergence, ConvergenceCriteria::Strong);
        let s = format!("{}", ss);
        assert!(s.contains("SS"));
    }
    #[test]
    fn test_hochschild_serre_e2() {
        let hs = HochschildComplexSS::new("N", "G", "Q", "M");
        let e2 = hs.e2_page();
        assert!(e2.contains("H^p"));
        let s = format!("{}", hs);
        assert!(s.contains("HS-SS"));
    }
    #[test]
    fn test_atiyah_hirzebruch_e2() {
        let ahss = AtiyahHirzebruchSS::new("X", "KU");
        let e2 = ahss.e2_page();
        assert!(e2.contains("KU"));
        let s = format!("{}", ahss);
        assert!(s.contains("AHSS"));
    }
    #[test]
    fn test_e_infty_ring_sphere() {
        let s = EInftyRing::sphere();
        assert!(s.is_sphere_spectrum);
        let disp = format!("{}", s);
        assert!(disp.contains("sphere"));
        let hq = EInftyRing::eilenberg_maclane("ℚ");
        assert!(!hq.is_sphere_spectrum);
    }
    #[test]
    fn test_module_spectrum_display() {
        let r = EInftyRing::new("R");
        let m = ModuleSpectrum::new("M", &r);
        let s = format!("{}", m);
        assert!(s.contains("R"));
        assert!(s.contains("M"));
    }
    #[test]
    fn test_tanaka_duality() {
        let lie = LieAlgebraInfty::dg_lie("g");
        let fmp = FormalModuliProblem::new("Def_G", "G-torsors").with_tangent_lie("g");
        let td = TanakaDuality::new(lie, fmp);
        assert!(td.verify_equivalence());
        let s = format!("{}", td);
        assert!(s.contains("Tanaka"));
    }
    #[test]
    fn test_deformation_functor() {
        let def = DeformationFunctor::new("X", "T_X[-1]");
        let desc = def.deformations_over("k[ε]/ε²");
        assert!(desc.contains("Def_X"));
        let s = format!("{}", def);
        assert!(s.contains("Def_X"));
    }
    #[test]
    fn test_quasi_category() {
        let qc = QuasiCategory::new("C");
        assert!(qc.inner_horns_fill);
        assert!(!qc.is_kan_complex(false));
        assert!(qc.is_kan_complex(true));
    }
    #[test]
    fn test_simplicial_object_level() {
        let so: SimplicialObject<u32> = SimplicialObject::new(vec![1, 2, 3, 4]);
        assert_eq!(so.level(0), Some(&1));
        assert_eq!(so.level(3), Some(&4));
        assert_eq!(so.level(4), None);
    }
    #[test]
    fn test_cotangent_complex() {
        let lxy = CotangentComplex::new("X", "Y");
        assert!(!lxy.is_formally_smooth());
        let s = format!("{}", lxy);
        assert!(s.contains("L_"));
    }
    #[test]
    fn test_virtual_fundamental_class() {
        let vfc = VirtualFundamentalClass::new("M", 3);
        assert_eq!(vfc.virtual_dim, 3);
        let s = format!("{}", vfc);
        assert!(s.contains("[M]^vir"));
    }
    #[test]
    fn test_virtual_dimension() {
        assert_eq!(virtual_dimension(5, 3), 2);
        assert_eq!(virtual_dimension(0, 0), 0);
    }
    #[test]
    fn test_differential_bidegree() {
        assert_eq!(differential_bidegree(2), (2, -1));
        assert_eq!(differential_bidegree(3), (3, -2));
        assert_eq!(differential_bidegree(1), (1, 0));
    }
    #[test]
    fn test_may_spectral_sequence() {
        let may = MaySpectralSequence::at_prime(2);
        let e2 = may.e2_description();
        assert!(e2.contains("E_2"));
        let s = format!("{}", may);
        assert!(s.contains("p=2"));
    }
    #[test]
    fn test_convergence_display() {
        assert_eq!(format!("{}", ConvergenceCriteria::Weak), "weakly converges");
        assert_eq!(
            format!("{}", ConvergenceCriteria::Strong),
            "strongly converges"
        );
        assert_eq!(
            format!("{}", ConvergenceCriteria::Conditional),
            "conditionally converges"
        );
    }
    #[test]
    fn test_derived_scheme_affine() {
        let x = DerivedScheme::affine("A");
        assert!(x.is_affine);
        assert!(x.classical().contains("H^0"));
        let s = format!("{}", x);
        assert!(s.contains("DerivedScheme"));
    }
    #[test]
    fn test_nerve_functor() {
        let n = NerveFunctor::of("C");
        let desc = n.n_simplices_description(2);
        assert!(desc.contains("2-tuple"));
        let s = format!("{}", n);
        assert!(s.contains("N(C)"));
    }
    #[test]
    fn test_kan_fibration() {
        let fib = KanFibration::new("E", "B");
        assert!(fib.is_kan);
        let s = format!("{}", fib);
        assert!(s.contains("KanFib"));
    }
    #[test]
    fn test_t_structure_heart() {
        let ts = TStructure::standard("D^b(Coh(X))");
        let h = ts.heart_description();
        assert!(h.contains("Heart"));
        let s = format!("{}", ts);
        assert!(s.contains("t-structure"));
    }
    #[test]
    fn test_derived_functor_display() {
        let lf = DerivedFunctor::left("F", "D(A)", "D(B)");
        assert!(lf.is_left);
        assert!(lf.name.starts_with('L'));
        let s = format!("{}", lf);
        assert!(s.contains("left"));
        let rf = DerivedFunctor::right("G", "D(A)", "D(B)");
        assert!(!rf.is_left);
        let s2 = format!("{}", rf);
        assert!(s2.contains("right"));
    }
    #[test]
    fn test_dg_category() {
        let dgcat = DGCategory::dg_modules("A");
        assert!(dgcat.is_pre_triangulated);
        let s = format!("{}", dgcat);
        assert!(s.contains("dgCat"));
    }
    #[test]
    fn test_quasi_isomorphism() {
        let qis = QuasiIsomorphism::new("A", "B");
        assert!(qis.cohomology_iso_desc.contains('≅'));
        let s = format!("{}", qis);
        assert!(s.contains("qis"));
    }
    #[test]
    fn test_obstruction_theory() {
        let ot = ObstructionTheory::new("M", "T^1", "T^2", 3);
        assert_eq!(ot.virtual_dim, 3);
        let s = format!("{}", ot);
        assert!(s.contains("ObstrThy"));
    }
    #[test]
    fn test_formal_moduli_problem() {
        let fmp = FormalModuliProblem::new("Def_A", "A∞-algebras").with_tangent_lie("L");
        assert!(fmp.tangent_lie.is_some());
        let s = format!("{}", fmp);
        assert!(s.contains("FMP"));
    }
    #[test]
    fn test_lie_algebra_infty() {
        let l = LieAlgebraInfty::new("L");
        let mc = l.maurer_cartan_description();
        assert!(mc.contains("MC"));
        let s = format!("{}", l);
        assert!(s.contains("L∞"));
    }
    #[test]
    fn test_tensor_product() {
        let tp = TensorProduct::new("M", "N", "R");
        let s = format!("{}", tp);
        assert!(s.contains("⊗"));
    }
    #[test]
    fn test_a_inf_relation_terms() {
        assert_eq!(a_inf_relation_terms(3), 3);
    }
    #[test]
    fn test_simplicial_face_identity() {
        assert!(simplicial_face_identity(0, 1));
        assert!(simplicial_face_identity(2, 2));
        assert!(!simplicial_face_identity(3, 1));
    }
}
#[cfg(test)]
mod tests_dag_ext {
    use super::*;
    #[test]
    fn test_stable_infty_category() {
        let sp = StableInftyCategory::spectra();
        assert!(sp.is_stable);
        assert!(sp.fiber_cofiber_sequence_coincide());
        let oct = sp.octahedral_axiom();
        assert!(oct.contains("stable"));
        let dc = StableInftyCategory::derived_category("Z");
        assert!(dc.triangulated_structure);
    }
    #[test]
    fn test_t_structure() {
        let ts = DAGTStructure::standard("D(Ab)");
        assert!(ts.is_abelian_heart());
        let desc = ts.bbd_description();
        assert!(desc.contains("Standard"));
        let pts = DAGTStructure::perverse("D(X)", "middle");
        let bdesc = pts.bbd_description();
        assert!(bdesc.contains("perverse"));
    }
    #[test]
    fn test_formal_moduli_problem() {
        let fmp = FormalModuliProb::new("DefX", "TX[-1]", "TX[-2]").with_dg_lie("gX");
        assert!(fmp.is_representable);
        let desc = fmp.lurie_pridham_theorem();
        assert!(desc.contains("Lurie-Pridham"));
    }
    #[test]
    fn test_square_zero_extension() {
        let sqz = SquareZeroExtension::new("A", "M");
        let cotangent = sqz.cotangent_complex_description();
        assert!(cotangent.contains("cotangent"));
        let obs = sqz.obstruction_class();
        assert!(obs.contains("Ext"));
    }
    #[test]
    fn test_shifted_symplectic() {
        let ss = ShiftedSymplecticStructure::new("Perf(X)", -2);
        assert!(!ss.is_classical_symplectic());
        let dt = ss.donaldson_thomas_connection();
        assert!(dt.contains("DT"));
        let ms = ShiftedSymplecticStructure::mapping_stack("T*[1]X", "BG", 2);
        assert!(ms.stack_name.contains("Map"));
    }
    #[test]
    fn test_derived_intersection() {
        let di = DerivedIntersection::new("X", "L1", "L2", -1);
        assert_eq!(di.virtual_dimension(), -1);
        let bf = di.behrend_fantechi_obstruction_theory();
        assert!(bf.contains("BF"));
    }
    #[test]
    fn test_spectral_scheme() {
        let ss = SpectralSchemeData::sphere_spectrum();
        assert!(ss.e_infty_ring);
        assert!(ss.is_affine);
        let loc = ss.chromatic_localization(2, 1);
        assert!(loc.contains("chromatic"));
        let ref_str = ss.lurie_sag_reference();
        assert!(ref_str.contains("Lurie"));
    }
}
#[cfg(test)]
mod tests_dag_ext2 {
    use super::*;
    #[test]
    fn test_derived_blowup() {
        let db = DerivedBlowup::new("X", "Z");
        assert!(db.rees_algebra_description().contains("Rees"));
        assert_eq!(db.derived_correction, -1);
    }
}
