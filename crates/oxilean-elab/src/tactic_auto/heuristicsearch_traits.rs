//! # HeuristicSearch - Trait Implementations
//!
//! This module contains trait implementations for `HeuristicSearch`.
//!
//! ## Implemented Traits
//!
//! - `ProofSearch`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::tactic::{Goal, TacticResult, TacticState};
use std::fmt;

use super::functions::ProofSearch;
use super::types::{AutoConfig, HeuristicSearch, SearchResult};

impl ProofSearch for HeuristicSearch {
    fn search_name(&self) -> &'static str {
        "heuristic"
    }
    fn search(&self, _goal: &Goal, _config: &AutoConfig) -> SearchResult {
        SearchResult::Failed
    }
}
