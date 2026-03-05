//! # TemplatePolyhedronDomain - Trait Implementations
//!
//! This module contains trait implementations for `TemplatePolyhedronDomain`.
//!
//! ## Implemented Traits
//!
//! - `Clone`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TemplatePolyhedronDomain;

impl Clone for TemplatePolyhedronDomain {
    fn clone(&self) -> Self {
        Self {
            dim: self.dim,
            template: self.template.clone(),
            bounds: self.bounds.clone(),
            is_bottom: self.is_bottom,
        }
    }
}
