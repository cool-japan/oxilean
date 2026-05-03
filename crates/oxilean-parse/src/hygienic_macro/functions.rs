//! Functions for hygienic macro expansion.

use super::types::{
    ExpandResult, HygieneCtx, HygieneViolation, MacroCall, MacroDef, MacroVar, ScopeId,
    ViolationKind,
};
use std::collections::HashSet;

// ────────────────────────────────────────────────────────────────────────────
// HygieneCtx implementation
// ────────────────────────────────────────────────────────────────────────────

impl HygieneCtx {
    /// Create a fresh `HygieneCtx` with a root scope (id = 0).
    pub fn new() -> Self {
        let root = ScopeId(0);
        Self {
            current_scope: root,
            bindings: Default::default(),
            counter: 1,
            scope_stack: vec![root],
        }
    }

    /// Enter a new child scope and return its `ScopeId`.
    pub fn enter_scope(&mut self) -> ScopeId {
        let id = ScopeId(self.counter);
        self.counter += 1;
        self.scope_stack.push(id);
        self.current_scope = id;
        id
    }

    /// Exit the scope with the given `id`.
    ///
    /// Removes all bindings introduced in `id` and any inner scopes, then
    /// restores the parent scope as current.
    pub fn exit_scope(&mut self, id: ScopeId) {
        // Collect scope ids that need to be removed (id and all inner scopes).
        let pos = self.scope_stack.iter().position(|&s| s == id);
        let removed: std::collections::HashSet<ScopeId> = if let Some(p) = pos {
            self.scope_stack.drain(p..).collect()
        } else {
            std::collections::HashSet::new()
        };

        // Remove per-name binding entries that belong to the removed scopes.
        for stack in self.bindings.values_mut() {
            stack.retain(|(s, _)| !removed.contains(s));
        }

        self.current_scope = self.scope_stack.last().copied().unwrap_or(ScopeId(0));
    }

    /// Bind `name` in the current scope and return the fresh hygienic name.
    ///
    /// The fresh name has the form `<name>#<counter>`, e.g. `x#42`.
    pub fn bind(&mut self, name: &str) -> String {
        let fresh = format!("{}#{}", name, self.counter);
        self.counter += 1;
        let scope = self.current_scope;
        self.bindings
            .entry(name.to_string())
            .or_default()
            .push((scope, fresh.clone()));
        fresh
    }

    /// Resolve `name` to its fresh hygienic name in the innermost enclosing scope.
    ///
    /// Returns `None` if the name has never been bound.
    pub fn resolve(&self, name: &str) -> Option<&str> {
        self.bindings
            .get(name)
            .and_then(|stack| stack.last())
            .map(|(_, fresh)| fresh.as_str())
    }

    /// Return the set of all names currently in scope (original names).
    pub(super) fn names_in_scope(&self) -> HashSet<String> {
        self.bindings
            .iter()
            .filter(|(_, stack)| !stack.is_empty())
            .map(|(name, _)| name.clone())
            .collect()
    }
}

impl Default for HygieneCtx {
    fn default() -> Self {
        Self::new()
    }
}

// ────────────────────────────────────────────────────────────────────────────
// alpha_rename
// ────────────────────────────────────────────────────────────────────────────

/// Rename all free (word-boundary) occurrences of `old` to `new_name` in `source`.
///
/// Uses a simple word-boundary heuristic: a match is free when it is not
/// immediately preceded or followed by an alphanumeric character or `_`.
pub fn alpha_rename(source: &str, old: &str, new_name: &str) -> String {
    if old.is_empty() {
        return source.to_string();
    }

    let mut result = String::with_capacity(source.len());
    let src_bytes = source.as_bytes();
    let old_bytes = old.as_bytes();
    let old_len = old_bytes.len();
    let src_len = src_bytes.len();
    let mut i = 0usize;

    while i < src_len {
        // Check whether `old` starts at position `i`.
        if i + old_len <= src_len && &src_bytes[i..i + old_len] == old_bytes {
            // Check left boundary.
            let left_ok = i == 0 || !is_ident_char(src_bytes[i - 1]);
            // Check right boundary.
            let right_ok = i + old_len == src_len || !is_ident_char(src_bytes[i + old_len]);

            if left_ok && right_ok {
                result.push_str(new_name);
                i += old_len;
                continue;
            }
        }
        // Advance one byte (safe because we're walking ASCII-compatible boundaries).
        result.push(src_bytes[i] as char);
        i += 1;
    }

    result
}

