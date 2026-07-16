//! Iota reduction helpers (recursor application), following the Lean 4
//! kernel's `type_checker.cpp` / `inductive_reduce_ext` semantics:
//!
//! * Recursor rule right-hand sides are **closed lambdas** over
//!   `params ++ motives ++ minors ++ fields`; reduction applies the RHS to
//!   the recursor's premises, the constructor's fields (the *last*
//!   `nfields` arguments of the major premise), and finally re-applies any
//!   arguments beyond the major premise — over-application never drops
//!   arguments.
//! * `Nat` literals expand one constructor layer at a time
//!   (`0 ↦ Nat.zero`, `n+1 ↦ Nat.succ (n : NatLit)`) when a `Nat` recursor
//!   is stuck on a literal major premise.
//! * `String` literals expand to `String.ofList (List.cons Char
//!   (Char.ofNat c₁) …)` and are then WHNF'd (Lean v4.32
//!   `string_lit_to_constructor` — `String`'s constructor is
//!   `ofByteArray` since the UTF-8 refactor, so the kernel goes through
//!   the `ofList` *function*); old-model environments with a one-field
//!   `String.mk : List Char → String` expand directly to the constructor.
//! * K-like reduction: when the recursor has the K flag, a major premise
//!   whose *type* is the inductive applied to the right parameters/indices
//!   reduces via the canonical (single, field-free) constructor even when
//!   the major is not syntactically a constructor application.

use crate::bignat::BigNat;
use crate::declaration::RecursorVal;
use crate::expr_util::{get_app_fn_args, mk_app, mk_app_refs};
use crate::instantiate::instantiate_type_lparams;
use crate::Node;
use crate::{Environment, Expr, Level, Literal, Name, TypeChecker};
use std::rc::Rc;

/// Expand a `Nat` literal into a one-layer constructor application so a
/// `Nat` recursor can consume it: `0 ↦ Nat.zero`,
/// `n ↦ Nat.succ (NatLit (n-1))` for `n > 0`. Constructor names are taken
/// from the recursor's own rules (zero-field rule / one-field rule), so
/// both flat (`"Nat.zero"`) and hierarchical (`Nat.zero`) naming styles
/// work. Only fires for a recursor that eliminates exactly `Nat`.
pub(super) fn nat_lit_to_ctor(n: &BigNat, rec_val: &RecursorVal) -> Option<Expr> {
    if rec_val.all.len() != 1 || rec_val.all[0].to_string() != "Nat" {
        return None;
    }
    if n.is_zero() {
        let zero_rule = rec_val.rules.iter().find(|r| r.nfields == 0)?;
        Some(Expr::Const(zero_rule.ctor.clone(), vec![]))
    } else {
        let succ_rule = rec_val.rules.iter().find(|r| r.nfields == 1)?;
        Some(Expr::App(
            Node::new(Expr::Const(succ_rule.ctor.clone(), vec![])),
            Node::new(Expr::Lit(Literal::Nat(n.pred()))),
        ))
    }
}

/// Build the `List Char` denotation of a string literal:
/// `List.cons Char (Char.ofNat <code₀>) (… (List.nil Char))`. Names are
/// spelled hierarchically (`List` ++ `cons`) or flat (`"List.cons"`)
/// according to `hierarchical`, so both the builtin environment and
/// lean4export replays resolve them.
fn char_list_of_str(s: &str, hierarchical: bool) -> Expr {
    let name_of = |dotted: &str| {
        if hierarchical {
            Name::from_str(dotted)
        } else {
            Name::str(dotted)
        }
    };
    let char_ty = Expr::Const(name_of("Char"), vec![]);
    let mut list = mk_app(
        Expr::Const(name_of("List.nil"), vec![Level::zero()]),
        std::slice::from_ref(&char_ty),
    );
    for c in s.chars().rev() {
        let code = Expr::Lit(Literal::Nat(BigNat::from(c as u32)));
        let ch = Expr::App(
            Node::new(Expr::Const(name_of("Char.ofNat"), vec![])),
            Node::new(code),
        );
        list = mk_app(
            Expr::Const(name_of("List.cons"), vec![Level::zero()]),
            &[char_ty.clone(), ch, list],
        );
    }
    list
}

