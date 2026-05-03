use super::super::functions::FORTRAN_KEYWORDS;
use super::super::functions::*;
use crate::lcnf::*;
use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};

use super::defs::*;

impl FortranBackend {
    pub fn new() -> Self {
        let mut reserved = HashSet::new();
        for kw in FORTRAN_KEYWORDS {
            reserved.insert(kw.to_lowercase());
        }
        FortranBackend {
            var_counter: 0,
            name_cache: HashMap::new(),
            reserved,
            indent_width: 2,
            max_line_len: 120,
        }
    }
    pub fn fresh_var(&mut self) -> String {
        let id = self.var_counter;
        self.var_counter += 1;
        format!("ftv{}", id)
    }
    pub fn mangle_name(&mut self, name: &str) -> String {
        if let Some(cached) = self.name_cache.get(name) {
            return cached.clone();
        }
        let mangled = mangle_fortran_ident(name, &self.reserved);
        self.name_cache.insert(name.to_string(), mangled.clone());
        mangled
    }
    /// Emit a complete Fortran module as a String.
    pub fn emit_module(&mut self, module: &FortranModule) -> String {
        let mut out = String::new();
        if let Some(doc) = &module.doc {
            for line in doc.lines() {
                out.push_str(&format!("! {}\n", line));
            }
        }
        out.push_str(&format!("MODULE {}\n", module.name.to_uppercase()));
        for used in &module.use_modules {
            out.push_str(&format!("  USE {}\n", used.to_uppercase()));
        }
        if module.implicit_none {
            out.push_str("  IMPLICIT NONE\n");
        }
        for dt in &module.derived_types {
            out.push_str(&self.emit_derived_type(dt, 1));
        }
        for decl in &module.module_vars {
            out.push_str(&self.emit_decl(decl, 1));
        }
        if !module.contains.is_empty() {
            out.push_str("CONTAINS\n");
            for sub in &module.contains {
                out.push_str(&self.emit_subprogram(sub, 0));
            }
        }
        out.push_str(&format!("END MODULE {}\n", module.name.to_uppercase()));
        out
    }
    /// Emit a standalone PROGRAM.
    pub fn emit_program(&mut self, prog: &FortranProgram) -> String {
        let mut out = String::new();
        out.push_str(&format!("PROGRAM {}\n", prog.name.to_uppercase()));
        for used in &prog.use_modules {
            out.push_str(&format!("  USE {}\n", used.to_uppercase()));
        }
        if prog.implicit_none {
            out.push_str("  IMPLICIT NONE\n");
        }
        for decl in &prog.decls {
            out.push_str(&self.emit_decl(decl, 1));
        }
        out.push_str("  ! --- executable section ---\n");
        for stmt in &prog.body {
            out.push_str(&self.emit_stmt(stmt, 1));
        }
        if !prog.contains.is_empty() {
            out.push_str("CONTAINS\n");
            for sub in &prog.contains {
                out.push_str(&self.emit_subprogram(sub, 0));
            }
        }
        out.push_str(&format!("END PROGRAM {}\n", prog.name.to_uppercase()));
        out
    }
    /// Emit a FUNCTION or SUBROUTINE.
    pub fn emit_subprogram(&mut self, sub: &FortranSubprogram, depth: usize) -> String {
        let indent = self.indent(depth);
        let mut out = String::new();
        if let Some(doc) = &sub.doc {
            for line in doc.lines() {
                out.push_str(&format!("{}! {}\n", indent, line));
            }
        }
        let mut prefix = String::new();
        if sub.is_pure {
            prefix.push_str("PURE ");
        }
        if sub.is_elemental {
            prefix.push_str("ELEMENTAL ");
        }
        if sub.is_recursive {
            prefix.push_str("RECURSIVE ");
        }
        let dummy_list = sub.dummy_args.join(", ");
        if sub.is_function() {
            let ret_ty = &sub.return_type;
            out.push_str(&format!(
                "{}{}{} FUNCTION {}({})\n",
                indent,
                prefix,
                ret_ty,
                sub.name.to_uppercase(),
                dummy_list
            ));
        } else {
            out.push_str(&format!(
                "{}{}SUBROUTINE {}({})\n",
                indent,
                prefix,
                sub.name.to_uppercase(),
                dummy_list
            ));
        }
        let inner = self.indent(depth + 1);
        out.push_str(&format!("{}IMPLICIT NONE\n", inner));
        if sub.is_function() {
            out.push_str(&format!(
                "{}{} :: {}\n",
                inner,
                sub.return_type,
                sub.name.to_uppercase()
            ));
        }
        for decl in &sub.decls {
            out.push_str(&self.emit_decl(decl, depth + 1));
        }
        if !sub.decls.is_empty() {
            out.push_str(&format!("{}! --- executable section ---\n", inner));
        }
        for stmt in &sub.body {
            out.push_str(&self.emit_stmt(stmt, depth + 1));
        }
        if sub.is_function() {
            out.push_str(&format!(
                "{}END FUNCTION {}\n",
                indent,
                sub.name.to_uppercase()
            ));
        } else {
            out.push_str(&format!(
                "{}END SUBROUTINE {}\n",
                indent,
                sub.name.to_uppercase()
            ));
        }
        out
    }
    /// Emit a derived-type definition.
    pub fn emit_derived_type(&self, dt: &FortranDerivedType, depth: usize) -> String {
        let indent = self.indent(depth);
        let mut out = String::new();
        if let Some(doc) = &dt.doc {
            out.push_str(&format!("{}! {}\n", indent, doc));
        }
        out.push_str(&format!("{}TYPE :: {}\n", indent, dt.name.to_uppercase()));
        for field in &dt.fields {
            out.push_str(&self.emit_decl(field, depth + 1));
        }
        out.push_str(&format!("{}END TYPE {}\n", indent, dt.name.to_uppercase()));
        out
    }
    /// Emit a variable declaration.
    pub fn emit_decl(&self, decl: &FortranDecl, depth: usize) -> String {
        let indent = self.indent(depth);
        let mut attrs: Vec<String> = Vec::new();
        if let Some(intent) = &decl.intent {
            attrs.push(format!("INTENT({})", intent));
        }
        if decl.is_parameter {
            attrs.push("PARAMETER".to_string());
        }
        let attr_str = if attrs.is_empty() {
            String::new()
        } else {
            format!(", {}", attrs.join(", "))
        };
        if let Some(init) = &decl.initial_value {
            format!(
                "{}{}{} :: {} = {}\n",
                indent,
                decl.ty,
                attr_str,
                decl.name.to_uppercase(),
                init
            )
        } else {
            format!(
                "{}{}{} :: {}\n",
                indent,
                decl.ty,
                attr_str,
                decl.name.to_uppercase()
            )
        }
    }
    /// Emit a Fortran statement with proper indentation.
    pub fn emit_stmt(&self, stmt: &FortranStmt, depth: usize) -> String {
        let indent = self.indent(depth);
        match stmt {
            FortranStmt::Assign(lhs, rhs) => format!("{}{} = {}\n", indent, lhs, rhs),
            FortranStmt::Call(name, args) => {
                let arg_list: Vec<String> = args.iter().map(|a| format!("{}", a)).collect();
                format!(
                    "{}CALL {}({})\n",
                    indent,
                    name.to_uppercase(),
                    arg_list.join(", ")
                )
            }
            FortranStmt::Return => format!("{}RETURN\n", indent),
            FortranStmt::If(branches, else_body) => {
                let mut out = String::new();
                for (idx, (cond, body)) in branches.iter().enumerate() {
                    if idx == 0 {
                        out.push_str(&format!("{}IF ({}) THEN\n", indent, cond));
                    } else {
                        out.push_str(&format!("{}ELSE IF ({}) THEN\n", indent, cond));
                    }
                    for s in body {
                        out.push_str(&self.emit_stmt(s, depth + 1));
                    }
                }
                if !else_body.is_empty() {
                    out.push_str(&format!("{}ELSE\n", indent));
                    for s in else_body {
                        out.push_str(&self.emit_stmt(s, depth + 1));
                    }
                }
                out.push_str(&format!("{}END IF\n", indent));
                out
            }
            FortranStmt::SelectCase(expr, cases, default) => {
                let mut out = format!("{}SELECT CASE ({})\n", indent, expr);
                for case in cases {
                    if let Some(vals) = &case.values {
                        let val_str: Vec<String> = vals.iter().map(|v| format!("{}", v)).collect();
                        out.push_str(&format!("{}CASE ({})\n", indent, val_str.join(", ")));
                    } else {
                        out.push_str(&format!("{}CASE DEFAULT\n", indent));
                    }
                    for s in &case.body {
                        out.push_str(&self.emit_stmt(s, depth + 1));
                    }
                }
                if !default.is_empty() {
                    out.push_str(&format!("{}CASE DEFAULT\n", indent));
                    for s in default {
                        out.push_str(&self.emit_stmt(s, depth + 1));
                    }
                }
                out.push_str(&format!("{}END SELECT\n", indent));
                out
            }
            FortranStmt::Do(label, body) => {
                let label_str = label
                    .as_deref()
                    .map(|l| format!("{}: ", l))
                    .unwrap_or_default();
                let mut out = format!("{}{}DO\n", indent, label_str);
                for s in body {
                    out.push_str(&self.emit_stmt(s, depth + 1));
                }
                out.push_str(&format!("{}END DO\n", indent));
                out
            }
            FortranStmt::DoCount(var, lo, hi, step, body) => {
                let step_str = step
                    .as_ref()
                    .map(|s| format!(", {}", s))
                    .unwrap_or_default();
                let mut out = format!(
                    "{}DO {} = {}, {}{}\n",
                    indent,
                    var.to_uppercase(),
                    lo,
                    hi,
                    step_str
                );
                for s in body {
                    out.push_str(&self.emit_stmt(s, depth + 1));
                }
                out.push_str(&format!("{}END DO\n", indent));
                out
            }
            FortranStmt::DoWhile(cond, body) => {
                let mut out = format!("{}DO WHILE ({})\n", indent, cond);
                for s in body {
                    out.push_str(&self.emit_stmt(s, depth + 1));
                }
                out.push_str(&format!("{}END DO\n", indent));
                out
            }
            FortranStmt::Exit(label) => match label {
                Some(l) => format!("{}EXIT {}\n", indent, l),
                None => format!("{}EXIT\n", indent),
            },
            FortranStmt::Cycle(label) => match label {
                Some(l) => format!("{}CYCLE {}\n", indent, l),
                None => format!("{}CYCLE\n", indent),
            },
            FortranStmt::Stop(code) => match code {
                Some(c) => format!("{}STOP {}\n", indent, c),
                None => format!("{}STOP\n", indent),
            },
            FortranStmt::Allocate(var, stat) => {
                if let Some(stat_var) = stat {
                    format!("{}ALLOCATE({}, STAT={})\n", indent, var, stat_var)
                } else {
                    format!("{}ALLOCATE({})\n", indent, var)
                }
            }
            FortranStmt::Deallocate(var, stat) => {
                if let Some(stat_var) = stat {
                    format!("{}DEALLOCATE({}, STAT={})\n", indent, var, stat_var)
                } else {
                    format!("{}DEALLOCATE({})\n", indent, var)
                }
            }
            FortranStmt::Nullify(ptr) => format!("{}NULLIFY({})\n", indent, ptr),
            FortranStmt::Print(exprs) => {
                if exprs.is_empty() {
                    format!("{}PRINT *\n", indent)
                } else {
                    let args: Vec<String> = exprs.iter().map(|e| format!("{}", e)).collect();
                    format!("{}PRINT *, {}\n", indent, args.join(", "))
                }
            }
            FortranStmt::Write(unit, fmt, exprs) => {
                let args: Vec<String> = exprs.iter().map(|e| format!("{}", e)).collect();
                let data = if args.is_empty() {
                    String::new()
                } else {
                    format!(" {}", args.join(", "))
                };
                format!("{}WRITE({}, {}){}\n", indent, unit, fmt, data)
            }
            FortranStmt::Read(unit, fmt, vars) => {
                let args: Vec<String> = vars.iter().map(|v| format!("{}", v)).collect();
                format!("{}READ({}, {}) {}\n", indent, unit, fmt, args.join(", "))
            }
            FortranStmt::Open(unit, file, status) => {
                format!(
                    "{}OPEN(UNIT={}, FILE='{}', STATUS='{}')\n",
                    indent, unit, file, status
                )
            }
            FortranStmt::Close(unit) => format!("{}CLOSE(UNIT={})\n", indent, unit),
            FortranStmt::Continue => format!("{}CONTINUE\n", indent),
            FortranStmt::Raw(code) => format!("{}{}\n", indent, code),
            FortranStmt::Block(stmts) => {
                let mut out = String::new();
                for s in stmts {
                    out.push_str(&self.emit_stmt(s, depth));
                }
                out
            }
        }
    }
    pub(crate) fn indent(&self, depth: usize) -> String {
        " ".repeat(depth * self.indent_width)
    }
    /// Compile an LCNF function to a `FortranSubprogram`.
    pub fn compile_lcnf_function(
        &mut self,
        func: &LcnfFunDecl,
    ) -> Result<FortranSubprogram, String> {
        let name = self.mangle_name(&func.name.to_string());
        let ret_ty = lcnf_type_to_fortran(&func.ret_type);
        let mut dummy_args: Vec<String> = Vec::new();
        let mut decls: Vec<FortranDecl> = Vec::new();
        for param in &func.params {
            let pname = format!("px{}", param.id.0);
            let pty = lcnf_type_to_fortran(&param.ty);
            dummy_args.push(pname.clone());
            decls.push(FortranDecl::param_in(pty, &pname));
        }
        let mut body_stmts = Vec::new();
        let result_expr = self.compile_expr(&func.body, &mut body_stmts, &mut decls)?;
        body_stmts.push(FortranStmt::Assign(
            FortranExpr::Var(name.to_uppercase()),
            result_expr,
        ));
        body_stmts.push(FortranStmt::Return);
        let mut sub = FortranSubprogram::function(name.clone(), ret_ty);
        sub.dummy_args = dummy_args;
        sub.decls = decls;
        sub.body = body_stmts;
        Ok(sub)
    }
    /// Compile an LCNF module to a `FortranModule`.
    pub fn compile_lcnf_module(&mut self, module: &LcnfModule) -> Result<FortranModule, String> {
        let mut fort_module = FortranModule::new("oxilean_generated");
        fort_module.use_modules.push("iso_fortran_env".to_string());
        let ctor_names = collect_ctor_names_module(module);
        for ctor_name in &ctor_names {
            fort_module
                .derived_types
                .push(make_ctor_derived_type(ctor_name));
        }
        for func in &module.fun_decls {
            let sub = self.compile_lcnf_function(func)?;
            fort_module.contains.push(sub);
        }
        Ok(fort_module)
    }
    pub(crate) fn compile_expr(
        &mut self,
        expr: &LcnfExpr,
        stmts: &mut Vec<FortranStmt>,
        decls: &mut Vec<FortranDecl>,
    ) -> Result<FortranExpr, String> {
        match expr {
            LcnfExpr::Return(arg) => Ok(self.compile_arg(arg)),
            LcnfExpr::Unreachable => {
                stmts.push(FortranStmt::Raw(
                    "STOP 'OxiLean: unreachable code reached'".to_string(),
                ));
                Ok(FortranExpr::Lit(FortranLit::Int(0)))
            }
            LcnfExpr::TailCall(func, args) => {
                let name = match func {
                    LcnfArg::Var(id) => format!("FX{}", id.0),
                    LcnfArg::Lit(_) => "UNKNOWN_FUNC".to_string(),
                    _ => "ERASED_FUNC".to_string(),
                };
                let fort_args: Vec<FortranExpr> =
                    args.iter().map(|a| self.compile_arg(a)).collect();
                Ok(FortranExpr::Call(name.to_uppercase(), fort_args))
            }
            LcnfExpr::Let {
                id,
                ty,
                value,
                body,
                ..
            } => {
                let var_name = format!("lv{}", id.0);
                let fort_ty = lcnf_type_to_fortran(ty);
                let val_expr = self.compile_let_value(value)?;
                decls.push(FortranDecl::local(fort_ty, &var_name));
                stmts.push(FortranStmt::Assign(
                    FortranExpr::Var(var_name.to_uppercase()),
                    val_expr,
                ));
                self.compile_expr(body, stmts, decls)
            }
            LcnfExpr::Case {
                scrutinee,
                alts,
                default,
                ..
            } => {
                let scrutinee_expr = FortranExpr::Var(format!("LV{}", scrutinee.0));
                let result_var = self.fresh_var();
                let result_decl = FortranDecl::local(FortranType::FtIntegerK(8), &result_var);
                decls.push(result_decl);
                let tag_expr = FortranExpr::Component(Box::new(scrutinee_expr), "tag".to_string());
                let mut cases: Vec<FortranCase> = Vec::new();
                for alt in alts {
                    let mut branch_stmts: Vec<FortranStmt> = Vec::new();
                    for (idx, param) in alt.params.iter().enumerate() {
                        let pname = format!("lv{}", param.id.0);
                        let pty = lcnf_type_to_fortran(&param.ty);
                        decls.push(FortranDecl::local(pty, &pname));
                        let field_expr = FortranExpr::Component(
                            Box::new(FortranExpr::Var(format!("SCRUTINEE_F{}", idx))),
                            format!("field{}", idx),
                        );
                        branch_stmts.push(FortranStmt::Assign(
                            FortranExpr::Var(pname.to_uppercase()),
                            field_expr,
                        ));
                    }
                    let branch_result = self.compile_expr(&alt.body, &mut branch_stmts, decls)?;
                    branch_stmts.push(FortranStmt::Assign(
                        FortranExpr::Var(result_var.to_uppercase()),
                        branch_result,
                    ));
                    cases.push(FortranCase {
                        values: Some(vec![FortranExpr::Lit(FortranLit::Int(alt.ctor_tag as i64))]),
                        body: branch_stmts,
                    });
                }
                let mut default_stmts: Vec<FortranStmt> = Vec::new();
                if let Some(def) = default {
                    let def_result = self.compile_expr(def, &mut default_stmts, decls)?;
                    default_stmts.push(FortranStmt::Assign(
                        FortranExpr::Var(result_var.to_uppercase()),
                        def_result,
                    ));
                } else {
                    default_stmts.push(FortranStmt::Raw(
                        "STOP 'OxiLean: unreachable branch'".to_string(),
                    ));
                }
                stmts.push(FortranStmt::SelectCase(tag_expr, cases, default_stmts));
                Ok(FortranExpr::Var(result_var.to_uppercase()))
            }
        }
    }
    pub(crate) fn compile_let_value(
        &mut self,
        value: &LcnfLetValue,
    ) -> Result<FortranExpr, String> {
        match value {
            LcnfLetValue::Lit(lit) => Ok(self.compile_lit(lit)),
            LcnfLetValue::Erased => Ok(FortranExpr::Lit(FortranLit::Logical(false))),
            LcnfLetValue::FVar(id) => Ok(FortranExpr::Var(format!("FX{}", id.0).to_uppercase())),
            LcnfLetValue::App(func, args) => {
                let name = match func {
                    LcnfArg::Var(id) => format!("FX{}", id.0),
                    _ => "UNKNOWN_FUNC".to_string(),
                };
                let fort_args: Vec<FortranExpr> =
                    args.iter().map(|a| self.compile_arg(a)).collect();
                Ok(FortranExpr::Call(name.to_uppercase(), fort_args))
            }
            LcnfLetValue::Proj(_name, idx, var) => {
                let base = FortranExpr::Var(format!("LV{}", var.0));
                Ok(FortranExpr::Component(
                    Box::new(base),
                    format!("field{}", idx),
                ))
            }
            LcnfLetValue::Ctor(name, tag, args) => {
                let ctor_name = self.mangle_name(name);
                let mut fields: Vec<(String, FortranExpr)> = Vec::new();
                fields.push((
                    "tag".to_string(),
                    FortranExpr::Lit(FortranLit::Int(*tag as i64)),
                ));
                for (idx, arg) in args.iter().enumerate() {
                    fields.push((format!("field{}", idx), self.compile_arg(arg)));
                }
                Ok(FortranExpr::TypeCtor(ctor_name.to_uppercase(), fields))
            }
            LcnfLetValue::Reset(_var) => Ok(FortranExpr::Lit(FortranLit::Logical(false))),
            LcnfLetValue::Reuse(_slot, name, tag, args) => {
                let ctor_name = self.mangle_name(name);
                let mut fields: Vec<(String, FortranExpr)> = Vec::new();
                fields.push((
                    "tag".to_string(),
                    FortranExpr::Lit(FortranLit::Int(*tag as i64)),
                ));
                for (idx, arg) in args.iter().enumerate() {
                    fields.push((format!("field{}", idx), self.compile_arg(arg)));
                }
                Ok(FortranExpr::TypeCtor(ctor_name.to_uppercase(), fields))
            }
        }
    }
    pub(crate) fn compile_arg(&self, arg: &LcnfArg) -> FortranExpr {
        match arg {
            LcnfArg::Var(id) => FortranExpr::Var(format!("LV{}", id.0)),
            LcnfArg::Lit(lit) => self.compile_lit(lit),
            LcnfArg::Erased => FortranExpr::Lit(FortranLit::Logical(false)),
            LcnfArg::Type(_) => FortranExpr::Lit(FortranLit::Logical(false)),
        }
    }
    pub(crate) fn compile_lit(&self, lit: &LcnfLit) -> FortranExpr {
        match lit {
            LcnfLit::Nat(n) => FortranExpr::Lit(FortranLit::Int(*n as i64)),
            LcnfLit::Str(s) => FortranExpr::Lit(FortranLit::Char(s.clone())),
        }
    }
}

