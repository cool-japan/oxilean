//! Normalization functions (part 3): list literals, subscripts, sigma, dfinsupp,
//! type ascriptions, head binders.

/// Normalize single-element list literals `[a]` in type positions.
///
/// In Lean 4, `[a]` is syntactic sugar for `List.cons a List.nil`.
/// OxiLean's parser may or may not handle `[a]` directly.
/// Replace single-element list literals with explicit form to ensure parsing.
/// Only replaces when content is a simple identifier (no operators, no spaces).
pub(super) fn normalize_list_literal_in_type(src: &str) -> String {
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut result = String::with_capacity(src.len() + 32);
    let mut i = 0;
    while i < len {
        if chars[i] == '[' {
            let preceded_by_ident = i > 0
                && (chars[i - 1].is_alphanumeric() || chars[i - 1] == '_' || chars[i - 1] == '\'');
            if preceded_by_ident {
                result.push(chars[i]);
                i += 1;
                continue;
            }
            let bracket_start = i;
            let mut j = i + 1;
            let mut depth = 1usize;
            let mut found_comma = false;
            let content_start = j;
            let _ = content_start;
            while j < len && depth > 0 {
                match chars[j] {
                    '[' => {
                        depth += 1;
                        j += 1;
                    }
                    ']' => {
                        depth = depth.saturating_sub(1);
                        j += 1;
                    }
                    ',' if depth == 1 => {
                        found_comma = true;
                        j += 1;
                    }
                    _ => {
                        j += 1;
                    }
                }
            }
            if depth == 0 && !found_comma && j > bracket_start + 2 {
                let inner: String = chars[bracket_start + 1..j - 1].iter().collect();
                let inner_trimmed = inner.trim();
                let is_simple = !inner_trimmed.is_empty()
                    && inner_trimmed
                        .chars()
                        .all(|c| c.is_alphanumeric() || c == '_' || c == '\'' || c == '!');
                if is_simple {
                    result.push_str("(List.cons ");
                    result.push_str(inner_trimmed);
                    result.push_str(" List.nil)");
                    i = j;
                    continue;
                }
            }
            let raw: String = chars[bracket_start..j].iter().collect();
            result.push_str(&raw);
            i = j;
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}
/// Normalize array/list subscript indexing notation.
///
/// `ident[0]` -> `ident` (drop numeric subscript for parsing purposes)
/// `ident[n]` -> `ident` (drop variable subscript too)
/// This handles cases like `l[0]` appearing in types where OxiLean
/// does not support subscript syntax.
pub(super) fn normalize_subscript_indexing(src: &str) -> String {
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut result = String::with_capacity(src.len());
    let mut i = 0;
    while i < len {
        if chars[i] == '[' {
            let preceded_by_ident = i > 0
                && (chars[i - 1].is_alphanumeric() || chars[i - 1] == '_' || chars[i - 1] == '\'');
            if preceded_by_ident {
                let bracket_start = i;
                let mut j = i + 1;
                let mut depth = 1usize;
                while j < len && depth > 0 {
                    match chars[j] {
                        '[' => depth += 1,
                        ']' => depth = depth.saturating_sub(1),
                        _ => {}
                    }
                    j += 1;
                }
                if depth == 0 {
                    let inner: String = chars[bracket_start + 1..j - 1].iter().collect();
                    let inner_trimmed = inner.trim();
                    let no_nested_brackets =
                        !inner_trimmed.contains('[') && !inner_trimmed.contains(']');
                    if no_nested_brackets && !inner_trimmed.is_empty() {
                        i = j;
                        continue;
                    }
                }
                let raw: String = chars[bracket_start..j].iter().collect();
                result.push_str(&raw);
                i = j;
            } else {
                result.push(chars[i]);
                i += 1;
            }
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}
/// Try to parse parenthesized binder groups after Sigma/PSigma.
///
/// Handles: `Sigma (x : T) (y : U), body` → nested `Sigma T (fun (x : T) -> Sigma U (fun (y : U) -> body))`
/// Returns (replacement_string, end_position) if successful.
fn try_parse_sigma_paren_binders(
    chars: &[char],
    start: usize,
    len: usize,
    kw: &str,
) -> Option<(String, usize)> {
    // Collect parenthesized binder groups
    let mut binders: Vec<(String, String)> = Vec::new(); // (name, type)
    let mut j = start;

    while j < len && chars[j] == '(' {
        // Find matching close paren
        let group_start = j;
        let mut depth = 0usize;
        while j < len {
            match chars[j] {
                '(' => depth += 1,
                ')' => {
                    depth -= 1;
                    if depth == 0 {
                        j += 1;
                        break;
                    }
                }
                _ => {}
            }
            j += 1;
        }
        if depth != 0 {
            return None;
        }
        // Parse the group content: (name : Type) or (name1 name2 : Type)
        let inner: String = chars[group_start + 1..j - 1].iter().collect();
        let inner = inner.trim();
        if let Some(colon_pos) = inner.find(':') {
            let names_part = inner[..colon_pos].trim();
            let type_part = inner[colon_pos + 1..].trim();
            // Handle multiple names: (x y : T) → separate binders
            for name in names_part.split_whitespace() {
                binders.push((name.to_string(), type_part.to_string()));
            }
        } else {
            // No colon — treat as untyped binder
            let name = inner.trim();
            if name.is_empty() {
                return None;
            }
            binders.push((name.to_string(), "_".to_string()));
        }
        // Skip spaces between groups
        while j < len && chars[j] == ' ' {
            j += 1;
        }
    }

    if binders.is_empty() {
        return None;
    }

    // Expect comma after binder groups
    if j >= len || chars[j] != ',' {
        return None;
    }
    j += 1;
    while j < len && chars[j] == ' ' {
        j += 1;
    }

    // Scan body (stop at unmatched close paren or `:=` at depth 0)
    let body_start = j;
    let mut depth = 0usize;
    while j < len {
        match chars[j] {
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' => {
                if depth == 0 {
                    break;
                }
                depth = depth.saturating_sub(1);
            }
            ':' if depth == 0 && j + 1 < len && chars[j + 1] == '=' => {
                break;
            }
            _ => {}
        }
        j += 1;
    }
    let body: String = chars[body_start..j].iter().collect();
    let body = body.trim_end();

    // Build nested sigma: Sigma T (fun (x : T) -> Sigma U (fun (y : U) -> body))
    let mut result = String::new();
    for (idx, (name, ty)) in binders.iter().enumerate() {
        if idx > 0 {
            // For subsequent binders, we're already inside a fun body
        }
        result.push_str(kw);
        result.push(' ');
        result.push_str(ty);
        result.push_str(" (fun (");
        result.push_str(name);
        result.push_str(" : ");
        result.push_str(ty);
        result.push_str(") -> ");
    }
    result.push_str(body);
    // Close all the parens
    for _ in &binders {
        result.push(')');
    }

    Some((result, j))
}

/// Normalize `Sigma` binders in type signatures.
///
/// `Sigma i, body` -> `Sigma (fun i -> body)` when preceded by `:` or `->`.
/// Handles Lean 4's `Σ i, body` notation (after Unicode normalization).
pub(super) fn normalize_sigma_in_binders(src: &str) -> String {
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut result = String::with_capacity(src.len() + 32);
    let mut i = 0;
    while i < len {
        let prev_is_word = i > 0 && (chars[i - 1].is_alphanumeric() || chars[i - 1] == '_');
        let is_sigma =
            !prev_is_word && i + 6 <= len && chars[i..i + 6].iter().collect::<String>() == "Sigma ";
        let is_psigma = !prev_is_word
            && i + 7 <= len
            && chars[i..i + 7].iter().collect::<String>() == "PSigma ";
        if is_sigma || is_psigma {
            let kw_len = if is_sigma { 6 } else { 7 };
            let kw = if is_sigma { "Sigma" } else { "PSigma" };
            let after_kw = i + kw_len;
            let mut j = after_kw;
            while j < len && chars[j] == ' ' {
                j += 1;
            }
            // Try parenthesized binder groups: Sigma (x : T) (y : U), body
            if j < len && chars[j] == '(' {
                if let Some((replacement, end_pos)) =
                    try_parse_sigma_paren_binders(&chars, j, len, kw)
                {
                    result.push_str(&replacement);
                    i = end_pos;
                    continue;
                }
            }
            let binder_start = j;
            while j < len && (chars[j].is_alphanumeric() || chars[j] == '_' || chars[j] == '\'') {
                j += 1;
            }
            let binder_end = j;
            if binder_end > binder_start {
                while j < len && chars[j] == ' ' {
                    j += 1;
                }
                let has_type_ann = j < len && chars[j] == ':';
                let type_end;
                if has_type_ann {
                    j += 1;
                    while j < len && chars[j] == ' ' {
                        j += 1;
                    }
                    let type_start = j;
                    let mut depth = 0usize;
                    while j < len {
                        match chars[j] {
                            '(' | '[' | '{' => depth += 1,
                            ')' | ']' | '}' => {
                                if depth == 0 {
                                    break;
                                }
                                depth = depth.saturating_sub(1);
                            }
                            ',' if depth == 0 => break,
                            _ => {}
                        }
                        j += 1;
                    }
                    type_end = j;
                    let _ = type_start;
                    let _ = type_end;
                } else {
                    type_end = binder_end;
                    let _ = type_end;
                }
                if j < len && chars[j] == ',' {
                    j += 1;
                    while j < len && chars[j] == ' ' {
                        j += 1;
                    }
                    let body_start = j;
                    let mut depth = 0usize;
                    while j < len {
                        match chars[j] {
                            '(' | '[' | '{' => depth += 1,
                            ')' | ']' | '}' => {
                                if depth == 0 {
                                    break;
                                }
                                depth = depth.saturating_sub(1);
                            }
                            ':' if depth == 0 && j + 1 < len && chars[j + 1] == '=' => {
                                break;
                            }
                            _ => {}
                        }
                        j += 1;
                    }
                    let body: String = chars[body_start..j].iter().collect();
                    let binder: String = chars[binder_start..binder_end].iter().collect();
                    if has_type_ann {
                        // Find the colon position after binder_end, skip past it
                        let colon_pos = (binder_end..type_end)
                            .find(|&k| chars[k] == ':')
                            .unwrap_or(binder_end);
                        let type_str: String =
                            chars[colon_pos + 1..type_end].iter().collect::<String>();
                        let type_str = type_str.trim();
                        result.push_str(&format!("{kw} (fun ({binder} : {type_str}) -> {body})"));
                    } else {
                        result.push_str(&format!("{kw} (fun {binder} -> {body})"));
                    }
                    i = j;
                    continue;
                }
            }
            result.push_str(kw);
            result.push(' ');
            i = after_kw;
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}
/// Normalize `DFinsupp`/`Π₀` notation.
///
/// After Unicode normalization, `Π₀` becomes `DFinsupp`.
/// `DFinsupp i : ι, β i` -> `DFinsupp (fun i -> β i)`
pub(super) fn normalize_dfinsupp_type(src: &str) -> String {
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut result = String::with_capacity(src.len() + 32);
    let mut i = 0;
    while i < len {
        let prev_is_word = i > 0 && (chars[i - 1].is_alphanumeric() || chars[i - 1] == '_');
        let is_dfinsupp = !prev_is_word
            && i + 9 <= len
            && chars[i..i + 9].iter().collect::<String>() == "DFinsupp ";
        if is_dfinsupp {
            let after_kw = i + 9;
            let mut j = after_kw;
            while j < len && chars[j] == ' ' {
                j += 1;
            }
            let binder_start = j;
            while j < len && (chars[j].is_alphanumeric() || chars[j] == '_' || chars[j] == '\'') {
                j += 1;
            }
            let binder_end = j;
            if binder_end > binder_start {
                while j < len && chars[j] == ' ' {
                    j += 1;
                }
                if j < len && chars[j] == ':' {
                    j += 1;
                    while j < len && chars[j] == ' ' {
                        j += 1;
                    }
                    let mut depth = 0usize;
                    while j < len {
                        match chars[j] {
                            '(' | '[' | '{' => depth += 1,
                            ')' | ']' | '}' => {
                                if depth == 0 {
                                    break;
                                }
                                depth = depth.saturating_sub(1);
                            }
                            ',' if depth == 0 => break,
                            _ => {}
                        }
                        j += 1;
                    }
                    if j < len && chars[j] == ',' {
                        j += 1;
                        while j < len && chars[j] == ' ' {
                            j += 1;
                        }
                        let body_start = j;
                        let mut depth2 = 0usize;
                        while j < len {
                            match chars[j] {
                                '(' | '[' | '{' => depth2 += 1,
                                ')' | ']' | '}' => {
                                    if depth2 == 0 {
                                        break;
                                    }
                                    depth2 = depth2.saturating_sub(1);
                                }
                                _ => {}
                            }
                            j += 1;
                        }
                        let body: String = chars[body_start..j].iter().collect();
                        let binder: String = chars[binder_start..binder_end].iter().collect();
                        result.push_str(&format!("DFinsupp (fun {binder} -> {body})"));
                        i = j;
                        continue;
                    }
                }
            }
            result.push_str("DFinsupp ");
            i = after_kw;
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}
/// Strip term-level type ascriptions that appear after `:=`.
///
/// In Lean 4, `def f := (expr : Type)` or `theorem t := show Type from expr`
/// can cause parsing issues. This strips the ascription after `:=`.
pub(super) fn strip_term_type_ascriptions(src: &str) -> String {
    if let Some(assign_pos) = src.find(":= ") {
        let before = &src[..assign_pos + 3];
        let after = src[assign_pos + 3..].trim();
        if after == "sorry" || after.starts_with("sorry ") {
            return src.to_string();
        }
        if after.starts_with("show ") || after.starts_with("(show ") {
            return format!("{before}sorry");
        }
        if after.starts_with('(') {
            let chars: Vec<char> = after.chars().collect();
            let mut depth = 0i32;
            let mut colon_pos = None;
            for (idx, &ch) in chars.iter().enumerate() {
                match ch {
                    '(' => depth += 1,
                    ')' => {
                        depth -= 1;
                        if depth == 0 && colon_pos.is_some() {
                            return format!("{before}sorry");
                        }
                    }
                    ':' if depth == 1 && idx > 0 => {
                        let next = chars.get(idx + 1).copied().unwrap_or(' ');
                        if next != '=' {
                            colon_pos = Some(idx);
                        }
                    }
                    _ => {}
                }
            }
        }
        if after.contains('\u{27E8}') || after.contains('\u{27E9}') {
            return format!("{before}sorry");
        }
        if after.starts_with("fun ") || after.starts_with("fun\t") {
            let inner_chars: Vec<char> = after.chars().collect();
            let has_arrow = inner_chars
                .windows(2)
                .any(|w| (w[0] == '=' || w[0] == '-') && w[1] == '>' || w[0] == '\u{2192}');
            if !has_arrow {
                return format!("{before}sorry");
            }
        }
        if after.starts_with('.') && after.len() > 1 {
            let second = after.chars().nth(1).unwrap_or(' ');
            if second.is_alphabetic() || second == '_' {
                return format!("{before}sorry");
            }
        }
        let has_complex_term = after.contains("rfl")
            || after.contains("trivial")
            || after.contains("inferInstance")
            || after.contains("Iff.rfl")
            || after.contains("Subtype.mk")
            || after.contains("absurd")
            || after.contains("Or.inl")
            || after.contains("Or.inr")
            || after.contains("And.intro")
            || after.contains("Eq.mpr")
            || after.contains("Eq.mp")
            || (after.contains(".mk") && !after.starts_with("sorry"));
        if has_complex_term {
            return format!("{before}sorry");
        }
    }
    src.to_string()
}
pub(super) fn normalize_head_binders(src: &str) -> String {
    let kw = if src.starts_with("theorem ") {
        "theorem"
    } else if src.starts_with("lemma ") {
        "lemma"
    } else if src.starts_with("def ") {
        "def"
    } else if src.starts_with("axiom ") {
        "axiom"
    } else {
        return src.to_string();
    };
    let rest = src[kw.len()..].trim_start();
    let name_end = rest
        .char_indices()
        .find(|(_, c)| c.is_whitespace() || *c == '(' || *c == '{' || *c == '[' || *c == ':')
        .map(|(i, _)| i)
        .unwrap_or(rest.len());
    let raw_name = &rest[..name_end];
    let name_no_univ = if let Some(brace_pos) = raw_name.find('{') {
        &raw_name[..brace_pos]
    } else {
        raw_name
    };
    let name_owned = name_no_univ.replace('.', "_");
    let name = name_owned.as_str();
    let after_name = rest[name_end..].trim_start();
    let after_name = if after_name.starts_with('{')
        && !after_name.starts_with("{ ")
        && !after_name.contains(':')
    {
        let close = after_name
            .find('}')
            .map(|i| &after_name[i + 1..])
            .unwrap_or(after_name);
        close.trim_start()
    } else {
        after_name
    };
    let (binders, colon_rest) = collect_binders_before_colon(after_name);
    let name_changed = name != raw_name.trim_end_matches(|c: char| c == '{' || c.is_whitespace());
    if binders.is_empty() {
        let after_clean = colon_rest.trim_start();
        // Handle missing type: `theorem foo := sorry` → `theorem foo : _ := sorry`
        if after_clean.starts_with(":=") && kw != "def" {
            return format!("{kw} {name} : _ {after_clean}");
        }
        let unchanged = !name_changed && after_clean == after_name.trim_start();
        if unchanged {
            return src.to_string();
        }
        return format!("{kw} {name} {after_clean}");
    }
    if !colon_rest.starts_with(':') || colon_rest.starts_with(":=") {
        if colon_rest.starts_with(":=") {
            // def/theorem with bare binders and no type → strip binders, use sorry
            if kw == "def" {
                return format!("def {name} := sorry");
            }
            // theorem/lemma/axiom without type annotation → add dummy type
            return format!("{kw} {name} : _ := sorry");
        }
        if name_changed {
            return format!("{kw} {name} {}", after_name);
        }
        return src.to_string();
    }
    let type_and_proof = colon_rest[1..].trim_start();
    let binders_explicit = binders.trim().replace('{', "(").replace('}', ")");
    format!("{kw} {name} : forall {binders_explicit}, {type_and_proof}")
}
/// Scan a string for bracket-delimited binder groups that contain `:` annotations.
/// Returns (collected_binders_string, remainder_starting_at_colon_or_assign).
fn collect_binders_before_colon(s: &str) -> (String, &str) {
    let mut binders = String::new();
    let s_bytes = s.as_bytes();
    let mut i = 0;
    loop {
        while i < s_bytes.len() && (s_bytes[i] == b' ' || s_bytes[i] == b'\t') {
            i += 1;
        }
        if i >= s_bytes.len() {
            break;
        }
        let ch = s_bytes[i];
        if ch == b'(' || ch == b'{' || ch == b'[' {
            let _close = match ch {
                b'(' => b')',
                b'{' => b'}',
                b'[' => b']',
                _ => unreachable!(),
            };
            let start = i;
            let mut depth = 0usize;
            let mut has_colon = false;
            while i < s_bytes.len() {
                let c = s_bytes[i];
                match c {
                    b'(' | b'{' | b'[' => depth += 1,
                    b')' | b'}' | b']' => {
                        depth = depth.saturating_sub(1);
                        if depth == 0 {
                            i += 1;
                            break;
                        }
                    }
                    b':' if depth == 1 => {
                        let next = if i + 1 < s_bytes.len() {
                            s_bytes[i + 1]
                        } else {
                            0
                        };
                        if next != b'=' {
                            has_colon = true;
                        }
                    }
                    _ => {}
                }
                i += 1;
            }
            if has_colon {
                if !binders.is_empty() {
                    binders.push(' ');
                }
                if let Ok(group) = std::str::from_utf8(&s_bytes[start..i]) {
                    binders.push_str(group);
                }
            }
        } else if ch == b':' {
            break;
        } else if ch.is_ascii_alphabetic() || ch == b'_' {
            let start = i;
            while i < s_bytes.len()
                && (s_bytes[i].is_ascii_alphanumeric() || s_bytes[i] == b'_' || s_bytes[i] == b'\'')
            {
                i += 1;
            }
            let mut j = i;
            while j < s_bytes.len() && s_bytes[j] == b' ' {
                j += 1;
            }
            let next_ch = if j < s_bytes.len() { s_bytes[j] } else { 0 };
            let next_next = if j + 1 < s_bytes.len() {
                s_bytes[j + 1]
            } else {
                0
            };
            if next_ch == b':' && next_next != b'=' {
                i = start;
                break;
            }
            if let Ok(ident) = std::str::from_utf8(&s_bytes[start..i]) {
                if !binders.is_empty() {
                    binders.push(' ');
                }
                binders.push('(');
                binders.push_str(ident);
                binders.push_str(" : _)");
            }
        } else {
            break;
        }
    }
    (binders, &s[i..])
}

/// Normalize `fun BINDERS, body` (comma-lambda) to `fun BINDERS -> body`.
///
/// Lean 4 sometimes uses `fun x, body` with a comma instead of `=>`.
/// After the `=> → ->` normalization, these comma-lambdas remain.
/// Only replaces the FIRST top-level comma (depth 0) after `fun `.
pub(super) fn normalize_fun_comma_lambda(src: &str) -> String {
    if !src.contains("fun ") {
        return src.to_string();
    }
    let mut result = String::with_capacity(src.len());
    let mut rest = src;
    while let Some(fun_pos) = rest.find("fun ") {
        // Check word boundary: fun must not be preceded by alphanumeric/underscore
        if fun_pos > 0 {
            let prev = rest.as_bytes()[fun_pos - 1];
            if prev.is_ascii_alphanumeric() || prev == b'_' || prev == b'\'' {
                result.push_str(&rest[..fun_pos + 4]);
                rest = &rest[fun_pos + 4..];
                continue;
            }
        }
        result.push_str(&rest[..fun_pos + 4]); // push up to and including "fun "
        rest = &rest[fun_pos + 4..];
        // Now scan for the first comma at depth 0
        let bytes = rest.as_bytes();
        let len = bytes.len();
        let mut depth = 0usize;
        let mut comma_pos = None;
        let mut has_arrow = false;
        let mut i = 0;
        while i < len {
            match bytes[i] {
                b'(' | b'[' | b'{' => depth += 1,
                b')' | b']' | b'}' => depth = depth.saturating_sub(1),
                b'-' if depth == 0 && i + 1 < len && bytes[i + 1] == b'>' => {
                    // Already has an arrow, skip this fun
                    has_arrow = true;
                    break;
                }
                b':' if depth == 0 && i + 1 < len && bytes[i + 1] == b'=' => {
                    // Hit := at depth 0, stop
                    break;
                }
                b',' if depth == 0 => {
                    comma_pos = Some(i);
                    break;
                }
                _ => {}
            }
            i += 1;
        }
        if !has_arrow {
            if let Some(cp) = comma_pos {
                // Check if the text before the comma contains `∀` or `forall` at depth 0.
                // If so, the comma is a forall/exists body separator, NOT a fun body separator.
                let before_comma = &rest[..cp];
                let has_quantifier_at_depth0 = {
                    let mut d = 0usize;
                    let mut found = false;
                    let bts = before_comma.as_bytes();
                    let mut k = 0;
                    while k < bts.len() {
                        match bts[k] {
                            b'(' | b'[' | b'{' => d += 1,
                            b')' | b']' | b'}' => d = d.saturating_sub(1),
                            0xE2 if d == 0
                                && k + 2 < bts.len()
                                && bts[k + 1] == 0x88
                                && bts[k + 2] == 0x80 =>
                            {
                                // ∀ (U+2200) = E2 88 80
                                found = true;
                                break;
                            }
                            0xE2 if d == 0
                                && k + 2 < bts.len()
                                && bts[k + 1] == 0x88
                                && bts[k + 2] == 0x83 =>
                            {
                                // ∃ (U+2203) = E2 88 83
                                found = true;
                                break;
                            }
                            b'f' if d == 0 && k + 6 <= bts.len() && &bts[k..k + 6] == b"forall" => {
                                found = true;
                                break;
                            }
                            b'e' if d == 0 && k + 6 <= bts.len() && &bts[k..k + 6] == b"exists" => {
                                found = true;
                                break;
                            }
                            _ => {}
                        }
                        k += 1;
                    }
                    found
                };
                if !has_quantifier_at_depth0 {
                    result.push_str(&rest[..cp]);
                    result.push_str(" ->");
                    rest = &rest[cp + 1..];
                    continue;
                }
            }
        }
        // No comma found or already has arrow, just continue
    }
    result.push_str(rest);
    result
}

/// Strip tick-subscript annotations (`'ident`) that appear after `]` or `)`.
///
/// In Lean 4, `expr'proof` uses `'` followed by an identifier as a proof obligation
/// annotation. After subscript normalization (`ident[n]` → `ident`), these ticks remain
/// and cause lexer errors ("unexpected tick").
///
/// Only strips `'ident` when immediately preceded by `]` or `)`.
pub(super) fn strip_tick_subscript(src: &str) -> String {
    if !src.contains('\'') {
        return src.to_string();
    }
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut result = String::with_capacity(src.len());
    let mut i = 0;
    while i < len {
        if chars[i] == '\'' && i > 0 && (chars[i - 1] == ']' || chars[i - 1] == ')') {
            // Check if followed by an identifier (alphanumeric/underscore)
            let start = i + 1;
            let mut j = start;
            while j < len && (chars[j].is_alphanumeric() || chars[j] == '_') {
                j += 1;
            }
            if j > start {
                // Skip the tick and the identifier
                i = j;
                continue;
            }
            // Also handle `]'(...)` or `)'(...)` — strip tick and paren group
            if j < len && chars[j] == '(' {
                let mut depth = 1usize;
                let mut k = j + 1;
                while k < len && depth > 0 {
                    match chars[k] {
                        '(' => depth += 1,
                        ')' => depth -= 1,
                        _ => {}
                    }
                    k += 1;
                }
                if depth == 0 {
                    // Skip the tick and the entire paren group
                    i = k;
                    continue;
                }
            }
            // Also strip bare tick after ] or ) with no following ident/paren
            // (e.g., `]' ` or `)' `)
            if j < len && (chars[j] == ' ' || chars[j] == ')' || chars[j] == ']') {
                // Skip just the tick
                i = j;
                continue;
            }
        }
        // Handle standalone tick as operator: ` ' ` → ` `
        if chars[i] == '\'' && i > 0 && chars[i - 1] == ' ' && i + 1 < len && chars[i + 1] == ' ' {
            // Check this isn't part of an identifier (like x' y)
            // A standalone tick between spaces is an operator — strip it
            i += 1;
            continue;
        }
        result.push(chars[i]);
        i += 1;
    }
    result
}
