//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
use super::functions::*;

/// Sequence alignment scoring.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AlignmentScorer {
    pub match_score: i32,
    pub mismatch_penalty: i32,
    pub gap_open: i32,
    pub gap_extend: i32,
}
#[allow(dead_code)]
impl AlignmentScorer {
    /// Default scoring scheme.
    pub fn default_scorer() -> Self {
        Self {
            match_score: 2,
            mismatch_penalty: -1,
            gap_open: -4,
            gap_extend: -1,
        }
    }
    /// BLOSUM62-like scoring.
    pub fn blosum62_approx() -> Self {
        Self {
            match_score: 4,
            mismatch_penalty: -1,
            gap_open: -11,
            gap_extend: -1,
        }
    }
    /// Score two characters.
    pub fn score_pair(&self, a: char, b: char) -> i32 {
        if a == b {
            self.match_score
        } else {
            self.mismatch_penalty
        }
    }
    /// Smith-Waterman local alignment description.
    pub fn smith_waterman_description(&self) -> String {
        format!(
            "Smith-Waterman: match={}, mismatch={}, gap_open={}, gap_ext={}",
            self.match_score, self.mismatch_penalty, self.gap_open, self.gap_extend
        )
    }
}
/// A 2D lattice conformation as a sequence of moves.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LatticeMove {
    Up,
    Down,
    Left,
    Right,
}
/// Protein structure data.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ProteinStructure {
    pub pdb_id: String,
    pub sequence: String,
    pub secondary: Vec<SecondaryStructure>,
    pub resolution_angstrom: f64,
}
#[allow(dead_code)]
impl ProteinStructure {
    /// Create protein structure record.
    pub fn new(pdb_id: &str, seq: &str, ss: Vec<SecondaryStructure>, res: f64) -> Self {
        Self {
            pdb_id: pdb_id.to_string(),
            sequence: seq.to_string(),
            secondary: ss,
            resolution_angstrom: res,
        }
    }
    /// Fraction of helix.
    pub fn helix_fraction(&self) -> f64 {
        if self.secondary.is_empty() {
            return 0.0;
        }
        let n_helix = self
            .secondary
            .iter()
            .filter(|s| **s == SecondaryStructure::AlphaHelix)
            .count();
        n_helix as f64 / self.secondary.len() as f64
    }
    /// High resolution?
    pub fn is_high_resolution(&self) -> bool {
        self.resolution_angstrom < 2.0
    }
}
/// A base pair in an RNA secondary structure.
#[derive(Debug, Clone, PartialEq)]
pub struct BasePair {
    pub i: usize,
    pub j: usize,
}
/// A branch in a phylogenetic tree.
#[derive(Debug, Clone)]
pub struct PhyloBranch {
    pub from: String,
    pub to: String,
    pub length: f64,
}
/// A BLAST hit: position in query and subject, and the matching word.
#[derive(Debug, Clone, PartialEq)]
pub struct BlastHit {
    pub query_pos: usize,
    pub subject_pos: usize,
    pub word: String,
    pub score: i32,
}
/// An RNA secondary structure folder using the Nussinov maximum base-pair algorithm
/// as a proxy for minimum free energy (MFE) folding.
#[derive(Debug, Clone)]
pub struct RNAMFEFolder {
    /// Penalty per unpaired base (negative = energetically unfavourable).
    pub unpaired_penalty: f64,
    /// Energy bonus per base pair.
    pub pair_bonus: f64,
}
impl RNAMFEFolder {
    /// Create an `RNAMFEFolder` with default Turner-like parameters.
    pub fn new() -> Self {
        RNAMFEFolder {
            unpaired_penalty: 0.0,
            pair_bonus: -1.0,
        }
    }
    /// Create an `RNAMFEFolder` with custom parameters.
    pub fn with_params(pair_bonus: f64, unpaired_penalty: f64) -> Self {
        RNAMFEFolder {
            unpaired_penalty,
            pair_bonus,
        }
    }
    /// Fold an RNA sequence; returns (approximate MFE, base-pair list).
    pub fn fold(&self, sequence: &str) -> (f64, Vec<BasePair>) {
        let (n_pairs, pairs) = nussinov(sequence);
        let n_unpaired = sequence.len().saturating_sub(2 * pairs.len());
        let mfe = n_pairs as f64 * self.pair_bonus + n_unpaired as f64 * self.unpaired_penalty;
        (mfe, pairs)
    }
    /// Return only the MFE estimate.
    pub fn mfe(&self, sequence: &str) -> f64 {
        self.fold(sequence).0
    }
}
/// Alignment result containing score and aligned sequences.
#[derive(Debug, Clone, PartialEq)]
pub struct Alignment {
    pub score: i32,
    pub aligned_a: String,
    pub aligned_b: String,
}
/// A De Bruijn graph node is a k-1-mer.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeBruijnNode(pub String);
/// A distance matrix for phylogenetic reconstruction.
#[derive(Debug, Clone)]
pub struct DistMatrix {
    pub labels: Vec<String>,
    pub data: Vec<Vec<f64>>,
}
impl DistMatrix {
    /// Create a new distance matrix.
    pub fn new(labels: Vec<String>, data: Vec<Vec<f64>>) -> Self {
        DistMatrix { labels, data }
    }
    /// Number of taxa.
    pub fn n(&self) -> usize {
        self.labels.len()
    }
    /// Get distance between taxa i and j.
    pub fn get(&self, i: usize, j: usize) -> f64 {
        self.data[i][j]
    }
}
/// De Bruijn graph constructed from reads.
#[derive(Debug, Clone)]
pub struct DeBruijnGraph {
    pub k: usize,
    pub edges: Vec<DeBruijnEdge>,
}
impl DeBruijnGraph {
    /// Build a De Bruijn graph of order k from a list of sequence reads.
    pub fn build(reads: &[&str], k: usize) -> Self {
        let mut edges = Vec::new();
        for read in reads {
            let chars: Vec<char> = read.chars().collect();
            if chars.len() < k {
                continue;
            }
            for i in 0..=(chars.len() - k) {
                let kmer: String = chars[i..i + k].iter().collect();
                let from = DeBruijnNode(chars[i..i + k - 1].iter().collect());
                let to = DeBruijnNode(chars[i + 1..i + k].iter().collect());
                edges.push(DeBruijnEdge {
                    from,
                    to,
                    label: kmer,
                });
            }
        }
        DeBruijnGraph { k, edges }
    }
    /// Count in-degree of a node.
    pub fn in_degree(&self, node: &DeBruijnNode) -> usize {
        self.edges.iter().filter(|e| &e.to == node).count()
    }
    /// Count out-degree of a node.
    pub fn out_degree(&self, node: &DeBruijnNode) -> usize {
        self.edges.iter().filter(|e| &e.from == node).count()
    }
    /// Attempt a greedy Eulerian path (genome assembly).
    ///
    /// Returns assembled sequence if a path exists, otherwise partial result.
    pub fn greedy_assemble(&self) -> String {
        if self.edges.is_empty() {
            return String::new();
        }
        let mut used = vec![false; self.edges.len()];
        let start = self.edges[0].from.clone();
        let mut path = vec![start.clone()];
        let mut current = start;
        loop {
            let next_edge = self
                .edges
                .iter()
                .enumerate()
                .find(|(idx, e)| !used[*idx] && e.from == current);
            match next_edge {
                Some((idx, edge)) => {
                    used[idx] = true;
                    current = edge.to.clone();
                    path.push(current.clone());
                }
                None => break,
            }
        }
        if path.is_empty() {
            return String::new();
        }
        let mut result = path[0].0.clone();
        for node in &path[1..] {
            if let Some(last_char) = node.0.chars().last() {
                result.push(last_char);
            }
        }
        result
    }
}
/// Gene expression matrix (simplified).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GeneExpressionMatrix {
    pub genes: Vec<String>,
    pub samples: Vec<String>,
    pub data: Vec<Vec<f64>>,
}
#[allow(dead_code)]
impl GeneExpressionMatrix {
    /// Create a gene expression matrix.
    pub fn new(genes: Vec<String>, samples: Vec<String>, data: Vec<Vec<f64>>) -> Self {
        Self {
            genes,
            samples,
            data,
        }
    }
    /// Compute mean expression of gene at index i.
    pub fn mean_expression(&self, gene_idx: usize) -> f64 {
        if gene_idx >= self.data.len() || self.data[gene_idx].is_empty() {
            return 0.0;
        }
        let row = &self.data[gene_idx];
        row.iter().sum::<f64>() / row.len() as f64
    }
    /// Number of genes.
    pub fn num_genes(&self) -> usize {
        self.genes.len()
    }
    /// Number of samples.
    pub fn num_samples(&self) -> usize {
        self.samples.len()
    }
}
/// A global sequence aligner using the Needleman-Wunsch algorithm.
#[derive(Debug, Clone)]
pub struct NeedlemanWunschGlobal {
    pub gap_penalty: i32,
    pub match_score: i32,
    pub mismatch_score: i32,
}
impl NeedlemanWunschGlobal {
    /// Create a new global aligner.
    pub fn new(match_score: i32, mismatch_score: i32, gap_penalty: i32) -> Self {
        NeedlemanWunschGlobal {
            gap_penalty,
            match_score,
            mismatch_score,
        }
    }
    /// Globally align two sequences and return an `Alignment`.
    pub fn align(&self, a: &str, b: &str) -> Alignment {
        let a_chars: Vec<char> = a.chars().collect();
        let b_chars: Vec<char> = b.chars().collect();
        let m = a_chars.len();
        let n = b_chars.len();
        let ms = self.match_score;
        let mm = self.mismatch_score;
        let gap = self.gap_penalty;
        let mut dp = vec![vec![0i32; n + 1]; m + 1];
        for i in 0..=m {
            dp[i][0] = i as i32 * gap;
        }
        for j in 0..=n {
            dp[0][j] = j as i32 * gap;
        }
        for i in 1..=m {
            for j in 1..=n {
                let subst = if a_chars[i - 1] == b_chars[j - 1] {
                    ms
                } else {
                    mm
                };
                let mat = dp[i - 1][j - 1] + subst;
                let del = dp[i - 1][j] + gap;
                let ins = dp[i][j - 1] + gap;
                dp[i][j] = mat.max(del).max(ins);
            }
        }
        let mut aligned_a = String::new();
        let mut aligned_b = String::new();
        let mut i = m;
        let mut j = n;
        while i > 0 || j > 0 {
            if i > 0 && j > 0 {
                let subst = if a_chars[i - 1] == b_chars[j - 1] {
                    ms
                } else {
                    mm
                };
                if dp[i][j] == dp[i - 1][j - 1] + subst {
                    aligned_a.push(a_chars[i - 1]);
                    aligned_b.push(b_chars[j - 1]);
                    i -= 1;
                    j -= 1;
                    continue;
                }
            }
            if i > 0 && dp[i][j] == dp[i - 1][j] + gap {
                aligned_a.push(a_chars[i - 1]);
                aligned_b.push('-');
                i -= 1;
            } else {
                aligned_a.push('-');
                aligned_b.push(b_chars[j - 1]);
                j -= 1;
            }
        }
        let aligned_a: String = aligned_a.chars().rev().collect();
        let aligned_b: String = aligned_b.chars().rev().collect();
        Alignment {
            score: dp[m][n],
            aligned_a,
            aligned_b,
        }
    }
    /// Compute the alignment identity fraction.
    pub fn identity(&self, a: &str, b: &str) -> f64 {
        let aln = self.align(a, b);
        let len = aln.aligned_a.len().max(1);
        let matches = aln
            .aligned_a
            .chars()
            .zip(aln.aligned_b.chars())
            .filter(|(ca, cb)| ca == cb && *ca != '-')
            .count();
        matches as f64 / len as f64
    }
}
/// Protein secondary structure element.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum SecondaryStructure {
    AlphaHelix,
    BetaStrand,
    Loop,
    Turn,
}
#[allow(dead_code)]
impl SecondaryStructure {
    /// One-letter code.
    pub fn code(&self) -> char {
        match self {
            Self::AlphaHelix => 'H',
            Self::BetaStrand => 'E',
            Self::Loop => 'C',
            Self::Turn => 'T',
        }
    }
}
/// A directed edge in the De Bruijn graph.
#[derive(Debug, Clone, PartialEq)]
pub struct DeBruijnEdge {
    pub from: DeBruijnNode,
    pub to: DeBruijnNode,
    pub label: String,
}
/// A genome assembler based on de Bruijn graphs.
#[derive(Debug, Clone)]
pub struct DeBruijnAssembler {
    pub k: usize,
}
impl DeBruijnAssembler {
    /// Create a new assembler with k-mer length `k`.
    pub fn new(k: usize) -> Self {
        DeBruijnAssembler { k }
    }
    /// Assemble reads into contigs using the de Bruijn graph approach.
    pub fn assemble(&self, reads: &[&str]) -> Vec<String> {
        let graph = DeBruijnGraph::build(reads, self.k);
        let contig = graph.greedy_assemble();
        if contig.is_empty() {
            vec![]
        } else {
            vec![contig]
        }
    }
    /// Return the number of distinct k-mers across all reads.
    pub fn count_kmers(&self, reads: &[&str]) -> usize {
        let graph = DeBruijnGraph::build(reads, self.k);
        let mut labels: std::collections::HashSet<String> = std::collections::HashSet::new();
        for edge in &graph.edges {
            labels.insert(edge.label.clone());
        }
        labels.len()
    }
}
/// A reusable Smith-Waterman local aligner.
#[derive(Debug, Clone)]
pub struct SmithWatermanAligner {
    pub gap_penalty: i32,
    pub match_score: i32,
    pub mismatch_score: i32,
}
impl SmithWatermanAligner {
    /// Create a new `SmithWatermanAligner` with given scoring parameters.
    pub fn new(match_score: i32, mismatch_score: i32, gap_penalty: i32) -> Self {
        SmithWatermanAligner {
            gap_penalty,
            match_score,
            mismatch_score,
        }
    }
    /// Align two sequences and return the best local alignment.
    pub fn align(&self, a: &str, b: &str) -> Alignment {
        let a_chars: Vec<char> = a.chars().collect();
        let b_chars: Vec<char> = b.chars().collect();
        let m = a_chars.len();
        let n = b_chars.len();
        let ms = self.match_score;
        let mm = self.mismatch_score;
        let gap = self.gap_penalty;
        let mut dp = vec![vec![0i32; n + 1]; m + 1];
        let mut best_score = 0i32;
        let mut best_i = 0usize;
        let mut best_j = 0usize;
        for i in 1..=m {
            for j in 1..=n {
                let subst = if a_chars[i - 1] == b_chars[j - 1] {
                    ms
                } else {
                    mm
                };
                let mat = dp[i - 1][j - 1] + subst;
                let del = dp[i - 1][j] + gap;
                let ins = dp[i][j - 1] + gap;
                dp[i][j] = 0i32.max(mat).max(del).max(ins);
                if dp[i][j] > best_score {
                    best_score = dp[i][j];
                    best_i = i;
                    best_j = j;
                }
            }
        }
        let mut aligned_a = String::new();
        let mut aligned_b = String::new();
        let mut i = best_i;
        let mut j = best_j;
        while i > 0 && j > 0 && dp[i][j] > 0 {
            let subst = if a_chars[i - 1] == b_chars[j - 1] {
                ms
            } else {
                mm
            };
            if dp[i][j] == dp[i - 1][j - 1] + subst {
                aligned_a.push(a_chars[i - 1]);
                aligned_b.push(b_chars[j - 1]);
                i -= 1;
                j -= 1;
            } else if dp[i][j] == dp[i - 1][j] + gap {
                aligned_a.push(a_chars[i - 1]);
                aligned_b.push('-');
                i -= 1;
            } else {
                aligned_a.push('-');
                aligned_b.push(b_chars[j - 1]);
                j -= 1;
            }
        }
        let aligned_a: String = aligned_a.chars().rev().collect();
        let aligned_b: String = aligned_b.chars().rev().collect();
        Alignment {
            score: best_score,
            aligned_a,
            aligned_b,
        }
    }
    /// Compute only the best local alignment score (no traceback).
    pub fn score_only(&self, a: &str, b: &str) -> i32 {
        self.align(a, b).score
    }
}
/// A phylogenetic tree as a list of branches.
#[derive(Debug, Clone)]
pub struct PhyloTree {
    pub branches: Vec<PhyloBranch>,
}
impl PhyloTree {
    /// Neighbor-joining algorithm for phylogenetic tree reconstruction.
    pub fn neighbor_joining(mut dist: DistMatrix) -> PhyloTree {
        let mut branches = Vec::new();
        let mut active: Vec<usize> = (0..dist.n()).collect();
        let mut node_count = dist.n();
        while active.len() > 2 {
            let n = active.len();
            let r: Vec<f64> = active
                .iter()
                .map(|&i| active.iter().map(|&j| dist.get(i, j)).sum::<f64>())
                .collect();
            let mut min_q = f64::INFINITY;
            let mut min_pair = (0, 1);
            for ai in 0..n {
                for aj in (ai + 1)..n {
                    let i = active[ai];
                    let j = active[aj];
                    let q = (n as f64 - 2.0) * dist.get(i, j) - r[ai] - r[aj];
                    if q < min_q {
                        min_q = q;
                        min_pair = (ai, aj);
                    }
                }
            }
            let (ai, aj) = min_pair;
            let i = active[ai];
            let j = active[aj];
            let n_f = n as f64;
            let d_ij = dist.get(i, j);
            let len_i = 0.5 * d_ij + (r[ai] - r[aj]) / (2.0 * (n_f - 2.0));
            let len_j = d_ij - len_i;
            let new_label = format!("node{}", node_count);
            node_count += 1;
            branches.push(PhyloBranch {
                from: new_label.clone(),
                to: dist.labels[i].clone(),
                length: len_i.max(0.0),
            });
            branches.push(PhyloBranch {
                from: new_label.clone(),
                to: dist.labels[j].clone(),
                length: len_j.max(0.0),
            });
            let remaining: Vec<usize> = active
                .iter()
                .enumerate()
                .filter(|&(idx, _)| idx != ai && idx != aj)
                .map(|(_, &k)| k)
                .collect();
            let new_idx = dist.data.len();
            let mut new_row = vec![0.0f64; new_idx + 1];
            for &k in &remaining {
                let d_new_k = 0.5 * (dist.get(i, k) + dist.get(j, k) - d_ij);
                new_row[k] = d_new_k.max(0.0);
                dist.data[k].push(d_new_k.max(0.0));
            }
            dist.data.push(new_row);
            dist.labels.push(new_label);
            active.retain(|&x| x != i && x != j);
            active.push(new_idx);
        }
        if active.len() == 2 {
            let i = active[0];
            let j = active[1];
            branches.push(PhyloBranch {
                from: dist.labels[i].clone(),
                to: dist.labels[j].clone(),
                length: dist.get(i, j),
            });
        }
        PhyloTree { branches }
    }
}
/// Fitch-parsimony scorer for phylogenetic trees.
///
/// Wraps the recursive `fitch_parsimony_score` and adds convenience helpers.
#[derive(Debug, Clone)]
pub struct PhylogeneticParsimony;
impl PhylogeneticParsimony {
    /// Create a new parsimony scorer.
    pub fn new() -> Self {
        PhylogeneticParsimony
    }
    /// Score a `ParsimonyTree` using the Fitch algorithm.
    /// Returns (number_of_mutations, ancestral_character_set).
    pub fn score(&self, tree: &ParsimonyTree) -> (usize, std::collections::HashSet<char>) {
        fitch_parsimony_score(tree)
    }
    /// Build a balanced binary tree from a slice of leaf characters.
    pub fn build_balanced(leaves: &[char]) -> ParsimonyTree {
        match leaves.len() {
            0 => ParsimonyTree::Leaf('?'),
            1 => ParsimonyTree::Leaf(leaves[0]),
            _ => {
                let mid = leaves.len() / 2;
                let left = Self::build_balanced(&leaves[..mid]);
                let right = Self::build_balanced(&leaves[mid..]);
                ParsimonyTree::Internal(Box::new(left), Box::new(right))
            }
        }
    }
    /// Compute the parsimony score for a flat slice of leaf characters
    /// arranged in a balanced binary tree.
    pub fn score_leaves(&self, leaves: &[char]) -> usize {
        let tree = Self::build_balanced(leaves);
        self.score(&tree).0
    }
}
/// A discrete hidden Markov model.
#[derive(Debug, Clone)]
pub struct HiddenMarkovModel {
    pub n_states: usize,
    pub n_symbols: usize,
    pub initial: Vec<f64>,
    pub transition: Vec<Vec<f64>>,
    pub emission: Vec<Vec<f64>>,
}
impl HiddenMarkovModel {
    /// Create an HMM with uniform distributions.
    pub fn uniform(n_states: usize, n_symbols: usize) -> Self {
        let p_state = 1.0 / n_states as f64;
        let p_sym = 1.0 / n_symbols as f64;
        HiddenMarkovModel {
            n_states,
            n_symbols,
            initial: vec![p_state; n_states],
            transition: vec![vec![p_state; n_states]; n_states],
            emission: vec![vec![p_sym; n_symbols]; n_states],
        }
    }
    /// Run the Viterbi algorithm on a symbol sequence.
    ///
    /// Returns the most probable state path.
    pub fn viterbi(&self, observations: &[usize]) -> Vec<usize> {
        let t = observations.len();
        if t == 0 {
            return vec![];
        }
        let n = self.n_states;
        let mut dp = vec![vec![f64::NEG_INFINITY; n]; t];
        let mut backtrack = vec![vec![0usize; n]; t];
        for s in 0..n {
            let obs = observations[0];
            if obs < self.n_symbols {
                dp[0][s] = self.initial[s].ln() + self.emission[s][obs].ln();
            }
        }
        for ti in 1..t {
            let obs = observations[ti];
            for s in 0..n {
                let emit = if obs < self.n_symbols {
                    self.emission[s][obs].ln()
                } else {
                    f64::NEG_INFINITY
                };
                let (best_prev, best_val) = (0..n)
                    .map(|prev| {
                        let v = dp[ti - 1][prev] + self.transition[prev][s].ln();
                        (prev, v)
                    })
                    .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
                    .unwrap_or((0, f64::NEG_INFINITY));
                dp[ti][s] = best_val + emit;
                backtrack[ti][s] = best_prev;
            }
        }
        let mut path = vec![0usize; t];
        path[t - 1] = (0..n)
            .max_by(|&a, &b| {
                dp[t - 1][a]
                    .partial_cmp(&dp[t - 1][b])
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap_or(0);
        for ti in (1..t).rev() {
            path[ti - 1] = backtrack[ti][path[ti]];
        }
        path
    }
    /// Forward algorithm: compute P(observations | model).
    pub fn forward_probability(&self, observations: &[usize]) -> f64 {
        let t = observations.len();
        if t == 0 {
            return 1.0;
        }
        let n = self.n_states;
        let mut alpha = vec![0.0f64; n];
        for s in 0..n {
            let obs = observations[0];
            if obs < self.n_symbols {
                alpha[s] = self.initial[s] * self.emission[s][obs];
            }
        }
        for ti in 1..t {
            let obs = observations[ti];
            let mut alpha_new = vec![0.0f64; n];
            for s in 0..n {
                let emit = if obs < self.n_symbols {
                    self.emission[s][obs]
                } else {
                    0.0
                };
                alpha_new[s] = (0..n)
                    .map(|prev| alpha[prev] * self.transition[prev][s])
                    .sum::<f64>()
                    * emit;
            }
            alpha = alpha_new;
        }
        alpha.iter().sum()
    }
}
/// A simple tree node for parsimony scoring.
#[derive(Debug, Clone)]
pub enum ParsimonyTree {
    Leaf(char),
    Internal(Box<ParsimonyTree>, Box<ParsimonyTree>),
}
/// Residue type in the HP (hydrophobic-polar) model.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HPResidue {
    H,
    P,
}
/// Regulatory network node.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RegulatoryNode {
    pub gene_name: String,
    pub is_transcription_factor: bool,
    pub targets: Vec<String>,
}
#[allow(dead_code)]
impl RegulatoryNode {
    /// Create a regulatory node.
    pub fn new(name: &str, is_tf: bool) -> Self {
        Self {
            gene_name: name.to_string(),
            is_transcription_factor: is_tf,
            targets: Vec::new(),
        }
    }
    /// Add a regulatory target.
    pub fn add_target(&mut self, target: &str) {
        self.targets.push(target.to_string());
    }
    /// Out-degree.
    pub fn out_degree(&self) -> usize {
        self.targets.len()
    }
    /// Description.
    pub fn description(&self) -> String {
        if self.is_transcription_factor {
            format!(
                "TF {} regulates {} genes",
                self.gene_name,
                self.targets.len()
            )
        } else {
            format!("Gene {} (non-TF)", self.gene_name)
        }
    }
}
/// Hidden Markov Model for sequence analysis.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SequenceHmm {
    pub name: String,
    pub num_states: usize,
    pub alphabet_size: usize,
}
#[allow(dead_code)]
impl SequenceHmm {
    /// Profile HMM for protein family.
    pub fn profile(name: &str, length: usize) -> Self {
        Self {
            name: name.to_string(),
            num_states: 3 * length,
            alphabet_size: 20,
        }
    }
    /// CpG island detector.
    pub fn cpg_island_detector() -> Self {
        Self {
            name: "CpG-HMM".to_string(),
            num_states: 2,
            alphabet_size: 4,
        }
    }
    /// Viterbi decoding gives most likely state sequence.
    pub fn viterbi_description(&self) -> String {
        format!("Viterbi on {} ({} states)", self.name, self.num_states)
    }
}
