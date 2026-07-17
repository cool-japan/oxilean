//! Nested-to-mutual inductive elaboration (C10).
//!
//! A *nested* inductive is one whose constructor fields mention the type being
//! defined under an already-declared foreign container, e.g.
//! `node : Array Syntax → Syntax` (`Syntax` occurs inside `Array`). Lean's
//! kernel compiles these by SPECIALIZING each container into an auxiliary
//! mutual inductive (the family type becomes a fresh leading parameter),
//! CHECKING that genuinely-mutual family with the ordinary machinery, then
//! RESTORING recursors stated over the *real* container so the exported
//! recursor re-verifies by def-eq.
//!
//! This module is built in stages (see
//! `docs/audit/2026-07-17-nested-inductives/design.md`):
//!
//! - **S1 (this file, now):** [`detect`] — recover the container `C`, its level
//!   arguments, its argument spine, and which argument positions mention a
//!   family-being-defined type. Behaviour of the kernel is unchanged; nested
//!   inductives are still rejected at the positivity leaf in `derive.rs`.
//! - **S2:** `specialize` — copy the container's mutual block into fresh
//!   auxiliaries, abstracting the family-occurring args as new leading params.
//! - **S3:** `restore` — repackage the derived recursors over the real
//!   container and drop the abstracted parameters.

use super::derive::{
    check_and_derive_family, has_const_occ, recursor_matches, DerivedFamily, InductiveSpec,
};
use crate::declaration::{ConstantInfo, InductiveVal, RecursorRule, RecursorVal};
use crate::expr_util::{get_app_fn_args, mk_app, replace_expr};
use crate::instantiate::instantiate_type_lparams;
use crate::subst::instantiate;
use crate::{Environment, Expr, KernelError, Level, Name, TypeChecker};

/// A nested occurrence `C Ds Is`: a family-being-defined type appears inside the
/// arguments of an already-declared foreign inductive `C`.
///
/// This is the raw *syntactic* capture produced at a strict-positivity leaf. It
/// does not yet decide whether `C` is a declared inductive, nor whether the
/// family occurs in a legal parameter position versus an (unsupported) index
/// position — that validation happens in S2 against `C`'s `InductiveVal`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NestedOcc {
    /// The foreign container inductive `C` (e.g. `List`, `Array`).
    pub container: Name,
    /// `C`'s universe-level arguments.
    pub container_levels: Vec<Level>,
    /// The full argument spine `Ds ++ Is` of the container application, in
    /// source order.
    pub args: Vec<Expr>,
    /// Indices into [`args`](NestedOcc::args) whose term mentions a
    /// family-being-defined type. In `Array Syntax` this is `[0]`; these are
    /// the positions S2 abstracts into fresh leading parameters (after checking
    /// they are parameter, not index, positions of `C`).
    pub family_positions: Vec<usize>,
}

/// Detect a nested occurrence at a strict-positivity leaf.
///
/// Given the WHNF of a constructor-field type whose spine is not a direct
/// application of a family member, return the nested occurrence when the spine
/// head is a *foreign* constant (not itself a family member) and at least one
/// argument mentions a family-being-defined type. Returns `None` for
/// - a non-`Const` head (nothing to un-nest here),
/// - a direct occurrence of a family member (`C ∈ family_names` — that is
///   ordinary recursion, handled elsewhere), or
/// - a foreign constant none of whose arguments mention the family (a plain
///   non-recursive field such as `List Nat`).
///
/// The check is purely syntactic and environment-free so it can be unit-tested
/// in isolation; declaredness and parameter-position validation are deferred to
/// S2.
pub fn detect(family_names: &[Name], ty: &Expr) -> Option<NestedOcc> {
    let (head, args) = get_app_fn_args(ty);
    let (container, container_levels) = match head {
        Expr::Const(n, levels) => (n.clone(), levels.clone()),
        _ => return None,
    };
    if family_names.contains(&container) {
        // A direct occurrence `T …` is ordinary (mutual) recursion, not nesting.
        return None;
    }
    let family_positions: Vec<usize> = args
        .iter()
        .copied()
        .enumerate()
        .filter(|(_, a)| has_const_occ(a, family_names))
        .map(|(i, _)| i)
        .collect();
    if family_positions.is_empty() {
        // `C` is present but carries no family type: an ordinary field.
        return None;
    }
    let args = args.into_iter().cloned().collect();
    Some(NestedOcc {
        container,
        container_levels,
        args,
        family_positions,
    })
}

// ---------------------------------------------------------------------------
// S2 — specialization (nested container → auxiliary mutual family).
// ---------------------------------------------------------------------------

/// How one auxiliary inductive maps back to the real container it specializes.
/// Retained through derivation so S3 restoration can substitute the auxiliary
/// type / constructors / recursor back to the real container constants and drop
/// the abstracted parameters.
#[derive(Clone, Debug)]
pub struct AuxMap {
    /// The fresh auxiliary inductive type name (e.g. `T._nested_0`).
    pub aux_type: Name,
    /// The real container type-former it specializes (e.g. `List`).
    pub container: Name,
    /// The container's universe-level arguments at the nested occurrence.
    pub container_levels: Vec<Level>,
    /// The concrete arguments `As` the container was specialized at (the
    /// abstracted parameters restoration must re-insert / drop).
    pub params: Vec<Expr>,
    /// `(auxiliary ctor name, real container ctor name)` in declaration order.
    pub aux_ctors: Vec<(Name, Name)>,
    /// The auxiliary recursor name the kernel will derive (`<aux_type>.rec`).
    pub aux_rec: Name,
    /// The real container's recursor name (`<container>.rec`).
    pub container_rec: Name,
}

