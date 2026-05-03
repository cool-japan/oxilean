//! Impl blocks for lean4_compat

use super::super::functions::*;
use oxilean_kernel::*;

use super::defs::*;

/// Checks a source file for Lean 4 constructs unsupported by OxiLean.
#[allow(dead_code)]
pub struct Lean4CompatChecker;
impl Lean4CompatChecker {
    /// Check `src` for known compatibility issues.
    #[allow(dead_code)]
    pub fn check(src: &str) -> Vec<CompatIssue> {
        let mut issues = Vec::new();
        for (i, line) in src.lines().enumerate() {
            let line_no = (i + 1) as u32;
            if line.contains("=>") && !line.trim_start().starts_with("--") {
                issues.push(CompatIssue::new(
                    line_no,
                    "Fat arrow `=>` should be replaced with `->` in OxiLean.",
                    IssueSeverity::Error,
                ));
            }
            if line.contains(".{") {
                issues.push(CompatIssue::new(
                    line_no,
                    "Universe annotation `.{...}` will be stripped.",
                    IssueSeverity::Warning,
                ));
            }
            if line.trim_start().starts_with("macro ") {
                issues.push(CompatIssue::new(
                    line_no,
                    "Lean 4 `macro` declaration is not supported; use `def` instead.",
                    IssueSeverity::Error,
                ));
            }
            if line.trim_start().starts_with("#eval") {
                issues.push(CompatIssue::new(
                    line_no,
                    "`#eval` is not available in OxiLean kernel mode.",
                    IssueSeverity::Warning,
                ));
            }
            if line.trim_start().starts_with("mutual") {
                issues.push(CompatIssue::new(
                    line_no,
                    "`mutual` blocks are supported.",
                    IssueSeverity::Info,
                ));
            }
        }
        issues
    }
    /// Filter issues by severity.
    #[allow(dead_code)]
    pub fn filter_by_severity(issues: &[CompatIssue], sev: IssueSeverity) -> Vec<&CompatIssue> {
        issues.iter().filter(|i| i.severity == sev).collect()
    }
    /// Returns true if there are any errors.
    #[allow(dead_code)]
    pub fn has_errors(issues: &[CompatIssue]) -> bool {
        issues.iter().any(|i| i.severity == IssueSeverity::Error)
    }
}
impl ScopeKind {
    /// Returns the Lean 4 keyword for this scope kind.
    #[allow(dead_code)]
    pub fn keyword(&self) -> &'static str {
        match self {
            ScopeKind::Namespace => "namespace",
            ScopeKind::Section => "section",
        }
    }
}
/// Applies textual rewrite rules to Lean 4 source fragments.
#[allow(dead_code)]
pub struct Lean4TermRewriter {
    rules: Vec<(String, String)>,
}
impl Lean4TermRewriter {
    /// Create an empty rewriter.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Lean4TermRewriter { rules: Vec::new() }
    }
    /// Add a rewrite rule: replace all occurrences of `from` with `to`.
    #[allow(dead_code)]
    pub fn add_rule(mut self, from: &str, to: &str) -> Self {
        self.rules.push((from.to_string(), to.to_string()));
        self
    }
    /// Apply all rules in order to `src`.
    #[allow(dead_code)]
    pub fn rewrite(&self, src: &str) -> String {
        let mut result = src.to_string();
        for (from, to) in &self.rules {
            result = result.replace(from.as_str(), to.as_str());
        }
        result
    }
    /// Returns the number of rules.
    #[allow(dead_code)]
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }
    /// Build a rewriter with the standard OxiLean adaptations.
    #[allow(dead_code)]
    pub fn standard() -> Self {
        Lean4TermRewriter::new()
            .add_rule(" => ", " -> ")
            .add_rule("←", "<-")
            .add_rule("where;", "where")
            .add_rule("∧", " && ")
            .add_rule("∨", " || ")
            .add_rule("¬", "!")
    }
}
impl Lean4Feature {
    /// Returns a human-readable label for this feature.
    pub fn label(&self) -> &'static str {
        match self {
            Lean4Feature::DoNotation => "do-notation",
            Lean4Feature::MacroExpansion => "macro-expansion",
            Lean4Feature::AutoBoundImplicits => "auto-bound-implicits",
            Lean4Feature::StructureInheritance => "structure-inheritance",
            Lean4Feature::DeclarationAttributes => "declaration-attributes",
            Lean4Feature::UniversePolymorphism => "universe-polymorphism",
            Lean4Feature::TacticMode => "tactic-mode",
            Lean4Feature::MetaProgramming => "meta-programming",
            Lean4Feature::PatternMatching => "pattern-matching",
            Lean4Feature::MutualRecursion => "mutual-recursion",
            Lean4Feature::WhereBindings => "where-bindings",
            Lean4Feature::Notation => "notation",
        }
    }
    /// All known features, in definition order.
    pub fn all() -> Vec<Lean4Feature> {
        vec![
            Lean4Feature::DoNotation,
            Lean4Feature::MacroExpansion,
            Lean4Feature::AutoBoundImplicits,
            Lean4Feature::StructureInheritance,
            Lean4Feature::DeclarationAttributes,
            Lean4Feature::UniversePolymorphism,
            Lean4Feature::TacticMode,
            Lean4Feature::MetaProgramming,
            Lean4Feature::PatternMatching,
            Lean4Feature::MutualRecursion,
            Lean4Feature::WhereBindings,
            Lean4Feature::Notation,
        ]
    }
}
impl Lean4Feature {
    /// Returns a short description of the feature.
    #[allow(dead_code)]
    pub fn description(&self) -> &'static str {
        match self {
            Lean4Feature::DoNotation => "Monadic do-notation sequencing with ← binds.",
            Lean4Feature::MacroExpansion => "User-defined syntactic macros via macro_rules.",
            Lean4Feature::AutoBoundImplicits => "Automatic implicit binding of free variables.",
            Lean4Feature::StructureInheritance => "Structure extension via `extends` keyword.",
            Lean4Feature::DeclarationAttributes => "Attributes like @[simp] on declarations.",
            Lean4Feature::UniversePolymorphism => "Definitions polymorphic over universe levels.",
            Lean4Feature::TacticMode => "Interactive proof construction via `by` blocks.",
            Lean4Feature::MetaProgramming => "Lean 4 meta-programming and macro monad.",
            Lean4Feature::PatternMatching => "Dependent pattern matching in definitions.",
            Lean4Feature::MutualRecursion => "Mutually recursive definitions via `mutual`.",
            Lean4Feature::WhereBindings => "Local definitions in `where` clauses.",
            Lean4Feature::Notation => "User-defined notation declarations.",
        }
    }
    /// Returns true if this is a core language feature (not a library feature).
    #[allow(dead_code)]
    pub fn is_core(&self) -> bool {
        matches!(
            self,
            Lean4Feature::TacticMode
                | Lean4Feature::PatternMatching
                | Lean4Feature::MutualRecursion
                | Lean4Feature::UniversePolymorphism
                | Lean4Feature::AutoBoundImplicits
        )
    }
    /// Returns true if this feature affects the surface syntax.
    #[allow(dead_code)]
    pub fn affects_surface_syntax(&self) -> bool {
        matches!(
            self,
            Lean4Feature::DoNotation
                | Lean4Feature::MacroExpansion
                | Lean4Feature::Notation
                | Lean4Feature::WhereBindings
                | Lean4Feature::StructureInheritance
        )
    }
}
/// Classifies Lean 4 identifiers into keyword categories.
#[allow(dead_code)]
pub struct Lean4KeywordClassifier;
impl Lean4KeywordClassifier {
    /// Classify a token string.
    #[allow(dead_code)]
    pub fn classify(token: &str) -> Lean4KeywordCategory {
        match token {
            "def" | "theorem" | "lemma" | "axiom" | "opaque" | "abbrev" | "noncomputable"
            | "private" | "protected" | "partial" => Lean4KeywordCategory::Declaration,
            "intro" | "intros" | "exact" | "apply" | "refl" | "simp" | "ring" | "linarith"
            | "omega" | "cases" | "induction" | "constructor" | "left" | "right" | "have"
            | "show" | "by_contra" | "push_neg" | "split" | "rw" | "assumption" | "trivial"
            | "sorry" | "clear" | "revert" | "repeat" | "try" | "first" | "all_goals"
            | "exfalso" | "exists" | "use" => Lean4KeywordCategory::Tactic,
            "structure" | "class" | "instance" | "extends" | "where" | "deriving" | "mutual" => {
                Lean4KeywordCategory::StructureClass
            }
            "if" | "then" | "else" | "match" | "with" | "fun" | "do" | "return" | "let" | "in"
            | "by" => Lean4KeywordCategory::ControlFlow,
            "import" | "namespace" | "open" | "section" | "end" | "variable" | "universe" => {
                Lean4KeywordCategory::Namespace
            }
            "Sort" | "Type" | "Prop" => Lean4KeywordCategory::Universe,
            "and" | "or" | "not" | "forall" => Lean4KeywordCategory::LogicalOp,
            _ => Lean4KeywordCategory::NotKeyword,
        }
    }
    /// Returns true if the token is any kind of keyword.
    #[allow(dead_code)]
    pub fn is_keyword(token: &str) -> bool {
        !matches!(
            Lean4KeywordClassifier::classify(token),
            Lean4KeywordCategory::NotKeyword
        )
    }
    /// Returns all Lean 4 declaration keywords.
    #[allow(dead_code)]
    pub fn declaration_keywords() -> Vec<&'static str> {
        vec![
            "def",
            "theorem",
            "lemma",
            "axiom",
            "opaque",
            "abbrev",
            "noncomputable",
            "private",
            "protected",
            "partial",
        ]
    }
    /// Returns all Lean 4 tactic keywords.
    #[allow(dead_code)]
    pub fn tactic_keywords() -> Vec<&'static str> {
        vec![
            "intro",
            "intros",
            "exact",
            "apply",
            "refl",
            "simp",
            "ring",
            "linarith",
            "omega",
            "cases",
            "induction",
            "constructor",
            "left",
            "right",
            "have",
            "show",
            "by_contra",
            "push_neg",
            "split",
            "rw",
            "assumption",
            "trivial",
            "sorry",
            "clear",
            "revert",
            "repeat",
            "try",
            "first",
            "all_goals",
            "exfalso",
            "exists",
            "use",
        ]
    }
    /// Returns all namespace-related keywords.
    #[allow(dead_code)]
    pub fn namespace_keywords() -> Vec<&'static str> {
        vec![
            "import",
            "namespace",
            "open",
            "section",
            "end",
            "variable",
            "universe",
        ]
    }
}
/// Resolves Lean 4 import statements to file paths.
#[allow(dead_code)]
pub struct Lean4ImportResolver {
    /// Root directories to search.
    roots: Vec<String>,
}
impl Lean4ImportResolver {
    /// Create a new resolver with the given roots.
    #[allow(dead_code)]
    pub fn new(roots: Vec<&str>) -> Self {
        Lean4ImportResolver {
            roots: roots.iter().map(|s| s.to_string()).collect(),
        }
    }
    /// Resolve a dotted module name to a relative file path.
    /// E.g. `Mathlib.Data.Nat.Basic` → `Mathlib/Data/Nat/Basic.lean`
    #[allow(dead_code)]
    pub fn module_to_path(module: &str) -> String {
        format!("{}.lean", module.replace('.', "/"))
    }
    /// Resolve a module name against the roots.
    /// Returns the first matching path, or None.
    #[allow(dead_code)]
    pub fn resolve(&self, module: &str) -> Option<String> {
        let rel = Self::module_to_path(module);
        self.roots.first().map(|root| format!("{}/{}", root, rel))
    }
    /// Parse `import Foo.Bar` statements from source, returning module names.
    #[allow(dead_code)]
    pub fn parse_imports(src: &str) -> Vec<String> {
        src.lines()
            .filter_map(|line| {
                let t = line.trim();
                t.strip_prefix("import ").map(|s| s.trim().to_string())
            })
            .collect()
    }
    /// Add a root directory.
    #[allow(dead_code)]
    pub fn add_root(&mut self, root: &str) {
        self.roots.push(root.to_string());
    }
    /// Returns the number of roots.
    #[allow(dead_code)]
    pub fn root_count(&self) -> usize {
        self.roots.len()
    }
}
/// Identifies which version of Lean 4 syntax is in use.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Lean4SyntaxVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}
impl Lean4SyntaxVersion {
    /// Lean 4 v4.0.0.
    #[allow(dead_code)]
    pub fn v4_0_0() -> Self {
        Lean4SyntaxVersion {
            major: 4,
            minor: 0,
            patch: 0,
        }
    }
    /// Lean 4 v4.3.0.
    #[allow(dead_code)]
    pub fn v4_3_0() -> Self {
        Lean4SyntaxVersion {
            major: 4,
            minor: 3,
            patch: 0,
        }
    }
    /// Lean 4 v4.6.0 (current stable).
    #[allow(dead_code)]
    pub fn v4_6_0() -> Self {
        Lean4SyntaxVersion {
            major: 4,
            minor: 6,
            patch: 0,
        }
    }
    /// Returns true if this version is at least `other`.
    #[allow(dead_code)]
    pub fn is_at_least(&self, other: &Lean4SyntaxVersion) -> bool {
        self >= other
    }
}
/// Classifies Lean 4 keywords by their syntactic role.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Lean4KeywordCategory {
    /// Declaration starters: `def`, `theorem`, `axiom`, etc.
    Declaration,
    /// Tactic keywords: `intro`, `apply`, `simp`, etc.
    Tactic,
    /// Structure/class keywords: `structure`, `class`, `instance`.
    StructureClass,
    /// Control flow: `if`, `then`, `else`, `match`, `fun`.
    ControlFlow,
    /// Import/namespace: `import`, `namespace`, `open`, `section`.
    Namespace,
    /// Universe: `Sort`, `Type`, `Prop`.
    Universe,
    /// Logical operators: `and`, `or`, `not`.
    LogicalOp,
    /// Not a keyword.
    NotKeyword,
}
/// Extracts Lean 4 docstrings from source code.
#[allow(dead_code)]
pub struct Lean4DocstringExtractor;
impl Lean4DocstringExtractor {
    /// Extract the content of a `/-- ... -/` docstring at the start of `src`.
    /// Returns `(docstring_content, rest_of_source)`.
    #[allow(dead_code)]
    pub fn extract_leading_docstring(src: &str) -> Option<(&str, &str)> {
        let src = src.trim_start();
        if !src.starts_with("/--") {
            return None;
        }
        let inner_start = 3;
        if let Some(end_pos) = src.find("-/") {
            let doc = &src[inner_start..end_pos];
            let rest = &src[end_pos + 2..];
            Some((doc.trim(), rest.trim_start()))
        } else {
            None
        }
    }
    /// Extract all docstrings from a source file, returning a list of
    /// `(line_number, content)` pairs.
    #[allow(dead_code)]
    pub fn extract_all_docstrings(src: &str) -> Vec<(usize, String)> {
        let mut results = Vec::new();
        let mut line_num = 1usize;
        let mut remaining = src;
        while !remaining.is_empty() {
            if let Some(start) = remaining.find("/--") {
                let before = &remaining[..start];
                line_num += before.chars().filter(|&c| c == '\n').count();
                let after_start = &remaining[start + 3..];
                if let Some(end) = after_start.find("-/") {
                    let content = after_start[..end].trim().to_string();
                    results.push((line_num, content));
                    let consumed = start + 3 + end + 2;
                    remaining = &remaining[consumed..];
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        results
    }
    /// Strip all docstrings from source, replacing with empty lines.
    #[allow(dead_code)]
    pub fn strip_docstrings(src: &str) -> String {
        let mut result = src.to_string();
        while let Some(start) = result.find("/--") {
            if let Some(end) = result[start..].find("-/") {
                let abs_end = start + end + 2;
                result.replace_range(start..abs_end, "");
            } else {
                break;
            }
        }
        result
    }
}
/// Severity of a compatibility issue.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IssueSeverity {
    Error,
    Warning,
    Info,
}
impl IssueSeverity {
    /// Returns the severity label.
    #[allow(dead_code)]
    pub fn label(&self) -> &'static str {
        match self {
            IssueSeverity::Error => "error",
            IssueSeverity::Warning => "warning",
            IssueSeverity::Info => "info",
        }
    }
}
impl Lean4TokenKind {
    /// Returns a human-readable label.
    #[allow(dead_code)]
    pub fn label(&self) -> &'static str {
        match self {
            Lean4TokenKind::Ident => "identifier",
            Lean4TokenKind::Keyword => "keyword",
            Lean4TokenKind::IntLit => "integer-literal",
            Lean4TokenKind::FloatLit => "float-literal",
            Lean4TokenKind::StringLit => "string-literal",
            Lean4TokenKind::CharLit => "char-literal",
            Lean4TokenKind::Arrow => "->",
            Lean4TokenKind::FatArrow => "=>",
            Lean4TokenKind::Dot => ".",
            Lean4TokenKind::Comma => ",",
            Lean4TokenKind::Colon => ":",
            Lean4TokenKind::ColonEq => ":=",
            Lean4TokenKind::Semicolon => ";",
            Lean4TokenKind::LParen => "(",
            Lean4TokenKind::RParen => ")",
            Lean4TokenKind::LBrace => "{",
            Lean4TokenKind::RBrace => "}",
            Lean4TokenKind::LBracket => "[",
            Lean4TokenKind::RBracket => "]",
            Lean4TokenKind::At => "@",
            Lean4TokenKind::Hash => "#",
            Lean4TokenKind::Pipe => "|",
            Lean4TokenKind::Backslash => "\\",
            Lean4TokenKind::Ampersand => "&",
            Lean4TokenKind::Star => "*",
            Lean4TokenKind::Plus => "+",
            Lean4TokenKind::Minus => "-",
            Lean4TokenKind::Slash => "/",
            Lean4TokenKind::Percent => "%",
            Lean4TokenKind::Eq => "=",
            Lean4TokenKind::Ne => "≠",
            Lean4TokenKind::Lt => "<",
            Lean4TokenKind::Gt => ">",
            Lean4TokenKind::Le => "≤",
            Lean4TokenKind::Ge => "≥",
            Lean4TokenKind::And => "∧",
            Lean4TokenKind::Or => "∨",
            Lean4TokenKind::Not => "¬",
            Lean4TokenKind::Eof => "<eof>",
            Lean4TokenKind::Unknown => "<unknown>",
        }
    }
    /// Returns true if this token can start an expression.
    #[allow(dead_code)]
    pub fn can_start_expr(&self) -> bool {
        matches!(
            self,
            Lean4TokenKind::Ident
                | Lean4TokenKind::IntLit
                | Lean4TokenKind::FloatLit
                | Lean4TokenKind::StringLit
                | Lean4TokenKind::CharLit
                | Lean4TokenKind::LParen
                | Lean4TokenKind::LBrace
                | Lean4TokenKind::LBracket
                | Lean4TokenKind::Backslash
                | Lean4TokenKind::Not
                | Lean4TokenKind::Minus
        )
    }
}
impl FieldVisibility {
    /// Returns the Lean 4 keyword for this visibility.
    #[allow(dead_code)]
    pub fn as_str(&self) -> &'static str {
        match self {
            FieldVisibility::Public => "public",
            FieldVisibility::Protected => "protected",
            FieldVisibility::Private => "private",
        }
    }
}
impl Lean4TypeAnnotation {
    /// Returns the OxiLean rendering brackets for this annotation form.
    #[allow(dead_code)]
    pub fn brackets(&self) -> (&'static str, &'static str) {
        match self {
            Lean4TypeAnnotation::Ascription => ("(", ")"),
            Lean4TypeAnnotation::Implicit => ("{", "}"),
            Lean4TypeAnnotation::Instance => ("[", "]"),
            Lean4TypeAnnotation::StrictImplicit => ("{{", "}}"),
            Lean4TypeAnnotation::AutoParam => ("(", " := _)"),
            Lean4TypeAnnotation::OptParam => ("(", ")?"),
        }
    }
    /// Returns a human-readable label for this annotation.
    #[allow(dead_code)]
    pub fn label(&self) -> &'static str {
        match self {
            Lean4TypeAnnotation::Ascription => "ascription",
            Lean4TypeAnnotation::Implicit => "implicit",
            Lean4TypeAnnotation::Instance => "instance",
            Lean4TypeAnnotation::StrictImplicit => "strict-implicit",
            Lean4TypeAnnotation::AutoParam => "auto-param",
            Lean4TypeAnnotation::OptParam => "opt-param",
        }
    }
    /// Returns true if this annotation form is implicitly resolved.
    #[allow(dead_code)]
    pub fn is_implicit(&self) -> bool {
        matches!(
            self,
            Lean4TypeAnnotation::Implicit
                | Lean4TypeAnnotation::Instance
                | Lean4TypeAnnotation::StrictImplicit
        )
    }
    /// Returns all annotation variants.
    #[allow(dead_code)]
    pub fn all() -> Vec<Lean4TypeAnnotation> {
        vec![
            Lean4TypeAnnotation::Ascription,
            Lean4TypeAnnotation::Implicit,
            Lean4TypeAnnotation::Instance,
            Lean4TypeAnnotation::StrictImplicit,
            Lean4TypeAnnotation::AutoParam,
            Lean4TypeAnnotation::OptParam,
        ]
    }
}
/// A structured report of OxiLean's Lean 4 compatibility.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Lean4CompatReport {
    /// Feature name.
    pub feature: String,
    /// Compatibility level.
    pub level: CompatLevel,
    /// Brief description of what is supported.
    pub supported_description: String,
    /// Brief description of what is not supported.
    pub gaps: Vec<String>,
    /// Known workarounds.
    pub workarounds: Vec<String>,
}
impl Lean4CompatReport {
    /// Create a new compat report.
    #[allow(dead_code)]
    pub fn new(feature: &str, level: CompatLevel) -> Self {
        Lean4CompatReport {
            feature: feature.to_string(),
            level,
            supported_description: String::new(),
            gaps: Vec::new(),
            workarounds: Vec::new(),
        }
    }
    /// Set the supported description.
    #[allow(dead_code)]
    pub fn with_supported(mut self, desc: &str) -> Self {
        self.supported_description = desc.to_string();
        self
    }
    /// Add a gap description.
    #[allow(dead_code)]
    pub fn with_gap(mut self, gap: &str) -> Self {
        self.gaps.push(gap.to_string());
        self
    }
    /// Add a workaround description.
    #[allow(dead_code)]
    pub fn with_workaround(mut self, w: &str) -> Self {
        self.workarounds.push(w.to_string());
        self
    }
    /// Format as a Markdown section.
    #[allow(dead_code)]
    pub fn to_markdown(&self) -> String {
        let mut out = String::new();
        let level_str = match &self.level {
            CompatLevel::Full => "Full",
            CompatLevel::Partial(_) => "Partial",
            CompatLevel::Stub => "Stub",
            CompatLevel::Unsupported => "Unsupported",
        };
        out.push_str(&format!("## {} [{}]\n\n", self.feature, level_str));
        if !self.supported_description.is_empty() {
            out.push_str(&format!(
                "**Supported:** {}\n\n",
                self.supported_description
            ));
        }
        if !self.gaps.is_empty() {
            out.push_str("**Gaps:**\n");
            for gap in &self.gaps {
                out.push_str(&format!("- {}\n", gap));
            }
            out.push('\n');
        }
        if !self.workarounds.is_empty() {
            out.push_str("**Workarounds:**\n");
            for w in &self.workarounds {
                out.push_str(&format!("- {}\n", w));
            }
            out.push('\n');
        }
        out
    }
    /// Returns true if the feature is fully supported.
    #[allow(dead_code)]
    pub fn is_full(&self) -> bool {
        matches!(self.level, CompatLevel::Full)
    }
    /// Returns true if there are known gaps.
    #[allow(dead_code)]
    pub fn has_gaps(&self) -> bool {
        !self.gaps.is_empty()
    }
}
/// A single compatibility issue found in source.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CompatIssue {
    /// Line number (1-based).
    pub line: u32,
    /// Description.
    pub message: String,
    /// Severity.
    pub severity: IssueSeverity,
}
impl CompatIssue {
    /// Create a new issue.
    #[allow(dead_code)]
    pub fn new(line: u32, message: &str, severity: IssueSeverity) -> Self {
        CompatIssue {
            line,
            message: message.to_string(),
            severity,
        }
    }
    /// Format for display.
    #[allow(dead_code)]
    pub fn format(&self) -> String {
        format!(
            "[{}] line {}: {}",
            self.severity.label(),
            self.line,
            self.message
        )
    }
}
/// Categories of Lean 4 / OxiLean elaboration errors.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Lean4ErrorKind {
    /// Type mismatch between expected and inferred types.
    TypeMismatch,
    /// Unknown identifier.
    UnknownIdent,
    /// Tactic failure.
    TacticFailure,
    /// Universe level error.
    UniverseError,
    /// Unsupported feature encountered.
    UnsupportedFeature,
    /// Syntax error in the source.
    SyntaxError,
    /// Instance synthesis failure.
    InstanceSynthesis,
    /// Application arity mismatch.
    ArityMismatch,
    /// Pattern matching non-exhaustive.
    NonExhaustiveMatch,
    /// Recursive definition not well-founded.
    TerminationError,
    /// Other / internal error.
    Internal,
}
impl Lean4ErrorKind {
    /// Returns a short label.
    #[allow(dead_code)]
    pub fn label(&self) -> &'static str {
        match self {
            Lean4ErrorKind::TypeMismatch => "type-mismatch",
            Lean4ErrorKind::UnknownIdent => "unknown-identifier",
            Lean4ErrorKind::TacticFailure => "tactic-failure",
            Lean4ErrorKind::UniverseError => "universe-error",
            Lean4ErrorKind::UnsupportedFeature => "unsupported-feature",
            Lean4ErrorKind::SyntaxError => "syntax-error",
            Lean4ErrorKind::InstanceSynthesis => "instance-synthesis",
            Lean4ErrorKind::ArityMismatch => "arity-mismatch",
            Lean4ErrorKind::NonExhaustiveMatch => "non-exhaustive-match",
            Lean4ErrorKind::TerminationError => "termination-error",
            Lean4ErrorKind::Internal => "internal",
        }
    }
    /// Returns true if the error is recoverable (elaboration can continue).
    #[allow(dead_code)]
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            Lean4ErrorKind::TacticFailure
                | Lean4ErrorKind::UnsupportedFeature
                | Lean4ErrorKind::InstanceSynthesis
        )
    }
}
/// A structured elaboration error.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Lean4ElabError {
    /// Error kind.
    pub kind: Lean4ErrorKind,
    /// Human-readable message.
    pub message: String,
    /// Source span: (line, col).
    pub span: Option<(u32, u32)>,
    /// Suggestions for fixing the error.
    pub hints: Vec<String>,
}
impl Lean4ElabError {
    /// Create a new elaboration error.
    #[allow(dead_code)]
    pub fn new(kind: Lean4ErrorKind, message: &str) -> Self {
        Lean4ElabError {
            kind,
            message: message.to_string(),
            span: None,
            hints: Vec::new(),
        }
    }
    /// Set the source span.
    #[allow(dead_code)]
    pub fn at(mut self, line: u32, col: u32) -> Self {
        self.span = Some((line, col));
        self
    }
    /// Add a hint.
    #[allow(dead_code)]
    pub fn with_hint(mut self, hint: &str) -> Self {
        self.hints.push(hint.to_string());
        self
    }
    /// Format the error as a human-readable string.
    #[allow(dead_code)]
    pub fn format(&self) -> String {
        let loc = self
            .span
            .map(|(l, c)| format!(" at {}:{}", l, c))
            .unwrap_or_default();
        let mut out = format!("error[{}]{}: {}", self.kind.label(), loc, self.message);
        for hint in &self.hints {
            out.push_str(&format!("\n  hint: {}", hint));
        }
        out
    }
    /// Returns true if this error is recoverable.
    #[allow(dead_code)]
    pub fn is_recoverable(&self) -> bool {
        self.kind.is_recoverable()
    }
}
/// A difference between two Lean 4 syntax versions.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Lean4SyntaxDiff {
    /// What changed.
    pub change: String,
    /// Version where the change was introduced.
    pub since: Lean4SyntaxVersion,
    /// Whether the old syntax is still accepted (deprecated).
    pub backward_compat: bool,
}
impl Lean4SyntaxDiff {
    /// Create a new syntax diff.
    #[allow(dead_code)]
    pub fn new(change: &str, since: Lean4SyntaxVersion, backward_compat: bool) -> Self {
        Lean4SyntaxDiff {
            change: change.to_string(),
            since,
            backward_compat,
        }
    }
    /// Returns true if old syntax still works.
    #[allow(dead_code)]
    pub fn is_backward_compat(&self) -> bool {
        self.backward_compat
    }
}
/// Converts between Lean 4 and OxiLean name conventions.
pub struct Lean4NameConverter;
impl Lean4NameConverter {
    /// Converts a Lean 4 dotted name to an OxiLean name.
    ///
    /// Currently keeps the dotted form as-is since OxiLean also uses dots.
    pub fn to_oxilean_name(lean4_name: &str) -> String {
        lean4_name.to_string()
    }
    /// Converts an OxiLean name back to Lean 4 dotted form.
    pub fn from_oxilean_name(oxilean_name: &str) -> String {
        oxilean_name.to_string()
    }
    /// Returns true if `name` is a valid OxiLean identifier.
    ///
    /// Rules: non-empty, starts with a letter or underscore, subsequent chars
    /// are alphanumeric, underscore, prime `'`, or dot `.` (for namespacing).
    pub fn is_valid_oxilean_name(name: &str) -> bool {
        if name.is_empty() {
            return false;
        }
        let mut chars = name.chars();
        let first = chars.next().expect("name is non-empty (checked above)");
        if !first.is_alphabetic() && first != '_' {
            return false;
        }
        chars.all(|c| c.is_alphanumeric() || c == '_' || c == '\'' || c == '.')
    }
}
impl Lean4NameConverter {
    /// Convert a CamelCase name to snake_case.
    #[allow(dead_code)]
    pub fn camel_to_snake(name: &str) -> String {
        let mut out = String::new();
        for (i, ch) in name.chars().enumerate() {
            if ch.is_uppercase() && i > 0 {
                out.push('_');
            }
            out.push(ch.to_lowercase().next().unwrap_or(ch));
        }
        out
    }
    /// Convert a snake_case name to CamelCase.
    #[allow(dead_code)]
    pub fn snake_to_camel(name: &str) -> String {
        name.split('_')
            .filter(|s| !s.is_empty())
            .map(|s| {
                let mut c = s.chars();
                match c.next() {
                    None => String::new(),
                    Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                }
            })
            .collect()
    }
    /// Strip the namespace prefix from a fully-qualified name.
    /// E.g. `Nat.add` → `add`.
    #[allow(dead_code)]
    pub fn strip_namespace(name: &str) -> &str {
        name.rsplit('.').next().unwrap_or(name)
    }
    /// Return the namespace part of a fully-qualified name.
    /// E.g. `Nat.add` → `Nat`.
    #[allow(dead_code)]
    pub fn namespace_of(name: &str) -> &str {
        if let Some(pos) = name.rfind('.') {
            &name[..pos]
        } else {
            ""
        }
    }
    /// Check if two names are in the same namespace.
    #[allow(dead_code)]
    pub fn same_namespace(a: &str, b: &str) -> bool {
        Self::namespace_of(a) == Self::namespace_of(b)
    }
    /// Compute the relative name of `name` from namespace `ns`.
    #[allow(dead_code)]
    pub fn relative_name<'a>(name: &'a str, ns: &str) -> &'a str {
        let prefix = if ns.is_empty() {
            String::new()
        } else {
            format!("{}.", ns)
        };
        if name.starts_with(&prefix) {
            &name[prefix.len()..]
        } else {
            name
        }
    }
}
