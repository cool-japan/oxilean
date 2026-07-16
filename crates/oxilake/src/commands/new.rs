use crate::manifest::{OxilakeManifest, PackageSection};
use anyhow::{bail, Result};
use std::path::PathBuf;

/// Scaffold a new OxiLean package at the given directory.
pub fn run(name: &str, path: Option<PathBuf>) -> Result<()> {
    let dir = path.unwrap_or_else(|| PathBuf::from(name));
    if dir.exists() {
        bail!("directory '{}' already exists", dir.display());
    }
    std::fs::create_dir_all(dir.join("src"))
        .map_err(|e| anyhow::anyhow!("creating package directory: {e}"))?;

    let manifest = OxilakeManifest {
        package: PackageSection {
            name: name.to_string(),
            version: "0.1.0".to_string(),
            lean_version: "0.1".to_string(),
            description: None,
        },
        dependencies: Default::default(),
    };
    manifest.save(&dir.join("oxilake.toml"))?;

    // Scaffold Main.lean
    std::fs::write(
        dir.join("src").join("Main.lean"),
        format!("-- {name}\n\ndef main : IO Unit := do\n  IO.println \"Hello from {name}!\"\n"),
    )
    .map_err(|e| anyhow::anyhow!("creating Main.lean: {e}"))?;

    println!("Created package '{}' at '{}'", name, dir.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_new_creates_structure() {
        let base = env::temp_dir().join("oxilake_test_new");
        let pkg_dir = base.join("mypkg");
        std::fs::remove_dir_all(&base).ok();

        run("mypkg", Some(pkg_dir.clone())).unwrap();

        assert!(pkg_dir.join("oxilake.toml").exists());
        assert!(pkg_dir.join("src").join("Main.lean").exists());

        let content = std::fs::read_to_string(pkg_dir.join("src/Main.lean")).unwrap();
        assert!(content.contains("mypkg"));

        std::fs::remove_dir_all(&base).ok();
    }

    #[test]
    fn test_new_fails_if_dir_exists() {
        let dir = env::temp_dir().join("oxilake_test_exists");
        std::fs::create_dir_all(&dir).ok();
        assert!(run("exists", Some(dir.clone())).is_err());
        std::fs::remove_dir_all(&dir).ok();
    }
}
