//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    AbundanceData, BlowUpData, ContractionType, ExtremeRay, IitakaFibration, KVVanishingData,
    KodairaDim, LogPair, LogPairData, MMPOperation, MMPStepData, MmpFlowchart, MmpStep, MoriCone,
    SarkisovLinkType, SingularityType, ZariskiDecomp,
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
pub fn real_ty() -> Expr {
    cst("Real")
}
pub fn bool_ty() -> Expr {
    cst("Bool")
}
pub fn list_ty(elem: Expr) -> Expr {
    app(cst("List"), elem)
}
/// `FunctionField : Variety → Field`
/// The function field k(X) of an irreducible variety X.
pub fn function_field_ty() -> Expr {
    arrow(cst("Variety"), cst("Field"))
}
/// `RationalMap : Variety → Variety → Type`
/// A rational map f : X ⇢ Y is a morphism defined on a dense open subset.
pub fn rational_map_ty() -> Expr {
    arrow(cst("Variety"), arrow(cst("Variety"), type0()))
}
/// `BirationaEquiv : Variety → Variety → Prop`
/// X and Y are birationally equivalent if there exist inverse rational maps.
pub fn birational_equiv_ty() -> Expr {
    arrow(cst("Variety"), arrow(cst("Variety"), prop()))
}
/// `birational_equiv_iff_function_fields :
///   X ≅_bir Y ↔ k(X) ≅ k(Y) as field extensions of k`
pub fn birational_equiv_iff_function_fields_ty() -> Expr {
    arrow(
        cst("Variety"),
        arrow(
            cst("Variety"),
            app2(
                cst("Iff"),
                app2(cst("BiratEquiv"), bvar(1), bvar(0)),
                app2(
                    cst("FieldIso"),
                    app(cst("FunctionField"), bvar(1)),
                    app(cst("FunctionField"), bvar(0)),
                ),
            ),
        ),
    )
}
/// `luröth_theorem : Every intermediate field k ⊂ K ⊂ k(t) is of the form k(f(t))`
pub fn luroth_theorem_ty() -> Expr {
    arrow(
        cst("Field"),
        arrow(cst("Field"), arrow(cst("Field"), prop())),
    )
}
/// `Resolution : Variety → Morphism`
/// A resolution of singularities is a proper birational morphism π : X̃ → X
/// where X̃ is smooth.
pub fn resolution_ty() -> Expr {
    arrow(cst("Variety"), cst("Morphism"))
}
/// `hironaka_resolution : ∀ (X : Variety) (char k = 0), ∃ (π : X̃ → X), smooth X̃ ∧ birational π`
/// Hironaka's theorem: every variety over a field of characteristic 0 has a resolution.
pub fn hironaka_resolution_ty() -> Expr {
    arrow(
        cst("Variety"),
        arrow(
            prop(),
            arrow(
                cst("Morphism"),
                app2(
                    cst("And"),
                    app(cst("IsSmooth"), bvar(0)),
                    app(cst("IsBirational"), bvar(0)),
                ),
            ),
        ),
    )
}
/// `embedded_resolution : ∀ (X : Variety) (Z : ClosedSubscheme X),
///   ∃ (π : X̃ → X), smooth X̃ ∧ π^{-1}(Z) is a normal crossing divisor`
pub fn embedded_resolution_ty() -> Expr {
    arrow(
        cst("Variety"),
        arrow(
            cst("ClosedSubscheme"),
            arrow(
                cst("Morphism"),
                app2(
                    cst("And"),
                    app(cst("IsSmooth"), cst("BlownUp")),
                    app(cst("IsNormalCrossing"), cst("ExceptionalPreimage")),
                ),
            ),
        ),
    )
}
/// `BlowUp : Variety → Subscheme → Variety`
/// The blow-up of X along a closed subscheme Z.
pub fn blow_up_ty() -> Expr {
    arrow(cst("Variety"), arrow(cst("Subscheme"), cst("Variety")))
}
/// `ExceptionalDivisor : BlowUp → Divisor`
/// The exceptional divisor E = π^{-1}(Z) ⊂ Bl_Z X.
pub fn exceptional_divisor_ty() -> Expr {
    arrow(cst("BlowUpData"), cst("Divisor"))
}
/// `blow_up_projective_bundle : Bl_Z X is a P^{r-1}-bundle over Z`
/// where r = codim(Z, X) and the exceptional divisor is P(N_{Z/X}).
pub fn blow_up_projective_bundle_ty() -> Expr {
    arrow(
        cst("Variety"),
        arrow(
            cst("Subscheme"),
            arrow(
                nat_ty(),
                app(cst("IsProjBundle"), cst("ExceptionalDivisor")),
            ),
        ),
    )
}
/// `normal_bundle_exceptional : N_{E/Bl_Z X} ≅ O_{P(N)}(-1)`
pub fn normal_bundle_exceptional_ty() -> Expr {
    arrow(
        cst("BlowUpData"),
        app2(
            cst("Iso"),
            app(cst("NormalBundle"), cst("ExceptionalDivisor")),
            app(cst("Tautological"), cst("ExceptionalDivisor")),
        ),
    )
}
/// `blow_up_chow_ring : A*(Bl_Z X) ≅ A*(X)[ξ]/(χ_r + c_1 ξ^{r-1} + … + c_r, ξ^r + j_* s_0 + …)`
pub fn blow_up_chow_ring_ty() -> Expr {
    arrow(
        cst("Variety"),
        arrow(
            cst("Subscheme"),
            app2(cst("Iso"), cst("ChowRing"), cst("BlowUpChowRing")),
        ),
    )
}
/// `CanonicalDivisor : SmoothVariety → Divisor`
/// K_X = det(Ω^1_X) = ∧^n Ω^1_X for a smooth n-fold.
pub fn canonical_divisor_ty() -> Expr {
    arrow(cst("SmoothVariety"), cst("Divisor"))
}
/// `Discrepancy : LogPair → Divisor → Rat`
/// The discrepancy a(E, X, Δ) measures how singular (X, Δ) is along E.
pub fn discrepancy_ty() -> Expr {
    arrow(cst("LogPair"), arrow(cst("Divisor"), cst("Rat")))
}
/// `IsKLT : LogPair → Prop`
/// (X, Δ) is Kawamata log-terminal if all discrepancies a(E, X, Δ) > -1.
pub fn is_klt_ty() -> Expr {
    arrow(cst("LogPair"), prop())
}
/// `IsDLT : LogPair → Prop`
/// (X, Δ) is divisorially log-terminal if all discrepancies a(E, X, Δ) > -1
/// for exceptional E, and for non-exceptional E: coefficients ≤ 1.
pub fn is_dlt_ty() -> Expr {
    arrow(cst("LogPair"), prop())
}
/// `IsLC : LogPair → Prop`
/// (X, Δ) is log-canonical if all discrepancies a(E, X, Δ) ≥ -1.
pub fn is_lc_ty() -> Expr {
    arrow(cst("LogPair"), prop())
}
/// `adjunction_formula : K_Y + Δ_Y = (K_X + Δ_X)|_Y` for a log pair (X, Δ) and divisor Y.
pub fn adjunction_formula_ty() -> Expr {
    arrow(
        cst("LogPair"),
        arrow(
            cst("Divisor"),
            app2(cst("Eq"), cst("Divisor"), cst("RestrictedKanDiv")),
        ),
    )
}
/// `ExtremalRay : Variety → NE_Curve`
/// An extremal ray R ⊂ NE(X) satisfies K_X · R < 0 and is a half-line.
pub fn extremal_ray_ty() -> Expr {
    arrow(cst("Variety"), cst("NECurve"))
}
/// `ContractionMorphism : ExtremalRay → Morphism`
/// By the Contraction Theorem, to each extremal ray there is a contraction morphism.
pub fn contraction_morphism_ty() -> Expr {
    arrow(cst("NECurve"), cst("Morphism"))
}
/// `mori_cone_theorem : NE(X) = NE(X)_{K≥0} + ∑_i R_i`
/// (finite sum for varieties with klt log pairs)
pub fn mori_cone_theorem_ty() -> Expr {
    arrow(
        cst("Variety"),
        arrow(
            prop(),
            app2(cst("Eq"), cst("NECone"), cst("SumExtremalRays")),
        ),
    )
}
/// `base_point_free_theorem : If K_X + Δ is nef and (X, Δ) is klt,
///   then K_X + mΔ is base-point-free for m ≥ m_0`.
pub fn base_point_free_theorem_ty() -> Expr {
    arrow(cst("LogPair"), arrow(prop(), arrow(nat_ty(), prop())))
}
/// `kawamata_viehweg_vanishing : H^i(X, K_X + L) = 0 for i > 0 when L is nef and big`
pub fn kawamata_viehweg_vanishing_ty() -> Expr {
    arrow(
        cst("SmoothVariety"),
        arrow(
            cst("Divisor"),
            arrow(
                prop(),
                arrow(nat_ty(), app2(cst("Eq"), type0(), cst("ZeroModule"))),
            ),
        ),
    )
}
/// `Flip : LogPair → LogPair`
/// A flip replaces a K-negative extremal contraction by a K-positive one.
pub fn flip_ty() -> Expr {
    arrow(cst("LogPair"), cst("LogPair"))
}
/// `Flop : Variety → Variety`
/// A flop is a birational map f : X ⇢ X+ where K_X ≡ 0 on the flipping curves.
pub fn flop_ty() -> Expr {
    arrow(cst("Variety"), cst("Variety"))
}
/// `DivisorialContraction : LogPair → Morphism`
/// A divisorial contraction contracts a divisor (rather than a small set).
pub fn divisorial_contraction_ty() -> Expr {
    arrow(cst("LogPair"), cst("Morphism"))
}
/// `flip_existence : ∀ (X, Δ : LogPair) (f : Morphism), IsFlipping f → ∃ flip f^+`
/// Hacon-McKernan flip existence theorem.
pub fn flip_existence_ty() -> Expr {
    arrow(
        cst("LogPair"),
        arrow(
            cst("Morphism"),
            arrow(prop(), arrow(cst("LogPair"), prop())),
        ),
    )
}
/// `flop_symmetry : K_{X+} ≡ 0 on flopped curves, and X and X+ are derived equivalent`
pub fn flop_symmetry_ty() -> Expr {
    arrow(
        cst("Variety"),
        arrow(
            cst("Variety"),
            app2(cst("And"), cst("KNumericallyTrivial"), cst("DerivedEquiv")),
        ),
    )
}
/// `flop_terminates : Every sequence of flips terminates (BCHM theorem)`
pub fn flop_terminates_ty() -> Expr {
    arrow(cst("LogPair"), prop())
}
/// `KodairaDimension : Variety → Int`
/// κ(X) ∈ {-∞, 0, 1, …, dim X} classifies varieties by growth of pluricanonical sections.
pub fn kodaira_dimension_ty() -> Expr {
    arrow(cst("Variety"), int_ty())
}
/// `IitakaFibration : Variety → Morphism`
/// For κ(X) ≥ 0, the Iitaka fibration φ : X → Y has dim Y = κ(X).
pub fn iitaka_fibration_ty() -> Expr {
    arrow(cst("Variety"), cst("Morphism"))
}
/// `kodaira_dim_additive : κ(X × Y) = κ(X) + κ(Y)` (Iitaka's C_{n,m} conjecture cases)
pub fn kodaira_dim_additive_ty() -> Expr {
    arrow(
        cst("Variety"),
        arrow(
            cst("Variety"),
            app2(
                cst("Eq"),
                int_ty(),
                app(
                    cst("KodairaDimension"),
                    app2(cst("Product"), bvar(1), bvar(0)),
                ),
            ),
        ),
    )
}
/// `castelnuovo_mumford_regularity : Reg(F) = min { m : H^i(F(m-i)) = 0, i > 0 }`
pub fn castelnuovo_mumford_regularity_ty() -> Expr {
    arrow(cst("CoherentSheaf"), int_ty())
}
/// `IsFano : Variety → Prop`
/// X is Fano if -K_X is ample (and X is smooth projective).
pub fn is_fano_ty() -> Expr {
    arrow(cst("Variety"), prop())
}
/// `FanoIndex : FanoVariety → Nat`
/// The Fano index ι(X) = max { r ∈ Z_{>0} : -K_X = r H for some ample H }.
pub fn fano_index_ty() -> Expr {
    arrow(cst("FanoVariety"), nat_ty())
}
/// `DelPezzoSurface : Nat → Variety`
/// Del Pezzo surface S_d of degree d = K_{S_d}^2 ∈ {1, …, 9}.
pub fn del_pezzo_surface_ty() -> Expr {
    arrow(nat_ty(), cst("Variety"))
}
/// `kobayashi_ochiai : If ι(X) > dim X, then X ≅ P^n; if ι(X) = dim X, then X is a quadric`
pub fn kobayashi_ochiai_ty() -> Expr {
    arrow(
        cst("FanoVariety"),
        arrow(prop(), app2(cst("Iso"), bvar(1), cst("ProjectiveSpace"))),
    )
}
/// `mori_mukai_classification : Classification of Fano 3-folds (105 deformation families)`
pub fn mori_mukai_classification_ty() -> Expr {
    arrow(cst("FanoVariety"), arrow(nat_ty(), prop()))
}
/// `LogMinimalModel : LogPair → LogPair`
/// A log-minimal model (X', Δ') of (X, Δ) is a birational model with K_{X'} + Δ' nef.
pub fn log_minimal_model_ty() -> Expr {
    arrow(cst("LogPair"), cst("LogPair"))
}
/// `abundance_conjecture : If (X, Δ) is klt and K_X + Δ is nef, then K_X + Δ is semi-ample`
pub fn abundance_conjecture_ty() -> Expr {
    arrow(cst("LogPair"), arrow(prop(), arrow(prop(), prop())))
}
/// `bchm_theorem : Existence of log-minimal models for klt pairs of general type`
/// (Birkar-Cascini-Hacon-McKernan 2010)
pub fn bchm_theorem_ty() -> Expr {
    arrow(cst("LogPair"), arrow(prop(), arrow(cst("LogPair"), prop())))
}
/// `non_vanishing_theorem : H^0(X, m(K_X + Δ)) ≠ 0 for some m > 0 when nef + effective`
pub fn non_vanishing_theorem_ty() -> Expr {
    arrow(cst("LogPair"), arrow(prop(), arrow(prop(), prop())))
}
/// `cone_theorem : NE(X) = NE(X)_{K≥0} + ∑ R_i, with R_i extremal and K_X · R_i < 0`
/// Mori's cone theorem (also called the Cone Theorem).
pub fn cone_theorem_ty() -> Expr {
    arrow(
        cst("LogPair"),
        arrow(
            prop(),
            app2(cst("Eq"), cst("NECone"), cst("SumExtremalRaysKNeg")),
        ),
    )
}
/// `contraction_theorem : ∀ extremal ray R ⊂ NE(X), ∃ contraction morphism cont_R : X → Z`
pub fn contraction_theorem_ty() -> Expr {
    arrow(cst("Variety"), arrow(cst("NECurve"), cst("Morphism")))
}
/// `MoriFiberSpace : Variety → Variety → Prop`
/// X → Z is a Mori fiber space if K_X is relatively anti-ample and dim Z < dim X.
pub fn mori_fiber_space_ty() -> Expr {
    arrow(cst("Variety"), arrow(cst("Variety"), prop()))
}
/// `rationality_theorem : The length of an extremal ray is at most 2 dim X + 1`
/// (Mori's rationality theorem for smooth varieties)
pub fn rationality_theorem_ty() -> Expr {
    arrow(
        cst("SmoothVariety"),
        arrow(
            cst("NECurve"),
            app2(cst("Le"), cst("RayLength"), cst("RayBound")),
        ),
    )
}
/// `IsTerminalSingularity : Variety → Prop`
/// Terminal singularities: all discrepancies a(E, X, 0) > 0 for exceptional E over X.
pub fn is_terminal_singularity_ty() -> Expr {
    arrow(cst("Variety"), prop())
}
/// `IsCanonicalSingularity : Variety → Prop`
/// Canonical singularities: all discrepancies a(E, X, 0) ≥ 0 for exceptional E over X.
pub fn is_canonical_singularity_ty() -> Expr {
    arrow(cst("Variety"), prop())
}
/// `terminal_implies_canonical : IsTerminal X → IsCanonical X`
pub fn terminal_implies_canonical_ty() -> Expr {
    arrow(
        cst("Variety"),
        arrow(
            app(cst("IsTerminal"), bvar(0)),
            app(cst("IsCanonical"), bvar(1)),
        ),
    )
}
/// `IsKltSingularity : LogPair → Prop`
/// (X, Δ) has klt (Kawamata log-terminal) singularities.
pub fn is_klt_singularity_ty() -> Expr {
    arrow(cst("LogPair"), prop())
}
/// `IsLogCanonicalPair : LogPair → Prop`
/// (X, Δ) is a log-canonical pair.
pub fn is_log_canonical_pair_ty() -> Expr {
    arrow(cst("LogPair"), prop())
}
/// `kodaira_surface_classification : Every minimal surface has Kodaira dimension in {-∞,0,1,2}`
pub fn kodaira_surface_classification_ty() -> Expr {
    arrow(
        cst("SmoothVariety"),
        arrow(prop(), app(cst("KodairaDimInRange"), bvar(1))),
    )
}
/// `IsCurve : Variety → Prop`
/// X is a curve (dimension 1).
pub fn is_curve_ty() -> Expr {
    arrow(cst("Variety"), prop())
}
/// `IsRationalSurface : Variety → Prop`
/// X is birationally equivalent to P^2.
pub fn is_rational_surface_ty() -> Expr {
    arrow(cst("Variety"), prop())
}
/// `IsK3Surface : Variety → Prop`
/// X is a K3 surface: K_X ≅ O_X and H^1(O_X) = 0.
pub fn is_k3_surface_ty() -> Expr {
    arrow(cst("Variety"), prop())
}
/// `IsEnriquesSurface : Variety → Prop`
/// X is an Enriques surface: 2K_X ≅ O_X and K_X ≇ O_X.
pub fn is_enriques_surface_ty() -> Expr {
    arrow(cst("Variety"), prop())
}
/// `IsGeneralTypeSurface : Variety → Prop`
/// X has Kodaira dimension 2 (i.e. κ(X) = 2 = dim X).
pub fn is_general_type_surface_ty() -> Expr {
    arrow(cst("Variety"), prop())
}
/// `IsRuledSurface : Variety → Prop`
/// X is birationally equivalent to C × P^1 for some curve C.
pub fn is_ruled_surface_ty() -> Expr {
    arrow(cst("Variety"), prop())
}
/// `castelnuovo_rationality : If q(X) = 0 and P_2(X) = 0, then X is rational`
/// Castelnuovo's rationality criterion for surfaces.
pub fn castelnuovo_rationality_ty() -> Expr {
    arrow(
        cst("SmoothVariety"),
        arrow(prop(), arrow(prop(), app(cst("IsRational"), bvar(2)))),
    )
}
/// `IsMinimalSurface : Variety → Prop`
/// A surface with no (-1)-curves (irreducible rational curves with self-intersection -1).
pub fn is_minimal_surface_ty() -> Expr {
    arrow(cst("Variety"), prop())
}
/// `grauert_riemenschneider_vanishing :
///   R^i π_* ω_X = 0 for i > 0, where π : X → Y is a birational morphism and X smooth`
pub fn grauert_riemenschneider_vanishing_ty() -> Expr {
    arrow(
        cst("SmoothVariety"),
        arrow(cst("Morphism"), arrow(nat_ty(), prop())),
    )
}
/// `kodaira_vanishing : H^i(X, K_X + L) = 0 for i > 0 when L is ample`
pub fn kodaira_vanishing_ty() -> Expr {
    arrow(
        cst("SmoothVariety"),
        arrow(cst("Divisor"), arrow(prop(), arrow(nat_ty(), prop()))),
    )
}
/// `nadel_vanishing : H^i(X, K_X + L) = 0 for i > 0 given the multiplier ideal`
pub fn nadel_vanishing_ty() -> Expr {
    arrow(
        cst("Variety"),
        arrow(cst("Divisor"), arrow(prop(), arrow(nat_ty(), prop()))),
    )
}
/// `ZariskiDecomposition : Divisor → (Divisor × Divisor)`
/// A pseudoeffective divisor D = P + N where P is nef and N is effective (Zariski decomposition).
pub fn zariski_decomposition_ty() -> Expr {
    arrow(
        cst("Divisor"),
        app2(cst("Prod"), cst("Divisor"), cst("Divisor")),
    )
}
/// `zariski_decomposition_uniqueness : The Zariski decomposition is unique when it exists`
pub fn zariski_decomposition_uniqueness_ty() -> Expr {
    arrow(cst("Divisor"), arrow(prop(), prop()))
}
/// `SarkisovLink : MoriModel → MoriModel → Nat`
/// A Sarkisov link of type I, II, III, or IV connecting two Mori fiber spaces.
pub fn sarkisov_link_ty() -> Expr {
    arrow(cst("MoriModel"), arrow(cst("MoriModel"), nat_ty()))
}
/// `sarkisov_program : Every birational map between Mori fiber spaces factors as Sarkisov links`
pub fn sarkisov_program_ty() -> Expr {
    arrow(
        cst("MoriModel"),
        arrow(cst("MoriModel"), arrow(prop(), prop())),
    )
}
/// `IsRationallyConnected : Variety → Prop`
/// X is rationally connected if any two points can be connected by a rational curve.
pub fn is_rationally_connected_ty() -> Expr {
    arrow(cst("Variety"), prop())
}
/// `IsNonUniruled : Variety → Prop`
/// X is non-uniruled if no rational curve passes through a general point.
pub fn is_non_uniruled_ty() -> Expr {
    arrow(cst("Variety"), prop())
}
/// `miyaoka_mori_theorem :
///   If K_X is not nef then X is uniruled (covered by rational curves)`
pub fn miyaoka_mori_theorem_ty() -> Expr {
    arrow(
        cst("SmoothVariety"),
        arrow(prop(), app(cst("IsUniruled"), bvar(1))),
    )
}
/// `rationally_connected_implies_fano_type :
///   If X is rationally connected then κ(X) = -∞`
pub fn rationally_connected_implies_fano_type_ty() -> Expr {
    arrow(
        cst("Variety"),
        arrow(
            app(cst("IsRationallyConnected"), bvar(0)),
            app2(cst("Eq"), int_ty(), app(cst("KodairaDimension"), bvar(1))),
        ),
    )
}
/// `bogomolov_miyaoka_yau : c_1(X)^2 ≤ 3 c_2(X) for minimal surface of general type`
pub fn bogomolov_miyaoka_yau_ty() -> Expr {
    arrow(
        cst("SmoothVariety"),
        arrow(
            prop(),
            app2(
                cst("Le"),
                app(cst("ChernNumberSq"), bvar(1)),
                app(cst("ChernNumberTwo"), bvar(1)),
            ),
        ),
    )
}
/// `noether_inequality : For a minimal surface of general type, p_g(X) ≤ (1/2) K_X^2 + 2`
pub fn noether_inequality_ty() -> Expr {
    arrow(cst("SmoothVariety"), arrow(prop(), prop()))
}
/// `IsMoriDreamSpace : Variety → Prop`
/// X is a Mori dream space if its Cox ring is finitely generated.
pub fn is_mori_dream_space_ty() -> Expr {
    arrow(cst("Variety"), prop())
}
/// `CoxRing : Variety → Ring`
/// The Cox ring (total coordinate ring) of a variety.
pub fn cox_ring_ty() -> Expr {
    arrow(cst("Variety"), cst("Ring"))
}
/// `mori_dream_space_mmp : In a Mori dream space, every MMP terminates`
pub fn mori_dream_space_mmp_ty() -> Expr {
    arrow(
        cst("Variety"),
        arrow(app(cst("IsMoriDreamSpace"), bvar(0)), prop()),
    )
}
/// `fano_is_mori_dream_space : Every Fano variety is a Mori dream space`
pub fn fano_is_mori_dream_space_ty() -> Expr {
    arrow(cst("FanoVariety"), app(cst("IsMoriDreamSpace"), bvar(0)))
}
/// `flip_termination_bchm : Termination of flips for klt pairs of general type`
/// (Birkar-Cascini-Hacon-McKernan 2010, the main BCHM result for termination)
pub fn flip_termination_bchm_ty() -> Expr {
    arrow(cst("LogPair"), arrow(prop(), prop()))
}
/// `special_termination : Termination of log flips near the boundary`
pub fn special_termination_ty() -> Expr {
    arrow(cst("LogPair"), arrow(prop(), prop()))
}
/// `IsFlipping : Morphism → Prop`
/// A flipping contraction: small contraction where K_X is relatively negative.
pub fn is_flipping_ty() -> Expr {
    arrow(cst("Morphism"), prop())
}
/// `IsFlopping : Morphism → Prop`
/// A flopping contraction: small contraction where K_X is numerically trivial.
pub fn is_flopping_ty() -> Expr {
    arrow(cst("Morphism"), prop())
}
/// `relative_mmp : For f : X → S and (X, Δ) klt, there exists a relative minimal model`
pub fn relative_mmp_ty() -> Expr {
    arrow(
        cst("LogPair"),
        arrow(cst("Morphism"), arrow(prop(), cst("LogPair"))),
    )
}
/// Register all birational geometry declarations in the kernel environment.
pub fn build_birational_geometry_env(env: &mut Environment) {
    let base_types: &[(&str, Expr)] = &[
        ("Variety", type1()),
        ("SmoothVariety", type1()),
        ("FanoVariety", type1()),
        ("Field", type0()),
        ("Divisor", type0()),
        ("NECurve", type0()),
        ("NECone", type0()),
        ("SumExtremalRays", cst("NECone")),
        ("SumExtremalRaysKNeg", cst("NECone")),
        ("Morphism", type0()),
        ("Subscheme", type0()),
        ("ClosedSubscheme", type0()),
        ("BlowUpData", type0()),
        ("LogPair", type0()),
        ("Rat", type0()),
        ("ChowRing", type0()),
        ("BlowUpChowRing", type0()),
        ("CoherentSheaf", type0()),
        ("ZeroModule", type0()),
        ("DerivedEquiv", prop()),
        ("KNumericallyTrivial", prop()),
        ("ExceptionalDivisor", cst("Divisor")),
        ("ExceptionalPreimage", cst("Divisor")),
        ("BlownUp", cst("Variety")),
        ("RestrictedKanDiv", cst("Divisor")),
        ("Nat", type0()),
        ("Int", type0()),
        ("Real", type0()),
        ("Bool", type0()),
        ("Ring", type0()),
        ("MoriModel", type0()),
        ("RayLength", nat_ty()),
        ("RayBound", nat_ty()),
        ("Eq", arrow(type0(), arrow(type0(), prop()))),
        ("Le", arrow(nat_ty(), arrow(nat_ty(), prop()))),
        ("And", arrow(prop(), arrow(prop(), prop()))),
        ("Iff", arrow(prop(), arrow(prop(), prop()))),
        ("Iso", arrow(type0(), arrow(type0(), prop()))),
        ("Prod", arrow(type0(), arrow(type0(), type0()))),
        ("FieldIso", arrow(cst("Field"), arrow(cst("Field"), prop()))),
        (
            "BiratEquiv",
            arrow(cst("Variety"), arrow(cst("Variety"), prop())),
        ),
        ("FunctionField", arrow(cst("Variety"), cst("Field"))),
        ("IsSmooth", arrow(cst("Morphism"), prop())),
        ("IsBirational", arrow(cst("Morphism"), prop())),
        ("IsNormalCrossing", arrow(cst("Divisor"), prop())),
        ("IsProjBundle", arrow(cst("Divisor"), prop())),
        ("Tautological", arrow(cst("Divisor"), type0())),
        ("NormalBundle", arrow(cst("Divisor"), type0())),
        (
            "Product",
            arrow(cst("Variety"), arrow(cst("Variety"), cst("Variety"))),
        ),
        ("KodairaDimension", arrow(cst("Variety"), int_ty())),
        ("ProjectiveSpace", cst("Variety")),
        ("List", arrow(type0(), type0())),
        ("IsTerminal", arrow(cst("Variety"), prop())),
        ("IsCanonical", arrow(cst("Variety"), prop())),
        ("KodairaDimInRange", arrow(cst("Variety"), prop())),
        ("IsRational", arrow(cst("Variety"), prop())),
        ("IsUniruled", arrow(cst("Variety"), prop())),
        ("IsRationallyConnected", arrow(cst("Variety"), prop())),
        ("IsMoriDreamSpace", arrow(cst("Variety"), prop())),
        ("CoxRing", arrow(cst("Variety"), cst("Ring"))),
        ("ChernNumberSq", arrow(cst("Variety"), nat_ty())),
        ("ChernNumberTwo", arrow(cst("Variety"), nat_ty())),
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
        ("function_field", function_field_ty),
        ("rational_map", rational_map_ty),
        ("birational_equiv", birational_equiv_ty),
        ("resolution", resolution_ty),
        ("blow_up", blow_up_ty),
        ("exceptional_divisor", exceptional_divisor_ty),
        ("canonical_divisor", canonical_divisor_ty),
        ("discrepancy", discrepancy_ty),
        ("is_klt", is_klt_ty),
        ("is_dlt", is_dlt_ty),
        ("is_lc", is_lc_ty),
        ("extremal_ray", extremal_ray_ty),
        ("contraction_morphism", contraction_morphism_ty),
        ("flip", flip_ty),
        ("flop", flop_ty),
        ("divisorial_contraction", divisorial_contraction_ty),
        ("kodaira_dimension", kodaira_dimension_ty),
        ("iitaka_fibration", iitaka_fibration_ty),
        (
            "castelnuovo_mumford_regularity",
            castelnuovo_mumford_regularity_ty,
        ),
        ("is_fano", is_fano_ty),
        ("fano_index", fano_index_ty),
        ("del_pezzo_surface", del_pezzo_surface_ty),
        ("log_minimal_model", log_minimal_model_ty),
        ("mori_fiber_space", mori_fiber_space_ty),
        ("is_terminal_singularity", is_terminal_singularity_ty),
        ("is_canonical_singularity", is_canonical_singularity_ty),
        ("is_klt_singularity", is_klt_singularity_ty),
        ("is_log_canonical_pair", is_log_canonical_pair_ty),
        ("is_curve", is_curve_ty),
        ("is_rational_surface", is_rational_surface_ty),
        ("is_k3_surface", is_k3_surface_ty),
        ("is_enriques_surface", is_enriques_surface_ty),
        ("is_general_type_surface", is_general_type_surface_ty),
        ("is_ruled_surface", is_ruled_surface_ty),
        ("is_minimal_surface", is_minimal_surface_ty),
        ("zariski_decomposition", zariski_decomposition_ty),
        ("sarkisov_link", sarkisov_link_ty),
        ("is_rationally_connected", is_rationally_connected_ty),
        ("is_non_uniruled", is_non_uniruled_ty),
        ("is_mori_dream_space", is_mori_dream_space_ty),
        ("cox_ring", cox_ring_ty),
        ("is_flipping", is_flipping_ty),
        ("is_flopping", is_flopping_ty),
        ("relative_mmp", relative_mmp_ty),
    ];
    for (name, mk_ty) in type_axioms {
        let ty = mk_ty();
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty,
        })
        .ok();
    }
    let theorem_axioms: &[(&str, fn() -> Expr)] = &[
        (
            "birational_equiv_iff_function_fields",
            birational_equiv_iff_function_fields_ty,
        ),
        ("luroth_theorem", luroth_theorem_ty),
        ("hironaka_resolution", hironaka_resolution_ty),
        ("embedded_resolution", embedded_resolution_ty),
        ("blow_up_projective_bundle", blow_up_projective_bundle_ty),
        ("normal_bundle_exceptional", normal_bundle_exceptional_ty),
        ("blow_up_chow_ring", blow_up_chow_ring_ty),
        ("adjunction_formula", adjunction_formula_ty),
        ("mori_cone_theorem", mori_cone_theorem_ty),
        ("base_point_free_theorem", base_point_free_theorem_ty),
        ("kawamata_viehweg_vanishing", kawamata_viehweg_vanishing_ty),
        ("flip_existence", flip_existence_ty),
        ("flop_symmetry", flop_symmetry_ty),
        ("flop_terminates", flop_terminates_ty),
        ("kodaira_dim_additive", kodaira_dim_additive_ty),
        ("kobayashi_ochiai", kobayashi_ochiai_ty),
        ("mori_mukai_classification", mori_mukai_classification_ty),
        ("abundance_conjecture", abundance_conjecture_ty),
        ("bchm_theorem", bchm_theorem_ty),
        ("non_vanishing_theorem", non_vanishing_theorem_ty),
        ("cone_theorem", cone_theorem_ty),
        ("contraction_theorem", contraction_theorem_ty),
        ("rationality_theorem", rationality_theorem_ty),
        ("terminal_implies_canonical", terminal_implies_canonical_ty),
        (
            "kodaira_surface_classification",
            kodaira_surface_classification_ty,
        ),
        ("castelnuovo_rationality", castelnuovo_rationality_ty),
        (
            "grauert_riemenschneider_vanishing",
            grauert_riemenschneider_vanishing_ty,
        ),
        ("kodaira_vanishing", kodaira_vanishing_ty),
        ("nadel_vanishing", nadel_vanishing_ty),
        (
            "zariski_decomposition_uniqueness",
            zariski_decomposition_uniqueness_ty,
        ),
        ("sarkisov_program", sarkisov_program_ty),
        ("miyaoka_mori_theorem", miyaoka_mori_theorem_ty),
        (
            "rationally_connected_implies_fano_type",
            rationally_connected_implies_fano_type_ty,
        ),
        ("bogomolov_miyaoka_yau", bogomolov_miyaoka_yau_ty),
        ("noether_inequality", noether_inequality_ty),
        ("mori_dream_space_mmp", mori_dream_space_mmp_ty),
        ("fano_is_mori_dream_space", fano_is_mori_dream_space_ty),
        ("flip_termination_bchm", flip_termination_bchm_ty),
        ("special_termination", special_termination_ty),
    ];
    for (name, mk_ty) in theorem_axioms {
        let ty = mk_ty();
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty,
        })
        .ok();
    }
}
/// Discrepancy calculation for a weighted blow-up.
///
/// For a smooth variety X and blow-up along a smooth center Z of codimension r,
/// the discrepancy of the exceptional divisor E is a(E, X, Δ) = r - 1 - a·(r-1)
/// where a is the log-canonical threshold contribution.
pub fn discrepancy_smooth_blowup(codim: usize, boundary_coeff: f64) -> f64 {
    let r = codim as f64;
    r - 1.0 - boundary_coeff * (r - 1.0)
}
/// Log-canonical threshold of a hypersurface singularity.
///
/// For a function f defining a hypersurface X = {f = 0} in C^n,
/// the LCT lct(C^n, X) = min(1, n / mult(f))
/// where mult(f) is the multiplicity of f at the origin.
pub fn log_canonical_threshold(ambient_dim: usize, multiplicity: usize) -> f64 {
    if multiplicity == 0 {
        return f64::INFINITY;
    }
    f64::min(1.0, ambient_dim as f64 / multiplicity as f64)
}
/// Kodaira dimension of a product.
///
/// κ(X × Y) = κ(X) + κ(Y) when both are non-negative.
pub fn kodaira_dim_product(kx: i64, ky: i64) -> i64 {
    match (kx, ky) {
        (-1, _) | (_, -1) => -1,
        (a, b) => a + b,
    }
}
/// Classify the Kodaira dimension.
pub fn classify_kodaira_dim(kappa: i64, dim: usize) -> &'static str {
    if kappa < 0 {
        "uniruled (κ = -∞)"
    } else if kappa == 0 {
        "Calabi-Yau type (κ = 0)"
    } else if kappa == dim as i64 {
        "general type (κ = dim X)"
    } else {
        "intermediate Kodaira dimension"
    }
}
/// Fano index of projective space P^n is n+1.
pub fn fano_index_projective_space(n: usize) -> usize {
    n + 1
}
/// Fano index of a quadric Q^n is n.
pub fn fano_index_quadric(n: usize) -> usize {
    n
}
/// Check Kobayashi-Ochiai: if ι(X) > dim X, then X ≅ P^n.
pub fn is_projective_space_by_fano_index(fano_index: usize, dim: usize) -> bool {
    fano_index == dim + 1
}
/// Check Kobayashi-Ochiai: if ι(X) = dim X, then X is a quadric.
pub fn is_quadric_by_fano_index(fano_index: usize, dim: usize) -> bool {
    fano_index == dim
}
/// Estimate the Kodaira dimension from pluricanonical section growth.
///
/// If h^0(X, mK_X) ~ C * m^κ for large m, then κ = κ(X).
/// This function estimates κ from a sequence of dimensions.
pub fn estimate_kodaira_dim(pluricanonical_dims: &[u64]) -> i64 {
    if pluricanonical_dims.is_empty() || *pluricanonical_dims.iter().max().unwrap_or(&0) == 0 {
        return -1;
    }
    let n = pluricanonical_dims.len();
    if n < 2 {
        return 0;
    }
    let first_nonzero = pluricanonical_dims.iter().position(|&d| d > 0);
    let last = pluricanonical_dims[n - 1];
    match first_nonzero {
        None => -1,
        Some(i) => {
            let first = pluricanonical_dims[i];
            if last <= first {
                0
            } else {
                let ratio = last as f64 / first as f64;
                let m_ratio = (n - 1 - i) as f64;
                if m_ratio < 1.0 {
                    return 0;
                }
                let kappa = ratio.ln() / m_ratio.ln();
                kappa.round() as i64
            }
        }
    }
}
/// Check the Bogomolov-Miyaoka-Yau inequality c_1^2 ≤ 3 c_2.
///
/// For a minimal surface of general type, this is a numerical constraint
/// on the Chern numbers. The equality case characterises ball quotients.
pub fn check_bmy_inequality(c1_sq: i64, c2: i64) -> bool {
    c1_sq <= 3 * c2
}
/// Check Noether's inequality for a minimal surface of general type.
///
/// p_g ≤ (1/2) K^2 + 2, equivalently 2 p_g - 4 ≤ K^2.
pub fn check_noether_inequality(p_g: i64, k_sq: i64) -> bool {
    2 * p_g - 4 <= k_sq
}
/// Classify a smooth projective surface by its Kodaira dimension and invariants.
///
/// Returns a string describing the classification in the Kodaira-Enriques table.
pub fn classify_surface(
    kodaira_dim: KodairaDim,
    irregularity: u32,
    geometric_genus: u32,
) -> &'static str {
    match kodaira_dim {
        KodairaDim::NegInfinity => {
            if irregularity == 0 {
                "rational surface"
            } else {
                "ruled surface over a curve"
            }
        }
        KodairaDim::Finite(0) => match (irregularity, geometric_genus) {
            (0, 0) => "Enriques surface",
            (0, 1) => "K3 surface",
            (q, 0) if q > 0 => "bielliptic surface",
            _ => "abelian surface",
        },
        KodairaDim::Finite(1) => "elliptic surface (Kodaira dim 1)",
        KodairaDim::Finite(2) => "surface of general type",
        _ => "unknown surface type",
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use oxilean_kernel::Environment;
    #[test]
    fn test_log_pair_klt() {
        let lp = LogPair::trivial("X")
            .with_boundary("D1", 0.3)
            .with_boundary("D2", 0.5);
        assert!(lp.is_klt());
        assert!(lp.is_log_canonical());
        let lp2 = LogPair::trivial("X").with_boundary("D", 1.0);
        assert!(!lp2.is_klt());
        assert!(lp2.is_log_canonical());
    }
    #[test]
    fn test_discrepancy_smooth_blowup() {
        let d = discrepancy_smooth_blowup(3, 0.0);
        assert!((d - 2.0).abs() < 1e-10);
        let d2 = discrepancy_smooth_blowup(1, 0.0);
        assert!(d2.abs() < 1e-10);
    }
    #[test]
    fn test_lct_hypersurface() {
        assert!((log_canonical_threshold(3, 2) - 1.0).abs() < 1e-10);
        assert!((log_canonical_threshold(2, 3) - 2.0 / 3.0).abs() < 1e-10);
        assert!((log_canonical_threshold(4, 1) - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_kodaira_dim_product() {
        assert_eq!(kodaira_dim_product(0, 0), 0);
        assert_eq!(kodaira_dim_product(1, 2), 3);
        assert_eq!(kodaira_dim_product(-1, 2), -1);
        assert_eq!(kodaira_dim_product(2, -1), -1);
    }
    #[test]
    fn test_mori_cone_p2() {
        let cone = MoriCone::projective_space(2);
        assert_eq!(cone.num_extremal_rays(), 1);
        assert!(cone.is_fano());
        assert_eq!(cone.extremal_rays[0], -3);
    }
    #[test]
    fn test_mori_cone_del_pezzo() {
        let cone = MoriCone::del_pezzo(5);
        assert_eq!(cone.num_extremal_rays(), 5);
        assert!(cone.is_fano());
    }
    #[test]
    fn test_blow_up_data() {
        let bu = BlowUpData::new("P^3", "pt", 3);
        assert_eq!(bu.exceptional_discrepancy(), 2);
        let bu2 = BlowUpData::new("S", "pt", 2);
        assert_eq!(bu2.exceptional_self_intersection(), -1);
    }
    #[test]
    fn test_build_birational_geometry_env() {
        let mut env = Environment::new();
        build_birational_geometry_env(&mut env);
        assert!(env.get(&Name::str("hironaka_resolution")).is_some());
        assert!(env.get(&Name::str("mori_cone_theorem")).is_some());
        assert!(env.get(&Name::str("flip_existence")).is_some());
        assert!(env.get(&Name::str("bchm_theorem")).is_some());
        assert!(env.get(&Name::str("abundance_conjecture")).is_some());
        assert!(env.get(&Name::str("kawamata_viehweg_vanishing")).is_some());
        assert!(env.get(&Name::str("cone_theorem")).is_some());
        assert!(env.get(&Name::str("contraction_theorem")).is_some());
        assert!(env.get(&Name::str("rationality_theorem")).is_some());
        assert!(env.get(&Name::str("terminal_implies_canonical")).is_some());
        assert!(env.get(&Name::str("castelnuovo_rationality")).is_some());
        assert!(env
            .get(&Name::str("grauert_riemenschneider_vanishing"))
            .is_some());
        assert!(env.get(&Name::str("miyaoka_mori_theorem")).is_some());
        assert!(env.get(&Name::str("bogomolov_miyaoka_yau")).is_some());
        assert!(env.get(&Name::str("fano_is_mori_dream_space")).is_some());
        assert!(env.get(&Name::str("flip_termination_bchm")).is_some());
    }
    #[test]
    fn test_kodaira_dim_enum() {
        let kd = KodairaDim::Finite(2);
        assert!(kd.is_general_type(2));
        assert!(!kd.is_uniruled());
        assert_eq!(kd.classify(2), "general type");
        let neg = KodairaDim::NegInfinity;
        assert!(neg.is_uniruled());
        assert!(!neg.is_general_type(3));
        let z = KodairaDim::zero();
        assert_eq!(z.classify(2), "Kodaira dim 0 (CY / K3 / abelian type)");
        let prod = KodairaDim::Finite(1).product(KodairaDim::Finite(2));
        assert_eq!(prod, KodairaDim::Finite(3));
        let prod2 = KodairaDim::NegInfinity.product(KodairaDim::Finite(2));
        assert_eq!(prod2, KodairaDim::NegInfinity);
    }
    #[test]
    fn test_kodaira_dim_ordering() {
        assert!(KodairaDim::NegInfinity < KodairaDim::Finite(0));
        assert!(KodairaDim::Finite(0) < KodairaDim::Finite(1));
        assert!(KodairaDim::Finite(2) > KodairaDim::NegInfinity);
    }
    #[test]
    fn test_mmp_step_enum() {
        let s = MmpStep::Contraction {
            divisor: "E".to_string(),
            singularity_type: SingularityType::Terminal,
        };
        assert!(!s.is_terminal());
        assert!(s.description().contains("E"));
        let f = MmpStep::Flip {
            locus: "C".to_string(),
        };
        assert!(!f.is_terminal());
        let m = MmpStep::MinimalModel;
        assert!(m.is_terminal());
        let fs = MmpStep::FiberSpace {
            base: "B".to_string(),
            fiber_dim: 1,
        };
        assert!(fs.is_terminal());
    }
    #[test]
    fn test_singularity_type_order() {
        let s = SingularityType::Smooth;
        let t = SingularityType::Terminal;
        let lc = SingularityType::LogCanonical;
        assert!(s.at_least_as_mild_as(&t));
        assert!(t.at_least_as_mild_as(&lc));
        assert!(!lc.at_least_as_mild_as(&s));
    }
    #[test]
    fn test_extreme_ray() {
        let ray = ExtremeRay::new("l", -3, ContractionType::FiberType);
        assert!(ray.is_k_negative());
        let step = ray.mmp_step();
        assert!(step.is_terminal());
    }
    #[test]
    fn test_mmp_flowchart() {
        let pair = LogPair::trivial("X").with_boundary("D", 0.3);
        let mut chart = MmpFlowchart::new(pair, 3);
        let steps = vec![
            MmpStep::Contraction {
                divisor: "E1".to_string(),
                singularity_type: SingularityType::Terminal,
            },
            MmpStep::Flip {
                locus: "C".to_string(),
            },
            MmpStep::MinimalModel,
        ];
        chart.run(steps);
        assert_eq!(chart.history.len(), 3);
        assert_eq!(chart.picard_number, 2);
        let summary = chart.summary();
        assert!(summary.contains("Step 1"));
        assert!(summary.contains("Step 3"));
    }
    #[test]
    fn test_zariski_decomp() {
        let z = ZariskiDecomp::new(
            "D",
            vec![("H".to_string(), 2.0)],
            vec![("E".to_string(), 0.5)],
        );
        assert!(z.is_negative_part_effective());
        assert!((z.nef_degree() - 2.0).abs() < 1e-10);
        assert!((z.neg_degree() - 0.5).abs() < 1e-10);
    }
    #[test]
    fn test_bmy_inequality() {
        assert!(check_bmy_inequality(9, 3));
        assert!(!check_bmy_inequality(10, 3));
    }
    #[test]
    fn test_noether_inequality() {
        assert!(check_noether_inequality(3, 4));
        assert!(!check_noether_inequality(5, 2));
    }
    #[test]
    fn test_classify_surface() {
        assert_eq!(
            classify_surface(KodairaDim::NegInfinity, 0, 0),
            "rational surface"
        );
        assert_eq!(
            classify_surface(KodairaDim::NegInfinity, 2, 0),
            "ruled surface over a curve"
        );
        assert_eq!(classify_surface(KodairaDim::Finite(0), 0, 1), "K3 surface");
        assert_eq!(
            classify_surface(KodairaDim::Finite(0), 0, 0),
            "Enriques surface"
        );
        assert_eq!(
            classify_surface(KodairaDim::Finite(2), 0, 0),
            "surface of general type"
        );
    }
    #[test]
    fn test_sarkisov_link_type() {
        let t = SarkisovLinkType::TypeII;
        assert_eq!(t.code(), 2);
        assert!(t.description().contains("both"));
    }
}
#[cfg(test)]
mod tests_birational_ext {
    use super::*;
    #[test]
    fn test_mmp_step() {
        let step = MMPStepData::new(MMPOperation::Flip, "X'").with_exceptional("E");
        assert!(!step.is_final());
        assert!(step.description().contains("Flip"));
        let final_step = MMPStepData::new(MMPOperation::MinimalModel, "X_min");
        assert!(final_step.is_final());
    }
    #[test]
    fn test_log_pair() {
        let mut lp = LogPairData::new("X");
        lp.add_boundary("D1", 0.5);
        lp.add_boundary("D2", 0.3);
        lp.add_log_discrepancy(0.2);
        lp.add_log_discrepancy(0.8);
        assert!(lp.is_klt());
        assert!(lp.is_log_canonical());
        assert_eq!(lp.singularity_type(), "klt (Kawamata log terminal)");
        assert!((lp.total_coefficient() - 0.8).abs() < 1e-10);
    }
    #[test]
    fn test_log_pair_not_klt() {
        let mut lp = LogPairData::new("X");
        lp.add_log_discrepancy(-0.5);
        lp.add_log_discrepancy(-1.0);
        assert!(!lp.is_klt());
        assert!(lp.is_log_canonical());
    }
    #[test]
    fn test_abundance_data() {
        let ad = AbundanceData::new("S", 2)
            .with_kodaira_dim(0)
            .nef()
            .abundant();
        assert!(ad.kx_nef && ad.kx_semi_ample);
        assert!(ad.abundance_known());
        assert!(ad.abundance_status().contains("holds"));
    }
}
#[cfg(test)]
mod tests_birational_ext2 {
    use super::*;
    #[test]
    fn test_kv_vanishing() {
        let kv = KVVanishingData::new("X", 3).nef_and_big();
        assert!(kv.vanishes_at(1));
        assert!(kv.vanishes_at(3));
        assert!(!kv.vanishes_at(0));
        assert!(kv.vanishing_statement().contains("K_X + L"));
    }
}
#[cfg(test)]
mod tests_birational_ext3 {
    use super::*;
    #[test]
    fn test_iitaka_fibration() {
        let if_data = IitakaFibration::new("X", "C", "F", 1);
        assert_eq!(if_data.base_dim, 1);
        assert!(if_data.addition_formula(0, 1));
        assert!(!if_data.is_general_type(3));
        let gt = IitakaFibration::new("X", "pt", "X", 3);
        assert!(gt.is_general_type(3));
    }
}
