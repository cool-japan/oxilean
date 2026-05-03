//! Types for Dung's abstract argumentation frameworks.

/// Opaque identifier for an argument in a framework.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ArgId(pub usize);

/// An argument with an identifier, a name, a claim, and a list of supporting premises.
#[derive(Debug, Clone)]
pub struct Argument {
    /// Unique identifier within the framework.
    pub id: ArgId,
    /// Human-readable name (e.g., "A", "Nixon_pacifist").
    pub name: String,
    /// The conclusion claimed by this argument.
    pub claim: String,
    /// Premises that support the conclusion.
    pub support: Vec<String>,
}

impl Argument {
    /// Construct a new argument.
    pub fn new(id: usize, name: impl Into<String>, claim: impl Into<String>) -> Self {
        Argument {
            id: ArgId(id),
            name: name.into(),
            claim: claim.into(),
            support: Vec::new(),
        }
    }

    /// Construct a new argument with explicit support list.
    pub fn with_support(
        id: usize,
        name: impl Into<String>,
        claim: impl Into<String>,
        support: Vec<String>,
    ) -> Self {
        Argument {
            id: ArgId(id),
            name: name.into(),
            claim: claim.into(),
            support,
        }
    }
}

/// A directed defeat (attack) relation between two arguments.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attack {
    /// The argument that attacks.
    pub attacker: ArgId,
    /// The argument being attacked.
    pub target: ArgId,
}

impl Attack {
    /// Construct a new attack.
    pub fn new(attacker: usize, target: usize) -> Self {
        Attack {
            attacker: ArgId(attacker),
            target: ArgId(target),
        }
    }
}

/// Dung's Abstract Argumentation Framework: AF = (Args, Att).
#[derive(Debug, Clone, Default)]
pub struct ArgumentationFramework {
    /// All arguments in the framework.
    pub arguments: Vec<Argument>,
    /// All attack relations.
    pub attacks: Vec<Attack>,
}

impl ArgumentationFramework {
    /// Create an empty argumentation framework.
    pub fn new() -> Self {
        ArgumentationFramework {
            arguments: Vec::new(),
            attacks: Vec::new(),
        }
    }

    /// Add an argument and return its id.
    pub fn add_argument(&mut self, name: impl Into<String>, claim: impl Into<String>) -> ArgId {
        let id = ArgId(self.arguments.len());
        self.arguments.push(Argument::new(id.0, name, claim));
        id
    }

    /// Add an attack relation.
    pub fn add_attack(&mut self, attacker: ArgId, target: ArgId) {
        self.attacks.push(Attack { attacker, target });
    }

    /// Retrieve an argument by id.
    pub fn get_argument(&self, id: ArgId) -> Option<&Argument> {
        self.arguments.iter().find(|a| a.id == id)
    }

    /// All argument ids in this framework.
    pub fn all_ids(&self) -> Vec<ArgId> {
        self.arguments.iter().map(|a| a.id).collect()
    }
}

/// A set of arguments accepted under some semantics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Extension {
    /// The accepted arguments.
    pub args: Vec<ArgId>,
}

impl Extension {
    /// Construct from a slice of ids.
    pub fn from_slice(ids: &[ArgId]) -> Self {
        let mut args = ids.to_vec();
        args.sort();
        Extension { args }
    }

    /// Check membership.
    pub fn contains(&self, id: ArgId) -> bool {
        self.args.contains(&id)
    }

    /// Number of arguments.
    pub fn len(&self) -> usize {
        self.args.len()
    }

    /// Whether the extension is empty.
    pub fn is_empty(&self) -> bool {
        self.args.is_empty()
    }
}

/// Argumentation semantics — determines which extensions are acceptable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExtensionSemantics {
    /// Complete extensions: admissible + contains all defended arguments.
    Complete,
    /// Grounded extension: the least complete extension (least fixed point).
    Grounded,
    /// Preferred extensions: maximal admissible sets.
    Preferred,
    /// Stable extensions: conflict-free sets that attack all non-members.
    Stable,
    /// Admissible sets: conflict-free + self-defending.
    Admissible,
    /// CF2 semantics: maximal conflict-free sets in each strongly connected component.
    CF2,
}

/// Acceptance status of a single argument under credulous/skeptical reasoning.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AcceptanceStatus {
    /// The argument is accepted.
    Accepted,
    /// The argument is rejected.
    Rejected,
    /// The argument's status cannot be determined.
    Undecided,
}

/// Result of credulous acceptance query.
#[derive(Debug, Clone)]
pub struct CredulousResult {
    /// Whether the argument is credulously accepted.
    pub status: AcceptanceStatus,
    /// Extensions that contain this argument (witnesses).
    pub supporting_extensions: Vec<Extension>,
}
