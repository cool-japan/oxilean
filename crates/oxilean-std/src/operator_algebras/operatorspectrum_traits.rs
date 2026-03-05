//! # OperatorSpectrum - Trait Implementations
//!
//! This module contains trait implementations for `OperatorSpectrum`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::OperatorSpectrum;
use std::fmt;

impl std::fmt::Display for OperatorSpectrum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Spectrum[{}](r={:.3}, self_adj={}, positive={})",
            self.operator_name, self.spectral_radius, self.is_self_adjoint, self.is_positive
        )
    }
}
