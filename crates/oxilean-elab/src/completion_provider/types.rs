//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;

use oxilean_kernel::*;
use std::collections::HashMap;

/// A middleware that adds a boosted score to all items matching a regex-like pattern.
pub struct BoostMiddleware {
    pub(super) pattern: String,
    pub(super) boost: f32,
}
impl BoostMiddleware {
    /// Create a boost middleware.
    pub fn new(pattern: impl Into<String>, boost: f32) -> Self {
        Self {
            pattern: pattern.into(),
            boost,
        }
    }
}
/// Statistics about completion usage.
#[derive(Debug, Clone, Default)]
pub struct CompletionStatistics {
    /// Total number of completion requests.
    pub total_requests: u64,
    /// Total number of accepted completions.
    pub total_accepted: u64,
    /// Per-label acceptance counts.
    pub label_counts: std::collections::HashMap<String, u64>,
}
impl CompletionStatistics {
    /// Create empty statistics.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record a completion request.
    pub fn record_request(&mut self) {
        self.total_requests += 1;
    }
    /// Record an accepted completion.
    pub fn record_acceptance(&mut self, label: impl Into<String>) {
        self.total_accepted += 1;
        *self.label_counts.entry(label.into()).or_default() += 1;
    }
    /// Return the acceptance rate in [0, 1].
    pub fn acceptance_rate(&self) -> f64 {
        if self.total_requests == 0 {
            0.0
        } else {
            self.total_accepted as f64 / self.total_requests as f64
        }
    }
    /// Return the top-N most accepted labels.
    pub fn top_accepted(&self, n: usize) -> Vec<(&str, u64)> {
        let mut pairs: Vec<(&str, u64)> = self
            .label_counts
            .iter()
            .map(|(k, v)| (k.as_str(), *v))
            .collect();
        pairs.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
        pairs.truncate(n);
        pairs
    }
    /// Return the total number of distinct accepted labels.
    pub fn distinct_accepted(&self) -> usize {
        self.label_counts.len()
    }
}
/// Fuzzy matcher that scores completion candidates against a query string.
#[derive(Debug, Clone)]
pub struct FuzzyMatcher {
    /// Weights for scoring.
    pub weights: FuzzyWeights,
    /// Whether matching is case-insensitive.
    pub case_insensitive: bool,
}
impl FuzzyMatcher {
    /// Create a new fuzzy matcher with default weights.
    pub fn new() -> Self {
        Self::default()
    }
    /// Create a case-sensitive fuzzy matcher.
    pub fn case_sensitive() -> Self {
        FuzzyMatcher {
            case_insensitive: false,
            ..Self::default()
        }
    }
    /// Test whether `candidate` matches `query` using fuzzy matching.
    pub fn matches(&self, query: &str, candidate: &str) -> FuzzyMatchResult {
        if query.is_empty() {
            return FuzzyMatchResult {
                matched: true,
                score: 0.0,
                match_positions: Vec::new(),
            };
        }
        let q: Vec<char> = if self.case_insensitive {
            query
                .chars()
                .map(|c| c.to_lowercase().next().unwrap_or(c))
                .collect()
        } else {
            query.chars().collect()
        };
        let c: Vec<char> = if self.case_insensitive {
            candidate
                .chars()
                .map(|c| c.to_lowercase().next().unwrap_or(c))
                .collect()
        } else {
            candidate.chars().collect()
        };
        let mut positions = Vec::new();
        let mut ci = 0usize;
        let mut score = 0.0f32;
        let mut last_match: Option<usize> = None;
        for (qi, &qchar) in q.iter().enumerate() {
            let mut found = false;
            while ci < c.len() {
                if c[ci] == qchar {
                    if qi == 0 && ci == 0 {
                        score += self.weights.prefix_bonus;
                    } else if ci > 0 && is_separator(c[ci - 1]) {
                        score += self.weights.word_start_bonus;
                    }
                    if let Some(last) = last_match {
                        if ci == last + 1 {
                            score += self.weights.consecutive_bonus;
                        } else {
                            score += self.weights.gap_penalty * (ci - last - 1) as f32;
                        }
                    }
                    positions.push(ci);
                    last_match = Some(ci);
                    ci += 1;
                    found = true;
                    break;
                }
                ci += 1;
            }
            if !found {
                return FuzzyMatchResult::no_match();
            }
        }
        FuzzyMatchResult {
            matched: true,
            score,
            match_positions: positions,
        }
    }
    /// Score a list of candidates against a query, returning them sorted by score.
    ///
    /// An exact match receives a bonus, and shorter candidates are preferred
    /// when scores are otherwise equal, so that "simp" beats "simp_all" for
    /// the query "simp".
    pub fn score_and_sort<'a>(&self, query: &str, candidates: &[&'a str]) -> Vec<(&'a str, f32)> {
        let mut scored: Vec<(&str, f32)> = candidates
            .iter()
            .filter_map(|c| {
                let result = self.matches(query, c);
                if result.matched {
                    let mut s = result.score;
                    // Exact match bonus
                    if *c == query {
                        s += 10.0;
                    }
                    Some((*c, s))
                } else {
                    None
                }
            })
            .collect();
        scored.sort_by(|a, b| match b.1.partial_cmp(&a.1) {
            Some(std::cmp::Ordering::Equal) | None => a.0.len().cmp(&b.0.len()),
            Some(ord) => ord,
        });
        scored
    }
    /// Apply fuzzy matching to completion items, returning scored items.
    pub fn score_completions<'a>(
        &self,
        query: &str,
        items: &'a [CompletionItem],
    ) -> Vec<(&'a CompletionItem, f32)> {
        let mut scored: Vec<(&CompletionItem, f32)> = items
            .iter()
            .filter_map(|item| {
                let result = self.matches(query, &item.label);
                if result.matched {
                    Some((item, result.score + item.score))
                } else {
                    None
                }
            })
            .collect();
        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        scored
    }
}
/// A library of pre-defined snippet templates.
pub struct SnippetLibrary {
    templates: Vec<SnippetTemplate>,
}
impl SnippetLibrary {
    /// Create the default snippet library.
    pub fn default_library() -> Self {
        let templates = vec![
            SnippetTemplate::new(
                "theorem_by",
                "thm",
                "theorem ${1:name} : ${2:type} := by\n  ${3:sorry}",
                "theorem with by-block",
            ),
            SnippetTemplate::new(
                "def_fn",
                "defn",
                "def ${1:name} (${2:x} : ${3:T}) : ${4:R} :=\n  ${5:body}",
                "function definition",
            ),
            SnippetTemplate::new(
                "struct",
                "struct",
                "structure ${1:Name} where\n  ${2:field} : ${3:Type}",
                "structure definition",
            ),
            SnippetTemplate::new(
                "inductive",
                "ind",
                "inductive ${1:Name} : ${2:Type} where\n  | ${3:ctor} : ${4:Type}",
                "inductive type",
            ),
            SnippetTemplate::new(
                "instance_block",
                "inst",
                "instance : ${1:Class} ${2:type} where\n  ${3:field} := ${4:impl}",
                "instance declaration",
            ),
            SnippetTemplate::new(
                "match_expr",
                "mex",
                "match ${1:expr} with\n| ${2:pat1} -> ${3:body1}\n| ${4:_} -> ${5:body2}",
                "match expression",
            ),
            SnippetTemplate::new(
                "if_then_else",
                "ite",
                "if ${1:cond} then ${2:e1} else ${3:e2}",
                "if-then-else",
            ),
            SnippetTemplate::new(
                "let_in",
                "letin",
                "let ${1:x} := ${2:val}\n${3:body}",
                "let binding",
            ),
        ];
        Self { templates }
    }
    /// Find a template by trigger prefix.
    pub fn matching_trigger(&self, prefix: &str) -> Vec<&SnippetTemplate> {
        self.templates
            .iter()
            .filter(|t| t.trigger.starts_with(prefix))
            .collect()
    }
    /// Return all templates as completion items.
    pub fn to_completion_items(&self) -> Vec<CompletionItem> {
        self.templates
            .iter()
            .map(|t| t.to_completion_item())
            .collect()
    }
    /// Return the number of templates.
    pub fn len(&self) -> usize {
        self.templates.len()
    }
    /// Return true if no templates are registered.
    pub fn is_empty(&self) -> bool {
        self.templates.is_empty()
    }
}
/// Tracks recently accepted completion items to boost their priority.
#[derive(Debug, Default)]
pub struct CompletionSession {
    /// Recently accepted labels (most recent first).
    recent: Vec<String>,
    /// Maximum number of recent items to track.
    max_recent: usize,
}
impl CompletionSession {
    /// Create a new session tracking at most `max_recent` items.
    pub fn new(max_recent: usize) -> Self {
        Self {
            recent: Vec::new(),
            max_recent,
        }
    }
    /// Record an accepted completion.
    pub fn accept(&mut self, label: impl Into<String>) {
        let label = label.into();
        self.recent.retain(|l| l != &label);
        self.recent.insert(0, label);
        if self.recent.len() > self.max_recent {
            self.recent.pop();
        }
    }
    /// Return the recency bonus for the given label (higher = more recent).
    pub fn recency_bonus(&self, label: &str) -> f32 {
        if let Some(pos) = self.recent.iter().position(|l| l == label) {
            1.0 / (1.0 + pos as f32)
        } else {
            0.0
        }
    }
    /// Return the list of recent completions.
    pub fn recent_labels(&self) -> &[String] {
        &self.recent
    }
    /// Return the number of tracked recent items.
    pub fn len(&self) -> usize {
        self.recent.len()
    }
    /// Return true if no items have been recorded.
    pub fn is_empty(&self) -> bool {
        self.recent.is_empty()
    }
}
/// Holds all registered completion items and provides filtered/sorted queries.
pub struct CompletionProvider {
    items: Vec<CompletionItem>,
}
impl CompletionProvider {
    /// Create an empty `CompletionProvider`.
    pub fn new() -> Self {
        CompletionProvider { items: Vec::new() }
    }
    /// Add a single completion item to the database.
    pub fn add_item(&mut self, item: CompletionItem) {
        self.items.push(item);
    }
    /// Return all items whose label starts with the context prefix.
    pub fn complete<'a>(&'a self, ctx: &CompletionContext) -> Vec<&'a CompletionItem> {
        self.items
            .iter()
            .filter(|item| {
                item.matches_prefix(&ctx.prefix)
                    && (!ctx.is_in_tactic_block
                        || matches!(
                            item.kind,
                            CompletionKind::Tactic
                                | CompletionKind::Keyword
                                | CompletionKind::Snippet
                                | CompletionKind::Variable
                                | CompletionKind::Function
                                | CompletionKind::Theorem
                        ))
            })
            .collect()
    }
    /// Return items matching the context, sorted by descending score then label.
    pub fn complete_sorted<'a>(&'a self, ctx: &CompletionContext) -> Vec<&'a CompletionItem> {
        let mut results = self.complete(ctx);
        results.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.sort_key.cmp(&b.sort_key))
        });
        results
    }
    /// Populate tactic completions.
    pub fn register_tactic_completions(&mut self) {
        let tactics: &[(&str, &str)] = &[
            ("intro", "intro <name>"),
            ("intros", "intros <names...>"),
            ("exact", "exact <term>"),
            ("assumption", "assumption"),
            ("apply", "apply <lemma>"),
            ("refl", "refl"),
            ("rw", "rw [<eq>]"),
            ("simp", "simp [<lemmas>]"),
            ("simp_all", "simp_all"),
            ("ring", "ring"),
            ("linarith", "linarith"),
            ("omega", "omega"),
            ("norm_num", "norm_num"),
            ("constructor", "constructor"),
            ("left", "left"),
            ("right", "right"),
            ("cases", "cases <expr>"),
            ("induction", "induction <expr>"),
            ("have", "have <name> : <type>"),
            ("show", "show <type>"),
            ("by_contra", "by_contra <name>"),
            ("push_neg", "push_neg"),
            ("exfalso", "exfalso"),
            ("trivial", "trivial"),
            ("sorry", "sorry"),
            ("split", "split"),
            ("clear", "clear <name>"),
            ("revert", "revert <name>"),
            ("exists", "exists <witness>"),
            ("use", "use <witness>"),
            ("obtain", "obtain <pattern> := <expr>"),
            ("repeat", "repeat <tactic>"),
            ("try", "try <tactic>"),
            ("first", "first | <t1> | <t2>"),
            ("all_goals", "all_goals <tactic>"),
            ("field_simp", "field_simp"),
            ("norm_cast", "norm_cast"),
            ("push_cast", "push_cast"),
            ("exact_mod_cast", "exact_mod_cast <term>"),
            ("conv", "conv => ..."),
            ("rename", "rename <name> => <new_name>"),
            ("contrapose", "contrapose"),
            ("simp_only", "simp only [<lemmas>]"),
        ];
        for (name, detail) in tactics {
            self.add_item(
                CompletionItem::new(name, CompletionKind::Tactic)
                    .with_detail(detail)
                    .with_score(2.0),
            );
        }
    }
    /// Populate keyword completions (def, theorem, axiom, etc.).
    pub fn register_keyword_completions(&mut self) {
        let keywords: &[(&str, &str)] = &[
            ("def", "def <name> : <type> := <body>"),
            ("theorem", "theorem <name> : <type> := <proof>"),
            ("lemma", "lemma <name> : <type> := <proof>"),
            ("axiom", "axiom <name> : <type>"),
            ("structure", "structure <name> where ..."),
            ("inductive", "inductive <name> : <type> where ..."),
            ("class", "class <name> where ..."),
            ("instance", "instance : <class> where ..."),
            ("namespace", "namespace <name>"),
            ("end", "end <name>"),
            ("section", "section <name>"),
            ("variable", "variable (<name> : <type>)"),
            ("open", "open <namespace>"),
            ("import", "import <module>"),
            ("noncomputable", "noncomputable"),
            ("private", "private"),
            ("protected", "protected"),
            ("mutual", "mutual ... end"),
            ("where", "where"),
            ("with", "with"),
            ("do", "do ..."),
            ("if", "if <cond> then <e1> else <e2>"),
            ("match", "match <expr> with | ..."),
            ("let", "let <name> := <expr>"),
            ("fun", "fun <args> -> <body>"),
            ("forall", "forall (<x> : <T>), <body>"),
            ("by", "by\n  <tactics>"),
            ("have", "have <name> : <type> := <proof>"),
            ("show", "show <type>"),
        ];
        for (name, detail) in keywords {
            self.add_item(
                CompletionItem::new(name, CompletionKind::Keyword)
                    .with_detail(detail)
                    .with_score(1.5),
            );
        }
    }
    /// Populate built-in type completions (Nat, Int, Bool, etc.).
    pub fn register_type_completions(&mut self) {
        let types: &[(&str, &str)] = &[
            ("Nat", "Type — natural numbers"),
            ("Int", "Type — integers"),
            ("Bool", "Type — booleans"),
            ("String", "Type — strings"),
            ("Char", "Type — characters"),
            ("Float", "Type — 64-bit floats"),
            ("UInt8", "Type — 8-bit unsigned integer"),
            ("UInt16", "Type — 16-bit unsigned integer"),
            ("UInt32", "Type — 32-bit unsigned integer"),
            ("UInt64", "Type — 64-bit unsigned integer"),
            ("Int8", "Type — 8-bit signed integer"),
            ("Int16", "Type — 16-bit signed integer"),
            ("Int32", "Type — 32-bit signed integer"),
            ("Int64", "Type — 64-bit signed integer"),
            ("List", "Type u — linked lists"),
            ("Option", "Type u — optional values"),
            ("Result", "Type u v — success or error"),
            ("Array", "Type u — dynamic arrays"),
            ("Prop", "Sort 0 — propositions"),
            ("Type", "Sort (u+1) — types"),
            ("Sort", "universe-polymorphic sort"),
            ("True", "Prop — always true"),
            ("False", "Prop — always false"),
            ("And", "Prop → Prop → Prop"),
            ("Or", "Prop → Prop → Prop"),
            ("Not", "Prop → Prop"),
            ("Iff", "Prop → Prop → Prop"),
            ("Eq", "α → α → Prop"),
            ("Ne", "α → α → Prop"),
            ("Exists", "(α → Prop) → Prop"),
            ("Unit", "Type — unit type"),
            ("Empty", "Type — empty type"),
            ("Prod", "Type u → Type v → Type (max u v)"),
            ("Sum", "Type u → Type v → Type (max u v)"),
            ("Fin", "Nat → Type — bounded naturals"),
            ("Subtype", "{ x : α // p x }"),
        ];
        for (name, detail) in types {
            self.add_item(
                CompletionItem::new(name, CompletionKind::Type)
                    .with_detail(detail)
                    .with_score(1.2),
            );
        }
    }
    /// Create a fully populated `CompletionProvider`.
    pub fn default_provider() -> Self {
        let mut p = CompletionProvider::new();
        p.register_tactic_completions();
        p.register_keyword_completions();
        p.register_type_completions();
        p.add_item(SnippetCompletions::theorem_snippet());
        p.add_item(SnippetCompletions::def_snippet());
        p.add_item(SnippetCompletions::match_snippet());
        p.add_item(SnippetCompletions::fun_snippet());
        p
    }
}
/// Metadata annotations attached to completion items.
#[derive(Debug, Clone, Default)]
pub struct CompletionAnnotation {
    /// Whether the item is deprecated.
    pub deprecated: bool,
    /// Whether the item was user-defined (vs built-in).
    pub user_defined: bool,
    /// Optional module path where the item is defined.
    pub module_path: Option<String>,
    /// Optional type signature string.
    pub type_signature: Option<String>,
}
impl CompletionAnnotation {
    /// Create a built-in annotation.
    pub fn builtin() -> Self {
        Self {
            user_defined: false,
            ..Self::default()
        }
    }
    /// Create a user-defined annotation.
    pub fn user() -> Self {
        Self {
            user_defined: true,
            ..Self::default()
        }
    }
    /// Mark as deprecated.
    pub fn deprecated(mut self) -> Self {
        self.deprecated = true;
        self
    }
    /// Set the module path.
    pub fn with_module(mut self, module: impl Into<String>) -> Self {
        self.module_path = Some(module.into());
        self
    }
    /// Set the type signature.
    pub fn with_type(mut self, ty: impl Into<String>) -> Self {
        self.type_signature = Some(ty.into());
        self
    }
}
#[allow(dead_code)]
pub struct SortByScoreStage;
/// A single code-completion candidate.
#[derive(Debug, Clone)]
pub struct CompletionItem {
    pub label: String,
    pub kind: CompletionKind,
    pub detail: Option<String>,
    pub documentation: Option<String>,
    pub insert_text: String,
    pub sort_key: String,
    pub score: f32,
}
impl CompletionItem {
    /// Create a new `CompletionItem` with the given label and kind.
    /// `insert_text` and `sort_key` default to the label; `score` defaults to 1.0.
    pub fn new(label: &str, kind: CompletionKind) -> Self {
        CompletionItem {
            label: label.to_string(),
            kind,
            detail: None,
            documentation: None,
            insert_text: label.to_string(),
            sort_key: label.to_string(),
            score: 1.0,
        }
    }
    /// Set the brief type or signature detail.
    pub fn with_detail(mut self, d: &str) -> Self {
        self.detail = Some(d.to_string());
        self
    }
    /// Override the text that is actually inserted on acceptance.
    pub fn with_insert(mut self, t: &str) -> Self {
        self.insert_text = t.to_string();
        self
    }
    /// Set the relevance score (higher = ranked first).
    pub fn with_score(mut self, s: f32) -> Self {
        self.score = s;
        self
    }
    /// Return true if the item's label starts with `prefix` (case-sensitive).
    pub fn matches_prefix(&self, prefix: &str) -> bool {
        self.label.starts_with(prefix)
    }
}
/// Scoring weights for fuzzy matching.
#[derive(Debug, Clone)]
pub struct FuzzyWeights {
    /// Score bonus for matching the start of the label.
    pub prefix_bonus: f32,
    /// Score bonus for matching after a separator (_, ., ::).
    pub word_start_bonus: f32,
    /// Score bonus for consecutive matching characters.
    pub consecutive_bonus: f32,
    /// Score penalty for skipped characters.
    pub gap_penalty: f32,
}
/// What triggered a completion request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompletionTrigger {
    /// Invoked explicitly by the user.
    Invoked,
    /// Triggered by typing a specific character.
    TriggerCharacter(char),
    /// Re-triggered to update an existing completion list.
    TriggerForIncompleteCompletions,
}
impl CompletionTrigger {
    /// Return the trigger character, if any.
    pub fn trigger_char(&self) -> Option<char> {
        match self {
            CompletionTrigger::TriggerCharacter(c) => Some(*c),
            _ => None,
        }
    }
    /// Return true if this was an explicit invocation.
    pub fn is_invoked(&self) -> bool {
        matches!(self, CompletionTrigger::Invoked)
    }
}
/// Entry in the completion cache.
#[derive(Debug, Clone)]
pub struct CompletionCacheEntry {
    /// The prefix used as cache key.
    pub prefix: String,
    /// The cached completion items (as labels for lightweight storage).
    pub item_labels: Vec<String>,
    /// Whether the cache entry is still valid.
    pub valid: bool,
}
impl CompletionCacheEntry {
    /// Create a new cache entry.
    pub fn new(prefix: impl Into<String>, item_labels: Vec<String>) -> Self {
        Self {
            prefix: prefix.into(),
            item_labels,
            valid: true,
        }
    }
    /// Invalidate this entry.
    pub fn invalidate(&mut self) {
        self.valid = false;
    }
}
#[allow(dead_code)]
pub struct CompletionPipeline {
    stages: Vec<Box<dyn CompletionStage>>,
}
#[allow(dead_code)]
impl CompletionPipeline {
    pub fn new() -> Self {
        CompletionPipeline { stages: Vec::new() }
    }
    pub fn add_stage<S: CompletionStage + 'static>(mut self, stage: S) -> Self {
        self.stages.push(Box::new(stage));
        self
    }
    pub fn run(
        &self,
        ctx: &CompletionContext,
        initial: Vec<CompletionItem>,
    ) -> Vec<CompletionItem> {
        self.stages
            .iter()
            .fold(initial, |items, stage| stage.process(ctx, items))
    }
    pub fn stage_names(&self) -> Vec<&'static str> {
        self.stages.iter().map(|s| s.name()).collect()
    }
}
#[allow(dead_code)]
pub struct DeduplicateStage;
/// A detailed score breakdown for a completion item.
#[derive(Debug, Clone, Default)]
pub struct CompletionScore {
    /// Score from fuzzy match.
    pub fuzzy_score: f32,
    /// Score from item priority/kind.
    pub kind_score: f32,
    /// Recency bonus (e.g., recently used items score higher).
    pub recency_bonus: f32,
    /// User-assigned boost.
    pub user_boost: f32,
}
impl CompletionScore {
    /// Compute the total score.
    pub fn total(&self) -> f32 {
        self.fuzzy_score + self.kind_score + self.recency_bonus + self.user_boost
    }
}
/// A filter that requires the item's score to be at least a minimum.
pub struct MinScoreFilter {
    pub min_score: f32,
}
impl MinScoreFilter {
    /// Create a filter with the given minimum score.
    pub fn new(min_score: f32) -> Self {
        Self { min_score }
    }
}
/// A simple LRU-like completion cache.
#[derive(Default)]
pub struct CompletionCache {
    entries: Vec<CompletionCacheEntry>,
    max_entries: usize,
}
impl CompletionCache {
    /// Create a new cache with the given maximum number of entries.
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: Vec::new(),
            max_entries,
        }
    }
    /// Look up a cache entry by prefix.
    pub fn get(&self, prefix: &str) -> Option<&CompletionCacheEntry> {
        self.entries.iter().find(|e| e.prefix == prefix && e.valid)
    }
    /// Insert or update a cache entry.
    pub fn put(&mut self, prefix: impl Into<String>, item_labels: Vec<String>) {
        let prefix = prefix.into();
        if let Some(existing) = self.entries.iter_mut().find(|e| e.prefix == prefix) {
            existing.item_labels = item_labels;
            existing.valid = true;
        } else {
            if self.entries.len() >= self.max_entries && self.max_entries > 0 {
                self.entries.remove(0);
            }
            self.entries
                .push(CompletionCacheEntry::new(prefix, item_labels));
        }
    }
    /// Invalidate all entries whose prefix starts with `invalidation_prefix`.
    pub fn invalidate_prefix(&mut self, invalidation_prefix: &str) {
        for entry in &mut self.entries {
            if entry.prefix.starts_with(invalidation_prefix) {
                entry.invalidate();
            }
        }
    }
    /// Clear all entries.
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    /// Return the number of entries (including invalid).
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Return the number of valid entries.
    pub fn valid_len(&self) -> usize {
        self.entries.iter().filter(|e| e.valid).count()
    }
    /// Return true if the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
