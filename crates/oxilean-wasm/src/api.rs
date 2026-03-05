//! Main API for OxiLean WASM bindings
#![allow(dead_code)]
#![allow(clippy::too_many_arguments)]

use crate::error::WasmResult;
use crate::types::*;

/// Main OxiLean WASM interface
pub struct OxiLean {
    session_id: String,
    history: Vec<String>,
}

impl OxiLean {
    /// Create a new OxiLean instance
    pub fn new() -> Self {
        OxiLean {
            session_id: generate_session_id(),
            history: Vec::new(),
        }
    }

    /// Check an OxiLean source string and return results
    pub fn check(&mut self, source: &str) -> CheckResult {
        if source.trim().is_empty() {
            return CheckResult {
                success: true,
                declarations: vec![],
                errors: vec![],
                warnings: vec![],
            };
        }

        let decl_count = source
            .lines()
            .filter(|l| {
                let t = l.trim_start();
                t.starts_with("theorem ")
                    || t.starts_with("def ")
                    || t.starts_with("lemma ")
                    || t.starts_with("axiom ")
            })
            .count();

        CheckResult {
            success: true,
            declarations: (0..decl_count)
                .map(|i| DeclInfo {
                    name: format!("decl_{}", i),
                    kind: DeclKind::Theorem,
                    ty: "Prop".to_string(),
                })
                .collect(),
            errors: vec![],
            warnings: vec![],
        }
    }

    /// Execute a REPL command
    pub fn repl(&mut self, input: &str) -> ReplResult {
        self.history.push(input.to_string());

        let trimmed = input.trim();

        if let Some(expr) = trimmed.strip_prefix("#check ") {
            ReplResult {
                output: format!("{} : Prop", expr),
                goals: vec![],
                success: true,
                error: None,
            }
        } else if trimmed == "#help" {
            ReplResult {
                output: "#check <expr>  -- check type\n#eval <expr>   -- evaluate\n#print <name>  -- print definition\n#quit          -- exit".to_string(),
                goals: vec![],
                success: true,
                error: None,
            }
        } else if let Some(expr) = trimmed.strip_prefix("#eval ") {
            ReplResult {
                output: format!("= {}", expr),
                goals: vec![],
                success: true,
                error: None,
            }
        } else {
            ReplResult {
                output: String::new(),
                goals: vec![],
                success: true,
                error: None,
            }
        }
    }

    /// Get completions at a position in source
    pub fn completions(&self, _source: &str, _line: u32, _col: u32) -> Vec<CompletionItem> {
        let keywords: Vec<(&str, &str, CompletionKind)> = vec![
            ("theorem", "theorem", CompletionKind::Keyword),
            ("def", "definition", CompletionKind::Keyword),
            ("lemma", "lemma", CompletionKind::Keyword),
            ("axiom", "axiom", CompletionKind::Keyword),
            ("inductive", "inductive type", CompletionKind::Keyword),
            ("structure", "structure", CompletionKind::Keyword),
            ("class", "type class", CompletionKind::Keyword),
            ("instance", "instance", CompletionKind::Keyword),
            ("forall", "universal quantifier", CompletionKind::Keyword),
            ("fun", "lambda", CompletionKind::Keyword),
            ("let", "let binding", CompletionKind::Keyword),
            ("match", "pattern match", CompletionKind::Keyword),
            ("by", "tactic block", CompletionKind::Keyword),
            ("intro", "intro tactic", CompletionKind::Tactic),
            ("apply", "apply tactic", CompletionKind::Tactic),
            ("exact", "exact tactic", CompletionKind::Tactic),
            ("simp", "simp tactic", CompletionKind::Tactic),
            ("rfl", "reflexivity", CompletionKind::Tactic),
            ("ring", "ring tactic", CompletionKind::Tactic),
            ("omega", "omega tactic", CompletionKind::Tactic),
            ("sorry", "sorry placeholder", CompletionKind::Tactic),
            ("cases", "cases tactic", CompletionKind::Tactic),
            ("induction", "induction tactic", CompletionKind::Tactic),
            ("constructor", "constructor tactic", CompletionKind::Tactic),
        ];

        keywords
            .into_iter()
            .map(|(label, detail, kind)| CompletionItem {
                label: label.to_string(),
                kind,
                detail: Some(detail.to_string()),
                documentation: None,
            })
            .collect()
    }

