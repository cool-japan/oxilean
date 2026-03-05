//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use std::collections::{HashMap, HashSet, VecDeque};

/// Structural validator for NixOS module expressions.
#[allow(dead_code)]
pub struct NixModuleValidator;
impl NixModuleValidator {
    /// Check that a NixOS module has the expected structure.
    ///
    /// A valid NixOS module is a lambda returning an attrset with
    /// `options` and/or `config` keys.
    pub fn validate(module: &NixModule) -> NixModuleValidationReport {
        let mut report = NixModuleValidationReport::default();
        match &module.top_level {
            NixExpr::Lambda(func) => match &*func.body {
                NixExpr::AttrSet(attrs) => {
                    let has_options = attrs.iter().any(|a| a.name == "options");
                    let has_config = attrs.iter().any(|a| a.name == "config");
                    if !has_options && !has_config {
                        report
                            .warnings
                            .push("Module body has neither 'options' nor 'config' keys".into());
                    }
                }
                _ => {
                    report
                        .errors
                        .push("NixOS module body should be an attribute set".into());
                }
            },
            _ => {
                if module.is_module_file {
                    report
                        .errors
                        .push("NixOS module file should start with a lambda".into());
                }
            }
        }
        report
    }
}
/// Helpers for emitting common `nixpkgs` patterns.
#[allow(dead_code)]
pub struct NixPkgsHelper;
impl NixPkgsHelper {
    /// Generate `pkgs.fetchurl { url = ...; sha256 = ...; }`.
    pub fn fetch_url(url: &str, sha256: &str) -> NixExpr {
        nix_apply(
            nix_select(nix_var("pkgs"), "fetchurl"),
            nix_set(vec![("url", nix_str(url)), ("sha256", nix_str(sha256))]),
        )
    }
    /// Generate `pkgs.fetchFromGitHub { owner, repo, rev, sha256 }`.
    pub fn fetch_from_github(owner: &str, repo: &str, rev: &str, sha256: &str) -> NixExpr {
        nix_apply(
            nix_select(nix_var("pkgs"), "fetchFromGitHub"),
            nix_set(vec![
                ("owner", nix_str(owner)),
                ("repo", nix_str(repo)),
                ("rev", nix_str(rev)),
                ("sha256", nix_str(sha256)),
            ]),
        )
    }
    /// Generate `pkgs.fetchGit { url; rev; }`.
    pub fn fetch_git(url: &str, rev: &str) -> NixExpr {
        nix_apply(
            nix_select(nix_var("pkgs"), "fetchGit"),
            nix_set(vec![("url", nix_str(url)), ("rev", nix_str(rev))]),
        )
    }
    /// Generate `pkgs.callPackage ./path.nix { }`.
    pub fn call_package(path: &str) -> NixExpr {
        nix_apply(
            nix_apply(nix_select(nix_var("pkgs"), "callPackage"), nix_path(path)),
            nix_set(vec![]),
        )
    }
    /// Generate `pkgs.writeShellScriptBin name content`.
    pub fn write_shell_script_bin(name: &str, content: &str) -> NixExpr {
        nix_apply(
            nix_apply(
                nix_select(nix_var("pkgs"), "writeShellScriptBin"),
                nix_str(name),
            ),
            NixExpr::Multiline(content.to_string()),
        )
    }
    /// Generate `pkgs.writeText name content`.
    pub fn write_text(name: &str, content: &str) -> NixExpr {
        nix_apply(
            nix_apply(nix_select(nix_var("pkgs"), "writeText"), nix_str(name)),
            nix_str(content),
        )
    }
    /// Generate a symlinkJoin package: `pkgs.symlinkJoin { name; paths = [...]; }`.
    pub fn symlink_join(name: &str, paths: Vec<NixExpr>) -> NixExpr {
        nix_apply(
            nix_select(nix_var("pkgs"), "symlinkJoin"),
            nix_set(vec![("name", nix_str(name)), ("paths", nix_list(paths))]),
        )
    }
}
/// Nix expression formatter with configurable options.
#[allow(dead_code)]
pub struct NixFormatter {
    /// Indentation width (default 2).
    pub indent: usize,
    /// Maximum line length before wrapping (default 80).
    pub max_line_len: usize,
    /// Whether to sort attribute set keys alphabetically.
    pub sort_attrs: bool,
}
impl NixFormatter {
    /// Create a new formatter with default options.
    pub fn new() -> Self {
        NixFormatter {
            indent: 2,
            max_line_len: 80,
            sort_attrs: true,
        }
    }
    /// Format a NixExpr, optionally sorting attribute keys.
    pub fn format(&self, expr: &NixExpr) -> String {
        if self.sort_attrs {
            let sorted = self.sort_attrset(expr);
            sorted.emit(0)
        } else {
            expr.emit(0)
        }
    }
    /// Recursively sort attribute set keys.
    pub(super) fn sort_attrset(&self, expr: &NixExpr) -> NixExpr {
        match expr {
            NixExpr::AttrSet(attrs) => {
                let mut sorted = attrs
                    .iter()
                    .map(|a| NixAttr {
                        name: a.name.clone(),
                        value: self.sort_attrset(&a.value),
                    })
                    .collect::<Vec<_>>();
                sorted.sort_by(|a, b| a.name.cmp(&b.name));
                NixExpr::AttrSet(sorted)
            }
            NixExpr::Rec(attrs) => {
                let mut sorted = attrs
                    .iter()
                    .map(|a| NixAttr {
                        name: a.name.clone(),
                        value: self.sort_attrset(&a.value),
                    })
                    .collect::<Vec<_>>();
                sorted.sort_by(|a, b| a.name.cmp(&b.name));
                NixExpr::Rec(sorted)
            }
            other => other.clone(),
        }
    }
    /// Estimate the rendered line length of an expression.
    pub fn estimate_width(expr: &NixExpr) -> usize {
        expr.emit(0).lines().map(|l| l.len()).max().unwrap_or(0)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum NixPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl NixPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            NixPassPhase::Analysis => "analysis",
            NixPassPhase::Transformation => "transformation",
            NixPassPhase::Verification => "verification",
            NixPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(self, NixPassPhase::Transformation | NixPassPhase::Cleanup)
    }
}
/// A simplified structural type checker for Nix expressions.
///
/// Nix is dynamically typed; this checker infers descriptive `NixType` tags.
#[allow(dead_code)]
pub struct NixTypeChecker;
impl NixTypeChecker {
    /// Infer the `NixType` of a NixExpr (best-effort, not full analysis).
    pub fn infer(expr: &NixExpr) -> NixType {
        match expr {
            NixExpr::Int(_) => NixType::Int,
            NixExpr::Float(_) => NixType::Float,
            NixExpr::Bool(_) => NixType::Bool,
            NixExpr::Str(_) | NixExpr::Antiquote(_, _, _) | NixExpr::Multiline(_) => {
                NixType::String
            }
            NixExpr::Path(_) => NixType::Path,
            NixExpr::Null => NixType::NullType,
            NixExpr::List(items) => {
                let elem_ty = items.first().map(Self::infer).unwrap_or(NixType::NullType);
                NixType::List(Box::new(elem_ty))
            }
            NixExpr::AttrSet(attrs) | NixExpr::Rec(attrs) => NixType::AttrSet(
                attrs
                    .iter()
                    .map(|a| (a.name.clone(), Self::infer(&a.value)))
                    .collect(),
            ),
            NixExpr::Lambda(_) => {
                NixType::Function(Box::new(NixType::NullType), Box::new(NixType::NullType))
            }
            NixExpr::If(_, t, f) => {
                let ty_t = Self::infer(t);
                let _ty_f = Self::infer(f);
                ty_t
            }
            NixExpr::Let(_, body) => Self::infer(body),
            NixExpr::With(_, body) => Self::infer(body),
            NixExpr::Apply(_, _) => NixType::NullType,
            NixExpr::Select(_, _, _) => NixType::NullType,
            NixExpr::Import(_) => NixType::AttrSet(vec![]),
            NixExpr::Var(_) => NixType::NullType,
            NixExpr::UnOp(op, e) => {
                if op == "!" {
                    NixType::Bool
                } else {
                    Self::infer(e)
                }
            }
            NixExpr::BinOp(op, lhs, _) => match op.as_str() {
                "==" | "!=" | "<" | ">" | "<=" | ">=" | "&&" | "||" => NixType::Bool,
                "++" => NixType::List(Box::new(NixType::NullType)),
                "//" => NixType::AttrSet(vec![]),
                _ => Self::infer(lhs),
            },
            _ => NixType::NullType,
        }
    }
}
#[allow(dead_code)]
pub struct NixConstantFoldingHelper;
impl NixConstantFoldingHelper {
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
#[derive(Debug, Clone)]
pub struct NixWorklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
impl NixWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        NixWorklist {
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
pub struct NixPassRegistry {
    pub(super) configs: Vec<NixPassConfig>,
    pub(super) stats: std::collections::HashMap<String, NixPassStats>,
}
impl NixPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        NixPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: NixPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), NixPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&NixPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&NixPassStats> {
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
pub struct NixCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct NixPassConfig {
    pub phase: NixPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
impl NixPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: NixPassPhase) -> Self {
        NixPassConfig {
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
/// The Nix code generation backend.
///
/// Converts OxiLean IR constructs into Nix expression language output.
pub struct NixBackend {
    /// Indent width (default 2)
    pub indent_width: usize,
}
impl NixBackend {
    /// Create a new NixBackend with default settings.
    pub fn new() -> Self {
        NixBackend { indent_width: 2 }
    }
    /// Emit a NixExpr at the given indentation level.
    pub fn emit_expr(&self, expr: &NixExpr, indent: usize) -> String {
        expr.emit(indent)
    }
    /// Emit a complete NixModule as a `.nix` file string.
    pub fn emit_module(&self, module: &NixModule) -> String {
        module.emit()
    }
    /// Emit a NixAttr binding.
    pub fn emit_attr(&self, attr: &NixAttr, indent: usize) -> String {
        attr.emit(indent)
    }
    /// Emit a NixFunction.
    pub fn emit_function(&self, func: &NixFunction, indent: usize) -> String {
        func.emit(indent)
    }
    /// Build a `mkDerivation` call from common parameters.
    #[allow(clippy::too_many_arguments)]
    pub fn make_derivation(
        &self,
        name: &str,
        version: &str,
        src: NixExpr,
        build_inputs: Vec<NixExpr>,
        build_phase: Option<&str>,
        install_phase: Option<&str>,
        extra_attrs: Vec<NixAttr>,
    ) -> NixExpr {
        let mut attrs = vec![
            NixAttr::new("name", NixExpr::Str(format!("{}-{}", name, version))),
            NixAttr::new("version", NixExpr::Str(version.to_string())),
            NixAttr::new("src", src),
            NixAttr::new("buildInputs", NixExpr::List(build_inputs)),
        ];
        if let Some(bp) = build_phase {
            attrs.push(NixAttr::new(
                "buildPhase",
                NixExpr::Multiline(bp.to_string()),
            ));
        }
        if let Some(ip) = install_phase {
            attrs.push(NixAttr::new(
                "installPhase",
                NixExpr::Multiline(ip.to_string()),
            ));
        }
        attrs.extend(extra_attrs);
        NixExpr::Apply(
            Box::new(NixExpr::Select(
                Box::new(NixExpr::Var("pkgs".into())),
                "stdenv.mkDerivation".into(),
                None,
            )),
            Box::new(NixExpr::AttrSet(attrs)),
        )
    }
    /// Build a NixOS module skeleton with options and config sections.
    pub fn make_nixos_module(
        &self,
        module_args: Vec<(String, Option<NixExpr>)>,
        options: Vec<NixAttr>,
        config: Vec<NixAttr>,
    ) -> NixModule {
        let body = NixExpr::AttrSet(vec![
            NixAttr::new("options", NixExpr::AttrSet(options)),
            NixAttr::new("config", NixExpr::AttrSet(config)),
        ]);
        let top = NixExpr::Lambda(Box::new(NixFunction {
            pattern: NixPattern::AttrPattern {
                attrs: module_args,
                ellipsis: true,
            },
            body: Box::new(body),
        }));
        NixModule::nixos_module(top)
    }
    /// Build an overlay expression: `final: prev: { ... }`
    pub fn make_overlay(&self, attrs: Vec<NixAttr>) -> NixExpr {
        NixExpr::Lambda(Box::new(NixFunction {
            pattern: NixPattern::Ident("final".into()),
            body: Box::new(NixExpr::Lambda(Box::new(NixFunction {
                pattern: NixPattern::Ident("prev".into()),
                body: Box::new(NixExpr::AttrSet(attrs)),
            }))),
        }))
    }
    /// Build a `flake.nix`-style output expression skeleton.
    pub fn make_flake(&self, description: &str, outputs_attrs: Vec<NixAttr>) -> NixExpr {
        NixExpr::AttrSet(vec![
            NixAttr::new("description", NixExpr::Str(description.to_string())),
            NixAttr::new(
                "outputs",
                NixExpr::Lambda(Box::new(NixFunction {
                    pattern: NixPattern::AttrPattern {
                        attrs: vec![("self".into(), None), ("nixpkgs".into(), None)],
                        ellipsis: true,
                    },
                    body: Box::new(NixExpr::AttrSet(outputs_attrs)),
                })),
            ),
        ])
    }
}
/// Helpers for generating flake.nix components.
#[allow(dead_code)]
pub struct NixFlakeHelper;
impl NixFlakeHelper {
    /// Generate `inputs.<name> = { url = "..."; }`.
    pub fn input(name: &str, url: &str) -> NixAttr {
        NixAttr::new(name, nix_set(vec![("url", nix_str(url))]))
    }
    /// Generate `inputs.<name> = { url = "..."; flake = false; }`.
    pub fn non_flake_input(name: &str, url: &str) -> NixAttr {
        NixAttr::new(
            name,
            nix_set(vec![("url", nix_str(url)), ("flake", nix_bool(false))]),
        )
    }
    /// Generate an `inputs.nixpkgs.follows = "<other>"` override.
    pub fn follows(input_name: &str, follows_from: &str) -> NixAttr {
        NixAttr::new(
            &format!("inputs.{}.follows", input_name),
            nix_str(follows_from),
        )
    }
    /// Generate a full `flake.nix` with inputs and outputs.
    pub fn full_flake(
        description: &str,
        inputs: Vec<NixAttr>,
        output_body: NixExpr,
        input_args: Vec<(String, Option<NixExpr>)>,
    ) -> NixExpr {
        NixExpr::AttrSet(vec![
            NixAttr::new("description", nix_str(description)),
            NixAttr::new("inputs", NixExpr::AttrSet(inputs)),
            NixAttr::new(
                "outputs",
                NixExpr::Lambda(Box::new(NixFunction {
                    pattern: NixPattern::AttrPattern {
                        attrs: input_args,
                        ellipsis: true,
                    },
                    body: Box::new(output_body),
                })),
            ),
        ])
    }
    /// Generate `perSystem = system: let pkgs = nixpkgs.legacyPackages.${system}; in ...`.
    pub fn per_system_let(body: NixExpr) -> NixExpr {
        NixExpr::Lambda(Box::new(NixFunction {
            pattern: NixPattern::Ident("system".into()),
            body: Box::new(nix_let(
                vec![(
                    "pkgs",
                    NixExpr::Select(
                        Box::new(NixExpr::Select(
                            Box::new(nix_var("nixpkgs")),
                            "legacyPackages".into(),
                            None,
                        )),
                        "system".into(),
                        None,
                    ),
                )],
                body,
            )),
        }))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct NixAnalysisCache {
    pub(super) entries: std::collections::HashMap<String, NixCacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
impl NixAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        NixAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&NixCacheEntry> {
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
            NixCacheEntry {
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
/// A Nix function definition: `pattern: body`
#[derive(Debug, Clone, PartialEq)]
pub struct NixFunction {
    /// Argument pattern
    pub pattern: NixPattern,
    /// Function body expression
    pub body: Box<NixExpr>,
}
impl NixFunction {
    /// Create a simple single-argument function.
    pub fn new(arg: impl Into<String>, body: NixExpr) -> Self {
        NixFunction {
            pattern: NixPattern::Ident(arg.into()),
            body: Box::new(body),
        }
    }
    /// Create a function with an attribute set pattern.
    pub fn with_attr_pattern(
        attrs: Vec<(String, Option<NixExpr>)>,
        ellipsis: bool,
        body: NixExpr,
    ) -> Self {
        NixFunction {
            pattern: NixPattern::AttrPattern { attrs, ellipsis },
            body: Box::new(body),
        }
    }
    pub(super) fn emit(&self, indent: usize) -> String {
        format!("{}: {}", self.pattern.emit(), self.body.emit(indent))
    }
}
/// A single binding inside a `let` expression: `name = expr;`
#[derive(Debug, Clone, PartialEq)]
pub struct NixLetBinding {
    /// Bound name
    pub name: String,
    /// Bound expression
    pub value: NixExpr,
}
impl NixLetBinding {
    pub fn new(name: impl Into<String>, value: NixExpr) -> Self {
        NixLetBinding {
            name: name.into(),
            value,
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct NixDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
impl NixDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        NixDominatorTree {
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
/// Helpers for generating Nix expressions dealing with nullable/optional values.
#[allow(dead_code)]
pub struct NixOptionalHelper;
impl NixOptionalHelper {
    /// Generate `if x == null then default else f x`.
    pub fn map_nullable(value: NixExpr, default: NixExpr, f: NixExpr) -> NixExpr {
        nix_if(
            NixExpr::BinOp(
                "==".into(),
                Box::new(value.clone()),
                Box::new(NixExpr::Null),
            ),
            default,
            nix_apply(f, value),
        )
    }
    /// Generate `if x == null then null else f x`.
    pub fn fmap_nullable(value: NixExpr, f: NixExpr) -> NixExpr {
        Self::map_nullable(value.clone(), NixExpr::Null, f)
    }
    /// Generate `x or default` (attribute access with fallback).
    pub fn with_default(base: NixExpr, attr: &str, default: NixExpr) -> NixExpr {
        NixExpr::Select(Box::new(base), attr.to_string(), Some(Box::new(default)))
    }
    /// Generate `lib.optionals cond list`.
    pub fn optionals(cond: NixExpr, list: NixExpr) -> NixExpr {
        nix_apply(
            nix_apply(nix_select(nix_var("lib"), "optionals"), cond),
            list,
        )
    }
    /// Generate `lib.optional cond value`.
    pub fn optional(cond: NixExpr, value: NixExpr) -> NixExpr {
        nix_apply(
            nix_apply(nix_select(nix_var("lib"), "optional"), cond),
            value,
        )
    }
}
/// A single part of an interpolated Nix string.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum NixStringPart {
    /// A literal string fragment.
    Literal(String),
    /// An interpolated expression `${expr}`.
    Interp(NixExpr),
}
/// Nix runtime type tags (Nix is dynamically typed, so these are descriptive).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NixType {
    /// Integer type: `42`
    Int,
    /// Float type: `3.14`
    Float,
    /// Boolean type: `true` / `false`
    Bool,
    /// String type: `"hello"`
    String,
    /// Path type: `./foo/bar` or `/nix/store/...`
    Path,
    /// The null type: `null`
    NullType,
    /// Homogeneous list type: `[ T ]`
    List(Box<NixType>),
    /// Attribute set type (optional field type map)
    AttrSet(Vec<(String, NixType)>),
    /// Function type: `T1 -> T2`
    Function(Box<NixType>, Box<NixType>),
    /// Derivation (a special attribute set produced by `derivation { ... }`)
    Derivation,
}
/// Function argument pattern in Nix.
#[derive(Debug, Clone, PartialEq)]
pub enum NixPattern {
    /// Simple identifier pattern: `x: body`
    Ident(String),
    /// Attribute set pattern: `{ a, b, c ? default, ... }: body`
    AttrPattern {
        /// Named attributes to destructure, with optional defaults
        attrs: Vec<(String, Option<NixExpr>)>,
        /// Whether `...` (ellipsis) is present to allow extra attributes
        ellipsis: bool,
    },
    /// At-pattern combining set destructuring with a binding:
    /// `{ a, b } @ pkg: body` (or `pkg @ { a, b }: body`)
    AtPattern {
        /// The identifier binding for the whole set
        ident: String,
        /// Whether the ident comes before or after the `@`
        ident_first: bool,
        /// Set pattern attributes
        attrs: Vec<(String, Option<NixExpr>)>,
        /// Whether `...` is present
        ellipsis: bool,
    },
}
impl NixPattern {
    pub(super) fn emit(&self) -> String {
        match self {
            NixPattern::Ident(x) => x.clone(),
            NixPattern::AttrPattern { attrs, ellipsis } => {
                let mut parts: Vec<String> = attrs
                    .iter()
                    .map(|(name, default)| match default {
                        None => name.clone(),
                        Some(d) => format!("{} ? {}", name, d.emit(0)),
                    })
                    .collect();
                if *ellipsis {
                    parts.push("...".into());
                }
                format!("{{ {} }}", parts.join(", "))
            }
            NixPattern::AtPattern {
                ident,
                ident_first,
                attrs,
                ellipsis,
            } => {
                let mut parts: Vec<String> = attrs
                    .iter()
                    .map(|(name, default)| match default {
                        None => name.clone(),
                        Some(d) => format!("{} ? {}", name, d.emit(0)),
                    })
                    .collect();
                if *ellipsis {
                    parts.push("...".into());
                }
                let set_pat = format!("{{ {} }}", parts.join(", "));
                if *ident_first {
                    format!("{} @ {}", ident, set_pat)
                } else {
                    format!("{} @ {}", set_pat, ident)
                }
            }
        }
    }
}
/// Helpers for generating NixOS systemd service configurations.
#[allow(dead_code)]
pub struct NixSystemdHelper;
impl NixSystemdHelper {
    /// Generate a `systemd.services.<name> = { ... }` config block.
    #[allow(clippy::too_many_arguments)]
    pub fn make_service(
        description: &str,
        exec_start: &str,
        after: Vec<&str>,
        wants: Vec<&str>,
        restart: &str,
        user: Option<&str>,
        extra_attrs: Vec<NixAttr>,
    ) -> NixExpr {
        let mut attrs = vec![
            NixAttr::new("description", nix_str(description)),
            NixAttr::new("after", nix_list(after.into_iter().map(nix_str).collect())),
            NixAttr::new("wants", nix_list(wants.into_iter().map(nix_str).collect())),
            NixAttr::new(
                "serviceConfig",
                NixExpr::AttrSet(vec![
                    NixAttr::new("ExecStart", nix_str(exec_start)),
                    NixAttr::new("Restart", nix_str(restart)),
                ]),
            ),
        ];
        if let Some(u) = user {
            if let NixExpr::AttrSet(ref mut service_attrs) = attrs[3].value {
                service_attrs.push(NixAttr::new("User", nix_str(u)));
            }
        }
        attrs.extend(extra_attrs);
        NixExpr::AttrSet(attrs)
    }
    /// Generate a `systemd.timers.<name>` configuration.
    pub fn make_timer(on_calendar: &str, description: &str) -> NixExpr {
        nix_set(vec![
            ("description", nix_str(description)),
            (
                "timerConfig",
                nix_set(vec![
                    ("OnCalendar", nix_str(on_calendar)),
                    ("Persistent", nix_bool(true)),
                ]),
            ),
        ])
    }
}
/// Nix expression AST.
#[derive(Debug, Clone, PartialEq)]
pub enum NixExpr {
    /// Integer literal: `42`
    Int(i64),
    /// Float literal: `3.14`
    Float(f64),
    /// Boolean literal: `true` / `false`
    Bool(bool),
    /// String literal: `"hello, world"`
    Str(String),
    /// Path literal: `./path/to/file` or `/absolute/path`
    Path(String),
    /// `null`
    Null,
    /// List literal: `[ e1 e2 e3 ]`
    List(Vec<NixExpr>),
    /// Attribute set: `{ a = 1; b = 2; }`
    AttrSet(Vec<NixAttr>),
    /// Recursive attribute set: `rec { a = 1; b = a + 1; }`
    Rec(Vec<NixAttr>),
    /// `with expr; body` — bring all attrs of `expr` into scope
    With(Box<NixExpr>, Box<NixExpr>),
    /// `let bindings in body`
    Let(Vec<NixLetBinding>, Box<NixExpr>),
    /// `if cond then t else f`
    If(Box<NixExpr>, Box<NixExpr>, Box<NixExpr>),
    /// Function abstraction: `pat: body`
    Lambda(Box<NixFunction>),
    /// Function application: `f arg`
    Apply(Box<NixExpr>, Box<NixExpr>),
    /// Attribute selection: `e.attr` or `e.attr or default`
    Select(Box<NixExpr>, String, Option<Box<NixExpr>>),
    /// `assert cond; body`
    Assert(Box<NixExpr>, Box<NixExpr>),
    /// Variable reference: `pkgs`, `lib.mkOption`, etc.
    Var(String),
    /// Inherit expression inside a set: `inherit (src) a b;`
    Inherit(Option<Box<NixExpr>>, Vec<String>),
    /// String interpolation antiquotation: `"prefix ${expr} suffix"`
    Antiquote(String, Box<NixExpr>, String),
    /// Multi-line (indented) string: `'' ... ''`
    Multiline(String),
    /// Unary operator: `!b`, `-n`
    UnOp(String, Box<NixExpr>),
    /// Binary operator: `a + b`, `a // b`, `a ++ b`, `a == b`, etc.
    BinOp(String, Box<NixExpr>, Box<NixExpr>),
    /// `builtins.import` or any `builtins.*` call
    Import(Box<NixExpr>),
}
impl NixExpr {
    /// Emit this expression as a Nix source string.
    pub fn emit(&self, indent: usize) -> String {
        let ind = " ".repeat(indent);
        let ind2 = " ".repeat(indent + 2);
        match self {
            NixExpr::Int(n) => n.to_string(),
            NixExpr::Float(f) => {
                let s = format!("{}", f);
                if s.contains('.') || s.contains('e') {
                    s
                } else {
                    format!("{}.0", s)
                }
            }
            NixExpr::Bool(b) => if *b { "true" } else { "false" }.into(),
            NixExpr::Str(s) => format!("\"{}\"", escape_nix_string(s)),
            NixExpr::Path(p) => p.clone(),
            NixExpr::Null => "null".into(),
            NixExpr::List(elems) => {
                if elems.is_empty() {
                    "[ ]".into()
                } else {
                    let items: Vec<String> = elems.iter().map(|e| e.emit(indent + 2)).collect();
                    format!(
                        "[\n{}{}\n{}]",
                        ind2,
                        items.join(format!("\n{}", ind2).as_str()),
                        ind
                    )
                }
            }
            NixExpr::AttrSet(attrs) => {
                if attrs.is_empty() {
                    "{ }".into()
                } else {
                    let lines: Vec<String> = attrs.iter().map(|a| a.emit(indent + 2)).collect();
                    format!("{{\n{}\n{}}}", lines.join("\n"), ind)
                }
            }
            NixExpr::Rec(attrs) => {
                if attrs.is_empty() {
                    "rec { }".into()
                } else {
                    let lines: Vec<String> = attrs.iter().map(|a| a.emit(indent + 2)).collect();
                    format!("rec {{\n{}\n{}}}", lines.join("\n"), ind)
                }
            }
            NixExpr::With(src, body) => {
                format!("with {};\n{}{}", src.emit(indent), ind, body.emit(indent))
            }
            NixExpr::Let(bindings, body) => {
                let mut out = "let\n".to_string();
                for b in bindings {
                    out.push_str(&format!(
                        "{}  {} = {};\n",
                        ind,
                        b.name,
                        b.value.emit(indent + 2)
                    ));
                }
                out.push_str(&format!("{}in\n{}{}", ind, ind2, body.emit(indent + 2)));
                out
            }
            NixExpr::If(cond, then_e, else_e) => {
                format!(
                    "if {}\nthen {}{}\nelse {}{}",
                    cond.emit(indent),
                    ind2,
                    then_e.emit(indent + 2),
                    ind2,
                    else_e.emit(indent + 2),
                )
            }
            NixExpr::Lambda(func) => func.emit(indent),
            NixExpr::Apply(func, arg) => {
                let fs = func.emit(indent);
                let as_ = arg_needs_parens(arg);
                if as_ {
                    format!("{} ({})", fs, arg.emit(indent))
                } else {
                    format!("{} {}", fs, arg.emit(indent))
                }
            }
            NixExpr::Select(expr, attr, default) => {
                let base = format!("{}.{}", expr.emit(indent), attr);
                match default {
                    None => base,
                    Some(d) => format!("{} or {}", base, d.emit(indent)),
                }
            }
            NixExpr::Assert(cond, body) => {
                format!(
                    "assert {};\n{}{}",
                    cond.emit(indent),
                    ind,
                    body.emit(indent)
                )
            }
            NixExpr::Var(name) => name.clone(),
            NixExpr::Inherit(src, names) => {
                let src_s = match src {
                    None => String::new(),
                    Some(s) => format!(" ({})", s.emit(indent)),
                };
                format!("inherit{} {};", src_s, names.join(" "))
            }
            NixExpr::Antiquote(prefix, expr, suffix) => {
                format!(
                    "\"{}${{{}}}{}\"",
                    escape_nix_string(prefix),
                    expr.emit(indent),
                    escape_nix_string(suffix)
                )
            }
            NixExpr::Multiline(content) => format!("''\n{}\n{}''", content, ind),
            NixExpr::UnOp(op, e) => format!("{}{}", op, e.emit(indent)),
            NixExpr::BinOp(op, lhs, rhs) => {
                format!("({} {} {})", lhs.emit(indent), op, rhs.emit(indent))
            }
            NixExpr::Import(path) => format!("import {}", path.emit(indent)),
        }
    }
}
/// Identifiers for Nix built-in functions.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NixBuiltin {
    IsInt,
    IsFloat,
    IsBool,
    IsString,
    IsPath,
    IsNull,
    IsList,
    IsAttrs,
    IsFunction,
    StringLength,
    SubString,
    Concat,
    ConcatStringsSep,
    ToString,
    ParseInt,
    ParseFloat,
    ToLower,
    ToUpper,
    HasSuffix,
    HasPrefix,
    StringSplit,
    ReplaceStrings,
    Length,
    Head,
    Tail,
    Filter,
    Map,
    FoldLeft,
    FoldRight,
    Concatmap,
    Elem,
    ElemAt,
    Flatten,
    Sort,
    Partition,
    GroupBy,
    ZipAttrsWith,
    Unique,
    Reversal,
    Intersect,
    SubtractLists,
    ListToAttrs,
    AttrNames,
    AttrValues,
    HasAttr,
    GetAttr,
    Intersect2,
    RemoveAttrs,
    MapAttrs,
    FilterAttrs,
    Foldl2,
    ToJSON,
    FromJSON,
    ToTOML,
    ReadFile,
    ReadDir,
    PathExists,
    BaseName,
    DirOf,
    ToPath,
    StorePath,
    DerivationStrict,
    PlaceholderOf,
    HashString,
    HashFile,
    TypeOf,
    Seq,
    DeepSeq,
    Trace,
    Abort,
    Throw,
    CurrentSystem,
    CurrentTime,
    NixVersion,
}
/// A complete Nix file (module).
///
/// In NixOS, module files have a specific structure:
/// ```nix
/// { config, pkgs, lib, ... }:
/// {
///   options = { ... };
///   config  = { ... };
/// }
/// ```
///
/// Plain Nix files are just a single expression.
#[derive(Debug, Clone, PartialEq)]
pub struct NixModule {
    /// The top-level expression of the file
    pub top_level: NixExpr,
    /// Whether this is a NixOS module file (adds the standard module header comment)
    pub is_module_file: bool,
}
impl NixModule {
    /// Create a plain Nix expression file.
    pub fn new(top_level: NixExpr) -> Self {
        NixModule {
            top_level,
            is_module_file: false,
        }
    }
    /// Create a NixOS module file.
    pub fn nixos_module(top_level: NixExpr) -> Self {
        NixModule {
            top_level,
            is_module_file: true,
        }
    }
    /// Emit the complete `.nix` file contents.
    pub fn emit(&self) -> String {
        let mut out = String::new();
        if self.is_module_file {
            out.push_str("# NixOS module generated by OxiLean\n");
        } else {
            out.push_str("# Nix expression generated by OxiLean\n");
        }
        out.push_str(&self.top_level.emit(0));
        out.push('\n');
        out
    }
}
/// Builder for constructing Nix strings with multiple interpolated parts.
#[allow(dead_code)]
pub struct NixStringInterpolator {
    pub(super) parts: Vec<NixStringPart>,
}
impl NixStringInterpolator {
    /// Create a new interpolator.
    pub fn new() -> Self {
        NixStringInterpolator { parts: Vec::new() }
    }
    /// Append a literal string part.
    pub fn lit(mut self, s: &str) -> Self {
        self.parts.push(NixStringPart::Literal(s.to_string()));
        self
    }
    /// Append an interpolated expression part.
    pub fn interp(mut self, expr: NixExpr) -> Self {
        self.parts.push(NixStringPart::Interp(expr));
        self
    }
    /// Build the final NixExpr string.
    ///
    /// Produces a chain of `++` and `Antiquote` expressions, or a simple
    /// `Str` if there are no interpolated parts.
    pub fn build(self) -> NixExpr {
        if self.parts.is_empty() {
            return NixExpr::Str(String::new());
        }
        let mut collapsed: Vec<NixStringPart> = Vec::new();
        for part in self.parts {
            match (&mut collapsed.last_mut(), &part) {
                (Some(NixStringPart::Literal(prev)), NixStringPart::Literal(next)) => {
                    prev.push_str(next);
                }
                _ => collapsed.push(part),
            }
        }
        if collapsed.len() == 1 {
            if let NixStringPart::Literal(s) = &collapsed[0] {
                return NixExpr::Str(s.clone());
            }
        }
        if collapsed.len() == 3 {
            if let (
                NixStringPart::Literal(pre),
                NixStringPart::Interp(expr),
                NixStringPart::Literal(post),
            ) = (&collapsed[0], &collapsed[1], &collapsed[2])
            {
                return NixExpr::Antiquote(pre.clone(), Box::new(expr.clone()), post.clone());
            }
        }
        let mut result: NixExpr = match &collapsed[0] {
            NixStringPart::Literal(s) => NixExpr::Str(s.clone()),
            NixStringPart::Interp(e) => e.clone(),
        };
        for part in &collapsed[1..] {
            let part_expr = match part {
                NixStringPart::Literal(s) => NixExpr::Str(s.clone()),
                NixStringPart::Interp(e) => e.clone(),
            };
            result = NixExpr::BinOp("++".into(), Box::new(result), Box::new(part_expr));
        }
        result
    }
}
/// Statistics about a Nix expression tree.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct NixExprStats {
    /// Total number of nodes.
    pub node_count: u64,
    /// Number of lambda nodes.
    pub lambdas: u64,
    /// Number of application nodes.
    pub applications: u64,
    /// Number of let-binding nodes.
    pub let_bindings: u64,
    /// Number of attribute set nodes.
    pub attr_sets: u64,
    /// Number of if-then-else nodes.
    pub conditionals: u64,
    /// Number of variable references.
    pub var_refs: u64,
    /// Number of literal nodes.
    pub literals: u64,
    /// Maximum nesting depth.
    pub max_depth: u64,
}
impl NixExprStats {
    /// Collect statistics from an expression.
    pub fn collect(expr: &NixExpr) -> Self {
        let mut stats = NixExprStats::default();
        stats.visit(expr, 0);
        stats
    }
    pub(super) fn visit(&mut self, expr: &NixExpr, depth: u64) {
        self.node_count += 1;
        if depth > self.max_depth {
            self.max_depth = depth;
        }
        match expr {
            NixExpr::Int(_)
            | NixExpr::Float(_)
            | NixExpr::Bool(_)
            | NixExpr::Str(_)
            | NixExpr::Path(_)
            | NixExpr::Null => {
                self.literals += 1;
            }
            NixExpr::Var(_) => {
                self.var_refs += 1;
            }
            NixExpr::Lambda(f) => {
                self.lambdas += 1;
                self.visit(&f.body, depth + 1);
            }
            NixExpr::Apply(f, arg) => {
                self.applications += 1;
                self.visit(f, depth + 1);
                self.visit(arg, depth + 1);
            }
            NixExpr::Let(bindings, body) => {
                self.let_bindings += bindings.len() as u64;
                for b in bindings {
                    self.visit(&b.value, depth + 1);
                }
                self.visit(body, depth + 1);
            }
            NixExpr::AttrSet(attrs) | NixExpr::Rec(attrs) => {
                self.attr_sets += 1;
                for a in attrs {
                    self.visit(&a.value, depth + 1);
                }
            }
            NixExpr::If(cond, t, f) => {
                self.conditionals += 1;
                self.visit(cond, depth + 1);
                self.visit(t, depth + 1);
                self.visit(f, depth + 1);
            }
            NixExpr::With(src, body) | NixExpr::Assert(src, body) => {
                self.visit(src, depth + 1);
                self.visit(body, depth + 1);
            }
            NixExpr::BinOp(_, lhs, rhs) => {
                self.visit(lhs, depth + 1);
                self.visit(rhs, depth + 1);
            }
            NixExpr::UnOp(_, e) => {
                self.visit(e, depth + 1);
            }
            NixExpr::Select(e, _, default) => {
                self.visit(e, depth + 1);
                if let Some(d) = default {
                    self.visit(d, depth + 1);
                }
            }
            NixExpr::List(items) => {
                for item in items {
                    self.visit(item, depth + 1);
                }
            }
            _ => {}
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct NixDepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
}
impl NixDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        NixDepGraph {
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
pub struct NixLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
impl NixLivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        NixLivenessInfo {
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
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct NixPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
impl NixPassStats {
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
/// A single attribute binding inside an attribute set: `name = value;`
#[derive(Debug, Clone, PartialEq)]
pub struct NixAttr {
    /// Attribute name (may be dotted: `"a.b.c"` for nested sets)
    pub name: String,
    /// Attribute value
    pub value: NixExpr,
}
impl NixAttr {
    /// Create a new attribute binding.
    pub fn new(name: impl Into<String>, value: NixExpr) -> Self {
        NixAttr {
            name: name.into(),
            value,
        }
    }
    pub(super) fn emit(&self, indent: usize) -> String {
        format!(
            "{}{} = {};",
            " ".repeat(indent),
            self.name,
            self.value.emit(indent)
        )
    }
}
/// Validation report for a NixOS module.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct NixModuleValidationReport {
    /// Errors that must be fixed.
    pub errors: Vec<String>,
    /// Warnings that are informational.
    pub warnings: Vec<String>,
}
impl NixModuleValidationReport {
    /// Returns true if there are no errors.
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }
}
/// A simplified Nix runtime value, for evaluation / interpretation stubs.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum NixValue {
    /// Integer value.
    Int(i64),
    /// Float value.
    Float(f64),
    /// Boolean value.
    Bool(bool),
    /// String value.
    Str(String),
    /// Path value (stored as string internally).
    Path(String),
    /// Null value.
    Null,
    /// List of values.
    List(Vec<NixValue>),
    /// Attribute set mapping strings to values.
    AttrSet(Vec<(String, NixValue)>),
    /// Closure (unevaluated lambda).
    Closure(NixPattern, Box<NixExpr>),
    /// Thunk (unevaluated expression, lazy evaluation placeholder).
    Thunk(Box<NixExpr>),
    /// Derivation placeholder.
    Derivation(Box<NixValue>),
}
impl NixValue {
    /// Return the Nix type tag string for this value.
    pub fn type_name(&self) -> &'static str {
        match self {
            NixValue::Int(_) => "int",
            NixValue::Float(_) => "float",
            NixValue::Bool(_) => "bool",
            NixValue::Str(_) => "string",
            NixValue::Path(_) => "path",
            NixValue::Null => "null",
            NixValue::List(_) => "list",
            NixValue::AttrSet(_) => "set",
            NixValue::Closure(_, _) => "lambda",
            NixValue::Thunk(_) => "thunk",
            NixValue::Derivation(_) => "derivation",
        }
    }
    /// Check if this value is truthy in a boolean context.
    pub fn is_truthy(&self) -> bool {
        match self {
            NixValue::Bool(b) => *b,
            NixValue::Int(n) => *n != 0,
            NixValue::Null => false,
            NixValue::Str(s) => !s.is_empty(),
            NixValue::List(l) => !l.is_empty(),
            NixValue::AttrSet(a) => !a.is_empty(),
            _ => true,
        }
    }
    /// Try to get an attribute from an AttrSet value.
    pub fn get_attr(&self, key: &str) -> Option<&NixValue> {
        if let NixValue::AttrSet(attrs) = self {
            attrs.iter().find(|(k, _)| k == key).map(|(_, v)| v)
        } else {
            None
        }
    }
    /// Convert this value to a Nix expression (for round-tripping).
    pub fn to_expr(&self) -> NixExpr {
        match self {
            NixValue::Int(n) => NixExpr::Int(*n),
            NixValue::Float(f) => NixExpr::Float(*f),
            NixValue::Bool(b) => NixExpr::Bool(*b),
            NixValue::Str(s) => NixExpr::Str(s.clone()),
            NixValue::Path(p) => NixExpr::Path(p.clone()),
            NixValue::Null => NixExpr::Null,
            NixValue::List(items) => NixExpr::List(items.iter().map(|v| v.to_expr()).collect()),
            NixValue::AttrSet(attrs) => NixExpr::AttrSet(
                attrs
                    .iter()
                    .map(|(k, v)| NixAttr::new(k.clone(), v.to_expr()))
                    .collect(),
            ),
            NixValue::Closure(pat, body) => NixExpr::Lambda(Box::new(NixFunction {
                pattern: pat.clone(),
                body: body.clone(),
            })),
            NixValue::Thunk(expr) => *expr.clone(),
            NixValue::Derivation(inner) => inner.to_expr(),
        }
    }
}
/// Helpers for emitting `lib.*` function calls.
#[allow(dead_code)]
pub struct NixLibHelper;
impl NixLibHelper {
    /// Generate `lib.mkOption { type; default; description; }`.
    pub fn mk_option(ty: NixExpr, default: Option<NixExpr>, description: Option<&str>) -> NixExpr {
        let mut attrs = vec![("type", ty)];
        if let Some(d) = default {
            attrs.push(("default", d));
        }
        if let Some(desc) = description {
            attrs.push(("description", nix_str(desc)));
        }
        nix_apply(nix_select(nix_var("lib"), "mkOption"), nix_set(attrs))
    }
    /// Generate `lib.mkIf cond value`.
    pub fn mk_if(cond: NixExpr, value: NixExpr) -> NixExpr {
        nix_apply(nix_apply(nix_select(nix_var("lib"), "mkIf"), cond), value)
    }
    /// Generate `lib.mkDefault value`.
    pub fn mk_default(value: NixExpr) -> NixExpr {
        nix_apply(nix_select(nix_var("lib"), "mkDefault"), value)
    }
    /// Generate `lib.mkForce value`.
    pub fn mk_force(value: NixExpr) -> NixExpr {
        nix_apply(nix_select(nix_var("lib"), "mkForce"), value)
    }
    /// Generate `lib.mkMerge [ ... ]`.
    pub fn mk_merge(items: Vec<NixExpr>) -> NixExpr {
        nix_apply(nix_select(nix_var("lib"), "mkMerge"), nix_list(items))
    }
    /// Generate `lib.types.str`.
    pub fn type_str() -> NixExpr {
        NixExpr::Select(
            Box::new(NixExpr::Select(
                Box::new(nix_var("lib")),
                "types".into(),
                None,
            )),
            "str".into(),
            None,
        )
    }
    /// Generate `lib.types.int`.
    pub fn type_int() -> NixExpr {
        NixExpr::Select(
            Box::new(NixExpr::Select(
                Box::new(nix_var("lib")),
                "types".into(),
                None,
            )),
            "int".into(),
            None,
        )
    }
    /// Generate `lib.types.bool`.
    pub fn type_bool() -> NixExpr {
        NixExpr::Select(
            Box::new(NixExpr::Select(
                Box::new(nix_var("lib")),
                "types".into(),
                None,
            )),
            "bool".into(),
            None,
        )
    }
    /// Generate `lib.types.listOf T`.
    pub fn type_list_of(ty: NixExpr) -> NixExpr {
        nix_apply(
            NixExpr::Select(
                Box::new(NixExpr::Select(
                    Box::new(nix_var("lib")),
                    "types".into(),
                    None,
                )),
                "listOf".into(),
                None,
            ),
            ty,
        )
    }
    /// Generate `lib.types.attrsOf T`.
    pub fn type_attrs_of(ty: NixExpr) -> NixExpr {
        nix_apply(
            NixExpr::Select(
                Box::new(NixExpr::Select(
                    Box::new(nix_var("lib")),
                    "types".into(),
                    None,
                )),
                "attrsOf".into(),
                None,
            ),
            ty,
        )
    }
    /// Generate `lib.concatMapStrings f list`.
    pub fn concat_map_strings(f: NixExpr, list: NixExpr) -> NixExpr {
        nix_apply(
            nix_apply(nix_select(nix_var("lib"), "concatMapStrings"), f),
            list,
        )
    }
    /// Generate `lib.concatStringsSep sep list`.
    pub fn concat_strings_sep(sep: &str, list: NixExpr) -> NixExpr {
        nix_apply(
            nix_apply(nix_select(nix_var("lib"), "concatStringsSep"), nix_str(sep)),
            list,
        )
    }
    /// Generate `lib.mapAttrsToList f attrs`.
    pub fn map_attrs_to_list(f: NixExpr, attrs: NixExpr) -> NixExpr {
        nix_apply(
            nix_apply(nix_select(nix_var("lib"), "mapAttrsToList"), f),
            attrs,
        )
    }
    /// Generate `lib.attrByPath ["a" "b"] default attrs`.
    pub fn attr_by_path(path: Vec<&str>, default: NixExpr, attrs: NixExpr) -> NixExpr {
        nix_apply(
            nix_apply(
                nix_apply(
                    nix_select(nix_var("lib"), "attrByPath"),
                    nix_list(path.into_iter().map(nix_str).collect()),
                ),
                default,
            ),
            attrs,
        )
    }
}
