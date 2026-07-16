use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Top-level manifest structure for `oxilake.toml`.
#[derive(Debug, Deserialize, Serialize)]
pub struct OxilakeManifest {
    pub package: PackageSection,
    #[serde(default)]
    pub dependencies: HashMap<String, DepSpec>,
}

/// The `[package]` section of `oxilake.toml`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PackageSection {
    pub name: String,
    pub version: String,
    #[serde(default = "default_lean_version")]
    pub lean_version: String,
    pub description: Option<String>,
}

fn default_lean_version() -> String {
    "0.1".to_string()
}

/// A dependency specification — either a plain version string or a local path.
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum DepSpec {
    Version(String),
    Path { path: String },
}

impl OxilakeManifest {
    /// Load and parse a manifest from the given path.
    pub fn load(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("reading manifest {}", path.display()))?;
        toml::from_str(&content).with_context(|| format!("parsing manifest {}", path.display()))
    }

    /// Serialize and write the manifest to the given path.
    pub fn save(&self, path: &Path) -> Result<()> {
        let content = toml::to_string_pretty(self).context("serializing manifest")?;
        std::fs::write(path, content)
            .with_context(|| format!("writing manifest {}", path.display()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_manifest_round_trip() {
        let dir = env::temp_dir().join("oxilake_test_manifest");
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("oxilake.toml");

        let manifest = OxilakeManifest {
            package: PackageSection {
                name: "test-pkg".to_string(),
                version: "0.1.0".to_string(),
                lean_version: "0.1".to_string(),
                description: Some("A test package".to_string()),
            },
            dependencies: Default::default(),
        };
        manifest.save(&path).unwrap();
        let loaded = OxilakeManifest::load(&path).unwrap();
        assert_eq!(loaded.package.name, "test-pkg");
        assert_eq!(loaded.package.version, "0.1.0");
        std::fs::remove_dir_all(&dir).ok();
    }
}
