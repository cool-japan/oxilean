//! Types for the `slim_check` tactic — random counterexample search.

/// Configuration for `slim_check`.
#[derive(Debug, Clone)]
pub struct SlimCheckConfig {
    /// Number of random test cases to try.
    pub num_tests: usize,
    /// Seed for the internal LCG random number generator.
    pub seed: u64,
    /// Maximum value used when generating bounded integers/naturals.
    pub max_size: usize,
}

impl Default for SlimCheckConfig {
    fn default() -> Self {
        Self {
            num_tests: 100,
            seed: 0x_dead_beef_cafe_babe,
            max_size: 100,
        }
    }
}

/// A concrete counterexample found by `slim_check`.
#[derive(Debug, Clone)]
pub struct Counterexample {
    /// The variable assignments that falsify the property, as `(name, value)` pairs.
    pub vars: Vec<(String, String)>,
    /// Human-readable description of the counterexample.
    pub description: String,
}

impl Counterexample {
    /// Format the counterexample as a human-readable string.
    pub fn display(&self) -> String {
        if self.vars.is_empty() {
            self.description.clone()
        } else {
            let assignments: Vec<String> = self
                .vars
                .iter()
                .map(|(k, v)| format!("{} := {}", k, v))
                .collect();
            format!("{} [{}]", self.description, assignments.join(", "))
        }
    }
}

/// The outcome of a `slim_check` run.
#[derive(Debug, Clone)]
pub enum SlimCheckOutcome {
    /// No counterexample was found after all tests.
    NoCounterexample,
    /// A concrete counterexample was found.
    Counterexample(Counterexample),
    /// The tactic gave up (e.g., could not generate valid test inputs).
    GaveUp(usize),
}

/// The full result of a `slim_check` invocation.
#[derive(Debug, Clone)]
pub struct SlimCheckResult {
    /// The overall outcome.
    pub result: SlimCheckOutcome,
    /// How many tests were actually executed.
    pub num_tested: usize,
}

impl SlimCheckResult {
    /// Whether a counterexample was found.
    pub fn has_counterexample(&self) -> bool {
        matches!(self.result, SlimCheckOutcome::Counterexample(_))
    }

    /// Retrieve the counterexample, if present.
    pub fn counterexample(&self) -> Option<&Counterexample> {
        if let SlimCheckOutcome::Counterexample(ref ce) = self.result {
            Some(ce)
        } else {
            None
        }
    }
}
