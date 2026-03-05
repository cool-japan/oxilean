//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet, VecDeque};

/// The OCaml code generation backend.
pub struct OcamlBackend {
    pub(super) module: OcamlModule,
}
impl OcamlBackend {
    /// Create a new OCaml backend for the given module name.
    pub fn new(module_name: &str) -> Self {
        OcamlBackend {
            module: OcamlModule::new(module_name),
        }
    }
    /// Add a definition to the generated module.
    pub fn add_definition(&mut self, def: OcamlDefinition) {
        self.module.add(def);
    }
    /// Build a variant type definition for an algebraic data type.
    ///
    /// Example:
    /// ```text
    /// type expr =
    ///   | Lit of int
    ///   | Add of expr * expr
    ///   | Mul of expr * expr
    /// ```
    pub fn make_adt(
        &self,
        name: &str,
        type_params: Vec<&str>,
        variants: Vec<(&str, Vec<OcamlType>)>,
    ) -> OcamlTypeDef {
        OcamlTypeDef {
            name: name.to_string(),
            type_params: type_params.iter().map(|s| s.to_string()).collect(),
            decl: OcamlTypeDecl::Variant(
                variants
                    .into_iter()
                    .map(|(n, ts)| (n.to_string(), ts))
                    .collect(),
            ),
        }
    }
    /// Build a tail-recursive list fold (common OCaml pattern).
    ///
    /// Generates: `let rec fold_left f acc = function | [] -> acc | x::xs -> fold_left f (f acc x) xs`
    pub fn make_fold_left(&self, name: &str) -> OcamlLetBinding {
        OcamlLetBinding {
            is_rec: true,
            name: name.to_string(),
            params: vec![("f".to_string(), None), ("acc".to_string(), None)],
            body: OcamlExpr::Match(
                Box::new(OcamlExpr::Var("lst".to_string())),
                vec![
                    (
                        OcamlPattern::List(vec![]),
                        OcamlExpr::Var("acc".to_string()),
                    ),
                    (
                        OcamlPattern::Cons(
                            Box::new(OcamlPattern::Var("x".to_string())),
                            Box::new(OcamlPattern::Var("xs".to_string())),
                        ),
                        OcamlExpr::App(
                            Box::new(OcamlExpr::Var(name.to_string())),
                            vec![
                                OcamlExpr::Var("f".to_string()),
                                OcamlExpr::App(
                                    Box::new(OcamlExpr::Var("f".to_string())),
                                    vec![
                                        OcamlExpr::Var("acc".to_string()),
                                        OcamlExpr::Var("x".to_string()),
                                    ],
                                ),
                                OcamlExpr::Var("xs".to_string()),
                            ],
                        ),
                    ),
                ],
            ),
            type_annotation: None,
        }
    }
    /// Emit the full `.ml` implementation.
    pub fn emit_module(&self) -> std::string::String {
        self.module.emit()
    }
    /// Emit the `.mli` interface file.
    pub fn emit_mli(&self) -> std::string::String {
        self.module.emit_mli()
    }
}
/// OCaml Bigarray layout.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BigarrayLayout {
    CLayout,
    FortranLayout,
}
#[allow(dead_code)]
impl BigarrayLayout {
    /// Return the layout constant name.
    pub fn layout_name(&self) -> &'static str {
        match self {
            BigarrayLayout::CLayout => "Bigarray.c_layout",
            BigarrayLayout::FortranLayout => "Bigarray.fortran_layout",
        }
    }
}
/// A single item in an OCaml module signature.
#[derive(Debug, Clone)]
pub enum OcamlSigItem {
    /// `val name : ty`
    Val(std::string::String, OcamlType),
    /// `type t` or full type decl
    Type(OcamlTypeDef),
    /// `module M : S`
    Module(std::string::String, std::string::String),
    /// `exception E of ty`
    Exception(std::string::String, Option<OcamlType>),
}
#[allow(dead_code)]
pub struct OCamlPassRegistry {
    pub(super) configs: Vec<OCamlPassConfig>,
    pub(super) stats: std::collections::HashMap<String, OCamlPassStats>,
}
impl OCamlPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        OCamlPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: OCamlPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), OCamlPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&OCamlPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&OCamlPassStats> {
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
/// OCaml ppx attribute node.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OcamlPpxAttr {
    /// Attribute name (e.g. "deriving", "inline", "unrolled").
    pub name: std::string::String,
    /// Attribute payload (optional).
    pub payload: Option<std::string::String>,
}
#[allow(dead_code)]
impl OcamlPpxAttr {
    /// Create a new ppx attribute.
    pub fn new(name: &str) -> Self {
        OcamlPpxAttr {
            name: name.to_string(),
            payload: None,
        }
    }
    /// Set the payload.
    pub fn with_payload(mut self, payload: &str) -> Self {
        self.payload = Some(payload.to_string());
        self
    }
    /// Deriving attribute: `[@deriving show, eq]`
    pub fn deriving(traits: &[&str]) -> Self {
        OcamlPpxAttr {
            name: "deriving".to_string(),
            payload: Some(traits.join(", ")),
        }
    }
    /// Emit the attribute.
    pub fn emit(&self) -> std::string::String {
        match &self.payload {
            None => format!("[@{}]", self.name),
            Some(p) => format!("[@{} {}]", self.name, p),
        }
    }
    /// Emit as a double-bracket attribute.
    pub fn emit_double(&self) -> std::string::String {
        match &self.payload {
            None => format!("[@@{}]", self.name),
            Some(p) => format!("[@@{} {}]", self.name, p),
        }
    }
}
/// An OCaml 5.x effect declaration.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OcamlEffect {
    /// Effect name (e.g. "State", "Async", "Fail").
    pub name: std::string::String,
    /// Effect continuation type.
    pub params: Vec<OcamlType>,
    /// Return type of the effect continuation.
    pub ret: OcamlType,
}
#[allow(dead_code)]
impl OcamlEffect {
    /// Create a new effect declaration.
    pub fn new(name: &str, params: Vec<OcamlType>, ret: OcamlType) -> Self {
        OcamlEffect {
            name: name.to_string(),
            params,
            ret,
        }
    }
    /// Emit the effect type declaration.
    pub fn emit_decl(&self) -> std::string::String {
        if self.params.is_empty() {
            format!("type _ Effect.t += {} : {}", self.name, self.ret)
        } else {
            let params_str: Vec<std::string::String> =
                self.params.iter().map(|t| t.to_string()).collect();
            format!(
                "type _ Effect.t += {} : {} -> {}",
                self.name,
                params_str.join(" -> "),
                self.ret
            )
        }
    }
    /// Emit an `Effect.perform` call.
    pub fn emit_perform(&self, args: &[&str]) -> std::string::String {
        if args.is_empty() {
            format!("Effect.perform {}", self.name)
        } else {
            format!("Effect.perform ({} {})", self.name, args.join(" "))
        }
    }
    /// Emit a handler for this effect.
    pub fn emit_handler_arm(&self, handler_body: &str) -> std::string::String {
        format!(
            "| Effect ({name} v), k -> {body}",
            name = self.name,
            body = handler_body
        )
    }
}
/// A Dune library stanza.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DuneLibrary {
    /// Library name.
    pub name: std::string::String,
    /// Public name (for installed packages).
    pub public_name: Option<std::string::String>,
    /// Module names.
    pub modules: Vec<std::string::String>,
    /// Library dependencies.
    pub libraries: Vec<std::string::String>,
    /// Preprocessor directives.
    pub preprocess: Vec<std::string::String>,
    /// Flags passed to ocamlopt/ocamlc.
    pub ocamlopt_flags: Vec<std::string::String>,
    /// Whether to wrap modules.
    pub wrapped: bool,
    /// Inline tests.
    pub inline_tests: bool,
}
#[allow(dead_code)]
impl DuneLibrary {
    /// Create a new Dune library stanza.
    pub fn new(name: &str) -> Self {
        DuneLibrary {
            name: name.to_string(),
            public_name: None,
            modules: vec![],
            libraries: vec![],
            preprocess: vec![],
            ocamlopt_flags: vec![],
            wrapped: true,
            inline_tests: false,
        }
    }
    /// Set public name.
    pub fn public_name(mut self, name: &str) -> Self {
        self.public_name = Some(name.to_string());
        self
    }
    /// Add a module.
    pub fn add_module(mut self, module: &str) -> Self {
        self.modules.push(module.to_string());
        self
    }
    /// Add a library dependency.
    pub fn add_dep(mut self, dep: &str) -> Self {
        self.libraries.push(dep.to_string());
        self
    }
    /// Add ppx preprocessor.
    pub fn add_ppx(mut self, ppx: &str) -> Self {
        self.preprocess.push(format!("(pps {})", ppx));
        self
    }
    /// Disable module wrapping.
    pub fn unwrapped(mut self) -> Self {
        self.wrapped = false;
        self
    }
    /// Enable inline tests.
    pub fn with_inline_tests(mut self) -> Self {
        self.inline_tests = true;
        self
    }
    /// Emit the Dune library stanza.
    pub fn emit(&self) -> std::string::String {
        let mut lines = vec!["(library".to_string()];
        lines.push(format!(" (name {})", self.name));
        if let Some(pub_name) = &self.public_name {
            lines.push(format!(" (public_name {})", pub_name));
        }
        if !self.modules.is_empty() {
            lines.push(format!(" (modules {})", self.modules.join(" ")));
        }
        if !self.libraries.is_empty() {
            lines.push(format!(" (libraries {})", self.libraries.join(" ")));
        }
        if !self.preprocess.is_empty() {
            lines.push(format!(" (preprocess {})", self.preprocess.join(" ")));
        }
        if !self.wrapped {
            lines.push(" (wrapped false)".to_string());
        }
        if self.inline_tests {
            lines.push(" (inline_tests)".to_string());
        }
        lines.push(")".to_string());
        lines.join("\n")
    }
}
/// A field in an OCaml record type.
#[derive(Debug, Clone)]
pub struct OcamlRecordField {
    pub name: std::string::String,
    pub ty: OcamlType,
    pub mutable: bool,
}
/// OCaml type representation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OcamlType {
    /// `int`
    Int,
    /// `float`
    Float,
    /// `bool`
    Bool,
    /// `char`
    Char,
    /// `string`
    String,
    /// `unit`
    Unit,
    /// Bottom type (represented as `'a` with no instances in practice)
    Never,
    /// `t list`
    List(Box<OcamlType>),
    /// `t array`
    Array(Box<OcamlType>),
    /// `t1 * t2 * ...` (product/tuple type)
    Tuple(Vec<OcamlType>),
    /// `t option`
    Option(Box<OcamlType>),
    /// `(t, e) result`
    Result(Box<OcamlType>, Box<OcamlType>),
    /// `a -> b` (function type)
    Fun(Box<OcamlType>, Box<OcamlType>),
    /// A named type: `my_type`, `Tree.t`
    Custom(std::string::String),
    /// A type variable: `'a`, `'b`
    Polymorphic(std::string::String),
    /// A module path type: `MyModule.t`
    Module(std::string::String),
}
/// A top-level OCaml let binding.
#[derive(Debug, Clone)]
pub struct OcamlLetBinding {
    pub is_rec: bool,
    pub name: std::string::String,
    pub params: Vec<(std::string::String, Option<OcamlType>)>,
    pub body: OcamlExpr,
    pub type_annotation: Option<OcamlType>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OCamlDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
impl OCamlDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        OCamlDominatorTree {
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
/// A full OCaml type definition.
#[derive(Debug, Clone)]
pub struct OcamlTypeDef {
    pub name: std::string::String,
    /// Type parameters: `'a`, `'b`, etc.
    pub type_params: Vec<std::string::String>,
    pub decl: OcamlTypeDecl,
}
/// An OUnit2 test suite.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OcamlTestSuite {
    /// Suite name.
    pub name: std::string::String,
    /// Test cases.
    pub cases: Vec<OcamlTestCase>,
}
#[allow(dead_code)]
impl OcamlTestSuite {
    /// Create a new test suite.
    pub fn new(name: &str) -> Self {
        OcamlTestSuite {
            name: name.to_string(),
            cases: vec![],
        }
    }
    /// Add a test case.
    pub fn add(mut self, case: OcamlTestCase) -> Self {
        self.cases.push(case);
        self
    }
    /// Emit OUnit2 test suite.
    pub fn emit_ounit(&self) -> std::string::String {
        let mut lines = vec![];
        lines.push("open OUnit2".to_string());
        lines.push(std::string::String::new());
        let cases_str: Vec<std::string::String> = self
            .cases
            .iter()
            .map(|c| format!("  {}", c.emit_ounit()))
            .collect();
        lines.push(format!("let suite = \"{}\" >::: [", self.name));
        lines.push(cases_str.join(";\n"));
        lines.push("]".to_string());
        lines.push(std::string::String::new());
        lines.push("let () = run_test_tt_main suite".to_string());
        lines.join("\n")
    }
    /// Emit Alcotest test suite.
    pub fn emit_alcotest(&self) -> std::string::String {
        let mut lines = vec![];
        lines.push("let () =".to_string());
        lines.push("  Alcotest.run \"tests\" [".to_string());
        let cases_str: Vec<std::string::String> = self
            .cases
            .iter()
            .map(|c| format!("    {}", c.emit_alcotest("Quick")))
            .collect();
        lines.push(format!("    \"{}\", [", self.name));
        lines.push(cases_str.join(";\n"));
        lines.push("    ]".to_string());
        lines.push("  ]".to_string());
        lines.join("\n")
    }
}
/// An OCaml Generalized Algebraic Data Type (GADT) variant.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OcamlGadtVariant {
    /// Constructor name.
    pub name: std::string::String,
    /// Parameter types.
    pub params: Vec<OcamlType>,
    /// The resulting index type (the right-hand side of `:`).
    pub result_type: std::string::String,
}
/// An OCaml functor definition.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OcamlFunctor {
    /// Functor name.
    pub name: std::string::String,
    /// Parameters.
    pub params: Vec<OcamlFunctorParam>,
    /// Body definitions.
    pub body: Vec<OcamlDefinition>,
    /// Optional result signature constraint.
    pub sig_constraint: Option<std::string::String>,
}
#[allow(dead_code)]
impl OcamlFunctor {
    /// Create a new functor.
    pub fn new(name: &str) -> Self {
        OcamlFunctor {
            name: name.to_string(),
            params: vec![],
            body: vec![],
            sig_constraint: None,
        }
    }
    /// Add a functor parameter.
    pub fn add_param(mut self, name: &str, module_type: &str) -> Self {
        self.params.push(OcamlFunctorParam {
            name: name.to_string(),
            module_type: module_type.to_string(),
        });
        self
    }
    /// Add a body definition.
    pub fn add_def(mut self, def: OcamlDefinition) -> Self {
        self.body.push(def);
        self
    }
    /// Set a result signature constraint.
    pub fn with_sig(mut self, sig: &str) -> Self {
        self.sig_constraint = Some(sig.to_string());
        self
    }
    /// Emit the functor definition.
    pub fn emit(&self) -> std::string::String {
        let params_str: Vec<std::string::String> = self
            .params
            .iter()
            .map(|p| format!("({} : {})", p.name, p.module_type))
            .collect();
        let sig_str = self
            .sig_constraint
            .as_ref()
            .map(|s| format!(" : {}", s))
            .unwrap_or_default();
        let mut lines = vec![format!(
            "module {} {}{}= struct",
            self.name,
            params_str.join(" "),
            sig_str
        )];
        for def in &self.body {
            for line in def.to_string().lines() {
                lines.push(format!("  {}", line));
            }
        }
        lines.push("end".to_string());
        lines.join("\n")
    }
}
/// A Dune executable stanza.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DuneExecutable {
    /// Executable name.
    pub name: std::string::String,
    /// Public name (installed as).
    pub public_name: Option<std::string::String>,
    /// Library dependencies.
    pub libraries: Vec<std::string::String>,
    /// Preprocessors.
    pub preprocess: Vec<std::string::String>,
    /// Extra OCaml flags.
    pub flags: Vec<std::string::String>,
}
#[allow(dead_code)]
impl DuneExecutable {
    /// Create a new Dune executable stanza.
    pub fn new(name: &str) -> Self {
        DuneExecutable {
            name: name.to_string(),
            public_name: None,
            libraries: vec![],
            preprocess: vec![],
            flags: vec![],
        }
    }
    /// Add a dependency.
    pub fn add_dep(mut self, dep: &str) -> Self {
        self.libraries.push(dep.to_string());
        self
    }
    /// Add ppx.
    pub fn add_ppx(mut self, ppx: &str) -> Self {
        self.preprocess.push(format!("(pps {})", ppx));
        self
    }
    /// Emit the stanza.
    pub fn emit(&self) -> std::string::String {
        let mut lines = vec!["(executable".to_string()];
        lines.push(format!(" (name {})", self.name));
        if let Some(pub_name) = &self.public_name {
            lines.push(format!(" (public_name {})", pub_name));
        }
        if !self.libraries.is_empty() {
            lines.push(format!(" (libraries {})", self.libraries.join(" ")));
        }
        if !self.preprocess.is_empty() {
            lines.push(format!(" (preprocess {})", self.preprocess.join(" ")));
        }
        lines.push(")".to_string());
        lines.join("\n")
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OCamlAnalysisCache {
    pub(super) entries: std::collections::HashMap<String, OCamlCacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
impl OCamlAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        OCamlAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&OCamlCacheEntry> {
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
            OCamlCacheEntry {
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
/// An OCaml module parameter (functor argument).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OcamlFunctorParam {
    /// Parameter name (e.g. "M").
    pub name: std::string::String,
    /// Module type (e.g. "Map.OrderedType").
    pub module_type: std::string::String,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OCamlDepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
}
impl OCamlDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        OCamlDepGraph {
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
/// An OUnit2-style test case.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OcamlTestCase {
    /// Test name.
    pub name: std::string::String,
    /// Test body code.
    pub body: std::string::String,
    /// Expected result (for assert_equal).
    pub expected: Option<std::string::String>,
    /// Actual expression to evaluate.
    pub actual: Option<std::string::String>,
}
#[allow(dead_code)]
impl OcamlTestCase {
    /// Create a new test case.
    pub fn new(name: &str, body: &str) -> Self {
        OcamlTestCase {
            name: name.to_string(),
            body: body.to_string(),
            expected: None,
            actual: None,
        }
    }
    /// Create an equality assertion test.
    pub fn assert_equal(name: &str, expected: &str, actual: &str) -> Self {
        OcamlTestCase {
            name: name.to_string(),
            body: format!("assert_equal ({}) ({})", expected, actual),
            expected: Some(expected.to_string()),
            actual: Some(actual.to_string()),
        }
    }
    /// Emit as an OUnit2 test case.
    pub fn emit_ounit(&self) -> std::string::String {
        format!("\"{}\" >:: (fun _ -> {})", self.name, self.body)
    }
    /// Emit as an inline Alcotest test.
    pub fn emit_alcotest(&self, test_type: &str) -> std::string::String {
        format!(
            "Alcotest.test_case \"{}\" `{} (fun () -> {})",
            self.name, test_type, self.body
        )
    }
}
/// A top-level definition in an OCaml module.
#[derive(Debug, Clone)]
pub enum OcamlDefinition {
    TypeDef(OcamlTypeDef),
    Let(OcamlLetBinding),
    Signature(OcamlSignature),
    /// `exception Name of ty`
    Exception(std::string::String, Option<OcamlType>),
    /// `open Module`
    Open(std::string::String),
    /// A nested module: `module M = struct ... end`
    SubModule(OcamlModule),
    /// A raw comment
    Comment(std::string::String),
}
/// OCaml Bigarray kind.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BigarrayKind {
    Float32,
    Float64,
    Int32,
    Int64,
    Int,
    Complex32,
    Complex64,
}
#[allow(dead_code)]
impl BigarrayKind {
    /// Return the Bigarray kind constant name.
    pub fn kind_name(&self) -> &'static str {
        match self {
            BigarrayKind::Float32 => "Bigarray.float32",
            BigarrayKind::Float64 => "Bigarray.float64",
            BigarrayKind::Int32 => "Bigarray.int32",
            BigarrayKind::Int64 => "Bigarray.int64",
            BigarrayKind::Int => "Bigarray.int",
            BigarrayKind::Complex32 => "Bigarray.complex32",
            BigarrayKind::Complex64 => "Bigarray.complex64",
        }
    }
    /// Return the corresponding OCaml element type name.
    pub fn element_type(&self) -> &'static str {
        match self {
            BigarrayKind::Float32 => "float",
            BigarrayKind::Float64 => "float",
            BigarrayKind::Int32 => "int32",
            BigarrayKind::Int64 => "int64",
            BigarrayKind::Int => "int",
            BigarrayKind::Complex32 => "Complex.t",
            BigarrayKind::Complex64 => "Complex.t",
        }
    }
}
/// OCaml pattern for match arms.
#[derive(Debug, Clone, PartialEq)]
pub enum OcamlPattern {
    /// `_` (wildcard)
    Wildcard,
    /// `x` (bind to variable)
    Var(std::string::String),
    /// A constant pattern: `42`, `true`, `'a'`, `"hello"`
    Const(OcamlLit),
    /// A tuple pattern: `(p1, p2, ...)`
    Tuple(Vec<OcamlPattern>),
    /// A cons pattern: `h :: t`
    Cons(Box<OcamlPattern>, Box<OcamlPattern>),
    /// A list pattern: `[p1; p2; ...]`
    List(Vec<OcamlPattern>),
    /// A constructor pattern: `Some p`, `Error e`, `Leaf`, `Node (l, v, r)`
    Ctor(std::string::String, Vec<OcamlPattern>),
    /// A record pattern: `{ field1 = p1; field2 = p2; ... }`
    Record(Vec<(std::string::String, OcamlPattern)>),
    /// An or-pattern: `p1 | p2`
    Or(Box<OcamlPattern>, Box<OcamlPattern>),
    /// An as-pattern: `p as x`
    As(Box<OcamlPattern>, std::string::String),
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OCamlLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
impl OCamlLivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        OCamlLivenessInfo {
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
/// OCaml literal values.
#[derive(Debug, Clone, PartialEq)]
pub enum OcamlLit {
    Int(i64),
    Float(f64),
    Bool(bool),
    Char(char),
    Str(std::string::String),
    Unit,
}
/// An OCaml type declaration body.
#[derive(Debug, Clone)]
pub enum OcamlTypeDecl {
    /// A type alias: `type t = int`
    Alias(OcamlType),
    /// A record type: `type t = { field: ty; ... }`
    Record(Vec<OcamlRecordField>),
    /// A variant type: `type t = A | B of ty1 * ty2 | ...`
    Variant(Vec<(std::string::String, Vec<OcamlType>)>),
    /// An abstract type (signature only): `type t`
    Abstract,
}
/// An OCaml GADT type definition.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OcamlGadt {
    /// GADT type name.
    pub name: std::string::String,
    /// Type parameter names.
    pub type_params: Vec<std::string::String>,
    /// Variants.
    pub variants: Vec<OcamlGadtVariant>,
}
#[allow(dead_code)]
impl OcamlGadt {
    /// Create a new GADT.
    pub fn new(name: &str, type_params: Vec<&str>) -> Self {
        OcamlGadt {
            name: name.to_string(),
            type_params: type_params.iter().map(|s| s.to_string()).collect(),
            variants: vec![],
        }
    }
    /// Add a variant.
    pub fn add_variant(mut self, name: &str, params: Vec<OcamlType>, result: &str) -> Self {
        self.variants.push(OcamlGadtVariant {
            name: name.to_string(),
            params,
            result_type: result.to_string(),
        });
        self
    }
    /// Emit the GADT definition.
    pub fn emit(&self) -> std::string::String {
        let type_params_str = if self.type_params.is_empty() {
            std::string::String::new()
        } else {
            format!(
                "({}) ",
                self.type_params
                    .iter()
                    .map(|p| format!("'{}", p))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        };
        let mut lines = vec![format!("type {}{}=", type_params_str, self.name)];
        for v in &self.variants {
            if v.params.is_empty() {
                lines.push(format!("  | {} : {}", v.name, v.result_type));
            } else {
                let params_str: Vec<std::string::String> =
                    v.params.iter().map(|t| t.to_string()).collect();
                lines.push(format!(
                    "  | {} : {} -> {}",
                    v.name,
                    params_str.join(" * "),
                    v.result_type
                ));
            }
        }
        lines.join("\n")
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OCamlCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
#[allow(dead_code)]
pub struct OCamlConstantFoldingHelper;
impl OCamlConstantFoldingHelper {
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
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum OCamlPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl OCamlPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            OCamlPassPhase::Analysis => "analysis",
            OCamlPassPhase::Transformation => "transformation",
            OCamlPassPhase::Verification => "verification",
            OCamlPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(
            self,
            OCamlPassPhase::Transformation | OCamlPassPhase::Cleanup
        )
    }
}
/// An OCaml module (`.ml` file or nested `module M = struct ... end`).
#[derive(Debug, Clone)]
pub struct OcamlModule {
    pub name: std::string::String,
    pub definitions: Vec<OcamlDefinition>,
    /// Whether this is a top-level file module (affects emit format).
    pub is_top_level: bool,
}
impl OcamlModule {
    /// Create a new top-level module.
    pub fn new(name: &str) -> Self {
        OcamlModule {
            name: name.to_string(),
            definitions: Vec::new(),
            is_top_level: true,
        }
    }
    /// Add a definition to the module.
    pub fn add(&mut self, def: OcamlDefinition) {
        self.definitions.push(def);
    }
    /// Emit the module as OCaml source code (`.ml`).
    pub fn emit(&self) -> std::string::String {
        if self.is_top_level {
            let mut out = std::string::String::new();
            for def in &self.definitions {
                out.push_str(&format!("{}\n\n", def));
            }
            out
        } else {
            let mut out = format!("module {} = struct\n", self.name);
            for def in &self.definitions {
                let text = def.to_string();
                for line in text.lines() {
                    out.push_str("  ");
                    out.push_str(line);
                    out.push('\n');
                }
                out.push('\n');
            }
            out.push_str("end");
            out
        }
    }
    /// Emit the module interface as OCaml source code (`.mli`).
    pub fn emit_mli(&self) -> std::string::String {
        let mut out = std::string::String::new();
        for def in &self.definitions {
            match def {
                OcamlDefinition::TypeDef(td) => {
                    out.push_str(&format!("{}\n\n", td));
                }
                OcamlDefinition::Let(lb) => {
                    if let Some(ret_ty) = &lb.type_annotation {
                        if lb.params.is_empty() {
                            out.push_str(&format!("val {} : {}\n\n", lb.name, ret_ty));
                        } else {
                            let mut ty = ret_ty.clone();
                            for (_, param_ty) in lb.params.iter().rev() {
                                let domain = param_ty
                                    .clone()
                                    .unwrap_or(OcamlType::Custom("_".to_string()));
                                ty = OcamlType::Fun(Box::new(domain), Box::new(ty));
                            }
                            out.push_str(&format!("val {} : {}\n\n", lb.name, ty));
                        }
                    }
                }
                OcamlDefinition::Signature(sig) => {
                    out.push_str(&format!("{}\n\n", sig));
                }
                OcamlDefinition::Exception(name, ty) => {
                    if let Some(t) = ty {
                        out.push_str(&format!("exception {} of {}\n\n", name, t));
                    } else {
                        out.push_str(&format!("exception {}\n\n", name));
                    }
                }
                OcamlDefinition::Open(m) => {
                    out.push_str(&format!("open {}\n\n", m));
                }
                OcamlDefinition::Comment(text) => {
                    out.push_str(&format!("(* {} *)\n\n", text));
                }
                OcamlDefinition::SubModule(_) => {}
            }
        }
        out
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OCamlWorklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
impl OCamlWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        OCamlWorklist {
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
/// OCaml expression AST node.
#[derive(Debug, Clone, PartialEq)]
pub enum OcamlExpr {
    /// A literal value.
    Lit(OcamlLit),
    /// A variable or module path: `x`, `List.map`
    Var(std::string::String),
    /// A binary operation: `a + b`, `a && b`
    BinOp(std::string::String, Box<OcamlExpr>, Box<OcamlExpr>),
    /// A unary operation: `not b`, `- n`
    UnaryOp(std::string::String, Box<OcamlExpr>),
    /// Function application: `f x` or `f x y` (curried)
    App(Box<OcamlExpr>, Vec<OcamlExpr>),
    /// A lambda: `fun x -> body`
    Lambda(Vec<std::string::String>, Box<OcamlExpr>),
    /// A let binding: `let x = e1 in e2`
    Let(std::string::String, Box<OcamlExpr>, Box<OcamlExpr>),
    /// A recursive let binding: `let rec f x = e1 in e2`
    LetRec(
        std::string::String,
        Vec<std::string::String>,
        Box<OcamlExpr>,
        Box<OcamlExpr>,
    ),
    /// If-then-else: `if cond then e1 else e2`
    IfThenElse(Box<OcamlExpr>, Box<OcamlExpr>, Box<OcamlExpr>),
    /// Match expression: `match e with | p1 -> e1 | p2 -> e2`
    Match(Box<OcamlExpr>, Vec<(OcamlPattern, OcamlExpr)>),
    /// Tuple: `(e1, e2, ...)`
    Tuple(Vec<OcamlExpr>),
    /// List literal: `[e1; e2; ...]`
    List(Vec<OcamlExpr>),
    /// Record literal: `{ field1 = e1; field2 = e2; ... }`
    Record(Vec<(std::string::String, OcamlExpr)>),
    /// Record field access: `e.field`
    Field(Box<OcamlExpr>, std::string::String),
    /// Module expression: `Module.expr`
    Module(std::string::String, Box<OcamlExpr>),
    /// `begin ... end` block (sequence of expressions)
    Begin(Vec<OcamlExpr>),
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OCamlPassConfig {
    pub phase: OCamlPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
impl OCamlPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: OCamlPassPhase) -> Self {
        OCamlPassConfig {
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
/// An OCaml module signature (`sig ... end`).
#[derive(Debug, Clone)]
pub struct OcamlSignature {
    pub name: std::string::String,
    pub items: Vec<OcamlSigItem>,
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct OCamlPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
impl OCamlPassStats {
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
