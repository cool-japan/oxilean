//! A term that arrives from outside, held against the term the kernel made.
//!
//! Some declarations are not checked, they are **made**: Lean's kernel
//! derives the recursors of an inductive family when the family is
//! declared, and it builds the types of the quotient primitives itself. It
//! never takes either from its input. When such a declaration nevertheless
//! arrives from outside — a recursor record or a `quot` record of an export
//! — it has one use: to be *compared* with what this kernel made. What goes
//! into the environment, and what iota reduction later runs on, is the
//! kernel's own term, never the supplied one.
//!
//! The comparison keeps to one rule: **a supplied term reaches definitional
//! equality only after it was type-checked.** Definitional equality is
//! defined on well-typed terms. Proof irrelevance, for one, equates two
//! terms whose types it reads off their heads without checking the
//! arguments, so an ill-typed term in a proof position compares equal to any
//! proof of the same proposition. A recursor rule whose right-hand side
//! carried such a term used to pass the comparison and — while the supplied
//! recursor was still the one installed — reduce to another value at a
//! constructor: a closed proof of `False` from no axiom.
//!
//! So a supplied term is compared in two steps:
//!
//! 1. **As written** ([`same_up_to_binders`]): the same term up to the names
//!    and the binder information of its binders. For a recursor the
//!    record's universe parameter NAMES are taken first — the derivation is
//!    put under them (`derived_under_supplied_names`) before the two are
//!    compared — so a record whose universe parameter names are exchanged
//!    consistently passes this step; Lean's replay, which compares the
//!    record with its own recursor by `==`, does not. An export written by
//!    Lean carries the kernel's derivation, so this settles it, and nothing
//!    is reduced.
//! 2. **Checked** (only when step 1 fails): the supplied term must be closed
//!    and well typed — a type must be a type; a rule's right-hand side must
//!    have the type of the derived right-hand side — and only then are the
//!    two compared by definitional equality, in an environment that holds
//!    the kernel's own family.
//!
//! Either way the supplied term is dropped afterwards. A record that passes
//! only the second step, or only after its universe parameter names were
//! taken, is accepted where Lean's replay marks it — up to beta, proof
//! irrelevance, eta, an equivalent universe level, or consistently
//! exchanged universe parameter names — and what is installed is the
//! derivation either way.

use crate::declaration::{ConstantInfo, RecursorRule, RecursorVal};
use crate::instantiate::instantiate_type_lparams;
use crate::{Environment, Expr, Level, Node, TypeChecker};

/// Whether two terms are the same up to the names and the binder
/// information of their binders: the same shape, the same constants at the
/// same universe levels (as written, not up to equivalence), the same bound
/// variables, the same literals. This is the equality Lean's `Expr` has
/// (`==`), and nothing is reduced to decide it.
pub fn same_up_to_binders(a: &Expr, b: &Expr) -> bool {
    match (a, b) {
        (Expr::BVar(i), Expr::BVar(j)) => i == j,
        (Expr::FVar(x), Expr::FVar(y)) => x == y,
        (Expr::Sort(l1), Expr::Sort(l2)) => l1 == l2,
        (Expr::Const(n1, ls1), Expr::Const(n2, ls2)) => n1 == n2 && ls1 == ls2,
        (Expr::Lit(l1), Expr::Lit(l2)) => l1 == l2,
        (Expr::App(f1, a1), Expr::App(f2, a2)) => same_node(f1, f2) && same_node(a1, a2),
        (Expr::Lam(_, _, t1, b1), Expr::Lam(_, _, t2, b2))
        | (Expr::Pi(_, _, t1, b1), Expr::Pi(_, _, t2, b2)) => {
            same_node(t1, t2) && same_node(b1, b2)
        }
        (Expr::Let(_, t1, v1, b1), Expr::Let(_, t2, v2, b2)) => {
            same_node(t1, t2) && same_node(v1, v2) && same_node(b1, b2)
        }
        (Expr::Proj(n1, i1, e1), Expr::Proj(n2, i2, e2)) => {
            n1 == n2 && i1 == i2 && same_node(e1, e2)
        }
        _ => false,
    }
}

