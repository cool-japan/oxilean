//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::context::ElabContext;

use super::types::{
    ClassConstraint, ClassError, CoherenceChecker, CoherenceError, CoherenceViolation,
    DefaultMethodFill, DefaultMethodRegistry, DeriveBuilder, DeriveRule, Instance, InstanceCache,
    InstanceCacheEntry, InstanceCacheKey, InstanceContextStack, InstanceResolutionOutcome,
    InstanceSearchBudget, InstanceSearchResult, InstanceSet, Method, SuperclassEdge, SynthStats,
    TypeClass, TypeClassHierarchy, TypeClassQuery, TypeClassRegistry, TypeclassDep,
    TypeclassDepGraph,
};
use oxilean_kernel::{Expr, Level, Name};

/// Extract the head constant of an expression (spine traversal).
///
/// For `App(App(f, a), b)` returns `f`; for `Const(n, _)` returns the expr
/// itself.  Returns `None` for binders, sorts, and other non-head-normal forms.
fn expr_head(e: &Expr) -> Option<&Expr> {
    match e {
        Expr::App(f, _) => expr_head(f),
        Expr::Const(_, _) => Some(e),
        _ => None,
    }
}
/// Collect the arguments of a spine: `App(App(f, a), b)` → `[a, b]`.
fn expr_args(e: &Expr) -> Vec<&Expr> {
    let mut args = Vec::new();
    let mut cur = e;
    while let Expr::App(f, a) = cur {
        args.push(a.as_ref());
        cur = f;
    }
    args.reverse();
    args
}
/// Check whether `candidate` matches `query` for instance selection.
///
/// Uses structural head-matching: two expressions match when their head
/// constants are equal and every non-`BVar` argument matches recursively.
/// `BVar` in the *candidate* acts as a wildcard (the instance parameter).
pub fn type_matches(candidate: &Expr, query: &Expr) -> bool {
    match (candidate, query) {
        _ if candidate == query => true,
        (Expr::BVar(_), _) => true,
        (Expr::Const(n1, _), Expr::Const(n2, _)) => n1 == n2,
        (Expr::App(f1, a1), Expr::App(f2, a2)) => type_matches(f1, f2) && type_matches(a1, a2),
        _ => false,
    }
}
/// Decompose a constraint expression into `(class_name, type_argument)`.
///
/// A constraint may be presented either as a bare `Const("Eq")` with a
/// separate `ty` argument (the registry-entry form), or as a full
/// application `App(Const("Eq"), Const("Nat"))` where the class name and
/// its type parameter are bundled together.
///
/// Returns `(class, ty)` suitable for registry lookup.
fn decompose_constraint(class: &Name, ty: &Expr) -> (Name, Expr) {
    if let Some(Expr::Const(head_name, _)) = expr_head(ty) {
        if head_name == class {
            let args = expr_args(ty);
            if let Some(first_arg) = args.first() {
                return (class.clone(), (*first_arg).clone());
            }
        }
    }
    (class.clone(), ty.clone())
}
/// Resolve a type class constraint.
///
/// The resolution proceeds in three steps:
/// 1. Decompose the constraint into `(class_name, type_argument)` — handling
///    the case where the full constraint is encoded as `App(ClassName, Type)`.
/// 2. Search the registry for the highest-priority matching instance using
///    structural head-matching (BVar wildcard).
/// 3. Return the implementation expression of the found instance.
pub fn resolve_constraint(
    _ctx: &ElabContext,
    registry: &TypeClassRegistry,
    class: &Name,
    ty: &Expr,
) -> Option<Expr> {
    let (resolved_class, resolved_ty) = decompose_constraint(class, ty);
    registry
        .find_best_instance(&resolved_class, &resolved_ty)
        .map(|inst| inst.implementation.clone())
}
/// Build an `Eq` class definition.
pub fn make_eq_class() -> TypeClass {
    let mut cls = TypeClass::new(Name::str("Eq"), vec![Name::str("α")]);
    cls.add_method(Method::new(
        Name::str("eq"),
        Expr::Const(Name::str("Eq.decEq"), vec![]),
    ));
    cls
}
/// Build an `Ord` class definition (extends `Eq`).
pub fn make_ord_class() -> TypeClass {
    let mut cls = TypeClass::new(Name::str("Ord"), vec![Name::str("α")]);
    cls.add_superclass(Name::str("Eq"));
    cls.add_method(Method::new(
        Name::str("compare"),
        Expr::Const(Name::str("Ord.compare"), vec![]),
    ));
    cls
}
/// Build a `Functor` class definition.
pub fn make_functor_class() -> TypeClass {
    let mut cls = TypeClass::new(Name::str("Functor"), vec![Name::str("f")]);
    cls.add_method(Method::new(
        Name::str("map"),
        Expr::Const(Name::str("Functor.map"), vec![]),
    ));
    cls
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::typeclass::*;
    fn nat_ty() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    fn bool_ty() -> Expr {
        Expr::Const(Name::str("Bool"), vec![])
    }
    fn sort0() -> Expr {
        Expr::Sort(Level::zero())
    }
    fn make_nat_eq_inst() -> Instance {
        Instance::new(Name::str("Eq"), nat_ty(), sort0()).with_name(Name::str("instEqNat"))
    }
    #[test]
    fn test_registry_create() {
        let registry = TypeClassRegistry::new();
        assert_eq!(registry.class_count(), 0);
        assert_eq!(registry.instance_count(), 0);
    }
    #[test]
    fn test_register_class() {
        let mut registry = TypeClassRegistry::new();
        registry.register_class(TypeClass::new(Name::str("Eq"), vec![Name::str("α")]));
        assert!(registry.get_class(&Name::str("Eq")).is_some());
        assert_eq!(registry.class_count(), 1);
    }
    #[test]
    fn test_register_instance() {
        let mut registry = TypeClassRegistry::new();
        registry.register_instance(make_nat_eq_inst());
        assert_eq!(registry.instance_count(), 1);
    }
    #[test]
    fn test_find_instance_exact() {
        let mut registry = TypeClassRegistry::new();
        registry.register_instance(make_nat_eq_inst());
        let found = registry.find_instance(&Name::str("Eq"), &nat_ty());
        assert!(found.is_some());
    }
    #[test]
    fn test_find_instance_wrong_class() {
        let mut registry = TypeClassRegistry::new();
        registry.register_instance(make_nat_eq_inst());
        let found = registry.find_instance(&Name::str("Ord"), &nat_ty());
        assert!(found.is_none());
    }
    #[test]
    fn test_find_instance_wrong_ty() {
        let mut registry = TypeClassRegistry::new();
        registry.register_instance(make_nat_eq_inst());
        let found = registry.find_instance(&Name::str("Eq"), &bool_ty());
        assert!(found.is_none());
    }
    #[test]
    fn test_get_instances() {
        let mut registry = TypeClassRegistry::new();
        registry.register_instance(make_nat_eq_inst());
        registry.register_instance(
            Instance::new(Name::str("Eq"), bool_ty(), sort0()).with_name(Name::str("instEqBool")),
        );
        assert_eq!(registry.get_instances(&Name::str("Eq")).len(), 2);
    }
    #[test]
    fn test_method_has_default() {
        let m = Method::with_default(
            Name::str("foo"),
            sort0(),
            Expr::Const(Name::str("foo_default"), vec![]),
        );
        assert!(m.has_default());
        let m2 = Method::new(Name::str("bar"), sort0());
        assert!(!m2.has_default());
    }
    #[test]
    fn test_typeclass_add_method() {
        let mut cls = TypeClass::new(Name::str("Eq"), vec![Name::str("α")]);
        cls.add_method(Method::new(Name::str("eq"), sort0()));
        assert_eq!(cls.method_count(), 1);
        assert!(cls.get_method(&Name::str("eq")).is_some());
    }
    #[test]
    fn test_typeclass_arity() {
        let cls = TypeClass::new(Name::str("Eq"), vec![Name::str("α"), Name::str("β")]);
        assert_eq!(cls.arity(), 2);
    }
    #[test]
    fn test_instance_method_impl() {
        let mut inst = make_nat_eq_inst();
        inst.add_method_impl(Name::str("eq"), sort0());
        assert!(inst.get_method_impl(&Name::str("eq")).is_some());
        assert!(inst.get_method_impl(&Name::str("cmp")).is_none());
    }
    #[test]
    fn test_instance_is_complete() {
        let mut cls = TypeClass::new(Name::str("Eq"), vec![Name::str("α")]);
        cls.add_method(Method::new(Name::str("eq"), sort0()));
        let mut inst = make_nat_eq_inst();
        assert!(!inst.is_complete_for(&cls));
        inst.add_method_impl(Name::str("eq"), sort0());
        assert!(inst.is_complete_for(&cls));
    }
    #[test]
    fn test_check_completeness_errors() {
        let mut registry = TypeClassRegistry::new();
        let mut cls = TypeClass::new(Name::str("Eq"), vec![Name::str("α")]);
        cls.add_method(Method::new(Name::str("eq"), sort0()));
        registry.register_class(cls);
        registry.register_instance(make_nat_eq_inst());
        let errors = registry.check_completeness();
        assert_eq!(errors.len(), 1);
    }
    #[test]
    fn test_coherence_check_no_errors() {
        let mut registry = TypeClassRegistry::new();
        registry.register_instance(make_nat_eq_inst());
        registry.register_instance(
            Instance::new(Name::str("Eq"), bool_ty(), sort0()).with_name(Name::str("instEqBool")),
        );
        assert!(registry.check_coherence().is_empty());
    }
    #[test]
    fn test_coherence_check_incoherence() {
        let mut registry = TypeClassRegistry::new();
        registry.register_instance(make_nat_eq_inst());
        registry.register_instance(
            Instance::new(Name::str("Eq"), nat_ty(), sort0()).with_name(Name::str("instEqNat2")),
        );
        let errors = registry.check_coherence();
        assert_eq!(errors.len(), 1);
    }
    #[test]
    fn test_pending_constraints() {
        let mut registry = TypeClassRegistry::new();
        registry.register_instance(make_nat_eq_inst());
        registry.add_constraint(ClassConstraint::new(Name::str("Eq"), nat_ty()));
        registry.add_constraint(ClassConstraint::new(Name::str("Ord"), nat_ty()));
        let unresolved = registry.resolve_pending();
        assert_eq!(unresolved.len(), 1);
    }
    #[test]
    fn test_make_eq_class() {
        let cls = make_eq_class();
        assert_eq!(cls.name, Name::str("Eq"));
        assert_eq!(cls.method_count(), 1);
    }
    #[test]
    fn test_make_ord_class() {
        let cls = make_ord_class();
        assert_eq!(cls.superclasses.len(), 1);
        assert_eq!(cls.superclasses[0], Name::str("Eq"));
    }
    #[test]
    fn test_class_error_display() {
        let e = ClassError::NoInstance {
            class: Name::str("Eq"),
            ty: nat_ty(),
        };
        assert!(!e.to_string().is_empty());
    }
    #[test]
    fn test_priority_selection() {
        let mut registry = TypeClassRegistry::new();
        registry.register_instance(
            Instance::new(Name::str("Eq"), nat_ty(), sort0())
                .with_name(Name::str("instEqNatLow"))
                .with_priority(200),
        );
        registry.register_instance(
            Instance::new(Name::str("Eq"), nat_ty(), sort0())
                .with_name(Name::str("instEqNatHigh"))
                .with_priority(10),
        );
        let best = registry
            .find_best_instance(&Name::str("Eq"), &nat_ty())
            .expect("test operation should succeed");
        assert_eq!(best.priority, 10);
    }
    #[test]
    fn test_resolve_superclasses() {
        let mut registry = TypeClassRegistry::new();
        let mut ord = make_ord_class();
        ord.add_superclass(Name::str("Eq"));
        registry.register_class(ord);
        registry.register_instance(make_nat_eq_inst());
        let supers = registry.resolve_superclasses(&Name::str("Ord"), &nat_ty());
        assert!(supers.contains_key(&Name::str("Eq")));
    }
}
/// Derive instances using registered rules.
///
/// Iterates through all registered rules and, for those whose premises are
/// satisfied for `ty`, produces a new instance.
pub fn derive_instances(
    registry: &mut TypeClassRegistry,
    rules: &[DeriveRule],
    ty: &Expr,
) -> Vec<Instance> {
    let mut derived = Vec::new();
    for rule in rules {
        let premises_satisfied = rule
            .premise_classes
            .iter()
            .all(|cls| registry.find_best_instance(cls, ty).is_some());
        if premises_satisfied
            && registry
                .find_best_instance(&rule.conclusion_class, ty)
                .is_none()
        {
            let impl_expr = match &rule.builder {
                DeriveBuilder::Combinator(c) => Expr::Const(c.clone(), vec![]),
                DeriveBuilder::Sorry => Expr::Const(Name::str("sorry"), vec![]),
            };
            let inst = Instance::new(rule.conclusion_class.clone(), ty.clone(), impl_expr)
                .with_name(rule.name.clone());
            derived.push(inst);
        }
    }
    derived
}
/// Compute all transitive superclasses of a given class.
///
/// Performs a BFS over the superclass relation.
pub fn all_superclasses(registry: &TypeClassRegistry, class: &Name) -> Vec<Name> {
    let mut visited = std::collections::HashSet::new();
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(class.clone());
    let mut result = Vec::new();
    while let Some(cur) = queue.pop_front() {
        if !visited.insert(cur.clone()) {
            continue;
        }
        if let Some(cls) = registry.get_class(&cur) {
            for super_name in &cls.superclasses {
                if !visited.contains(super_name) {
                    queue.push_back(super_name.clone());
                    result.push(super_name.clone());
                }
            }
        }
    }
    result
}
#[cfg(test)]
mod tests_extra {
    use super::*;
    use crate::typeclass::*;
    fn nat_ty() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    fn sort0() -> Expr {
        Expr::Sort(Level::zero())
    }
    #[test]
    fn test_derive_rule_sorry() {
        let rule = DeriveRule::sorry(
            Name::str("deriveHashFromEq"),
            Name::str("Hash"),
            vec![Name::str("Eq")],
        );
        assert_eq!(rule.conclusion_class, Name::str("Hash"));
        assert_eq!(rule.premise_classes.len(), 1);
    }
    #[test]
    fn test_derive_instances_with_satisfied_premise() {
        let mut registry = TypeClassRegistry::new();
        registry.register_instance(
            Instance::new(Name::str("Eq"), nat_ty(), sort0()).with_name(Name::str("instEqNat")),
        );
        let rules = vec![DeriveRule::sorry(
            Name::str("instHashNat"),
            Name::str("Hash"),
            vec![Name::str("Eq")],
        )];
        let derived = derive_instances(&mut registry, &rules, &nat_ty());
        assert_eq!(derived.len(), 1);
        assert_eq!(derived[0].class, Name::str("Hash"));
    }
    #[test]
    fn test_derive_instances_unsatisfied_premise() {
        let mut registry = TypeClassRegistry::new();
        let rules = vec![DeriveRule::sorry(
            Name::str("instHashNat"),
            Name::str("Hash"),
            vec![Name::str("Eq")],
        )];
        let derived = derive_instances(&mut registry, &rules, &nat_ty());
        assert!(derived.is_empty());
    }
    #[test]
    fn test_derive_instances_no_duplicate() {
        let mut registry = TypeClassRegistry::new();
        registry.register_instance(
            Instance::new(Name::str("Eq"), nat_ty(), sort0()).with_name(Name::str("instEqNat")),
        );
        registry.register_instance(
            Instance::new(Name::str("Hash"), nat_ty(), sort0())
                .with_name(Name::str("instHashNatExisting")),
        );
        let rules = vec![DeriveRule::sorry(
            Name::str("instHashNat"),
            Name::str("Hash"),
            vec![Name::str("Eq")],
        )];
        let derived = derive_instances(&mut registry, &rules, &nat_ty());
        assert!(derived.is_empty(), "should not re-derive existing instance");
    }
    #[test]
    fn test_all_superclasses_empty() {
        let registry = TypeClassRegistry::new();
        let supers = all_superclasses(&registry, &Name::str("Eq"));
        assert!(supers.is_empty());
    }
    #[test]
    fn test_all_superclasses_chain() {
        let mut registry = TypeClassRegistry::new();
        let eq = TypeClass::new(Name::str("Eq"), vec![Name::str("α")]);
        let mut ord = TypeClass::new(Name::str("Ord"), vec![Name::str("α")]);
        ord.add_superclass(Name::str("Eq"));
        let mut ord_plus = TypeClass::new(Name::str("OrdPlus"), vec![Name::str("α")]);
        ord_plus.add_superclass(Name::str("Ord"));
        registry.register_class(eq);
        registry.register_class(ord);
        registry.register_class(ord_plus);
        let supers = all_superclasses(&registry, &Name::str("OrdPlus"));
        assert!(supers.contains(&Name::str("Ord")));
        assert!(supers.contains(&Name::str("Eq")));
    }
    #[test]
    fn test_class_error_ambiguous_display() {
        let e = ClassError::Ambiguous {
            class: Name::str("Eq"),
            candidates: vec![Name::str("a"), Name::str("b")],
        };
        assert!(e.to_string().contains("ambiguous"));
    }
    #[test]
    fn test_class_error_missing_method_display() {
        let e = ClassError::MissingMethod {
            instance: Name::str("I"),
            method: Name::str("eq"),
        };
        assert!(e.to_string().contains("missing method"));
    }
    #[test]
    fn test_functor_class() {
        let cls = make_functor_class();
        assert_eq!(cls.name, Name::str("Functor"));
        assert_eq!(cls.method_count(), 1);
        assert!(cls.get_method(&Name::str("map")).is_some());
    }
    #[test]
    fn test_instance_with_priority() {
        let inst = Instance::new(Name::str("Eq"), nat_ty(), sort0()).with_priority(42);
        assert_eq!(inst.priority, 42);
    }
    #[test]
    fn test_type_matches_exact_equal() {
        assert!(type_matches(&nat_ty(), &nat_ty()));
    }
    #[test]
    fn test_type_matches_bvar_wildcard() {
        let bvar = Expr::BVar(0);
        assert!(type_matches(&bvar, &nat_ty()));
    }
    #[test]
    fn test_type_matches_const_different_names() {
        let bool_ty = Expr::Const(Name::str("Bool"), vec![]);
        assert!(!type_matches(&nat_ty(), &bool_ty));
    }
    #[test]
    fn test_type_matches_app_heads_equal() {
        let list_nat = Expr::App(
            Box::new(Expr::Const(Name::str("List"), vec![])),
            Box::new(nat_ty()),
        );
        let list_bool = Expr::App(
            Box::new(Expr::Const(Name::str("List"), vec![])),
            Box::new(Expr::Const(Name::str("Bool"), vec![])),
        );
        assert!(!type_matches(&list_nat, &list_bool));
        let list_wild = Expr::App(
            Box::new(Expr::Const(Name::str("List"), vec![])),
            Box::new(Expr::BVar(0)),
        );
        assert!(type_matches(&list_wild, &list_nat));
        assert!(type_matches(&list_wild, &list_bool));
    }
    #[test]
    fn test_type_matches_sort_mismatch() {
        assert!(!type_matches(&sort0(), &nat_ty()));
    }
    #[test]
    fn test_decompose_bare_class() {
        let (cls, ty) = decompose_constraint(&Name::str("Eq"), &nat_ty());
        assert_eq!(cls, Name::str("Eq"));
        assert_eq!(ty, nat_ty());
    }
    #[test]
    fn test_decompose_app_constraint() {
        let eq_nat = Expr::App(
            Box::new(Expr::Const(Name::str("Eq"), vec![])),
            Box::new(nat_ty()),
        );
        let (cls, ty) = decompose_constraint(&Name::str("Eq"), &eq_nat);
        assert_eq!(cls, Name::str("Eq"));
        assert_eq!(ty, nat_ty());
    }
    #[test]
    fn test_decompose_app_different_head() {
        let ord_nat = Expr::App(
            Box::new(Expr::Const(Name::str("Ord"), vec![])),
            Box::new(nat_ty()),
        );
        let (cls, ty) = decompose_constraint(&Name::str("Eq"), &ord_nat);
        assert_eq!(cls, Name::str("Eq"));
        assert_eq!(ty, ord_nat);
    }
    #[test]
    fn test_resolve_constraint_found() {
        let mut registry = TypeClassRegistry::new();
        let impl_expr = Expr::Const(Name::str("instEqNat"), vec![]);
        registry.register_instance(
            Instance::new(Name::str("Eq"), nat_ty(), impl_expr.clone())
                .with_name(Name::str("instEqNat")),
        );
        let env = oxilean_kernel::Environment::new();
        let ctx = crate::context::ElabContext::new(&env);
        let result = resolve_constraint(&ctx, &registry, &Name::str("Eq"), &nat_ty());
        assert_eq!(result, Some(impl_expr));
    }
    #[test]
    fn test_resolve_constraint_not_found() {
        let registry = TypeClassRegistry::new();
        let env = oxilean_kernel::Environment::new();
        let ctx = crate::context::ElabContext::new(&env);
        let result = resolve_constraint(&ctx, &registry, &Name::str("Eq"), &nat_ty());
        assert!(result.is_none());
    }
    #[test]
    fn test_resolve_constraint_app_form() {
        let mut registry = TypeClassRegistry::new();
        let impl_expr = Expr::Const(Name::str("instEqNat"), vec![]);
        registry.register_instance(
            Instance::new(Name::str("Eq"), nat_ty(), impl_expr.clone())
                .with_name(Name::str("instEqNat")),
        );
        let env = oxilean_kernel::Environment::new();
        let ctx = crate::context::ElabContext::new(&env);
        let app_constraint = Expr::App(
            Box::new(Expr::Const(Name::str("Eq"), vec![])),
            Box::new(nat_ty()),
        );
        let result = resolve_constraint(&ctx, &registry, &Name::str("Eq"), &app_constraint);
        assert_eq!(result, Some(impl_expr));
    }
    #[test]
    fn test_resolve_constraint_prefers_higher_priority() {
        let mut registry = TypeClassRegistry::new();
        let low_impl = Expr::Const(Name::str("instLow"), vec![]);
        let high_impl = Expr::Const(Name::str("instHigh"), vec![]);
        registry.register_instance(
            Instance::new(Name::str("Eq"), nat_ty(), low_impl)
                .with_name(Name::str("instLow"))
                .with_priority(200),
        );
        registry.register_instance(
            Instance::new(Name::str("Eq"), nat_ty(), high_impl.clone())
                .with_name(Name::str("instHigh"))
                .with_priority(5),
        );
        let env = oxilean_kernel::Environment::new();
        let ctx = crate::context::ElabContext::new(&env);
        let result = resolve_constraint(&ctx, &registry, &Name::str("Eq"), &nat_ty());
        assert_eq!(result, Some(high_impl));
    }
}
#[cfg(test)]
mod typeclass_extra_tests {
    use super::*;
    use crate::typeclass::*;
    use oxilean_kernel::{Expr, Level, Name};
    fn nat_ty() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    fn sort0() -> Expr {
        Expr::Sort(Level::zero())
    }
    #[test]
    fn test_instance_set_insert_order() {
        let mut set = InstanceSet::new(Name::str("Eq"));
        let i1 = Instance::new(Name::str("Eq"), nat_ty(), sort0()).with_priority(10);
        let i2 = Instance::new(Name::str("Eq"), nat_ty(), sort0()).with_priority(100);
        set.insert(i1.clone());
        set.insert(i2.clone());
        assert_eq!(
            set.best().expect("test operation should succeed").priority,
            100
        );
    }
    #[test]
    fn test_instance_set_is_empty() {
        let set = InstanceSet::new(Name::str("Eq"));
        assert!(set.is_empty());
    }
    #[test]
    fn test_instance_set_len() {
        let mut set = InstanceSet::new(Name::str("Eq"));
        set.insert(Instance::new(Name::str("Eq"), nat_ty(), sort0()));
        assert_eq!(set.len(), 1);
    }
    #[test]
    fn test_coherence_checker_no_violations() {
        let mut checker = CoherenceChecker::new();
        let mut set = InstanceSet::new(Name::str("Eq"));
        let i1 = Instance::new(Name::str("Eq"), nat_ty(), sort0()).with_priority(10);
        let i2 = Instance::new(Name::str("Eq"), nat_ty(), sort0()).with_priority(20);
        set.insert(i1);
        set.insert(i2);
        checker.check(&Name::str("Eq"), &set);
        assert!(!checker.has_violations());
    }
    #[test]
    fn test_coherence_checker_violation() {
        let mut checker = CoherenceChecker::new();
        let mut set = InstanceSet::new(Name::str("Eq"));
        let i1 = Instance::new(Name::str("Eq"), nat_ty(), sort0()).with_priority(50);
        let i2 = Instance::new(Name::str("Eq"), nat_ty(), sort0()).with_priority(50);
        set.insert(i1);
        set.insert(i2);
        checker.check(&Name::str("Eq"), &set);
        assert!(checker.has_violations());
    }
    #[test]
    fn test_coherence_violation_display() {
        let v = CoherenceViolation::new(Name::str("Eq"), None, None, "conflict");
        assert!(v.to_string().contains("coherence violation"));
    }
    #[test]
    fn test_instance_search_result_found() {
        let inst = Instance::new(Name::str("Eq"), nat_ty(), sort0());
        let r = InstanceSearchResult::Found(inst);
        assert!(r.is_found());
        assert!(!r.is_not_found());
        assert!(!r.is_ambiguous());
    }
    #[test]
    fn test_instance_search_result_not_found() {
        let r = InstanceSearchResult::NotFound;
        assert!(r.is_not_found());
        assert!(r.into_option().is_none());
    }
    #[test]
    fn test_default_method_registry() {
        let mut reg = DefaultMethodRegistry::new();
        reg.register(
            Name::str("Functor"),
            Name::str("map"),
            Expr::Const(Name::str("defaultMap"), vec![]),
        );
        assert!(!reg.is_empty());
        assert_eq!(reg.len(), 1);
        assert!(reg.get(&Name::str("Functor"), &Name::str("map")).is_some());
        assert!(reg.get(&Name::str("Functor"), &Name::str("pure")).is_none());
    }
    #[test]
    fn test_instance_context_stack_push_pop() {
        let mut stack = InstanceContextStack::new();
        stack.push_frame();
        let inst = Instance::new(Name::str("Eq"), nat_ty(), sort0());
        stack.add_local(inst);
        assert_eq!(stack.all_locals().len(), 1);
        let frame = stack.pop_frame();
        assert_eq!(frame.len(), 1);
        assert!(stack.all_locals().is_empty());
    }
    #[test]
    fn test_instance_context_stack_depth() {
        let mut stack = InstanceContextStack::new();
        assert_eq!(stack.depth(), 0);
        stack.push_frame();
        assert_eq!(stack.depth(), 1);
        stack.push_frame();
        assert_eq!(stack.depth(), 2);
        stack.pop_frame();
        assert_eq!(stack.depth(), 1);
    }
    #[test]
    fn test_typeclass_query_new() {
        let q = TypeClassQuery::new(Name::str("Eq"), nat_ty());
        assert_eq!(q.class, Name::str("Eq"));
        assert_eq!(q.max_depth, 8);
        assert!(!q.allow_synthetic);
    }
    #[test]
    fn test_typeclass_query_with_max_depth() {
        let q = TypeClassQuery::new(Name::str("Eq"), nat_ty()).with_max_depth(4);
        assert_eq!(q.max_depth, 4);
    }
    #[test]
    fn test_typeclass_query_allow_synthetic() {
        let q = TypeClassQuery::new(Name::str("Eq"), nat_ty()).allow_synthetic();
        assert!(q.allow_synthetic);
    }
    #[test]
    fn test_instance_set_remove_by_name() {
        let mut set = InstanceSet::new(Name::str("Eq"));
        let i1 = Instance::new(Name::str("Eq"), nat_ty(), sort0()).with_name(Name::str("myInst"));
        set.insert(i1);
        assert_eq!(set.len(), 1);
        let removed = set.remove_by_name(&Name::str("myInst"));
        assert!(removed);
        assert!(set.is_empty());
    }
}
/// Fill all missing methods in an instance with their defaults.
#[allow(dead_code)]
pub fn fill_default_methods(class: &TypeClass, provided: &[Name]) -> Vec<DefaultMethodFill> {
    class
        .methods
        .iter()
        .filter(|m| m.default_impl.is_some() && !provided.contains(&m.name))
        .map(|m| {
            DefaultMethodFill::from_default(
                m.name.clone(),
                m.default_impl
                    .clone()
                    .expect("default_impl is Some: filtered by is_some() above"),
            )
        })
        .collect()
}
/// Run basic coherence checks on an `InstanceSet`.
#[allow(dead_code)]
pub fn check_coherence(set: &InstanceSet) -> Vec<CoherenceError> {
    let mut errors = Vec::new();
    let instances: Vec<&Instance> = set.iter().collect();
    for i in 0..instances.len() {
        for j in (i + 1)..instances.len() {
            let a = instances[i];
            let b = instances[j];
            if a.ty == b.ty {
                errors.push(CoherenceError::OverlappingInstances(
                    a.name.clone().unwrap_or_else(|| Name::str("_")),
                    b.name.clone().unwrap_or_else(|| Name::str("_")),
                ));
            }
        }
    }
    errors
}
#[cfg(test)]
mod typeclass_ext_tests {
    use super::*;
    use crate::typeclass::*;
    fn nat_ty() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    fn sort0() -> Expr {
        Expr::Sort(oxilean_kernel::Level::zero())
    }
    #[test]
    fn test_hierarchy_direct_superclasses() {
        let mut h = TypeClassHierarchy::new();
        h.add_superclass(SuperclassEdge::new(
            Name::str("Ord"),
            Name::str("Eq"),
            Name::str("Ord.toEq"),
            0,
        ));
        let parents = h.direct_superclasses(&Name::str("Ord"));
        assert_eq!(parents.len(), 1);
        assert_eq!(parents[0].parent, Name::str("Eq"));
    }
    #[test]
    fn test_hierarchy_is_superclass_direct() {
        let mut h = TypeClassHierarchy::new();
        h.add_superclass(SuperclassEdge::new(
            Name::str("Ord"),
            Name::str("Eq"),
            Name::str("Ord.toEq"),
            0,
        ));
        assert!(h.is_superclass(&Name::str("Ord"), &Name::str("Eq")));
        assert!(!h.is_superclass(&Name::str("Eq"), &Name::str("Ord")));
    }
    #[test]
    fn test_hierarchy_is_superclass_transitive() {
        let mut h = TypeClassHierarchy::new();
        h.add_superclass(SuperclassEdge::new(
            Name::str("LinearOrder"),
            Name::str("Ord"),
            Name::str("LinearOrder.toOrd"),
            0,
        ));
        h.add_superclass(SuperclassEdge::new(
            Name::str("Ord"),
            Name::str("Eq"),
            Name::str("Ord.toEq"),
            0,
        ));
        assert!(h.is_superclass(&Name::str("LinearOrder"), &Name::str("Eq")));
    }
    #[test]
    fn test_hierarchy_all_superclasses() {
        let mut h = TypeClassHierarchy::new();
        h.add_superclass(SuperclassEdge::new(
            Name::str("A"),
            Name::str("B"),
            Name::str("A.toB"),
            0,
        ));
        h.add_superclass(SuperclassEdge::new(
            Name::str("B"),
            Name::str("C"),
            Name::str("B.toC"),
            0,
        ));
        let supers = h.all_superclasses(&Name::str("A"));
        assert!(supers.contains(&Name::str("B")));
        assert!(supers.contains(&Name::str("C")));
    }
    #[test]
    fn test_hierarchy_self_is_superclass() {
        let h = TypeClassHierarchy::new();
        assert!(h.is_superclass(&Name::str("Eq"), &Name::str("Eq")));
    }
    #[test]
    fn test_instance_cache_insert_lookup() {
        let mut cache = InstanceCache::new();
        let key = InstanceCacheKey::new(Name::str("Eq"), "Nat");
        cache.insert(
            key.clone(),
            InstanceCacheEntry::Found(Name::str("instEqNat")),
        );
        let entry = cache.lookup(&key);
        assert!(matches!(entry, Some(InstanceCacheEntry::Found(_))));
    }
    #[test]
    fn test_instance_cache_miss() {
        let mut cache = InstanceCache::new();
        let key = InstanceCacheKey::new(Name::str("Eq"), "Int");
        let entry = cache.lookup(&key);
        assert!(entry.is_none());
        assert!((cache.hit_rate() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_instance_cache_hit_rate() {
        let mut cache = InstanceCache::new();
        let key = InstanceCacheKey::new(Name::str("Eq"), "Nat");
        cache.insert(key.clone(), InstanceCacheEntry::NotFound);
        cache.lookup(&key);
        cache.lookup(&InstanceCacheKey::new(Name::str("Eq"), "Int"));
        assert!((cache.hit_rate() - 0.5).abs() < 1e-10);
    }
    #[test]
    fn test_instance_cache_invalidate() {
        let mut cache = InstanceCache::new();
        let key = InstanceCacheKey::new(Name::str("Eq"), "Nat");
        cache.insert(key.clone(), InstanceCacheEntry::Found(Name::str("inst")));
        cache.invalidate(&key);
        assert!(cache.is_empty());
    }
    #[test]
    fn test_instance_cache_clear() {
        let mut cache = InstanceCache::new();
        let key = InstanceCacheKey::new(Name::str("Eq"), "Nat");
        cache.insert(key, InstanceCacheEntry::NotFound);
        cache.clear();
        assert!(cache.is_empty());
    }
    #[test]
    fn test_coherence_error_display() {
        let e = CoherenceError::OverlappingInstances(Name::str("a"), Name::str("b"));
        let s = e.to_string();
        assert!(s.contains("overlapping"));
    }
    #[test]
    fn test_check_coherence_no_error() {
        let mut set = InstanceSet::new(Name::str("Eq"));
        set.insert(Instance::new(Name::str("Eq"), nat_ty(), sort0()));
        let errs = check_coherence(&set);
        assert!(errs.is_empty());
    }
    #[test]
    fn test_check_coherence_duplicate() {
        let mut set = InstanceSet::new(Name::str("Eq"));
        set.insert(Instance::new(Name::str("Eq"), nat_ty(), sort0()));
        set.insert(Instance::new(Name::str("Eq"), nat_ty(), sort0()));
        let errs = check_coherence(&set);
        assert!(!errs.is_empty());
    }
    #[test]
    fn test_synth_stats_record() {
        let mut s = SynthStats::new();
        s.record(true, true, 3);
        s.record(false, false, 5);
        assert_eq!(s.requests, 2);
        assert_eq!(s.successes, 1);
        assert_eq!(s.failures, 1);
        assert_eq!(s.cache_hits, 1);
        assert_eq!(s.max_search_depth, 5);
    }
    #[test]
    fn test_synth_stats_success_rate() {
        let mut s = SynthStats::new();
        s.record(true, false, 1);
        s.record(true, false, 1);
        s.record(false, false, 1);
        assert!((s.success_rate() - 2.0 / 3.0).abs() < 1e-10);
    }
    #[test]
    fn test_synth_stats_merge() {
        let mut a = SynthStats::new();
        a.record(true, true, 2);
        let mut b = SynthStats::new();
        b.record(false, false, 8);
        a.merge(&b);
        assert_eq!(a.requests, 2);
        assert_eq!(a.max_search_depth, 8);
    }
    #[test]
    fn test_synth_stats_summary() {
        let mut s = SynthStats::new();
        s.record(true, false, 1);
        let summary = s.summary();
        assert!(summary.contains("requests=1"));
        assert!(summary.contains("ok=1"));
    }
    #[test]
    fn test_default_method_fill_from_default() {
        let fill = DefaultMethodFill::from_default(Name::str("toStr"), nat_ty());
        assert!(!fill.is_synthesised);
    }
    #[test]
    fn test_default_method_fill_synthesised() {
        let fill = DefaultMethodFill::synthesised(Name::str("compare"), nat_ty());
        assert!(fill.is_synthesised);
    }
}
#[cfg(test)]
mod typeclass_budget_tests {
    use super::*;
    use crate::typeclass::*;
    #[test]
    fn test_instance_search_budget_default() {
        let b = InstanceSearchBudget::default();
        assert!(b.allows_candidates(100));
        assert!(!b.allows_candidates(200));
    }
    #[test]
    fn test_instance_search_budget_chain() {
        let b = InstanceSearchBudget::default();
        assert!(b.allows_chain_length(5));
        assert!(!b.allows_chain_length(20));
    }
    #[test]
    fn test_outcome_success() {
        let e = Expr::Const(Name::str("Nat.instAdd"), vec![]);
        let o = InstanceResolutionOutcome::Success(e.clone());
        assert!(o.is_success());
        assert!(o.success_expr().is_some());
    }
    #[test]
    fn test_outcome_no_instance() {
        let o = InstanceResolutionOutcome::NoInstance;
        assert!(o.is_no_instance());
        assert!(!o.is_success());
    }
    #[test]
    fn test_typeclass_dep_graph_deps_of() {
        let mut g = TypeclassDepGraph::new();
        g.add_dep(TypeclassDep {
            dependent: Name::str("Monoid"),
            dependency: Name::str("Semigroup"),
            is_superclass: true,
        });
        g.add_dep(TypeclassDep {
            dependent: Name::str("Group"),
            dependency: Name::str("Monoid"),
            is_superclass: true,
        });
        let deps = g.deps_of(&Name::str("Group"));
        assert_eq!(deps.len(), 1);
        let sc = g.superclass_edges();
        assert_eq!(sc.len(), 2);
    }
}
