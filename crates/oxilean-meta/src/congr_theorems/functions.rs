//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    CongrArgKind, CongrBuilder, CongrClosure, CongrStats, CongrTheoremRegistry,
    CongrTheoremsAnalysisPass, CongrTheoremsConfig, CongrTheoremsConfigValue,
    CongrTheoremsDiagnostics, CongrTheoremsDiff, CongrTheoremsExtConfig2500,
    CongrTheoremsExtConfigVal2500, CongrTheoremsExtDiag2500, CongrTheoremsExtDiff2500,
    CongrTheoremsExtPass2500, CongrTheoremsExtPipeline2500, CongrTheoremsExtResult2500,
    CongrTheoremsPipeline, CongrTheoremsResult, CongrThmsBuilder, CongrThmsCounterMap,
    CongrThmsExtMap, CongrThmsExtUtil, CongrThmsStateMachine, CongrThmsWindow, CongrThmsWorkQueue,
    MetaCongrTheorem,
};
use oxilean_kernel::{Expr, Level, Name};
use std::collections::HashMap;

/// Generate a simple congruence theorem.
pub fn mk_simple_congr(fn_name: Name, num_args: u32, arg_types: &[Expr]) -> MetaCongrTheorem {
    let arg_kinds = vec![CongrArgKind::Eq; num_args as usize];
    let result_ty = build_congr_type(&fn_name, num_args, arg_types, &arg_kinds);
    MetaCongrTheorem {
        fn_name,
        num_args,
        arg_kinds,
        ty: result_ty,
        proof: None,
    }
}
/// Generate a congruence theorem with some fixed arguments.
pub fn mk_congr_with_fixed(
    fn_name: Name,
    arg_kinds: Vec<CongrArgKind>,
    arg_types: &[Expr],
) -> MetaCongrTheorem {
    let num_args = arg_kinds.len() as u32;
    let result_ty = build_congr_type(&fn_name, num_args, arg_types, &arg_kinds);
    MetaCongrTheorem {
        fn_name,
        num_args,
        arg_kinds,
        ty: result_ty,
        proof: None,
    }
}
/// Generate a heterogeneous congruence theorem.
pub fn mk_heq_congr(fn_name: Name, num_args: u32, arg_types: &[Expr]) -> MetaCongrTheorem {
    let arg_kinds = vec![CongrArgKind::HEq; num_args as usize];
    let result_ty = build_congr_type(&fn_name, num_args, arg_types, &arg_kinds);
    MetaCongrTheorem {
        fn_name,
        num_args,
        arg_kinds,
        ty: result_ty,
        proof: None,
    }
}
/// Generate a mixed congruence theorem from a specification.
pub fn mk_mixed_congr(fn_name: &Name, spec: &[(CongrArgKind, Expr)]) -> MetaCongrTheorem {
    let arg_kinds: Vec<CongrArgKind> = spec.iter().map(|(k, _)| *k).collect();
    let arg_types: Vec<Expr> = spec.iter().map(|(_, t)| t.clone()).collect();
    let num_args = arg_kinds.len() as u32;
    let result_ty = build_congr_type(fn_name, num_args, &arg_types, &arg_kinds);
    MetaCongrTheorem {
        fn_name: fn_name.clone(),
        num_args,
        arg_kinds,
        ty: result_ty,
        proof: None,
    }
}
pub(super) fn build_congr_type(
    fn_name: &Name,
    num_args: u32,
    _arg_types: &[Expr],
    arg_kinds: &[CongrArgKind],
) -> Expr {
    let sort = Expr::Sort(Level::zero());
    let fn_const = Expr::Const(fn_name.clone(), vec![]);
    let mut lhs = fn_const.clone();
    let mut rhs = fn_const;
    for i in 0..num_args {
        match arg_kinds.get(i as usize) {
            Some(CongrArgKind::Fixed) => {
                let arg = Expr::BVar(i);
                lhs = Expr::App(Box::new(lhs), Box::new(arg.clone()));
                rhs = Expr::App(Box::new(rhs), Box::new(arg));
            }
            _ => {
                let a = Expr::BVar(i * 2);
                let b = Expr::BVar(i * 2 + 1);
                lhs = Expr::App(Box::new(lhs), Box::new(a));
                rhs = Expr::App(Box::new(rhs), Box::new(b));
            }
        }
    }
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Eq"), vec![Level::zero()])),
                Box::new(sort),
            )),
            Box::new(lhs),
        )),
        Box::new(rhs),
    )
}
/// Count the number of equality subgoals.
pub fn num_eq_goals(theorem: &MetaCongrTheorem) -> u32 {
    theorem
        .arg_kinds
        .iter()
        .filter(|k| matches!(k, CongrArgKind::Eq | CongrArgKind::HEq))
        .count() as u32
}
/// Count the number of fixed arguments.
pub fn num_fixed_args(theorem: &MetaCongrTheorem) -> u32 {
    theorem
        .arg_kinds
        .iter()
        .filter(|k| matches!(k, CongrArgKind::Fixed | CongrArgKind::FixedNoParam))
        .count() as u32
}
/// Check if a congruence theorem is simple (all args are Eq).
pub fn is_simple_congr(theorem: &MetaCongrTheorem) -> bool {
    theorem.arg_kinds.iter().all(|k| *k == CongrArgKind::Eq)
}
/// Check if a congruence theorem is heterogeneous.
pub fn is_heq_congr(theorem: &MetaCongrTheorem) -> bool {
    theorem.arg_kinds.iter().all(|k| *k == CongrArgKind::HEq)
}
/// Check if the theorem has any fixed arguments.
pub fn has_fixed_args(theorem: &MetaCongrTheorem) -> bool {
    theorem
        .arg_kinds
        .iter()
        .any(|k| matches!(k, CongrArgKind::Fixed | CongrArgKind::FixedNoParam))
}
/// Check if the theorem has any cast arguments.
pub fn has_cast_args(theorem: &MetaCongrTheorem) -> bool {
    theorem
        .arg_kinds
        .iter()
        .any(|k| matches!(k, CongrArgKind::Cast))
}
/// Count the number of subsingleton arguments.
pub fn num_subsingleton_args(theorem: &MetaCongrTheorem) -> u32 {
    theorem
        .arg_kinds
        .iter()
        .filter(|k| matches!(k, CongrArgKind::Subsingle))
        .count() as u32
}
/// Get a summary string describing a congruence theorem.
pub fn describe_congr(theorem: &MetaCongrTheorem) -> String {
    let kinds: Vec<&str> = theorem
        .arg_kinds
        .iter()
        .map(|k| match k {
            CongrArgKind::Fixed => "fixed",
            CongrArgKind::Eq => "eq",
            CongrArgKind::HEq => "heq",
            CongrArgKind::Cast => "cast",
            CongrArgKind::Subsingle => "subsingleton",
            CongrArgKind::FixedNoParam => "fixed_no_param",
        })
        .collect();
    format!(
        "{}/{}: [{}]",
        theorem.fn_name,
        theorem.num_args,
        kinds.join(", ")
    )
}
/// Generate default congruence theorems for common standard library functions.
pub fn default_congr_registry() -> CongrTheoremRegistry {
    let mut registry = CongrTheoremRegistry::new();
    registry.register(mk_simple_congr(Name::str("Nat.add"), 2, &[]));
    registry.register(mk_simple_congr(Name::str("Nat.mul"), 2, &[]));
    registry.register(mk_simple_congr(Name::str("Nat.sub"), 2, &[]));
    registry.register(mk_congr_with_fixed(
        Name::str("List.map"),
        vec![CongrArgKind::Fixed, CongrArgKind::Eq],
        &[],
    ));
    registry.register(mk_simple_congr(Name::str("List.append"), 2, &[]));
    registry.register(mk_simple_congr(Name::str("List.length"), 1, &[]));
    registry.register(mk_simple_congr(Name::str("And"), 2, &[]));
    registry.register(mk_simple_congr(Name::str("Or"), 2, &[]));
    registry
}
/// Validate a congruence theorem.
pub fn validate_congr(theorem: &MetaCongrTheorem) -> Result<(), String> {
    if theorem.arg_kinds.len() != theorem.num_args as usize {
        return Err(format!(
            "arg_kinds length {} does not match num_args {}",
            theorem.arg_kinds.len(),
            theorem.num_args
        ));
    }
    if theorem.fn_name.to_string().is_empty() {
        return Err("Function name is empty".to_string());
    }
    Ok(())
}
/// Compute the weight of a congruence theorem.
pub fn congr_weight(theorem: &MetaCongrTheorem) -> u32 {
    theorem
        .arg_kinds
        .iter()
        .map(|k| match k {
            CongrArgKind::Fixed => 0,
            CongrArgKind::FixedNoParam => 0,
            CongrArgKind::Subsingle => 1,
            CongrArgKind::Eq => 2,
            CongrArgKind::Cast => 3,
            CongrArgKind::HEq => 4,
        })
        .sum()
}
/// Filter to only keep preferred (lowest weight) theorems.
pub fn filter_preferred(theorems: &[MetaCongrTheorem]) -> Vec<&MetaCongrTheorem> {
    if theorems.is_empty() {
        return vec![];
    }
    let min_weight = theorems.iter().map(congr_weight).min().unwrap_or(0);
    theorems
        .iter()
        .filter(|t| congr_weight(t) == min_weight)
        .collect()
}
/// Specialize a congruence theorem by fixing specific argument positions.
pub fn specialize_congr(theorem: &MetaCongrTheorem, fixed_positions: &[usize]) -> MetaCongrTheorem {
    let mut new_kinds = theorem.arg_kinds.clone();
    for &pos in fixed_positions {
        if pos < new_kinds.len() && new_kinds[pos] == CongrArgKind::Eq {
            new_kinds[pos] = CongrArgKind::Fixed;
        }
    }
    MetaCongrTheorem {
        fn_name: theorem.fn_name.clone(),
        num_args: theorem.num_args,
        arg_kinds: new_kinds,
        ty: theorem.ty.clone(),
        proof: theorem.proof.clone(),
    }
}
/// Analyze a function type to suggest congruence argument kinds.
pub fn analyze_fn_type(fn_ty: &Expr) -> Vec<CongrArgKind> {
    let mut kinds = Vec::new();
    analyze_fn_type_impl(fn_ty, &mut kinds, 0);
    kinds
}
pub(super) fn analyze_fn_type_impl(ty: &Expr, kinds: &mut Vec<CongrArgKind>, _depth: u32) {
    let mut current = ty;
    while let Expr::Pi(bi, _, _, codomain) = current {
        let kind = match bi {
            oxilean_kernel::BinderInfo::Implicit
            | oxilean_kernel::BinderInfo::InstImplicit
            | oxilean_kernel::BinderInfo::StrictImplicit => CongrArgKind::Fixed,
            oxilean_kernel::BinderInfo::Default => CongrArgKind::Eq,
        };
        kinds.push(kind);
        current = codomain;
    }
}
/// Compute the explicit argument arity from a function type.
pub fn count_explicit_args(fn_ty: &Expr) -> u32 {
    analyze_fn_type(fn_ty)
        .iter()
        .filter(|k| matches!(k, CongrArgKind::Eq))
        .count() as u32
}
/// Select the best congruence theorem for a given arity.
pub fn best_congr_for_arity(
    theorems: &[MetaCongrTheorem],
    arity: u32,
) -> Option<&MetaCongrTheorem> {
    if let Some(t) = theorems.iter().find(|t| t.num_args == arity) {
        return Some(t);
    }
    theorems
        .iter()
        .min_by_key(|t| (t.num_args as i64 - arity as i64).unsigned_abs())
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::congr_theorems::*;
    fn nat_ty() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    #[test]
    fn test_mk_simple_congr() {
        let thm = mk_simple_congr(Name::str("Nat.add"), 2, &[nat_ty(), nat_ty()]);
        assert_eq!(thm.fn_name, Name::str("Nat.add"));
        assert_eq!(thm.num_args, 2);
        assert!(thm.arg_kinds.iter().all(|k| *k == CongrArgKind::Eq));
    }
    #[test]
    fn test_mk_congr_with_fixed() {
        let thm = mk_congr_with_fixed(
            Name::str("List.map"),
            vec![CongrArgKind::Fixed, CongrArgKind::Eq, CongrArgKind::Eq],
            &[nat_ty(), nat_ty(), nat_ty()],
        );
        assert_eq!(thm.num_args, 3);
        assert_eq!(thm.arg_kinds[0], CongrArgKind::Fixed);
        assert_eq!(thm.arg_kinds[1], CongrArgKind::Eq);
    }
    #[test]
    fn test_num_eq_goals() {
        let thm = mk_congr_with_fixed(
            Name::str("f"),
            vec![CongrArgKind::Fixed, CongrArgKind::Eq, CongrArgKind::HEq],
            &[],
        );
        assert_eq!(num_eq_goals(&thm), 2);
    }
    #[test]
    fn test_num_fixed_args() {
        let thm = mk_congr_with_fixed(
            Name::str("f"),
            vec![CongrArgKind::Fixed, CongrArgKind::Eq, CongrArgKind::Fixed],
            &[],
        );
        assert_eq!(num_fixed_args(&thm), 2);
    }
    #[test]
    fn test_is_simple_congr() {
        let simple = mk_simple_congr(Name::str("f"), 2, &[]);
        assert!(is_simple_congr(&simple));
        let not_simple = mk_congr_with_fixed(
            Name::str("f"),
            vec![CongrArgKind::Fixed, CongrArgKind::Eq],
            &[],
        );
        assert!(!is_simple_congr(&not_simple));
    }
    #[test]
    fn test_mk_heq_congr() {
        let thm = mk_heq_congr(Name::str("g"), 3, &[]);
        assert!(is_heq_congr(&thm));
        assert!(!is_simple_congr(&thm));
    }
    #[test]
    fn test_registry() {
        let mut registry = CongrTheoremRegistry::new();
        let thm = mk_simple_congr(Name::str("Nat.add"), 2, &[]);
        registry.register(thm);
        assert!(registry.lookup(&Name::str("Nat.add")).is_some());
        assert!(registry.lookup(&Name::str("unknown")).is_none());
    }
    #[test]
    fn test_registry_lookup_arity() {
        let mut registry = CongrTheoremRegistry::new();
        registry.register(mk_simple_congr(Name::str("f"), 2, &[]));
        registry.register(mk_simple_congr(Name::str("f"), 3, &[]));
        assert_eq!(
            registry
                .lookup_arity(&Name::str("f"), 2)
                .expect("value should be present")
                .num_args,
            2
        );
        assert_eq!(
            registry
                .lookup_arity(&Name::str("f"), 3)
                .expect("value should be present")
                .num_args,
            3
        );
        assert!(registry.lookup_arity(&Name::str("f"), 4).is_none());
    }
    #[test]
    fn test_default_registry() {
        let registry = default_congr_registry();
        assert!(!registry.is_empty());
        assert!(registry.lookup(&Name::str("Nat.add")).is_some());
        assert!(registry.lookup(&Name::str("List.map")).is_some());
    }
    #[test]
    fn test_validate_congr_ok() {
        let thm = mk_simple_congr(Name::str("f"), 2, &[]);
        assert!(validate_congr(&thm).is_ok());
    }
    #[test]
    fn test_validate_congr_bad() {
        let bad_thm = MetaCongrTheorem {
            fn_name: Name::str("g"),
            num_args: 5,
            arg_kinds: vec![CongrArgKind::Eq],
            ty: Expr::Sort(Level::zero()),
            proof: None,
        };
        assert!(validate_congr(&bad_thm).is_err());
    }
    #[test]
    fn test_congr_weight() {
        let eq_thm = mk_simple_congr(Name::str("f"), 2, &[]);
        let fixed_thm = mk_congr_with_fixed(
            Name::str("f"),
            vec![CongrArgKind::Fixed, CongrArgKind::Fixed],
            &[],
        );
        assert!(congr_weight(&eq_thm) > congr_weight(&fixed_thm));
    }
    #[test]
    fn test_specialize_congr() {
        let thm = mk_simple_congr(Name::str("f"), 3, &[]);
        let specialized = specialize_congr(&thm, &[0, 2]);
        assert_eq!(specialized.arg_kinds[0], CongrArgKind::Fixed);
        assert_eq!(specialized.arg_kinds[1], CongrArgKind::Eq);
        assert_eq!(specialized.arg_kinds[2], CongrArgKind::Fixed);
    }
    #[test]
    fn test_congr_stats() {
        let mut stats = CongrStats::new();
        assert_eq!(stats.hit_rate(), 1.0);
        stats.record_application();
        stats.record_application();
        stats.record_miss();
        assert!((stats.hit_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_congr_closure() {
        let mut closure = CongrClosure::new();
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let bool_ty = Expr::Const(Name::str("Bool"), vec![]);
        closure.add_equality(nat.clone(), bool_ty.clone());
        assert!(closure.are_congr(&nat, &bool_ty));
        assert!(closure.are_congr(&bool_ty, &nat));
        assert_eq!(closure.num_equalities(), 1);
        closure.clear();
        assert_eq!(closure.num_equalities(), 0);
    }
    #[test]
    fn test_describe_congr() {
        let thm = mk_congr_with_fixed(
            Name::str("List.map"),
            vec![CongrArgKind::Fixed, CongrArgKind::Eq],
            &[],
        );
        let desc = describe_congr(&thm);
        assert!(desc.contains("List.map"));
        assert!(desc.contains("fixed"));
        assert!(desc.contains("eq"));
    }
    #[test]
    fn test_has_fixed_args() {
        let thm = mk_simple_congr(Name::str("f"), 2, &[]);
        assert!(!has_fixed_args(&thm));
        let thm2 = mk_congr_with_fixed(
            Name::str("f"),
            vec![CongrArgKind::Fixed, CongrArgKind::Eq],
            &[],
        );
        assert!(has_fixed_args(&thm2));
    }
    #[test]
    fn test_congr_closure_add_find() {
        let mut closure = CongrClosure::new();
        let a = Expr::Sort(Level::zero());
        let b = Expr::Sort(Level::succ(Level::zero()));
        closure.add_equality(a.clone(), b.clone());
        assert!(closure.are_congr(&a, &b));
        assert!(!closure.are_congr(&a, &Expr::Const(Name::str("X"), vec![])));
    }
    #[test]
    fn test_congr_builder() {
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let thm = CongrBuilder::for_fn(Name::str("f"))
            .fixed_arg(nat.clone())
            .eq_arg(nat.clone())
            .build();
        assert_eq!(thm.num_args, 2);
        assert_eq!(thm.arg_kinds[0], CongrArgKind::Fixed);
        assert_eq!(thm.arg_kinds[1], CongrArgKind::Eq);
    }
    #[test]
    fn test_filter_preferred() {
        let eq_thm = mk_simple_congr(Name::str("f"), 2, &[]);
        let fixed_thm = mk_congr_with_fixed(
            Name::str("f"),
            vec![CongrArgKind::Fixed, CongrArgKind::Fixed],
            &[],
        );
        let thms = vec![eq_thm, fixed_thm];
        let preferred = filter_preferred(&thms);
        assert!(!preferred.is_empty());
    }
    #[test]
    fn test_registry_iter() {
        let mut registry = CongrTheoremRegistry::new();
        registry.register(mk_simple_congr(Name::str("f"), 2, &[]));
        registry.register(mk_simple_congr(Name::str("g"), 1, &[]));
        let count = registry.iter().count();
        assert_eq!(count, 2);
    }
}
/// Map a function over the arg kinds of a congruence theorem.
pub fn map_arg_kinds(
    theorem: &MetaCongrTheorem,
    f: impl Fn(CongrArgKind) -> CongrArgKind,
) -> MetaCongrTheorem {
    MetaCongrTheorem {
        fn_name: theorem.fn_name.clone(),
        num_args: theorem.num_args,
        arg_kinds: theorem.arg_kinds.iter().map(|&k| f(k)).collect(),
        ty: theorem.ty.clone(),
        proof: theorem.proof.clone(),
    }
}
/// Convert all Eq arguments to Fixed.
pub fn make_all_fixed(theorem: &MetaCongrTheorem) -> MetaCongrTheorem {
    map_arg_kinds(theorem, |k| match k {
        CongrArgKind::Eq => CongrArgKind::Fixed,
        other => other,
    })
}
/// Convert all Fixed arguments to Eq.
pub fn make_all_eq(theorem: &MetaCongrTheorem) -> MetaCongrTheorem {
    map_arg_kinds(theorem, |k| match k {
        CongrArgKind::Fixed | CongrArgKind::FixedNoParam => CongrArgKind::Eq,
        other => other,
    })
}
/// Compare two congruence theorems for equivalence of their arg kinds.
pub fn same_shape(a: &MetaCongrTheorem, b: &MetaCongrTheorem) -> bool {
    a.num_args == b.num_args && a.arg_kinds == b.arg_kinds
}
/// Combine two registries, with `other` taking precedence for duplicate entries.
pub fn merge_registries(
    base: CongrTheoremRegistry,
    other: CongrTheoremRegistry,
) -> CongrTheoremRegistry {
    let mut result = base;
    for thm in other.iter() {
        result.register(thm.clone());
    }
    result
}
/// A snapshot of the registry for inspection.
pub fn registry_snapshot(registry: &CongrTheoremRegistry) -> Vec<(String, u32)> {
    let mut snapshot: Vec<(String, u32)> = registry
        .iter()
        .map(|t| (t.fn_name.to_string(), t.num_args))
        .collect();
    snapshot.sort();
    snapshot
}
/// Check if a theorem dominates another (is at least as good for all positions).
///
/// A theorem A dominates B if every Eq position in A is also Eq in B,
/// and A has at most as many subgoals.
pub fn dominates(a: &MetaCongrTheorem, b: &MetaCongrTheorem) -> bool {
    if a.num_args != b.num_args {
        return false;
    }
    num_eq_goals(a) <= num_eq_goals(b)
}
/// Group theorems by their function name.
pub fn group_by_fn(
    theorems: &[MetaCongrTheorem],
) -> std::collections::HashMap<String, Vec<&MetaCongrTheorem>> {
    let mut groups: std::collections::HashMap<String, Vec<&MetaCongrTheorem>> =
        std::collections::HashMap::new();
    for t in theorems {
        groups.entry(t.fn_name.to_string()).or_default().push(t);
    }
    groups
}
/// Generate a unique key for a congruence theorem.
pub fn theorem_key(t: &MetaCongrTheorem) -> String {
    let kinds: Vec<&str> = t
        .arg_kinds
        .iter()
        .map(|k| match k {
            CongrArgKind::Fixed => "F",
            CongrArgKind::FixedNoParam => "N",
            CongrArgKind::Eq => "E",
            CongrArgKind::HEq => "H",
            CongrArgKind::Cast => "C",
            CongrArgKind::Subsingle => "S",
        })
        .collect();
    format!("{}:{}", t.fn_name, kinds.join(""))
}
/// Check if a registry contains a theorem with the given key.
pub fn registry_has_key(registry: &CongrTheoremRegistry, key: &str) -> bool {
    registry.iter().any(|t| theorem_key(t) == key)
}
/// Get all distinct function names in a registry.
pub fn registry_fn_names(registry: &CongrTheoremRegistry) -> Vec<String> {
    let mut names: Vec<String> = registry.iter().map(|t| t.fn_name.to_string()).collect();
    names.sort();
    names.dedup();
    names
}
/// Return the number of theorems in a registry for a given function name.
pub fn registry_count_for(registry: &CongrTheoremRegistry, name: &Name) -> usize {
    registry.lookup(name).map_or(0, |v| v.len())
}
/// Clone a theorem with a new function name.
pub fn rename_theorem(theorem: &MetaCongrTheorem, new_name: Name) -> MetaCongrTheorem {
    MetaCongrTheorem {
        fn_name: new_name,
        num_args: theorem.num_args,
        arg_kinds: theorem.arg_kinds.clone(),
        ty: theorem.ty.clone(),
        proof: theorem.proof.clone(),
    }
}
/// Pad a theorem with additional wildcard (Fixed) arguments.
pub fn pad_with_fixed(theorem: &MetaCongrTheorem, count: u32) -> MetaCongrTheorem {
    let mut new_kinds = theorem.arg_kinds.clone();
    new_kinds.extend(std::iter::repeat(CongrArgKind::Fixed).take(count as usize));
    MetaCongrTheorem {
        fn_name: theorem.fn_name.clone(),
        num_args: theorem.num_args + count,
        arg_kinds: new_kinds,
        ty: theorem.ty.clone(),
        proof: theorem.proof.clone(),
    }
}
/// Truncate a theorem to at most `max_args` arguments.
pub fn truncate_args(theorem: &MetaCongrTheorem, max_args: u32) -> MetaCongrTheorem {
    let take = (theorem.num_args.min(max_args)) as usize;
    let new_kinds = theorem.arg_kinds[..take].to_vec();
    MetaCongrTheorem {
        fn_name: theorem.fn_name.clone(),
        num_args: take as u32,
        arg_kinds: new_kinds,
        ty: theorem.ty.clone(),
        proof: theorem.proof.clone(),
    }
}
#[cfg(test)]
mod extra_tests {
    use super::*;
    use crate::congr_theorems::*;
    fn nat_ty() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    #[test]
    fn test_make_all_fixed() {
        let thm = mk_simple_congr(Name::str("f"), 3, &[]);
        let fixed = make_all_fixed(&thm);
        assert!(fixed.arg_kinds.iter().all(|k| *k == CongrArgKind::Fixed));
    }
    #[test]
    fn test_make_all_eq() {
        let thm = mk_congr_with_fixed(
            Name::str("f"),
            vec![CongrArgKind::Fixed, CongrArgKind::Fixed],
            &[],
        );
        let eq_thm = make_all_eq(&thm);
        assert!(eq_thm.arg_kinds.iter().all(|k| *k == CongrArgKind::Eq));
    }
    #[test]
    fn test_same_shape() {
        let t1 = mk_simple_congr(Name::str("f"), 2, &[]);
        let t2 = mk_simple_congr(Name::str("g"), 2, &[]);
        assert!(same_shape(&t1, &t2));
        let t3 = mk_simple_congr(Name::str("h"), 3, &[]);
        assert!(!same_shape(&t1, &t3));
    }
    #[test]
    fn test_merge_registries() {
        let mut r1 = CongrTheoremRegistry::new();
        r1.register(mk_simple_congr(Name::str("f"), 2, &[]));
        let mut r2 = CongrTheoremRegistry::new();
        r2.register(mk_simple_congr(Name::str("g"), 1, &[]));
        let merged = merge_registries(r1, r2);
        assert_eq!(merged.len(), 2);
    }
    #[test]
    fn test_registry_snapshot() {
        let mut r = CongrTheoremRegistry::new();
        r.register(mk_simple_congr(Name::str("z"), 1, &[]));
        r.register(mk_simple_congr(Name::str("a"), 2, &[]));
        let snap = registry_snapshot(&r);
        assert!(snap[0].0 <= snap[1].0);
    }
    #[test]
    fn test_dominates() {
        let t1 = mk_simple_congr(Name::str("f"), 2, &[]);
        let t2 = mk_congr_with_fixed(
            Name::str("f"),
            vec![CongrArgKind::Fixed, CongrArgKind::Eq],
            &[],
        );
        assert!(dominates(&t2, &t1));
    }
    #[test]
    fn test_group_by_fn() {
        let thms = vec![
            mk_simple_congr(Name::str("f"), 2, &[]),
            mk_simple_congr(Name::str("f"), 3, &[]),
            mk_simple_congr(Name::str("g"), 1, &[]),
        ];
        let groups = group_by_fn(&thms);
        assert_eq!(
            groups
                .get("f")
                .expect("element at \'f\' should exist")
                .len(),
            2
        );
        assert_eq!(
            groups
                .get("g")
                .expect("element at \'g\' should exist")
                .len(),
            1
        );
    }
    #[test]
    fn test_theorem_key() {
        let t = mk_congr_with_fixed(
            Name::str("f"),
            vec![CongrArgKind::Fixed, CongrArgKind::Eq],
            &[],
        );
        let key = theorem_key(&t);
        assert!(key.contains("f"));
        assert!(key.contains("FE"));
    }
    #[test]
    fn test_rename_theorem() {
        let t = mk_simple_congr(Name::str("old"), 2, &[]);
        let renamed = rename_theorem(&t, Name::str("new"));
        assert_eq!(renamed.fn_name, Name::str("new"));
        assert_eq!(renamed.num_args, 2);
    }
    #[test]
    fn test_pad_with_fixed() {
        let t = mk_simple_congr(Name::str("f"), 2, &[]);
        let padded = pad_with_fixed(&t, 2);
        assert_eq!(padded.num_args, 4);
        assert_eq!(padded.arg_kinds[2], CongrArgKind::Fixed);
        assert_eq!(padded.arg_kinds[3], CongrArgKind::Fixed);
    }
    #[test]
    fn test_truncate_args() {
        let t = mk_simple_congr(Name::str("f"), 5, &[]);
        let trunc = truncate_args(&t, 3);
        assert_eq!(trunc.num_args, 3);
        assert_eq!(trunc.arg_kinds.len(), 3);
    }
    #[test]
    fn test_registry_fn_names() {
        let mut r = CongrTheoremRegistry::new();
        r.register(mk_simple_congr(Name::str("add"), 2, &[]));
        r.register(mk_simple_congr(Name::str("mul"), 2, &[]));
        r.register(mk_simple_congr(Name::str("add"), 3, &[]));
        let names = registry_fn_names(&r);
        assert_eq!(names.len(), 2);
        assert!(names.contains(&"add".to_string()));
    }
    #[test]
    fn test_registry_count_for() {
        let mut r = CongrTheoremRegistry::new();
        r.register(mk_simple_congr(Name::str("f"), 2, &[]));
        r.register(mk_simple_congr(Name::str("f"), 3, &[]));
        assert_eq!(registry_count_for(&r, &Name::str("f")), 2);
        assert_eq!(registry_count_for(&r, &Name::str("g")), 0);
    }
}
/// Compute the "weight" of a congruence theorem for priority sorting.
///
/// Lower weight = preferred theorem. The heuristic is:
/// - More `Eq` arguments = more work to prove, weight++
/// - More `Fixed` arguments = less flexible, slight penalty
/// - `Subsingle` arguments = free, reduce weight
#[allow(dead_code)]
pub fn theorem_weight(thm: &MetaCongrTheorem) -> i32 {
    let mut w = 0i32;
    for kind in &thm.arg_kinds {
        match kind {
            CongrArgKind::Eq | CongrArgKind::HEq => w += 2,
            CongrArgKind::Fixed | CongrArgKind::FixedNoParam => w += 1,
            CongrArgKind::Cast => w += 3,
            CongrArgKind::Subsingle => w -= 1,
        }
    }
    w
}
/// Sort a list of theorems by weight (ascending).
#[allow(dead_code)]
pub fn sort_by_weight(thms: &mut [MetaCongrTheorem]) {
    thms.sort_by_key(theorem_weight);
}
/// Filter theorems that have all their argument kinds in the allowed set.
#[allow(dead_code)]
pub fn filter_by_kinds<'a>(
    thms: &'a [MetaCongrTheorem],
    allowed: &[CongrArgKind],
) -> Vec<&'a MetaCongrTheorem> {
    thms.iter()
        .filter(|t| t.arg_kinds.iter().all(|k| allowed.contains(k)))
        .collect()
}
/// Return the index of the first `Eq` argument, or `None`.
#[allow(dead_code)]
pub fn first_eq_idx(thm: &MetaCongrTheorem) -> Option<usize> {
    thm.arg_kinds
        .iter()
        .position(|k| matches!(k, CongrArgKind::Eq))
}
/// Return the index of the last `Fixed` argument, or `None`.
#[allow(dead_code)]
pub fn last_fixed_idx(thm: &MetaCongrTheorem) -> Option<usize> {
    thm.arg_kinds
        .iter()
        .rposition(|k| matches!(k, CongrArgKind::Fixed | CongrArgKind::FixedNoParam))
}
/// Count arguments of each kind.
#[allow(dead_code)]
pub fn arg_kind_histogram(thm: &MetaCongrTheorem) -> HashMap<CongrArgKind, usize> {
    let mut hist = HashMap::new();
    for kind in &thm.arg_kinds {
        *hist.entry(*kind).or_insert(0) += 1;
    }
    hist
}
/// Check whether all arguments of a theorem are `Fixed`.
#[allow(dead_code)]
pub fn is_all_fixed(thm: &MetaCongrTheorem) -> bool {
    thm.arg_kinds
        .iter()
        .all(|k| matches!(k, CongrArgKind::Fixed | CongrArgKind::FixedNoParam))
}
/// Check whether any argument is `Cast`.
#[allow(dead_code)]
pub fn has_cast_arg(thm: &MetaCongrTheorem) -> bool {
    thm.arg_kinds
        .iter()
        .any(|k| matches!(k, CongrArgKind::Cast))
}
/// Check whether any argument is an `HEq`.
#[allow(dead_code)]
pub fn has_heq_arg(thm: &MetaCongrTheorem) -> bool {
    thm.arg_kinds.iter().any(|k| matches!(k, CongrArgKind::HEq))
}
/// Normalize a theorem by replacing `HEq` with `Eq` (down-casting heterogeneous
/// equalities).
#[allow(dead_code)]
pub fn normalize_heq(thm: &MetaCongrTheorem) -> MetaCongrTheorem {
    let kinds: Vec<CongrArgKind> = thm
        .arg_kinds
        .iter()
        .map(|k| {
            if *k == CongrArgKind::HEq {
                CongrArgKind::Eq
            } else {
                *k
            }
        })
        .collect();
    MetaCongrTheorem {
        fn_name: thm.fn_name.clone(),
        num_args: thm.num_args,
        arg_kinds: kinds,
        ty: thm.ty.clone(),
        proof: thm.proof.clone(),
    }
}
/// Summarize a theorem in a human-readable one-liner.
#[allow(dead_code)]
pub fn summarize(thm: &MetaCongrTheorem) -> String {
    let kinds_str: String = thm
        .arg_kinds
        .iter()
        .map(|k| match k {
            CongrArgKind::Fixed => "F",
            CongrArgKind::Eq => "E",
            CongrArgKind::HEq => "H",
            CongrArgKind::Cast => "C",
            CongrArgKind::Subsingle => "S",
            CongrArgKind::FixedNoParam => "N",
        })
        .collect();
    format!(
        "congr[{}|{}|w={}]",
        thm.fn_name,
        kinds_str,
        theorem_weight(thm)
    )
}
#[cfg(test)]
mod extra_congr_tests {
    use super::*;
    use crate::congr_theorems::*;
    fn mk_thm(name: &str, kinds: Vec<CongrArgKind>) -> MetaCongrTheorem {
        let n = kinds.len() as u32;
        MetaCongrTheorem {
            fn_name: Name::str(name),
            num_args: n,
            arg_kinds: kinds,
            ty: Expr::Sort(Level::zero()),
            proof: None,
        }
    }
    #[test]
    fn test_theorem_weight_eq_heavy() {
        let t = mk_thm("f", vec![CongrArgKind::Eq, CongrArgKind::Eq]);
        assert_eq!(theorem_weight(&t), 4);
    }
    #[test]
    fn test_theorem_weight_fixed() {
        let t = mk_thm("f", vec![CongrArgKind::Fixed]);
        assert_eq!(theorem_weight(&t), 1);
    }
    #[test]
    fn test_theorem_weight_subsingle_reduces() {
        let t = mk_thm("f", vec![CongrArgKind::Subsingle, CongrArgKind::Eq]);
        assert_eq!(theorem_weight(&t), 1);
    }
    #[test]
    fn test_sort_by_weight() {
        let mut thms = vec![
            mk_thm("f", vec![CongrArgKind::Eq, CongrArgKind::Eq]),
            mk_thm("g", vec![CongrArgKind::Fixed]),
            mk_thm("h", vec![CongrArgKind::Subsingle]),
        ];
        sort_by_weight(&mut thms);
        assert!(theorem_weight(&thms[0]) <= theorem_weight(&thms[1]));
    }
    #[test]
    fn test_filter_by_kinds() {
        let thms = vec![
            mk_thm("f", vec![CongrArgKind::Eq]),
            mk_thm("g", vec![CongrArgKind::Fixed]),
            mk_thm("h", vec![CongrArgKind::Cast]),
        ];
        let allowed = vec![CongrArgKind::Eq, CongrArgKind::Fixed];
        let filtered = filter_by_kinds(&thms, &allowed);
        assert_eq!(filtered.len(), 2);
    }
    #[test]
    fn test_first_eq_idx() {
        let t = mk_thm("f", vec![CongrArgKind::Fixed, CongrArgKind::Eq]);
        assert_eq!(first_eq_idx(&t), Some(1));
    }
    #[test]
    fn test_last_fixed_idx() {
        let t = mk_thm(
            "f",
            vec![CongrArgKind::Fixed, CongrArgKind::Eq, CongrArgKind::Fixed],
        );
        assert_eq!(last_fixed_idx(&t), Some(2));
    }
    #[test]
    fn test_arg_kind_histogram() {
        let t = mk_thm(
            "f",
            vec![CongrArgKind::Eq, CongrArgKind::Eq, CongrArgKind::Fixed],
        );
        let hist = arg_kind_histogram(&t);
        assert_eq!(
            *hist
                .get(&CongrArgKind::Eq)
                .expect("element at &CongrArgKind::Eq should exist"),
            2
        );
        assert_eq!(
            *hist
                .get(&CongrArgKind::Fixed)
                .expect("element at &CongrArgKind::Fixed should exist"),
            1
        );
    }
    #[test]
    fn test_is_all_fixed() {
        let t = mk_thm("f", vec![CongrArgKind::Fixed, CongrArgKind::Fixed]);
        assert!(is_all_fixed(&t));
        let t2 = mk_thm("g", vec![CongrArgKind::Fixed, CongrArgKind::Eq]);
        assert!(!is_all_fixed(&t2));
    }
    #[test]
    fn test_has_cast_arg() {
        let t = mk_thm("f", vec![CongrArgKind::Eq, CongrArgKind::Cast]);
        assert!(has_cast_arg(&t));
        assert!(!has_cast_arg(&mk_thm("g", vec![CongrArgKind::Eq])));
    }
    #[test]
    fn test_has_heq_arg() {
        let t = mk_thm("f", vec![CongrArgKind::HEq]);
        assert!(has_heq_arg(&t));
    }
    #[test]
    fn test_normalize_heq() {
        let t = mk_thm("f", vec![CongrArgKind::HEq, CongrArgKind::Fixed]);
        let n = normalize_heq(&t);
        assert_eq!(n.arg_kinds[0], CongrArgKind::Eq);
        assert_eq!(n.arg_kinds[1], CongrArgKind::Fixed);
    }
    #[test]
    fn test_summarize() {
        let t = mk_thm("add", vec![CongrArgKind::Eq, CongrArgKind::Fixed]);
        let s = summarize(&t);
        assert!(s.contains("add"));
        assert!(s.contains("EF"));
    }
    #[test]
    fn test_theorem_weight_cast() {
        let t = mk_thm("f", vec![CongrArgKind::Cast]);
        assert_eq!(theorem_weight(&t), 3);
    }
    #[test]
    fn test_filter_by_kinds_empty() {
        let thms: Vec<MetaCongrTheorem> = vec![];
        let filtered = filter_by_kinds(&thms, &[CongrArgKind::Eq]);
        assert!(filtered.is_empty());
    }
}
#[cfg(test)]
mod congrthms_ext2_tests {
    use super::*;
    use crate::congr_theorems::*;
    #[test]
    fn test_congrthms_ext_util_basic() {
        let mut u = CongrThmsExtUtil::new("test");
        u.push(10);
        u.push(20);
        assert_eq!(u.sum(), 30);
        assert_eq!(u.len(), 2);
    }
    #[test]
    fn test_congrthms_ext_util_min_max() {
        let mut u = CongrThmsExtUtil::new("mm");
        u.push(5);
        u.push(1);
        u.push(9);
        assert_eq!(u.min_val(), Some(1));
        assert_eq!(u.max_val(), Some(9));
    }
    #[test]
    fn test_congrthms_ext_util_flags() {
        let mut u = CongrThmsExtUtil::new("flags");
        u.set_flag(3);
        assert!(u.has_flag(3));
        assert!(!u.has_flag(2));
    }
    #[test]
    fn test_congrthms_ext_util_pop() {
        let mut u = CongrThmsExtUtil::new("pop");
        u.push(42);
        assert_eq!(u.pop(), Some(42));
        assert!(u.is_empty());
    }
    #[test]
    fn test_congrthms_ext_map_basic() {
        let mut m: CongrThmsExtMap<i32> = CongrThmsExtMap::new();
        m.insert("key", 42);
        assert_eq!(m.get("key"), Some(&42));
        assert!(m.contains("key"));
        assert!(!m.contains("other"));
    }
    #[test]
    fn test_congrthms_ext_map_get_or_default() {
        let mut m: CongrThmsExtMap<i32> = CongrThmsExtMap::new();
        m.insert("k", 5);
        assert_eq!(m.get_or_default("k"), 5);
        assert_eq!(m.get_or_default("missing"), 0);
    }
    #[test]
    fn test_congrthms_ext_map_keys_sorted() {
        let mut m: CongrThmsExtMap<i32> = CongrThmsExtMap::new();
        m.insert("z", 1);
        m.insert("a", 2);
        m.insert("m", 3);
        let keys = m.keys_sorted();
        assert_eq!(keys[0].as_str(), "a");
        assert_eq!(keys[2].as_str(), "z");
    }
    #[test]
    fn test_congrthms_window_mean() {
        let mut w = CongrThmsWindow::new(3);
        w.push(1.0);
        w.push(2.0);
        w.push(3.0);
        assert!((w.mean() - 2.0).abs() < 1e-10);
    }
    #[test]
    fn test_congrthms_window_evict() {
        let mut w = CongrThmsWindow::new(2);
        w.push(10.0);
        w.push(20.0);
        w.push(30.0);
        assert_eq!(w.len(), 2);
        assert!((w.mean() - 25.0).abs() < 1e-10);
    }
    #[test]
    fn test_congrthms_window_std_dev() {
        let mut w = CongrThmsWindow::new(10);
        for i in 0..10 {
            w.push(i as f64);
        }
        assert!(w.std_dev() > 0.0);
    }
    #[test]
    fn test_congrthms_builder_basic() {
        let b = CongrThmsBuilder::new("test")
            .add_item("a")
            .add_item("b")
            .set_config("key", "val");
        assert_eq!(b.item_count(), 2);
        assert!(b.has_config("key"));
        assert_eq!(b.get_config("key"), Some("val"));
    }
    #[test]
    fn test_congrthms_builder_summary() {
        let b = CongrThmsBuilder::new("suite").add_item("x");
        let s = b.build_summary();
        assert!(s.contains("suite"));
    }
    #[test]
    fn test_congrthms_state_machine_start() {
        let mut sm = CongrThmsStateMachine::new();
        assert!(sm.start());
        assert!(sm.state.is_running());
    }
    #[test]
    fn test_congrthms_state_machine_complete() {
        let mut sm = CongrThmsStateMachine::new();
        sm.start();
        sm.complete();
        assert!(sm.state.is_terminal());
    }
    #[test]
    fn test_congrthms_state_machine_fail() {
        let mut sm = CongrThmsStateMachine::new();
        sm.fail("oops");
        assert!(sm.state.is_terminal());
        assert_eq!(sm.state.error_msg(), Some("oops"));
    }
    #[test]
    fn test_congrthms_state_machine_no_transition_after_terminal() {
        let mut sm = CongrThmsStateMachine::new();
        sm.complete();
        assert!(!sm.start());
    }
    #[test]
    fn test_congrthms_work_queue_basic() {
        let mut wq = CongrThmsWorkQueue::new(10);
        wq.enqueue("task1".to_string());
        wq.enqueue("task2".to_string());
        assert_eq!(wq.pending_count(), 2);
        let t = wq.dequeue();
        assert_eq!(t, Some("task1".to_string()));
        assert_eq!(wq.processed_count(), 1);
    }
    #[test]
    fn test_congrthms_work_queue_capacity() {
        let mut wq = CongrThmsWorkQueue::new(2);
        wq.enqueue("a".to_string());
        wq.enqueue("b".to_string());
        assert!(wq.is_full());
        assert!(!wq.enqueue("c".to_string()));
    }
    #[test]
    fn test_congrthms_counter_map_basic() {
        let mut cm = CongrThmsCounterMap::new();
        cm.increment("apple");
        cm.increment("apple");
        cm.increment("banana");
        assert_eq!(cm.count("apple"), 2);
        assert_eq!(cm.count("banana"), 1);
        assert_eq!(cm.num_unique(), 2);
    }
    #[test]
    fn test_congrthms_counter_map_frequency() {
        let mut cm = CongrThmsCounterMap::new();
        cm.increment("a");
        cm.increment("a");
        cm.increment("b");
        assert!((cm.frequency("a") - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_congrthms_counter_map_most_common() {
        let mut cm = CongrThmsCounterMap::new();
        cm.increment("x");
        cm.increment("y");
        cm.increment("x");
        let (k, v) = cm.most_common().expect("most_common should succeed");
        assert_eq!(k.as_str(), "x");
        assert_eq!(v, 2);
    }
}
#[cfg(test)]
mod congrtheorems_analysis_tests {
    use super::*;
    use crate::congr_theorems::*;
    #[test]
    fn test_congrtheorems_result_ok() {
        let r = CongrTheoremsResult::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_congrtheorems_result_err() {
        let r = CongrTheoremsResult::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_congrtheorems_result_partial() {
        let r = CongrTheoremsResult::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_congrtheorems_result_skipped() {
        let r = CongrTheoremsResult::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_congrtheorems_analysis_pass_run() {
        let mut p = CongrTheoremsAnalysisPass::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_congrtheorems_analysis_pass_empty_input() {
        let mut p = CongrTheoremsAnalysisPass::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_congrtheorems_analysis_pass_success_rate() {
        let mut p = CongrTheoremsAnalysisPass::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_congrtheorems_analysis_pass_disable() {
        let mut p = CongrTheoremsAnalysisPass::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_congrtheorems_pipeline_basic() {
        let mut pipeline = CongrTheoremsPipeline::new("main_pipeline");
        pipeline.add_pass(CongrTheoremsAnalysisPass::new("pass1"));
        pipeline.add_pass(CongrTheoremsAnalysisPass::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_congrtheorems_pipeline_disabled_pass() {
        let mut pipeline = CongrTheoremsPipeline::new("partial");
        let mut p = CongrTheoremsAnalysisPass::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(CongrTheoremsAnalysisPass::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_congrtheorems_diff_basic() {
        let mut d = CongrTheoremsDiff::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_congrtheorems_diff_summary() {
        let mut d = CongrTheoremsDiff::new();
        d.add("x");
        d.add("y");
        d.remove("z");
        let s = d.summary();
        assert!(s.contains("+2"));
    }
    #[test]
    fn test_congrtheorems_config_set_get() {
        let mut cfg = CongrTheoremsConfig::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_congrtheorems_config_read_only() {
        let mut cfg = CongrTheoremsConfig::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_congrtheorems_config_remove() {
        let mut cfg = CongrTheoremsConfig::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_congrtheorems_diagnostics_basic() {
        let mut diag = CongrTheoremsDiagnostics::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_congrtheorems_diagnostics_max_errors() {
        let mut diag = CongrTheoremsDiagnostics::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_congrtheorems_diagnostics_clear() {
        let mut diag = CongrTheoremsDiagnostics::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_congrtheorems_config_value_types() {
        let b = CongrTheoremsConfigValue::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = CongrTheoremsConfigValue::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = CongrTheoremsConfigValue::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = CongrTheoremsConfigValue::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = CongrTheoremsConfigValue::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
#[cfg(test)]
mod congr_theorems_ext_tests_2500 {
    use super::*;
    use crate::congr_theorems::*;
    #[test]
    fn test_congr_theorems_ext_result_ok_2500() {
        let r = CongrTheoremsExtResult2500::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_congr_theorems_ext_result_err_2500() {
        let r = CongrTheoremsExtResult2500::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_congr_theorems_ext_result_partial_2500() {
        let r = CongrTheoremsExtResult2500::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_congr_theorems_ext_result_skipped_2500() {
        let r = CongrTheoremsExtResult2500::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_congr_theorems_ext_pass_run_2500() {
        let mut p = CongrTheoremsExtPass2500::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_congr_theorems_ext_pass_empty_2500() {
        let mut p = CongrTheoremsExtPass2500::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_congr_theorems_ext_pass_rate_2500() {
        let mut p = CongrTheoremsExtPass2500::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_congr_theorems_ext_pass_disable_2500() {
        let mut p = CongrTheoremsExtPass2500::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_congr_theorems_ext_pipeline_basic_2500() {
        let mut pipeline = CongrTheoremsExtPipeline2500::new("main_pipeline");
        pipeline.add_pass(CongrTheoremsExtPass2500::new("pass1"));
        pipeline.add_pass(CongrTheoremsExtPass2500::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_congr_theorems_ext_pipeline_disabled_2500() {
        let mut pipeline = CongrTheoremsExtPipeline2500::new("partial");
        let mut p = CongrTheoremsExtPass2500::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(CongrTheoremsExtPass2500::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_congr_theorems_ext_diff_basic_2500() {
        let mut d = CongrTheoremsExtDiff2500::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_congr_theorems_ext_config_set_get_2500() {
        let mut cfg = CongrTheoremsExtConfig2500::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_congr_theorems_ext_config_read_only_2500() {
        let mut cfg = CongrTheoremsExtConfig2500::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_congr_theorems_ext_config_remove_2500() {
        let mut cfg = CongrTheoremsExtConfig2500::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_congr_theorems_ext_diagnostics_basic_2500() {
        let mut diag = CongrTheoremsExtDiag2500::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_congr_theorems_ext_diagnostics_max_errors_2500() {
        let mut diag = CongrTheoremsExtDiag2500::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_congr_theorems_ext_diagnostics_clear_2500() {
        let mut diag = CongrTheoremsExtDiag2500::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_congr_theorems_ext_config_value_types_2500() {
        let b = CongrTheoremsExtConfigVal2500::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = CongrTheoremsExtConfigVal2500::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = CongrTheoremsExtConfigVal2500::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = CongrTheoremsExtConfigVal2500::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = CongrTheoremsExtConfigVal2500::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
