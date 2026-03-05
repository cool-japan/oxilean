//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::path::{Path, PathBuf};

use super::types::{
    Dependency, DependencySource, EnvOverride, EnvOverrideSet, ExtendedBuildTarget, Feature,
    GitReference, LintConfig, LockedDependency, Lockfile, Manifest, ManifestError, ManifestParser,
    PackageRegistry, PlatformCondition, PlatformDependency, PlatformResolver, RegistryEntry,
    TomlSerialValue, TomlValue, Version, VersionConstraint, WorkspaceManifestConfig,
};

/// Parse a version constraint from a string.
pub fn parse_version_constraint(input: &str) -> Result<VersionConstraint, ManifestError> {
    let input = input.trim();
    if input.is_empty() || input == "*" {
        return Ok(VersionConstraint::Any);
    }
    if input.contains("||") {
        let parts: Vec<&str> = input.split("||").collect();
        let mut constraints = Vec::new();
        for part in parts {
            constraints.push(parse_version_constraint(part)?);
        }
        return Ok(VersionConstraint::Or(constraints));
    }
    if input.contains(',') {
        let parts: Vec<&str> = input.split(',').collect();
        let mut constraints = Vec::new();
        for part in parts {
            constraints.push(parse_version_constraint(part)?);
        }
        return Ok(VersionConstraint::And(constraints));
    }
    let input = input.trim();
    if let Some(rest) = input.strip_prefix('^') {
        let v: Version = rest
            .trim()
            .parse()
            .map_err(|e| ManifestError::InvalidVersion(format!("{}", e)))?;
        return Ok(VersionConstraint::Caret(v));
    }
    if let Some(rest) = input.strip_prefix('~') {
        let v: Version = rest
            .trim()
            .parse()
            .map_err(|e| ManifestError::InvalidVersion(format!("{}", e)))?;
        return Ok(VersionConstraint::Tilde(v));
    }
    if let Some(rest) = input.strip_prefix(">=") {
        let v: Version = rest
            .trim()
            .parse()
            .map_err(|e| ManifestError::InvalidVersion(format!("{}", e)))?;
        return Ok(VersionConstraint::GreaterEqual(v));
    }
    if let Some(rest) = input.strip_prefix('>') {
        let v: Version = rest
            .trim()
            .parse()
            .map_err(|e| ManifestError::InvalidVersion(format!("{}", e)))?;
        return Ok(VersionConstraint::Greater(v));
    }
    if let Some(rest) = input.strip_prefix("<=") {
        let v: Version = rest
            .trim()
            .parse()
            .map_err(|e| ManifestError::InvalidVersion(format!("{}", e)))?;
        return Ok(VersionConstraint::LessEqual(v));
    }
    if let Some(rest) = input.strip_prefix('<') {
        let v: Version = rest
            .trim()
            .parse()
            .map_err(|e| ManifestError::InvalidVersion(format!("{}", e)))?;
        return Ok(VersionConstraint::Less(v));
    }
    if let Some(rest) = input.strip_prefix('=') {
        let v: Version = rest
            .trim()
            .parse()
            .map_err(|e| ManifestError::InvalidVersion(format!("{}", e)))?;
        return Ok(VersionConstraint::Exact(v));
    }
    let v: Version = input
        .parse()
        .map_err(|e| ManifestError::InvalidVersion(format!("{}", e)))?;
    Ok(VersionConstraint::Exact(v))
}
/// Parse a manifest file from a path.
pub fn parse_manifest(path: &Path) -> Result<Manifest, ManifestError> {
    let source = std::fs::read_to_string(path)
        .map_err(|e| ManifestError::IoError(format!("{}: {}", path.display(), e)))?;
    parse_manifest_str(&source, path)
}
/// Parse a manifest from a string.
pub fn parse_manifest_str(source: &str, manifest_path: &Path) -> Result<Manifest, ManifestError> {
    let parser = ManifestParser::new(source);
    let toml = parser.parse()?;
    let table = toml
        .as_table()
        .ok_or_else(|| ManifestError::ParseError("expected table at root".to_string()))?;
    let pkg = table
        .get("package")
        .and_then(|v| v.as_table())
        .ok_or_else(|| ManifestError::MissingField("package".to_string()))?;
    let name = pkg
        .get("name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ManifestError::MissingField("package.name".to_string()))?
        .to_string();
    let version_str = pkg
        .get("version")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ManifestError::MissingField("package.version".to_string()))?;
    let version: Version = version_str
        .parse()
        .map_err(|e| ManifestError::InvalidVersion(format!("{}", e)))?;
    let mut manifest = Manifest::new(&name, version);
    manifest.manifest_path = manifest_path.to_path_buf();
    if let Some(desc) = pkg.get("description").and_then(|v| v.as_str()) {
        manifest.metadata.description = Some(desc.to_string());
    }
    if let Some(lic) = pkg.get("license").and_then(|v| v.as_str()) {
        manifest.metadata.license = Some(lic.to_string());
    }
    if let Some(repo) = pkg.get("repository").and_then(|v| v.as_str()) {
        manifest.metadata.repository = Some(repo.to_string());
    }
    if let Some(deps) = table.get("dependencies").and_then(|v| v.as_table()) {
        for (dep_name, dep_val) in deps {
            let dep = parse_dependency_value(dep_name, dep_val)?;
            manifest.add_dependency(dep);
        }
    }
    if let Some(deps) = table.get("dev-dependencies").and_then(|v| v.as_table()) {
        for (dep_name, dep_val) in deps {
            let dep = parse_dependency_value(dep_name, dep_val)?;
            manifest.add_dev_dependency(dep);
        }
    }
    if let Some(features) = table.get("features").and_then(|v| v.as_table()) {
        for (feat_name, feat_val) in features {
            let mut feature = Feature::new(feat_name);
            if let Some(arr) = feat_val.as_array() {
                for item in arr {
                    if let Some(s) = item.as_str() {
                        if let Some(dep_name) = s.strip_prefix("dep:") {
                            feature.enables.push(dep_name.to_string());
                        } else {
                            feature.implies.push(s.to_string());
                        }
                    }
                }
            }
            manifest.add_feature(feature);
        }
    }
    Ok(manifest)
}
fn parse_dependency_value(name: &str, value: &TomlValue) -> Result<Dependency, ManifestError> {
    match value {
        TomlValue::Str(version_str) => {
            let constraint = parse_version_constraint(version_str)?;
            Ok(Dependency::new(name, constraint))
        }
        TomlValue::Table(t) => {
            let version = if let Some(v) = t.get("version").and_then(|v| v.as_str()) {
                parse_version_constraint(v)?
            } else {
                VersionConstraint::Any
            };
            let mut dep = Dependency::new(name, version);
            if let Some(path) = t.get("path").and_then(|v| v.as_str()) {
                dep.source = DependencySource::Path {
                    path: PathBuf::from(path),
                };
            }
            if let Some(git_url) = t.get("git").and_then(|v| v.as_str()) {
                let reference = if let Some(branch) = t.get("branch").and_then(|v| v.as_str()) {
                    GitReference::Branch(branch.to_string())
                } else if let Some(tag) = t.get("tag").and_then(|v| v.as_str()) {
                    GitReference::Tag(tag.to_string())
                } else if let Some(rev) = t.get("rev").and_then(|v| v.as_str()) {
                    GitReference::Rev(rev.to_string())
                } else {
                    GitReference::Default
                };
                dep.source = DependencySource::Git {
                    url: git_url.to_string(),
                    reference,
                };
            }
            if let Some(opt) = t.get("optional").and_then(|v| v.as_bool()) {
                dep.optional = opt;
            }
            if let Some(feats) = t.get("features").and_then(|v| v.as_array()) {
                for f in feats {
                    if let Some(s) = f.as_str() {
                        dep.features.push(s.to_string());
                    }
                }
            }
            Ok(dep)
        }
        _ => Err(ManifestError::ParseError(format!(
            "invalid dependency format for '{}'",
            name
        ))),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_version_parsing() {
        let v: Version = "1.2.3".parse().expect("parse should succeed");
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 2);
        assert_eq!(v.patch, 3);
        assert!(v.pre.is_none());
        assert!(v.build_meta.is_none());
    }
    #[test]
    fn test_version_with_pre() {
        let v: Version = "1.0.0-alpha.1".parse().expect("parse should succeed");
        assert_eq!(v.pre, Some("alpha.1".to_string()));
    }
    #[test]
    fn test_version_ordering() {
        let v1: Version = "1.0.0".parse().expect("parse should succeed");
        let v2: Version = "1.0.1".parse().expect("parse should succeed");
        let v3: Version = "1.1.0".parse().expect("parse should succeed");
        let v4: Version = "2.0.0".parse().expect("parse should succeed");
        assert!(v1 < v2);
        assert!(v2 < v3);
        assert!(v3 < v4);
    }
    #[test]
    fn test_caret_constraint() {
        let c = VersionConstraint::Caret(Version::new(1, 2, 3));
        assert!(c.matches(&Version::new(1, 2, 3)));
        assert!(c.matches(&Version::new(1, 9, 0)));
        assert!(!c.matches(&Version::new(2, 0, 0)));
        assert!(!c.matches(&Version::new(1, 2, 2)));
    }
    #[test]
    fn test_tilde_constraint() {
        let c = VersionConstraint::Tilde(Version::new(1, 2, 0));
        assert!(c.matches(&Version::new(1, 2, 0)));
        assert!(c.matches(&Version::new(1, 2, 5)));
        assert!(!c.matches(&Version::new(1, 3, 0)));
    }
    #[test]
    fn test_manifest_validation() {
        let manifest = Manifest::new("test-pkg", Version::new(0, 1, 0));
        let errors = manifest.validate();
        assert!(errors.is_empty());
    }
    #[test]
    fn test_manifest_parse_simple() {
        let source = r#"
[package]
name = "my-pkg"
version = "1.0.0"
description = "A test package"

[dependencies]
foo = "^1.0.0"
"#;
        let manifest = parse_manifest_str(source, Path::new("OxiLean.toml"))
            .expect("manifest operation should succeed");
        assert_eq!(manifest.name, "my-pkg");
        assert_eq!(manifest.version, Version::new(1, 0, 0));
        assert!(manifest.dependencies.contains_key("foo"));
    }
    #[test]
    fn test_lockfile_serialization() {
        let mut lockfile = Lockfile::new();
        lockfile.add_package(LockedDependency {
            name: "foo".to_string(),
            version: Version::new(1, 0, 0),
            source: "registry".to_string(),
            checksum: Some("abc123".to_string()),
            dependencies: vec!["bar".to_string()],
        });
        let serialized = lockfile.serialize();
        assert!(serialized.contains("foo"));
        assert!(serialized.contains("1.0.0"));
    }
    #[test]
    fn test_feature_resolution() {
        let mut manifest = Manifest::new("test", Version::new(0, 1, 0));
        manifest.add_feature(Feature::new("full").imply("logging").imply("metrics"));
        manifest.add_feature(Feature::new("logging"));
        manifest.add_feature(Feature::new("metrics"));
        manifest.default_features = vec!["logging".to_string()];
        let resolved = manifest.resolve_features(&["full".to_string()]);
        assert!(resolved.contains_key("full"));
        assert!(resolved.contains_key("logging"));
        assert!(resolved.contains_key("metrics"));
    }
}
/// Merges a workspace manifest into a package manifest.
#[allow(dead_code)]
pub fn merge_workspace_into_package(workspace: &WorkspaceManifestConfig, package: &mut Manifest) {
    if package.version == (Version::new(0, 0, 0)) {
        if let Some(ref v) = workspace.version {
            package.version = v.clone();
        }
    }
    for (name, dep) in &workspace.shared_deps {
        package
            .dependencies
            .entry(name.clone())
            .or_insert_with(|| dep.clone());
    }
}
/// Serialize a `Manifest` to a minimal TOML-like string.
#[allow(dead_code)]
pub fn manifest_to_toml(manifest: &Manifest) -> String {
    let mut lines = Vec::new();
    lines.push("[package]".to_string());
    lines.push(format!("name = \"{}\"", manifest.name));
    lines.push(format!("version = \"{}\"", manifest.version));
    if let Some(ref desc) = manifest.metadata.description {
        lines.push(format!("description = \"{}\"", desc));
    }
    if let Some(ref license) = manifest.metadata.license {
        lines.push(format!("license = \"{}\"", license));
    }
    if !manifest.metadata.authors.is_empty() {
        let authors: Vec<_> = manifest
            .metadata
            .authors
            .iter()
            .map(|a| format!("\"{}\"", a))
            .collect();
        lines.push(format!("authors = [{}]", authors.join(", ")));
    }
    if !manifest.dependencies.is_empty() {
        lines.push(String::new());
        lines.push("[dependencies]".to_string());
        for (name, dep) in &manifest.dependencies {
            let constraint_str = format!("{}", dep.version);
            match &dep.source {
                DependencySource::Registry { .. } => {
                    lines.push(format!("{} = \"{}\"", name, constraint_str));
                }
                DependencySource::Path { path: p } => {
                    lines.push(format!("{} = {{ path = \"{}\" }}", name, p.display()));
                }
                DependencySource::Git { url, reference } => {
                    let ref_part = match reference {
                        GitReference::Rev(r) => format!(", rev = \"{}\"", r),
                        GitReference::Branch(b) => format!(", branch = \"{}\"", b),
                        GitReference::Tag(t) => format!(", tag = \"{}\"", t),
                        GitReference::Default => String::new(),
                    };
                    lines.push(format!("{} = {{ git = \"{}\"{} }}", name, url, ref_part));
                }
            }
        }
    }
    if !manifest.features.is_empty() {
        lines.push(String::new());
        lines.push("[features]".to_string());
        for (name, feat) in &manifest.features {
            let implies: Vec<_> = feat.implies.iter().map(|i| format!("\"{}\"", i)).collect();
            lines.push(format!("{} = [{}]", name, implies.join(", ")));
        }
    }
    lines.join("\n")
}
#[cfg(test)]
mod extended_manifest_tests {
    use super::*;
    #[test]
    fn test_env_override_resolve_default() {
        let o =
            EnvOverride::new("OXILEAN_TEST_NONEXISTENT_VAR_XYZ", "version").with_default("0.99.0");
        assert_eq!(o.resolve(), Some("0.99.0".to_string()));
        assert!(!o.is_set());
    }
    #[test]
    fn test_env_override_set() {
        let mut set = EnvOverrideSet::new();
        set.add(EnvOverride::new("NONEXISTENT_VAR_A", "name").with_default("my-pkg"));
        set.add(EnvOverride::new("NONEXISTENT_VAR_B", "version").with_default("1.2.3"));
        let resolved = set.resolve_all();
        assert_eq!(resolved.get("name"), Some(&"my-pkg".to_string()));
        assert_eq!(resolved.get("version"), Some(&"1.2.3".to_string()));
    }
    #[test]
    fn test_platform_condition_os() {
        let cond = PlatformCondition::linux();
        assert!(cond.eval("linux", "x86_64", "x86_64-unknown-linux-gnu"));
        assert!(!cond.eval("windows", "x86_64", "x86_64-pc-windows-msvc"));
    }
    #[test]
    fn test_platform_condition_and() {
        let cond = PlatformCondition::And(vec![
            PlatformCondition::linux(),
            PlatformCondition::x86_64(),
        ]);
        assert!(cond.eval("linux", "x86_64", "x86_64-unknown-linux-gnu"));
        assert!(!cond.eval("linux", "aarch64", "aarch64-unknown-linux-gnu"));
    }
    #[test]
    fn test_platform_condition_or() {
        let cond = PlatformCondition::Or(vec![
            PlatformCondition::linux(),
            PlatformCondition::windows(),
        ]);
        assert!(cond.eval("linux", "x86_64", ""));
        assert!(cond.eval("windows", "x86_64", ""));
        assert!(!cond.eval("macos", "x86_64", ""));
    }
    #[test]
    fn test_platform_condition_not() {
        let cond = PlatformCondition::Not(Box::new(PlatformCondition::windows()));
        assert!(cond.eval("linux", "x86_64", ""));
        assert!(!cond.eval("windows", "x86_64", ""));
    }
    #[test]
    fn test_platform_resolver() {
        let dep_linux = Dependency::registry("libc", VersionConstraint::Any);
        let dep_windows = Dependency::registry("winapi", VersionConstraint::Any);
        let mut resolver = PlatformResolver::new();
        resolver.add(PlatformDependency::new(
            PlatformCondition::linux(),
            "libc",
            dep_linux,
        ));
        resolver.add(PlatformDependency::new(
            PlatformCondition::windows(),
            "winapi",
            dep_windows,
        ));
        let linux_deps = resolver.resolve("linux", "x86_64", "x86_64-unknown-linux-gnu");
        assert!(linux_deps.contains_key("libc"));
        assert!(!linux_deps.contains_key("winapi"));
        let win_deps = resolver.resolve("windows", "x86_64", "x86_64-pc-windows-msvc");
        assert!(!win_deps.contains_key("libc"));
        assert!(win_deps.contains_key("winapi"));
    }
    #[test]
    fn test_workspace_config() {
        let ws = WorkspaceManifestConfig::new("/workspace")
            .add_member("crates/foo")
            .add_member("crates/bar")
            .with_version(Version::new(1, 0, 0));
        assert_eq!(ws.member_count(), 2);
        assert!(ws.validate().is_empty());
    }
    #[test]
    fn test_workspace_merge() {
        let ws = WorkspaceManifestConfig::new("/ws")
            .with_version(Version::new(2, 0, 0))
            .add_shared_dep(
                "serde",
                Dependency::registry("serde", VersionConstraint::Any),
            );
        let mut pkg = Manifest::new("my-pkg", Version::new(0, 0, 0));
        merge_workspace_into_package(&ws, &mut pkg);
        assert_eq!(pkg.version, Version::new(2, 0, 0));
        assert!(pkg.dependencies.contains_key("serde"));
    }
    #[test]
    fn test_lint_config_rustflags() {
        let lint = LintConfig::new()
            .allow("dead_code")
            .deny("unused_imports")
            .warn("clippy::all");
        let flags = lint.to_rustflags();
        assert!(flags.contains("-A dead_code"));
        assert!(flags.contains("-D unused_imports"));
        assert!(flags.contains("-W clippy::all"));
    }
    #[test]
    fn test_extended_build_target_binary() {
        let target = ExtendedBuildTarget::binary("my-tool")
            .require_feature("network")
            .link_lib("ssl")
            .rustflag("--edition=2021");
        assert_eq!(target.kind, "bin");
        assert_eq!(target.required_features, vec!["network"]);
        assert!(target.link_libs.contains(&"ssl".to_string()));
    }
    #[test]
    fn test_toml_serial_value_serialization() {
        assert_eq!(
            TomlSerialValue::String("hello".to_string()).to_toml_string(),
            "\"hello\""
        );
        assert_eq!(TomlSerialValue::Integer(42).to_toml_string(), "42");
        assert_eq!(TomlSerialValue::Bool(true).to_toml_string(), "true");
        let arr = TomlSerialValue::Array(vec![
            TomlSerialValue::String("a".to_string()),
            TomlSerialValue::String("b".to_string()),
        ]);
        assert_eq!(arr.to_toml_string(), "[\"a\", \"b\"]");
    }
    #[test]
    fn test_manifest_to_toml() {
        let mut manifest = Manifest::new("my-pkg", Version::new(1, 0, 0));
        manifest.metadata.description = Some("A test package".to_string());
        manifest.add_dependency(Dependency::registry(
            "foo",
            VersionConstraint::Caret(Version::new(1, 0, 0)),
        ));
        let toml = manifest_to_toml(&manifest);
        assert!(toml.contains("[package]"));
        assert!(toml.contains("name = \"my-pkg\""));
        assert!(toml.contains("[dependencies]"));
        assert!(toml.contains("foo"));
    }
    #[test]
    fn test_registry_entry() {
        let entry = RegistryEntry::new("serde", Version::new(1, 0, 0))
            .with_version(Version::new(1, 1, 0))
            .with_downloads(100_000);
        assert_eq!(entry.versions.len(), 2);
        assert_eq!(entry.latest, Version::new(1, 1, 0));
        assert_eq!(entry.downloads, 100_000);
    }
    #[test]
    fn test_package_registry_lookup_and_best_version() {
        let mut registry = PackageRegistry::new();
        let entry =
            RegistryEntry::new("serde", Version::new(1, 0, 0)).with_version(Version::new(1, 2, 0));
        registry.publish(entry);
        let found = registry.lookup("serde");
        assert!(found.is_some());
        let best = registry.best_version("serde", &VersionConstraint::Caret(Version::new(1, 0, 0)));
        assert_eq!(best, Some(&Version::new(1, 2, 0)));
    }
    #[test]
    fn test_package_registry_search() {
        let mut registry = PackageRegistry::new();
        registry.publish(RegistryEntry::new("serde", Version::new(1, 0, 0)));
        registry.publish(RegistryEntry::new("serde_json", Version::new(1, 0, 0)));
        registry.publish(RegistryEntry::new("tokio", Version::new(1, 0, 0)));
        let results = registry.search("serde");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_platform_condition_display() {
        let cond = PlatformCondition::linux();
        let s = format!("{}", cond);
        assert!(s.contains("linux"));
    }
}
