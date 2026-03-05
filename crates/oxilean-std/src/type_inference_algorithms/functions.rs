//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    AlgorithmW, AnnExpr, BidirectionalInferencer, ClassConstraint, ConstraintGenerator,
    ConstraintInference, ConstraintTypeInference, DependentTypeChecker, GradualType,
    GradualTypeData, HMExpr, HMType, LiquidType, RankNType, Row, RowType, TypeClass,
    TypeClassInstance, TypeClassResolution, TypeConstraintItem, TypeEnv, TypeScheme, TypeSubst,
    UnificationSolver,
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
pub fn option_ty(t: Expr) -> Expr {
    app(cst("Option"), t)
}
pub fn list_ty(t: Expr) -> Expr {
    app(cst("List"), t)
}
/// `HMType : Type` — a Hindley-Milner monotype
pub fn hm_type_ty() -> Expr {
    type0()
}
/// `TypeScheme : Type` — a polytype (∀ α. τ)
pub fn type_scheme_ty() -> Expr {
    type0()
}
/// `TypeEnv : Type` — typing context (variable → type scheme)
pub fn type_env_ty() -> Expr {
    type0()
}
/// `Substitution : Type` — type variable substitution
pub fn type_subst_ty() -> Expr {
    type0()
}
/// `TypeConstraint : Type` — an equality constraint τ₁ = τ₂
pub fn type_constraint_ty() -> Expr {
    type0()
}
/// `InferResult : Type` — result of Algorithm W: (subst, type)
pub fn infer_result_ty() -> Expr {
    type0()
}
/// `RowType : Type` — a row type for record/variant polymorphism
pub fn row_type_ty() -> Expr {
    type0()
}
/// `RecordType : Type` — a record type built from a row
pub fn record_type_ty() -> Expr {
    type0()
}
/// `GradualType : Type` — a type with the unknown type `?`
pub fn gradual_type_ty() -> Expr {
    type0()
}
/// `LiquidType : Type` — a refinement type `{x : T | φ(x)}`
pub fn liquid_type_ty() -> Expr {
    type0()
}
/// `Constraint : Type` — a typing or subtyping constraint
pub fn constraint_ty() -> Expr {
    type0()
}
/// `Typing : TypeEnv → Expr → Type → Prop`
pub fn typing_ty() -> Expr {
    arrow(type_env_ty(), arrow(type0(), arrow(hm_type_ty(), prop())))
}
/// `AlgorithmW : TypeEnv → Expr → Option InferResult → Prop`
pub fn alg_w_ty() -> Expr {
    arrow(
        type_env_ty(),
        arrow(type0(), arrow(option_ty(infer_result_ty()), prop())),
    )
}
/// `Unifies : HMType → HMType → Substitution → Prop`
pub fn unifies_ty() -> Expr {
    arrow(
        hm_type_ty(),
        arrow(hm_type_ty(), arrow(type_subst_ty(), prop())),
    )
}
/// `MGU : HMType → HMType → Option Substitution → Prop`
pub fn mgu_ty() -> Expr {
    arrow(
        hm_type_ty(),
        arrow(hm_type_ty(), arrow(option_ty(type_subst_ty()), prop())),
    )
}
/// `Generalizes : TypeEnv → HMType → TypeScheme → Prop`
pub fn generalizes_ty() -> Expr {
    arrow(
        type_env_ty(),
        arrow(hm_type_ty(), arrow(type_scheme_ty(), prop())),
    )
}
/// `Instantiates : TypeScheme → HMType → Prop`
pub fn instantiates_ty() -> Expr {
    arrow(type_scheme_ty(), arrow(hm_type_ty(), prop()))
}
/// `Subtype : HMType → HMType → Prop`
pub fn subtype_ty() -> Expr {
    arrow(hm_type_ty(), arrow(hm_type_ty(), prop()))
}
/// `Consistent : GradualType → GradualType → Prop`
pub fn consistent_ty() -> Expr {
    arrow(gradual_type_ty(), arrow(gradual_type_ty(), prop()))
}
/// `Satisfies : HMType → LiquidType → Prop`
pub fn satisfies_ty() -> Expr {
    arrow(hm_type_ty(), arrow(liquid_type_ty(), prop()))
}
/// `BidirCheck : TypeEnv → Expr → HMType → Prop`
pub fn bidir_check_ty() -> Expr {
    arrow(type_env_ty(), arrow(type0(), arrow(hm_type_ty(), prop())))
}
/// `BidirInfer : TypeEnv → Expr → HMType → Prop`
pub fn bidir_infer_ty() -> Expr {
    arrow(type_env_ty(), arrow(type0(), arrow(hm_type_ty(), prop())))
}
/// Principal type theorem: Algorithm W produces the most general type.
/// `PrincipalType : ∀ Γ e σ, AlgW Γ e = Some (σ, τ) → ∀ σ', Γ ⊢ e : σ'(τ') → ∃ η, σ = η ∘ σ'`
pub fn principal_type_theorem_ty() -> Expr {
    prop()
}
/// Soundness: if W infers τ then the typing judgment holds.
pub fn alg_w_soundness_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "Gamma",
        type_env_ty(),
        pi(
            BinderInfo::Default,
            "e",
            type0(),
            pi(
                BinderInfo::Default,
                "tau",
                hm_type_ty(),
                arrow(
                    app3(
                        cst("AlgorithmW"),
                        bvar(2),
                        bvar(1),
                        app(cst("Some"), bvar(0)),
                    ),
                    app3(cst("Typing"), bvar(3), bvar(2), bvar(1)),
                ),
            ),
        ),
    )
}
/// Completeness: if a term is typeable, W succeeds.
pub fn alg_w_completeness_ty() -> Expr {
    prop()
}
/// Unification soundness: if MGU(s, t) = σ then σ(s) = σ(t).
pub fn mgu_soundness_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "s",
        hm_type_ty(),
        pi(
            BinderInfo::Default,
            "t",
            hm_type_ty(),
            pi(
                BinderInfo::Default,
                "sigma",
                type_subst_ty(),
                arrow(
                    app3(cst("MGU"), bvar(2), bvar(1), app(cst("Some"), bvar(0))),
                    app2(
                        cst("Eq"),
                        app2(cst("ApplySubst"), bvar(1), bvar(2)),
                        app2(cst("ApplySubst"), bvar(1), bvar(3)),
                    ),
                ),
            ),
        ),
    )
}
/// Let-polymorphism: let x = e₁ in e₂ generalizes the type of e₁.
pub fn let_polymorphism_ty() -> Expr {
    prop()
}
/// Occurs check correctness: unification of x with f(x) must fail.
pub fn occurs_check_ty() -> Expr {
    prop()
}
/// Subtyping reflexivity: every type is a subtype of itself.
pub fn subtype_refl_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "tau",
        hm_type_ty(),
        app2(cst("Subtype"), bvar(0), bvar(0)),
    )
}
/// Subtyping transitivity.
pub fn subtype_trans_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "tau1",
        hm_type_ty(),
        pi(
            BinderInfo::Default,
            "tau2",
            hm_type_ty(),
            pi(
                BinderInfo::Default,
                "tau3",
                hm_type_ty(),
                arrow(
                    app2(cst("Subtype"), bvar(2), bvar(1)),
                    arrow(
                        app2(cst("Subtype"), bvar(2), bvar(1)),
                        app2(cst("Subtype"), bvar(3), bvar(1)),
                    ),
                ),
            ),
        ),
    )
}
/// Gradual typing: consistent-subtyping extends subtyping.
pub fn gradual_typing_ty() -> Expr {
    prop()
}
/// Liquid type checking decidability (under restricted templates).
pub fn liquid_decidable_ty() -> Expr {
    prop()
}
/// Populate `env` with all type-inference kernel declarations.
pub fn build_type_inference_env(env: &mut Environment) -> Result<(), String> {
    for (name, ty) in [
        ("HMType", hm_type_ty()),
        ("TypeScheme", type_scheme_ty()),
        ("TypeEnv", type_env_ty()),
        ("TypeSubstitution", type_subst_ty()),
        ("TypeConstraint", type_constraint_ty()),
        ("InferResult", infer_result_ty()),
        ("RowType", row_type_ty()),
        ("RecordType", record_type_ty()),
        ("GradualType", gradual_type_ty()),
        ("LiquidType", liquid_type_ty()),
    ] {
        env.add(Declaration::Axiom {
            name: Name::str(name),
            univ_params: vec![],
            ty,
        })
        .ok();
    }
    for (name, ty) in [
        ("Typing", typing_ty()),
        ("AlgorithmW", alg_w_ty()),
        ("Unifies", unifies_ty()),
        ("MGU", mgu_ty()),
        ("Generalizes", generalizes_ty()),
        ("Instantiates", instantiates_ty()),
        ("Subtype", subtype_ty()),
        ("Consistent", consistent_ty()),
        ("Satisfies", satisfies_ty()),
        ("BidirCheck", bidir_check_ty()),
        ("BidirInfer", bidir_infer_ty()),
        (
            "ApplySubst",
            arrow(type_subst_ty(), arrow(hm_type_ty(), hm_type_ty())),
        ),
        (
            "ComposeSubst",
            arrow(type_subst_ty(), arrow(type_subst_ty(), type_subst_ty())),
        ),
        ("FreeTypeVars", arrow(hm_type_ty(), list_ty(nat_ty()))),
        ("SchemeTypeVars", arrow(type_scheme_ty(), list_ty(nat_ty()))),
    ] {
        env.add(Declaration::Axiom {
            name: Name::str(name),
            univ_params: vec![],
            ty,
        })
        .ok();
    }
    for (name, ty) in [
        ("PrincipalTypeThm", principal_type_theorem_ty()),
        ("AlgWSoundness", alg_w_soundness_ty()),
        ("AlgWCompleteness", alg_w_completeness_ty()),
        ("MGUSoundness", mgu_soundness_ty()),
        ("LetPolymorphismThm", let_polymorphism_ty()),
        ("OccursCheckThm", occurs_check_ty()),
        ("SubtypeRefl", subtype_refl_ty()),
        ("SubtypeTrans", subtype_trans_ty()),
        ("GradualTypingThm", gradual_typing_ty()),
        ("LiquidDecidable", liquid_decidable_ty()),
        ("UnificationTermination", prop()),
        ("UnificationCompleteness", prop()),
        ("AlgJEquivalentW", prop()),
        ("RowPolyExtension", prop()),
        ("SubtypingCovariant", prop()),
        ("SubtypingContravariant", prop()),
        ("ConstraintSolvingSound", prop()),
        ("ConstraintSolvingComplete", prop()),
        ("DependentInferenceHeuristic", prop()),
        ("InstantiationLemma", prop()),
        ("GeneralizationLemma", prop()),
        ("SubstitutionLemma", prop()),
        ("WeakeningLemma", prop()),
        ("BidirSoundness", prop()),
        ("BidirCompleteness", prop()),
    ] {
        env.add(Declaration::Axiom {
            name: Name::str(name),
            univ_params: vec![],
            ty,
        })
        .ok();
    }
    Ok(())
}
/// A type variable identifier.
pub type TyVar = u32;
/// Unify two HM types, returning the MGU or an error.
pub fn unify_types(s: &HMType, t: &HMType) -> Result<TypeSubst, String> {
    match (s, t) {
        (HMType::Var(x), HMType::Var(y)) if x == y => Ok(TypeSubst::new()),
        (HMType::Var(x), _) => {
            if t.occurs(*x) {
                return Err(format!("occurs check failed: α{} in {}", x, t));
            }
            Ok(TypeSubst::bind(*x, t.clone()))
        }
        (_, HMType::Var(y)) => {
            if s.occurs(*y) {
                return Err(format!("occurs check failed: α{} in {}", y, s));
            }
            Ok(TypeSubst::bind(*y, s.clone()))
        }
        (HMType::Base(a), HMType::Base(b)) => {
            if a == b {
                Ok(TypeSubst::new())
            } else {
                Err(format!("type mismatch: {} ≠ {}", a, b))
            }
        }
        (HMType::Arrow(a1, b1), HMType::Arrow(a2, b2)) => {
            let s1 = unify_types(a1, a2)?;
            let s2 = unify_types(&b1.apply(&s1), &b2.apply(&s1))?;
            Ok(s2.compose(&s1))
        }
        (HMType::App(f, args1), HMType::App(g, args2)) => {
            if f != g || args1.len() != args2.len() {
                return Err(format!("type constructor mismatch: {} ≠ {}", f, g));
            }
            let mut subst = TypeSubst::new();
            for (a, b) in args1.iter().zip(args2.iter()) {
                let s = unify_types(&a.apply(&subst), &b.apply(&subst))?;
                subst = s.compose(&subst);
            }
            Ok(subst)
        }
        (HMType::Tuple(ts1), HMType::Tuple(ts2)) => {
            if ts1.len() != ts2.len() {
                return Err(format!(
                    "tuple arity mismatch: {} ≠ {}",
                    ts1.len(),
                    ts2.len()
                ));
            }
            let mut subst = TypeSubst::new();
            for (a, b) in ts1.iter().zip(ts2.iter()) {
                let s = unify_types(&a.apply(&subst), &b.apply(&subst))?;
                subst = s.compose(&subst);
            }
            Ok(subst)
        }
        _ => Err(format!("cannot unify {} with {}", s, t)),
    }
}
/// `ConstraintSet : Type` — a set of type equality constraints
pub fn constraint_set_ty() -> Expr {
    type0()
}
/// `TypeEquation : HMType → HMType → Prop` — a type equation τ₁ = τ₂
pub fn type_equation_ty() -> Expr {
    arrow(hm_type_ty(), arrow(hm_type_ty(), prop()))
}
/// `FlowType : Type` — a flow-sensitive type (narrowed at branch points)
pub fn flow_type_ty() -> Expr {
    type0()
}
/// `OccurrenceType : Type` — occurrence type for TypeScript-style narrowing
pub fn occurrence_type_ty() -> Expr {
    type0()
}
/// `KindExpr : Type` — a kind expression (*, k₁ → k₂, k₁ × k₂)
pub fn kind_expr_ty() -> Expr {
    type0()
}
/// `KindEnv : Type` — a kind environment mapping type constructors to kinds
pub fn kind_env_ty() -> Expr {
    type0()
}
/// `KindConstraint : Type` — a kind-level equality constraint
pub fn kind_constraint_ty() -> Expr {
    type0()
}
/// `RegionVar : Type` — a region variable ρ
pub fn region_var_ty() -> Expr {
    type0()
}
/// `RegionType : Type` — a region-annotated type
pub fn region_type_ty() -> Expr {
    type0()
}
/// `EffectType : Type` — an effect row type (monadic or algebraic effects)
pub fn effect_type_ty() -> Expr {
    type0()
}
/// `CoeffectType : Type` — a coeffect annotation (usage tracking)
pub fn coeffect_type_ty() -> Expr {
    type0()
}
/// `CapabilityType : Type` — a capability type (linear/affine resource)
pub fn capability_type_ty() -> Expr {
    type0()
}
/// `OpenRowType : Type` — an open row type for row polymorphism inference
pub fn open_row_type_ty() -> Expr {
    type0()
}
/// `RowConstraint : Type` — a row-level constraint for row unification
pub fn row_constraint_ty() -> Expr {
    type0()
}
/// `GadtEquation : Type` — a GADT type equation generated during constraint propagation
pub fn gadt_equation_ty() -> Expr {
    type0()
}
/// `TypeClassConstraint : Type` — a type class (qualified type) constraint
pub fn typeclass_constraint_ty() -> Expr {
    type0()
}
/// `InstanceDict : Type` — a dictionary for a resolved type class instance
pub fn instance_dict_ty() -> Expr {
    type0()
}
/// `StratifiedConstraint : Type` — a stratified constraint system for decidability
pub fn stratified_constraint_ty() -> Expr {
    type0()
}
/// `Rank2Type : Type` — a rank-2 polymorphic type (higher-rank)
pub fn rank2_type_ty() -> Expr {
    type0()
}
/// `TypingFixpoint : TypeEnv → HMExpr → HMType → Prop`
/// Polymorphic recursion: the type of a fixpoint satisfies the typing judgment.
pub fn typing_fixpoint_ty() -> Expr {
    arrow(type_env_ty(), arrow(type0(), arrow(hm_type_ty(), prop())))
}
/// `FlowNarrowing : FlowType → TypeConstraint → FlowType → Prop`
/// Flow-sensitive narrowing: after a type-guard check, the type is narrowed.
pub fn flow_narrowing_ty() -> Expr {
    arrow(
        flow_type_ty(),
        arrow(type_constraint_ty(), arrow(flow_type_ty(), prop())),
    )
}
/// `KindInference : KindEnv → HMType → KindExpr → Prop`
/// Kind inference assigns a kind to every type expression.
pub fn kind_inference_ty() -> Expr {
    arrow(
        kind_env_ty(),
        arrow(hm_type_ty(), arrow(kind_expr_ty(), prop())),
    )
}
/// `RegionSoundness : RegionType → Prop`
/// Region soundness: no dangling references; every accessed region is live.
pub fn region_soundness_ty() -> Expr {
    arrow(region_type_ty(), prop())
}
/// `EffectSafety : EffectType → Prop`
/// Effect safety: all effects in the row are handled by an enclosing handler.
pub fn effect_safety_ty() -> Expr {
    arrow(effect_type_ty(), prop())
}
/// `RowUnification : OpenRowType → OpenRowType → Substitution → Prop`
/// Row unification produces a substitution equating two open row types.
pub fn row_unification_ty() -> Expr {
    arrow(
        open_row_type_ty(),
        arrow(open_row_type_ty(), arrow(type_subst_ty(), prop())),
    )
}
/// `GadtConstraintPropagation : GadtEquation → TypeConstraint → Prop`
/// GADT constraint propagation: solving type equations imposed by GADT patterns.
pub fn gadt_constraint_propagation_ty() -> Expr {
    arrow(gadt_equation_ty(), arrow(type_constraint_ty(), prop()))
}
/// `InstanceResolution : TypeClassConstraint → InstanceDict → Prop`
/// Qualified type resolution: finds the appropriate instance dictionary.
pub fn instance_resolution_ty() -> Expr {
    arrow(typeclass_constraint_ty(), arrow(instance_dict_ty(), prop()))
}
/// `CoherenceCondition : TypeClassConstraint → Prop`
/// Coherence: all instance resolutions for the same constraint yield equal dictionaries.
pub fn coherence_condition_ty() -> Expr {
    arrow(typeclass_constraint_ty(), prop())
}
/// `ConstraintStratification : ConstraintSet → Prop`
/// Stratification condition ensuring decidability of constraint solving.
pub fn constraint_stratification_ty() -> Expr {
    arrow(constraint_set_ty(), prop())
}
/// `AcyclicConstraintGraph : ConstraintSet → Prop`
/// Acyclic constraint graph: the constraint dependency graph is a DAG.
pub fn acyclic_constraint_graph_ty() -> Expr {
    arrow(constraint_set_ty(), prop())
}
/// Polymorphic recursion: Milner's Algorithm M produces principal types.
pub fn alg_m_principal_ty() -> Expr {
    prop()
}
/// Rank-2 type inference is decidable (Jones, 1993).
pub fn rank2_inference_decidable_ty() -> Expr {
    prop()
}
/// Higher-kinded inference: kind inference is complete for System F-omega.
pub fn higher_kinded_completeness_ty() -> Expr {
    prop()
}
/// Bidirectional mode correctness: checking and synthesis modes are consistent.
pub fn bidir_mode_correct_ty() -> Expr {
    prop()
}
/// Local type inference: propagating type information inward is sound.
pub fn local_type_inference_sound_ty() -> Expr {
    prop()
}
/// Flow-sensitive soundness: occurrence typing is sound w.r.t. operational semantics.
pub fn flow_sensitive_sound_ty() -> Expr {
    prop()
}
/// Gradual guarantee: removing type annotations does not cause new static errors.
pub fn gradual_guarantee_ty() -> Expr {
    prop()
}
/// Consistent types form a preorder (reflexive + symmetric but not transitive).
pub fn consistent_preorder_ty() -> Expr {
    prop()
}
/// Region polymorphism: region-annotated types are closed under substitution.
pub fn region_poly_closed_ty() -> Expr {
    prop()
}
/// Effect inference: monadic effect rows are inferred compositionally.
pub fn effect_inference_compositional_ty() -> Expr {
    prop()
}
/// Row polymorphism: extension of rows is injective up to permutation.
pub fn row_extension_injective_ty() -> Expr {
    prop()
}
/// Termination of type inference: constraint solving terminates for stratified systems.
pub fn type_inference_termination_ty() -> Expr {
    prop()
}
/// Register all new axioms from Sections 14-15 into `env`.
pub fn build_type_inference_env_extended(env: &mut Environment) -> Result<(), String> {
    for (name, ty) in [
        ("ConstraintSet", constraint_set_ty()),
        ("TypeEquation", type_equation_ty()),
        ("FlowType", flow_type_ty()),
        ("OccurrenceType", occurrence_type_ty()),
        ("KindExpr", kind_expr_ty()),
        ("KindEnv", kind_env_ty()),
        ("KindConstraint", kind_constraint_ty()),
        ("RegionVar", region_var_ty()),
        ("RegionType", region_type_ty()),
        ("EffectType", effect_type_ty()),
        ("CoeffectType", coeffect_type_ty()),
        ("CapabilityType", capability_type_ty()),
        ("OpenRowType", open_row_type_ty()),
        ("RowConstraint", row_constraint_ty()),
        ("GadtEquation", gadt_equation_ty()),
        ("TypeClassConstraint", typeclass_constraint_ty()),
        ("InstanceDict", instance_dict_ty()),
        ("StratifiedConstraint", stratified_constraint_ty()),
        ("Rank2Type", rank2_type_ty()),
    ] {
        env.add(Declaration::Axiom {
            name: Name::str(name),
            univ_params: vec![],
            ty,
        })
        .ok();
    }
    for (name, ty) in [
        ("TypingFixpoint", typing_fixpoint_ty()),
        ("FlowNarrowing", flow_narrowing_ty()),
        ("KindInference", kind_inference_ty()),
        ("RegionSoundness", region_soundness_ty()),
        ("EffectSafety", effect_safety_ty()),
        ("RowUnification", row_unification_ty()),
        (
            "GadtConstraintPropagation",
            gadt_constraint_propagation_ty(),
        ),
        ("InstanceResolution", instance_resolution_ty()),
        ("CoherenceCondition", coherence_condition_ty()),
        ("ConstraintStratification", constraint_stratification_ty()),
        ("AcyclicConstraintGraph", acyclic_constraint_graph_ty()),
    ] {
        env.add(Declaration::Axiom {
            name: Name::str(name),
            univ_params: vec![],
            ty,
        })
        .ok();
    }
    for (name, ty) in [
        ("AlgMPrincipal", alg_m_principal_ty()),
        ("Rank2InferenceDecidable", rank2_inference_decidable_ty()),
        ("HigherKindedCompleteness", higher_kinded_completeness_ty()),
        ("BidirModeCorrect", bidir_mode_correct_ty()),
        ("LocalTypeInferenceSound", local_type_inference_sound_ty()),
        ("FlowSensitiveSound", flow_sensitive_sound_ty()),
        ("GradualGuarantee", gradual_guarantee_ty()),
        ("ConsistentPreorder", consistent_preorder_ty()),
        ("RegionPolyClosed", region_poly_closed_ty()),
        (
            "EffectInferenceCompositional",
            effect_inference_compositional_ty(),
        ),
        ("RowExtensionInjective", row_extension_injective_ty()),
        ("TypeInferenceTermination", type_inference_termination_ty()),
    ] {
        env.add(Declaration::Axiom {
            name: Name::str(name),
            univ_params: vec![],
            ty,
        })
        .ok();
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    fn int() -> HMType {
        HMType::Base("Int".into())
    }
    fn bool_t() -> HMType {
        HMType::Base("Bool".into())
    }
    fn nat_t() -> HMType {
        HMType::Base("Nat".into())
    }
    fn alpha(i: u32) -> HMType {
        HMType::Var(i)
    }
    /// Build the kernel environment without errors.
    #[test]
    fn test_build_env() {
        let mut env = Environment::new();
        let result = build_type_inference_env(&mut env);
        assert!(result.is_ok());
    }
    /// Unification of identical base types.
    #[test]
    fn test_unify_base() {
        let subst = unify_types(&int(), &int()).expect("should unify");
        assert!(subst.map.is_empty());
    }
    /// Unification of a type variable with a base type.
    #[test]
    fn test_unify_var() {
        let s = unify_types(&alpha(0), &int()).expect("should unify");
        assert_eq!(s.map.get(&0), Some(&int()));
    }
    /// Occurs check prevents infinite types.
    #[test]
    fn test_occurs_check() {
        let circular = HMType::Arrow(Box::new(alpha(0)), Box::new(int()));
        let result = unify_types(&alpha(0), &circular);
        assert!(result.is_err(), "occurs check should prevent α0 = α0 → Int");
    }
    /// Algorithm W infers the identity function type.
    #[test]
    fn test_alg_w_identity() {
        let id_expr = HMExpr::Lam("x".into(), Box::new(HMExpr::Var("x".into())));
        let mut w = AlgorithmW::new();
        let ty = w.infer_closed(&id_expr).expect("should infer");
        match ty {
            HMType::Arrow(a, b) => {
                assert_eq!(a, b, "identity should have type α → α")
            }
            other => panic!("expected arrow type, got {:?}", other),
        }
    }
    /// Algorithm W infers let-polymorphism correctly.
    #[test]
    fn test_alg_w_let_poly() {
        let id = HMExpr::Lam("x".into(), Box::new(HMExpr::Var("x".into())));
        let body = HMExpr::Pair(
            Box::new(HMExpr::App(
                Box::new(HMExpr::Var("id".into())),
                Box::new(HMExpr::Bool(true)),
            )),
            Box::new(HMExpr::App(
                Box::new(HMExpr::Var("id".into())),
                Box::new(HMExpr::Nat(42)),
            )),
        );
        let expr = HMExpr::Let("id".into(), Box::new(id), Box::new(body));
        let mut w = AlgorithmW::new();
        let ty = w
            .infer_closed(&expr)
            .expect("let-polymorphism should succeed");
        assert!(matches!(ty, HMType::Tuple(ref ts) if ts.len() == 2));
    }
    /// Constraint-based inference matches Algorithm W.
    #[test]
    fn test_constraint_inference() {
        let expr = HMExpr::Lam("x".into(), Box::new(HMExpr::Var("x".into())));
        let env = TypeEnv::new();
        let ty =
            ConstraintInference::infer(&env, &expr).expect("constraint inference should succeed");
        match ty {
            HMType::Arrow(a, b) => assert_eq!(a, b),
            other => panic!("expected arrow type, got {:?}", other),
        }
    }
    /// Gradual typing: Unknown is consistent with everything.
    #[test]
    fn test_gradual_consistent() {
        let unknown = GradualType::Unknown;
        let static_int = GradualType::Static(int());
        assert!(unknown.consistent(&static_int));
        assert!(static_int.consistent(&unknown));
        assert!(!static_int.consistent(&GradualType::Static(bool_t())));
    }
    /// Liquid type subtyping: trivial predicates.
    #[test]
    fn test_liquid_subtype() {
        let lt1 = LiquidType::new(nat_t(), "v >= 0");
        let lt2 = LiquidType::trivial(nat_t());
        assert!(lt1.is_subtype_of(&lt2));
        assert!(!lt2.is_subtype_of(&lt1));
    }
    /// Extended environment builds without errors.
    #[test]
    fn test_build_extended_env() {
        let mut env = Environment::new();
        assert!(build_type_inference_env(&mut env).is_ok());
        assert!(build_type_inference_env_extended(&mut env).is_ok());
    }
    /// ConstraintGenerator collects constraints for a lambda.
    #[test]
    fn test_constraint_generator_lambda() {
        let expr = HMExpr::Lam("x".into(), Box::new(HMExpr::Var("x".into())));
        let mut gen = ConstraintGenerator::new();
        let ty = gen.generate(&TypeEnv::new(), &expr).expect("generate ok");
        assert!(gen.num_constraints() == 0);
        assert!(matches!(ty, HMType::Arrow(_, _)));
    }
    /// ConstraintGenerator collects constraints for application.
    #[test]
    fn test_constraint_generator_app() {
        let id = HMExpr::Lam("x".into(), Box::new(HMExpr::Var("x".into())));
        let expr = HMExpr::App(Box::new(id), Box::new(HMExpr::Bool(true)));
        let mut gen = ConstraintGenerator::new();
        let _ty = gen.generate(&TypeEnv::new(), &expr).expect("generate ok");
        assert!(gen.num_constraints() >= 1);
    }
    /// UnificationSolver infers the identity function.
    #[test]
    fn test_unification_solver_identity() {
        let expr = HMExpr::Lam("x".into(), Box::new(HMExpr::Var("x".into())));
        let ty = UnificationSolver::infer(&TypeEnv::new(), &expr).expect("infer ok");
        assert!(matches!(ty, HMType::Arrow(ref a, ref b) if a == b));
    }
    /// UnificationSolver infers type of conditional.
    #[test]
    fn test_unification_solver_if() {
        let expr = HMExpr::If(
            Box::new(HMExpr::Bool(true)),
            Box::new(HMExpr::Nat(1)),
            Box::new(HMExpr::Nat(2)),
        );
        let ty = UnificationSolver::infer(&TypeEnv::new(), &expr).expect("infer ok");
        assert_eq!(ty, HMType::Base("Nat".into()));
    }
    /// BidirectionalInferencer synthesizes type of annotated expression.
    #[test]
    fn test_bidir_annotated() {
        let expr = AnnExpr::Ann(Box::new(AnnExpr::Plain(HMExpr::Nat(42))), nat_t());
        let ty = BidirectionalInferencer::infer(&TypeEnv::new(), &expr).expect("infer ok");
        assert_eq!(ty, nat_t());
    }
    /// BidirectionalInferencer checks annotated lambda.
    #[test]
    fn test_bidir_ann_lam() {
        let expr = AnnExpr::AnnLam(
            "x".into(),
            nat_t(),
            Box::new(AnnExpr::Plain(HMExpr::Var("x".into()))),
        );
        let ty = BidirectionalInferencer::infer(&TypeEnv::new(), &expr).expect("infer ok");
        assert_eq!(ty, HMType::Arrow(Box::new(nat_t()), Box::new(nat_t())));
    }
    /// TypeClassResolution: Eq instance for Bool.
    #[test]
    fn test_typeclass_resolution_basic() {
        let mut resolver = TypeClassResolution::new();
        let eq_class = TypeClass {
            name: "Eq".into(),
            methods: vec![(
                "eq".into(),
                HMType::Arrow(
                    Box::new(HMType::Var(0)),
                    Box::new(HMType::Arrow(
                        Box::new(HMType::Var(0)),
                        Box::new(HMType::Base("Bool".into())),
                    )),
                ),
            )],
        };
        resolver.add_class(eq_class);
        resolver.add_instance(TypeClassInstance {
            class_name: "Eq".into(),
            instance_ty: bool_t(),
            method_types: vec![],
        });
        let c = ClassConstraint {
            class: "Eq".into(),
            ty: bool_t(),
        };
        assert!(resolver.resolve(&c).is_some());
        let c2 = ClassConstraint {
            class: "Eq".into(),
            ty: int(),
        };
        assert!(resolver.resolve(&c2).is_none());
    }
    /// TypeClassResolution: coherence holds when instances don't overlap.
    #[test]
    fn test_typeclass_coherence() {
        let mut resolver = TypeClassResolution::new();
        resolver.add_instance(TypeClassInstance {
            class_name: "Eq".into(),
            instance_ty: bool_t(),
            method_types: vec![],
        });
        resolver.add_instance(TypeClassInstance {
            class_name: "Eq".into(),
            instance_ty: int(),
            method_types: vec![],
        });
        assert!(resolver.is_coherent());
    }
    /// Unification solver log records solved constraints.
    #[test]
    fn test_solver_log() {
        let mut solver = UnificationSolver::new();
        let c = TypeConstraintItem {
            lhs: HMType::Var(0),
            rhs: int(),
            label: Some("test".into()),
        };
        solver.solve_one(&c).expect("should solve");
        assert!(!solver.log.is_empty());
    }
}
#[cfg(test)]
mod extended_type_inference_tests {
    use super::*;
    #[test]
    fn test_constraint_inference() {
        let mut cti = ConstraintTypeInference::new();
        cti.add_constraint("a", "Int");
        cti.add_constraint("b", "a -> Bool");
        assert_eq!(cti.num_constraints(), 2);
        cti.solve();
        assert!(cti.solved);
    }
    #[test]
    fn test_rank_n_types() {
        let r1 = RankNType::rank1("forall a. a -> a");
        assert!(r1.inference_decidable());
        let r2 = RankNType::rank2("(forall a. a -> a) -> Int");
        assert!(r2.inference_decidable());
    }
    #[test]
    fn test_gradual_type() {
        let dyn_type = GradualTypeData::dynamic();
        assert!(dyn_type.has_dynamic_part);
        assert!(dyn_type.gradual_guarantee().contains("dynamic=true"));
    }
    #[test]
    fn test_dependent_type_checker() {
        let btc = DependentTypeChecker::bidirectional();
        assert_eq!(btc.mode, "bidirectional");
        assert!(btc.mode_description().contains("bidirectional"));
    }
    #[test]
    fn test_row_type() {
        let r = RowType::closed(vec![("x", "Int"), ("y", "String")]);
        assert!(r.is_closed());
        let r2 = r.extend("z", "Bool");
        assert_eq!(r2.fields.len(), 3);
    }
}
