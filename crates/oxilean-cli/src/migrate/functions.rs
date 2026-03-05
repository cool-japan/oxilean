//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use super::types::{
    ApiRewrite, ApiRewriteSet, ExtendedMigrationReport, MigrationConfig, MigrationRecord,
    MigrationReport, MigrationRule, MigrationSession, MigrationSnapshot, MigrationStatus,
    ProposedChange, RuleApplication, RulePriority, RuleRegistry, SourceLanguage, ValidationMessage,
    ValidationResult, ValidationSeverity, Version, VersionMigrationChain, VersionMigrationStep,
};

/// Replace `=>` with `->` in lambda and match expressions.
/// Careful not to replace `=>` inside string literals or comments.
pub fn transform_fat_arrow(line: &str) -> (String, usize) {
    let trimmed = line.trim_start();
    if trimmed.starts_with("--") {
        return (line.to_string(), 0);
    }
    let mut result = String::with_capacity(line.len());
    let mut count = 0usize;
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;
    let mut in_string = false;
    while i < chars.len() {
        if chars[i] == '"' && (i == 0 || chars[i - 1] != '\\') {
            in_string = !in_string;
            result.push(chars[i]);
            i += 1;
        } else if !in_string && chars[i] == '=' && i + 1 < chars.len() && chars[i + 1] == '>' {
            if i > 0 && chars[i - 1] == '<' {
                result.push(chars[i]);
                i += 1;
            } else {
                result.push_str("->");
                count += 1;
                i += 2;
            }
        } else if !in_string && chars[i] == '-' && i + 1 < chars.len() && chars[i + 1] == '-' {
            for c in &chars[i..] {
                result.push(*c);
            }
            return (result, count);
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    (result, count)
}
/// Replace `::` (cons) with `List.cons` where appropriate.
/// Does NOT replace `::` inside qualified names like `Foo::bar`.
pub fn transform_cons(line: &str) -> (String, usize) {
    let trimmed = line.trim_start();
    if trimmed.starts_with("--") {
        return (line.to_string(), 0);
    }
    let mut result = String::with_capacity(line.len() + 16);
    let mut count = 0usize;
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;
    let mut in_string = false;
    while i < chars.len() {
        if chars[i] == '"' && (i == 0 || chars[i - 1] != '\\') {
            in_string = !in_string;
            result.push(chars[i]);
            i += 1;
        } else if !in_string && chars[i] == ':' && i + 1 < chars.len() && chars[i + 1] == ':' {
            let prev_alpha = i > 0 && (chars[i - 1].is_alphanumeric() || chars[i - 1] == '_');
            let next_alpha =
                i + 2 < chars.len() && (chars[i + 2].is_alphanumeric() || chars[i + 2] == '_');
            if prev_alpha && next_alpha {
                result.push_str("::");
                i += 2;
            } else {
                result.push_str("List.cons");
                count += 1;
                i += 2;
            }
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    (result, count)
}
/// Replace `++` with `List.append`.
pub fn transform_append(line: &str) -> (String, usize) {
    let trimmed = line.trim_start();
    if trimmed.starts_with("--") {
        return (line.to_string(), 0);
    }
    let mut result = String::with_capacity(line.len() + 16);
    let mut count = 0usize;
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;
    let mut in_string = false;
    while i < chars.len() {
        if chars[i] == '"' && (i == 0 || chars[i - 1] != '\\') {
            in_string = !in_string;
            result.push(chars[i]);
            i += 1;
        } else if !in_string && chars[i] == '+' && i + 1 < chars.len() && chars[i + 1] == '+' {
            if i + 2 < chars.len() && chars[i + 2] == '+' {
                result.push(chars[i]);
                i += 1;
            } else {
                result.push_str("List.append");
                count += 1;
                i += 2;
            }
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    (result, count)
}
/// Replace `∣` (divides, U+2223) with `Nat.dvd`.
pub fn transform_divides(line: &str) -> (String, usize) {
    let old = '\u{2223}';
    let replacement = "Nat.dvd";
    let count = line.chars().filter(|&c| c == old).count();
    if count == 0 {
        return (line.to_string(), 0);
    }
    let result = line.replace(old, replacement);
    (result, count)
}
/// Replace `↦` (mapsto, U+21A6) with `->`.
pub fn transform_mapsto(line: &str) -> (String, usize) {
    let old = '\u{21A6}';
    let replacement = "->";
    let count = line.chars().filter(|&c| c == old).count();
    if count == 0 {
        return (line.to_string(), 0);
    }
    let result = line.replace(old, replacement);
    (result, count)
}
/// Rename `.lean` imports to `.oxilean`.
pub fn transform_import_extension(line: &str) -> (String, usize) {
    let trimmed = line.trim();
    if trimmed.starts_with("import ") && trimmed.ends_with(".lean") {
        let replaced = line.replacen(".lean", ".oxilean", 1);
        return (replaced, 1);
    }
    (line.to_string(), 0)
}
/// Normalize `#check` / `#eval` / `#print` to lowercase.
pub fn transform_hash_commands(line: &str) -> (String, usize) {
    let trimmed = line.trim_start();
    let commands = ["#Check", "#Eval", "#Print"];
    for cmd in &commands {
        if trimmed.starts_with(cmd) {
            let lower = cmd.to_lowercase();
            let result = line.replacen(cmd, &lower, 1);
            return (result, 1);
        }
    }
    (line.to_string(), 0)
}
/// Apply a single migration rule to the full source text (all lines).
/// Returns the transformed text and a `RuleApplication` summary.
pub fn apply_migration_rule(source: &str, rule: &MigrationRule) -> (String, RuleApplication) {
    let mut total_changes = 0usize;
    let mut output_lines: Vec<String> = Vec::new();
    for line in source.lines() {
        let (transformed, changes) = rule.apply(line);
        total_changes += changes;
        output_lines.push(transformed);
    }
    let result = output_lines.join("\n");
    let result = if source.ends_with('\n') && !result.ends_with('\n') {
        result + "\n"
    } else {
        result
    };
    let app = RuleApplication {
        rule_name: rule.name.clone(),
        changes: total_changes,
    };
    (result, app)
}
/// Apply all rules from a registry to the source text, in priority order.
/// Returns the final transformed text and a list of all rule applications.
pub fn apply_all_rules(source: &str, registry: &RuleRegistry) -> (String, Vec<RuleApplication>) {
    let mut text = source.to_string();
    let mut apps = Vec::new();
    for rule in registry.iter() {
        let (new_text, app) = apply_migration_rule(&text, rule);
        text = new_text;
        apps.push(app);
    }
    (text, apps)
}
/// Migrate a single source string using the provided registry.
/// Returns the transformed source and the list of rule applications.
pub fn migrate_source(source: &str, registry: &RuleRegistry) -> (String, Vec<RuleApplication>) {
    apply_all_rules(source, registry)
}
/// Migrate all matching files from `config.source_dir` to `config.target_dir`.
///
/// In dry-run mode, no files are written but the report still reflects what
/// *would* have changed.
pub fn batch_migrate(config: &MigrationConfig, registry: &RuleRegistry) -> MigrationReport {
    let mut report = MigrationReport::new();
    let entries = match std::fs::read_dir(&config.source_dir) {
        Ok(e) => e,
        Err(e) => {
            report.record_error(
                config.source_dir.clone(),
                format!("cannot read source directory: {e}"),
            );
            return report;
        }
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_file() || !config.should_migrate(&path) {
            report.record_skip();
            continue;
        }
        let source = match std::fs::read_to_string(&path) {
            Ok(s) => s,
            Err(e) => {
                report.record_error(path.clone(), format!("read error: {e}"));
                continue;
            }
        };
        let (transformed, apps) = apply_all_rules(&source, registry);
        let had_changes = apps.iter().any(|a| a.changes > 0);
        report.record_file(&apps, had_changes);
        if config.verbose {
            let total: usize = apps.iter().map(|a| a.changes).sum();
            eprintln!("[migrate] {} : {} change(s)", path.display(), total);
        }
        if !config.dry_run && had_changes {
            let relative = path.strip_prefix(&config.source_dir).unwrap_or(&path);
            let mut target_path = config.target_dir.join(relative);
            target_path.set_extension("oxilean");
            if !config.overwrite && target_path.exists() {
                report.record_error(
                    target_path.clone(),
                    "target file exists and overwrite is disabled".to_string(),
                );
                continue;
            }
            if let Some(parent) = target_path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            if let Err(e) = std::fs::write(&target_path, &transformed) {
                report.record_error(target_path, format!("write error: {e}"));
            }
        }
    }
    report
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_transform_fat_arrow_basic() {
        let (out, count) = transform_fat_arrow("| x => x + 1");
        assert_eq!(out, "| x -> x + 1");
        assert_eq!(count, 1);
    }
    #[test]
    fn test_transform_fat_arrow_in_string() {
        let (out, count) = transform_fat_arrow(r#"let s := "a => b""#);
        assert_eq!(out, r#"let s := "a => b""#);
        assert_eq!(count, 0);
    }
    #[test]
    fn test_transform_fat_arrow_comment() {
        let (out, count) = transform_fat_arrow("-- this => that");
        assert_eq!(out, "-- this => that");
        assert_eq!(count, 0);
    }
    #[test]
    fn test_transform_fat_arrow_inline_comment() {
        let (out, count) = transform_fat_arrow("| x => x -- note: arrow");
        assert_eq!(out, "| x -> x -- note: arrow");
        assert_eq!(count, 1);
    }
    #[test]
    fn test_transform_cons_basic() {
        let (out, count) = transform_cons("h :: t");
        assert_eq!(out, "h List.cons t");
        assert_eq!(count, 1);
    }
    #[test]
    fn test_transform_cons_qualified_name() {
        let (out, count) = transform_cons("Foo::bar");
        assert_eq!(out, "Foo::bar");
        assert_eq!(count, 0);
    }
    #[test]
    fn test_transform_append_basic() {
        let (out, count) = transform_append("xs ++ ys");
        assert_eq!(out, "xs List.append ys");
        assert_eq!(count, 1);
    }
    #[test]
    fn test_transform_divides() {
        let (out, count) = transform_divides("m \u{2223} n");
        assert_eq!(out, "m Nat.dvd n");
        assert_eq!(count, 1);
    }
    #[test]
    fn test_transform_mapsto() {
        let (out, count) = transform_mapsto("x \u{21A6} y");
        assert_eq!(out, "x -> y");
        assert_eq!(count, 1);
    }
    #[test]
    fn test_transform_import_extension() {
        let (out, count) = transform_import_extension("import Mathlib.Data.Nat.Basic.lean");
        assert_eq!(out, "import Mathlib.Data.Nat.Basic.oxilean");
        assert_eq!(count, 1);
    }
    #[test]
    fn test_transform_hash_commands() {
        let (out, count) = transform_hash_commands("#Check Nat");
        assert_eq!(out, "#check Nat");
        assert_eq!(count, 1);
    }
    #[test]
    fn test_rule_registry_builtins() {
        let reg = RuleRegistry::with_builtins();
        assert!(reg.len() >= 7);
        assert!(reg.get("fat_arrow").is_some());
        assert!(reg.get("cons").is_some());
        assert!(reg.get("nonexistent").is_none());
    }
    #[test]
    fn test_rule_registry_ordering() {
        let reg = RuleRegistry::with_builtins();
        let rules: Vec<_> = reg.iter().collect();
        for window in rules.windows(2) {
            assert!(window[0].priority <= window[1].priority);
        }
    }
    #[test]
    fn test_rule_registry_remove() {
        let mut reg = RuleRegistry::with_builtins();
        let before = reg.len();
        assert!(reg.remove("cons"));
        assert_eq!(reg.len(), before - 1);
        assert!(reg.get("cons").is_none());
        assert!(!reg.remove("cons"));
    }
    #[test]
    fn test_apply_migration_rule() {
        let rule = MigrationRule::new(
            "fat_arrow",
            "Replace => with ->",
            RulePriority::Normal,
            transform_fat_arrow,
        );
        let source = "| x => x\n| y => y\n";
        let (result, app) = apply_migration_rule(source, &rule);
        assert_eq!(result, "| x -> x\n| y -> y\n");
        assert_eq!(app.rule_name, "fat_arrow");
        assert_eq!(app.changes, 2);
    }
    #[test]
    fn test_apply_all_rules() {
        let registry = RuleRegistry::with_builtins();
        let source = "fun x => h :: t";
        let (result, apps) = apply_all_rules(source, &registry);
        assert!(result.contains("->"));
        assert!(result.contains("List.cons"));
        assert!(!result.contains("=>"));
        let total: usize = apps.iter().map(|a| a.changes).sum();
        assert!(total >= 2);
    }
    #[test]
    fn test_migration_report() {
        let mut report = MigrationReport::new();
        let apps = vec![
            RuleApplication {
                rule_name: "fat_arrow".to_string(),
                changes: 3,
            },
            RuleApplication {
                rule_name: "cons".to_string(),
                changes: 1,
            },
        ];
        report.record_file(&apps, true);
        report.record_skip();
        assert_eq!(report.files_processed, 1);
        assert_eq!(report.files_changed, 1);
        assert_eq!(report.files_skipped, 1);
        assert_eq!(report.changes_made, 4);
        assert!(report.is_success());
        let summary = report.summary();
        assert!(summary.contains("Files processed: 1"));
        assert!(summary.contains("Total changes:   4"));
    }
    #[test]
    fn test_migration_report_errors() {
        let mut report = MigrationReport::new();
        report.record_error(PathBuf::from("/tmp/test.lean"), "read error".to_string());
        assert!(!report.is_success());
        assert_eq!(report.errors.len(), 1);
        let summary = report.summary();
        assert!(summary.contains("Errors (1)"));
    }
    #[test]
    fn test_migration_config_should_migrate() {
        let cfg = MigrationConfig::new("/src", "/dst");
        assert!(cfg.should_migrate(Path::new("foo.lean")));
        assert!(!cfg.should_migrate(Path::new("foo.rs")));
        assert!(!cfg.should_migrate(Path::new("foo")));
    }
    #[test]
    fn test_migration_config_builder() {
        let cfg = MigrationConfig::new("/src", "/dst")
            .with_dry_run(true)
            .with_verbose(true)
            .with_overwrite(true);
        assert!(cfg.dry_run);
        assert!(cfg.verbose);
        assert!(cfg.overwrite);
    }
    #[test]
    fn test_migrate_source_end_to_end() {
        let registry = RuleRegistry::with_builtins();
        let source = "def f := fun x => x\ndef g := h :: t ++ u\n#Check Nat\n";
        let (result, apps) = migrate_source(source, &registry);
        assert!(result.contains("fun x -> x"));
        assert!(result.contains("List.cons"));
        assert!(result.contains("List.append"));
        assert!(result.contains("#check"));
        let total: usize = apps.iter().map(|a| a.changes).sum();
        assert!(total >= 4);
    }
    #[test]
    fn test_fat_arrow_preserves_iff() {
        let (out, count) = transform_fat_arrow("A <=> B");
        assert_eq!(out, "A <=> B");
        assert_eq!(count, 0);
    }
    #[test]
    fn test_multiple_arrows_on_one_line() {
        let (out, count) = transform_fat_arrow("| x => | y => z");
        assert_eq!(out, "| x -> | y -> z");
        assert_eq!(count, 2);
    }
}
/// Replace Lean 3 `#reduce` with `#eval`.
fn transform_lean3_hash_commands(line: &str) -> (String, usize) {
    let trimmed = line.trim_start();
    if trimmed.starts_with("--") {
        return (line.to_string(), 0);
    }
    let mut result = line.to_string();
    let mut count = 0;
    if result.contains("#reduce ") {
        result = result.replace("#reduce ", "#eval ");
        count += 1;
    }
    if result.contains("#Print ") {
        result = result.replace("#Print ", "#print ");
        count += 1;
    }
    (result, count)
}
/// In Lean 3, `begin` blocks become `by` blocks.
fn transform_lean3_begin_end(line: &str) -> (String, usize) {
    let trimmed = line.trim();
    if trimmed == "begin" {
        let indent = &line[..line.len() - line.trim_start().len()];
        return (format!("{}by", indent), 1);
    }
    if trimmed == "end" {
        return (String::new(), 1);
    }
    (line.to_string(), 0)
}
/// Lean 3 `assume h : P,` → `intro h`.
fn transform_lean3_assume(line: &str) -> (String, usize) {
    let trimmed = line.trim_start();
    if trimmed.starts_with("assume ") {
        let indent = &line[..line.len() - trimmed.len()];
        let rest = &trimmed["assume ".len()..];
        let rest = rest.trim_end_matches(',').trim();
        return (format!("{}intro {}", indent, rest), 1);
    }
    (line.to_string(), 0)
}
/// Lean 3 `have h : P, from proof` → `have h : P := proof`.
fn transform_lean3_have_from(line: &str) -> (String, usize) {
    if let Some(pos) = line.find(", from ") {
        let before = &line[..pos];
        let after = &line[pos + ", from ".len()..];
        return (format!("{} := {}", before, after), 1);
    }
    (line.to_string(), 0)
}
/// Lean 3 `show P, from e` → `show P; exact e`.
fn transform_lean3_show_from(line: &str) -> (String, usize) {
    let trimmed = line.trim_start();
    if trimmed.starts_with("show ") {
        if let Some(pos) = trimmed.find(", from ") {
            let indent = &line[..line.len() - trimmed.len()];
            let ty = &trimmed["show ".len()..pos];
            let proof = &trimmed[pos + ", from ".len()..];
            return (format!("{}show {}; exact {}", indent, ty, proof), 1);
        }
    }
    (line.to_string(), 0)
}
/// Lean 3 theorem binder migration hint (no-op).
fn transform_lean3_theorem_binders(line: &str) -> (String, usize) {
    (line.to_string(), 0)
}
/// Build a `RuleRegistry` for Lean 3 → OxiLean migration.
pub fn lean3_rule_registry() -> RuleRegistry {
    let mut reg = RuleRegistry::with_builtins();
    reg.add(MigrationRule::new(
        "lean3_hash_cmds",
        "Replace #reduce with #eval",
        RulePriority::High,
        transform_lean3_hash_commands,
    ));
    reg.add(MigrationRule::new(
        "lean3_begin_end",
        "Replace begin/end with by blocks",
        RulePriority::High,
        transform_lean3_begin_end,
    ));
    reg.add(MigrationRule::new(
        "lean3_assume",
        "Replace assume h : P with intro h",
        RulePriority::Normal,
        transform_lean3_assume,
    ));
    reg.add(MigrationRule::new(
        "lean3_have_from",
        "Replace have h : P, from e with have h : P := e",
        RulePriority::Normal,
        transform_lean3_have_from,
    ));
    reg.add(MigrationRule::new(
        "lean3_show_from",
        "Replace show P, from e with show P; exact e",
        RulePriority::Normal,
        transform_lean3_show_from,
    ));
    reg.add(MigrationRule::new(
        "lean3_theorem_binders",
        "Hint about theorem binder migration",
        RulePriority::Low,
        transform_lean3_theorem_binders,
    ));
    reg
}
/// Replace Coq `Lemma foo : P.` with `theorem foo : P :=`.
fn transform_coq_lemma(line: &str) -> (String, usize) {
    let trimmed = line.trim_start();
    let indent = &line[..line.len() - trimmed.len()];
    for kw in &["Lemma ", "Theorem ", "Proposition ", "Corollary "] {
        if trimmed.starts_with(kw) {
            let rest = trimmed[kw.len()..].trim_end_matches('.').trim_end();
            return (format!("{}theorem {} :=", indent, rest), 1);
        }
    }
    (line.to_string(), 0)
}
/// Replace Coq `Definition foo := ...` with `def foo :=`.
fn transform_coq_definition(line: &str) -> (String, usize) {
    let trimmed = line.trim_start();
    let indent = &line[..line.len() - trimmed.len()];
    if trimmed.starts_with("Definition ") {
        let rest = &trimmed["Definition ".len()..];
        let rest = rest.trim_end_matches('.');
        return (format!("{}def {}", indent, rest), 1);
    }
    (line.to_string(), 0)
}
/// Remove Coq `Proof.` marker.
fn transform_coq_proof(line: &str) -> (String, usize) {
    let trimmed = line.trim();
    if trimmed == "Proof." {
        return (String::new(), 1);
    }
    (line.to_string(), 0)
}
/// Remove Coq `Qed.` / `Admitted.` markers.
fn transform_coq_qed(line: &str) -> (String, usize) {
    let trimmed = line.trim();
    if trimmed == "Qed." || trimmed == "Admitted." {
        return (String::new(), 1);
    }
    (line.to_string(), 0)
}
/// Remove trailing dots from tactic lines.
fn transform_coq_tactics_dot(line: &str) -> (String, usize) {
    let trimmed = line.trim_end();
    if trimmed.ends_with('.') && !trimmed.starts_with("--") {
        let without_dot = &trimmed[..trimmed.len() - 1];
        if let Some(first_word) = without_dot.trim_start().split_whitespace().next() {
            let coq_tactics = [
                "intros",
                "intro",
                "apply",
                "exact",
                "assumption",
                "rewrite",
                "unfold",
                "simpl",
                "ring",
                "tauto",
                "auto",
                "omega",
                "split",
                "left",
                "right",
                "exists",
                "induction",
                "destruct",
                "contradiction",
                "exfalso",
                "trivial",
            ];
            if coq_tactics.contains(&first_word) {
                return (without_dot.to_string(), 1);
            }
        }
    }
    (line.to_string(), 0)
}
/// Normalize Coq forall syntax (no-op placeholder).
fn transform_coq_forall(line: &str) -> (String, usize) {
    (line.to_string(), 0)
}
/// Build a `RuleRegistry` for Coq → OxiLean migration.
pub fn coq_rule_registry() -> RuleRegistry {
    let mut reg = RuleRegistry::new();
    reg.add(MigrationRule::new(
        "coq_lemma",
        "Convert Coq Lemma/Theorem to theorem",
        RulePriority::High,
        transform_coq_lemma,
    ));
    reg.add(MigrationRule::new(
        "coq_definition",
        "Convert Coq Definition to def",
        RulePriority::High,
        transform_coq_definition,
    ));
    reg.add(MigrationRule::new(
        "coq_proof",
        "Remove Proof. marker",
        RulePriority::Normal,
        transform_coq_proof,
    ));
    reg.add(MigrationRule::new(
        "coq_qed",
        "Remove Qed. / Admitted. markers",
        RulePriority::Normal,
        transform_coq_qed,
    ));
    reg.add(MigrationRule::new(
        "coq_tactics_dot",
        "Remove trailing dots from tactic lines",
        RulePriority::Normal,
        transform_coq_tactics_dot,
    ));
    reg.add(MigrationRule::new(
        "coq_forall",
        "Normalize Coq forall syntax",
        RulePriority::Low,
        transform_coq_forall,
    ));
    reg.add(MigrationRule::new(
        "fat_arrow",
        "Replace => with ->",
        RulePriority::Normal,
        transform_fat_arrow,
    ));
    reg
}
/// Replace Agda `data Foo : Set where` with `inductive Foo : Type where`.
fn transform_agda_data(line: &str) -> (String, usize) {
    let trimmed = line.trim_start();
    let indent = &line[..line.len() - trimmed.len()];
    if trimmed.starts_with("data ") && trimmed.contains(": Set") {
        let result = line
            .replace(": Set", ": Type")
            .replace("data ", "inductive ");
        return (result, 1);
    }
    if trimmed.starts_with("data ") && trimmed.contains(": Set\u{2081}") {
        let result = line
            .replace(": Set\u{2081}", ": Type 1")
            .replace("data ", "inductive ");
        return (format!("{}{}", indent, result.trim_start()), 1);
    }
    (line.to_string(), 0)
}
/// Replace Agda `record Foo : Set where` with `structure Foo : Type where`.
fn transform_agda_record(line: &str) -> (String, usize) {
    let trimmed = line.trim_start();
    if trimmed.starts_with("record ") {
        let result = line
            .replace("record ", "structure ")
            .replace(": Set", ": Type");
        return (result, 1);
    }
    (line.to_string(), 0)
}
/// Replace Agda `module Foo where` with `namespace Foo`.
fn transform_agda_module(line: &str) -> (String, usize) {
    let trimmed = line.trim_start();
    let indent = &line[..line.len() - trimmed.len()];
    if trimmed.starts_with("module ") && trimmed.ends_with(" where") {
        let name = trimmed["module ".len()..trimmed.len() - " where".len()].trim();
        return (format!("{}namespace {}", indent, name), 1);
    }
    (line.to_string(), 0)
}
/// Agda `open` passthrough.
fn transform_agda_open(line: &str) -> (String, usize) {
    (line.to_string(), 0)
}
/// Agda arrow passthrough.
fn transform_agda_arrow(line: &str) -> (String, usize) {
    (line.to_string(), 0)
}
/// Build a `RuleRegistry` for Agda → OxiLean migration.
pub fn agda_rule_registry() -> RuleRegistry {
    let mut reg = RuleRegistry::new();
    reg.add(MigrationRule::new(
        "agda_data",
        "Convert Agda data to inductive",
        RulePriority::High,
        transform_agda_data,
    ));
    reg.add(MigrationRule::new(
        "agda_record",
        "Convert Agda record to structure",
        RulePriority::High,
        transform_agda_record,
    ));
    reg.add(MigrationRule::new(
        "agda_module",
        "Convert Agda module ... where to namespace",
        RulePriority::High,
        transform_agda_module,
    ));
    reg.add(MigrationRule::new(
        "agda_open",
        "Normalize Agda open imports",
        RulePriority::Normal,
        transform_agda_open,
    ));
    reg.add(MigrationRule::new(
        "agda_arrow",
        "Normalize Agda arrows",
        RulePriority::Low,
        transform_agda_arrow,
    ));
    reg.add(MigrationRule::new(
        "fat_arrow",
        "Replace => with ->",
        RulePriority::Normal,
        transform_fat_arrow,
    ));
    reg
}
/// Rewrite a single import path.
pub fn rewrite_import_path(import_line: &str, path_map: &[(String, String)]) -> (String, bool) {
    let trimmed = import_line.trim();
    if !trimmed.starts_with("import ") {
        return (import_line.to_string(), false);
    }
    let path = trimmed["import ".len()..].trim();
    for (old, new) in path_map {
        if path.starts_with(old.as_str()) {
            let suffix = &path[old.len()..];
            let rewritten = format!("import {}{}", new, suffix);
            return (rewritten, true);
        }
    }
    (import_line.to_string(), false)
}
/// Rewrite all import paths in a source text.
pub fn rewrite_all_imports(source: &str, path_map: &[(String, String)]) -> (String, usize) {
    let mut count = 0;
    let lines: Vec<String> = source
        .lines()
        .map(|line| {
            let (new_line, changed) = rewrite_import_path(line, path_map);
            if changed {
                count += 1;
            }
            new_line
        })
        .collect();
    let result = lines.join("\n");
    let result = if source.ends_with('\n') && !result.ends_with('\n') {
        result + "\n"
    } else {
        result
    };
    (result, count)
}
/// Rewrite namespace declarations.
pub fn rewrite_namespaces(source: &str, ns_map: &[(String, String)]) -> (String, usize) {
    let mut count = 0;
    let lines: Vec<String> = source
        .lines()
        .map(|line| {
            let trimmed = line.trim_start();
            let indent = &line[..line.len() - trimmed.len()];
            for (old_ns, new_ns) in ns_map {
                let prefix = format!("namespace {}", old_ns);
                if trimmed.starts_with(&prefix) {
                    let suffix = &trimmed[prefix.len()..];
                    count += 1;
                    return format!("{}namespace {}{}", indent, new_ns, suffix);
                }
                let open_prefix = format!("open {}", old_ns);
                if trimmed.starts_with(&open_prefix) {
                    let suffix = &trimmed[open_prefix.len()..];
                    count += 1;
                    return format!("{}open {}{}", indent, new_ns, suffix);
                }
            }
            line.to_string()
        })
        .collect();
    let result = lines.join("\n");
    let result = if source.ends_with('\n') && !result.ends_with('\n') {
        result + "\n"
    } else {
        result
    };
    (result, count)
}
/// Built-in OxiLean v0.1 → v0.2 API rewrites.
pub fn oxilean_v0_1_to_v0_2_rewrites() -> ApiRewriteSet {
    let mut set = ApiRewriteSet::new("OxiLean v0.1 to v0.2");
    set.add(ApiRewrite::new("Nat.succ_pos", "Nat.succ_pos'").with_message("renamed in v0.2"));
    set.add(
        ApiRewrite::new("List.length_cons", "List.length_cons'")
            .with_message("generalized signature"),
    );
    set.add(ApiRewrite::new("Eq.mpr", "Eq.mpr'").with_message("API breaking change"));
    set
}
/// Validate a migrated OxiLean source.
pub fn validate_migrated_source(path: PathBuf, source: &str) -> ValidationResult {
    let mut result = ValidationResult::pass(path);
    for (lineno, line) in source.lines().enumerate() {
        let lineno = lineno + 1;
        let trimmed = line.trim();
        if trimmed.contains(" => ") && !trimmed.starts_with("--") {
            result.add_message(
                ValidationMessage::warning("possible residual `=>` arrow (should be `->`)")
                    .at_line(lineno),
            );
        }
        if trimmed.starts_with("#Check ") {
            result.add_message(
                ValidationMessage::warning("#Check should be #check (lowercase)").at_line(lineno),
            );
        }
        if trimmed == "Proof." || trimmed == "Qed." {
            result.add_message(
                ValidationMessage::error("residual Coq syntax: Proof. / Qed.").at_line(lineno),
            );
        }
        if trimmed.starts_with("module ") && trimmed.ends_with(" where") {
            result.add_message(
                ValidationMessage::warning(
                    "possible Agda `module ... where` (should be `namespace`)",
                )
                .at_line(lineno),
            );
        }
    }
    result
}
/// Compute proposed changes without applying them.
pub fn compute_proposed_changes(source: &str, registry: &RuleRegistry) -> Vec<ProposedChange> {
    let mut proposed = Vec::new();
    for rule in registry.iter() {
        for (lineno, line) in source.lines().enumerate() {
            let (transformed, changes) = rule.apply(line);
            if changes > 0 && transformed != line {
                proposed.push(ProposedChange::new(
                    lineno + 1,
                    &rule.name,
                    line,
                    transformed,
                ));
            }
        }
    }
    proposed
}
/// Apply a subset of proposed changes.
pub fn apply_accepted_changes(
    source: &str,
    proposed: &[ProposedChange],
    accepted: &[usize],
) -> String {
    let accepted_set: std::collections::HashSet<usize> = accepted.iter().copied().collect();
    let line_map: HashMap<usize, &str> = proposed
        .iter()
        .enumerate()
        .filter(|(idx, _)| accepted_set.contains(idx))
        .map(|(_, change)| (change.line - 1, change.proposed.as_str()))
        .collect();
    let result: Vec<String> = source
        .lines()
        .enumerate()
        .map(|(i, line)| {
            if let Some(replacement) = line_map.get(&i) {
                replacement.to_string()
            } else {
                line.to_string()
            }
        })
        .collect();
    let joined = result.join("\n");
    if source.ends_with('\n') && !joined.ends_with('\n') {
        joined + "\n"
    } else {
        joined
    }
}
#[cfg(test)]
mod expanded_tests {
    use super::*;
    #[test]
    fn test_lean3_hash_commands() {
        let (out, count) = transform_lean3_hash_commands("#reduce 1 + 1");
        assert_eq!(out, "#eval 1 + 1");
        assert_eq!(count, 1);
    }
    #[test]
    fn test_lean3_begin_end_begin() {
        let (out, count) = transform_lean3_begin_end("begin");
        assert!(out.contains("by"));
        assert_eq!(count, 1);
    }
    #[test]
    fn test_lean3_begin_end_end() {
        let (out, count) = transform_lean3_begin_end("end");
        assert_eq!(out, "");
        assert_eq!(count, 1);
    }
    #[test]
    fn test_lean3_assume() {
        let (out, count) = transform_lean3_assume("assume h : P,");
        assert!(out.contains("intro"));
        assert_eq!(count, 1);
    }
    #[test]
    fn test_lean3_have_from() {
        let (out, count) = transform_lean3_have_from("have h : P, from e");
        assert!(out.contains(":="));
        assert!(!out.contains(", from "));
        assert_eq!(count, 1);
    }
    #[test]
    fn test_lean3_show_from() {
        let (out, count) = transform_lean3_show_from("show P, from e");
        assert!(out.contains("show P"));
        assert!(out.contains("exact e"));
        assert_eq!(count, 1);
    }
    #[test]
    fn test_lean3_rule_registry_has_rules() {
        let reg = lean3_rule_registry();
        assert!(reg.len() >= 6);
    }
    #[test]
    fn test_coq_lemma() {
        let (out, count) = transform_coq_lemma("Lemma foo : P -> Q.");
        assert!(out.contains("theorem"));
        assert!(!out.contains("Lemma"));
        assert_eq!(count, 1);
    }
    #[test]
    fn test_coq_definition() {
        let (out, count) = transform_coq_definition("Definition myDef := 42.");
        assert!(out.contains("def"));
        assert!(!out.contains("Definition"));
        assert_eq!(count, 1);
    }
    #[test]
    fn test_coq_proof() {
        let (out, count) = transform_coq_proof("Proof.");
        assert_eq!(out, "");
        assert_eq!(count, 1);
    }
    #[test]
    fn test_coq_qed() {
        let (out, count) = transform_coq_qed("Qed.");
        assert_eq!(out, "");
        assert_eq!(count, 1);
    }
    #[test]
    fn test_coq_admitted() {
        let (out, count) = transform_coq_qed("Admitted.");
        assert_eq!(out, "");
        assert_eq!(count, 1);
    }
    #[test]
    fn test_coq_tactics_dot() {
        let (out, count) = transform_coq_tactics_dot("  intros h1 h2.");
        assert!(!out.ends_with('.'));
        assert_eq!(count, 1);
    }
    #[test]
    fn test_coq_rule_registry_has_rules() {
        let reg = coq_rule_registry();
        assert!(reg.len() >= 5);
    }
    #[test]
    fn test_agda_data() {
        let (out, count) = transform_agda_data("data Foo : Set where");
        assert!(out.contains("inductive"));
        assert!(out.contains("Type"));
        assert_eq!(count, 1);
    }
    #[test]
    fn test_agda_record() {
        let (out, count) = transform_agda_record("record Bar : Set where");
        assert!(out.contains("structure"));
        assert_eq!(count, 1);
    }
    #[test]
    fn test_agda_module() {
        let (out, count) = transform_agda_module("module MyMod where");
        assert!(out.contains("namespace MyMod"));
        assert_eq!(count, 1);
    }
    #[test]
    fn test_agda_rule_registry_has_rules() {
        let reg = agda_rule_registry();
        assert!(reg.len() >= 5);
    }
    #[test]
    fn test_rewrite_import_path_match() {
        let map = vec![("Mathlib".to_string(), "OxiLean".to_string())];
        let (out, changed) = rewrite_import_path("import Mathlib.Data.Nat", &map);
        assert!(changed);
        assert!(out.contains("OxiLean"));
    }
    #[test]
    fn test_rewrite_import_path_no_match() {
        let map = vec![("Mathlib".to_string(), "OxiLean".to_string())];
        let (out, changed) = rewrite_import_path("import Other.Stuff", &map);
        assert!(!changed);
        assert!(out.contains("Other.Stuff"));
    }
    #[test]
    fn test_rewrite_all_imports() {
        let src = "import Mathlib.A\nimport Other.B\nimport Mathlib.C\n";
        let map = vec![("Mathlib".to_string(), "OxiLean".to_string())];
        let (out, count) = rewrite_all_imports(src, &map);
        assert_eq!(count, 2);
        assert!(out.contains("import OxiLean.A"));
        assert!(out.contains("import OxiLean.C"));
        assert!(out.contains("import Other.B"));
    }
    #[test]
    fn test_rewrite_namespaces() {
        let src = "namespace OldNs\ndef foo := 1\n";
        let map = vec![("OldNs".to_string(), "NewNs".to_string())];
        let (out, count) = rewrite_namespaces(src, &map);
        assert!(count >= 1);
        assert!(out.contains("namespace NewNs"));
    }
    #[test]
    fn test_api_rewrite_apply() {
        let rw = ApiRewrite::new("old_func", "new_func");
        let (out, count) = rw.apply_to_line("  old_func arg1 arg2");
        assert_eq!(count, 1);
        assert!(out.contains("new_func"));
        assert!(!out.contains("old_func"));
    }
    #[test]
    fn test_api_rewrite_skips_comment() {
        let rw = ApiRewrite::new("old_func", "new_func");
        let (out, count) = rw.apply_to_line("-- old_func in a comment");
        assert_eq!(count, 0);
        assert_eq!(out, "-- old_func in a comment");
    }
    #[test]
    fn test_api_rewrite_set() {
        let mut set = ApiRewriteSet::new("test set");
        set.add(ApiRewrite::new("old1", "new1"));
        set.add(ApiRewrite::new("old2", "new2"));
        let src = "old1 old2 old1";
        let (out, count) = set.apply_to_source(src);
        assert_eq!(count, 3);
        assert!(out.contains("new1"));
        assert!(out.contains("new2"));
    }
    #[test]
    fn test_api_rewrite_set_describe() {
        let set = oxilean_v0_1_to_v0_2_rewrites();
        let desc = set.describe();
        assert!(desc.contains("v0.1"));
        assert!(desc.contains("->"));
    }
    #[test]
    fn test_migration_snapshot() {
        let snap = MigrationSnapshot::new(
            PathBuf::from("/tmp/foo.lean"),
            "original".to_string(),
            "migrated".to_string(),
            vec![RuleApplication {
                rule_name: "fat_arrow".to_string(),
                changes: 1,
            }],
        );
        assert!(snap.has_changes());
        assert_eq!(snap.total_changes(), 1);
        assert_eq!(snap.rollback_content(), "original");
    }
    #[test]
    fn test_migration_session_process() {
        let mut session = MigrationSession::new();
        let registry = RuleRegistry::with_builtins();
        let path = PathBuf::from("/tmp/test.lean");
        let source = "fun x => x".to_string();
        let snap = session.process(path, source, &registry);
        assert!(snap.migrated_content.contains("->"));
        assert_eq!(session.len(), 1);
    }
    #[test]
    fn test_migration_session_dry_run_report() {
        let mut session = MigrationSession::new();
        let registry = RuleRegistry::with_builtins();
        let path = PathBuf::from("/tmp/test.lean");
        let _ = session.process(path, "fun x => x".to_string(), &registry);
        let report = session.dry_run_report();
        assert!(report.contains("Dry-run"));
    }
    #[test]
    fn test_validate_clean_source() {
        let source = "def foo : Nat := 42\n";
        let result = validate_migrated_source(PathBuf::from("test.oxilean"), source);
        assert!(result.passed);
        assert!(result.messages.is_empty());
    }
    #[test]
    fn test_validate_residual_fat_arrow() {
        let source = "| x => x\n";
        let result = validate_migrated_source(PathBuf::from("test.oxilean"), source);
        assert!(result
            .messages
            .iter()
            .any(|m| m.severity == ValidationSeverity::Warning));
    }
    #[test]
    fn test_validate_coq_syntax() {
        let source = "Qed.\n";
        let result = validate_migrated_source(PathBuf::from("test.oxilean"), source);
        assert!(!result.passed);
    }
    #[test]
    fn test_compute_proposed_changes() {
        let registry = RuleRegistry::with_builtins();
        let source = "| x => x\n| y => y\n";
        let proposed = compute_proposed_changes(source, &registry);
        assert!(!proposed.is_empty());
    }
    #[test]
    fn test_apply_accepted_changes() {
        let registry = RuleRegistry::with_builtins();
        let source = "| x => x\nno change\n";
        let proposed = compute_proposed_changes(source, &registry);
        assert!(!proposed.is_empty());
        let accepted: Vec<usize> = (0..proposed.len()).collect();
        let result = apply_accepted_changes(source, &proposed, &accepted);
        assert!(!result.is_empty());
    }
    #[test]
    fn test_extended_migration_report() {
        let mut report = ExtendedMigrationReport::new();
        let apps = vec![RuleApplication {
            rule_name: "fat_arrow".to_string(),
            changes: 3,
        }];
        report.record_file_migration(PathBuf::from("foo.lean"), &apps, true, Some(true));
        assert_eq!(report.base.files_changed, 1);
        let summary = report.detailed_summary();
        assert!(summary.contains("foo.lean"));
        assert!(summary.contains("validated OK"));
    }
    #[test]
    fn test_version_parse() {
        let v = Version::parse("1.2.3").expect("parsing should succeed");
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 2);
        assert_eq!(v.patch, 3);
    }
    #[test]
    fn test_version_display() {
        let v = Version {
            major: 0,
            minor: 1,
            patch: 0,
        };
        assert_eq!(v.to_string(), "0.1.0");
    }
    #[test]
    fn test_version_parse_invalid() {
        assert!(Version::parse("1.2").is_none());
        assert!(Version::parse("a.b.c").is_none());
    }
    #[test]
    fn test_version_migration_chain() {
        let from = Version::parse("0.1.0").expect("parsing should succeed");
        let to = Version::parse("0.2.0").expect("parsing should succeed");
        let set = oxilean_v0_1_to_v0_2_rewrites();
        let step = VersionMigrationStep::new(from, to, set);
        let mut chain = VersionMigrationChain::new();
        chain.add_step(step);
        assert_eq!(chain.len(), 1);
        let summary = chain.summary();
        assert!(summary.contains("0.1.0"));
        assert!(summary.contains("0.2.0"));
    }
    #[test]
    fn test_version_migration_chain_apply() {
        let from = Version::parse("0.1.0").expect("parsing should succeed");
        let to = Version::parse("0.2.0").expect("parsing should succeed");
        let mut set = ApiRewriteSet::new("test");
        set.add(ApiRewrite::new("old_api", "new_api"));
        let step = VersionMigrationStep::new(from, to, set);
        let mut chain = VersionMigrationChain::new();
        chain.add_step(step);
        let src = "def x := old_api 1\n";
        let (out, count) = chain.apply(src);
        assert_eq!(count, 1);
        assert!(out.contains("new_api"));
    }
    #[test]
    fn test_source_language_name() {
        assert_eq!(SourceLanguage::Lean4.name(), "Lean 4");
        assert_eq!(SourceLanguage::Coq.name(), "Coq");
        assert_eq!(SourceLanguage::Agda.name(), "Agda");
    }
    #[test]
    fn test_source_language_extensions() {
        let exts = SourceLanguage::Coq.extensions();
        assert!(exts.contains(&".v"));
        let exts = SourceLanguage::Agda.extensions();
        assert!(exts.contains(&".agda"));
    }
    #[test]
    fn test_proposed_change_display() {
        let change = ProposedChange::new(5, "fat_arrow", "| x => x", "| x -> x");
        let s = change.display();
        assert!(s.contains("Line 5"));
        assert!(s.contains("fat_arrow"));
    }
    #[test]
    fn test_api_rewrite_with_message() {
        let rw = ApiRewrite::new("old", "new").with_message("deprecated in v2");
        assert!(rw.message.is_some());
        assert_eq!(
            rw.message
                .as_deref()
                .expect("type conversion should succeed"),
            "deprecated in v2"
        );
    }
    #[test]
    fn test_validation_message_at_line() {
        let msg = ValidationMessage::warning("test").at_line(42);
        assert_eq!(msg.line, Some(42));
    }
    #[test]
    fn test_validation_result_pass() {
        let r = ValidationResult::pass(PathBuf::from("test.oxilean"));
        assert!(r.passed);
        assert!(r.messages.is_empty());
    }
    #[test]
    fn test_validation_result_fail() {
        let r = ValidationResult::fail(PathBuf::from("test.oxilean"), "bad syntax");
        assert!(!r.passed);
        assert_eq!(r.messages.len(), 1);
    }
    #[test]
    fn test_migration_session_empty() {
        let session = MigrationSession::new();
        assert!(session.is_empty());
        assert_eq!(session.len(), 0);
    }
    #[test]
    fn test_version_migration_chain_empty() {
        let chain = VersionMigrationChain::new();
        assert!(chain.is_empty());
        let (out, count) = chain.apply("unchanged");
        assert_eq!(out, "unchanged");
        assert_eq!(count, 0);
    }
}
#[allow(dead_code)]
pub fn migration_version_string(version: u32) -> String {
    format!("v{:04}", version)
}
#[allow(dead_code)]
pub fn parse_migration_version(s: &str) -> Option<u32> {
    s.strip_prefix('v')?.parse().ok()
}
#[cfg(test)]
mod migrate_extra_tests {
    use super::*;
    #[test]
    fn test_migration_status_success() {
        assert!(MigrationStatus::Applied.is_success());
        assert!(MigrationStatus::Skipped("already done".to_string()).is_success());
        assert!(!MigrationStatus::Failed("err".to_string()).is_success());
    }
    #[test]
    fn test_migration_record() {
        let mut r = MigrationRecord::new(1, "add_index");
        r.mark_applied(12345);
        assert_eq!(r.status, MigrationStatus::Applied);
        assert_eq!(r.applied_at_ms, Some(12345));
    }
    #[test]
    fn test_version_string() {
        assert_eq!(migration_version_string(42), "v0042");
        assert_eq!(parse_migration_version("v0042"), Some(42));
        assert_eq!(parse_migration_version("bad"), None);
    }
}
