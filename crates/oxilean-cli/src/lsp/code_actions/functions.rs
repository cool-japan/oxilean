//! Functions for the LSP code actions module.
//!
//! Implements `textDocument/codeAction` logic: analyzing Lean4-like source
//! text to produce quick-fixes, refactoring suggestions, and other code actions.

use crate::lsp::{JsonValue, Position, Range};

use super::types::{CodeAction, CodeActionContext, CodeActionKind, CodeTextEdit, WorkspaceEdit};

// ============================================================================
// Top-level handler
// ============================================================================

/// Handle a `textDocument/codeAction` request.
///
/// Inspects the source at the given URI and range, using the provided context
/// to filter which action kinds are relevant. Returns a sorted list of
/// applicable `CodeAction`s.
pub fn handle_code_action(
    _uri: &str,
    range: &Range,
    context: &CodeActionContext,
    source: &str,
) -> Vec<CodeAction> {
    let line = range.start.line;
    let col = range.start.character;
    let mut actions: Vec<CodeAction> = Vec::new();

    // QuickFix category
    if context.allows_kind(&CodeActionKind::QuickFix) {
        if let Some(action) = suggest_add_sorry(source, line) {
            actions.push(action);
        }
        if let Some(action) = suggest_add_type_annotation(source, line) {
            actions.push(action);
        }
    }

    // RefactorRewrite category
    if context.allows_kind(&CodeActionKind::RefactorRewrite) {
        if let Some(action) = suggest_rename_to_snake_case(source, line, col) {
            actions.push(action);
        }
    }

    // RefactorExtract category
    if context.allows_kind(&CodeActionKind::RefactorExtract) {
        if let Some(action) = suggest_extract_definition(source, range) {
            actions.push(action);
        }
    }

    // RefactorInline category
    if context.allows_kind(&CodeActionKind::RefactorInline) {
        if let Some(action) = suggest_inline_definition(source, line) {
            actions.push(action);
        }
    }

    actions
}

// ============================================================================
// Suggestion functions
// ============================================================================

/// Suggest adding a type annotation to an unannotated `let` binding.
///
/// Detects patterns like `let x := ...` (without `: Type`) on the given line,
/// and proposes inserting `: _` as a placeholder annotation.
///
/// Returns `None` if the line does not contain a candidate let binding.
pub fn suggest_add_type_annotation(source: &str, line: u32) -> Option<CodeAction> {
    let lines: Vec<&str> = source.lines().collect();
    let line_idx = line as usize;
    if line_idx >= lines.len() {
        return None;
    }
    let text = lines[line_idx];

    // Look for `let <ident> :=` pattern (no colon-annotation before :=)
    let trimmed = text.trim_start();
    if !trimmed.starts_with("let ") {
        return None;
    }

    // Find the position of `:=`
    let assign_offset = text.find(":=")?;

    // Check that there is no `:` (type annotation) between `let` and `:=`
    let after_let = text.find("let ").map(|p| p + 4)?;
    let between = &text[after_let..assign_offset];
    if between.contains(':') {
        return None;
    }

    // The identifier is the word after `let `
    let ident_end = between.find(|c: char| !c.is_alphanumeric() && c != '_' && c != '\'');
    let ident = match ident_end {
        Some(end) => between[..end].trim(),
        None => between.trim(),
    };
    if ident.is_empty() {
        return None;
    }

    // Insertion point: just before `:=`
    let insert_col = assign_offset as u32;
    let insert_range = Range::new(
        Position::new(line, insert_col),
        Position::new(line, insert_col),
    );

    let uri_placeholder = "current_document".to_string();
    let mut edit = WorkspaceEdit::new();
    edit.add_edit(uri_placeholder, CodeTextEdit::new(insert_range, " : _"));

    Some(CodeAction {
        title: format!("Add type annotation to '{}'", ident),
        kind: CodeActionKind::QuickFix,
        diagnostics: Vec::new(),
        edit: Some(edit),
        command: None,
        is_preferred: false,
    })
}

