//! Auto-generated module (split from types.rs)
//!
//! Second half of type definitions and impl blocks.

use super::defs::*;
use std::collections::{HashMap, HashSet, VecDeque};

/// Dominator tree for ZigExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ZigExtDomTree {
    pub(crate) idom: Vec<Option<usize>>,
    pub(crate) children: Vec<Vec<usize>>,
    pub(crate) depth: Vec<usize>,
}
impl ZigExtDomTree {
    #[allow(dead_code)]
    pub fn new(n: usize) -> Self {
        Self {
            idom: vec![None; n],
            children: vec![Vec::new(); n],
            depth: vec![0; n],
        }
    }
    #[allow(dead_code)]
    pub fn set_idom(&mut self, node: usize, dom: usize) {
        if node < self.idom.len() {
            self.idom[node] = Some(dom);
            if dom < self.children.len() {
                self.children[dom].push(node);
            }
            self.depth[node] = if dom < self.depth.len() {
                self.depth[dom] + 1
            } else {
                1
            };
        }
    }
    #[allow(dead_code)]
    pub fn dominates(&self, a: usize, mut b: usize) -> bool {
        if a == b {
            return true;
        }
        let n = self.idom.len();
        for _ in 0..n {
            match self.idom.get(b).copied().flatten() {
                None => return false,
                Some(p) if p == a => return true,
                Some(p) if p == b => return false,
                Some(p) => b = p,
            }
        }
        false
    }
    #[allow(dead_code)]
    pub fn children_of(&self, n: usize) -> &[usize] {
        self.children.get(n).map(|v| v.as_slice()).unwrap_or(&[])
    }
    #[allow(dead_code)]
    pub fn depth_of(&self, n: usize) -> usize {
        self.depth.get(n).copied().unwrap_or(0)
    }
    #[allow(dead_code)]
    pub fn lca(&self, mut a: usize, mut b: usize) -> usize {
        let n = self.idom.len();
        for _ in 0..(2 * n) {
            if a == b {
                return a;
            }
            if self.depth_of(a) > self.depth_of(b) {
                a = self.idom.get(a).and_then(|x| *x).unwrap_or(a);
            } else {
                b = self.idom.get(b).and_then(|x| *x).unwrap_or(b);
            }
        }
        0
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ZigCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
/// Represents a complete Zig module (source file).
pub struct ZigModule {
    pub name: String,
    pub imports: Vec<String>,
    pub structs: Vec<ZigStruct>,
    pub fns: Vec<ZigFn>,
    pub consts: Vec<(String, ZigType, ZigExpr)>,
}
impl ZigModule {
    /// Create a new empty module.
    pub fn new(name: &str) -> Self {
        ZigModule {
            name: name.to_string(),
            imports: Vec::new(),
            structs: Vec::new(),
            fns: Vec::new(),
            consts: Vec::new(),
        }
    }
    /// Add an import statement (e.g., `@import("std")`).
    pub fn add_import(&mut self, path: &str) {
        self.imports.push(path.to_string());
    }
    /// Add a struct definition.
    pub fn add_struct(&mut self, s: ZigStruct) {
        self.structs.push(s);
    }
    /// Add a function definition.
    pub fn add_fn(&mut self, f: ZigFn) {
        self.fns.push(f);
    }
    /// Generate the complete Zig module source code.
    pub fn codegen(&self) -> String {
        let mut parts: Vec<String> = Vec::new();
        for (i, path) in self.imports.iter().enumerate() {
            let binding = format!("_import_{}", i);
            parts.push(format!("const {} = @import(\"{}\");", binding, path));
        }
        for (name, ty, expr) in &self.consts {
            parts.push(format!(
                "const {}: {} = {};",
                name,
                ty.codegen(),
                expr.codegen()
            ));
        }
        for s in &self.structs {
            parts.push(s.codegen());
        }
        for f in &self.fns {
            parts.push(f.codegen());
        }
        parts.join("\n\n")
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ZigPackedField {
    pub name: String,
    pub ty: ZigType,
    pub bit_width: Option<u32>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ZigAllocatorKind {
    GeneralPurpose,
    Arena,
    Page,
    FixedBuffer(usize),
    C,
    LogToFile,
}
impl ZigAllocatorKind {
    #[allow(dead_code)]
    pub fn type_name(&self) -> &str {
        match self {
            ZigAllocatorKind::GeneralPurpose => "std.heap.GeneralPurposeAllocator(.{})",
            ZigAllocatorKind::Arena => "std.heap.ArenaAllocator",
            ZigAllocatorKind::Page => "std.heap.page_allocator",
            ZigAllocatorKind::FixedBuffer(_) => "std.heap.FixedBufferAllocator",
            ZigAllocatorKind::C => "std.heap.c_allocator",
            ZigAllocatorKind::LogToFile => "std.heap.LoggingAllocator",
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum ZigPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl ZigPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            ZigPassPhase::Analysis => "analysis",
            ZigPassPhase::Transformation => "transformation",
            ZigPassPhase::Verification => "verification",
            ZigPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(self, ZigPassPhase::Transformation | ZigPassPhase::Cleanup)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ZigDepGraph {
    pub(crate) nodes: Vec<u32>,
    pub(crate) edges: Vec<(u32, u32)>,
}
impl ZigDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        ZigDepGraph {
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
/// Statistics for ZigExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ZigExtPassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}
impl ZigExtPassStats {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn visit(&mut self) {
        self.nodes_visited += 1;
    }
    #[allow(dead_code)]
    pub fn modify(&mut self) {
        self.nodes_modified += 1;
        self.changed = true;
    }
    #[allow(dead_code)]
    pub fn iterate(&mut self) {
        self.iterations += 1;
    }
    #[allow(dead_code)]
    pub fn error(&mut self) {
        self.errors += 1;
    }
    #[allow(dead_code)]
    pub fn efficiency(&self) -> f64 {
        if self.nodes_visited == 0 {
            0.0
        } else {
            self.nodes_modified as f64 / self.nodes_visited as f64
        }
    }
    #[allow(dead_code)]
    pub fn merge(&mut self, o: &ZigExtPassStats) {
        self.iterations += o.iterations;
        self.changed |= o.changed;
        self.nodes_visited += o.nodes_visited;
        self.nodes_modified += o.nodes_modified;
        self.time_ms += o.time_ms;
        self.memory_bytes = self.memory_bytes.max(o.memory_bytes);
        self.errors += o.errors;
    }
}
/// Represents a Zig struct definition.
pub struct ZigStruct {
    pub name: String,
    pub fields: Vec<(String, ZigType)>,
    pub is_pub: bool,
}
impl ZigStruct {
    /// Create a new struct with the given name.
    pub fn new(name: &str) -> Self {
        ZigStruct {
            name: name.to_string(),
            fields: Vec::new(),
            is_pub: false,
        }
    }
    /// Add a field to the struct.
    pub fn add_field(&mut self, name: &str, ty: ZigType) {
        self.fields.push((name.to_string(), ty));
    }
    /// Generate Zig source code for this struct.
    pub fn codegen(&self) -> String {
        let pub_str = if self.is_pub { "pub " } else { "" };
        let fields_str: Vec<String> = self
            .fields
            .iter()
            .map(|(name, ty)| format!("    {}: {},", name, ty.codegen()))
            .collect();
        format!(
            "{}const {} = struct {{\n{}\n}};",
            pub_str,
            self.name,
            fields_str.join("\n")
        )
    }
}
/// Liveness analysis for ZigExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ZigExtLiveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}
impl ZigExtLiveness {
    #[allow(dead_code)]
    pub fn new(n: usize) -> Self {
        Self {
            live_in: vec![Vec::new(); n],
            live_out: vec![Vec::new(); n],
            defs: vec![Vec::new(); n],
            uses: vec![Vec::new(); n],
        }
    }
    #[allow(dead_code)]
    pub fn live_in(&self, b: usize, v: usize) -> bool {
        self.live_in.get(b).map(|s| s.contains(&v)).unwrap_or(false)
    }
    #[allow(dead_code)]
    pub fn live_out(&self, b: usize, v: usize) -> bool {
        self.live_out
            .get(b)
            .map(|s| s.contains(&v))
            .unwrap_or(false)
    }
    #[allow(dead_code)]
    pub fn add_def(&mut self, b: usize, v: usize) {
        if let Some(s) = self.defs.get_mut(b) {
            if !s.contains(&v) {
                s.push(v);
            }
        }
    }
    #[allow(dead_code)]
    pub fn add_use(&mut self, b: usize, v: usize) {
        if let Some(s) = self.uses.get_mut(b) {
            if !s.contains(&v) {
                s.push(v);
            }
        }
    }
    #[allow(dead_code)]
    pub fn var_is_used_in_block(&self, b: usize, v: usize) -> bool {
        self.uses.get(b).map(|s| s.contains(&v)).unwrap_or(false)
    }
    #[allow(dead_code)]
    pub fn var_is_def_in_block(&self, b: usize, v: usize) -> bool {
        self.defs.get(b).map(|s| s.contains(&v)).unwrap_or(false)
    }
}
/// Zig type representation.
#[derive(Debug, Clone, PartialEq)]
pub enum ZigType {
    Void,
    Bool,
    U8,
    U64,
    I64,
    F64,
    /// Generic signed integer type.
    Int,
    Ptr(Box<ZigType>),
    Slice(Box<ZigType>),
    Struct(String),
    Fn(Vec<ZigType>, Box<ZigType>),
    Optional(Box<ZigType>),
    ErrorUnion(Box<ZigType>),
    Anyopaque,
}
impl ZigType {
    /// Emit Zig source representation of this type.
    pub fn codegen(&self) -> String {
        match self {
            ZigType::Void => "void".to_string(),
            ZigType::Bool => "bool".to_string(),
            ZigType::U8 => "u8".to_string(),
            ZigType::U64 => "u64".to_string(),
            ZigType::I64 => "i64".to_string(),
            ZigType::F64 => "f64".to_string(),
            ZigType::Int => "i64".to_string(),
            ZigType::Ptr(inner) => format!("*{}", inner.codegen()),
            ZigType::Slice(inner) => format!("[]{}", inner.codegen()),
            ZigType::Struct(name) => name.clone(),
            ZigType::Fn(params, ret) => {
                let params_str: Vec<String> = params.iter().map(|p| p.codegen()).collect();
                format!("fn({}) {}", params_str.join(", "), ret.codegen())
            }
            ZigType::Optional(inner) => format!("?{}", inner.codegen()),
            ZigType::ErrorUnion(inner) => format!("!{}", inner.codegen()),
            ZigType::Anyopaque => "anyopaque".to_string(),
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ZigTestBlock {
    pub name: String,
    pub body: Vec<String>,
}
impl ZigTestBlock {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        ZigTestBlock {
            name: name.into(),
            body: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn add_line(&mut self, line: impl Into<String>) {
        self.body.push(line.into());
    }
    #[allow(dead_code)]
    pub fn add_expect(mut self, lhs: &str, rhs: &str) -> Self {
        self.body
            .push(format!("try std.testing.expectEqual({}, {});", lhs, rhs));
        self
    }
    #[allow(dead_code)]
    pub fn add_expect_equal_strings(mut self, lhs: &str, rhs: &str) -> Self {
        self.body.push(format!(
            "try std.testing.expectEqualStrings({}, {});",
            lhs, rhs
        ));
        self
    }
    #[allow(dead_code)]
    pub fn emit(&self) -> String {
        let mut out = format!("test \"{}\" {{\n", self.name);
        for line in &self.body {
            out.push_str(&format!("    {}\n", line));
        }
        out.push('}');
        out
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ZigTaggedUnion {
    pub name: String,
    pub tag_type: Option<String>,
    pub fields: Vec<(String, Option<ZigType>)>,
}
impl ZigTaggedUnion {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        ZigTaggedUnion {
            name: name.into(),
            tag_type: None,
            fields: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn with_tag(mut self, ty: impl Into<String>) -> Self {
        self.tag_type = Some(ty.into());
        self
    }
    #[allow(dead_code)]
    pub fn add_field(&mut self, name: impl Into<String>, ty: Option<ZigType>) {
        self.fields.push((name.into(), ty));
    }
    #[allow(dead_code)]
    pub fn emit(&self) -> String {
        let tag_part = if let Some(ref t) = self.tag_type {
            format!("union({}) ", t)
        } else {
            "union ".to_string()
        };
        let mut out = format!("const {} = {}{{\n", self.name, tag_part);
        for (name, ty) in &self.fields {
            match ty {
                Some(_) => out.push_str(&format!("    {}: {},\n", name, "type")),
                None => out.push_str(&format!("    {},\n", name)),
            }
        }
        out.push_str("};");
        out
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ZigAnalysisCache {
    pub(crate) entries: std::collections::HashMap<String, ZigCacheEntry>,
    pub(crate) max_size: usize,
    pub(crate) hits: u64,
    pub(crate) misses: u64,
}
impl ZigAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        ZigAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&ZigCacheEntry> {
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
            ZigCacheEntry {
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
/// Zig expression representation.
#[derive(Debug, Clone, PartialEq)]
pub enum ZigExpr {
    Ident(String),
    IntLit(i64),
    FloatLit(f64),
    BoolLit(bool),
    StringLit(String),
    NullLit,
    BinOp {
        op: String,
        lhs: Box<ZigExpr>,
        rhs: Box<ZigExpr>,
    },
    UnaryOp {
        op: String,
        operand: Box<ZigExpr>,
    },
    Call {
        callee: Box<ZigExpr>,
        args: Vec<ZigExpr>,
    },
    FieldAccess {
        base: Box<ZigExpr>,
        field: String,
    },
    Try(Box<ZigExpr>),
    Await(Box<ZigExpr>),
    Comptime(Box<ZigExpr>),
    If {
        cond: Box<ZigExpr>,
        then: Box<ZigExpr>,
        else_: Option<Box<ZigExpr>>,
    },
    Block(Vec<ZigStmt>),
}
impl ZigExpr {
    /// Emit Zig source for this expression.
    pub fn codegen(&self) -> String {
        match self {
            ZigExpr::Ident(name) => name.clone(),
            ZigExpr::IntLit(n) => n.to_string(),
            ZigExpr::FloatLit(f) => format!("{}", f),
            ZigExpr::BoolLit(b) => b.to_string(),
            ZigExpr::StringLit(s) => {
                format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
            }
            ZigExpr::NullLit => "null".to_string(),
            ZigExpr::BinOp { op, lhs, rhs } => {
                format!("({} {} {})", lhs.codegen(), op, rhs.codegen())
            }
            ZigExpr::UnaryOp { op, operand } => format!("({}{})", op, operand.codegen()),
            ZigExpr::Call { callee, args } => {
                let args_str: Vec<String> = args.iter().map(|a| a.codegen()).collect();
                format!("{}({})", callee.codegen(), args_str.join(", "))
            }
            ZigExpr::FieldAccess { base, field } => {
                format!("{}.{}", base.codegen(), field)
            }
            ZigExpr::Try(inner) => format!("try {}", inner.codegen()),
            ZigExpr::Await(inner) => format!("await {}", inner.codegen()),
            ZigExpr::Comptime(inner) => format!("comptime {}", inner.codegen()),
            ZigExpr::If { cond, then, else_ } => {
                let else_str = else_
                    .as_ref()
                    .map(|e| format!(" else {}", e.codegen()))
                    .unwrap_or_default();
                format!("if ({}) {}{}", cond.codegen(), then.codegen(), else_str)
            }
            ZigExpr::Block(stmts) => {
                let body: Vec<String> = stmts
                    .iter()
                    .map(|s| format!("    {}", s.codegen()))
                    .collect();
                format!("{{\n{}\n}}", body.join("\n"))
            }
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ZigOptimizeMode {
    Debug,
    ReleaseSafe,
    ReleaseFast,
    ReleaseSmall,
}
impl ZigOptimizeMode {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            ZigOptimizeMode::Debug => "Debug",
            ZigOptimizeMode::ReleaseSafe => "ReleaseSafe",
            ZigOptimizeMode::ReleaseFast => "ReleaseFast",
            ZigOptimizeMode::ReleaseSmall => "ReleaseSmall",
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ZigDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
impl ZigDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        ZigDominatorTree {
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ZigImport {
    pub module: String,
    pub alias: Option<String>,
}
impl ZigImport {
    #[allow(dead_code)]
    pub fn std() -> Self {
        ZigImport {
            module: "std".to_string(),
            alias: Some("std".to_string()),
        }
    }
    #[allow(dead_code)]
    pub fn module(module: impl Into<String>) -> Self {
        let m = module.into();
        ZigImport {
            module: m.clone(),
            alias: Some(m),
        }
    }
    #[allow(dead_code)]
    pub fn with_alias(mut self, alias: impl Into<String>) -> Self {
        self.alias = Some(alias.into());
        self
    }
    #[allow(dead_code)]
    pub fn emit(&self) -> String {
        if let Some(ref alias) = self.alias {
            format!("const {} = @import(\"{}\");", alias, self.module)
        } else {
            format!("_ = @import(\"{}\");", self.module)
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ZigAllocatorUsage {
    pub allocator_type: ZigAllocatorKind,
    pub var_name: String,
}
impl ZigAllocatorUsage {
    #[allow(dead_code)]
    pub fn new(kind: ZigAllocatorKind, var_name: impl Into<String>) -> Self {
        ZigAllocatorUsage {
            allocator_type: kind,
            var_name: var_name.into(),
        }
    }
    #[allow(dead_code)]
    pub fn emit_init(&self) -> String {
        match &self.allocator_type {
            ZigAllocatorKind::GeneralPurpose => {
                format!(
                    "var {} = std.heap.GeneralPurposeAllocator(.{{}}){{}};\ndefer _ = {}.deinit();",
                    self.var_name, self.var_name
                )
            }
            ZigAllocatorKind::Arena => {
                format!(
                    "var {} = std.heap.ArenaAllocator.init(std.heap.page_allocator);\ndefer {}.deinit();",
                    self.var_name, self.var_name
                )
            }
            ZigAllocatorKind::FixedBuffer(size) => {
                format!(
                    "var buf: [{}]u8 = undefined;\nvar {} = std.heap.FixedBufferAllocator.init(&buf);",
                    size, self.var_name
                )
            }
            _ => format!("// allocator: {}", self.allocator_type.type_name()),
        }
    }
    #[allow(dead_code)]
    pub fn emit_interface_call(&self) -> String {
        match &self.allocator_type {
            ZigAllocatorKind::GeneralPurpose => format!("{}.allocator()", self.var_name),
            ZigAllocatorKind::Arena => format!("{}.allocator()", self.var_name),
            ZigAllocatorKind::FixedBuffer(_) => format!("{}.allocator()", self.var_name),
            ZigAllocatorKind::Page => "std.heap.page_allocator".to_string(),
            ZigAllocatorKind::C => "std.heap.c_allocator".to_string(),
            _ => format!("{}.allocator()", self.var_name),
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ZigComptime {
    pub body: Vec<ZigStmt>,
    pub is_block: bool,
}
impl ZigComptime {
    #[allow(dead_code)]
    pub fn new() -> Self {
        ZigComptime {
            body: Vec::new(),
            is_block: true,
        }
    }
    #[allow(dead_code)]
    pub fn add_stmt(&mut self, stmt: ZigStmt) {
        self.body.push(stmt);
    }
    #[allow(dead_code)]
    pub fn emit(&self) -> String {
        if self.is_block {
            format!(
                "comptime {{\n{}}}",
                self.body
                    .iter()
                    .map(|_| "    // stmt\n")
                    .collect::<String>()
            )
        } else {
            "comptime expr".to_string()
        }
    }
}
#[allow(dead_code)]
pub struct ZigPassRegistry {
    pub(crate) configs: Vec<ZigPassConfig>,
    pub(crate) stats: std::collections::HashMap<String, ZigPassStats>,
}
impl ZigPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        ZigPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: ZigPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), ZigPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&ZigPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&ZigPassStats> {
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
pub struct ZigGenericFn {
    pub name: String,
    pub type_params: Vec<String>,
    pub params: Vec<(String, String)>,
    pub return_type: String,
    pub body: Vec<ZigStmt>,
}
impl ZigGenericFn {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        ZigGenericFn {
            name: name.into(),
            type_params: Vec::new(),
            params: Vec::new(),
            return_type: "void".to_string(),
            body: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn type_param(mut self, tp: impl Into<String>) -> Self {
        self.type_params.push(tp.into());
        self
    }
    #[allow(dead_code)]
    pub fn param(mut self, name: impl Into<String>, ty: impl Into<String>) -> Self {
        self.params.push((name.into(), ty.into()));
        self
    }
    #[allow(dead_code)]
    pub fn returns(mut self, ty: impl Into<String>) -> Self {
        self.return_type = ty.into();
        self
    }
    #[allow(dead_code)]
    pub fn emit(&self) -> String {
        let tp_part = if self.type_params.is_empty() {
            String::new()
        } else {
            format!("comptime {}: type, ", self.type_params.join(", comptime "))
        };
        let params: Vec<String> = self
            .params
            .iter()
            .map(|(n, t)| format!("{}: {}", n, t))
            .collect();
        format!(
            "fn {}({}{}){} {{\n    // body\n}}",
            self.name,
            tp_part,
            params.join(", "),
            self.return_type
        )
    }
}
/// The Zig code generation backend.
pub struct ZigBackend;
impl ZigBackend {
    /// Create a new ZigBackend.
    pub fn new() -> Self {
        ZigBackend
    }
    /// Emit source code for a complete Zig module.
    pub fn emit_module(&self, module: &ZigModule) -> String {
        module.codegen()
    }
    /// Emit source code for a single Zig function.
    pub fn emit_fn(&self, f: &ZigFn) -> String {
        f.codegen()
    }
    /// Emit source code for a single Zig struct.
    pub fn emit_struct(&self, s: &ZigStruct) -> String {
        s.codegen()
    }
    /// Sanitize an OxiLean name for use as a Zig identifier.
    ///
    /// Replaces dots and other special characters with underscores,
    /// prepends `ox_` if the name starts with a digit or is a Zig keyword.
    pub fn compile_name(oxilean_name: &str) -> String {
        let sanitized: String = oxilean_name
            .chars()
            .map(|c| {
                if c.is_alphanumeric() || c == '_' {
                    c
                } else {
                    '_'
                }
            })
            .collect();
        let result = if sanitized.is_empty() {
            "ox_empty".to_string()
        } else if sanitized
            .chars()
            .next()
            .map(|c| c.is_ascii_digit())
            .unwrap_or(false)
        {
            format!("ox_{}", sanitized)
        } else {
            sanitized
        };
        let zig_keywords = [
            "align",
            "allowzero",
            "and",
            "anyframe",
            "anytype",
            "asm",
            "async",
            "await",
            "break",
            "callconv",
            "catch",
            "comptime",
            "const",
            "continue",
            "defer",
            "else",
            "enum",
            "errdefer",
            "error",
            "export",
            "extern",
            "fn",
            "for",
            "if",
            "inline",
            "noalias",
            "noinline",
            "nosuspend",
            "opaque",
            "or",
            "orelse",
            "packed",
            "pub",
            "resume",
            "return",
            "struct",
            "suspend",
            "switch",
            "test",
            "threadlocal",
            "try",
            "union",
            "unreachable",
            "usingnamespace",
            "var",
            "volatile",
            "while",
        ];
        if zig_keywords.contains(&result.as_str()) {
            format!("ox_{}", result)
        } else {
            result
        }
    }
}
/// Configuration for ZigExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ZigExtPassConfig {
    pub name: String,
    pub phase: ZigExtPassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}
impl ZigExtPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            phase: ZigExtPassPhase::Middle,
            enabled: true,
            max_iterations: 100,
            debug: 0,
            timeout_ms: None,
        }
    }
    #[allow(dead_code)]
    pub fn with_phase(mut self, phase: ZigExtPassPhase) -> Self {
        self.phase = phase;
        self
    }
    #[allow(dead_code)]
    pub fn with_max_iter(mut self, n: usize) -> Self {
        self.max_iterations = n;
        self
    }
    #[allow(dead_code)]
    pub fn with_debug(mut self, d: u32) -> Self {
        self.debug = d;
        self
    }
    #[allow(dead_code)]
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }
    #[allow(dead_code)]
    pub fn with_timeout(mut self, ms: u64) -> Self {
        self.timeout_ms = Some(ms);
        self
    }
    #[allow(dead_code)]
    pub fn is_debug_enabled(&self) -> bool {
        self.debug > 0
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ZigBuiltinFn {
    pub name: String,
    pub args: Vec<String>,
}
impl ZigBuiltinFn {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        ZigBuiltinFn {
            name: name.into(),
            args: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn arg(mut self, a: impl Into<String>) -> Self {
        self.args.push(a.into());
        self
    }
    #[allow(dead_code)]
    pub fn emit(&self) -> String {
        format!("@{}({})", self.name, self.args.join(", "))
    }
}
