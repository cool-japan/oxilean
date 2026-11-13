//! Which inductive type a projection may name, and how the type of its
//! operand must look.
//!
//! `Proj(S, i, e)` reads field `i` of `e`. Lean 4's kernel (`infer_proj`)
//! asks of `S` that it is an inductive type with **exactly one
//! constructor**, and nothing else: `S` may be recursive (an ordinary
//! recursive `structure`), may have indices, may sit in a mutual block and
//! may have nested occurrences. The type of `e`, in weak head normal form,
//! must be `S` applied to as many arguments as `S` has parameters **and
//! indices**. The constructor's telescope is then instantiated with the
//! parameters alone — an index is an argument of the type and never an
//! argument of the constructor — and field `i` is the `i`-th binder after
//! them.
//!
//! This is not the condition of structure eta, of unit-like equality or of
//! eta inside iota. Those keep [`Environment::is_structure_like`]: one
//! constructor, no indices, not recursive. `s = S.mk s.0 s.1` holds by
//! definition only for such an `S`, while `s.0` is typed, and `(S.mk a b).0`
//! reduces to `a`, for every one-constructor `S`. (Lean's library states
//! the narrower condition as `Lean.isNonRecStructure`, whose documentation
//! says it "must be in sync with `is_non_rec_structure()`"; that is the
//! library's comment in the toolchain's `Lean/Structure.lean`, not a
//! reading of the kernel's C++ source, which the toolchain does not ship.
//! Both rules were measured against Lean's kernel, not read.)
//!
//! Both inference paths of this kernel — [`super::types::TypeChecker`]'s
//! `infer_proj` and the def-eq checker's `quick_proj_type` — ask the two
//! questions through the two functions of this module, so the rule is
//! stated once. The reducer needs neither: it only replaces a projection of
//! a constructor application of `S` by the field, whatever `S` is.

use crate::declaration::{ConstructorVal, InductiveVal};
use crate::error::KernelError;
use crate::{Environment, Expr, Name};

/// The inductive type and the constructor that `Proj(struct_name, idx, _)`
/// reads: `struct_name` must be an inductive type with exactly one
/// constructor, and `idx` one of that constructor's fields. Recursion,
/// indices, mutual blocks and nested occurrences are all admitted.
#[allow(clippy::result_large_err)]
pub(crate) fn proj_target<'env>(
    env: &'env Environment,
    struct_name: &Name,
    idx: u32,
) -> Result<(&'env InductiveVal, &'env ConstructorVal), KernelError> {
    let ind_val = env.get_inductive_val(struct_name).ok_or_else(|| {
        KernelError::Other(format!(
            "invalid projection: {} is not an inductive type",
            struct_name
        ))
    })?;
    let [ctor_name] = ind_val.ctors.as_slice() else {
        return Err(KernelError::Other(format!(
            "invalid projection: {} has {} constructors; \
             a projection reads an inductive type with exactly one",
            struct_name,
            ind_val.ctors.len()
        )));
    };
    let ctor_val = env
        .get_constructor_val(ctor_name)
        .ok_or_else(|| KernelError::Other(format!("constructor not found: {}", ctor_name)))?;
    if idx >= ctor_val.num_fields {
        return Err(KernelError::Other(format!(
            "field index {} out of range for {} (has {} fields)",
            idx, struct_name, ctor_val.num_fields
        )));
    }
    Ok((ind_val, ctor_val))
}

/// The parameters among `type_args`, the arguments of a projection
/// operand's type `S ps is`: the first `num_params` of them, when there are
/// exactly as many arguments as `S` has parameters and indices; `None`
/// otherwise. Only these instantiate the constructor's telescope; the whole
/// of `type_args` is what the sort of `S ps is` is read off.
pub(crate) fn proj_type_params<'a>(
    ind_val: &InductiveVal,
    type_args: &'a [Expr],
) -> Option<&'a [Expr]> {
    let num_params = ind_val.num_params as usize;
    if type_args.len() != num_params + ind_val.num_indices as usize {
        return None;
    }
    type_args.get(..num_params)
}

#[cfg(test)]
mod tests {
    use super::proj_type_params;
    use crate::declaration::{ConstantVal, InductiveVal};
    use crate::{Expr, Level, Name};

    /// An inductive type with the given numbers of parameters and indices
    /// (only those two fields are read by `proj_type_params`).
    fn family(num_params: u32, num_indices: u32) -> InductiveVal {
        InductiveVal {
            common: ConstantVal {
                name: Name::str("S"),
                level_params: vec![],
                ty: Expr::Sort(Level::succ(Level::zero())),
            },
            num_params,
            num_indices,
            all: vec![Name::str("S")],
            ctors: vec![Name::str("S.mk")],
            num_nested: 0,
            is_rec: false,
            is_unsafe: false,
            is_reflexive: false,
            is_prop: false,
        }
    }

    /// Four distinct arguments.
    fn args() -> [Expr; 4] {
        [Expr::BVar(0), Expr::BVar(1), Expr::BVar(2), Expr::BVar(3)]
    }

    #[test]
    fn the_operand_type_carries_exactly_the_parameters_and_the_indices() {
        let a = args();
        // Two parameters and one index: exactly three arguments, and the
        // parameters are the first two.
        let s = family(2, 1);
        assert_eq!(proj_type_params(&s, &a[..3]), Some(&a[..2]));
        // Too few: the index missing, a parameter missing, none at all.
        assert_eq!(proj_type_params(&s, &a[..2]), None);
        assert_eq!(proj_type_params(&s, &a[..1]), None);
        assert_eq!(proj_type_params(&s, &a[..0]), None);
        // Too many.
        assert_eq!(proj_type_params(&s, &a[..4]), None);
    }

    #[test]
    fn without_an_index_the_arguments_are_the_parameters() {
        let a = args();
        let s = family(2, 0);
        assert_eq!(proj_type_params(&s, &a[..2]), Some(&a[..2]));
        assert_eq!(proj_type_params(&s, &a[..1]), None);
        assert_eq!(proj_type_params(&s, &a[..3]), None);
    }

    #[test]
    fn without_a_parameter_no_argument_instantiates_the_constructor() {
        let a = args();
        // Indices only: all of them must be there, and none is a parameter.
        let s = family(0, 2);
        assert_eq!(proj_type_params(&s, &a[..2]), Some(&a[..0]));
        assert_eq!(proj_type_params(&s, &a[..1]), None);
        assert_eq!(proj_type_params(&s, &a[..3]), None);
        // Neither: no argument at all.
        let unit = family(0, 0);
        assert_eq!(proj_type_params(&unit, &a[..0]), Some(&a[..0]));
        assert_eq!(proj_type_params(&unit, &a[..1]), None);
    }
}
