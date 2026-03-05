//! # MemoryPackageSource - Trait Implementations
//!
//! This module contains trait implementations for `MemoryPackageSource`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `PackageSource`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::manifest::{Dependency, DependencySource, Manifest, Version, VersionConstraint};

use super::functions::PackageSource;
use super::types::{MemoryPackageSource, PackageSummary};

impl Default for MemoryPackageSource {
    fn default() -> Self {
        Self::new()
    }
}

impl PackageSource for MemoryPackageSource {
    fn available_versions(&self, name: &str) -> Vec<PackageSummary> {
        self.packages.get(name).cloned().unwrap_or_default()
    }
    fn get_summary(&self, name: &str, version: &Version) -> Option<PackageSummary> {
        self.packages
            .get(name)?
            .iter()
            .find(|s| &s.version == version)
            .cloned()
    }
}
