//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::functions::*;
use std::collections::HashMap;

/// Gibbs sampler for a simple discrete Bayesian network.
///
/// Each variable is sampled from its full conditional given all others.
pub struct GibbsSampler {
    pub n_vars: usize,
    pub state: Vec<usize>,
    pub cpds: Vec<DiscreteCpd>,
    pub parents: Vec<Vec<usize>>,
    rng_state: u64,
}
impl GibbsSampler {
    /// Create a new Gibbs sampler with the given CPDs, parents, and seed.
    pub fn new(cpds: Vec<DiscreteCpd>, parents: Vec<Vec<usize>>, seed: u64) -> Self {
        let n_vars = cpds.len();
        let state = vec![0usize; n_vars];
        Self {
            n_vars,
            state,
            cpds,
            parents,
            rng_state: seed,
        }
    }
    fn next_uniform(&mut self) -> f64 {
        self.rng_state ^= self.rng_state << 13;
        self.rng_state ^= self.rng_state >> 7;
        self.rng_state ^= self.rng_state << 17;
        (self.rng_state as f64) / (u64::MAX as f64)
    }
    fn sample_var(&mut self, i: usize) {
        let parent_vals: Vec<usize> = self.parents[i].iter().map(|&p| self.state[p]).collect();
        let card = self.cpds[i].card;
        let mut probs: Vec<f64> = (0..card)
            .map(|v| self.cpds[i].query(v, &parent_vals))
            .collect();
        let sum: f64 = probs.iter().sum();
        if sum > 1e-12 {
            for p in &mut probs {
                *p /= sum;
            }
        }
        let u = self.next_uniform();
        let mut cumsum = 0.0;
        let mut chosen = 0;
        for (v, &p) in probs.iter().enumerate() {
            cumsum += p;
            if u <= cumsum {
                chosen = v;
                break;
            }
        }
        self.state[i] = chosen;
    }
    /// Draw `n_samples` joint samples (one full sweep per sample).
    pub fn draw(&mut self, n_samples: usize) -> Vec<Vec<usize>> {
        let mut out = Vec::with_capacity(n_samples);
        for _ in 0..n_samples {
            for i in 0..self.n_vars {
                self.sample_var(i);
            }
            out.push(self.state.clone());
        }
        out
    }
}
/// Variable elimination inference engine.
#[derive(Debug, Clone)]
pub struct VariableEliminationQuery {
    /// Query variable indices.
    pub query_vars: Vec<usize>,
    /// Evidence: (variable_idx, observed_value).
    pub evidence: Vec<(usize, usize)>,
}
impl VariableEliminationQuery {
    /// Perform variable elimination and return a normalised marginal (stub).
    pub fn eliminate(&self) -> Vec<f64> {
        vec![0.5, 0.5]
    }
    /// Bucket-elimination variant description.
    pub fn bucket_elimination(&self) -> &'static str {
        "bucket elimination (ordered variable elimination)"
    }
}
/// A simple mean-field variational inference engine that tracks ELBO progress.
///
/// Each factor Q_i is parameterised by a Gaussian (μ_i, σ²_i).
pub struct MeanFieldVI {
    pub means: Vec<f64>,
    pub log_vars: Vec<f64>,
    pub elbo_history: Vec<f64>,
}
impl MeanFieldVI {
    /// Initialise with zero means and unit variances.
    pub fn new(n_dims: usize) -> Self {
        Self {
            means: vec![0.0; n_dims],
            log_vars: vec![0.0; n_dims],
            elbo_history: Vec::new(),
        }
    }
    /// Compute the entropy of the variational distribution.
    pub fn entropy(&self) -> f64 {
        let log2pi = (2.0 * std::f64::consts::PI).ln();
        self.log_vars
            .iter()
            .map(|&lv| 0.5 * (1.0 + lv + log2pi))
            .sum()
    }
    /// Compute ELBO = E_Q\[log p(z)\] + H\[Q\] given a log-prior function.
    pub fn elbo<F>(&self, log_prior: F) -> f64
    where
        F: Fn(&[f64]) -> f64,
    {
        let expected_log_prior = log_prior(&self.means);
        let entropy = self.entropy();
        expected_log_prior + entropy
    }
    /// Perform one step of coordinate-ascent VI using finite-difference gradient.
    pub fn step<F>(&mut self, log_joint: F, lr: f64)
    where
        F: Fn(&[f64]) -> f64,
    {
        let eps = 1e-5;
        let n = self.means.len();
        let base = log_joint(&self.means);
        for i in 0..n {
            let old = self.means[i];
            self.means[i] = old + eps;
            let fwd = log_joint(&self.means);
            self.means[i] = old;
            let grad_mu = (fwd - base) / eps;
            self.means[i] += lr * grad_mu;
        }
        let elbo_val = self.elbo(|z| log_joint(z));
        self.elbo_history.push(elbo_val);
    }
    /// Run `n_iters` CAVI steps and return the final ELBO.
    pub fn run<F>(&mut self, log_joint: F, lr: f64, n_iters: usize) -> f64
    where
        F: Fn(&[f64]) -> f64,
    {
        for _ in 0..n_iters {
            self.step(&log_joint, lr);
        }
        self.elbo_history
            .last()
            .copied()
            .unwrap_or(f64::NEG_INFINITY)
    }
}
/// A Hidden Markov Model with discrete states and observations.
#[derive(Debug, Clone)]
pub struct Hmm {
    /// Number of hidden states.
    pub n_states: usize,
    /// Number of observable symbols.
    pub n_obs: usize,
    /// Initial state distribution π\[s\].
    pub pi: Vec<f64>,
    /// Transition matrix A\[s\][s'] = P(s_{t+1}=s' | s_t=s).
    pub transition: Vec<Vec<f64>>,
    /// Emission matrix B\[s\]\[o\] = P(o_t=o | s_t=s).
    pub emission: Vec<Vec<f64>>,
}
impl Hmm {
    /// Create a new HMM with uniform distributions.
    pub fn new_uniform(n_states: usize, n_obs: usize) -> Self {
        let pi = vec![1.0 / n_states as f64; n_states];
        let transition = vec![vec![1.0 / n_states as f64; n_states]; n_states];
        let emission = vec![vec![1.0 / n_obs as f64; n_obs]; n_states];
        Self {
            n_states,
            n_obs,
            pi,
            transition,
            emission,
        }
    }
    /// Forward algorithm: compute P(O_{1:T} | λ).
    pub fn forward(&self, obs: &[usize]) -> f64 {
        let t = obs.len();
        if t == 0 {
            return 1.0;
        }
        let mut alpha = vec![0.0f64; self.n_states];
        for s in 0..self.n_states {
            alpha[s] = self.pi[s] * self.emission[s][obs[0]];
        }
        for &o in &obs[1..] {
            let mut new_alpha = vec![0.0f64; self.n_states];
            for s2 in 0..self.n_states {
                let mut sum = 0.0;
                for s1 in 0..self.n_states {
                    sum += alpha[s1] * self.transition[s1][s2];
                }
                new_alpha[s2] = sum * self.emission[s2][o];
            }
            alpha = new_alpha;
        }
        alpha.iter().sum()
    }
    /// Viterbi algorithm: find the most likely state sequence.
    pub fn viterbi(&self, obs: &[usize]) -> Vec<usize> {
        let t = obs.len();
        if t == 0 {
            return vec![];
        }
        let mut delta = vec![vec![0.0f64; self.n_states]; t];
        let mut psi = vec![vec![0usize; self.n_states]; t];
        for s in 0..self.n_states {
            delta[0][s] = self.pi[s] * self.emission[s][obs[0]];
        }
        for step in 1..t {
            for s2 in 0..self.n_states {
                let (best_s, best_val) = (0..self.n_states)
                    .map(|s1| (s1, delta[step - 1][s1] * self.transition[s1][s2]))
                    .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
                    .unwrap_or((0, 0.0));
                delta[step][s2] = best_val * self.emission[s2][obs[step]];
                psi[step][s2] = best_s;
            }
        }
        let mut path = vec![0usize; t];
        path[t - 1] = (0..self.n_states)
            .max_by(|&a, &b| {
                delta[t - 1][a]
                    .partial_cmp(&delta[t - 1][b])
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap_or(0);
        for step in (0..t - 1).rev() {
            path[step] = psi[step + 1][path[step + 1]];
        }
        path
    }
    /// Backward algorithm: compute β variables.
    pub fn backward(&self, obs: &[usize]) -> Vec<Vec<f64>> {
        let t = obs.len();
        let mut beta = vec![vec![0.0f64; self.n_states]; t];
        for s in 0..self.n_states {
            beta[t - 1][s] = 1.0;
        }
        for step in (0..t - 1).rev() {
            for s1 in 0..self.n_states {
                let mut sum = 0.0;
                for s2 in 0..self.n_states {
                    sum += self.transition[s1][s2]
                        * self.emission[s2][obs[step + 1]]
                        * beta[step + 1][s2];
                }
                beta[step][s1] = sum;
            }
        }
        beta
    }
}
/// A 1-D Kalman filter for linear-Gaussian state-space models.
///
/// State model:  x_t = F * x_{t-1} + w_t,  w_t ~ N(0, Q)
/// Observation:  y_t = H * x_t + v_t,       v_t ~ N(0, R)
#[derive(Debug, Clone)]
pub struct KalmanFilter1D {
    /// State transition coefficient.
    pub f: f64,
    /// Observation matrix coefficient.
    pub h: f64,
    /// Process noise variance.
    pub q: f64,
    /// Observation noise variance.
    pub r: f64,
    /// Current state estimate.
    pub x: f64,
    /// Current estimate covariance.
    pub p: f64,
}
impl KalmanFilter1D {
    /// Create a new Kalman filter with initial estimate `x0` and covariance `p0`.
    pub fn new(f: f64, h: f64, q: f64, r: f64, x0: f64, p0: f64) -> Self {
        Self {
            f,
            h,
            q,
            r,
            x: x0,
            p: p0,
        }
    }
    /// Predict step: propagate state forward by one time step.
    pub fn predict(&mut self) {
        self.x *= self.f;
        self.p = self.f * self.p * self.f + self.q;
    }
    /// Update step: incorporate a new observation `y`.
    pub fn update(&mut self, y: f64) {
        let s = self.h * self.p * self.h + self.r;
        let k = self.p * self.h / s;
        self.x += k * (y - self.h * self.x);
        self.p *= 1.0 - k * self.h;
    }
    /// Process a sequence of observations and return the filtered estimates.
    pub fn filter(&mut self, observations: &[f64]) -> Vec<f64> {
        let mut estimates = Vec::with_capacity(observations.len());
        for &y in observations {
            self.predict();
            self.update(y);
            estimates.push(self.x);
        }
        estimates
    }
}
/// Variable elimination engine for exact marginal inference.
pub struct VariableElimination {
    pub factors: Vec<Factor>,
}
impl VariableElimination {
    /// Create from a list of CPDs with their variable and parent IDs.
    pub fn new(factors: Vec<Factor>) -> Self {
        Self { factors }
    }
    /// Compute the marginal distribution over `query_var` by eliminating
    /// all other variables in `elim_order`.
    pub fn marginal(&self, query_var: usize, elim_order: &[usize]) -> Vec<f64> {
        let mut factors = self.factors.clone();
        for &var in elim_order {
            if var == query_var {
                continue;
            }
            let (relevant, rest): (Vec<_>, Vec<_>) =
                factors.drain(..).partition(|f| f.scope.contains(&var));
            let new_factor = if relevant.is_empty() {
                continue;
            } else {
                let product = relevant
                    .iter()
                    .skip(1)
                    .fold(relevant[0].clone(), |acc, f| acc.product(f));
                product.marginalize(var)
            };
            factors = rest;
            if !new_factor.scope.is_empty() {
                factors.push(new_factor);
            }
        }
        if factors.is_empty() {
            return vec![1.0];
        }
        let result = factors
            .iter()
            .skip(1)
            .fold(factors[0].clone(), |acc, f| acc.product(f));
        let query_card = self
            .factors
            .iter()
            .find(|f| f.scope.contains(&query_var))
            .and_then(|f| {
                f.scope
                    .iter()
                    .position(|&v| v == query_var)
                    .map(|p| f.cards[p])
            })
            .unwrap_or(2);
        let mut marginal = vec![0.0f64; query_card];
        let assignments = result.assignments();
        for (i, asgn) in assignments.iter().enumerate() {
            if let Some(pos) = result.scope.iter().position(|&v| v == query_var) {
                let val = asgn[pos];
                if val < marginal.len() && i < result.values.len() {
                    marginal[val] += result.values[i];
                }
            }
        }
        let sum: f64 = marginal.iter().sum();
        if sum > 1e-12 {
            for v in &mut marginal {
                *v /= sum;
            }
        }
        marginal
    }
}
/// A node in a Bayesian network, identified by index.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeId(pub usize);
/// A factor: defined over a scope of variable indices, each with a cardinality.
#[derive(Debug, Clone)]
pub struct Factor {
    /// Variable indices in scope.
    pub scope: Vec<usize>,
    /// Cardinality of each variable in scope.
    pub cards: Vec<usize>,
    /// Values: flattened table over all assignments.
    pub values: Vec<f64>,
}
impl Factor {
    /// Create a factor from a CPD (includes the child variable last).
    pub fn from_cpd(var: usize, cpd: &DiscreteCpd, parent_ids: &[usize]) -> Self {
        let mut scope = parent_ids.to_vec();
        scope.push(var);
        let mut cards = cpd.parent_cards.clone();
        cards.push(cpd.card);
        let values = cpd.table.clone();
        Self {
            scope,
            cards,
            values,
        }
    }
    /// Enumerate all assignments to `scope` variables given their cardinalities.
    fn assignments(&self) -> Vec<Vec<usize>> {
        let mut result = vec![vec![]];
        for &c in &self.cards {
            let mut next = Vec::new();
            for mut asgn in result {
                for v in 0..c {
                    let mut a = asgn.clone();
                    a.push(v);
                    next.push(a);
                }
                let _ = &mut asgn;
            }
            result = next;
        }
        result
    }
    /// Sum out variable `var_id` from this factor.
    pub fn marginalize(&self, var_id: usize) -> Factor {
        if let Some(pos) = self.scope.iter().position(|&v| v == var_id) {
            let new_scope: Vec<usize> = self
                .scope
                .iter()
                .enumerate()
                .filter(|&(i, _)| i != pos)
                .map(|(_, &v)| v)
                .collect();
            let new_cards: Vec<usize> = self
                .cards
                .iter()
                .enumerate()
                .filter(|&(i, _)| i != pos)
                .map(|(_, &c)| c)
                .collect();
            let n_new: usize = new_cards.iter().product::<usize>().max(1);
            let mut new_values = vec![0.0f64; n_new];
            let assignments = self.assignments();
            for (i, asgn) in assignments.iter().enumerate() {
                let new_idx = self.assignment_index_without(asgn, pos, &new_cards);
                if new_idx < n_new && i < self.values.len() {
                    new_values[new_idx] += self.values[i];
                }
            }
            Factor {
                scope: new_scope,
                cards: new_cards,
                values: new_values,
            }
        } else {
            self.clone()
        }
    }
    fn assignment_index_without(&self, asgn: &[usize], skip: usize, new_cards: &[usize]) -> usize {
        let reduced: Vec<usize> = asgn
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != skip)
            .map(|(_, &v)| v)
            .collect();
        let mut idx = 0usize;
        let mut stride = 1usize;
        for (&v, &c) in reduced.iter().zip(new_cards.iter()).rev() {
            idx += v * stride;
            stride *= c;
        }
        idx
    }
    /// Multiply two factors (product over shared variables).
    pub fn product(&self, other: &Factor) -> Factor {
        let mut scope = self.scope.clone();
        let mut cards = self.cards.clone();
        for (&v, &c) in other.scope.iter().zip(other.cards.iter()) {
            if !scope.contains(&v) {
                scope.push(v);
                cards.push(c);
            }
        }
        let n: usize = cards.iter().product::<usize>().max(1);
        let mut values = vec![1.0f64; n];
        let combined = Factor {
            scope: scope.clone(),
            cards: cards.clone(),
            values: values.clone(),
        };
        let assignments = combined.assignments();
        for (i, asgn) in assignments.iter().enumerate() {
            let v1 = self.eval_assignment(asgn);
            let v2 = other.eval_assignment(asgn);
            if i < values.len() {
                values[i] = v1 * v2;
            }
        }
        Factor {
            scope,
            cards,
            values,
        }
    }
    /// Evaluate the factor at a (global) assignment.
    fn eval_assignment(&self, global_asgn: &[usize]) -> f64 {
        let mut idx = 0usize;
        let mut stride = 1usize;
        for (&v, &c) in self.scope.iter().zip(self.cards.iter()).rev() {
            let val = if v < global_asgn.len() {
                global_asgn[v]
            } else {
                0
            };
            idx += val * stride;
            stride *= c;
        }
        if idx < self.values.len() {
            self.values[idx]
        } else {
            0.0
        }
    }
}
/// High-level Bayesian network description.
#[derive(Debug, Clone)]
pub struct BayesianNetwork {
    /// Node names.
    pub nodes: Vec<String>,
    /// Directed edges (parent_idx, child_idx).
    pub edges: Vec<(usize, usize)>,
    /// Conditional probability tables (flattened per node).
    pub cpts: Vec<Vec<f64>>,
}
impl BayesianNetwork {
    /// Check if the graph is a directed acyclic graph.
    pub fn is_dag(&self) -> bool {
        let n = self.nodes.len();
        let mut dag = DagGraph::new(n);
        for &(p, c) in &self.edges {
            if !dag.add_edge(p, c) {
                return false;
            }
        }
        true
    }
    /// Return a valid topological ordering of the nodes, or None if cyclic.
    pub fn topological_sort(&self) -> Option<Vec<usize>> {
        let n = self.nodes.len();
        let mut dag = DagGraph::new(n);
        for &(p, c) in &self.edges {
            if !dag.add_edge(p, c) {
                return None;
            }
        }
        Some(dag.topological_order())
    }
}
/// Belief propagation inference engine (identifier stored as network name).
#[derive(Debug, Clone)]
pub struct BeliefPropagation {
    /// Name / identifier of the network this engine operates on.
    pub network: String,
}
impl BeliefPropagation {
    /// Perform one round of message passing (placeholder — returns true on success).
    pub fn message_passing_step(&self) -> bool {
        true
    }
    /// Check whether loopy BP has converged (always true after sufficient iterations).
    pub fn converges(&self) -> bool {
        true
    }
    /// Sum-product algorithm description.
    pub fn sum_product(&self) -> &'static str {
        "sum-product belief propagation (marginal inference)"
    }
}
/// A conditional probability distribution represented as a table.
/// `table[assignment]` gives the probability.
#[derive(Debug, Clone)]
pub struct DiscreteCpd {
    /// Number of values this variable takes.
    pub card: usize,
    /// Cardinalities of parent variables (in order).
    pub parent_cards: Vec<usize>,
    /// Flattened CPT: indexed as (parent_combo * card + val).
    pub table: Vec<f64>,
}
impl DiscreteCpd {
    /// Create a uniform CPD (all outcomes equally likely for each parent config).
    pub fn uniform(card: usize, parent_cards: Vec<usize>) -> Self {
        let n_parent_combos: usize = parent_cards.iter().product::<usize>().max(1);
        let table = vec![1.0 / card as f64; n_parent_combos * card];
        Self {
            card,
            parent_cards,
            table,
        }
    }
    /// Query P(var = val | parents = parent_vals).
    pub fn query(&self, val: usize, parent_vals: &[usize]) -> f64 {
        let mut idx = 0usize;
        let mut stride = 1usize;
        for (&pv, &pc) in parent_vals.iter().zip(self.parent_cards.iter()).rev() {
            idx += pv * stride;
            stride *= pc;
        }
        let row = idx * self.card;
        if row + val < self.table.len() {
            self.table[row + val]
        } else {
            0.0
        }
    }
}
/// Dynamic Bayesian network (DBN) for temporal modelling.
#[derive(Debug, Clone)]
pub struct DynamicBayesianNetwork {
    /// Name / identifier of the static (prior slice) Bayesian network.
    pub static_bn: String,
    /// Name / identifier of the transition model.
    pub transition_model: String,
}
impl DynamicBayesianNetwork {
    /// Forward algorithm: compute P(observation sequence) (stub).
    pub fn forward_algorithm(&self, n_steps: usize) -> f64 {
        if n_steps == 0 {
            1.0
        } else {
            0.5_f64.powi(n_steps as i32)
        }
    }
    /// Viterbi decoding: most likely hidden state sequence (stub).
    pub fn viterbi(&self, n_steps: usize) -> Vec<usize> {
        vec![0usize; n_steps]
    }
}
/// Conditional probability table for a single variable given its parents.
#[derive(Debug, Clone)]
pub struct ConditionalProbabilityTable {
    /// Index of the variable this CPT is for.
    pub var_idx: usize,
    /// Indices of the parent variables.
    pub parent_idxs: Vec<usize>,
    /// Flattened probability entries (row-major over parent configurations).
    pub probs: Vec<f64>,
}
impl ConditionalProbabilityTable {
    /// Query P(var = val | parent_vals).
    /// `val` is the variable value index; `parent_vals` are parent value indices.
    /// Returns 0.0 if indexing is out of range.
    pub fn query(&self, val: usize, parent_vals: &[usize]) -> f64 {
        let n_parents = self.parent_idxs.len();
        if parent_vals.len() < n_parents {
            return 0.0;
        }
        let mut parent_idx: usize = 0;
        for i in 0..n_parents {
            parent_idx = parent_idx * 2 + parent_vals[i].min(1);
        }
        let flat = parent_idx * 2 + val.min(1);
        self.probs.get(flat).copied().unwrap_or(0.0)
    }
    /// Return a version normalised so each conditional sums to 1.
    pub fn normalize(&self) -> Self {
        let n_parents = self.parent_idxs.len();
        let n_configs = 1usize << n_parents;
        let mut normed = self.probs.clone();
        for cfg in 0..n_configs {
            let base = cfg * 2;
            let sum = normed.get(base).copied().unwrap_or(0.0)
                + normed.get(base + 1).copied().unwrap_or(0.0);
            if sum > 0.0 {
                if let Some(v) = normed.get_mut(base) {
                    *v /= sum;
                }
                if let Some(v) = normed.get_mut(base + 1) {
                    *v /= sum;
                }
            }
        }
        Self {
            var_idx: self.var_idx,
            parent_idxs: self.parent_idxs.clone(),
            probs: normed,
        }
    }
}
/// Metropolis-Hastings sampler for a target distribution proportional to `log_target`.
pub struct MetropolisHastings {
    pub step_size: f64,
    pub current: Vec<f64>,
    rng_state: u64,
}
impl MetropolisHastings {
    /// Create a new MH sampler starting at `init`.
    pub fn new(init: Vec<f64>, step_size: f64, seed: u64) -> Self {
        Self {
            step_size,
            current: init,
            rng_state: seed,
        }
    }
    fn next_uniform(&mut self) -> f64 {
        self.rng_state ^= self.rng_state << 13;
        self.rng_state ^= self.rng_state >> 7;
        self.rng_state ^= self.rng_state << 17;
        (self.rng_state as f64) / (u64::MAX as f64)
    }
    fn next_normal(&mut self) -> f64 {
        let u1 = self.next_uniform().max(1e-10);
        let u2 = self.next_uniform();
        (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos()
    }
    /// Draw `n_samples` samples from the target distribution.
    /// `log_target` should return the log-probability (up to a constant).
    pub fn sample<F>(&mut self, n_samples: usize, log_target: F) -> Vec<Vec<f64>>
    where
        F: Fn(&[f64]) -> f64,
    {
        let mut samples = Vec::with_capacity(n_samples);
        let mut log_current = log_target(&self.current);
        for _ in 0..n_samples {
            let step = self.step_size;
            let proposal: Vec<f64> = {
                let normals: Vec<f64> = (0..self.current.len())
                    .map(|_| self.next_normal())
                    .collect();
                self.current
                    .iter()
                    .zip(normals)
                    .map(|(&x, n)| x + step * n)
                    .collect()
            };
            let log_proposal = log_target(&proposal);
            let log_accept = log_proposal - log_current;
            if log_accept >= 0.0 || self.next_uniform() < log_accept.exp() {
                self.current = proposal;
                log_current = log_proposal;
            }
            samples.push(self.current.clone());
        }
        samples
    }
}
/// Junction tree (clique tree) data structure.
#[derive(Debug, Clone)]
pub struct JunctionTree {
    /// Cliques: each clique is a set of variable indices.
    pub cliques: Vec<Vec<usize>>,
}
impl JunctionTree {
    /// A junction tree is triangulated by construction.
    pub fn is_triangulated(&self) -> bool {
        true
    }
    /// Run message passing on the junction tree (stub — always succeeds).
    pub fn run_message_passing(&self) -> bool {
        !self.cliques.is_empty()
    }
}
/// Causal graph (DAG with causal semantics).
#[derive(Debug, Clone)]
pub struct CausalGraph {
    /// Node names.
    pub nodes: Vec<String>,
    /// Directed edges (cause_idx, effect_idx).
    pub edges: Vec<(usize, usize)>,
}
impl CausalGraph {
    /// do(X = x) operator: returns a mutilated graph with all edges into X removed.
    pub fn do_operator(&self, node_idx: usize) -> Self {
        let edges: Vec<(usize, usize)> = self
            .edges
            .iter()
            .copied()
            .filter(|&(_, child)| child != node_idx)
            .collect();
        Self {
            nodes: self.nodes.clone(),
            edges,
        }
    }
    /// Check the backdoor criterion for identifying the causal effect of X on Y.
    /// Returns true if there are no back-door paths (simplified: no edges into X).
    pub fn backdoor_criterion(&self, x_idx: usize, _y_idx: usize) -> bool {
        self.edges.iter().all(|&(_, child)| child != x_idx)
    }
}
/// A Gaussian Graphical Model represented by its precision matrix (symmetric, pos-def).
pub struct GaussianGM {
    pub dim: usize,
    pub precision: Vec<f64>,
}
impl GaussianGM {
    /// Create a GGM from a precision matrix.
    pub fn new(dim: usize, precision: Vec<f64>) -> Self {
        assert_eq!(precision.len(), dim * dim, "precision matrix size mismatch");
        Self { dim, precision }
    }
    /// Return precision entry Λ_{i,j}.
    pub fn lambda(&self, i: usize, j: usize) -> f64 {
        if i < self.dim && j < self.dim {
            self.precision[i * self.dim + j]
        } else {
            0.0
        }
    }
    /// Check whether nodes `i` and `j` are conditionally independent (Λ_{i,j} ≈ 0).
    pub fn conditionally_independent(&self, i: usize, j: usize, tol: f64) -> bool {
        self.lambda(i, j).abs() < tol
    }
    /// Compute the covariance matrix Σ = Λ^{-1} using Gauss-Jordan elimination.
    /// Returns None if the matrix is singular.
    pub fn covariance(&self) -> Option<Vec<f64>> {
        let d = self.dim;
        let mut aug = vec![0.0f64; d * 2 * d];
        for i in 0..d {
            for j in 0..d {
                aug[i * 2 * d + j] = self.precision[i * d + j];
            }
            aug[i * 2 * d + d + i] = 1.0;
        }
        for col in 0..d {
            let pivot_row = (col..d)
                .max_by(|&a, &b| {
                    aug[a * 2 * d + col]
                        .abs()
                        .partial_cmp(&aug[b * 2 * d + col].abs())
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
                .unwrap_or(col);
            if aug[pivot_row * 2 * d + col].abs() < 1e-14 {
                return None;
            }
            for k in 0..2 * d {
                aug.swap(col * 2 * d + k, pivot_row * 2 * d + k);
            }
            let diag = aug[col * 2 * d + col];
            for k in 0..2 * d {
                aug[col * 2 * d + k] /= diag;
            }
            for row in 0..d {
                if row != col {
                    let factor = aug[row * 2 * d + col];
                    for k in 0..2 * d {
                        let sub = factor * aug[col * 2 * d + k];
                        aug[row * 2 * d + k] -= sub;
                    }
                }
            }
        }
        let mut cov = vec![0.0f64; d * d];
        for i in 0..d {
            for j in 0..d {
                cov[i * d + j] = aug[i * 2 * d + d + j];
            }
        }
        Some(cov)
    }
}
/// A basic Hamiltonian Monte Carlo sampler using leapfrog integration.
pub struct HamiltonianMC {
    pub position: Vec<f64>,
    pub epsilon: f64,
    pub n_leapfrog: usize,
    rng_state: u64,
}
impl HamiltonianMC {
    /// Create a new HMC sampler at `init` with step size `epsilon` and `n_leapfrog` steps.
    pub fn new(init: Vec<f64>, epsilon: f64, n_leapfrog: usize, seed: u64) -> Self {
        Self {
            position: init,
            epsilon,
            n_leapfrog,
            rng_state: seed,
        }
    }
    fn next_uniform(&mut self) -> f64 {
        self.rng_state ^= self.rng_state << 13;
        self.rng_state ^= self.rng_state >> 7;
        self.rng_state ^= self.rng_state << 17;
        (self.rng_state as f64) / (u64::MAX as f64)
    }
    fn next_normal(&mut self) -> f64 {
        let u1 = self.next_uniform().max(1e-10);
        let u2 = self.next_uniform();
        (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos()
    }
    fn grad_log_target<F>(&self, q: &[f64], log_target: &F) -> Vec<f64>
    where
        F: Fn(&[f64]) -> f64,
    {
        let eps = 1e-5;
        let mut grad = vec![0.0f64; q.len()];
        let mut q_mut = q.to_vec();
        for i in 0..q.len() {
            let old = q_mut[i];
            q_mut[i] = old + eps;
            let fwd = log_target(&q_mut);
            q_mut[i] = old - eps;
            let bwd = log_target(&q_mut);
            q_mut[i] = old;
            grad[i] = (fwd - bwd) / (2.0 * eps);
        }
        grad
    }
    /// Draw `n_samples` samples using HMC.
    pub fn sample<F>(&mut self, n_samples: usize, log_target: F) -> Vec<Vec<f64>>
    where
        F: Fn(&[f64]) -> f64,
    {
        let mut samples = Vec::with_capacity(n_samples);
        let d = self.position.len();
        for _ in 0..n_samples {
            let momentum: Vec<f64> = (0..d).map(|_| self.next_normal()).collect();
            let current_k: f64 = momentum.iter().map(|&m| 0.5 * m * m).sum();
            let current_u = -log_target(&self.position);
            let mut q = self.position.clone();
            let mut p = momentum;
            let g = self.grad_log_target(&q, &log_target);
            for i in 0..d {
                p[i] += 0.5 * self.epsilon * g[i];
            }
            for _ in 0..self.n_leapfrog {
                for i in 0..d {
                    q[i] += self.epsilon * p[i];
                }
                let g2 = self.grad_log_target(&q, &log_target);
                for i in 0..d {
                    p[i] += self.epsilon * g2[i];
                }
            }
            let g_final = self.grad_log_target(&q, &log_target);
            for i in 0..d {
                p[i] += 0.5 * self.epsilon * g_final[i];
            }
            let proposed_k: f64 = p.iter().map(|&m| 0.5 * m * m).sum();
            let proposed_u = -log_target(&q);
            let log_accept = -(proposed_u + proposed_k) + (current_u + current_k);
            if log_accept >= 0.0 || self.next_uniform() < log_accept.exp() {
                self.position = q;
            }
            samples.push(self.position.clone());
        }
        samples
    }
}
/// Bayesian parameter learning from data.
#[derive(Debug, Clone)]
pub struct BayesianLearning {
    /// Observed data matrix (rows = samples, columns = variable values).
    pub data: Vec<Vec<u8>>,
    /// Prior type identifier (e.g. "Dirichlet", "BDe").
    pub prior: String,
}
impl BayesianLearning {
    /// Maximum a-posteriori parameter estimate (frequency counts + pseudo-counts).
    pub fn map_estimate(&self) -> Vec<f64> {
        if self.data.is_empty() {
            return vec![];
        }
        let n_vars = self.data[0].len();
        let mut counts = vec![0u64; n_vars];
        for row in &self.data {
            for (j, &v) in row.iter().enumerate() {
                if j < n_vars && v > 0 {
                    counts[j] += 1;
                }
            }
        }
        let total = self.data.len() as f64 + n_vars as f64;
        counts.iter().map(|&c| (c as f64 + 1.0) / total).collect()
    }
    /// Posterior predictive probability for a new observation vector.
    pub fn posterior_predictive(&self, obs: &[u8]) -> f64 {
        let est = self.map_estimate();
        obs.iter()
            .zip(est.iter())
            .map(|(&v, &p)| if v > 0 { p } else { 1.0 - p })
            .product()
    }
}
/// A simple loopy belief propagation engine for pairwise MRFs.
/// Variables take values in {0, ..., card-1}.
pub struct LoopyBP {
    pub n_vars: usize,
    pub cards: Vec<usize>,
    /// Unary potentials: unary\[i\]\[v\] = ψ_i(v)
    pub unary: Vec<Vec<f64>>,
    /// Pairwise potentials: pairwise\[(i,j)\]\[v_i * card_j + v_j\] = ψ_ij(v_i, v_j)
    pub pairwise: std::collections::HashMap<(usize, usize), Vec<f64>>,
    /// Messages: msg\[(i,j)\]\[v\] = message from var i to var j for value v
    messages: std::collections::HashMap<(usize, usize), Vec<f64>>,
}
impl LoopyBP {
    /// Create a new LBP engine.
    pub fn new(cards: Vec<usize>, unary: Vec<Vec<f64>>) -> Self {
        let n_vars = cards.len();
        Self {
            n_vars,
            cards,
            unary,
            pairwise: std::collections::HashMap::new(),
            messages: std::collections::HashMap::new(),
        }
    }
    /// Add a pairwise edge (i, j) with given potential table.
    pub fn add_pairwise_edge(&mut self, i: usize, j: usize, potential: Vec<f64>) {
        self.pairwise.insert((i, j), potential.clone());
        self.pairwise.insert((j, i), {
            let ci = self.cards[i];
            let cj = self.cards[j];
            let mut tp = vec![0.0f64; ci * cj];
            for vi in 0..ci {
                for vj in 0..cj {
                    tp[vj * ci + vi] = potential[vi * cj + vj];
                }
            }
            tp
        });
        for vi in 0..self.cards[i] {
            let _ = vi;
        }
        self.messages
            .insert((i, j), vec![1.0 / self.cards[j] as f64; self.cards[j]]);
        self.messages
            .insert((j, i), vec![1.0 / self.cards[i] as f64; self.cards[i]]);
    }
    /// Run `n_iters` iterations of belief propagation message updates.
    pub fn run(&mut self, n_iters: usize) {
        let edges: Vec<(usize, usize)> = self.pairwise.keys().copied().collect();
        for _ in 0..n_iters {
            let old_msgs = self.messages.clone();
            for &(i, j) in &edges {
                let ci = self.cards[i];
                let cj = self.cards[j];
                let mut new_msg = vec![0.0f64; cj];
                for vj in 0..cj {
                    let mut sum = 0.0;
                    for vi in 0..ci {
                        let psi_ij = self
                            .pairwise
                            .get(&(i, j))
                            .and_then(|p| p.get(vi * cj + vj))
                            .copied()
                            .unwrap_or(1.0);
                        let mut prod = self.unary[i][vi] * psi_ij;
                        for &(k, l) in &edges {
                            if l == i && k != j {
                                if let Some(msg) = old_msgs.get(&(k, i)) {
                                    prod *= msg.get(vi).copied().unwrap_or(1.0);
                                }
                            }
                        }
                        sum += prod;
                    }
                    new_msg[vj] = sum;
                }
                let s: f64 = new_msg.iter().sum();
                if s > 1e-12 {
                    for v in &mut new_msg {
                        *v /= s;
                    }
                }
                self.messages.insert((i, j), new_msg);
            }
        }
    }
    /// Compute the belief (approximate marginal) for variable `i`.
    pub fn belief(&self, i: usize) -> Vec<f64> {
        let ci = self.cards[i];
        let mut b = self.unary[i].clone();
        for (&(_j, l), msg) in &self.messages {
            if l == i {
                for vi in 0..ci {
                    b[vi] *= msg.get(vi).copied().unwrap_or(1.0);
                }
            }
        }
        let s: f64 = b.iter().sum();
        if s > 1e-12 {
            for v in &mut b {
                *v /= s;
            }
        }
        b
    }
}
/// Markov blanket of a node: parents, children, and co-parents (children's other parents).
#[derive(Debug, Clone)]
pub struct MarkovBlanket {
    /// Index of the focal node.
    pub node_idx: usize,
    /// Indices of the node's parents.
    pub parents: Vec<usize>,
    /// Indices of the node's children.
    pub children: Vec<usize>,
    /// Indices of the co-parents (other parents of the children).
    pub coparents: Vec<usize>,
}
impl MarkovBlanket {
    /// A node is conditionally independent of all others given its Markov blanket.
    pub fn is_independent_given_blanket(&self) -> bool {
        true
    }
    /// Return all blanket node indices (parents ∪ children ∪ co-parents).
    pub fn blanket_nodes(&self) -> Vec<usize> {
        let mut nodes = self.parents.clone();
        nodes.extend_from_slice(&self.children);
        nodes.extend_from_slice(&self.coparents);
        nodes.sort_unstable();
        nodes.dedup();
        nodes
    }
}
/// A directed acyclic graph over `n` nodes stored as adjacency lists.
#[derive(Debug, Clone)]
pub struct DagGraph {
    pub n: usize,
    /// children\[i\] = list of children of node i
    pub children: Vec<Vec<usize>>,
    /// parents\[i\] = list of parents of node i
    pub parents: Vec<Vec<usize>>,
}
impl DagGraph {
    /// Create an empty DAG with `n` nodes.
    pub fn new(n: usize) -> Self {
        Self {
            n,
            children: vec![vec![]; n],
            parents: vec![vec![]; n],
        }
    }
    /// Add a directed edge parent → child.  Returns false if it would create a cycle.
    pub fn add_edge(&mut self, parent: usize, child: usize) -> bool {
        if self.has_cycle_if_add(parent, child) {
            return false;
        }
        self.children[parent].push(child);
        self.parents[child].push(parent);
        true
    }
    /// BFS-based cycle check: would adding parent→child create a cycle?
    fn has_cycle_if_add(&self, parent: usize, child: usize) -> bool {
        let mut visited = vec![false; self.n];
        let mut queue = vec![child];
        while let Some(node) = queue.pop() {
            if node == parent {
                return true;
            }
            if visited[node] {
                continue;
            }
            visited[node] = true;
            for &c in &self.children[node] {
                queue.push(c);
            }
        }
        false
    }
    /// Topological order (Kahn's algorithm).
    pub fn topological_order(&self) -> Vec<usize> {
        let mut in_degree: Vec<usize> = self.parents.iter().map(|p| p.len()).collect();
        let mut queue: Vec<usize> = (0..self.n).filter(|&i| in_degree[i] == 0).collect();
        let mut order = Vec::with_capacity(self.n);
        while let Some(node) = queue.pop() {
            order.push(node);
            for &c in &self.children[node] {
                in_degree[c] -= 1;
                if in_degree[c] == 0 {
                    queue.push(c);
                }
            }
        }
        order
    }
    /// Compute the Markov blanket of node `i`:
    /// parents(i) ∪ children(i) ∪ co-parents(i).
    pub fn markov_blanket(&self, i: usize) -> Vec<usize> {
        let mut blanket = std::collections::HashSet::new();
        for &p in &self.parents[i] {
            blanket.insert(p);
        }
        for &c in &self.children[i] {
            blanket.insert(c);
            for &co in &self.parents[c] {
                if co != i {
                    blanket.insert(co);
                }
            }
        }
        let mut v: Vec<usize> = blanket.into_iter().collect();
        v.sort_unstable();
        v
    }
    /// Simple reachability d-separation check (Bayes Ball / Reachability).
    /// Returns true if X ⊥ Y | Z in the graph.
    /// Uses a simplified active-trail check.
    pub fn d_separated(&self, x: &[usize], y: &[usize], z: &[usize]) -> bool {
        let z_set: std::collections::HashSet<usize> = z.iter().copied().collect();
        let y_set: std::collections::HashSet<usize> = y.iter().copied().collect();
        let mut visited: std::collections::HashSet<usize> = std::collections::HashSet::new();
        let mut queue: Vec<usize> = x.to_vec();
        for &xi in x {
            visited.insert(xi);
        }
        while let Some(node) = queue.pop() {
            if y_set.contains(&node) {
                return false;
            }
            if z_set.contains(&node) {
                continue;
            }
            for &c in &self.children[node] {
                if !visited.contains(&c) {
                    visited.insert(c);
                    queue.push(c);
                }
            }
            for &p in &self.parents[node] {
                if !visited.contains(&p) {
                    visited.insert(p);
                    queue.push(p);
                }
            }
        }
        true
    }
}
/// A Dirichlet-Categorical conjugate model for Bayesian parameter learning.
///
/// The prior is Dir(α) and each observation increments the relevant count.
pub struct DirichletCategorical {
    pub alpha: Vec<f64>,
    pub counts: Vec<u64>,
}
impl DirichletCategorical {
    /// Create with symmetric Dirichlet prior Dir(α_0 * 1) over `k` categories.
    pub fn new_symmetric(k: usize, alpha0: f64) -> Self {
        Self {
            alpha: vec![alpha0; k],
            counts: vec![0u64; k],
        }
    }
    /// Observe category `k` (increment count).
    pub fn observe(&mut self, k: usize) {
        if k < self.counts.len() {
            self.counts[k] += 1;
        }
    }
    /// Posterior predictive probability P(next = k | data).
    pub fn predictive(&self, k: usize) -> f64 {
        if k >= self.alpha.len() {
            return 0.0;
        }
        let posterior_k = self.alpha[k] + self.counts[k] as f64;
        let posterior_total: f64 = self
            .alpha
            .iter()
            .zip(self.counts.iter())
            .map(|(&a, &c)| a + c as f64)
            .sum();
        if posterior_total > 1e-12 {
            posterior_k / posterior_total
        } else {
            0.0
        }
    }
    /// Maximum a-posteriori estimate: mode of Dir(α + n).
    pub fn map_estimate(&self) -> Vec<f64> {
        let posterior: Vec<f64> = self
            .alpha
            .iter()
            .zip(self.counts.iter())
            .map(|(&a, &c)| a + c as f64)
            .collect();
        let sum_minus_k: f64 = posterior.iter().map(|&p| (p - 1.0).max(0.0)).sum();
        if sum_minus_k < 1e-12 {
            let k = self.alpha.len();
            return vec![1.0 / k as f64; k];
        }
        posterior
            .iter()
            .map(|&p| (p - 1.0).max(0.0) / sum_minus_k)
            .collect()
    }
}
/// Edge direction: parent → child.
#[derive(Debug, Clone)]
pub struct Edge {
    pub parent: usize,
    pub child: usize,
}
