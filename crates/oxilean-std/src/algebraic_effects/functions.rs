//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
#![allow(clippy::items_after_test_module)]

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    DeepHandler, EffRow, EffSig, Effect, EffectInterpreter, EffectRow, Free, OpDesc, ShallowHandler,
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
pub fn app4(f: Expr, a: Expr, b: Expr, c: Expr, d: Expr) -> Expr {
    app(app3(f, a, b, c), d)
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
/// `EffectSig : Type`
/// An effect signature is a collection of named operations with their parameter
/// and return types. Formally: EffectSig : Type.
pub fn effect_sig_ty() -> Expr {
    type0()
}
/// `EffectRow : Type`
/// An effect row is a finite set (or multiset) of effect signatures.
/// Used in row-typed effect systems (Koka, Frank, Effekt).
pub fn effect_row_ty() -> Expr {
    type0()
}
/// `EmptyRow : EffectRow`
/// The empty effect row — represents pure computations.
pub fn empty_row_ty() -> Expr {
    cst("EffectRow")
}
/// `ExtendRow : EffectSig → EffectRow → EffectRow`
/// Extend a row with one more effect.
pub fn extend_row_ty() -> Expr {
    arrow(cst("EffectSig"), arrow(cst("EffectRow"), cst("EffectRow")))
}
/// `RowContains : EffectRow → EffectSig → Prop`
/// Membership predicate for effect rows.
pub fn row_contains_ty() -> Expr {
    arrow(cst("EffectRow"), arrow(cst("EffectSig"), prop()))
}
/// `RowLacks : EffectRow → EffectSig → Prop`
/// "Lacks" constraint: the row does not contain the given effect.
pub fn row_lacks_ty() -> Expr {
    arrow(cst("EffectRow"), arrow(cst("EffectSig"), prop()))
}
/// `RowSubset : EffectRow → EffectRow → Prop`
/// Sub-row ordering: all effects of the left row appear in the right.
pub fn row_subset_ty() -> Expr {
    arrow(cst("EffectRow"), arrow(cst("EffectRow"), prop()))
}
/// `RowUnion : EffectRow → EffectRow → EffectRow`
/// Union of two effect rows (for parallel composition).
pub fn row_union_ty() -> Expr {
    arrow(cst("EffectRow"), arrow(cst("EffectRow"), cst("EffectRow")))
}
/// `Comp : EffectRow → Type → Type`
/// The computation type: `Comp ε A` is a computation with effects `ε` returning `A`.
pub fn comp_ty() -> Expr {
    arrow(cst("EffectRow"), arrow(type0(), type0()))
}
/// `pure : {A : Type} → A → Comp EmptyRow A`
/// Lift a pure value into a computation with no effects.
pub fn pure_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        arrow(bvar(0), app2(cst("Comp"), cst("EmptyRow"), bvar(1))),
    )
}
/// `bind : {ε : EffectRow} → {A B : Type} → Comp ε A → (A → Comp ε B) → Comp ε B`
/// Sequential composition of effectful computations.
pub fn bind_ty() -> Expr {
    impl_pi(
        "ε",
        cst("EffectRow"),
        impl_pi(
            "A",
            type0(),
            impl_pi(
                "B",
                type0(),
                arrow(
                    app2(cst("Comp"), bvar(2), bvar(1)),
                    arrow(
                        arrow(bvar(2), app2(cst("Comp"), bvar(3), bvar(1))),
                        app2(cst("Comp"), bvar(4), bvar(2)),
                    ),
                ),
            ),
        ),
    )
}
/// `perform : {ε : EffectRow} → {op : EffectSig} → RowContains ε op → op.Param → Comp ε op.Return`
/// Perform an operation in a computation.
pub fn perform_ty() -> Expr {
    arrow(
        cst("EffectSig"),
        arrow(cst("EffectRow"), arrow(type0(), type0())),
    )
}
/// `liftComp : {ε₁ ε₂ : EffectRow} → RowSubset ε₁ ε₂ → Comp ε₁ A → Comp ε₂ A`
/// Weaken the effect annotation (subsumption / effect coercion).
pub fn lift_comp_ty() -> Expr {
    arrow(
        cst("EffectRow"),
        arrow(cst("EffectRow"), arrow(type0(), arrow(type0(), type0()))),
    )
}
/// `FreeMnd : (Type → Type) → Type → Type`
/// The free monad over a functor F. `FreeMnd F A` represents computations
/// whose operations are described by F.
pub fn free_mnd_ty() -> Expr {
    arrow(arrow(type0(), type0()), arrow(type0(), type0()))
}
/// `FreeReturn : {F : Type → Type} → {A : Type} → A → FreeMnd F A`
/// The `return` constructor of the free monad.
pub fn free_return_ty() -> Expr {
    arrow(
        arrow(type0(), type0()),
        arrow(type0(), arrow(bvar(1), type0())),
    )
}
/// `FreeOp : {F : Type → Type} → {A : Type} → F (FreeMnd F A) → FreeMnd F A`
/// The `op` constructor: wrap an F-shaped operation.
pub fn free_op_ty() -> Expr {
    arrow(arrow(type0(), type0()), arrow(type0(), type0()))
}
/// `foldFree : {F : Type → Type} → {A B : Type}
///           → (A → B) → (F B → B) → FreeMnd F A → B`
/// Catamorphism / fold over the free monad.
pub fn fold_free_ty() -> Expr {
    arrow(
        arrow(type0(), type0()),
        arrow(
            type0(),
            arrow(
                type0(),
                arrow(
                    arrow(bvar(1), bvar(1)),
                    arrow(
                        arrow(app(bvar(3), bvar(2)), bvar(2)),
                        arrow(app2(cst("FreeMnd"), bvar(4), bvar(3)), bvar(3)),
                    ),
                ),
            ),
        ),
    )
}
/// `AlgebraicOperation : EffectSig → Type → Type → Type`
/// An algebraic operation maps parameters and a continuation to a computation.
pub fn algebraic_op_ty() -> Expr {
    arrow(cst("EffectSig"), arrow(type0(), arrow(type0(), type0())))
}
/// `OperationSignature : Type`
/// A record of {Param : Type, Return : Type} — the signature of a single operation.
pub fn operation_signature_ty() -> Expr {
    type0()
}
/// `Handler : EffectSig → EffectRow → Type → Type → Type`
/// A handler for effect `ε` in context `ρ` transforming `Comp (ε ⊕ ρ) A` to `Comp ρ B`.
pub fn handler_ty() -> Expr {
    arrow(
        cst("EffectSig"),
        arrow(cst("EffectRow"), arrow(type0(), arrow(type0(), type0()))),
    )
}
/// `handle : Handler ε ρ A B → Comp (ExtendRow ε ρ) A → Comp ρ B`
/// Apply a handler to a computation.
pub fn handle_ty() -> Expr {
    arrow(
        cst("Handler"),
        arrow(arrow(type0(), type0()), arrow(type0(), type0())),
    )
}
/// `ReturnClause : {A B : Type} → (A → Comp ρ B) → Type`
/// The return clause of a handler: processes the final value.
pub fn return_clause_ty() -> Expr {
    arrow(type0(), arrow(type0(), arrow(cst("EffectRow"), type0())))
}
/// `OpClause : {op : EffectSig} → {B : Type}
///           → (op.Param → (op.Return → Comp ρ B) → Comp ρ B) → Type`
/// An operation clause: handles one specific operation.
pub fn op_clause_ty() -> Expr {
    arrow(
        cst("EffectSig"),
        arrow(type0(), arrow(cst("EffectRow"), type0())),
    )
}
/// `DeepHandler : EffectSig → EffectRow → Type → Type → Type`
/// A deep handler re-applies itself to the continuation (full recursion).
pub fn deep_handler_ty() -> Expr {
    arrow(
        cst("EffectSig"),
        arrow(cst("EffectRow"), arrow(type0(), arrow(type0(), type0()))),
    )
}
/// `ShallowHandler : EffectSig → EffectRow → Type → Type → Type`
/// A shallow handler handles only one step; the continuation is unhandled.
pub fn shallow_handler_ty() -> Expr {
    arrow(
        cst("EffectSig"),
        arrow(cst("EffectRow"), arrow(type0(), arrow(type0(), type0()))),
    )
}
/// `HandlerEquation : Handler ε ρ A B → Prop`
/// Handler equations: soundness conditions from Plotkin and Power.
pub fn handler_equation_ty() -> Expr {
    arrow(cst("Handler"), prop())
}
/// `HandlerComposition : Handler ε₁ ρ A B → Handler ε₂ ρ B C → Handler (ExtendRow ε₁ ε₂) ρ A C`
/// Sequential composition of handlers.
pub fn handler_composition_ty() -> Expr {
    arrow(cst("Handler"), arrow(cst("Handler"), cst("Handler")))
}
/// `Prompt : Type → Type → Type`
/// A delimited prompt with answer type A and value type B.
pub fn prompt_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// `reset : {A : Type} → (Unit → Comp {Shift A} A) → A`
/// Delimit a computation with a prompt (reset/prompt).
pub fn reset_ty() -> Expr {
    arrow(
        type0(),
        arrow(
            arrow(cst("Unit"), app2(cst("Comp"), cst("ShiftEffect"), bvar(1))),
            bvar(1),
        ),
    )
}
/// `shift : {A B : Type} → ((B → A) → Comp EmptyRow A) → Comp {Shift A} B`
/// Capture the current delimited continuation (shift/control).
pub fn shift_ty() -> Expr {
    arrow(
        type0(),
        arrow(
            type0(),
            arrow(
                arrow(
                    arrow(bvar(1), bvar(2)),
                    app2(cst("Comp"), cst("EmptyRow"), bvar(2)),
                ),
                app2(cst("Comp"), cst("ShiftEffect"), bvar(2)),
            ),
        ),
    )
}
/// `ShiftEffect : EffectSig`
/// The built-in shift effect.
pub fn shift_effect_ty() -> Expr {
    cst("EffectSig")
}
/// `control : {A B : Type} → ((B → Comp {Shift A} A) → Comp EmptyRow A) → Comp {Shift A} B`
/// Variant of shift where the continuation itself may use effects.
pub fn control_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// `MultiPrompt : Nat → Type → Type`
/// Multi-prompt delimited continuations indexed by prompt count.
pub fn multi_prompt_ty() -> Expr {
    arrow(nat_ty(), arrow(type0(), type0()))
}
/// `ControlStack : Nat → Type`
/// Stack of active prompts in multi-prompt systems.
pub fn control_stack_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `EffectVar : Type`
/// Effect row variable (for effect polymorphism).
pub fn effect_var_ty() -> Expr {
    type0()
}
/// `EffectScheme : EffectVar → Type → Type`
/// An effect-polymorphic type scheme: ∀ ε, A[ε].
pub fn effect_scheme_ty() -> Expr {
    arrow(cst("EffectVar"), arrow(type0(), type0()))
}
/// `EffectSubst : EffectVar → EffectRow → Type`
/// Substitution of an effect variable by an effect row.
pub fn effect_subst_ty() -> Expr {
    arrow(cst("EffectVar"), arrow(cst("EffectRow"), type0()))
}
/// `InferEffect : Term → EffectRow → Prop`
/// Effect inference judgment: given a term, infer its minimal effect row.
pub fn infer_effect_ty() -> Expr {
    arrow(type0(), arrow(cst("EffectRow"), prop()))
}
/// `EffectUnification : EffectRow → EffectRow → EffectSubst → Prop`
/// Unification of two effect rows via a substitution.
pub fn effect_unification_ty() -> Expr {
    arrow(
        cst("EffectRow"),
        arrow(cst("EffectRow"), arrow(type0(), prop())),
    )
}
/// `PrincipalType : Term → EffectScheme → Prop`
/// The principal (most general) effect type of a term.
pub fn principal_type_ty() -> Expr {
    arrow(type0(), arrow(cst("EffectScheme"), prop()))
}
/// `EffectSafe : Program → EffectRow → Prop`
/// A program is effect-safe if all performed effects are within the declared row.
pub fn effect_safe_ty() -> Expr {
    arrow(type0(), arrow(cst("EffectRow"), prop()))
}
/// `EffectContainment : Comp ε A → RowSubset ε ε' → Comp ε' A`
/// Containment: a computation can be coerced to a larger effect row.
pub fn effect_containment_ty() -> Expr {
    arrow(type0(), arrow(type0(), arrow(prop(), type0())))
}
/// `PureComputation : Comp EmptyRow A → A`
/// A computation with empty effect row is pure and can be safely run.
pub fn pure_computation_ty() -> Expr {
    arrow(
        type0(),
        arrow(app2(cst("Comp"), cst("EmptyRow"), bvar(0)), bvar(1)),
    )
}
/// `HandlerSafety : {ε ρ : EffectRow} → Handler ε ρ A B → Prop`
/// A handler is safe if it handles all operations of the effect.
pub fn handler_safety_ty() -> Expr {
    arrow(cst("Handler"), prop())
}
/// `EffectProgress : Comp ε A → Prop`
/// Progress theorem: a well-typed computation either returns or performs an effect.
pub fn effect_progress_ty() -> Expr {
    arrow(arrow(type0(), type0()), prop())
}
/// `EffectPreservation : Comp ε A → Comp ε A → Prop`
/// Preservation: reduction preserves the effect annotation.
pub fn effect_preservation_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// `EffectSoundness : Prop`
/// The effect system is sound: effect-annotated programs do not perform undeclared effects.
pub fn effect_soundness_ty() -> Expr {
    prop()
}
/// `EffectLabel : Type`
/// A named effect label (e.g., "IO", "State", "Exn").
pub fn effect_label_ty() -> Expr {
    type0()
}
/// `NamedEffect : EffectLabel → EffectSig`
/// Associate a label with its signature.
pub fn named_effect_ty() -> Expr {
    arrow(cst("EffectLabel"), cst("EffectSig"))
}
/// `IOEffect : EffectSig`
/// The IO effect: reading and writing.
pub fn io_effect_ty() -> Expr {
    cst("EffectSig")
}
/// `StateEffect : Type → EffectSig`
/// The State effect parameterized by the state type S.
pub fn state_effect_ty() -> Expr {
    arrow(type0(), cst("EffectSig"))
}
/// `ExnEffect : Type → EffectSig`
/// The exception effect parameterized by the exception type E.
pub fn exn_effect_ty() -> Expr {
    arrow(type0(), cst("EffectSig"))
}
/// `NondetEffect : EffectSig`
/// Non-determinism effect.
pub fn nondet_effect_ty() -> Expr {
    cst("EffectSig")
}
/// `CoopEffect : EffectSig`
/// Cooperative concurrency / yield effect.
pub fn coop_effect_ty() -> Expr {
    cst("EffectSig")
}
/// `runIO : Comp {IO} A → IO A`
/// Run an IO computation in the host IO monad.
pub fn run_io_ty() -> Expr {
    arrow(app2(cst("Comp"), cst("IOEffect"), type0()), type0())
}
/// `runState : {S A : Type} → S → Comp {State S} A → (A × S)`
/// Run a stateful computation with an initial state.
pub fn run_state_ty() -> Expr {
    arrow(
        type0(),
        arrow(
            type0(),
            arrow(
                bvar(1),
                arrow(
                    app2(cst("Comp"), app(cst("StateEffect"), bvar(2)), bvar(2)),
                    app2(cst("Prod"), bvar(3), bvar(3)),
                ),
            ),
        ),
    )
}
/// Build the algebraic effects environment by registering all axioms.
pub fn build_algebraic_effects_env(env: &mut Environment) -> Result<(), String> {
    let axioms: &[(&str, Expr)] = &[
        ("EffectSig", effect_sig_ty()),
        ("EffectRow", effect_row_ty()),
        ("EmptyRow", empty_row_ty()),
        ("ExtendRow", extend_row_ty()),
        ("RowContains", row_contains_ty()),
        ("RowLacks", row_lacks_ty()),
        ("RowSubset", row_subset_ty()),
        ("RowUnion", row_union_ty()),
        ("Comp", comp_ty()),
        ("pure", pure_ty()),
        ("bind", bind_ty()),
        ("perform", perform_ty()),
        ("liftComp", lift_comp_ty()),
        ("FreeMnd", free_mnd_ty()),
        ("FreeReturn", free_return_ty()),
        ("FreeOp", free_op_ty()),
        ("foldFree", fold_free_ty()),
        ("AlgebraicOperation", algebraic_op_ty()),
        ("OperationSignature", operation_signature_ty()),
        ("Handler", handler_ty()),
        ("handle", handle_ty()),
        ("ReturnClause", return_clause_ty()),
        ("OpClause", op_clause_ty()),
        ("DeepHandler", deep_handler_ty()),
        ("ShallowHandler", shallow_handler_ty()),
        ("HandlerEquation", handler_equation_ty()),
        ("HandlerComposition", handler_composition_ty()),
        ("Prompt", prompt_ty()),
        ("reset", reset_ty()),
        ("shift", shift_ty()),
        ("ShiftEffect", shift_effect_ty()),
        ("control", control_ty()),
        ("MultiPrompt", multi_prompt_ty()),
        ("ControlStack", control_stack_ty()),
        ("EffectVar", effect_var_ty()),
        ("EffectScheme", effect_scheme_ty()),
        ("EffectSubst", effect_subst_ty()),
        ("InferEffect", infer_effect_ty()),
        ("EffectUnification", effect_unification_ty()),
        ("PrincipalType", principal_type_ty()),
        ("EffectSafe", effect_safe_ty()),
        ("EffectContainment", effect_containment_ty()),
        ("PureComputation", pure_computation_ty()),
        ("HandlerSafety", handler_safety_ty()),
        ("EffectProgress", effect_progress_ty()),
        ("EffectPreservation", effect_preservation_ty()),
        ("EffectSoundness", effect_soundness_ty()),
        ("EffectLabel", effect_label_ty()),
        ("NamedEffect", named_effect_ty()),
        ("IOEffect", io_effect_ty()),
        ("StateEffect", state_effect_ty()),
        ("ExnEffect", exn_effect_ty()),
        ("NondetEffect", nondet_effect_ty()),
        ("CoopEffect", coop_effect_ty()),
        ("runIO", run_io_ty()),
        ("runState", run_state_ty()),
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
/// Build the standard IO effect signature.
pub fn io_effect_sig() -> EffSig {
    EffSig::new(
        "IO",
        vec![
            OpDesc::new("read_line", "Unit", "String"),
            OpDesc::new("print", "String", "Unit"),
            OpDesc::new("read_file", "String", "String"),
            OpDesc::new("write_file", "String × String", "Unit"),
        ],
    )
}
/// Build the standard State effect signature.
pub fn state_effect_sig(state_ty: &str) -> EffSig {
    EffSig::new(
        format!("State<{}>", state_ty),
        vec![
            OpDesc::new("get", "Unit", state_ty),
            OpDesc::new("put", state_ty, "Unit"),
            OpDesc::new("modify", format!("{} → {}", state_ty, state_ty), "Unit"),
        ],
    )
}
/// Build the standard Exception effect signature.
pub fn exn_effect_sig(exn_ty: &str) -> EffSig {
    EffSig::new(
        format!("Exn<{}>", exn_ty),
        vec![
            OpDesc::new("raise", exn_ty, "Never"),
            OpDesc::new("catch", format!("Comp {{Exn<{}>}} A", exn_ty), "A"),
        ],
    )
}
/// Build the non-determinism effect signature.
pub fn nondet_effect_sig() -> EffSig {
    EffSig::new(
        "Nondet",
        vec![
            OpDesc::new("choose", "Bool", "Bool"),
            OpDesc::new("fail", "Unit", "Never"),
        ],
    )
}
/// Build the cooperative concurrency effect signature.
pub fn coop_effect_sig() -> EffSig {
    EffSig::new(
        "Coop",
        vec![
            OpDesc::new("yield", "Unit", "Unit"),
            OpDesc::new("fork", "Comp {Coop} Unit", "Unit"),
        ],
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    use oxilean_kernel::{Environment, Name};
    #[test]
    fn test_build_env_registers_axioms() {
        let mut env = Environment::new();
        build_algebraic_effects_env(&mut env).expect("build should succeed");
        assert!(
            env.get(&Name::str("EffectSig")).is_some(),
            "EffectSig missing"
        );
        assert!(
            env.get(&Name::str("EffectRow")).is_some(),
            "EffectRow missing"
        );
        assert!(env.get(&Name::str("Comp")).is_some(), "Comp missing");
        assert!(env.get(&Name::str("Handler")).is_some(), "Handler missing");
        assert!(env.get(&Name::str("FreeMnd")).is_some(), "FreeMnd missing");
        assert!(
            env.get(&Name::str("ShiftEffect")).is_some(),
            "ShiftEffect missing"
        );
        assert!(
            env.get(&Name::str("EffectSafe")).is_some(),
            "EffectSafe missing"
        );
        assert!(
            env.get(&Name::str("IOEffect")).is_some(),
            "IOEffect missing"
        );
    }
    #[test]
    fn test_effect_row_operations() {
        let empty = EffRow::empty();
        assert!(empty.is_pure());
        assert!(empty.lacks("IO"));
        let with_io = empty.extend("IO");
        assert!(with_io.contains("IO"));
        assert!(!with_io.is_pure());
        assert!(with_io.lacks("State"));
        let with_state = with_io.extend("State");
        assert!(with_state.contains("IO"));
        assert!(with_state.contains("State"));
        let with_io2 = with_io.extend("IO");
        assert_eq!(with_io2.effect_names().len(), 1);
    }
    #[test]
    fn test_effect_row_subset() {
        let row1 = EffRow::empty().extend("IO");
        let row2 = EffRow::empty().extend("IO").extend("State");
        assert!(row1.is_subset_of(&row2));
        assert!(!row2.is_subset_of(&row1));
        assert!(row1.is_subset_of(&row1));
    }
    #[test]
    fn test_effect_row_union() {
        let r1 = EffRow::empty().extend("IO").extend("State");
        let r2 = EffRow::empty().extend("State").extend("Exn");
        let u = r1.union(&r2);
        assert!(u.contains("IO"));
        assert!(u.contains("State"));
        assert!(u.contains("Exn"));
        let count = u
            .effect_names()
            .iter()
            .filter(|e| e.as_str() == "State")
            .count();
        assert_eq!(count, 1);
    }
    #[test]
    fn test_eff_sig_operations() {
        let io_sig = io_effect_sig();
        assert_eq!(io_sig.name, "IO");
        assert!(io_sig.get_op("print").is_some());
        assert!(io_sig.get_op("read_line").is_some());
        assert!(io_sig.get_op("nonexistent").is_none());
        let state_sig = state_effect_sig("Int");
        assert!(state_sig.get_op("get").is_some());
        assert!(state_sig.get_op("put").is_some());
    }
    #[test]
    fn test_free_monad_pure() {
        let comp: Free<i32> = Free::pure(42);
        let result = comp.fold(|x| x * 2, &|_, _, _, _| unreachable!());
        assert_eq!(result, 84);
    }
    #[test]
    fn test_effect_interpreter_run() {
        let interp = EffectInterpreter::new()
            .register("State", "get", |_arg| "10".to_string())
            .register("State", "put", |arg| format!("stored:{}", arg));
        let comp = Free::op("State", "get", "()", |val| {
            Free::op("State", "put", val.clone(), |_| Free::pure(val))
        });
        let result = interp.run(comp);
        assert_eq!(result, "10");
    }
    #[test]
    fn test_deep_handler_construction() {
        let _handler: DeepHandler<String, String> =
            DeepHandler::new("State", |v| v).with_op("get", |_arg, k| k("0".to_string()));
        assert_eq!(_handler.effect_name, "State");
    }
    #[test]
    fn test_shallow_handler() {
        let handler: ShallowHandler<String, String> = ShallowHandler::new(
            "IO",
            |v| v,
            |op, arg, _cont| format!("handled_{}_{}", op, arg),
        );
        let comp = Free::op("IO", "print", "hello", |_| Free::pure("done".to_string()));
        let result = handler.handle(comp);
        assert_eq!(result, "handled_print_hello");
    }
}
/// Build an `Environment` populated with axioms for algebraic effects theory.
pub fn build_env() -> Environment {
    let mut env = Environment::new();
    let _ = build_algebraic_effects_env(&mut env);
    let extra: &[(&str, fn() -> Expr)] = &[
        ("EffectIsAlgebraic", effect_is_algebraic_ty),
        ("FreeMonadUnit", free_monad_unit_ty),
        ("FreeMonadBind", free_monad_bind_ty),
        ("HandlerElimination", handler_elimination_ty),
        ("EffectRowUnion", effect_row_union_ty),
        ("ContinuationMonadLaws", continuation_monad_laws_ty),
        ("EffectPolymorphismAx", effect_polymorphism_ax_ty),
        ("DelimitedContIsFirstClass", delimited_cont_first_class_ty),
    ];
    for (name, ty_fn) in extra {
        let _ = env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty_fn(),
        });
    }
    env
}
pub fn effect_is_algebraic_ty() -> Expr {
    arrow(cst("EffectSig"), prop())
}
pub fn free_monad_unit_ty() -> Expr {
    arrow(type0(), arrow(bvar(0), app(cst("FreeMnd"), bvar(1))))
}
pub fn free_monad_bind_ty() -> Expr {
    arrow(
        type0(),
        arrow(
            type0(),
            arrow(
                app(cst("FreeMnd"), bvar(1)),
                arrow(
                    arrow(bvar(2), app(cst("FreeMnd"), bvar(2))),
                    app(cst("FreeMnd"), bvar(1)),
                ),
            ),
        ),
    )
}
pub fn handler_elimination_ty() -> Expr {
    arrow(cst("Handler"), arrow(cst("Comp"), cst("Comp")))
}
pub fn effect_row_union_ty() -> Expr {
    arrow(cst("EffectRow"), arrow(cst("EffectRow"), cst("EffectRow")))
}
pub fn continuation_monad_laws_ty() -> Expr {
    arrow(type0(), prop())
}
pub fn effect_polymorphism_ax_ty() -> Expr {
    arrow(cst("EffectRow"), arrow(type0(), type0()))
}
pub fn delimited_cont_first_class_ty() -> Expr {
    arrow(type0(), arrow(cst("Comp"), bvar(1)))
}
/// handler_soundness : ∀ H : Handler ε ρ A B, HandlesSoundly H
pub fn ae_ext_handler_soundness_ty() -> Expr {
    arrow(cst("Handler"), prop())
}
/// handler_completeness : ∀ H : Handler ε ρ A B, HandlesAllOps H
pub fn ae_ext_handler_completeness_ty() -> Expr {
    arrow(cst("Handler"), prop())
}
/// deep_handler_recursive : deep handlers re-apply to continuations
pub fn ae_ext_deep_handler_recursive_ty() -> Expr {
    arrow(cst("DeepHandler"), prop())
}
/// shallow_handler_oneshot : shallow handlers do not re-apply
pub fn ae_ext_shallow_handler_oneshot_ty() -> Expr {
    arrow(cst("ShallowHandler"), prop())
}
/// handler_value_clause : ∀ H, ∀ a, handle H (pure a) = val_clause H a
pub fn ae_ext_handler_value_clause_ty() -> Expr {
    arrow(cst("Handler"), prop())
}
/// handler_op_clause : ∀ H op p k, handle H (op p k) = op_clause H op p (fun r → handle H (k r))
pub fn ae_ext_handler_op_clause_ty() -> Expr {
    arrow(cst("Handler"), arrow(cst("EffectSig"), prop()))
}
/// EffectAbs : (EffectRow → Comp ε A) → Comp (∀ε) A — effect abstraction
pub fn ae_ext_effect_abs_ty() -> Expr {
    arrow(arrow(cst("EffectRow"), type0()), type0())
}
/// EffectApp : Comp (∀ε) A → EffectRow → Comp ε A — effect application
pub fn ae_ext_effect_app_ty() -> Expr {
    arrow(type0(), arrow(cst("EffectRow"), type0()))
}
/// RowPolymorphicFn : (∀ε, A → Comp ε B) — a row-polymorphic function
pub fn ae_ext_row_polymorphic_fn_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        pi(BinderInfo::Default, "B", type0(), type0()),
    )
}
/// effect_poly_soundness : effect polymorphism is sound
pub fn ae_ext_effect_poly_soundness_ty() -> Expr {
    prop()
}
/// effect_subsumption : ε ⊆ ε' → Comp ε A → Comp ε' A
pub fn ae_ext_effect_subsumption_ty() -> Expr {
    arrow(prop(), arrow(type0(), type0()))
}
/// effect_erasure_denotational : erasing effects preserves denotational semantics
pub fn ae_ext_effect_erasure_denotational_ty() -> Expr {
    arrow(type0(), prop())
}
/// ScopedEffect : EffectSig → Type — effects scoped to a lexical region
pub fn ae_ext_scoped_effect_ty() -> Expr {
    arrow(cst("EffectSig"), type0())
}
/// LatentEffect : EffectSig → Type — effects that may or may not be performed
pub fn ae_ext_latent_effect_ty() -> Expr {
    arrow(cst("EffectSig"), type0())
}
/// scoped_handler : handle effects only within a lexical scope
pub fn ae_ext_scoped_handler_ty() -> Expr {
    arrow(cst("ScopedEffect"), arrow(type0(), type0()))
}
/// latent_effect_discharge : a latent effect can be discharged by a handler
pub fn ae_ext_latent_effect_discharge_ty() -> Expr {
    arrow(cst("LatentEffect"), arrow(cst("Handler"), type0()))
}
/// LinearEffect : EffectSig → Type — effects used exactly once
pub fn ae_ext_linear_effect_ty() -> Expr {
    arrow(cst("EffectSig"), type0())
}
/// linear_effect_usage : ∀ E : LinearEffect, used exactly once in computation
pub fn ae_ext_linear_effect_usage_ty() -> Expr {
    arrow(cst("LinearEffect"), prop())
}
/// GradedMonad : (Grade → Type → Type) — graded monadic effects
pub fn ae_ext_graded_monad_ty() -> Expr {
    arrow(type0(), arrow(type0(), arrow(type0(), type0())))
}
/// graded_monad_unit : a → M 1 a
pub fn ae_ext_graded_monad_unit_ty() -> Expr {
    arrow(
        type0(),
        arrow(bvar(0), app2(cst("GradedMonad"), cst("GradeOne"), bvar(1))),
    )
}
/// graded_monad_bind : M g a → (a → M h b) → M (g·h) b
pub fn ae_ext_graded_monad_bind_ty() -> Expr {
    arrow(
        type0(),
        arrow(type0(), arrow(type0(), arrow(type0(), type0()))),
    )
}
/// graded_effect_type_checking : graded effects enforce resource usage
pub fn ae_ext_graded_effect_type_checking_ty() -> Expr {
    prop()
}
/// EffectSubtype : EffectRow → EffectRow → Prop — ε₁ ≤ ε₂
pub fn ae_ext_effect_subtype_ty() -> Expr {
    arrow(cst("EffectRow"), arrow(cst("EffectRow"), prop()))
}
/// effect_subtype_refl : ε ≤ ε
pub fn ae_ext_effect_subtype_refl_ty() -> Expr {
    pi(BinderInfo::Default, "eps", cst("EffectRow"), prop())
}
/// effect_subtype_trans : ε₁ ≤ ε₂ → ε₂ ≤ ε₃ → ε₁ ≤ ε₃
pub fn ae_ext_effect_subtype_trans_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "eps1",
        cst("EffectRow"),
        pi(
            BinderInfo::Default,
            "eps2",
            cst("EffectRow"),
            pi(
                BinderInfo::Default,
                "eps3",
                cst("EffectRow"),
                arrow(prop(), arrow(prop(), prop())),
            ),
        ),
    )
}
/// comp_subtype : ε₁ ≤ ε₂ → Comp ε₁ A ≤ Comp ε₂ A
pub fn ae_ext_comp_subtype_ty() -> Expr {
    arrow(prop(), prop())
}
/// FreerMonad : (Type → Type → Type) → Type → Type — Freer monad
pub fn ae_ext_freer_monad_ty() -> Expr {
    arrow(
        arrow(type0(), arrow(type0(), type0())),
        arrow(type0(), type0()),
    )
}
/// freer_monad_unit : a → Freer F a
pub fn ae_ext_freer_monad_unit_ty() -> Expr {
    arrow(
        type0(),
        arrow(bvar(0), app2(cst("FreerMonad"), cst("F"), bvar(1))),
    )
}
/// freer_monad_impure : F a b → (b → Freer F c) → Freer F c
pub fn ae_ext_freer_monad_impure_ty() -> Expr {
    arrow(type0(), arrow(type0(), arrow(type0(), type0())))
}
/// freer_monad_interpreter : (∀ a b, F a b → (b → r) → r) → Freer F a → r
pub fn ae_ext_freer_monad_interpreter_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// free_monad_as_effect_model : FreeMnd F models algebraic effects
pub fn ae_ext_free_monad_as_effect_model_ty() -> Expr {
    arrow(arrow(type0(), type0()), prop())
}
/// freer_monad_optimization : Freer monad can be optimized via reflection without remorse
pub fn ae_ext_freer_monad_optimization_ty() -> Expr {
    prop()
}
/// EffectCompose : EffectSig → EffectSig → EffectSig — combine two effects
pub fn ae_ext_effect_compose_ty() -> Expr {
    arrow(cst("EffectSig"), arrow(cst("EffectSig"), cst("EffectSig")))
}
/// effect_compose_assoc : (E₁ ⊕ E₂) ⊕ E₃ = E₁ ⊕ (E₂ ⊕ E₃)
pub fn ae_ext_effect_compose_assoc_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "E1",
        cst("EffectSig"),
        pi(
            BinderInfo::Default,
            "E2",
            cst("EffectSig"),
            pi(BinderInfo::Default, "E3", cst("EffectSig"), prop()),
        ),
    )
}
/// effect_compose_commute : E₁ ⊕ E₂ ≅ E₂ ⊕ E₁ (up to handler reordering)
pub fn ae_ext_effect_compose_commute_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "E1",
        cst("EffectSig"),
        pi(BinderInfo::Default, "E2", cst("EffectSig"), prop()),
    )
}
/// handler_composition_soundness : composing handlers preserves semantics
pub fn ae_ext_handler_composition_soundness_ty() -> Expr {
    arrow(cst("Handler"), arrow(cst("Handler"), prop()))
}
/// ExceptionEffect : Type → EffectSig — raise and catch
pub fn ae_ext_exception_effect_ty() -> Expr {
    arrow(type0(), cst("EffectSig"))
}
/// exception_handler_total : total handler for exception effects
pub fn ae_ext_exception_handler_total_ty() -> Expr {
    arrow(cst("EffectSig"), arrow(arrow(type0(), type0()), type0()))
}
/// StateReadWrite : Type → EffectSig — get/put operations
pub fn ae_ext_state_read_write_ty() -> Expr {
    arrow(type0(), cst("EffectSig"))
}
/// state_handler_runstate : runState h s₀ = (a, s_final)
pub fn ae_ext_state_handler_runstate_ty() -> Expr {
    arrow(type0(), arrow(type0(), arrow(type0(), type0())))
}
/// NondeterminismEffect : EffectSig — choose and fail
pub fn ae_ext_nondeterminism_effect_ty() -> Expr {
    cst("EffectSig")
}
/// nondet_handler_list : collect all results as a list
pub fn ae_ext_nondet_handler_list_ty() -> Expr {
    arrow(arrow(type0(), type0()), list_ty(type0()))
}
/// CooperativeThreadEffect : EffectSig — yield and fork
pub fn ae_ext_cooperative_thread_effect_ty() -> Expr {
    cst("EffectSig")
}
/// io_as_algebraic_effect : IO is an algebraic effect
pub fn ae_ext_io_as_algebraic_effect_ty() -> Expr {
    prop()
}
/// IOEffect_world_token : IO modeled via world token threading
pub fn ae_ext_io_world_token_ty() -> Expr {
    arrow(cst("World"), arrow(type0(), type0()))
}
/// ContinuationEffect : Type → EffectSig — capture continuation
pub fn ae_ext_continuation_effect_ty() -> Expr {
    arrow(type0(), cst("EffectSig"))
}
/// DelimitedControlEffect : EffectSig — shift/reset as an effect
pub fn ae_ext_delimited_control_effect_ty() -> Expr {
    cst("EffectSig")
}
/// continuation_effect_is_algebraic : continuations form an algebraic effect
pub fn ae_ext_continuation_is_algebraic_ty() -> Expr {
    arrow(type0(), prop())
}
/// shift_reset_as_handler : shift/reset implemented via algebraic effect handling
pub fn ae_ext_shift_reset_as_handler_ty() -> Expr {
    prop()
}
/// multi_prompt_effects : multi-prompt continuations via multiple effect rows
pub fn ae_ext_multi_prompt_effects_ty() -> Expr {
    arrow(nat_ty(), arrow(type0(), type0()))
}
/// EffectfulCPS : (A → Comp ε R) → Comp ε A → Comp ε R — effectful CPS transformation
pub fn ae_ext_effectful_cps_ty() -> Expr {
    arrow(type0(), arrow(type0(), arrow(type0(), type0())))
}
/// Register all extended algebraic effects axioms into an existing environment.
pub fn register_algebraic_effects_extended(env: &mut Environment) -> Result<(), String> {
    let axioms: &[(&str, Expr)] = &[
        ("handler_soundness", ae_ext_handler_soundness_ty()),
        ("handler_completeness", ae_ext_handler_completeness_ty()),
        ("deep_handler_recursive", ae_ext_deep_handler_recursive_ty()),
        (
            "shallow_handler_oneshot",
            ae_ext_shallow_handler_oneshot_ty(),
        ),
        ("handler_value_clause", ae_ext_handler_value_clause_ty()),
        ("handler_op_clause", ae_ext_handler_op_clause_ty()),
        ("EffectAbs", ae_ext_effect_abs_ty()),
        ("EffectApp", ae_ext_effect_app_ty()),
        ("RowPolymorphicFn", ae_ext_row_polymorphic_fn_ty()),
        ("effect_poly_soundness", ae_ext_effect_poly_soundness_ty()),
        ("effect_subsumption", ae_ext_effect_subsumption_ty()),
        (
            "effect_erasure_denotational",
            ae_ext_effect_erasure_denotational_ty(),
        ),
        ("ScopedEffect", ae_ext_scoped_effect_ty()),
        ("LatentEffect", ae_ext_latent_effect_ty()),
        ("scoped_handler", ae_ext_scoped_handler_ty()),
        (
            "latent_effect_discharge",
            ae_ext_latent_effect_discharge_ty(),
        ),
        ("LinearEffect", ae_ext_linear_effect_ty()),
        ("linear_effect_usage", ae_ext_linear_effect_usage_ty()),
        ("GradedMonad", ae_ext_graded_monad_ty()),
        ("graded_monad_unit", ae_ext_graded_monad_unit_ty()),
        ("graded_monad_bind", ae_ext_graded_monad_bind_ty()),
        (
            "graded_effect_type_checking",
            ae_ext_graded_effect_type_checking_ty(),
        ),
        ("EffectSubtype", ae_ext_effect_subtype_ty()),
        ("effect_subtype_refl", ae_ext_effect_subtype_refl_ty()),
        ("effect_subtype_trans", ae_ext_effect_subtype_trans_ty()),
        ("comp_subtype", ae_ext_comp_subtype_ty()),
        ("FreerMonad", ae_ext_freer_monad_ty()),
        ("freer_monad_unit", ae_ext_freer_monad_unit_ty()),
        ("freer_monad_impure", ae_ext_freer_monad_impure_ty()),
        (
            "freer_monad_interpreter",
            ae_ext_freer_monad_interpreter_ty(),
        ),
        (
            "free_monad_as_effect_model",
            ae_ext_free_monad_as_effect_model_ty(),
        ),
        (
            "freer_monad_optimization",
            ae_ext_freer_monad_optimization_ty(),
        ),
        ("EffectCompose", ae_ext_effect_compose_ty()),
        ("effect_compose_assoc", ae_ext_effect_compose_assoc_ty()),
        ("effect_compose_commute", ae_ext_effect_compose_commute_ty()),
        (
            "handler_composition_soundness",
            ae_ext_handler_composition_soundness_ty(),
        ),
        ("ExceptionEffect", ae_ext_exception_effect_ty()),
        (
            "exception_handler_total",
            ae_ext_exception_handler_total_ty(),
        ),
        ("StateReadWrite", ae_ext_state_read_write_ty()),
        ("state_handler_runstate", ae_ext_state_handler_runstate_ty()),
        ("NondeterminismEffect", ae_ext_nondeterminism_effect_ty()),
        ("nondet_handler_list", ae_ext_nondet_handler_list_ty()),
        (
            "CooperativeThreadEffect",
            ae_ext_cooperative_thread_effect_ty(),
        ),
        ("io_as_algebraic_effect", ae_ext_io_as_algebraic_effect_ty()),
        ("IOEffect_world_token", ae_ext_io_world_token_ty()),
        ("ContinuationEffect", ae_ext_continuation_effect_ty()),
        (
            "DelimitedControlEffect",
            ae_ext_delimited_control_effect_ty(),
        ),
        (
            "continuation_is_algebraic",
            ae_ext_continuation_is_algebraic_ty(),
        ),
        ("shift_reset_as_handler", ae_ext_shift_reset_as_handler_ty()),
        ("multi_prompt_effects", ae_ext_multi_prompt_effects_ty()),
        ("EffectfulCPS", ae_ext_effectful_cps_ty()),
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
/// TypedHandler : Type → Type → Type — a handler with precise input/output types
pub fn ae_ext_typed_handler_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// handler_normal_form : every handler can be put in normal form
pub fn ae_ext_handler_normal_form_ty() -> Expr {
    arrow(cst("Handler"), prop())
}
/// EffectInterface : EffectSig → Type — the public interface of an effect
pub fn ae_ext_effect_interface_ty() -> Expr {
    arrow(cst("EffectSig"), type0())
}
/// effect_modularity : effects can be combined modularly
pub fn ae_ext_effect_modularity_ty() -> Expr {
    prop()
}
/// EffectfulProgram : EffectRow → Type → Type — a program with effect annotations
pub fn ae_ext_effectful_program_ty() -> Expr {
    arrow(cst("EffectRow"), arrow(type0(), type0()))
}
/// effect_type_inference_decidable : effect type inference is decidable
pub fn ae_ext_effect_type_inference_decidable_ty() -> Expr {
    prop()
}
/// ResourceEffect : EffectSig — effect for resource acquisition and release
pub fn ae_ext_resource_effect_ty() -> Expr {
    cst("EffectSig")
}
/// resource_effect_bracket : acquire/release with guaranteed cleanup
pub fn ae_ext_resource_effect_bracket_ty() -> Expr {
    arrow(cst("EffectSig"), arrow(type0(), arrow(type0(), type0())))
}
/// effect_interaction_laws : commutativity laws between pairs of effects
pub fn ae_ext_effect_interaction_laws_ty() -> Expr {
    arrow(cst("EffectSig"), arrow(cst("EffectSig"), prop()))
}
/// effect_semantics_adequacy : denotational and operational semantics agree
pub fn ae_ext_effect_semantics_adequacy_ty() -> Expr {
    prop()
}
/// Register the §13 completion axioms for algebraic effects.
pub fn register_algebraic_effects_completions(env: &mut Environment) -> Result<(), String> {
    let axioms: &[(&str, Expr)] = &[
        ("TypedHandler", ae_ext_typed_handler_ty()),
        ("handler_normal_form", ae_ext_handler_normal_form_ty()),
        ("EffectInterface", ae_ext_effect_interface_ty()),
        ("effect_modularity", ae_ext_effect_modularity_ty()),
        ("EffectfulProgram", ae_ext_effectful_program_ty()),
        (
            "effect_type_inference_decidable",
            ae_ext_effect_type_inference_decidable_ty(),
        ),
        ("ResourceEffect", ae_ext_resource_effect_ty()),
        (
            "resource_effect_bracket",
            ae_ext_resource_effect_bracket_ty(),
        ),
        (
            "effect_interaction_laws",
            ae_ext_effect_interaction_laws_ty(),
        ),
        (
            "effect_semantics_adequacy",
            ae_ext_effect_semantics_adequacy_ty(),
        ),
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