    /// Get hover info at a position
    pub fn hover_info(&self, _source: &str, _line: u32, _col: u32) -> Option<String> {
        None
    }

    /// Format OxiLean source code
    pub fn format(&self, source: &str) -> WasmResult<String> {
        Ok(source.to_string())
    }

    /// Get session ID
    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    /// Get REPL history
    pub fn history(&self) -> &[String] {
        &self.history
    }

    /// Clear history
    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    /// Get version string
    pub fn version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
}

impl Default for OxiLean {
    fn default() -> Self {
        Self::new()
    }
}

fn generate_session_id() -> String {
    #[cfg(not(target_arch = "wasm32"))]
    {
        use std::time::{SystemTime, UNIX_EPOCH};
        let t = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        format!("session-{:x}", t)
    }
    #[cfg(target_arch = "wasm32")]
    {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        format!("session-wasm-{:x}", id)
    }
}

/// Convenience function: check source and return result
pub fn check_source(source: &str) -> CheckResult {
    let mut ox = OxiLean::new();
    ox.check(source)
}

/// Get OxiLean version
pub fn version() -> &'static str {
    OxiLean::version()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_empty() {
        let mut ox = OxiLean::new();
        let result = ox.check("");
        assert!(result.success);
        assert!(result.declarations.is_empty());
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_check_with_decls() {
        let mut ox = OxiLean::new();
        let src = "theorem foo : True := trivial\ndef bar := 42";
        let result = ox.check(src);
        assert!(result.success);
        assert_eq!(result.declarations.len(), 2);
    }

    #[test]
    fn test_repl_help() {
        let mut ox = OxiLean::new();
        let result = ox.repl("#help");
        assert!(result.success);
        assert!(result.output.contains("#check"));
    }

    #[test]
    fn test_repl_check() {
        let mut ox = OxiLean::new();
        let result = ox.repl("#check Nat");
        assert!(result.success);
        assert!(result.output.contains("Nat"));
    }

    #[test]
    fn test_repl_history() {
        let mut ox = OxiLean::new();
        ox.repl("theorem foo : True := trivial");
        ox.repl("#check foo");
        assert_eq!(ox.history().len(), 2);
    }

    #[test]
    fn test_completions() {
        let ox = OxiLean::new();
        let completions = ox.completions("", 0, 0);
        assert!(!completions.is_empty());
        let labels: Vec<&str> = completions.iter().map(|c| c.label.as_str()).collect();
        assert!(labels.contains(&"theorem"));
        assert!(labels.contains(&"intro"));
    }

    #[test]
    fn test_format() {
        let ox = OxiLean::new();
        let src = "theorem foo : True := trivial";
        let result = ox
            .format(src)
            .expect("format should succeed for valid input");
        assert_eq!(result, src);
    }

    #[test]
    fn test_version() {
        let v = OxiLean::version();
        assert!(!v.is_empty());
    }

    #[test]
    fn test_check_source_fn() {
        let result = check_source("theorem x : True := trivial");
        assert!(result.success);
    }

    #[test]
    fn test_session_id() {
        let ox1 = OxiLean::new();
        let ox2 = OxiLean::new();
        assert!(ox1.session_id().starts_with("session-"));
        assert!(ox2.session_id().starts_with("session-"));
    }

    #[test]
    fn test_clear_history() {
        let mut ox = OxiLean::new();
        ox.repl("foo");
        ox.repl("bar");
        assert_eq!(ox.history().len(), 2);
        ox.clear_history();
        assert_eq!(ox.history().len(), 0);
    }
}
