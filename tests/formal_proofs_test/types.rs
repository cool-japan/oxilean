//! Shared types for the formal proofs test suite.

/// A single formal proof test case, consisting of a name, the OxiLean source
/// declaration to type-check, and the expected result category.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ProofTestCase {
    /// Human-readable identifier for the test.
    pub name: &'static str,
    /// OxiLean source text (a single declaration: theorem, def, or axiom).
    pub source: &'static str,
    /// Expected type description (informational; not yet verified against kernel output).
    pub expected_type: &'static str,
}

impl ProofTestCase {
    /// Construct a new proof test case.
    pub const fn new(
        name: &'static str,
        source: &'static str,
        expected_type: &'static str,
    ) -> Self {
        Self {
            name,
            source,
            expected_type,
        }
    }
}

/// Outcome of running a single proof test.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ProofOutcome {
    /// The test case that was run.
    pub case_name: &'static str,
    /// Whether parsing succeeded.
    pub parse_ok: bool,
    /// Whether elaboration succeeded.
    pub elab_ok: bool,
    /// Error message, if any.
    pub error: Option<String>,
}

impl ProofOutcome {
    /// Returns `true` if both parse and elab succeeded.
    pub fn success(&self) -> bool {
        self.parse_ok && self.elab_ok
    }
}

/// Aggregated statistics over a collection of proof test cases.
#[derive(Debug, Default)]
pub struct ProofSuiteStats {
    /// Total number of test cases attempted.
    pub total: usize,
    /// Number of cases where parsing succeeded.
    pub parse_ok: usize,
    /// Number of cases where elaboration also succeeded.
    pub elab_ok: usize,
    /// Failed test case names and their errors.
    pub failures: Vec<(String, String)>,
}

impl ProofSuiteStats {
    /// Parse success rate as a percentage.
    pub fn parse_rate(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            100.0 * self.parse_ok as f64 / self.total as f64
        }
    }

    /// Elaboration success rate as a percentage.
    pub fn elab_rate(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            100.0 * self.elab_ok as f64 / self.total as f64
        }
    }

    /// Merge another stats block into this one.
    #[allow(dead_code)]
    pub fn merge(&mut self, other: ProofSuiteStats) {
        self.total += other.total;
        self.parse_ok += other.parse_ok;
        self.elab_ok += other.elab_ok;
        for f in other.failures {
            if self.failures.len() < 20 {
                self.failures.push(f);
            }
        }
    }
}
