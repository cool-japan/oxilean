//! # TypeSignature - Trait Implementations
//!
//! This module contains trait implementations for `TypeSignature`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TypeSignature;

impl std::fmt::Display for TypeSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.args.is_empty() {
            write!(f, "{}", self.head)
        } else if self.head == "->" {
            write!(f, "{} -> {}", self.args[0], self.args[1])
        } else {
            let args: Vec<String> = self.args.iter().map(|a| a.to_string()).collect();
            write!(f, "{} {}", self.head, args.join(" "))
        }
    }
}