/// Suggest renaming a camelCase identifier to snake_case.
///
/// Scans the token at the given `(line, col)` position for a camelCase pattern,
/// and produces a `RefactorRewrite` action with the converted name.
///
/// Returns `None` if no camelCase identifier is found at that position.
pub fn suggest_rename_to_snake_case(source: &str, line: u32, col: u32) -> Option<CodeAction> {
    let lines: Vec<&str> = source.lines().collect();
    let line_idx = line as usize;
    if line_idx >= lines.len() {
        return None;
    }
    let text = lines[line_idx];
    let col_idx = col as usize;

    // Extract the identifier at the cursor
    let (ident, start_col, end_col) = extract_ident_at(text, col_idx)?;

    // Check if it is camelCase (has uppercase letters after the first char)
    if !is_camel_case(&ident) {
        return None;
    }

    let snake = camel_to_snake(&ident);
    if snake == ident {
        return None;
    }

    let edit_range = Range::new(
        Position::new(line, start_col as u32),
        Position::new(line, end_col as u32),
    );
    let mut edit = WorkspaceEdit::new();
    edit.add_edit(
        "current_document",
        CodeTextEdit::new(edit_range, snake.clone()),
    );

    Some(CodeAction {
        title: format!("Rename '{}' to '{}'", ident, snake),
        kind: CodeActionKind::RefactorRewrite,
        diagnostics: Vec::new(),
        edit: Some(edit),
        command: None,
        is_preferred: false,
    })
}

/// Suggest extracting the selected expression as a new top-level `def`.
///
/// When the user selects a range spanning at least one non-whitespace token,
/// this action proposes extracting that text into a fresh `def extracted := ...`
/// placed just above the current line, replacing the selection with the new name.
///
/// Returns `None` if the selected range is empty or contains only whitespace.
pub fn suggest_extract_definition(source: &str, range: &Range) -> Option<CodeAction> {
    let lines: Vec<&str> = source.lines().collect();
    let start_line = range.start.line as usize;
    let end_line = range.end.line as usize;

    if start_line >= lines.len() {
        return None;
    }

    // Collect the selected text
    let selected: String = if start_line == end_line {
        let text = lines[start_line];
        let s = range.start.character as usize;
        let e = (range.end.character as usize).min(text.len());
        if s >= e {
            return None;
        }
        text[s..e].to_string()
    } else {
        let mut parts = Vec::new();
        for (i, &ln) in lines[start_line..=end_line.min(lines.len() - 1)]
            .iter()
            .enumerate()
        {
            if i == 0 {
                let s = range.start.character as usize;
                parts.push(&ln[s.min(ln.len())..]);
            } else if i == end_line - start_line {
                let e = range.end.character as usize;
                parts.push(&ln[..e.min(ln.len())]);
            } else {
                parts.push(ln);
            }
        }
        parts.join("\n")
    };

    let trimmed = selected.trim();
    if trimmed.is_empty() {
        return None;
    }

    // The leading indentation of the target line
    let indent = lines[start_line]
        .chars()
        .take_while(|c| c.is_whitespace())
        .collect::<String>();

    // Insertion: a new def just before the current line
    let insertion = format!("{}def extracted := {}\n", indent, trimmed);
    let insert_pos = Position::new(range.start.line, 0);
    let insert_range = Range::new(insert_pos.clone(), insert_pos);

    // Replacement: the selection with `extracted`
    let replace_range = Range::new(
        Position::new(range.start.line, range.start.character),
        Position::new(range.end.line, range.end.character),
    );

    let mut edit = WorkspaceEdit::new();
    edit.add_edit(
        "current_document",
        CodeTextEdit::new(insert_range, insertion),
    );
    edit.add_edit(
        "current_document",
        CodeTextEdit::new(replace_range, "extracted".to_string()),
    );

    Some(CodeAction {
        title: "Extract as new definition".to_string(),
        kind: CodeActionKind::RefactorExtract,
        diagnostics: Vec::new(),
        edit: Some(edit),
        command: None,
        is_preferred: false,
    })
}