impl FortranExtConstFolder {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            folds: 0,
            failures: 0,
            enabled: true,
        }
    }
    #[allow(dead_code)]
    pub fn add_i64(&mut self, a: i64, b: i64) -> Option<i64> {
        self.folds += 1;
        a.checked_add(b)
    }
    #[allow(dead_code)]
    pub fn sub_i64(&mut self, a: i64, b: i64) -> Option<i64> {
        self.folds += 1;
        a.checked_sub(b)
    }
    #[allow(dead_code)]
    pub fn mul_i64(&mut self, a: i64, b: i64) -> Option<i64> {
        self.folds += 1;
        a.checked_mul(b)
    }
    #[allow(dead_code)]
    pub fn div_i64(&mut self, a: i64, b: i64) -> Option<i64> {
        if b == 0 {
            self.failures += 1;
            None
        } else {
            self.folds += 1;
            a.checked_div(b)
        }
    }
    #[allow(dead_code)]
    pub fn rem_i64(&mut self, a: i64, b: i64) -> Option<i64> {
        if b == 0 {
            self.failures += 1;
            None
        } else {
            self.folds += 1;
            a.checked_rem(b)
        }
    }
    #[allow(dead_code)]
    pub fn neg_i64(&mut self, a: i64) -> Option<i64> {
        self.folds += 1;
        a.checked_neg()
    }
    #[allow(dead_code)]
    pub fn shl_i64(&mut self, a: i64, s: u32) -> Option<i64> {
        if s >= 64 {
            self.failures += 1;
            None
        } else {
            self.folds += 1;
            a.checked_shl(s)
        }
    }
    #[allow(dead_code)]
    pub fn shr_i64(&mut self, a: i64, s: u32) -> Option<i64> {
        if s >= 64 {
            self.failures += 1;
            None
        } else {
            self.folds += 1;
            a.checked_shr(s)
        }
    }
    #[allow(dead_code)]
    pub fn and_i64(&mut self, a: i64, b: i64) -> i64 {
        self.folds += 1;
        a & b
    }
    #[allow(dead_code)]
    pub fn or_i64(&mut self, a: i64, b: i64) -> i64 {
        self.folds += 1;
        a | b
    }
    #[allow(dead_code)]
    pub fn xor_i64(&mut self, a: i64, b: i64) -> i64 {
        self.folds += 1;
        a ^ b
    }
    #[allow(dead_code)]
    pub fn not_i64(&mut self, a: i64) -> i64 {
        self.folds += 1;
        !a
    }
    #[allow(dead_code)]
    pub fn fold_count(&self) -> usize {
        self.folds
    }
    #[allow(dead_code)]
    pub fn failure_count(&self) -> usize {
        self.failures
    }
    #[allow(dead_code)]
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    #[allow(dead_code)]
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    #[allow(dead_code)]
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

