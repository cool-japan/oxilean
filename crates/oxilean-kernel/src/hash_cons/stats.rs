//! Statistics for the hash-consing arena.

/// Statistics snapshot emitted by [`HashConsArena::stats`].
///
/// [`HashConsArena::stats`]: super::arena::HashConsArena::stats
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HashConsStats {
    /// Number of `mk_*` calls that found an existing node (cache hit).
    pub hits: u64,
    /// Number of `mk_*` calls that inserted a new node (cache miss).
    pub misses: u64,
    /// Total number of `mk_*` calls (`hits + misses`).
    pub total: u64,
    /// Number of distinct nodes actually stored in the arena.
    pub unique_nodes: usize,
}

impl HashConsStats {
    /// Fraction of calls that were cache hits (`hits / total`).
    ///
    /// Returns `0.0` when `total` is zero.
    pub fn hit_rate(&self) -> f64 {
        if self.total == 0 {
            return 0.0;
        }
        self.hits as f64 / self.total as f64
    }

    /// Fraction of calls that resulted in a new allocation (`misses / total`).
    ///
    /// Returns `0.0` when `total` is zero.
    pub fn dedup_ratio(&self) -> f64 {
        if self.total == 0 {
            return 0.0;
        }
        self.misses as f64 / self.total as f64
    }

    /// Returns `true` when at least one deduplication hit occurred.
    pub fn has_dedup(&self) -> bool {
        self.hits > 0
    }
}

impl std::fmt::Display for HashConsStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "HashConsStats {{ hits: {}, misses: {}, total: {}, unique_nodes: {}, hit_rate: {:.2}% }}",
            self.hits,
            self.misses,
            self.total,
            self.unique_nodes,
            self.hit_rate() * 100.0,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stats_hit_rate_zero_total() {
        let s = HashConsStats {
            hits: 0,
            misses: 0,
            total: 0,
            unique_nodes: 0,
        };
        assert_eq!(s.hit_rate(), 0.0);
    }

    #[test]
    fn test_stats_hit_rate_all_hits() {
        let s = HashConsStats {
            hits: 10,
            misses: 0,
            total: 10,
            unique_nodes: 5,
        };
        assert!((s.hit_rate() - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_stats_dedup_ratio() {
        let s = HashConsStats {
            hits: 7,
            misses: 3,
            total: 10,
            unique_nodes: 3,
        };
        assert!((s.dedup_ratio() - 0.3).abs() < 1e-10);
    }

    #[test]
    fn test_stats_has_dedup_true() {
        let s = HashConsStats {
            hits: 1,
            misses: 9,
            total: 10,
            unique_nodes: 9,
        };
        assert!(s.has_dedup());
    }

    #[test]
    fn test_stats_has_dedup_false_when_no_hits() {
        let s = HashConsStats {
            hits: 0,
            misses: 5,
            total: 5,
            unique_nodes: 5,
        };
        assert!(!s.has_dedup());
    }

    #[test]
    fn test_stats_display_does_not_panic() {
        let s = HashConsStats {
            hits: 3,
            misses: 2,
            total: 5,
            unique_nodes: 2,
        };
        let text = format!("{}", s);
        assert!(text.contains("hits: 3"));
        assert!(text.contains("misses: 2"));
    }
}
