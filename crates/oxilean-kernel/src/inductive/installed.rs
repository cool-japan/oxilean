//! The guard of an install: every constant that a term the kernel derived
//! names resolves, and to the constant the derivation meant.
//!
//! Where the kernel derives a term itself, that term is what goes into the
//! environment, and a declaration from outside is only compared with it
//! ([`super::supplied`]): a recursor's type and its rules, on the ordinary
//! and on the nested path (the auxiliary recursors of a nested family
//! among them), and the types of the quotient primitives. A derived term
//! names other constants. A recursor's rule calls the recursors of the
//! sibling types of a mutual or nested family **by name** — the names the
//! derivation gave them — and the derivation also names the family's types,
//! its constructors and whatever the declaration's own types mention.
//!
//! Those names are a promise about the environment: iota reduction later
//! unfolds the rule and runs into the constant of each name. If a name is
//! not installed, a later declaration can define it — with another type —
//! and the reduct of an installed rule is then ill typed. That is a closed
//! proof of `False` from no axiom: a mutual bundle whose records named one
//! recursor `A.rec` (Lean's name, the component `rec` under `A`) and the
//! other with the one flat component `"C.rec"` was derived in two naming
//! styles, so the installed `C.rec`'s rule called a hierarchical `A.rec`
//! that was never installed, and the file then defined one.
//!
//! So an install is guarded, for each term the kernel derived and installs:
//!
//! * every constant it names **resolves** in the environment as it stands
//!   after the install — a member of the bundle, or a constant that was
//!   already present;
//! * a name the derivation gives a constant **of the declaration** — a type
//!   of the family, the recursor of one (`T.rec`, or the flat `"T.rec"` of
//!   the kernel's own tests, in the naming of the recursor that mentions it,
//!   as [`super::derive`] names the family), an auxiliary recursor
//!   `<first type>.rec_<k>` of a nested family — resolves to that constant:
//!   a type or a recursor of the same family, never to a constant of that
//!   name that something else declared;
//! * the auxiliary types of a nested derivation (`<first type>._nested_<k>`
//!   and their recursors) are never installed, so a term that still names
//!   one is refused.
//!
//! A name that fails one of these refuses the whole bundle, and nothing of
//! it is left in the environment. The guard reads the terms once and copies
//! nothing: it runs on the bundle as installed by
//! [`crate::check_constant_infos_atomic`] (the ordinary path) and before the
//! constants are added by [`add_derived_family`] (the nested path) -- the
//! two paths of the export replay -- and `Environment::add_quot` holds the
//! canonical types of the quotient primitives to the same rule. It does not
//! run in [`crate::add_inductive_family`], which adds a derivation one
//! constant at a time, neither atomically nor guarded (see its
//! documentation); the export replay does not call it.

use crate::declaration::ConstantInfo;
use crate::expr_util::find_mentioned_name;
use crate::name::NameView;
use crate::{Environment, KernelError, Name};

/// Which members of a bundle are terms the kernel derived.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum DerivedTerms {
    /// Only the recursors (the ordinary path: the types and constructors
    /// are the declaration's own, each type-checked when it was added).
    Recursors,
    /// Every member (the nested path: the whole family that is added is the
    /// kernel's derivation, held identical to the declaration).
    All,
}

/// The name a derivation gives the recursor of the type `type_name`: Lean's
/// `T.rec` (the component `rec` under `T`) or, in the kernel's own tests,
/// the one flat component `"T.rec"`.
pub(crate) fn recursor_name(type_name: &Name, flat: bool) -> Name {
    if flat {
        Name::str(format!("{type_name}.rec"))
    } else {
        Name::mk_str(type_name.clone(), "rec")
    }
}

/// Whether a recursor's name is in the kernel's flat style (one component).
pub(crate) fn is_flat_style(recursor: &Name) -> bool {
    recursor.depth() <= 1
}

/// `name` as Lean writes it: components joined by `.`, and a component that
/// itself contains a `.` set in `«»` — so that the one flat component
/// `"T.rec"` reads `«T.rec»` and the hierarchical name reads `T.rec`.
pub fn escaped_name(name: &Name) -> String {
    match name.view() {
        NameView::Anonymous => "[anonymous]".to_string(),
        NameView::Str(parent, s) => {
            let part = if s.contains('.') {
                format!("«{s}»")
            } else {
                s.to_string()
            };
            if parent.is_anonymous() {
                part
            } else {
                format!("{}.{part}", escaped_name(parent))
            }
        }
        NameView::Num(parent, n) => {
            if parent.is_anonymous() {
                n.to_string()
            } else {
                format!("{}.{n}", escaped_name(parent))
            }
        }
    }
}

