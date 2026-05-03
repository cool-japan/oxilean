use super::super::functions::*;
use std::collections::HashMap;
use std::collections::{HashSet, VecDeque};

use super::defs::*;

impl PHPBackend {
    /// Create a new PHPBackend with default settings.
    pub fn new() -> Self {
        PHPBackend {
            indent: "    ".to_string(),
            mangle_cache: HashMap::new(),
            emit_docs: true,
        }
    }
    /// Create a PHPBackend with a custom indent string.
    pub fn with_indent(indent: impl Into<std::string::String>) -> Self {
        PHPBackend {
            indent: indent.into(),
            mangle_cache: HashMap::new(),
            emit_docs: true,
        }
    }
    /// Emit a PHP type hint as a string.
    pub fn emit_type(&self, ty: &PHPType) -> std::string::String {
        format!("{}", ty)
    }
    /// Emit a PHP expression as a string.
    pub fn emit_expr(&self, expr: &PHPExpr) -> std::string::String {
        format!("{}", expr)
    }
    /// Mangle an OxiLean name to a valid PHP identifier.
    ///
    /// PHP identifiers must match `[a-zA-Z_\x7f-\xff][a-zA-Z0-9_\x7f-\xff]*`.
    pub fn mangle_name(&self, name: &str) -> std::string::String {
        if let Some(cached) = self.mangle_cache.get(name) {
            return cached.clone();
        }
        let mut result = std::string::String::new();
        let mut first = true;
        for c in name.chars() {
            if first {
                if c.is_alphabetic() || c == '_' {
                    result.push(c);
                } else {
                    result.push('_');
                    if c.is_alphanumeric() {
                        result.push(c);
                    }
                }
                first = false;
            } else {
                match c {
                    'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => result.push(c),
                    '.' | ':' | '\'' => result.push('_'),
                    '-' => result.push('_'),
                    _ => {
                        let code = c as u32;
                        result.push_str(&format!("_u{:04X}_", code));
                    }
                }
            }
        }
        let reserved = [
            "abstract",
            "and",
            "array",
            "as",
            "break",
            "callable",
            "case",
            "catch",
            "class",
            "clone",
            "const",
            "continue",
            "declare",
            "default",
            "die",
            "do",
            "echo",
            "else",
            "elseif",
            "empty",
            "enddeclare",
            "endfor",
            "endforeach",
            "endif",
            "endswitch",
            "endwhile",
            "enum",
            "eval",
            "exit",
            "extends",
            "final",
            "finally",
            "fn",
            "for",
            "foreach",
            "function",
            "global",
            "goto",
            "if",
            "implements",
            "include",
            "include_once",
            "instanceof",
            "insteadof",
            "interface",
            "isset",
            "list",
            "match",
            "namespace",
            "new",
            "null",
            "or",
            "print",
            "private",
            "protected",
            "public",
            "readonly",
            "require",
            "require_once",
            "return",
            "static",
            "switch",
            "throw",
            "trait",
            "try",
            "unset",
            "use",
            "var",
            "while",
            "xor",
            "yield",
        ];
        if reserved.contains(&result.as_str()) {
            result.push_str("_ox");
        }
        result
    }
    /// Emit a parameter declaration.
    pub(crate) fn emit_param(&self, param: &PHPParam) -> std::string::String {
        format_param(param)
    }
    /// Emit a PHP function (top-level or standalone).
    pub fn emit_function(&self, func: &PHPFunction) -> std::string::String {
        let mut out = std::string::String::new();
        if self.emit_docs {
            if let Some(doc) = &func.doc_comment {
                out.push_str("/**\n");
                for line in doc.lines() {
                    out.push_str(&format!(" * {}\n", line));
                }
                out.push_str(" */\n");
            }
        }
        if let Some(vis) = &func.visibility {
            out.push_str(&format!("{} ", vis));
        }
        if func.is_static {
            out.push_str("static ");
        }
        if func.is_abstract {
            out.push_str("abstract ");
        }
        let params_s: Vec<std::string::String> =
            func.params.iter().map(|p| self.emit_param(p)).collect();
        out.push_str(&format!("function {}({})", func.name, params_s.join(", ")));
        if let Some(ret) = &func.return_type {
            out.push_str(&format!(": {}", ret));
        }
        if func.is_abstract {
            out.push_str(";\n");
        } else {
            out.push_str("\n{\n");
            for line in &func.body {
                out.push_str(&format!("{}{}\n", self.indent, line));
            }
            out.push_str("}\n");
        }
        out
    }
    /// Emit a PHP property declaration.
    pub(crate) fn emit_property(&self, prop: &PHPProperty) -> std::string::String {
        let mut s = format!("{} ", prop.visibility);
        if prop.is_static {
            s.push_str("static ");
        }
        if prop.readonly {
            s.push_str("readonly ");
        }
        if let Some(ty) = &prop.ty {
            s.push_str(&format!("{} ", ty));
        }
        s.push_str(&format!("${}", prop.name));
        if let Some(default) = &prop.default {
            s.push_str(&format!(" = {}", default));
        }
        s.push(';');
        s
    }
    /// Emit a PHP interface declaration.
    pub fn emit_interface(&self, iface: &PHPInterface) -> std::string::String {
        let mut out = std::string::String::new();
        out.push_str(&format!("interface {}", iface.name));
        if !iface.extends.is_empty() {
            out.push_str(&format!(" extends {}", iface.extends.join(", ")));
        }
        out.push_str("\n{\n");
        for (name, val) in &iface.constants {
            out.push_str(&format!("{}const {} = {};\n", self.indent, name, val));
        }
        for method in &iface.methods {
            out.push_str(&self.indent_block(&self.emit_function(method)));
        }
        out.push_str("}\n");
        out
    }
    /// Emit a PHP trait declaration.
    pub fn emit_trait(&self, tr: &PHPTrait) -> std::string::String {
        let mut out = std::string::String::new();
        out.push_str(&format!("trait {}\n{{\n", tr.name));
        for prop in &tr.properties {
            out.push_str(&format!("{}{}\n", self.indent, self.emit_property(prop)));
        }
        for method in &tr.methods {
            out.push_str(&self.indent_block(&self.emit_function(method)));
        }
        out.push_str("}\n");
        out
    }
    /// Emit a PHP 8.1 enum declaration.
    pub fn emit_enum(&self, en: &PHPEnum) -> std::string::String {
        let mut out = std::string::String::new();
        out.push_str(&format!("enum {}", en.name));
        if let Some(bt) = &en.backing_type {
            out.push_str(&format!(": {}", bt));
        }
        if !en.implements.is_empty() {
            out.push_str(&format!(" implements {}", en.implements.join(", ")));
        }
        out.push_str("\n{\n");
        for case in &en.cases {
            if let Some(val) = &case.value {
                out.push_str(&format!("{}case {} = {};\n", self.indent, case.name, val));
            } else {
                out.push_str(&format!("{}case {};\n", self.indent, case.name));
            }
        }
        for method in &en.methods {
            out.push_str(&self.indent_block(&self.emit_function(method)));
        }
        out.push_str("}\n");
        out
    }
    /// Emit a PHP class declaration.
    pub fn emit_class(&self, class: &PHPClass) -> std::string::String {
        let mut out = std::string::String::new();
        if class.is_abstract {
            out.push_str("abstract ");
        }
        if class.is_final {
            out.push_str("final ");
        }
        if class.is_readonly {
            out.push_str("readonly ");
        }
        out.push_str(&format!("class {}", class.name));
        if let Some(parent) = &class.parent {
            out.push_str(&format!(" extends {}", parent));
        }
        if !class.interfaces.is_empty() {
            out.push_str(&format!(" implements {}", class.interfaces.join(", ")));
        }
        out.push_str("\n{\n");
        for tr in &class.traits {
            out.push_str(&format!("{}use {};\n", self.indent, tr));
        }
        if !class.traits.is_empty() {
            out.push('\n');
        }
        for (name, ty, val) in &class.constants {
            out.push_str(&format!(
                "{}const {}: {} = {};\n",
                self.indent, name, ty, val
            ));
        }
        for prop in &class.properties {
            out.push_str(&format!("{}{}\n", self.indent, self.emit_property(prop)));
        }
        if !class.properties.is_empty() {
            out.push('\n');
        }
        for method in &class.methods {
            out.push_str(&self.indent_block(&self.emit_function(method)));
            out.push('\n');
        }
        out.push_str("}\n");
        out
    }
    /// Emit a complete PHP script.
    pub fn emit_script(&self, script: &PHPScript) -> std::string::String {
        let mut out = std::string::String::from("<?php\n");
        if script.strict_types {
            out.push_str("declare(strict_types=1);\n\n");
        }
        if let Some(ns) = &script.namespace {
            out.push_str(&format!("namespace {};\n\n", ns));
        }
        for (path, alias) in &script.uses {
            match alias {
                Some(a) => out.push_str(&format!("use {} as {};\n", path, a)),
                None => out.push_str(&format!("use {};\n", path)),
            }
        }
        if !script.uses.is_empty() {
            out.push('\n');
        }
        for iface in &script.interfaces {
            out.push_str(&self.emit_interface(iface));
            out.push('\n');
        }
        for tr in &script.traits {
            out.push_str(&self.emit_trait(tr));
            out.push('\n');
        }
        for en in &script.enums {
            out.push_str(&self.emit_enum(en));
            out.push('\n');
        }
        for class in &script.classes {
            out.push_str(&self.emit_class(class));
            out.push('\n');
        }
        for func in &script.functions {
            out.push_str(&self.emit_function(func));
            out.push('\n');
        }
        for line in &script.main {
            out.push_str(line);
            out.push('\n');
        }
        out
    }
    /// Indent each line of a block by one level.
    pub(crate) fn indent_block(&self, block: &str) -> std::string::String {
        block
            .lines()
            .map(|line| {
                if line.trim().is_empty() {
                    std::string::String::new()
                } else {
                    format!("{}{}", self.indent, line)
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
            + "\n"
    }
    /// Emit a namespace block.
    pub fn emit_namespace(&self, ns: &PHPNamespace) -> std::string::String {
        let mut script = PHPScript::new();
        script.namespace = Some(ns.path.clone());
        script.uses = ns.uses.clone();
        script.functions = ns.functions.clone();
        script.classes = ns.classes.clone();
        script.interfaces = ns.interfaces.clone();
        script.traits = ns.traits.clone();
        script.enums = ns.enums.clone();
        self.emit_script(&script)
    }
}

impl PHPExtDepGraph {
    #[allow(dead_code)]
    pub fn new(n: usize) -> Self {
        Self {
            n,
            adj: vec![Vec::new(); n],
            rev: vec![Vec::new(); n],
            edge_count: 0,
        }
    }
    #[allow(dead_code)]
    pub fn add_edge(&mut self, from: usize, to: usize) {
        if from < self.n && to < self.n {
            if !self.adj[from].contains(&to) {
                self.adj[from].push(to);
                self.rev[to].push(from);
                self.edge_count += 1;
            }
        }
    }
    #[allow(dead_code)]
    pub fn succs(&self, n: usize) -> &[usize] {
        self.adj.get(n).map(|v| v.as_slice()).unwrap_or(&[])
    }
    #[allow(dead_code)]
    pub fn preds(&self, n: usize) -> &[usize] {
        self.rev.get(n).map(|v| v.as_slice()).unwrap_or(&[])
    }
    #[allow(dead_code)]
    pub fn topo_sort(&self) -> Option<Vec<usize>> {
        let mut deg: Vec<usize> = (0..self.n).map(|i| self.rev[i].len()).collect();
        let mut q: std::collections::VecDeque<usize> =
            (0..self.n).filter(|&i| deg[i] == 0).collect();
        let mut out = Vec::with_capacity(self.n);
        while let Some(u) = q.pop_front() {
            out.push(u);
            for &v in &self.adj[u] {
                deg[v] -= 1;
                if deg[v] == 0 {
                    q.push_back(v);
                }
            }
        }
        if out.len() == self.n {
            Some(out)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn has_cycle(&self) -> bool {
        self.topo_sort().is_none()
    }
    #[allow(dead_code)]
    pub fn reachable(&self, start: usize) -> Vec<usize> {
        let mut vis = vec![false; self.n];
        let mut stk = vec![start];
        let mut out = Vec::new();
        while let Some(u) = stk.pop() {
            if u < self.n && !vis[u] {
                vis[u] = true;
                out.push(u);
                for &v in &self.adj[u] {
                    if !vis[v] {
                        stk.push(v);
                    }
                }
            }
        }
        out
    }
    #[allow(dead_code)]
    pub fn scc(&self) -> Vec<Vec<usize>> {
        let mut visited = vec![false; self.n];
        let mut order = Vec::new();
        for i in 0..self.n {
            if !visited[i] {
                let mut stk = vec![(i, 0usize)];
                while let Some((u, idx)) = stk.last_mut() {
                    if !visited[*u] {
                        visited[*u] = true;
                    }
                    if *idx < self.adj[*u].len() {
                        let v = self.adj[*u][*idx];
                        *idx += 1;
                        if !visited[v] {
                            stk.push((v, 0));
                        }
                    } else {
                        order.push(*u);
                        stk.pop();
                    }
                }
            }
        }
        let mut comp = vec![usize::MAX; self.n];
        let mut components: Vec<Vec<usize>> = Vec::new();
        for &start in order.iter().rev() {
            if comp[start] == usize::MAX {
                let cid = components.len();
                let mut component = Vec::new();
                let mut stk = vec![start];
                while let Some(u) = stk.pop() {
                    if comp[u] == usize::MAX {
                        comp[u] = cid;
                        component.push(u);
                        for &v in &self.rev[u] {
                            if comp[v] == usize::MAX {
                                stk.push(v);
                            }
                        }
                    }
                }
                components.push(component);
            }
        }
        components
    }
    #[allow(dead_code)]
    pub fn node_count(&self) -> usize {
        self.n
    }
    #[allow(dead_code)]
    pub fn edge_count(&self) -> usize {
        self.edge_count
    }
}

impl PHPPassStats {
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

impl PHPExtDomTree {
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

impl PHPEnum {
    /// Create a new pure enum.
    pub fn new(name: impl Into<std::string::String>) -> Self {
        PHPEnum {
            name: name.into(),
            backing_type: None,
            cases: vec![],
            implements: vec![],
            methods: vec![],
        }
    }
    /// Create a string-backed enum.
    pub fn string_backed(name: impl Into<std::string::String>) -> Self {
        PHPEnum {
            name: name.into(),
            backing_type: Some(PHPType::String),
            cases: vec![],
            implements: vec![],
            methods: vec![],
        }
    }
}

impl PHPPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: PHPPassPhase) -> Self {
        PHPPassConfig {
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

impl PHPConstantFoldingHelper {
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

impl PHPProperty {
    /// Create a public property.
    pub fn public(name: impl Into<std::string::String>, ty: Option<PHPType>) -> Self {
        PHPProperty {
            name: name.into(),
            ty,
            visibility: PHPVisibility::Public,
            is_static: false,
            readonly: false,
            default: None,
        }
    }
    /// Create a private property.
    pub fn private(name: impl Into<std::string::String>, ty: Option<PHPType>) -> Self {
        PHPProperty {
            name: name.into(),
            ty,
            visibility: PHPVisibility::Private,
            is_static: false,
            readonly: false,
            default: None,
        }
    }
}

impl PHPNamespace {
    /// Create a new namespace.
    pub fn new(path: impl Into<std::string::String>) -> Self {
        PHPNamespace {
            path: path.into(),
            uses: vec![],
            functions: vec![],
            classes: vec![],
            interfaces: vec![],
            traits: vec![],
            enums: vec![],
        }
    }
}

impl PHPDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        PHPDepGraph {
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

impl PHPInterface {
    /// Create a new interface.
    pub fn new(name: impl Into<std::string::String>) -> Self {
        PHPInterface {
            name: name.into(),
            extends: vec![],
            methods: vec![],
            constants: vec![],
        }
    }
}
