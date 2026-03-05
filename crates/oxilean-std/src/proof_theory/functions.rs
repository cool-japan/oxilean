//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    Clause, Combinator, CutEliminator, Formula, HerbrandInstance, HerbrandInstanceGenerator,
    HerbrandTerm, LKNode, LKRule, NDTerm, ResolutionProver, Sequent, SequentCalculusProof,
};

pub(super) fn app(f: Expr, a: Expr) -> Expr {
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
/// Sequent type: Γ ⊢ Δ
pub fn sequent_ty() -> Expr {
    type0()
}
/// Proof derivation type
pub fn derivation_ty() -> Expr {
    arrow(sequent_ty(), type0())
}
/// Cut rule: Γ⊢A,Δ and Γ',A⊢Δ' → Γ,Γ'⊢Δ,Δ'
pub fn cut_rule_ty() -> Expr {
    let seq = sequent_ty();
    arrow(seq.clone(), arrow(seq.clone(), arrow(seq, prop())))
}
/// Natural deduction system type
pub fn natural_deduction_ty() -> Expr {
    type0()
}
/// Typed lambda term (under Curry-Howard)
pub fn lambda_term_ty() -> Expr {
    arrow(type0(), type0())
}
/// Beta-normal form type
pub fn normal_form_ty() -> Expr {
    arrow(type0(), prop())
}
/// Gentzen's Hauptsatz: cut-free provability
/// CutElimination : ∀ (seq : Sequent), Derivable seq → CutFreeDer seq
pub fn cut_elimination_ty() -> Expr {
    let seq = cst("Sequent");
    impl_pi(
        "seq",
        seq.clone(),
        arrow(
            app(cst("Derivable"), bvar(0)),
            app(cst("CutFreeDerivable"), bvar(1)),
        ),
    )
}
/// Cut-free proofs use only subformulas
/// SubformulaProperty : ∀ (seq : Sequent), CutFreeDerivable seq → SubformulasClosed seq
pub fn subformula_property_ty() -> Expr {
    let seq = cst("Sequent");
    impl_pi(
        "seq",
        seq.clone(),
        arrow(
            app(cst("CutFreeDerivable"), bvar(0)),
            app(cst("SubformulasClosed"), bvar(1)),
        ),
    )
}
/// Strong normalization for STLC
/// Normalization : ∀ (t : LambdaTerm), WellTyped t → StronglyNormalizing t
pub fn normalization_ty() -> Expr {
    impl_pi(
        "t",
        cst("LambdaTerm"),
        arrow(
            app(cst("WellTyped"), bvar(0)),
            app(cst("StronglyNormalizing"), bvar(1)),
        ),
    )
}
/// Consistency: ⊬ False
/// Consistency : ¬ Provable False
pub fn consistency_ty() -> Expr {
    arrow(app(cst("Provable"), cst("FalseFormula")), cst("False"))
}
/// Completeness of propositional calculus
/// CompletenessPC : ∀ (f : Formula), Tautology f → Provable f
pub fn completeness_propositional_ty() -> Expr {
    impl_pi(
        "f",
        cst("Formula"),
        arrow(
            app(cst("Tautology"), bvar(0)),
            app(cst("Provable"), bvar(1)),
        ),
    )
}
/// Curry-Howard: propositions = types, proofs = programs
/// CurryHoward : ∀ (P : Prop), Proof P ↔ (∃ t : LambdaTerm, HasType t P)
pub fn curry_howard_ty() -> Expr {
    impl_pi(
        "P",
        prop(),
        app2(
            cst("Iff"),
            app(cst("Proof"), bvar(0)),
            app2(
                cst("Exists"),
                cst("LambdaTerm"),
                app(cst("HasType"), bvar(1)),
            ),
        ),
    )
}
/// LK system: classical sequent calculus type
/// LKDerivable : Sequent → Prop
pub fn lk_derivable_ty() -> Expr {
    arrow(cst("Sequent"), prop())
}
/// LJ system: intuitionistic sequent calculus (single-succedent sequents)
/// LJDerivable : Sequent → Prop
pub fn lj_derivable_ty() -> Expr {
    arrow(cst("Sequent"), prop())
}
/// LK init rule: A ⊢ A is derivable in LK
/// LKInit : ∀ (A : Formula), LKDerivable (AxiomSeq A)
pub fn lk_init_ty() -> Expr {
    impl_pi(
        "A",
        cst("Formula"),
        app(cst("LKDerivable"), app(cst("AxiomSeq"), bvar(0))),
    )
}
/// LK left-contraction: Γ,A,A ⊢ Δ → Γ,A ⊢ Δ
/// LKLeftContraction : ∀ (s t : Sequent), LKDerivable s → LKDerivable t
pub fn lk_left_contraction_ty() -> Expr {
    impl_pi(
        "s",
        cst("Sequent"),
        impl_pi(
            "t",
            cst("Sequent"),
            arrow(
                app(cst("LKDerivable"), bvar(1)),
                app(cst("LKDerivable"), bvar(1)),
            ),
        ),
    )
}
/// LK right-weakening: Γ ⊢ Δ → Γ ⊢ Δ,A
/// LKRightWeakening : ∀ (s : Sequent) (A : Formula), LKDerivable s → LKDerivable (WeakenRight s A)
pub fn lk_right_weakening_ty() -> Expr {
    impl_pi(
        "s",
        cst("Sequent"),
        impl_pi(
            "A",
            cst("Formula"),
            arrow(
                app(cst("LKDerivable"), bvar(1)),
                app(
                    cst("LKDerivable"),
                    app2(cst("WeakenRight"), bvar(2), bvar(0)),
                ),
            ),
        ),
    )
}
/// LK left-conjunction rule: Γ,A ⊢ Δ → Γ,A∧B ⊢ Δ
/// LKAndLeft1 : ∀ (s t : Sequent), LKDerivable s → LKDerivable t
pub fn lk_and_left1_ty() -> Expr {
    impl_pi(
        "s",
        cst("Sequent"),
        impl_pi(
            "t",
            cst("Sequent"),
            arrow(
                app(cst("LKDerivable"), bvar(1)),
                app(cst("LKDerivable"), bvar(1)),
            ),
        ),
    )
}
/// LK right-conjunction rule: Γ ⊢ A,Δ and Γ ⊢ B,Δ → Γ ⊢ A∧B,Δ
/// LKAndRight : ∀ (s1 s2 s3 : Sequent), LKDerivable s1 → LKDerivable s2 → LKDerivable s3
pub fn lk_and_right_ty() -> Expr {
    impl_pi(
        "s1",
        cst("Sequent"),
        impl_pi(
            "s2",
            cst("Sequent"),
            impl_pi(
                "s3",
                cst("Sequent"),
                arrow(
                    app(cst("LKDerivable"), bvar(2)),
                    arrow(
                        app(cst("LKDerivable"), bvar(2)),
                        app(cst("LKDerivable"), bvar(1)),
                    ),
                ),
            ),
        ),
    )
}
/// LK cut rule (explicit): Γ ⊢ A,Δ and Γ',A ⊢ Δ' → Γ,Γ' ⊢ Δ,Δ'
/// LKCut : ∀ (s1 s2 s3 : Sequent) (A : Formula), LKDerivable s1 → LKDerivable s2 → LKDerivable s3
pub fn lk_cut_ty() -> Expr {
    impl_pi(
        "s1",
        cst("Sequent"),
        impl_pi(
            "s2",
            cst("Sequent"),
            impl_pi(
                "s3",
                cst("Sequent"),
                impl_pi(
                    "A",
                    cst("Formula"),
                    arrow(
                        app(cst("LKDerivable"), bvar(3)),
                        arrow(
                            app(cst("LKDerivable"), bvar(3)),
                            app(cst("LKDerivable"), bvar(2)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// LJ left-implication: Γ,A→B,Γ' ⊢ A and Γ,B,Γ' ⊢ C → Γ,A→B,Γ' ⊢ C
/// LJImpLeft : ∀ (s1 s2 s3 : Sequent), LJDerivable s1 → LJDerivable s2 → LJDerivable s3
pub fn lj_imp_left_ty() -> Expr {
    impl_pi(
        "s1",
        cst("Sequent"),
        impl_pi(
            "s2",
            cst("Sequent"),
            impl_pi(
                "s3",
                cst("Sequent"),
                arrow(
                    app(cst("LJDerivable"), bvar(2)),
                    arrow(
                        app(cst("LJDerivable"), bvar(2)),
                        app(cst("LJDerivable"), bvar(1)),
                    ),
                ),
            ),
        ),
    )
}
/// LJ right-implication: Γ,A ⊢ B → Γ ⊢ A→B
/// LJImpRight : ∀ (s t : Sequent), LJDerivable s → LJDerivable t
pub fn lj_imp_right_ty() -> Expr {
    impl_pi(
        "s",
        cst("Sequent"),
        impl_pi(
            "t",
            cst("Sequent"),
            arrow(
                app(cst("LJDerivable"), bvar(1)),
                app(cst("LJDerivable"), bvar(1)),
            ),
        ),
    )
}
/// Natural deduction: →-introduction (deduction theorem)
/// NDImpIntro : ∀ (A B : Formula), NDDerivable (cons A empty) B → NDDerivable empty (imp A B)
pub fn nd_imp_intro_ty() -> Expr {
    impl_pi(
        "A",
        cst("Formula"),
        impl_pi(
            "B",
            cst("Formula"),
            arrow(
                app2(
                    cst("NDDerivable"),
                    app2(cst("Cons"), bvar(1), cst("EmptyCtx")),
                    bvar(1),
                ),
                app2(
                    cst("NDDerivable"),
                    cst("EmptyCtx"),
                    app2(cst("ImpForm"), bvar(2), bvar(1)),
                ),
            ),
        ),
    )
}
/// Natural deduction: →-elimination (modus ponens)
/// NDImpElim : ∀ (Γ : Context) (A B : Formula),
///   NDDerivable Γ (imp A B) → NDDerivable Γ A → NDDerivable Γ B
pub fn nd_imp_elim_ty() -> Expr {
    impl_pi(
        "ctx",
        cst("Context"),
        impl_pi(
            "A",
            cst("Formula"),
            impl_pi(
                "B",
                cst("Formula"),
                arrow(
                    app2(
                        cst("NDDerivable"),
                        bvar(2),
                        app2(cst("ImpForm"), bvar(1), bvar(0)),
                    ),
                    arrow(
                        app2(cst("NDDerivable"), bvar(3), bvar(2)),
                        app2(cst("NDDerivable"), bvar(4), bvar(2)),
                    ),
                ),
            ),
        ),
    )
}
/// Natural deduction: ∧-introduction
/// NDAndIntro : ∀ (Γ : Context) (A B : Formula),
///   NDDerivable Γ A → NDDerivable Γ B → NDDerivable Γ (and A B)
pub fn nd_and_intro_ty() -> Expr {
    impl_pi(
        "ctx",
        cst("Context"),
        impl_pi(
            "A",
            cst("Formula"),
            impl_pi(
                "B",
                cst("Formula"),
                arrow(
                    app2(cst("NDDerivable"), bvar(2), bvar(1)),
                    arrow(
                        app2(cst("NDDerivable"), bvar(3), bvar(1)),
                        app2(
                            cst("NDDerivable"),
                            bvar(4),
                            app2(cst("AndForm"), bvar(2), bvar(1)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// Natural deduction: ∧-elimination left
/// NDAndElimLeft : ∀ (Γ : Context) (A B : Formula),
///   NDDerivable Γ (and A B) → NDDerivable Γ A
pub fn nd_and_elim_left_ty() -> Expr {
    impl_pi(
        "ctx",
        cst("Context"),
        impl_pi(
            "A",
            cst("Formula"),
            impl_pi(
                "B",
                cst("Formula"),
                arrow(
                    app2(
                        cst("NDDerivable"),
                        bvar(2),
                        app2(cst("AndForm"), bvar(1), bvar(0)),
                    ),
                    app2(cst("NDDerivable"), bvar(3), bvar(2)),
                ),
            ),
        ),
    )
}
/// Natural deduction: ∨-introduction left
/// NDOrIntroLeft : ∀ (Γ : Context) (A B : Formula),
///   NDDerivable Γ A → NDDerivable Γ (or A B)
pub fn nd_or_intro_left_ty() -> Expr {
    impl_pi(
        "ctx",
        cst("Context"),
        impl_pi(
            "A",
            cst("Formula"),
            impl_pi(
                "B",
                cst("Formula"),
                arrow(
                    app2(cst("NDDerivable"), bvar(2), bvar(1)),
                    app2(
                        cst("NDDerivable"),
                        bvar(3),
                        app2(cst("OrForm"), bvar(2), bvar(1)),
                    ),
                ),
            ),
        ),
    )
}
/// Ordinal type
pub fn ordinal_ty() -> Expr {
    type0()
}
/// Epsilon-zero: ε₀ = ωᵒᵐᵉᵍᵃ^ω (the proof-theoretic ordinal of PA)
/// EpsilonZero : Ordinal
pub fn epsilon_zero_ty() -> Expr {
    cst("Ordinal")
}
/// Proof-theoretic ordinal of a system S: |S| : Ordinal
/// ProofTheoreticOrdinal : ProofSystem → Ordinal
pub fn proof_theoretic_ordinal_ty() -> Expr {
    arrow(cst("ProofSystem"), cst("Ordinal"))
}
/// Ordinal of PA equals ε₀: |PA| = ε₀
/// PAOrdinalEpsilonZero : Eq Ordinal (ProofTheoreticOrdinal PA) EpsilonZero
pub fn pa_ordinal_epsilon_zero_ty() -> Expr {
    app3(
        cst("Eq"),
        cst("Ordinal"),
        app(cst("ProofTheoreticOrdinal"), cst("PA")),
        cst("EpsilonZero"),
    )
}
/// Ordinal induction up to ε₀
/// OrdinalInductionEpsilonZero :
///   ∀ (P : Ordinal → Prop),
///     (∀ α, (∀ β < α, P β) → P α) →
///     ∀ α < EpsilonZero, P α
pub fn ordinal_induction_epsilon_zero_ty() -> Expr {
    impl_pi(
        "P",
        arrow(cst("Ordinal"), prop()),
        arrow(
            impl_pi(
                "alpha",
                cst("Ordinal"),
                arrow(
                    impl_pi(
                        "beta",
                        cst("Ordinal"),
                        arrow(app2(cst("OrdLt"), bvar(0), bvar(1)), app(bvar(3), bvar(1))),
                    ),
                    app(bvar(2), bvar(1)),
                ),
            ),
            impl_pi(
                "alpha",
                cst("Ordinal"),
                arrow(
                    app2(cst("OrdLt"), bvar(0), cst("EpsilonZero")),
                    app(bvar(3), bvar(1)),
                ),
            ),
        ),
    )
}
/// Gentzen's theorem: PA is consistent, proved in PRA + TI(ε₀)
/// GentzenConsistency : ConsistentSystem PA
pub fn gentzen_consistency_ty() -> Expr {
    app(cst("ConsistentSystem"), cst("PA"))
}
/// Gentzen's Hauptsatz for LK: all LK derivations can be cut-eliminated
/// GentzenHauptsatz : ∀ (s : Sequent), LKDerivable s → CutFreeLKDerivable s
pub fn gentzen_hauptsatz_ty() -> Expr {
    impl_pi(
        "s",
        cst("Sequent"),
        arrow(
            app(cst("LKDerivable"), bvar(0)),
            app(cst("CutFreeLKDerivable"), bvar(1)),
        ),
    )
}
/// First incompleteness theorem: ∃ statement undecidable in PA
/// GodelFirstIncompleteness :
///   ∀ (S : ConsistentRecEnum), ∃ (φ : Sentence), ¬Provable S φ ∧ ¬Provable S (neg φ)
pub fn godel_first_incompleteness_ty() -> Expr {
    impl_pi(
        "S",
        cst("ConsistentRecEnum"),
        app2(
            cst("Exists"),
            cst("Sentence"),
            app2(
                cst("And"),
                app2(cst("NotProvable"), bvar(1), bvar(0)),
                app2(cst("NotProvable"), bvar(2), app(cst("NegForm"), bvar(1))),
            ),
        ),
    )
}
/// Second incompleteness theorem: PA cannot prove its own consistency
/// GodelSecondIncompleteness : ¬Provable PA (ConsistStmt PA)
pub fn godel_second_incompleteness_ty() -> Expr {
    arrow(
        app2(cst("ProvableIn"), cst("PA"), cst("ConPa")),
        cst("False"),
    )
}
/// Rosser's strengthening: stronger undecidable sentence without omega-consistency
/// RosserSentence : ∀ (S : ConsistentRecEnum), ∃ (ρ : Sentence), ¬Provable S ρ ∧ ¬Provable S (neg ρ)
pub fn rosser_sentence_ty() -> Expr {
    impl_pi(
        "S",
        cst("ConsistentRecEnum"),
        app2(
            cst("Exists"),
            cst("Sentence"),
            app2(
                cst("And"),
                arrow(app2(cst("ProvableIn"), bvar(1), bvar(0)), cst("False")),
                arrow(
                    app2(cst("ProvableIn"), bvar(2), app(cst("NegForm"), bvar(1))),
                    cst("False"),
                ),
            ),
        ),
    )
}
/// Diagonal lemma (self-reference / fixed-point lemma)
/// DiagonalLemma :
///   ∀ (φ : Formula → Formula), ∃ (ψ : Sentence), Provable PA (iff ψ (φ (godel_num ψ)))
pub fn diagonal_lemma_ty() -> Expr {
    impl_pi(
        "phi",
        arrow(cst("Formula"), cst("Formula")),
        app2(
            cst("Exists"),
            cst("Sentence"),
            app2(
                cst("ProvableIn"),
                cst("PA"),
                app2(
                    cst("IffForm"),
                    bvar(0),
                    app(bvar(2), app(cst("GodelNum"), bvar(1))),
                ),
            ),
        ),
    )
}
/// Tarski's undefinability of truth
/// TarskiUndefinability : ¬∃ (φ : Formula → Bool), ∀ (ψ : Sentence), φ (godel_num ψ) = true ↔ TrueIn Std ψ
pub fn tarski_undefinability_ty() -> Expr {
    arrow(
        app2(
            cst("Exists"),
            arrow(cst("Formula"), bool_ty()),
            impl_pi(
                "psi",
                cst("Sentence"),
                app2(
                    cst("Iff"),
                    app2(
                        cst("BoolEq"),
                        app(bvar(1), app(cst("GodelNum"), bvar(0))),
                        cst("BoolTrue"),
                    ),
                    app2(cst("TrueIn"), cst("StandardModel"), bvar(1)),
                ),
            ),
        ),
        cst("False"),
    )
}
/// Gödel completeness theorem: semantic entailment implies syntactic provability
/// GodelCompleteness :
///   ∀ (T : Theory) (φ : Formula), Entails T φ → ProvableIn T φ
pub fn godel_completeness_ty() -> Expr {
    impl_pi(
        "T",
        cst("Theory"),
        impl_pi(
            "phi",
            cst("Formula"),
            arrow(
                app2(cst("Entails"), bvar(1), bvar(0)),
                app2(cst("ProvableIn"), bvar(2), bvar(1)),
            ),
        ),
    )
}
/// Henkin completeness: every consistent theory has a model
/// HenkinCompleteness : ∀ (T : Theory), ConsistentTheory T → ∃ M, ModelOf M T
pub fn henkin_completeness_ty() -> Expr {
    impl_pi(
        "T",
        cst("Theory"),
        arrow(
            app(cst("ConsistentTheory"), bvar(0)),
            app2(cst("Exists"), cst("Model"), app(cst("ModelOf"), bvar(1))),
        ),
    )
}
/// Compactness theorem: if every finite subset has a model, T has a model
/// Compactness :
///   ∀ (T : Theory),
///     (∀ Δ, FiniteSubset Δ T → ∃ M, ModelOf M Δ) → ∃ M, ModelOf M T
pub fn compactness_ty() -> Expr {
    impl_pi(
        "T",
        cst("Theory"),
        arrow(
            impl_pi(
                "Delta",
                cst("Theory"),
                arrow(
                    app2(cst("FiniteSubset"), bvar(0), bvar(1)),
                    app2(cst("Exists"), cst("Model"), app(cst("ModelOf"), bvar(1))),
                ),
            ),
            app2(cst("Exists"), cst("Model"), app(cst("ModelOf"), bvar(1))),
        ),
    )
}
/// Herbrand's theorem: universal theory is unsatisfiable iff a ground instance is
/// HerbrandTheorem :
///   ∀ (T : UniversalTheory), ¬Satisfiable T ↔ ∃ (I : HerbrandInstance), Refutes I T
pub fn herbrand_theorem_ty() -> Expr {
    impl_pi(
        "T",
        cst("UniversalTheory"),
        app2(
            cst("Iff"),
            arrow(app(cst("Satisfiable"), bvar(0)), cst("False")),
            app2(
                cst("Exists"),
                cst("HerbrandInstance"),
                app(app(cst("Refutes"), bvar(0)), bvar(2)),
            ),
        ),
    )
}
/// Craig interpolation theorem
/// CraigInterpolation :
///   ∀ (A B : Formula), Tautology (imp A B) →
///     ∃ (I : Formula), SubformAtoms I A ∧ SubformAtoms I B ∧ Tautology (imp A I) ∧ Tautology (imp I B)
pub fn craig_interpolation_ty() -> Expr {
    impl_pi(
        "A",
        cst("Formula"),
        impl_pi(
            "B",
            cst("Formula"),
            arrow(
                app(cst("Tautology"), app2(cst("ImpForm"), bvar(1), bvar(0))),
                app2(
                    cst("Exists"),
                    cst("Formula"),
                    app3(
                        cst("And3"),
                        app2(cst("SubformAtoms"), bvar(0), bvar(3)),
                        app2(cst("SubformAtoms"), bvar(1), bvar(2)),
                        app2(
                            cst("And"),
                            app(cst("Tautology"), app2(cst("ImpForm"), bvar(3), bvar(1))),
                            app(cst("Tautology"), app2(cst("ImpForm"), bvar(1), bvar(2))),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// Lyndon interpolation: interpolant uses only positive/negative occurrences properly
/// LyndonInterpolation :
///   ∀ (A B : Formula), Tautology (imp A B) →
///     ∃ (I : Formula), LyndonInterpolant A B I
pub fn lyndon_interpolation_ty() -> Expr {
    impl_pi(
        "A",
        cst("Formula"),
        impl_pi(
            "B",
            cst("Formula"),
            arrow(
                app(cst("Tautology"), app2(cst("ImpForm"), bvar(1), bvar(0))),
                app2(
                    cst("Exists"),
                    cst("Formula"),
                    app3(cst("LyndonInterpolant"), bvar(2), bvar(1), bvar(0)),
                ),
            ),
        ),
    )
}
/// Beth's definability theorem: implicit definability implies explicit definability
/// BethDefinability :
///   ∀ (T : Theory) (P : Predicate),
///     ImplicitlyDefines T P → ∃ (φ : Formula), ExplicitlyDefines T P φ
pub fn beth_definability_ty() -> Expr {
    impl_pi(
        "T",
        cst("Theory"),
        impl_pi(
            "P",
            cst("Predicate"),
            arrow(
                app2(cst("ImplicitlyDefines"), bvar(1), bvar(0)),
                app2(
                    cst("Exists"),
                    cst("Formula"),
                    app3(cst("ExplicitlyDefines"), bvar(2), bvar(2), bvar(0)),
                ),
            ),
        ),
    )
}
/// Robinson's resolution theorem: a set of clauses is unsatisfiable iff the empty clause is derivable
/// ResolutionCompleteness :
///   ∀ (Φ : ClauseSet), ¬Satisfiable Φ ↔ ResolutionRefutable Φ
pub fn resolution_completeness_ty() -> Expr {
    impl_pi(
        "Phi",
        cst("ClauseSet"),
        app2(
            cst("Iff"),
            arrow(app(cst("Satisfiable"), bvar(0)), cst("False")),
            app(cst("ResolutionRefutable"), bvar(1)),
        ),
    )
}
/// Frege proof system type
/// FregeSystem : ProofSystem → Prop (marks it as a Frege system)
pub fn frege_system_ty() -> Expr {
    arrow(cst("ProofSystem"), prop())
}
/// Extended Frege (EF) simulates Frege with polynomial overhead
/// EFSimulatesFrege :
///   ∀ (f : FregeProof), ∃ (e : ExtFregeProof), SimulatesWithBound f e Polynomial
pub fn ef_simulates_frege_ty() -> Expr {
    impl_pi(
        "f",
        cst("FregeProof"),
        app2(
            cst("Exists"),
            cst("ExtFregeProof"),
            app3(
                cst("SimulatesWithBound"),
                bvar(1),
                bvar(0),
                cst("Polynomial"),
            ),
        ),
    )
}
/// Cook-Reckhow theorem: all Frege systems p-simulate each other
/// CookReckhow :
///   ∀ (F1 F2 : ProofSystem), FregeSystem F1 → FregeSystem F2 → PSimulate F1 F2
pub fn cook_reckhow_ty() -> Expr {
    impl_pi(
        "F1",
        cst("ProofSystem"),
        impl_pi(
            "F2",
            cst("ProofSystem"),
            arrow(
                app(cst("FregeSystem"), bvar(1)),
                arrow(
                    app(cst("FregeSystem"), bvar(1)),
                    app2(cst("PSimulate"), bvar(3), bvar(2)),
                ),
            ),
        ),
    )
}
/// Takeuti's conjecture (proved by Tait): cut elimination for higher-order logic
/// TakeutiConjecture : ∀ (seq : HOSequent), HODerivable seq → CutFreeHODerivable seq
pub fn takeuti_conjecture_ty() -> Expr {
    impl_pi(
        "seq",
        cst("HOSequent"),
        arrow(
            app(cst("HODerivable"), bvar(0)),
            app(cst("CutFreeHODerivable"), bvar(1)),
        ),
    )
}
/// Weak normalization: every well-typed term has a normal form
/// WeakNormalization : ∀ (t : LambdaTerm), WellTyped t → ∃ (n : LambdaTerm), NormalForm n ∧ BetaReducesTo t n
pub fn weak_normalization_ty() -> Expr {
    impl_pi(
        "t",
        cst("LambdaTerm"),
        arrow(
            app(cst("WellTyped"), bvar(0)),
            app2(
                cst("Exists"),
                cst("LambdaTerm"),
                app2(
                    cst("And"),
                    app(cst("NormalForm"), bvar(0)),
                    app2(cst("BetaReducesTo"), bvar(2), bvar(1)),
                ),
            ),
        ),
    )
}
/// Church-Rosser (confluence) property for beta-reduction
/// ChurchRosser :
///   ∀ (t s1 s2 : LambdaTerm), BetaReducesTo t s1 → BetaReducesTo t s2 →
///     ∃ u, BetaReducesTo s1 u ∧ BetaReducesTo s2 u
pub fn church_rosser_ty() -> Expr {
    impl_pi(
        "t",
        cst("LambdaTerm"),
        impl_pi(
            "s1",
            cst("LambdaTerm"),
            impl_pi(
                "s2",
                cst("LambdaTerm"),
                arrow(
                    app2(cst("BetaReducesTo"), bvar(2), bvar(1)),
                    arrow(
                        app2(cst("BetaReducesTo"), bvar(3), bvar(1)),
                        app2(
                            cst("Exists"),
                            cst("LambdaTerm"),
                            app2(
                                cst("And"),
                                app2(cst("BetaReducesTo"), bvar(4), bvar(0)),
                                app2(cst("BetaReducesTo"), bvar(4), bvar(1)),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// Register all proof theory axioms into the kernel environment.
pub fn build_proof_theory_env(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("Sequent", sequent_ty()),
        ("Derivation", derivation_ty()),
        ("CutRule", cut_rule_ty()),
        ("NaturalDeduction", natural_deduction_ty()),
        ("LambdaTerm", lambda_term_ty()),
        ("NormalForm", normal_form_ty()),
        ("Ordinal", ordinal_ty()),
        ("Formula", type0()),
        ("Derivable", arrow(cst("Sequent"), prop())),
        ("CutFreeDerivable", arrow(cst("Sequent"), prop())),
        ("SubformulasClosed", arrow(cst("Sequent"), prop())),
        ("WellTyped", arrow(type0(), prop())),
        ("StronglyNormalizing", arrow(type0(), prop())),
        ("Provable", arrow(cst("Formula"), prop())),
        ("Tautology", arrow(cst("Formula"), prop())),
        ("Proof", arrow(prop(), type0())),
        ("HasType", arrow(cst("LambdaTerm"), arrow(prop(), prop()))),
        ("FalseFormula", cst("Formula")),
        ("LKDerivable", lk_derivable_ty()),
        ("LJDerivable", lj_derivable_ty()),
        ("AxiomSeq", arrow(cst("Formula"), cst("Sequent"))),
        (
            "WeakenRight",
            arrow(cst("Sequent"), arrow(cst("Formula"), cst("Sequent"))),
        ),
        ("CutFreeLKDerivable", arrow(cst("Sequent"), prop())),
        ("Context", type0()),
        ("EmptyCtx", cst("Context")),
        (
            "Cons",
            arrow(cst("Formula"), arrow(cst("Context"), cst("Context"))),
        ),
        (
            "NDDerivable",
            arrow(cst("Context"), arrow(cst("Formula"), prop())),
        ),
        (
            "ImpForm",
            arrow(cst("Formula"), arrow(cst("Formula"), cst("Formula"))),
        ),
        (
            "AndForm",
            arrow(cst("Formula"), arrow(cst("Formula"), cst("Formula"))),
        ),
        (
            "OrForm",
            arrow(cst("Formula"), arrow(cst("Formula"), cst("Formula"))),
        ),
        ("NegForm", arrow(cst("Formula"), cst("Formula"))),
        (
            "IffForm",
            arrow(cst("Formula"), arrow(cst("Formula"), cst("Formula"))),
        ),
        (
            "OrdLt",
            arrow(cst("Ordinal"), arrow(cst("Ordinal"), prop())),
        ),
        ("EpsilonZero", epsilon_zero_ty()),
        ("ProofSystem", type0()),
        ("ProofTheoreticOrdinal", proof_theoretic_ordinal_ty()),
        ("PA", cst("ProofSystem")),
        ("ConsistentSystem", arrow(cst("ProofSystem"), prop())),
        ("Sentence", type0()),
        ("ConsistentRecEnum", type0()),
        (
            "NotProvable",
            arrow(cst("ConsistentRecEnum"), arrow(cst("Sentence"), prop())),
        ),
        (
            "ProvableIn",
            arrow(cst("ProofSystem"), arrow(cst("Formula"), prop())),
        ),
        ("ConPa", cst("Formula")),
        ("GodelNum", arrow(cst("Sentence"), cst("Formula"))),
        (
            "TrueIn",
            arrow(cst("Model"), arrow(cst("Sentence"), prop())),
        ),
        ("StandardModel", cst("Model")),
        ("BoolEq", arrow(bool_ty(), arrow(bool_ty(), prop()))),
        ("BoolTrue", bool_ty()),
        ("Theory", type0()),
        (
            "Entails",
            arrow(cst("Theory"), arrow(cst("Formula"), prop())),
        ),
        ("ConsistentTheory", arrow(cst("Theory"), prop())),
        ("Model", type0()),
        ("ModelOf", arrow(cst("Model"), arrow(cst("Theory"), prop()))),
        (
            "FiniteSubset",
            arrow(cst("Theory"), arrow(cst("Theory"), prop())),
        ),
        ("UniversalTheory", type0()),
        ("Satisfiable", arrow(cst("UniversalTheory"), prop())),
        ("HerbrandInstance", type0()),
        (
            "Refutes",
            arrow(
                cst("HerbrandInstance"),
                arrow(cst("UniversalTheory"), prop()),
            ),
        ),
        (
            "SubformAtoms",
            arrow(cst("Formula"), arrow(cst("Formula"), prop())),
        ),
        ("And3", arrow(prop(), arrow(prop(), arrow(prop(), prop())))),
        (
            "LyndonInterpolant",
            arrow(
                cst("Formula"),
                arrow(cst("Formula"), arrow(cst("Formula"), prop())),
            ),
        ),
        ("Predicate", type0()),
        (
            "ImplicitlyDefines",
            arrow(cst("Theory"), arrow(cst("Predicate"), prop())),
        ),
        (
            "ExplicitlyDefines",
            arrow(
                cst("Theory"),
                arrow(cst("Predicate"), arrow(cst("Formula"), prop())),
            ),
        ),
        ("ClauseSet", type0()),
        ("ResolutionRefutable", arrow(cst("ClauseSet"), prop())),
        ("FregeProof", type0()),
        ("ExtFregeProof", type0()),
        (
            "SimulatesWithBound",
            arrow(
                cst("FregeProof"),
                arrow(cst("ExtFregeProof"), arrow(cst("Bound"), prop())),
            ),
        ),
        ("Bound", type0()),
        ("Polynomial", cst("Bound")),
        (
            "PSimulate",
            arrow(cst("ProofSystem"), arrow(cst("ProofSystem"), prop())),
        ),
        ("HOSequent", type0()),
        ("HODerivable", arrow(cst("HOSequent"), prop())),
        ("CutFreeHODerivable", arrow(cst("HOSequent"), prop())),
        (
            "BetaReducesTo",
            arrow(cst("LambdaTerm"), arrow(cst("LambdaTerm"), prop())),
        ),
        ("cut_elimination", cut_elimination_ty()),
        ("subformula_property", subformula_property_ty()),
        ("normalization", normalization_ty()),
        ("consistency", consistency_ty()),
        (
            "completeness_propositional",
            completeness_propositional_ty(),
        ),
        ("curry_howard", curry_howard_ty()),
        ("lk_init", lk_init_ty()),
        ("lk_left_contraction", lk_left_contraction_ty()),
        ("lk_right_weakening", lk_right_weakening_ty()),
        ("lk_and_left1", lk_and_left1_ty()),
        ("lk_and_right", lk_and_right_ty()),
        ("lk_cut", lk_cut_ty()),
        ("lj_imp_left", lj_imp_left_ty()),
        ("lj_imp_right", lj_imp_right_ty()),
        ("nd_imp_intro", nd_imp_intro_ty()),
        ("nd_imp_elim", nd_imp_elim_ty()),
        ("nd_and_intro", nd_and_intro_ty()),
        ("nd_and_elim_left", nd_and_elim_left_ty()),
        ("nd_or_intro_left", nd_or_intro_left_ty()),
        ("pa_ordinal_epsilon_zero", pa_ordinal_epsilon_zero_ty()),
        (
            "ordinal_induction_epsilon_zero",
            ordinal_induction_epsilon_zero_ty(),
        ),
        ("gentzen_consistency", gentzen_consistency_ty()),
        ("gentzen_hauptsatz", gentzen_hauptsatz_ty()),
        (
            "godel_first_incompleteness",
            godel_first_incompleteness_ty(),
        ),
        (
            "godel_second_incompleteness",
            godel_second_incompleteness_ty(),
        ),
        ("rosser_sentence", rosser_sentence_ty()),
        ("diagonal_lemma", diagonal_lemma_ty()),
        ("tarski_undefinability", tarski_undefinability_ty()),
        ("godel_completeness", godel_completeness_ty()),
        ("henkin_completeness", henkin_completeness_ty()),
        ("compactness", compactness_ty()),
        ("herbrand_theorem", herbrand_theorem_ty()),
        ("craig_interpolation", craig_interpolation_ty()),
        ("lyndon_interpolation", lyndon_interpolation_ty()),
        ("beth_definability", beth_definability_ty()),
        ("resolution_completeness", resolution_completeness_ty()),
        ("frege_system", frege_system_ty()),
        ("ef_simulates_frege", ef_simulates_frege_ty()),
        ("cook_reckhow", cook_reckhow_ty()),
        ("takeuti_conjecture", takeuti_conjecture_ty()),
        ("weak_normalization", weak_normalization_ty()),
        ("church_rosser", church_rosser_ty()),
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
/// Evaluate a formula under a truth assignment.
pub fn eval(f: &Formula, assignment: &std::collections::HashMap<String, bool>) -> bool {
    match f {
        Formula::Atom(s) => *assignment.get(s).unwrap_or(&false),
        Formula::True_ => true,
        Formula::False_ => false,
        Formula::Neg(inner) => !eval(inner, assignment),
        Formula::And(a, b) => eval(a, assignment) && eval(b, assignment),
        Formula::Or(a, b) => eval(a, assignment) || eval(b, assignment),
        Formula::Implies(a, b) => !eval(a, assignment) || eval(b, assignment),
        Formula::Iff(a, b) => eval(a, assignment) == eval(b, assignment),
    }
}
/// Generate all truth table rows for the formula.
/// Returns a vector of (assignment, value) pairs.
pub fn truth_table(f: &Formula) -> Vec<(std::collections::HashMap<String, bool>, bool)> {
    let atoms = f.atoms();
    let n = atoms.len();
    let mut rows = Vec::with_capacity(1 << n);
    for mask in 0u64..(1u64 << n) {
        let mut assignment = std::collections::HashMap::new();
        for (i, atom) in atoms.iter().enumerate() {
            assignment.insert(atom.clone(), (mask >> i) & 1 == 1);
        }
        let val = eval(f, &assignment);
        rows.push((assignment, val));
    }
    rows
}
/// DPLL SAT solver for propositional logic in CNF.
///
/// Clauses are represented as `Vec<i32>` where positive literals are variable
/// indices and negative literals are their negations.
/// Returns a satisfying partial assignment (as a list of literals) or `None`.
pub fn dpll_sat(clauses: &[Vec<i32>]) -> Option<Vec<i32>> {
    let mut assignment = Vec::new();
    if dpll_inner(&clauses.to_vec(), &mut assignment) {
        Some(assignment)
    } else {
        None
    }
}
pub fn dpll_inner(clauses: &Vec<Vec<i32>>, assignment: &mut Vec<i32>) -> bool {
    let mut clauses = clauses.clone();
    loop {
        let unit = clauses.iter().find(|c| c.len() == 1).map(|c| c[0]);
        match unit {
            None => break,
            Some(lit) => {
                assignment.push(lit);
                clauses = propagate(&clauses, lit);
            }
        }
    }
    if clauses.iter().any(|c| c.is_empty()) {
        return false;
    }
    if clauses.is_empty() {
        return true;
    }
    let lit = clauses[0][0];
    let mut pos_clauses = propagate(&clauses, lit);
    let mut pos_assign = assignment.clone();
    pos_assign.push(lit);
    if dpll_inner(&pos_clauses, &mut pos_assign) {
        assignment.clear();
        assignment.extend(pos_assign);
        return true;
    }
    let neg_lit = -lit;
    pos_clauses = propagate(&clauses, neg_lit);
    let mut neg_assign = assignment.clone();
    neg_assign.push(neg_lit);
    if dpll_inner(&pos_clauses, &mut neg_assign) {
        assignment.clear();
        assignment.extend(neg_assign);
        return true;
    }
    false
}
pub fn propagate(clauses: &[Vec<i32>], lit: i32) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    for clause in clauses {
        if clause.contains(&lit) {
            continue;
        }
        let new_clause: Vec<i32> = clause.iter().filter(|&&l| l != -lit).copied().collect();
        result.push(new_clause);
    }
    result
}
/// Convert a formula to CNF (simplified Tseitin-style).
/// Returns a list of clauses (each clause is a list of literals).
/// Atoms are assigned indices starting from 1.
pub fn to_cnf(f: &Formula) -> Vec<Vec<i32>> {
    let atoms = f.atoms();
    let atom_idx: std::collections::HashMap<String, i32> = atoms
        .iter()
        .enumerate()
        .map(|(i, s)| (s.clone(), (i + 1) as i32))
        .collect();
    let mut clauses = Vec::new();
    to_cnf_inner(f, &atom_idx, &mut clauses, true);
    clauses
}
pub fn to_cnf_inner(
    f: &Formula,
    idx: &std::collections::HashMap<String, i32>,
    clauses: &mut Vec<Vec<i32>>,
    polarity: bool,
) {
    match f {
        Formula::Atom(s) => {
            let lit = *idx.get(s).unwrap_or(&1);
            clauses.push(vec![if polarity { lit } else { -lit }]);
        }
        Formula::True_ => {
            if !polarity {
                clauses.push(vec![]);
            }
        }
        Formula::False_ => {
            if polarity {
                clauses.push(vec![]);
            }
        }
        Formula::Neg(inner) => {
            to_cnf_inner(inner, idx, clauses, !polarity);
        }
        Formula::And(a, b) => {
            if polarity {
                to_cnf_inner(a, idx, clauses, true);
                to_cnf_inner(b, idx, clauses, true);
            } else {
                to_cnf_inner(a, idx, clauses, false);
                to_cnf_inner(b, idx, clauses, false);
            }
        }
        Formula::Or(a, b) => {
            if polarity {
                to_cnf_inner(a, idx, clauses, true);
                to_cnf_inner(b, idx, clauses, true);
            } else {
                to_cnf_inner(a, idx, clauses, false);
                to_cnf_inner(b, idx, clauses, false);
            }
        }
        Formula::Implies(a, b) => {
            to_cnf_inner(
                &Formula::or(Formula::neg(*a.clone()), *b.clone()),
                idx,
                clauses,
                polarity,
            );
        }
        Formula::Iff(a, b) => {
            to_cnf_inner(
                &Formula::and(
                    Formula::implies(*a.clone(), *b.clone()),
                    Formula::implies(*b.clone(), *a.clone()),
                ),
                idx,
                clauses,
                polarity,
            );
        }
    }
}
/// Check if a propositional sequent is provable using truth tables.
/// For a sequent Γ ⊢ Δ this checks ⋀Γ → ⋁Δ is a tautology.
pub fn is_provable_propositional(seq: &Sequent) -> bool {
    let premise = seq
        .antecedent
        .iter()
        .cloned()
        .reduce(Formula::and)
        .unwrap_or(Formula::True_);
    let conclusion = seq
        .succedent
        .iter()
        .cloned()
        .reduce(Formula::or)
        .unwrap_or(Formula::False_);
    Formula::implies(premise, conclusion).is_tautology()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_formula_tautology() {
        let a = Formula::atom("A");
        let f = Formula::implies(a.clone(), a);
        assert!(f.is_tautology());
    }
    #[test]
    fn test_formula_not_tautology() {
        let a = Formula::atom("A");
        assert!(!a.is_tautology());
    }
    #[test]
    fn test_formula_satisfiable() {
        let a = Formula::atom("A");
        assert!(a.is_satisfiable());
        assert!(!Formula::False_.is_satisfiable());
    }
    #[test]
    fn test_formula_contradiction() {
        assert!(Formula::False_.is_contradiction());
        assert!(!Formula::atom("A").is_contradiction());
    }
    #[test]
    fn test_formula_atoms() {
        let f = Formula::and(Formula::atom("A"), Formula::atom("B"));
        let atoms = f.atoms();
        assert!(atoms.contains(&"A".to_string()));
        assert!(atoms.contains(&"B".to_string()));
        assert_eq!(atoms.len(), 2);
    }
    #[test]
    fn test_formula_depth() {
        let a = Formula::atom("A");
        assert_eq!(a.depth(), 0);
        let ab = Formula::and(Formula::atom("A"), Formula::atom("B"));
        assert_eq!(ab.depth(), 1);
    }
    #[test]
    fn test_sequent_is_axiom() {
        let a = Formula::atom("A");
        let seq = Sequent::new(vec![a.clone()], vec![a]);
        assert!(seq.is_axiom());
        let seq2 = Sequent::new(vec![Formula::atom("A")], vec![Formula::atom("B")]);
        assert!(!seq2.is_axiom());
    }
    #[test]
    fn test_dpll_sat_simple() {
        let result = dpll_sat(&[vec![1]]);
        assert!(result.is_some());
        let result2 = dpll_sat(&[vec![1], vec![-1]]);
        assert!(result2.is_none());
        let result3 = dpll_sat(&[vec![1, -1]]);
        assert!(result3.is_some());
    }
    #[test]
    fn test_combinator_i_reduce() {
        let ix = Combinator::app(Combinator::I, Combinator::K);
        let reduced = ix.reduce_step().expect("I x should reduce");
        assert!(matches!(reduced, Combinator::K));
    }
    #[test]
    fn test_combinator_k_reduce() {
        let kxy = Combinator::app(Combinator::app(Combinator::K, Combinator::I), Combinator::S);
        let reduced = kxy.reduce_step().expect("K x y should reduce");
        assert!(matches!(reduced, Combinator::I));
    }
    #[test]
    fn test_build_proof_theory_env() {
        let mut env = oxilean_kernel::Environment::new();
        build_proof_theory_env(&mut env);
    }
    #[test]
    fn test_formula_to_nnf_double_neg() {
        let a = Formula::atom("A");
        let nn_a = Formula::neg(Formula::neg(a.clone()));
        let nnf = nn_a.to_nnf();
        assert_eq!(nnf, a);
    }
    #[test]
    fn test_formula_to_nnf_de_morgan() {
        let a = Formula::atom("A");
        let b = Formula::atom("B");
        let f = Formula::neg(Formula::and(a.clone(), b.clone()));
        let nnf = f.to_nnf();
        assert_eq!(nnf, Formula::or(Formula::neg(a), Formula::neg(b)));
    }
    #[test]
    fn test_sequent_calculus_proof_valid() {
        let a = Formula::atom("A");
        let seq = Sequent::new(vec![a.clone()], vec![a]);
        let proof = SequentCalculusProof::new(LKNode::axiom(seq));
        assert!(proof.is_valid());
        assert!(proof.is_cut_free());
        assert_eq!(proof.size(), 1);
    }
    #[test]
    fn test_lk_node_cut_free() {
        let a = Formula::atom("A");
        let seq = Sequent::new(vec![a.clone()], vec![a.clone()]);
        let leaf = LKNode::axiom(seq.clone());
        let parent = LKNode::unary(seq, LKRule::LeftWeaken, leaf);
        assert!(parent.is_cut_free());
    }
    #[test]
    fn test_lk_node_not_cut_free() {
        let a = Formula::atom("A");
        let seq = Sequent::new(vec![a.clone()], vec![a.clone()]);
        let leaf1 = LKNode::axiom(seq.clone());
        let leaf2 = LKNode::axiom(seq.clone());
        let cut_node = LKNode::binary(seq, LKRule::Cut(a.clone()), leaf1, leaf2);
        assert!(!cut_node.is_cut_free());
    }
    #[test]
    fn test_nd_term_beta_reduction() {
        let id = NDTerm::Lam(
            "x".to_string(),
            Box::new(Formula::atom("A")),
            Box::new(NDTerm::Var("x".to_string())),
        );
        let applied = NDTerm::App(Box::new(id), Box::new(NDTerm::Var("K".to_string())));
        let reduced = applied.reduce_step().expect("beta-redex should reduce");
        assert_eq!(reduced, NDTerm::Var("K".to_string()));
    }
    #[test]
    fn test_nd_term_fst_reduction() {
        let pair = NDTerm::Pair(
            Box::new(NDTerm::Var("a".to_string())),
            Box::new(NDTerm::Var("b".to_string())),
        );
        let fst = NDTerm::Fst(Box::new(pair));
        let reduced = fst.reduce_step().expect("fst should reduce");
        assert_eq!(reduced, NDTerm::Var("a".to_string()));
    }
    #[test]
    fn test_nd_term_subst() {
        let lam = NDTerm::Lam(
            "y".to_string(),
            Box::new(Formula::atom("T")),
            Box::new(NDTerm::Var("y".to_string())),
        );
        let result = lam.subst("x", &NDTerm::Var("K".to_string()));
        assert_eq!(result, lam);
    }
    #[test]
    fn test_cut_eliminator_verify() {
        let a = Formula::atom("A");
        let seq = Sequent::new(vec![a.clone()], vec![a]);
        let proof = SequentCalculusProof::new(LKNode::axiom(seq));
        assert!(CutEliminator::verify(&proof));
    }
    #[test]
    fn test_resolution_prover_unsat() {
        let clauses = vec![Clause::new(&[1]), Clause::new(&[-1])];
        let prover = ResolutionProver::new(clauses);
        assert!(prover.refute(100));
    }
    #[test]
    fn test_resolution_prover_sat() {
        let clauses = vec![Clause::new(&[1])];
        let prover = ResolutionProver::new(clauses);
        assert!(!prover.refute(100));
    }
    #[test]
    fn test_clause_resolve() {
        let c1 = Clause::new(&[1, 2]);
        let c2 = Clause::new(&[-1, 3]);
        let resolvent = Clause::resolve(&c1, &c2, 1).expect("should resolve");
        assert!(resolvent.0.contains(&2));
        assert!(resolvent.0.contains(&3));
        assert!(!resolvent.0.contains(&1));
    }
    #[test]
    fn test_herbrand_terms_depth_zero() {
        let gen = HerbrandInstanceGenerator::new(vec!["a".to_string(), "b".to_string()], vec![]);
        let terms = gen.terms_up_to_depth(0);
        assert_eq!(terms.len(), 2);
    }
    #[test]
    fn test_herbrand_instance_bind() {
        let mut inst = HerbrandInstance::new();
        inst.bind("x", HerbrandTerm::constant("a"));
        assert_eq!(inst.lookup("x"), Some(&HerbrandTerm::constant("a")));
    }
    #[test]
    fn test_herbrand_instance_generator_next() {
        let mut gen =
            HerbrandInstanceGenerator::new(vec!["a".to_string()], vec![("f".to_string(), 1)]);
        let instances = gen.next_instances(&["x".to_string()]);
        assert!(!instances.is_empty());
    }
    #[test]
    fn test_is_provable_propositional() {
        let a = Formula::atom("A");
        let seq = Sequent::new(vec![], vec![Formula::implies(a.clone(), a)]);
        assert!(is_provable_propositional(&seq));
    }
    #[test]
    fn test_herbrand_term_display() {
        let t = HerbrandTerm::fun("f", vec![HerbrandTerm::constant("a")]);
        assert_eq!(t.to_string(), "f(a)");
    }
    #[test]
    fn test_clause_display_empty() {
        let c = Clause::new(&[]);
        assert_eq!(c.to_string(), "□");
    }
}
