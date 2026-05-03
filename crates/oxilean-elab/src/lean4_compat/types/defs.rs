//! Type definitions for lean4_compat

use super::super::functions::*;
use oxilean_kernel::*;

/// How well OxiLean supports a given Lean 4 feature.
use oxilean_kernel::*;
#[derive(Debug, Clone, PartialEq)]
pub enum CompatLevel {
    /// Fully supported.
    Full,
    /// Partially supported; the string describes the gap.
    Partial(String),
    /// Present as a stub / placeholder only.
    Stub,
    /// Not supported at all.
    Unsupported,
}

/// A matrix mapping each `Lean4Feature` to an OxiLean `CompatLevel`.
pub struct Lean4CompatMatrix {
    pub(crate) entries: Vec<(Lean4Feature, CompatLevel)>,
}

/// A parsed Lean 4 attribute.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lean4Attribute {
    /// Attribute name, e.g. `simp`, `instance`, `inline`.
    pub name: String,
    /// Optional arguments, e.g. `[Nat.add_comm]`.
    pub args: Vec<String>,
}

/// A single Lean 4 token with its text and span.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Lean4Token {
    /// The token's syntactic category.
    pub kind: Lean4TokenKind,
    /// The exact source text.
    pub text: String,
    /// Byte offset in source.
    pub offset: usize,
    /// Line number (1-based).
    pub line: u32,
    /// Column (0-based).
    pub col: u32,
}

/// Describes a Lean 4 structure or class.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Lean4StructureDescriptor {
    /// Structure name.
    pub name: String,
    /// Parent structure (single inheritance).
    pub parent: Option<String>,
    /// Fields.
    pub fields: Vec<Lean4FieldDescriptor>,
    /// Whether this is a class (typeclass).
    pub is_class: bool,
    /// Universe parameters.
    pub universe_params: Vec<String>,
}

/// A single constructor of an inductive type.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Lean4Constructor {
    /// Constructor name.
    pub name: String,
    /// Argument types.
    pub arg_types: Vec<String>,
    /// Documentation.
    pub doc: String,
}

/// Tracks the current namespace stack during source analysis.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Lean4NamespaceTracker {
    pub(crate) stack: Vec<String>,
}

/// Describes a Lean 4 inductive type.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Lean4InductiveDescriptor {
    /// Type name.
    pub name: String,
    /// Type parameters.
    pub params: Vec<(String, String)>,
    /// Result sort.
    pub sort: String,
    /// Constructors.
    pub constructors: Vec<Lean4Constructor>,
    /// Universe parameters.
    pub universe_params: Vec<String>,
    /// Whether it is a `structure` (single-constructor inductive).
    pub is_structure: bool,
}

/// Normalizes universe-polymorphic Lean 4 syntax.
#[allow(dead_code)]
pub struct Lean4UniverseNormalizer;

impl CompatLevel {
    /// Returns true if the feature is at least partially usable.
    pub fn is_any_support(&self) -> bool {
        !matches!(self, CompatLevel::Unsupported | CompatLevel::Stub)
    }
}

