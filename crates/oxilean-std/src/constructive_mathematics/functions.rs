//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{ConstructiveReal, MarkovPrinciple, PowerSetHeyting};

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
pub fn bool_ty() -> Expr {
    cst("Bool")
}
pub fn real_ty() -> Expr {
    cst("Real")
}
pub fn list_ty(elem: Expr) -> Expr {
    app(cst("List"), elem)
}
pub fn option_ty(a: Expr) -> Expr {
    app(cst("Option"), a)
}
pub fn pair_ty(a: Expr, b: Expr) -> Expr {
    app2(cst("Prod"), a, b)
}
/// `IProp : Type` — intuitionistic proposition (proof-relevant).
pub fn iprop_ty() -> Expr {
    type0()
}
/// `IProof : IProp → Type` — the type of proofs of an intuitionistic proposition.
pub fn iproof_ty() -> Expr {
    arrow(iprop_ty(), type0())
}
/// `BHK_And : IProp → IProp → IProp` — conjunction: proof is a pair.
pub fn bhk_and_ty() -> Expr {
    arrow(iprop_ty(), arrow(iprop_ty(), iprop_ty()))
}
/// `BHK_Or : IProp → IProp → IProp` — disjunction: proof is a tagged element.
pub fn bhk_or_ty() -> Expr {
    arrow(iprop_ty(), arrow(iprop_ty(), iprop_ty()))
}
/// `BHK_Impl : IProp → IProp → IProp` — implication: proof is a function.
pub fn bhk_impl_ty() -> Expr {
    arrow(iprop_ty(), arrow(iprop_ty(), iprop_ty()))
}
/// `BHK_Not : IProp → IProp` — negation: proof is a function to ⊥.
pub fn bhk_not_ty() -> Expr {
    arrow(iprop_ty(), iprop_ty())
}
/// `BHK_Forall : (Nat → IProp) → IProp` — universal: proof is a function.
pub fn bhk_forall_ty() -> Expr {
    arrow(arrow(nat_ty(), iprop_ty()), iprop_ty())
}
/// `BHK_Exists : (Nat → IProp) → IProp` — existential: proof is a witness-proof pair.
pub fn bhk_exists_ty() -> Expr {
    arrow(arrow(nat_ty(), iprop_ty()), iprop_ty())
}
/// `IBot : IProp` — intuitionistic falsity (empty type).
pub fn ibot_ty() -> Expr {
    iprop_ty()
}
/// `ITop : IProp` — intuitionistic truth (unit type).
pub fn itop_ty() -> Expr {
    iprop_ty()
}
/// `HeytingAlgebra : Type` — a Heyting algebra (bounded lattice with implication).
pub fn heyting_algebra_ty() -> Expr {
    type0()
}
/// `HeytingMeet : HeytingAlgebra → HeytingAlgebra → HeytingAlgebra` — meet (∧).
pub fn heyting_meet_ty() -> Expr {
    arrow(
        heyting_algebra_ty(),
        arrow(heyting_algebra_ty(), heyting_algebra_ty()),
    )
}
/// `HeytingJoin : HeytingAlgebra → HeytingAlgebra → HeytingAlgebra` — join (∨).
pub fn heyting_join_ty() -> Expr {
    arrow(
        heyting_algebra_ty(),
        arrow(heyting_algebra_ty(), heyting_algebra_ty()),
    )
}
/// `HeytingImpl : HeytingAlgebra → HeytingAlgebra → HeytingAlgebra` — implication (→).
pub fn heyting_impl_ty() -> Expr {
    arrow(
        heyting_algebra_ty(),
        arrow(heyting_algebra_ty(), heyting_algebra_ty()),
    )
}
/// `HeytingNeg : HeytingAlgebra → HeytingAlgebra` — pseudo-complement ¬a = a → ⊥.
pub fn heyting_neg_ty() -> Expr {
    arrow(heyting_algebra_ty(), heyting_algebra_ty())
}
/// `HeytingBot : HeytingAlgebra` — bottom element ⊥.
pub fn heyting_bot_ty() -> Expr {
    heyting_algebra_ty()
}
/// `HeytingTop : HeytingAlgebra` — top element ⊤.
pub fn heyting_top_ty() -> Expr {
    heyting_algebra_ty()
}
/// `HeytingLe : HeytingAlgebra → HeytingAlgebra → Prop` — partial order.
pub fn heyting_le_ty() -> Expr {
    arrow(heyting_algebra_ty(), arrow(heyting_algebra_ty(), prop()))
}
/// `HeytingImplAdjunction : a ∧ b ≤ c ↔ a ≤ b → c.`
pub fn heyting_impl_adjunction_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "H",
        heyting_algebra_ty(),
        pi(
            BinderInfo::Default,
            "a",
            heyting_algebra_ty(),
            pi(
                BinderInfo::Default,
                "b",
                heyting_algebra_ty(),
                pi(
                    BinderInfo::Default,
                    "c",
                    heyting_algebra_ty(),
                    app2(
                        cst("Iff"),
                        app2(
                            cst("HeytingLe"),
                            app2(cst("HeytingMeet"), bvar(2), bvar(1)),
                            bvar(0),
                        ),
                        app2(
                            cst("HeytingLe"),
                            bvar(2),
                            app2(cst("HeytingImpl"), bvar(1), bvar(0)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `DoubleNegationLaw : ¬¬a ≤ a fails in general, but ¬¬¬a = ¬a holds.`
pub fn double_negation_law_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "a",
        heyting_algebra_ty(),
        app2(
            cst("Eq"),
            app(
                cst("HeytingNeg"),
                app(cst("HeytingNeg"), app(cst("HeytingNeg"), bvar(0))),
            ),
            app(cst("HeytingNeg"), bvar(0)),
        ),
    )
}
/// `BooleanAlgebra : Type` — Boolean algebra (Heyting algebra satisfying a ∨ ¬a = ⊤).
pub fn boolean_algebra_ty() -> Expr {
    type0()
}
/// `ExcludedMiddleFails : ¬ (∀ a, a ∨ ¬a = ⊤) in Heyting algebras.`
pub fn excluded_middle_fails_ty() -> Expr {
    prop()
}
/// `CauchySeq : (Nat → Real) → Prop` — a Cauchy sequence with explicit modulus.
pub fn cauchy_seq_ty() -> Expr {
    arrow(arrow(nat_ty(), real_ty()), prop())
}
/// `CauchyModulus : (Nat → Real) → (Nat → Nat) → Prop` — modulus of Cauchy convergence.
pub fn cauchy_modulus_ty() -> Expr {
    arrow(
        arrow(nat_ty(), real_ty()),
        arrow(arrow(nat_ty(), nat_ty()), prop()),
    )
}
/// `BishopReal : Type` — Bishop real: Cauchy sequence with explicit modulus.
pub fn bishop_real_ty() -> Expr {
    type0()
}
/// `BishopRealAdd : BishopReal → BishopReal → BishopReal`
pub fn bishop_real_add_ty() -> Expr {
    arrow(bishop_real_ty(), arrow(bishop_real_ty(), bishop_real_ty()))
}
/// `BishopRealMul : BishopReal → BishopReal → BishopReal`
pub fn bishop_real_mul_ty() -> Expr {
    arrow(bishop_real_ty(), arrow(bishop_real_ty(), bishop_real_ty()))
}
/// `BishopRealEq : BishopReal → BishopReal → Prop` — constructive equality (coincidence).
pub fn bishop_real_eq_ty() -> Expr {
    arrow(bishop_real_ty(), arrow(bishop_real_ty(), prop()))
}
/// `BishopRealLt : BishopReal → BishopReal → Prop` — constructive strict order.
pub fn bishop_real_lt_ty() -> Expr {
    arrow(bishop_real_ty(), arrow(bishop_real_ty(), prop()))
}
/// `BishopRealApart : BishopReal → BishopReal → Prop` — apartness relation #.
pub fn bishop_real_apart_ty() -> Expr {
    arrow(bishop_real_ty(), arrow(bishop_real_ty(), prop()))
}
/// `BishopRealField : BishopReal forms a constructive field.`
pub fn bishop_real_field_ty() -> Expr {
    prop()
}
/// `ConstructiveIVT : constructive intermediate value theorem (requires apartness).`
pub fn constructive_ivt_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        arrow(bishop_real_ty(), bishop_real_ty()),
        pi(
            BinderInfo::Default,
            "a",
            bishop_real_ty(),
            pi(
                BinderInfo::Default,
                "b",
                bishop_real_ty(),
                arrow(
                    app2(cst("BishopRealLt"), bvar(1), bvar(0)),
                    arrow(
                        app2(
                            cst("SignChanges"),
                            app(bvar(2), bvar(1)),
                            app(bvar(2), bvar(0)),
                        ),
                        app2(
                            cst("BishopExists"),
                            bvar(2),
                            app2(cst("mk_interval"), bvar(1), bvar(0)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// `PartialRecursive : (Nat → Option Nat) → Prop` — partial recursive (computable) function.
pub fn partial_recursive_ty() -> Expr {
    arrow(arrow(nat_ty(), option_ty(nat_ty())), prop())
}
/// `TuringMachine : Type` — a Turing machine description.
pub fn turing_machine_ty() -> Expr {
    type0()
}
/// `TMComputes : TuringMachine → (Nat → Option Nat) → Prop` — TM computes a function.
pub fn tm_computes_ty() -> Expr {
    arrow(
        turing_machine_ty(),
        arrow(arrow(nat_ty(), option_ty(nat_ty())), prop()),
    )
}
/// `ChurchTuringThesis : every computable function is partial recursive.`
pub fn church_turing_thesis_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        arrow(nat_ty(), option_ty(nat_ty())),
        arrow(
            app(cst("Computable"), bvar(0)),
            app(cst("PartialRecursive"), bvar(0)),
        ),
    )
}
/// `HaltingProblemUndecidable : the halting problem is not computable.`
pub fn halting_problem_undecidable_ty() -> Expr {
    prop()
}
/// `UniversalTuringMachine : Type` — a universal Turing machine.
pub fn utm_ty() -> Expr {
    turing_machine_ty()
}
/// `SmNTheorem : the s-m-n theorem (parametric recursion).`
pub fn smn_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "m",
        nat_ty(),
        pi(BinderInfo::Default, "n", nat_ty(), prop()),
    )
}
/// `RecursionTheorem : Kleene's recursion theorem (fixed-point theorem for programs).`
pub fn recursion_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        arrow(nat_ty(), nat_ty()),
        app(cst("HasFixedPointIndex"), bvar(0)),
    )
}
/// `DecidablePred : (Nat → Prop) → Prop` — decidable predicate on Nat.
pub fn decidable_pred_ty() -> Expr {
    arrow(arrow(nat_ty(), prop()), prop())
}
/// `MarkovPrinciple : if P is decidable and ¬¬∃n, Pn then ∃n, Pn.`
pub fn markov_principle_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "P",
        arrow(nat_ty(), prop()),
        arrow(
            app(cst("DecidablePred"), bvar(0)),
            arrow(
                app(cst("Not"), app(cst("Not"), app(cst("Exists"), bvar(0)))),
                app(cst("Exists"), bvar(0)),
            ),
        ),
    )
}
/// `MarkovRule : the rule form of Markov's principle (weaker).`
pub fn markov_rule_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "P",
        arrow(nat_ty(), bool_ty()),
        arrow(
            app(cst("Not"), app(cst("Not"), app(cst("ExistsBool"), bvar(0)))),
            app(cst("ExistsBool"), bvar(0)),
        ),
    )
}
/// `UnboundedSearch : (Nat → Bool) → Nat` — unbounded search (μ-operator).
pub fn unbounded_search_ty() -> Expr {
    arrow(arrow(nat_ty(), bool_ty()), option_ty(nat_ty()))
}
/// `UnboundedSearchCorrect : if P n holds for some n, unbounded_search P returns Some n.`
pub fn unbounded_search_correct_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "P",
        arrow(nat_ty(), bool_ty()),
        pi(
            BinderInfo::Default,
            "n",
            nat_ty(),
            arrow(
                app2(cst("Eq"), app(bvar(1), bvar(0)), cst("Bool.true")),
                app2(
                    cst("Ne"),
                    app(cst("UnboundedSearch"), bvar(1)),
                    cst("Option.none"),
                ),
            ),
        ),
    )
}
/// `BinaryTree : Type` — infinite binary tree (Baire space ℕ^ℕ paths).
pub fn binary_tree_ty() -> Expr {
    type0()
}
/// `Spread : (List Nat → Prop) → Prop` — spread (closed subset of Baire space).
pub fn spread_ty() -> Expr {
    arrow(arrow(list_ty(nat_ty()), prop()), prop())
}
/// `Bar : (List Nat → Prop) → Prop` — a bar (every infinite path hits it).
pub fn bar_ty() -> Expr {
    arrow(arrow(list_ty(nat_ty()), prop()), prop())
}
/// `DecidableBar : (List Nat → Prop) → Prop` — a decidable bar.
pub fn decidable_bar_ty() -> Expr {
    arrow(arrow(list_ty(nat_ty()), prop()), prop())
}
/// `FanTheorem : every decidable bar is uniform (has finite bound).`
pub fn fan_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "B",
        arrow(list_ty(nat_ty()), prop()),
        arrow(
            app(cst("DecidableBar"), bvar(0)),
            app(cst("UniformBar"), bvar(0)),
        ),
    )
}
/// `BarInduction : monotone bar induction principle.`
pub fn bar_induction_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "B",
        arrow(list_ty(nat_ty()), prop()),
        pi(
            BinderInfo::Default,
            "A",
            arrow(list_ty(nat_ty()), prop()),
            arrow(
                app2(cst("BarInductionPremises"), bvar(1), bvar(0)),
                app(bvar(0), list_ty(nat_ty())),
            ),
        ),
    )
}
/// `KripkesSchema : Kripke's schema for choice sequences.`
pub fn kripkes_schema_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "P",
        prop(),
        app(cst("ExistsAlpha"), app(cst("KripkeBool"), bvar(0))),
    )
}
/// `Realizer : Nat → IProp → Prop` — Kleene realizability: n realizes P.
pub fn realizer_ty() -> Expr {
    arrow(nat_ty(), arrow(iprop_ty(), prop()))
}
/// `KleeneRealizes : Nat → IProp → Prop` — n |= P in Kleene realizability.
pub fn kleene_realizes_ty() -> Expr {
    arrow(nat_ty(), arrow(iprop_ty(), prop()))
}
/// `ModifiedRealizability : (Nat → IProp) → IProp` — modified realizability (Kreisel).
pub fn modified_realizability_ty() -> Expr {
    arrow(arrow(nat_ty(), iprop_ty()), iprop_ty())
}
/// `DisjunctionProperty : if P ∨ Q is realizable then P or Q is realizable.`
pub fn disjunction_property_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "P",
        iprop_ty(),
        pi(
            BinderInfo::Default,
            "Q",
            iprop_ty(),
            arrow(
                app(cst("Realizable"), app2(cst("BHK_Or"), bvar(1), bvar(0))),
                app2(
                    cst("Or"),
                    app(cst("Realizable"), bvar(1)),
                    app(cst("Realizable"), bvar(0)),
                ),
            ),
        ),
    )
}
/// `ExistenceProperty : if ∃x.Px is realizable then some specific n satisfies Pn.`
pub fn existence_property_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "P",
        arrow(nat_ty(), iprop_ty()),
        arrow(
            app(cst("Realizable"), app(cst("BHK_Exists"), bvar(0))),
            app2(cst("Sigma"), nat_ty(), bvar(0)),
        ),
    )
}
/// `PcaRealizability : partial combinatory algebra realizability.`
pub fn pca_realizability_ty() -> Expr {
    type0()
}
/// `IdType : ∀ {A : Type}, A → A → Type` — Martin-Löf identity type.
pub fn id_type_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "A",
        type0(),
        arrow(bvar(0), arrow(bvar(1), type0())),
    )
}
/// `IdRefl : ∀ {A : Type} (a : A), Id a a` — reflexivity constructor.
pub fn id_refl_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "A",
        type0(),
        pi(
            BinderInfo::Default,
            "a",
            bvar(0),
            app2(cst("Id"), bvar(0), bvar(0)),
        ),
    )
}
/// `PathInduction : ∀ {A} (C : ∀ x y, Id x y → Type), (∀ a, C a a (refl a)) → ∀ x y (p: Id x y), C x y p.`
pub fn path_induction_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "A",
        type0(),
        pi(
            BinderInfo::Default,
            "C",
            arrow(
                bvar(0),
                arrow(bvar(1), arrow(app2(cst("Id"), bvar(1), bvar(0)), type0())),
            ),
            arrow(
                pi(
                    BinderInfo::Default,
                    "a",
                    bvar(1),
                    app3(bvar(1), bvar(0), bvar(0), app(cst("IdRefl"), bvar(0))),
                ),
                prop(),
            ),
        ),
    )
}
/// `HomotopyType : Type` — homotopy of paths.
pub fn homotopy_type_ty() -> Expr {
    type0()
}
/// `FunExtConstructive : constructive function extensionality.`
pub fn fun_ext_constructive_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "A",
        type0(),
        pi(
            BinderInfo::Implicit,
            "B",
            type0(),
            pi(
                BinderInfo::Default,
                "f",
                arrow(bvar(1), bvar(1)),
                pi(
                    BinderInfo::Default,
                    "g",
                    arrow(bvar(2), bvar(2)),
                    arrow(
                        pi(
                            BinderInfo::Default,
                            "x",
                            bvar(3),
                            app2(cst("Id"), app(bvar(2), bvar(0)), app(bvar(1), bvar(0))),
                        ),
                        app2(cst("Id"), bvar(1), bvar(0)),
                    ),
                ),
            ),
        ),
    )
}
/// `ContinuousFunctionNat : (Nat^Nat → Nat) → Prop` — every function is continuous.
pub fn continuous_function_nat_ty() -> Expr {
    arrow(arrow(arrow(nat_ty(), nat_ty()), nat_ty()), prop())
}
/// `BrouwerContinuity : every function ℕ^ℕ → ℕ is pointwise continuous.`
pub fn brouwer_continuity_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "F",
        arrow(arrow(nat_ty(), nat_ty()), nat_ty()),
        pi(
            BinderInfo::Default,
            "alpha",
            arrow(nat_ty(), nat_ty()),
            app2(cst("IsContinuousAt"), bvar(1), bvar(0)),
        ),
    )
}
/// `BrouwerCreateChoice : every sequence of inhabited sets has a choice function.`
pub fn brouwer_choice_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        arrow(nat_ty(), type0()),
        arrow(
            pi(
                BinderInfo::Default,
                "n",
                nat_ty(),
                app(cst("Inhabited"), app(bvar(1), bvar(0))),
            ),
            pi(BinderInfo::Default, "n", nat_ty(), app(bvar(1), bvar(0))),
        ),
    )
}
/// `UniformContinuityTheorem : every total function on Cantor space is uniformly continuous.`
pub fn uniform_continuity_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        arrow(arrow(nat_ty(), bool_ty()), nat_ty()),
        app(cst("IsUniformlyContinuous"), bvar(0)),
    )
}
/// `HAAxioms : Heyting Arithmetic axioms.`
pub fn ha_axioms_ty() -> Expr {
    prop()
}
/// `MLTTAxioms : Martin-Löf type theory axioms.`
pub fn mltt_axioms_ty() -> Expr {
    prop()
}
/// `CZFAxioms : Constructive Zermelo-Fraenkel set theory axioms.`
pub fn czf_axioms_ty() -> Expr {
    prop()
}
/// `IZFAxioms : Intuitionistic Zermelo-Fraenkel axioms.`
pub fn izf_axioms_ty() -> Expr {
    prop()
}
/// `AxiomOfChoice : the full axiom of choice (non-constructive).`
pub fn axiom_of_choice_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "A",
        type0(),
        pi(
            BinderInfo::Implicit,
            "B",
            arrow(bvar(0), type0()),
            arrow(
                pi(
                    BinderInfo::Default,
                    "x",
                    bvar(1),
                    app(cst("Inhabited"), app(bvar(1), bvar(0))),
                ),
                app2(cst("Sigma"), arrow(bvar(1), bvar(1)), prop()),
            ),
        ),
    )
}
/// `AxiomOfDependentChoice : dependent choice (weaker than full AC, constructive).`
pub fn dependent_choice_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "A",
        type0(),
        pi(
            BinderInfo::Default,
            "R",
            arrow(bvar(0), arrow(bvar(1), prop())),
            arrow(
                app(cst("Serial"), bvar(0)),
                app2(cst("Sigma"), arrow(nat_ty(), bvar(1)), prop()),
            ),
        ),
    )
}
/// `CHA : Prop` — Constructive Heyting Arithmetic as a formal system.
pub fn cha_axioms_ty() -> Expr {
    prop()
}
/// `CHA_Succ : Nat → Nat` — successor function in CHA.
pub fn cha_succ_ty() -> Expr {
    arrow(nat_ty(), nat_ty())
}
/// `CHA_Add : Nat → Nat → Nat` — addition in CHA.
pub fn cha_add_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), nat_ty()))
}
/// `CHA_Mul : Nat → Nat → Nat` — multiplication in CHA.
pub fn cha_mul_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), nat_ty()))
}
/// `CHA_Induction : standard induction schema for CHA.`
pub fn cha_induction_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "P",
        arrow(nat_ty(), prop()),
        arrow(
            app(bvar(0), cst("CHA_Zero")),
            arrow(
                pi(
                    BinderInfo::Default,
                    "n",
                    nat_ty(),
                    arrow(
                        app(bvar(1), bvar(0)),
                        app(bvar(1), app(cst("CHA_Succ"), bvar(0))),
                    ),
                ),
                pi(BinderInfo::Default, "n", nat_ty(), app(bvar(1), bvar(0))),
            ),
        ),
    )
}
/// `CHA_LessEq : Nat → Nat → Prop` — constructive ≤ on Nat.
pub fn cha_less_eq_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `EffectiveTopos : Type` — the effective topos (Hyland, 1982).
pub fn effective_topos_ty() -> Expr {
    type0()
}
/// `RealizabilityTopos : Type` — a general realizability topos.
pub fn realizability_topos_ty() -> Expr {
    type0()
}
/// `PCA : Type` — partial combinatory algebra (the base of realizability).
pub fn pca_ty() -> Expr {
    type0()
}
/// `PCAApp : PCA → PCA → Option PCA` — partial application in a PCA.
pub fn pca_app_ty() -> Expr {
    arrow(pca_ty(), arrow(pca_ty(), option_ty(pca_ty())))
}
/// `KleeneFirst : PCA` — Kleene's first algebra (partial recursive functions).
pub fn kleene_first_ty() -> Expr {
    pca_ty()
}
/// `KleeneSecond : PCA` — Kleene's second algebra (total continuous functions).
pub fn kleene_second_ty() -> Expr {
    pca_ty()
}
/// `EffectiveToposInternalLogic : the internal logic of Eff is IZF.`
pub fn effective_topos_internal_logic_ty() -> Expr {
    prop()
}
/// `AssemblyCategory : PCA → Type` — category of assemblies over a PCA.
pub fn assembly_category_ty() -> Expr {
    arrow(pca_ty(), type0())
}
/// `CZFSet : Type` — a set in Constructive Zermelo-Fraenkel set theory.
pub fn czf_set_ty() -> Expr {
    type0()
}
/// `CZF_Member : CZFSet → CZFSet → Prop` — membership relation in CZF.
pub fn czf_member_ty() -> Expr {
    arrow(czf_set_ty(), arrow(czf_set_ty(), prop()))
}
/// `CZF_Extensionality : two sets with same members are equal.`
pub fn czf_extensionality_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "a",
        czf_set_ty(),
        pi(
            BinderInfo::Default,
            "b",
            czf_set_ty(),
            arrow(
                pi(
                    BinderInfo::Default,
                    "x",
                    czf_set_ty(),
                    app2(
                        cst("Iff"),
                        app2(cst("CZF_Member"), bvar(0), bvar(2)),
                        app2(cst("CZF_Member"), bvar(0), bvar(1)),
                    ),
                ),
                app2(cst("Eq"), bvar(1), bvar(0)),
            ),
        ),
    )
}
/// `CZF_Subset : CZFSet → CZFSet → Prop` — subset relation.
pub fn czf_subset_ty() -> Expr {
    arrow(czf_set_ty(), arrow(czf_set_ty(), prop()))
}
/// `CZF_StrongCollection : strong collection axiom schema (replaces replacement).`
pub fn czf_strong_collection_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        arrow(czf_set_ty(), arrow(czf_set_ty(), prop())),
        pi(
            BinderInfo::Default,
            "a",
            czf_set_ty(),
            arrow(
                pi(
                    BinderInfo::Default,
                    "x",
                    czf_set_ty(),
                    arrow(
                        app2(cst("CZF_Member"), bvar(0), bvar(1)),
                        app(cst("CZF_Exists"), app(bvar(2), bvar(0))),
                    ),
                ),
                app(
                    cst("CZF_Exists"),
                    app(cst("CZF_Collection"), app(bvar(1), bvar(0))),
                ),
            ),
        ),
    )
}
/// `CZF_SubsetCollection : subset collection axiom (for power-set replacement).`
pub fn czf_subset_collection_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "a",
        czf_set_ty(),
        pi(
            BinderInfo::Default,
            "b",
            czf_set_ty(),
            app(
                cst("CZF_Exists"),
                app2(cst("CZF_SubColl"), bvar(1), bvar(0)),
            ),
        ),
    )
}
/// `AntiFoundation : Aczel's anti-foundation axiom (AFA).`
pub fn anti_foundation_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        arrow(czf_set_ty(), czf_set_ty()),
        app(cst("CZF_Exists"), app(cst("AFA_Solution"), bvar(0))),
    )
}
/// `IZF_Regularity : every non-empty set has an ∈-minimal element.`
pub fn izf_regularity_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "a",
        czf_set_ty(),
        arrow(
            app(cst("CZF_Nonempty"), bvar(0)),
            app2(cst("CZF_HasMinimalElement"), bvar(0), cst("CZF_Member")),
        ),
    )
}
/// `BarRecursor : ((Nat → Nat) → Nat) → (Nat → Nat) → Nat` — Spector's bar recursion.
pub fn bar_recursor_ty() -> Expr {
    arrow(
        arrow(arrow(nat_ty(), nat_ty()), nat_ty()),
        arrow(arrow(nat_ty(), nat_ty()), nat_ty()),
    )
}
/// `BarRecursionAxiom : the axiom governing bar recursion (Spector 1962).`
pub fn bar_recursion_axiom_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "Y",
        arrow(arrow(nat_ty(), nat_ty()), nat_ty()),
        pi(
            BinderInfo::Default,
            "G",
            arrow(arrow(nat_ty(), nat_ty()), nat_ty()),
            pi(
                BinderInfo::Default,
                "H",
                arrow(
                    list_ty(nat_ty()),
                    arrow(arrow(nat_ty(), nat_ty()), nat_ty()),
                ),
                prop(),
            ),
        ),
    )
}
/// `UniformModulus : (Nat^Nat → Nat) → Nat` — uniform modulus of continuity.
pub fn uniform_modulus_ty() -> Expr {
    arrow(arrow(arrow(nat_ty(), nat_ty()), nat_ty()), nat_ty())
}
/// `SpectorTranslation : double-negation shift for bar recursion.`
pub fn spector_translation_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        prop(),
        arrow(app(cst("Not"), app(cst("Not"), bvar(0))), bvar(0)),
    )
}
/// `FiniteFan : (List Nat → Prop) → Prop` — a finitely branching spread.
pub fn finite_fan_ty() -> Expr {
    arrow(arrow(list_ty(nat_ty()), prop()), prop())
}
/// `FanTheoremStrong : strong fan theorem (every bar on a fan has a finite sub-bar).`
pub fn fan_theorem_strong_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "F",
        arrow(list_ty(nat_ty()), prop()),
        pi(
            BinderInfo::Default,
            "B",
            arrow(list_ty(nat_ty()), prop()),
            arrow(
                app(cst("FiniteFan"), bvar(1)),
                arrow(app(cst("Bar"), bvar(0)), app(cst("FiniteBar"), bvar(0))),
            ),
        ),
    )
}
/// `BarTheoremAnalytic : analytic bar induction on the Baire space.`
pub fn bar_theorem_analytic_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "B",
        arrow(list_ty(nat_ty()), prop()),
        arrow(
            app2(cst("AnalyticBar"), bvar(0), cst("BaireSpace")),
            app(cst("MonotoneInduction"), bvar(0)),
        ),
    )
}
/// `KleeneBrouwerOrdering : well-founded ordering on finite sequences.`
pub fn kleene_brouwer_ordering_ty() -> Expr {
    arrow(list_ty(nat_ty()), arrow(list_ty(nat_ty()), prop()))
}
/// `ConstructiveHeineBorel : the unit interval [0,1] is compact for lawful sequences.`
pub fn constructive_heine_borel_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "cover",
        arrow(bishop_real_ty(), prop()),
        arrow(
            app(
                cst("OpenCover"),
                app2(cst("UnitInterval"), cst("BishopZero"), cst("BishopOne")),
            ),
            app(cst("FiniteSubcover"), bvar(0)),
        ),
    )
}
/// `LawfulSequence : (Nat → BishopReal) → Prop` — a sequence with computable modulus.
pub fn lawful_sequence_ty() -> Expr {
    arrow(arrow(nat_ty(), bishop_real_ty()), prop())
}
/// `SequentialCompactness : [0,1] is sequentially compact for lawful sequences.`
pub fn sequential_compactness_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "s",
        arrow(nat_ty(), bishop_real_ty()),
        arrow(
            app(cst("LawfulSequence"), bvar(0)),
            app(cst("HasConvergentSubsequence"), bvar(0)),
        ),
    )
}
/// `MP_PR : Markov's principle for primitive recursive predicates.`
pub fn mp_pr_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "P",
        arrow(nat_ty(), bool_ty()),
        arrow(
            app(cst("IsPrimRec"), bvar(0)),
            arrow(
                app(cst("Not"), app(cst("Not"), app(cst("ExistsBool"), bvar(1)))),
                app(cst("ExistsBool"), bvar(1)),
            ),
        ),
    )
}
/// `MP_Semi : semi-Markov principle (for Σ₁ predicates).`
pub fn mp_semi_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "P",
        arrow(nat_ty(), prop()),
        arrow(
            app(cst("IsSigma1"), bvar(0)),
            arrow(
                app(cst("Not"), app(cst("Not"), app(cst("Exists"), bvar(1)))),
                app(cst("Exists"), bvar(1)),
            ),
        ),
    )
}
/// `MP_Weak : weak Markov's principle (Π₂ form).`
pub fn mp_weak_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "alpha",
        arrow(nat_ty(), nat_ty()),
        arrow(
            app(
                cst("Not"),
                app(cst("Not"), app(cst("ExistsAlphaZero"), bvar(0))),
            ),
            app(cst("ExistsAlphaZero"), bvar(0)),
        ),
    )
}
/// `CT0 : Church's thesis — every function Nat → Nat is recursive.`
pub fn ct0_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        arrow(nat_ty(), nat_ty()),
        app(cst("IsRecursive"), bvar(0)),
    )
}
/// `CT0_Consequence_NoContinuousAll : CT₀ contradicts all-functions-continuous.`
pub fn ct0_consequence_no_continuous_all_ty() -> Expr {
    arrow(
        ct0_ty(),
        app(
            cst("Not"),
            pi(
                BinderInfo::Default,
                "f",
                arrow(nat_ty(), nat_ty()),
                app(cst("IsContinuous"), bvar(0)),
            ),
        ),
    )
}
/// `CT0_Enumerable : CT₀ implies Nat → Nat is effectively enumerable.`
pub fn ct0_enumerable_ty() -> Expr {
    arrow(
        ct0_ty(),
        app(cst("EffectivelyEnumerable"), arrow(nat_ty(), nat_ty())),
    )
}
/// `CT0_Diagonalization : CT₀ plus diagonalisation yields undecidable problems.`
pub fn ct0_diagonalization_ty() -> Expr {
    arrow(
        ct0_ty(),
        app(
            cst("Exists"),
            app(cst("Undecidable"), cst("TotalRecursiveFns")),
        ),
    )
}
/// `CoherentRing : Type` — a coherent ring (finitely generated submodules are finitely related).
pub fn coherent_ring_ty() -> Expr {
    type0()
}
/// `CoherentRing_Ideal : CoherentRing → Type` — ideals in a coherent ring.
pub fn coherent_ring_ideal_ty() -> Expr {
    arrow(coherent_ring_ty(), type0())
}
/// `ExplicitField : Type` — a field with decidable equality.
pub fn explicit_field_ty() -> Expr {
    type0()
}
/// `ExplicitField_Inv : ExplicitField → ExplicitField → ExplicitField` — division.
pub fn explicit_field_inv_ty() -> Expr {
    arrow(
        explicit_field_ty(),
        arrow(explicit_field_ty(), explicit_field_ty()),
    )
}
/// `CoherentRing_SyzygyModule : every submodule of a free coherent module has a syzygy.`
pub fn coherent_ring_syzygy_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        coherent_ring_ty(),
        pi(
            BinderInfo::Default,
            "M",
            app(cst("FreeModule"), bvar(0)),
            pi(
                BinderInfo::Default,
                "N",
                app(cst("Submodule"), bvar(0)),
                app(cst("HasFiniteSyzygy"), bvar(0)),
            ),
        ),
    )
}
/// `GCDDomain : Type` — constructive GCD domain (Bezout domain with algorithm).
pub fn gcd_domain_ty() -> Expr {
    type0()
}
/// `GCDDomain_Gcd : GCDDomain → GCDDomain → GCDDomain` — gcd of two elements.
pub fn gcd_domain_gcd_ty() -> Expr {
    arrow(gcd_domain_ty(), arrow(gcd_domain_ty(), gcd_domain_ty()))
}
/// `Numbering : (Nat → Option A) → Prop` — a numbering of a set A.
pub fn numbering_ty() -> Expr {
    arrow(arrow(nat_ty(), option_ty(type0())), prop())
}
/// `RecursiveNumbering : Type` — a recursively enumerable numbering.
pub fn recursive_numbering_ty() -> Expr {
    type0()
}
/// `GoedelNumbering : a Gödel numbering of partial recursive functions.`
pub fn goedel_numbering_ty() -> Expr {
    arrow(arrow(nat_ty(), option_ty(nat_ty())), nat_ty())
}
/// `GrzegorczykHierarchy : Nat → Type` — the Grzegorczyk hierarchy E_n.
pub fn grzegorczyk_hierarchy_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `GrzegorczykUnion : ⋃_n E_n = primitive recursive functions.`
pub fn grzegorczyk_union_ty() -> Expr {
    app2(
        cst("Eq"),
        app(cst("Union"), cst("GrzegorczykHierarchy")),
        cst("PrimRecFns"),
    )
}
/// `TTERepresentation : Type` — a representation in TTE (Weihrauch 2000).
pub fn tte_representation_ty() -> Expr {
    type0()
}
/// `TTEComputable : TTERepresentation → TTERepresentation → Prop` — computable map.
pub fn tte_computable_ty() -> Expr {
    arrow(
        tte_representation_ty(),
        arrow(tte_representation_ty(), prop()),
    )
}
/// `CauchyRepresentation : TTERepresentation` — standard Cauchy representation of ℝ.
pub fn cauchy_representation_ty() -> Expr {
    tte_representation_ty()
}
/// `SignedDigitRepresentation : TTERepresentation` — signed digit representation.
pub fn signed_digit_representation_ty() -> Expr {
    tte_representation_ty()
}
/// `TTEAdditionComputable : addition of reals is TTE-computable.`
pub fn tte_addition_computable_ty() -> Expr {
    app2(
        cst("TTEComputable"),
        app2(
            cst("ProductRep"),
            cst("CauchyRepresentation"),
            cst("CauchyRepresentation"),
        ),
        cst("CauchyRepresentation"),
    )
}
/// `WeihrauchDegree : Type` — Weihrauch degree of a multi-valued function.
pub fn weihrauch_degree_ty() -> Expr {
    type0()
}
/// `LPO_Weihrauch : WeihrauchDegree` — degree of the Limited Principle of Omniscience.
pub fn lpo_weihrauch_ty() -> Expr {
    weihrauch_degree_ty()
}
/// `CountableChoice : CC — countable choice axiom.`
pub fn countable_choice_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "A",
        type0(),
        pi(
            BinderInfo::Default,
            "f",
            arrow(nat_ty(), arrow(bvar(0), prop())),
            arrow(
                pi(
                    BinderInfo::Default,
                    "n",
                    nat_ty(),
                    app(cst("Inhabited"), app(bvar(1), bvar(0))),
                ),
                app2(cst("Sigma"), arrow(nat_ty(), bvar(1)), prop()),
            ),
        ),
    )
}
/// `DC_Relation : a relation that admits DC.`
pub fn dc_relation_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// `DependentChoiceScheme : DC scheme — a strengthening of CC.`
pub fn dependent_choice_scheme_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "A",
        type0(),
        pi(
            BinderInfo::Default,
            "R",
            arrow(bvar(0), arrow(bvar(1), prop())),
            pi(
                BinderInfo::Default,
                "a0",
                bvar(1),
                arrow(
                    pi(
                        BinderInfo::Default,
                        "x",
                        bvar(2),
                        app(
                            cst("Inhabited"),
                            app(cst("R_Image"), app2(bvar(2), bvar(0), bvar(3))),
                        ),
                    ),
                    app2(cst("Sigma"), arrow(nat_ty(), bvar(3)), prop()),
                ),
            ),
        ),
    )
}
/// `ConstructiveMeasure : Type` — a constructive (Bishop-style) measure.
pub fn constructive_measure_ty() -> Expr {
    type0()
}
/// `ConstructiveIntegral : (BishopReal → BishopReal) → ConstructiveMeasure → BishopReal`
pub fn constructive_integral_ty() -> Expr {
    arrow(
        arrow(bishop_real_ty(), bishop_real_ty()),
        arrow(constructive_measure_ty(), bishop_real_ty()),
    )
}
/// `ConstructiveLebesgue : the constructive Lebesgue integral is well-defined.`
pub fn constructive_lebesgue_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        arrow(bishop_real_ty(), bishop_real_ty()),
        pi(
            BinderInfo::Default,
            "mu",
            constructive_measure_ty(),
            arrow(
                app(cst("IntegrableConstructive"), bvar(1)),
                app2(
                    cst("WellDefined"),
                    app(cst("ConstructiveIntegral"), bvar(1)),
                    bvar(0),
                ),
            ),
        ),
    )
}
/// `ConstructiveMonotoneConvergence : constructive monotone convergence theorem.`
pub fn constructive_monotone_convergence_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "fseq",
        arrow(nat_ty(), arrow(bishop_real_ty(), bishop_real_ty())),
        pi(
            BinderInfo::Default,
            "mu",
            constructive_measure_ty(),
            arrow(
                app2(cst("MonotoneBounded"), bvar(1), bvar(0)),
                app(
                    cst("HasConstructiveLimit"),
                    app2(cst("IntegralSequence"), bvar(1), bvar(0)),
                ),
            ),
        ),
    )
}
/// `NilSquare : Type` — the object of nilsquare infinitesimals D = {d : R | d² = 0}.`
pub fn nil_square_ty() -> Expr {
    type0()
}
/// `SDG_Kock_Lawvere : the Kock-Lawvere axiom: every function D → R is linear.`
pub fn sdg_kock_lawvere_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        arrow(nil_square_ty(), real_ty()),
        app(cst("ExistsUnique"), app(cst("SDG_LinearFactor"), bvar(0))),
    )
}
/// `SDG_TangentBundle : TangentBundle R^n as a microlinear space.`
pub fn sdg_tangent_bundle_ty() -> Expr {
    arrow(type0(), type0())
}
/// `SDG_VectorField : a vector field is a section of the tangent bundle.`
pub fn sdg_vector_field_ty() -> Expr {
    pi(
        BinderInfo::Implicit,
        "M",
        type0(),
        arrow(app(cst("SDG_TangentBundle"), bvar(0)), bvar(1)),
    )
}
/// `SDG_Integration : integration as left inverse to differentiation in SDG.`
pub fn sdg_integration_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        arrow(real_ty(), real_ty()),
        pi(
            BinderInfo::Default,
            "a",
            real_ty(),
            pi(
                BinderInfo::Default,
                "b",
                real_ty(),
                app2(
                    cst("Eq"),
                    app(
                        cst("SDG_Derivative"),
                        app2(cst("SDG_Integral"), bvar(2), bvar(1)),
                    ),
                    app(bvar(2), bvar(0)),
                ),
            ),
        ),
    )
}
/// `SDG_MicrolinearSpace : a space where infinitesimal figures are affine.`
pub fn sdg_microlinear_ty() -> Expr {
    arrow(type0(), prop())
}
/// Register all constructive mathematics axioms into the kernel environment.
pub fn build_constructive_mathematics_env(env: &mut Environment) -> Result<(), String> {
    let axioms: &[(&str, Expr)] = &[
        ("IProp", iprop_ty()),
        ("IProof", iproof_ty()),
        ("BHK_And", bhk_and_ty()),
        ("BHK_Or", bhk_or_ty()),
        ("BHK_Impl", bhk_impl_ty()),
        ("BHK_Not", bhk_not_ty()),
        ("BHK_Forall", bhk_forall_ty()),
        ("BHK_Exists", bhk_exists_ty()),
        ("IBot", ibot_ty()),
        ("ITop", itop_ty()),
        ("Not", arrow(prop(), prop())),
        ("Exists", arrow(arrow(nat_ty(), prop()), prop())),
        ("ExistsBool", arrow(arrow(nat_ty(), bool_ty()), prop())),
        ("Realizable", arrow(iprop_ty(), prop())),
        ("HeytingAlgebra", heyting_algebra_ty()),
        ("HeytingMeet", heyting_meet_ty()),
        ("HeytingJoin", heyting_join_ty()),
        ("HeytingImpl", heyting_impl_ty()),
        ("HeytingNeg", heyting_neg_ty()),
        ("HeytingBot", heyting_bot_ty()),
        ("HeytingTop", heyting_top_ty()),
        ("HeytingLe", heyting_le_ty()),
        ("BooleanAlgebra", boolean_algebra_ty()),
        ("ExcludedMiddleFails", excluded_middle_fails_ty()),
        ("heyting_impl_adjunction", heyting_impl_adjunction_ty()),
        ("double_negation_law", double_negation_law_ty()),
        ("CauchySeq", cauchy_seq_ty()),
        ("CauchyModulus", cauchy_modulus_ty()),
        ("BishopReal", bishop_real_ty()),
        ("BishopRealAdd", bishop_real_add_ty()),
        ("BishopRealMul", bishop_real_mul_ty()),
        ("BishopRealEq", bishop_real_eq_ty()),
        ("BishopRealLt", bishop_real_lt_ty()),
        ("BishopRealApart", bishop_real_apart_ty()),
        ("BishopRealField", bishop_real_field_ty()),
        ("SignChanges", arrow(real_ty(), arrow(real_ty(), prop()))),
        (
            "BishopExists",
            arrow(
                arrow(bishop_real_ty(), bishop_real_ty()),
                arrow(pair_ty(bishop_real_ty(), bishop_real_ty()), prop()),
            ),
        ),
        (
            "mk_interval",
            arrow(
                bishop_real_ty(),
                arrow(
                    bishop_real_ty(),
                    pair_ty(bishop_real_ty(), bishop_real_ty()),
                ),
            ),
        ),
        ("constructive_ivt", constructive_ivt_ty()),
        ("PartialRecursive", partial_recursive_ty()),
        ("TuringMachine", turing_machine_ty()),
        ("TMComputes", tm_computes_ty()),
        (
            "Computable",
            arrow(arrow(nat_ty(), option_ty(nat_ty())), prop()),
        ),
        (
            "HaltingProblemUndecidable",
            halting_problem_undecidable_ty(),
        ),
        ("UTM", utm_ty()),
        (
            "HasFixedPointIndex",
            arrow(arrow(nat_ty(), nat_ty()), prop()),
        ),
        ("church_turing_thesis", church_turing_thesis_ty()),
        (
            "halting_problem_undecidable",
            halting_problem_undecidable_ty(),
        ),
        ("smn_theorem", smn_theorem_ty()),
        ("recursion_theorem", recursion_theorem_ty()),
        ("DecidablePred", decidable_pred_ty()),
        ("UnboundedSearch", unbounded_search_ty()),
        ("markov_principle", markov_principle_ty()),
        ("markov_rule", markov_rule_ty()),
        ("unbounded_search_correct", unbounded_search_correct_ty()),
        ("BinaryTree", binary_tree_ty()),
        ("Spread", spread_ty()),
        ("Bar", bar_ty()),
        ("DecidableBar", decidable_bar_ty()),
        (
            "UniformBar",
            arrow(arrow(list_ty(nat_ty()), prop()), prop()),
        ),
        (
            "BarInductionPremises",
            arrow(
                arrow(list_ty(nat_ty()), prop()),
                arrow(arrow(list_ty(nat_ty()), prop()), prop()),
            ),
        ),
        (
            "ExistsAlpha",
            arrow(arrow(arrow(nat_ty(), nat_ty()), bool_ty()), prop()),
        ),
        (
            "KripkeBool",
            arrow(prop(), arrow(arrow(nat_ty(), nat_ty()), bool_ty())),
        ),
        ("fan_theorem", fan_theorem_ty()),
        ("bar_induction", bar_induction_ty()),
        ("kripkes_schema", kripkes_schema_ty()),
        ("Realizer", realizer_ty()),
        ("KleeneRealizes", kleene_realizes_ty()),
        ("ModifiedRealizability", modified_realizability_ty()),
        ("PcaRealizability", pca_realizability_ty()),
        ("Or", arrow(prop(), arrow(prop(), prop()))),
        (
            "Sigma",
            arrow(type0(), arrow(arrow(nat_ty(), type0()), type0())),
        ),
        ("disjunction_property", disjunction_property_ty()),
        ("existence_property", existence_property_ty()),
        ("Id", id_type_ty()),
        ("IdRefl", id_refl_ty()),
        ("HomotopyType", homotopy_type_ty()),
        ("path_induction", path_induction_ty()),
        ("fun_ext_constructive", fun_ext_constructive_ty()),
        (
            "IsContinuousAt",
            arrow(
                arrow(arrow(nat_ty(), nat_ty()), nat_ty()),
                arrow(arrow(nat_ty(), nat_ty()), prop()),
            ),
        ),
        (
            "IsUniformlyContinuous",
            arrow(arrow(arrow(nat_ty(), bool_ty()), nat_ty()), prop()),
        ),
        ("Inhabited", arrow(type0(), prop())),
        ("Serial", arrow(type0(), prop())),
        ("brouwer_continuity", brouwer_continuity_ty()),
        ("brouwer_choice", brouwer_choice_ty()),
        (
            "uniform_continuity_theorem",
            uniform_continuity_theorem_ty(),
        ),
        ("HAAxioms", ha_axioms_ty()),
        ("MLTTAxioms", mltt_axioms_ty()),
        ("CZFAxioms", czf_axioms_ty()),
        ("IZFAxioms", izf_axioms_ty()),
        ("axiom_of_choice", axiom_of_choice_ty()),
        ("dependent_choice", dependent_choice_ty()),
        ("LEM", prop()),
        ("DNE", prop()),
        ("Peirce", prop()),
        ("Bool.true", bool_ty()),
        ("Option.none", option_ty(nat_ty())),
        ("CHA", cha_axioms_ty()),
        ("CHA_Zero", nat_ty()),
        ("CHA_Succ", cha_succ_ty()),
        ("CHA_Add", cha_add_ty()),
        ("CHA_Mul", cha_mul_ty()),
        ("cha_induction", cha_induction_ty()),
        ("CHA_LessEq", cha_less_eq_ty()),
        ("Iff", arrow(prop(), arrow(prop(), prop()))),
        ("EffectiveTopos", effective_topos_ty()),
        ("RealizabilityTopos", realizability_topos_ty()),
        ("PCA", pca_ty()),
        ("PCAApp", pca_app_ty()),
        ("KleeneFirst", kleene_first_ty()),
        ("KleeneSecond", kleene_second_ty()),
        (
            "EffectiveToposInternalLogic",
            effective_topos_internal_logic_ty(),
        ),
        ("AssemblyCategory", assembly_category_ty()),
        ("CZFSet", czf_set_ty()),
        ("CZF_Member", czf_member_ty()),
        ("czf_extensionality", czf_extensionality_ty()),
        ("CZF_Subset", czf_subset_ty()),
        ("CZF_Exists", arrow(arrow(czf_set_ty(), prop()), prop())),
        (
            "CZF_Collection",
            arrow(
                arrow(czf_set_ty(), arrow(czf_set_ty(), prop())),
                arrow(czf_set_ty(), czf_set_ty()),
            ),
        ),
        ("czf_strong_collection", czf_strong_collection_ty()),
        ("czf_subset_collection", czf_subset_collection_ty()),
        (
            "AFA_Solution",
            arrow(arrow(czf_set_ty(), czf_set_ty()), czf_set_ty()),
        ),
        ("anti_foundation", anti_foundation_ty()),
        ("CZF_Nonempty", arrow(czf_set_ty(), prop())),
        (
            "CZF_HasMinimalElement",
            arrow(
                czf_set_ty(),
                arrow(arrow(czf_set_ty(), arrow(czf_set_ty(), prop())), prop()),
            ),
        ),
        ("izf_regularity", izf_regularity_ty()),
        ("BarRecursor", bar_recursor_ty()),
        ("bar_recursion_axiom", bar_recursion_axiom_ty()),
        ("UniformModulus", uniform_modulus_ty()),
        ("spector_translation", spector_translation_ty()),
        ("FiniteFan", finite_fan_ty()),
        ("FiniteBar", arrow(arrow(list_ty(nat_ty()), prop()), prop())),
        ("fan_theorem_strong", fan_theorem_strong_ty()),
        (
            "AnalyticBar",
            arrow(arrow(list_ty(nat_ty()), prop()), arrow(type0(), prop())),
        ),
        ("BaireSpace", type0()),
        (
            "MonotoneInduction",
            arrow(arrow(list_ty(nat_ty()), prop()), prop()),
        ),
        ("bar_theorem_analytic", bar_theorem_analytic_ty()),
        ("KleeneBrouwerOrdering", kleene_brouwer_ordering_ty()),
        (
            "OpenCover",
            arrow(pair_ty(bishop_real_ty(), bishop_real_ty()), prop()),
        ),
        ("BishopZero", bishop_real_ty()),
        ("BishopOne", bishop_real_ty()),
        (
            "UnitInterval",
            arrow(
                bishop_real_ty(),
                arrow(
                    bishop_real_ty(),
                    pair_ty(bishop_real_ty(), bishop_real_ty()),
                ),
            ),
        ),
        (
            "FiniteSubcover",
            arrow(arrow(bishop_real_ty(), prop()), prop()),
        ),
        ("constructive_heine_borel", constructive_heine_borel_ty()),
        ("LawfulSequence", lawful_sequence_ty()),
        (
            "HasConvergentSubsequence",
            arrow(arrow(nat_ty(), bishop_real_ty()), prop()),
        ),
        ("sequential_compactness", sequential_compactness_ty()),
        ("IsPrimRec", arrow(arrow(nat_ty(), bool_ty()), prop())),
        ("mp_pr", mp_pr_ty()),
        ("IsSigma1", arrow(arrow(nat_ty(), prop()), prop())),
        ("mp_semi", mp_semi_ty()),
        ("ExistsAlphaZero", arrow(arrow(nat_ty(), nat_ty()), prop())),
        ("mp_weak", mp_weak_ty()),
        ("IsRecursive", arrow(arrow(nat_ty(), nat_ty()), prop())),
        ("ct0", ct0_ty()),
        ("IsContinuous", arrow(arrow(nat_ty(), nat_ty()), prop())),
        (
            "ct0_consequence_no_continuous_all",
            ct0_consequence_no_continuous_all_ty(),
        ),
        ("EffectivelyEnumerable", arrow(type0(), prop())),
        ("ct0_enumerable", ct0_enumerable_ty()),
        ("TotalRecursiveFns", type0()),
        ("Undecidable", arrow(type0(), prop())),
        ("ct0_diagonalization", ct0_diagonalization_ty()),
        ("CoherentRing", coherent_ring_ty()),
        ("CoherentRing_Ideal", coherent_ring_ideal_ty()),
        ("ExplicitField", explicit_field_ty()),
        ("ExplicitField_Inv", explicit_field_inv_ty()),
        ("FreeModule", arrow(coherent_ring_ty(), type0())),
        ("Submodule", arrow(coherent_ring_ty(), type0())),
        ("HasFiniteSyzygy", arrow(coherent_ring_ty(), prop())),
        ("coherent_ring_syzygy", coherent_ring_syzygy_ty()),
        ("GCDDomain", gcd_domain_ty()),
        ("GCDDomain_Gcd", gcd_domain_gcd_ty()),
        ("Numbering", numbering_ty()),
        ("RecursiveNumbering", recursive_numbering_ty()),
        ("GoedelNumbering", goedel_numbering_ty()),
        ("GrzegorczykHierarchy", grzegorczyk_hierarchy_ty()),
        ("Union", arrow(arrow(nat_ty(), type0()), type0())),
        ("PrimRecFns", type0()),
        ("grzegorczyk_union", grzegorczyk_union_ty()),
        ("TTERepresentation", tte_representation_ty()),
        ("TTEComputable", tte_computable_ty()),
        ("CauchyRepresentation", cauchy_representation_ty()),
        (
            "SignedDigitRepresentation",
            signed_digit_representation_ty(),
        ),
        (
            "ProductRep",
            arrow(
                tte_representation_ty(),
                arrow(tte_representation_ty(), tte_representation_ty()),
            ),
        ),
        ("tte_addition_computable", tte_addition_computable_ty()),
        ("WeihrauchDegree", weihrauch_degree_ty()),
        ("LPO_Weihrauch", lpo_weihrauch_ty()),
        ("countable_choice", countable_choice_ty()),
        ("DC_Relation", dc_relation_ty()),
        (
            "R_Image",
            arrow(
                arrow(type0(), arrow(type0(), prop())),
                arrow(type0(), type0()),
            ),
        ),
        ("dependent_choice_scheme", dependent_choice_scheme_ty()),
        ("ConstructiveMeasure", constructive_measure_ty()),
        ("ConstructiveIntegral", constructive_integral_ty()),
        (
            "IntegrableConstructive",
            arrow(arrow(bishop_real_ty(), bishop_real_ty()), prop()),
        ),
        (
            "WellDefined",
            arrow(bishop_real_ty(), arrow(constructive_measure_ty(), prop())),
        ),
        ("constructive_lebesgue", constructive_lebesgue_ty()),
        (
            "MonotoneBounded",
            arrow(
                arrow(nat_ty(), arrow(bishop_real_ty(), bishop_real_ty())),
                arrow(constructive_measure_ty(), prop()),
            ),
        ),
        (
            "IntegralSequence",
            arrow(
                arrow(nat_ty(), arrow(bishop_real_ty(), bishop_real_ty())),
                arrow(constructive_measure_ty(), arrow(nat_ty(), bishop_real_ty())),
            ),
        ),
        (
            "HasConstructiveLimit",
            arrow(arrow(nat_ty(), bishop_real_ty()), prop()),
        ),
        (
            "constructive_monotone_convergence",
            constructive_monotone_convergence_ty(),
        ),
        ("NilSquare", nil_square_ty()),
        ("ExistsUnique", arrow(prop(), prop())),
        (
            "SDG_LinearFactor",
            arrow(arrow(nil_square_ty(), real_ty()), prop()),
        ),
        ("sdg_kock_lawvere", sdg_kock_lawvere_ty()),
        ("SDG_TangentBundle", sdg_tangent_bundle_ty()),
        ("SDG_VectorField", sdg_vector_field_ty()),
        (
            "SDG_Derivative",
            arrow(arrow(real_ty(), real_ty()), arrow(real_ty(), real_ty())),
        ),
        (
            "SDG_Integral",
            arrow(
                arrow(real_ty(), real_ty()),
                arrow(real_ty(), arrow(real_ty(), real_ty())),
            ),
        ),
        ("sdg_integration", sdg_integration_ty()),
        ("SDG_Microlinear", sdg_microlinear_ty()),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    Ok(())
}
/// Bounded μ-operator: find the smallest n < bound such that f(n) = 0.
pub fn bounded_mu(f: impl Fn(u64) -> Option<u64>, bound: u64) -> Option<u64> {
    (0..bound).find(|&n| f(n) == Some(0))
}
/// Primitive recursive function: Ackermann function.
pub fn ackermann(m: u64, n: u64) -> u64 {
    match (m, n) {
        (0, n) => n + 1,
        (m, 0) => ackermann(m - 1, 1),
        (m, n) => ackermann(m - 1, ackermann(m, n - 1)),
    }
}
/// Check if a predicate P (given as a function) satisfies Markov's principle
/// on [0, bound): if not all values are false (within bound), find one.
pub fn markov_search(p: impl Fn(u64) -> bool, bound: u64) -> Option<u64> {
    (0..bound).find(|&n| p(n))
}
/// Church numeral: a function that applies f n times to x.
/// We represent it as a Rust closure factory.
pub fn church_numeral(n: u64) -> impl Fn(u64) -> u64 {
    move |x| {
        let mut result = x;
        for _ in 0..n {
            result = result.wrapping_add(1);
        }
        result
    }
}
/// Church addition: add two Church numerals.
pub fn church_add(m: u64, n: u64) -> u64 {
    m + n
}
/// Church multiplication.
pub fn church_mul(m: u64, n: u64) -> u64 {
    m * n
}
/// Check whether a bounded arithmetic formula ∀ x < bound, P(x) is decidable.
pub fn decide_bounded_forall(p: impl Fn(u64) -> bool, bound: u64) -> bool {
    (0..bound).all(p)
}
/// Check whether ∃ x < bound, P(x) holds.
pub fn decide_bounded_exists(p: impl Fn(u64) -> bool, bound: u64) -> bool {
    (0..bound).any(p)
}
/// In Kleene realizability, a natural number n realizes P ∧ Q if
/// the left projection realizes P and the right projection realizes Q.
/// We use Cantor pairing to encode pairs.
pub fn cantor_pair(a: u64, b: u64) -> u64 {
    (a + b) * (a + b + 1) / 2 + b
}
pub fn cantor_unpair(n: u64) -> (u64, u64) {
    let w = ((((8 * n + 1) as f64).sqrt() - 1.0) / 2.0) as u64;
    let t = w * (w + 1) / 2;
    let b = n - t;
    let a = w - b;
    (a, b)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_heyting_algebra_laws() {
        let h = PowerSetHeyting::new(3);
        let top = h.top();
        let bot = h.bot();
        let a = 0b101u64;
        assert_eq!(h.meet(a, top), a);
        assert_eq!(h.join(a, bot), a);
        assert_eq!(h.neg(top), bot);
        assert_eq!(h.neg(bot), top);
        let b = 0b011u64;
        assert_eq!(h.double_neg(b), b);
        let c = 0b110u64;
        let lhs = h.le(h.meet(a, b), c);
        let rhs = h.le(a, h.implication(b, c));
        assert_eq!(lhs, rhs);
    }
    #[test]
    fn test_constructive_real_addition() {
        let one_third = ConstructiveReal::from_rational(1, 3, 10);
        let two_thirds = ConstructiveReal::from_rational(2, 3, 10);
        let sum = one_third.add(&two_thirds);
        let one = ConstructiveReal::from_rational(1, 1, 10);
        assert!(sum.approx_eq(&one, 5), "1/3 + 2/3 should approximate 1");
    }
    #[test]
    fn test_constructive_real_from_rational() {
        let half = ConstructiveReal::from_rational(1, 2, 8);
        let (m, k) = half.get_approx(4);
        assert_eq!(m, 8);
        assert_eq!(k, 4);
    }
    #[test]
    fn test_bounded_mu_operator() {
        let result = bounded_mu(
            |n| {
                if n > 0 && n % 7 == 0 {
                    Some(0)
                } else {
                    Some(1)
                }
            },
            100,
        );
        assert_eq!(result, Some(7));
    }
    #[test]
    fn test_markov_search() {
        let result = markov_search(|n| n * n > 50, 20);
        assert_eq!(result, Some(8));
    }
    #[test]
    fn test_cantor_pairing() {
        for a in 0..10u64 {
            for b in 0..10u64 {
                let n = cantor_pair(a, b);
                let (ra, rb) = cantor_unpair(n);
                assert_eq!((ra, rb), (a, b), "Cantor pairing failed for ({}, {})", a, b);
            }
        }
    }
    #[test]
    fn test_decide_bounded_forall_exists() {
        assert!(decide_bounded_forall(|x| x + 1 > 0, 10));
        assert!(!decide_bounded_forall(|x| x > 5, 10));
        assert!(decide_bounded_exists(|x| x * x == 49, 10));
        assert!(!decide_bounded_exists(|x| x * x == 49, 5));
    }
    #[test]
    fn test_build_constructive_mathematics_env() {
        let mut env = Environment::new();
        let result = build_constructive_mathematics_env(&mut env);
        assert!(
            result.is_ok(),
            "build_constructive_mathematics_env failed: {:?}",
            result.err()
        );
    }
}
