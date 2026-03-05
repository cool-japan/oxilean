//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

/// Custom tactic syntax definition
#[derive(Debug, Clone, PartialEq)]
pub struct CustomTactic {
    /// Tactic name
    pub name: String,
    /// Parameters
    pub params: Vec<String>,
    /// Implementation body
    pub body: String,
}
/// A rewrite rule: a lemma name with an optional reverse flag.
#[derive(Debug, Clone, PartialEq)]
pub struct RewriteRule {
    /// Lemma name
    pub lemma: String,
    /// If true, rewrite right-to-left
    pub reverse: bool,
}
/// Arguments to the `simp` tactic.
#[derive(Debug, Clone, PartialEq)]
pub struct SimpArgs {
    /// Whether `only` was specified
    pub only: bool,
    /// Lemma names in the simp set
    pub lemmas: Vec<String>,
    /// Configuration key-value pairs
    pub config: Vec<(String, String)>,
}
/// A case arm for `cases` / `induction`.
#[derive(Debug, Clone, PartialEq)]
pub struct CaseArm {
    /// Constructor/case name
    pub name: String,
    /// Bound variable names
    pub bindings: Vec<String>,
    /// Tactic for this arm
    pub tactic: TacticExpr,
}
/// Side for `conv` tactic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConvSide {
    /// Convert on the left-hand side
    Lhs,
    /// Convert on the right-hand side
    Rhs,
}
/// A parsed tactic expression.
#[derive(Debug, Clone, PartialEq)]
pub enum TacticExpr {
    /// Basic tactic by name
    Basic(String),
    /// Tactic with arguments
    WithArgs(String, Vec<String>),
    /// Sequence of tactics (t1 ; t2)
    Seq(Box<TacticExpr>, Box<TacticExpr>),
    /// Alternative tactics (t1 <|> t2)
    Alt(Box<TacticExpr>, Box<TacticExpr>),
    /// All goals combinator (t <;> t')
    AllGoals(Box<TacticExpr>, Box<TacticExpr>),
    /// Repeat tactic
    Repeat(Box<TacticExpr>),
    /// Try tactic (don't fail if it fails)
    Try(Box<TacticExpr>),
    /// First successful alternative
    First(Vec<TacticExpr>),
    /// Focus on specific goal
    Focus(usize, Box<TacticExpr>),
    /// All goals
    All(Box<TacticExpr>),
    /// Any goal
    Any(Box<TacticExpr>),
    /// Block of tactics: `{ t1; t2; ... }`
    Block(Vec<TacticExpr>),
    /// Introduce hypotheses: `intro x y z`
    Intro(Vec<String>),
    /// Apply a lemma: `apply lem`
    Apply(String),
    /// Provide an exact proof term: `exact e`
    Exact(String),
    /// Rewrite: `rewrite [lem1, <-lem2]`
    Rewrite(Vec<RewriteRule>),
    /// Simplification: `simp`, `simp only [lem1]`, `simp [*]`
    Simp(SimpArgs),
    /// Case analysis: `cases x with | c1 => t1 | c2 => t2`
    Cases(String, Vec<CaseArm>),
    /// Induction: `induction x with | c => t`
    Induction(String, Vec<CaseArm>),
    /// Local hypothesis: `have h : T := by t`
    Have(String, Option<String>, Box<TacticExpr>),
    /// Local definition: `let x := e`
    Let(String, String),
    /// Show goal form: `show T`
    Show(String),
    /// Suffices: `suffices h : T by t`
    Suffices(String, Box<TacticExpr>),
    /// Calculational proof
    Calc(Vec<CalcStep>),
    /// Conversion mode: `conv_lhs => t` or `conv_rhs => t`
    Conv(ConvSide, Box<TacticExpr>),
    /// omega decision procedure
    Omega,
    /// Ring decision procedure
    Ring,
    /// Decide (boolean decidability)
    Decide,
    /// Normalize numerals
    NormNum,
    /// Apply constructor
    Constructor,
    /// Choose left disjunct
    Left,
    /// Choose right disjunct
    Right,
    /// Existential introduction: `exists e`
    Existsi(String),
    /// Clear hypotheses
    Clear(Vec<String>),
    /// Revert hypotheses
    Revert(Vec<String>),
    /// Substitute a variable
    Subst(String),
    /// Derive a contradiction
    Contradiction,
    /// Switch to proving False
    Exfalso,
    /// Proof by contradiction: `by_contra h`
    ByContra(Option<String>),
    /// Close by assumption
    Assumption,
    /// Close trivially
    Trivial,
    /// Reflexivity
    Rfl,
    /// Intros (shorthand for intro)
    Intros(Vec<String>),
    /// Generalize for induction
    Generalize(String, String),
    /// Induction on pattern
    InductionPat(String, String),
    /// Obtain hypothesis
    Obtain(String, Box<TacticExpr>),
    /// Rcases (recursive cases)
    Rcases(String, Vec<String>),
    /// Tauto (propositional solver)
    Tauto,
    /// Ac_rfl (associative-commutative reflexivity)
    AcRfl,
    /// Custom tactic
    Custom(CustomTactic),
    /// Located tactic (for IDE support)
    Located(Box<TacticExpr>, TacticLocation),
}
/// A single step in a `calc` proof.
#[derive(Debug, Clone, PartialEq)]
pub struct CalcStep {
    /// The relation symbol (e.g. "=", "<=", "<")
    pub relation: String,
    /// The right-hand-side expression
    pub rhs: String,
    /// Justification tactic
    pub justification: TacticExpr,
}
/// Location information for IDE support
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TacticLocation {
    /// Line number (1-based)
    pub line: usize,
    /// Column number (1-based)
    pub column: usize,
    /// Tactic name
    pub name: String,
}
