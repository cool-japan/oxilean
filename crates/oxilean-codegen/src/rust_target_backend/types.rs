//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use std::collections::HashMap;

use super::functions::RUST_KEYWORDS;

use super::functions::*;
use std::collections::{HashSet, VecDeque};

/// An enum variant.
#[derive(Debug, Clone, PartialEq)]
pub enum RustVariant {
    /// `Foo` — unit variant
    Unit(std::string::String),
    /// `Foo(T1, T2)` — tuple variant
    Tuple(std::string::String, Vec<RustType>),
    /// `Foo { x: T1, y: T2 }` — struct variant
    Struct(std::string::String, Vec<(std::string::String, RustType)>),
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RustLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
impl RustLivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        RustLivenessInfo {
            live_in: vec![std::collections::HashSet::new(); block_count],
            live_out: vec![std::collections::HashSet::new(); block_count],
            defs: vec![std::collections::HashSet::new(); block_count],
            uses: vec![std::collections::HashSet::new(); block_count],
        }
    }
    #[allow(dead_code)]
    pub fn add_def(&mut self, block: usize, var: u32) {
        if block < self.defs.len() {
            self.defs[block].insert(var);
        }
    }
    #[allow(dead_code)]
    pub fn add_use(&mut self, block: usize, var: u32) {
        if block < self.uses.len() {
            self.uses[block].insert(var);
        }
    }
    #[allow(dead_code)]
    pub fn is_live_in(&self, block: usize, var: u32) -> bool {
        self.live_in
            .get(block)
            .map(|s| s.contains(&var))
            .unwrap_or(false)
    }
    #[allow(dead_code)]
    pub fn is_live_out(&self, block: usize, var: u32) -> bool {
        self.live_out
            .get(block)
            .map(|s| s.contains(&var))
            .unwrap_or(false)
    }
}
/// A Rust function item.
#[derive(Debug, Clone, PartialEq)]
pub struct RustFn {
    pub name: std::string::String,
    pub generics: Vec<(std::string::String, Vec<std::string::String>)>,
    pub params: Vec<(std::string::String, RustType, bool)>,
    pub return_type: Option<RustType>,
    pub body: Vec<RustStmt>,
    pub attrs: Vec<std::string::String>,
    pub visibility: RustVisibility,
    pub is_async: bool,
    pub is_unsafe: bool,
}
impl RustFn {
    /// Create a simple public function.
    pub fn new(
        name: impl Into<std::string::String>,
        params: Vec<(std::string::String, RustType, bool)>,
        ret: Option<RustType>,
        body: Vec<RustStmt>,
    ) -> Self {
        RustFn {
            name: name.into(),
            generics: Vec::new(),
            params,
            return_type: ret,
            body,
            attrs: Vec::new(),
            visibility: RustVisibility::Pub,
            is_async: false,
            is_unsafe: false,
        }
    }
    /// Emit Rust source for this function.
    pub fn emit(&self) -> std::string::String {
        let mut out = std::string::String::new();
        for attr in &self.attrs {
            out.push_str(&format!("#[{}]\n", attr));
        }
        let async_kw = if self.is_async { "async " } else { "" };
        let unsafe_kw = if self.is_unsafe { "unsafe " } else { "" };
        let generics_str = if self.generics.is_empty() {
            std::string::String::new()
        } else {
            let gs: Vec<_> = self
                .generics
                .iter()
                .map(|(name, bounds)| {
                    if bounds.is_empty() {
                        name.clone()
                    } else {
                        format!("{}: {}", name, bounds.join(" + "))
                    }
                })
                .collect();
            format!("<{}>", gs.join(", "))
        };
        let params_str = self
            .params
            .iter()
            .map(|(n, ty, is_mut)| {
                let mut_kw = if *is_mut { "mut " } else { "" };
                if n == "self" {
                    if *is_mut {
                        "&mut self".to_string()
                    } else {
                        "&self".to_string()
                    }
                } else {
                    format!("{}{}: {}", mut_kw, n, ty)
                }
            })
            .collect::<Vec<_>>()
            .join(", ");
        let ret_str = self
            .return_type
            .as_ref()
            .map(|t| format!(" -> {}", t))
            .unwrap_or_default();
        out.push_str(&format!(
            "{}{}{}fn {}{}({}){}",
            self.visibility, async_kw, unsafe_kw, self.name, generics_str, params_str, ret_str
        ));
        if self.body.is_empty() {
            out.push_str(" {}");
        } else {
            out.push_str(" {\n");
            for stmt in &self.body {
                out.push_str(&format!("    {}\n", emit_stmt(stmt, 1)));
            }
            out.push('}');
        }
        out
    }
}
/// A Rust struct item.
#[derive(Debug, Clone, PartialEq)]
pub enum RustStructFields {
    /// Named fields: `{ x: i32, y: f64 }`
    Named(Vec<(std::string::String, RustType, RustVisibility)>),
    /// Tuple fields: `(i32, f64)`
    Tuple(Vec<(RustType, RustVisibility)>),
    /// Unit struct: no fields
    Unit,
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct RustPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
impl RustPassStats {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn record_run(&mut self, changes: u64, time_ms: u64, iterations: u32) {
        self.total_runs += 1;
        self.successful_runs += 1;
        self.total_changes += changes;
        self.time_ms += time_ms;
        self.iterations_used = iterations;
    }
    #[allow(dead_code)]
    pub fn average_changes_per_run(&self) -> f64 {
        if self.total_runs == 0 {
            return 0.0;
        }
        self.total_changes as f64 / self.total_runs as f64
    }
    #[allow(dead_code)]
    pub fn success_rate(&self) -> f64 {
        if self.total_runs == 0 {
            return 0.0;
        }
        self.successful_runs as f64 / self.total_runs as f64
    }
    #[allow(dead_code)]
    pub fn format_summary(&self) -> String {
        format!(
            "Runs: {}/{}, Changes: {}, Time: {}ms",
            self.successful_runs, self.total_runs, self.total_changes, self.time_ms
        )
    }
}
/// A top-level Rust item.
#[derive(Debug, Clone, PartialEq)]
pub enum RustItem {
    Fn(RustFn),
    Struct(RustStruct),
    Enum(RustEnum),
    Impl(RustImpl),
    TypeAlias {
        name: std::string::String,
        generics: Vec<std::string::String>,
        ty: RustType,
        visibility: RustVisibility,
    },
    Const {
        name: std::string::String,
        ty: RustType,
        value: RustExpr,
        visibility: RustVisibility,
    },
    Static {
        name: std::string::String,
        ty: RustType,
        value: RustExpr,
        mutable: bool,
        visibility: RustVisibility,
    },
    Mod {
        name: std::string::String,
        items: Vec<RustItem>,
        visibility: RustVisibility,
    },
    Use {
        path: std::string::String,
        visibility: RustVisibility,
    },
    ExternCrate {
        name: std::string::String,
        alias: Option<std::string::String>,
    },
    Trait {
        name: std::string::String,
        generics: Vec<(std::string::String, Vec<std::string::String>)>,
        bounds: Vec<std::string::String>,
        items: Vec<RustFn>,
        visibility: RustVisibility,
    },
}
impl RustItem {
    /// Emit Rust source for this item.
    pub fn emit(&self) -> std::string::String {
        match self {
            RustItem::Fn(f) => f.emit(),
            RustItem::Struct(s) => s.emit(),
            RustItem::Enum(e) => e.emit(),
            RustItem::Impl(i) => i.emit(),
            RustItem::TypeAlias {
                name,
                generics,
                ty,
                visibility,
            } => {
                let g = if generics.is_empty() {
                    std::string::String::new()
                } else {
                    format!("<{}>", generics.join(", "))
                };
                format!("{}type {}{} = {};", visibility, name, g, ty)
            }
            RustItem::Const {
                name,
                ty,
                value,
                visibility,
            } => {
                format!(
                    "{}const {}: {} = {};",
                    visibility,
                    name,
                    ty,
                    emit_expr(value, 0)
                )
            }
            RustItem::Static {
                name,
                ty,
                value,
                mutable,
                visibility,
            } => {
                let mut_kw = if *mutable { "mut " } else { "" };
                format!(
                    "{}static {}{}: {} = {};",
                    visibility,
                    mut_kw,
                    name,
                    ty,
                    emit_expr(value, 0)
                )
            }
            RustItem::Mod {
                name,
                items,
                visibility,
            } => {
                let mut out = format!("{}mod {} {{\n", visibility, name);
                for item in items {
                    for line in item.emit().lines() {
                        out.push_str(&format!("    {}\n", line));
                    }
                }
                out.push('}');
                out
            }
            RustItem::Use { path, visibility } => format!("{}use {};", visibility, path),
            RustItem::ExternCrate { name, alias } => match alias {
                Some(a) => format!("extern crate {} as {};", name, a),
                None => format!("extern crate {};", name),
            },
            RustItem::Trait {
                name,
                generics,
                bounds,
                items,
                visibility,
            } => {
                let gs: Vec<_> = generics
                    .iter()
                    .map(|(n, bs)| {
                        if bs.is_empty() {
                            n.clone()
                        } else {
                            format!("{}: {}", n, bs.join(" + "))
                        }
                    })
                    .collect();
                let g_str = if gs.is_empty() {
                    std::string::String::new()
                } else {
                    format!("<{}>", gs.join(", "))
                };
                let bounds_str = if bounds.is_empty() {
                    std::string::String::new()
                } else {
                    format!(": {}", bounds.join(" + "))
                };
                let mut out = format!("{}trait {}{}{} {{\n", visibility, name, g_str, bounds_str);
                for item in items {
                    for line in item.emit().lines() {
                        out.push_str(&format!("    {}\n", line));
                    }
                }
                out.push('}');
                out
            }
        }
    }
}
/// Rust literal values.
#[derive(Debug, Clone, PartialEq)]
pub enum RustLit {
    Int(i64),
    UInt(u64),
    Float(f64),
    Bool(bool),
    Char(char),
    Str(std::string::String),
    Unit,
}
/// Rust statement.
#[derive(Debug, Clone, PartialEq)]
pub enum RustStmt {
    /// `let pat: ty = expr;`
    Let {
        pat: RustPattern,
        ty: Option<RustType>,
        value: Option<RustExpr>,
    },
    /// Expression statement: `expr;`
    Expr(RustExpr),
    /// Expression without semicolon (last expr in block)
    ExprNoSemi(RustExpr),
    /// `return expr;`
    Return(Option<RustExpr>),
    /// `break [expr];`
    Break(Option<RustExpr>),
    /// `continue;`
    Continue,
}
/// A complete Rust module (source file or inline module).
#[derive(Debug, Clone, PartialEq)]
pub struct RustModule {
    pub name: std::string::String,
    pub items: Vec<RustItem>,
}
impl RustModule {
    /// Create a new empty Rust module.
    pub fn new(name: impl Into<std::string::String>) -> Self {
        RustModule {
            name: name.into(),
            items: Vec::new(),
        }
    }
    /// Emit valid Rust source code for this module.
    pub fn emit(&self) -> std::string::String {
        let mut out = std::string::String::new();
        for item in &self.items {
            out.push_str(&item.emit());
            out.push_str("\n\n");
        }
        out
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct RustStruct {
    pub name: std::string::String,
    pub generics: Vec<std::string::String>,
    pub fields: RustStructFields,
    pub attrs: Vec<std::string::String>,
    pub derives: Vec<std::string::String>,
    pub visibility: RustVisibility,
}
impl RustStruct {
    /// Emit Rust source for this struct.
    pub fn emit(&self) -> std::string::String {
        let mut out = std::string::String::new();
        if !self.derives.is_empty() {
            out.push_str(&format!("#[derive({})]\n", self.derives.join(", ")));
        }
        for attr in &self.attrs {
            out.push_str(&format!("#[{}]\n", attr));
        }
        let generics_str = if self.generics.is_empty() {
            std::string::String::new()
        } else {
            format!("<{}>", self.generics.join(", "))
        };
        match &self.fields {
            RustStructFields::Named(fields) => {
                out.push_str(&format!(
                    "{}struct {}{} {{\n",
                    self.visibility, self.name, generics_str
                ));
                for (name, ty, vis) in fields {
                    out.push_str(&format!("    {}{}:{},\n", vis, name, ty));
                }
                out.push('}');
            }
            RustStructFields::Tuple(fields) => {
                let fs: Vec<_> = fields
                    .iter()
                    .map(|(ty, vis)| format!("{}{}", vis, ty))
                    .collect();
                out.push_str(&format!(
                    "{}struct {}{}({});",
                    self.visibility,
                    self.name,
                    generics_str,
                    fs.join(", ")
                ));
            }
            RustStructFields::Unit => {
                out.push_str(&format!(
                    "{}struct {}{};",
                    self.visibility, self.name, generics_str
                ));
            }
        }
        out
    }
}
/// A Rust enum item.
#[derive(Debug, Clone, PartialEq)]
pub struct RustEnum {
    pub name: std::string::String,
    pub generics: Vec<std::string::String>,
    pub variants: Vec<RustVariant>,
    pub attrs: Vec<std::string::String>,
    pub derives: Vec<std::string::String>,
    pub visibility: RustVisibility,
}
impl RustEnum {
    /// Emit Rust source for this enum.
    pub fn emit(&self) -> std::string::String {
        let mut out = std::string::String::new();
        if !self.derives.is_empty() {
            out.push_str(&format!("#[derive({})]\n", self.derives.join(", ")));
        }
        for attr in &self.attrs {
            out.push_str(&format!("#[{}]\n", attr));
        }
        let generics_str = if self.generics.is_empty() {
            std::string::String::new()
        } else {
            format!("<{}>", self.generics.join(", "))
        };
        out.push_str(&format!(
            "{}enum {}{} {{\n",
            self.visibility, self.name, generics_str
        ));
        for variant in &self.variants {
            match variant {
                RustVariant::Unit(name) => {
                    out.push_str(&format!("    {},\n", name));
                }
                RustVariant::Tuple(name, fields) => {
                    let fs: Vec<_> = fields.iter().map(|t| t.to_string()).collect();
                    out.push_str(&format!("    {}({}),\n", name, fs.join(", ")));
                }
                RustVariant::Struct(name, fields) => {
                    out.push_str(&format!("    {} {{\n", name));
                    for (fn_, ft) in fields {
                        out.push_str(&format!("        {}: {},\n", fn_, ft));
                    }
                    out.push_str("    },\n");
                }
            }
        }
        out.push('}');
        out
    }
}
/// A Rust `impl` block.
#[derive(Debug, Clone, PartialEq)]
pub struct RustImpl {
    pub for_type: std::string::String,
    pub trait_impl: Option<std::string::String>,
    pub generics: Vec<(std::string::String, Vec<std::string::String>)>,
    pub methods: Vec<RustFn>,
    pub associated_types: Vec<(std::string::String, RustType)>,
}
impl RustImpl {
    /// Emit Rust source for this impl block.
    pub fn emit(&self) -> std::string::String {
        let mut out = std::string::String::new();
        let generics_str = if self.generics.is_empty() {
            std::string::String::new()
        } else {
            let gs: Vec<_> = self
                .generics
                .iter()
                .map(|(n, bounds)| {
                    if bounds.is_empty() {
                        n.clone()
                    } else {
                        format!("{}: {}", n, bounds.join(" + "))
                    }
                })
                .collect();
            format!("<{}>", gs.join(", "))
        };
        match &self.trait_impl {
            Some(tr) => out.push_str(&format!(
                "impl{} {} for {} {{\n",
                generics_str, tr, self.for_type
            )),
            None => out.push_str(&format!("impl{} {} {{\n", generics_str, self.for_type)),
        }
        for (name, ty) in &self.associated_types {
            out.push_str(&format!("    type {} = {};\n", name, ty));
        }
        for method in &self.methods {
            for line in method.emit().lines() {
                out.push_str(&format!("    {}\n", line));
            }
        }
        out.push('}');
        out
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum RustPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl RustPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            RustPassPhase::Analysis => "analysis",
            RustPassPhase::Transformation => "transformation",
            RustPassPhase::Verification => "verification",
            RustPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(self, RustPassPhase::Transformation | RustPassPhase::Cleanup)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RustAnalysisCache {
    pub(super) entries: std::collections::HashMap<String, RustCacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
impl RustAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        RustAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&RustCacheEntry> {
        if self.entries.contains_key(key) {
            self.hits += 1;
            self.entries.get(key)
        } else {
            self.misses += 1;
            None
        }
    }
    #[allow(dead_code)]
    pub fn insert(&mut self, key: String, data: Vec<u8>) {
        if self.entries.len() >= self.max_size {
            if let Some(oldest) = self.entries.keys().next().cloned() {
                self.entries.remove(&oldest);
            }
        }
        self.entries.insert(
            key.clone(),
            RustCacheEntry {
                key,
                data,
                timestamp: 0,
                valid: true,
            },
        );
    }
    #[allow(dead_code)]
    pub fn invalidate(&mut self, key: &str) {
        if let Some(entry) = self.entries.get_mut(key) {
            entry.valid = false;
        }
    }
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    #[allow(dead_code)]
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            return 0.0;
        }
        self.hits as f64 / total as f64
    }
    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        self.entries.len()
    }
}
/// Rust type representation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RustType {
    I8,
    I16,
    I32,
    I64,
    I128,
    Isize,
    U8,
    U16,
    U32,
    U64,
    U128,
    Usize,
    F32,
    F64,
    Bool,
    Char,
    Str,
    Unit,
    Never,
    /// `String` (owned)
    RustString,
    /// `Vec<T>`
    Vec(Box<RustType>),
    /// `Option<T>`
    Option(Box<RustType>),
    /// `Result<T, E>`
    Result(Box<RustType>, Box<RustType>),
    /// Tuple: `(A, B, ...)`
    Tuple(Vec<RustType>),
    /// `[T]` (unsized slice)
    Slice(Box<RustType>),
    /// `&T` or `&mut T`
    Ref(bool, Box<RustType>),
    /// Named/custom type
    Custom(std::string::String),
    /// Generic instantiation: `MyType<A, B>`
    Generic(std::string::String, Vec<RustType>),
    /// Lifetime annotation placeholder: `'a`
    Lifetime(std::string::String),
    /// `impl Fn(A, B) -> R` style type
    Fn(Vec<RustType>, Box<RustType>),
}
#[allow(dead_code)]
pub struct RustPassRegistry {
    pub(super) configs: Vec<RustPassConfig>,
    pub(super) stats: std::collections::HashMap<String, RustPassStats>,
}
impl RustPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        RustPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: RustPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), RustPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&RustPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&RustPassStats> {
        self.stats.get(name)
    }
    #[allow(dead_code)]
    pub fn total_passes(&self) -> usize {
        self.configs.len()
    }
    #[allow(dead_code)]
    pub fn enabled_count(&self) -> usize {
        self.enabled_passes().len()
    }
    #[allow(dead_code)]
    pub fn update_stats(&mut self, name: &str, changes: u64, time_ms: u64, iter: u32) {
        if let Some(stats) = self.stats.get_mut(name) {
            stats.record_run(changes, time_ms, iter);
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RustCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
/// Rust visibility modifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RustVisibility {
    Private,
    Pub,
    PubCrate,
    PubSuper,
}
/// Rust pattern for match arms.
#[derive(Debug, Clone, PartialEq)]
pub enum RustPattern {
    /// `_`
    Wildcard,
    /// `x` or `mut x`
    Var(std::string::String, bool),
    /// Literal pattern: `42`, `true`, `"hello"`
    Lit(RustLit),
    /// Tuple pattern: `(a, b, c)`
    Tuple(Vec<RustPattern>),
    /// Struct pattern: `Foo { x, y }`
    Struct(std::string::String, Vec<(std::string::String, RustPattern)>),
    /// Enum variant pattern: `Some(x)`, `Ok(v)`
    Enum(std::string::String, Vec<RustPattern>),
    /// Reference pattern: `&x` or `&mut x`
    Ref(bool, Box<RustPattern>),
    /// Or pattern: `A | B | C`
    Or(Vec<RustPattern>),
    /// Range pattern: `1..=10`
    Range(Box<RustLit>, Box<RustLit>),
    /// Guard pattern: `x if condition`
    Guard(Box<RustPattern>, Box<RustExpr>),
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RustWorklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
impl RustWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        RustWorklist {
            items: std::collections::VecDeque::new(),
            in_worklist: std::collections::HashSet::new(),
        }
    }
    #[allow(dead_code)]
    pub fn push(&mut self, item: u32) -> bool {
        if self.in_worklist.insert(item) {
            self.items.push_back(item);
            true
        } else {
            false
        }
    }
    #[allow(dead_code)]
    pub fn pop(&mut self) -> Option<u32> {
        let item = self.items.pop_front()?;
        self.in_worklist.remove(&item);
        Some(item)
    }
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.items.len()
    }
    #[allow(dead_code)]
    pub fn contains(&self, item: u32) -> bool {
        self.in_worklist.contains(&item)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RustDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
impl RustDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        RustDominatorTree {
            idom: vec![None; size],
            dom_children: vec![Vec::new(); size],
            dom_depth: vec![0; size],
        }
    }
    #[allow(dead_code)]
    pub fn set_idom(&mut self, node: usize, idom: u32) {
        self.idom[node] = Some(idom);
    }
    #[allow(dead_code)]
    pub fn dominates(&self, a: usize, b: usize) -> bool {
        if a == b {
            return true;
        }
        let mut cur = b;
        loop {
            match self.idom[cur] {
                Some(parent) if parent as usize == a => return true,
                Some(parent) if parent as usize == cur => return false,
                Some(parent) => cur = parent as usize,
                None => return false,
            }
        }
    }
    #[allow(dead_code)]
    pub fn depth(&self, node: usize) -> u32 {
        self.dom_depth.get(node).copied().unwrap_or(0)
    }
}
/// Rust native target code generation backend.
pub struct RustTargetBackend {
    pub(super) fresh_counter: u64,
    pub(super) name_cache: HashMap<std::string::String, std::string::String>,
}
impl RustTargetBackend {
    /// Create a new `RustTargetBackend`.
    pub fn new() -> Self {
        RustTargetBackend {
            fresh_counter: 0,
            name_cache: HashMap::new(),
        }
    }
    /// Generate a fresh variable name.
    pub fn fresh_var(&mut self) -> std::string::String {
        let n = self.fresh_counter;
        self.fresh_counter += 1;
        format!("_t{}", n)
    }
    /// Mangle an OxiLean name to a valid Rust identifier.
    pub fn mangle_name(&mut self, name: &str) -> std::string::String {
        if let Some(cached) = self.name_cache.get(name) {
            return cached.clone();
        }
        if name.is_empty() {
            return "_anon".to_string();
        }
        let mangled: std::string::String = name
            .chars()
            .map(|c| match c {
                '.' | ':' => '_',
                '\'' => '_',
                c if c.is_alphanumeric() || c == '_' => c,
                _ => '_',
            })
            .collect();
        let mangled = if RUST_KEYWORDS.contains(&mangled.as_str()) {
            format!("{}_", mangled)
        } else if mangled.starts_with(|c: char| c.is_ascii_digit()) {
            format!("_{}", mangled)
        } else {
            mangled
        };
        self.name_cache.insert(name.to_string(), mangled.clone());
        mangled
    }
    /// Map an LCNF type to a Rust type.
    pub fn lcnf_to_rust_type(ty: &LcnfType) -> RustType {
        match ty {
            LcnfType::Nat => RustType::U64,
            LcnfType::LcnfString => RustType::RustString,
            LcnfType::Unit | LcnfType::Erased | LcnfType::Irrelevant => RustType::Unit,
            LcnfType::Object => RustType::Custom("Box<dyn std::any::Any>".to_string()),
            LcnfType::Var(name) => RustType::Custom(name.clone()),
            LcnfType::Fun(params, ret) => {
                let p: Vec<_> = params.iter().map(Self::lcnf_to_rust_type).collect();
                RustType::Fn(p, Box::new(Self::lcnf_to_rust_type(ret)))
            }
            LcnfType::Ctor(name, args) => {
                if args.is_empty() {
                    RustType::Custom(name.clone())
                } else {
                    let a: Vec<_> = args.iter().map(Self::lcnf_to_rust_type).collect();
                    RustType::Generic(name.clone(), a)
                }
            }
        }
    }
    /// Compile an LCNF literal to a Rust expression.
    pub fn compile_lit(lit: &LcnfLit) -> RustExpr {
        match lit {
            LcnfLit::Nat(n) => RustExpr::Lit(RustLit::UInt(*n)),
            LcnfLit::Str(s) => RustExpr::Lit(RustLit::Str(s.clone())),
        }
    }
    /// Compile an LCNF let-value to a Rust expression.
    pub fn compile_let_value(&mut self, value: &LcnfLetValue) -> RustExpr {
        match value {
            LcnfLetValue::App(func, args) => {
                let func_expr = self.compile_arg(func);
                let rust_args: Vec<_> = args.iter().map(|a| self.compile_arg(a)).collect();
                RustExpr::Call(Box::new(func_expr), rust_args)
            }
            LcnfLetValue::Ctor(ctor_name, _tag, fields) => {
                let rust_fields: Vec<_> = fields
                    .iter()
                    .enumerate()
                    .map(|(i, f)| (format!("_{}", i), self.compile_arg(f)))
                    .collect();
                RustExpr::Struct(ctor_name.clone(), rust_fields)
            }
            LcnfLetValue::Proj(_name, index, var) => {
                let val_expr = RustExpr::Var(var.to_string());
                RustExpr::Field(Box::new(val_expr), format!("_{}", index))
            }
            LcnfLetValue::Lit(lit) => Self::compile_lit(lit),
            LcnfLetValue::Erased => RustExpr::Lit(RustLit::Unit),
            LcnfLetValue::FVar(id) => RustExpr::Var(id.to_string()),
            LcnfLetValue::Reset(_) => RustExpr::Lit(RustLit::Unit),
            LcnfLetValue::Reuse(_slot, ctor_name, _tag, fields) => {
                let rust_fields: Vec<_> = fields
                    .iter()
                    .enumerate()
                    .map(|(i, f)| (format!("_{}", i), self.compile_arg(f)))
                    .collect();
                RustExpr::Struct(ctor_name.clone(), rust_fields)
            }
        }
    }
    /// Compile an LCNF argument to a Rust expression.
    pub fn compile_arg(&mut self, arg: &LcnfArg) -> RustExpr {
        match arg {
            LcnfArg::Var(id) => RustExpr::Var(id.to_string()),
            LcnfArg::Lit(lit) => Self::compile_lit(lit),
            LcnfArg::Erased => RustExpr::Lit(RustLit::Unit),
            LcnfArg::Type(_) => RustExpr::Lit(RustLit::Unit),
        }
    }
    /// Compile an LCNF expression into Rust statements, returning the result expr.
    pub fn compile_expr(&mut self, expr: &LcnfExpr, stmts: &mut Vec<RustStmt>) -> RustExpr {
        match expr {
            LcnfExpr::Return(arg) => self.compile_arg(arg),
            LcnfExpr::Let {
                id,
                ty,
                value,
                body,
                ..
            } => {
                let val_expr = self.compile_let_value(value);
                let rust_ty = Self::lcnf_to_rust_type(ty);
                stmts.push(RustStmt::Let {
                    pat: RustPattern::Var(id.to_string(), false),
                    ty: Some(rust_ty),
                    value: Some(val_expr),
                });
                self.compile_expr(body, stmts)
            }
            LcnfExpr::TailCall(func, args) => {
                let func_expr = self.compile_arg(func);
                let rust_args: Vec<_> = args.iter().map(|a| self.compile_arg(a)).collect();
                RustExpr::Call(Box::new(func_expr), rust_args)
            }
            LcnfExpr::Case {
                scrutinee,
                alts,
                default,
                ..
            } => {
                let scrut_var = RustExpr::Var(scrutinee.to_string());
                let tag_field = RustExpr::Field(Box::new(scrut_var.clone()), "tag".to_string());
                let mut arms: Vec<(RustPattern, RustExpr)> = Vec::new();
                for alt in alts {
                    let mut case_stmts: Vec<RustStmt> = Vec::new();
                    for (idx, param) in alt.params.iter().enumerate() {
                        let field_access =
                            RustExpr::Field(Box::new(scrut_var.clone()), format!("_{}", idx));
                        case_stmts.push(RustStmt::Let {
                            pat: RustPattern::Var(param.id.to_string(), false),
                            ty: None,
                            value: Some(field_access),
                        });
                    }
                    let case_result = self.compile_expr(&alt.body, &mut case_stmts);
                    let arm_expr = if case_stmts.is_empty() {
                        case_result
                    } else {
                        RustExpr::Block(case_stmts, Some(Box::new(case_result)))
                    };
                    arms.push((
                        RustPattern::Lit(RustLit::Str(alt.ctor_name.clone())),
                        arm_expr,
                    ));
                }
                if let Some(def) = default {
                    let mut def_stmts: Vec<RustStmt> = Vec::new();
                    let def_result = self.compile_expr(def, &mut def_stmts);
                    let def_expr = if def_stmts.is_empty() {
                        def_result
                    } else {
                        RustExpr::Block(def_stmts, Some(Box::new(def_result)))
                    };
                    arms.push((RustPattern::Wildcard, def_expr));
                }
                let result_var = self.fresh_var();
                stmts.push(RustStmt::Let {
                    pat: RustPattern::Var(result_var.clone(), false),
                    ty: None,
                    value: Some(RustExpr::Match {
                        scrutinee: Box::new(tag_field),
                        arms,
                    }),
                });
                RustExpr::Var(result_var)
            }
            LcnfExpr::Unreachable => {
                RustExpr::MacroCall("unreachable".to_string(), std::string::String::new())
            }
        }
    }
    /// Compile an LCNF function declaration to a `RustFn`.
    pub fn compile_decl(&mut self, decl: &LcnfFunDecl) -> Result<RustFn, std::string::String> {
        let rust_name = self.mangle_name(&decl.name);
        let params: Vec<_> = decl
            .params
            .iter()
            .map(|p| {
                let ty = Self::lcnf_to_rust_type(&p.ty);
                (p.id.to_string(), ty, false)
            })
            .collect();
        let ret_ty = Self::lcnf_to_rust_type(&decl.ret_type);
        let mut body_stmts: Vec<RustStmt> = Vec::new();
        let result_expr = self.compile_expr(&decl.body, &mut body_stmts);
        body_stmts.push(RustStmt::ExprNoSemi(result_expr));
        Ok(RustFn {
            name: rust_name,
            generics: Vec::new(),
            params,
            return_type: Some(ret_ty),
            body: body_stmts,
            attrs: Vec::new(),
            visibility: RustVisibility::Pub,
            is_async: false,
            is_unsafe: false,
        })
    }
    /// Compile a list of declarations and emit a `RustModule`.
    pub fn emit_module(&mut self, name: &str, decls: &[LcnfFunDecl]) -> RustModule {
        let mut module = RustModule::new(name);
        for decl in decls {
            if let Ok(func) = self.compile_decl(decl) {
                module.items.push(RustItem::Fn(func));
            }
        }
        module
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RustDepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
}
impl RustDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        RustDepGraph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn add_node(&mut self, id: u32) {
        if !self.nodes.contains(&id) {
            self.nodes.push(id);
        }
    }
    #[allow(dead_code)]
    pub fn add_dep(&mut self, dep: u32, dependent: u32) {
        self.add_node(dep);
        self.add_node(dependent);
        self.edges.push((dep, dependent));
    }
    #[allow(dead_code)]
    pub fn dependents_of(&self, node: u32) -> Vec<u32> {
        self.edges
            .iter()
            .filter(|(d, _)| *d == node)
            .map(|(_, dep)| *dep)
            .collect()
    }
    #[allow(dead_code)]
    pub fn dependencies_of(&self, node: u32) -> Vec<u32> {
        self.edges
            .iter()
            .filter(|(_, dep)| *dep == node)
            .map(|(d, _)| *d)
            .collect()
    }
    #[allow(dead_code)]
    pub fn topological_sort(&self) -> Vec<u32> {
        let mut in_degree: std::collections::HashMap<u32, u32> = std::collections::HashMap::new();
        for &n in &self.nodes {
            in_degree.insert(n, 0);
        }
        for (_, dep) in &self.edges {
            *in_degree.entry(*dep).or_insert(0) += 1;
        }
        let mut queue: std::collections::VecDeque<u32> = self
            .nodes
            .iter()
            .filter(|&&n| in_degree[&n] == 0)
            .copied()
            .collect();
        let mut result = Vec::new();
        while let Some(node) = queue.pop_front() {
            result.push(node);
            for dep in self.dependents_of(node) {
                let cnt = in_degree.entry(dep).or_insert(0);
                *cnt = cnt.saturating_sub(1);
                if *cnt == 0 {
                    queue.push_back(dep);
                }
            }
        }
        result
    }
    #[allow(dead_code)]
    pub fn has_cycle(&self) -> bool {
        self.topological_sort().len() < self.nodes.len()
    }
}
#[allow(dead_code)]
pub struct RustConstantFoldingHelper;
impl RustConstantFoldingHelper {
    #[allow(dead_code)]
    pub fn fold_add_i64(a: i64, b: i64) -> Option<i64> {
        a.checked_add(b)
    }
    #[allow(dead_code)]
    pub fn fold_sub_i64(a: i64, b: i64) -> Option<i64> {
        a.checked_sub(b)
    }
    #[allow(dead_code)]
    pub fn fold_mul_i64(a: i64, b: i64) -> Option<i64> {
        a.checked_mul(b)
    }
    #[allow(dead_code)]
    pub fn fold_div_i64(a: i64, b: i64) -> Option<i64> {
        if b == 0 {
            None
        } else {
            a.checked_div(b)
        }
    }
    #[allow(dead_code)]
    pub fn fold_add_f64(a: f64, b: f64) -> f64 {
        a + b
    }
    #[allow(dead_code)]
    pub fn fold_mul_f64(a: f64, b: f64) -> f64 {
        a * b
    }
    #[allow(dead_code)]
    pub fn fold_neg_i64(a: i64) -> Option<i64> {
        a.checked_neg()
    }
    #[allow(dead_code)]
    pub fn fold_not_bool(a: bool) -> bool {
        !a
    }
    #[allow(dead_code)]
    pub fn fold_and_bool(a: bool, b: bool) -> bool {
        a && b
    }
    #[allow(dead_code)]
    pub fn fold_or_bool(a: bool, b: bool) -> bool {
        a || b
    }
    #[allow(dead_code)]
    pub fn fold_shl_i64(a: i64, b: u32) -> Option<i64> {
        a.checked_shl(b)
    }
    #[allow(dead_code)]
    pub fn fold_shr_i64(a: i64, b: u32) -> Option<i64> {
        a.checked_shr(b)
    }
    #[allow(dead_code)]
    pub fn fold_rem_i64(a: i64, b: i64) -> Option<i64> {
        if b == 0 {
            None
        } else {
            Some(a % b)
        }
    }
    #[allow(dead_code)]
    pub fn fold_bitand_i64(a: i64, b: i64) -> i64 {
        a & b
    }
    #[allow(dead_code)]
    pub fn fold_bitor_i64(a: i64, b: i64) -> i64 {
        a | b
    }
    #[allow(dead_code)]
    pub fn fold_bitxor_i64(a: i64, b: i64) -> i64 {
        a ^ b
    }
    #[allow(dead_code)]
    pub fn fold_bitnot_i64(a: i64) -> i64 {
        !a
    }
}
/// Rust expression AST.
#[derive(Debug, Clone, PartialEq)]
pub enum RustExpr {
    /// Literal: `42`, `"hello"`, `true`
    Lit(RustLit),
    /// Variable reference: `x`
    Var(std::string::String),
    /// Binary operator: `a + b`
    BinOp {
        op: std::string::String,
        lhs: Box<RustExpr>,
        rhs: Box<RustExpr>,
    },
    /// Unary operator: `!x`, `-x`, `*x`
    UnaryOp {
        op: std::string::String,
        operand: Box<RustExpr>,
    },
    /// Block expression: `{ stmts... expr }`
    Block(Vec<RustStmt>, Option<Box<RustExpr>>),
    /// If expression: `if cond { then } else { else }`
    If {
        cond: Box<RustExpr>,
        then_block: Vec<RustStmt>,
        else_block: Option<Vec<RustStmt>>,
    },
    /// Match expression: `match x { pat => expr, ... }`
    Match {
        scrutinee: Box<RustExpr>,
        arms: Vec<(RustPattern, RustExpr)>,
    },
    /// Loop expression: `loop { ... }`
    Loop(Vec<RustStmt>),
    /// For loop: `for pat in iter { ... }`
    For {
        pat: RustPattern,
        iter: Box<RustExpr>,
        body: Vec<RustStmt>,
    },
    /// While loop: `while cond { ... }`
    While {
        cond: Box<RustExpr>,
        body: Vec<RustStmt>,
    },
    /// Return: `return expr`
    Return(Option<Box<RustExpr>>),
    /// Break: `break [expr]`
    Break(Option<Box<RustExpr>>),
    /// Continue
    Continue,
    /// Closure: `|params| body`
    Closure {
        params: Vec<(std::string::String, Option<RustType>)>,
        ret_ty: Option<RustType>,
        body: Box<RustExpr>,
        is_move: bool,
    },
    /// Function call: `f(args)`
    Call(Box<RustExpr>, Vec<RustExpr>),
    /// Method call: `obj.method(args)`
    MethodCall {
        receiver: Box<RustExpr>,
        method: std::string::String,
        args: Vec<RustExpr>,
    },
    /// Field access: `x.field`
    Field(Box<RustExpr>, std::string::String),
    /// Index: `x[i]`
    Index(Box<RustExpr>, Box<RustExpr>),
    /// Reference: `&x` or `&mut x`
    Ref(bool, Box<RustExpr>),
    /// Dereference: `*x`
    Deref(Box<RustExpr>),
    /// Struct literal: `Foo { x: 1, y: 2 }`
    Struct(std::string::String, Vec<(std::string::String, RustExpr)>),
    /// Tuple literal: `(a, b, c)`
    Tuple(Vec<RustExpr>),
    /// Array literal: `[a, b, c]`
    Array(Vec<RustExpr>),
    /// Range: `a..b` or `a..=b`
    Range(Option<Box<RustExpr>>, Option<Box<RustExpr>>, bool),
    /// `?` try operator
    Try(Box<RustExpr>),
    /// `.await`
    Await(Box<RustExpr>),
    /// Path expression: `std::path::to::item`
    Path(Vec<std::string::String>),
    /// Macro call: `println!("hello", x)`
    MacroCall(std::string::String, std::string::String),
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RustPassConfig {
    pub phase: RustPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
impl RustPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: RustPassPhase) -> Self {
        RustPassConfig {
            phase,
            enabled: true,
            max_iterations: 10,
            debug_output: false,
            pass_name: name.into(),
        }
    }
    #[allow(dead_code)]
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }
    #[allow(dead_code)]
    pub fn with_debug(mut self) -> Self {
        self.debug_output = true;
        self
    }
    #[allow(dead_code)]
    pub fn max_iter(mut self, n: u32) -> Self {
        self.max_iterations = n;
        self
    }
}
