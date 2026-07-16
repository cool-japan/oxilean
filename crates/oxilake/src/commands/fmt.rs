//! `oxilake fmt` — format all `.lean` source files in the project.
//!
//! Uses the `oxilean_parse` AST-based formatter (`format_module`) when a file
//! parses cleanly, and a lightweight normalisation pass as a fallback for files
//! that contain parse errors.
//!
//! The `--check` flag exits with a non-zero code and prints which files *would*
//! be changed, without modifying them on disk.

use crate::manifest::OxilakeManifest;
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

/// Format (or check) all `.lean` files in the package at `manifest_path`.
///
/// # Parameters
/// - `manifest_path`: Path to `oxilake.toml`.
/// - `check_only`: When `true`, report files that need formatting without
///   writing them.
///
/// # Errors
/// Returns an error when `check_only` is `true` and at least one file needs
/// formatting, or when any I/O error occurs.
pub fn run(manifest_path: &Path, check_only: bool) -> Result<()> {
    let manifest = OxilakeManifest::load(manifest_path)
        .with_context(|| format!("loading manifest {}", manifest_path.display()))?;

    let project_root = manifest_path.parent().unwrap_or_else(|| Path::new("."));

    println!(
        "Formatting '{}' v{} ...",
        manifest.package.name, manifest.package.version
    );

    let source_files = find_lean_files(project_root)?;
    let mut changed = 0usize;

    for file in &source_files {
        let original =
            std::fs::read_to_string(file).with_context(|| format!("reading {}", file.display()))?;

        let formatted = format_source(&original);

        if formatted != original {
            if check_only {
                eprintln!("  Would format: {}", file.display());
                changed += 1;
            } else {
                std::fs::write(file, formatted.as_bytes())
                    .with_context(|| format!("writing {}", file.display()))?;
                println!("  Formatted: {}", file.display());
                changed += 1;
            }
        }
    }

    if check_only && changed > 0 {
        return Err(anyhow::anyhow!(
            "{} file(s) need formatting (run `oxilake fmt` to fix)",
            changed
        ));
    }

    if changed == 0 {
        println!("  All files already formatted.");
    } else {
        println!("  Formatted {changed} file(s).");
    }

    Ok(())
}

// ─────────────────────────────────────────────────────────────────────────────
// Source formatter
// ─────────────────────────────────────────────────────────────────────────────

/// Format a `.lean` source string.
///
/// First attempts an AST round-trip using `oxilean_parse::core_types::parse_file`
/// followed by `oxilean_parse::formatter_adv::format_module`.  If parsing fails,
/// falls back to `normalise_source` which is a text-level normalisation that is
/// guaranteed to be idempotent.
///
/// The result always ends with exactly one newline (`'\n'`).
pub(crate) fn format_source(source: &str) -> String {
    // Try the AST-based path first.
    match try_ast_format(source) {
        Some(formatted) => ensure_trailing_newline(&formatted),
        None => {
            // Parser failed (e.g. partial/broken source) — fall back to
            // text-level normalisation.
            ensure_trailing_newline(&normalise_source(source))
        }
    }
}

/// Attempt to format `source` via the AST pipeline.
///
/// Returns `None` if lexing or parsing fails.
fn try_ast_format(source: &str) -> Option<String> {
    use oxilean_parse::core_types::parse_file;
    use oxilean_parse::formatter_adv::format_module;

    // Empty files are trivially formatted.
    if source.trim().is_empty() {
        return Some(String::new());
    }

    let decls = parse_file(source).ok()?;

    // format_module returns a String (the formatted source).
    let formatted = format_module(&decls);

    // If the formatter returned an empty string for non-empty source, fall
    // back rather than deleting content.
    if formatted.trim().is_empty() && !source.trim().is_empty() {
        return None;
    }

    Some(formatted)
}

