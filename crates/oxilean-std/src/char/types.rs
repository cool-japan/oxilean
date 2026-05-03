//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

/// Applies Unicode normalization passes to strings and character sequences.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CharNormalizer {
    /// Which normalization form to apply.
    pub form: NormalizationForm,
    /// Strip control characters.
    pub strip_controls: bool,
    /// Map all Unicode whitespace to ASCII space.
    pub normalize_whitespace_flag: bool,
}
impl CharNormalizer {
    /// Create a normalizer.
    #[allow(dead_code)]
    pub fn new(form: NormalizationForm) -> Self {
        CharNormalizer {
            form,
            strip_controls: false,
            normalize_whitespace_flag: false,
        }
    }
    /// Enable control-char stripping.
    #[allow(dead_code)]
    pub fn with_strip_controls(mut self) -> Self {
        self.strip_controls = true;
        self
    }
    /// Enable whitespace normalization.
    #[allow(dead_code)]
    pub fn with_normalize_whitespace(mut self) -> Self {
        self.normalize_whitespace_flag = true;
        self
    }
    /// Apply the pipeline to `input`.
    #[allow(dead_code)]
    pub fn normalize(&self, input: &str) -> String {
        let mut s = input.to_owned();
        if self.strip_controls {
            s = strip_control_chars(&s);
        }
        if self.normalize_whitespace_flag {
            s = normalize_whitespace(&s);
        }
        match self.form {
            NormalizationForm::Nfc | NormalizationForm::Nfkc => normalize_to_nfc_approx(&s),
            NormalizationForm::Nfd | NormalizationForm::Nfkd | NormalizationForm::None => s,
        }
    }
    /// Normalize a single character (best-effort, returns the char unchanged).
    #[allow(dead_code)]
    pub fn normalize_char(&self, c: char) -> Vec<char> {
        vec![c]
    }
    /// Human-readable description.
    #[allow(dead_code)]
    pub fn description(&self) -> String {
        let form = match self.form {
            NormalizationForm::Nfc => "NFC",
            NormalizationForm::Nfd => "NFD",
            NormalizationForm::Nfkc => "NFKC",
            NormalizationForm::Nfkd => "NFKD",
            NormalizationForm::None => "None",
        };
        format!(
            "CharNormalizer(form={}, strip_controls={}, normalize_whitespace={})",
            form, self.strip_controls, self.normalize_whitespace_flag
        )
    }
}
/// A compact representation of a Unicode character with metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharInfo {
    /// The character itself.
    pub ch: char,
    /// Unicode code point.
    pub code_point: u32,
    /// UTF-8 encoded length in bytes.
    pub utf8_len: usize,
    /// Whether this character is ASCII.
    pub is_ascii: bool,
    /// General Unicode category.
    pub category: CharCategory,
}
impl CharInfo {
    /// Create a new `CharInfo` for a given character.
    pub fn new(c: char) -> Self {
        CharInfo {
            ch: c,
            code_point: c as u32,
            utf8_len: c.len_utf8(),
            is_ascii: c.is_ascii(),
            category: unicode_category(c),
        }
    }
    /// Return true if the character is a letter.
    pub fn is_letter(&self) -> bool {
        matches!(
            self.category,
            CharCategory::UppercaseLetter
                | CharCategory::LowercaseLetter
                | CharCategory::TitlecaseLetter
                | CharCategory::ModifierLetter
                | CharCategory::OtherLetter
        )
    }
    /// Return true if the character is a digit.
    pub fn is_digit(&self) -> bool {
        matches!(self.category, CharCategory::DecimalNumber)
    }
    /// Return true if the character is whitespace.
    pub fn is_whitespace(&self) -> bool {
        matches!(
            self.category,
            CharCategory::SpaceSeparator | CharCategory::LineSeparator
        )
    }
}
/// A table mapping OxiLean character predicate names to Rust predicates.
#[allow(clippy::type_complexity)]
pub struct CharPredicateTable {
    entries: Vec<(&'static str, fn(char) -> bool)>,
}
impl CharPredicateTable {
    /// Create the default predicate table.
    pub fn new() -> Self {
        CharPredicateTable {
            entries: vec![
                ("isAlpha", |c: char| c.is_alphabetic()),
                ("isDigit", |c: char| c.is_ascii_digit()),
                ("isAlphaNum", |c: char| c.is_alphanumeric()),
                ("isUpper", |c: char| c.is_uppercase()),
                ("isLower", |c: char| c.is_lowercase()),
                ("isWhitespace", |c: char| c.is_whitespace()),
                ("isAscii", |c: char| c.is_ascii()),
                ("isControl", |c: char| c.is_control()),
                ("isPrint", |c: char| !c.is_control()),
                ("isHexDigit", |c: char| c.is_ascii_hexdigit()),
            ],
        }
    }
    /// Look up a predicate by OxiLean name.
    pub fn lookup(&self, name: &str) -> Option<fn(char) -> bool> {
        self.entries
            .iter()
            .find(|(n, _)| *n == name)
            .map(|(_, f)| *f)
    }
    /// Apply the predicate named `name` to character `c`.
    pub fn apply(&self, name: &str, c: char) -> Option<bool> {
        self.lookup(name).map(|f| f(c))
    }
    /// Return all predicate names.
    pub fn names(&self) -> Vec<&'static str> {
        self.entries.iter().map(|(n, _)| *n).collect()
    }
}
/// A compact char range: all code points in \[start, end\] (inclusive).
///
/// Used to describe Unicode blocks or script ranges.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CharRange {
    /// First code point in the range.
    pub start: u32,
    /// Last code point in the range (inclusive).
    pub end: u32,
}
impl CharRange {
    /// Create a char range.
    #[allow(dead_code)]
    pub fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }
    /// Check whether a code point is within this range.
    #[allow(dead_code)]
    pub fn contains(&self, cp: u32) -> bool {
        cp >= self.start && cp <= self.end
    }
    /// Number of code points in this range.
    #[allow(dead_code)]
    pub fn size(&self) -> u32 {
        self.end.saturating_sub(self.start) + 1
    }
    /// Iterate over all valid chars in this range.
    #[allow(dead_code)]
    pub fn chars(&self) -> impl Iterator<Item = char> {
        let start = self.start;
        let end = self.end;
        (start..=end).filter_map(char::from_u32)
    }
}
/// A Unicode scalar value bundled with precomputed metadata.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnicodeChar {
    /// The underlying Rust character.
    pub ch: char,
    /// Unicode code point.
    pub code_point: u32,
    /// UTF-8 encoded length (1-4 bytes).
    pub utf8_width: usize,
    /// UTF-16 code unit count (1 or 2).
    pub utf16_width: usize,
    /// True when ASCII.
    pub is_ascii: bool,
    /// True when a combining character (heuristic).
    pub is_combining: bool,
    /// True when in surrogate range (never valid in Rust char).
    pub is_surrogate: bool,
}
impl UnicodeChar {
    /// Construct from a Rust `char`.
    #[allow(dead_code)]
    pub fn new(c: char) -> Self {
        let cp = c as u32;
        let is_combining = (0x0300..=0x036F).contains(&cp)
            || (0x1AB0..=0x1AFF).contains(&cp)
            || (0x1DC0..=0x1DFF).contains(&cp)
            || (0x20D0..=0x20FF).contains(&cp)
            || (0xFE20..=0xFE2F).contains(&cp);
        let is_surrogate = (0xD800..=0xDFFF).contains(&cp);
        UnicodeChar {
            ch: c,
            code_point: cp,
            utf8_width: c.len_utf8(),
            utf16_width: c.len_utf16(),
            is_ascii: c.is_ascii(),
            is_combining,
            is_surrogate,
        }
    }
    /// Build a kernel expression for this character.
    #[allow(dead_code)]
    pub fn to_expr(&self) -> Expr {
        make_char_literal(self.code_point)
    }
    /// Return simplified Unicode block name.
    #[allow(dead_code)]
    pub fn block_name(&self) -> &'static str {
        match self.code_point {
            0x0000..=0x007F => "Basic Latin",
            0x0080..=0x00FF => "Latin-1 Supplement",
            0x0100..=0x017F => "Latin Extended-A",
            0x0180..=0x024F => "Latin Extended-B",
            0x0300..=0x036F => "Combining Diacritical Marks",
            0x0370..=0x03FF => "Greek and Coptic",
            0x0400..=0x04FF => "Cyrillic",
            0x0500..=0x052F => "Cyrillic Supplement",
            0x0600..=0x06FF => "Arabic",
            0x0900..=0x097F => "Devanagari",
            0x4E00..=0x9FFF => "CJK Unified Ideographs",
            0x1D400..=0x1D7FF => "Mathematical Alphanumeric Symbols",
            0x1F600..=0x1F64F => "Emoticons",
            _ => "Other",
        }
    }
    /// True when no case distinction.
    #[allow(dead_code)]
    pub fn is_caseless(&self) -> bool {
        !self.ch.is_uppercase() && !self.ch.is_lowercase()
    }
}
/// Normalization form selector.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NormalizationForm {
    Nfc,
    Nfd,
    Nfkc,
    Nfkd,
    None,
}
/// Encodes and decodes characters to/from various byte representations.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CharEncoder {
    /// The active encoding.
    pub encoding: CharEncoding,
}
impl CharEncoder {
    /// Create a new encoder.
    #[allow(dead_code)]
    pub fn new(encoding: CharEncoding) -> Self {
        CharEncoder { encoding }
    }
    /// Encode `c` as bytes.
    #[allow(dead_code)]
    pub fn encode(&self, c: char) -> Vec<u8> {
        match self.encoding {
            CharEncoding::Utf8 => {
                let mut buf = [0u8; 4];
                let len = c.encode_utf8(&mut buf).len();
                buf[..len].to_vec()
            }
            CharEncoding::Utf16Le => {
                let mut buf = [0u16; 2];
                let len = c.encode_utf16(&mut buf).len();
                buf[..len].iter().flat_map(|u| u.to_le_bytes()).collect()
            }
            CharEncoding::Utf16Be => {
                let mut buf = [0u16; 2];
                let len = c.encode_utf16(&mut buf).len();
                buf[..len].iter().flat_map(|u| u.to_be_bytes()).collect()
            }
            CharEncoding::Utf32Le => (c as u32).to_le_bytes().to_vec(),
        }
    }
    /// Decode the first character from `bytes`.
    #[allow(dead_code)]
    pub fn decode_first(&self, bytes: &[u8]) -> Option<(char, usize)> {
        match self.encoding {
            CharEncoding::Utf8 => utf8_decode_first(bytes),
            CharEncoding::Utf32Le => {
                if bytes.len() < 4 {
                    return None;
                }
                let cp = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
                char::from_u32(cp).map(|c| (c, 4))
            }
            CharEncoding::Utf16Le => {
                if bytes.len() < 2 {
                    return None;
                }
                let u0 = u16::from_le_bytes([bytes[0], bytes[1]]);
                if (0xD800..=0xDBFF).contains(&u0) {
                    if bytes.len() < 4 {
                        return None;
                    }
                    let u1 = u16::from_le_bytes([bytes[2], bytes[3]]);
                    let cp = 0x10000 + ((u0 as u32 - 0xD800) << 10) + (u1 as u32 - 0xDC00);
                    char::from_u32(cp).map(|c| (c, 4))
                } else {
                    char::from_u32(u0 as u32).map(|c| (c, 2))
                }
            }
            CharEncoding::Utf16Be => {
                if bytes.len() < 2 {
                    return None;
                }
                let u0 = u16::from_be_bytes([bytes[0], bytes[1]]);
                if (0xD800..=0xDBFF).contains(&u0) {
                    if bytes.len() < 4 {
                        return None;
                    }
                    let u1 = u16::from_be_bytes([bytes[2], bytes[3]]);
                    let cp = 0x10000 + ((u0 as u32 - 0xD800) << 10) + (u1 as u32 - 0xDC00);
                    char::from_u32(cp).map(|c| (c, 4))
                } else {
                    char::from_u32(u0 as u32).map(|c| (c, 2))
                }
            }
        }
    }
    /// Encode an entire string.
    #[allow(dead_code)]
    pub fn encode_str(&self, s: &str) -> Vec<u8> {
        s.chars().flat_map(|c| self.encode(c)).collect()
    }
}
/// Unicode general category for a character.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CharCategory {
    /// Uppercase letter (Lu)
    UppercaseLetter,
    /// Lowercase letter (Ll)
    LowercaseLetter,
    /// Titlecase letter (Lt)
    TitlecaseLetter,
    /// Modifier letter (Lm)
    ModifierLetter,
    /// Other letter (Lo)
    OtherLetter,
    /// Decimal digit (Nd)
    DecimalNumber,
    /// Letter number (Nl)
    LetterNumber,
    /// Other number (No)
    OtherNumber,
    /// Connector punctuation (Pc)
    ConnectorPunctuation,
    /// Dash punctuation (Pd)
    DashPunctuation,
    /// Open punctuation (Ps)
    OpenPunctuation,
    /// Close punctuation (Pe)
    ClosePunctuation,
    /// Space separator (Zs)
    SpaceSeparator,
    /// Line separator (Zl)
    LineSeparator,
    /// Control (Cc)
    Control,
    /// Format (Cf)
    Format,
    /// Math symbol (Sm)
    MathSymbol,
    /// Currency symbol (Sc)
    CurrencySymbol,
    /// Other symbol (So)
    OtherSymbol,
    /// Unknown / unclassified
    Unknown,
}
/// Named Unicode character blocks relevant to OxiLean.
#[allow(dead_code)]
pub struct UnicodeBlocks;
impl UnicodeBlocks {
    /// Basic Latin (ASCII range).
    pub const BASIC_LATIN: CharRange = CharRange {
        start: 0x0000,
        end: 0x007F,
    };
    /// Latin-1 Supplement.
    pub const LATIN1_SUPPLEMENT: CharRange = CharRange {
        start: 0x0080,
        end: 0x00FF,
    };
    /// Greek and Coptic.
    pub const GREEK: CharRange = CharRange {
        start: 0x0370,
        end: 0x03FF,
    };
    /// Mathematical Operators.
    pub const MATH_OPERATORS: CharRange = CharRange {
        start: 0x2200,
        end: 0x22FF,
    };
    /// Supplemental Mathematical Operators.
    pub const SUPP_MATH_OPERATORS: CharRange = CharRange {
        start: 0x2A00,
        end: 0x2AFF,
    };
    /// Mathematical Alphanumeric Symbols.
    pub const MATH_ALPHANUMERIC: CharRange = CharRange {
        start: 0x1D400,
        end: 0x1D7FF,
    };
    /// Letterlike Symbols.
    pub const LETTERLIKE: CharRange = CharRange {
        start: 0x2100,
        end: 0x214F,
    };
    /// Arrows.
    pub const ARROWS: CharRange = CharRange {
        start: 0x2190,
        end: 0x21FF,
    };
    /// Check if a code point is in the mathematical operators range.
    #[allow(dead_code)]
    pub fn is_math_operator(cp: u32) -> bool {
        Self::MATH_OPERATORS.contains(cp) || Self::SUPP_MATH_OPERATORS.contains(cp)
    }
    /// Check if a code point is in the Greek range.
    #[allow(dead_code)]
    pub fn is_greek(cp: u32) -> bool {
        Self::GREEK.contains(cp)
    }
    /// Check if a code point is in the arrows range.
    #[allow(dead_code)]
    pub fn is_arrow(cp: u32) -> bool {
        Self::ARROWS.contains(cp)
    }
}
/// Classifies characters by configurable named rules.
#[allow(dead_code)]
pub struct CharClassifier {
    rules: Vec<(&'static str, fn(char) -> bool)>,
}
impl CharClassifier {
    /// Build with the standard Unicode-aware rule set.
    #[allow(dead_code)]
    pub fn standard() -> Self {
        CharClassifier {
            rules: vec![
                ("letter", |c| c.is_alphabetic()),
                ("digit", |c| c.is_numeric()),
                ("alphanumeric", |c| c.is_alphanumeric()),
                ("whitespace", |c| c.is_whitespace()),
                ("uppercase", |c| c.is_uppercase()),
                ("lowercase", |c| c.is_lowercase()),
                ("ascii", |c| c.is_ascii()),
                ("control", |c| c.is_control()),
                ("printable", |c| !c.is_control()),
                ("hex_digit", |c| c.is_ascii_hexdigit()),
                ("combining", |c| {
                    let cp = c as u32;
                    (0x0300..=0x036F).contains(&cp) || (0x20D0..=0x20FF).contains(&cp)
                }),
                ("emoji", |c| {
                    let cp = c as u32;
                    (0x1F600..=0x1F64F).contains(&cp)
                        || (0x1F300..=0x1F5FF).contains(&cp)
                        || (0x2600..=0x26FF).contains(&cp)
                }),
            ],
        }
    }
    /// All matching class names for `c`.
    #[allow(dead_code)]
    pub fn classify(&self, c: char) -> Vec<&'static str> {
        self.rules
            .iter()
            .filter(|(_, pred)| pred(c))
            .map(|(name, _)| *name)
            .collect()
    }
    /// True when `c` belongs to `class_name`.
    #[allow(dead_code)]
    pub fn belongs_to(&self, c: char, class_name: &str) -> bool {
        self.rules
            .iter()
            .find(|(name, _)| *name == class_name)
            .is_some_and(|(_, pred)| pred(c))
    }
    /// All registered class names.
    #[allow(dead_code)]
    pub fn class_names(&self) -> Vec<&'static str> {
        self.rules.iter().map(|(name, _)| *name).collect()
    }
}
/// A grapheme cluster: one or more code points forming a user-perceived char.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphemeCluster {
    /// Code points in this cluster.
    pub codepoints: Vec<char>,
}
impl GraphemeCluster {
    /// Singleton cluster.
    #[allow(dead_code)]
    pub fn singleton(base: char) -> Self {
        GraphemeCluster {
            codepoints: vec![base],
        }
    }
    /// Base character with combining marks.
    #[allow(dead_code)]
    pub fn with_combining(base: char, combining: impl IntoIterator<Item = char>) -> Self {
        let mut codepoints = vec![base];
        codepoints.extend(combining);
        GraphemeCluster { codepoints }
    }
    /// True when cluster is a single code point.
    #[allow(dead_code)]
    pub fn is_singleton(&self) -> bool {
        self.codepoints.len() == 1
    }
    /// True when cluster contains a combining mark.
    #[allow(dead_code)]
    pub fn has_combining(&self) -> bool {
        self.codepoints.iter().skip(1).any(|&c| {
            let cp = c as u32;
            (0x0300..=0x036F).contains(&cp) || (0x20D0..=0x20FF).contains(&cp)
        })
    }
    /// Render as `String`.
    #[allow(dead_code)]
    pub fn to_string_repr(&self) -> String {
        self.codepoints.iter().collect()
    }
    /// Total UTF-8 byte length.
    #[allow(dead_code)]
    pub fn utf8_byte_len(&self) -> usize {
        self.codepoints.iter().map(|c| c.len_utf8()).sum()
    }
    /// First (base) code point.
    #[allow(dead_code)]
    pub fn base(&self) -> Option<char> {
        self.codepoints.first().copied()
    }
    /// Attempt NFC composition to a single char.
    #[allow(dead_code)]
    pub fn try_compose(&self) -> Option<char> {
        if self.codepoints.len() == 2 {
            compose_pair(self.codepoints[0], self.codepoints[1])
        } else if self.codepoints.len() == 1 {
            Some(self.codepoints[0])
        } else {
            None
        }
    }
}
/// A simple char scanner for iterating over source text.
///
/// Provides look-ahead operations useful in the OxiLean lexer.
#[allow(dead_code)]
pub struct CharScanner {
    chars: Vec<char>,
    pos: usize,
}
impl CharScanner {
    /// Create a new scanner from a string.
    #[allow(dead_code)]
    pub fn new(s: &str) -> Self {
        Self {
            chars: s.chars().collect(),
            pos: 0,
        }
    }
    /// Peek at the current character without consuming.
    #[allow(dead_code)]
    pub fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }
    /// Peek at the character `offset` positions ahead.
    #[allow(dead_code)]
    pub fn peek_at(&self, offset: usize) -> Option<char> {
        self.chars.get(self.pos + offset).copied()
    }
    /// Consume and return the current character.
    #[allow(dead_code)]
    pub fn advance(&mut self) -> Option<char> {
        let c = self.chars.get(self.pos).copied();
        if c.is_some() {
            self.pos += 1;
        }
        c
    }
    /// Consume the current character if it equals `expected`.
    #[allow(dead_code)]
    pub fn eat(&mut self, expected: char) -> bool {
        if self.peek() == Some(expected) {
            self.pos += 1;
            true
        } else {
            false
        }
    }
    /// Consume while `predicate` returns true. Returns consumed string.
    #[allow(dead_code)]
    pub fn take_while(&mut self, predicate: impl Fn(char) -> bool) -> String {
        let start = self.pos;
        while self.peek().is_some_and(&predicate) {
            self.pos += 1;
        }
        self.chars[start..self.pos].iter().collect()
    }
    /// Return the remaining (unconsumed) characters.
    #[allow(dead_code)]
    pub fn remaining(&self) -> usize {
        self.chars.len().saturating_sub(self.pos)
    }
    /// Check if the scanner is at end of input.
    #[allow(dead_code)]
    pub fn is_eof(&self) -> bool {
        self.pos >= self.chars.len()
    }
    /// Return all consumed characters as a string.
    #[allow(dead_code)]
    pub fn consumed(&self) -> String {
        self.chars[..self.pos].iter().collect()
    }
}
/// Supported encodings for `CharEncoder`.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharEncoding {
    Utf8,
    Utf16Le,
    Utf16Be,
    Utf32Le,
}
