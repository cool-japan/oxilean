//! Kernel-side inductive family checking and recursor derivation.
//!
//! This module implements the Lean 4 kernel's `inductive.cpp` semantics for
//! declaring (mutual) inductive families:
//!
//! 1. **Signature checking** — every type in the family is a telescope of
//!    `num_params` shared parameters followed by indices, ending in a sort;
//!    all types of a mutual family must land in the *same* sort.
//! 2. **Constructor checking** — each constructor takes the family
//!    parameters first (definitionally equal binders), then fields; each
//!    field's universe must fit the inductive's universe (no constraint when
//!    the family lives in `Prop`); the codomain must be the parent type
//!    applied to exactly the parameters followed by index terms.
//! 3. **Strict positivity** — constructor argument types are reduced to WHNF
//!    (so negativity hidden behind definitions is found) and occurrences of
//!    any family member may only appear as the codomain of a field telescope
//!    applied to exactly the parameters. Occurrences nested under another
//!    type constructor (`mk : List T → T`) are *nested inductives*, which
//!    this kernel deliberately rejects with
//!    [`KernelError::UnsupportedNestedInductive`] instead of compiling them
//!    to mutual families.
//! 4. **Recursor derivation** — the recursors are derived *by the kernel*
//!    (never trusted from the outside): one recursor per family member,
//!    each carrying all `k` motives and all minor premises. Minor premises
//!    include induction hypotheses for recursive fields (Pi-wrapped for
//!    reflexive fields). The elimination universe is decided by the kernel:
//!    a family whose resultant sort is never `0` eliminates into any
//!    universe (a fresh level parameter is prepended); a possibly-`Prop`
//!    family eliminates only into `Prop` unless it is a subsingleton
//!    eliminator (at most one constructor whose every field is a proposition
//!    or appears among the resulting indices — e.g. `Eq`, `Acc`, `False`).
//!    The K flag is set exactly per Lean: single non-mutual `Prop` type with
//!    one constructor whose arguments are all parameters.
//!
//! Recursor rule right-hand sides use Lean's convention: each RHS is a
//! **closed lambda** over `params ++ motives ++ minors ++ fields`; iota
//! reduction applies it to the recursor's premises and the constructor's
//! fields and lets beta do the rest (see `reduce::iota`).

use crate::declaration::{
    ConstantInfo, ConstantVal, ConstructorVal, InductiveVal, RecursorRule, RecursorVal,
};
use crate::expr_util::{get_app_fn, get_app_fn_args, mk_app};
use crate::infer::LocalDecl;
use crate::instantiate::instantiate_type_lparams;
use crate::level::{is_equivalent, is_geq, normalize as normalize_level};
use crate::subst::instantiate;
use crate::Node;
use crate::{BinderInfo, Environment, Expr, FVarId, KernelError, Level, Name, TypeChecker};
use std::rc::Rc;

/// A single inductive type inside a (possibly mutual) family declaration.
#[derive(Clone, Debug)]
pub struct InductiveSpec {
    /// Name of the inductive type.
    pub name: Name,
    /// Full type: `Π params.. indices.., Sort l`.
    pub ty: Expr,
    /// Constructors as `(name, full type)` pairs. The constructor type must
    /// re-bind the family parameters first.
    pub ctors: Vec<(Name, Expr)>,
    /// Optional explicit recursor name (defaults to `<name>.rec`).
    pub rec_name: Option<Name>,
}

impl InductiveSpec {
    /// Create a spec with the default recursor name `<name>.rec`.
    pub fn new(name: Name, ty: Expr, ctors: Vec<(Name, Expr)>) -> Self {
        Self {
            name,
            ty,
            ctors,
            rec_name: None,
        }
    }
    /// Override the generated recursor's name.
    pub fn with_rec_name(mut self, rec_name: Name) -> Self {
        self.rec_name = Some(rec_name);
        self
    }
    fn effective_rec_name(&self) -> Name {
        self.rec_name
            .clone()
            .unwrap_or_else(|| Name::Str(Box::new(self.name.clone()), "rec".to_string()))
    }
}

/// The kernel-derived declarations for an inductive family.
#[derive(Clone, Debug)]
pub struct DerivedFamily {
    /// One `ConstantInfo::Inductive` per family member (declaration order).
    pub inductives: Vec<ConstantInfo>,
    /// All `ConstantInfo::Constructor`s (family order, then ctor order).
    pub constructors: Vec<ConstantInfo>,
    /// One `ConstantInfo::Recursor` per family member.
    pub recursors: Vec<ConstantInfo>,
}

impl DerivedFamily {
    /// All constant infos in dependency order (inductives, ctors, recursors).
    pub fn into_constant_infos(self) -> Vec<ConstantInfo> {
        let mut out = self.inductives;
        out.extend(self.constructors);
        out.extend(self.recursors);
        out
    }
}

// ---------------------------------------------------------------------------
// Derivation context: fresh free variables + optional type checker.
// ---------------------------------------------------------------------------

/// Base offset for derivation-local free variables so they can never collide
/// with the `TypeChecker`'s internal fresh variables (which count up from 0).
const DERIVE_FVAR_BASE: u64 = 1 << 40;

#[derive(Clone, Debug)]
struct Binder {
    fvar: FVarId,
    bi: BinderInfo,
    name: Name,
    ty: Expr,
}

fn fv(b: &Binder) -> Expr {
    Expr::FVar(b.fvar)
}

/// Checking/derivation context. In checked mode it owns a `TypeChecker`
/// (WHNF, def-eq, sort inference over the working environment); in
/// unchecked mode (legacy `InductiveType::to_constant_infos`) peeling is
/// purely syntactic and no typing judgements are made.
struct DeriveCtx<'e> {
    tc: Option<TypeChecker<'e>>,
    next: u64,
}

impl<'e> DeriveCtx<'e> {
    fn checked(env: &'e Environment) -> Self {
        Self {
            tc: Some(TypeChecker::new(env)),
            next: 0,
        }
    }
    fn unchecked() -> Self {
        Self { tc: None, next: 0 }
    }
    fn is_checked(&self) -> bool {
        self.tc.is_some()
    }
    fn fresh(&mut self, bi: BinderInfo, name: Name, ty: Expr) -> Binder {
        let id = FVarId(DERIVE_FVAR_BASE + self.next);
        self.next += 1;
        if let Some(tc) = self.tc.as_mut() {
            tc.push_local(LocalDecl {
                fvar: id,
                name: name.clone(),
                ty: ty.clone(),
                val: None,
            });
        }
        Binder {
            fvar: id,
            bi,
            name,
            ty,
        }
    }
    fn whnf(&mut self, e: &Expr) -> Expr {
        match self.tc.as_mut() {
            Some(tc) => tc.whnf(e),
            None => e.clone(),
        }
    }
    fn is_def_eq(&mut self, a: &Expr, b: &Expr) -> bool {
        match self.tc.as_mut() {
            Some(tc) => tc.is_def_eq(a, b),
            None => a == b,
        }
    }
    #[allow(clippy::result_large_err)]
    fn ensure_sort(&mut self, e: &Expr) -> Result<Option<Level>, KernelError> {
        match self.tc.as_mut() {
            Some(tc) => tc.ensure_sort(e).map(Some),
            None => Ok(None),
        }
    }
}

