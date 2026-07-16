//! Project symbol provider — scans .oxilean source files for declaration names.
//!
//! This module provides lightweight shell-completion integration for project symbols.
//! It uses simple line-prefix scanning (no full parse) to extract top-level declaration
//! names from `.oxilean` and `.lean` source files.

use super::types::{CompletionCandidate, DynamicCompletionRegistry};
use std::path::Path;

/// Recognised top-level declaration prefixes (keyword + space).
const DECL_PREFIXES: &[(&str, &str)] = &[
    ("noncomputable def ", "noncomputable definition"),
    ("private def ", "private definition"),
    ("protected def ", "protected definition"),
    ("theorem ", "theorem"),
    ("def ", "definition"),
    ("lemma ", "lemma"),
    ("axiom ", "axiom"),
    ("class ", "type class"),
    ("instance ", "instance"),
    ("abbrev ", "abbreviation"),
    ("structure ", "structure"),
    ("inductive ", "inductive type"),
];

/// Extract declaration names from a single source file using line-prefix scanning.
///
/// Returns a `Vec` of `(name, kind)` pairs for every top-level declaration found.
/// Lines starting with `--` (after trimming leading whitespace) are skipped.
pub fn extract_decl_names_from_source(source: &str) -> Vec<(String, &'static str)> {
    let mut results = Vec::new();
    for line in source.lines() {
        let trimmed = line.trim_start();
        // Skip comments
        if trimmed.starts_with("--") {
            continue;
        }
        for (prefix, kind) in DECL_PREFIXES {
            if let Some(rest) = trimmed.strip_prefix(prefix) {
                // Name ends at whitespace, '(', ':', or '{'
                let name: String = rest
                    .chars()
                    .take_while(|c| !c.is_whitespace() && *c != '(' && *c != ':' && *c != '{')
                    .collect();
                if !name.is_empty()
                    && name
                        .chars()
                        .next()
                        .map(|c| c.is_alphabetic() || c == '_')
                        .unwrap_or(false)
                {
                    results.push((name, *kind));
                }
                break;
            }
        }
    }
    results
}

/// Scan an oxilean project directory and collect declaration names.
///
/// Walks `project_dir` recursively up to `max_depth` levels, looking for
/// `*.oxilean` and `*.lean` files. Returns one `CompletionCandidate` per
/// declaration name found. Results are sorted by priority then name, and
/// deduplicated by text.
pub fn project_symbol_candidates(project_dir: &Path, max_depth: usize) -> Vec<CompletionCandidate> {
    let mut candidates: Vec<CompletionCandidate> = Vec::new();
    collect_from_dir(project_dir, max_depth, 0, &mut candidates);
    candidates.sort_by(|a, b| a.priority.cmp(&b.priority).then(a.text.cmp(&b.text)));
    candidates.dedup_by(|a, b| a.text == b.text);
    candidates
}

fn collect_from_dir(
    dir: &Path,
    max_depth: usize,
    depth: usize,
    out: &mut Vec<CompletionCandidate>,
) {
    if depth > max_depth {
        return;
    }
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() {
            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
            if ext == "oxilean" || ext == "lean" {
                if let Ok(source) = std::fs::read_to_string(&path) {
                    let file_label = path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("")
                        .to_owned();
                    let decls = extract_decl_names_from_source(&source);
                    for (name, kind) in decls {
                        let desc = format!("{} — {}", kind, file_label);
                        out.push(CompletionCandidate::new(name, desc).with_priority(50));
                    }
                }
            }
        } else if path.is_dir() && depth < max_depth {
            let dir_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if !dir_name.starts_with('.') && dir_name != "target" && dir_name != "node_modules" {
                collect_from_dir(&path, max_depth, depth + 1, out);
            }
        }
    }
}

/// Extension methods on `DynamicCompletionRegistry` for project-symbol integration.
pub trait ProjectSymbolRegistration {
    /// Register project symbols from `project_dir` as completions for shell completion.
    ///
    /// Scans up to `max_depth` directory levels for `*.oxilean` and `*.lean` files,
    /// extracts top-level declaration names, and stores them in the registry.
    fn register_project_symbols(&mut self, project_dir: &Path, max_depth: usize);

    /// Return all currently registered project symbols.
    fn project_symbol_candidates(&self) -> &[CompletionCandidate];
}

impl ProjectSymbolRegistration for DynamicCompletionRegistry {
    fn register_project_symbols(&mut self, project_dir: &Path, max_depth: usize) {
        let candidates = project_symbol_candidates(project_dir, max_depth);
        self.add_project_symbols(candidates);
    }