/// Expand a `String` literal exactly as Lean's `string_lit_to_constructor`
/// (kernel/inductive.cpp, v4.32): the term `String.ofList (l : List Char)`
/// where `l` is the literal's characters. Since Lean ≥ 4.28 the `String`
/// constructor is `String.ofByteArray (bytes) (validity proof)` — the kernel
/// therefore expands to the *function* `String.ofList` and WHNFs the result
/// at every use site (iota major, projection struct, def-eq extension),
/// which unfolds `ofList l = ⟨List.utf8Encode l, .intro l rfl⟩` into
/// constructor form.
///
/// Environments in the OLD model (a single one-field constructor
/// `String.mk : List Char → String`, e.g. the builtin env) have no
/// `String.ofList`; for exactness *against that model* the literal expands
/// directly to the constructor application, as Lean's kernel did before the
/// UTF-8 `String` refactor.
///
/// Returns `None` when the environment declares neither form — the literal
/// stays stuck (safe incompleteness, never a wrong reduction).
pub(crate) fn str_lit_expansion(s: &str, env: &Environment) -> Option<Expr> {
    // v4.32 model: `String.ofList` (hierarchical spelling in lean4export
    // replays; flat spelling tolerated for synthetic envs).
    for (of_list, hierarchical) in [
        (Name::str("String").append_str("ofList"), true),
        (Name::str("String.ofList"), false),
    ] {
        if env.find(&of_list).is_some() {
            return Some(Expr::App(
                Node::new(Expr::Const(of_list, vec![])),
                Node::new(char_list_of_str(s, hierarchical)),
            ));
        }
    }
    // Old model: a single one-field constructor (`String.mk : List Char →
    // String`).
    if let Some(iv) = env.get_inductive_val(&Name::str("String")) {
        let ctor_name = iv.ctors.first()?;
        let cv = env.get_constructor_val(ctor_name)?;
        if iv.ctors.len() == 1 && cv.num_fields == 1 && cv.num_params == 0 {
            let hierarchical = ctor_name.depth() > 1;
            return Some(Expr::App(
                Node::new(Expr::Const(ctor_name.clone(), vec![])),
                Node::new(char_list_of_str(s, hierarchical)),
            ));
        }
    }
    None
}

/// Lean's `to_cnstr_when_K`: for a K-flagged recursor, if the major
/// premise's *type* reduces to `I As Is` for the recursor's inductive `I`,
/// build the canonical constructor application `I.mk As`, and use it as the
/// major premise when its type is definitionally equal to the major's type.
/// The major is typed in a checker seeded with `locals` — the free-variable
/// context mirrored from the surrounding checker (Lean's kernel uses its
/// local context here). Returns `None` (leaving the term stuck) whenever
/// the major cannot be typed — safe incompleteness, never a wrong
/// reduction.
pub(super) fn to_ctor_when_k(
    rec_val: &RecursorVal,
    major: &Expr,
    env: &Environment,
    locals: &std::collections::HashMap<u64, Expr>,
) -> Option<Expr> {
    let ind_name = rec_val.all.first()?;
    let mut tc = TypeChecker::with_fvar_types(env, locals);
    let app_ty = tc.infer_type(major).ok()?;
    let app_ty = tc.whnf(&app_ty);
    let (head, ty_args) = get_app_fn_args(&app_ty);
    let (head_name, levels) = match head {
        Expr::Const(n, ls) => (n, ls),
        _ => return None,
    };
    if head_name != ind_name {
        return None;
    }
    let iv = env.get_inductive_val(ind_name)?;
    if ty_args.len() != (iv.num_params + iv.num_indices) as usize {
        return None;
    }
    let ctor_name = iv.ctors.first()?;
    let params: Vec<Expr> = ty_args[..iv.num_params as usize]
        .iter()
        .map(|e| (*e).clone())
        .collect();
    let new_ctor = mk_app(Expr::Const(ctor_name.clone(), levels.clone()), &params);
    let new_ty = tc.infer_type(&new_ctor).ok()?;
    if !tc.is_def_eq(&app_ty, &new_ty) {
        return None;
    }
    Some(new_ctor)
}