impl FortLivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        FortLivenessInfo {
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

impl FortranSubprogram {
    pub fn function(name: impl Into<String>, ret: FortranType) -> Self {
        FortranSubprogram {
            name: name.into(),
            return_type: ret,
            dummy_args: Vec::new(),
            decls: Vec::new(),
            body: Vec::new(),
            is_pure: false,
            is_elemental: false,
            is_recursive: false,
            doc: None,
        }
    }
    pub fn subroutine(name: impl Into<String>) -> Self {
        FortranSubprogram {
            name: name.into(),
            return_type: FortranType::FtVoid,
            dummy_args: Vec::new(),
            decls: Vec::new(),
            body: Vec::new(),
            is_pure: false,
            is_elemental: false,
            is_recursive: false,
            doc: None,
        }
    }
    pub fn is_function(&self) -> bool {
        self.return_type != FortranType::FtVoid
    }
}

impl FortranExtDepGraph {
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

impl FortranDecl {
    pub fn local(ty: FortranType, name: impl Into<String>) -> Self {
        FortranDecl {
            ty,
            name: name.into(),
            intent: None,
            is_parameter: false,
            initial_value: None,
            doc: None,
        }
    }
    pub fn param_in(ty: FortranType, name: impl Into<String>) -> Self {
        FortranDecl {
            ty,
            name: name.into(),
            intent: Some(FortranIntent::In),
            is_parameter: false,
            initial_value: None,
            doc: None,
        }
    }
    pub fn param_out(ty: FortranType, name: impl Into<String>) -> Self {
        FortranDecl {
            ty,
            name: name.into(),
            intent: Some(FortranIntent::Out),
            is_parameter: false,
            initial_value: None,
            doc: None,
        }
    }
}

impl FortranExtPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn register(&mut self, c: FortranExtPassConfig) {
        self.stats.push(FortranExtPassStats::new());
        self.configs.push(c);
    }
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.configs.len()
    }
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.configs.is_empty()
    }
    #[allow(dead_code)]
    pub fn get(&self, i: usize) -> Option<&FortranExtPassConfig> {
        self.configs.get(i)
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, i: usize) -> Option<&FortranExtPassStats> {
        self.stats.get(i)
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&FortranExtPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn passes_in_phase(&self, ph: &FortranExtPassPhase) -> Vec<&FortranExtPassConfig> {
        self.configs
            .iter()
            .filter(|c| c.enabled && &c.phase == ph)
            .collect()
    }
    #[allow(dead_code)]
    pub fn total_nodes_visited(&self) -> usize {
        self.stats.iter().map(|s| s.nodes_visited).sum()
    }
    #[allow(dead_code)]
    pub fn any_changed(&self) -> bool {
        self.stats.iter().any(|s| s.changed)
    }
}