    fn project_symbol_candidates(&self) -> &[CompletionCandidate] {
        self.project_symbols()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::temp_dir;
    use std::fs;
    use std::path::PathBuf;

    fn tmp_subdir(name: &str) -> PathBuf {
        let dir = temp_dir().join(format!("oxilean_psp_test_{}", name));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("create temp dir");
        dir
    }

    #[test]
    fn test_extract_theorem() {
        let source = "theorem Foo : Nat := 0";
        let results = extract_decl_names_from_source(source);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].0, "Foo");
        assert_eq!(results[0].1, "theorem");
    }

    #[test]
    fn test_extract_def() {
        let source = "def myFunc (x : Int) : Int := x";
        let results = extract_decl_names_from_source(source);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].0, "myFunc");
        assert_eq!(results[0].1, "definition");
    }

    #[test]
    fn test_extract_lemma() {
        let source = "lemma bar_lemma : True";
        let results = extract_decl_names_from_source(source);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].0, "bar_lemma");
        assert_eq!(results[0].1, "lemma");
    }

    #[test]
    fn test_extract_multiple_decls() {
        let source = "\
theorem Alpha : Nat := 0
def beta_func : Bool := true
axiom gamma_ax : False
";
        let results = extract_decl_names_from_source(source);
        assert_eq!(results.len(), 3);
        let names: Vec<&str> = results.iter().map(|(n, _)| n.as_str()).collect();
        assert!(names.contains(&"Alpha"));
        assert!(names.contains(&"beta_func"));
        assert!(names.contains(&"gamma_ax"));
    }

    #[test]
    fn test_extract_ignores_comments() {
        let source = "-- theorem NotADecl : Nat";
        let results = extract_decl_names_from_source(source);
        assert!(
            results.is_empty(),
            "commented-out declarations must be ignored"
        );
    }

    #[test]
    fn test_extract_ignores_indent_prefix() {
        // Leading spaces are stripped via trim_start, so indented decls should be found.
        let source = "  def indentedDef : Nat := 42";
        let results = extract_decl_names_from_source(source);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].0, "indentedDef");
    }

    #[test]
    fn test_project_symbols_from_dir() {
        let dir = tmp_subdir("from_dir");
        let file = dir.join("test.oxilean");
        fs::write(
            &file,
            "theorem MyThm : Nat := 0\ndef myHelper : Bool := false\n",
        )
        .expect("write temp file");

        let candidates = project_symbol_candidates(&dir, 0);
        assert_eq!(
            candidates.len(),
            2,
            "expected 2 candidates, got {:?}",
            candidates
        );
        let names: Vec<&str> = candidates.iter().map(|c| c.text.as_str()).collect();
        assert!(names.contains(&"MyThm"));
        assert!(names.contains(&"myHelper"));
    }

    #[test]
    fn test_project_symbols_dedup() {
        let dir = tmp_subdir("dedup");
        // Write the same decl name in two different files.
        fs::write(dir.join("a.oxilean"), "theorem SharedName : Nat := 0\n").expect("write file a");
        fs::write(dir.join("b.oxilean"), "theorem SharedName : Bool := true\n")
            .expect("write file b");

        let candidates = project_symbol_candidates(&dir, 0);
        let shared: Vec<_> = candidates
            .iter()
            .filter(|c| c.text == "SharedName")
            .collect();
        assert_eq!(
            shared.len(),
            1,
            "duplicate declaration name must appear only once after dedup"
        );
    }

    #[test]
    fn test_register_project_symbols_into_registry() {
        let dir = tmp_subdir("registry");
        fs::write(dir.join("lib.oxilean"), "theorem RegThm : Nat := 0\n").expect("write lib file");

        let mut registry = DynamicCompletionRegistry::new();
        registry.register_project_symbols(&dir, 0);
        let syms = registry.project_symbol_candidates();
        assert!(!syms.is_empty(), "registry should have at least one symbol");
        assert!(syms.iter().any(|c| c.text == "RegThm"));
    }

    #[test]
    fn test_extract_noncomputable_def() {
        let source = "noncomputable def RealPi : Float := 3.14159";
        let results = extract_decl_names_from_source(source);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].0, "RealPi");
        assert_eq!(results[0].1, "noncomputable definition");
    }

    #[test]
    fn test_project_symbols_lean_extension() {
        let dir = tmp_subdir("lean_ext");
        fs::write(dir.join("compat.lean"), "def leanDef : Nat := 1\n").expect("write .lean file");

        let candidates = project_symbol_candidates(&dir, 0);
        let names: Vec<&str> = candidates.iter().map(|c| c.text.as_str()).collect();
        assert!(
            names.contains(&"leanDef"),
            ".lean files should also be scanned"
        );
    }
}
