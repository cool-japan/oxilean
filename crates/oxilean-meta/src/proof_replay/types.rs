//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::Name;
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::io::Write;

/// A typed slot for ProofReplay configuration.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ProofReplayConfigValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
#[allow(dead_code)]
impl ProofReplayConfigValue {
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            ProofReplayConfigValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
    pub fn as_int(&self) -> Option<i64> {
        match self {
            ProofReplayConfigValue::Int(i) => Some(*i),
            _ => None,
        }
    }
    pub fn as_float(&self) -> Option<f64> {
        match self {
            ProofReplayConfigValue::Float(f) => Some(*f),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        match self {
            ProofReplayConfigValue::Str(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_list(&self) -> Option<&[String]> {
        match self {
            ProofReplayConfigValue::List(v) => Some(v),
            _ => None,
        }
    }
    pub fn type_name(&self) -> &'static str {
        match self {
            ProofReplayConfigValue::Bool(_) => "bool",
            ProofReplayConfigValue::Int(_) => "int",
            ProofReplayConfigValue::Float(_) => "float",
            ProofReplayConfigValue::Str(_) => "str",
            ProofReplayConfigValue::List(_) => "list",
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ProofReplayExtConfigVal2600 {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
impl ProofReplayExtConfigVal2600 {
    #[allow(dead_code)]
    pub fn as_bool(&self) -> Option<bool> {
        if let ProofReplayExtConfigVal2600::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_int(&self) -> Option<i64> {
        if let ProofReplayExtConfigVal2600::Int(i) = self {
            Some(*i)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_float(&self) -> Option<f64> {
        if let ProofReplayExtConfigVal2600::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_str(&self) -> Option<&str> {
        if let ProofReplayExtConfigVal2600::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_list(&self) -> Option<&[String]> {
        if let ProofReplayExtConfigVal2600::List(l) = self {
            Some(l)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn type_name(&self) -> &'static str {
        match self {
            ProofReplayExtConfigVal2600::Bool(_) => "bool",
            ProofReplayExtConfigVal2600::Int(_) => "int",
            ProofReplayExtConfigVal2600::Float(_) => "float",
            ProofReplayExtConfigVal2600::Str(_) => "str",
            ProofReplayExtConfigVal2600::List(_) => "list",
        }
    }
}
#[allow(dead_code)]
pub struct ProofReplayExtConfig2600 {
    pub(super) values: std::collections::HashMap<String, ProofReplayExtConfigVal2600>,
    pub(super) read_only: bool,
    pub(super) name: String,
}
impl ProofReplayExtConfig2600 {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            values: std::collections::HashMap::new(),
            read_only: false,
            name: String::new(),
        }
    }
    #[allow(dead_code)]
    pub fn named(name: &str) -> Self {
        Self {
            values: std::collections::HashMap::new(),
            read_only: false,
            name: name.to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn set(&mut self, key: &str, value: ProofReplayExtConfigVal2600) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    #[allow(dead_code)]
    pub fn get(&self, key: &str) -> Option<&ProofReplayExtConfigVal2600> {
        self.values.get(key)
    }
    #[allow(dead_code)]
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.get(key)?.as_bool()
    }
    #[allow(dead_code)]
    pub fn get_int(&self, key: &str) -> Option<i64> {
        self.get(key)?.as_int()
    }
    #[allow(dead_code)]
    pub fn get_str(&self, key: &str) -> Option<&str> {
        self.get(key)?.as_str()
    }
    #[allow(dead_code)]
    pub fn set_bool(&mut self, key: &str, v: bool) -> bool {
        self.set(key, ProofReplayExtConfigVal2600::Bool(v))
    }
    #[allow(dead_code)]
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, ProofReplayExtConfigVal2600::Int(v))
    }
    #[allow(dead_code)]
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, ProofReplayExtConfigVal2600::Str(v.to_string()))
    }
    #[allow(dead_code)]
    pub fn lock(&mut self) {
        self.read_only = true;
    }
    #[allow(dead_code)]
    pub fn unlock(&mut self) {
        self.read_only = false;
    }
    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        self.values.len()
    }
    #[allow(dead_code)]
    pub fn has(&self, key: &str) -> bool {
        self.values.contains_key(key)
    }
    #[allow(dead_code)]
    pub fn remove(&mut self, key: &str) -> bool {
        self.values.remove(key).is_some()
    }
}
/// A lightweight expression type used for metavariable-aware proof terms.
///
/// Metavariable IDs ≥ 1_000_000 are treated as unresolved placeholders.
/// When an assignment `mvar_id → value` is known, `apply_assignments` walks
/// the tree and substitutes.
#[derive(Clone, Debug, PartialEq)]
pub enum MetaExpr {
    /// A free variable.  IDs ≥ 1_000_000 are metavariables.
    FVar(u64),
    /// A named constant
    Const(String),
    /// Function application
    App(Box<MetaExpr>, Box<MetaExpr>),
    /// Lambda abstraction (binder name, domain type, body)
    Lam(String, Box<MetaExpr>, Box<MetaExpr>),
    /// Dependent product (binder name, domain type, codomain)
    Pi(String, Box<MetaExpr>, Box<MetaExpr>),
    /// Let binding (name, type, value, body)
    Let(String, Box<MetaExpr>, Box<MetaExpr>, Box<MetaExpr>),
    /// Structure field projection (field index, struct expression)
    Proj(u32, Box<MetaExpr>),
    /// A natural-number literal
    Nat(u64),
}
impl MetaExpr {
    /// Recursively substitute all assigned metavariables in `assignments` throughout
    /// this expression.
    ///
    /// For `FVar(id)` where `id >= 1_000_000`: if `assignments` contains an entry
    /// for `id`, replace the node with the assigned value (and continue substituting
    /// into the replacement in case it itself contains metavariables).
    /// All other node kinds are recursed into.
    pub fn apply_assignments(&self, assignments: &HashMap<u64, MetaExpr>) -> MetaExpr {
        match self {
            MetaExpr::FVar(id) if *id >= METAVAR_THRESHOLD => {
                if let Some(val) = assignments.get(id) {
                    val.apply_assignments(assignments)
                } else {
                    self.clone()
                }
            }
            MetaExpr::FVar(_) | MetaExpr::Const(_) | MetaExpr::Nat(_) => self.clone(),
            MetaExpr::App(f, arg) => MetaExpr::App(
                Box::new(f.apply_assignments(assignments)),
                Box::new(arg.apply_assignments(assignments)),
            ),
            MetaExpr::Lam(name, dom, body) => MetaExpr::Lam(
                name.clone(),
                Box::new(dom.apply_assignments(assignments)),
                Box::new(body.apply_assignments(assignments)),
            ),
            MetaExpr::Pi(name, dom, cod) => MetaExpr::Pi(
                name.clone(),
                Box::new(dom.apply_assignments(assignments)),
                Box::new(cod.apply_assignments(assignments)),
            ),
            MetaExpr::Let(name, ty, val, body) => MetaExpr::Let(
                name.clone(),
                Box::new(ty.apply_assignments(assignments)),
                Box::new(val.apply_assignments(assignments)),
                Box::new(body.apply_assignments(assignments)),
            ),
            MetaExpr::Proj(idx, inner) => {
                MetaExpr::Proj(*idx, Box::new(inner.apply_assignments(assignments)))
            }
        }
    }
    /// Return true if this expression contains any unresolved metavariable
    /// (i.e. `FVar(id)` with `id >= 1_000_000`) that is not covered by `assignments`.
    pub fn has_unresolved_metavars(&self, assignments: &HashMap<u64, MetaExpr>) -> bool {
        match self {
            MetaExpr::FVar(id) if *id >= METAVAR_THRESHOLD => !assignments.contains_key(id),
            MetaExpr::FVar(_) | MetaExpr::Const(_) | MetaExpr::Nat(_) => false,
            MetaExpr::App(f, arg) => {
                f.has_unresolved_metavars(assignments) || arg.has_unresolved_metavars(assignments)
            }
            MetaExpr::Lam(_, dom, body) | MetaExpr::Pi(_, dom, body) => {
                dom.has_unresolved_metavars(assignments)
                    || body.has_unresolved_metavars(assignments)
            }
            MetaExpr::Let(_, ty, val, body) => {
                ty.has_unresolved_metavars(assignments)
                    || val.has_unresolved_metavars(assignments)
                    || body.has_unresolved_metavars(assignments)
            }
            MetaExpr::Proj(_, inner) => inner.has_unresolved_metavars(assignments),
        }
    }
}
/// A pipeline of ProofReplay analysis passes.
#[allow(dead_code)]
pub struct ProofReplayPipeline {
    pub passes: Vec<ProofReplayAnalysisPass>,
    pub name: String,
    pub total_inputs_processed: usize,
}
#[allow(dead_code)]
impl ProofReplayPipeline {
    pub fn new(name: &str) -> Self {
        ProofReplayPipeline {
            passes: Vec::new(),
            name: name.to_string(),
            total_inputs_processed: 0,
        }
    }
    pub fn add_pass(&mut self, pass: ProofReplayAnalysisPass) {
        self.passes.push(pass);
    }
    pub fn run_all(&mut self, input: &str) -> Vec<ProofReplayResult> {
        self.total_inputs_processed += 1;
        self.passes
            .iter_mut()
            .filter(|p| p.enabled)
            .map(|p| p.run(input))
            .collect()
    }
    pub fn num_passes(&self) -> usize {
        self.passes.len()
    }
    pub fn num_enabled_passes(&self) -> usize {
        self.passes.iter().filter(|p| p.enabled).count()
    }
    pub fn total_success_rate(&self) -> f64 {
        if self.passes.is_empty() {
            0.0
        } else {
            let total_rate: f64 = self.passes.iter().map(|p| p.success_rate()).sum();
            total_rate / self.passes.len() as f64
        }
    }
}
/// Replays proof scripts
pub struct ProofReplayer {
    /// Maximum history depth
    pub(super) max_history: usize,
}
impl ProofReplayer {
    /// Create a new replayer
    pub fn new() -> Self {
        ProofReplayer { max_history: 1000 }
    }
    /// Create with custom history depth
    pub fn with_history(max_history: usize) -> Self {
        ProofReplayer { max_history }
    }
    /// Replay a proof script
    pub fn replay(&self, script: &ProofScript) -> Result<ReplayState, ReplayError> {
        script.validate_structure()?;
        let mut state = ReplayState::new(script.goal.clone(), script.hypotheses.clone());
        for step in &script.steps {
            self.replay_step(step, &mut state)?;
        }
        Ok(state)
    }
    /// Replay a single step
    pub fn replay_step(
        &self,
        step: &ProofStep,
        state: &mut ReplayState,
    ) -> Result<(), ReplayError> {
        state.save_state();
        match step {
            ProofStep::Intro { name } => {
                self.apply_intro(state, name.as_ref())?;
            }
            ProofStep::Apply { term } => {
                self.apply_apply(state, term)?;
            }
            ProofStep::Exact { term } => {
                self.apply_exact(state, term)?;
            }
            ProofStep::Rewrite { lemma, location } => {
                self.apply_rewrite(state, lemma, location.as_deref())?;
            }
            ProofStep::Cases { term } => {
                self.apply_cases(state, term)?;
            }
            ProofStep::Induction { term } => {
                self.apply_induction(state, term)?;
            }
            ProofStep::Clear { name } => {
                self.apply_clear(state, name)?;
            }
            ProofStep::Subst { var } => {
                self.apply_subst(state, var)?;
            }
            ProofStep::Have { name, ty, proof } => {
                self.apply_have(state, name.as_ref(), ty, proof)?;
            }
            ProofStep::Calc { entries } => {
                self.apply_calc(state, entries)?;
            }
            ProofStep::Simp {
                lemmas,
                use_default,
            } => {
                self.apply_simp(state, lemmas, *use_default)?;
            }
            ProofStep::Omega => {
                self.apply_omega(state)?;
            }
            ProofStep::Trivial => {
                self.apply_trivial(state)?;
            }
            ProofStep::Sequence { steps } => {
                for substep in steps {
                    self.replay_step(substep, state)?;
                }
            }
            ProofStep::First { alternatives } => {
                let mut last_err = None;
                for alt in alternatives {
                    match self.replay_step(alt, state) {
                        Ok(()) => return Ok(()),
                        Err(e) => last_err = Some(e),
                    }
                    state.restore_state()?;
                }
                if let Some(e) = last_err {
                    return Err(e);
                }
                return Err(ReplayError::TacticFailed(
                    "No alternative succeeded".to_string(),
                ));
            }
        }
        Ok(())
    }
    fn apply_intro(&self, state: &mut ReplayState, name: Option<&Name>) -> Result<(), ReplayError> {
        let goal = state.goal.trim().to_string();
        if goal.starts_with('∀') || goal.starts_with("forall") {
            let var_name = name
                .map(|n| n.to_string())
                .unwrap_or_else(|| "x".to_string());
            let new_goal = if let Some(comma_pos) = goal.find(',') {
                goal[comma_pos + 1..].trim().to_string()
            } else {
                format!("body[{}/x]", var_name)
            };
            let hyp_ty = extract_binder_type(&goal, &var_name);
            state.hypotheses.insert(var_name, hyp_ty);
            state.goal = new_goal;
            Ok(())
        } else if goal.contains("->") || goal.contains('→') {
            let var_name = name
                .map(|n| n.to_string())
                .unwrap_or_else(|| "h".to_string());
            let (antecedent, consequent) = split_arrow(&goal);
            state.hypotheses.insert(var_name, antecedent);
            state.goal = consequent;
            Ok(())
        } else {
            Err(ReplayError::TacticFailed(
                "Cannot intro non-forall goal".to_string(),
            ))
        }
    }
    fn apply_apply(&self, state: &mut ReplayState, term: &str) -> Result<(), ReplayError> {
        if term.is_empty() {
            return Err(ReplayError::InvalidProofState(
                "Empty apply term".to_string(),
            ));
        }
        if let Some(hyp_ty) = state.hypotheses.get(term).cloned() {
            if hyp_ty.contains("->") || hyp_ty.contains('→') {
                let (antecedent, _consequent) = split_arrow(&hyp_ty);
                state.goal = antecedent;
            } else {
                state.goal = format!("subgoal[apply {}]", term);
            }
        } else {
            state.goal = format!("subgoal[apply {}]", term);
        }
        Ok(())
    }
    fn apply_exact(&self, state: &mut ReplayState, term: &str) -> Result<(), ReplayError> {
        if term.is_empty() {
            return Err(ReplayError::InvalidProofState(
                "Empty exact term".to_string(),
            ));
        }
        state.complete = true;
        state.goal = format!("proved by {}", term);
        Ok(())
    }
    fn apply_rewrite(
        &self,
        state: &mut ReplayState,
        lemma: &str,
        _location: Option<&str>,
    ) -> Result<(), ReplayError> {
        if lemma.is_empty() {
            return Err(ReplayError::InvalidProofState(
                "Empty rewrite lemma".to_string(),
            ));
        }
        state.goal = format!("(goal after rewrite {})", lemma);
        Ok(())
    }
    fn apply_cases(&self, state: &mut ReplayState, term: &str) -> Result<(), ReplayError> {
        if term.is_empty() {
            return Err(ReplayError::InvalidProofState(
                "Empty cases term".to_string(),
            ));
        }
        if !state.hypotheses.contains_key(term) {
            return Err(ReplayError::UnknownHypothesis(term.to_string()));
        }
        state.goal = format!("(goal after cases {})", term);
        Ok(())
    }
    fn apply_induction(&self, state: &mut ReplayState, term: &str) -> Result<(), ReplayError> {
        if term.is_empty() {
            return Err(ReplayError::InvalidProofState(
                "Empty induction term".to_string(),
            ));
        }
        if !state.hypotheses.contains_key(term) {
            return Err(ReplayError::UnknownHypothesis(term.to_string()));
        }
        state.goal = format!("(goal after induction {})", term);
        Ok(())
    }
    fn apply_clear(&self, state: &mut ReplayState, name: &Name) -> Result<(), ReplayError> {
        let name_str = name.to_string();
        if state.hypotheses.remove(&name_str).is_none() {
            return Err(ReplayError::UnknownHypothesis(name_str));
        }
        Ok(())
    }
    fn apply_subst(&self, state: &mut ReplayState, var: &Name) -> Result<(), ReplayError> {
        let var_str = var.to_string();
        if state.hypotheses.contains_key(&var_str) {
            state.hypotheses.remove(&var_str);
        }
        state.goal = format!("(goal after subst {})", var_str);
        Ok(())
    }
    fn apply_have(
        &self,
        state: &mut ReplayState,
        name: Option<&Name>,
        ty: &str,
        proof: &str,
    ) -> Result<(), ReplayError> {
        if ty.is_empty() || proof.is_empty() {
            return Err(ReplayError::InvalidProofState(
                "Empty have type or proof".to_string(),
            ));
        }
        let hyp_name = name
            .map(|n| n.to_string())
            .unwrap_or_else(|| "h".to_string());
        state.hypotheses.insert(hyp_name, ty.to_string());
        Ok(())
    }
    fn apply_calc(
        &self,
        state: &mut ReplayState,
        entries: &[CalcEntry],
    ) -> Result<(), ReplayError> {
        if entries.is_empty() {
            return Err(ReplayError::InvalidProofState("Empty calc".to_string()));
        }
        state.goal = format!("(goal after calc with {} steps)", entries.len());
        Ok(())
    }
    fn apply_simp(
        &self,
        state: &mut ReplayState,
        lemmas: &[String],
        use_default: bool,
    ) -> Result<(), ReplayError> {
        state.goal = format!(
            "(goal after simp with {} lemmas, default={})",
            lemmas.len(),
            use_default
        );
        Ok(())
    }
    fn apply_omega(&self, state: &mut ReplayState) -> Result<(), ReplayError> {
        state.complete = true;
        state.goal = "(proved by omega)".to_string();
        Ok(())
    }
    fn apply_trivial(&self, state: &mut ReplayState) -> Result<(), ReplayError> {
        state.complete = true;
        state.goal = "(proved by trivial)".to_string();
        Ok(())
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ProofReplayExtResult2600 {
    /// Operation completed successfully.
    Ok(String),
    /// Operation encountered an error.
    Err(String),
    /// Operation partially completed.
    Partial { done: usize, total: usize },
    /// Operation was skipped.
    Skipped,
}
impl ProofReplayExtResult2600 {
    #[allow(dead_code)]
    pub fn is_ok(&self) -> bool {
        matches!(self, ProofReplayExtResult2600::Ok(_))
    }
    #[allow(dead_code)]
    pub fn is_err(&self) -> bool {
        matches!(self, ProofReplayExtResult2600::Err(_))
    }
    #[allow(dead_code)]
    pub fn is_partial(&self) -> bool {
        matches!(self, ProofReplayExtResult2600::Partial { .. })
    }
    #[allow(dead_code)]
    pub fn is_skipped(&self) -> bool {
        matches!(self, ProofReplayExtResult2600::Skipped)
    }
    #[allow(dead_code)]
    pub fn ok_msg(&self) -> Option<&str> {
        if let ProofReplayExtResult2600::Ok(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn err_msg(&self) -> Option<&str> {
        if let ProofReplayExtResult2600::Err(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn progress(&self) -> f64 {
        match self {
            ProofReplayExtResult2600::Ok(_) => 1.0,
            ProofReplayExtResult2600::Err(_) => 0.0,
            ProofReplayExtResult2600::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
            ProofReplayExtResult2600::Skipped => 0.5,
        }
    }
}
#[allow(dead_code)]
pub struct ProofReplayExtDiff2600 {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
impl ProofReplayExtDiff2600 {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            added: Vec::new(),
            removed: Vec::new(),
            unchanged: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn add(&mut self, s: &str) {
        self.added.push(s.to_string());
    }
    #[allow(dead_code)]
    pub fn remove(&mut self, s: &str) {
        self.removed.push(s.to_string());
    }
    #[allow(dead_code)]
    pub fn keep(&mut self, s: &str) {
        self.unchanged.push(s.to_string());
    }
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty()
    }
    #[allow(dead_code)]
    pub fn total_changes(&self) -> usize {
        self.added.len() + self.removed.len()
    }
    #[allow(dead_code)]
    pub fn net_additions(&self) -> i64 {
        self.added.len() as i64 - self.removed.len() as i64
    }
    #[allow(dead_code)]
    pub fn summary(&self) -> String {
        format!(
            "+{} -{} =={}",
            self.added.len(),
            self.removed.len(),
            self.unchanged.len()
        )
    }
}
/// Error type for proof replay operations
#[derive(Clone, Debug)]
pub enum ReplayError {
    /// Invalid tactic structure
    InvalidStructure(String),
    /// Tactic application failed
    TacticFailed(String),
    /// Unknown hypothesis
    UnknownHypothesis(String),
    /// Type mismatch
    TypeMismatch(String),
    /// Serialization error
    SerializationError(String),
    /// Invalid proof state
    InvalidProofState(String),
    /// Goal mismatch
    GoalMismatch(String),
}
#[allow(dead_code)]
pub struct ProofReplayExtPipeline2600 {
    pub name: String,
    pub passes: Vec<ProofReplayExtPass2600>,
    pub run_count: usize,
}
impl ProofReplayExtPipeline2600 {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            passes: Vec::new(),
            run_count: 0,
        }
    }
    #[allow(dead_code)]
    pub fn add_pass(&mut self, pass: ProofReplayExtPass2600) {
        self.passes.push(pass);
    }
    #[allow(dead_code)]
    pub fn run_all(&mut self, input: &str) -> Vec<ProofReplayExtResult2600> {
        self.run_count += 1;
        self.passes
            .iter_mut()
            .filter(|p| p.enabled)
            .map(|p| p.run(input))
            .collect()
    }
    #[allow(dead_code)]
    pub fn num_passes(&self) -> usize {
        self.passes.len()
    }
    #[allow(dead_code)]
    pub fn num_enabled_passes(&self) -> usize {
        self.passes.iter().filter(|p| p.enabled).count()
    }
    #[allow(dead_code)]
    pub fn total_success_rate(&self) -> f64 {
        let total: usize = self.passes.iter().map(|p| p.total_runs).sum();
        let ok: usize = self.passes.iter().map(|p| p.successes).sum();
        if total == 0 {
            0.0
        } else {
            ok as f64 / total as f64
        }
    }
}
/// An analysis pass for ProofReplay.
#[allow(dead_code)]
pub struct ProofReplayAnalysisPass {
    pub name: String,
    pub enabled: bool,
    pub results: Vec<ProofReplayResult>,
    pub total_runs: usize,
}
#[allow(dead_code)]
impl ProofReplayAnalysisPass {
    pub fn new(name: &str) -> Self {
        ProofReplayAnalysisPass {
            name: name.to_string(),
            enabled: true,
            results: Vec::new(),
            total_runs: 0,
        }
    }
    pub fn run(&mut self, input: &str) -> ProofReplayResult {
        self.total_runs += 1;
        let result = if input.is_empty() {
            ProofReplayResult::Err("empty input".to_string())
        } else {
            ProofReplayResult::Ok(format!("processed: {}", input))
        };
        self.results.push(result.clone());
        result
    }
    pub fn success_count(&self) -> usize {
        self.results.iter().filter(|r| r.is_ok()).count()
    }
    pub fn error_count(&self) -> usize {
        self.results.iter().filter(|r| r.is_err()).count()
    }
    pub fn success_rate(&self) -> f64 {
        if self.total_runs == 0 {
            0.0
        } else {
            self.success_count() as f64 / self.total_runs as f64
        }
    }
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    pub fn clear_results(&mut self) {
        self.results.clear();
    }
}
#[allow(dead_code)]
pub struct ProofReplayExtDiag2600 {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
impl ProofReplayExtDiag2600 {
    #[allow(dead_code)]
    pub fn new(max_errors: usize) -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
            notes: Vec::new(),
            max_errors,
        }
    }
    #[allow(dead_code)]
    pub fn error(&mut self, msg: &str) {
        if self.errors.len() < self.max_errors {
            self.errors.push(msg.to_string());
        }
    }
    #[allow(dead_code)]
    pub fn warning(&mut self, msg: &str) {
        self.warnings.push(msg.to_string());
    }
    #[allow(dead_code)]
    pub fn note(&mut self, msg: &str) {
        self.notes.push(msg.to_string());
    }
    #[allow(dead_code)]
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
    #[allow(dead_code)]
    pub fn num_errors(&self) -> usize {
        self.errors.len()
    }
    #[allow(dead_code)]
    pub fn num_warnings(&self) -> usize {
        self.warnings.len()
    }
    #[allow(dead_code)]
    pub fn is_clean(&self) -> bool {
        self.errors.is_empty() && self.warnings.is_empty()
    }
    #[allow(dead_code)]
    pub fn at_error_limit(&self) -> bool {
        self.errors.len() >= self.max_errors
    }
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.errors.clear();
        self.warnings.clear();
        self.notes.clear();
    }
    #[allow(dead_code)]
    pub fn summary(&self) -> String {
        format!(
            "{} error(s), {} warning(s)",
            self.errors.len(),
            self.warnings.len()
        )
    }
}
/// A single step in a calc proof
#[derive(Clone, Debug)]
pub struct CalcEntry {
    /// Relation symbol (e.g., "=", "<", "≤")
    pub rel: String,
    /// Left-hand side expression
    pub lhs: String,
    /// Right-hand side expression
    pub rhs: String,
    /// Proof of this step
    pub proof: String,
}
/// A proof constraint asserting that a `ConstraintExpr` must hold.
#[derive(Clone, Debug)]
pub struct Constraint {
    /// The expression that must evaluate to true
    pub expr: ConstraintExpr,
    /// Optional human-readable label for debugging
    pub label: Option<String>,
}
impl Constraint {
    /// Create a new unlabelled constraint.
    pub fn new(expr: ConstraintExpr) -> Self {
        Constraint { expr, label: None }
    }
    /// Create a labelled constraint.
    pub fn labelled(expr: ConstraintExpr, label: impl Into<String>) -> Self {
        Constraint {
            expr,
            label: Some(label.into()),
        }
    }
}
#[allow(dead_code)]
pub struct ProofReplayExtPass2600 {
    pub name: String,
    pub total_runs: usize,
    pub successes: usize,
    pub errors: usize,
    pub enabled: bool,
    pub results: Vec<ProofReplayExtResult2600>,
}
impl ProofReplayExtPass2600 {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            total_runs: 0,
            successes: 0,
            errors: 0,
            enabled: true,
            results: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn run(&mut self, input: &str) -> ProofReplayExtResult2600 {
        if !self.enabled {
            return ProofReplayExtResult2600::Skipped;
        }
        self.total_runs += 1;
        let result = if input.is_empty() {
            self.errors += 1;
            ProofReplayExtResult2600::Err(format!("empty input in pass '{}'", self.name))
        } else {
            self.successes += 1;
            ProofReplayExtResult2600::Ok(format!(
                "processed {} chars in pass '{}'",
                input.len(),
                self.name
            ))
        };
        self.results.push(result.clone());
        result
    }
    #[allow(dead_code)]
    pub fn success_count(&self) -> usize {
        self.successes
    }
    #[allow(dead_code)]
    pub fn error_count(&self) -> usize {
        self.errors
    }
    #[allow(dead_code)]
    pub fn success_rate(&self) -> f64 {
        if self.total_runs == 0 {
            0.0
        } else {
            self.successes as f64 / self.total_runs as f64
        }
    }
    #[allow(dead_code)]
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    #[allow(dead_code)]
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    #[allow(dead_code)]
    pub fn clear_results(&mut self) {
        self.results.clear();
    }
}
/// Represents a single tactic step in a proof.
#[derive(Clone, Debug)]
pub enum ProofStep {
    /// Introduce a hypothesis
    Intro {
        /// Optional name for the introduced variable
        name: Option<Name>,
    },
    /// Apply a term to the goal
    Apply {
        /// Term to apply
        term: String,
    },
    /// Provide an exact proof term
    Exact {
        /// Proof term
        term: String,
    },
    /// Rewrite using an equality
    Rewrite {
        /// Rewrite lemma
        lemma: String,
        /// Optional location in term
        location: Option<String>,
    },
    /// Case analysis on a term
    Cases {
        /// Term to analyze
        term: String,
    },
    /// Proof by induction
    Induction {
        /// Term to induct on
        term: String,
    },
    /// Clear a hypothesis
    Clear {
        /// Hypothesis name
        name: Name,
    },
    /// Substitute an equality
    Subst {
        /// Variable to substitute
        var: Name,
    },
    /// Introduce a new hypothesis
    Have {
        /// Hypothesis name
        name: Option<Name>,
        /// Type of hypothesis
        ty: String,
        /// Proof of hypothesis
        proof: String,
    },
    /// Calc mode step
    Calc {
        /// Calculation entries
        entries: Vec<CalcEntry>,
    },
    /// Simp simplification
    Simp {
        /// Simp lemmas to use
        lemmas: Vec<String>,
        /// Whether to use default simp set
        use_default: bool,
    },
    /// Omega tactic
    Omega,
    /// Trivial tactic (reflexivity)
    Trivial,
    /// Sequence of steps
    Sequence {
        /// Steps to execute in sequence
        steps: Vec<ProofStep>,
    },
    /// First tactic to succeed
    First {
        /// Alternative tactics
        alternatives: Vec<ProofStep>,
    },
}
/// A proof script consisting of tactic steps
#[derive(Clone, Debug)]
pub struct ProofScript {
    /// The steps in the proof
    pub(super) steps: Vec<ProofStep>,
    /// Initial goal (as string representation)
    pub(super) goal: String,
    /// Initial hypotheses
    pub(super) hypotheses: Vec<(Name, String)>,
    /// New tactics introduced
    pub(super) new: Vec<ProofStep>,
}
impl ProofScript {
    /// Create a new proof script
    pub fn new(goal: String, hypotheses: Vec<(Name, String)>) -> Self {
        ProofScript {
            steps: Vec::new(),
            goal,
            hypotheses,
            new: Vec::new(),
        }
    }
    /// Add a step to the proof
    pub fn add_step(&mut self, step: ProofStep) {
        self.steps.push(step);
    }
    /// Add multiple steps
    pub fn add_steps(&mut self, steps: Vec<ProofStep>) {
        self.steps.extend(steps);
    }
    /// Get all steps
    pub fn steps(&self) -> &[ProofStep] {
        &self.steps
    }
    /// Get mutable steps
    pub fn steps_mut(&mut self) -> &mut Vec<ProofStep> {
        &mut self.steps
    }
    /// Get the goal
    pub fn goal(&self) -> &str {
        &self.goal
    }
    /// Get hypotheses
    pub fn hypotheses(&self) -> &[(Name, String)] {
        &self.hypotheses
    }
    /// Validate the structure of the proof
    pub fn validate_structure(&self) -> Result<(), ReplayError> {
        self.validate_steps(&self.steps)?;
        Ok(())
    }
    fn validate_steps(&self, steps: &[ProofStep]) -> Result<(), ReplayError> {
        for (idx, step) in steps.iter().enumerate() {
            match step {
                ProofStep::Sequence { steps } => {
                    self.validate_steps(steps)?;
                }
                ProofStep::First { alternatives } => {
                    if alternatives.is_empty() {
                        return Err(ReplayError::InvalidStructure(format!(
                            "First tactic at index {} has no alternatives",
                            idx
                        )));
                    }
                    for alt in alternatives {
                        if let ProofStep::Sequence { .. } = alt {}
                    }
                }
                ProofStep::Calc { entries } if entries.is_empty() => {
                    return Err(ReplayError::InvalidStructure(format!(
                        "Calc at index {} has no entries",
                        idx
                    )));
                }
                _ => {}
            }
        }
        Ok(())
    }
}
/// Binary serialization format for proofs
pub struct ProofSerializer;
impl ProofSerializer {
    /// Serialize a proof script to bytes
    pub fn serialize(script: &ProofScript) -> Result<Vec<u8>, ReplayError> {
        let mut buf = Vec::new();
        buf.write_all(b"PROOF").map_err(|e| {
            ReplayError::SerializationError(format!("Failed to write magic: {}", e))
        })?;
        buf.write_all(&[1u8]).map_err(|e| {
            ReplayError::SerializationError(format!("Failed to write version: {}", e))
        })?;
        let goal_bytes = script.goal.as_bytes();
        buf.write_all(&(goal_bytes.len() as u32).to_le_bytes())
            .map_err(|e| {
                ReplayError::SerializationError(format!("Failed to write goal length: {}", e))
            })?;
        buf.write_all(goal_bytes)
            .map_err(|e| ReplayError::SerializationError(format!("Failed to write goal: {}", e)))?;
        buf.write_all(&(script.hypotheses.len() as u32).to_le_bytes())
            .map_err(|e| {
                ReplayError::SerializationError(format!("Failed to write hypotheses count: {}", e))
            })?;
        for (name, ty) in &script.hypotheses {
            let name_str = name.to_string();
            let name_bytes = name_str.as_bytes();
            buf.write_all(&(name_bytes.len() as u32).to_le_bytes())
                .map_err(|e| {
                    ReplayError::SerializationError(format!(
                        "Failed to write hypothesis name length: {}",
                        e
                    ))
                })?;
            buf.write_all(name_bytes).map_err(|e| {
                ReplayError::SerializationError(format!("Failed to write hypothesis name: {}", e))
            })?;
            let ty_bytes = ty.as_bytes();
            buf.write_all(&(ty_bytes.len() as u32).to_le_bytes())
                .map_err(|e| {
                    ReplayError::SerializationError(format!("Failed to write type length: {}", e))
                })?;
            buf.write_all(ty_bytes).map_err(|e| {
                ReplayError::SerializationError(format!("Failed to write type: {}", e))
            })?;
        }
        buf.write_all(&(script.steps.len() as u32).to_le_bytes())
            .map_err(|e| {
                ReplayError::SerializationError(format!("Failed to write steps count: {}", e))
            })?;
        Ok(buf)
    }
    /// Deserialize a proof script from bytes
    pub fn deserialize(data: &[u8]) -> Result<ProofScript, ReplayError> {
        if data.len() < 6 {
            return Err(ReplayError::SerializationError(
                "Data too short for header".to_string(),
            ));
        }
        if &data[0..5] != b"PROOF" {
            return Err(ReplayError::SerializationError(
                "Invalid magic number".to_string(),
            ));
        }
        if data[5] != 1 {
            return Err(ReplayError::SerializationError(
                "Unsupported version".to_string(),
            ));
        }
        let mut offset = 6;
        if offset + 4 > data.len() {
            return Err(ReplayError::SerializationError(
                "Truncated goal length".to_string(),
            ));
        }
        let goal_len = u32::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
        ]) as usize;
        offset += 4;
        if offset + goal_len > data.len() {
            return Err(ReplayError::SerializationError(
                "Truncated goal".to_string(),
            ));
        }
        let goal = String::from_utf8(data[offset..offset + goal_len].to_vec()).map_err(|e| {
            ReplayError::SerializationError(format!("Invalid UTF-8 in goal: {}", e))
        })?;
        offset += goal_len;
        if offset + 4 > data.len() {
            return Err(ReplayError::SerializationError(
                "Truncated hypotheses count".to_string(),
            ));
        }
        let hyp_count = u32::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
        ]) as usize;
        offset += 4;
        let mut hypotheses = Vec::new();
        for _ in 0..hyp_count {
            if offset + 4 > data.len() {
                return Err(ReplayError::SerializationError(
                    "Truncated hypothesis name length".to_string(),
                ));
            }
            let name_len = u32::from_le_bytes([
                data[offset],
                data[offset + 1],
                data[offset + 2],
                data[offset + 3],
            ]) as usize;
            offset += 4;
            if offset + name_len > data.len() {
                return Err(ReplayError::SerializationError(
                    "Truncated hypothesis name".to_string(),
                ));
            }
            let name_str =
                String::from_utf8(data[offset..offset + name_len].to_vec()).map_err(|e| {
                    ReplayError::SerializationError(format!(
                        "Invalid UTF-8 in hypothesis name: {}",
                        e
                    ))
                })?;
            offset += name_len;
            if offset + 4 > data.len() {
                return Err(ReplayError::SerializationError(
                    "Truncated type length".to_string(),
                ));
            }
            let ty_len = u32::from_le_bytes([
                data[offset],
                data[offset + 1],
                data[offset + 2],
                data[offset + 3],
            ]) as usize;
            offset += 4;
            if offset + ty_len > data.len() {
                return Err(ReplayError::SerializationError(
                    "Truncated type".to_string(),
                ));
            }
            let ty = String::from_utf8(data[offset..offset + ty_len].to_vec()).map_err(|e| {
                ReplayError::SerializationError(format!("Invalid UTF-8 in type: {}", e))
            })?;
            offset += ty_len;
            hypotheses.push((Name::str(name_str), ty));
        }
        Ok(ProofScript::new(goal, hypotheses))
    }
}
/// A configuration store for ProofReplay.
#[allow(dead_code)]
pub struct ProofReplayConfig {
    pub values: std::collections::HashMap<String, ProofReplayConfigValue>,
    pub read_only: bool,
}
#[allow(dead_code)]
impl ProofReplayConfig {
    pub fn new() -> Self {
        ProofReplayConfig {
            values: std::collections::HashMap::new(),
            read_only: false,
        }
    }
    pub fn set(&mut self, key: &str, value: ProofReplayConfigValue) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    pub fn get(&self, key: &str) -> Option<&ProofReplayConfigValue> {
        self.values.get(key)
    }
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.get(key)?.as_bool()
    }
    pub fn get_int(&self, key: &str) -> Option<i64> {
        self.get(key)?.as_int()
    }
    pub fn get_str(&self, key: &str) -> Option<&str> {
        self.get(key)?.as_str()
    }
    pub fn set_bool(&mut self, key: &str, v: bool) -> bool {
        self.set(key, ProofReplayConfigValue::Bool(v))
    }
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, ProofReplayConfigValue::Int(v))
    }
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, ProofReplayConfigValue::Str(v.to_string()))
    }
    pub fn lock(&mut self) {
        self.read_only = true;
    }
    pub fn unlock(&mut self) {
        self.read_only = false;
    }
    pub fn size(&self) -> usize {
        self.values.len()
    }
    pub fn has(&self, key: &str) -> bool {
        self.values.contains_key(key)
    }
    pub fn remove(&mut self, key: &str) -> bool {
        self.values.remove(key).is_some()
    }
}
/// A diff for ProofReplay analysis results.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ProofReplayDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
#[allow(dead_code)]
impl ProofReplayDiff {
    pub fn new() -> Self {
        ProofReplayDiff {
            added: Vec::new(),
            removed: Vec::new(),
            unchanged: Vec::new(),
        }
    }
    pub fn add(&mut self, s: &str) {
        self.added.push(s.to_string());
    }
    pub fn remove(&mut self, s: &str) {
        self.removed.push(s.to_string());
    }
    pub fn keep(&mut self, s: &str) {
        self.unchanged.push(s.to_string());
    }
    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty()
    }
    pub fn total_changes(&self) -> usize {
        self.added.len() + self.removed.len()
    }
    pub fn net_additions(&self) -> i64 {
        self.added.len() as i64 - self.removed.len() as i64
    }
    pub fn summary(&self) -> String {
        format!(
            "+{} -{} =={}",
            self.added.len(),
            self.removed.len(),
            self.unchanged.len()
        )
    }
}
/// A diagnostic reporter for ProofReplay.
#[allow(dead_code)]
pub struct ProofReplayDiagnostics {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
#[allow(dead_code)]
impl ProofReplayDiagnostics {
    pub fn new(max_errors: usize) -> Self {
        ProofReplayDiagnostics {
            errors: Vec::new(),
            warnings: Vec::new(),
            notes: Vec::new(),
            max_errors,
        }
    }
    pub fn error(&mut self, msg: &str) {
        if self.errors.len() < self.max_errors {
            self.errors.push(msg.to_string());
        }
    }
    pub fn warning(&mut self, msg: &str) {
        self.warnings.push(msg.to_string());
    }
    pub fn note(&mut self, msg: &str) {
        self.notes.push(msg.to_string());
    }
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
    pub fn num_errors(&self) -> usize {
        self.errors.len()
    }
    pub fn num_warnings(&self) -> usize {
        self.warnings.len()
    }
    pub fn is_clean(&self) -> bool {
        self.errors.is_empty() && self.warnings.is_empty()
    }
    pub fn at_error_limit(&self) -> bool {
        self.errors.len() >= self.max_errors
    }
    pub fn clear(&mut self) {
        self.errors.clear();
        self.warnings.clear();
        self.notes.clear();
    }
    pub fn summary(&self) -> String {
        format!(
            "{} error(s), {} warning(s)",
            self.errors.len(),
            self.warnings.len()
        )
    }
}
/// State during proof replay
pub struct ReplayState {
    /// Current goal
    pub(super) goal: String,
    /// Current hypotheses
    pub(super) hypotheses: BTreeMap<String, String>,
    /// History of states
    pub(super) history: VecDeque<(String, BTreeMap<String, String>)>,
    /// Whether the proof is complete
    pub(super) complete: bool,
}
impl ReplayState {
    /// Create a new replay state
    pub fn new(goal: String, hypotheses: Vec<(Name, String)>) -> Self {
        let mut hyps = BTreeMap::new();
        for (name, ty) in hypotheses {
            hyps.insert(name.to_string(), ty);
        }
        ReplayState {
            goal,
            hypotheses: hyps,
            history: VecDeque::new(),
            complete: false,
        }
    }
    /// Get the current goal
    pub fn goal(&self) -> &str {
        &self.goal
    }
    /// Get current hypotheses
    pub fn hypotheses(&self) -> &BTreeMap<String, String> {
        &self.hypotheses
    }
    /// Check if proof is complete
    pub fn is_complete(&self) -> bool {
        self.complete
    }
    /// Save current state to history
    fn save_state(&mut self) {
        self.history
            .push_back((self.goal.clone(), self.hypotheses.clone()));
    }
    /// Restore previous state
    fn restore_state(&mut self) -> Result<(), ReplayError> {
        if let Some((goal, hyps)) = self.history.pop_back() {
            self.goal = goal;
            self.hypotheses = hyps;
            Ok(())
        } else {
            Err(ReplayError::InvalidProofState(
                "No previous state to restore".to_string(),
            ))
        }
    }
}
/// Compresses proof scripts by eliminating dead code
pub struct ProofCompressor;
impl ProofCompressor {
    /// Compress a proof script
    pub fn compress(script: &mut ProofScript) -> Result<(), ReplayError> {
        script.validate_structure()?;
        let steps = std::mem::take(&mut script.steps);
        let compressed = Self::compress_steps(&steps)?;
        script.steps = compressed;
        Ok(())
    }
    /// Eliminate dead steps using backward liveness analysis.
    ///
    /// A step is live if its output (variables it defines) is used by any
    /// subsequent live step or in the final proof term.  We approximate
    /// this by tracking which hypothesis names are *needed* (referenced in
    /// later goal strings or apply/exact terms) and mark a step live if it
    /// produces any needed name.
    pub fn eliminate_dead_steps(steps: &[ProofStep]) -> Result<Vec<ProofStep>, ReplayError> {
        let n = steps.len();
        let mut live = vec![false; n];
        let mut needed: HashSet<String> = HashSet::new();
        for i in (0..n).rev() {
            let step = &steps[i];
            let closes_goal = matches!(
                step,
                ProofStep::Trivial
                    | ProofStep::Omega
                    | ProofStep::Exact { .. }
                    | ProofStep::Calc { .. }
            );
            let names_used = step_uses(step);
            let names_defined = step_defines(step);
            let produces_needed = names_defined.iter().any(|n| needed.contains(n));
            let goal_modifier = matches!(
                step,
                ProofStep::Apply { .. }
                    | ProofStep::Rewrite { .. }
                    | ProofStep::Simp { .. }
                    | ProofStep::Cases { .. }
                    | ProofStep::Induction { .. }
                    | ProofStep::Subst { .. }
            );
            let is_live = closes_goal || goal_modifier || produces_needed;
            live[i] = is_live;
            if is_live {
                needed.extend(names_used);
            }
            for name in &names_defined {
                needed.remove(name);
            }
        }
        let live_steps = steps
            .iter()
            .enumerate()
            .filter(|(i, _)| live[*i])
            .map(|(_, s)| s.clone())
            .collect();
        Ok(live_steps)
    }
    /// Merge consecutive sequences
    pub fn merge_sequences(steps: &[ProofStep]) -> Vec<ProofStep> {
        let mut merged = Vec::new();
        let mut current_sequence = Vec::new();
        for step in steps {
            match step {
                ProofStep::Sequence { steps } => {
                    current_sequence.extend(steps.clone());
                }
                _ => {
                    if !current_sequence.is_empty() {
                        merged.push(ProofStep::Sequence {
                            steps: std::mem::take(&mut current_sequence),
                        });
                    }
                    merged.push(step.clone());
                }
            }
        }
        if !current_sequence.is_empty() {
            merged.push(ProofStep::Sequence {
                steps: current_sequence,
            });
        }
        merged
    }
    fn compress_steps(steps: &[ProofStep]) -> Result<Vec<ProofStep>, ReplayError> {
        let dead_eliminated = Self::eliminate_dead_steps(steps)?;
        Ok(Self::merge_sequences(&dead_eliminated))
    }
    /// Apply an assignment `id := val` to a list of constraints by substituting
    /// `id` with `val` throughout every constraint expression.
    ///
    /// Constraints that become trivially true (i.e. `Eq(x, x)` or `Lit("true")`)
    /// after substitution are removed.
    pub fn apply_assignment(
        id: &str,
        val: &ConstraintExpr,
        constraints: Vec<Constraint>,
    ) -> Vec<Constraint> {
        constraints
            .into_iter()
            .filter_map(|c| {
                let new_expr = c.expr.substitute(id, val);
                if is_trivially_true(&new_expr) {
                    None
                } else {
                    Some(Constraint {
                        expr: new_expr,
                        label: c.label,
                    })
                }
            })
            .collect()
    }
}
/// A symbolic expression used inside proof constraints.
///
/// Constraints record relationships between proof variables that must hold
/// for a compressed proof to remain valid.
#[derive(Clone, Debug, PartialEq)]
pub enum ConstraintExpr {
    /// A named variable (e.g. a metavariable or step output identifier)
    Var(String),
    /// A literal string value
    Lit(String),
    /// Function application: `f(args...)`
    App(Box<ConstraintExpr>, Vec<ConstraintExpr>),
    /// Equality: `lhs = rhs`
    Eq(Box<ConstraintExpr>, Box<ConstraintExpr>),
    /// Conjunction: `lhs ∧ rhs`
    And(Box<ConstraintExpr>, Box<ConstraintExpr>),
    /// Negation: `¬ expr`
    Not(Box<ConstraintExpr>),
}
impl ConstraintExpr {
    /// Substitute all occurrences of variable `id` with `val` throughout this expression.
    pub fn substitute(&self, id: &str, val: &ConstraintExpr) -> ConstraintExpr {
        match self {
            ConstraintExpr::Var(name) => {
                if name == id {
                    val.clone()
                } else {
                    self.clone()
                }
            }
            ConstraintExpr::Lit(_) => self.clone(),
            ConstraintExpr::App(f, args) => ConstraintExpr::App(
                Box::new(f.substitute(id, val)),
                args.iter().map(|a| a.substitute(id, val)).collect(),
            ),
            ConstraintExpr::Eq(lhs, rhs) => ConstraintExpr::Eq(
                Box::new(lhs.substitute(id, val)),
                Box::new(rhs.substitute(id, val)),
            ),
            ConstraintExpr::And(lhs, rhs) => ConstraintExpr::And(
                Box::new(lhs.substitute(id, val)),
                Box::new(rhs.substitute(id, val)),
            ),
            ConstraintExpr::Not(inner) => ConstraintExpr::Not(Box::new(inner.substitute(id, val))),
        }
    }
    /// Collect all variable names referenced in this expression.
    pub fn free_vars(&self) -> HashSet<String> {
        match self {
            ConstraintExpr::Var(name) => {
                let mut s = HashSet::new();
                s.insert(name.clone());
                s
            }
            ConstraintExpr::Lit(_) => HashSet::new(),
            ConstraintExpr::App(f, args) => {
                let mut vars = f.free_vars();
                for a in args {
                    vars.extend(a.free_vars());
                }
                vars
            }
            ConstraintExpr::Eq(lhs, rhs) | ConstraintExpr::And(lhs, rhs) => {
                let mut vars = lhs.free_vars();
                vars.extend(rhs.free_vars());
                vars
            }
            ConstraintExpr::Not(inner) => inner.free_vars(),
        }
    }
}
/// A result type for ProofReplay analysis.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum ProofReplayResult {
    Ok(String),
    Err(String),
    Partial { done: usize, total: usize },
    Skipped,
}
#[allow(dead_code)]
impl ProofReplayResult {
    pub fn is_ok(&self) -> bool {
        matches!(self, ProofReplayResult::Ok(_))
    }
    pub fn is_err(&self) -> bool {
        matches!(self, ProofReplayResult::Err(_))
    }
    pub fn is_partial(&self) -> bool {
        matches!(self, ProofReplayResult::Partial { .. })
    }
    pub fn is_skipped(&self) -> bool {
        matches!(self, ProofReplayResult::Skipped)
    }
    pub fn ok_msg(&self) -> Option<&str> {
        match self {
            ProofReplayResult::Ok(s) => Some(s),
            _ => None,
        }
    }
    pub fn err_msg(&self) -> Option<&str> {
        match self {
            ProofReplayResult::Err(s) => Some(s),
            _ => None,
        }
    }
    pub fn progress(&self) -> f64 {
        match self {
            ProofReplayResult::Ok(_) => 1.0,
            ProofReplayResult::Err(_) => 0.0,
            ProofReplayResult::Skipped => 0.0,
            ProofReplayResult::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
        }
    }
}