/// The nesting-free mutual family produced by [`specialize`]: the originals (with
/// nested container applications rewritten to auxiliary references) followed by
/// the freshly-synthesized auxiliary inductives.
#[derive(Clone, Debug)]
pub struct ExpandedFamily {
    /// Originals (rewritten) followed by auxiliaries — a genuine mutual family.
    pub specs: Vec<InductiveSpec>,
    /// Number of leading `specs` that are the original (user-facing) types.
    pub num_originals: usize,
    /// One entry per auxiliary type (restoration metadata for S3).
    pub aux_maps: Vec<AuxMap>,
    /// Shared level parameters of the family.
    pub lparams: Vec<Name>,
    /// Shared parameter count of the family.
    pub num_params: u32,
}

/// A nested container application found while scanning a constructor type.
struct Found {
    container: Name,
    levels: Vec<Level>,
    /// The container's parameter arguments (length = container `numParams`).
    as_args: Vec<Expr>,
    num_params: usize,
}

fn lookup_inductive<'a>(env: &'a Environment, name: &Name) -> Option<&'a InductiveVal> {
    match env.find(name)? {
        ConstantInfo::Inductive(iv) => Some(iv),
        _ => None,
    }
}

/// `(full type, level params)` of a constructor, if declared.
fn lookup_ctor(env: &Environment, name: &Name) -> Option<(Expr, Vec<Name>)> {
    match env.find(name)? {
        ConstantInfo::Constructor(cv) => {
            Some((cv.common.ty.clone(), cv.common.level_params.clone()))
        }
        _ => None,
    }
}

fn last_component(name: &Name) -> String {
    match name {
        Name::Str(_, s) => s.clone(),
        _ => "ctor".to_string(),
    }
}

/// Instantiate the leading `args.len()` Pi binders of `ty` with `args`.
fn instantiate_leading(ty: &Expr, args: &[Expr]) -> Expr {
    let mut cur = ty.clone();
    for arg in args {
        match cur {
            Expr::Pi(_, _, _dom, body) => {
                cur = instantiate(&body, arg);
            }
            _ => break,
        }
    }
    cur
}

/// Replace every occurrence of `container container_levels as_args` (matched on
/// the first `np` arguments) with `aux` applied to the remaining (index)
/// arguments.
fn replace_container_app(
    ty: &Expr,
    container: &Name,
    container_levels: &[Level],
    as_args: &[Expr],
    np: usize,
    aux: &Name,
) -> Expr {
    replace_expr(ty, &mut |e, _depth| {
        let (head, args) = get_app_fn_args(e);
        if let Expr::Const(n, levels) = head {
            if n == container && levels.as_slice() == container_levels && args.len() >= np {
                let matches = as_args
                    .iter()
                    .zip(args.iter().take(np))
                    .all(|(a, b)| a == *b);
                if matches {
                    let rest: Vec<Expr> = args[np..]
                        .iter()
                        .map(|x| {
                            replace_container_app(x, container, container_levels, as_args, np, aux)
                        })
                        .collect();
                    return Some(mk_app(Expr::Const(aux.clone(), vec![]), &rest));
                }
            }
        }
        None
    })
}

/// Find the first nested container application anywhere inside `e`.
///
/// Returns `Err(UnsupportedNestedInductive)` when a family type occurs in an
/// *index* position of the container (Lean's kernel cannot un-nest those).
#[allow(clippy::result_large_err)]
fn scan(env: &Environment, all_family: &[Name], e: &Expr) -> Result<Option<Found>, KernelError> {
    let (head, args) = get_app_fn_args(e);
    if let Expr::Const(c, levels) = head {
        if !all_family.contains(c) {
            if let Some(iv) = lookup_inductive(env, c) {
                let np = iv.num_params as usize;
                if args.len() >= np {
                    let fam_in_param = args.iter().take(np).any(|a| has_const_occ(a, all_family));
                    let fam_in_index = args.iter().skip(np).any(|a| has_const_occ(a, all_family));
                    if fam_in_index {
                        return Err(KernelError::UnsupportedNestedInductive(
                            all_family.first().cloned().unwrap_or(Name::Anonymous),
                        ));
                    }
                    if fam_in_param {
                        let as_args = args.iter().take(np).map(|x| (**x).clone()).collect();
                        return Ok(Some(Found {
                            container: c.clone(),
                            levels: levels.clone(),
                            as_args,
                            num_params: np,
                        }));
                    }
                }
            }
        }
    }
    match e {
        Expr::App(f, a) => {
            if let Some(x) = scan(env, all_family, f)? {
                return Ok(Some(x));
            }
            scan(env, all_family, a)
        }
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            if let Some(x) = scan(env, all_family, ty)? {
                return Ok(Some(x));
            }
            scan(env, all_family, body)
        }
        Expr::Let(_, ty, val, body) => {
            if let Some(x) = scan(env, all_family, ty)? {
                return Ok(Some(x));
            }
            if let Some(x) = scan(env, all_family, val)? {
                return Ok(Some(x));
            }
            scan(env, all_family, body)
        }
        Expr::Proj(_, _, inner) => scan(env, all_family, inner),
        _ => Ok(None),
    }
}

