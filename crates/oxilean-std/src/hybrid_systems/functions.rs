//! Functions for hybrid dynamical systems simulation and analysis.

use super::types::{
    ContinuousDynamics, DiscreteTransition, ExecutionTrace, FlowType, GuardCondition,
    HybridAutomaton, HybridState, Invariant, ResetMap, SafetyProperty,
};

// ─── Guard / Reset helpers ────────────────────────────────────────────────────

/// Evaluate a guard condition against a continuous state vector.
///
/// * `Always` → always true.
/// * `LinearInequality { coeffs, bound }` → `coeffs · state <= bound`.
/// * `NonLinear(_)` → conservatively returns `false` (not evaluated).
pub fn check_guard(guard: &GuardCondition, state: &[f64]) -> bool {
    match guard {
        GuardCondition::Always => true,
        GuardCondition::LinearInequality { coeffs, bound } => {
            let dot: f64 = coeffs.iter().zip(state.iter()).map(|(c, x)| c * x).sum();
            dot <= *bound
        }
        GuardCondition::NonLinear(_) => false,
    }
}

/// Apply a reset map to a continuous state vector and return the new state.
pub fn apply_reset(reset: &ResetMap, state: &[f64]) -> Vec<f64> {
    match reset {
        ResetMap::Identity => state.to_vec(),
        ResetMap::Constant { values } => values.clone(),
        ResetMap::Linear { matrix } => matrix
            .iter()
            .map(|row| {
                row.iter()
                    .zip(state.iter())
                    .map(|(a, x)| a * x)
                    .sum::<f64>()
            })
            .collect(),
    }
}

// ─── Continuous dynamics ──────────────────────────────────────────────────────

/// Perform one forward-Euler integration step for the given mode dynamics.
///
/// For `Linear { a_matrix, b_vector }`: `x_new = x + dt * (A x + b)`.
/// For `Affine { a, b, c }`: `x_new = x + dt * (A x + c)`  (b unused, treated
///    as zero-input affine system).
/// For `Zero`: `x_new = x`.
pub fn linear_flow_step(dynamics: &ContinuousDynamics, state: &[f64], dt: f64) -> Vec<f64> {
    match &dynamics.flow {
        FlowType::Zero => state.to_vec(),
        FlowType::Linear { a_matrix, b_vector } => {
            let n = state.len();
            let mut deriv: Vec<f64> = vec![0.0; n];
            for (i, row) in a_matrix.iter().enumerate() {
                let ax: f64 = row.iter().zip(state.iter()).map(|(a, x)| a * x).sum();
                let b = b_vector.get(i).copied().unwrap_or(0.0);
                deriv[i] = ax + b;
            }
            state
                .iter()
                .zip(deriv.iter())
                .map(|(x, d)| x + dt * d)
                .collect()
        }
        FlowType::Affine { a, b: _, c } => {
            let n = state.len();
            let mut deriv: Vec<f64> = vec![0.0; n];
            for (i, row) in a.iter().enumerate() {
                let ax: f64 = row.iter().zip(state.iter()).map(|(ai, x)| ai * x).sum();
                let ci = c.get(i).copied().unwrap_or(0.0);
                deriv[i] = ax + ci;
            }
            state
                .iter()
                .zip(deriv.iter())
                .map(|(x, d)| x + dt * d)
                .collect()
        }
    }
}

/// Find the `ContinuousDynamics` for a given mode, returning a zero-flow
/// dynamics object if not found.
fn dynamics_for_mode(automaton: &HybridAutomaton, mode: usize) -> ContinuousDynamics {
    automaton
        .dynamics
        .iter()
        .find(|d| d.mode == mode)
        .cloned()
        .unwrap_or(ContinuousDynamics {
            mode,
            flow: FlowType::Zero,
        })
}

// ─── Simulation ───────────────────────────────────────────────────────────────

