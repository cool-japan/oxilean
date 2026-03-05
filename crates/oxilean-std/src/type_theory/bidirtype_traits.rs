//! # BidirType - Trait Implementations
//!
//! This module contains trait implementations for `BidirType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::BidirType;
use std::fmt;

impl std::fmt::Display for BidirType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BidirType::Type(n) => write!(f, "Type_{n}"),
            BidirType::Pi(x, dom, cod) => write!(f, "Π ({x} : {dom}). {cod}"),
            BidirType::Fun(a, b) => write!(f, "({a} → {b})"),
            BidirType::Base(n) => write!(f, "{n}"),
            BidirType::Ann(t, ty) => write!(f, "({t} : {ty})"),
        }
    }
}