#[inline]
fn is_ident_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_' || b == b'#'
}

// ────────────────────────────────────────────────────────────────────────────
// gensym
// ────────────────────────────────────────────────────────────────────────────

/// Generate a fresh name derived from `base` that is not currently present in `ctx`.
///
/// Tries `base`, then `base#0`, `base#1`, … until a free name is found.
pub fn gensym(base: &str, ctx: &HygieneCtx) -> String {
    let all_fresh: HashSet<String> = ctx
        .bindings
        .values()
        .flat_map(|stack| stack.iter().map(|(_, n)| n.clone()))
        .collect();

    // Also avoid names that equal original bound names.
    let orig_names: HashSet<String> = ctx.bindings.keys().cloned().collect();

    if !all_fresh.contains(base) && !orig_names.contains(base) {
        return base.to_string();
    }

    let mut idx = 0u64;
    loop {
        let candidate = format!("{}#{}", base, idx);
        if !all_fresh.contains(&candidate) && !orig_names.contains(&candidate) {
            return candidate;
        }
        idx += 1;
    }
}

// ────────────────────────────────────────────────────────────────────────────
// expand_macro
// ────────────────────────────────────────────────────────────────────────────

/// Expand `def` at `call`, producing a hygienic `ExpandResult`.
///
/// Algorithm
/// 1. Open a fresh expansion scope.
/// 2. For each formal parameter, alpha-rename occurrences in the body with
///    the corresponding actual argument.
/// 3. For each name introduced *inside* the macro body (those that look like
///    bare identifiers not matching a parameter), bind them hygienically so
///    they cannot capture names at the call site.
/// 4. Record introduced and used `MacroVar`s.
/// 5. Close the expansion scope.
pub fn expand_macro(def: &MacroDef, call: &MacroCall, ctx: &mut HygieneCtx) -> ExpandResult {
    let expansion_scope = ctx.enter_scope();

    // Substitute each actual argument for the corresponding formal parameter.
    let mut expanded = def.body.clone();
    let param_count = def.params.len().min(call.args.len());
    for idx in 0..param_count {
        let param = &def.params[idx];
        let arg = &call.args[idx];
        // Parameters may appear as `$param` or plain `param` in the body.
        expanded = alpha_rename(&expanded, &format!("${}", param), arg);
        expanded = alpha_rename(&expanded, param, arg);
    }

    // Collect words that look like identifiers introduced by the macro body
    // (i.e., not supplied by arguments).
    let arg_words: HashSet<&str> = call.args.iter().map(|s| s.as_str()).collect();
    let body_idents = collect_idents(&def.body);

    let mut introduced_names = Vec::new();
    let mut used_names = Vec::new();

    for ident in &body_idents {
        // Skip parameter names — they are replaced by actual arguments.
        if def.params.contains(ident) {
            continue;
        }
        // Skip identifiers that appear in the arguments (they come from outside).
        if arg_words.contains(ident.as_str()) {
            continue;
        }

        // Determine if this ident is being *introduced* (bound) or merely *used*.
        // Heuristic: names that appear in the macro body but were not bound
        // before the expansion are introduced.
        if ctx.resolve(ident).is_none() {
            // Bind it freshly so it cannot capture anything outside.
            let fresh = ctx.bind(ident);
            expanded = alpha_rename(&expanded, ident, &fresh);
            introduced_names.push(MacroVar::new(ident.clone(), expansion_scope));
        } else {
            used_names.push(MacroVar::new(ident.clone(), expansion_scope));
        }
    }

    ctx.exit_scope(expansion_scope);

    ExpandResult {
        expanded,
        introduced_names,
        used_names,
    }
}

// ────────────────────────────────────────────────────────────────────────────
// check_hygiene
// ────────────────────────────────────────────────────────────────────────────

