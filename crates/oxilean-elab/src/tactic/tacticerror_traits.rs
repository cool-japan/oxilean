//! # TacticError - Trait Implementations
//!
//! This module contains trait implementations for `TacticError`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//! - `std::error::Error`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TacticError;
use std::fmt;

impl fmt::Display for TacticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TacticError::GoalNotFound(name) => write!(f, "goal '{}' not found", name),
            TacticError::NoGoals => write!(f, "no goals to solve"),
            TacticError::TooManyGoals => write!(f, "too many goals"),
            TacticError::TypeMismatch(msg) => write!(f, "type mismatch: {}", msg),
            TacticError::TypeMismatchDetailed {
                expected,
                actual,
                context,
            } => {
                write!(
                    f,
                    "type mismatch: expected `{}`, found `{}`",
                    expected, actual
                )?;
                if !context.is_empty() {
                    write!(f, " ({})", context)?;
                }
                Ok(())
            }
            TacticError::UnknownTactic(name) => write!(f, "unknown tactic: {}", name),
            TacticError::InvalidArg(msg) => write!(f, "invalid argument: {}", msg),
            TacticError::InternalError(msg) => {
                write!(f, "internal tactic error: {}", msg)
            }
        }
    }
}

impl std::error::Error for TacticError {}

/// Format a tactic failure message including the current proof goal.
///
/// Used by the elaborator to produce actionable error messages that show both
/// *what* went wrong and *where* (i.e., the goal the tactic was attempting to
/// close).
///
/// # Parameters
///
/// - `tactic_name`: The name of the tactic that failed (e.g. `"omega"`, `"ring"`).
/// - `error`:       The [`TacticError`] returned by the tactic.
/// - `goal`:        An optional pretty-printed string of the current proof goal
///   (e.g. `"⊢ 1 = 2"`).  Pass `None` or `Some("")` to omit the
///   goal from the output.
///
/// # Example
///
/// ```
/// use oxilean_elab::tactic::{TacticError, format_tactic_failure};
/// let msg = format_tactic_failure("omega", &TacticError::NoGoals, None);
/// assert!(msg.contains("omega"));
/// assert!(msg.contains("no goals"));
/// ```
pub fn format_tactic_failure(tactic_name: &str, error: &TacticError, goal: Option<&str>) -> String {
    let base = format!("tactic '{}' failed: {}", tactic_name, error);
    match goal {
        Some(g) if !g.is_empty() => format!("{}\n  goal: {}", base, g),
        _ => base,
    }
}
