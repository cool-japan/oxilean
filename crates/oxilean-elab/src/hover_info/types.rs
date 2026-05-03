//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;

use oxilean_kernel::*;
use std::collections::{HashMap, VecDeque};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HoverDiagnosticSeverity {
    Error,
    Warning,
    Info,
    Hint,
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HoverLinkKind {
    Definition,
    Reference,
    Documentation,
    Source,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct HoverAnnotation {
    pub kind: HoverAnnotationKind,
    pub description: String,
    pub extra: Option<String>,
}
#[allow(dead_code)]
impl HoverAnnotation {
    pub fn new(kind: HoverAnnotationKind, description: impl Into<String>) -> Self {
        HoverAnnotation {
            kind,
            description: description.into(),
            extra: None,
        }
    }
    pub fn with_extra(mut self, extra: impl Into<String>) -> Self {
        self.extra = Some(extra.into());
        self
    }
    pub fn is_implicit(&self) -> bool {
        matches!(
            self.kind,
            HoverAnnotationKind::ImplicitArgument | HoverAnnotationKind::InstanceArgument
        )
    }
}
#[allow(dead_code)]
pub struct HoverCache {
    entries: HashMap<(u32, u32), HoverInfo>,
    capacity: usize,
    access_order: Vec<(u32, u32)>,
}
#[allow(dead_code)]
impl HoverCache {
    pub fn new(capacity: usize) -> Self {
        HoverCache {
            entries: HashMap::new(),
            capacity,
            access_order: Vec::new(),
        }
    }
    pub fn get(&mut self, line: u32, col: u32) -> Option<&HoverInfo> {
        let key = (line, col);
        if self.entries.contains_key(&key) {
            self.access_order.retain(|k| k != &key);
            self.access_order.push(key);
            self.entries.get(&key)
        } else {
            None
        }
    }
    pub fn insert(&mut self, line: u32, col: u32, info: HoverInfo) {
        let key = (line, col);
        if self.entries.len() >= self.capacity && !self.entries.contains_key(&key) {
            if let Some(oldest) = self.access_order.first().cloned() {
                self.entries.remove(&oldest);
                self.access_order.remove(0);
            }
        }
        self.entries.insert(key, info);
        self.access_order.retain(|k| k != &key);
        self.access_order.push(key);
    }
    pub fn invalidate(&mut self, line: u32, col: u32) {
        let key = (line, col);
        self.entries.remove(&key);
        self.access_order.retain(|k| k != &key);
    }
    pub fn clear(&mut self) {
        self.entries.clear();
        self.access_order.clear();
    }
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct HoverStats {
    pub total_requests: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub index_hits: u64,
    pub index_misses: u64,
    pub total_entries: usize,
}
#[allow(dead_code)]
impl HoverStats {
    pub fn new() -> Self {
        HoverStats::default()
    }
    pub fn cache_hit_rate(&self) -> f64 {
        let total = self.cache_hits + self.cache_misses;
        if total == 0 {
            0.0
        } else {
            self.cache_hits as f64 / total as f64
        }
    }
    pub fn index_hit_rate(&self) -> f64 {
        let total = self.index_hits + self.index_misses;
        if total == 0 {
            0.0
        } else {
            self.index_hits as f64 / total as f64
        }
    }
    pub fn record_cache_hit(&mut self) {
        self.total_requests += 1;
        self.cache_hits += 1;
    }
    pub fn record_cache_miss(&mut self) {
        self.cache_misses += 1;
    }
    pub fn record_index_hit(&mut self) {
        self.index_hits += 1;
    }
    pub fn record_index_miss(&mut self) {
        self.index_misses += 1;
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct HoverResponse {
    pub contents: String,
    pub range: Option<HoverLocation>,
    pub annotations: Vec<HoverAnnotation>,
    pub links: Vec<HoverLink>,
    pub diagnostics: Vec<HoverDiagnostic>,
}
#[allow(dead_code)]
impl HoverResponse {
    pub fn new(contents: impl Into<String>) -> Self {
        HoverResponse {
            contents: contents.into(),
            range: None,
            annotations: Vec::new(),
            links: Vec::new(),
            diagnostics: Vec::new(),
        }
    }
    pub fn with_range(mut self, loc: HoverLocation) -> Self {
        self.range = Some(loc);
        self
    }
    pub fn with_annotation(mut self, ann: HoverAnnotation) -> Self {
        self.annotations.push(ann);
        self
    }
    pub fn with_link(mut self, link: HoverLink) -> Self {
        self.links.push(link);
        self
    }
    pub fn with_diagnostic(mut self, diag: HoverDiagnostic) -> Self {
        self.diagnostics.push(diag);
        self
    }
    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(|d| d.is_error())
    }
    pub fn annotation_count(&self) -> usize {
        self.annotations.len()
    }
    pub fn is_empty(&self) -> bool {
        self.contents.is_empty()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct HoverGoalInfo {
    pub goal_type: String,
    pub hypotheses: Vec<HoverHypothesisInfo>,
    pub goal_index: usize,
    pub total_goals: usize,
}
#[allow(dead_code)]
impl HoverGoalInfo {
    pub fn new(goal_type: impl Into<String>) -> Self {
        HoverGoalInfo {
            goal_type: goal_type.into(),
            hypotheses: Vec::new(),
            goal_index: 0,
            total_goals: 1,
        }
    }
    pub fn with_hyp(mut self, h: HoverHypothesisInfo) -> Self {
        self.hypotheses.push(h);
        self
    }
    pub fn with_indices(mut self, index: usize, total: usize) -> Self {
        self.goal_index = index;
        self.total_goals = total;
        self
    }
    pub fn render_plaintext(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "Goal {}/{}\n",
            self.goal_index + 1,
            self.total_goals
        ));
        for h in &self.hypotheses {
            out.push_str(&format!("  {} : {}\n", h.hyp_name, h.hyp_type));
        }
        out.push_str("  ⊢ ");
        out.push_str(&self.goal_type);
        out.push('\n');
        out
    }
    pub fn has_hypotheses(&self) -> bool {
        !self.hypotheses.is_empty()
    }
}
/// A pre-populated database of hover results, keyed by name.
pub struct HoverProvider {
    env: HashMap<String, HoverResult>,
}
impl HoverProvider {
    /// Create an empty `HoverProvider`.
    pub fn new() -> Self {
        HoverProvider {
            env: HashMap::new(),
        }
    }
    /// Register a name → result mapping.
    pub fn register(&mut self, name: &str, result: HoverResult) {
        self.env.insert(name.to_string(), result);
    }
    /// Look up an exact name.
    pub fn lookup(&self, name: &str) -> Option<&HoverResult> {
        self.env.get(name)
    }
    /// Return all results whose name starts with `prefix`.
    pub fn lookup_prefix(&self, prefix: &str) -> Vec<&HoverResult> {
        self.env
            .values()
            .filter(|r| r.name.starts_with(prefix))
            .collect()
    }
    /// Register a tactic with its documentation.
    pub fn register_tactic(&mut self, name: &str, doc: &str) {
        let result = HoverResult::new(HoverKind::Tactic, name).with_doc(doc);
        self.register(name, result);
    }
    /// Register a keyword with its documentation.
    pub fn register_keyword(&mut self, name: &str, doc: &str) {
        let result = HoverResult::new(HoverKind::Keyword, name).with_doc(doc);
        self.register(name, result);
    }
    /// Create a `HoverProvider` pre-populated with all known tactics.
    pub fn default_tactics() -> Self {
        let mut provider = HoverProvider::new();
        let tactics: &[(&str, &str)] = &[
            (
                "intro",
                "Introduce a hypothesis or variable into the context.",
            ),
            (
                "intros",
                "Introduce multiple hypotheses or variables at once.",
            ),
            ("exact", "Close the goal with an exact proof term."),
            (
                "assumption",
                "Close the goal using a hypothesis in the context.",
            ),
            ("apply", "Apply a lemma or function to the current goal."),
            ("refl", "Close a reflexivity goal `a = a`."),
            ("rw", "Rewrite the goal using an equality."),
            ("simp", "Simplify the goal using built-in simp lemmas."),
            ("simp_all", "Simplify using all hypotheses and simp lemmas."),
            ("ring", "Prove ring equalities automatically."),
            ("linarith", "Prove linear arithmetic goals automatically."),
            ("omega", "Decide linear integer/natural arithmetic."),
            ("norm_num", "Evaluate numeric expressions automatically."),
            (
                "constructor",
                "Apply the unique constructor of the goal type.",
            ),
            ("left", "Select the left branch of a disjunction goal."),
            ("right", "Select the right branch of a disjunction goal."),
            ("cases", "Case-split on an expression or hypothesis."),
            ("induction", "Perform induction on a term."),
            (
                "have",
                "Introduce a local lemma with the given name and type.",
            ),
            (
                "show",
                "Change the current goal to a definitionally equal type.",
            ),
            (
                "by_contra",
                "Introduce the negation of the goal as a hypothesis.",
            ),
            ("by_contradiction", "Same as by_contra."),
            ("contrapose", "Transform goal A → B to ¬B → ¬A."),
            (
                "push_neg",
                "Push negations inward through logical connectives.",
            ),
            ("exfalso", "Change the goal to False."),
            ("trivial", "Close trivial goals (True, refl, etc.)."),
            ("sorry", "Admit the current goal (placeholder)."),
            (
                "split",
                "Split an Iff goal into forward and backward goals.",
            ),
            ("clear", "Remove a hypothesis from the context."),
            ("revert", "Move a hypothesis back into the goal."),
            ("exists", "Provide a witness for an existential goal."),
            (
                "use",
                "Provide a witness for an existential goal (alias for exists).",
            ),
            (
                "obtain",
                "Destructs a hypothesis (like cases with a pattern).",
            ),
            ("repeat", "Repeat a tactic until it fails."),
            ("try", "Apply a tactic, succeeding even if it fails."),
            (
                "first",
                "Try each alternative tactic, using the first that succeeds.",
            ),
            ("all_goals", "Apply a tactic to all remaining goals."),
            (
                "field_simp",
                "Simplify field expressions by clearing denominators.",
            ),
            ("norm_cast", "Normalize casts between numeric types."),
            ("push_cast", "Push casts inward through expressions."),
            ("exact_mod_cast", "Close goal modulo cast normalization."),
            ("conv", "Enter conversion mode for targeted rewriting."),
            ("rename", "Rename a hypothesis in the context."),
        ];
        for (name, doc) in tactics {
            provider.register_tactic(name, doc);
        }
        provider
    }
}
#[allow(dead_code)]
pub struct HoverThrottler {
    min_interval_ms: u64,
    last_request_ms: u64,
    request_count: u64,
    first_request: bool,
}
#[allow(dead_code)]
impl HoverThrottler {
    pub fn new(min_interval_ms: u64) -> Self {
        HoverThrottler {
            min_interval_ms,
            last_request_ms: 0,
            request_count: 0,
            first_request: true,
        }
    }
    /// Returns true if the request should proceed (not throttled).
    pub fn should_proceed(&mut self, now_ms: u64) -> bool {
        if self.first_request || now_ms >= self.last_request_ms + self.min_interval_ms {
            self.last_request_ms = now_ms;
            self.request_count += 1;
            self.first_request = false;
            true
        } else {
            false
        }
    }
    pub fn request_count(&self) -> u64 {
        self.request_count
    }
    pub fn reset(&mut self) {
        self.last_request_ms = 0;
        self.request_count = 0;
        self.first_request = true;
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HoverLocation {
    pub line: u32,
    pub col: u32,
    pub end_line: u32,
    pub end_col: u32,
}
#[allow(dead_code)]
impl HoverLocation {
    pub fn new(line: u32, col: u32, end_line: u32, end_col: u32) -> Self {
        HoverLocation {
            line,
            col,
            end_line,
            end_col,
        }
    }
    pub fn single_line(line: u32, col_start: u32, col_end: u32) -> Self {
        HoverLocation {
            line,
            col: col_start,
            end_line: line,
            end_col: col_end,
        }
    }
    pub fn contains(&self, line: u32, col: u32) -> bool {
        if line < self.line || line > self.end_line {
            return false;
        }
        if line == self.line && col < self.col {
            return false;
        }
        if line == self.end_line && col > self.end_col {
            return false;
        }
        true
    }
    pub fn is_single_line(&self) -> bool {
        self.line == self.end_line
    }
}
#[allow(dead_code)]
pub struct TacticSuggestionEngine {
    rules: Vec<(fn(&str) -> bool, &'static str, &'static str, f32)>,
}
#[allow(dead_code)]
impl TacticSuggestionEngine {
    pub fn new() -> Self {
        let rules: Vec<(fn(&str) -> bool, &'static str, &'static str, f32)> = vec![
            (
                |g| g.contains(" = "),
                "rfl",
                "goal is an equality — try rfl",
                0.9,
            ),
            (
                |g| g.contains("∧") || g.contains("/\\"),
                "constructor",
                "goal is a conjunction",
                0.85,
            ),
            (
                |g| g.contains("∨") || g.contains("\\/"),
                "left",
                "goal is a disjunction",
                0.7,
            ),
            (
                |g| g.contains("∃"),
                "exists",
                "goal is existential — provide a witness",
                0.8,
            ),
            (
                |g| g.starts_with("¬") || g.starts_with("Not"),
                "by_contra",
                "goal is negation",
                0.75,
            ),
            (
                |g| g.contains("Nat") || g.contains("Int"),
                "omega",
                "arithmetic goal — try omega",
                0.7,
            ),
            (
                |g| g.contains("↔") || g.contains("Iff"),
                "constructor",
                "goal is Iff",
                0.8,
            ),
            (|g| g.contains("True"), "trivial", "goal is True", 0.95),
        ];
        TacticSuggestionEngine { rules }
    }
    pub fn suggest(&self, goal: &str) -> Vec<HoverTacticSuggestion> {
        let mut suggestions: Vec<HoverTacticSuggestion> = self
            .rules
            .iter()
            .filter(|(pred, _, _, _)| pred(goal))
            .map(|(_, tactic, reason, conf)| HoverTacticSuggestion::new(*tactic, *reason, *conf))
            .collect();
        suggestions.sort_by(|a, b| {
            b.confidence
                .partial_cmp(&a.confidence)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        suggestions
    }
    pub fn best_suggestion(&self, goal: &str) -> Option<HoverTacticSuggestion> {
        self.suggest(goal).into_iter().next()
    }
}
#[allow(dead_code)]
pub struct HoverFormatter {
    format: HoverFormat,
    max_doc_length: usize,
    show_universe: bool,
}
#[allow(dead_code)]
impl HoverFormatter {
    pub fn new(format: HoverFormat) -> Self {
        HoverFormatter {
            format,
            max_doc_length: 500,
            show_universe: false,
        }
    }
    pub fn with_max_doc_length(mut self, n: usize) -> Self {
        self.max_doc_length = n;
        self
    }
    pub fn with_show_universe(mut self, show: bool) -> Self {
        self.show_universe = show;
        self
    }
    pub fn format_info(&self, info: &HoverInfo) -> String {
        match self.format {
            HoverFormat::Markdown => HoverMarkdown::render_info(info),
            HoverFormat::PlainText => {
                let mut out = format!("{}: {}\n", info.kind.as_str(), info.name);
                if let Some(ty) = &info.type_signature {
                    out.push_str(&format!("  : {}\n", ty));
                }
                if let Some(doc) = &info.doc_string {
                    let truncated = if doc.len() > self.max_doc_length {
                        format!("{}…", &doc[..self.max_doc_length])
                    } else {
                        doc.clone()
                    };
                    out.push_str(&truncated);
                    out.push('\n');
                }
                out
            }
            HoverFormat::Html => {
                let mut out = format!(
                    "<b>{}</b> <code>{}</code><br/>",
                    info.kind.as_str(),
                    info.name
                );
                if let Some(ty) = &info.type_signature {
                    out.push_str(&format!("<pre>{}</pre>", ty));
                }
                if let Some(doc) = &info.doc_string {
                    out.push_str(&format!("<p>{}</p>", doc));
                }
                out
            }
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct HoverRendererConfig {
    pub format: HoverFormat,
    pub show_source_location: bool,
    pub show_type_signature: bool,
    pub show_doc_string: bool,
    pub show_module: bool,
    pub max_output_chars: usize,
}
#[allow(dead_code)]
impl HoverRendererConfig {
    pub fn new() -> Self {
        HoverRendererConfig {
            format: HoverFormat::Markdown,
            show_source_location: false,
            show_type_signature: true,
            show_doc_string: true,
            show_module: true,
            max_output_chars: 2000,
        }
    }
    pub fn with_format(mut self, fmt: HoverFormat) -> Self {
        self.format = fmt;
        self
    }
    pub fn with_source_location(mut self, show: bool) -> Self {
        self.show_source_location = show;
        self
    }
    pub fn compact(mut self) -> Self {
        self.show_source_location = false;
        self.show_module = false;
        self.max_output_chars = 500;
        self
    }
}
/// Provides context-aware hover results by combining a `HoverProvider` with
/// on-the-fly analysis of the surrounding source.
pub struct ContextualHover {
    provider: HoverProvider,
}
impl ContextualHover {
    /// Create a `ContextualHover` backed by the default tactic database.
    pub fn new() -> Self {
        ContextualHover {
            provider: HoverProvider::default_tactics(),
        }
    }
    /// Return a hover result for `word` at the given source position.
    /// `_source`, `_line`, and `_col` are available for future context analysis.
    pub fn hover_at_word(
        &self,
        word: &str,
        _source: &str,
        _line: u32,
        _col: u32,
    ) -> Option<HoverResult> {
        if let Some(r) = self.provider.lookup(word) {
            return Some(r.clone());
        }
        if Self::is_builtin_type(word) {
            return Some(
                HoverResult::new(HoverKind::Definition, word)
                    .with_doc(&format!("`{}` is a built-in type.", word)),
            );
        }
        None
    }
    /// Return true if `word` is a recognized tactic keyword.
    pub fn is_tactic_keyword(word: &str) -> bool {
        matches!(
            word,
            "intro"
                | "intros"
                | "exact"
                | "assumption"
                | "apply"
                | "refl"
                | "rw"
                | "simp"
                | "simp_all"
                | "ring"
                | "linarith"
                | "omega"
                | "norm_num"
                | "constructor"
                | "left"
                | "right"
                | "cases"
                | "induction"
                | "have"
                | "show"
                | "by_contra"
                | "by_contradiction"
                | "contrapose"
                | "push_neg"
                | "exfalso"
                | "trivial"
                | "sorry"
                | "split"
                | "clear"
                | "revert"
                | "exists"
                | "use"
                | "obtain"
                | "repeat"
                | "try"
                | "first"
                | "all_goals"
                | "field_simp"
                | "norm_cast"
                | "push_cast"
                | "exact_mod_cast"
                | "conv"
                | "rename"
        )
    }
    /// Return true if `word` is a recognized built-in type name.
    pub fn is_builtin_type(word: &str) -> bool {
        matches!(
            word,
            "Nat"
                | "Int"
                | "Bool"
                | "String"
                | "Char"
                | "Float"
                | "UInt8"
                | "UInt16"
                | "UInt32"
                | "UInt64"
                | "Int8"
                | "Int16"
                | "Int32"
                | "Int64"
                | "List"
                | "Option"
                | "Result"
                | "Array"
                | "Type"
                | "Prop"
                | "Sort"
                | "True"
                | "False"
                | "And"
                | "Or"
                | "Not"
                | "Iff"
                | "Eq"
                | "Ne"
                | "Exists"
                | "Unit"
                | "Empty"
                | "Prod"
                | "Sum"
                | "Fin"
                | "Subtype"
        )
    }
}
#[allow(dead_code)]
pub struct HoverEnricher {
    tactic_docs: HashMap<String, String>,
    keyword_docs: HashMap<String, String>,
}
#[allow(dead_code)]
impl HoverEnricher {
    pub fn new() -> Self {
        let mut tactic_docs = HashMap::new();
        let mut keyword_docs = HashMap::new();
        tactic_docs.insert(
            "simp".to_string(),
            "Simplifies the goal using a set of lemmas and built-in reduction rules.".to_string(),
        );
        tactic_docs.insert(
            "omega".to_string(),
            "Solves linear arithmetic goals over integers and naturals.".to_string(),
        );
        tactic_docs.insert(
            "ring".to_string(),
            "Proves equalities in commutative (semi)rings by normalization.".to_string(),
        );
        tactic_docs.insert(
            "linarith".to_string(),
            "Proves linear arithmetic goals using the Farkas lemma.".to_string(),
        );
        tactic_docs.insert(
            "exact".to_string(),
            "Closes the goal by providing an exact proof term.".to_string(),
        );
        tactic_docs.insert(
            "apply".to_string(),
            "Applies a function or lemma, generating subgoals for each argument.".to_string(),
        );
        tactic_docs.insert(
            "intro".to_string(),
            "Introduces a binder from a Pi type into the context.".to_string(),
        );
        tactic_docs.insert(
            "cases".to_string(),
            "Case splits on an inductive type or proposition.".to_string(),
        );
        tactic_docs.insert(
            "induction".to_string(),
            "Applies the recursor of an inductive type.".to_string(),
        );
        tactic_docs.insert(
            "rw".to_string(),
            "Rewrites the goal using an equality hypothesis.".to_string(),
        );
        keyword_docs.insert(
            "theorem".to_string(),
            "Declares a theorem (proof obligation).".to_string(),
        );
        keyword_docs.insert(
            "def".to_string(),
            "Defines a new function or value.".to_string(),
        );
        keyword_docs.insert(
            "axiom".to_string(),
            "Postulates a proposition without proof.".to_string(),
        );
        keyword_docs.insert(
            "structure".to_string(),
            "Defines a record type with named fields.".to_string(),
        );
        keyword_docs.insert(
            "inductive".to_string(),
            "Defines an inductive (algebraic) data type.".to_string(),
        );
        keyword_docs.insert(
            "namespace".to_string(),
            "Opens a namespace for scoped definitions.".to_string(),
        );
        keyword_docs.insert(
            "section".to_string(),
            "Defines a section for scoped variables.".to_string(),
        );
        keyword_docs.insert(
            "variable".to_string(),
            "Declares a section variable.".to_string(),
        );
        keyword_docs.insert("import".to_string(), "Imports a module.".to_string());
        HoverEnricher {
            tactic_docs,
            keyword_docs,
        }
    }
    pub fn enrich(&self, info: &mut HoverInfo) {
        if info.doc_string.is_some() {
            return;
        }
        let doc = match info.kind {
            HoverKind::Tactic => self.tactic_docs.get(&info.name).cloned(),
            HoverKind::Keyword => self.keyword_docs.get(&info.name).cloned(),
            _ => None,
        };
        if let Some(d) = doc {
            info.doc_string = Some(d);
        }
    }
    pub fn known_tactic(&self, name: &str) -> bool {
        self.tactic_docs.contains_key(name)
    }
    pub fn known_keyword(&self, name: &str) -> bool {
        self.keyword_docs.contains_key(name)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct HoverDocReference {
    pub name: String,
    pub url: String,
    pub section: Option<String>,
}
#[allow(dead_code)]
impl HoverDocReference {
    pub fn new(name: impl Into<String>, url: impl Into<String>) -> Self {
        HoverDocReference {
            name: name.into(),
            url: url.into(),
            section: None,
        }
    }
    pub fn with_section(mut self, section: impl Into<String>) -> Self {
        self.section = Some(section.into());
        self
    }
    pub fn full_url(&self) -> String {
        if let Some(sec) = &self.section {
            format!("{}#{}", self.url, sec)
        } else {
            self.url.clone()
        }
    }
}
#[allow(dead_code)]
pub struct HoverInfoBuilder {
    name: String,
    kind: HoverKind,
    type_sig: Option<String>,
    doc: Option<String>,
    module: Option<String>,
}
#[allow(dead_code)]
impl HoverInfoBuilder {
    pub fn new(name: impl Into<String>, kind: HoverKind) -> Self {
        HoverInfoBuilder {
            name: name.into(),
            kind,
            type_sig: None,
            doc: None,
            module: None,
        }
    }
    pub fn type_sig(mut self, sig: impl Into<String>) -> Self {
        self.type_sig = Some(sig.into());
        self
    }
    pub fn doc(mut self, doc: impl Into<String>) -> Self {
        self.doc = Some(doc.into());
        self
    }
    pub fn module(mut self, m: impl Into<String>) -> Self {
        self.module = Some(m.into());
        self
    }
    pub fn build(self) -> HoverInfo {
        HoverInfo {
            name: self.name,
            kind: self.kind,
            type_signature: self.type_sig,
            doc_string: self.doc,
            module_path: self.module,
        }
    }
}
#[allow(dead_code)]
pub struct HoverDiagnosticCollection {
    diagnostics: Vec<HoverDiagnostic>,
}
#[allow(dead_code)]
impl HoverDiagnosticCollection {
    pub fn new() -> Self {
        HoverDiagnosticCollection {
            diagnostics: Vec::new(),
        }
    }
    pub fn add(&mut self, diag: HoverDiagnostic) {
        self.diagnostics.push(diag);
    }
    pub fn errors(&self) -> Vec<&HoverDiagnostic> {
        self.diagnostics.iter().filter(|d| d.is_error()).collect()
    }
    pub fn at(&self, line: u32, col: u32) -> Vec<&HoverDiagnostic> {
        self.diagnostics
            .iter()
            .filter(|d| d.location.contains(line, col))
            .collect()
    }
    pub fn count(&self) -> usize {
        self.diagnostics.len()
    }
    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(|d| d.is_error())
    }
    pub fn clear(&mut self) {
        self.diagnostics.clear();
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct HoverHypothesisInfo {
    pub hyp_name: String,
    pub hyp_type: String,
    pub is_local_def: bool,
    pub value: Option<String>,
}
#[allow(dead_code)]
impl HoverHypothesisInfo {
    pub fn new(hyp_name: impl Into<String>, hyp_type: impl Into<String>) -> Self {
        HoverHypothesisInfo {
            hyp_name: hyp_name.into(),
            hyp_type: hyp_type.into(),
            is_local_def: false,
            value: None,
        }
    }
    pub fn local_def(mut self, value: impl Into<String>) -> Self {
        self.is_local_def = true;
        self.value = Some(value.into());
        self
    }
    pub fn to_hover_info(&self) -> HoverInfo {
        let doc = if let Some(val) = &self.value {
            Some(format!("local def := {}", val))
        } else {
            None
        };
        HoverInfo {
            name: self.hyp_name.clone(),
            kind: HoverKind::LocalVar,
            type_signature: Some(self.hyp_type.clone()),
            doc_string: doc,
            module_path: None,
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct HoverModuleInfo {
    pub module_name: String,
    pub file_path: Option<String>,
    pub exports: Vec<String>,
    pub doc: Option<String>,
}
#[allow(dead_code)]
impl HoverModuleInfo {
    pub fn new(module_name: impl Into<String>) -> Self {
        HoverModuleInfo {
            module_name: module_name.into(),
            file_path: None,
            exports: Vec::new(),
            doc: None,
        }
    }
    pub fn with_file(mut self, path: impl Into<String>) -> Self {
        self.file_path = Some(path.into());
        self
    }
    pub fn with_export(mut self, name: impl Into<String>) -> Self {
        self.exports.push(name.into());
        self
    }
    pub fn with_doc(mut self, doc: impl Into<String>) -> Self {
        self.doc = Some(doc.into());
        self
    }
    pub fn export_count(&self) -> usize {
        self.exports.len()
    }
    pub fn to_hover_info(&self) -> HoverInfo {
        HoverInfo {
            name: self.module_name.clone(),
            kind: HoverKind::Keyword,
            type_signature: None,
            doc_string: self
                .doc
                .clone()
                .or_else(|| Some(format!("{} exports", self.exports.len()))),
            module_path: self.file_path.clone(),
        }
    }
}
#[allow(dead_code)]
pub struct HoverPerformanceMonitor {
    samples: Vec<u64>,
    max_samples: usize,
}
#[allow(dead_code)]
impl HoverPerformanceMonitor {
    pub fn new(max_samples: usize) -> Self {
        HoverPerformanceMonitor {
            samples: Vec::with_capacity(max_samples),
            max_samples,
        }
    }
    pub fn record(&mut self, latency_us: u64) {
        if self.samples.len() >= self.max_samples {
            self.samples.remove(0);
        }
        self.samples.push(latency_us);
    }
    pub fn mean_us(&self) -> f64 {
        if self.samples.is_empty() {
            return 0.0;
        }
        self.samples.iter().sum::<u64>() as f64 / self.samples.len() as f64
    }
    pub fn max_us(&self) -> u64 {
        self.samples.iter().copied().max().unwrap_or(0)
    }
    pub fn min_us(&self) -> u64 {
        self.samples.iter().copied().min().unwrap_or(0)
    }
    pub fn p95_us(&self) -> u64 {
        if self.samples.is_empty() {
            return 0;
        }
        let mut sorted = self.samples.clone();
        sorted.sort_unstable();
        let idx = ((sorted.len() as f64) * 0.95) as usize;
        sorted[idx.min(sorted.len() - 1)]
    }
    pub fn sample_count(&self) -> usize {
        self.samples.len()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HoverAnnotationKind {
    TypeAscription,
    ImplicitArgument,
    InstanceArgument,
    CoercedExpr,
    ExpectedType,
    SynthesizedTerm,
}
/// Rich hover information used by IDE-oriented components in this module.
/// (Distinct from `info_tree::HoverInfo` which uses kernel `Expr` types.)
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct HoverInfo {
    /// The entity name as a display string.
    pub name: String,
    /// The kind of entity (definition, theorem, tactic, …).
    pub kind: HoverKind,
    /// The type signature as a pretty-printed string, if available.
    pub type_signature: Option<String>,
    /// Documentation string, if available.
    pub doc_string: Option<String>,
    /// Fully-qualified module path, if available.
    pub module_path: Option<String>,
}
#[allow(dead_code)]
impl HoverInfo {
    /// Create a minimal `HoverInfo` with only name and kind.
    pub fn new(name: impl Into<String>, kind: HoverKind) -> Self {
        Self {
            name: name.into(),
            kind,
            type_signature: None,
            doc_string: None,
            module_path: None,
        }
    }
    /// Return true if a type signature is available.
    pub fn has_type(&self) -> bool {
        self.type_signature.is_some()
    }
    /// Return true if documentation is available.
    pub fn has_doc(&self) -> bool {
        self.doc_string.is_some()
    }
}
#[allow(dead_code)]
pub struct HoverRegionMerger;
#[allow(dead_code)]
impl HoverRegionMerger {
    /// Merges a list of (location, info) pairs, preferring narrower ranges on overlap.
    pub fn merge(entries: Vec<HoverEntry>) -> Vec<HoverEntry> {
        let mut seen_names = std::collections::HashSet::new();
        entries
            .into_iter()
            .filter(|e| seen_names.insert(e.info.name.clone()))
            .collect()
    }
    /// Returns entries sorted by location (line, col).
    pub fn sorted(entries: Vec<HoverEntry>) -> Vec<HoverEntry> {
        let mut v = entries;
        v.sort_by_key(|e| (e.location.line, e.location.col));
        v
    }
}
#[allow(dead_code)]
pub struct HoverIndex {
    entries: Vec<HoverEntry>,
}
#[allow(dead_code)]
impl HoverIndex {
    pub fn new() -> Self {
        HoverIndex {
            entries: Vec::new(),
        }
    }
    pub fn insert(&mut self, entry: HoverEntry) {
        self.entries.push(entry);
    }
    pub fn query(&self, line: u32, col: u32) -> Option<&HoverInfo> {
        self.entries
            .iter()
            .filter(|e| e.at_cursor(line, col))
            .min_by_key(|e| {
                let loc = &e.location;
                let row_span = loc.end_line - loc.line;
                let col_span = if row_span == 0 {
                    loc.end_col - loc.col
                } else {
                    u32::MAX
                };
                (row_span, col_span)
            })
            .map(|e| &e.info)
    }
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    pub fn clear(&mut self) {
        self.entries.clear();
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct HoverDiagnostic {
    pub severity: HoverDiagnosticSeverity,
    pub message: String,
    pub location: HoverLocation,
    pub code: Option<String>,
}
#[allow(dead_code)]
impl HoverDiagnostic {
    pub fn new(
        severity: HoverDiagnosticSeverity,
        message: impl Into<String>,
        location: HoverLocation,
    ) -> Self {
        HoverDiagnostic {
            severity,
            message: message.into(),
            location,
            code: None,
        }
    }
    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }
    pub fn is_error(&self) -> bool {
        self.severity == HoverDiagnosticSeverity::Error
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct HoverDecorationConfig {
    pub border_style: String,
    pub background_color: Option<String>,
    pub font_style: String,
    pub max_width: Option<u32>,
    pub max_height: Option<u32>,
}
#[allow(dead_code)]
impl HoverDecorationConfig {
    pub fn new() -> Self {
        HoverDecorationConfig {
            border_style: "solid".to_string(),
            background_color: None,
            font_style: "normal".to_string(),
            max_width: Some(600),
            max_height: Some(300),
        }
    }
    pub fn with_background(mut self, color: impl Into<String>) -> Self {
        self.background_color = Some(color.into());
        self
    }
    pub fn with_max_width(mut self, w: u32) -> Self {
        self.max_width = Some(w);
        self
    }
    pub fn with_font_style(mut self, style: impl Into<String>) -> Self {
        self.font_style = style.into();
        self
    }
}
/// The kind of entity being hovered over.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HoverKind {
    Definition,
    Theorem,
    Axiom,
    LocalVar,
    Constructor,
    Field,
    Tactic,
    Universe,
    Keyword,
}
impl HoverKind {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            HoverKind::Definition => "def",
            HoverKind::Theorem => "theorem",
            HoverKind::Axiom => "axiom",
            HoverKind::LocalVar => "variable",
            HoverKind::Constructor => "constructor",
            HoverKind::Field => "field",
            HoverKind::Tactic => "tactic",
            HoverKind::Universe => "universe",
            HoverKind::Keyword => "keyword",
        }
    }
}
#[allow(dead_code)]
pub struct HoverRequestContext {
    pub file_uri: String,
    pub line: u32,
    pub col: u32,
    pub trigger_char: Option<char>,
    pub client_supports_markdown: bool,
}
#[allow(dead_code)]
impl HoverRequestContext {
    pub fn new(file_uri: impl Into<String>, line: u32, col: u32) -> Self {
        HoverRequestContext {
            file_uri: file_uri.into(),
            line,
            col,
            trigger_char: None,
            client_supports_markdown: true,
        }
    }
    pub fn with_trigger(mut self, ch: char) -> Self {
        self.trigger_char = Some(ch);
        self
    }
    pub fn without_markdown(mut self) -> Self {
        self.client_supports_markdown = false;
        self
    }
    pub fn preferred_format(&self) -> HoverFormat {
        if self.client_supports_markdown {
            HoverFormat::Markdown
        } else {
            HoverFormat::PlainText
        }
    }
}
#[allow(dead_code)]
pub struct HoverDocReferenceIndex {
    refs: HashMap<String, HoverDocReference>,
}
#[allow(dead_code)]
impl HoverDocReferenceIndex {
    pub fn new() -> Self {
        HoverDocReferenceIndex {
            refs: HashMap::new(),
        }
    }
    pub fn insert(&mut self, r: HoverDocReference) {
        self.refs.insert(r.name.clone(), r);
    }
    pub fn lookup(&self, name: &str) -> Option<&HoverDocReference> {
        self.refs.get(name)
    }
    pub fn len(&self) -> usize {
        self.refs.len()
    }
    pub fn is_empty(&self) -> bool {
        self.refs.is_empty()
    }
}
#[allow(dead_code)]
pub struct HoverRenderer {
    config: HoverRendererConfig,
    enricher: HoverEnricher,
    doc_index: HoverDocReferenceIndex,
}
#[allow(dead_code)]
impl HoverRenderer {
    pub fn new(config: HoverRendererConfig) -> Self {
        HoverRenderer {
            config,
            enricher: HoverEnricher::new(),
            doc_index: HoverDocReferenceIndex::new(),
        }
    }
    pub fn with_doc_ref(mut self, r: HoverDocReference) -> Self {
        self.doc_index.insert(r);
        self
    }
    pub fn render(&self, info: &HoverInfo) -> String {
        let mut info = info.clone();
        self.enricher.enrich(&mut info);
        let formatter = HoverFormatter::new(self.config.format)
            .with_max_doc_length(self.config.max_output_chars);
        let mut out = formatter.format_info(&info);
        if self.config.show_module {
            if let Some(m) = &info.module_path {
                out.push_str(&format!("\n*module: {}*", m));
            }
        }
        if let Some(doc_ref) = self.doc_index.lookup(&info.name) {
            let link = HoverLink::to_docs(&info.name, doc_ref.full_url());
            out.push_str(&format!("\n{}", link.to_markdown()));
        }
        if out.len() > self.config.max_output_chars {
            out.truncate(self.config.max_output_chars);
            out.push('…');
        }
        out
    }
}
#[allow(dead_code)]
pub struct HoverHistory {
    positions: std::collections::VecDeque<(u32, u32)>,
    max_size: usize,
}
#[allow(dead_code)]
impl HoverHistory {
    pub fn new(max_size: usize) -> Self {
        HoverHistory {
            positions: std::collections::VecDeque::with_capacity(max_size),
            max_size,
        }
    }
    pub fn push(&mut self, line: u32, col: u32) {
        if self.positions.len() >= self.max_size {
            self.positions.pop_front();
        }
        self.positions.push_back((line, col));
    }
    pub fn last(&self) -> Option<(u32, u32)> {
        self.positions.back().copied()
    }
    pub fn len(&self) -> usize {
        self.positions.len()
    }
    pub fn is_empty(&self) -> bool {
        self.positions.is_empty()
    }
    pub fn clear(&mut self) {
        self.positions.clear();
    }
    pub fn positions(&self) -> impl Iterator<Item = &(u32, u32)> {
        self.positions.iter()
    }
    pub fn unique_lines(&self) -> Vec<u32> {
        let mut lines: Vec<u32> = self.positions.iter().map(|(l, _)| *l).collect();
        lines.sort_unstable();
        lines.dedup();
        lines
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct HoverEntry {
    pub location: HoverLocation,
    pub info: HoverInfo,
}
#[allow(dead_code)]
impl HoverEntry {
    pub fn new(location: HoverLocation, info: HoverInfo) -> Self {
        HoverEntry { location, info }
    }
    pub fn at_cursor(&self, line: u32, col: u32) -> bool {
        self.location.contains(line, col)
    }
}
#[allow(dead_code)]
pub struct HoverMarkdown;
#[allow(dead_code)]
impl HoverMarkdown {
    pub fn render_info(info: &HoverInfo) -> String {
        let mut out = String::new();
        out.push_str(&format!("**{}** `{}`\n\n", info.kind.as_str(), info.name));
        if let Some(ty) = &info.type_signature {
            out.push_str(&format!("```lean\n{}\n```\n\n", ty));
        }
        if let Some(doc) = &info.doc_string {
            out.push_str(doc);
            out.push('\n');
        }
        out
    }
    pub fn render_code_block(code: &str) -> String {
        format!("```lean\n{}\n```", code)
    }
    pub fn render_section(title: &str, content: &str) -> String {
        format!("### {}\n\n{}\n", title, content)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct HoverTacticSuggestion {
    pub tactic: String,
    pub reason: String,
    pub confidence: f32,
}
#[allow(dead_code)]
impl HoverTacticSuggestion {
    pub fn new(tactic: impl Into<String>, reason: impl Into<String>, confidence: f32) -> Self {
        HoverTacticSuggestion {
            tactic: tactic.into(),
            reason: reason.into(),
            confidence,
        }
    }
}
/// The result of a hover query, containing all information to display.
#[derive(Debug, Clone)]
pub struct HoverResult {
    pub kind: HoverKind,
    pub name: String,
    pub type_signature: String,
    pub documentation: String,
    pub source_location: Option<String>,
}
impl HoverResult {
    /// Create a new `HoverResult` with the given kind and name.
    pub fn new(kind: HoverKind, name: &str) -> Self {
        HoverResult {
            kind,
            name: name.to_string(),
            type_signature: String::new(),
            documentation: String::new(),
            source_location: None,
        }
    }
    /// Set the type signature.
    pub fn with_type(mut self, ty: &str) -> Self {
        self.type_signature = ty.to_string();
        self
    }
    /// Set the documentation string.
    pub fn with_doc(mut self, doc: &str) -> Self {
        self.documentation = doc.to_string();
        self
    }
    /// Set the source location as "file:line:col".
    pub fn with_location(mut self, loc: &str) -> Self {
        self.source_location = Some(loc.to_string());
        self
    }
    /// Format the hover result as a Markdown string suitable for LSP responses.
    pub fn to_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("**{}** `{}`\n\n", self.kind.as_str(), self.name));
        if !self.type_signature.is_empty() {
            out.push_str("```lean\n");
            out.push_str(&self.type_signature);
            out.push_str("\n```\n\n");
        }
        if !self.documentation.is_empty() {
            out.push_str(&self.documentation);
            out.push('\n');
        }
        if let Some(ref loc) = self.source_location {
            out.push_str(&format!("\n*Defined at: {}*\n", loc));
        }
        out
    }
    /// Format the hover result as plain text.
    pub fn to_plain_text(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("[{}] {}", self.kind.as_str(), self.name));
        if !self.type_signature.is_empty() {
            out.push_str(&format!(" : {}", self.type_signature));
        }
        if !self.documentation.is_empty() {
            out.push_str(&format!("\n{}", self.documentation));
        }
        if let Some(ref loc) = self.source_location {
            out.push_str(&format!("\nDefined at: {}", loc));
        }
        out
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct HoverLink {
    pub label: String,
    pub target: String,
    pub kind: HoverLinkKind,
}
#[allow(dead_code)]
impl HoverLink {
    pub fn to_definition(label: impl Into<String>, target: impl Into<String>) -> Self {
        HoverLink {
            label: label.into(),
            target: target.into(),
            kind: HoverLinkKind::Definition,
        }
    }
    pub fn to_docs(label: impl Into<String>, url: impl Into<String>) -> Self {
        HoverLink {
            label: label.into(),
            target: url.into(),
            kind: HoverLinkKind::Documentation,
        }
    }
    pub fn to_markdown(&self) -> String {
        format!("[{}]({})", self.label, self.target)
    }
}
#[allow(dead_code)]
pub struct HoverTypeSignatureParser;
#[allow(dead_code)]
impl HoverTypeSignatureParser {
    /// Splits a type signature at top-level arrows.
    pub fn split_arrows(sig: &str) -> Vec<String> {
        let mut parts = Vec::new();
        let mut depth = 0usize;
        let mut start = 0;
        let bytes = sig.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            match bytes[i] {
                b'(' | b'[' | b'{' => depth += 1,
                b')' | b']' | b'}' if depth > 0 => {
                    depth -= 1;
                }
                b'-' if depth == 0 && i + 1 < bytes.len() && bytes[i + 1] == b'>' => {
                    parts.push(sig[start..i].trim().to_string());
                    start = i + 2;
                    i += 2;
                    continue;
                }
                _ => {}
            }
            i += 1;
        }
        let last = sig[start..].trim().to_string();
        if !last.is_empty() {
            parts.push(last);
        }
        parts
    }
    /// Counts the arity (number of arguments) from a type signature.
    pub fn arity(sig: &str) -> usize {
        let parts = Self::split_arrows(sig);
        if parts.len() <= 1 {
            0
        } else {
            parts.len() - 1
        }
    }
    /// Extracts the return type (last component).
    pub fn return_type(sig: &str) -> String {
        let parts = Self::split_arrows(sig);
        parts.last().cloned().unwrap_or_default()
    }
    /// Checks if a type signature is a proposition (ends with Prop).
    pub fn is_prop(sig: &str) -> bool {
        let ret = Self::return_type(sig);
        ret.trim() == "Prop" || ret.trim() == "Bool"
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HoverFormat {
    PlainText,
    Markdown,
    Html,
}
#[allow(dead_code)]
pub struct HoverSession {
    cache: HoverCache,
    index: HoverIndex,
    enricher: HoverEnricher,
    formatter: HoverFormatter,
    request_count: u64,
    hit_count: u64,
}
#[allow(dead_code)]
impl HoverSession {
    pub fn new(format: HoverFormat) -> Self {
        HoverSession {
            cache: HoverCache::new(128),
            index: HoverIndex::new(),
            enricher: HoverEnricher::new(),
            formatter: HoverFormatter::new(format),
            request_count: 0,
            hit_count: 0,
        }
    }
    pub fn add_entry(&mut self, entry: HoverEntry) {
        self.index.insert(entry);
    }
    pub fn hover_at(&mut self, line: u32, col: u32) -> Option<String> {
        self.request_count += 1;
        if let Some(cached) = self.cache.get(line, col) {
            self.hit_count += 1;
            return Some(self.formatter.format_info(cached));
        }
        if let Some(info) = self.index.query(line, col) {
            let mut info = info.clone();
            self.enricher.enrich(&mut info);
            let rendered = self.formatter.format_info(&info);
            self.cache.insert(line, col, info);
            Some(rendered)
        } else {
            None
        }
    }
    pub fn cache_hit_rate(&self) -> f64 {
        if self.request_count == 0 {
            0.0
        } else {
            self.hit_count as f64 / self.request_count as f64
        }
    }
    pub fn invalidate_cache(&mut self) {
        self.cache.clear();
    }
    pub fn reset(&mut self) {
        self.cache.clear();
        self.index.clear();
        self.request_count = 0;
        self.hit_count = 0;
    }
}
#[allow(dead_code)]
pub struct HoverProviderChain {
    providers: Vec<Box<dyn HoverProviderTrait>>,
}
#[allow(dead_code)]
impl HoverProviderChain {
    pub fn new() -> Self {
        HoverProviderChain {
            providers: Vec::new(),
        }
    }
    pub fn add<P: HoverProviderTrait + 'static>(mut self, provider: P) -> Self {
        self.providers.push(Box::new(provider));
        self.providers.sort_by_key(|p| -p.priority());
        self
    }
    pub fn provide(&self, ctx: &HoverRequestContext) -> Option<HoverResponse> {
        for provider in &self.providers {
            if let Some(resp) = provider.provide(ctx) {
                return Some(resp);
            }
        }
        None
    }
    pub fn len(&self) -> usize {
        self.providers.len()
    }
    pub fn is_empty(&self) -> bool {
        self.providers.is_empty()
    }
}