/// Suggest inlining a simple single-use `def` on the given line.
///
/// Detects a `def <name> := <expr>` pattern on the given line where the
/// definition appears to be a single-line constant (no parameters). Proposes
/// removing the definition line and replacing usages of `<name>` with `<expr>`.
///
/// Because a full rename across the document requires a workspace-aware resolver,
/// this implementation inserts an annotation comment that marks the substitution
/// for the editor to perform.
///
/// Returns `None` if the line does not match a simple `def` pattern.
pub fn suggest_inline_definition(source: &str, line: u32) -> Option<CodeAction> {
    let lines: Vec<&str> = source.lines().collect();
    let line_idx = line as usize;
    if line_idx >= lines.len() {
        return None;
    }
    let text = lines[line_idx];
    let trimmed = text.trim_start();

    // Match `def <name> := <expr>` with no parameters
    if !trimmed.starts_with("def ") {
        return None;
    }

    let after_def = &trimmed[4..];
    let assign_pos = after_def.find(":=")?;

    // Ensure there are no parentheses before `:=` (would indicate parameters)
    let name_part = &after_def[..assign_pos];
    if name_part.contains('(') || name_part.contains('[') || name_part.contains(':') {
        return None;
    }

    let name = name_part.trim();
    if name.is_empty() {
        return None;
    }

    let expr = after_def[assign_pos + 2..].trim();
    if expr.is_empty() {
        return None;
    }

    // Delete the entire definition line
    let delete_range = Range::new(Position::new(line, 0), Position::new(line + 1, 0));

    let mut edit = WorkspaceEdit::new();
    edit.add_edit(
        "current_document",
        CodeTextEdit::new(delete_range, String::new()),
    );

    Some(CodeAction {
        title: format!("Inline definition '{}'", name),
        kind: CodeActionKind::RefactorInline,
        diagnostics: Vec::new(),
        edit: Some(edit),
        command: Some(format!("oxilean.inlineDefinition:{}:{}", name, expr)),
        is_preferred: false,
    })
}

/// Suggest adding a `sorry` placeholder to an unfinished proof.
///
/// Detects lines that contain a `by` block opener or a `theorem`/`lemma`
/// signature ending with `:= by` where the proof body appears empty or missing.
/// Proposes inserting `sorry` as a placeholder to make the file typecheck.
///
/// Returns `None` if the line does not appear to require a proof.
pub fn suggest_add_sorry(source: &str, line: u32) -> Option<CodeAction> {
    let lines: Vec<&str> = source.lines().collect();
    let line_idx = line as usize;
    if line_idx >= lines.len() {
        return None;
    }
    let text = lines[line_idx];
    let trimmed = text.trim();

    // Heuristic: line ends with `:= by` or is purely `by` (tactic block opener)
    // and the next line is either missing or starts a new top-level declaration.
    let needs_sorry = trimmed.ends_with(":= by")
        || trimmed == "by"
        || trimmed.ends_with("by {}")
        || (trimmed.starts_with("theorem ") || trimmed.starts_with("lemma "))
            && trimmed.ends_with(":= by");

    if !needs_sorry {
        return None;
    }

    // Check the next line — if it's a new top-level declaration, the proof is empty
    let next_line_is_new_decl = lines.get(line_idx + 1).map_or(true, |l| {
        let t = l.trim_start();
        t.starts_with("def ")
            || t.starts_with("theorem ")
            || t.starts_with("lemma ")
            || t.starts_with("axiom ")
            || t.starts_with("instance ")
            || t.starts_with("class ")
            || t.starts_with("structure ")
            || t.is_empty()
    });

    if !next_line_is_new_decl {
        return None;
    }

    // Insert `  sorry` on a new line after the current one
    let indent = text
        .chars()
        .take_while(|c| c.is_whitespace())
        .collect::<String>();
    let insertion = format!("{}  sorry\n", indent);
    let insert_pos = Position::new(line + 1, 0);
    let insert_range = Range::new(insert_pos.clone(), insert_pos);

    let mut edit = WorkspaceEdit::new();
    edit.add_edit(
        "current_document",
        CodeTextEdit::new(insert_range, insertion),
    );

    Some(CodeAction {
        title: "Add `sorry` placeholder".to_string(),
        kind: CodeActionKind::QuickFix,
        diagnostics: Vec::new(),
        edit: Some(edit),
        command: None,
        is_preferred: true,
    })
}

