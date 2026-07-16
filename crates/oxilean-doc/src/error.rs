//! Error type for the `oxilean-doc` multi-file generation pipeline.

use std::fmt;
use std::io;

/// Errors that can occur during documentation generation.
#[derive(Debug)]
pub enum DocError {
    /// An I/O error (file creation, write, etc.).
    Io(io::Error),
}

impl fmt::Display for DocError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DocError::Io(e) => write!(f, "I/O error: {e}"),
        }
    }
}

impl std::error::Error for DocError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            DocError::Io(e) => Some(e),
        }
    }
}

impl From<io::Error> for DocError {
    fn from(e: io::Error) -> Self {
        DocError::Io(e)
    }
}
