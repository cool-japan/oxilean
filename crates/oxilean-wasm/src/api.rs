//! Main API for OxiLean WASM bindings
#![allow(dead_code)]
#![allow(clippy::too_many_arguments)]

use crate::error::WasmResult;
use crate::types::*;
use oxilean_elab::{elaborate_decl, DeclElabError};
use oxilean_kernel::Environment;
use oxilean_parse::{print_decl, print_expr, Decl, Lexer, Located, Parser};

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

        // Parse the source with error recovery
        let mut parse_errors: Vec<String> = Vec::new();
        let decls = parse_source_recovering(source, &mut parse_errors);

        // If we got absolutely no declarations (hard parse failure), fall back
        if decls.is_empty() && !parse_errors.is_empty() {
            let errors = parse_errors
                .iter()
                .map(|msg| ErrorInfo {
                    message: msg.clone(),
                    line: None,
                    column: None,
                    source: Some("parser".to_string()),
                })
                .collect();
            return CheckResult {
                success: false,
                declarations: vec![],
                errors,
                warnings: vec![],
            };
        }

        // Elaborate each declaration against a fresh environment
        let env = Environment::new();
        let mut declarations: Vec<DeclInfo> = Vec::new();
        let mut errors: Vec<ErrorInfo> = Vec::new();
        let mut warnings: Vec<WarningInfo> = Vec::new();

        // Convert parse errors to ErrorInfo
        for msg in &parse_errors {
            errors.push(ErrorInfo {
                message: msg.clone(),
                line: None,
                column: None,
                source: Some("parser".to_string()),
            });
        }

        for located in &decls {
            let decl = &located.value;

            // Extract declaration info from AST (parse-level)
            if let Some(info) = decl_info_from_parse(decl) {
                // Attempt elaboration for richer type info
                match elaborate_decl(&env, decl) {
                    Ok(pending) => {
                        // Use elaborated type string if available
                        let ty_str = pending_decl_type_string(&pending);
                        declarations.push(DeclInfo {
                            name: info.name,
                            kind: info.kind,
                            ty: ty_str,
                        });
                    }
                    Err(elab_err) => {
                        // Use the parse-level type info and record elaboration warning
                        declarations.push(info);
                        let warn_msg = format_elab_error(&elab_err);
                        warnings.push(WarningInfo {
                            message: warn_msg,
                            line: None,
                            column: None,
                        });
                    }
                }
            }
            // Non-declaration commands (Import, Open, Variable, etc.) are ignored
        }

        let success = errors.is_empty();
        CheckResult {
            success,
            declarations,
            errors,
            warnings,
        }
    }

    /// Execute a REPL command
    pub fn repl(&mut self, input: &str) -> ReplResult {
        self.history.push(input.to_string());

        let trimmed = input.trim();

        if let Some(expr_str) = trimmed.strip_prefix("#check ") {
            // Try to parse and type-check the expression
            let type_str = infer_type_string(expr_str);
            ReplResult {
                output: format!("{} : {}", expr_str, type_str),
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
        } else if !trimmed.is_empty() {
            // Try to parse and elaborate as a declaration
            let tokens = Lexer::new(trimmed).tokenize();
            let mut parser = Parser::new(tokens);
            match parser.parse_decl() {
                Ok(_located) => ReplResult {
                    output: String::new(),
                    goals: vec![],
                    success: true,
                    error: None,
                },
                Err(e) => ReplResult {
                    output: String::new(),
                    goals: vec![],
                    success: false,
                    error: Some(format!("parse error: {e}")),
                },
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
    pub fn completions(&self, source: &str, _line: u32, _col: u32) -> Vec<CompletionItem> {
        // Start with the hardcoded keyword/tactic list
        let mut items = base_keyword_completions();

        // Also collect names from parsing the current source
        if !source.trim().is_empty() {
            let mut errs = Vec::new();
            let decls = parse_source_recovering(source, &mut errs);
            for located in &decls {
                if let Some(name) = decl_name(&located.value) {
                    let kind = completion_kind_for_decl(&located.value);
                    items.push(CompletionItem {
                        label: name.clone(),
                        kind,
                        detail: Some("defined in source".to_string()),
                        documentation: None,
                    });
                }
            }
        }

        items
    }

    /// Get hover info at a position
    pub fn hover_info(&self, source: &str, line: u32, _col: u32) -> Option<String> {
        if source.trim().is_empty() {
            return None;
        }

        // Parse the source and find a declaration spanning the given position
        let mut errs = Vec::new();
        let decls = parse_source_recovering(source, &mut errs);

        // Find a decl whose span covers (line, col)
        for located in &decls {
            let span = &located.span;
            // span.line is 1-indexed; line param is 0-indexed
            let span_line = span.line as u32;
            if span_line == line + 1 || (span_line <= line + 1) {
                if let Some(name) = decl_name(&located.value) {
                    let ty_hint = decl_type_hint(&located.value);
                    return Some(format!("{} : {}", name, ty_hint));
                }
            }
        }

        None
    }

    /// Format OxiLean source code
    pub fn format(&self, source: &str) -> WasmResult<String> {
        if source.trim().is_empty() {
            return Ok(source.to_string());
        }

        // Parse the source; if any decls succeed, pretty-print them
        let mut errs = Vec::new();
        let decls = parse_source_recovering(source, &mut errs);

        if decls.is_empty() {
            // Parse failed entirely — return original source unchanged
            return Ok(source.to_string());
        }

        // Re-print each declaration
        let formatted: Vec<String> = decls
            .iter()
            .map(|located| print_decl(&located.value))
            .collect();
        Ok(formatted.join("\n\n"))
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

// ─── Helper functions ────────────────────────────────────────────────────────

/// Parse source with per-declaration error recovery.
/// Collects successfully parsed declarations; appends error messages for failures.
fn parse_source_recovering(source: &str, errors: &mut Vec<String>) -> Vec<Located<Decl>> {
    let tokens = Lexer::new(source).tokenize();
    let mut parser = Parser::new(tokens);
    let mut decls = Vec::new();

    while !parser.is_eof() {
        match parser.parse_decl() {
            Ok(located) => decls.push(located),
            Err(e) => {
                errors.push(e.to_string());
                // Advance past the problematic token to continue parsing
                parser.advance();
            }
        }
    }

    decls
}

/// Extract a `DeclInfo` from a parsed `Decl` using surface-level information.
fn decl_info_from_parse(decl: &Decl) -> Option<DeclInfo> {
    match decl {
        Decl::Theorem { name, ty, .. } => Some(DeclInfo {
            name: name.clone(),
            kind: DeclKind::Theorem,
            ty: print_expr(&ty.value),
        }),
        Decl::Definition { name, ty, val, .. } => {
            let ty_str = match ty {
                Some(t) => print_expr(&t.value),
                None => print_expr(&val.value),
            };
            Some(DeclInfo {
                name: name.clone(),
                kind: DeclKind::Definition,
                ty: ty_str,
            })
        }
        Decl::Axiom { name, ty, .. } => Some(DeclInfo {
            name: name.clone(),
            kind: DeclKind::Axiom,
            ty: print_expr(&ty.value),
        }),
        Decl::Inductive { name, ty, .. } => Some(DeclInfo {
            name: name.clone(),
            kind: DeclKind::Inductive,
            ty: print_expr(&ty.value),
        }),
        Decl::Structure { name, .. } => Some(DeclInfo {
            name: name.clone(),
            kind: DeclKind::Structure,
            ty: "Type".to_string(),
        }),
        Decl::ClassDecl { name, .. } => Some(DeclInfo {
            name: name.clone(),
            kind: DeclKind::Class,
            ty: "Type".to_string(),
        }),
        Decl::InstanceDecl {
            name,
            class_name,
            ty,
            ..
        } => {
            let inst_name = name
                .clone()
                .unwrap_or_else(|| format!("inst_{}", class_name));
            Some(DeclInfo {
                name: inst_name,
                kind: DeclKind::Instance,
                ty: print_expr(&ty.value),
            })
        }
        // Non-declaration commands don't produce DeclInfo
        _ => None,
    }
}

/// Extract the name from a parsed `Decl` (if it has one).
fn decl_name(decl: &Decl) -> Option<String> {
    match decl {
        Decl::Theorem { name, .. }
        | Decl::Definition { name, .. }
        | Decl::Axiom { name, .. }
        | Decl::Inductive { name, .. }
        | Decl::Structure { name, .. }
        | Decl::ClassDecl { name, .. }
        | Decl::Namespace { name, .. }
        | Decl::SectionDecl { name, .. } => Some(name.clone()),
        Decl::InstanceDecl {
            name, class_name, ..
        } => Some(
            name.as_ref()
                .cloned()
                .unwrap_or_else(|| format!("inst_{}", class_name)),
        ),
        _ => None,
    }
}

/// Get a human-readable type hint from a parsed `Decl`.
fn decl_type_hint(decl: &Decl) -> String {
    match decl {
        Decl::Theorem { ty, .. } => print_expr(&ty.value),
        Decl::Definition { ty, val, .. } => match ty {
            Some(t) => print_expr(&t.value),
            None => print_expr(&val.value),
        },
        Decl::Axiom { ty, .. } => print_expr(&ty.value),
        Decl::Inductive { ty, .. } => print_expr(&ty.value),
        Decl::Structure { .. } | Decl::ClassDecl { .. } => "Type".to_string(),
        Decl::InstanceDecl { ty, .. } => print_expr(&ty.value),
        _ => "_".to_string(),
    }
}

/// Get the CompletionKind for a declaration.
fn completion_kind_for_decl(decl: &Decl) -> CompletionKind {
    match decl {
        Decl::Theorem { .. } => CompletionKind::Theorem,
        Decl::Definition { .. } => CompletionKind::Definition,
        Decl::Axiom { .. } => CompletionKind::Definition,
        Decl::Inductive { .. } => CompletionKind::Definition,
        Decl::Structure { .. } | Decl::ClassDecl { .. } => CompletionKind::Definition,
        Decl::InstanceDecl { .. } => CompletionKind::Definition,
        _ => CompletionKind::Keyword,
    }
}

/// Get a type string from a `PendingDecl` (after elaboration).
fn pending_decl_type_string(pending: &oxilean_elab::PendingDecl) -> String {
    use oxilean_elab::PendingDecl;
    use oxilean_kernel::prettyprint::print_expr as kernel_print;
    match pending {
        PendingDecl::Theorem { ty, .. }
        | PendingDecl::Definition { ty, .. }
        | PendingDecl::Axiom { ty, .. }
        | PendingDecl::Inductive { ty, .. }
        | PendingDecl::Opaque { ty, .. } => kernel_print(ty),
    }
}

/// Format a `DeclElabError` as a human-readable string.
fn format_elab_error(err: &DeclElabError) -> String {
    format!("elaboration warning: {}", err)
}

/// Try to infer the type of a surface expression string.
/// Returns a best-effort string (may just return the expression unchanged).
fn infer_type_string(expr_str: &str) -> String {
    // Parse the expression string
    let tokens = Lexer::new(expr_str).tokenize();
    let mut parser = Parser::new(tokens);
    match parser.parse_expr() {
        Ok(located) => {
            // We have a parsed expression; try to elaborate it
            let env = Environment::new();
            let mut ctx = oxilean_elab::ElabContext::new(&env);
            match oxilean_elab::elaborate_expr(&mut ctx, &located) {
                Ok(kernel_expr) => {
                    use oxilean_kernel::prettyprint::print_expr as kernel_print;
                    kernel_print(&kernel_expr)
                }
                Err(_) => {
                    // Fall back: show the parsed surface expression
                    print_expr(&located.value)
                }
            }
        }
        Err(_) => {
            // Cannot parse: return expression string as-is
            expr_str.to_string()
        }
    }
}

/// Base keyword and tactic completion items (always present).
fn base_keyword_completions() -> Vec<CompletionItem> {
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
    fn test_check_real_names() {
        let mut ox = OxiLean::new();
        let src = "theorem foo : True := trivial";
        let result = ox.check(src);
        assert!(result.success);
        assert_eq!(result.declarations.len(), 1);
        assert_eq!(result.declarations[0].name, "foo");
        assert!(matches!(result.declarations[0].kind, DeclKind::Theorem));
    }

    #[test]
    fn test_check_definition_real_name() {
        let mut ox = OxiLean::new();
        let src = "def myDef := 42";
        let result = ox.check(src);
        // Should have a declaration with the real name
        assert_eq!(result.declarations.len(), 1);
        assert_eq!(result.declarations[0].name, "myDef");
        assert!(matches!(result.declarations[0].kind, DeclKind::Definition));
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
    fn test_completions_from_source() {
        let ox = OxiLean::new();
        let src = "def myHelper := 1";
        let completions = ox.completions(src, 0, 0);
        let labels: Vec<&str> = completions.iter().map(|c| c.label.as_str()).collect();
        assert!(labels.contains(&"myHelper"));
    }

    #[test]
    fn test_format() {
        let ox = OxiLean::new();
        let src = "theorem foo : True := trivial";
        let result = ox
            .format(src)
            .expect("format should succeed for valid input");
        // Formatted output should contain the name and keyword
        assert!(result.contains("foo"));
        assert!(result.contains("theorem") || result.contains("lemma"));
    }

    #[test]
    fn test_format_empty() {
        let ox = OxiLean::new();
        let result = ox.format("").expect("format of empty should succeed");
        assert_eq!(result, "");
    }

    #[test]
    fn test_format_invalid() {
        let ox = OxiLean::new();
        // Completely invalid source should come back unchanged
        let src = "$$$ not valid oxilean $$$";
        let result = ox.format(src).expect("format should not error");
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

    #[test]
    fn test_hover_info_with_source() {
        let ox = OxiLean::new();
        let src = "theorem foo : True := trivial";
        // Line 0 is the first line; hover over it
        let info = ox.hover_info(src, 0, 8);
        // Should return Some with the name and type
        assert!(info.is_some());
        let s = info.expect("expected hover info for theorem declaration");
        assert!(s.contains("foo"));
    }

    #[test]
    fn test_hover_info_empty() {
        let ox = OxiLean::new();
        assert!(ox.hover_info("", 0, 0).is_none());
    }
}
