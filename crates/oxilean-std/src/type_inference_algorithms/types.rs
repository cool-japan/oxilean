//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet};

use super::functions::*;

/// A gradual type: extends HMType with the unknown type `?`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GradualType {
    /// The unknown type `?` — consistent with any type.
    Unknown,
    /// A static type embedded in the gradual system.
    Static(HMType),
    /// A gradual arrow `τ₁ → τ₂`.
    Arrow(Box<GradualType>, Box<GradualType>),
}
impl GradualType {
    /// Two gradual types are *consistent* if they can be related at runtime.
    pub fn consistent(&self, other: &GradualType) -> bool {
        match (self, other) {
            (GradualType::Unknown, _) | (_, GradualType::Unknown) => true,
            (GradualType::Arrow(a1, b1), GradualType::Arrow(a2, b2)) => {
                a1.consistent(a2) && b1.consistent(b2)
            }
            (GradualType::Static(s), GradualType::Static(t)) => s == t,
            _ => false,
        }
    }
    /// Returns the *static approximation* (join with Unknown).
    pub fn precision(&self) -> usize {
        match self {
            GradualType::Unknown => 0,
            GradualType::Static(_) => 2,
            GradualType::Arrow(a, b) => 1 + a.precision() + b.precision(),
        }
    }
}
/// Generates typing constraints for an HM expression in a single pass.
///
/// Unlike `ConstraintInference`, this struct separates generation from solving,
/// allowing the constraints to be inspected before solving.
pub struct ConstraintGenerator {
    fresh: TyVar,
    /// All constraints collected during the generation pass.
    pub constraints: Vec<TypeConstraintItem>,
}
impl ConstraintGenerator {
    /// Create a fresh generator.
    pub fn new() -> Self {
        ConstraintGenerator {
            fresh: 0,
            constraints: Vec::new(),
        }
    }
    fn fresh_var(&mut self) -> HMType {
        let v = self.fresh;
        self.fresh += 1;
        HMType::Var(v)
    }
    fn emit_eq(&mut self, lhs: HMType, rhs: HMType, label: impl Into<String>) {
        self.constraints.push(TypeConstraintItem {
            lhs,
            rhs,
            label: Some(label.into()),
        });
    }
    /// Generate constraints for `expr` under `env`; return the assigned type.
    pub fn generate(&mut self, env: &TypeEnv, expr: &HMExpr) -> Result<HMType, String> {
        match expr {
            HMExpr::Bool(_) => Ok(HMType::Base("Bool".into())),
            HMExpr::Nat(_) => Ok(HMType::Base("Nat".into())),
            HMExpr::Var(x) => match env.bindings.get(x) {
                Some(scheme) => Ok(scheme.instantiate(&mut self.fresh)),
                None => Err(format!("unbound variable '{}'", x)),
            },
            HMExpr::Lam(x, body) => {
                let param_ty = self.fresh_var();
                let env2 = env.extend(x.clone(), TypeScheme::mono(param_ty.clone()));
                let body_ty = self.generate(&env2, body)?;
                Ok(HMType::Arrow(Box::new(param_ty), Box::new(body_ty)))
            }
            HMExpr::App(func, arg) => {
                let func_ty = self.generate(env, func)?;
                let arg_ty = self.generate(env, arg)?;
                let ret_ty = self.fresh_var();
                let expected = HMType::Arrow(Box::new(arg_ty), Box::new(ret_ty.clone()));
                self.emit_eq(func_ty, expected, "app");
                Ok(ret_ty)
            }
            HMExpr::Let(x, e1, e2) => {
                let ty1 = self.generate(env, e1)?;
                let env2 = env.extend(x.clone(), TypeScheme::mono(ty1));
                self.generate(&env2, e2)
            }
            HMExpr::Pair(e1, e2) => {
                let ty1 = self.generate(env, e1)?;
                let ty2 = self.generate(env, e2)?;
                Ok(HMType::Tuple(vec![ty1, ty2]))
            }
            HMExpr::If(cond, then, else_) => {
                let cond_ty = self.generate(env, cond)?;
                self.emit_eq(cond_ty, HMType::Base("Bool".into()), "if-cond");
                let then_ty = self.generate(env, then)?;
                let else_ty = self.generate(env, else_)?;
                let result = self.fresh_var();
                self.emit_eq(then_ty, result.clone(), "if-then");
                self.emit_eq(else_ty, result.clone(), "if-else");
                Ok(result)
            }
        }
    }
    /// Return the number of constraints generated.
    pub fn num_constraints(&self) -> usize {
        self.constraints.len()
    }
}
/// Annotated expression: a surface expression with optional type annotation.
#[derive(Debug, Clone)]
pub enum AnnExpr {
    /// An un-annotated expression.
    Plain(HMExpr),
    /// An expression with an explicit type annotation `(e : T)`.
    Ann(Box<AnnExpr>, HMType),
    /// Lambda with an annotated parameter `λ(x : T). e`.
    AnnLam(String, HMType, Box<AnnExpr>),
    /// Application of an annotated function.
    AnnApp(Box<AnnExpr>, Box<AnnExpr>),
}
/// A type scheme: ∀ α₁ ... αₙ. τ.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeScheme {
    /// Universally quantified type variables.
    pub vars: Vec<TyVar>,
    /// The body type.
    pub body: HMType,
}
impl TypeScheme {
    /// Creates a monomorphic type scheme (no universally bound variables).
    pub fn mono(ty: HMType) -> Self {
        TypeScheme {
            vars: vec![],
            body: ty,
        }
    }
    /// Returns the set of free type variables (those not bound by the scheme).
    pub fn ftv(&self) -> HashSet<TyVar> {
        let mut fv = self.body.ftv();
        for v in &self.vars {
            fv.remove(v);
        }
        fv
    }
    /// Apply a substitution to the body, skipping bound variables.
    pub fn apply(&self, subst: &TypeSubst) -> TypeScheme {
        let restricted = TypeSubst {
            map: subst
                .map
                .iter()
                .filter(|(k, _)| !self.vars.contains(k))
                .map(|(k, v)| (*k, v.clone()))
                .collect(),
        };
        TypeScheme {
            vars: self.vars.clone(),
            body: self.body.apply(&restricted),
        }
    }
    /// Instantiate this scheme with fresh type variables starting at `fresh`.
    pub fn instantiate(&self, fresh: &mut TyVar) -> HMType {
        let mut subst = TypeSubst::new();
        for &v in &self.vars {
            subst.map.insert(v, HMType::Var(*fresh));
            *fresh += 1;
        }
        self.body.apply(&subst)
    }
}
/// Algorithm W state: holds a fresh type variable counter.
pub struct AlgorithmW {
    fresh: TyVar,
}
impl AlgorithmW {
    /// Creates a new Algorithm W instance.
    pub fn new() -> Self {
        AlgorithmW { fresh: 0 }
    }
    /// Generates a fresh type variable.
    pub fn fresh_var(&mut self) -> HMType {
        let v = self.fresh;
        self.fresh += 1;
        HMType::Var(v)
    }
    /// Infer the type of `expr` in environment `env`.
    ///
    /// Returns `(substitution, inferred_type)` or an error.
    pub fn infer(&mut self, env: &TypeEnv, expr: &HMExpr) -> Result<(TypeSubst, HMType), String> {
        match expr {
            HMExpr::Bool(_) => Ok((TypeSubst::new(), HMType::Base("Bool".into()))),
            HMExpr::Nat(_) => Ok((TypeSubst::new(), HMType::Base("Nat".into()))),
            HMExpr::Var(x) => match env.bindings.get(x) {
                Some(scheme) => {
                    let ty = scheme.instantiate(&mut self.fresh);
                    Ok((TypeSubst::new(), ty))
                }
                None => Err(format!("unbound variable: {}", x)),
            },
            HMExpr::Lam(x, body) => {
                let param_ty = self.fresh_var();
                let env2 = env.extend(x.clone(), TypeScheme::mono(param_ty.clone()));
                let (s1, body_ty) = self.infer(&env2, body)?;
                let result_ty = HMType::Arrow(Box::new(param_ty.apply(&s1)), Box::new(body_ty));
                Ok((s1, result_ty))
            }
            HMExpr::App(func, arg) => {
                let result_ty = self.fresh_var();
                let (s1, func_ty) = self.infer(env, func)?;
                let env2 = env.apply(&s1);
                let (s2, arg_ty) = self.infer(&env2, arg)?;
                let expected_func_ty = HMType::Arrow(Box::new(arg_ty), Box::new(result_ty.clone()));
                let s3 = unify_types(&func_ty.apply(&s2), &expected_func_ty)
                    .map_err(|e| format!("application: {}", e))?;
                let final_ty = result_ty.apply(&s3).apply(&s2).apply(&s1);
                let final_subst = s3.compose(&s2).compose(&s1);
                Ok((final_subst, final_ty))
            }
            HMExpr::Let(x, e1, e2) => {
                let (s1, ty1) = self.infer(env, e1)?;
                let env1 = env.apply(&s1);
                let scheme = env1.generalize(&ty1);
                let env2 = env1.extend(x.clone(), scheme);
                let (s2, ty2) = self.infer(&env2, e2)?;
                Ok((s2.compose(&s1), ty2))
            }
            HMExpr::Pair(e1, e2) => {
                let (s1, ty1) = self.infer(env, e1)?;
                let env2 = env.apply(&s1);
                let (s2, ty2) = self.infer(&env2, e2)?;
                let ty1 = ty1.apply(&s2);
                let pair_ty = HMType::Tuple(vec![ty1, ty2]);
                Ok((s2.compose(&s1), pair_ty))
            }
            HMExpr::If(cond, then, else_) => {
                let (s0, cond_ty) = self.infer(env, cond)?;
                let s_bool = unify_types(&cond_ty, &HMType::Base("Bool".into()))
                    .map_err(|e| format!("if condition: {}", e))?;
                let subst0 = s_bool.compose(&s0);
                let env1 = env.apply(&subst0);
                let (s1, then_ty) = self.infer(&env1, then)?;
                let env2 = env1.apply(&s1);
                let (s2, else_ty) = self.infer(&env2, else_)?;
                let s3 = unify_types(&then_ty.apply(&s2), &else_ty)
                    .map_err(|e| format!("if branches: {}", e))?;
                let result_ty = else_ty.apply(&s3);
                let final_subst = s3.compose(&s2).compose(&s1).compose(&subst0);
                Ok((final_subst, result_ty))
            }
        }
    }
    /// Convenience: infer starting from an empty environment.
    pub fn infer_closed(&mut self, expr: &HMExpr) -> Result<HMType, String> {
        let env = TypeEnv::new();
        let (subst, ty) = self.infer(&env, expr)?;
        Ok(ty.apply(&subst))
    }
}
/// A type class declaration: a name and a list of required method types.
#[derive(Debug, Clone)]
pub struct TypeClass {
    /// Class name (e.g., "Eq", "Ord", "Functor").
    pub name: String,
    /// Method names and their type templates.
    pub methods: Vec<(String, HMType)>,
}
/// Constraint-based type inference.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ConstraintTypeInference {
    pub constraints: Vec<(String, String)>,
    pub solved: bool,
}
#[allow(dead_code)]
impl ConstraintTypeInference {
    /// Create a constraint set.
    pub fn new() -> Self {
        Self {
            constraints: Vec::new(),
            solved: false,
        }
    }
    /// Add a type equality constraint.
    pub fn add_constraint(&mut self, t1: &str, t2: &str) {
        self.constraints.push((t1.to_string(), t2.to_string()));
    }
    /// Number of constraints.
    pub fn num_constraints(&self) -> usize {
        self.constraints.len()
    }
    /// Solve (mark as solved after unification).
    pub fn solve(&mut self) {
        self.solved = true;
    }
}
/// Rank-N polymorphism data.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RankNType {
    pub rank: usize,
    pub type_str: String,
    pub is_prenex: bool,
}
#[allow(dead_code)]
impl RankNType {
    /// Rank-1 (prenex/Hindley-Milner) type.
    pub fn rank1(ty: &str) -> Self {
        Self {
            rank: 1,
            type_str: ty.to_string(),
            is_prenex: true,
        }
    }
    /// Rank-2 type.
    pub fn rank2(ty: &str) -> Self {
        Self {
            rank: 2,
            type_str: ty.to_string(),
            is_prenex: false,
        }
    }
    /// Rank-N types require annotation in general (undecidable type inference for N >= 3).
    pub fn inference_decidable(&self) -> bool {
        self.rank <= 2
    }
}
/// A type equality constraint.
#[derive(Debug, Clone)]
pub struct TypeConstraintEq {
    pub lhs: HMType,
    pub rhs: HMType,
}
/// A constraint generated during type checking: lhs must equal rhs.
#[derive(Debug, Clone)]
pub struct TypeConstraintItem {
    /// Left-hand side type.
    pub lhs: HMType,
    /// Right-hand side type.
    pub rhs: HMType,
    /// Optional label for debugging.
    pub label: Option<String>,
}
/// Solves a list of type constraints using Robinson's unification algorithm.
///
/// Extends the basic solver with:
/// - Constraint prioritization (base-type constraints first)
/// - Detailed error messages with constraint labels
/// - Support for inspecting the final substitution
pub struct UnificationSolver {
    /// The accumulated substitution.
    pub subst: TypeSubst,
    /// Log of solved constraints (for debugging).
    pub log: Vec<String>,
}
impl UnificationSolver {
    /// Create a new solver with an empty substitution.
    pub fn new() -> Self {
        UnificationSolver {
            subst: TypeSubst::new(),
            log: Vec::new(),
        }
    }
    /// Solve a single constraint, updating the accumulated substitution.
    pub fn solve_one(&mut self, item: &TypeConstraintItem) -> Result<(), String> {
        let lhs = item.lhs.apply(&self.subst);
        let rhs = item.rhs.apply(&self.subst);
        let label = item.label.as_deref().unwrap_or("?");
        match unify_types(&lhs, &rhs) {
            Ok(new_subst) => {
                self.log.push(format!(
                    "[{}] {} ~ {} => {} bindings",
                    label,
                    lhs,
                    rhs,
                    new_subst.map.len()
                ));
                self.subst = new_subst.compose(&self.subst);
                Ok(())
            }
            Err(e) => Err(format!("[{}] {}", label, e)),
        }
    }
    /// Solve all constraints in order, stopping on first error.
    pub fn solve_all(&mut self, constraints: &[TypeConstraintItem]) -> Result<(), String> {
        for c in constraints {
            self.solve_one(c)?;
        }
        Ok(())
    }
    /// Apply the final substitution to a type.
    pub fn apply(&self, ty: &HMType) -> HMType {
        ty.apply(&self.subst)
    }
    /// Full pipeline: generate + solve for an expression.
    pub fn infer(env: &TypeEnv, expr: &HMExpr) -> Result<HMType, String> {
        let mut gen = ConstraintGenerator::new();
        let ty = gen.generate(env, expr)?;
        let mut solver = UnificationSolver::new();
        solver.solve_all(&gen.constraints)?;
        Ok(solver.apply(&ty))
    }
}
/// Type class constraint: `C τ` meaning type τ must have class C.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClassConstraint {
    /// The required class.
    pub class: String,
    /// The type argument.
    pub ty: HMType,
}
/// Dependent type checking algorithm data.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DependentTypeChecker {
    pub mode: String,
    pub universe_levels: usize,
    pub has_cumulativity: bool,
}
#[allow(dead_code)]
impl DependentTypeChecker {
    /// Bidirectional type checking.
    pub fn bidirectional() -> Self {
        Self {
            mode: "bidirectional".to_string(),
            universe_levels: 1,
            has_cumulativity: false,
        }
    }
    /// With universe levels.
    pub fn with_universes(levels: usize, cumulative: bool) -> Self {
        Self {
            mode: "bidirectional+universes".to_string(),
            universe_levels: levels,
            has_cumulativity: cumulative,
        }
    }
    /// Checking vs inference mode description.
    pub fn mode_description(&self) -> String {
        format!(
            "Type checker: {} mode, {} universe levels, cumulativity={}",
            self.mode, self.universe_levels, self.has_cumulativity
        )
    }
}
/// Constraint-based type inference: generate constraints, then solve.
pub struct ConstraintInference {
    fresh: TyVar,
    constraints: Vec<TypeConstraintEq>,
}
impl ConstraintInference {
    /// Creates a fresh constraint inference engine.
    pub fn new() -> Self {
        ConstraintInference {
            fresh: 0,
            constraints: Vec::new(),
        }
    }
    fn fresh_var(&mut self) -> HMType {
        let v = self.fresh;
        self.fresh += 1;
        HMType::Var(v)
    }
    fn emit(&mut self, lhs: HMType, rhs: HMType) {
        self.constraints.push(TypeConstraintEq { lhs, rhs });
    }
    /// Generate constraints for `expr` in `env`, returning a type variable.
    pub fn gen_constraints(&mut self, env: &TypeEnv, expr: &HMExpr) -> Result<HMType, String> {
        match expr {
            HMExpr::Bool(_) => Ok(HMType::Base("Bool".into())),
            HMExpr::Nat(_) => Ok(HMType::Base("Nat".into())),
            HMExpr::Var(x) => match env.bindings.get(x) {
                Some(scheme) => Ok(scheme.instantiate(&mut self.fresh)),
                None => Err(format!("unbound variable: {}", x)),
            },
            HMExpr::Lam(x, body) => {
                let param_ty = self.fresh_var();
                let env2 = env.extend(x.clone(), TypeScheme::mono(param_ty.clone()));
                let body_ty = self.gen_constraints(&env2, body)?;
                Ok(HMType::Arrow(Box::new(param_ty), Box::new(body_ty)))
            }
            HMExpr::App(func, arg) => {
                let func_ty = self.gen_constraints(env, func)?;
                let arg_ty = self.gen_constraints(env, arg)?;
                let result_ty = self.fresh_var();
                self.emit(
                    func_ty,
                    HMType::Arrow(Box::new(arg_ty), Box::new(result_ty.clone())),
                );
                Ok(result_ty)
            }
            HMExpr::Let(x, e1, e2) => {
                let ty1 = self.gen_constraints(env, e1)?;
                let env2 = env.extend(x.clone(), TypeScheme::mono(ty1));
                self.gen_constraints(&env2, e2)
            }
            HMExpr::Pair(e1, e2) => {
                let ty1 = self.gen_constraints(env, e1)?;
                let ty2 = self.gen_constraints(env, e2)?;
                Ok(HMType::Tuple(vec![ty1, ty2]))
            }
            HMExpr::If(cond, then, else_) => {
                let cond_ty = self.gen_constraints(env, cond)?;
                self.emit(cond_ty, HMType::Base("Bool".into()));
                let then_ty = self.gen_constraints(env, then)?;
                let else_ty = self.gen_constraints(env, else_)?;
                self.emit(then_ty.clone(), else_ty);
                Ok(then_ty)
            }
        }
    }
    /// Solve the collected constraints by unification.
    pub fn solve(self) -> Result<TypeSubst, String> {
        let mut subst = TypeSubst::new();
        for c in self.constraints {
            let lhs = c.lhs.apply(&subst);
            let rhs = c.rhs.apply(&subst);
            let s = unify_types(&lhs, &rhs)?;
            subst = s.compose(&subst);
        }
        Ok(subst)
    }
    /// Full pipeline: generate constraints then solve.
    pub fn infer(env: &TypeEnv, expr: &HMExpr) -> Result<HMType, String> {
        let mut ci = ConstraintInference::new();
        let ty = ci.gen_constraints(env, expr)?;
        let subst = ci.solve()?;
        Ok(ty.apply(&subst))
    }
}
/// Bidirectional type checker using a simple type system.
pub struct BidirChecker {
    fresh: TyVar,
}
impl BidirChecker {
    /// Creates a new bidirectional checker.
    pub fn new() -> Self {
        BidirChecker { fresh: 0 }
    }
    fn fresh_var(&mut self) -> HMType {
        let v = self.fresh;
        self.fresh += 1;
        HMType::Var(v)
    }
    /// Check mode: verify `expr` has type `expected` under `env`.
    pub fn check(
        &mut self,
        env: &TypeEnv,
        expr: &HMExpr,
        expected: &HMType,
    ) -> Result<TypeSubst, String> {
        match expr {
            HMExpr::Lam(x, body) => {
                if let HMType::Arrow(dom, cod) = expected {
                    let env2 = env.extend(x.clone(), TypeScheme::mono(*dom.clone()));
                    self.check(&env2, body, cod)
                } else {
                    Err(format!("expected arrow type, got {}", expected))
                }
            }
            _ => {
                let mut w = AlgorithmW { fresh: self.fresh };
                let (subst, inferred) = w.infer(env, expr)?;
                self.fresh = w.fresh;
                let unified = unify_types(&inferred.apply(&subst), expected)?;
                Ok(unified.compose(&subst))
            }
        }
    }
    /// Infer the type of `expr` under `env`.
    pub fn infer_type(
        &mut self,
        env: &TypeEnv,
        expr: &HMExpr,
    ) -> Result<(TypeSubst, HMType), String> {
        let mut w = AlgorithmW { fresh: self.fresh };
        let result = w.infer(env, expr);
        self.fresh = w.fresh;
        result
    }
}
/// A Hindley-Milner monotype.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HMType {
    /// Type variable α.
    Var(TyVar),
    /// Base type (e.g., Int, Bool).
    Base(String),
    /// Function type τ₁ → τ₂.
    Arrow(Box<HMType>, Box<HMType>),
    /// Type constructor applied to arguments (e.g., List τ).
    App(String, Vec<HMType>),
    /// Tuple type (τ₁, τ₂, ..., τₙ).
    Tuple(Vec<HMType>),
}
impl HMType {
    /// Returns the set of free type variables.
    pub fn ftv(&self) -> HashSet<TyVar> {
        match self {
            HMType::Var(v) => {
                let mut s = HashSet::new();
                s.insert(*v);
                s
            }
            HMType::Base(_) => HashSet::new(),
            HMType::Arrow(a, b) => {
                let mut s = a.ftv();
                s.extend(b.ftv());
                s
            }
            HMType::App(_, args) => args.iter().flat_map(|a| a.ftv()).collect(),
            HMType::Tuple(ts) => ts.iter().flat_map(|t| t.ftv()).collect(),
        }
    }
    /// Apply a type substitution to this type.
    pub fn apply(&self, subst: &TypeSubst) -> HMType {
        match self {
            HMType::Var(v) => subst.map.get(v).cloned().unwrap_or(HMType::Var(*v)),
            HMType::Base(s) => HMType::Base(s.clone()),
            HMType::Arrow(a, b) => {
                HMType::Arrow(Box::new(a.apply(subst)), Box::new(b.apply(subst)))
            }
            HMType::App(c, args) => {
                HMType::App(c.clone(), args.iter().map(|a| a.apply(subst)).collect())
            }
            HMType::Tuple(ts) => HMType::Tuple(ts.iter().map(|t| t.apply(subst)).collect()),
        }
    }
    /// Returns `true` if `var` occurs in this type.
    pub fn occurs(&self, var: TyVar) -> bool {
        self.ftv().contains(&var)
    }
}
/// A bidirectional type inferencer that propagates known types inward.
///
/// In *check mode* (`check`), we verify the expression has the given type.
/// In *synth mode* (`synth`), we synthesize a type from the expression.
pub struct BidirectionalInferencer {
    fresh: TyVar,
    /// Collected constraints during bidirectional checking.
    pub constraints: Vec<TypeConstraintItem>,
}
impl BidirectionalInferencer {
    /// Create a fresh inferencer.
    pub fn new() -> Self {
        BidirectionalInferencer {
            fresh: 0,
            constraints: Vec::new(),
        }
    }
    fn fresh_var(&mut self) -> HMType {
        let v = self.fresh;
        self.fresh += 1;
        HMType::Var(v)
    }
    fn emit(&mut self, lhs: HMType, rhs: HMType, label: &str) {
        self.constraints.push(TypeConstraintItem {
            lhs,
            rhs,
            label: Some(label.into()),
        });
    }
    /// Synthesis mode: infer a type for `expr`.
    pub fn synth(&mut self, env: &TypeEnv, expr: &AnnExpr) -> Result<HMType, String> {
        match expr {
            AnnExpr::Plain(e) => {
                let mut gen = ConstraintGenerator {
                    fresh: self.fresh,
                    constraints: Vec::new(),
                };
                let ty = gen.generate(env, e)?;
                self.fresh = gen.fresh;
                self.constraints.extend(gen.constraints);
                Ok(ty)
            }
            AnnExpr::Ann(e, ann_ty) => {
                self.check(env, e, ann_ty)?;
                Ok(ann_ty.clone())
            }
            AnnExpr::AnnLam(x, param_ty, body) => {
                let env2 = env.extend(x.clone(), TypeScheme::mono(param_ty.clone()));
                let body_ty = self.synth(&env2, body)?;
                Ok(HMType::Arrow(Box::new(param_ty.clone()), Box::new(body_ty)))
            }
            AnnExpr::AnnApp(func, arg) => {
                let func_ty = self.synth(env, func)?;
                let arg_ty = self.synth(env, arg)?;
                let ret = self.fresh_var();
                let expected = HMType::Arrow(Box::new(arg_ty), Box::new(ret.clone()));
                self.emit(func_ty, expected, "bidir-app");
                Ok(ret)
            }
        }
    }
    /// Check mode: verify `expr` has type `expected`.
    pub fn check(
        &mut self,
        env: &TypeEnv,
        expr: &AnnExpr,
        expected: &HMType,
    ) -> Result<(), String> {
        match expr {
            AnnExpr::AnnLam(x, param_ty, body) => {
                if let HMType::Arrow(dom, cod) = expected {
                    self.emit(param_ty.clone(), *dom.clone(), "lam-param");
                    let env2 = env.extend(x.clone(), TypeScheme::mono(param_ty.clone()));
                    self.check(&env2, body, cod)
                } else {
                    Err(format!("expected arrow type, got {}", expected))
                }
            }
            _ => {
                let inferred = self.synth(env, expr)?;
                self.emit(inferred, expected.clone(), "check-synth");
                Ok(())
            }
        }
    }
    /// Solve all collected constraints and apply the substitution to `ty`.
    pub fn solve_and_apply(&mut self, ty: &HMType) -> Result<HMType, String> {
        let mut solver = UnificationSolver::new();
        solver.solve_all(&self.constraints)?;
        Ok(solver.apply(ty))
    }
    /// Full bidirectional inference from an annotated expression.
    pub fn infer(env: &TypeEnv, expr: &AnnExpr) -> Result<HMType, String> {
        let mut bi = BidirectionalInferencer::new();
        let ty = bi.synth(env, expr)?;
        bi.solve_and_apply(&ty)
    }
}
/// Algorithm J uses mutable type variables (union-find style).
///
/// This is an imperative variant of W that avoids repeated composition
/// by threading substitutions through a mutable reference.
///
/// A mutable type used in Algorithm J.
#[derive(Debug, Clone)]
pub enum MutType {
    /// An unbound type variable with index `id`.
    Free(u32),
    /// A type variable that has been unified to `ty`.
    Bound(Box<MutType>),
    /// A base type.
    Base(String),
    /// Arrow type.
    Arrow(Box<MutType>, Box<MutType>),
    /// Type application.
    App(String, Vec<MutType>),
}
impl MutType {
    /// Dereference all bound type variables (path compression).
    pub fn deref(&self) -> MutType {
        match self {
            MutType::Bound(inner) => inner.deref(),
            other => other.clone(),
        }
    }
    /// Collect free variable indices.
    pub fn free_vars(&self) -> HashSet<u32> {
        match self.deref() {
            MutType::Free(i) => {
                let mut s = HashSet::new();
                s.insert(i);
                s
            }
            MutType::Bound(_) => HashSet::new(),
            MutType::Base(_) => HashSet::new(),
            MutType::Arrow(a, b) => {
                let mut s = a.free_vars();
                s.extend(b.free_vars());
                s
            }
            MutType::App(_, args) => args.iter().flat_map(|a| a.free_vars()).collect(),
        }
    }
}
/// A typing environment: maps term variable names to type schemes.
#[derive(Debug, Clone, Default)]
pub struct TypeEnv {
    pub bindings: HashMap<String, TypeScheme>,
}
impl TypeEnv {
    /// Creates an empty environment.
    pub fn new() -> Self {
        TypeEnv {
            bindings: HashMap::new(),
        }
    }
    /// Extends the environment with a binding `x : σ`.
    pub fn extend(&self, x: impl Into<String>, sigma: TypeScheme) -> TypeEnv {
        let mut env = self.clone();
        env.bindings.insert(x.into(), sigma);
        env
    }
    /// Returns the free type variables of all schemes in the environment.
    pub fn ftv(&self) -> HashSet<TyVar> {
        self.bindings.values().flat_map(|s| s.ftv()).collect()
    }
    /// Apply a substitution to all type schemes.
    pub fn apply(&self, subst: &TypeSubst) -> TypeEnv {
        TypeEnv {
            bindings: self
                .bindings
                .iter()
                .map(|(k, v)| (k.clone(), v.apply(subst)))
                .collect(),
        }
    }
    /// Generalize `ty` over all free variables not in the environment.
    pub fn generalize(&self, ty: &HMType) -> TypeScheme {
        let env_fv = self.ftv();
        let ty_fv = ty.ftv();
        let bound: Vec<TyVar> = ty_fv.difference(&env_fv).copied().collect();
        TypeScheme {
            vars: bound,
            body: ty.clone(),
        }
    }
}
/// A simple lambda-calculus with let-expressions and literals.
#[derive(Debug, Clone)]
pub enum HMExpr {
    /// Variable reference.
    Var(String),
    /// Lambda abstraction `λx. e`.
    Lam(String, Box<HMExpr>),
    /// Application `e1 e2`.
    App(Box<HMExpr>, Box<HMExpr>),
    /// Let-binding `let x = e1 in e2`.
    Let(String, Box<HMExpr>, Box<HMExpr>),
    /// Boolean literal.
    Bool(bool),
    /// Natural number literal.
    Nat(u64),
    /// Pair `(e1, e2)`.
    Pair(Box<HMExpr>, Box<HMExpr>),
    /// If-then-else.
    If(Box<HMExpr>, Box<HMExpr>, Box<HMExpr>),
}
/// A type substitution: map from type variables to types.
#[derive(Debug, Clone, Default)]
pub struct TypeSubst {
    pub map: HashMap<TyVar, HMType>,
}
impl TypeSubst {
    /// Creates an empty substitution.
    pub fn new() -> Self {
        TypeSubst {
            map: HashMap::new(),
        }
    }
    /// Binds type variable `v` to type `ty`.
    pub fn bind(v: TyVar, ty: HMType) -> Self {
        TypeSubst {
            map: HashMap::from([(v, ty)]),
        }
    }
    /// Compose `self` after `other`: `(self ∘ other)(v) = self(other(v))`.
    pub fn compose(&self, other: &TypeSubst) -> TypeSubst {
        let mut result = TypeSubst::new();
        for (&v, ty) in &other.map {
            result.map.insert(v, ty.apply(self));
        }
        for (&v, ty) in &self.map {
            if !other.map.contains_key(&v) {
                result.map.insert(v, ty.clone());
            }
        }
        result
    }
}
/// A liquid type: a base type refined by a predicate.
#[derive(Debug, Clone)]
pub struct LiquidType {
    /// The base type.
    pub base: HMType,
    /// The predicate (represented as a string formula for demonstration).
    pub predicate: String,
}
impl LiquidType {
    /// Creates a liquid type `{x : base | predicate}`.
    pub fn new(base: HMType, predicate: impl Into<String>) -> Self {
        LiquidType {
            base,
            predicate: predicate.into(),
        }
    }
    /// A trivially-true liquid type (no refinement).
    pub fn trivial(base: HMType) -> Self {
        LiquidType::new(base, "true")
    }
    /// Checks if the refinement is syntactically trivial.
    pub fn is_trivial(&self) -> bool {
        self.predicate.trim() == "true"
    }
    /// Subtyping check (syntactic approximation):
    /// `{x : T | P}` <: `{x : T | Q}` if base types match and Q is implied by P.
    ///
    /// In a real implementation this would call an SMT solver; here we do
    /// a conservative syntactic check.
    pub fn is_subtype_of(&self, other: &LiquidType) -> bool {
        self.base == other.base && (other.is_trivial() || self.predicate == other.predicate)
    }
}
/// Gradual typing data.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GradualTypeData {
    pub static_portion: String,
    pub has_dynamic_part: bool,
    pub consistency_relation: String,
}
#[allow(dead_code)]
impl GradualTypeData {
    /// Fully static type.
    pub fn static_type(ty: &str) -> Self {
        Self {
            static_portion: ty.to_string(),
            has_dynamic_part: false,
            consistency_relation: "structural equality".to_string(),
        }
    }
    /// Dynamic type (the unknown type ?).
    pub fn dynamic() -> Self {
        Self {
            static_portion: "?".to_string(),
            has_dynamic_part: true,
            consistency_relation: "everything is consistent with ?".to_string(),
        }
    }
    /// Gradual guarantee: static type safety + flexibility.
    pub fn gradual_guarantee(&self) -> String {
        format!(
            "Gradual guarantee for {}: static={}, dynamic={}",
            self.static_portion, !self.has_dynamic_part, self.has_dynamic_part
        )
    }
}
/// Resolves type class constraints given a set of instances.
///
/// This implements the instance resolution algorithm as described in
/// "Type Classes in Haskell" (Hall et al., 1996).
pub struct TypeClassResolution {
    /// Registered type class declarations.
    pub classes: HashMap<String, TypeClass>,
    /// Registered instances.
    pub instances: Vec<TypeClassInstance>,
}
impl TypeClassResolution {
    /// Create an empty resolver.
    pub fn new() -> Self {
        TypeClassResolution {
            classes: HashMap::new(),
            instances: Vec::new(),
        }
    }
    /// Register a type class.
    pub fn add_class(&mut self, class: TypeClass) {
        self.classes.insert(class.name.clone(), class);
    }
    /// Register an instance.
    pub fn add_instance(&mut self, inst: TypeClassInstance) {
        self.instances.push(inst);
    }
    /// Attempt to resolve constraint `C τ`, returning the matching instance if found.
    pub fn resolve(&self, constraint: &ClassConstraint) -> Option<&TypeClassInstance> {
        self.instances.iter().find(|inst| {
            inst.class_name == constraint.class && self.matches(&inst.instance_ty, &constraint.ty)
        })
    }
    /// Check whether `pattern` matches `ty` (simple syntactic matching).
    fn matches(&self, pattern: &HMType, ty: &HMType) -> bool {
        match (pattern, ty) {
            (HMType::Var(_), _) => true,
            (HMType::Base(a), HMType::Base(b)) => a == b,
            (HMType::Arrow(a1, b1), HMType::Arrow(a2, b2)) => {
                self.matches(a1, a2) && self.matches(b1, b2)
            }
            (HMType::App(f1, args1), HMType::App(f2, args2)) => {
                f1 == f2
                    && args1.len() == args2.len()
                    && args1
                        .iter()
                        .zip(args2.iter())
                        .all(|(a, b)| self.matches(a, b))
            }
            _ => false,
        }
    }
    /// Resolve a set of constraints, returning errors for unresolved ones.
    pub fn resolve_all(
        &self,
        constraints: &[ClassConstraint],
    ) -> Result<Vec<&TypeClassInstance>, Vec<ClassConstraint>> {
        let mut resolved = Vec::new();
        let mut failed = Vec::new();
        for c in constraints {
            match self.resolve(c) {
                Some(inst) => resolved.push(inst),
                None => failed.push(c.clone()),
            }
        }
        if failed.is_empty() {
            Ok(resolved)
        } else {
            Err(failed)
        }
    }
    /// Check coherence: for each constraint, at most one matching instance exists.
    pub fn is_coherent(&self) -> bool {
        for inst1 in &self.instances {
            for inst2 in &self.instances {
                if std::ptr::eq(inst1, inst2) {
                    continue;
                }
                if inst1.class_name == inst2.class_name
                    && self.matches(&inst1.instance_ty, &inst2.instance_ty)
                    && self.matches(&inst2.instance_ty, &inst1.instance_ty)
                {
                    return false;
                }
            }
        }
        true
    }
}
/// A row: a list of (label, type) pairs, possibly ending with a row variable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Row {
    /// The empty row.
    Empty,
    /// A row variable.
    Var(TyVar),
    /// A row extension: `(label : ty | rest)`.
    Extend(String, HMType, Box<Row>),
}
impl Row {
    /// Creates a record type from this row.
    pub fn to_record_type(&self) -> HMType {
        HMType::App("Record".into(), vec![self.to_hm_type()])
    }
    fn to_hm_type(&self) -> HMType {
        match self {
            Row::Empty => HMType::Base("RowNil".into()),
            Row::Var(v) => HMType::Var(*v),
            Row::Extend(label, ty, rest) => HMType::App(
                "RowCons".into(),
                vec![HMType::Base(label.clone()), ty.clone(), rest.to_hm_type()],
            ),
        }
    }
    /// Collect the labels defined in this row.
    pub fn labels(&self) -> Vec<String> {
        match self {
            Row::Empty | Row::Var(_) => vec![],
            Row::Extend(l, _, rest) => {
                let mut labels = rest.labels();
                labels.push(l.clone());
                labels
            }
        }
    }
    /// Look up the type of `label` in this row.
    pub fn lookup(&self, label: &str) -> Option<HMType> {
        match self {
            Row::Empty | Row::Var(_) => None,
            Row::Extend(l, ty, rest) => {
                if l == label {
                    Some(ty.clone())
                } else {
                    rest.lookup(label)
                }
            }
        }
    }
}
/// A type class instance: provides concrete method implementations.
#[derive(Debug, Clone)]
pub struct TypeClassInstance {
    /// The class this instance belongs to.
    pub class_name: String,
    /// The concrete type this instance is for.
    pub instance_ty: HMType,
    /// Concrete method types (must match the class template after substitution).
    pub method_types: Vec<(String, HMType)>,
}
/// Mode of bidirectional type checking.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// Check mode: verify the expression has the given type.
    Check,
    /// Infer mode: synthesize the type of the expression.
    Infer,
}
/// Row polymorphism for extensible records.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RowType {
    pub fields: Vec<(String, String)>,
    pub row_variable: Option<String>,
}
#[allow(dead_code)]
impl RowType {
    /// Closed row type (no row variable).
    pub fn closed(fields: Vec<(&str, &str)>) -> Self {
        Self {
            fields: fields
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
            row_variable: None,
        }
    }
    /// Open row type (with row variable for extensibility).
    pub fn open(fields: Vec<(&str, &str)>, row_var: &str) -> Self {
        Self {
            fields: fields
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
            row_variable: Some(row_var.to_string()),
        }
    }
    /// Is this a closed record type?
    pub fn is_closed(&self) -> bool {
        self.row_variable.is_none()
    }
    /// Extend with a new field.
    pub fn extend(&self, field: &str, ty: &str) -> Self {
        let mut new_fields = self.fields.clone();
        new_fields.push((field.to_string(), ty.to_string()));
        Self {
            fields: new_fields,
            row_variable: self.row_variable.clone(),
        }
    }
}