/// Lean's `toCtorWhenStruct`: for a recursor over a **structure-like**
/// inductive (single constructor, no indices, not recursive), a major
/// premise that is stuck (not a constructor application) can be replaced by
/// the eta-expanded constructor form `I.mk params (e.0) (e.1) …` — structure
/// eta is definitional in Lean 4, so iota can then fire on majors that are
/// free variables or otherwise opaque terms (e.g. the inner `Prod.casesOn`
/// of a nested match like `| (k, v) :: p => …`).
///
/// Mirrors lean4lean's `toCtorWhenStruct` exactly, including the guards:
/// the major's type must whnf to `I params`, and a **`Prop`-sorted** type is
/// left alone (Lean's `== .sort .zero` check; proof-irrelevant majors are
/// handled by the K path, not eta). The major is typed in a checker seeded
/// with `locals` — the free-variable context mirrored from the surrounding
/// checker. Returns `None` (leaving the term stuck) whenever any guard
/// fails — safe incompleteness, never a wrong reduction.
pub(super) fn to_ctor_when_struct(
    rec_val: &RecursorVal,
    major: &Expr,
    env: &Environment,
    locals: &std::collections::HashMap<u64, Expr>,
) -> Option<Expr> {
    if rec_val.all.len() != 1 {
        return None;
    }
    let ind_name = rec_val.all.first()?;
    if !env.is_structure_like(ind_name) {
        return None;
    }
    // Already a constructor application: nothing to do (the caller's rule
    // application handles it directly).
    if let (Expr::Const(head_name, _), _) = get_app_fn_args(major) {
        if env.is_constructor(head_name) {
            return None;
        }
    }
    let mut tc = TypeChecker::with_fvar_types(env, locals);
    let e_ty = tc.infer_type(major).ok()?;
    let e_ty = tc.whnf(&e_ty);
    let (ty_head, ty_args) = get_app_fn_args(&e_ty);
    let (head_name, levels) = match ty_head {
        Expr::Const(n, ls) => (n, ls),
        _ => return None,
    };
    if head_name != ind_name {
        return None;
    }
    // Do not eta-expand proofs: `(← whnf (← inferType eType)) == .sort .zero`.
    let e_ty_sort = tc.infer_type(&e_ty).ok()?;
    let e_ty_sort = tc.whnf(&e_ty_sort);
    if matches!(&e_ty_sort, Expr::Sort(l) if l.is_zero()) {
        return None;
    }
    let iv = env.get_inductive_val(ind_name)?;
    // Structure-like has no indices, so the type's arguments are exactly the
    // parameters.
    if ty_args.len() != iv.num_params as usize {
        return None;
    }
    let ctor_name = iv.ctors.first()?;
    let cv = env.get_constructor_val(ctor_name)?;
    let params: Vec<Expr> = ty_args.iter().map(|e| (*e).clone()).collect();
    let mut result = mk_app(Expr::Const(ctor_name.clone(), levels.clone()), &params);
    for i in 0..cv.num_fields {
        result = Expr::App(
            Node::new(result),
            Node::new(Expr::Proj(ind_name.clone(), i, Node::new(major.clone()))),
        );
    }
    Some(result)
}

/// Apply a recursor rule to a constructor-headed major premise (Lean's
/// `inductive_reduce_ext` core). `rec_args` are all arguments of the
/// recursor application; `major` must be in WHNF (possibly rewritten by the
/// literal/K expansions). Returns the un-normalised reduct; the caller
/// re-runs WHNF (which performs the beta steps).
pub(super) fn apply_recursor_rule(
    rec_val: &RecursorVal,
    rec_levels: &[Level],
    rec_args: &[&Expr],
    major_idx: usize,
    major: &Expr,
    env: &Environment,
) -> Option<Expr> {
    let (ctor_fn, ctor_args) = get_app_fn_args(major);
    let ctor_name = match ctor_fn {
        Expr::Const(name, _) => name,
        _ => return None,
    };
    if !env.is_constructor(ctor_name) {
        return None;
    }
    let rule = rec_val.get_rule(ctor_name)?;
    let nfields = rule.nfields as usize;
    if ctor_args.len() < nfields {
        return None;
    }
    let rhs = if rec_val.common.level_params.is_empty() || rec_levels.is_empty() {
        rule.rhs.clone()
    } else {
        instantiate_type_lparams(&rule.rhs, &rec_val.common.level_params, rec_levels)
    };
    // params ++ motives ++ minors (the premises before the indices).
    let n_premises = (rec_val.num_params + rec_val.num_motives + rec_val.num_minors) as usize;
    if rec_args.len() < n_premises {
        return None;
    }
    let mut r = mk_app_refs(rhs, &rec_args[..n_premises]);
    // The constructor's fields are the *last* `nfields` arguments.
    r = mk_app_refs(r, &ctor_args[ctor_args.len() - nfields..]);
    // Never drop over-application: re-apply everything past the major.
    r = mk_app_refs(r, &rec_args[major_idx + 1..]);
    Some(r)
}
