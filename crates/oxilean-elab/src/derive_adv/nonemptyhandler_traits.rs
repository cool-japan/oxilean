//! # NonemptyHandler - Trait Implementations
//!
//! This module contains trait implementations for `NonemptyHandler`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `DeriveHandler`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Expr, FVarId, Level, Literal, Name};
use std::fmt;

use super::functions::DeriveHandler;
use super::functions::{
    build_match_body, derive_beq, derive_decidable_eq, derive_hashable, derive_inhabited,
    derive_nonempty, derive_ord, derive_repr, derive_to_string, mk_and_chain, mk_app2, mk_beq_call,
    mk_bool_lit, mk_class_app, mk_compare_call, mk_decidable_and_chain, mk_hash_combine,
    mk_lex_compare, mk_lhs_var, mk_ordering_lit, mk_repr_string, mk_rhs_var,
};
use super::types::{AdvDeriveError, AdvDeriveResult, NonemptyHandler, TypeInfoAdv};

impl Default for NonemptyHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DeriveHandler for NonemptyHandler {
    fn class_name(&self) -> Name {
        Name::str("Nonempty")
    }
    fn can_derive(&self, type_info: &TypeInfoAdv) -> bool {
        !type_info.constructors.is_empty()
    }
    fn derive(&self, type_info: &TypeInfoAdv) -> Result<AdvDeriveResult, AdvDeriveError> {
        derive_nonempty(type_info)
    }
}
