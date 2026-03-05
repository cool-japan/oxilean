//! # RegistryError - Trait Implementations
//!
//! This module contains trait implementations for `RegistryError`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::manifest::{Dependency, ManifestError, Version, VersionConstraint};

use super::types::RegistryError;
use std::fmt;

impl fmt::Display for RegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PackageNotFound(name) => write!(f, "package not found: {}", name),
            Self::VersionNotFound { package, version } => {
                write!(f, "version {} not found for package {}", version, package)
            }
            Self::AuthRequired => write!(f, "authentication required"),
            Self::AuthFailed(msg) => write!(f, "authentication failed: {}", msg),
            Self::NetworkError(msg) => write!(f, "network error: {}", msg),
            Self::RateLimited { retry_after } => {
                write!(f, "rate limited, retry after {} seconds", retry_after)
            }
            Self::InvalidPackage(msg) => write!(f, "invalid package: {}", msg),
            Self::ChecksumMismatch { expected, actual } => {
                write!(
                    f,
                    "checksum mismatch: expected {}, got {}",
                    expected, actual
                )
            }
            Self::IoError(msg) => write!(f, "IO error: {}", msg),
            Self::ManifestError(e) => write!(f, "manifest error: {}", e),
            Self::VersionExists { package, version } => {
                write!(f, "version {} already exists for {}", version, package)
            }
            Self::NameReserved(name) => write!(f, "package name is reserved: {}", name),
        }
    }
}
