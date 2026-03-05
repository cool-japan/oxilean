//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    BooleanValuedModel, CBAForcingPoset, CompleteBA, ConstructibleUniverse, FiniteSupportIteration,
    ForcingPoset, ForcingRelation, GenericExtension, GenericFilter, GenericUltrapower,
    LargeCardinalLevel, MartinsAxiom, MathiasForcingPoset, PName, ProperForcingAxiom,
    SacksForcingPoset,
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
pub fn lam(bi: BinderInfo, name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Lam(bi, Name::str(name), Box::new(dom), Box::new(body))
}
pub fn arrow(a: Expr, b: Expr) -> Expr {
    pi(BinderInfo::Default, "_", a, b)
}
pub fn bvar(n: u32) -> Expr {
    Expr::BVar(n)
}
pub fn set_ty() -> Expr {
    cst("Set")
}
pub fn nat_ty() -> Expr {
    cst("Nat")
}
pub fn bool_ty() -> Expr {
    cst("Bool")
}
pub fn prop_ty() -> Expr {
    prop()
}
pub fn and(p: Expr, q: Expr) -> Expr {
    app2(cst("And"), p, q)
}
pub fn or(p: Expr, q: Expr) -> Expr {
    app2(cst("Or"), p, q)
}
pub fn not(p: Expr) -> Expr {
    app(cst("Not"), p)
}
pub fn iff(p: Expr, q: Expr) -> Expr {
    app2(cst("Iff"), p, q)
}
pub fn forall_set(name: &str, body: Expr) -> Expr {
    pi(BinderInfo::Default, name, set_ty(), body)
}
pub fn forall_prop(name: &str, body: Expr) -> Expr {
    pi(BinderInfo::Default, name, prop(), body)
}
pub fn exists_set(name: &str, body: Expr) -> Expr {
    app(
        cst("Exists"),
        lam(BinderInfo::Default, name, set_ty(), body),
    )
}
pub fn eq_set(x: Expr, y: Expr) -> Expr {
    app3(cst("Eq"), set_ty(), x, y)
}
pub fn mem(x: Expr, y: Expr) -> Expr {
    app2(cst("Mem"), x, y)
}
pub fn subset(x: Expr, y: Expr) -> Expr {
    app2(cst("Subset"), x, y)
}
/// Kunen's inconsistency theorem: there is no non-trivial j: V → V.
pub fn kunen_inconsistency() -> &'static str {
    "Kunen's theorem (ZFC): there is no non-trivial elementary embedding j: V → V. \
     In particular, there are no Reinhardt cardinals in ZFC."
}
/// Type: ForcingPoset — a type equipped with a preorder and maximum element.
pub fn forcing_poset_type_ty() -> Expr {
    type0()
}
/// Type: PartialOrder P — predicate that P is a partial order.
pub fn partial_order_ty() -> Expr {
    arrow(type0(), prop())
}
/// Type: Dense(P, D) — D is a dense subset of P.
/// Dense : (P : Type) → (Set P) → Prop
pub fn dense_subset_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "P",
        type0(),
        arrow(arrow(bvar(0), prop()), prop()),
    )
}
/// Type: Compatible(P, p, q) — p and q are compatible conditions in P.
/// Compatible : (P : Type) → P → P → Prop
pub fn compatible_conditions_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "P",
        type0(),
        arrow(bvar(0), arrow(bvar(1), prop())),
    )
}
/// Type: GenericFilter(P, G) — G is a P-generic filter over M.
/// GenericFilter : (P : Type) → (Set P) → Prop
pub fn generic_filter_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "P",
        type0(),
        arrow(arrow(bvar(0), prop()), prop()),
    )
}
/// Axiom: The Forcing Theorem (truth lemma).
/// ∀ P : ForcingPoset, ∀ G : GenericFilter P, ∀ φ : Prop,
///   (∃ p ∈ G, p ⊩ φ) ↔ M[G] ⊨ φ
pub fn forcing_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "P",
        type0(),
        pi(
            BinderInfo::Default,
            "G",
            arrow(bvar(0), prop()),
            forall_prop(
                "φ",
                iff(
                    app3(cst("GenericForces"), bvar(2), bvar(1), bvar(0)),
                    app3(cst("HoldsInExtension"), bvar(2), bvar(1), bvar(0)),
                ),
            ),
        ),
    )
}
/// Axiom: Definability — the forcing relation is definable in the ground model.
/// ∀ P, Definable M (ForcingRelation P)
pub fn forcing_definability_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "P",
        type0(),
        app2(
            cst("Definable"),
            cst("GroundModel"),
            app(cst("ForcingRelation"), bvar(0)),
        ),
    )
}
/// Axiom: ccc forcing preserves all cardinals.
/// ∀ P, CCC P → PreservesCardinals P
pub fn ccc_preserves_cardinals_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "P",
        type0(),
        arrow(
            app(cst("IsCCC"), bvar(0)),
            app(cst("PreservesCardinals"), bvar(0)),
        ),
    )
}
/// Martin's Axiom: ∀ P : CCC-poset, ∀ family D of < 2^ℵ₀ dense sets,
///   ∃ generic filter G meeting every D in the family.
pub fn martins_axiom_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "P",
        type0(),
        arrow(
            app(cst("IsCCC"), bvar(0)),
            pi(
                BinderInfo::Default,
                "D",
                arrow(arrow(bvar(1), prop()), prop()),
                arrow(
                    app2(cst("FamilyOfDense"), bvar(2), bvar(0)),
                    exists_set("G", app2(cst("MeetsAllDense"), bvar(0), bvar(1))),
                ),
            ),
        ),
    )
}
/// Proper Forcing Axiom: same as MA but for proper forcings.
pub fn proper_forcing_axiom_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "P",
        type0(),
        arrow(
            app(cst("IsProper"), bvar(0)),
            pi(
                BinderInfo::Default,
                "D",
                arrow(arrow(bvar(1), prop()), prop()),
                arrow(
                    app2(cst("FamilyOfDense"), bvar(2), bvar(0)),
                    exists_set("G", app2(cst("MeetsAllDense"), bvar(0), bvar(1))),
                ),
            ),
        ),
    )
}
/// Inaccessible cardinal axiom: ∃ κ, IsInaccessible κ.
pub fn inaccessible_cardinal_ty() -> Expr {
    exists_set("kappa", app(cst("IsInaccessible"), bvar(0)))
}
/// Measurable cardinal axiom: ∃ κ, IsMeasurable κ.
pub fn measurable_cardinal_ty() -> Expr {
    exists_set("kappa", app(cst("IsMeasurable"), bvar(0)))
}
/// Woodin cardinal axiom: ∃ κ, IsWoodin κ.
pub fn woodin_cardinal_ty() -> Expr {
    exists_set("kappa", app(cst("IsWoodin"), bvar(0)))
}
/// Supercompact cardinal axiom: ∃ κ, IsSupercompact κ.
pub fn supercompact_cardinal_ty() -> Expr {
    exists_set("kappa", app(cst("IsSupercompact"), bvar(0)))
}
/// Shoenfield absoluteness: every Σ¹₂ sentence is forcing-absolute.
/// ∀ φ : Sigma12, ∀ G : GenericFilter, HoldsInExtension G φ ↔ Holds φ
pub fn shoenfield_absoluteness_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "φ",
        cst("Sigma12"),
        pi(
            BinderInfo::Default,
            "G",
            cst("GenericFilter"),
            iff(
                app2(cst("HoldsInExt"), bvar(0), bvar(1)),
                app(cst("Holds"), bvar(1)),
            ),
        ),
    )
}
/// V = L implies GCH: if V = L then ∀ α, 2^{ℵ_α} = ℵ_{α+1}.
pub fn v_eq_l_implies_gch_ty() -> Expr {
    arrow(
        cst("AxiomOfConstructibility"),
        forall_set(
            "alpha",
            eq_set(
                app(cst("TwoPow"), app(cst("Aleph"), bvar(0))),
                app(cst("Aleph"), app(cst("OrdSucc"), bvar(0))),
            ),
        ),
    )
}
/// Jensen's covering lemma.
pub fn covering_lemma_ty() -> Expr {
    arrow(
        not(cst("ZeroSharpExists")),
        forall_set(
            "X",
            arrow(
                app(cst("IsUncountable"), bvar(0)),
                exists_set(
                    "Y",
                    and(
                        mem(bvar(1), cst("L")),
                        and(
                            subset(bvar(1), bvar(0)),
                            eq_set(app(cst("Card"), bvar(1)), app(cst("Card"), bvar(0))),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// Build the forcing theory environment with all axioms and theorems.
pub fn build_forcing_theory_env() -> Environment {
    let mut env = Environment::new();
    let base_types: &[(&str, Expr)] = &[
        ("Set", type0()),
        ("Nat", type0()),
        ("Bool", type0()),
        ("Prop", type1()),
        ("And", arrow(prop(), arrow(prop(), prop()))),
        ("Or", arrow(prop(), arrow(prop(), prop()))),
        ("Not", arrow(prop(), prop())),
        ("Iff", arrow(prop(), arrow(prop(), prop()))),
        ("Mem", arrow(set_ty(), arrow(set_ty(), prop()))),
        ("Subset", arrow(set_ty(), arrow(set_ty(), prop()))),
        (
            "Eq",
            pi(
                BinderInfo::Default,
                "T",
                type0(),
                arrow(bvar(0), arrow(bvar(1), prop())),
            ),
        ),
        ("Exists", arrow(arrow(set_ty(), prop()), prop())),
        ("ForcingPoset", type0()),
        ("PartialOrder", arrow(type0(), prop())),
        ("IsCCC", arrow(type0(), prop())),
        ("IsProper", arrow(type0(), prop())),
        ("IsSigmaClosed", arrow(type0(), prop())),
        (
            "DenseSubset",
            arrow(type0(), arrow(arrow(set_ty(), prop()), prop())),
        ),
        (
            "Compatible",
            arrow(type0(), arrow(set_ty(), arrow(set_ty(), prop()))),
        ),
        (
            "GenericFilter",
            arrow(type0(), arrow(arrow(set_ty(), prop()), prop())),
        ),
        (
            "GenericForces",
            arrow(
                type0(),
                arrow(arrow(set_ty(), prop()), arrow(prop(), prop())),
            ),
        ),
        (
            "HoldsInExtension",
            arrow(
                type0(),
                arrow(arrow(set_ty(), prop()), arrow(prop(), prop())),
            ),
        ),
        (
            "HoldsInExt",
            arrow(cst("GenericFilter"), arrow(cst("Sigma12"), prop())),
        ),
        ("Holds", arrow(cst("Sigma12"), prop())),
        ("Definable", arrow(set_ty(), arrow(set_ty(), prop()))),
        ("ForcingRelation", arrow(type0(), set_ty())),
        ("GroundModel", set_ty()),
        ("PreservesCardinals", arrow(type0(), prop())),
        (
            "FamilyOfDense",
            arrow(
                type0(),
                arrow(arrow(arrow(set_ty(), prop()), prop()), prop()),
            ),
        ),
        (
            "MeetsAllDense",
            arrow(
                type0(),
                arrow(arrow(arrow(set_ty(), prop()), prop()), prop()),
            ),
        ),
        ("Ordinal", type0()),
        ("Cardinal", type0()),
        ("OrdSucc", arrow(cst("Ordinal"), cst("Ordinal"))),
        ("Aleph", arrow(cst("Ordinal"), cst("Cardinal"))),
        ("Beth", arrow(cst("Ordinal"), cst("Cardinal"))),
        ("TwoPow", arrow(cst("Cardinal"), cst("Cardinal"))),
        ("Card", arrow(set_ty(), cst("Ordinal"))),
        ("IsUncountable", arrow(set_ty(), prop())),
        ("IsInaccessible", arrow(set_ty(), prop())),
        ("IsMahlo", arrow(set_ty(), prop())),
        ("IsWeaklyCompact", arrow(set_ty(), prop())),
        ("IsMeasurable", arrow(set_ty(), prop())),
        ("IsStrong", arrow(set_ty(), prop())),
        ("IsWoodin", arrow(set_ty(), prop())),
        ("IsSupercompact", arrow(set_ty(), prop())),
        ("IsExtendible", arrow(set_ty(), prop())),
        ("IsHuge", arrow(set_ty(), prop())),
        ("L", set_ty()),
        ("ZeroSharpExists", prop()),
        ("AxiomOfConstructibility", prop()),
        ("Sigma12", type0()),
        ("PName", arrow(type0(), type0())),
        (
            "Interpretation",
            arrow(type0(), arrow(cst("GenericFilter"), type0())),
        ),
        ("Singleton", arrow(set_ty(), set_ty())),
        ("EmptySet", set_ty()),
        ("Union", arrow(set_ty(), arrow(set_ty(), set_ty()))),
    ];
    for (name, ty) in base_types {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    let axioms: &[(&str, fn() -> Expr)] = &[
        ("forcing_theorem", forcing_theorem_ty),
        ("forcing_definability", forcing_definability_ty),
        ("ccc_preserves_cardinals", ccc_preserves_cardinals_ty),
        ("martins_axiom", martins_axiom_ty),
        ("proper_forcing_axiom", proper_forcing_axiom_ty),
        ("inaccessible_cardinal", inaccessible_cardinal_ty),
        ("measurable_cardinal", measurable_cardinal_ty),
        ("woodin_cardinal", woodin_cardinal_ty),
        ("supercompact_cardinal", supercompact_cardinal_ty),
        ("shoenfield_absoluteness", shoenfield_absoluteness_ty),
        ("v_eq_l_implies_gch", v_eq_l_implies_gch_ty),
        ("covering_lemma", covering_lemma_ty),
    ];
    for (name, mk_ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: mk_ty(),
        })
        .ok();
    }
    env
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_forcing_poset_ccc() {
        let cohen = ForcingPoset::cohen_forcing();
        assert!(cohen.satisfies_ccc());
        assert!(cohen.is_proper());
        let sacks = ForcingPoset::sacks_forcing();
        assert!(!sacks.satisfies_ccc());
        assert!(sacks.is_proper());
        let col = ForcingPoset::collapse_forcing("ω_1");
        assert!(!col.satisfies_ccc());
    }
    #[test]
    fn test_generic_filter() {
        let g = GenericFilter::new("Cohen(ω,2)");
        assert!(g.is_generic_over_model);
        assert!(g.meets_all_dense_sets);
        assert!(g.exists_over_countable_model());
    }
    #[test]
    fn test_generic_extension() {
        let g = GenericFilter::new("Cohen(ω,2)");
        let ext = GenericExtension::new("M", g);
        assert!(ext.satisfies_zfc);
        assert!(ext.shoenfield_absoluteness());
        assert!(ext.ground_model_definable());
        assert_eq!(ext.fundamental_theorem(), "M[G] ⊨ φ ↔ ∃ p ∈ G, p ⊩ φ");
    }
    #[test]
    fn test_boolean_valued_model() {
        let bvm = BooleanValuedModel::new("RO(Cohen)");
        assert!(bvm.is_complete);
        assert!(bvm.satisfies_zfc_full_value);
        let classical = BooleanValuedModel::classical_model();
        assert_eq!(classical.boolean_algebra, "2");
        let eq_val = bvm.boolean_equality_value("x", "y");
        assert!(eq_val.contains("x") && eq_val.contains("y"));
    }
    #[test]
    fn test_large_cardinal_levels() {
        assert!(LargeCardinalLevel::Measurable < LargeCardinalLevel::Strong);
        assert!(LargeCardinalLevel::Woodin < LargeCardinalLevel::Supercompact);
        assert!(LargeCardinalLevel::Supercompact < LargeCardinalLevel::IZero);
        assert!(!LargeCardinalLevel::Inaccessible.above_measurable());
        assert!(LargeCardinalLevel::Strong.above_measurable());
        assert!(LargeCardinalLevel::Supercompact.above_measurable());
        let desc = LargeCardinalLevel::Measurable.description();
        assert!(desc.contains("ultrafilter"));
    }
    #[test]
    fn test_martins_axiom() {
        let ma = MartinsAxiom::consistent_with_not_ch();
        assert!(ma.holds);
        assert!(ma.implies_large_continuum());
        assert!(ma.small_sets_are_null());
        assert!(ma.suslin_hypothesis_follows());
        let ma0 = MartinsAxiom::ma_aleph_0();
        assert!(ma0.holds);
        assert!(!ma0.implies_large_continuum());
    }
    #[test]
    fn test_proper_forcing_axiom() {
        let pfa = ProperForcingAxiom::consistent_from_supercompact();
        assert!(pfa.holds);
        assert!(pfa.continuum_equals_aleph_2());
        assert!(pfa.p_ideal_dichotomy());
        assert!(pfa.all_aronszajn_trees_special());
    }
    #[test]
    fn test_constructible_universe() {
        let l = ConstructibleUniverse::new();
        assert!(l.v_eq_l_implies_gch());
        assert!(l.satisfies_ac);
        let msg = ConstructibleUniverse::no_measurables_in_l();
        assert!(msg.contains("measurable"));
    }
    #[test]
    fn test_build_forcing_theory_env() {
        let env = build_forcing_theory_env();
        assert!(env.get(&Name::str("ForcingPoset")).is_some());
        assert!(env.get(&Name::str("GenericFilter")).is_some());
        assert!(env.get(&Name::str("IsCCC")).is_some());
        assert!(env.get(&Name::str("IsProper")).is_some());
        assert!(env.get(&Name::str("IsMeasurable")).is_some());
        assert!(env.get(&Name::str("IsWoodin")).is_some());
        assert!(env.get(&Name::str("IsSupercompact")).is_some());
        assert!(env.get(&Name::str("forcing_theorem")).is_some());
        assert!(env.get(&Name::str("ccc_preserves_cardinals")).is_some());
        assert!(env.get(&Name::str("martins_axiom")).is_some());
        assert!(env.get(&Name::str("proper_forcing_axiom")).is_some());
        assert!(env.get(&Name::str("shoenfield_absoluteness")).is_some());
        assert!(env.get(&Name::str("v_eq_l_implies_gch")).is_some());
        assert!(env.get(&Name::str("covering_lemma")).is_some());
    }
}
/// Build a minimal kernel `Environment` for forcing theory axioms.
///
/// This is an alias for [`build_forcing_theory_env`] exposed under the
/// canonical name `build_env` for consistency with other std modules.
pub fn build_env() -> Environment {
    build_forcing_theory_env()
}
/// Finite-support iterated forcing: (P_α, Q̇_α)_{α<λ} with finite support conditions.
/// FinSupportIteration : (λ : Ordinal) → Type
pub fn fin_support_iteration_ty() -> Expr {
    arrow(cst("Ordinal"), type0())
}
/// Countable-support iterated forcing: conditions have countable support.
/// CSIteration : (λ : Ordinal) → Type
pub fn countable_support_iteration_ty() -> Expr {
    arrow(cst("Ordinal"), type0())
}
/// Easton support iteration: support of each condition is an Easton set.
/// EastonIteration : (λ : Ordinal) → (Ordinal → Ordinal) → Type
pub fn easton_support_iteration_ty() -> Expr {
    arrow(
        cst("Ordinal"),
        arrow(arrow(cst("Ordinal"), cst("Ordinal")), type0()),
    )
}
/// Axiom: finite-support iteration of ccc forcings is ccc.
/// ∀ λ, ∀ iter : FinSupportIteration λ, IsCCC iter
pub fn fin_support_ccc_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "lambda",
        cst("Ordinal"),
        pi(
            BinderInfo::Default,
            "iter",
            app(cst("FinSupportIteration"), bvar(0)),
            app(cst("IsCCC"), bvar(0)),
        ),
    )
}
/// Axiom: countable-support iteration of proper forcings is proper.
/// ∀ λ, ∀ iter : CSIteration λ, AllProper iter → IsProper iter
pub fn countable_support_proper_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "lambda",
        cst("Ordinal"),
        pi(
            BinderInfo::Default,
            "iter",
            app(cst("CSIteration"), bvar(0)),
            arrow(
                app(cst("AllStepsProper"), bvar(0)),
                app(cst("IsProper"), bvar(0)),
            ),
        ),
    )
}
/// Axiom: Easton's theorem — Easton functions are the only constraints on GCH patterns.
/// ∀ F : EastonFunction, ∃ model, CardinalPattern model = F
pub fn eastons_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "F",
        cst("EastonFunction"),
        exists_set("M", app2(cst("CardinalPattern"), bvar(0), bvar(1))),
    )
}
/// Axiom: properness criterion via countable elementary submodels.
/// ∀ P, IsProper P ↔ ∀ M ≺ H_θ countable, ∀ p ∈ P ∩ M, ∃ q ≤ p, q (M,P)-generic
pub fn properness_criterion_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "P",
        type0(),
        iff(
            app(cst("IsProper"), bvar(0)),
            pi(
                BinderInfo::Default,
                "M",
                cst("CountableModel"),
                pi(
                    BinderInfo::Default,
                    "p",
                    bvar(1),
                    exists_set(
                        "q",
                        and(
                            app2(cst("LeCondition"), bvar(0), bvar(1)),
                            app3(cst("IsGenericForModel"), bvar(3), bvar(2), bvar(0)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// Axiom: semi-properness — preservation of stationarity in [ω₁]^ω.
/// ∀ P, IsSemiProper P → PreservesStationaryOmega1 P
pub fn semi_proper_stationarity_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "P",
        type0(),
        arrow(
            app(cst("IsSemiProper"), bvar(0)),
            app(cst("PreservesStationaryOmega1"), bvar(0)),
        ),
    )
}
/// Axiom: PFA implies BPFA (Bounded Proper Forcing Axiom).
/// PFA → BPFA
pub fn pfa_implies_bpfa_ty() -> Expr {
    arrow(cst("PFA"), cst("BPFA"))
}
/// Axiom: Martin's Maximum is consistent relative to a supercompact cardinal.
/// IsSupercompact κ → Con(ZFC + MM)
pub fn mm_consistency_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "kappa",
        cst("Cardinal"),
        arrow(
            app(cst("IsSupercompact"), bvar(0)),
            app(cst("Con"), cst("ZFC_plus_MM")),
        ),
    )
}
/// Axiom: MM implies 2^ℵ₀ = ℵ₂.
/// MM → TwoPow(Aleph(0)) = Aleph(2)
pub fn mm_implies_aleph2_continuum_ty() -> Expr {
    arrow(
        cst("MartinsMaximum"),
        eq_set(
            app(cst("TwoPow"), app(cst("Aleph"), cst("Zero"))),
            app(cst("Aleph"), cst("Two")),
        ),
    )
}
/// Axiom: MM implies the Stationary Reflection Principle.
/// MM → StationaryReflection
pub fn mm_implies_stationary_reflection_ty() -> Expr {
    arrow(cst("MartinsMaximum"), cst("StationaryReflection"))
}
/// Axiom: Lévy collapse of a supercompact makes ω₁ = (former supercompact).
/// IsSupercompact κ → Col(ω,<κ)-generic makes κ = ω₁
pub fn levy_collapse_supercompact_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "kappa",
        cst("Cardinal"),
        arrow(
            app(cst("IsSupercompact"), bvar(0)),
            app2(cst("AfterLevyCollapse"), bvar(0), cst("Aleph1")),
        ),
    )
}
/// Axiom: forcing indestructibility of supercompact cardinals (Laver preparation).
/// ∃ P, LaverPreparation P κ → ∀ Q proper, κ supercompact in V[G][H]
pub fn laver_indestructibility_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "kappa",
        cst("Cardinal"),
        arrow(
            app(cst("IsSupercompact"), bvar(0)),
            exists_set("P", app2(cst("LaverIndestructible"), bvar(0), bvar(1))),
        ),
    )
}
/// Axiom: Woodin cardinals and generic absoluteness.
/// ∀ n ∈ ω, nWoodin → ProjectiveAbsoluteness n
pub fn woodin_generic_absoluteness_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        arrow(
            app(cst("NWoodinCardinals"), bvar(0)),
            app(cst("ProjectiveAbsoluteness"), bvar(0)),
        ),
    )
}
/// Axiom: omega many Woodin cardinals imply all projective sets are measurable.
/// OmegaWoodin → ∀ A : ProjectiveSet, IsMeasurableSet A
pub fn omega_woodin_projective_measurable_ty() -> Expr {
    arrow(
        cst("OmegaWoodinCardinals"),
        pi(
            BinderInfo::Default,
            "A",
            cst("ProjectiveSet"),
            app(cst("IsMeasurableSet"), bvar(0)),
        ),
    )
}
/// Axiom: class forcing may fail to preserve ZFC (Zarach's theorem).
/// ∃ P : ClassForcing, ¬(PreservesZFC P)
pub fn class_forcing_zfc_failure_ty() -> Expr {
    exists_set(
        "P",
        and(
            app(cst("IsClassForcing"), bvar(0)),
            not(app(cst("PreservesZFC"), bvar(0))),
        ),
    )
}
/// Axiom: pretameness is necessary for class forcing to produce ZF models.
/// ∀ P : ClassForcing, Pretame P → PreservesZF P
pub fn pretame_class_forcing_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "P",
        cst("ClassForcing"),
        arrow(
            app(cst("IsPretame"), bvar(0)),
            app(cst("PreservesZF"), bvar(0)),
        ),
    )
}
/// Axiom: amenable class forcing (Friedman) preserves ZFC.
/// ∀ P : AmenableClassForcing, PreservesZFC P
pub fn amenable_class_forcing_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "P",
        cst("AmenableClassForcing"),
        app(cst("PreservesZFC"), bvar(0)),
    )
}
/// Axiom: every complete Boolean algebra B gives a Boolean-valued model V^B of ZFC.
/// ∀ B : CBA, BooleanValuedModel B ⊨ ZFC
pub fn complete_ba_gives_bvm_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "B",
        cst("CompleteBA"),
        app2(cst("ModelsZFC"), app(cst("BVModel"), bvar(0)), cst("ZFC")),
    )
}
/// Axiom: quotient of V^B by a generic ultrafilter is a classical two-valued model.
/// ∀ B : CBA, ∀ U : GenericUltrafilter B, (V^B / U) ⊨ ZFC classically
pub fn bvm_quotient_ultrafilter_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "B",
        cst("CompleteBA"),
        pi(
            BinderInfo::Default,
            "U",
            app(cst("GenericUltrafilter"), bvar(0)),
            app2(
                cst("ClassicalModel"),
                app2(cst("BVMQuotient"), bvar(1), bvar(0)),
                cst("ZFC"),
            ),
        ),
    )
}
/// Axiom: the regular open algebra RO(P) is the completion of any forcing poset P.
/// ∀ P : ForcingPoset, IsCompletion(RO(P), P)
pub fn regular_open_completion_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "P",
        type0(),
        app2(
            cst("IsCompletion"),
            app(cst("RegularOpenAlgebra"), bvar(0)),
            bvar(0),
        ),
    )
}
/// Axiom: a measurable cardinal gives an ultrapower embedding j: V → M.
/// IsMeasurable κ → ∃ j : V → M, IsElementaryEmbedding j ∧ CritPoint j = κ
pub fn measurable_ultrapower_embedding_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "kappa",
        cst("Cardinal"),
        arrow(
            app(cst("IsMeasurable"), bvar(0)),
            exists_set(
                "j",
                and(
                    app(cst("IsElementaryEmbedding"), bvar(0)),
                    eq_set(app(cst("CritPoint"), bvar(0)), bvar(1)),
                ),
            ),
        ),
    )
}
/// Axiom: forcing can produce elementary embeddings (generic embeddings).
/// ∀ P κ, P collapses κ → ∃ j : V → V[G], GenericEmbedding j κ
pub fn generic_elementary_embedding_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "P",
        type0(),
        pi(
            BinderInfo::Default,
            "kappa",
            cst("Cardinal"),
            arrow(
                app2(cst("CollapsesCardinal"), bvar(1), bvar(0)),
                exists_set("j", app2(cst("GenericEmbedding"), bvar(0), bvar(1))),
            ),
        ),
    )
}
/// Axiom: Łoś theorem for ultrapowers: j(x) = [const_x]_U.
/// ∀ x, ∀ U : Ultrafilter, UltrapowerImage(j_U, x) = ConstantClass(x, U)
pub fn los_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "U",
        cst("Ultrafilter"),
        forall_set(
            "x",
            eq_set(
                app2(
                    cst("UltrapowerImage"),
                    app(cst("UltrapowerMap"), bvar(1)),
                    bvar(0),
                ),
                app2(cst("ConstantClass"), bvar(0), bvar(1)),
            ),
        ),
    )
}
/// Axiom: HOD is an inner model of ZFC definable without parameters.
/// IsInnerModel HOD ∧ DefinableWithoutParameters HOD
pub fn hod_inner_model_ty() -> Expr {
    and(
        app(cst("IsInnerModel"), cst("HOD")),
        app(cst("DefinableWithoutParameters"), cst("HOD")),
    )
}
/// Axiom: every set-generic extension of V has the same HOD.
/// ∀ G : SetGenericFilter, HOD(V[G]) = HOD(V)
pub fn hod_invariant_under_forcing_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        cst("SetGenericFilter"),
        eq_set(app(cst("HOD"), app(cst("GenericExt"), bvar(0))), cst("HOD")),
    )
}
/// Axiom: V = HOD implies GCH (Gödel).
/// VEqualsHOD → GCH
pub fn v_eq_hod_implies_gch_ty() -> Expr {
    arrow(cst("VEqualsHOD"), cst("GCH"))
}
/// Axiom: the Sharp function 0# and its generalizations x# for x ∈ ℝ.
/// ∀ x : Real, ZeroSharpOfX x ↔ LNotClose x
pub fn x_sharp_characterization_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "x",
        cst("Real"),
        iff(
            app(cst("XSharp"), bvar(0)),
            app(cst("LNotCloseAt"), bvar(0)),
        ),
    )
}
/// Axiom: Silver forcing is proper and adds no new reals over a Silver filter.
/// IsSilverForcing P → IsProper P ∧ NoNewReals P
pub fn silver_forcing_proper_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "P",
        type0(),
        arrow(
            app(cst("IsSilverForcing"), bvar(0)),
            and(
                app(cst("IsProper"), bvar(0)),
                app(cst("NoNewReals"), bvar(0)),
            ),
        ),
    )
}
/// Axiom: Sacks forcing adds a minimal degree — the generic real is minimal over M.
/// SacksForcing ⊩ MinimalGenericDegree
pub fn sacks_minimal_degree_ty() -> Expr {
    app(
        cst("Forces"),
        app2(
            cst("SacksForcing_axiom"),
            cst("SacksForcing"),
            cst("MinimalGenericDegree"),
        ),
    )
}
/// Axiom: Sacks forcing is proper (hence preserves ω₁).
/// IsProper SacksForcing
pub fn sacks_proper_ty() -> Expr {
    app(cst("IsProper"), cst("SacksForcing"))
}
/// Axiom: Sacks reals have minimal Turing degree over the ground model.
/// ∀ G : SacksGeneric, ∀ A : G-computable, A ∈ M ∨ G T-reduces-to A
pub fn sacks_minimal_turing_degree_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        cst("SacksGeneric"),
        pi(
            BinderInfo::Default,
            "A",
            cst("Subset_omega"),
            arrow(
                app2(cst("GComputable"), bvar(1), bvar(0)),
                or(
                    mem(bvar(0), cst("GroundModel")),
                    app2(cst("TuringReduces"), bvar(1), bvar(0)),
                ),
            ),
        ),
    )
}
/// Axiom: Mathias forcing is proper and adds a Ramsey ultrafilter.
/// MathiasForcing_U P → IsProper P ∧ AddsRamseyUltrafilter P
pub fn mathias_forcing_proper_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "P",
        type0(),
        arrow(
            app(cst("IsMathiasForcing"), bvar(0)),
            and(
                app(cst("IsProper"), bvar(0)),
                app(cst("AddsRamseyUltrafilter"), bvar(0)),
            ),
        ),
    )
}
/// Axiom: Mathias reals are Ramsey (partition property).
/// ∀ r : MathiasReal, ∀ f : [r]^2 → 2, ∃ H ⊆ r infinite, f is constant on [H]^2
pub fn mathias_ramsey_property_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "r",
        cst("MathiasReal"),
        pi(
            BinderInfo::Default,
            "f",
            arrow(app(cst("PairsOf"), bvar(0)), cst("Bool")),
            exists_set(
                "H",
                and(
                    and(subset(bvar(0), bvar(1)), app(cst("IsInfinite"), bvar(0))),
                    app2(cst("ConstantOn"), bvar(2), app(cst("PairsOf"), bvar(0))),
                ),
            ),
        ),
    )
}
/// Axiom: a Suslin tree exists iff ◇ holds (Diamond principle).
/// (∃ Suslin tree) ↔ Diamond
pub fn suslin_tree_diamond_ty() -> Expr {
    iff(
        exists_set("T", app(cst("IsSuslinTree"), bvar(0))),
        cst("Diamond"),
    )
}
/// Axiom: MA_ω₁ implies there are no Suslin trees (MA refutes ◇).
/// MA_omega1 → ¬(∃ Suslin tree)
pub fn ma_no_suslin_tree_ty() -> Expr {
    arrow(
        cst("MA_omega1"),
        not(exists_set("T", app(cst("IsSuslinTree"), bvar(0)))),
    )
}
/// Axiom: every Aronszajn tree is special under PFA.
/// PFA → ∀ T : AronszajnTree, IsSpecial T
pub fn pfa_aronszajn_special_ty() -> Expr {
    arrow(
        cst("PFA"),
        pi(
            BinderInfo::Default,
            "T",
            cst("AronszajnTree"),
            app(cst("IsSpecial"), bvar(0)),
        ),
    )
}
/// Axiom: Suslin's hypothesis — no Suslin tree exists — is consistent with ZFC.
/// Con(ZFC + SH)
pub fn suslin_hypothesis_consistent_ty() -> Expr {
    app(cst("Con"), cst("ZFC_plus_SH"))
}
/// Axiom: check names for ground model sets: x̌[G] = x for all generic G.
/// ∀ x ∈ M, ∀ G : GenericFilter, Interpretation(CheckName x, G) = x
pub fn check_name_interpretation_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "x",
        set_ty(),
        pi(
            BinderInfo::Default,
            "G",
            cst("GenericFilter"),
            eq_set(
                app2(cst("Interpret"), app(cst("CheckName"), bvar(1)), bvar(0)),
                bvar(1),
            ),
        ),
    )
}
/// Axiom: the generic filter name Ġ satisfies Ġ[G] = G.
/// ∀ G : GenericFilter, Interpretation(GenericName, G) = G
pub fn generic_name_interpretation_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        cst("GenericFilter"),
        eq_set(
            app2(cst("Interpret"), cst("GenericNameDot"), bvar(0)),
            bvar(0),
        ),
    )
}
/// Axiom: every element of M[G] is the interpretation of some P-name in M.
/// ∀ x ∈ M[G], ∃ τ ∈ M, τ is a P-name ∧ τ[G] = x
pub fn every_element_has_pname_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "x",
        cst("GenericExtSet"),
        exists_set(
            "tau",
            and(
                app(cst("IsPName"), bvar(0)),
                eq_set(app2(cst("Interpret"), bvar(0), cst("GenericG")), bvar(1)),
            ),
        ),
    )
}
/// Register the advanced forcing axioms (§13) into an existing environment.
///
/// Call this after [`build_forcing_theory_env`] to add the extended axiom set.
pub fn register_advanced_forcing_axioms(env: &mut Environment) {
    let new_base_types: &[(&str, Expr)] = &[
        ("FinSupportIteration", arrow(cst("Ordinal"), type0())),
        ("CSIteration", arrow(cst("Ordinal"), type0())),
        ("EastonFunction", type0()),
        ("CardinalPattern", arrow(set_ty(), arrow(set_ty(), prop()))),
        ("AllStepsProper", arrow(type0(), prop())),
        ("CountableModel", type0()),
        ("LeCondition", arrow(set_ty(), arrow(set_ty(), prop()))),
        (
            "IsGenericForModel",
            arrow(
                type0(),
                arrow(cst("CountableModel"), arrow(set_ty(), prop())),
            ),
        ),
        ("IsSemiProper", arrow(type0(), prop())),
        ("PreservesStationaryOmega1", arrow(type0(), prop())),
        ("PFA", prop()),
        ("BPFA", prop()),
        ("MartinsMaximum", prop()),
        ("ZFC_plus_MM", prop()),
        ("Con", arrow(prop(), prop())),
        ("StationaryReflection", prop()),
        (
            "CardinalCollapse_pred",
            arrow(type0(), arrow(cst("Cardinal"), prop())),
        ),
        (
            "AfterLevyCollapse",
            arrow(cst("Cardinal"), arrow(cst("Cardinal"), prop())),
        ),
        ("Aleph1", cst("Cardinal")),
        (
            "LaverIndestructible",
            arrow(cst("Cardinal"), arrow(set_ty(), prop())),
        ),
        ("NWoodinCardinals", arrow(nat_ty(), prop())),
        ("ProjectiveAbsoluteness", arrow(nat_ty(), prop())),
        ("OmegaWoodinCardinals", prop()),
        ("ProjectiveSet", type0()),
        ("IsMeasurableSet", arrow(cst("ProjectiveSet"), prop())),
        ("IsClassForcing", arrow(set_ty(), prop())),
        ("ClassForcing", type0()),
        ("PreservesZFC", arrow(set_ty(), prop())),
        ("PreservesZF", arrow(set_ty(), prop())),
        ("IsPretame", arrow(set_ty(), prop())),
        ("AmenableClassForcing", type0()),
        ("CompleteBA", type0()),
        ("BVModel", arrow(cst("CompleteBA"), set_ty())),
        ("ModelsZFC", arrow(set_ty(), arrow(prop(), prop()))),
        ("ZFC", prop()),
        ("GenericUltrafilter", arrow(cst("CompleteBA"), type0())),
        ("ClassicalModel", arrow(set_ty(), arrow(prop(), prop()))),
        (
            "BVMQuotient",
            arrow(
                cst("CompleteBA"),
                arrow(cst("GenericUltrafilter"), set_ty()),
            ),
        ),
        ("RegularOpenAlgebra", arrow(type0(), cst("CompleteBA"))),
        (
            "IsCompletion",
            arrow(cst("CompleteBA"), arrow(type0(), prop())),
        ),
        ("IsElementaryEmbedding", arrow(set_ty(), prop())),
        ("CritPoint", arrow(set_ty(), cst("Cardinal"))),
        (
            "CollapsesCardinal",
            arrow(type0(), arrow(cst("Cardinal"), prop())),
        ),
        (
            "GenericEmbedding",
            arrow(set_ty(), arrow(cst("Cardinal"), prop())),
        ),
        ("Ultrafilter", type0()),
        (
            "UltrapowerImage",
            arrow(set_ty(), arrow(set_ty(), set_ty())),
        ),
        ("UltrapowerMap", arrow(cst("Ultrafilter"), set_ty())),
        (
            "ConstantClass",
            arrow(set_ty(), arrow(cst("Ultrafilter"), set_ty())),
        ),
        ("HOD", set_ty()),
        ("IsInnerModel", arrow(set_ty(), prop())),
        ("DefinableWithoutParameters", arrow(set_ty(), prop())),
        ("SetGenericFilter", type0()),
        ("GenericExt", arrow(cst("SetGenericFilter"), set_ty())),
        ("VEqualsHOD", prop()),
        ("GCH", prop()),
        ("Real", type0()),
        ("XSharp", arrow(cst("Real"), prop())),
        ("LNotCloseAt", arrow(cst("Real"), prop())),
        ("IsSilverForcing", arrow(type0(), prop())),
        ("NoNewReals", arrow(type0(), prop())),
        ("Forces", arrow(prop(), prop())),
        ("SacksForcing_axiom", arrow(type0(), arrow(prop(), prop()))),
        ("SacksForcing", type0()),
        ("MinimalGenericDegree", prop()),
        ("SacksGeneric", type0()),
        ("Subset_omega", type0()),
        (
            "GComputable",
            arrow(cst("SacksGeneric"), arrow(cst("Subset_omega"), prop())),
        ),
        (
            "TuringReduces",
            arrow(cst("SacksGeneric"), arrow(cst("Subset_omega"), prop())),
        ),
        ("IsMathiasForcing", arrow(type0(), prop())),
        ("AddsRamseyUltrafilter", arrow(type0(), prop())),
        ("MathiasReal", type0()),
        ("PairsOf", arrow(cst("MathiasReal"), type0())),
        ("IsInfinite", arrow(set_ty(), prop())),
        (
            "ConstantOn",
            arrow(arrow(type0(), cst("Bool")), arrow(type0(), prop())),
        ),
        ("IsSuslinTree", arrow(set_ty(), prop())),
        ("Diamond", prop()),
        ("MA_omega1", prop()),
        ("AronszajnTree", type0()),
        ("IsSpecial", arrow(cst("AronszajnTree"), prop())),
        ("ZFC_plus_SH", prop()),
        ("CheckName", arrow(set_ty(), set_ty())),
        (
            "Interpret",
            arrow(set_ty(), arrow(cst("GenericFilter"), set_ty())),
        ),
        ("GenericNameDot", set_ty()),
        ("GenericExtSet", type0()),
        ("IsPName", arrow(set_ty(), prop())),
        ("GenericG", cst("GenericFilter")),
    ];
    for (name, ty) in new_base_types {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    let new_axioms: &[(&str, fn() -> Expr)] = &[
        ("fin_support_iteration_type", fin_support_iteration_ty),
        (
            "countable_support_iteration_type",
            countable_support_iteration_ty,
        ),
        ("fin_support_ccc", fin_support_ccc_ty),
        ("countable_support_proper", countable_support_proper_ty),
        ("eastons_theorem", eastons_theorem_ty),
        ("properness_criterion", properness_criterion_ty),
        ("semi_proper_stationarity", semi_proper_stationarity_ty),
        ("pfa_implies_bpfa", pfa_implies_bpfa_ty),
        ("mm_consistency", mm_consistency_ty),
        (
            "mm_implies_aleph2_continuum",
            mm_implies_aleph2_continuum_ty,
        ),
        (
            "mm_implies_stationary_reflection",
            mm_implies_stationary_reflection_ty,
        ),
        ("levy_collapse_supercompact", levy_collapse_supercompact_ty),
        ("laver_indestructibility", laver_indestructibility_ty),
        (
            "woodin_generic_absoluteness",
            woodin_generic_absoluteness_ty,
        ),
        (
            "omega_woodin_projective_measurable",
            omega_woodin_projective_measurable_ty,
        ),
        ("class_forcing_zfc_failure", class_forcing_zfc_failure_ty),
        ("pretame_class_forcing", pretame_class_forcing_ty),
        ("amenable_class_forcing", amenable_class_forcing_ty),
        ("complete_ba_gives_bvm", complete_ba_gives_bvm_ty),
        ("bvm_quotient_ultrafilter", bvm_quotient_ultrafilter_ty),
        ("regular_open_completion", regular_open_completion_ty),
        (
            "measurable_ultrapower_embedding",
            measurable_ultrapower_embedding_ty,
        ),
        (
            "generic_elementary_embedding",
            generic_elementary_embedding_ty,
        ),
        ("los_theorem", los_theorem_ty),
        ("hod_inner_model", hod_inner_model_ty),
        (
            "hod_invariant_under_forcing",
            hod_invariant_under_forcing_ty,
        ),
        ("v_eq_hod_implies_gch", v_eq_hod_implies_gch_ty),
        ("x_sharp_characterization", x_sharp_characterization_ty),
        ("silver_forcing_proper", silver_forcing_proper_ty),
        ("sacks_minimal_degree", sacks_minimal_degree_ty),
        ("sacks_proper", sacks_proper_ty),
        (
            "sacks_minimal_turing_degree",
            sacks_minimal_turing_degree_ty,
        ),
        ("mathias_forcing_proper", mathias_forcing_proper_ty),
        ("mathias_ramsey_property", mathias_ramsey_property_ty),
        ("suslin_tree_diamond", suslin_tree_diamond_ty),
        ("ma_no_suslin_tree", ma_no_suslin_tree_ty),
        ("pfa_aronszajn_special", pfa_aronszajn_special_ty),
        (
            "suslin_hypothesis_consistent",
            suslin_hypothesis_consistent_ty,
        ),
        ("check_name_interpretation", check_name_interpretation_ty),
        (
            "generic_name_interpretation",
            generic_name_interpretation_ty,
        ),
        ("every_element_has_pname", every_element_has_pname_ty),
    ];
    for (name, mk_ty) in new_axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: mk_ty(),
        })
        .ok();
    }
}
/// Build an extended forcing theory environment including all advanced axioms.
pub fn build_extended_forcing_env() -> Environment {
    let mut env = build_forcing_theory_env();
    register_advanced_forcing_axioms(&mut env);
    env
}
#[cfg(test)]
mod tests_advanced {
    use super::*;
    #[test]
    fn test_finite_support_iteration() {
        let iter =
            FiniteSupportIteration::new("omega_2", vec!["Cohen".to_string(), "Random".to_string()]);
        assert!(iter.is_ccc());
        assert!(iter.initial_segment_ccc(1));
        assert!(iter.initial_segment_ccc(2));
        assert!(!iter.initial_segment_ccc(3));
        assert!(iter.forcing_relation_definable());
        let desc = iter.realizes_easton("F");
        assert!(desc.contains("Easton"));
    }
    #[test]
    fn test_sacks_forcing_poset() {
        let sacks =
            SacksForcingPoset::new(vec!["01".to_string(), "0110".to_string(), "10".to_string()]);
        assert!(sacks.is_proper());
        assert!(sacks.preserves_omega1());
        assert!(sacks.countable_support_iterable());
        assert!(sacks.compatible(0, 1));
        assert!(!sacks.compatible(0, 2));
    }
    #[test]
    fn test_mathias_forcing_poset() {
        let plain = MathiasForcingPoset::new();
        assert!(!plain.is_proper());
        assert!(!plain.ramsey_ultrafilter);
        let ramsey = MathiasForcingPoset::with_ramsey_ultrafilter("U_Ramsey");
        assert!(ramsey.is_proper());
        assert!(ramsey.preserves_omega1());
        assert!(ramsey.adds_pseudointersection());
        assert!(ramsey.no_new_omega1_sequences());
    }
    #[test]
    fn test_generic_ultrapower() {
        let up = GenericUltrapower::new("kappa");
        assert!(up.is_wellfounded());
        assert!(up.embedding_is_elementary());
        assert_eq!(up.critical_point(), "kappa");
        assert!(up.j_kappa_above_kappa());
        let img = up.embedding_moves_kappa();
        assert!(img.contains("kappa"));
        assert!(img.contains('>'));
        let los = up.los_theorem();
        assert!(los.contains("Ult"));
    }
    #[test]
    fn test_cba_forcing_poset() {
        let cohen = CBAForcingPoset::cohen();
        assert!(cohen.atomless);
        assert!(cohen.is_separative());
        let measure = CBAForcingPoset::measure_algebra();
        assert!(measure.atomless);
        assert!(!cohen.forcing_equivalent(&measure));
        let bv = cohen.boolean_value("CH");
        assert!(bv.contains("CH"));
        let sup_bv = cohen.sup_boolean_values("phi(x)", "x");
        assert!(sup_bv.contains("phi(x)"));
    }
    #[test]
    fn test_extended_forcing_env() {
        let env = build_extended_forcing_env();
        assert!(env.get(&Name::str("forcing_theorem")).is_some());
        assert!(env.get(&Name::str("martins_axiom")).is_some());
        assert!(env.get(&Name::str("fin_support_ccc")).is_some());
        assert!(env.get(&Name::str("sacks_proper")).is_some());
        assert!(env.get(&Name::str("mathias_forcing_proper")).is_some());
        assert!(env.get(&Name::str("suslin_tree_diamond")).is_some());
        assert!(env.get(&Name::str("los_theorem")).is_some());
        assert!(env.get(&Name::str("laver_indestructibility")).is_some());
        assert!(env.get(&Name::str("hod_inner_model")).is_some());
        assert!(env.get(&Name::str("check_name_interpretation")).is_some());
        assert!(env.get(&Name::str("every_element_has_pname")).is_some());
        assert!(env.get(&Name::str("mm_implies_aleph2_continuum")).is_some());
        assert!(env.get(&Name::str("pfa_aronszajn_special")).is_some());
    }
}
