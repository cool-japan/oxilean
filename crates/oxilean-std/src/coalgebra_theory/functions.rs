//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    BisimRelation, BisimulationChecker, CoList, FinalCoalgebraApprox, HopfAlgebraOps, HyperSet,
    OptionStream, ProbabilisticAutomaton, ProductivityChecker, Stream, StreamCoalgebra, LTS,
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
pub fn option_ty(elem: Expr) -> Expr {
    app(cst("Option"), elem)
}
/// Functor: endofunctor on Type.
/// Type: (Type → Type) → Prop
pub fn functor_ty() -> Expr {
    arrow(arrow(type0(), type0()), prop())
}
/// FCoalgebra: a pair (A, α : A → F A) for an endofunctor F.
/// Type: (Type → Type) → Type
pub fn f_coalgebra_ty() -> Expr {
    arrow(arrow(type0(), type0()), type0())
}
/// CarrierOf: the carrier set of a coalgebra.
/// Type: FCoalgebra F → Type
pub fn carrier_of_ty() -> Expr {
    arrow(cst("FCoalgebra"), type0())
}
/// StructureMap: the structure map α : A → F A.
/// Type: ∀ (c : FCoalgebra F), CarrierOf c → F (CarrierOf c)
pub fn structure_map_ty() -> Expr {
    impl_pi(
        "c",
        cst("FCoalgebra"),
        arrow(
            app(cst("CarrierOf"), bvar(0)),
            app(cst("F"), app(cst("CarrierOf"), bvar(1))),
        ),
    )
}
/// CoalgebraMorphism: a map f : A → B respecting structure maps.
/// Type: FCoalgebra F → FCoalgebra F → Type
pub fn coalgebra_morphism_ty() -> Expr {
    arrow(cst("FCoalgebra"), arrow(cst("FCoalgebra"), type0()))
}
/// MorphismCondition: f commutes with structure maps.
/// Type: ∀ c d, CoalgebraMorphism c d → Prop
pub fn morphism_condition_ty() -> Expr {
    impl_pi(
        "c",
        cst("FCoalgebra"),
        impl_pi(
            "d",
            cst("FCoalgebra"),
            arrow(app2(cst("CoalgebraMorphism"), bvar(1), bvar(0)), prop()),
        ),
    )
}
/// Bisimulation: a relation R on the carrier that is preserved by F.
/// Type: FCoalgebra F → FCoalgebra F → (CarrierOf c1 → CarrierOf c2 → Prop) → Prop
pub fn bisimulation_ty() -> Expr {
    arrow(
        cst("FCoalgebra"),
        arrow(
            cst("FCoalgebra"),
            arrow(arrow(type0(), arrow(type0(), prop())), prop()),
        ),
    )
}
/// Bisimilar: the greatest bisimulation (coinductively defined).
/// Type: FCoalgebra F → CarrierOf c → CarrierOf c → Prop
pub fn bisimilar_ty() -> Expr {
    arrow(cst("FCoalgebra"), arrow(type0(), arrow(type0(), prop())))
}
/// BisimilarityCoind: bisimilarity is the greatest bisimulation.
/// Type: ∀ c (R : _ → _ → Prop), Bisimulation c c R → ∀ x y, R x y → Bisimilar c x y
pub fn bisimilarity_coind_ty() -> Expr {
    impl_pi(
        "c",
        cst("FCoalgebra"),
        arrow(
            arrow(type0(), arrow(type0(), prop())),
            arrow(
                app3(cst("Bisimulation"), bvar(1), bvar(1), bvar(0)),
                impl_pi(
                    "x",
                    type0(),
                    impl_pi(
                        "y",
                        type0(),
                        arrow(
                            app2(bvar(3), bvar(1), bvar(0)),
                            app3(cst("Bisimilar"), bvar(5), bvar(2), bvar(1)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// BisimUpTo: bisimulation up-to bisimilarity.
/// Type: FCoalgebra F → (CarrierOf c → CarrierOf c → Prop) → Prop
pub fn bisim_up_to_ty() -> Expr {
    arrow(
        cst("FCoalgebra"),
        arrow(arrow(type0(), arrow(type0(), prop())), prop()),
    )
}
/// GFP: greatest fixed point of a monotone predicate transformer.
/// Type: (Prop → Prop) → Prop
pub fn gfp_ty() -> Expr {
    arrow(arrow(prop(), prop()), prop())
}
/// GFPCharacterization: ν F = ⋂ { X | X ⊆ F X } (Knaster-Tarski dual).
/// Type: ∀ (Φ : Prop → Prop), Monotone Φ → GFP Φ ↔ (∀ X, X → Φ X → X)
pub fn gfp_characterization_ty() -> Expr {
    impl_pi(
        "phi",
        arrow(prop(), prop()),
        arrow(
            app(cst("Monotone"), bvar(0)),
            app2(
                cst("Iff"),
                app(cst("GFP"), bvar(1)),
                arrow(prop(), arrow(arrow(prop(), bvar(2)), prop())),
            ),
        ),
    )
}
/// CoinductionPrinciple: to prove GFP Φ, it suffices to find an invariant X with X ⊆ Φ X.
/// Type: ∀ (Φ : Prop → Prop), Monotone Φ → ∀ (X : Prop), X → Φ X → GFP Φ
pub fn coinduction_principle_ty() -> Expr {
    impl_pi(
        "phi",
        arrow(prop(), prop()),
        arrow(
            app(cst("Monotone"), bvar(0)),
            impl_pi(
                "x",
                prop(),
                arrow(
                    bvar(0),
                    arrow(app(bvar(2), bvar(1)), app(cst("GFP"), bvar(3))),
                ),
            ),
        ),
    )
}
/// LFP: least fixed point (for contrast).
/// Type: (Prop → Prop) → Prop
pub fn lfp_ty() -> Expr {
    arrow(arrow(prop(), prop()), prop())
}
/// Monotone: a function on Prop is monotone.
/// Type: (Prop → Prop) → Prop
pub fn monotone_ty() -> Expr {
    arrow(arrow(prop(), prop()), prop())
}
/// TerminalCoalgebra: the final F-coalgebra.
/// Type: (Type → Type) → FCoalgebra F
pub fn terminal_coalgebra_ty() -> Expr {
    arrow(arrow(type0(), type0()), cst("FCoalgebra"))
}
/// FinalMorphism: the unique morphism from any coalgebra to the terminal one.
/// Type: ∀ (c : FCoalgebra F), CoalgebraMorphism c (TerminalCoalgebra F)
pub fn final_morphism_ty() -> Expr {
    impl_pi(
        "c",
        cst("FCoalgebra"),
        app2(
            cst("CoalgebraMorphism"),
            bvar(0),
            app(cst("TerminalCoalgebra"), cst("F")),
        ),
    )
}
/// FinalUniqueness: the final morphism is unique.
/// Type: ∀ c (f g : CoalgebraMorphism c (TerminalCoalgebra F)), f = g
pub fn final_uniqueness_ty() -> Expr {
    impl_pi(
        "c",
        cst("FCoalgebra"),
        arrow(
            app2(
                cst("CoalgebraMorphism"),
                bvar(0),
                app(cst("TerminalCoalgebra"), cst("F")),
            ),
            arrow(
                app2(
                    cst("CoalgebraMorphism"),
                    bvar(1),
                    app(cst("TerminalCoalgebra"), cst("F")),
                ),
                prop(),
            ),
        ),
    )
}
/// LambeksLemma: the structure map of the terminal coalgebra is an isomorphism.
/// Type: ∀ (tz : TerminalCoalgebra F), IsIso (StructureMap tz)
pub fn lambeks_lemma_ty() -> Expr {
    arrow(
        app(cst("TerminalCoalgebra"), cst("F")),
        app(cst("IsIso"), app(cst("StructureMap"), bvar(0))),
    )
}
/// Stream: the type of infinite sequences over A.
/// Type: Type → Type
pub fn stream_ty() -> Expr {
    arrow(type0(), type0())
}
/// StreamHead: the first element of a stream.
/// Type: Stream A → A
pub fn stream_head_ty() -> Expr {
    impl_pi("a", type0(), arrow(app(cst("Stream"), bvar(0)), bvar(1)))
}
/// StreamTail: the rest of the stream after the first element.
/// Type: Stream A → Stream A
pub fn stream_tail_ty() -> Expr {
    impl_pi(
        "a",
        type0(),
        arrow(app(cst("Stream"), bvar(0)), app(cst("Stream"), bvar(1))),
    )
}
/// StreamCons: prepend an element to a stream.
/// Type: A → Stream A → Stream A
pub fn stream_cons_ty() -> Expr {
    impl_pi(
        "a",
        type0(),
        arrow(
            bvar(0),
            arrow(app(cst("Stream"), bvar(1)), app(cst("Stream"), bvar(2))),
        ),
    )
}
/// StreamCoalgebra: streams form an F-coalgebra for F(X) = A × X.
/// Type: ∀ (A : Type), FCoalgebra (ProdFunctor A)
pub fn stream_coalgebra_ty() -> Expr {
    impl_pi(
        "a",
        type0(),
        app(cst("FCoalgebra"), app(cst("ProdFunctor"), bvar(0))),
    )
}
/// StreamBisim: two streams are bisimilar iff they are equal (extensionally).
/// Type: ∀ (A : Type) (s t : Stream A), Bisimilar (StreamCoalgebra A) s t ↔ s = t
pub fn stream_bisim_ty() -> Expr {
    impl_pi(
        "a",
        type0(),
        impl_pi(
            "s",
            app(cst("Stream"), bvar(0)),
            impl_pi(
                "t",
                app(cst("Stream"), bvar(1)),
                app2(
                    cst("Iff"),
                    app3(
                        cst("Bisimilar"),
                        app(cst("StreamCoalgebra"), bvar(2)),
                        bvar(1),
                        bvar(0),
                    ),
                    app2(cst("Eq"), bvar(2), bvar(1)),
                ),
            ),
        ),
    )
}
/// StreamNth: nth element of a stream.
/// Type: Stream A → Nat → A
pub fn stream_nth_ty() -> Expr {
    impl_pi(
        "a",
        type0(),
        arrow(app(cst("Stream"), bvar(0)), arrow(nat_ty(), bvar(1))),
    )
}
/// StreamMap: apply a function to every element.
/// Type: (A → B) → Stream A → Stream B
pub fn stream_map_ty() -> Expr {
    impl_pi(
        "a",
        type0(),
        impl_pi(
            "b",
            type0(),
            arrow(
                arrow(bvar(1), bvar(0)),
                arrow(app(cst("Stream"), bvar(2)), app(cst("Stream"), bvar(2))),
            ),
        ),
    )
}
/// StreamZip: zip two streams elementwise.
/// Type: (A → B → C) → Stream A → Stream B → Stream C
pub fn stream_zip_ty() -> Expr {
    arrow(
        type0(),
        arrow(
            type0(),
            arrow(
                type0(),
                arrow(
                    arrow(bvar(2), arrow(bvar(1), bvar(0))),
                    arrow(
                        app(cst("Stream"), bvar(3)),
                        arrow(app(cst("Stream"), bvar(3)), app(cst("Stream"), bvar(3))),
                    ),
                ),
            ),
        ),
    )
}
/// Anamorphism: the corecursive unfold operator.
/// Type: (S → F S) → S → Terminal F
pub fn anamorphism_ty() -> Expr {
    arrow(
        arrow(type0(), app(cst("F"), type0())),
        arrow(type0(), app(cst("TerminalCoalgebra"), cst("F"))),
    )
}
/// Apomorphism: generalized anamorphism (can return a fixed value or continue).
/// Type: (S → F (Terminal F + S)) → S → Terminal F
pub fn apomorphism_ty() -> Expr {
    arrow(
        arrow(
            type0(),
            app(
                cst("F"),
                app2(cst("Sum"), app(cst("TerminalCoalgebra"), cst("F")), type0()),
            ),
        ),
        arrow(type0(), app(cst("TerminalCoalgebra"), cst("F"))),
    )
}
/// FusionLaw: two anamorphisms can be fused into one if the coalgebra morphism commutes.
/// Type: ∀ (f : S → F S) (g : T → F T) (h : S → T),
///         (∀ s, F h (f s) = g (h s)) → ana f = ana g ∘ h
pub fn fusion_law_ty() -> Expr {
    arrow(
        arrow(type0(), app(cst("F"), type0())),
        arrow(
            arrow(type0(), app(cst("F"), type0())),
            arrow(
                arrow(type0(), type0()),
                arrow(
                    impl_pi(
                        "s",
                        type0(),
                        app2(
                            cst("Eq"),
                            app(app(cst("Fmap"), cst("h")), app(cst("f"), bvar(0))),
                            app(cst("g"), app(cst("h"), bvar(1))),
                        ),
                    ),
                    prop(),
                ),
            ),
        ),
    )
}
/// Later: Nakano's later modality ▷A.
/// Type: Prop → Prop
pub fn later_ty() -> Expr {
    arrow(prop(), prop())
}
/// GuardedFix: the guarded fixed point operator.
/// Type: (▷A → A) → A
pub fn guarded_fix_ty() -> Expr {
    impl_pi(
        "a",
        prop(),
        arrow(arrow(app(cst("Later"), bvar(0)), bvar(1)), bvar(1)),
    )
}
/// LaterMonotone: if A implies B then ▷A implies ▷B.
/// Type: ∀ (A B : Prop), (A → B) → ▷A → ▷B
pub fn later_monotone_ty() -> Expr {
    impl_pi(
        "a",
        prop(),
        impl_pi(
            "b",
            prop(),
            arrow(
                arrow(bvar(1), bvar(0)),
                arrow(app(cst("Later"), bvar(2)), app(cst("Later"), bvar(2))),
            ),
        ),
    )
}
/// LaterIntro: A implies ▷A.
/// Type: ∀ (A : Prop), A → ▷A
pub fn later_intro_ty() -> Expr {
    impl_pi("a", prop(), arrow(bvar(0), app(cst("Later"), bvar(1))))
}
/// LöbInduction: (▷A → A) → A.
/// Type: ∀ (A : Prop), (▷A → A) → A
pub fn lob_induction_ty() -> Expr {
    impl_pi(
        "a",
        prop(),
        arrow(arrow(app(cst("Later"), bvar(0)), bvar(1)), bvar(1)),
    )
}
/// Ordinal: an ordinal bound for sized types.
pub fn ordinal_ty() -> Expr {
    type0()
}
/// SizedType: a type indexed by an ordinal size bound.
/// Type: Ordinal → Type
pub fn sized_type_ty() -> Expr {
    arrow(cst("Ordinal"), type0())
}
/// SizeSucc: successor ordinal bound.
pub fn size_succ_ty() -> Expr {
    arrow(cst("Ordinal"), cst("Ordinal"))
}
/// SizeOmega: the limit ordinal ω.
pub fn size_omega_ty() -> Expr {
    cst("Ordinal")
}
/// SizedStream: a stream at a given size.
/// Type: Type → Ordinal → Type
pub fn sized_stream_ty() -> Expr {
    arrow(type0(), arrow(cst("Ordinal"), type0()))
}
/// SizedFix: fixed point of a size-indexed functor.
/// Type: (Ordinal → Type → Type) → Ordinal → Type
pub fn sized_fix_ty() -> Expr {
    arrow(
        arrow(cst("Ordinal"), arrow(type0(), type0())),
        arrow(cst("Ordinal"), type0()),
    )
}
/// CoList: a (possibly infinite) list.
/// Type: Type → Type
pub fn colist_ty() -> Expr {
    arrow(type0(), type0())
}
/// CoListNil: the empty colist.
/// Type: CoList A
pub fn colist_nil_ty() -> Expr {
    impl_pi("a", type0(), app(cst("CoList"), bvar(0)))
}
/// CoListCons: prepend an element to a colist.
/// Type: A → CoList A → CoList A
pub fn colist_cons_ty() -> Expr {
    impl_pi(
        "a",
        type0(),
        arrow(
            bvar(0),
            arrow(app(cst("CoList"), bvar(1)), app(cst("CoList"), bvar(2))),
        ),
    )
}
/// CoTree: an infinitely-branching tree.
/// Type: Type → Type → Type
pub fn cotree_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// CoTreeNode: construct a cotree node.
/// Type: A → (B → CoTree A B) → CoTree A B
pub fn cotree_node_ty() -> Expr {
    arrow(
        type0(),
        arrow(
            type0(),
            arrow(
                bvar(1),
                arrow(
                    arrow(bvar(0), app2(cst("CoTree"), bvar(2), bvar(1))),
                    app2(cst("CoTree"), bvar(3), bvar(2)),
                ),
            ),
        ),
    )
}
/// NonWellFoundedSet: a set allowing non-well-founded membership.
pub fn non_wf_set_ty() -> Expr {
    type0()
}
/// BisimEquality: equality of non-well-founded sets is bisimilarity.
pub fn bisim_equality_ty() -> Expr {
    arrow(
        cst("NonWellFoundedSet"),
        arrow(cst("NonWellFoundedSet"), prop()),
    )
}
/// BisimCoinduction: coinduction proof principle via bisimulation.
/// Type: ∀ (c : FCoalgebra F) (R : Carrier → Carrier → Prop),
///         BisimUpTo c R → ∀ x y, R x y → Bisimilar c x y
pub fn bisim_coinduction_ty() -> Expr {
    impl_pi(
        "c",
        cst("FCoalgebra"),
        arrow(
            arrow(type0(), arrow(type0(), prop())),
            arrow(
                app2(cst("BisimUpTo"), bvar(1), bvar(0)),
                arrow(
                    type0(),
                    arrow(type0(), arrow(app2(bvar(2), bvar(1), bvar(0)), prop())),
                ),
            ),
        ),
    )
}
/// BisimUpToRefl: reflexivity closure is a bisimulation up-to.
/// Type: ∀ (c : FCoalgebra F), BisimUpTo c (λ x y, x = y)
pub fn bisim_up_to_refl_ty() -> Expr {
    impl_pi(
        "c",
        cst("FCoalgebra"),
        app2(
            cst("BisimUpTo"),
            bvar(0),
            arrow(type0(), arrow(type0(), prop())),
        ),
    )
}
/// BisimUpToTrans: transitivity closure preserves bisimulation up-to.
/// Type: ∀ (c : FCoalgebra F) (R : _ → _ → Prop),
///         BisimUpTo c R → BisimUpTo c (transitiveClosure R)
pub fn bisim_up_to_trans_ty() -> Expr {
    impl_pi(
        "c",
        cst("FCoalgebra"),
        arrow(
            arrow(type0(), arrow(type0(), prop())),
            arrow(
                app2(cst("BisimUpTo"), bvar(1), bvar(0)),
                app2(
                    cst("BisimUpTo"),
                    bvar(2),
                    app(cst("TransitiveClosure"), bvar(1)),
                ),
            ),
        ),
    )
}
/// CoinductiveProofPrinciple: to prove bisimilarity it suffices to exhibit a bisimulation.
/// Type: ∀ (R : Carrier → Carrier → Prop), Bisimulation c c R → ∀ x y, R x y → x ~ y
pub fn coinductive_proof_principle_ty() -> Expr {
    arrow(
        arrow(type0(), arrow(type0(), prop())),
        arrow(
            app3(cst("Bisimulation"), cst("c"), cst("c"), bvar(0)),
            arrow(
                type0(),
                arrow(type0(), arrow(app2(bvar(3), bvar(1), bvar(0)), prop())),
            ),
        ),
    )
}
/// FinalCoalgebraTheorem: every F-coalgebra has a unique morphism into the final coalgebra.
/// Type: ∀ (F : Type → Type) (c : FCoalgebra F),
///         ∃! (h : CoalgebraMorphism c (TerminalCoalgebra F)), True
pub fn final_coalgebra_theorem_ty() -> Expr {
    arrow(
        arrow(type0(), type0()),
        arrow(
            cst("FCoalgebra"),
            app(
                cst("ExistsUnique"),
                app2(
                    cst("CoalgebraMorphism"),
                    bvar(0),
                    app(cst("TerminalCoalgebra"), bvar(1)),
                ),
            ),
        ),
    )
}
/// LambeksLemmaCoalg: the structure map of the terminal coalgebra is an isomorphism.
/// Type: IsIso (StructureMap (TerminalCoalgebra F))
pub fn lambeks_lemma_coalg_ty() -> Expr {
    app(
        cst("IsIso"),
        app(cst("StructureMap"), app(cst("TerminalCoalgebra"), cst("F"))),
    )
}
/// TerminalCoalgebraUnfold: the terminal coalgebra unfold law.
/// Type: ∀ (c : FCoalgebra F) (x : Carrier c),
///         StructureMap (ana x) = Fmap (ana) (StructureMap x)
pub fn terminal_coalgebra_unfold_ty() -> Expr {
    impl_pi(
        "c",
        cst("FCoalgebra"),
        arrow(
            app(cst("CarrierOf"), bvar(0)),
            app2(
                cst("Eq"),
                app(cst("StructureMap"), app(cst("ana"), bvar(0))),
                app2(cst("Fmap"), cst("ana"), app(cst("StructureMap"), bvar(1))),
            ),
        ),
    )
}
/// GuardedRecursion: a guarded recursive definition is productive.
/// Type: ∀ (A : Type), (▷A → A) → A
pub fn guarded_recursion_ty() -> Expr {
    impl_pi(
        "a",
        type0(),
        arrow(arrow(app(cst("Later"), bvar(0)), bvar(1)), bvar(1)),
    )
}
/// Productivity: a corecursive definition produces each output in finite time.
/// Type: ∀ (f : Nat → A) (n : Nat), f n is defined in finite steps
pub fn productivity_ty() -> Expr {
    arrow(arrow(nat_ty(), type0()), arrow(nat_ty(), prop()))
}
/// CopatternMatching: defining coinductive functions by observations.
/// Type: ∀ (A : Type), (∀ obs, ObsType obs A → A) → A
pub fn copattern_matching_ty() -> Expr {
    impl_pi(
        "a",
        type0(),
        arrow(
            arrow(
                cst("Obs"),
                arrow(app2(cst("ObsType"), cst("obs"), bvar(0)), bvar(1)),
            ),
            bvar(1),
        ),
    )
}
/// CoquandGuardCondition: Coquand's syntactic guard condition for corecursion.
/// Type: Prop (a formal property of corecursive definitions)
pub fn coquand_guard_condition_ty() -> Expr {
    prop()
}
/// StreamDiffEq: a stream satisfies a differential equation s' = f s.
/// Type: ∀ (A : Type) (f : Stream A → Stream A) (s : Stream A),
///         StreamTail s = f s → StreamDiffSol f s
pub fn stream_diff_eq_ty() -> Expr {
    impl_pi(
        "a",
        type0(),
        arrow(
            arrow(app(cst("Stream"), bvar(0)), app(cst("Stream"), bvar(1))),
            arrow(
                app(cst("Stream"), bvar(1)),
                arrow(
                    app2(
                        cst("Eq"),
                        app(cst("StreamTail"), bvar(0)),
                        app(bvar(1), bvar(0)),
                    ),
                    app2(cst("StreamDiffSol"), bvar(2), bvar(1)),
                ),
            ),
        ),
    )
}
/// StreamBisimCoinduction: stream bisimilarity implies stream equality.
/// Type: ∀ (A : Type) (R : Stream A → Stream A → Prop),
///         StreamBisimulation R → ∀ s t, R s t → s = t
pub fn stream_bisim_coinduction_ty() -> Expr {
    impl_pi(
        "a",
        type0(),
        arrow(
            arrow(
                app(cst("Stream"), bvar(0)),
                arrow(app(cst("Stream"), bvar(1)), prop()),
            ),
            arrow(
                app(cst("StreamBisimulation"), bvar(0)),
                arrow(
                    app(cst("Stream"), bvar(1)),
                    arrow(
                        app(cst("Stream"), bvar(2)),
                        arrow(
                            app2(bvar(3), bvar(1), bvar(0)),
                            app2(cst("Eq"), bvar(2), bvar(1)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// StreamFunctor: the stream functor S ↦ A × S.
pub fn stream_functor_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// MooreMachine: a Moore machine as a coalgebra for F(Q) = O × Q^I.
/// Type: Type → Type → Type → Type  (states, inputs, outputs)
pub fn moore_machine_ty() -> Expr {
    arrow(type0(), arrow(type0(), arrow(type0(), type0())))
}
/// MealyMachine: a Mealy machine as a coalgebra for F(Q) = (O × Q)^I.
pub fn mealy_machine_ty() -> Expr {
    arrow(type0(), arrow(type0(), arrow(type0(), type0())))
}
/// LanguageEquivalence: two automata are language-equivalent iff bisimilar.
/// Type: ∀ (M1 M2 : MooreMachine Q I O), Bisimilar M1 M2 ↔ LanguageEq M1 M2
pub fn language_equivalence_ty() -> Expr {
    arrow(
        cst("MooreMachine"),
        arrow(
            cst("MooreMachine"),
            app2(
                cst("Iff"),
                app2(cst("Bisimilar"), bvar(1), bvar(0)),
                app2(cst("LanguageEq"), bvar(2), bvar(1)),
            ),
        ),
    )
}
/// AutomataMinimization: every automaton has a minimal bisimulation quotient.
pub fn automata_minimization_ty() -> Expr {
    arrow(cst("MooreMachine"), cst("MooreMachine"))
}
/// MossModality: the nabla (∇) operator in coalgebraic modal logic.
/// Type: F(Prop) → Prop  (Moss's nabla)
pub fn moss_modality_ty() -> Expr {
    arrow(app(cst("F"), prop()), prop())
}
/// NablaOperator: ∇ φ holds at x iff α(x) ∈ F(⟦φ⟧).
pub fn nabla_operator_ty() -> Expr {
    arrow(app(cst("F"), prop()), prop())
}
/// PaigeTarjanBisim: the Paige-Tarjan partition refinement algorithm decides bisimilarity.
/// Type: Nat → Nat → Prop  (states p q are bisimilar iff algorithm says so)
pub fn paige_tarjan_bisim_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// CoalgModalLogicSoundness: the coalgebraic modal logic is sound w.r.t. bisimilarity.
pub fn coalg_modal_logic_soundness_ty() -> Expr {
    prop()
}
/// BehaviouralEquivalence: two elements are behaviourally equivalent iff all observers agree.
pub fn behavioural_equivalence_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// BehaviouralEquivIsBisim: behavioural equivalence coincides with bisimilarity.
pub fn behavioural_equiv_is_bisim_ty() -> Expr {
    prop()
}
/// PSPACEBisimDecision: bisimilarity for finite-state coalgebras is decidable in PSPACE.
pub fn pspace_bisim_decision_ty() -> Expr {
    prop()
}
/// Bialgebra: a structure that is both an algebra and a coalgebra compatibly.
/// Type: Type → Type
pub fn bialgebra_ty() -> Expr {
    arrow(type0(), type0())
}
/// HopfAlgebra: a bialgebra with an antipode map S : H → H.
pub fn hopf_algebra_ty() -> Expr {
    arrow(type0(), type0())
}
/// DistributiveLaw: a natural transformation F G → G F making bialgebras.
pub fn distributive_law_ty() -> Expr {
    arrow(
        arrow(type0(), type0()),
        arrow(arrow(type0(), type0()), arrow(type0(), type0())),
    )
}
/// ConvolutionProduct: (f * g)(x) = μ ∘ (f ⊗ g) ∘ Δ in a bialgebra.
pub fn convolution_product_ty() -> Expr {
    arrow(
        arrow(type0(), type0()),
        arrow(arrow(type0(), type0()), arrow(type0(), type0())),
    )
}
/// HopfAntipode: the antipode S satisfying μ(S ⊗ id)Δ = η ε = μ(id ⊗ S)Δ.
pub fn hopf_antipode_ty() -> Expr {
    arrow(cst("HopfAlgebra"), arrow(type0(), type0()))
}
/// GroupLikeBialgebra: elements g with Δg = g ⊗ g and εg = 1 form a group.
pub fn group_like_bialgebra_ty() -> Expr {
    arrow(cst("HopfAlgebra"), prop())
}
/// InteractionSystem: dependent coalgebra (interaction trees).
/// Type: (Type → Type) → Type → Type
pub fn interaction_system_ty() -> Expr {
    arrow(arrow(type0(), type0()), arrow(type0(), type0()))
}
/// NonDeterministicCoalgebra: coalgebra for F(X) = P(X) (powerset functor).
pub fn nondeterministic_coalgebra_ty() -> Expr {
    arrow(type0(), app(cst("Powerset"), type0()))
}
/// InteractionTreeBind: monadic bind for interaction trees.
pub fn interaction_tree_bind_ty() -> Expr {
    arrow(
        cst("InteractionTree"),
        arrow(
            arrow(type0(), cst("InteractionTree")),
            cst("InteractionTree"),
        ),
    )
}
/// Comonad: a functor W with ε : W A → A and δ : W A → W (W A).
pub fn comonad_ty() -> Expr {
    arrow(arrow(type0(), type0()), prop())
}
/// ComonadExtract: ε : W A → A.
pub fn comonad_extract_ty() -> Expr {
    impl_pi(
        "w",
        arrow(type0(), type0()),
        impl_pi("a", type0(), arrow(app(bvar(1), bvar(0)), bvar(1))),
    )
}
/// ComonadDuplicate: δ : W A → W (W A).
pub fn comonad_duplicate_ty() -> Expr {
    impl_pi(
        "w",
        arrow(type0(), type0()),
        impl_pi(
            "a",
            type0(),
            arrow(app(bvar(1), bvar(0)), app(bvar(2), app(bvar(2), bvar(1)))),
        ),
    )
}
/// ComonadicSemantics: Moggi-style comonadic semantics for effectful computations.
pub fn comonadic_semantics_ty() -> Expr {
    arrow(cst("Comonad"), arrow(type0(), type0()))
}
/// GuardedCorecursion: a guarded corecursive definition has a unique solution.
pub fn guarded_corecursion_ty() -> Expr {
    impl_pi(
        "a",
        type0(),
        arrow(arrow(app(cst("Later"), bvar(0)), bvar(1)), bvar(1)),
    )
}
/// UniquenessOfGuardedSolutions: any two solutions of the same guarded equation are equal.
pub fn uniqueness_of_guarded_solutions_ty() -> Expr {
    prop()
}
/// CoinductiveDataType: a coinductive type defined by observations.
pub fn coinductive_data_type_ty() -> Expr {
    arrow(cst("ObsSignature"), type0())
}
/// Register all coalgebra theory axioms into the kernel environment.
pub fn build_coalgebra_theory_env(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("Functor", functor_ty()),
        ("FCoalgebra", f_coalgebra_ty()),
        ("CarrierOf", carrier_of_ty()),
        ("StructureMap", structure_map_ty()),
        ("CoalgebraMorphism", coalgebra_morphism_ty()),
        ("MorphismCondition", morphism_condition_ty()),
        ("F", arrow(type0(), type0())),
        ("IsIso", arrow(arrow(type0(), type0()), prop())),
        ("Bisimulation", bisimulation_ty()),
        ("Bisimilar", bisimilar_ty()),
        ("BisimilarityCoind", bisimilarity_coind_ty()),
        ("BisimUpTo", bisim_up_to_ty()),
        ("GFP", gfp_ty()),
        ("GFPCharacterization", gfp_characterization_ty()),
        ("CoinductionPrinciple", coinduction_principle_ty()),
        ("LFP", lfp_ty()),
        ("Monotone", monotone_ty()),
        ("Iff", arrow(prop(), arrow(prop(), prop()))),
        ("Eq", arrow(type0(), arrow(type0(), prop()))),
        ("TerminalCoalgebra", terminal_coalgebra_ty()),
        ("FinalMorphism", final_morphism_ty()),
        ("FinalUniqueness", final_uniqueness_ty()),
        ("LambeksLemma", lambeks_lemma_ty()),
        ("Stream", stream_ty()),
        ("StreamHead", stream_head_ty()),
        ("StreamTail", stream_tail_ty()),
        ("StreamCons", stream_cons_ty()),
        ("StreamCoalgebra", stream_coalgebra_ty()),
        ("StreamBisim", stream_bisim_ty()),
        ("StreamNth", stream_nth_ty()),
        ("StreamMap", stream_map_ty()),
        ("StreamZip", stream_zip_ty()),
        ("ProdFunctor", arrow(type0(), arrow(type0(), type0()))),
        ("Anamorphism", anamorphism_ty()),
        ("Apomorphism", apomorphism_ty()),
        ("FusionLaw", fusion_law_ty()),
        (
            "Fmap",
            arrow(
                arrow(type0(), type0()),
                arrow(arrow(type0(), type0()), arrow(type0(), type0())),
            ),
        ),
        ("Sum", arrow(type0(), arrow(type0(), type0()))),
        ("Later", later_ty()),
        ("GuardedFix", guarded_fix_ty()),
        ("LaterMonotone", later_monotone_ty()),
        ("LaterIntro", later_intro_ty()),
        ("LobInduction", lob_induction_ty()),
        ("Ordinal", ordinal_ty()),
        ("SizedType", sized_type_ty()),
        ("SizeSucc", size_succ_ty()),
        ("SizeOmega", size_omega_ty()),
        ("SizedStream", sized_stream_ty()),
        ("SizedFix", sized_fix_ty()),
        ("CoList", colist_ty()),
        ("CoListNil", colist_nil_ty()),
        ("CoListCons", colist_cons_ty()),
        ("CoTree", cotree_ty()),
        ("CoTreeNode", cotree_node_ty()),
        ("NonWellFoundedSet", non_wf_set_ty()),
        ("BisimEquality", bisim_equality_ty()),
        ("BisimCoinduction", bisim_coinduction_ty()),
        ("BisimUpToRefl", bisim_up_to_refl_ty()),
        ("BisimUpToTrans", bisim_up_to_trans_ty()),
        (
            "CoinductiveProofPrinciple",
            coinductive_proof_principle_ty(),
        ),
        (
            "TransitiveClosure",
            arrow(
                arrow(type0(), arrow(type0(), prop())),
                arrow(type0(), arrow(type0(), prop())),
            ),
        ),
        ("FinalCoalgebraTheorem", final_coalgebra_theorem_ty()),
        ("LambeksLemmaCoalg", lambeks_lemma_coalg_ty()),
        ("TerminalCoalgebraUnfold", terminal_coalgebra_unfold_ty()),
        ("ExistsUnique", arrow(type0(), prop())),
        (
            "ana",
            arrow(type0(), app(cst("TerminalCoalgebra"), cst("F"))),
        ),
        ("GuardedRecursion", guarded_recursion_ty()),
        ("Productivity", productivity_ty()),
        ("CopatternMatching", copattern_matching_ty()),
        ("CoquandGuardCondition", coquand_guard_condition_ty()),
        ("Obs", type0()),
        ("ObsType", arrow(type0(), arrow(type0(), type0()))),
        ("StreamDiffEq", stream_diff_eq_ty()),
        ("StreamBisimCoinduction", stream_bisim_coinduction_ty()),
        ("StreamFunctor", stream_functor_ty()),
        (
            "StreamDiffSol",
            arrow(
                arrow(app(cst("Stream"), type0()), app(cst("Stream"), type0())),
                arrow(app(cst("Stream"), type0()), prop()),
            ),
        ),
        (
            "StreamBisimulation",
            arrow(arrow(type0(), arrow(type0(), prop())), prop()),
        ),
        ("MooreMachine", moore_machine_ty()),
        ("MealyMachine", mealy_machine_ty()),
        ("LanguageEquivalence", language_equivalence_ty()),
        ("AutomataMinimization", automata_minimization_ty()),
        (
            "LanguageEq",
            arrow(cst("MooreMachine"), arrow(cst("MooreMachine"), prop())),
        ),
        ("MossModality", moss_modality_ty()),
        ("NablaOperator", nabla_operator_ty()),
        ("PaigeTarjanBisim", paige_tarjan_bisim_ty()),
        ("CoalgModalLogicSoundness", coalg_modal_logic_soundness_ty()),
        ("BehaviouralEquivalence", behavioural_equivalence_ty()),
        ("BehaviouralEquivIsBisim", behavioural_equiv_is_bisim_ty()),
        ("PSPACEBisimDecision", pspace_bisim_decision_ty()),
        ("Bialgebra", bialgebra_ty()),
        ("HopfAlgebra", hopf_algebra_ty()),
        ("DistributiveLaw", distributive_law_ty()),
        ("ConvolutionProduct", convolution_product_ty()),
        ("HopfAntipode", hopf_antipode_ty()),
        ("GroupLikeBialgebra", group_like_bialgebra_ty()),
        ("InteractionSystem", interaction_system_ty()),
        ("NonDeterministicCoalgebra", nondeterministic_coalgebra_ty()),
        ("InteractionTreeBind", interaction_tree_bind_ty()),
        ("Powerset", arrow(type0(), type0())),
        ("InteractionTree", type0()),
        ("Comonad", comonad_ty()),
        ("ComonadExtract", comonad_extract_ty()),
        ("ComonadDuplicate", comonad_duplicate_ty()),
        ("ComonadicSemantics", comonadic_semantics_ty()),
        ("GuardedCorecursion", guarded_corecursion_ty()),
        (
            "UniquenessOfGuardedSolutions",
            uniqueness_of_guarded_solutions_ty(),
        ),
        ("CoinductiveDataType", coinductive_data_type_ty()),
        ("ObsSignature", type0()),
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
/// Trait for an F-coalgebra: a carrier type with a structure map.
pub trait Coalgebra: Sized {
    /// The "observable" type produced by the structure map.
    type Obs;
    /// The structure map α : Self → Obs.
    fn observe(&self) -> Self::Obs;
}
/// Compute the greatest fixed point of a monotone set function by Kleene iteration.
///
/// Represents sets as `Vec<bool>` over a finite universe of size `n`.
/// Starts from the top element (all true) and iterates until stable.
pub fn gfp_kleene<F>(n: usize, f: F, fuel: usize) -> Vec<bool>
where
    F: Fn(&[bool]) -> Vec<bool>,
{
    let mut current = vec![true; n];
    for _ in 0..fuel {
        let next = f(&current);
        if next == current {
            return current;
        }
        let meet: Vec<bool> = current
            .iter()
            .zip(next.iter())
            .map(|(&a, &b)| a && b)
            .collect();
        if meet == current {
            return current;
        }
        current = meet;
    }
    current
}
/// Compute the least fixed point by Kleene iteration from the bottom.
pub fn lfp_kleene<F>(n: usize, f: F, fuel: usize) -> Vec<bool>
where
    F: Fn(&[bool]) -> Vec<bool>,
{
    let mut current = vec![false; n];
    for _ in 0..fuel {
        let next = f(&current);
        if next == current {
            return current;
        }
        let join: Vec<bool> = current
            .iter()
            .zip(next.iter())
            .map(|(&a, &b)| a || b)
            .collect();
        if join == current {
            return current;
        }
        current = join;
    }
    current
}
/// Compute the first `n` elements of the stream produced by an anamorphism.
/// Seed `s`, step function `f : S → (A, S)`.
pub fn ana<S: Clone, A>(seed: S, f: impl Fn(S) -> (A, S), n: usize) -> Vec<A> {
    let mut result = Vec::with_capacity(n);
    let mut s = seed;
    for _ in 0..n {
        let (a, next_s) = f(s);
        result.push(a);
        s = next_s;
    }
    result
}
/// Hylomorphism: compose an anamorphism and a catamorphism.
/// `unfold : S → (A, S)`, `fold : (A, B) → B`, `base : B`.
pub fn hylo<S: Clone, A, B>(
    seed: S,
    unfold: impl Fn(S) -> Option<(A, S)>,
    fold: impl Fn(A, B) -> B,
    base: B,
) -> B {
    match unfold(seed) {
        None => base,
        Some((a, next)) => {
            let b = hylo(next, unfold, &fold, base);
            fold(a, b)
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_stream_nats() {
        let nats = Stream::<u64>::nats_from(0);
        let first5 = nats.take(5);
        assert_eq!(first5, vec![0, 1, 2, 3, 4]);
    }
    #[test]
    fn test_stream_constant() {
        let ones = Stream::constant(42u64);
        let first3 = ones.take(3);
        assert_eq!(first3, vec![42, 42, 42]);
    }
    #[test]
    fn test_stream_nth() {
        let nats = Stream::<u64>::nats_from(10);
        assert_eq!(nats.nth(3), 13);
    }
    #[test]
    fn test_lts_bisimilar_self() {
        let mut lts = LTS::new(3);
        lts.add_transition(0, "a", 1);
        lts.add_transition(0, "a", 2);
        lts.add_transition(1, "b", 1);
        lts.add_transition(2, "b", 2);
        assert!(lts.bisimilar(1, 2));
    }
    #[test]
    fn test_lts_not_bisimilar() {
        let mut lts = LTS::new(3);
        lts.add_transition(0, "a", 1);
        lts.add_transition(2, "a", 2);
        lts.add_transition(1, "b", 1);
        assert!(!lts.bisimilar(0, 2));
    }
    #[test]
    fn test_bisim_relation() {
        let mut rel = BisimRelation::empty();
        rel.add(0, 1);
        rel.add(1, 2);
        assert!(rel.contains(0, 1));
        assert!(!rel.contains(1, 0));
        rel.close_symmetric();
        assert!(rel.contains(1, 0));
    }
    #[test]
    fn test_gfp_kleene_all_true() {
        let result = gfp_kleene(4, |x| x.to_vec(), 100);
        assert!(result.iter().all(|&b| b));
    }
    #[test]
    fn test_lfp_kleene_all_false() {
        let result = lfp_kleene(4, |x| x.to_vec(), 100);
        assert!(result.iter().all(|&b| !b));
    }
    #[test]
    fn test_colist_take() {
        let cl: CoList<u64> = CoList::from_vec(vec![1, 2, 3]);
        let v = cl.take(10);
        assert_eq!(v, vec![1, 2, 3]);
    }
    #[test]
    fn test_colist_repeat() {
        let cl: CoList<u64> = CoList::repeat(7u64);
        let v = cl.take(4);
        assert_eq!(v, vec![7, 7, 7, 7]);
    }
    #[test]
    fn test_ana_fibonacci() {
        let fibs = ana((0u64, 1u64), |(a, b)| (a, (b, a + b)), 8);
        assert_eq!(fibs, vec![0, 1, 1, 2, 3, 5, 8, 13]);
    }
    #[test]
    fn test_build_coalgebra_env() {
        let mut env = Environment::new();
        build_coalgebra_theory_env(&mut env);
        assert!(env.get(&Name::str("FCoalgebra")).is_some());
        assert!(env.get(&Name::str("Bisimilar")).is_some());
        assert!(env.get(&Name::str("Stream")).is_some());
        assert!(env.get(&Name::str("GFP")).is_some());
        assert!(env.get(&Name::str("Later")).is_some());
        assert!(env.get(&Name::str("CoList")).is_some());
    }
    #[test]
    fn test_build_coalgebra_env_extended() {
        let mut env = Environment::new();
        build_coalgebra_theory_env(&mut env);
        assert!(env.get(&Name::str("BisimCoinduction")).is_some());
        assert!(env.get(&Name::str("CoinductiveProofPrinciple")).is_some());
        assert!(env.get(&Name::str("FinalCoalgebraTheorem")).is_some());
        assert!(env.get(&Name::str("LambeksLemmaCoalg")).is_some());
        assert!(env.get(&Name::str("GuardedRecursion")).is_some());
        assert!(env.get(&Name::str("CoquandGuardCondition")).is_some());
        assert!(env.get(&Name::str("StreamDiffEq")).is_some());
        assert!(env.get(&Name::str("StreamBisimCoinduction")).is_some());
        assert!(env.get(&Name::str("MooreMachine")).is_some());
        assert!(env.get(&Name::str("LanguageEquivalence")).is_some());
        assert!(env.get(&Name::str("MossModality")).is_some());
        assert!(env.get(&Name::str("NablaOperator")).is_some());
        assert!(env.get(&Name::str("HopfAlgebra")).is_some());
        assert!(env.get(&Name::str("ConvolutionProduct")).is_some());
        assert!(env.get(&Name::str("Comonad")).is_some());
        assert!(env.get(&Name::str("ComonadExtract")).is_some());
        assert!(env.get(&Name::str("GuardedCorecursion")).is_some());
        assert!(env.get(&Name::str("CoinductiveDataType")).is_some());
    }
    #[test]
    fn test_stream_coalgebra_observe() {
        let sc = StreamCoalgebra::new(vec![10u64, 20, 30]);
        assert_eq!(sc.observe_head(), Some(&10u64));
        assert_eq!(sc.collect_remaining(), vec![10, 20, 30]);
    }
    #[test]
    fn test_stream_coalgebra_advance() {
        let mut sc = StreamCoalgebra::new(vec![1u64, 2, 3]);
        sc.advance();
        assert_eq!(sc.observe_head(), Some(&2u64));
        sc.advance();
        assert_eq!(sc.observe_head(), Some(&3u64));
        sc.advance();
        assert_eq!(sc.observe_head(), None);
    }
    #[test]
    fn test_stream_coalgebra_bisimilar() {
        let a = StreamCoalgebra::new(vec![1u64, 2, 3, 4, 5]);
        let b = StreamCoalgebra::new(vec![1u64, 2, 3, 9, 9]);
        assert!(a.bisimilar_up_to(&b, 3));
        assert!(!a.bisimilar_up_to(&b, 4));
    }
    #[test]
    fn test_bisimulation_checker() {
        let mut lts = LTS::new(4);
        lts.add_transition(0, "a", 1);
        lts.add_transition(2, "a", 3);
        lts.add_transition(1, "b", 1);
        lts.add_transition(3, "b", 3);
        let checker = BisimulationChecker::new(lts);
        assert!(checker.check(0, 2));
        assert!(checker.check(1, 3));
    }
    #[test]
    fn test_bisimulation_checker_partition() {
        let mut lts = LTS::new(4);
        lts.add_transition(0, "a", 1);
        lts.add_transition(2, "a", 3);
        lts.add_transition(1, "b", 1);
        lts.add_transition(3, "b", 3);
        let checker = BisimulationChecker::new(lts);
        let classes = checker.compute_partition();
        assert_eq!(classes.len(), 2);
    }
    #[test]
    fn test_is_bisimulation() {
        let mut lts = LTS::new(4);
        lts.add_transition(0, "a", 1);
        lts.add_transition(2, "a", 3);
        lts.add_transition(1, "b", 1);
        lts.add_transition(3, "b", 3);
        let checker = BisimulationChecker::new(lts);
        let pairs = vec![(0, 2), (2, 0), (1, 3), (3, 1)];
        assert!(checker.is_bisimulation(&pairs));
    }
    #[test]
    fn test_final_coalgebra_approx_build() {
        let approx = FinalCoalgebraApprox::build_tree(3, 2);
        assert!(approx.has_root());
        assert_eq!(approx.num_nodes(), 15);
    }
    #[test]
    fn test_final_coalgebra_approx_branching() {
        let approx = FinalCoalgebraApprox::build_tree(2, 3);
        assert_eq!(approx.branching_at(0), 3);
    }
    #[test]
    fn test_final_coalgebra_approx_isomorphic() {
        let a = FinalCoalgebraApprox::build_tree(3, 2);
        let b = FinalCoalgebraApprox::build_tree(3, 2);
        assert!(a.isomorphic_up_to(&b, 3));
    }
    #[test]
    fn test_hopf_algebra_zero() {
        let z = HopfAlgebraOps::zero(4);
        assert_eq!(z.counit(), 0.0);
        assert_eq!(z.antipode().coords, vec![0.0; 4]);
    }
    #[test]
    fn test_hopf_algebra_basis() {
        let e0 = HopfAlgebraOps::basis(3, 0);
        assert_eq!(e0.coords, vec![1.0, 0.0, 0.0]);
        assert_eq!(e0.counit(), 1.0);
    }
    #[test]
    fn test_hopf_algebra_add_scale() {
        let e0 = HopfAlgebraOps::basis(3, 0);
        let e1 = HopfAlgebraOps::basis(3, 1);
        let sum = e0.add(&e1);
        assert_eq!(sum.coords, vec![1.0, 1.0, 0.0]);
        let scaled = sum.scale(2.0);
        assert_eq!(scaled.coords, vec![2.0, 2.0, 0.0]);
    }
    #[test]
    fn test_hopf_algebra_comultiply() {
        let e0 = HopfAlgebraOps::basis(3, 0);
        let (left, right) = e0.comultiply();
        assert_eq!(left, right);
    }
    #[test]
    fn test_hopf_algebra_convolution() {
        let e0 = HopfAlgebraOps::basis(3, 0);
        let result = e0.convolution(|x| x.clone(), |x| x.clone());
        assert_eq!(result.coords, vec![1.0, 0.0, 0.0]);
    }
}
#[cfg(test)]
mod tests_coalgebra_extended {
    use super::*;
    #[test]
    fn test_probabilistic_automaton_acceptance() {
        let mut pa = ProbabilisticAutomaton::new(2, 1);
        pa.set_accept(0, 1.0);
        pa.set_accept(1, 0.0);
        pa.set_trans(0, 0, 0, 1.0);
        pa.set_trans(0, 0, 1, 0.0);
        let prob_empty = pa.acceptance_prob(&[]);
        assert!((prob_empty - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_probabilistic_automaton_distance() {
        let pa1 = ProbabilisticAutomaton::new(2, 1);
        let mut pa2 = ProbabilisticAutomaton::new(2, 1);
        pa2.set_accept(0, 0.5);
        let d = pa1.behavioral_distance(&pa2);
        assert!((d - 0.5).abs() < 1e-10);
    }
    #[test]
    fn test_productivity_checker() {
        let pc = ProductivityChecker::new("nats", 1);
        assert!(pc.is_productive);
        assert!(pc.check_guardedness(1));
        assert!(!pc.check_guardedness(0));
    }
    #[test]
    fn test_quine_atom() {
        let mut hs = HyperSet::new(3);
        hs.add_member(0, 0);
        assert!(hs.is_quine_atom(0));
        assert!(hs.has_cycle_from(0));
        assert!(!hs.is_well_founded());
    }
    #[test]
    fn test_hyperset_well_founded() {
        let mut hs = HyperSet::new(3);
        hs.add_member(0, 1);
        hs.add_member(1, 2);
        assert!(hs.is_well_founded());
    }
    #[test]
    fn test_option_stream_map() {
        let s = OptionStream::finite(vec![1i64, 2, 3]);
        let doubled = s.map(|&x| x * 2);
        assert_eq!(doubled.prefix, vec![2, 4, 6]);
        assert!(doubled.terminates);
    }
    #[test]
    fn test_option_stream_zip() {
        let s1 = OptionStream::finite(vec![1i64, 2, 3]);
        let s2 = OptionStream::finite(vec![10i64, 20, 30]);
        let sum = s1.zip_with(&s2, |&a, &b| a + b);
        assert_eq!(sum.prefix, vec![11, 22, 33]);
    }
    #[test]
    fn test_option_stream_tail() {
        let s = OptionStream::finite(vec![1i64, 2, 3]);
        let t = s.tail();
        assert_eq!(t.prefix, vec![2, 3]);
    }
}