/// Specialize the nested container occurrences of `specs` into an auxiliary
/// mutual family (S2). Returns `Ok(None)` when there is no nesting (the caller
/// should use the ordinary derivation path), or when the shape is not yet
/// supported by this stage (parametrized families or multi-member containers),
/// in which case the ordinary path re-produces the current `UnsupportedNested`
/// verdict — so behaviour never silently changes.
#[allow(clippy::result_large_err)]
pub fn specialize(
    env: &Environment,
    lparams: &[Name],
    num_params: u32,
    specs: &[InductiveSpec],
) -> Result<Option<ExpandedFamily>, KernelError> {
    // S2 scope: family with no shared parameters (covers `Lean.Syntax` and the
    // textbook `NestedT`). Parametrized nested families need family-parameter
    // re-binding in the auxiliaries — deferred; fall back to the ordinary path.
    if num_params != 0 {
        return Ok(None);
    }

    let orig_names: Vec<Name> = specs.iter().map(|s| s.name.clone()).collect();
    let mut all_family = orig_names.clone();
    let mut cur_specs = specs.to_vec();
    let mut aux_maps: Vec<AuxMap> = Vec::new();
    let mut aux_count = 0usize;
    let mut fuel = 256usize;

    loop {
        fuel = fuel.checked_sub(1).ok_or_else(|| {
            KernelError::Other("nested inductive specialization did not terminate".to_string())
        })?;

        // Find the first not-yet-specialized nested occurrence in any ctor type.
        let mut found: Option<Found> = None;
        'outer: for spec in &cur_specs {
            for (_, cty) in &spec.ctors {
                if let Some(f) = scan(env, &all_family, cty)? {
                    found = Some(f);
                    break 'outer;
                }
            }
        }
        let Some(f) = found else { break };

        let iv = match lookup_inductive(env, &f.container) {
            // Multi-member mutual containers are not handled by this stage.
            Some(iv) if iv.all.len() == 1 => iv.clone(),
            _ => return Ok(None),
        };

        let aux_name = Name::Str(
            Box::new(orig_names[0].clone()),
            format!("_nested_{}", aux_count),
        );
        aux_count += 1;
        let c_levels = f.levels.clone();
        let as_args = f.as_args.clone();
        let np = f.num_params;

        // Auxiliary type: the container's type with its level params and its
        // `np` parameters fixed. (No family parameters to re-bind: `num_params`
        // is 0 in this stage.)
        let aux_ty = {
            let t = instantiate_type_lparams(&iv.common.ty, &iv.common.level_params, &c_levels);
            instantiate_leading(&t, &as_args)
        };

        // Auxiliary constructors: each container ctor specialized at `As`, with
        // the container application rewritten to the auxiliary reference.
        let mut aux_ctors: Vec<(Name, Expr)> = Vec::with_capacity(iv.ctors.len());
        let mut ctor_name_map: Vec<(Name, Name)> = Vec::with_capacity(iv.ctors.len());
        for cn in &iv.ctors {
            let (cty, clp) = lookup_ctor(env, cn).ok_or_else(|| {
                KernelError::Other(format!(
                    "constructor '{}' of nested container '{}' not found",
                    cn, f.container
                ))
            })?;
            let t = instantiate_type_lparams(&cty, &clp, &c_levels);
            let t = instantiate_leading(&t, &as_args);
            let t = replace_container_app(&t, &f.container, &c_levels, &as_args, np, &aux_name);
            let aux_cn = Name::Str(Box::new(aux_name.clone()), last_component(cn));
            aux_ctors.push((aux_cn.clone(), t));
            ctor_name_map.push((aux_cn, cn.clone()));
        }

        aux_maps.push(AuxMap {
            aux_type: aux_name.clone(),
            container: f.container.clone(),
            container_levels: c_levels.clone(),
            params: as_args.clone(),
            aux_ctors: ctor_name_map,
            aux_rec: Name::Str(Box::new(aux_name.clone()), "rec".to_string()),
            container_rec: Name::Str(Box::new(f.container.clone()), "rec".to_string()),
        });

        all_family.push(aux_name.clone());
        cur_specs.push(InductiveSpec::new(aux_name.clone(), aux_ty, aux_ctors));

        // Rewrite `container As` → aux across every ctor type (originals + aux).
        for spec in cur_specs.iter_mut() {
            for (_, cty) in spec.ctors.iter_mut() {
                *cty = replace_container_app(cty, &f.container, &c_levels, &as_args, np, &aux_name);
            }
        }
    }

    if all_family.len() == orig_names.len() {
        return Ok(None);
    }

    Ok(Some(ExpandedFamily {
        specs: cur_specs,
        num_originals: orig_names.len(),
        aux_maps,
        lparams: lparams.to_vec(),
        num_params,
    }))
}

/// Nested-aware family derivation. When `specs` contains a nested inductive,
/// specialize it into an auxiliary mutual family and derive that (returning the
/// [`ExpandedFamily`] so S3 can restore the recursors); otherwise fall back to
/// the ordinary [`check_and_derive_family`].
///
/// The derived recursors are still the **raw** auxiliary recursors — restoration
/// over the real container is S3.
#[allow(clippy::result_large_err)]
pub fn check_and_derive_family_nested(
    env: &Environment,
    lparams: &[Name],
    num_params: u32,
    specs: &[InductiveSpec],
) -> Result<(DerivedFamily, Option<ExpandedFamily>), KernelError> {
    match specialize(env, lparams, num_params, specs)? {
        None => Ok((
            check_and_derive_family(env, lparams, num_params, specs)?,
            None,
        )),
        Some(exp) => {
            let fam = check_and_derive_family(env, &exp.lparams, exp.num_params, &exp.specs)?;
            Ok((fam, Some(exp)))
        }
    }
}

// ---------------------------------------------------------------------------
// S3 — restoration (auxiliary family → real container).
// ---------------------------------------------------------------------------