/// [`same_up_to_binders`] on two subterms; one allocation is itself.
fn same_node(a: &Node, b: &Node) -> bool {
    Node::ptr_eq(a, b) || same_up_to_binders(a, b)
}

/// A supplied term may be handed to the type checker: it is closed (no free
/// variable, no loose bound variable) and no deeper than this thread allows.
fn may_be_checked(supplied: &Expr) -> bool {
    let name = crate::Name::default();
    crate::check::check_term_depth(&name, supplied).is_ok()
        && crate::check::check_closed_term(&name, supplied).is_ok()
}

/// Whether the supplied type is the kernel's: the same as written, or a
/// closed term that is a type and — only then — definitionally equal to the
/// kernel's.
fn type_matches(tc: &mut TypeChecker<'_>, kernel: &Expr, supplied: &Expr) -> bool {
    same_up_to_binders(kernel, supplied)
        || (may_be_checked(supplied)
            && tc.ensure_sort(supplied).is_ok()
            && tc.is_def_eq(kernel, supplied))
}

/// Whether the supplied value is the kernel's: the same as written, or a
/// closed, well-typed term whose type is the type of the kernel's and which
/// is — only then — definitionally equal to the kernel's.
fn value_matches(tc: &mut TypeChecker<'_>, kernel: &Expr, supplied: &Expr) -> bool {
    if same_up_to_binders(kernel, supplied) {
        return true;
    }
    if !may_be_checked(supplied) {
        return false;
    }
    let Ok(supplied_ty) = tc.infer_type(supplied) else {
        return false;
    };
    let Ok(kernel_ty) = tc.infer_type(kernel) else {
        return false;
    };
    tc.is_def_eq(&kernel_ty, &supplied_ty) && tc.is_def_eq(kernel, supplied)
}

/// Whether a type that arrived from outside the kernel is the type
/// `kernel` — one the kernel constructed itself and has in `env`, such as
/// the canonical type of a quotient primitive. The supplied type is compared
/// as written first; failing that it must be a closed term that is a type in
/// `env`, and only then is it compared by definitional equality (see the
/// module docs). The supplied type is not installed by this call or by
/// anything it does.
pub fn supplied_type_matches(env: &Environment, kernel: &Expr, supplied: &Expr) -> bool {
    same_up_to_binders(kernel, supplied)
        || type_matches(&mut TypeChecker::new(env), kernel, supplied)
}

/// The derived recursor `d` as it is installed for the supplied declaration
/// `rv`: the kernel's type, rules, counts and `k` flag, under the supplied
/// universe parameter **names** (they are not semantic — a constant's
/// universe parameters are instantiated by position) and with the supplied
/// `is_unsafe` flag. `None` when the two have a different number of universe
/// parameters, or the supplied names are not distinct.
pub(crate) fn derived_under_supplied_names(
    d: &RecursorVal,
    rv: &RecursorVal,
) -> Option<RecursorVal> {
    let from = &d.common.level_params;
    let to = &rv.common.level_params;
    if from.len() != to.len() {
        return None;
    }
    if to.iter().enumerate().any(|(i, n)| to[..i].contains(n)) {
        return None;
    }
    let mut installed = d.clone();
    installed.is_unsafe = rv.is_unsafe;
    if from != to {
        let levels: Vec<Level> = to.iter().cloned().map(Level::param).collect();
        installed.common.ty = instantiate_type_lparams(&d.common.ty, from, &levels);
        installed.rules = d
            .rules
            .iter()
            .map(|rule| RecursorRule {
                ctor: rule.ctor.clone(),
                nfields: rule.nfields,
                rhs: instantiate_type_lparams(&rule.rhs, from, &levels),
            })
            .collect();
        installed.common.level_params = to.clone();
    }
    Some(installed)
}

/// What the comparison as written says about a supplied recursor.
enum AsWritten {
    /// Every count, the `k` flag, every rule's constructor and field count,
    /// the type and every right-hand side are the kernel's.
    Same,
    /// A count, the `k` flag, the number of rules, or a rule's constructor
    /// or field count is not the kernel's: no term needs to be looked at.
    Differs,
    /// Everything but the terms is the kernel's, and a term is not the
    /// kernel's as written.
    Unsettled,
}

