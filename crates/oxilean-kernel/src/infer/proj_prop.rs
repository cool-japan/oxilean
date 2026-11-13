//! No data out of a proof: the side condition on the type of a projection.
//!
//! `Proj(S, i, e)` reads field `i` of `e : S ps`. When `S ps` is a
//! proposition, `e` is a proof, and any two proofs of one proposition are
//! definitionally equal (proof irrelevance). A projection that returned DATA
//! from a proof would therefore be a function that is constant by definition
//! and still computes on constructors: `h.0` out of `h : ∃ n : Nat, True`
//! gives `0 = 1` from the two witnesses `⟨0, _⟩` and `⟨1, _⟩`, and `False`
//! from that, without an axiom.
//!
//! Lean 4's kernel closes this in the inference of a projection's type
//! (`infer_proj`): when the structure's type is a proposition,
//!
//! * the projected field's type must be a proposition, and
//! * so must the type of every earlier field that a later field's type
//!   depends on (the projection of that earlier field appears in the type
//!   being computed).
//!
//! A structure in `Type`, a proposition's proof fields (`And`'s both, and
//! both fields of an existential over a proposition), and a proof field
//! after a data field it does not depend on are untouched.
//!
//! The two inference paths of this kernel both read a projection's type off
//! the constructor's telescope — [`TypeChecker`]'s `infer_proj_field_type`
//! and the def-eq checker's `quick_proj_type` — and both ask the two
//! questions through the functions of this module.
//!
//! "Is a proposition" is decided up to universe equivalence
//! ([`crate::level::is_equivalent`] with `0`), as proof irrelevance decides
//! it in the def-eq checker: the two must agree on what a proof is.

use super::types::TypeChecker;
use crate::declaration::InductiveVal;
use crate::error::KernelError;
use crate::instantiate::instantiate_type_lparams;
use crate::subst::instantiate;
use crate::{Expr, Level};

/// Whether `sort` — the weak head normal form of the type of a type — says
/// that type is a proposition: `Some(true)` for `Sort l` with `l` equivalent
/// to `0`, `Some(false)` for any other `Sort`, and `None` when `sort` is not
/// a `Sort` at all (a reduction cut short, or an ill-formed term): the
/// question is then undecided and the caller must not read it as "no".
pub(crate) fn is_prop_sort(sort: &Expr) -> Option<bool> {
    match sort {
        Expr::Sort(l) => Some(crate::level::is_equivalent(l, &Level::zero())),
        _ => None,
    }
}

/// The sort of the structure type `I.{levels} args`, read off the declared
/// type of the inductive `I` (what infer-only inference of the application
/// computes): the declared type's universe parameters are instantiated with
/// `levels`, its leading binders with `args`, and what is left is reduced
/// with `whnf`. `args` are ALL the arguments of the operand's type: the
/// parameters and, for a family with indices, the indices after them (a
/// projection may name one; see `proj_target`). What is left is then a
/// `Sort` for a well-formed `I`; anything else — a binder, when arguments
/// are missing — is returned as it is and [`is_prop_sort`] answers `None`
/// for it.
pub(crate) fn structure_type_sort(
    ind_val: &InductiveVal,
    levels: &[Level],
    args: &[Expr],
    mut whnf: impl FnMut(&Expr) -> Expr,
) -> Expr {
    let level_params = &ind_val.common.level_params;
    let mut ty = if level_params.is_empty() || levels.is_empty() {
        ind_val.common.ty.clone()
    } else {
        instantiate_type_lparams(&ind_val.common.ty, level_params, levels)
    };
    for arg in args {
        if !matches!(ty, Expr::Pi(_, _, _, _)) {
            ty = whnf(&ty);
        }
        match ty {
            Expr::Pi(_, _, _, body) => ty = instantiate(&body, arg),
            not_a_binder => return not_a_binder,
        }
    }
    whnf(&ty)
}

impl TypeChecker<'_> {
    /// Whether the structure type `I.{levels} args` of a projection's
    /// operand is a proposition (`args`: its parameters and its indices).
    /// An undecided answer is an error, never "no": the side condition
    /// must not be skipped on it.
    #[allow(clippy::result_large_err)]
    pub(super) fn proj_struct_is_prop(
        &mut self,
        ind_val: &InductiveVal,
        levels: &[Level],
        args: &[Expr],
    ) -> Result<bool, KernelError> {
        let sort = structure_type_sort(ind_val, levels, args, |e| self.whnf(e));
        is_prop_sort(&sort).ok_or_else(|| {
            KernelError::Other(format!(
                "invalid projection: cannot tell whether {} is a proposition \
                 (its type ends in {}, not in a sort)",
                ind_val.common.name, sort
            ))
        })
    }
    /// The side condition itself, for a structure that is a proposition:
    /// `field_ty`, the type of its field `field`, must be a proposition.
    /// Asked of the projected field and of every earlier field a later
    /// field's type depends on.
    #[allow(clippy::result_large_err)]
    pub(super) fn ensure_prop_field(
        &mut self,
        ind_val: &InductiveVal,
        field: u32,
        field_ty: &Expr,
    ) -> Result<(), KernelError> {
        let sort = self.infer_type(field_ty)?;
        let sort = self.whnf(&sort);
        if is_prop_sort(&sort) == Some(true) {
            return Ok(());
        }
        Err(KernelError::Other(format!(
            "invalid projection: {} is a proposition and the type of its field {}, {}, \
             is not; data cannot be projected out of a proof",
            ind_val.common.name, field, field_ty
        )))
    }
}
