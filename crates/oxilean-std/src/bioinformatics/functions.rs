//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    Alignment, AlignmentScorer, BasePair, BlastHit, DeBruijnAssembler, DeBruijnGraph, DistMatrix,
    GeneExpressionMatrix, HPResidue, HiddenMarkovModel, LatticeMove, NeedlemanWunschGlobal,
    ParsimonyTree, PhyloTree, PhylogeneticParsimony, ProteinStructure, RNAMFEFolder,
    RegulatoryNode, SecondaryStructure, SequenceHmm, SmithWatermanAligner,
};

pub fn app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
pub fn app2(f: Expr, a: Expr, b: Expr) -> Expr {
    app(app(f, a), b)
}
pub fn app3(f: Expr, a: Expr, b: Expr, c: Expr) -> Expr {
    app(app2(f, a, b), c)
}
pub fn cst(s: &str) -> Expr {
    Expr::Const(Name::str(s), vec![])
}
pub fn prop() -> Expr {
    Expr::Sort(Level::zero())
}
pub fn type0() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
pub fn pi(bi: BinderInfo, name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Pi(bi, Name::str(name), Box::new(dom), Box::new(body))
}
pub fn arrow(a: Expr, b: Expr) -> Expr {
    pi(BinderInfo::Default, "_", a, b)
}
pub fn nat_ty() -> Expr {
    cst("Nat")
}
pub fn real_ty() -> Expr {
    cst("Real")
}
pub fn bool_ty() -> Expr {
    cst("Bool")
}
pub fn int_ty() -> Expr {
    cst("Int")
}
pub fn list_ty(a: Expr) -> Expr {
    app(cst("List"), a)
}
pub fn string_ty() -> Expr {
    cst("String")
}
/// `DNABase : Type` — nucleotide base (A, C, G, T).
pub fn dna_base_ty() -> Expr {
    type0()
}
/// `AminoAcid : Type` — amino acid residue.
pub fn amino_acid_ty() -> Expr {
    type0()
}
/// `Sequence : Type` — a sequence of characters (DNA/RNA/protein).
pub fn sequence_ty() -> Expr {
    list_ty(cst("Char"))
}
/// `AlignmentScore : Type` — real-valued alignment score.
pub fn alignment_score_ty() -> Expr {
    real_ty()
}
/// `EditDistance : Sequence → Sequence → Nat` — edit distance function type.
pub fn edit_distance_ty() -> Expr {
    arrow(sequence_ty(), arrow(sequence_ty(), nat_ty()))
}
/// `ScoreMatrix : Type` — substitution scoring matrix.
pub fn score_matrix_ty() -> Expr {
    arrow(cst("Char"), arrow(cst("Char"), real_ty()))
}
/// `DeBruijnGraph : Type` — graph for genome assembly.
pub fn debruijn_graph_ty() -> Expr {
    type0()
}
/// `PhyloTree : Type` — phylogenetic tree.
pub fn phylo_tree_ty() -> Expr {
    type0()
}
/// `HMM : Type` — hidden Markov model.
pub fn hmm_ty() -> Expr {
    type0()
}
/// `RNAStructure : Type` — RNA secondary structure (set of base pairs).
pub fn rna_structure_ty() -> Expr {
    list_ty(app2(cst("Prod"), nat_ty(), nat_ty()))
}
/// Needleman-Wunsch produces an optimal global alignment.
///
/// `needleman_wunsch_optimal : ∀ (s t : Sequence) (sc : ScoreMatrix),
///     IsOptimalGlobalAlignment (needleman_wunsch s t sc)`
pub fn needleman_wunsch_optimal_ty() -> Expr {
    let seq = sequence_ty();
    let sm = score_matrix_ty();
    let concl = app3(
        cst("IsOptimalGlobalAlignment"),
        cst("s"),
        cst("t"),
        app3(cst("needleman_wunsch"), cst("s"), cst("t"), cst("sc")),
    );
    arrow(seq.clone(), arrow(seq, arrow(sm, concl)))
}
/// Smith-Waterman score is non-negative.
///
/// `smith_waterman_nonneg : ∀ (s t : Sequence) (sc : ScoreMatrix),
///     0 ≤ smith_waterman_score s t sc`
pub fn smith_waterman_nonneg_ty() -> Expr {
    let seq = sequence_ty();
    let sm = score_matrix_ty();
    let score = app3(cst("smith_waterman_score"), cst("s"), cst("t"), cst("sc"));
    let concl = app2(cst("Le"), cst("Real.zero"), score);
    arrow(seq.clone(), arrow(seq, arrow(sm, concl)))
}
/// Levenshtein distance is a metric (triangle inequality).
///
/// `levenshtein_triangle : ∀ (a b c : Sequence),
///     edit_distance a c ≤ edit_distance a b + edit_distance b c`
pub fn levenshtein_triangle_ty() -> Expr {
    let seq = sequence_ty();
    let dab = app2(cst("edit_distance"), cst("a"), cst("b"));
    let dbc = app2(cst("edit_distance"), cst("b"), cst("c"));
    let dac = app2(cst("edit_distance"), cst("a"), cst("c"));
    let concl = app2(cst("Le"), dac, app2(cst("Nat.add"), dab, dbc));
    arrow(seq.clone(), arrow(seq.clone(), arrow(seq, concl)))
}
/// Nussinov algorithm maximises base-pair count.
///
/// `nussinov_maximum : ∀ (s : Sequence),
///     IsMaxBasePairing (nussinov s)`
pub fn nussinov_maximum_ty() -> Expr {
    let seq = sequence_ty();
    let concl = app(cst("IsMaxBasePairing"), app(cst("nussinov"), cst("s")));
    arrow(seq, concl)
}
/// Neighbor-joining produces an additive tree on the given distance matrix.
///
/// `neighbor_joining_additive : ∀ (D : DistMatrix),
///     IsAdditiveTree (neighbor_joining D) D`
pub fn neighbor_joining_additive_ty() -> Expr {
    let dm = cst("DistMatrix");
    let concl = app2(
        cst("IsAdditiveTree"),
        app(cst("neighbor_joining"), cst("D")),
        cst("D"),
    );
    arrow(dm, concl)
}
/// Viterbi algorithm finds the most probable state sequence for an HMM.
///
/// `viterbi_optimal : ∀ (h : HMM) (obs : Sequence),
///     IsMostProbableStateSeq (viterbi h obs) h obs`
pub fn viterbi_optimal_ty() -> Expr {
    let h_ty = hmm_ty();
    let seq = sequence_ty();
    let concl = app3(
        cst("IsMostProbableStateSeq"),
        app2(cst("viterbi"), cst("h"), cst("obs")),
        cst("h"),
        cst("obs"),
    );
    arrow(h_ty, arrow(seq, concl))
}
/// Register bioinformatics axioms and theorems in the OxiLean kernel environment.
pub fn build_bioinformatics_env(env: &mut Environment) -> Result<(), Box<dyn std::error::Error>> {
    let axioms: &[(&str, Expr)] = &[
        ("DNABase", dna_base_ty()),
        ("AminoAcid", amino_acid_ty()),
        ("AlignmentScore", alignment_score_ty()),
        ("ScoreMatrix", score_matrix_ty()),
        ("DeBruijnGraph", debruijn_graph_ty()),
        ("PhyloTree", phylo_tree_ty()),
        ("HMM", hmm_ty()),
        ("RNAStructure", rna_structure_ty()),
        (
            "IsOptimalGlobalAlignment",
            arrow(
                sequence_ty(),
                arrow(sequence_ty(), arrow(cst("Alignment"), prop())),
            ),
        ),
        ("IsMaxBasePairing", arrow(rna_structure_ty(), prop())),
        (
            "IsAdditiveTree",
            arrow(phylo_tree_ty(), arrow(cst("DistMatrix"), prop())),
        ),
        (
            "IsMostProbableStateSeq",
            arrow(sequence_ty(), arrow(hmm_ty(), arrow(sequence_ty(), prop()))),
        ),
        ("edit_distance", edit_distance_ty()),
        (
            "needleman_wunsch",
            arrow(
                sequence_ty(),
                arrow(sequence_ty(), arrow(score_matrix_ty(), cst("Alignment"))),
            ),
        ),
        (
            "smith_waterman_score",
            arrow(
                sequence_ty(),
                arrow(sequence_ty(), arrow(score_matrix_ty(), real_ty())),
            ),
        ),
        ("nussinov", arrow(sequence_ty(), rna_structure_ty())),
        (
            "neighbor_joining",
            arrow(cst("DistMatrix"), phylo_tree_ty()),
        ),
        (
            "viterbi",
            arrow(hmm_ty(), arrow(sequence_ty(), sequence_ty())),
        ),
        ("needleman_wunsch_optimal", needleman_wunsch_optimal_ty()),
        ("smith_waterman_nonneg", smith_waterman_nonneg_ty()),
        ("levenshtein_triangle", levenshtein_triangle_ty()),
        ("nussinov_maximum", nussinov_maximum_ty()),
        ("neighbor_joining_additive", neighbor_joining_additive_ty()),
        ("viterbi_optimal", viterbi_optimal_ty()),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    Ok(())
}
/// Substitution scoring function type.
pub type SubstScore = fn(char, char) -> i32;
/// Default BLOSUM-like scoring: +1 for match, -1 for mismatch.
pub fn default_score(a: char, b: char) -> i32 {
    if a == b {
        1
    } else {
        -1
    }
}
/// Needleman-Wunsch global sequence alignment.
///
/// Returns optimal global alignment score and aligned sequences.
pub fn needleman_wunsch(a: &str, b: &str, gap_penalty: i32, score_fn: SubstScore) -> Alignment {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let m = a_chars.len();
    let n = b_chars.len();
    let mut dp = vec![vec![0i32; n + 1]; m + 1];
    for i in 0..=m {
        dp[i][0] = i as i32 * gap_penalty;
    }
    for j in 0..=n {
        dp[0][j] = j as i32 * gap_penalty;
    }
    for i in 1..=m {
        for j in 1..=n {
            let mat = dp[i - 1][j - 1] + score_fn(a_chars[i - 1], b_chars[j - 1]);
            let del = dp[i - 1][j] + gap_penalty;
            let ins = dp[i][j - 1] + gap_penalty;
            dp[i][j] = mat.max(del).max(ins);
        }
    }
    let mut aligned_a = String::new();
    let mut aligned_b = String::new();
    let mut i = m;
    let mut j = n;
    while i > 0 || j > 0 {
        if i > 0 && j > 0 {
            let mat = dp[i - 1][j - 1] + score_fn(a_chars[i - 1], b_chars[j - 1]);
            if dp[i][j] == mat {
                aligned_a.push(a_chars[i - 1]);
                aligned_b.push(b_chars[j - 1]);
                i -= 1;
                j -= 1;
                continue;
            }
        }
        if i > 0 && dp[i][j] == dp[i - 1][j] + gap_penalty {
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
/// Smith-Waterman local sequence alignment.
///
/// Returns the best local alignment score and substrings.
pub fn smith_waterman(a: &str, b: &str, gap_penalty: i32, score_fn: SubstScore) -> Alignment {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let m = a_chars.len();
    let n = b_chars.len();
    let mut dp = vec![vec![0i32; n + 1]; m + 1];
    let mut best_score = 0i32;
    let mut best_i = 0usize;
    let mut best_j = 0usize;
    for i in 1..=m {
        for j in 1..=n {
            let mat = dp[i - 1][j - 1] + score_fn(a_chars[i - 1], b_chars[j - 1]);
            let del = dp[i - 1][j] + gap_penalty;
            let ins = dp[i][j - 1] + gap_penalty;
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
        let mat = dp[i - 1][j - 1] + score_fn(a_chars[i - 1], b_chars[j - 1]);
        if dp[i][j] == mat {
            aligned_a.push(a_chars[i - 1]);
            aligned_b.push(b_chars[j - 1]);
            i -= 1;
            j -= 1;
        } else if dp[i][j] == dp[i - 1][j] + gap_penalty {
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
/// Simplified BLAST: find all exact k-mer seeds between query and subject.
pub fn blast_seed_hits(query: &str, subject: &str, word_size: usize) -> Vec<BlastHit> {
    let mut hits = Vec::new();
    let q_chars: Vec<char> = query.chars().collect();
    let s_chars: Vec<char> = subject.chars().collect();
    if word_size == 0 || q_chars.len() < word_size || s_chars.len() < word_size {
        return hits;
    }
    for qi in 0..=(q_chars.len() - word_size) {
        let word: String = q_chars[qi..qi + word_size].iter().collect();
        for si in 0..=(s_chars.len() - word_size) {
            let subj_word: String = s_chars[si..si + word_size].iter().collect();
            if word == subj_word {
                hits.push(BlastHit {
                    query_pos: qi,
                    subject_pos: si,
                    word: word.clone(),
                    score: word_size as i32,
                });
            }
        }
    }
    hits
}
/// Compute the Levenshtein edit distance between two strings.
pub fn levenshtein(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let m = a_chars.len();
    let n = b_chars.len();
    let mut prev: Vec<usize> = (0..=n).collect();
    let mut curr = vec![0usize; n + 1];
    for i in 1..=m {
        curr[0] = i;
        for j in 1..=n {
            let cost = if a_chars[i - 1] == b_chars[j - 1] {
                0
            } else {
                1
            };
            curr[j] = (curr[j - 1] + 1).min(prev[j] + 1).min(prev[j - 1] + cost);
        }
        std::mem::swap(&mut prev, &mut curr);
    }
    prev[n]
}
/// Fitch parsimony: compute minimum number of mutations on a binary tree.
pub fn fitch_parsimony_score(tree: &ParsimonyTree) -> (usize, std::collections::HashSet<char>) {
    match tree {
        ParsimonyTree::Leaf(c) => {
            let mut s = std::collections::HashSet::new();
            s.insert(*c);
            (0, s)
        }
        ParsimonyTree::Internal(left, right) => {
            let (sl, sl_set) = fitch_parsimony_score(left);
            let (sr, sr_set) = fitch_parsimony_score(right);
            let intersection: std::collections::HashSet<char> =
                sl_set.intersection(&sr_set).cloned().collect();
            if intersection.is_empty() {
                let union: std::collections::HashSet<char> =
                    sl_set.union(&sr_set).cloned().collect();
                (sl + sr + 1, union)
            } else {
                (sl + sr, intersection)
            }
        }
    }
}
/// Check if two RNA bases can form a Watson-Crick or wobble pair.
pub fn can_pair(a: char, b: char) -> bool {
    matches!(
        (a, b),
        ('A', 'U') | ('U', 'A') | ('G', 'C') | ('C', 'G') | ('G', 'U') | ('U', 'G')
    )
}
/// Nussinov algorithm for RNA secondary structure prediction.
///
/// Returns the maximum number of base pairs and the set of pairs.
pub fn nussinov(sequence: &str) -> (usize, Vec<BasePair>) {
    let seq: Vec<char> = sequence.chars().collect();
    let n = seq.len();
    if n == 0 {
        return (0, vec![]);
    }
    let mut dp = vec![vec![0usize; n]; n];
    for len in 2..=n {
        for i in 0..=(n - len) {
            let j = i + len - 1;
            dp[i][j] = dp[i][j - 1];
            for k in i..j {
                if can_pair(seq[k], seq[j]) {
                    let pair_score = if k > 0 { dp[i][k - 1] } else { 0 };
                    let inner = if k < j - 1 { dp[k + 1][j - 1] } else { 0 };
                    let s = pair_score + inner + 1;
                    if s > dp[i][j] {
                        dp[i][j] = s;
                    }
                }
            }
        }
    }
    let mut pairs = Vec::new();
    let mut stack: Vec<(usize, usize)> = Vec::new();
    if n > 0 {
        stack.push((0, n - 1));
    }
    while let Some((i, j)) = stack.pop() {
        if i >= j {
            continue;
        }
        if dp[i][j] == dp[i][j - 1] {
            stack.push((i, j - 1));
        } else {
            for k in i..j {
                if can_pair(seq[k], seq[j]) {
                    let pair_score = if k > 0 { dp[i][k - 1] } else { 0 };
                    let inner = if k < j - 1 { dp[k + 1][j - 1] } else { 0 };
                    if pair_score + inner + 1 == dp[i][j] {
                        pairs.push(BasePair { i: k, j });
                        if k > 0 {
                            stack.push((i, k - 1));
                        }
                        if k < j - 1 {
                            stack.push((k + 1, j - 1));
                        }
                        break;
                    }
                }
            }
        }
    }
    (dp[0][n - 1], pairs)
}
/// Compute HP lattice energy for a given conformation.
///
/// Energy = -1 for each non-bonded H-H topological contact.
pub fn hp_energy(sequence: &[HPResidue], moves: &[LatticeMove]) -> i32 {
    if sequence.is_empty() || moves.len() + 1 != sequence.len() {
        return 0;
    }
    let mut positions: Vec<(i32, i32)> = Vec::with_capacity(sequence.len());
    positions.push((0, 0));
    for &mv in moves {
        let (x, y) = *positions
            .last()
            .expect("positions is non-empty: initialized with one element");
        let next = match mv {
            LatticeMove::Up => (x, y + 1),
            LatticeMove::Down => (x, y - 1),
            LatticeMove::Left => (x - 1, y),
            LatticeMove::Right => (x + 1, y),
        };
        positions.push(next);
    }
    let mut energy = 0i32;
    let n = positions.len();
    for i in 0..n {
        for j in (i + 2)..n {
            if sequence[i] == HPResidue::H && sequence[j] == HPResidue::H {
                let (xi, yi) = positions[i];
                let (xj, yj) = positions[j];
                let manhattan = (xi - xj).abs() + (yi - yj).abs();
                if manhattan == 1 {
                    energy -= 1;
                }
            }
        }
    }
    energy
}
/// `SmithWatermanOptimal : ∀ (s t : Sequence) (sc : ScoreMatrix), IsOptimalLocalAlignment (smith_waterman s t sc) s t sc`
pub fn smith_waterman_optimal_ty() -> Expr {
    let seq = sequence_ty();
    let sm = score_matrix_ty();
    let concl = app3(
        cst("IsOptimalLocalAlignment"),
        app3(cst("smith_waterman"), cst("s"), cst("t"), cst("sc")),
        cst("s"),
        cst("t"),
    );
    arrow(seq.clone(), arrow(seq, arrow(sm, concl)))
}
/// `AffineGapOptimal : ∀ (s t : Sequence) (o e : Real), IsOptimalAffineAlignment (affine_gap_align s t o e)`
pub fn affine_gap_optimal_ty() -> Expr {
    let seq = sequence_ty();
    let concl = app2(
        cst("IsOptimalAffineAlignment"),
        app3(
            app(cst("affine_gap_align"), cst("s")),
            cst("t"),
            cst("o"),
            cst("e"),
        ),
        cst("s"),
    );
    arrow(
        seq.clone(),
        arrow(seq, arrow(real_ty(), arrow(real_ty(), concl))),
    )
}
/// `NeedlemanWunschUnique : ∀ (s t : Sequence) (sc : ScoreMatrix), UniqueOptimalGlobalAlignment s t sc`
pub fn needleman_wunsch_unique_ty() -> Expr {
    let seq = sequence_ty();
    let sm = score_matrix_ty();
    let concl = app3(
        cst("UniqueOptimalGlobalAlignment"),
        cst("s"),
        cst("t"),
        cst("sc"),
    );
    arrow(seq.clone(), arrow(seq, arrow(sm, concl)))
}
/// `MSAConsistency : ∀ (seqs : List Sequence), IsPairwiseConsistent (msa seqs)`
pub fn msa_consistency_ty() -> Expr {
    let seqs = list_ty(sequence_ty());
    let concl = app(cst("IsPairwiseConsistent"), app(cst("msa"), cst("seqs")));
    arrow(seqs, concl)
}
/// `ProgressiveAlignmentConverges : ∀ (guide_tree : PhyloTree) (seqs : List Sequence), Converges (progressive_align guide_tree seqs)`
pub fn progressive_alignment_converges_ty() -> Expr {
    let tree = phylo_tree_ty();
    let seqs = list_ty(sequence_ty());
    let concl = app(
        cst("Converges"),
        app2(cst("progressive_align"), cst("guide_tree"), cst("seqs")),
    );
    arrow(tree, arrow(seqs, concl))
}
/// `MaximumParsimony : ∀ (seqs : List Sequence), IsMinimumMutationTree (max_parsimony_tree seqs) seqs`
pub fn maximum_parsimony_ty() -> Expr {
    let seqs = list_ty(sequence_ty());
    let concl = app2(
        cst("IsMinimumMutationTree"),
        app(cst("max_parsimony_tree"), cst("seqs")),
        cst("seqs"),
    );
    arrow(seqs, concl)
}
/// `MaximumLikelihoodTree : ∀ (seqs : List Sequence) (model : SubstModel), MaximizesLikelihood (ml_tree seqs model) seqs model`
pub fn maximum_likelihood_tree_ty() -> Expr {
    let seqs = list_ty(sequence_ty());
    let model = cst("SubstModel");
    let concl = app3(
        cst("MaximizesLikelihood"),
        app2(cst("ml_tree"), cst("seqs"), cst("model")),
        cst("seqs"),
        cst("model"),
    );
    arrow(seqs, arrow(model, concl))
}
/// `BayesianPosteriorTree : ∀ (seqs : List Sequence) (prior : Prior), IsPosteriorModeSampler (bayesian_tree seqs prior)`
pub fn bayesian_posterior_tree_ty() -> Expr {
    let seqs = list_ty(sequence_ty());
    let prior = cst("Prior");
    let concl = app(
        cst("IsPosteriorModeSampler"),
        app2(cst("bayesian_tree"), cst("seqs"), cst("prior")),
    );
    arrow(seqs, arrow(prior, concl))
}
/// `DeBruijnEulerianPath : ∀ (reads : List Sequence) (k : Nat), HasEulerianPath (debruijn_graph reads k)`
pub fn debruijn_eulerian_path_ty() -> Expr {
    let reads = list_ty(sequence_ty());
    let concl = app(
        cst("HasEulerianPath"),
        app2(cst("debruijn_graph"), cst("reads"), cst("k")),
    );
    arrow(reads, arrow(nat_ty(), concl))
}
/// `OLCAssemblyCorrect : ∀ (reads : List Sequence), IsCorrectAssembly (olc_assemble reads) reads`
pub fn olc_assembly_correct_ty() -> Expr {
    let reads = list_ty(sequence_ty());
    let concl = app2(
        cst("IsCorrectAssembly"),
        app(cst("olc_assemble"), cst("reads")),
        cst("reads"),
    );
    arrow(reads, concl)
}
/// `RepeatResolutionComplete : ∀ (g : DeBruijnGraph) (cov : Real), RepeatsResolved (resolve_repeats g cov)`
pub fn repeat_resolution_complete_ty() -> Expr {
    let g = debruijn_graph_ty();
    let concl = app(
        cst("RepeatsResolved"),
        app2(cst("resolve_repeats"), cst("g"), cst("cov")),
    );
    arrow(g, arrow(real_ty(), concl))
}
/// `HMMGeneModel : ∀ (genome : Sequence) (h : HMM), IsGeneAnnotation (hmm_gene_predict genome h) genome`
pub fn hmm_gene_model_ty() -> Expr {
    let genome = sequence_ty();
    let h = hmm_ty();
    let concl = app2(
        cst("IsGeneAnnotation"),
        app2(cst("hmm_gene_predict"), cst("genome"), cst("h")),
        cst("genome"),
    );
    arrow(genome, arrow(h, concl))
}
/// `ORFFinding : ∀ (s : Sequence), ContainsStartStop (find_orfs s)`
pub fn orf_finding_ty() -> Expr {
    let seq = sequence_ty();
    let concl = app(cst("ContainsStartStop"), app(cst("find_orfs"), cst("s")));
    arrow(seq, concl)
}
/// `SpliceSiteConsensus : ∀ (s : Sequence), ObeysDonorAcceptorConsensus (predict_splice_sites s)`
pub fn splice_site_consensus_ty() -> Expr {
    let seq = sequence_ty();
    let concl = app(
        cst("ObeysDonorAcceptorConsensus"),
        app(cst("predict_splice_sites"), cst("s")),
    );
    arrow(seq, concl)
}
/// `RamachandranAllowed : ∀ (phi psi : Real), IsAllowedConformation phi psi`
pub fn ramachandran_allowed_ty() -> Expr {
    let concl = app2(cst("IsAllowedConformation"), cst("phi"), cst("psi"));
    arrow(real_ty(), arrow(real_ty(), concl))
}
/// `SecondaryStructurePrediction : ∀ (seq : Sequence), HasSecondaryStructure (predict_ss seq) seq`
pub fn secondary_structure_prediction_ty() -> Expr {
    let seq = sequence_ty();
    let concl = app2(
        cst("HasSecondaryStructure"),
        app(cst("predict_ss"), cst("seq")),
        cst("seq"),
    );
    arrow(seq, concl)
}
/// `FoldingFreeEnergy : ∀ (structure : ProteinStructure), FoldingEnergyBelowThreshold (folding_energy structure)`
pub fn folding_free_energy_ty() -> Expr {
    let struc = cst("ProteinStructure");
    let concl = app(
        cst("FoldingEnergyBelowThreshold"),
        app(cst("folding_energy"), cst("structure")),
    );
    arrow(struc, concl)
}
/// `RNAMinFreeEnergy : ∀ (s : Sequence), IsMinimumFreeEnergyStructure (mfe_fold s) s`
pub fn rna_mfe_ty() -> Expr {
    let seq = sequence_ty();
    let concl = app2(
        cst("IsMinimumFreeEnergyStructure"),
        app(cst("mfe_fold"), cst("s")),
        cst("s"),
    );
    arrow(seq, concl)
}
/// `RNAPartitionFunction : ∀ (s : Sequence), PartitionFunctionPositive (partition_fn s)`
pub fn rna_partition_function_ty() -> Expr {
    let seq = sequence_ty();
    let concl = app(
        cst("PartitionFunctionPositive"),
        app(cst("partition_fn"), cst("s")),
    );
    arrow(seq, concl)
}
/// `BasePairProbabilities : ∀ (s : Sequence), ProbabilitiesSumToOne (bp_probabilities s)`
pub fn base_pair_probabilities_ty() -> Expr {
    let seq = sequence_ty();
    let concl = app(
        cst("ProbabilitiesSumToOne"),
        app(cst("bp_probabilities"), cst("s")),
    );
    arrow(seq, concl)
}
/// `ReadClassification : ∀ (r : Sequence) (db : TaxonomyDB), IsAssignedTaxon (classify_read r db) r`
pub fn read_classification_ty() -> Expr {
    let r = sequence_ty();
    let db = cst("TaxonomyDB");
    let concl = app2(
        cst("IsAssignedTaxon"),
        app2(cst("classify_read"), cst("r"), cst("db")),
        cst("r"),
    );
    arrow(r, arrow(db, concl))
}
/// `CommunityProfileNormalized : ∀ (reads : List Sequence) (db : TaxonomyDB), ProfileSumsToOne (community_profile reads db)`
pub fn community_profile_normalized_ty() -> Expr {
    let reads = list_ty(sequence_ty());
    let db = cst("TaxonomyDB");
    let concl = app(
        cst("ProfileSumsToOne"),
        app2(cst("community_profile"), cst("reads"), cst("db")),
    );
    arrow(reads, arrow(db, concl))
}
/// `VariantQualityScore : ∀ (site : GenomicSite) (reads : List Sequence), QualityScoreValid (variant_quality site reads)`
pub fn variant_quality_score_ty() -> Expr {
    let site = cst("GenomicSite");
    let reads = list_ty(sequence_ty());
    let concl = app(
        cst("QualityScoreValid"),
        app2(cst("variant_quality"), cst("site"), cst("reads")),
    );
    arrow(site, arrow(reads, concl))
}
/// `GenotypeLikelihood : ∀ (geno : Genotype) (reads : List Sequence), LikelihoodInUnitInterval (genotype_likelihood geno reads)`
pub fn genotype_likelihood_ty() -> Expr {
    let geno = cst("Genotype");
    let reads = list_ty(sequence_ty());
    let concl = app(
        cst("LikelihoodInUnitInterval"),
        app2(cst("genotype_likelihood"), cst("geno"), cst("reads")),
    );
    arrow(geno, arrow(reads, concl))
}
/// `PhasingConsistency : ∀ (variants : List Variant) (reads : List Sequence), IsHaplotypicallyCons (phase_variants variants reads)`
pub fn phasing_consistency_ty() -> Expr {
    let variants = list_ty(cst("Variant"));
    let reads = list_ty(sequence_ty());
    let concl = app(
        cst("IsHaplotypicallyCons"),
        app2(cst("phase_variants"), cst("variants"), cst("reads")),
    );
    arrow(variants, arrow(reads, concl))
}
/// `PeakCallingEnrichment : ∀ (signal control : List Real), PeakEnriched (call_peaks signal control)`
pub fn peak_calling_enrichment_ty() -> Expr {
    let sig = list_ty(real_ty());
    let concl = app(
        cst("PeakEnriched"),
        app2(cst("call_peaks"), cst("signal"), cst("control")),
    );
    arrow(sig.clone(), arrow(sig, concl))
}
/// `MotifDiscoverySignificant : ∀ (peaks : List Sequence), MotifStatisticallySignificant (discover_motifs peaks)`
pub fn motif_discovery_significant_ty() -> Expr {
    let peaks = list_ty(sequence_ty());
    let concl = app(
        cst("MotifStatisticallySignificant"),
        app(cst("discover_motifs"), cst("peaks")),
    );
    arrow(peaks, concl)
}
/// `SNPAssociationFDR : ∀ (snps : List SNP) (phenotype : List Real) (alpha : Real), PassesFDRThreshold (gwas_test snps phenotype) alpha`
pub fn snp_association_fdr_ty() -> Expr {
    let snps = list_ty(cst("SNP"));
    let pheno = list_ty(real_ty());
    let concl = app2(
        cst("PassesFDRThreshold"),
        app2(cst("gwas_test"), cst("snps"), cst("phenotype")),
        cst("alpha"),
    );
    arrow(snps, arrow(pheno, arrow(real_ty(), concl)))
}
/// `PopulationStratificationCorrection : ∀ (pcs : Nat) (data : GWASData), StratificationControlled (correct_stratification data pcs)`
pub fn population_stratification_ty() -> Expr {
    let data = cst("GWASData");
    let concl = app(
        cst("StratificationControlled"),
        app2(cst("correct_stratification"), cst("data"), cst("pcs")),
    );
    arrow(nat_ty(), arrow(data, concl))
}
/// `LinkageDisequilibriumBound : ∀ (s1 s2 : SNP), LDBetweenZeroAndOne (compute_ld s1 s2)`
pub fn linkage_disequilibrium_bound_ty() -> Expr {
    let snp = cst("SNP");
    let concl = app(
        cst("LDBetweenZeroAndOne"),
        app2(cst("compute_ld"), cst("s1"), cst("s2")),
    );
    arrow(snp.clone(), arrow(snp, concl))
}
/// `BindingAffinityPositive : ∀ (drug target : Molecule), BindingAffinityNonNeg (binding_affinity drug target)`
pub fn binding_affinity_positive_ty() -> Expr {
    let mol = cst("Molecule");
    let concl = app(
        cst("BindingAffinityNonNeg"),
        app2(cst("binding_affinity"), cst("drug"), cst("target")),
    );
    arrow(mol.clone(), arrow(mol, concl))
}
/// `DockingScoreConsistent : ∀ (pose : DockingPose), DockingScoreFinite (docking_score pose)`
pub fn docking_score_consistent_ty() -> Expr {
    let pose = cst("DockingPose");
    let concl = app(
        cst("DockingScoreFinite"),
        app(cst("docking_score"), cst("pose")),
    );
    arrow(pose, concl)
}
/// `ScRNAClusteringStable : ∀ (counts : ExprMatrix) (k : Nat), ClusteringStable (scrna_cluster counts k)`
pub fn scrna_clustering_stable_ty() -> Expr {
    let matrix = cst("ExprMatrix");
    let concl = app(
        cst("ClusteringStable"),
        app2(cst("scrna_cluster"), cst("counts"), cst("k")),
    );
    arrow(matrix, arrow(nat_ty(), concl))
}
/// `TrajectoryInferenceContinuous : ∀ (counts : ExprMatrix), TrajectoryIsContinuous (infer_trajectory counts)`
pub fn trajectory_inference_continuous_ty() -> Expr {
    let matrix = cst("ExprMatrix");
    let concl = app(
        cst("TrajectoryIsContinuous"),
        app(cst("infer_trajectory"), cst("counts")),
    );
    arrow(matrix, concl)
}
/// `BatchCorrectionPreservesSignal : ∀ (batches : List ExprMatrix), SignalPreserved (correct_batch batches)`
pub fn batch_correction_preserves_signal_ty() -> Expr {
    let batches = list_ty(cst("ExprMatrix"));
    let concl = app(
        cst("SignalPreserved"),
        app(cst("correct_batch"), cst("batches")),
    );
    arrow(batches, concl)
}
/// `PPINetworkConnected : ∀ (proteins : List Protein), NetworkConnected (build_ppi proteins)`
pub fn ppi_network_connected_ty() -> Expr {
    let proteins = list_ty(cst("Protein"));
    let concl = app(
        cst("NetworkConnected"),
        app(cst("build_ppi"), cst("proteins")),
    );
    arrow(proteins, concl)
}
/// `PathwayEnrichmentSignificant : ∀ (gene_list : List Gene) (pathway_db : PathwayDB), EnrichmentSignificant (pathway_enrich gene_list pathway_db)`
pub fn pathway_enrichment_significant_ty() -> Expr {
    let genes = list_ty(cst("Gene"));
    let db = cst("PathwayDB");
    let concl = app(
        cst("EnrichmentSignificant"),
        app2(cst("pathway_enrich"), cst("gene_list"), cst("pathway_db")),
    );
    arrow(genes, arrow(db, concl))
}
/// `DNAMethylationCpGBimodal : ∀ (cpg_sites : List CpGSite), MethylationBimodal (measure_methylation cpg_sites)`
pub fn dna_methylation_bimodal_ty() -> Expr {
    let cpg = list_ty(cst("CpGSite"));
    let concl = app(
        cst("MethylationBimodal"),
        app(cst("measure_methylation"), cst("cpg_sites")),
    );
    arrow(cpg, concl)
}
/// `HistoneModificationEnriched : ∀ (mark : HistoneMark) (region : GenomicRegion), MarkEnriched (histone_enrichment mark region)`
pub fn histone_modification_enriched_ty() -> Expr {
    let mark = cst("HistoneMark");
    let region = cst("GenomicRegion");
    let concl = app(
        cst("MarkEnriched"),
        app2(cst("histone_enrichment"), cst("mark"), cst("region")),
    );
    arrow(mark, arrow(region, concl))
}
/// `ChromatinAccessibilityNormalized : ∀ (atac_reads : List Sequence), AccessibilityScoreNormalized (compute_accessibility atac_reads)`
pub fn chromatin_accessibility_normalized_ty() -> Expr {
    let reads = list_ty(sequence_ty());
    let concl = app(
        cst("AccessibilityScoreNormalized"),
        app(cst("compute_accessibility"), cst("atac_reads")),
    );
    arrow(reads, concl)
}
/// Register the extended bioinformatics axioms in the OxiLean kernel environment.
pub fn build_bioinformatics_env_extended(
    env: &mut Environment,
) -> Result<(), Box<dyn std::error::Error>> {
    let axioms: &[(&str, Expr)] = &[
        ("smith_waterman_optimal", smith_waterman_optimal_ty()),
        ("affine_gap_optimal", affine_gap_optimal_ty()),
        ("needleman_wunsch_unique", needleman_wunsch_unique_ty()),
        ("msa_consistency", msa_consistency_ty()),
        (
            "progressive_alignment_converges",
            progressive_alignment_converges_ty(),
        ),
        ("maximum_parsimony", maximum_parsimony_ty()),
        ("maximum_likelihood_tree", maximum_likelihood_tree_ty()),
        ("bayesian_posterior_tree", bayesian_posterior_tree_ty()),
        ("debruijn_eulerian_path", debruijn_eulerian_path_ty()),
        ("olc_assembly_correct", olc_assembly_correct_ty()),
        (
            "repeat_resolution_complete",
            repeat_resolution_complete_ty(),
        ),
        ("hmm_gene_model", hmm_gene_model_ty()),
        ("orf_finding", orf_finding_ty()),
        ("splice_site_consensus", splice_site_consensus_ty()),
        ("ramachandran_allowed", ramachandran_allowed_ty()),
        (
            "secondary_structure_prediction",
            secondary_structure_prediction_ty(),
        ),
        ("folding_free_energy", folding_free_energy_ty()),
        ("rna_mfe", rna_mfe_ty()),
        ("rna_partition_function", rna_partition_function_ty()),
        ("base_pair_probabilities", base_pair_probabilities_ty()),
        ("read_classification", read_classification_ty()),
        (
            "community_profile_normalized",
            community_profile_normalized_ty(),
        ),
        ("variant_quality_score", variant_quality_score_ty()),
        ("genotype_likelihood", genotype_likelihood_ty()),
        ("phasing_consistency", phasing_consistency_ty()),
        ("peak_calling_enrichment", peak_calling_enrichment_ty()),
        (
            "motif_discovery_significant",
            motif_discovery_significant_ty(),
        ),
        ("snp_association_fdr", snp_association_fdr_ty()),
        ("population_stratification", population_stratification_ty()),
        (
            "linkage_disequilibrium_bound",
            linkage_disequilibrium_bound_ty(),
        ),
        ("binding_affinity_positive", binding_affinity_positive_ty()),
        ("docking_score_consistent", docking_score_consistent_ty()),
        ("scrna_clustering_stable", scrna_clustering_stable_ty()),
        (
            "trajectory_inference_continuous",
            trajectory_inference_continuous_ty(),
        ),
        (
            "batch_correction_preserves_signal",
            batch_correction_preserves_signal_ty(),
        ),
        ("ppi_network_connected", ppi_network_connected_ty()),
        (
            "pathway_enrichment_significant",
            pathway_enrichment_significant_ty(),
        ),
        ("dna_methylation_bimodal", dna_methylation_bimodal_ty()),
        (
            "histone_modification_enriched",
            histone_modification_enriched_ty(),
        ),
        (
            "chromatin_accessibility_normalized",
            chromatin_accessibility_normalized_ty(),
        ),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_needleman_wunsch_identical() {
        let aln = needleman_wunsch("ACGT", "ACGT", -1, default_score);
        assert_eq!(aln.score, 4);
        assert_eq!(aln.aligned_a, "ACGT");
        assert_eq!(aln.aligned_b, "ACGT");
    }
    #[test]
    fn test_needleman_wunsch_different() {
        let aln = needleman_wunsch("AG", "CT", -1, default_score);
        assert_eq!(aln.score, -2);
    }
    #[test]
    fn test_smith_waterman_local() {
        let aln = smith_waterman("AACGTAAGC", "CGTAA", -2, default_score);
        assert!(aln.score >= 3, "expected score >= 3, got {}", aln.score);
    }
    #[test]
    fn test_levenshtein_identical() {
        assert_eq!(levenshtein("kitten", "kitten"), 0);
    }
    #[test]
    fn test_levenshtein_classic() {
        assert_eq!(levenshtein("kitten", "sitting"), 3);
    }
    #[test]
    fn test_debruijn_build_and_assemble() {
        let graph = DeBruijnGraph::build(&["ACGT"], 3);
        assert_eq!(graph.edges.len(), 2);
        let assembled = graph.greedy_assemble();
        assert!(assembled.contains("AC"), "assembled should contain 'AC'");
    }
    #[test]
    fn test_nussinov_simple() {
        let (score, pairs) = nussinov("AUGCAU");
        assert!(score > 0 || pairs.is_empty());
    }
    #[test]
    fn test_hmm_viterbi_length() {
        let hmm = HiddenMarkovModel::uniform(2, 4);
        let obs = vec![0, 1, 2, 3, 0];
        let path = hmm.viterbi(&obs);
        assert_eq!(path.len(), obs.len(), "path length must match observations");
        assert!(path.iter().all(|&s| s < 2), "all states must be valid");
    }
    #[test]
    fn test_blast_seed_hits() {
        let hits = blast_seed_hits("ACGTACGT", "TACGTAC", 4);
        assert!(!hits.is_empty(), "should find at least one 4-mer hit");
        assert!(
            hits.iter().all(|h| h.word.len() == 4),
            "all words must be of length 4"
        );
    }
    #[test]
    fn test_neighbor_joining_builds_tree() {
        let labels = vec!["A".into(), "B".into(), "C".into(), "D".into()];
        let data = vec![
            vec![0.0, 5.0, 9.0, 9.0],
            vec![5.0, 0.0, 10.0, 10.0],
            vec![9.0, 10.0, 0.0, 8.0],
            vec![9.0, 10.0, 8.0, 0.0],
        ];
        let dm = DistMatrix::new(labels, data);
        let tree = PhyloTree::neighbor_joining(dm);
        assert!(!tree.branches.is_empty(), "tree must have branches");
    }
    #[test]
    fn test_build_bioinformatics_env() {
        let mut env = Environment::new();
        build_bioinformatics_env(&mut env).expect("build should succeed");
    }
    #[test]
    fn test_smith_waterman_optimal_ty_builds() {
        let ty = smith_waterman_optimal_ty();
        assert!(matches!(ty, Expr::Pi(..)));
    }
    #[test]
    fn test_msa_consistency_ty_builds() {
        let ty = msa_consistency_ty();
        assert!(matches!(ty, Expr::Pi(..)));
    }
    #[test]
    fn test_debruijn_eulerian_path_ty_builds() {
        let ty = debruijn_eulerian_path_ty();
        assert!(matches!(ty, Expr::Pi(..)));
    }
    #[test]
    fn test_rna_mfe_ty_builds() {
        let ty = rna_mfe_ty();
        assert!(matches!(ty, Expr::Pi(..)));
    }
    #[test]
    fn test_snp_association_fdr_ty_builds() {
        let ty = snp_association_fdr_ty();
        assert!(matches!(ty, Expr::Pi(..)));
    }
    #[test]
    fn test_scrna_clustering_stable_ty_builds() {
        let ty = scrna_clustering_stable_ty();
        assert!(matches!(ty, Expr::Pi(..)));
    }
    #[test]
    fn test_build_bioinformatics_env_extended() {
        let mut env = Environment::new();
        build_bioinformatics_env_extended(&mut env).expect("extended build should succeed");
    }
    #[test]
    fn test_sw_aligner_identical() {
        let aligner = SmithWatermanAligner::new(2, -1, -2);
        let score = aligner.score_only("ACGT", "ACGT");
        assert!(score > 0, "identical sequences should score positively");
    }
    #[test]
    fn test_sw_aligner_disjoint() {
        let aligner = SmithWatermanAligner::new(2, -1, -2);
        let aln = aligner.align("AAAA", "TTTT");
        assert!(aln.score >= 0, "SW score is always non-negative");
    }
    #[test]
    fn test_sw_aligner_local_hit() {
        let aligner = SmithWatermanAligner::new(1, -1, -2);
        let aln = aligner.align("XXXACGTYYY", "ACGT");
        assert!(aln.score >= 4, "should find the ACGT local match");
    }
    #[test]
    fn test_debruijn_assembler_basic() {
        let assembler = DeBruijnAssembler::new(3);
        let contigs = assembler.assemble(&["ACGTAC"]);
        assert!(!contigs.is_empty(), "should produce at least one contig");
        assert!(
            contigs[0].len() >= 3,
            "contig must be at least k chars long"
        );
    }
    #[test]
    fn test_debruijn_assembler_kmer_count() {
        let assembler = DeBruijnAssembler::new(3);
        let count = assembler.count_kmers(&["ACGTA"]);
        assert_eq!(count, 3, "should count 3 distinct 3-mers in ACGTA");
    }
    #[test]
    fn test_debruijn_assembler_empty() {
        let assembler = DeBruijnAssembler::new(4);
        let contigs = assembler.assemble(&["AC"]);
        assert!(
            contigs.is_empty(),
            "reads shorter than k produce no contigs"
        );
    }
    #[test]
    fn test_nw_global_identical() {
        let nwg = NeedlemanWunschGlobal::new(1, -1, -1);
        let aln = nwg.align("ACGT", "ACGT");
        assert_eq!(aln.score, 4, "identical 4-char seqs score 4");
    }
    #[test]
    fn test_nw_global_identity_fraction() {
        let nwg = NeedlemanWunschGlobal::new(1, -1, -1);
        let id = nwg.identity("ACGT", "ACGT");
        assert!(
            (id - 1.0).abs() < 1e-9,
            "identical sequences have identity 1.0"
        );
    }
    #[test]
    fn test_nw_global_identity_partial() {
        let nwg = NeedlemanWunschGlobal::new(1, -1, -2);
        let id = nwg.identity("ACGT", "ACGG");
        assert!(id > 0.5, "3/4 matching should give identity > 0.5");
    }
    #[test]
    fn test_rna_mfe_folder_default() {
        let folder = RNAMFEFolder::new();
        let mfe = folder.mfe("AUGCAU");
        assert!(mfe <= 0.0, "MFE should be non-positive with pair_bonus=-1");
    }
    #[test]
    fn test_rna_mfe_folder_pairs() {
        let folder = RNAMFEFolder::new();
        let (mfe, pairs) = folder.fold("GGGAAACCC");
        assert!(
            pairs.is_empty() || mfe < 0.0,
            "folding result should be consistent"
        );
    }
    #[test]
    fn test_rna_mfe_folder_empty() {
        let folder = RNAMFEFolder::default();
        let (mfe, pairs) = folder.fold("");
        assert_eq!(pairs.len(), 0);
        assert_eq!(mfe, 0.0);
    }
    #[test]
    fn test_rna_mfe_folder_custom_params() {
        let folder = RNAMFEFolder::with_params(-2.0, -0.5);
        let mfe = folder.mfe("GCGCGC");
        assert!(mfe.is_finite());
    }
    #[test]
    fn test_parsimony_all_same() {
        let scorer = PhylogeneticParsimony::new();
        let leaves = ['A', 'A', 'A', 'A'];
        let score = scorer.score_leaves(&leaves);
        assert_eq!(score, 0, "all identical leaves need 0 mutations");
    }
    #[test]
    fn test_parsimony_all_different() {
        let scorer = PhylogeneticParsimony::new();
        let leaves = ['A', 'C', 'G', 'T'];
        let score = scorer.score_leaves(&leaves);
        assert!(
            score >= 2,
            "four different leaves need at least 2 mutations"
        );
    }
    #[test]
    fn test_parsimony_single_leaf() {
        let scorer = PhylogeneticParsimony::new();
        let tree = ParsimonyTree::Leaf('X');
        let (s, chars) = scorer.score(&tree);
        assert_eq!(s, 0);
        assert!(chars.contains(&'X'));
    }
    #[test]
    fn test_parsimony_build_balanced_single() {
        let tree = PhylogeneticParsimony::build_balanced(&['A']);
        assert!(matches!(tree, ParsimonyTree::Leaf('A')));
    }
    #[test]
    fn test_parsimony_default() {
        let scorer = PhylogeneticParsimony;
        let score = scorer.score_leaves(&['A', 'A']);
        assert_eq!(score, 0);
    }
}
#[cfg(test)]
mod extended_bio_tests {
    use super::*;
    #[test]
    fn test_alignment_scorer() {
        let scorer = AlignmentScorer::default_scorer();
        assert_eq!(scorer.score_pair('A', 'A'), 2);
        assert_eq!(scorer.score_pair('A', 'B'), -1);
        assert!(scorer
            .smith_waterman_description()
            .contains("Smith-Waterman"));
    }
    #[test]
    fn test_gene_expression() {
        let gems = GeneExpressionMatrix::new(
            vec!["BRCA1".to_string(), "TP53".to_string()],
            vec!["S1".to_string(), "S2".to_string()],
            vec![vec![1.0, 3.0], vec![2.0, 2.0]],
        );
        assert_eq!(gems.num_genes(), 2);
        assert_eq!(gems.num_samples(), 2);
        assert!((gems.mean_expression(0) - 2.0).abs() < 1e-9);
    }
    #[test]
    fn test_regulatory_node() {
        let mut node = RegulatoryNode::new("MYC", true);
        node.add_target("CDK4");
        node.add_target("CCND1");
        assert_eq!(node.out_degree(), 2);
        assert!(node.description().contains("TF"));
    }
    #[test]
    fn test_protein_structure() {
        let ss = vec![
            SecondaryStructure::AlphaHelix,
            SecondaryStructure::AlphaHelix,
            SecondaryStructure::BetaStrand,
        ];
        let ps = ProteinStructure::new("1ABC", "MAK", ss, 1.8);
        assert!(ps.is_high_resolution());
        assert!((ps.helix_fraction() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_secondary_structure_codes() {
        assert_eq!(SecondaryStructure::AlphaHelix.code(), 'H');
        assert_eq!(SecondaryStructure::BetaStrand.code(), 'E');
    }
    #[test]
    fn test_sequence_hmm() {
        let profile = SequenceHmm::profile("Kinase", 100);
        assert_eq!(profile.num_states, 300);
        let cpg = SequenceHmm::cpg_island_detector();
        assert_eq!(cpg.alphabet_size, 4);
        assert!(cpg.viterbi_description().contains("Viterbi"));
    }
}