// ---------------------------------------------------------------------------
// FVar abstraction (telescope closing).
// ---------------------------------------------------------------------------

/// Replace `fvar` with the bound variable for the binder about to be
/// wrapped. All expressions built by this module are fvar-closed (they never
/// contain loose BVars), so — unlike `subst::abstract_expr` — existing bound
/// variables are never shifted.
fn abstract_fvar(expr: &Expr, fvar: FVarId, depth: u32) -> Expr {
    match expr {
        Expr::FVar(id) if *id == fvar => Expr::BVar(depth),
        Expr::BVar(_) | Expr::Sort(_) | Expr::FVar(_) | Expr::Const(_, _) | Expr::Lit(_) => {
            expr.clone()
        }
        Expr::App(f, a) => Expr::App(
            Node::new(abstract_fvar(f, fvar, depth)),
            Node::new(abstract_fvar(a, fvar, depth)),
        ),
        Expr::Lam(bi, n, ty, body) => Expr::Lam(
            *bi,
            n.clone(),
            Node::new(abstract_fvar(ty, fvar, depth)),
            Node::new(abstract_fvar(body, fvar, depth + 1)),
        ),
        Expr::Pi(bi, n, ty, body) => Expr::Pi(
            *bi,
            n.clone(),
            Node::new(abstract_fvar(ty, fvar, depth)),
            Node::new(abstract_fvar(body, fvar, depth + 1)),
        ),
        Expr::Let(n, ty, val, body) => Expr::Let(
            n.clone(),
            Node::new(abstract_fvar(ty, fvar, depth)),
            Node::new(abstract_fvar(val, fvar, depth)),
            Node::new(abstract_fvar(body, fvar, depth + 1)),
        ),
        Expr::Proj(n, i, e) => Expr::Proj(n.clone(), *i, Node::new(abstract_fvar(e, fvar, depth))),
    }
}

/// Close `body` over `binders` (outermost first) with Pi (`lam == false`)
/// or Lambda (`lam == true`) binders. `bi_override` forces a binder info on
/// every binder of the group (recursors make params/motives/indices
/// implicit regardless of how they were declared).
fn close(binders: &[Binder], body: &Expr, lam: bool, bi_override: Option<BinderInfo>) -> Expr {
    let mut r = body.clone();
    for b in binders.iter().rev() {
        let inner = abstract_fvar(&r, b.fvar, 0);
        let bi = bi_override.unwrap_or(b.bi);
        r = if lam {
            Expr::Lam(
                bi,
                b.name.clone(),
                Node::new(b.ty.clone()),
                Node::new(inner),
            )
        } else {
            Expr::Pi(
                bi,
                b.name.clone(),
                Node::new(b.ty.clone()),
                Node::new(inner),
            )
        };
    }
    r
}

// ---------------------------------------------------------------------------
// Small syntactic helpers.
// ---------------------------------------------------------------------------

fn has_const_occ(e: &Expr, names: &[Name]) -> bool {
    match e {
        Expr::Const(n, _) => names.contains(n),
        Expr::BVar(_) | Expr::FVar(_) | Expr::Sort(_) | Expr::Lit(_) => false,
        Expr::App(f, a) => has_const_occ(f, names) || has_const_occ(a, names),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            has_const_occ(ty, names) || has_const_occ(body, names)
        }
        Expr::Let(_, ty, val, body) => {
            has_const_occ(ty, names) || has_const_occ(val, names) || has_const_occ(body, names)
        }
        Expr::Proj(_, _, e) => has_const_occ(e, names),
    }
}

fn collect_level_param_names(l: &Level, out: &mut Vec<Name>) {
    match l {
        Level::Zero => {}
        Level::Succ(a) => collect_level_param_names(a, out),
        Level::Max(a, b) | Level::IMax(a, b) => {
            collect_level_param_names(a, out);
            collect_level_param_names(b, out);
        }
        Level::Param(n) => {
            if !out.contains(n) {
                out.push(n.clone());
            }
        }
        Level::MVar(_) => {}
    }
}

fn collect_expr_level_params(e: &Expr, out: &mut Vec<Name>) {
    match e {
        Expr::Sort(l) => collect_level_param_names(l, out),
        Expr::Const(_, ls) => {
            for l in ls {
                collect_level_param_names(l, out);
            }
        }
        Expr::BVar(_) | Expr::FVar(_) | Expr::Lit(_) => {}
        Expr::App(f, a) => {
            collect_expr_level_params(f, out);
            collect_expr_level_params(a, out);
        }
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            collect_expr_level_params(ty, out);
            collect_expr_level_params(body, out);
        }
        Expr::Let(_, ty, val, body) => {
            collect_expr_level_params(ty, out);
            collect_expr_level_params(val, out);
            collect_expr_level_params(body, out);
        }
        Expr::Proj(_, _, e) => collect_expr_level_params(e, out),
    }
}