/// Serialize a `CodeAction` to a JSON value (LSP format).
///
/// This is a convenience wrapper around [`CodeAction::to_json`].
pub fn code_action_to_json(action: &CodeAction) -> JsonValue {
    action.to_json()
}

// ============================================================================
// Internal helpers
// ============================================================================

/// Extract the identifier token that spans the given column in `text`.
///
/// Returns `(identifier_string, start_byte, end_byte)` or `None` if the
/// character at `col` is not an identifier character.
fn extract_ident_at(text: &str, col: usize) -> Option<(String, usize, usize)> {
    let bytes = text.as_bytes();
    if col >= bytes.len() {
        return None;
    }
    if !is_ident_byte(bytes[col]) {
        return None;
    }
    // Walk backward to the start
    let mut start = col;
    while start > 0 && is_ident_byte(bytes[start - 1]) {
        start -= 1;
    }
    // Walk forward to the end
    let mut end = col + 1;
    while end < bytes.len() && is_ident_byte(bytes[end]) {
        end += 1;
    }
    let ident = text[start..end].to_string();
    Some((ident, start, end))
}

/// Return whether a byte can appear in an identifier.
fn is_ident_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_' || b == b'\''
}

/// Return whether the string looks like camelCase (has an uppercase letter
/// after the first character, and at least one lowercase letter).
fn is_camel_case(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() < 2 {
        return false;
    }
    let has_upper_after_first = chars[1..].iter().any(|c| c.is_uppercase());
    let has_lower = chars.iter().any(|c| c.is_lowercase());
    has_upper_after_first && has_lower
}

