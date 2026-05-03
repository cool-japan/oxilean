//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    CharCategory, CharInfo, CharPredicateTable, CharRange, CharScanner, UnicodeBlocks,
};

/// Build Char type in the environment.
pub fn build_char_env(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    env.add(Declaration::Axiom {
        name: Name::str("Char"),
        univ_params: vec![],
        ty: type1.clone(),
    })
    .map_err(|e| e.to_string())?;
    let of_nat_ty = Expr::Pi(
        oxilean_kernel::BinderInfo::Default,
        Name::str("n"),
        Box::new(Expr::Const(Name::str("Nat"), vec![])),
        Box::new(Expr::Const(Name::str("Char"), vec![])),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Char.ofNat"),
        univ_params: vec![],
        ty: of_nat_ty,
    })
    .map_err(|e| e.to_string())?;
    let to_nat_ty = Expr::Pi(
        oxilean_kernel::BinderInfo::Default,
        Name::str("c"),
        Box::new(Expr::Const(Name::str("Char"), vec![])),
        Box::new(Expr::Const(Name::str("Nat"), vec![])),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Char.toNat"),
        univ_params: vec![],
        ty: to_nat_ty,
    })
    .map_err(|e| e.to_string())?;
    let is_alpha_ty = Expr::Pi(
        oxilean_kernel::BinderInfo::Default,
        Name::str("c"),
        Box::new(Expr::Const(Name::str("Char"), vec![])),
        Box::new(Expr::Const(Name::str("Bool"), vec![])),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Char.isAlpha"),
        univ_params: vec![],
        ty: is_alpha_ty,
    })
    .map_err(|e| e.to_string())?;
    let is_digit_ty = Expr::Pi(
        oxilean_kernel::BinderInfo::Default,
        Name::str("c"),
        Box::new(Expr::Const(Name::str("Char"), vec![])),
        Box::new(Expr::Const(Name::str("Bool"), vec![])),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Char.isDigit"),
        univ_params: vec![],
        ty: is_digit_ty,
    })
    .map_err(|e| e.to_string())?;
    build_char_predicates(env)?;
    build_char_conversions(env)?;
    build_char_comparisons(env)?;
    Ok(())
}
/// Build character predicate axioms (isUpper, isLower, isAlphaNum, etc.)
pub fn build_char_predicates(env: &mut Environment) -> Result<(), String> {
    let char_to_bool = Expr::Pi(
        BinderInfo::Default,
        Name::str("c"),
        Box::new(Expr::Const(Name::str("Char"), vec![])),
        Box::new(Expr::Const(Name::str("Bool"), vec![])),
    );
    let predicates = [
        "Char.isUpper",
        "Char.isLower",
        "Char.isAlphaNum",
        "Char.isWhitespace",
        "Char.isPunctuation",
        "Char.isAscii",
        "Char.isControl",
        "Char.isPrint",
        "Char.isHexDigit",
        "Char.isOctDigit",
    ];
    for pred_name in predicates {
        env.add(Declaration::Axiom {
            name: Name::str(pred_name),
            univ_params: vec![],
            ty: char_to_bool.clone(),
        })
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}
/// Build character conversion axioms (toUpper, toLower, digitToNat, etc.)
pub fn build_char_conversions(env: &mut Environment) -> Result<(), String> {
    let char_to_char = Expr::Pi(
        BinderInfo::Default,
        Name::str("c"),
        Box::new(Expr::Const(Name::str("Char"), vec![])),
        Box::new(Expr::Const(Name::str("Char"), vec![])),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Char.toUpper"),
        univ_params: vec![],
        ty: char_to_char.clone(),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("Char.toLower"),
        univ_params: vec![],
        ty: char_to_char.clone(),
    })
    .map_err(|e| e.to_string())?;
    let char_to_nat = Expr::Pi(
        BinderInfo::Default,
        Name::str("c"),
        Box::new(Expr::Const(Name::str("Char"), vec![])),
        Box::new(Expr::Const(Name::str("Nat"), vec![])),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Char.digitToNat"),
        univ_params: vec![],
        ty: char_to_nat.clone(),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("Char.hexDigitToNat"),
        univ_params: vec![],
        ty: char_to_nat.clone(),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("Char.utf8Size"),
        univ_params: vec![],
        ty: char_to_nat,
    })
    .map_err(|e| e.to_string())?;
    Ok(())
}
/// Build character comparison axioms (beq, lt, le, etc.)
pub fn build_char_comparisons(env: &mut Environment) -> Result<(), String> {
    let char_char_to_bool = Expr::Pi(
        BinderInfo::Default,
        Name::str("a"),
        Box::new(Expr::Const(Name::str("Char"), vec![])),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("b"),
            Box::new(Expr::Const(Name::str("Char"), vec![])),
            Box::new(Expr::Const(Name::str("Bool"), vec![])),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Char.beq"),
        univ_params: vec![],
        ty: char_char_to_bool.clone(),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("Char.blt"),
        univ_params: vec![],
        ty: char_char_to_bool.clone(),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("Char.ble"),
        univ_params: vec![],
        ty: char_char_to_bool,
    })
    .map_err(|e| e.to_string())?;
    Ok(())
}
/// Return the Unicode general category for a Rust `char`.
pub fn unicode_category(c: char) -> CharCategory {
    if c.is_uppercase() {
        CharCategory::UppercaseLetter
    } else if c.is_lowercase() {
        CharCategory::LowercaseLetter
    } else if c.is_ascii_digit() {
        CharCategory::DecimalNumber
    } else if c.is_ascii_whitespace() || c == '\u{00A0}' {
        CharCategory::SpaceSeparator
    } else if c.is_control() {
        CharCategory::Control
    } else if c.is_ascii_punctuation() {
        CharCategory::ConnectorPunctuation
    } else if c == '£' || c == '$' || c == '€' || c == '¥' {
        CharCategory::CurrencySymbol
    } else if c == '+' || c == '-' || c == '=' || c == '<' || c == '>' {
        CharCategory::MathSymbol
    } else {
        CharCategory::Unknown
    }
}
/// Return true if `c` is an ASCII alphabetic character.
pub fn is_ascii_alpha(c: char) -> bool {
    c.is_ascii_alphabetic()
}
/// Return true if `c` is an ASCII alphanumeric character.
pub fn is_ascii_alnum(c: char) -> bool {
    c.is_ascii_alphanumeric()
}
/// Return true if `c` is a valid Unicode scalar value (all Rust `char` are).
pub fn is_valid_unicode(c: char) -> bool {
    let cp = c as u32;
    cp <= 0x10FFFF
}
/// Return true if `c` is a ASCII hexadecimal digit.
pub fn is_hex_digit(c: char) -> bool {
    c.is_ascii_hexdigit()
}
/// Return true if `c` is a printable ASCII character.
pub fn is_printable_ascii(c: char) -> bool {
    c.is_ascii() && !c.is_control()
}
/// Return true if `c` is a Unicode letter (alphabetic in the broad sense).
pub fn is_unicode_letter(c: char) -> bool {
    c.is_alphabetic()
}
/// Return the number of UTF-8 bytes required to encode `c`.
pub fn utf8_encoded_len(c: char) -> usize {
    c.len_utf8()
}
/// Return the number of UTF-16 code units required to encode `c`.
pub fn utf16_encoded_len(c: char) -> usize {
    c.len_utf16()
}
/// Encode `c` as a UTF-8 byte array (up to 4 bytes). Returns (bytes, len).
pub fn utf8_encode(c: char) -> ([u8; 4], usize) {
    let mut buf = [0u8; 4];
    let len = c.encode_utf8(&mut buf).len();
    (buf, len)
}
/// Encode `c` as UTF-16 code units (up to 2 u16s). Returns (units, len).
pub fn utf16_encode(c: char) -> ([u16; 2], usize) {
    let mut buf = [0u16; 2];
    let len = c.encode_utf16(&mut buf).len();
    (buf, len)
}
/// Try to decode a single UTF-8 character from the byte slice.
/// Returns `Some((char, bytes_consumed))` or `None` on invalid input.
pub fn utf8_decode_first(bytes: &[u8]) -> Option<(char, usize)> {
    if bytes.is_empty() {
        return None;
    }
    let s = std::str::from_utf8(bytes).ok()?;
    let c = s.chars().next()?;
    Some((c, c.len_utf8()))
}
/// Convert an ASCII digit character to its numeric value (0–9).
/// Returns `None` if `c` is not an ASCII digit.
pub fn ascii_digit_value(c: char) -> Option<u32> {
    if c.is_ascii_digit() {
        Some(c as u32 - b'0' as u32)
    } else {
        None
    }
}
/// Convert an ASCII hex digit to its numeric value (0–15).
/// Returns `None` if `c` is not a hex digit.
pub fn hex_digit_value(c: char) -> Option<u32> {
    match c {
        '0'..='9' => Some(c as u32 - b'0' as u32),
        'a'..='f' => Some(c as u32 - b'a' as u32 + 10),
        'A'..='F' => Some(c as u32 - b'A' as u32 + 10),
        _ => None,
    }
}
/// Return `Some(char)` if the Unicode code point `cp` is a valid scalar value.
pub fn from_code_point(cp: u32) -> Option<char> {
    char::from_u32(cp)
}
/// Return the Unicode code point of `c` as a `u32`.
pub fn to_code_point(c: char) -> u32 {
    c as u32
}
/// Convert a character to title case (single char; Rust approximation).
pub fn to_titlecase_first(c: char) -> char {
    c.to_uppercase().next().unwrap_or(c)
}
/// Fold a character for case-insensitive comparison (to lowercase).
pub fn case_fold(c: char) -> char {
    c.to_lowercase().next().unwrap_or(c)
}
/// Return all characters in a Latin alphabet range \[a, z\] or \[A, Z\].
pub fn latin_alphabet(uppercase: bool) -> Vec<char> {
    if uppercase {
        ('A'..='Z').collect()
    } else {
        ('a'..='z').collect()
    }
}
/// Return all ASCII digit characters ['0'..='9'].
pub fn ascii_digits() -> Vec<char> {
    ('0'..='9').collect()
}
/// Return true if `c` is an identifier-start character (letter or underscore).
pub fn is_ident_start(c: char) -> bool {
    c == '_' || c.is_alphabetic()
}
/// Return true if `c` is an identifier-continue character.
pub fn is_ident_continue(c: char) -> bool {
    c == '_' || c.is_alphanumeric() || c == '\''
}
/// Return true if the char is a valid OxiLean name character.
pub fn is_oxilean_name_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '.' || c == '\''
}
/// Escape a character for display in OxiLean syntax (e.g., '\n' → "\\n").
pub fn escape_char(c: char) -> String {
    match c {
        '\n' => "\\n".to_string(),
        '\t' => "\\t".to_string(),
        '\r' => "\\r".to_string(),
        '\\' => "\\\\".to_string(),
        '\'' => "\\'".to_string(),
        '"' => "\\\"".to_string(),
        '\0' => "\\0".to_string(),
        c if c.is_control() => format!("\\u{{{:04X}}}", c as u32),
        c => c.to_string(),
    }
}
/// Unescape a single-character escape sequence.
/// Input: the char after the backslash. Returns the decoded char or None.
pub fn unescape_char(c: char) -> Option<char> {
    match c {
        'n' => Some('\n'),
        't' => Some('\t'),
        'r' => Some('\r'),
        '\\' => Some('\\'),
        '\'' => Some('\''),
        '"' => Some('"'),
        '0' => Some('\0'),
        _ => None,
    }
}
/// Build an `Expr` that represents `Char.ofNat n_expr` application.
pub fn make_char_of_nat(n_expr: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::Const(Name::str("Char.ofNat"), vec![])),
        Box::new(n_expr),
    )
}
/// Build an `Expr` that represents `Char.toNat c_expr` application.
pub fn make_char_to_nat(c_expr: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::Const(Name::str("Char.toNat"), vec![])),
        Box::new(c_expr),
    )
}
/// Build an `Expr` for a character literal given a Unicode code point.
pub fn make_char_literal(code_point: u32) -> Expr {
    let nat_lit = Expr::Lit(oxilean_kernel::Literal::Nat(code_point.into()));
    make_char_of_nat(nat_lit)
}
/// Decode an OxiLean character escape sequence of the form `\uXXXX`.
pub fn decode_unicode_escape(hex: &str) -> Option<char> {
    let cp = u32::from_str_radix(hex.trim_start_matches('{').trim_end_matches('}'), 16).ok()?;
    char::from_u32(cp)
}
/// Summarize all registered char axiom names for a given environment.
pub fn registered_char_names(env: &Environment) -> Vec<String> {
    let candidates = [
        "Char",
        "Char.ofNat",
        "Char.toNat",
        "Char.isAlpha",
        "Char.isDigit",
        "Char.isUpper",
        "Char.isLower",
        "Char.isAlphaNum",
        "Char.isWhitespace",
        "Char.isPunctuation",
        "Char.isAscii",
        "Char.isControl",
        "Char.isPrint",
        "Char.isHexDigit",
        "Char.isOctDigit",
        "Char.toUpper",
        "Char.toLower",
        "Char.digitToNat",
        "Char.hexDigitToNat",
        "Char.utf8Size",
        "Char.beq",
        "Char.blt",
        "Char.ble",
    ];
    candidates
        .iter()
        .filter(|n| env.get(&Name::str(**n)).is_some())
        .map(|s| s.to_string())
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    fn setup_env() -> Environment {
        let mut env = Environment::new();
        let type1 = Expr::Sort(Level::succ(Level::zero()));
        env.add(Declaration::Axiom {
            name: Name::str("Nat"),
            univ_params: vec![],
            ty: type1.clone(),
        })
        .expect("operation should succeed");
        env.add(Declaration::Axiom {
            name: Name::str("Bool"),
            univ_params: vec![],
            ty: type1,
        })
        .expect("operation should succeed");
        env
    }
    #[test]
    fn test_build_char_env() {
        let mut env = setup_env();
        assert!(build_char_env(&mut env).is_ok());
        assert!(env.get(&Name::str("Char")).is_some());
        assert!(env.get(&Name::str("Char.ofNat")).is_some());
        assert!(env.get(&Name::str("Char.toNat")).is_some());
    }
    #[test]
    fn test_char_is_alpha() {
        let mut env = setup_env();
        build_char_env(&mut env).expect("build_char_env should succeed");
        let decl = env
            .get(&Name::str("Char.isAlpha"))
            .expect("declaration 'Char.isAlpha' should exist in env");
        assert!(matches!(decl, Declaration::Axiom { .. }));
    }
    #[test]
    fn test_char_is_digit() {
        let mut env = setup_env();
        build_char_env(&mut env).expect("build_char_env should succeed");
        let decl = env
            .get(&Name::str("Char.isDigit"))
            .expect("declaration 'Char.isDigit' should exist in env");
        assert!(matches!(decl, Declaration::Axiom { .. }));
    }
    #[test]
    fn test_char_predicates_registered() {
        let mut env = setup_env();
        build_char_env(&mut env).expect("build_char_env should succeed");
        assert!(env.get(&Name::str("Char.isUpper")).is_some());
        assert!(env.get(&Name::str("Char.isLower")).is_some());
        assert!(env.get(&Name::str("Char.isAlphaNum")).is_some());
        assert!(env.get(&Name::str("Char.isWhitespace")).is_some());
        assert!(env.get(&Name::str("Char.isAscii")).is_some());
    }
    #[test]
    fn test_char_conversions_registered() {
        let mut env = setup_env();
        build_char_env(&mut env).expect("build_char_env should succeed");
        assert!(env.get(&Name::str("Char.toUpper")).is_some());
        assert!(env.get(&Name::str("Char.toLower")).is_some());
        assert!(env.get(&Name::str("Char.digitToNat")).is_some());
        assert!(env.get(&Name::str("Char.hexDigitToNat")).is_some());
        assert!(env.get(&Name::str("Char.utf8Size")).is_some());
    }
    #[test]
    fn test_char_comparisons_registered() {
        let mut env = setup_env();
        build_char_env(&mut env).expect("build_char_env should succeed");
        assert!(env.get(&Name::str("Char.beq")).is_some());
        assert!(env.get(&Name::str("Char.blt")).is_some());
        assert!(env.get(&Name::str("Char.ble")).is_some());
    }
    #[test]
    fn test_unicode_category_letter() {
        assert_eq!(unicode_category('A'), CharCategory::UppercaseLetter);
        assert_eq!(unicode_category('z'), CharCategory::LowercaseLetter);
    }
    #[test]
    fn test_unicode_category_digit() {
        assert_eq!(unicode_category('5'), CharCategory::DecimalNumber);
    }
    #[test]
    fn test_unicode_category_whitespace() {
        assert_eq!(unicode_category(' '), CharCategory::SpaceSeparator);
    }
    #[test]
    fn test_utf8_encoded_len_ascii() {
        assert_eq!(utf8_encoded_len('A'), 1);
        assert_eq!(utf8_encoded_len('\n'), 1);
    }
    #[test]
    fn test_utf8_encoded_len_multibyte() {
        assert_eq!(utf8_encoded_len('€'), 3);
        assert_eq!(utf8_encoded_len('𝄞'), 4);
    }
    #[test]
    fn test_hex_digit_value() {
        assert_eq!(hex_digit_value('0'), Some(0));
        assert_eq!(hex_digit_value('9'), Some(9));
        assert_eq!(hex_digit_value('a'), Some(10));
        assert_eq!(hex_digit_value('F'), Some(15));
        assert_eq!(hex_digit_value('g'), None);
        assert_eq!(hex_digit_value('z'), None);
    }
    #[test]
    fn test_ascii_digit_value() {
        assert_eq!(ascii_digit_value('0'), Some(0));
        assert_eq!(ascii_digit_value('9'), Some(9));
        assert_eq!(ascii_digit_value('a'), None);
    }
    #[test]
    fn test_from_code_point() {
        assert_eq!(from_code_point(65), Some('A'));
        assert_eq!(from_code_point(0x1F600), Some('😀'));
        assert_eq!(from_code_point(0xD800), None);
    }
    #[test]
    fn test_to_code_point() {
        assert_eq!(to_code_point('A'), 65);
        assert_eq!(to_code_point('\0'), 0);
    }
    #[test]
    fn test_escape_char() {
        assert_eq!(escape_char('\n'), "\\n");
        assert_eq!(escape_char('\t'), "\\t");
        assert_eq!(escape_char('A'), "A");
        assert_eq!(escape_char('\\'), "\\\\");
    }
    #[test]
    fn test_unescape_char() {
        assert_eq!(unescape_char('n'), Some('\n'));
        assert_eq!(unescape_char('t'), Some('\t'));
        assert_eq!(unescape_char('z'), None);
    }
    #[test]
    fn test_char_info_new() {
        let info = CharInfo::new('A');
        assert_eq!(info.ch, 'A');
        assert_eq!(info.code_point, 65);
        assert_eq!(info.utf8_len, 1);
        assert!(info.is_ascii);
        assert!(info.is_letter());
        assert!(!info.is_digit());
    }
    #[test]
    fn test_char_info_digit() {
        let info = CharInfo::new('7');
        assert!(info.is_digit());
        assert!(!info.is_letter());
    }
    #[test]
    fn test_char_info_whitespace() {
        let info = CharInfo::new(' ');
        assert!(info.is_whitespace());
    }
    #[test]
    fn test_predicate_table_lookup() {
        let table = CharPredicateTable::new();
        let pred = table.lookup("isAlpha");
        assert!(pred.is_some());
        assert!(pred.expect("pred should be valid")('A'));
        assert!(!pred.expect("pred should be valid")('1'));
    }
    #[test]
    fn test_predicate_table_apply() {
        let table = CharPredicateTable::new();
        assert_eq!(table.apply("isDigit", '5'), Some(true));
        assert_eq!(table.apply("isDigit", 'a'), Some(false));
        assert_eq!(table.apply("nonexistent", 'a'), None);
    }
    #[test]
    fn test_predicate_table_names() {
        let table = CharPredicateTable::new();
        let names = table.names();
        assert!(names.contains(&"isAlpha"));
        assert!(names.contains(&"isDigit"));
        assert!(names.contains(&"isUpper"));
        assert!(names.len() >= 10);
    }
    #[test]
    fn test_is_ident_start() {
        assert!(is_ident_start('a'));
        assert!(is_ident_start('_'));
        assert!(is_ident_start('Z'));
        assert!(!is_ident_start('1'));
        assert!(!is_ident_start(' '));
    }
    #[test]
    fn test_is_ident_continue() {
        assert!(is_ident_continue('a'));
        assert!(is_ident_continue('_'));
        assert!(is_ident_continue('1'));
        assert!(is_ident_continue('\''));
        assert!(!is_ident_continue(' '));
        assert!(!is_ident_continue('.'));
    }
    #[test]
    fn test_latin_alphabet() {
        let upper = latin_alphabet(true);
        assert_eq!(upper.len(), 26);
        assert_eq!(upper[0], 'A');
        assert_eq!(upper[25], 'Z');
        let lower = latin_alphabet(false);
        assert_eq!(lower.len(), 26);
        assert_eq!(lower[0], 'a');
    }
    #[test]
    fn test_ascii_digits_vec() {
        let digits = ascii_digits();
        assert_eq!(digits.len(), 10);
        assert_eq!(digits[0], '0');
        assert_eq!(digits[9], '9');
    }
    #[test]
    fn test_utf8_decode_first_ascii() {
        let bytes = b"Hello";
        let result = utf8_decode_first(bytes);
        assert_eq!(result, Some(('H', 1)));
    }
    #[test]
    fn test_utf8_decode_first_multibyte() {
        let c = '€';
        let mut buf = [0u8; 4];
        let s = c.encode_utf8(&mut buf);
        let bytes = s.as_bytes();
        let result = utf8_decode_first(bytes);
        assert_eq!(result, Some(('€', 3)));
    }
    #[test]
    fn test_decode_unicode_escape() {
        assert_eq!(decode_unicode_escape("0041"), Some('A'));
        assert_eq!(decode_unicode_escape("{0041}"), Some('A'));
        assert_eq!(decode_unicode_escape("1F600"), Some('😀'));
    }
    #[test]
    fn test_registered_char_names() {
        let mut env = setup_env();
        build_char_env(&mut env).expect("build_char_env should succeed");
        let names = registered_char_names(&env);
        assert!(names.contains(&"Char".to_string()));
        assert!(names.contains(&"Char.ofNat".to_string()));
        assert!(names.len() >= 5);
    }
    #[test]
    fn test_make_char_literal() {
        let expr = make_char_literal(65);
        assert!(matches!(expr, Expr::App(_, _)));
    }
    #[test]
    fn test_is_valid_unicode() {
        assert!(is_valid_unicode('A'));
        assert!(is_valid_unicode('😀'));
        assert!(is_valid_unicode('\0'));
    }
    #[test]
    fn test_case_fold() {
        assert_eq!(case_fold('A'), 'a');
        assert_eq!(case_fold('z'), 'z');
        assert_eq!(case_fold('5'), '5');
    }
}
/// Normalize a string by converting all Unicode whitespace to ASCII space.
#[allow(dead_code)]
pub fn normalize_whitespace(s: &str) -> String {
    s.chars()
        .map(|c| if c.is_whitespace() { ' ' } else { c })
        .collect()
}
/// Remove non-printable control characters from a string.
#[allow(dead_code)]
pub fn strip_control_chars(s: &str) -> String {
    s.chars().filter(|c| !c.is_control()).collect()
}
/// Normalize Unicode to NFC-like representation (common composed forms).
///
/// This is a heuristic approximation of NFC normalization that handles
/// the most common Latin characters with diacritics in decomposed form
/// (base letter + combining diacritic) and maps them to the corresponding
/// precomposed Unicode characters.  It is idempotent on already-composed text.
#[allow(dead_code)]
pub fn normalize_to_nfc_approx(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut result = String::with_capacity(s.len());
    let mut i = 0;
    while i < chars.len() {
        let base = chars[i];
        if i + 1 < chars.len() {
            if let Some(composed) = compose_pair(base, chars[i + 1]) {
                result.push(composed);
                i += 2;
                continue;
            }
        }
        result.push(base);
        i += 1;
    }
    result
}
/// Attempt to compose a (base, combining diacritic) pair into a precomposed char.
///
/// Returns `Some(c)` for common Latin + diacritic combinations, `None` otherwise.
#[allow(dead_code)]
pub fn compose_pair(base: char, combining: char) -> Option<char> {
    match (base, combining) {
        ('A', '\u{0300}') => Some('À'),
        ('E', '\u{0300}') => Some('È'),
        ('I', '\u{0300}') => Some('Ì'),
        ('O', '\u{0300}') => Some('Ò'),
        ('U', '\u{0300}') => Some('Ù'),
        ('W', '\u{0300}') => Some('Ẁ'),
        ('Y', '\u{0300}') => Some('Ỳ'),
        ('N', '\u{0300}') => Some('Ǹ'),
        ('a', '\u{0300}') => Some('à'),
        ('e', '\u{0300}') => Some('è'),
        ('i', '\u{0300}') => Some('ì'),
        ('o', '\u{0300}') => Some('ò'),
        ('u', '\u{0300}') => Some('ù'),
        ('w', '\u{0300}') => Some('ẁ'),
        ('y', '\u{0300}') => Some('ỳ'),
        ('n', '\u{0300}') => Some('ǹ'),
        ('A', '\u{0301}') => Some('Á'),
        ('C', '\u{0301}') => Some('Ć'),
        ('E', '\u{0301}') => Some('É'),
        ('G', '\u{0301}') => Some('Ǵ'),
        ('I', '\u{0301}') => Some('Í'),
        ('K', '\u{0301}') => Some('Ḱ'),
        ('L', '\u{0301}') => Some('Ĺ'),
        ('M', '\u{0301}') => Some('Ḿ'),
        ('N', '\u{0301}') => Some('Ń'),
        ('O', '\u{0301}') => Some('Ó'),
        ('P', '\u{0301}') => Some('Ṕ'),
        ('R', '\u{0301}') => Some('Ŕ'),
        ('S', '\u{0301}') => Some('Ś'),
        ('U', '\u{0301}') => Some('Ú'),
        ('W', '\u{0301}') => Some('Ẃ'),
        ('Y', '\u{0301}') => Some('Ý'),
        ('Z', '\u{0301}') => Some('Ź'),
        ('a', '\u{0301}') => Some('á'),
        ('c', '\u{0301}') => Some('ć'),
        ('e', '\u{0301}') => Some('é'),
        ('g', '\u{0301}') => Some('ǵ'),
        ('i', '\u{0301}') => Some('í'),
        ('k', '\u{0301}') => Some('ḱ'),
        ('l', '\u{0301}') => Some('ĺ'),
        ('m', '\u{0301}') => Some('ḿ'),
        ('n', '\u{0301}') => Some('ń'),
        ('o', '\u{0301}') => Some('ó'),
        ('p', '\u{0301}') => Some('ṕ'),
        ('r', '\u{0301}') => Some('ŕ'),
        ('s', '\u{0301}') => Some('ś'),
        ('u', '\u{0301}') => Some('ú'),
        ('w', '\u{0301}') => Some('ẃ'),
        ('y', '\u{0301}') => Some('ý'),
        ('z', '\u{0301}') => Some('ź'),
        ('A', '\u{0302}') => Some('Â'),
        ('C', '\u{0302}') => Some('Ĉ'),
        ('E', '\u{0302}') => Some('Ê'),
        ('G', '\u{0302}') => Some('Ĝ'),
        ('H', '\u{0302}') => Some('Ĥ'),
        ('I', '\u{0302}') => Some('Î'),
        ('J', '\u{0302}') => Some('Ĵ'),
        ('O', '\u{0302}') => Some('Ô'),
        ('S', '\u{0302}') => Some('Ŝ'),
        ('U', '\u{0302}') => Some('Û'),
        ('W', '\u{0302}') => Some('Ŵ'),
        ('Y', '\u{0302}') => Some('Ŷ'),
        ('Z', '\u{0302}') => Some('Ẑ'),
        ('a', '\u{0302}') => Some('â'),
        ('c', '\u{0302}') => Some('ĉ'),
        ('e', '\u{0302}') => Some('ê'),
        ('g', '\u{0302}') => Some('ĝ'),
        ('h', '\u{0302}') => Some('ĥ'),
        ('i', '\u{0302}') => Some('î'),
        ('j', '\u{0302}') => Some('ĵ'),
        ('o', '\u{0302}') => Some('ô'),
        ('s', '\u{0302}') => Some('ŝ'),
        ('u', '\u{0302}') => Some('û'),
        ('w', '\u{0302}') => Some('ŵ'),
        ('y', '\u{0302}') => Some('ŷ'),
        ('z', '\u{0302}') => Some('ẑ'),
        ('A', '\u{0303}') => Some('Ã'),
        ('E', '\u{0303}') => Some('Ẽ'),
        ('I', '\u{0303}') => Some('Ĩ'),
        ('N', '\u{0303}') => Some('Ñ'),
        ('O', '\u{0303}') => Some('Õ'),
        ('U', '\u{0303}') => Some('Ũ'),
        ('V', '\u{0303}') => Some('Ṽ'),
        ('Y', '\u{0303}') => Some('Ỹ'),
        ('a', '\u{0303}') => Some('ã'),
        ('e', '\u{0303}') => Some('ẽ'),
        ('i', '\u{0303}') => Some('ĩ'),
        ('n', '\u{0303}') => Some('ñ'),
        ('o', '\u{0303}') => Some('õ'),
        ('u', '\u{0303}') => Some('ũ'),
        ('v', '\u{0303}') => Some('ṽ'),
        ('y', '\u{0303}') => Some('ỹ'),
        ('A', '\u{0304}') => Some('Ā'),
        ('E', '\u{0304}') => Some('Ē'),
        ('I', '\u{0304}') => Some('Ī'),
        ('O', '\u{0304}') => Some('Ō'),
        ('U', '\u{0304}') => Some('Ū'),
        ('G', '\u{0304}') => Some('Ḡ'),
        ('Y', '\u{0304}') => Some('Ȳ'),
        ('a', '\u{0304}') => Some('ā'),
        ('e', '\u{0304}') => Some('ē'),
        ('i', '\u{0304}') => Some('ī'),
        ('o', '\u{0304}') => Some('ō'),
        ('u', '\u{0304}') => Some('ū'),
        ('g', '\u{0304}') => Some('ḡ'),
        ('y', '\u{0304}') => Some('ȳ'),
        ('A', '\u{0306}') => Some('Ă'),
        ('E', '\u{0306}') => Some('Ĕ'),
        ('G', '\u{0306}') => Some('Ğ'),
        ('I', '\u{0306}') => Some('Ĭ'),
        ('O', '\u{0306}') => Some('Ŏ'),
        ('U', '\u{0306}') => Some('Ŭ'),
        ('a', '\u{0306}') => Some('ă'),
        ('e', '\u{0306}') => Some('ĕ'),
        ('g', '\u{0306}') => Some('ğ'),
        ('i', '\u{0306}') => Some('ĭ'),
        ('o', '\u{0306}') => Some('ŏ'),
        ('u', '\u{0306}') => Some('ŭ'),
        ('B', '\u{0307}') => Some('Ḃ'),
        ('C', '\u{0307}') => Some('Ċ'),
        ('D', '\u{0307}') => Some('Ḋ'),
        ('E', '\u{0307}') => Some('Ė'),
        ('F', '\u{0307}') => Some('Ḟ'),
        ('G', '\u{0307}') => Some('Ġ'),
        ('H', '\u{0307}') => Some('Ḣ'),
        ('I', '\u{0307}') => Some('İ'),
        ('M', '\u{0307}') => Some('Ṁ'),
        ('N', '\u{0307}') => Some('Ṅ'),
        ('P', '\u{0307}') => Some('Ṗ'),
        ('R', '\u{0307}') => Some('Ṙ'),
        ('S', '\u{0307}') => Some('Ṡ'),
        ('T', '\u{0307}') => Some('Ṫ'),
        ('W', '\u{0307}') => Some('Ẇ'),
        ('X', '\u{0307}') => Some('Ẋ'),
        ('Y', '\u{0307}') => Some('Ẏ'),
        ('Z', '\u{0307}') => Some('Ż'),
        ('b', '\u{0307}') => Some('ḃ'),
        ('c', '\u{0307}') => Some('ċ'),
        ('d', '\u{0307}') => Some('ḋ'),
        ('e', '\u{0307}') => Some('ė'),
        ('f', '\u{0307}') => Some('ḟ'),
        ('g', '\u{0307}') => Some('ġ'),
        ('h', '\u{0307}') => Some('ḣ'),
        ('m', '\u{0307}') => Some('ṁ'),
        ('n', '\u{0307}') => Some('ṅ'),
        ('p', '\u{0307}') => Some('ṗ'),
        ('r', '\u{0307}') => Some('ṙ'),
        ('s', '\u{0307}') => Some('ṡ'),
        ('t', '\u{0307}') => Some('ṫ'),
        ('w', '\u{0307}') => Some('ẇ'),
        ('x', '\u{0307}') => Some('ẋ'),
        ('y', '\u{0307}') => Some('ẏ'),
        ('z', '\u{0307}') => Some('ż'),
        ('A', '\u{0308}') => Some('Ä'),
        ('E', '\u{0308}') => Some('Ë'),
        ('H', '\u{0308}') => Some('Ḧ'),
        ('I', '\u{0308}') => Some('Ï'),
        ('O', '\u{0308}') => Some('Ö'),
        ('U', '\u{0308}') => Some('Ü'),
        ('W', '\u{0308}') => Some('Ẅ'),
        ('X', '\u{0308}') => Some('Ẍ'),
        ('Y', '\u{0308}') => Some('Ÿ'),
        ('a', '\u{0308}') => Some('ä'),
        ('e', '\u{0308}') => Some('ë'),
        ('h', '\u{0308}') => Some('ḧ'),
        ('i', '\u{0308}') => Some('ï'),
        ('o', '\u{0308}') => Some('ö'),
        ('t', '\u{0308}') => Some('ẗ'),
        ('u', '\u{0308}') => Some('ü'),
        ('w', '\u{0308}') => Some('ẅ'),
        ('x', '\u{0308}') => Some('ẍ'),
        ('y', '\u{0308}') => Some('ÿ'),
        ('A', '\u{030A}') => Some('Å'),
        ('U', '\u{030A}') => Some('Ů'),
        ('a', '\u{030A}') => Some('å'),
        ('u', '\u{030A}') => Some('ů'),
        ('w', '\u{030A}') => Some('ẘ'),
        ('y', '\u{030A}') => Some('ẙ'),
        ('O', '\u{030B}') => Some('Ő'),
        ('U', '\u{030B}') => Some('Ű'),
        ('o', '\u{030B}') => Some('ő'),
        ('u', '\u{030B}') => Some('ű'),
        ('A', '\u{030C}') => Some('Ǎ'),
        ('C', '\u{030C}') => Some('Č'),
        ('D', '\u{030C}') => Some('Ď'),
        ('E', '\u{030C}') => Some('Ě'),
        ('G', '\u{030C}') => Some('Ǧ'),
        ('H', '\u{030C}') => Some('Ȟ'),
        ('I', '\u{030C}') => Some('Ǐ'),
        ('K', '\u{030C}') => Some('Ǩ'),
        ('L', '\u{030C}') => Some('Ľ'),
        ('N', '\u{030C}') => Some('Ň'),
        ('O', '\u{030C}') => Some('Ǒ'),
        ('R', '\u{030C}') => Some('Ř'),
        ('S', '\u{030C}') => Some('Š'),
        ('T', '\u{030C}') => Some('Ť'),
        ('U', '\u{030C}') => Some('Ǔ'),
        ('Z', '\u{030C}') => Some('Ž'),
        ('a', '\u{030C}') => Some('ǎ'),
        ('c', '\u{030C}') => Some('č'),
        ('d', '\u{030C}') => Some('ď'),
        ('e', '\u{030C}') => Some('ě'),
        ('g', '\u{030C}') => Some('ǧ'),
        ('h', '\u{030C}') => Some('ȟ'),
        ('i', '\u{030C}') => Some('ǐ'),
        ('j', '\u{030C}') => Some('ǰ'),
        ('k', '\u{030C}') => Some('ǩ'),
        ('l', '\u{030C}') => Some('ľ'),
        ('n', '\u{030C}') => Some('ň'),
        ('o', '\u{030C}') => Some('ǒ'),
        ('r', '\u{030C}') => Some('ř'),
        ('s', '\u{030C}') => Some('š'),
        ('t', '\u{030C}') => Some('ť'),
        ('u', '\u{030C}') => Some('ǔ'),
        ('z', '\u{030C}') => Some('ž'),
        ('C', '\u{0327}') => Some('Ç'),
        ('D', '\u{0327}') => Some('Ḑ'),
        ('E', '\u{0327}') => Some('Ȩ'),
        ('G', '\u{0327}') => Some('Ģ'),
        ('H', '\u{0327}') => Some('Ḩ'),
        ('K', '\u{0327}') => Some('Ķ'),
        ('L', '\u{0327}') => Some('Ļ'),
        ('N', '\u{0327}') => Some('Ņ'),
        ('R', '\u{0327}') => Some('Ŗ'),
        ('S', '\u{0327}') => Some('Ş'),
        ('T', '\u{0327}') => Some('Ţ'),
        ('c', '\u{0327}') => Some('ç'),
        ('d', '\u{0327}') => Some('ḑ'),
        ('e', '\u{0327}') => Some('ȩ'),
        ('g', '\u{0327}') => Some('ģ'),
        ('h', '\u{0327}') => Some('ḩ'),
        ('k', '\u{0327}') => Some('ķ'),
        ('l', '\u{0327}') => Some('ļ'),
        ('n', '\u{0327}') => Some('ņ'),
        ('r', '\u{0327}') => Some('ŗ'),
        ('s', '\u{0327}') => Some('ş'),
        ('t', '\u{0327}') => Some('ţ'),
        ('A', '\u{0328}') => Some('Ą'),
        ('E', '\u{0328}') => Some('Ę'),
        ('I', '\u{0328}') => Some('Į'),
        ('O', '\u{0328}') => Some('Ǫ'),
        ('U', '\u{0328}') => Some('Ų'),
        ('a', '\u{0328}') => Some('ą'),
        ('e', '\u{0328}') => Some('ę'),
        ('i', '\u{0328}') => Some('į'),
        ('o', '\u{0328}') => Some('ǫ'),
        ('u', '\u{0328}') => Some('ų'),
        _ => None,
    }
}
/// Return all unique chars in a string, in order of first occurrence.
#[allow(dead_code)]
pub fn unique_chars(s: &str) -> Vec<char> {
    let mut seen = Vec::new();
    for c in s.chars() {
        if !seen.contains(&c) {
            seen.push(c);
        }
    }
    seen
}
/// Count occurrences of each character in a string.
#[allow(dead_code)]
pub fn char_frequency(s: &str) -> std::collections::HashMap<char, usize> {
    let mut map = std::collections::HashMap::new();
    for c in s.chars() {
        *map.entry(c).or_insert(0) += 1;
    }
    map
}
/// Check if a string is composed entirely of ASCII characters.
#[allow(dead_code)]
pub fn is_all_ascii(s: &str) -> bool {
    s.is_ascii()
}
/// Reverse a string correctly (by Unicode scalar values, not bytes).
#[allow(dead_code)]
pub fn reverse_str(s: &str) -> String {
    s.chars().rev().collect()
}
#[cfg(test)]
mod extra_char_tests {
    use super::*;
    #[test]
    fn test_char_range_contains() {
        let r = CharRange::new(65, 90);
        assert!(r.contains(65));
        assert!(r.contains(90));
        assert!(!r.contains(91));
    }
    #[test]
    fn test_char_range_size() {
        let r = CharRange::new(65, 90);
        assert_eq!(r.size(), 26);
    }
    #[test]
    fn test_char_range_chars() {
        let r = CharRange::new(65, 67);
        let chars: Vec<char> = r.chars().collect();
        assert_eq!(chars, vec!['A', 'B', 'C']);
    }
    #[test]
    fn test_unicode_blocks_basic_latin() {
        assert!(UnicodeBlocks::BASIC_LATIN.contains(b'A' as u32));
        assert!(!UnicodeBlocks::BASIC_LATIN.contains(0x100));
    }
    #[test]
    fn test_unicode_blocks_is_math_operator() {
        assert!(UnicodeBlocks::is_math_operator(0x2200));
        assert!(!UnicodeBlocks::is_math_operator(b'A' as u32));
    }
    #[test]
    fn test_unicode_blocks_is_greek() {
        assert!(UnicodeBlocks::is_greek(0x03B1));
        assert!(!UnicodeBlocks::is_greek(b'a' as u32));
    }
    #[test]
    fn test_unicode_blocks_is_arrow() {
        assert!(UnicodeBlocks::is_arrow(0x2192));
    }
    #[test]
    fn test_char_scanner_peek() {
        let scanner = CharScanner::new("abc");
        assert_eq!(scanner.peek(), Some('a'));
    }
    #[test]
    fn test_char_scanner_advance() {
        let mut scanner = CharScanner::new("abc");
        assert_eq!(scanner.advance(), Some('a'));
        assert_eq!(scanner.advance(), Some('b'));
        assert_eq!(scanner.peek(), Some('c'));
    }
    #[test]
    fn test_char_scanner_eat_success() {
        let mut scanner = CharScanner::new("abc");
        assert!(scanner.eat('a'));
        assert_eq!(scanner.peek(), Some('b'));
    }
    #[test]
    fn test_char_scanner_eat_fail() {
        let mut scanner = CharScanner::new("abc");
        assert!(!scanner.eat('z'));
        assert_eq!(scanner.peek(), Some('a'));
    }
    #[test]
    fn test_char_scanner_take_while_digits() {
        let mut scanner = CharScanner::new("123abc");
        let digits = scanner.take_while(|c| c.is_ascii_digit());
        assert_eq!(digits, "123");
        assert_eq!(scanner.peek(), Some('a'));
    }
    #[test]
    fn test_char_scanner_is_eof() {
        let mut scanner = CharScanner::new("x");
        assert!(!scanner.is_eof());
        scanner.advance();
        assert!(scanner.is_eof());
    }
    #[test]
    fn test_char_scanner_remaining() {
        let scanner = CharScanner::new("hello");
        assert_eq!(scanner.remaining(), 5);
    }
    #[test]
    fn test_normalize_whitespace() {
        let s = "hello\tworld\n";
        let norm = normalize_whitespace(s);
        assert!(!norm.contains('\t'));
        assert!(!norm.contains('\n'));
    }
    #[test]
    fn test_strip_control_chars() {
        let s = "hello\x00world\x1b";
        let clean = strip_control_chars(s);
        assert_eq!(clean, "helloworld");
    }
    #[test]
    fn test_unique_chars() {
        let chars = unique_chars("abcabc");
        assert_eq!(chars, vec!['a', 'b', 'c']);
    }
    #[test]
    fn test_char_frequency() {
        let freq = char_frequency("aabbbc");
        assert_eq!(freq[&'a'], 2);
        assert_eq!(freq[&'b'], 3);
        assert_eq!(freq[&'c'], 1);
    }
    #[test]
    fn test_is_all_ascii() {
        assert!(is_all_ascii("hello"));
        assert!(!is_all_ascii("héllo"));
    }
    #[test]
    fn test_reverse_str() {
        assert_eq!(reverse_str("hello"), "olleh");
        assert_eq!(reverse_str(""), "");
    }
    #[test]
    fn test_char_scanner_consumed() {
        let mut scanner = CharScanner::new("abc");
        scanner.advance();
        scanner.advance();
        assert_eq!(scanner.consumed(), "ab");
    }
    #[test]
    fn test_char_scanner_peek_at_offset() {
        let scanner = CharScanner::new("xyz");
        assert_eq!(scanner.peek_at(0), Some('x'));
        assert_eq!(scanner.peek_at(2), Some('z'));
        assert_eq!(scanner.peek_at(3), None);
    }
}
pub fn ch_ext_app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
pub fn ch_ext_app2(f: Expr, a: Expr, b: Expr) -> Expr {
    ch_ext_app(ch_ext_app(f, a), b)
}
pub fn ch_ext_cst(s: &str) -> Expr {
    Expr::Const(Name::str(s), vec![])
}
pub fn ch_ext_prop() -> Expr {
    Expr::Sort(Level::zero())
}
pub fn ch_ext_type0() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
pub fn ch_ext_bvar(n: u32) -> Expr {
    Expr::BVar(n)
}
pub fn ch_ext_nat_ty() -> Expr {
    ch_ext_cst("Nat")
}
pub fn ch_ext_char_ty() -> Expr {
    ch_ext_cst("Char")
}
pub fn ch_ext_bool_ty() -> Expr {
    ch_ext_cst("Bool")
}
pub fn ch_ext_string_ty() -> Expr {
    ch_ext_cst("String")
}
pub fn ch_ext_list_ty(elem: Expr) -> Expr {
    ch_ext_app(ch_ext_cst("List"), elem)
}
pub fn ch_ext_option_ty(inner: Expr) -> Expr {
    ch_ext_app(ch_ext_cst("Option"), inner)
}
pub fn ch_ext_prod_ty(a: Expr, b: Expr) -> Expr {
    ch_ext_app2(ch_ext_cst("Prod"), a, b)
}
pub fn ch_ext_arrow(dom: Expr, cod: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::Anonymous,
        Box::new(dom),
        Box::new(cod),
    )
}
pub fn ch_ext_pi(name: &str, dom: Expr, cod: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str(name),
        Box::new(dom),
        Box::new(cod),
    )
}
pub fn ch_ext_impl_pi(name: &str, dom: Expr, cod: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str(name),
        Box::new(dom),
        Box::new(cod),
    )
}
/// `Char.isValidScalar : Char -> Prop`
pub fn char_is_valid_scalar_ty() -> Expr {
    ch_ext_arrow(ch_ext_char_ty(), ch_ext_prop())
}
/// `Char.unicodeMax : Nat`
pub fn char_unicode_max_ty() -> Expr {
    ch_ext_nat_ty()
}
/// `Char.codepoint_lt_max : (c : Char) -> Char.toNat c < Char.unicodeMax`
pub fn char_codepoint_lt_max_ty() -> Expr {
    ch_ext_pi(
        "c",
        ch_ext_char_ty(),
        ch_ext_app2(
            ch_ext_cst("Nat.lt"),
            ch_ext_app(ch_ext_cst("Char.toNat"), ch_ext_bvar(0)),
            ch_ext_cst("Char.unicodeMax"),
        ),
    )
}
/// `Char.isoNat : {n : Nat} -> n < Char.unicodeMax -> Char.toNat (Char.ofNat n) = n`
pub fn char_iso_nat_ty() -> Expr {
    ch_ext_impl_pi(
        "n",
        ch_ext_nat_ty(),
        ch_ext_pi(
            "h",
            ch_ext_app2(
                ch_ext_cst("Nat.lt"),
                ch_ext_bvar(0),
                ch_ext_cst("Char.unicodeMax"),
            ),
            ch_ext_app2(
                ch_ext_cst("Eq"),
                ch_ext_app(
                    ch_ext_cst("Char.toNat"),
                    ch_ext_app(ch_ext_cst("Char.ofNat"), ch_ext_bvar(1)),
                ),
                ch_ext_bvar(1),
            ),
        ),
    )
}
/// `Char.ofNat_toNat : (c : Char) -> Char.ofNat (Char.toNat c) = c`
pub fn char_of_nat_to_nat_ty() -> Expr {
    ch_ext_pi(
        "c",
        ch_ext_char_ty(),
        ch_ext_app2(
            ch_ext_cst("Eq"),
            ch_ext_app(
                ch_ext_cst("Char.ofNat"),
                ch_ext_app(ch_ext_cst("Char.toNat"), ch_ext_bvar(0)),
            ),
            ch_ext_bvar(0),
        ),
    )
}
/// `Char.toUInt32 : Char -> UInt32`
pub fn char_to_uint32_ty() -> Expr {
    ch_ext_arrow(ch_ext_char_ty(), ch_ext_cst("UInt32"))
}
/// `Char.ofUInt32 : UInt32 -> Option Char`
pub fn char_of_uint32_ty() -> Expr {
    ch_ext_arrow(ch_ext_cst("UInt32"), ch_ext_option_ty(ch_ext_char_ty()))
}
/// `Char.toUInt32_injective : (a b : Char) -> Char.toUInt32 a = Char.toUInt32 b -> a = b`
pub fn char_to_uint32_injective_ty() -> Expr {
    ch_ext_pi(
        "a",
        ch_ext_char_ty(),
        ch_ext_pi(
            "b",
            ch_ext_char_ty(),
            ch_ext_arrow(
                ch_ext_app2(
                    ch_ext_cst("Eq"),
                    ch_ext_app(ch_ext_cst("Char.toUInt32"), ch_ext_bvar(1)),
                    ch_ext_app(ch_ext_cst("Char.toUInt32"), ch_ext_bvar(0)),
                ),
                ch_ext_app2(ch_ext_cst("Eq"), ch_ext_bvar(1), ch_ext_bvar(0)),
            ),
        ),
    )
}
/// `Char.decEq : (a b : Char) -> Decidable (a = b)`
pub fn char_dec_eq_ty() -> Expr {
    ch_ext_pi(
        "a",
        ch_ext_char_ty(),
        ch_ext_pi(
            "b",
            ch_ext_char_ty(),
            ch_ext_app(
                ch_ext_cst("Decidable"),
                ch_ext_app2(ch_ext_cst("Eq"), ch_ext_bvar(1), ch_ext_bvar(0)),
            ),
        ),
    )
}
/// `Char.lt : Char -> Char -> Prop`
pub fn char_lt_ty() -> Expr {
    ch_ext_arrow(
        ch_ext_char_ty(),
        ch_ext_arrow(ch_ext_char_ty(), ch_ext_prop()),
    )
}
/// `Char.le : Char -> Char -> Prop`
pub fn char_le_ty() -> Expr {
    ch_ext_arrow(
        ch_ext_char_ty(),
        ch_ext_arrow(ch_ext_char_ty(), ch_ext_prop()),
    )
}
/// `Char.lt_irrefl : (c : Char) -> Not (Char.lt c c)`
pub fn char_lt_irrefl_ty() -> Expr {
    ch_ext_pi(
        "c",
        ch_ext_char_ty(),
        ch_ext_app(
            ch_ext_cst("Not"),
            ch_ext_app2(ch_ext_cst("Char.lt"), ch_ext_bvar(0), ch_ext_bvar(0)),
        ),
    )
}
/// `Char.lt_trans : (a b c : Char) -> Char.lt a b -> Char.lt b c -> Char.lt a c`
pub fn char_lt_trans_ty() -> Expr {
    ch_ext_pi(
        "a",
        ch_ext_char_ty(),
        ch_ext_pi(
            "b",
            ch_ext_char_ty(),
            ch_ext_pi(
                "c",
                ch_ext_char_ty(),
                ch_ext_arrow(
                    ch_ext_app2(ch_ext_cst("Char.lt"), ch_ext_bvar(2), ch_ext_bvar(1)),
                    ch_ext_arrow(
                        ch_ext_app2(ch_ext_cst("Char.lt"), ch_ext_bvar(1), ch_ext_bvar(0)),
                        ch_ext_app2(ch_ext_cst("Char.lt"), ch_ext_bvar(2), ch_ext_bvar(0)),
                    ),
                ),
            ),
        ),
    )
}
/// `Char.lt_total : (a b : Char) -> Char.lt a b Or a = b Or Char.lt b a`
pub fn char_lt_total_ty() -> Expr {
    ch_ext_pi(
        "a",
        ch_ext_char_ty(),
        ch_ext_pi(
            "b",
            ch_ext_char_ty(),
            ch_ext_app2(
                ch_ext_cst("Or"),
                ch_ext_app2(ch_ext_cst("Char.lt"), ch_ext_bvar(1), ch_ext_bvar(0)),
                ch_ext_app2(
                    ch_ext_cst("Or"),
                    ch_ext_app2(ch_ext_cst("Eq"), ch_ext_bvar(1), ch_ext_bvar(0)),
                    ch_ext_app2(ch_ext_cst("Char.lt"), ch_ext_bvar(0), ch_ext_bvar(1)),
                ),
            ),
        ),
    )
}
/// `Char.IsAlpha : Char -> Prop`
pub fn char_is_alpha_prop_ty() -> Expr {
    ch_ext_arrow(ch_ext_char_ty(), ch_ext_prop())
}
/// `Char.IsDigit : Char -> Prop`
pub fn char_is_digit_prop_ty() -> Expr {
    ch_ext_arrow(ch_ext_char_ty(), ch_ext_prop())
}
/// `Char.IsAlphaNum : Char -> Prop`
pub fn char_is_alphanum_prop_ty() -> Expr {
    ch_ext_arrow(ch_ext_char_ty(), ch_ext_prop())
}
/// `Char.IsSpace : Char -> Prop`
pub fn char_is_space_prop_ty() -> Expr {
    ch_ext_arrow(ch_ext_char_ty(), ch_ext_prop())
}
/// `Char.IsPunct : Char -> Prop`
pub fn char_is_punct_prop_ty() -> Expr {
    ch_ext_arrow(ch_ext_char_ty(), ch_ext_prop())
}
/// `Char.not_surrogate : (c : Char) -> Not (0xD800 <= Char.toNat c And Char.toNat c <= 0xDFFF)`
pub fn char_not_surrogate_ty() -> Expr {
    ch_ext_pi(
        "c",
        ch_ext_char_ty(),
        ch_ext_app(
            ch_ext_cst("Not"),
            ch_ext_app2(
                ch_ext_cst("And"),
                ch_ext_app2(
                    ch_ext_cst("Nat.le"),
                    Expr::Lit(oxilean_kernel::Literal::Nat(0xD800_u32.into())),
                    ch_ext_app(ch_ext_cst("Char.toNat"), ch_ext_bvar(0)),
                ),
                ch_ext_app2(
                    ch_ext_cst("Nat.le"),
                    ch_ext_app(ch_ext_cst("Char.toNat"), ch_ext_bvar(0)),
                    Expr::Lit(oxilean_kernel::Literal::Nat(0xDFFF_u32.into())),
                ),
            ),
        ),
    )
}
/// `Char.succ : Char -> Option Char`
pub fn char_succ_ty() -> Expr {
    ch_ext_arrow(ch_ext_char_ty(), ch_ext_option_ty(ch_ext_char_ty()))
}
/// `Char.pred : Char -> Option Char`
pub fn char_pred_ty() -> Expr {
    ch_ext_arrow(ch_ext_char_ty(), ch_ext_option_ty(ch_ext_char_ty()))
}
/// `Char.succ_pred_prop : (c : Char) -> Prop`
pub fn char_succ_pred_ty() -> Expr {
    ch_ext_pi("c", ch_ext_char_ty(), ch_ext_prop())
}
/// `Char.toString : Char -> String`
pub fn char_to_string_ty() -> Expr {
    ch_ext_arrow(ch_ext_char_ty(), ch_ext_string_ty())
}
/// `Char.toList : Char -> List Char`
pub fn char_to_list_ty() -> Expr {
    ch_ext_arrow(ch_ext_char_ty(), ch_ext_list_ty(ch_ext_char_ty()))
}
/// `Char.isPrefix : Char -> String -> Bool`
pub fn char_is_prefix_ty() -> Expr {
    ch_ext_arrow(
        ch_ext_char_ty(),
        ch_ext_arrow(ch_ext_string_ty(), ch_ext_bool_ty()),
    )
}
/// `Char.lexLt : Char -> Char -> Prop`
pub fn char_lex_lt_ty() -> Expr {
    ch_ext_arrow(
        ch_ext_char_ty(),
        ch_ext_arrow(ch_ext_char_ty(), ch_ext_prop()),
    )
}
/// `Char.UnicodeCategory : Type`
pub fn char_unicode_category_ty() -> Expr {
    ch_ext_type0()
}
/// `Char.generalCategory : Char -> Char.UnicodeCategory`
pub fn char_general_category_fn_ty() -> Expr {
    ch_ext_arrow(ch_ext_char_ty(), ch_ext_cst("Char.UnicodeCategory"))
}
/// `Char.isLetter : Char -> Prop`
pub fn char_is_letter_ty() -> Expr {
    ch_ext_arrow(ch_ext_char_ty(), ch_ext_prop())
}
/// `Char.isNumber : Char -> Prop`
pub fn char_is_number_ty() -> Expr {
    ch_ext_arrow(ch_ext_char_ty(), ch_ext_prop())
}
/// `Char.isSymbol : Char -> Prop`
pub fn char_is_symbol_ty() -> Expr {
    ch_ext_arrow(ch_ext_char_ty(), ch_ext_prop())
}
/// `Char.isMark : Char -> Prop`
pub fn char_is_mark_ty() -> Expr {
    ch_ext_arrow(ch_ext_char_ty(), ch_ext_prop())
}
/// `Char.utf8Width : Char -> Nat`
pub fn char_utf8_width_ty() -> Expr {
    ch_ext_arrow(ch_ext_char_ty(), ch_ext_nat_ty())
}
/// `Char.utf8Width_range : (c : Char) -> 1 <= Char.utf8Width c And Char.utf8Width c <= 4`
pub fn char_utf8_width_range_ty() -> Expr {
    ch_ext_pi(
        "c",
        ch_ext_char_ty(),
        ch_ext_app2(
            ch_ext_cst("And"),
            ch_ext_app2(
                ch_ext_cst("Nat.le"),
                Expr::Lit(oxilean_kernel::Literal::Nat(1_u32.into())),
                ch_ext_app(ch_ext_cst("Char.utf8Width"), ch_ext_bvar(0)),
            ),
            ch_ext_app2(
                ch_ext_cst("Nat.le"),
                ch_ext_app(ch_ext_cst("Char.utf8Width"), ch_ext_bvar(0)),
                Expr::Lit(oxilean_kernel::Literal::Nat(4_u32.into())),
            ),
        ),
    )
}
/// `Char.utf8Bytes : Char -> List Nat`
pub fn char_utf8_bytes_ty() -> Expr {
    ch_ext_arrow(ch_ext_char_ty(), ch_ext_list_ty(ch_ext_nat_ty()))
}
/// `Char.nfcNormalize : Char -> Option Char`
pub fn char_nfc_normalize_ty() -> Expr {
    ch_ext_arrow(ch_ext_char_ty(), ch_ext_option_ty(ch_ext_char_ty()))
}
/// `Char.nfdDecompose : Char -> List Char`
pub fn char_nfd_decompose_ty() -> Expr {
    ch_ext_arrow(ch_ext_char_ty(), ch_ext_list_ty(ch_ext_char_ty()))
}
/// `Char.nfkcNormalize : Char -> Option Char`
pub fn char_nfkc_normalize_ty() -> Expr {
    ch_ext_arrow(ch_ext_char_ty(), ch_ext_option_ty(ch_ext_char_ty()))
}
/// `Char.nfkdDecompose : Char -> List Char`
pub fn char_nfkd_decompose_ty() -> Expr {
    ch_ext_arrow(ch_ext_char_ty(), ch_ext_list_ty(ch_ext_char_ty()))
}
/// `Char.BidiCategory : Type`
pub fn char_bidi_category_ty() -> Expr {
    ch_ext_type0()
}
/// `Char.bidiCategory : Char -> Char.BidiCategory`
pub fn char_bidi_category_fn_ty() -> Expr {
    ch_ext_arrow(ch_ext_char_ty(), ch_ext_cst("Char.BidiCategory"))
}
/// `Char.isLTR : Char -> Prop`
pub fn char_is_ltr_ty() -> Expr {
    ch_ext_arrow(ch_ext_char_ty(), ch_ext_prop())
}
/// `Char.isRTL : Char -> Prop`
pub fn char_is_rtl_ty() -> Expr {
    ch_ext_arrow(ch_ext_char_ty(), ch_ext_prop())
}
/// `Char.caseFold : Char -> Char`
pub fn char_case_fold_ty() -> Expr {
    ch_ext_arrow(ch_ext_char_ty(), ch_ext_char_ty())
}
/// `Char.caseFold_idempotent : (c : Char) -> Char.caseFold (Char.caseFold c) = Char.caseFold c`
pub fn char_case_fold_idempotent_ty() -> Expr {
    ch_ext_pi(
        "c",
        ch_ext_char_ty(),
        ch_ext_app2(
            ch_ext_cst("Eq"),
            ch_ext_app(
                ch_ext_cst("Char.caseFold"),
                ch_ext_app(ch_ext_cst("Char.caseFold"), ch_ext_bvar(0)),
            ),
            ch_ext_app(ch_ext_cst("Char.caseFold"), ch_ext_bvar(0)),
        ),
    )
}
/// `Char.CollationKey : Type`
pub fn char_collation_key_ty() -> Expr {
    ch_ext_type0()
}
/// `Char.collationKey : Char -> Char.CollationKey`
pub fn char_collation_key_fn_ty() -> Expr {
    ch_ext_arrow(ch_ext_char_ty(), ch_ext_cst("Char.CollationKey"))
}
/// `Char.collationLe : Char.CollationKey -> Char.CollationKey -> Prop`
pub fn char_collation_le_ty() -> Expr {
    ch_ext_arrow(
        ch_ext_cst("Char.CollationKey"),
        ch_ext_arrow(ch_ext_cst("Char.CollationKey"), ch_ext_prop()),
    )
}
/// `Char.RegexClass : Type`
pub fn char_regex_class_ty() -> Expr {
    ch_ext_type0()
}
/// `Char.matchesClass : Char -> Char.RegexClass -> Bool`
pub fn char_matches_class_ty() -> Expr {
    ch_ext_arrow(
        ch_ext_char_ty(),
        ch_ext_arrow(ch_ext_cst("Char.RegexClass"), ch_ext_bool_ty()),
    )
}
/// `Char.isCombining : Char -> Bool`
pub fn char_is_combining_ty() -> Expr {
    ch_ext_arrow(ch_ext_char_ty(), ch_ext_bool_ty())
}
/// `Char.graphemeBreakProp : Char -> Nat`
pub fn char_grapheme_break_prop_ty() -> Expr {
    ch_ext_arrow(ch_ext_char_ty(), ch_ext_nat_ty())
}
/// `Char.isTerminal : Char -> Prop`
pub fn char_is_terminal_ty() -> Expr {
    ch_ext_arrow(ch_ext_char_ty(), ch_ext_prop())
}
/// `Char.TerminalAlphabet : Type`
pub fn char_terminal_alphabet_ty() -> Expr {
    ch_ext_type0()
}
/// `Char.inAlphabet : Char -> Char.TerminalAlphabet -> Prop`
pub fn char_in_alphabet_ty() -> Expr {
    ch_ext_arrow(
        ch_ext_char_ty(),
        ch_ext_arrow(ch_ext_cst("Char.TerminalAlphabet"), ch_ext_prop()),
    )
}
/// `Char.ascii_subset : (c : Char) -> Prop`
pub fn char_ascii_subset_ty() -> Expr {
    ch_ext_pi(
        "c",
        ch_ext_char_ty(),
        ch_ext_arrow(
            ch_ext_app2(
                ch_ext_cst("Eq"),
                ch_ext_app(ch_ext_cst("Char.isAscii"), ch_ext_bvar(0)),
                ch_ext_cst("Bool.true"),
            ),
            ch_ext_app2(
                ch_ext_cst("Nat.lt"),
                ch_ext_app(ch_ext_cst("Char.toNat"), ch_ext_bvar(0)),
                Expr::Lit(oxilean_kernel::Literal::Nat(128_u32.into())),
            ),
        ),
    )
}
/// `Char.natToDigit : Nat -> Option Char`
pub fn char_nat_to_digit_ty() -> Expr {
    ch_ext_arrow(ch_ext_nat_ty(), ch_ext_option_ty(ch_ext_char_ty()))
}
/// `Char.digitToNat_natToDigit : (n : Nat) -> n < 10 -> Prop`
pub fn char_digit_round_trip_ty() -> Expr {
    ch_ext_pi(
        "n",
        ch_ext_nat_ty(),
        ch_ext_arrow(
            ch_ext_app2(
                ch_ext_cst("Nat.lt"),
                ch_ext_bvar(0),
                Expr::Lit(oxilean_kernel::Literal::Nat(10_u32.into())),
            ),
            ch_ext_prop(),
        ),
    )
}
/// `Char.composeWith : Char -> Char -> Option Char`
pub fn char_compose_with_ty() -> Expr {
    ch_ext_arrow(
        ch_ext_char_ty(),
        ch_ext_arrow(ch_ext_char_ty(), ch_ext_option_ty(ch_ext_char_ty())),
    )
}
/// `Char.decomposeFirst : Char -> Option (Prod Char Char)`
pub fn char_decompose_first_ty() -> Expr {
    ch_ext_arrow(
        ch_ext_char_ty(),
        ch_ext_option_ty(ch_ext_prod_ty(ch_ext_char_ty(), ch_ext_char_ty())),
    )
}
/// Register all extended Char kernel axioms into `env`.
pub fn register_char_extended_axioms(env: &mut Environment) {
    let axioms: &[(&str, fn() -> Expr)] = &[
        ("Char.isValidScalar", char_is_valid_scalar_ty),
        ("Char.unicodeMax", char_unicode_max_ty),
        ("Char.codepoint_lt_max", char_codepoint_lt_max_ty),
        ("Char.isoNat", char_iso_nat_ty),
        ("Char.ofNat_toNat", char_of_nat_to_nat_ty),
        ("Char.toUInt32", char_to_uint32_ty),
        ("Char.ofUInt32", char_of_uint32_ty),
        ("Char.toUInt32_injective", char_to_uint32_injective_ty),
        ("Char.decEq", char_dec_eq_ty),
        ("Char.lt", char_lt_ty),
        ("Char.le", char_le_ty),
        ("Char.lt_irrefl", char_lt_irrefl_ty),
        ("Char.lt_trans", char_lt_trans_ty),
        ("Char.lt_total", char_lt_total_ty),
        ("Char.IsAlpha", char_is_alpha_prop_ty),
        ("Char.IsDigit", char_is_digit_prop_ty),
        ("Char.IsAlphaNum", char_is_alphanum_prop_ty),
        ("Char.IsSpace", char_is_space_prop_ty),
        ("Char.IsPunct", char_is_punct_prop_ty),
        ("Char.not_surrogate", char_not_surrogate_ty),
        ("Char.succ", char_succ_ty),
        ("Char.pred", char_pred_ty),
        ("Char.succ_pred", char_succ_pred_ty),
        ("Char.toString", char_to_string_ty),
        ("Char.toList", char_to_list_ty),
        ("Char.isPrefix", char_is_prefix_ty),
        ("Char.lexLt", char_lex_lt_ty),
        ("Char.UnicodeCategory", char_unicode_category_ty),
        ("Char.generalCategory", char_general_category_fn_ty),
        ("Char.isLetter", char_is_letter_ty),
        ("Char.isNumber", char_is_number_ty),
        ("Char.isSymbol", char_is_symbol_ty),
        ("Char.isMark", char_is_mark_ty),
        ("Char.utf8Width", char_utf8_width_ty),
        ("Char.utf8Width_range", char_utf8_width_range_ty),
        ("Char.utf8Bytes", char_utf8_bytes_ty),
        ("Char.nfcNormalize", char_nfc_normalize_ty),
        ("Char.nfdDecompose", char_nfd_decompose_ty),
        ("Char.nfkcNormalize", char_nfkc_normalize_ty),
        ("Char.nfkdDecompose", char_nfkd_decompose_ty),
        ("Char.BidiCategory", char_bidi_category_ty),
        ("Char.bidiCategory", char_bidi_category_fn_ty),
        ("Char.isLTR", char_is_ltr_ty),
        ("Char.isRTL", char_is_rtl_ty),
        ("Char.caseFold", char_case_fold_ty),
        ("Char.caseFold_idempotent", char_case_fold_idempotent_ty),
        ("Char.CollationKey", char_collation_key_ty),
        ("Char.collationKey", char_collation_key_fn_ty),
        ("Char.collationLe", char_collation_le_ty),
        ("Char.RegexClass", char_regex_class_ty),
        ("Char.matchesClass", char_matches_class_ty),
        ("Char.isCombining", char_is_combining_ty),
        ("Char.graphemeBreakProp", char_grapheme_break_prop_ty),
        ("Char.isTerminal", char_is_terminal_ty),
        ("Char.TerminalAlphabet", char_terminal_alphabet_ty),
        ("Char.inAlphabet", char_in_alphabet_ty),
        ("Char.ascii_subset", char_ascii_subset_ty),
        ("Char.natToDigit", char_nat_to_digit_ty),
        ("Char.digitToNat_natToDigit", char_digit_round_trip_ty),
        ("Char.composeWith", char_compose_with_ty),
        ("Char.decomposeFirst", char_decompose_first_ty),
    ];
    for (name, build) in axioms {
        let _ = env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: build(),
        });
    }
}
