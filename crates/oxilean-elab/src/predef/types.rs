//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Environment, Expr, Level, Name};
use std::collections::{BTreeMap, HashMap, HashSet};

/// A well-founded ordering used to justify termination.
///
/// Contains the relation (a binary relation on some type) and
/// a proof that it is well-founded.
#[derive(Clone, Debug)]
pub struct WellFoundedOrder {
    /// The well-founded relation, e.g., `Nat.lt`.
    ///
    /// Must be of type `A -> A -> Prop` for some type `A`.
    pub relation: Expr,
    /// Proof that the relation is well-founded.
    ///
    /// Must be of type `WellFounded rel` where `rel` is the relation.
    pub proof: Expr,
    /// The type that the relation operates on.
    pub domain_type: Expr,
    /// Optional measure function `f : Args -> A` that maps the function's
    /// arguments to the domain of the relation.
    pub measure: Option<Expr>,
    /// Optional motive `C : A -> Sort u` (the return type of the recursive function).
    ///
    /// When set, `build_wf_fix` uses it to construct accurate IH types.
    /// When `None`, the IH type falls back to `Prop` (Sort 0).
    pub motive: Option<Expr>,
    /// Whether this order was automatically inferred or user-supplied.
    pub is_auto: bool,
}
impl WellFoundedOrder {
    /// Create a new well-founded order.
    pub fn new(relation: Expr, proof: Expr, domain_type: Expr) -> Self {
        Self {
            relation,
            proof,
            domain_type,
            measure: None,
            motive: None,
            is_auto: false,
        }
    }
    /// Set the measure function.
    pub fn with_measure(mut self, measure: Expr) -> Self {
        self.measure = Some(measure);
        self
    }
    /// Set the return-type motive `C : A -> Sort u`.
    ///
    /// This allows `build_wf_fix` to produce an IH type `(y : A) -> r y x -> C y`
    /// rather than a placeholder.
    pub fn with_motive(mut self, motive: Expr) -> Self {
        self.motive = Some(motive);
        self
    }
    /// Mark as automatically inferred.
    pub fn with_auto(mut self, auto: bool) -> Self {
        self.is_auto = auto;
        self
    }
    /// Build the well-founded recursion application.
    ///
    /// Produces `WellFounded.fix rel_wf (fun x ih => body)` where `rel_wf` is
    /// the well-foundedness proof.
    pub fn build_wf_fix(&self, body: &Expr, param_name: &Name) -> Expr {
        let wf_fix = Expr::Const(Name::str("WellFounded").append_str("fix"), vec![]);
        let functional = Expr::Lam(
            BinderInfo::Default,
            param_name.clone(),
            Box::new(self.domain_type.clone()),
            Box::new(Expr::Lam(
                BinderInfo::Default,
                Name::str("ih"),
                Box::new(self.build_ih_type(param_name)),
                Box::new(body.clone()),
            )),
        );
        let app1 = Expr::App(Box::new(wf_fix), Box::new(self.proof.clone()));
        Expr::App(Box::new(app1), Box::new(functional))
    }
    /// Build the inductive hypothesis type for the well-founded recursion.
    ///
    /// Produces `(y : A) -> r y x -> C y` where:
    /// - `A` is `self.domain_type`
    /// - `r` is `self.relation`
    /// - `C` is `self.motive` (or `Prop` if not set)
    /// - `x` is `BVar(1)` (the recursive parameter, one binder up)
    fn build_ih_type(&self, _param_name: &Name) -> Expr {
        let y_name = Name::str("y");
        let y_var = Expr::BVar(0);
        let x_var = Expr::BVar(1);
        let rel_app = Expr::App(
            Box::new(Expr::App(
                Box::new(self.relation.clone()),
                Box::new(y_var.clone()),
            )),
            Box::new(x_var),
        );
        let c_y = match &self.motive {
            Some(motive) => Expr::App(Box::new(motive.clone()), Box::new(y_var)),
            None => Expr::Sort(Level::Zero),
        };
        Expr::Pi(
            BinderInfo::Default,
            y_name,
            Box::new(self.domain_type.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("_"),
                Box::new(rel_app),
                Box::new(c_y),
            )),
        )
    }
    /// Validate that the relation and proof have compatible types.
    pub fn validate(&self) -> Result<(), TerminationError> {
        match &self.relation {
            Expr::Const(_, _) | Expr::Lam(_, _, _, _) | Expr::App(_, _) => Ok(()),
            _ => Err(TerminationError::InvalidRelation(format!(
                "expected a function expression for the well-founded relation, got {:?}",
                self.relation
            ))),
        }
    }
}
/// Errors that can occur during termination checking.
#[derive(Clone, Debug)]
pub enum TerminationError {
    /// No structurally decreasing argument found.
    NoDecreasingArg(Name),
    /// The recursive call does not decrease the expected argument.
    CallNotDecreasing {
        /// The function making the recursive call.
        caller: Name,
        /// The function being called.
        callee: Name,
        /// Description of what went wrong.
        reason: String,
    },
    /// Failed to prove the well-founded relation decreases.
    WellFoundedFailure(String),
    /// Invalid well-founded relation (not a binary relation).
    InvalidRelation(String),
    /// Mutual recursion group has no decreasing argument on some cycle.
    MutualNoDecrease(Vec<Name>),
    /// Empty mutual group.
    EmptyMutualGroup,
    /// Nested recursion detected but not allowed.
    NestedRecursion(String),
    /// Recursion through a non-inductive type.
    NonInductiveRecursion(String),
    /// Maximum recursion depth exceeded during analysis.
    MaxDepthExceeded(usize),
    /// The definition uses an unsupported recursion pattern.
    UnsupportedPattern(String),
    /// Internal error.
    InternalError(String),
}
/// A single recursive call found during body analysis.
///
/// Records the call site location, which function is called, and
/// what arguments are passed.
#[derive(Clone, Debug)]
pub struct RecCall {
    /// The name of the function being called recursively.
    pub callee: Name,
    /// The arguments passed at this call site (in order).
    pub args: Vec<Expr>,
    /// The index of the structurally decreasing argument (if identified).
    pub decreasing_arg_idx: Option<usize>,
    /// Whether this call is in a pattern match branch.
    pub in_match_branch: bool,
    /// The constructor pattern that guards this call (if any).
    pub guard_ctor: Option<Name>,
    /// Depth of nesting (for nested recursion detection).
    pub nesting_depth: u32,
}
impl RecCall {
    /// Create a new recursive call record.
    pub fn new(callee: Name, args: Vec<Expr>) -> Self {
        Self {
            callee,
            args,
            decreasing_arg_idx: None,
            in_match_branch: false,
            guard_ctor: None,
            nesting_depth: 0,
        }
    }
    /// Mark this call as occurring in a match branch guarded by a constructor.
    pub fn with_guard(mut self, ctor: Name) -> Self {
        self.in_match_branch = true;
        self.guard_ctor = Some(ctor);
        self
    }
    /// Set the decreasing argument index.
    pub fn with_decreasing_arg(mut self, idx: usize) -> Self {
        self.decreasing_arg_idx = Some(idx);
        self
    }
    /// Set the nesting depth.
    pub fn with_nesting_depth(mut self, depth: u32) -> Self {
        self.nesting_depth = depth;
        self
    }
    /// Check if the call has an identified decreasing argument.
    pub fn has_decreasing_arg(&self) -> bool {
        self.decreasing_arg_idx.is_some()
    }
}
/// A proof obligation generated by the termination checker.
///
/// When using well-founded recursion, the user must prove that
/// each recursive call's argument is smaller under the chosen relation.
#[derive(Clone, Debug)]
pub struct ProofObligation {
    /// A human-readable description of what needs to be proved.
    pub description: String,
    /// The proposition to prove (as an expression).
    pub goal: Expr,
    /// The local context (hypotheses) available for the proof.
    pub context: Vec<(Name, Expr)>,
    /// The source location (byte range) associated with this obligation.
    pub source_range: Option<(usize, usize)>,
    /// Whether this obligation was discharged automatically.
    pub auto_discharged: bool,
}
impl ProofObligation {
    /// Create a new proof obligation.
    pub fn new(description: String, goal: Expr) -> Self {
        Self {
            description,
            goal,
            context: Vec::new(),
            source_range: None,
            auto_discharged: false,
        }
    }
    /// Add a hypothesis to the obligation context.
    pub fn with_hypothesis(mut self, name: Name, ty: Expr) -> Self {
        self.context.push((name, ty));
        self
    }
    /// Set the source range.
    pub fn with_source_range(mut self, start: usize, end: usize) -> Self {
        self.source_range = Some((start, end));
        self
    }
    /// Mark as automatically discharged.
    pub fn mark_discharged(mut self) -> Self {
        self.auto_discharged = true;
        self
    }
}
/// Classification of recursion used by a definition.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RecursionKind {
    /// Structural recursion on the named parameter.
    ///
    /// The parameter decreases structurally at every recursive call site,
    /// which guarantees termination by the well-ordering of the inductive type.
    Structural(Name),
    /// Well-founded recursion using a given relation and measure.
    ///
    /// The user supplies a well-founded relation (e.g., `<` on `Nat`) and
    /// optionally a measure function mapping arguments to the relation's domain.
    WellFounded {
        /// The well-founded relation expression (e.g., `Nat.lt`).
        rel: Expr,
        /// Optional measure function mapping arguments to the relation domain.
        measure: Option<Expr>,
    },
    /// Mutual recursion involving the listed function names.
    ///
    /// All functions in the group must collectively terminate, verified
    /// through a decrease matrix.
    Mutual(Vec<Name>),
    /// The definition is not recursive at all.
    NonRecursive,
}
/// The result of a termination check.
#[derive(Clone, Debug)]
pub struct TerminationResult {
    /// The kind of recursion detected.
    pub kind: RecursionKind,
    /// The structural recursion parameter (if structural).
    pub structural_param: Option<StructuralRecParam>,
    /// The well-founded order (if well-founded).
    pub wf_order: Option<WellFoundedOrder>,
    /// All recursive calls found in the body.
    pub calls: Vec<RecCall>,
    /// Proof obligations generated (if requested).
    pub proof_obligations: Vec<ProofObligation>,
    /// Whether the definition is safe to add to the environment.
    pub is_safe: bool,
    /// The resulting kernel term (with fix combinator applied).
    pub result_term: Option<Expr>,
}
impl TerminationResult {
    /// Create a result for a non-recursive definition.
    pub fn non_recursive() -> Self {
        Self {
            kind: RecursionKind::NonRecursive,
            structural_param: None,
            wf_order: None,
            calls: Vec::new(),
            proof_obligations: Vec::new(),
            is_safe: true,
            result_term: None,
        }
    }
    /// Create a result for a structurally recursive definition.
    pub fn structural(param: StructuralRecParam, calls: Vec<RecCall>) -> Self {
        Self {
            kind: RecursionKind::Structural(param.param_name.clone()),
            structural_param: Some(param),
            wf_order: None,
            calls,
            proof_obligations: Vec::new(),
            is_safe: true,
            result_term: None,
        }
    }
    /// Create a result for well-founded recursion.
    pub fn well_founded(order: WellFoundedOrder, calls: Vec<RecCall>) -> Self {
        Self {
            kind: RecursionKind::WellFounded {
                rel: order.relation.clone(),
                measure: order.measure.clone(),
            },
            structural_param: None,
            wf_order: Some(order),
            calls,
            proof_obligations: Vec::new(),
            is_safe: true,
            result_term: None,
        }
    }
    /// Create a result for mutual recursion.
    pub fn mutual(names: Vec<Name>, calls: Vec<RecCall>) -> Self {
        Self {
            kind: RecursionKind::Mutual(names),
            structural_param: None,
            wf_order: None,
            calls,
            proof_obligations: Vec::new(),
            is_safe: true,
            result_term: None,
        }
    }
    /// Set the result term.
    pub fn with_result_term(mut self, term: Expr) -> Self {
        self.result_term = Some(term);
        self
    }
    /// Add proof obligations.
    pub fn with_obligations(mut self, obligations: Vec<ProofObligation>) -> Self {
        self.proof_obligations = obligations;
        self
    }
}
/// The main termination checker.
///
/// Given a function name, its parameters, body, and the environment,
/// determines whether the function terminates and how.
pub struct TerminationChecker<'env> {
    /// The global environment for looking up inductive types and definitions.
    #[allow(dead_code)]
    env: &'env Environment,
    /// Configuration options.
    config: PreDefConfig,
    /// Cache of known inductive type names.
    inductive_types: HashSet<Name>,
    /// Current analysis depth (for cycle detection).
    current_depth: usize,
    /// Collected proof obligations.
    obligations: Vec<ProofObligation>,
}
impl<'env> TerminationChecker<'env> {
    /// Create a new termination checker.
    pub fn new(env: &'env Environment, config: PreDefConfig) -> Self {
        Self {
            env,
            config,
            inductive_types: Self::collect_inductive_types(env),
            current_depth: 0,
            obligations: Vec::new(),
        }
    }
    /// Create with default configuration.
    pub fn with_defaults(env: &'env Environment) -> Self {
        Self::new(env, PreDefConfig::new())
    }
    /// Collect names of all inductive types in the environment.
    fn collect_inductive_types(env: &Environment) -> HashSet<Name> {
        let mut result = HashSet::new();
        result.insert(Name::str("Nat"));
        result.insert(Name::str("Bool"));
        result.insert(Name::str("List"));
        result.insert(Name::str("Option"));
        result.insert(Name::str("Sum"));
        result.insert(Name::str("Prod"));
        result.insert(Name::str("Fin"));
        result.insert(Name::str("Vector"));
        result.insert(Name::str("Tree"));
        result.insert(Name::str("Unit"));
        result.insert(Name::str("Empty"));
        result.insert(Name::str("Sigma"));
        for name in env.constant_names() {
            if env.is_inductive(name) {
                result.insert(name.clone());
            }
        }
        result
    }
    /// Check termination of a single recursive definition.
    ///
    /// Returns a `TerminationResult` indicating how the definition terminates
    /// (structural, well-founded, or non-recursive), or an error if termination
    /// cannot be established.
    pub fn check(
        &mut self,
        name: &Name,
        params: &[(Name, Expr)],
        body: &Expr,
        _env: &Environment,
    ) -> Result<TerminationResult, TerminationError> {
        if self.current_depth > self.config.max_depth {
            return Err(TerminationError::MaxDepthExceeded(self.config.max_depth));
        }
        self.current_depth += 1;
        let fn_names: HashSet<Name> = [name.clone()].into_iter().collect();
        let calls = find_recursive_calls(body, &fn_names);
        if calls.is_empty() {
            self.current_depth -= 1;
            return Ok(TerminationResult::non_recursive());
        }
        match self.try_structural_recursion(name, params, &calls) {
            Ok(result) => {
                self.current_depth -= 1;
                return Ok(result);
            }
            Err(_) => {}
        }
        if self.config.auto_wf {
            match self.try_well_founded_recursion(name, params, body, &calls) {
                Ok(result) => {
                    self.current_depth -= 1;
                    return Ok(result);
                }
                Err(_) => {}
            }
        }
        self.current_depth -= 1;
        Err(TerminationError::NoDecreasingArg(name.clone()))
    }
    /// Try to establish structural recursion.
    ///
    /// Checks each parameter to see if it decreases structurally at
    /// every recursive call site.
    fn try_structural_recursion(
        &self,
        name: &Name,
        params: &[(Name, Expr)],
        calls: &[RecCall],
    ) -> Result<TerminationResult, TerminationError> {
        for (idx, (param_name, param_ty)) in params.iter().enumerate() {
            let inductive_name = match self.get_inductive_type_name(param_ty) {
                Some(name) => name,
                None => continue,
            };
            let all_decrease = calls.iter().all(|call| {
                if call.callee != *name {
                    return true;
                }
                if idx < call.args.len() {
                    check_structural_decrease(param_name, &call.args[idx])
                } else {
                    false
                }
            });
            if all_decrease {
                let param = StructuralRecParam::new(param_name.clone(), idx, inductive_name);
                return Ok(TerminationResult::structural(param, calls.to_vec()));
            }
        }
        Err(TerminationError::NoDecreasingArg(name.clone()))
    }
    /// Try to establish well-founded recursion.
    ///
    /// Attempts to find a well-founded relation that decreases at every
    /// recursive call site. For `Nat`, this is `Nat.lt`.
    fn try_well_founded_recursion(
        &mut self,
        name: &Name,
        params: &[(Name, Expr)],
        _body: &Expr,
        calls: &[RecCall],
    ) -> Result<TerminationResult, TerminationError> {
        for (idx, (param_name, param_ty)) in params.iter().enumerate() {
            if self.is_nat_type(param_ty) {
                let nat_lt = Expr::Const(Name::str("Nat").append_str("lt"), vec![]);
                let nat_lt_wf =
                    Expr::Const(Name::str("Nat").append_str("lt").append_str("wf"), vec![]);
                let nat_type = Expr::Const(Name::str("Nat"), vec![]);
                let order = WellFoundedOrder::new(nat_lt, nat_lt_wf, nat_type).with_auto(true);
                if self.config.generate_proof_obligations {
                    for call in calls {
                        if call.callee == *name && idx < call.args.len() {
                            let obligation =
                                self.build_decrease_obligation(param_name, &call.args[idx], &order);
                            self.obligations.push(obligation);
                        }
                    }
                }
                return Ok(TerminationResult::well_founded(order, calls.to_vec())
                    .with_obligations(self.obligations.clone()));
            }
        }
        Err(TerminationError::WellFoundedFailure(format!(
            "no suitable well-founded relation found for '{}'",
            name
        )))
    }
    /// Check if an expression is the Nat type.
    fn is_nat_type(&self, expr: &Expr) -> bool {
        matches!(expr, Expr::Const(name, _) if * name == Name::str("Nat"))
    }
    /// Extract the inductive type name from a type expression.
    fn get_inductive_type_name(&self, ty: &Expr) -> Option<Name> {
        match ty {
            Expr::Const(name, _) => {
                if self.inductive_types.contains(name) {
                    Some(name.clone())
                } else {
                    None
                }
            }
            Expr::App(func, _) => self.get_inductive_type_name(func),
            _ => None,
        }
    }
    /// Build a proof obligation that a recursive argument is smaller.
    fn build_decrease_obligation(
        &self,
        param_name: &Name,
        arg: &Expr,
        order: &WellFoundedOrder,
    ) -> ProofObligation {
        let param_ref = Expr::Const(param_name.clone(), vec![]);
        let goal = Expr::App(
            Box::new(Expr::App(
                Box::new(order.relation.clone()),
                Box::new(arg.clone()),
            )),
            Box::new(param_ref),
        );
        ProofObligation::new(
            format!(
                "show that recursive argument is smaller than '{}' under {:?}",
                param_name, order.relation
            ),
            goal,
        )
        .with_hypothesis(param_name.clone(), order.domain_type.clone())
    }
    /// Get collected proof obligations.
    pub fn obligations(&self) -> &[ProofObligation] {
        &self.obligations
    }
    /// Reset the checker state for reuse.
    pub fn reset(&mut self) {
        self.current_depth = 0;
        self.obligations.clear();
    }
}
/// A group of mutually recursive definitions to be checked together.
///
/// Contains the function names, their bodies, and a decrease matrix
/// that tracks which arguments decrease at each call site.
#[derive(Clone, Debug)]
pub struct MutualRecGroup {
    /// Names of all functions in the mutual group.
    pub names: Vec<Name>,
    /// Bodies of the functions (in the same order as names).
    pub bodies: Vec<Expr>,
    /// Types of the functions.
    pub types: Vec<Expr>,
    /// Parameter lists for each function.
    pub params: Vec<Vec<(Name, Expr)>>,
    /// Decrease matrix: for each pair (caller, callee), records
    /// which argument positions decrease.
    ///
    /// `decrease_matrix[(caller_idx, callee_idx)]` is a vector of
    /// `ArgDecrease` entries, one per argument of the callee.
    pub decrease_matrix: BTreeMap<(usize, usize), Vec<ArgDecrease>>,
    /// Whether the group has been validated.
    pub validated: bool,
}
impl MutualRecGroup {
    /// Create a new mutual recursion group.
    pub fn new() -> Self {
        Self {
            names: Vec::new(),
            bodies: Vec::new(),
            types: Vec::new(),
            params: Vec::new(),
            decrease_matrix: BTreeMap::new(),
            validated: false,
        }
    }
    /// Add a function to the mutual group.
    pub fn add_function(&mut self, name: Name, ty: Expr, body: Expr, params: Vec<(Name, Expr)>) {
        self.names.push(name);
        self.types.push(ty);
        self.bodies.push(body);
        self.params.push(params);
    }
    /// Get the index of a function by name.
    pub fn index_of(&self, name: &Name) -> Option<usize> {
        self.names.iter().position(|n| n == name)
    }
    /// Number of functions in the group.
    pub fn size(&self) -> usize {
        self.names.len()
    }
    /// Record a decrease entry in the matrix.
    pub fn record_decrease(
        &mut self,
        caller_idx: usize,
        callee_idx: usize,
        arg_decreases: Vec<ArgDecrease>,
    ) {
        self.decrease_matrix
            .insert((caller_idx, callee_idx), arg_decreases);
    }
    /// Validate mutual termination via the decrease matrix.
    ///
    /// For each cycle in the call graph, there must be at least one edge
    /// where an argument strictly decreases. This is checked by examining
    /// the decrease matrix.
    pub fn validate_mutual_termination(&mut self) -> Result<(), TerminationError> {
        let n = self.names.len();
        if n == 0 {
            return Err(TerminationError::EmptyMutualGroup);
        }
        for i in 0..n {
            if let Some(decreases) = self.decrease_matrix.get(&(i, i)) {
                let has_decrease = decreases.contains(&ArgDecrease::Decreasing);
                if !has_decrease && !decreases.is_empty() {
                    return Err(TerminationError::NoDecreasingArg(self.names[i].clone()));
                }
            }
        }
        let sccs = self.find_sccs();
        for scc in &sccs {
            if scc.len() > 1 {
                self.validate_scc(scc)?;
            }
        }
        self.validated = true;
        Ok(())
    }
    /// Find strongly connected components in the call graph.
    fn find_sccs(&self) -> Vec<Vec<usize>> {
        let n = self.names.len();
        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
        for &(caller, callee) in self.decrease_matrix.keys() {
            if !adj[caller].contains(&callee) {
                adj[caller].push(callee);
            }
        }
        let mut index_counter: usize = 0;
        let mut stack: Vec<usize> = Vec::new();
        let mut on_stack = vec![false; n];
        let mut indices: Vec<Option<usize>> = vec![None; n];
        let mut lowlinks = vec![0usize; n];
        let mut result: Vec<Vec<usize>> = Vec::new();
        for v in 0..n {
            if indices[v].is_none() {
                Self::tarjan_dfs(
                    v,
                    &adj,
                    &mut index_counter,
                    &mut stack,
                    &mut on_stack,
                    &mut indices,
                    &mut lowlinks,
                    &mut result,
                );
            }
        }
        result
    }
    /// Tarjan DFS helper.
    #[allow(clippy::too_many_arguments)]
    fn tarjan_dfs(
        v: usize,
        adj: &[Vec<usize>],
        index_counter: &mut usize,
        stack: &mut Vec<usize>,
        on_stack: &mut Vec<bool>,
        indices: &mut Vec<Option<usize>>,
        lowlinks: &mut Vec<usize>,
        result: &mut Vec<Vec<usize>>,
    ) {
        indices[v] = Some(*index_counter);
        lowlinks[v] = *index_counter;
        *index_counter += 1;
        stack.push(v);
        on_stack[v] = true;
        for &w in &adj[v] {
            if indices[w].is_none() {
                Self::tarjan_dfs(
                    w,
                    adj,
                    index_counter,
                    stack,
                    on_stack,
                    indices,
                    lowlinks,
                    result,
                );
                lowlinks[v] = lowlinks[v].min(lowlinks[w]);
            } else if on_stack[w] {
                // Safety: on_stack[w] is true, so w was visited and indices[w] is Some
                lowlinks[v] =
                    lowlinks[v].min(indices[w].expect("on-stack node must have an index"));
            }
        }
        // Safety: indices[v] was set to Some at the start of this function
        if lowlinks[v] == indices[v].expect("current node must have an index") {
            let mut component = Vec::new();
            loop {
                // Safety: v is on the stack; loop terminates when w == v
                let w = stack.pop().expect("stack must contain current node v");
                on_stack[w] = false;
                component.push(w);
                if w == v {
                    break;
                }
            }
            result.push(component);
        }
    }
    /// Validate that an SCC has a decreasing argument on some cycle edge.
    fn validate_scc(&self, scc: &[usize]) -> Result<(), TerminationError> {
        let scc_set: HashSet<usize> = scc.iter().cloned().collect();
        let mut found_decrease = false;
        for &i in scc {
            for &j in scc {
                if let Some(decreases) = self.decrease_matrix.get(&(i, j)) {
                    if decreases.contains(&ArgDecrease::Decreasing) {
                        found_decrease = true;
                        break;
                    }
                }
            }
            if found_decrease {
                break;
            }
        }
        if !found_decrease && !scc_set.is_empty() {
            let scc_names: Vec<Name> = scc.iter().map(|&i| self.names[i].clone()).collect();
            return Err(TerminationError::MutualNoDecrease(scc_names));
        }
        Ok(())
    }
    /// Build the decrease matrix by analyzing all function bodies.
    pub fn build_decrease_matrix(&mut self, env: &Environment) {
        let names_set: HashSet<Name> = self.names.iter().cloned().collect();
        let n = self.names.len();
        for caller_idx in 0..n {
            let body = &self.bodies[caller_idx].clone();
            let calls = find_recursive_calls(body, &names_set);
            for call in &calls {
                if let Some(callee_idx) = self.index_of(&call.callee) {
                    let callee_params = &self.params[callee_idx];
                    let caller_params = &self.params[caller_idx];
                    let mut arg_decreases = Vec::new();
                    for (arg_pos, arg_expr) in call.args.iter().enumerate() {
                        if arg_pos < callee_params.len() {
                            let decrease = classify_argument_decrease(arg_expr, caller_params, env);
                            arg_decreases.push(decrease);
                        }
                    }
                    while arg_decreases.len() < callee_params.len() {
                        arg_decreases.push(ArgDecrease::Missing);
                    }
                    self.record_decrease(caller_idx, callee_idx, arg_decreases);
                }
            }
        }
    }
    /// Pretty-print the decrease matrix for debugging.
    pub fn format_matrix(&self) -> String {
        let mut out = String::new();
        out.push_str("Decrease Matrix:\n");
        for ((caller, callee), decreases) in &self.decrease_matrix {
            let caller_name = &self.names[*caller];
            let callee_name = &self.names[*callee];
            out.push_str(&format!("  {} -> {} : [", caller_name, callee_name));
            for (i, d) in decreases.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                out.push_str(&format!("{}", d));
            }
            out.push_str("]\n");
        }
        out
    }
}
/// Orchestrates pre-definition analysis for a function declaration.
///
/// This is the main entry point that coordinates termination checking,
/// structural recursion detection, and fix term construction.
pub struct PreDefAnalyzer<'env> {
    /// The termination checker.
    checker: TerminationChecker<'env>,
    /// The environment.
    env: &'env Environment,
    /// Results of analysis, keyed by function name.
    results: HashMap<Name, TerminationResult>,
}
impl<'env> PreDefAnalyzer<'env> {
    /// Create a new pre-definition analyzer.
    pub fn new(env: &'env Environment) -> Self {
        Self {
            checker: TerminationChecker::with_defaults(env),
            env,
            results: HashMap::new(),
        }
    }
    /// Create with custom configuration.
    pub fn with_config(env: &'env Environment, config: PreDefConfig) -> Self {
        Self {
            checker: TerminationChecker::new(env, config),
            env,
            results: HashMap::new(),
        }
    }
    /// Analyze a single definition.
    pub fn analyze(
        &mut self,
        name: &Name,
        params: &[(Name, Expr)],
        body: &Expr,
        ret_type: &Expr,
    ) -> Result<TerminationResult, TerminationError> {
        let mut result = self.checker.check(name, params, body, self.env)?;
        if let Some(ref param) = result.structural_param {
            let fix_term = build_fix_term(name, params, body, param.param_idx, ret_type);
            result = result.with_result_term(fix_term);
        }
        self.results.insert(name.clone(), result.clone());
        Ok(result)
    }
    /// Analyze a mutual recursion group.
    pub fn analyze_mutual(
        &mut self,
        group: &mut MutualRecGroup,
    ) -> Result<TerminationResult, TerminationError> {
        group.build_decrease_matrix(self.env);
        group.validate_mutual_termination()?;
        let calls_all: Vec<RecCall> = {
            let fn_names: HashSet<Name> = group.names.iter().cloned().collect();
            let mut all_calls = Vec::new();
            for body in &group.bodies {
                let calls = find_recursive_calls(body, &fn_names);
                all_calls.extend(calls);
            }
            all_calls
        };
        let result = TerminationResult::mutual(group.names.clone(), calls_all);
        for name in &group.names {
            self.results.insert(name.clone(), result.clone());
        }
        Ok(result)
    }
    /// Get the analysis result for a function.
    pub fn get_result(&self, name: &Name) -> Option<&TerminationResult> {
        self.results.get(name)
    }
    /// Get all results.
    pub fn all_results(&self) -> &HashMap<Name, TerminationResult> {
        &self.results
    }
    /// Reset the analyzer for reuse.
    pub fn reset(&mut self) {
        self.checker.reset();
        self.results.clear();
    }
}
/// How an argument changes from caller to callee in a mutual recursive call.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ArgDecrease {
    /// The argument is structurally smaller.
    Decreasing,
    /// The argument is the same (equal).
    Equal,
    /// The argument may be larger or unrelated.
    Unknown,
    /// The argument is not passed at all (partial application).
    Missing,
}
/// Tracks the subterm relation for an inductive type.
///
/// Given an inductive type like `Nat` with constructors `zero` and `succ n`,
/// this records that `n` is a subterm of `succ n`.
#[derive(Clone, Debug)]
pub struct SubtermRelation {
    /// The inductive type.
    pub inductive_type: Name,
    /// Map from constructor name to the indices of recursive arguments.
    pub recursive_args: HashMap<Name, Vec<usize>>,
    /// Map from constructor name to total number of arguments.
    pub ctor_arities: HashMap<Name, usize>,
}
impl SubtermRelation {
    /// Create a new subterm relation for the given inductive type.
    pub fn new(inductive_type: Name) -> Self {
        Self {
            inductive_type,
            recursive_args: HashMap::new(),
            ctor_arities: HashMap::new(),
        }
    }
    /// Register a constructor with its recursive argument positions.
    pub fn add_constructor(
        &mut self,
        ctor_name: Name,
        arity: usize,
        recursive_positions: Vec<usize>,
    ) {
        self.ctor_arities.insert(ctor_name.clone(), arity);
        self.recursive_args.insert(ctor_name, recursive_positions);
    }
    /// Check if a given constructor argument position is a recursive subterm.
    pub fn is_recursive_arg(&self, ctor: &Name, arg_idx: usize) -> bool {
        self.recursive_args
            .get(ctor)
            .map(|args| args.contains(&arg_idx))
            .unwrap_or(false)
    }
    /// Get all recursive argument positions for a constructor.
    pub fn get_recursive_args(&self, ctor: &Name) -> &[usize] {
        self.recursive_args
            .get(ctor)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }
    /// Build the standard subterm relations for well-known types.
    pub fn build_standard() -> Vec<Self> {
        let mut relations = Vec::new();
        let mut nat = SubtermRelation::new(Name::str("Nat"));
        nat.add_constructor(Name::str("Nat.zero"), 0, vec![]);
        nat.add_constructor(Name::str("Nat.succ"), 1, vec![0]);
        relations.push(nat);
        let mut list = SubtermRelation::new(Name::str("List"));
        list.add_constructor(Name::str("List.nil"), 0, vec![]);
        list.add_constructor(Name::str("List.cons"), 2, vec![1]);
        relations.push(list);
        let mut bool_rel = SubtermRelation::new(Name::str("Bool"));
        bool_rel.add_constructor(Name::str("Bool.true"), 0, vec![]);
        bool_rel.add_constructor(Name::str("Bool.false"), 0, vec![]);
        relations.push(bool_rel);
        let mut option = SubtermRelation::new(Name::str("Option"));
        option.add_constructor(Name::str("Option.none"), 0, vec![]);
        option.add_constructor(Name::str("Option.some"), 1, vec![]);
        relations.push(option);
        relations
    }
}
/// A lightweight detector that quickly determines if a definition is recursive.
///
/// Unlike the full `TerminationChecker`, this only checks for the presence
/// of recursive calls without verifying termination.
pub struct RecursionDetector;
impl RecursionDetector {
    /// Check if a body contains any recursive calls to the given function name.
    pub fn is_recursive(name: &Name, body: &Expr) -> bool {
        Self::contains_call(body, name)
    }
    /// Check if a body contains calls to any function in the given set.
    pub fn is_mutually_recursive(names: &HashSet<Name>, body: &Expr) -> bool {
        Self::contains_any_call(body, names)
    }
    /// Count the number of recursive calls.
    pub fn count_calls(name: &Name, body: &Expr) -> usize {
        let names: HashSet<Name> = [name.clone()].into_iter().collect();
        find_recursive_calls(body, &names).len()
    }
    fn contains_call(expr: &Expr, name: &Name) -> bool {
        match expr {
            Expr::Const(n, _) => n == name,
            Expr::App(func, arg) => {
                Self::contains_call(func, name) || Self::contains_call(arg, name)
            }
            Expr::Lam(_, _, ty, body) => {
                Self::contains_call(ty, name) || Self::contains_call(body, name)
            }
            Expr::Pi(_, _, ty, body) => {
                Self::contains_call(ty, name) || Self::contains_call(body, name)
            }
            Expr::Let(_, ty, val, body) => {
                Self::contains_call(ty, name)
                    || Self::contains_call(val, name)
                    || Self::contains_call(body, name)
            }
            Expr::Proj(_, _, base) => Self::contains_call(base, name),
            _ => false,
        }
    }
    fn contains_any_call(expr: &Expr, names: &HashSet<Name>) -> bool {
        match expr {
            Expr::Const(n, _) => names.contains(n),
            Expr::App(func, arg) => {
                Self::contains_any_call(func, names) || Self::contains_any_call(arg, names)
            }
            Expr::Lam(_, _, ty, body) => {
                Self::contains_any_call(ty, names) || Self::contains_any_call(body, names)
            }
            Expr::Pi(_, _, ty, body) => {
                Self::contains_any_call(ty, names) || Self::contains_any_call(body, names)
            }
            Expr::Let(_, ty, val, body) => {
                Self::contains_any_call(ty, names)
                    || Self::contains_any_call(val, names)
                    || Self::contains_any_call(body, names)
            }
            Expr::Proj(_, _, base) => Self::contains_any_call(base, names),
            _ => false,
        }
    }
}
/// Information about a structurally recursive parameter.
///
/// Identifies which parameter of a function decreases structurally
/// at every recursive call site, along with the inductive type
/// that establishes the well-ordering.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StructuralRecParam {
    /// Name of the decreasing parameter.
    pub param_name: Name,
    /// Index of the parameter in the function's parameter list (0-based).
    pub param_idx: usize,
    /// The inductive type of the decreasing parameter (e.g., `Nat`, `List`).
    pub inductive_type: Name,
    /// Universe levels of the inductive type.
    pub univ_levels: Vec<Level>,
    /// Number of type-level parameters of the inductive type.
    pub num_type_params: usize,
    /// Whether the parameter is a direct inductive argument or nested.
    pub is_direct: bool,
}
impl StructuralRecParam {
    /// Create a new structural recursion parameter descriptor.
    pub fn new(param_name: Name, param_idx: usize, inductive_type: Name) -> Self {
        Self {
            param_name,
            param_idx,
            inductive_type,
            univ_levels: Vec::new(),
            num_type_params: 0,
            is_direct: true,
        }
    }
    /// Set universe levels for the inductive type.
    pub fn with_univ_levels(mut self, levels: Vec<Level>) -> Self {
        self.univ_levels = levels;
        self
    }
    /// Set the number of type parameters.
    pub fn with_num_type_params(mut self, n: usize) -> Self {
        self.num_type_params = n;
        self
    }
    /// Mark whether this is a direct recursive parameter.
    pub fn with_direct(mut self, direct: bool) -> Self {
        self.is_direct = direct;
        self
    }
}
/// Configuration options for pre-definition analysis.
#[derive(Clone, Debug)]
pub struct PreDefConfig {
    /// Maximum recursion depth for termination checking.
    pub max_depth: usize,
    /// Whether to try automatic well-founded recursion inference.
    pub auto_wf: bool,
    /// Whether to allow partial definitions (with sorry/axiom).
    pub allow_partial: bool,
    /// Whether to generate detailed termination proof obligations.
    pub generate_proof_obligations: bool,
    /// Whether to use fuel-based termination (for non-terminating computations).
    pub use_fuel: bool,
    /// Maximum number of unfolding steps for termination analysis.
    pub max_unfolding: usize,
    /// Whether to try nested recursion through well-founded order.
    pub allow_nested_recursion: bool,
}
impl PreDefConfig {
    /// Create a default configuration.
    pub fn new() -> Self {
        Self {
            max_depth: 100,
            auto_wf: true,
            allow_partial: false,
            generate_proof_obligations: false,
            use_fuel: false,
            max_unfolding: 50,
            allow_nested_recursion: true,
        }
    }
    /// Enable proof obligation generation.
    pub fn with_proof_obligations(mut self) -> Self {
        self.generate_proof_obligations = true;
        self
    }
    /// Set maximum depth.
    pub fn with_max_depth(mut self, depth: usize) -> Self {
        self.max_depth = depth;
        self
    }
    /// Allow partial definitions.
    pub fn with_partial(mut self) -> Self {
        self.allow_partial = true;
        self
    }
}
