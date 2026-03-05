//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
use super::functions::*;

pub struct NeuralNetwork {
    pub layers: Vec<Layer>,
}
impl NeuralNetwork {
    pub fn new(layers: Vec<Layer>) -> Self {
        NeuralNetwork { layers }
    }
    pub fn forward(&self, input: &[f64]) -> Vec<f64> {
        let mut current: Vec<f64> = input.to_vec();
        for layer in &self.layers {
            current = layer.forward(&current);
        }
        current
    }
    pub fn forward_cached(&self, input: &[f64]) -> (Vec<Vec<f64>>, Vec<Vec<f64>>, Vec<Vec<f64>>) {
        let mut activations = vec![input.to_vec()];
        let mut z_cache = Vec::new();
        let mut a_cache = Vec::new();
        let mut current = input.to_vec();
        for layer in &self.layers {
            let (z, a) = layer.forward_with_cache(&current);
            z_cache.push(z);
            a_cache.push(a.clone());
            activations.push(a.clone());
            current = a;
        }
        (activations, z_cache, a_cache)
    }
    pub fn n_params(&self) -> usize {
        self.layers.iter().map(|l| l.n_params()).sum()
    }
    pub fn predict_class(&self, input: &[f64]) -> usize {
        let out = self.forward(input);
        out.iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i)
            .unwrap_or(0)
    }
    pub fn depth(&self) -> usize {
        self.layers.len()
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UncertaintyStrategy {
    LeastConfident,
    MarginSampling,
    Entropy,
}
pub struct GradientDescent {
    pub learning_rate: f64,
    pub max_epochs: u32,
    pub tolerance: f64,
}
impl GradientDescent {
    pub fn new(lr: f64, max_epochs: u32) -> Self {
        GradientDescent {
            learning_rate: lr,
            max_epochs,
            tolerance: 1e-8,
        }
    }
    pub fn minimize_quadratic(&self, a: f64, b: f64, x0: f64) -> f64 {
        let mut x = x0;
        for _ in 0..self.max_epochs {
            let grad = 2.0 * a * x + b;
            let new_x = x - self.learning_rate * grad;
            if (new_x - x).abs() < self.tolerance {
                return new_x;
            }
            x = new_x;
        }
        x
    }
    pub fn minimize_numerical<F: Fn(f64) -> f64>(&self, f: &F, x0: f64) -> f64 {
        let h = 1e-7;
        let mut x = x0;
        for _ in 0..self.max_epochs {
            let grad = (f(x + h) - f(x - h)) / (2.0 * h);
            let new_x = x - self.learning_rate * grad;
            if (new_x - x).abs() < self.tolerance {
                return new_x;
            }
            x = new_x;
        }
        x
    }
}
pub struct PolynomialRegression {
    pub coefficients: Vec<f64>,
    pub degree: usize,
}
impl PolynomialRegression {
    fn make_features(x: f64, degree: usize) -> Vec<f64> {
        let mut features = Vec::with_capacity(degree + 1);
        let mut xp = 1.0;
        for _ in 0..=degree {
            features.push(xp);
            xp *= x;
        }
        features
    }
    pub fn fit(x_data: &[f64], y_data: &[f64], degree: usize, lr: f64, epochs: u32) -> Self {
        let n = x_data.len().min(y_data.len());
        let mut coeffs = vec![0.0f64; degree + 1];
        for _ in 0..epochs {
            let mut grads = vec![0.0f64; degree + 1];
            for i in 0..n {
                let features = Self::make_features(x_data[i], degree);
                let pred: f64 = features.iter().zip(coeffs.iter()).map(|(f, c)| f * c).sum();
                let err = pred - y_data[i];
                for (j, feat) in features.iter().enumerate() {
                    grads[j] += 2.0 * err * feat / n as f64;
                }
            }
            for (c, g) in coeffs.iter_mut().zip(grads.iter()) {
                *c -= lr * g;
            }
        }
        PolynomialRegression {
            coefficients: coeffs,
            degree,
        }
    }
    pub fn predict(&self, x: f64) -> f64 {
        let features = Self::make_features(x, self.degree);
        features
            .iter()
            .zip(self.coefficients.iter())
            .map(|(f, c)| f * c)
            .sum()
    }
}
#[derive(Debug, Clone)]
pub struct ElasticWeightConsolidation {
    pub lambda: f64,
    pub fisher_diagonal: Vec<f64>,
    pub theta_star: Vec<f64>,
}
impl ElasticWeightConsolidation {
    pub fn new(lambda: f64) -> Self {
        ElasticWeightConsolidation {
            lambda,
            fisher_diagonal: Vec::new(),
            theta_star: Vec::new(),
        }
    }
    /// Store the current parameters and a diagonal Fisher approximation
    /// computed from `gradients` (one gradient vector per data point).
    pub fn consolidate(&mut self, params: &[f64], gradients: &[Vec<f64>]) {
        self.theta_star = params.to_vec();
        let n = gradients.len();
        if n == 0 {
            self.fisher_diagonal = vec![0.0; params.len()];
            return;
        }
        let mut fisher = vec![0.0f64; params.len()];
        for grad in gradients {
            for (i, &g) in grad.iter().enumerate() {
                if i < fisher.len() {
                    fisher[i] += g * g;
                }
            }
        }
        for f in &mut fisher {
            *f /= n as f64;
        }
        self.fisher_diagonal = fisher;
    }
    /// EWC penalty: lambda/2 * sum_i F_i * (theta_i - theta*_i)^2
    pub fn penalty(&self, params: &[f64]) -> f64 {
        if self.fisher_diagonal.is_empty() || self.theta_star.is_empty() {
            return 0.0;
        }
        let sum: f64 = params
            .iter()
            .zip(self.theta_star.iter())
            .zip(self.fisher_diagonal.iter())
            .map(|((p, s), f)| f * (p - s).powi(2))
            .sum();
        0.5 * self.lambda * sum
    }
    /// Gradient of the EWC penalty w.r.t. params
    pub fn penalty_gradient(&self, params: &[f64]) -> Vec<f64> {
        if self.fisher_diagonal.is_empty() || self.theta_star.is_empty() {
            return vec![0.0; params.len()];
        }
        params
            .iter()
            .zip(self.theta_star.iter())
            .zip(self.fisher_diagonal.iter())
            .map(|((p, s), f)| self.lambda * f * (p - s))
            .collect()
    }
}
#[derive(Debug, Clone)]
pub struct ShapleyExplainer {
    pub n_features: usize,
    pub n_samples: usize,
    pub background: Vec<Vec<f64>>,
}
impl ShapleyExplainer {
    pub fn new(n_features: usize, n_samples: usize, background: Vec<Vec<f64>>) -> Self {
        ShapleyExplainer {
            n_features,
            n_samples,
            background,
        }
    }
    /// Estimate Shapley values for `x` using the given model function via
    /// the random-permutation sampling approach.
    pub fn explain<F: Fn(&[f64]) -> f64>(&self, x: &[f64], model: &F) -> Vec<f64> {
        let d = self.n_features.min(x.len());
        if d == 0 || self.background.is_empty() {
            return vec![0.0; d];
        }
        let mut phi = vec![0.0f64; d];
        let n_bg = self.background.len();
        let mut state: u64 = 0x5851f42d4c957f2d_u64;
        let lcg = |s: &mut u64| -> usize {
            *s = s
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            (*s >> 33) as usize
        };
        for _ in 0..self.n_samples {
            let mut perm: Vec<usize> = (0..d).collect();
            for i in (1..d).rev() {
                let j = lcg(&mut state) % (i + 1);
                perm.swap(i, j);
            }
            let bg_idx = lcg(&mut state) % n_bg;
            let bg = &self.background[bg_idx];
            let mut coalition: Vec<bool> = vec![false; d];
            let mut prev_val = {
                let inp: Vec<f64> = (0..d)
                    .map(|j| if j < bg.len() { bg[j] } else { 0.0 })
                    .collect();
                model(&inp)
            };
            for &feat in &perm {
                coalition[feat] = true;
                let inp: Vec<f64> = (0..d)
                    .map(|j| {
                        if coalition[j] {
                            if j < x.len() {
                                x[j]
                            } else {
                                0.0
                            }
                        } else if j < bg.len() {
                            bg[j]
                        } else {
                            0.0
                        }
                    })
                    .collect();
                let new_val = model(&inp);
                phi[feat] += new_val - prev_val;
                prev_val = new_val;
            }
        }
        for p in &mut phi {
            *p /= self.n_samples as f64;
        }
        phi
    }
}
pub struct KMeans {
    pub k: usize,
    pub centroids: Vec<Vec<f64>>,
    pub max_iter: u32,
}
impl KMeans {
    pub fn new(k: usize, max_iter: u32) -> Self {
        KMeans {
            k,
            centroids: Vec::new(),
            max_iter,
        }
    }
    pub fn fit(&mut self, data: &[Vec<f64>], _seed: u64) -> Vec<usize> {
        if data.is_empty() || self.k == 0 {
            return vec![];
        }
        let k = self.k.min(data.len());
        self.centroids = data[..k].to_vec();
        let mut assignments = vec![0usize; data.len()];
        for _ in 0..self.max_iter {
            let new_assignments = self.assign_clusters(data);
            if new_assignments == assignments {
                assignments = new_assignments;
                break;
            }
            assignments = new_assignments;
            self.update_centroids(data, &assignments);
        }
        assignments
    }
    pub fn predict(&self, point: &[f64]) -> usize {
        self.centroids
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| {
                let da: f64 = a
                    .iter()
                    .zip(point.iter())
                    .map(|(x, y)| (x - y).powi(2))
                    .sum();
                let db: f64 = b
                    .iter()
                    .zip(point.iter())
                    .map(|(x, y)| (x - y).powi(2))
                    .sum();
                da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(i, _)| i)
            .unwrap_or(0)
    }
    pub fn inertia(&self, data: &[Vec<f64>]) -> f64 {
        data.iter()
            .map(|p| {
                self.centroids
                    .iter()
                    .map(|c| {
                        c.iter()
                            .zip(p.iter())
                            .map(|(ci, pi)| (ci - pi).powi(2))
                            .sum::<f64>()
                    })
                    .fold(f64::INFINITY, f64::min)
            })
            .sum()
    }
    fn assign_clusters(&self, data: &[Vec<f64>]) -> Vec<usize> {
        data.iter().map(|p| self.predict(p)).collect()
    }
    fn update_centroids(&mut self, data: &[Vec<f64>], assignments: &[usize]) {
        let dim = if data.is_empty() { 0 } else { data[0].len() };
        let k = self.centroids.len();
        let mut sums = vec![vec![0.0f64; dim]; k];
        let mut counts = vec![0usize; k];
        for (point, &cluster) in data.iter().zip(assignments.iter()) {
            if cluster < k {
                for (s, v) in sums[cluster].iter_mut().zip(point.iter()) {
                    *s += v;
                }
                counts[cluster] += 1;
            }
        }
        for c in 0..k {
            if counts[c] > 0 {
                for d in 0..dim {
                    self.centroids[c][d] = sums[c][d] / counts[c] as f64;
                }
            }
        }
    }
}
#[derive(Debug, Clone)]
pub struct Layer {
    pub weights: Vec<Vec<f64>>,
    pub biases: Vec<f64>,
    pub activation: Activation,
}
impl Layer {
    pub fn new(n_in: usize, n_out: usize, activation: Activation) -> Self {
        let mut state: u64 = (n_in as u64).wrapping_mul(6364136223846793005)
            ^ (n_out as u64).wrapping_add(1442695040888963407);
        let mut next = move || -> f64 {
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            let bits = ((state >> 33) as u32) as f64;
            bits / u32::MAX as f64 * 0.2 - 0.1
        };
        let weights = (0..n_out)
            .map(|_| (0..n_in).map(|_| next()).collect())
            .collect();
        let biases = vec![0.0; n_out];
        Layer {
            weights,
            biases,
            activation,
        }
    }
    pub fn from_weights(weights: Vec<Vec<f64>>, biases: Vec<f64>, activation: Activation) -> Self {
        Layer {
            weights,
            biases,
            activation,
        }
    }
    pub fn forward(&self, input: &[f64]) -> Vec<f64> {
        self.weights
            .iter()
            .enumerate()
            .map(|(i, row)| {
                let z: f64 = row
                    .iter()
                    .zip(input.iter())
                    .map(|(w, x)| w * x)
                    .sum::<f64>()
                    + self.biases[i];
                self.activation.apply(z)
            })
            .collect()
    }
    pub fn forward_with_cache(&self, input: &[f64]) -> (Vec<f64>, Vec<f64>) {
        let mut z_vals = Vec::with_capacity(self.weights.len());
        let mut a_vals = Vec::with_capacity(self.weights.len());
        for (i, row) in self.weights.iter().enumerate() {
            let z: f64 = row
                .iter()
                .zip(input.iter())
                .map(|(w, x)| w * x)
                .sum::<f64>()
                + self.biases[i];
            z_vals.push(z);
            a_vals.push(self.activation.apply(z));
        }
        (z_vals, a_vals)
    }
    pub fn n_params(&self) -> usize {
        let n_out = self.weights.len();
        let n_in = if n_out > 0 { self.weights[0].len() } else { 0 };
        n_out * n_in + n_out
    }
    pub fn n_inputs(&self) -> usize {
        if self.weights.is_empty() {
            0
        } else {
            self.weights[0].len()
        }
    }
    pub fn n_outputs(&self) -> usize {
        self.weights.len()
    }
}
pub struct AdamOptimizer {
    pub learning_rate: f64,
    pub beta1: f64,
    pub beta2: f64,
    pub epsilon: f64,
    pub max_epochs: u32,
}
impl AdamOptimizer {
    pub fn new(lr: f64, max_epochs: u32) -> Self {
        AdamOptimizer {
            learning_rate: lr,
            beta1: 0.9,
            beta2: 0.999,
            epsilon: 1e-8,
            max_epochs,
        }
    }
    pub fn minimize_quadratic(&self, a: f64, b: f64, x0: f64) -> f64 {
        let mut x = x0;
        let mut m = 0.0;
        let mut v = 0.0;
        for t in 1..=self.max_epochs {
            let grad = 2.0 * a * x + b;
            m = self.beta1 * m + (1.0 - self.beta1) * grad;
            v = self.beta2 * v + (1.0 - self.beta2) * grad * grad;
            let m_hat = m / (1.0 - self.beta1.powi(t as i32));
            let v_hat = v / (1.0 - self.beta2.powi(t as i32));
            let new_x = x - self.learning_rate * m_hat / (v_hat.sqrt() + self.epsilon);
            if (new_x - x).abs() < 1e-10 {
                return new_x;
            }
            x = new_x;
        }
        x
    }
}
#[derive(Debug, Clone)]
pub struct PACBayesBound {
    pub delta: f64,
    pub n_samples: usize,
}
impl PACBayesBound {
    pub fn new(delta: f64, n_samples: usize) -> Self {
        PACBayesBound { delta, n_samples }
    }
    /// McAllester 2003 bound:
    /// R(Q) <= R_emp(Q) + sqrt( (KL(Q||P) + ln(2*sqrt(n)/delta)) / (2*n) )
    pub fn mcallester(&self, empirical_risk: f64, kl_divergence: f64) -> f64 {
        let n = self.n_samples as f64;
        let inside = (kl_divergence + (2.0 * n.sqrt() / self.delta).ln()) / (2.0 * n);
        empirical_risk + inside.max(0.0).sqrt()
    }
    /// Catoni 2007 bound (simplified, lambda-dependent):
    /// R(Q) <= (1 - e^{-lambda*R_emp(Q)*n}) / lambda  + KL(Q||P)/(lambda*n)
    pub fn catoni(&self, empirical_risk: f64, kl_divergence: f64, lambda: f64) -> f64 {
        let n = self.n_samples as f64;
        if lambda.abs() < 1e-12 {
            return empirical_risk;
        }
        let term1 = (1.0 - (-lambda * empirical_risk * n).exp()) / lambda;
        let term2 = kl_divergence / (lambda * n);
        term1 + term2
    }
    /// KL divergence between two Bernoulli distributions (for {0,1} risks)
    pub fn kl_bernoulli(q: f64, p: f64) -> f64 {
        let eps = 1e-12;
        let q = q.clamp(eps, 1.0 - eps);
        let p = p.clamp(eps, 1.0 - eps);
        q * (q / p).ln() + (1.0 - q) * ((1.0 - q) / (1.0 - p)).ln()
    }
    /// KL divergence between two Gaussians with equal variance sigma^2
    pub fn kl_gaussians(mu_q: f64, mu_p: f64, sigma: f64) -> f64 {
        (mu_q - mu_p).powi(2) / (2.0 * sigma * sigma)
    }
}
pub struct MomentumSGD {
    pub learning_rate: f64,
    pub momentum: f64,
    pub max_epochs: u32,
}
impl MomentumSGD {
    pub fn new(lr: f64, momentum: f64, max_epochs: u32) -> Self {
        MomentumSGD {
            learning_rate: lr,
            momentum,
            max_epochs,
        }
    }
    pub fn minimize_quadratic(&self, a: f64, b: f64, x0: f64) -> f64 {
        let mut x = x0;
        let mut velocity = 0.0;
        for _ in 0..self.max_epochs {
            let grad = 2.0 * a * x + b;
            velocity = self.momentum * velocity - self.learning_rate * grad;
            let new_x = x + velocity;
            if (new_x - x).abs() < 1e-10 {
                return new_x;
            }
            x = new_x;
        }
        x
    }
}
#[derive(Debug, Clone)]
pub struct RandomizedSmoothingClassifier {
    pub sigma: f64,
    pub n_samples: usize,
    pub confidence: f64,
}
impl RandomizedSmoothingClassifier {
    pub fn new(sigma: f64, n_samples: usize, confidence: f64) -> Self {
        RandomizedSmoothingClassifier {
            sigma,
            n_samples,
            confidence,
        }
    }
    /// Smooth prediction: return class with highest vote count after adding
    /// Gaussian noise `n_samples` times.
    pub fn smooth_predict<F: Fn(&[f64]) -> usize>(&self, x: &[f64], base_classifier: &F) -> usize {
        let mut votes: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
        let mut state: u64 = 0xdeadbeefcafe1234_u64;
        let mut lcg = || -> f64 {
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            let u = (state >> 11) as f64 / (1u64 << 53) as f64;
            u.clamp(1e-15, 1.0 - 1e-15)
        };
        for _ in 0..self.n_samples {
            let noisy: Vec<f64> = x
                .chunks(2)
                .flat_map(|chunk| {
                    let u1 = lcg();
                    let u2 = lcg();
                    let r = (-2.0 * u1.ln()).sqrt();
                    let theta = std::f64::consts::TAU * u2;
                    let n1 = r * theta.cos() * self.sigma;
                    let n2 = r * theta.sin() * self.sigma;
                    if chunk.len() == 2 {
                        vec![chunk[0] + n1, chunk[1] + n2]
                    } else {
                        vec![chunk[0] + n1]
                    }
                })
                .take(x.len())
                .collect();
            let cls = base_classifier(&noisy);
            *votes.entry(cls).or_insert(0) += 1;
        }
        votes
            .into_iter()
            .max_by_key(|(_, v)| *v)
            .map(|(c, _)| c)
            .unwrap_or(0)
    }
    /// Certify: return the L2 radius for which the top-class prediction is
    /// guaranteed to hold.  Returns 0.0 if not certifiable.
    pub fn certify<F: Fn(&[f64]) -> usize>(&self, x: &[f64], base_classifier: &F) -> (usize, f64) {
        let mut votes: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
        let mut state: u64 = 0x123456789abcdef0_u64;
        let mut lcg = || -> f64 {
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            let u = (state >> 11) as f64 / (1u64 << 53) as f64;
            u.clamp(1e-15, 1.0 - 1e-15)
        };
        for _ in 0..self.n_samples {
            let noisy: Vec<f64> = x
                .chunks(2)
                .flat_map(|chunk| {
                    let u1 = lcg();
                    let u2 = lcg();
                    let r = (-2.0 * u1.ln()).sqrt();
                    let theta = std::f64::consts::TAU * u2;
                    let n1 = r * theta.cos() * self.sigma;
                    let n2 = r * theta.sin() * self.sigma;
                    if chunk.len() == 2 {
                        vec![chunk[0] + n1, chunk[1] + n2]
                    } else {
                        vec![chunk[0] + n1]
                    }
                })
                .take(x.len())
                .collect();
            let cls = base_classifier(&noisy);
            *votes.entry(cls).or_insert(0) += 1;
        }
        let top = votes
            .iter()
            .max_by_key(|(_, v)| **v)
            .map(|(&c, &v)| (c, v))
            .unwrap_or((0, 0));
        let p_hat = top.1 as f64 / self.n_samples as f64;
        let z = (-((1.0 - self.confidence) / 2.0).ln() * 2.0).sqrt();
        let p_lower =
            (p_hat - z * (p_hat * (1.0 - p_hat) / self.n_samples as f64).sqrt()).clamp(0.0, 1.0);
        if p_lower > 0.5 {
            let radius = self.sigma * Self::probit(p_lower);
            (top.0, radius)
        } else {
            (top.0, 0.0)
        }
    }
    fn probit(p: f64) -> f64 {
        let p = p.clamp(1e-15, 1.0 - 1e-15);
        let t = if p < 0.5 {
            (-2.0 * p.ln()).sqrt()
        } else {
            (-2.0 * (1.0 - p).ln()).sqrt()
        };
        let c = [2.515517, 0.802853, 0.010328];
        let d = [1.432788, 0.189269, 0.001308];
        let num = c[0] + c[1] * t + c[2] * t * t;
        let den = 1.0 + d[0] * t + d[1] * t * t + d[2] * t * t * t;
        let result = t - num / den;
        if p < 0.5 {
            -result
        } else {
            result
        }
    }
}
pub struct KnnClassifier {
    pub k: usize,
    pub data: Vec<(Vec<f64>, usize)>,
}
impl KnnClassifier {
    pub fn new(k: usize) -> Self {
        KnnClassifier {
            k,
            data: Vec::new(),
        }
    }
    pub fn fit(&mut self, data: Vec<(Vec<f64>, usize)>) {
        self.data = data;
    }
    pub fn predict(&self, point: &[f64]) -> usize {
        let mut distances: Vec<(f64, usize)> = self
            .data
            .iter()
            .map(|(features, label)| (Self::euclidean_distance(point, features), *label))
            .collect();
        distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        let k = self.k.min(distances.len());
        let mut votes: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
        for (_, label) in &distances[..k] {
            *votes.entry(*label).or_insert(0) += 1;
        }
        votes
            .into_iter()
            .max_by_key(|(_, v)| *v)
            .map(|(l, _)| l)
            .unwrap_or(0)
    }
    pub fn predict_proba(&self, point: &[f64]) -> std::collections::HashMap<usize, f64> {
        let mut distances: Vec<(f64, usize)> = self
            .data
            .iter()
            .map(|(features, label)| (Self::euclidean_distance(point, features), *label))
            .collect();
        distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        let k = self.k.min(distances.len());
        let mut counts: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
        for (_, label) in &distances[..k] {
            *counts.entry(*label).or_insert(0) += 1;
        }
        let total = k as f64;
        counts
            .into_iter()
            .map(|(label, count)| (label, count as f64 / total))
            .collect()
    }
    fn euclidean_distance(a: &[f64], b: &[f64]) -> f64 {
        a.iter()
            .zip(b.iter())
            .map(|(x, y)| (x - y).powi(2))
            .sum::<f64>()
            .sqrt()
    }
}
pub struct LinearRegression {
    pub weights: Vec<f64>,
    pub bias: f64,
}
impl LinearRegression {
    pub fn new(n_features: usize) -> Self {
        LinearRegression {
            weights: vec![0.0; n_features],
            bias: 0.0,
        }
    }
    pub fn predict(&self, x: &[f64]) -> f64 {
        self.weights
            .iter()
            .zip(x.iter())
            .map(|(w, xi)| w * xi)
            .sum::<f64>()
            + self.bias
    }
    pub fn fit_least_squares(x_data: &[Vec<f64>], y_data: &[f64]) -> Self {
        if x_data.is_empty() || y_data.is_empty() {
            return LinearRegression::new(1);
        }
        let n_features = x_data[0].len();
        let n = x_data.len().min(y_data.len()) as f64;
        let y_mean = y_data.iter().sum::<f64>() / n;
        let x_mean = if n_features > 0 && !x_data.is_empty() {
            x_data.iter().map(|x| x[0]).sum::<f64>() / n
        } else {
            0.0
        };
        let mut cov_xy = 0.0f64;
        let mut var_xx = 0.0f64;
        for (x, y) in x_data.iter().zip(y_data.iter()) {
            let xi = if x.is_empty() { 0.0 } else { x[0] };
            cov_xy += (xi - x_mean) * (y - y_mean);
            var_xx += (xi - x_mean).powi(2);
        }
        let w = if var_xx.abs() > 1e-12 {
            cov_xy / var_xx
        } else {
            0.0
        };
        let b = y_mean - w * x_mean;
        let mut weights = vec![0.0f64; n_features];
        if !weights.is_empty() {
            weights[0] = w;
        }
        LinearRegression { weights, bias: b }
    }
    pub fn r_squared(&self, x_data: &[Vec<f64>], y_data: &[f64]) -> f64 {
        if y_data.is_empty() {
            return 0.0;
        }
        let y_mean = y_data.iter().sum::<f64>() / y_data.len() as f64;
        let ss_tot: f64 = y_data.iter().map(|y| (y - y_mean).powi(2)).sum();
        let ss_res: f64 = x_data
            .iter()
            .zip(y_data.iter())
            .map(|(x, y)| (y - self.predict(x)).powi(2))
            .sum();
        if ss_tot < 1e-12 {
            1.0
        } else {
            1.0 - ss_res / ss_tot
        }
    }
    pub fn mse(&self, x_data: &[Vec<f64>], y_data: &[f64]) -> f64 {
        let n = x_data.len().min(y_data.len());
        if n == 0 {
            return 0.0;
        }
        let sum: f64 = x_data
            .iter()
            .zip(y_data.iter())
            .map(|(x, y)| (y - self.predict(x)).powi(2))
            .sum();
        sum / n as f64
    }
}
#[derive(Debug, Clone)]
pub struct UncertaintySampler {
    pub strategy: UncertaintyStrategy,
}
impl UncertaintySampler {
    pub fn new(strategy: UncertaintyStrategy) -> Self {
        UncertaintySampler { strategy }
    }
    /// Score a probability distribution (softmax output).
    /// Higher score = more uncertain.
    pub fn score(&self, probs: &[f64]) -> f64 {
        match self.strategy {
            UncertaintyStrategy::LeastConfident => {
                let max_p = probs.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                1.0 - max_p
            }
            UncertaintyStrategy::MarginSampling => {
                if probs.len() < 2 {
                    return 0.0;
                }
                let mut sorted = probs.to_vec();
                sorted.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));
                1.0 - (sorted[0] - sorted[1])
            }
            UncertaintyStrategy::Entropy => {
                let eps = 1e-15;
                -probs
                    .iter()
                    .filter(|&&p| p > 0.0)
                    .map(|&p| p * (p + eps).ln())
                    .sum::<f64>()
            }
        }
    }
    /// Given a batch of probability distributions (one per candidate),
    /// return the index of the most uncertain candidate.
    pub fn select_query(&self, candidates: &[Vec<f64>]) -> usize {
        candidates
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| {
                self.score(a)
                    .partial_cmp(&self.score(b))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(i, _)| i)
            .unwrap_or(0)
    }
}
pub struct DecisionStump {
    pub feature_idx: usize,
    pub threshold: f64,
    pub polarity: i32,
}
impl DecisionStump {
    pub fn new(feature_idx: usize, threshold: f64, polarity: i32) -> Self {
        DecisionStump {
            feature_idx,
            threshold,
            polarity,
        }
    }
    pub fn predict(&self, x: &[f64]) -> usize {
        let val = if self.feature_idx < x.len() {
            x[self.feature_idx]
        } else {
            0.0
        };
        if self.polarity > 0 {
            if val >= self.threshold {
                1
            } else {
                0
            }
        } else if val < self.threshold {
            1
        } else {
            0
        }
    }
    pub fn find_best(data: &[(Vec<f64>, usize)], weights: &[f64]) -> Self {
        let n_features = if data.is_empty() { 0 } else { data[0].0.len() };
        let mut best_err = f64::INFINITY;
        let mut best = DecisionStump::new(0, 0.0, 1);
        for feat in 0..n_features {
            let mut values: Vec<f64> = data.iter().map(|(x, _)| x[feat]).collect();
            values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            values.dedup();
            for &thresh in &values {
                for &pol in &[1, -1] {
                    let stump = DecisionStump::new(feat, thresh, pol);
                    let err: f64 = data
                        .iter()
                        .zip(weights.iter())
                        .map(|((x, y), w)| if stump.predict(x) != *y { *w } else { 0.0 })
                        .sum();
                    if err < best_err {
                        best_err = err;
                        best = stump;
                    }
                }
            }
        }
        best
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Activation {
    ReLU,
    Sigmoid,
    Tanh,
    Linear,
    Softmax,
    LeakyReLU,
    ELU,
}
impl Activation {
    pub fn apply(&self, x: f64) -> f64 {
        match self {
            Activation::ReLU => x.max(0.0),
            Activation::Sigmoid => 1.0 / (1.0 + (-x).exp()),
            Activation::Tanh => x.tanh(),
            Activation::Linear => x,
            Activation::Softmax => x,
            Activation::LeakyReLU => {
                if x > 0.0 {
                    x
                } else {
                    0.01 * x
                }
            }
            Activation::ELU => {
                if x > 0.0 {
                    x
                } else {
                    x.exp() - 1.0
                }
            }
        }
    }
    pub fn derivative(&self, x: f64) -> f64 {
        match self {
            Activation::ReLU => {
                if x > 0.0 {
                    1.0
                } else {
                    0.0
                }
            }
            Activation::Sigmoid => {
                let s = self.apply(x);
                s * (1.0 - s)
            }
            Activation::Tanh => 1.0 - x.tanh().powi(2),
            Activation::Linear => 1.0,
            Activation::Softmax => 1.0,
            Activation::LeakyReLU => {
                if x > 0.0 {
                    1.0
                } else {
                    0.01
                }
            }
            Activation::ELU => {
                if x > 0.0 {
                    1.0
                } else {
                    x.exp()
                }
            }
        }
    }
    pub fn apply_softmax(values: &[f64]) -> Vec<f64> {
        if values.is_empty() {
            return vec![];
        }
        let max_val = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let exps: Vec<f64> = values.iter().map(|&v| (v - max_val).exp()).collect();
        let sum: f64 = exps.iter().sum();
        if sum.abs() < 1e-15 {
            vec![1.0 / values.len() as f64; values.len()]
        } else {
            exps.iter().map(|&e| e / sum).collect()
        }
    }
}
