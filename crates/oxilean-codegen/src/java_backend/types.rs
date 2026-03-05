//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use std::collections::HashSet;

use super::functions::JAVA_KEYWORDS;

use super::functions::*;
use std::collections::{HashMap, VecDeque};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JavaWorklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
impl JavaWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        JavaWorklist {
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
#[derive(Debug, Clone, Default)]
pub struct JavaPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
impl JavaPassStats {
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
/// A Java method definition.
#[derive(Debug, Clone)]
pub struct JavaMethod {
    pub name: std::string::String,
    pub return_type: JavaType,
    pub params: Vec<(std::string::String, JavaType)>,
    pub body: Vec<JavaStmt>,
    pub visibility: Visibility,
    pub is_static: bool,
    pub is_final: bool,
    pub is_abstract: bool,
    pub annotations: Vec<std::string::String>,
    /// Checked exceptions declared in `throws`
    pub throws: Vec<std::string::String>,
}
impl JavaMethod {
    /// Create a simple public non-static method.
    pub fn new(
        name: &str,
        return_type: JavaType,
        params: Vec<(&str, JavaType)>,
        body: Vec<JavaStmt>,
    ) -> Self {
        JavaMethod {
            name: name.to_string(),
            return_type,
            params: params
                .into_iter()
                .map(|(n, t)| (n.to_string(), t))
                .collect(),
            body,
            visibility: Visibility::Public,
            is_static: false,
            is_final: false,
            is_abstract: false,
            annotations: Vec::new(),
            throws: Vec::new(),
        }
    }
}
/// A complete Java compilation unit (one `.java` file).
#[derive(Debug, Clone)]
pub struct JavaModule {
    pub package: std::string::String,
    pub imports: Vec<std::string::String>,
    pub classes: Vec<JavaClass>,
    pub interfaces: Vec<SealedInterface>,
    pub records: Vec<JavaRecord>,
    pub enums: Vec<JavaEnum>,
}
impl JavaModule {
    /// Create a new empty module in the given package.
    pub fn new(package: &str) -> Self {
        JavaModule {
            package: package.to_string(),
            imports: Vec::new(),
            classes: Vec::new(),
            interfaces: Vec::new(),
            records: Vec::new(),
            enums: Vec::new(),
        }
    }
    /// Emit a complete Java source file as a `String`.
    pub fn emit(&self) -> std::string::String {
        let mut out = std::string::String::new();
        if !self.package.is_empty() {
            out.push_str(&format!("package {};\n\n", self.package));
        }
        for imp in &self.imports {
            out.push_str(&format!("import {};\n", imp));
        }
        if !self.imports.is_empty() {
            out.push('\n');
        }
        for iface in &self.interfaces {
            emit_sealed_interface(&mut out, iface, 0);
            out.push('\n');
        }
        for rec in &self.records {
            emit_record(&mut out, rec, 0);
            out.push('\n');
        }
        for en in &self.enums {
            emit_enum(&mut out, en, 0);
            out.push('\n');
        }
        for cls in &self.classes {
            emit_class(&mut out, cls, 0);
            out.push('\n');
        }
        out
    }
}
/// Java code generation backend.
pub struct JavaBackend {
    pub(super) var_counter: u64,
}
impl JavaBackend {
    /// Create a new `JavaBackend`.
    pub fn new() -> Self {
        JavaBackend { var_counter: 0 }
    }
    /// Mangle a name so it does not clash with Java keywords or invalid characters.
    pub fn mangle_name(&self, name: &str) -> std::string::String {
        let sanitized: std::string::String = name
            .chars()
            .map(|c| match c {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => c,
                '.' | ':' | '\'' | '!' | '?' | '@' => '_',
                _ => '_',
            })
            .collect();
        let sanitized = if sanitized.starts_with(|c: char| c.is_ascii_digit()) {
            format!("_{}", sanitized)
        } else {
            sanitized
        };
        if JAVA_KEYWORDS.contains(&sanitized.as_str()) {
            format!("{}_", sanitized)
        } else if sanitized.is_empty() {
            "_anon".to_string()
        } else {
            sanitized
        }
    }
    /// Generate a fresh temporary variable name.
    pub fn fresh_var(&mut self) -> std::string::String {
        let v = self.var_counter;
        self.var_counter += 1;
        format!("_t{}", v)
    }
    /// Compile a slice of LCNF declarations into a complete Java source string.
    pub fn emit_module(decls: &[LcnfFunDecl]) -> Result<std::string::String, std::string::String> {
        let mut backend = JavaBackend::new();
        let mut methods = Vec::new();
        let mut ctor_names: HashSet<std::string::String> = HashSet::new();
        for decl in decls {
            collect_ctor_names_from_expr(&decl.body, &mut ctor_names);
        }
        let mut fun_class = JavaClass::new("OxiLeanGenerated");
        fun_class
            .annotations
            .push("@SuppressWarnings(\"all\")".to_string());
        for decl in decls {
            let m = backend.compile_decl(decl)?;
            methods.push(m);
        }
        fun_class.methods = methods;
        let mut records: Vec<JavaRecord> = ctor_names
            .into_iter()
            .collect::<Vec<_>>()
            .into_iter()
            .map(|name| {
                let mangled = backend.mangle_name(&name);
                JavaRecord {
                    name: mangled,
                    components: vec![("tag".to_string(), JavaType::Int)],
                    methods: Vec::new(),
                    is_sealed: false,
                    implements: Vec::new(),
                    annotations: Vec::new(),
                }
            })
            .collect();
        records.sort_by(|a, b| a.name.cmp(&b.name));
        let mut module = JavaModule::new("oxilean.generated");
        module.imports = vec![
            "java.util.List".to_string(),
            "java.util.Map".to_string(),
            "java.util.Optional".to_string(),
            "java.util.function.Function".to_string(),
            "java.util.function.Supplier".to_string(),
            "java.util.stream.Collectors".to_string(),
        ];
        module.records = records;
        module.classes = vec![fun_class];
        Ok(module.emit())
    }
    /// Compile a single LCNF function declaration to a `JavaMethod`.
    pub fn compile_decl(&mut self, decl: &LcnfFunDecl) -> Result<JavaMethod, std::string::String> {
        let name = self.mangle_name(&decl.name.to_string());
        let params: Vec<(std::string::String, JavaType)> = decl
            .params
            .iter()
            .map(|p| (self.mangle_name(&p.name), lcnf_type_to_java(&p.ty)))
            .collect();
        let return_type = lcnf_type_to_java(&decl.ret_type);
        let mut body: Vec<JavaStmt> = Vec::new();
        let result_expr = self.compile_expr(&decl.body, &mut body)?;
        match &return_type {
            JavaType::Void => {
                body.push(JavaStmt::Expr(result_expr));
            }
            _ => {
                body.push(JavaStmt::Return(Some(result_expr)));
            }
        }
        let mut method = JavaMethod {
            name,
            return_type,
            params: params.into_iter().collect(),
            body,
            visibility: Visibility::Public,
            is_static: true,
            is_final: false,
            is_abstract: false,
            annotations: Vec::new(),
            throws: Vec::new(),
        };
        method
            .annotations
            .push("@SuppressWarnings(\"unchecked\")".to_string());
        Ok(method)
    }
    /// Compile an LCNF expression, emitting binding statements into `stmts`.
    pub fn compile_expr(
        &mut self,
        expr: &LcnfExpr,
        stmts: &mut Vec<JavaStmt>,
    ) -> Result<JavaExpr, std::string::String> {
        match expr {
            LcnfExpr::Return(arg) => Ok(self.compile_arg(arg)),
            LcnfExpr::Unreachable => Ok(JavaExpr::MethodCall(
                Box::new(JavaExpr::Var("OxiLeanRuntime".to_string())),
                "unreachable".to_string(),
                vec![],
            )),
            LcnfExpr::TailCall(func, args) => {
                let callee = self.compile_arg(func);
                let java_args: Vec<JavaExpr> = args.iter().map(|a| self.compile_arg(a)).collect();
                Ok(JavaExpr::Call(Box::new(callee), java_args))
            }
            LcnfExpr::Let {
                id: _,
                name,
                ty,
                value,
                body,
            } => {
                let java_val = self.compile_let_value(value)?;
                let var_name = self.mangle_name(name);
                let java_ty = lcnf_type_to_java(ty);
                stmts.push(JavaStmt::LocalVar {
                    ty: Some(java_ty),
                    name: var_name.clone(),
                    init: Some(java_val),
                    is_final: true,
                });
                self.compile_expr(body, stmts)
            }
            LcnfExpr::Case {
                scrutinee,
                scrutinee_ty: _,
                alts,
                default,
            } => {
                let result_var = self.fresh_var();
                let scrutinee_expr = JavaExpr::Var(format!("_x{}", scrutinee.0));
                stmts.push(JavaStmt::LocalVar {
                    ty: Some(JavaType::Object),
                    name: result_var.clone(),
                    init: Some(JavaExpr::Null),
                    is_final: false,
                });
                let mut cases: Vec<(JavaExpr, Vec<JavaStmt>)> = Vec::new();
                for alt in alts {
                    let mut branch_stmts: Vec<JavaStmt> = Vec::new();
                    for (idx, param) in alt.params.iter().enumerate() {
                        let param_name = self.mangle_name(&param.name);
                        let field_access = JavaExpr::FieldAccess(
                            Box::new(JavaExpr::Var(format!("_x{}", scrutinee.0))),
                            format!("field{}", idx),
                        );
                        branch_stmts.push(JavaStmt::LocalVar {
                            ty: Some(lcnf_type_to_java(&param.ty)),
                            name: param_name,
                            init: Some(field_access),
                            is_final: true,
                        });
                    }
                    let branch_result = self.compile_expr(&alt.body, &mut branch_stmts)?;
                    branch_stmts.push(JavaStmt::Expr(JavaExpr::BinOp(
                        "=".to_string(),
                        Box::new(JavaExpr::Var(result_var.clone())),
                        Box::new(branch_result),
                    )));
                    branch_stmts.push(JavaStmt::Break(None));
                    let tag_label = JavaExpr::Lit(JavaLit::Int(alt.ctor_tag as i64));
                    cases.push((tag_label, branch_stmts));
                }
                let mut default_stmts: Vec<JavaStmt> = Vec::new();
                if let Some(def) = default {
                    let def_result = self.compile_expr(def, &mut default_stmts)?;
                    default_stmts.push(JavaStmt::Expr(JavaExpr::BinOp(
                        "=".to_string(),
                        Box::new(JavaExpr::Var(result_var.clone())),
                        Box::new(def_result),
                    )));
                } else {
                    default_stmts.push(JavaStmt::Throw(JavaExpr::New(
                        "IllegalStateException".to_string(),
                        vec![JavaExpr::Lit(JavaLit::Str(
                            "OxiLean: unreachable".to_string(),
                        ))],
                    )));
                }
                let discriminant =
                    JavaExpr::FieldAccess(Box::new(scrutinee_expr), "tag".to_string());
                stmts.push(JavaStmt::Switch {
                    scrutinee: discriminant,
                    cases,
                    default: default_stmts,
                });
                Ok(JavaExpr::Var(result_var))
            }
        }
    }
    /// Compile an LCNF let-value to a Java expression.
    pub(super) fn compile_let_value(
        &mut self,
        value: &LcnfLetValue,
    ) -> Result<JavaExpr, std::string::String> {
        match value {
            LcnfLetValue::Lit(lit) => Ok(self.compile_lit(lit)),
            LcnfLetValue::Erased => Ok(JavaExpr::Null),
            LcnfLetValue::FVar(id) => Ok(JavaExpr::Var(format!("_x{}", id.0))),
            LcnfLetValue::App(func, args) => {
                let callee = self.compile_arg(func);
                let java_args: Vec<JavaExpr> = args.iter().map(|a| self.compile_arg(a)).collect();
                Ok(JavaExpr::Call(Box::new(callee), java_args))
            }
            LcnfLetValue::Proj(_name, idx, var) => {
                let base = JavaExpr::Var(format!("_x{}", var.0));
                Ok(JavaExpr::FieldAccess(
                    Box::new(base),
                    format!("field{}", idx),
                ))
            }
            LcnfLetValue::Ctor(name, _tag, args) => {
                let ctor_name = self.mangle_name(name);
                let java_args: Vec<JavaExpr> = args.iter().map(|a| self.compile_arg(a)).collect();
                Ok(JavaExpr::New(ctor_name, java_args))
            }
            LcnfLetValue::Reset(_var) => Ok(JavaExpr::Null),
            LcnfLetValue::Reuse(_slot, name, _tag, args) => {
                let ctor_name = self.mangle_name(name);
                let java_args: Vec<JavaExpr> = args.iter().map(|a| self.compile_arg(a)).collect();
                Ok(JavaExpr::New(ctor_name, java_args))
            }
        }
    }
    /// Compile an LCNF argument to a Java expression.
    pub(super) fn compile_arg(&self, arg: &LcnfArg) -> JavaExpr {
        match arg {
            LcnfArg::Var(id) => JavaExpr::Var(format!("_x{}", id.0)),
            LcnfArg::Lit(lit) => self.compile_lit(lit),
            LcnfArg::Erased => JavaExpr::Null,
            LcnfArg::Type(_) => JavaExpr::Null,
        }
    }
    /// Compile an LCNF literal.
    pub(super) fn compile_lit(&self, lit: &LcnfLit) -> JavaExpr {
        match lit {
            LcnfLit::Nat(n) => JavaExpr::Lit(JavaLit::Long(*n as i64)),
            LcnfLit::Str(s) => JavaExpr::Lit(JavaLit::Str(s.clone())),
        }
    }
}
/// Java statement AST.
#[derive(Debug, Clone, PartialEq)]
pub enum JavaStmt {
    /// Expression statement: `expr;`
    Expr(JavaExpr),
    /// Local variable declaration: `Type name = init;` or `var name = init;`
    LocalVar {
        ty: Option<JavaType>,
        name: std::string::String,
        init: Option<JavaExpr>,
        is_final: bool,
    },
    /// If statement: `if (cond) { then } else { else_ }`
    If(JavaExpr, Vec<JavaStmt>, Vec<JavaStmt>),
    /// Switch expression/statement
    Switch {
        scrutinee: JavaExpr,
        cases: Vec<(JavaExpr, Vec<JavaStmt>)>,
        default: Vec<JavaStmt>,
    },
    /// Classic for loop: `for (init; cond; update) { body }`
    For {
        init: Option<Box<JavaStmt>>,
        cond: Option<JavaExpr>,
        update: Option<JavaExpr>,
        body: Vec<JavaStmt>,
    },
    /// Enhanced for loop: `for (Type elem : iterable) { body }`
    ForEach {
        ty: JavaType,
        elem: std::string::String,
        iterable: JavaExpr,
        body: Vec<JavaStmt>,
    },
    /// While loop: `while (cond) { body }`
    While(JavaExpr, Vec<JavaStmt>),
    /// Do-while loop: `do { body } while (cond);`
    DoWhile(Vec<JavaStmt>, JavaExpr),
    /// Return statement: `return expr;` or `return;`
    Return(Option<JavaExpr>),
    /// Throw statement: `throw expr;`
    Throw(JavaExpr),
    /// Try-catch-finally
    TryCatch {
        body: Vec<JavaStmt>,
        catches: Vec<JavaCatchClause>,
        finally: Vec<JavaStmt>,
    },
    /// Try-with-resources
    TryWithResources {
        resources: Vec<(std::string::String, JavaExpr)>,
        body: Vec<JavaStmt>,
        catches: Vec<JavaCatchClause>,
        finally: Vec<JavaStmt>,
    },
    /// Synchronized block: `synchronized (lock) { body }`
    Synchronized(JavaExpr, Vec<JavaStmt>),
    /// Break statement
    Break(Option<std::string::String>),
    /// Continue statement
    Continue(Option<std::string::String>),
    /// Assert statement: `assert cond : msg;`
    Assert(JavaExpr, Option<JavaExpr>),
}
/// Modifiers that can appear on a class.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClassModifier {
    Sealed,
    Abstract,
    Final,
    Static,
    NonSealed,
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum JavaPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl JavaPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            JavaPassPhase::Analysis => "analysis",
            JavaPassPhase::Transformation => "transformation",
            JavaPassPhase::Verification => "verification",
            JavaPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(self, JavaPassPhase::Transformation | JavaPassPhase::Cleanup)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JavaPassConfig {
    pub phase: JavaPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
impl JavaPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: JavaPassPhase) -> Self {
        JavaPassConfig {
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
/// Java access modifiers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Visibility {
    Public,
    Protected,
    Private,
    /// Package-private (no modifier)
    Package,
}
/// A Java enum constant.
#[derive(Debug, Clone)]
pub struct JavaEnumConstant {
    pub name: std::string::String,
    pub args: Vec<JavaExpr>,
    pub annotations: Vec<std::string::String>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JavaDepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
}
impl JavaDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        JavaDepGraph {
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
#[derive(Debug, Clone)]
pub struct JavaCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
/// A single catch clause in a try-catch.
#[derive(Debug, Clone, PartialEq)]
pub struct JavaCatchClause {
    /// Exception type(s) (e.g. `"IOException"` or `"IOException | SQLException"`)
    pub exception_types: Vec<std::string::String>,
    /// Bound variable name
    pub var_name: std::string::String,
    /// Handler body
    pub body: Vec<JavaStmt>,
}
/// Java expression AST.
#[derive(Debug, Clone, PartialEq)]
pub enum JavaExpr {
    /// Literal value: `42`, `"hello"`, `null`
    Lit(JavaLit),
    /// Variable or identifier: `x`, `MyClass`
    Var(std::string::String),
    /// Binary operation: `a + b`
    BinOp(std::string::String, Box<JavaExpr>, Box<JavaExpr>),
    /// Unary operation: `!x`, `-n`
    UnaryOp(std::string::String, Box<JavaExpr>),
    /// Static or free function call: `f(a, b)`
    Call(Box<JavaExpr>, Vec<JavaExpr>),
    /// Instance method call: `obj.method(args...)`
    MethodCall(Box<JavaExpr>, std::string::String, Vec<JavaExpr>),
    /// Object instantiation: `new Foo(args...)`
    New(std::string::String, Vec<JavaExpr>),
    /// Cast: `(Type) expr`
    Cast(JavaType, Box<JavaExpr>),
    /// Instanceof check: `expr instanceof Type`
    Instanceof(Box<JavaExpr>, std::string::String),
    /// Ternary: `cond ? then : else`
    Ternary(Box<JavaExpr>, Box<JavaExpr>, Box<JavaExpr>),
    /// Null literal (convenience variant)
    Null,
    /// Lambda expression: `(x, y) -> expr`
    Lambda(Vec<std::string::String>, Box<JavaExpr>),
    /// Method reference: `Class::method`
    MethodRef(std::string::String, std::string::String),
    /// Array access: `arr[idx]`
    ArrayAccess(Box<JavaExpr>, Box<JavaExpr>),
    /// Field access: `obj.field`
    FieldAccess(Box<JavaExpr>, std::string::String),
}
/// A field declaration in a Java class.
#[derive(Debug, Clone)]
pub struct JavaField {
    pub name: std::string::String,
    pub ty: JavaType,
    pub init: Option<JavaExpr>,
    pub visibility: Visibility,
    pub is_static: bool,
    pub is_final: bool,
    pub annotations: Vec<std::string::String>,
}
/// A Java class declaration.
#[derive(Debug, Clone)]
pub struct JavaClass {
    pub name: std::string::String,
    pub superclass: Option<std::string::String>,
    pub interfaces: Vec<std::string::String>,
    pub fields: Vec<JavaField>,
    pub methods: Vec<JavaMethod>,
    pub inner_classes: Vec<JavaClass>,
    pub modifiers: Vec<ClassModifier>,
    pub annotations: Vec<std::string::String>,
    pub type_params: Vec<std::string::String>,
    pub visibility: Visibility,
    /// Permitted subclasses (for sealed classes)
    pub permits: Vec<std::string::String>,
}
impl JavaClass {
    /// Create a simple public class.
    pub fn new(name: &str) -> Self {
        JavaClass {
            name: name.to_string(),
            superclass: None,
            interfaces: Vec::new(),
            fields: Vec::new(),
            methods: Vec::new(),
            inner_classes: Vec::new(),
            modifiers: Vec::new(),
            annotations: Vec::new(),
            type_params: Vec::new(),
            visibility: Visibility::Public,
            permits: Vec::new(),
        }
    }
}
/// A Java enum declaration.
#[derive(Debug, Clone)]
pub struct JavaEnum {
    pub name: std::string::String,
    pub constants: Vec<JavaEnumConstant>,
    pub fields: Vec<JavaField>,
    pub methods: Vec<JavaMethod>,
    pub interfaces: Vec<std::string::String>,
    pub visibility: Visibility,
    pub annotations: Vec<std::string::String>,
}
impl JavaEnum {
    /// Create a simple public enum with named constants.
    pub fn new(name: &str, constants: Vec<&str>) -> Self {
        JavaEnum {
            name: name.to_string(),
            constants: constants
                .into_iter()
                .map(|c| JavaEnumConstant {
                    name: c.to_string(),
                    args: Vec::new(),
                    annotations: Vec::new(),
                })
                .collect(),
            fields: Vec::new(),
            methods: Vec::new(),
            interfaces: Vec::new(),
            visibility: Visibility::Public,
            annotations: Vec::new(),
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JavaDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
impl JavaDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        JavaDominatorTree {
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
pub struct JavaPassRegistry {
    pub(super) configs: Vec<JavaPassConfig>,
    pub(super) stats: std::collections::HashMap<String, JavaPassStats>,
}
impl JavaPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        JavaPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: JavaPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), JavaPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&JavaPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&JavaPassStats> {
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
pub struct JavaConstantFoldingHelper;
impl JavaConstantFoldingHelper {
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
/// A sealed interface declaration (Java 17+).
///
/// ```java
/// public sealed interface Expr permits Lit, Add, Mul {}
/// ```
#[derive(Debug, Clone)]
pub struct SealedInterface {
    pub name: std::string::String,
    /// Permitted subclasses/records
    pub permits: Vec<std::string::String>,
    /// Methods defined in the interface (default or abstract)
    pub methods: Vec<JavaMethod>,
    /// Annotations
    pub annotations: Vec<std::string::String>,
    /// Extended interfaces
    pub extends: Vec<std::string::String>,
}
impl SealedInterface {
    /// Create a simple sealed interface.
    pub fn new(name: &str, permits: Vec<&str>) -> Self {
        SealedInterface {
            name: name.to_string(),
            permits: permits.into_iter().map(|s| s.to_string()).collect(),
            methods: Vec::new(),
            annotations: Vec::new(),
            extends: Vec::new(),
        }
    }
}
/// A Java record declaration (Java 16+).
///
/// ```java
/// public record Point(int x, int y) {}
/// ```
#[derive(Debug, Clone)]
pub struct JavaRecord {
    pub name: std::string::String,
    /// Record components (name, type)
    pub components: Vec<(std::string::String, JavaType)>,
    /// Additional methods defined inside the record
    pub methods: Vec<JavaMethod>,
    /// Whether the record is `sealed` (Java 17+)
    pub is_sealed: bool,
    /// Interfaces this record implements
    pub implements: Vec<std::string::String>,
    /// Annotations
    pub annotations: Vec<std::string::String>,
}
impl JavaRecord {
    /// Create a simple public record.
    pub fn new(name: &str, components: Vec<(&str, JavaType)>) -> Self {
        JavaRecord {
            name: name.to_string(),
            components: components
                .into_iter()
                .map(|(n, t)| (n.to_string(), t))
                .collect(),
            methods: Vec::new(),
            is_sealed: false,
            implements: Vec::new(),
            annotations: Vec::new(),
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JavaAnalysisCache {
    pub(super) entries: std::collections::HashMap<String, JavaCacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
impl JavaAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        JavaAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&JavaCacheEntry> {
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
            JavaCacheEntry {
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
/// Java type representation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum JavaType {
    /// `int` — 32-bit signed integer
    Int,
    /// `long` — 64-bit signed integer
    Long,
    /// `double` — 64-bit floating point
    Double,
    /// `float` — 32-bit floating point
    Float,
    /// `boolean`
    Boolean,
    /// `char`
    Char,
    /// `byte`
    Byte,
    /// `short`
    Short,
    /// `void`
    Void,
    /// `String`
    String,
    /// `Object`
    Object,
    /// `T[]` — Java array
    Array(Box<JavaType>),
    /// `List<T>`
    List(Box<JavaType>),
    /// `Map<K, V>`
    Map(Box<JavaType>, Box<JavaType>),
    /// `Optional<T>`
    Optional(Box<JavaType>),
    /// User-defined named type
    Custom(std::string::String),
    /// Generic instantiation: `MyType<A, B>`
    Generic(std::string::String, Vec<JavaType>),
}
/// Java literal values.
#[derive(Debug, Clone, PartialEq)]
pub enum JavaLit {
    /// Integer literal: `42`
    Int(i64),
    /// Long literal: `42L`
    Long(i64),
    /// Double literal: `3.14`
    Double(f64),
    /// Float literal: `3.14f`
    Float(f64),
    /// Boolean literal: `true` / `false`
    Bool(bool),
    /// Char literal: `'a'`
    Char(char),
    /// String literal: `"hello"`
    Str(std::string::String),
    /// Null literal: `null`
    Null,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JavaLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
impl JavaLivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        JavaLivenessInfo {
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
