//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::tokens::Span;

/// A cache for source map lookups.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct SourceMapCache {
    /// Cached lookups: (line, col) -> (orig_line, orig_col)
    pub cache: std::collections::HashMap<(usize, usize), (usize, usize)>,
}
impl SourceMapCache {
    /// Create a new empty cache.
    #[allow(dead_code)]
    pub fn new() -> Self {
        SourceMapCache {
            cache: std::collections::HashMap::new(),
        }
    }
    /// Insert a cached mapping.
    #[allow(dead_code)]
    pub fn insert(&mut self, gen_line: usize, gen_col: usize, orig_line: usize, orig_col: usize) {
        self.cache
            .insert((gen_line, gen_col), (orig_line, orig_col));
    }
    /// Look up a cached mapping.
    #[allow(dead_code)]
    pub fn lookup(&self, gen_line: usize, gen_col: usize) -> Option<(usize, usize)> {
        self.cache.get(&(gen_line, gen_col)).copied()
    }
    /// Returns the number of cached entries.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.cache.len()
    }
    /// Returns true if empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }
}
/// A source position with both byte offset and line/column information.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourcePosition {
    /// Byte offset from the start of the file.
    pub offset: usize,
    /// 1-based line number.
    pub line: usize,
    /// 1-based column number.
    pub column: usize,
}
#[allow(dead_code)]
impl SourcePosition {
    /// Create a source position.
    pub fn new(offset: usize, line: usize, column: usize) -> Self {
        Self {
            offset,
            line,
            column,
        }
    }
    /// The origin position (offset 0, line 1, col 1).
    pub fn origin() -> Self {
        Self {
            offset: 0,
            line: 1,
            column: 1,
        }
    }
    /// Advance by one character (non-newline).
    pub fn advance_col(self, bytes: usize) -> Self {
        Self {
            offset: self.offset + bytes,
            column: self.column + 1,
            ..self
        }
    }
    /// Advance to the next line.
    pub fn advance_line(self, bytes: usize) -> Self {
        Self {
            offset: self.offset + bytes,
            line: self.line + 1,
            column: 1,
        }
    }
    /// Check if this position is before another.
    pub fn is_before(&self, other: &SourcePosition) -> bool {
        self.offset < other.offset
    }
    /// Check if this position is after another.
    pub fn is_after(&self, other: &SourcePosition) -> bool {
        self.offset > other.offset
    }
}
/// Severity of an IDE diagnostic.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DiagnosticSeverity {
    /// A hint (informational).
    Hint,
    /// An informational message.
    Information,
    /// A warning.
    Warning,
    /// An error.
    Error,
}
/// A semantic token for IDE highlighting.
#[derive(Clone, Debug, PartialEq)]
pub struct SemanticToken {
    /// The source span of this token.
    pub span: Span,
    /// The token type.
    pub token_type: SemanticTokenType,
    /// Additional modifiers.
    pub modifiers: Vec<SemanticModifier>,
}
impl SemanticToken {
    /// Create a new semantic token without modifiers.
    #[allow(dead_code)]
    pub fn new(span: Span, token_type: SemanticTokenType) -> Self {
        Self {
            span,
            token_type,
            modifiers: Vec::new(),
        }
    }
    /// Create a semantic token with modifiers.
    #[allow(dead_code)]
    pub fn with_modifiers(
        span: Span,
        token_type: SemanticTokenType,
        modifiers: Vec<SemanticModifier>,
    ) -> Self {
        Self {
            span,
            token_type,
            modifiers,
        }
    }
}
/// A batch of source map entries.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Default)]
pub struct SourceMapBatch {
    /// All entries
    pub entries: Vec<RangeTransform>,
}
impl SourceMapBatch {
    /// Create a new batch.
    #[allow(dead_code)]
    pub fn new() -> Self {
        SourceMapBatch {
            entries: Vec::new(),
        }
    }
    /// Add an entry.
    #[allow(dead_code)]
    pub fn add(&mut self, e: RangeTransform) {
        self.entries.push(e);
    }
    /// Returns the total coverage (sum of original ranges).
    #[allow(dead_code)]
    pub fn total_coverage(&self) -> usize {
        self.entries.iter().map(|e| e.orig_len()).sum()
    }
    /// Returns the number of length-preserving entries.
    #[allow(dead_code)]
    pub fn length_preserving_count(&self) -> usize {
        self.entries
            .iter()
            .filter(|e| e.is_length_preserving())
            .count()
    }
}
/// A symbol in the document symbol outline (for IDE sidebar).
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct DocumentSymbol {
    /// The name of the symbol.
    pub name: String,
    /// The kind of symbol.
    pub kind: SymbolKind,
    /// The span of the entire declaration.
    pub full_span: Span,
    /// The span of just the name.
    pub name_span: Span,
    /// Child symbols (e.g. structure fields, inductive constructors).
    pub children: Vec<DocumentSymbol>,
}
#[allow(dead_code)]
impl DocumentSymbol {
    /// Create a new document symbol.
    pub fn new(name: String, kind: SymbolKind, full_span: Span, name_span: Span) -> Self {
        Self {
            name,
            kind,
            full_span,
            name_span,
            children: Vec::new(),
        }
    }
    /// Add a child symbol.
    pub fn add_child(&mut self, child: DocumentSymbol) {
        self.children.push(child);
    }
    /// Whether this symbol has children.
    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }
    /// Recursively find a symbol by name.
    pub fn find_by_name(&self, name: &str) -> Option<&DocumentSymbol> {
        if self.name == name {
            return Some(self);
        }
        for child in &self.children {
            if let Some(found) = child.find_by_name(name) {
                return Some(found);
            }
        }
        None
    }
    /// Flatten into a list (self first, then children recursively).
    pub fn flatten(&self) -> Vec<&DocumentSymbol> {
        let mut result = vec![self];
        for child in &self.children {
            result.extend(child.flatten());
        }
        result
    }
}
/// A labeled region of source text (e.g. a function body, an import block).
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct SourceRegion {
    /// Label for the region.
    pub label: String,
    /// Span of the region.
    pub span: Span,
    /// Optional metadata string.
    pub meta: Option<String>,
}
#[allow(dead_code)]
impl SourceRegion {
    /// Create a source region.
    pub fn new(label: impl Into<String>, span: Span) -> Self {
        Self {
            label: label.into(),
            span,
            meta: None,
        }
    }
    /// Create a source region with metadata.
    pub fn with_meta(label: impl Into<String>, span: Span, meta: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            span,
            meta: Some(meta.into()),
        }
    }
    /// Whether this region contains a byte offset.
    pub fn contains_offset(&self, offset: usize) -> bool {
        offset >= self.span.start && offset < self.span.end
    }
    /// The byte length of the region.
    pub fn byte_len(&self) -> usize {
        self.span.end.saturating_sub(self.span.start)
    }
}
/// Modifier flags for semantic tokens.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SemanticModifier {
    /// This token is a definition site.
    Definition,
    /// This token is a declaration.
    Declaration,
    /// This token is deprecated.
    Deprecated,
    /// This token is inside documentation.
    Documentation,
}
/// Semantic token type for syntax highlighting.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SemanticTokenType {
    /// A keyword (e.g. `def`, `theorem`, `if`).
    Keyword,
    /// A variable reference.
    Variable,
    /// A function name.
    Function,
    /// A type name.
    Type,
    /// A constructor name.
    Constructor,
    /// An operator symbol.
    Operator,
    /// A number literal.
    Number,
    /// A string literal.
    StringLit,
    /// A comment.
    Comment,
    /// A tactic name.
    Tactic,
    /// A namespace name.
    Namespace,
}
/// A single entry in the source map.
#[derive(Clone, Debug, PartialEq)]
pub struct SourceEntry {
    /// Source span for this entry.
    pub span: Span,
    /// What kind of entry this is.
    pub kind: EntryKind,
    /// Optional name associated with this entry.
    pub name: Option<String>,
    /// Optional type information as a display string.
    pub ty_info: Option<String>,
}
impl SourceEntry {
    /// Create a new source entry.
    #[allow(dead_code)]
    pub fn new(span: Span, kind: EntryKind) -> Self {
        Self {
            span,
            kind,
            name: None,
            ty_info: None,
        }
    }
    /// Create a source entry with a name.
    #[allow(dead_code)]
    pub fn with_name(span: Span, kind: EntryKind, name: &str) -> Self {
        Self {
            span,
            kind,
            name: Some(name.to_string()),
            ty_info: None,
        }
    }
    /// Create a source entry with name and type info.
    #[allow(dead_code)]
    pub fn with_name_and_type(span: Span, kind: EntryKind, name: &str, ty: &str) -> Self {
        Self {
            span,
            kind,
            name: Some(name.to_string()),
            ty_info: Some(ty.to_string()),
        }
    }
    /// Check whether a byte offset falls within this entry's span.
    #[allow(dead_code)]
    pub fn contains_offset(&self, offset: usize) -> bool {
        offset >= self.span.start && offset < self.span.end
    }
}
/// The kind of source entry.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EntryKind {
    /// A definition site (def, theorem, axiom, etc.)
    Definition,
    /// A reference to a previously defined name.
    Reference,
    /// A type annotation expression.
    TypeAnnotation,
    /// A binder (lambda, forall, let parameter).
    Binder,
    /// A constructor of an inductive type.
    Constructor,
    /// A pattern in a match expression.
    Pattern,
    /// A tactic invocation.
    Tactic,
    /// A keyword token.
    Keyword,
    /// A literal value (number, string, char).
    Literal,
    /// A comment (line or block).
    Comment,
    /// A documentation comment.
    DocComment,
    /// An operator symbol.
    Operator,
}
/// A diagnostic message with a source location.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct SourceDiagnostic {
    /// Span of the diagnostic.
    pub span: Span,
    /// Severity level.
    pub severity: DiagnosticSeverity,
    /// The diagnostic message.
    pub message: String,
    /// Optional code (e.g. "E001").
    pub code: Option<String>,
    /// Optional related spans (e.g. for "note" entries).
    pub related: Vec<(Span, String)>,
}
#[allow(dead_code)]
impl SourceDiagnostic {
    /// Create a new diagnostic.
    pub fn new(span: Span, severity: DiagnosticSeverity, message: impl Into<String>) -> Self {
        Self {
            span,
            severity,
            message: message.into(),
            code: None,
            related: Vec::new(),
        }
    }
    /// Create an error diagnostic.
    pub fn error(span: Span, message: impl Into<String>) -> Self {
        Self::new(span, DiagnosticSeverity::Error, message)
    }
    /// Create a warning diagnostic.
    pub fn warning(span: Span, message: impl Into<String>) -> Self {
        Self::new(span, DiagnosticSeverity::Warning, message)
    }
    /// Create an informational diagnostic.
    pub fn info(span: Span, message: impl Into<String>) -> Self {
        Self::new(span, DiagnosticSeverity::Information, message)
    }
    /// Set the error code.
    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }
    /// Add a related location.
    pub fn with_related(mut self, span: Span, message: impl Into<String>) -> Self {
        self.related.push((span, message.into()));
        self
    }
    /// Whether this is an error.
    pub fn is_error(&self) -> bool {
        self.severity == DiagnosticSeverity::Error
    }
    /// Whether this is a warning.
    pub fn is_warning(&self) -> bool {
        self.severity == DiagnosticSeverity::Warning
    }
}
/// The result of a go-to-definition lookup.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct GoToDefinitionResult {
    /// Name that was queried.
    pub name: String,
    /// The span(s) of the definition site.
    pub definition_spans: Vec<Span>,
    /// Whether the definition is in the same file.
    pub is_local: bool,
}
#[allow(dead_code)]
impl GoToDefinitionResult {
    /// Create a result.
    pub fn new(name: String, spans: Vec<Span>, is_local: bool) -> Self {
        Self {
            name,
            definition_spans: spans,
            is_local,
        }
    }
    /// Whether the definition was found.
    pub fn found(&self) -> bool {
        !self.definition_spans.is_empty()
    }
    /// The primary (first) definition span.
    pub fn primary_span(&self) -> Option<&Span> {
        self.definition_spans.first()
    }
}
/// Statistics about a source map.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct SourceMapStats {
    /// Total number of entries.
    pub total_entries: usize,
    /// Number of definition entries.
    pub definitions: usize,
    /// Number of reference entries.
    pub references: usize,
    /// Number of tactic entries.
    pub tactics: usize,
    /// Number of comment entries.
    pub comments: usize,
    /// Number of operator entries.
    pub operators: usize,
}
#[allow(dead_code)]
impl SourceMapStats {
    /// Compute stats from a list of source entries.
    pub fn from_entries(entries: &[SourceEntry]) -> Self {
        let mut definitions = 0usize;
        let mut references = 0usize;
        let mut tactics = 0usize;
        let mut comments = 0usize;
        let mut operators = 0usize;
        for entry in entries {
            match entry.kind {
                EntryKind::Definition => definitions += 1,
                EntryKind::Reference => references += 1,
                EntryKind::Tactic => tactics += 1,
                EntryKind::Comment | EntryKind::DocComment => comments += 1,
                EntryKind::Operator => operators += 1,
                _ => {}
            }
        }
        Self {
            total_entries: entries.len(),
            definitions,
            references,
            tactics,
            comments,
            operators,
        }
    }
}
/// A combined source index with definitions, references, and symbols.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct SourceIndex {
    /// Definition index.
    pub definitions: DefinitionIndex,
    /// Reference index.
    pub references: ReferenceIndex,
    /// Document symbols.
    pub symbols: Vec<DocumentSymbol>,
    /// Source regions.
    pub regions: Vec<SourceRegion>,
}
#[allow(dead_code)]
impl SourceIndex {
    /// Create an empty source index.
    pub fn new() -> Self {
        Self::default()
    }
    /// Register a definition.
    pub fn register_definition(&mut self, name: impl Into<String>, span: Span) {
        self.definitions.register(name, span);
    }
    /// Record a reference.
    pub fn record_reference(&mut self, name: impl Into<String>, span: Span) {
        self.references.record(name, span);
    }
    /// Add a document symbol.
    pub fn add_symbol(&mut self, symbol: DocumentSymbol) {
        self.symbols.push(symbol);
    }
    /// Add a source region.
    pub fn add_region(&mut self, region: SourceRegion) {
        self.regions.push(region);
    }
    /// Find the symbol at a given byte offset.
    pub fn symbol_at_offset(&self, offset: usize) -> Option<&DocumentSymbol> {
        self.symbols
            .iter()
            .find(|&sym| sym.full_span.start <= offset && offset < sym.full_span.end)
    }
    /// Get all definition spans for a name.
    pub fn definition_spans(&self, name: &str) -> &[Span] {
        self.definitions.lookup(name)
    }
    /// Get all use-site spans for a name.
    pub fn reference_spans(&self, name: &str) -> &[Span] {
        self.references.uses_of(name)
    }
    /// Symbols that are top-level definitions (not inside a namespace).
    pub fn top_level_symbols(&self) -> &[DocumentSymbol] {
        &self.symbols
    }
}
/// An index mapping definition names to their source spans.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct DefinitionIndex {
    entries: std::collections::HashMap<String, Vec<Span>>,
}
#[allow(dead_code)]
impl DefinitionIndex {
    /// Create an empty definition index.
    pub fn new() -> Self {
        Self {
            entries: std::collections::HashMap::new(),
        }
    }
    /// Register a definition name with its span.
    pub fn register(&mut self, name: impl Into<String>, span: Span) {
        self.entries.entry(name.into()).or_default().push(span);
    }
    /// Look up all spans for a given name.
    pub fn lookup(&self, name: &str) -> &[Span] {
        self.entries.get(name).map(|v| v.as_slice()).unwrap_or(&[])
    }
    /// Check if a name is defined.
    pub fn contains(&self, name: &str) -> bool {
        self.entries.contains_key(name)
    }
    /// All defined names.
    pub fn names(&self) -> Vec<&str> {
        self.entries.keys().map(|s| s.as_str()).collect()
    }
    /// Number of unique definition names.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Whether the index is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Remove all definitions for a name.
    pub fn remove(&mut self, name: &str) -> Vec<Span> {
        self.entries.remove(name).unwrap_or_default()
    }
    /// Merge another index into this one.
    pub fn merge(&mut self, other: DefinitionIndex) {
        for (name, spans) in other.entries {
            self.entries.entry(name).or_default().extend(spans);
        }
    }
}
/// A bidirectional position mapper between two source representations.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct BidiMapper {
    /// Forward mappings (original offset -> generated offset)
    pub forward: Vec<(usize, usize)>,
    /// Backward mappings (generated offset -> original offset)
    pub backward: Vec<(usize, usize)>,
}
impl BidiMapper {
    /// Create a new empty mapper.
    #[allow(dead_code)]
    pub fn new() -> Self {
        BidiMapper {
            forward: Vec::new(),
            backward: Vec::new(),
        }
    }
    /// Add a mapping.
    #[allow(dead_code)]
    pub fn add(&mut self, orig: usize, gen: usize) {
        self.forward.push((orig, gen));
        self.backward.push((gen, orig));
    }
    /// Look up the generated offset for an original offset.
    #[allow(dead_code)]
    pub fn to_gen(&self, orig: usize) -> Option<usize> {
        self.forward
            .iter()
            .rev()
            .find(|(o, _)| *o <= orig)
            .map(|(_, g)| *g)
    }
    /// Look up the original offset for a generated offset.
    #[allow(dead_code)]
    pub fn to_orig(&self, gen: usize) -> Option<usize> {
        self.backward
            .iter()
            .rev()
            .find(|(g, _)| *g <= gen)
            .map(|(_, o)| *o)
    }
    /// Returns the number of mappings.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.forward.len()
    }
    /// Returns true if empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.forward.is_empty()
    }
}
/// A source range transformation record.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct RangeTransform {
    /// Original start offset
    pub orig_start: usize,
    /// Original end offset
    pub orig_end: usize,
    /// Generated start offset
    pub gen_start: usize,
    /// Generated end offset
    pub gen_end: usize,
}
impl RangeTransform {
    /// Create a new range transform.
    #[allow(dead_code)]
    pub fn new(orig_start: usize, orig_end: usize, gen_start: usize, gen_end: usize) -> Self {
        RangeTransform {
            orig_start,
            orig_end,
            gen_start,
            gen_end,
        }
    }
    /// Length of the original range.
    #[allow(dead_code)]
    pub fn orig_len(&self) -> usize {
        self.orig_end.saturating_sub(self.orig_start)
    }
    /// Length of the generated range.
    #[allow(dead_code)]
    pub fn gen_len(&self) -> usize {
        self.gen_end.saturating_sub(self.gen_start)
    }
    /// Whether the mapping is length-preserving.
    #[allow(dead_code)]
    pub fn is_length_preserving(&self) -> bool {
        self.orig_len() == self.gen_len()
    }
}
/// A sorted source map for binary search lookups.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct SortedSourceMap {
    /// Sorted (gen_offset, orig_offset) pairs
    pub pairs: Vec<(usize, usize)>,
}
impl SortedSourceMap {
    /// Create a new empty sorted source map.
    #[allow(dead_code)]
    pub fn new() -> Self {
        SortedSourceMap { pairs: Vec::new() }
    }
    /// Add a mapping.
    #[allow(dead_code)]
    pub fn add(&mut self, gen: usize, orig: usize) {
        self.pairs.push((gen, orig));
        self.pairs.sort_by_key(|(g, _)| *g);
    }
    /// Look up the original offset for a generated offset.
    #[allow(dead_code)]
    pub fn lookup(&self, gen: usize) -> Option<usize> {
        let idx = self.pairs.partition_point(|(g, _)| *g <= gen);
        if idx == 0 {
            return None;
        }
        Some(self.pairs[idx - 1].1)
    }
    /// Returns the number of pairs.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.pairs.len()
    }
    /// Returns true if empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.pairs.is_empty()
    }
}
/// A source map difference: what changed between two maps.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct SourceMapDiff {
    /// Number of added segments
    pub added: usize,
    /// Number of removed segments
    pub removed: usize,
    /// Number of modified segments
    pub modified: usize,
}
/// Source map: the main data structure mapping source positions to AST nodes.
///
/// Supports IDE features such as hover, go-to-definition, semantic
/// highlighting, and reference lookup.
pub struct SourceMap {
    /// All entries in the source map, sorted by span start.
    entries: Vec<SourceEntry>,
    /// Byte offset of each line start (index 0 = line 1).
    line_offsets: Vec<usize>,
    /// The original source text.
    source: String,
    /// Pre-computed semantic tokens.
    semantic_tokens: Vec<SemanticToken>,
}
impl SourceMap {
    /// Create a new, empty source map for the given source text.
    #[allow(dead_code)]
    pub fn new(source: &str) -> Self {
        let line_offsets = Self::compute_line_offsets(source);
        Self {
            entries: Vec::new(),
            line_offsets,
            source: source.to_string(),
            semantic_tokens: Vec::new(),
        }
    }
    /// Get the original source text.
    #[allow(dead_code)]
    pub fn source(&self) -> &str {
        &self.source
    }
    /// Get all entries.
    #[allow(dead_code)]
    pub fn entries(&self) -> &[SourceEntry] {
        &self.entries
    }
    /// Return the total number of lines in the source.
    #[allow(dead_code)]
    pub fn line_count(&self) -> usize {
        self.line_offsets.len()
    }
    /// Convert a byte offset to a (line, column) pair.
    ///
    /// Lines and columns are 1-indexed, matching the convention in `Span`.
    #[allow(dead_code)]
    pub fn offset_to_position(&self, offset: usize) -> (usize, usize) {
        let line_idx = match self.line_offsets.binary_search(&offset) {
            Ok(idx) => idx,
            Err(idx) => {
                if idx == 0 {
                    0
                } else {
                    idx - 1
                }
            }
        };
        let line_start = self.line_offsets[line_idx];
        let col = offset.saturating_sub(line_start);
        (line_idx + 1, col + 1)
    }
    /// Convert a (line, column) pair to a byte offset.
    ///
    /// Lines and columns are 1-indexed.
    #[allow(dead_code)]
    pub fn position_to_offset(&self, line: usize, col: usize) -> usize {
        if line == 0 || line > self.line_offsets.len() {
            return self.source.len();
        }
        let line_start = self.line_offsets[line - 1];
        let col_offset = if col > 0 { col - 1 } else { 0 };
        let max_col = self.line_length(line);
        line_start + col_offset.min(max_col)
    }
    /// Get the text content of a specific line (1-indexed).
    #[allow(dead_code)]
    pub fn line_content(&self, line: usize) -> &str {
        if line == 0 || line > self.line_offsets.len() {
            return "";
        }
        let start = self.line_offsets[line - 1];
        let end = if line < self.line_offsets.len() {
            self.line_offsets[line]
        } else {
            self.source.len()
        };
        let text = &self.source[start..end];
        text.trim_end_matches('\n').trim_end_matches('\r')
    }
    /// Get the length (in bytes) of a specific line (1-indexed), excluding newline.
    #[allow(dead_code)]
    fn line_length(&self, line: usize) -> usize {
        self.line_content(line).len()
    }
    /// Find the entry at a given (line, column) position (1-indexed).
    #[allow(dead_code)]
    pub fn entry_at(&self, line: usize, col: usize) -> Option<&SourceEntry> {
        let offset = self.position_to_offset(line, col);
        let mut best: Option<&SourceEntry> = None;
        for entry in &self.entries {
            if entry.contains_offset(offset) {
                match best {
                    None => best = Some(entry),
                    Some(prev) => {
                        let prev_size = prev.span.end - prev.span.start;
                        let curr_size = entry.span.end - entry.span.start;
                        if curr_size < prev_size {
                            best = Some(entry);
                        }
                    }
                }
            }
        }
        best
    }
    /// Find all entries whose spans overlap with the given range.
    #[allow(dead_code)]
    pub fn entries_in_range(&self, start: &Span, end: &Span) -> Vec<&SourceEntry> {
        let range_start = start.start;
        let range_end = end.end;
        self.entries
            .iter()
            .filter(|e| e.span.start < range_end && e.span.end > range_start)
            .collect()
    }
    /// Get all definition entries.
    #[allow(dead_code)]
    pub fn definitions(&self) -> Vec<&SourceEntry> {
        self.entries
            .iter()
            .filter(|e| e.kind == EntryKind::Definition)
            .collect()
    }
    /// Get all references to a given name.
    #[allow(dead_code)]
    pub fn references_to(&self, name: &str) -> Vec<&SourceEntry> {
        self.entries
            .iter()
            .filter(|e| e.kind == EntryKind::Reference && e.name.as_deref() == Some(name))
            .collect()
    }
    /// Get all entries with the given kind.
    #[allow(dead_code)]
    pub fn entries_of_kind(&self, kind: &EntryKind) -> Vec<&SourceEntry> {
        self.entries.iter().filter(|e| &e.kind == kind).collect()
    }
    /// Produce hover information for a given (line, column) position.
    #[allow(dead_code)]
    pub fn hover_info(&self, line: usize, col: usize) -> Option<HoverInfo> {
        let entry = self.entry_at(line, col)?;
        let name = entry.name.clone()?;
        let definition_span = if entry.kind == EntryKind::Reference {
            self.entries
                .iter()
                .find(|e| e.kind == EntryKind::Definition && e.name.as_deref() == Some(&name))
                .map(|e| e.span.clone())
        } else if entry.kind == EntryKind::Definition {
            Some(entry.span.clone())
        } else {
            None
        };
        let doc = definition_span
            .as_ref()
            .and_then(|def_span| self.doc_for_definition(def_span.start));
        Some(HoverInfo {
            name,
            kind: entry.kind.clone(),
            ty: entry.ty_info.clone(),
            doc,
            definition_span,
        })
    }
    /// Find the doc comment that immediately precedes a definition at `def_start`.
    ///
    /// Searches for `DocComment` entries whose span ends at or before
    /// `def_start` and for which no *other* non-comment, non-keyword entry
    /// falls in between.  Returns the text of the closest such comment, or
    /// `None`.
    fn doc_for_definition(&self, def_start: usize) -> Option<String> {
        let best = self
            .entries
            .iter()
            .filter(|e| e.kind == EntryKind::DocComment && e.span.end <= def_start)
            .max_by_key(|e| e.span.end)?;
        let doc_end = best.span.end;
        let intervening_non_doc = self.entries.iter().any(|e| {
            e.span.start >= doc_end
                && e.span.start < def_start
                && !matches!(
                    e.kind,
                    EntryKind::DocComment | EntryKind::Comment | EntryKind::Keyword
                )
        });
        if intervening_non_doc {
            None
        } else {
            best.name.clone()
        }
    }
    /// Produce all semantic tokens for the source map.
    #[allow(dead_code)]
    pub fn semantic_tokens(&self) -> Vec<SemanticToken> {
        if !self.semantic_tokens.is_empty() {
            return self.semantic_tokens.clone();
        }
        self.entries
            .iter()
            .map(|entry| {
                let token_type = Self::entry_kind_to_token_type(&entry.kind);
                let modifiers = Self::entry_kind_to_modifiers(&entry.kind);
                SemanticToken {
                    span: entry.span.clone(),
                    token_type,
                    modifiers,
                }
            })
            .collect()
    }
    /// Get the text corresponding to a span.
    #[allow(dead_code)]
    pub fn span_text(&self, span: &Span) -> &str {
        let start = span.start.min(self.source.len());
        let end = span.end.min(self.source.len());
        &self.source[start..end]
    }
    /// Find the definition of a name and return its span.
    #[allow(dead_code)]
    pub fn go_to_definition(&self, line: usize, col: usize) -> Option<Span> {
        let entry = self.entry_at(line, col)?;
        let name = entry.name.as_deref()?;
        if entry.kind == EntryKind::Definition {
            return Some(entry.span.clone());
        }
        self.entries
            .iter()
            .find(|e| e.kind == EntryKind::Definition && e.name.as_deref() == Some(name))
            .map(|e| e.span.clone())
    }
    /// Find all occurrences (definitions + references) of a name at position.
    #[allow(dead_code)]
    pub fn find_all_occurrences(&self, line: usize, col: usize) -> Vec<&SourceEntry> {
        let entry = match self.entry_at(line, col) {
            Some(e) => e,
            None => return Vec::new(),
        };
        let name = match &entry.name {
            Some(n) => n.clone(),
            None => return Vec::new(),
        };
        self.entries
            .iter()
            .filter(|e| {
                e.name.as_deref() == Some(&name)
                    && matches!(e.kind, EntryKind::Definition | EntryKind::Reference)
            })
            .collect()
    }
    /// Compute line offsets for a source string.
    fn compute_line_offsets(source: &str) -> Vec<usize> {
        let mut offsets = vec![0usize];
        for (i, ch) in source.char_indices() {
            if ch == '\n' {
                offsets.push(i + 1);
            }
        }
        offsets
    }
    /// Map an `EntryKind` to a `SemanticTokenType`.
    fn entry_kind_to_token_type(kind: &EntryKind) -> SemanticTokenType {
        match kind {
            EntryKind::Definition => SemanticTokenType::Function,
            EntryKind::Reference => SemanticTokenType::Variable,
            EntryKind::TypeAnnotation => SemanticTokenType::Type,
            EntryKind::Binder => SemanticTokenType::Variable,
            EntryKind::Constructor => SemanticTokenType::Constructor,
            EntryKind::Pattern => SemanticTokenType::Variable,
            EntryKind::Tactic => SemanticTokenType::Tactic,
            EntryKind::Keyword => SemanticTokenType::Keyword,
            EntryKind::Literal => SemanticTokenType::Number,
            EntryKind::Comment => SemanticTokenType::Comment,
            EntryKind::DocComment => SemanticTokenType::Comment,
            EntryKind::Operator => SemanticTokenType::Operator,
        }
    }
    /// Map an `EntryKind` to semantic modifiers.
    fn entry_kind_to_modifiers(kind: &EntryKind) -> Vec<SemanticModifier> {
        match kind {
            EntryKind::Definition => vec![SemanticModifier::Definition],
            EntryKind::DocComment => vec![SemanticModifier::Documentation],
            _ => Vec::new(),
        }
    }
}
/// An index mapping name references to their use-site spans.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ReferenceIndex {
    entries: std::collections::HashMap<String, Vec<Span>>,
}
#[allow(dead_code)]
impl ReferenceIndex {
    /// Create an empty reference index.
    pub fn new() -> Self {
        Self {
            entries: std::collections::HashMap::new(),
        }
    }
    /// Record a reference to a name.
    pub fn record(&mut self, name: impl Into<String>, span: Span) {
        self.entries.entry(name.into()).or_default().push(span);
    }
    /// All use-sites for a given name.
    pub fn uses_of(&self, name: &str) -> &[Span] {
        self.entries.get(name).map(|v| v.as_slice()).unwrap_or(&[])
    }
    /// All names that are referenced.
    pub fn referenced_names(&self) -> Vec<&str> {
        self.entries.keys().map(|s| s.as_str()).collect()
    }
    /// Total number of reference entries.
    pub fn total_references(&self) -> usize {
        self.entries.values().map(|v| v.len()).sum()
    }
}
/// Builder for constructing a `SourceMap` during parsing.
pub struct SourceMapBuilder {
    /// Entries accumulated during parsing.
    entries: Vec<SourceEntry>,
    /// Semantic tokens accumulated during parsing.
    semantic_tokens: Vec<SemanticToken>,
    /// The original source text.
    source: String,
}
impl SourceMapBuilder {
    /// Create a new builder for the given source text.
    #[allow(dead_code)]
    pub fn new(source: &str) -> Self {
        Self {
            entries: Vec::new(),
            semantic_tokens: Vec::new(),
            source: source.to_string(),
        }
    }
    /// Add an arbitrary entry to the source map.
    #[allow(dead_code)]
    pub fn add_entry(&mut self, span: Span, kind: EntryKind, name: Option<String>) {
        self.entries.push(SourceEntry {
            span,
            kind,
            name,
            ty_info: None,
        });
    }
    /// Add an entry with type information.
    #[allow(dead_code)]
    pub fn add_entry_with_type(
        &mut self,
        span: Span,
        kind: EntryKind,
        name: Option<String>,
        ty_info: String,
    ) {
        self.entries.push(SourceEntry {
            span,
            kind,
            name,
            ty_info: Some(ty_info),
        });
    }
    /// Add a definition entry.
    #[allow(dead_code)]
    pub fn add_definition(&mut self, span: Span, name: &str) {
        self.entries.push(SourceEntry {
            span,
            kind: EntryKind::Definition,
            name: Some(name.to_string()),
            ty_info: None,
        });
    }
    /// Add a definition entry with type info.
    #[allow(dead_code)]
    pub fn add_definition_with_type(&mut self, span: Span, name: &str, ty_info: &str) {
        self.entries.push(SourceEntry {
            span,
            kind: EntryKind::Definition,
            name: Some(name.to_string()),
            ty_info: Some(ty_info.to_string()),
        });
    }
    /// Add a reference entry.
    #[allow(dead_code)]
    pub fn add_reference(&mut self, span: Span, name: &str) {
        self.entries.push(SourceEntry {
            span,
            kind: EntryKind::Reference,
            name: Some(name.to_string()),
            ty_info: None,
        });
    }
    /// Add a binder entry.
    #[allow(dead_code)]
    pub fn add_binder(&mut self, span: Span, name: &str) {
        self.entries.push(SourceEntry {
            span,
            kind: EntryKind::Binder,
            name: Some(name.to_string()),
            ty_info: None,
        });
    }
    /// Add a constructor entry.
    #[allow(dead_code)]
    pub fn add_constructor(&mut self, span: Span, name: &str) {
        self.entries.push(SourceEntry {
            span,
            kind: EntryKind::Constructor,
            name: Some(name.to_string()),
            ty_info: None,
        });
    }
    /// Add a keyword entry.
    #[allow(dead_code)]
    pub fn add_keyword(&mut self, span: Span) {
        self.entries.push(SourceEntry {
            span,
            kind: EntryKind::Keyword,
            name: None,
            ty_info: None,
        });
    }
    /// Add a literal entry.
    #[allow(dead_code)]
    pub fn add_literal(&mut self, span: Span) {
        self.entries.push(SourceEntry {
            span,
            kind: EntryKind::Literal,
            name: None,
            ty_info: None,
        });
    }
    /// Add an operator entry.
    #[allow(dead_code)]
    pub fn add_operator(&mut self, span: Span, symbol: &str) {
        self.entries.push(SourceEntry {
            span,
            kind: EntryKind::Operator,
            name: Some(symbol.to_string()),
            ty_info: None,
        });
    }
    /// Add a comment entry.
    #[allow(dead_code)]
    pub fn add_comment(&mut self, span: Span) {
        self.entries.push(SourceEntry {
            span,
            kind: EntryKind::Comment,
            name: None,
            ty_info: None,
        });
    }
    /// Add a doc comment entry.
    #[allow(dead_code)]
    pub fn add_doc_comment(&mut self, span: Span, text: &str) {
        self.entries.push(SourceEntry {
            span,
            kind: EntryKind::DocComment,
            name: Some(text.to_string()),
            ty_info: None,
        });
    }
    /// Add a tactic entry.
    #[allow(dead_code)]
    pub fn add_tactic(&mut self, span: Span, name: &str) {
        self.entries.push(SourceEntry {
            span,
            kind: EntryKind::Tactic,
            name: Some(name.to_string()),
            ty_info: None,
        });
    }
    /// Add a pattern entry.
    #[allow(dead_code)]
    pub fn add_pattern(&mut self, span: Span, name: Option<&str>) {
        self.entries.push(SourceEntry {
            span,
            kind: EntryKind::Pattern,
            name: name.map(|n| n.to_string()),
            ty_info: None,
        });
    }
    /// Add a type annotation entry.
    #[allow(dead_code)]
    pub fn add_type_annotation(&mut self, span: Span) {
        self.entries.push(SourceEntry {
            span,
            kind: EntryKind::TypeAnnotation,
            name: None,
            ty_info: None,
        });
    }
    /// Add a pre-computed semantic token.
    #[allow(dead_code)]
    pub fn add_semantic_token(&mut self, token: SemanticToken) {
        self.semantic_tokens.push(token);
    }
    /// Get the current number of entries.
    #[allow(dead_code)]
    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }
    /// Build the final `SourceMap`.
    ///
    /// Entries are sorted by span start offset for efficient lookup.
    #[allow(dead_code)]
    pub fn build(mut self) -> SourceMap {
        self.entries.sort_by(|a, b| {
            a.span.start.cmp(&b.span.start).then_with(|| {
                let a_size = a.span.end - a.span.start;
                let b_size = b.span.end - b.span.start;
                a_size.cmp(&b_size)
            })
        });
        self.semantic_tokens
            .sort_by(|a, b| a.span.start.cmp(&b.span.start));
        let line_offsets = SourceMap::compute_line_offsets(&self.source);
        SourceMap {
            entries: self.entries,
            line_offsets,
            source: self.source,
            semantic_tokens: self.semantic_tokens,
        }
    }
}
/// Hover information for IDE tooltips.
#[derive(Clone, Debug, PartialEq)]
pub struct HoverInfo {
    /// The name at the hover position.
    pub name: String,
    /// The kind of the entry.
    pub kind: EntryKind,
    /// The type of the entry (if known).
    pub ty: Option<String>,
    /// Documentation string (if available).
    pub doc: Option<String>,
    /// Span of the definition site (if known).
    pub definition_span: Option<Span>,
}
/// A source map chain that composes two mappings.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct MapChain {
    /// First map (A -> B)
    pub first: BidiMapper,
    /// Second map (B -> C)
    pub second: BidiMapper,
}
impl MapChain {
    /// Create a new chain.
    #[allow(dead_code)]
    pub fn new(first: BidiMapper, second: BidiMapper) -> Self {
        MapChain { first, second }
    }
    /// Map from A to C.
    #[allow(dead_code)]
    pub fn a_to_c(&self, a: usize) -> Option<usize> {
        let b = self.first.to_gen(a)?;
        self.second.to_gen(b)
    }
    /// Map from C to A.
    #[allow(dead_code)]
    pub fn c_to_a(&self, c: usize) -> Option<usize> {
        let b = self.second.to_orig(c)?;
        self.first.to_orig(b)
    }
}
/// The kind of a document symbol.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolKind {
    /// A theorem or lemma.
    Theorem,
    /// A definition.
    Definition,
    /// An axiom.
    Axiom,
    /// An inductive type.
    Inductive,
    /// A structure.
    Structure,
    /// A class.
    Class,
    /// An instance.
    Instance,
    /// A namespace.
    Namespace,
    /// A section.
    Section,
    /// A constructor.
    Constructor,
    /// A field.
    Field,
    /// A variable declaration.
    Variable,
}
