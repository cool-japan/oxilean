//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::{Expr, KernelError, Level, Name};

use super::types::{
    ConfigNode, FocusStack, InductiveEnv, InductiveError, InductiveFamily, InductiveType,
    InductiveTypeBuilder, InductiveTypeInfo, IntroRule, LabelSet, NonEmptyVec, RecursorBuilder,
    SimpleDag, SmallMap, StatSummary, TransformStat, VersionedRecord, WindowIterator,
};

/// Count the number of Pi arguments in a type.
pub(super) fn count_pi_args(ty: &Expr) -> u32 {
    match ty {
        Expr::Pi(_, _, _, body) => 1 + count_pi_args(body),
        _ => 0,
    }
}
/// Peel all Pi binders from a type, returning a list of domain types and the final body.
pub(super) fn peel_pi_binders(ty: &Expr) -> (Vec<Expr>, &Expr) {
    let mut domains = Vec::new();
    let mut current = ty;
    while let Expr::Pi(_, _, dom, body) = current {
        domains.push(dom.as_ref().clone());
        current = body;
    }
    (domains, current)
}
/// Lift all free BVars in an expression by `amount` (used to adjust de Bruijn indices
/// when inserting new binders outside the expression).
pub(super) fn lift_expr_bvars(expr: &Expr, amount: u32) -> Expr {
    lift_expr_bvars_at(expr, amount, 0)
}
fn lift_expr_bvars_at(expr: &Expr, amount: u32, cutoff: u32) -> Expr {
    match expr {
        Expr::BVar(i) => {
            if *i >= cutoff {
                Expr::BVar(i + amount)
            } else {
                expr.clone()
            }
        }
        Expr::Sort(_) | Expr::FVar(_) | Expr::Const(_, _) | Expr::Lit(_) => expr.clone(),
        Expr::App(f, a) => Expr::App(
            Box::new(lift_expr_bvars_at(f, amount, cutoff)),
            Box::new(lift_expr_bvars_at(a, amount, cutoff)),
        ),
        Expr::Pi(bi, n, ty, body) => Expr::Pi(
            *bi,
            n.clone(),
            Box::new(lift_expr_bvars_at(ty, amount, cutoff)),
            Box::new(lift_expr_bvars_at(body, amount, cutoff + 1)),
        ),
        Expr::Lam(bi, n, ty, body) => Expr::Lam(
            *bi,
            n.clone(),
            Box::new(lift_expr_bvars_at(ty, amount, cutoff)),
            Box::new(lift_expr_bvars_at(body, amount, cutoff + 1)),
        ),
        Expr::Let(n, ty, val, body) => Expr::Let(
            n.clone(),
            Box::new(lift_expr_bvars_at(ty, amount, cutoff)),
            Box::new(lift_expr_bvars_at(val, amount, cutoff)),
            Box::new(lift_expr_bvars_at(body, amount, cutoff + 1)),
        ),
        Expr::Proj(n, idx, e) => Expr::Proj(
            n.clone(),
            *idx,
            Box::new(lift_expr_bvars_at(e, amount, cutoff)),
        ),
    }
}
/// Check an inductive type declaration for validity.
#[allow(clippy::result_large_err)]
pub fn check_inductive(ind: &InductiveType) -> Result<(), KernelError> {
    for intro in &ind.intro_rules {
        if !returns_type(&intro.ty, &ind.name) {
            return Err(KernelError::InvalidInductive(format!(
                "constructor {} does not return type {}",
                intro.name, ind.name
            )));
        }
    }
    for intro in &ind.intro_rules {
        check_positivity(&ind.name, &intro.ty)?;
    }
    Ok(())
}
fn returns_type(ty: &Expr, name: &Name) -> bool {
    match ty {
        Expr::Const(n, _) => n == name,
        Expr::App(f, _) => returns_type(f, name),
        Expr::Pi(_, _, _, cod) => returns_type(cod, name),
        _ => false,
    }
}
#[allow(clippy::result_large_err)]
fn check_positivity(ind_name: &Name, ty: &Expr) -> Result<(), KernelError> {
    check_positivity_rec(ind_name, ty, true)
}
#[allow(clippy::result_large_err)]
fn check_positivity_rec(ind_name: &Name, ty: &Expr, positive: bool) -> Result<(), KernelError> {
    match ty {
        Expr::Const(n, _) => {
            if n == ind_name && !positive {
                return Err(KernelError::InvalidInductive(format!(
                    "negative occurrence of {} in constructor type",
                    ind_name
                )));
            }
            Ok(())
        }
        Expr::App(f, a) => {
            check_positivity_rec(ind_name, f, positive)?;
            check_positivity_rec(ind_name, a, positive)
        }
        Expr::Pi(_, _, dom, cod) => {
            check_positivity_rec_domain(ind_name, dom)?;
            check_positivity_rec(ind_name, cod, positive)
        }
        Expr::Lam(_, _, ty_inner, body) => {
            check_positivity_rec(ind_name, ty_inner, positive)?;
            check_positivity_rec(ind_name, body, positive)
        }
        _ => Ok(()),
    }
}
/// Check the domain of a Pi binder for strict positivity.
/// Direct references to `ind_name` are allowed; references inside
/// nested function types (which would be truly negative) are rejected.
#[allow(clippy::result_large_err)]
fn check_positivity_rec_domain(ind_name: &Name, dom: &Expr) -> Result<(), KernelError> {
    match dom {
        Expr::Const(_, _) => Ok(()),
        Expr::App(f, a) => {
            check_positivity_rec_domain(ind_name, f)?;
            check_positivity_rec_domain(ind_name, a)
        }
        Expr::Pi(_, _, inner_dom, inner_cod) => {
            check_positivity_rec(ind_name, inner_dom, false)?;
            check_positivity_rec_domain(ind_name, inner_cod)
        }
        _ => Ok(()),
    }
}
/// Reduce a recursor application (iota-reduction).
///
/// This is the legacy API. The new implementation is in `reduce.rs`.
pub fn reduce_recursor(_rec_name: &Name, _args: &[Expr]) -> Option<Expr> {
    None
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::BinderInfo;
    #[test]
    fn test_inductive_creation() {
        let bool_ty = Expr::Sort(Level::zero());
        let true_intro = IntroRule {
            name: Name::str("true"),
            ty: Expr::Const(Name::str("Bool"), vec![]),
        };
        let false_intro = IntroRule {
            name: Name::str("false"),
            ty: Expr::Const(Name::str("Bool"), vec![]),
        };
        let bool_ind = InductiveType::new(
            Name::str("Bool"),
            vec![],
            0,
            0,
            bool_ty,
            vec![true_intro, false_intro],
        );
        assert_eq!(bool_ind.name, Name::str("Bool"));
        assert_eq!(bool_ind.intro_rules.len(), 2);
        assert!(!bool_ind.is_recursive());
    }
    #[test]
    fn test_inductive_env() {
        let mut env = InductiveEnv::new();
        let bool_ty = Expr::Sort(Level::zero());
        let bool_ind = InductiveType::new(
            Name::str("Bool"),
            vec![],
            0,
            0,
            bool_ty,
            vec![IntroRule {
                name: Name::str("true"),
                ty: Expr::Const(Name::str("Bool"), vec![]),
            }],
        );
        env.add(bool_ind).expect("value should be present");
        assert!(env.get(&Name::str("Bool")).is_some());
        assert!(env.is_constructor(&Name::str("true")));
    }
    #[test]
    fn test_positivity_check() {
        let pos_ty = Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Const(Name::str("Bool"), vec![])),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
        );
        assert!(check_positivity(&Name::str("Nat"), &pos_ty).is_ok());
        let neg_ty = Expr::Pi(
            BinderInfo::Default,
            Name::str("f"),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("n"),
                Box::new(Expr::Const(Name::str("Nat"), vec![])),
                Box::new(Expr::Const(Name::str("Bool"), vec![])),
            )),
            Box::new(Expr::Const(Name::str("Bool"), vec![])),
        );
        assert!(check_positivity(&Name::str("Nat"), &neg_ty).is_err());
    }
    #[test]
    fn test_recursive_detection() {
        let nat_ty = Expr::Sort(Level::zero());
        let nat_ind = InductiveType::new(
            Name::str("Nat"),
            vec![],
            0,
            0,
            nat_ty,
            vec![
                IntroRule {
                    name: Name::str("zero"),
                    ty: Expr::Const(Name::str("Nat"), vec![]),
                },
                IntroRule {
                    name: Name::str("succ"),
                    ty: Expr::Pi(
                        BinderInfo::Default,
                        Name::str("n"),
                        Box::new(Expr::Const(Name::str("Nat"), vec![])),
                        Box::new(Expr::Const(Name::str("Nat"), vec![])),
                    ),
                },
            ],
        );
        assert!(nat_ind.is_recursive());
    }
    #[test]
    fn test_to_constant_infos() {
        let nat_ind = InductiveType::new(
            Name::str("Nat"),
            vec![],
            0,
            0,
            Expr::Sort(Level::succ(Level::zero())),
            vec![
                IntroRule {
                    name: Name::str("Nat.zero"),
                    ty: Expr::Const(Name::str("Nat"), vec![]),
                },
                IntroRule {
                    name: Name::str("Nat.succ"),
                    ty: Expr::Pi(
                        BinderInfo::Default,
                        Name::str("n"),
                        Box::new(Expr::Const(Name::str("Nat"), vec![])),
                        Box::new(Expr::Const(Name::str("Nat"), vec![])),
                    ),
                },
            ],
        );
        let (ind_ci, ctor_cis, rec_ci) = nat_ind.to_constant_infos();
        assert!(ind_ci.is_inductive());
        let iv = ind_ci.to_inductive_val().expect("iv should be present");
        assert_eq!(iv.ctors.len(), 2);
        assert!(iv.is_rec);
        assert_eq!(ctor_cis.len(), 2);
        assert!(ctor_cis[0].is_constructor());
        let cv0 = ctor_cis[0]
            .to_constructor_val()
            .expect("cv0 should be present");
        assert_eq!(cv0.num_fields, 0);
        let cv1 = ctor_cis[1]
            .to_constructor_val()
            .expect("cv1 should be present");
        assert_eq!(cv1.num_fields, 1);
        assert!(rec_ci.is_recursor());
        let rv = rec_ci.to_recursor_val().expect("rv should be present");
        assert_eq!(rv.num_minors, 2);
        assert_eq!(rv.get_major_idx(), 3);
    }
    #[test]
    fn test_register_in_env() {
        let mut env = crate::Environment::new();
        let mut ind_env = InductiveEnv::new();
        let bool_ind = InductiveType::new(
            Name::str("Bool"),
            vec![],
            0,
            0,
            Expr::Sort(Level::succ(Level::zero())),
            vec![
                IntroRule {
                    name: Name::str("Bool.true"),
                    ty: Expr::Const(Name::str("Bool"), vec![]),
                },
                IntroRule {
                    name: Name::str("Bool.false"),
                    ty: Expr::Const(Name::str("Bool"), vec![]),
                },
            ],
        );
        ind_env
            .register_in_env(&bool_ind, &mut env)
            .expect("value should be present");
        assert!(env.is_inductive(&Name::str("Bool")));
        assert!(env.is_constructor(&Name::str("Bool.true")));
        assert!(env.is_constructor(&Name::str("Bool.false")));
        assert!(env.is_recursor(&Name::str("Bool").append_str("rec")));
    }
}
/// Build the `Bool` inductive type (no parameters, two constructors).
#[allow(dead_code)]
pub fn mk_bool_inductive() -> InductiveType {
    InductiveTypeBuilder::new()
        .name(Name::str("Bool"))
        .ty(Expr::Sort(Level::succ(Level::zero())))
        .intro_rule(
            Name::str("Bool.true"),
            Expr::Const(Name::str("Bool"), vec![]),
        )
        .intro_rule(
            Name::str("Bool.false"),
            Expr::Const(Name::str("Bool"), vec![]),
        )
        .build()
        .expect("Bool inductive type build failed")
}
/// Build the `Nat` inductive type.
#[allow(dead_code)]
pub fn mk_nat_inductive() -> InductiveType {
    InductiveTypeBuilder::new()
        .name(Name::str("Nat"))
        .ty(Expr::Sort(Level::succ(Level::zero())))
        .intro_rule(Name::str("Nat.zero"), Expr::Const(Name::str("Nat"), vec![]))
        .intro_rule(
            Name::str("Nat.succ"),
            Expr::Pi(
                crate::BinderInfo::Default,
                Name::str("n"),
                Box::new(Expr::Const(Name::str("Nat"), vec![])),
                Box::new(Expr::Const(Name::str("Nat"), vec![])),
            ),
        )
        .build()
        .expect("Nat inductive type build failed")
}
/// Build the `Unit` inductive type (single constructor, no fields).
#[allow(dead_code)]
pub fn mk_unit_inductive() -> InductiveType {
    InductiveTypeBuilder::new()
        .name(Name::str("Unit"))
        .ty(Expr::Sort(Level::succ(Level::zero())))
        .intro_rule(
            Name::str("Unit.unit"),
            Expr::Const(Name::str("Unit"), vec![]),
        )
        .build()
        .expect("Unit inductive type build failed")
}
/// Build the `Empty` inductive type (no constructors — ex falso).
#[allow(dead_code)]
pub fn mk_empty_inductive() -> InductiveType {
    InductiveTypeBuilder::new()
        .name(Name::str("Empty"))
        .ty(Expr::Sort(Level::succ(Level::zero())))
        .build()
        .unwrap_or_else(|_| {
            InductiveType::new(
                Name::str("Empty"),
                vec![],
                0,
                0,
                Expr::Sort(Level::succ(Level::zero())),
                vec![],
            )
        })
}
/// Given an `InductiveType`, return the number of fields in each constructor.
#[allow(dead_code)]
pub fn constructor_field_counts(ind: &InductiveType) -> Vec<(Name, u32)> {
    ind.intro_rules
        .iter()
        .map(|r| {
            let total = count_pi_args(&r.ty);
            let fields = total.saturating_sub(ind.num_params);
            (r.name.clone(), fields)
        })
        .collect()
}
/// Check whether an inductive type is a singleton (one constructor, no recursive fields).
#[allow(dead_code)]
pub fn is_singleton_inductive(ind: &InductiveType) -> bool {
    if ind.intro_rules.len() != 1 {
        return false;
    }
    !ind.is_recursive()
}
/// Check whether an inductive type is an enum (all constructors have no fields).
#[allow(dead_code)]
pub fn is_enum_inductive(ind: &InductiveType) -> bool {
    ind.intro_rules.iter().all(|r| {
        let fields = count_pi_args(&r.ty).saturating_sub(ind.num_params);
        fields == 0
    })
}
/// Return the recursor name for a given inductive type name.
#[allow(dead_code)]
pub fn recursor_name(ind_name: &Name) -> Name {
    Name::Str(Box::new(ind_name.clone()), "rec".to_string())
}
#[cfg(test)]
mod extra_tests {
    use super::*;
    use crate::BinderInfo;
    #[test]
    fn test_builder_basic() {
        let ind = InductiveTypeBuilder::new()
            .name(Name::str("MyType"))
            .ty(Expr::Sort(Level::succ(Level::zero())))
            .intro_rule(
                Name::str("MyType.mk"),
                Expr::Const(Name::str("MyType"), vec![]),
            )
            .build()
            .expect("value should be present");
        assert_eq!(ind.name, Name::str("MyType"));
        assert_eq!(ind.intro_rules.len(), 1);
    }
    #[test]
    fn test_builder_missing_name_fails() {
        let result = InductiveTypeBuilder::new()
            .ty(Expr::Sort(Level::zero()))
            .build();
        assert!(result.is_err());
    }
    #[test]
    fn test_builder_missing_ty_fails() {
        let result = InductiveTypeBuilder::new().name(Name::str("X")).build();
        assert!(result.is_err());
    }
    #[test]
    fn test_mk_bool_inductive() {
        let b = mk_bool_inductive();
        assert_eq!(b.name, Name::str("Bool"));
        assert_eq!(b.intro_rules.len(), 2);
        assert!(is_enum_inductive(&b));
    }
    #[test]
    fn test_mk_nat_inductive() {
        let n = mk_nat_inductive();
        assert_eq!(n.name, Name::str("Nat"));
        assert!(n.is_recursive());
        assert!(!is_enum_inductive(&n));
    }
    #[test]
    fn test_mk_unit_inductive() {
        let u = mk_unit_inductive();
        assert!(is_singleton_inductive(&u));
    }
    #[test]
    fn test_mk_empty_inductive() {
        let e = mk_empty_inductive();
        assert_eq!(e.name, Name::str("Empty"));
        assert_eq!(e.intro_rules.len(), 0);
        assert!(!is_singleton_inductive(&e));
    }
    #[test]
    fn test_constructor_field_counts_bool() {
        let b = mk_bool_inductive();
        let counts = constructor_field_counts(&b);
        assert_eq!(counts.len(), 2);
        assert_eq!(counts[0].1, 0);
        assert_eq!(counts[1].1, 0);
    }
    #[test]
    fn test_constructor_field_counts_nat() {
        let n = mk_nat_inductive();
        let counts = constructor_field_counts(&n);
        assert_eq!(counts[0].1, 0);
        assert_eq!(counts[1].1, 1);
    }
    #[test]
    fn test_recursor_name() {
        let n = Name::str("Nat");
        let r = recursor_name(&n);
        assert_eq!(r, Name::Str(Box::new(Name::str("Nat")), "rec".to_string()));
    }
    #[test]
    fn test_is_prop_flag() {
        let mut ind = mk_bool_inductive();
        assert!(!ind.is_prop);
        ind.is_prop = true;
        assert!(ind.is_prop);
    }
    #[test]
    fn test_arity() {
        let ind = InductiveTypeBuilder::new()
            .name(Name::str("Vec"))
            .ty(Expr::Sort(Level::succ(Level::zero())))
            .num_params(1)
            .num_indices(1)
            .intro_rule(Name::str("Vec.nil"), Expr::Const(Name::str("Vec"), vec![]))
            .build()
            .expect("value should be present");
        assert_eq!(ind.arity(), 2);
    }
    #[test]
    fn test_builder_univ_params() {
        let ind = InductiveTypeBuilder::new()
            .name(Name::str("List"))
            .ty(Expr::Sort(Level::succ(Level::zero())))
            .univ_params(vec![Name::str("u")])
            .intro_rule(
                Name::str("List.nil"),
                Expr::Const(Name::str("List"), vec![Level::param(Name::str("u"))]),
            )
            .build()
            .expect("value should be present");
        assert_eq!(ind.univ_params.len(), 1);
    }
    #[test]
    fn test_is_nested_flag() {
        let ind = InductiveTypeBuilder::new()
            .name(Name::str("NTree"))
            .ty(Expr::Sort(Level::succ(Level::zero())))
            .is_nested(true)
            .intro_rule(
                Name::str("NTree.leaf"),
                Expr::Const(Name::str("NTree"), vec![]),
            )
            .build()
            .expect("value should be present");
        assert!(ind.is_nested);
    }
    #[test]
    fn test_positivity_passes_for_nat_succ() {
        let succ_ty = Expr::Pi(
            BinderInfo::Default,
            Name::str("n"),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
        );
        assert!(check_positivity(&Name::str("Nat"), &succ_ty).is_ok());
    }
}
#[cfg(test)]
mod inductive_extra_tests {
    use super::*;
    fn mk_nat_type() -> InductiveType {
        InductiveType::new(
            Name::str("Nat"),
            vec![],
            0,
            0,
            Expr::Sort(Level::succ(Level::zero())),
            vec![
                IntroRule {
                    name: Name::str("Nat.zero"),
                    ty: Expr::Const(Name::str("Nat"), vec![]),
                },
                IntroRule {
                    name: Name::str("Nat.succ"),
                    ty: Expr::Pi(
                        crate::BinderInfo::Default,
                        Name::str("n"),
                        Box::new(Expr::Const(Name::str("Nat"), vec![])),
                        Box::new(Expr::Const(Name::str("Nat"), vec![])),
                    ),
                },
            ],
        )
    }
    #[test]
    fn test_recursor_builder_validate() {
        let builder = RecursorBuilder::new(Name::str("Nat.rec"));
        assert!(builder.validate().is_ok());
    }
    #[test]
    fn test_recursor_builder_build_name() {
        let builder = RecursorBuilder::new(Name::str("List.rec"));
        assert_eq!(builder.build_name(), Name::str("List.rec"));
    }
    #[test]
    fn test_recursor_builder_chain() {
        let builder = RecursorBuilder::new(Name::str("Nat.rec"))
            .num_params(0)
            .num_indices(0)
            .is_prop(false);
        assert!(!builder.is_prop);
        assert_eq!(builder.num_params, 0);
    }
    #[test]
    fn test_inductive_family_singleton() {
        let nat = mk_nat_type();
        let fam = InductiveFamily::singleton(nat.clone());
        assert!(fam.is_singleton());
        assert_eq!(fam.len(), 1);
        assert_eq!(fam.total_constructors(), 2);
    }
    #[test]
    fn test_inductive_family_type_names() {
        let nat = mk_nat_type();
        let fam = InductiveFamily::singleton(nat);
        let names = fam.type_names();
        assert_eq!(names.len(), 1);
        assert_eq!(*names[0], Name::str("Nat"));
    }
    #[test]
    fn test_inductive_family_all_constructor_names() {
        let nat = mk_nat_type();
        let fam = InductiveFamily::singleton(nat);
        let ctors = fam.all_constructor_names();
        assert_eq!(ctors.len(), 2);
    }
    #[test]
    fn test_inductive_family_find_type() {
        let nat = mk_nat_type();
        let fam = InductiveFamily::singleton(nat);
        assert!(fam.find_type(&Name::str("Nat")).is_some());
        assert!(fam.find_type(&Name::str("List")).is_none());
    }
    #[test]
    fn test_inductive_type_info_from_type() {
        let nat = mk_nat_type();
        let info = InductiveTypeInfo::from_type(&nat, false);
        assert_eq!(info.name, Name::str("Nat"));
        assert_eq!(info.num_constructors, 2);
        assert!(!info.is_prop);
        assert!(!info.is_mutual);
    }
    #[test]
    fn test_inductive_type_info_summary() {
        let nat = mk_nat_type();
        let info = InductiveTypeInfo::from_type(&nat, false);
        let s = info.summary();
        assert!(s.contains("Nat"));
        assert!(s.contains("ctors"));
    }
    #[test]
    fn test_inductive_error_display() {
        let e = InductiveError::AlreadyDefined(Name::str("Nat"));
        assert!(e.to_string().contains("already defined"));
        let e2 = InductiveError::NonStrictlyPositive(Name::str("T"));
        assert!(e2.to_string().contains("non-strictly-positive"));
    }
    #[test]
    fn test_inductive_error_other() {
        let e = InductiveError::Other("some problem".to_string());
        assert!(e.to_string().contains("some problem"));
    }
    #[test]
    fn test_inductive_family_empty() {
        let fam = InductiveFamily::new(vec![], vec![]);
        assert!(fam.is_empty());
        assert_eq!(fam.total_constructors(), 0);
    }
    #[test]
    fn test_inductive_type_is_recursive() {
        let nat = mk_nat_type();
        assert!(nat.is_recursive());
    }
    #[test]
    fn test_inductive_type_num_constructors() {
        let nat = mk_nat_type();
        assert_eq!(nat.num_constructors(), 2);
    }
    fn mk_bool_type() -> InductiveType {
        InductiveType::new(
            Name::str("Bool"),
            vec![],
            0,
            0,
            Expr::Sort(Level::succ(Level::zero())),
            vec![
                IntroRule {
                    name: Name::str("Bool.false"),
                    ty: Expr::Const(Name::str("Bool"), vec![]),
                },
                IntroRule {
                    name: Name::str("Bool.true"),
                    ty: Expr::Const(Name::str("Bool"), vec![]),
                },
            ],
        )
    }
    #[test]
    fn test_recursor_rhs_bool_false() {
        let bool_ind = mk_bool_type();
        let (_, _, rec_ci) = bool_ind.to_constant_infos();
        let rec_val = rec_ci.to_recursor_val().expect("rec_val should be present");
        let rule_false = rec_val
            .get_rule(&Name::str("Bool.false"))
            .expect("rule_false should be present");
        assert_eq!(rule_false.nfields, 0);
        assert_eq!(rule_false.rhs, Expr::BVar(1));
    }
    #[test]
    fn test_recursor_rhs_bool_true() {
        let bool_ind = mk_bool_type();
        let (_, _, rec_ci) = bool_ind.to_constant_infos();
        let rec_val = rec_ci.to_recursor_val().expect("rec_val should be present");
        let rule_true = rec_val
            .get_rule(&Name::str("Bool.true"))
            .expect("rule_true should be present");
        assert_eq!(rule_true.nfields, 0);
        assert_eq!(rule_true.rhs, Expr::BVar(0));
    }
    #[test]
    fn test_recursor_rhs_nat_zero() {
        let nat_ind = mk_nat_type();
        let (_, _, rec_ci) = nat_ind.to_constant_infos();
        let rec_val = rec_ci.to_recursor_val().expect("rec_val should be present");
        let rule_zero = rec_val
            .get_rule(&Name::str("Nat.zero"))
            .expect("rule_zero should be present");
        assert_eq!(rule_zero.nfields, 0);
        assert_eq!(rule_zero.rhs, Expr::BVar(1));
    }
    #[test]
    fn test_recursor_rhs_nat_succ() {
        let nat_ind = mk_nat_type();
        let (_, _, rec_ci) = nat_ind.to_constant_infos();
        let rec_val = rec_ci.to_recursor_val().expect("rec_val should be present");
        let rule_succ = rec_val
            .get_rule(&Name::str("Nat.succ"))
            .expect("rule_succ should be present");
        assert_eq!(rule_succ.nfields, 1);
        let nat_rec = Expr::Const(Name::str("Nat").append_str("rec"), vec![]);
        let ih = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::App(Box::new(nat_rec), Box::new(Expr::BVar(3)))),
                    Box::new(Expr::BVar(2)),
                )),
                Box::new(Expr::BVar(1)),
            )),
            Box::new(Expr::BVar(0)),
        );
        let expected = Expr::App(
            Box::new(Expr::App(Box::new(Expr::BVar(1)), Box::new(Expr::BVar(0)))),
            Box::new(ih),
        );
        assert_eq!(rule_succ.rhs, expected);
    }
    #[test]
    fn test_recursor_iota_bool() {
        use crate::reduce::Reducer;
        use crate::{BinderInfo, Environment};
        let mut env = Environment::new();
        let mut ind_env = crate::InductiveEnv::new();
        let bool_ind = mk_bool_type();
        ind_env
            .register_in_env(&bool_ind, &mut env)
            .expect("value should be present");
        let bool_const = Expr::Const(Name::str("Bool"), vec![]);
        let false_const = Expr::Const(Name::str("Bool.false"), vec![]);
        let true_const = Expr::Const(Name::str("Bool.true"), vec![]);
        let motive = Expr::Lam(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(bool_const.clone()),
            Box::new(bool_const.clone()),
        );
        let rec_app = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Bool").append_str("rec"), vec![])),
                        Box::new(motive),
                    )),
                    Box::new(false_const.clone()),
                )),
                Box::new(true_const.clone()),
            )),
            Box::new(false_const.clone()),
        );
        let mut reducer = Reducer::new();
        let result = reducer.whnf_env(&rec_app, &env);
        assert_eq!(
            result, false_const,
            "Bool.rec motive false true Bool.false should reduce to Bool.false"
        );
    }
    #[test]
    fn test_recursor_iota_nat_zero() {
        use crate::reduce::Reducer;
        use crate::{BinderInfo, Environment};
        let mut env = Environment::new();
        let mut ind_env = crate::InductiveEnv::new();
        let nat_ind = mk_nat_type();
        ind_env
            .register_in_env(&nat_ind, &mut env)
            .expect("value should be present");
        let nat_const = Expr::Const(Name::str("Nat"), vec![]);
        let zero_const = Expr::Const(Name::str("Nat.zero"), vec![]);
        let motive = Expr::Lam(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(nat_const.clone()),
            Box::new(nat_const.clone()),
        );
        let zero_val = zero_const.clone();
        let succ_fn = Expr::Lam(
            BinderInfo::Default,
            Name::str("n"),
            Box::new(nat_const.clone()),
            Box::new(Expr::Lam(
                BinderInfo::Default,
                Name::str("ih"),
                Box::new(nat_const.clone()),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Nat.succ"), vec![])),
                    Box::new(Expr::BVar(1)),
                )),
            )),
        );
        let rec_app = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Nat").append_str("rec"), vec![])),
                        Box::new(motive),
                    )),
                    Box::new(zero_val.clone()),
                )),
                Box::new(succ_fn),
            )),
            Box::new(zero_const.clone()),
        );
        let mut reducer = Reducer::new();
        let result = reducer.whnf_env(&rec_app, &env);
        assert_eq!(
            result, zero_val,
            "Nat.rec motive zero_val succ_fn Nat.zero should reduce to zero_val"
        );
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
