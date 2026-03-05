//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use crate::lsp::{
    analyze_document, DefinitionInfo, Document, JsonValue, Position, Range, SymbolKind,
};
use oxilean_kernel::Environment;
use std::collections::HashMap;

/// Unique identifier for a widget instance.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct WidgetId(pub String);
impl WidgetId {
    /// Create a new widget ID.
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
    /// Generate a widget ID from URI and position.
    pub fn from_position(uri: &str, line: u32, character: u32) -> Self {
        Self(format!("{}:{}:{}", uri, line, character))
    }
}
/// Kind of proof tree node.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProofNodeKind {
    /// Root theorem.
    Theorem,
    /// A tactic application.
    Tactic,
    /// A sub-goal.
    Goal,
    /// A hypothesis.
    Hypothesis,
    /// A by-block.
    ByBlock,
    /// A sorry placeholder.
    Sorry,
}
impl ProofNodeKind {
    /// Return a string identifier.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Theorem => "theorem",
            Self::Tactic => "tactic",
            Self::Goal => "goal",
            Self::Hypothesis => "hypothesis",
            Self::ByBlock => "byBlock",
            Self::Sorry => "sorry",
        }
    }
}
/// The full state for a goal viewer widget.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct GoalViewerState {
    pub goals: Vec<GoalDisplay>,
    pub cursor_pos: usize,
    pub show_hypotheses: bool,
    pub show_types: bool,
    pub compact_mode: bool,
    pub search_query: String,
}
impl GoalViewerState {
    /// Create an initial empty state.
    #[allow(dead_code)]
    pub fn empty() -> Self {
        Self {
            goals: vec![],
            cursor_pos: 0,
            show_hypotheses: true,
            show_types: true,
            compact_mode: false,
            search_query: String::new(),
        }
    }
    /// Return the currently focused goal.
    #[allow(dead_code)]
    pub fn current_goal(&self) -> Option<&GoalDisplay> {
        self.goals.get(self.cursor_pos)
    }
    /// Move cursor to next goal.
    #[allow(dead_code)]
    pub fn next_goal(&mut self) {
        if !self.goals.is_empty() {
            self.cursor_pos = (self.cursor_pos + 1) % self.goals.len();
        }
    }
    /// Move cursor to previous goal.
    #[allow(dead_code)]
    pub fn prev_goal(&mut self) {
        if !self.goals.is_empty() {
            self.cursor_pos = if self.cursor_pos == 0 {
                self.goals.len() - 1
            } else {
                self.cursor_pos - 1
            };
        }
    }
    /// Filter hypotheses by the current search query.
    #[allow(dead_code)]
    pub fn filtered_hypotheses(&self) -> Vec<&HypothesisDisplay> {
        let Some(goal) = self.current_goal() else {
            return vec![];
        };
        if self.search_query.is_empty() {
            return goal.hypotheses.iter().collect();
        }
        goal.hypotheses
            .iter()
            .filter(|h| {
                h.name.contains(&self.search_query) || h.type_str.contains(&self.search_query)
            })
            .collect()
    }
    /// Render the goal viewer to a string.
    #[allow(dead_code)]
    pub fn render(&self) -> String {
        if self.goals.is_empty() {
            return "No goals.".to_string();
        }
        let mut out = format!("Goals: {}\n", self.goals.len());
        for (i, goal) in self.goals.iter().enumerate() {
            let marker = if i == self.cursor_pos { "▶ " } else { "  " };
            out.push_str(&format!("{}Goal {}: {}\n", marker, i + 1, goal.target));
            if self.show_hypotheses && i == self.cursor_pos {
                for hyp in &goal.hypotheses {
                    if self.show_types {
                        out.push_str(&format!("  {} : {}\n", hyp.name, hyp.type_str));
                    } else {
                        out.push_str(&format!("  {}\n", hyp.name));
                    }
                }
            }
        }
        out
    }
}
/// Response to a widget event.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum WidgetEventResponse {
    /// No action needed
    NoOp,
    /// Widget should re-render
    Redraw,
    /// Open a file at a location
    OpenFile { uri: String, line: u32 },
    /// Apply a code action
    ApplyAction { title: String, uri: String },
}
/// Tracks overall proof progress in a file.
#[derive(Clone, Debug)]
pub struct ProofProgress {
    /// Total number of theorems.
    pub total_theorems: usize,
    /// Number of fully proven theorems.
    pub proven_theorems: usize,
    /// Number of theorems with sorry.
    pub sorry_theorems: usize,
    /// Number of theorems in progress.
    pub in_progress_theorems: usize,
    /// Progress percentage (0-100).
    pub progress_percent: f64,
    /// Per-theorem progress info.
    pub theorem_progress: Vec<TheoremProgress>,
}
impl ProofProgress {
    /// Serialize to JSON.
    pub fn to_json(&self) -> JsonValue {
        JsonValue::Object(vec![
            (
                "totalTheorems".to_string(),
                JsonValue::Number(self.total_theorems as f64),
            ),
            (
                "provenTheorems".to_string(),
                JsonValue::Number(self.proven_theorems as f64),
            ),
            (
                "sorryTheorems".to_string(),
                JsonValue::Number(self.sorry_theorems as f64),
            ),
            (
                "inProgressTheorems".to_string(),
                JsonValue::Number(self.in_progress_theorems as f64),
            ),
            (
                "progressPercent".to_string(),
                JsonValue::Number(self.progress_percent),
            ),
            (
                "theorems".to_string(),
                JsonValue::Array(
                    self.theorem_progress
                        .iter()
                        .map(|t| {
                            JsonValue::Object(vec![
                                ("name".to_string(), JsonValue::String(t.name.clone())),
                                (
                                    "status".to_string(),
                                    JsonValue::String(t.status.as_str().to_string()),
                                ),
                                (
                                    "sorryCount".to_string(),
                                    JsonValue::Number(t.sorry_count as f64),
                                ),
                                ("range".to_string(), t.range.to_json()),
                            ])
                        })
                        .collect(),
                ),
            ),
        ])
    }
}
/// Color values for widget theming.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct WidgetColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
impl WidgetColor {
    /// Create a new color.
    #[allow(dead_code)]
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }
    /// Create a new color with alpha.
    #[allow(dead_code)]
    pub fn with_alpha(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    /// Return the CSS hex color string.
    #[allow(dead_code)]
    pub fn to_css_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}