impl Lean4CompatMatrix {
    /// Creates a new compatibility matrix with the known support levels.
    pub fn new() -> Self {
        let entries = vec![
            (Lean4Feature::DoNotation, CompatLevel::Full),
            (
                Lean4Feature::MacroExpansion,
                CompatLevel::Partial("hygienic macros not yet supported".to_string()),
            ),
            (
                Lean4Feature::AutoBoundImplicits,
                CompatLevel::Partial("single-level auto-bind only".to_string()),
            ),
            (
                Lean4Feature::StructureInheritance,
                CompatLevel::Partial("single extends only".to_string()),
            ),
            (Lean4Feature::DeclarationAttributes, CompatLevel::Full),
            (
                Lean4Feature::UniversePolymorphism,
                CompatLevel::Partial("universe variables erased during elaboration".to_string()),
            ),
            (Lean4Feature::TacticMode, CompatLevel::Full),
            (Lean4Feature::MetaProgramming, CompatLevel::Stub),
            (Lean4Feature::PatternMatching, CompatLevel::Full),
            (Lean4Feature::MutualRecursion, CompatLevel::Full),
            (
                Lean4Feature::WhereBindings,
                CompatLevel::Partial("only top-level where supported".to_string()),
            ),
            (
                Lean4Feature::Notation,
                CompatLevel::Partial("basic fixity notation only".to_string()),
            ),
        ];
        Self { entries }
    }
    /// Returns the compatibility level for the given feature.
    pub fn compat_level(&self, feature: &Lean4Feature) -> CompatLevel {
        self.entries
            .iter()
            .find(|(f, _)| f == feature)
            .map(|(_, l)| l.clone())
            .unwrap_or(CompatLevel::Unsupported)
    }
    /// Returns true if the feature has at least partial support.
    pub fn is_supported(&self, feature: &Lean4Feature) -> bool {
        self.compat_level(feature).is_any_support()
    }
    /// Returns true if the feature is `Partial`.
    pub fn partially_supported(&self, feature: &Lean4Feature) -> bool {
        matches!(self.compat_level(feature), CompatLevel::Partial(_))
    }
    /// Returns references to features with `Unsupported` or `Stub` compat level.
    pub fn unsupported_features(&self) -> Vec<&Lean4Feature> {
        self.entries
            .iter()
            .filter(|(_, l)| matches!(l, CompatLevel::Unsupported | CompatLevel::Stub))
            .map(|(f, _)| f)
            .collect()
    }
    /// Returns references to features with `Full` compat level.
    pub fn full_supported_features(&self) -> Vec<&Lean4Feature> {
        self.entries
            .iter()
            .filter(|(_, l)| matches!(l, CompatLevel::Full))
            .map(|(f, _)| f)
            .collect()
    }
    /// Formats a human-readable report of all features and their support levels.
    pub fn report(&self) -> String {
        let mut lines = vec!["Lean 4 Compatibility Matrix:".to_string()];
        for (feature, level) in &self.entries {
            let level_str = match level {
                CompatLevel::Full => "Full".to_string(),
                CompatLevel::Partial(msg) => format!("Partial ({})", msg),
                CompatLevel::Stub => "Stub".to_string(),
                CompatLevel::Unsupported => "Unsupported".to_string(),
            };
            lines.push(format!("  {:<25} {}", feature.label(), level_str));
        }
        lines.join("\n")
    }
}

impl Lean4CompatMatrix {
    /// Returns true if all features are at least partially supported.
    #[allow(dead_code)]
    pub fn all_at_least_partial(&self) -> bool {
        self.entries.iter().all(|(_, l)| l.is_any_support())
    }
    /// Returns the count of features with a given support level type.
    #[allow(dead_code)]
    pub fn count_full(&self) -> usize {
        self.entries
            .iter()
            .filter(|(_, l)| matches!(l, CompatLevel::Full))
            .count()
    }
    /// Returns the count of partially supported features.
    #[allow(dead_code)]
    pub fn count_partial(&self) -> usize {
        self.entries
            .iter()
            .filter(|(_, l)| matches!(l, CompatLevel::Partial(_)))
            .count()
    }
    /// Returns the count of stub features.
    #[allow(dead_code)]
    pub fn count_stub(&self) -> usize {
        self.entries
            .iter()
            .filter(|(_, l)| matches!(l, CompatLevel::Stub))
            .count()
    }
    /// Returns the count of unsupported features.
    #[allow(dead_code)]
    pub fn count_unsupported(&self) -> usize {
        self.entries
            .iter()
            .filter(|(_, l)| matches!(l, CompatLevel::Unsupported))
            .count()
    }
    /// Returns a summary line like "Full: 4, Partial: 6, Stub: 1, Unsupported: 1".
    #[allow(dead_code)]
    pub fn summary_line(&self) -> String {
        format!(
            "Full: {}, Partial: {}, Stub: {}, Unsupported: {}",
            self.count_full(),
            self.count_partial(),
            self.count_stub(),
            self.count_unsupported(),
        )
    }
    /// Override the compat level for a feature.
    #[allow(dead_code)]
    pub fn set_level(&mut self, feature: Lean4Feature, level: CompatLevel) {
        if let Some(entry) = self.entries.iter_mut().find(|(f, _)| f == &feature) {
            entry.1 = level;
        } else {
            self.entries.push((feature, level));
        }
    }
    /// Returns an iterator over (feature, level) entries.
    #[allow(dead_code)]
    pub fn iter(&self) -> impl Iterator<Item = &(Lean4Feature, CompatLevel)> {
        self.entries.iter()
    }
}

