//! # VyperDecorator - Trait Implementations
//!
//! This module contains trait implementations for `VyperDecorator`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::VyperDecorator;
use std::fmt;

impl fmt::Display for VyperDecorator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VyperDecorator::External => write!(f, "@external"),
            VyperDecorator::Internal => write!(f, "@internal"),
            VyperDecorator::View => write!(f, "@view"),
            VyperDecorator::Pure => write!(f, "@pure"),
            VyperDecorator::Payable => write!(f, "@payable"),
            VyperDecorator::NonReentrant(key) => write!(f, "@nonreentrant(\"{}\")", key),
            VyperDecorator::Deploy => write!(f, "@deploy"),
        }
    }
}
