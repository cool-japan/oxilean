//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};
use std::collections::{BTreeSet, HashSet, VecDeque};

use super::types::{
    AndersenPTA, ConstPropState, EraserState, FixpointSolver, IFCTracker, Interval, NullTracker,
    Nullability, PDGraph, SecurityLevel, Sign, TaintState, TypestateAutomaton,
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
pub fn bvar(n: u32) -> Expr {
    Expr::BVar(n)
}
pub fn nat_ty() -> Expr {
    cst("Nat")
}
pub fn bool_ty() -> Expr {
    cst("Bool")
}
pub fn string_ty() -> Expr {
    cst("String")
}
pub fn list_ty(elem: Expr) -> Expr {
    app(cst("List"), elem)
}
pub fn option_ty(elem: Expr) -> Expr {
    app(cst("Option"), elem)
}
pub fn set_ty(elem: Expr) -> Expr {
    app(cst("Set"), elem)
}
pub fn pair_ty(a: Expr, b: Expr) -> Expr {
    app2(cst("Prod"), a, b)
}
/// `Lattice : Type → Type` — A complete lattice on a carrier type.
pub fn lattice_ty() -> Expr {
    arrow(type0(), type0())
}
/// `LatticeBottom : ∀ L, Lattice L → L` — Bottom element ⊥.
pub fn lattice_bottom_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "L",
        type0(),
        arrow(app(cst("Lattice"), bvar(0)), bvar(1)),
    )
}
/// `LatticeTop : ∀ L, Lattice L → L` — Top element ⊤.
pub fn lattice_top_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "L",
        type0(),
        arrow(app(cst("Lattice"), bvar(0)), bvar(1)),
    )
}
/// `LatticeJoin : ∀ L, Lattice L → L → L → L` — Join operator ⊔.
pub fn lattice_join_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "L",
        type0(),
        arrow(
            app(cst("Lattice"), bvar(0)),
            arrow(bvar(1), arrow(bvar(2), bvar(3))),
        ),
    )
}
/// `LatticeMeet : ∀ L, Lattice L → L → L → L` — Meet operator ⊓.
pub fn lattice_meet_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "L",
        type0(),
        arrow(
            app(cst("Lattice"), bvar(0)),
            arrow(bvar(1), arrow(bvar(2), bvar(3))),
        ),
    )
}
/// `LatticeLeq : ∀ L, Lattice L → L → L → Prop` — Partial order ⊑.
pub fn lattice_leq_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "L",
        type0(),
        arrow(
            app(cst("Lattice"), bvar(0)),
            arrow(bvar(1), arrow(bvar(2), prop())),
        ),
    )
}
/// `MooreFamily : Set (Set A) → Prop` — Closed under arbitrary intersection.
pub fn moore_family_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        arrow(set_ty(set_ty(bvar(0))), prop()),
    )
}
/// `CompleteLattice_GaloisConnection : every Galois connection yields a complete lattice`
pub fn complete_lattice_galois_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "L",
        type0(),
        arrow(
            app(cst("GaloisConn"), bvar(0)),
            app(cst("CompleteLattice"), bvar(1)),
        ),
    )
}
/// `ConcreteSemantics : Program → Set State` — Collecting semantics.
pub fn concrete_semantics_ty() -> Expr {
    arrow(cst("Program"), set_ty(cst("ConcreteState")))
}
/// `AbstractSemantics : Program → AbsDom` — Abstract semantics over an abstract domain.
pub fn abstract_semantics_ty() -> Expr {
    arrow(cst("Program"), cst("AbsDom"))
}
/// `GaloisConnection : (C → A) × (A → C) → Prop`
/// (α, γ) form a Galois connection: α(c) ⊑ a ↔ c ⊆ γ(a).
pub fn galois_connection_ty() -> Expr {
    pair_ty(
        arrow(cst("ConcreteState"), cst("AbsDom")),
        arrow(cst("AbsDom"), cst("ConcreteState")),
    )
}
/// `Abstraction : ConcreteSemantics → AbsDom` — Abstraction function α.
pub fn abstraction_ty() -> Expr {
    arrow(concrete_semantics_ty(), cst("AbsDom"))
}
/// `Concretization : AbsDom → ConcreteSemantics` — Concretization function γ.
pub fn concretization_ty() -> Expr {
    arrow(cst("AbsDom"), concrete_semantics_ty())
}
/// `GaloisInsertion : GaloisConnection → Prop` — α∘γ = id (optimal abstraction).
pub fn galois_insertion_ty() -> Expr {
    arrow(galois_connection_ty(), prop())
}
/// `Widening : AbsDom → AbsDom → AbsDom` — Widening operator ▽.
pub fn widening_ty() -> Expr {
    arrow(cst("AbsDom"), arrow(cst("AbsDom"), cst("AbsDom")))
}
/// `Narrowing : AbsDom → AbsDom → AbsDom` — Narrowing operator △.
pub fn narrowing_ty() -> Expr {
    arrow(cst("AbsDom"), arrow(cst("AbsDom"), cst("AbsDom")))
}
/// `Widening_Terminates : widening guarantees termination of iteration`
pub fn widening_terminates_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        arrow(cst("AbsDom"), cst("AbsDom")),
        app(
            cst("Terminates"),
            app2(cst("fixpoint_widen"), bvar(0), cst("Widening")),
        ),
    )
}
/// `AbstractInterp_Sound : ∀ prog, α(collect prog) ⊑ abs_semantics prog`
pub fn abstract_interp_sound_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "prog",
        cst("Program"),
        app(
            app(cst("Leq"), app(cst("α"), app(cst("collect"), bvar(0)))),
            app(cst("abs_semantics"), bvar(0)),
        ),
    )
}
/// `Narrowing_RefinesPost : ∀ x y, y ⊑ x → y △ (f x) ⊑ x`
pub fn narrowing_refines_post_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "x",
        cst("AbsDom"),
        pi(
            BinderInfo::Default,
            "y",
            cst("AbsDom"),
            arrow(
                app2(cst("Leq"), bvar(0), bvar(1)),
                app(
                    app(
                        cst("Leq"),
                        app2(cst("Narrowing"), bvar(1), app(cst("f"), bvar(2))),
                    ),
                    bvar(2),
                ),
            ),
        ),
    )
}
/// `HeapGraph : Type` — A shape graph: nodes are abstract heap cells.
pub fn heap_graph_ty() -> Expr {
    type0()
}
/// `HeapNode : Type` — An abstract heap node (summary or concrete).
pub fn heap_node_ty() -> Expr {
    type0()
}
/// `SharingPred : HeapGraph → HeapNode → HeapNode → Prop`
/// May-sharing: two nodes may alias through some path.
pub fn sharing_pred_ty() -> Expr {
    arrow(
        heap_graph_ty(),
        arrow(heap_node_ty(), arrow(heap_node_ty(), prop())),
    )
}
/// `CyclicPred : HeapGraph → HeapNode → Prop`
/// A node may be part of a cyclic structure.
pub fn cyclic_pred_ty() -> Expr {
    arrow(heap_graph_ty(), arrow(heap_node_ty(), prop()))
}
/// `ShapeDescriptor : Type` — An abstract shape: tree, dag, list, cyclic-list, graph.
pub fn shape_descriptor_ty() -> Expr {
    type0()
}
/// `shape_join : ShapeDescriptor → ShapeDescriptor → ShapeDescriptor`
pub fn shape_join_ty() -> Expr {
    arrow(
        shape_descriptor_ty(),
        arrow(shape_descriptor_ty(), shape_descriptor_ty()),
    )
}
/// `shape_transfer : HeapOp → ShapeDescriptor → ShapeDescriptor`
pub fn shape_transfer_ty() -> Expr {
    arrow(
        cst("HeapOp"),
        arrow(shape_descriptor_ty(), shape_descriptor_ty()),
    )
}
/// `ShapeSound : ∀ op s, shape_transfer op s over-approximates concrete shapes`
pub fn shape_sound_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "op",
        cst("HeapOp"),
        pi(
            BinderInfo::Default,
            "s",
            shape_descriptor_ty(),
            app(
                cst("OverApprox"),
                app2(cst("shape_transfer"), bvar(1), bvar(0)),
            ),
        ),
    )
}
/// `ThreeValued : Type` — Three-valued logic {True, False, Maybe} for must/may.
pub fn three_valued_ty() -> Expr {
    type0()
}
/// `MustAlias : HeapGraph → HeapNode → HeapNode → ThreeValued`
pub fn must_alias_ty() -> Expr {
    arrow(
        heap_graph_ty(),
        arrow(heap_node_ty(), arrow(heap_node_ty(), three_valued_ty())),
    )
}
/// `AllocSite : Type` — An allocation site (abstract heap location).
pub fn alloc_site_ty() -> Expr {
    nat_ty()
}
/// `PointsTo : Var → Set AllocSite` — Points-to set for a variable.
pub fn points_to_ty() -> Expr {
    arrow(cst("Var"), set_ty(alloc_site_ty()))
}
/// `AndersenAnalysis : Program → PointsTo` — Andersen inclusion-based pointer analysis.
pub fn andersen_analysis_ty() -> Expr {
    arrow(cst("Program"), points_to_ty())
}
/// `SteensgaardAnalysis : Program → (Var → Var)` — Steensgaard unification-based analysis.
/// Returns equivalence classes (union-find representative).
pub fn steensgaard_analysis_ty() -> Expr {
    arrow(cst("Program"), arrow(cst("Var"), cst("Var")))
}
/// `Andersen_Sound : ∀ prog, andersen is a sound over-approximation`
pub fn andersen_sound_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "prog",
        cst("Program"),
        app(cst("Sound"), app(cst("AndersenAnalysis"), bvar(0))),
    )
}
/// `Steensgaard_SubAndersen : Steensgaard ⊑ Andersen (less precise but O(n α(n)))`
pub fn steensgaard_sub_andersen_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "prog",
        cst("Program"),
        app(
            app(cst("Leq"), app(cst("SteensgaardAnalysis"), bvar(0))),
            app(cst("AndersenAnalysis"), bvar(0)),
        ),
    )
}
/// `Andersen_Cubic : Andersen analysis is O(n³) in the worst case`
pub fn andersen_cubic_ty() -> Expr {
    app(
        cst("TimeComplexity"),
        app(cst("CubicN"), cst("AndersenAnalysis")),
    )
}
/// `Steensgaard_AlmostLinear : Steensgaard is nearly O(n) via union-find`
pub fn steensgaard_almost_linear_ty() -> Expr {
    app(
        cst("TimeComplexity"),
        app(cst("InvAckermann"), cst("SteensgaardAnalysis")),
    )
}
/// `AliasResult : Var → Var → Prop` — Two variables may alias.
pub fn alias_result_ty() -> Expr {
    arrow(cst("Var"), arrow(cst("Var"), prop()))
}
/// `MayAlias : Program → Var → Var → Bool`
pub fn may_alias_ty() -> Expr {
    arrow(
        cst("Program"),
        arrow(cst("Var"), arrow(cst("Var"), bool_ty())),
    )
}
/// `MustAliasPair : Program → Var → Var → Bool`
pub fn must_alias_pair_ty() -> Expr {
    arrow(
        cst("Program"),
        arrow(cst("Var"), arrow(cst("Var"), bool_ty())),
    )
}
/// `TypeBasedAlias : Type → Type → Bool` — TBAA: types determine alias potential.
pub fn type_based_alias_ty() -> Expr {
    arrow(cst("IRType"), arrow(cst("IRType"), bool_ty()))
}
/// `NoAlias : Var → Var → Prop` — Two variables definitely do not alias.
pub fn no_alias_ty() -> Expr {
    arrow(cst("Var"), arrow(cst("Var"), prop()))
}
/// `Alias_Undecidable : alias analysis is undecidable in general`
pub fn alias_undecidable_ty() -> Expr {
    app(cst("Undecidable"), cst("ExactAliasAnalysis"))
}
/// `SoftType : Type` — A soft type: a type that may be violated at runtime.
pub fn soft_type_ty() -> Expr {
    type0()
}
/// `SoftTypeCheck : Program → SoftType → Bool`
pub fn soft_type_check_ty() -> Expr {
    arrow(cst("Program"), arrow(soft_type_ty(), bool_ty()))
}
/// `RefinementType : Type → Pred → Type` — `{x : T | P(x)}`.
pub fn refinement_type_ty() -> Expr {
    arrow(type0(), arrow(arrow(bvar(0), prop()), type0()))
}
/// `EffectType : Type → EffectSet → Type` — A type annotated with an effect.
pub fn effect_type_ty() -> Expr {
    arrow(type0(), arrow(cst("EffectSet"), type0()))
}
/// `EffectSubtype : EffectType → EffectType → Prop` — Subtyping with effects.
pub fn effect_subtype_ty() -> Expr {
    arrow(effect_type_ty(), arrow(effect_type_ty(), prop()))
}
/// `Liquid_Haskell_Sound : ∀ prog, liquid_type_check prog → no_runtime_type_errors prog`
pub fn liquid_haskell_sound_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "prog",
        cst("Program"),
        arrow(
            app(cst("liquid_type_check"), bvar(0)),
            app(cst("no_runtime_errors"), bvar(1)),
        ),
    )
}
/// `CallSite : Type` — A call site in the program.
pub fn call_site_ty() -> Expr {
    nat_ty()
}
/// `AbstractClosure : Type` — An abstract closure: (lambda, env).
pub fn abstract_closure_ty() -> Expr {
    type0()
}
/// `ZeroCFA : Program → CallSite → Set AbstractClosure`
/// 0-CFA: context-insensitive closure analysis.
pub fn zero_cfa_ty() -> Expr {
    arrow(
        cst("Program"),
        arrow(call_site_ty(), set_ty(abstract_closure_ty())),
    )
}
/// `KCFA : Nat → Program → (CallSite × List CallSite) → Set AbstractClosure`
/// k-CFA: context-sensitive with call-string depth k.
pub fn k_cfa_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(
            cst("Program"),
            arrow(
                pair_ty(call_site_ty(), list_ty(call_site_ty())),
                set_ty(abstract_closure_ty()),
            ),
        ),
    )
}
/// `CFA_Overapprox : ∀ k prog, k_cfa k prog ⊇ actual call targets`
pub fn cfa_overapprox_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "prog",
            cst("Program"),
            app(cst("Overapproximates"), app2(cst("KCFA"), bvar(1), bvar(0))),
        ),
    )
}
/// `CFA_Monotone_in_k : 0-CFA ⊑ 1-CFA ⊑ 2-CFA ⊑ …`
pub fn cfa_monotone_k_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        nat_ty(),
        app(
            app(cst("Leq"), app2(cst("KCFA"), bvar(0), cst("prog"))),
            app2(cst("KCFA"), app(cst("Succ"), bvar(0)), cst("prog")),
        ),
    )
}
/// `KCFA_Complexity : k-CFA is EXPTIME-complete for k ≥ 1`
pub fn k_cfa_complexity_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "k",
        nat_ty(),
        arrow(
            app(cst("Ge"), app2(cst("Ge"), bvar(0), nat_ty())),
            app(cst("EXPTIMEComplete"), app(cst("KCFA"), bvar(1))),
        ),
    )
}
/// `Effect : Type` — An abstract program effect (read, write, alloc, throw, io).
pub fn effect_ty() -> Expr {
    type0()
}
/// `EffectSet : Type` — A set of effects annotating an expression or function.
pub fn effect_set_ty() -> Expr {
    set_ty(effect_ty())
}
/// `ReadEffect : Var → Effect` — Reading from variable.
pub fn read_effect_ty() -> Expr {
    arrow(cst("Var"), effect_ty())
}
/// `WriteEffect : Var → Effect` — Writing to variable.
pub fn write_effect_ty() -> Expr {
    arrow(cst("Var"), effect_ty())
}
/// `ExnEffect : ExnType → Effect` — Raising an exception.
pub fn exn_effect_ty() -> Expr {
    arrow(cst("ExnType"), effect_ty())
}
/// `infer_effects : Program → EffectMap` — Effect inference over a whole program.
pub fn infer_effects_ty() -> Expr {
    arrow(cst("Program"), cst("EffectMap"))
}
/// `EffectSound : ∀ prog, infer_effects prog ⊇ actual_effects prog`
pub fn effect_sound_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "prog",
        cst("Program"),
        app(
            app(cst("Superset"), app(cst("infer_effects"), bvar(0))),
            app(cst("actual_effects"), bvar(0)),
        ),
    )
}
/// `PureFunction : Expr → Prop` — A function with empty effect set.
pub fn pure_function_ty() -> Expr {
    arrow(cst("Expr"), prop())
}
/// `ResourceType : Type` — A linear/affine resource type.
pub fn resource_type_ty() -> Expr {
    type0()
}
/// `UsageAnnotation : Type` — Usage count: 0, 1, or ω (unrestricted).
pub fn usage_annotation_ty() -> Expr {
    type0()
}
/// `LinearType : ResourceType → Prop` — Resource must be used exactly once.
pub fn linear_type_ty() -> Expr {
    arrow(resource_type_ty(), prop())
}
/// `AffineType : ResourceType → Prop` — Resource used at most once.
pub fn affine_type_ty() -> Expr {
    arrow(resource_type_ty(), prop())
}
/// `ResourceUsageAnalysis : Program → Var → UsageAnnotation`
/// Infers usage count (0, 1, ω) for each variable.
pub fn resource_usage_analysis_ty() -> Expr {
    arrow(cst("Program"), arrow(cst("Var"), usage_annotation_ty()))
}
/// `LeakFreedom : Program → Prop` — All acquired resources are released.
pub fn leak_freedom_ty() -> Expr {
    arrow(cst("Program"), prop())
}
/// `LinearType_LeakFree : ∀ prog, well_typed_linear prog → LeakFreedom prog`
pub fn linear_type_leak_free_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "prog",
        cst("Program"),
        arrow(
            app(cst("WellTypedLinear"), bvar(0)),
            app(cst("LeakFreedom"), bvar(1)),
        ),
    )
}
/// `UsageCount_Sound : usage analysis is a sound over-approximation of actual usage`
pub fn usage_count_sound_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "prog",
        cst("Program"),
        app(cst("Sound"), app(cst("ResourceUsageAnalysis"), bvar(0))),
    )
}
/// `Thread : Type` — An abstract thread identifier.
pub fn thread_ty() -> Expr {
    nat_ty()
}
/// `LockSet : Type` — The set of locks held by a thread.
pub fn lock_set_ty() -> Expr {
    set_ty(nat_ty())
}
/// `HappensBefore : Event → Event → Prop` — The happens-before partial order.
pub fn happens_before_ty() -> Expr {
    arrow(cst("Event"), arrow(cst("Event"), prop()))
}
/// `DataRace : Event → Event → Prop`
/// Two events form a data race if they access the same location, at least
/// one is a write, they are from different threads, and not ordered by HB.
pub fn data_race_ty() -> Expr {
    arrow(cst("Event"), arrow(cst("Event"), prop()))
}
/// `EraserLockSet : Thread → MemLoc → Set Lock`
/// Eraser algorithm: track C(v) = intersection of lock sets for variable v.
pub fn eraser_lock_set_ty() -> Expr {
    arrow(thread_ty(), arrow(cst("MemLoc"), lock_set_ty()))
}
/// `Eraser_Invariant : C(v) ≠ ∅ → no data race on v`
pub fn eraser_invariant_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "v",
        cst("MemLoc"),
        arrow(
            app(cst("NonEmpty"), app(cst("C"), bvar(0))),
            app(cst("NoRace"), bvar(1)),
        ),
    )
}
/// `TSan_Sound : ThreadSanitizer reports all races`
pub fn tsan_sound_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "prog",
        cst("Program"),
        arrow(
            app(cst("HasRace"), bvar(0)),
            app(cst("TSanReports"), bvar(1)),
        ),
    )
}
/// `DataRaceFreedom : Program → Prop` — No data races in the program.
pub fn data_race_freedom_ty() -> Expr {
    arrow(cst("Program"), prop())
}
/// `DRF_Sequential : ∀ prog, DRF prog → sequential_consistent prog`
pub fn drf_sequential_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "prog",
        cst("Program"),
        arrow(
            app(cst("DataRaceFreedom"), bvar(0)),
            app(cst("SeqConsistent"), bvar(1)),
        ),
    )
}
/// `TaintSource : Type` — A source of tainted data (user input, env vars, etc.).
pub fn taint_source_ty() -> Expr {
    type0()
}
/// `TaintSink : Type` — A sink that must not receive tainted data (SQL query, shell cmd).
pub fn taint_sink_ty() -> Expr {
    type0()
}
/// `Sanitizer : Type` — A function that removes taint (e.g., HTML-escape).
pub fn sanitizer_ty() -> Expr {
    type0()
}
/// `TaintLabel : Var → Bool` — Is a variable tainted?
pub fn taint_label_ty() -> Expr {
    arrow(cst("Var"), bool_ty())
}
/// `TaintPropagation : Program → TaintLabel → TaintLabel`
/// Propagate taint through the program.
pub fn taint_propagation_ty() -> Expr {
    arrow(cst("Program"), arrow(taint_label_ty(), taint_label_ty()))
}
/// `TaintViolation : Program → TaintLabel → Set (Var × TaintSink) → Prop`
/// A taint violation: tainted variable reaches a sink without sanitization.
pub fn taint_violation_ty() -> Expr {
    arrow(
        cst("Program"),
        arrow(
            taint_label_ty(),
            arrow(set_ty(pair_ty(cst("Var"), taint_sink_ty())), prop()),
        ),
    )
}
/// `Taint_Sound : ∀ prog, taint_propagation over-approximates actual taint flow`
pub fn taint_sound_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "prog",
        cst("Program"),
        app(cst("Sound"), app(cst("TaintPropagation"), bvar(0))),
    )
}
/// `Taint_NoFalseNegatives : all real taint violations are reported`
pub fn taint_no_false_neg_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "prog",
        cst("Program"),
        arrow(
            app(cst("RealViolation"), bvar(0)),
            app(cst("Reported"), bvar(1)),
        ),
    )
}
/// `SepLogicHeap : Type` — A heap in separation logic (finite partial map Addr → Val).
pub fn sep_logic_heap_ty() -> Expr {
    arrow(cst("Addr"), option_ty(cst("Val")))
}
/// `SepConj : SepPred → SepPred → SepPred` — Separating conjunction P * Q.
pub fn sep_conj_ty() -> Expr {
    arrow(
        arrow(sep_logic_heap_ty(), prop()),
        arrow(
            arrow(sep_logic_heap_ty(), prop()),
            arrow(sep_logic_heap_ty(), prop()),
        ),
    )
}
/// `SepImp : SepPred → SepPred → SepPred` — Magic wand P −∗ Q.
pub fn sep_imp_ty() -> Expr {
    arrow(
        arrow(sep_logic_heap_ty(), prop()),
        arrow(
            arrow(sep_logic_heap_ty(), prop()),
            arrow(sep_logic_heap_ty(), prop()),
        ),
    )
}
/// `PointsToCell : Addr → Val → SepPred` — Singleton heap cell l ↦ v.
pub fn points_to_cell_ty() -> Expr {
    arrow(
        cst("Addr"),
        arrow(cst("Val"), arrow(sep_logic_heap_ty(), prop())),
    )
}
/// `FrameRule : ∀ prog P Q R, {P} prog {Q} → {P * R} prog {Q * R}` — Frame rule.
pub fn frame_rule_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "P",
        arrow(sep_logic_heap_ty(), prop()),
        pi(
            BinderInfo::Default,
            "Q",
            arrow(sep_logic_heap_ty(), prop()),
            pi(
                BinderInfo::Default,
                "R",
                arrow(sep_logic_heap_ty(), prop()),
                arrow(
                    app3(cst("HoareTriple"), bvar(2), cst("prog"), bvar(1)),
                    app3(
                        cst("HoareTriple"),
                        app2(cst("SepConj"), bvar(2), bvar(0)),
                        cst("prog"),
                        app2(cst("SepConj"), bvar(1), bvar(0)),
                    ),
                ),
            ),
        ),
    )
}
/// `HeapShape_TreePred : HeapNode → SepPred` — Predicate for an acyclic linked list.
pub fn heap_shape_tree_pred_ty() -> Expr {
    arrow(heap_node_ty(), arrow(sep_logic_heap_ty(), prop()))
}
/// `MemorySafety : Program → SepPred → Prop` — No out-of-bounds or dangling access.
pub fn memory_safety_ty() -> Expr {
    arrow(
        cst("Program"),
        arrow(arrow(sep_logic_heap_ty(), prop()), prop()),
    )
}
/// `OwnershipTransfer : SepPred → SepPred → Prop`
/// Ownership transfer from pre to post (move semantics).
pub fn ownership_transfer_ty() -> Expr {
    arrow(
        arrow(sep_logic_heap_ty(), prop()),
        arrow(arrow(sep_logic_heap_ty(), prop()), prop()),
    )
}
/// `Typestate : Type` — A typestate: a type annotated with a protocol state.
pub fn typestate_ty() -> Expr {
    type0()
}
/// `TypestateProtocol : Type` — A resource usage protocol (e.g. Opened/Closed file).
pub fn typestate_protocol_ty() -> Expr {
    arrow(typestate_ty(), set_ty(typestate_ty()))
}
/// `TypestateTransition : Typestate → Op → Typestate → Prop`
/// Specifies valid state transitions under operations.
pub fn typestate_transition_ty() -> Expr {
    arrow(
        typestate_ty(),
        arrow(cst("Op"), arrow(typestate_ty(), prop())),
    )
}
/// `TypestateCheck : Program → TypestateProtocol → Prop`
/// Program respects the resource usage protocol.
pub fn typestate_check_ty() -> Expr {
    arrow(cst("Program"), arrow(typestate_protocol_ty(), prop()))
}
/// `MustUseResource : Typestate → Prop` — Resource in this state must be consumed.
pub fn must_use_resource_ty() -> Expr {
    arrow(typestate_ty(), prop())
}
/// `TypestateSound : typestate analysis is a sound approximation of runtime states`
pub fn typestate_sound_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "prog",
        cst("Program"),
        arrow(
            app(cst("TypestateCheck"), bvar(0)),
            app(cst("NoProtocolViolation"), bvar(1)),
        ),
    )
}
/// `Region : Type` — A memory region (stack frame, heap generation, etc.).
pub fn region_ty() -> Expr {
    nat_ty()
}
/// `RegionAnnotation : Expr → Region` — Maps an expression to its allocation region.
pub fn region_annotation_ty() -> Expr {
    arrow(cst("Expr"), region_ty())
}
/// `EscapeAnalysis : Program → Var → Bool`
/// Returns true if a variable's value may escape its defining scope.
pub fn escape_analysis_ty() -> Expr {
    arrow(cst("Program"), arrow(cst("Var"), bool_ty()))
}
/// `RegionInference : Program → RegionAnnotation`
/// Infer allocation regions to enable stack allocation of non-escaping values.
pub fn region_inference_ty() -> Expr {
    arrow(cst("Program"), region_annotation_ty())
}
/// `Escape_Sound : ∀ prog v, ¬escape prog v → stack_allocated prog v`
pub fn escape_sound_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "prog",
        cst("Program"),
        pi(
            BinderInfo::Default,
            "v",
            cst("Var"),
            arrow(
                app(
                    app(cst("Not"), app2(cst("EscapeAnalysis"), bvar(1), bvar(0))),
                    nat_ty(),
                ),
                app2(cst("StackAllocated"), bvar(1), bvar(0)),
            ),
        ),
    )
}
/// `RegionSubtyping : Region → Region → Prop` — Lifetime/region containment.
pub fn region_subtyping_ty() -> Expr {
    arrow(region_ty(), arrow(region_ty(), prop()))
}
/// `MonadicEffect : Type → EffectSet → Type`
/// A computation of type T with effects E (monadic encoding).
pub fn monadic_effect_ty() -> Expr {
    arrow(type0(), arrow(effect_set_ty(), type0()))
}
/// `GradedMonad : (EffectSet → Type → Type) → Prop`
/// A graded monad indexed by effect sets.
pub fn graded_monad_ty() -> Expr {
    arrow(arrow(effect_set_ty(), arrow(type0(), type0())), prop())
}
/// `CapabilitySet : Type` — A set of capabilities (permissions) held at a program point.
pub fn capability_set_ty() -> Expr {
    set_ty(cst("Capability"))
}
/// `CapabilityJudgment : Context → Expr → EffectType → Prop`
/// Typing judgment: in context Γ with capabilities C, expression e has effect type T.
pub fn capability_judgment_ty() -> Expr {
    arrow(
        cst("Context"),
        arrow(cst("Expr"), arrow(effect_type_ty(), prop())),
    )
}
/// `EffectPolymorphism : ∀ e f, e : T!{f} → e : T!{f ∪ g}` — Effect weakening.
pub fn effect_polymorphism_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        effect_set_ty(),
        pi(
            BinderInfo::Default,
            "g",
            effect_set_ty(),
            pi(
                BinderInfo::Default,
                "T",
                type0(),
                arrow(
                    app2(cst("HasEffectType"), bvar(0), bvar(2)),
                    app2(
                        cst("HasEffectType"),
                        bvar(0),
                        app2(cst("EffectUnion"), bvar(2), bvar(1)),
                    ),
                ),
            ),
        ),
    )
}
/// `AlgebraicEffectHandler : EffectSet → Type → Type`
/// Handler type that handles a set of algebraic effects.
pub fn algebraic_effect_handler_ty() -> Expr {
    arrow(effect_set_ty(), arrow(type0(), type0()))
}
/// `GradualType : Type` — A gradual type (may contain the unknown type ?).
pub fn gradual_type_ty() -> Expr {
    type0()
}
/// `UnknownType : GradualType` — The dynamic unknown type ?.
pub fn unknown_type_ty() -> Expr {
    gradual_type_ty()
}
/// `ConsistencyRel : GradualType → GradualType → Prop` — Type consistency (~).
pub fn consistency_rel_ty() -> Expr {
    arrow(gradual_type_ty(), arrow(gradual_type_ty(), prop()))
}
/// `CastInsertion : GradualExpr → StaticExpr` — Elaborates gradual to casted static code.
pub fn cast_insertion_ty() -> Expr {
    arrow(cst("GradualExpr"), cst("StaticExpr"))
}
/// `CastCorrectness : ∀ e, eval (cast_insert e) = gradual_eval e`
/// Cast insertion preserves semantics.
pub fn cast_correctness_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "e",
        cst("GradualExpr"),
        app(
            app(
                cst("Eq"),
                app(cst("eval"), app(cst("CastInsertion"), bvar(0))),
            ),
            app(cst("gradual_eval"), bvar(0)),
        ),
    )
}
/// `Blame_Theorem : dynamic cast failures are attributed to correct source location`
pub fn blame_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "e",
        cst("GradualExpr"),
        arrow(
            app(cst("CastFails"), bvar(0)),
            app(cst("BlameCorrect"), bvar(1)),
        ),
    )
}
/// `LiquidType : BaseType → Qualifier → RefinementType`
/// A liquid type `{v : B | Q(v)}` with a qualifier drawn from a template set.
pub fn liquid_type_ty() -> Expr {
    arrow(
        cst("BaseType"),
        arrow(cst("Qualifier"), cst("RefinementType")),
    )
}
/// `QualifierInstantiation : Template → Substitution → Qualifier`
/// A qualifier instantiated from a Liquid Haskell template.
pub fn qualifier_instantiation_ty() -> Expr {
    arrow(
        cst("Template"),
        arrow(cst("Substitution"), cst("Qualifier")),
    )
}
/// `SubtypingRefinement : RefinementType → RefinementType → Prop`
/// `{v : B | P} <: {v : B | Q}` iff P ⊢ Q.
pub fn subtyping_refinement_ty() -> Expr {
    arrow(cst("RefinementType"), arrow(cst("RefinementType"), prop()))
}
/// `RefinementInference : Program → Var → RefinementType`
/// Automatically infer strongest safe refinement types.
pub fn refinement_inference_ty() -> Expr {
    arrow(cst("Program"), arrow(cst("Var"), cst("RefinementType")))
}
/// `LiquidType_Complete : liquid type inference is complete for the qualifier set`
pub fn liquid_type_complete_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "prog",
        cst("Program"),
        arrow(
            app(cst("QualifierSetFinite"), bvar(0)),
            app(cst("RefinementInferenceComplete"), bvar(1)),
        ),
    )
}
/// `SecurityLabel : Type` — A security lattice element (e.g. Low, High, Secret).
pub fn security_label_ty() -> Expr {
    type0()
}
/// `SecrecyLattice : SecurityLabel → SecurityLabel → Prop` — Label ordering ⊑.
pub fn secrecy_lattice_ty() -> Expr {
    arrow(security_label_ty(), arrow(security_label_ty(), prop()))
}
/// `LabelEnv : Var → SecurityLabel` — Security label environment.
pub fn label_env_ty() -> Expr {
    arrow(cst("Var"), security_label_ty())
}
/// `NonInterference : Program → LabelEnv → Prop`
/// Low-equivalent inputs produce low-equivalent outputs.
pub fn non_interference_ty() -> Expr {
    arrow(cst("Program"), arrow(label_env_ty(), prop()))
}
/// `Declassification : Expr → SecurityLabel → SecurityLabel → Prop`
/// Explicit declassification from high to low label.
pub fn declassification_ty() -> Expr {
    arrow(
        cst("Expr"),
        arrow(security_label_ty(), arrow(security_label_ty(), prop())),
    )
}
/// `IFCTypeSystem : Context → Expr → LabeledType → Prop`
/// Information flow control typing judgment.
pub fn ifc_type_system_ty() -> Expr {
    arrow(
        cst("Context"),
        arrow(cst("Expr"), arrow(cst("LabeledType"), prop())),
    )
}
/// `NI_Theorem : ∀ prog env, IFC_typed prog env → NonInterference prog env`
pub fn ni_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "prog",
        cst("Program"),
        pi(
            BinderInfo::Default,
            "env",
            label_env_ty(),
            arrow(
                app2(cst("IFC_typed"), bvar(1), bvar(0)),
                app2(cst("NonInterference"), bvar(2), bvar(1)),
            ),
        ),
    )
}
/// `ConstantFolding : Expr → Expr` — Evaluate constant sub-expressions at compile time.
pub fn constant_folding_ty() -> Expr {
    arrow(cst("Expr"), cst("Expr"))
}
/// `ConstantPropagation : Program → Var → Option Val`
/// Statically determine the value of a variable if it is constant.
pub fn constant_propagation_ty() -> Expr {
    arrow(cst("Program"), arrow(cst("Var"), option_ty(cst("Val"))))
}
/// `ConstFold_Correct : ∀ e, eval (const_fold e) = eval e`
pub fn const_fold_correct_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "e",
        cst("Expr"),
        app(
            app(
                cst("Eq"),
                app(cst("eval"), app(cst("ConstantFolding"), bvar(0))),
            ),
            app(cst("eval"), bvar(0)),
        ),
    )
}
/// `IntervalDomain : Type` — Abstract domain for value range analysis (intervals).
pub fn interval_domain_ty() -> Expr {
    type0()
}
/// `BitfieldDomain : Type` — Abstract domain tracking known/unknown bits.
pub fn bitfield_domain_ty() -> Expr {
    type0()
}
/// `ValueRangeAnalysis : Program → Var → IntervalDomain`
/// Compute abstract value ranges for all variables.
pub fn value_range_analysis_ty() -> Expr {
    arrow(cst("Program"), arrow(cst("Var"), interval_domain_ty()))
}
/// `VRA_Sound : ∀ prog, value_range_analysis over-approximates concrete values`
pub fn vra_sound_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "prog",
        cst("Program"),
        app(cst("Sound"), app(cst("ValueRangeAnalysis"), bvar(0))),
    )
}
/// `NullabilityAnnotation : Type → Bool → Type`
/// Type annotated with nullability: T? vs T!.
pub fn nullability_annotation_ty() -> Expr {
    arrow(type0(), arrow(bool_ty(), type0()))
}
/// `NullPointerAnalysis : Program → Var → ThreeValued`
/// Three-valued result: definitely null / definitely non-null / maybe null.
pub fn null_pointer_analysis_ty() -> Expr {
    arrow(cst("Program"), arrow(cst("Var"), three_valued_ty()))
}
/// `DefiniteAssignment : Program → Var → Prop`
/// Every use of a variable is preceded by a definition.
pub fn definite_assignment_ty() -> Expr {
    arrow(cst("Program"), arrow(cst("Var"), prop()))
}
/// `NullSafety : Program → Prop` — No null dereferences occur.
pub fn null_safety_ty() -> Expr {
    arrow(cst("Program"), prop())
}
/// `NullAnalysis_Sound : ∀ prog, null_pointer_analysis is sound`
pub fn null_analysis_sound_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "prog",
        cst("Program"),
        app(cst("Sound"), app(cst("NullPointerAnalysis"), bvar(0))),
    )
}
/// `LockOrder : Lock → Lock → Prop` — A consistent lock acquisition order.
pub fn lock_order_ty() -> Expr {
    arrow(cst("Lock"), arrow(cst("Lock"), prop()))
}
/// `DeadlockFreedom : Program → Prop` — No circular lock dependencies.
pub fn deadlock_freedom_ty() -> Expr {
    arrow(cst("Program"), prop())
}
/// `LockOrder_Acyclic : ∀ prog, acyclic_lock_order prog → DeadlockFreedom prog`
pub fn lock_order_acyclic_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "prog",
        cst("Program"),
        arrow(
            app(cst("AcyclicLockOrder"), bvar(0)),
            app(cst("DeadlockFreedom"), bvar(1)),
        ),
    )
}
/// `AtomicBlock : Program → Stmt → Prop`
/// A statement executes atomically with respect to all other threads.
pub fn atomic_block_ty() -> Expr {
    arrow(cst("Program"), arrow(cst("Stmt"), prop()))
}
/// `Atomicity_Serializability : ∀ prog s, Atomic prog s → serializable_execution prog s`
pub fn atomicity_serializability_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "prog",
        cst("Program"),
        pi(
            BinderInfo::Default,
            "s",
            cst("Stmt"),
            arrow(
                app2(cst("AtomicBlock"), bvar(1), bvar(0)),
                app2(cst("SerializableExec"), bvar(2), bvar(1)),
            ),
        ),
    )
}
/// `LockSetAnalysis : Program → Thread → MemLoc → LockSet`
/// Compute the set of locks held by thread t when accessing loc.
pub fn lock_set_analysis_ty() -> Expr {
    arrow(
        cst("Program"),
        arrow(thread_ty(), arrow(cst("MemLoc"), lock_set_ty())),
    )
}
/// `OwnershipType : Type` — A type carrying ownership (unique, shared, borrowed).
pub fn ownership_type_ty() -> Expr {
    type0()
}
/// `BorrowKind : Type` — Borrow flavor: immutable (&) or mutable (&mut).
pub fn borrow_kind_ty() -> Expr {
    type0()
}
/// `Lifetime : Type` — A lifetime region for borrow validity.
pub fn lifetime_ty() -> Expr {
    nat_ty()
}
/// `BorrowCheck : Program → Prop` — Rust-like borrow checker soundness.
pub fn borrow_check_ty() -> Expr {
    arrow(cst("Program"), prop())
}
/// `OwnershipUnique : OwnershipType → Prop` — At most one owner at any time.
pub fn ownership_unique_ty() -> Expr {
    arrow(ownership_type_ty(), prop())
}
/// `BorrowCheck_MemSafe : ∀ prog, borrow_check prog → memory_safe prog`
pub fn borrow_check_mem_safe_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "prog",
        cst("Program"),
        arrow(
            app(cst("BorrowCheck"), bvar(0)),
            app(cst("MemorySafe"), bvar(1)),
        ),
    )
}
/// `LifetimeSubtyping : Lifetime → Lifetime → Prop` — 'a: 'b (a outlives b).
pub fn lifetime_subtyping_ty() -> Expr {
    arrow(lifetime_ty(), arrow(lifetime_ty(), prop()))
}
/// `NonLexicalLifetime : Program → LifetimeAnnotation`
/// Non-lexical lifetime inference for precise scope analysis.
pub fn non_lexical_lifetime_ty() -> Expr {
    arrow(cst("Program"), cst("LifetimeAnnotation"))
}
/// `DataDependence : Stmt → Stmt → Prop` — def-use data dependence edge.
pub fn data_dependence_ty() -> Expr {
    arrow(cst("Stmt"), arrow(cst("Stmt"), prop()))
}
/// `ControlDependence : Stmt → Stmt → Prop` — Control dependence edge.
pub fn control_dependence_ty() -> Expr {
    arrow(cst("Stmt"), arrow(cst("Stmt"), prop()))
}
/// `ProgramDependenceGraph : Program → PDG`
/// Build the program dependence graph combining data and control edges.
pub fn program_dependence_graph_ty() -> Expr {
    arrow(cst("Program"), cst("PDG"))
}
/// `BackwardSlice : PDG → SliceCriterion → Set Stmt`
/// Backward slice: all statements that can affect the criterion.
pub fn backward_slice_ty() -> Expr {
    arrow(
        cst("PDG"),
        arrow(cst("SliceCriterion"), set_ty(cst("Stmt"))),
    )
}
/// `ForwardSlice : PDG → SliceCriterion → Set Stmt`
/// Forward slice: all statements that the criterion can affect.
pub fn forward_slice_ty() -> Expr {
    arrow(
        cst("PDG"),
        arrow(cst("SliceCriterion"), set_ty(cst("Stmt"))),
    )
}
/// `Slice_Correct : ∀ prog c, backward_slice preserves criterion semantics`
pub fn slice_correct_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "prog",
        cst("Program"),
        pi(
            BinderInfo::Default,
            "c",
            cst("SliceCriterion"),
            app(
                cst("PreservesSemantics"),
                app2(
                    cst("BackwardSlice"),
                    app(cst("ProgramDependenceGraph"), bvar(1)),
                    bvar(0),
                ),
            ),
        ),
    )
}
/// Populate an OxiLean kernel `Environment` with all static-analysis axioms.
pub fn build_static_analysis_env() -> Environment {
    let mut env = Environment::new();
    let axioms: &[(&str, Expr)] = &[
        ("Lattice", lattice_ty()),
        ("LatticeBottom", lattice_bottom_ty()),
        ("LatticeTop", lattice_top_ty()),
        ("LatticeJoin", lattice_join_ty()),
        ("LatticeMeet", lattice_meet_ty()),
        ("LatticeLeq", lattice_leq_ty()),
        ("MooreFamily", moore_family_ty()),
        ("CompleteLattice_GaloisConn", complete_lattice_galois_ty()),
        ("ConcreteSemantics", concrete_semantics_ty()),
        ("AbstractSemantics", abstract_semantics_ty()),
        ("GaloisConnection", galois_connection_ty()),
        ("Abstraction", abstraction_ty()),
        ("Concretization", concretization_ty()),
        ("GaloisInsertion", galois_insertion_ty()),
        ("Widening", widening_ty()),
        ("Narrowing", narrowing_ty()),
        ("Widening_Terminates", widening_terminates_ty()),
        ("AbstractInterp_Sound", abstract_interp_sound_ty()),
        ("Narrowing_RefinesPost", narrowing_refines_post_ty()),
        ("HeapGraph", heap_graph_ty()),
        ("HeapNode", heap_node_ty()),
        ("SharingPred", sharing_pred_ty()),
        ("CyclicPred", cyclic_pred_ty()),
        ("ShapeDescriptor", shape_descriptor_ty()),
        ("shape_join", shape_join_ty()),
        ("shape_transfer", shape_transfer_ty()),
        ("ShapeSound", shape_sound_ty()),
        ("ThreeValued", three_valued_ty()),
        ("MustAlias", must_alias_ty()),
        ("AllocSite", alloc_site_ty()),
        ("PointsTo", points_to_ty()),
        ("AndersenAnalysis", andersen_analysis_ty()),
        ("SteensgaardAnalysis", steensgaard_analysis_ty()),
        ("Andersen_Sound", andersen_sound_ty()),
        ("Steensgaard_SubAndersen", steensgaard_sub_andersen_ty()),
        ("Andersen_Cubic", andersen_cubic_ty()),
        ("Steensgaard_AlmostLinear", steensgaard_almost_linear_ty()),
        ("AliasResult", alias_result_ty()),
        ("MayAlias", may_alias_ty()),
        ("MustAliasPair", must_alias_pair_ty()),
        ("TypeBasedAlias", type_based_alias_ty()),
        ("NoAlias", no_alias_ty()),
        ("Alias_Undecidable", alias_undecidable_ty()),
        ("SoftType", soft_type_ty()),
        ("SoftTypeCheck", soft_type_check_ty()),
        ("RefinementType", refinement_type_ty()),
        ("EffectType", effect_type_ty()),
        ("EffectSubtype", effect_subtype_ty()),
        ("Liquid_Haskell_Sound", liquid_haskell_sound_ty()),
        ("CallSite", call_site_ty()),
        ("AbstractClosure", abstract_closure_ty()),
        ("ZeroCFA", zero_cfa_ty()),
        ("KCFA", k_cfa_ty()),
        ("CFA_Overapprox", cfa_overapprox_ty()),
        ("CFA_Monotone_in_k", cfa_monotone_k_ty()),
        ("KCFA_Complexity", k_cfa_complexity_ty()),
        ("Effect", effect_ty()),
        ("EffectSet", effect_set_ty()),
        ("ReadEffect", read_effect_ty()),
        ("WriteEffect", write_effect_ty()),
        ("ExnEffect", exn_effect_ty()),
        ("infer_effects", infer_effects_ty()),
        ("EffectSound", effect_sound_ty()),
        ("PureFunction", pure_function_ty()),
        ("ResourceType", resource_type_ty()),
        ("UsageAnnotation", usage_annotation_ty()),
        ("LinearType", linear_type_ty()),
        ("AffineType", affine_type_ty()),
        ("ResourceUsageAnalysis", resource_usage_analysis_ty()),
        ("LeakFreedom", leak_freedom_ty()),
        ("LinearType_LeakFree", linear_type_leak_free_ty()),
        ("UsageCount_Sound", usage_count_sound_ty()),
        ("Thread", thread_ty()),
        ("LockSet", lock_set_ty()),
        ("HappensBefore", happens_before_ty()),
        ("DataRace", data_race_ty()),
        ("EraserLockSet", eraser_lock_set_ty()),
        ("Eraser_Invariant", eraser_invariant_ty()),
        ("TSan_Sound", tsan_sound_ty()),
        ("DataRaceFreedom", data_race_freedom_ty()),
        ("DRF_Sequential", drf_sequential_ty()),
        ("TaintSource", taint_source_ty()),
        ("TaintSink", taint_sink_ty()),
        ("Sanitizer", sanitizer_ty()),
        ("TaintLabel", taint_label_ty()),
        ("TaintPropagation", taint_propagation_ty()),
        ("TaintViolation", taint_violation_ty()),
        ("Taint_Sound", taint_sound_ty()),
        ("Taint_NoFalseNegatives", taint_no_false_neg_ty()),
        ("SepLogicHeap", sep_logic_heap_ty()),
        ("SepConj", sep_conj_ty()),
        ("SepImp", sep_imp_ty()),
        ("PointsToCell", points_to_cell_ty()),
        ("FrameRule", frame_rule_ty()),
        ("HeapShape_TreePred", heap_shape_tree_pred_ty()),
        ("MemorySafety", memory_safety_ty()),
        ("OwnershipTransfer", ownership_transfer_ty()),
        ("Typestate", typestate_ty()),
        ("TypestateProtocol", typestate_protocol_ty()),
        ("TypestateTransition", typestate_transition_ty()),
        ("TypestateCheck", typestate_check_ty()),
        ("MustUseResource", must_use_resource_ty()),
        ("TypestateSound", typestate_sound_ty()),
        ("Region", region_ty()),
        ("RegionAnnotation", region_annotation_ty()),
        ("EscapeAnalysis", escape_analysis_ty()),
        ("RegionInference", region_inference_ty()),
        ("Escape_Sound", escape_sound_ty()),
        ("RegionSubtyping", region_subtyping_ty()),
        ("MonadicEffect", monadic_effect_ty()),
        ("GradedMonad", graded_monad_ty()),
        ("CapabilitySet", capability_set_ty()),
        ("CapabilityJudgment", capability_judgment_ty()),
        ("EffectPolymorphism", effect_polymorphism_ty()),
        ("AlgebraicEffectHandler", algebraic_effect_handler_ty()),
        ("GradualType", gradual_type_ty()),
        ("UnknownType", unknown_type_ty()),
        ("ConsistencyRel", consistency_rel_ty()),
        ("CastInsertion", cast_insertion_ty()),
        ("CastCorrectness", cast_correctness_ty()),
        ("Blame_Theorem", blame_theorem_ty()),
        ("LiquidType", liquid_type_ty()),
        ("QualifierInstantiation", qualifier_instantiation_ty()),
        ("SubtypingRefinement", subtyping_refinement_ty()),
        ("RefinementInference", refinement_inference_ty()),
        ("LiquidType_Complete", liquid_type_complete_ty()),
        ("SecurityLabel", security_label_ty()),
        ("SecrecyLattice", secrecy_lattice_ty()),
        ("LabelEnv", label_env_ty()),
        ("NonInterference", non_interference_ty()),
        ("Declassification", declassification_ty()),
        ("IFCTypeSystem", ifc_type_system_ty()),
        ("NI_Theorem", ni_theorem_ty()),
        ("ConstantFolding", constant_folding_ty()),
        ("ConstantPropagation", constant_propagation_ty()),
        ("ConstFold_Correct", const_fold_correct_ty()),
        ("IntervalDomain", interval_domain_ty()),
        ("BitfieldDomain", bitfield_domain_ty()),
        ("ValueRangeAnalysis", value_range_analysis_ty()),
        ("VRA_Sound", vra_sound_ty()),
        ("NullabilityAnnotation", nullability_annotation_ty()),
        ("NullPointerAnalysis", null_pointer_analysis_ty()),
        ("DefiniteAssignment", definite_assignment_ty()),
        ("NullSafety", null_safety_ty()),
        ("NullAnalysis_Sound", null_analysis_sound_ty()),
        ("LockOrder", lock_order_ty()),
        ("DeadlockFreedom", deadlock_freedom_ty()),
        ("LockOrder_Acyclic", lock_order_acyclic_ty()),
        ("AtomicBlock", atomic_block_ty()),
        ("Atomicity_Serializability", atomicity_serializability_ty()),
        ("LockSetAnalysis", lock_set_analysis_ty()),
        ("OwnershipType", ownership_type_ty()),
        ("BorrowKind", borrow_kind_ty()),
        ("Lifetime", lifetime_ty()),
        ("BorrowCheck", borrow_check_ty()),
        ("OwnershipUnique", ownership_unique_ty()),
        ("BorrowCheck_MemSafe", borrow_check_mem_safe_ty()),
        ("LifetimeSubtyping", lifetime_subtyping_ty()),
        ("NonLexicalLifetime", non_lexical_lifetime_ty()),
        ("DataDependence", data_dependence_ty()),
        ("ControlDependence", control_dependence_ty()),
        ("ProgramDependenceGraph", program_dependence_graph_ty()),
        ("BackwardSlice", backward_slice_ty()),
        ("ForwardSlice", forward_slice_ty()),
        ("Slice_Correct", slice_correct_ty()),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    env
}
#[cfg(test)]
mod tests {
    use super::*;
    /// Smoke test: build env produces expected axioms.
    #[test]
    fn test_build_env_nonempty() {
        let env = build_static_analysis_env();
        let names = [
            "Lattice",
            "GaloisConnection",
            "Widening",
            "Narrowing",
            "AndersenAnalysis",
            "HappensBefore",
            "TaintPropagation",
            "ShapeDescriptor",
            "KCFA",
            "DataRaceFreedom",
        ];
        let count = names
            .iter()
            .filter(|&&n| env.get(&Name::str(n)).is_some())
            .count();
        assert_eq!(count, 10);
    }
    /// Interval lattice: join, meet, order.
    #[test]
    fn test_interval_lattice() {
        let a = Interval::Range(1, 5);
        let b = Interval::Range(3, 8);
        assert_eq!(a.join(&b), Interval::Range(1, 8));
        assert_eq!(a.meet(&b), Interval::Range(3, 5));
        assert!(Interval::Bottom.leq(&a));
        assert!(a.leq(&Interval::top()));
        assert!(!b.leq(&a));
    }
    /// Interval widening drives to ⊤.
    #[test]
    fn test_interval_widening() {
        let mut x = Interval::Range(0, 0);
        for _ in 0..5 {
            let next = x.add(&Interval::single(1));
            x = x.widen(&next);
        }
        assert_eq!(x, Interval::Range(0, i64::MAX));
    }
    /// Sign domain: join and add.
    #[test]
    fn test_sign_domain() {
        assert_eq!(Sign::of(3), Sign::Pos);
        assert_eq!(Sign::of(-2), Sign::Neg);
        assert_eq!(Sign::of(0), Sign::Zero);
        assert_eq!(Sign::Pos.join(&Sign::Neg), Sign::Top);
        assert_eq!(Sign::Pos.add(&Sign::Zero), Sign::Pos);
        assert_eq!(Sign::Neg.neg(), Sign::Pos);
    }
    /// Andersen PTA: simple copy chain.
    #[test]
    fn test_andersen_copy_chain() {
        let mut pta = AndersenPTA::new(3);
        pta.add_alloc(0, 0);
        pta.add_copy(0, 1);
        pta.add_copy(1, 2);
        pta.solve();
        assert!(pta.pts[0].contains(&0));
        assert!(pta.pts[1].contains(&0));
        assert!(pta.pts[2].contains(&0));
        assert!(pta.may_alias(0, 1));
        assert!(pta.may_alias(1, 2));
    }
    /// Taint propagation: source → propagation → sink.
    #[test]
    fn test_taint_propagation() {
        let mut ts = TaintState::new();
        ts.add_source("user_input");
        ts.propagate("x", &["user_input"]);
        ts.propagate("y", &["x", "const"]);
        assert!(ts.violates("x"));
        assert!(ts.violates("y"));
        ts.sanitize("x");
        assert!(!ts.violates("x"));
        assert!(ts.violates("y"));
    }
    /// Eraser race detection: two threads, no common lock → race.
    #[test]
    fn test_eraser_race_detected() {
        let mut state = EraserState::new();
        let locks_t1: BTreeSet<usize> = vec![1].into_iter().collect();
        let locks_t2: BTreeSet<usize> = vec![2].into_iter().collect();
        state.observe_access(0, &locks_t1, true);
        state.observe_access(1, &locks_t2, true);
        assert!(state.has_race());
    }
    /// Eraser: same lock on both accesses → no race.
    #[test]
    fn test_eraser_no_race() {
        let mut state = EraserState::new();
        let locks: BTreeSet<usize> = vec![42].into_iter().collect();
        state.observe_access(0, &locks, true);
        state.observe_access(1, &locks, false);
        assert!(!state.has_race());
    }
    /// Fixpoint solver: reachability over a 3-node chain.
    #[test]
    fn test_fixpoint_reachability() {
        let edges = vec![vec![1], vec![2], vec![]];
        let mut solver = FixpointSolver::new(
            3,
            edges,
            false,
            |a: &bool, b: &bool| *a || *b,
            |n, v| if n == 0 { true } else { *v },
        );
        solver.values[0] = true;
        solver.solve();
        assert!(solver.values[1]);
        assert!(solver.values[2]);
    }
    /// New axioms are registered in the environment.
    #[test]
    fn test_new_axioms_registered() {
        let env = build_static_analysis_env();
        let names = [
            "SepConj",
            "FrameRule",
            "TypestateSound",
            "EscapeAnalysis",
            "GradedMonad",
            "CastCorrectness",
            "LiquidType",
            "NonInterference",
            "ConstantFolding",
            "NullSafety",
            "DeadlockFreedom",
            "BorrowCheck",
            "BackwardSlice",
        ];
        let count = names
            .iter()
            .filter(|&&n| env.get(&Name::str(n)).is_some())
            .count();
        assert_eq!(count, names.len());
    }
    /// TypestateAutomaton: file open/close protocol.
    #[test]
    fn test_typestate_automaton_file() {
        let mut aut = TypestateAutomaton::new(2, 0);
        aut.add_transition(0, "open", 1);
        aut.add_transition(1, "read", 1);
        aut.add_transition(1, "close", 0);
        aut.set_accepting(0);
        assert!(aut.accepts(&["open", "read", "close"]));
        assert!(!aut.accepts(&["open", "read"]));
        assert!(aut.violates(&["open", "open"]));
    }
    /// IFCTracker: high-labeled data must not flow to low sink.
    #[test]
    fn test_ifc_tracker_violation() {
        let mut tracker = IFCTracker::new();
        tracker.assign("password", SecurityLevel::High);
        tracker.assign("user_id", SecurityLevel::Low);
        tracker.propagate("derived", &["password", "user_id"]);
        assert_eq!(tracker.label_of("derived"), SecurityLevel::High);
        tracker.check_flow("derived", &SecurityLevel::Low);
        assert!(tracker.has_violation());
    }
    /// IFCTracker: low-labeled data can flow to high sink.
    #[test]
    fn test_ifc_tracker_ok() {
        let mut tracker = IFCTracker::new();
        tracker.assign("public_id", SecurityLevel::Low);
        tracker.check_flow("public_id", &SecurityLevel::High);
        assert!(!tracker.has_violation());
    }
    /// ConstPropState: basic folding and join.
    #[test]
    fn test_const_prop_state() {
        let mut s1 = ConstPropState::new();
        s1.set_const("x", 10);
        s1.set_const("y", 5);
        assert_eq!(s1.fold_add("x", "y"), Some(15));
        let mut s2 = ConstPropState::new();
        s2.set_const("x", 10);
        s2.set_top("y");
        let joined = s1.join(&s2);
        assert_eq!(joined.get("x"), Some(10));
        assert_eq!(joined.get("y"), None);
    }
    /// PDGraph: backward and forward slicing.
    #[test]
    fn test_pd_graph_slicing() {
        let mut pdg = PDGraph::new(4);
        pdg.add_data_edge(0, 1);
        pdg.add_data_edge(1, 3);
        let bwd = pdg.backward_slice(3);
        assert!(bwd.contains(&3));
        assert!(bwd.contains(&1));
        assert!(bwd.contains(&0));
        assert!(!bwd.contains(&2));
        let fwd = pdg.forward_slice(0);
        assert!(fwd.contains(&0));
        assert!(fwd.contains(&1));
        assert!(fwd.contains(&3));
    }
    /// NullTracker: alarm on dereference of maybe-null variable.
    #[test]
    fn test_null_tracker_alarm() {
        let mut tracker = NullTracker::new();
        tracker.declare_maybe_null("ptr");
        tracker.dereference("ptr");
        assert!(tracker.has_alarm());
    }
    /// NullTracker: no alarm on definitely non-null variable.
    #[test]
    fn test_null_tracker_no_alarm() {
        let mut tracker = NullTracker::new();
        tracker.declare_non_null("safe_ptr");
        tracker.dereference("safe_ptr");
        assert!(!tracker.has_alarm());
    }
    /// NullTracker: join merges states correctly.
    #[test]
    fn test_null_tracker_join() {
        let mut t1 = NullTracker::new();
        t1.declare_non_null("p");
        let mut t2 = NullTracker::new();
        t2.declare_null("p");
        let merged = t1.join(&t2);
        assert_eq!(merged.get("p"), &Nullability::MaybeNull);
    }
}