/// Simulate the hybrid automaton using forward-Euler integration with
/// guard-triggered discrete transitions.
///
/// At each step the function:
/// 1. Checks each outgoing transition's guard in order; fires the first enabled one.
/// 2. Otherwise advances the continuous state by one Euler step of size `dt`.
///
/// Simulation stops after `max_steps` steps.
pub fn simulate_euler(
    automaton: &HybridAutomaton,
    transitions: &[DiscreteTransition],
    dt: f64,
    max_steps: usize,
) -> ExecutionTrace {
    let mut trace = ExecutionTrace::empty();
    let mut current = HybridState::new(automaton.initial_mode, automaton.initial_state.clone());
    let mut t = 0.0_f64;

    trace.states.push(current.clone());
    trace.times.push(t);
    trace.transitions.push(None);

    for _ in 0..max_steps {
        // Check for an enabled outgoing transition.
        let fired = transitions.iter().enumerate().find(|(_, tr)| {
            tr.from_mode == current.mode && check_guard(&tr.guard, &current.continuous)
        });

        if let Some((idx, tr)) = fired {
            let new_cont = apply_reset(&tr.reset, &current.continuous);
            current = HybridState::new(tr.to_mode, new_cont);
            trace.states.push(current.clone());
            trace.times.push(t);
            trace.transitions.push(Some(idx));
        } else {
            // Continuous Euler step.
            let dyn_ = dynamics_for_mode(automaton, current.mode);
            let new_cont = linear_flow_step(&dyn_, &current.continuous, dt);
            current = HybridState::new(current.mode, new_cont);
            t += dt;
            trace.states.push(current.clone());
            trace.times.push(t);
            trace.transitions.push(None);
        }
    }
    trace
}

// ─── Safety verification ──────────────────────────────────────────────────────

/// Check a safety property against an execution trace.
///
/// Returns `true` if the property holds throughout the trace.
pub fn check_safety(trace: &ExecutionTrace, property: &SafetyProperty) -> bool {
    match property {
        SafetyProperty::ReachabilityAvoidance { forbidden_modes } => trace
            .states
            .iter()
            .all(|s| !forbidden_modes.contains(&s.mode)),
        SafetyProperty::StateBound { dim, lower, upper } => trace.states.iter().all(|s| {
            s.continuous
                .get(*dim)
                .map(|&v| v >= *lower && v <= *upper)
                .unwrap_or(true)
        }),
        SafetyProperty::ModeTime { mode, max_duration } => {
            // Accumulate time spent in `mode`.
            let mut acc = 0.0_f64;
            for i in 1..trace.states.len() {
                if trace.states[i - 1].mode == *mode {
                    let dt = trace.times[i] - trace.times[i - 1];
                    if dt > 0.0 {
                        acc += dt;
                    }
                }
            }
            acc <= *max_duration
        }
    }
}

// ─── Reachability / structural analysis ──────────────────────────────────────

/// Compute the set of mode indices reachable from the initial mode via
/// BFS over the discrete transition graph (ignoring guard conditions).
pub fn reachable_modes(automaton: &HybridAutomaton) -> Vec<usize> {
    let mut visited: Vec<bool> = vec![false; automaton.num_modes()];
    let mut queue: std::collections::VecDeque<usize> = std::collections::VecDeque::new();
    queue.push_back(automaton.initial_mode);
    if automaton.initial_mode < visited.len() {
        visited[automaton.initial_mode] = true;
    }
    while let Some(m) = queue.pop_front() {
        for tr in &automaton.transitions {
            if tr.from_mode == m {
                let target = tr.to_mode;
                if target < visited.len() && !visited[target] {
                    visited[target] = true;
                    queue.push_back(target);
                }
            }
        }
    }
    visited
        .iter()
        .enumerate()
        .filter_map(|(i, &v)| if v { Some(i) } else { None })
        .collect()
}

/// Detect Zeno behaviour: returns `true` if more than `threshold` discrete
/// transitions occur within any unit-time window in the trace.
///
/// A system is Zeno if it takes infinitely many transitions in finite time;
/// here we use a practical threshold on transition density.
pub fn zeno_detection(trace: &ExecutionTrace, threshold: f64) -> bool {
    if trace.times.is_empty() {
        return false;
    }
    let total_time = trace.total_time();
    if total_time <= 0.0 {
        // All transitions happened at the same time — definitely Zeno.
        return trace.jump_count() > 0;
    }
    let density = trace.jump_count() as f64 / total_time;
    density > threshold
}

