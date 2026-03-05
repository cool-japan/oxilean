//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::Name;
use std::collections::{HashMap, HashSet};

/// Metadata about an LCNF module.
#[derive(Clone, Debug, Default)]
pub struct LcnfModuleMetadata {
    /// Number of declarations converted.
    pub decl_count: usize,
    /// Number of lambdas lifted.
    pub lambdas_lifted: usize,
    /// Number of proofs erased.
    pub proofs_erased: usize,
    /// Number of types erased.
    pub types_erased: usize,
    /// Total LCNF let bindings generated.
    pub let_bindings: usize,
}
/// An argument to a function call or constructor in LCNF.
///
/// In ANF, arguments must be "atomic" — either variables or literals.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum LcnfArg {
    /// A variable reference.
    Var(LcnfVarId),
    /// A literal value.
    Lit(LcnfLit),
    /// An erased argument (placeholder for proof terms).
    Erased,
    /// A type argument (may be erased depending on config).
    Type(LcnfType),
}
/// An LCNF module — a collection of declarations.
#[derive(Clone, Debug, Default)]
pub struct LcnfModule {
    /// Top-level function declarations.
    pub fun_decls: Vec<LcnfFunDecl>,
    /// External declarations (axioms, opaques).
    pub extern_decls: Vec<LcnfExternDecl>,
    /// Name of the module.
    pub name: String,
    /// Metadata about the conversion.
    pub metadata: LcnfModuleMetadata,
}
/// Configuration for pretty-printing LCNF.
#[derive(Clone, Debug)]
pub struct PrettyConfig {
    pub indent: usize,
    pub max_width: usize,
    pub show_types: bool,
    pub show_erased: bool,
}
/// A unique variable identifier in LCNF.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct LcnfVarId(pub u64);
/// Collector that finds all free variables in an LCNF expression.
pub struct FreeVarCollector {
    pub(super) bound: HashSet<LcnfVarId>,
    pub(super) free: HashSet<LcnfVarId>,
}
impl FreeVarCollector {
    pub(super) fn new() -> Self {
        FreeVarCollector {
            bound: HashSet::new(),
            free: HashSet::new(),
        }
    }
    pub(super) fn collect_from_arg(&mut self, arg: &LcnfArg) {
        if let LcnfArg::Var(id) = arg {
            if !self.bound.contains(id) {
                self.free.insert(*id);
            }
        }
    }
    pub(super) fn collect_from_let_value(&mut self, val: &LcnfLetValue) {
        match val {
            LcnfLetValue::App(func, args) => {
                self.collect_from_arg(func);
                for arg in args {
                    self.collect_from_arg(arg);
                }
            }
            LcnfLetValue::Proj(_, _, var) => {
                if !self.bound.contains(var) {
                    self.free.insert(*var);
                }
            }
            LcnfLetValue::Ctor(_, _, args) => {
                for arg in args {
                    self.collect_from_arg(arg);
                }
            }
            LcnfLetValue::FVar(id) => {
                if !self.bound.contains(id) {
                    self.free.insert(*id);
                }
            }
            LcnfLetValue::Lit(_)
            | LcnfLetValue::Erased
            | LcnfLetValue::Reset(_)
            | LcnfLetValue::Reuse(_, _, _, _) => {}
        }
    }
    pub(super) fn collect_expr(&mut self, expr: &LcnfExpr) {
        match expr {
            LcnfExpr::Let {
                id, value, body, ..
            } => {
                self.collect_from_let_value(value);
                self.bound.insert(*id);
                self.collect_expr(body);
            }
            LcnfExpr::Case {
                scrutinee,
                alts,
                default,
                ..
            } => {
                if !self.bound.contains(scrutinee) {
                    self.free.insert(*scrutinee);
                }
                for alt in alts {
                    let saved = self.bound.clone();
                    for param in &alt.params {
                        self.bound.insert(param.id);
                    }
                    self.collect_expr(&alt.body);
                    self.bound = saved;
                }
                if let Some(def) = default {
                    self.collect_expr(def);
                }
            }
            LcnfExpr::Return(arg) => self.collect_from_arg(arg),
            LcnfExpr::Unreachable => {}
            LcnfExpr::TailCall(func, args) => {
                self.collect_from_arg(func);
                for arg in args {
                    self.collect_from_arg(arg);
                }
            }
        }
    }
}
/// LCNF type representation.
///
/// Types in LCNF are simplified compared to kernel types.
/// Proof-irrelevant types and universes are erased.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum LcnfType {
    /// Erased type (proof-irrelevant or computationally irrelevant).
    Erased,
    /// A type variable or named type.
    Var(String),
    /// Function type: params -> return.
    Fun(Vec<LcnfType>, Box<LcnfType>),
    /// Constructor/inductive type with type arguments.
    Ctor(String, Vec<LcnfType>),
    /// Object type (boxed/heap-allocated).
    Object,
    /// Natural number type.
    Nat,
    /// String type.
    LcnfString,
    /// Unit type (erased value placeholder).
    Unit,
    /// Irrelevant (computationally meaningless, e.g. proofs).
    Irrelevant,
}
/// Counts how many times each variable is used (referenced).
pub struct UsageCounter {
    pub(super) counts: HashMap<LcnfVarId, usize>,
}
impl UsageCounter {
    pub(super) fn new() -> Self {
        UsageCounter {
            counts: HashMap::new(),
        }
    }
    pub(super) fn count_arg(&mut self, arg: &LcnfArg) {
        if let LcnfArg::Var(id) = arg {
            *self.counts.entry(*id).or_insert(0) += 1;
        }
    }
    pub(super) fn count_let_value(&mut self, val: &LcnfLetValue) {
        match val {
            LcnfLetValue::App(func, args) => {
                self.count_arg(func);
                for arg in args {
                    self.count_arg(arg);
                }
            }
            LcnfLetValue::Proj(_, _, var) => {
                *self.counts.entry(*var).or_insert(0) += 1;
            }
            LcnfLetValue::Ctor(_, _, args) => {
                for arg in args {
                    self.count_arg(arg);
                }
            }
            LcnfLetValue::FVar(id) => {
                *self.counts.entry(*id).or_insert(0) += 1;
            }
            LcnfLetValue::Lit(_)
            | LcnfLetValue::Erased
            | LcnfLetValue::Reset(_)
            | LcnfLetValue::Reuse(_, _, _, _) => {}
        }
    }
    pub(super) fn count_expr(&mut self, expr: &LcnfExpr) {
        match expr {
            LcnfExpr::Let { value, body, .. } => {
                self.count_let_value(value);
                self.count_expr(body);
            }
            LcnfExpr::Case {
                scrutinee,
                alts,
                default,
                ..
            } => {
                *self.counts.entry(*scrutinee).or_insert(0) += 1;
                for alt in alts {
                    self.count_expr(&alt.body);
                }
                if let Some(def) = default {
                    self.count_expr(def);
                }
            }
            LcnfExpr::Return(arg) => self.count_arg(arg),
            LcnfExpr::Unreachable => {}
            LcnfExpr::TailCall(func, args) => {
                self.count_arg(func);
                for arg in args {
                    self.count_arg(arg);
                }
            }
        }
    }
}
/// A let-bound value in LCNF.
///
/// In ANF, every complex operation is let-bound so that
/// arguments to subsequent operations are always atomic.
#[derive(Clone, PartialEq, Debug)]
pub enum LcnfLetValue {
    /// Function application: `f(args...)`.
    App(LcnfArg, Vec<LcnfArg>),
    /// Projection: `proj_idx(struct_var)`.
    Proj(String, u32, LcnfVarId),
    /// Constructor application: `Ctor(args...)`.
    Ctor(String, u32, Vec<LcnfArg>),
    /// Literal value.
    Lit(LcnfLit),
    /// Erased value.
    Erased,
    /// A free variable reference (unresolved).
    FVar(LcnfVarId),
    /// Reset (free fields of) a unique object, returning a reusable memory slot.
    /// Used by the reset-reuse optimization to recycle allocations.
    Reset(LcnfVarId),
    /// Reuse a freed slot to construct a new value.
    /// `Reuse(slot, ctor_name, ctor_tag, args)` — like `Ctor` but using pre-allocated memory.
    Reuse(LcnfVarId, String, u32, Vec<LcnfArg>),
}
/// A mapping from variable IDs to replacement arguments.
#[derive(Clone, Debug, Default)]
pub struct Substitution(pub HashMap<LcnfVarId, LcnfArg>);
impl Substitution {
    pub fn new() -> Self {
        Substitution(HashMap::new())
    }
    pub fn insert(&mut self, var: LcnfVarId, arg: LcnfArg) {
        self.0.insert(var, arg);
    }
    pub fn get(&self, var: &LcnfVarId) -> Option<&LcnfArg> {
        self.0.get(var)
    }
    pub fn contains(&self, var: &LcnfVarId) -> bool {
        self.0.contains_key(var)
    }
    pub fn compose(&self, other: &Substitution) -> Substitution {
        let mut result = HashMap::new();
        for (var, arg) in &self.0 {
            result.insert(*var, substitute_arg(arg, other));
        }
        for (var, arg) in &other.0 {
            result.entry(*var).or_insert_with(|| arg.clone());
        }
        Substitution(result)
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
/// A parameter declaration in LCNF.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct LcnfParam {
    /// The variable ID for this parameter.
    pub id: LcnfVarId,
    /// The name hint for this parameter.
    pub name: String,
    /// The type of this parameter.
    pub ty: LcnfType,
    /// Whether this parameter is erased (proof-irrelevant).
    pub erased: bool,
    /// Whether this parameter is borrowed (no RC inc/dec needed).
    pub borrowed: bool,
}
/// Records the site where a variable is defined.
#[derive(Clone, Debug, PartialEq)]
pub struct DefinitionSite {
    pub var: LcnfVarId,
    pub name: String,
    pub ty: LcnfType,
    pub depth: usize,
}
/// A case alternative (branch) in LCNF.
#[derive(Clone, PartialEq, Debug)]
pub struct LcnfAlt {
    /// The constructor name for this alternative.
    pub ctor_name: String,
    /// The constructor tag (index in the inductive type).
    pub ctor_tag: u32,
    /// Parameters bound by the constructor.
    pub params: Vec<LcnfParam>,
    /// The body of this alternative.
    pub body: LcnfExpr,
}
/// An external (axiom/opaque) declaration.
#[derive(Clone, PartialEq, Debug)]
pub struct LcnfExternDecl {
    /// Name of the external declaration.
    pub name: String,
    /// Parameters.
    pub params: Vec<LcnfParam>,
    /// Return type.
    pub ret_type: LcnfType,
}
/// A top-level function declaration in LCNF.
#[derive(Clone, PartialEq, Debug)]
pub struct LcnfFunDecl {
    /// The fully qualified name of this function.
    pub name: String,
    /// The original kernel name.
    pub original_name: Option<Name>,
    /// Parameters of this function.
    pub params: Vec<LcnfParam>,
    /// Return type.
    pub ret_type: LcnfType,
    /// The function body in LCNF.
    pub body: LcnfExpr,
    /// Whether this function is recursive.
    pub is_recursive: bool,
    /// Whether this function was lifted from a nested lambda.
    pub is_lifted: bool,
    /// Inlining cost heuristic (lower = more likely to inline).
    pub inline_cost: usize,
}
/// Errors that can occur when validating LCNF.
#[derive(Clone, Debug, PartialEq)]
pub enum ValidationError {
    UnboundVariable(LcnfVarId),
    DuplicateBinding(LcnfVarId),
    EmptyCase,
    InvalidTag(String, u32),
    NonAtomicArgument,
}
/// A builder for constructing LCNF expressions incrementally.
pub struct LcnfBuilder {
    pub(super) next_id: u64,
    pub(super) bindings: Vec<(LcnfVarId, String, LcnfType, LcnfLetValue)>,
}
impl LcnfBuilder {
    pub fn new() -> Self {
        LcnfBuilder {
            next_id: 0,
            bindings: Vec::new(),
        }
    }
    pub fn with_start_id(start: u64) -> Self {
        LcnfBuilder {
            next_id: start,
            bindings: Vec::new(),
        }
    }
    pub fn fresh_var(&mut self, _name: &str, _ty: LcnfType) -> LcnfVarId {
        let id = LcnfVarId(self.next_id);
        self.next_id += 1;
        id
    }
    pub fn let_bind(&mut self, name: &str, ty: LcnfType, val: LcnfLetValue) -> LcnfVarId {
        let id = LcnfVarId(self.next_id);
        self.next_id += 1;
        self.bindings.push((id, name.to_string(), ty, val));
        id
    }
    pub fn let_app(
        &mut self,
        name: &str,
        ty: LcnfType,
        func: LcnfArg,
        args: Vec<LcnfArg>,
    ) -> LcnfVarId {
        self.let_bind(name, ty, LcnfLetValue::App(func, args))
    }
    pub fn let_ctor(
        &mut self,
        name: &str,
        ty: LcnfType,
        ctor: &str,
        tag: u32,
        args: Vec<LcnfArg>,
    ) -> LcnfVarId {
        self.let_bind(name, ty, LcnfLetValue::Ctor(ctor.to_string(), tag, args))
    }
    pub fn let_proj(
        &mut self,
        name: &str,
        ty: LcnfType,
        type_name: &str,
        idx: u32,
        var: LcnfVarId,
    ) -> LcnfVarId {
        self.let_bind(
            name,
            ty,
            LcnfLetValue::Proj(type_name.to_string(), idx, var),
        )
    }
    pub fn build_return(self, arg: LcnfArg) -> LcnfExpr {
        self.wrap_bindings(LcnfExpr::Return(arg))
    }
    pub fn build_case(
        self,
        scrutinee: LcnfVarId,
        scrutinee_ty: LcnfType,
        alts: Vec<LcnfAlt>,
        default: Option<LcnfExpr>,
    ) -> LcnfExpr {
        self.wrap_bindings(LcnfExpr::Case {
            scrutinee,
            scrutinee_ty,
            alts,
            default: default.map(Box::new),
        })
    }
    pub fn build_tail_call(self, func: LcnfArg, args: Vec<LcnfArg>) -> LcnfExpr {
        self.wrap_bindings(LcnfExpr::TailCall(func, args))
    }
    pub(super) fn wrap_bindings(self, terminal: LcnfExpr) -> LcnfExpr {
        let mut result = terminal;
        for (id, name, ty, value) in self.bindings.into_iter().rev() {
            result = LcnfExpr::Let {
                id,
                name,
                ty,
                value,
                body: Box::new(result),
            };
        }
        result
    }
    pub fn peek_next_id(&self) -> u64 {
        self.next_id
    }
    pub fn binding_count(&self) -> usize {
        self.bindings.len()
    }
}
/// A cost model for estimating runtime cost.
#[derive(Clone, Debug)]
pub struct CostModel {
    pub let_cost: u64,
    pub app_cost: u64,
    pub case_cost: u64,
    pub return_cost: u64,
    pub branch_penalty: u64,
}
/// Literal values in LCNF.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum LcnfLit {
    /// Natural number literal.
    Nat(u64),
    /// String literal.
    Str(String),
}
/// Core LCNF expression in administrative normal form.
#[derive(Clone, PartialEq, Debug)]
pub enum LcnfExpr {
    /// Let binding: `let x : ty := val; body`.
    Let {
        /// The variable being bound.
        id: LcnfVarId,
        /// Name hint.
        name: String,
        /// Type of the binding.
        ty: LcnfType,
        /// The value being bound.
        value: LcnfLetValue,
        /// The continuation expression.
        body: Box<LcnfExpr>,
    },
    /// Case split (pattern match).
    Case {
        /// The scrutinee variable.
        scrutinee: LcnfVarId,
        /// The type of the scrutinee.
        scrutinee_ty: LcnfType,
        /// The alternatives.
        alts: Vec<LcnfAlt>,
        /// Default alternative (if not all constructors covered).
        default: Option<Box<LcnfExpr>>,
    },
    /// Return a value (terminal expression).
    Return(LcnfArg),
    /// Unreachable code (after exhaustive match).
    Unreachable,
    /// Function application in tail position.
    TailCall(LcnfArg, Vec<LcnfArg>),
}
