//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    AsyncSessionEndpoint, BaseType, ChoreographyEngine, ChoreographyStep, DeadlockChecker, GType,
    GradualSessionMonitor, LType, Message, MonitorResult, ProbBranch, ProbSessionScheduler, Role,
    SType, SessionChecker, SessionEndpoint, SessionOp, SessionSubtypeChecker,
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
pub fn string_ty() -> Expr {
    cst("String")
}
/// `SType : Type`
/// The type of binary session types.
pub fn stype_ty() -> Expr {
    type0()
}
/// `Send : Type → SType → SType`
/// `!T.S` — send a value of type T, then continue as S.
pub fn send_ty() -> Expr {
    arrow(type0(), arrow(cst("SType"), cst("SType")))
}
/// `Recv : Type → SType → SType`
/// `?T.S` — receive a value of type T, then continue as S.
pub fn recv_ty() -> Expr {
    arrow(type0(), arrow(cst("SType"), cst("SType")))
}
/// `End : SType`
/// Session termination — the session has ended.
pub fn end_ty() -> Expr {
    cst("SType")
}
/// `SChoice : SType → SType → SType`
/// Internal choice: `S₁ ⊕ S₂` — select one branch to send.
pub fn schoice_ty() -> Expr {
    arrow(cst("SType"), arrow(cst("SType"), cst("SType")))
}
/// `SBranch : SType → SType → SType`
/// External choice: `S₁ & S₂` — offer both branches, peer selects.
pub fn sbranch_ty() -> Expr {
    arrow(cst("SType"), arrow(cst("SType"), cst("SType")))
}
/// `SRec : (SType → SType) → SType`
/// Iso-recursive session type: `μX.S(X)`.
pub fn srec_ty() -> Expr {
    arrow(arrow(cst("SType"), cst("SType")), cst("SType"))
}
/// `SVar : Nat → SType`
/// Session type variable (de Bruijn index).
pub fn svar_ty() -> Expr {
    arrow(nat_ty(), cst("SType"))
}
/// `dual : SType → SType`
/// Duality: swaps `Send` and `Recv`, swaps `SChoice` and `SBranch`.
pub fn dual_ty() -> Expr {
    arrow(cst("SType"), cst("SType"))
}
/// `dual_involutive : ∀ (S : SType), dual (dual S) = S`
pub fn dual_involutive_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "S",
        cst("SType"),
        app2(
            cst("Eq"),
            app(cst("dual"), app(cst("dual"), bvar(0))),
            bvar(0),
        ),
    )
}
/// `dual_send : ∀ (T : Type) (S : SType), dual (Send T S) = Recv T (dual S)`
pub fn dual_send_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "T",
        type0(),
        pi(
            BinderInfo::Default,
            "S",
            cst("SType"),
            app2(
                cst("Eq"),
                app(cst("dual"), app2(cst("Send"), bvar(1), bvar(0))),
                app2(cst("Recv"), bvar(1), app(cst("dual"), bvar(0))),
            ),
        ),
    )
}
/// `dual_end : dual End = End`
pub fn dual_end_ty() -> Expr {
    app2(cst("Eq"), app(cst("dual"), cst("End")), cst("End"))
}
/// `Channel : SType → Type`
/// A typed channel: `Chan S` is a channel whose protocol is `S`.
pub fn channel_ty() -> Expr {
    arrow(cst("SType"), type0())
}
/// `send_msg : {T : Type} → {S : SType} → Chan (Send T S) → T → Chan S`
/// Send a message along a channel.
pub fn send_msg_ty() -> Expr {
    impl_pi(
        "T",
        type0(),
        impl_pi(
            "S",
            cst("SType"),
            arrow(
                app(cst("Channel"), app2(cst("Send"), bvar(1), bvar(0))),
                arrow(bvar(2), app(cst("Channel"), bvar(1))),
            ),
        ),
    )
}
/// `recv_msg : {T : Type} → {S : SType} → Chan (Recv T S) → (T × Chan S)`
/// Receive a message from a channel.
pub fn recv_msg_ty() -> Expr {
    impl_pi(
        "T",
        type0(),
        impl_pi(
            "S",
            cst("SType"),
            arrow(
                app(cst("Channel"), app2(cst("Recv"), bvar(1), bvar(0))),
                app2(cst("Prod"), bvar(2), app(cst("Channel"), bvar(1))),
            ),
        ),
    )
}
/// `close : Chan End → Unit`
/// Close a finished channel.
pub fn close_ty() -> Expr {
    arrow(app(cst("Channel"), cst("End")), cst("Unit"))
}
/// `LinearType : Type → Prop`
/// Predicate asserting a type is linear (used exactly once).
pub fn linear_type_ty() -> Expr {
    arrow(type0(), prop())
}
/// `LinearCtx : Type`
/// A linear typing context: a finite map from names to (linear) types.
pub fn linear_ctx_ty() -> Expr {
    type0()
}
/// `CtxSplit : LinearCtx → LinearCtx → LinearCtx → Prop`
/// Context splitting: the left context is the disjoint union of the right two.
pub fn ctx_split_ty() -> Expr {
    arrow(
        cst("LinearCtx"),
        arrow(cst("LinearCtx"), arrow(cst("LinearCtx"), prop())),
    )
}
/// `LinearWellTyped : LinearCtx → Process → SType → Prop`
/// Typing judgment for linear processes.
pub fn linear_well_typed_ty() -> Expr {
    arrow(
        cst("LinearCtx"),
        arrow(type0(), arrow(cst("SType"), prop())),
    )
}
/// `LinearSafety : Process → Prop`
/// A process is safe if channels are used linearly (exactly once).
pub fn linear_safety_ty() -> Expr {
    arrow(type0(), prop())
}
/// `AffineType : Type → Prop`
/// Predicate for affine types (used at most once).
pub fn affine_type_ty() -> Expr {
    arrow(type0(), prop())
}
/// `UnrestrictedType : Type → Prop`
/// Predicate for unrestricted (reusable) types.
pub fn unrestricted_type_ty() -> Expr {
    arrow(type0(), prop())
}
/// `Role : Type`
/// A participant role in a multiparty session.
pub fn role_ty() -> Expr {
    type0()
}
/// `GType : Type`
/// A global session type (the overall protocol from a bird's-eye view).
pub fn gtype_ty() -> Expr {
    type0()
}
/// `GComm : Role → Role → Type → GType → GType`
/// `p → q : T. G` — role p sends a value of type T to role q, then continues as G.
pub fn gcomm_ty() -> Expr {
    arrow(
        cst("Role"),
        arrow(
            cst("Role"),
            arrow(type0(), arrow(cst("GType"), cst("GType"))),
        ),
    )
}
/// `GChoice : Role → Role → GType → GType → GType`
/// `p + q { G₁, G₂ }` — role p makes a choice communicated to q.
pub fn gchoice_ty() -> Expr {
    arrow(
        cst("Role"),
        arrow(
            cst("Role"),
            arrow(cst("GType"), arrow(cst("GType"), cst("GType"))),
        ),
    )
}
/// `GEnd : GType`
/// The end of the global protocol.
pub fn gend_ty() -> Expr {
    cst("GType")
}
/// `GRec : (GType → GType) → GType`
/// Recursive global type: `μX.G(X)`.
pub fn grec_ty() -> Expr {
    arrow(arrow(cst("GType"), cst("GType")), cst("GType"))
}
/// `WellFormed : GType → Prop`
/// A global type is well-formed if all roles are consistent and the protocol terminates.
pub fn well_formed_ty() -> Expr {
    arrow(cst("GType"), prop())
}
/// `Participants : GType → List Role`
/// Extract the set of roles participating in a global type.
pub fn participants_ty() -> Expr {
    arrow(cst("GType"), list_ty(cst("Role")))
}
/// `LType : Type`
/// A local session type: the protocol from one participant's perspective.
pub fn ltype_ty() -> Expr {
    type0()
}
/// `LSend : Role → Type → LType → LType`
/// `!q(T).L` — send T to role q, continue as L.
pub fn lsend_ty() -> Expr {
    arrow(
        cst("Role"),
        arrow(type0(), arrow(cst("LType"), cst("LType"))),
    )
}
/// `LRecv : Role → Type → LType → LType`
/// `?p(T).L` — receive T from role p, continue as L.
pub fn lrecv_ty() -> Expr {
    arrow(
        cst("Role"),
        arrow(type0(), arrow(cst("LType"), cst("LType"))),
    )
}
/// `LChoice : Role → LType → LType → LType`
/// Internal choice offered to role `r`.
pub fn lchoice_ty() -> Expr {
    arrow(
        cst("Role"),
        arrow(cst("LType"), arrow(cst("LType"), cst("LType"))),
    )
}
/// `LBranch : Role → LType → LType → LType`
/// External choice from role `r`.
pub fn lbranch_ty() -> Expr {
    arrow(
        cst("Role"),
        arrow(cst("LType"), arrow(cst("LType"), cst("LType"))),
    )
}
/// `LEnd : LType`
/// Session end from one participant's perspective.
pub fn lend_ty() -> Expr {
    cst("LType")
}
/// `project : GType → Role → LType`
/// Project a global type onto a specific role's local type.
pub fn project_ty() -> Expr {
    arrow(cst("GType"), arrow(cst("Role"), cst("LType")))
}
/// `ProjectionSound : ∀ (G : GType) (p : Role), WellFormed G →
///   LocallyConsistent (project G p)`
/// Soundness of projection: projections of well-formed globals are locally consistent.
pub fn projection_sound_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        cst("GType"),
        pi(
            BinderInfo::Default,
            "p",
            cst("Role"),
            arrow(
                app(cst("WellFormed"), bvar(1)),
                app(
                    cst("LocallyConsistent"),
                    app2(cst("project"), bvar(2), bvar(1)),
                ),
            ),
        ),
    )
}
/// `LocallyConsistent : LType → Prop`
pub fn locally_consistent_ty() -> Expr {
    arrow(cst("LType"), prop())
}
/// `Mergeability : LType → LType → Prop`
/// Two local types can be merged (for projection of choices not involving a role).
pub fn mergeability_ty() -> Expr {
    arrow(cst("LType"), arrow(cst("LType"), prop()))
}
/// `Process : Type`
/// The type of processes in the session calculus.
pub fn process_ty() -> Expr {
    type0()
}
/// `Reduces : Process → Process → Prop`
/// One-step reduction relation.
pub fn reduces_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// `Stuck : Process → Prop`
/// A process is stuck if it cannot reduce and is not a value.
pub fn stuck_ty() -> Expr {
    arrow(type0(), prop())
}
/// `DeadlockFree : Process → Prop`
/// A process is deadlock-free if all reachable states are not stuck.
pub fn deadlock_free_ty() -> Expr {
    arrow(type0(), prop())
}
/// `Progress : Process → SType → Prop`
/// Progress: a well-typed process either terminates or can take a step.
pub fn progress_ty() -> Expr {
    arrow(type0(), arrow(cst("SType"), prop()))
}
/// `SubjectReduction : Prop`
/// Subject reduction: typing is preserved by reduction.
pub fn subject_reduction_ty() -> Expr {
    prop()
}
/// `TypeSafety : Prop`
/// Type safety: well-typed processes do not go wrong.
pub fn type_safety_ty() -> Expr {
    prop()
}
/// `DeadlockFreedomThm : ∀ (P : Process) (S : SType), WellTyped P S → DeadlockFree P`
/// Deadlock freedom from type well-typedness.
pub fn deadlock_freedom_thm_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "P",
        type0(),
        pi(
            BinderInfo::Default,
            "S",
            cst("SType"),
            arrow(
                app2(cst("WellTypedProcess"), bvar(1), bvar(0)),
                app(cst("DeadlockFree"), bvar(2)),
            ),
        ),
    )
}
/// `WellTypedProcess : Process → SType → Prop`
pub fn well_typed_process_ty() -> Expr {
    arrow(type0(), arrow(cst("SType"), prop()))
}
/// `ProgressThm : ∀ (P : Process) (S : SType), WellTyped P S →
///   IsValue P ∨ ∃ P', Reduces P P'`
pub fn progress_thm_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "P",
        type0(),
        pi(
            BinderInfo::Default,
            "S",
            cst("SType"),
            arrow(
                app2(cst("WellTypedProcess"), bvar(1), bvar(0)),
                app2(
                    cst("Or"),
                    app(cst("IsValue"), bvar(2)),
                    app(cst("Exists"), app(cst("Reduces"), bvar(3))),
                ),
            ),
        ),
    )
}
/// `IsValue : Process → Prop`
pub fn is_value_ty() -> Expr {
    arrow(type0(), prop())
}
/// `InferSType : Process → SType → Prop`
/// Session type inference judgment.
pub fn infer_stype_ty() -> Expr {
    arrow(type0(), arrow(cst("SType"), prop()))
}
/// `PrincipalSType : Process → SType → Prop`
/// The principal (most general) session type of a process.
pub fn principal_stype_ty() -> Expr {
    arrow(type0(), arrow(cst("SType"), prop()))
}
/// `STypeUnify : SType → SType → STypeSubst → Prop`
/// Unification of session types under a substitution.
pub fn stype_unify_ty() -> Expr {
    arrow(cst("SType"), arrow(cst("SType"), arrow(type0(), prop())))
}
/// `STypeSubst : Type`
/// A session type substitution.
pub fn stype_subst_ty() -> Expr {
    type0()
}
/// `InferenceComplete : Prop`
/// Completeness of session type inference.
pub fn inference_complete_ty() -> Expr {
    prop()
}
/// `InferenceSound : Prop`
/// Soundness of session type inference.
pub fn inference_sound_ty() -> Expr {
    prop()
}
/// `DepSType : Type`
/// Dependent session types: the continuation type can depend on communicated values.
pub fn dep_stype_ty() -> Expr {
    type0()
}
/// `DepSend : {T : Type} → (T → DepSType) → DepSType`
/// `!x : T. S(x)` — send x of type T, continuation depends on x.
pub fn dep_send_ty() -> Expr {
    impl_pi(
        "T",
        type0(),
        arrow(arrow(bvar(0), cst("DepSType")), cst("DepSType")),
    )
}
/// `DepRecv : {T : Type} → (T → DepSType) → DepSType`
/// `?x : T. S(x)` — receive x of type T, continuation depends on x.
pub fn dep_recv_ty() -> Expr {
    impl_pi(
        "T",
        type0(),
        arrow(arrow(bvar(0), cst("DepSType")), cst("DepSType")),
    )
}
/// `DepEnd : DepSType`
pub fn dep_end_ty() -> Expr {
    cst("DepSType")
}
/// `DepDual : DepSType → DepSType`
pub fn dep_dual_ty() -> Expr {
    arrow(cst("DepSType"), cst("DepSType"))
}
/// `DepChannel : DepSType → Type`
pub fn dep_channel_ty() -> Expr {
    arrow(cst("DepSType"), type0())
}
/// `DepSendMsg : {T : Type} → {S : T → DepSType} → DepChan (DepSend S) → (x : T) → DepChan (S x)`
pub fn dep_send_msg_ty() -> Expr {
    arrow(cst("DepSType"), arrow(type0(), arrow(type0(), type0())))
}
/// `Unfold : SType → SType`
/// Unfold one step of a recursive session type: `μX.S(X) ↦ S(μX.S(X))`.
pub fn unfold_ty() -> Expr {
    arrow(cst("SType"), cst("SType"))
}
/// `Fold : SType → SType`
/// Fold back a session type into a recursive form.
pub fn fold_ty() -> Expr {
    arrow(cst("SType"), cst("SType"))
}
/// `IsoRecursive : SType → Prop`
/// Predicate for iso-recursive session types (explicit fold/unfold).
pub fn iso_recursive_ty() -> Expr {
    arrow(cst("SType"), prop())
}
/// `EquiRecursive : SType → Prop`
/// Predicate for equi-recursive session types (implicit fold/unfold).
pub fn equi_recursive_ty() -> Expr {
    arrow(cst("SType"), prop())
}
/// `Coinductive : SType → Prop`
/// Coinductive (possibly infinite) session type.
pub fn coinductive_ty() -> Expr {
    arrow(cst("SType"), prop())
}
/// `STypeEquiv : SType → SType → Prop`
/// Bisimilarity / coinductive equality of session types.
pub fn stype_equiv_ty() -> Expr {
    arrow(cst("SType"), arrow(cst("SType"), prop()))
}
/// `UnfoldPreservesDual : ∀ S, dual (Unfold S) = Unfold (dual S)`
pub fn unfold_preserves_dual_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "S",
        cst("SType"),
        app2(
            cst("Eq"),
            app(cst("dual"), app(cst("Unfold"), bvar(0))),
            app(cst("Unfold"), app(cst("dual"), bvar(0))),
        ),
    )
}
/// `AsyncSType : Type`
/// Asynchronous session type with buffered channels.
pub fn async_stype_ty() -> Expr {
    type0()
}
/// `AsyncSend : Type → AsyncSType → AsyncSType`
/// Asynchronous send: message is placed in a buffer; continuation does not wait.
pub fn async_send_ty() -> Expr {
    arrow(type0(), arrow(cst("AsyncSType"), cst("AsyncSType")))
}
/// `AsyncRecv : Type → AsyncSType → AsyncSType`
/// Asynchronous receive: eventually retrieve from buffer; continuation after receipt.
pub fn async_recv_ty() -> Expr {
    arrow(type0(), arrow(cst("AsyncSType"), cst("AsyncSType")))
}
/// `EventualDelivery : AsyncSType → Prop`
/// Every message sent is eventually delivered (liveness guarantee).
pub fn eventual_delivery_ty() -> Expr {
    arrow(cst("AsyncSType"), prop())
}
/// `BufferBound : AsyncSType → Nat → Prop`
/// The buffer size is bounded by `n`; prevents unbounded queuing.
pub fn buffer_bound_ty() -> Expr {
    arrow(cst("AsyncSType"), arrow(nat_ty(), prop()))
}
/// `AsyncDual : AsyncSType → AsyncSType`
/// Dual of an asynchronous session type.
pub fn async_dual_ty() -> Expr {
    arrow(cst("AsyncSType"), cst("AsyncSType"))
}
/// `AsyncEnd : AsyncSType`
/// Termination of an asynchronous session.
pub fn async_end_ty() -> Expr {
    cst("AsyncSType")
}
/// `LLProp : Type`
/// Propositions in multiplicative intuitionistic linear logic (MILL).
pub fn ll_prop_ty() -> Expr {
    type0()
}
/// `Tensor : LLProp → LLProp → LLProp`
/// Multiplicative conjunction: `A ⊗ B`.
pub fn tensor_ty() -> Expr {
    arrow(cst("LLProp"), arrow(cst("LLProp"), cst("LLProp")))
}
/// `Lolli : LLProp → LLProp → LLProp`
/// Linear implication: `A ⊸ B`.
pub fn lolli_ty() -> Expr {
    arrow(cst("LLProp"), arrow(cst("LLProp"), cst("LLProp")))
}
/// `Par : LLProp → LLProp → LLProp`
/// Multiplicative disjunction: `A ⅋ B`.
pub fn par_ty() -> Expr {
    arrow(cst("LLProp"), arrow(cst("LLProp"), cst("LLProp")))
}
/// `Bang : LLProp → LLProp`
/// Exponential modality: `!A` — unrestricted use.
pub fn bang_ty() -> Expr {
    arrow(cst("LLProp"), cst("LLProp"))
}
/// `WhyNot : LLProp → LLProp`
/// Exponential modality: `?A` — potential use.
pub fn why_not_ty() -> Expr {
    arrow(cst("LLProp"), cst("LLProp"))
}
/// `CutElim : Prop`
/// Cut elimination theorem for MILL: every proof can be made cut-free.
pub fn cut_elim_ty() -> Expr {
    prop()
}
/// `CurryHowardLL : LLProp → SType → Prop`
/// The Curry-Howard correspondence between MILL propositions and session types.
pub fn curry_howard_ll_ty() -> Expr {
    arrow(cst("LLProp"), arrow(cst("SType"), prop()))
}
/// `SSubtype : SType → SType → Prop`
/// Behavioral subtyping: `S₁ <: S₂` means S₁ can be used where S₂ is expected.
pub fn ssubtype_ty() -> Expr {
    arrow(cst("SType"), arrow(cst("SType"), prop()))
}
/// `SSubtypeRefl : ∀ S, SSubtype S S`
pub fn ssubtype_refl_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "S",
        cst("SType"),
        app2(cst("SSubtype"), bvar(0), bvar(0)),
    )
}
/// `SSubtypeTrans : ∀ S₁ S₂ S₃, SSubtype S₁ S₂ → SSubtype S₂ S₃ → SSubtype S₁ S₃`
pub fn ssubtype_trans_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "S1",
        cst("SType"),
        pi(
            BinderInfo::Default,
            "S2",
            cst("SType"),
            pi(
                BinderInfo::Default,
                "S3",
                cst("SType"),
                arrow(
                    app2(cst("SSubtype"), bvar(2), bvar(1)),
                    arrow(
                        app2(cst("SSubtype"), bvar(2), bvar(1)),
                        app2(cst("SSubtype"), bvar(4), bvar(2)),
                    ),
                ),
            ),
        ),
    )
}
/// `SendCovariant : ∀ T S₁ S₂, SSubtype S₁ S₂ → SSubtype (Send T S₁) (Send T S₂)`
/// Send is covariant in the continuation.
pub fn send_covariant_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "T",
        type0(),
        pi(
            BinderInfo::Default,
            "S1",
            cst("SType"),
            pi(
                BinderInfo::Default,
                "S2",
                cst("SType"),
                arrow(
                    app2(cst("SSubtype"), bvar(1), bvar(0)),
                    app2(
                        cst("SSubtype"),
                        app2(cst("Send"), bvar(2), bvar(2)),
                        app2(cst("Send"), bvar(2), bvar(1)),
                    ),
                ),
            ),
        ),
    )
}
/// `RecvContravariant : ∀ T S₁ S₂, SSubtype S₁ S₂ → SSubtype (Recv T S₂) (Recv T S₁)`
/// Receive is contravariant in the continuation (dual direction).
pub fn recv_contravariant_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "T",
        type0(),
        pi(
            BinderInfo::Default,
            "S1",
            cst("SType"),
            pi(
                BinderInfo::Default,
                "S2",
                cst("SType"),
                arrow(
                    app2(cst("SSubtype"), bvar(1), bvar(0)),
                    app2(
                        cst("SSubtype"),
                        app2(cst("Recv"), bvar(2), bvar(1)),
                        app2(cst("Recv"), bvar(2), bvar(2)),
                    ),
                ),
            ),
        ),
    )
}
/// `HOSType : Type`
/// Higher-order session types: channels can carry session channels.
pub fn ho_stype_ty() -> Expr {
    type0()
}
/// `SendChan : SType → HOSType → HOSType`
/// `!Chan(S).T` — send a channel of type S, then continue as T.
pub fn send_chan_ty() -> Expr {
    arrow(cst("SType"), arrow(cst("HOSType"), cst("HOSType")))
}
/// `RecvChan : SType → HOSType → HOSType`
/// `?Chan(S).T` — receive a channel of type S, then continue as T.
pub fn recv_chan_ty() -> Expr {
    arrow(cst("SType"), arrow(cst("HOSType"), cst("HOSType")))
}
/// `SessionPassing : HOSType → Prop`
/// A higher-order session type that involves session-passing.
pub fn session_passing_ty() -> Expr {
    arrow(cst("HOSType"), prop())
}
/// `HODual : HOSType → HOSType`
/// Dual of a higher-order session type.
pub fn ho_dual_ty() -> Expr {
    arrow(cst("HOSType"), cst("HOSType"))
}
/// `ExSType : Type`
/// Session types extended with exceptions and failure handling.
pub fn ex_stype_ty() -> Expr {
    type0()
}
/// `Throw : Type → ExSType`
/// Raise an exception of the given type, terminating the session.
pub fn throw_ty() -> Expr {
    arrow(type0(), cst("ExSType"))
}
/// `Catch : ExSType → ExSType → ExSType`
/// `try S catch H` — run S, if exception H is raised, switch to handler.
pub fn catch_ty() -> Expr {
    arrow(cst("ExSType"), arrow(cst("ExSType"), cst("ExSType")))
}
/// `ExceptionSafe : ExSType → Prop`
/// A session type is exception-safe if all exceptions are handled.
pub fn exception_safe_ty() -> Expr {
    arrow(cst("ExSType"), prop())
}
/// `FaultTolerant : ExSType → Prop`
/// A process is fault-tolerant if it can recover from any exception.
pub fn fault_tolerant_ty() -> Expr {
    arrow(cst("ExSType"), prop())
}
/// `GSType : Type`
/// Gradual session types: mixing static and dynamic checking.
pub fn g_stype_ty() -> Expr {
    type0()
}
/// `GradualSend : Type → GSType → GSType`
/// Gradual send: partially statically checked.
pub fn gradual_send_ty() -> Expr {
    arrow(type0(), arrow(cst("GSType"), cst("GSType")))
}
/// `GradualRecv : Type → GSType → GSType`
/// Gradual receive: partially statically checked.
pub fn gradual_recv_ty() -> Expr {
    arrow(type0(), arrow(cst("GSType"), cst("GSType")))
}
/// `DynSType : GSType`
/// The dynamic session type `?` — completely unchecked at compile time.
pub fn dyn_stype_ty() -> Expr {
    cst("GSType")
}
/// `CastInsertion : GSType → SType → GSType → Prop`
/// Cast insertion: convert between gradual and static session types.
pub fn cast_insertion_ty() -> Expr {
    arrow(
        cst("GSType"),
        arrow(cst("SType"), arrow(cst("GSType"), prop())),
    )
}
/// `DynamicMonitor : GSType → Prop`
/// A session type that must be monitored at runtime.
pub fn dynamic_monitor_ty() -> Expr {
    arrow(cst("GSType"), prop())
}
/// `GradualConsistency : GSType → GSType → Prop`
/// Gradual consistency: two session types agree on their static parts.
pub fn gradual_consistency_ty() -> Expr {
    arrow(cst("GSType"), arrow(cst("GSType"), prop()))
}
/// `ProbSType : Type`
/// Probabilistic session type with Markov-chain semantics.
pub fn prob_stype_ty() -> Expr {
    type0()
}
/// `ProbChoice : SType → SType → Expr`
/// `S₁ ⊕[p] S₂` — choose S₁ with probability p, S₂ with probability 1-p.
pub fn prob_choice_ty() -> Expr {
    arrow(
        cst("SType"),
        arrow(cst("SType"), arrow(cst("Real"), cst("ProbSType"))),
    )
}
/// `ExpectedCost : ProbSType → Nat → Prop`
/// Expected number of communication steps is bounded by n.
pub fn expected_cost_ty() -> Expr {
    arrow(cst("ProbSType"), arrow(nat_ty(), prop()))
}
/// `MarkovChainProtocol : ProbSType → Prop`
/// The probabilistic session type has a finite-state Markov chain interpretation.
pub fn markov_chain_protocol_ty() -> Expr {
    arrow(cst("ProbSType"), prop())
}
/// `Termination : ProbSType → Prop`
/// The probabilistic session terminates with probability 1 (almost-sure termination).
pub fn prob_termination_ty() -> Expr {
    arrow(cst("ProbSType"), prop())
}
/// `Choreography : Type`
/// A global choreography: the global view of a multiparty protocol.
pub fn choreography_ty() -> Expr {
    type0()
}
/// `Realization : Choreography → GType → Prop`
/// A choreography is realized by a global type.
pub fn realization_ty() -> Expr {
    arrow(cst("Choreography"), arrow(cst("GType"), prop()))
}
/// `GlobalView : GType → Choreography`
/// Extract the global view (choreography) from a global type.
pub fn global_view_ty() -> Expr {
    arrow(cst("GType"), cst("Choreography"))
}
/// `EndpointProjection : Choreography → Role → LType`
/// Project a choreography onto a role's local type.
pub fn endpoint_projection_ty() -> Expr {
    arrow(cst("Choreography"), arrow(cst("Role"), cst("LType")))
}
/// `ChoreographyConsistency : Choreography → Prop`
/// A choreography is consistent if all endpoint projections are compatible.
pub fn choreography_consistency_ty() -> Expr {
    arrow(cst("Choreography"), prop())
}
/// `AmbienceCompatibility : Choreography → GType → Prop`
/// The choreography is compatible with the ambient global type.
pub fn ambience_compatibility_ty() -> Expr {
    arrow(cst("Choreography"), arrow(cst("GType"), prop()))
}
/// `SharedMemType : Type`
/// Session types for shared memory concurrency.
pub fn shared_mem_type_ty() -> Expr {
    type0()
}
/// `Mutex : Type → SharedMemType`
/// A mutex protecting a resource of type T.
pub fn mutex_ty() -> Expr {
    arrow(type0(), cst("SharedMemType"))
}
/// `Acquire : SharedMemType → SType → SType`
/// Acquire a lock, then continue with a typed session.
pub fn acquire_ty() -> Expr {
    arrow(cst("SharedMemType"), arrow(cst("SType"), cst("SType")))
}
/// `Release : SharedMemType → SType → SType`
/// Release a lock, then continue.
pub fn release_ty() -> Expr {
    arrow(cst("SharedMemType"), arrow(cst("SType"), cst("SType")))
}
/// `DataRaceFree : SharedMemType → Prop`
/// A shared memory protocol is data-race free by type structure.
pub fn data_race_free_ty() -> Expr {
    arrow(cst("SharedMemType"), prop())
}
/// Build the session types environment.
pub fn build_session_types_env(env: &mut Environment) -> Result<(), String> {
    let axioms: &[(&str, Expr)] = &[
        ("SType", stype_ty()),
        ("Send", send_ty()),
        ("Recv", recv_ty()),
        ("End", end_ty()),
        ("SChoice", schoice_ty()),
        ("SBranch", sbranch_ty()),
        ("SRec", srec_ty()),
        ("SVar", svar_ty()),
        ("dual", dual_ty()),
        ("dual_involutive", dual_involutive_ty()),
        ("dual_send", dual_send_ty()),
        ("dual_end", dual_end_ty()),
        ("Channel", channel_ty()),
        ("send_msg", send_msg_ty()),
        ("recv_msg", recv_msg_ty()),
        ("close", close_ty()),
        ("LinearType", linear_type_ty()),
        ("LinearCtx", linear_ctx_ty()),
        ("CtxSplit", ctx_split_ty()),
        ("LinearWellTyped", linear_well_typed_ty()),
        ("LinearSafety", linear_safety_ty()),
        ("AffineType", affine_type_ty()),
        ("UnrestrictedType", unrestricted_type_ty()),
        ("Role", role_ty()),
        ("GType", gtype_ty()),
        ("GComm", gcomm_ty()),
        ("GChoice", gchoice_ty()),
        ("GEnd", gend_ty()),
        ("GRec", grec_ty()),
        ("WellFormed", well_formed_ty()),
        ("Participants", participants_ty()),
        ("LType", ltype_ty()),
        ("LSend", lsend_ty()),
        ("LRecv", lrecv_ty()),
        ("LChoice", lchoice_ty()),
        ("LBranch", lbranch_ty()),
        ("LEnd", lend_ty()),
        ("project", project_ty()),
        ("LocallyConsistent", locally_consistent_ty()),
        ("Mergeability", mergeability_ty()),
        ("ProjectionSound", projection_sound_ty()),
        ("Process", process_ty()),
        ("Reduces", reduces_ty()),
        ("Stuck", stuck_ty()),
        ("DeadlockFree", deadlock_free_ty()),
        ("Progress", progress_ty()),
        ("SubjectReduction", subject_reduction_ty()),
        ("TypeSafety", type_safety_ty()),
        ("WellTypedProcess", well_typed_process_ty()),
        ("DeadlockFreedomThm", deadlock_freedom_thm_ty()),
        ("ProgressThm", progress_thm_ty()),
        ("IsValue", is_value_ty()),
        ("InferSType", infer_stype_ty()),
        ("PrincipalSType", principal_stype_ty()),
        ("STypeUnify", stype_unify_ty()),
        ("STypeSubst", stype_subst_ty()),
        ("InferenceComplete", inference_complete_ty()),
        ("InferenceSound", inference_sound_ty()),
        ("DepSType", dep_stype_ty()),
        ("DepSend", dep_send_ty()),
        ("DepRecv", dep_recv_ty()),
        ("DepEnd", dep_end_ty()),
        ("DepDual", dep_dual_ty()),
        ("DepChannel", dep_channel_ty()),
        ("DepSendMsg", dep_send_msg_ty()),
        ("Unfold", unfold_ty()),
        ("Fold", fold_ty()),
        ("IsoRecursive", iso_recursive_ty()),
        ("EquiRecursive", equi_recursive_ty()),
        ("Coinductive", coinductive_ty()),
        ("STypeEquiv", stype_equiv_ty()),
        ("UnfoldPreservesDual", unfold_preserves_dual_ty()),
        ("AsyncSType", async_stype_ty()),
        ("AsyncSend", async_send_ty()),
        ("AsyncRecv", async_recv_ty()),
        ("EventualDelivery", eventual_delivery_ty()),
        ("BufferBound", buffer_bound_ty()),
        ("AsyncDual", async_dual_ty()),
        ("AsyncEnd", async_end_ty()),
        ("LLProp", ll_prop_ty()),
        ("Tensor", tensor_ty()),
        ("Lolli", lolli_ty()),
        ("Par", par_ty()),
        ("Bang", bang_ty()),
        ("WhyNot", why_not_ty()),
        ("CutElim", cut_elim_ty()),
        ("CurryHowardLL", curry_howard_ll_ty()),
        ("SSubtype", ssubtype_ty()),
        ("SSubtypeRefl", ssubtype_refl_ty()),
        ("SSubtypeTrans", ssubtype_trans_ty()),
        ("SendCovariant", send_covariant_ty()),
        ("RecvContravariant", recv_contravariant_ty()),
        ("HOSType", ho_stype_ty()),
        ("SendChan", send_chan_ty()),
        ("RecvChan", recv_chan_ty()),
        ("SessionPassing", session_passing_ty()),
        ("HODual", ho_dual_ty()),
        ("ExSType", ex_stype_ty()),
        ("Throw", throw_ty()),
        ("Catch", catch_ty()),
        ("ExceptionSafe", exception_safe_ty()),
        ("FaultTolerant", fault_tolerant_ty()),
        ("GSType", g_stype_ty()),
        ("GradualSend", gradual_send_ty()),
        ("GradualRecv", gradual_recv_ty()),
        ("DynSType", dyn_stype_ty()),
        ("CastInsertion", cast_insertion_ty()),
        ("DynamicMonitor", dynamic_monitor_ty()),
        ("GradualConsistency", gradual_consistency_ty()),
        ("ProbSType", prob_stype_ty()),
        ("ProbChoice", prob_choice_ty()),
        ("ExpectedCost", expected_cost_ty()),
        ("MarkovChainProtocol", markov_chain_protocol_ty()),
        ("ProbTermination", prob_termination_ty()),
        ("Choreography", choreography_ty()),
        ("Realization", realization_ty()),
        ("GlobalView", global_view_ty()),
        ("EndpointProjection", endpoint_projection_ty()),
        ("ChoreographyConsistency", choreography_consistency_ty()),
        ("AmbienceCompatibility", ambience_compatibility_ty()),
        ("SharedMemType", shared_mem_type_ty()),
        ("Mutex", mutex_ty()),
        ("Acquire", acquire_ty()),
        ("Release", release_ty()),
        ("DataRaceFree", data_race_free_ty()),
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
#[cfg(test)]
mod tests {
    use super::*;
    use oxilean_kernel::{Environment, Name};
    #[test]
    fn test_build_env_registers_axioms() {
        let mut env = Environment::new();
        build_session_types_env(&mut env).expect("build should succeed");
        assert!(env.get(&Name::str("SType")).is_some(), "SType missing");
        assert!(env.get(&Name::str("Send")).is_some(), "Send missing");
        assert!(env.get(&Name::str("Recv")).is_some(), "Recv missing");
        assert!(env.get(&Name::str("dual")).is_some(), "dual missing");
        assert!(env.get(&Name::str("GType")).is_some(), "GType missing");
        assert!(env.get(&Name::str("project")).is_some(), "project missing");
        assert!(
            env.get(&Name::str("DeadlockFree")).is_some(),
            "DeadlockFree missing"
        );
        assert!(
            env.get(&Name::str("DepSType")).is_some(),
            "DepSType missing"
        );
    }
    #[test]
    fn test_dual_involutive() {
        let st = SType::Send(
            Box::new(BaseType::Nat),
            Box::new(SType::Recv(Box::new(BaseType::Bool), Box::new(SType::End))),
        );
        let d = st.dual();
        let dd = d.dual();
        assert_eq!(st, dd, "dual should be involutive");
    }
    #[test]
    fn test_dual_swaps_send_recv() {
        let send = SType::Send(Box::new(BaseType::Nat), Box::new(SType::End));
        assert!(matches!(send.dual(), SType::Recv(_, _)));
        let recv = SType::Recv(Box::new(BaseType::Bool), Box::new(SType::End));
        assert!(matches!(recv.dual(), SType::Send(_, _)));
    }
    #[test]
    fn test_dual_choice() {
        let choice = SType::Choice(Box::new(SType::End), Box::new(SType::End));
        let d = choice.dual();
        assert!(matches!(d, SType::Branch(_, _)));
        let branch = SType::Branch(Box::new(SType::End), Box::new(SType::End));
        assert!(matches!(branch.dual(), SType::Choice(_, _)));
    }
    #[test]
    fn test_session_endpoint_send_recv() {
        let st = SType::Send(
            Box::new(BaseType::Nat),
            Box::new(SType::Recv(Box::new(BaseType::Nat), Box::new(SType::End))),
        );
        let mut ep = SessionEndpoint::new(st);
        ep.send(Message::Nat(42)).expect("send should succeed");
        assert!(ep.remaining.is_recv());
    }
    #[test]
    fn test_session_type_checker() {
        let st = SType::Send(
            Box::new(BaseType::Nat),
            Box::new(SType::Recv(Box::new(BaseType::Bool), Box::new(SType::End))),
        );
        let mut checker = SessionChecker::new();
        checker.register_channel("ch", st);
        let ops = vec![
            SessionOp::Send(BaseType::Nat),
            SessionOp::Recv(BaseType::Bool),
            SessionOp::Close,
        ];
        let result = checker.check_usage("ch", &ops);
        assert!(
            result.is_ok(),
            "Well-typed usage should type check: {:?}",
            result
        );
        assert_eq!(result.expect("result should be valid"), SType::End);
    }
    #[test]
    fn test_global_type_projection() {
        let alice = Role::new("Alice");
        let bob = Role::new("Bob");
        let global = GType::Comm {
            sender: alice.clone(),
            receiver: bob.clone(),
            msg_ty: BaseType::Nat,
            cont: Box::new(GType::End),
        };
        let alice_proj = global.project(&alice);
        let bob_proj = global.project(&bob);
        assert!(
            matches!(alice_proj, LType::Send(_, _, _)),
            "Alice should have LSend"
        );
        assert!(
            matches!(bob_proj, LType::Recv(_, _, _)),
            "Bob should have LRecv"
        );
    }
    #[test]
    fn test_deadlock_checker() {
        let mut checker = DeadlockChecker::new();
        checker.add_wait("ch1", "A", "B");
        checker.add_wait("ch2", "B", "C");
        assert!(checker.is_deadlock_free(), "Should be deadlock-free");
        checker.add_wait("ch3", "C", "A");
        assert!(
            !checker.is_deadlock_free(),
            "Cycle: should NOT be deadlock-free"
        );
    }
    #[test]
    fn test_recursive_session_unfold() {
        let nat_stream = SType::Rec(
            "X".to_string(),
            Box::new(SType::Send(
                Box::new(BaseType::Nat),
                Box::new(SType::Var("X".to_string())),
            )),
        );
        let unfolded = nat_stream.unfold();
        assert!(unfolded.is_send());
    }
    #[test]
    fn test_new_axioms_registered() {
        let mut env = Environment::new();
        build_session_types_env(&mut env).expect("build should succeed");
        assert!(env.get(&Name::str("AsyncSType")).is_some());
        assert!(env.get(&Name::str("AsyncSend")).is_some());
        assert!(env.get(&Name::str("EventualDelivery")).is_some());
        assert!(env.get(&Name::str("BufferBound")).is_some());
        assert!(env.get(&Name::str("AsyncDual")).is_some());
        assert!(env.get(&Name::str("LLProp")).is_some());
        assert!(env.get(&Name::str("Tensor")).is_some());
        assert!(env.get(&Name::str("Lolli")).is_some());
        assert!(env.get(&Name::str("Bang")).is_some());
        assert!(env.get(&Name::str("CutElim")).is_some());
        assert!(env.get(&Name::str("CurryHowardLL")).is_some());
        assert!(env.get(&Name::str("SSubtype")).is_some());
        assert!(env.get(&Name::str("SSubtypeRefl")).is_some());
        assert!(env.get(&Name::str("SendCovariant")).is_some());
        assert!(env.get(&Name::str("RecvContravariant")).is_some());
        assert!(env.get(&Name::str("HOSType")).is_some());
        assert!(env.get(&Name::str("SendChan")).is_some());
        assert!(env.get(&Name::str("SessionPassing")).is_some());
        assert!(env.get(&Name::str("ExSType")).is_some());
        assert!(env.get(&Name::str("Throw")).is_some());
        assert!(env.get(&Name::str("Catch")).is_some());
        assert!(env.get(&Name::str("FaultTolerant")).is_some());
        assert!(env.get(&Name::str("GSType")).is_some());
        assert!(env.get(&Name::str("DynSType")).is_some());
        assert!(env.get(&Name::str("CastInsertion")).is_some());
        assert!(env.get(&Name::str("GradualConsistency")).is_some());
        assert!(env.get(&Name::str("ProbSType")).is_some());
        assert!(env.get(&Name::str("ProbChoice")).is_some());
        assert!(env.get(&Name::str("MarkovChainProtocol")).is_some());
        assert!(env.get(&Name::str("ProbTermination")).is_some());
        assert!(env.get(&Name::str("Choreography")).is_some());
        assert!(env.get(&Name::str("Realization")).is_some());
        assert!(env.get(&Name::str("EndpointProjection")).is_some());
        assert!(env.get(&Name::str("ChoreographyConsistency")).is_some());
        assert!(env.get(&Name::str("SharedMemType")).is_some());
        assert!(env.get(&Name::str("Mutex")).is_some());
        assert!(env.get(&Name::str("Acquire")).is_some());
        assert!(env.get(&Name::str("DataRaceFree")).is_some());
    }
    #[test]
    fn test_async_endpoint_send_buffered() {
        let st = SType::Send(Box::new(BaseType::Nat), Box::new(SType::End));
        let mut ep = AsyncSessionEndpoint::new(st);
        ep.async_send(Message::Nat(7))
            .expect("async_send should succeed");
        assert_eq!(ep.outbox_len(), 1, "message should be in outbox");
        assert!(
            ep.remaining == SType::End,
            "remaining should advance to End"
        );
    }
    #[test]
    fn test_async_endpoint_flush_and_recv() {
        let sender_st = SType::Send(Box::new(BaseType::Nat), Box::new(SType::End));
        let recv_st = SType::Recv(Box::new(BaseType::Nat), Box::new(SType::End));
        let mut sender = AsyncSessionEndpoint::new(sender_st);
        let mut receiver = AsyncSessionEndpoint::new(recv_st);
        sender.async_send(Message::Nat(42)).expect("send ok");
        let flushed = sender.flush_to(&mut receiver);
        assert_eq!(flushed, 1);
        assert_eq!(receiver.inbox_len(), 1);
        let msg = receiver.async_recv().expect("recv ok");
        assert!(matches!(msg, Message::Nat(42)));
    }
    #[test]
    fn test_subtype_reflexivity() {
        let mut checker = SessionSubtypeChecker::new();
        let st = SType::Send(Box::new(BaseType::Nat), Box::new(SType::End));
        assert!(
            checker.is_subtype(&st, &st),
            "every type is a subtype of itself"
        );
    }
    #[test]
    fn test_subtype_end() {
        let mut checker = SessionSubtypeChecker::new();
        assert!(checker.is_subtype(&SType::End, &SType::End));
    }
    #[test]
    fn test_subtype_incompatible() {
        let mut checker = SessionSubtypeChecker::new();
        let send = SType::Send(Box::new(BaseType::Nat), Box::new(SType::End));
        let recv = SType::Recv(Box::new(BaseType::Nat), Box::new(SType::End));
        assert!(
            !checker.is_subtype(&send, &recv),
            "Send is not subtype of Recv"
        );
    }
    #[test]
    fn test_prob_scheduler_probabilities_sum_to_one() {
        let branches = vec![
            ProbBranch {
                label: "A".into(),
                weight: 1.0,
                cont: SType::End,
            },
            ProbBranch {
                label: "B".into(),
                weight: 3.0,
                cont: SType::End,
            },
        ];
        let sched = ProbSessionScheduler::new(branches);
        let probs = sched.probabilities();
        let sum: f64 = probs.iter().sum();
        assert!((sum - 1.0).abs() < 1e-9, "probabilities should sum to 1");
    }
    #[test]
    fn test_prob_scheduler_greedy_choice() {
        let branches = vec![
            ProbBranch {
                label: "low".into(),
                weight: 1.0,
                cont: SType::End,
            },
            ProbBranch {
                label: "high".into(),
                weight: 9.0,
                cont: SType::End,
            },
        ];
        let sched = ProbSessionScheduler::new(branches);
        assert_eq!(
            sched.greedy_choice(),
            Some(1),
            "highest weight branch is index 1"
        );
    }
    #[test]
    fn test_choreography_engine_simple() {
        let alice = Role::new("Alice");
        let bob = Role::new("Bob");
        let global = GType::Comm {
            sender: alice.clone(),
            receiver: bob.clone(),
            msg_ty: BaseType::Nat,
            cont: Box::new(GType::End),
        };
        let mut engine = ChoreographyEngine::new();
        engine.execute(&global).expect("execution should succeed");
        assert_eq!(engine.comm_count(), 1, "one communication step");
        assert!(matches!(engine.trace[0], ChoreographyStep::Comm { .. }));
        assert!(matches!(engine.trace[1], ChoreographyStep::End));
    }
    #[test]
    fn test_monitor_exact_match() {
        let st = SType::Send(Box::new(BaseType::Nat), Box::new(SType::End));
        let mut monitor = GradualSessionMonitor::new(st);
        let result = monitor.check_send(&BaseType::Nat);
        assert_eq!(result, MonitorResult::Ok);
        assert!(monitor.is_safe());
        assert!(monitor.casts.is_empty());
    }
    #[test]
    fn test_monitor_type_mismatch_inserts_cast() {
        let st = SType::Send(Box::new(BaseType::Nat), Box::new(SType::End));
        let mut monitor = GradualSessionMonitor::new(st);
        let result = monitor.check_send(&BaseType::Bool);
        assert!(matches!(result, MonitorResult::CastInserted(_)));
        assert!(!monitor.casts.is_empty());
    }
    #[test]
    fn test_monitor_wrong_direction_failure() {
        let st = SType::Recv(Box::new(BaseType::Nat), Box::new(SType::End));
        let mut monitor = GradualSessionMonitor::new(st);
        let result = monitor.check_send(&BaseType::Nat);
        assert!(matches!(result, MonitorResult::Failure(_)));
        assert!(!monitor.is_safe());
    }
}