/// Text-level source normalisation (fallback when AST parsing fails).
///
/// This is a conservative, **idempotent** transformation:
/// - Trailing whitespace is stripped from every line.
/// - Runs of multiple consecutive blank lines are collapsed to at most one.
/// - The result does *not* have a trailing newline (that is added by the
///   caller via `ensure_trailing_newline`).
fn normalise_source(source: &str) -> String {
    let mut out = String::with_capacity(source.len());
    let mut prev_blank = false;

    for line in source.lines() {
        let trimmed = line.trim_end();
        let is_blank = trimmed.is_empty();

        // Collapse two-or-more consecutive blank lines to one.
        if is_blank && prev_blank {
            continue;
        }

        if !out.is_empty() {
            out.push('\n');
        }
        out.push_str(trimmed);
        prev_blank = is_blank;
    }

    out
}

/// Ensure that `s` ends with exactly one `'\n'`.
fn ensure_trailing_newline(s: &str) -> String {
    if s.is_empty() {
        return "\n".to_string();
    }
    let trimmed = s.trim_end_matches('\n');
    format!("{trimmed}\n")
}

// ─────────────────────────────────────────────────────────────────────────────
// File discovery
// ─────────────────────────────────────────────────────────────────────────────

/// Recursively collect all `.lean` files under `dir`, skipping `target/`.
fn find_lean_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut result = Vec::new();
    collect_lean_files(dir, &mut result)?;
    Ok(result)
}