impl Lean4Attribute {
    /// Create a new attribute with no arguments.
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Lean4Attribute {
            name: name.to_string(),
            args: Vec::new(),
        }
    }
    /// Create an attribute with arguments.
    #[allow(dead_code)]
    pub fn with_args(name: &str, args: Vec<&str>) -> Self {
        Lean4Attribute {
            name: name.to_string(),
            args: args.iter().map(|s| s.to_string()).collect(),
        }
    }
    /// Format as `@[name arg1 arg2]`.
    #[allow(dead_code)]
    pub fn format(&self) -> String {
        if self.args.is_empty() {
            format!("@[{}]", self.name)
        } else {
            format!("@[{} {}]", self.name, self.args.join(" "))
        }
    }
    /// Returns true if this is a simp attribute.
    #[allow(dead_code)]
    pub fn is_simp(&self) -> bool {
        self.name == "simp"
    }
    /// Returns true if this is an instance attribute.
    #[allow(dead_code)]
    pub fn is_instance(&self) -> bool {
        self.name == "instance"
    }
    /// Returns true if this is a reducibility attribute.
    #[allow(dead_code)]
    pub fn is_reducibility(&self) -> bool {
        matches!(
            self.name.as_str(),
            "reducible" | "semireducible" | "irreducible"
        )
    }
}

impl Lean4Token {
    /// Create a new token.
    #[allow(dead_code)]
    pub fn new(kind: Lean4TokenKind, text: &str, offset: usize, line: u32, col: u32) -> Self {
        Lean4Token {
            kind,
            text: text.to_string(),
            offset,
            line,
            col,
        }
    }
    /// Returns true if this is an identifier token.
    #[allow(dead_code)]
    pub fn is_ident(&self) -> bool {
        self.kind == Lean4TokenKind::Ident
    }
    /// Returns true if this is a keyword token.
    #[allow(dead_code)]
    pub fn is_keyword(&self) -> bool {
        self.kind == Lean4TokenKind::Keyword
    }
    /// Returns true if this is end-of-file.
    #[allow(dead_code)]
    pub fn is_eof(&self) -> bool {
        self.kind == Lean4TokenKind::Eof
    }
}

impl Lean4StructureDescriptor {
    /// Create a new structure descriptor.
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Lean4StructureDescriptor {
            name: name.to_string(),
            parent: None,
            fields: Vec::new(),
            is_class: false,
            universe_params: Vec::new(),
        }
    }
    /// Mark as a typeclass.
    #[allow(dead_code)]
    pub fn as_class(mut self) -> Self {
        self.is_class = true;
        self
    }
    /// Set the parent.
    #[allow(dead_code)]
    pub fn extends(mut self, parent: &str) -> Self {
        self.parent = Some(parent.to_string());
        self
    }
    /// Add a field.
    #[allow(dead_code)]
    pub fn add_field(mut self, field: Lean4FieldDescriptor) -> Self {
        self.fields.push(field);
        self
    }
    /// Add a universe parameter.
    #[allow(dead_code)]
    pub fn add_universe(mut self, u: &str) -> Self {
        self.universe_params.push(u.to_string());
        self
    }
    /// Format as a Lean 4 structure declaration.
    #[allow(dead_code)]
    pub fn format(&self) -> String {
        let kw = if self.is_class { "class" } else { "structure" };
        let mut out = format!("{} {} where\n", kw, self.name);
        if let Some(ref p) = self.parent {
            out = format!("{} {} extends {} where\n", kw, self.name, p);
        }
        for field in &self.fields {
            out.push_str(&format!("  {}\n", field.format()));
        }
        out
    }
    /// Returns the number of (non-inherited) own fields.
    #[allow(dead_code)]
    pub fn own_field_count(&self) -> usize {
        self.fields.iter().filter(|f| !f.inherited).count()
    }
}

impl Lean4Constructor {
    /// Create a new constructor.
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Lean4Constructor {
            name: name.to_string(),
            arg_types: Vec::new(),
            doc: String::new(),
        }
    }
    /// Add an argument type.
    #[allow(dead_code)]
    pub fn with_arg(mut self, arg: &str) -> Self {
        self.arg_types.push(arg.to_string());
        self
    }
    /// Add documentation.
    #[allow(dead_code)]
    pub fn with_doc(mut self, doc: &str) -> Self {
        self.doc = doc.to_string();
        self
    }
    /// Format as a constructor signature.
    #[allow(dead_code)]
    pub fn format(&self) -> String {
        if self.arg_types.is_empty() {
            format!("| {}", self.name)
        } else {
            format!("| {} : {} -> Self", self.name, self.arg_types.join(" -> "))
        }
    }
    /// Returns the arity (number of arguments).
    #[allow(dead_code)]
    pub fn arity(&self) -> usize {
        self.arg_types.len()
    }
}

