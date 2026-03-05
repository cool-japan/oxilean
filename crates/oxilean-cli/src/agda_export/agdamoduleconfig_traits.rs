//! # AgdaModuleConfig - Trait Implementations
//!
//! This module contains trait implementations for `AgdaModuleConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::AgdaModuleConfig;
use std::fmt;

impl Default for AgdaModuleConfig {
    fn default() -> Self {
        Self {
            module_name: "Main".to_string(),
            imports: vec![
                "Relation.Binary.PropositionalEquality".to_string(),
                "Data.Nat".to_string(),
                "Data.List".to_string(),
            ],
            safe_mode: true,
            eta_equality: false,
            instance_arguments: true,
            exact_split: false,
            prop_name: "Set".to_string(),
            set_name: "Set₁".to_string(),
        }
    }
}
