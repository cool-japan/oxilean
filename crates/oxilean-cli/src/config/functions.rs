//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashMap;
use std::path::PathBuf;

use super::types::{
    CacheConfig, Config, ConfigAliases, ConfigAnnotation, ConfigBuilder, ConfigChanges,
    ConfigComparison, ConfigHistory, ConfigMigrationStep, ConfigMigrator, ConfigPreset,
    ConfigPresetMap, ConfigSchema, ConfigSchemaEntry, ConfigStats, ConfigValidator,
    ConfigValueType, ExtendedConfig, LayeredConfig, LintSeverity, LspConfig, OutputFormat,
    PerfProfile, ProofCheckMode, SessionConfig,
};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_config_new() {
        let config = Config::new();
        assert_eq!(config.verbosity, 1);
        assert!(config.color);
        assert!(config.unicode);
    }
    #[test]
    fn test_add_library_path() {
        let mut config = Config::new();
        let path = PathBuf::from("/custom/lib");
        config.add_library_path(path.clone());
        assert!(config.library_path.contains(&path));
    }
    #[test]
    fn test_set_verbosity() {
        let mut config = Config::new();
        config.set_verbosity(3);
        assert_eq!(config.verbosity, 3);
        assert!(config.is_debug());
        config.set_verbosity(5);
        assert_eq!(config.verbosity, 3);
    }
    #[test]
    fn test_verbosity_levels() {
        let mut config = Config::new();
        config.set_verbosity(0);
        assert!(config.is_quiet());
        assert!(!config.is_verbose());
        assert!(!config.is_debug());
        config.set_verbosity(2);
        assert!(config.is_verbose());
        assert!(!config.is_debug());
        config.set_verbosity(3);
        assert!(config.is_debug());
    }
    #[test]
    fn test_custom_settings() {
        let mut config = Config::new();
        config.set_custom("key".to_string(), "value".to_string());
        assert_eq!(config.get_custom("key"), Some(&"value".to_string()));
        assert_eq!(config.get_custom("missing"), None);
    }
    #[test]
    fn test_config_builder() {
        let config = ConfigBuilder::new()
            .verbosity(2)
            .color(false)
            .unicode(false)
            .max_errors(5)
            .experimental(true)
            .build();
        assert_eq!(config.verbosity, 2);
        assert!(!config.color);
        assert!(!config.unicode);
        assert_eq!(config.max_errors, 5);
        assert!(config.experimental);
    }
}
/// Return the number of logical CPUs on this machine.
pub fn num_cpus() -> usize {
    std::fs::read_to_string("/proc/cpuinfo")
        .ok()
        .map(|s| s.lines().filter(|l| l.starts_with("processor")).count())
        .filter(|&n| n > 0)
        .unwrap_or(4)
}
/// Parse a KEY=VALUE string into a key/value pair.
///
/// Returns None if the string does not contain =.
pub fn parse_define(s: &str) -> Option<(String, String)> {
    let pos = s.find('=')?;
    let key = s[..pos].to_string();
    let val = s[pos + 1..].to_string();
    Some((key, val))
}
/// Merge multiple Config objects into one, giving priority to later entries.
pub fn merge_configs(configs: &[Config]) -> Config {
    let mut result = Config::new();
    for c in configs {
        for p in &c.library_path {
            result.add_library_path(p.clone());
        }
        result.verbosity = c.verbosity;
        result.color = c.color;
        result.unicode = c.unicode;
        result.max_errors = c.max_errors;
        result.experimental = c.experimental;
        result.timeout = c.timeout;
        for (k, v) in &c.custom {
            result.custom.insert(k.clone(), v.clone());
        }
    }
    result
}
/// Serialize a Config to a simple key=value string format.
pub fn config_to_string(config: &Config) -> String {
    let mut out = String::new();
    out.push_str(&format!("verbosity={}\n", config.verbosity));
    out.push_str(&format!("color={}\n", config.color));
    out.push_str(&format!("unicode={}\n", config.unicode));
    out.push_str(&format!("max_errors={}\n", config.max_errors));
    out.push_str(&format!("experimental={}\n", config.experimental));
    if let Some(t) = config.timeout {
        out.push_str(&format!("timeout={}\n", t));
    }
    for (k, v) in &config.custom {
        out.push_str(&format!("custom.{}={}\n", k, v));
    }
    out
}
/// Parse a simple key=value format back into a Config.
pub fn config_from_string(s: &str) -> Config {
    let mut config = Config::new();
    for line in s.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((k, v)) = parse_define(line) {
            match k.as_str() {
                "verbosity" => {
                    if let Ok(n) = v.parse::<u8>() {
                        config.verbosity = n.min(3);
                    }
                }
                "color" => config.color = v == "true",
                "unicode" => config.unicode = v == "true",
                "max_errors" => {
                    if let Ok(n) = v.parse::<usize>() {
                        config.max_errors = n;
                    }
                }
                "experimental" => config.experimental = v == "true",
                "timeout" => {
                    if let Ok(n) = v.parse::<u64>() {
                        config.timeout = Some(n);
                    }
                }
                k if k.starts_with("custom.") => {
                    let key = k["custom.".len()..].to_string();
                    config.custom.insert(key, v);
                }
                _ => {}
            }
        }
    }
    config
}
#[cfg(test)]
mod extended_tests {
    use super::*;
    #[test]
    fn test_extended_config_new() {
        let cfg = ExtendedConfig::new();
        assert_eq!(cfg.output_format, OutputFormat::Text);
        assert_eq!(cfg.proof_check, ProofCheckMode::Full);
        assert!(!cfg.sorry_allowed());
    }
    #[test]
    fn test_with_output_format() {
        let cfg = ExtendedConfig::new().with_output_format(OutputFormat::Json);
        assert_eq!(cfg.output_format, OutputFormat::Json);
    }
    #[test]
    fn test_with_proof_check_skip() {
        let cfg = ExtendedConfig::new().with_proof_check(ProofCheckMode::Skip);
        assert!(cfg.sorry_allowed());
    }
    #[test]
    fn test_with_threads() {
        let cfg = ExtendedConfig::new().with_threads(8);
        assert_eq!(cfg.effective_threads(), 8);
        let cfg0 = ExtendedConfig::new().with_threads(0);
        assert_eq!(cfg0.effective_threads(), 1);
    }
    #[test]
    fn test_define() {
        let cfg = ExtendedConfig::new().define("FOO".into(), "BAR".into());
        assert_eq!(cfg.defines.get("FOO"), Some(&"BAR".to_string()));
    }
    #[test]
    fn test_add_import_path() {
        let mut cfg = ExtendedConfig::new();
        cfg.add_import_path(PathBuf::from("/lib/lean"));
        cfg.add_import_path(PathBuf::from("/lib/lean"));
        assert_eq!(cfg.import_paths.len(), 1);
    }
    #[test]
    fn test_validate_small_recursion() {
        let mut cfg = ExtendedConfig::new();
        cfg.max_recursion = 10;
        let warns = cfg.validate();
        assert!(warns.iter().any(|w| w.contains("max_recursion")));
    }
    #[test]
    fn test_parse_define() {
        let (k, v) = parse_define("FOO=BAR").expect("parsing should succeed");
        assert_eq!(k, "FOO");
        assert_eq!(v, "BAR");
        assert!(parse_define("NOEQUALS").is_none());
    }
    #[test]
    fn test_config_roundtrip() {
        let mut config = Config::new();
        config.verbosity = 2;
        config.color = false;
        config.unicode = false;
        config.max_errors = 5;
        config.set_custom("theme".into(), "dark".into());
        let s = config_to_string(&config);
        let restored = config_from_string(&s);
        assert_eq!(restored.verbosity, 2);
        assert!(!restored.color);
        assert_eq!(restored.max_errors, 5);
        assert_eq!(restored.get_custom("theme"), Some(&"dark".to_string()));
    }
    #[test]
    fn test_merge_configs() {
        let mut c1 = Config::new();
        c1.verbosity = 0;
        let mut c2 = Config::new();
        c2.verbosity = 3;
        c2.color = false;
        let merged = merge_configs(&[c1, c2]);
        assert_eq!(merged.verbosity, 3);
        assert!(!merged.color);
    }
    #[test]
    fn test_lint_severity_ordering() {
        assert!(LintSeverity::Hint < LintSeverity::Warning);
        assert!(LintSeverity::Warning < LintSeverity::Error);
        assert!(LintSeverity::Error < LintSeverity::Off);
    }
    #[test]
    fn test_output_format_default() {
        assert_eq!(OutputFormat::default(), OutputFormat::Text);
    }
    #[test]
    fn test_cache_config_default() {
        let cache = CacheConfig::default();
        assert!(cache.enabled);
        assert_eq!(cache.max_size, 1 << 30);
    }
    #[test]
    fn test_lsp_config_default() {
        let lsp = LspConfig::default();
        assert!(lsp.hover);
        assert!(lsp.definition);
        assert_eq!(lsp.max_completions, 50);
    }
    #[test]
    fn test_perf_profile_default() {
        assert_eq!(PerfProfile::default(), PerfProfile::Balanced);
    }
    #[test]
    fn test_merge_extended_config() {
        let mut base = ExtendedConfig::new();
        let mut other = ExtendedConfig::new();
        other.defines.insert("KEY".into(), "VAL".into());
        other.import_paths.push(PathBuf::from("/extra"));
        base.merge(&other);
        assert_eq!(base.defines.get("KEY"), Some(&"VAL".to_string()));
        assert!(base.import_paths.contains(&PathBuf::from("/extra")));
    }
}
#[cfg(test)]
mod preset_tests {
    use super::*;
    #[test]
    fn test_debug_preset() {
        let p = ConfigPreset::debug();
        assert_eq!(p.name, "debug");
        assert_eq!(p.config.verbosity, 3);
        assert!(p.config.experimental);
    }
    #[test]
    fn test_ci_preset() {
        let p = ConfigPreset::ci();
        assert_eq!(p.name, "ci");
        assert_eq!(p.config.verbosity, 0);
        assert!(!p.config.color);
    }
    #[test]
    fn test_release_preset() {
        let p = ConfigPreset::release();
        assert_eq!(p.name, "release");
        assert!(!p.config.experimental);
    }
}
/// Load a config from environment variables.
///
/// Reads `OXILEAN_VERBOSITY`, `OXILEAN_COLOR`, `OXILEAN_UNICODE`,
/// `OXILEAN_MAX_ERRORS`, and `OXILEAN_TIMEOUT`.
#[allow(dead_code)]
pub fn config_from_env() -> Config {
    let mut config = Config::new();
    if let Ok(v) = std::env::var("OXILEAN_VERBOSITY") {
        if let Ok(n) = v.parse::<u8>() {
            config.set_verbosity(n);
        }
    }
    if let Ok(v) = std::env::var("OXILEAN_COLOR") {
        config.color = v != "0" && !v.eq_ignore_ascii_case("false");
    }
    if let Ok(v) = std::env::var("OXILEAN_UNICODE") {
        config.unicode = v != "0" && !v.eq_ignore_ascii_case("false");
    }
    if let Ok(v) = std::env::var("OXILEAN_MAX_ERRORS") {
        if let Ok(n) = v.parse::<usize>() {
            config.set_max_errors(n);
        }
    }
    if let Ok(v) = std::env::var("OXILEAN_TIMEOUT") {
        if let Ok(n) = v.parse::<u64>() {
            config.timeout = Some(n);
        }
    }
    config
}
#[cfg(test)]
mod config_extra_tests {
    use super::*;
    #[test]
    fn test_config_changes_diff_no_change() {
        let c = Config::new();
        let diff = ConfigChanges::diff(&c, &c);
        assert!(diff.is_empty());
    }
    #[test]
    fn test_config_changes_diff_verbosity() {
        let old = Config::new();
        let mut new = Config::new();
        new.set_verbosity(3);
        let diff = ConfigChanges::diff(&old, &new);
        assert!(diff.field_changed("verbosity"));
        assert_eq!(diff.len(), 1);
    }
    #[test]
    fn test_config_validator_empty_lib() {
        let mut c = Config::new();
        c.library_path.clear();
        let warns = ConfigValidator::validate(&c);
        assert!(warns.iter().any(|w| w.contains("library_path")));
    }
    #[test]
    fn test_config_validator_zero_errors() {
        let mut c = Config::new();
        c.set_max_errors(0);
        let warns = ConfigValidator::validate(&c);
        assert!(warns.iter().any(|w| w.contains("max_errors")));
    }
    #[test]
    fn test_session_config_override() {
        let mut s = SessionConfig::new(Config::new());
        s.set_override("verbosity", "3");
        assert!(s.dirty);
        assert_eq!(s.num_overrides(), 1);
        s.apply_overrides();
        assert_eq!(s.config.verbosity, 3);
        assert!(!s.dirty);
    }
    #[test]
    fn test_session_config_mark_saved() {
        let mut s = SessionConfig::new(Config::new());
        s.set_override("color", "false");
        s.mark_saved();
        assert!(!s.dirty);
        assert_eq!(s.num_overrides(), 1);
    }
    #[test]
    fn test_layered_config_resolve() {
        let base = Config::new();
        let mut project = Config::new();
        project.set_verbosity(2);
        let resolved = LayeredConfig::new(base).with_project(project).resolve();
        assert_eq!(resolved.verbosity, 2);
    }
    #[test]
    fn test_layered_config_layer_count() {
        let base = Config::new();
        let lc = LayeredConfig::new(base.clone())
            .with_project(base.clone())
            .with_user(base.clone());
        assert_eq!(lc.layer_count(), 3);
    }
    #[test]
    fn test_layered_config_session_wins() {
        let base = Config::new();
        let mut session = Config::new();
        session.set_verbosity(3);
        let mut project = Config::new();
        project.set_verbosity(1);
        let resolved = LayeredConfig::new(base)
            .with_project(project)
            .with_session(session)
            .resolve();
        assert_eq!(resolved.verbosity, 3);
    }
    #[test]
    fn test_parse_define_empty_val() {
        let (k, v) = parse_define("KEY=").expect("parsing should succeed");
        assert_eq!(k, "KEY");
        assert_eq!(v, "");
    }
    #[test]
    fn test_config_from_string_comments() {
        let s = "# this is a comment\nverbosity=2\n";
        let c = config_from_string(s);
        assert_eq!(c.verbosity, 2);
    }
    #[test]
    fn test_config_from_string_custom() {
        let s = "custom.theme=dark\ncustom.lang=en\n";
        let c = config_from_string(s);
        assert_eq!(c.get_custom("theme"), Some(&"dark".to_string()));
        assert_eq!(c.get_custom("lang"), Some(&"en".to_string()));
    }
}
#[allow(dead_code)]
pub fn default_oxilean_schema() -> ConfigSchema {
    let mut schema = ConfigSchema::new();
    schema.add_entry(ConfigSchemaEntry {
        key: "project.name".into(),
        value_type: ConfigValueType::String,
        required: true,
        default: None,
        description: "Project name".into(),
    });
    schema.add_entry(ConfigSchemaEntry {
        key: "build.jobs".into(),
        value_type: ConfigValueType::Integer,
        required: false,
        default: Some("4".into()),
        description: "Number of parallel build jobs".into(),
    });
    schema.add_entry(ConfigSchemaEntry {
        key: "build.output_dir".into(),
        value_type: ConfigValueType::Path,
        required: false,
        default: Some(".oxilean/build".into()),
        description: "Build output directory".into(),
    });
    schema.add_entry(ConfigSchemaEntry {
        key: "lint.enabled".into(),
        value_type: ConfigValueType::Bool,
        required: false,
        default: Some("true".into()),
        description: "Enable linting".into(),
    });
    schema.add_entry(ConfigSchemaEntry {
        key: "codegen.backend".into(),
        value_type: ConfigValueType::String,
        required: false,
        default: Some("zig".into()),
        description: "Codegen backend (zig, x86_64, wasm)".into(),
    });
    schema
}
#[allow(dead_code)]
pub fn serialize_config_toml(map: &std::collections::HashMap<String, String>) -> String {
    let mut keys: Vec<&String> = map.keys().collect();
    keys.sort();
    let mut out = String::new();
    let mut current_section = String::new();
    for key in keys {
        let parts: Vec<&str> = key.splitn(2, '.').collect();
        if parts.len() == 2 {
            let section = parts[0];
            let field = parts[1];
            if section != current_section {
                if !current_section.is_empty() {
                    out.push('\n');
                }
                out.push_str(&format!("[{}]\n", section));
                current_section = section.to_string();
            }
            let val = map.get(key).expect("key is from map.keys()");
            out.push_str(&format!("{} = \"{}\"\n", field, val));
        } else {
            out.push_str(&format!(
                "{} = \"{}\"\n",
                key,
                map.get(key).expect("key is from map.keys()")
            ));
        }
    }
    out
}
#[allow(dead_code)]
pub fn deserialize_config_toml(src: &str) -> std::collections::HashMap<String, String> {
    let mut map = std::collections::HashMap::new();
    let mut section = String::new();
    for line in src.lines() {
        let line = line.trim();
        if line.starts_with('[') && line.ends_with(']') {
            section = line[1..line.len() - 1].to_string();
        } else if let Some(eq) = line.find('=') {
            let key_part = line[..eq].trim();
            let val_part = line[eq + 1..].trim().trim_matches('"');
            let full_key = if section.is_empty() {
                key_part.to_string()
            } else {
                format!("{}.{}", section, key_part)
            };
            map.insert(full_key, val_part.to_string());
        }
    }
    map
}
#[allow(dead_code)]
pub fn default_config_migrator() -> ConfigMigrator {
    let mut m = ConfigMigrator::new();
    m.add_step(ConfigMigrationStep {
        from_version: 1,
        to_version: 2,
        description: "rename 'output' to 'build.output_dir'".into(),
        apply: |map| {
            if let Some(v) = map.remove("output") {
                map.insert("build.output_dir".into(), v);
            }
        },
    });
    m.add_step(ConfigMigrationStep {
        from_version: 2,
        to_version: 3,
        description: "rename 'threads' to 'build.jobs'".into(),
        apply: |map| {
            if let Some(v) = map.remove("threads") {
                map.insert("build.jobs".into(), v);
            }
        },
    });
    m
}
#[allow(dead_code)]
pub fn builtin_presets() -> Vec<ConfigPresetMap> {
    vec![
        ConfigPresetMap::new("minimal", "Minimal config for quick experiments")
            .set("build.jobs", "1")
            .set("lint.enabled", "false")
            .set("codegen.backend", "zig"),
        ConfigPresetMap::new("ci", "Config tuned for CI environments")
            .set("build.jobs", "8")
            .set("lint.enabled", "true")
            .set("codegen.backend", "x86_64"),
        ConfigPresetMap::new("dev", "Config for local development")
            .set("build.jobs", "4")
            .set("lint.enabled", "true")
            .set("codegen.backend", "zig"),
        ConfigPresetMap::new("release", "Optimized release config")
            .set("build.jobs", "16")
            .set("lint.enabled", "true")
            .set("codegen.backend", "x86_64"),
    ]
}
#[allow(dead_code)]
pub fn apply_preset(map: &mut std::collections::HashMap<String, String>, preset: &ConfigPresetMap) {
    for (k, v) in &preset.values {
        map.insert(k.clone(), v.clone());
    }
}
#[allow(dead_code)]
pub const SECRET_KEY_PREFIXES: &[&str] = &["auth.", "token.", "secret.", "api_key", "password"];
#[allow(dead_code)]
pub fn is_secret_key(key: &str) -> bool {
    SECRET_KEY_PREFIXES
        .iter()
        .any(|prefix| key.starts_with(prefix))
}
#[allow(dead_code)]
pub fn render_config_masked(map: &std::collections::HashMap<String, String>) -> String {
    let mut keys: Vec<&String> = map.keys().collect();
    keys.sort();
    let max_key = keys.iter().map(|k| k.len()).max().unwrap_or(10);
    let mut out = String::new();
    for key in keys {
        let val = if is_secret_key(key) {
            "***".to_string()
        } else {
            map[key].clone()
        };
        out.push_str(&format!("{:<width$} = {}\n", key, val, width = max_key));
    }
    out
}
#[allow(dead_code)]
pub fn compute_config_stats(map: &std::collections::HashMap<String, String>) -> ConfigStats {
    let mut stats = ConfigStats::default();
    stats.total_keys = map.len();
    for (k, v) in map {
        if is_secret_key(k) {
            stats.secret_keys += 1;
        }
        if let Some(dot) = k.find('.') {
            stats.sections.insert(k[..dot].to_string());
        }
        if k.len() > stats.longest_key {
            stats.longest_key = k.len();
        }
        if v.len() > stats.longest_value {
            stats.longest_value = v.len();
        }
    }
    stats
}
#[allow(dead_code)]
pub fn render_config_as_table(map: &std::collections::HashMap<String, String>) -> String {
    let mut keys: Vec<&String> = map.keys().collect();
    keys.sort();
    let max_key_len = keys.iter().map(|k| k.len()).max().unwrap_or(10);
    let mut out = String::new();
    out.push_str(&format!(
        "{:<width$}  {}\n",
        "KEY",
        "VALUE",
        width = max_key_len
    ));
    out.push_str(&"-".repeat(max_key_len + 20));
    out.push('\n');
    for key in keys {
        let val = map.get(key).expect("key is from map.keys()");
        out.push_str(&format!("{:<width$}  {}\n", key, val, width = max_key_len));
    }
    out
}
#[allow(dead_code)]
pub fn render_config_as_env(map: &std::collections::HashMap<String, String>) -> String {
    let mut keys: Vec<&String> = map.keys().collect();
    keys.sort();
    keys.iter()
        .map(|k| {
            let env_key = k.to_uppercase().replace('.', "_").replace('-', "_");
            format!("{}={}", env_key, map[*k])
        })
        .collect::<Vec<_>>()
        .join("\n")
}
#[allow(dead_code)]
pub fn render_config_as_json(map: &std::collections::HashMap<String, String>) -> String {
    let mut keys: Vec<&String> = map.keys().collect();
    keys.sort();
    let mut out = String::from("{\n");
    for (i, key) in keys.iter().enumerate() {
        let val = map.get(*key).expect("key is from map.keys()");
        let comma = if i + 1 < map.len() { "," } else { "" };
        out.push_str(&format!("  \"{}\": \"{}\"{}\n", key, val, comma));
    }
    out.push('}');
    out
}
#[allow(dead_code)]
pub fn compare_configs(
    old: &std::collections::HashMap<String, String>,
    new: &std::collections::HashMap<String, String>,
) -> ConfigComparison {
    let mut added = Vec::new();
    let mut removed = Vec::new();
    let mut changed = Vec::new();
    let mut unchanged = Vec::new();
    for (k, v) in new {
        match old.get(k) {
            None => added.push((k.clone(), v.clone())),
            Some(ov) if ov != v => changed.push((k.clone(), ov.clone(), v.clone())),
            _ => unchanged.push(k.clone()),
        }
    }
    for k in old.keys() {
        if !new.contains_key(k) {
            removed.push(k.clone());
        }
    }
    added.sort_by(|a, b| a.0.cmp(&b.0));
    removed.sort();
    changed.sort_by(|a, b| a.0.cmp(&b.0));
    unchanged.sort();
    ConfigComparison {
        added,
        removed,
        changed,
        unchanged,
    }
}
#[allow(dead_code)]
pub fn format_comparison(cmp: &ConfigComparison) -> String {
    let mut out = String::new();
    for (k, v) in &cmp.added {
        out.push_str(&format!("+ {} = {}\n", k, v));
    }
    for k in &cmp.removed {
        out.push_str(&format!("- {}\n", k));
    }
    for (k, ov, nv) in &cmp.changed {
        out.push_str(&format!("~ {} : {} -> {}\n", k, ov, nv));
    }
    out
}
#[allow(dead_code)]
pub fn validate_config_values(map: &std::collections::HashMap<String, String>) -> Vec<String> {
    let mut errors = Vec::new();
    if let Some(jobs) = map.get("build.jobs") {
        if jobs.parse::<usize>().is_err() {
            errors.push("build.jobs must be a positive integer".to_string());
        } else if jobs.parse::<usize>().unwrap_or(0) == 0 {
            errors.push("build.jobs must be > 0".to_string());
        }
    }
    if let Some(backend) = map.get("codegen.backend") {
        let valid = ["zig", "x86_64", "wasm", "aarch64"];
        if !valid.contains(&backend.as_str()) {
            errors.push(format!("codegen.backend '{}' is not valid", backend));
        }
    }
    errors
}
#[cfg(test)]
mod config_schema_tests {
    use super::*;
    #[test]
    fn test_schema_required_keys() {
        let schema = default_oxilean_schema();
        let required = schema.required_keys();
        assert!(required.contains(&"project.name"));
    }
    #[test]
    fn test_schema_validate_missing_required() {
        let schema = default_oxilean_schema();
        let map = std::collections::HashMap::new();
        let errors = schema.validate_map(&map);
        assert!(errors.iter().any(|e| e.contains("project.name")));
    }
    #[test]
    fn test_schema_validate_unknown_key() {
        let schema = default_oxilean_schema();
        let mut map = std::collections::HashMap::new();
        map.insert("project.name".into(), "test".into());
        map.insert("nonexistent.key".into(), "val".into());
        let errors = schema.validate_map(&map);
        assert!(errors.iter().any(|e| e.contains("unknown config key")));
    }
    #[test]
    fn test_serialize_deserialize_roundtrip() {
        let mut map = std::collections::HashMap::new();
        map.insert("build.jobs".into(), "4".into());
        map.insert("project.name".into(), "test".into());
        let serialized = serialize_config_toml(&map);
        let deserialized = deserialize_config_toml(&serialized);
        assert_eq!(
            deserialized.get("build.jobs").map(|s| s.as_str()),
            Some("4")
        );
        assert_eq!(
            deserialized.get("project.name").map(|s| s.as_str()),
            Some("test")
        );
    }
    #[test]
    fn test_config_aliases_resolve() {
        let aliases = ConfigAliases::new();
        assert_eq!(aliases.resolve("j"), "build.jobs");
        assert_eq!(aliases.resolve("backend"), "codegen.backend");
        assert_eq!(aliases.resolve("unknown"), "unknown");
    }
    #[test]
    fn test_config_history_record_and_undo() {
        let mut history = ConfigHistory::new(5);
        history.record("build.jobs", Some("4"), Some("8"));
        let undo = history.undo_last();
        assert_eq!(undo, Some(("build.jobs", Some("4"))));
    }
    #[test]
    fn test_config_history_max_entries() {
        let mut history = ConfigHistory::new(3);
        for i in 0..5 {
            history.record("key", Some(&i.to_string()), Some(&(i + 1).to_string()));
        }
        assert!(history.entries.len() <= 3);
    }
    #[test]
    fn test_is_secret_key() {
        assert!(is_secret_key("auth.token"));
        assert!(is_secret_key("password"));
        assert!(!is_secret_key("build.jobs"));
    }
    #[test]
    fn test_render_config_masked() {
        let mut map = std::collections::HashMap::new();
        map.insert("build.jobs".into(), "4".into());
        map.insert("auth.token".into(), "secret123".into());
        let rendered = render_config_masked(&map);
        assert!(rendered.contains("***"));
        assert!(rendered.contains("4"));
        assert!(!rendered.contains("secret123"));
    }
    #[test]
    fn test_builtin_presets_names() {
        let presets = builtin_presets();
        let names: Vec<&str> = presets.iter().map(|p| p.name.as_str()).collect();
        assert!(names.contains(&"minimal"));
        assert!(names.contains(&"ci"));
    }
    #[test]
    fn test_apply_preset() {
        let presets = builtin_presets();
        let ci = presets
            .iter()
            .find(|p| p.name == "ci")
            .expect("find should succeed");
        let mut map = std::collections::HashMap::new();
        apply_preset(&mut map, ci);
        assert_eq!(map.get("build.jobs").map(|s| s.as_str()), Some("8"));
    }
    #[test]
    fn test_config_migration() {
        let migrator = default_config_migrator();
        let mut map = std::collections::HashMap::new();
        map.insert("output".into(), "/tmp/build".into());
        map.insert("threads".into(), "4".into());
        let log = migrator.migrate(&mut map, 1, 3);
        assert_eq!(log.len(), 2);
        assert_eq!(
            map.get("build.output_dir").map(|s| s.as_str()),
            Some("/tmp/build")
        );
        assert_eq!(map.get("build.jobs").map(|s| s.as_str()), Some("4"));
    }
    #[test]
    fn test_render_config_as_env() {
        let mut map = std::collections::HashMap::new();
        map.insert("build.jobs".into(), "4".into());
        let env_str = render_config_as_env(&map);
        assert!(env_str.contains("BUILD_JOBS=4"));
    }
    #[test]
    fn test_render_config_as_json() {
        let mut map = std::collections::HashMap::new();
        map.insert("key".into(), "val".into());
        let json = render_config_as_json(&map);
        assert!(json.contains("\"key\""));
        assert!(json.contains("\"val\""));
    }
    #[test]
    fn test_compute_config_stats() {
        let mut map = std::collections::HashMap::new();
        map.insert("build.jobs".into(), "4".into());
        map.insert("auth.token".into(), "s".into());
        let stats = compute_config_stats(&map);
        assert_eq!(stats.total_keys, 2);
        assert_eq!(stats.secret_keys, 1);
        assert!(stats.sections.contains("build"));
        assert!(stats.sections.contains("auth"));
    }
    #[test]
    fn test_compare_configs_added() {
        let old = std::collections::HashMap::new();
        let mut new = std::collections::HashMap::new();
        new.insert("key".into(), "val".into());
        let cmp = compare_configs(&old, &new);
        assert_eq!(cmp.added.len(), 1);
    }
    #[test]
    fn test_compare_configs_removed() {
        let mut old = std::collections::HashMap::new();
        old.insert("key".into(), "val".into());
        let new = std::collections::HashMap::new();
        let cmp = compare_configs(&old, &new);
        assert_eq!(cmp.removed.len(), 1);
    }
    #[test]
    fn test_validate_config_values_invalid_jobs() {
        let mut map = std::collections::HashMap::new();
        map.insert("build.jobs".into(), "abc".into());
        let errors = validate_config_values(&map);
        assert!(!errors.is_empty());
    }
    #[test]
    fn test_validate_config_values_invalid_backend() {
        let mut map = std::collections::HashMap::new();
        map.insert("codegen.backend".into(), "invalid_backend".into());
        let errors = validate_config_values(&map);
        assert!(!errors.is_empty());
    }
}
#[allow(dead_code)]
pub fn apply_env_overrides(map: &mut std::collections::HashMap<String, String>) {
    let prefix = "OXILEAN_CONFIG_";
    for (k, v) in std::env::vars() {
        if let Some(suffix) = k.strip_prefix(prefix) {
            let config_key = suffix.to_lowercase().replace('_', ".");
            map.insert(config_key, v);
        }
    }
}
#[allow(dead_code)]
pub fn config_from_env_only() -> std::collections::HashMap<String, String> {
    let mut map = std::collections::HashMap::new();
    apply_env_overrides(&mut map);
    map
}
#[allow(dead_code)]
pub fn generate_config_docs(schema: &ConfigSchema) -> String {
    let mut doc = String::from("# OxiLean Configuration Reference\n\n");
    let mut sections: std::collections::HashMap<String, Vec<&ConfigSchemaEntry>> =
        std::collections::HashMap::new();
    for entry in &schema.entries {
        let section = entry.key.split('.').next().unwrap_or("general");
        sections.entry(section.to_string()).or_default().push(entry);
    }
    let mut section_names: Vec<&String> = sections.keys().collect();
    section_names.sort();
    for section in section_names {
        doc.push_str(&format!("## {}\n\n", section));
        let entries = &sections[section];
        for entry in entries {
            let req = if entry.required { " *(required)*" } else { "" };
            doc.push_str(&format!("### `{}`{}\n\n", entry.key, req));
            doc.push_str(&format!("{}\n\n", entry.description));
            if let Some(def) = &entry.default {
                doc.push_str(&format!("Default: `{}`\n\n", def));
            }
            doc.push_str(&format!("Type: `{:?}`\n\n", entry.value_type));
        }
    }
    doc
}
#[allow(dead_code)]
pub fn apply_config_patch(
    base: &mut std::collections::HashMap<String, String>,
    patch: &std::collections::HashMap<String, Option<String>>,
) {
    for (k, v) in patch {
        match v {
            Some(val) => {
                base.insert(k.clone(), val.clone());
            }
            None => {
                base.remove(k);
            }
        }
    }
}
#[cfg(test)]
mod config_ext2_tests {
    use super::*;
    #[test]
    fn test_config_annotation_doc_string() {
        let ann = ConfigAnnotation::new("build.jobs")
            .tag("build")
            .example("4")
            .since("0.1.0");
        let doc = ann.to_doc_string();
        assert!(doc.contains("build.jobs"));
        assert!(doc.contains("build"));
        assert!(doc.contains("0.1.0"));
    }
    #[test]
    fn test_generate_config_docs() {
        let schema = default_oxilean_schema();
        let docs = generate_config_docs(&schema);
        assert!(docs.contains("# OxiLean Configuration Reference"));
        assert!(docs.contains("project.name"));
    }
    #[test]
    fn test_apply_config_patch_add() {
        let mut base = std::collections::HashMap::new();
        base.insert("a".into(), "1".into());
        let mut patch = std::collections::HashMap::new();
        patch.insert("b".into(), Some("2".to_string()));
        apply_config_patch(&mut base, &patch);
        assert_eq!(base.get("b").map(|s| s.as_str()), Some("2"));
    }
    #[test]
    fn test_apply_config_patch_remove() {
        let mut base = std::collections::HashMap::new();
        base.insert("a".into(), "1".into());
        let mut patch = std::collections::HashMap::new();
        patch.insert("a".into(), None);
        apply_config_patch(&mut base, &patch);
        assert!(!base.contains_key("a"));
    }
    #[test]
    fn test_render_config_as_table_contains_header() {
        let mut map = std::collections::HashMap::new();
        map.insert("x".into(), "y".into());
        let table = render_config_as_table(&map);
        assert!(table.contains("KEY"));
        assert!(table.contains("VALUE"));
    }
}
#[allow(dead_code)]
pub fn config_one_liner(map: &std::collections::HashMap<String, String>) -> String {
    let jobs = map.get("build.jobs").map(|s| s.as_str()).unwrap_or("?");
    let backend = map
        .get("codegen.backend")
        .map(|s| s.as_str())
        .unwrap_or("?");
    let lint = map.get("lint.enabled").map(|s| s.as_str()).unwrap_or("?");
    let name = map
        .get("project.name")
        .map(|s| s.as_str())
        .unwrap_or("unnamed");
    format!(
        "project={} jobs={} backend={} lint={}",
        name, jobs, backend, lint
    )
}
#[allow(dead_code)]
pub fn config_key_count(map: &std::collections::HashMap<String, String>) -> usize {
    map.len()
}
#[allow(dead_code)]
pub fn config_section_count(map: &std::collections::HashMap<String, String>) -> usize {
    let mut sections = std::collections::HashSet::new();
    for k in map.keys() {
        if let Some(dot) = k.find('.') {
            sections.insert(&k[..dot]);
        }
    }
    sections.len()
}
#[allow(dead_code)]
pub fn config_has_required(
    map: &std::collections::HashMap<String, String>,
    schema: &ConfigSchema,
) -> bool {
    schema.required_keys().iter().all(|k| map.contains_key(*k))
}
#[cfg(test)]
mod config_summary_tests {
    use super::*;
    #[test]
    fn test_config_one_liner() {
        let mut map = std::collections::HashMap::new();
        map.insert("project.name".into(), "myproj".into());
        map.insert("build.jobs".into(), "4".into());
        map.insert("codegen.backend".into(), "zig".into());
        map.insert("lint.enabled".into(), "true".into());
        let s = config_one_liner(&map);
        assert!(s.contains("project=myproj"));
        assert!(s.contains("jobs=4"));
        assert!(s.contains("backend=zig"));
    }
    #[test]
    fn test_config_key_count() {
        let mut map = std::collections::HashMap::new();
        map.insert("a".into(), "1".into());
        map.insert("b".into(), "2".into());
        assert_eq!(config_key_count(&map), 2);
    }
    #[test]
    fn test_config_section_count() {
        let mut map = std::collections::HashMap::new();
        map.insert("build.jobs".into(), "4".into());
        map.insert("build.output".into(), "/tmp".into());
        map.insert("lint.enabled".into(), "true".into());
        assert_eq!(config_section_count(&map), 2);
    }
    #[test]
    fn test_config_has_required_true() {
        let schema = default_oxilean_schema();
        let mut map = std::collections::HashMap::new();
        map.insert("project.name".into(), "test".into());
        assert!(config_has_required(&map, &schema));
    }
    #[test]
    fn test_config_has_required_false() {
        let schema = default_oxilean_schema();
        let map = std::collections::HashMap::new();
        assert!(!config_has_required(&map, &schema));
    }
}
#[allow(dead_code)]
pub fn config_fingerprint(map: &std::collections::HashMap<String, String>) -> u64 {
    let mut keys: Vec<&String> = map.keys().collect();
    keys.sort();
    let mut h: u64 = 14695981039346656037;
    for key in keys {
        for byte in key.as_bytes() {
            h = h.wrapping_mul(1099511628211);
            h ^= *byte as u64;
        }
        for byte in map[key].as_bytes() {
            h = h.wrapping_mul(1099511628211);
            h ^= *byte as u64;
        }
    }
    h
}
#[cfg(test)]
mod config_fingerprint_tests {
    use super::*;
    #[test]
    fn test_fingerprint_deterministic() {
        let mut m = std::collections::HashMap::new();
        m.insert("a".into(), "1".into());
        let f1 = config_fingerprint(&m);
        let f2 = config_fingerprint(&m);
        assert_eq!(f1, f2);
    }
    #[test]
    fn test_fingerprint_differs_on_change() {
        let mut m = std::collections::HashMap::new();
        m.insert("a".into(), "1".into());
        let f1 = config_fingerprint(&m);
        m.insert("a".into(), "2".into());
        let f2 = config_fingerprint(&m);
        assert_ne!(f1, f2);
    }
}