/// Rewrite auxiliary type-formers and constructors back to the real container:
/// `Const(aux_type)` → `container As`, `Const(aux_ctor)` → `real_ctor As`. The
/// auxiliary recursor names are left untouched (they are retained, restored, in
/// the environment). Because only the head `Const` is rewritten, any following
/// arguments (indices / constructor fields) are preserved.
fn restore_expr(e: &Expr, aux_maps: &[AuxMap]) -> Expr {
    replace_expr(e, &mut |sub, _depth| {
        if let Expr::Const(name, _levels) = sub {
            for m in aux_maps {
                if name == &m.aux_type {
                    return Some(mk_app(
                        Expr::Const(m.container.clone(), m.container_levels.clone()),
                        &m.params,
                    ));
                }
                for (aux_ctor, real_ctor) in &m.aux_ctors {
                    if name == aux_ctor {
                        return Some(mk_app(
                            Expr::Const(real_ctor.clone(), m.container_levels.clone()),
                            &m.params,
                        ));
                    }
                }
            }
        }
        None
    })
}

/// Map an auxiliary constructor name in a recursor rule back to the real
/// container constructor, so the reducer keys the rule by the real ctor.
fn restore_ctor_name(ctor: &Name, aux_maps: &[AuxMap]) -> Name {
    for m in aux_maps {
        for (aux_ctor, real_ctor) in &m.aux_ctors {
            if ctor == aux_ctor {
                return real_ctor.clone();
            }
        }
    }
    ctor.clone()
}

/// Restore a raw expanded-family derivation (S3): rewrite the auxiliary
/// type-formers / constructors back to the real container in every recursor
/// type and rule, re-key rules by the real constructor, drop the auxiliary
/// inductives and constructors, keep only the original types' inductives and
/// constructors (with `num_nested` set and `all` narrowed to the originals),
/// and RETAIN the auxiliary recursors — now stated over the real container —
/// because reducing a user-facing recursor still emits them.
pub fn restore(fam: DerivedFamily, exp: &ExpandedFamily) -> DerivedFamily {
    let orig_names: Vec<Name> = exp.specs[..exp.num_originals]
        .iter()
        .map(|s| s.name.clone())
        .collect();
    let num_nested = exp.aux_maps.len() as u32;

    let mut inductives = Vec::with_capacity(exp.num_originals);
    for ci in fam.inductives.iter().take(exp.num_originals) {
        if let ConstantInfo::Inductive(iv) = ci {
            let mut iv = iv.clone();
            iv.common.ty = restore_expr(&iv.common.ty, &exp.aux_maps);
            iv.all = orig_names.clone();
            iv.num_nested = num_nested;
            inductives.push(ConstantInfo::Inductive(iv));
        }
    }

    let mut constructors = Vec::new();
    for ci in &fam.constructors {
        if let ConstantInfo::Constructor(cv) = ci {
            if orig_names.contains(&cv.induct) {
                let mut cv = cv.clone();
                cv.common.ty = restore_expr(&cv.common.ty, &exp.aux_maps);
                constructors.push(ConstantInfo::Constructor(cv));
            }
        }
    }

    let mut recursors = Vec::with_capacity(fam.recursors.len());
    for ci in &fam.recursors {
        if let ConstantInfo::Recursor(rv) = ci {
            let mut rv = rv.clone();
            rv.common.ty = restore_expr(&rv.common.ty, &exp.aux_maps);
            rv.all = orig_names.clone();
            rv.rules = rv
                .rules
                .iter()
                .map(|rule| RecursorRule {
                    ctor: restore_ctor_name(&rule.ctor, &exp.aux_maps),
                    nfields: rule.nfields,
                    rhs: restore_expr(&rule.rhs, &exp.aux_maps),
                })
                .collect();
            recursors.push(ConstantInfo::Recursor(rv));
        }
    }

    DerivedFamily {
        inductives,
        constructors,
        recursors,
    }
}

// ---------------------------------------------------------------------------
// S4 — export replay: verify exported recursors against the restored family.
// ---------------------------------------------------------------------------

/// The sorted set of constructor names a recursor's rules fire on. Uniquely
/// identifies which (original or container) member a restored recursor belongs
/// to, so exported and re-derived recursors can be paired without relying on a
/// naming convention.
fn rule_ctor_set(rv: &RecursorVal) -> Vec<Name> {
    let mut s: Vec<Name> = rv.rules.iter().map(|r| r.ctor.clone()).collect();
    s.sort();
    s
}

/// Rewrite recursor self-references according to `name_map` (provisional → real
/// exported name), in every recursor's own name and in the `Const` references
/// inside recursor types and rule right-hand sides.
fn rename_recursors(fam: DerivedFamily, name_map: &[(Name, Name)]) -> DerivedFamily {
    fn rename_expr(e: &Expr, name_map: &[(Name, Name)]) -> Expr {
        replace_expr(e, &mut |sub, _depth| {
            if let Expr::Const(n, levels) = sub {
                for (from, to) in name_map {
                    if n == from {
                        return Some(Expr::Const(to.clone(), levels.clone()));
                    }
                }
            }
            None
        })
    }
    let mut recursors = Vec::with_capacity(fam.recursors.len());
    for ci in fam.recursors {
        if let ConstantInfo::Recursor(mut rv) = ci {
            for (from, to) in name_map {
                if rv.common.name == *from {
                    rv.common.name = to.clone();
                    break;
                }
            }
            rv.common.ty = rename_expr(&rv.common.ty, name_map);
            rv.rules = rv
                .rules
                .into_iter()
                .map(|r| RecursorRule {
                    ctor: r.ctor,
                    nfields: r.nfields,
                    rhs: rename_expr(&r.rhs, name_map),
                })
                .collect();
            recursors.push(ConstantInfo::Recursor(rv));
        } else {
            recursors.push(ci);
        }
    }
    DerivedFamily {
        inductives: fam.inductives,
        constructors: fam.constructors,
        recursors,
    }
}