/// The names a derivation gives the constants of the family `family` when
/// the constant that mentions them is named `owner`, with what each must
/// resolve to.
struct Meant<'a> {
    family: &'a [Name],
    recursors: Vec<Name>,
    auxiliary: Vec<Name>,
}

impl<'a> Meant<'a> {
    fn new(family: &'a [Name], owner: &Name, recursor_owner: bool, num_nested: u32) -> Self {
        let flat = recursor_owner && is_flat_style(owner);
        let mut recursors: Vec<Name> = family.iter().map(|t| recursor_name(t, flat)).collect();
        let mut auxiliary = Vec::new();
        if let Some(first) = family.first() {
            for k in 0..num_nested {
                recursors.push(Name::mk_str(first.clone(), format!("rec_{}", k + 1)));
                let aux_type = Name::mk_str(first.clone(), format!("_nested_{k}"));
                auxiliary.push(Name::mk_str(aux_type.clone(), "rec"));
                auxiliary.push(aux_type);
            }
        }
        Meant {
            family,
            recursors,
            auxiliary,
        }
    }
}

/// The family (`all`) a member of a bundle belongs to (a constructor's is
/// its type's).
fn family_of<'a>(
    ci: &'a ConstantInfo,
    resolve: &dyn Fn(&Name) -> Option<&'a ConstantInfo>,
) -> Option<&'a [Name]> {
    match ci {
        ConstantInfo::Inductive(iv) => Some(&iv.all),
        ConstantInfo::Recursor(rv) => Some(&rv.all),
        ConstantInfo::Constructor(cv) => match resolve(&cv.induct) {
            Some(ConstantInfo::Inductive(iv)) => Some(&iv.all),
            _ => None,
        },
        _ => None,
    }
}

/// Check, for every term of `bundle` that the kernel derived (`which`), that
/// each constant it names resolves — a member of `bundle` or a constant of
/// `env` — and that a name the derivation gives a constant of the
/// declaration resolves to that constant (see the module documentation).
///
/// `env` may hold the bundle already (the ordinary path checks after the
/// install, and undoes it on an error) or not yet (the nested path checks
/// before it adds anything): members of `bundle` are looked up in `bundle`
/// first either way.
#[allow(clippy::result_large_err)]
pub(crate) fn check_derived_references<'b>(
    env: &'b Environment,
    bundle: &[&'b ConstantInfo],
    which: DerivedTerms,
) -> Result<(), KernelError> {
    let resolve = |n: &Name| -> Option<&'b ConstantInfo> {
        bundle
            .iter()
            .copied()
            .find(|ci| ci.name() == n)
            .or_else(|| env.find(n))
    };
    for ci in bundle.iter().copied() {
        let is_recursor = matches!(ci, ConstantInfo::Recursor(_));
        if which == DerivedTerms::Recursors && !is_recursor {
            continue;
        }
        let Some(family) = family_of(ci, &resolve) else {
            continue;
        };
        let num_nested = match family.first().and_then(&resolve) {
            Some(ConstantInfo::Inductive(iv)) => iv.num_nested,
            _ => 0,
        };
        let meant = Meant::new(family, ci.name(), is_recursor, num_nested);
        let mut terms = vec![ci.ty()];
        if let ConstantInfo::Recursor(rv) = ci {
            terms.extend(rv.rules.iter().map(|r| &r.rhs));
        }
        for term in terms {
            let mut why: Option<String> = None;
            let hit = find_mentioned_name(term, |c| match judge_name(c, &meant, &resolve) {
                Ok(()) => false,
                Err(reason) => {
                    why = Some(reason);
                    true
                }
            });
            if let Some(c) = hit {
                return Err(refused(ci, c, &why.unwrap_or_default()));
            }
        }
        if let ConstantInfo::Recursor(rv) = ci {
            for rule in &rv.rules {
                if !matches!(resolve(&rule.ctor), Some(ConstantInfo::Constructor(_))) {
                    return Err(refused(
                        ci,
                        &rule.ctor,
                        "a rule is keyed by it, and it is not a constructor in the environment \
                         after the install",
                    ));
                }
            }
        }
    }
    Ok(())
}