impl Lean4NamespaceTracker {
    /// Create an empty tracker (at root namespace).
    #[allow(dead_code)]
    pub fn new() -> Self {
        Lean4NamespaceTracker { stack: Vec::new() }
    }
    /// Push a new namespace onto the stack.
    #[allow(dead_code)]
    pub fn push(&mut self, name: &str) {
        self.stack.push(name.to_string());
    }
    /// Pop the innermost namespace.
    #[allow(dead_code)]
    pub fn pop(&mut self) -> Option<String> {
        self.stack.pop()
    }
    /// Returns the fully-qualified current namespace.
    #[allow(dead_code)]
    pub fn current(&self) -> String {
        self.stack.join(".")
    }
    /// Resolve a name relative to the current namespace.
    #[allow(dead_code)]
    pub fn resolve(&self, name: &str) -> String {
        if self.stack.is_empty() {
            name.to_string()
        } else {
            format!("{}.{}", self.current(), name)
        }
    }
    /// Returns true if we are at the root namespace.
    #[allow(dead_code)]
    pub fn is_root(&self) -> bool {
        self.stack.is_empty()
    }
    /// Returns the depth of the namespace stack.
    #[allow(dead_code)]
    pub fn depth(&self) -> usize {
        self.stack.len()
    }
}

impl Lean4InductiveDescriptor {
    /// Create a new inductive descriptor.
    #[allow(dead_code)]
    pub fn new(name: &str, sort: &str) -> Self {
        Lean4InductiveDescriptor {
            name: name.to_string(),
            params: Vec::new(),
            sort: sort.to_string(),
            constructors: Vec::new(),
            universe_params: Vec::new(),
            is_structure: false,
        }
    }
    /// Add a type parameter.
    #[allow(dead_code)]
    pub fn with_param(mut self, name: &str, ty: &str) -> Self {
        self.params.push((name.to_string(), ty.to_string()));
        self
    }
    /// Add a constructor.
    #[allow(dead_code)]
    pub fn with_constructor(mut self, ctor: Lean4Constructor) -> Self {
        self.constructors.push(ctor);
        self
    }
    /// Mark as a structure.
    #[allow(dead_code)]
    pub fn as_structure(mut self) -> Self {
        self.is_structure = true;
        self
    }
    /// Returns the number of constructors.
    #[allow(dead_code)]
    pub fn constructor_count(&self) -> usize {
        self.constructors.len()
    }
    /// Format as a Lean 4 inductive declaration.
    #[allow(dead_code)]
    pub fn format(&self) -> String {
        let params = self
            .params
            .iter()
            .map(|(n, t)| format!("({n} : {t})"))
            .collect::<Vec<_>>()
            .join(" ");
        let header = if params.is_empty() {
            format!("inductive {} : {} where", self.name, self.sort)
        } else {
            format!("inductive {} {} : {} where", self.name, params, self.sort)
        };
        let ctors = self
            .constructors
            .iter()
            .map(|c| format!("  {}", c.format()))
            .collect::<Vec<_>>()
            .join("\n");
        format!("{}\n{}", header, ctors)
    }
}

