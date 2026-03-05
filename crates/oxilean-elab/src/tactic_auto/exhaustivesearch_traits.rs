//! # ExhaustiveSearch - Trait Implementations
//!
//! This module contains trait implementations for `ExhaustiveSearch`.
//!
//! ## Implemented Traits
//!
//! - `ProofSearch`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::tactic::{Goal, TacticResult, TacticState};
use std::fmt;

use super::functions::ProofSearch;
use super::types::{AutoConfig, ExhaustiveSearch, SearchResult};

impl ProofSearch for ExhaustiveSearch {
    fn search_name(&self) -> &'static str {
        "exhaustive"
    }
    fn search(&self, _goal: &Goal, _config: &AutoConfig) -> SearchResult {
        SearchResult::Failed
    }
}