/// Check an `ExpandResult` for hygiene violations.
///
/// Currently reports:
/// - `ShadowingOuter`: an introduced name has the same base name as a used name
///   in a different scope — this represents a potential shadowing issue.
/// - `CapturingFree`: a used name originates from a different scope than the
///   introduced names, suggesting a free variable may be captured.
pub fn check_hygiene(result: &ExpandResult) -> Vec<HygieneViolation> {
    let mut violations = Vec::new();

    for used in &result.used_names {
        for introduced in &result.introduced_names {
            if used.name == introduced.name && used.scope != introduced.scope {
                violations.push(HygieneViolation {
                    name: used.name.clone(),
                    def_scope: introduced.scope,
                    use_scope: used.scope,
                    kind: ViolationKind::CapturingFree,
                });
            }
        }
    }

    // Detect introduced names that shadow each other across scopes.
    for (i, a) in result.introduced_names.iter().enumerate() {
        for b in result.introduced_names.iter().skip(i + 1) {
            if a.name == b.name && a.scope != b.scope {
                violations.push(HygieneViolation {
                    name: a.name.clone(),
                    def_scope: a.scope,
                    use_scope: b.scope,
                    kind: ViolationKind::ShadowingOuter,
                });
            }
        }
    }

    violations
}

// ────────────────────────────────────────────────────────────────────────────
// Helpers
// ────────────────────────────────────────────────────────────────────────────

/// Collect all word-boundary identifiers from `src`.
fn collect_idents(src: &str) -> Vec<String> {
    let mut idents = Vec::new();
    let mut current = String::new();

    for ch in src.chars() {
        if ch.is_alphanumeric() || ch == '_' {
            current.push(ch);
        } else {
            if !current.is_empty() {
                let ident = std::mem::take(&mut current);
                if !idents.contains(&ident) {
                    idents.push(ident);
                }
            }
        }
    }
    if !current.is_empty() && !idents.contains(&current) {
        idents.push(current);
    }
    idents
}

