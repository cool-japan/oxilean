//! # CacheBackendKind - Trait Implementations
//!
//! This module contains trait implementations for `CacheBackendKind`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CacheBackendKind;

impl std::fmt::Display for CacheBackendKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CacheBackendKind::Http => write!(f, "http"),
            CacheBackendKind::S3 => write!(f, "s3"),
            CacheBackendKind::Gcs => write!(f, "gcs"),
            CacheBackendKind::Local => write!(f, "local"),
        }
    }
}
