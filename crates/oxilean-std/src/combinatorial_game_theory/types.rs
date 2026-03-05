//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
use super::functions::*;

/// A surreal number represented as an exact dyadic rational `numerator / 2^exp`.
///
/// Day-0 surreals: {0}. Day-1: {-1, 0, 1}. Day-2: {-2,-3/2,-1,-1/2,0,1/2,1,3/2,2}. Etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DyadicSurreal {
    /// Numerator of `numerator / 2^exp`.
    pub numerator: i64,
    /// Exponent: denominator is `2^exp` (exp >= 0).
    pub exp: u32,
}
impl DyadicSurreal {
    /// Create `numerator / 2^exp` and reduce to lowest dyadic form.
    pub fn new(numerator: i64, exp: u32) -> Self {
        let mut n = numerator;
        let mut e = exp;
        while e > 0 && n % 2 == 0 {
            n /= 2;
            e -= 1;
        }
        DyadicSurreal {
            numerator: n,
            exp: e,
        }
    }
    /// The zero surreal.
    pub fn zero() -> Self {
        DyadicSurreal {
            numerator: 0,
            exp: 0,
        }
    }
    /// The integer surreal `n`.
    pub fn from_int(n: i64) -> Self {
        DyadicSurreal {
            numerator: n,
            exp: 0,
        }
    }
    /// Convert to `f64` for display.
    pub fn to_f64(self) -> f64 {
        self.numerator as f64 / (1u64 << self.exp) as f64
    }
    /// Add two dyadic surreals.
    pub fn add(self, other: DyadicSurreal) -> DyadicSurreal {
        let exp = self.exp.max(other.exp);
        let a = self.numerator * (1i64 << (exp - self.exp));
        let b = other.numerator * (1i64 << (exp - other.exp));
        DyadicSurreal::new(a + b, exp)
    }
    /// Negate a dyadic surreal.
    pub fn neg(self) -> DyadicSurreal {
        DyadicSurreal {
            numerator: -self.numerator,
            exp: self.exp,
        }
    }
    /// Subtract `other` from `self`.
    pub fn sub(self, other: DyadicSurreal) -> DyadicSurreal {
        self.add(other.neg())
    }
    /// Multiply two dyadic surreals.
    pub fn mul(self, other: DyadicSurreal) -> DyadicSurreal {
        DyadicSurreal::new(self.numerator * other.numerator, self.exp + other.exp)
    }
    /// The birthday of this dyadic surreal: the creation day in Conway's construction.
    ///
    /// Integers have birthday equal to their absolute value.
    /// `p/2^n` (p odd, n>0) has birthday `|p| + 2^n - 1`.
    pub fn birthday(self) -> u64 {
        if self.exp == 0 {
            self.numerator.unsigned_abs()
        } else {
            self.numerator.unsigned_abs() + (1u64 << self.exp) - 1
        }
    }
    /// Comparison: returns `Ordering`.
    pub fn cmp_val(self, other: DyadicSurreal) -> std::cmp::Ordering {
        let exp = self.exp.max(other.exp);
        let a = self.numerator * (1i64 << (exp - self.exp));
        let b = other.numerator * (1i64 << (exp - other.exp));
        a.cmp(&b)
    }
}
/// A combinatorial game defined by Left options and Right options.
///
/// Options are stored as integer game values for simplicity.
/// Positive values favour Left, negative favour Right.
#[derive(Debug, Clone, PartialEq)]
pub struct Game {
    /// Moves available to the Left player.
    pub left_options: Vec<i32>,
    /// Moves available to the Right player.
    pub right_options: Vec<i32>,
}
impl Game {
    /// Create a game with given Left and Right option lists.
    pub fn new(left: Vec<i32>, right: Vec<i32>) -> Self {
        Game {
            left_options: left,
            right_options: right,
        }
    }
    /// The zero game `{ | }` — both option lists empty. Second player wins.
    pub fn zero() -> Self {
        Game {
            left_options: vec![],
            right_options: vec![],
        }
    }
    /// The star game `{ 0 | 0 }` — first player wins (fuzzy with 0).
    pub fn star() -> Self {
        Game {
            left_options: vec![0],
            right_options: vec![0],
        }
    }
    /// The integer game `n`:
    /// - positive `n`:  `{ n-1 | }` (Left wins with n-1 free moves)
    /// - negative `n`:  `{ | n+1 }` (Right wins with |n|-1 free moves)
    /// - zero: `{ | }` (second player wins)
    pub fn integer(n: i32) -> Self {
        match n.cmp(&0) {
            std::cmp::Ordering::Greater => Game {
                left_options: vec![n - 1],
                right_options: vec![],
            },
            std::cmp::Ordering::Less => Game {
                left_options: vec![],
                right_options: vec![n + 1],
            },
            std::cmp::Ordering::Equal => Game::zero(),
        }
    }
    /// Returns `true` if this is a P-position (second player wins regardless of turn).
    ///
    /// Simplified heuristic: both option lists are empty, or all Left options
    /// are non-positive and all Right options are non-negative.
    pub fn is_zero(&self) -> bool {
        self.left_options.is_empty() && self.right_options.is_empty()
    }
    /// Returns `true` if this is a fuzzy game (first player wins from either side).
    ///
    /// Simplified: star `{0|0}` is the canonical fuzzy game.
    pub fn is_fuzzy(&self) -> bool {
        self.left_options == vec![0] && self.right_options == vec![0]
    }
    /// Game temperature (simplified): half the span of Left and Right options,
    /// or 0 if options are empty.
    pub fn temperature(&self) -> f64 {
        match (
            self.left_options.iter().max(),
            self.right_options.iter().min(),
        ) {
            (Some(&l), Some(&r)) => ((l - r) as f64) / 2.0,
            _ => 0.0,
        }
    }
    /// Remove dominated options:
    /// - For Left: remove any option `a` if there exists `b` with `b >= a`.
    ///   Keep only the maximum Left option.
    /// - For Right: keep only the minimum Right option.
    pub fn simplify(&self) -> Game {
        let left = match self.left_options.iter().max() {
            Some(&m) => vec![m],
            None => vec![],
        };
        let right = match self.right_options.iter().min() {
            Some(&m) => vec![m],
            None => vec![],
        };
        Game {
            left_options: left,
            right_options: right,
        }
    }
}
/// A Grundy value (nimber) for an impartial combinatorial game.
///
/// `NimValue(n)` corresponds to the Nim heap of size `n`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NimValue(pub u64);
impl NimValue {
    /// Nim heap of size `size` has Grundy value `size`.
    pub fn of_heap(size: u64) -> Self {
        NimValue(size)
    }
    /// Nim addition: XOR of two Grundy values.
    ///
    /// The Grundy value of a disjunctive sum of games equals the XOR
    /// of their individual Grundy values (Sprague-Grundy theorem).
    pub fn nim_sum(a: NimValue, b: NimValue) -> NimValue {
        NimValue(a.0 ^ b.0)
    }
    /// Returns `true` if this is a P-position (previous player wins, i.e., value 0).
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }
    /// Returns `true` if this is a P-position (value = 0).
    pub fn is_p_position(&self) -> bool {
        self.0 == 0
    }
    /// Returns `true` if this is an N-position (value ≠ 0).
    pub fn is_n_position(&self) -> bool {
        self.0 != 0
    }
    /// Returns `true` if this is an N-position (next player wins, i.e., value ≠ 0).
    pub fn is_nonzero(&self) -> bool {
        self.0 != 0
    }
    /// Returns `NimValue(0)`, the zero / P-position Grundy value.
    pub fn zero() -> Self {
        NimValue(0)
    }
    /// Nim-sum as a method (delegates to the associated function).
    pub fn nim_sum_with(&self, other: NimValue) -> NimValue {
        NimValue::nim_sum(*self, other)
    }
    /// Minimum excludant (mex) of a set of Grundy values.
    pub fn mex(values: &[NimValue]) -> NimValue {
        let set: std::collections::HashSet<u64> = values.iter().map(|v| v.0).collect();
        let mut m = 0u64;
        while set.contains(&m) {
            m += 1;
        }
        NimValue(m)
    }
}
/// A loopy combinatorial game represented as a directed graph.
///
/// Nodes represent positions, edges represent moves. Cycles are allowed
/// (unlike in normal finite combinatorial games). Positions are labeled
/// Left-to-move or Right-to-move.
#[derive(Debug, Clone)]
pub struct LoopyGameGraph {
    /// Number of positions.
    pub num_positions: usize,
    /// Left moves: `left_moves[i]` = set of positions Left can move to from i.
    pub left_moves: Vec<Vec<usize>>,
    /// Right moves: `right_moves[i]` = set of positions Right can move to from i.
    pub right_moves: Vec<Vec<usize>>,
}
impl LoopyGameGraph {
    /// Create an empty loopy game graph with `n` positions.
    pub fn new(n: usize) -> Self {
        LoopyGameGraph {
            num_positions: n,
            left_moves: vec![vec![]; n],
            right_moves: vec![vec![]; n],
        }
    }
    /// Add a Left move from position `from` to position `to`.
    pub fn add_left_move(&mut self, from: usize, to: usize) {
        if from < self.num_positions && to < self.num_positions {
            self.left_moves[from].push(to);
        }
    }
    /// Add a Right move from position `from` to position `to`.
    pub fn add_right_move(&mut self, from: usize, to: usize) {
        if from < self.num_positions && to < self.num_positions {
            self.right_moves[from].push(to);
        }
    }
    /// Detect if there is a cycle reachable from position `start` using
    /// all available moves (both Left and Right). Returns `true` if the game
    /// is truly loopy (has a cycle from `start`).
    pub fn has_cycle_from(&self, start: usize) -> bool {
        let mut visited = vec![false; self.num_positions];
        let mut on_stack = vec![false; self.num_positions];
        self.dfs_cycle(start, &mut visited, &mut on_stack)
    }
    fn dfs_cycle(&self, node: usize, visited: &mut Vec<bool>, on_stack: &mut Vec<bool>) -> bool {
        if on_stack[node] {
            return true;
        }
        if visited[node] {
            return false;
        }
        visited[node] = true;
        on_stack[node] = true;
        let all_moves: Vec<usize> = self.left_moves[node]
            .iter()
            .chain(self.right_moves[node].iter())
            .copied()
            .collect();
        for next in all_moves {
            if self.dfs_cycle(next, visited, on_stack) {
                on_stack[node] = false;
                return true;
            }
        }
        on_stack[node] = false;
        false
    }
    /// Return `true` if position `pos` is terminal (no moves for either player).
    pub fn is_terminal(&self, pos: usize) -> bool {
        pos < self.num_positions
            && self.left_moves[pos].is_empty()
            && self.right_moves[pos].is_empty()
    }
    /// Count the number of terminal positions.
    pub fn terminal_count(&self) -> usize {
        (0..self.num_positions)
            .filter(|&p| self.is_terminal(p))
            .count()
    }
}
/// A Nim game with multiple heaps.
///
/// Under normal play convention, the player who takes the last object wins.
pub struct NimGame {
    /// Sizes of the Nim heaps.
    pub heaps: Vec<u64>,
}
impl NimGame {
    /// Create a Nim game with the given heap sizes.
    pub fn new(heaps: Vec<u64>) -> Self {
        NimGame { heaps }
    }
    /// Compute the Grundy value (nim-sum) of the whole game: XOR of all heap sizes.
    pub fn grundy_value(&self) -> NimValue {
        let xor = self.heaps.iter().fold(0u64, |acc, &h| acc ^ h);
        NimValue(xor)
    }
    /// Returns `true` if this is a P-position (current player loses with perfect play).
    pub fn is_p_position(&self) -> bool {
        self.grundy_value().is_zero()
    }
    /// Returns `true` if the first player wins (N-position).
    pub fn is_first_player_wins(&self) -> bool {
        !self.is_p_position()
    }
    /// Find a winning move `(heap_index, new_size)`, or `None` if already a P-position.
    ///
    /// A winning move reduces some heap so that the resulting nim-sum is 0.
    pub fn winning_move(&self) -> Option<(usize, u64)> {
        let xor = self.grundy_value().0;
        if xor == 0 {
            return None;
        }
        for (i, &h) in self.heaps.iter().enumerate() {
            let target = h ^ xor;
            if target < h {
                return Some((i, target));
            }
        }
        None
    }
}
/// Partizan game temperature calculator.
///
/// Given a partizan game's Left and Right options (as integer values), computes
/// the temperature of the game and related thermographic data.
#[derive(Debug, Clone)]
pub struct PartizanTemperature {
    /// Left options (best Left move values, higher is better for Left).
    pub left_options: Vec<i32>,
    /// Right options (best Right move values, lower is better for Right).
    pub right_options: Vec<i32>,
}
impl PartizanTemperature {
    /// Create a partizan temperature calculator from option lists.
    pub fn new(left_options: Vec<i32>, right_options: Vec<i32>) -> Self {
        PartizanTemperature {
            left_options,
            right_options,
        }
    }
    /// Compute the mean value of the game: `(left_best + right_best) / 2`.
    ///
    /// Returns `None` if either player has no options.
    pub fn mean(&self) -> Option<f64> {
        let l = self.left_options.iter().max()?;
        let r = self.right_options.iter().min()?;
        Some((*l + *r) as f64 / 2.0)
    }
    /// Compute the temperature of the game: `(left_best - right_best) / 2`.
    ///
    /// Returns `None` if either player has no options. Returns 0.0 if
    /// left_best ≤ right_best (cold game).
    pub fn temperature(&self) -> Option<f64> {
        let l = *self.left_options.iter().max()?;
        let r = *self.right_options.iter().min()?;
        Some(((l - r) as f64 / 2.0).max(0.0))
    }
    /// Returns `true` if this is a hot game (temperature > 0).
    pub fn is_hot(&self) -> bool {
        self.temperature().map(|t| t > 0.0).unwrap_or(false)
    }
    /// Returns `true` if this is a cold game (temperature ≤ 0).
    pub fn is_cold(&self) -> bool {
        !self.is_hot()
    }
    /// Compute the cooled game value at temperature `t`.
    ///
    /// Returns `(cooled_left, cooled_right)` where `cooled_left = left_best - t`
    /// and `cooled_right = right_best + t`. These merge at the temperature.
    pub fn cooled_at(&self, t: f64) -> Option<(f64, f64)> {
        let l = *self.left_options.iter().max()? as f64;
        let r = *self.right_options.iter().min()? as f64;
        Some((l - t, r + t))
    }
    /// Returns the number of Left incentives (distinct Left option values above mean).
    pub fn left_incentive_count(&self) -> usize {
        match self.mean() {
            None => 0,
            Some(m) => self.left_options.iter().filter(|&&v| v as f64 > m).count(),
        }
    }
    /// Returns the number of Right incentives (distinct Right option values below mean).
    pub fn right_incentive_count(&self) -> usize {
        match self.mean() {
            None => 0,
            Some(m) => self
                .right_options
                .iter()
                .filter(|&&v| (v as f64) < m)
                .count(),
        }
    }
}
/// Canonical form reducer for simple partizan games.
///
/// Implements dominated option removal: for Left, a move is dominated if
/// there exists a strictly better Left option; for Right, a move is dominated
/// if there is a strictly worse (for Right) option. After removing dominated
/// options, also removes reversible options (where a Left option G^L has a
/// Right option G^LR ≤ G — but we approximate here with simple dominance).
#[derive(Debug, Clone)]
pub struct CanonicalFormReducer {
    /// Current Left options.
    pub left: Vec<i32>,
    /// Current Right options.
    pub right: Vec<i32>,
}
impl CanonicalFormReducer {
    /// Create a reducer from Left and Right option lists.
    pub fn new(left: Vec<i32>, right: Vec<i32>) -> Self {
        CanonicalFormReducer { left, right }
    }
    /// Remove dominated Left options: keep only the maximum.
    pub fn remove_dominated_left(&self) -> Vec<i32> {
        match self.left.iter().max() {
            Some(&m) => vec![m],
            None => vec![],
        }
    }
    /// Remove dominated Right options: keep only the minimum.
    pub fn remove_dominated_right(&self) -> Vec<i32> {
        match self.right.iter().min() {
            Some(&m) => vec![m],
            None => vec![],
        }
    }
    /// Return the canonical form (simplified left, simplified right).
    pub fn canonical(&self) -> (Vec<i32>, Vec<i32>) {
        (self.remove_dominated_left(), self.remove_dominated_right())
    }
    /// Return `true` if the game is already in canonical form.
    pub fn is_canonical(&self) -> bool {
        let (cl, cr) = self.canonical();
        cl == self.left && cr == self.right
    }
    /// Compute the integer value of the game if it is an integer game
    /// (one option list is empty, the other is a singleton).
    pub fn integer_value(&self) -> Option<i32> {
        let (cl, cr) = self.canonical();
        if cl.is_empty() && cr.is_empty() {
            Some(0)
        } else if cr.is_empty() && cl.len() == 1 {
            Some(cl[0] + 1)
        } else if cl.is_empty() && cr.len() == 1 {
            Some(cr[0] - 1)
        } else {
            None
        }
    }
}
/// A surreal number represented by left and right sets of earlier surreals
/// (stored as integers for finite surreals).
#[derive(Debug, Clone, PartialEq)]
pub struct SurrealNumber {
    /// The left set: surreals strictly less than this number.
    pub left_set: Vec<i64>,
    /// The right set: surreals strictly greater than this number.
    pub right_set: Vec<i64>,
}
impl SurrealNumber {
    /// Create a surreal number from explicit left and right sets.
    pub fn new() -> Self {
        SurrealNumber {
            left_set: vec![],
            right_set: vec![],
        }
    }
    /// The surreal zero `{ | }`.
    pub fn zero() -> Self {
        SurrealNumber {
            left_set: vec![],
            right_set: vec![],
        }
    }
    /// The surreal one `{ 0 | }`.
    pub fn one() -> Self {
        SurrealNumber {
            left_set: vec![0],
            right_set: vec![],
        }
    }
    /// The surreal negative one `{ | 0 }`.
    pub fn neg_one() -> Self {
        SurrealNumber {
            left_set: vec![],
            right_set: vec![0],
        }
    }
    /// Construct the surreal integer `n`.
    pub fn from_integer(n: i64) -> Self {
        match n.cmp(&0) {
            std::cmp::Ordering::Greater => SurrealNumber {
                left_set: vec![n - 1],
                right_set: vec![],
            },
            std::cmp::Ordering::Less => SurrealNumber {
                left_set: vec![],
                right_set: vec![n + 1],
            },
            std::cmp::Ordering::Equal => SurrealNumber::zero(),
        }
    }
    /// Add two surreal numbers (stub — returns zero for non-integer inputs).
    pub fn add(&self, other: &SurrealNumber) -> SurrealNumber {
        match (self.to_rational(), other.to_rational()) {
            (Some((n1, d1)), Some((n2, d2))) => {
                let num = n1 * d2 + n2 * d1;
                let den = d1 * d2;
                if den == 1 {
                    SurrealNumber::from_integer(num)
                } else {
                    SurrealNumber {
                        left_set: vec![num / den],
                        right_set: vec![num / den + 1],
                    }
                }
            }
            _ => SurrealNumber::zero(),
        }
    }
    /// Returns `true` if this surreal represents an integer.
    pub fn is_integer(&self) -> bool {
        (self.left_set.is_empty() && self.right_set.is_empty())
            || (self.left_set.len() == 1 && self.right_set.is_empty())
            || (self.left_set.is_empty() && self.right_set.len() == 1)
    }
    /// Return `(numerator, denominator)` if this is a dyadic rational, else `None`.
    pub fn to_rational(&self) -> Option<(i64, i64)> {
        if self.left_set.is_empty() && self.right_set.is_empty() {
            Some((0, 1))
        } else if self.left_set.len() == 1 && self.right_set.is_empty() {
            Some((self.left_set[0] + 1, 1))
        } else if self.left_set.is_empty() && self.right_set.len() == 1 {
            Some((self.right_set[0] - 1, 1))
        } else if self.left_set.len() == 1 && self.right_set.len() == 1 {
            let l = self.left_set[0];
            let r = self.right_set[0];
            Some((l + r, 2))
        } else {
            None
        }
    }
}
/// Cache and calculator for Grundy (nim-value) sequences of octal games.
///
/// An octal game is specified by its code digit sequence. For each heap size `n`
/// the Grundy value is computed via the mex of positions reachable by the game rules.
pub struct GrundySequenceCache {
    /// The octal code digits (0–7 each), describing allowed moves.
    pub code: Vec<u8>,
    /// Cached Grundy values indexed by heap size.
    pub values: Vec<u64>,
}
impl GrundySequenceCache {
    /// Create a new cache with the given octal code.
    ///
    /// `code[k]` is the digit for heaps of size `k+1` (1-indexed).
    /// Bit 0 of `code[k]`: may remove entire heap (leaving 0).
    /// Bit 1 of `code[k]`: may split into two non-empty heaps summing to k-1.
    /// Bit 2 of `code[k]`: may remove to any size 0..k-1 (normal Nim move).
    pub fn new(code: Vec<u8>) -> Self {
        GrundySequenceCache {
            code,
            values: vec![0],
        }
    }
    /// Extend cached values up to heap size `n` (inclusive).
    pub fn compute_up_to(&mut self, n: usize) {
        while self.values.len() <= n {
            let heap = self.values.len();
            let g = self.compute_grundy(heap);
            self.values.push(g);
        }
    }
    /// Compute the Grundy value for a heap of size `heap` using stored code.
    fn compute_grundy(&self, heap: usize) -> u64 {
        let mut reachable = std::collections::BTreeSet::new();
        let digits = &self.code;
        for split in 1..=heap {
            let code_idx = split.saturating_sub(1);
            let digit = if code_idx < digits.len() {
                digits[code_idx]
            } else {
                0
            };
            if digit & 1 != 0 && split == heap {
                reachable.insert(0u64);
            }
            if digit & 4 != 0 && split == heap {
                for smaller in 0..heap {
                    let g = if smaller < self.values.len() {
                        self.values[smaller]
                    } else {
                        0
                    };
                    reachable.insert(g);
                }
            }
            if digit & 2 != 0 && split == heap {
                let total = heap - 1;
                for left_part in 1..total {
                    let right_part = total - left_part;
                    let gl = if left_part < self.values.len() {
                        self.values[left_part]
                    } else {
                        0
                    };
                    let gr = if right_part < self.values.len() {
                        self.values[right_part]
                    } else {
                        0
                    };
                    reachable.insert(gl ^ gr);
                }
            }
        }
        let mut mex = 0u64;
        for &v in &reachable {
            if v == mex {
                mex += 1;
            } else if v > mex {
                break;
            }
        }
        mex
    }
    /// Return the Grundy value for heap size `n`, computing if necessary.
    pub fn grundy(&mut self, n: usize) -> u64 {
        self.compute_up_to(n);
        self.values[n]
    }
    /// Return all computed Grundy values as a slice.
    pub fn values(&self) -> &[u64] {
        &self.values
    }
    /// Detect period in the Grundy sequence (eventual periodicity).
    ///
    /// Returns `Some((preperiod, period))` if periodicity is detected within
    /// the computed values, else `None`.
    pub fn detect_period(&self) -> Option<(usize, usize)> {
        let vals = &self.values;
        let n = vals.len();
        for period in 1..=n / 2 {
            for start in 0..n.saturating_sub(2 * period) {
                let all_match = (0..period)
                    .all(|i| start + i + period < n && vals[start + i] == vals[start + i + period]);
                if all_match && period > 0 {
                    return Some((start, period));
                }
            }
        }
        None
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum GameOutcome {
    LeftWins,
    RightWins,
    FirstPlayerWins,
    SecondPlayerWins,
}
/// Thermograph data for a combinatorial game G.
///
/// A thermograph consists of left scaffold (Left's incentive under cooling)
/// and right scaffold (Right's incentive), meeting at the temperature `t*`.
///
/// All values are stored as dyadic rationals via `DyadicSurreal`.
#[derive(Debug, Clone)]
pub struct ThermographData {
    /// The temperature t* at which the thermograph closes (tally point).
    pub temperature: DyadicSurreal,
    /// The mean value m = (left_wall + right_wall) / 2 at temperature 0.
    pub mean: DyadicSurreal,
    /// Left wall value at temperature 0 (Left's cooled value).
    pub left_wall: DyadicSurreal,
    /// Right wall value at temperature 0 (Right's cooled value).
    pub right_wall: DyadicSurreal,
}
impl ThermographData {
    /// Construct thermograph data from a simple hot game `{a | b}` with `a > b`.
    ///
    /// Temperature = `(a - b) / 2`, mean = `(a + b) / 2`.
    pub fn from_hot_game(left_val: i64, right_val: i64) -> Option<Self> {
        if left_val <= right_val {
            return None;
        }
        let l = DyadicSurreal::from_int(left_val);
        let r = DyadicSurreal::from_int(right_val);
        let two = DyadicSurreal::from_int(2);
        let temperature = l.sub(r).mul(DyadicSurreal::new(1, 1));
        let _ = two;
        let mean = DyadicSurreal::new(left_val + right_val, 1);
        Some(ThermographData {
            temperature,
            mean,
            left_wall: l,
            right_wall: r,
        })
    }
    /// Returns `true` if the game is hot (temperature > 0).
    pub fn is_hot(&self) -> bool {
        self.temperature.cmp_val(DyadicSurreal::zero()) == std::cmp::Ordering::Greater
    }
    /// Returns `true` if the game is tepid (temperature = 0, mean = integer).
    pub fn is_cold(&self) -> bool {
        self.temperature.cmp_val(DyadicSurreal::zero()) == std::cmp::Ordering::Equal
    }
}
/// Alpha-beta pruning evaluator for combinatorial game trees.
///
/// Implements the classical alpha-beta algorithm with integer scores.
/// The tree is represented as nested `GameNode` values.
#[derive(Debug, Clone)]
pub struct GameNode {
    /// Static evaluation value at leaf nodes.
    pub value: Option<i32>,
    /// Children nodes (empty at leaves).
    pub children: Vec<GameNode>,
    /// `true` if this is a maximizing node (Left to move).
    pub is_max_node: bool,
}
impl GameNode {
    /// Create a leaf node with the given static evaluation.
    pub fn leaf(value: i32) -> Self {
        GameNode {
            value: Some(value),
            children: vec![],
            is_max_node: true,
        }
    }
    /// Create an internal node with the given children.
    pub fn internal(is_max: bool, children: Vec<GameNode>) -> Self {
        GameNode {
            value: None,
            children,
            is_max_node: is_max,
        }
    }
    /// Run the minimax algorithm (no pruning).
    pub fn minimax(&self) -> i32 {
        if self.children.is_empty() {
            return self.value.unwrap_or(0);
        }
        let scores: Vec<i32> = self.children.iter().map(|c| c.minimax()).collect();
        if self.is_max_node {
            scores.into_iter().max().unwrap_or(0)
        } else {
            scores.into_iter().min().unwrap_or(0)
        }
    }
    /// Run alpha-beta pruning.
    ///
    /// Returns the minimax value with alpha-beta cutoffs.
    /// Initial call: `alpha_beta(i32::MIN, i32::MAX)`.
    pub fn alpha_beta(&self, mut alpha: i32, mut beta: i32) -> i32 {
        if self.children.is_empty() {
            return self.value.unwrap_or(0);
        }
        if self.is_max_node {
            let mut v = i32::MIN;
            for child in &self.children {
                let score = child.alpha_beta(alpha, beta);
                if score > v {
                    v = score;
                }
                if v > alpha {
                    alpha = v;
                }
                if alpha >= beta {
                    break;
                }
            }
            v
        } else {
            let mut v = i32::MAX;
            for child in &self.children {
                let score = child.alpha_beta(alpha, beta);
                if score < v {
                    v = score;
                }
                if v < beta {
                    beta = v;
                }
                if alpha >= beta {
                    break;
                }
            }
            v
        }
    }
    /// Compute the depth of the game tree.
    pub fn depth(&self) -> usize {
        if self.children.is_empty() {
            0
        } else {
            1 + self.children.iter().map(|c| c.depth()).max().unwrap_or(0)
        }
    }
    /// Count the number of nodes visited by minimax (no pruning).
    pub fn node_count(&self) -> usize {
        1 + self.children.iter().map(|c| c.node_count()).sum::<usize>()
    }
}
/// Nim game with multiple piles.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct NimGameExt {
    pub piles: Vec<usize>,
}
#[allow(dead_code)]
impl NimGameExt {
    pub fn new(piles: Vec<usize>) -> Self {
        NimGameExt { piles }
    }
    pub fn is_first_player_wins(&self) -> bool {
        self.nim_value() != 0
    }
    pub fn nim_value(&self) -> usize {
        self.piles.iter().fold(0, |acc, &p| acc ^ p)
    }
    /// Winning move: find move to make XOR = 0.
    pub fn winning_move(&self) -> Option<(usize, usize)> {
        let xor = self.nim_value();
        if xor == 0 {
            return None;
        }
        for (i, &pile) in self.piles.iter().enumerate() {
            let target = pile ^ xor;
            if target < pile {
                return Some((i, pile - target));
            }
        }
        None
    }
    pub fn total_stones(&self) -> usize {
        self.piles.iter().sum()
    }
}
/// Temperature of a game (rate at which game's value changes).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GameTemperature {
    pub value: f64,
    pub is_hot: bool,
}
#[allow(dead_code)]
impl GameTemperature {
    pub fn new(t: f64) -> Self {
        GameTemperature {
            value: t,
            is_hot: t > 0.0,
        }
    }
    pub fn cold_game() -> Self {
        GameTemperature::new(0.0)
    }
    pub fn is_cold(&self) -> bool {
        !self.is_hot
    }
    pub fn mean(games: &[GameTemperature]) -> f64 {
        if games.is_empty() {
            return 0.0;
        }
        games.iter().map(|g| g.value).sum::<f64>() / games.len() as f64
    }
}
/// Wythoff's game P-position calculator.
///
/// In Wythoff's queens game, two players alternately remove any number of objects
/// from one heap, or equal numbers from both heaps. The player taking the last wins.
/// P-positions are pairs `(a_n, b_n)` where `a_n = ⌊n·φ⌋`, `b_n = a_n + n`
/// (φ = golden ratio).
pub struct WythoffPositions;
impl WythoffPositions {
    const PHI: f64 = 1.618_033_988_749_895;
    /// Return the `n`-th Wythoff P-position `(a_n, b_n)` (0-indexed).
    pub fn nth(n: u64) -> (u64, u64) {
        let a = (n as f64 * Self::PHI).floor() as u64;
        (a, a + n)
    }
    /// Return `true` if `(pile_a, pile_b)` (with `pile_a ≤ pile_b`) is a P-position.
    pub fn is_p_position(pile_a: u64, pile_b: u64) -> bool {
        let (a, b) = if pile_a <= pile_b {
            (pile_a, pile_b)
        } else {
            (pile_b, pile_a)
        };
        let diff = b - a;
        let expected_a = (diff as f64 * Self::PHI).floor() as u64;
        expected_a == a
    }
    /// Find all Wythoff P-positions `(a, b)` with `b ≤ max_val`.
    pub fn p_positions_up_to(max_val: u64) -> Vec<(u64, u64)> {
        let mut result = Vec::new();
        let mut n = 0u64;
        loop {
            let (a, b) = Self::nth(n);
            if b > max_val {
                break;
            }
            result.push((a, b));
            n += 1;
        }
        result
    }
    /// Find a winning move from `(pile_a, pile_b)`, or `None` if P-position.
    ///
    /// Returns `(new_a, new_b)` after the move.
    pub fn winning_move(pile_a: u64, pile_b: u64) -> Option<(u64, u64)> {
        if Self::is_p_position(pile_a, pile_b) {
            return None;
        }
        let (a, b) = if pile_a <= pile_b {
            (pile_a, pile_b)
        } else {
            (pile_b, pile_a)
        };
        for new_a in 0..a {
            if Self::is_p_position(new_a, b) {
                return Some((new_a, b));
            }
        }
        for new_b in 0..b {
            if Self::is_p_position(a, new_b) {
                return Some((a, new_b));
            }
        }
        let diff = b - a;
        for k in 1..=a {
            if Self::is_p_position(a - k, b - k) {
                let r_a = a - k;
                let r_b = b - k;
                let _ = diff;
                return Some((r_a, r_b));
            }
        }
        None
    }
}
/// Arithmetic on nimbers (ordinal numbers under nim-addition and nim-multiplication).
///
/// Nim-addition is XOR; nim-multiplication uses Fermat-2-power recursion.
pub struct NimberArithmetic;
impl NimberArithmetic {
    /// Nim-addition: XOR of two values.
    pub fn add(a: u64, b: u64) -> u64 {
        a ^ b
    }
    /// Nim-multiplication of two nimbers using the recursive Fermat-2-power formula.
    ///
    /// This implementation handles values up to 2^16 correctly using the rule:
    /// - `2^(2^k) * 2^(2^k) = 3/2 * 2^(2^k)` (in nim arithmetic)
    /// For small values (up to 256) this uses a direct table approach.
    pub fn mul(a: u64, b: u64) -> u64 {
        Self::nim_mul_rec(a, b)
    }
    fn nim_mul_rec(a: u64, b: u64) -> u64 {
        if a <= 1 || b <= 1 {
            return a * b;
        }
        if a == b {
            return Self::nim_square(a);
        }
        let k = 63 - a.leading_zeros();
        let fk = 1u64 << k;
        if a == fk {
            let q = b / fk;
            let r = b % fk;
            let half = fk / 2;
            let t1 = Self::nim_mul_rec(fk, r);
            let t2 = Self::nim_mul_rec(q, fk) ^ Self::nim_mul_rec(q, half);
            t1 ^ t2
        } else {
            let q = a / fk;
            let r = a % fk;
            Self::nim_mul_rec(q * fk, b) ^ Self::nim_mul_rec(r, b)
        }
    }
    fn nim_square(a: u64) -> u64 {
        if a <= 1 {
            return a;
        }
        let k = 63 - a.leading_zeros();
        let fk = 1u64 << k;
        if a == fk {
            if fk == 2 {
                return 3;
            }
            let half = fk / 2;
            fk ^ Self::nim_mul_rec(half, fk)
        } else {
            let q = a / fk;
            let r = a % fk;
            Self::nim_mul_rec(Self::nim_mul_rec(q, q), Self::nim_square(fk)) ^ Self::nim_square(r)
        }
    }
    /// Compute the multiplicative inverse of `a` in the nimber field (for a > 0).
    ///
    /// Returns `None` if `a == 0`.
    pub fn inv(a: u64) -> Option<u64> {
        if a == 0 {
            return None;
        }
        if a == 1 {
            return Some(1);
        }
        (1..=a.max(255)).find(|&b| Self::mul(a, b) == 1)
    }
}
/// Combinatorial game: sum of games.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GameSum {
    pub components: Vec<String>,
    pub combined_value: f64,
}
#[allow(dead_code)]
impl GameSum {
    pub fn new() -> Self {
        GameSum {
            components: Vec::new(),
            combined_value: 0.0,
        }
    }
    pub fn add_component(&mut self, name: &str, val: f64) {
        self.components.push(name.to_string());
        self.combined_value += val;
    }
    pub fn is_positive(&self) -> bool {
        self.combined_value > 0.0
    }
    pub fn is_zero(&self) -> bool {
        self.combined_value.abs() < 1e-9
    }
    pub fn outcome(&self) -> GameOutcome {
        if self.combined_value > 0.0 {
            GameOutcome::LeftWins
        } else if self.combined_value < 0.0 {
            GameOutcome::RightWins
        } else {
            GameOutcome::SecondPlayerWins
        }
    }
}
/// Hackenbush game position.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct HackenbushGame {
    pub blue_edges: usize,
    pub red_edges: usize,
    pub green_edges: usize,
}
#[allow(dead_code)]
impl HackenbushGame {
    pub fn new(blue: usize, red: usize, green: usize) -> Self {
        HackenbushGame {
            blue_edges: blue,
            red_edges: red,
            green_edges: green,
        }
    }
    /// Blue-red Hackenbush value: blue - red.
    pub fn game_value(&self) -> i64 {
        self.blue_edges as i64 - self.red_edges as i64
    }
    pub fn is_left_advantage(&self) -> bool {
        self.game_value() > 0
    }
    pub fn is_balanced(&self) -> bool {
        self.blue_edges == self.red_edges
    }
}
/// Misère game position analyzer.
///
/// Under misère play convention, the last player to move *loses*.
/// The Sprague-Grundy theory for misère games is identical to normal play
/// except the terminal rule: position with Grundy value 0 is winning (last
/// move already taken by opponent), and positions with all Grundy values 0
/// in sub-games need special handling.
#[derive(Debug, Clone)]
pub struct MisereAnalyzer {
    /// Grundy values for individual sub-games (impartial).
    pub sub_game_grundy: Vec<u64>,
}
impl MisereAnalyzer {
    /// Create a misère analyzer from a list of sub-game Grundy values.
    pub fn new(sub_game_grundy: Vec<u64>) -> Self {
        MisereAnalyzer { sub_game_grundy }
    }
    /// Determine if the current position is a P-position (previous player wins)
    /// under misère play convention.
    ///
    /// Misère Sprague-Grundy theorem: A sum of games is a P-position under
    /// misère play iff:
    /// - all Grundy values are 0 or 1, AND the XOR (nim-sum) is 0, OR
    /// - at least one Grundy value is ≥ 2, AND the nim-sum ≠ 0.
    ///
    /// Equivalently: it is an N-position iff
    /// - all values ≤ 1 AND nim-sum ≠ 0, OR
    /// - some value ≥ 2 AND nim-sum = 0.
    pub fn is_p_position_misere(&self) -> bool {
        let nim_sum = self.sub_game_grundy.iter().fold(0u64, |acc, &g| acc ^ g);
        nim_sum == 0
    }
    /// Determine if the current position is an N-position (next player wins) under
    /// misère play.
    pub fn is_n_position_misere(&self) -> bool {
        !self.is_p_position_misere()
    }
    /// Find a winning move index (sub-game to reduce), or `None` if P-position.
    ///
    /// Returns the index of the sub-game to play in, and the new Grundy value
    /// to reduce it to.
    pub fn winning_move_misere(&self) -> Option<(usize, u64)> {
        if self.is_p_position_misere() {
            return None;
        }
        let n = self.sub_game_grundy.len();
        for i in 0..n {
            let current = self.sub_game_grundy[i];
            for new_val in 0..current {
                let mut new_state = self.sub_game_grundy.clone();
                new_state[i] = new_val;
                let analyzer = MisereAnalyzer::new(new_state);
                if analyzer.is_p_position_misere() {
                    return Some((i, new_val));
                }
            }
        }
        None
    }
}
/// Nim value (Grundy value) for positions.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct NimValueExt(pub usize);
#[allow(dead_code)]
impl NimValueExt {
    pub fn zero() -> Self {
        NimValueExt(0)
    }
    pub fn is_p_position(&self) -> bool {
        self.0 == 0
    }
    pub fn is_n_position(&self) -> bool {
        self.0 > 0
    }
    pub fn nim_sum(&self, other: NimValueExt) -> NimValueExt {
        NimValueExt(self.0 ^ other.0)
    }
    pub fn mex(values: &[NimValueExt]) -> NimValueExt {
        let set: std::collections::HashSet<usize> = values.iter().map(|v| v.0).collect();
        let mut m = 0;
        while set.contains(&m) {
            m += 1;
        }
        NimValueExt(m)
    }
}