/// The comparison of step 1: nothing is reduced, nothing is type-checked.
/// Rule `i` of the derivation is the rule of the type's `i`-th constructor;
/// the supplied rule at the same place must name that constructor (so no
/// rule is missing, none is there twice, none names a constructor of another
/// type, and no two are exchanged).
fn as_written(d: &RecursorVal, rv: &RecursorVal) -> AsWritten {
    if d.common.name != rv.common.name
        || d.common.level_params != rv.common.level_params
        || d.num_params != rv.num_params
        || d.num_indices != rv.num_indices
        || d.num_motives != rv.num_motives
        || d.num_minors != rv.num_minors
        || d.k != rv.k
        || d.rules.len() != rv.rules.len()
    {
        return AsWritten::Differs;
    }
    if d.rules
        .iter()
        .zip(&rv.rules)
        .any(|(drule, srule)| srule.ctor != drule.ctor || srule.nfields != drule.nfields)
    {
        return AsWritten::Differs;
    }
    let same = same_up_to_binders(&d.common.ty, &rv.common.ty)
        && d.rules
            .iter()
            .zip(&rv.rules)
            .all(|(drule, srule)| same_up_to_binders(&drule.rhs, &srule.rhs));
    if same {
        AsWritten::Same
    } else {
        AsWritten::Unsettled
    }
}

