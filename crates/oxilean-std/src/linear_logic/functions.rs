//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    BiFormula, BoundedLinearLogic, CoherenceSpace, DialecticaTransform, GeometryOfInteraction,
    Heap, LinFormula, LinSequent, LinearFormula, LinearSequent, LinearTypeSystem, Link, LlFormula,
    LlGame, LlRule, PhaseSpace, ProofStructure, SepLogicTriple,
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
pub fn bool_ty() -> Expr {
    cst("Bool")
}
pub fn list_ty(elem: Expr) -> Expr {
    app(cst("List"), elem)
}
/// LinearFormula: the universe of linear logic formulas.
/// Type: Type
pub fn linear_formula_ty() -> Expr {
    type0()
}
/// Tensor product A ⊗ B.
/// Type: LinearFormula → LinearFormula → LinearFormula
pub fn tensor_ty() -> Expr {
    arrow(
        cst("LinearFormula"),
        arrow(cst("LinearFormula"), cst("LinearFormula")),
    )
}
/// Par connective A ⅋ B (multiplicative disjunction).
/// Type: LinearFormula → LinearFormula → LinearFormula
pub fn par_ty() -> Expr {
    arrow(
        cst("LinearFormula"),
        arrow(cst("LinearFormula"), cst("LinearFormula")),
    )
}
/// With connective A & B (additive conjunction).
/// Type: LinearFormula → LinearFormula → LinearFormula
pub fn with_ty() -> Expr {
    arrow(
        cst("LinearFormula"),
        arrow(cst("LinearFormula"), cst("LinearFormula")),
    )
}
/// Plus connective A ⊕ B (additive disjunction).
/// Type: LinearFormula → LinearFormula → LinearFormula
pub fn plus_ty() -> Expr {
    arrow(
        cst("LinearFormula"),
        arrow(cst("LinearFormula"), cst("LinearFormula")),
    )
}
/// Bang modality !A (of-course / exponential).
/// Type: LinearFormula → LinearFormula
pub fn bang_ty() -> Expr {
    arrow(cst("LinearFormula"), cst("LinearFormula"))
}
/// Why-not modality ?A (why-not / exponential dual).
/// Type: LinearFormula → LinearFormula
pub fn why_not_ty() -> Expr {
    arrow(cst("LinearFormula"), cst("LinearFormula"))
}
/// Linear negation A^⊥.
/// Type: LinearFormula → LinearFormula
pub fn linear_neg_ty() -> Expr {
    arrow(cst("LinearFormula"), cst("LinearFormula"))
}
/// Linear implication A ⊸ B = A^⊥ ⅋ B.
/// Type: LinearFormula → LinearFormula → LinearFormula
pub fn lollipop_ty() -> Expr {
    arrow(
        cst("LinearFormula"),
        arrow(cst("LinearFormula"), cst("LinearFormula")),
    )
}
/// Multiplicative unit 1 (unit for ⊗).
pub fn mult_unit_ty() -> Expr {
    cst("LinearFormula")
}
/// Additive unit ⊤ (unit for &).
pub fn add_top_ty() -> Expr {
    cst("LinearFormula")
}
/// Multiplicative bottom ⊥ (unit for ⅋).
pub fn mult_bot_ty() -> Expr {
    cst("LinearFormula")
}
/// Additive zero 0 (unit for ⊕).
pub fn add_zero_ty() -> Expr {
    cst("LinearFormula")
}
/// LinearContext: a multiset of linear formulas.
/// Type: Type
pub fn linear_context_ty() -> Expr {
    type0()
}
/// LinearSequent: a one-sided sequent ⊢ Γ in linear logic.
/// Type: LinearContext → Prop
pub fn linear_sequent_ty() -> Expr {
    arrow(cst("LinearContext"), prop())
}
/// ProvableLL: ⊢ Γ is provable in full linear logic.
/// Type: LinearContext → Prop
pub fn provable_ll_ty() -> Expr {
    arrow(cst("LinearContext"), prop())
}
/// ProvableMLL: provable in the multiplicative fragment.
/// Type: LinearContext → Prop
pub fn provable_mll_ty() -> Expr {
    arrow(cst("LinearContext"), prop())
}
/// ProvableMALL: provable in the multiplicative-additive fragment.
/// Type: LinearContext → Prop
pub fn provable_mall_ty() -> Expr {
    arrow(cst("LinearContext"), prop())
}
/// CutElimLL: cut is admissible in linear logic.
/// Type: ∀ (Γ : LinearContext), ProvableLL Γ (with cut) → ProvableLL Γ (cut-free)
pub fn cut_elim_ll_ty() -> Expr {
    impl_pi(
        "gamma",
        cst("LinearContext"),
        arrow(
            app(cst("ProvableLL"), bvar(0)),
            app(cst("ProvableLL"), bvar(1)),
        ),
    )
}
/// ExchangeRule: formulas in a sequent can be permuted.
/// Type: ∀ (Γ Δ : LinearContext), Permutation Γ Δ → ProvableLL Γ → ProvableLL Δ
pub fn exchange_rule_ty() -> Expr {
    impl_pi(
        "gamma",
        cst("LinearContext"),
        impl_pi(
            "delta",
            cst("LinearContext"),
            arrow(
                app2(cst("Permutation"), bvar(1), bvar(0)),
                arrow(
                    app(cst("ProvableLL"), bvar(2)),
                    app(cst("ProvableLL"), bvar(2)),
                ),
            ),
        ),
    )
}
/// TensorIntro: from ⊢ Γ, A and ⊢ Δ, B derive ⊢ Γ, Δ, A ⊗ B.
pub fn tensor_intro_ty() -> Expr {
    arrow(
        cst("LinearContext"),
        arrow(
            cst("LinearContext"),
            arrow(
                cst("LinearFormula"),
                arrow(
                    cst("LinearFormula"),
                    arrow(
                        app(cst("ProvableLL"), bvar(3)),
                        arrow(
                            app(cst("ProvableLL"), bvar(3)),
                            app(cst("ProvableLL"), bvar(4)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// BangIntro: from ⊢ !Γ, A derive ⊢ !Γ, !A.
pub fn bang_intro_ty() -> Expr {
    arrow(
        cst("LinearContext"),
        arrow(
            cst("LinearFormula"),
            arrow(
                app(cst("ProvableLL"), bvar(1)),
                app(cst("ProvableLL"), bvar(2)),
            ),
        ),
    )
}
/// Dereliction: from ⊢ Γ, A derive ⊢ Γ, !A (weakening for !).
pub fn dereliction_ty() -> Expr {
    arrow(
        cst("LinearFormula"),
        arrow(
            app(cst("ProvableLL"), cst("LinearContext")),
            app(cst("ProvableLL"), cst("LinearContext")),
        ),
    )
}
/// ProofStructure: a graph-based presentation of a proof.
/// Type: Type
pub fn proof_structure_ty() -> Expr {
    type0()
}
/// IsCorrectProofNet: satisfies the Danos-Regnier criterion.
/// Type: ProofStructure → Prop
pub fn is_correct_proof_net_ty() -> Expr {
    arrow(cst("ProofStructure"), prop())
}
/// ProofNetToSequent: every correct proof net corresponds to a sequent proof.
/// Type: ∀ (ps : ProofStructure), IsCorrectProofNet ps → ProvableMLL (conclusion ps)
pub fn proof_net_to_sequent_ty() -> Expr {
    impl_pi(
        "ps",
        cst("ProofStructure"),
        arrow(
            app(cst("IsCorrectProofNet"), bvar(0)),
            app(cst("ProvableMLL"), app(cst("ConclusionOf"), bvar(1))),
        ),
    )
}
/// SequentToProofNet: every MLL proof has a corresponding proof net.
pub fn sequent_to_proof_net_ty() -> Expr {
    impl_pi(
        "gamma",
        cst("LinearContext"),
        arrow(
            app(cst("ProvableMLL"), bvar(0)),
            app2(
                cst("Exists"),
                cst("ProofStructure"),
                cst("IsCorrectProofNet"),
            ),
        ),
    )
}
/// PhaseSpace: a commutative monoid (M, ·, 1) with a subset ⊥ ⊆ M.
/// Type: Type
pub fn phase_space_ty() -> Expr {
    type0()
}
/// Fact: a ⊥-closed subset of a phase space.
/// Type: PhaseSpace → Set PhaseSpace → Prop
pub fn fact_ty() -> Expr {
    arrow(
        cst("PhaseSpace"),
        arrow(app(cst("Set"), cst("PhaseSpace")), prop()),
    )
}
/// PhaseSemanticsValid: all theorems of LL hold in all phase spaces.
/// Type: LinearFormula → Prop
pub fn phase_semantics_valid_ty() -> Expr {
    arrow(cst("LinearFormula"), prop())
}
/// PhaseCompleteness: LL is complete for phase semantics.
/// Type: ∀ (A : LinearFormula), PhaseSemanticsValid A → ProvableLL (singleton A)
pub fn phase_completeness_ty() -> Expr {
    impl_pi(
        "a",
        cst("LinearFormula"),
        arrow(
            app(cst("PhaseSemanticsValid"), bvar(0)),
            app(cst("ProvableLL"), app(cst("SingletonCtx"), bvar(1))),
        ),
    )
}
/// CoherenceSpace: a reflexive, symmetric binary relation on a set (web).
/// Type: Type
pub fn coherence_space_ty() -> Expr {
    type0()
}
/// WebOf: the underlying token set of a coherence space.
/// Type: CoherenceSpace → Type
pub fn web_of_ty() -> Expr {
    arrow(cst("CoherenceSpace"), type0())
}
/// Clique: a set of mutually coherent tokens.
/// Type: CoherenceSpace → Set (WebOf cs) → Prop
pub fn clique_ty() -> Expr {
    arrow(cst("CoherenceSpace"), arrow(type0(), prop()))
}
/// CoherenceTensor: tensor product of two coherence spaces.
/// Type: CoherenceSpace → CoherenceSpace → CoherenceSpace
pub fn coherence_tensor_ty() -> Expr {
    arrow(
        cst("CoherenceSpace"),
        arrow(cst("CoherenceSpace"), cst("CoherenceSpace")),
    )
}
/// CoherenceLinearMap: a stable map between coherence spaces (linear morphism).
/// Type: CoherenceSpace → CoherenceSpace → Type
pub fn coherence_linear_map_ty() -> Expr {
    arrow(cst("CoherenceSpace"), arrow(cst("CoherenceSpace"), type0()))
}
/// Arena: a game arena (moves, polarity, justification pointers).
/// Type: Type
pub fn arena_ty() -> Expr {
    type0()
}
/// Position: a valid play prefix in an arena.
/// Type: Arena → Type
pub fn position_ty() -> Expr {
    arrow(cst("Arena"), type0())
}
/// Strategy: an even-length closed prefix-closed set of positions.
/// Type: Arena → Type
pub fn strategy_ty() -> Expr {
    arrow(cst("Arena"), type0())
}
/// InnocentStrategy: a strategy determined by its view.
/// Type: Strategy → Prop
pub fn innocent_strategy_ty() -> Expr {
    arrow(cst("Strategy"), prop())
}
/// WinningStrategy: a strategy that beats all opponent strategies.
/// Type: Strategy → Prop
pub fn winning_strategy_ty() -> Expr {
    arrow(cst("Strategy"), prop())
}
/// GameCompose: sequential composition of strategies.
/// Type: Strategy A B → Strategy B C → Strategy A C
pub fn game_compose_ty() -> Expr {
    arrow(cst("Strategy"), arrow(cst("Strategy"), cst("Strategy")))
}
/// GameSemanticsSoundness: every LL proof yields a winning innocent strategy.
pub fn game_semantics_soundness_ty() -> Expr {
    impl_pi(
        "gamma",
        cst("LinearContext"),
        arrow(
            app(cst("ProvableLL"), bvar(0)),
            app2(
                cst("Exists"),
                cst("Strategy"),
                app2(cst("And"), cst("InnocentStrategy"), cst("WinningStrategy")),
            ),
        ),
    )
}
/// RelevantFormula: formulas in relevant logic (contracting, no weakening).
pub fn relevant_formula_ty() -> Expr {
    type0()
}
/// ProvableR: provable in Anderson-Belnap relevant logic R.
pub fn provable_r_ty() -> Expr {
    arrow(cst("RelevantFormula"), prop())
}
/// ContractionRule: A → A → B implies A → B (relevant but not affine).
pub fn contraction_rule_ty() -> Expr {
    impl_pi(
        "a",
        cst("RelevantFormula"),
        impl_pi(
            "b",
            cst("RelevantFormula"),
            arrow(
                app2(
                    cst("ProvableR"),
                    app2(cst("Arr"), bvar(1), app2(cst("Arr"), bvar(2), bvar(1))),
                    bvar(0),
                ),
                app2(
                    cst("ProvableR"),
                    app2(cst("Arr"), bvar(2), bvar(1)),
                    bvar(1),
                ),
            ),
        ),
    )
}
/// AffineFormula: formulas in affine logic (weakening, no contraction).
pub fn affine_formula_ty() -> Expr {
    type0()
}
/// ProvableAff: provable in affine logic.
pub fn provable_aff_ty() -> Expr {
    arrow(cst("AffineFormula"), prop())
}
/// WeakeningRule: from ⊢ Γ derive ⊢ Γ, A (affine).
pub fn weakening_rule_ty() -> Expr {
    impl_pi(
        "gamma",
        cst("LinearContext"),
        impl_pi(
            "a",
            cst("LinearFormula"),
            arrow(
                app(cst("ProvableLL"), bvar(1)),
                app(cst("ProvableLL"), bvar(2)),
            ),
        ),
    )
}
/// BIFormula: a formula in the logic of bunched implications.
pub fn bi_formula_ty() -> Expr {
    type0()
}
/// SepConj: separating conjunction P * Q (multiplicative).
pub fn sep_conj_ty() -> Expr {
    arrow(cst("BIFormula"), arrow(cst("BIFormula"), cst("BIFormula")))
}
/// SepImpl: magic wand P -* Q (multiplicative implication).
pub fn sep_impl_ty() -> Expr {
    arrow(cst("BIFormula"), arrow(cst("BIFormula"), cst("BIFormula")))
}
/// Heap: the resource model for separation logic.
pub fn heap_ty() -> Expr {
    type0()
}
/// DisjointHeaps: two heaps with disjoint domains.
pub fn disjoint_heaps_ty() -> Expr {
    arrow(cst("Heap"), arrow(cst("Heap"), prop()))
}
/// SepConjSemantics: h ⊨ P * Q iff h = h1 ∪ h2, h1 ⊨ P, h2 ⊨ Q.
pub fn sep_conj_semantics_ty() -> Expr {
    impl_pi(
        "h",
        cst("Heap"),
        impl_pi(
            "p",
            cst("BIFormula"),
            impl_pi(
                "q",
                cst("BIFormula"),
                app2(
                    cst("Iff"),
                    app2(
                        cst("Satisfies"),
                        bvar(2),
                        app2(cst("SepConj"), bvar(1), bvar(0)),
                    ),
                    app2(cst("Exists"), cst("Heap"), cst("DisjointHeaps")),
                ),
            ),
        ),
    )
}
/// FrameRule: {P} C {Q} → {P * R} C {Q * R}.
pub fn frame_rule_ty() -> Expr {
    arrow(
        cst("BIFormula"),
        arrow(
            cst("BIFormula"),
            arrow(
                cst("BIFormula"),
                arrow(
                    app3(cst("Hoare"), bvar(2), cst("Command"), bvar(1)),
                    app3(
                        cst("Hoare"),
                        app2(cst("SepConj"), bvar(3), bvar(0)),
                        cst("Command"),
                        app2(cst("SepConj"), bvar(2), bvar(1)),
                    ),
                ),
            ),
        ),
    )
}
/// Register all linear logic axioms into the kernel environment.
pub fn build_linear_logic_env(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("LinearFormula", linear_formula_ty()),
        ("Tensor", tensor_ty()),
        ("Par", par_ty()),
        ("With", with_ty()),
        ("Plus", plus_ty()),
        ("Bang", bang_ty()),
        ("WhyNot", why_not_ty()),
        ("LinearNeg", linear_neg_ty()),
        ("Lollipop", lollipop_ty()),
        ("MultUnit", mult_unit_ty()),
        ("AddTop", add_top_ty()),
        ("MultBot", mult_bot_ty()),
        ("AddZero", add_zero_ty()),
        ("LinearContext", linear_context_ty()),
        ("LinearSequent", linear_sequent_ty()),
        ("ProvableLL", provable_ll_ty()),
        ("ProvableMLL", provable_mll_ty()),
        ("ProvableMALL", provable_mall_ty()),
        ("CutElimLL", cut_elim_ll_ty()),
        ("ExchangeRule", exchange_rule_ty()),
        ("TensorIntro", tensor_intro_ty()),
        ("BangIntro", bang_intro_ty()),
        ("Dereliction", dereliction_ty()),
        (
            "Permutation",
            arrow(cst("LinearContext"), arrow(cst("LinearContext"), prop())),
        ),
        (
            "SingletonCtx",
            arrow(cst("LinearFormula"), cst("LinearContext")),
        ),
        (
            "ConclusionOf",
            arrow(cst("ProofStructure"), cst("LinearContext")),
        ),
        ("ProofStructure", proof_structure_ty()),
        ("IsCorrectProofNet", is_correct_proof_net_ty()),
        ("ProofNetToSequent", proof_net_to_sequent_ty()),
        ("SequentToProofNet", sequent_to_proof_net_ty()),
        ("PhaseSpace", phase_space_ty()),
        ("Fact", fact_ty()),
        ("PhaseSemanticsValid", phase_semantics_valid_ty()),
        ("PhaseCompleteness", phase_completeness_ty()),
        ("CoherenceSpace", coherence_space_ty()),
        ("WebOf", web_of_ty()),
        ("Clique", clique_ty()),
        ("CoherenceTensor", coherence_tensor_ty()),
        ("CoherenceLinearMap", coherence_linear_map_ty()),
        ("Arena", arena_ty()),
        ("Position", position_ty()),
        ("Strategy", strategy_ty()),
        ("InnocentStrategy", innocent_strategy_ty()),
        ("WinningStrategy", winning_strategy_ty()),
        ("GameCompose", game_compose_ty()),
        ("GameSemanticsSoundness", game_semantics_soundness_ty()),
        ("RelevantFormula", relevant_formula_ty()),
        ("ProvableR", provable_r_ty()),
        ("ContractionRule", contraction_rule_ty()),
        ("AffineFormula", affine_formula_ty()),
        ("ProvableAff", provable_aff_ty()),
        ("WeakeningRule", weakening_rule_ty()),
        (
            "Arr",
            arrow(
                cst("RelevantFormula"),
                arrow(cst("RelevantFormula"), cst("RelevantFormula")),
            ),
        ),
        ("BIFormula", bi_formula_ty()),
        ("SepConj", sep_conj_ty()),
        ("SepImpl", sep_impl_ty()),
        ("Heap", heap_ty()),
        ("DisjointHeaps", disjoint_heaps_ty()),
        (
            "Satisfies",
            arrow(cst("Heap"), arrow(cst("BIFormula"), prop())),
        ),
        (
            "Hoare",
            arrow(
                cst("BIFormula"),
                arrow(cst("Command"), arrow(cst("BIFormula"), prop())),
            ),
        ),
        ("Command", type0()),
        ("SepConjSemantics", sep_conj_semantics_ty()),
        ("FrameRule", frame_rule_ty()),
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
    fn test_linear_formula_dual_involutive() {
        let a = LinFormula::Atom("A".to_string());
        let b = LinFormula::Tensor(
            Box::new(LinFormula::Atom("A".to_string())),
            Box::new(LinFormula::Atom("B".to_string())),
        );
        assert_eq!(a.dual().dual(), a);
        assert!(matches!(b.dual(), LinFormula::Par(_, _)));
        assert_eq!(b.dual().dual(), b);
    }
    #[test]
    fn test_linear_formula_lollipop() {
        let a = LinFormula::Atom("A".to_string());
        let b = LinFormula::Atom("B".to_string());
        let lol = LinFormula::lollipop(a, b);
        assert!(matches!(lol, LinFormula::Par(_, _)));
    }
    #[test]
    fn test_linear_formula_complexity() {
        let a = LinFormula::Atom("A".to_string());
        let b = LinFormula::Atom("B".to_string());
        let t = LinFormula::Tensor(Box::new(a), Box::new(b));
        assert_eq!(t.complexity(), 1);
        let bang_t = LinFormula::Bang(Box::new(t.clone()));
        assert_eq!(bang_t.complexity(), 2);
    }
    #[test]
    fn test_sequent_is_axiom() {
        let a = LinFormula::Atom("A".to_string());
        let seq = LinSequent::new(vec![a.clone(), a.dual()]);
        assert!(seq.is_axiom());
        let b = LinFormula::Atom("B".to_string());
        let seq2 = LinSequent::new(vec![a, b]);
        assert!(!seq2.is_axiom());
    }
    #[test]
    fn test_proof_structure_correct() {
        let mut ps = ProofStructure::new(2);
        ps.add_link(Link::axiom(0, 1));
        assert!(ps.is_correct());
    }
    #[test]
    fn test_proof_structure_incorrect_cycle() {
        let mut ps = ProofStructure::new(4);
        ps.add_link(Link::axiom(0, 1));
        assert!(!ps.is_correct());
    }
    #[test]
    fn test_coherence_space_clique() {
        let cs = CoherenceSpace::complete(3);
        assert!(cs.is_clique(&[0, 1, 2]));
        let cs_flat = CoherenceSpace::flat(3);
        assert!(cs_flat.is_clique(&[0]));
        assert!(!cs_flat.is_clique(&[0, 1]));
    }
    #[test]
    fn test_phase_space_trivial() {
        let ps = PhaseSpace::trivial();
        let all = vec![true];
        assert!(ps.is_fact(&all));
    }
    #[test]
    fn test_heap_sep_split() {
        let h = Heap::singleton(1, 10)
            .union(&Heap::singleton(2, 20))
            .expect("union should succeed");
        let result = h.sep_split(|h1| h1.read(1) == Some(10), |h2| h2.read(2) == Some(20));
        assert!(result.is_some());
    }
    #[test]
    fn test_build_linear_logic_env() {
        let mut env = Environment::new();
        build_linear_logic_env(&mut env);
        assert!(env.get(&Name::str("LinearFormula")).is_some());
        assert!(env.get(&Name::str("Tensor")).is_some());
        assert!(env.get(&Name::str("Bang")).is_some());
        assert!(env.get(&Name::str("SepConj")).is_some());
        assert!(env.get(&Name::str("CoherenceSpace")).is_some());
    }
}
/// Build the public-API `build_env` function (alias for `build_linear_logic_env`).
pub fn build_env(env: &mut oxilean_kernel::Environment) {
    build_linear_logic_env(env);
}
#[cfg(test)]
mod ll_ext_tests {
    use super::*;
    #[test]
    fn test_ll_formula_negation() {
        let a = LlFormula::atom("A");
        let neg_a = a.linear_negation();
        assert!(matches!(neg_a, LlFormula::Neg(_)));
        let one = LlFormula::One;
        assert_eq!(one.linear_negation(), LlFormula::Bottom);
    }
    #[test]
    fn test_ll_formula_types() {
        let a = LlFormula::tensor(LlFormula::atom("A"), LlFormula::atom("B"));
        assert!(a.is_multiplicative());
        let b = LlFormula::with_op(LlFormula::atom("A"), LlFormula::atom("B"));
        assert!(b.is_additive());
        let c = LlFormula::of_course(LlFormula::atom("A"));
        assert!(c.is_exponential());
    }
    #[test]
    fn test_ll_rules() {
        let rules = [
            LlRule::Ax,
            LlRule::Cut,
            LlRule::TensorR,
            LlRule::ParR,
            LlRule::Contraction,
            LlRule::Weakening,
        ];
        for r in &rules {
            assert!(!r.name().is_empty());
        }
        assert!(LlRule::Contraction.is_structural());
        assert!(!LlRule::TensorR.is_structural());
    }
    #[test]
    fn test_linear_type_system() {
        let rust = LinearTypeSystem::rust_ownership();
        assert!(rust.prevents_use_after_free());
        assert!(rust.affine_types);
    }
    #[test]
    fn test_phase_semantics() {
        let ps = PhaseSpace::new("(M, *)");
        assert!(!ps.completeness_description().is_empty());
    }
}
#[cfg(test)]
mod ll_game_tests {
    use super::*;
    #[test]
    fn test_ll_game() {
        let g = LlGame::new("A tensor B");
        assert!(!g.abramsky_jagadeesan_description().is_empty());
    }
    #[test]
    fn test_goi() {
        let goi = GeometryOfInteraction::girard_goi();
        assert!(!goi.dynamic_description().is_empty());
    }
}
#[cfg(test)]
mod sep_logic_tests {
    use super::*;
    #[test]
    fn test_bi_formula() {
        let emp = BiFormula::Emp;
        assert!(emp.is_separation_connective());
        let sc = BiFormula::sep_conj(BiFormula::atom("P"), BiFormula::atom("Q"));
        assert!(sc.is_separation_connective());
    }
    #[test]
    fn test_separation_logic() {
        let triple = SepLogicTriple::new("x -> 3", "*x := 5", "x -> 5");
        let framed = triple.frame_rule("y -> 7");
        assert!(framed.precondition.contains("y -> 7"));
        assert!(!triple.display().is_empty());
    }
}
#[cfg(test)]
mod dialectica_tests {
    use super::*;
    #[test]
    fn test_dialectica() {
        let dt = DialecticaTransform::new("A -> B", "A_d -> B_d");
        assert!(!dt.de_paiva_description().is_empty());
    }
    #[test]
    fn test_bll() {
        let bll = BoundedLinearLogic::new("n^2");
        assert!(bll.captures_ptime());
        assert!(!bll.description().is_empty());
    }
}
