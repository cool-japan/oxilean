//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

/// Normalize `BigProd`/`BigSum` bounded notation to lambda form.
///
/// `BigProd p Mem s, body` → `BigProd s (fun p -> body)`
/// `BigSum p Mem s, body` → `BigSum s (fun p -> body)`
/// Also handles `BigProd p < n, body` → `BigProd n (fun p -> body)`
///
/// Called after Unicode normalization has replaced ∏ → BigProd, ∑ → BigSum.
pub(super) fn normalize_big_prod_sum(src: &str) -> String {
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut result = String::with_capacity(src.len() + 32);
    let mut i = 0;
    while i < len {
        let prev_is_word = i > 0 && (chars[i - 1].is_alphanumeric() || chars[i - 1] == '_');
        let is_bigprod = !prev_is_word
            && i + 8 <= len
            && chars[i..i + 8].iter().collect::<String>() == "BigProd ";
        let is_bigsum = !prev_is_word
            && i + 7 <= len
            && chars[i..i + 7].iter().collect::<String>() == "BigSum ";
        let is_bigunion = !prev_is_word
            && i + 9 <= len
            && chars[i..i + 9].iter().collect::<String>() == "BigUnion ";
        let is_biginter = !prev_is_word
            && i + 9 <= len
            && chars[i..i + 9].iter().collect::<String>() == "BigInter ";
        let is_isup =
            !prev_is_word && i + 5 <= len && chars[i..i + 5].iter().collect::<String>() == "ISup ";
        let is_iinf =
            !prev_is_word && i + 5 <= len && chars[i..i + 5].iter().collect::<String>() == "IInf ";
        if is_bigprod || is_bigsum || is_bigunion || is_biginter || is_isup || is_iinf {
            let (kw, kw_len) = if is_bigprod {
                ("BigProd", 7usize)
            } else if is_bigsum {
                ("BigSum", 6usize)
            } else if is_bigunion {
                ("BigUnion", 8usize)
            } else if is_biginter {
                ("BigInter", 8usize)
            } else if is_isup {
                ("ISup", 4usize)
            } else {
                ("IInf", 4usize)
            };
            i += kw_len + 1;
            while i < len && chars[i] == ' ' {
                i += 1;
            }
            let binder_start = i;
            if i < len && (chars[i] == '(' || chars[i] == '{' || chars[i] == '[') {
                while i < len && (chars[i] == '(' || chars[i] == '{' || chars[i] == '[') {
                    let open = chars[i];
                    let close = match open {
                        '(' => ')',
                        '{' => '}',
                        _ => ']',
                    };
                    let mut depth = 0usize;
                    while i < len {
                        if chars[i] == open {
                            depth += 1;
                        } else if chars[i] == close {
                            depth -= 1;
                            if depth == 0 {
                                i += 1;
                                break;
                            }
                        }
                        i += 1;
                    }
                    while i < len && chars[i] == ' ' {
                        i += 1;
                    }
                }
                let binder: String = chars[binder_start..i].iter().collect();
                let binder = binder.trim().to_string();
                if i < len && chars[i] == ',' {
                    i += 1;
                    while i < len && chars[i] == ' ' {
                        i += 1;
                    }
                    let body_start = i;
                    let mut depth = 0usize;
                    let mut body_end = i;
                    while i < len {
                        match chars[i] {
                            '(' | '{' | '[' => {
                                depth += 1;
                                i += 1;
                                body_end = i;
                            }
                            ')' | '}' | ']' if depth == 0 => break,
                            ')' | '}' | ']' => {
                                depth -= 1;
                                i += 1;
                                body_end = i;
                            }
                            ':' if depth == 0 && i + 1 < len && chars[i + 1] == '=' => {
                                break;
                            }
                            _ => {
                                i += 1;
                                body_end = i;
                            }
                        }
                    }
                    let body: String = chars[body_start..body_end].iter().collect();
                    let body = normalize_big_prod_sum(body.trim_end());
                    result.push_str(kw);
                    result.push(' ');
                    result.push_str(&format!("(fun {} -> {})", binder.trim(), body.trim_end()));
                } else {
                    result.push_str(kw);
                    result.push(' ');
                    result.push_str(&binder);
                    result.push(' ');
                }
                continue;
            }
            let ident_start = i;
            while i < len && (chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == '\'') {
                i += 1;
            }
            while i < len
                && chars[i] == '.'
                && i + 1 < len
                && (chars[i + 1].is_alphanumeric() || chars[i + 1] == '_')
            {
                i += 1;
                while i < len && (chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == '\'')
                {
                    i += 1;
                }
            }
            let ident: String = chars[ident_start..i].iter().collect();
            if ident.is_empty() {
                result.push_str(kw);
                result.push(' ');
                continue;
            }
            while i < len && chars[i] == ' ' {
                i += 1;
            }
            let has_mem = i + 3 <= len
                && chars[i..i + 3].iter().collect::<String>() == "Mem"
                && (i + 3 >= len || !chars[i + 3].is_alphanumeric());
            let has_lt = i < len && chars[i] == '<';
            let has_le = i < len && chars[i] == '\u{2264}';
            let has_comma = i < len && chars[i] == ',';
            let has_typed_binder =
                i < len && chars[i] == ':' && (i + 1 >= len || chars[i + 1] != '=');
            if !has_mem && !has_lt && !has_le {
                if has_typed_binder {
                    i += 1;
                    while i < len && chars[i] == ' ' {
                        i += 1;
                    }
                    let type_start = i;
                    let mut depth = 0usize;
                    while i < len {
                        match chars[i] {
                            '(' | '{' | '[' => {
                                depth += 1;
                                i += 1;
                            }
                            ')' | '}' | ']' if depth == 0 => break,
                            ')' | '}' | ']' => {
                                depth -= 1;
                                i += 1;
                            }
                            ',' if depth == 0 => break,
                            _ => {
                                i += 1;
                            }
                        }
                    }
                    let ty: String = chars[type_start..i].iter().collect();
                    let ty = ty.trim().to_string();
                    if i < len && chars[i] == ',' {
                        i += 1;
                        while i < len && chars[i] == ' ' {
                            i += 1;
                        }
                        let body_start = i;
                        let mut depth = 0usize;
                        let mut body_end = i;
                        while i < len {
                            match chars[i] {
                                '(' | '{' | '[' => {
                                    depth += 1;
                                    i += 1;
                                    body_end = i;
                                }
                                ')' | '}' | ']' if depth == 0 => break,
                                ')' | '}' | ']' => {
                                    depth -= 1;
                                    i += 1;
                                    body_end = i;
                                }
                                ':' if depth == 0 && i + 1 < len && chars[i + 1] == '=' => {
                                    break;
                                }
                                _ => {
                                    i += 1;
                                    body_end = i;
                                }
                            }
                        }
                        let body: String = chars[body_start..body_end].iter().collect();
                        let body = normalize_big_prod_sum(body.trim_end());
                        let ident_str = if ident == "_" {
                            "x".to_string()
                        } else {
                            ident.clone()
                        };
                        result.push_str(kw);
                        result.push(' ');
                        result.push_str(&format!(
                            "(fun ({} : {}) -> {})",
                            ident_str,
                            ty,
                            body.trim_end()
                        ));
                    } else {
                        result.push_str(kw);
                        result.push(' ');
                        result.push_str(&ident);
                        result.push_str(" : ");
                        result.push_str(&ty);
                        result.push(' ');
                    }
                } else if has_comma {
                    i += 1;
                    while i < len && chars[i] == ' ' {
                        i += 1;
                    }
                    let body_start = i;
                    let mut depth = 0usize;
                    let mut body_end = i;
                    while i < len {
                        match chars[i] {
                            '(' | '{' | '[' => {
                                depth += 1;
                                i += 1;
                                body_end = i;
                            }
                            ')' | '}' | ']' if depth == 0 => break,
                            ')' | '}' | ']' => {
                                depth -= 1;
                                i += 1;
                                body_end = i;
                            }
                            ':' if depth == 0 && i + 1 < len && chars[i + 1] == '=' => {
                                break;
                            }
                            _ => {
                                i += 1;
                                body_end = i;
                            }
                        }
                    }
                    let body: String = chars[body_start..body_end].iter().collect();
                    let body = normalize_big_prod_sum(body.trim_end());
                    let ident_bare = if ident == "_" {
                        "x".to_string()
                    } else {
                        ident.clone()
                    };
                    result.push_str(kw);
                    result.push(' ');
                    result.push_str(&format!("(fun {} -> {})", ident_bare, body.trim_end()));
                } else {
                    result.push_str(kw);
                    result.push(' ');
                    result.push_str(&ident);
                    result.push(' ');
                }
                continue;
            }
            if has_mem {
                i += 3;
            } else {
                i += 1;
            }
            while i < len && chars[i] == ' ' {
                i += 1;
            }
            let range_start = i;
            let mut depth = 0usize;
            while i < len {
                match chars[i] {
                    '(' | '{' | '[' => {
                        depth += 1;
                        i += 1;
                    }
                    ')' | '}' | ']' if depth == 0 => break,
                    ')' | '}' | ']' => {
                        depth = depth.saturating_sub(1);
                        i += 1;
                    }
                    ',' if depth == 0 => break,
                    _ => {
                        i += 1;
                    }
                }
            }
            let range_expr: String = chars[range_start..i].iter().collect();
            if i < len && chars[i] == ',' {
                i += 1;
            }
            while i < len && chars[i] == ' ' {
                i += 1;
            }
            let body_start = i;
            let mut depth = 0usize;
            let mut body_end = i;
            while i < len {
                match chars[i] {
                    '(' | '{' | '[' => {
                        depth += 1;
                        i += 1;
                        body_end = i;
                    }
                    ')' | '}' | ']' if depth == 0 => break,
                    ')' | '}' | ']' => {
                        depth = depth.saturating_sub(1);
                        i += 1;
                        body_end = i;
                    }
                    ':' if depth == 0 && i + 1 < len && chars[i + 1] == '=' => break,
                    _ => {
                        i += 1;
                        body_end = i;
                    }
                }
            }
            let body: String = chars[body_start..body_end].iter().collect();
            let body = normalize_big_prod_sum(body.trim_end());
            result.push_str(kw);
            result.push(' ');
            result.push_str(range_expr.trim());
            result.push_str(" (fun ");
            result.push_str(&ident);
            result.push_str(" -> ");
            result.push_str(body.trim_end());
            result.push(')');
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}
/// Normalize `∃!` (unique existence) quantifiers to `ExistsUnique (fun binder -> body)`.
///
/// Must run BEFORE `normalize_exists_quantifier` since `∃!` starts with `∃` (U+2203).
/// `∃! x : T, P x` → `ExistsUnique (fun (x : T) -> P x)`
/// `∃! _ : T, P` → `ExistsUnique (fun (hole_0 : T) -> P)`
/// `∃! x, P x` → `ExistsUnique (fun x -> P x)`
pub(super) fn normalize_exists_unique(src: &str) -> String {
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut result = String::with_capacity(src.len() + 32);
    let mut i = 0;
    while i < len {
        let prev_is_word = i > 0 && {
            let p = chars[i - 1];
            p.is_alphanumeric() || p == '_' || p == '\''
        };
        let is_exists_unique =
            chars[i] == '\u{2203}' && !prev_is_word && i + 1 < len && chars[i + 1] == '!';
        if is_exists_unique {
            i += 2;
            while i < len && chars[i] == ' ' {
                i += 1;
            }
            if i >= len {
                result.push_str("ExistsUnique");
                break;
            }
            if chars[i] == '(' || chars[i] == '{' || chars[i] == '[' {
                let binder_start = i;
                let mut depth = 0usize;
                while i < len {
                    match chars[i] {
                        '(' | '{' | '[' => {
                            depth += 1;
                            i += 1;
                        }
                        ')' | '}' | ']' => {
                            depth = depth.saturating_sub(1);
                            i += 1;
                            if depth == 0 {
                                break;
                            }
                        }
                        _ => {
                            i += 1;
                        }
                    }
                }
                let binder: String = chars[binder_start..i].iter().collect();
                while i < len && chars[i] == ' ' {
                    i += 1;
                }
                if i < len && chars[i] == ',' {
                    i += 1;
                }
                while i < len && chars[i] == ' ' {
                    i += 1;
                }
                result.push_str("ExistsUnique (fun ");
                result.push_str(&binder);
                result.push_str(" -> ");
            } else {
                let binder_start = i;
                while i < len && (chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == '\'')
                {
                    i += 1;
                }
                let raw_name: String = chars[binder_start..i].iter().collect();
                let binder_name = if raw_name == "_" {
                    "hole_0".to_string()
                } else {
                    raw_name
                };
                while i < len && chars[i] == ' ' {
                    i += 1;
                }
                if i < len && chars[i] == ':' && (i + 1 >= len || chars[i + 1] != '=') {
                    i += 1;
                    while i < len && chars[i] == ' ' {
                        i += 1;
                    }
                    let type_start = i;
                    let mut depth = 0usize;
                    while i < len {
                        match chars[i] {
                            '(' | '{' | '[' => {
                                depth += 1;
                                i += 1;
                            }
                            ')' | '}' | ']' if depth == 0 => break,
                            ')' | '}' | ']' => {
                                depth = depth.saturating_sub(1);
                                i += 1;
                            }
                            ',' if depth == 0 => break,
                            ':' if depth == 0 && i + 1 < len && chars[i + 1] == '=' => {
                                break;
                            }
                            _ => {
                                i += 1;
                            }
                        }
                    }
                    let type_part: String = chars[type_start..i].iter().collect();
                    if i < len && chars[i] == ',' {
                        i += 1;
                    }
                    while i < len && chars[i] == ' ' {
                        i += 1;
                    }
                    result.push_str("ExistsUnique (fun (");
                    result.push_str(&binder_name);
                    result.push_str(" : ");
                    result.push_str(type_part.trim());
                    result.push_str(") -> ");
                } else {
                    if i < len && chars[i] == ',' {
                        i += 1;
                    }
                    while i < len && chars[i] == ' ' {
                        i += 1;
                    }
                    result.push_str("ExistsUnique (fun ");
                    result.push_str(&binder_name);
                    result.push_str(" -> ");
                }
            }
            let body_start = i;
            let mut depth = 0usize;
            while i < len {
                match chars[i] {
                    '(' | '{' | '[' => {
                        depth += 1;
                        i += 1;
                    }
                    ')' | '}' | ']' if depth == 0 => break,
                    ')' | '}' | ']' => {
                        depth = depth.saturating_sub(1);
                        i += 1;
                    }
                    ':' if depth == 0 && i + 1 < len && chars[i + 1] == '=' => break,
                    _ => {
                        i += 1;
                    }
                }
            }
            let body: String = chars[body_start..i].iter().collect();
            result.push_str(body.trim_end());
            result.push(')');
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}
/// Convert `∃ binder, body` expressions to `Exists (fun binder -> body)`.
/// OxiLean's parser does not handle `∃` as an expression keyword (no `parse_exists`).
/// `∃ a, P a` → `Exists (fun a -> P a)`
/// `∃ (a : T), P a` → `Exists (fun (a : T) -> P a)`
pub(super) fn normalize_exists_quantifier(src: &str) -> String {
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut result = String::with_capacity(src.len() + 16);
    let mut i = 0;
    while i < len {
        let is_exists_unicode = chars[i] == '\u{2203}';
        let prev_is_word = i > 0 && {
            let p = chars[i - 1];
            p.is_alphanumeric() || p == '_' || p == '\''
        };
        let is_exists_unicode = is_exists_unicode && !prev_is_word;
        if is_exists_unicode {
            i += 1;
            let mut extra_close_parens = 0usize;
            while i < len && chars[i] == ' ' {
                i += 1;
            }
            if i >= len {
                result.push_str("Exists");
                break;
            }
            if chars[i] == '(' || chars[i] == '{' || chars[i] == '[' {
                let binder_start = i;
                let mut depth = 0usize;
                while i < len {
                    match chars[i] {
                        '(' | '{' | '[' => {
                            depth += 1;
                            i += 1;
                        }
                        ')' | '}' | ']' => {
                            depth = depth.saturating_sub(1);
                            i += 1;
                            if depth == 0 {
                                break;
                            }
                        }
                        _ => {
                            i += 1;
                        }
                    }
                }
                let binder: String = chars[binder_start..i].iter().collect();
                while i < len && chars[i] == ' ' {
                    i += 1;
                }
                if i < len && chars[i] == ',' {
                    i += 1;
                }
                while i < len && chars[i] == ' ' {
                    i += 1;
                }
                result.push_str("Exists (fun ");
                result.push_str(&binder);
                result.push_str(" -> ");
            } else {
                let binder_start = i;
                while i < len && (chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == '\'')
                {
                    i += 1;
                }
                let first_name: String = chars[binder_start..i].iter().collect();
                let mut all_names: Vec<String> = vec![first_name];
                loop {
                    let saved_i = i;
                    while i < len && chars[i] == ' ' {
                        i += 1;
                    }
                    if i < len && (chars[i].is_alphabetic() || chars[i] == '_') {
                        let extra_start = i;
                        let mut j = i;
                        while j < len
                            && (chars[j].is_alphanumeric() || chars[j] == '_' || chars[j] == '\'')
                        {
                            j += 1;
                        }
                        let mut k = j;
                        while k < len && chars[k] == ' ' {
                            k += 1;
                        }
                        let next_is_colon =
                            k < len && chars[k] == ':' && (k + 1 >= len || chars[k + 1] != '=');
                        if next_is_colon {
                            let extra_name: String = chars[extra_start..j].iter().collect();
                            all_names.push(extra_name);
                            i = j;
                            continue;
                        }
                        i = saved_i;
                    } else {
                        i = saved_i;
                    }
                    break;
                }
                while i < len && chars[i] == ' ' {
                    i += 1;
                }
                if i < len && chars[i] == ':' && (i + 1 >= len || chars[i + 1] != '=') {
                    i += 1;
                    while i < len && chars[i] == ' ' {
                        i += 1;
                    }
                    let type_start = i;
                    let mut depth = 0usize;
                    while i < len {
                        match chars[i] {
                            '(' | '{' | '[' => {
                                depth += 1;
                                i += 1;
                            }
                            ')' | '}' | ']' if depth == 0 => {
                                break;
                            }
                            ')' | '}' | ']' => {
                                depth = depth.saturating_sub(1);
                                i += 1;
                            }
                            ',' if depth == 0 => {
                                break;
                            }
                            ':' if depth == 0 && i + 1 < len && chars[i + 1] == '=' => {
                                break;
                            }
                            _ => {
                                i += 1;
                            }
                        }
                    }
                    let type_part: String = chars[type_start..i].iter().collect();
                    let type_trimmed = type_part.trim().to_string();
                    if i < len && chars[i] == ',' {
                        i += 1;
                    }
                    while i < len && chars[i] == ' ' {
                        i += 1;
                    }
                    for name in &all_names {
                        result.push_str("Exists (fun (");
                        result.push_str(name);
                        result.push_str(" : ");
                        result.push_str(&type_trimmed);
                        result.push_str(") -> ");
                    }
                    extra_close_parens = all_names.len().saturating_sub(1);
                } else {
                    if i < len && chars[i] == ',' {
                        i += 1;
                    }
                    while i < len && chars[i] == ' ' {
                        i += 1;
                    }
                    result.push_str("Exists (fun ");
                    result.push_str(&all_names[0]);
                    result.push_str(" -> ");
                }
            }
            let body_start = i;
            let mut depth = 0usize;
            while i < len {
                match chars[i] {
                    '(' | '{' | '[' => {
                        depth += 1;
                        i += 1;
                    }
                    ')' | '}' | ']' if depth == 0 => {
                        break;
                    }
                    ')' | '}' | ']' => {
                        depth = depth.saturating_sub(1);
                        i += 1;
                    }
                    ':' if depth == 0 && i + 1 < len && chars[i + 1] == '=' => {
                        break;
                    }
                    _ => {
                        i += 1;
                    }
                }
            }
            let body: String = chars[body_start..i].iter().collect();
            result.push_str(body.trim_end());
            result.push(')');
            for _ in 0..extra_close_parens {
                result.push(')');
            }
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}
/// Parenthesize bare (unbracketed) binders after `forall`/`∀`.
/// `∀ h : A, body` → `∀ (h : A), body`
/// `∀ x y : T, body` → `∀ (x y : T), body`
/// OxiLean requires typed binders to be wrapped in `()`, `{}`, or `[]`.
pub(super) fn parenthesize_bare_forall_binders(src: &str) -> String {
    let mut result = String::with_capacity(src.len() + 32);
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut i = 0;
    while i < len {
        let prev_is_word = i > 0 && {
            let p = chars[i - 1];
            p.is_alphanumeric() || p == '_' || p == '\''
        };
        let is_forall_unicode = chars[i] == '\u{2200}' && !prev_is_word;
        let is_forall_word = !prev_is_word
            && len >= i + 6
            && chars[i..i + 6].iter().collect::<String>() == "forall"
            && (i + 6 >= len || {
                let next = chars[i + 6];
                !next.is_alphanumeric() && next != '_' && next != '\''
            });
        if is_forall_unicode || is_forall_word {
            let kw_len = if is_forall_unicode { 1 } else { 6 };
            let kw: String = chars[i..i + kw_len].iter().collect();
            result.push_str(&kw);
            i += kw_len;
            let ws_start = i;
            while i < len && chars[i] == ' ' {
                result.push(chars[i]);
                i += 1;
            }
            let _ = ws_start;
            if i < len && (chars[i] == '(' || chars[i] == '{' || chars[i] == '[') {
                continue;
            }
            let binder_start = i;
            let mut j = i;
            let mut found_colon = false;
            let mut colon_pos = 0;
            while j < len && chars[j] != ',' && chars[j] != ')' && chars[j] != '\n' {
                if chars[j] == ':' && j + 1 < len && chars[j + 1] != '=' {
                    found_colon = true;
                    colon_pos = j;
                    break;
                }
                if chars[j] == '(' || chars[j] == '{' || chars[j] == '[' {
                    break;
                }
                j += 1;
            }
            if !found_colon {
                continue;
            }
            let binder_prefix: String = chars[binder_start..colon_pos].iter().collect();
            let all_ident = binder_prefix
                .chars()
                .all(|c| c.is_alphanumeric() || c == '_' || c == '\'' || c == ' ');
            if !all_ident {
                continue;
            }
            let mut k = colon_pos + 1;
            let mut depth = 0usize;
            let mut comma_pos = None;
            while k < len {
                match chars[k] {
                    '(' | '{' | '[' => {
                        depth += 1;
                        k += 1;
                    }
                    ')' | '}' | ']' => {
                        if depth > 0 {
                            depth -= 1;
                            k += 1;
                        } else {
                            break;
                        }
                    }
                    ',' if depth == 0 => {
                        comma_pos = Some(k);
                        break;
                    }
                    _ => {
                        k += 1;
                    }
                }
            }
            if let Some(cp) = comma_pos {
                let type_part: String = chars[colon_pos + 1..cp].iter().collect();
                let binders_part = binder_prefix.trim();
                result.push('(');
                result.push_str(binders_part);
                result.push_str(" :");
                result.push_str(&type_part);
                result.push(')');
                i = cp;
            } else {
                let bare: String = chars[binder_start..j].iter().collect();
                result.push_str(&bare);
                i = j;
            }
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}
pub(super) fn strip_where_block(src: &str) -> String {
    if let Some(where_pos) = src.find(" where ") {
        let before_where = &src[..where_pos];
        if before_where.contains(':') {
            return format!("{before_where} := sorry");
        }
    }
    src.to_string()
}
pub(super) fn strip_universe_annotations(src: &str) -> String {
    let bytes = src.as_bytes();
    let len = bytes.len();
    let mut result = String::with_capacity(len);
    let mut pos = 0usize;
    while pos < len {
        if let Some(rel) = bytes[pos..]
            .windows(2)
            .position(|w| w[0] == b'.' && w[1] == b'{')
        {
            let dot_pos = pos + rel;
            result.push_str(&src[pos..dot_pos]);
            let mut i = dot_pos + 1;
            let mut depth = 0usize;
            while i < len {
                match bytes[i] {
                    b'{' => {
                        depth += 1;
                        i += 1;
                    }
                    b'}' => {
                        depth = depth.saturating_sub(1);
                        i += 1;
                        if depth == 0 {
                            break;
                        }
                    }
                    _ => {
                        i += 1;
                    }
                }
            }
            pos = i;
        } else {
            result.push_str(&src[pos..]);
            break;
        }
    }
    result
}
/// Strip `@[attr]` annotations that appear before the declaration keyword.
pub(super) fn strip_attributes(src: &str) -> String {
    let s = src.trim_start();
    if !s.starts_with("@[") {
        return src.to_string();
    }
    let mut depth = 0usize;
    let mut i = 0;
    for (idx, ch) in s.char_indices() {
        match ch {
            '[' => depth += 1,
            ']' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    i = idx + 1;
                    break;
                }
            }
            _ => {}
        }
    }
    while i < s.len() && s.as_bytes()[i] == b' ' {
        i += 1;
    }
    s[i..].to_string()
}
/// Replace ALL proof terms after `:=` with `:= sorry`.
///
/// Strip `have : T := val` constructs from return type positions.
///
/// Lean 4 allows `theorem foo : have : T := val; rest_type := proof`
/// where `have : T := val` introduces a local hypothesis in the return type.
/// These are extracted incorrectly as single-line declarations because `find_top_level_assign`
/// finds the `:=` inside `have : T := val` rather than the actual proof.
///
/// Strategy: if the string looks like `keyword name ... : have : ... :=`,
/// replace the entire `have : T :=` portion (and everything after) with `:= sorry`.
pub(super) fn normalize_have_in_type(src: &str) -> String {
    let kw_list = [": have : ", ": haveI : ", ": letI : "];
    let bytes = src.as_bytes();
    let len = bytes.len();
    for kw in &kw_list {
        let kw_bytes = kw.as_bytes();
        let kw_len = kw_bytes.len();
        let mut depth = 0usize;
        let mut i = 0;
        let mut found_pos = None;
        while i < len {
            match bytes[i] {
                b'(' | b'{' | b'[' => {
                    depth += 1;
                    i += 1;
                }
                b')' | b'}' | b']' => {
                    depth = depth.saturating_sub(1);
                    i += 1;
                }
                b':' if depth == 0 && i + 1 < len && bytes[i + 1] == b'=' => {
                    break;
                }
                _ => {
                    if depth == 0 && i + kw_len <= len && &bytes[i..i + kw_len] == kw_bytes {
                        found_pos = Some(i + 1);
                        break;
                    }
                    i += 1;
                }
            }
        }
        if let Some(have_pos) = found_pos {
            let before_have = &src[..have_pos];
            return format!("{before_have}True := sorry");
        }
    }
    src.to_string()
}
/// This replaces both tactic proofs (`:= by ...`) and term proofs (`:= ⟨h, h.1⟩`).
/// The goal is to measure whether the TYPE/SIGNATURE (before `:=`) can parse,
/// not whether OxiLean can handle the specific proof term.
pub(super) fn replace_proof_with_sorry(src: &str) -> String {
    if let Some(pos) = find_top_level_assign(src) {
        let before = &src[..pos];
        format!("{before}:= sorry")
    } else {
        src.to_string()
    }
}
/// Find the byte position of the top-level `:=` assignment in a declaration.
/// Must be at depth 0 (not inside parentheses/brackets).
pub(super) fn find_top_level_assign(src: &str) -> Option<usize> {
    let bytes = src.as_bytes();
    let len = bytes.len();
    let mut depth = 0usize;
    let mut i = 0;
    while i < len {
        match bytes[i] {
            b'(' | b'{' | b'[' => {
                depth += 1;
                i += 1;
            }
            b')' | b'}' | b']' => {
                depth = depth.saturating_sub(1);
                i += 1;
            }
            b':' if depth == 0 && i + 1 < len && bytes[i + 1] == b'=' => {
                return Some(i);
            }
            b'"' => {
                i += 1;
                while i < len && bytes[i] != b'"' {
                    if bytes[i] == b'\\' {
                        i += 1;
                    }
                    i += 1;
                }
                i += 1;
            }
            _ => {
                i += 1;
            }
        }
    }
    None
}
/// Parenthesize `identifier.identifier` dot expressions.
///
/// OxiLean's parser treats `f x.field` as `(f x).field` rather than `f (x.field)`,
/// because the postfix dot loop runs at `parse_expr_prec` level (wrapping the ENTIRE
/// prefix expression), not per-argument. This causes:
///   `reverse l.revzip = reverse l.reverse`
/// to fail because after parsing `reverse l` and the outer `.revzip`, the second
/// `.reverse` has no outer wrapper to consume it.
///
/// The fix: wrap `word.word` patterns in explicit parentheses so:
///   `l.revzip` → `(l.revzip)` as a function argument
/// This makes the parse unambiguous.
pub(super) fn parenthesize_dot_exprs(src: &str) -> String {
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut result = String::with_capacity(src.len() + 32);
    let mut i = 0;
    while i < len {
        let ch = chars[i];
        if ch.is_alphanumeric() || ch == '_' || ch == '\'' {
            let ident_start = i;
            while i < len && (chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == '\'') {
                i += 1;
            }
            let ident: String = chars[ident_start..i].iter().collect();
            if i < len
                && chars[i] == '.'
                && i + 1 < len
                && (chars[i + 1].is_alphabetic() || chars[i + 1] == '_')
            {
                let dot_pos = i;
                i += 1;
                let field_start = i;
                while i < len && (chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == '\'')
                {
                    i += 1;
                }
                let field: String = chars[field_start..i].iter().collect();
                let prev_char = if ident_start > 0 {
                    chars[ident_start - 1]
                } else {
                    '\0'
                };
                let needs_paren = prev_char == ' '
                    || prev_char == '\t'
                    || prev_char == ','
                    || prev_char == '('
                    || prev_char == '['
                    || prev_char == '{';
                let is_kw = matches!(
                    ident.as_str(),
                    "theorem"
                        | "def"
                        | "lemma"
                        | "axiom"
                        | "forall"
                        | "fun"
                        | "let"
                        | "in"
                        | "match"
                        | "with"
                        | "if"
                        | "then"
                        | "else"
                        | "have"
                        | "show"
                        | "by"
                        | "sorry"
                );
                let already_in_paren = result.ends_with('(');
                let _ = dot_pos;
                if needs_paren && !is_kw && !already_in_paren {
                    let mut expr = format!("({}.{})", ident, field);
                    while i < len
                        && chars[i] == '.'
                        && i + 1 < len
                        && (chars[i + 1].is_alphabetic() || chars[i + 1] == '_')
                    {
                        i += 1;
                        let chain_field_start = i;
                        while i < len
                            && (chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == '\'')
                        {
                            i += 1;
                        }
                        let chain_field: String = chars[chain_field_start..i].iter().collect();
                        expr = format!("({}.{})", expr, chain_field);
                    }
                    result.push_str(&expr);
                } else {
                    result.push_str(&ident);
                    result.push('.');
                    result.push_str(&field);
                    while i < len
                        && chars[i] == '.'
                        && i + 1 < len
                        && (chars[i + 1].is_alphabetic() || chars[i + 1] == '_')
                    {
                        result.push('.');
                        i += 1;
                        let chain_field_start = i;
                        while i < len
                            && (chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == '\'')
                        {
                            i += 1;
                        }
                        let chain_field: String = chars[chain_field_start..i].iter().collect();
                        result.push_str(&chain_field);
                    }
                }
            } else {
                result.push_str(&ident);
            }
        } else {
            result.push(ch);
            i += 1;
        }
    }
    result
}
/// Strip `@identifier` explicit-argument override syntax.
///
/// In Lean 4, `@Nat.add_comm` means "explicit version of Nat.add_comm".
/// In OxiLean, `@` is not a supported prefix. We strip it.
/// Note: This only strips `@name` patterns (not `@[` which is an attribute).
pub(super) fn strip_explicit_at_prefix(src: &str) -> String {
    let mut result = String::with_capacity(src.len());
    let mut chars = src.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '@' {
            if let Some(&next) = chars.peek() {
                if next != '[' && (next.is_alphanumeric() || next == '_') {
                    continue;
                }
            }
        }
        result.push(ch);
    }
    result
}
/// Normalize bounded quantifier notation from `ISup`/`IInf` to lambda form.
///
/// `ISup k < expr, body` → `ISup (fun k -> body)` (dropping the bound)
/// `IInf k < expr, body` → `IInf (fun k -> body)` (dropping the bound)
/// Also handles `≤` bounds: `ISup k ≤ expr, body` → `ISup (fun k -> body)`
///
/// The bound is dropped for parsing purposes — we just want the signature to parse.
pub(super) fn normalize_bounded_quantifiers(src: &str) -> String {
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut result = String::with_capacity(src.len() + 32);
    let mut i = 0;
    while i < len {
        let is_isup = len >= i + 5
            && chars[i..i + 5].iter().collect::<String>() == "ISup "
            && (i == 0 || !chars[i - 1].is_alphanumeric());
        let is_iinf = len >= i + 5
            && chars[i..i + 5].iter().collect::<String>() == "IInf "
            && (i == 0 || !chars[i - 1].is_alphanumeric());
        if is_isup || is_iinf {
            let kw = if is_isup { "ISup" } else { "IInf" };
            let kw_len = 4;
            i += kw_len + 1;
            while i < len && chars[i] == ' ' {
                i += 1;
            }
            let ident_start = i;
            while i < len && (chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == '\'') {
                i += 1;
            }
            let ident: String = chars[ident_start..i].iter().collect();
            if ident.is_empty() {
                result.push_str(kw);
                result.push(' ');
                continue;
            }
            while i < len && chars[i] == ' ' {
                i += 1;
            }
            let has_lt = i < len && chars[i] == '<';
            let has_le = i < len && chars[i] == '\u{2264}';
            let has_mem = i + 3 <= len
                && chars[i..i + 3].iter().collect::<String>() == "Mem"
                && (i + 3 >= len || {
                    let next = chars[i + 3];
                    !next.is_alphanumeric() && next != '_'
                });
            if !has_lt && !has_le && !has_mem {
                result.push_str(kw);
                result.push(' ');
                result.push_str(&ident);
                continue;
            }
            if has_mem {
                i += 3;
            } else {
                i += 1;
            }
            while i < len && chars[i] == ' ' {
                i += 1;
            }
            let mut depth = 0usize;
            while i < len {
                match chars[i] {
                    '(' | '{' | '[' => {
                        depth += 1;
                        i += 1;
                    }
                    ')' | '}' | ']' if depth == 0 => break,
                    ')' | '}' | ']' => {
                        depth -= 1;
                        i += 1;
                    }
                    ',' if depth == 0 => break,
                    _ => {
                        i += 1;
                    }
                }
            }
            if i < len && chars[i] == ',' {
                i += 1;
            }
            while i < len && chars[i] == ' ' {
                i += 1;
            }
            result.push_str(kw);
            result.push_str(" (fun ");
            result.push_str(&ident);
            result.push_str(" -> ");
            let body_start = i;
            let mut depth = 0usize;
            let mut body_end = i;
            while i < len {
                match chars[i] {
                    '(' | '{' | '[' => {
                        depth += 1;
                        i += 1;
                        body_end = i;
                    }
                    ')' | '}' | ']' if depth == 0 => break,
                    ')' | '}' | ']' => {
                        depth -= 1;
                        i += 1;
                        body_end = i;
                    }
                    ':' if depth == 0 && i + 1 < len && chars[i + 1] == '=' => break,
                    _ => {
                        i += 1;
                        body_end = i;
                    }
                }
            }
            let body: String = chars[body_start..body_end].iter().collect();
            result.push_str(body.trim_end());
            result.push(')');
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}
/// Normalize bounded forall/exists quantifiers in type positions.
///
/// `∀ n < m, body` → `forall (n : _), body` (dropping the bound)
/// `∀ n ≤ m, body` → `forall (n : _), body` (dropping the bound)
/// `∃ n < m, body` → `Exists (fun (n : _) -> body)` (dropping the bound)
///
/// The bound is dropped since we only want the signature shape to parse.
/// Called AFTER unicode normalization has replaced ∀ → remains as `∀` until
/// parenthesize_bare_forall_binders runs.
pub(super) fn normalize_bounded_forall(src: &str) -> String {
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut result = String::with_capacity(src.len() + 32);
    let mut i = 0;
    while i < len {
        let prev_is_word = i > 0 && {
            let p = chars[i - 1];
            p.is_alphanumeric() || p == '_' || p == '\''
        };
        let is_forall_unicode = chars[i] == '\u{2200}' && !prev_is_word;
        let is_exists_unicode = chars[i] == '\u{2203}' && !prev_is_word;
        let is_forall_word = !prev_is_word
            && i + 6 <= len
            && chars[i..i + 6].iter().collect::<String>() == "forall"
            && (i + 6 >= len || {
                let next = chars[i + 6];
                !next.is_alphanumeric() && next != '_' && next != '\''
            });
        let is_exists_word = !prev_is_word
            && i + 6 <= len
            && chars[i..i + 6].iter().collect::<String>() == "exists"
            && (i + 6 >= len || {
                let next = chars[i + 6];
                !next.is_alphanumeric() && next != '_' && next != '\''
            });
        let is_exists_kw = is_exists_unicode || is_exists_word;
        if is_forall_unicode || is_forall_word || is_exists_kw {
            let kw_len = if is_forall_unicode || is_exists_unicode {
                1
            } else {
                6
            };
            let saved_i = i;
            i += kw_len;
            while i < len && chars[i] == ' ' {
                i += 1;
            }
            if i < len && chars[i] != '(' && chars[i] != '{' && chars[i] != '[' {
                let ident_start = i;
                while i < len && (chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == '\'')
                {
                    i += 1;
                }
                let ident: String = chars[ident_start..i].iter().collect();
                if ident.is_empty() {
                    i = saved_i;
                    result.push(chars[i]);
                    i += 1;
                    continue;
                }
                while i < len && chars[i] == ' ' {
                    i += 1;
                }
                let has_lt = i < len && chars[i] == '<' && (i + 1 >= len || chars[i + 1] != '=');
                let has_le = i < len && chars[i] == '\u{2264}';
                let has_ge = i + 2 <= len
                    && chars[i] == '>'
                    && chars[i + 1] == '='
                    && (i + 2 >= len || chars[i + 2] == ' ' || !chars[i + 2].is_alphanumeric());
                let has_gt = i < len && chars[i] == '>' && (i + 1 >= len || chars[i + 1] != '=');
                let has_mem = i + 3 <= len
                    && chars[i..i + 3].iter().collect::<String>() == "Mem"
                    && (i + 3 >= len || {
                        let next = chars[i + 3];
                        !next.is_alphanumeric() && next != '_'
                    });
                let has_ne = i + 2 <= len
                    && chars[i] == '!'
                    && chars[i + 1] == '='
                    && (i + 2 >= len || chars[i + 2] == ' ' || !chars[i + 2].is_alphanumeric());
                if has_lt || has_le || has_mem || has_ne || has_gt || has_ge {
                    let bound_op_len = if has_mem {
                        3
                    } else if has_ne || has_ge {
                        2
                    } else {
                        1
                    };
                    i += bound_op_len;
                    while i < len && chars[i] == ' ' {
                        i += 1;
                    }
                    let mut depth = 0usize;
                    while i < len {
                        match chars[i] {
                            '(' | '{' | '[' => {
                                depth += 1;
                                i += 1;
                            }
                            ')' | '}' | ']' if depth == 0 => break,
                            ')' | '}' | ']' => {
                                depth = depth.saturating_sub(1);
                                i += 1;
                            }
                            ',' if depth == 0 => break,
                            _ => {
                                i += 1;
                            }
                        }
                    }
                    if i < len && chars[i] == ',' {
                        i += 1;
                    }
                    while i < len && chars[i] == ' ' {
                        i += 1;
                    }
                    let kw: String = chars[saved_i..saved_i + kw_len].iter().collect();
                    result.push_str(&kw);
                    result.push_str(" (");
                    result.push_str(&ident);
                    result.push_str(" : _), ");
                } else {
                    let kw: String = chars[saved_i..saved_i + kw_len].iter().collect();
                    result.push_str(&kw);
                    result.push(' ');
                    result.push_str(&ident);
                }
            } else {
                let kw: String = chars[saved_i..saved_i + kw_len].iter().collect();
                result.push_str(&kw);
                result.push(' ');
            }
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}
/// Normalize Lean 4 method name suffixes `!` and `?` in identifiers.
///
/// In Lean 4, `head!` means "head, panics on empty" and `head?` means "Option-returning head".
/// OxiLean's lexer tokenizes `!` as prefix NOT and `?` as a hole/wildcard.
/// So `head! l` would parse as `head (! l)` (wrong) and `head? l` would fail.
/// We normalize: `ident!` → `ident_bang`, `ident?` → `ident_opt`.
pub(super) fn normalize_lean_method_names(src: &str) -> String {
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut result = String::with_capacity(src.len() + 8);
    let mut i = 0;
    while i < len {
        if chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == '\'' {
            let ident_start = i;
            while i < len && (chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == '\'') {
                i += 1;
            }
            let ident: String = chars[ident_start..i].iter().collect();
            if i < len && chars[i] == '!' && (i + 1 >= len || chars[i + 1] != '=') {
                result.push_str(&ident);
                result.push_str("_bang");
                i += 1;
            } else if i < len && chars[i] == '?' {
                result.push_str(&ident);
                result.push_str("_opt");
                i += 1;
            } else {
                result.push_str(&ident);
            }
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}
/// Strip inline `(by tactic)` proof obligations in term/type positions → `sorry`.
///
/// In some Lean 4 theorems, `(by tactic)` appears as a function argument
/// in the type signature (not just the proof body). For example:
/// `getLast (concat l a) (by simp) = a`
/// After `replace_proof_with_sorry` the main `:= proof` is handled, but
/// these inline `(by ...)` arguments remain in the type part.
pub(super) fn normalize_inline_by(src: &str) -> String {
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut result = String::with_capacity(src.len());
    let mut i = 0;
    while i < len {
        if chars[i] == '(' && i + 4 <= len && chars[i..i + 4].iter().collect::<String>() == "(by " {
            let mut depth = 1usize;
            let mut j = i + 1;
            while j < len && depth > 0 {
                match chars[j] {
                    '(' | '\u{27E8}' => {
                        depth += 1;
                        j += 1;
                    }
                    ')' | '\u{27E9}' => {
                        depth = depth.saturating_sub(1);
                        j += 1;
                    }
                    _ => {
                        j += 1;
                    }
                }
            }
            result.push_str("sorry");
            i = j;
        } else if i + 5 <= len && chars[i..i + 5].iter().collect::<String>() == ", by " {
            let mut depth = 0usize;
            let mut j = i + 5;
            while j < len {
                match chars[j] {
                    '(' | '\u{27E8}' | '[' => {
                        depth += 1;
                        j += 1;
                    }
                    ')' | '\u{27E9}' | ']' if depth == 0 => break,
                    ')' | '\u{27E9}' | ']' => {
                        depth = depth.saturating_sub(1);
                        j += 1;
                    }
                    ',' if depth == 0 => break,
                    _ => {
                        j += 1;
                    }
                }
            }
            result.push_str(", sorry");
            i = j;
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}
/// Normalize `{ x : T Subtype P }` (Lean 4 subtype `{ x : T // P }` after `//` → `Subtype`).
///
/// After step 4 replaces `//` with `Subtype`, the expression `{b // b ≠ a}` becomes
/// `{b Subtype b != a}`. OxiLean's parser would parse `{b ...}` as an implicit binder
/// `{b}` then fail on the remaining `Subtype b != a }`.
///
/// This function converts `{ ident (: type)? Subtype pred }` → `(SubtypeOf pred)`.
/// It also handles `CardSet { x : T Subtype pred }` → `(CardinalSize pred)`.
pub(super) fn normalize_subtype_braces(src: &str) -> String {
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut result = String::with_capacity(src.len());
    let mut i = 0;
    while i < len {
        if chars[i] == '{' {
            let mut j = i + 1;
            let mut depth = 1usize;
            let mut subtype_pos: Option<usize> = None;
            let mut close_brace: Option<usize> = None;
            while j < len && depth > 0 {
                match chars[j] {
                    '{' => {
                        depth += 1;
                        j += 1;
                    }
                    '}' => {
                        depth -= 1;
                        if depth == 0 {
                            close_brace = Some(j);
                        } else {
                            j += 1;
                        }
                    }
                    'S' if depth == 1 && j + 7 <= len => {
                        let word: String = chars[j..j + 7].iter().collect();
                        if word == "Subtype"
                            && (j == 0 || !chars[j - 1].is_alphanumeric())
                            && (j + 7 >= len || !chars[j + 7].is_alphanumeric())
                        {
                            subtype_pos = Some(j);
                        }
                        j += 1;
                    }
                    _ => {
                        j += 1;
                    }
                }
            }
            if let (Some(sub_pos), Some(close_pos)) = (subtype_pos, close_brace) {
                let pred_start = sub_pos + 7;
                let pred_str: String = chars[pred_start..close_pos].iter().collect();
                let pred_str = pred_str.trim();
                let result_trimmed = result.trim_end();
                if result_trimmed.ends_with("CardSet") {
                    let trim_len = result.len() - result_trimmed.len() + "CardSet".len();
                    result.truncate(result.len() - trim_len);
                    result.push_str("(CardinalSize ");
                    result.push_str(pred_str);
                    result.push(')');
                } else {
                    result.push_str("(SubtypeOf ");
                    result.push_str(pred_str);
                    result.push(')');
                }
                i = close_pos + 1;
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
/// Strip default values from binder type annotations.
///
/// Lean 4 allows default parameter values: `(h : n = n := rfl)`
/// OxiLean does not support default values. Strip `:= <default>` inside binder groups.
/// `(h : n = n := rfl)` → `(h : n = n)`
/// This runs on the FULL string, stripping `:=` and its following value inside `(...)`.
pub(super) fn normalize_default_binder_values(src: &str) -> String {
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut result = String::with_capacity(src.len());
    let mut i = 0;
    while i < len {
        if chars[i] == '(' {
            let paren_start = i;
            i += 1;
            let inner_start = i;
            let mut depth = 1usize;
            let mut assign_pos: Option<usize> = None;
            let mut inner_chars = Vec::new();
            let mut j = i;
            while j < len {
                match chars[j] {
                    '(' | '{' | '[' => {
                        depth += 1;
                        inner_chars.push(chars[j]);
                        j += 1;
                    }
                    ')' | '}' | ']' => {
                        depth -= 1;
                        if depth == 0 {
                            break;
                        }
                        inner_chars.push(chars[j]);
                        j += 1;
                    }
                    ':' if depth == 1 && j + 1 < len && chars[j + 1] == '=' => {
                        if assign_pos.is_none() {
                            assign_pos = Some(inner_chars.len());
                        }
                        inner_chars.push(':');
                        inner_chars.push('=');
                        j += 2;
                    }
                    _ => {
                        inner_chars.push(chars[j]);
                        j += 1;
                    }
                }
            }
            if j < len {
                i = j + 1;
            } else {
                result.push('(');
                let inner_str: String = inner_chars.iter().collect();
                result.push_str(&inner_str);
                continue;
            }
            let inner_str: String = inner_chars.iter().collect();
            if let Some(assign_idx) = assign_pos {
                let byte_offset = inner_str
                    .char_indices()
                    .nth(assign_idx)
                    .map(|(b, _)| b)
                    .unwrap_or(inner_str.len());
                let before_assign = &inner_str[..byte_offset];
                if before_assign.contains(':') && !before_assign.contains(":=") {
                    result.push('(');
                    result.push_str(before_assign.trim_end());
                    result.push(')');
                } else {
                    result.push('(');
                    result.push_str(&inner_str);
                    result.push(')');
                }
            } else {
                result.push('(');
                result.push_str(&inner_str);
                result.push(')');
            }
            let _ = (paren_start, inner_start);
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}
/// Parenthesize bare typed binders in lambda expressions.
///
/// Lean 4 allows: `fun a : T => body` (bare binder with type annotation)
/// OxiLean requires: `fun (a : T) -> body` (binder must be in parentheses)
///
/// `fun a : T -> body` → `fun (a : T) -> body`
/// `fun a b : T -> body` → `fun (a b : T) -> body`  (multi-name binders)
/// Only transforms when the binder has `: type` annotation; skips `fun a -> body`.
pub(super) fn normalize_fun_bare_binders(src: &str) -> String {
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut result = String::with_capacity(src.len() + 32);
    let mut i = 0;
    while i < len {
        let prev_is_word = i > 0 && {
            let p = chars[i - 1];
            p.is_alphanumeric() || p == '_' || p == '\''
        };
        let is_fun = !prev_is_word
            && i + 4 <= len
            && chars[i..i + 4].iter().collect::<String>() == "fun "
            && (i + 4 >= len || chars[i + 4] != '(');
        if is_fun {
            let fun_kw_end = i + 3;
            let saved_i = i;
            i += 4;
            while i < len && chars[i] == ' ' {
                i += 1;
            }
            if i >= len || chars[i] == '(' || chars[i] == '{' || chars[i] == '[' {
                let kw: String = chars[saved_i..fun_kw_end + 1].iter().collect();
                result.push_str(&kw);
                continue;
            }
            let binders_start = i;
            let mut names_end = i;
            let mut colon_pos = None;
            let mut j = i;
            while j < len {
                if chars[j] == ':'
                    && (j + 1 >= len || chars[j + 1] != '=')
                    && (j + 1 >= len || chars[j + 1] != ':')
                {
                    let prefix: String = chars[binders_start..j].iter().collect();
                    let all_ident_or_space = prefix
                        .trim()
                        .chars()
                        .all(|c| c.is_alphanumeric() || c == '_' || c == '\'' || c == ' ');
                    if all_ident_or_space && !prefix.trim().is_empty() {
                        names_end = j;
                        colon_pos = Some(j);
                        break;
                    } else {
                        break;
                    }
                }
                if chars[j] == '-' && j + 1 < len && chars[j + 1] == '>' {
                    break;
                }
                if chars[j] == '(' || chars[j] == '{' || chars[j] == '[' {
                    break;
                }
                j += 1;
            }
            if colon_pos.is_none() {
                let kw: String = chars[saved_i..fun_kw_end + 1].iter().collect();
                result.push_str(&kw);
                continue;
            }
            let colon_idx = colon_pos.unwrap();
            let names_str: String = chars[binders_start..names_end].iter().collect();
            let names_trimmed = names_str.trim();
            i = colon_idx + 1;
            while i < len && chars[i] == ' ' {
                i += 1;
            }
            let type_start = i;
            let mut depth = 0usize;
            let mut arrow_pos = None;
            while i < len {
                match chars[i] {
                    '(' | '{' | '[' | '\u{27E8}' => {
                        depth += 1;
                        i += 1;
                    }
                    ')' | '}' | ']' | '\u{27E9}' if depth == 0 => break,
                    ')' | '}' | ']' | '\u{27E9}' => {
                        depth = depth.saturating_sub(1);
                        i += 1;
                    }
                    '-' if depth == 0 && i + 1 < len && chars[i + 1] == '>' => {
                        arrow_pos = Some(i);
                        break;
                    }
                    _ => {
                        i += 1;
                    }
                }
            }
            if arrow_pos.is_none() {
                i = saved_i;
                result.push(chars[i]);
                i += 1;
                continue;
            }
            let type_str: String = chars[type_start..arrow_pos.unwrap()].iter().collect();
            let type_trimmed = type_str.trim_end();
            if type_trimmed.is_empty() {
                i = saved_i;
                result.push(chars[i]);
                i += 1;
                continue;
            }
            result.push_str("fun (");
            result.push_str(names_trimmed);
            result.push_str(" : ");
            result.push_str(type_trimmed);
            result.push_str(") ");
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}
