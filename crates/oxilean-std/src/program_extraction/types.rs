//! Types for program extraction from constructive proofs (Curry-Howard correspondence).

/// A constructive proposition in intuitionistic logic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConstructiveProp {
    /// Logical truth (⊤).
    True,
    /// Logical falsehood (⊥).
    False,
    /// Conjunction (P ∧ Q).
    And(Box<ConstructiveProp>, Box<ConstructiveProp>),
    /// Disjunction (P ∨ Q).
    Or(Box<ConstructiveProp>, Box<ConstructiveProp>),
    /// Negation (¬P), defined as P → ⊥.
    Not(Box<ConstructiveProp>),
    /// Implication (P → Q).
    Implies(Box<ConstructiveProp>, Box<ConstructiveProp>),
    /// Existential quantification (∃x. P(x)).
    Exists(String, Box<ConstructiveProp>),
    /// Universal quantification (∀x. P(x)).
    Forall(String, Box<ConstructiveProp>),
    /// Atomic proposition with arguments.
    Atom(String, Vec<String>),
    /// Propositional equality of two terms.
    Eq(String, String),
}

/// A proof term in the Curry-Howard correspondence.
/// Each term witnesses a constructive proof of some proposition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProofTerm {
    /// Witness for ⊤ (unit).
    Unit,
    /// Witness for P ∧ Q (pair of proofs).
    Pair(Box<ProofTerm>, Box<ProofTerm>),
    /// First projection of a pair (proof of P from proof of P ∧ Q).
    Fst(Box<ProofTerm>),
    /// Second projection of a pair (proof of Q from proof of P ∧ Q).
    Snd(Box<ProofTerm>),
    /// Left injection into a disjunction (proof of P ∨ Q from proof of P).
    Inl(Box<ProofTerm>),
    /// Right injection into a disjunction (proof of P ∨ Q from proof of Q).
    Inr(Box<ProofTerm>),
    /// Lambda abstraction (proof of P → Q).
    Lambda(String, Box<ProofTerm>),
    /// Function application (modus ponens).
    App(Box<ProofTerm>, Box<ProofTerm>),
    /// Existential witness packing (pack a witness with a proof).
    Pack(String, Box<ProofTerm>),
    /// Existential witness unpacking.
    Unpack {
        /// The variable binding the witness value.
        witness: String,
        /// The variable binding the proof of the body proposition.
        proof_var: String,
        /// The term being unpacked.
        packed: Box<ProofTerm>,
        /// The body using the unpacked witness and proof.
        body: Box<ProofTerm>,
    },
    /// Proof by contradiction / ex falso quodlibet from ⊥.
    Absurd(Box<ProofTerm>),
    /// Variable reference.
    Var(String),
}

/// A program extracted from a constructive proof via Curry-Howard.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtractedProgram {
    /// Name of the extracted program.
    pub name: String,
    /// Type of the input (as a string description).
    pub input_type: String,
    /// Type of the output (as a string description).
    pub output_type: String,
    /// The extracted program body as a proof term.
    pub body: ProofTerm,
    /// The original constructive proposition this program proves.
    pub original_prop: ConstructiveProp,
}

/// A Hoare-style specification for a program.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProgramSpec {
    /// Precondition (must hold before execution).
    pub pre: ConstructiveProp,
    /// Postcondition (must hold after execution).
    pub post: ConstructiveProp,
    /// Name of the specified program.
    pub name: String,
}

/// Result of a program extraction attempt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExtractionResult {
    /// Successful extraction yielding a program.
    Success(ExtractedProgram),
    /// The proposition is not constructive (contains classical reasoning).
    NonConstructive(String),
    /// Extraction failed for some other reason.
    Failure(String),
}
