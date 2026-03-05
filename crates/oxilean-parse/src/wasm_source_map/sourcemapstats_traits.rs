//! # SourceMapStats - Trait Implementations
//!
//! This module contains trait implementations for `SourceMapStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::SourceMapStats;

impl std::fmt::Display for SourceMapStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SourceMapStats {{ mappings: {}, sources: {}, lines: {}, size: {} }}",
            self.mapping_count, self.source_count, self.line_count, self.encoded_size
        )
    }
}
