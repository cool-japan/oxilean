//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
#![allow(clippy::items_after_test_module)]

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    CompleteOneType, DLOAtom, EFGame, FiniteModel, FiniteStructure, FirstOrderFormula,
    MorleyRankComputer, MorleyRankResult, PartialIso, QuantifierEliminator, Theory, TypeSpace,
    UltrafilterProduct,
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
pub fn impl_pi(name: &str, dom: Expr, body: Expr) -> Expr {
    pi(BinderInfo::Implicit, name, dom, body)
}
pub fn bvar(n: u32) -> Expr {
    Expr::BVar(n)
}
pub fn nat_ty() -> Expr {
    cst("Nat")
}
/// First-order structure (domain + interpretations)
pub fn structure_ty() -> Expr {
    type0()
}
/// Satisfaction relation: M ⊨ φ
pub fn satisfaction_ty() -> Expr {
    arrow(cst("Structure"), arrow(cst("FOFormula"), prop()))
}
/// Theory: set of sentences
pub fn theory_ty() -> Expr {
    arrow(cst("FOFormula"), prop())
}
/// Model of a theory
pub fn model_ty() -> Expr {
    arrow(
        arrow(cst("FOFormula"), prop()),
        arrow(cst("Structure"), prop()),
    )
}
/// Elementary equivalence: M ≡ N
pub fn elementary_equiv_ty() -> Expr {
    arrow(cst("Structure"), arrow(cst("Structure"), prop()))
}
/// Compactness theorem: if every finite subset is satisfiable, so is the whole theory.
pub fn compactness_ty() -> Expr {
    let theory = arrow(cst("FOFormula"), prop());
    impl_pi(
        "T",
        theory.clone(),
        arrow(
            impl_pi("S", cst("FiniteSubset"), app(cst("Satisfiable"), bvar(0))),
            app(cst("Satisfiable"), bvar(1)),
        ),
    )
}
/// Löwenheim-Skolem theorem: satisfiable → model of any infinite cardinality.
pub fn lowenheim_skolem_ty() -> Expr {
    let theory = arrow(cst("FOFormula"), prop());
    impl_pi(
        "T",
        theory,
        arrow(
            app(cst("Satisfiable"), bvar(0)),
            impl_pi(
                "κ",
                cst("Cardinal"),
                app2(cst("InfiniteModel"), bvar(2), bvar(0)),
            ),
        ),
    )
}
/// Gödel completeness theorem for first-order logic.
pub fn completeness_fo_ty() -> Expr {
    let theory = arrow(cst("FOFormula"), prop());
    impl_pi(
        "T",
        theory,
        impl_pi(
            "φ",
            cst("FOFormula"),
            arrow(
                app2(cst("Entails"), bvar(1), bvar(0)),
                app2(cst("Proves"), bvar(2), bvar(1)),
            ),
        ),
    )
}
/// Gödel's first incompleteness theorem.
pub fn godel_incompleteness_ty() -> Expr {
    let theory = arrow(cst("FOFormula"), prop());
    impl_pi(
        "T",
        theory,
        arrow(
            app(cst("Consistent"), bvar(0)),
            arrow(
                app2(cst("Extends"), bvar(1), cst("PeanoArithmetic")),
                arrow(app(cst("Complete"), bvar(2)), cst("False")),
            ),
        ),
    )
}
/// Lindström's theorem: FOL is maximal with compactness + Löwenheim-Skolem.
pub fn lindstrom_ty() -> Expr {
    impl_pi(
        "L",
        cst("AbstractLogic"),
        arrow(
            app(cst("HasCompactness"), bvar(0)),
            arrow(
                app(cst("HasLowenheimSkolem"), bvar(1)),
                app2(cst("LogicLeq"), bvar(2), cst("FOL")),
            ),
        ),
    )
}
/// Ehrenfeucht-Fraïssé theorem: EF games characterize elementary equivalence.
pub fn ehrenfeucht_fraisse_ty() -> Expr {
    impl_pi(
        "M",
        cst("Structure"),
        impl_pi(
            "N",
            cst("Structure"),
            app2(
                cst("Iff"),
                app2(cst("ElementaryEquiv"), bvar(1), bvar(0)),
                app2(cst("DuplicatorWins"), bvar(2), bvar(1)),
            ),
        ),
    )
}
/// Tarski truth definition: sentence is true in M iff M ⊨ φ.
/// TarskiTruth : ∀ (M : Structure) (φ : Sentence), TrueIn M φ ↔ Satisfies M φ
pub fn tarski_truth_ty() -> Expr {
    impl_pi(
        "M",
        cst("Structure"),
        impl_pi(
            "φ",
            cst("Sentence"),
            app2(
                cst("Iff"),
                app2(cst("TrueIn"), bvar(1), bvar(0)),
                app2(cst("Satisfies"), bvar(2), bvar(1)),
            ),
        ),
    )
}
/// Complete theory: for every sentence φ, T ⊢ φ or T ⊢ ¬φ.
/// CompleteTheory : Theory → Prop
pub fn complete_theory_ty() -> Expr {
    arrow(cst("Theory"), prop())
}
/// Decidable theory: T is decidable iff there is an algorithm deciding T ⊢ φ.
/// DecidableTheory : Theory → Prop
pub fn decidable_theory_ty() -> Expr {
    arrow(cst("Theory"), prop())
}
/// A theory is model-complete if every embedding between models is elementary.
/// ModelComplete : Theory → Prop
pub fn model_complete_ty() -> Expr {
    arrow(cst("Theory"), prop())
}
/// Type space S_n(T) over a theory: set of complete n-types.
/// TypeSpace : Theory → Nat → Type
pub fn type_space_ty() -> Expr {
    arrow(cst("Theory"), arrow(nat_ty(), type0()))
}
/// A complete type: maximal consistent set of formulas with free variable x.
/// CompleteType : Theory → Type
pub fn complete_type_ty() -> Expr {
    arrow(cst("Theory"), type0())
}
/// Type realization: M realizes p iff some element satisfies all formulas in p.
/// Realizes : Structure → CompleteType → Prop
pub fn realizes_ty() -> Expr {
    arrow(cst("Structure"), arrow(cst("CompleteType"), prop()))
}
/// Saturated model: M is κ-saturated if it realizes all types over sets of size < κ.
/// Saturated : Structure → Cardinal → Prop
pub fn saturated_ty() -> Expr {
    arrow(cst("Structure"), arrow(cst("Cardinal"), prop()))
}
/// Omitting types theorem: if p is non-isolated, some model omits p.
/// OmittingTypes : ∀ (T : Theory) (p : CompleteType T), ¬Isolated p → ∃ M : Model T, ¬Realizes M p
pub fn omitting_types_ty() -> Expr {
    impl_pi(
        "T",
        cst("Theory"),
        impl_pi(
            "p",
            app(cst("CompleteTypeOf"), bvar(0)),
            arrow(
                arrow(app(cst("Isolated"), bvar(0)), cst("False")),
                app2(
                    cst("ExistsModel"),
                    bvar(2),
                    arrow(app(cst("Realizes"), bvar(0)), cst("False")),
                ),
            ),
        ),
    )
}
/// ω-categoricity: T is ω-categorical iff T has a unique countable model up to isomorphism.
/// OmegaCategorical : Theory → Prop
pub fn omega_categorical_ty() -> Expr {
    arrow(cst("Theory"), prop())
}
/// Ryll-Nardzewski theorem: T is ω-categorical iff Aut(M) acts oligomorphically.
/// RyllNardzewski : ∀ (T : Theory), OmegaCategorical T ↔ Oligomorphic (Aut (CountableModel T))
pub fn ryll_nardzewski_ty() -> Expr {
    impl_pi(
        "T",
        cst("Theory"),
        app2(
            cst("Iff"),
            app(cst("OmegaCategorical"), bvar(0)),
            app(
                cst("Oligomorphic"),
                app(cst("Aut"), app(cst("CountableModel"), bvar(1))),
            ),
        ),
    )
}
/// κ-categoricity: T has a unique model of cardinality κ up to isomorphism.
/// KappaCategorical : Theory → Cardinal → Prop
pub fn kappa_categorical_ty() -> Expr {
    arrow(cst("Theory"), arrow(cst("Cardinal"), prop()))
}
/// Morley's categoricity theorem: uncountably categorical in one cardinality → in all.
/// MorleyCategoricity : ∀ (T : Theory) (κ : Cardinal), Uncountable κ → KappaCategorical T κ
///   → ∀ λ : Cardinal, Uncountable λ → KappaCategorical T λ
pub fn morley_categoricity_ty() -> Expr {
    impl_pi(
        "T",
        cst("Theory"),
        impl_pi(
            "κ",
            cst("Cardinal"),
            arrow(
                app(cst("Uncountable"), bvar(0)),
                arrow(
                    app2(cst("KappaCategorical"), bvar(2), bvar(1)),
                    impl_pi(
                        "λ",
                        cst("Cardinal"),
                        arrow(
                            app(cst("Uncountable"), bvar(0)),
                            app2(cst("KappaCategorical"), bvar(4), bvar(1)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// Stable theory: T is stable iff no formula has the order property.
/// StableTheory : Theory → Prop
pub fn stable_theory_ty() -> Expr {
    arrow(cst("Theory"), prop())
}
/// Superstable theory: stable + no long forking chains.
/// SuperstableTheory : Theory → Prop
pub fn superstable_theory_ty() -> Expr {
    arrow(cst("Theory"), prop())
}
/// ω-stable theory: stable in every infinite cardinal.
/// OmegaStableTheory : Theory → Prop
pub fn omega_stable_theory_ty() -> Expr {
    arrow(cst("Theory"), prop())
}
/// Stability spectrum: for which κ is T stable.
/// StableAt : Theory → Cardinal → Prop
pub fn stable_at_ty() -> Expr {
    arrow(cst("Theory"), arrow(cst("Cardinal"), prop()))
}
/// Forking independence: a ⊥_C b in structure M.
/// Forking : Structure → Elem → Elem → Set → Prop
pub fn forking_ty() -> Expr {
    arrow(
        cst("Structure"),
        arrow(
            cst("Elem"),
            arrow(cst("Elem"), arrow(cst("ElemSet"), prop())),
        ),
    )
}
/// Non-forking extension: every type over A has a non-forking extension over B ⊇ A.
/// NonForkingExtension : ∀ (T : Theory), Stable T → ∀ p : Type A, ∃ q : Type B, NonForking q
pub fn non_forking_extension_ty() -> Expr {
    impl_pi(
        "T",
        cst("Theory"),
        arrow(
            app(cst("Stable"), bvar(0)),
            app2(cst("HasNonForkingExtensions"), bvar(1), bvar(0)),
        ),
    )
}
/// Morley rank: ordinal-valued rank of a definable set.
/// MorleyRank : Theory → DefSet → Ordinal
pub fn morley_rank_ty() -> Expr {
    arrow(cst("Theory"), arrow(cst("DefSet"), cst("Ordinal")))
}
/// Morley degree: finite degree of a definable set of given Morley rank.
/// MorleyDegree : Theory → DefSet → Nat
pub fn morley_degree_ty() -> Expr {
    arrow(cst("Theory"), arrow(cst("DefSet"), nat_ty()))
}
/// Strongly minimal set: every definable subset is finite or cofinite.
/// StronglyMinimal : Theory → DefSet → Prop
pub fn strongly_minimal_ty() -> Expr {
    arrow(cst("Theory"), arrow(cst("DefSet"), prop()))
}
/// Strongly minimal theory: the theory itself has a strongly minimal formula.
/// StronglyMinimalTheory : Theory → Prop
pub fn strongly_minimal_theory_ty() -> Expr {
    arrow(cst("Theory"), prop())
}
/// NIP (Not the Independence Property): T has NIP iff no formula has IP.
/// NIPTheory : Theory → Prop
pub fn nip_theory_ty() -> Expr {
    arrow(cst("Theory"), prop())
}
/// dp-rank: a cardinal-valued complexity measure for NIP theories.
/// DpRank : Theory → Cardinal
pub fn dp_rank_ty() -> Expr {
    arrow(cst("Theory"), cst("Cardinal"))
}
/// VC dimension of a formula φ in theory T.
/// VCDimension : Theory → FOFormula → Nat
pub fn vc_dimension_ty() -> Expr {
    arrow(cst("Theory"), arrow(cst("FOFormula"), nat_ty()))
}
/// Simple theory: T is simple iff forking is symmetric and transitive.
/// SimpleTheory : Theory → Prop
pub fn simple_theory_ty() -> Expr {
    arrow(cst("Theory"), prop())
}
/// NSOP (Not the Strict Order Property): T does not encode a strict order.
/// NSOPTheory : Theory → Prop
pub fn nsop_theory_ty() -> Expr {
    arrow(cst("Theory"), prop())
}
/// o-minimal structure: every definable subset of M is a finite union of intervals/points.
/// OMinimalStructure : Structure → Prop
pub fn o_minimal_structure_ty() -> Expr {
    arrow(cst("Structure"), prop())
}
/// o-minimal theory: all models are o-minimal.
/// OMinimalTheory : Theory → Prop
pub fn o_minimal_theory_ty() -> Expr {
    arrow(cst("Theory"), prop())
}
/// Cell decomposition in o-minimal structures.
/// CellDecomposition : ∀ (M : Structure), OMinimal M → ∀ f : DefMap, CellDecomp M f
pub fn cell_decomposition_ty() -> Expr {
    impl_pi(
        "M",
        cst("Structure"),
        arrow(
            app(cst("OMinimal"), bvar(0)),
            impl_pi(
                "f",
                cst("DefMap"),
                app2(cst("CellDecomp"), bvar(2), bvar(0)),
            ),
        ),
    )
}
/// Algebraically closed valued field (ACVF): theory of algebraically closed fields
/// with a non-trivial valuation.
/// ACVFTheory : Theory
pub fn acvf_theory_ty() -> Expr {
    cst("Theory")
}
/// Algebraically closed field theory (ACF_p for characteristic p).
/// ACFTheory : Nat → Theory
pub fn acf_theory_ty() -> Expr {
    arrow(nat_ty(), cst("Theory"))
}
/// Real closed field theory (RCF): complete decidable theory.
/// RCFTheory : Theory
pub fn rcf_theory_ty() -> Expr {
    cst("Theory")
}
/// Tarski's theorem: RCF admits quantifier elimination.
/// TarskiQE : QuantifierElim RCFTheory
pub fn tarski_qe_ty() -> Expr {
    app(cst("QuantifierElim"), cst("RCFTheory"))
}
/// Fraïssé limit of a class of finite structures.
/// FraisseLimitOf : StructureClass → Structure
pub fn fraisse_limit_ty() -> Expr {
    arrow(cst("StructureClass"), cst("Structure"))
}
/// Ultrahomogeneous structure: every isomorphism between finite substructures extends.
/// Ultrahomogeneous : Structure → Prop
pub fn ultrahomogeneous_ty() -> Expr {
    arrow(cst("Structure"), prop())
}
/// Fraïssé's theorem: a countable homogeneous structure is a Fraïssé limit.
/// FraisseTheorem : ∀ M : Structure, Countable M → Homogeneous M → IsFraisseLimitOf M (Age M)
pub fn fraisse_theorem_ty() -> Expr {
    impl_pi(
        "M",
        cst("Structure"),
        arrow(
            app(cst("Countable"), bvar(0)),
            arrow(
                app(cst("Homogeneous"), bvar(1)),
                app2(cst("IsFraisseLimitOf"), bvar(2), app(cst("Age"), bvar(3))),
            ),
        ),
    )
}
/// Back-and-forth system between two structures.
/// BackAndForthSystem : Structure → Structure → Prop
pub fn back_and_forth_ty() -> Expr {
    arrow(cst("Structure"), arrow(cst("Structure"), prop()))
}
/// Rado graph (random graph): unique countable homogeneous universal graph.
/// RadoGraph : Structure
pub fn rado_graph_ty() -> Expr {
    cst("Structure")
}
/// A theory admits quantifier elimination.
/// QuantifierElim : Theory → Prop
pub fn quantifier_elim_ty() -> Expr {
    arrow(cst("Theory"), prop())
}
/// DLO (dense linear orders without endpoints) admits quantifier elimination.
/// DLOQuantifierElim : QuantifierElim DLOTheory
pub fn dlo_qe_ty() -> Expr {
    app(cst("QuantifierElim"), cst("DLOTheory"))
}
/// ACF admits quantifier elimination.
/// ACFQuantifierElim : ∀ p : Nat, QuantifierElim (ACFTheory p)
pub fn acf_qe_ty() -> Expr {
    impl_pi(
        "p",
        nat_ty(),
        app(cst("QuantifierElim"), app(cst("ACFTheory"), bvar(0))),
    )
}
/// Definably amenable group: a group with a definable invariant measure.
/// DefinablyAmenable : Theory → Prop
pub fn definably_amenable_ty() -> Expr {
    arrow(cst("Theory"), prop())
}
/// Elementary substructure: N ≺ M means N is an elementary substructure of M.
/// ElementarySubstructure : Structure → Structure → Prop
pub fn elementary_substructure_ty() -> Expr {
    arrow(cst("Structure"), arrow(cst("Structure"), prop()))
}
/// Tarski-Vaught test: N ≺ M iff for every formula φ(x,ā), N ⊨ ∃x φ implies N has witness.
/// TarskiVaughtTest : ∀ N M, ElementarySubstructure N M ↔ TarskiVaughtCondition N M
pub fn tarski_vaught_ty() -> Expr {
    impl_pi(
        "N",
        cst("Structure"),
        impl_pi(
            "M",
            cst("Structure"),
            app2(
                cst("Iff"),
                app2(cst("ElemSub"), bvar(1), bvar(0)),
                app2(cst("TarskiVaughtCond"), bvar(2), bvar(1)),
            ),
        ),
    )
}
/// Register all model theory axioms into the kernel environment.
pub fn build_model_theory_env(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("Structure", structure_ty()),
        ("FOFormula", type0()),
        ("Satisfaction", satisfaction_ty()),
        ("Theory", theory_ty()),
        ("Model", model_ty()),
        ("ElementaryEquiv", elementary_equiv_ty()),
        ("Cardinal", type0()),
        ("FiniteSubset", type0()),
        ("Satisfiable", arrow(cst("Theory"), prop())),
        (
            "InfiniteModel",
            arrow(cst("Theory"), arrow(cst("Cardinal"), prop())),
        ),
        (
            "Entails",
            arrow(cst("Theory"), arrow(cst("FOFormula"), prop())),
        ),
        (
            "Proves",
            arrow(cst("Theory"), arrow(cst("FOFormula"), prop())),
        ),
        ("Consistent", arrow(cst("Theory"), prop())),
        ("Complete", arrow(cst("Theory"), prop())),
        (
            "Extends",
            arrow(cst("Theory"), arrow(cst("Theory"), prop())),
        ),
        ("PeanoArithmetic", arrow(cst("FOFormula"), prop())),
        ("AbstractLogic", type0()),
        ("HasCompactness", arrow(cst("AbstractLogic"), prop())),
        ("HasLowenheimSkolem", arrow(cst("AbstractLogic"), prop())),
        (
            "LogicLeq",
            arrow(cst("AbstractLogic"), arrow(cst("AbstractLogic"), prop())),
        ),
        ("FOL", cst("AbstractLogic")),
        (
            "DuplicatorWins",
            arrow(cst("Structure"), arrow(cst("Structure"), prop())),
        ),
        ("Sentence", type0()),
        (
            "TrueIn",
            arrow(cst("Structure"), arrow(cst("Sentence"), prop())),
        ),
        (
            "Satisfies",
            arrow(cst("Structure"), arrow(cst("Sentence"), prop())),
        ),
        ("Ordinal", type0()),
        ("DefSet", type0()),
        ("DefMap", type0()),
        ("ElemSet", type0()),
        ("Elem", type0()),
        ("StructureClass", type0()),
        ("CompleteType", type0()),
        ("CompleteTypeOf", arrow(cst("Theory"), type0())),
        ("Isolated", arrow(cst("CompleteType"), prop())),
        (
            "ExistsModel",
            arrow(
                cst("Theory"),
                arrow(arrow(cst("Structure"), prop()), prop()),
            ),
        ),
        (
            "Realizes",
            arrow(cst("Structure"), arrow(cst("CompleteType"), prop())),
        ),
        (
            "Saturated",
            arrow(cst("Structure"), arrow(cst("Cardinal"), prop())),
        ),
        ("OmegaCategorical", arrow(cst("Theory"), prop())),
        ("Oligomorphic", arrow(cst("Group"), prop())),
        ("Aut", arrow(cst("Structure"), cst("Group"))),
        ("Group", type0()),
        ("CountableModel", arrow(cst("Theory"), cst("Structure"))),
        (
            "KappaCategorical",
            arrow(cst("Theory"), arrow(cst("Cardinal"), prop())),
        ),
        ("Uncountable", arrow(cst("Cardinal"), prop())),
        ("Stable", arrow(cst("Theory"), prop())),
        (
            "HasNonForkingExtensions",
            arrow(cst("Theory"), arrow(cst("Theory"), prop())),
        ),
        (
            "MorleyRankVal",
            arrow(cst("Theory"), arrow(cst("DefSet"), cst("Ordinal"))),
        ),
        (
            "StronglyMinimal",
            arrow(cst("Theory"), arrow(cst("DefSet"), prop())),
        ),
        ("NIP", arrow(cst("Theory"), prop())),
        ("OMinimal", arrow(cst("Structure"), prop())),
        (
            "CellDecomp",
            arrow(cst("Structure"), arrow(cst("DefMap"), prop())),
        ),
        ("RCFTheory", cst("Theory")),
        ("DLOTheory", cst("Theory")),
        ("QuantifierElim", arrow(cst("Theory"), prop())),
        ("StructureClass", type0()),
        (
            "IsFraisseLimitOf",
            arrow(cst("Structure"), arrow(cst("StructureClass"), prop())),
        ),
        ("Age", arrow(cst("Structure"), cst("StructureClass"))),
        ("Countable", arrow(cst("Structure"), prop())),
        ("Homogeneous", arrow(cst("Structure"), prop())),
        (
            "ElemSub",
            arrow(cst("Structure"), arrow(cst("Structure"), prop())),
        ),
        (
            "TarskiVaughtCond",
            arrow(cst("Structure"), arrow(cst("Structure"), prop())),
        ),
        ("compactness", compactness_ty()),
        ("lowenheim_skolem", lowenheim_skolem_ty()),
        ("completeness_fo", completeness_fo_ty()),
        ("godel_incompleteness", godel_incompleteness_ty()),
        ("lindstrom", lindstrom_ty()),
        ("ehrenfeucht_fraisse", ehrenfeucht_fraisse_ty()),
        ("tarski_truth", tarski_truth_ty()),
        ("complete_theory", complete_theory_ty()),
        ("decidable_theory", decidable_theory_ty()),
        ("model_complete", model_complete_ty()),
        ("type_space", type_space_ty()),
        ("complete_type", complete_type_ty()),
        ("realizes", realizes_ty()),
        ("saturated", saturated_ty()),
        ("omitting_types", omitting_types_ty()),
        ("omega_categorical", omega_categorical_ty()),
        ("ryll_nardzewski", ryll_nardzewski_ty()),
        ("kappa_categorical", kappa_categorical_ty()),
        ("morley_categoricity", morley_categoricity_ty()),
        ("stable_theory", stable_theory_ty()),
        ("superstable_theory", superstable_theory_ty()),
        ("omega_stable_theory", omega_stable_theory_ty()),
        ("stable_at", stable_at_ty()),
        ("forking", forking_ty()),
        ("non_forking_extension", non_forking_extension_ty()),
        ("morley_rank", morley_rank_ty()),
        ("morley_degree", morley_degree_ty()),
        ("strongly_minimal", strongly_minimal_ty()),
        ("strongly_minimal_theory", strongly_minimal_theory_ty()),
        ("nip_theory", nip_theory_ty()),
        ("dp_rank", dp_rank_ty()),
        ("vc_dimension", vc_dimension_ty()),
        ("simple_theory", simple_theory_ty()),
        ("nsop_theory", nsop_theory_ty()),
        ("o_minimal_structure", o_minimal_structure_ty()),
        ("o_minimal_theory", o_minimal_theory_ty()),
        ("cell_decomposition", cell_decomposition_ty()),
        ("acvf_theory", acvf_theory_ty()),
        ("acf_theory", acf_theory_ty()),
        ("rcf_theory", rcf_theory_ty()),
        ("tarski_qe", tarski_qe_ty()),
        ("fraisse_limit", fraisse_limit_ty()),
        ("ultrahomogeneous", ultrahomogeneous_ty()),
        ("fraisse_theorem", fraisse_theorem_ty()),
        ("back_and_forth", back_and_forth_ty()),
        ("rado_graph", rado_graph_ty()),
        ("quantifier_elim", quantifier_elim_ty()),
        ("dlo_qe", dlo_qe_ty()),
        ("acf_qe", acf_qe_ty()),
        ("definably_amenable", definably_amenable_ty()),
        ("elementary_substructure", elementary_substructure_ty()),
        ("tarski_vaught", tarski_vaught_ty()),
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
/// Theory of groups (identity, inverse, associativity).
pub fn theory_of_groups() -> Theory {
    let mut t = Theory::new("Groups");
    t.signature = vec!["e".to_string(), "inv".to_string(), "mul".to_string()];
    t.add_axiom("∀ x, mul(e, x) = x");
    t.add_axiom("∀ x, mul(inv(x), x) = e");
    t.add_axiom("∀ x y z, mul(mul(x,y),z) = mul(x,mul(y,z))");
    t
}
/// Theory of rings (additive group + multiplicative monoid + distributivity).
pub fn theory_of_rings() -> Theory {
    let mut t = Theory::new("Rings");
    t.signature = vec![
        "zero".to_string(),
        "one".to_string(),
        "add".to_string(),
        "neg".to_string(),
        "mul".to_string(),
    ];
    t.add_axiom("∀ x, add(zero, x) = x");
    t.add_axiom("∀ x, add(neg(x), x) = zero");
    t.add_axiom("∀ x y z, add(add(x,y),z) = add(x,add(y,z))");
    t.add_axiom("∀ x y, add(x,y) = add(y,x)");
    t.add_axiom("∀ x, mul(one, x) = x");
    t.add_axiom("∀ x, mul(x, one) = x");
    t.add_axiom("∀ x y z, mul(mul(x,y),z) = mul(x,mul(y,z))");
    t.add_axiom("∀ x y z, mul(x,add(y,z)) = add(mul(x,y),mul(x,z))");
    t
}
/// Theory of linear orders (reflexive, antisymmetric, transitive, total).
pub fn theory_of_linear_orders() -> Theory {
    let mut t = Theory::new("LinearOrders");
    t.signature = vec!["leq".to_string()];
    t.add_axiom("∀ x, leq(x,x)");
    t.add_axiom("∀ x y, leq(x,y) ∧ leq(y,x) → x = y");
    t.add_axiom("∀ x y z, leq(x,y) ∧ leq(y,z) → leq(x,z)");
    t.add_axiom("∀ x y, leq(x,y) ∨ leq(y,x)");
    t
}
/// Theory of dense linear orders without endpoints (DLO).
pub fn theory_of_dense_linear_orders() -> Theory {
    let mut t = theory_of_linear_orders();
    t.name = "DenseLinearOrders".to_string();
    t.add_axiom("∀ x y, x < y → ∃ z, x < z ∧ z < y");
    t.add_axiom("∀ x, ∃ y, y < x");
    t.add_axiom("∀ x, ∃ y, x < y");
    t
}
/// Theory of Peano Arithmetic.
pub fn theory_of_peano_arithmetic() -> Theory {
    let mut t = Theory::new("PeanoArithmetic");
    t.signature = vec![
        "zero".to_string(),
        "succ".to_string(),
        "add".to_string(),
        "mul".to_string(),
    ];
    t.add_axiom("∀ x, succ(x) ≠ zero");
    t.add_axiom("∀ x y, succ(x) = succ(y) → x = y");
    t.add_axiom("∀ x, add(x, zero) = x");
    t.add_axiom("∀ x y, add(x, succ(y)) = succ(add(x, y))");
    t.add_axiom("∀ x, mul(x, zero) = zero");
    t.add_axiom("∀ x y, mul(x, succ(y)) = add(mul(x, y), x)");
    t.add_axiom("(P(zero) ∧ ∀ x, P(x) → P(succ(x))) → ∀ x, P(x)");
    t
}
/// Theory of real closed fields.
pub fn theory_of_real_closed_fields() -> Theory {
    let mut t = theory_of_rings();
    t.name = "RealClosedFields".to_string();
    t.add_axiom("∀ x y, x < y ∨ x = y ∨ y < x");
    t.add_axiom("∀ x, x > 0 → ∃ y, y * y = x");
    t.add_axiom("Every odd-degree polynomial has a root");
    t
}
/// Theory of algebraically closed fields of characteristic 0 (ACF_0).
pub fn theory_of_acf0() -> Theory {
    let mut t = theory_of_rings();
    t.name = "ACF0".to_string();
    t.add_axiom("∀ x, x ≠ 0 → ∃ y, x * y = 1");
    t.add_axiom("Every non-constant polynomial has a root");
    t.add_axiom("char = 0: 1+1≠0, 1+1+1≠0, ...");
    t
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::proof_theory::Formula;
    #[test]
    fn test_finite_structure_new() {
        let s = FiniteStructure::new(vec!["a".to_string(), "b".to_string(), "c".to_string()]);
        assert_eq!(s.domain_size(), 3);
        assert!(s.constants.is_empty());
        assert!(s.relations.is_empty());
    }
    #[test]
    fn test_finite_structure_add_relation() {
        let mut s = FiniteStructure::new(vec!["0".to_string(), "1".to_string(), "2".to_string()]);
        s.add_relation("lt", vec![vec![0, 1], vec![0, 2], vec![1, 2]]);
        assert!(s.satisfies_relation("lt", &[0, 1]));
        assert!(s.satisfies_relation("lt", &[1, 2]));
        assert!(!s.satisfies_relation("lt", &[2, 1]));
    }
    #[test]
    fn test_theory_of_groups() {
        let t = theory_of_groups();
        assert!(t.n_axioms() >= 3);
        assert_eq!(t.name, "Groups");
    }
    #[test]
    fn test_theory_of_peano_arithmetic() {
        let t = theory_of_peano_arithmetic();
        assert!(t.n_axioms() >= 5);
        assert_eq!(t.name, "PeanoArithmetic");
    }
    #[test]
    fn test_ef_game_same_size_no_win() {
        let a = FiniteStructure::new(vec!["0".to_string(), "1".to_string()]);
        let b = FiniteStructure::new(vec!["x".to_string(), "y".to_string()]);
        let game = EFGame::new(3, a, b);
        assert!(!game.spoiler_wins());
    }
    #[test]
    fn test_ef_game_diff_size_spoiler_wins() {
        let a = FiniteStructure::new(vec!["0".to_string()]);
        let b = FiniteStructure::new(vec!["x".to_string(), "y".to_string()]);
        let game = EFGame::new(2, a, b);
        assert!(game.spoiler_wins());
    }
    #[test]
    fn test_formula_from_proof_theory() {
        let a = Formula::atom("A");
        let taut = Formula::implies(a.clone(), a);
        assert!(taut.is_tautology());
    }
    #[test]
    fn test_ultrafilter_product_trivial() {
        let trivial = UltrafilterProduct::new(0, 5);
        assert!(trivial.is_trivial());
        let non_trivial = UltrafilterProduct::new(3, 5);
        assert!(!non_trivial.is_trivial());
    }
    #[test]
    fn test_first_order_formula_tautology() {
        let t = FirstOrderFormula::True;
        assert!(t.is_propositional_tautology());
        let f = FirstOrderFormula::False;
        assert!(!f.is_propositional_tautology());
        let tf = FirstOrderFormula::or(FirstOrderFormula::True, FirstOrderFormula::False);
        assert!(tf.is_propositional_tautology());
    }
    #[test]
    fn test_finite_model_satisfaction() {
        let mut s = FiniteStructure::new(vec!["0".to_string(), "1".to_string()]);
        s.add_relation("lt", vec![vec![0, 1]]);
        let m = FiniteModel::new(s);
        assert!(m.satisfies(&FirstOrderFormula::True));
        assert!(!m.satisfies(&FirstOrderFormula::False));
    }
    #[test]
    fn test_finite_model_forall_true() {
        let s = FiniteStructure::new(vec!["0".to_string(), "1".to_string()]);
        let m = FiniteModel::new(s);
        let f = FirstOrderFormula::ForAll(Box::new(FirstOrderFormula::True));
        assert!(m.satisfies(&f));
    }
    #[test]
    fn test_finite_model_exists_false() {
        let s = FiniteStructure::new(vec!["0".to_string(), "1".to_string()]);
        let m = FiniteModel::new(s);
        let f = FirstOrderFormula::Exists(Box::new(FirstOrderFormula::False));
        assert!(!m.satisfies(&f));
    }
    #[test]
    fn test_type_space_accumulation() {
        let mut ts = TypeSpace::new("DLO");
        let mut tp1 = CompleteOneType::new();
        tp1.add_formula("x > 0");
        tp1.add_formula("x < 1");
        tp1.mark_realized();
        ts.add_type(tp1);
        let mut tp2 = CompleteOneType::new();
        tp2.add_formula("x = 0");
        ts.add_type(tp2);
        assert_eq!(ts.cardinality(), 2);
        assert!(!ts.all_realized());
        assert_eq!(ts.isolated_count(), 1);
    }
    #[test]
    fn test_morley_rank_finite() {
        let computer = MorleyRankComputer::new(5);
        assert_eq!(computer.rank(&[0, 1]), MorleyRankResult::Finite(0));
        assert_eq!(computer.rank(&[0, 1, 2, 3, 4]), MorleyRankResult::Infinite);
        assert_eq!(computer.rank(&[]), MorleyRankResult::Finite(0));
    }
    #[test]
    fn test_morley_strongly_minimal() {
        let computer = MorleyRankComputer::new(4);
        assert!(computer.is_strongly_minimal(&[2]));
        assert!(!computer.is_strongly_minimal(&[1, 2]));
    }
    #[test]
    fn test_quantifier_eliminator_satisfiable() {
        let qe = QuantifierEliminator::new(1);
        let atoms = vec![DLOAtom::GtConst(0, 0), DLOAtom::LtConst(0, 5)];
        assert!(qe.is_satisfiable(&atoms));
        let bad = vec![DLOAtom::GtConst(0, 5), DLOAtom::LtConst(0, 3)];
        assert!(!qe.is_satisfiable(&bad));
    }
    #[test]
    fn test_partial_iso_extend() {
        let mut iso = PartialIso::new();
        assert!(iso.extend(0, 1));
        assert!(iso.extend(1, 0));
        assert!(!iso.extend(0, 2));
        assert_eq!(iso.size(), 2);
    }
    #[test]
    fn test_partial_iso_image() {
        let mut iso = PartialIso::new();
        iso.extend(3, 7);
        assert_eq!(iso.image(3), Some(7));
        assert_eq!(iso.image(5), None);
    }
    #[test]
    fn test_build_model_theory_env() {
        let mut env = oxilean_kernel::Environment::new();
        build_model_theory_env(&mut env);
        assert!(env.get(&oxilean_kernel::Name::str("compactness")).is_some());
        assert!(env
            .get(&oxilean_kernel::Name::str("morley_categoricity"))
            .is_some());
        assert!(env.get(&oxilean_kernel::Name::str("tarski_qe")).is_some());
        assert!(env
            .get(&oxilean_kernel::Name::str("fraisse_theorem"))
            .is_some());
        assert!(env.get(&oxilean_kernel::Name::str("nip_theory")).is_some());
    }
    #[test]
    fn test_theory_of_acf0() {
        let t = theory_of_acf0();
        assert_eq!(t.name, "ACF0");
        assert!(t.n_axioms() >= 3);
    }
    #[test]
    fn test_quantifier_eliminator_eliminate() {
        let qe = QuantifierEliminator::new(1);
        let atoms = vec![DLOAtom::GtConst(0, 1), DLOAtom::LtConst(0, 4)];
        let projected = qe.eliminate_variable(&atoms, 0);
        assert!(projected.is_empty() || qe.is_satisfiable(&projected));
    }
}
pub fn mt_ext_theory_prop() -> Expr {
    arrow(cst("Theory"), prop())
}
pub fn mt_ext_theory_cardinal() -> Expr {
    arrow(cst("Theory"), cst("Cardinal"))
}
/// U-rank (Lascar rank) of a type over a theory.
/// URank : Theory → CompleteType → Ordinal
pub fn u_rank_ty() -> Expr {
    arrow(cst("Theory"), arrow(cst("CompleteType"), cst("Ordinal")))
}
/// Lascar rank: the U-rank defined by Lascar using forking chains.
/// LascarRank : Theory → CompleteType → Ordinal
pub fn lascar_rank_ty() -> Expr {
    arrow(cst("Theory"), arrow(cst("CompleteType"), cst("Ordinal")))
}
/// Shelah's main gap theorem: theories are either classifiable or maximal.
/// ShElahMainGap : Theory → Prop
pub fn shelah_main_gap_ty() -> Expr {
    mt_ext_theory_prop()
}
/// Classifiable theory: superstable with NDOP and NOTOP.
/// ClassifiableTheory : Theory → Prop
pub fn classifiable_theory_ty() -> Expr {
    mt_ext_theory_prop()
}
/// NDOP (No Dimensional Order Property): structure theorem condition.
/// NDOPTheory : Theory → Prop
pub fn ndop_theory_ty() -> Expr {
    mt_ext_theory_prop()
}
/// NOTOP (No Omitting Types Order Property).
/// NOTOPTheory : Theory → Prop
pub fn notop_theory_ty() -> Expr {
    mt_ext_theory_prop()
}
/// Simple theory: forking defines an independence relation satisfying symmetry.
/// SimpleTheoryForkingAxiom : ∀ (T : Theory), Simple T → ForkingSymmetric T
pub fn simple_theory_forking_ty() -> Expr {
    impl_pi(
        "T",
        cst("Theory"),
        arrow(
            app(cst("Simple"), bvar(0)),
            app(cst("ForkingSymmetric"), bvar(1)),
        ),
    )
}
/// NSOP₁ theory: no strict order property of the first kind.
/// NSOP1Theory : Theory → Prop
pub fn nsop1_theory_ty() -> Expr {
    mt_ext_theory_prop()
}
/// NTP₂ theory: no tree property of the second kind.
/// NTP2Theory : Theory → Prop
pub fn ntp2_theory_ty() -> Expr {
    mt_ext_theory_prop()
}
/// Kim-independence: a ⊥^K_C b defined via Kim-forking.
/// KimIndependence : Structure → Elem → Elem → ElemSet → Prop
pub fn kim_independence_ty() -> Expr {
    arrow(
        cst("Structure"),
        arrow(
            cst("Elem"),
            arrow(cst("Elem"), arrow(cst("ElemSet"), prop())),
        ),
    )
}
/// Kim's lemma: in an NSOP₁ theory, Kim-forking equals forking over models.
/// KimsLemma : ∀ (T : Theory), NSOP1 T → KimForkingEqualsForking T
pub fn kims_lemma_ty() -> Expr {
    impl_pi(
        "T",
        cst("Theory"),
        arrow(
            app(cst("NSOP1"), bvar(0)),
            app(cst("KimForkingEqualsForking"), bvar(1)),
        ),
    )
}
/// Beautiful pairs: a pair (M, A) where A ⊆ M has special saturation.
/// BeautifulPair : Structure → ElemSet → Prop
pub fn beautiful_pair_ty() -> Expr {
    arrow(cst("Structure"), arrow(cst("ElemSet"), prop()))
}
/// Pseudo-algebraically closed field (PAC): every absolutely irreducible variety has a point.
/// PACField : Theory → Prop
pub fn pac_field_ty() -> Expr {
    mt_ext_theory_prop()
}
/// Existentially closed model: every existential sentence true in an extension holds in M.
/// ExistentiallyClosed : Theory → Structure → Prop
pub fn existentially_closed_ty() -> Expr {
    arrow(cst("Theory"), arrow(cst("Structure"), prop()))
}
/// Amalgamation property: any two extensions of a model can be amalgamated.
/// AmalgamationProperty : Theory → Prop
pub fn amalgamation_property_ty() -> Expr {
    mt_ext_theory_prop()
}
/// Joint embedding property: any two models embed into a common model.
/// JointEmbeddingProperty : Theory → Prop
pub fn joint_embedding_property_ty() -> Expr {
    mt_ext_theory_prop()
}
/// Differential closed field of characteristic 0 (DCF₀).
/// DCF0Theory : Theory
pub fn dcf0_theory_ty() -> Expr {
    cst("Theory")
}
/// Ordered field (as a theory extending RCF).
/// OrderedFieldTheory : Theory
pub fn ordered_field_theory_ty() -> Expr {
    cst("Theory")
}
/// Valued field: field with a non-Archimedean valuation.
/// ValuedFieldTheory : Theory
pub fn valued_field_theory_ty() -> Expr {
    cst("Theory")
}
/// Residue field of a valued field.
/// ResidueField : ValuedField → Field
pub fn residue_field_ty() -> Expr {
    arrow(cst("ValuedField"), cst("Field"))
}
/// Value group of a valued field.
/// ValueGroup : ValuedField → OrderedAbelianGroup
pub fn value_group_ty() -> Expr {
    arrow(cst("ValuedField"), cst("OrderedAbelianGroup"))
}
/// Henselian field: satisfies Hensel's lemma.
/// HenselianField : Theory → Prop
pub fn henselian_field_ty() -> Expr {
    mt_ext_theory_prop()
}
/// Ax-Kochen theorem: almost all p-adic fields satisfy the same first-order sentences as ℤ_p.
/// AxKochenTheorem : ∀ (φ : FOFormula), ∃ N, ∀ p > N, QpSatisfies p φ ↔ ZpSatisfies p φ
pub fn ax_kochen_ty() -> Expr {
    impl_pi(
        "φ",
        cst("FOFormula"),
        app2(cst("AlmostAllPadicAgree"), bvar(0), cst("PadicIntegers")),
    )
}
/// Elimination of imaginaries: every definable equivalence class has a canonical parameter.
/// EliminationOfImaginaries : Theory → Prop
pub fn elimination_of_imaginaries_ty() -> Expr {
    mt_ext_theory_prop()
}
/// Geometric theory: a theory with quantifier elimination down to positive primitive formulas.
/// GeometricTheory : Theory → Prop
pub fn geometric_theory_ty() -> Expr {
    mt_ext_theory_prop()
}
/// Strongly dependent theory: dp-rank = 1.
/// StronglyDependent : Theory → Prop
pub fn strongly_dependent_ty() -> Expr {
    mt_ext_theory_prop()
}
/// Bounded theory: has only finitely many types over finite sets.
/// BoundedTheory : Theory → Prop
pub fn bounded_theory_ty() -> Expr {
    mt_ext_theory_prop()
}
/// Saturated model existence: every theory has a κ-saturated model for any κ.
/// SaturatedModelExists : ∀ (T : Theory) (κ : Cardinal), ∃ M : Model T, Saturated M κ
pub fn saturated_model_exists_ty() -> Expr {
    impl_pi(
        "T",
        cst("Theory"),
        impl_pi(
            "κ",
            cst("Cardinal"),
            app2(cst("ExistsSaturated"), bvar(1), bvar(0)),
        ),
    )
}
/// Monster model: a large saturated model containing all smaller models as elementary substructures.
/// MonsterModel : Theory → Structure
pub fn monster_model_ty() -> Expr {
    arrow(cst("Theory"), cst("Structure"))
}
/// Imaginaries: the Meq construction adding canonical parameters for all definable equivalences.
/// Meq : Theory → Theory
pub fn meq_ty() -> Expr {
    arrow(cst("Theory"), cst("Theory"))
}
/// Strongly minimal formula: a formula that defines a strongly minimal set.
/// StronglyMinimalFormula : Theory → FOFormula → Prop
pub fn strongly_minimal_formula_ty() -> Expr {
    arrow(cst("Theory"), arrow(cst("FOFormula"), prop()))
}
/// Pregeometry: a closure operator satisfying exchange and finite character.
/// Pregeometry : Structure → DefSet → Prop
pub fn pregeometry_ty() -> Expr {
    arrow(cst("Structure"), arrow(cst("DefSet"), prop()))
}
/// Independence theorem in simple theories: amalgamation over models.
/// IndependenceTheoremSimple : ∀ (T : Theory), Simple T → IndependenceTheorem T
pub fn independence_theorem_simple_ty() -> Expr {
    impl_pi(
        "T",
        cst("Theory"),
        arrow(
            app(cst("Simple"), bvar(0)),
            app(cst("IndependenceTheorem"), bvar(1)),
        ),
    )
}
/// Morley's theorem on categoricity and stability.
/// MorleyStabilityThm : ∀ T, OmegaCategorical T → Stable T
pub fn morley_stability_thm_ty() -> Expr {
    impl_pi(
        "T",
        cst("Theory"),
        arrow(
            app(cst("OmegaCategorical"), bvar(0)),
            app(cst("StableTheory"), bvar(1)),
        ),
    )
}
/// Forking calculus: monotonicity, base monotonicity, finite character.
/// ForkingCalculus : Theory → Prop
pub fn forking_calculus_ty() -> Expr {
    mt_ext_theory_prop()
}
/// Boolean combination of types: type of a tuple over a set.
/// TupleType : Theory → Nat → ElemSet → Type
pub fn tuple_type_ty() -> Expr {
    arrow(
        cst("Theory"),
        arrow(nat_ty(), arrow(cst("ElemSet"), type0())),
    )
}
/// Isolated type: a type isolated by a single formula.
/// IsolatedType : Theory → CompleteType → Prop
pub fn isolated_type_ty() -> Expr {
    arrow(cst("Theory"), arrow(cst("CompleteType"), prop()))
}
/// Non-isolated type: not isolated by any formula.
/// NonIsolatedType : Theory → CompleteType → Prop
pub fn non_isolated_type_ty() -> Expr {
    arrow(cst("Theory"), arrow(cst("CompleteType"), prop()))
}
/// dp-minimality: dp-rank = 1 (strongest NIP condition below strong dependence).
/// DpMinimalTheory : Theory → Prop
pub fn dp_minimal_theory_ty() -> Expr {
    mt_ext_theory_prop()
}
/// Coheir: non-forking extension preserving types over the base.
/// Coheir : Theory → CompleteType → ElemSet → Prop
pub fn coheir_ty() -> Expr {
    arrow(
        cst("Theory"),
        arrow(cst("CompleteType"), arrow(cst("ElemSet"), prop())),
    )
}
/// Extended stability spectrum.
/// ExtendedStabilitySpectrum : Theory → Cardinal → Cardinal → Prop
pub fn extended_stability_spectrum_ty() -> Expr {
    arrow(
        cst("Theory"),
        arrow(cst("Cardinal"), arrow(cst("Cardinal"), prop())),
    )
}
/// dp-rank cardinal value.
/// DpRankCardinal : Theory → Cardinal
pub fn dp_rank_cardinal_ty() -> Expr {
    mt_ext_theory_cardinal()
}
/// Register all extended model theory axioms into the kernel environment.
pub fn register_model_theory_extended(env: &mut Environment) -> Result<(), String> {
    let axioms: &[(&str, Expr)] = &[
        ("ValuedField", type0()),
        ("Field", type0()),
        ("OrderedAbelianGroup", type0()),
        ("Simple", arrow(cst("Theory"), prop())),
        ("ForkingSymmetric", arrow(cst("Theory"), prop())),
        ("NSOP1", arrow(cst("Theory"), prop())),
        ("KimForkingEqualsForking", arrow(cst("Theory"), prop())),
        (
            "AlmostAllPadicAgree",
            arrow(cst("FOFormula"), arrow(cst("Theory"), prop())),
        ),
        ("PadicIntegers", cst("Theory")),
        (
            "ExistsSaturated",
            arrow(cst("Theory"), arrow(cst("Cardinal"), prop())),
        ),
        ("StableTheory", arrow(cst("Theory"), prop())),
        ("IndependenceTheorem", arrow(cst("Theory"), prop())),
        ("u_rank", u_rank_ty()),
        ("lascar_rank", lascar_rank_ty()),
        ("shelah_main_gap", shelah_main_gap_ty()),
        ("classifiable_theory", classifiable_theory_ty()),
        ("ndop_theory", ndop_theory_ty()),
        ("notop_theory", notop_theory_ty()),
        ("simple_theory_forking", simple_theory_forking_ty()),
        ("nsop1_theory", nsop1_theory_ty()),
        ("ntp2_theory", ntp2_theory_ty()),
        ("kim_independence", kim_independence_ty()),
        ("kims_lemma", kims_lemma_ty()),
        ("beautiful_pair", beautiful_pair_ty()),
        ("pac_field", pac_field_ty()),
        ("existentially_closed", existentially_closed_ty()),
        ("amalgamation_property", amalgamation_property_ty()),
        ("joint_embedding_property", joint_embedding_property_ty()),
        ("dcf0_theory", dcf0_theory_ty()),
        ("ordered_field_theory", ordered_field_theory_ty()),
        ("valued_field_theory", valued_field_theory_ty()),
        ("residue_field", residue_field_ty()),
        ("value_group", value_group_ty()),
        ("henselian_field", henselian_field_ty()),
        ("ax_kochen", ax_kochen_ty()),
        (
            "elimination_of_imaginaries",
            elimination_of_imaginaries_ty(),
        ),
        ("geometric_theory", geometric_theory_ty()),
        ("strongly_dependent", strongly_dependent_ty()),
        ("bounded_theory", bounded_theory_ty()),
        ("saturated_model_exists", saturated_model_exists_ty()),
        ("monster_model", monster_model_ty()),
        ("meq", meq_ty()),
        ("strongly_minimal_formula", strongly_minimal_formula_ty()),
        ("pregeometry", pregeometry_ty()),
        (
            "independence_theorem_simple",
            independence_theorem_simple_ty(),
        ),
        ("morley_stability_thm", morley_stability_thm_ty()),
        ("forking_calculus", forking_calculus_ty()),
        ("tuple_type", tuple_type_ty()),
        ("isolated_type", isolated_type_ty()),
        ("non_isolated_type", non_isolated_type_ty()),
        ("dp_minimal_theory", dp_minimal_theory_ty()),
        ("coheir", coheir_ty()),
        (
            "extended_stability_spectrum",
            extended_stability_spectrum_ty(),
        ),
        ("dp_rank_cardinal", dp_rank_cardinal_ty()),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .map_err(|e| format!("Failed to add {}: {:?}", name, e))?;
    }
    Ok(())
}