fn collect_lean_files(dir: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
    for entry in
        std::fs::read_dir(dir).with_context(|| format!("reading directory {}", dir.display()))?
    {
        let entry = entry.with_context(|| format!("reading entry in {}", dir.display()))?;
        let path = entry.path();

        if path.is_dir() {
            // Skip `target/` to avoid touching build artefacts.
            if path.file_name().map(|n| n == "target").unwrap_or(false) {
                continue;
            }
            collect_lean_files(&path, out)?;
        } else if path.extension().map(|e| e == "lean").unwrap_or(false) {
            out.push(path);
        }
    }
    Ok(())
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;

    // ── normalise_source ─────────────────────────────────────────────────────

    #[test]
    fn test_normalise_trailing_whitespace_stripped() {
        let src = "def foo :=   \ndef bar :=  \n";
        let norm = normalise_source(src);
        for line in norm.lines() {
            assert!(
                !line.ends_with(' '),
                "trailing space found in line: {line:?}"
            );
        }
    }

    #[test]
    fn test_normalise_collapses_consecutive_blank_lines() {
        let src = "a\n\n\n\nb";
        let norm = normalise_source(src);
        assert!(
            !norm.contains("\n\n\n"),
            "three consecutive newlines remain"
        );
    }

    #[test]
    fn test_normalise_idempotent() {
        let src = "  hello   \n\n\nworld  \n";
        let first = normalise_source(src);
        let second = normalise_source(&first);
        assert_eq!(first, second, "normalise_source is not idempotent");
    }

    // ── format_source (public API) ────────────────────────────────────────────

    #[test]
    fn test_format_source_trailing_newline() {
        let src = "-- just a comment";
        let formatted = format_source(src);
        assert!(
            formatted.ends_with('\n'),
            "formatted source must end with newline"
        );
    }

    #[test]
    fn test_format_source_idempotent_comment_only() {
        // Comments-only files won't parse as AST decls, so the fallback path is
        // exercised.  It must still be idempotent.
        let src = "-- hello\n-- world\n";
        let first = format_source(src);
        let second = format_source(&first);
        assert_eq!(first, second, "format_source must be idempotent");
    }

    #[test]
    fn test_format_source_idempotent_valid_lean() {
        // A syntactically valid declaration exercises the AST path.
        let src = "def hello : Nat := 42\n";
        let first = format_source(src);
        let second = format_source(&first);
        assert_eq!(first, second, "AST format_source must be idempotent");
    }

    #[test]
    fn test_format_source_empty() {
        let formatted = format_source("");
        assert!(formatted.ends_with('\n'));
    }

    // ── find_lean_files ──────────────────────────────────────────────────────

    #[test]
    fn test_find_lean_files_skips_target() {
        let dir = env::temp_dir().join("oxilake_fmt_find_lean_6543");
        fs::remove_dir_all(&dir).ok();
        fs::create_dir_all(dir.join("src")).expect("create src");
        fs::create_dir_all(dir.join("target")).expect("create target");
        fs::write(dir.join("src").join("Foo.lean"), "-- foo\n").expect("write Foo.lean");
        fs::write(dir.join("target").join("Bar.lean"), "-- bar\n").expect("write Bar.lean");

        let files = find_lean_files(&dir).expect("find_lean_files");
        assert!(
            files
                .iter()
                .all(|f| !f.to_string_lossy().contains("target")),
            "target/ directory should be skipped; found: {files:?}"
        );
        assert!(
            files.iter().any(|f| f.to_string_lossy().contains("Foo")),
            "Foo.lean should be found"
        );

        fs::remove_dir_all(&dir).ok();
    }

    // ── run (integration) ────────────────────────────────────────────────────

    #[test]
    fn test_run_formats_files() {
        let dir = env::temp_dir().join("oxilake_fmt_run_7890");
        fs::remove_dir_all(&dir).ok();
        fs::create_dir_all(dir.join("src")).expect("create src");

        fs::write(
            dir.join("oxilake.toml"),
            "[package]\nname = \"fmt-test\"\nversion = \"0.1.0\"\nlean_version = \"0.1\"\n",
        )
        .expect("write manifest");

        // Write a file that needs trailing-whitespace normalisation.
        let unformatted = "-- hello   \n-- world\n";
        fs::write(dir.join("src").join("A.lean"), unformatted).expect("write A.lean");

        let result = run(&dir.join("oxilake.toml"), false);
        assert!(result.is_ok(), "fmt run should succeed: {:?}", result.err());

        let content = fs::read_to_string(dir.join("src").join("A.lean")).expect("read A.lean");
        assert!(
            !content.contains("   \n"),
            "trailing spaces should have been removed"
        );

        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn test_run_check_detects_unformatted() {
        let dir = env::temp_dir().join("oxilake_fmt_check_3141");
        fs::remove_dir_all(&dir).ok();
        fs::create_dir_all(dir.join("src")).expect("create src");

        fs::write(
            dir.join("oxilake.toml"),
            "[package]\nname = \"fmt-check\"\nversion = \"0.1.0\"\nlean_version = \"0.1\"\n",
        )
        .expect("write manifest");

        // A file with trailing spaces — needs formatting.
        fs::write(dir.join("src").join("B.lean"), "-- trailing   \n").expect("write B.lean");

        let result = run(&dir.join("oxilake.toml"), true);
        assert!(
            result.is_err(),
            "check mode should fail on unformatted file"
        );

        // File must NOT have been modified.
        let content = fs::read_to_string(dir.join("src").join("B.lean")).expect("read B.lean");
        assert!(
            content.contains("trailing   "),
            "check mode must not modify files"
        );

        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn test_run_check_passes_on_already_formatted() {
        let dir = env::temp_dir().join("oxilake_fmt_check_ok_2718");
        fs::remove_dir_all(&dir).ok();
        fs::create_dir_all(dir.join("src")).expect("create src");

        fs::write(
            dir.join("oxilake.toml"),
            "[package]\nname = \"fmt-check-ok\"\nversion = \"0.1.0\"\nlean_version = \"0.1\"\n",
        )
        .expect("write manifest");

        // Write a comment-only file that is already normalised.
        let src = "-- clean file\n";
        fs::write(dir.join("src").join("C.lean"), src).expect("write C.lean");
        // Verify it is actually already in its formatted form.
        assert_eq!(format_source(src), src);

        let result = run(&dir.join("oxilake.toml"), true);
        assert!(
            result.is_ok(),
            "check mode should pass on already-formatted files: {:?}",
            result.err()
        );

        fs::remove_dir_all(&dir).ok();
    }
}