/// Verify a nested `inductive` bundle for export replay and return the
/// kernel-derived family to install (recursors renamed to the exported names).
///
/// Runs the nested-aware derivation (specialize → derive → restore), pairs each
/// re-derived recursor with an exported one by the set of constructors its rules
/// fire on, renames the re-derived recursors to the exported names (so
/// self-references align), then requires every exported recursor to be
/// definitionally equal to the corresponding re-derived one
/// ([`recursor_matches`]). A missing pair or a failed def-eq is a rejection —
/// no exported recursor is ever accepted without a sound kernel re-derivation.
///
/// Returns [`KernelError::UnsupportedNestedInductive`] for shapes this stage
/// cannot un-nest (parametrized families, family in an index position,
/// non-inductive or multi-member containers), which replay buckets as the named
/// `nested inductives` unsupported feature — never a wrong accept.
#[allow(clippy::result_large_err)]
pub fn verify_nested_bundle(
    env: &Environment,
    lparams: &[Name],
    num_params: u32,
    specs: &[InductiveSpec],
    exported_recs: &[RecursorVal],
) -> Result<DerivedFamily, KernelError> {
    let (raw, exp_opt) = check_and_derive_family_nested(env, lparams, num_params, specs)?;
    let Some(exp) = exp_opt else {
        return Err(KernelError::UnsupportedNestedInductive(
            specs
                .first()
                .map(|s| s.name.clone())
                .unwrap_or(Name::Anonymous),
        ));
    };
    let restored = restore(raw, &exp);

    if restored.recursors.len() != exported_recs.len() {
        return Err(KernelError::InvalidRecursor(format!(
            "nested inductive '{}': kernel derived {} recursors but the export lists {}",
            specs[0].name,
            restored.recursors.len(),
            exported_recs.len()
        )));
    }

    // Pair each re-derived recursor with an exported one by rule-constructor set.
    let mut name_map: Vec<(Name, Name)> = Vec::with_capacity(restored.recursors.len());
    for ci in &restored.recursors {
        if let ConstantInfo::Recursor(rv) = ci {
            let set = rule_ctor_set(rv);
            let er = exported_recs
                .iter()
                .find(|er| rule_ctor_set(er) == set)
                .ok_or_else(|| {
                    KernelError::InvalidRecursor(format!(
                        "nested inductive '{}': no exported recursor eliminates the same \
                         constructors as the kernel-derived '{}'",
                        specs[0].name, rv.common.name
                    ))
                })?;
            name_map.push((rv.common.name.clone(), er.common.name.clone()));
        }
    }

    let renamed = rename_recursors(restored, &name_map);

    // Every exported recursor must def-eq the re-derived one of the same name.
    let mut tc = TypeChecker::new(env);
    for er in exported_recs {
        let mut ok = false;
        for ci in &renamed.recursors {
            if let ConstantInfo::Recursor(d) = ci {
                if d.common.name == er.common.name && recursor_matches(&mut tc, d, er) {
                    ok = true;
                    break;
                }
            }
        }
        if !ok {
            return Err(KernelError::InvalidRecursor(format!(
                "exported recursor '{}' does not match the kernel-derived nested recursor",
                er.common.name
            )));
        }
    }

    Ok(renamed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Node;

    fn c(name: &str, levels: Vec<Level>) -> Expr {
        Expr::Const(Name::str(name), levels)
    }

    fn app(f: Expr, a: Expr) -> Expr {
        Expr::App(Node::new(f), Node::new(a))
    }

    #[test]
    fn detects_list_of_family() {
        // field type `List T`, family = {T}: nested under List at position 0.
        let fam = [Name::str("T")];
        let ty = app(c("List", vec![Level::zero()]), c("T", vec![]));
        let occ = detect(&fam, &ty).expect("nested occurrence");
        assert_eq!(occ.container, Name::str("List"));
        assert_eq!(occ.container_levels, vec![Level::zero()]);
        assert_eq!(occ.family_positions, vec![0]);
        assert_eq!(occ.args.len(), 1);
    }

    #[test]
    fn direct_occurrence_is_not_nested() {
        // `T x` — head is the family member itself: ordinary recursion.
        let fam = [Name::str("T")];
        let ty = app(c("T", vec![]), c("x", vec![]));
        assert!(detect(&fam, &ty).is_none());
    }

    #[test]
    fn foreign_container_without_family_is_not_nested() {
        // `List Nat` — foreign container, no family type inside.
        let fam = [Name::str("T")];
        let ty = app(c("List", vec![Level::zero()]), c("Nat", vec![]));
        assert!(detect(&fam, &ty).is_none());
    }

    #[test]
    fn bare_const_is_not_nested() {
        let fam = [Name::str("T")];
        assert!(detect(&fam, &c("Nat", vec![])).is_none());
    }

    #[test]
    fn family_in_later_argument_position() {
        // `Prod Nat T` — family at position 1 only.
        let fam = [Name::str("T")];
        let ty = app(
            app(
                c("Prod", vec![Level::zero(), Level::zero()]),
                c("Nat", vec![]),
            ),
            c("T", vec![]),
        );
        let occ = detect(&fam, &ty).expect("nested occurrence");
        assert_eq!(occ.container, Name::str("Prod"));
        assert_eq!(occ.family_positions, vec![1]);
        assert_eq!(occ.args.len(), 2);
    }

    #[test]
    fn family_nested_deeper_in_argument() {
        // `Array (List T)` — family occurs (transitively) inside Array's arg 0.
        let fam = [Name::str("T")];
        let inner = app(c("List", vec![Level::zero()]), c("T", vec![]));
        let ty = app(c("Array", vec![Level::zero()]), inner);
        let occ = detect(&fam, &ty).expect("nested occurrence");
        assert_eq!(occ.container, Name::str("Array"));
        assert_eq!(occ.family_positions, vec![0]);
    }

    #[test]
    fn multiple_family_members() {
        // family = {T, U}; `Sum T U` nests both at positions 0 and 1.
        let fam = [Name::str("T"), Name::str("U")];
        let ty = app(
            app(c("Sum", vec![Level::zero(), Level::zero()]), c("T", vec![])),
            c("U", vec![]),
        );
        let occ = detect(&fam, &ty).expect("nested occurrence");
        assert_eq!(occ.family_positions, vec![0, 1]);
    }

    // -- S2: specialization + nested derivation ----------------------------

    #[test]
    fn non_nested_family_uses_ordinary_path() {
        // `inductive Foo | mk : Foo` has no nesting: specialize returns None.
        let env = Environment::new();
        let spec = InductiveSpec::new(
            Name::str("Foo"),
            Expr::Sort(Level::succ(Level::zero())),
            vec![(Name::str("Foo.mk"), c("Foo", vec![]))],
        );
        let (fam, exp) =
            check_and_derive_family_nested(&env, &[], 0, &[spec]).expect("ordinary derivation");
        assert!(exp.is_none(), "non-nested family must not expand");
        assert_eq!(fam.recursors.len(), 1);
    }

    #[test]
    fn nested_t_derives_via_specialization() {
        // `inductive NestedT | mk : List NestedT → NestedT`.
        let mut env = Environment::new();
        crate::builtin::init_builtin_env(&mut env).expect("builtin env");
        let t = c("NestedT", vec![]);
        let list_t = app(c("List", vec![Level::zero()]), t.clone());
        let spec = InductiveSpec::new(
            Name::str("NestedT"),
            Expr::Sort(Level::succ(Level::zero())),
            vec![(
                Name::str("NestedT.mk"),
                Expr::Pi(
                    crate::BinderInfo::Default,
                    Name::str("l"),
                    Node::new(list_t),
                    Node::new(t),
                ),
            )],
        );

        let (fam, exp) = check_and_derive_family_nested(&env, &[], 0, &[spec])
            .expect("nested inductive now derives");
        let exp = exp.expect("NestedT must be recognised as nested");

        // Expanded to the mutual family { NestedT, aux List-of-NestedT }.
        assert_eq!(exp.num_originals, 1);
        assert_eq!(exp.specs.len(), 2);
        assert_eq!(exp.aux_maps.len(), 1);
        let m = &exp.aux_maps[0];
        assert_eq!(m.container, Name::str("List"));
        let real_ctors: Vec<Name> = m.aux_ctors.iter().map(|(_, r)| r.clone()).collect();
        assert!(real_ctors.contains(&Name::str("List.nil")));
        assert!(real_ctors.contains(&Name::str("List.cons")));

        // Two inductives, three constructors (mk, nil✝, cons✝), two recursors.
        assert_eq!(fam.inductives.len(), 2);
        assert_eq!(fam.constructors.len(), 3);
        assert_eq!(fam.recursors.len(), 2);

        // NestedT's recursor carries a motive per family member (2) and a minor
        // per constructor (3), and fires on `NestedT.mk`.
        let nestedt_rec = fam
            .recursors
            .iter()
            .find_map(|ci| match ci {
                ConstantInfo::Recursor(rv) if rv.get_rule(&Name::str("NestedT.mk")).is_some() => {
                    Some(rv)
                }
                _ => None,
            })
            .expect("a recursor eliminating NestedT.mk");
        assert_eq!(nestedt_rec.num_motives, 2);
        assert_eq!(nestedt_rec.num_minors, 3);
    }

    #[test]
    fn nested_t_aux_recursor_iota_reduces() {
        // The raw auxiliary recursor must iota-reduce like any mutual recursor:
        // `ListNestedT.rec mN mL minMk minNil minCons ListNestedT.nil → minNil`.
        let mut env = crate::Environment::new();
        crate::builtin::init_builtin_env(&mut env).expect("builtin env");
        let t = c("NestedT", vec![]);
        let list_t = app(c("List", vec![Level::zero()]), t.clone());
        let spec = InductiveSpec::new(
            Name::str("NestedT"),
            Expr::Sort(Level::succ(Level::zero())),
            vec![(
                Name::str("NestedT.mk"),
                Expr::Pi(
                    crate::BinderInfo::Default,
                    Name::str("l"),
                    Node::new(list_t),
                    Node::new(t),
                ),
            )],
        );
        let (fam, exp) = check_and_derive_family_nested(&env, &[], 0, &[spec]).expect("derive");
        let exp = exp.expect("nested");

        // Install the whole expanded family so the reducer can see the aux
        // constructor (for its cidx) and the aux recursor (for its rules).
        let mut env2 = env.clone();
        for ci in fam.clone().into_constant_infos() {
            env2.add_constant(ci).expect("install expanded family");
        }

        let aux = &exp.aux_maps[0];
        let aux_nil = aux
            .aux_ctors
            .iter()
            .find(|(_, r)| *r == Name::str("List.nil"))
            .expect("aux nil ctor")
            .0
            .clone();

        // The recursor gets a fresh elimination level param (large elimination);
        // any level argument works for a reduction that never typechecks.
        let ph = |s: &str| Expr::Const(Name::str(s), vec![]);
        let e = mk_app(
            Expr::Const(aux.aux_rec.clone(), vec![Level::zero()]),
            &[
                ph("mN"),
                ph("mL"),
                ph("minMk"),
                ph("minNil"),
                ph("minCons"),
                Expr::Const(aux_nil, vec![]),
            ],
        );

        let reduced = crate::TypeChecker::new(&env2).whnf(&e);
        let head = get_app_fn_args(&reduced).0.clone();
        assert_eq!(
            head,
            Expr::Const(Name::str("minNil"), vec![]),
            "aux recursor did not iota-reduce to the nil minor: {:?}",
            reduced
        );
    }

    #[test]
    fn reach_through_double_nesting_expands_to_three() {
        // `inductive Two | mk : List (List Two) → Two` reaches through one
        // container into another (like `Lean.Syntax`'s Array-then-List), so the
        // fixpoint must synthesize TWO auxiliaries: List-of-Two and
        // List-of-(List-of-Two).
        let mut env = crate::Environment::new();
        crate::builtin::init_builtin_env(&mut env).expect("builtin env");
        let two = c("Two", vec![]);
        let list = |x: Expr| app(c("List", vec![Level::zero()]), x);
        let spec = InductiveSpec::new(
            Name::str("Two"),
            Expr::Sort(Level::succ(Level::zero())),
            vec![(
                Name::str("Two.mk"),
                Expr::Pi(
                    crate::BinderInfo::Default,
                    Name::str("l"),
                    Node::new(list(list(two.clone()))),
                    Node::new(two),
                ),
            )],
        );
        let (fam, exp) = check_and_derive_family_nested(&env, &[], 0, &[spec]).expect("derive");
        let exp = exp.expect("nested");
        assert_eq!(exp.num_originals, 1);
        assert_eq!(exp.specs.len(), 3, "Two + two List auxiliaries");
        assert_eq!(exp.aux_maps.len(), 2);
        // Three members ⇒ three motives / four minors (mk + nil/cons ×2 = 5?).
        // mk(1) + aux0 nil,cons(2) + aux1 nil,cons(2) = 5 minors, 3 motives.
        let rec = fam
            .recursors
            .iter()
            .find_map(|ci| match ci {
                ConstantInfo::Recursor(rv) if rv.get_rule(&Name::str("Two.mk")).is_some() => {
                    Some(rv)
                }
                _ => None,
            })
            .expect("Two's recursor");
        assert_eq!(rec.num_motives, 3);
        assert_eq!(rec.num_minors, 5);
    }

    // -- S3: restoration over the real container ---------------------------

    #[test]
    fn nested_t_restore_produces_real_list_recursor() {
        use crate::expr_util::collect_consts;
        let mut env = crate::Environment::new();
        crate::builtin::init_builtin_env(&mut env).expect("builtin env");
        let t = c("NestedT", vec![]);
        let list_t = app(c("List", vec![Level::zero()]), t.clone());
        let spec = InductiveSpec::new(
            Name::str("NestedT"),
            Expr::Sort(Level::succ(Level::zero())),
            vec![(
                Name::str("NestedT.mk"),
                Expr::Pi(
                    crate::BinderInfo::Default,
                    Name::str("l"),
                    Node::new(list_t),
                    Node::new(t.clone()),
                ),
            )],
        );
        let (fam, exp) = check_and_derive_family_nested(&env, &[], 0, &[spec]).expect("derive");
        let exp = exp.expect("nested");
        let aux_type = exp.aux_maps[0].aux_type.clone();
        let aux_rec = exp.aux_maps[0].aux_rec.clone();
        let restored = restore(fam, &exp);

        // Only the user-facing type survives: 1 inductive, 1 ctor, 2 recursors.
        assert_eq!(restored.inductives.len(), 1);
        assert_eq!(restored.constructors.len(), 1);
        assert_eq!(restored.recursors.len(), 2);

        match &restored.inductives[0] {
            ConstantInfo::Inductive(iv) => {
                assert_eq!(iv.num_nested, 1);
                assert_eq!(iv.all, vec![Name::str("NestedT")]);
            }
            _ => panic!("expected inductive"),
        }

        // NestedT.mk now mentions the real `List`, not the auxiliary type.
        match &restored.constructors[0] {
            ConstantInfo::Constructor(cv) => {
                let consts = collect_consts(&cv.common.ty);
                assert!(consts.contains(&Name::str("List")));
                assert!(!consts.contains(&aux_type));
            }
            _ => panic!("expected constructor"),
        }

        // Install the restored family (real `List` already present); every
        // restored recursor type must re-typecheck.
        let mut env2 = env.clone();
        for ci in restored.clone().into_constant_infos() {
            env2.add_constant(ci).expect("install restored family");
        }
        for ci in &restored.recursors {
            if let ConstantInfo::Recursor(rv) = ci {
                crate::TypeChecker::new(&env2)
                    .infer_type(&rv.common.ty)
                    .expect("restored recursor type re-checks");
            }
        }

        let ph = |s: &str| Expr::Const(Name::str(s), vec![]);
        let nil_app = mk_app(
            Expr::Const(Name::str("List.nil"), vec![Level::zero()]),
            std::slice::from_ref(&t),
        );

        // The retained auxiliary recursor iota-reduces on a REAL `List.nil`.
        let e_aux = mk_app(
            Expr::Const(aux_rec, vec![Level::zero()]),
            &[
                ph("mN"),
                ph("mL"),
                ph("minMk"),
                ph("minNil"),
                ph("minCons"),
                nil_app.clone(),
            ],
        );
        let r_aux = crate::TypeChecker::new(&env2).whnf(&e_aux);
        assert_eq!(
            get_app_fn_args(&r_aux).0.clone(),
            Expr::Const(Name::str("minNil"), vec![]),
            "restored aux recursor did not fire on real List.nil: {:?}",
            r_aux
        );

        // The user-facing recursor fires on `NestedT.mk (@List.nil NestedT)`.
        let main_rec = restored
            .recursors
            .iter()
            .find_map(|ci| match ci {
                ConstantInfo::Recursor(rv) if rv.get_rule(&Name::str("NestedT.mk")).is_some() => {
                    Some(rv.common.name.clone())
                }
                _ => None,
            })
            .expect("NestedT.rec");
        let major = app(c("NestedT.mk", vec![]), nil_app);
        let e_main = mk_app(
            Expr::Const(main_rec, vec![Level::zero()]),
            &[
                ph("mN"),
                ph("mL"),
                ph("minMk"),
                ph("minNil"),
                ph("minCons"),
                major,
            ],
        );
        let r_main = crate::TypeChecker::new(&env2).whnf(&e_main);
        assert_eq!(
            get_app_fn_args(&r_main).0.clone(),
            Expr::Const(Name::str("minMk"), vec![]),
            "restored NestedT.rec did not fire on NestedT.mk: {:?}",
            r_main
        );
    }

    // -- S4: verify exported recursors against the restored family ----------

    /// Build NestedT env + spec, and the exported recursors as Lean would name
    /// them (`NestedT.rec` over `NestedT.mk`, `NestedT.rec_1` over List ctors).
    fn nested_t_bundle() -> (crate::Environment, InductiveSpec, Vec<RecursorVal>) {
        let mut env = crate::Environment::new();
        crate::builtin::init_builtin_env(&mut env).expect("builtin env");
        let t = c("NestedT", vec![]);
        let list_t = app(c("List", vec![Level::zero()]), t.clone());
        let spec = InductiveSpec::new(
            Name::str("NestedT"),
            Expr::Sort(Level::succ(Level::zero())),
            vec![(
                Name::str("NestedT.mk"),
                Expr::Pi(
                    crate::BinderInfo::Default,
                    Name::str("l"),
                    Node::new(list_t),
                    Node::new(t),
                ),
            )],
        );
        let (raw, exp) = check_and_derive_family_nested(&env, &[], 0, std::slice::from_ref(&spec))
            .expect("derive");
        let exp = exp.expect("nested");
        let restored = restore(raw, &exp);
        // Canonical export names, keyed by which constructors each rec fires on.
        let name_map: Vec<(Name, Name)> = restored
            .recursors
            .iter()
            .filter_map(|ci| match ci {
                ConstantInfo::Recursor(rv) => {
                    let exported = if rule_ctor_set(rv).contains(&Name::str("NestedT.mk")) {
                        Name::Str(Box::new(Name::str("NestedT")), "rec".to_string())
                    } else {
                        Name::Str(Box::new(Name::str("NestedT")), "rec_1".to_string())
                    };
                    Some((rv.common.name.clone(), exported))
                }
                _ => None,
            })
            .collect();
        let canonical = rename_recursors(restored, &name_map);
        let exported_recs: Vec<RecursorVal> = canonical
            .recursors
            .iter()
            .filter_map(|ci| match ci {
                ConstantInfo::Recursor(rv) => Some(rv.clone()),
                _ => None,
            })
            .collect();
        (env, spec, exported_recs)
    }

    #[test]
    fn verify_nested_bundle_accepts_matching_export() {
        let (env, spec, exported_recs) = nested_t_bundle();
        let out = verify_nested_bundle(&env, &[], 0, &[spec], &exported_recs)
            .expect("matching export verifies");
        // Recursors renamed to the exported names.
        let names: Vec<Name> = out
            .recursors
            .iter()
            .filter_map(|ci| match ci {
                ConstantInfo::Recursor(rv) => Some(rv.common.name.clone()),
                _ => None,
            })
            .collect();
        assert!(names.contains(&Name::Str(
            Box::new(Name::str("NestedT")),
            "rec".to_string()
        )));
        assert!(names.contains(&Name::Str(
            Box::new(Name::str("NestedT")),
            "rec_1".to_string()
        )));
    }

    #[test]
    fn verify_nested_bundle_rejects_corrupted_recursor() {
        let (env, spec, mut exported_recs) = nested_t_bundle();
        // Corrupt one exported recursor's metadata: the def-eq match must fail.
        exported_recs[0].num_minors += 1;
        let err = verify_nested_bundle(&env, &[], 0, &[spec], &exported_recs)
            .expect_err("corrupted export must be rejected");
        assert!(matches!(err, KernelError::InvalidRecursor(_)));
    }

    #[test]
    fn verify_nested_bundle_rejects_wrong_recursor_count() {
        let (env, spec, mut exported_recs) = nested_t_bundle();
        exported_recs.pop();
        let err = verify_nested_bundle(&env, &[], 0, &[spec], &exported_recs)
            .expect_err("wrong recursor count must be rejected");
        assert!(matches!(err, KernelError::InvalidRecursor(_)));
    }
}