impl FortranModule {
    pub fn new(name: impl Into<String>) -> Self {
        FortranModule {
            name: name.into(),
            use_modules: Vec::new(),
            implicit_none: true,
            module_vars: Vec::new(),
            derived_types: Vec::new(),
            contains: Vec::new(),
            doc: None,
        }
    }
}

impl FortranExtPassPhase {
    #[allow(dead_code)]
    pub fn is_early(&self) -> bool {
        matches!(self, Self::Early)
    }
    #[allow(dead_code)]
    pub fn is_middle(&self) -> bool {
        matches!(self, Self::Middle)
    }
    #[allow(dead_code)]
    pub fn is_late(&self) -> bool {
        matches!(self, Self::Late)
    }
    #[allow(dead_code)]
    pub fn is_finalize(&self) -> bool {
        matches!(self, Self::Finalize)
    }
    #[allow(dead_code)]
    pub fn order(&self) -> u32 {
        match self {
            Self::Early => 0,
            Self::Middle => 1,
            Self::Late => 2,
            Self::Finalize => 3,
        }
    }
    #[allow(dead_code)]
    pub fn from_order(n: u32) -> Option<Self> {
        match n {
            0 => Some(Self::Early),
            1 => Some(Self::Middle),
            2 => Some(Self::Late),
            3 => Some(Self::Finalize),
            _ => None,
        }
    }
}

