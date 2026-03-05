//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{Declaration, Environment, Expr, Name};

use super::types::{
    CanonicalFormReducer, DyadicSurreal, Game, GameNode, GameOutcome, GameSum, GameTemperature,
    GrundySequenceCache, HackenbushGame, LoopyGameGraph, MisereAnalyzer, NimGame, NimValue,
    NimValueExt, NimberArithmetic, PartizanTemperature, SurrealNumber, ThermographData,
    WythoffPositions,
};

pub fn app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
pub fn cst(s: &str) -> Expr {
    Expr::Const(Name::str(s), vec![])
}
pub fn prop() -> Expr {
    Expr::Sort(oxilean_kernel::Level::zero())
}
pub fn type0() -> Expr {
    Expr::Sort(oxilean_kernel::Level::succ(oxilean_kernel::Level::zero()))
}
pub fn nat_ty() -> Expr {
    cst("Nat")
}
pub fn arrow(a: Expr, b: Expr) -> Expr {
    Expr::Pi(
        oxilean_kernel::BinderInfo::Default,
        Name::str("_"),
        Box::new(a),
        Box::new(b),
    )
}
/// `Game : Type` — combinatorial game with Left and Right move sets.
pub fn game_ty() -> Expr {
    type0()
}
/// `SurrealNumber : Type` — surreal number type.
pub fn surreal_number_ty() -> Expr {
    type0()
}
/// `GameValue : Type` — Grundy value (nimber) type.
pub fn game_value_ty() -> Expr {
    arrow(nat_ty(), nat_ty())
}
/// `PartizanGame : Type` — game where Left and Right have different move sets.
pub fn partizan_game_ty() -> Expr {
    type0()
}
/// `sprague_grundy : ∀ (g : Game), ∃ (n : Nat), GameEquiv g (Nim n)` —
/// every impartial game is equivalent to a Nim heap.
pub fn sprague_grundy_ty() -> Expr {
    arrow(game_ty(), prop())
}
/// `nim_value_sum : ∀ (a b : Nat), GrundyValue (NimSum a b) = XorNat a b` —
/// Grundy value of sum equals XOR of values (Nim addition).
pub fn nim_value_sum_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `surreal_completeness : OrderedField SurrealNumber` —
/// the surreal numbers form a complete ordered field.
pub fn surreal_completeness_ty() -> Expr {
    app(cst("OrderedField"), surreal_number_ty())
}
/// `combinatorial_game_group : CommGroup GameType` —
/// combinatorial games form an abelian group under disjunctive sum.
pub fn combinatorial_game_group_ty() -> Expr {
    app(cst("CommGroup"), game_ty())
}
/// `NimberField : Prop` — nimbers (ordinal XOR arithmetic) form a field
/// under nim-addition and nim-multiplication.
pub fn nimber_field_ty() -> Expr {
    app(cst("Field"), cst("Nimber"))
}
/// `nim_product : Nat → Nat → Nat` — nim-product (nimber multiplication) of two
/// Grundy values using XOR-based field arithmetic.
pub fn nim_product_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), nat_ty()))
}
/// `surreal_birthday : SurrealNumber → Nat` — the birthday (ordinal creation day)
/// of a surreal number.
pub fn surreal_birthday_ty() -> Expr {
    arrow(surreal_number_ty(), nat_ty())
}
/// `surreal_le : SurrealNumber → SurrealNumber → Prop` — the canonical total order
/// on surreal numbers (x ≤ y iff no right option of x is ≤ x and no left option of
/// y is ≥ y).
pub fn surreal_le_ty() -> Expr {
    arrow(surreal_number_ty(), arrow(surreal_number_ty(), prop()))
}
/// `surreal_add : SurrealNumber → SurrealNumber → SurrealNumber` — surreal addition
/// defined by transfinite induction.
pub fn surreal_add_ty() -> Expr {
    arrow(
        surreal_number_ty(),
        arrow(surreal_number_ty(), surreal_number_ty()),
    )
}
/// `surreal_mul : SurrealNumber → SurrealNumber → SurrealNumber` — surreal
/// multiplication defined by transfinite induction.
pub fn surreal_mul_ty() -> Expr {
    arrow(
        surreal_number_ty(),
        arrow(surreal_number_ty(), surreal_number_ty()),
    )
}
/// `surreal_neg : SurrealNumber → SurrealNumber` — negation of a surreal number
/// (swap left and right sets and negate each element).
pub fn surreal_neg_ty() -> Expr {
    arrow(surreal_number_ty(), surreal_number_ty())
}
/// `GamePositive : Game → Prop` — a game G is positive (Left wins regardless of who
/// moves first) iff G > 0.
pub fn game_positive_ty() -> Expr {
    arrow(game_ty(), prop())
}
/// `GameNegative : Game → Prop` — a game G is negative (Right wins regardless of
/// who moves first) iff G < 0.
pub fn game_negative_ty() -> Expr {
    arrow(game_ty(), prop())
}
/// `GameFuzzy : Game → Prop` — a game G is fuzzy (first player wins from either side)
/// iff G ∥ 0.
pub fn game_fuzzy_ty() -> Expr {
    arrow(game_ty(), prop())
}
/// `GameZero : Game → Prop` — a game G is zero (second player wins) iff G = 0.
pub fn game_zero_ty() -> Expr {
    arrow(game_ty(), prop())
}
/// `GrundyValue : Game → Nat` — the Sprague-Grundy value (nimber) of an
/// impartial game, computed as the mex of successor Grundy values.
pub fn grundy_value_ty() -> Expr {
    arrow(game_ty(), nat_ty())
}
/// `Mex : (Nat → Prop) → Nat` — minimum excludant: smallest natural number
/// not in the given set; used to compute Grundy values.
pub fn mex_ty() -> Expr {
    arrow(arrow(nat_ty(), prop()), nat_ty())
}
/// `WythoffGame : Nat → Nat → Prop` — Wythoff's game P-position predicate.
/// `WythoffGame m n = True` iff `(m, n)` is a P-position in Wythoff's queens game.
pub fn wythoff_game_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `BeattySequence : Nat → Nat → Nat` — the `n`-th term of the Beatty sequence
/// `⌊n·α⌋` for the given rational approximation parameter.
pub fn beatty_sequence_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), nat_ty()))
}
/// `beatty_partition : ∀ (r s : Nat), (1/r + 1/s = 1) → ∀ n, exactly one of
/// ⌊n·r⌋, ⌊n·s⌋ equals n` — Beatty's theorem on complementary sequences.
pub fn beatty_partition_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `MisereGame : Game → Prop` — predicate identifying misère-play convention
/// (last player to move loses).
pub fn misere_game_ty() -> Expr {
    arrow(game_ty(), prop())
}
/// `MisereQuotient : Game → Nat` — misère quotient monoid element for a game.
pub fn misere_quotient_ty() -> Expr {
    arrow(game_ty(), nat_ty())
}
/// `LoopyGame : Type` — a loopy combinatorial game (may have infinite play lines).
pub fn loopy_game_ty() -> Expr {
    type0()
}
/// `OctalGame : Nat → Nat → Nat` — nim-value at position `n` of the octal game
/// with code `code` (Dawson/Grundy-Smith octal notation).
pub fn octal_game_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), nat_ty()))
}
/// `GameCool : Game → SurrealNumber → Game` — cooling a game G by temperature `t`
/// produces the game G_t with all incentives reduced.
pub fn game_cool_ty() -> Expr {
    arrow(game_ty(), arrow(surreal_number_ty(), game_ty()))
}
/// `GameWarm : Game → SurrealNumber → Game` — warming (heating) a game G by `t`.
pub fn game_warm_ty() -> Expr {
    arrow(game_ty(), arrow(surreal_number_ty(), game_ty()))
}
/// `GameMean : Game → SurrealNumber` — the mean value of a hot game G (half-integer
/// or integer describing its "average" outcome under optimal play).
pub fn game_mean_ty() -> Expr {
    arrow(game_ty(), surreal_number_ty())
}
/// `AtomicWeight : Game → SurrealNumber` — atomic weight of an all-small game
/// (measures how far from star-like the game is).
pub fn atomic_weight_ty() -> Expr {
    arrow(game_ty(), surreal_number_ty())
}
/// `UpGame : Game` — the infinitesimal game ↑ = {0 | *}; positive but smaller
/// than any positive number.
pub fn up_game_ty() -> Expr {
    game_ty()
}
/// `DownGame : Game` — the infinitesimal game ↓ = {* | 0}; negative but larger
/// than any negative number.
pub fn down_game_ty() -> Expr {
    game_ty()
}
/// `StarGame : Game` — the first player wins fuzzy game * = {0 | 0}.
pub fn star_game_ty() -> Expr {
    game_ty()
}
/// `SwitchGame : SurrealNumber → SurrealNumber → Game` — the hot game {a | b}
/// where a > b, representing a temperature `(a-b)/2` position.
pub fn switch_game_ty() -> Expr {
    arrow(surreal_number_ty(), arrow(surreal_number_ty(), game_ty()))
}
/// `NortonProduct : Game → Game → Game` — Norton product G × H defined via
/// the "atomic weight" operation for partizan game analysis.
pub fn norton_product_ty() -> Expr {
    arrow(game_ty(), arrow(game_ty(), game_ty()))
}
/// `ReducedCanonicalForm : Game → Game` — the reduced canonical form of a game,
/// eliminating dominated and reversible options.
pub fn reduced_canonical_form_ty() -> Expr {
    arrow(game_ty(), game_ty())
}
/// `TinyGame : SurrealNumber → Game` — the tiny game `{0 | {0 | -x}}` for x > 0,
/// an infinitesimal positive game smaller than any positive number.
pub fn tiny_game_ty() -> Expr {
    arrow(surreal_number_ty(), game_ty())
}
/// `MinyGame : SurrealNumber → Game` — the miny game `{{x | 0} | 0}` for x > 0,
/// the negative counterpart of tiny.
pub fn miny_game_ty() -> Expr {
    arrow(surreal_number_ty(), game_ty())
}
/// `AllSmallGame : Game → Prop` — predicate for all-small games (every subposition
/// has moves for both Left and Right, or is terminal).
pub fn all_small_game_ty() -> Expr {
    arrow(game_ty(), prop())
}
/// `game_birthday : Game → Nat` — birthday of a game (earliest day of creation in
/// the Conway surreal/game construction).
pub fn game_birthday_ty() -> Expr {
    arrow(game_ty(), nat_ty())
}
/// `Thermograph : Game → Prop` — existence of the thermograph (hot-game temperature
/// profile) for a game G.
pub fn thermograph_ty() -> Expr {
    arrow(game_ty(), prop())
}
/// `game_le_total : ∀ (G H : Game), G ≤ H ∨ H ≤ G` — the partial order on games
/// is total when restricted to surreal values.
pub fn game_le_total_ty() -> Expr {
    arrow(game_ty(), arrow(game_ty(), prop()))
}
/// `canonical_form_unique : ∀ (G H : Game), GameEquiv G H → CanonForm G = CanonForm H` —
/// canonical form is well-defined on equivalence classes.
pub fn canonical_form_unique_ty() -> Expr {
    arrow(game_ty(), arrow(game_ty(), prop()))
}
/// `FractionGame : Nat → Nat → Game` — the fraction game `m/2^n` (dyadic rational
/// game value).
pub fn fraction_game_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), game_ty()))
}
/// `OrdinalGame : Nat → Game` — game corresponding to the ordinal α, where
/// ordinal ω is represented by the surreal {0,1,2,...|} (no right options).
pub fn ordinal_game_ty() -> Expr {
    arrow(nat_ty(), game_ty())
}
/// `NimberAdd : Nat → Nat → Nat` — nim addition (XOR) of two ordinals/nimbers,
/// identical to bitwise XOR under the Sprague-Grundy correspondence.
pub fn nimber_add_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), nat_ty()))
}
/// `NimberMul : Nat → Nat → Nat` — nim multiplication of two nimbers via
/// the recursive Fermat-2-power field structure.
pub fn nimber_mul_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), nat_ty()))
}
/// `NimberInv : Nat → Nat` — multiplicative inverse in the nimber field (for non-zero).
pub fn nimber_inv_ty() -> Expr {
    arrow(nat_ty(), nat_ty())
}
/// `SpragueMisere : Game → Nat` — misère Grundy value under misère-play convention;
/// equals normal Grundy value except for all-zero positions.
pub fn sprague_misere_ty() -> Expr {
    arrow(game_ty(), nat_ty())
}
/// `MisereQuotientMonoid : Game → Prop` — the misère quotient monoid structure
/// on games, where game equivalence is determined by misère play outcome.
pub fn misere_quotient_monoid_ty() -> Expr {
    arrow(game_ty(), prop())
}
/// `GameTemperatureMap : Game → surreal_number` — temperature function mapping
/// a partizan game to its temperature (the largest t where cooling changes strategy).
pub fn game_temperature_map_ty() -> Expr {
    arrow(game_ty(), surreal_number_ty())
}
/// `CoolingOperator : SurrealNumber → Game → Game` — the cooling operator C_t
/// taking a game G to its cooled version G_t = {GL_t - t | GR_t + t}.
pub fn cooling_operator_ty() -> Expr {
    arrow(surreal_number_ty(), arrow(game_ty(), game_ty()))
}
/// `GameIncentive : Game → SurrealNumber` — the Left incentive of a game G,
/// defined as G^L - G (the gain from making a Left move).
pub fn game_incentive_ty() -> Expr {
    arrow(game_ty(), surreal_number_ty())
}
/// `ThermographLeft : Game → SurrealNumber → SurrealNumber` — Left scaffold of the
/// thermograph at temperature t (Left's cooled optimal value).
pub fn thermograph_left_ty() -> Expr {
    arrow(game_ty(), arrow(surreal_number_ty(), surreal_number_ty()))
}
/// `ThermographRight : Game → SurrealNumber → SurrealNumber` — Right scaffold of
/// the thermograph at temperature t.
pub fn thermograph_right_ty() -> Expr {
    arrow(game_ty(), arrow(surreal_number_ty(), surreal_number_ty()))
}
/// `GameAtomicWeight : Game → SurrealNumber` — atomic weight of an all-small game,
/// measuring how many free moves of advantage Left has asymptotically.
pub fn game_atomic_weight_ty() -> Expr {
    arrow(game_ty(), surreal_number_ty())
}
/// `AtomicWeightLaw : ∀ G H : Game, AtomicWeight(G+H) = AtomicWeight(G) + AtomicWeight(H)` —
/// the atomic weight is additive over disjunctive sum.
pub fn atomic_weight_law_ty() -> Expr {
    arrow(game_ty(), arrow(game_ty(), prop()))
}
/// `DicotGame : Game → Prop` — a game is dicot if every position (including itself)
/// has moves for both players or is terminal.
pub fn dicot_game_ty() -> Expr {
    arrow(game_ty(), prop())
}
/// `StarInversion : Game → Game` — star inversion of a game G: adding * to G
/// switches the outcome class from P/N to N/P while preserving fuzzy outcomes.
pub fn star_inversion_ty() -> Expr {
    arrow(game_ty(), game_ty())
}
/// `GamePartialOrder : Game → Game → Prop` — the partial order G ≥ H on games:
/// Left prefers G to H in any environment (G - H ≥ 0).
pub fn game_partial_order_ty() -> Expr {
    arrow(game_ty(), arrow(game_ty(), prop()))
}
/// `GameParallelFuzzy : Game → Prop` — G ∥ 0 (G is fuzzy, incomparable to zero)
/// meaning the first player wins regardless of who moves first.
pub fn game_parallel_fuzzy_ty() -> Expr {
    arrow(game_ty(), prop())
}
/// `LoopyGameDraw : LoopyGame → Prop` — predicate for a loopy game that can result
/// in a draw (infinite play with neither player winning).
pub fn loopy_game_draw_ty() -> Expr {
    arrow(loopy_game_ty(), prop())
}
/// `LoopyGameStopper : LoopyGame → Prop` — a stopper is a loopy game where the
/// second player can always force the game to stop.
pub fn loopy_game_stopper_ty() -> Expr {
    arrow(loopy_game_ty(), prop())
}
/// `SurrealOmega : SurrealNumber` — the first infinite surreal number ω = {1,2,3,...|}
/// (no right options), corresponding to the ordinal ω.
pub fn surreal_omega_ty() -> Expr {
    surreal_number_ty()
}
/// `SurrealEpsilon : SurrealNumber` — the surreal ε = 1/ω, the smallest positive
/// infinitesimal surreal: ε = {0 | 1, 1/2, 1/4, ...}.
pub fn surreal_epsilon_ty() -> Expr {
    surreal_number_ty()
}
/// `SurrealDivide : SurrealNumber → SurrealNumber → SurrealNumber` — surreal division
/// s / t (for t ≠ 0), defined via the surreal multiplicative inverse.
pub fn surreal_divide_ty() -> Expr {
    arrow(
        surreal_number_ty(),
        arrow(surreal_number_ty(), surreal_number_ty()),
    )
}
/// `ShortGame : Game → Prop` — a game is short if it has finitely many positions
/// (no infinite play paths), ensuring canonical form exists.
pub fn short_game_ty() -> Expr {
    arrow(game_ty(), prop())
}
/// `GameBirthdayBound : Game → Nat → Prop` — G was born on day ≤ n, meaning G
/// can be constructed in n rounds of the surreal/game construction.
pub fn game_birthday_bound_ty() -> Expr {
    arrow(game_ty(), arrow(nat_ty(), prop()))
}
/// `HotGame : Game → Prop` — a game is hot if its temperature is strictly positive,
/// meaning both players have incentive to move first.
pub fn hot_game_ty() -> Expr {
    arrow(game_ty(), prop())
}
/// `ColdGame : Game → Prop` — a game is cold (temperature ≤ 0), meaning neither
/// player has incentive to move from the "average" perspective.
pub fn cold_game_ty() -> Expr {
    arrow(game_ty(), prop())
}
/// `DoubleUp : Game` — the game ⇑ = {↑ | *} = {{0|*}|*}, twice-up; used in
/// temperature theory to characterize all-small games.
pub fn double_up_ty() -> Expr {
    game_ty()
}
/// `GameAlphaBeta : Game → Nat → Nat → Nat` — alpha-beta pruning score for game G
/// with window [alpha, beta], returning the minimax value within that window.
pub fn game_alpha_beta_ty() -> Expr {
    arrow(game_ty(), arrow(nat_ty(), arrow(nat_ty(), nat_ty())))
}
/// `MinimaxValue : Game → Nat` — the full minimax value of a game tree G,
/// computed by exhaustive search (no alpha-beta pruning).
pub fn minimax_value_ty() -> Expr {
    arrow(game_ty(), nat_ty())
}
/// `GameTreeDepth : Game → Nat` — depth of a game tree: length of the longest
/// play line from the root to a terminal position.
pub fn game_tree_depth_ty() -> Expr {
    arrow(game_ty(), nat_ty())
}
/// `CanonFormBirthday : Game → Nat` — the birthday of the canonical form of G;
/// equals the birthday of G (canonical form is on the same or earlier day).
pub fn canon_form_birthday_ty() -> Expr {
    arrow(game_ty(), nat_ty())
}
/// `NortonProductLaw : Game → Game → Prop` — Norton product law: G×H satisfies
/// the left-right switch property characterizing Norton's operation.
pub fn norton_product_law_ty() -> Expr {
    arrow(game_ty(), arrow(game_ty(), prop()))
}
/// `GameSumAssoc : ∀ G H K : Game, (G+H)+K ≡ G+(H+K)` — associativity of
/// the disjunctive sum of combinatorial games.
pub fn game_sum_assoc_ty() -> Expr {
    arrow(game_ty(), arrow(game_ty(), arrow(game_ty(), prop())))
}
/// `GameNegInverse : ∀ G : Game, G + (-G) ≡ 0` — the negative of a game is its
/// additive inverse under disjunctive sum.
pub fn game_neg_inverse_ty() -> Expr {
    arrow(game_ty(), prop())
}
/// `InfinitesimalGame : Game → Prop` — a game G is infinitesimal if |G| < 1/n for
/// all positive integers n (G is between -ε and ε for all rational ε > 0).
pub fn infinitesimal_game_ty() -> Expr {
    arrow(game_ty(), prop())
}
/// `UpnGame : Nat → Game` — the n-th up game ↑ⁿ = n·↑, the n-fold disjunctive sum
/// of the game ↑ = {0|*}.
pub fn upn_game_ty() -> Expr {
    arrow(nat_ty(), game_ty())
}
/// `GameCongruence : Game → Game → Game → Prop` — G ≡ H (mod J): games G and H
/// are congruent modulo J (differ by a multiple of J in the quotient group).
pub fn game_congruence_ty() -> Expr {
    arrow(game_ty(), arrow(game_ty(), arrow(game_ty(), prop())))
}
/// `MultipleGame : Nat → Game → Game` — the n-fold disjunctive sum n·G of a game G
/// with itself.
pub fn multiple_game_ty() -> Expr {
    arrow(nat_ty(), arrow(game_ty(), game_ty()))
}
/// `OrdinalSumGame : Game → Game → Game` — ordinal sum G:H of games (Left in G
/// can also choose to move in H; Right in G sees only G options unless G is empty).
pub fn ordinal_sum_game_ty() -> Expr {
    arrow(game_ty(), arrow(game_ty(), game_ty()))
}
/// Populate `env` with all combinatorial game theory axioms and theorems.
pub fn build_combinatorial_game_env(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("Game", game_ty()),
        ("SurrealNumber", surreal_number_ty()),
        ("GameValue", game_value_ty()),
        ("PartizanGame", partizan_game_ty()),
        ("sprague_grundy", sprague_grundy_ty()),
        ("nim_value_sum", nim_value_sum_ty()),
        ("surreal_completeness", surreal_completeness_ty()),
        ("combinatorial_game_group", combinatorial_game_group_ty()),
        ("Nim", arrow(nat_ty(), game_ty())),
        ("NimSum", arrow(nat_ty(), arrow(nat_ty(), nat_ty()))),
        ("XorNat", arrow(nat_ty(), arrow(nat_ty(), nat_ty()))),
        ("GameEquiv", arrow(game_ty(), arrow(game_ty(), prop()))),
        ("OrderedField", arrow(type0(), prop())),
        ("CommGroup", arrow(type0(), prop())),
        (
            "DisjunctiveSum",
            arrow(game_ty(), arrow(game_ty(), game_ty())),
        ),
        ("GameTemperature", arrow(game_ty(), cst("Real"))),
        ("Nimber", type0()),
        ("Field", arrow(type0(), prop())),
        ("nimber_field", nimber_field_ty()),
        ("nim_product", nim_product_ty()),
        ("surreal_birthday", surreal_birthday_ty()),
        ("surreal_le", surreal_le_ty()),
        ("surreal_add", surreal_add_ty()),
        ("surreal_mul", surreal_mul_ty()),
        ("surreal_neg", surreal_neg_ty()),
        ("GamePositive", game_positive_ty()),
        ("GameNegative", game_negative_ty()),
        ("GameFuzzy", game_fuzzy_ty()),
        ("GameZero", game_zero_ty()),
        ("GrundyValue", grundy_value_ty()),
        ("Mex", mex_ty()),
        ("WythoffGame", wythoff_game_ty()),
        ("BeattySequence", beatty_sequence_ty()),
        ("beatty_partition", beatty_partition_ty()),
        ("MisereGame", misere_game_ty()),
        ("MisereQuotient", misere_quotient_ty()),
        ("LoopyGame", loopy_game_ty()),
        ("OctalGame", octal_game_ty()),
        ("GameCool", game_cool_ty()),
        ("GameWarm", game_warm_ty()),
        ("GameMean", game_mean_ty()),
        ("AtomicWeight", atomic_weight_ty()),
        ("UpGame", up_game_ty()),
        ("DownGame", down_game_ty()),
        ("StarGame", star_game_ty()),
        ("SwitchGame", switch_game_ty()),
        ("NortonProduct", norton_product_ty()),
        ("ReducedCanonicalForm", reduced_canonical_form_ty()),
        ("TinyGame", tiny_game_ty()),
        ("MinyGame", miny_game_ty()),
        ("AllSmallGame", all_small_game_ty()),
        ("game_birthday", game_birthday_ty()),
        ("Thermograph", thermograph_ty()),
        ("game_le_total", game_le_total_ty()),
        ("canonical_form_unique", canonical_form_unique_ty()),
        ("FractionGame", fraction_game_ty()),
        ("OrdinalGame", ordinal_game_ty()),
        ("NimberAdd", nimber_add_ty()),
        ("NimberMul", nimber_mul_ty()),
        ("NimberInv", nimber_inv_ty()),
        ("SpragueMisere", sprague_misere_ty()),
        ("MisereQuotientMonoid", misere_quotient_monoid_ty()),
        ("GameTemperatureMap", game_temperature_map_ty()),
        ("CoolingOperator", cooling_operator_ty()),
        ("GameIncentive", game_incentive_ty()),
        ("ThermographLeft", thermograph_left_ty()),
        ("ThermographRight", thermograph_right_ty()),
        ("GameAtomicWeight", game_atomic_weight_ty()),
        ("atomic_weight_law", atomic_weight_law_ty()),
        ("DicotGame", dicot_game_ty()),
        ("StarInversion", star_inversion_ty()),
        ("GamePartialOrder", game_partial_order_ty()),
        ("GameParallelFuzzy", game_parallel_fuzzy_ty()),
        ("LoopyGameDraw", loopy_game_draw_ty()),
        ("LoopyGameStopper", loopy_game_stopper_ty()),
        ("SurrealOmega", surreal_omega_ty()),
        ("SurrealEpsilon", surreal_epsilon_ty()),
        ("SurrealDivide", surreal_divide_ty()),
        ("ShortGame", short_game_ty()),
        ("GameBirthdayBound", game_birthday_bound_ty()),
        ("HotGame", hot_game_ty()),
        ("ColdGame", cold_game_ty()),
        ("DoubleUp", double_up_ty()),
        ("GameAlphaBeta", game_alpha_beta_ty()),
        ("MinimaxValue", minimax_value_ty()),
        ("GameTreeDepth", game_tree_depth_ty()),
        ("CanonFormBirthday", canon_form_birthday_ty()),
        ("NortonProductLaw", norton_product_law_ty()),
        ("game_sum_assoc", game_sum_assoc_ty()),
        ("game_neg_inverse", game_neg_inverse_ty()),
        ("InfinitesimalGame", infinitesimal_game_ty()),
        ("UpnGame", upn_game_ty()),
        ("GameCongruence", game_congruence_ty()),
        ("MultipleGame", multiple_game_ty()),
        ("OrdinalSumGame", ordinal_sum_game_ty()),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_nim_value_of_heap() {
        assert_eq!(NimValue::of_heap(0).0, 0);
        assert_eq!(NimValue::of_heap(5).0, 5);
        assert_eq!(NimValue::of_heap(u64::MAX).0, u64::MAX);
    }
    #[test]
    fn test_nim_sum_xor() {
        assert_eq!(NimValue::nim_sum(NimValue(3), NimValue(5)).0, 3 ^ 5);
        assert_eq!(NimValue::nim_sum(NimValue(0), NimValue(7)).0, 7);
        assert_eq!(NimValue::nim_sum(NimValue(6), NimValue(6)).0, 0);
    }
    #[test]
    fn test_nim_value_p_n_positions() {
        assert!(NimValue(0).is_zero());
        assert!(!NimValue(0).is_nonzero());
        assert!(!NimValue(3).is_zero());
        assert!(NimValue(3).is_nonzero());
    }
    #[test]
    fn test_game_zero_star() {
        let z = Game::zero();
        assert!(z.is_zero());
        assert!(!z.is_fuzzy());
        let s = Game::star();
        assert!(!s.is_zero());
        assert!(s.is_fuzzy());
    }
    #[test]
    fn test_game_integer() {
        let g2 = Game::integer(2);
        assert_eq!(g2.left_options, vec![1]);
        assert!(g2.right_options.is_empty());
        let gn2 = Game::integer(-2);
        assert!(gn2.left_options.is_empty());
        assert_eq!(gn2.right_options, vec![-1]);
        let g0 = Game::integer(0);
        assert!(g0.is_zero());
    }
    #[test]
    fn test_game_temperature() {
        let g = Game::new(vec![1], vec![-1]);
        assert!((g.temperature() - 1.0).abs() < 1e-10);
        let z = Game::zero();
        assert!((z.temperature() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_nim_game_grundy_and_winning_move() {
        let g1 = NimGame::new(vec![3]);
        assert_eq!(g1.grundy_value().0, 3);
        assert!(!g1.is_p_position());
        let mv = g1.winning_move();
        assert!(mv.is_some());
        let (idx, new_size) = mv.expect("mv should be valid");
        assert_eq!(idx, 0);
        assert_eq!(new_size, 0);
        let g2 = NimGame::new(vec![4, 4]);
        assert_eq!(g2.grundy_value().0, 0);
        assert!(g2.is_p_position());
        assert!(g2.winning_move().is_none());
    }
    #[test]
    fn test_surreal_number_integer() {
        let z = SurrealNumber::zero();
        assert!(z.is_integer());
        assert_eq!(z.to_rational(), Some((0, 1)));
        let one = SurrealNumber::one();
        assert!(one.is_integer());
        let neg = SurrealNumber::neg_one();
        assert!(neg.is_integer());
        let n = SurrealNumber::from_integer(3);
        assert!(n.is_integer());
    }
    #[test]
    fn test_build_combinatorial_game_env() {
        let mut env = Environment::new();
        build_combinatorial_game_env(&mut env);
        assert!(env.get(&Name::str("Game")).is_some());
        assert!(env.get(&Name::str("sprague_grundy")).is_some());
        assert!(env.get(&Name::str("nim_value_sum")).is_some());
        assert!(env.get(&Name::str("SurrealNumber")).is_some());
        assert!(env.get(&Name::str("nimber_field")).is_some());
        assert!(env.get(&Name::str("nim_product")).is_some());
        assert!(env.get(&Name::str("GrundyValue")).is_some());
        assert!(env.get(&Name::str("Mex")).is_some());
        assert!(env.get(&Name::str("WythoffGame")).is_some());
        assert!(env.get(&Name::str("MisereGame")).is_some());
        assert!(env.get(&Name::str("LoopyGame")).is_some());
        assert!(env.get(&Name::str("GameCool")).is_some());
        assert!(env.get(&Name::str("GameMean")).is_some());
        assert!(env.get(&Name::str("AtomicWeight")).is_some());
        assert!(env.get(&Name::str("UpGame")).is_some());
        assert!(env.get(&Name::str("DownGame")).is_some());
        assert!(env.get(&Name::str("NortonProduct")).is_some());
        assert!(env.get(&Name::str("ReducedCanonicalForm")).is_some());
        assert!(env.get(&Name::str("TinyGame")).is_some());
        assert!(env.get(&Name::str("AllSmallGame")).is_some());
        assert!(env.get(&Name::str("Thermograph")).is_some());
        assert!(env.get(&Name::str("FractionGame")).is_some());
    }
    #[test]
    fn test_dyadic_surreal_basic() {
        let z = DyadicSurreal::zero();
        assert_eq!(z.numerator, 0);
        assert_eq!(z.exp, 0);
        assert!((z.to_f64() - 0.0).abs() < 1e-12);
        let one = DyadicSurreal::from_int(1);
        assert_eq!(one.numerator, 1);
        assert_eq!(one.exp, 0);
        let half = DyadicSurreal::new(1, 1);
        assert!((half.to_f64() - 0.5).abs() < 1e-12);
        let three_quarters = DyadicSurreal::new(3, 2);
        assert!((three_quarters.to_f64() - 0.75).abs() < 1e-12);
    }
    #[test]
    fn test_dyadic_surreal_arithmetic() {
        let a = DyadicSurreal::from_int(3);
        let b = DyadicSurreal::from_int(5);
        assert_eq!(a.add(b).numerator, 8);
        let half = DyadicSurreal::new(1, 1);
        let quarter = DyadicSurreal::new(1, 2);
        let three_quarters = half.add(quarter);
        assert!((three_quarters.to_f64() - 0.75).abs() < 1e-12);
        let neg = DyadicSurreal::from_int(3).neg();
        assert_eq!(neg.numerator, -3);
        let prod = DyadicSurreal::from_int(3).mul(DyadicSurreal::from_int(4));
        assert_eq!(prod.numerator, 12);
    }
    #[test]
    fn test_dyadic_surreal_birthday() {
        assert_eq!(DyadicSurreal::zero().birthday(), 0);
        assert_eq!(DyadicSurreal::from_int(1).birthday(), 1);
        assert_eq!(DyadicSurreal::from_int(3).birthday(), 3);
        let half = DyadicSurreal::new(1, 1);
        assert_eq!(half.birthday(), 2);
    }
    #[test]
    fn test_nimber_arithmetic_add() {
        assert_eq!(NimberArithmetic::add(0, 5), 5);
        assert_eq!(NimberArithmetic::add(3, 5), 3 ^ 5);
        assert_eq!(NimberArithmetic::add(7, 7), 0);
    }
    #[test]
    fn test_nimber_arithmetic_mul_small() {
        assert_eq!(NimberArithmetic::mul(0, 5), 0);
        assert_eq!(NimberArithmetic::mul(1, 5), 5);
        assert_eq!(NimberArithmetic::mul(2, 2), 3);
        assert_eq!(NimberArithmetic::mul(2, 3), 1);
        assert_eq!(NimberArithmetic::mul(3, 3), 2);
    }
    #[test]
    fn test_wythoff_positions() {
        assert!(WythoffPositions::is_p_position(0, 0));
        assert!(WythoffPositions::is_p_position(1, 2));
        assert!(WythoffPositions::is_p_position(3, 5));
        assert!(WythoffPositions::is_p_position(4, 7));
        assert!(!WythoffPositions::is_p_position(1, 1));
        assert!(!WythoffPositions::is_p_position(2, 3));
        let (a0, b0) = WythoffPositions::nth(0);
        assert_eq!((a0, b0), (0, 0));
        let (a1, b1) = WythoffPositions::nth(1);
        assert_eq!((a1, b1), (1, 2));
        let (a2, b2) = WythoffPositions::nth(2);
        assert_eq!((a2, b2), (3, 5));
    }
    #[test]
    fn test_wythoff_winning_move() {
        let mv = WythoffPositions::winning_move(1, 1);
        assert!(mv.is_some());
        let (na, nb) = mv.expect("mv should be valid");
        assert!(WythoffPositions::is_p_position(na, nb));
        assert!(WythoffPositions::winning_move(1, 2).is_none());
    }
    #[test]
    fn test_thermograph_hot_game() {
        let thermo = ThermographData::from_hot_game(3, -1)
            .expect("ThermographData::from_hot_game should succeed");
        assert!(thermo.is_hot());
        assert!(!thermo.is_cold());
        assert!((thermo.temperature.to_f64() - 2.0).abs() < 1e-10);
        assert!((thermo.mean.to_f64() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_thermograph_not_hot() {
        assert!(ThermographData::from_hot_game(1, 3).is_none());
    }
    #[test]
    fn test_grundy_sequence_cache_nim() {
        let nim_code: Vec<u8> = vec![4; 8];
        let mut cache = GrundySequenceCache::new(nim_code);
        for n in 0..8usize {
            assert_eq!(cache.grundy(n), n as u64, "Grundy({n}) should be {n}");
        }
    }
    #[test]
    fn test_grundy_sequence_cache_single_digit() {
        let mut cache = GrundySequenceCache::new(vec![1u8]);
        assert_eq!(cache.grundy(0), 0);
        assert_eq!(cache.grundy(1), 1);
        assert_eq!(cache.grundy(2), 0);
    }
    #[test]
    fn test_grundy_sequence_cache_detect_period() {
        let nim_code: Vec<u8> = vec![4; 12];
        let mut cache = GrundySequenceCache::new(nim_code);
        cache.compute_up_to(10);
        assert_eq!(cache.values().len(), 11);
        for n in 0..=10 {
            assert_eq!(cache.values()[n], n as u64);
        }
    }
    #[test]
    fn test_build_advanced_cgt_axioms_in_env() {
        let mut env = Environment::new();
        build_combinatorial_game_env(&mut env);
        assert!(env.get(&Name::str("OrdinalGame")).is_some());
        assert!(env.get(&Name::str("NimberAdd")).is_some());
        assert!(env.get(&Name::str("NimberMul")).is_some());
        assert!(env.get(&Name::str("NimberInv")).is_some());
        assert!(env.get(&Name::str("SpragueMisere")).is_some());
        assert!(env.get(&Name::str("MisereQuotientMonoid")).is_some());
        assert!(env.get(&Name::str("GameTemperatureMap")).is_some());
        assert!(env.get(&Name::str("CoolingOperator")).is_some());
        assert!(env.get(&Name::str("GameIncentive")).is_some());
        assert!(env.get(&Name::str("ThermographLeft")).is_some());
        assert!(env.get(&Name::str("ThermographRight")).is_some());
        assert!(env.get(&Name::str("GameAtomicWeight")).is_some());
        assert!(env.get(&Name::str("atomic_weight_law")).is_some());
        assert!(env.get(&Name::str("DicotGame")).is_some());
        assert!(env.get(&Name::str("StarInversion")).is_some());
        assert!(env.get(&Name::str("GamePartialOrder")).is_some());
        assert!(env.get(&Name::str("GameParallelFuzzy")).is_some());
        assert!(env.get(&Name::str("LoopyGameDraw")).is_some());
        assert!(env.get(&Name::str("LoopyGameStopper")).is_some());
        assert!(env.get(&Name::str("SurrealOmega")).is_some());
        assert!(env.get(&Name::str("SurrealEpsilon")).is_some());
        assert!(env.get(&Name::str("SurrealDivide")).is_some());
        assert!(env.get(&Name::str("ShortGame")).is_some());
        assert!(env.get(&Name::str("GameBirthdayBound")).is_some());
        assert!(env.get(&Name::str("HotGame")).is_some());
        assert!(env.get(&Name::str("ColdGame")).is_some());
        assert!(env.get(&Name::str("DoubleUp")).is_some());
        assert!(env.get(&Name::str("GameAlphaBeta")).is_some());
        assert!(env.get(&Name::str("MinimaxValue")).is_some());
        assert!(env.get(&Name::str("GameTreeDepth")).is_some());
        assert!(env.get(&Name::str("CanonFormBirthday")).is_some());
        assert!(env.get(&Name::str("NortonProductLaw")).is_some());
        assert!(env.get(&Name::str("game_sum_assoc")).is_some());
        assert!(env.get(&Name::str("game_neg_inverse")).is_some());
        assert!(env.get(&Name::str("InfinitesimalGame")).is_some());
        assert!(env.get(&Name::str("UpnGame")).is_some());
        assert!(env.get(&Name::str("GameCongruence")).is_some());
        assert!(env.get(&Name::str("MultipleGame")).is_some());
        assert!(env.get(&Name::str("OrdinalSumGame")).is_some());
    }
    #[test]
    fn test_game_node_leaf_minimax() {
        let leaf = GameNode::leaf(42);
        assert_eq!(leaf.minimax(), 42);
        assert_eq!(leaf.alpha_beta(i32::MIN, i32::MAX), 42);
        assert_eq!(leaf.depth(), 0);
        assert_eq!(leaf.node_count(), 1);
    }
    #[test]
    fn test_game_node_simple_tree() {
        let tree = GameNode::internal(true, vec![GameNode::leaf(3), GameNode::leaf(5)]);
        assert_eq!(tree.minimax(), 5);
        assert_eq!(tree.alpha_beta(i32::MIN, i32::MAX), 5);
        assert_eq!(tree.depth(), 1);
        assert_eq!(tree.node_count(), 3);
    }
    #[test]
    fn test_game_node_alpha_beta_matches_minimax() {
        let tree = GameNode::internal(
            true,
            vec![
                GameNode::internal(false, vec![GameNode::leaf(3), GameNode::leaf(5)]),
                GameNode::internal(false, vec![GameNode::leaf(2), GameNode::leaf(9)]),
            ],
        );
        let mm = tree.minimax();
        let ab = tree.alpha_beta(i32::MIN, i32::MAX);
        assert_eq!(mm, ab);
        assert_eq!(mm, 3);
    }
    #[test]
    fn test_misere_analyzer_basic() {
        let a = MisereAnalyzer::new(vec![0]);
        assert!(a.is_p_position_misere());
        let b = MisereAnalyzer::new(vec![1]);
        assert!(b.is_n_position_misere());
        let c = MisereAnalyzer::new(vec![1, 1]);
        assert!(c.is_p_position_misere());
        let d = MisereAnalyzer::new(vec![2]);
        assert!(d.is_n_position_misere());
        let e = MisereAnalyzer::new(vec![3, 3]);
        assert!(e.is_p_position_misere());
    }
    #[test]
    fn test_misere_analyzer_winning_move() {
        let pos = MisereAnalyzer::new(vec![1, 1]);
        assert!(pos.winning_move_misere().is_none());
        let pos2 = MisereAnalyzer::new(vec![2, 2]);
        assert!(pos2.winning_move_misere().is_none());
    }
    #[test]
    fn test_loopy_game_graph_no_cycle() {
        let mut g = LoopyGameGraph::new(3);
        g.add_left_move(0, 1);
        g.add_left_move(1, 2);
        assert!(!g.has_cycle_from(0));
        assert!(g.is_terminal(2));
        assert!(!g.is_terminal(0));
        assert_eq!(g.terminal_count(), 1);
    }
    #[test]
    fn test_loopy_game_graph_with_cycle() {
        let mut g = LoopyGameGraph::new(3);
        g.add_left_move(0, 1);
        g.add_left_move(1, 2);
        g.add_left_move(2, 0);
        assert!(g.has_cycle_from(0));
        assert_eq!(g.terminal_count(), 0);
    }
    #[test]
    fn test_partizan_temperature_hot_game() {
        let pt = PartizanTemperature::new(vec![3], vec![-1]);
        assert!(pt.is_hot());
        assert!(!pt.is_cold());
        let temp = pt.temperature().expect("temperature should succeed");
        assert!((temp - 2.0).abs() < 1e-10);
        let mean = pt.mean().expect("mean should succeed");
        assert!((mean - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_partizan_temperature_cooled() {
        let pt = PartizanTemperature::new(vec![3], vec![-1]);
        let (cl, cr) = pt.cooled_at(1.0).expect("cooled_at should succeed");
        assert!((cl - 2.0).abs() < 1e-10);
        assert!((cr - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_partizan_temperature_cold_game() {
        let pt = PartizanTemperature::new(vec![1], vec![3]);
        assert!(pt.is_cold());
        let temp = pt.temperature().expect("temperature should succeed");
        assert!((temp - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_canonical_form_reducer_basic() {
        let r = CanonicalFormReducer::new(vec![3, 1], vec![-1, -3]);
        let (cl, cr) = r.canonical();
        assert_eq!(cl, vec![3]);
        assert_eq!(cr, vec![-3]);
        assert!(!r.is_canonical());
    }
    #[test]
    fn test_canonical_form_reducer_already_canonical() {
        let r = CanonicalFormReducer::new(vec![2], vec![-1]);
        assert!(r.is_canonical());
    }
    #[test]
    fn test_canonical_form_reducer_integer() {
        let r = CanonicalFormReducer::new(vec![2], vec![]);
        assert_eq!(r.integer_value(), Some(3));
        let r2 = CanonicalFormReducer::new(vec![], vec![-2]);
        assert_eq!(r2.integer_value(), Some(-3));
        let r3 = CanonicalFormReducer::new(vec![], vec![]);
        assert_eq!(r3.integer_value(), Some(0));
        let r4 = CanonicalFormReducer::new(vec![1], vec![-1]);
        assert_eq!(r4.integer_value(), None);
    }
}
/// Sprague-Grundy theorem application.
#[allow(dead_code)]
pub fn sprague_grundy_impartial_sum(grundy_values: &[NimValueExt]) -> NimValueExt {
    grundy_values
        .iter()
        .fold(NimValueExt(0), |acc, &g| acc.nim_sum(g))
}
#[cfg(test)]
mod tests_cgt_extra {
    use super::*;
    #[test]
    fn test_nim_value() {
        let v0 = NimValue::zero();
        assert!(v0.is_p_position());
        assert!(!v0.is_n_position());
        let v2 = NimValue(2);
        let v3 = NimValue(3);
        let ns = NimValue::nim_sum(v2, v3);
        assert_eq!(ns, NimValue(1));
    }
    #[test]
    fn test_mex() {
        let vals = vec![NimValue(0), NimValue(1), NimValue(3)];
        let m = NimValue::mex(&vals);
        assert_eq!(m, NimValue(2));
    }
    #[test]
    fn test_nim_game() {
        let g = NimGame::new(vec![3, 4, 5]);
        assert!(g.is_first_player_wins());
        assert!(g.winning_move().is_some());
        let g2 = NimGame::new(vec![1, 2, 3]);
        assert!(!g2.is_first_player_wins());
        assert!(g2.winning_move().is_none());
    }
    #[test]
    fn test_game_sum() {
        let mut gs = GameSum::new();
        gs.add_component("G1", 2.5);
        gs.add_component("G2", -1.0);
        assert!(gs.is_positive());
        assert_eq!(gs.outcome(), GameOutcome::LeftWins);
    }
    #[test]
    fn test_hackenbush() {
        let h = HackenbushGame::new(3, 2, 1);
        assert_eq!(h.game_value(), 1);
        assert!(h.is_left_advantage());
        assert!(!h.is_balanced());
    }
    #[test]
    fn test_sprague_grundy() {
        let vals = vec![NimValueExt(3), NimValueExt(5)];
        let total = sprague_grundy_impartial_sum(&vals);
        assert_eq!(total, NimValueExt(6));
    }
    #[test]
    fn test_temperature() {
        let hot = GameTemperature::new(3.0);
        assert!(hot.is_hot);
        let cold = GameTemperature::cold_game();
        assert!(cold.is_cold());
        let temps = vec![GameTemperature::new(2.0), GameTemperature::new(4.0)];
        assert!((GameTemperature::mean(&temps) - 3.0).abs() < 1e-9);
    }
}
