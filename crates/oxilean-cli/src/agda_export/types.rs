//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use std::collections::HashMap;
use std::path::Path;

/// An Agda pattern-matching clause.
#[derive(Debug, Clone)]
pub struct AgdaClause {
    pub patterns: Vec<String>,
    pub body: String,
}
impl AgdaClause {
    pub fn new(patterns: Vec<String>, body: &str) -> Self {
        AgdaClause {
            patterns,
            body: body.to_string(),
        }
    }
    /// Render as `f p1 p2 = body`.
    pub fn render(&self, fn_name: &str) -> String {
        if self.patterns.is_empty() {
            format!("{} = {}", fn_name, self.body)
        } else {
            format!("{} {} = {}", fn_name, self.patterns.join(" "), self.body)
        }
    }
}
/// Agda proof terms and tactics (via Agda's reflection).
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum AgdaProofTerm {
    Refl,
    Trans(Box<AgdaProofTerm>, Box<AgdaProofTerm>),
    Sym(Box<AgdaProofTerm>),
    Cong(String, Box<AgdaProofTerm>),
    Lambda(String, Box<AgdaProofTerm>),
    App(Box<AgdaProofTerm>, Box<AgdaProofTerm>),
    Var(String),
    Hole(String),
}
impl AgdaProofTerm {
    /// Render to Agda source.
    #[allow(dead_code)]
    pub fn render(&self) -> String {
        match self {
            AgdaProofTerm::Refl => "refl".to_string(),
            AgdaProofTerm::Sym(p) => format!("sym ({})", p.render()),
            AgdaProofTerm::Trans(a, b) => {
                format!("trans ({}) ({})", a.render(), b.render())
            }
            AgdaProofTerm::Cong(f, p) => format!("cong {} ({})", f, p.render()),
            AgdaProofTerm::Lambda(v, body) => format!("λ {} → {}", v, body.render()),
            AgdaProofTerm::App(f, x) => format!("({}) ({})", f.render(), x.render()),
            AgdaProofTerm::Var(name) => name.clone(),
            AgdaProofTerm::Hole(name) => format!("?{}", name),
        }
    }
    /// Count the depth of the proof tree.
    #[allow(dead_code)]
    pub fn depth(&self) -> usize {
        match self {
            AgdaProofTerm::Refl | AgdaProofTerm::Var(_) | AgdaProofTerm::Hole(_) => 0,
            AgdaProofTerm::Sym(p) => 1 + p.depth(),
            AgdaProofTerm::Cong(_, p) => 1 + p.depth(),
            AgdaProofTerm::Lambda(_, body) => 1 + body.depth(),
            AgdaProofTerm::Trans(a, b) | AgdaProofTerm::App(a, b) => 1 + a.depth().max(b.depth()),
        }
    }
}
/// Implicit/explicit/instance argument wrapper.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArgKind {
    /// `(x : T)` — explicit
    Explicit,
    /// `{x : T}` — implicit
    Implicit,
    /// `⦃x : T⦄` — instance
    Instance,
}
/// An Agda function definition with clauses.
#[derive(Debug, Clone)]
pub struct AgdaFunctionDef {
    pub name: String,
    pub signature: String,
    pub clauses: Vec<AgdaClause>,
    pub pragmas: Vec<AgdaPragma>,
}
impl AgdaFunctionDef {
    pub fn new(name: &str, signature: &str) -> Self {
        AgdaFunctionDef {
            name: name.to_string(),
            signature: signature.to_string(),
            clauses: Vec::new(),
            pragmas: Vec::new(),
        }
    }
    pub fn add_clause(&mut self, clause: AgdaClause) {
        self.clauses.push(clause);
    }
    pub fn add_pragma(&mut self, pragma: AgdaPragma) {
        self.pragmas.push(pragma);
    }
    /// Render the full function definition.
    pub fn render(&self) -> String {
        let mut out = String::new();
        for pragma in &self.pragmas {
            out.push_str(&format!("{}\n", pragma.render()));
        }
        out.push_str(&format!("{} : {}\n", self.name, self.signature));
        if self.clauses.is_empty() {
            out.push_str(&format!("{} = {{}}\n", self.name));
        } else {
            for clause in &self.clauses {
                out.push_str(&format!("{}\n", clause.render(&self.name)));
            }
        }
        out
    }
}
/// Manages export to multiple Agda files (one per module).
pub struct MultiFileExport {
    files: Vec<ExportFile>,
    base_module: String,
}
impl MultiFileExport {
    /// Create a new multi-file export with the given base module name.
    pub fn new(base_module: &str) -> Self {
        MultiFileExport {
            files: Vec::new(),
            base_module: base_module.to_string(),
        }
    }
    /// Add an exporter as a submodule file.
    pub fn add_module(&mut self, submodule: &str, exporter: &AgdaExporter) {
        self.files.push(ExportFile {
            path: format!("{}/{}.agda", self.base_module.replace('.', "/"), submodule),
            content: exporter.export(),
        });
    }
    /// Add a file with raw content.
    pub fn add_raw(&mut self, path: &str, content: &str) {
        self.files.push(ExportFile {
            path: path.to_string(),
            content: content.to_string(),
        });
    }
    /// Write all files to disk under `base_dir`.
    pub fn write_all(&self, base_dir: &Path) -> std::io::Result<()> {
        for file in &self.files {
            let full_path = base_dir.join(&file.path);
            if let Some(parent) = full_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(&full_path, &file.content)?;
        }
        Ok(())
    }
    /// Return the number of files.
    pub fn file_count(&self) -> usize {
        self.files.len()
    }
    /// Return all file paths.
    pub fn paths(&self) -> Vec<&str> {
        self.files.iter().map(|f| f.path.as_str()).collect()
    }
}
/// Kind of an OxiLean declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OxiDeclKind {
    Def,
    Theorem,
    Axiom,
    Inductive,
    Structure,
}
/// Statistics for an Agda export run.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct AgdaExportStats {
    pub theorems_exported: usize,
    pub definitions_exported: usize,
    pub records_exported: usize,
    pub inductives_exported: usize,
    pub total_lines: usize,
}
impl AgdaExportStats {
    /// Return a summary string.
    #[allow(dead_code)]
    pub fn summary(&self) -> String {
        format!(
            "Agda Export: {} theorems, {} defs, {} records, {} inductives ({} lines)",
            self.theorems_exported,
            self.definitions_exported,
            self.records_exported,
            self.inductives_exported,
            self.total_lines,
        )
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AgdaImportDecl {
    pub module_path: Vec<String>,
    pub import_all: bool,
    pub hiding: Vec<String>,
    pub renaming: Vec<(String, String)>,
}
#[allow(dead_code)]
impl AgdaImportDecl {
    pub fn new(module_path: Vec<String>) -> Self {
        Self {
            module_path,
            import_all: true,
            hiding: Vec::new(),
            renaming: Vec::new(),
        }
    }
    pub fn with_hiding(mut self, names: Vec<String>) -> Self {
        self.hiding = names;
        self
    }
    pub fn with_renaming(mut self, pairs: Vec<(String, String)>) -> Self {
        self.renaming = pairs;
        self
    }
    pub fn render(&self) -> String {
        let path = self.module_path.join(".");
        let mut s = format!("import {}", path);
        if !self.hiding.is_empty() {
            let h = self.hiding.join("; ");
            s.push_str(&format!(" hiding ({})", h));
        }
        if !self.renaming.is_empty() {
            let r: Vec<String> = self
                .renaming
                .iter()
                .map(|(a, b)| format!("{} to {}", a, b))
                .collect();
            s.push_str(&format!(" renaming ({})", r.join("; ")));
        }
        s
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AgdaExportConfigSimple {
    pub module_prefix: String,
    pub use_unicode: bool,
    pub emit_pragmas: bool,
    pub indent_size: usize,
}
/// A field of a record type.
#[derive(Debug, Clone)]
pub struct RecordField {
    pub name: String,
    pub ty: String,
    pub is_implicit: bool,
}
impl RecordField {
    pub fn new(name: &str, ty: &str) -> Self {
        RecordField {
            name: name.to_string(),
            ty: ty.to_string(),
            is_implicit: false,
        }
    }
    pub fn implicit(mut self) -> Self {
        self.is_implicit = true;
        self
    }
}
/// Agda compiler/tool pragmas.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgdaPragma {
    /// `{-# BUILTIN NAME value #-}`
    Builtin { name: String, value: String },
    /// `{-# TERMINATING #-}`
    Terminating,
    /// `{-# NON_TERMINATING #-}`
    NonTerminating,
    /// `{-# INLINE name #-}`
    Inline { name: String },
    /// `{-# COMPILE GHC ... #-}`
    CompileGhc { decl: String },
    /// `{-# OPTIONS --allow-unsolved-metas #-}`
    Options { args: Vec<String> },
    /// `{-# REWRITE rule #-}`
    Rewrite { rule: String },
    /// Custom pragma text.
    Custom { text: String },
}
impl AgdaPragma {
    pub fn render(&self) -> String {
        match self {
            AgdaPragma::Builtin { name, value } => {
                format!("{{-# BUILTIN {} {} #-}}", name, value)
            }
            AgdaPragma::Terminating => "{-# TERMINATING #-}".to_string(),
            AgdaPragma::NonTerminating => "{-# NON_TERMINATING #-}".to_string(),
            AgdaPragma::Inline { name } => format!("{{-# INLINE {} #-}}", name),
            AgdaPragma::CompileGhc { decl } => format!("{{-# COMPILE GHC {} #-}}", decl),
            AgdaPragma::Options { args } => {
                format!("{{-# OPTIONS {} #-}}", args.join(" "))
            }
            AgdaPragma::Rewrite { rule } => format!("{{-# REWRITE {} #-}}", rule),
            AgdaPragma::Custom { text } => format!("{{-# {} #-}}", text),
        }
    }
}
/// Generates Coq source text.
pub struct CoqExporter {
    config: CoqExportConfig,
    items: Vec<CoqItem>,
}
impl CoqExporter {
    /// Create a new exporter with the given configuration.
    pub fn new(config: CoqExportConfig) -> Self {
        CoqExporter {
            config,
            items: Vec::new(),
        }
    }
    /// Append an axiom.
    pub fn add_axiom(&mut self, name: &str, ty: &str) {
        self.items.push(CoqItem::Axiom {
            name: name.to_string(),
            ty: ty.to_string(),
        });
    }
    /// Append a definition.
    pub fn add_definition(&mut self, name: &str, ty: &str, body: &str) {
        self.items.push(CoqItem::Definition {
            name: name.to_string(),
            ty: ty.to_string(),
            body: body.to_string(),
        });
    }
    /// Append a lemma.
    pub fn add_lemma(&mut self, name: &str, ty: &str, proof: &str) {
        self.items.push(CoqItem::Lemma {
            name: name.to_string(),
            ty: ty.to_string(),
            proof: proof.to_string(),
        });
    }
    /// Append an inductive type.
    pub fn add_inductive(&mut self, decl: InductiveDecl) {
        self.items.push(CoqItem::Inductive(decl));
    }
    /// Append a notation.
    pub fn add_notation(&mut self, symbol: &str, body: &str) {
        self.items.push(CoqItem::Notation {
            symbol: symbol.to_string(),
            body: body.to_string(),
        });
    }
    /// Append a Require/Import.
    pub fn add_require(&mut self, module: &str, import: bool) {
        self.items.push(CoqItem::Require {
            module: module.to_string(),
            import,
        });
    }
    /// Append a comment.
    pub fn add_comment(&mut self, text: &str) {
        self.items.push(CoqItem::Comment(text.to_string()));
    }
    /// Append a blank line.
    pub fn add_blank(&mut self) {
        self.items.push(CoqItem::Blank);
    }
    /// Generate the complete Coq source text.
    pub fn export(&self) -> String {
        let mut out = String::new();
        out.push_str("Require Import Coq.Init.Prelude.\n");
        for import in &self.config.imports {
            out.push_str(&format!("Require Import {}.\n", import));
        }
        out.push('\n');
        out.push_str(&format!("Section {}.\n\n", self.config.module_name));
        for item in &self.items {
            match item {
                CoqItem::Axiom { name, ty } => {
                    out.push_str(&format!("Axiom {} : {}.\n", name, ty));
                }
                CoqItem::Definition { name, ty, body } => {
                    out.push_str(&format!("Definition {} : {} := {}.\n", name, ty, body));
                }
                CoqItem::Lemma { name, ty, proof } => {
                    out.push_str(&format!(
                        "Lemma {} : {}.\nProof.\n  {}.\nQed.\n",
                        name, ty, proof
                    ));
                }
                CoqItem::Inductive(decl) => {
                    out.push_str(&decl.render_coq());
                }
                CoqItem::Notation { symbol, body } => {
                    out.push_str(&format!("Notation \"{}\" := {}.\n", symbol, body));
                }
                CoqItem::Require { module, import } => {
                    if *import {
                        out.push_str(&format!("Require Import {}.\n", module));
                    } else {
                        out.push_str(&format!("Require {}.\n", module));
                    }
                }
                CoqItem::Comment(text) => {
                    out.push_str(&format!("(* {} *)\n", text));
                }
                CoqItem::Blank => {
                    out.push('\n');
                }
            }
        }
        out.push_str(&format!("\nEnd {}.\n", self.config.module_name));
        out
    }
    /// Export to a file.
    pub fn export_to_file(&self, path: &Path) -> std::io::Result<()> {
        std::fs::write(path, self.export())
    }
}
/// A single argument (name + type + kind).
#[derive(Debug, Clone)]
pub struct ArgSpec {
    pub name: String,
    pub ty: String,
    pub kind: ArgKind,
}
impl ArgSpec {
    pub fn explicit(name: &str, ty: &str) -> Self {
        ArgSpec {
            name: name.to_string(),
            ty: ty.to_string(),
            kind: ArgKind::Explicit,
        }
    }
    pub fn implicit(name: &str, ty: &str) -> Self {
        ArgSpec {
            name: name.to_string(),
            ty: ty.to_string(),
            kind: ArgKind::Implicit,
        }
    }
    pub fn instance(name: &str, ty: &str) -> Self {
        ArgSpec {
            name: name.to_string(),
            ty: ty.to_string(),
            kind: ArgKind::Instance,
        }
    }
    /// Render this argument in Agda syntax.
    pub fn render_agda(&self) -> String {
        match self.kind {
            ArgKind::Explicit => format!("({} : {})", self.name, self.ty),
            ArgKind::Implicit => format!("{{{} : {}}}", self.name, self.ty),
            ArgKind::Instance => format!("⦃{} : {}⦄", self.name, self.ty),
        }
    }
    /// Render this argument in Coq syntax.
    pub fn render_coq(&self) -> String {
        match self.kind {
            ArgKind::Explicit => format!("({} : {})", self.name, self.ty),
            ArgKind::Implicit => format!("{{{} : {}}}", self.name, self.ty),
            ArgKind::Instance => format!("`{{{} : {}}}", self.name, self.ty),
        }
    }
}
/// Generates Agda source text from a list of [`AgdaDecl`]s.
pub struct AgdaExporter {
    config: AgdaExportConfig,
    decls: Vec<AgdaDecl>,
}
impl AgdaExporter {
    /// Create a new exporter with the given configuration.
    pub fn new(config: AgdaExportConfig) -> Self {
        AgdaExporter {
            config,
            decls: Vec::new(),
        }
    }
    /// Append a postulate (axiom) declaration.
    pub fn add_postulate(&mut self, name: &str, ty: &str) {
        match self.decls.last_mut() {
            Some(AgdaDecl::PostulateBlock { items }) => {
                items.push((name.to_string(), ty.to_string()));
            }
            _ => {
                self.decls.push(AgdaDecl::PostulateBlock {
                    items: vec![(name.to_string(), ty.to_string())],
                });
            }
        }
    }
    /// Append a definition with a type annotation and body.
    pub fn add_def(&mut self, name: &str, ty: &str, body: &str) {
        self.decls.push(AgdaDecl::Def {
            name: name.to_string(),
            ty: ty.to_string(),
            body: body.to_string(),
        });
    }
    /// Append a type signature only.
    pub fn add_type_def(&mut self, name: &str, ty: &str) {
        self.decls.push(AgdaDecl::TypeDef {
            name: name.to_string(),
            ty: ty.to_string(),
        });
    }
    /// Append a data type declaration.
    pub fn add_data(&mut self, decl: InductiveDecl) {
        self.decls.push(AgdaDecl::Data(decl));
    }
    /// Append a record declaration.
    pub fn add_record(&mut self, decl: RecordDecl) {
        self.decls.push(AgdaDecl::Record(decl));
    }
    /// Append a function definition.
    pub fn add_function(&mut self, def: AgdaFunctionDef) {
        self.decls.push(AgdaDecl::Function(def));
    }
    /// Append a pragma.
    pub fn add_pragma(&mut self, pragma: AgdaPragma) {
        self.decls.push(AgdaDecl::Pragma(pragma));
    }
    /// Append a blank line.
    pub fn add_blank(&mut self) {
        self.decls.push(AgdaDecl::Blank);
    }
    /// Append a comment.
    pub fn add_comment(&mut self, text: &str) {
        self.decls.push(AgdaDecl::Comment(text.to_string()));
    }
    /// Append an open import.
    pub fn add_open(&mut self, module: &str, using: Option<Vec<String>>) {
        self.decls.push(AgdaDecl::Open {
            module: module.to_string(),
            using,
        });
    }
    /// Append a fixity declaration.
    pub fn add_fixity(&mut self, fixity: &str, prec: u8, operator: &str) {
        self.decls.push(AgdaDecl::Fixity {
            fixity: fixity.to_string(),
            prec,
            operator: operator.to_string(),
        });
    }
    /// Append an instance declaration.
    pub fn add_instance(&mut self, name: &str, ty: &str, body: &str) {
        self.decls.push(AgdaDecl::Instance {
            name: name.to_string(),
            ty: ty.to_string(),
            body: body.to_string(),
        });
    }
    /// Generate the complete Agda source text.
    pub fn export(&self) -> String {
        let ind = " ".repeat(self.config.indent_size);
        let mut out = String::new();
        if !self.config.options.is_empty() {
            out.push_str(&format!(
                "{{-# OPTIONS {} #-}}\n",
                self.config.options.join(" ")
            ));
            out.push('\n');
        }
        out.push_str(&format!("module {} where\n", self.config.module_name));
        if !self.config.imports.is_empty() {
            out.push('\n');
            for import in &self.config.imports {
                out.push_str(&format!("open import {}\n", import));
            }
        }
        for decl in &self.decls {
            out.push('\n');
            out.push_str(&self.render_decl(decl, &ind, 0));
        }
        out
    }
    fn render_decl(&self, decl: &AgdaDecl, ind: &str, depth: usize) -> String {
        let prefix = ind.repeat(depth);
        match decl {
            AgdaDecl::Module { name, contents } => {
                let mut s = format!("{}module {} where\n", prefix, name);
                for inner in contents {
                    s.push_str(&self.render_decl(inner, ind, depth + 1));
                }
                s
            }
            AgdaDecl::TypeDef { name, ty } => format!("{}{} : {}\n", prefix, name, ty),
            AgdaDecl::Def { name, ty, body } => {
                format!(
                    "{}{} : {}\n{}{} = {}\n",
                    prefix, name, ty, prefix, name, body
                )
            }
            AgdaDecl::PostulateBlock { items } => {
                let mut s = format!("{}postulate\n", prefix);
                for (name, ty) in items {
                    s.push_str(&format!("{}  {} : {}\n", prefix, name, ty));
                }
                s
            }
            AgdaDecl::Data(decl) => {
                let inner = decl.render_agda(self.config.indent_size);
                inner
                    .lines()
                    .map(|l| format!("{}{}\n", prefix, l))
                    .collect()
            }
            AgdaDecl::Record(decl) => {
                let inner = decl.render_agda(self.config.indent_size);
                inner
                    .lines()
                    .map(|l| format!("{}{}\n", prefix, l))
                    .collect()
            }
            AgdaDecl::Function(def) => {
                let inner = def.render();
                inner
                    .lines()
                    .map(|l| format!("{}{}\n", prefix, l))
                    .collect()
            }
            AgdaDecl::Pragma(p) => format!("{}{}\n", prefix, p.render()),
            AgdaDecl::Blank => "\n".to_string(),
            AgdaDecl::Comment(text) => {
                let mut s = String::new();
                for line in text.lines() {
                    s.push_str(&format!("{}-- {}\n", prefix, line));
                }
                s
            }
            AgdaDecl::Open { module, using } => {
                if let Some(names) = using {
                    format!(
                        "{}open import {} using ({})\n",
                        prefix,
                        module,
                        names.join("; ")
                    )
                } else {
                    format!("{}open import {}\n", prefix, module)
                }
            }
            AgdaDecl::Fixity {
                fixity,
                prec,
                operator,
            } => {
                format!("{}{} {} {}\n", prefix, fixity, prec, operator)
            }
            AgdaDecl::Instance { name, ty, body } => {
                format!(
                    "{}instance\n{}  {} : {}\n{}  {} = {}\n",
                    prefix, prefix, name, ty, prefix, name, body
                )
            }
        }
    }
    /// Write the exported source to a file at `path`.
    pub fn export_to_file(&self, path: &Path) -> std::io::Result<()> {
        std::fs::write(path, self.export())
    }
}
/// A name cache for deduplication during batch export.
pub struct NameCache {
    seen: HashMap<String, usize>,
}
impl NameCache {
    pub fn new() -> Self {
        NameCache {
            seen: HashMap::new(),
        }
    }
    /// Return a unique Agda name for the given OxiLean name.
    pub fn unique_agda_name(&mut self, name: &str) -> String {
        let base = sanitize_agda_name(name);
        let count = self.seen.entry(base.clone()).or_insert(0);
        if *count == 0 {
            *count = 1;
            base
        } else {
            let result = format!("{}_{}", base, count);
            *count += 1;
            result
        }
    }
    /// Return a unique Coq name for the given OxiLean name.
    pub fn unique_coq_name(&mut self, name: &str) -> String {
        let base = sanitize_coq_name(name);
        let count = self.seen.entry(base.clone()).or_insert(0);
        if *count == 0 {
            *count = 1;
            base
        } else {
            let result = format!("{}_{}", base, count);
            *count += 1;
            result
        }
    }
    /// Number of names in cache.
    pub fn len(&self) -> usize {
        self.seen.len()
    }
    /// True if empty.
    pub fn is_empty(&self) -> bool {
        self.seen.is_empty()
    }
}
/// Configuration for generating Agda modules.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct AgdaModuleConfig {
    pub module_name: String,
    pub imports: Vec<String>,
    pub safe_mode: bool,
    pub eta_equality: bool,
    pub instance_arguments: bool,
    pub exact_split: bool,
    pub prop_name: String,
    pub set_name: String,
}
impl AgdaModuleConfig {
    /// Generate the pragma header for this configuration.
    #[allow(dead_code)]
    pub fn pragma_header(&self) -> String {
        let mut out = String::new();
        if self.safe_mode {
            out.push_str("{-# OPTIONS --safe #-}\n");
        }
        if self.eta_equality {
            out.push_str("{-# OPTIONS --eta-equality #-}\n");
        }
        if self.exact_split {
            out.push_str("{-# OPTIONS --exact-split #-}\n");
        }
        out.push_str(&format!("module {} where\n\n", self.module_name));
        for import in &self.imports {
            out.push_str(&format!("open import {}\n", import));
        }
        out.push('\n');
        out
    }
}
/// A field in an Agda record.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct AgdaRecordField {
    pub name: String,
    pub field_type: String,
}
/// A record type declaration.
#[derive(Debug, Clone)]
pub struct RecordDecl {
    pub name: String,
    pub params: Vec<ArgSpec>,
    pub sort: String,
    pub constructor_name: Option<String>,
    pub fields: Vec<RecordField>,
}
impl RecordDecl {
    pub fn new(name: &str, sort: &str) -> Self {
        RecordDecl {
            name: name.to_string(),
            params: Vec::new(),
            sort: sort.to_string(),
            constructor_name: None,
            fields: Vec::new(),
        }
    }
    pub fn with_constructor(mut self, ctor: &str) -> Self {
        self.constructor_name = Some(ctor.to_string());
        self
    }
    pub fn add_field(&mut self, field: RecordField) {
        self.fields.push(field);
    }
    /// Render to Agda `record` declaration.
    pub fn render_agda(&self, indent_size: usize) -> String {
        let ind = " ".repeat(indent_size);
        let params_str: String = self
            .params
            .iter()
            .map(|p| format!(" {}", p.render_agda()))
            .collect();
        let mut out = format!("record {}{} : {} where\n", self.name, params_str, self.sort);
        if let Some(ctor) = &self.constructor_name {
            out.push_str(&format!("{}constructor {}\n", ind, ctor));
        }
        out.push_str(&format!("{}field\n", ind));
        for field in &self.fields {
            let ann = if field.is_implicit {
                format!("{{{} : {}}}", field.name, field.ty)
            } else {
                format!("{} : {}", field.name, field.ty)
            };
            out.push_str(&format!("{}  {}\n", ind, ann));
        }
        out
    }
}
/// A constructor for an inductive type.
#[derive(Debug, Clone)]
pub struct Constructor {
    pub name: String,
    pub args: Vec<ArgSpec>,
    pub return_type: String,
}
impl Constructor {
    pub fn new(name: &str, return_type: &str) -> Self {
        Constructor {
            name: name.to_string(),
            args: Vec::new(),
            return_type: return_type.to_string(),
        }
    }
    pub fn with_arg(mut self, arg: ArgSpec) -> Self {
        self.args.push(arg);
        self
    }
    /// Render in Agda style: `ctor : A → B → T`
    pub fn render_agda(&self, indent: &str) -> String {
        if self.args.is_empty() {
            format!("{}{} : {}", indent, self.name, self.return_type)
        } else {
            let arg_types: Vec<String> = self.args.iter().map(|a| a.ty.clone()).collect();
            let sig = format!("{} → {}", arg_types.join(" → "), self.return_type);
            format!("{}{} : {}", indent, self.name, sig)
        }
    }
    /// Render in Coq style: `| ctor : A → B → T`
    pub fn render_coq(&self, indent: &str) -> String {
        if self.args.is_empty() {
            format!("{}| {} : {}", indent, self.name, self.return_type)
        } else {
            let arg_types: Vec<String> = self.args.iter().map(|a| a.ty.clone()).collect();
            let sig = format!("{} -> {}", arg_types.join(" -> "), self.return_type);
            format!("{}| {} : {}", indent, self.name, sig)
        }
    }
}
/// Configuration for Agda code generation.
pub struct AgdaExportConfig {
    pub module_name: String,
    pub use_unicode: bool,
    pub indent_size: usize,
    /// Imports to add at the top of the module.
    pub imports: Vec<String>,
    /// Preamble options (e.g. `--allow-unsolved-metas`).
    pub options: Vec<String>,
}
impl AgdaExportConfig {
    /// A minimal config with just a module name.
    pub fn minimal(module_name: &str) -> Self {
        AgdaExportConfig {
            module_name: module_name.to_string(),
            use_unicode: true,
            indent_size: 2,
            imports: Vec::new(),
            options: Vec::new(),
        }
    }
    /// Add an import.
    pub fn with_import(mut self, import: &str) -> Self {
        self.imports.push(import.to_string());
        self
    }
    /// Add an option.
    pub fn with_option(mut self, opt: &str) -> Self {
        self.options.push(opt.to_string());
        self
    }
}
/// An output file for multi-file export.
pub struct ExportFile {
    pub path: String,
    pub content: String,
}
/// Configuration for Coq code generation.
pub struct CoqExportConfig {
    pub module_name: String,
    pub imports: Vec<String>,
}
impl CoqExportConfig {
    pub fn new(module_name: &str) -> Self {
        CoqExportConfig {
            module_name: module_name.to_string(),
            imports: Vec::new(),
        }
    }
    pub fn with_import(mut self, import: &str) -> Self {
        self.imports.push(import.to_string());
        self
    }
}
/// A high-level OxiLean declaration (simplified for batch processing).
#[derive(Debug, Clone)]
pub struct OxiDecl {
    pub name: String,
    pub kind: OxiDeclKind,
    pub type_sig: String,
    pub body: Option<String>,
}
impl OxiDecl {
    pub fn def(name: &str, ty: &str, body: &str) -> Self {
        OxiDecl {
            name: name.to_string(),
            kind: OxiDeclKind::Def,
            type_sig: ty.to_string(),
            body: Some(body.to_string()),
        }
    }
    pub fn axiom(name: &str, ty: &str) -> Self {
        OxiDecl {
            name: name.to_string(),
            kind: OxiDeclKind::Axiom,
            type_sig: ty.to_string(),
            body: None,
        }
    }
    pub fn theorem(name: &str, ty: &str, proof: &str) -> Self {
        OxiDecl {
            name: name.to_string(),
            kind: OxiDeclKind::Theorem,
            type_sig: ty.to_string(),
            body: Some(proof.to_string()),
        }
    }
}
/// Statistics from a batch conversion.
#[derive(Debug, Default, Clone)]
pub struct ConversionStats {
    pub total: usize,
    pub success: usize,
    pub skipped: usize,
    pub errors: usize,
    pub error_messages: Vec<String>,
}
impl ConversionStats {
    pub fn record_success(&mut self) {
        self.total += 1;
        self.success += 1;
    }
    pub fn record_skip(&mut self) {
        self.total += 1;
        self.skipped += 1;
    }
    pub fn record_error(&mut self, msg: &str) {
        self.total += 1;
        self.errors += 1;
        self.error_messages.push(msg.to_string());
    }
    pub fn success_rate(&self) -> f64 {
        if self.total == 0 {
            1.0
        } else {
            self.success as f64 / self.total as f64
        }
    }
    pub fn summary(&self) -> String {
        format!(
            "total={} success={} skipped={} errors={} rate={:.1}%",
            self.total,
            self.success,
            self.skipped,
            self.errors,
            self.success_rate() * 100.0
        )
    }
}
enum CoqItem {
    Axiom {
        name: String,
        ty: String,
    },
    Definition {
        name: String,
        ty: String,
        body: String,
    },
    Lemma {
        name: String,
        ty: String,
        proof: String,
    },
    Inductive(InductiveDecl),
    Notation {
        symbol: String,
        body: String,
    },
    Require {
        module: String,
        import: bool,
    },
    Comment(String),
    Blank,
}
/// A single Agda top-level declaration.
pub enum AgdaDecl {
    /// A nested module with its own contents.
    Module {
        name: String,
        contents: Vec<AgdaDecl>,
    },
    /// A bare type signature.
    TypeDef { name: String, ty: String },
    /// A definition with a type annotation and a body.
    Def {
        name: String,
        ty: String,
        body: String,
    },
    /// A `postulate` block containing multiple (name, type) pairs.
    PostulateBlock { items: Vec<(String, String)> },
    /// A data type declaration.
    Data(InductiveDecl),
    /// A record type declaration.
    Record(RecordDecl),
    /// A function with pattern clauses.
    Function(AgdaFunctionDef),
    /// A pragma line.
    Pragma(AgdaPragma),
    /// A blank line separator.
    Blank,
    /// A comment line.
    Comment(String),
    /// An open import.
    Open {
        module: String,
        using: Option<Vec<String>>,
    },
    /// A fixity declaration.
    Fixity {
        fixity: String,
        prec: u8,
        operator: String,
    },
    /// An instance declaration.
    Instance {
        name: String,
        ty: String,
        body: String,
    },
}
/// An Agda record definition.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct AgdaRecord {
    pub name: String,
    pub params: Vec<(String, String)>,
    pub fields: Vec<AgdaRecordField>,
    pub constructor_name: Option<String>,
}
impl AgdaRecord {
    /// Create a new record.
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            params: vec![],
            fields: vec![],
            constructor_name: None,
        }
    }
    /// Add a parameter.
    #[allow(dead_code)]
    pub fn with_param(mut self, name: impl Into<String>, ty: impl Into<String>) -> Self {
        self.params.push((name.into(), ty.into()));
        self
    }
    /// Add a field.
    #[allow(dead_code)]
    pub fn with_field(mut self, name: impl Into<String>, ty: impl Into<String>) -> Self {
        self.fields.push(AgdaRecordField {
            name: name.into(),
            field_type: ty.into(),
        });
        self
    }
    /// Set the constructor name.
    #[allow(dead_code)]
    pub fn with_constructor(mut self, ctor: impl Into<String>) -> Self {
        self.constructor_name = Some(ctor.into());
        self
    }
    /// Render to Agda source.
    #[allow(dead_code)]
    pub fn render(&self) -> String {
        let params: String = self
            .params
            .iter()
            .map(|(n, t)| format!("({} : {}) ", n, t))
            .collect();
        let mut out = format!("record {}{}: Set where\n", self.name, params);
        if let Some(ref ctor) = self.constructor_name {
            out.push_str(&format!("  constructor {}\n", ctor));
        }
        out.push_str("  field\n");
        for field in &self.fields {
            out.push_str(&format!("    {} : {}\n", field.name, field.field_type));
        }
        out
    }
}
/// An inductive (data) type declaration.
#[derive(Debug, Clone)]
pub struct InductiveDecl {
    pub name: String,
    pub params: Vec<ArgSpec>,
    pub sort: String,
    pub constructors: Vec<Constructor>,
}
impl InductiveDecl {
    pub fn new(name: &str, sort: &str) -> Self {
        InductiveDecl {
            name: name.to_string(),
            params: Vec::new(),
            sort: sort.to_string(),
            constructors: Vec::new(),
        }
    }
    pub fn add_param(&mut self, param: ArgSpec) {
        self.params.push(param);
    }
    pub fn add_constructor(&mut self, ctor: Constructor) {
        self.constructors.push(ctor);
    }
    /// Render to Agda `data` declaration.
    pub fn render_agda(&self, indent_size: usize) -> String {
        let ind = " ".repeat(indent_size);
        let params_str: String = self
            .params
            .iter()
            .map(|p| format!(" {}", p.render_agda()))
            .collect();
        let mut out = format!("data {}{} : {} where\n", self.name, params_str, self.sort);
        for ctor in &self.constructors {
            out.push_str(&format!("{}\n", ctor.render_agda(&ind)));
        }
        out
    }
    /// Render to Coq `Inductive` declaration.
    pub fn render_coq(&self) -> String {
        let params_str: String = self
            .params
            .iter()
            .map(|p| format!(" {}", p.render_coq()))
            .collect();
        let mut out = format!("Inductive {}{} : {} :=\n", self.name, params_str, self.sort);
        for ctor in &self.constructors {
            out.push_str(&format!("{}\n", ctor.render_coq("  ")));
        }
        out.push_str(".\n");
        out
    }
}