/// Convert a camelCase string to snake_case.
fn camel_to_snake(s: &str) -> String {
    let mut result = String::with_capacity(s.len() + 4);
    for c in s.chars() {
        if c.is_uppercase() {
            if !result.is_empty() {
                result.push('_');
            }
            result.extend(c.to_lowercase());
        } else {
            result.push(c);
        }
    }
    result
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lsp::{Position, Range};

    fn make_range(sl: u32, sc: u32, el: u32, ec: u32) -> Range {
        Range::new(Position::new(sl, sc), Position::new(el, ec))
    }

    // --- camel_to_snake ---

    #[test]
    fn test_camel_to_snake_basic() {
        assert_eq!(camel_to_snake("camelCase"), "camel_case");
    }

    #[test]
    fn test_camel_to_snake_multiple_humps() {
        assert_eq!(camel_to_snake("myFooBarBaz"), "my_foo_bar_baz");
    }

    #[test]
    fn test_camel_to_snake_already_snake() {
        assert_eq!(camel_to_snake("snake_case"), "snake_case");
    }

    #[test]
    fn test_camel_to_snake_leading_upper() {
        assert_eq!(camel_to_snake("FooBar"), "foo_bar");
    }

    // --- is_camel_case ---

    #[test]
    fn test_is_camel_case_yes() {
        assert!(is_camel_case("myVar"));
        assert!(is_camel_case("helloWorld"));
    }

    #[test]
    fn test_is_camel_case_no_snake() {
        assert!(!is_camel_case("my_var"));
    }

    #[test]
    fn test_is_camel_case_single_char() {
        assert!(!is_camel_case("x"));
    }

    // --- suggest_add_type_annotation ---

    #[test]
    fn test_suggest_add_type_annotation_unannotated() {
        let src = "let x := 42";
        let action = suggest_add_type_annotation(src, 0);
        assert!(action.is_some());
        let a = action.unwrap();
        assert!(a.title.contains('x'));
        assert_eq!(a.kind, CodeActionKind::QuickFix);
    }

    #[test]
    fn test_suggest_add_type_annotation_already_annotated() {
        let src = "let x : Nat := 42";
        let action = suggest_add_type_annotation(src, 0);
        assert!(action.is_none());
    }

    #[test]
    fn test_suggest_add_type_annotation_no_let() {
        let src = "def foo := 42";
        let action = suggest_add_type_annotation(src, 0);
        assert!(action.is_none());
    }

    #[test]
    fn test_suggest_add_type_annotation_out_of_bounds() {
        let src = "let x := 1";
        let action = suggest_add_type_annotation(src, 99);
        assert!(action.is_none());
    }

    // --- suggest_rename_to_snake_case ---

    #[test]
    fn test_rename_camel_to_snake() {
        let src = "def myValue := 42";
        // cursor on 'm' of myValue
        let action = suggest_rename_to_snake_case(src, 0, 4);
        assert!(action.is_some());
        let a = action.unwrap();
        assert!(a.title.contains("my_value"));
        assert_eq!(a.kind, CodeActionKind::RefactorRewrite);
    }

    #[test]
    fn test_rename_already_snake() {
        let src = "def my_value := 42";
        let action = suggest_rename_to_snake_case(src, 0, 4);
        assert!(action.is_none());
    }

    #[test]
    fn test_rename_out_of_range() {
        let src = "def myVal := 1";
        let action = suggest_rename_to_snake_case(src, 5, 0);
        assert!(action.is_none());
    }

    // --- suggest_extract_definition ---

    #[test]
    fn test_extract_nonempty_selection() {
        let src = "def foo := 1 + 2 + 3\n";
        let range = make_range(0, 11, 0, 20);
        let action = suggest_extract_definition(src, &range);
        assert!(action.is_some());
        let a = action.unwrap();
        assert_eq!(a.kind, CodeActionKind::RefactorExtract);
        assert!(a.title.contains("Extract"));
    }

    #[test]
    fn test_extract_empty_selection() {
        let src = "def foo := 1\n";
        let range = make_range(0, 5, 0, 5);
        let action = suggest_extract_definition(src, &range);
        assert!(action.is_none());
    }

    #[test]
    fn test_extract_whitespace_only() {
        let src = "def foo :=   \n";
        let range = make_range(0, 10, 0, 13);
        let action = suggest_extract_definition(src, &range);
        assert!(action.is_none());
    }

    // --- suggest_inline_definition ---

    #[test]
    fn test_inline_simple_def() {
        let src = "def pi := 3.14159";
        let action = suggest_inline_definition(src, 0);
        assert!(action.is_some());
        let a = action.unwrap();
        assert!(a.title.contains("pi"));
        assert_eq!(a.kind, CodeActionKind::RefactorInline);
    }

    #[test]
    fn test_inline_def_with_params_rejected() {
        let src = "def add (x y : Nat) := x + y";
        let action = suggest_inline_definition(src, 0);
        assert!(action.is_none());
    }

    #[test]
    fn test_inline_theorem_rejected() {
        let src = "theorem foo : True := trivial";
        let action = suggest_inline_definition(src, 0);
        assert!(action.is_none());
    }

    // --- suggest_add_sorry ---

    #[test]
    fn test_suggest_sorry_by_line() {
        let src = "theorem foo : True := by\n";
        let action = suggest_add_sorry(src, 0);
        assert!(action.is_some());
        let a = action.unwrap();
        assert!(a.title.contains("sorry"));
        assert_eq!(a.kind, CodeActionKind::QuickFix);
    }

    #[test]
    fn test_suggest_sorry_standalone_by() {
        let src = "by\n";
        let action = suggest_add_sorry(src, 0);
        assert!(action.is_some());
    }

    #[test]
    fn test_suggest_sorry_no_match() {
        let src = "def x := 42\n";
        let action = suggest_add_sorry(src, 0);
        assert!(action.is_none());
    }

    #[test]
    fn test_suggest_sorry_next_line_has_proof() {
        let src = "theorem foo : True := by\n  trivial\n";
        let action = suggest_add_sorry(src, 0);
        // Next line has `trivial` (not empty/new decl), so no sorry needed
        assert!(action.is_none());
    }

    // --- handle_code_action ---

    #[test]
    fn test_handle_code_action_empty_source() {
        let context = CodeActionContext::new();
        let range = make_range(0, 0, 0, 0);
        let actions = handle_code_action("file:///test.lean", &range, &context, "");
        assert!(actions.is_empty());
    }

    #[test]
    fn test_handle_code_action_filters_by_kind() {
        let src = "let myVar := 42\n";
        let mut context = CodeActionContext::new();
        context.only = vec![CodeActionKind::RefactorExtract];
        let range = make_range(0, 0, 0, 3);
        let actions = handle_code_action("file:///test.lean", &range, &context, src);
        for a in &actions {
            assert_eq!(a.kind, CodeActionKind::RefactorExtract);
        }
    }

    #[test]
    fn test_handle_code_action_quickfix_let_annotation() {
        let src = "let foo := 10\n";
        let context = CodeActionContext::new();
        let range = make_range(0, 0, 0, 0);
        let actions = handle_code_action("file:///test.lean", &range, &context, src);
        let has_annotation = actions
            .iter()
            .any(|a| a.title.contains("type annotation") || a.title.contains("sorry"));
        assert!(has_annotation);
    }

    // --- code_action_to_json ---

    #[test]
    fn test_code_action_to_json_structure() {
        let mut edit = WorkspaceEdit::new();
        edit.add_edit(
            "file:///test.lean",
            CodeTextEdit::new(make_range(0, 0, 0, 5), "hello".to_string()),
        );
        let action = CodeAction::quick_fix("Fix it", Vec::new(), edit);
        let json = code_action_to_json(&action);
        assert!(matches!(json, JsonValue::Object(_)));
        if let JsonValue::Object(ref entries) = json {
            let has_title = entries.iter().any(|(k, _)| k == "title");
            let has_kind = entries.iter().any(|(k, _)| k == "kind");
            assert!(has_title, "missing title field");
            assert!(has_kind, "missing kind field");
        }
    }

    #[test]
    fn test_workspace_edit_to_json() {
        let mut edit = WorkspaceEdit::new();
        edit.add_edit(
            "file:///a.lean",
            CodeTextEdit::new(make_range(1, 0, 1, 3), "xyz".to_string()),
        );
        let json = edit.to_json();
        assert!(matches!(json, JsonValue::Object(_)));
    }

    #[test]
    fn test_code_action_kind_round_trip() {
        let kinds = [
            CodeActionKind::QuickFix,
            CodeActionKind::Refactor,
            CodeActionKind::RefactorExtract,
            CodeActionKind::RefactorInline,
            CodeActionKind::RefactorRewrite,
            CodeActionKind::Source,
        ];
        for k in &kinds {
            let s = k.as_str();
            let parsed = CodeActionKind::from_str(s);
            assert_eq!(parsed.as_ref(), Some(k), "round-trip failed for {:?}", k);
        }
    }

    #[test]
    fn test_code_action_kind_unknown() {
        assert!(CodeActionKind::from_str("unknown.action").is_none());
    }

    #[test]
    fn test_extract_ident_at_middle() {
        let text = "def myVar := 1";
        let result = extract_ident_at(text, 5);
        assert!(result.is_some());
        let (ident, _, _) = result.unwrap();
        assert_eq!(ident, "myVar");
    }

    #[test]
    fn test_extract_ident_at_space() {
        let text = "def myVar := 1";
        let result = extract_ident_at(text, 3);
        // col 3 is a space
        assert!(result.is_none());
    }

    #[test]
    fn test_extract_ident_at_out_of_bounds() {
        let text = "abc";
        let result = extract_ident_at(text, 100);
        assert!(result.is_none());
    }
}
