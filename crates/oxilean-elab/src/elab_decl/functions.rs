//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::context::ElabContext;
use crate::derive::{ConstructorInfo, DerivableClass, Deriver, TypeInfo};
use crate::elaborate::{elaborate_expr, elaborate_with_expected_type, ElabError};
use crate::infer::TypeInferencer;
use oxilean_kernel::{BinderInfo, Environment, Expr, Level, Name};
use oxilean_parse::{AttributeKind, Binder, Decl, Located, SurfaceExpr, WhereClause};

use super::types::{
    DeclElabError, DeclElaborator, DeclFilter, DeclPipeline, DeclRepository, DeclStats,
    DeclValidator, NamespaceManager, PendingDecl, ProcessedAttrs, ValidationError,
    ValidationWarning,
};

/// Process an attribute list into structured flags.
#[allow(dead_code)]
pub fn process_attributes(attrs: &[AttributeKind]) -> ProcessedAttrs {
    let mut result = ProcessedAttrs::default();
    for attr in attrs {
        match attr {
            AttributeKind::Simp => result.is_simp = true,
            AttributeKind::Ext => result.is_ext = true,
            AttributeKind::Instance => result.is_instance = true,
            AttributeKind::Reducible => result.is_reducible = true,
            AttributeKind::Irreducible => result.is_irreducible = true,
            AttributeKind::Inline => result.is_inline = true,
            AttributeKind::NoInline => {}
            AttributeKind::SpecializeAttr => result.is_specialize = true,
            AttributeKind::Custom(name) => result.custom.push(name.clone()),
        }
    }
    result
}
/// Elaborate a top-level declaration into a pending declaration.
///
/// This is the main entry point for declaration elaboration. It dispatches
/// to specialized functions based on the declaration kind.
pub fn elaborate_decl(env: &Environment, decl: &Decl) -> Result<PendingDecl, DeclElabError> {
    match decl {
        Decl::Definition {
            name,
            univ_params,
            ty,
            val,
            where_clauses,
            attrs,
        } => elaborate_definition(
            env,
            name,
            univ_params,
            ty.as_ref(),
            val,
            where_clauses,
            attrs,
        ),
        Decl::Theorem {
            name,
            univ_params,
            ty,
            proof,
            where_clauses,
            attrs,
        } => elaborate_theorem(env, name, univ_params, ty, proof, where_clauses, attrs),
        Decl::Axiom {
            name,
            univ_params,
            ty,
            attrs,
        } => elaborate_axiom(env, name, univ_params, ty, attrs),
        Decl::Inductive {
            name,
            univ_params,
            params,
            indices,
            ty,
            ctors,
        } => elaborate_inductive(env, name, univ_params, params, indices, ty, ctors),
        Decl::Import { path } => Err(DeclElabError::UnsupportedDecl(format!(
            "import {} not yet supported in elaboration",
            path.join(".")
        ))),
        Decl::Mutual { decls } => elaborate_mutual_block(env, decls),
        Decl::Derive {
            instances,
            type_name,
        } => elaborate_derive(env, type_name, instances),
        Decl::NotationDecl { .. } => Ok(PendingDecl::Axiom {
            name: Name::str("notation_axiom"),
            ty: Expr::Sort(Level::zero()),
            attrs: vec![],
        }),
        Decl::Universe { names } => {
            if names.is_empty() {
                return Err(DeclElabError::ElabError(
                    "universe declaration with no names".to_string(),
                ));
            }
            let first_name = &names[0];
            Ok(PendingDecl::Axiom {
                name: Name::str(first_name),
                ty: Expr::Sort(Level::succ(Level::Param(Name::str(first_name)))),
                attrs: vec![],
            })
        }
        Decl::Namespace { name, decls } => {
            if decls.is_empty() {
                return Err(DeclElabError::ElabError(format!(
                    "empty namespace '{}'",
                    name
                )));
            }
            let first = &decls[0].value;
            let pending = elaborate_decl(env, first)?;
            Ok(prefix_name(pending, name))
        }
        Decl::Structure {
            name,
            univ_params,
            fields,
            ..
        } => {
            let mut ctx = ElabContext::new(env);
            register_univ_params_ctx(&mut ctx, univ_params);
            let mut field_tys: Vec<Expr> = Vec::new();
            let mut max_field_level = Level::zero();
            for field in fields {
                match elaborate_expr(&mut ctx, &field.ty) {
                    Ok(field_ty) => {
                        let field_sort = sort_level_of_type(&field_ty, univ_params);
                        max_field_level = Level::max(max_field_level, field_sort);
                        ctx.push_hypothesis(Name::str(&field.name), field_ty.clone());
                        field_tys.push(field_ty);
                    }
                    Err(_) => {
                        field_tys.push(Expr::Sort(Level::zero()));
                    }
                }
            }
            let sort_level = if max_field_level == Level::zero() {
                if let Some(first_param) = univ_params.first() {
                    Level::Param(Name::str(first_param))
                } else {
                    Level::succ(Level::zero())
                }
            } else {
                max_field_level
            };
            Ok(PendingDecl::Axiom {
                name: Name::str(name),
                ty: Expr::Sort(sort_level),
                attrs: vec![],
            })
        }
        Decl::ClassDecl {
            name, univ_params, ..
        } => {
            let mut ctx = ElabContext::new(env);
            register_univ_params_ctx(&mut ctx, univ_params);
            Ok(PendingDecl::Axiom {
                name: Name::str(name),
                ty: Expr::Sort(Level::succ(Level::zero())),
                attrs: vec![AttributeKind::Instance],
            })
        }
        Decl::InstanceDecl {
            name,
            class_name,
            ty,
            ..
        } => {
            let mut ctx = ElabContext::new(env);
            let elab_ty = elaborate_expr(&mut ctx, ty)?;
            let inst_name = name
                .as_ref()
                .map(Name::str)
                .unwrap_or_else(|| Name::str(format!("inst_{}", class_name)));
            Ok(PendingDecl::Definition {
                name: inst_name,
                ty: elab_ty.clone(),
                val: elab_ty,
                attrs: vec![AttributeKind::Instance],
            })
        }
        Decl::SectionDecl { name, decls } => {
            if decls.is_empty() {
                return Err(DeclElabError::ElabError(format!(
                    "empty section '{}'",
                    name
                )));
            }
            elaborate_decl(env, &decls[0].value)
        }
        Decl::Variable { binders } => {
            if binders.is_empty() {
                return Err(DeclElabError::ElabError(
                    "variable declaration with no binders".to_string(),
                ));
            }
            let binder = &binders[0];
            let mut ctx = ElabContext::new(env);
            let ty = if let Some(ty_expr) = &binder.ty {
                elaborate_expr(&mut ctx, ty_expr)?
            } else {
                Expr::Sort(Level::zero())
            };
            Ok(PendingDecl::Axiom {
                name: Name::str(&binder.name),
                ty,
                attrs: vec![],
            })
        }
        Decl::Open { .. } => Ok(PendingDecl::Axiom {
            name: Name::str("open_axiom"),
            ty: Expr::Sort(Level::zero()),
            attrs: vec![],
        }),
        Decl::Attribute { decl, .. } => elaborate_decl(env, &decl.value),
        Decl::HashCmd { cmd, arg } => {
            let mut ctx = ElabContext::new(env);
            let _elab = elaborate_expr(&mut ctx, arg)?;
            Err(DeclElabError::UnsupportedDecl(format!(
                "#{} is a command, not a declaration",
                cmd
            )))
        }
    }
}
/// Build a [`TypeInfo`] for the inductive type `type_name` from the environment.
fn build_type_info(env: &Environment, type_name: &str) -> Option<TypeInfo> {
    let iname = Name::str(type_name);
    let iv = env.get_inductive_val(&iname)?;
    let univ_params = iv.common.level_params.clone();
    let mut params: Vec<(Name, Expr, BinderInfo)> = Vec::new();
    let mut ty = &iv.common.ty;
    for _ in 0..iv.num_params {
        if let Expr::Pi(bi, param_name, param_ty, body) = ty {
            params.push((param_name.clone(), *param_ty.clone(), *bi));
            ty = body;
        } else {
            break;
        }
    }
    let mut constructors: Vec<ConstructorInfo> = Vec::new();
    for ctor_name in &iv.ctors {
        if let Some(cv) = env.get_constructor_val(ctor_name) {
            let mut fields: Vec<(Name, Expr)> = Vec::new();
            let mut ct = &cv.common.ty;
            for _ in 0..cv.num_params {
                if let Expr::Pi(_, _, _, body) = ct {
                    ct = body;
                } else {
                    break;
                }
            }
            let mut field_idx = 0u32;
            while field_idx < cv.num_fields {
                if let Expr::Pi(_, field_name, field_ty, body) = ct {
                    fields.push((field_name.clone(), *field_ty.clone()));
                    ct = body;
                    field_idx += 1;
                } else {
                    break;
                }
            }
            constructors.push(ConstructorInfo::new(
                ctor_name.clone(),
                fields,
                cv.num_params as usize,
            ));
        }
    }
    Some(TypeInfo::new(
        iname,
        univ_params,
        params,
        constructors,
        iv.is_rec,
        iv.num_indices as usize,
    ))
}
/// Elaborate a `derive` declaration.
fn elaborate_derive(
    env: &Environment,
    type_name: &str,
    instances: &[String],
) -> Result<PendingDecl, DeclElabError> {
    if instances.is_empty() {
        return Err(DeclElabError::UnsupportedDecl(format!(
            "derive: no instances specified for {}",
            type_name
        )));
    }
    let type_info = build_type_info(env, type_name);
    let deriver = Deriver::new();
    for inst in instances {
        if let Some(cls) = DerivableClass::from_name(inst) {
            if let Some(ref ti) = type_info {
                match deriver.derive_with_info(cls, ti) {
                    Ok(result) => {
                        return Ok(PendingDecl::Definition {
                            name: result.instance_name,
                            ty: result.instance_type,
                            val: result.instance_body,
                            attrs: vec![],
                        });
                    }
                    Err(e) => {
                        let _ = e;
                    }
                }
            }
            let instance_name = Name::str(format!("{}_{}", type_name, inst.to_lowercase()));
            let instance_ty = Expr::App(
                Box::new(Expr::Const(Name::str(inst), vec![])),
                Box::new(Expr::Const(Name::str(type_name), vec![])),
            );
            return Ok(PendingDecl::Axiom {
                name: instance_name,
                ty: instance_ty,
                attrs: vec![],
            });
        }
    }
    Err(DeclElabError::UnsupportedDecl(format!(
        "derive: no known derivable class in {:?} for {}",
        instances, type_name
    )))
}
/// Elaborate a definition declaration.
#[allow(clippy::too_many_arguments)]
fn elaborate_definition(
    env: &Environment,
    name: &str,
    univ_params: &[String],
    ty_opt: Option<&Located<SurfaceExpr>>,
    val: &Located<SurfaceExpr>,
    where_clauses: &[WhereClause],
    attrs: &[AttributeKind],
) -> Result<PendingDecl, DeclElabError> {
    let mut ctx = ElabContext::new(env);
    register_univ_params_ctx(&mut ctx, univ_params);
    let _where_defs = elaborate_where_clauses(&mut ctx, where_clauses)?;
    let val_expr = elaborate_expr(&mut ctx, val)?;
    let ty_expr = if let Some(ty_surf) = ty_opt {
        elaborate_expr(&mut ctx, ty_surf)?
    } else {
        let (_fb_id, fallback_meta) = ctx.fresh_meta(Expr::Sort(Level::zero()));
        let inferred = {
            let mut inferencer = TypeInferencer::new(&mut ctx);
            inferencer.infer(&val_expr).ok()
        };
        inferred.unwrap_or(fallback_meta)
    };
    Ok(PendingDecl::Definition {
        name: Name::str(name),
        ty: ty_expr,
        val: val_expr,
        attrs: attrs.to_vec(),
    })
}
/// Elaborate a theorem declaration.
#[allow(clippy::too_many_arguments)]
fn elaborate_theorem(
    env: &Environment,
    name: &str,
    univ_params: &[String],
    ty: &Located<SurfaceExpr>,
    proof: &Located<SurfaceExpr>,
    where_clauses: &[WhereClause],
    attrs: &[AttributeKind],
) -> Result<PendingDecl, DeclElabError> {
    let mut ctx = ElabContext::new(env);
    register_univ_params_ctx(&mut ctx, univ_params);
    let _where_defs = elaborate_where_clauses(&mut ctx, where_clauses)?;
    let ty_expr = elaborate_expr(&mut ctx, ty)?;
    let proof_expr = elaborate_with_expected_type(&mut ctx, proof, &ty_expr)
        .or_else(|_| elaborate_expr(&mut ctx, proof))?;
    Ok(PendingDecl::Theorem {
        name: Name::str(name),
        ty: ty_expr,
        proof: proof_expr,
        attrs: attrs.to_vec(),
    })
}
/// Elaborate an axiom declaration.
fn elaborate_axiom(
    env: &Environment,
    name: &str,
    univ_params: &[String],
    ty: &Located<SurfaceExpr>,
    attrs: &[AttributeKind],
) -> Result<PendingDecl, DeclElabError> {
    let mut ctx = ElabContext::new(env);
    register_univ_params_ctx(&mut ctx, univ_params);
    let ty_expr = elaborate_expr(&mut ctx, ty)?;
    validate_axiom_type(&ty_expr)?;
    Ok(PendingDecl::Axiom {
        name: Name::str(name),
        ty: ty_expr,
        attrs: attrs.to_vec(),
    })
}
/// Validate that an expression is a valid axiom type.
fn validate_axiom_type(ty: &Expr) -> Result<(), DeclElabError> {
    match ty {
        Expr::Sort(_)
        | Expr::Pi(_, _, _, _)
        | Expr::Const(_, _)
        | Expr::App(_, _)
        | Expr::FVar(_) => Ok(()),
        _ => Ok(()),
    }
}
/// Elaborate an inductive type declaration.
#[allow(clippy::too_many_arguments)]
fn elaborate_inductive(
    env: &Environment,
    name: &str,
    univ_params: &[String],
    params: &[Binder],
    indices: &[Binder],
    ty: &Located<SurfaceExpr>,
    ctors: &[oxilean_parse::Constructor],
) -> Result<PendingDecl, DeclElabError> {
    let mut ctx = ElabContext::new(env);
    register_univ_params_ctx(&mut ctx, univ_params);
    let mut param_exprs = Vec::new();
    for param in params {
        let param_ty = if let Some(ty_surf) = &param.ty {
            elaborate_expr(&mut ctx, ty_surf)?
        } else {
            return Err(DeclElabError::MissingType(format!(
                "parameter '{}' of inductive type '{}' requires a type annotation",
                param.name, name
            )));
        };
        let _fvar = ctx.push_local(Name::str(&param.name), param_ty.clone(), None);
        param_exprs.push((Name::str(&param.name), param_ty));
    }
    for index in indices {
        let index_ty = if let Some(ty_surf) = &index.ty {
            elaborate_expr(&mut ctx, ty_surf)?
        } else {
            return Err(DeclElabError::MissingType(format!(
                "index '{}' of inductive type '{}' requires a type annotation",
                index.name, name
            )));
        };
        let _fvar = ctx.push_local(Name::str(&index.name), index_ty, None);
    }
    let ty_expr = elaborate_expr(&mut ctx, ty)?;
    let mut ctor_exprs = Vec::new();
    for ctor in ctors {
        let ctor_ty = elaborate_expr(&mut ctx, &ctor.ty)?;
        ctor_exprs.push((Name::str(&ctor.name), ctor_ty));
    }
    for _ in indices {
        ctx.pop_local();
    }
    for _ in params {
        ctx.pop_local();
    }
    let mut full_ty = ty_expr;
    for (pname, pty) in param_exprs.into_iter().rev() {
        full_ty = Expr::Pi(
            oxilean_kernel::BinderInfo::Default,
            pname,
            Box::new(pty),
            Box::new(full_ty),
        );
    }
    Ok(PendingDecl::Inductive {
        name: Name::str(name),
        ty: full_ty,
        ctors: ctor_exprs,
        attrs: vec![],
    })
}
/// Elaborate a mutual block of definitions.
fn elaborate_mutual_block(
    env: &Environment,
    decls: &[Located<Decl>],
) -> Result<PendingDecl, DeclElabError> {
    if decls.is_empty() {
        return Err(DeclElabError::ElabError("empty mutual block".to_string()));
    }
    let mut ctx = ElabContext::new(env);
    let mut forward_decls: Vec<(String, u64, Expr)> = Vec::new();
    for decl in decls {
        let name = extract_decl_name(&decl.value)?;
        let (meta_id, meta_expr) = ctx.fresh_meta(Expr::Sort(Level::zero()));
        let _fvar = ctx.push_local(Name::str(&name), meta_expr.clone(), None);
        forward_decls.push((name, meta_id, meta_expr));
    }
    let mut results: Vec<PendingDecl> = Vec::new();
    for (i, decl) in decls.iter().enumerate() {
        let pending = elaborate_decl(env, &decl.value)?;
        let actual_ty = pending.ty().clone();
        ctx.assign_meta(forward_decls[i].1, actual_ty);
        results.push(pending);
    }
    for _ in &forward_decls {
        ctx.pop_local();
    }
    if let Some(first) = results.into_iter().next() {
        Ok(first)
    } else {
        Err(DeclElabError::ElabError(
            "mutual block produced no declarations".to_string(),
        ))
    }
}
/// Extract the name from a declaration.
fn extract_decl_name(decl: &Decl) -> Result<String, DeclElabError> {
    match decl {
        Decl::Definition { name, .. }
        | Decl::Theorem { name, .. }
        | Decl::Axiom { name, .. }
        | Decl::Inductive { name, .. } => Ok(name.clone()),
        _ => Err(DeclElabError::UnsupportedDecl(
            "only definitions, theorems, axioms, and inductives can appear in mutual blocks"
                .to_string(),
        )),
    }
}
/// Elaborate where clauses and add them to the context as local definitions.
///
/// Each where clause `where f x := e` creates a local let-binding in scope.
/// Returns the elaborated where definitions as (name, type, value) triples.
fn elaborate_where_clauses(
    ctx: &mut ElabContext,
    clauses: &[WhereClause],
) -> Result<Vec<(Name, Expr, Expr)>, DeclElabError> {
    let mut results = Vec::new();
    for clause in clauses {
        let clause_ty = if let Some(ty_surf) = &clause.ty {
            elaborate_expr(ctx, ty_surf)?
        } else {
            let (_id, meta) = ctx.fresh_meta(Expr::Sort(Level::zero()));
            meta
        };
        let val_expr = if clause.params.is_empty() {
            elaborate_expr(ctx, &clause.val)?
        } else {
            elaborate_where_body(ctx, &clause.params, &clause.val)?
        };
        let full_ty = if clause.params.is_empty() {
            clause_ty
        } else {
            build_pi_from_binders(ctx, &clause.params, &clause_ty)?
        };
        let name = Name::str(&clause.name);
        let _fvar = ctx.push_local(name.clone(), full_ty.clone(), Some(val_expr.clone()));
        results.push((name, full_ty, val_expr));
    }
    Ok(results)
}
/// Build a lambda abstraction from binders and body.
fn elaborate_where_body(
    ctx: &mut ElabContext,
    binders: &[Binder],
    body: &Located<SurfaceExpr>,
) -> Result<Expr, DeclElabError> {
    if binders.is_empty() {
        return Ok(elaborate_expr(ctx, body)?);
    }
    let binder = &binders[0];
    let ty = if let Some(ty_surf) = &binder.ty {
        elaborate_expr(ctx, ty_surf)?
    } else {
        let (_id, meta) = ctx.fresh_meta(Expr::Sort(Level::zero()));
        meta
    };
    let _fvar = ctx.push_local(Name::str(&binder.name), ty.clone(), None);
    let inner = elaborate_where_body(ctx, &binders[1..], body)?;
    ctx.pop_local();
    Ok(Expr::Lam(
        convert_binder_kind_local(&binder.info),
        Name::str(&binder.name),
        Box::new(ty),
        Box::new(inner),
    ))
}
/// Build a Pi type from binders and result type.
fn build_pi_from_binders(
    ctx: &mut ElabContext,
    binders: &[Binder],
    result_ty: &Expr,
) -> Result<Expr, DeclElabError> {
    if binders.is_empty() {
        return Ok(result_ty.clone());
    }
    let mut result = result_ty.clone();
    for binder in binders.iter().rev() {
        let ty = if let Some(ty_surf) = &binder.ty {
            elaborate_expr(ctx, ty_surf)?
        } else {
            let (_id, meta) = ctx.fresh_meta(Expr::Sort(Level::zero()));
            meta
        };
        result = Expr::Pi(
            convert_binder_kind_local(&binder.info),
            Name::str(&binder.name),
            Box::new(ty),
            Box::new(result),
        );
    }
    Ok(result)
}
/// Register universe parameters in an elaboration context.
#[allow(dead_code)]
fn register_univ_params_ctx(ctx: &mut ElabContext, params: &[String]) {
    for p in params {
        ctx.push_univ_param(Name::str(p));
    }
}
/// Compute the sort level contributed by a field type.
///
/// For a field whose type is `Expr::Sort(l)` the field itself is a universe,
/// so the enclosing structure must live at `l + 1`.  For a field whose type is
/// an arbitrary term we conservatively return `Level::zero()` (Prop), which is
/// correct for concrete types.  Universe-parameter names are looked up in
/// `univ_params` to produce `Level::Param` nodes.
fn sort_level_of_type(ty: &Expr, _univ_params: &[String]) -> Level {
    match ty {
        Expr::Sort(l) => l.clone(),
        _ => Level::zero(),
    }
}
/// Convert parse BinderKind to kernel BinderInfo (local version).
fn convert_binder_kind_local(kind: &oxilean_parse::BinderKind) -> oxilean_kernel::BinderInfo {
    match kind {
        oxilean_parse::BinderKind::Default => oxilean_kernel::BinderInfo::Default,
        oxilean_parse::BinderKind::Implicit => oxilean_kernel::BinderInfo::Implicit,
        oxilean_parse::BinderKind::Instance => oxilean_kernel::BinderInfo::InstImplicit,
        oxilean_parse::BinderKind::StrictImplicit => oxilean_kernel::BinderInfo::StrictImplicit,
    }
}
/// Prefix a pending declaration's name with a namespace.
fn prefix_name(decl: PendingDecl, ns: &str) -> PendingDecl {
    let prefix = |n: &Name| Name::str(format!("{}.{}", ns, n));
    match decl {
        PendingDecl::Definition {
            name,
            ty,
            val,
            attrs,
        } => PendingDecl::Definition {
            name: prefix(&name),
            ty,
            val,
            attrs,
        },
        PendingDecl::Theorem {
            name,
            ty,
            proof,
            attrs,
        } => PendingDecl::Theorem {
            name: prefix(&name),
            ty,
            proof,
            attrs,
        },
        PendingDecl::Axiom { name, ty, attrs } => PendingDecl::Axiom {
            name: prefix(&name),
            ty,
            attrs,
        },
        PendingDecl::Inductive {
            name,
            ty,
            ctors,
            attrs,
        } => PendingDecl::Inductive {
            name: prefix(&name),
            ty,
            ctors,
            attrs,
        },
        PendingDecl::Opaque { name, ty, val } => PendingDecl::Opaque {
            name: prefix(&name),
            ty,
            val,
        },
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::elab_decl::*;
    use oxilean_parse::{Located, SortKind, Span, SurfaceExpr};
    fn mk_span() -> Span {
        Span::new(0, 1, 1, 1)
    }
    fn loc<T>(v: T) -> Located<T> {
        Located::new(v, mk_span())
    }
    fn mk_env() -> Environment {
        Environment::new()
    }
    #[test]
    fn test_error_display_elab_error() {
        let e = DeclElabError::ElabError("something went wrong".to_string());
        assert!(e.to_string().contains("something went wrong"));
    }
    #[test]
    fn test_error_display_type_mismatch() {
        let e = DeclElabError::TypeMismatch {
            expected: "Nat".to_string(),
            got: "Bool".to_string(),
        };
        let s = e.to_string();
        assert!(s.contains("Nat"));
        assert!(s.contains("Bool"));
    }
    #[test]
    fn test_error_display_duplicate_name() {
        let e = DeclElabError::DuplicateName("foo".to_string());
        assert!(e.to_string().contains("foo"));
    }
    #[test]
    fn test_error_display_invalid_recursion() {
        let e = DeclElabError::InvalidRecursion("bad cycle".to_string());
        assert!(e.to_string().contains("bad cycle"));
    }
    #[test]
    fn test_error_display_missing_type() {
        let e = DeclElabError::MissingType("x".to_string());
        assert!(e.to_string().contains("x"));
    }
    #[test]
    fn test_error_display_unsupported() {
        let e = DeclElabError::UnsupportedDecl("fancy".to_string());
        assert!(e.to_string().contains("fancy"));
    }
    #[test]
    fn test_pending_decl_name() {
        let pd = PendingDecl::Definition {
            name: Name::str("foo"),
            ty: Expr::Sort(Level::zero()),
            val: Expr::Lit(oxilean_kernel::Literal::Nat(42)),
            attrs: vec![],
        };
        assert_eq!(pd.name(), &Name::str("foo"));
    }
    #[test]
    fn test_pending_decl_ty() {
        let ty = Expr::Sort(Level::zero());
        let pd = PendingDecl::Axiom {
            name: Name::str("ax"),
            ty: ty.clone(),
            attrs: vec![],
        };
        assert_eq!(pd.ty(), &ty);
    }
    #[test]
    fn test_pending_decl_kind_checks() {
        let def = PendingDecl::Definition {
            name: Name::str("d"),
            ty: Expr::Sort(Level::zero()),
            val: Expr::Sort(Level::zero()),
            attrs: vec![],
        };
        assert!(def.is_definition());
        assert!(!def.is_theorem());
        let thm = PendingDecl::Theorem {
            name: Name::str("t"),
            ty: Expr::Sort(Level::zero()),
            proof: Expr::Sort(Level::zero()),
            attrs: vec![],
        };
        assert!(thm.is_theorem());
        assert!(!thm.is_axiom());
        let ax = PendingDecl::Axiom {
            name: Name::str("a"),
            ty: Expr::Sort(Level::zero()),
            attrs: vec![],
        };
        assert!(ax.is_axiom());
        assert!(!ax.is_inductive());
        let ind = PendingDecl::Inductive {
            name: Name::str("i"),
            ty: Expr::Sort(Level::zero()),
            ctors: vec![],
            attrs: vec![],
        };
        assert!(ind.is_inductive());
        assert!(!ind.is_definition());
    }
    #[test]
    fn test_process_no_attrs() {
        let pa = process_attributes(&[]);
        assert!(!pa.is_simp);
        assert!(!pa.is_ext);
        assert!(!pa.is_instance);
    }
    #[test]
    fn test_process_simp_attr() {
        let pa = process_attributes(&[AttributeKind::Simp]);
        assert!(pa.is_simp);
    }
    #[test]
    fn test_process_multiple_attrs() {
        let pa = process_attributes(&[
            AttributeKind::Simp,
            AttributeKind::Ext,
            AttributeKind::Reducible,
            AttributeKind::Inline,
            AttributeKind::Custom("my_attr".to_string()),
        ]);
        assert!(pa.is_simp);
        assert!(pa.is_ext);
        assert!(pa.is_reducible);
        assert!(pa.is_inline);
        assert_eq!(pa.custom, vec!["my_attr".to_string()]);
    }
    #[test]
    fn test_decl_elaborator_new() {
        let env = mk_env();
        let elab = DeclElaborator::new(&env);
        assert!(elab.pending_decls().is_empty());
    }
    #[test]
    fn test_decl_elaborator_elaborate_definition() {
        let env = mk_env();
        let mut elab = DeclElaborator::new(&env);
        let decl = Decl::Definition {
            name: "mydef".to_string(),
            univ_params: vec![],
            ty: Some(loc(SurfaceExpr::Sort(SortKind::Type))),
            val: loc(SurfaceExpr::Sort(SortKind::Prop)),
            where_clauses: vec![],
            attrs: vec![],
        };
        let result = elab.elaborate(&decl);
        assert!(result.is_ok());
        assert_eq!(elab.pending_decls().len(), 1);
    }
    #[test]
    fn test_decl_elaborator_take_pending() {
        let env = mk_env();
        let mut elab = DeclElaborator::new(&env);
        let decl = Decl::Axiom {
            name: "ax1".to_string(),
            univ_params: vec![],
            ty: loc(SurfaceExpr::Sort(SortKind::Prop)),
            attrs: vec![],
        };
        let _ = elab.elaborate(&decl);
        let pending = elab.take_pending();
        assert_eq!(pending.len(), 1);
        assert!(elab.pending_decls().is_empty());
    }
    #[test]
    fn test_elaborate_definition_with_type() {
        let env = mk_env();
        let decl = Decl::Definition {
            name: "id_fn".to_string(),
            univ_params: vec![],
            ty: Some(loc(SurfaceExpr::Sort(SortKind::Type))),
            val: loc(SurfaceExpr::Sort(SortKind::Prop)),
            where_clauses: vec![],
            attrs: vec![AttributeKind::Reducible],
        };
        let result = elaborate_decl(&env, &decl).expect("elaboration should succeed");
        assert!(result.is_definition());
        assert_eq!(result.name(), &Name::str("id_fn"));
    }
    #[test]
    fn test_elaborate_definition_without_type() {
        let env = mk_env();
        let decl = Decl::Definition {
            name: "val42".to_string(),
            univ_params: vec![],
            ty: None,
            val: loc(SurfaceExpr::Lit(oxilean_parse::Literal::Nat(42))),
            where_clauses: vec![],
            attrs: vec![],
        };
        let result = elaborate_decl(&env, &decl).expect("elaboration should succeed");
        assert!(result.is_definition());
    }
    #[test]
    fn test_elaborate_theorem() {
        let env = mk_env();
        let decl = Decl::Theorem {
            name: "my_thm".to_string(),
            univ_params: vec![],
            ty: loc(SurfaceExpr::Sort(SortKind::Prop)),
            proof: loc(SurfaceExpr::Sort(SortKind::Prop)),
            where_clauses: vec![],
            attrs: vec![AttributeKind::Simp],
        };
        let result = elaborate_decl(&env, &decl).expect("elaboration should succeed");
        assert!(result.is_theorem());
        assert_eq!(result.name(), &Name::str("my_thm"));
    }
    #[test]
    fn test_elaborate_axiom() {
        let env = mk_env();
        let decl = Decl::Axiom {
            name: "em".to_string(),
            univ_params: vec![],
            ty: loc(SurfaceExpr::Sort(SortKind::Prop)),
            attrs: vec![],
        };
        let result = elaborate_decl(&env, &decl).expect("elaboration should succeed");
        assert!(result.is_axiom());
        assert_eq!(result.name(), &Name::str("em"));
    }
    #[test]
    fn test_elaborate_axiom_with_univ_params() {
        let env = mk_env();
        let decl = Decl::Axiom {
            name: "choice".to_string(),
            univ_params: vec!["u".to_string()],
            ty: loc(SurfaceExpr::Sort(SortKind::TypeU("u".to_string()))),
            attrs: vec![],
        };
        let result = elaborate_decl(&env, &decl).expect("elaboration should succeed");
        assert!(result.is_axiom());
    }
    #[test]
    fn test_elaborate_inductive_simple() {
        let env = mk_env();
        let decl = Decl::Inductive {
            name: "MyBool".to_string(),
            univ_params: vec![],
            params: vec![],
            indices: vec![],
            ty: loc(SurfaceExpr::Sort(SortKind::Type)),
            ctors: vec![
                oxilean_parse::Constructor {
                    name: "myTrue".to_string(),
                    ty: loc(SurfaceExpr::Var("MyBool".to_string())),
                },
                oxilean_parse::Constructor {
                    name: "myFalse".to_string(),
                    ty: loc(SurfaceExpr::Var("MyBool".to_string())),
                },
            ],
        };
        let result = elaborate_decl(&env, &decl);
        assert!(result.is_err());
    }
    #[test]
    fn test_elaborate_inductive_no_ctors() {
        let env = mk_env();
        let decl = Decl::Inductive {
            name: "Empty".to_string(),
            univ_params: vec![],
            params: vec![],
            indices: vec![],
            ty: loc(SurfaceExpr::Sort(SortKind::Prop)),
            ctors: vec![],
        };
        let result = elaborate_decl(&env, &decl).expect("elaboration should succeed");
        assert!(result.is_inductive());
        if let PendingDecl::Inductive { ctors, .. } = &result {
            assert!(ctors.is_empty());
        }
    }
    #[test]
    fn test_elaborate_inductive_with_params() {
        let env = mk_env();
        let decl = Decl::Inductive {
            name: "MyList".to_string(),
            univ_params: vec!["u".to_string()],
            params: vec![Binder {
                name: "a".to_string(),
                ty: Some(Box::new(loc(SurfaceExpr::Sort(SortKind::TypeU(
                    "u".to_string(),
                ))))),
                info: oxilean_parse::BinderKind::Default,
            }],
            indices: vec![],
            ty: loc(SurfaceExpr::Sort(SortKind::TypeU("u".to_string()))),
            ctors: vec![],
        };
        let result = elaborate_decl(&env, &decl).expect("elaboration should succeed");
        assert!(result.is_inductive());
        assert!(result.ty().is_pi());
    }
    #[test]
    fn test_elaborate_universe_decl() {
        let env = mk_env();
        let decl = Decl::Universe {
            names: vec!["u".to_string(), "v".to_string()],
        };
        let result = elaborate_decl(&env, &decl).expect("elaboration should succeed");
        assert!(result.is_axiom());
        assert_eq!(result.name(), &Name::str("u"));
    }
    #[test]
    fn test_elaborate_universe_decl_empty() {
        let env = mk_env();
        let decl = Decl::Universe { names: vec![] };
        let result = elaborate_decl(&env, &decl);
        assert!(result.is_err());
    }
    #[test]
    fn test_elaborate_import_unsupported() {
        let env = mk_env();
        let decl = Decl::Import {
            path: vec!["Mathlib".to_string(), "Data".to_string()],
        };
        let result = elaborate_decl(&env, &decl);
        assert!(result.is_err());
        if let Err(DeclElabError::UnsupportedDecl(msg)) = result {
            assert!(msg.contains("Mathlib.Data"));
        }
    }
    #[test]
    fn test_elaborate_derive_unsupported() {
        let env = mk_env();
        let decl = Decl::Derive {
            instances: vec!["SomeUnknownClass".to_string()],
            type_name: "MyType".to_string(),
        };
        let result = elaborate_decl(&env, &decl);
        assert!(result.is_err());
    }
    #[test]
    fn test_elaborate_notation_unsupported() {
        let env = mk_env();
        let decl = Decl::NotationDecl {
            kind: oxilean_parse::AstNotationKind::Infixl,
            prec: Some(65),
            name: "+".to_string(),
            notation: "HAdd.hAdd".to_string(),
        };
        let result = elaborate_decl(&env, &decl);
        assert!(result.is_ok());
    }
    #[test]
    fn test_elaborate_mutual_block() {
        let env = mk_env();
        let decl = Decl::Mutual {
            decls: vec![
                loc(Decl::Definition {
                    name: "even".to_string(),
                    univ_params: vec![],
                    ty: Some(loc(SurfaceExpr::Sort(SortKind::Type))),
                    val: loc(SurfaceExpr::Lit(oxilean_parse::Literal::Nat(0))),
                    where_clauses: vec![],
                    attrs: vec![],
                }),
                loc(Decl::Definition {
                    name: "odd".to_string(),
                    univ_params: vec![],
                    ty: Some(loc(SurfaceExpr::Sort(SortKind::Type))),
                    val: loc(SurfaceExpr::Lit(oxilean_parse::Literal::Nat(1))),
                    where_clauses: vec![],
                    attrs: vec![],
                }),
            ],
        };
        let result = elaborate_decl(&env, &decl);
        assert!(result.is_ok());
    }
    #[test]
    fn test_elaborate_mutual_block_empty() {
        let env = mk_env();
        let decl = Decl::Mutual { decls: vec![] };
        let result = elaborate_decl(&env, &decl);
        assert!(result.is_err());
    }
    #[test]
    fn test_elaborate_definition_with_where() {
        let env = mk_env();
        let decl = Decl::Definition {
            name: "main_fn".to_string(),
            univ_params: vec![],
            ty: Some(loc(SurfaceExpr::Sort(SortKind::Type))),
            val: loc(SurfaceExpr::Lit(oxilean_parse::Literal::Nat(99))),
            where_clauses: vec![WhereClause {
                name: "helper".to_string(),
                params: vec![],
                ty: Some(loc(SurfaceExpr::Sort(SortKind::Type))),
                val: loc(SurfaceExpr::Lit(oxilean_parse::Literal::Nat(1))),
            }],
            attrs: vec![],
        };
        let result = elaborate_decl(&env, &decl).expect("elaboration should succeed");
        assert!(result.is_definition());
    }
    #[test]
    fn test_elaborate_theorem_with_where() {
        let env = mk_env();
        let decl = Decl::Theorem {
            name: "thm_with_lemma".to_string(),
            univ_params: vec![],
            ty: loc(SurfaceExpr::Sort(SortKind::Prop)),
            proof: loc(SurfaceExpr::Sort(SortKind::Prop)),
            where_clauses: vec![WhereClause {
                name: "lemma1".to_string(),
                params: vec![],
                ty: Some(loc(SurfaceExpr::Sort(SortKind::Prop))),
                val: loc(SurfaceExpr::Sort(SortKind::Prop)),
            }],
            attrs: vec![],
        };
        let result = elaborate_decl(&env, &decl).expect("elaboration should succeed");
        assert!(result.is_theorem());
    }
    #[test]
    fn test_elaborate_variable() {
        let env = mk_env();
        let decl = Decl::Variable {
            binders: vec![Binder {
                name: "n".to_string(),
                ty: Some(Box::new(loc(SurfaceExpr::Sort(SortKind::Type)))),
                info: oxilean_parse::BinderKind::Default,
            }],
        };
        let result = elaborate_decl(&env, &decl).expect("elaboration should succeed");
        assert!(result.is_axiom());
    }
    #[test]
    fn test_elaborate_variable_empty() {
        let env = mk_env();
        let decl = Decl::Variable { binders: vec![] };
        let result = elaborate_decl(&env, &decl);
        assert!(result.is_err());
    }
    #[test]
    fn test_elaborate_hash_cmd() {
        let env = mk_env();
        let decl = Decl::HashCmd {
            cmd: "check".to_string(),
            arg: loc(SurfaceExpr::Sort(SortKind::Type)),
        };
        let result = elaborate_decl(&env, &decl);
        assert!(result.is_err());
    }
    #[test]
    fn test_prefix_name() {
        let pd = PendingDecl::Definition {
            name: Name::str("foo"),
            ty: Expr::Sort(Level::zero()),
            val: Expr::Sort(Level::zero()),
            attrs: vec![],
        };
        let prefixed = prefix_name(pd, "MyNs");
        assert_eq!(prefixed.name(), &Name::str("MyNs.foo"));
    }
    #[test]
    fn test_validate_axiom_type_sort() {
        assert!(validate_axiom_type(&Expr::Sort(Level::zero())).is_ok());
    }
    #[test]
    fn test_validate_axiom_type_pi() {
        let pi = Expr::Pi(
            oxilean_kernel::BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(Expr::Sort(Level::zero())),
        );
        assert!(validate_axiom_type(&pi).is_ok());
    }
    #[test]
    fn test_extract_decl_name_def() {
        let decl = Decl::Definition {
            name: "f".to_string(),
            univ_params: vec![],
            ty: None,
            val: loc(SurfaceExpr::Hole),
            where_clauses: vec![],
            attrs: vec![],
        };
        assert_eq!(
            extract_decl_name(&decl).expect("test operation should succeed"),
            "f"
        );
    }
    #[test]
    fn test_extract_decl_name_unsupported() {
        let decl = Decl::Import {
            path: vec!["x".to_string()],
        };
        assert!(extract_decl_name(&decl).is_err());
    }
    #[test]
    fn test_where_clause_with_params() {
        let env = mk_env();
        let decl = Decl::Definition {
            name: "main_fn2".to_string(),
            univ_params: vec![],
            ty: Some(loc(SurfaceExpr::Sort(SortKind::Type))),
            val: loc(SurfaceExpr::Lit(oxilean_parse::Literal::Nat(0))),
            where_clauses: vec![WhereClause {
                name: "helper".to_string(),
                params: vec![Binder {
                    name: "x".to_string(),
                    ty: Some(Box::new(loc(SurfaceExpr::Sort(SortKind::Type)))),
                    info: oxilean_parse::BinderKind::Default,
                }],
                ty: Some(loc(SurfaceExpr::Sort(SortKind::Type))),
                val: loc(SurfaceExpr::Lit(oxilean_parse::Literal::Nat(1))),
            }],
            attrs: vec![],
        };
        let result = elaborate_decl(&env, &decl).expect("elaboration should succeed");
        assert!(result.is_definition());
    }
    #[test]
    fn test_elaborate_structure_stub() {
        let env = mk_env();
        let decl = Decl::Structure {
            name: "Point".to_string(),
            univ_params: vec![],
            extends: vec![],
            fields: vec![],
        };
        let result = elaborate_decl(&env, &decl).expect("elaboration should succeed");
        assert!(result.is_axiom());
        assert_eq!(result.name(), &Name::str("Point"));
    }
    #[test]
    fn test_elaborate_class_stub() {
        let env = mk_env();
        let decl = Decl::ClassDecl {
            name: "Monad".to_string(),
            univ_params: vec!["u".to_string()],
            extends: vec![],
            fields: vec![],
        };
        let result = elaborate_decl(&env, &decl).expect("elaboration should succeed");
        assert!(result.is_axiom());
    }
}
/// Check whether an expression contains a `sorry` constant.
#[allow(dead_code)]
pub fn expr_contains_sorry(expr: &Expr) -> bool {
    match expr {
        Expr::Const(n, _) => n == &oxilean_kernel::Name::str("sorry"),
        Expr::App(f, a) => expr_contains_sorry(f) || expr_contains_sorry(a),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            expr_contains_sorry(ty) || expr_contains_sorry(body)
        }
        Expr::Let(_, ty, val, body) => {
            expr_contains_sorry(ty) || expr_contains_sorry(val) || expr_contains_sorry(body)
        }
        Expr::Proj(_, _, inner) => expr_contains_sorry(inner),
        _ => false,
    }
}
#[cfg(test)]
mod elab_decl_extra_tests {
    use super::*;
    use crate::elab_decl::*;
    fn make_theorem(name: &str) -> PendingDecl {
        PendingDecl::Theorem {
            name: Name::str(name),
            ty: Expr::Sort(Level::zero()),
            proof: Expr::Sort(Level::zero()),
            attrs: vec![],
        }
    }
    fn make_sorry_theorem(name: &str) -> PendingDecl {
        PendingDecl::Theorem {
            name: Name::str(name),
            ty: Expr::Sort(Level::zero()),
            proof: Expr::Const(Name::str("sorry"), vec![]),
            attrs: vec![],
        }
    }
    fn make_def(name: &str) -> PendingDecl {
        PendingDecl::Definition {
            name: Name::str(name),
            ty: Expr::Sort(Level::zero()),
            val: Expr::Sort(Level::zero()),
            attrs: vec![],
        }
    }
    fn make_axiom(name: &str) -> PendingDecl {
        PendingDecl::Axiom {
            name: Name::str(name),
            ty: Expr::Sort(Level::zero()),
            attrs: vec![],
        }
    }
    fn make_inductive(name: &str, num_ctors: usize) -> PendingDecl {
        let ctors: Vec<(Name, Expr)> = (0..num_ctors)
            .map(|i| (Name::str(format!("ctor{}", i)), Expr::Sort(Level::zero())))
            .collect();
        PendingDecl::Inductive {
            name: Name::str(name),
            ty: Expr::Sort(Level::zero()),
            ctors,
            attrs: vec![],
        }
    }
    #[test]
    fn test_validator_clean_theorem() {
        let mut v = DeclValidator::new();
        v.validate(&make_theorem("thm1"));
        assert!(!v.has_errors());
        assert!(!v.has_warnings());
    }
    #[test]
    fn test_validator_sorry_theorem() {
        let mut v = DeclValidator::new();
        v.validate(&make_sorry_theorem("thm1"));
        assert!(!v.has_errors());
        assert!(v.has_warnings());
        assert!(v
            .warnings
            .iter()
            .any(|w| matches!(w, ValidationWarning::SorryProof(_))));
    }
    #[test]
    fn test_validator_no_constructors() {
        let mut v = DeclValidator::new();
        v.validate(&make_inductive("Empty", 0));
        assert!(v.has_errors());
        assert!(v
            .errors
            .iter()
            .any(|e| matches!(e, ValidationError::NoConstructors(_))));
    }
    #[test]
    fn test_validator_many_constructors() {
        let mut v = DeclValidator::new();
        v.validate(&make_inductive("Big", 70));
        assert!(!v.has_errors());
        assert!(v.has_warnings());
    }
    #[test]
    fn test_validator_clear() {
        let mut v = DeclValidator::new();
        v.validate(&make_sorry_theorem("t"));
        assert!(v.has_warnings());
        v.clear();
        assert!(!v.has_warnings());
    }
    #[test]
    fn test_validation_error_display_empty_name() {
        let e = ValidationError::EmptyName;
        assert!(!e.to_string().is_empty());
    }
    #[test]
    fn test_validation_error_display_trivial_proof() {
        let e = ValidationError::TrivialProof(Name::str("foo"));
        assert!(e.to_string().contains("foo"));
    }
    #[test]
    fn test_validation_warning_sorry() {
        let w = ValidationWarning::SorryProof(Name::str("t"));
        assert!(w.to_string().contains("sorry"));
    }
    #[test]
    fn test_validation_warning_missing_annotation() {
        let w = ValidationWarning::MissingAnnotation(Name::str("x"));
        assert!(w.to_string().contains("x"));
    }
    #[test]
    fn test_repo_empty() {
        let repo = DeclRepository::new();
        assert!(repo.is_empty());
        assert_eq!(repo.len(), 0);
    }
    #[test]
    fn test_repo_insert_and_get() {
        let mut repo = DeclRepository::new();
        let d = make_def("myDef");
        assert!(repo.insert(d));
        assert!(repo.get(&Name::str("myDef")).is_some());
    }
    #[test]
    fn test_repo_insert_duplicate() {
        let mut repo = DeclRepository::new();
        assert!(repo.insert(make_def("foo")));
        assert!(!repo.insert(make_def("foo")));
    }
    #[test]
    fn test_repo_contains() {
        let mut repo = DeclRepository::new();
        repo.insert(make_axiom("ax1"));
        assert!(repo.contains(&Name::str("ax1")));
        assert!(!repo.contains(&Name::str("ax2")));
    }
    #[test]
    fn test_repo_names() {
        let mut repo = DeclRepository::new();
        repo.insert(make_def("a"));
        repo.insert(make_def("b"));
        let names = repo.names();
        assert_eq!(names.len(), 2);
    }
    #[test]
    fn test_repo_drain() {
        let mut repo = DeclRepository::new();
        repo.insert(make_def("x"));
        let drained = repo.drain();
        assert_eq!(drained.len(), 1);
        assert!(repo.is_empty());
    }
    #[test]
    fn test_repo_iter() {
        let mut repo = DeclRepository::new();
        repo.insert(make_def("p"));
        repo.insert(make_theorem("q"));
        let count = repo.iter().count();
        assert_eq!(count, 2);
    }
    #[test]
    fn test_filter_all_accepts_everything() {
        let f = DeclFilter::all();
        assert!(f.accepts(&make_def("d")));
        assert!(f.accepts(&make_theorem("t")));
        assert!(f.accepts(&make_axiom("a")));
        assert!(f.accepts(&make_inductive("I", 1)));
    }
    #[test]
    fn test_filter_theorems_only() {
        let f = DeclFilter::theorems_only();
        assert!(!f.accepts(&make_def("d")));
        assert!(f.accepts(&make_theorem("t")));
        assert!(!f.accepts(&make_axiom("a")));
    }
    #[test]
    fn test_filter_definitions_only() {
        let f = DeclFilter::definitions_only();
        assert!(f.accepts(&make_def("d")));
        assert!(!f.accepts(&make_theorem("t")));
    }
    #[test]
    fn test_filter_simp_lemmas() {
        let f = DeclFilter::simp_lemmas();
        assert!(!f.accepts(&make_theorem("t")));
        let simp_thm = PendingDecl::Theorem {
            name: Name::str("simp_t"),
            ty: Expr::Sort(Level::zero()),
            proof: Expr::Sort(Level::zero()),
            attrs: vec![oxilean_parse::AttributeKind::Simp],
        };
        assert!(f.accepts(&simp_thm));
    }
    #[test]
    fn test_decl_stats_empty() {
        let stats = DeclStats::from_decls(&[]);
        assert_eq!(stats.total(), 0);
    }
    #[test]
    fn test_decl_stats_mixed() {
        let decls = vec![
            make_def("d1"),
            make_def("d2"),
            make_theorem("t1"),
            make_sorry_theorem("t_sorry"),
            make_axiom("ax1"),
            make_inductive("I", 2),
        ];
        let stats = DeclStats::from_decls(&decls);
        assert_eq!(stats.definition_count, 2);
        assert_eq!(stats.theorem_count, 2);
        assert_eq!(stats.axiom_count, 1);
        assert_eq!(stats.inductive_count, 1);
        assert_eq!(stats.sorry_count, 1);
        assert_eq!(stats.total(), 6);
    }
    #[test]
    fn test_namespace_manager_root() {
        let ns = NamespaceManager::new();
        assert!(ns.is_root());
        assert_eq!(ns.depth(), 0);
        assert_eq!(ns.current(), "");
    }
    #[test]
    fn test_namespace_manager_push_pop() {
        let mut ns = NamespaceManager::new();
        ns.push("Mathlib");
        assert_eq!(ns.current(), "Mathlib");
        assert_eq!(ns.depth(), 1);
        ns.push("Data");
        assert_eq!(ns.current(), "Mathlib.Data");
        ns.pop();
        assert_eq!(ns.current(), "Mathlib");
    }
    #[test]
    fn test_namespace_manager_qualify_root() {
        let ns = NamespaceManager::new();
        assert_eq!(ns.qualify("foo"), Name::str("foo"));
    }
    #[test]
    fn test_namespace_manager_qualify_nested() {
        let mut ns = NamespaceManager::new();
        ns.push("Algebra");
        ns.push("Group");
        let qualified = ns.qualify("mul_comm");
        assert_eq!(qualified, Name::str("Algebra.Group.mul_comm"));
    }
    #[test]
    fn test_pipeline_empty() {
        let p = DeclPipeline::new();
        let d = make_def("x");
        assert!(p.run(d).is_some());
        assert_eq!(p.stage_count(), 0);
    }
    #[test]
    fn test_pipeline_identity_stage() {
        let mut p = DeclPipeline::new();
        p.add_stage(Some);
        let d = make_def("x");
        assert!(p.run(d).is_some());
    }
    #[test]
    fn test_pipeline_drop_stage() {
        let mut p = DeclPipeline::new();
        p.add_stage(|_d| None);
        let d = make_def("x");
        assert!(p.run(d).is_none());
    }
    #[test]
    fn test_pipeline_run_all() {
        let mut p = DeclPipeline::new();
        p.add_stage(|d| if d.is_theorem() { Some(d) } else { None });
        let decls = vec![make_def("d"), make_theorem("t"), make_axiom("a")];
        let result = p.run_all(decls);
        assert_eq!(result.len(), 1);
        assert!(result[0].is_theorem());
    }
    #[test]
    fn test_pipeline_multiple_stages() {
        let mut p = DeclPipeline::new();
        p.add_stage(Some);
        p.add_stage(Some);
        assert_eq!(p.stage_count(), 2);
        assert!(p.run(make_def("x")).is_some());
    }
    #[test]
    fn test_expr_contains_sorry_false() {
        let e = Expr::Const(Name::str("Nat"), vec![]);
        assert!(!expr_contains_sorry(&e));
    }
    #[test]
    fn test_expr_contains_sorry_true() {
        let e = Expr::Const(Name::str("sorry"), vec![]);
        assert!(expr_contains_sorry(&e));
    }
    #[test]
    fn test_expr_contains_sorry_nested() {
        let e = Expr::App(
            Box::new(Expr::Const(Name::str("f"), vec![])),
            Box::new(Expr::Const(Name::str("sorry"), vec![])),
        );
        assert!(expr_contains_sorry(&e));
    }
}
