//! # RoundTripConfigExt - Trait Implementations
//!
//! This module contains trait implementations for `RoundTripConfigExt`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::RoundTripConfigExt;

impl Default for RoundTripConfigExt {
    fn default() -> Self {
        RoundTripConfigExt {
            normalise_whitespace: true,
            strip_comments: false,
            case_insensitive: false,
            allow_trailing_newlines: true,
            max_edit_distance: 0,
        }
    }
}
