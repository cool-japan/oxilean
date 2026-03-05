//! # Version - Trait Implementations
//!
//! This module contains trait implementations for `Version`.
//!
//! ## Implemented Traits
//!
//! - `PartialOrd`
//! - `Ord`
//! - `Display`
//! - `FromStr`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::str::FromStr;

use super::types::{Version, VersionParseError};
use std::fmt;

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(std::cmp::Ord::cmp(self, other))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cmp_precedence(other)
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)?;
        if let Some(ref pre) = self.pre {
            write!(f, "-{}", pre)?;
        }
        if let Some(ref bm) = self.build_meta {
            write!(f, "+{}", bm)?;
        }
        Ok(())
    }
}

impl FromStr for Version {
    type Err = VersionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(VersionParseError::Empty);
        }
        let (version_part, build_meta) = if let Some(idx) = s.find('+') {
            (&s[..idx], Some(s[idx + 1..].to_string()))
        } else {
            (s, None)
        };
        let (version_part, pre) = if let Some(idx) = version_part.find('-') {
            (
                &version_part[..idx],
                Some(version_part[idx + 1..].to_string()),
            )
        } else {
            (version_part, None)
        };
        let parts: Vec<&str> = version_part.split('.').collect();
        if parts.len() < 3 {
            return Err(VersionParseError::MissingComponent("patch"));
        }
        let major = parts[0]
            .parse::<u64>()
            .map_err(|_| VersionParseError::InvalidNumber(parts[0].to_string()))?;
        let minor = parts[1]
            .parse::<u64>()
            .map_err(|_| VersionParseError::InvalidNumber(parts[1].to_string()))?;
        let patch = parts[2]
            .parse::<u64>()
            .map_err(|_| VersionParseError::InvalidNumber(parts[2].to_string()))?;
        Ok(Version {
            major,
            minor,
            patch,
            pre,
            build_meta,
        })
    }
}