/// Whether the supplied recursor `rv` is the derived recursor `d` — `d`
/// already under the supplied universe parameter names
/// ([`derived_under_supplied_names`]); the caller pairs the two by name.
///
/// The same counts and `k` flag, the same type, and the same rules — rule by
/// rule in the order of the constructors (the order Lean's kernel gives
/// them), each for the same constructor with the same number of fields and
/// the same right-hand side. "The same" for a term is the module's rule:
/// the same as written, or — type-checked first — definitionally equal.
///
/// The checked step needs the family the recursor belongs to: `family` is
/// the kernel's own constants of it (its types, constructors and
/// recursors, in groups); those `env` does not hold yet are staged into a
/// private copy of `env`, which is dropped afterwards. That copy is made
/// only when a term is not the kernel's as written.
pub(crate) fn recursor_matches(
    env: &Environment,
    family: &[&[ConstantInfo]],
    d: &RecursorVal,
    rv: &RecursorVal,
) -> bool {
    match as_written(d, rv) {
        AsWritten::Same => true,
        AsWritten::Differs => false,
        AsWritten::Unsettled => {
            let mut work = env.clone();
            // The recursor under comparison first, so that it is the one of
            // its name; then what `env` does not hold of the family.
            let staged = std::iter::once(ConstantInfo::Recursor(d.clone()))
                .chain(family.iter().flat_map(|group| group.iter().cloned()));
            for ci in staged {
                if !work.contains(ci.name()) && work.add_constant(ci).is_err() {
                    return false;
                }
            }
            let mut tc = TypeChecker::new(&work);
            type_matches(&mut tc, &d.common.ty, &rv.common.ty)
                && d.rules
                    .iter()
                    .zip(&rv.rules)
                    .all(|(drule, srule)| value_matches(&mut tc, &drule.rhs, &srule.rhs))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BinderInfo, Name};

    fn sort(n: u32) -> Expr {
        let mut l = Level::zero();
        for _ in 0..n {
            l = Level::succ(l);
        }
        Expr::Sort(l)
    }

    fn pi(bi: BinderInfo, name: &str, dom: Expr, body: Expr) -> Expr {
        Expr::Pi(bi, Name::str(name), Node::new(dom), Node::new(body))
    }

    #[test]
    fn binder_names_and_binder_information_are_not_compared() {
        let a = pi(BinderInfo::Default, "x", sort(1), Expr::BVar(0));
        let b = pi(BinderInfo::Implicit, "y", sort(1), Expr::BVar(0));
        assert!(same_up_to_binders(&a, &b));
    }

    #[test]
    fn levels_are_compared_as_written() {
        // `Sort (max 0 0)` is equivalent to `Sort 0` and is not the same term.
        let m = Expr::Sort(Level::max(Level::zero(), Level::zero()));
        assert!(!same_up_to_binders(&m, &sort(0)));
        let c0 = Expr::Const(Name::str("C"), vec![Level::zero()]);
        let c1 = Expr::Const(Name::str("C"), vec![Level::succ(Level::zero())]);
        assert!(!same_up_to_binders(&c0, &c1));
        assert!(same_up_to_binders(&c0, &c0.clone()));
    }

    #[test]
    fn a_binder_is_not_a_lambda_and_a_shape_is_compared_throughout() {
        let p = pi(BinderInfo::Default, "x", sort(1), Expr::BVar(0));
        let l = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Node::new(sort(1)),
            Node::new(Expr::BVar(0)),
        );
        assert!(!same_up_to_binders(&p, &l));
        let q = pi(BinderInfo::Default, "x", sort(1), sort(0));
        assert!(!same_up_to_binders(&p, &q));
    }

    #[test]
    fn a_supplied_type_that_is_not_closed_is_never_type_checked() {
        let env = Environment::new();
        // `Sort 1` against a loose bound variable: not the same as written,
        // and not a term the checker may be given.
        assert!(!supplied_type_matches(&env, &sort(1), &Expr::BVar(0)));
        // `(α : Type) → α` against the same with a FREE variable for `α`:
        // the checker mints free variables by number for the binders it
        // opens, so a term that arrives with one in it could pass for the
        // bound variable. It is refused before the checker sees it.
        let kernel = pi(BinderInfo::Default, "α", sort(1), Expr::BVar(0));
        for id in 0..4 {
            let free = pi(
                BinderInfo::Default,
                "α",
                sort(1),
                Expr::FVar(crate::FVarId(id)),
            );
            assert!(!supplied_type_matches(&env, &kernel, &free), "FVar {id}");
        }
        // The same type, written with other binder names, is the kernel's.
        let a = pi(BinderInfo::Default, "x", sort(1), sort(0));
        let b = pi(BinderInfo::Default, "α", sort(1), sort(0));
        assert!(supplied_type_matches(&env, &a, &b));
    }

    /// A recursor `T.rec` with the given universe parameter names, whose
    /// type and one rule mention the first of them.
    fn recursor(level_params: &[&str]) -> RecursorVal {
        let first = Level::param(Name::str(level_params[0]));
        RecursorVal {
            common: crate::declaration::ConstantVal {
                name: Name::str("T.rec"),
                level_params: level_params.iter().map(|p| Name::str(*p)).collect(),
                ty: Expr::Sort(first.clone()),
            },
            all: vec![Name::str("T")],
            num_params: 0,
            num_indices: 0,
            num_motives: 1,
            num_minors: 1,
            rules: vec![RecursorRule {
                ctor: Name::str("T.mk"),
                nfields: 0,
                rhs: Expr::Sort(first),
            }],
            k: false,
            is_unsafe: false,
        }
    }

    #[test]
    fn the_derived_recursor_is_put_under_the_supplied_universe_parameter_names() {
        let derived = recursor(&["u", "w"]);
        // The same names: the derivation as it is, with the supplied flag.
        let mut supplied = recursor(&["u", "w"]);
        supplied.is_unsafe = true;
        let installed = derived_under_supplied_names(&derived, &supplied);
        assert_eq!(
            installed,
            Some(RecursorVal {
                is_unsafe: true,
                ..derived.clone()
            })
        );
        // Other names: the derivation with its parameters renamed by position.
        let installed = derived_under_supplied_names(&derived, &recursor(&["v", "w"]));
        assert_eq!(installed, Some(recursor(&["v", "w"])));
        // Another number of names, or a name twice: not a declaration of it.
        assert_eq!(
            derived_under_supplied_names(&derived, &recursor(&["u"])),
            None
        );
        assert_eq!(
            derived_under_supplied_names(&derived, &recursor(&["w", "w"])),
            None
        );
    }
}
