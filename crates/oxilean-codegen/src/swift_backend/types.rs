//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::functions::OXILEAN_SWIFT_RUNTIME;

use super::functions::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SwiftDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
impl SwiftDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        SwiftDominatorTree {
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
/// A Swift struct declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct SwiftStructDecl {
    /// Struct name
    pub name: String,
    /// Fields (stored properties)
    pub fields: Vec<SwiftField>,
    /// Methods
    pub methods: Vec<SwiftFunc>,
    /// Protocol conformances
    pub conformances: Vec<SwiftConformance>,
    /// Whether `public`
    pub is_public: bool,
    /// Generic parameters
    pub generic_params: Vec<String>,
}
impl SwiftStructDecl {
    /// Create a new empty struct.
    pub fn new(name: impl Into<String>) -> Self {
        SwiftStructDecl {
            name: name.into(),
            fields: Vec::new(),
            methods: Vec::new(),
            conformances: Vec::new(),
            is_public: false,
            generic_params: Vec::new(),
        }
    }
    /// Emit as Swift source.
    pub fn codegen(&self) -> String {
        let vis = if self.is_public { "public " } else { "" };
        let generics = if self.generic_params.is_empty() {
            String::new()
        } else {
            format!("<{}>", self.generic_params.join(", "))
        };
        let conformances = if self.conformances.is_empty() {
            String::new()
        } else {
            format!(
                ": {}",
                self.conformances
                    .iter()
                    .map(|c| c.0.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        };
        let mut out = format!(
            "{}struct {}{}{} {{\n",
            vis, self.name, generics, conformances
        );
        for field in &self.fields {
            out += &field.codegen();
            out += "\n";
        }
        for method in &self.methods {
            for line in method.codegen().lines() {
                out += "    ";
                out += line;
                out += "\n";
            }
        }
        out += "}\n";
        out
    }
}
/// A Swift function (or method) declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct SwiftFunc {
    /// Function name
    pub name: String,
    /// Parameter list
    pub params: Vec<SwiftParam>,
    /// Return type (defaults to Void)
    pub return_type: SwiftType,
    /// Function body statements
    pub body: Vec<SwiftStmt>,
    /// Whether the function is `public`
    pub is_public: bool,
    /// Whether the function is `private`
    pub is_private: bool,
    /// Whether the function can `throw`
    pub throws: bool,
    /// Whether the function is `async`
    pub is_async: bool,
    /// Whether the function is `static`
    pub is_static: bool,
    /// Whether the function is `mutating`
    pub is_mutating: bool,
    /// Generic type parameters: `<T: Equatable, U>`
    pub generic_params: Vec<String>,
    /// Where clause
    pub where_clause: Option<String>,
}
impl SwiftFunc {
    /// Create a minimal function with the given name and return type.
    pub fn new(name: impl Into<String>, return_type: SwiftType) -> Self {
        SwiftFunc {
            name: name.into(),
            params: Vec::new(),
            return_type,
            body: Vec::new(),
            is_public: false,
            is_private: false,
            throws: false,
            is_async: false,
            is_static: false,
            is_mutating: false,
            generic_params: Vec::new(),
            where_clause: None,
        }
    }
    /// Emit the function as a Swift source string.
    pub fn codegen(&self) -> String {
        let mut out = String::new();
        if self.is_public {
            out += "public ";
        } else if self.is_private {
            out += "private ";
        }
        if self.is_static {
            out += "static ";
        }
        if self.is_mutating {
            out += "mutating ";
        }
        out += "func ";
        out += &self.name;
        if !self.generic_params.is_empty() {
            out += "<";
            out += &self.generic_params.join(", ");
            out += ">";
        }
        out += "(";
        for (i, p) in self.params.iter().enumerate() {
            if i > 0 {
                out += ", ";
            }
            out += &p.to_string();
        }
        out += ")";
        if self.is_async {
            out += " async";
        }
        if self.throws {
            out += " throws";
        }
        if self.return_type != SwiftType::SwiftVoid {
            out += &format!(" -> {}", self.return_type);
        }
        if let Some(ref wh) = self.where_clause {
            out += &format!(" where {}", wh);
        }
        out += " {\n";
        out += &emit_block(&self.body, 4);
        if !self.body.is_empty() {
            out += "\n";
        }
        out += "}\n";
        out
    }
}
/// A conformance / protocol conformance constraint.
#[derive(Debug, Clone, PartialEq)]
pub struct SwiftConformance(pub String);
#[allow(dead_code)]
pub struct SwiftPassRegistry {
    pub(super) configs: Vec<SwiftPassConfig>,
    pub(super) stats: std::collections::HashMap<String, SwiftPassStats>,
}
impl SwiftPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        SwiftPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: SwiftPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), SwiftPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&SwiftPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&SwiftPassStats> {
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
pub struct SwiftCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
/// A Swift source module (one `.swift` file or a logical unit).
#[derive(Debug, Clone, PartialEq)]
pub struct SwiftModule {
    /// Module/file name (without `.swift` extension)
    pub name: String,
    /// `import` statements
    pub imports: Vec<String>,
    /// Top-level type declarations
    pub types: Vec<SwiftTypeDecl>,
    /// Top-level function declarations
    pub funcs: Vec<SwiftFunc>,
    /// Extensions
    pub extensions: Vec<SwiftExtension>,
    /// Global `let`/`var` declarations emitted verbatim
    pub globals: Vec<SwiftStmt>,
}
impl SwiftModule {
    /// Create a new empty module.
    pub fn new(name: impl Into<String>) -> Self {
        SwiftModule {
            name: name.into(),
            imports: Vec::new(),
            types: Vec::new(),
            funcs: Vec::new(),
            extensions: Vec::new(),
            globals: Vec::new(),
        }
    }
    /// Add an import (deduplicating).
    pub fn add_import(&mut self, module: impl Into<String>) {
        let m = module.into();
        if !self.imports.contains(&m) {
            self.imports.push(m);
        }
    }
    /// Emit the complete Swift source for this module.
    pub fn codegen(&self) -> String {
        let mut out = String::new();
        out += &format!("// OxiLean-generated Swift module: {}\n", self.name);
        out += OXILEAN_SWIFT_RUNTIME;
        out += "\n";
        for imp in &self.imports {
            out += &format!("import {}\n", imp);
        }
        if !self.imports.is_empty() {
            out += "\n";
        }
        for g in &self.globals {
            out += &format!("{}\n", emit_stmt(g, 0));
        }
        if !self.globals.is_empty() {
            out += "\n";
        }
        for ty in &self.types {
            out += &ty.codegen();
            out += "\n";
        }
        for func in &self.funcs {
            out += &func.codegen();
            out += "\n";
        }
        for ext in &self.extensions {
            out += &ext.codegen();
            out += "\n";
        }
        out
    }
}
/// A `case` arm in a switch statement.
#[derive(Debug, Clone, PartialEq)]
pub struct SwiftCase {
    /// Pattern string (e.g. `".some(let x)"`, `"0"`, `"default"`)
    pub pattern: String,
    /// Body statements for this case
    pub body: Vec<SwiftStmt>,
}
#[allow(dead_code)]
pub struct SwiftConstantFoldingHelper;
impl SwiftConstantFoldingHelper {
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
/// Swift expression for code generation.
#[derive(Debug, Clone, PartialEq)]
pub enum SwiftExpr {
    /// A literal value: `42`, `"hello"`, `true`, `nil`
    SwiftLitExpr(SwiftLit),
    /// A variable or identifier: `x`, `myVar`, `MyType`
    SwiftVar(String),
    /// A function/method call: `f(a, b)`
    SwiftCall {
        /// The function or method being called
        callee: Box<SwiftExpr>,
        /// Labeled arguments: `(label, expr)` — label is empty for unlabeled
        args: Vec<(String, SwiftExpr)>,
    },
    /// Binary operator: `lhs + rhs`, `a == b`
    SwiftBinOp {
        op: String,
        lhs: Box<SwiftExpr>,
        rhs: Box<SwiftExpr>,
    },
    /// Member access: `obj.field`
    SwiftMember(Box<SwiftExpr>, String),
    /// Subscript: `arr[idx]`
    SwiftSubscript(Box<SwiftExpr>, Box<SwiftExpr>),
    /// Unary prefix operator: `!x`, `-n`
    SwiftUnary(String, Box<SwiftExpr>),
    /// Ternary conditional: `cond ? then : else`
    SwiftTernary(Box<SwiftExpr>, Box<SwiftExpr>, Box<SwiftExpr>),
    /// Closure expression: `{ params in body }`
    SwiftClosure {
        /// Parameter names with optional types
        params: Vec<(String, Option<SwiftType>)>,
        /// Return type annotation
        return_type: Option<SwiftType>,
        /// Body statements
        body: Vec<SwiftStmt>,
    },
    /// Switch expression (Swift 5.9+): `switch x { case .a: expr }`
    SwiftSwitchExpr {
        /// The scrutinee
        subject: Box<SwiftExpr>,
        /// Arms: `(pattern_str, result_expr)`
        arms: Vec<(String, SwiftExpr)>,
    },
    /// Optional chaining: `obj?.field`
    SwiftOptionalChain(Box<SwiftExpr>, String),
    /// Force unwrap: `expr!`
    SwiftForceUnwrap(Box<SwiftExpr>),
    /// Array literal: `[a, b, c]`
    SwiftArrayLit(Vec<SwiftExpr>),
    /// Dictionary literal: `[k1: v1, k2: v2]`
    SwiftDictLit(Vec<(SwiftExpr, SwiftExpr)>),
    /// Tuple literal: `(a, b)`
    SwiftTupleLit(Vec<SwiftExpr>),
    /// Type cast: `expr as Type`
    SwiftAs(Box<SwiftExpr>, SwiftType),
    /// Try expression: `try expr`
    SwiftTry(Box<SwiftExpr>),
    /// Await expression: `await expr`
    SwiftAwait(Box<SwiftExpr>),
    /// Self reference
    SwiftSelf,
    /// Super reference
    SwiftSuper,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SwiftWorklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
impl SwiftWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        SwiftWorklist {
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
/// Swift literal values.
#[derive(Debug, Clone, PartialEq)]
pub enum SwiftLit {
    /// Integer literal: `42`, `-7`
    Int(i64),
    /// Boolean literal: `true` or `false`
    Bool(bool),
    /// String literal: `"hello"`
    Str(String),
    /// `nil` literal
    Nil,
    /// Float/Double literal: `3.14`
    Float(f64),
}
/// A Swift class declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct SwiftClassDecl {
    /// Class name
    pub name: String,
    /// Superclass (if any)
    pub superclass: Option<String>,
    /// Fields
    pub fields: Vec<SwiftField>,
    /// Methods
    pub methods: Vec<SwiftFunc>,
    /// Protocol conformances
    pub conformances: Vec<SwiftConformance>,
    /// Whether `public`
    pub is_public: bool,
    /// Whether `final`
    pub is_final: bool,
    /// Generic parameters
    pub generic_params: Vec<String>,
}
impl SwiftClassDecl {
    /// Create a new class.
    pub fn new(name: impl Into<String>) -> Self {
        SwiftClassDecl {
            name: name.into(),
            superclass: None,
            fields: Vec::new(),
            methods: Vec::new(),
            conformances: Vec::new(),
            is_public: false,
            is_final: false,
            generic_params: Vec::new(),
        }
    }
    /// Emit as Swift source.
    pub fn codegen(&self) -> String {
        let vis = if self.is_public { "public " } else { "" };
        let final_ = if self.is_final { "final " } else { "" };
        let generics = if self.generic_params.is_empty() {
            String::new()
        } else {
            format!("<{}>", self.generic_params.join(", "))
        };
        let mut inherit = Vec::new();
        if let Some(ref sc) = self.superclass {
            inherit.push(sc.clone());
        }
        for c in &self.conformances {
            inherit.push(c.0.clone());
        }
        let inherit_str = if inherit.is_empty() {
            String::new()
        } else {
            format!(": {}", inherit.join(", "))
        };
        let mut out = format!(
            "{}{}class {}{}{} {{\n",
            vis, final_, self.name, generics, inherit_str
        );
        for field in &self.fields {
            out += &field.codegen();
            out += "\n";
        }
        for method in &self.methods {
            for line in method.codegen().lines() {
                out += "    ";
                out += line;
                out += "\n";
            }
        }
        out += "}\n";
        out
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SwiftLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
impl SwiftLivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        SwiftLivenessInfo {
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
/// Swift type representation for type-directed code generation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SwiftType {
    /// `Int` — platform-native signed integer
    SwiftInt,
    /// `Bool` — boolean
    SwiftBool,
    /// `String` — Unicode string
    SwiftString,
    /// `Double` — 64-bit floating-point
    SwiftDouble,
    /// `Float` — 32-bit floating-point
    SwiftFloat,
    /// `Void` — unit / no value
    SwiftVoid,
    /// `[T]` — array of T
    SwiftArray(Box<SwiftType>),
    /// `[K: V]` — dictionary
    SwiftDict(Box<SwiftType>, Box<SwiftType>),
    /// `T?` — optional
    SwiftOptional(Box<SwiftType>),
    /// `(A, B, ...)` — tuple
    SwiftTuple(Vec<SwiftType>),
    /// `(A, B) -> R` — function type
    SwiftFunc(Vec<SwiftType>, Box<SwiftType>),
    /// Named enum type
    SwiftEnum(String),
    /// Named struct type
    SwiftStruct(String),
    /// Named class type
    SwiftClass(String),
    /// Named protocol type
    SwiftProtocol(String),
    /// Generic type: `Array<T>`, `Result<T, E>`, etc.
    SwiftGeneric(String, Vec<SwiftType>),
    /// `Any` — existential any
    SwiftAny,
    /// `AnyObject` — class-constrained any
    SwiftAnyObject,
    /// `Never` — uninhabited type
    SwiftNever,
    /// Raw named type (user-supplied string)
    SwiftNamed(String),
}
/// An enum case declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct SwiftEnumCase {
    /// Case name
    pub name: String,
    /// Associated value types (empty for bare cases)
    pub associated: Vec<SwiftType>,
    /// Raw value (for raw-value enums)
    pub raw_value: Option<SwiftLit>,
}
impl SwiftEnumCase {
    /// A bare enum case with no associated value.
    pub fn bare(name: impl Into<String>) -> Self {
        SwiftEnumCase {
            name: name.into(),
            associated: Vec::new(),
            raw_value: None,
        }
    }
    /// An enum case with associated values.
    pub fn with_values(name: impl Into<String>, values: Vec<SwiftType>) -> Self {
        SwiftEnumCase {
            name: name.into(),
            associated: values,
            raw_value: None,
        }
    }
    /// Emit as Swift source.
    pub fn codegen(&self) -> String {
        let mut out = format!("    case {}", self.name);
        if !self.associated.is_empty() {
            out += "(";
            for (i, t) in self.associated.iter().enumerate() {
                if i > 0 {
                    out += ", ";
                }
                out += &t.to_string();
            }
            out += ")";
        }
        if let Some(ref rv) = self.raw_value {
            out += &format!(" = {}", rv);
        }
        out
    }
}
/// A field in a Swift struct or class.
#[derive(Debug, Clone, PartialEq)]
pub struct SwiftField {
    /// Field name
    pub name: String,
    /// Field type
    pub ty: SwiftType,
    /// Whether the field is `var` (mutable) vs `let` (immutable)
    pub mutable: bool,
    /// Whether the field is `public`
    pub is_public: bool,
    /// Default value expression
    pub default: Option<SwiftExpr>,
}
impl SwiftField {
    /// Create an immutable field.
    pub fn new_let(name: impl Into<String>, ty: SwiftType) -> Self {
        SwiftField {
            name: name.into(),
            ty,
            mutable: false,
            is_public: false,
            default: None,
        }
    }
    /// Create a mutable field.
    pub fn new_var(name: impl Into<String>, ty: SwiftType) -> Self {
        SwiftField {
            name: name.into(),
            ty,
            mutable: true,
            is_public: false,
            default: None,
        }
    }
    /// Emit as Swift source.
    pub fn codegen(&self) -> String {
        let vis = if self.is_public { "public " } else { "" };
        let kw = if self.mutable { "var" } else { "let" };
        let default = self
            .default
            .as_ref()
            .map(|v| format!(" = {}", v))
            .unwrap_or_default();
        format!("    {}{} {}: {}{}", vis, kw, self.name, self.ty, default)
    }
}
/// A parameter in a Swift function declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct SwiftParam {
    /// External label (use `_` for unlabeled)
    pub label: String,
    /// Internal parameter name
    pub name: String,
    /// Parameter type
    pub ty: SwiftType,
    /// Default value expression
    pub default: Option<SwiftExpr>,
    /// Whether the parameter is variadic (`...`)
    pub variadic: bool,
    /// Whether the parameter is `inout`
    pub inout: bool,
}
impl SwiftParam {
    /// Create a simple labeled parameter.
    pub fn new(name: impl Into<String>, ty: SwiftType) -> Self {
        SwiftParam {
            label: String::new(),
            name: name.into(),
            ty,
            default: None,
            variadic: false,
            inout: false,
        }
    }
    /// Create a parameter with separate external label and internal name.
    pub fn labeled(label: impl Into<String>, name: impl Into<String>, ty: SwiftType) -> Self {
        SwiftParam {
            label: label.into(),
            name: name.into(),
            ty,
            default: None,
            variadic: false,
            inout: false,
        }
    }
}
/// Swift statement for code generation.
#[derive(Debug, Clone, PartialEq)]
pub enum SwiftStmt {
    /// `let name: Type = expr`
    Let {
        name: String,
        ty: Option<SwiftType>,
        value: SwiftExpr,
    },
    /// `var name: Type = expr`
    Var {
        name: String,
        ty: Option<SwiftType>,
        value: Option<SwiftExpr>,
    },
    /// `target = expr`
    Assign { target: SwiftExpr, value: SwiftExpr },
    /// `return expr?`
    Return(Option<SwiftExpr>),
    /// `if cond { then } else { else }`
    If {
        cond: SwiftExpr,
        then_body: Vec<SwiftStmt>,
        else_body: Vec<SwiftStmt>,
    },
    /// `if let name = expr { ... } else { ... }`
    IfLet {
        name: String,
        value: SwiftExpr,
        then_body: Vec<SwiftStmt>,
        else_body: Vec<SwiftStmt>,
    },
    /// `guard cond else { ... }`
    Guard {
        cond: SwiftExpr,
        else_body: Vec<SwiftStmt>,
    },
    /// `switch subject { case ... }`
    Switch {
        subject: SwiftExpr,
        cases: Vec<SwiftCase>,
    },
    /// `for name in collection { body }`
    For {
        name: String,
        collection: SwiftExpr,
        body: Vec<SwiftStmt>,
    },
    /// `while cond { body }`
    While {
        cond: SwiftExpr,
        body: Vec<SwiftStmt>,
    },
    /// `throw expr`
    Throw(SwiftExpr),
    /// `break`
    Break,
    /// `continue`
    Continue,
    /// Bare expression statement
    ExprStmt(SwiftExpr),
    /// A raw block of statements: `{ stmts }`
    Block(Vec<SwiftStmt>),
    /// A raw string inserted verbatim (for runtime preamble, etc.)
    Raw(String),
}
/// Discriminated union of all Swift type declarations.
#[derive(Debug, Clone, PartialEq)]
pub enum SwiftTypeDecl {
    Enum(SwiftEnumDecl),
    Struct(SwiftStructDecl),
    Class(SwiftClassDecl),
}
impl SwiftTypeDecl {
    /// Emit as Swift source.
    pub fn codegen(&self) -> String {
        match self {
            SwiftTypeDecl::Enum(e) => e.codegen(),
            SwiftTypeDecl::Struct(s) => s.codegen(),
            SwiftTypeDecl::Class(c) => c.codegen(),
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct SwiftPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
impl SwiftPassStats {
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SwiftAnalysisCache {
    pub(super) entries: std::collections::HashMap<String, SwiftCacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
impl SwiftAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        SwiftAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&SwiftCacheEntry> {
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
            SwiftCacheEntry {
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
/// A Swift `extension` declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct SwiftExtension {
    /// The type being extended
    pub target: String,
    /// Additional conformances added by this extension
    pub conformances: Vec<SwiftConformance>,
    /// Methods added
    pub methods: Vec<SwiftFunc>,
    /// Where clause
    pub where_clause: Option<String>,
}
impl SwiftExtension {
    /// Create a new extension.
    pub fn new(target: impl Into<String>) -> Self {
        SwiftExtension {
            target: target.into(),
            conformances: Vec::new(),
            methods: Vec::new(),
            where_clause: None,
        }
    }
    /// Emit as Swift source.
    pub fn codegen(&self) -> String {
        let conformances = if self.conformances.is_empty() {
            String::new()
        } else {
            format!(
                ": {}",
                self.conformances
                    .iter()
                    .map(|c| c.0.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        };
        let where_str = self
            .where_clause
            .as_ref()
            .map(|w| format!(" where {}", w))
            .unwrap_or_default();
        let mut out = format!(
            "extension {}{}{} {{\n",
            self.target, conformances, where_str
        );
        for method in &self.methods {
            for line in method.codegen().lines() {
                out += "    ";
                out += line;
                out += "\n";
            }
        }
        out += "}\n";
        out
    }
}
/// Swift code generation backend.
///
/// Compiles LCNF declarations/expressions to valid Swift source code.
/// Entry points:
/// - [`SwiftBackend::compile_module`] — compile a complete `LcnfDecl` collection
/// - [`SwiftBackend::compile_decl`]   — compile a single `LcnfDecl`
/// - [`SwiftBackend::compile_expr`]   — compile an `LcnfExpr`
/// - [`SwiftBackend::mangle_name`]    — escape an OxiLean identifier for Swift
pub struct SwiftBackend {
    /// Fresh variable counter for temporaries
    pub(super) var_counter: u64,
    /// Whether to emit public declarations
    pub emit_public: bool,
    /// Whether to emit inline comments
    pub emit_comments: bool,
}
impl SwiftBackend {
    /// Create a new backend with default settings.
    pub fn new() -> Self {
        SwiftBackend {
            var_counter: 0,
            emit_public: true,
            emit_comments: true,
        }
    }
    /// Allocate a fresh temporary variable name.
    pub fn fresh_var(&mut self) -> String {
        let n = self.var_counter;
        self.var_counter += 1;
        format!("_ox{}", n)
    }
    /// Mangle an OxiLean name into a valid Swift identifier.
    ///
    /// Rules:
    /// 1. Empty input → `"ox_empty"`
    /// 2. Swift keywords → prefixed with `ox_`
    /// 3. Leading digit → prefixed with `ox_`
    /// 4. Non-alphanumeric / non-underscore characters → replaced with `_`
    pub fn mangle_name(name: &str) -> String {
        if name.is_empty() {
            return "ox_empty".to_string();
        }
        let sanitized: String = name
            .chars()
            .map(|c| {
                if c.is_alphanumeric() || c == '_' {
                    c
                } else {
                    '_'
                }
            })
            .collect();
        let sanitized = if sanitized.starts_with(|c: char| c.is_ascii_digit()) {
            format!("ox_{}", sanitized)
        } else {
            sanitized
        };
        if is_swift_keyword(&sanitized) {
            format!("ox_{}", sanitized)
        } else {
            sanitized
        }
    }
    /// Compile an LCNF argument to a Swift expression.
    pub fn compile_arg(&self, arg: &LcnfArg) -> SwiftExpr {
        match arg {
            LcnfArg::Var(id) => SwiftExpr::SwiftVar(format!("{}", id)),
            LcnfArg::Lit(lit) => self.compile_lit(lit),
            LcnfArg::Erased => SwiftExpr::SwiftMember(
                Box::new(SwiftExpr::SwiftVar("OxValue".to_string())),
                "erased".to_string(),
            ),
            LcnfArg::Type(_) => SwiftExpr::SwiftMember(
                Box::new(SwiftExpr::SwiftVar("OxValue".to_string())),
                "erased".to_string(),
            ),
        }
    }
    /// Compile an LCNF literal to a Swift expression.
    pub fn compile_lit(&self, lit: &LcnfLit) -> SwiftExpr {
        match lit {
            LcnfLit::Nat(n) => SwiftExpr::SwiftLitExpr(SwiftLit::Int(*n as i64)),
            LcnfLit::Str(s) => SwiftExpr::SwiftLitExpr(SwiftLit::Str(s.clone())),
        }
    }
    /// Compile an LCNF expression to a list of Swift statements.
    ///
    /// The final statement is always a `return`.
    pub fn compile_expr(&self, expr: &LcnfExpr) -> Vec<SwiftStmt> {
        match expr {
            LcnfExpr::Return(arg) => vec![SwiftStmt::Return(Some(self.compile_arg(arg)))],
            LcnfExpr::Unreachable => {
                vec![SwiftStmt::ExprStmt(SwiftExpr::SwiftCall {
                    callee: Box::new(SwiftExpr::SwiftVar("ox_unreachable".to_string())),
                    args: Vec::new(),
                })]
            }
            LcnfExpr::TailCall(func, args) => {
                let callee = Box::new(self.compile_arg(func));
                let call_args = args
                    .iter()
                    .map(|a| (String::new(), self.compile_arg(a)))
                    .collect();
                vec![SwiftStmt::Return(Some(SwiftExpr::SwiftCall {
                    callee,
                    args: call_args,
                }))]
            }
            LcnfExpr::Let {
                id, value, body, ..
            } => {
                let var_name = format!("{}", id);
                let rhs = self.compile_let_value(value);
                let mut stmts = vec![SwiftStmt::Let {
                    name: var_name,
                    ty: None,
                    value: rhs,
                }];
                stmts.extend(self.compile_expr(body));
                stmts
            }
            LcnfExpr::Case {
                scrutinee,
                alts,
                default,
                ..
            } => {
                let subject = SwiftExpr::SwiftVar(format!("{}", scrutinee));
                let mut cases: Vec<SwiftCase> = alts
                    .iter()
                    .map(|alt| {
                        let pattern = format!(".ctor(tag: {}, _)", alt.ctor_tag);
                        let body = self.compile_expr(&alt.body);
                        SwiftCase { pattern, body }
                    })
                    .collect();
                if let Some(def) = default {
                    let def_body = self.compile_expr(def);
                    cases.push(SwiftCase {
                        pattern: "default".to_string(),
                        body: def_body,
                    });
                } else {
                    cases.push(SwiftCase {
                        pattern: "default".to_string(),
                        body: vec![SwiftStmt::ExprStmt(SwiftExpr::SwiftCall {
                            callee: Box::new(SwiftExpr::SwiftVar("ox_unreachable".to_string())),
                            args: Vec::new(),
                        })],
                    });
                }
                vec![SwiftStmt::Switch { subject, cases }]
            }
        }
    }
    /// Compile a let-value to a Swift expression.
    pub fn compile_let_value(&self, value: &LcnfLetValue) -> SwiftExpr {
        match value {
            LcnfLetValue::Lit(lit) => self.compile_lit(lit),
            LcnfLetValue::Erased => SwiftExpr::SwiftMember(
                Box::new(SwiftExpr::SwiftVar("OxValue".to_string())),
                "erased".to_string(),
            ),
            LcnfLetValue::FVar(id) => SwiftExpr::SwiftVar(format!("{}", id)),
            LcnfLetValue::App(func, args) => {
                let callee = Box::new(self.compile_arg(func));
                let call_args = args
                    .iter()
                    .map(|a| (String::new(), self.compile_arg(a)))
                    .collect();
                SwiftExpr::SwiftCall {
                    callee,
                    args: call_args,
                }
            }
            LcnfLetValue::Ctor(name, tag, args) => {
                let tag_pair = (
                    "tag".to_string(),
                    SwiftExpr::SwiftLitExpr(SwiftLit::Int(*tag as i64)),
                );
                let field_exprs: Vec<SwiftExpr> =
                    args.iter().map(|a| self.compile_arg(a)).collect();
                let fields_array = SwiftExpr::SwiftArrayLit(field_exprs);
                let _ = name;
                SwiftExpr::SwiftCall {
                    callee: Box::new(SwiftExpr::SwiftMember(
                        Box::new(SwiftExpr::SwiftVar("OxValue".to_string())),
                        "ctor".to_string(),
                    )),
                    args: vec![tag_pair, ("fields".to_string(), fields_array)],
                }
            }
            LcnfLetValue::Proj(_, idx, var) => {
                let base = SwiftExpr::SwiftMember(
                    Box::new(SwiftExpr::SwiftVar(format!("{}", var))),
                    "fields".to_string(),
                );
                SwiftExpr::SwiftSubscript(
                    Box::new(base),
                    Box::new(SwiftExpr::SwiftLitExpr(SwiftLit::Int(*idx as i64))),
                )
            }
            LcnfLetValue::Reset(var) => SwiftExpr::SwiftVar(format!("{}", var)),
            LcnfLetValue::Reuse(_, name, tag, args) => {
                let tag_pair = (
                    "tag".to_string(),
                    SwiftExpr::SwiftLitExpr(SwiftLit::Int(*tag as i64)),
                );
                let field_exprs: Vec<SwiftExpr> =
                    args.iter().map(|a| self.compile_arg(a)).collect();
                let fields_array = SwiftExpr::SwiftArrayLit(field_exprs);
                let _ = name;
                SwiftExpr::SwiftCall {
                    callee: Box::new(SwiftExpr::SwiftMember(
                        Box::new(SwiftExpr::SwiftVar("OxValue".to_string())),
                        "ctor".to_string(),
                    )),
                    args: vec![tag_pair, ("fields".to_string(), fields_array)],
                }
            }
        }
    }
    /// Compile a single LCNF function declaration to a [`SwiftFunc`].
    pub fn compile_decl(&self, decl: &LcnfFunDecl) -> SwiftFunc {
        let safe_name = Self::mangle_name(&decl.name.to_string());
        let params: Vec<SwiftParam> = decl
            .params
            .iter()
            .map(|p| {
                let pname = Self::mangle_name(&p.name);
                SwiftParam {
                    label: "_".to_string(),
                    name: pname,
                    ty: self.compile_lcnf_type(&p.ty),
                    default: None,
                    variadic: false,
                    inout: false,
                }
            })
            .collect();
        let body = self.compile_expr(&decl.body);
        let mut func = SwiftFunc::new(safe_name, SwiftType::SwiftNamed("OxValue".to_string()));
        func.params = params;
        func.body = body;
        func.is_public = self.emit_public;
        func
    }
    /// Map an LCNF type to the closest Swift type.
    pub fn compile_lcnf_type(&self, ty: &LcnfType) -> SwiftType {
        match ty {
            LcnfType::Nat | LcnfType::Unit | LcnfType::Erased | LcnfType::Irrelevant => {
                SwiftType::SwiftNamed("OxValue".to_string())
            }
            LcnfType::LcnfString => SwiftType::SwiftNamed("OxValue".to_string()),
            LcnfType::Object => SwiftType::SwiftNamed("OxValue".to_string()),
            LcnfType::Var(_) => SwiftType::SwiftNamed("OxValue".to_string()),
            LcnfType::Fun(params, ret) => {
                let p: Vec<SwiftType> = params.iter().map(|p| self.compile_lcnf_type(p)).collect();
                let r = Box::new(self.compile_lcnf_type(ret));
                SwiftType::SwiftFunc(p, r)
            }
            LcnfType::Ctor(_, _) => SwiftType::SwiftNamed("OxValue".to_string()),
        }
    }
    /// Compile a collection of LCNF declarations into a [`SwiftModule`].
    pub fn compile_module(&self, name: &str, decls: &[LcnfFunDecl]) -> SwiftModule {
        let mut module = SwiftModule::new(name);
        module.add_import("Foundation");
        for decl in decls {
            let func = self.compile_decl(decl);
            module.funcs.push(func);
        }
        module
    }
    /// Emit a [`SwiftModule`] to a Swift source string.
    pub fn emit_module(&self, module: &SwiftModule) -> String {
        module.codegen()
    }
    /// Emit a single [`SwiftFunc`] to a Swift source string.
    pub fn emit_func(&self, func: &SwiftFunc) -> String {
        func.codegen()
    }
    /// Emit a single [`SwiftTypeDecl`] to a Swift source string.
    pub fn emit_type_decl(&self, decl: &SwiftTypeDecl) -> String {
        decl.codegen()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SwiftPassConfig {
    pub phase: SwiftPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
impl SwiftPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: SwiftPassPhase) -> Self {
        SwiftPassConfig {
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
/// A Swift enum declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct SwiftEnumDecl {
    /// Enum name
    pub name: String,
    /// Enum cases
    pub cases: Vec<SwiftEnumCase>,
    /// Methods defined on this enum
    pub methods: Vec<SwiftFunc>,
    /// Protocol conformances / inheritance list
    pub conformances: Vec<SwiftConformance>,
    /// Whether `public`
    pub is_public: bool,
    /// Generic parameters
    pub generic_params: Vec<String>,
}
impl SwiftEnumDecl {
    /// Create a new empty enum.
    pub fn new(name: impl Into<String>) -> Self {
        SwiftEnumDecl {
            name: name.into(),
            cases: Vec::new(),
            methods: Vec::new(),
            conformances: Vec::new(),
            is_public: false,
            generic_params: Vec::new(),
        }
    }
    /// Emit as Swift source.
    pub fn codegen(&self) -> String {
        let vis = if self.is_public { "public " } else { "" };
        let generics = if self.generic_params.is_empty() {
            String::new()
        } else {
            format!("<{}>", self.generic_params.join(", "))
        };
        let conformances = if self.conformances.is_empty() {
            String::new()
        } else {
            format!(
                ": {}",
                self.conformances
                    .iter()
                    .map(|c| c.0.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        };
        let mut out = format!("{}enum {}{}{} {{\n", vis, self.name, generics, conformances);
        for case in &self.cases {
            out += &case.codegen();
            out += "\n";
        }
        for method in &self.methods {
            for line in method.codegen().lines() {
                out += "    ";
                out += line;
                out += "\n";
            }
        }
        out += "}\n";
        out
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum SwiftPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl SwiftPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            SwiftPassPhase::Analysis => "analysis",
            SwiftPassPhase::Transformation => "transformation",
            SwiftPassPhase::Verification => "verification",
            SwiftPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(
            self,
            SwiftPassPhase::Transformation | SwiftPassPhase::Cleanup
        )
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SwiftDepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
}
impl SwiftDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        SwiftDepGraph {
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
