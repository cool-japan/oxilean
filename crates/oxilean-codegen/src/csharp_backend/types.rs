//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use std::fmt::Write as FmtWrite;

use super::functions::CSHARP_RUNTIME;
use super::functions::*;

/// A `case` inside a `switch` statement.
#[derive(Debug, Clone, PartialEq)]
pub struct CSharpSwitchCase {
    /// The case label expression (or pattern string for C# 8+ pattern matching)
    pub label: std::string::String,
    pub stmts: Vec<CSharpStmt>,
}
/// C# statement AST.
#[derive(Debug, Clone, PartialEq)]
pub enum CSharpStmt {
    /// Expression statement: `expr;`
    Expr(CSharpExpr),
    /// Assignment: `target = value;`
    Assign {
        target: CSharpExpr,
        value: CSharpExpr,
    },
    /// Local variable declaration: `var name = expr;` or `Type name = expr;`
    LocalVar {
        name: std::string::String,
        ty: Option<CSharpType>,
        init: Option<CSharpExpr>,
        is_const: bool,
    },
    /// `if (cond) { ... } else { ... }`
    If {
        cond: CSharpExpr,
        then_stmts: Vec<CSharpStmt>,
        else_stmts: Vec<CSharpStmt>,
    },
    /// `switch (expr) { case ...: ... }`
    Switch {
        expr: CSharpExpr,
        cases: Vec<CSharpSwitchCase>,
        default: Vec<CSharpStmt>,
    },
    /// `while (cond) { ... }`
    While {
        cond: CSharpExpr,
        body: Vec<CSharpStmt>,
    },
    /// `for (init; cond; step) { ... }`
    For {
        init: Option<Box<CSharpStmt>>,
        cond: Option<CSharpExpr>,
        step: Option<CSharpExpr>,
        body: Vec<CSharpStmt>,
    },
    /// `foreach (var item in collection) { ... }`
    ForEach {
        var_name: std::string::String,
        var_ty: Option<CSharpType>,
        collection: CSharpExpr,
        body: Vec<CSharpStmt>,
    },
    /// `return expr;` or `return;`
    Return(Option<CSharpExpr>),
    /// `throw expr;`
    Throw(CSharpExpr),
    /// `try { ... } catch (Type e) { ... } finally { ... }`
    TryCatch {
        try_stmts: Vec<CSharpStmt>,
        catches: Vec<CSharpCatchClause>,
        finally_stmts: Vec<CSharpStmt>,
    },
    /// `using (expr) { ... }` or `using var x = expr;`
    Using {
        resource: CSharpExpr,
        var_name: Option<std::string::String>,
        body: Vec<CSharpStmt>,
    },
    /// `lock (obj) { ... }`
    Lock {
        obj: CSharpExpr,
        body: Vec<CSharpStmt>,
    },
    /// `break;`
    Break,
    /// `continue;`
    Continue,
    /// `yield return expr;`
    YieldReturn(CSharpExpr),
    /// `yield break;`
    YieldBreak,
}
/// A C# interface declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct CSharpInterface {
    pub name: std::string::String,
    /// Interface method signatures (body must be empty or default impl)
    pub methods: Vec<CSharpMethod>,
    /// Properties in the interface
    pub properties: Vec<CSharpProperty>,
    /// Extends other interfaces
    pub extends: Vec<std::string::String>,
    pub visibility: CSharpVisibility,
    pub type_params: Vec<std::string::String>,
}
impl CSharpInterface {
    pub fn new(name: &str) -> Self {
        CSharpInterface {
            name: name.to_string(),
            methods: Vec::new(),
            properties: Vec::new(),
            extends: Vec::new(),
            visibility: CSharpVisibility::Public,
            type_params: Vec::new(),
        }
    }
    pub fn emit(&self, indent: &str) -> std::string::String {
        let inner = format!("{}    ", indent);
        let mut out = std::string::String::new();
        let type_params_str = if self.type_params.is_empty() {
            std::string::String::new()
        } else {
            format!("<{}>", self.type_params.join(", "))
        };
        let extends_str = if self.extends.is_empty() {
            std::string::String::new()
        } else {
            format!(" : {}", self.extends.join(", "))
        };
        let _ = writeln!(
            out,
            "{}{} interface {}{}{}",
            indent, self.visibility, self.name, type_params_str, extends_str
        );
        let _ = writeln!(out, "{}{{", indent);
        for prop in &self.properties {
            out.push_str(&prop.emit(&inner));
        }
        for method in &self.methods {
            // In a C# interface, methods are implicitly abstract.
            // Emit without the `abstract` keyword.
            let mut iface_method = method.clone();
            iface_method.is_abstract = false;
            if method.body.is_empty() && method.expr_body.is_none() {
                // Emit as a signature with semicolon, no `abstract` keyword.
                let params_str = method
                    .params
                    .iter()
                    .map(|(n, t)| format!("{} {}", t, n))
                    .collect::<Vec<_>>()
                    .join(", ");
                let type_params_str = if method.type_params.is_empty() {
                    std::string::String::new()
                } else {
                    format!("<{}>", method.type_params.join(", "))
                };
                let _ = writeln!(
                    out,
                    "{}{} {} {}{}({});",
                    inner,
                    method.visibility,
                    method.return_type,
                    method.name,
                    type_params_str,
                    params_str
                );
            } else {
                out.push_str(&iface_method.emit(&inner));
            }
        }
        let _ = writeln!(out, "{}}}", indent);
        out
    }
}
/// A C# field declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct CSharpField {
    pub name: std::string::String,
    pub ty: CSharpType,
    pub visibility: CSharpVisibility,
    pub is_static: bool,
    pub is_readonly: bool,
    pub is_const: bool,
    pub default_value: Option<CSharpExpr>,
}
impl CSharpField {
    pub fn new(name: &str, ty: CSharpType) -> Self {
        CSharpField {
            name: name.to_string(),
            ty,
            visibility: CSharpVisibility::Private,
            is_static: false,
            is_readonly: false,
            is_const: false,
            default_value: None,
        }
    }
    pub fn emit(&self, indent: &str) -> std::string::String {
        let mut out = std::string::String::new();
        let mut mods = vec![format!("{}", self.visibility)];
        if self.is_static {
            mods.push("static".to_string());
        }
        if self.is_const {
            mods.push("const".to_string());
        }
        if self.is_readonly {
            mods.push("readonly".to_string());
        }
        if let Some(val) = &self.default_value {
            let _ = writeln!(
                out,
                "{}{} {} {} = {};",
                indent,
                mods.join(" "),
                self.ty,
                self.name,
                val
            );
        } else {
            let _ = writeln!(
                out,
                "{}{} {} {};",
                indent,
                mods.join(" "),
                self.ty,
                self.name
            );
        }
        out
    }
}
/// A C# method (member function).
#[derive(Debug, Clone, PartialEq)]
pub struct CSharpMethod {
    /// Method name
    pub name: std::string::String,
    /// Return type
    pub return_type: CSharpType,
    /// Parameters: `(name, type)`
    pub params: Vec<(std::string::String, CSharpType)>,
    /// Method body statements
    pub body: Vec<CSharpStmt>,
    /// Access modifier
    pub visibility: CSharpVisibility,
    /// `static` modifier
    pub is_static: bool,
    /// `async` modifier
    pub is_async: bool,
    /// `override` modifier
    pub is_override: bool,
    /// `virtual` modifier
    pub is_virtual: bool,
    /// `abstract` modifier (body must be empty)
    pub is_abstract: bool,
    /// Generic type parameters: `<T, U>`
    pub type_params: Vec<std::string::String>,
    /// Expression body (for `=> expr` methods)
    pub expr_body: Option<CSharpExpr>,
}
impl CSharpMethod {
    /// Create a new method with default settings (public, non-static).
    pub fn new(name: &str, return_type: CSharpType) -> Self {
        CSharpMethod {
            name: name.to_string(),
            return_type,
            params: Vec::new(),
            body: Vec::new(),
            visibility: CSharpVisibility::Public,
            is_static: false,
            is_async: false,
            is_override: false,
            is_virtual: false,
            is_abstract: false,
            type_params: Vec::new(),
            expr_body: None,
        }
    }
    /// Emit the method as C# source code.
    pub fn emit(&self, indent: &str) -> std::string::String {
        let inner = format!("{}    ", indent);
        let mut out = std::string::String::new();
        let mut mods = vec![format!("{}", self.visibility)];
        if self.is_static {
            mods.push("static".to_string());
        }
        if self.is_async {
            mods.push("async".to_string());
        }
        if self.is_abstract {
            mods.push("abstract".to_string());
        }
        if self.is_override {
            mods.push("override".to_string());
        }
        if self.is_virtual {
            mods.push("virtual".to_string());
        }
        let type_params_str = if self.type_params.is_empty() {
            std::string::String::new()
        } else {
            format!("<{}>", self.type_params.join(", "))
        };
        let params_str = self
            .params
            .iter()
            .map(|(n, t)| format!("{} {}", t, n))
            .collect::<Vec<_>>()
            .join(", ");
        let _ = write!(
            out,
            "{}{} {} {}{}({})",
            indent,
            mods.join(" "),
            self.return_type,
            self.name,
            type_params_str,
            params_str
        );
        if self.is_abstract {
            let _ = writeln!(out, ";");
            return out;
        }
        if let Some(expr) = &self.expr_body {
            let _ = writeln!(out, " => {};", expr);
            return out;
        }
        let _ = writeln!(out);
        let _ = writeln!(out, "{}{{", indent);
        emit_stmts(&self.body, &inner, &mut out);
        let _ = writeln!(out, "{}}}", indent);
        out
    }
}
/// A complete C# compilation unit (one `.cs` file).
#[derive(Debug, Clone)]
pub struct CSharpModule {
    /// The namespace for all contained types
    pub namespace: std::string::String,
    /// `using` directives (namespaces to import)
    pub using_directives: Vec<std::string::String>,
    /// Class declarations
    pub classes: Vec<CSharpClass>,
    /// Record declarations
    pub records: Vec<CSharpRecord>,
    /// Interface declarations
    pub interfaces: Vec<CSharpInterface>,
    /// Enum declarations
    pub enums: Vec<CSharpEnum>,
    /// Top-level comment / header
    pub header_comment: Option<std::string::String>,
    /// `#nullable enable` directive
    pub nullable_enable: bool,
}
impl CSharpModule {
    /// Create a new module with the given namespace.
    pub fn new(namespace: &str) -> Self {
        CSharpModule {
            namespace: namespace.to_string(),
            using_directives: Vec::new(),
            classes: Vec::new(),
            records: Vec::new(),
            interfaces: Vec::new(),
            enums: Vec::new(),
            header_comment: None,
            nullable_enable: true,
        }
    }
    /// Add a using directive, deduplicating automatically.
    pub fn add_using(&mut self, ns: &str) {
        if !self.using_directives.iter().any(|u| u == ns) {
            self.using_directives.push(ns.to_string());
        }
    }
    /// Emit the complete C# source file as a string.
    pub fn emit(&self) -> std::string::String {
        let mut out = std::string::String::new();
        if let Some(comment) = &self.header_comment {
            for line in comment.lines() {
                let _ = writeln!(out, "// {}", line);
            }
            let _ = writeln!(out);
        }
        if self.nullable_enable {
            let _ = writeln!(out, "#nullable enable");
            let _ = writeln!(out);
        }
        let mut usings = self.using_directives.clone();
        usings.sort();
        usings.dedup();
        for u in &usings {
            let _ = writeln!(out, "using {};", u);
        }
        if !usings.is_empty() {
            let _ = writeln!(out);
        }
        let _ = writeln!(out, "namespace {};", self.namespace);
        let _ = writeln!(out);
        for e in &self.enums {
            out.push_str(&e.emit(""));
            let _ = writeln!(out);
        }
        for iface in &self.interfaces {
            out.push_str(&iface.emit(""));
            let _ = writeln!(out);
        }
        for rec in &self.records {
            out.push_str(&rec.emit(""));
            let _ = writeln!(out);
        }
        for cls in &self.classes {
            out.push_str(&cls.emit(""));
            let _ = writeln!(out);
        }
        out.push_str(CSHARP_RUNTIME);
        out
    }
}
/// A C# constructor.
#[derive(Debug, Clone, PartialEq)]
pub struct CSharpConstructor {
    pub class_name: std::string::String,
    pub params: Vec<(std::string::String, CSharpType)>,
    pub body: Vec<CSharpStmt>,
    pub visibility: CSharpVisibility,
    /// `(is_base, args)` — `true` means `base(...)`, `false` means `this(...)`
    pub base_call: Option<(bool, Vec<CSharpExpr>)>,
}
impl CSharpConstructor {
    pub fn new(class_name: &str) -> Self {
        CSharpConstructor {
            class_name: class_name.to_string(),
            params: Vec::new(),
            body: Vec::new(),
            visibility: CSharpVisibility::Public,
            base_call: None,
        }
    }
    pub fn emit(&self, indent: &str) -> std::string::String {
        let inner = format!("{}    ", indent);
        let mut out = std::string::String::new();
        let params_str = self
            .params
            .iter()
            .map(|(n, t)| format!("{} {}", t, n))
            .collect::<Vec<_>>()
            .join(", ");
        let base_str = match &self.base_call {
            None => std::string::String::new(),
            Some((is_base, args)) => {
                let kw = if *is_base { "base" } else { "this" };
                let args_str = args
                    .iter()
                    .map(|a| format!("{}", a))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!(" : {}({})", kw, args_str)
            }
        };
        let _ = writeln!(
            out,
            "{}{} {}({}){}",
            indent, self.visibility, self.class_name, params_str, base_str
        );
        let _ = writeln!(out, "{}{{", indent);
        emit_stmts(&self.body, &inner, &mut out);
        let _ = writeln!(out, "{}}}", indent);
        out
    }
}
/// Part of a string interpolation `$"..."`.
#[derive(Debug, Clone, PartialEq)]
pub enum CSharpInterpolationPart {
    /// Literal text segment
    Text(std::string::String),
    /// `{expr}` hole
    Expr(CSharpExpr),
    /// `{expr:format}` hole with format specifier
    ExprFmt(CSharpExpr, std::string::String),
}
/// A C# class declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct CSharpClass {
    pub name: std::string::String,
    /// Base class (single inheritance)
    pub base_class: Option<std::string::String>,
    /// Implemented interfaces
    pub interfaces: Vec<std::string::String>,
    /// Methods
    pub methods: Vec<CSharpMethod>,
    /// Properties (auto-properties and expression-body)
    pub properties: Vec<CSharpProperty>,
    /// Constructors
    pub constructors: Vec<CSharpConstructor>,
    /// `sealed` — cannot be inherited
    pub is_sealed: bool,
    /// `abstract` — cannot be instantiated directly
    pub is_abstract: bool,
    /// `static` — no instances
    pub is_static: bool,
    /// `partial` modifier
    pub is_partial: bool,
    pub visibility: CSharpVisibility,
    pub type_params: Vec<std::string::String>,
    /// Fields
    pub fields: Vec<CSharpField>,
}
impl CSharpClass {
    pub fn new(name: &str) -> Self {
        CSharpClass {
            name: name.to_string(),
            base_class: None,
            interfaces: Vec::new(),
            methods: Vec::new(),
            properties: Vec::new(),
            constructors: Vec::new(),
            is_sealed: false,
            is_abstract: false,
            is_static: false,
            is_partial: false,
            visibility: CSharpVisibility::Public,
            type_params: Vec::new(),
            fields: Vec::new(),
        }
    }
    /// Emit the class as C# source code.
    pub fn emit(&self, indent: &str) -> std::string::String {
        let inner = format!("{}    ", indent);
        let mut out = std::string::String::new();
        let mut mods = vec![format!("{}", self.visibility)];
        if self.is_static {
            mods.push("static".to_string());
        }
        if self.is_sealed {
            mods.push("sealed".to_string());
        }
        if self.is_abstract {
            mods.push("abstract".to_string());
        }
        if self.is_partial {
            mods.push("partial".to_string());
        }
        let type_params_str = if self.type_params.is_empty() {
            std::string::String::new()
        } else {
            format!("<{}>", self.type_params.join(", "))
        };
        let mut inherits: Vec<std::string::String> = Vec::new();
        if let Some(base) = &self.base_class {
            inherits.push(base.clone());
        }
        inherits.extend(self.interfaces.iter().cloned());
        let inherit_str = if inherits.is_empty() {
            std::string::String::new()
        } else {
            format!(" : {}", inherits.join(", "))
        };
        let _ = writeln!(
            out,
            "{}{} class {}{}{}",
            indent,
            mods.join(" "),
            self.name,
            type_params_str,
            inherit_str
        );
        let _ = writeln!(out, "{}{{", indent);
        for field in &self.fields {
            out.push_str(&field.emit(&inner));
        }
        for ctor in &self.constructors {
            out.push_str(&ctor.emit(&inner));
        }
        for prop in &self.properties {
            out.push_str(&prop.emit(&inner));
        }
        for method in &self.methods {
            out.push_str(&method.emit(&inner));
        }
        let _ = writeln!(out, "{}}}", indent);
        out
    }
}
/// C# expression AST for code generation.
#[derive(Debug, Clone, PartialEq)]
pub enum CSharpExpr {
    /// A literal value: `42`, `"hello"`, `true`, `null`
    Lit(CSharpLit),
    /// A variable or identifier: `x`, `myVar`, `MyType`
    Var(std::string::String),
    /// Binary operator: `lhs + rhs`, `a == b`
    BinOp {
        op: std::string::String,
        lhs: Box<CSharpExpr>,
        rhs: Box<CSharpExpr>,
    },
    /// Unary operator: `!x`, `-n`, `~flags`
    UnaryOp {
        op: std::string::String,
        operand: Box<CSharpExpr>,
    },
    /// Static/free function call: `Math.Abs(x)`, `Foo(a, b)`
    Call {
        callee: Box<CSharpExpr>,
        args: Vec<CSharpExpr>,
    },
    /// Method call on a receiver: `list.Where(pred)`
    MethodCall {
        receiver: Box<CSharpExpr>,
        method: std::string::String,
        type_args: Vec<CSharpType>,
        args: Vec<CSharpExpr>,
    },
    /// Object creation: `new Foo(a, b)`
    New {
        ty: CSharpType,
        args: Vec<CSharpExpr>,
    },
    /// Lambda / anonymous function: `x => x + 1` or `(x, y) => x + y`
    Lambda {
        params: Vec<(std::string::String, Option<CSharpType>)>,
        body: Box<CSharpExpr>,
    },
    /// Ternary conditional: `cond ? then_expr : else_expr`
    Ternary {
        cond: Box<CSharpExpr>,
        then_expr: Box<CSharpExpr>,
        else_expr: Box<CSharpExpr>,
    },
    /// `null` literal (convenience alias)
    Null,
    /// `default(T)` or `default`
    Default(Option<CSharpType>),
    /// `nameof(x)` expression
    NameOf(std::string::String),
    /// `typeof(T)` expression
    TypeOf(CSharpType),
    /// `await expr`
    Await(Box<CSharpExpr>),
    /// `throw new Exception(msg)` as expression (C# 7+)
    Throw(Box<CSharpExpr>),
    /// `expr is Pattern` / `expr is Type varName`
    Is {
        expr: Box<CSharpExpr>,
        pattern: std::string::String,
    },
    /// `expr as Type`
    As {
        expr: Box<CSharpExpr>,
        ty: CSharpType,
    },
    /// Member access: `obj.Field`
    Member(Box<CSharpExpr>, std::string::String),
    /// Index access: `arr[idx]`
    Index(Box<CSharpExpr>, Box<CSharpExpr>),
    /// Switch expression: `expr switch { arm1, arm2, ... }`
    SwitchExpr {
        scrutinee: Box<CSharpExpr>,
        arms: Vec<CSharpSwitchArm>,
    },
    /// String interpolation: `$"Hello {name}!"`
    Interpolated(Vec<CSharpInterpolationPart>),
    /// Collection expression: `[a, b, c]` (C# 12)
    CollectionExpr(Vec<CSharpExpr>),
}
/// A C# auto-property or expression-body property.
#[derive(Debug, Clone, PartialEq)]
pub struct CSharpProperty {
    pub name: std::string::String,
    pub ty: CSharpType,
    pub visibility: CSharpVisibility,
    pub has_getter: bool,
    pub has_setter: bool,
    pub is_init_only: bool,
    pub is_static: bool,
    pub default_value: Option<CSharpExpr>,
    /// Expression body: `public int X => 42;`
    pub expr_body: Option<CSharpExpr>,
}
impl CSharpProperty {
    pub fn new_auto(name: &str, ty: CSharpType) -> Self {
        CSharpProperty {
            name: name.to_string(),
            ty,
            visibility: CSharpVisibility::Public,
            has_getter: true,
            has_setter: true,
            is_init_only: false,
            is_static: false,
            default_value: None,
            expr_body: None,
        }
    }
    pub fn emit(&self, indent: &str) -> std::string::String {
        let mut out = std::string::String::new();
        let mut mods = vec![format!("{}", self.visibility)];
        if self.is_static {
            mods.push("static".to_string());
        }
        if let Some(expr) = &self.expr_body {
            let _ = writeln!(
                out,
                "{}{} {} {} => {};",
                indent,
                mods.join(" "),
                self.ty,
                self.name,
                expr
            );
            return out;
        }
        let accessors = match (self.has_getter, self.has_setter, self.is_init_only) {
            (true, true, false) => "{ get; set; }",
            (true, false, false) => "{ get; }",
            (true, _, true) => "{ get; init; }",
            (false, true, false) => "{ set; }",
            _ => "{ get; set; }",
        };
        if let Some(val) = &self.default_value {
            let _ = writeln!(
                out,
                "{}{} {} {} {} = {};",
                indent,
                mods.join(" "),
                self.ty,
                self.name,
                accessors,
                val
            );
        } else {
            let _ = writeln!(
                out,
                "{}{} {} {} {};",
                indent,
                mods.join(" "),
                self.ty,
                self.name,
                accessors
            );
        }
        out
    }
}
/// A C# enum declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct CSharpEnum {
    pub name: std::string::String,
    pub variants: Vec<(std::string::String, Option<i64>)>,
    pub visibility: CSharpVisibility,
    pub underlying_type: Option<CSharpType>,
}
impl CSharpEnum {
    pub fn new(name: &str) -> Self {
        CSharpEnum {
            name: name.to_string(),
            variants: Vec::new(),
            visibility: CSharpVisibility::Public,
            underlying_type: None,
        }
    }
    pub fn emit(&self, indent: &str) -> std::string::String {
        let inner = format!("{}    ", indent);
        let mut out = std::string::String::new();
        let base_str = match &self.underlying_type {
            None => std::string::String::new(),
            Some(t) => format!(" : {}", t),
        };
        let _ = writeln!(
            out,
            "{}{} enum {}{}",
            indent, self.visibility, self.name, base_str
        );
        let _ = writeln!(out, "{}{{", indent);
        for (name, val) in &self.variants {
            if let Some(v) = val {
                let _ = writeln!(out, "{}{} = {},", inner, name, v);
            } else {
                let _ = writeln!(out, "{}{},", inner, name);
            }
        }
        let _ = writeln!(out, "{}}}", indent);
        out
    }
}
/// C# access modifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CSharpVisibility {
    Public,
    Private,
    Protected,
    Internal,
    ProtectedInternal,
    PrivateProtected,
}
/// C# type representation for type-directed code generation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CSharpType {
    /// `int` — 32-bit signed integer
    Int,
    /// `long` — 64-bit signed integer (used for Nat)
    Long,
    /// `double` — 64-bit IEEE 754 float
    Double,
    /// `float` — 32-bit IEEE 754 float
    Float,
    /// `bool`
    Bool,
    /// `string`
    String,
    /// `void`
    Void,
    /// `object`
    Object,
    /// `List<T>`
    List(Box<CSharpType>),
    /// `Dictionary<K, V>`
    Dict(Box<CSharpType>, Box<CSharpType>),
    /// `(T0, T1, ...)` — value tuple
    Tuple(Vec<CSharpType>),
    /// Named type (class, record, interface, enum, …)
    Custom(std::string::String),
    /// `T?` — nullable reference / value type
    Nullable(Box<CSharpType>),
    /// `Task<T>` — async task
    Task(Box<CSharpType>),
    /// `IEnumerable<T>`
    IEnumerable(Box<CSharpType>),
    /// `Func<T0, T1, ..., R>` — delegate type
    Func(Vec<CSharpType>, Box<CSharpType>),
    /// `Action<T0, T1, ...>` — void delegate
    Action(Vec<CSharpType>),
}
/// C# literal values.
#[derive(Debug, Clone, PartialEq)]
pub enum CSharpLit {
    /// Integer literal: `42`, `-7`
    Int(i64),
    /// Long literal: `42L`
    Long(i64),
    /// Boolean literal: `true` / `false`
    Bool(bool),
    /// String literal: `"hello"`
    Str(std::string::String),
    /// `null` literal
    Null,
    /// Float literal: `3.14f`
    Float(f64),
    /// Double literal: `3.14`
    Double(f64),
    /// Character literal: `'x'`
    Char(char),
}
/// A `catch` clause in a `try`/`catch` block.
#[derive(Debug, Clone, PartialEq)]
pub struct CSharpCatchClause {
    /// Exception type (e.g. `Exception`, `InvalidOperationException`)
    pub exception_type: CSharpType,
    /// Bound name for the exception variable
    pub var_name: std::string::String,
    /// Body statements
    pub stmts: Vec<CSharpStmt>,
}
/// A C# record (C# 9+).
/// `public record Foo(int X, string Y);`
#[derive(Debug, Clone, PartialEq)]
pub struct CSharpRecord {
    pub name: std::string::String,
    /// Positional parameters (primary constructor parameters)
    pub fields: Vec<(std::string::String, CSharpType)>,
    /// Additional methods
    pub methods: Vec<CSharpMethod>,
    /// `readonly` record (record struct)
    pub is_readonly: bool,
    /// `sealed` modifier
    pub is_sealed: bool,
    /// Base record to inherit from
    pub base_record: Option<std::string::String>,
    /// Implemented interfaces
    pub interfaces: Vec<std::string::String>,
    pub visibility: CSharpVisibility,
}
impl CSharpRecord {
    pub fn new(name: &str) -> Self {
        CSharpRecord {
            name: name.to_string(),
            fields: Vec::new(),
            methods: Vec::new(),
            is_readonly: false,
            is_sealed: false,
            base_record: None,
            interfaces: Vec::new(),
            visibility: CSharpVisibility::Public,
        }
    }
    /// Emit the record as C# source code.
    pub fn emit(&self, indent: &str) -> std::string::String {
        let inner = format!("{}    ", indent);
        let mut out = std::string::String::new();
        let mut mods = vec![format!("{}", self.visibility)];
        if self.is_sealed {
            mods.push("sealed".to_string());
        }
        if self.is_readonly {
            mods.push("readonly".to_string());
        }
        let fields_str = self
            .fields
            .iter()
            .map(|(n, t)| format!("{} {}", t, n))
            .collect::<Vec<_>>()
            .join(", ");
        let mut inherits: Vec<std::string::String> = Vec::new();
        if let Some(base) = &self.base_record {
            inherits.push(base.clone());
        }
        inherits.extend(self.interfaces.iter().cloned());
        let inherit_str = if inherits.is_empty() {
            std::string::String::new()
        } else {
            format!(" : {}", inherits.join(", "))
        };
        let record_kw = if self.is_readonly {
            "record struct"
        } else {
            "record"
        };
        if self.methods.is_empty() {
            let _ = writeln!(
                out,
                "{}{} {} {}({}){}",
                indent,
                mods.join(" "),
                record_kw,
                self.name,
                fields_str,
                inherit_str
            );
            if out.ends_with('\n') {
                out.pop();
                out.push(';');
                out.push('\n');
            }
        } else {
            let _ = writeln!(
                out,
                "{}{} {} {}({}){}",
                indent,
                mods.join(" "),
                record_kw,
                self.name,
                fields_str,
                inherit_str
            );
            let _ = writeln!(out, "{}{{", indent);
            for method in &self.methods {
                out.push_str(&method.emit(&inner));
            }
            let _ = writeln!(out, "{}}}", indent);
        }
        out
    }
}
/// C# code generation backend for OxiLean.
///
/// Transforms LCNF declarations into idiomatic C# 12 source code.
pub struct CSharpBackend {
    /// Whether to emit `public` visibility on generated declarations
    pub emit_public: bool,
    /// Whether to emit XML doc comments
    pub emit_comments: bool,
    /// Counter for fresh variable names
    pub(super) var_counter: u64,
    /// Whether to prefer async method generation
    pub prefer_async: bool,
}
impl CSharpBackend {
    /// Create a new backend with default settings.
    pub fn new() -> Self {
        CSharpBackend {
            emit_public: true,
            emit_comments: true,
            var_counter: 0,
            prefer_async: false,
        }
    }
    /// Generate a fresh local variable name.
    pub fn fresh_var(&mut self) -> std::string::String {
        let n = self.var_counter;
        self.var_counter += 1;
        format!("_cs{}", n)
    }
    /// Mangle an LCNF name into a valid C# identifier.
    pub fn mangle_name(name: &str) -> std::string::String {
        if name.is_empty() {
            return "ox_empty".to_string();
        }
        let mangled: std::string::String = name
            .chars()
            .map(|c| {
                if c.is_alphanumeric() || c == '_' {
                    c
                } else {
                    '_'
                }
            })
            .collect();
        if mangled
            .chars()
            .next()
            .map(|c| c.is_ascii_digit())
            .unwrap_or(false)
            || is_csharp_keyword(&mangled)
        {
            format!("ox_{}", mangled)
        } else {
            mangled
        }
    }
    /// Compile a single LCNF function declaration into a C# method.
    pub fn compile_decl(&self, decl: &LcnfFunDecl) -> CSharpMethod {
        let ret_ty = lcnf_type_to_csharp(&decl.ret_type);
        let mut method = CSharpMethod::new(&Self::mangle_name(&decl.name), ret_ty);
        method.visibility = if self.emit_public {
            CSharpVisibility::Public
        } else {
            CSharpVisibility::Private
        };
        method.is_static = true;
        for param in &decl.params {
            if param.erased {
                continue;
            }
            let param_ty = lcnf_type_to_csharp(&param.ty);
            let param_name = format!("_x{}", param.id.0);
            method.params.push((param_name, param_ty));
        }
        let mut stmts: Vec<CSharpStmt> = Vec::new();
        let result = self.compile_expr_to_stmts(&decl.body, &mut stmts);
        stmts.push(CSharpStmt::Return(Some(result)));
        method.body = stmts;
        method
    }
    /// Compile an LCNF expression, appending any needed statements to `stmts`,
    /// and returning a C# expression that yields the final value.
    pub(super) fn compile_expr_to_stmts(
        &self,
        expr: &LcnfExpr,
        stmts: &mut Vec<CSharpStmt>,
    ) -> CSharpExpr {
        match expr {
            LcnfExpr::Return(arg) => self.compile_arg(arg),
            LcnfExpr::Unreachable => CSharpExpr::Throw(Box::new(CSharpExpr::New {
                ty: CSharpType::Custom("InvalidOperationException".to_string()),
                args: vec![CSharpExpr::Lit(CSharpLit::Str(
                    "OxiLean: unreachable code reached".to_string(),
                ))],
            })),
            LcnfExpr::TailCall(func, args) => {
                let callee = self.compile_arg(func);
                let cs_args: Vec<CSharpExpr> = args.iter().map(|a| self.compile_arg(a)).collect();
                CSharpExpr::Call {
                    callee: Box::new(callee),
                    args: cs_args,
                }
            }
            LcnfExpr::Let {
                id, value, body, ..
            } => {
                let val_expr = self.compile_let_value(value);
                let var_name = format!("_x{}", id.0);
                stmts.push(CSharpStmt::LocalVar {
                    name: var_name,
                    ty: None,
                    init: Some(val_expr),
                    is_const: false,
                });
                self.compile_expr_to_stmts(body, stmts)
            }
            LcnfExpr::Case {
                scrutinee,
                alts,
                default,
                ..
            } => {
                let scrutinee_expr = CSharpExpr::Var(format!("_x{}", scrutinee.0));
                let tag_expr =
                    CSharpExpr::Member(Box::new(scrutinee_expr.clone()), "Tag".to_string());
                let result_var = format!("_cs_case{}", stmts.len());
                stmts.push(CSharpStmt::LocalVar {
                    name: result_var.clone(),
                    ty: Some(CSharpType::Object),
                    init: Some(CSharpExpr::Null),
                    is_const: false,
                });
                let mut cases: Vec<CSharpSwitchCase> = Vec::new();
                for alt in alts {
                    let mut branch_stmts: Vec<CSharpStmt> = Vec::new();
                    for (field_idx, param) in alt.params.iter().enumerate() {
                        if param.erased {
                            continue;
                        }
                        let field_access = CSharpExpr::Member(
                            Box::new(scrutinee_expr.clone()),
                            format!("Field{}", field_idx),
                        );
                        branch_stmts.push(CSharpStmt::LocalVar {
                            name: format!("_x{}", param.id.0),
                            ty: Some(lcnf_type_to_csharp(&param.ty)),
                            init: Some(field_access),
                            is_const: false,
                        });
                    }
                    let branch_result = self.compile_expr_to_stmts(&alt.body, &mut branch_stmts);
                    branch_stmts.push(CSharpStmt::Assign {
                        target: CSharpExpr::Var(result_var.clone()),
                        value: branch_result,
                    });
                    branch_stmts.push(CSharpStmt::Break);
                    cases.push(CSharpSwitchCase {
                        label: format!("{}", alt.ctor_tag),
                        stmts: branch_stmts,
                    });
                }
                let mut default_stmts: Vec<CSharpStmt> = Vec::new();
                if let Some(def) = default {
                    let def_result = self.compile_expr_to_stmts(def, &mut default_stmts);
                    default_stmts.push(CSharpStmt::Assign {
                        target: CSharpExpr::Var(result_var.clone()),
                        value: def_result,
                    });
                } else {
                    default_stmts.push(CSharpStmt::Throw(CSharpExpr::New {
                        ty: CSharpType::Custom("InvalidOperationException".to_string()),
                        args: vec![CSharpExpr::Lit(CSharpLit::Str(
                            "OxiLean: unreachable case".to_string(),
                        ))],
                    }));
                }
                stmts.push(CSharpStmt::Switch {
                    expr: tag_expr,
                    cases,
                    default: default_stmts,
                });
                CSharpExpr::Var(result_var)
            }
        }
    }
    /// Compile an LCNF let-value to a C# expression.
    pub(super) fn compile_let_value(&self, value: &LcnfLetValue) -> CSharpExpr {
        match value {
            LcnfLetValue::Lit(lit) => self.compile_lit(lit),
            LcnfLetValue::Erased => CSharpExpr::Null,
            LcnfLetValue::FVar(id) => CSharpExpr::Var(format!("_x{}", id.0)),
            LcnfLetValue::App(func, args) => {
                let callee = self.compile_arg(func);
                let cs_args: Vec<CSharpExpr> = args.iter().map(|a| self.compile_arg(a)).collect();
                CSharpExpr::Call {
                    callee: Box::new(callee),
                    args: cs_args,
                }
            }
            LcnfLetValue::Proj(_name, idx, var) => {
                let base = CSharpExpr::Var(format!("_x{}", var.0));
                CSharpExpr::Member(Box::new(base), format!("Field{}", idx))
            }
            LcnfLetValue::Ctor(name, _tag, args) => {
                let ctor_name = Self::mangle_name(name);
                let cs_args: Vec<CSharpExpr> = args.iter().map(|a| self.compile_arg(a)).collect();
                CSharpExpr::New {
                    ty: CSharpType::Custom(ctor_name),
                    args: cs_args,
                }
            }
            LcnfLetValue::Reset(_var) => CSharpExpr::Null,
            LcnfLetValue::Reuse(_slot, name, _tag, args) => {
                let ctor_name = Self::mangle_name(name);
                let cs_args: Vec<CSharpExpr> = args.iter().map(|a| self.compile_arg(a)).collect();
                CSharpExpr::New {
                    ty: CSharpType::Custom(ctor_name),
                    args: cs_args,
                }
            }
        }
    }
    /// Compile an LCNF argument to a C# expression.
    pub(super) fn compile_arg(&self, arg: &LcnfArg) -> CSharpExpr {
        match arg {
            LcnfArg::Var(id) => CSharpExpr::Var(format!("_x{}", id.0)),
            LcnfArg::Lit(lit) => self.compile_lit(lit),
            LcnfArg::Erased => CSharpExpr::Null,
            LcnfArg::Type(_) => CSharpExpr::Null,
        }
    }
    /// Compile an LCNF literal to a C# expression.
    pub(super) fn compile_lit(&self, lit: &LcnfLit) -> CSharpExpr {
        match lit {
            LcnfLit::Nat(n) => CSharpExpr::Lit(CSharpLit::Long(*n as i64)),
            LcnfLit::Str(s) => CSharpExpr::Lit(CSharpLit::Str(s.clone())),
        }
    }
    /// Compile a complete list of LCNF declarations into a `CSharpModule`.
    pub fn emit_module(&self, namespace: &str, decls: &[LcnfFunDecl]) -> CSharpModule {
        let mut module = CSharpModule::new(namespace);
        module.header_comment = Some(format!(
            "OxiLean-generated C# module: {}\nGenerated by OxiLean CSharpBackend",
            namespace
        ));
        module.add_using("System");
        module.add_using("System.Collections.Generic");
        module.add_using("System.Linq");
        module.add_using("System.Threading.Tasks");
        let mut runtime_class = CSharpClass::new("OxiLeanRuntime");
        runtime_class.is_static = true;
        runtime_class.visibility = CSharpVisibility::Internal;
        for decl in decls {
            let method = self.compile_decl(decl);
            runtime_class.methods.push(method);
        }
        module.classes.push(runtime_class);
        module
    }
}
/// One arm in a `switch` expression.
#[derive(Debug, Clone, PartialEq)]
pub struct CSharpSwitchArm {
    /// Pattern string (e.g. `Foo(var a, var b)`, `> 0`, `_`)
    pub pattern: std::string::String,
    /// Optional guard: `when condition`
    pub guard: Option<CSharpExpr>,
    /// Result expression
    pub body: CSharpExpr,
}