/// A ranker that combines multiple signals into a final ordering.
#[derive(Default)]
pub struct CompletionRanker {
    fuzzy_matcher: FuzzyMatcher,
    session: CompletionSession,
}
impl CompletionRanker {
    /// Create a new ranker.
    pub fn new() -> Self {
        Self {
            fuzzy_matcher: FuzzyMatcher::new(),
            session: CompletionSession::new(20),
        }
    }
    /// Record that an item was accepted.
    pub fn record_acceptance(&mut self, label: impl Into<String>) {
        self.session.accept(label);
    }
    /// Rank items against the given query, returning them in descending score order.
    pub fn rank<'a>(
        &self,
        query: &str,
        items: &'a [CompletionItem],
    ) -> Vec<(&'a CompletionItem, f32)> {
        let mut scored: Vec<(&CompletionItem, f32)> = items
            .iter()
            .filter_map(|item| {
                let fuzzy_result = self.fuzzy_matcher.matches(query, &item.label);
                if !fuzzy_result.matched {
                    return None;
                }
                let recency = self.session.recency_bonus(&item.label);
                let total = fuzzy_result.score + item.score + recency;
                Some((item, total))
            })
            .collect();
        scored.sort_by(|a, b| {
            b.1.partial_cmp(&a.1)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.0.sort_key.cmp(&b.0.sort_key))
        });
        scored
    }
}
/// An annotated completion item that carries extended metadata.
#[derive(Debug, Clone)]
pub struct AnnotatedCompletionItem {
    /// The underlying completion item.
    pub item: CompletionItem,
    /// Extended annotation.
    pub annotation: CompletionAnnotation,
}
impl AnnotatedCompletionItem {
    /// Create a new annotated item.
    pub fn new(item: CompletionItem, annotation: CompletionAnnotation) -> Self {
        Self { item, annotation }
    }
    /// Return true if the item is deprecated.
    pub fn is_deprecated(&self) -> bool {
        self.annotation.deprecated
    }
    /// Return the label of the item.
    pub fn label(&self) -> &str {
        &self.item.label
    }
    /// Return the type signature, if set.
    pub fn type_signature(&self) -> Option<&str> {
        self.annotation.type_signature.as_deref()
    }
}
/// A snippet template with named placeholders.
#[derive(Debug, Clone)]
pub struct SnippetTemplate {
    /// Template ID.
    pub id: String,
    /// The template text with `${n:hint}` style placeholders.
    pub template: String,
    /// Human-readable description.
    pub description: String,
    /// Trigger prefix.
    pub trigger: String,
}
impl SnippetTemplate {
    /// Create a new snippet template.
    pub fn new(
        id: impl Into<String>,
        trigger: impl Into<String>,
        template: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            trigger: trigger.into(),
            template: template.into(),
            description: description.into(),
        }
    }
    /// Convert this template to a `CompletionItem`.
    pub fn to_completion_item(&self) -> CompletionItem {
        CompletionItem {
            label: self.trigger.clone(),
            kind: CompletionKind::Snippet,
            detail: Some(self.description.clone()),
            documentation: None,
            insert_text: self.template.clone(),
            sort_key: self.trigger.clone(),
            score: 2.0,
        }
    }
    /// Return the number of placeholders in the template.
    pub fn placeholder_count(&self) -> usize {
        self.template.matches("${").count()
    }
}
#[allow(dead_code)]
pub struct TruncateStage {
    pub(super) limit: usize,
}
#[allow(dead_code)]
impl TruncateStage {
    pub fn new(limit: usize) -> Self {
        TruncateStage { limit }
    }
}
/// Factory for common snippet completion items.
pub struct SnippetCompletions;
impl SnippetCompletions {
    /// Snippet for a theorem skeleton.
    pub fn theorem_snippet() -> CompletionItem {
        CompletionItem::new("theorem!", CompletionKind::Snippet)
            .with_detail("theorem skeleton")
            .with_insert("theorem ${1:name} : ${2:type} := by\n  ${3:sorry}")
            .with_score(3.0)
    }
    /// Snippet for a definition skeleton.
    pub fn def_snippet() -> CompletionItem {
        CompletionItem::new("def!", CompletionKind::Snippet)
            .with_detail("def skeleton")
            .with_insert("def ${1:name} : ${2:type} :=\n  ${3:body}")
            .with_score(3.0)
    }
    /// Snippet for a match expression skeleton.
    pub fn match_snippet() -> CompletionItem {
        CompletionItem::new("match!", CompletionKind::Snippet)
            .with_detail("match expression skeleton")
            .with_insert("match ${1:expr} with\n| ${2:pattern} -> ${3:body}")
            .with_score(2.5)
    }
    /// Snippet for a fun expression skeleton.
    pub fn fun_snippet() -> CompletionItem {
        CompletionItem::new("fun!", CompletionKind::Snippet)
            .with_detail("fun expression skeleton")
            .with_insert("fun ${1:x} -> ${2:body}")
            .with_score(2.5)
    }
}
/// Extended documentation for a completion item, shown in IDE hover.
#[derive(Debug, Clone, Default)]
pub struct CompletionDocumentation {
    /// One-line summary.
    pub summary: String,
    /// Longer description (may include markdown).
    pub description: Option<String>,
    /// An example usage snippet.
    pub example: Option<String>,
    /// Links to external references.
    pub see_also: Vec<String>,
}
impl CompletionDocumentation {
    /// Create documentation with just a summary.
    pub fn summary(s: impl Into<String>) -> Self {
        Self {
            summary: s.into(),
            ..Self::default()
        }
    }
    /// Set the description.
    pub fn with_description(mut self, d: impl Into<String>) -> Self {
        self.description = Some(d.into());
        self
    }
    /// Set an example.
    pub fn with_example(mut self, e: impl Into<String>) -> Self {
        self.example = Some(e.into());
        self
    }
    /// Add a see-also link.
    pub fn add_see_also(&mut self, link: impl Into<String>) {
        self.see_also.push(link.into());
    }
    /// Render to markdown.
    pub fn to_markdown(&self) -> String {
        let mut out = format!("**{}**\n", self.summary);
        if let Some(desc) = &self.description {
            out.push_str(&format!("\n{}\n", desc));
        }
        if let Some(example) = &self.example {
            out.push_str(&format!("\n```\n{}\n```\n", example));
        }
        if !self.see_also.is_empty() {
            out.push_str("\nSee also:\n");
            for link in &self.see_also {
                out.push_str(&format!("- {}\n", link));
            }
        }
        out
    }
}
/// A chain of filters applied in conjunction (all must accept).
#[derive(Default)]
pub struct FilterChain {
    filters: Vec<Box<dyn CompletionFilter>>,
}
impl FilterChain {
    /// Create an empty filter chain.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a filter to the chain.
    pub fn add<F: CompletionFilter + 'static>(&mut self, filter: F) {
        self.filters.push(Box::new(filter));
    }
    /// Return true if all filters accept the item.
    pub fn accepts(&self, item: &CompletionItem, ctx: &CompletionContext) -> bool {
        self.filters.iter().all(|f| f.accepts(item, ctx))
    }
    /// Return filter names.
    pub fn filter_names(&self) -> Vec<&str> {
        self.filters.iter().map(|f| f.filter_name()).collect()
    }
    /// Apply this chain to a slice of items, returning matching ones.
    pub fn apply<'a>(
        &self,
        items: &'a [CompletionItem],
        ctx: &CompletionContext,
    ) -> Vec<&'a CompletionItem> {
        items
            .iter()
            .filter(|item| self.accepts(item, ctx))
            .collect()
    }
}
/// Result of a fuzzy match attempt.
#[derive(Debug, Clone)]
pub struct FuzzyMatchResult {
    /// Whether the query matches the candidate.
    pub matched: bool,
    /// Computed relevance score (higher = better match).
    pub score: f32,
    /// Positions in the candidate string where query chars were matched.
    pub match_positions: Vec<usize>,
}
impl FuzzyMatchResult {
    /// Create a non-matching result.
    pub fn no_match() -> Self {
        FuzzyMatchResult {
            matched: false,
            score: 0.0,
            match_positions: Vec::new(),
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct CompletionHistogram {
    counts: std::collections::HashMap<String, u64>,
}
#[allow(dead_code)]
impl CompletionHistogram {
    pub fn new() -> Self {
        CompletionHistogram {
            counts: std::collections::HashMap::new(),
        }
    }
    pub fn record(&mut self, kind: &CompletionKind) {
        let key = format!("{:?}", kind);
        *self.counts.entry(key).or_insert(0) += 1;
    }
    pub fn record_all(&mut self, items: &[CompletionItem]) {
        for item in items {
            self.record(&item.kind);
        }
    }
    pub fn count(&self, kind: &CompletionKind) -> u64 {
        let key = format!("{:?}", kind);
        self.counts.get(&key).copied().unwrap_or(0)
    }
    pub fn top_kinds(&self, n: usize) -> Vec<(String, u64)> {
        let mut pairs: Vec<(String, u64)> =
            self.counts.iter().map(|(k, v)| (k.clone(), *v)).collect();
        pairs.sort_by(|a, b| b.1.cmp(&a.1));
        pairs.truncate(n);
        pairs
    }
    pub fn total(&self) -> u64 {
        self.counts.values().sum()
    }
}
/// The kind of completion item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompletionKind {
    Function,
    Theorem,
    Type,
    Tactic,
    Variable,
    Constructor,
    Field,
    Keyword,
    Snippet,
}
impl CompletionKind {
    #[allow(dead_code)]
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            CompletionKind::Function => "function",
            CompletionKind::Theorem => "theorem",
            CompletionKind::Type => "type",
            CompletionKind::Tactic => "tactic",
            CompletionKind::Variable => "variable",
            CompletionKind::Constructor => "constructor",
            CompletionKind::Field => "field",
            CompletionKind::Keyword => "keyword",
            CompletionKind::Snippet => "snippet",
        }
    }
}
/// A richer completion context including trigger information.
#[derive(Debug, Clone)]
pub struct EnrichedCompletionContext {
    /// Base context.
    pub base: CompletionContext,
    /// What triggered this completion.
    pub trigger: CompletionTrigger,
    /// Surrounding text (for context-sensitive completions).
    pub surrounding: Option<String>,
}
impl EnrichedCompletionContext {
    /// Create an enriched context from a base context.
    pub fn from_base(base: CompletionContext) -> Self {
        Self {
            base,
            trigger: CompletionTrigger::Invoked,
            surrounding: None,
        }
    }
    /// Set the trigger.
    pub fn with_trigger(mut self, trigger: CompletionTrigger) -> Self {
        self.trigger = trigger;
        self
    }
    /// Set surrounding text.
    pub fn with_surrounding(mut self, text: impl Into<String>) -> Self {
        self.surrounding = Some(text.into());
        self
    }
}
/// Context describing where a completion was triggered.
#[derive(Debug, Clone)]
pub struct CompletionContext {
    pub prefix: String,
    pub line: u32,
    pub col: u32,
    pub is_in_tactic_block: bool,
    pub is_in_type_position: bool,
}
impl CompletionContext {
    /// Create a new `CompletionContext`.
    pub fn new(prefix: &str, line: u32, col: u32) -> Self {
        CompletionContext {
            prefix: prefix.to_string(),
            line,
            col,
            is_in_tactic_block: false,
            is_in_type_position: false,
        }
    }
}
/// A filter that requires the item's kind to be in a set of allowed kinds.
pub struct KindFilter {
    pub allowed: Vec<CompletionKind>,
}
impl KindFilter {
    /// Create a filter that allows the given kinds.
    pub fn new(allowed: Vec<CompletionKind>) -> Self {
        Self { allowed }
    }
    /// Create a filter that allows only tactics.
    pub fn tactics_only() -> Self {
        Self::new(vec![CompletionKind::Tactic])
    }
    /// Create a filter that allows only types.
    pub fn types_only() -> Self {
        Self::new(vec![CompletionKind::Type])
    }
}
/// A middleware that limits the number of returned items.
pub struct LimitMiddleware {
    pub(super) max_items: usize,
}
impl LimitMiddleware {
    /// Create a new limit middleware.
    pub fn new(max_items: usize) -> Self {
        Self { max_items }
    }
}
/// A filter that requires the item's label to contain a substring.
pub struct LabelContainsFilter {
    pub substring: String,
}
impl LabelContainsFilter {
    /// Create a filter requiring the label to contain `s`.
    pub fn new(s: impl Into<String>) -> Self {
        Self {
            substring: s.into(),
        }
    }
}
