//! The `group` tactic: decide equalities in groups.
//!
//! Normalizes both sides of a goal `a = b` as reduced words in the free
//! group and closes the goal with `rfl` when the words are equal.
//! Handles multiplication, inversion, and the group identity.

#![allow(dead_code)]
#![allow(missing_docs)]

pub mod functions;
pub mod types;

pub use functions::{
    expr_to_group_word, invert_word, reduce_word, reduce_word_with_config, tac_group,
    tac_group_with_config, words_equal,
};
pub use types::{GroupConfig, GroupLetter, GroupWord};
