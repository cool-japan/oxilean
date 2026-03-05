//! # WorkStealDeque - Trait Implementations
//!
//! This module contains trait implementations for `WorkStealDeque`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;

use super::types::WorkStealDeque;

impl<T> Default for WorkStealDeque<T> {
    fn default() -> Self {
        Self {
            items: VecDeque::new(),
        }
    }
}
