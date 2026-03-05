//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    CoherentLocale, CompactRegularLocale, Dcpo, FormalBall, FormalTopology, Frame, FrameHom,
    FrameNucleus, IsbellAdjunction, Locale, LocaleFrame, LocalicGroup, LocalicReals,
    LocalicValuation, Nucleus, PointlessMap, Quantale, ScottTopologyFrame, SoberSpace,
    SoberTopSpace, Spectrum, StoneCechCompact, StoneCechCompactification, StoneDuality, Sublocale,
    SublocaleType, UniformLocale, UniformityType, ValuationType,
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
pub fn forall_ty(name: &str, body: Expr) -> Expr {
    pi(BinderInfo::Default, name, type0(), body)
}
pub fn forall_prop(name: &str, body: Expr) -> Expr {
    pi(BinderInfo::Default, name, prop(), body)
}
pub fn exists_prop(name: &str, body: Expr) -> Expr {
    app(cst("Exists"), lam(BinderInfo::Default, name, prop(), body))
}
pub fn eq_prop(p: Expr, q: Expr) -> Expr {
    app3(cst("Eq"), prop(), p, q)
}
pub fn eq_ty(t: Expr, x: Expr, y: Expr) -> Expr {
    app3(cst("Eq"), t, x, y)
}
/// Type: Frame — a type equipped with complete lattice structure and infinite distributivity.
pub fn frame_type_ty() -> Expr {
    type0()
}
/// Type: IsFrame(L) — predicate that L carries a frame structure.
/// IsFrame : Type → Prop
pub fn is_frame_ty() -> Expr {
    arrow(type0(), prop())
}
/// Type: FrameHom(L, M) — type of frame homomorphisms from L to M.
/// FrameHom : Type → Type → Type
pub fn frame_hom_ty() -> Expr {
    forall_ty("L", forall_ty("M", type0()))
}
/// Type: Nucleus(L) — type of nuclei on a frame L.
/// Nucleus : Type → Type
pub fn nucleus_type_ty() -> Expr {
    forall_ty("L", type0())
}
/// Axiom: Frame infinite distributive law.
/// ∀ (L : Type), IsFrame L → ∀ (a : L) (S : L → Prop),
///   meet a (join S) = join (fun s => meet a s)
pub fn frame_distributive_law_ty() -> Expr {
    forall_ty(
        "L",
        arrow(
            app(cst("IsFrame"), bvar(0)),
            pi(
                BinderInfo::Default,
                "a",
                bvar(1),
                pi(
                    BinderInfo::Default,
                    "S",
                    arrow(bvar(2), prop()),
                    eq_ty(
                        bvar(3),
                        app2(cst("FrameMeet"), bvar(1), app(cst("FrameJoin"), bvar(0))),
                        app(cst("FrameJoin"), app(cst("FrameMeet"), bvar(1))),
                    ),
                ),
            ),
        ),
    )
}
/// Axiom: Nucleus conditions — j is inflationary, idempotent, and preserves meets.
/// ∀ (L : Type), IsFrame L → ∀ (j : L → L), IsNucleus L j ↔
///   (∀ a, a ≤ j a) ∧ (∀ a, j (j a) = j a) ∧ (∀ a b, j (meet a b) = meet (j a) (j b))
pub fn nucleus_axiom_ty() -> Expr {
    forall_ty(
        "L",
        arrow(
            app(cst("IsFrame"), bvar(0)),
            pi(
                BinderInfo::Default,
                "j",
                arrow(bvar(1), bvar(2)),
                iff(
                    app3(cst("IsNucleus"), bvar(2), bvar(0), bvar(1)),
                    and(
                        and(
                            pi(
                                BinderInfo::Default,
                                "a",
                                bvar(3),
                                app2(cst("FrameLeq"), bvar(0), app(bvar(2), bvar(0))),
                            ),
                            pi(
                                BinderInfo::Default,
                                "a",
                                bvar(3),
                                eq_ty(
                                    bvar(4),
                                    app(bvar(2), app(bvar(2), bvar(0))),
                                    app(bvar(2), bvar(0)),
                                ),
                            ),
                        ),
                        pi(
                            BinderInfo::Default,
                            "a",
                            bvar(3),
                            pi(
                                BinderInfo::Default,
                                "b",
                                bvar(4),
                                eq_ty(
                                    bvar(5),
                                    app(bvar(3), app2(cst("FrameMeet"), bvar(1), bvar(0))),
                                    app2(
                                        cst("FrameMeet"),
                                        app(bvar(3), bvar(1)),
                                        app(bvar(3), bvar(0)),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// Axiom: Stone duality — the functor Ω: SoberSpace → SpatialFrame is an equivalence.
/// Ω : SoberSpace ≃ SpatialFrame^op
pub fn stone_duality_ty() -> Expr {
    app2(
        cst("CategoryEquivalence"),
        cst("SoberSpace"),
        cst("SpatialFrameOp"),
    )
}
/// Axiom: Isbell adjunction — Ω ⊣ pt.
/// IsbellAdj : Adjunction Ω pt
pub fn isbell_adjunction_ty() -> Expr {
    app2(cst("Adjunction"), cst("OmegaFunctor"), cst("PtFunctor"))
}
/// Axiom: Sheaves on a locale form a Grothendieck topos.
/// ∀ (L : Locale), IsGrothendieckTopos (Sheaves L)
pub fn sheaves_locale_topos_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "L",
        cst("Locale"),
        app(cst("IsGrothendieckTopos"), app(cst("Sheaves"), bvar(0))),
    )
}
/// Axiom: Localic groups have Haar measure.
/// ∀ (G : LocalicGroup), IsLocallyCompact G → ∃! μ : HaarMeasure G, IsHaar μ
pub fn haar_measure_existence_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        cst("LocalicGroup"),
        arrow(
            app(cst("IsLocallyCompact"), bvar(0)),
            app(
                cst("ExistsUnique"),
                lam(
                    BinderInfo::Default,
                    "mu",
                    app(cst("HaarMeasure"), bvar(1)),
                    app(cst("IsHaar"), bvar(0)),
                ),
            ),
        ),
    )
}
/// Axiom: LT topologies on Sh(L) correspond to nuclei on L.
/// ∀ (L : Locale),
///   LTTopologies (Sheaves L) ≃ Nuclei L
pub fn lt_topologies_nuclei_correspondence_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "L",
        cst("Locale"),
        app2(
            cst("TypeEquiv"),
            app(cst("LTTopologies"), app(cst("Sheaves"), bvar(0))),
            app(cst("Nuclei"), bvar(0)),
        ),
    )
}
/// Type: Quantale — a complete lattice with associative multiplication distributing over joins.
/// Quantale : Type
pub fn quantale_type_ty() -> Expr {
    type0()
}
/// Axiom: Quantale distributivity law.
/// ∀ (Q : Type), IsQuantale Q →
///   ∀ (a : Q) (S : Q → Prop), a * (join S) = join (fun s => a * s)
pub fn quantale_distributivity_ty() -> Expr {
    forall_ty(
        "Q",
        arrow(
            app(cst("IsQuantale"), bvar(0)),
            pi(
                BinderInfo::Default,
                "a",
                bvar(1),
                pi(
                    BinderInfo::Default,
                    "S",
                    arrow(bvar(2), prop()),
                    eq_ty(
                        bvar(3),
                        app2(cst("QMul"), bvar(1), app(cst("FrameJoin"), bvar(0))),
                        app(cst("FrameJoin"), app(cst("QMulLeft"), bvar(1))),
                    ),
                ),
            ),
        ),
    )
}
/// Axiom: Every frame is a commutative unital quantale.
/// ∀ (L : Type), IsFrame L → IsQuantale L
pub fn frame_is_quantale_ty() -> Expr {
    forall_ty(
        "L",
        arrow(
            app(cst("IsFrame"), bvar(0)),
            app(cst("IsQuantale"), bvar(0)),
        ),
    )
}
/// Axiom: Formal topology covering axiom (inductive generation).
/// ∀ (S : FormalTopology) (a : Base S) (U : Cover S a), FormalCover S a U
pub fn formal_topology_covering_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "S",
        cst("FormalTopology"),
        pi(
            BinderInfo::Default,
            "a",
            app(cst("Base"), bvar(0)),
            pi(
                BinderInfo::Default,
                "U",
                app2(cst("Cover"), bvar(1), bvar(0)),
                app3(cst("FormalCover"), bvar(2), bvar(1), bvar(0)),
            ),
        ),
    )
}
/// Axiom: The Scott topology on a continuous domain is sober.
/// ∀ (D : ContinuousDomain), IsSober (ScottTopology D)
pub fn scott_topology_sober_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "D",
        cst("ContinuousDomain"),
        app(cst("IsSober"), app(cst("ScottTopology"), bvar(0))),
    )
}
/// Axiom: Way-below relation interpolation (approximation axiom).
/// ∀ (D : ContinuousDomain) (x : D), x = SupDir {y | y ≪ x}
pub fn way_below_interpolation_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "D",
        cst("ContinuousDomain"),
        pi(
            BinderInfo::Default,
            "x",
            bvar(0),
            eq_ty(
                bvar(1),
                bvar(0),
                app(
                    cst("SupDir"),
                    lam(
                        BinderInfo::Default,
                        "y",
                        bvar(2),
                        app2(cst("WayBelow"), bvar(0), bvar(1)),
                    ),
                ),
            ),
        ),
    )
}
/// Axiom: Valuations on a locale form a locale (the valuation locale).
/// ∀ (L : Locale), IsLocale (ValuationLocale L)
pub fn valuation_locale_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "L",
        cst("Locale"),
        app(cst("IsLocale"), app(cst("ValuationLocale"), bvar(0))),
    )
}
/// Axiom: Riesz representation for locales.
/// ∀ (L : CompactRegularLocale),
///   Valuations L ≃ BoundedLinearFunctionals (ContinuousFunctions L)
pub fn localic_riesz_representation_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "L",
        cst("CompactRegularLocale"),
        app2(
            cst("TypeEquiv"),
            app(cst("Valuations"), bvar(0)),
            app(
                cst("BoundedLinearFunctionals"),
                app(cst("ContinuousFunctions"), bvar(0)),
            ),
        ),
    )
}
/// Axiom: Uniform locale completion.
/// ∀ (U : UniformLocale), ∃ (C : CompleteUniformLocale),
///   UniformEmbed U C ∧ IsDense (ImageEmbed U C) C
pub fn uniform_locale_completion_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "U",
        cst("UniformLocale"),
        app(
            cst("Exists"),
            lam(
                BinderInfo::Default,
                "C",
                cst("CompleteUniformLocale"),
                and(
                    app2(cst("UniformEmbed"), bvar(1), bvar(0)),
                    app2(
                        cst("IsDense"),
                        app2(cst("ImageEmbed"), bvar(1), bvar(0)),
                        bvar(0),
                    ),
                ),
            ),
        ),
    )
}
/// Axiom: Sublocales form a co-frame (dual frame).
/// ∀ (L : Locale), IsCoFrame (SublocaleLatice L)
pub fn sublocales_coframe_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "L",
        cst("Locale"),
        app(cst("IsCoFrame"), app(cst("SublocaleLatice"), bvar(0))),
    )
}
/// Axiom: Every open sublocale and its complement are supplementary.
/// ∀ (L : Locale) (a : L), OpenSublocale a ∪ ClosedSublocale a = L
pub fn open_closed_supplementary_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "L",
        cst("Locale"),
        pi(
            BinderInfo::Default,
            "a",
            bvar(0),
            eq_ty(
                app(cst("SublocaleLatice"), bvar(1)),
                app2(
                    cst("SublocaleJoin"),
                    app(cst("OpenSublocale"), bvar(0)),
                    app(cst("ClosedSublocale"), bvar(0)),
                ),
                bvar(1),
            ),
        ),
    )
}
/// Axiom: Soberification — the Isbell adjunction unit at X is an iso iff X is sober.
/// ∀ (X : TopSpace), IsSober X ↔ IsIso (IsbellUnit X)
pub fn soberification_iso_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("TopSpace"),
        iff(
            app(cst("IsSober"), bvar(0)),
            app(cst("IsIso"), app(cst("IsbellUnit"), bvar(0))),
        ),
    )
}
/// Axiom: Compact regular locales are normal.
/// ∀ (L : CompactRegularLocale), IsNormal L
pub fn compact_regular_normal_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "L",
        cst("CompactRegularLocale"),
        app(cst("IsNormal"), bvar(0)),
    )
}
/// Axiom: Compact regular locales are paracompact.
/// ∀ (L : CompactRegularLocale), IsParacompact L
pub fn compact_regular_paracompact_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "L",
        cst("CompactRegularLocale"),
        app(cst("IsParacompact"), bvar(0)),
    )
}
/// Axiom: Localic Dedekind reals are Dedekind complete.
/// DedekindComplete ℝ_loc
pub fn localic_reals_dedekind_complete_ty() -> Expr {
    app(cst("DedekindComplete"), cst("LocalicReals"))
}
/// Axiom: The localic reals satisfy the Heine-Borel property.
/// ∀ (a b : ℝ_loc), IsCompact (ClosedInterval a b)
pub fn localic_heine_borel_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "a",
        cst("LocalicReals"),
        pi(
            BinderInfo::Default,
            "b",
            cst("LocalicReals"),
            app(
                cst("IsCompact"),
                app2(cst("ClosedInterval"), bvar(1), bvar(0)),
            ),
        ),
    )
}
/// Axiom: Priestley duality — bounded distributive lattices ↔ Priestley spaces.
/// BoundedDistribLat ≃ PriestleySpace^op
pub fn priestley_duality_ty() -> Expr {
    app2(
        cst("CategoryEquivalence"),
        cst("BoundedDistribLat"),
        cst("PriestleySpaceOp"),
    )
}
/// Axiom: Stone representation — Boolean algebras ↔ Stone spaces.
/// BoolAlg ≃ StoneSpace^op
pub fn stone_representation_ty() -> Expr {
    app2(
        cst("CategoryEquivalence"),
        cst("BoolAlg"),
        cst("StoneSpaceOp"),
    )
}
/// Axiom: Coherent locales classify coherent theories.
/// ∀ (T : CoherentTheory), ∃ (L : CoherentLocale), ClassifiesTheory L T
pub fn coherent_locale_classification_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "T",
        cst("CoherentTheory"),
        app(
            cst("Exists"),
            lam(
                BinderInfo::Default,
                "L",
                cst("CoherentLocale"),
                app2(cst("ClassifiesTheory"), bvar(0), bvar(1)),
            ),
        ),
    )
}
/// Axiom: Stone-Čech universal property.
/// ∀ (X : CRegSpace) (K : KHaus) (f : Continuous X K),
///   ∃! (βf : Continuous (βX) K), βf ∘ ι = f
pub fn stone_cech_universal_property_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("CRegSpace"),
        pi(
            BinderInfo::Default,
            "K",
            cst("KHaus"),
            pi(
                BinderInfo::Default,
                "f",
                app2(cst("Continuous"), bvar(2), bvar(1)),
                app(
                    cst("ExistsUnique"),
                    lam(
                        BinderInfo::Default,
                        "bf",
                        app2(cst("Continuous"), app(cst("StoneCech"), bvar(3)), bvar(2)),
                        eq_ty(
                            app2(cst("Continuous"), app(cst("StoneCech"), bvar(4)), bvar(3)),
                            app2(cst("Compose"), bvar(0), app(cst("StoneCechEmbed"), bvar(4))),
                            bvar(1),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// Build the point-free topology (locale theory) environment.
pub fn build_point_free_topology_env() -> Environment {
    let mut env = Environment::new();
    let base_types: &[(&str, Expr)] = &[
        ("Prop", type1()),
        ("And", arrow(prop(), arrow(prop(), prop()))),
        ("Or", arrow(prop(), arrow(prop(), prop()))),
        ("Not", arrow(prop(), prop())),
        ("Iff", arrow(prop(), arrow(prop(), prop()))),
        (
            "Eq",
            pi(
                BinderInfo::Default,
                "T",
                type0(),
                arrow(bvar(0), arrow(bvar(1), prop())),
            ),
        ),
        ("Exists", arrow(arrow(prop(), prop()), prop())),
        ("ExistsUnique", arrow(arrow(prop(), prop()), prop())),
        ("Frame", type0()),
        ("Locale", type0()),
        ("IsFrame", arrow(type0(), prop())),
        ("IsSpatialFrame", arrow(type0(), prop())),
        ("IsCoherentFrame", arrow(type0(), prop())),
        ("IsRegularFrame", arrow(type0(), prop())),
        ("IsCompactFrame", arrow(type0(), prop())),
        ("Nucleus", arrow(type0(), type0())),
        (
            "IsNucleus",
            arrow(
                type0(),
                arrow(arrow(type0(), type0()), arrow(type0(), prop())),
            ),
        ),
        ("FrameMeet", arrow(type0(), arrow(type0(), type0()))),
        ("FrameJoin", arrow(arrow(type0(), prop()), type0())),
        ("FrameTop", type0()),
        ("FrameBot", type0()),
        ("FrameLeq", arrow(type0(), arrow(type0(), prop()))),
        ("FrameHeyting", arrow(type0(), arrow(type0(), type0()))),
        ("SoberSpace", type0()),
        ("SpatialFrameOp", type0()),
        ("KHaus", type0()),
        ("CRegSpace", type0()),
        (
            "CategoryEquivalence",
            arrow(type0(), arrow(type0(), prop())),
        ),
        ("TypeEquiv", arrow(type0(), arrow(type0(), prop()))),
        ("Adjunction", arrow(type0(), arrow(type0(), prop()))),
        ("OmegaFunctor", type0()),
        ("PtFunctor", type0()),
        ("Sheaves", arrow(cst("Locale"), type0())),
        ("IsGrothendieckTopos", arrow(type0(), prop())),
        ("LTTopologies", arrow(type0(), type0())),
        ("Nuclei", arrow(cst("Locale"), type0())),
        ("CoherentLocale", type0()),
        ("CoherentTheory", type0()),
        (
            "ClassifiesTheory",
            arrow(cst("CoherentLocale"), arrow(cst("CoherentTheory"), prop())),
        ),
        ("LocalicGroup", type0()),
        ("IsLocallyCompact", arrow(type0(), prop())),
        ("HaarMeasure", arrow(cst("LocalicGroup"), type0())),
        ("IsHaar", arrow(type0(), prop())),
        ("StoneCech", arrow(cst("CRegSpace"), cst("KHaus"))),
        ("StoneCechEmbed", arrow(cst("CRegSpace"), type0())),
        ("Continuous", arrow(type0(), arrow(type0(), type0()))),
        ("Compose", arrow(type0(), arrow(type0(), type0()))),
        ("CompleteBA", type0()),
        ("IsCompleteBA", arrow(type0(), prop())),
        ("IsAtomless", arrow(type0(), prop())),
        ("Quantale", type0()),
        ("IsQuantale", arrow(type0(), prop())),
        ("QMul", arrow(type0(), arrow(type0(), type0()))),
        ("QMulLeft", arrow(type0(), arrow(type0(), type0()))),
        ("FormalTopology", type0()),
        ("Base", arrow(cst("FormalTopology"), type0())),
        (
            "Cover",
            arrow(
                cst("FormalTopology"),
                arrow(type0(), arrow(type0(), prop())),
            ),
        ),
        (
            "FormalCover",
            arrow(
                cst("FormalTopology"),
                arrow(type0(), arrow(type0(), prop())),
            ),
        ),
        ("ContinuousDomain", type0()),
        ("ScottTopology", arrow(cst("ContinuousDomain"), type0())),
        ("IsSober", arrow(type0(), prop())),
        ("SupDir", arrow(arrow(type0(), prop()), type0())),
        ("WayBelow", arrow(type0(), arrow(type0(), prop()))),
        ("ValuationLocale", arrow(cst("Locale"), cst("Locale"))),
        ("IsLocale", arrow(type0(), prop())),
        ("Valuations", arrow(cst("Locale"), type0())),
        ("BoundedLinearFunctionals", arrow(type0(), type0())),
        ("ContinuousFunctions", arrow(type0(), type0())),
        ("UniformLocale", type0()),
        ("CompleteUniformLocale", type0()),
        (
            "UniformEmbed",
            arrow(
                cst("UniformLocale"),
                arrow(cst("CompleteUniformLocale"), prop()),
            ),
        ),
        ("IsDense", arrow(type0(), arrow(type0(), prop()))),
        (
            "ImageEmbed",
            arrow(
                cst("UniformLocale"),
                arrow(cst("CompleteUniformLocale"), type0()),
            ),
        ),
        ("SublocaleLatice", arrow(cst("Locale"), type0())),
        ("IsCoFrame", arrow(type0(), prop())),
        ("SublocaleJoin", arrow(type0(), arrow(type0(), type0()))),
        ("OpenSublocale", arrow(type0(), type0())),
        ("ClosedSublocale", arrow(type0(), type0())),
        ("TopSpace", type0()),
        ("IsIso", arrow(type0(), prop())),
        ("IsbellUnit", arrow(cst("TopSpace"), type0())),
        ("CompactRegularLocale", type0()),
        ("IsNormal", arrow(type0(), prop())),
        ("IsParacompact", arrow(type0(), prop())),
        ("LocalicReals", type0()),
        ("DedekindComplete", arrow(type0(), prop())),
        (
            "ClosedInterval",
            arrow(cst("LocalicReals"), arrow(cst("LocalicReals"), type0())),
        ),
        ("IsCompact", arrow(type0(), prop())),
        ("BoundedDistribLat", type0()),
        ("PriestleySpaceOp", type0()),
        ("BoolAlg", type0()),
        ("StoneSpaceOp", type0()),
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
        ("frame_distributive_law", frame_distributive_law_ty),
        ("nucleus_axiom", nucleus_axiom_ty),
        ("stone_duality", stone_duality_ty),
        ("isbell_adjunction", isbell_adjunction_ty),
        ("sheaves_locale_topos", sheaves_locale_topos_ty),
        ("haar_measure_existence", haar_measure_existence_ty),
        (
            "lt_topologies_nuclei",
            lt_topologies_nuclei_correspondence_ty,
        ),
        (
            "coherent_locale_classification",
            coherent_locale_classification_ty,
        ),
        (
            "stone_cech_universal_property",
            stone_cech_universal_property_ty,
        ),
        ("quantale_type", quantale_type_ty),
        ("quantale_distributivity", quantale_distributivity_ty),
        ("frame_is_quantale", frame_is_quantale_ty),
        ("formal_topology_covering", formal_topology_covering_ty),
        ("scott_topology_sober", scott_topology_sober_ty),
        ("way_below_interpolation", way_below_interpolation_ty),
        ("valuation_locale", valuation_locale_ty),
        (
            "localic_riesz_representation",
            localic_riesz_representation_ty,
        ),
        ("uniform_locale_completion", uniform_locale_completion_ty),
        ("sublocales_coframe", sublocales_coframe_ty),
        ("open_closed_supplementary", open_closed_supplementary_ty),
        ("soberification_iso", soberification_iso_ty),
        ("compact_regular_normal", compact_regular_normal_ty),
        (
            "compact_regular_paracompact",
            compact_regular_paracompact_ty,
        ),
        (
            "localic_reals_dedekind_complete",
            localic_reals_dedekind_complete_ty,
        ),
        ("localic_heine_borel", localic_heine_borel_ty),
        ("priestley_duality", priestley_duality_ty),
        ("stone_representation", stone_representation_ty),
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
    fn test_frame_properties() {
        let f = Frame::new("TestFrame");
        assert!(f.satisfies_infinite_distributivity());
        assert!(!f.name.is_empty());
        let opens = Frame::opens_of("ℝ");
        assert!(opens.is_spatial());
        let two = Frame::two();
        assert_eq!(two.name, "2");
        assert!(two.is_compact());
        let ps = Frame::power_set("X");
        assert!(ps.is_spatial());
    }
    #[test]
    fn test_locale_construction() {
        let l = Locale::of_space("ℝ");
        assert!(l.is_sober());
        let lr = LocalicReals::new();
        assert!(lr.agrees_with_classical());
        assert!(!lr.is_compact);
        let ui = LocalicReals::unit_interval();
        assert!(ui.is_compact);
        let sc = Locale::stone_cech("ℕ");
        assert!(sc.description.contains("Stone-Čech"));
    }
    #[test]
    fn test_nucleus_validity() {
        let id_nuc = Nucleus::identity("L");
        assert!(id_nuc.is_valid());
        assert_eq!(id_nuc.name, "id");
        let closed_nuc = Nucleus::closed("L", "a");
        assert!(closed_nuc.is_valid());
        assert!(closed_nuc.name.contains("closed"));
        let open_nuc = Nucleus::open("L", "b");
        assert!(open_nuc.is_valid());
        assert!(open_nuc.name.contains("open"));
        let sublocale = id_nuc.sublocale_elements();
        assert!(sublocale.contains("id(a)"));
    }
    #[test]
    fn test_stone_cech_compactification() {
        let bx = StoneCechCompactification::of("X");
        assert!(bx.is_compact_hausdorff());
        let bn = StoneCechCompactification::beta_nat();
        assert_eq!(bn.space, "ℕ");
        assert!(bn.points_are_ultrafilters());
        let rem = bn.remainder();
        assert!(rem.contains("ℕ"));
    }
    #[test]
    fn test_localic_group() {
        let r = LocalicGroup::real_line();
        assert!(r.is_abelian);
        assert!(!r.is_compact);
        assert!(r.has_haar_measure());
        let circle = LocalicGroup::circle();
        assert!(circle.is_compact);
        assert!(circle.is_abelian);
        let zp = LocalicGroup::p_adic_integers(7);
        assert!(zp.is_compact);
        assert_eq!(zp.locale_name, "ℤ_7");
        let dual = r.pontryagin_dual();
        assert!(dual.contains("Pontryagin"));
    }
    #[test]
    fn test_coherent_locale() {
        let cl = CoherentLocale::new("SpecDL");
        assert!(cl.is_compact);
        assert!(cl.is_spectral_space());
        let spec = CoherentLocale::of_distributive_lattice("DL");
        assert!(spec.name.contains("Spec"));
        let th = spec.classifying_theory();
        assert!(th.contains("Coherent theory"));
    }
    #[test]
    fn test_isbell_adjunction() {
        let omega = IsbellAdjunction::omega_functor();
        assert!(omega.contains("Ω"));
        let pt = IsbellAdjunction::pt_functor();
        assert!(pt.contains("pt"));
        let fixed = IsbellAdjunction::fixed_points();
        assert!(fixed.contains("Sober") && fixed.contains("Spatial"));
        let nonsp = IsbellAdjunction::non_spatial_example();
        assert!(nonsp.contains("no points"));
    }
    #[test]
    fn test_stone_duality() {
        let sd = StoneDuality::boolean_algebras_dual();
        assert!(sd.contains("Stone"));
        let gd = StoneDuality::gelfand_duality();
        assert!(gd.contains("C*"));
        let fl = StoneDuality::frames_locales_duality();
        assert!(fl.contains("Loc = Frm^op"));
    }
    #[test]
    fn test_build_point_free_topology_env() {
        let env = build_point_free_topology_env();
        assert!(env.get(&Name::str("Frame")).is_some());
        assert!(env.get(&Name::str("Locale")).is_some());
        assert!(env.get(&Name::str("IsFrame")).is_some());
        assert!(env.get(&Name::str("Nucleus")).is_some());
        assert!(env.get(&Name::str("IsNucleus")).is_some());
        assert!(env.get(&Name::str("Sheaves")).is_some());
        assert!(env.get(&Name::str("StoneCech")).is_some());
        assert!(env.get(&Name::str("LocalicGroup")).is_some());
        assert!(env.get(&Name::str("frame_distributive_law")).is_some());
        assert!(env.get(&Name::str("nucleus_axiom")).is_some());
        assert!(env.get(&Name::str("stone_duality")).is_some());
        assert!(env.get(&Name::str("isbell_adjunction")).is_some());
        assert!(env.get(&Name::str("sheaves_locale_topos")).is_some());
        assert!(env.get(&Name::str("haar_measure_existence")).is_some());
        assert!(env.get(&Name::str("lt_topologies_nuclei")).is_some());
        assert!(env
            .get(&Name::str("coherent_locale_classification"))
            .is_some());
        assert!(env
            .get(&Name::str("stone_cech_universal_property"))
            .is_some());
    }
    #[test]
    fn test_new_axioms_registered() {
        let env = build_point_free_topology_env();
        let expected = [
            "Quantale",
            "IsQuantale",
            "FormalTopology",
            "ContinuousDomain",
            "ScottTopology",
            "WayBelow",
            "UniformLocale",
            "CompleteUniformLocale",
            "SublocaleLatice",
            "CompactRegularLocale",
            "BoolAlg",
            "BoundedDistribLat",
            "quantale_distributivity",
            "frame_is_quantale",
            "formal_topology_covering",
            "scott_topology_sober",
            "way_below_interpolation",
            "valuation_locale",
            "localic_riesz_representation",
            "uniform_locale_completion",
            "sublocales_coframe",
            "open_closed_supplementary",
            "soberification_iso",
            "compact_regular_normal",
            "compact_regular_paracompact",
            "localic_reals_dedekind_complete",
            "localic_heine_borel",
            "priestley_duality",
            "stone_representation",
        ];
        for name in &expected {
            assert!(
                env.get(&Name::str(*name)).is_some(),
                "Expected '{}' not found in environment",
                name
            );
        }
    }
    #[test]
    fn test_quantale() {
        let q = Quantale::new("Q", false, true);
        assert!(!q.is_commutative);
        assert!(q.is_unital);
        assert!(!q.is_frame());
        let frm = Quantale::from_frame("L");
        assert!(frm.is_commutative);
        assert!(frm.is_unital);
        assert!(frm.is_frame());
        let gq = Quantale::girard("G");
        assert!(gq.is_involutive);
        assert!(gq.is_commutative);
        let rel = Quantale::relations("X");
        assert!(!rel.is_commutative);
        assert!(rel.name.contains("Rel"));
    }
    #[test]
    fn test_formal_topology() {
        let ft = FormalTopology::new("S");
        assert!(ft.is_inductively_generated);
        assert!(ft.is_predicative);
        assert!(ft.is_sober());
        let fr = FormalTopology::formal_reals();
        assert!(fr.base.contains("ℚ"));
        let fc = FormalTopology::formal_cantor();
        assert!(fc.base.contains("2"));
        let cm = ft.continuous_map_description("T");
        assert!(cm.contains("S") && cm.contains("T"));
    }
    #[test]
    fn test_scott_topology_frame() {
        let d = ScottTopologyFrame::new("D");
        assert!(!d.has_way_below);
        assert!(!d.is_sober());
        let cd = ScottTopologyFrame::continuous_domain("ℕ_⊥");
        assert!(cd.has_way_below);
        assert!(cd.is_continuous_domain);
        assert!(cd.is_sober());
        let wb = cd.way_below_description();
        assert!(wb.contains("≪"));
        let flat = ScottTopologyFrame::flat_domain("A");
        assert!(flat.dcpo_name.contains("perp"));
    }
    #[test]
    fn test_localic_valuation() {
        let pv = LocalicValuation::probability("L");
        assert!(pv.is_probability_measure());
        assert_eq!(pv.valuation_type, ValuationType::Probability);
        let sv = LocalicValuation::simple("M");
        assert_eq!(sv.valuation_type, ValuationType::Simple);
        assert!(!sv.is_probability_measure());
        let dv = LocalicValuation::dirac("X", "x_0");
        assert!(dv.is_probability_measure());
        assert!(dv.locale_name.contains("δ_x_0"));
        let riesz = LocalicValuation::riesz_representation_description();
        assert!(riesz.contains("Riesz"));
    }
    #[test]
    fn test_uniform_locale() {
        let ul = UniformLocale::weil("L");
        assert_eq!(ul.uniformity_type, UniformityType::Weil);
        assert!(!ul.is_compact());
        let r = UniformLocale::reals();
        assert!(r.is_complete);
        assert_eq!(r.uniformity_type, UniformityType::Metric);
        let compl = ul.completion();
        assert!(compl.is_complete);
        assert!(compl.locale_name.contains("Compl"));
        let tb = UniformLocale {
            locale_name: "K".to_string(),
            is_complete: true,
            is_totally_bounded: true,
            uniformity_type: UniformityType::Covering,
        };
        assert!(tb.is_compact());
    }
    #[test]
    fn test_sublocale() {
        let open = Sublocale::open("L", "a");
        assert_eq!(open.sublocale_type, SublocaleType::Open);
        assert!(!open.is_dense());
        let closed = Sublocale::closed("L", "b");
        assert_eq!(closed.sublocale_type, SublocaleType::Closed);
        let dn = Sublocale::double_negation("L");
        assert!(dn.is_dense());
        assert!(dn.nucleus.name.contains("¬¬"));
        assert!(Sublocale::are_complementary(&open, &closed));
        let other_open = Sublocale::open("M", "a");
        assert!(!Sublocale::are_complementary(&other_open, &closed));
    }
    #[test]
    fn test_compact_regular_locale() {
        let crl = CompactRegularLocale::new("L");
        assert!(crl.is_normal);
        assert!(crl.is_paracompact);
        let ui = CompactRegularLocale::unit_interval();
        assert!(ui.is_second_countable);
        assert!(ui.is_compact());
        let norm = crl.normality_statement();
        assert!(norm.contains("disjoint"));
        let para = crl.paracompactness_statement();
        assert!(para.contains("paracompact") || para.contains("locally finite"));
    }
    #[test]
    fn test_sober_space() {
        let sober = SoberSpace::new("X", true);
        assert!(sober.is_sober);
        assert!(sober.is_t0);
        assert!(sober.is_fixed_by_soberification());
        let non_sober = SoberSpace::new("Y", false);
        assert!(!non_sober.is_fixed_by_soberification());
        let sober_hull = sober.soberification();
        assert!(sober_hull.contains("pt(Ω(X))"));
        let cond = SoberSpace::alexandrov_sobriety_condition();
        assert!(cond.contains("Alexandrov") || cond.contains("sober"));
    }
    #[test]
    fn test_frame_hom_compose() {
        let f = FrameHom::new("L", "M");
        let g = FrameHom::new("M", "N");
        let fg = FrameHom::compose(&f, &g);
        assert!(fg.is_some());
        let fg = fg.expect("fg should be valid");
        assert_eq!(fg.source, "L");
        assert_eq!(fg.target, "N");
        let h = FrameHom::new("X", "Y");
        assert!(FrameHom::compose(&f, &h).is_none());
    }
}
#[cfg(test)]
mod tests_pft_extra {
    use super::*;
    #[test]
    fn test_locale_sierpinski() {
        let s = LocaleFrame::sierpinski();
        assert_eq!(s.n_elements(), 3);
        assert_eq!(s.top, "top");
        assert_eq!(s.bottom, "0");
    }
    #[test]
    fn test_sober_space_properties() {
        assert!(SoberTopSpace::hausdorff_is_sober());
        assert!(SoberTopSpace::sober_implies_t0());
    }
    #[test]
    fn test_nucleus() {
        let n = FrameNucleus::double_negation("Frame1");
        assert_eq!(n.frame_name, "Frame1");
        let id = FrameNucleus::identity("Frame1");
        assert!(id.description.contains("identity"));
    }
    #[test]
    fn test_spectrum() {
        let mut spec = Spectrum::new("DistLattice");
        spec.add_prime_filter("pf1");
        spec.add_prime_filter("pf2");
        assert_eq!(spec.n_points(), 2);
    }
    #[test]
    fn test_dcpo_algebraic() {
        let d = Dcpo::flat_domain("Lift_Nat");
        assert!(d.is_pointed);
        assert!(d.every_algebraic_is_continuous());
        assert!(d.scott_topology_is_sober());
    }
    #[test]
    fn test_stone_cech() {
        let sc = StoneCechCompact::new("RealLine", true);
        assert!(sc.compactification_exists());
        assert!(sc.is_compact_hausdorff());
    }
    #[test]
    fn test_pointless_map_compose() {
        let f = PointlessMap::new("X", "Y");
        let g = PointlessMap::new("Y", "Z");
        let fg = PointlessMap::compose(&f, &g);
        assert!(fg.is_some());
        let fg = fg.expect("fg should be valid");
        assert_eq!(fg.source_locale, "X");
        assert_eq!(fg.target_locale, "Z");
        let h = PointlessMap::new("A", "B");
        assert!(PointlessMap::compose(&f, &h).is_none());
    }
    #[test]
    fn test_formal_ball() {
        let b1 = FormalBall::new(0.0, 2.0);
        let b2 = FormalBall::new(0.0, 1.0);
        assert!(b1.contains(1.5));
        assert!(!b1.contains(3.0));
        assert!(b2.is_below(&b1));
        let balls = vec![FormalBall::new(0.0, 3.0), FormalBall::new(1.0, 2.0)];
        let sup = FormalBall::supremum(&balls);
        assert!(sup.is_some());
    }
}
