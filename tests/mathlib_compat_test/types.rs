//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

/// Statistics from a compatibility run.
#[derive(Debug, Default)]
pub(super) struct CompatStats {
    /// Total declarations attempted
    pub(super) total: usize,
    /// Declarations that parsed successfully
    pub(super) parsed_ok: usize,
    /// Files processed
    pub(super) files_processed: usize,
    /// Representative failures (snippet, error category)
    pub(super) failures: Vec<(String, String)>,
}
impl CompatStats {
    pub(super) fn success_rate(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            100.0 * self.parsed_ok as f64 / self.total as f64
        }
    }
    pub(super) fn merge(&mut self, other: CompatStats) {
        self.total += other.total;
        self.parsed_ok += other.parsed_ok;
        self.files_processed += other.files_processed;
        for f in other.failures.into_iter().take(3) {
            if self.failures.len() < 5 {
                self.failures.push(f);
            }
        }
    }
}
