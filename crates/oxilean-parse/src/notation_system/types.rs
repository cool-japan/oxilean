//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashMap;

/// A registered notation declaration.
///
/// This captures the full information from a `notation`, `prefix`, `infix`,
/// etc. declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotationEntry {
    /// The kind of notation (prefix, infix, mixfix, etc.)
    pub kind: NotationKind,
    /// Name of the notation (often the operator symbol)
    pub name: String,
    /// The pattern parts describing the syntax
    pub pattern: Vec<NotationPart>,
    /// The expansion string (the right-hand side of `=>`)
    pub expansion: String,
    /// Priority / precedence level
    pub priority: u32,
    /// Scopes in which this notation is active
    pub scopes: Vec<String>,
}
impl NotationEntry {
    /// Create a new notation entry.
    pub fn new(
        kind: NotationKind,
        name: String,
        pattern: Vec<NotationPart>,
        expansion: String,
        priority: u32,
    ) -> Self {
        Self {
            kind,
            name,
            pattern,
            expansion,
            priority,
            scopes: Vec::new(),
        }
    }
    /// Create a new notation entry with scopes.
    pub fn with_scopes(mut self, scopes: Vec<String>) -> Self {
        self.scopes = scopes;
        self
    }
    /// Check whether this entry belongs to a given scope.
    pub fn in_scope(&self, scope: &str) -> bool {
        self.scopes.iter().any(|s| s == scope)
    }
    /// Check whether this entry is unscoped (global).
    pub fn is_global(&self) -> bool {
        self.scopes.is_empty()
    }
}
/// A scope guard that automatically pops when dropped.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ScopeGuard<'a> {
    pub(super) env: &'a mut NotationEnv,
}
impl<'a> ScopeGuard<'a> {
    /// Create a new scope guard, pushing a new scope.
    #[allow(dead_code)]
    pub fn new(env: &'a mut NotationEnv) -> Self {
        env.push_scope();
        ScopeGuard { env }
    }
}
/// An operator alias entry.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct OperatorAlias {
    /// The alias (surface notation)
    pub alias: String,
    /// The canonical form
    pub canonical: String,
}
impl OperatorAlias {
    /// Create a new alias.
    #[allow(dead_code)]
    pub fn new(alias: &str, canonical: &str) -> Self {
        OperatorAlias {
            alias: alias.to_string(),
            canonical: canonical.to_string(),
        }
    }
}
/// A notation category (e.g. term, tactic, command).
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NotationCategory {
    /// Term-level notation
    Term,
    /// Tactic-level notation
    Tactic,
    /// Command-level notation
    Command,
    /// Custom category
    Custom(String),
}
/// A registered operator (simpler than a full notation entry).
///
/// Operators are the common case: a single symbol with a fixity and precedence
/// that expands to a function application.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorEntry {
    /// The operator symbol (e.g. `+`, `*`, `->`)
    pub symbol: String,
    /// The fixity of the operator
    pub fixity: Fixity,
    /// Precedence level
    pub precedence: u32,
    /// The expansion (fully qualified function name)
    pub expansion: String,
}
impl OperatorEntry {
    /// Create a new operator entry.
    pub fn new(symbol: String, fixity: Fixity, precedence: u32, expansion: String) -> Self {
        Self {
            symbol,
            fixity,
            precedence,
            expansion,
        }
    }
    /// Check whether this operator is a prefix operator.
    pub fn is_prefix(&self) -> bool {
        self.fixity == Fixity::Prefix
    }
    /// Check whether this operator is an infix operator (left or right).
    pub fn is_infix(&self) -> bool {
        matches!(self.fixity, Fixity::Infixl | Fixity::Infixr)
    }
    /// Check whether this operator is a postfix operator.
    pub fn is_postfix(&self) -> bool {
        self.fixity == Fixity::Postfix
    }
    /// Check whether this operator is left-associative.
    pub fn is_left_assoc(&self) -> bool {
        self.fixity == Fixity::Infixl
    }
    /// Check whether this operator is right-associative.
    pub fn is_right_assoc(&self) -> bool {
        self.fixity == Fixity::Infixr
    }
}
/// A registry of operator overloads.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct OverloadRegistry {
    /// All overload entries
    pub entries: Vec<OverloadEntry>,
}
impl OverloadRegistry {
    /// Create a new empty registry.
    #[allow(dead_code)]
    pub fn new() -> Self {
        OverloadRegistry {
            entries: Vec::new(),
        }
    }
    /// Register an overload.
    #[allow(dead_code)]
    pub fn register(&mut self, entry: OverloadEntry) {
        self.entries.push(entry);
    }
    /// Find the highest-priority overload for a symbol.
    #[allow(dead_code)]
    pub fn best_overload(&self, symbol: &str) -> Option<&OverloadEntry> {
        self.entries
            .iter()
            .filter(|e| e.symbol == symbol)
            .max_by_key(|e| e.priority)
    }
    /// Returns all overloads for a symbol.
    #[allow(dead_code)]
    pub fn all_overloads(&self, symbol: &str) -> Vec<&OverloadEntry> {
        self.entries.iter().filter(|e| e.symbol == symbol).collect()
    }
}
/// The main notation/operator table.
///
/// Contains all registered notations and operators, supports scoped activation,
/// and provides fast lookup by symbol.
#[derive(Debug, Clone)]
pub struct NotationTable {
    /// All registered notation entries.
    entries: Vec<NotationEntry>,
    /// Fast lookup by operator symbol.
    operators: HashMap<String, OperatorEntry>,
    /// Scope name -> indices into `entries` that belong to that scope.
    scoped_entries: HashMap<String, Vec<usize>>,
    /// Currently active (opened) scopes.
    active_scopes: Vec<String>,
}
impl NotationTable {
    /// Create an empty notation table.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            operators: HashMap::new(),
            scoped_entries: HashMap::new(),
            active_scopes: Vec::new(),
        }
    }
    /// Register a notation entry.
    pub fn register_notation(&mut self, entry: NotationEntry) {
        let idx = self.entries.len();
        for scope in &entry.scopes {
            self.scoped_entries
                .entry(scope.clone())
                .or_default()
                .push(idx);
        }
        self.entries.push(entry);
    }
    /// Register an operator entry.
    pub fn register_operator(&mut self, entry: OperatorEntry) {
        self.operators.insert(entry.symbol.clone(), entry);
    }
    /// Look up a prefix operator by symbol.
    pub fn lookup_prefix(&self, symbol: &str) -> Option<&OperatorEntry> {
        self.operators
            .get(symbol)
            .filter(|op| op.fixity == Fixity::Prefix)
    }
    /// Look up an infix operator by symbol (either left- or right-associative).
    pub fn lookup_infix(&self, symbol: &str) -> Option<&OperatorEntry> {
        self.operators
            .get(symbol)
            .filter(|op| matches!(op.fixity, Fixity::Infixl | Fixity::Infixr))
    }
    /// Look up a postfix operator by symbol.
    pub fn lookup_postfix(&self, symbol: &str) -> Option<&OperatorEntry> {
        self.operators
            .get(symbol)
            .filter(|op| op.fixity == Fixity::Postfix)
    }
    /// Look up any operator by symbol regardless of fixity.
    pub fn lookup_operator(&self, symbol: &str) -> Option<&OperatorEntry> {
        self.operators.get(symbol)
    }
    /// Get the precedence of an operator by symbol.
    pub fn get_precedence(&self, symbol: &str) -> Option<u32> {
        self.operators.get(symbol).map(|op| op.precedence)
    }
    /// Get a notation entry by index.
    pub fn get_entry(&self, index: usize) -> Option<&NotationEntry> {
        self.entries.get(index)
    }
    /// Get the number of registered notation entries.
    pub fn notation_count(&self) -> usize {
        self.entries.len()
    }
    /// Get the number of registered operators.
    pub fn operator_count(&self) -> usize {
        self.operators.len()
    }
    /// Open a scope, making its notations active.
    ///
    /// Returns the notation entries that belong to that scope.
    pub fn open_scope(&mut self, scope: &str) -> Vec<&NotationEntry> {
        if !self.active_scopes.contains(&scope.to_string()) {
            self.active_scopes.push(scope.to_string());
        }
        self.scoped_entries
            .get(scope)
            .map(|indices| {
                indices
                    .iter()
                    .filter_map(|&idx| self.entries.get(idx))
                    .collect()
            })
            .unwrap_or_default()
    }
    /// Close a scope, deactivating its notations.
    pub fn close_scope(&mut self, scope: &str) {
        self.active_scopes.retain(|s| s != scope);
    }
    /// Check whether a scope is currently active.
    pub fn is_scope_active(&self, scope: &str) -> bool {
        self.active_scopes.contains(&scope.to_string())
    }
    /// Get all currently active scopes.
    pub fn active_scope_names(&self) -> &[String] {
        &self.active_scopes
    }
    /// Get all currently active notations (global + active scopes).
    pub fn active_notations(&self) -> Vec<&NotationEntry> {
        self.entries
            .iter()
            .filter(|entry| {
                entry.is_global() || entry.scopes.iter().any(|s| self.active_scopes.contains(s))
            })
            .collect()
    }
    /// Find all notation entries matching a given name.
    pub fn find_by_name(&self, name: &str) -> Vec<&NotationEntry> {
        self.entries.iter().filter(|e| e.name == name).collect()
    }
    /// Find all notation entries of a given kind.
    pub fn find_by_kind(&self, kind: &NotationKind) -> Vec<&NotationEntry> {
        self.entries.iter().filter(|e| &e.kind == kind).collect()
    }
    /// Find all operators with a given fixity.
    pub fn find_operators_by_fixity(&self, fixity: &Fixity) -> Vec<&OperatorEntry> {
        self.operators
            .values()
            .filter(|op| &op.fixity == fixity)
            .collect()
    }
    /// Get all operator symbols, sorted alphabetically.
    pub fn all_operator_symbols(&self) -> Vec<&str> {
        let mut symbols: Vec<&str> = self.operators.keys().map(|s| s.as_str()).collect();
        symbols.sort();
        symbols
    }
    /// Compare the precedence of two symbols.
    ///
    /// Returns `Some(Ordering)` if both symbols are registered, `None` otherwise.
    pub fn compare_precedence(&self, a: &str, b: &str) -> Option<std::cmp::Ordering> {
        let pa = self.get_precedence(a)?;
        let pb = self.get_precedence(b)?;
        Some(pa.cmp(&pb))
    }
    /// Create a notation table pre-populated with standard Lean 4 operators.
    ///
    /// Includes arithmetic, comparison, logical, arrow, and assignment operators
    /// with their standard precedences.
    pub fn builtin_operators() -> Self {
        let mut table = Self::new();
        table.register_operator(OperatorEntry::new(
            "+".to_string(),
            Fixity::Infixl,
            65,
            "HAdd.hAdd".to_string(),
        ));
        table.register_operator(OperatorEntry::new(
            "-".to_string(),
            Fixity::Infixl,
            65,
            "HSub.hSub".to_string(),
        ));
        table.register_operator(OperatorEntry::new(
            "*".to_string(),
            Fixity::Infixl,
            70,
            "HMul.hMul".to_string(),
        ));
        table.register_operator(OperatorEntry::new(
            "/".to_string(),
            Fixity::Infixl,
            70,
            "HDiv.hDiv".to_string(),
        ));
        table.register_operator(OperatorEntry::new(
            "%".to_string(),
            Fixity::Infixl,
            70,
            "HMod.hMod".to_string(),
        ));
        table.register_operator(OperatorEntry::new(
            "^".to_string(),
            Fixity::Infixr,
            75,
            "HPow.hPow".to_string(),
        ));
        table.register_operator(OperatorEntry::new(
            "=".to_string(),
            Fixity::Infixl,
            50,
            "Eq".to_string(),
        ));
        table.register_operator(OperatorEntry::new(
            "!=".to_string(),
            Fixity::Infixl,
            50,
            "Ne".to_string(),
        ));
        table.register_operator(OperatorEntry::new(
            "<".to_string(),
            Fixity::Infixl,
            50,
            "LT.lt".to_string(),
        ));
        table.register_operator(OperatorEntry::new(
            ">".to_string(),
            Fixity::Infixl,
            50,
            "GT.gt".to_string(),
        ));
        table.register_operator(OperatorEntry::new(
            "<=".to_string(),
            Fixity::Infixl,
            50,
            "LE.le".to_string(),
        ));
        table.register_operator(OperatorEntry::new(
            ">=".to_string(),
            Fixity::Infixl,
            50,
            "GE.ge".to_string(),
        ));
        table.register_operator(OperatorEntry::new(
            "&&".to_string(),
            Fixity::Infixl,
            35,
            "And".to_string(),
        ));
        table.register_operator(OperatorEntry::new(
            "||".to_string(),
            Fixity::Infixl,
            30,
            "Or".to_string(),
        ));
        table.register_operator(OperatorEntry::new(
            "!".to_string(),
            Fixity::Prefix,
            100,
            "Not".to_string(),
        ));
        table.register_operator(OperatorEntry::new(
            "->".to_string(),
            Fixity::Infixr,
            25,
            "Arrow".to_string(),
        ));
        table.register_operator(OperatorEntry::new(
            ":=".to_string(),
            Fixity::Infixl,
            1,
            "Assign".to_string(),
        ));
        let ops: Vec<(String, NotationKind, u32, String)> = vec![
            ("+".into(), NotationKind::Infixl, 65, "HAdd.hAdd".into()),
            ("-".into(), NotationKind::Infixl, 65, "HSub.hSub".into()),
            ("*".into(), NotationKind::Infixl, 70, "HMul.hMul".into()),
            ("/".into(), NotationKind::Infixl, 70, "HDiv.hDiv".into()),
            ("%".into(), NotationKind::Infixl, 70, "HMod.hMod".into()),
            ("^".into(), NotationKind::Infixr, 75, "HPow.hPow".into()),
            ("=".into(), NotationKind::Infixl, 50, "Eq".into()),
            ("!=".into(), NotationKind::Infixl, 50, "Ne".into()),
            ("<".into(), NotationKind::Infixl, 50, "LT.lt".into()),
            (">".into(), NotationKind::Infixl, 50, "GT.gt".into()),
            ("<=".into(), NotationKind::Infixl, 50, "LE.le".into()),
            (">=".into(), NotationKind::Infixl, 50, "GE.ge".into()),
            ("&&".into(), NotationKind::Infixl, 35, "And".into()),
            ("||".into(), NotationKind::Infixl, 30, "Or".into()),
            ("!".into(), NotationKind::Prefix, 100, "Not".into()),
            ("->".into(), NotationKind::Infixr, 25, "Arrow".into()),
            (":=".into(), NotationKind::Infixl, 1, "Assign".into()),
        ];
        for (sym, kind, prec, expansion) in ops {
            let pattern = match &kind {
                NotationKind::Prefix => {
                    vec![
                        NotationPart::Literal(sym.clone()),
                        NotationPart::Placeholder("x".into()),
                    ]
                }
                NotationKind::Postfix => {
                    vec![
                        NotationPart::Placeholder("x".into()),
                        NotationPart::Literal(sym.clone()),
                    ]
                }
                _ => {
                    vec![
                        NotationPart::Placeholder("lhs".into()),
                        NotationPart::Literal(sym.clone()),
                        NotationPart::Placeholder("rhs".into()),
                    ]
                }
            };
            table.register_notation(NotationEntry::new(kind, sym, pattern, expansion, prec));
        }
        table
    }
}
/// A simple notation formatter that expands `_ op _` patterns.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct NotationFormatter {
    /// Registry to use for expansion
    pub registry: NotationRegistry,
}
impl NotationFormatter {
    /// Create a new formatter.
    #[allow(dead_code)]
    pub fn new(reg: NotationRegistry) -> Self {
        NotationFormatter { registry: reg }
    }
    /// Count total rules.
    #[allow(dead_code)]
    pub fn rule_count(&self) -> usize {
        self.registry.rules.len()
    }
}
/// A user-defined notation rule.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct NotationRule {
    /// The notation pattern (e.g. "_ + _")
    pub pattern: String,
    /// The expansion template
    pub expansion: String,
    /// The precedence level
    pub prec: PrecLevel,
    /// The namespace this rule belongs to
    pub namespace: Option<String>,
}
impl NotationRule {
    /// Create a new notation rule.
    #[allow(dead_code)]
    pub fn new(pattern: &str, expansion: &str, prec: PrecLevel) -> Self {
        NotationRule {
            pattern: pattern.to_string(),
            expansion: expansion.to_string(),
            prec,
            namespace: None,
        }
    }
    /// Set the namespace.
    #[allow(dead_code)]
    pub fn in_namespace(mut self, ns: &str) -> Self {
        self.namespace = Some(ns.to_string());
        self
    }
}
/// A notation token kind for pattern matching.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotationTokenKind {
    /// A literal symbol to match exactly
    Literal(String),
    /// A hole that matches any expression
    Hole,
    /// A named hole
    NamedHole(String),
}
/// An overloaded operator entry.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct OverloadEntry {
    /// The operator symbol
    pub symbol: String,
    /// The type class providing this operator
    pub typeclass: String,
    /// Priority (higher = preferred)
    pub priority: u32,
}
impl OverloadEntry {
    /// Create a new overload entry.
    #[allow(dead_code)]
    pub fn new(symbol: &str, typeclass: &str, priority: u32) -> Self {
        OverloadEntry {
            symbol: symbol.to_string(),
            typeclass: typeclass.to_string(),
            priority,
        }
    }
}
/// A simple notation registry that maps patterns to rules.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct NotationRegistry {
    /// All registered rules
    pub rules: Vec<NotationRule>,
}
impl NotationRegistry {
    /// Create an empty registry.
    #[allow(dead_code)]
    pub fn new() -> Self {
        NotationRegistry { rules: Vec::new() }
    }
    /// Register a new rule.
    #[allow(dead_code)]
    pub fn register(&mut self, rule: NotationRule) {
        self.rules.push(rule);
    }
    /// Find rules by pattern (exact match).
    #[allow(dead_code)]
    pub fn find_by_pattern(&self, pattern: &str) -> Vec<&NotationRule> {
        self.rules.iter().filter(|r| r.pattern == pattern).collect()
    }
    /// Find rules in a given namespace.
    #[allow(dead_code)]
    pub fn find_by_namespace(&self, ns: &str) -> Vec<&NotationRule> {
        self.rules
            .iter()
            .filter(|r| r.namespace.as_deref() == Some(ns))
            .collect()
    }
    /// Returns the total number of rules.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.rules.len()
    }
    /// Returns true if the registry is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }
}
/// A precedence table for built-in operators.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct BuiltinPrecTable {
    /// Entries: (operator, precedence, left_assoc)
    pub entries: Vec<(String, u32, bool)>,
}
impl BuiltinPrecTable {
    /// Create the standard Lean-like precedence table.
    #[allow(dead_code)]
    pub fn standard() -> Self {
        BuiltinPrecTable {
            entries: vec![
                ("=".to_string(), 50, false),
                ("<".to_string(), 50, false),
                (">".to_string(), 50, false),
                ("<=".to_string(), 50, false),
                (">=".to_string(), 50, false),
                ("≠".to_string(), 50, false),
                ("∧".to_string(), 35, true),
                ("∨".to_string(), 30, true),
                ("→".to_string(), 25, false),
                ("↔".to_string(), 20, false),
                ("+".to_string(), 65, true),
                ("-".to_string(), 65, true),
                ("*".to_string(), 70, true),
                ("/".to_string(), 70, true),
                ("%".to_string(), 70, true),
                ("^".to_string(), 75, false),
            ],
        }
    }
    /// Look up the precedence of an operator.
    #[allow(dead_code)]
    pub fn lookup(&self, op: &str) -> Option<(u32, bool)> {
        self.entries
            .iter()
            .find(|(o, _, _)| o == op)
            .map(|(_, p, l)| (*p, *l))
    }
}
/// A notation scope for open/close semantics.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct NotationScope {
    /// Name of this scope
    pub name: String,
    /// Rules added by opening this scope
    pub rules: Vec<NotationRule>,
}
impl NotationScope {
    /// Create a new notation scope.
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        NotationScope {
            name: name.to_string(),
            rules: Vec::new(),
        }
    }
    /// Add a rule to this scope.
    #[allow(dead_code)]
    pub fn add_rule(&mut self, rule: NotationRule) {
        self.rules.push(rule);
    }
}
/// A macro rule with parameters.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct MacroRule {
    /// The macro name
    pub name: String,
    /// Parameter names
    pub params: Vec<String>,
    /// The body template
    pub body: String,
}
impl MacroRule {
    /// Create a new macro rule.
    #[allow(dead_code)]
    pub fn new(name: &str, params: Vec<&str>, body: &str) -> Self {
        MacroRule {
            name: name.to_string(),
            params: params.into_iter().map(|s| s.to_string()).collect(),
            body: body.to_string(),
        }
    }
    /// Instantiate the macro with given arguments (simple text substitution).
    #[allow(dead_code)]
    pub fn instantiate(&self, args: &[&str]) -> String {
        if args.len() != self.params.len() {
            return format!(
                "(macro-error: {} expects {} args, got {})",
                self.name,
                self.params.len(),
                args.len()
            );
        }
        let mut result = self.body.clone();
        for (param, arg) in self.params.iter().zip(args.iter()) {
            result = result.replace(param.as_str(), arg);
        }
        result
    }
}
/// A collection of syntax sugar definitions.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct SyntaxSugarLibrary {
    /// All registered sugars
    pub sugars: Vec<SyntaxSugar>,
}
impl SyntaxSugarLibrary {
    /// Create an empty library.
    #[allow(dead_code)]
    pub fn new() -> Self {
        SyntaxSugarLibrary { sugars: Vec::new() }
    }
    /// Add a sugar.
    #[allow(dead_code)]
    pub fn add(&mut self, sugar: SyntaxSugar) {
        self.sugars.push(sugar);
    }
    /// Find a sugar by name.
    #[allow(dead_code)]
    pub fn find(&self, name: &str) -> Option<&SyntaxSugar> {
        self.sugars.iter().find(|s| s.name == name)
    }
    /// Returns the number of sugars.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.sugars.len()
    }
    /// Returns true if empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.sugars.is_empty()
    }
}
/// A syntax extension hook.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct SyntaxExtension {
    /// Name of the extension
    pub name: String,
    /// Whether this is an infix extension
    pub is_infix: bool,
    /// Whether this is a prefix extension
    pub is_prefix: bool,
    /// Whether this is a postfix extension
    pub is_postfix: bool,
}
impl SyntaxExtension {
    /// Create a new infix syntax extension.
    #[allow(dead_code)]
    pub fn infix(name: &str) -> Self {
        SyntaxExtension {
            name: name.to_string(),
            is_infix: true,
            is_prefix: false,
            is_postfix: false,
        }
    }
    /// Create a new prefix syntax extension.
    #[allow(dead_code)]
    pub fn prefix(name: &str) -> Self {
        SyntaxExtension {
            name: name.to_string(),
            is_infix: false,
            is_prefix: true,
            is_postfix: false,
        }
    }
}
/// A collection of operator aliases.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct OperatorAliasTable {
    /// All entries
    pub entries: Vec<OperatorAlias>,
}
impl OperatorAliasTable {
    /// Create a new empty alias table.
    #[allow(dead_code)]
    pub fn new() -> Self {
        OperatorAliasTable {
            entries: Vec::new(),
        }
    }
    /// Add an alias.
    #[allow(dead_code)]
    pub fn add(&mut self, alias: OperatorAlias) {
        self.entries.push(alias);
    }
    /// Resolve an alias to its canonical form.
    #[allow(dead_code)]
    pub fn resolve(&self, op: &str) -> String {
        self.entries
            .iter()
            .find(|e| e.alias == op)
            .map(|e| e.canonical.clone())
            .unwrap_or_else(|| op.to_string())
    }
    /// Returns the number of entries.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Returns true if empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
/// A notation group that bundles related notation rules.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct NotationGroup {
    /// Group name
    pub name: String,
    /// Rules in this group
    pub rules: Vec<NotationRule>,
    /// Whether this group is currently active
    pub active: bool,
}
impl NotationGroup {
    /// Create a new active group.
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        NotationGroup {
            name: name.to_string(),
            rules: Vec::new(),
            active: true,
        }
    }
    /// Add a rule.
    #[allow(dead_code)]
    pub fn add(&mut self, rule: NotationRule) {
        self.rules.push(rule);
    }
    /// Deactivate this group.
    #[allow(dead_code)]
    pub fn deactivate(&mut self) {
        self.active = false;
    }
    /// Returns active rules.
    #[allow(dead_code)]
    pub fn active_rules(&self) -> Vec<&NotationRule> {
        if self.active {
            self.rules.iter().collect()
        } else {
            Vec::new()
        }
    }
}
/// A priority queue for notation rules (by precedence).
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct NotationPriorityQueue {
    /// Rules sorted by precedence (descending)
    pub rules: Vec<NotationRule>,
}
impl NotationPriorityQueue {
    /// Create a new empty queue.
    #[allow(dead_code)]
    pub fn new() -> Self {
        NotationPriorityQueue { rules: Vec::new() }
    }
    /// Insert a rule (maintaining sort order).
    #[allow(dead_code)]
    pub fn insert(&mut self, rule: NotationRule) {
        let pos = self
            .rules
            .partition_point(|r| r.prec.value >= rule.prec.value);
        self.rules.insert(pos, rule);
    }
    /// Returns rules with at least the given precedence.
    #[allow(dead_code)]
    pub fn rules_at_or_above(&self, prec: u32) -> Vec<&NotationRule> {
        self.rules.iter().filter(|r| r.prec.value >= prec).collect()
    }
    /// Returns the number of rules.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.rules.len()
    }
    /// Returns true if empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }
}
/// The kind of a notation declaration.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NotationKind {
    /// Prefix operator (e.g. `!`, `-`)
    Prefix,
    /// Postfix operator (e.g. `!` factorial)
    Postfix,
    /// Left-associative infix operator (e.g. `+`, `-`)
    Infixl,
    /// Right-associative infix operator (e.g. `->`, `^`)
    Infixr,
    /// General mixfix notation (e.g. `if _ then _ else _`)
    Notation,
}
/// An abstract syntax sugar definition.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct SyntaxSugar {
    /// Name of this syntactic sugar
    pub name: String,
    /// The surface form
    pub surface: String,
    /// The desugared core form
    pub core: String,
}
impl SyntaxSugar {
    /// Create a new syntax sugar definition.
    #[allow(dead_code)]
    pub fn new(name: &str, surface: &str, core: &str) -> Self {
        SyntaxSugar {
            name: name.to_string(),
            surface: surface.to_string(),
            core: core.to_string(),
        }
    }
}
/// A simple token pattern matcher for notation expansion.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct PatternMatcher {
    /// The pattern to match (tokens separated by spaces, `_` = hole)
    pub pattern: Vec<String>,
}
impl PatternMatcher {
    /// Create from a pattern string.
    #[allow(dead_code)]
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Self {
        PatternMatcher {
            pattern: s.split_whitespace().map(|t| t.to_string()).collect(),
        }
    }
    /// Count holes (underscores) in the pattern.
    #[allow(dead_code)]
    pub fn hole_count(&self) -> usize {
        self.pattern.iter().filter(|t| t.as_str() == "_").count()
    }
    /// Returns true if the pattern is entirely holes.
    #[allow(dead_code)]
    pub fn all_holes(&self) -> bool {
        self.pattern.iter().all(|t| t.as_str() == "_")
    }
}
/// Fixity of an operator entry.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Fixity {
    /// Prefix operator
    Prefix,
    /// Left-associative infix operator
    Infixl,
    /// Right-associative infix operator
    Infixr,
    /// Postfix operator
    Postfix,
}
/// A notation conflict: two rules with the same pattern but different expansions.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct NotationConflict {
    /// The conflicting pattern
    pub pattern: String,
    /// First expansion
    pub expansion_a: String,
    /// Second expansion
    pub expansion_b: String,
}
/// A notation environment with scoped rules.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct NotationEnv {
    /// Stack of scopes, outermost first
    pub scopes: Vec<Vec<NotationRule>>,
}
impl NotationEnv {
    /// Create a new notation environment with one empty scope.
    #[allow(dead_code)]
    pub fn new() -> Self {
        NotationEnv {
            scopes: vec![Vec::new()],
        }
    }
    /// Push a new scope.
    #[allow(dead_code)]
    pub fn push_scope(&mut self) {
        self.scopes.push(Vec::new());
    }
    /// Pop the current scope.
    #[allow(dead_code)]
    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }
    /// Add a rule to the current scope.
    #[allow(dead_code)]
    pub fn add(&mut self, rule: NotationRule) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.push(rule);
        }
    }
    /// Look up all rules for a pattern (innermost scope first).
    #[allow(dead_code)]
    pub fn lookup(&self, pattern: &str) -> Vec<&NotationRule> {
        let mut result = Vec::new();
        for scope in self.scopes.iter().rev() {
            for rule in scope {
                if rule.pattern == pattern {
                    result.push(rule);
                }
            }
        }
        result
    }
    /// Total number of rules across all scopes.
    #[allow(dead_code)]
    pub fn total_rules(&self) -> usize {
        self.scopes.iter().map(|s| s.len()).sum()
    }
}
/// A notation precedence level with associativity.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrecLevel {
    /// The precedence value (higher = tighter binding)
    pub value: u32,
    /// Whether this is left-associative
    pub left_assoc: bool,
    /// Whether this is right-associative
    pub right_assoc: bool,
}
impl PrecLevel {
    /// Create a new left-associative precedence level.
    #[allow(dead_code)]
    pub fn left(value: u32) -> Self {
        PrecLevel {
            value,
            left_assoc: true,
            right_assoc: false,
        }
    }
    /// Create a new right-associative precedence level.
    #[allow(dead_code)]
    pub fn right(value: u32) -> Self {
        PrecLevel {
            value,
            left_assoc: false,
            right_assoc: true,
        }
    }
    /// Create a non-associative level.
    #[allow(dead_code)]
    pub fn none(value: u32) -> Self {
        PrecLevel {
            value,
            left_assoc: false,
            right_assoc: false,
        }
    }
}
/// A single part of a notation pattern.
///
/// Notation patterns are sequences of literals and placeholders:
/// ```text
/// notation:50 lhs " + " rhs => HAdd.hAdd lhs rhs
///              ^^^  ^^^^^  ^^^
///          Placeholder Literal Placeholder
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotationPart {
    /// A literal string token (e.g. `" + "`, `"if"`)
    Literal(String),
    /// A placeholder for a variable position (e.g. `lhs`, `rhs`)
    Placeholder(String),
    /// A precedence annotation on a placeholder
    Prec(u32),
}
