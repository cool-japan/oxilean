//! # DiscoveryOptions - Trait Implementations
//!
//! This module contains trait implementations for `DiscoveryOptions`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::fmt;

use super::types::DiscoveryOptions;

impl Default for DiscoveryOptions {
    fn default() -> Self {
        Self {
            extensions: vec!["lean".to_string(), "ox".to_string()],
            exclude_dirs: {
                let mut set = HashSet::new();
                set.insert("build".to_string());
                set.insert(".git".to_string());
                set.insert("target".to_string());
                set.insert(".oxilean".to_string());
                set
            },
            auto_namespace: true,
        }
    }
}