/// Why the name `c` in a term of a constant whose family is described by
/// `meant` does not stand for the constant the derivation meant, if it
/// does not.
fn judge_name<'a>(
    c: &Name,
    meant: &Meant<'_>,
    resolve: &dyn Fn(&Name) -> Option<&'a ConstantInfo>,
) -> Result<(), String> {
    if meant.auxiliary.contains(c) {
        return Err(
            "an auxiliary constant of the nested derivation, which is never installed".into(),
        );
    }
    let Some(found) = resolve(c) else {
        return Err("it is not in the environment after the install".into());
    };
    let of_family = |all: &[Name]| all == meant.family;
    if meant.recursors.contains(c) {
        return match found {
            ConstantInfo::Recursor(rv) if of_family(&rv.all) => Ok(()),
            _ => Err(
                "the derivation names a recursor of its family so, and the constant of \
                      that name is not one"
                    .into(),
            ),
        };
    }
    if meant.family.contains(c) {
        return match found {
            ConstantInfo::Inductive(iv) if of_family(&iv.all) => Ok(()),
            _ => Err(
                "the derivation names a type of its family so, and the constant of that \
                      name is not one"
                    .into(),
            ),
        };
    }
    Ok(())
}

fn refused(ci: &ConstantInfo, c: &Name, why: &str) -> KernelError {
    let what = match ci {
        ConstantInfo::Recursor(_) => "recursor",
        ConstantInfo::Inductive(_) => "inductive type",
        ConstantInfo::Constructor(_) => "constructor",
        _ => "constant",
    };
    KernelError::InvalidRecursor(format!(
        "the {what} '{}' the kernel derived names '{}': {why}; the bundle is not installed",
        escaped_name(ci.name()),
        escaped_name(c)
    ))
}

/// Add a family the kernel derived — the nested path's install — as one
/// group, after the guard: every constant that a term of the family names
/// resolves in the environment as it will stand after the install, and to
/// the constant the derivation meant (see the module documentation). On a
/// refusal nothing is added. A name of the group that is already declared,
/// or declared twice in it, is refused as by [`Environment::add_constants`]
/// (`KernelError::Other` with that message).
#[allow(clippy::result_large_err)]
pub fn add_derived_family(
    env: &mut Environment,
    group: Vec<ConstantInfo>,
) -> Result<(), KernelError> {
    {
        let bundle: Vec<&ConstantInfo> = group.iter().collect();
        check_derived_references(env, &bundle, DerivedTerms::All)?;
    }
    env.add_constants(group)
        .map_err(|e| KernelError::Other(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escaped_name_sets_a_dotted_component_apart() {
        let flat = Name::str("T.rec");
        let hierarchical = Name::mk_str(Name::str("T"), "rec");
        assert_eq!(escaped_name(&flat), "«T.rec»");
        assert_eq!(escaped_name(&hierarchical), "T.rec");
        assert_eq!(flat.to_string(), hierarchical.to_string());
        assert_eq!(escaped_name(&Name::mk_num(Name::str("x"), 3)), "x.3");
    }

    #[test]
    fn the_names_meant_follow_the_owner_and_the_nesting() {
        let family = vec![Name::str("A"), Name::str("C")];
        let flat = Meant::new(&family, &Name::str("A.rec"), true, 0);
        assert_eq!(
            flat.recursors,
            vec![Name::str("A.rec"), Name::str("C.rec")],
            "a flat recursor's derivation names its siblings flat"
        );
        let hierarchical = Meant::new(&family, &recursor_name(&Name::str("C"), false), true, 0);
        assert_eq!(
            hierarchical.recursors,
            vec![
                recursor_name(&Name::str("A"), false),
                recursor_name(&Name::str("C"), false)
            ]
        );
        let nested = Meant::new(&family[..1], &recursor_name(&family[0], false), true, 2);
        assert_eq!(nested.recursors.len(), 3);
        assert_eq!(nested.recursors[2].to_string(), "A.rec_2");
        assert_eq!(nested.auxiliary.len(), 4);
    }
}
