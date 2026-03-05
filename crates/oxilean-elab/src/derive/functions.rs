//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Expr, Literal, Name};

use super::types::{
    AdditionalDerivableClass, BatchDeriver, ConstructorInfo, DerivableClass, DerivationContext,
    DerivationError, DerivationPlan, DeriveCommand, DeriveRegistry, DeriveResult, DeriveStats,
    Deriver, FieldAnalysis, InstanceNamer, StructuralEqDeriver, TypeAnalysis, TypeInfo,
};

/// Custom deriver function type.
pub type CustomDeriverFn =
    Box<dyn Fn(&TypeInfo) -> Result<DeriveResult, DerivationError> + Send + Sync>;
/// Create a field comparison expression: `BEq.beq lhs rhs`.
#[allow(dead_code)]
pub fn mk_field_comparison(field_ty: &Expr, lhs: &Expr, rhs: &Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("BEq.beq"), vec![])),
                Box::new(field_ty.clone()),
            )),
            Box::new(lhs.clone()),
        )),
        Box::new(rhs.clone()),
    )
}
/// Chain expressions with `and`: `a && b && c ...`.
///
/// An empty list yields `true`.
#[allow(dead_code)]
pub fn mk_and_chain(exprs: &[Expr]) -> Expr {
    if exprs.is_empty() {
        return mk_bool_lit(true);
    }
    let mut result = exprs[0].clone();
    for e in &exprs[1..] {
        result = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("and"), vec![])),
                Box::new(result),
            )),
            Box::new(e.clone()),
        );
    }
    result
}
/// Combine hash expressions: `mixHash h1 (mixHash h2 ...)`.
///
/// An empty list yields `hash 0`.
#[allow(dead_code)]
pub fn mk_hash_combine(exprs: &[Expr]) -> Expr {
    if exprs.is_empty() {
        return Expr::App(
            Box::new(Expr::Const(Name::str("hash"), vec![])),
            Box::new(Expr::Lit(Literal::Nat(0))),
        );
    }
    if exprs.len() == 1 {
        return exprs[0].clone();
    }
    // Safety: exprs has at least 2 elements (empty and single-element cases handled above)
    let mut result = exprs
        .last()
        .expect("exprs is non-empty after early returns")
        .clone();
    for e in exprs[..exprs.len() - 1].iter().rev() {
        result = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("mixHash"), vec![])),
                Box::new(e.clone()),
            )),
            Box::new(result),
        );
    }
    result
}
/// Build a repr string expression for a constructor.
///
/// E.g. `"CtorName" ++ " " ++ repr_field_0 ++ " " ++ repr_field_1`.
#[allow(dead_code)]
pub fn mk_repr_string(ctor_name: &Name, field_reprs: &[Expr]) -> Expr {
    let mut result = Expr::Lit(Literal::Str(format!("{}", ctor_name)));
    for field_repr in field_reprs {
        result = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("String.append"), vec![])),
                Box::new(result),
            )),
            Box::new(Expr::Lit(Literal::Str(" ".to_string()))),
        );
        result = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("String.append"), vec![])),
                Box::new(result),
            )),
            Box::new(field_repr.clone()),
        );
    }
    result
}
/// Collect type information from a simple inductive type description.
///
/// In a real implementation this would query the `Environment`.
#[allow(dead_code)]
pub fn collect_type_info(
    name: Name,
    constructors: Vec<ConstructorInfo>,
    is_recursive: bool,
) -> Result<TypeInfo, DerivationError> {
    Ok(TypeInfo {
        name,
        univ_params: vec![],
        params: vec![],
        constructors,
        is_recursive,
        num_indices: 0,
    })
}
/// Check if an instance is (trivially) available for a class applied to a type.
///
/// Placeholder -- a real implementation queries the environment.
#[allow(dead_code)]
pub fn check_instance_available(_class: &Name, _ty: &Expr) -> bool {
    true
}
/// Create a `Bool.true` or `Bool.false` literal.
pub fn mk_bool_lit(val: bool) -> Expr {
    if val {
        Expr::Const(Name::str("Bool.true"), vec![])
    } else {
        Expr::Const(Name::str("Bool.false"), vec![])
    }
}
/// Create a variable reference for the i-th LHS field.
pub fn mk_lhs_field_var(i: usize) -> Expr {
    Expr::FVar(oxilean_kernel::FVarId::new(2000 + i as u64))
}
/// Create a variable reference for the i-th RHS field.
pub fn mk_rhs_field_var(i: usize) -> Expr {
    Expr::FVar(oxilean_kernel::FVarId::new(3000 + i as u64))
}
/// Build a simplified match body from a list of arms.
///
/// Returns the first arm for a single-arm match, otherwise wraps in a
/// simple case-split structure.
pub fn build_match_body(arms: &[Expr]) -> Expr {
    if arms.is_empty() {
        return Expr::Const(Name::str("absurd"), vec![]);
    }
    if arms.len() == 1 {
        return arms[0].clone();
    }
    let mut result: Expr = Expr::Const(Name::str("casesOn"), vec![]);
    for arm in arms {
        result = Expr::App(Box::new(result), Box::new(arm.clone()));
    }
    result
}
/// Chain decidable AND checks.
pub fn mk_decidable_and_chain(exprs: &[Expr]) -> Expr {
    if exprs.is_empty() {
        return Expr::App(
            Box::new(Expr::Const(Name::str("Decidable.isTrue"), vec![])),
            Box::new(Expr::Const(Name::str("rfl"), vec![])),
        );
    }
    if exprs.len() == 1 {
        return exprs[0].clone();
    }
    let mut result = exprs[0].clone();
    for e in &exprs[1..] {
        result = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Decidable.and"), vec![])),
                Box::new(result),
            )),
            Box::new(e.clone()),
        );
    }
    result
}
/// Compare fields lexicographically using `Ord.compare` and `compareOrdering`.
///
/// For a list of fields `[(name, ty)]`, generates:
///   `compareOrdering (Ord.compare f0_l f0_r) (compareOrdering (Ord.compare f1_l f1_r) ...)`
/// The innermost (rightmost) comparison returns directly without wrapping.
pub fn mk_lex_field_compare(fields: &[(Name, Expr)]) -> Expr {
    let comparisons: Vec<Expr> = fields
        .iter()
        .enumerate()
        .map(|(i, (_, field_ty))| {
            Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Ord.compare"), vec![])),
                        Box::new(field_ty.clone()),
                    )),
                    Box::new(mk_lhs_field_var(i)),
                )),
                Box::new(mk_rhs_field_var(i)),
            )
        })
        .collect();
    if comparisons.is_empty() {
        return Expr::Const(Name::str("Ordering.eq"), vec![]);
    }
    if comparisons.len() == 1 {
        return comparisons[0].clone();
    }
    let mut result = comparisons[comparisons.len() - 1].clone();
    for c in comparisons[..comparisons.len() - 1].iter().rev() {
        result = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("compareOrdering"), vec![])),
                Box::new(c.clone()),
            )),
            Box::new(result),
        );
    }
    result
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::derive::*;
    use oxilean_kernel::Level;
    fn nat_ty() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    fn string_ty() -> Expr {
        Expr::Const(Name::str("String"), vec![])
    }
    fn bool_ty() -> Expr {
        Expr::Const(Name::str("Bool"), vec![])
    }
    /// Simple enum: `inductive Color | red | green | blue`
    fn color_type_info() -> TypeInfo {
        TypeInfo {
            name: Name::str("Color"),
            univ_params: vec![],
            params: vec![],
            constructors: vec![
                ConstructorInfo::new(Name::str("Color.red"), vec![], 0),
                ConstructorInfo::new(Name::str("Color.green"), vec![], 0),
                ConstructorInfo::new(Name::str("Color.blue"), vec![], 0),
            ],
            is_recursive: false,
            num_indices: 0,
        }
    }
    /// Struct with fields: `structure Point | mk (x : Nat) (y : Nat)`
    fn point_type_info() -> TypeInfo {
        TypeInfo {
            name: Name::str("Point"),
            univ_params: vec![],
            params: vec![],
            constructors: vec![ConstructorInfo::new(
                Name::str("Point.mk"),
                vec![(Name::str("x"), nat_ty()), (Name::str("y"), nat_ty())],
                0,
            )],
            is_recursive: false,
            num_indices: 0,
        }
    }
    /// Mixed: `inductive Shape | circle (r : Nat) | rect (w : Nat) (h : Nat)`
    fn shape_type_info() -> TypeInfo {
        TypeInfo {
            name: Name::str("Shape"),
            univ_params: vec![],
            params: vec![],
            constructors: vec![
                ConstructorInfo::new(
                    Name::str("Shape.circle"),
                    vec![(Name::str("r"), nat_ty())],
                    0,
                ),
                ConstructorInfo::new(
                    Name::str("Shape.rect"),
                    vec![(Name::str("w"), nat_ty()), (Name::str("h"), nat_ty())],
                    0,
                ),
            ],
            is_recursive: false,
            num_indices: 0,
        }
    }
    /// Recursive type: `inductive Tree | leaf | node (left : Tree) (right : Tree)`
    fn tree_type_info() -> TypeInfo {
        let tree_ty = Expr::Const(Name::str("Tree"), vec![]);
        TypeInfo {
            name: Name::str("Tree"),
            univ_params: vec![],
            params: vec![],
            constructors: vec![
                ConstructorInfo::new(Name::str("Tree.leaf"), vec![], 0),
                ConstructorInfo::new(
                    Name::str("Tree.node"),
                    vec![
                        (Name::str("left"), tree_ty.clone()),
                        (Name::str("right"), tree_ty),
                    ],
                    0,
                ),
            ],
            is_recursive: true,
            num_indices: 0,
        }
    }
    /// Empty type (no constructors).
    fn empty_type_info() -> TypeInfo {
        TypeInfo {
            name: Name::str("Empty"),
            univ_params: vec![],
            params: vec![],
            constructors: vec![],
            is_recursive: false,
            num_indices: 0,
        }
    }
    /// Person with String field.
    fn person_type_info() -> TypeInfo {
        TypeInfo {
            name: Name::str("Person"),
            univ_params: vec![],
            params: vec![],
            constructors: vec![ConstructorInfo::new(
                Name::str("Person.mk"),
                vec![
                    (Name::str("name"), string_ty()),
                    (Name::str("age"), nat_ty()),
                ],
                0,
            )],
            is_recursive: false,
            num_indices: 0,
        }
    }
    #[test]
    fn test_deriver_create() {
        let deriver = Deriver::new();
        assert!(!deriver.is_debug());
    }
    #[test]
    fn test_can_derive() {
        let deriver = Deriver::new();
        let ty = Expr::Const(Name::str("Nat"), vec![]);
        assert!(deriver.can_derive(DerivableClass::Eq, &ty));
    }
    #[test]
    fn test_registry() {
        let registry = DeriveRegistry::default();
        assert!(registry.is_derivable(DerivableClass::Eq));
        assert!(registry.is_derivable(DerivableClass::Show));
        assert!(registry.is_derivable(DerivableClass::BEq));
        assert!(registry.is_derivable(DerivableClass::Repr));
        assert!(registry.is_derivable(DerivableClass::Hashable));
        assert!(registry.is_derivable(DerivableClass::Inhabited));
        assert!(registry.is_derivable(DerivableClass::DecidableEq));
        assert!(registry.is_derivable(DerivableClass::Nonempty));
        assert!(registry.is_derivable(DerivableClass::ToString));
        assert_eq!(registry.all_classes().len(), 11);
    }
    #[test]
    fn test_set_debug() {
        let mut deriver = Deriver::new();
        deriver.set_debug(true);
        assert!(deriver.is_debug());
    }
    #[test]
    fn test_derivable_class_names() {
        assert_eq!(DerivableClass::BEq.class_name(), Name::str("BEq"));
        assert_eq!(
            DerivableClass::DecidableEq.class_name(),
            Name::str("DecidableEq")
        );
    }
    #[test]
    fn test_derivable_class_from_name() {
        assert_eq!(DerivableClass::from_name("BEq"), Some(DerivableClass::BEq));
        assert_eq!(
            DerivableClass::from_name("Inhabited"),
            Some(DerivableClass::Inhabited)
        );
        assert_eq!(DerivableClass::from_name("Unknown"), None);
    }
    #[test]
    fn test_derivable_class_display() {
        let s = format!("{}", DerivableClass::BEq);
        assert!(s.contains("BEq"));
    }
    #[test]
    fn test_derivation_error_display() {
        let e = DerivationError::CannotDerive("test".into());
        assert!(format!("{}", e).contains("cannot derive"));
        let e = DerivationError::MissingInstance("BEq for Foo".into());
        assert!(format!("{}", e).contains("missing instance"));
        let e = DerivationError::RecursiveType("Tree".into());
        assert!(format!("{}", e).contains("recursive"));
        let e = DerivationError::Other("oops".into());
        assert!(format!("{}", e).contains("oops"));
    }
    #[test]
    fn test_constructor_info_nullary() {
        let ci = ConstructorInfo::new(Name::str("Unit.unit"), vec![], 0);
        assert!(ci.is_nullary());
        assert_eq!(ci.num_fields(), 0);
    }
    #[test]
    fn test_constructor_info_with_fields() {
        let ci = ConstructorInfo::new(
            Name::str("Pair.mk"),
            vec![(Name::str("fst"), nat_ty()), (Name::str("snd"), nat_ty())],
            0,
        );
        assert!(!ci.is_nullary());
        assert_eq!(ci.num_fields(), 2);
    }
    #[test]
    fn test_type_info_enum() {
        let ti = color_type_info();
        assert!(ti.is_enum());
        assert_eq!(ti.num_constructors(), 3);
        assert_eq!(ti.total_fields(), 0);
    }
    #[test]
    fn test_type_info_struct() {
        let ti = point_type_info();
        assert!(!ti.is_enum());
        assert_eq!(ti.num_constructors(), 1);
        assert_eq!(ti.total_fields(), 2);
    }
    #[test]
    fn test_type_info_recursive() {
        let ti = tree_type_info();
        assert!(ti.is_recursive);
    }
    #[test]
    fn test_derive_beq_enum() {
        let deriver = Deriver::new();
        let result = deriver.derive_beq(&color_type_info());
        assert!(result.is_ok());
        let dr = result.expect("test operation should succeed");
        assert_eq!(dr.instance_name, Name::str("inst_BEq_Color"));
    }
    #[test]
    fn test_derive_beq_struct() {
        let deriver = Deriver::new();
        let result = deriver.derive_beq(&point_type_info());
        assert!(result.is_ok());
        let dr = result.expect("test operation should succeed");
        assert!(dr.instance_body.is_lambda());
    }
    #[test]
    fn test_derive_beq_mixed() {
        let deriver = Deriver::new();
        let result = deriver.derive_beq(&shape_type_info());
        assert!(result.is_ok());
    }
    #[test]
    fn test_derive_beq_recursive_fails() {
        let deriver = Deriver::new();
        let result = deriver.derive_beq(&tree_type_info());
        assert!(matches!(result, Err(DerivationError::RecursiveType(_))));
    }
    #[test]
    fn test_derive_repr_enum() {
        let deriver = Deriver::new();
        let result = deriver.derive_repr(&color_type_info());
        assert!(result.is_ok());
        let dr = result.expect("test operation should succeed");
        assert_eq!(dr.instance_name, Name::str("inst_Repr_Color"));
    }
    #[test]
    fn test_derive_repr_struct() {
        let deriver = Deriver::new();
        let result = deriver.derive_repr(&point_type_info());
        assert!(result.is_ok());
    }
    #[test]
    fn test_derive_repr_person() {
        let deriver = Deriver::new();
        let result = deriver.derive_repr(&person_type_info());
        assert!(result.is_ok());
    }
    #[test]
    fn test_derive_hashable_enum() {
        let deriver = Deriver::new();
        let result = deriver.derive_hashable(&color_type_info());
        assert!(result.is_ok());
        let dr = result.expect("test operation should succeed");
        assert_eq!(dr.instance_name, Name::str("inst_Hashable_Color"));
    }
    #[test]
    fn test_derive_hashable_struct() {
        let deriver = Deriver::new();
        let result = deriver.derive_hashable(&point_type_info());
        assert!(result.is_ok());
    }
    #[test]
    fn test_derive_hashable_recursive_fails() {
        let deriver = Deriver::new();
        let result = deriver.derive_hashable(&tree_type_info());
        assert!(matches!(result, Err(DerivationError::RecursiveType(_))));
    }
    #[test]
    fn test_derive_inhabited_enum() {
        let deriver = Deriver::new();
        let result = deriver.derive_inhabited(&color_type_info());
        assert!(result.is_ok());
        let dr = result.expect("test operation should succeed");
        assert_eq!(dr.instance_name, Name::str("inst_Inhabited_Color"));
        assert!(matches!(dr.instance_body, Expr::Const(n, _) if n == Name::str("Color.red")));
    }
    #[test]
    fn test_derive_inhabited_struct() {
        let deriver = Deriver::new();
        let result = deriver.derive_inhabited(&point_type_info());
        assert!(result.is_ok());
    }
    #[test]
    fn test_derive_inhabited_empty_fails() {
        let deriver = Deriver::new();
        let result = deriver.derive_inhabited(&empty_type_info());
        assert!(matches!(result, Err(DerivationError::CannotDerive(_))));
    }
    #[test]
    fn test_derive_decidable_eq_enum() {
        let deriver = Deriver::new();
        let result = deriver.derive_decidable_eq(&color_type_info());
        assert!(result.is_ok());
        let dr = result.expect("test operation should succeed");
        assert_eq!(dr.instance_name, Name::str("inst_DecidableEq_Color"));
    }
    #[test]
    fn test_derive_decidable_eq_struct() {
        let deriver = Deriver::new();
        let result = deriver.derive_decidable_eq(&point_type_info());
        assert!(result.is_ok());
    }
    #[test]
    fn test_derive_decidable_eq_recursive_fails() {
        let deriver = Deriver::new();
        let result = deriver.derive_decidable_eq(&tree_type_info());
        assert!(matches!(result, Err(DerivationError::RecursiveType(_))));
    }
    #[test]
    fn test_derive_nonempty_enum() {
        let deriver = Deriver::new();
        let result = deriver.derive_nonempty(&color_type_info());
        assert!(result.is_ok());
        let dr = result.expect("test operation should succeed");
        assert_eq!(dr.instance_name, Name::str("inst_Nonempty_Color"));
    }
    #[test]
    fn test_derive_nonempty_empty_fails() {
        let deriver = Deriver::new();
        let result = deriver.derive_nonempty(&empty_type_info());
        assert!(matches!(result, Err(DerivationError::CannotDerive(_))));
    }
    #[test]
    fn test_derive_to_string() {
        let deriver = Deriver::new();
        let result = deriver.derive_to_string(&color_type_info());
        assert!(result.is_ok());
        let dr = result.expect("test operation should succeed");
        assert_eq!(dr.instance_name, Name::str("inst_ToString_Color"));
        assert!(dr.instance_body.is_lambda());
    }
    #[test]
    fn test_derive_with_info_beq() {
        let deriver = Deriver::new();
        let result = deriver.derive_with_info(DerivableClass::BEq, &color_type_info());
        assert!(result.is_ok());
    }
    #[test]
    fn test_derive_with_info_repr() {
        let deriver = Deriver::new();
        let result = deriver.derive_with_info(DerivableClass::Repr, &color_type_info());
        assert!(result.is_ok());
    }
    #[test]
    fn test_derive_ord_enum() {
        let deriver = Deriver::new();
        let result = deriver.derive_with_info(DerivableClass::Ord, &color_type_info());
        assert!(result.is_ok());
        let dr = result.expect("test operation should succeed");
        assert_eq!(dr.instance_name, Name::str("inst_Ord_Color"));
        assert!(dr.instance_body.is_lambda());
    }
    #[test]
    fn test_derive_ord_struct() {
        let deriver = Deriver::new();
        let result = deriver.derive_ord(&point_type_info());
        assert!(result.is_ok());
        let dr = result.expect("test operation should succeed");
        assert_eq!(dr.instance_name, Name::str("inst_Ord_Point"));
    }
    #[test]
    fn test_derive_ord_recursive_fails() {
        let deriver = Deriver::new();
        let result = deriver.derive_ord(&tree_type_info());
        assert!(matches!(result, Err(DerivationError::RecursiveType(_))));
    }
    #[test]
    fn test_registry_custom_deriver() {
        let mut registry = DeriveRegistry::new();
        registry.register_custom_deriver(Name::str("MyClass"), |ti| {
            Ok(DeriveResult {
                instance_name: Name::str(format!("inst_MyClass_{}", ti.name)),
                instance_type: Expr::Sort(Level::zero()),
                instance_body: Expr::Sort(Level::zero()),
                aux_defs: vec![],
            })
        });
        assert!(registry.has_deriver(&Name::str("MyClass")));
        assert!(!registry.has_deriver(&Name::str("Unknown")));
        assert_eq!(registry.num_custom_derivers(), 1);
    }
    #[test]
    fn test_registry_derive_all() {
        let registry = DeriveRegistry::new();
        let classes = vec![
            DerivableClass::BEq,
            DerivableClass::Repr,
            DerivableClass::Inhabited,
        ];
        let results = registry.derive_all(&classes, &color_type_info());
        assert!(results.is_ok());
        assert_eq!(results.expect("test operation should succeed").len(), 3);
    }
    #[test]
    fn test_registry_derive_all_with_failure() {
        let registry = DeriveRegistry::new();
        let classes = vec![DerivableClass::BEq, DerivableClass::Hashable];
        let results = registry.derive_all(&classes, &tree_type_info());
        assert!(results.is_err());
    }
    #[test]
    fn test_registry_derive_all_custom() {
        let mut registry = DeriveRegistry::new();
        registry.register_custom_deriver(Name::str("BEq"), |ti| {
            Ok(DeriveResult {
                instance_name: Name::str(format!("custom_BEq_{}", ti.name)),
                instance_type: Expr::Sort(Level::zero()),
                instance_body: Expr::Sort(Level::zero()),
                aux_defs: vec![],
            })
        });
        let classes = vec![DerivableClass::BEq];
        let results = registry.derive_all(&classes, &tree_type_info());
        assert!(results.is_ok());
        let dr = &results.expect("test operation should succeed")[0];
        assert_eq!(dr.instance_name, Name::str("custom_BEq_Tree"));
    }
    #[test]
    fn test_mk_field_comparison() {
        let result = mk_field_comparison(
            &nat_ty(),
            &Expr::FVar(oxilean_kernel::FVarId::new(1)),
            &Expr::FVar(oxilean_kernel::FVarId::new(2)),
        );
        assert!(matches!(result, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_and_chain_empty() {
        let result = mk_and_chain(&[]);
        assert!(matches!(result, Expr::Const(n, _) if n == Name::str("Bool.true")));
    }
    #[test]
    fn test_mk_and_chain_single() {
        let e = Expr::Const(Name::str("x"), vec![]);
        let result = mk_and_chain(std::slice::from_ref(&e));
        assert_eq!(result, e);
    }
    #[test]
    fn test_mk_and_chain_multiple() {
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let result = mk_and_chain(&[a, b]);
        assert!(matches!(result, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_hash_combine_empty() {
        let result = mk_hash_combine(&[]);
        assert!(matches!(result, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_hash_combine_single() {
        let e = Expr::Const(Name::str("h"), vec![]);
        let result = mk_hash_combine(std::slice::from_ref(&e));
        assert_eq!(result, e);
    }
    #[test]
    fn test_mk_hash_combine_multiple() {
        let a = Expr::Const(Name::str("h1"), vec![]);
        let b = Expr::Const(Name::str("h2"), vec![]);
        let result = mk_hash_combine(&[a, b]);
        assert!(matches!(result, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_repr_string_no_fields() {
        let result = mk_repr_string(&Name::str("Unit"), &[]);
        assert!(matches!(result, Expr::Lit(Literal::Str(s)) if s == "Unit"));
    }
    #[test]
    fn test_mk_repr_string_with_fields() {
        let field = Expr::Const(Name::str("repr_x"), vec![]);
        let result = mk_repr_string(&Name::str("Pair"), &[field]);
        assert!(matches!(result, Expr::App(_, _)));
    }
    #[test]
    fn test_collect_type_info() {
        let ti = collect_type_info(
            Name::str("Foo"),
            vec![ConstructorInfo::new(Name::str("Foo.mk"), vec![], 0)],
            false,
        );
        assert!(ti.is_ok());
        let ti = ti.expect("test operation should succeed");
        assert_eq!(ti.name, Name::str("Foo"));
        assert_eq!(ti.num_constructors(), 1);
    }
    #[test]
    fn test_check_instance_available() {
        assert!(check_instance_available(&Name::str("BEq"), &nat_ty()));
    }
    #[test]
    fn test_derive_result_aux_defs() {
        let dr = DeriveResult {
            instance_name: Name::str("inst"),
            instance_type: Expr::Sort(Level::zero()),
            instance_body: Expr::Sort(Level::zero()),
            aux_defs: vec![(
                Name::str("aux"),
                Expr::Sort(Level::zero()),
                Expr::Sort(Level::zero()),
            )],
        };
        assert_eq!(dr.aux_defs.len(), 1);
    }
    #[test]
    fn test_default_deriver() {
        let d = Deriver::default();
        assert!(!d.is_debug());
    }
    #[test]
    fn test_default_registry() {
        let r = DeriveRegistry::default();
        assert!(r.all_classes().len() >= 4);
    }
    #[test]
    fn test_bool_field_type() {
        let ti = TypeInfo {
            name: Name::str("Flag"),
            univ_params: vec![],
            params: vec![],
            constructors: vec![ConstructorInfo::new(
                Name::str("Flag.mk"),
                vec![(Name::str("val"), bool_ty())],
                0,
            )],
            is_recursive: false,
            num_indices: 0,
        };
        let deriver = Deriver::new();
        let result = deriver.derive_beq(&ti);
        assert!(result.is_ok());
    }
}
/// Create a projection expression for the n-th field of a constructor application.
#[allow(dead_code)]
pub fn mk_field_proj(expr: &Expr, field_idx: usize, field_name: &Name) -> Expr {
    Expr::Proj(field_name.clone(), field_idx as u32, Box::new(expr.clone()))
}
/// Create a constructor application from a name and a list of argument expressions.
#[allow(dead_code)]
pub fn mk_ctor_app(ctor_name: &Name, args: &[Expr]) -> Expr {
    let mut result: Expr = Expr::Const(ctor_name.clone(), vec![]);
    for arg in args {
        result = Expr::App(Box::new(result), Box::new(arg.clone()));
    }
    result
}
/// Create an `Eq` proposition: `a = b`.
#[allow(dead_code)]
pub fn mk_eq_prop(ty: &Expr, lhs: &Expr, rhs: &Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Eq"), vec![])),
                Box::new(ty.clone()),
            )),
            Box::new(lhs.clone()),
        )),
        Box::new(rhs.clone()),
    )
}
/// Create a `Ne` proposition: `a ≠ b`.
#[allow(dead_code)]
pub fn mk_ne_prop(ty: &Expr, lhs: &Expr, rhs: &Expr) -> Expr {
    Expr::App(
        Box::new(Expr::Const(Name::str("Ne"), vec![])),
        Box::new(mk_eq_prop(ty, lhs, rhs)),
    )
}
/// Create an `And` type: `A ∧ B`.
#[allow(dead_code)]
pub fn mk_and_type(a: &Expr, b: &Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("And"), vec![])),
            Box::new(a.clone()),
        )),
        Box::new(b.clone()),
    )
}
/// Create an `Or` type: `A ∨ B`.
#[allow(dead_code)]
pub fn mk_or_type(a: &Expr, b: &Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("Or"), vec![])),
            Box::new(a.clone()),
        )),
        Box::new(b.clone()),
    )
}
/// Create a `Not` type: `¬ A`.
#[allow(dead_code)]
pub fn mk_not_type(a: &Expr) -> Expr {
    Expr::App(
        Box::new(Expr::Const(Name::str("Not"), vec![])),
        Box::new(a.clone()),
    )
}
/// Create a Pi type: `∀ (x : A), B`.
#[allow(dead_code)]
pub fn mk_pi_type(binder: BinderInfo, name: &Name, domain: &Expr, body: &Expr) -> Expr {
    Expr::Pi(
        binder,
        name.clone(),
        Box::new(domain.clone()),
        Box::new(body.clone()),
    )
}
/// Build a chain of Pi types from a list of `(name, domain)` pairs.
#[allow(dead_code)]
pub fn mk_pi_chain(binders: &[(Name, Expr)], body: &Expr) -> Expr {
    binders.iter().rev().fold(body.clone(), |acc, (n, ty)| {
        Expr::Pi(
            BinderInfo::Default,
            n.clone(),
            Box::new(ty.clone()),
            Box::new(acc),
        )
    })
}
/// Build a chain of lambda abstractions from a list of `(name, domain)` pairs.
#[allow(dead_code)]
pub fn mk_lam_chain(binders: &[(Name, Expr)], body: &Expr) -> Expr {
    binders.iter().rev().fold(body.clone(), |acc, (n, ty)| {
        Expr::Lam(
            BinderInfo::Default,
            n.clone(),
            Box::new(ty.clone()),
            Box::new(acc),
        )
    })
}
/// Check if an expression looks like a primitive type (Nat, String, Bool).
#[allow(dead_code)]
pub fn is_primitive_type(ty: &Expr) -> bool {
    matches!(
        ty, Expr::Const(n, _) if matches!(n.to_string().as_str(), "Nat" | "String" |
        "Bool" | "Int" | "Float" | "Char")
    )
}
#[cfg(test)]
mod extended_tests {
    use super::*;
    use crate::derive::*;
    use oxilean_kernel::{BinderInfo, Level};
    fn nat_ty() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    fn bool_ty() -> Expr {
        Expr::Const(Name::str("Bool"), vec![])
    }
    fn string_ty() -> Expr {
        Expr::Const(Name::str("String"), vec![])
    }
    fn simple_color() -> TypeInfo {
        TypeInfo {
            name: Name::str("Color"),
            univ_params: vec![],
            params: vec![],
            constructors: vec![
                ConstructorInfo::new(Name::str("Color.red"), vec![], 0),
                ConstructorInfo::new(Name::str("Color.green"), vec![], 0),
                ConstructorInfo::new(Name::str("Color.blue"), vec![], 0),
            ],
            is_recursive: false,
            num_indices: 0,
        }
    }
    fn simple_point() -> TypeInfo {
        TypeInfo {
            name: Name::str("Point"),
            univ_params: vec![],
            params: vec![],
            constructors: vec![ConstructorInfo::new(
                Name::str("Point.mk"),
                vec![(Name::str("x"), nat_ty()), (Name::str("y"), nat_ty())],
                0,
            )],
            is_recursive: false,
            num_indices: 0,
        }
    }
    #[test]
    fn test_additional_derivable_class_names() {
        assert_eq!(
            AdditionalDerivableClass::Functor.class_name(),
            Name::str("Functor")
        );
        assert_eq!(
            AdditionalDerivableClass::Semigroup.class_name(),
            Name::str("Semigroup")
        );
    }
    #[test]
    fn test_additional_derivable_requires_enum() {
        assert!(AdditionalDerivableClass::Enum.requires_enum());
        assert!(AdditionalDerivableClass::Bounded.requires_enum());
        assert!(!AdditionalDerivableClass::Functor.requires_enum());
    }
    #[test]
    fn test_additional_derivable_requires_type_param() {
        assert!(AdditionalDerivableClass::Functor.requires_type_param());
        assert!(AdditionalDerivableClass::Monad.requires_type_param());
        assert!(!AdditionalDerivableClass::Semigroup.requires_type_param());
    }
    #[test]
    fn test_additional_derivable_display() {
        let s = format!("{}", AdditionalDerivableClass::Monoid);
        assert_eq!(s, "Monoid");
    }
    #[test]
    fn test_derivation_context_cache() {
        let mut ctx = DerivationContext::new();
        let deriver = Deriver::new();
        let ti = simple_color();
        assert!(ctx.lookup(&Name::str("BEq"), &ti.name).is_none());
        assert_eq!(ctx.cache_misses, 1);
        let result = deriver.derive_beq(&ti).expect("derive should succeed");
        ctx.store(Name::str("BEq"), ti.name.clone(), result);
        assert!(ctx.lookup(&Name::str("BEq"), &ti.name).is_some());
        assert_eq!(ctx.cache_hits, 1);
        let rate = ctx.hit_rate();
        assert!((rate - 0.5).abs() < 1e-10);
    }
    #[test]
    fn test_derivation_context_clear() {
        let mut ctx = DerivationContext::new();
        let _ = ctx.lookup(&Name::str("BEq"), &Name::str("Foo"));
        ctx.clear();
        assert_eq!(ctx.cache_hits, 0);
        assert_eq!(ctx.cache_misses, 0);
    }
    #[test]
    fn test_derivation_plan_execute_success() {
        let ti = simple_color();
        let plan = DerivationPlan::new(
            ti,
            vec![
                DerivableClass::BEq,
                DerivableClass::Repr,
                DerivableClass::Inhabited,
            ],
        );
        let (results, errors) = plan.execute();
        assert_eq!(results.len(), 3);
        assert!(errors.is_empty());
    }
    #[test]
    fn test_derivation_plan_fail_fast() {
        let ti = TypeInfo {
            name: Name::str("Recursive"),
            univ_params: vec![],
            params: vec![],
            constructors: vec![ConstructorInfo::new(
                Name::str("Recursive.mk"),
                vec![(
                    Name::str("self"),
                    Expr::Const(Name::str("Recursive"), vec![]),
                )],
                0,
            )],
            is_recursive: true,
            num_indices: 0,
        };
        let plan = DerivationPlan::new(
            ti,
            vec![
                DerivableClass::BEq,
                DerivableClass::Repr,
                DerivableClass::Inhabited,
            ],
        )
        .with_fail_fast(true);
        let (results, errors) = plan.execute();
        assert_eq!(errors.len(), 1);
        assert!(results.is_empty());
    }
    #[test]
    fn test_derivation_plan_no_fail_fast() {
        let ti = TypeInfo {
            name: Name::str("Recursive2"),
            univ_params: vec![],
            params: vec![],
            constructors: vec![ConstructorInfo::new(
                Name::str("Recursive2.mk"),
                vec![(
                    Name::str("self"),
                    Expr::Const(Name::str("Recursive2"), vec![]),
                )],
                0,
            )],
            is_recursive: true,
            num_indices: 0,
        };
        let plan = DerivationPlan::new(
            ti,
            vec![
                DerivableClass::BEq,
                DerivableClass::Repr,
                DerivableClass::Inhabited,
            ],
        )
        .with_fail_fast(false);
        let (results, errors) = plan.execute();
        assert_eq!(errors.len(), 1);
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_structural_eq_deriver() {
        let deriver = StructuralEqDeriver::new();
        let ti = simple_color();
        let result = deriver.derive_structural_eq(&ti);
        assert!(result.is_ok());
        assert!(matches!(
            result.expect("test operation should succeed"),
            Expr::Lam(_, _, _, _)
        ));
    }
    #[test]
    fn test_structural_eq_deriver_empty() {
        let deriver = StructuralEqDeriver::new();
        let ti = TypeInfo {
            name: Name::str("Empty"),
            univ_params: vec![],
            params: vec![],
            constructors: vec![],
            is_recursive: false,
            num_indices: 0,
        };
        assert!(deriver.derive_structural_eq(&ti).is_err());
    }
    #[test]
    fn test_structural_eq_ctor_count() {
        let deriver = StructuralEqDeriver::new();
        let ti = simple_color();
        let expr = deriver.ctor_count_expr(&ti);
        assert!(matches!(expr, Expr::Lit(Literal::Nat(3))));
    }
    #[test]
    fn test_instance_namer_default() {
        let namer = InstanceNamer::default_namer();
        let name = namer.name_for(DerivableClass::BEq, &Name::str("Color"));
        assert_eq!(name, Name::str("inst_BEq_Color"));
    }
    #[test]
    fn test_instance_namer_custom() {
        let namer = InstanceNamer::new("derive", ".");
        let name = namer.name_for_str("Hashable", &Name::str("Point"));
        assert_eq!(name, Name::str("derive.Hashable.Point"));
    }
    #[test]
    fn test_mk_ctor_app() {
        let name = Name::str("Pair.mk");
        let args = vec![Expr::Lit(Literal::Nat(1)), Expr::Lit(Literal::Nat(2))];
        let result = mk_ctor_app(&name, &args);
        assert!(matches!(result, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_eq_prop() {
        let ty = nat_ty();
        let lhs = Expr::BVar(0);
        let rhs = Expr::BVar(1);
        let eq = mk_eq_prop(&ty, &lhs, &rhs);
        assert!(matches!(eq, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_and_type() {
        let a = Expr::Const(Name::str("P"), vec![]);
        let b = Expr::Const(Name::str("Q"), vec![]);
        let and = mk_and_type(&a, &b);
        assert!(matches!(and, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_or_type() {
        let a = Expr::Const(Name::str("P"), vec![]);
        let b = Expr::Const(Name::str("Q"), vec![]);
        let or = mk_or_type(&a, &b);
        assert!(matches!(or, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_not_type() {
        let a = Expr::Const(Name::str("P"), vec![]);
        let not = mk_not_type(&a);
        assert!(matches!(not, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_pi_type() {
        let ty = nat_ty();
        let body = Expr::Sort(Level::zero());
        let pi = mk_pi_type(BinderInfo::Default, &Name::str("x"), &ty, &body);
        assert!(matches!(pi, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_mk_pi_chain() {
        let binders = vec![(Name::str("x"), nat_ty()), (Name::str("y"), bool_ty())];
        let body = Expr::Sort(Level::zero());
        let chain = mk_pi_chain(&binders, &body);
        assert!(matches!(chain, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_mk_lam_chain() {
        let binders = vec![(Name::str("x"), nat_ty()), (Name::str("y"), bool_ty())];
        let body = Expr::BVar(0);
        let chain = mk_lam_chain(&binders, &body);
        assert!(matches!(chain, Expr::Lam(_, _, _, _)));
    }
    #[test]
    fn test_derive_stats() {
        let mut stats = DeriveStats::new();
        assert_eq!(stats.success_rate(), 1.0);
        let dr = DeriveResult {
            instance_name: Name::str("inst"),
            instance_type: Expr::Sort(Level::zero()),
            instance_body: Expr::Sort(Level::zero()),
            aux_defs: vec![],
        };
        stats.record_success(&dr);
        stats.record_failure();
        assert_eq!(stats.attempted, 2);
        assert_eq!(stats.succeeded, 1);
        assert_eq!(stats.failed, 1);
        assert!((stats.success_rate() - 0.5).abs() < 1e-10);
        let s = format!("{}", stats);
        assert!(s.contains("attempted=2"));
    }
    #[test]
    fn test_batch_deriver_for_all() {
        let mut bd = BatchDeriver::new();
        let types = vec![simple_color(), simple_point()];
        let results = bd.derive_for_all(DerivableClass::Repr, &types);
        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|r| r.is_ok()));
        assert_eq!(bd.stats.succeeded, 2);
    }
    #[test]
    fn test_batch_deriver_classes_for() {
        let mut bd = BatchDeriver::new();
        let ti = simple_color();
        let classes = vec![
            DerivableClass::BEq,
            DerivableClass::Repr,
            DerivableClass::Inhabited,
        ];
        let results = bd.derive_classes_for(&classes, &ti);
        assert_eq!(results.len(), 3);
        assert!(results.iter().all(|r| r.is_ok()));
    }
    #[test]
    fn test_batch_deriver_all_standard() {
        let mut bd = BatchDeriver::new();
        let ti = simple_color();
        let results = bd.derive_all_standard(&ti);
        assert!(!results.is_empty());
    }
    #[test]
    fn test_field_analysis_unit() {
        let ctor = ConstructorInfo::new(Name::str("Unit.mk"), vec![], 0);
        let fa = FieldAnalysis::analyze(&ctor);
        assert!(fa.is_unit());
        assert!(!fa.is_newtype());
    }
    #[test]
    fn test_field_analysis_newtype() {
        let ctor = ConstructorInfo::new(
            Name::str("Wrapper.mk"),
            vec![(Name::str("val"), nat_ty())],
            0,
        );
        let fa = FieldAnalysis::analyze(&ctor);
        assert!(fa.is_newtype());
        assert!(!fa.is_unit());
        assert!(fa.all_primitive);
    }
    #[test]
    fn test_field_analysis_non_primitive() {
        let ctor = ConstructorInfo::new(
            Name::str("Node.mk"),
            vec![(Name::str("child"), Expr::Const(Name::str("Tree"), vec![]))],
            0,
        );
        let fa = FieldAnalysis::analyze(&ctor);
        assert!(!fa.all_primitive);
    }
    #[test]
    fn test_type_analysis_enum() {
        let ti = simple_color();
        let ta = TypeAnalysis::analyze(&ti);
        assert!(ta.is_enum);
        assert!(!ta.is_newtype);
        assert!(!ta.is_unit);
        assert_eq!(ta.num_ctors, 3);
    }
    #[test]
    fn test_type_analysis_struct() {
        let ti = simple_point();
        let ta = TypeAnalysis::analyze(&ti);
        assert!(!ta.is_enum);
        assert!(ta.is_newtype || ta.num_ctors == 1);
    }
    #[test]
    fn test_type_analysis_derivable_classes() {
        let ti = simple_color();
        let ta = TypeAnalysis::analyze(&ti);
        let classes = ta.definitely_derivable_classes();
        assert!(classes.contains(&DerivableClass::BEq));
        assert!(classes.contains(&DerivableClass::Repr));
        assert!(classes.contains(&DerivableClass::Inhabited));
    }
    #[test]
    fn test_is_primitive_type() {
        assert!(is_primitive_type(&nat_ty()));
        assert!(is_primitive_type(&bool_ty()));
        assert!(is_primitive_type(&string_ty()));
        assert!(!is_primitive_type(&Expr::Const(Name::str("Tree"), vec![])));
    }
    #[test]
    fn test_derive_command_parse() {
        let cmd = DeriveCommand::new(
            Name::str("Color"),
            vec!["BEq".to_string(), "Repr".to_string(), "Unknown".to_string()],
        );
        let parsed = cmd.parse_classes();
        assert_eq!(parsed.len(), 3);
        assert!(parsed[0].is_some());
        assert!(parsed[1].is_some());
        assert!(parsed[2].is_none());
        let unknown = cmd.unknown_classes();
        assert_eq!(unknown, vec!["Unknown"]);
        assert!(!cmd.all_known());
    }
    #[test]
    fn test_derive_command_all_known() {
        let cmd = DeriveCommand::new(
            Name::str("Point"),
            vec!["BEq".to_string(), "Repr".to_string()],
        );
        assert!(cmd.all_known());
    }
    #[test]
    fn test_mk_ne_prop() {
        let ty = nat_ty();
        let lhs = Expr::BVar(0);
        let rhs = Expr::BVar(1);
        let ne = mk_ne_prop(&ty, &lhs, &rhs);
        assert!(matches!(ne, Expr::App(_, _)));
    }
    #[test]
    fn test_deriver_old_api_cannot_derive_without_type_info() {
        let d = Deriver::new();
        let ty = Expr::Const(Name::str("Nat"), vec![]);
        let result = d.derive(DerivableClass::BEq, &ty);
        assert!(result.is_err());
        let msg = result.unwrap_err();
        assert!(msg.contains("derive_with_info"));
    }
    #[test]
    fn test_deriver_can_derive_all_classes() {
        let d = Deriver::new();
        let ty = Expr::Const(Name::str("Nat"), vec![]);
        for class in [
            DerivableClass::Eq,
            DerivableClass::Ord,
            DerivableClass::Show,
            DerivableClass::Default,
            DerivableClass::BEq,
            DerivableClass::Repr,
            DerivableClass::Hashable,
            DerivableClass::Inhabited,
            DerivableClass::DecidableEq,
            DerivableClass::Nonempty,
            DerivableClass::ToString,
        ] {
            assert!(d.can_derive(class, &ty));
        }
    }
    #[test]
    fn test_mk_field_proj() {
        let base = Expr::BVar(0);
        let field = Name::str("x");
        let proj = mk_field_proj(&base, 0, &field);
        assert!(matches!(proj, Expr::Proj(_, 0, _)));
    }
    #[test]
    fn test_derive_registry_all_classes() {
        let registry = DeriveRegistry::new();
        let classes = registry.all_classes();
        assert!(classes.len() >= 11);
        assert!(registry.is_derivable(DerivableClass::BEq));
        assert!(registry.is_derivable(DerivableClass::DecidableEq));
    }
    #[test]
    fn test_derive_registry_custom_deriver() {
        let mut registry = DeriveRegistry::new();
        let _ti = simple_color();
        registry.register_custom_deriver(Name::str("MyClass"), |ti| {
            Ok(DeriveResult {
                instance_name: Name::str(format!("custom_{}", ti.name)),
                instance_type: Expr::Const(Name::str("MyClass"), vec![]),
                instance_body: Expr::Sort(Level::zero()),
                aux_defs: vec![],
            })
        });
        assert_eq!(registry.num_custom_derivers(), 1);
        assert!(registry.has_deriver(&Name::str("MyClass")));
    }
    #[test]
    fn test_derive_registry_derive_all() {
        let registry = DeriveRegistry::new();
        let ti = simple_color();
        let results = registry.derive_all(&[DerivableClass::BEq, DerivableClass::Repr], &ti);
        assert!(results.is_ok());
        let results = results.expect("test operation should succeed");
        assert_eq!(results.len(), 2);
    }
}