/// Events that can be sent to a widget.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum WidgetEvent {
    /// User clicked on a position within the widget
    Click { x: u32, y: u32 },
    /// User scrolled the widget
    Scroll { delta: i32 },
    /// User hovered over a position
    Hover { x: u32, y: u32 },
    /// Widget was focused
    Focus,
    /// Widget lost focus
    Blur,
    /// Content should be refreshed
    Refresh,
    /// User pressed a key
    KeyPress { key: String, modifiers: u32 },
}
/// Tracks which widget currently has focus.
#[allow(dead_code)]
pub struct WidgetFocusTracker {
    focused: Option<WidgetId>,
    focus_order: Vec<WidgetId>,
}
impl WidgetFocusTracker {
    /// Create a new focus tracker.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            focused: None,
            focus_order: vec![],
        }
    }
    /// Focus a specific widget.
    #[allow(dead_code)]
    pub fn focus(&mut self, id: WidgetId) {
        if !self.focus_order.contains(&id) {
            self.focus_order.push(id.clone());
        }
        self.focused = Some(id);
    }
    /// Blur the currently focused widget.
    #[allow(dead_code)]
    pub fn blur(&mut self) {
        self.focused = None;
    }
    /// Return the currently focused widget ID.
    #[allow(dead_code)]
    pub fn current(&self) -> Option<&WidgetId> {
        self.focused.as_ref()
    }
    /// Move focus to the next widget in the order.
    #[allow(dead_code)]
    pub fn focus_next(&mut self) {
        if self.focus_order.is_empty() {
            return;
        }
        let next = if let Some(ref current) = self.focused {
            let idx = self.focus_order.iter().position(|id| id == current);
            match idx {
                Some(i) => self.focus_order[(i + 1) % self.focus_order.len()].clone(),
                None => self.focus_order[0].clone(),
            }
        } else {
            self.focus_order[0].clone()
        };
        self.focused = Some(next);
    }
}
/// A node in the proof tree.
#[derive(Clone, Debug)]
pub struct ProofTreeNode {
    /// Unique ID for this node.
    pub id: String,
    /// Display label.
    pub label: String,
    /// Node kind.
    pub kind: ProofNodeKind,
    /// Status of this node.
    pub status: ProofNodeStatus,
    /// Range in source code.
    pub range: Range,
    /// Children nodes.
    pub children: Vec<ProofTreeNode>,
    /// Additional info.
    pub info: Option<String>,
}
impl ProofTreeNode {
    /// Serialize to JSON.
    pub fn to_json(&self) -> JsonValue {
        let mut entries = vec![
            ("id".to_string(), JsonValue::String(self.id.clone())),
            ("label".to_string(), JsonValue::String(self.label.clone())),
            (
                "kind".to_string(),
                JsonValue::String(self.kind.as_str().to_string()),
            ),
            (
                "status".to_string(),
                JsonValue::String(self.status.as_str().to_string()),
            ),
            ("range".to_string(), self.range.to_json()),
            (
                "children".to_string(),
                JsonValue::Array(self.children.iter().map(|c| c.to_json()).collect()),
            ),
        ];
        if let Some(ref info) = self.info {
            entries.push(("info".to_string(), JsonValue::String(info.clone())));
        }
        JsonValue::Object(entries)
    }
    /// Count total nodes in the tree.
    pub fn node_count(&self) -> usize {
        1 + self.children.iter().map(|c| c.node_count()).sum::<usize>()
    }
    /// Check if all children are proven.
    pub fn all_proven(&self) -> bool {
        self.status == ProofNodeStatus::Proven && self.children.iter().all(|c| c.all_proven())
    }
    /// Count sorry nodes.
    pub fn sorry_count(&self) -> usize {
        let own = if self.kind == ProofNodeKind::Sorry {
            1
        } else {
            0
        };
        own + self.children.iter().map(|c| c.sorry_count()).sum::<usize>()
    }
}
/// Layout hints for a widget.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct WidgetLayout {
    pub width: u32,
    pub height: u32,
    pub padding: u32,
    pub scrollable: bool,
    pub collapsible: bool,
    pub collapsed: bool,
}
impl WidgetLayout {
    /// Create a default layout.
    #[allow(dead_code)]
    pub fn default_layout() -> Self {
        Self {
            width: 400,
            height: 300,
            padding: 8,
            scrollable: true,
            collapsible: true,
            collapsed: false,
        }
    }
    /// Create a compact layout.
    #[allow(dead_code)]
    pub fn compact() -> Self {
        Self {
            width: 200,
            height: 150,
            padding: 4,
            scrollable: false,
            collapsible: false,
            collapsed: false,
        }
    }
    /// Toggle collapse state.
    #[allow(dead_code)]
    pub fn toggle_collapsed(&mut self) {
        if self.collapsible {
            self.collapsed = !self.collapsed;
        }
    }
}
/// Performance statistics for a widget.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct WidgetPerfStats {
    pub render_count: u64,
    pub total_render_us: u64,
    pub min_render_us: u64,
    pub max_render_us: u64,
    pub event_count: u64,
}
impl WidgetPerfStats {
    /// Record a render with the given duration in microseconds.
    #[allow(dead_code)]
    pub fn record_render(&mut self, duration_us: u64) {
        self.render_count += 1;
        self.total_render_us += duration_us;
        if self.render_count == 1 || duration_us < self.min_render_us {
            self.min_render_us = duration_us;
        }
        if duration_us > self.max_render_us {
            self.max_render_us = duration_us;
        }
    }
    /// Return the average render time in microseconds.
    #[allow(dead_code)]
    pub fn avg_render_us(&self) -> f64 {
        if self.render_count == 0 {
            0.0
        } else {
            self.total_render_us as f64 / self.render_count as f64
        }
    }
}
/// A proof goal with structured information.
#[derive(Clone, Debug)]
pub struct ProofGoal {
    /// Goal index (for multiple goals).
    pub index: usize,
    /// Hypotheses in the context.
    pub hypotheses: Vec<Hypothesis>,
    /// The target type to prove.
    pub target: String,
    /// Optional tag/label for this goal.
    pub tag: Option<String>,
    /// Whether this goal is the focused goal.
    pub is_focused: bool,
}
impl ProofGoal {
    /// Serialize to JSON.
    pub fn to_json(&self) -> JsonValue {
        JsonValue::Object(vec![
            ("index".to_string(), JsonValue::Number(self.index as f64)),
            (
                "hypotheses".to_string(),
                JsonValue::Array(self.hypotheses.iter().map(|h| h.to_json()).collect()),
            ),
            ("target".to_string(), JsonValue::String(self.target.clone())),
            (
                "tag".to_string(),
                self.tag
                    .as_ref()
                    .map(|t| JsonValue::String(t.clone()))
                    .unwrap_or(JsonValue::Null),
            ),
            ("isFocused".to_string(), JsonValue::Bool(self.is_focused)),
        ])
    }
}
/// Undo/redo history for widget states.
#[allow(dead_code)]
pub struct WidgetHistory {
    past: Vec<WidgetSnapshot>,
    future: Vec<WidgetSnapshot>,
    max_size: usize,
}
impl WidgetHistory {
    /// Create a new history store.
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        Self {
            past: vec![],
            future: vec![],
            max_size,
        }
    }
    /// Push a snapshot (clears the redo stack).
    #[allow(dead_code)]
    pub fn push(&mut self, snapshot: WidgetSnapshot) {
        self.future.clear();
        self.past.push(snapshot);
        if self.past.len() > self.max_size {
            self.past.remove(0);
        }
    }
    /// Undo: return the previous snapshot.
    #[allow(dead_code)]
    pub fn undo(&mut self) -> Option<WidgetSnapshot> {
        let snap = self.past.pop()?;
        self.future.push(snap.clone());
        Some(snap)
    }
    /// Redo: return the next snapshot.
    #[allow(dead_code)]
    pub fn redo(&mut self) -> Option<WidgetSnapshot> {
        let snap = self.future.pop()?;
        self.past.push(snap.clone());
        Some(snap)
    }
    /// Return whether undo is available.
    /// Requires at least two snapshots: the current state plus a previous one.
    #[allow(dead_code)]
    pub fn can_undo(&self) -> bool {
        self.past.len() > 1
    }
    /// Return whether redo is available.
    #[allow(dead_code)]
    pub fn can_redo(&self) -> bool {
        !self.future.is_empty()
    }
}
/// A listener for widget events.
#[derive(Clone)]
pub struct WidgetEventListener {
    /// Name of the listener.
    pub name: String,
    /// Widget kinds this listener is interested in.
    pub kinds: Vec<WidgetKind>,
}
/// The goal viewer widget that displays proof state.
pub struct GoalViewerWidget<'a> {
    /// Reference to the environment.
    env: &'a Environment,
}
impl<'a> GoalViewerWidget<'a> {
    /// Create a new goal viewer.
    pub fn new(env: &'a Environment) -> Self {
        Self { env }
    }
    /// Compute the goal state at a position.
    pub fn compute_goals(&self, doc: &Document, pos: &Position) -> Vec<ProofGoal> {
        let analysis = analyze_document(&doc.uri, &doc.content, self.env);
        let mut current_theorem = None;
        for def in &analysis.definitions {
            if def.kind == SymbolKind::Method && def.range.start.line <= pos.line {
                current_theorem = Some(def);
            }
        }
        let thm = match current_theorem {
            Some(t) => t,
            None => return Vec::new(),
        };
        let mut in_proof = false;
        let mut line_idx = pos.line as usize;
        while line_idx >= thm.range.start.line as usize {
            if let Some(line) = doc.get_line(line_idx as u32) {
                let trimmed = line.trim();
                if trimmed == "by" || trimmed.starts_with("by ") || trimmed.ends_with(" by") {
                    in_proof = true;
                    break;
                }
            }
            if line_idx == 0 {
                break;
            }
            line_idx -= 1;
        }
        if !in_proof {
            return Vec::new();
        }
        let mut hyps = Vec::new();
        let mut tactic_lines = Vec::new();
        let by_line = line_idx;
        for l in by_line + 1..=pos.line as usize {
            if let Some(line) = doc.get_line(l as u32) {
                let trimmed = line.trim();
                if !trimmed.is_empty() {
                    tactic_lines.push(trimmed.to_string());
                }
            }
        }
        for tactic_line in &tactic_lines {
            if let Some(rest) = tactic_line.strip_prefix("intro ") {
                for name in rest.split_whitespace() {
                    hyps.push(Hypothesis {
                        name: name.to_string(),
                        ty: "?".to_string(),
                        is_new: true,
                        is_used: false,
                    });
                }
            } else if tactic_line.starts_with("intros") {
                hyps.push(Hypothesis {
                    name: "...".to_string(),
                    ty: "(auto-introduced)".to_string(),
                    is_new: true,
                    is_used: false,
                });
            } else if let Some(rest) = tactic_line.strip_prefix("have ") {
                if let Some(colon_pos) = rest.find(':') {
                    let name = rest[..colon_pos].trim();
                    let ty = rest[colon_pos + 1..].trim();
                    let ty = ty.split(":=").next().unwrap_or(ty).trim();
                    hyps.push(Hypothesis {
                        name: name.to_string(),
                        ty: ty.to_string(),
                        is_new: true,
                        is_used: false,
                    });
                }
            }
        }
        let target = thm.ty.as_deref().unwrap_or("?goal").to_string();
        vec![ProofGoal {
            index: 0,
            hypotheses: hyps,
            target,
            tag: Some(thm.name.clone()),
            is_focused: true,
        }]
    }
    /// Render the goal viewer widget data.
    pub fn render(&self, doc: &Document, pos: &Position) -> JsonValue {
        let goals = self.compute_goals(doc, pos);
        JsonValue::Object(vec![
            (
                "kind".to_string(),
                JsonValue::String("goalViewer".to_string()),
            ),
            (
                "goals".to_string(),
                JsonValue::Array(goals.iter().map(|g| g.to_json()).collect()),
            ),
            (
                "goalCount".to_string(),
                JsonValue::Number(goals.len() as f64),
            ),
        ])
    }
}
/// Progress for a single theorem.
#[derive(Clone, Debug)]
pub struct TheoremProgress {
    /// Theorem name.
    pub name: String,
    /// Status.
    pub status: ProofNodeStatus,
    /// Number of sorry in this theorem.
    pub sorry_count: usize,
    /// Range in the document.
    pub range: Range,
}
/// A node in a proof tree.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ProofTreeNodeV2 {
    pub id: usize,
    pub tactic: String,
    pub goal_before: String,
    pub goal_after: Option<String>,
    pub children: Vec<ProofTreeNodeV2>,
    pub is_sorry: bool,
    pub is_complete: bool,
}
impl ProofTreeNodeV2 {
    /// Create a leaf node.
    #[allow(dead_code)]
    pub fn leaf(id: usize, tactic: impl Into<String>, goal: impl Into<String>) -> Self {
        Self {
            id,
            tactic: tactic.into(),
            goal_before: goal.into(),
            goal_after: None,
            children: vec![],
            is_sorry: false,
            is_complete: false,
        }
    }
    /// Count total nodes in this subtree.
    #[allow(dead_code)]
    pub fn node_count(&self) -> usize {
        1 + self.children.iter().map(|c| c.node_count()).sum::<usize>()
    }
    /// Find a node by ID.
    #[allow(dead_code)]
    pub fn find(&self, id: usize) -> Option<&ProofTreeNodeV2> {
        if self.id == id {
            return Some(self);
        }
        for child in &self.children {
            if let Some(found) = child.find(id) {
                return Some(found);
            }
        }
        None
    }
    /// Render the tree as indented text.
    #[allow(dead_code)]
    pub fn render(&self, depth: usize) -> String {
        let indent: String = "  ".repeat(depth);
        let status = if self.is_complete {
            "✓"
        } else if self.is_sorry {
            "?"
        } else {
            "○"
        };
        let mut out = format!("{}{} [{}] {}\n", indent, status, self.id, self.tactic);
        for child in &self.children {
            out.push_str(&child.render(depth + 1));
        }
        out
    }
}
/// Status of a proof tree node.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProofNodeStatus {
    /// Successfully proven.
    Proven,
    /// In progress.
    InProgress,
    /// Failed / has errors.
    Failed,
    /// Admitted (sorry).
    Admitted,
    /// Not yet started.
    Pending,
}
impl ProofNodeStatus {
    /// Return a string identifier.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Proven => "proven",
            Self::InProgress => "inProgress",
            Self::Failed => "failed",
            Self::Admitted => "admitted",
            Self::Pending => "pending",
        }
    }
}
/// A hypothesis in the proof context.
#[derive(Clone, Debug)]
pub struct Hypothesis {
    /// Hypothesis name.
    pub name: String,
    /// Hypothesis type.
    pub ty: String,
    /// Whether this hypothesis was recently introduced.
    pub is_new: bool,
    /// Whether this hypothesis is used in the current proof step.
    pub is_used: bool,
}
impl Hypothesis {
    /// Serialize to JSON.
    pub fn to_json(&self) -> JsonValue {
        JsonValue::Object(vec![
            ("name".to_string(), JsonValue::String(self.name.clone())),
            ("type".to_string(), JsonValue::String(self.ty.clone())),
            ("isNew".to_string(), JsonValue::Bool(self.is_new)),
            ("isUsed".to_string(), JsonValue::Bool(self.is_used)),
        ])
    }
}
/// Theme for a widget.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct WidgetTheme {
    pub background: WidgetColor,
    pub foreground: WidgetColor,
    pub border: WidgetColor,
    pub highlight: WidgetColor,
    pub error_color: WidgetColor,
    pub success_color: WidgetColor,
    pub font_size: u32,
    pub font_family: String,
}
impl WidgetTheme {
    /// Default dark theme.
    #[allow(dead_code)]
    pub fn dark() -> Self {
        Self {
            background: WidgetColor::new(30, 30, 30),
            foreground: WidgetColor::new(220, 220, 220),
            border: WidgetColor::new(80, 80, 80),
            highlight: WidgetColor::new(50, 120, 200),
            error_color: WidgetColor::new(220, 50, 50),
            success_color: WidgetColor::new(50, 180, 50),
            font_size: 14,
            font_family: "monospace".to_string(),
        }
    }
    /// Default light theme.
    #[allow(dead_code)]
    pub fn light() -> Self {
        Self {
            background: WidgetColor::new(255, 255, 255),
            foreground: WidgetColor::new(30, 30, 30),
            border: WidgetColor::new(180, 180, 180),
            highlight: WidgetColor::new(0, 80, 200),
            error_color: WidgetColor::new(200, 0, 0),
            success_color: WidgetColor::new(0, 160, 0),
            font_size: 14,
            font_family: "monospace".to_string(),
        }
    }
}
/// A searchable index of widget content.
#[allow(dead_code)]
pub struct WidgetSearchIndex {
    entries: Vec<(WidgetId, String)>,
}
impl WidgetSearchIndex {
    /// Create a new index.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { entries: vec![] }
    }
    /// Index content for a widget.
    #[allow(dead_code)]
    pub fn index(&mut self, id: WidgetId, content: String) {
        if let Some(entry) = self.entries.iter_mut().find(|(wid, _)| wid == &id) {
            entry.1 = content;
        } else {
            self.entries.push((id, content));
        }
    }
    /// Search for widgets containing the query.
    #[allow(dead_code)]
    pub fn search(&self, query: &str) -> Vec<&WidgetId> {
        self.entries
            .iter()
            .filter(|(_, content)| content.contains(query))
            .map(|(id, _)| id)
            .collect()
    }
    /// Remove an entry.
    #[allow(dead_code)]
    pub fn remove(&mut self, id: &WidgetId) {
        self.entries.retain(|(wid, _)| wid != id);
    }
}
/// Where a panel is docked.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PanelPosition {
    Left,
    Right,
    Bottom,
    Floating,
}
/// Generates tactic suggestions based on proof context.
pub struct TacticSuggestor<'a> {
    /// Reference to the environment.
    env: &'a Environment,
}
impl<'a> TacticSuggestor<'a> {
    /// Create a new tactic suggestor.
    pub fn new(env: &'a Environment) -> Self {
        Self { env }
    }
    /// Generate suggestions for a proof goal.
    pub fn suggest(&self, goal: &ProofGoal) -> Vec<TacticSuggestion> {
        let mut suggestions = Vec::new();
        suggestions.extend(self.suggest_from_goal_shape(&goal.target));
        suggestions.extend(self.suggest_from_hypotheses(&goal.hypotheses, &goal.target));
        suggestions.extend(self.suggest_common_patterns(goal));
        suggestions.sort_by(|a, b| {
            b.confidence
                .partial_cmp(&a.confidence)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        if suggestions.len() > 10 {
            suggestions.truncate(10);
        }
        suggestions
    }
    /// Suggest tactics based on the shape of the goal.
    fn suggest_from_goal_shape(&self, target: &str) -> Vec<TacticSuggestion> {
        let mut suggestions = Vec::new();
        if target.contains('=') {
            suggestions.push(TacticSuggestion {
                tactic: "rfl".to_string(),
                confidence: 0.7,
                explanation: "Goal is an equality; try reflexivity".to_string(),
                category: SuggestionCategory::GoalBased,
                closes_goal: true,
            });
            suggestions.push(TacticSuggestion {
                tactic: "simp".to_string(),
                confidence: 0.6,
                explanation: "Simplification may close equality goals".to_string(),
                category: SuggestionCategory::GoalBased,
                closes_goal: false,
            });
            suggestions.push(TacticSuggestion {
                tactic: "ring".to_string(),
                confidence: 0.5,
                explanation: "Ring tactic for algebraic equalities".to_string(),
                category: SuggestionCategory::GoalBased,
                closes_goal: true,
            });
        }
        if target.contains("/\\") || target.contains("And") {
            suggestions.push(TacticSuggestion {
                tactic: "constructor".to_string(),
                confidence: 0.9,
                explanation: "Split conjunction into two sub-goals".to_string(),
                category: SuggestionCategory::GoalBased,
                closes_goal: false,
            });
        }
        if target.contains("\\/") || target.contains("Or") {
            suggestions.push(TacticSuggestion {
                tactic: "left".to_string(),
                confidence: 0.5,
                explanation: "Choose the left disjunct".to_string(),
                category: SuggestionCategory::GoalBased,
                closes_goal: false,
            });
            suggestions.push(TacticSuggestion {
                tactic: "right".to_string(),
                confidence: 0.5,
                explanation: "Choose the right disjunct".to_string(),
                category: SuggestionCategory::GoalBased,
                closes_goal: false,
            });
        }
        if target.contains("->") || target.contains("forall") {
            suggestions.push(TacticSuggestion {
                tactic: "intro h".to_string(),
                confidence: 0.9,
                explanation: "Introduce the hypothesis".to_string(),
                category: SuggestionCategory::GoalBased,
                closes_goal: false,
            });
        }
        if target.contains("Exists") || target.contains("\\exists") {
            suggestions.push(TacticSuggestion {
                tactic: "use ?_".to_string(),
                confidence: 0.7,
                explanation: "Provide an existential witness".to_string(),
                category: SuggestionCategory::GoalBased,
                closes_goal: false,
            });
        }
        if target == "True" {
            suggestions.push(TacticSuggestion {
                tactic: "trivial".to_string(),
                confidence: 1.0,
                explanation: "Goal is True; trivially provable".to_string(),
                category: SuggestionCategory::GoalBased,
                closes_goal: true,
            });
        }
        if target.contains("Nat") || target.contains("Int") {
            suggestions.push(TacticSuggestion {
                tactic: "omega".to_string(),
                confidence: 0.6,
                explanation: "Try linear arithmetic solver".to_string(),
                category: SuggestionCategory::GoalBased,
                closes_goal: true,
            });
        }
        suggestions
    }
    /// Suggest tactics based on available hypotheses.
    fn suggest_from_hypotheses(&self, hyps: &[Hypothesis], _target: &str) -> Vec<TacticSuggestion> {
        let mut suggestions = Vec::new();
        if !hyps.is_empty() {
            suggestions.push(TacticSuggestion {
                tactic: "assumption".to_string(),
                confidence: 0.4,
                explanation: "Try to close goal from context".to_string(),
                category: SuggestionCategory::HypothesisBased,
                closes_goal: true,
            });
            for hyp in hyps {
                suggestions.push(TacticSuggestion {
                    tactic: format!("exact {}", hyp.name),
                    confidence: 0.3,
                    explanation: format!("Use hypothesis {} directly", hyp.name),
                    category: SuggestionCategory::HypothesisBased,
                    closes_goal: true,
                });
                suggestions.push(TacticSuggestion {
                    tactic: format!("apply {}", hyp.name),
                    confidence: 0.3,
                    explanation: format!("Apply hypothesis {}", hyp.name),
                    category: SuggestionCategory::HypothesisBased,
                    closes_goal: false,
                });
            }
        }
        suggestions
    }
    /// Suggest common tactic patterns.
    fn suggest_common_patterns(&self, _goal: &ProofGoal) -> Vec<TacticSuggestion> {
        vec![
            TacticSuggestion {
                tactic: "simp".to_string(),
                confidence: 0.3,
                explanation: "General simplification".to_string(),
                category: SuggestionCategory::PatternBased,
                closes_goal: false,
            },
            TacticSuggestion {
                tactic: "norm_num".to_string(),
                confidence: 0.2,
                explanation: "Normalize numeric expressions".to_string(),
                category: SuggestionCategory::PatternBased,
                closes_goal: false,
            },
        ]
    }
    /// Render the suggestion widget data.
    pub fn render(&self, goal: &ProofGoal) -> JsonValue {
        let suggestions = self.suggest(goal);
        JsonValue::Object(vec![
            (
                "kind".to_string(),
                JsonValue::String("tacticSuggestion".to_string()),
            ),
            (
                "suggestions".to_string(),
                JsonValue::Array(suggestions.iter().map(|s| s.to_json()).collect()),
            ),
            (
                "goalTarget".to_string(),
                JsonValue::String(goal.target.clone()),
            ),
        ])
    }
}
/// The state for a tactic suggestion widget.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct TacticSuggestionState {
    pub suggestions: Vec<TacticSuggestionItem>,
    pub selected_index: usize,
    pub filter_text: String,
}
impl TacticSuggestionState {
    /// Create a new state.
    #[allow(dead_code)]
    pub fn new(suggestions: Vec<TacticSuggestionItem>) -> Self {
        Self {
            suggestions,
            selected_index: 0,
            filter_text: String::new(),
        }
    }
    /// Return filtered suggestions.
    #[allow(dead_code)]
    pub fn filtered(&self) -> Vec<&TacticSuggestionItem> {
        if self.filter_text.is_empty() {
            return self.suggestions.iter().collect();
        }
        self.suggestions
            .iter()
            .filter(|s| {
                s.tactic.contains(&self.filter_text) || s.description.contains(&self.filter_text)
            })
            .collect()
    }
    /// Sort suggestions by confidence (highest first).
    #[allow(dead_code)]
    pub fn sort_by_confidence(&mut self) {
        self.suggestions.sort_by(|a, b| {
            b.confidence
                .partial_cmp(&a.confidence)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
    }
    /// Return the currently selected suggestion.
    #[allow(dead_code)]
    pub fn selected(&self) -> Option<&TacticSuggestionItem> {
        self.suggestions.get(self.selected_index)
    }
}
/// Manages all active widget instances.
pub struct WidgetManager {
    /// Active widgets keyed by ID.
    widgets: HashMap<WidgetId, WidgetState>,
    /// Maximum number of active widgets.
    max_widgets: usize,
    /// Event listeners.
    listeners: Vec<WidgetEventListener>,
    /// Widget ID counter.
    next_id: u64,
}
impl WidgetManager {
    /// Create a new widget manager.
    pub fn new() -> Self {
        Self {
            widgets: HashMap::new(),
            max_widgets: 50,
            listeners: Vec::new(),
            next_id: 0,
        }
    }
    /// Create a new widget and return its ID.
    pub fn create_widget(&mut self, kind: WidgetKind, uri: &str, position: Position) -> WidgetId {
        self.next_id += 1;
        let id = WidgetId::new(format!("widget_{}", self.next_id));
        let state = WidgetState::new(id.clone(), kind, uri, position);
        self.widgets.insert(id.clone(), state);
        while self.widgets.len() > self.max_widgets {
            if let Some(oldest) = self.widgets.keys().next().cloned() {
                self.widgets.remove(&oldest);
            }
        }
        id
    }
    /// Get a widget by ID.
    pub fn get_widget(&self, id: &WidgetId) -> Option<&WidgetState> {
        self.widgets.get(id)
    }
    /// Get a mutable widget by ID.
    pub fn get_widget_mut(&mut self, id: &WidgetId) -> Option<&mut WidgetState> {
        self.widgets.get_mut(id)
    }
    /// Update widget data.
    pub fn update_data(&mut self, id: &WidgetId, data: JsonValue, version: i64) -> bool {
        if let Some(state) = self.widgets.get_mut(id) {
            state.data = data;
            state.version = version;
            true
        } else {
            false
        }
    }
    /// Toggle widget visibility.
    pub fn toggle_visibility(&mut self, id: &WidgetId) -> bool {
        if let Some(state) = self.widgets.get_mut(id) {
            state.visible = !state.visible;
            true
        } else {
            false
        }
    }
    /// Toggle widget collapsed state.
    pub fn toggle_collapsed(&mut self, id: &WidgetId) -> bool {
        if let Some(state) = self.widgets.get_mut(id) {
            state.collapsed = !state.collapsed;
            true
        } else {
            false
        }
    }
    /// Remove a widget.
    pub fn remove_widget(&mut self, id: &WidgetId) -> bool {
        self.widgets.remove(id).is_some()
    }
    /// Get all widgets for a URI.
    pub fn widgets_for_uri(&self, uri: &str) -> Vec<&WidgetState> {
        self.widgets.values().filter(|w| w.uri == uri).collect()
    }
    /// Get all widgets of a specific kind.
    pub fn widgets_of_kind(&self, kind: WidgetKind) -> Vec<&WidgetState> {
        self.widgets.values().filter(|w| w.kind == kind).collect()
    }
    /// Remove all widgets for a URI.
    pub fn remove_widgets_for_uri(&mut self, uri: &str) {
        self.widgets.retain(|_, w| w.uri != uri);
    }
    /// Add an event listener.
    pub fn add_listener(&mut self, listener: WidgetEventListener) {
        self.listeners.push(listener);
    }
    /// Get the total number of active widgets.
    pub fn widget_count(&self) -> usize {
        self.widgets.len()
    }
    /// Serialize all widgets to JSON.
    pub fn all_to_json(&self) -> JsonValue {
        JsonValue::Array(self.widgets.values().map(|w| w.to_json()).collect())
    }
    /// Clear all widgets.
    pub fn clear(&mut self) {
        self.widgets.clear();
    }
}
/// The kind of widget.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WidgetKind {
    /// Goal viewer widget.
    GoalViewer,
    /// Proof tree widget.
    ProofTree,
    /// Tactic suggestion widget.
    TacticSuggestion,
    /// Type inspector widget.
    TypeInspector,
    /// Hypothesis explorer widget.
    HypothesisExplorer,
    /// Expression tree widget.
    ExpressionTree,
    /// Documentation panel widget.
    DocumentationPanel,
    /// Proof progress widget.
    ProofProgress,
}
impl WidgetKind {
    /// Return a string identifier.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::GoalViewer => "goalViewer",
            Self::ProofTree => "proofTree",
            Self::TacticSuggestion => "tacticSuggestion",
            Self::TypeInspector => "typeInspector",
            Self::HypothesisExplorer => "hypothesisExplorer",
            Self::ExpressionTree => "expressionTree",
            Self::DocumentationPanel => "documentationPanel",
            Self::ProofProgress => "proofProgress",
        }
    }
}
/// State of a widget instance.
#[derive(Clone, Debug)]
pub struct WidgetState {
    /// Widget identifier.
    pub id: WidgetId,
    /// Widget kind.
    pub kind: WidgetKind,
    /// Whether the widget is visible.
    pub visible: bool,
    /// Whether the widget is collapsed.
    pub collapsed: bool,
    /// Document URI this widget is associated with.
    pub uri: String,
    /// Position in the document.
    pub position: Position,
    /// Widget-specific data.
    pub data: JsonValue,
    /// Version of the document when data was computed.
    pub version: i64,
}
impl WidgetState {
    /// Create a new widget state.
    pub fn new(id: WidgetId, kind: WidgetKind, uri: &str, position: Position) -> Self {
        Self {
            id,
            kind,
            visible: true,
            collapsed: false,
            uri: uri.to_string(),
            position,
            data: JsonValue::Null,
            version: 0,
        }
    }
    /// Serialize to JSON.
    pub fn to_json(&self) -> JsonValue {
        JsonValue::Object(vec![
            ("id".to_string(), JsonValue::String(self.id.0.clone())),
            (
                "kind".to_string(),
                JsonValue::String(self.kind.as_str().to_string()),
            ),
            ("visible".to_string(), JsonValue::Bool(self.visible)),
            ("collapsed".to_string(), JsonValue::Bool(self.collapsed)),
            ("uri".to_string(), JsonValue::String(self.uri.clone())),
            ("position".to_string(), self.position.to_json()),
            ("data".to_string(), self.data.clone()),
            (
                "version".to_string(),
                JsonValue::Number(self.version as f64),
            ),
        ])
    }
}
/// Event targeting information.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct TargetedEvent {
    pub target: WidgetId,
    pub event: WidgetEvent,
}
/// Category of tactic suggestion.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(clippy::enum_variant_names)]
pub enum SuggestionCategory {
    /// Based on goal shape.
    GoalBased,
    /// Based on hypothesis usage.
    HypothesisBased,
    /// Based on common patterns.
    PatternBased,
    /// Based on type information.
    TypeBased,
    /// Based on similar proofs.
    SimilarityBased,
}
impl SuggestionCategory {
    /// Return a string identifier.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::GoalBased => "goalBased",
            Self::HypothesisBased => "hypothesisBased",
            Self::PatternBased => "patternBased",
            Self::TypeBased => "typeBased",
            Self::SimilarityBased => "similarityBased",
        }
    }
}
/// A panel that contains multiple widgets.
#[allow(dead_code)]
pub struct WidgetPanel {
    pub title: String,
    pub widgets: Vec<WidgetId>,
    pub visible: bool,
    pub docked_position: PanelPosition,
}
impl WidgetPanel {
    /// Create a new panel.
    #[allow(dead_code)]
    pub fn new(title: impl Into<String>, position: PanelPosition) -> Self {
        Self {
            title: title.into(),
            widgets: vec![],
            visible: true,
            docked_position: position,
        }
    }
    /// Add a widget to this panel.
    #[allow(dead_code)]
    pub fn add_widget(&mut self, id: WidgetId) {
        self.widgets.push(id);
    }
    /// Toggle visibility.
    #[allow(dead_code)]
    pub fn toggle_visible(&mut self) {
        self.visible = !self.visible;
    }
}
/// Builds proof trees from document analysis.
pub struct ProofTreeBuilder<'a> {
    /// Reference to the environment.
    env: &'a Environment,
    /// Node ID counter.
    next_id: usize,
}
impl<'a> ProofTreeBuilder<'a> {
    /// Create a new proof tree builder.
    pub fn new(env: &'a Environment) -> Self {
        Self { env, next_id: 0 }
    }
    /// Generate a unique node ID.
    fn gen_id(&mut self) -> String {
        self.next_id += 1;
        format!("node_{}", self.next_id)
    }
    /// Build a proof tree for an entire document.
    pub fn build_document_tree(&mut self, doc: &Document) -> Vec<ProofTreeNode> {
        let analysis = analyze_document(&doc.uri, &doc.content, self.env);
        let mut trees = Vec::new();
        for def in &analysis.definitions {
            if def.kind == SymbolKind::Method {
                let tree = self.build_theorem_tree(doc, def);
                trees.push(tree);
            }
        }
        trees
    }
    /// Build a proof tree for a single theorem.
    fn build_theorem_tree(&mut self, doc: &Document, def: &DefinitionInfo) -> ProofTreeNode {
        let ty = def.ty.as_deref().unwrap_or("?");
        let mut children = Vec::new();
        let mut has_sorry = false;
        let mut in_proof = false;
        let start_line = def.range.start.line as usize;
        for line_num in start_line..doc.line_count() {
            if let Some(line) = doc.get_line(line_num as u32) {
                let trimmed = line.trim();
                if trimmed == "by" || trimmed.starts_with("by ") || trimmed.ends_with(" by") {
                    in_proof = true;
                    children.push(ProofTreeNode {
                        id: self.gen_id(),
                        label: "by".to_string(),
                        kind: ProofNodeKind::ByBlock,
                        status: ProofNodeStatus::InProgress,
                        range: Range::single_line(line_num as u32, 0, line.len() as u32),
                        children: Vec::new(),
                        info: None,
                    });
                    continue;
                }
                if in_proof && !trimmed.is_empty() {
                    let indent = line.len() - trimmed.len();
                    if indent == 0
                        && ["def", "theorem", "lemma", "axiom", "inductive"]
                            .iter()
                            .any(|kw| trimmed.starts_with(kw))
                    {
                        break;
                    }
                    let status = if trimmed == "sorry" {
                        has_sorry = true;
                        ProofNodeStatus::Admitted
                    } else if trimmed == "rfl" || trimmed == "trivial" || trimmed == "assumption" {
                        ProofNodeStatus::Proven
                    } else {
                        ProofNodeStatus::InProgress
                    };
                    let kind = if trimmed == "sorry" {
                        ProofNodeKind::Sorry
                    } else {
                        ProofNodeKind::Tactic
                    };
                    children.push(ProofTreeNode {
                        id: self.gen_id(),
                        label: trimmed.to_string(),
                        kind,
                        status,
                        range: Range::single_line(
                            line_num as u32,
                            indent as u32,
                            line.len() as u32,
                        ),
                        children: Vec::new(),
                        info: None,
                    });
                }
            }
        }
        let overall_status = if has_sorry {
            ProofNodeStatus::Admitted
        } else if children.iter().all(|c| c.status == ProofNodeStatus::Proven)
            && !children.is_empty()
        {
            ProofNodeStatus::Proven
        } else {
            ProofNodeStatus::InProgress
        };
        ProofTreeNode {
            id: self.gen_id(),
            label: format!("{} : {}", def.name, ty),
            kind: ProofNodeKind::Theorem,
            status: overall_status,
            range: def.range.clone(),
            children,
            info: Some(format!("theorem {}", def.name)),
        }
    }
    /// Render the full proof tree as JSON.
    pub fn render_document_tree(&mut self, doc: &Document) -> JsonValue {
        let trees = self.build_document_tree(doc);
        let total_nodes: usize = trees.iter().map(|t| t.node_count()).sum();
        let total_sorry: usize = trees.iter().map(|t| t.sorry_count()).sum();
        let all_proven = trees.iter().all(|t| t.all_proven());
        JsonValue::Object(vec![
            (
                "kind".to_string(),
                JsonValue::String("proofTree".to_string()),
            ),
            (
                "trees".to_string(),
                JsonValue::Array(trees.iter().map(|t| t.to_json()).collect()),
            ),
            (
                "totalNodes".to_string(),
                JsonValue::Number(total_nodes as f64),
            ),
            (
                "totalSorry".to_string(),
                JsonValue::Number(total_sorry as f64),
            ),
            ("allProven".to_string(), JsonValue::Bool(all_proven)),
            ("uri".to_string(), JsonValue::String(doc.uri.clone())),
        ])
    }
}
/// A tactic suggestion with metadata.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct TacticSuggestionItem {
    pub tactic: String,
    pub description: String,
    pub confidence: f64,
    pub closes_goal: bool,
    pub applicable: bool,
}
impl TacticSuggestionItem {
    /// Create a new suggestion.
    #[allow(dead_code)]
    pub fn new(tactic: impl Into<String>, description: impl Into<String>, confidence: f64) -> Self {
        Self {
            tactic: tactic.into(),
            description: description.into(),
            confidence: confidence.clamp(0.0, 1.0),
            closes_goal: false,
            applicable: true,
        }
    }
    /// Mark this suggestion as closing the goal.
    #[allow(dead_code)]
    pub fn closes_goal(mut self) -> Self {
        self.closes_goal = true;
        self
    }
}
/// A tactic suggestion with confidence level.
#[derive(Clone, Debug)]
pub struct TacticSuggestion {
    /// The suggested tactic text.
    pub tactic: String,
    /// Confidence level (0.0 to 1.0).
    pub confidence: f64,
    /// Explanation of why this tactic is suggested.
    pub explanation: String,
    /// Category of the suggestion.
    pub category: SuggestionCategory,
    /// Whether this suggestion would close the goal.
    pub closes_goal: bool,
}
impl TacticSuggestion {
    /// Serialize to JSON.
    pub fn to_json(&self) -> JsonValue {
        JsonValue::Object(vec![
            ("tactic".to_string(), JsonValue::String(self.tactic.clone())),
            ("confidence".to_string(), JsonValue::Number(self.confidence)),
            (
                "explanation".to_string(),
                JsonValue::String(self.explanation.clone()),
            ),
            (
                "category".to_string(),
                JsonValue::String(self.category.as_str().to_string()),
            ),
            ("closesGoal".to_string(), JsonValue::Bool(self.closes_goal)),
        ])
    }
}
/// A goal for display.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct GoalDisplay {
    pub index: usize,
    pub target: String,
    pub hypotheses: Vec<HypothesisDisplay>,
    pub tactic_state: String,
    pub is_closed: bool,
}
/// A hypothesis for display.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct HypothesisDisplay {
    pub name: String,
    pub type_str: String,
    pub value_str: Option<String>,
    pub is_local_def: bool,
}
/// Dispatches events to the appropriate widgets.
#[allow(dead_code)]
pub struct WidgetDispatcher {
    subscribers: std::collections::HashMap<WidgetId, Vec<WidgetEventResponse>>,
}
impl WidgetDispatcher {
    /// Create a new dispatcher.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            subscribers: std::collections::HashMap::new(),
        }
    }
    /// Record a response for a widget.
    #[allow(dead_code)]
    pub fn record_response(&mut self, id: WidgetId, response: WidgetEventResponse) {
        self.subscribers.entry(id).or_default().push(response);
    }
    /// Return all recorded responses for a widget.
    #[allow(dead_code)]
    pub fn responses_for(&self, id: &WidgetId) -> &[WidgetEventResponse] {
        self.subscribers
            .get(id)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }
    /// Clear all responses.
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.subscribers.clear();
    }
}
/// A snapshot of widget state for undo/redo history.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct WidgetSnapshot {
    pub widget_id: WidgetId,
    pub timestamp: std::time::Instant,
    pub rendered: String,
}