#[allow(clippy::result_large_err)]
fn check_level_params_declared(
    e: &Expr,
    lparams: &[Name],
    what: &str,
    name: &Name,
) -> Result<(), KernelError> {
    let mut used = Vec::new();
    collect_expr_level_params(e, &mut used);
    for u in used {
        if !lparams.contains(&u) {
            return Err(KernelError::InvalidInductive(format!(
                "{} '{}' uses undeclared universe parameter '{}'",
                what, name, u
            )));
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Telescope peeling.
// ---------------------------------------------------------------------------

/// Peel every Pi binder from `ty` into fresh fvars (WHNF-assisted in checked
/// mode), returning the binders and the fvar-instantiated remainder.
fn peel_all(ctx: &mut DeriveCtx<'_>, ty: &Expr) -> (Vec<Binder>, Expr) {
    let mut out = Vec::new();
    let mut cur = ty.clone();
    loop {
        let curw = if matches!(cur, Expr::Pi(_, _, _, _)) {
            cur
        } else {
            let w = ctx.whnf(&cur);
            if !matches!(w, Expr::Pi(_, _, _, _)) {
                return (out, w);
            }
            w
        };
        if let Expr::Pi(bi, name, dom, body) = curw {
            let b = ctx.fresh(bi, name, (*dom).clone());
            cur = instantiate(&body, &fv(&b));
            out.push(b);
        } else {
            return (out, curw);
        }
    }
}

/// Peel `params.len()` binders from `ty`, instantiating each with the
/// corresponding shared parameter fvar. In checked mode the binder domain
/// must be definitionally equal to the parameter's type.
#[allow(clippy::result_large_err)]
fn peel_against_params(
    ctx: &mut DeriveCtx<'_>,
    ty: &Expr,
    params: &[Binder],
    what: &str,
    name: &Name,
) -> Result<Expr, KernelError> {
    let mut cur = ty.clone();
    for (i, p) in params.iter().enumerate() {
        let curw = if matches!(cur, Expr::Pi(_, _, _, _)) {
            cur
        } else {
            ctx.whnf(&cur)
        };
        match curw {
            Expr::Pi(_, _, dom, body) => {
                if ctx.is_checked() && !ctx.is_def_eq(&dom, &p.ty) {
                    return Err(KernelError::InvalidInductive(format!(
                        "{} '{}': parameter #{} does not match the family parameter type",
                        what,
                        name,
                        i + 1
                    )));
                }
                cur = instantiate(&body, &fv(p));
            }
            _ => {
                return Err(KernelError::InvalidInductive(format!(
                    "{} '{}' does not bind the {} family parameter(s)",
                    what,
                    name,
                    params.len()
                )));
            }
        }
    }
    Ok(cur)
}

// ---------------------------------------------------------------------------
// Family signature (pre-analysis of the inductive types themselves).
// ---------------------------------------------------------------------------

struct FamilyPre {
    lparams: Vec<Name>,
    ind_levels: Vec<Level>,
    np: usize,
    params: Vec<Binder>,
    names: Vec<Name>,
    /// Per-type index telescopes (fvar binders reused for motives and the
    /// outer recursor telescope; both fully close over them).
    indices: Vec<Vec<Binder>>,
    /// Resultant sort level (shared across the family).
    ind_level: Level,
}

#[allow(clippy::result_large_err)]
fn build_family_pre(
    ctx: &mut DeriveCtx<'_>,
    lparams: &[Name],
    num_params: u32,
    types: &[(Name, Expr)],
) -> Result<FamilyPre, KernelError> {
    if types.is_empty() {
        return Err(KernelError::InvalidInductive(
            "inductive family must contain at least one type".to_string(),
        ));
    }
    for (i, lp) in lparams.iter().enumerate() {
        if lparams[..i].contains(lp) {
            return Err(KernelError::InvalidInductive(format!(
                "duplicate universe parameter '{}'",
                lp
            )));
        }
    }
    for (i, (n, _)) in types.iter().enumerate() {
        if types[..i].iter().any(|(m, _)| m == n) {
            return Err(KernelError::InvalidInductive(format!(
                "duplicate inductive type name '{}'",
                n
            )));
        }
    }
    let np = num_params as usize;
    let ind_levels: Vec<Level> = lparams.iter().cloned().map(Level::param).collect();

    for (n, ty) in types {
        check_level_params_declared(ty, lparams, "inductive type", n)?;
        if ctx.is_checked() {
            ctx.ensure_sort(ty)?;
        }
    }

    // Shared parameter telescope from the first type.
    let mut params = Vec::new();
    let mut cur = types[0].1.clone();
    for i in 0..np {
        let curw = if matches!(cur, Expr::Pi(_, _, _, _)) {
            cur
        } else {
            ctx.whnf(&cur)
        };
        match curw {
            Expr::Pi(bi, name, dom, body) => {
                let b = ctx.fresh(bi, name, (*dom).clone());
                cur = instantiate(&body, &fv(&b));
                params.push(b);
            }
            _ => {
                return Err(KernelError::InvalidInductive(format!(
                    "inductive type '{}' has fewer than {} parameter binders (got {})",
                    types[0].0, np, i
                )));
            }
        }
    }

    let mut names = Vec::new();
    let mut indices = Vec::new();
    let mut ind_level: Option<Level> = None;
    for (j, (n, ty)) in types.iter().enumerate() {
        let rest = if j == 0 {
            cur.clone()
        } else {
            peel_against_params(ctx, ty, &params, "inductive type", n)?
        };
        let (idx_binders, cod) = peel_all(ctx, &rest);
        let codw = ctx.whnf(&cod);
        let level = match codw {
            Expr::Sort(l) => l,
            other => {
                return Err(KernelError::InvalidInductive(format!(
                    "inductive type '{}' must end in a sort, found '{}'",
                    n, other
                )));
            }
        };
        match &ind_level {
            None => ind_level = Some(level),
            Some(l0) => {
                if !is_equivalent(l0, &level) {
                    return Err(KernelError::InvalidInductive(format!(
                        "mutually inductive type '{}' lives in a different universe than '{}'",
                        n, types[0].0
                    )));
                }
            }
        }
        names.push(n.clone());
        indices.push(idx_binders);
    }
    // Normalize so `is_zero`/`is_not_zero` see through `imax`/`max` shapes.
    let ind_level = normalize_level(&ind_level.unwrap_or_else(Level::zero));
    Ok(FamilyPre {
        lparams: lparams.to_vec(),
        ind_levels,
        np,
        params,
        names,
        indices,
        ind_level,
    })
}

/// `T_j params index-args` with exactly the declared levels and parameters?
/// Returns the family position and the index argument terms.
fn as_valid_ind_app(pre: &FamilyPre, e: &Expr, strict: bool) -> Option<(usize, Vec<Expr>)> {
    let (head, args) = get_app_fn_args(e);
    let (n, levels) = match head {
        Expr::Const(n, levels) => (n, levels),
        _ => return None,
    };
    let j = pre.names.iter().position(|x| x == n)?;
    if strict {
        if levels.len() != pre.ind_levels.len()
            || !levels
                .iter()
                .zip(&pre.ind_levels)
                .all(|(a, b)| is_equivalent(a, b))
        {
            return None;
        }
        if args.len() != pre.np + pre.indices[j].len() {
            return None;
        }
        for (arg, p) in args.iter().zip(&pre.params) {
            if **arg != Expr::FVar(p.fvar) {
                return None;
            }
        }
    }
    let idx = args.iter().skip(pre.np).map(|e| (*e).clone()).collect();
    Some((j, idx))
}

// ---------------------------------------------------------------------------
// Constructor analysis (checking + field classification).
// ---------------------------------------------------------------------------

struct RecFieldSig {
    /// The field's own Pi telescope (non-empty for reflexive fields).
    ds: Vec<Binder>,
    /// Which family member the field recurses into.
    type_idx: usize,
    /// Index terms of the recursive occurrence (excludes the parameters).
    index_args: Vec<Expr>,
}

struct FieldSig {
    binder: Binder,
    /// Sort level of the field's type (checked mode only).
    sort_level: Option<Level>,
    rec: Option<RecFieldSig>,
}

struct CtorSig {
    name: Name,
    ty: Expr,
    fields: Vec<FieldSig>,
    /// Index arguments of the constructor's codomain (excludes parameters).
    ret_index_args: Vec<Expr>,
}

/// Lean's `check_positivity`: WHNF, then any occurrence of a family member
/// must be a strictly positive one — never inside a Pi domain, and at the
/// spine only as a valid `T params indices` application. Occurrences nested
/// under another constant's arguments are classified as nested inductives.
#[allow(clippy::result_large_err)]
fn check_positivity(
    ctx: &mut DeriveCtx<'_>,
    pre: &FamilyPre,
    ty: &Expr,
    ctor_name: &Name,
    decl_name: &Name,
) -> Result<(), KernelError> {
    let t = ctx.whnf(ty);
    if !has_const_occ(&t, &pre.names) {
        return Ok(());
    }
    match t {
        Expr::Pi(bi, name, dom, body) => {
            if has_const_occ(&dom, &pre.names) {
                return Err(KernelError::InvalidInductive(format!(
                    "non-strictly-positive occurrence of '{}' in constructor '{}'",
                    decl_name, ctor_name
                )));
            }
            let b = ctx.fresh(bi, name, (*dom).clone());
            check_positivity(ctx, pre, &instantiate(&body, &fv(&b)), ctor_name, decl_name)
        }
        _ => {
            if as_valid_ind_app(pre, &t, true).is_some() {
                return Ok(());
            }
            match get_app_fn(&t) {
                Expr::Const(c, _) if !pre.names.contains(c) => {
                    Err(KernelError::UnsupportedNestedInductive(decl_name.clone()))
                }
                _ => Err(KernelError::InvalidInductive(format!(
                    "invalid occurrence of '{}' in constructor '{}' (recursive occurrences \
                     must be applied to exactly the family parameters)",
                    decl_name, ctor_name
                ))),
            }
        }
    }
}

/// Classify a field as recursive: peel the field's own Pi telescope and
/// test whether the codomain is a valid application of a family member.
fn classify_rec_field(
    ctx: &mut DeriveCtx<'_>,
    pre: &FamilyPre,
    field_ty: &Expr,
) -> Option<RecFieldSig> {
    if !has_const_occ(field_ty, &pre.names) {
        return None;
    }
    let (ds, cod) = peel_all(ctx, field_ty);
    let codw = ctx.whnf(&cod);
    let strict = ctx.is_checked();
    let (type_idx, index_args) = as_valid_ind_app(pre, &codw, strict)?;
    Some(RecFieldSig {
        ds,
        type_idx,
        index_args,
    })
}

#[allow(clippy::result_large_err)]
fn check_ctor(
    ctx: &mut DeriveCtx<'_>,
    pre: &FamilyPre,
    type_idx: usize,
    cname: &Name,
    cty: &Expr,
) -> Result<CtorSig, KernelError> {
    check_level_params_declared(cty, &pre.lparams, "constructor", cname)?;
    if ctx.is_checked() {
        // Fully typecheck the constructor type (it must be a type).
        ctx.ensure_sort(cty)?;
    }
    let decl_name = &pre.names[type_idx];
    let rest = peel_against_params(ctx, cty, &pre.params, "constructor", cname)?;
    let (field_binders, cod) = peel_all(ctx, &rest);

    let mut fields = Vec::with_capacity(field_binders.len());
    for b in field_binders {
        // Normalized so a Prop-valued Pi field (`imax(u, 0)`) is recognised
        // as `0` by the universe rule and the elimination decision.
        let sort_level = ctx.ensure_sort(&b.ty)?.map(|l| normalize_level(&l));
        if let Some(sl) = &sort_level {
            // Universe rule: unless the family lives in Prop, every field
            // must fit inside the inductive's universe.
            if !pre.ind_level.is_zero() && !is_geq(&pre.ind_level, sl) {
                return Err(KernelError::InvalidInductive(format!(
                    "universe level of field '{}' in constructor '{}' is too big \
                     for inductive type '{}'",
                    b.name, cname, decl_name
                )));
            }
        }
        if ctx.is_checked() {
            check_positivity(ctx, pre, &b.ty, cname, decl_name)?;
        }
        let rec = classify_rec_field(ctx, pre, &b.ty);
        fields.push(FieldSig {
            binder: b,
            sort_level,
            rec,
        });
    }

    let codw = ctx.whnf(&cod);
    let ret_index_args = if ctx.is_checked() {
        match as_valid_ind_app(pre, &codw, true) {
            Some((j, idx)) if j == type_idx => idx,
            _ => {
                return Err(KernelError::InvalidInductive(format!(
                    "constructor '{}' must return '{}' applied to exactly the family \
                     parameters followed by index terms",
                    cname, decl_name
                )));
            }
        }
    } else {
        match as_valid_ind_app(pre, &codw, false) {
            Some((j, idx)) if j == type_idx => idx,
            _ => {
                return Err(KernelError::InvalidInductive(format!(
                    "constructor '{}' does not return inductive type '{}'",
                    cname, decl_name
                )));
            }
        }
    };

    Ok(CtorSig {
        name: cname.clone(),
        ty: cty.clone(),
        fields,
        ret_index_args,
    })
}

// ---------------------------------------------------------------------------
// Elimination universe / K flag.
// ---------------------------------------------------------------------------

/// Lean's `elim_only_at_universe_zero`, inverted: does the family support
/// large elimination? The decision is made by the kernel from the checked
/// signature — the caller-supplied `is_prop` flag is never consulted.
fn is_large_eliminating(pre: &FamilyPre, ctors: &[Vec<CtorSig>]) -> bool {
    if pre.ind_level.is_not_zero() {
        // The resultant sort is never 0: not a proposition.
        return true;
    }
    if pre.names.len() > 1 {
        return false;
    }
    let cs = &ctors[0];
    match cs.len() {
        0 => true, // e.g. False / Empty: eliminates anywhere.
        1 => {
            // Subsingleton eliminator: every field either lives in Prop or
            // appears among the resulting type's indices.
            let c = &cs[0];
            c.fields.iter().all(|f| {
                let field_is_prop = matches!(&f.sort_level, Some(l) if l.is_zero());
                field_is_prop || c.ret_index_args.contains(&Expr::FVar(f.binder.fvar))
            })
        }
        _ => false,
    }
}

/// Lean's `init_K_target`: K-like reduction is enabled only for a single
/// (non-mutual) Prop inductive with exactly one constructor whose arguments
/// are all parameters.
fn compute_k_target(pre: &FamilyPre, ctors: &[Vec<CtorSig>]) -> bool {
    pre.names.len() == 1
        && pre.ind_level.is_zero()
        && ctors[0].len() == 1
        && ctors[0][0].fields.is_empty()
}

/// Fresh elimination-universe parameter name: `u`, `u_1`, `u_2`, ...
fn mk_elim_level_name(lparams: &[Name]) -> Name {
    let mut cand = Name::str("u");
    let mut i = 1u32;
    while lparams.contains(&cand) {
        cand = Name::str(format!("u_{}", i));
        i += 1;
    }
    cand
}

// ---------------------------------------------------------------------------
// Recursor derivation.
// ---------------------------------------------------------------------------

fn last_component_name(n: &Name, fallback: &str) -> Name {
    Name::str(n.last_str().unwrap_or(fallback).to_string())
}

#[allow(clippy::result_large_err)]
fn derive_family_core(
    ctx: &mut DeriveCtx<'_>,
    pre: &FamilyPre,
    specs: &[InductiveSpec],
    ctors: Vec<Vec<CtorSig>>,
) -> Result<DerivedFamily, KernelError> {
    let k = pre.names.len();
    let total_ctors: usize = ctors.iter().map(|c| c.len()).sum();
    let rec_names: Vec<Name> = specs.iter().map(|s| s.effective_rec_name()).collect();

    let large_elim = is_large_eliminating(pre, &ctors);
    let k_target = compute_k_target(pre, &ctors);
    let (rec_lparams, elim_level) = if large_elim {
        let fresh = mk_elim_level_name(&pre.lparams);
        let mut lp = Vec::with_capacity(pre.lparams.len() + 1);
        lp.push(fresh.clone());
        lp.extend(pre.lparams.iter().cloned());
        (lp, Level::param(fresh))
    } else {
        (pre.lparams.clone(), Level::zero())
    };
    let rec_levels: Vec<Level> = rec_lparams.iter().cloned().map(Level::param).collect();
    let param_exprs: Vec<Expr> = pre.params.iter().map(fv).collect();

    // --- Motives (one per family member) -----------------------------------
    let mut motives: Vec<Binder> = Vec::with_capacity(k);
    for j in 0..k {
        let mut t_args = param_exprs.clone();
        t_args.extend(pre.indices[j].iter().map(fv));
        let t_ty = mk_app(
            Expr::Const(pre.names[j].clone(), pre.ind_levels.clone()),
            &t_args,
        );
        let x = ctx.fresh(BinderInfo::Default, Name::str("t"), t_ty);
        let mut dom = pre.indices[j].clone();
        dom.push(x);
        let motive_ty = close(
            &dom,
            &Expr::Sort(elim_level.clone()),
            false,
            Some(BinderInfo::Default),
        );
        let mname = if k == 1 {
            Name::str("motive")
        } else {
            Name::str(format!("motive_{}", j + 1))
        };
        motives.push(ctx.fresh(BinderInfo::Implicit, mname, motive_ty));
    }
    let motive_exprs: Vec<Expr> = motives.iter().map(fv).collect();

    // --- Minor premises (all constructors, family order) -------------------
    // For each ctor we also remember the per-field IH values used in the
    // reduction rule (grouped after the fields, mirroring the minor's type).
    struct MinorPlan {
        type_idx: usize,
        ctor_idx: usize,
        minor: Binder,
    }
    let mut minors: Vec<MinorPlan> = Vec::with_capacity(total_ctors);
    for (j, cs) in ctors.iter().enumerate() {
        for (ci, c) in cs.iter().enumerate() {
            let mut ih_binders: Vec<Binder> = Vec::new();
            for f in &c.fields {
                if let Some(rf) = &f.rec {
                    let ds_exprs: Vec<Expr> = rf.ds.iter().map(fv).collect();
                    let applied_field = mk_app(fv(&f.binder), &ds_exprs);
                    let mut m_args = rf.index_args.clone();
                    m_args.push(applied_field);
                    let ih_body = mk_app(fv(&motives[rf.type_idx]), &m_args);
                    let ih_ty = close(&rf.ds, &ih_body, false, None);
                    let ih_name =
                        Name::str(format!("{}_ih", f.binder.name.last_str().unwrap_or("a")));
                    ih_binders.push(ctx.fresh(BinderInfo::Default, ih_name, ih_ty));
                }
            }
            let mut ctor_args = param_exprs.clone();
            ctor_args.extend(c.fields.iter().map(|f| fv(&f.binder)));
            let ctor_app = mk_app(
                Expr::Const(c.name.clone(), pre.ind_levels.clone()),
                &ctor_args,
            );
            let mut concl_args = c.ret_index_args.clone();
            concl_args.push(ctor_app);
            let concl = mk_app(fv(&motives[j]), &concl_args);
            let mut tele: Vec<Binder> = c.fields.iter().map(|f| f.binder.clone()).collect();
            tele.extend(ih_binders);
            let minor_ty = close(&tele, &concl, false, None);
            let minor = ctx.fresh(
                BinderInfo::Default,
                last_component_name(&c.name, "minor"),
                minor_ty,
            );
            minors.push(MinorPlan {
                type_idx: j,
                ctor_idx: ci,
                minor,
            });
        }
    }
    let minor_binders: Vec<Binder> = minors.iter().map(|m| m.minor.clone()).collect();
    let minor_exprs: Vec<Expr> = minor_binders.iter().map(fv).collect();

    // --- Recursor per family member -----------------------------------------
    let mut recursors = Vec::with_capacity(k);
    for j in 0..k {
        // Type: Π {params} {motives} (minors) {indices} (t : T_j params idx), motive_j idx t
        let mut t_args = param_exprs.clone();
        t_args.extend(pre.indices[j].iter().map(fv));
        let major_ty = mk_app(
            Expr::Const(pre.names[j].clone(), pre.ind_levels.clone()),
            &t_args,
        );
        let major = ctx.fresh(BinderInfo::Default, Name::str("t"), major_ty);
        let mut concl_args: Vec<Expr> = pre.indices[j].iter().map(fv).collect();
        concl_args.push(fv(&major));
        let concl = mk_app(fv(&motives[j]), &concl_args);

        let mut rec_ty = close(std::slice::from_ref(&major), &concl, false, None);
        rec_ty = close(&pre.indices[j], &rec_ty, false, Some(BinderInfo::Implicit));
        rec_ty = close(&minor_binders, &rec_ty, false, Some(BinderInfo::Default));
        rec_ty = close(&motives, &rec_ty, false, Some(BinderInfo::Implicit));
        rec_ty = close(&pre.params, &rec_ty, false, Some(BinderInfo::Implicit));

        // Rules: one per own constructor; RHS is a closed lambda over
        // params ++ motives ++ minors ++ fields (Lean's convention).
        let mut rules = Vec::with_capacity(ctors[j].len());
        for plan in minors.iter().filter(|m| m.type_idx == j) {
            let c = &ctors[j][plan.ctor_idx];
            let mut body_args: Vec<Expr> = c.fields.iter().map(|f| fv(&f.binder)).collect();
            for f in &c.fields {
                if let Some(rf) = &f.rec {
                    let ds_exprs: Vec<Expr> = rf.ds.iter().map(fv).collect();
                    let mut rec_args = param_exprs.clone();
                    rec_args.extend(motive_exprs.iter().cloned());
                    rec_args.extend(minor_exprs.iter().cloned());
                    rec_args.extend(rf.index_args.iter().cloned());
                    rec_args.push(mk_app(fv(&f.binder), &ds_exprs));
                    let rec_call = mk_app(
                        Expr::Const(rec_names[rf.type_idx].clone(), rec_levels.clone()),
                        &rec_args,
                    );
                    body_args.push(close(&rf.ds, &rec_call, true, Some(BinderInfo::Default)));
                }
            }
            let body = mk_app(fv(&plan.minor), &body_args);
            let field_binders: Vec<Binder> = c.fields.iter().map(|f| f.binder.clone()).collect();
            let mut rhs = close(&field_binders, &body, true, Some(BinderInfo::Default));
            rhs = close(&minor_binders, &rhs, true, Some(BinderInfo::Default));
            rhs = close(&motives, &rhs, true, Some(BinderInfo::Default));
            rhs = close(&pre.params, &rhs, true, Some(BinderInfo::Default));
            rules.push(RecursorRule {
                ctor: c.name.clone(),
                nfields: c.fields.len() as u32,
                rhs,
            });
        }

        recursors.push(ConstantInfo::Recursor(RecursorVal {
            common: ConstantVal {
                name: rec_names[j].clone(),
                level_params: rec_lparams.clone(),
                ty: rec_ty,
            },
            all: pre.names.clone(),
            num_params: pre.np as u32,
            num_indices: pre.indices[j].len() as u32,
            num_motives: k as u32,
            num_minors: total_ctors as u32,
            rules,
            k: k_target,
            is_unsafe: false,
        }));
    }

    // --- Inductive + constructor infos --------------------------------------
    let is_rec_family = ctors
        .iter()
        .flatten()
        .any(|c| c.fields.iter().any(|f| f.rec.is_some()));
    let is_reflexive = ctors.iter().flatten().any(|c| {
        c.fields
            .iter()
            .any(|f| matches!(&f.rec, Some(rf) if !rf.ds.is_empty()))
    });
    let is_prop = pre.ind_level.is_zero();

    let mut inductives = Vec::with_capacity(k);
    let mut constructors = Vec::with_capacity(total_ctors);
    for (j, spec) in specs.iter().enumerate() {
        inductives.push(ConstantInfo::Inductive(InductiveVal {
            common: ConstantVal {
                name: pre.names[j].clone(),
                level_params: pre.lparams.clone(),
                ty: spec.ty.clone(),
            },
            num_params: pre.np as u32,
            num_indices: pre.indices[j].len() as u32,
            all: pre.names.clone(),
            ctors: ctors[j].iter().map(|c| c.name.clone()).collect(),
            num_nested: 0,
            is_rec: is_rec_family,
            is_unsafe: false,
            is_reflexive,
            is_prop,
        }));
        for (ci, c) in ctors[j].iter().enumerate() {
            constructors.push(ConstantInfo::Constructor(ConstructorVal {
                common: ConstantVal {
                    name: c.name.clone(),
                    level_params: pre.lparams.clone(),
                    ty: c.ty.clone(),
                },
                induct: pre.names[j].clone(),
                cidx: ci as u32,
                num_params: pre.np as u32,
                num_fields: c.fields.len() as u32,
                is_unsafe: false,
            }));
        }
    }

    Ok(DerivedFamily {
        inductives,
        constructors,
        recursors,
    })
}

// ---------------------------------------------------------------------------
// Public entry points.
// ---------------------------------------------------------------------------

#[allow(clippy::result_large_err)]
fn check_distinct_ctor_names(specs: &[InductiveSpec]) -> Result<(), KernelError> {
    let mut seen: Vec<&Name> = Vec::new();
    for spec in specs {
        for (cname, _) in &spec.ctors {
            if seen.contains(&cname) {
                return Err(KernelError::InvalidInductive(format!(
                    "duplicate constructor name '{}'",
                    cname
                )));
            }
            seen.push(cname);
        }
    }
    Ok(())
}

/// Check an inductive family against `env` and derive its constructors and
/// recursors, without modifying `env`. This is the kernel's independent
/// re-derivation path: signature checking, constructor telescope/universe
/// checking, WHNF-hardened strict positivity, and Lean-exact recursor
/// generation (elimination universe and K flag computed by the kernel).
///
/// The family types themselves may already exist in `env` (they must then
/// be identical `Inductive` declarations); otherwise they are staged into a
/// private working copy so that constructor types can reference them.
#[allow(clippy::result_large_err)]
pub fn check_and_derive_family(
    env: &Environment,
    lparams: &[Name],
    num_params: u32,
    specs: &[InductiveSpec],
) -> Result<DerivedFamily, KernelError> {
    check_distinct_ctor_names(specs)?;
    let family_names: Vec<Name> = specs.iter().map(|s| s.name.clone()).collect();

    // Stage the type constants so constructor types can mention them.
    let mut work = env.clone();
    for spec in specs {
        match work.find(&spec.name) {
            Some(ConstantInfo::Inductive(iv)) => {
                if iv.common.ty != spec.ty || iv.common.level_params != lparams {
                    return Err(KernelError::InvalidInductive(format!(
                        "inductive type '{}' is already declared with a different signature",
                        spec.name
                    )));
                }
            }
            Some(_) => {
                return Err(KernelError::InvalidInductive(format!(
                    "'{}' is already declared and is not an inductive type",
                    spec.name
                )));
            }
            None => {
                let stub = ConstantInfo::Inductive(InductiveVal {
                    common: ConstantVal {
                        name: spec.name.clone(),
                        level_params: lparams.to_vec(),
                        ty: spec.ty.clone(),
                    },
                    num_params,
                    num_indices: 0,
                    all: family_names.clone(),
                    ctors: spec.ctors.iter().map(|(n, _)| n.clone()).collect(),
                    num_nested: 0,
                    is_rec: false,
                    is_unsafe: false,
                    is_reflexive: false,
                    is_prop: false,
                });
                work.add_constant(stub)
                    .map_err(|e| KernelError::Other(e.to_string()))?;
            }
        }
    }

    let types: Vec<(Name, Expr)> = specs
        .iter()
        .map(|s| (s.name.clone(), s.ty.clone()))
        .collect();
    let mut ctx = DeriveCtx::checked(&work);
    let pre = build_family_pre(&mut ctx, lparams, num_params, &types)?;
    let mut ctor_sigs = Vec::with_capacity(specs.len());
    for (j, spec) in specs.iter().enumerate() {
        let mut cs = Vec::with_capacity(spec.ctors.len());
        for (cname, cty) in &spec.ctors {
            cs.push(check_ctor(&mut ctx, &pre, j, cname, cty)?);
        }
        ctor_sigs.push(cs);
    }
    derive_family_core(&mut ctx, &pre, specs, ctor_sigs)
}

/// Check an inductive family and add its types, constructors, and
/// kernel-derived recursors to `env`. This is the entry point an export
/// replayer should use for `inductive` blocks (mutual families included).
#[allow(clippy::result_large_err)]
pub fn add_inductive_family(
    env: &mut Environment,
    lparams: Vec<Name>,
    num_params: u32,
    specs: Vec<InductiveSpec>,
) -> Result<(), KernelError> {
    let fam = check_and_derive_family(env, &lparams, num_params, &specs)?;
    for ci in fam.into_constant_infos() {
        env.add_constant(ci)
            .map_err(|e| KernelError::Other(e.to_string()))?;
    }
    Ok(())
}

/// Purely syntactic derivation without an environment (legacy
/// `InductiveType::to_constant_infos` path). No positivity, universe, or
/// def-eq checks are performed, and — lacking sort inference — a
/// possibly-Prop single-constructor type is granted large elimination only
/// when every field appears among the resulting indices (this
/// under-approximates Lean's rule towards *small* elimination, which is the
/// sound direction). Use [`check_and_derive_family`] whenever an
/// environment is available.
#[allow(clippy::result_large_err)]
pub fn derive_family_unchecked(
    lparams: &[Name],
    num_params: u32,
    specs: &[InductiveSpec],
) -> Result<DerivedFamily, KernelError> {
    check_distinct_ctor_names(specs)?;
    let types: Vec<(Name, Expr)> = specs
        .iter()
        .map(|s| (s.name.clone(), s.ty.clone()))
        .collect();
    let mut ctx = DeriveCtx::unchecked();
    let pre = build_family_pre(&mut ctx, lparams, num_params, &types)?;
    let mut ctor_sigs = Vec::with_capacity(specs.len());
    for (j, spec) in specs.iter().enumerate() {
        let mut cs = Vec::with_capacity(spec.ctors.len());
        for (cname, cty) in &spec.ctors {
            cs.push(check_ctor(&mut ctx, &pre, j, cname, cty)?);
        }
        ctor_sigs.push(cs);
    }
    derive_family_core(&mut ctx, &pre, specs, ctor_sigs)
}

// ---------------------------------------------------------------------------
// Verification of externally-supplied declarations (export replay path).
// ---------------------------------------------------------------------------

/// Validate an externally-supplied `InductiveVal` (declaration-level checks;
/// constructor and recursor consistency is enforced when those arrive).
/// Crucially, the `is_prop` flag must agree with the *declared type* — a
/// caller cannot lie its way into large elimination (the recursor check
/// recomputes the elimination universe from the type anyway).
#[allow(clippy::result_large_err)]
pub(crate) fn validate_inductive_val(
    env: &Environment,
    iv: &InductiveVal,
) -> Result<(), KernelError> {
    for (i, lp) in iv.common.level_params.iter().enumerate() {
        if iv.common.level_params[..i].contains(lp) {
            return Err(KernelError::InvalidInductive(format!(
                "duplicate universe parameter '{}' in inductive '{}'",
                lp, iv.common.name
            )));
        }
    }
    check_level_params_declared(
        &iv.common.ty,
        &iv.common.level_params,
        "inductive type",
        &iv.common.name,
    )?;
    let mut ctx = DeriveCtx::checked(env);
    ctx.ensure_sort(&iv.common.ty)?;
    let (binders, rest) = peel_all(&mut ctx, &iv.common.ty);
    let restw = ctx.whnf(&rest);
    let level = match restw {
        Expr::Sort(l) => l,
        other => {
            return Err(KernelError::InvalidInductive(format!(
                "inductive type '{}' must end in a sort, found '{}'",
                iv.common.name, other
            )));
        }
    };
    if binders.len() != (iv.num_params + iv.num_indices) as usize {
        return Err(KernelError::InvalidInductive(format!(
            "inductive '{}' declares {} params + {} indices but its type has {} binders",
            iv.common.name,
            iv.num_params,
            iv.num_indices,
            binders.len()
        )));
    }
    if iv.is_prop != level.is_zero() {
        return Err(KernelError::InvalidInductive(format!(
            "inductive '{}': is_prop flag ({}) is inconsistent with the declared sort",
            iv.common.name, iv.is_prop
        )));
    }
    if !iv.all.contains(&iv.common.name) {
        return Err(KernelError::InvalidInductive(format!(
            "inductive '{}' is missing from its own mutual group",
            iv.common.name
        )));
    }
    for (i, c) in iv.ctors.iter().enumerate() {
        if iv.ctors[..i].contains(c) {
            return Err(KernelError::InvalidInductive(format!(
                "duplicate constructor '{}' in inductive '{}'",
                c, iv.common.name
            )));
        }
    }
    Ok(())
}

/// Build the family pre-analysis for the mutual group of `iv` from the
/// environment (all sibling `InductiveVal`s must already be present).
#[allow(clippy::result_large_err)]
fn family_pre_from_env<'e>(
    env: &'e Environment,
    all: &[Name],
    err_kind: fn(String) -> KernelError,
) -> Result<(DeriveCtx<'e>, FamilyPre), KernelError> {
    let mut lparams: Option<Vec<Name>> = None;
    let mut np: Option<u32> = None;
    let mut types = Vec::with_capacity(all.len());
    for tname in all {
        let siv = env.get_inductive_val(tname).ok_or_else(|| {
            err_kind(format!(
                "mutual inductive sibling '{}' is not declared in the environment",
                tname
            ))
        })?;
        if siv.all != all {
            return Err(err_kind(format!(
                "inductive '{}' declares a different mutual group",
                tname
            )));
        }
        match (&lparams, &np) {
            (None, None) => {
                lparams = Some(siv.common.level_params.clone());
                np = Some(siv.num_params);
            }
            (Some(lp), Some(n)) if *lp != siv.common.level_params || *n != siv.num_params => {
                return Err(err_kind(format!(
                    "mutual inductive '{}' disagrees on level params or param count",
                    tname
                )));
            }
            _ => {}
        }
        types.push((tname.clone(), siv.common.ty.clone()));
    }
    let lparams = lparams.unwrap_or_default();
    let np = np.unwrap_or(0);
    let mut ctx = DeriveCtx::checked(env);
    let pre = build_family_pre(&mut ctx, &lparams, np, &types)?;
    // Cross-check the stored index counts against the recomputed telescopes.
    for (j, tname) in all.iter().enumerate() {
        if let Some(siv) = env.get_inductive_val(tname) {
            if siv.num_indices as usize != pre.indices[j].len() {
                return Err(err_kind(format!(
                    "inductive '{}' declares {} indices but its type has {}",
                    tname,
                    siv.num_indices,
                    pre.indices[j].len()
                )));
            }
        }
    }
    Ok((ctx, pre))
}

/// Verify an externally-supplied `ConstructorVal` against the environment's
/// inductive: parameter telescope (def-eq), field count, universe fit,
/// strict positivity, and a valid codomain. Metadata (cidx, num_params,
/// level params) must agree with the parent inductive.
#[allow(clippy::result_large_err)]
pub(crate) fn verify_constructor_val(
    env: &Environment,
    cv: &ConstructorVal,
) -> Result<(), KernelError> {
    let iv = env.get_inductive_val(&cv.induct).ok_or_else(|| {
        KernelError::InvalidInductive(format!(
            "constructor '{}' references unknown inductive '{}'",
            cv.common.name, cv.induct
        ))
    })?;
    if cv.common.level_params != iv.common.level_params {
        return Err(KernelError::InvalidInductive(format!(
            "constructor '{}' must use the level parameters of '{}'",
            cv.common.name, cv.induct
        )));
    }
    if cv.num_params != iv.num_params {
        return Err(KernelError::InvalidInductive(format!(
            "constructor '{}': num_params ({}) disagrees with inductive '{}' ({})",
            cv.common.name, cv.num_params, cv.induct, iv.num_params
        )));
    }
    match iv.ctors.iter().position(|c| c == &cv.common.name) {
        Some(pos) if pos as u32 == cv.cidx => {}
        Some(pos) => {
            return Err(KernelError::InvalidInductive(format!(
                "constructor '{}' has cidx {} but is at position {} in '{}'",
                cv.common.name, cv.cidx, pos, cv.induct
            )));
        }
        None => {
            return Err(KernelError::InvalidInductive(format!(
                "constructor '{}' is not listed by inductive '{}'",
                cv.common.name, cv.induct
            )));
        }
    }
    let all = iv.all.clone();
    let induct = cv.induct.clone();
    let (mut ctx, pre) = family_pre_from_env(env, &all, KernelError::InvalidInductive)?;
    let type_idx = pre.names.iter().position(|n| *n == induct).ok_or_else(|| {
        KernelError::InvalidInductive(format!(
            "inductive '{}' missing from its own mutual group",
            induct
        ))
    })?;
    let sig = check_ctor(&mut ctx, &pre, type_idx, &cv.common.name, &cv.common.ty)?;
    if sig.fields.len() != cv.num_fields as usize {
        return Err(KernelError::InvalidInductive(format!(
            "constructor '{}': num_fields ({}) disagrees with its type ({} fields)",
            cv.common.name,
            cv.num_fields,
            sig.fields.len()
        )));
    }
    Ok(())
}

/// Verify an externally-supplied `RecursorVal` by *re-deriving* the
/// recursors for its inductive family from the environment and requiring
/// the supplied declaration to match the derivation: identical structural
/// metadata (param/index/motive/minor counts, K flag) and definitionally
/// equal type and rule right-hand sides (up to level-parameter renaming).
/// This is the "re-derive, don't trust" hardening for export replay.
#[allow(clippy::result_large_err)]
pub(crate) fn verify_recursor_val(env: &Environment, rv: &RecursorVal) -> Result<(), KernelError> {
    if rv.all.is_empty() {
        return Err(KernelError::InvalidRecursor(format!(
            "recursor '{}' has an empty mutual group",
            rv.common.name
        )));
    }
    // Reconstruct the family specs from the environment.
    let mut lparams: Option<Vec<Name>> = None;
    let mut np: Option<u32> = None;
    let mut specs = Vec::with_capacity(rv.all.len());
    let single = rv.all.len() == 1;
    // Sibling recursor names inside rule RHSes must match structurally. For
    // a single-type family the only possible self-reference is the recursor
    // being verified, so use its own name; for mutual families follow the
    // supplied name's style (hierarchical `T.rec` vs flat `"T.rec"`).
    let flat_style = rv.common.name.depth() <= 1;
    for tname in &rv.all {
        let iv = env.get_inductive_val(tname).ok_or_else(|| {
            KernelError::InvalidRecursor(format!(
                "recursor '{}' references unknown inductive '{}'",
                rv.common.name, tname
            ))
        })?;
        if iv.all != rv.all {
            return Err(KernelError::InvalidRecursor(format!(
                "recursor '{}' and inductive '{}' disagree on the mutual group",
                rv.common.name, tname
            )));
        }
        match (&lparams, &np) {
            (None, None) => {
                lparams = Some(iv.common.level_params.clone());
                np = Some(iv.num_params);
            }
            (Some(lp), Some(n)) if *lp != iv.common.level_params || *n != iv.num_params => {
                return Err(KernelError::InvalidRecursor(format!(
                    "mutual inductive '{}' disagrees on level params or param count",
                    tname
                )));
            }
            _ => {}
        }
        let mut ctors = Vec::with_capacity(iv.ctors.len());
        for cname in &iv.ctors {
            let cval = env.get_constructor_val(cname).ok_or_else(|| {
                KernelError::InvalidRecursor(format!(
                    "recursor '{}': constructor '{}' of '{}' is not declared yet",
                    rv.common.name, cname, tname
                ))
            })?;
            ctors.push((cname.clone(), cval.common.ty.clone()));
        }
        let rec_name = if single {
            rv.common.name.clone()
        } else if flat_style {
            Name::str(format!("{}.rec", tname))
        } else {
            Name::Str(Box::new(tname.clone()), "rec".to_string())
        };
        specs.push(InductiveSpec {
            name: tname.clone(),
            ty: iv.common.ty.clone(),
            ctors,
            rec_name: Some(rec_name),
        });
    }
    let lparams = lparams.unwrap_or_default();
    let np = np.unwrap_or(0);
    let fam = check_and_derive_family(env, &lparams, np, &specs)?;

    let mut tc = TypeChecker::new(env);
    for d_ci in &fam.recursors {
        if let ConstantInfo::Recursor(d) = d_ci {
            if recursor_matches(&mut tc, d, rv) {
                return Ok(());
            }
        }
    }
    Err(KernelError::InvalidRecursor(format!(
        "recursor '{}' does not match the kernel-derived recursor for '{}'",
        rv.common.name, rv.all[0]
    )))
}

fn recursor_matches(tc: &mut TypeChecker<'_>, d: &RecursorVal, rv: &RecursorVal) -> bool {
    if d.common.level_params.len() != rv.common.level_params.len() {
        return false;
    }
    if d.num_params != rv.num_params
        || d.num_indices != rv.num_indices
        || d.num_motives != rv.num_motives
        || d.num_minors != rv.num_minors
        || d.k != rv.k
        || d.rules.len() != rv.rules.len()
    {
        return false;
    }
    // Align the derived declaration onto the supplied level parameter names.
    let levels: Vec<Level> = rv
        .common
        .level_params
        .iter()
        .cloned()
        .map(Level::param)
        .collect();
    let d_ty = instantiate_type_lparams(&d.common.ty, &d.common.level_params, &levels);
    if !tc.is_def_eq(&d_ty, &rv.common.ty) {
        return false;
    }
    for drule in &d.rules {
        let srule = match rv.get_rule(&drule.ctor) {
            Some(r) => r,
            None => return false,
        };
        if srule.nfields != drule.nfields {
            return false;
        }
        let d_rhs = instantiate_type_lparams(&drule.rhs, &d.common.level_params, &levels);
        if !tc.is_def_eq(&d_rhs, &srule.rhs) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn nat_spec() -> InductiveSpec {
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        InductiveSpec::new(
            Name::str("Nat"),
            Expr::Sort(Level::succ(Level::zero())),
            vec![
                (Name::str("Nat.zero"), nat.clone()),
                (
                    Name::str("Nat.succ"),
                    Expr::Pi(
                        BinderInfo::Default,
                        Name::str("n"),
                        Node::new(nat.clone()),
                        Node::new(nat),
                    ),
                ),
            ],
        )
        .with_rec_name(Name::str("Nat.rec"))
    }

    #[test]
    fn nat_derivation_shape() {
        let env = Environment::new();
        let fam = check_and_derive_family(&env, &[], 0, &[nat_spec()]).expect("derive Nat");
        assert_eq!(fam.inductives.len(), 1);
        assert_eq!(fam.constructors.len(), 2);
        assert_eq!(fam.recursors.len(), 1);
        let rv = match &fam.recursors[0] {
            ConstantInfo::Recursor(rv) => rv,
            other => panic!("expected recursor, got {:?}", other),
        };
        assert_eq!(rv.num_motives, 1);
        assert_eq!(rv.num_minors, 2);
        assert_eq!(rv.num_params, 0);
        assert_eq!(rv.num_indices, 0);
        assert!(!rv.k);
        // Large elimination: fresh level param "u" prepended.
        assert_eq!(rv.common.level_params, vec![Name::str("u")]);
        // succ minor has an IH: (n : Nat) → motive n → motive (Nat.succ n).
        let succ_rule = rv.get_rule(&Name::str("Nat.succ")).expect("succ rule");
        assert_eq!(succ_rule.nfields, 1);
    }

    #[test]
    fn unchecked_matches_checked_for_nat() {
        let env = Environment::new();
        let a = check_and_derive_family(&env, &[], 0, &[nat_spec()]).expect("checked");
        let b = derive_family_unchecked(&[], 0, &[nat_spec()]).expect("unchecked");
        assert_eq!(a.recursors, b.recursors);
        assert_eq!(a.constructors, b.constructors);
    }

    #[test]
    fn nested_inductive_rejected() {
        // inductive T | mk : List T → T  (nested occurrence under List)
        let mut env = Environment::new();
        crate::builtin::init_builtin_env(&mut env).expect("builtin env");
        let t = Expr::Const(Name::str("T"), vec![]);
        let list_t = Expr::App(
            Node::new(Expr::Const(Name::str("List"), vec![Level::zero()])),
            Node::new(t.clone()),
        );
        let spec = InductiveSpec::new(
            Name::str("T"),
            Expr::Sort(Level::succ(Level::zero())),
            vec![(
                Name::str("T.mk"),
                Expr::Pi(
                    BinderInfo::Default,
                    Name::str("l"),
                    Node::new(list_t),
                    Node::new(t),
                ),
            )],
        );
        let err = check_and_derive_family(&env, &[], 0, &[spec]).expect_err("must reject");
        assert!(matches!(err, KernelError::UnsupportedNestedInductive(_)));
    }
}
