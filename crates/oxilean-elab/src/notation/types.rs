//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{Expr, Level, Name};

use std::collections::{HashMap, HashSet};

/// A part of a notation pattern.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NotationPart {
    /// Fixed literal text, e.g. `+` or `if`.
    Literal(String),
    /// An argument placeholder with a name and precedence.
    Placeholder(String, u32),
    /// An optional part.
    Optional(Box<NotationPart>),
}
#[allow(dead_code)]
#[derive(Default)]
pub struct NotationCompletionHelper;
#[allow(dead_code)]
impl NotationCompletionHelper {
    pub fn new() -> Self {
        NotationCompletionHelper
    }
    pub fn completions_for_prefix<'a>(
        &self,
        prefix: &str,
        registry: &'a NotationRegistry,
    ) -> Vec<&'a Notation> {
        registry
            .all_notations()
            .iter()
            .filter(|n| n.pattern.starts_with(prefix))
            .collect()
    }
    pub fn suggest_similar(
        &self,
        input: &str,
        registry: &NotationRegistry,
        max: usize,
    ) -> Vec<String> {
        let mut found: Vec<(usize, String)> = registry
            .all_notations()
            .iter()
            .filter_map(|n| {
                let common = n
                    .pattern
                    .chars()
                    .zip(input.chars())
                    .take_while(|(a, b)| a == b)
                    .count();
                if common > 0 {
                    Some((common, n.pattern.clone()))
                } else {
                    None
                }
            })
            .collect();
        found.sort_by_key(|b| std::cmp::Reverse(b.0));
        found.truncate(max);
        found.into_iter().map(|(_, s)| s).collect()
    }
}
/// Notation registry for tracking defined notations.
pub struct NotationRegistry {
    /// All registered notations, keyed by name.
    notations: Vec<Notation>,
    /// Quick lookup for prefix notations by symbol.
    prefix_map: HashMap<String, usize>,
    /// Quick lookup for infix notations by symbol.
    infix_map: HashMap<String, usize>,
    /// Quick lookup for postfix notations by symbol.
    postfix_map: HashMap<String, usize>,
    /// Active scopes.
    active_scopes: Vec<Name>,
}
impl NotationRegistry {
    /// Create a new empty notation registry.
    pub fn new() -> Self {
        Self {
            notations: Vec::new(),
            prefix_map: HashMap::new(),
            infix_map: HashMap::new(),
            postfix_map: HashMap::new(),
            active_scopes: Vec::new(),
        }
    }
    /// Register a notation.
    pub fn register(&mut self, notation: Notation) {
        let idx = self.notations.len();
        match &notation.kind {
            NotationKind::Prefix { .. } => {
                self.prefix_map.insert(notation.pattern.clone(), idx);
            }
            NotationKind::Infixl { .. } | NotationKind::Infixr { .. } => {
                self.infix_map.insert(notation.pattern.clone(), idx);
            }
            NotationKind::Postfix { .. } => {
                self.postfix_map.insert(notation.pattern.clone(), idx);
            }
            NotationKind::Notation | NotationKind::Macro => {}
        }
        self.notations.push(notation);
    }
    /// Register a prefix notation.
    #[allow(dead_code)]
    pub fn register_prefix(&mut self, name: Name, symbol: &str, fun_name: Name, precedence: u32) {
        let notation = Notation {
            name,
            kind: NotationKind::Prefix { precedence },
            pattern: symbol.to_string(),
            parts: vec![
                NotationPart::Literal(symbol.to_string()),
                NotationPart::Placeholder("a".to_string(), precedence),
            ],
            expansion: NotationExpansion::Simple(Expr::Const(fun_name, vec![])),
            priority: precedence,
            scope: None,
            is_builtin: false,
        };
        self.register(notation);
    }
    /// Register an infix notation.
    #[allow(dead_code)]
    pub fn register_infix(
        &mut self,
        name: Name,
        symbol: &str,
        fun_name: Name,
        precedence: u32,
        left_assoc: bool,
    ) {
        let kind = if left_assoc {
            NotationKind::Infixl { precedence }
        } else {
            NotationKind::Infixr { precedence }
        };
        let notation = Notation {
            name,
            kind,
            pattern: symbol.to_string(),
            parts: vec![
                NotationPart::Placeholder("a".to_string(), precedence),
                NotationPart::Literal(symbol.to_string()),
                NotationPart::Placeholder("b".to_string(), precedence),
            ],
            expansion: NotationExpansion::Simple(Expr::Const(fun_name, vec![])),
            priority: precedence,
            scope: None,
            is_builtin: false,
        };
        self.register(notation);
    }
    /// Register a postfix notation.
    #[allow(dead_code)]
    pub fn register_postfix(&mut self, name: Name, symbol: &str, fun_name: Name, precedence: u32) {
        let notation = Notation {
            name,
            kind: NotationKind::Postfix { precedence },
            pattern: symbol.to_string(),
            parts: vec![
                NotationPart::Placeholder("a".to_string(), precedence),
                NotationPart::Literal(symbol.to_string()),
            ],
            expansion: NotationExpansion::Simple(Expr::Const(fun_name, vec![])),
            priority: precedence,
            scope: None,
            is_builtin: false,
        };
        self.register(notation);
    }
    /// Register a general notation from structured parts and expansion.
    #[allow(dead_code)]
    pub fn register_notation_parts(
        &mut self,
        name: Name,
        parts: Vec<NotationPart>,
        expansion: NotationExpansion,
        scope: Option<Name>,
    ) {
        let notation = Notation {
            name,
            kind: NotationKind::Notation,
            pattern: parts_to_pattern(&parts),
            parts,
            expansion,
            priority: 0,
            scope,
            is_builtin: false,
        };
        self.register(notation);
    }
    /// Unregister a notation by name.
    #[allow(dead_code)]
    pub fn unregister(&mut self, name: &Name) {
        if let Some(pos) = self.notations.iter().position(|n| &n.name == name) {
            let notation = &self.notations[pos];
            match &notation.kind {
                NotationKind::Prefix { .. } => {
                    self.prefix_map.remove(&notation.pattern);
                }
                NotationKind::Infixl { .. } | NotationKind::Infixr { .. } => {
                    self.infix_map.remove(&notation.pattern);
                }
                NotationKind::Postfix { .. } => {
                    self.postfix_map.remove(&notation.pattern);
                }
                NotationKind::Notation | NotationKind::Macro => {}
            }
            self.notations.remove(pos);
            self.rebuild_indices();
        }
    }
    /// Rebuild the quick-lookup indices after a removal.
    fn rebuild_indices(&mut self) {
        self.prefix_map.clear();
        self.infix_map.clear();
        self.postfix_map.clear();
        for (idx, notation) in self.notations.iter().enumerate() {
            match &notation.kind {
                NotationKind::Prefix { .. } => {
                    self.prefix_map.insert(notation.pattern.clone(), idx);
                }
                NotationKind::Infixl { .. } | NotationKind::Infixr { .. } => {
                    self.infix_map.insert(notation.pattern.clone(), idx);
                }
                NotationKind::Postfix { .. } => {
                    self.postfix_map.insert(notation.pattern.clone(), idx);
                }
                NotationKind::Notation | NotationKind::Macro => {}
            }
        }
    }
    /// Find a notation by pattern string match.
    #[allow(dead_code)]
    pub fn find_notation(&self, input: &str) -> Option<&Notation> {
        if let Some(&idx) = self.prefix_map.get(input) {
            let n = &self.notations[idx];
            if self.is_active(n) {
                return Some(n);
            }
        }
        if let Some(&idx) = self.infix_map.get(input) {
            let n = &self.notations[idx];
            if self.is_active(n) {
                return Some(n);
            }
        }
        if let Some(&idx) = self.postfix_map.get(input) {
            let n = &self.notations[idx];
            if self.is_active(n) {
                return Some(n);
            }
        }
        self.notations
            .iter()
            .rev()
            .find(|notation| self.is_active(notation) && input.contains(&notation.pattern))
    }
    /// Look up a prefix notation by symbol.
    #[allow(dead_code)]
    pub fn lookup_prefix(&self, symbol: &str) -> Option<&Notation> {
        self.prefix_map.get(symbol).and_then(|&idx| {
            let n = &self.notations[idx];
            if self.is_active(n) {
                Some(n)
            } else {
                None
            }
        })
    }
    /// Look up an infix notation by symbol.
    #[allow(dead_code)]
    pub fn lookup_infix(&self, symbol: &str) -> Option<&Notation> {
        self.infix_map.get(symbol).and_then(|&idx| {
            let n = &self.notations[idx];
            if self.is_active(n) {
                Some(n)
            } else {
                None
            }
        })
    }
    /// Look up a postfix notation by symbol.
    #[allow(dead_code)]
    pub fn lookup_postfix(&self, symbol: &str) -> Option<&Notation> {
        self.postfix_map.get(symbol).and_then(|&idx| {
            let n = &self.notations[idx];
            if self.is_active(n) {
                Some(n)
            } else {
                None
            }
        })
    }
    /// Expand a notation applied to arguments.
    #[allow(dead_code)]
    pub fn expand_notation(&self, notation: &Notation, args: &[Expr]) -> Result<Expr, String> {
        expand_notation_impl(notation, args)
    }
    /// Expand a notation by pattern string (legacy simple API).
    pub fn expand(&self, input: &str) -> Option<String> {
        for notation in &self.notations {
            if self.is_active(notation) && input.contains(&notation.pattern) {
                if let NotationExpansion::Simple(Expr::Const(name, _)) = &notation.expansion {
                    return Some(name.to_string());
                }
                return None;
            }
        }
        None
    }
    /// Get all registered notations.
    pub fn all_notations(&self) -> &[Notation] {
        &self.notations
    }
    /// Register all built-in notations (arithmetic, logic, etc.).
    #[allow(dead_code)]
    pub fn register_builtins(&mut self) {
        self.register_builtin_arithmetic();
        self.register_builtin_logic();
        self.register_builtin_comparison();
        self.register_builtin_misc();
    }
    /// Register built-in arithmetic notations.
    fn register_builtin_arithmetic(&mut self) {
        let ops = [
            ("+", "HAdd.hAdd", 65u32),
            ("-", "HSub.hSub", 65),
            ("*", "HMul.hMul", 70),
            ("/", "HDiv.hDiv", 70),
        ];
        for (sym, fun, prec) in ops {
            let mut notation = Notation {
                name: Name::str(sym),
                kind: NotationKind::Infixl { precedence: prec },
                pattern: sym.to_string(),
                parts: vec![
                    NotationPart::Placeholder("a".to_string(), prec),
                    NotationPart::Literal(sym.to_string()),
                    NotationPart::Placeholder("b".to_string(), prec),
                ],
                expansion: NotationExpansion::Simple(Expr::Const(Name::str(fun), vec![])),
                priority: prec,
                scope: None,
                is_builtin: true,
            };
            notation.is_builtin = true;
            self.register(notation);
        }
    }
    /// Register built-in logic notations.
    fn register_builtin_logic(&mut self) {
        self.register_builtin_infix("&&", "And", 35, false);
        self.register_builtin_infix("||", "Or", 30, false);
        let not_notation = Notation {
            name: Name::str("!"),
            kind: NotationKind::Prefix { precedence: 100 },
            pattern: "!".to_string(),
            parts: vec![
                NotationPart::Literal("!".to_string()),
                NotationPart::Placeholder("a".to_string(), 100),
            ],
            expansion: NotationExpansion::Simple(Expr::Const(Name::str("Not"), vec![])),
            priority: 100,
            scope: None,
            is_builtin: true,
        };
        self.register(not_notation);
    }
    /// Register built-in comparison notations.
    fn register_builtin_comparison(&mut self) {
        let cmp_ops: &[(&str, &str, u32)] = &[
            ("=", "Eq", 50),
            ("\u{2260}", "Ne", 50),
            ("<", "LT.lt", 50),
            ("\u{2264}", "LE.le", 50),
        ];
        for &(sym, fun, prec) in cmp_ops {
            self.register_builtin_infix(sym, fun, prec, true);
        }
    }
    /// Register miscellaneous built-in notations.
    fn register_builtin_misc(&mut self) {
        self.register_builtin_infix("++", "HAppend.hAppend", 65, false);
        self.register_builtin_infix(">>", "Bind.bind", 55, true);
        self.register_builtin_infix("::", "List.cons", 67, false);
    }
    /// Helper: register a built-in infix notation.
    fn register_builtin_infix(
        &mut self,
        symbol: &str,
        fun_name: &str,
        precedence: u32,
        left_assoc: bool,
    ) {
        let kind = if left_assoc {
            NotationKind::Infixl { precedence }
        } else {
            NotationKind::Infixr { precedence }
        };
        let notation = Notation {
            name: Name::str(symbol),
            kind,
            pattern: symbol.to_string(),
            parts: vec![
                NotationPart::Placeholder("a".to_string(), precedence),
                NotationPart::Literal(symbol.to_string()),
                NotationPart::Placeholder("b".to_string(), precedence),
            ],
            expansion: NotationExpansion::Simple(Expr::Const(Name::str(fun_name), vec![])),
            priority: precedence,
            scope: None,
            is_builtin: true,
        };
        self.register(notation);
    }
    /// Open a notation scope. Notations in this scope become active.
    #[allow(dead_code)]
    pub fn open_scope(&mut self, name: Name) {
        if !self.active_scopes.contains(&name) {
            self.active_scopes.push(name);
        }
    }
    /// Close a notation scope. Notations in this scope become inactive.
    #[allow(dead_code)]
    pub fn close_scope(&mut self, name: &Name) {
        self.active_scopes.retain(|s| s != name);
    }
    /// Check if a scope is active.
    #[allow(dead_code)]
    pub fn is_scope_active(&self, name: &Name) -> bool {
        self.active_scopes.contains(name)
    }
    /// Get all active scopes.
    #[allow(dead_code)]
    pub fn active_scopes(&self) -> &[Name] {
        &self.active_scopes
    }
    /// Check whether a notation is active given current scopes.
    fn is_active(&self, notation: &Notation) -> bool {
        match &notation.scope {
            None => true,
            Some(scope) => self.active_scopes.contains(scope),
        }
    }
}
/// A statement in a do block (for desugaring).
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum DoStatement {
    /// `x ← e`: bind `e` and name the result `x`.
    Bind(Name, Expr),
    /// `let x := e`: let binding.
    Let(Name, Expr),
    /// `e`: a pure expression (last statement or discarded result).
    Expr(Expr),
    /// `return e`: pure/return.
    Return(Expr),
    /// `for x in xs do body`: for-in loop.
    ForIn(Name, Expr, Box<DoStatement>),
}
#[allow(dead_code)]
pub struct NotationTokenizer<'a> {
    input: &'a str,
    pos: usize,
}
#[allow(dead_code)]
impl<'a> NotationTokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        NotationTokenizer { input, pos: 0 }
    }
    pub fn remaining(&self) -> &str {
        &self.input[self.pos..]
    }
    pub fn is_done(&self) -> bool {
        self.pos >= self.input.len()
    }
    pub fn peek_char(&self) -> Option<char> {
        self.remaining().chars().next()
    }
    pub fn next_token(&mut self) -> NotationToken {
        if self.is_done() {
            return NotationToken::EndOfInput;
        }
        let ch = self
            .peek_char()
            .expect("not done: peek_char returns Some when !is_done()");
        if ch.is_whitespace() {
            while !self.is_done() && self.peek_char().is_some_and(|c| c.is_whitespace()) {
                self.advance();
            }
            return NotationToken::Whitespace;
        }
        if ch.is_ascii_digit() {
            let start = self.pos;
            while !self.is_done() && self.peek_char().is_some_and(|c| c.is_ascii_digit()) {
                self.advance();
            }
            let s: &str = &self.input[start..self.pos];
            return NotationToken::Number(s.parse().unwrap_or(0));
        }
        if ch.is_alphabetic() || ch == '_' {
            let start = self.pos;
            while !self.is_done()
                && self
                    .peek_char()
                    .is_some_and(|c| c.is_alphanumeric() || c == '_')
            {
                self.advance();
            }
            let s = self.input[start..self.pos].to_string();
            return NotationToken::Identifier(s);
        }
        self.advance();
        NotationToken::Symbol(ch.to_string())
    }
    fn advance(&mut self) {
        if let Some(c) = self.peek_char() {
            self.pos += c.len_utf8();
        }
    }
    pub fn tokenize_all(&mut self) -> Vec<NotationToken> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.next_token();
            let done = tok == NotationToken::EndOfInput;
            tokens.push(tok);
            if done {
                break;
            }
        }
        tokens
    }
}
#[allow(dead_code)]
pub struct NotationMigrationHelper {
    rewrites: Vec<(String, String)>,
}
#[allow(dead_code)]
impl NotationMigrationHelper {
    pub fn new() -> Self {
        let rewrites = vec![
            ("infixl".to_string(), "infix (left-assoc)".to_string()),
            ("infixr".to_string(), "infix (right-assoc)".to_string()),
            ("notation3".to_string(), "notation".to_string()),
        ];
        NotationMigrationHelper { rewrites }
    }
    pub fn migrate(&self, source: &str) -> String {
        let mut result = source.to_string();
        for (old, new) in &self.rewrites {
            result = result.replace(old.as_str(), new.as_str());
        }
        result
    }
    pub fn add_rewrite(&mut self, from: impl Into<String>, to: impl Into<String>) {
        self.rewrites.push((from.into(), to.into()));
    }
    pub fn rewrite_count(&self) -> usize {
        self.rewrites.len()
    }
}
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct NotationStats {
    pub total_expansions: u64,
    pub cache_hits: u64,
    pub unique_symbols_used: std::collections::HashSet<String>,
    pub expansion_depth_max: usize,
}
#[allow(dead_code)]
impl NotationStats {
    pub fn new() -> Self {
        NotationStats::default()
    }
    pub fn record_expansion(&mut self, symbol: &str, depth: usize) {
        self.total_expansions += 1;
        self.unique_symbols_used.insert(symbol.to_string());
        if depth > self.expansion_depth_max {
            self.expansion_depth_max = depth;
        }
    }
    pub fn record_cache_hit(&mut self) {
        self.cache_hits += 1;
    }
    pub fn unique_symbol_count(&self) -> usize {
        self.unique_symbols_used.len()
    }
    pub fn cache_hit_rate(&self) -> f64 {
        if self.total_expansions == 0 {
            0.0
        } else {
            self.cache_hits as f64 / self.total_expansions as f64
        }
    }
}
#[allow(dead_code)]
pub struct NotationElabContext {
    pub registry: NotationRegistry,
    pub scope_stack: NotationScopeStack,
    pub stats: NotationStats,
    pub cache: NotationExpansionCache,
}
#[allow(dead_code)]
impl NotationElabContext {
    pub fn new() -> Self {
        let mut reg = NotationRegistry::new();
        reg.register_builtins();
        NotationElabContext {
            registry: reg,
            scope_stack: NotationScopeStack::new(),
            stats: NotationStats::new(),
            cache: NotationExpansionCache::new(512),
        }
    }
    pub fn open_scope(&mut self, name: Name) {
        self.scope_stack.push();
        self.registry.open_scope(name);
    }
    pub fn close_scope(&mut self, name: &Name) {
        self.scope_stack.pop();
        self.registry.close_scope(name);
    }
    pub fn register_notation(&mut self, n: Notation) {
        self.scope_stack.add_to_current(n.clone());
        self.registry.register(n);
    }
    pub fn find(&self, pattern: &str) -> Option<&Notation> {
        self.registry.find_notation(pattern)
    }
    pub fn builtin_count(&self) -> usize {
        self.registry
            .all_notations()
            .iter()
            .filter(|n| n.is_builtin)
            .count()
    }
}
#[allow(dead_code)]
pub struct NotationDocstringRegistry {
    docs: HashMap<Name, NotationDocstring>,
}
#[allow(dead_code)]
impl NotationDocstringRegistry {
    pub fn new() -> Self {
        NotationDocstringRegistry {
            docs: HashMap::new(),
        }
    }
    pub fn register(&mut self, doc: NotationDocstring) {
        self.docs.insert(doc.notation_name.clone(), doc);
    }
    pub fn lookup(&self, name: &Name) -> Option<&NotationDocstring> {
        self.docs.get(name)
    }
    pub fn len(&self) -> usize {
        self.docs.len()
    }
    pub fn is_empty(&self) -> bool {
        self.docs.is_empty()
    }
}
#[allow(dead_code)]
pub struct NotationScope {
    level: usize,
    notations: Vec<Notation>,
}
#[allow(dead_code)]
impl NotationScope {
    pub fn new(level: usize) -> Self {
        NotationScope {
            level,
            notations: Vec::new(),
        }
    }
    pub fn add(&mut self, n: Notation) {
        self.notations.push(n);
    }
    pub fn notations(&self) -> &[Notation] {
        &self.notations
    }
    pub fn level(&self) -> usize {
        self.level
    }
    pub fn count(&self) -> usize {
        self.notations.len()
    }
}
#[allow(dead_code)]
pub struct NotationDSL {
    notations: Vec<Notation>,
}
#[allow(dead_code)]
impl NotationDSL {
    pub fn new() -> Self {
        NotationDSL {
            notations: Vec::new(),
        }
    }
    pub fn infixl(mut self, sym: &str, prec: u32, expansion: Name) -> Self {
        self.notations.push(Notation {
            name: Name::str(format!("infixl_{}", sym)),
            kind: NotationKind::Infixl { precedence: prec },
            pattern: sym.to_string(),
            parts: vec![],
            expansion: NotationExpansion::Simple(Expr::Const(expansion, vec![])),
            priority: prec,
            scope: None,
            is_builtin: false,
        });
        self
    }
    pub fn infixr(mut self, sym: &str, prec: u32, expansion: Name) -> Self {
        self.notations.push(Notation {
            name: Name::str(format!("infixr_{}", sym)),
            kind: NotationKind::Infixr { precedence: prec },
            pattern: sym.to_string(),
            parts: vec![],
            expansion: NotationExpansion::Simple(Expr::Const(expansion, vec![])),
            priority: prec,
            scope: None,
            is_builtin: false,
        });
        self
    }
    pub fn prefix(mut self, sym: &str, prec: u32, expansion: Name) -> Self {
        self.notations.push(Notation {
            name: Name::str(format!("prefix_{}", sym)),
            kind: NotationKind::Prefix { precedence: prec },
            pattern: sym.to_string(),
            parts: vec![],
            expansion: NotationExpansion::Simple(Expr::Const(expansion, vec![])),
            priority: prec,
            scope: None,
            is_builtin: false,
        });
        self
    }
    pub fn build(self) -> Vec<Notation> {
        self.notations
    }
    pub fn count(&self) -> usize {
        self.notations.len()
    }
}
#[allow(dead_code)]
pub struct NotationConflictDetector;
#[allow(dead_code)]
impl NotationConflictDetector {
    pub fn new() -> Self {
        NotationConflictDetector
    }
    pub fn detect(&self, registry: &NotationRegistry) -> Vec<NotationConflict> {
        let mut conflicts = Vec::new();
        let all: Vec<&Notation> = registry.all_notations().iter().collect();
        let mut by_sym: HashMap<&str, Vec<&Notation>> = HashMap::new();
        for n in &all {
            by_sym.entry(&n.pattern).or_default().push(n);
        }
        for (sym, group) in &by_sym {
            for i in 0..group.len() {
                for j in (i + 1)..group.len() {
                    if group[i].priority == group[j].priority {
                        conflicts.push(NotationConflict {
                            symbol: sym.to_string(),
                            first: group[i].name.clone(),
                            second: group[j].name.clone(),
                            conflict_type: NotationConflictType::SamePrecedence,
                        });
                    }
                }
            }
        }
        conflicts
    }
    pub fn has_conflicts(&self, registry: &NotationRegistry) -> bool {
        !self.detect(registry).is_empty()
    }
}
#[allow(dead_code)]
pub struct NotationSearchIndex {
    by_symbol: HashMap<String, Vec<usize>>,
    by_kind: HashMap<String, Vec<usize>>,
    notations: Vec<Notation>,
}
#[allow(dead_code)]
impl NotationSearchIndex {
    pub fn new() -> Self {
        NotationSearchIndex {
            by_symbol: HashMap::new(),
            by_kind: HashMap::new(),
            notations: Vec::new(),
        }
    }
    pub fn insert(&mut self, notation: Notation) {
        let idx = self.notations.len();
        let sym = notation.pattern.clone();
        let kind_key = format!("{:?}", notation.kind);
        self.by_symbol.entry(sym).or_default().push(idx);
        self.by_kind.entry(kind_key).or_default().push(idx);
        self.notations.push(notation);
    }
    pub fn lookup_by_symbol(&self, sym: &str) -> Vec<&Notation> {
        self.by_symbol
            .get(sym)
            .map(|idxs| idxs.iter().map(|&i| &self.notations[i]).collect())
            .unwrap_or_default()
    }
    pub fn lookup_by_kind(&self, kind: &NotationKind) -> Vec<&Notation> {
        let key = format!("{:?}", kind);
        self.by_kind
            .get(&key)
            .map(|idxs| idxs.iter().map(|&i| &self.notations[i]).collect())
            .unwrap_or_default()
    }
    pub fn count(&self) -> usize {
        self.notations.len()
    }
    pub fn all(&self) -> &[Notation] {
        &self.notations
    }
}
#[allow(dead_code)]
pub struct NotationExpansionCache {
    cache: HashMap<String, Expr>,
    hit_count: u64,
    miss_count: u64,
    capacity: usize,
}
#[allow(dead_code)]
impl NotationExpansionCache {
    pub fn new(capacity: usize) -> Self {
        NotationExpansionCache {
            cache: HashMap::new(),
            hit_count: 0,
            miss_count: 0,
            capacity,
        }
    }
    pub fn get(&mut self, key: &str) -> Option<&Expr> {
        if let Some(val) = self.cache.get(key) {
            self.hit_count += 1;
            Some(val)
        } else {
            self.miss_count += 1;
            None
        }
    }
    pub fn insert(&mut self, key: String, expr: Expr) {
        if self.cache.len() >= self.capacity {
            if let Some(first_key) = self.cache.keys().next().cloned() {
                self.cache.remove(&first_key);
            }
        }
        self.cache.insert(key, expr);
    }
    pub fn hit_rate(&self) -> f64 {
        let total = self.hit_count + self.miss_count;
        if total == 0 {
            0.0
        } else {
            self.hit_count as f64 / total as f64
        }
    }
    pub fn size(&self) -> usize {
        self.cache.len()
    }
    pub fn clear(&mut self) {
        self.cache.clear();
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct NotationDocstring {
    pub notation_name: Name,
    pub summary: String,
    pub examples: Vec<String>,
}
#[allow(dead_code)]
impl NotationDocstring {
    pub fn new(notation_name: Name, summary: impl Into<String>) -> Self {
        NotationDocstring {
            notation_name,
            summary: summary.into(),
            examples: Vec::new(),
        }
    }
    pub fn with_example(mut self, ex: impl Into<String>) -> Self {
        self.examples.push(ex.into());
        self
    }
    pub fn has_examples(&self) -> bool {
        !self.examples.is_empty()
    }
    pub fn example_count(&self) -> usize {
        self.examples.len()
    }
}
#[allow(dead_code)]
pub struct NotationPrettyPrinter {
    show_expansion: bool,
    show_precedence: bool,
}
#[allow(dead_code)]
impl NotationPrettyPrinter {
    pub fn new() -> Self {
        NotationPrettyPrinter {
            show_expansion: true,
            show_precedence: true,
        }
    }
    pub fn without_expansion(mut self) -> Self {
        self.show_expansion = false;
        self
    }
    pub fn without_precedence(mut self) -> Self {
        self.show_precedence = false;
        self
    }
    pub fn print(&self, n: &Notation) -> String {
        let mut s = format!("notation `{}` ({:?})", n.pattern, n.kind);
        if self.show_precedence {
            s.push_str(&format!(" prec={}", n.priority));
        }
        if matches!(
            n.kind,
            NotationKind::Infixl { .. } | NotationKind::Infixr { .. }
        ) {
            s.push_str(" left-assoc");
        }
        s
    }
    pub fn print_all(&self, notations: &[Notation]) -> Vec<String> {
        notations.iter().map(|n| self.print(n)).collect()
    }
}
#[allow(dead_code)]
pub struct NotationPatternCompiler;
#[allow(dead_code)]
impl NotationPatternCompiler {
    pub fn new() -> Self {
        NotationPatternCompiler
    }
    /// Computes the "token count" for a notation pattern.
    pub fn token_count(&self, pattern: &str) -> usize {
        let mut tz = NotationTokenizer::new(pattern);
        tz.tokenize_all()
            .iter()
            .filter(|t| !matches!(t, NotationToken::Whitespace | NotationToken::EndOfInput))
            .count()
    }
    /// Checks whether a pattern is "simple" (just one symbol).
    pub fn is_simple(&self, pattern: &str) -> bool {
        self.token_count(pattern) == 1
    }
    /// Extracts all symbol and keyword tokens from a pattern.
    ///
    /// Placeholders (`_`) and whitespace are excluded.
    pub fn extract_symbols(&self, pattern: &str) -> Vec<String> {
        let mut tz = NotationTokenizer::new(pattern);
        tz.tokenize_all()
            .into_iter()
            .filter_map(|t| match t {
                NotationToken::Symbol(s) => Some(s),
                NotationToken::Identifier(s) if s != "_" => Some(s),
                _ => None,
            })
            .collect()
    }
}
#[allow(dead_code)]
pub struct NotationEnvironment {
    pub elab: NotationElabContext,
    pub conflict_detector: NotationConflictDetector,
    pub sorter: NotationSorter,
    pub compiler: NotationPatternCompiler,
    pub doc_registry: NotationDocstringRegistry,
}
#[allow(dead_code)]
impl NotationEnvironment {
    pub fn new() -> Self {
        NotationEnvironment {
            elab: NotationElabContext::new(),
            conflict_detector: NotationConflictDetector::new(),
            sorter: NotationSorter,
            compiler: NotationPatternCompiler::new(),
            doc_registry: NotationDocstringRegistry::new(),
        }
    }
    pub fn register_notation_with_doc(&mut self, n: Notation, doc: NotationDocstring) {
        self.elab.register_notation(n);
        self.doc_registry.register(doc);
    }
    pub fn detect_conflicts(&self) -> Vec<NotationConflict> {
        self.conflict_detector.detect(&self.elab.registry)
    }
    pub fn has_conflicts(&self) -> bool {
        !self.detect_conflicts().is_empty()
    }
}
/// A notation definition.
#[derive(Clone, Debug)]
pub struct Notation {
    /// The name identifying this notation.
    pub name: Name,
    /// The kind (prefix, infix, postfix, mixfix, macro).
    pub kind: NotationKind,
    /// The pattern to match (kept for simple string-based lookups).
    pub pattern: String,
    /// Structured pattern parts for mixfix notations.
    pub parts: Vec<NotationPart>,
    /// How this notation expands.
    pub expansion: NotationExpansion,
    /// Priority (higher = tried first).
    pub priority: u32,
    /// Optional scope restriction.
    pub scope: Option<Name>,
    /// Whether this is a built-in notation.
    pub is_builtin: bool,
}
impl Notation {
    /// Create a simple notation from a pattern/expansion string pair (legacy compat).
    #[allow(dead_code)]
    pub fn simple(pattern: &str, expansion_name: &str) -> Self {
        Self {
            name: Name::str(pattern),
            kind: NotationKind::Notation,
            pattern: pattern.to_string(),
            parts: vec![NotationPart::Literal(pattern.to_string())],
            expansion: NotationExpansion::Simple(Expr::Const(Name::str(expansion_name), vec![])),
            priority: 0,
            scope: None,
            is_builtin: false,
        }
    }
}
/// How a notation expands into a kernel expression.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NotationExpansion {
    /// Direct expression expansion (constant application).
    Simple(Expr),
    /// Template-based expansion with argument references.
    Template(Vec<ExpansionPart>),
    /// Delegate to a custom expansion function by name.
    Custom(Name),
}
/// The kind of a notation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NotationKind {
    /// Prefix operator, e.g. `!` or `-` (unary).
    Prefix {
        /// Binding precedence.
        precedence: u32,
    },
    /// Left-associative infix operator, e.g. `+`, `*`.
    Infixl {
        /// Binding precedence.
        precedence: u32,
    },
    /// Right-associative infix operator, e.g. `^`, `::`.
    Infixr {
        /// Binding precedence.
        precedence: u32,
    },
    /// Postfix operator, e.g. `!` (factorial).
    Postfix {
        /// Binding precedence.
        precedence: u32,
    },
    /// General mixfix notation.
    Notation,
    /// Notation backed by a macro expansion.
    Macro,
}
impl NotationKind {
    /// Get the precedence of this notation kind, if applicable.
    #[allow(dead_code)]
    pub fn precedence(&self) -> Option<u32> {
        match self {
            NotationKind::Prefix { precedence } => Some(*precedence),
            NotationKind::Infixl { precedence } => Some(*precedence),
            NotationKind::Infixr { precedence } => Some(*precedence),
            NotationKind::Postfix { precedence } => Some(*precedence),
            NotationKind::Notation | NotationKind::Macro => None,
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotationToken {
    Symbol(String),
    Identifier(String),
    Number(u64),
    Whitespace,
    EndOfInput,
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotationConflictType {
    SamePrecedence,
    SameSymbol,
    AssociativityConflict,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct NotationConflict {
    pub symbol: String,
    pub first: Name,
    pub second: Name,
    pub conflict_type: NotationConflictType,
}
#[allow(dead_code)]
pub struct NotationScopeStack {
    scopes: Vec<NotationScope>,
}
#[allow(dead_code)]
impl NotationScopeStack {
    pub fn new() -> Self {
        NotationScopeStack { scopes: Vec::new() }
    }
    pub fn push(&mut self) {
        let level = self.scopes.len();
        self.scopes.push(NotationScope::new(level));
    }
    pub fn pop(&mut self) -> Option<NotationScope> {
        self.scopes.pop()
    }
    pub fn add_to_current(&mut self, n: Notation) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.add(n);
        }
    }
    pub fn visible(&self) -> Vec<&Notation> {
        self.scopes.iter().flat_map(|s| s.notations()).collect()
    }
    pub fn depth(&self) -> usize {
        self.scopes.len()
    }
    pub fn is_empty(&self) -> bool {
        self.scopes.is_empty()
    }
}
#[allow(dead_code)]
pub struct NotationSorter;
#[allow(dead_code)]
impl NotationSorter {
    pub fn by_priority(notations: &mut [Notation]) {
        notations.sort_by_key(|b| std::cmp::Reverse(b.priority));
    }
    pub fn by_pattern_length(notations: &mut [Notation]) {
        notations.sort_by_key(|b| std::cmp::Reverse(b.pattern.len()));
    }
}
#[allow(dead_code)]
pub struct NotationExtensionMarker;
#[allow(dead_code)]
impl NotationExtensionMarker {
    pub fn new() -> Self {
        NotationExtensionMarker
    }
    pub fn description() -> &'static str {
        "NotationExtensionMarker: placeholder for future notation system extensions."
    }
}
/// A fragment in a template-based expansion.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExpansionPart {
    /// A constant name reference.
    Text(Name),
    /// A reference to the i-th matched argument.
    Arg(usize),
    /// Application of two expansion parts.
    App(Box<ExpansionPart>, Box<ExpansionPart>),
}
