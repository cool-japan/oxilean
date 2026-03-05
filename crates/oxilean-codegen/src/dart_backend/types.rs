//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use std::collections::{HashMap, HashSet};

use super::functions::DART_KEYWORDS;
use super::functions::*;

/// Dart expression AST.
#[derive(Debug, Clone, PartialEq)]
pub enum DartExpr {
    /// Literal value
    Lit(DartLit),
    /// Variable reference
    Var(String),
    /// Property access: `expr.field`
    Field(Box<DartExpr>, String),
    /// Method call: `expr.method(args)`
    MethodCall(Box<DartExpr>, String, Vec<DartExpr>),
    /// Static / free function call: `name(args)`
    Call(Box<DartExpr>, Vec<DartExpr>),
    /// Constructor call: `ClassName(args)` or `ClassName.named(args)`
    New(String, Option<String>, Vec<DartExpr>),
    /// List literal: `[e1, e2, ...]`
    ListLit(Vec<DartExpr>),
    /// Map literal: `{k1: v1, k2: v2, ...}`
    MapLit(Vec<(DartExpr, DartExpr)>),
    /// Set literal: `{e1, e2, ...}`
    SetLit(Vec<DartExpr>),
    /// Anonymous function: `(params) { body }` or `(params) => expr`
    Lambda(Vec<(DartType, String)>, Box<DartExpr>),
    /// Arrow function: `(params) => expr`
    Arrow(Vec<(DartType, String)>, Box<DartExpr>),
    /// Binary operation: `left op right`
    BinOp(Box<DartExpr>, String, Box<DartExpr>),
    /// Unary operation: `op expr`
    UnaryOp(String, Box<DartExpr>),
    /// Conditional / ternary: `cond ? then : else`
    Ternary(Box<DartExpr>, Box<DartExpr>, Box<DartExpr>),
    /// Null-aware access: `expr?.field`
    NullAware(Box<DartExpr>, String),
    /// Null coalescing: `expr ?? fallback`
    NullCoalesce(Box<DartExpr>, Box<DartExpr>),
    /// Cascade: `expr..method(args)`
    Cascade(Box<DartExpr>, String, Vec<DartExpr>),
    /// `await expr`
    Await(Box<DartExpr>),
    /// Type cast: `expr as Type`
    As(Box<DartExpr>, DartType),
    /// Type check: `expr is Type`
    Is(Box<DartExpr>, DartType),
    /// `throw expr`
    Throw(Box<DartExpr>),
    /// Spread in list/map: `...expr`
    Spread(Box<DartExpr>),
    /// Index access: `expr[index]`
    Index(Box<DartExpr>, Box<DartExpr>),
    /// Raw Dart snippet (for runtime helpers)
    Raw(String),
}
/// A Dart `enum` declaration (enhanced enums with members).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DartEnum {
    pub name: String,
    pub variants: Vec<DartEnumVariant>,
    pub implements: Vec<DartType>,
    pub doc: Option<String>,
}
#[allow(dead_code)]
impl DartEnum {
    pub fn new(name: impl Into<String>) -> Self {
        DartEnum {
            name: name.into(),
            variants: vec![],
            implements: vec![],
            doc: None,
        }
    }
    pub fn add_variant(&mut self, v: DartEnumVariant) {
        self.variants.push(v);
    }
    pub fn emit(&self) -> String {
        let mut out = String::new();
        if let Some(doc) = &self.doc {
            out.push_str(&format!("/// {}\n", doc));
        }
        let impl_part = if self.implements.is_empty() {
            String::new()
        } else {
            let impls: Vec<String> = self.implements.iter().map(|t| t.to_string()).collect();
            format!(" implements {}", impls.join(", "))
        };
        out.push_str(&format!("enum {}{} {{\n", self.name, impl_part));
        for (i, variant) in self.variants.iter().enumerate() {
            let comma = if i + 1 < self.variants.len() {
                ","
            } else {
                ";"
            };
            if let Some(doc) = &variant.doc {
                out.push_str(&format!("  /// {}\n", doc));
            }
            out.push_str(&format!("  {}{}\n", variant.name, comma));
        }
        out.push_str("}\n");
        out
    }
}
/// Dart literal values.
#[derive(Debug, Clone, PartialEq)]
pub enum DartLit {
    /// Integer literal: `0`, `42`
    Int(i64),
    /// Double literal: `3.14`
    Double(f64),
    /// Boolean literal: `true` or `false`
    Bool(bool),
    /// String literal: `'hello'`
    Str(String),
    /// `null`
    Null,
}
/// A Dart import directive.
#[derive(Debug, Clone)]
pub struct DartImport {
    pub uri: String,
    pub as_prefix: Option<String>,
    pub show: Vec<String>,
    pub hide: Vec<String>,
    pub is_deferred: bool,
}
impl DartImport {
    pub fn simple(uri: impl Into<String>) -> Self {
        DartImport {
            uri: uri.into(),
            as_prefix: None,
            show: Vec::new(),
            hide: Vec::new(),
            is_deferred: false,
        }
    }
    pub fn with_prefix(uri: impl Into<String>, prefix: impl Into<String>) -> Self {
        DartImport {
            uri: uri.into(),
            as_prefix: Some(prefix.into()),
            show: Vec::new(),
            hide: Vec::new(),
            is_deferred: false,
        }
    }
    /// Emit the import statement as Dart source.
    pub fn emit(&self) -> String {
        let mut out = format!("import '{}'", self.uri);
        if self.is_deferred {
            out.push_str(" deferred");
        }
        if let Some(prefix) = &self.as_prefix {
            out.push_str(&format!(" as {}", prefix));
        }
        if !self.show.is_empty() {
            out.push_str(&format!(" show {}", self.show.join(", ")));
        }
        if !self.hide.is_empty() {
            out.push_str(&format!(" hide {}", self.hide.join(", ")));
        }
        out.push_str(";\n");
        out
    }
}
/// A Dart class declaration.
#[derive(Debug, Clone)]
pub struct DartClass {
    pub name: String,
    pub type_params: Vec<String>,
    pub extends: Option<String>,
    pub implements: Vec<String>,
    pub mixins: Vec<String>,
    pub fields: Vec<DartField>,
    pub constructors: Vec<DartFunction>,
    pub methods: Vec<DartFunction>,
    pub is_abstract: bool,
    pub doc: Option<String>,
}
impl DartClass {
    pub fn new(name: impl Into<String>) -> Self {
        DartClass {
            name: name.into(),
            type_params: Vec::new(),
            extends: None,
            implements: Vec::new(),
            mixins: Vec::new(),
            fields: Vec::new(),
            constructors: Vec::new(),
            methods: Vec::new(),
            is_abstract: false,
            doc: None,
        }
    }
}
/// A Dart extension method block.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DartExtension {
    pub name: Option<String>,
    pub on_type: DartType,
    pub methods: Vec<DartFunction>,
    pub getters: Vec<DartFunction>,
}
#[allow(dead_code)]
impl DartExtension {
    pub fn new(on_type: DartType) -> Self {
        DartExtension {
            name: None,
            on_type,
            methods: vec![],
            getters: vec![],
        }
    }
    pub fn named(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn add_method(&mut self, m: DartFunction) {
        self.methods.push(m);
    }
    pub fn add_getter(&mut self, g: DartFunction) {
        self.getters.push(g);
    }
    pub fn emit(&self, backend: &DartBackend, indent_level: usize) -> String {
        let name_part = self
            .name
            .as_deref()
            .map(|n| format!(" {}", n))
            .unwrap_or_default();
        let pad = "    ".repeat(indent_level);
        let mut out = format!("{}extension{} on {} {{\n", pad, name_part, self.on_type);
        for g in &self.getters {
            out.push_str(&backend.emit_getter(g, indent_level + 1));
        }
        for m in &self.methods {
            out.push_str(&backend.emit_function(m, indent_level + 1));
        }
        out.push_str(&format!("{}}}\n", pad));
        out
    }
}
/// A single enum variant with optional associated values.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DartEnumVariant {
    pub name: String,
    pub values: Vec<DartExpr>,
    pub doc: Option<String>,
}
#[allow(dead_code)]
impl DartEnumVariant {
    pub fn simple(name: impl Into<String>) -> Self {
        DartEnumVariant {
            name: name.into(),
            values: vec![],
            doc: None,
        }
    }
}
/// A Dart `mixin` declaration.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DartMixin {
    pub name: String,
    pub on_types: Vec<DartType>,
    pub fields: Vec<DartField>,
    pub methods: Vec<DartFunction>,
    pub doc: Option<String>,
}
#[allow(dead_code)]
impl DartMixin {
    pub fn new(name: impl Into<String>) -> Self {
        DartMixin {
            name: name.into(),
            on_types: vec![],
            fields: vec![],
            methods: vec![],
            doc: None,
        }
    }
    pub fn with_on(mut self, ty: DartType) -> Self {
        self.on_types.push(ty);
        self
    }
    pub fn with_doc(mut self, doc: impl Into<String>) -> Self {
        self.doc = Some(doc.into());
        self
    }
    pub fn emit(&self, backend: &DartBackend, indent_level: usize) -> String {
        let pad = "    ".repeat(indent_level);
        let mut out = String::new();
        if let Some(doc) = &self.doc {
            out.push_str(&format!("{}/// {}\n", pad, doc));
        }
        let on_part = if self.on_types.is_empty() {
            String::new()
        } else {
            let types: Vec<String> = self.on_types.iter().map(|t| t.to_string()).collect();
            format!(" on {}", types.join(", "))
        };
        out.push_str(&format!("{}mixin {}{} {{\n", pad, self.name, on_part));
        for field in &self.fields {
            out.push_str(&format!(
                "{}{}\n",
                "    ".repeat(indent_level + 1),
                emit_dart_field(field)
            ));
        }
        for method in &self.methods {
            out.push_str(&backend.emit_function(method, indent_level + 1));
        }
        out.push_str(&format!("{}}}\n", pad));
        out
    }
}
/// A Dart file/module (compilation unit).
#[derive(Debug, Clone)]
pub struct DartModule {
    pub imports: Vec<DartImport>,
    pub exports: Vec<String>,
    pub classes: Vec<DartClass>,
    pub functions: Vec<DartFunction>,
    pub globals: Vec<(DartType, String, DartExpr)>,
    pub part_of: Option<String>,
}
impl DartModule {
    pub fn new() -> Self {
        DartModule {
            imports: Vec::new(),
            exports: Vec::new(),
            classes: Vec::new(),
            functions: Vec::new(),
            globals: Vec::new(),
            part_of: None,
        }
    }
}
/// A Dart top-level or class function/method.
#[derive(Debug, Clone)]
pub struct DartFunction {
    pub name: String,
    pub return_type: DartType,
    pub params: Vec<DartParam>,
    pub body: Vec<DartStmt>,
    pub is_async: bool,
    pub is_static: bool,
    pub is_abstract: bool,
    pub type_params: Vec<String>,
    pub doc: Option<String>,
}
impl DartFunction {
    pub fn new(name: impl Into<String>, return_type: DartType) -> Self {
        DartFunction {
            name: name.into(),
            return_type,
            params: Vec::new(),
            body: Vec::new(),
            is_async: false,
            is_static: false,
            is_abstract: false,
            type_params: Vec::new(),
            doc: None,
        }
    }
}
/// A Dart annotation (metadata).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum DartAnnotation {
    /// `@override`
    Override,
    /// `@deprecated`
    Deprecated,
    /// `@visibleForTesting`
    VisibleForTesting,
    /// `@immutable`
    Immutable,
    /// `@sealed`
    Sealed,
    /// Custom annotation with optional arguments.
    Custom(String, Vec<String>),
}
/// A parameter in a Dart function/method.
#[derive(Debug, Clone, PartialEq)]
pub struct DartParam {
    pub ty: DartType,
    pub name: String,
    pub is_named: bool,
    pub is_required: bool,
    pub default_value: Option<DartExpr>,
}
impl DartParam {
    pub fn positional(ty: DartType, name: impl Into<String>) -> Self {
        DartParam {
            ty,
            name: name.into(),
            is_named: false,
            is_required: true,
            default_value: None,
        }
    }
    pub fn named_required(ty: DartType, name: impl Into<String>) -> Self {
        DartParam {
            ty,
            name: name.into(),
            is_named: true,
            is_required: true,
            default_value: None,
        }
    }
    pub fn named_optional(ty: DartType, name: impl Into<String>, default: DartExpr) -> Self {
        DartParam {
            ty,
            name: name.into(),
            is_named: true,
            is_required: false,
            default_value: Some(default),
        }
    }
}
/// Dart code generation backend.
pub struct DartBackend {
    /// Counter for generating unique variable names.
    pub(super) var_counter: u64,
    /// Dart keywords that must be escaped.
    pub(super) keywords: HashSet<&'static str>,
    /// Map from mangled name → original name.
    pub(super) name_cache: HashMap<String, String>,
    /// Indentation step (spaces).
    pub(super) indent_width: usize,
}
impl DartBackend {
    pub fn new() -> Self {
        let mut keywords = HashSet::new();
        for kw in DART_KEYWORDS {
            keywords.insert(*kw);
        }
        DartBackend {
            var_counter: 0,
            keywords,
            name_cache: HashMap::new(),
            indent_width: 2,
        }
    }
    /// Generate a fresh local variable name.
    pub fn fresh_var(&mut self) -> String {
        let id = self.var_counter;
        self.var_counter += 1;
        format!("_v{}", id)
    }
    /// Mangle an OxiLean identifier to a valid Dart identifier.
    pub fn mangle_name(&mut self, name: &str) -> String {
        if let Some(cached) = self.name_cache.get(name) {
            return cached.clone();
        }
        let mangled = mangle_dart_ident(name, &self.keywords);
        self.name_cache.insert(name.to_string(), mangled.clone());
        mangled
    }
    /// Emit a complete Dart module (file) as a String.
    pub fn emit_module(&mut self, module: &DartModule) -> String {
        let mut out = String::new();
        if let Some(part) = &module.part_of {
            out.push_str(&format!("part of '{}';\n\n", part));
        }
        for import in &module.imports {
            out.push_str(&self.emit_import(import));
        }
        if !module.imports.is_empty() {
            out.push('\n');
        }
        for exp in &module.exports {
            out.push_str(&format!("export '{}';\n", exp));
        }
        if !module.exports.is_empty() {
            out.push('\n');
        }
        for (ty, name, init) in &module.globals {
            out.push_str(&format!("final {} {} = {};\n", ty, name, init));
        }
        if !module.globals.is_empty() {
            out.push('\n');
        }
        for func in &module.functions {
            out.push_str(&self.emit_function(func, 0));
            out.push('\n');
        }
        for class in &module.classes {
            out.push_str(&self.emit_class(class, 0));
            out.push('\n');
        }
        out
    }
    /// Emit a `DartImport`.
    pub fn emit_import(&self, import: &DartImport) -> String {
        let mut line = format!("import '{}'", import.uri);
        if import.is_deferred {
            line.push_str(" deferred");
        }
        if let Some(prefix) = &import.as_prefix {
            line.push_str(&format!(" as {}", prefix));
        }
        if !import.show.is_empty() {
            line.push_str(&format!(" show {}", import.show.join(", ")));
        }
        if !import.hide.is_empty() {
            line.push_str(&format!(" hide {}", import.hide.join(", ")));
        }
        line.push_str(";\n");
        line
    }
    /// Emit a Dart class declaration.
    pub fn emit_class(&self, class: &DartClass, depth: usize) -> String {
        let indent = self.indent(depth);
        let mut out = String::new();
        if let Some(doc) = &class.doc {
            for line in doc.lines() {
                out.push_str(&format!("{}/// {}\n", indent, line));
            }
        }
        if class.is_abstract {
            out.push_str(&format!("{}abstract ", indent));
        } else {
            out.push_str(&indent);
        }
        out.push_str("class ");
        out.push_str(&class.name);
        if !class.type_params.is_empty() {
            out.push('<');
            out.push_str(&class.type_params.join(", "));
            out.push('>');
        }
        if let Some(ext) = &class.extends {
            out.push_str(&format!(" extends {}", ext));
        }
        if !class.mixins.is_empty() {
            out.push_str(&format!(" with {}", class.mixins.join(", ")));
        }
        if !class.implements.is_empty() {
            out.push_str(&format!(" implements {}", class.implements.join(", ")));
        }
        out.push_str(" {\n");
        let inner = self.indent(depth + 1);
        for field in &class.fields {
            if let Some(doc) = &field.doc {
                out.push_str(&format!("{}/// {}\n", inner, doc));
            }
            let mut modifiers = String::new();
            if field.is_static {
                modifiers.push_str("static ");
            }
            if field.is_final {
                modifiers.push_str("final ");
            }
            if field.is_late {
                modifiers.push_str("late ");
            }
            if let Some(init) = &field.default_value {
                out.push_str(&format!(
                    "{}{}{} {} = {};\n",
                    inner, modifiers, field.ty, field.name, init
                ));
            } else {
                out.push_str(&format!(
                    "{}{}{} {};\n",
                    inner, modifiers, field.ty, field.name
                ));
            }
        }
        if !class.fields.is_empty() && (!class.constructors.is_empty() || !class.methods.is_empty())
        {
            out.push('\n');
        }
        for ctor in &class.constructors {
            out.push_str(&self.emit_constructor(ctor, &class.name, depth + 1));
            out.push('\n');
        }
        for method in &class.methods {
            out.push_str(&self.emit_function(method, depth + 1));
            out.push('\n');
        }
        out.push_str(&format!("{}}}\n", indent));
        out
    }
    /// Emit a Dart constructor (uses class name, not return type).
    pub fn emit_constructor(&self, ctor: &DartFunction, class_name: &str, depth: usize) -> String {
        let indent = self.indent(depth);
        let mut out = String::new();
        if let Some(doc) = &ctor.doc {
            out.push_str(&format!("{}/// {}\n", indent, doc));
        }
        if ctor.is_static {
            out.push_str(&format!("{}static ", indent));
        } else {
            out.push_str(&indent);
        }
        let ctor_name = if ctor.name.is_empty() {
            class_name.to_string()
        } else {
            format!("{}.{}", class_name, ctor.name)
        };
        out.push_str(&ctor_name);
        out.push('(');
        out.push_str(&self.emit_params(&ctor.params));
        out.push(')');
        if ctor.is_abstract {
            out.push_str(";\n");
        } else {
            out.push_str(" {\n");
            for stmt in &ctor.body {
                out.push_str(&self.emit_stmt(stmt, depth + 1));
            }
            out.push_str(&format!("{}}}\n", indent));
        }
        out
    }
    /// Emit a Dart function or method.
    pub fn emit_function(&self, func: &DartFunction, depth: usize) -> String {
        let indent = self.indent(depth);
        let mut out = String::new();
        if let Some(doc) = &func.doc {
            for line in doc.lines() {
                out.push_str(&format!("{}/// {}\n", indent, line));
            }
        }
        if func.is_static {
            out.push_str(&format!("{}static ", indent));
        } else {
            out.push_str(&indent);
        }
        let async_suffix = if func.is_async { " async" } else { "" };
        let ret_ty = if func.is_async {
            match &func.return_type {
                DartType::DtFuture(_) => format!("{}", func.return_type),
                other => format!("Future<{}>", other),
            }
        } else {
            format!("{}", func.return_type)
        };
        let type_params_str = if func.type_params.is_empty() {
            String::new()
        } else {
            format!("<{}>", func.type_params.join(", "))
        };
        out.push_str(&format!(
            "{} {}{}({}){}",
            ret_ty,
            func.name,
            type_params_str,
            self.emit_params(&func.params),
            async_suffix,
        ));
        if func.is_abstract {
            out.push_str(";\n");
        } else {
            out.push_str(" {\n");
            for stmt in &func.body {
                out.push_str(&self.emit_stmt(stmt, depth + 1));
            }
            out.push_str(&format!("{}}}\n", indent));
        }
        out
    }
    /// Emit a Dart getter (a function with `get` keyword, no params).
    pub fn emit_getter(&self, func: &DartFunction, depth: usize) -> String {
        let indent = self.indent(depth);
        let mut out = String::new();
        if let Some(doc) = &func.doc {
            for line in doc.lines() {
                out.push_str(&format!("{}/// {}\n", indent, line));
            }
        }
        if func.is_static {
            out.push_str(&format!("{}static ", indent));
        } else {
            out.push_str(&indent);
        }
        out.push_str(&format!("{} get {}", func.return_type, func.name));
        if func.is_abstract {
            out.push_str(";\n");
        } else {
            out.push_str(" {\n");
            for stmt in &func.body {
                out.push_str(&self.emit_stmt(stmt, depth + 1));
            }
            out.push_str(&format!("{}}}\n", indent));
        }
        out
    }
    /// Emit a parameter list.
    pub fn emit_params(&self, params: &[DartParam]) -> String {
        let positional: Vec<&DartParam> = params.iter().filter(|p| !p.is_named).collect();
        let named: Vec<&DartParam> = params.iter().filter(|p| p.is_named).collect();
        let mut parts: Vec<String> = positional
            .iter()
            .map(|p| format!("{} {}", p.ty, p.name))
            .collect();
        if !named.is_empty() {
            let named_parts: Vec<String> = named
                .iter()
                .map(|p| {
                    let req = if p.is_required { "required " } else { "" };
                    if let Some(def) = &p.default_value {
                        format!("{}{} {} = {}", req, p.ty, p.name, def)
                    } else {
                        format!("{}{} {}", req, p.ty, p.name)
                    }
                })
                .collect();
            parts.push(format!("{{{}}}", named_parts.join(", ")));
        }
        parts.join(", ")
    }
    /// Emit a Dart statement with proper indentation.
    pub fn emit_stmt(&self, stmt: &DartStmt, depth: usize) -> String {
        let indent = self.indent(depth);
        match stmt {
            DartStmt::VarDecl(ty, name, init) => {
                format!("{}{} {} = {};\n", indent, ty, name, init)
            }
            DartStmt::VarInferred(name, init) => {
                format!("{}var {} = {};\n", indent, name, init)
            }
            DartStmt::FinalDecl(ty, name, init) => {
                format!("{}final {} {} = {};\n", indent, ty, name, init)
            }
            DartStmt::ConstDecl(ty, name, init) => {
                format!("{}const {} {} = {};\n", indent, ty, name, init)
            }
            DartStmt::Assign(name, val) => format!("{}{} = {};\n", indent, name, val),
            DartStmt::FieldAssign(obj, field, val) => {
                format!("{}{}.{} = {};\n", indent, obj, field, val)
            }
            DartStmt::IndexAssign(obj, idx, val) => {
                format!("{}{}[{}] = {};\n", indent, obj, idx, val)
            }
            DartStmt::Return(None) => format!("{}return;\n", indent),
            DartStmt::Return(Some(expr)) => format!("{}return {};\n", indent, expr),
            DartStmt::Expr(expr) => format!("{}{};\n", indent, expr),
            DartStmt::If(cond, then, else_) => {
                let mut out = format!("{}if ({}) {{\n", indent, cond);
                for s in then {
                    out.push_str(&self.emit_stmt(s, depth + 1));
                }
                if !else_.is_empty() {
                    out.push_str(&format!("{}}} else {{\n", indent));
                    for s in else_ {
                        out.push_str(&self.emit_stmt(s, depth + 1));
                    }
                }
                out.push_str(&format!("{}}}\n", indent));
                out
            }
            DartStmt::While(cond, body) => {
                let mut out = format!("{}while ({}) {{\n", indent, cond);
                for s in body {
                    out.push_str(&self.emit_stmt(s, depth + 1));
                }
                out.push_str(&format!("{}}}\n", indent));
                out
            }
            DartStmt::DoWhile(body, cond) => {
                let mut out = format!("{}do {{\n", indent);
                for s in body {
                    out.push_str(&self.emit_stmt(s, depth + 1));
                }
                out.push_str(&format!("{}}} while ({});\n", indent, cond));
                out
            }
            DartStmt::For(init, cond, update, body) => {
                let init_str = self.emit_stmt(init, 0).trim_end_matches('\n').to_string();
                let update_str = self
                    .emit_stmt(update, 0)
                    .trim_end_matches(";\n")
                    .trim()
                    .to_string();
                let mut out = format!(
                    "{}for ({}; {}; {}) {{\n",
                    indent,
                    init_str.trim_start(),
                    cond,
                    update_str
                );
                for s in body {
                    out.push_str(&self.emit_stmt(s, depth + 1));
                }
                out.push_str(&format!("{}}}\n", indent));
                out
            }
            DartStmt::ForIn(var, iter, body) => {
                let mut out = format!("{}for (final {} in {}) {{\n", indent, var, iter);
                for s in body {
                    out.push_str(&self.emit_stmt(s, depth + 1));
                }
                out.push_str(&format!("{}}}\n", indent));
                out
            }
            DartStmt::Break => format!("{}break;\n", indent),
            DartStmt::Continue => format!("{}continue;\n", indent),
            DartStmt::Throw(expr) => format!("{}throw {};\n", indent, expr),
            DartStmt::TryCatch(body, catch_var, handler, fin) => {
                let mut out = format!("{}try {{\n", indent);
                for s in body {
                    out.push_str(&self.emit_stmt(s, depth + 1));
                }
                out.push_str(&format!("{}}} catch ({}) {{\n", indent, catch_var));
                for s in handler {
                    out.push_str(&self.emit_stmt(s, depth + 1));
                }
                if !fin.is_empty() {
                    out.push_str(&format!("{}}} finally {{\n", indent));
                    for s in fin {
                        out.push_str(&self.emit_stmt(s, depth + 1));
                    }
                }
                out.push_str(&format!("{}}}\n", indent));
                out
            }
            DartStmt::Switch(expr, cases, default) => {
                let mut out = format!("{}switch ({}) {{\n", indent, expr);
                for (val, stmts) in cases {
                    out.push_str(&format!("{}  case {}:\n", indent, val));
                    for s in stmts {
                        out.push_str(&self.emit_stmt(s, depth + 2));
                    }
                    out.push_str(&format!("{}    break;\n", indent));
                }
                if !default.is_empty() {
                    out.push_str(&format!("{}  default:\n", indent));
                    for s in default {
                        out.push_str(&self.emit_stmt(s, depth + 2));
                    }
                }
                out.push_str(&format!("{}}}\n", indent));
                out
            }
            DartStmt::Assert(expr) => format!("{}assert({});\n", indent, expr),
            DartStmt::Block(stmts) => {
                let mut out = format!("{}{{\n", indent);
                for s in stmts {
                    out.push_str(&self.emit_stmt(s, depth + 1));
                }
                out.push_str(&format!("{}}}\n", indent));
                out
            }
            DartStmt::Raw(code) => format!("{}{}\n", indent, code),
        }
    }
    pub(super) fn indent(&self, depth: usize) -> String {
        " ".repeat(depth * self.indent_width)
    }
    /// Compile an LCNF function to a `DartFunction`.
    pub fn compile_lcnf_function(&mut self, func: &LcnfFunDecl) -> Result<DartFunction, String> {
        let name = self.mangle_name(&func.name.to_string());
        let ret_ty = lcnf_type_to_dart(&func.ret_type);
        let mut params = Vec::new();
        for param in &func.params {
            let pname = format!("_x{}", param.id.0);
            let pty = lcnf_type_to_dart(&param.ty);
            params.push(DartParam::positional(pty, pname));
        }
        let mut body_stmts = Vec::new();
        let result_expr = self.compile_expr(&func.body, &mut body_stmts)?;
        body_stmts.push(DartStmt::Return(Some(result_expr)));
        let mut dart_fn = DartFunction::new(name, ret_ty);
        dart_fn.params = params;
        dart_fn.body = body_stmts;
        Ok(dart_fn)
    }
    /// Compile an LCNF module to a `DartModule`.
    pub fn compile_lcnf_module(&mut self, module: &LcnfModule) -> Result<DartModule, String> {
        let mut dart_module = DartModule::new();
        dart_module.imports.push(DartImport::simple("dart:core"));
        dart_module
            .imports
            .push(DartImport::simple("dart:collection"));
        let ctor_names = collect_ctor_names(module);
        for ctor_name in &ctor_names {
            dart_module.classes.push(make_ctor_class(ctor_name));
        }
        dart_module
            .functions
            .push(DartFunction::new("_unreachable", DartType::DtDynamic));
        for func in &module.fun_decls {
            let dart_fn = self.compile_lcnf_function(func)?;
            dart_module.functions.push(dart_fn);
        }
        Ok(dart_module)
    }
    pub(super) fn compile_expr(
        &mut self,
        expr: &LcnfExpr,
        stmts: &mut Vec<DartStmt>,
    ) -> Result<DartExpr, String> {
        match expr {
            LcnfExpr::Return(arg) => Ok(self.compile_arg(arg)),
            LcnfExpr::Unreachable => {
                stmts.push(DartStmt::Throw(DartExpr::New(
                    "StateError".to_string(),
                    None,
                    vec![DartExpr::Lit(DartLit::Str(
                        "OxiLean: unreachable".to_string(),
                    ))],
                )));
                Ok(DartExpr::Lit(DartLit::Null))
            }
            LcnfExpr::TailCall(func, args) => {
                let callee = self.compile_arg(func);
                let dart_args: Vec<DartExpr> = args.iter().map(|a| self.compile_arg(a)).collect();
                Ok(DartExpr::Call(Box::new(callee), dart_args))
            }
            LcnfExpr::Let {
                id,
                name: _,
                ty,
                value,
                body,
            } => {
                let var_name = format!("_x{}", id.0);
                let dart_ty = lcnf_type_to_dart(ty);
                let val_expr = self.compile_let_value(value)?;
                stmts.push(DartStmt::VarDecl(dart_ty, var_name, val_expr));
                self.compile_expr(body, stmts)
            }
            LcnfExpr::Case {
                scrutinee,
                scrutinee_ty: _,
                alts,
                default,
            } => {
                let scrutinee_expr = DartExpr::Var(format!("_x{}", scrutinee.0));
                let result_var = self.fresh_var();
                let mut cases: Vec<(DartExpr, Vec<DartStmt>)> = Vec::new();
                for alt in alts {
                    let mut branch_stmts: Vec<DartStmt> = Vec::new();
                    for (idx, param) in alt.params.iter().enumerate() {
                        let field_access = DartExpr::Index(
                            Box::new(DartExpr::Field(
                                Box::new(scrutinee_expr.clone()),
                                "fields".to_string(),
                            )),
                            Box::new(DartExpr::Lit(DartLit::Int(idx as i64))),
                        );
                        let pname = format!("_x{}", param.id.0);
                        let pty = lcnf_type_to_dart(&param.ty);
                        branch_stmts.push(DartStmt::FinalDecl(pty, pname, field_access));
                    }
                    let branch_result = self.compile_expr(&alt.body, &mut branch_stmts)?;
                    branch_stmts.push(DartStmt::Assign(result_var.clone(), branch_result));
                    let tag_val = DartExpr::Lit(DartLit::Int(alt.ctor_tag as i64));
                    cases.push((tag_val, branch_stmts));
                }
                let mut default_stmts: Vec<DartStmt> = Vec::new();
                if let Some(def) = default {
                    let def_result = self.compile_expr(def, &mut default_stmts)?;
                    default_stmts.push(DartStmt::Assign(result_var.clone(), def_result));
                } else {
                    default_stmts.push(DartStmt::Throw(DartExpr::New(
                        "StateError".to_string(),
                        None,
                        vec![DartExpr::Lit(DartLit::Str(
                            "OxiLean: unreachable branch".to_string(),
                        ))],
                    )));
                }
                let discriminant = DartExpr::Field(Box::new(scrutinee_expr), "tag".to_string());
                stmts.push(DartStmt::VarDecl(
                    DartType::DtDynamic,
                    result_var.clone(),
                    DartExpr::Lit(DartLit::Null),
                ));
                stmts.push(DartStmt::Switch(discriminant, cases, default_stmts));
                Ok(DartExpr::Var(result_var))
            }
        }
    }
    pub(super) fn compile_let_value(&mut self, value: &LcnfLetValue) -> Result<DartExpr, String> {
        match value {
            LcnfLetValue::Lit(lit) => Ok(self.compile_lit(lit)),
            LcnfLetValue::Erased => Ok(DartExpr::Lit(DartLit::Null)),
            LcnfLetValue::FVar(id) => Ok(DartExpr::Var(format!("_x{}", id.0))),
            LcnfLetValue::App(func, args) => {
                let callee = self.compile_arg(func);
                let dart_args: Vec<DartExpr> = args.iter().map(|a| self.compile_arg(a)).collect();
                Ok(DartExpr::Call(Box::new(callee), dart_args))
            }
            LcnfLetValue::Proj(_name, idx, var) => {
                let base = DartExpr::Var(format!("_x{}", var.0));
                Ok(DartExpr::Index(
                    Box::new(DartExpr::Field(Box::new(base), "fields".to_string())),
                    Box::new(DartExpr::Lit(DartLit::Int(*idx as i64))),
                ))
            }
            LcnfLetValue::Ctor(name, _tag, args) => {
                let ctor_name = self.mangle_name(name);
                let dart_args: Vec<DartExpr> = args.iter().map(|a| self.compile_arg(a)).collect();
                Ok(DartExpr::New(ctor_name, None, dart_args))
            }
            LcnfLetValue::Reset(_var) => Ok(DartExpr::Lit(DartLit::Null)),
            LcnfLetValue::Reuse(_slot, name, _tag, args) => {
                let ctor_name = self.mangle_name(name);
                let dart_args: Vec<DartExpr> = args.iter().map(|a| self.compile_arg(a)).collect();
                Ok(DartExpr::New(ctor_name, None, dart_args))
            }
        }
    }
    pub(super) fn compile_arg(&self, arg: &LcnfArg) -> DartExpr {
        match arg {
            LcnfArg::Var(id) => DartExpr::Var(format!("_x{}", id.0)),
            LcnfArg::Lit(lit) => self.compile_lit(lit),
            LcnfArg::Erased => DartExpr::Lit(DartLit::Null),
            LcnfArg::Type(_) => DartExpr::Lit(DartLit::Null),
        }
    }
    pub(super) fn compile_lit(&self, lit: &LcnfLit) -> DartExpr {
        match lit {
            LcnfLit::Nat(n) => DartExpr::Lit(DartLit::Int(*n as i64)),
            LcnfLit::Str(s) => DartExpr::Lit(DartLit::Str(s.clone())),
        }
    }
}
/// A complete Dart source file.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DartFile {
    pub imports: Vec<DartImport>,
    pub type_aliases: Vec<DartTypeAlias>,
    pub enums: Vec<DartEnum>,
    pub classes: Vec<DartClass>,
    pub mixins: Vec<DartMixin>,
    pub top_level_functions: Vec<DartFunction>,
    pub top_level_vars: Vec<(DartType, String, Option<DartExpr>)>,
    pub library_name: Option<String>,
}
#[allow(dead_code)]
impl DartFile {
    pub fn new() -> Self {
        DartFile {
            imports: vec![],
            type_aliases: vec![],
            enums: vec![],
            classes: vec![],
            mixins: vec![],
            top_level_functions: vec![],
            top_level_vars: vec![],
            library_name: None,
        }
    }
    pub fn with_library(mut self, name: impl Into<String>) -> Self {
        self.library_name = Some(name.into());
        self
    }
    pub fn add_import(&mut self, imp: DartImport) {
        self.imports.push(imp);
    }
    pub fn add_enum(&mut self, e: DartEnum) {
        self.enums.push(e);
    }
    pub fn add_class(&mut self, c: DartClass) {
        self.classes.push(c);
    }
    pub fn add_function(&mut self, f: DartFunction) {
        self.top_level_functions.push(f);
    }
    pub fn add_type_alias(&mut self, ta: DartTypeAlias) {
        self.type_aliases.push(ta);
    }
    pub fn add_mixin(&mut self, m: DartMixin) {
        self.mixins.push(m);
    }
    /// Emit the full Dart source file.
    pub fn emit(&self, backend: &DartBackend) -> String {
        let mut out = String::new();
        if let Some(lib) = &self.library_name {
            out.push_str(&format!("library {};\n\n", lib));
        }
        for imp in &self.imports {
            out.push_str(&imp.emit());
        }
        if !self.imports.is_empty() {
            out.push('\n');
        }
        for ta in &self.type_aliases {
            out.push_str(&ta.emit());
        }
        if !self.type_aliases.is_empty() {
            out.push('\n');
        }
        for e in &self.enums {
            out.push_str(&e.emit());
            out.push('\n');
        }
        for m in &self.mixins {
            out.push_str(&m.emit(backend, 0));
            out.push('\n');
        }
        for c in &self.classes {
            out.push_str(&backend.emit_class(c, 0));
            out.push('\n');
        }
        for (ty, name, init) in &self.top_level_vars {
            if let Some(val) = init {
                out.push_str(&format!("{} {} = {};\n", ty, name, val));
            } else {
                out.push_str(&format!("late {} {};\n", ty, name));
            }
        }
        if !self.top_level_vars.is_empty() {
            out.push('\n');
        }
        for func in &self.top_level_functions {
            out.push_str(&backend.emit_function(func, 0));
        }
        out
    }
}
/// Helper for building common Dart stream patterns.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DartStreamBuilder {
    pub item_type: DartType,
}
#[allow(dead_code)]
impl DartStreamBuilder {
    pub fn new(item_type: DartType) -> Self {
        DartStreamBuilder { item_type }
    }
    /// Generate a `Stream.fromIterable([...])` expression.
    pub fn from_iterable(items: Vec<DartExpr>) -> DartExpr {
        DartExpr::MethodCall(
            Box::new(DartExpr::Var("Stream".to_string())),
            "fromIterable".to_string(),
            vec![DartExpr::ListLit(items)],
        )
    }
    /// Generate a `stream.listen((item) { ... })` statement.
    pub fn listen(stream: DartExpr, param: &str, body: Vec<DartStmt>) -> DartStmt {
        let backend = DartBackend::new();
        let body_str: String = body.iter().map(|s| backend.emit_stmt(s, 1)).collect();
        let closure = DartExpr::Raw(format!("({}) {{\n{}}}", param, body_str));
        DartStmt::Expr(DartExpr::MethodCall(
            Box::new(stream),
            "listen".to_string(),
            vec![closure],
        ))
    }
    /// Generate a `StreamController<T>` declaration.
    pub fn controller_decl(&self, name: &str) -> DartStmt {
        let ty = DartType::DtGeneric("StreamController".to_string(), vec![self.item_type.clone()]);
        DartStmt::VarDecl(
            ty.clone(),
            name.to_string(),
            DartExpr::New("StreamController".to_string(), None, vec![]),
        )
    }
}
/// Describes a sealed class hierarchy for ADT-style sum types.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DartSealedHierarchy {
    pub base_name: String,
    pub variants: Vec<DartClass>,
}
#[allow(dead_code)]
impl DartSealedHierarchy {
    pub fn new(base_name: impl Into<String>) -> Self {
        DartSealedHierarchy {
            base_name: base_name.into(),
            variants: vec![],
        }
    }
    pub fn add_variant(&mut self, variant: DartClass) {
        self.variants.push(variant);
    }
    /// Emit the sealed base class and all variant subclasses.
    pub fn emit(&self, backend: &DartBackend) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "sealed class {} {{\n  const {}();\n}}\n\n",
            self.base_name, self.base_name
        ));
        for variant in &self.variants {
            out.push_str(&backend.emit_class(variant, 0));
            out.push('\n');
        }
        out
    }
}
/// A Dart `typedef` (type alias) declaration.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DartTypeAlias {
    pub name: String,
    pub ty: DartType,
    pub doc: Option<String>,
}
#[allow(dead_code)]
impl DartTypeAlias {
    pub fn new(name: impl Into<String>, ty: DartType) -> Self {
        DartTypeAlias {
            name: name.into(),
            ty,
            doc: None,
        }
    }
    pub fn with_doc(mut self, doc: impl Into<String>) -> Self {
        self.doc = Some(doc.into());
        self
    }
    pub fn emit(&self) -> String {
        let mut out = String::new();
        if let Some(doc) = &self.doc {
            out.push_str(&format!("/// {}\n", doc));
        }
        out.push_str(&format!("typedef {} = {};\n", self.name, self.ty));
        out
    }
}
/// Dart statement AST.
#[derive(Debug, Clone, PartialEq)]
pub enum DartStmt {
    /// `Type name = expr;` or `var name = expr;`
    VarDecl(DartType, String, DartExpr),
    /// `var name = expr;` (inferred type)
    VarInferred(String, DartExpr),
    /// `final Type name = expr;`
    FinalDecl(DartType, String, DartExpr),
    /// `const Type name = expr;`
    ConstDecl(DartType, String, DartExpr),
    /// `name = expr;`
    Assign(String, DartExpr),
    /// `expr.field = value;`
    FieldAssign(DartExpr, String, DartExpr),
    /// `expr[idx] = value;`
    IndexAssign(DartExpr, DartExpr, DartExpr),
    /// `return expr;`
    Return(Option<DartExpr>),
    /// Expression statement: `expr;`
    Expr(DartExpr),
    /// `if (cond) { then } else { else_ }`
    If(DartExpr, Vec<DartStmt>, Vec<DartStmt>),
    /// `while (cond) { body }`
    While(DartExpr, Vec<DartStmt>),
    /// `do { body } while (cond);`
    DoWhile(Vec<DartStmt>, DartExpr),
    /// `for (init; cond; update) { body }`
    For(Box<DartStmt>, DartExpr, Box<DartStmt>, Vec<DartStmt>),
    /// `for (final elem in iterable) { body }`
    ForIn(String, DartExpr, Vec<DartStmt>),
    /// `break;`
    Break,
    /// `continue;`
    Continue,
    /// `throw expr;`
    Throw(DartExpr),
    /// `try { body } catch (e) { handler } finally { fin }`
    TryCatch(Vec<DartStmt>, String, Vec<DartStmt>, Vec<DartStmt>),
    /// `switch (expr) { case v: stmts ... default: stmts }`
    Switch(DartExpr, Vec<(DartExpr, Vec<DartStmt>)>, Vec<DartStmt>),
    /// `assert(expr);`
    Assert(DartExpr),
    /// Block `{ stmts }`
    Block(Vec<DartStmt>),
    /// Raw Dart code
    Raw(String),
}
/// A Dart import directive.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DartImportExt {
    pub uri: String,
    pub prefix: Option<String>,
    pub show: Vec<String>,
    pub hide: Vec<String>,
    pub is_deferred: bool,
}
#[allow(dead_code)]
impl DartImportExt {
    pub fn simple(uri: impl Into<String>) -> Self {
        DartImportExt {
            uri: uri.into(),
            prefix: None,
            show: vec![],
            hide: vec![],
            is_deferred: false,
        }
    }
    pub fn with_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }
    pub fn show_identifiers(mut self, ids: Vec<String>) -> Self {
        self.show = ids;
        self
    }
    pub fn hide_identifiers(mut self, ids: Vec<String>) -> Self {
        self.hide = ids;
        self
    }
    pub fn deferred(mut self) -> Self {
        self.is_deferred = true;
        self
    }
    pub fn emit(&self) -> String {
        let mut out = format!("import '{}'", self.uri);
        if self.is_deferred {
            out.push_str(" deferred");
        }
        if let Some(prefix) = &self.prefix {
            out.push_str(&format!(" as {}", prefix));
        }
        if !self.show.is_empty() {
            out.push_str(&format!(" show {}", self.show.join(", ")));
        }
        if !self.hide.is_empty() {
            out.push_str(&format!(" hide {}", self.hide.join(", ")));
        }
        out.push_str(";\n");
        out
    }
}
/// Metrics collected over a Dart file's generated output.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct DartCodeMetrics {
    pub total_lines: usize,
    pub class_count: usize,
    pub function_count: usize,
    pub import_count: usize,
}
#[allow(dead_code)]
impl DartCodeMetrics {
    pub fn collect(file: &DartFile) -> Self {
        DartCodeMetrics {
            total_lines: 0,
            class_count: file.classes.len(),
            function_count: file.top_level_functions.len(),
            import_count: file.imports.len(),
        }
    }
    pub fn update_lines(&mut self, source: &str) {
        self.total_lines = source.lines().count();
    }
}
/// A field in a Dart class.
#[derive(Debug, Clone)]
pub struct DartField {
    pub ty: DartType,
    pub name: String,
    pub is_final: bool,
    pub is_static: bool,
    pub is_late: bool,
    pub default_value: Option<DartExpr>,
    pub doc: Option<String>,
}
impl DartField {
    pub fn new(ty: DartType, name: impl Into<String>) -> Self {
        DartField {
            ty,
            name: name.into(),
            is_final: false,
            is_static: false,
            is_late: false,
            default_value: None,
            doc: None,
        }
    }
    pub fn final_field(ty: DartType, name: impl Into<String>) -> Self {
        DartField {
            ty,
            name: name.into(),
            is_final: true,
            is_static: false,
            is_late: false,
            default_value: None,
            doc: None,
        }
    }
}
/// Dart type representation (null-safe Dart ≥ 2.12).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DartType {
    /// `int`
    DtInt,
    /// `double`
    DtDouble,
    /// `bool`
    DtBool,
    /// `String`
    DtString,
    /// `void`
    DtVoid,
    /// `dynamic`
    DtDynamic,
    /// `Object`
    DtObject,
    /// `Null`
    DtNull,
    /// `T?` — nullable wrapper
    DtNullable(Box<DartType>),
    /// `List<T>`
    DtList(Box<DartType>),
    /// `Map<K, V>`
    DtMap(Box<DartType>, Box<DartType>),
    /// `Set<T>`
    DtSet(Box<DartType>),
    /// `Future<T>`
    DtFuture(Box<DartType>),
    /// `Stream<T>`
    DtStream(Box<DartType>),
    /// `T Function(P0, P1, ...)` — function type
    DtFunction(Vec<DartType>, Box<DartType>),
    /// Named class, e.g. `MyClass`
    DtNamed(String),
    /// Generic instantiation, e.g. `MyClass<int, String>`
    DtGeneric(String, Vec<DartType>),
}
/// Null safety helper expressions.
#[allow(dead_code)]
pub struct DartNullSafety;
#[allow(dead_code)]
impl DartNullSafety {
    /// `expr!` — null assertion operator.
    pub fn assert_non_null(expr: DartExpr) -> DartExpr {
        DartExpr::Raw(format!("{}!", expr))
    }
    /// `expr ?? fallback` — null coalescing.
    pub fn coalesce(expr: DartExpr, fallback: DartExpr) -> DartExpr {
        DartExpr::NullCoalesce(Box::new(expr), Box::new(fallback))
    }
    /// `expr?.field` — null-safe field access.
    pub fn safe_field(expr: DartExpr, field: impl Into<String>) -> DartExpr {
        DartExpr::NullAware(Box::new(expr), field.into())
    }
    /// Emit a null check guard statement.
    pub fn guard_not_null(var: &str, _ty: DartType) -> DartStmt {
        DartStmt::If(
            DartExpr::BinOp(
                Box::new(DartExpr::Var(var.to_string())),
                "==".to_string(),
                Box::new(DartExpr::Lit(DartLit::Null)),
            ),
            vec![DartStmt::Throw(DartExpr::New(
                "ArgumentError".to_string(),
                None,
                vec![DartExpr::Lit(DartLit::Str(format!(
                    "{} must not be null",
                    var
                )))],
            ))],
            vec![],
        )
    }
    /// `T? x = null;` — nullable variable declaration.
    pub fn nullable_decl(ty: DartType, name: &str) -> DartStmt {
        DartStmt::VarDecl(
            DartType::DtNullable(Box::new(ty)),
            name.to_string(),
            DartExpr::Lit(DartLit::Null),
        )
    }
}
