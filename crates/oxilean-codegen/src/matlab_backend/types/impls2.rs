//! Implementation blocks (part 2)

use super::defs::*;
use std::collections::HashMap;
use std::collections::{HashSet, VecDeque};
use std::fmt::Write as FmtWrite;

impl MatlabMatrix {
    /// Create a new matrix.
    #[allow(dead_code)]
    pub fn new() -> Self {
        MatlabMatrix { rows: Vec::new() }
    }
    /// Add a row to the matrix.
    #[allow(dead_code)]
    pub fn add_row(mut self, row: Vec<MatlabExpr>) -> Self {
        self.rows.push(row);
        self
    }
    /// Number of rows.
    #[allow(dead_code)]
    pub fn num_rows(&self) -> usize {
        self.rows.len()
    }
    /// Number of columns (from the first row).
    #[allow(dead_code)]
    pub fn num_cols(&self) -> usize {
        self.rows.first().map(|r| r.len()).unwrap_or(0)
    }
    /// Emit as a MATLAB matrix literal.
    #[allow(dead_code)]
    pub fn emit(&self) -> String {
        let rows: Vec<String> = self
            .rows
            .iter()
            .map(|r| {
                r.iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            })
            .collect();
        format!("[{}]", rows.join("; "))
    }
    /// Create an identity matrix of size `n`.
    #[allow(dead_code)]
    pub fn identity(n: usize) -> MatlabExpr {
        MatlabExpr::Call(
            Box::new(MatlabExpr::Var("eye".to_string())),
            vec![MatlabExpr::Lit(MatlabLiteral::Integer(n as i64))],
        )
    }
    /// Create a zeros matrix of shape `(m, n)`.
    #[allow(dead_code)]
    pub fn zeros(m: usize, n: usize) -> MatlabExpr {
        MatlabExpr::Call(
            Box::new(MatlabExpr::Var("zeros".to_string())),
            vec![
                MatlabExpr::Lit(MatlabLiteral::Integer(m as i64)),
                MatlabExpr::Lit(MatlabLiteral::Integer(n as i64)),
            ],
        )
    }
    /// Create an ones matrix of shape `(m, n)`.
    #[allow(dead_code)]
    pub fn ones(m: usize, n: usize) -> MatlabExpr {
        MatlabExpr::Call(
            Box::new(MatlabExpr::Var("ones".to_string())),
            vec![
                MatlabExpr::Lit(MatlabLiteral::Integer(m as i64)),
                MatlabExpr::Lit(MatlabLiteral::Integer(n as i64)),
            ],
        )
    }
}
impl MatlabStats {
    /// Compute stats from a module builder.
    #[allow(dead_code)]
    pub fn from_module(module: &MatlabModuleBuilder) -> Self {
        let total_stmts = module.functions.iter().map(|f| f.body.len()).sum::<usize>();
        MatlabStats {
            num_functions: module.functions.len(),
            num_classes: module.classes.len(),
            total_stmts,
            matrix_ops: 0,
        }
    }
    /// Merge another stats record.
    #[allow(dead_code)]
    pub fn merge(&mut self, other: &MatlabStats) {
        self.num_functions += other.num_functions;
        self.num_classes += other.num_classes;
        self.total_stmts += other.total_stmts;
        self.matrix_ops += other.matrix_ops;
    }
}
impl MatlabParam {
    pub fn required(name: &str) -> Self {
        MatlabParam {
            name: name.to_string(),
            default_value: None,
            validator: None,
        }
    }
    pub fn with_default(name: &str, default: MatlabExpr) -> Self {
        MatlabParam {
            name: name.to_string(),
            default_value: Some(default),
            validator: None,
        }
    }
    pub fn typed(name: &str, ty: MatlabType) -> Self {
        MatlabParam {
            name: name.to_string(),
            default_value: None,
            validator: Some(ty),
        }
    }
}
impl MatlabClassdef {
    pub fn new(name: &str) -> Self {
        MatlabClassdef {
            name: name.to_string(),
            superclasses: Vec::new(),
            properties: Vec::new(),
            methods: Vec::new(),
            events: Vec::new(),
            enumerations: Vec::new(),
        }
    }
    pub fn inherits(mut self, parent: &str) -> Self {
        self.superclasses.push(parent.to_string());
        self
    }
}
impl MatlabFile {
    pub fn new() -> Self {
        MatlabFile {
            functions: Vec::new(),
            scripts: Vec::new(),
            classdef: None,
            header_comment: None,
            is_script: false,
        }
    }
    pub fn script() -> Self {
        MatlabFile {
            is_script: true,
            ..Self::new()
        }
    }
    pub fn add_function(&mut self, fun: MatlabFunction) {
        self.functions.push(fun);
    }
    pub fn add_script_stmt(&mut self, stmt: MatlabStmt) {
        self.scripts.push(stmt);
    }
    pub fn with_classdef(mut self, cls: MatlabClassdef) -> Self {
        self.classdef = Some(cls);
        self
    }
    pub fn with_header(mut self, comment: &str) -> Self {
        self.header_comment = Some(comment.to_string());
        self
    }
}
impl MatlabScript {
    /// Create a new script.
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        MatlabScript {
            name: name.into(),
            header_comments: Vec::new(),
            statements: Vec::new(),
        }
    }
    /// Add a header comment.
    #[allow(dead_code)]
    pub fn add_comment(mut self, comment: impl Into<String>) -> Self {
        self.header_comments.push(comment.into());
        self
    }
    /// Add a statement.
    #[allow(dead_code)]
    pub fn add_stmt(mut self, stmt: MatlabStmt) -> Self {
        self.statements.push(stmt);
        self
    }
    /// Emit the full script source.
    #[allow(dead_code)]
    pub fn emit(&self) -> String {
        let mut backend = MatlabBackend::new();
        for comment in &self.header_comments {
            backend.emit_stmt(&MatlabStmt::Comment(comment.clone()));
        }
        for stmt in &self.statements {
            backend.emit_stmt(stmt);
        }
        backend.take_output()
    }
}
impl MatlabPlot {
    /// Create a new plot with defaults.
    #[allow(dead_code)]
    pub fn new(title: impl Into<String>) -> Self {
        MatlabPlot {
            title: title.into(),
            xlabel: String::new(),
            ylabel: String::new(),
            series: Vec::new(),
            grid: true,
            legend: false,
            figure_size: None,
        }
    }
    /// Add a data series.
    #[allow(dead_code)]
    pub fn add_series(mut self, var: impl Into<String>, style: impl Into<String>) -> Self {
        self.series.push((var.into(), style.into()));
        self
    }
    /// Set axis labels.
    #[allow(dead_code)]
    pub fn labels(mut self, xlabel: impl Into<String>, ylabel: impl Into<String>) -> Self {
        self.xlabel = xlabel.into();
        self.ylabel = ylabel.into();
        self
    }
    /// Enable legend.
    #[allow(dead_code)]
    pub fn with_legend(mut self) -> Self {
        self.legend = true;
        self
    }
    /// Emit MATLAB plotting code.
    #[allow(dead_code)]
    pub fn emit(&self) -> String {
        let mut out = String::new();
        out.push_str("figure;\n");
        if let Some([w, h]) = self.figure_size {
            out.push_str(&format!(
                "set(gcf, 'Position', [100, 100, {}, {}]);\n",
                w, h
            ));
        }
        for (i, (var, style)) in self.series.iter().enumerate() {
            if i == 0 {
                out.push_str(&format!("plot({}, '{}');\n", var, style));
            } else {
                out.push_str("hold on;\n");
                out.push_str(&format!("plot({}, '{}');\n", var, style));
            }
        }
        if !self.series.is_empty() {
            out.push_str("hold off;\n");
        }
        if !self.title.is_empty() {
            out.push_str(&format!("title('{}');\n", self.title));
        }
        if !self.xlabel.is_empty() {
            out.push_str(&format!("xlabel('{}');\n", self.xlabel));
        }
        if !self.ylabel.is_empty() {
            out.push_str(&format!("ylabel('{}');\n", self.ylabel));
        }
        if self.grid {
            out.push_str("grid on;\n");
        }
        if self.legend {
            let labels: Vec<_> = self
                .series
                .iter()
                .map(|(v, _)| format!("'{}'", v))
                .collect();
            out.push_str(&format!("legend({});\n", labels.join(", ")));
        }
        out
    }
}
impl MatlabStructField {
    /// Create a new struct field.
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, value: MatlabExpr) -> Self {
        MatlabStructField {
            name: name.into(),
            value,
        }
    }
}
impl MatlabValidation {
    /// Emit `validateattributes(x, {'class'}, {attrs...})`.
    #[allow(dead_code)]
    pub fn validate_attributes(var: &str, class: &str, attributes: &[&str]) -> MatlabStmt {
        let attrs_str = attributes
            .iter()
            .map(|a| format!("'{}'", a))
            .collect::<Vec<_>>()
            .join(", ");
        MatlabStmt::Expr(
            MatlabExpr::Call(
                Box::new(MatlabExpr::Var("validateattributes".to_string())),
                vec![
                    MatlabExpr::Var(var.to_string()),
                    MatlabExpr::Lit(MatlabLiteral::Char(format!("{{{{'{}'}}}}", class))),
                    MatlabExpr::Lit(MatlabLiteral::Char(format!("{{{{{}}}}}", attrs_str))),
                ],
            ),
            true,
        )
    }
    /// Emit `narginchk(min, max)`.
    #[allow(dead_code)]
    pub fn narginchk(min: i64, max: i64) -> MatlabStmt {
        MatlabStmt::Expr(
            MatlabExpr::Call(
                Box::new(MatlabExpr::Var("narginchk".to_string())),
                vec![
                    MatlabExpr::Lit(MatlabLiteral::Integer(min)),
                    MatlabExpr::Lit(MatlabLiteral::Integer(max)),
                ],
            ),
            true,
        )
    }
    /// Emit `nargoutchk(min, max)`.
    #[allow(dead_code)]
    pub fn nargoutchk(min: i64, max: i64) -> MatlabStmt {
        MatlabStmt::Expr(
            MatlabExpr::Call(
                Box::new(MatlabExpr::Var("nargoutchk".to_string())),
                vec![
                    MatlabExpr::Lit(MatlabLiteral::Integer(min)),
                    MatlabExpr::Lit(MatlabLiteral::Integer(max)),
                ],
            ),
            true,
        )
    }
}
impl MatlabCellArray {
    /// Create a new empty cell array.
    #[allow(dead_code)]
    pub fn new() -> Self {
        MatlabCellArray {
            elements: Vec::new(),
        }
    }
    /// Add an element.
    #[allow(dead_code)]
    pub fn add(mut self, elem: MatlabExpr) -> Self {
        self.elements.push(elem);
        self
    }
    /// Emit as a MATLAB cell array literal `{...}`.
    #[allow(dead_code)]
    pub fn emit(&self) -> String {
        let elems: Vec<_> = self.elements.iter().map(|e| e.to_string()).collect();
        format!("{{{}}}", elems.join(", "))
    }
}
impl MatlabBackend {
    /// Create a new MATLAB backend.
    pub fn new() -> Self {
        MatlabBackend {
            output: String::new(),
            indent: 0,
            indent_str: "  ".to_string(),
            classes: HashMap::new(),
            octave_compat: false,
        }
    }
    /// Create a backend configured for Octave compatibility.
    pub fn octave() -> Self {
        MatlabBackend {
            octave_compat: true,
            ..Self::new()
        }
    }
    /// Take the accumulated output, resetting the buffer.
    pub fn take_output(&mut self) -> String {
        std::mem::take(&mut self.output)
    }
    /// Register a known class definition.
    pub fn register_class(&mut self, cls: MatlabClassdef) {
        self.classes.insert(cls.name.clone(), cls);
    }
    pub(super) fn current_indent(&self) -> String {
        self.indent_str.repeat(self.indent)
    }
    pub(super) fn emit_line(&mut self, line: &str) {
        let indent = self.current_indent();
        let _ = writeln!(self.output, "{}{}", indent, line);
    }
    pub(super) fn emit_raw(&mut self, s: &str) {
        self.output.push_str(s);
    }
    pub(super) fn indent_up(&mut self) {
        self.indent += 1;
    }
    pub(super) fn indent_down(&mut self) {
        if self.indent > 0 {
            self.indent -= 1;
        }
    }
    /// Emit a complete MATLAB file.
    pub fn emit_file(&mut self, file: &MatlabFile) {
        if let Some(header) = &file.header_comment {
            for line in header.lines() {
                self.emit_line(&format!("% {}", line));
            }
            self.emit_line("");
        }
        if let Some(cls) = &file.classdef.clone() {
            self.emit_classdef(cls);
            return;
        }
        if file.is_script {
            for stmt in &file.scripts.clone() {
                self.emit_stmt(stmt);
            }
            return;
        }
        for (idx, fun) in file.functions.iter().enumerate() {
            if idx > 0 {
                self.emit_line("");
            }
            self.emit_function(fun);
        }
    }
    /// Emit a MATLAB function definition.
    pub fn emit_function(&mut self, fun: &MatlabFunction) {
        let outputs_str = match fun.outputs.len() {
            0 => String::new(),
            1 => format!("{} = ", fun.outputs[0]),
            _ => format!("[{}] = ", fun.outputs.join(", ")),
        };
        let inputs_str: Vec<String> = fun.inputs.iter().map(|p| p.name.clone()).collect();
        self.emit_line(&format!(
            "function {}{}({})",
            outputs_str,
            fun.name,
            inputs_str.join(", ")
        ));
        self.indent_up();
        if let Some(help) = &fun.help_text {
            for line in help.lines() {
                self.emit_line(&format!("% {}", line));
            }
        }
        if !fun.argument_validation.is_empty() {
            self.emit_line("arguments");
            self.indent_up();
            for av in &fun.argument_validation {
                self.emit_arg_validation(av);
            }
            self.indent_down();
            self.emit_line("end");
        }
        for stmt in &fun.body {
            self.emit_stmt(stmt);
        }
        self.indent_down();
        self.emit_line("end");
    }
    pub(super) fn emit_arg_validation(&mut self, av: &MatlabArgValidation) {
        let size_str = if let Some(sizes) = &av.size {
            let s: Vec<String> = sizes
                .iter()
                .map(|d| d.map(|n| n.to_string()).unwrap_or_else(|| ":".to_string()))
                .collect();
            format!("({}) ", s.join(","))
        } else {
            String::new()
        };
        let class_str = if let Some(cls) = &av.class {
            format!("{} ", cls)
        } else {
            String::new()
        };
        let validators_str = if !av.validators.is_empty() {
            format!(" {{{}}}", av.validators.join(", "))
        } else {
            String::new()
        };
        let default_str = if let Some(def) = &av.default {
            format!(" = {}", self.emit_expr(def))
        } else {
            String::new()
        };
        self.emit_line(&format!(
            "{}{}{}{}{}",
            av.name, size_str, class_str, validators_str, default_str
        ));
    }
    /// Emit a MATLAB classdef.
    pub fn emit_classdef(&mut self, cls: &MatlabClassdef) {
        let inherits_str = if cls.superclasses.is_empty() {
            String::new()
        } else {
            format!(" < {}", cls.superclasses.join(" & "))
        };
        self.emit_line(&format!("classdef {}{}", cls.name, inherits_str));
        self.indent_up();
        let pub_props: Vec<&MatlabProperty> = cls
            .properties
            .iter()
            .filter(|p| p.access == PropAccess::Public)
            .collect();
        let prot_props: Vec<&MatlabProperty> = cls
            .properties
            .iter()
            .filter(|p| p.access == PropAccess::Protected)
            .collect();
        let priv_props: Vec<&MatlabProperty> = cls
            .properties
            .iter()
            .filter(|p| p.access == PropAccess::Private)
            .collect();
        if !pub_props.is_empty() {
            self.emit_line("properties");
            self.indent_up();
            for prop in pub_props {
                self.emit_property(prop);
            }
            self.indent_down();
            self.emit_line("end");
        }
        if !prot_props.is_empty() {
            self.emit_line("properties (Access = protected)");
            self.indent_up();
            for prop in prot_props {
                self.emit_property(prop);
            }
            self.indent_down();
            self.emit_line("end");
        }
        if !priv_props.is_empty() {
            self.emit_line("properties (Access = private)");
            self.indent_up();
            for prop in priv_props {
                self.emit_property(prop);
            }
            self.indent_down();
            self.emit_line("end");
        }
        if !cls.events.is_empty() {
            self.emit_line("events");
            self.indent_up();
            for ev in &cls.events {
                self.emit_line(ev);
            }
            self.indent_down();
            self.emit_line("end");
        }
        if !cls.enumerations.is_empty() {
            self.emit_line("enumeration");
            self.indent_up();
            for (name, args) in &cls.enumerations {
                let args_str: Vec<String> = args.iter().map(|a| self.emit_expr(a)).collect();
                if args_str.is_empty() {
                    self.emit_line(name);
                } else {
                    self.emit_line(&format!("{}({})", name, args_str.join(", ")));
                }
            }
            self.indent_down();
            self.emit_line("end");
        }
        if !cls.methods.is_empty() {
            self.emit_line("methods");
            self.indent_up();
            for method in &cls.methods.clone() {
                self.emit_function(method);
                self.emit_line("");
            }
            self.indent_down();
            self.emit_line("end");
        }
        self.indent_down();
        self.emit_line("end");
    }
    pub(super) fn emit_property(&mut self, prop: &MatlabProperty) {
        let ty_str = if let Some(ty) = &prop.ty {
            format!(" ({})", ty)
        } else {
            String::new()
        };
        if let Some(default) = &prop.default {
            let default_str = self.emit_expr(default);
            self.emit_line(&format!("{}{} = {}", prop.name, ty_str, default_str));
        } else {
            self.emit_line(&format!("{}{}", prop.name, ty_str));
        }
    }
    /// Emit a single MATLAB statement.
    pub fn emit_stmt(&mut self, stmt: &MatlabStmt) {
        match stmt {
            MatlabStmt::Assign { lhs, rhs, suppress } => {
                let rhs_str = self.emit_expr(rhs);
                let semi = if *suppress { ";" } else { "" };
                match lhs.len() {
                    0 => self.emit_line(&format!("{}{}", rhs_str, semi)),
                    1 => self.emit_line(&format!("{} = {}{}", lhs[0], rhs_str, semi)),
                    _ => self.emit_line(&format!("[{}] = {}{}", lhs.join(", "), rhs_str, semi)),
                }
            }
            MatlabStmt::AssignIndex {
                obj,
                indices,
                cell_index,
                rhs,
                suppress,
            } => {
                let obj_str = self.emit_expr(obj);
                let idx_str: Vec<String> = indices.iter().map(|i| self.emit_expr(i)).collect();
                let rhs_str = self.emit_expr(rhs);
                let semi = if *suppress { ";" } else { "" };
                let (open, close) = if *cell_index { ("{", "}") } else { ("(", ")") };
                self.emit_line(&format!(
                    "{}{}{}{}{}){} = {}{}",
                    obj_str,
                    open,
                    idx_str.join(", "),
                    close,
                    "",
                    "",
                    rhs_str,
                    semi
                ));
                if let Some(bad) = self.output.lines().last().map(|l| l.to_string()) {
                    let len_to_remove = bad.len() + 1;
                    let new_len = self.output.len().saturating_sub(len_to_remove);
                    self.output.truncate(new_len);
                }
                let indent = self.current_indent();
                let _ = writeln!(
                    self.output,
                    "{}{}{}{}{}{}{}",
                    indent,
                    obj_str,
                    open,
                    idx_str.join(", "),
                    close,
                    format_args!(" = {}", rhs_str),
                    semi
                );
            }
            MatlabStmt::AssignField {
                obj,
                field,
                rhs,
                suppress,
            } => {
                let rhs_str = self.emit_expr(rhs);
                let semi = if *suppress { ";" } else { "" };
                self.emit_line(&format!("{}.{} = {}{}", obj, field, rhs_str, semi));
            }
            MatlabStmt::ForLoop { var, range, body } => {
                let range_str = self.emit_expr(range);
                self.emit_line(&format!("for {} = {}", var, range_str));
                self.indent_up();
                for s in body {
                    self.emit_stmt(s);
                }
                self.indent_down();
                self.emit_line("end");
            }
            MatlabStmt::WhileLoop { cond, body } => {
                let cond_str = self.emit_expr(cond);
                self.emit_line(&format!("while {}", cond_str));
                self.indent_up();
                for s in body {
                    self.emit_stmt(s);
                }
                self.indent_down();
                self.emit_line("end");
            }
            MatlabStmt::IfElseIf {
                cond,
                then_body,
                elseif_branches,
                else_body,
            } => {
                let cond_str = self.emit_expr(cond);
                self.emit_line(&format!("if {}", cond_str));
                self.indent_up();
                for s in then_body {
                    self.emit_stmt(s);
                }
                self.indent_down();
                for (elif_cond, elif_body) in elseif_branches {
                    let elif_str = self.emit_expr(elif_cond);
                    self.emit_line(&format!("elseif {}", elif_str));
                    self.indent_up();
                    for s in elif_body {
                        self.emit_stmt(s);
                    }
                    self.indent_down();
                }
                if let Some(else_stmts) = else_body {
                    self.emit_line("else");
                    self.indent_up();
                    for s in else_stmts {
                        self.emit_stmt(s);
                    }
                    self.indent_down();
                }
                self.emit_line("end");
            }
            MatlabStmt::SwitchCase {
                expr,
                cases,
                otherwise,
            } => {
                let expr_str = self.emit_expr(expr);
                self.emit_line(&format!("switch {}", expr_str));
                self.indent_up();
                for (val, body) in cases {
                    let val_str = self.emit_expr(val);
                    self.emit_line(&format!("case {}", val_str));
                    self.indent_up();
                    for s in body {
                        self.emit_stmt(s);
                    }
                    self.indent_down();
                }
                if let Some(other_stmts) = otherwise {
                    self.emit_line("otherwise");
                    self.indent_up();
                    for s in other_stmts {
                        self.emit_stmt(s);
                    }
                    self.indent_down();
                }
                self.indent_down();
                self.emit_line("end");
            }
            MatlabStmt::Return => self.emit_line("return;"),
            MatlabStmt::Break => self.emit_line("break;"),
            MatlabStmt::Continue => self.emit_line("continue;"),
            MatlabStmt::Error(fmt_expr, args) => {
                let fmt_str = self.emit_expr(fmt_expr);
                if args.is_empty() {
                    self.emit_line(&format!("error({});", fmt_str));
                } else {
                    let args_str: Vec<String> = args.iter().map(|a| self.emit_expr(a)).collect();
                    self.emit_line(&format!("error({}, {});", fmt_str, args_str.join(", ")));
                }
            }
            MatlabStmt::Warning(fmt_expr, args) => {
                let fmt_str = self.emit_expr(fmt_expr);
                if args.is_empty() {
                    self.emit_line(&format!("warning({});", fmt_str));
                } else {
                    let args_str: Vec<String> = args.iter().map(|a| self.emit_expr(a)).collect();
                    self.emit_line(&format!("warning({}, {});", fmt_str, args_str.join(", ")));
                }
            }
            MatlabStmt::Disp(expr) => {
                let e_str = self.emit_expr(expr);
                self.emit_line(&format!("disp({});", e_str));
            }
            MatlabStmt::FunctionDef(fun) => {
                self.emit_function(fun);
            }
            MatlabStmt::TryCatch {
                body,
                catch_var,
                catch_body,
            } => {
                self.emit_line("try");
                self.indent_up();
                for s in body {
                    self.emit_stmt(s);
                }
                self.indent_down();
                if let Some(var) = catch_var {
                    self.emit_line(&format!("catch {}", var));
                } else {
                    self.emit_line("catch");
                }
                self.indent_up();
                for s in catch_body {
                    self.emit_stmt(s);
                }
                self.indent_down();
                self.emit_line("end");
            }
            MatlabStmt::ValidateProp(name, expr) => {
                let e_str = self.emit_expr(expr);
                self.emit_line(&format!("validateattributes({}, {});", name, e_str));
            }
            MatlabStmt::Expr(expr, suppress) => {
                let e_str = self.emit_expr(expr);
                let semi = if *suppress { ";" } else { "" };
                self.emit_line(&format!("{}{}", e_str, semi));
            }
            MatlabStmt::Comment(text) => {
                for line in text.lines() {
                    self.emit_line(&format!("% {}", line));
                }
            }
            MatlabStmt::Global(names) => {
                self.emit_line(&format!("global {}", names.join(" ")));
            }
            MatlabStmt::Persistent(names) => {
                self.emit_line(&format!("persistent {}", names.join(" ")));
            }
            MatlabStmt::ClassdefStmt(s) => {
                self.emit_line(s);
            }
        }
    }
    /// Emit a MATLAB expression to a string.
    pub fn emit_expr(&mut self, expr: &MatlabExpr) -> String {
        self.emit_expr_pure(expr)
    }
    /// Emit a MATLAB expression to a string (pure).
    pub fn emit_expr_pure(&self, expr: &MatlabExpr) -> String {
        match expr {
            MatlabExpr::Lit(lit) => self.emit_literal(lit),
            MatlabExpr::Var(name) => name.clone(),
            MatlabExpr::MatrixLit(rows) => {
                let rows_str: Vec<String> = rows
                    .iter()
                    .map(|row| {
                        let elems: Vec<String> =
                            row.iter().map(|e| self.emit_expr_pure(e)).collect();
                        elems.join(", ")
                    })
                    .collect();
                format!("[{}]", rows_str.join("; "))
            }
            MatlabExpr::CellLit(rows) => {
                let rows_str: Vec<String> = rows
                    .iter()
                    .map(|row| {
                        let elems: Vec<String> =
                            row.iter().map(|e| self.emit_expr_pure(e)).collect();
                        elems.join(", ")
                    })
                    .collect();
                format!("{{{}}}", rows_str.join("; "))
            }
            MatlabExpr::ColonRange { start, step, end } => {
                let start_str = self.emit_expr_pure(start);
                let end_str = self.emit_expr_pure(end);
                if let Some(step_expr) = step {
                    let step_str = self.emit_expr_pure(step_expr);
                    format!("{}:{}:{}", start_str, step_str, end_str)
                } else {
                    format!("{}:{}", start_str, end_str)
                }
            }
            MatlabExpr::Call(func, args) => {
                let func_str = self.emit_expr_pure(func);
                let args_str: Vec<String> = args.iter().map(|a| self.emit_expr_pure(a)).collect();
                format!("{}({})", func_str, args_str.join(", "))
            }
            MatlabExpr::Index {
                obj,
                indices,
                cell_index,
            } => {
                let obj_str = self.emit_expr_pure(obj);
                let idx_str: Vec<String> = indices.iter().map(|i| self.emit_expr_pure(i)).collect();
                let (open, close) = if *cell_index { ("{", "}") } else { ("(", ")") };
                format!("{}{}{}{}", obj_str, open, idx_str.join(", "), close)
            }
            MatlabExpr::FieldAccess(obj, field) => {
                let obj_str = self.emit_expr_pure(obj);
                format!("{}.{}", obj_str, field)
            }
            MatlabExpr::BinaryOp(op, lhs, rhs) => {
                let lhs_str = self.emit_expr_pure(lhs);
                let rhs_str = self.emit_expr_pure(rhs);
                format!("{} {} {}", lhs_str, op, rhs_str)
            }
            MatlabExpr::UnaryOp(op, operand, postfix) => {
                let operand_str = self.emit_expr_pure(operand);
                if *postfix {
                    format!("{}{}", operand_str, op)
                } else {
                    format!("{}{}", op, operand_str)
                }
            }
            MatlabExpr::IfExpr(cond, then_expr, else_expr) => {
                let cond_str = self.emit_expr_pure(cond);
                let then_str = self.emit_expr_pure(then_expr);
                let else_str = self.emit_expr_pure(else_expr);
                format!("({{{}; {}}}{{{}+1}})", else_str, then_str, cond_str)
            }
            MatlabExpr::AnonFunc(params, body) => {
                let params_str = params.join(", ");
                let body_str = self.emit_expr_pure(body);
                format!("@({}) {}", params_str, body_str)
            }
            MatlabExpr::End => "end".to_string(),
            MatlabExpr::Colon => ":".to_string(),
            MatlabExpr::Nargin => "nargin".to_string(),
            MatlabExpr::Nargout => "nargout".to_string(),
        }
    }
    pub(crate) fn emit_literal(&self, lit: &MatlabLiteral) -> String {
        match lit {
            MatlabLiteral::Double(f) => {
                if f.fract() == 0.0 && f.abs() < 1e15 {
                    format!("{}", *f as i64)
                } else {
                    format!("{}", f)
                }
            }
            MatlabLiteral::Integer(n) => format!("{}", n),
            MatlabLiteral::Logical(b) => {
                if *b {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }
            MatlabLiteral::Char(s) => format!("'{}'", s.replace('\'', "''")),
            MatlabLiteral::Str(s) => format!("\"{}\"", s.replace('"', "\"\"")),
            MatlabLiteral::Empty => "[]".to_string(),
            MatlabLiteral::NaN => "NaN".to_string(),
            MatlabLiteral::Inf(neg) => {
                if *neg {
                    "-Inf".to_string()
                } else {
                    "Inf".to_string()
                }
            }
            MatlabLiteral::Pi => "pi".to_string(),
            MatlabLiteral::Eps => "eps".to_string(),
        }
    }
}
impl MatlabLivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        MatlabLivenessInfo {
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
impl MatlabNumericOps {
    /// Element-wise multiply `a .* b`.
    #[allow(dead_code)]
    pub fn elem_mul(a: MatlabExpr, b: MatlabExpr) -> MatlabExpr {
        MatlabExpr::BinaryOp(".*".to_string(), Box::new(a), Box::new(b))
    }
    /// Element-wise divide `a ./ b`.
    #[allow(dead_code)]
    pub fn elem_div(a: MatlabExpr, b: MatlabExpr) -> MatlabExpr {
        MatlabExpr::BinaryOp("./".to_string(), Box::new(a), Box::new(b))
    }
    /// Matrix multiply `a * b`.
    #[allow(dead_code)]
    pub fn mat_mul(a: MatlabExpr, b: MatlabExpr) -> MatlabExpr {
        MatlabExpr::BinaryOp("*".to_string(), Box::new(a), Box::new(b))
    }
    /// Matrix power `a ^ n`.
    #[allow(dead_code)]
    pub fn mat_pow(a: MatlabExpr, n: MatlabExpr) -> MatlabExpr {
        MatlabExpr::BinaryOp("^".to_string(), Box::new(a), Box::new(n))
    }
    /// Element-wise power `a .^ n`.
    #[allow(dead_code)]
    pub fn elem_pow(a: MatlabExpr, n: MatlabExpr) -> MatlabExpr {
        MatlabExpr::BinaryOp(".^".to_string(), Box::new(a), Box::new(n))
    }
    /// Colon range `start:stop`.
    #[allow(dead_code)]
    pub fn range(start: MatlabExpr, stop: MatlabExpr) -> MatlabExpr {
        MatlabExpr::ColonRange {
            start: Box::new(start),
            step: None,
            end: Box::new(stop),
        }
    }
    /// Colon range with step `start:step:stop`.
    #[allow(dead_code)]
    pub fn range_step(start: MatlabExpr, step: MatlabExpr, stop: MatlabExpr) -> MatlabExpr {
        MatlabExpr::ColonRange {
            start: Box::new(start),
            step: Some(Box::new(step)),
            end: Box::new(stop),
        }
    }
    /// `abs(x)`.
    #[allow(dead_code)]
    pub fn abs(x: MatlabExpr) -> MatlabExpr {
        MatlabExpr::Call(Box::new(MatlabExpr::Var("abs".to_string())), vec![x])
    }
    /// `sum(x)`.
    #[allow(dead_code)]
    pub fn sum(x: MatlabExpr) -> MatlabExpr {
        MatlabExpr::Call(Box::new(MatlabExpr::Var("sum".to_string())), vec![x])
    }
    /// `prod(x)`.
    #[allow(dead_code)]
    pub fn prod(x: MatlabExpr) -> MatlabExpr {
        MatlabExpr::Call(Box::new(MatlabExpr::Var("prod".to_string())), vec![x])
    }
    /// `min(x)`.
    #[allow(dead_code)]
    pub fn min(x: MatlabExpr) -> MatlabExpr {
        MatlabExpr::Call(Box::new(MatlabExpr::Var("min".to_string())), vec![x])
    }
    /// `max(x)`.
    #[allow(dead_code)]
    pub fn max(x: MatlabExpr) -> MatlabExpr {
        MatlabExpr::Call(Box::new(MatlabExpr::Var("max".to_string())), vec![x])
    }
    /// `mean(x)`.
    #[allow(dead_code)]
    pub fn mean(x: MatlabExpr) -> MatlabExpr {
        MatlabExpr::Call(Box::new(MatlabExpr::Var("mean".to_string())), vec![x])
    }
    /// `std(x)`.
    #[allow(dead_code)]
    pub fn std(x: MatlabExpr) -> MatlabExpr {
        MatlabExpr::Call(Box::new(MatlabExpr::Var("std".to_string())), vec![x])
    }
    /// `sqrt(x)`.
    #[allow(dead_code)]
    pub fn sqrt(x: MatlabExpr) -> MatlabExpr {
        MatlabExpr::Call(Box::new(MatlabExpr::Var("sqrt".to_string())), vec![x])
    }
    /// `norm(x)`.
    #[allow(dead_code)]
    pub fn norm(x: MatlabExpr) -> MatlabExpr {
        MatlabExpr::Call(Box::new(MatlabExpr::Var("norm".to_string())), vec![x])
    }
    /// `det(A)`.
    #[allow(dead_code)]
    pub fn det(a: MatlabExpr) -> MatlabExpr {
        MatlabExpr::Call(Box::new(MatlabExpr::Var("det".to_string())), vec![a])
    }
    /// `inv(A)`.
    #[allow(dead_code)]
    pub fn inv(a: MatlabExpr) -> MatlabExpr {
        MatlabExpr::Call(Box::new(MatlabExpr::Var("inv".to_string())), vec![a])
    }
    /// `eig(A)`.
    #[allow(dead_code)]
    pub fn eig(a: MatlabExpr) -> MatlabExpr {
        MatlabExpr::Call(Box::new(MatlabExpr::Var("eig".to_string())), vec![a])
    }
    /// `svd(A)`.
    #[allow(dead_code)]
    pub fn svd(a: MatlabExpr) -> MatlabExpr {
        MatlabExpr::Call(Box::new(MatlabExpr::Var("svd".to_string())), vec![a])
    }
    /// `linspace(a, b, n)`.
    #[allow(dead_code)]
    pub fn linspace(a: MatlabExpr, b: MatlabExpr, n: MatlabExpr) -> MatlabExpr {
        MatlabExpr::Call(
            Box::new(MatlabExpr::Var("linspace".to_string())),
            vec![a, b, n],
        )
    }
    /// `mod(a, m)`.
    #[allow(dead_code)]
    pub fn matlab_mod(a: MatlabExpr, m: MatlabExpr) -> MatlabExpr {
        MatlabExpr::Call(Box::new(MatlabExpr::Var("mod".to_string())), vec![a, m])
    }
    /// `floor(x)`.
    #[allow(dead_code)]
    pub fn floor(x: MatlabExpr) -> MatlabExpr {
        MatlabExpr::Call(Box::new(MatlabExpr::Var("floor".to_string())), vec![x])
    }
    /// `ceil(x)`.
    #[allow(dead_code)]
    pub fn ceil(x: MatlabExpr) -> MatlabExpr {
        MatlabExpr::Call(Box::new(MatlabExpr::Var("ceil".to_string())), vec![x])
    }
    /// `round(x)`.
    #[allow(dead_code)]
    pub fn round(x: MatlabExpr) -> MatlabExpr {
        MatlabExpr::Call(Box::new(MatlabExpr::Var("round".to_string())), vec![x])
    }
    /// `fix(x)` — truncate toward zero.
    #[allow(dead_code)]
    pub fn fix(x: MatlabExpr) -> MatlabExpr {
        MatlabExpr::Call(Box::new(MatlabExpr::Var("fix".to_string())), vec![x])
    }
    /// `rem(a, m)` — remainder (sign matches dividend).
    #[allow(dead_code)]
    pub fn rem(a: MatlabExpr, m: MatlabExpr) -> MatlabExpr {
        MatlabExpr::Call(Box::new(MatlabExpr::Var("rem".to_string())), vec![a, m])
    }
}
impl MatlabFunction {
    pub fn new(
        name: &str,
        inputs: Vec<MatlabParam>,
        outputs: Vec<String>,
        body: Vec<MatlabStmt>,
    ) -> Self {
        MatlabFunction {
            name: name.to_string(),
            inputs,
            outputs,
            body,
            is_nested: false,
            is_local: false,
            help_text: None,
            argument_validation: Vec::new(),
        }
    }
    pub fn nested(mut self) -> Self {
        self.is_nested = true;
        self
    }
    pub fn local(mut self) -> Self {
        self.is_local = true;
        self
    }
    pub fn with_help(mut self, help: &str) -> Self {
        self.help_text = Some(help.to_string());
        self
    }
}
impl MatlabModuleBuilder {
    /// Create a new module builder.
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        MatlabModuleBuilder {
            name: name.into(),
            functions: Vec::new(),
            classes: Vec::new(),
            scripts: Vec::new(),
            globals: Vec::new(),
            config: MatlabGenConfig::default(),
        }
    }
    /// Add a function.
    #[allow(dead_code)]
    pub fn add_function(mut self, func: MatlabFunction) -> Self {
        self.functions.push(func);
        self
    }
    /// Add a class.
    #[allow(dead_code)]
    pub fn add_class(mut self, cls: MatlabClassdef) -> Self {
        self.classes.push(cls);
        self
    }
    /// Add a script.
    #[allow(dead_code)]
    pub fn add_script(mut self, script: MatlabScript) -> Self {
        self.scripts.push(script);
        self
    }
    /// Declare a global variable.
    #[allow(dead_code)]
    pub fn declare_global(mut self, name: impl Into<String>) -> Self {
        self.globals.push(name.into());
        self
    }
    /// Emit the entire module.
    #[allow(dead_code)]
    pub fn emit(&self) -> String {
        let mut backend = MatlabBackend::new();
        if !self.globals.is_empty() {
            let globals = self.globals.join(" ");
            backend.emit_stmt(&MatlabStmt::Comment(format!("globals: {}", globals)));
        }
        for func in &self.functions {
            backend.emit_function(func);
        }
        for cls in &self.classes {
            backend.emit_classdef(cls);
        }
        backend.take_output()
    }
    /// Number of items in the module.
    #[allow(dead_code)]
    pub fn total_items(&self) -> usize {
        self.functions.len() + self.classes.len() + self.scripts.len()
    }
}
