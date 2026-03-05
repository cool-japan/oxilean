//! # CandidateSource - Trait Implementations
//!
//! This module contains trait implementations for `CandidateSource`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CandidateSource;

impl std::fmt::Display for CandidateSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CandidateSource::LocalHyp(n) => write!(f, "hyp:{}", n),
            CandidateSource::ProvidedLemma(i) => write!(f, "lemma[{}]", i),
            CandidateSource::EnvironmentDecl(n) => write!(f, "env:{}", n),
        }
    }
}