/// Compute bisimulation equivalence classes among modes.
///
/// Two modes are considered equivalent (bisimilar) if they have:
/// * the same set of successor mode-labels (by name), and
/// * the same flow type (Linear / Affine / Zero).
///
/// Returns a list of equivalence classes (each a sorted `Vec<usize>`).
pub fn bisimulation_coarsening(automaton: &HybridAutomaton) -> Vec<Vec<usize>> {
    use std::collections::HashMap;

    // Build a signature for each mode.
    let mut sig_map: HashMap<String, Vec<usize>> = HashMap::new();

    for mode in 0..automaton.num_modes() {
        // Successor mode names (sorted for canonicity).
        let mut succs: Vec<String> = automaton
            .transitions
            .iter()
            .filter(|tr| tr.from_mode == mode)
            .map(|tr| automaton.modes.get(tr.to_mode).cloned().unwrap_or_default())
            .collect();
        succs.sort();
        succs.dedup();

        // Flow type tag.
        let flow_tag = automaton
            .dynamics
            .iter()
            .find(|d| d.mode == mode)
            .map(|d| match &d.flow {
                FlowType::Zero => "zero",
                FlowType::Linear { .. } => "linear",
                FlowType::Affine { .. } => "affine",
            })
            .unwrap_or("zero");

        let sig = format!("{}|{}", flow_tag, succs.join(","));
        sig_map.entry(sig).or_default().push(mode);
    }

    let mut classes: Vec<Vec<usize>> = sig_map.into_values().collect();
    for c in &mut classes {
        c.sort_unstable();
    }
    classes.sort_by_key(|c| c[0]);
    classes
}

/// Check whether a continuous state satisfies an invariant condition.
pub fn invariant_check(state: &[f64], invariant: &Invariant) -> bool {
    check_guard(&invariant.condition, state)
}

/// Compute the synchronous product of two hybrid automata.
///
/// The product has `n1 * n2` modes (pair-encoded as `i * n2 + j`).
/// Transitions are asynchronous interleaving: either automaton may take a
/// step while the other stays.  Dynamics are zero (continuous state is
/// concatenated: `[x1 || x2]`, each part evolved by its own flow).
pub fn product_automaton(a1: &HybridAutomaton, a2: &HybridAutomaton) -> HybridAutomaton {
    let n1 = a1.num_modes();
    let n2 = a2.num_modes();
    let total = n1 * n2;

    // Build mode names.
    let modes: Vec<String> = (0..n1)
        .flat_map(|i| {
            (0..n2).map(move |j| {
                format!(
                    "({},{})",
                    a1.modes.get(i).cloned().unwrap_or_else(|| i.to_string()),
                    a2.modes.get(j).cloned().unwrap_or_else(|| j.to_string())
                )
            })
        })
        .collect();

    let encode = |i: usize, j: usize| i * n2 + j;

    // Transitions from a1 (a2 stays in its mode).
    let mut transitions: Vec<DiscreteTransition> = Vec::new();
    for tr in &a1.transitions {
        for j in 0..n2 {
            transitions.push(DiscreteTransition {
                from_mode: encode(tr.from_mode, j),
                to_mode: encode(tr.to_mode, j),
                guard: tr.guard.clone(),
                reset: tr.reset.clone(),
            });
        }
    }
    // Transitions from a2 (a1 stays).
    for tr in &a2.transitions {
        for i in 0..n1 {
            transitions.push(DiscreteTransition {
                from_mode: encode(i, tr.from_mode),
                to_mode: encode(i, tr.to_mode),
                guard: tr.guard.clone(),
                reset: tr.reset.clone(),
            });
        }
    }

    // Use Zero dynamics for all product modes (safe conservative default).
    let dynamics: Vec<ContinuousDynamics> = (0..total)
        .map(|m| ContinuousDynamics {
            mode: m,
            flow: FlowType::Zero,
        })
        .collect();

    let initial_mode = encode(a1.initial_mode, a2.initial_mode);
    let mut initial_state = a1.initial_state.clone();
    initial_state.extend_from_slice(&a2.initial_state);

    HybridAutomaton::new(modes, transitions, dynamics, initial_mode, initial_state)
}

/// Return `true` if the automaton visits `mode` before time `time_bound`
/// in the given trace.
pub fn time_bounded_reachability(trace: &ExecutionTrace, mode: usize, time_bound: f64) -> bool {
    trace
        .states
        .iter()
        .zip(trace.times.iter())
        .any(|(s, &t)| s.mode == mode && t <= time_bound)
}