impl Lean4UniverseNormalizer {
    /// Strip `.{u}`, `.{u, v}`, `.{u v}` universe annotations.
    #[allow(dead_code)]
    pub fn strip_universe_annotations(src: &str) -> String {
        let mut result = String::with_capacity(src.len());
        let mut chars = src.chars().peekable();
        while let Some(ch) = chars.next() {
            if ch == '.' && chars.peek() == Some(&'{') {
                chars.next();
                let mut depth = 1usize;
                for c in chars.by_ref() {
                    match c {
                        '{' => depth += 1,
                        '}' => {
                            depth -= 1;
                            if depth == 0 {
                                break;
                            }
                        }
                        _ => {}
                    }
                }
            } else {
                result.push(ch);
            }
        }
        result
    }
    /// Normalize `Sort*` and `Type*` to `Type`.
    #[allow(dead_code)]
    pub fn normalize_sort_star(src: &str) -> String {
        src.replace("Sort*", "Type").replace("Type*", "Type")
    }
    /// Strip universe variable declarations: `universe u v w`.
    #[allow(dead_code)]
    pub fn strip_universe_decls(src: &str) -> String {
        let mut out = String::new();
        for line in src.lines() {
            let trimmed = line.trim_start();
            if trimmed.starts_with("universe ") {
            } else {
                out.push_str(line);
                out.push('\n');
            }
        }
        if !src.ends_with('\n') {
            out.truncate(out.trim_end_matches('\n').len());
        }
        out
    }
    /// Apply all normalizations.
    #[allow(dead_code)]
    pub fn normalize_all(src: &str) -> String {
        let s = Self::strip_universe_decls(src);
        let s = Self::strip_universe_annotations(&s);
        Self::normalize_sort_star(&s)
    }
}
/// A named Boolean option for the Lean 4 elaborator.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Lean4Option {
    /// Option name as used in `set_option`.
    pub name: String,
    /// Current value.
    pub value: bool,
    /// Default value.
    pub default: bool,
    /// Brief description.
    pub description: String,
}
impl Lean4Option {
    /// Create a new Boolean option.
    #[allow(dead_code)]
    pub fn new(name: &str, default: bool, description: &str) -> Self {
        Lean4Option {
            name: name.to_string(),
            value: default,
            default,
            description: description.to_string(),
        }
    }
    /// Set the current value.
    #[allow(dead_code)]
    pub fn set(mut self, value: bool) -> Self {
        self.value = value;
        self
    }
    /// Returns true if the option is currently enabled.
    #[allow(dead_code)]
    pub fn is_enabled(&self) -> bool {
        self.value
    }
    /// Returns true if the option is at its default value.
    #[allow(dead_code)]
    pub fn is_default(&self) -> bool {
        self.value == self.default
    }
    /// Format as a `set_option` command.
    #[allow(dead_code)]
    pub fn format_set_option(&self) -> String {
        format!("set_option {} {}", self.name, self.value)
    }
}
/// Maps between line/column positions and byte offsets in a source file.
#[allow(dead_code)]
pub struct Lean4PositionMapper {
    /// Byte offsets of the start of each line.
    line_starts: Vec<usize>,
}
impl Lean4PositionMapper {
    /// Build a mapper from source text.
    #[allow(dead_code)]
    pub fn new(src: &str) -> Self {
        let mut line_starts = vec![0usize];
        for (i, ch) in src.char_indices() {
            if ch == '\n' {
                line_starts.push(i + 1);
            }
        }
        Lean4PositionMapper { line_starts }
    }
    /// Convert a byte offset to (line, col) (both 1-based).
    #[allow(dead_code)]
    pub fn offset_to_line_col(&self, offset: usize) -> (u32, u32) {
        let line = self
            .line_starts
            .partition_point(|&start| start <= offset)
            .saturating_sub(1);
        let col = offset - self.line_starts[line];
        (line as u32 + 1, col as u32 + 1)
    }
    /// Convert (line, col) (1-based) to a byte offset.
    #[allow(dead_code)]
    pub fn line_col_to_offset(&self, line: u32, col: u32) -> usize {
        let line_idx = (line.saturating_sub(1)) as usize;
        let start = self.line_starts.get(line_idx).copied().unwrap_or(0);
        start + (col.saturating_sub(1)) as usize
    }
    /// Returns the number of lines in the source.
    #[allow(dead_code)]
    pub fn line_count(&self) -> usize {
        self.line_starts.len()
    }
}
/// Manages the stack of sections and namespaces during source analysis.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Lean4SectionManager {
    /// Stack entries: (kind, name).
    stack: Vec<(ScopeKind, String)>,
    /// Variables declared in each scope.
    scope_variables: Vec<Vec<(String, String)>>,
}
impl Lean4SectionManager {
    /// Create an empty section manager.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Lean4SectionManager {
            stack: Vec::new(),
            scope_variables: Vec::new(),
        }
    }
    /// Enter a namespace.
    #[allow(dead_code)]
    pub fn enter_namespace(&mut self, name: &str) {
        self.stack.push((ScopeKind::Namespace, name.to_string()));
        self.scope_variables.push(Vec::new());
    }
    /// Enter a section.
    #[allow(dead_code)]
    pub fn enter_section(&mut self, name: &str) {
        self.stack.push((ScopeKind::Section, name.to_string()));
        self.scope_variables.push(Vec::new());
    }
    /// Exit the current scope. Returns the scope kind and name.
    #[allow(dead_code)]
    pub fn exit(&mut self) -> Option<(ScopeKind, String)> {
        self.scope_variables.pop();
        self.stack.pop()
    }
    /// Add a variable to the current scope.
    #[allow(dead_code)]
    pub fn add_variable(&mut self, name: &str, ty: &str) {
        if let Some(vars) = self.scope_variables.last_mut() {
            vars.push((name.to_string(), ty.to_string()));
        }
    }
    /// Returns the current namespace path.
    #[allow(dead_code)]
    pub fn current_namespace(&self) -> String {
        self.stack
            .iter()
            .filter(|(k, _)| k == &ScopeKind::Namespace)
            .map(|(_, n)| n.as_str())
            .collect::<Vec<_>>()
            .join(".")
    }
    /// Returns the nesting depth.
    #[allow(dead_code)]
    pub fn depth(&self) -> usize {
        self.stack.len()
    }
    /// Returns all variables visible in the current scope.
    #[allow(dead_code)]
    pub fn visible_variables(&self) -> Vec<(String, String)> {
        self.scope_variables.iter().flatten().cloned().collect()
    }
}
/// Adapts Lean 4 surface syntax fragments to OxiLean conventions.
pub struct Lean4SyntaxAdapter;
impl Lean4SyntaxAdapter {
    /// Creates a new adapter.
    pub fn new() -> Self {
        Self
    }
    /// Converts `=>` used as function-body arrows to `->`.
    ///
    /// This is a simple textual replacement; it does not parse the input.
    /// Context-insensitive: also replaces `=>` inside strings/comments.
    pub fn adapt_arrow_syntax(src: &str) -> String {
        src.replace(" => ", " -> ")
    }
    /// Normalises `where` clauses by removing trailing semicolons after `where`.
    pub fn adapt_where_clause(src: &str) -> String {
        src.replace("where;", "where")
    }
    /// Normalises `do` block syntax: converts `←` bind arrows to `<-`.
    pub fn adapt_do_notation(src: &str) -> String {
        src.replace('←', "<-")
    }
    /// Ensures match arms use `->` rather than `=>`.
    pub fn adapt_match_syntax(src: &str) -> String {
        let mut result = String::with_capacity(src.len());
        let mut chars = src.chars().peekable();
        while let Some(ch) = chars.next() {
            if ch == '=' && chars.peek() == Some(&'>') {
                chars.next();
                result.push('-');
                result.push('>');
            } else {
                result.push(ch);
            }
        }
        result
    }
    /// Converts `fun x =>` lambda syntax to `fun x ->`.
    pub fn adapt_lambda(src: &str) -> String {
        src.replace(" => ", " -> ")
    }
    /// Applies all adaptations in sequence.
    pub fn adapt_all(src: &str) -> String {
        let s = Self::adapt_do_notation(src);
        let s = Self::adapt_where_clause(&s);
        Self::adapt_match_syntax(&s)
    }
}
impl Lean4SyntaxAdapter {
    /// Normalizes `#check` and `#print` commands (strips them).
    #[allow(dead_code)]
    pub fn strip_check_commands(src: &str) -> String {
        src.lines()
            .filter(|l| {
                let t = l.trim_start();
                !t.starts_with("#check") && !t.starts_with("#print") && !t.starts_with("#eval")
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
    /// Convert Lean 4 `by exact` shorthand to full form.
    #[allow(dead_code)]
    pub fn expand_by_exact(src: &str) -> String {
        src.replace(":= by exact ", ":= ")
    }
    /// Normalise `‹T›` anonymous constructor syntax to `(inferInstance : T)`.
    #[allow(dead_code)]
    pub fn adapt_angle_instance(src: &str) -> String {
        let mut result = String::with_capacity(src.len());
        let mut chars = src.chars().peekable();
        while let Some(ch) = chars.next() {
            if ch == '\u{2039}' {
                let mut inner = String::new();
                for c in chars.by_ref() {
                    if c == '\u{203a}' {
                        break;
                    }
                    inner.push(c);
                }
                result.push_str(&format!("(inferInstance : {})", inner));
            } else {
                result.push(ch);
            }
        }
        result
    }
    /// Strips section/namespace variable declarations that are not supported.
    #[allow(dead_code)]
    pub fn strip_variable_commands(src: &str) -> String {
        src.lines()
            .filter(|l| !l.trim_start().starts_with("variable "))
            .collect::<Vec<_>>()
            .join("\n")
    }
    /// Normalises `if h : P then t else e` (decidable if-then-else) to `if P then t else e`.
    #[allow(dead_code)]
    pub fn simplify_decidable_if(src: &str) -> String {
        let mut out = String::new();
        let mut remaining = src;
        while let Some(if_pos) = remaining.find("if ") {
            out.push_str(&remaining[..if_pos + 3]);
            remaining = &remaining[if_pos + 3..];
            if let Some(colon_pos) = remaining.find(':') {
                let before_colon = &remaining[..colon_pos];
                if before_colon
                    .trim()
                    .chars()
                    .all(|c| c.is_alphanumeric() || c == '_')
                {
                    remaining = remaining[colon_pos + 1..].trim_start();
                }
            }
        }
        out.push_str(remaining);
        out
    }
}
/// A collection of Lean 4 elaborator options.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Lean4OptionConfig {
    options: Vec<Lean4Option>,
}
impl Lean4OptionConfig {
    /// Create a new empty config.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Lean4OptionConfig {
            options: Vec::new(),
        }
    }
    /// Add an option.
    #[allow(dead_code, clippy::should_implement_trait)]
    pub fn add(mut self, opt: Lean4Option) -> Self {
        self.options.push(opt);
        self
    }
    /// Look up an option by name.
    #[allow(dead_code)]
    pub fn get(&self, name: &str) -> Option<&Lean4Option> {
        self.options.iter().find(|o| o.name == name)
    }
    /// Set an option value.
    #[allow(dead_code)]
    pub fn set_value(&mut self, name: &str, value: bool) {
        if let Some(opt) = self.options.iter_mut().find(|o| o.name == name) {
            opt.value = value;
        }
    }
    /// Build the default OxiLean option configuration.
    #[allow(dead_code)]
    pub fn defaults() -> Self {
        Lean4OptionConfig::new()
            .add(Lean4Option::new(
                "pp.all",
                false,
                "Print all implicit arguments.",
            ))
            .add(Lean4Option::new(
                "pp.unicode",
                true,
                "Use Unicode in pretty-printing.",
            ))
            .add(Lean4Option::new(
                "pp.funBinderTypes",
                true,
                "Show types in fun binders.",
            ))
            .add(Lean4Option::new(
                "pp.universes",
                false,
                "Show universe levels.",
            ))
            .add(Lean4Option::new(
                "pp.notation",
                true,
                "Use notation in output.",
            ))
            .add(Lean4Option::new(
                "pp.structure.proj",
                true,
                "Use dot-projection notation.",
            ))
            .add(Lean4Option::new(
                "trace.Elab",
                false,
                "Trace elaboration steps.",
            ))
            .add(Lean4Option::new(
                "trace.Meta.Tactic",
                false,
                "Trace tactic execution.",
            ))
    }
    /// Returns the number of options.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.options.len()
    }
    /// Returns true if empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.options.is_empty()
    }
    /// Format all non-default options as `set_option` commands.
    #[allow(dead_code)]
    pub fn format_non_defaults(&self) -> String {
        self.options
            .iter()
            .filter(|o| !o.is_default())
            .map(|o| o.format_set_option())
            .collect::<Vec<_>>()
            .join("\n")
    }
}
/// Describes a single field in a Lean 4 structure.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Lean4FieldDescriptor {
    /// Field name.
    pub name: String,
    /// Field type as a string.
    pub type_str: String,
    /// Optional default value.
    pub default: Option<String>,
    /// Visibility.
    pub visibility: FieldVisibility,
    /// True if this field is auto-generated from `extends`.
    pub inherited: bool,
}
impl Lean4FieldDescriptor {
    /// Create a new public field with no default.
    #[allow(dead_code)]
    pub fn new(name: &str, type_str: &str) -> Self {
        Lean4FieldDescriptor {
            name: name.to_string(),
            type_str: type_str.to_string(),
            default: None,
            visibility: FieldVisibility::Public,
            inherited: false,
        }
    }
    /// Set a default value.
    #[allow(dead_code)]
    pub fn with_default(mut self, val: &str) -> Self {
        self.default = Some(val.to_string());
        self
    }
    /// Mark as private.
    #[allow(dead_code)]
    pub fn private(mut self) -> Self {
        self.visibility = FieldVisibility::Private;
        self
    }
    /// Mark as inherited.
    #[allow(dead_code)]
    pub fn inherited(mut self) -> Self {
        self.inherited = true;
        self
    }
    /// Format as a Lean 4 field declaration.
    #[allow(dead_code)]
    pub fn format(&self) -> String {
        let mut out = String::new();
        if matches!(self.visibility, FieldVisibility::Private) {
            out.push_str("private ");
        }
        out.push_str(&format!("{} : {}", self.name, self.type_str));
        if let Some(ref d) = self.default {
            out.push_str(&format!(" := {}", d));
        }
        out
    }
}
/// Tracks the effect of `open Foo` commands.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Lean4OpenCommand {
    /// The namespace being opened.
    pub namespace: String,
    /// Optional subset: `open Foo (bar baz)`.
    pub names: Vec<String>,
    /// Whether this is a scoped open: `open scoped Foo`.
    pub scoped: bool,
}
impl Lean4OpenCommand {
    /// Create a full open command.
    #[allow(dead_code)]
    pub fn full(namespace: &str) -> Self {
        Lean4OpenCommand {
            namespace: namespace.to_string(),
            names: Vec::new(),
            scoped: false,
        }
    }
    /// Create a partial open command.
    #[allow(dead_code)]
    pub fn partial(namespace: &str, names: Vec<&str>) -> Self {
        Lean4OpenCommand {
            namespace: namespace.to_string(),
            names: names.iter().map(|s| s.to_string()).collect(),
            scoped: false,
        }
    }
    /// Create a scoped open command.
    #[allow(dead_code)]
    pub fn scoped(namespace: &str) -> Self {
        Lean4OpenCommand {
            namespace: namespace.to_string(),
            names: Vec::new(),
            scoped: true,
        }
    }
    /// Resolve a short name using this open command.
    /// Returns the fully-qualified name if the short name is exposed.
    #[allow(dead_code)]
    pub fn resolve(&self, short: &str) -> Option<String> {
        if self.names.is_empty() || self.names.contains(&short.to_string()) {
            Some(format!("{}.{}", self.namespace, short))
        } else {
            None
        }
    }
    /// Format as a Lean 4 open command.
    #[allow(dead_code)]
    pub fn format(&self) -> String {
        let prefix = if self.scoped { "open scoped " } else { "open " };
        if self.names.is_empty() {
            format!("{}{}", prefix, self.namespace)
        } else {
            format!("{}{} ({})", prefix, self.namespace, self.names.join(" "))
        }
    }
}