impl FortranExtCache {
    #[allow(dead_code)]
    pub fn new(cap: usize) -> Self {
        Self {
            entries: Vec::new(),
            cap,
            total_hits: 0,
            total_misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: u64) -> Option<&[u8]> {
        for e in self.entries.iter_mut() {
            if e.0 == key && e.2 {
                e.3 += 1;
                self.total_hits += 1;
                return Some(&e.1);
            }
        }
        self.total_misses += 1;
        None
    }
    #[allow(dead_code)]
    pub fn put(&mut self, key: u64, data: Vec<u8>) {
        if self.entries.len() >= self.cap {
            self.entries.retain(|e| e.2);
            if self.entries.len() >= self.cap {
                self.entries.remove(0);
            }
        }
        self.entries.push((key, data, true, 0));
    }
    #[allow(dead_code)]
    pub fn invalidate(&mut self) {
        for e in self.entries.iter_mut() {
            e.2 = false;
        }
    }
    #[allow(dead_code)]
    pub fn hit_rate(&self) -> f64 {
        let t = self.total_hits + self.total_misses;
        if t == 0 {
            0.0
        } else {
            self.total_hits as f64 / t as f64
        }
    }
    #[allow(dead_code)]
    pub fn live_count(&self) -> usize {
        self.entries.iter().filter(|e| e.2).count()
    }
}

impl FortWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        FortWorklist {
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

impl FortranExtPassStats {
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
    pub fn merge(&mut self, o: &FortranExtPassStats) {
        self.iterations += o.iterations;
        self.changed |= o.changed;
        self.nodes_visited += o.nodes_visited;
        self.nodes_modified += o.nodes_modified;
        self.time_ms += o.time_ms;
        self.memory_bytes = self.memory_bytes.max(o.memory_bytes);
        self.errors += o.errors;
    }
}

impl FortDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        FortDominatorTree {
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