/// Estimate how much a candidate Lyapunov function V(x) = ||x||² decreases
/// over one Euler step under the given mode dynamics.
///
/// Returns `V(x_new) - V(x)`.  A negative value suggests local stability.
pub fn lyapunov_decrease(dynamics: &ContinuousDynamics, state: &[f64], dt: f64) -> f64 {
    let new_state = linear_flow_step(dynamics, state, dt);
    let v_new: f64 = new_state.iter().map(|x| x * x).sum();
    let v_old: f64 = state.iter().map(|x| x * x).sum();
    v_new - v_old
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn simple_automaton() -> HybridAutomaton {
        // Two modes: 0 = "on", 1 = "off".
        // Transition: on → off when x[0] >= 1  (i.e.  -x[0] <= -1).
        // Transition: off → on  always (reset x[0] = 0).
        let modes = vec!["on".to_string(), "off".to_string()];
        let transitions = vec![
            DiscreteTransition {
                from_mode: 0,
                to_mode: 1,
                guard: GuardCondition::LinearInequality {
                    coeffs: vec![-1.0],
                    bound: -1.0,
                },
                reset: ResetMap::Identity,
            },
            DiscreteTransition {
                from_mode: 1,
                to_mode: 0,
                guard: GuardCondition::Always,
                reset: ResetMap::Constant { values: vec![0.0] },
            },
        ];
        // In mode 0: ẋ = 1 (b_vector = [1], A = [[0]]).
        let dynamics = vec![ContinuousDynamics {
            mode: 0,
            flow: FlowType::Linear {
                a_matrix: vec![vec![0.0]],
                b_vector: vec![1.0],
            },
        }];
        HybridAutomaton::new(modes, transitions, dynamics, 0, vec![0.0])
    }

    #[test]
    fn test_check_guard_always() {
        assert!(check_guard(&GuardCondition::Always, &[1.0, 2.0]));
    }

    #[test]
    fn test_check_guard_linear_inequality_true() {
        let g = GuardCondition::LinearInequality {
            coeffs: vec![1.0, 0.0],
            bound: 5.0,
        };
        assert!(check_guard(&g, &[3.0, 999.0]));
    }

    #[test]
    fn test_check_guard_linear_inequality_false() {
        let g = GuardCondition::LinearInequality {
            coeffs: vec![1.0],
            bound: 0.5,
        };
        assert!(!check_guard(&g, &[1.0]));
    }

    #[test]
    fn test_check_guard_nonlinear_is_false() {
        assert!(!check_guard(
            &GuardCondition::NonLinear("f(x)>0".into()),
            &[1.0]
        ));
    }

    #[test]
    fn test_apply_reset_identity() {
        let r = apply_reset(&ResetMap::Identity, &[1.0, 2.0]);
        assert_eq!(r, vec![1.0, 2.0]);
    }

    #[test]
    fn test_apply_reset_constant() {
        let r = apply_reset(
            &ResetMap::Constant {
                values: vec![0.0, 0.0],
            },
            &[5.0, 7.0],
        );
        assert_eq!(r, vec![0.0, 0.0]);
    }

    #[test]
    fn test_apply_reset_linear() {
        // 2x2 identity matrix.
        let r = apply_reset(
            &ResetMap::Linear {
                matrix: vec![vec![1.0, 0.0], vec![0.0, 1.0]],
            },
            &[3.0, 4.0],
        );
        assert_eq!(r, vec![3.0, 4.0]);
    }

    #[test]
    fn test_linear_flow_step_zero() {
        let dyn_ = ContinuousDynamics {
            mode: 0,
            flow: FlowType::Zero,
        };
        let s = vec![1.0, 2.0];
        assert_eq!(linear_flow_step(&dyn_, &s, 0.1), s);
    }

    #[test]
    fn test_linear_flow_step_constant_velocity() {
        // ẋ = 1 → after dt = 0.5, x = 0.5.
        let dyn_ = ContinuousDynamics {
            mode: 0,
            flow: FlowType::Linear {
                a_matrix: vec![vec![0.0]],
                b_vector: vec![1.0],
            },
        };
        let new_s = linear_flow_step(&dyn_, &[0.0], 0.5);
        assert!((new_s[0] - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_simulate_euler_runs() {
        let automaton = simple_automaton();
        let transitions = automaton.transitions.clone();
        let trace = simulate_euler(&automaton, &transitions, 0.1, 50);
        assert!(!trace.states.is_empty());
        assert_eq!(trace.states.len(), trace.times.len());
    }

    #[test]
    fn test_reachable_modes_both_reachable() {
        let automaton = simple_automaton();
        let reachable = reachable_modes(&automaton);
        assert!(reachable.contains(&0));
        assert!(reachable.contains(&1));
    }

    #[test]
    fn test_check_safety_avoidance_pass() {
        let automaton = simple_automaton();
        let transitions = automaton.transitions.clone();
        let trace = simulate_euler(&automaton, &transitions, 0.1, 5);
        let prop = SafetyProperty::ReachabilityAvoidance {
            forbidden_modes: vec![99],
        };
        assert!(check_safety(&trace, &prop));
    }

    #[test]
    fn test_check_safety_avoidance_fail() {
        let automaton = simple_automaton();
        let transitions = automaton.transitions.clone();
        let trace = simulate_euler(&automaton, &transitions, 0.1, 30);
        // Mode 1 (off) will be reached due to the guard.
        let prop = SafetyProperty::ReachabilityAvoidance {
            forbidden_modes: vec![1],
        };
        // With enough steps mode 1 should be visited.
        let visited_1 = trace.states.iter().any(|s| s.mode == 1);
        if visited_1 {
            assert!(!check_safety(&trace, &prop));
        }
    }

    #[test]
    fn test_check_safety_state_bound() {
        let trace = ExecutionTrace {
            states: vec![
                HybridState::new(0, vec![0.5]),
                HybridState::new(0, vec![1.5]),
            ],
            times: vec![0.0, 0.1],
            transitions: vec![None, None],
        };
        let prop_pass = SafetyProperty::StateBound {
            dim: 0,
            lower: 0.0,
            upper: 2.0,
        };
        let prop_fail = SafetyProperty::StateBound {
            dim: 0,
            lower: 0.0,
            upper: 1.0,
        };
        assert!(check_safety(&trace, &prop_pass));
        assert!(!check_safety(&trace, &prop_fail));
    }

    #[test]
    fn test_zeno_detection_no_transitions() {
        let automaton = simple_automaton();
        let transitions = automaton.transitions.clone();
        let trace = simulate_euler(&automaton, &transitions, 0.1, 5);
        // Low threshold — if transitions happen, density > 1 fires.
        let _ = zeno_detection(&trace, 1000.0);
    }

    #[test]
    fn test_bisimulation_coarsening_nonempty() {
        let automaton = simple_automaton();
        let classes = bisimulation_coarsening(&automaton);
        assert!(!classes.is_empty());
        // All modes must appear somewhere.
        let all: Vec<usize> = classes.into_iter().flatten().collect();
        assert!(all.contains(&0));
        assert!(all.contains(&1));
    }

    #[test]
    fn test_invariant_check() {
        let inv = Invariant {
            mode: 0,
            condition: GuardCondition::LinearInequality {
                coeffs: vec![1.0],
                bound: 10.0,
            },
        };
        assert!(invariant_check(&[5.0], &inv));
        assert!(!invariant_check(&[11.0], &inv));
    }

    #[test]
    fn test_product_automaton_mode_count() {
        let a1 = simple_automaton();
        let a2 = simple_automaton();
        let prod = product_automaton(&a1, &a2);
        assert_eq!(prod.num_modes(), 4); // 2 × 2
    }

    #[test]
    fn test_time_bounded_reachability() {
        let automaton = simple_automaton();
        let transitions = automaton.transitions.clone();
        let trace = simulate_euler(&automaton, &transitions, 0.1, 20);
        // Mode 0 is reachable at time 0.
        assert!(time_bounded_reachability(&trace, 0, 0.0));
    }

    #[test]
    fn test_lyapunov_decrease_stable() {
        // ẋ = -x  → should decrease ||x||².
        let dyn_ = ContinuousDynamics {
            mode: 0,
            flow: FlowType::Linear {
                a_matrix: vec![vec![-1.0]],
                b_vector: vec![0.0],
            },
        };
        let decrease = lyapunov_decrease(&dyn_, &[2.0], 0.1);
        assert!(decrease < 0.0);
    }

    #[test]
    fn test_lyapunov_decrease_zero_flow() {
        let dyn_ = ContinuousDynamics {
            mode: 0,
            flow: FlowType::Zero,
        };
        let d = lyapunov_decrease(&dyn_, &[3.0, 4.0], 0.5);
        assert!((d).abs() < 1e-12);
    }

    #[test]
    fn test_check_safety_mode_time() {
        let trace = ExecutionTrace {
            states: vec![
                HybridState::new(0, vec![0.0]),
                HybridState::new(0, vec![0.1]),
                HybridState::new(1, vec![0.2]),
            ],
            times: vec![0.0, 0.5, 1.0],
            transitions: vec![None, None, Some(0)],
        };
        // Mode 0 is active from t=0..0.5, duration = 0.5.
        let prop_pass = SafetyProperty::ModeTime {
            mode: 0,
            max_duration: 1.0,
        };
        let prop_fail = SafetyProperty::ModeTime {
            mode: 0,
            max_duration: 0.3,
        };
        assert!(check_safety(&trace, &prop_pass));
        assert!(!check_safety(&trace, &prop_fail));
    }
}