/// The kind of a scope entry.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScopeKind {
    Namespace,
    Section,
}

/// Features present in Lean 4, with varying levels of OxiLean support.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Lean4Feature {
    /// `do`-notation for monadic sequencing.
    DoNotation,
    /// Macro expansion (`macro_rules!`, `syntax`, `macro`).
    MacroExpansion,
    /// Automatic bound implicit variables.
    AutoBoundImplicits,
    /// Structure inheritance via `extends`.
    StructureInheritance,
    /// Declaration attributes (`@[simp]`, `@[instance]`, …).
    DeclarationAttributes,
    /// Universe-polymorphic definitions.
    UniversePolymorphism,
    /// `by tactic` proof blocks.
    TacticMode,
    /// `Lean.Elab.Tactic` meta-programming APIs.
    MetaProgramming,
    /// Full dependent pattern matching.
    PatternMatching,
    /// `mutual` recursive definitions.
    MutualRecursion,
    /// `where` local bindings in definitions.
    WhereBindings,
    /// User-defined `notation` declarations.
    Notation,
}

/// A simplified token kind for Lean 4 source analysis.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Lean4TokenKind {
    Ident,
    Keyword,
    IntLit,
    FloatLit,
    StringLit,
    CharLit,
    Arrow,
    FatArrow,
    Dot,
    Comma,
    Colon,
    ColonEq,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    At,
    Hash,
    Pipe,
    Backslash,
    Ampersand,
    Star,
    Plus,
    Minus,
    Slash,
    Percent,
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
    And,
    Or,
    Not,
    Eof,
    Unknown,
}

/// The visibility of a field or method.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldVisibility {
    Public,
    Protected,
    Private,
}

/// Represents a Lean 4 type annotation form.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Lean4TypeAnnotation {
    /// An explicit type ascription `(e : T)`.
    Ascription,
    /// An implicit binder `{x : T}`.
    Implicit,
    /// An instance binder `[inst : C]`.
    Instance,
    /// A strict implicit binder `{{x : T}}`.
    StrictImplicit,
    /// An auto-param `(x : T := default)`.
    AutoParam,
    /// An optional param `(x : T)?`.
    OptParam,
}
