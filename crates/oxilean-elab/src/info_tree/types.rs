//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{Expr, Level, Name};
use std::collections::HashMap;

use super::functions::*;

/// A collection of semantic tokens covering a source file.
#[derive(Clone, Debug, Default)]
pub struct SemanticTokenList {
    tokens: Vec<SemanticToken>,
}
impl SemanticTokenList {
    /// Create an empty list.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a token.
    pub fn push(&mut self, token: SemanticToken) {
        self.tokens.push(token);
    }
    /// Sort tokens by start position.
    pub fn sort(&mut self) {
        self.tokens.sort_by_key(|t| t.range.0);
    }
    /// Return the number of tokens.
    pub fn len(&self) -> usize {
        self.tokens.len()
    }
    /// Return `true` if no tokens.
    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }
    /// Iterate over all tokens.
    pub fn iter(&self) -> impl Iterator<Item = &SemanticToken> {
        self.tokens.iter()
    }
    /// Return tokens of a specific kind.
    pub fn of_kind(&self, kind: SemanticTokenKind) -> Vec<&SemanticToken> {
        self.tokens.iter().filter(|t| t.kind == kind).collect()
    }
}
/// Information attached to an info tree node.
///
/// Contains the source range and the specific kind of information
/// recorded at this point in the elaboration.
#[derive(Clone, Debug)]
pub struct Info {
    /// The byte range in the source file: (start, end).
    pub stx_range: (usize, usize),
    /// The kind of information at this node.
    pub data: InfoData,
}
impl Info {
    /// Create a new info entry.
    pub fn new(start: usize, end: usize, data: InfoData) -> Self {
        Self {
            stx_range: (start, end),
            data,
        }
    }
    /// Check if a position falls within this info's range.
    pub fn contains_pos(&self, pos: usize) -> bool {
        pos >= self.stx_range.0 && pos < self.stx_range.1
    }
    /// Get the length of the source range.
    pub fn range_len(&self) -> usize {
        self.stx_range.1 - self.stx_range.0
    }
}
impl Info {
    #[allow(dead_code)]
    fn stx_range_copy(&self) -> (usize, usize) {
        self.stx_range
    }
}
/// Kind of completion item.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CompletionKind {
    /// A function or definition.
    Function,
    /// A type or type constructor.
    Type,
    /// A tactic.
    Tactic,
    /// A local variable.
    Variable,
    /// A namespace.
    Namespace,
    /// A keyword.
    Keyword,
    /// A field of a structure.
    Field,
    /// A constructor.
    Constructor,
    /// A theorem or lemma.
    Theorem,
    /// An attribute.
    Attribute,
    /// An option name.
    Option,
}
/// Builds an info tree during elaboration.
///
/// The builder maintains a stack of open nodes. As the elaborator
/// enters and exits subexpressions, it pushes and pops nodes on
/// the stack. When a node is popped, it becomes a child of its parent.
pub struct InfoTreeBuilder {
    /// Stack of open nodes. The top of the stack is the current node
    /// being elaborated. When it is completed, it becomes a child of
    /// the node below it.
    stack: Vec<BuilderFrame>,
    /// The completed root trees (for multi-command files).
    roots: Vec<InfoTree>,
    /// Whether info collection is enabled.
    enabled: bool,
    /// Accumulated local context entries.
    context_stack: Vec<Vec<LocalContextEntry>>,
    /// Documentation strings keyed by name.
    doc_strings: HashMap<Name, String>,
}
impl InfoTreeBuilder {
    /// Create a new info tree builder.
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            roots: Vec::new(),
            enabled: true,
            context_stack: vec![Vec::new()],
            doc_strings: HashMap::new(),
        }
    }
    /// Create a disabled builder (for performance when info is not needed).
    pub fn disabled() -> Self {
        Self {
            stack: Vec::new(),
            roots: Vec::new(),
            enabled: false,
            context_stack: vec![Vec::new()],
            doc_strings: HashMap::new(),
        }
    }
    /// Check if the builder is enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    /// Enable or disable info collection.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    /// Push a new node onto the stack.
    pub fn push_info(&mut self, start: usize, end: usize, data: InfoData) {
        if !self.enabled {
            return;
        }
        let info = Info::new(start, end, data);
        self.stack.push(BuilderFrame::new_node(info));
    }
    /// Push a term info node.
    pub fn push_term_info(&mut self, start: usize, end: usize, expr: Expr, type_: Expr) {
        self.push_info(start, end, InfoData::TermInfo { expr, type_ });
    }
    /// Push a field info node.
    pub fn push_field_info(
        &mut self,
        start: usize,
        end: usize,
        struct_name: Name,
        field_name: Name,
        field_type: Option<Expr>,
    ) {
        self.push_info(
            start,
            end,
            InfoData::FieldInfo {
                struct_name,
                field_name,
                field_type,
            },
        );
    }
    /// Push a tactic info node.
    pub fn push_tactic_info(
        &mut self,
        start: usize,
        end: usize,
        state_before: TacticStateInfo,
        state_after: TacticStateInfo,
    ) {
        self.push_info(
            start,
            end,
            InfoData::TacticInfo {
                state_before,
                state_after,
            },
        );
    }
    /// Push a command info node.
    pub fn push_command_info(
        &mut self,
        start: usize,
        end: usize,
        name: String,
        decl_name: Option<Name>,
    ) {
        self.push_info(start, end, InfoData::CommandInfo { name, decl_name });
    }
    /// Push a reference info node.
    pub fn push_ref_info(
        &mut self,
        start: usize,
        end: usize,
        name: Name,
        ty: Option<Expr>,
        doc: Option<String>,
    ) {
        self.push_info(start, end, InfoData::RefInfo { name, ty, doc });
    }
    /// Push a context frame onto the stack.
    pub fn push_context(&mut self, range: Option<(usize, usize)>, lctx: Vec<LocalContextEntry>) {
        if !self.enabled {
            return;
        }
        self.context_stack.push(lctx.clone());
        self.stack.push(BuilderFrame::new_context(range, lctx));
    }
    /// Pop the current frame from the stack, making it a child of its parent.
    ///
    /// If there is no parent, the frame becomes a root tree.
    pub fn pop(&mut self) -> Option<InfoTree> {
        if !self.enabled {
            return None;
        }
        let frame = self.stack.pop()?;
        let tree = if frame.is_context {
            self.context_stack.pop();
            InfoTree::Context {
                lctx: frame.lctx,
                children: frame.children,
                range: frame.range,
            }
        } else if let Some(info) = frame.info {
            InfoTree::Node {
                info,
                children: frame.children,
            }
        } else {
            return None;
        };
        if let Some(parent) = self.stack.last_mut() {
            parent.children.push(tree.clone());
        } else {
            self.roots.push(tree.clone());
        }
        Some(tree)
    }
    /// Pop all remaining frames and return the complete tree.
    pub fn finish(&mut self) -> Vec<InfoTree> {
        while !self.stack.is_empty() {
            self.pop();
        }
        std::mem::take(&mut self.roots)
    }
    /// Add a leaf info entry without push/pop (for simple annotations).
    pub fn add_leaf(&mut self, start: usize, end: usize, data: InfoData) {
        if !self.enabled {
            return;
        }
        let info = Info::new(start, end, data);
        let leaf = InfoTree::leaf(info);
        if let Some(parent) = self.stack.last_mut() {
            parent.children.push(leaf);
        } else {
            self.roots.push(leaf);
        }
    }
    /// Add a hole entry.
    pub fn add_hole(&mut self, expected_type: Option<Expr>, range: Option<(usize, usize)>) {
        if !self.enabled {
            return;
        }
        let hole = InfoTree::hole(expected_type, range);
        if let Some(parent) = self.stack.last_mut() {
            parent.children.push(hole);
        } else {
            self.roots.push(hole);
        }
    }
    /// Register a documentation string for a name.
    pub fn register_doc(&mut self, name: Name, doc: String) {
        self.doc_strings.insert(name, doc);
    }
    /// Look up a documentation string.
    pub fn get_doc(&self, name: &Name) -> Option<&String> {
        self.doc_strings.get(name)
    }
    /// Get the current local context.
    pub fn current_context(&self) -> &[LocalContextEntry] {
        self.context_stack
            .last()
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }
    /// Get the current stack depth.
    pub fn stack_depth(&self) -> usize {
        self.stack.len()
    }
    /// Check if the builder has any completed trees.
    pub fn has_roots(&self) -> bool {
        !self.roots.is_empty()
    }
    /// Get the number of root trees.
    pub fn root_count(&self) -> usize {
        self.roots.len()
    }
}
/// A local context entry visible during elaboration.
#[derive(Clone, Debug)]
pub struct LocalContextEntry {
    /// The variable name.
    pub name: Name,
    /// The variable type.
    pub ty: Expr,
    /// Optional value (for let-bindings).
    pub val: Option<Expr>,
    /// Whether this entry is a user-written name or auto-generated.
    pub is_user_name: bool,
}
impl LocalContextEntry {
    /// Create a new local context entry.
    pub fn new(name: Name, ty: Expr) -> Self {
        Self {
            name,
            ty,
            val: None,
            is_user_name: true,
        }
    }
    /// Create a let-bound entry.
    pub fn let_bound(name: Name, ty: Expr, val: Expr) -> Self {
        Self {
            name,
            ty,
            val: Some(val),
            is_user_name: true,
        }
    }
    /// Mark as auto-generated.
    pub fn with_auto(mut self) -> Self {
        self.is_user_name = false;
        self
    }
}
/// Collects all definition names from an info tree.
pub struct DefinitionCollector {
    /// Collected definition names.
    pub definitions: Vec<(Name, (usize, usize))>,
}
impl DefinitionCollector {
    /// Create a new definition collector.
    pub fn new() -> Self {
        Self {
            definitions: Vec::new(),
        }
    }
    /// Collect definitions from a tree.
    pub fn collect(tree: &InfoTree) -> Vec<(Name, (usize, usize))> {
        let mut collector = Self::new();
        walk_info_tree(tree, &mut collector);
        collector.definitions
    }
}
/// An info tree node, recording elaboration information for IDE support.
///
/// The tree mirrors the structure of the elaborated expression, with
/// each node carrying source location and type information.
#[derive(Clone, Debug)]
pub enum InfoTree {
    /// A node with attached info and child subtrees.
    Node {
        /// The information attached to this node.
        info: Info,
        /// Child info trees (one per subexpression).
        children: Vec<InfoTree>,
    },
    /// A hole (unsolved metavariable) with its expected type.
    Hole {
        /// The expected type of the hole.
        expected_type: Option<Expr>,
        /// Source range of the hole.
        range: Option<(usize, usize)>,
    },
    /// A local context scope, wrapping children with additional hypotheses.
    Context {
        /// The local context entries in scope.
        lctx: Vec<LocalContextEntry>,
        /// Child info trees within this context.
        children: Vec<InfoTree>,
        /// Source range of the context scope.
        range: Option<(usize, usize)>,
    },
}
impl InfoTree {
    /// Create a leaf node with info and no children.
    pub fn leaf(info: Info) -> Self {
        InfoTree::Node {
            info,
            children: Vec::new(),
        }
    }
    /// Create a node with info and children.
    pub fn node(info: Info, children: Vec<InfoTree>) -> Self {
        InfoTree::Node { info, children }
    }
    /// Create a hole node.
    pub fn hole(expected_type: Option<Expr>, range: Option<(usize, usize)>) -> Self {
        InfoTree::Hole {
            expected_type,
            range,
        }
    }
    /// Create a context node.
    pub fn context(
        lctx: Vec<LocalContextEntry>,
        children: Vec<InfoTree>,
        range: Option<(usize, usize)>,
    ) -> Self {
        InfoTree::Context {
            lctx,
            children,
            range,
        }
    }
    /// Get the source range of this tree node.
    pub fn range(&self) -> Option<(usize, usize)> {
        match self {
            InfoTree::Node { info, .. } => Some(info.stx_range),
            InfoTree::Hole { range, .. } => *range,
            InfoTree::Context { range, .. } => *range,
        }
    }
    /// Check if a position falls within this node's range.
    pub fn contains_pos(&self, pos: usize) -> bool {
        if let Some((start, end)) = self.range() {
            pos >= start && pos < end
        } else {
            false
        }
    }
    /// Count the total number of nodes in this tree.
    pub fn node_count(&self) -> usize {
        match self {
            InfoTree::Node { children, .. } => {
                1 + children.iter().map(|c| c.node_count()).sum::<usize>()
            }
            InfoTree::Hole { .. } => 1,
            InfoTree::Context { children, .. } => {
                1 + children.iter().map(|c| c.node_count()).sum::<usize>()
            }
        }
    }
    /// Get all children of this node.
    pub fn children(&self) -> &[InfoTree] {
        match self {
            InfoTree::Node { children, .. } => children,
            InfoTree::Hole { .. } => &[],
            InfoTree::Context { children, .. } => children,
        }
    }
    /// Get the depth of the tree.
    pub fn depth(&self) -> usize {
        match self {
            InfoTree::Node { children, .. } | InfoTree::Context { children, .. } => {
                1 + children.iter().map(|c| c.depth()).max().unwrap_or(0)
            }
            InfoTree::Hole { .. } => 1,
        }
    }
}
/// A go-to-definition entry: maps a source range to a target location.
#[derive(Clone, Debug)]
pub struct GotoDefEntry {
    /// The source range to jump from (byte offsets).
    pub source_range: (u32, u32),
    /// The target file path (empty = same file).
    pub target_file: String,
    /// The target range in the target file.
    pub target_range: (u32, u32),
    /// The name being jumped to.
    pub name: Name,
}
impl GotoDefEntry {
    /// Create an in-file go-to-def entry.
    pub fn in_file(source_range: (u32, u32), target_range: (u32, u32), name: Name) -> Self {
        Self {
            source_range,
            target_file: String::new(),
            target_range,
            name,
        }
    }
    /// Create a cross-file go-to-def entry.
    pub fn cross_file(
        source_range: (u32, u32),
        target_file: impl Into<String>,
        target_range: (u32, u32),
        name: Name,
    ) -> Self {
        Self {
            source_range,
            target_file: target_file.into(),
            target_range,
            name,
        }
    }
    /// Return `true` if the target is in the same file.
    pub fn is_local(&self) -> bool {
        self.target_file.is_empty()
    }
}
/// Collects all holes (unsolved metavariables) from an info tree.
pub struct HoleCollector {
    /// Collected holes with their expected types and ranges.
    pub holes: Vec<HoleEntry>,
}
impl HoleCollector {
    /// Create a new hole collector.
    pub fn new() -> Self {
        Self { holes: Vec::new() }
    }
    /// Collect holes from a tree.
    pub fn collect(tree: &InfoTree) -> Vec<HoleEntry> {
        let mut collector = Self::new();
        walk_info_tree(tree, &mut collector);
        collector.holes
    }
}
/// A frame on the builder stack, representing an in-progress node.
#[derive(Clone, Debug)]
struct BuilderFrame {
    /// The info for this node (set when opened).
    info: Option<Info>,
    /// Children accumulated so far.
    children: Vec<InfoTree>,
    /// The source range for context frames.
    range: Option<(usize, usize)>,
    /// Whether this frame is a context frame.
    is_context: bool,
    /// Local context entries for context frames.
    lctx: Vec<LocalContextEntry>,
}
impl BuilderFrame {
    fn new_node(info: Info) -> Self {
        let range = Some(info.stx_range);
        Self {
            info: Some(info),
            children: Vec::new(),
            range,
            is_context: false,
            lctx: Vec::new(),
        }
    }
    fn new_context(range: Option<(usize, usize)>, lctx: Vec<LocalContextEntry>) -> Self {
        Self {
            info: None,
            children: Vec::new(),
            range,
            is_context: true,
            lctx,
        }
    }
}
/// Index of go-to-definition entries for fast range lookup.
#[derive(Clone, Debug, Default)]
pub struct GotoDefIndex {
    entries: Vec<GotoDefEntry>,
}
impl GotoDefIndex {
    /// Create an empty index.
    pub fn new() -> Self {
        Self::default()
    }
    /// Insert an entry.
    pub fn insert(&mut self, entry: GotoDefEntry) {
        self.entries.push(entry);
    }
    /// Find all entries whose source range contains `pos`.
    pub fn lookup_pos(&self, pos: u32) -> Vec<&GotoDefEntry> {
        self.entries
            .iter()
            .filter(|e| e.source_range.0 <= pos && pos < e.source_range.1)
            .collect()
    }
    /// Number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Return `true` if the index is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
/// Collects all term infos for a given name.
pub struct NameUsageCollector {
    /// The name to search for.
    pub(super) target: Name,
    /// Collected usage ranges.
    pub usages: Vec<(usize, usize)>,
}
impl NameUsageCollector {
    /// Create a new name usage collector.
    pub fn new(target: Name) -> Self {
        Self {
            target,
            usages: Vec::new(),
        }
    }
    /// Collect usages from a tree.
    pub fn collect(tree: &InfoTree, name: &Name) -> Vec<(usize, usize)> {
        let mut collector = Self::new(name.clone());
        walk_info_tree(tree, &mut collector);
        collector.usages
    }
}
/// A single reference to a name in the source.
#[derive(Clone, Debug)]
pub struct NameReference {
    /// The name being referenced.
    pub name: Name,
    /// The range in the source where this reference occurs.
    pub range: (u32, u32),
    /// Whether this reference is a definition site.
    pub is_def_site: bool,
}
impl NameReference {
    /// Create a use-site reference.
    pub fn use_site(name: Name, range: (u32, u32)) -> Self {
        Self {
            name,
            range,
            is_def_site: false,
        }
    }
    /// Create a definition-site reference.
    pub fn def_site(name: Name, range: (u32, u32)) -> Self {
        Self {
            name,
            range,
            is_def_site: true,
        }
    }
}
/// The kind of a semantic token for syntax highlighting.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SemanticTokenKind {
    /// A keyword (def, theorem, etc.).
    Keyword,
    /// A type name.
    Type,
    /// A function/definition name.
    Function,
    /// A variable.
    Variable,
    /// A constructor.
    Constructor,
    /// A tactic keyword.
    Tactic,
    /// A number literal.
    Number,
    /// A string literal.
    String,
    /// An operator.
    Operator,
    /// A namespace.
    Namespace,
    /// A comment.
    Comment,
}
/// Information displayed when hovering over a term.
#[derive(Clone, Debug)]
pub struct HoverInfo {
    /// The expression under the cursor.
    pub expr: Option<Expr>,
    /// The type of the expression.
    pub ty: Option<Expr>,
    /// The fully qualified name (if a reference).
    pub name: Option<Name>,
    /// Documentation string.
    pub doc: Option<String>,
    /// The source range of the hovered term.
    pub range: (usize, usize),
    /// The local context at the hover position.
    pub local_context: Vec<LocalContextEntry>,
    /// Tactic state (if hovering over a tactic).
    pub tactic_state: Option<TacticStateInfo>,
}
impl HoverInfo {
    /// Create a new hover info.
    pub fn new(range: (usize, usize)) -> Self {
        Self {
            expr: None,
            ty: None,
            name: None,
            doc: None,
            range,
            local_context: Vec::new(),
            tactic_state: None,
        }
    }
    /// Set the expression.
    pub fn with_expr(mut self, expr: Expr) -> Self {
        self.expr = Some(expr);
        self
    }
    /// Set the type.
    pub fn with_type(mut self, ty: Expr) -> Self {
        self.ty = Some(ty);
        self
    }
    /// Set the name.
    pub fn with_name(mut self, name: Name) -> Self {
        self.name = Some(name);
        self
    }
    /// Set the documentation.
    pub fn with_doc(mut self, doc: String) -> Self {
        self.doc = Some(doc);
        self
    }
    /// Set the local context.
    pub fn with_local_context(mut self, ctx: Vec<LocalContextEntry>) -> Self {
        self.local_context = ctx;
        self
    }
    /// Set the tactic state.
    pub fn with_tactic_state(mut self, state: TacticStateInfo) -> Self {
        self.tactic_state = Some(state);
        self
    }
    /// Check if this hover info has any useful content.
    pub fn has_content(&self) -> bool {
        self.expr.is_some()
            || self.ty.is_some()
            || self.name.is_some()
            || self.doc.is_some()
            || self.tactic_state.is_some()
    }
    /// Format as a markdown string for display in the IDE.
    pub fn to_markdown(&self) -> String {
        let mut md = String::new();
        if let Some(name) = &self.name {
            md.push_str(&format!("**{}**\n\n", name));
        }
        if let Some(ty) = &self.ty {
            md.push_str(&format!("```lean\n{:?}\n```\n\n", ty));
        }
        if let Some(doc) = &self.doc {
            md.push_str(doc);
            md.push('\n');
        }
        if let Some(state) = &self.tactic_state {
            md.push_str(&format!("\n---\n\n**Tactic state:**\n{}\n", state));
        }
        if !self.local_context.is_empty() {
            md.push_str("\n---\n\n**Context:**\n");
            for entry in &self.local_context {
                md.push_str(&format!("- {}\n", entry));
            }
        }
        md
    }
}
/// Information about a single proof goal.
#[derive(Clone, Debug)]
pub struct GoalInfo {
    /// The goal name/identifier.
    pub name: Option<Name>,
    /// Hypotheses available in this goal.
    pub hypotheses: Vec<(Name, Expr)>,
    /// The target type to prove.
    pub target: Expr,
}
impl GoalInfo {
    /// Create a new goal info.
    pub fn new(target: Expr) -> Self {
        Self {
            name: None,
            hypotheses: Vec::new(),
            target,
        }
    }
    /// Create with a name.
    pub fn named(name: Name, target: Expr) -> Self {
        Self {
            name: Some(name),
            hypotheses: Vec::new(),
            target,
        }
    }
    /// Add a hypothesis.
    pub fn with_hypothesis(mut self, name: Name, ty: Expr) -> Self {
        self.hypotheses.push((name, ty));
        self
    }
    /// Add multiple hypotheses.
    pub fn with_hypotheses(mut self, hyps: Vec<(Name, Expr)>) -> Self {
        self.hypotheses.extend(hyps);
        self
    }
}
/// The specific kind of information recorded at an info tree node.
#[derive(Clone, Debug)]
pub enum InfoData {
    /// Information about a term: its expression and type.
    TermInfo {
        /// The elaborated kernel expression.
        expr: Expr,
        /// The type of the expression.
        type_: Expr,
    },
    /// Information about a field access.
    FieldInfo {
        /// The structure type being accessed.
        struct_name: Name,
        /// The field being accessed.
        field_name: Name,
        /// The type of the field.
        field_type: Option<Expr>,
    },
    /// Information about a tactic step.
    TacticInfo {
        /// The tactic state before this tactic runs.
        state_before: TacticStateInfo,
        /// The tactic state after this tactic runs.
        state_after: TacticStateInfo,
    },
    /// Information about a macro expansion.
    MacroExpansion {
        /// The expression before macro expansion.
        before: Expr,
        /// The expression after macro expansion.
        after: Expr,
    },
    /// Information about a command.
    CommandInfo {
        /// The command name (e.g., "def", "theorem", "inductive").
        name: String,
        /// The declared name (if any).
        decl_name: Option<Name>,
    },
    /// Information for completion suggestions.
    CompletionInfo {
        /// The prefix typed by the user.
        prefix: String,
        /// The expected type at this position (if known).
        expected_type: Option<Expr>,
        /// Available completions.
        completions: Vec<CompletionItem>,
    },
    /// Information about a universe level.
    LevelInfo {
        /// The universe level.
        level: Level,
    },
    /// Information about a binder.
    BinderInfo {
        /// The binder name.
        name: Name,
        /// The binder type.
        ty: Expr,
        /// The binder kind (explicit, implicit, instance).
        kind: BinderKind,
    },
    /// Information about a definition reference.
    RefInfo {
        /// The fully qualified name being referenced.
        name: Name,
        /// The type of the referenced definition.
        ty: Option<Expr>,
        /// Documentation string (if any).
        doc: Option<String>,
    },
}
/// An index mapping names to all their occurrences in a source file.
#[derive(Clone, Debug, Default)]
pub struct FindReferencesIndex {
    entries: HashMap<Name, Vec<NameReference>>,
}
impl FindReferencesIndex {
    /// Create an empty index.
    pub fn new() -> Self {
        Self::default()
    }
    /// Insert a reference.
    pub fn insert(&mut self, reference: NameReference) {
        self.entries
            .entry(reference.name.clone())
            .or_default()
            .push(reference);
    }
    /// Find all references to a given name.
    pub fn find(&self, name: &Name) -> &[NameReference] {
        self.entries.get(name).map(|v| v.as_slice()).unwrap_or(&[])
    }
    /// Find all references at or containing `pos`.
    pub fn at_pos(&self, pos: u32) -> Vec<&NameReference> {
        self.entries
            .values()
            .flatten()
            .filter(|r| r.range.0 <= pos && pos < r.range.1)
            .collect()
    }
    /// Number of distinct names in the index.
    pub fn num_names(&self) -> usize {
        self.entries.len()
    }
    /// Total number of references.
    pub fn total_refs(&self) -> usize {
        self.entries.values().map(|v| v.len()).sum()
    }
}
/// A single semantic token annotating a source range.
#[derive(Clone, Debug)]
pub struct SemanticToken {
    /// Byte range in the source (start, end).
    pub range: (u32, u32),
    /// Token kind.
    pub kind: SemanticTokenKind,
    /// Optional modifier flags (e.g. bold, deprecated).
    pub modifiers: u32,
}
impl SemanticToken {
    /// Create a token with no modifiers.
    pub fn new(range: (u32, u32), kind: SemanticTokenKind) -> Self {
        Self {
            range,
            kind,
            modifiers: 0,
        }
    }
    /// Set a modifier flag.
    pub fn with_modifier(mut self, flag: u32) -> Self {
        self.modifiers |= flag;
        self
    }
    /// Length of this token in bytes.
    pub fn len(&self) -> u32 {
        self.range.1.saturating_sub(self.range.0)
    }
}
/// A completion item suggested by the info tree.
#[derive(Clone, Debug)]
pub struct CompletionItem {
    /// The label shown to the user.
    pub label: String,
    /// The text to insert.
    pub insert_text: String,
    /// The kind of completion (function, type, tactic, etc.).
    pub kind: CompletionKind,
    /// Documentation for this completion.
    pub documentation: Option<String>,
    /// The type signature (for display).
    pub type_signature: Option<String>,
    /// Sort priority (lower = shown first).
    pub sort_priority: u32,
}
impl CompletionItem {
    /// Create a new completion item.
    pub fn new(label: String, insert_text: String, kind: CompletionKind) -> Self {
        Self {
            label,
            insert_text,
            kind,
            documentation: None,
            type_signature: None,
            sort_priority: 100,
        }
    }
    /// Set documentation.
    pub fn with_doc(mut self, doc: String) -> Self {
        self.documentation = Some(doc);
        self
    }
    /// Set type signature.
    pub fn with_type(mut self, sig: String) -> Self {
        self.type_signature = Some(sig);
        self
    }
    /// Set sort priority.
    pub fn with_priority(mut self, priority: u32) -> Self {
        self.sort_priority = priority;
        self
    }
}
/// Tactic state information for display in the IDE.
#[derive(Clone, Debug)]
pub struct TacticStateInfo {
    /// Goals remaining.
    pub goals: Vec<GoalInfo>,
    /// The current focus index.
    pub focus: usize,
}
impl TacticStateInfo {
    /// Create a new tactic state info.
    pub fn new() -> Self {
        Self {
            goals: Vec::new(),
            focus: 0,
        }
    }
    /// Create with a list of goals.
    pub fn with_goals(goals: Vec<GoalInfo>) -> Self {
        Self { goals, focus: 0 }
    }
    /// Create with a single goal.
    pub fn with_single_goal(goal: GoalInfo) -> Self {
        Self {
            goals: vec![goal],
            focus: 0,
        }
    }
    /// Check if there are no remaining goals.
    pub fn is_solved(&self) -> bool {
        self.goals.is_empty()
    }
    /// Get the focused goal.
    pub fn focused_goal(&self) -> Option<&GoalInfo> {
        self.goals.get(self.focus)
    }
    /// Number of remaining goals.
    pub fn num_goals(&self) -> usize {
        self.goals.len()
    }
}
/// Binder kind for the info tree.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BinderKind {
    /// Explicit binder `(x : T)`.
    Explicit,
    /// Implicit binder `{x : T}`.
    Implicit,
    /// Strict implicit binder.
    StrictImplicit,
    /// Instance binder `[x : T]`.
    Instance,
}
