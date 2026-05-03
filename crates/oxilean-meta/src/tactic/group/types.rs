//! Types for the `group` tactic — group word normalization.

use oxilean_kernel::Expr;

/// Configuration for the `group` tactic.
#[derive(Debug, Clone)]
pub struct GroupConfig {
    /// Maximum number of letters in a reduced word.  Words longer than this
    /// cannot be normalized and will cause the tactic to fail.
    pub max_length: usize,
    /// Maximum number of reduction steps before giving up.
    pub max_steps: usize,
}

impl Default for GroupConfig {
    fn default() -> Self {
        Self {
            max_length: 1024,
            max_steps: 4096,
        }
    }
}

/// A single letter in a group word: either an atom `x` or its inverse `x⁻¹`.
#[derive(Debug, Clone, PartialEq)]
pub struct GroupLetter {
    /// The underlying expression (must be structurally an atom, i.e. `FVar`,
    /// `Const`, or other leaf).
    pub atom: Expr,
    /// `true` means this letter represents `atom⁻¹`.
    pub inverse: bool,
}

impl GroupLetter {
    /// Construct a positive letter.
    pub fn pos(atom: Expr) -> Self {
        Self {
            atom,
            inverse: false,
        }
    }

    /// Construct an inverse letter.
    pub fn inv(atom: Expr) -> Self {
        Self {
            atom,
            inverse: true,
        }
    }

    /// Return the inverse of this letter (flip the `inverse` flag).
    pub fn invert(&self) -> Self {
        Self {
            atom: self.atom.clone(),
            inverse: !self.inverse,
        }
    }
}

/// A reduced group word: a sequence of `GroupLetter`s.
///
/// A word is *reduced* if it contains no adjacent inverse pairs `x · x⁻¹`
/// or `x⁻¹ · x`.
#[derive(Debug, Clone, PartialEq)]
pub struct GroupWord {
    /// The letters of the word in order.
    pub letters: Vec<GroupLetter>,
}

impl GroupWord {
    /// The identity word (empty).
    pub fn identity() -> Self {
        Self {
            letters: Vec::new(),
        }
    }

    /// A word consisting of a single atom.
    pub fn atom(expr: Expr) -> Self {
        Self {
            letters: vec![GroupLetter::pos(expr)],
        }
    }

    /// A word consisting of a single inverse atom.
    pub fn atom_inv(expr: Expr) -> Self {
        Self {
            letters: vec![GroupLetter::inv(expr)],
        }
    }

    /// Concatenate two words (before reduction).
    pub fn concat(mut self, other: GroupWord) -> Self {
        self.letters.extend(other.letters);
        self
    }

    /// Length of the word (number of letters).
    pub fn len(&self) -> usize {
        self.letters.len()
    }

    /// Whether this word is the identity.
    pub fn is_empty(&self) -> bool {
        self.letters.is_empty()
    }
}
