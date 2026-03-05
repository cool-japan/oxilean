//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Environment, Expr, FVarId, Level, Name};
use oxilean_parse::Decl;
use std::collections::HashMap;

use super::types::{
    FieldInfo, FlattenedStructure, ProjectionDecl, StructElabError, StructUpdateBuilder,
    StructureElaborator, StructureInfo, StructureStats,
};

/// Elaborate a list of field declarations into `FieldInfo` entries.
#[allow(dead_code)]
pub fn elaborate_fields(
    struct_name: &Name,
    fields: &[oxilean_parse::FieldDecl],
) -> Result<Vec<FieldInfo>, StructElabError> {
    let mut result = Vec::new();
    let mut seen_names: Vec<String> = Vec::new();
    for (idx, field) in fields.iter().enumerate() {
        if seen_names.contains(&field.name) {
            return Err(StructElabError::DuplicateField(format!(
                "duplicate field '{}' in structure '{}'",
                field.name, struct_name
            )));
        }
        seen_names.push(field.name.clone());
        let field_ty = surface_to_placeholder_type(&field.ty.value);
        let default_val = field
            .default
            .as_ref()
            .map(|d| surface_to_placeholder_type(&d.value));
        let field_name = Name::str(&field.name);
        let proj_name = Name::str(format!("{}.{}", struct_name, field.name));
        result.push(FieldInfo {
            name: field_name,
            ty: field_ty,
            binder_info: BinderInfo::Default,
            default_val,
            proj_name,
            idx,
            is_inherited: false,
            from_parent: None,
        });
    }
    Ok(result)
}
/// Convert a surface expression to a best-effort kernel expression.
///
/// Handles the most common surface expressions structurally. For complex
/// cases that require full elaboration (e.g., dependent types, type-class
/// resolution), falls back to a `Type` sort placeholder.
pub(super) fn surface_to_placeholder_type(surf: &oxilean_parse::SurfaceExpr) -> Expr {
    use oxilean_parse::{BinderKind, SortKind, SurfaceExpr as SE};
    match surf {
        SE::Var(name) => Expr::Const(Name::str(name), Vec::new()),
        SE::Sort(SortKind::Type) | SE::Sort(SortKind::TypeU(_)) => {
            Expr::Sort(Level::succ(Level::zero()))
        }
        SE::Sort(SortKind::Prop) | SE::Sort(SortKind::SortU(_)) => Expr::Sort(Level::zero()),
        SE::Lit(oxilean_parse::Literal::Nat(n)) => Expr::Lit(oxilean_kernel::Literal::Nat(*n)),
        SE::Lit(oxilean_parse::Literal::String(s)) => {
            Expr::Lit(oxilean_kernel::Literal::Str(s.clone()))
        }
        SE::App(f, a) => {
            let f_expr = surface_to_placeholder_type(&f.value);
            let a_expr = surface_to_placeholder_type(&a.value);
            Expr::App(Box::new(f_expr), Box::new(a_expr))
        }
        SE::Pi(binders, body) => {
            let body_expr = surface_to_placeholder_type(&body.value);
            binders.iter().rev().fold(body_expr, |acc, b| {
                let dom =
                    b.ty.as_ref()
                        .map(|t| surface_to_placeholder_type(&t.value))
                        .unwrap_or_else(|| Expr::Sort(Level::succ(Level::zero())));
                let info = match b.info {
                    BinderKind::Implicit | BinderKind::StrictImplicit => {
                        oxilean_kernel::BinderInfo::Implicit
                    }
                    BinderKind::Instance => oxilean_kernel::BinderInfo::InstImplicit,
                    BinderKind::Default => oxilean_kernel::BinderInfo::Default,
                };
                Expr::Pi(info, Name::str(&b.name), Box::new(dom), Box::new(acc))
            })
        }
        SE::Lam(binders, body) => {
            let body_expr = surface_to_placeholder_type(&body.value);
            binders.iter().rev().fold(body_expr, |acc, b| {
                let dom =
                    b.ty.as_ref()
                        .map(|t| surface_to_placeholder_type(&t.value))
                        .unwrap_or_else(|| Expr::Sort(Level::succ(Level::zero())));
                let info = match b.info {
                    BinderKind::Implicit | BinderKind::StrictImplicit => {
                        oxilean_kernel::BinderInfo::Implicit
                    }
                    BinderKind::Instance => oxilean_kernel::BinderInfo::InstImplicit,
                    BinderKind::Default => oxilean_kernel::BinderInfo::Default,
                };
                Expr::Lam(info, Name::str(&b.name), Box::new(dom), Box::new(acc))
            })
        }
        SE::Ann(e, _ty) => surface_to_placeholder_type(&e.value),
        SE::Proj(e, field) => {
            let e_expr = surface_to_placeholder_type(&e.value);
            Expr::Proj(Name::str(field), 0, Box::new(e_expr))
        }
        SE::Hole => Expr::Sort(Level::succ(Level::zero())),
        _ => Expr::Sort(Level::succ(Level::zero())),
    }
}
/// Extract the head constant name from a type expression.
///
/// For example, `App(App(Const("Foo"), a), b)` yields `Some(Name::str("Foo"))`.
#[allow(dead_code)]
pub(super) fn head_const_name(expr: &Expr) -> Option<Name> {
    match expr {
        Expr::Const(name, _) => Some(name.clone()),
        Expr::App(fun, _) => head_const_name(fun),
        _ => None,
    }
}
/// Count the number of fields in a structure by name.
#[allow(dead_code)]
pub(super) fn field_count(elab: &StructureElaborator<'_>, name: &Name) -> usize {
    elab.get_fields(name).len()
}
/// Check if all fields have default values.
#[allow(dead_code)]
pub(super) fn all_fields_have_defaults(info: &StructureInfo) -> bool {
    info.fields.iter().all(|f| f.default_val.is_some())
}
/// Get the names of all own (non-inherited) fields.
#[allow(dead_code)]
pub(super) fn own_field_names(info: &StructureInfo) -> Vec<Name> {
    info.fields
        .iter()
        .filter(|f| !f.is_inherited)
        .map(|f| f.name.clone())
        .collect()
}
/// Get the names of all inherited fields.
#[allow(dead_code)]
pub(super) fn inherited_field_names(info: &StructureInfo) -> Vec<Name> {
    info.fields
        .iter()
        .filter(|f| f.is_inherited)
        .map(|f| f.name.clone())
        .collect()
}
/// Check whether a structure has any parent structures.
#[allow(dead_code)]
pub(super) fn has_parents(info: &StructureInfo) -> bool {
    !info.parent_structs.is_empty()
}
/// Collect all ancestor structures transitively.
#[allow(dead_code)]
pub(super) fn all_ancestors(elab: &StructureElaborator<'_>, name: &Name) -> Vec<Name> {
    let mut result = Vec::new();
    let mut work = vec![name.clone()];
    let mut visited = Vec::new();
    while let Some(current) = work.pop() {
        if visited.contains(&current) {
            continue;
        }
        visited.push(current.clone());
        if let Some(info) = elab.lookup_structure(&current) {
            for parent in &info.parent_structs {
                result.push(parent.clone());
                work.push(parent.clone());
            }
        }
    }
    result
}
/// Build a mapping from field name to field index.
#[allow(dead_code)]
pub(super) fn field_index_map(info: &StructureInfo) -> HashMap<Name, usize> {
    info.fields
        .iter()
        .map(|f| (f.name.clone(), f.idx))
        .collect()
}
/// Build a mapping from projection name to field info.
#[allow(dead_code)]
pub(super) fn proj_to_field_map(info: &StructureInfo) -> HashMap<Name, &FieldInfo> {
    info.fields
        .iter()
        .map(|f| (f.proj_name.clone(), f))
        .collect()
}
/// Build a default-filled constructor call.
///
/// Produces `mk default_1 default_2 ...` using default values where available
/// and placeholder metavariables (represented as `FVar` with high ID) where not.
#[allow(dead_code)]
pub(super) fn default_ctor_call(info: &StructureInfo) -> Expr {
    let mut result = Expr::Const(info.ctor_name.clone(), Vec::new());
    for (i, field) in info.fields.iter().enumerate() {
        let arg = field
            .default_val
            .clone()
            .unwrap_or_else(|| Expr::FVar(FVarId(2_000_000 + i as u64)));
        result = Expr::App(Box::new(result), Box::new(arg));
    }
    result
}
/// Validate that a set of update field names are all valid.
#[allow(dead_code)]
pub fn validate_update_fields(
    info: &StructureInfo,
    update_names: &[Name],
) -> Result<(), StructElabError> {
    for uname in update_names {
        if !info.fields.iter().any(|f| &f.name == uname) {
            return Err(StructElabError::Other(format!(
                "unknown field '{}' in structure '{}'",
                uname, info.name
            )));
        }
    }
    Ok(())
}
/// Generate all projection declarations including inherited ones.
#[allow(dead_code)]
pub(super) fn all_projection_decls(
    elab: &StructureElaborator<'_>,
    info: &StructureInfo,
) -> Vec<ProjectionDecl> {
    elab.generate_projections(info)
}
/// Compute the flattened list of fields including transitive parents.
#[allow(dead_code)]
pub(super) fn flattened_fields(elab: &StructureElaborator<'_>, name: &Name) -> Vec<FieldInfo> {
    let mut result = Vec::new();
    let ancestors = all_ancestors(elab, name);
    for ancestor in &ancestors {
        if let Some(info) = elab.lookup_structure(ancestor) {
            for field in &info.fields {
                if !result.iter().any(|f: &FieldInfo| f.name == field.name) {
                    let mut inherited = field.clone();
                    inherited.is_inherited = true;
                    inherited.from_parent = Some(ancestor.clone());
                    result.push(inherited);
                }
            }
        }
    }
    if let Some(info) = elab.lookup_structure(name) {
        for field in &info.fields {
            if !result.iter().any(|f: &FieldInfo| f.name == field.name) {
                result.push(field.clone());
            }
        }
    }
    result
}
/// Determine if two structures are in an inheritance relationship.
#[allow(dead_code)]
pub(super) fn is_ancestor(
    elab: &StructureElaborator<'_>,
    potential_ancestor: &Name,
    descendant: &Name,
) -> bool {
    all_ancestors(elab, descendant).contains(potential_ancestor)
}
/// Get the depth of the inheritance chain for a structure.
#[allow(dead_code)]
pub(super) fn inheritance_depth(elab: &StructureElaborator<'_>, name: &Name) -> usize {
    match elab.lookup_structure(name) {
        Some(info) if !info.parent_structs.is_empty() => {
            1 + info
                .parent_structs
                .iter()
                .map(|p| inheritance_depth(elab, p))
                .max()
                .unwrap_or(0)
        }
        _ => 0,
    }
}
/// Generate a `to_parent` coercion function.
///
/// For structure `Child extends Parent`, generate a function:
/// `Child.toParent : Child -> Parent := fun c => Parent.mk c.field1 c.field2 ...`
#[allow(dead_code)]
pub(super) fn generate_to_parent_coercion(
    elab: &StructureElaborator<'_>,
    child_name: &Name,
    parent_name: &Name,
) -> Option<Expr> {
    let child_info = elab.lookup_structure(child_name)?;
    let parent_info = elab.lookup_structure(parent_name)?;
    let child_ty = Expr::Const(child_name.clone(), Vec::new());
    let mut body = Expr::Const(parent_info.ctor_name.clone(), Vec::new());
    for parent_field in &parent_info.fields {
        if let Some(child_field) = child_info
            .fields
            .iter()
            .find(|f| f.name == parent_field.name)
        {
            let proj = Expr::Proj(
                child_name.clone(),
                child_field.idx as u32,
                Box::new(Expr::BVar(0)),
            );
            body = Expr::App(Box::new(body), Box::new(proj));
        } else {
            return None;
        }
    }
    Some(Expr::Lam(
        BinderInfo::Default,
        Name::str("self"),
        Box::new(child_ty),
        Box::new(body),
    ))
}
/// Build a field-by-field equality proposition for two structure values.
///
/// Produces `And (a.f1 = b.f1) (And (a.f2 = b.f2) ... True)`.
#[allow(dead_code)]
pub(super) fn field_wise_eq(struct_name: &Name, info: &StructureInfo, a: &Expr, b: &Expr) -> Expr {
    let mut result = Expr::Const(Name::str("True"), Vec::new());
    for field in info.fields.iter().rev() {
        let proj_a = Expr::Proj(struct_name.clone(), field.idx as u32, Box::new(a.clone()));
        let proj_b = Expr::Proj(struct_name.clone(), field.idx as u32, Box::new(b.clone()));
        let eq = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Eq"), Vec::new())),
                    Box::new(field.ty.clone()),
                )),
                Box::new(proj_a),
            )),
            Box::new(proj_b),
        );
        result = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("And"), Vec::new())),
                Box::new(eq),
            )),
            Box::new(result),
        );
    }
    result
}
/// Check if a field has instance-implicit binder info.
#[allow(dead_code)]
pub(super) fn is_instance_field(field: &FieldInfo) -> bool {
    field.binder_info == BinderInfo::InstImplicit
}
/// Check if a field has implicit binder info.
#[allow(dead_code)]
pub(super) fn is_implicit_field(field: &FieldInfo) -> bool {
    field.binder_info == BinderInfo::Implicit
}
/// Count the number of own (non-inherited) fields.
#[allow(dead_code)]
pub(super) fn own_field_count(info: &StructureInfo) -> usize {
    info.fields.iter().filter(|f| !f.is_inherited).count()
}
/// Count the number of inherited fields.
#[allow(dead_code)]
pub(super) fn inherited_field_count(info: &StructureInfo) -> usize {
    info.fields.iter().filter(|f| f.is_inherited).count()
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::structure::*;
    use oxilean_parse::{FieldDecl, Located, SortKind, Span, SurfaceExpr};
    fn mk_span() -> Span {
        Span::new(0, 1, 1, 1)
    }
    fn mk_type_expr() -> Located<SurfaceExpr> {
        Located::new(SurfaceExpr::Sort(SortKind::Type), mk_span())
    }
    fn mk_var_expr(name: &str) -> Located<SurfaceExpr> {
        Located::new(SurfaceExpr::Var(name.to_string()), mk_span())
    }
    fn mk_field(name: &str, ty: Located<SurfaceExpr>) -> FieldDecl {
        FieldDecl {
            name: name.to_string(),
            ty,
            default: None,
        }
    }
    fn mk_field_with_default(
        name: &str,
        ty: Located<SurfaceExpr>,
        default: Located<SurfaceExpr>,
    ) -> FieldDecl {
        FieldDecl {
            name: name.to_string(),
            ty,
            default: Some(default),
        }
    }
    fn mk_structure_decl(name: &str, fields: Vec<FieldDecl>) -> Decl {
        Decl::Structure {
            name: name.to_string(),
            univ_params: Vec::new(),
            extends: Vec::new(),
            fields,
        }
    }
    fn mk_structure_extends(name: &str, extends: Vec<&str>, fields: Vec<FieldDecl>) -> Decl {
        Decl::Structure {
            name: name.to_string(),
            univ_params: Vec::new(),
            extends: extends.into_iter().map(String::from).collect(),
            fields,
        }
    }
    fn mk_class_decl(name: &str, fields: Vec<FieldDecl>) -> Decl {
        Decl::ClassDecl {
            name: name.to_string(),
            univ_params: Vec::new(),
            extends: Vec::new(),
            fields,
        }
    }
    fn mk_class_extends(name: &str, extends: Vec<&str>, fields: Vec<FieldDecl>) -> Decl {
        Decl::ClassDecl {
            name: name.to_string(),
            univ_params: Vec::new(),
            extends: extends.into_iter().map(String::from).collect(),
            fields,
        }
    }
    #[test]
    fn test_elaborate_empty_structure() {
        let env = Environment::new();
        let mut elab = StructureElaborator::new(&env);
        let decl = mk_structure_decl("Empty", vec![]);
        let info = elab
            .elaborate_structure(&decl)
            .expect("elaboration should succeed");
        assert_eq!(info.name, Name::str("Empty"));
        assert!(info.fields.is_empty());
        assert!(!info.is_class);
    }
    #[test]
    fn test_elaborate_structure_with_fields() {
        let env = Environment::new();
        let mut elab = StructureElaborator::new(&env);
        let decl = mk_structure_decl(
            "Point",
            vec![
                mk_field("x", mk_var_expr("Nat")),
                mk_field("y", mk_var_expr("Nat")),
            ],
        );
        let info = elab
            .elaborate_structure(&decl)
            .expect("elaboration should succeed");
        assert_eq!(info.fields.len(), 2);
        assert_eq!(info.fields[0].name, Name::str("x"));
        assert_eq!(info.fields[1].name, Name::str("y"));
        assert_eq!(info.ctor_name, Name::str("Point.mk"));
    }
    #[test]
    fn test_elaborate_structure_duplicate_field() {
        let env = Environment::new();
        let mut elab = StructureElaborator::new(&env);
        let decl = mk_structure_decl(
            "Bad",
            vec![
                mk_field("x", mk_var_expr("Nat")),
                mk_field("x", mk_var_expr("Nat")),
            ],
        );
        let result = elab.elaborate_structure(&decl);
        assert!(result.is_err());
        match result.unwrap_err() {
            StructElabError::DuplicateField(msg) => assert!(msg.contains("x")),
            other => panic!("expected DuplicateField, got {:?}", other),
        }
    }
    #[test]
    fn test_elaborate_structure_field_default() {
        let env = Environment::new();
        let mut elab = StructureElaborator::new(&env);
        let nat_zero = Located::new(SurfaceExpr::Lit(oxilean_parse::Literal::Nat(0)), mk_span());
        let decl = mk_structure_decl(
            "Config",
            vec![mk_field_with_default("width", mk_var_expr("Nat"), nat_zero)],
        );
        let info = elab
            .elaborate_structure(&decl)
            .expect("elaboration should succeed");
        assert!(info.fields[0].default_val.is_some());
    }
    #[test]
    fn test_elaborate_structure_univ_params() {
        let env = Environment::new();
        let mut elab = StructureElaborator::new(&env);
        let decl = Decl::Structure {
            name: "Container".to_string(),
            univ_params: vec!["u".to_string(), "v".to_string()],
            extends: Vec::new(),
            fields: vec![mk_field("val", mk_type_expr())],
        };
        let info = elab
            .elaborate_structure(&decl)
            .expect("elaboration should succeed");
        assert_eq!(info.univ_params.len(), 2);
        assert_eq!(info.univ_params[0], Name::str("u"));
    }
    #[test]
    fn test_elaborate_structure_with_parent() {
        let env = Environment::new();
        let mut elab = StructureElaborator::new(&env);
        let parent_decl = mk_structure_decl("Base", vec![mk_field("id", mk_var_expr("Nat"))]);
        let parent_info = elab
            .elaborate_structure(&parent_decl)
            .expect("elaboration should succeed");
        elab.register_structure(parent_info);
        let child_decl = mk_structure_extends(
            "Child",
            vec!["Base"],
            vec![mk_field("extra", mk_var_expr("Nat"))],
        );
        let child_info = elab
            .elaborate_structure(&child_decl)
            .expect("elaboration should succeed");
        assert_eq!(child_info.fields.len(), 2);
        assert!(child_info.fields[0].is_inherited);
        assert_eq!(child_info.fields[0].name, Name::str("id"));
        assert!(!child_info.fields[1].is_inherited);
        assert_eq!(child_info.fields[1].name, Name::str("extra"));
        assert_eq!(child_info.parent_structs.len(), 1);
    }
    #[test]
    fn test_elaborate_structure_parent_not_found() {
        let env = Environment::new();
        let mut elab = StructureElaborator::new(&env);
        let decl = mk_structure_extends(
            "Child",
            vec!["NonExistent"],
            vec![mk_field("x", mk_var_expr("Nat"))],
        );
        let result = elab.elaborate_structure(&decl);
        assert!(result.is_err());
        match result.unwrap_err() {
            StructElabError::ParentNotFound(msg) => assert!(msg.contains("NonExistent")),
            other => panic!("expected ParentNotFound, got {:?}", other),
        }
    }
    #[test]
    fn test_elaborate_structure_duplicate_with_parent() {
        let env = Environment::new();
        let mut elab = StructureElaborator::new(&env);
        let parent_decl = mk_structure_decl("Base", vec![mk_field("id", mk_var_expr("Nat"))]);
        let parent_info = elab
            .elaborate_structure(&parent_decl)
            .expect("elaboration should succeed");
        elab.register_structure(parent_info);
        let child_decl = mk_structure_extends(
            "Child",
            vec!["Base"],
            vec![mk_field("id", mk_var_expr("Nat"))],
        );
        let result = elab.elaborate_structure(&child_decl);
        assert!(result.is_err());
        match result.unwrap_err() {
            StructElabError::DuplicateField(msg) => assert!(msg.contains("id")),
            other => panic!("expected DuplicateField, got {:?}", other),
        }
    }
    #[test]
    fn test_elaborate_class() {
        let env = Environment::new();
        let mut elab = StructureElaborator::new(&env);
        let decl = mk_class_decl("Functor", vec![mk_field("map", mk_var_expr("Nat"))]);
        let info = elab
            .elaborate_class(&decl)
            .expect("elaboration should succeed");
        assert!(info.is_class);
        assert_eq!(info.name, Name::str("Functor"));
        assert_eq!(info.fields.len(), 1);
    }
    #[test]
    fn test_elaborate_class_extends_class() {
        let env = Environment::new();
        let mut elab = StructureElaborator::new(&env);
        let parent_class = mk_class_decl("Functor", vec![mk_field("map", mk_var_expr("Nat"))]);
        let parent_info = elab
            .elaborate_class(&parent_class)
            .expect("elaboration should succeed");
        elab.register_structure(parent_info);
        let child_class = mk_class_extends(
            "Applicative",
            vec!["Functor"],
            vec![mk_field("pure", mk_var_expr("Nat"))],
        );
        let child_info = elab
            .elaborate_class(&child_class)
            .expect("elaboration should succeed");
        assert!(child_info.is_class);
        assert_eq!(child_info.fields.len(), 2);
        assert!(child_info.fields[0].is_inherited);
    }
    #[test]
    fn test_elaborate_class_extends_non_class_fails() {
        let env = Environment::new();
        let mut elab = StructureElaborator::new(&env);
        let parent_struct = mk_structure_decl("Base", vec![mk_field("x", mk_var_expr("Nat"))]);
        let parent_info = elab
            .elaborate_structure(&parent_struct)
            .expect("elaboration should succeed");
        elab.register_structure(parent_info);
        let child_class = mk_class_extends(
            "MyClass",
            vec!["Base"],
            vec![mk_field("y", mk_var_expr("Nat"))],
        );
        let result = elab.elaborate_class(&child_class);
        assert!(result.is_err());
        match result.unwrap_err() {
            StructElabError::InvalidClass(msg) => assert!(msg.contains("Base")),
            other => panic!("expected InvalidClass, got {:?}", other),
        }
    }
    #[test]
    fn test_generate_projections() {
        let env = Environment::new();
        let mut elab = StructureElaborator::new(&env);
        let decl = mk_structure_decl(
            "Point",
            vec![
                mk_field("x", mk_var_expr("Nat")),
                mk_field("y", mk_var_expr("Nat")),
            ],
        );
        let info = elab
            .elaborate_structure(&decl)
            .expect("elaboration should succeed");
        let projs = elab.generate_projections(&info);
        assert_eq!(projs.len(), 2);
        assert_eq!(projs[0].name, Name::str("Point.x"));
        assert_eq!(projs[0].field_idx, 0);
        assert_eq!(projs[1].name, Name::str("Point.y"));
        assert_eq!(projs[1].field_idx, 1);
    }
    #[test]
    fn test_projection_types() {
        let env = Environment::new();
        let mut elab = StructureElaborator::new(&env);
        let decl = mk_structure_decl("Single", vec![mk_field("val", mk_var_expr("Nat"))]);
        let info = elab
            .elaborate_structure(&decl)
            .expect("elaboration should succeed");
        let projs = elab.generate_projections(&info);
        assert!(projs[0].ty.is_pi());
    }
    #[test]
    fn test_generate_constructor() {
        let env = Environment::new();
        let mut elab = StructureElaborator::new(&env);
        let decl = mk_structure_decl(
            "Pair",
            vec![
                mk_field("fst", mk_var_expr("Nat")),
                mk_field("snd", mk_var_expr("Nat")),
            ],
        );
        let info = elab
            .elaborate_structure(&decl)
            .expect("elaboration should succeed");
        let ctor = elab.generate_constructor(&info);
        assert_eq!(ctor.name, Name::str("Pair.mk"));
        assert_eq!(ctor.num_fields, 2);
    }
    #[test]
    fn test_constructor_type_is_pi() {
        let env = Environment::new();
        let mut elab = StructureElaborator::new(&env);
        let decl = mk_structure_decl("Wrapper", vec![mk_field("inner", mk_var_expr("Nat"))]);
        let info = elab
            .elaborate_structure(&decl)
            .expect("elaboration should succeed");
        let ctor = elab.generate_constructor(&info);
        assert!(ctor.ty.is_pi());
    }
    #[test]
    fn test_resolve_anonymous_ctor() {
        let env = Environment::new();
        let mut elab = StructureElaborator::new(&env);
        let decl = mk_structure_decl(
            "Point",
            vec![
                mk_field("x", mk_var_expr("Nat")),
                mk_field("y", mk_var_expr("Nat")),
            ],
        );
        let info = elab
            .elaborate_structure(&decl)
            .expect("elaboration should succeed");
        elab.register_structure(info);
        let args = vec![
            Expr::Lit(oxilean_kernel::Literal::Nat(1)),
            Expr::Lit(oxilean_kernel::Literal::Nat(2)),
        ];
        let result = elab.resolve_anonymous_ctor(&Name::str("Point"), &args);
        assert!(result.is_ok());
    }
    #[test]
    fn test_resolve_anonymous_ctor_wrong_count() {
        let env = Environment::new();
        let mut elab = StructureElaborator::new(&env);
        let decl = mk_structure_decl(
            "Point",
            vec![
                mk_field("x", mk_var_expr("Nat")),
                mk_field("y", mk_var_expr("Nat")),
            ],
        );
        let info = elab
            .elaborate_structure(&decl)
            .expect("elaboration should succeed");
        elab.register_structure(info);
        let args = vec![Expr::Lit(oxilean_kernel::Literal::Nat(1))];
        let result = elab.resolve_anonymous_ctor(&Name::str("Point"), &args);
        assert!(result.is_err());
    }
    #[test]
    fn test_get_parent_fields() {
        let env = Environment::new();
        let mut elab = StructureElaborator::new(&env);
        let parent_decl =
            mk_structure_decl("Animal", vec![mk_field("name", mk_var_expr("String"))]);
        let parent_info = elab
            .elaborate_structure(&parent_decl)
            .expect("elaboration should succeed");
        elab.register_structure(parent_info);
        let child_decl = mk_structure_extends(
            "Dog",
            vec!["Animal"],
            vec![mk_field("breed", mk_var_expr("String"))],
        );
        let child_info = elab
            .elaborate_structure(&child_decl)
            .expect("elaboration should succeed");
        elab.register_structure(child_info);
        let parent_fields = elab.get_parent_fields(&Name::str("Dog"));
        assert_eq!(parent_fields.len(), 1);
        assert_eq!(parent_fields[0].name, Name::str("name"));
    }
    #[test]
    fn test_is_structure_and_is_class() {
        let env = Environment::new();
        let mut elab = StructureElaborator::new(&env);
        let struct_decl = mk_structure_decl("Foo", vec![mk_field("x", mk_var_expr("Nat"))]);
        let struct_info = elab
            .elaborate_structure(&struct_decl)
            .expect("elaboration should succeed");
        elab.register_structure(struct_info);
        let class_decl = mk_class_decl("Bar", vec![mk_field("y", mk_var_expr("Nat"))]);
        let class_info = elab
            .elaborate_class(&class_decl)
            .expect("elaboration should succeed");
        elab.register_structure(class_info);
        assert!(elab.is_structure(&Name::str("Foo")));
        assert!(!elab.is_class(&Name::str("Foo")));
        assert!(elab.is_structure(&Name::str("Bar")));
        assert!(elab.is_class(&Name::str("Bar")));
        assert!(!elab.is_structure(&Name::str("Unknown")));
    }
    #[test]
    fn test_circular_inheritance_direct() {
        let env = Environment::new();
        let mut elab = StructureElaborator::new(&env);
        let a_info = StructureInfo {
            name: Name::str("A"),
            univ_params: Vec::new(),
            params: Vec::new(),
            fields: Vec::new(),
            parent_structs: vec![Name::str("B")],
            is_class: false,
            ctor_name: Name::str("A.mk"),
            mk_name: Name::str("A.mk"),
        };
        elab.register_structure(a_info);
        let b_info = StructureInfo {
            name: Name::str("B"),
            univ_params: Vec::new(),
            params: Vec::new(),
            fields: Vec::new(),
            parent_structs: vec![Name::str("A")],
            is_class: false,
            ctor_name: Name::str("B.mk"),
            mk_name: Name::str("B.mk"),
        };
        elab.register_structure(b_info);
        let c_decl = mk_structure_extends("C", vec!["A"], vec![]);
        let d_decl = Decl::Structure {
            name: "B".to_string(),
            univ_params: Vec::new(),
            extends: vec!["A".to_string()],
            fields: Vec::new(),
        };
        let result = elab.elaborate_structure(&d_decl);
        assert!(result.is_err());
        match result.unwrap_err() {
            StructElabError::CircularInheritance(msg) => {
                assert!(msg.contains("circular"));
            }
            other => panic!("expected CircularInheritance, got {:?}", other),
        }
        let result_c = elab.elaborate_structure(&c_decl);
        assert!(result_c.is_err());
    }
    #[test]
    fn test_no_circular_inheritance() {
        let env = Environment::new();
        let mut elab = StructureElaborator::new(&env);
        let a_decl = mk_structure_decl("A", vec![mk_field("a", mk_var_expr("Nat"))]);
        let a_info = elab
            .elaborate_structure(&a_decl)
            .expect("elaboration should succeed");
        elab.register_structure(a_info);
        let b_decl = mk_structure_extends("B", vec!["A"], vec![mk_field("b", mk_var_expr("Nat"))]);
        let b_info = elab
            .elaborate_structure(&b_decl)
            .expect("elaboration should succeed");
        elab.register_structure(b_info);
        let c_decl = mk_structure_extends("C", vec!["B"], vec![mk_field("c", mk_var_expr("Nat"))]);
        let result = elab.elaborate_structure(&c_decl);
        assert!(result.is_ok());
    }
    #[test]
    fn test_default_field_values() {
        let env = Environment::new();
        let mut elab = StructureElaborator::new(&env);
        let default_val =
            Located::new(SurfaceExpr::Lit(oxilean_parse::Literal::Nat(42)), mk_span());
        let decl = mk_structure_decl(
            "Config",
            vec![
                mk_field_with_default("timeout", mk_var_expr("Nat"), default_val),
                mk_field("name", mk_var_expr("String")),
            ],
        );
        let info = elab
            .elaborate_structure(&decl)
            .expect("elaboration should succeed");
        assert!(info.fields[0].default_val.is_some());
        assert!(info.fields[1].default_val.is_none());
    }
    #[test]
    fn test_struct_update() {
        let env = Environment::new();
        let mut elab = StructureElaborator::new(&env);
        let decl = mk_structure_decl(
            "Point",
            vec![
                mk_field("x", mk_var_expr("Nat")),
                mk_field("y", mk_var_expr("Nat")),
            ],
        );
        let info = elab
            .elaborate_structure(&decl)
            .expect("elaboration should succeed");
        elab.register_structure(info);
        let base = Expr::FVar(FVarId(99));
        let updates = vec![(Name::str("x"), Expr::Lit(oxilean_kernel::Literal::Nat(10)))];
        let result = elab.elaborate_struct_update(&Name::str("Point"), &base, &updates);
        assert!(result.is_ok());
    }
    #[test]
    fn test_struct_update_invalid_field() {
        let env = Environment::new();
        let mut elab = StructureElaborator::new(&env);
        let decl = mk_structure_decl("Point", vec![mk_field("x", mk_var_expr("Nat"))]);
        let info = elab
            .elaborate_structure(&decl)
            .expect("elaboration should succeed");
        elab.register_structure(info);
        let base = Expr::FVar(FVarId(99));
        let updates = vec![(Name::str("z"), Expr::Lit(oxilean_kernel::Literal::Nat(0)))];
        let result = elab.elaborate_struct_update(&Name::str("Point"), &base, &updates);
        assert!(result.is_err());
    }
    #[test]
    fn test_eta_expand_struct() {
        let env = Environment::new();
        let mut elab = StructureElaborator::new(&env);
        let decl = mk_structure_decl(
            "Pair",
            vec![
                mk_field("fst", mk_var_expr("Nat")),
                mk_field("snd", mk_var_expr("Nat")),
            ],
        );
        let info = elab
            .elaborate_structure(&decl)
            .expect("elaboration should succeed");
        elab.register_structure(info);
        let e = Expr::FVar(FVarId(42));
        let expanded = elab.eta_expand_struct(&Name::str("Pair"), &e);
        assert!(expanded.is_ok());
        assert!(expanded.expect("macro expansion should succeed").is_app());
    }
    #[test]
    fn test_eta_reduce_struct() {
        let env = Environment::new();
        let mut elab = StructureElaborator::new(&env);
        let decl = mk_structure_decl(
            "Pair",
            vec![
                mk_field("fst", mk_var_expr("Nat")),
                mk_field("snd", mk_var_expr("Nat")),
            ],
        );
        let info = elab
            .elaborate_structure(&decl)
            .expect("elaboration should succeed");
        elab.register_structure(info);
        let base = Expr::FVar(FVarId(42));
        let expanded = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Pair.mk"), Vec::new())),
                Box::new(Expr::Proj(Name::str("Pair"), 0, Box::new(base.clone()))),
            )),
            Box::new(Expr::Proj(Name::str("Pair"), 1, Box::new(base.clone()))),
        );
        let reduced = elab.eta_reduce_struct(&Name::str("Pair"), &expanded);
        assert_eq!(reduced, Some(base));
    }
    #[test]
    fn test_eta_reduce_fails_different_bases() {
        let env = Environment::new();
        let mut elab = StructureElaborator::new(&env);
        let decl = mk_structure_decl(
            "Pair",
            vec![
                mk_field("fst", mk_var_expr("Nat")),
                mk_field("snd", mk_var_expr("Nat")),
            ],
        );
        let info = elab
            .elaborate_structure(&decl)
            .expect("elaboration should succeed");
        elab.register_structure(info);
        let base1 = Expr::FVar(FVarId(42));
        let base2 = Expr::FVar(FVarId(43));
        let expr = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Pair.mk"), Vec::new())),
                Box::new(Expr::Proj(Name::str("Pair"), 0, Box::new(base1))),
            )),
            Box::new(Expr::Proj(Name::str("Pair"), 1, Box::new(base2))),
        );
        let reduced = elab.eta_reduce_struct(&Name::str("Pair"), &expr);
        assert!(reduced.is_none());
    }
    #[test]
    fn test_generate_recursor() {
        let env = Environment::new();
        let mut elab = StructureElaborator::new(&env);
        let decl = mk_structure_decl("Singleton", vec![mk_field("val", mk_var_expr("Nat"))]);
        let info = elab
            .elaborate_structure(&decl)
            .expect("elaboration should succeed");
        let rec = elab.generate_recursor(&info);
        assert_eq!(rec.name, Name::str("Singleton.rec"));
        assert_eq!(rec.num_fields, 1);
    }
    #[test]
    fn test_register_and_lookup() {
        let env = Environment::new();
        let mut elab = StructureElaborator::new(&env);
        let decl = mk_structure_decl("Foo", vec![mk_field("x", mk_var_expr("Nat"))]);
        let info = elab
            .elaborate_structure(&decl)
            .expect("elaboration should succeed");
        elab.register_structure(info);
        assert!(elab.lookup_structure(&Name::str("Foo")).is_some());
        assert!(elab.lookup_structure(&Name::str("Bar")).is_none());
    }
    #[test]
    fn test_get_fields() {
        let env = Environment::new();
        let mut elab = StructureElaborator::new(&env);
        let decl = mk_structure_decl(
            "Thing",
            vec![
                mk_field("a", mk_var_expr("Nat")),
                mk_field("b", mk_var_expr("Nat")),
                mk_field("c", mk_var_expr("Nat")),
            ],
        );
        let info = elab
            .elaborate_structure(&decl)
            .expect("elaboration should succeed");
        elab.register_structure(info);
        let fields = elab.get_fields(&Name::str("Thing"));
        assert_eq!(fields.len(), 3);
    }
    #[test]
    fn test_head_const_name() {
        let e1 = Expr::Const(Name::str("Foo"), Vec::new());
        assert_eq!(head_const_name(&e1), Some(Name::str("Foo")));
        let e2 = Expr::App(
            Box::new(Expr::Const(Name::str("Bar"), Vec::new())),
            Box::new(Expr::BVar(0)),
        );
        assert_eq!(head_const_name(&e2), Some(Name::str("Bar")));
        let e3 = Expr::BVar(0);
        assert_eq!(head_const_name(&e3), None);
    }
    #[test]
    fn test_all_fields_have_defaults() {
        let info_with_defaults = StructureInfo {
            name: Name::str("Test"),
            univ_params: Vec::new(),
            params: Vec::new(),
            fields: vec![FieldInfo {
                name: Name::str("x"),
                ty: Expr::Sort(Level::zero()),
                binder_info: BinderInfo::Default,
                default_val: Some(Expr::Lit(oxilean_kernel::Literal::Nat(0))),
                proj_name: Name::str("Test.x"),
                idx: 0,
                is_inherited: false,
                from_parent: None,
            }],
            parent_structs: Vec::new(),
            is_class: false,
            ctor_name: Name::str("Test.mk"),
            mk_name: Name::str("Test.mk"),
        };
        assert!(all_fields_have_defaults(&info_with_defaults));
        let info_without = StructureInfo {
            name: Name::str("Test2"),
            univ_params: Vec::new(),
            params: Vec::new(),
            fields: vec![FieldInfo {
                name: Name::str("x"),
                ty: Expr::Sort(Level::zero()),
                binder_info: BinderInfo::Default,
                default_val: None,
                proj_name: Name::str("Test2.x"),
                idx: 0,
                is_inherited: false,
                from_parent: None,
            }],
            parent_structs: Vec::new(),
            is_class: false,
            ctor_name: Name::str("Test2.mk"),
            mk_name: Name::str("Test2.mk"),
        };
        assert!(!all_fields_have_defaults(&info_without));
    }
    #[test]
    fn test_own_and_inherited_field_counts() {
        let info = StructureInfo {
            name: Name::str("Mixed"),
            univ_params: Vec::new(),
            params: Vec::new(),
            fields: vec![
                FieldInfo {
                    name: Name::str("a"),
                    ty: Expr::Sort(Level::zero()),
                    binder_info: BinderInfo::Default,
                    default_val: None,
                    proj_name: Name::str("Mixed.a"),
                    idx: 0,
                    is_inherited: true,
                    from_parent: Some(Name::str("Parent")),
                },
                FieldInfo {
                    name: Name::str("b"),
                    ty: Expr::Sort(Level::zero()),
                    binder_info: BinderInfo::Default,
                    default_val: None,
                    proj_name: Name::str("Mixed.b"),
                    idx: 1,
                    is_inherited: false,
                    from_parent: None,
                },
            ],
            parent_structs: vec![Name::str("Parent")],
            is_class: false,
            ctor_name: Name::str("Mixed.mk"),
            mk_name: Name::str("Mixed.mk"),
        };
        assert_eq!(own_field_count(&info), 1);
        assert_eq!(inherited_field_count(&info), 1);
        assert_eq!(own_field_names(&info).len(), 1);
        assert_eq!(inherited_field_names(&info).len(), 1);
    }
    #[test]
    fn test_has_parents() {
        let with_parents = StructureInfo {
            name: Name::str("Child"),
            univ_params: Vec::new(),
            params: Vec::new(),
            fields: Vec::new(),
            parent_structs: vec![Name::str("Parent")],
            is_class: false,
            ctor_name: Name::str("Child.mk"),
            mk_name: Name::str("Child.mk"),
        };
        assert!(has_parents(&with_parents));
        let without_parents = StructureInfo {
            name: Name::str("Root"),
            univ_params: Vec::new(),
            params: Vec::new(),
            fields: Vec::new(),
            parent_structs: Vec::new(),
            is_class: false,
            ctor_name: Name::str("Root.mk"),
            mk_name: Name::str("Root.mk"),
        };
        assert!(!has_parents(&without_parents));
    }
    #[test]
    fn test_field_index_map() {
        let info = StructureInfo {
            name: Name::str("Test"),
            univ_params: Vec::new(),
            params: Vec::new(),
            fields: vec![
                FieldInfo {
                    name: Name::str("a"),
                    ty: Expr::Sort(Level::zero()),
                    binder_info: BinderInfo::Default,
                    default_val: None,
                    proj_name: Name::str("Test.a"),
                    idx: 0,
                    is_inherited: false,
                    from_parent: None,
                },
                FieldInfo {
                    name: Name::str("b"),
                    ty: Expr::Sort(Level::zero()),
                    binder_info: BinderInfo::Default,
                    default_val: None,
                    proj_name: Name::str("Test.b"),
                    idx: 1,
                    is_inherited: false,
                    from_parent: None,
                },
            ],
            parent_structs: Vec::new(),
            is_class: false,
            ctor_name: Name::str("Test.mk"),
            mk_name: Name::str("Test.mk"),
        };
        let map = field_index_map(&info);
        assert_eq!(map.get(&Name::str("a")), Some(&0));
        assert_eq!(map.get(&Name::str("b")), Some(&1));
    }
    #[test]
    fn test_surface_to_placeholder() {
        let t = surface_to_placeholder_type(&SurfaceExpr::Sort(SortKind::Prop));
        assert!(t.is_prop());
        let t2 = surface_to_placeholder_type(&SurfaceExpr::Var("Nat".to_string()));
        assert!(matches!(t2, Expr::Const(_, _)));
    }
    #[test]
    fn test_wrong_decl_type() {
        let env = Environment::new();
        let mut elab = StructureElaborator::new(&env);
        let decl = Decl::Axiom {
            name: "foo".to_string(),
            univ_params: Vec::new(),
            ty: Located::new(SurfaceExpr::Sort(SortKind::Type), mk_span()),
            attrs: Vec::new(),
        };
        assert!(elab.elaborate_structure(&decl).is_err());
        assert!(elab.elaborate_class(&decl).is_err());
    }
    #[test]
    fn test_is_instance_and_implicit_field() {
        let inst_field = FieldInfo {
            name: Name::str("inst"),
            ty: Expr::Sort(Level::zero()),
            binder_info: BinderInfo::InstImplicit,
            default_val: None,
            proj_name: Name::str("T.inst"),
            idx: 0,
            is_inherited: false,
            from_parent: None,
        };
        assert!(is_instance_field(&inst_field));
        assert!(!is_implicit_field(&inst_field));
        let impl_field = FieldInfo {
            name: Name::str("impl"),
            ty: Expr::Sort(Level::zero()),
            binder_info: BinderInfo::Implicit,
            default_val: None,
            proj_name: Name::str("T.impl"),
            idx: 1,
            is_inherited: false,
            from_parent: None,
        };
        assert!(!is_instance_field(&impl_field));
        assert!(is_implicit_field(&impl_field));
    }
    #[test]
    fn test_inheritance_depth() {
        let env = Environment::new();
        let mut elab = StructureElaborator::new(&env);
        let a_decl = mk_structure_decl("A", vec![mk_field("a", mk_var_expr("Nat"))]);
        let a_info = elab
            .elaborate_structure(&a_decl)
            .expect("elaboration should succeed");
        elab.register_structure(a_info);
        let b_decl = mk_structure_extends("B", vec!["A"], vec![mk_field("b", mk_var_expr("Nat"))]);
        let b_info = elab
            .elaborate_structure(&b_decl)
            .expect("elaboration should succeed");
        elab.register_structure(b_info);
        let c_decl = mk_structure_extends("C", vec!["B"], vec![mk_field("c", mk_var_expr("Nat"))]);
        let c_info = elab
            .elaborate_structure(&c_decl)
            .expect("elaboration should succeed");
        elab.register_structure(c_info);
        assert_eq!(inheritance_depth(&elab, &Name::str("A")), 0);
        assert_eq!(inheritance_depth(&elab, &Name::str("B")), 1);
        assert_eq!(inheritance_depth(&elab, &Name::str("C")), 2);
    }
    #[test]
    fn test_all_ancestors() {
        let env = Environment::new();
        let mut elab = StructureElaborator::new(&env);
        let a_decl = mk_structure_decl("A", vec![]);
        let a_info = elab
            .elaborate_structure(&a_decl)
            .expect("elaboration should succeed");
        elab.register_structure(a_info);
        let b_decl = mk_structure_extends("B", vec!["A"], vec![]);
        let b_info = elab
            .elaborate_structure(&b_decl)
            .expect("elaboration should succeed");
        elab.register_structure(b_info);
        let c_decl = mk_structure_extends("C", vec!["B"], vec![]);
        let c_info = elab
            .elaborate_structure(&c_decl)
            .expect("elaboration should succeed");
        elab.register_structure(c_info);
        let ancestors = all_ancestors(&elab, &Name::str("C"));
        assert!(ancestors.contains(&Name::str("B")));
        assert!(ancestors.contains(&Name::str("A")));
    }
    #[test]
    fn test_is_ancestor() {
        let env = Environment::new();
        let mut elab = StructureElaborator::new(&env);
        let a_decl = mk_structure_decl("X", vec![]);
        let a_info = elab
            .elaborate_structure(&a_decl)
            .expect("elaboration should succeed");
        elab.register_structure(a_info);
        let b_decl = mk_structure_extends("Y", vec!["X"], vec![]);
        let b_info = elab
            .elaborate_structure(&b_decl)
            .expect("elaboration should succeed");
        elab.register_structure(b_info);
        assert!(is_ancestor(&elab, &Name::str("X"), &Name::str("Y")));
        assert!(!is_ancestor(&elab, &Name::str("Y"), &Name::str("X")));
    }
    #[test]
    fn test_default_ctor_call() {
        let info = StructureInfo {
            name: Name::str("T"),
            univ_params: Vec::new(),
            params: Vec::new(),
            fields: vec![
                FieldInfo {
                    name: Name::str("a"),
                    ty: Expr::Sort(Level::zero()),
                    binder_info: BinderInfo::Default,
                    default_val: Some(Expr::Lit(oxilean_kernel::Literal::Nat(1))),
                    proj_name: Name::str("T.a"),
                    idx: 0,
                    is_inherited: false,
                    from_parent: None,
                },
                FieldInfo {
                    name: Name::str("b"),
                    ty: Expr::Sort(Level::zero()),
                    binder_info: BinderInfo::Default,
                    default_val: None,
                    proj_name: Name::str("T.b"),
                    idx: 1,
                    is_inherited: false,
                    from_parent: None,
                },
            ],
            parent_structs: Vec::new(),
            is_class: false,
            ctor_name: Name::str("T.mk"),
            mk_name: Name::str("T.mk"),
        };
        let call = default_ctor_call(&info);
        assert!(call.is_app());
    }
    #[test]
    fn test_validate_update_fields() {
        let info = StructureInfo {
            name: Name::str("T"),
            univ_params: Vec::new(),
            params: Vec::new(),
            fields: vec![FieldInfo {
                name: Name::str("x"),
                ty: Expr::Sort(Level::zero()),
                binder_info: BinderInfo::Default,
                default_val: None,
                proj_name: Name::str("T.x"),
                idx: 0,
                is_inherited: false,
                from_parent: None,
            }],
            parent_structs: Vec::new(),
            is_class: false,
            ctor_name: Name::str("T.mk"),
            mk_name: Name::str("T.mk"),
        };
        assert!(validate_update_fields(&info, &[Name::str("x")]).is_ok());
        assert!(validate_update_fields(&info, &[Name::str("z")]).is_err());
    }
}
/// Flatten the inheritance chain of a structure into a `FlattenedStructure`.
#[allow(dead_code)]
pub fn flatten_structure(
    info: &StructureInfo,
    parents: &HashMap<Name, StructureInfo>,
) -> FlattenedStructure {
    let mut flat = FlattenedStructure::new();
    let mut queue = std::collections::VecDeque::new();
    for parent in &info.parent_structs {
        queue.push_back(parent.clone());
    }
    let mut visited = std::collections::HashSet::new();
    while let Some(cur) = queue.pop_front() {
        if visited.contains(&cur) {
            continue;
        }
        visited.insert(cur.clone());
        flat.ancestors.push(cur.clone());
        if let Some(pinfo) = parents.get(&cur) {
            for f in &pinfo.fields {
                if flat.fields.iter().all(|ef| ef.name != f.name) {
                    let mut inherited_field = f.clone();
                    inherited_field.is_inherited = true;
                    inherited_field.from_parent = Some(cur.clone());
                    flat.field_sources.insert(f.name.clone(), cur.clone());
                    flat.fields.push(inherited_field);
                }
            }
            for parent2 in &pinfo.parent_structs {
                queue.push_back(parent2.clone());
            }
        }
    }
    for f in &info.fields {
        if flat.fields.iter().all(|ef| ef.name != f.name) {
            flat.field_sources.insert(f.name.clone(), Name::str("self"));
            flat.fields.push(f.clone());
        }
    }
    flat
}
/// Normalize a chain of projections `s.f1.f2.f3` into a simpler form.
#[allow(dead_code)]
pub fn normalize_projection_chain(expr: &Expr, info: &StructureInfo) -> Option<(Name, u32, Expr)> {
    if let Expr::Proj(field, idx, base) = expr {
        if let Expr::App(_, _) = base.as_ref() {
            return Some((field.clone(), *idx, *base.clone()));
        }
        if info.fields.iter().any(|f| &f.name == field) {
            return Some((field.clone(), *idx, *base.clone()));
        }
    }
    None
}
#[cfg(test)]
mod structure_ext_tests {
    use super::*;
    use crate::structure::*;
    fn nat_ty() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    fn mk_field(name: &str, idx: usize) -> FieldInfo {
        FieldInfo {
            name: Name::str(name),
            ty: nat_ty(),
            binder_info: BinderInfo::Default,
            default_val: None,
            proj_name: Name::str(format!("S.{}", name)),
            idx,
            is_inherited: false,
            from_parent: None,
        }
    }
    fn mk_struct(name: &str, fields: Vec<FieldInfo>) -> StructureInfo {
        StructureInfo {
            name: Name::str(name),
            fields,
            parent_structs: Vec::new(),
            is_class: false,
            ctor_name: Name::str(format!("{}.mk", name)),
            mk_name: Name::str(format!("{}.mk", name)),
            params: Vec::new(),
            univ_params: Vec::new(),
        }
    }
    #[test]
    fn test_flattened_structure_own_only() {
        let info = mk_struct("S", vec![mk_field("x", 0), mk_field("y", 1)]);
        let flat = flatten_structure(&info, &HashMap::new());
        assert_eq!(flat.own_field_count(), 2);
        assert_eq!(flat.inherited_field_count(), 0);
        assert!(flat.ancestors.is_empty());
    }
    #[test]
    fn test_flattened_structure_with_parent() {
        let parent = mk_struct("P", vec![mk_field("a", 0)]);
        let mut child = mk_struct("C", vec![mk_field("b", 0)]);
        child.parent_structs = vec![Name::str("P")];
        let mut parents = HashMap::new();
        parents.insert(Name::str("P"), parent);
        let flat = flatten_structure(&child, &parents);
        assert_eq!(flat.fields.len(), 2);
        assert_eq!(flat.inherited_field_count(), 1);
        assert_eq!(flat.own_field_count(), 1);
    }
    #[test]
    fn test_struct_update_builder_build() {
        let info = mk_struct("S", vec![mk_field("x", 0), mk_field("y", 1)]);
        let base = Expr::Const(Name::str("s"), vec![]);
        let builder =
            StructUpdateBuilder::new(Name::str("S"), base).update(Name::str("x"), nat_ty());
        let result = builder.build(&info);
        assert!(result.is_ok());
    }
    #[test]
    fn test_struct_update_builder_invalid_field() {
        let info = mk_struct("S", vec![mk_field("x", 0)]);
        let base = Expr::Const(Name::str("s"), vec![]);
        let builder =
            StructUpdateBuilder::new(Name::str("S"), base).update(Name::str("z"), nat_ty());
        let result = builder.build(&info);
        assert!(result.is_err());
    }
    #[test]
    fn test_struct_update_builder_num_updates() {
        let base = Expr::Const(Name::str("s"), vec![]);
        let builder = StructUpdateBuilder::new(Name::str("S"), base)
            .update(Name::str("x"), nat_ty())
            .update(Name::str("y"), nat_ty());
        assert_eq!(builder.num_updates(), 2);
    }
    #[test]
    fn test_structure_stats_from_info() {
        let mut info = mk_struct("S", vec![mk_field("x", 0), mk_field("y", 1)]);
        info.fields[1].is_inherited = true;
        info.fields[0].default_val = Some(nat_ty());
        let stats = StructureStats::from_info(&info);
        assert_eq!(stats.num_fields, 2);
        assert_eq!(stats.num_own(), 1);
        assert_eq!(stats.num_inherited, 1);
        assert_eq!(stats.num_defaults, 1);
    }
    #[test]
    fn test_structure_stats_summary() {
        let info = mk_struct("S", vec![mk_field("x", 0)]);
        let stats = StructureStats::from_info(&info);
        let s = stats.summary();
        assert!(s.contains("fields=1"));
    }
    #[test]
    fn test_flattened_structure_get_field() {
        let info = mk_struct("S", vec![mk_field("x", 0)]);
        let flat = flatten_structure(&info, &HashMap::new());
        assert!(flat.get_field(&Name::str("x")).is_some());
        assert!(flat.get_field(&Name::str("z")).is_none());
    }
    #[test]
    fn test_flattened_structure_field_names() {
        let info = mk_struct("S", vec![mk_field("a", 0), mk_field("b", 1)]);
        let flat = flatten_structure(&info, &HashMap::new());
        let names: Vec<&Name> = flat.field_names();
        assert_eq!(names.len(), 2);
    }
}
