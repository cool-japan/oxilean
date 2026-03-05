//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet};

use super::types::{
    CostModel, DefinitionSite, FreeVarCollector, LcnfAlt, LcnfArg, LcnfExpr, LcnfFunDecl,
    LcnfLetValue, LcnfModule, LcnfParam, LcnfType, LcnfVarId, PrettyConfig, Substitution,
    UsageCounter, ValidationError,
};

/// Mapping from original names to mangled LCNF names.
pub type NameMap = HashMap<String, String>;
/// Immutable visitor trait for LCNF expressions.
pub trait LcnfVisitor {
    fn visit_expr(&mut self, expr: &LcnfExpr) {
        walk_expr(self, expr);
    }
    fn visit_let_value(&mut self, val: &LcnfLetValue) {
        walk_let_value(self, val);
    }
    fn visit_arg(&mut self, _arg: &LcnfArg) {}
    fn visit_type(&mut self, _ty: &LcnfType) {}
    fn visit_alt(&mut self, alt: &LcnfAlt) {
        walk_alt(self, alt);
    }
    fn visit_fun_decl(&mut self, decl: &LcnfFunDecl) {
        walk_fun_decl(self, decl);
    }
    fn visit_param(&mut self, _param: &LcnfParam) {}
}
/// Recursively walk children of an expression.
pub fn walk_expr<V: LcnfVisitor + ?Sized>(visitor: &mut V, expr: &LcnfExpr) {
    match expr {
        LcnfExpr::Let {
            ty, value, body, ..
        } => {
            visitor.visit_type(ty);
            visitor.visit_let_value(value);
            visitor.visit_expr(body);
        }
        LcnfExpr::Case {
            scrutinee_ty,
            alts,
            default,
            ..
        } => {
            visitor.visit_type(scrutinee_ty);
            for alt in alts {
                visitor.visit_alt(alt);
            }
            if let Some(def) = default {
                visitor.visit_expr(def);
            }
        }
        LcnfExpr::Return(arg) => visitor.visit_arg(arg),
        LcnfExpr::Unreachable => {}
        LcnfExpr::TailCall(func, args) => {
            visitor.visit_arg(func);
            for arg in args {
                visitor.visit_arg(arg);
            }
        }
    }
}
/// Recursively walk children of a let-bound value.
pub fn walk_let_value<V: LcnfVisitor + ?Sized>(visitor: &mut V, val: &LcnfLetValue) {
    match val {
        LcnfLetValue::App(func, args) => {
            visitor.visit_arg(func);
            for arg in args {
                visitor.visit_arg(arg);
            }
        }
        LcnfLetValue::Proj(..) => {}
        LcnfLetValue::Ctor(_, _, args) => {
            for arg in args {
                visitor.visit_arg(arg);
            }
        }
        LcnfLetValue::Lit(_)
        | LcnfLetValue::Erased
        | LcnfLetValue::FVar(_)
        | LcnfLetValue::Reset(_)
        | LcnfLetValue::Reuse(_, _, _, _) => {}
    }
}
/// Recursively walk children of a case alternative.
pub fn walk_alt<V: LcnfVisitor + ?Sized>(visitor: &mut V, alt: &LcnfAlt) {
    for param in &alt.params {
        visitor.visit_param(param);
    }
    visitor.visit_expr(&alt.body);
}
/// Recursively walk children of a function declaration.
pub fn walk_fun_decl<V: LcnfVisitor + ?Sized>(visitor: &mut V, decl: &LcnfFunDecl) {
    for param in &decl.params {
        visitor.visit_param(param);
    }
    visitor.visit_type(&decl.ret_type);
    visitor.visit_expr(&decl.body);
}
/// Mutable visitor trait for in-place mutation of LCNF expressions.
pub trait LcnfMutVisitor {
    fn visit_expr_mut(&mut self, expr: &mut LcnfExpr) {
        walk_expr_mut(self, expr);
    }
    fn visit_let_value_mut(&mut self, val: &mut LcnfLetValue) {
        walk_let_value_mut(self, val);
    }
    fn visit_arg_mut(&mut self, _arg: &mut LcnfArg) {}
    fn visit_type_mut(&mut self, _ty: &mut LcnfType) {}
    fn visit_alt_mut(&mut self, alt: &mut LcnfAlt) {
        walk_alt_mut(self, alt);
    }
    fn visit_fun_decl_mut(&mut self, decl: &mut LcnfFunDecl) {
        walk_fun_decl_mut(self, decl);
    }
    fn visit_param_mut(&mut self, _param: &mut LcnfParam) {}
}
/// Walk children of an expression mutably.
pub fn walk_expr_mut<V: LcnfMutVisitor + ?Sized>(visitor: &mut V, expr: &mut LcnfExpr) {
    match expr {
        LcnfExpr::Let {
            ty, value, body, ..
        } => {
            visitor.visit_type_mut(ty);
            visitor.visit_let_value_mut(value);
            visitor.visit_expr_mut(body);
        }
        LcnfExpr::Case {
            scrutinee_ty,
            alts,
            default,
            ..
        } => {
            visitor.visit_type_mut(scrutinee_ty);
            for alt in alts {
                visitor.visit_alt_mut(alt);
            }
            if let Some(def) = default {
                visitor.visit_expr_mut(def);
            }
        }
        LcnfExpr::Return(arg) => visitor.visit_arg_mut(arg),
        LcnfExpr::Unreachable => {}
        LcnfExpr::TailCall(func, args) => {
            visitor.visit_arg_mut(func);
            for arg in args {
                visitor.visit_arg_mut(arg);
            }
        }
    }
}
/// Walk children of a let-bound value mutably.
pub fn walk_let_value_mut<V: LcnfMutVisitor + ?Sized>(visitor: &mut V, val: &mut LcnfLetValue) {
    match val {
        LcnfLetValue::App(func, args) => {
            visitor.visit_arg_mut(func);
            for arg in args {
                visitor.visit_arg_mut(arg);
            }
        }
        LcnfLetValue::Proj(..) => {}
        LcnfLetValue::Ctor(_, _, args) => {
            for arg in args {
                visitor.visit_arg_mut(arg);
            }
        }
        LcnfLetValue::Lit(_)
        | LcnfLetValue::Erased
        | LcnfLetValue::FVar(_)
        | LcnfLetValue::Reset(_)
        | LcnfLetValue::Reuse(_, _, _, _) => {}
    }
}
/// Walk children of a case alternative mutably.
pub fn walk_alt_mut<V: LcnfMutVisitor + ?Sized>(visitor: &mut V, alt: &mut LcnfAlt) {
    for param in &mut alt.params {
        visitor.visit_param_mut(param);
    }
    visitor.visit_expr_mut(&mut alt.body);
}
/// Walk children of a function declaration mutably.
pub fn walk_fun_decl_mut<V: LcnfMutVisitor + ?Sized>(visitor: &mut V, decl: &mut LcnfFunDecl) {
    for param in &mut decl.params {
        visitor.visit_param_mut(param);
    }
    visitor.visit_type_mut(&mut decl.ret_type);
    visitor.visit_expr_mut(&mut decl.body);
}
/// Bottom-up transformation trait (folder).
pub trait LcnfFolder {
    fn fold_expr(&mut self, expr: LcnfExpr) -> LcnfExpr {
        match expr {
            LcnfExpr::Let {
                id,
                name,
                ty,
                value,
                body,
            } => {
                let new_value = self.fold_let_value(value);
                let new_body = self.fold_expr(*body);
                LcnfExpr::Let {
                    id,
                    name,
                    ty,
                    value: new_value,
                    body: Box::new(new_body),
                }
            }
            LcnfExpr::Case {
                scrutinee,
                scrutinee_ty,
                alts,
                default,
            } => {
                let new_alts = alts
                    .into_iter()
                    .map(|alt| {
                        let new_body = self.fold_expr(alt.body);
                        LcnfAlt {
                            ctor_name: alt.ctor_name,
                            ctor_tag: alt.ctor_tag,
                            params: alt.params,
                            body: new_body,
                        }
                    })
                    .collect();
                let new_default = default.map(|d| Box::new(self.fold_expr(*d)));
                LcnfExpr::Case {
                    scrutinee,
                    scrutinee_ty,
                    alts: new_alts,
                    default: new_default,
                }
            }
            other => other,
        }
    }
    fn fold_let_value(&mut self, val: LcnfLetValue) -> LcnfLetValue {
        val
    }
}
/// Collect all free variable IDs in an expression.
pub fn free_vars(expr: &LcnfExpr) -> HashSet<LcnfVarId> {
    let mut collector = FreeVarCollector::new();
    collector.collect_expr(expr);
    collector.free
}
/// Collect all let-bound variable IDs in an expression.
pub fn bound_vars(expr: &LcnfExpr) -> HashSet<LcnfVarId> {
    let mut result = HashSet::new();
    collect_bound_vars(expr, &mut result);
    result
}
pub(super) fn collect_bound_vars(expr: &LcnfExpr, result: &mut HashSet<LcnfVarId>) {
    match expr {
        LcnfExpr::Let { id, body, .. } => {
            result.insert(*id);
            collect_bound_vars(body, result);
        }
        LcnfExpr::Case { alts, default, .. } => {
            for alt in alts {
                for param in &alt.params {
                    result.insert(param.id);
                }
                collect_bound_vars(&alt.body, result);
            }
            if let Some(def) = default {
                collect_bound_vars(def, result);
            }
        }
        LcnfExpr::Return(_) | LcnfExpr::Unreachable | LcnfExpr::TailCall(..) => {}
    }
}
/// Collect all variable IDs appearing in an expression (free + bound).
pub fn all_vars(expr: &LcnfExpr) -> HashSet<LcnfVarId> {
    let mut result = free_vars(expr);
    result.extend(bound_vars(expr));
    result
}
/// Count how many times each variable is referenced in an expression.
pub fn usage_counts(expr: &LcnfExpr) -> HashMap<LcnfVarId, usize> {
    let mut counter = UsageCounter::new();
    counter.count_expr(expr);
    counter.counts
}
/// Check whether all variables are used at most once.
pub fn is_linear(expr: &LcnfExpr) -> bool {
    usage_counts(expr).values().all(|&c| c <= 1)
}
/// Collect all definition sites in an expression.
pub fn definition_sites(expr: &LcnfExpr) -> Vec<DefinitionSite> {
    let mut sites = Vec::new();
    collect_definition_sites(expr, 0, &mut sites);
    sites
}
pub(super) fn collect_definition_sites(
    expr: &LcnfExpr,
    depth: usize,
    sites: &mut Vec<DefinitionSite>,
) {
    match expr {
        LcnfExpr::Let {
            id, name, ty, body, ..
        } => {
            sites.push(DefinitionSite {
                var: *id,
                name: name.clone(),
                ty: ty.clone(),
                depth,
            });
            collect_definition_sites(body, depth + 1, sites);
        }
        LcnfExpr::Case { alts, default, .. } => {
            for alt in alts {
                for param in &alt.params {
                    sites.push(DefinitionSite {
                        var: param.id,
                        name: param.name.clone(),
                        ty: param.ty.clone(),
                        depth: depth + 1,
                    });
                }
                collect_definition_sites(&alt.body, depth + 1, sites);
            }
            if let Some(def) = default {
                collect_definition_sites(def, depth + 1, sites);
            }
        }
        LcnfExpr::Return(_) | LcnfExpr::Unreachable | LcnfExpr::TailCall(..) => {}
    }
}
/// Apply a substitution to an argument.
pub fn substitute_arg(arg: &LcnfArg, subst: &Substitution) -> LcnfArg {
    if let LcnfArg::Var(id) = arg {
        if let Some(replacement) = subst.get(id) {
            return replacement.clone();
        }
    }
    arg.clone()
}
/// Apply a substitution to a let-bound value.
pub fn substitute_let_value(val: &LcnfLetValue, subst: &Substitution) -> LcnfLetValue {
    match val {
        LcnfLetValue::App(func, args) => LcnfLetValue::App(
            substitute_arg(func, subst),
            args.iter().map(|a| substitute_arg(a, subst)).collect(),
        ),
        LcnfLetValue::Proj(name, idx, var) => {
            if let Some(LcnfArg::Var(new_var)) = subst.get(var) {
                LcnfLetValue::Proj(name.clone(), *idx, *new_var)
            } else {
                val.clone()
            }
        }
        LcnfLetValue::Ctor(name, tag, args) => LcnfLetValue::Ctor(
            name.clone(),
            *tag,
            args.iter().map(|a| substitute_arg(a, subst)).collect(),
        ),
        LcnfLetValue::FVar(id) => {
            if let Some(LcnfArg::Var(new_id)) = subst.get(id) {
                LcnfLetValue::FVar(*new_id)
            } else {
                val.clone()
            }
        }
        LcnfLetValue::Lit(_)
        | LcnfLetValue::Erased
        | LcnfLetValue::Reset(_)
        | LcnfLetValue::Reuse(_, _, _, _) => val.clone(),
    }
}
/// Apply a substitution to an expression.
pub fn substitute_expr(expr: &LcnfExpr, subst: &Substitution) -> LcnfExpr {
    match expr {
        LcnfExpr::Let {
            id,
            name,
            ty,
            value,
            body,
        } => {
            let new_value = substitute_let_value(value, subst);
            let mut inner_subst = subst.clone();
            inner_subst.0.remove(id);
            LcnfExpr::Let {
                id: *id,
                name: name.clone(),
                ty: ty.clone(),
                value: new_value,
                body: Box::new(substitute_expr(body, &inner_subst)),
            }
        }
        LcnfExpr::Case {
            scrutinee,
            scrutinee_ty,
            alts,
            default,
        } => {
            let new_scrutinee = if let Some(LcnfArg::Var(new_id)) = subst.get(scrutinee) {
                *new_id
            } else {
                *scrutinee
            };
            let new_alts = alts
                .iter()
                .map(|alt| {
                    let mut inner_subst = subst.clone();
                    for param in &alt.params {
                        inner_subst.0.remove(&param.id);
                    }
                    LcnfAlt {
                        ctor_name: alt.ctor_name.clone(),
                        ctor_tag: alt.ctor_tag,
                        params: alt.params.clone(),
                        body: substitute_expr(&alt.body, &inner_subst),
                    }
                })
                .collect();
            let new_default = default
                .as_ref()
                .map(|d| Box::new(substitute_expr(d, subst)));
            LcnfExpr::Case {
                scrutinee: new_scrutinee,
                scrutinee_ty: scrutinee_ty.clone(),
                alts: new_alts,
                default: new_default,
            }
        }
        LcnfExpr::Return(arg) => LcnfExpr::Return(substitute_arg(arg, subst)),
        LcnfExpr::Unreachable => LcnfExpr::Unreachable,
        LcnfExpr::TailCall(func, args) => LcnfExpr::TailCall(
            substitute_arg(func, subst),
            args.iter().map(|a| substitute_arg(a, subst)).collect(),
        ),
    }
}
/// Rename variables according to the given mapping.
pub fn rename_vars(expr: &LcnfExpr, rename: &HashMap<LcnfVarId, LcnfVarId>) -> LcnfExpr {
    let subst = Substitution(
        rename
            .iter()
            .map(|(old, new)| (*old, LcnfArg::Var(*new)))
            .collect(),
    );
    rename_expr_inner(expr, rename, &subst)
}
pub(super) fn rename_expr_inner(
    expr: &LcnfExpr,
    rename: &HashMap<LcnfVarId, LcnfVarId>,
    subst: &Substitution,
) -> LcnfExpr {
    match expr {
        LcnfExpr::Let {
            id,
            name,
            ty,
            value,
            body,
        } => {
            let new_id = rename.get(id).copied().unwrap_or(*id);
            LcnfExpr::Let {
                id: new_id,
                name: name.clone(),
                ty: ty.clone(),
                value: substitute_let_value(value, subst),
                body: Box::new(rename_expr_inner(body, rename, subst)),
            }
        }
        LcnfExpr::Case {
            scrutinee,
            scrutinee_ty,
            alts,
            default,
        } => {
            let new_scrutinee = rename.get(scrutinee).copied().unwrap_or(*scrutinee);
            let new_alts = alts
                .iter()
                .map(|alt| {
                    let new_params: Vec<LcnfParam> = alt
                        .params
                        .iter()
                        .map(|p| LcnfParam {
                            id: rename.get(&p.id).copied().unwrap_or(p.id),
                            name: p.name.clone(),
                            ty: p.ty.clone(),
                            erased: p.erased,
                            borrowed: false,
                        })
                        .collect();
                    LcnfAlt {
                        ctor_name: alt.ctor_name.clone(),
                        ctor_tag: alt.ctor_tag,
                        params: new_params,
                        body: rename_expr_inner(&alt.body, rename, subst),
                    }
                })
                .collect();
            let new_default = default
                .as_ref()
                .map(|d| Box::new(rename_expr_inner(d, rename, subst)));
            LcnfExpr::Case {
                scrutinee: new_scrutinee,
                scrutinee_ty: scrutinee_ty.clone(),
                alts: new_alts,
                default: new_default,
            }
        }
        LcnfExpr::Return(arg) => LcnfExpr::Return(substitute_arg(arg, subst)),
        LcnfExpr::Unreachable => LcnfExpr::Unreachable,
        LcnfExpr::TailCall(func, args) => LcnfExpr::TailCall(
            substitute_arg(func, subst),
            args.iter().map(|a| substitute_arg(a, subst)).collect(),
        ),
    }
}
/// Check structural equality up to variable renaming (alpha-equivalence).
pub fn alpha_equiv(e1: &LcnfExpr, e2: &LcnfExpr) -> bool {
    let mut l2r: HashMap<LcnfVarId, LcnfVarId> = HashMap::new();
    let mut r2l: HashMap<LcnfVarId, LcnfVarId> = HashMap::new();
    alpha_equiv_expr(e1, e2, &mut l2r, &mut r2l)
}
pub(super) fn alpha_equiv_var(
    v1: LcnfVarId,
    v2: LcnfVarId,
    l2r: &HashMap<LcnfVarId, LcnfVarId>,
    r2l: &HashMap<LcnfVarId, LcnfVarId>,
) -> bool {
    match (l2r.get(&v1), r2l.get(&v2)) {
        (Some(&mapped), Some(&mapped_back)) => mapped == v2 && mapped_back == v1,
        (None, None) => v1 == v2,
        _ => false,
    }
}
pub(super) fn alpha_equiv_arg(
    a1: &LcnfArg,
    a2: &LcnfArg,
    l2r: &HashMap<LcnfVarId, LcnfVarId>,
    r2l: &HashMap<LcnfVarId, LcnfVarId>,
) -> bool {
    match (a1, a2) {
        (LcnfArg::Var(v1), LcnfArg::Var(v2)) => alpha_equiv_var(*v1, *v2, l2r, r2l),
        (LcnfArg::Lit(l1), LcnfArg::Lit(l2)) => l1 == l2,
        (LcnfArg::Erased, LcnfArg::Erased) => true,
        (LcnfArg::Type(t1), LcnfArg::Type(t2)) => t1 == t2,
        _ => false,
    }
}
pub(super) fn alpha_equiv_let_value(
    v1: &LcnfLetValue,
    v2: &LcnfLetValue,
    l2r: &HashMap<LcnfVarId, LcnfVarId>,
    r2l: &HashMap<LcnfVarId, LcnfVarId>,
) -> bool {
    match (v1, v2) {
        (LcnfLetValue::App(f1, a1), LcnfLetValue::App(f2, a2)) => {
            alpha_equiv_arg(f1, f2, l2r, r2l)
                && a1.len() == a2.len()
                && a1
                    .iter()
                    .zip(a2.iter())
                    .all(|(x, y)| alpha_equiv_arg(x, y, l2r, r2l))
        }
        (LcnfLetValue::Proj(n1, i1, var1), LcnfLetValue::Proj(n2, i2, var2)) => {
            n1 == n2 && i1 == i2 && alpha_equiv_var(*var1, *var2, l2r, r2l)
        }
        (LcnfLetValue::Ctor(n1, t1, a1), LcnfLetValue::Ctor(n2, t2, a2)) => {
            n1 == n2
                && t1 == t2
                && a1.len() == a2.len()
                && a1
                    .iter()
                    .zip(a2.iter())
                    .all(|(x, y)| alpha_equiv_arg(x, y, l2r, r2l))
        }
        (LcnfLetValue::Lit(l1), LcnfLetValue::Lit(l2)) => l1 == l2,
        (LcnfLetValue::Erased, LcnfLetValue::Erased) => true,
        (LcnfLetValue::FVar(id1), LcnfLetValue::FVar(id2)) => alpha_equiv_var(*id1, *id2, l2r, r2l),
        _ => false,
    }
}
#[allow(clippy::too_many_arguments)]
pub(super) fn alpha_equiv_expr(
    e1: &LcnfExpr,
    e2: &LcnfExpr,
    l2r: &mut HashMap<LcnfVarId, LcnfVarId>,
    r2l: &mut HashMap<LcnfVarId, LcnfVarId>,
) -> bool {
    match (e1, e2) {
        (
            LcnfExpr::Let {
                id: id1,
                ty: ty1,
                value: val1,
                body: body1,
                ..
            },
            LcnfExpr::Let {
                id: id2,
                ty: ty2,
                value: val2,
                body: body2,
                ..
            },
        ) => {
            if ty1 != ty2 || !alpha_equiv_let_value(val1, val2, l2r, r2l) {
                return false;
            }
            l2r.insert(*id1, *id2);
            r2l.insert(*id2, *id1);
            let result = alpha_equiv_expr(body1, body2, l2r, r2l);
            l2r.remove(id1);
            r2l.remove(id2);
            result
        }
        (
            LcnfExpr::Case {
                scrutinee: s1,
                scrutinee_ty: st1,
                alts: alts1,
                default: def1,
            },
            LcnfExpr::Case {
                scrutinee: s2,
                scrutinee_ty: st2,
                alts: alts2,
                default: def2,
            },
        ) => {
            if !alpha_equiv_var(*s1, *s2, l2r, r2l) || st1 != st2 || alts1.len() != alts2.len() {
                return false;
            }
            for (a1, a2) in alts1.iter().zip(alts2.iter()) {
                if a1.ctor_name != a2.ctor_name
                    || a1.ctor_tag != a2.ctor_tag
                    || a1.params.len() != a2.params.len()
                {
                    return false;
                }
                for (p1, p2) in a1.params.iter().zip(a2.params.iter()) {
                    l2r.insert(p1.id, p2.id);
                    r2l.insert(p2.id, p1.id);
                }
                let ok = alpha_equiv_expr(&a1.body, &a2.body, l2r, r2l);
                for (p1, p2) in a1.params.iter().zip(a2.params.iter()) {
                    l2r.remove(&p1.id);
                    r2l.remove(&p2.id);
                }
                if !ok {
                    return false;
                }
            }
            match (def1, def2) {
                (Some(d1), Some(d2)) => alpha_equiv_expr(d1, d2, l2r, r2l),
                (None, None) => true,
                _ => false,
            }
        }
        (LcnfExpr::Return(a1), LcnfExpr::Return(a2)) => alpha_equiv_arg(a1, a2, l2r, r2l),
        (LcnfExpr::Unreachable, LcnfExpr::Unreachable) => true,
        (LcnfExpr::TailCall(f1, a1), LcnfExpr::TailCall(f2, a2)) => {
            alpha_equiv_arg(f1, f2, l2r, r2l)
                && a1.len() == a2.len()
                && a1
                    .iter()
                    .zip(a2.iter())
                    .all(|(x, y)| alpha_equiv_arg(x, y, l2r, r2l))
        }
        _ => false,
    }
}
/// Count the number of AST nodes in an expression.
pub fn expr_size(expr: &LcnfExpr) -> usize {
    match expr {
        LcnfExpr::Let { value, body, .. } => 1 + let_value_size(value) + expr_size(body),
        LcnfExpr::Case { alts, default, .. } => {
            let alt_size: usize = alts.iter().map(|a| 1 + expr_size(&a.body)).sum();
            let def_size = default.as_ref().map_or(0, |d| expr_size(d));
            1 + alt_size + def_size
        }
        LcnfExpr::Return(_) => 1,
        LcnfExpr::Unreachable => 1,
        LcnfExpr::TailCall(_, args) => 1 + args.len(),
    }
}
pub(super) fn let_value_size(val: &LcnfLetValue) -> usize {
    match val {
        LcnfLetValue::App(_, args) => 1 + args.len(),
        LcnfLetValue::Proj(..) => 1,
        LcnfLetValue::Ctor(_, _, args) => 1 + args.len(),
        LcnfLetValue::Lit(_)
        | LcnfLetValue::Erased
        | LcnfLetValue::FVar(_)
        | LcnfLetValue::Reset(_)
        | LcnfLetValue::Reuse(_, _, _, _) => 1,
    }
}
/// Compute the maximum nesting depth of an expression.
pub fn expr_depth(expr: &LcnfExpr) -> usize {
    match expr {
        LcnfExpr::Let { body, .. } => 1 + expr_depth(body),
        LcnfExpr::Case { alts, default, .. } => {
            let max_alt = alts.iter().map(|a| expr_depth(&a.body)).max().unwrap_or(0);
            let def_depth = default.as_ref().map_or(0, |d| expr_depth(d));
            1 + max_alt.max(def_depth)
        }
        LcnfExpr::Return(_) | LcnfExpr::Unreachable | LcnfExpr::TailCall(..) => 1,
    }
}
/// Compute a heuristic inlining cost for a function declaration.
pub fn compute_inline_cost(decl: &LcnfFunDecl) -> usize {
    let base = expr_size(&decl.body);
    let depth_penalty = expr_depth(&decl.body);
    let branch_penalty = count_branches(&decl.body) * 5;
    let recursive_penalty = if decl.is_recursive { 100 } else { 0 };
    let param_bonus = if decl.params.len() <= 2 {
        0
    } else {
        decl.params.len() * 2
    };
    base + depth_penalty + branch_penalty + recursive_penalty + param_bonus
}
/// Estimate the runtime cost of an expression under the given cost model.
pub fn estimate_runtime_cost(expr: &LcnfExpr, model: &CostModel) -> u64 {
    match expr {
        LcnfExpr::Let { value, body, .. } => {
            let val_cost = match value {
                LcnfLetValue::App(..) | LcnfLetValue::Ctor(..) => model.app_cost,
                LcnfLetValue::Proj(..) | LcnfLetValue::Lit(_) | LcnfLetValue::FVar(_) => {
                    model.let_cost
                }
                LcnfLetValue::Erased | LcnfLetValue::Reset(_) | LcnfLetValue::Reuse(_, _, _, _) => {
                    0
                }
            };
            model.let_cost + val_cost + estimate_runtime_cost(body, model)
        }
        LcnfExpr::Case { alts, default, .. } => {
            let max_alt_cost = alts
                .iter()
                .map(|a| estimate_runtime_cost(&a.body, model))
                .max()
                .unwrap_or(0);
            let def_cost = default
                .as_ref()
                .map_or(0, |d| estimate_runtime_cost(d, model));
            model.case_cost
                + model.branch_penalty * (alts.len() as u64)
                + max_alt_cost.max(def_cost)
        }
        LcnfExpr::Return(_) => model.return_cost,
        LcnfExpr::Unreachable => 0,
        LcnfExpr::TailCall(_, args) => model.app_cost + (args.len() as u64),
    }
}
/// Estimate the number of heap allocations (from constructor applications).
pub fn count_allocations(expr: &LcnfExpr) -> usize {
    match expr {
        LcnfExpr::Let { value, body, .. } => {
            let alloc = match value {
                LcnfLetValue::Ctor(_, _, args) if !args.is_empty() => 1,
                _ => 0,
            };
            alloc + count_allocations(body)
        }
        LcnfExpr::Case { alts, default, .. } => {
            let alt_allocs: usize = alts.iter().map(|a| count_allocations(&a.body)).sum();
            let def_allocs = default.as_ref().map_or(0, |d| count_allocations(d));
            alt_allocs + def_allocs
        }
        LcnfExpr::Return(_) | LcnfExpr::Unreachable | LcnfExpr::TailCall(..) => 0,
    }
}
/// Count the number of case splits in an expression.
pub fn count_branches(expr: &LcnfExpr) -> usize {
    match expr {
        LcnfExpr::Let { body, .. } => count_branches(body),
        LcnfExpr::Case { alts, default, .. } => {
            let inner: usize = alts.iter().map(|a| count_branches(&a.body)).sum();
            let def_branches = default.as_ref().map_or(0, |d| count_branches(d));
            1 + inner + def_branches
        }
        LcnfExpr::Return(_) | LcnfExpr::Unreachable | LcnfExpr::TailCall(..) => 0,
    }
}
/// Validate an expression with respect to a set of bound variables.
pub fn validate_expr(expr: &LcnfExpr, bound: &HashSet<LcnfVarId>) -> Result<(), ValidationError> {
    match expr {
        LcnfExpr::Let {
            id, value, body, ..
        } => {
            validate_let_value(value, bound)?;
            let mut new_bound = bound.clone();
            if !new_bound.insert(*id) {
                return Err(ValidationError::DuplicateBinding(*id));
            }
            validate_expr(body, &new_bound)
        }
        LcnfExpr::Case {
            scrutinee,
            alts,
            default,
            ..
        } => {
            if !bound.contains(scrutinee) {
                return Err(ValidationError::UnboundVariable(*scrutinee));
            }
            if alts.is_empty() && default.is_none() {
                return Err(ValidationError::EmptyCase);
            }
            for alt in alts {
                let mut alt_bound = bound.clone();
                for param in &alt.params {
                    if !alt_bound.insert(param.id) {
                        return Err(ValidationError::DuplicateBinding(param.id));
                    }
                }
                validate_expr(&alt.body, &alt_bound)?;
            }
            if let Some(def) = default {
                validate_expr(def, bound)?;
            }
            Ok(())
        }
        LcnfExpr::Return(arg) => validate_arg_bound(arg, bound),
        LcnfExpr::Unreachable => Ok(()),
        LcnfExpr::TailCall(func, args) => {
            validate_arg_bound(func, bound)?;
            for arg in args {
                validate_arg_bound(arg, bound)?;
            }
            Ok(())
        }
    }
}
pub(super) fn validate_arg_bound(
    arg: &LcnfArg,
    bound: &HashSet<LcnfVarId>,
) -> Result<(), ValidationError> {
    if let LcnfArg::Var(id) = arg {
        if !bound.contains(id) {
            return Err(ValidationError::UnboundVariable(*id));
        }
    }
    Ok(())
}
pub(super) fn validate_let_value(
    val: &LcnfLetValue,
    bound: &HashSet<LcnfVarId>,
) -> Result<(), ValidationError> {
    match val {
        LcnfLetValue::App(func, args) => {
            validate_arg_bound(func, bound)?;
            for arg in args {
                validate_arg_bound(arg, bound)?;
            }
            Ok(())
        }
        LcnfLetValue::Proj(_, _, var) => {
            if !bound.contains(var) {
                Err(ValidationError::UnboundVariable(*var))
            } else {
                Ok(())
            }
        }
        LcnfLetValue::Ctor(_, _, args) => {
            for arg in args {
                validate_arg_bound(arg, bound)?;
            }
            Ok(())
        }
        LcnfLetValue::FVar(id) => {
            if !bound.contains(id) {
                Err(ValidationError::UnboundVariable(*id))
            } else {
                Ok(())
            }
        }
        LcnfLetValue::Lit(_)
        | LcnfLetValue::Erased
        | LcnfLetValue::Reset(_)
        | LcnfLetValue::Reuse(_, _, _, _) => Ok(()),
    }
}
/// Validate a function declaration.
pub fn validate_fun_decl(decl: &LcnfFunDecl) -> Result<(), ValidationError> {
    let mut bound = HashSet::new();
    for param in &decl.params {
        if !bound.insert(param.id) {
            return Err(ValidationError::DuplicateBinding(param.id));
        }
    }
    validate_expr(&decl.body, &bound)
}
/// Validate an entire module, collecting all errors.
pub fn validate_module(module: &LcnfModule) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();
    for decl in &module.fun_decls {
        if let Err(e) = validate_fun_decl(decl) {
            errors.push(e);
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
/// Check that the ANF invariant holds: all arguments are atomic.
pub fn check_anf_invariant(expr: &LcnfExpr) -> bool {
    match expr {
        LcnfExpr::Let { value, body, .. } => {
            check_let_value_anf(value) && check_anf_invariant(body)
        }
        LcnfExpr::Case { alts, default, .. } => {
            alts.iter().all(|a| check_anf_invariant(&a.body))
                && default.as_ref().is_none_or(|d| check_anf_invariant(d))
        }
        LcnfExpr::Return(_) | LcnfExpr::Unreachable | LcnfExpr::TailCall(..) => true,
    }
}
pub(super) fn check_let_value_anf(val: &LcnfLetValue) -> bool {
    match val {
        LcnfLetValue::App(func, args) => is_atomic_arg(func) && args.iter().all(is_atomic_arg),
        LcnfLetValue::Ctor(_, _, args) => args.iter().all(is_atomic_arg),
        _ => true,
    }
}
pub(super) fn is_atomic_arg(arg: &LcnfArg) -> bool {
    matches!(
        arg,
        LcnfArg::Var(_) | LcnfArg::Lit(_) | LcnfArg::Erased | LcnfArg::Type(_)
    )
}
/// Pretty-print an expression to a string.
pub fn pretty_print_expr(expr: &LcnfExpr, config: &PrettyConfig) -> String {
    let mut output = String::new();
    pp_expr(&mut output, expr, config, 0);
    output
}
pub(super) fn pp_indent(output: &mut String, config: &PrettyConfig, level: usize) {
    for _ in 0..level * config.indent {
        output.push(' ');
    }
}
pub(super) fn pp_arg(output: &mut String, arg: &LcnfArg, config: &PrettyConfig) {
    match arg {
        LcnfArg::Var(id) => output.push_str(&id.to_string()),
        LcnfArg::Lit(lit) => output.push_str(&lit.to_string()),
        LcnfArg::Erased => {
            if config.show_erased {
                output.push('◻');
            } else {
                output.push('_');
            }
        }
        LcnfArg::Type(ty) => {
            if config.show_types {
                output.push('@');
                output.push_str(&ty.to_string());
            } else {
                output.push('_');
            }
        }
    }
}
pub(super) fn pp_let_value(output: &mut String, val: &LcnfLetValue, config: &PrettyConfig) {
    match val {
        LcnfLetValue::App(func, args) => {
            pp_arg(output, func, config);
            output.push('(');
            for (i, a) in args.iter().enumerate() {
                if i > 0 {
                    output.push_str(", ");
                }
                pp_arg(output, a, config);
            }
            output.push(')');
        }
        LcnfLetValue::Proj(name, idx, var) => {
            output.push_str(&format!("{}.{} {}", name, idx, var));
        }
        LcnfLetValue::Ctor(name, tag, args) => {
            output.push_str(&format!("{}#{}", name, tag));
            if !args.is_empty() {
                output.push('(');
                for (i, a) in args.iter().enumerate() {
                    if i > 0 {
                        output.push_str(", ");
                    }
                    pp_arg(output, a, config);
                }
                output.push(')');
            }
        }
        LcnfLetValue::Lit(lit) => output.push_str(&lit.to_string()),
        LcnfLetValue::Erased => output.push_str("erased"),
        LcnfLetValue::FVar(id) => output.push_str(&id.to_string()),
        LcnfLetValue::Reset(var) => output.push_str(&format!("reset({})", var)),
        LcnfLetValue::Reuse(slot, name, tag, _) => {
            output.push_str(&format!("reuse({}, {}#{})", slot, name, tag))
        }
    }
}
pub(super) fn pp_expr(output: &mut String, expr: &LcnfExpr, config: &PrettyConfig, level: usize) {
    match expr {
        LcnfExpr::Let {
            id,
            name,
            ty,
            value,
            body,
        } => {
            pp_indent(output, config, level);
            output.push_str("let ");
            output.push_str(&id.to_string());
            if !name.is_empty() {
                output.push_str(&format!(" ({})", name));
            }
            if config.show_types {
                output.push_str(&format!(" : {}", ty));
            }
            output.push_str(" := ");
            pp_let_value(output, value, config);
            output.push('\n');
            pp_expr(output, body, config, level);
        }
        LcnfExpr::Case {
            scrutinee,
            scrutinee_ty,
            alts,
            default,
        } => {
            pp_indent(output, config, level);
            output.push_str(&format!("case {}", scrutinee));
            if config.show_types {
                output.push_str(&format!(" : {}", scrutinee_ty));
            }
            output.push_str(" of\n");
            for alt in alts {
                pp_indent(output, config, level + 1);
                output.push_str(&format!("| {}#{}", alt.ctor_name, alt.ctor_tag));
                for p in &alt.params {
                    output.push_str(&format!(" {}", p.id));
                }
                output.push_str(" =>\n");
                pp_expr(output, &alt.body, config, level + 2);
            }
            if let Some(def) = default {
                pp_indent(output, config, level + 1);
                output.push_str("| _ =>\n");
                pp_expr(output, def, config, level + 2);
            }
        }
        LcnfExpr::Return(arg) => {
            pp_indent(output, config, level);
            output.push_str("return ");
            pp_arg(output, arg, config);
            output.push('\n');
        }
        LcnfExpr::Unreachable => {
            pp_indent(output, config, level);
            output.push_str("unreachable\n");
        }
        LcnfExpr::TailCall(func, args) => {
            pp_indent(output, config, level);
            output.push_str("tailcall ");
            pp_arg(output, func, config);
            output.push('(');
            for (i, a) in args.iter().enumerate() {
                if i > 0 {
                    output.push_str(", ");
                }
                pp_arg(output, a, config);
            }
            output.push_str(")\n");
        }
    }
}
/// Pretty-print a function declaration.
pub fn pretty_print_fun_decl(decl: &LcnfFunDecl, config: &PrettyConfig) -> String {
    let mut output = String::new();
    output.push_str("def ");
    output.push_str(&decl.name);
    output.push('(');
    for (i, param) in decl.params.iter().enumerate() {
        if i > 0 {
            output.push_str(", ");
        }
        output.push_str(&format!("{}", param.id));
        if !param.name.is_empty() {
            output.push_str(&format!(" ({})", param.name));
        }
        if config.show_types {
            output.push_str(&format!(" : {}", param.ty));
        }
    }
    output.push(')');
    if config.show_types {
        output.push_str(&format!(" : {}", decl.ret_type));
    }
    if decl.is_recursive {
        output.push_str(" [rec]");
    }
    if decl.is_lifted {
        output.push_str(" [lifted]");
    }
    output.push_str(" :=\n");
    pp_expr(&mut output, &decl.body, config, 1);
    output
}
/// Pretty-print an entire module.
pub fn pretty_print_module(module: &LcnfModule, config: &PrettyConfig) -> String {
    let mut output = String::new();
    output.push_str(&format!("-- module {}\n", module.name));
    output.push_str(&format!(
        "-- {} decls, {} externs\n\n",
        module.fun_decls.len(),
        module.extern_decls.len()
    ));
    for decl in &module.extern_decls {
        output.push_str("extern ");
        output.push_str(&decl.name);
        output.push('(');
        for (i, param) in decl.params.iter().enumerate() {
            if i > 0 {
                output.push_str(", ");
            }
            if config.show_types {
                output.push_str(&format!("{} : {}", param.id, param.ty));
            } else {
                output.push_str(&format!("{}", param.id));
            }
        }
        output.push(')');
        if config.show_types {
            output.push_str(&format!(" : {}", decl.ret_type));
        }
        output.push('\n');
    }
    if !module.extern_decls.is_empty() {
        output.push('\n');
    }
    for decl in &module.fun_decls {
        output.push_str(&pretty_print_fun_decl(decl, config));
        output.push('\n');
    }
    output
}
/// Inline a single let binding by substituting its value into all uses.
pub fn inline_let(expr: LcnfExpr, var: LcnfVarId) -> LcnfExpr {
    match expr {
        LcnfExpr::Let {
            id,
            name,
            ty,
            value,
            body,
        } if id == var => {
            if let Some(arg) = let_value_to_arg(&value) {
                let mut subst = Substitution::new();
                subst.insert(id, arg);
                substitute_expr(&body, &subst)
            } else {
                LcnfExpr::Let {
                    id,
                    name,
                    ty,
                    value,
                    body,
                }
            }
        }
        LcnfExpr::Let {
            id,
            name,
            ty,
            value,
            body,
        } => LcnfExpr::Let {
            id,
            name,
            ty,
            value,
            body: Box::new(inline_let(*body, var)),
        },
        LcnfExpr::Case {
            scrutinee,
            scrutinee_ty,
            alts,
            default,
        } => LcnfExpr::Case {
            scrutinee,
            scrutinee_ty,
            alts: alts
                .into_iter()
                .map(|a| LcnfAlt {
                    ctor_name: a.ctor_name,
                    ctor_tag: a.ctor_tag,
                    params: a.params,
                    body: inline_let(a.body, var),
                })
                .collect(),
            default: default.map(|d| Box::new(inline_let(*d, var))),
        },
        other => other,
    }
}
/// Try to convert a let value to an atomic argument for inlining.
pub(super) fn let_value_to_arg(val: &LcnfLetValue) -> Option<LcnfArg> {
    match val {
        LcnfLetValue::Lit(lit) => Some(LcnfArg::Lit(lit.clone())),
        LcnfLetValue::Erased => Some(LcnfArg::Erased),
        LcnfLetValue::FVar(id) => Some(LcnfArg::Var(*id)),
        _ => None,
    }
}
/// Flatten nested let chains.
pub fn flatten_lets(expr: LcnfExpr) -> LcnfExpr {
    let mut bindings: Vec<(LcnfVarId, String, LcnfType, LcnfLetValue)> = Vec::new();
    let terminal = collect_lets(expr, &mut bindings);
    let mut result = flatten_lets_in_terminal(terminal);
    for (id, name, ty, value) in bindings.into_iter().rev() {
        result = LcnfExpr::Let {
            id,
            name,
            ty,
            value,
            body: Box::new(result),
        };
    }
    result
}
pub(super) fn collect_lets(
    expr: LcnfExpr,
    bindings: &mut Vec<(LcnfVarId, String, LcnfType, LcnfLetValue)>,
) -> LcnfExpr {
    match expr {
        LcnfExpr::Let {
            id,
            name,
            ty,
            value,
            body,
        } => {
            bindings.push((id, name, ty, value));
            collect_lets(*body, bindings)
        }
        other => other,
    }
}
pub(super) fn flatten_lets_in_terminal(expr: LcnfExpr) -> LcnfExpr {
    match expr {
        LcnfExpr::Case {
            scrutinee,
            scrutinee_ty,
            alts,
            default,
        } => LcnfExpr::Case {
            scrutinee,
            scrutinee_ty,
            alts: alts
                .into_iter()
                .map(|a| LcnfAlt {
                    ctor_name: a.ctor_name,
                    ctor_tag: a.ctor_tag,
                    params: a.params,
                    body: flatten_lets(a.body),
                })
                .collect(),
            default: default.map(|d| Box::new(flatten_lets(*d))),
        },
        other => other,
    }
}
/// Simplify a case with a single alternative into a let chain.
pub fn simplify_trivial_case(expr: LcnfExpr) -> LcnfExpr {
    match expr {
        LcnfExpr::Case {
            scrutinee,
            alts,
            default: None,
            ..
        } if alts.len() == 1 => {
            let alt = alts.into_iter().next().expect(
                "alts has exactly one element; guaranteed by pattern guard alts.len() == 1",
            );
            let mut result = simplify_trivial_case(alt.body);
            for (idx, param) in alt.params.iter().enumerate().rev() {
                result = LcnfExpr::Let {
                    id: param.id,
                    name: param.name.clone(),
                    ty: param.ty.clone(),
                    value: LcnfLetValue::Proj(alt.ctor_name.clone(), idx as u32, scrutinee),
                    body: Box::new(result),
                };
            }
            result
        }
        LcnfExpr::Let {
            id,
            name,
            ty,
            value,
            body,
        } => LcnfExpr::Let {
            id,
            name,
            ty,
            value,
            body: Box::new(simplify_trivial_case(*body)),
        },
        LcnfExpr::Case {
            scrutinee,
            scrutinee_ty,
            alts,
            default,
        } => LcnfExpr::Case {
            scrutinee,
            scrutinee_ty,
            alts: alts
                .into_iter()
                .map(|a| LcnfAlt {
                    ctor_name: a.ctor_name,
                    ctor_tag: a.ctor_tag,
                    params: a.params,
                    body: simplify_trivial_case(a.body),
                })
                .collect(),
            default: default.map(|d| Box::new(simplify_trivial_case(*d))),
        },
        other => other,
    }
}
/// Remove unused let bindings (dead code elimination).
pub fn remove_unused_lets(expr: LcnfExpr) -> LcnfExpr {
    match expr {
        LcnfExpr::Let {
            id,
            name,
            ty,
            value,
            body,
        } => {
            let new_body = remove_unused_lets(*body);
            let counts = usage_counts(&new_body);
            if counts.get(&id).copied().unwrap_or(0) == 0 {
                new_body
            } else {
                LcnfExpr::Let {
                    id,
                    name,
                    ty,
                    value,
                    body: Box::new(new_body),
                }
            }
        }
        LcnfExpr::Case {
            scrutinee,
            scrutinee_ty,
            alts,
            default,
        } => LcnfExpr::Case {
            scrutinee,
            scrutinee_ty,
            alts: alts
                .into_iter()
                .map(|a| LcnfAlt {
                    ctor_name: a.ctor_name,
                    ctor_tag: a.ctor_tag,
                    params: a.params,
                    body: remove_unused_lets(a.body),
                })
                .collect(),
            default: default.map(|d| Box::new(remove_unused_lets(*d))),
        },
        other => other,
    }
}
/// Hoist let bindings out of case branches when the same binding
/// appears in all branches.
pub fn hoist_lets(expr: LcnfExpr) -> LcnfExpr {
    match expr {
        LcnfExpr::Let {
            id,
            name,
            ty,
            value,
            body,
        } => LcnfExpr::Let {
            id,
            name,
            ty,
            value,
            body: Box::new(hoist_lets(*body)),
        },
        LcnfExpr::Case {
            scrutinee,
            scrutinee_ty,
            alts,
            default,
        } => {
            let alts: Vec<LcnfAlt> = alts
                .into_iter()
                .map(|a| LcnfAlt {
                    ctor_name: a.ctor_name,
                    ctor_tag: a.ctor_tag,
                    params: a.params,
                    body: hoist_lets(a.body),
                })
                .collect();
            let default = default.map(|d| Box::new(hoist_lets(*d)));
            if alts.len() < 2 || default.is_some() {
                return LcnfExpr::Case {
                    scrutinee,
                    scrutinee_ty,
                    alts,
                    default,
                };
            }
            let first_let = match &alts[0].body {
                LcnfExpr::Let {
                    name, ty, value, ..
                } => Some((name.clone(), ty.clone(), value.clone())),
                _ => None,
            };
            if let Some((common_name, common_ty, common_value)) = first_let {
                let all_same = alts.iter().all(|a| {
                    matches!(
                        & a.body, LcnfExpr::Let { name, ty, value, .. } if * name ==
                        common_name && * ty == common_ty && * value == common_value
                    )
                });
                if all_same {
                    let hoisted_id = match &alts[0].body {
                        LcnfExpr::Let { id, .. } => *id,
                        _ => unreachable!(),
                    };
                    let new_alts: Vec<LcnfAlt> = alts
                        .into_iter()
                        .map(|a| {
                            let inner_body = match a.body {
                                LcnfExpr::Let { id, body, .. } => {
                                    if id != hoisted_id {
                                        let mut subst = Substitution::new();
                                        subst.insert(id, LcnfArg::Var(hoisted_id));
                                        substitute_expr(&body, &subst)
                                    } else {
                                        *body
                                    }
                                }
                                other => other,
                            };
                            LcnfAlt {
                                ctor_name: a.ctor_name,
                                ctor_tag: a.ctor_tag,
                                params: a.params,
                                body: inner_body,
                            }
                        })
                        .collect();
                    return LcnfExpr::Let {
                        id: hoisted_id,
                        name: common_name,
                        ty: common_ty,
                        value: common_value,
                        body: Box::new(LcnfExpr::Case {
                            scrutinee,
                            scrutinee_ty,
                            alts: new_alts,
                            default: None,
                        }),
                    };
                }
            }
            LcnfExpr::Case {
                scrutinee,
                scrutinee_ty,
                alts,
                default,
            }
        }
        other => other,
    }
}
