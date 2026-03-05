//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use super::prologgoalbuilder_type::PrologGoalBuilder;

/// Right-hand side element of a DCG rule.
#[derive(Debug, Clone, PartialEq)]
pub enum DcgRhs {
    /// A nonterminal call.
    NonTerminal(PrologTerm),
    /// A terminal list `[a, b, c]`.
    Terminals(Vec<PrologTerm>),
    /// An epsilon (empty string) — written `[]`.
    Epsilon,
    /// A Prolog goal `{Goal1, Goal2}`.
    Goal(Vec<PrologTerm>),
    /// A disjunction `(A | B)`.
    Disjunction(Vec<DcgRhs>, Vec<DcgRhs>),
    /// A pushback notation `A, B` sequence.
    Seq(Vec<DcgRhs>),
}
/// A top-level Prolog directive `:- Goal.`
#[derive(Debug, Clone, PartialEq)]
pub enum PrologDirective {
    /// `:- module(Name, [Exports]).`
    Module(String, Vec<String>),
    /// `:- use_module(library(Name)).`
    UseModuleLibrary(String),
    /// `:- use_module(path).`
    UseModulePath(String),
    /// `:- use_module(path, [Imports]).`
    UseModuleImports(String, Vec<String>),
    /// `:- dynamic Name/Arity.`
    Dynamic(String, usize),
    /// `:- discontiguous Name/Arity.`
    Discontiguous(String, usize),
    /// `:- ensure_loaded(path).`
    EnsureLoaded(String),
    /// `:- module_info.`
    ModuleInfo,
    /// `:- set_prolog_flag(Flag, Value).`
    SetPrologFlag(String, String),
    /// `:- op(Priority, Type, Operator).`
    Op(u16, String, String),
    /// `:- meta_predicate Declaration.`
    MetaPredicate(PrologTerm),
    /// An arbitrary directive goal.
    Arbitrary(PrologTerm),
}
impl PrologDirective {
    /// Emit this directive as a Prolog string.
    pub fn emit(&self) -> String {
        match self {
            PrologDirective::Module(name, exports) => {
                let exp_str = exports.join(", ");
                format!(":- module({}, [{}]).", name, exp_str)
            }
            PrologDirective::UseModuleLibrary(lib) => {
                format!(":- use_module(library({})).", lib)
            }
            PrologDirective::UseModulePath(path) => format!(":- use_module({}).", path),
            PrologDirective::UseModuleImports(path, imports) => {
                let imp_str = imports.join(", ");
                format!(":- use_module({}, [{}]).", path, imp_str)
            }
            PrologDirective::Dynamic(name, arity) => {
                format!(":- dynamic {}/{}.", name, arity)
            }
            PrologDirective::Discontiguous(name, arity) => {
                format!(":- discontiguous {}/{}.", name, arity)
            }
            PrologDirective::EnsureLoaded(path) => format!(":- ensure_loaded({}).", path),
            PrologDirective::ModuleInfo => ":- module_info.".to_string(),
            PrologDirective::SetPrologFlag(flag, val) => {
                format!(":- set_prolog_flag({}, {}).", flag, val)
            }
            PrologDirective::Op(prio, op_type, op) => {
                format!(":- op({}, {}, {}).", prio, op_type, op)
            }
            PrologDirective::MetaPredicate(term) => {
                format!(":- meta_predicate {}.", term)
            }
            PrologDirective::Arbitrary(goal) => format!(":- {}.", goal),
        }
    }
}
/// Fluent builder for `PrologModule`.
#[allow(dead_code)]
pub struct PrologModuleBuilder {
    pub(super) module: PrologModule,
}
impl PrologModuleBuilder {
    /// Start a named module.
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        PrologModuleBuilder {
            module: PrologModule::new(name),
        }
    }
    /// Start a script (no module declaration).
    #[allow(dead_code)]
    pub fn script() -> Self {
        PrologModuleBuilder {
            module: PrologModule::script(),
        }
    }
    /// Export a predicate.
    #[allow(dead_code)]
    pub fn export(mut self, indicator: impl Into<String>) -> Self {
        self.module.exported_predicates.push(indicator.into());
        self
    }
    /// Add a `use_module(library(Name))` directive.
    #[allow(dead_code)]
    pub fn use_library(mut self, lib: impl Into<String>) -> Self {
        self.module
            .items
            .push(PrologItem::Directive(PrologDirective::UseModuleLibrary(
                lib.into(),
            )));
        self
    }
    /// Add a `use_module(path)` directive.
    #[allow(dead_code)]
    pub fn use_module(mut self, path: impl Into<String>) -> Self {
        self.module
            .items
            .push(PrologItem::Directive(PrologDirective::UseModulePath(
                path.into(),
            )));
        self
    }
    /// Add a predicate.
    #[allow(dead_code)]
    pub fn add_predicate(mut self, pred: PrologPredicate) -> Self {
        self.module.items.push(PrologItem::Predicate(pred));
        self
    }
    /// Add a DCG rule.
    #[allow(dead_code)]
    pub fn add_dcg(mut self, rule: DcgRule) -> Self {
        self.module.items.push(PrologItem::Dcg(rule));
        self
    }
    /// Add a blank line.
    #[allow(dead_code)]
    pub fn blank(mut self) -> Self {
        self.module.items.push(PrologItem::BlankLine);
        self
    }
    /// Add a section comment.
    #[allow(dead_code)]
    pub fn section(mut self, title: impl Into<String>) -> Self {
        self.module
            .items
            .push(PrologItem::SectionComment(title.into()));
        self
    }
    /// Add a line comment.
    #[allow(dead_code)]
    pub fn comment(mut self, text: impl Into<String>) -> Self {
        self.module.items.push(PrologItem::LineComment(text.into()));
        self
    }
    /// Set the description.
    #[allow(dead_code)]
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.module.description = Some(desc.into());
        self
    }
    /// Finalise.
    #[allow(dead_code)]
    pub fn build(self) -> PrologModule {
        self.module
    }
    /// Emit directly.
    #[allow(dead_code)]
    pub fn emit(self) -> String {
        PrologBackend::swi().emit_module(&self.module)
    }
}
/// Fluent builder for `PrologPredicate`.
#[allow(dead_code)]
pub struct PrologPredicateBuilder {
    pub(super) pred: PrologPredicate,
}
impl PrologPredicateBuilder {
    /// Start building a predicate.
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, arity: usize) -> Self {
        PrologPredicateBuilder {
            pred: PrologPredicate::new(name, arity),
        }
    }
    /// Mark as dynamic.
    #[allow(dead_code)]
    pub fn dynamic(mut self) -> Self {
        self.pred.is_dynamic = true;
        self
    }
    /// Mark as exported.
    #[allow(dead_code)]
    pub fn exported(mut self) -> Self {
        self.pred.is_exported = true;
        self
    }
    /// Mark as discontiguous.
    #[allow(dead_code)]
    pub fn discontiguous(mut self) -> Self {
        self.pred.is_discontiguous = true;
        self
    }
    /// Set module qualification.
    #[allow(dead_code)]
    pub fn module(mut self, m: impl Into<String>) -> Self {
        self.pred.module = Some(m.into());
        self
    }
    /// Add a doc comment.
    #[allow(dead_code)]
    pub fn doc(mut self, d: impl Into<String>) -> Self {
        self.pred.doc = Some(d.into());
        self
    }
    /// Add a clause.
    #[allow(dead_code)]
    pub fn clause(mut self, c: PrologClause) -> Self {
        self.pred.clauses.push(c);
        self
    }
    /// Add a fact clause.
    #[allow(dead_code)]
    pub fn fact(mut self, head: PrologTerm) -> Self {
        self.pred.clauses.push(PrologClause::fact(head));
        self
    }
    /// Add a rule clause.
    #[allow(dead_code)]
    pub fn rule(mut self, head: PrologTerm, body: Vec<PrologTerm>) -> Self {
        self.pred.clauses.push(PrologClause::rule(head, body));
        self
    }
    /// Finalise.
    #[allow(dead_code)]
    pub fn build(self) -> PrologPredicate {
        self.pred
    }
    /// Emit.
    #[allow(dead_code)]
    pub fn emit(self) -> String {
        self.pred.emit()
    }
}
/// Fluent builder for `PrologClause`.
#[allow(dead_code)]
pub struct PrologClauseBuilder {
    pub(super) head: PrologTerm,
    pub(super) body: Vec<PrologTerm>,
    pub(super) comment: Option<String>,
}
impl PrologClauseBuilder {
    /// Start building a clause with the given head.
    #[allow(dead_code)]
    pub fn head(head: PrologTerm) -> Self {
        PrologClauseBuilder {
            head,
            body: vec![],
            comment: None,
        }
    }
    /// Add a body goal.
    #[allow(dead_code)]
    pub fn goal(mut self, g: PrologTerm) -> Self {
        self.body.push(g);
        self
    }
    /// Add goals from a `PrologGoalBuilder`.
    #[allow(dead_code)]
    pub fn goals(mut self, builder: PrologGoalBuilder) -> Self {
        self.body.extend(builder.build());
        self
    }
    /// Set a comment.
    #[allow(dead_code)]
    pub fn comment(mut self, c: impl Into<String>) -> Self {
        self.comment = Some(c.into());
        self
    }
    /// Build into a `PrologClause`.
    #[allow(dead_code)]
    pub fn build(self) -> PrologClause {
        let mut clause = if self.body.is_empty() {
            PrologClause::fact(self.head)
        } else {
            PrologClause::rule(self.head, self.body)
        };
        clause.comment = self.comment;
        clause
    }
}
/// An item at the top level of a Prolog source file.
#[derive(Debug, Clone, PartialEq)]
pub enum PrologItem {
    /// A top-level directive.
    Directive(PrologDirective),
    /// A predicate (grouped clauses).
    Predicate(PrologPredicate),
    /// A standalone clause (not grouped into a predicate).
    Clause(PrologClause),
    /// A DCG grammar rule.
    Dcg(DcgRule),
    /// A blank line separator (for readability).
    BlankLine,
    /// A section comment `% === ... ===`.
    SectionComment(String),
    /// A line comment.
    LineComment(String),
}
impl PrologItem {
    /// Emit this item as a Prolog string.
    pub fn emit(&self) -> String {
        match self {
            PrologItem::Directive(d) => d.emit(),
            PrologItem::Predicate(p) => p.emit(),
            PrologItem::Clause(c) => c.emit(),
            PrologItem::Dcg(r) => r.emit(),
            PrologItem::BlankLine => String::new(),
            PrologItem::SectionComment(s) => {
                let bar = "=".repeat(s.len() + 8);
                format!("% {}\n% === {} ===\n% {}", bar, s, bar)
            }
            PrologItem::LineComment(s) => format!("% {}", s),
        }
    }
}
/// Generate complete snippets of common Prolog predicates.
#[allow(dead_code)]
pub struct PrologSnippets;
impl PrologSnippets {
    /// Generate a complete `member/2` predicate.
    #[allow(dead_code)]
    pub fn member_predicate() -> PrologPredicate {
        let mut pred = PrologPredicate::new("member", 2);
        pred.add_clause(PrologClause::fact(compound(
            "member",
            vec![var("X"), PrologTerm::list_partial(vec![var("X")], var("_"))],
        )));
        pred.add_clause(PrologClause::rule(
            compound(
                "member",
                vec![var("X"), PrologTerm::list_partial(vec![var("_")], var("T"))],
            ),
            vec![compound("member", vec![var("X"), var("T")])],
        ));
        pred
    }
    /// Generate a complete `append/3` predicate.
    #[allow(dead_code)]
    pub fn append_predicate() -> PrologPredicate {
        let mut pred = PrologPredicate::new("append", 3);
        pred.add_clause(PrologClause::fact(compound(
            "append",
            vec![PrologTerm::Nil, var("L"), var("L")],
        )));
        pred.add_clause(PrologClause::rule(
            compound(
                "append",
                vec![
                    PrologTerm::list_partial(vec![var("H")], var("T")),
                    var("L"),
                    PrologTerm::list_partial(vec![var("H")], var("R")),
                ],
            ),
            vec![compound("append", vec![var("T"), var("L"), var("R")])],
        ));
        pred
    }
    /// Generate a `length/2` predicate.
    #[allow(dead_code)]
    pub fn length_predicate() -> PrologPredicate {
        let mut pred = PrologPredicate::new("my_length", 2);
        pred.add_clause(PrologClause::fact(compound(
            "my_length",
            vec![PrologTerm::Nil, PrologTerm::Integer(0)],
        )));
        pred.add_clause(PrologClause::rule(
            compound(
                "my_length",
                vec![PrologTerm::list_partial(vec![var("_")], var("T")), var("N")],
            ),
            vec![
                compound("my_length", vec![var("T"), var("N1")]),
                is_eval(var("N"), arith_add(var("N1"), PrologTerm::Integer(1))),
            ],
        ));
        pred
    }
    /// Generate a `max_list/2` predicate.
    #[allow(dead_code)]
    pub fn max_list_predicate() -> PrologPredicate {
        let mut pred = PrologPredicate::new("max_list", 2);
        pred.add_clause(PrologClause::fact(compound(
            "max_list",
            vec![PrologTerm::list(vec![var("X")]), var("X")],
        )));
        pred.add_clause(PrologClause::rule(
            compound(
                "max_list",
                vec![
                    PrologTerm::list_partial(vec![var("H")], var("T")),
                    var("Max"),
                ],
            ),
            vec![
                compound("max_list", vec![var("T"), var("Max1")]),
                is_eval(var("Max"), compound("max", vec![var("H"), var("Max1")])),
            ],
        ));
        pred
    }
    /// Generate a `sum_list/2` predicate.
    #[allow(dead_code)]
    pub fn sum_list_predicate() -> PrologPredicate {
        let mut pred = PrologPredicate::new("sum_list", 2);
        pred.add_clause(PrologClause::fact(compound(
            "sum_list",
            vec![PrologTerm::Nil, PrologTerm::Integer(0)],
        )));
        pred.add_clause(PrologClause::rule(
            compound(
                "sum_list",
                vec![
                    PrologTerm::list_partial(vec![var("H")], var("T")),
                    var("Sum"),
                ],
            ),
            vec![
                compound("sum_list", vec![var("T"), var("Rest")]),
                is_eval(var("Sum"), arith_add(var("H"), var("Rest"))),
            ],
        ));
        pred
    }
    /// Generate a `last/2` predicate.
    #[allow(dead_code)]
    pub fn last_predicate() -> PrologPredicate {
        let mut pred = PrologPredicate::new("my_last", 2);
        pred.add_clause(PrologClause::fact(compound(
            "my_last",
            vec![PrologTerm::list(vec![var("X")]), var("X")],
        )));
        pred.add_clause(PrologClause::rule(
            compound(
                "my_last",
                vec![
                    PrologTerm::list_partial(vec![var("_")], var("T")),
                    var("Last"),
                ],
            ),
            vec![compound("my_last", vec![var("T"), var("Last")])],
        ));
        pred
    }
    /// Generate a `reverse/2` predicate (accumulator style).
    #[allow(dead_code)]
    pub fn reverse_predicate() -> PrologPredicate {
        let mut pred = PrologPredicate::new("my_reverse", 2);
        pred.add_clause(PrologClause::rule(
            compound("my_reverse", vec![var("L"), var("R")]),
            vec![compound(
                "reverse_acc",
                vec![var("L"), PrologTerm::Nil, var("R")],
            )],
        ));
        pred
    }
    /// Generate a `msort_dedup/2` predicate using msort + remove_dups.
    #[allow(dead_code)]
    pub fn msort_dedup_predicate() -> PrologPredicate {
        let mut pred = PrologPredicate::new("msort_dedup", 2);
        pred.add_clause(PrologClause::rule(
            compound("msort_dedup", vec![var("List"), var("Sorted")]),
            vec![
                compound("msort", vec![var("List"), var("Tmp")]),
                compound("list_to_set", vec![var("Tmp"), var("Sorted")]),
            ],
        ));
        pred
    }
    /// Generate a `flatten/2` predicate (simple version).
    #[allow(dead_code)]
    pub fn flatten_predicate() -> PrologPredicate {
        let mut pred = PrologPredicate::new("my_flatten", 2);
        pred.add_clause(PrologClause::fact(compound(
            "my_flatten",
            vec![PrologTerm::Nil, PrologTerm::Nil],
        )));
        pred.add_clause(PrologClause::rule(
            compound(
                "my_flatten",
                vec![
                    PrologTerm::list_partial(vec![var("H")], var("T")),
                    var("Flat"),
                ],
            ),
            vec![
                compound("is_list", vec![var("H")]),
                PrologTerm::Cut,
                compound("my_flatten", vec![var("H"), var("FH")]),
                compound("my_flatten", vec![var("T"), var("FT")]),
                compound("append", vec![var("FH"), var("FT"), var("Flat")]),
            ],
        ));
        pred.add_clause(PrologClause::rule(
            compound(
                "my_flatten",
                vec![
                    PrologTerm::list_partial(vec![var("H")], var("T")),
                    var("Flat"),
                ],
            ),
            vec![
                compound("my_flatten", vec![var("T"), var("FT")]),
                compound(
                    "append",
                    vec![PrologTerm::list(vec![var("H")]), var("FT"), var("Flat")],
                ),
            ],
        ));
        pred
    }
    /// Generate a `nth0/3` predicate.
    #[allow(dead_code)]
    pub fn nth0_predicate() -> PrologPredicate {
        let mut pred = PrologPredicate::new("my_nth0", 3);
        pred.add_clause(PrologClause::fact(compound(
            "my_nth0",
            vec![
                PrologTerm::Integer(0),
                PrologTerm::list_partial(vec![var("H")], var("_")),
                var("H"),
            ],
        )));
        pred.add_clause(PrologClause::rule(
            compound(
                "my_nth0",
                vec![
                    var("N"),
                    PrologTerm::list_partial(vec![var("_")], var("T")),
                    var("Elem"),
                ],
            ),
            vec![
                arith_gt(var("N"), PrologTerm::Integer(0)),
                is_eval(var("N1"), arith_sub(var("N"), PrologTerm::Integer(1))),
                compound("my_nth0", vec![var("N1"), var("T"), var("Elem")]),
            ],
        ));
        pred
    }
}
/// Prolog argument mode.
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum PrologMode {
    /// `+` — argument must be instantiated.
    In,
    /// `-` — argument will be instantiated by the predicate.
    Out,
    /// `?` — argument may or may not be instantiated.
    InOut,
    /// `:` — meta-argument (goal, callable).
    Meta,
    /// `@` — argument will not be further instantiated.
    NotFurther,
}
/// A Prolog predicate: a collection of clauses sharing the same functor/arity.
#[derive(Debug, Clone, PartialEq)]
pub struct PrologPredicate {
    /// Predicate name (functor).
    pub name: String,
    /// Arity.
    pub arity: usize,
    /// All clauses for this predicate (in order).
    pub clauses: Vec<PrologClause>,
    /// Whether this predicate is declared `:- dynamic`.
    pub is_dynamic: bool,
    /// Whether this predicate is exported from its module.
    pub is_exported: bool,
    /// Whether this predicate is declared `:- discontiguous`.
    pub is_discontiguous: bool,
    /// Optional module qualification.
    pub module: Option<String>,
    /// Optional documentation string.
    pub doc: Option<String>,
}
impl PrologPredicate {
    /// Create a new predicate with no clauses.
    pub fn new(name: impl Into<String>, arity: usize) -> Self {
        PrologPredicate {
            name: name.into(),
            arity,
            clauses: vec![],
            is_dynamic: false,
            is_exported: false,
            is_discontiguous: false,
            module: None,
            doc: None,
        }
    }
    /// Add a clause.
    pub fn add_clause(&mut self, clause: PrologClause) {
        self.clauses.push(clause);
    }
    /// Mark as dynamic.
    pub fn dynamic(mut self) -> Self {
        self.is_dynamic = true;
        self
    }
    /// Mark as exported.
    pub fn exported(mut self) -> Self {
        self.is_exported = true;
        self
    }
    /// Mark as discontiguous.
    pub fn discontiguous(mut self) -> Self {
        self.is_discontiguous = true;
        self
    }
    /// The indicator `Name/Arity` as a string.
    pub fn indicator(&self) -> String {
        format!("{}/{}", self.name, self.arity)
    }
    /// Emit all clauses for this predicate.
    pub fn emit(&self) -> String {
        let mut out = String::new();
        if let Some(doc) = &self.doc {
            for line in doc.lines() {
                out.push_str(&format!("%% {}\n", line));
            }
        }
        if self.is_dynamic {
            out.push_str(&format!(":- dynamic {}/{}.\n", self.name, self.arity));
        }
        if self.is_discontiguous {
            out.push_str(&format!(":- discontiguous {}/{}.\n", self.name, self.arity));
        }
        for clause in &self.clauses {
            out.push_str(&clause.emit());
            out.push('\n');
        }
        out
    }
}
/// A complete Prolog source module (maps to a `.pl` file).
#[derive(Debug, Clone)]
pub struct PrologModule {
    /// Module name (used in `:- module/2`). `None` means no module declaration.
    pub name: Option<String>,
    /// Exported predicates as `"Name/Arity"` strings.
    pub exported_predicates: Vec<String>,
    /// Top-level items in order.
    pub items: Vec<PrologItem>,
    /// File-level comment/description.
    pub description: Option<String>,
}
impl PrologModule {
    /// Create a new named module.
    pub fn new(name: impl Into<String>) -> Self {
        PrologModule {
            name: Some(name.into()),
            exported_predicates: vec![],
            items: vec![],
            description: None,
        }
    }
    /// Create a script (no module declaration).
    pub fn script() -> Self {
        PrologModule {
            name: None,
            exported_predicates: vec![],
            items: vec![],
            description: None,
        }
    }
    /// Set the file description.
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }
    /// Export a predicate.
    pub fn export(&mut self, indicator: impl Into<String>) {
        self.exported_predicates.push(indicator.into());
    }
    /// Add an item.
    pub fn add(&mut self, item: PrologItem) {
        self.items.push(item);
    }
    /// Add a directive.
    pub fn directive(&mut self, d: PrologDirective) {
        self.items.push(PrologItem::Directive(d));
    }
    /// Add a predicate.
    pub fn predicate(&mut self, p: PrologPredicate) {
        self.items.push(PrologItem::Predicate(p));
    }
    /// Add a DCG rule.
    pub fn dcg_rule(&mut self, r: DcgRule) {
        self.items.push(PrologItem::Dcg(r));
    }
    /// Add a blank line.
    pub fn blank(&mut self) {
        self.items.push(PrologItem::BlankLine);
    }
    /// Add a section comment.
    pub fn section(&mut self, title: impl Into<String>) {
        self.items.push(PrologItem::SectionComment(title.into()));
    }
}
/// Fluent builder for `DcgRule`.
#[allow(dead_code)]
pub struct PrologDCGBuilder {
    pub(super) lhs: PrologTerm,
    pub(super) rhs: Vec<DcgRhs>,
    pub(super) guards: Vec<PrologTerm>,
    pub(super) comment: Option<String>,
}
impl PrologDCGBuilder {
    /// Start a DCG rule with the given LHS nonterminal.
    #[allow(dead_code)]
    pub fn lhs(lhs: PrologTerm) -> Self {
        PrologDCGBuilder {
            lhs,
            rhs: vec![],
            guards: vec![],
            comment: None,
        }
    }
    /// Add a nonterminal call.
    #[allow(dead_code)]
    pub fn nonterminal(mut self, t: PrologTerm) -> Self {
        self.rhs.push(DcgRhs::NonTerminal(t));
        self
    }
    /// Add terminal tokens.
    #[allow(dead_code)]
    pub fn terminals(mut self, ts: Vec<PrologTerm>) -> Self {
        self.rhs.push(DcgRhs::Terminals(ts));
        self
    }
    /// Add an epsilon.
    #[allow(dead_code)]
    pub fn epsilon(mut self) -> Self {
        self.rhs.push(DcgRhs::Epsilon);
        self
    }
    /// Add a Prolog guard `{Goals}`.
    #[allow(dead_code)]
    pub fn guard(mut self, g: PrologTerm) -> Self {
        self.guards.push(g);
        self
    }
    /// Add a comment.
    #[allow(dead_code)]
    pub fn comment(mut self, c: impl Into<String>) -> Self {
        self.comment = Some(c.into());
        self
    }
    /// Build the `DcgRule`.
    #[allow(dead_code)]
    pub fn build(self) -> DcgRule {
        DcgRule {
            lhs: self.lhs,
            rhs: self.rhs,
            guards: self.guards,
            comment: self.comment,
        }
    }
    /// Emit directly.
    #[allow(dead_code)]
    pub fn emit(self) -> String {
        self.build().emit()
    }
}
/// The Prolog code generation backend.
///
/// Converts a `PrologModule` (or individual items) into `.pl` source text.
#[derive(Debug, Default)]
pub struct PrologBackend {
    /// Whether to emit SWI-Prolog-specific pragmas (`:- use_module(library(lists))`).
    pub swi_mode: bool,
    /// Whether to add `:- encoding(utf8).` at the top.
    pub utf8_encoding: bool,
    /// Extra options reserved for future use.
    pub options: PrologBackendOptions,
}
impl PrologBackend {
    /// Create a backend in default (ISO) mode.
    pub fn new() -> Self {
        PrologBackend {
            swi_mode: false,
            utf8_encoding: false,
            options: PrologBackendOptions {
                blank_between_predicates: true,
                emit_docs: true,
            },
        }
    }
    /// Create a backend targeting SWI-Prolog.
    pub fn swi() -> Self {
        PrologBackend {
            swi_mode: true,
            utf8_encoding: true,
            options: PrologBackendOptions {
                blank_between_predicates: true,
                emit_docs: true,
            },
        }
    }
    /// Emit a complete `PrologModule` as `.pl` source text.
    pub fn emit_module(&self, module: &PrologModule) -> String {
        let mut out = String::new();
        if self.utf8_encoding {
            out.push_str(":- encoding(utf8).\n");
        }
        if let Some(desc) = &module.description {
            out.push_str("%% ");
            out.push_str(desc);
            out.push('\n');
        }
        if let Some(name) = &module.name {
            let exports = module.exported_predicates.join(",\n               ");
            if module.exported_predicates.is_empty() {
                out.push_str(&format!(":- module({}, []).\n", name));
            } else {
                out.push_str(&format!(
                    ":- module({}, [\n               {}\n              ]).\n",
                    name, exports
                ));
            }
            out.push('\n');
        }
        for item in &module.items {
            let s = item.emit();
            if !s.is_empty() {
                out.push_str(&s);
                out.push('\n');
            } else {
                out.push('\n');
            }
            if self.options.blank_between_predicates && matches!(item, PrologItem::Predicate(_)) {
                out.push('\n');
            }
        }
        out
    }
    /// Emit a single clause.
    pub fn emit_clause(&self, clause: &PrologClause) -> String {
        clause.emit()
    }
    /// Emit a single predicate.
    pub fn emit_predicate(&self, pred: &PrologPredicate) -> String {
        pred.emit()
    }
    /// Emit a single DCG rule.
    pub fn emit_dcg(&self, rule: &DcgRule) -> String {
        rule.emit()
    }
    /// Emit a single directive.
    pub fn emit_directive(&self, directive: &PrologDirective) -> String {
        directive.emit()
    }
    /// Emit a list of clauses separated by newlines.
    pub fn emit_clauses(&self, clauses: &[PrologClause]) -> String {
        clauses
            .iter()
            .map(|c| c.emit())
            .collect::<Vec<_>>()
            .join("\n")
    }
    /// Build a standard SWI-Prolog module preamble.
    pub fn build_swi_preamble(
        &self,
        module_name: &str,
        exports: &[(&str, usize)],
        libraries: &[&str],
    ) -> String {
        let mut out = String::new();
        out.push_str(":- encoding(utf8).\n\n");
        let exp_strs: Vec<String> = exports
            .iter()
            .map(|(n, a)| format!("{}/{}", n, a))
            .collect();
        if exp_strs.is_empty() {
            out.push_str(&format!(":- module({}, []).\n", module_name));
        } else {
            let exp_list = exp_strs.join(",\n               ");
            out.push_str(&format!(
                ":- module({}, [\n               {}\n              ]).\n",
                module_name, exp_list
            ));
        }
        for lib in libraries {
            out.push_str(&format!(":- use_module(library({})).\n", lib));
        }
        out
    }
}
/// Build dynamic database manipulation sequences.
#[allow(dead_code)]
pub struct PrologAssertionBuilder;
impl PrologAssertionBuilder {
    /// `assertz(Head)` — assert a fact at the end.
    #[allow(dead_code)]
    pub fn assertz_fact(head: PrologTerm) -> PrologTerm {
        compound("assertz", vec![head])
    }
    /// `asserta(Head)` — assert a fact at the front.
    #[allow(dead_code)]
    pub fn asserta_fact(head: PrologTerm) -> PrologTerm {
        compound("asserta", vec![head])
    }
    /// `assertz((Head :- Body))` — assert a rule at the end.
    #[allow(dead_code)]
    pub fn assertz_rule(head: PrologTerm, body: PrologTerm) -> PrologTerm {
        let rule = PrologTerm::Op(":-".to_string(), Box::new(head), Box::new(body));
        compound("assertz", vec![rule])
    }
    /// `retract(Head)` — retract a fact.
    #[allow(dead_code)]
    pub fn retract(head: PrologTerm) -> PrologTerm {
        compound("retract", vec![head])
    }
    /// `retractall(Head)` — retract all matching facts.
    #[allow(dead_code)]
    pub fn retractall(head: PrologTerm) -> PrologTerm {
        compound("retractall", vec![head])
    }
    /// `abolish(Name/Arity)` — remove all clauses.
    #[allow(dead_code)]
    pub fn abolish(name: &str, arity: usize) -> PrologTerm {
        compound(
            "abolish",
            vec![PrologTerm::Op(
                "/".to_string(),
                Box::new(atom(name)),
                Box::new(PrologTerm::Integer(arity as i64)),
            )],
        )
    }
}
/// Configuration options for the Prolog backend.
#[derive(Debug, Default, Clone)]
pub struct PrologBackendOptions {
    /// Emit blank lines between predicates.
    pub blank_between_predicates: bool,
    /// Emit `%% doc` comments for predicates with doc strings.
    pub emit_docs: bool,
}
/// Build CLP(FD) constraint goals for SWI-Prolog.
#[allow(dead_code)]
pub struct PrologConstraints;
impl PrologConstraints {
    /// `X in Low..High` — domain constraint.
    #[allow(dead_code)]
    pub fn in_range(x: PrologTerm, low: PrologTerm, high: PrologTerm) -> PrologTerm {
        PrologTerm::Op(
            "in".to_string(),
            Box::new(x),
            Box::new(PrologTerm::Op(
                "..".to_string(),
                Box::new(low),
                Box::new(high),
            )),
        )
    }
    /// `X #= Y` — arithmetic equality constraint.
    #[allow(dead_code)]
    pub fn clp_eq(x: PrologTerm, y: PrologTerm) -> PrologTerm {
        PrologTerm::Op("#=".to_string(), Box::new(x), Box::new(y))
    }
    /// `X #\= Y` — arithmetic disequality constraint.
    #[allow(dead_code)]
    pub fn clp_neq(x: PrologTerm, y: PrologTerm) -> PrologTerm {
        PrologTerm::Op("#\\=".to_string(), Box::new(x), Box::new(y))
    }
    /// `X #< Y` — less-than constraint.
    #[allow(dead_code)]
    pub fn clp_lt(x: PrologTerm, y: PrologTerm) -> PrologTerm {
        PrologTerm::Op("#<".to_string(), Box::new(x), Box::new(y))
    }
    /// `X #> Y` — greater-than constraint.
    #[allow(dead_code)]
    pub fn clp_gt(x: PrologTerm, y: PrologTerm) -> PrologTerm {
        PrologTerm::Op("#>".to_string(), Box::new(x), Box::new(y))
    }
    /// `X #=< Y` — less-than-or-equal constraint.
    #[allow(dead_code)]
    pub fn clp_le(x: PrologTerm, y: PrologTerm) -> PrologTerm {
        PrologTerm::Op("#=<".to_string(), Box::new(x), Box::new(y))
    }
    /// `X #>= Y` — greater-than-or-equal constraint.
    #[allow(dead_code)]
    pub fn clp_ge(x: PrologTerm, y: PrologTerm) -> PrologTerm {
        PrologTerm::Op("#>=".to_string(), Box::new(x), Box::new(y))
    }
    /// `all_different(Vars)` — all-different constraint.
    #[allow(dead_code)]
    pub fn all_different(vars: Vec<PrologTerm>) -> PrologTerm {
        compound("all_different", vec![PrologTerm::list(vars)])
    }
    /// `label(Vars)` — labelling goal.
    #[allow(dead_code)]
    pub fn label(vars: Vec<PrologTerm>) -> PrologTerm {
        compound("label", vec![PrologTerm::list(vars)])
    }
    /// `sum(Vars, #=, Sum)` — sum constraint.
    #[allow(dead_code)]
    pub fn sum_eq(vars: Vec<PrologTerm>, sum: PrologTerm) -> PrologTerm {
        compound("sum", vec![PrologTerm::list(vars), atom("#="), sum])
    }
}
/// A Prolog clause: either a fact `Head.` or a rule `Head :- Body.`
#[derive(Debug, Clone, PartialEq)]
pub struct PrologClause {
    /// The head of the clause.
    pub head: PrologTerm,
    /// Body goals (empty for facts).
    pub body: Vec<PrologTerm>,
    /// True if this is a fact (body is empty and displayed without `:-`).
    pub is_fact: bool,
    /// Optional comment for documentation.
    pub comment: Option<String>,
}
impl PrologClause {
    /// Create a fact clause.
    pub fn fact(head: PrologTerm) -> Self {
        PrologClause {
            head,
            body: vec![],
            is_fact: true,
            comment: None,
        }
    }
    /// Create a rule clause.
    pub fn rule(head: PrologTerm, body: Vec<PrologTerm>) -> Self {
        PrologClause {
            head,
            body,
            is_fact: false,
            comment: None,
        }
    }
    /// Add a comment.
    pub fn with_comment(mut self, comment: impl Into<String>) -> Self {
        self.comment = Some(comment.into());
        self
    }
    /// Emit this clause as a Prolog string.
    pub fn emit(&self) -> String {
        let mut out = String::new();
        if let Some(c) = &self.comment {
            out.push_str(&format!("% {}\n", c));
        }
        if self.is_fact || self.body.is_empty() {
            out.push_str(&format!("{}.", self.head));
        } else {
            out.push_str(&format!("{} :-", self.head));
            if self.body.len() == 1 {
                out.push_str(&format!("\n    {}.", self.body[0]));
            } else {
                for (i, goal) in self.body.iter().enumerate() {
                    if i < self.body.len() - 1 {
                        out.push_str(&format!("\n    {},", goal));
                    } else {
                        out.push_str(&format!("\n    {}.", goal));
                    }
                }
            }
        }
        out
    }
}
/// Prolog type descriptor for plDoc-style type checking.
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum PrologType {
    /// `integer`
    Integer,
    /// `float`
    Float,
    /// `atom`
    Atom,
    /// `string`
    PrologString,
    /// `is_list(T)`
    List(Box<PrologType>),
    /// `compound`
    Compound,
    /// `callable`
    Callable,
    /// `term`
    Term,
    /// `boolean` (true/false atom)
    Boolean,
    /// `var`
    Var,
    /// `nonvar`
    Nonvar,
    /// `number`
    Number,
    /// `atomic`
    Atomic,
    /// `positive_integer`
    PositiveInteger,
    /// `nonneg`
    NonNeg,
    /// Custom type name.
    Custom(String),
}
/// A DCG (Definite Clause Grammar) rule: `lhs --> rhs.`
#[derive(Debug, Clone, PartialEq)]
pub struct DcgRule {
    /// Left-hand side nonterminal.
    pub lhs: PrologTerm,
    /// Right-hand side: sequence of nonterminals, terminals `[token]`, and pushback notation.
    pub rhs: Vec<DcgRhs>,
    /// Optional Prolog goals in `{...}`.
    pub guards: Vec<PrologTerm>,
    /// Optional comment.
    pub comment: Option<String>,
}
impl DcgRule {
    /// Emit this DCG rule as a Prolog string.
    pub fn emit(&self) -> String {
        let mut out = String::new();
        if let Some(c) = &self.comment {
            out.push_str(&format!("% {}\n", c));
        }
        out.push_str(&format!("{} -->", self.lhs));
        let mut parts: Vec<String> = self.rhs.iter().map(|r| format!("{}", r)).collect();
        if !self.guards.is_empty() {
            let guard_str = self
                .guards
                .iter()
                .map(|g| format!("{}", g))
                .collect::<Vec<_>>()
                .join(", ");
            parts.push(format!("{{{}}}", guard_str));
        }
        if parts.is_empty() {
            out.push_str("\n    [].");
        } else if parts.len() == 1 {
            out.push_str(&format!("\n    {}.", parts[0]));
        } else {
            for (i, p) in parts.iter().enumerate() {
                if i < parts.len() - 1 {
                    out.push_str(&format!("\n    {},", p));
                } else {
                    out.push_str(&format!("\n    {}.", p));
                }
            }
        }
        out
    }
}
/// A plDoc-style predicate type signature.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct PrologTypeSig {
    /// Predicate name.
    pub name: String,
    /// Parameter types and modes (mode, type).
    pub params: Vec<(PrologMode, PrologType)>,
    /// Optional description.
    pub description: Option<String>,
}
impl PrologTypeSig {
    /// Emit the type signature as a plDoc comment block.
    #[allow(dead_code)]
    pub fn emit_pldoc(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("%% {}/{}\n", self.name, self.params.len()));
        if let Some(desc) = &self.description {
            out.push_str(&format!("%  {}\n", desc));
        }
        for (mode, ty) in &self.params {
            out.push_str(&format!("%  @param {} {}\n", mode, ty));
        }
        out
    }
    /// Emit as a `:- pred` directive (SICStus style).
    #[allow(dead_code)]
    pub fn emit_pred_directive(&self) -> String {
        let param_str: Vec<String> = self
            .params
            .iter()
            .map(|(m, t)| format!("{}({})", m, t))
            .collect();
        format!(":- pred {}({}).\n", self.name, param_str.join(", "))
    }
}
/// Build an arithmetic expression for use in `X is Expr`.
#[allow(dead_code)]
pub struct PrologArith;
impl PrologArith {
    /// Add: `X + Y`.
    #[allow(dead_code)]
    pub fn add(x: PrologTerm, y: PrologTerm) -> PrologTerm {
        arith_add(x, y)
    }
    /// Sub: `X - Y`.
    #[allow(dead_code)]
    pub fn sub(x: PrologTerm, y: PrologTerm) -> PrologTerm {
        arith_sub(x, y)
    }
    /// Mul: `X * Y`.
    #[allow(dead_code)]
    pub fn mul(x: PrologTerm, y: PrologTerm) -> PrologTerm {
        arith_mul(x, y)
    }
    /// Div: `X // Y` — integer division.
    #[allow(dead_code)]
    pub fn idiv(x: PrologTerm, y: PrologTerm) -> PrologTerm {
        PrologTerm::Op("//".to_string(), Box::new(x), Box::new(y))
    }
    /// Mod: `X mod Y`.
    #[allow(dead_code)]
    pub fn mmod(x: PrologTerm, y: PrologTerm) -> PrologTerm {
        arith_mod(x, y)
    }
    /// Absolute value: `abs(X)`.
    #[allow(dead_code)]
    pub fn abs(x: PrologTerm) -> PrologTerm {
        compound("abs", vec![x])
    }
    /// Max: `max(X, Y)`.
    #[allow(dead_code)]
    pub fn max(x: PrologTerm, y: PrologTerm) -> PrologTerm {
        compound("max", vec![x, y])
    }
    /// Min: `min(X, Y)`.
    #[allow(dead_code)]
    pub fn min(x: PrologTerm, y: PrologTerm) -> PrologTerm {
        compound("min", vec![x, y])
    }
    /// Exponentiation: `X ^ Y`.
    #[allow(dead_code)]
    pub fn pow(x: PrologTerm, y: PrologTerm) -> PrologTerm {
        PrologTerm::Op("^".to_string(), Box::new(x), Box::new(y))
    }
    /// Square root: `sqrt(X)`.
    #[allow(dead_code)]
    pub fn sqrt(x: PrologTerm) -> PrologTerm {
        compound("sqrt", vec![x])
    }
    /// Truncate: `truncate(X)`.
    #[allow(dead_code)]
    pub fn truncate(x: PrologTerm) -> PrologTerm {
        compound("truncate", vec![x])
    }
    /// Round: `round(X)`.
    #[allow(dead_code)]
    pub fn round(x: PrologTerm) -> PrologTerm {
        compound("round", vec![x])
    }
    /// Sign: `sign(X)`.
    #[allow(dead_code)]
    pub fn sign(x: PrologTerm) -> PrologTerm {
        compound("sign", vec![x])
    }
    /// Bitwise AND: `X /\ Y`.
    #[allow(dead_code)]
    pub fn bitand(x: PrologTerm, y: PrologTerm) -> PrologTerm {
        PrologTerm::Op("/\\".to_string(), Box::new(x), Box::new(y))
    }
    /// Bitwise OR: `X \/ Y`.
    #[allow(dead_code)]
    pub fn bitor(x: PrologTerm, y: PrologTerm) -> PrologTerm {
        PrologTerm::Op("\\/".to_string(), Box::new(x), Box::new(y))
    }
    /// Bitwise XOR: `X xor Y`.
    #[allow(dead_code)]
    pub fn xor(x: PrologTerm, y: PrologTerm) -> PrologTerm {
        PrologTerm::Op("xor".to_string(), Box::new(x), Box::new(y))
    }
    /// Left shift: `X << Y`.
    #[allow(dead_code)]
    pub fn shl(x: PrologTerm, y: PrologTerm) -> PrologTerm {
        PrologTerm::Op("<<".to_string(), Box::new(x), Box::new(y))
    }
    /// Right shift: `X >> Y`.
    #[allow(dead_code)]
    pub fn shr(x: PrologTerm, y: PrologTerm) -> PrologTerm {
        PrologTerm::Op(">>".to_string(), Box::new(x), Box::new(y))
    }
}
/// A Prolog term (the universal data structure in Prolog).
#[derive(Debug, Clone, PartialEq)]
pub enum PrologTerm {
    /// An atom: lowercase identifier or quoted string, e.g. `foo`, `'hello world'`
    Atom(String),
    /// An integer literal, e.g. `42`, `-7`
    Integer(i64),
    /// A float literal, e.g. `3.14`
    Float(f64),
    /// A variable: uppercase-starting or `_`-prefixed name, e.g. `X`, `_Acc`
    Variable(String),
    /// A compound term: `functor(arg1, arg2, ...)`, e.g. `f(a, b)`, `+(1, 2)`
    Compound(String, Vec<PrologTerm>),
    /// A proper or partial list: `[H|T]`, `[1,2,3]`
    List(Vec<PrologTerm>, Option<Box<PrologTerm>>),
    /// The empty list `[]`
    Nil,
    /// A DCG rule term: `phrase(Rule, List)`
    DcgPhrase(Box<PrologTerm>, Box<PrologTerm>),
    /// An operator term written in infix notation, e.g. `X is Y + 1`
    Op(String, Box<PrologTerm>, Box<PrologTerm>),
    /// A prefix operator term, e.g. `\+(X)`
    PrefixOp(String, Box<PrologTerm>),
    /// A cut `!`
    Cut,
    /// Anonymous variable `_`
    Anon,
}
impl PrologTerm {
    /// Create an atom.
    pub fn atom(s: impl Into<String>) -> Self {
        PrologTerm::Atom(s.into())
    }
    /// Create a variable.
    pub fn var(s: impl Into<String>) -> Self {
        PrologTerm::Variable(s.into())
    }
    /// Create a compound term.
    pub fn compound(functor: impl Into<String>, args: Vec<PrologTerm>) -> Self {
        PrologTerm::Compound(functor.into(), args)
    }
    /// Create a proper list from elements.
    pub fn list(elems: Vec<PrologTerm>) -> Self {
        PrologTerm::List(elems, None)
    }
    /// Create a partial list `[H1,H2,...|T]`.
    pub fn list_partial(elems: Vec<PrologTerm>, tail: PrologTerm) -> Self {
        PrologTerm::List(elems, Some(Box::new(tail)))
    }
    /// Create an infix operator term.
    pub fn op(op: impl Into<String>, lhs: PrologTerm, rhs: PrologTerm) -> Self {
        PrologTerm::Op(op.into(), Box::new(lhs), Box::new(rhs))
    }
    /// Create a prefix operator term.
    pub fn prefix_op(op: impl Into<String>, arg: PrologTerm) -> Self {
        PrologTerm::PrefixOp(op.into(), Box::new(arg))
    }
    /// Arity of this term as a functor (0 for atoms, variables, etc.)
    pub fn functor_arity(&self) -> usize {
        match self {
            PrologTerm::Compound(_, args) => args.len(),
            _ => 0,
        }
    }
    /// True if this term needs parentheses when used as an argument.
    pub(super) fn needs_parens_as_arg(&self) -> bool {
        matches!(self, PrologTerm::Op(_, _, _) | PrologTerm::PrefixOp(_, _))
    }
    /// Whether an atom needs quoting.
    pub(super) fn needs_quoting(s: &str) -> bool {
        if s.is_empty() {
            return true;
        }
        let mut chars = s.chars();
        let first = chars
            .next()
            .expect("s is non-empty; guaranteed by early return on s.is_empty() above");
        if s.chars().all(|c| "#&*+-./:<=>?@\\^~".contains(c)) {
            return false;
        }
        if !first.is_lowercase() && first != '_' {
            return true;
        }
        s.chars().any(|c| !c.is_alphanumeric() && c != '_')
    }
    /// Format an atom, quoting if necessary.
    pub(super) fn fmt_atom(s: &str) -> String {
        if Self::needs_quoting(s) {
            format!("'{}'", s.replace('\'', "\\'"))
        } else {
            s.to_string()
        }
    }
}
/// Standard higher-order predicates in Prolog.
#[allow(dead_code)]
pub struct PrologMetaPredicates;
impl PrologMetaPredicates {
    /// A `maplist/2` call: `maplist(Goal, List)`.
    #[allow(dead_code)]
    pub fn maplist(goal: PrologTerm, list: PrologTerm) -> PrologTerm {
        compound("maplist", vec![goal, list])
    }
    /// A `maplist/3` call: `maplist(Goal, List, Result)`.
    #[allow(dead_code)]
    pub fn maplist2(goal: PrologTerm, list: PrologTerm, result: PrologTerm) -> PrologTerm {
        compound("maplist", vec![goal, list, result])
    }
    /// An `include/3` call.
    #[allow(dead_code)]
    pub fn include(goal: PrologTerm, list: PrologTerm, result: PrologTerm) -> PrologTerm {
        compound("include", vec![goal, list, result])
    }
    /// An `exclude/3` call.
    #[allow(dead_code)]
    pub fn exclude(goal: PrologTerm, list: PrologTerm, result: PrologTerm) -> PrologTerm {
        compound("exclude", vec![goal, list, result])
    }
    /// A `foldl/4` call.
    #[allow(dead_code)]
    pub fn foldl(goal: PrologTerm, list: PrologTerm, v0: PrologTerm, v: PrologTerm) -> PrologTerm {
        compound("foldl", vec![goal, list, v0, v])
    }
    /// An `aggregate_all/3` call.
    #[allow(dead_code)]
    pub fn aggregate_all(template: PrologTerm, goal: PrologTerm, result: PrologTerm) -> PrologTerm {
        compound("aggregate_all", vec![template, goal, result])
    }
    /// A `call/N` call.
    #[allow(dead_code)]
    pub fn call_n(f: PrologTerm, mut args: Vec<PrologTerm>) -> PrologTerm {
        let mut all_args = vec![f];
        all_args.append(&mut args);
        compound("call", all_args)
    }
    /// A `once/1` call.
    #[allow(dead_code)]
    pub fn once(goal: PrologTerm) -> PrologTerm {
        compound("once", vec![goal])
    }
    /// An `ignore/1` call (succeeds even if goal fails).
    #[allow(dead_code)]
    pub fn ignore(goal: PrologTerm) -> PrologTerm {
        compound("ignore", vec![goal])
    }
    /// A `forall/2` call: `forall(Cond, Action)`.
    #[allow(dead_code)]
    pub fn forall(cond: PrologTerm, action: PrologTerm) -> PrologTerm {
        compound("forall", vec![cond, action])
    }
    /// A `findall/3` call.
    #[allow(dead_code)]
    pub fn findall(template: PrologTerm, goal: PrologTerm, bag: PrologTerm) -> PrologTerm {
        compound("findall", vec![template, goal, bag])
    }
    /// A `bagof/3` call.
    #[allow(dead_code)]
    pub fn bagof(template: PrologTerm, goal: PrologTerm, bag: PrologTerm) -> PrologTerm {
        compound("bagof", vec![template, goal, bag])
    }
    /// A `setof/3` call.
    #[allow(dead_code)]
    pub fn setof(template: PrologTerm, goal: PrologTerm, bag: PrologTerm) -> PrologTerm {
        compound("setof", vec![template, goal, bag])
    }
}