// ────────────────────────────────────────────────────────────────────────────
// Tests
// ────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hygienic_macro::types::*;

    fn root_scope() -> ScopeId {
        ScopeId(0)
    }

    // ── HygieneCtx ──────────────────────────────────────────────────────────

    #[test]
    fn test_ctx_new() {
        let ctx = HygieneCtx::new();
        assert_eq!(ctx.current_scope, root_scope());
        assert!(ctx.bindings.is_empty());
    }

    #[test]
    fn test_enter_exit_scope() {
        let mut ctx = HygieneCtx::new();
        let s1 = ctx.enter_scope();
        assert_ne!(s1, root_scope());
        assert_eq!(ctx.current_scope, s1);

        let s2 = ctx.enter_scope();
        assert_ne!(s2, s1);
        assert_eq!(ctx.current_scope, s2);

        ctx.exit_scope(s2);
        assert_eq!(ctx.current_scope, s1);

        ctx.exit_scope(s1);
        assert_eq!(ctx.current_scope, root_scope());
    }

    #[test]
    fn test_bind_and_resolve() {
        let mut ctx = HygieneCtx::new();
        let fresh = ctx.bind("x");
        assert!(fresh.starts_with("x#"));
        assert_eq!(ctx.resolve("x"), Some(fresh.as_str()));
    }

    #[test]
    fn test_resolve_none_for_unbound() {
        let ctx = HygieneCtx::new();
        assert_eq!(ctx.resolve("unknown"), None);
    }

    #[test]
    fn test_inner_scope_binding_shadows() {
        let mut ctx = HygieneCtx::new();
        let outer_fresh = ctx.bind("x");
        let s = ctx.enter_scope();
        let inner_fresh = ctx.bind("x");
        // Inner binding should resolve.
        assert_eq!(ctx.resolve("x"), Some(inner_fresh.as_str()));
        ctx.exit_scope(s);
        // After exit, outer binding should still resolve.
        assert_eq!(ctx.resolve("x"), Some(outer_fresh.as_str()));
    }

    #[test]
    fn test_bind_uniqueness() {
        let mut ctx = HygieneCtx::new();
        let a = ctx.bind("y");
        let b = ctx.bind("y");
        assert_ne!(a, b);
    }

    #[test]
    fn test_multiple_names() {
        let mut ctx = HygieneCtx::new();
        ctx.bind("a");
        ctx.bind("b");
        assert!(ctx.resolve("a").is_some());
        assert!(ctx.resolve("b").is_some());
    }

    #[test]
    fn test_scope_stack_after_nested() {
        let mut ctx = HygieneCtx::new();
        let s1 = ctx.enter_scope();
        let s2 = ctx.enter_scope();
        ctx.exit_scope(s2);
        ctx.exit_scope(s1);
        assert_eq!(ctx.current_scope, root_scope());
    }

    // ── alpha_rename ────────────────────────────────────────────────────────

    #[test]
    fn test_alpha_rename_basic() {
        let result = alpha_rename("let x = x + 1", "x", "y");
        assert_eq!(result, "let y = y + 1");
    }

    #[test]
    fn test_alpha_rename_no_partial_match() {
        let result = alpha_rename("let xy = x", "x", "z");
        // "xy" should not be touched; "x" standalone should.
        assert_eq!(result, "let xy = z");
    }

    #[test]
    fn test_alpha_rename_empty_old() {
        let result = alpha_rename("hello", "", "x");
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_alpha_rename_multiple() {
        let result = alpha_rename("a + a * a", "a", "b");
        assert_eq!(result, "b + b * b");
    }

    #[test]
    fn test_alpha_rename_no_occurrences() {
        let result = alpha_rename("def f x := x", "y", "z");
        assert_eq!(result, "def f x := x");
    }

    #[test]
    fn test_alpha_rename_at_start_and_end() {
        let result = alpha_rename("x + y + x", "x", "w");
        assert_eq!(result, "w + y + w");
    }

    #[test]
    fn test_alpha_rename_with_hash_suffix() {
        // Names like x#1 should not match "x".
        let result = alpha_rename("x + x#1 + x", "x", "z");
        // x#1 should not be renamed because '#' is an ident char in our heuristic.
        assert_eq!(result, "z + x#1 + z");
    }

    // ── gensym ──────────────────────────────────────────────────────────────

    #[test]
    fn test_gensym_fresh_base() {
        let ctx = HygieneCtx::new();
        let name = gensym("tmp", &ctx);
        assert_eq!(name, "tmp");
    }

    #[test]
    fn test_gensym_avoids_bound() {
        let mut ctx = HygieneCtx::new();
        ctx.bind("tmp");
        let name = gensym("tmp", &ctx);
        // Should differ from "tmp" since it's bound.
        assert_ne!(name, "tmp");
        assert!(name.starts_with("tmp#"));
    }

    #[test]
    fn test_gensym_multiple_collisions() {
        let mut ctx = HygieneCtx::new();
        ctx.bind("v"); // Produces v#<counter>.
                       // Also manually simulate that "v#0" is taken by binding v again.
        ctx.bind("v");
        let name = gensym("v", &ctx);
        assert!(name.starts_with("v"));
    }

    // ── expand_macro ────────────────────────────────────────────────────────

    #[test]
    fn test_expand_simple_identity() {
        let def = MacroDef {
            name: "id".into(),
            params: vec!["x".into()],
            body: "x".into(),
            def_scope: ScopeId(0),
        };
        let call = MacroCall {
            name: "id".into(),
            args: vec!["42".into()],
            call_scope: ScopeId(0),
        };
        let mut ctx = HygieneCtx::new();
        let result = expand_macro(&def, &call, &mut ctx);
        assert!(result.expanded.contains("42"));
    }

    #[test]
    fn test_expand_renames_introduced_names() {
        let def = MacroDef {
            name: "swap".into(),
            params: vec!["a".into(), "b".into()],
            body: "let tmp = a; a = b; b = tmp; tmp".into(),
            def_scope: ScopeId(0),
        };
        let call = MacroCall {
            name: "swap".into(),
            args: vec!["p".into(), "q".into()],
            call_scope: ScopeId(0),
        };
        let mut ctx = HygieneCtx::new();
        let result = expand_macro(&def, &call, &mut ctx);
        // The expansion should contain actual args.
        assert!(result.expanded.contains("p"));
        assert!(result.expanded.contains("q"));
        // Should have introduced names.
        assert!(!result.introduced_names.is_empty());
    }

    #[test]
    fn test_expand_no_params() {
        let def = MacroDef {
            name: "unit".into(),
            params: vec![],
            body: "()".into(),
            def_scope: ScopeId(0),
        };
        let call = MacroCall {
            name: "unit".into(),
            args: vec![],
            call_scope: ScopeId(0),
        };
        let mut ctx = HygieneCtx::new();
        let result = expand_macro(&def, &call, &mut ctx);
        assert_eq!(result.expanded, "()");
    }

    #[test]
    fn test_expand_returns_result() {
        let def = MacroDef {
            name: "double".into(),
            params: vec!["n".into()],
            body: "n + n".into(),
            def_scope: ScopeId(0),
        };
        let call = MacroCall {
            name: "double".into(),
            args: vec!["5".into()],
            call_scope: ScopeId(0),
        };
        let mut ctx = HygieneCtx::new();
        let result = expand_macro(&def, &call, &mut ctx);
        assert!(result.expanded.contains("5"));
    }

    // ── check_hygiene ────────────────────────────────────────────────────────

    #[test]
    fn test_check_hygiene_clean() {
        let result = ExpandResult {
            expanded: "x + y".into(),
            introduced_names: vec![MacroVar::new("tmp", ScopeId(1))],
            used_names: vec![MacroVar::new("z", ScopeId(2))],
        };
        let violations = check_hygiene(&result);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_check_hygiene_capturing_free() {
        let result = ExpandResult {
            expanded: "x".into(),
            introduced_names: vec![MacroVar::new("x", ScopeId(1))],
            used_names: vec![MacroVar::new("x", ScopeId(2))],
        };
        let violations = check_hygiene(&result);
        assert!(!violations.is_empty());
        assert!(violations
            .iter()
            .any(|v| v.kind == ViolationKind::CapturingFree));
    }

    #[test]
    fn test_check_hygiene_shadowing_outer() {
        let result = ExpandResult {
            expanded: "".into(),
            introduced_names: vec![
                MacroVar::new("x", ScopeId(1)),
                MacroVar::new("x", ScopeId(2)),
            ],
            used_names: vec![],
        };
        let violations = check_hygiene(&result);
        assert!(violations
            .iter()
            .any(|v| v.kind == ViolationKind::ShadowingOuter));
    }

    #[test]
    fn test_scope_id_display() {
        let s = ScopeId(42);
        assert_eq!(format!("{}", s), "scope#42");
    }

    #[test]
    fn test_macrovar_display() {
        let v = MacroVar::new("x", ScopeId(3));
        assert_eq!(format!("{}", v), "x@scope#3");
    }

    #[test]
    fn test_violation_kind_display() {
        assert_eq!(
            format!("{}", ViolationKind::CapturingFree),
            "capturing-free"
        );
        assert_eq!(
            format!("{}", ViolationKind::CapturingBound),
            "capturing-bound"
        );
        assert_eq!(
            format!("{}", ViolationKind::ShadowingOuter),
            "shadowing-outer"
        );
    }

    #[test]
    fn test_default_ctx() {
        let ctx = HygieneCtx::default();
        assert_eq!(ctx.current_scope, ScopeId(0));
    }

    #[test]
    fn test_collect_idents_helper() {
        let idents = collect_idents("let x = y + z");
        assert!(idents.contains(&"let".to_string()));
        assert!(idents.contains(&"x".to_string()));
        assert!(idents.contains(&"y".to_string()));
        assert!(idents.contains(&"z".to_string()));
    }

    #[test]
    fn test_expand_dollar_params() {
        let def = MacroDef {
            name: "m".into(),
            params: vec!["x".into()],
            body: "$x * $x".into(),
            def_scope: ScopeId(0),
        };
        let call = MacroCall {
            name: "m".into(),
            args: vec!["n".into()],
            call_scope: ScopeId(0),
        };
        let mut ctx = HygieneCtx::new();
        let result = expand_macro(&def, &call, &mut ctx);
        assert!(result.expanded.contains("n"));
    }

    #[test]
    fn test_names_in_scope() {
        let mut ctx = HygieneCtx::new();
        ctx.bind("alpha");
        ctx.bind("beta");
        let names = ctx.names_in_scope();
        assert!(names.contains("alpha"));
        assert!(names.contains("beta"));
    }
}
