//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::{Expr, Name};
use std::collections::HashMap;

use super::types::{
    ClassEdge, ConfigNode, DecisionNode, Either2, FlatSubstitution, FocusStack, Instance,
    InstanceImpl, InstancePriority, InstanceSearchResult, LabelSet, LayeredTypeClassRegistry,
    Method, MethodImpl, NonEmptyVec, NullResolver, PathBuf, RewriteRule, RewriteRuleSet, SimpleDag,
    SlidingSum, SmallMap, SparseVec, StackCalc, StatSummary, Stopwatch, StringPool, TokenBucket,
    TransformStat, TransitiveClosure, TypeClass, TypeClassRegistry, TypeClassStats,
    VersionedRecord, WindowIterator, WriteOnce,
};

/// Check whether two type expressions are compatible for instance matching.
///
/// `needle` is the queried type; `haystack` is the registered instance type.
/// Returns `true` if `needle` matches `haystack`, treating free/bound
/// variables in `haystack` as wildcards (polymorphic instance parameters).
pub fn instances_match(needle: &Expr, haystack: &Expr) -> bool {
    match (needle, haystack) {
        _ if needle == haystack => true,
        (_, Expr::BVar(_)) | (_, Expr::FVar(_)) => true,
        (Expr::Sort(_), Expr::Sort(_)) => true,
        (Expr::Const(n1, _), Expr::Const(n2, _)) => n1 == n2,
        (Expr::App(f1, a1), Expr::App(f2, a2)) => {
            instances_match(f1, f2) && instances_match(a1, a2)
        }
        (Expr::Pi(_, _, d1, c1), Expr::Pi(_, _, d2, c2)) => {
            instances_match(d1, d2) && instances_match(c1, c2)
        }
        (Expr::Lam(_, _, d1, b1), Expr::Lam(_, _, d2, b2)) => {
            instances_match(d1, d2) && instances_match(b1, b2)
        }
        _ => false,
    }
}
/// Check whether an expression is a type class constraint application.
///
/// Returns `true` if the head of the expression is a registered class name.
pub fn is_class_constraint(expr: &Expr, registry: &TypeClassRegistry) -> bool {
    class_name_of_constraint(expr)
        .map(|n| registry.is_class(&n))
        .unwrap_or(false)
}
/// Extract the class name from a constraint expression.
///
/// Given `Add Nat` returns `Some("Add")`; given a bare `Nat` returns `None`.
pub fn class_name_of_constraint(expr: &Expr) -> Option<Name> {
    match expr {
        Expr::Const(name, _) => Some(name.clone()),
        Expr::App(f, _) => class_name_of_constraint(f),
        _ => None,
    }
}
/// Build the projection term for the n-th method of a class.
///
/// The projection is an application of `ClassName.methodName`.
pub fn build_method_projection(class: &Name, method: &Name, _index: usize) -> Expr {
    let proj_name = class.clone().append_str(method.to_string());
    Expr::Const(proj_name, vec![])
}
/// Register the built-in `Inhabited` class in the given registry.
pub fn register_inhabited(registry: &mut TypeClassRegistry) {
    use crate::Level;
    let name = Name::str("Inhabited");
    let ty = Expr::Sort(Level::succ(Level::zero()));
    let mut cls = TypeClass::new(name.clone(), vec![Name::str("α")], ty);
    cls.add_method(Method::new(Name::str("default"), Expr::BVar(0), 0));
    registry.register_class(cls);
}
/// Register the built-in `Add` class in the given registry.
pub fn register_add(registry: &mut TypeClassRegistry) {
    use crate::Level;
    let name = Name::str("Add");
    let ty = Expr::Sort(Level::succ(Level::zero()));
    let mut cls = TypeClass::new(name.clone(), vec![Name::str("α")], ty);
    let add_ty = Expr::Pi(
        crate::BinderInfo::Default,
        Name::str("a"),
        Box::new(Expr::BVar(0)),
        Box::new(Expr::Pi(
            crate::BinderInfo::Default,
            Name::str("b"),
            Box::new(Expr::BVar(1)),
            Box::new(Expr::BVar(2)),
        )),
    );
    cls.add_method(Method::new(Name::str("add"), add_ty, 0));
    registry.register_class(cls);
}
/// Register the built-in `Mul` class in the given registry.
pub fn register_mul(registry: &mut TypeClassRegistry) {
    use crate::Level;
    let name = Name::str("Mul");
    let ty = Expr::Sort(Level::succ(Level::zero()));
    let mut cls = TypeClass::new(name.clone(), vec![Name::str("α")], ty);
    let mul_ty = Expr::Pi(
        crate::BinderInfo::Default,
        Name::str("a"),
        Box::new(Expr::BVar(0)),
        Box::new(Expr::Pi(
            crate::BinderInfo::Default,
            Name::str("b"),
            Box::new(Expr::BVar(1)),
            Box::new(Expr::BVar(2)),
        )),
    );
    cls.add_method(Method::new(Name::str("mul"), mul_ty, 0));
    registry.register_class(cls);
}
/// Register the `HEq` (heterogeneous equality) class placeholder.
pub fn register_heq(registry: &mut TypeClassRegistry) {
    use crate::Level;
    let name = Name::str("HEq");
    let ty = Expr::Sort(Level::zero());
    let cls = TypeClass::new(name, vec![Name::str("α"), Name::str("β")], ty).mark_prop();
    registry.register_class(cls);
}
/// Register a simple zero-method marker class (like `Nonempty`).
pub fn register_marker(registry: &mut TypeClassRegistry, class_name: &str) {
    use crate::Level;
    let name = Name::str(class_name);
    let ty = Expr::Sort(Level::succ(Level::zero()));
    let cls = TypeClass::new(name, vec![Name::str("α")], ty);
    registry.register_class(cls);
}
/// Build a default registry with common built-in classes.
pub fn default_registry() -> TypeClassRegistry {
    let mut reg = TypeClassRegistry::new();
    register_inhabited(&mut reg);
    register_add(&mut reg);
    register_mul(&mut reg);
    register_heq(&mut reg);
    register_marker(&mut reg, "Nonempty");
    register_marker(&mut reg, "Decidable");
    register_marker(&mut reg, "DecidableEq");
    register_marker(&mut reg, "Fintype");
    reg
}
/// Check whether the given class name is a "pure Prop" class.
pub fn is_prop_class(name: &Name, registry: &TypeClassRegistry) -> bool {
    registry.get_class(name).map(|c| c.is_prop).unwrap_or(false)
}
/// Return all methods for a class, or an empty slice if the class is unknown.
pub fn methods_of(class: &Name, registry: &TypeClassRegistry) -> Vec<Method> {
    registry
        .get_class(class)
        .map(|c| c.methods.clone())
        .unwrap_or_default()
}
/// Check if an instance covers (implements) all required methods.
pub fn instance_is_complete(instance: &Instance, registry: &TypeClassRegistry) -> bool {
    let class_name = &instance.class;
    match registry.get_class(class_name) {
        None => false,
        Some(cls) => instance.implements_all(cls),
    }
}
/// Get all super-class names required by a class, transitively.
pub fn transitive_supers(class: &Name, registry: &TypeClassRegistry) -> Vec<Name> {
    let mut result = Vec::new();
    let mut work_list = vec![class.clone()];
    while let Some(c) = work_list.pop() {
        if let Some(cls) = registry.get_class(&c) {
            for sup in &cls.super_classes {
                if !result.contains(sup) {
                    result.push(sup.clone());
                    work_list.push(sup.clone());
                }
            }
        }
    }
    result
}
/// Merge two registries together (second registry overrides on conflicts).
pub fn merge_registries(base: TypeClassRegistry, overlay: TypeClassRegistry) -> TypeClassRegistry {
    let mut result = base;
    for (_, cls) in overlay.classes {
        result.register_class(cls);
    }
    for inst in overlay.instances {
        result.register_instance(inst);
    }
    result
}
/// Search for a method implementation across all matching instances.
pub fn find_method_impl(
    class: &Name,
    ty: &Expr,
    method: &str,
    registry: &TypeClassRegistry,
) -> Option<Expr> {
    let instances = registry.find_instances(class, ty);
    for inst in instances {
        if let Some(impl_expr) = inst.get_method_impl(method) {
            return Some(impl_expr.clone());
        }
    }
    None
}
/// Count the total number of method implementations across all instances.
pub fn total_method_impls(registry: &TypeClassRegistry) -> usize {
    registry.instances.iter().map(|i| i.methods.len()).sum()
}
/// Print a human-readable summary of all classes and instances.
pub fn describe_registry(registry: &TypeClassRegistry) -> String {
    let mut lines = Vec::new();
    lines.push("=== Type Class Registry ===".to_string());
    let mut class_names: Vec<&String> = registry.classes.keys().collect();
    class_names.sort();
    for name in class_names {
        let cls = &registry.classes[name];
        lines.push(format!("class {} ({} method(s))", name, cls.method_count()));
        for m in &cls.methods {
            lines.push(format!("  method: {}", m.name));
        }
    }
    lines.push(format!("--- {} instance(s) ---", registry.instance_count()));
    for inst in &registry.instances {
        let label = inst
            .instance_name
            .as_ref()
            .map(|n| n.to_string())
            .unwrap_or_else(|| "<anon>".to_string());
        lines.push(format!(
            "  instance {} : {} (priority {})",
            label, inst.class, inst.priority
        ));
    }
    lines.join("\n")
}
/// Check whether two class names are the same.
pub fn classes_equal(a: &Name, b: &Name) -> bool {
    a == b
}
/// Check whether an instance has priority strictly higher than another.
pub fn has_higher_priority(a: &Instance, b: &Instance) -> bool {
    a.priority < b.priority
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::Level;
    fn mk_sort() -> Expr {
        Expr::Sort(Level::zero())
    }
    fn mk_const(s: &str) -> Expr {
        Expr::Const(Name::str(s), vec![])
    }
    fn add_class() -> TypeClass {
        let mut cls = TypeClass::new(Name::str("Add"), vec![Name::str("α")], mk_sort());
        cls.add_method(Method::new(Name::str("add"), mk_sort(), 0));
        cls
    }
    fn nat_instance() -> Instance {
        Instance::new(Name::str("Add"), mk_const("Nat"))
    }
    #[test]
    fn test_register_and_find_class() {
        let mut reg = TypeClassRegistry::new();
        reg.register_class(add_class());
        assert!(reg.is_class(&Name::str("Add")));
        assert!(!reg.is_class(&Name::str("Mul")));
        assert_eq!(reg.class_count(), 1);
    }
    #[test]
    fn test_class_method_lookup() {
        let cls = add_class();
        assert!(cls.find_method(&Name::str("add")).is_some());
        assert!(cls.find_method(&Name::str("mul")).is_none());
        assert_eq!(cls.method_count(), 1);
    }
    #[test]
    fn test_instance_registration() {
        let mut reg = TypeClassRegistry::new();
        reg.register_class(add_class());
        reg.register_instance(nat_instance());
        assert_eq!(reg.instance_count(), 1);
    }
    #[test]
    fn test_find_instances() {
        let mut reg = TypeClassRegistry::new();
        reg.register_class(add_class());
        reg.register_instance(nat_instance());
        let found = reg.find_instances(&Name::str("Add"), &mk_const("Nat"));
        assert_eq!(found.len(), 1);
        let none = reg.find_instances(&Name::str("Add"), &mk_const("Int"));
        assert!(none.is_empty());
    }
    #[test]
    fn test_find_best_instance_unique() {
        let mut reg = TypeClassRegistry::new();
        reg.register_class(add_class());
        reg.register_instance(nat_instance());
        let result = reg.find_best_instance(&Name::str("Add"), &mk_const("Nat"));
        assert!(result.is_found());
    }
    #[test]
    fn test_find_best_instance_not_found() {
        let mut reg = TypeClassRegistry::new();
        reg.register_class(add_class());
        let result = reg.find_best_instance(&Name::str("Add"), &mk_const("Nat"));
        assert!(matches!(result, InstanceSearchResult::NotFound));
    }
    #[test]
    fn test_priority_ordering() {
        let mut reg = TypeClassRegistry::new();
        reg.register_class(add_class());
        let low = Instance::new(Name::str("Add"), mk_const("Nat")).with_priority(200);
        let high = Instance::new(Name::str("Add"), mk_const("Nat")).with_priority(50);
        reg.register_instance(low);
        reg.register_instance(high);
        let result = reg.find_best_instance(&Name::str("Add"), &mk_const("Nat"));
        if let InstanceSearchResult::Found(inst) = result {
            assert_eq!(inst.priority, 50);
        } else {
            panic!("Expected Found");
        }
    }
    #[test]
    fn test_ambiguous_instances() {
        let mut reg = TypeClassRegistry::new();
        reg.register_class(add_class());
        let a = Instance::new(Name::str("Add"), mk_const("Nat")).with_priority(100);
        let b = Instance::new(Name::str("Add"), mk_const("Nat")).with_priority(100);
        reg.register_instance(a);
        reg.register_instance(b);
        let result = reg.find_best_instance(&Name::str("Add"), &mk_const("Nat"));
        assert!(result.is_ambiguous());
    }
    #[test]
    fn test_local_instance_clear() {
        let mut reg = TypeClassRegistry::new();
        reg.register_class(add_class());
        let local = nat_instance().as_local();
        let global = Instance::named(Name::str("Add"), mk_const("Nat"), Name::str("natAdd"));
        reg.register_instance(local);
        reg.register_instance(global);
        assert_eq!(reg.instance_count(), 2);
        reg.clear_local_instances();
        assert_eq!(reg.instance_count(), 1);
    }
    #[test]
    fn test_class_name_of_constraint() {
        let c = mk_const("Add");
        assert_eq!(class_name_of_constraint(&c), Some(Name::str("Add")));
        let app = Expr::App(Box::new(mk_const("Add")), Box::new(mk_const("Nat")));
        assert_eq!(class_name_of_constraint(&app), Some(Name::str("Add")));
        assert_eq!(class_name_of_constraint(&Expr::BVar(0)), None);
    }
    #[test]
    fn test_default_registry() {
        let reg = default_registry();
        assert!(reg.is_class(&Name::str("Inhabited")));
        assert!(reg.is_class(&Name::str("Add")));
        assert!(reg.is_class(&Name::str("Mul")));
        assert!(reg.is_class(&Name::str("Decidable")));
    }
    #[test]
    fn test_transitive_supers() {
        let mut reg = TypeClassRegistry::new();
        let cls_a = TypeClass::new(Name::str("A"), vec![], mk_sort());
        let mut cls_b = TypeClass::new(Name::str("B"), vec![], mk_sort());
        cls_b.add_super(Name::str("A"));
        let mut cls_c = TypeClass::new(Name::str("C"), vec![], mk_sort());
        cls_c.add_super(Name::str("B"));
        reg.register_class(cls_a);
        reg.register_class(cls_b);
        reg.register_class(cls_c);
        let supers = transitive_supers(&Name::str("C"), &reg);
        assert!(supers.contains(&Name::str("B")));
        assert!(supers.contains(&Name::str("A")));
    }
    #[test]
    fn test_method_projection() {
        let mut reg = TypeClassRegistry::new();
        reg.register_class(add_class());
        let proj = reg.method_projection(&Name::str("Add"), &Name::str("add"));
        assert!(proj.is_some());
    }
    #[test]
    fn test_instance_method_impl() {
        let mut inst = nat_instance();
        inst.add_method_impl("add", mk_const("Nat.add"));
        assert_eq!(inst.get_method_impl("add"), Some(&mk_const("Nat.add")));
        assert_eq!(inst.implemented_count(), 1);
    }
    #[test]
    fn test_merge_registries() {
        let mut reg1 = TypeClassRegistry::new();
        reg1.register_class(add_class());
        let mut reg2 = TypeClassRegistry::new();
        reg2.register_instance(nat_instance());
        let merged = merge_registries(reg1, reg2);
        assert_eq!(merged.class_count(), 1);
        assert_eq!(merged.instance_count(), 1);
    }
    #[test]
    fn test_instances_for_class() {
        let mut reg = TypeClassRegistry::new();
        reg.register_class(add_class());
        reg.register_instance(nat_instance());
        reg.register_instance(Instance::new(Name::str("Add"), mk_const("Int")));
        let all = reg.instances_for_class(&Name::str("Add"));
        assert_eq!(all.len(), 2);
    }
    #[test]
    fn test_total_method_impls() {
        let mut reg = TypeClassRegistry::new();
        reg.register_class(add_class());
        let mut inst = nat_instance();
        inst.add_method_impl("add", mk_const("Nat.add"));
        reg.register_instance(inst);
        assert_eq!(total_method_impls(&reg), 1);
    }
}
/// Build the full class hierarchy graph from a registry.
///
/// Returns a list of all parent–child edges discovered by inspecting
/// every registered class's `super_classes` list.
pub fn build_class_hierarchy(registry: &TypeClassRegistry) -> Vec<ClassEdge> {
    let mut edges = Vec::new();
    for cls in registry.classes.values() {
        for sup in &cls.super_classes {
            edges.push(ClassEdge::new(sup.clone(), cls.name.clone()));
        }
    }
    edges
}
/// Check whether `ancestor` is an ancestor of `descendant` in the class hierarchy.
pub fn is_ancestor(ancestor: &Name, descendant: &Name, registry: &TypeClassRegistry) -> bool {
    let supers = transitive_supers(descendant, registry);
    supers.contains(ancestor)
}
/// Find all instances that are complete (implement every method declared by the class).
pub fn complete_instances(registry: &TypeClassRegistry) -> Vec<&Instance> {
    registry
        .instances
        .iter()
        .filter(|inst| instance_is_complete(inst, registry))
        .collect()
}
/// Validate that all instances in the registry reference known classes.
pub fn validate_registry(registry: &TypeClassRegistry) -> Vec<String> {
    let mut errors = Vec::new();
    for inst in &registry.instances {
        if !registry.is_class(&inst.class) {
            errors.push(format!("Instance references unknown class: {}", inst.class));
        }
    }
    for cls in registry.classes.values() {
        for sup in &cls.super_classes {
            if !registry.is_class(sup) {
                errors.push(format!(
                    "Class {} references unknown super-class: {}",
                    cls.name, sup
                ));
            }
        }
    }
    errors
}
/// Check whether a class exists and has at least one registered instance.
pub fn class_has_instance(class: &Name, registry: &TypeClassRegistry) -> bool {
    registry.instances_for_class(class).iter().any(|_| true)
}
/// Count the number of classes with zero registered instances.
pub fn classes_without_instances(registry: &TypeClassRegistry) -> usize {
    registry
        .classes
        .keys()
        .filter(|name| {
            let n = Name::str(name.as_str());
            registry.instances_for_class(&n).is_empty()
        })
        .count()
}
/// Return method names shared between two classes.
pub fn shared_methods(a: &TypeClass, b: &TypeClass) -> Vec<Name> {
    a.method_names()
        .filter(|n| b.find_method(n).is_some())
        .cloned()
        .collect()
}
/// Return all (class_name, method_name) pairs in the registry.
pub fn all_method_pairs(registry: &TypeClassRegistry) -> Vec<(Name, Name)> {
    let mut pairs = Vec::new();
    for cls in registry.classes.values() {
        for m in &cls.methods {
            pairs.push((cls.name.clone(), m.name.clone()));
        }
    }
    pairs
}
/// A trait object for custom instance resolution strategies.
///
/// Implementors can supply alternative instance lookup logic.
pub trait InstanceResolver: std::fmt::Debug {
    /// Attempt to resolve an instance for `class` applied to `ty`.
    fn resolve(&self, class: &Name, ty: &Expr) -> Option<Instance>;
    /// Name of this resolver strategy.
    fn name(&self) -> &'static str;
}
/// Check whether two instances are compatible (same class, overlapping params).
#[allow(dead_code)]
pub fn instances_compatible(a: &Instance, b: &Instance) -> bool {
    a.class == b.class
}
/// Check whether an instance subsumes another (has a more general pattern).
///
/// For now this is a simple syntactic check.
#[allow(dead_code)]
pub fn instance_subsumes(general: &Instance, specific: &Instance) -> bool {
    if general.class != specific.class {
        return false;
    }
    general.ty == specific.ty || matches!(general.ty, Expr::Sort(_))
}
/// Return a human-readable summary of a `TypeClass`.
#[allow(dead_code)]
pub fn typeclass_summary(cls: &TypeClass) -> String {
    let params: Vec<String> = cls.params.iter().map(|p| p.to_string()).collect();
    let methods: Vec<String> = cls.methods.iter().map(|m| m.name.to_string()).collect();
    format!(
        "class {} ({}) {{ {} }}",
        cls.name,
        params.join(", "),
        methods.join(", ")
    )
}
/// Return a human-readable summary of an `Instance`.
#[allow(dead_code)]
pub fn instance_summary(inst: &Instance) -> String {
    let n = inst
        .instance_name
        .as_ref()
        .map(|n| n.to_string())
        .unwrap_or_else(|| "_anon".to_string());
    format!("instance {} : {}", n, inst.class)
}
#[cfg(test)]
mod typeclass_extra_tests {
    use super::*;
    use crate::Level;
    fn mk_sort() -> Expr {
        Expr::Sort(Level::zero())
    }
    fn mk_name(s: &str) -> Name {
        Name::str(s)
    }
    #[test]
    fn test_typeclass_stats_hit_rate_empty() {
        let s = TypeClassStats::new();
        assert_eq!(s.hit_rate(), 1.0);
    }
    #[test]
    fn test_typeclass_stats_hit_rate() {
        let mut s = TypeClassStats {
            total_lookups: 10,
            cache_hits: 8,
            ..Default::default()
        };
        assert!((s.hit_rate() - 0.8).abs() < 1e-10);
        let s2 = TypeClassStats {
            cache_hits: 2,
            total_lookups: 2,
            ..Default::default()
        };
        s.merge(&s2);
        assert_eq!(s.cache_hits, 10);
    }
    #[test]
    fn test_layered_registry_push_pop() {
        let mut reg = LayeredTypeClassRegistry::new();
        assert_eq!(reg.depth(), 0);
        reg.push_layer();
        assert_eq!(reg.depth(), 1);
        reg.pop_layer();
        assert_eq!(reg.depth(), 0);
    }
    #[test]
    fn test_layered_registry_add_instance_top() {
        let mut reg = LayeredTypeClassRegistry::new();
        reg.push_layer();
        let inst = Instance::named(mk_name("Add"), mk_sort(), mk_name("MyInst"));
        reg.add_instance(inst.clone());
        assert_eq!(reg.total_instances(), 1);
        reg.pop_layer();
        assert_eq!(reg.total_instances(), 0);
    }
    #[test]
    fn test_layered_registry_find_global() {
        let mut reg = LayeredTypeClassRegistry::new();
        let inst = Instance::named(
            mk_name("Add"),
            Expr::Const(mk_name("Nat"), vec![]),
            mk_name("NatAdd"),
        );
        reg.add_instance(inst);
        let found = reg.find_instance(&mk_name("Add"), &Expr::Const(mk_name("Nat"), vec![]));
        assert!(found.is_some());
    }
    #[test]
    fn test_instance_priority_ordering() {
        assert!(InstancePriority::Low < InstancePriority::Normal);
        assert!(InstancePriority::Normal < InstancePriority::High);
        assert!(InstancePriority::High < InstancePriority::Forced);
    }
    #[test]
    fn test_instance_priority_from_u32() {
        assert_eq!(InstancePriority::from_u32(0), InstancePriority::Low);
        assert_eq!(InstancePriority::from_u32(100), InstancePriority::Normal);
        assert_eq!(InstancePriority::from_u32(200), InstancePriority::High);
        assert_eq!(InstancePriority::from_u32(999), InstancePriority::Forced);
    }
    #[test]
    fn test_method_impl_custom() {
        let m = MethodImpl::custom(mk_name("add"), mk_sort());
        assert!(!m.is_default);
    }
    #[test]
    fn test_method_impl_default() {
        let m = MethodImpl::default_impl(mk_name("mul"), mk_sort());
        assert!(m.is_default);
    }
    #[test]
    fn test_instance_impl_add_get() {
        let mut ii = InstanceImpl::new();
        ii.add(MethodImpl::custom(mk_name("add"), mk_sort()));
        ii.add(MethodImpl::default_impl(mk_name("mul"), mk_sort()));
        assert_eq!(ii.len(), 2);
        assert!(ii.get(&mk_name("add")).is_some());
        assert!(ii.get(&mk_name("div")).is_none());
        assert_eq!(ii.count_defaults(), 1);
    }
    #[test]
    fn test_instances_compatible() {
        let make_inst = |cls: &str| Instance::new(mk_name(cls), mk_sort());
        let a = make_inst("Add");
        let b = make_inst("Add");
        let c = make_inst("Mul");
        assert!(instances_compatible(&a, &b));
        assert!(!instances_compatible(&a, &c));
    }
    #[test]
    fn test_typeclass_summary() {
        let cls = TypeClass {
            name: mk_name("Add"),
            params: vec![mk_name("α")],
            ty: mk_sort(),
            methods: vec![Method::new(mk_name("add"), mk_sort(), 0)],
            super_classes: vec![],
            is_prop: false,
        };
        let s = typeclass_summary(&cls);
        assert!(s.contains("Add"));
        assert!(s.contains("add"));
    }
    #[test]
    fn test_null_resolver() {
        let r = NullResolver;
        assert_eq!(r.name(), "null");
        assert!(r.resolve(&mk_name("Add"), &mk_sort()).is_none());
    }
    #[test]
    fn test_instance_subsumes_empty_params() {
        let general = Instance::new(mk_name("Eq"), Expr::Sort(crate::Level::zero()));
        let specific = Instance::new(mk_name("Eq"), mk_sort());
        assert!(instance_subsumes(&general, &specific));
    }
}
#[cfg(test)]
mod tests_padding_infra {
    use super::*;
    #[test]
    fn test_stat_summary() {
        let mut ss = StatSummary::new();
        ss.record(10.0);
        ss.record(20.0);
        ss.record(30.0);
        assert_eq!(ss.count(), 3);
        assert!((ss.mean().expect("mean should succeed") - 20.0).abs() < 1e-9);
        assert_eq!(ss.min().expect("min should succeed") as i64, 10);
        assert_eq!(ss.max().expect("max should succeed") as i64, 30);
    }
    #[test]
    fn test_transform_stat() {
        let mut ts = TransformStat::new();
        ts.record_before(100.0);
        ts.record_after(80.0);
        let ratio = ts.mean_ratio().expect("ratio should be present");
        assert!((ratio - 0.8).abs() < 1e-9);
    }
    #[test]
    fn test_small_map() {
        let mut m: SmallMap<u32, &str> = SmallMap::new();
        m.insert(3, "three");
        m.insert(1, "one");
        m.insert(2, "two");
        assert_eq!(m.get(&2), Some(&"two"));
        assert_eq!(m.len(), 3);
        let keys = m.keys();
        assert_eq!(*keys[0], 1);
        assert_eq!(*keys[2], 3);
    }
    #[test]
    fn test_label_set() {
        let mut ls = LabelSet::new();
        ls.add("foo");
        ls.add("bar");
        ls.add("foo");
        assert_eq!(ls.count(), 2);
        assert!(ls.has("bar"));
        assert!(!ls.has("baz"));
    }
    #[test]
    fn test_config_node() {
        let mut root = ConfigNode::section("root");
        let child = ConfigNode::leaf("key", "value");
        root.add_child(child);
        assert_eq!(root.num_children(), 1);
    }
    #[test]
    fn test_versioned_record() {
        let mut vr = VersionedRecord::new(0u32);
        vr.update(1);
        vr.update(2);
        assert_eq!(*vr.current(), 2);
        assert_eq!(vr.version(), 2);
        assert!(vr.has_history());
        assert_eq!(*vr.at_version(0).expect("value should be present"), 0);
    }
    #[test]
    fn test_simple_dag() {
        let mut dag = SimpleDag::new(4);
        dag.add_edge(0, 1);
        dag.add_edge(1, 2);
        dag.add_edge(2, 3);
        assert!(dag.can_reach(0, 3));
        assert!(!dag.can_reach(3, 0));
        let order = dag.topological_sort().expect("order should be present");
        assert_eq!(order, vec![0, 1, 2, 3]);
    }
    #[test]
    fn test_focus_stack() {
        let mut fs: FocusStack<&str> = FocusStack::new();
        fs.focus("a");
        fs.focus("b");
        assert_eq!(fs.current(), Some(&"b"));
        assert_eq!(fs.depth(), 2);
        fs.blur();
        assert_eq!(fs.current(), Some(&"a"));
    }
}
#[cfg(test)]
mod tests_extra_iterators {
    use super::*;
    #[test]
    fn test_window_iterator() {
        let data = vec![1u32, 2, 3, 4, 5];
        let windows: Vec<_> = WindowIterator::new(&data, 3).collect();
        assert_eq!(windows.len(), 3);
        assert_eq!(windows[0], &[1, 2, 3]);
        assert_eq!(windows[2], &[3, 4, 5]);
    }
    #[test]
    fn test_non_empty_vec() {
        let mut nev = NonEmptyVec::singleton(10u32);
        nev.push(20);
        nev.push(30);
        assert_eq!(nev.len(), 3);
        assert_eq!(*nev.first(), 10);
        assert_eq!(*nev.last(), 30);
    }
}
#[cfg(test)]
mod tests_padding2 {
    use super::*;
    #[test]
    fn test_sliding_sum() {
        let mut ss = SlidingSum::new(3);
        ss.push(1.0);
        ss.push(2.0);
        ss.push(3.0);
        assert!((ss.sum() - 6.0).abs() < 1e-9);
        ss.push(4.0);
        assert!((ss.sum() - 9.0).abs() < 1e-9);
        assert_eq!(ss.count(), 3);
    }
    #[test]
    fn test_path_buf() {
        let mut pb = PathBuf::new();
        pb.push("src");
        pb.push("main");
        assert_eq!(pb.as_str(), "src/main");
        assert_eq!(pb.depth(), 2);
        pb.pop();
        assert_eq!(pb.as_str(), "src");
    }
    #[test]
    fn test_string_pool() {
        let mut pool = StringPool::new();
        let s = pool.take();
        assert!(s.is_empty());
        pool.give("hello".to_string());
        let s2 = pool.take();
        assert!(s2.is_empty());
        assert_eq!(pool.free_count(), 0);
    }
    #[test]
    fn test_transitive_closure() {
        let mut tc = TransitiveClosure::new(4);
        tc.add_edge(0, 1);
        tc.add_edge(1, 2);
        tc.add_edge(2, 3);
        assert!(tc.can_reach(0, 3));
        assert!(!tc.can_reach(3, 0));
        let r = tc.reachable_from(0);
        assert_eq!(r.len(), 4);
    }
    #[test]
    fn test_token_bucket() {
        let mut tb = TokenBucket::new(100, 10);
        assert_eq!(tb.available(), 100);
        assert!(tb.try_consume(50));
        assert_eq!(tb.available(), 50);
        assert!(!tb.try_consume(60));
        assert_eq!(tb.capacity(), 100);
    }
    #[test]
    fn test_rewrite_rule_set() {
        let mut rrs = RewriteRuleSet::new();
        rrs.add(RewriteRule::unconditional(
            "beta",
            "App(Lam(x, b), v)",
            "b[x:=v]",
        ));
        rrs.add(RewriteRule::conditional("comm", "a + b", "b + a"));
        assert_eq!(rrs.len(), 2);
        assert_eq!(rrs.unconditional_rules().len(), 1);
        assert_eq!(rrs.conditional_rules().len(), 1);
        assert!(rrs.get("beta").is_some());
        let disp = rrs
            .get("beta")
            .expect("element at \'beta\' should exist")
            .display();
        assert!(disp.contains("→"));
    }
}
#[cfg(test)]
mod tests_padding3 {
    use super::*;
    #[test]
    fn test_decision_node() {
        let tree = DecisionNode::Branch {
            key: "x".into(),
            val: "1".into(),
            yes_branch: Box::new(DecisionNode::Leaf("yes".into())),
            no_branch: Box::new(DecisionNode::Leaf("no".into())),
        };
        let mut ctx = std::collections::HashMap::new();
        ctx.insert("x".into(), "1".into());
        assert_eq!(tree.evaluate(&ctx), "yes");
        ctx.insert("x".into(), "2".into());
        assert_eq!(tree.evaluate(&ctx), "no");
        assert_eq!(tree.depth(), 1);
    }
    #[test]
    fn test_flat_substitution() {
        let mut sub = FlatSubstitution::new();
        sub.add("foo", "bar");
        sub.add("baz", "qux");
        assert_eq!(sub.apply("foo and baz"), "bar and qux");
        assert_eq!(sub.len(), 2);
    }
    #[test]
    fn test_stopwatch() {
        let mut sw = Stopwatch::start();
        sw.split();
        sw.split();
        assert_eq!(sw.num_splits(), 2);
        assert!(sw.elapsed_ms() >= 0.0);
        for &s in sw.splits() {
            assert!(s >= 0.0);
        }
    }
    #[test]
    fn test_either2() {
        let e: Either2<i32, &str> = Either2::First(42);
        assert!(e.is_first());
        let mapped = e.map_first(|x| x * 2);
        assert_eq!(mapped.first(), Some(84));
        let e2: Either2<i32, &str> = Either2::Second("hello");
        assert!(e2.is_second());
        assert_eq!(e2.second(), Some("hello"));
    }
    #[test]
    fn test_write_once() {
        let wo: WriteOnce<u32> = WriteOnce::new();
        assert!(!wo.is_written());
        assert!(wo.write(42));
        assert!(!wo.write(99));
        assert_eq!(wo.read(), Some(42));
    }
    #[test]
    fn test_sparse_vec() {
        let mut sv: SparseVec<i32> = SparseVec::new(100);
        sv.set(5, 10);
        sv.set(50, 20);
        assert_eq!(*sv.get(5), 10);
        assert_eq!(*sv.get(50), 20);
        assert_eq!(*sv.get(0), 0);
        assert_eq!(sv.nnz(), 2);
        sv.set(5, 0);
        assert_eq!(sv.nnz(), 1);
    }
    #[test]
    fn test_stack_calc() {
        let mut calc = StackCalc::new();
        calc.push(3);
        calc.push(4);
        calc.add();
        assert_eq!(calc.peek(), Some(7));
        calc.push(2);
        calc.mul();
        assert_eq!(calc.peek(), Some(14));
    }
}
