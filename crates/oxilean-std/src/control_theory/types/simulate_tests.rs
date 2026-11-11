//! The four `simulate` functions against verbatim copies of the previous
//! ones, which read the latest state back from `states` with `expect`.

use super::{DiscreteStateSpace, LtiSystem, MpcController, StateSpaceModel};

fn old_state_space(
    m: &StateSpaceModel,
    initial: Vec<f64>,
    inputs: &[Vec<f64>],
    dt: f64,
) -> Vec<Vec<f64>> {
    let mut states = vec![initial];
    for inp in inputs {
        let next = m.euler_step(states.last().expect("initial state"), inp, dt);
        states.push(next);
    }
    states
}
fn old_lti(m: &LtiSystem, initial: Vec<f64>, inputs: &[Vec<f64>], dt: f64) -> Vec<Vec<f64>> {
    let mut states = vec![initial];
    for inp in inputs {
        let next = m.euler_step(states.last().expect("initial state"), inp, dt);
        states.push(next);
    }
    states
}
fn old_discrete(m: &DiscreteStateSpace, initial: Vec<f64>, inputs: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let mut states = vec![initial];
    for inp in inputs {
        let next = m.step(states.last().expect("initial state"), inp);
        states.push(next);
    }
    states
}
fn old_mpc(
    m: &MpcController,
    initial: Vec<f64>,
    steps: usize,
    dt: f64,
) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
    let mut states = vec![initial];
    let mut inputs = Vec::new();
    for _ in 0..steps {
        let x = states.last().expect("initial state");
        let u = m.control(x).unwrap_or_else(|| vec![0.0; m.b[0].len()]);
        let ax = MpcController::mat_vec(&m.a, x);
        let bu = MpcController::mat_vec(&m.b, &u);
        let next: Vec<f64> = ax
            .iter()
            .zip(bu.iter())
            .zip(x.iter())
            .map(|((&axi, &bui), &xi)| xi + dt * (axi + bui))
            .collect();
        inputs.push(u);
        states.push(next);
    }
    (states, inputs)
}

fn a() -> Vec<Vec<f64>> {
    vec![vec![-0.5, 0.25], vec![0.125, -1.0]]
}
fn b() -> Vec<Vec<f64>> {
    vec![vec![1.0], vec![0.5]]
}
fn c() -> Vec<Vec<f64>> {
    vec![vec![1.0, 0.0]]
}
fn d() -> Vec<Vec<f64>> {
    vec![vec![0.0]]
}
fn inputs(len: usize) -> Vec<Vec<f64>> {
    (0..len).map(|k| vec![(k as f64 * 0.7).sin()]).collect()
}

#[test]
fn trajectories_are_unchanged() {
    for len in 0..6 {
        let u = inputs(len);
        let m = StateSpaceModel::new(a(), b(), c(), d());
        assert_eq!(
            m.simulate(vec![1.0, -2.0], &u, 0.1),
            old_state_space(&m, vec![1.0, -2.0], &u, 0.1)
        );
        let m = LtiSystem::new(a(), b(), c());
        assert_eq!(
            m.simulate(vec![0.5, 3.0], &u, 0.05),
            old_lti(&m, vec![0.5, 3.0], &u, 0.05)
        );
        let m = DiscreteStateSpace::new(a(), b(), c(), d(), 0.1);
        assert_eq!(
            m.simulate(vec![2.0, 1.0], &u),
            old_discrete(&m, vec![2.0, 1.0], &u)
        );
        let identity = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        for u_max in [vec![], vec![0.3]] {
            let m = MpcController::new(a(), b(), identity.clone(), vec![vec![0.5]], 3, u_max);
            assert_eq!(
                m.simulate(vec![1.0, 1.0], len, 0.1),
                old_mpc(&m, vec![1.0, 1.0], len, 0.1)
            );
        }
    }
}
