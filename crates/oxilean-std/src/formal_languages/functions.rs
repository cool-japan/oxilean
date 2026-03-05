//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};
use std::collections::{HashMap, HashSet, VecDeque};

use super::types::{
    BuchiAutomaton, Cfg, ContextFreeGrammar, CykParser, Dfa, Nfa, PdaSimulator, PushdownAutomaton,
    Regex, RegexExt, TapeDir, TmDirection, TuringMachine, TuringMachineSimulator,
};

pub fn app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
pub fn app2(f: Expr, a: Expr, b: Expr) -> Expr {
    app(app(f, a), b)
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
pub fn bvar(n: u32) -> Expr {
    Expr::BVar(n)
}
pub fn nat_ty() -> Expr {
    cst("Nat")
}
pub fn bool_ty() -> Expr {
    cst("Bool")
}
pub fn string_ty() -> Expr {
    cst("String")
}
pub fn list_nat_ty() -> Expr {
    app(cst("List"), nat_ty())
}
/// Alphabet type: a finite set of symbols (Type)
pub fn alphabet_ty() -> Expr {
    type0()
}
/// Word type: List over an alphabet (Type → Type)
pub fn word_ty() -> Expr {
    arrow(type0(), type0())
}
/// Language type: set of words over an alphabet (Type → Prop)
pub fn language_ty() -> Expr {
    arrow(type0(), prop())
}
/// DFA type: deterministic finite automaton (Type)
pub fn dfa_ty() -> Expr {
    type0()
}
/// NFA type: nondeterministic finite automaton (Type)
pub fn nfa_ty() -> Expr {
    type0()
}
/// PDA type: pushdown automaton (Type)
pub fn pda_ty() -> Expr {
    type0()
}
/// CFG type: context-free grammar (Type)
pub fn cfg_ty() -> Expr {
    type0()
}
/// TuringMachine type: a Turing machine (Type)
pub fn turing_machine_ty() -> Expr {
    type0()
}
/// RegularLanguage type: predicate recognising regular languages (Type → Prop)
pub fn regular_language_ty() -> Expr {
    arrow(type0(), prop())
}
/// ContextFreeLanguage type: predicate recognising context-free languages (Type → Prop)
pub fn context_free_language_ty() -> Expr {
    arrow(type0(), prop())
}
/// RecursivelyEnumerable type: predicate recognising RE languages (Type → Prop)
pub fn recursively_enumerable_ty() -> Expr {
    arrow(type0(), prop())
}
/// PumpingLemmaRegular: every regular language satisfies the pumping lemma (Prop)
pub fn pumping_lemma_regular_ty() -> Expr {
    prop()
}
/// PumpingLemmaCFL: every context-free language satisfies the CFL pumping lemma (Prop)
pub fn pumping_lemma_cfl_ty() -> Expr {
    prop()
}
/// MyhillNerode: a language is regular iff its Myhill-Nerode equivalence has finite index (Prop)
pub fn myhill_nerode_ty() -> Expr {
    prop()
}
/// KleeneTheorem: a language is regular iff it is recognised by some DFA (Prop)
pub fn kleene_theorem_ty() -> Expr {
    prop()
}
/// ChomskyNormalForm: every CFG can be converted to Chomsky Normal Form (Prop)
pub fn chomsky_normal_form_ty() -> Expr {
    prop()
}
/// HaltingUndecidable: the halting problem is undecidable (Prop)
pub fn halting_undecidable_ty() -> Expr {
    prop()
}
/// RiceTheorem: every non-trivial semantic property of Turing machines is undecidable (Prop)
pub fn rice_theorem_ty() -> Expr {
    prop()
}
/// CflClosedUnion: the class of context-free languages is closed under union (Prop)
pub fn cfl_closed_union_ty() -> Expr {
    prop()
}
/// RegularClosedComplement: the class of regular languages is closed under complement (Prop)
pub fn regular_closed_complement_ty() -> Expr {
    prop()
}
/// RegEx type: regular expression over an alphabet (Type)
pub fn regex_ty() -> Expr {
    type0()
}
/// LBA type: linear bounded automaton (Type)
pub fn lba_ty() -> Expr {
    type0()
}
/// BuchiAutomaton type: automaton over infinite words (Type)
pub fn buchi_automaton_ty() -> Expr {
    type0()
}
/// EpsNFA type: NFA with epsilon transitions (Type)
pub fn eps_nfa_ty() -> Expr {
    type0()
}
/// MealyMachine type: finite transducer with output on transitions (Type)
pub fn mealy_machine_ty() -> Expr {
    type0()
}
/// MooreMachine type: finite transducer with output on states (Type)
pub fn moore_machine_ty() -> Expr {
    type0()
}
/// TwoWayDFA type: DFA with two-way read-only tape head (Type)
pub fn two_way_dfa_ty() -> Expr {
    type0()
}
/// CountingAutomaton type: automaton extended with counters (Type)
pub fn counting_automaton_ty() -> Expr {
    type0()
}
/// WeightedAutomaton type: automaton with weights in a semiring (Type)
pub fn weighted_automaton_ty() -> Expr {
    type0()
}
/// OmegaLanguage type: set of infinite words over an alphabet (Type → Prop)
pub fn omega_language_ty() -> Expr {
    arrow(type0(), prop())
}
/// SubsetConstruction: every NFA has an equivalent DFA via subset construction (Prop)
pub fn subset_construction_ty() -> Expr {
    prop()
}
/// GreibachNormalForm: every CFG can be converted to Greibach Normal Form (Prop)
pub fn greibach_normal_form_ty() -> Expr {
    prop()
}
/// CflPdaEquivalence: a language is context-free iff it is accepted by some NPDA (Prop)
pub fn cfl_pda_equivalence_ty() -> Expr {
    prop()
}
/// LbaAcceptsCsl: a language is context-sensitive iff it is accepted by some LBA (Prop)
pub fn lba_accepts_csl_ty() -> Expr {
    prop()
}
/// PostCorrespondenceProblemUndecidable: the Post Correspondence Problem is undecidable (Prop)
pub fn post_correspondence_undecidable_ty() -> Expr {
    prop()
}
/// TimeHierarchyTheorem: DTIME(f(n)) ⊊ DTIME(g(n)) when g grows faster (Prop)
pub fn time_hierarchy_theorem_ty() -> Expr {
    prop()
}
/// SpaceHierarchyTheorem: DSPACE(f(n)) ⊊ DSPACE(g(n)) when g grows faster (Prop)
pub fn space_hierarchy_theorem_ty() -> Expr {
    prop()
}
/// UniversalTuringMachineExists: there exists a universal Turing machine (Prop)
pub fn universal_turing_machine_ty() -> Expr {
    prop()
}
/// ChurchTuringThesis: every effectively computable function is Turing-computable (Prop)
pub fn church_turing_thesis_ty() -> Expr {
    prop()
}
/// EarleyParsingCorrect: Earley's algorithm correctly parses all CFLs in O(n³) (Prop)
pub fn earley_parsing_correct_ty() -> Expr {
    prop()
}
/// CykParsingCorrect: the CYK algorithm decides CFL membership in O(n³|G|) (Prop)
pub fn cyk_parsing_correct_ty() -> Expr {
    prop()
}
/// RegularClosed: regular languages are closed under Boolean operations (Prop)
pub fn regular_closed_boolean_ty() -> Expr {
    prop()
}
/// BuchiComplementable: Buchi automata are closed under complementation (Prop)
pub fn buchi_complementable_ty() -> Expr {
    prop()
}
/// BuchiClosedUnion: Buchi automata are closed under union (Prop)
pub fn buchi_closed_union_ty() -> Expr {
    prop()
}
/// BuchiClosedIntersection: Buchi automata are closed under intersection (Prop)
pub fn buchi_closed_intersection_ty() -> Expr {
    prop()
}
/// OmegaRegularEqualsBuchi: a language is omega-regular iff it is accepted by some Buchi automaton (Prop)
pub fn omega_regular_equals_buchi_ty() -> Expr {
    prop()
}
/// MyhillNerodeOmega: Myhill-Nerode theorem for omega-regular languages (Prop)
pub fn myhill_nerode_omega_ty() -> Expr {
    prop()
}
/// PumpingLemmaOmega: every omega-regular language satisfies an omega pumping lemma (Prop)
pub fn pumping_lemma_omega_ty() -> Expr {
    prop()
}
/// RegExToDfaCorrect: every regular expression is equivalent to a DFA (Kleene, constructive) (Prop)
pub fn regex_to_dfa_correct_ty() -> Expr {
    prop()
}
/// DfaToRegExCorrect: every DFA has an equivalent regular expression (state elimination) (Prop)
pub fn dfa_to_regex_correct_ty() -> Expr {
    prop()
}
/// GlushkovConstruction: the Glushkov automaton for a regex accepts the same language (Prop)
pub fn glushkov_construction_ty() -> Expr {
    prop()
}
/// EmptyWordProblemDecidable: deciding if a grammar generates the empty string is decidable (Prop)
pub fn empty_word_problem_decidable_ty() -> Expr {
    prop()
}
/// AmbiguityUndecidable: deciding whether a CFG is ambiguous is undecidable (Prop)
pub fn ambiguity_undecidable_ty() -> Expr {
    prop()
}
/// IntersectionCflReg: the intersection of a CFL and a regular language is context-free (Prop)
pub fn intersection_cfl_reg_ty() -> Expr {
    prop()
}
/// RegularNotClosedArbitrary: there exist languages not expressible by any DFA (Prop)
pub fn regular_not_closed_arbitrary_ty() -> Expr {
    prop()
}
/// MealyMooreEquivalent: every Mealy machine has an equivalent Moore machine (Prop)
pub fn mealy_moore_equivalent_ty() -> Expr {
    prop()
}
/// TwoWayDfaEqualsDfa: two-way DFAs recognise exactly the regular languages (Prop)
pub fn two_way_dfa_equals_dfa_ty() -> Expr {
    prop()
}
/// ComplexityPvsNP: the question whether P = NP (represented as Prop placeholder) (Prop)
pub fn p_vs_np_ty() -> Expr {
    prop()
}
/// Register all formal-language axioms and theorems into `env`.
pub fn build_formal_languages_env(env: &mut Environment) -> Result<(), String> {
    for (name, ty) in [
        ("Alphabet", alphabet_ty()),
        ("Word", word_ty()),
        ("FormalLanguage", language_ty()),
        ("DFA", dfa_ty()),
        ("NFA", nfa_ty()),
        ("PDA", pda_ty()),
        ("CFG", cfg_ty()),
        ("TuringMachine", turing_machine_ty()),
        ("RegularLanguage", regular_language_ty()),
        ("ContextFreeLanguage", context_free_language_ty()),
        ("RecursivelyEnumerable", recursively_enumerable_ty()),
        ("RegEx", regex_ty()),
        ("LBA", lba_ty()),
        ("BuchiAutomaton", buchi_automaton_ty()),
        ("EpsNFA", eps_nfa_ty()),
        ("MealyMachine", mealy_machine_ty()),
        ("MooreMachine", moore_machine_ty()),
        ("TwoWayDFA", two_way_dfa_ty()),
        ("CountingAutomaton", counting_automaton_ty()),
        ("WeightedAutomaton", weighted_automaton_ty()),
        ("OmegaLanguage", omega_language_ty()),
    ] {
        env.add(Declaration::Axiom {
            name: Name::str(name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    for (name, ty) in [
        ("DFAAccepts", arrow(dfa_ty(), arrow(string_ty(), prop()))),
        ("NFAAccepts", arrow(nfa_ty(), arrow(string_ty(), prop()))),
        ("PDAAccepts", arrow(pda_ty(), arrow(string_ty(), prop()))),
        ("CFGDerives", arrow(cfg_ty(), arrow(string_ty(), prop()))),
        (
            "TMAccepts",
            arrow(turing_machine_ty(), arrow(string_ty(), prop())),
        ),
        ("LBAAccepts", arrow(lba_ty(), arrow(string_ty(), prop()))),
        (
            "BuchiAccepts",
            arrow(
                buchi_automaton_ty(),
                arrow(arrow(nat_ty(), nat_ty()), prop()),
            ),
        ),
        (
            "RegExMatches",
            arrow(regex_ty(), arrow(string_ty(), prop())),
        ),
        ("IsRegular", arrow(arrow(string_ty(), prop()), prop())),
        ("IsContextFree", arrow(arrow(string_ty(), prop()), prop())),
        ("IsDecidable", arrow(arrow(string_ty(), prop()), prop())),
        ("IsSemiDecidable", arrow(arrow(string_ty(), prop()), prop())),
        (
            "IsContextSensitive",
            arrow(arrow(string_ty(), prop()), prop()),
        ),
        (
            "IsOmegaRegular",
            arrow(arrow(nat_ty(), arrow(nat_ty(), prop())), prop()),
        ),
    ] {
        env.add(Declaration::Axiom {
            name: Name::str(name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    for (name, ty) in [
        ("dfaMinimize", arrow(dfa_ty(), dfa_ty())),
        ("nfaToDfa", arrow(nfa_ty(), dfa_ty())),
        ("cfgToCnf", arrow(cfg_ty(), cfg_ty())),
        ("cfgToGnf", arrow(cfg_ty(), cfg_ty())),
        ("regexToNfa", arrow(regex_ty(), nfa_ty())),
        ("dfaToRegex", arrow(dfa_ty(), regex_ty())),
        ("epsNfaToNfa", arrow(eps_nfa_ty(), nfa_ty())),
        (
            "mealyToMoore",
            arrow(mealy_machine_ty(), moore_machine_ty()),
        ),
        (
            "mooreToMealy",
            arrow(moore_machine_ty(), mealy_machine_ty()),
        ),
        ("dfaProduct", arrow(dfa_ty(), arrow(dfa_ty(), dfa_ty()))),
        ("dfaComplement", arrow(dfa_ty(), dfa_ty())),
        ("cfgToPda", arrow(cfg_ty(), pda_ty())),
        ("pdaToCfg", arrow(pda_ty(), cfg_ty())),
    ] {
        env.add(Declaration::Axiom {
            name: Name::str(name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    for (name, ty) in [
        ("PumpingLemmaRegular", pumping_lemma_regular_ty()),
        ("PumpingLemmaCFL", pumping_lemma_cfl_ty()),
        ("MyhillNerode", myhill_nerode_ty()),
        ("KleeneTheorem", kleene_theorem_ty()),
        ("ChomskyNormalForm", chomsky_normal_form_ty()),
        ("HaltingUndecidable", halting_undecidable_ty()),
        ("RiceTheorem", rice_theorem_ty()),
        ("CflClosedUnion", cfl_closed_union_ty()),
        ("RegularClosedComplement", regular_closed_complement_ty()),
        ("RegularClosedIntersection", prop()),
        ("RegularClosedUnion", prop()),
        ("RegularClosedConcatenation", prop()),
        ("RegularClosedKleeneStar", prop()),
        ("CflClosedConcatenation", prop()),
        ("CflClosedKleeneStar", prop()),
        ("CflNotClosedIntersection", prop()),
        ("CflNotClosedComplement", prop()),
        ("RegularSubsetCFL", prop()),
        ("CFLSubsetCSL", prop()),
        ("CSLSubsetRE", prop()),
        ("EmptinessRegularDecidable", prop()),
        ("MembershipRegularDecidable", prop()),
        ("MembershipCFLDecidable", prop()),
        ("EmptinessCFLDecidable", prop()),
        ("NfaDfaEquivalence", prop()),
        ("DfaMinimizationCorrect", prop()),
        ("BarHillellLemma", prop()),
    ] {
        env.add(Declaration::Axiom {
            name: Name::str(name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    for (name, ty) in [
        ("SubsetConstruction", subset_construction_ty()),
        ("GreibachNormalForm", greibach_normal_form_ty()),
        ("CflPdaEquivalence", cfl_pda_equivalence_ty()),
        ("LbaAcceptsCsl", lba_accepts_csl_ty()),
        (
            "PostCorrespondenceUndecidable",
            post_correspondence_undecidable_ty(),
        ),
        ("TimeHierarchyTheorem", time_hierarchy_theorem_ty()),
        ("SpaceHierarchyTheorem", space_hierarchy_theorem_ty()),
        (
            "UniversalTuringMachineExists",
            universal_turing_machine_ty(),
        ),
        ("ChurchTuringThesis", church_turing_thesis_ty()),
        ("EarleyParsingCorrect", earley_parsing_correct_ty()),
        ("CykParsingCorrect", cyk_parsing_correct_ty()),
        ("RegularClosedBoolean", regular_closed_boolean_ty()),
        ("BuchiComplementable", buchi_complementable_ty()),
        ("BuchiClosedUnion", buchi_closed_union_ty()),
        ("BuchiClosedIntersection", buchi_closed_intersection_ty()),
        ("OmegaRegularEqualsBuchi", omega_regular_equals_buchi_ty()),
        ("MyhillNerodeOmega", myhill_nerode_omega_ty()),
        ("PumpingLemmaOmega", pumping_lemma_omega_ty()),
        ("RegExToDfaCorrect", regex_to_dfa_correct_ty()),
        ("DfaToRegExCorrect", dfa_to_regex_correct_ty()),
        ("GlushkovConstruction", glushkov_construction_ty()),
        (
            "EmptyWordProblemDecidable",
            empty_word_problem_decidable_ty(),
        ),
        ("AmbiguityUndecidable", ambiguity_undecidable_ty()),
        ("IntersectionCflReg", intersection_cfl_reg_ty()),
        (
            "RegularNotClosedArbitrary",
            regular_not_closed_arbitrary_ty(),
        ),
        ("MealyMooreEquivalent", mealy_moore_equivalent_ty()),
        ("TwoWayDfaEqualsDfa", two_way_dfa_equals_dfa_ty()),
        ("PvsNP", p_vs_np_ty()),
    ] {
        env.add(Declaration::Axiom {
            name: Name::str(name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    Ok(())
}
/// Minimize a DFA using the table-filling (Hopcroft's) algorithm.
///
/// Returns a new DFA that accepts the same language with the minimum number
/// of states (unreachable states are removed first).
pub fn dfa_minimize(dfa: &Dfa) -> Dfa {
    let n = dfa.states;
    let sigma = dfa.alphabet.len();
    let mut reachable = vec![false; n];
    let mut queue: VecDeque<usize> = VecDeque::new();
    reachable[dfa.start] = true;
    queue.push_back(dfa.start);
    while let Some(q) = queue.pop_front() {
        for c in 0..sigma {
            let r = dfa.delta[q][c];
            if !reachable[r] {
                reachable[r] = true;
                queue.push_back(r);
            }
        }
    }
    let reach_states: Vec<usize> = (0..n).filter(|&q| reachable[q]).collect();
    let reach_index: HashMap<usize, usize> = reach_states
        .iter()
        .enumerate()
        .map(|(i, &q)| (q, i))
        .collect();
    let m = reach_states.len();
    let mut dist = vec![vec![false; m]; m];
    for i in 0..m {
        for j in (i + 1)..m {
            if dfa.accept[reach_states[i]] != dfa.accept[reach_states[j]] {
                dist[i][j] = true;
                dist[j][i] = true;
            }
        }
    }
    let mut changed = true;
    while changed {
        changed = false;
        for i in 0..m {
            for j in (i + 1)..m {
                if dist[i][j] {
                    continue;
                }
                'outer: for c in 0..sigma {
                    let ri = reach_index[&dfa.delta[reach_states[i]][c]];
                    let rj = reach_index[&dfa.delta[reach_states[j]][c]];
                    if ri != rj && (dist[ri][rj] || dist[rj][ri]) {
                        dist[i][j] = true;
                        dist[j][i] = true;
                        changed = true;
                        break 'outer;
                    }
                }
            }
        }
    }
    let mut class_of = vec![usize::MAX; m];
    let mut num_classes = 0usize;
    for i in 0..m {
        if class_of[i] == usize::MAX {
            class_of[i] = num_classes;
            for j in (i + 1)..m {
                if !dist[i][j] {
                    class_of[j] = num_classes;
                }
            }
            num_classes += 1;
        }
    }
    let start_class = class_of[reach_index[&dfa.start]];
    let mut new_delta = vec![vec![0usize; sigma]; num_classes];
    let mut new_accept = vec![false; num_classes];
    for i in 0..m {
        let ci = class_of[i];
        new_accept[ci] = dfa.accept[reach_states[i]];
        for c in 0..sigma {
            let ri = reach_index[&dfa.delta[reach_states[i]][c]];
            new_delta[ci][c] = class_of[ri];
        }
    }
    Dfa {
        states: num_classes,
        alphabet: dfa.alphabet.clone(),
        delta: new_delta,
        start: start_class,
        accept: new_accept,
    }
}
/// Convert an NFA to an equivalent DFA using the subset construction.
///
/// The resulting DFA has states corresponding to non-empty subsets of NFA
/// states that are reachable from the start state.
pub fn nfa_to_dfa(nfa: &Nfa) -> Dfa {
    let sigma = nfa.alphabet.len();
    let mut subset_to_id: HashMap<Vec<usize>, usize> = HashMap::new();
    let mut id_to_subset: Vec<Vec<usize>> = Vec::new();
    let mut queue: VecDeque<usize> = VecDeque::new();
    let start_set: Vec<usize> = vec![nfa.start];
    subset_to_id.insert(start_set.clone(), 0);
    id_to_subset.push(start_set);
    queue.push_back(0);
    let mut dfa_delta: Vec<Vec<usize>> = Vec::new();
    while let Some(id) = queue.pop_front() {
        let mut row = vec![0usize; sigma];
        let subset = id_to_subset[id].clone();
        for c in 0..sigma {
            let mut next_set: HashSet<usize> = HashSet::new();
            for &q in &subset {
                for &r in &nfa.delta[q][c] {
                    next_set.insert(r);
                }
            }
            let mut next_vec: Vec<usize> = next_set.into_iter().collect();
            next_vec.sort_unstable();
            if next_vec.is_empty() {
                row[c] = usize::MAX;
            } else {
                let next_id = if let Some(&eid) = subset_to_id.get(&next_vec) {
                    eid
                } else {
                    let eid = id_to_subset.len();
                    subset_to_id.insert(next_vec.clone(), eid);
                    id_to_subset.push(next_vec);
                    queue.push_back(eid);
                    eid
                };
                row[c] = next_id;
            }
        }
        dfa_delta.push(row);
    }
    let has_dead = dfa_delta.iter().any(|row| row.contains(&usize::MAX));
    let dead_id = id_to_subset.len();
    if has_dead {
        for row in &mut dfa_delta {
            for s in row.iter_mut() {
                if *s == usize::MAX {
                    *s = dead_id;
                }
            }
        }
        dfa_delta.push(vec![dead_id; sigma]);
        id_to_subset.push(vec![]);
    }
    let total = id_to_subset.len();
    let accept: Vec<bool> = (0..total)
        .map(|id| {
            if id == dead_id && has_dead {
                false
            } else {
                id_to_subset[id].iter().any(|&q| nfa.accept[q])
            }
        })
        .collect();
    Dfa {
        states: total,
        alphabet: nfa.alphabet.clone(),
        delta: dfa_delta,
        start: 0,
        accept,
    }
}
/// Check whether a pumping-lemma decomposition exists for `word` with
/// pumping length `pump_len`.
///
/// For a word `w` with `|w| ≥ pump_len`, this checks that there exists a
/// split `w = xyz` with `|xy| ≤ pump_len`, `|y| ≥ 1`.
/// Returns `true` iff such a split exists (i.e. the word is long enough and
/// the pump length is positive).
pub fn check_pumping_lemma_regular(word: &[u8], pump_len: usize) -> bool {
    if pump_len == 0 {
        return false;
    }
    if word.len() < pump_len {
        return false;
    }
    true
}
/// Compile a [`Regex`] to an equivalent [`Dfa`] using Thompson's construction
/// (builds an ε-NFA) followed by the subset construction.
///
/// The resulting DFA is over the alphabet implied by the literals in the regex.
pub fn regex_to_dfa(regex: &Regex) -> Dfa {
    fn collect_chars(r: &Regex, out: &mut Vec<char>) {
        match r {
            Regex::Empty | Regex::Epsilon => {}
            Regex::Char(c) => {
                if !out.contains(c) {
                    out.push(*c);
                }
            }
            Regex::Alt(a, b) | Regex::Concat(a, b) => {
                collect_chars(a, out);
                collect_chars(b, out);
            }
            Regex::Star(a) => collect_chars(a, out),
        }
    }
    let mut alphabet = Vec::new();
    collect_chars(regex, &mut alphabet);
    alphabet.sort_unstable();
    if alphabet.is_empty() {
        let accepts_eps = regex.matches("");
        return Dfa {
            states: 1,
            alphabet: vec![],
            delta: vec![vec![]],
            start: 0,
            accept: vec![accepts_eps],
        };
    }
    const MAX_STATES: usize = 256;
    let sigma = alphabet.len();
    struct ThompsonNfa {
        transitions: Vec<Vec<Vec<usize>>>,
        eps: Vec<Vec<usize>>,
    }
    impl ThompsonNfa {
        fn new_state(&mut self) -> usize {
            let id = self.transitions.len();
            self.transitions.push(vec![]);
            self.eps.push(vec![]);
            id
        }
    }
    fn build_thompson(r: &Regex, alphabet: &[char], nfa: &mut ThompsonNfa) -> (usize, usize) {
        match r {
            Regex::Empty => {
                let s = nfa.new_state();
                let a = nfa.new_state();
                nfa.transitions[s].resize(alphabet.len(), vec![]);
                nfa.transitions[a].resize(alphabet.len(), vec![]);
                (s, a)
            }
            Regex::Epsilon => {
                let s = nfa.new_state();
                nfa.transitions[s].resize(alphabet.len(), vec![]);
                (s, s)
            }
            Regex::Char(c) => {
                let s = nfa.new_state();
                let a = nfa.new_state();
                nfa.transitions[s].resize(alphabet.len(), vec![]);
                nfa.transitions[a].resize(alphabet.len(), vec![]);
                if let Some(idx) = alphabet.iter().position(|&x| x == *c) {
                    nfa.transitions[s][idx].push(a);
                }
                (s, a)
            }
            Regex::Alt(r1, r2) => {
                let (s1, a1) = build_thompson(r1, alphabet, nfa);
                let (s2, a2) = build_thompson(r2, alphabet, nfa);
                let s = nfa.new_state();
                let a = nfa.new_state();
                nfa.transitions[s].resize(alphabet.len(), vec![]);
                nfa.transitions[a].resize(alphabet.len(), vec![]);
                nfa.eps[s].push(s1);
                nfa.eps[s].push(s2);
                nfa.eps[a1].push(a);
                nfa.eps[a2].push(a);
                (s, a)
            }
            Regex::Concat(r1, r2) => {
                let (s1, a1) = build_thompson(r1, alphabet, nfa);
                let (s2, a2) = build_thompson(r2, alphabet, nfa);
                nfa.eps[a1].push(s2);
                (s1, a2)
            }
            Regex::Star(r1) => {
                let (s1, a1) = build_thompson(r1, alphabet, nfa);
                let s = nfa.new_state();
                let a = nfa.new_state();
                nfa.transitions[s].resize(alphabet.len(), vec![]);
                nfa.transitions[a].resize(alphabet.len(), vec![]);
                nfa.eps[s].push(s1);
                nfa.eps[s].push(a);
                nfa.eps[a1].push(s1);
                nfa.eps[a1].push(a);
                (s, a)
            }
        }
    }
    let mut tnfa = ThompsonNfa {
        transitions: vec![],
        eps: vec![],
    };
    let (t_start, t_accept) = build_thompson(regex, &alphabet, &mut tnfa);
    let n_tnfa = tnfa.transitions.len();
    let eps_closure = |states: &HashSet<usize>| -> HashSet<usize> {
        let mut closure = states.clone();
        let mut stack: Vec<usize> = states.iter().copied().collect();
        while let Some(q) = stack.pop() {
            if q < tnfa.eps.len() {
                for &r in &tnfa.eps[q] {
                    if closure.insert(r) {
                        stack.push(r);
                    }
                }
            }
        }
        closure
    };
    let start_set = eps_closure(&{
        let mut s = HashSet::new();
        s.insert(t_start);
        s
    });
    let mut subset_map: HashMap<Vec<usize>, usize> = HashMap::new();
    let mut id_to_set: Vec<Vec<usize>> = Vec::new();
    let mut dfa_delta_build: Vec<Vec<usize>> = Vec::new();
    let mut bfs: VecDeque<usize> = VecDeque::new();
    let mut start_sorted: Vec<usize> = start_set.into_iter().collect();
    start_sorted.sort_unstable();
    subset_map.insert(start_sorted.clone(), 0);
    id_to_set.push(start_sorted);
    bfs.push_back(0);
    while let Some(id) = bfs.pop_front() {
        if dfa_delta_build.len() > MAX_STATES {
            break;
        }
        let mut row = vec![0usize; sigma];
        let set: HashSet<usize> = id_to_set[id].iter().copied().collect();
        for c in 0..sigma {
            let mut next_set: HashSet<usize> = HashSet::new();
            for &q in &set {
                if q < n_tnfa && c < tnfa.transitions[q].len() {
                    for &r in &tnfa.transitions[q][c] {
                        next_set.insert(r);
                    }
                }
            }
            let next_closed = eps_closure(&next_set);
            let mut next_sorted: Vec<usize> = next_closed.into_iter().collect();
            next_sorted.sort_unstable();
            let next_id = if let Some(&eid) = subset_map.get(&next_sorted) {
                eid
            } else {
                let eid = id_to_set.len();
                subset_map.insert(next_sorted.clone(), eid);
                id_to_set.push(next_sorted);
                bfs.push_back(eid);
                eid
            };
            row[c] = next_id;
        }
        dfa_delta_build.push(row);
    }
    let total = id_to_set.len();
    let accept_states: Vec<bool> = (0..total)
        .map(|id| id_to_set[id].contains(&t_accept))
        .collect();
    Dfa {
        states: total,
        alphabet,
        delta: dfa_delta_build,
        start: 0,
        accept: accept_states,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    /// DFA accepting strings over {0,1} with an even number of 0s.
    /// States: 0 = even (start, accept), 1 = odd.
    #[test]
    fn test_dfa_even_zeros() {
        let alphabet = vec!['0', '1'];
        let delta = vec![vec![1, 0], vec![0, 1]];
        let dfa = Dfa {
            states: 2,
            alphabet,
            delta,
            start: 0,
            accept: vec![true, false],
        };
        assert!(dfa.accepts(""));
        assert!(dfa.accepts("1111"));
        assert!(dfa.accepts("00"));
        assert!(dfa.accepts("010"));
        assert!(!dfa.accepts("0"));
        assert!(dfa.accepts("001"));
        assert!(!dfa.accepts("0111"));
    }
    /// NFA accepting strings over {a,b} that end in "ab".
    /// States: 0 (start), 1 (saw 'a'), 2 (saw 'ab', accept).
    #[test]
    fn test_nfa_accepts() {
        let alphabet = vec!['a', 'b'];
        let delta = vec![
            vec![vec![0, 1], vec![0]],
            vec![vec![], vec![2]],
            vec![vec![], vec![]],
        ];
        let nfa = Nfa {
            states: 3,
            alphabet,
            delta,
            start: 0,
            accept: vec![false, false, true],
        };
        assert!(nfa.accepts("ab"));
        assert!(nfa.accepts("aab"));
        assert!(nfa.accepts("bab"));
        assert!(nfa.accepts("ababab"));
        assert!(!nfa.accepts("a"));
        assert!(!nfa.accepts("ba"));
        assert!(!nfa.accepts("b"));
        assert!(!nfa.accepts(""));
    }
    /// Basic regex matching tests.
    #[test]
    fn test_regex_matches() {
        let r = Regex::Concat(
            Box::new(Regex::Char('a')),
            Box::new(Regex::Star(Box::new(Regex::Alt(
                Box::new(Regex::Char('b')),
                Box::new(Regex::Char('c')),
            )))),
        );
        assert!(r.matches("a"));
        assert!(r.matches("ab"));
        assert!(r.matches("ac"));
        assert!(r.matches("abbc"));
        assert!(r.matches("abcbc"));
        assert!(!r.matches(""));
        assert!(!r.matches("b"));
        assert!(!r.matches("ba"));
        let eps = Regex::Epsilon;
        assert!(eps.matches(""));
        assert!(!eps.matches("a"));
        let empty = Regex::Empty;
        assert!(!empty.matches(""));
        assert!(!empty.matches("a"));
    }
    /// Convert NFA to DFA and verify equivalent acceptance.
    #[test]
    fn test_nfa_to_dfa() {
        let alphabet = vec!['a', 'b'];
        let delta = vec![
            vec![vec![0, 1], vec![0]],
            vec![vec![], vec![2]],
            vec![vec![], vec![]],
        ];
        let nfa = Nfa {
            states: 3,
            alphabet,
            delta,
            start: 0,
            accept: vec![false, false, true],
        };
        let dfa = nfa_to_dfa(&nfa);
        assert!(dfa.accepts("ab"));
        assert!(dfa.accepts("aab"));
        assert!(dfa.accepts("bab"));
        assert!(!dfa.accepts("a"));
        assert!(!dfa.accepts("ba"));
        assert!(!dfa.accepts("b"));
        assert!(!dfa.accepts(""));
    }
    /// Test CNF detection for a context-free grammar.
    #[test]
    fn test_cfg_cnf() {
        let cnf = Cfg {
            variables: vec!["S".into(), "A".into(), "B".into()],
            terminals: vec!['a', 'b'],
            rules: vec![
                ("S".into(), vec!["A".into(), "B".into()]),
                ("A".into(), vec!["a".into()]),
                ("B".into(), vec!["b".into()]),
            ],
            start: "S".into(),
        };
        assert!(cnf.is_in_cnf());
        let not_cnf = Cfg {
            variables: vec!["S".into(), "A".into(), "B".into(), "C".into()],
            terminals: vec!['a', 'b', 'c'],
            rules: vec![
                ("S".into(), vec!["A".into(), "B".into(), "C".into()]),
                ("A".into(), vec!["a".into()]),
                ("B".into(), vec!["b".into()]),
                ("C".into(), vec!["c".into()]),
            ],
            start: "S".into(),
        };
        assert!(!not_cnf.is_in_cnf());
    }
    /// Test that build_formal_languages_env succeeds.
    #[test]
    fn test_build_env() {
        let mut env = Environment::new();
        let result = build_formal_languages_env(&mut env);
        assert!(result.is_ok());
    }
    /// CYK parser: grammar for {aⁿbⁿ | n ≥ 1} in CNF.
    /// S → AB | AC, C → SB, A → a, B → b.
    #[test]
    fn test_cyk_parser() {
        let cyk = CykParser {
            start: "S".into(),
            binary: vec![
                ("S".into(), "A".into(), "B".into()),
                ("S".into(), "A".into(), "C".into()),
                ("C".into(), "S".into(), "B".into()),
            ],
            unit: vec![("A".to_string(), 'a'), ("B".to_string(), 'b')],
            derives_empty: false,
        };
        assert!(cyk.accepts("ab"));
        assert!(cyk.accepts("aabb"));
        assert!(cyk.accepts("aaabbb"));
        assert!(!cyk.accepts("a"));
        assert!(!cyk.accepts("b"));
        assert!(!cyk.accepts("aab"));
        assert!(!cyk.accepts("abb"));
        assert!(!cyk.accepts(""));
    }
    /// PDA simulator: accept {a^n b^n | n ≥ 1} by final state.
    /// States: 0=start, 1=reading_a, 2=reading_b, 3=accept.
    /// Stack alphabet: 0=Z (bottom), 1=A.
    #[test]
    fn test_pda_simulator() {
        let transitions = vec![
            (0, Some(0usize), None, 1, vec![1usize]),
            (1, Some(0usize), None, 1, vec![1usize]),
            (1, Some(1usize), Some(1usize), 2, vec![]),
            (2, Some(1usize), Some(1usize), 2, vec![]),
            (2, None, Some(0usize), 3, vec![]),
        ];
        let pda = PdaSimulator {
            states: 4,
            alphabet: vec!['a', 'b'],
            stack_alphabet: vec!["Z".into(), "A".into()],
            transitions,
            start: 0,
            initial_stack: 0,
            accept: vec![false, false, false, true],
        };
        assert!(pda.accepts("ab"));
        assert!(pda.accepts("aabb"));
        assert!(!pda.accepts("a"));
        assert!(!pda.accepts("b"));
        assert!(!pda.accepts("aba"));
    }
    /// Turing machine: copy unary (a^n → a^n a^n) — acceptance test only.
    /// Simple TM that accepts strings of the form "a^n".
    #[test]
    fn test_turing_machine_accepts() {
        let mut delta: HashMap<(usize, usize), (usize, usize, TmDirection)> = HashMap::new();
        delta.insert((0, 1), (0, 1, TmDirection::Right));
        delta.insert((0, 0), (1, 0, TmDirection::Stay));
        let mut accept_set = HashSet::new();
        accept_set.insert(1usize);
        let mut reject_set = HashSet::new();
        reject_set.insert(2usize);
        let tm = TuringMachineSimulator {
            states: 3,
            tape_alphabet: vec!["_".into(), "a".into()],
            delta,
            start: 0,
            accept: accept_set,
            reject: reject_set,
            step_limit: 1000,
        };
        assert!(tm.run(""));
        assert!(tm.run("a"));
        assert!(tm.run("aaa"));
        assert!(tm.run("aaaaaaa"));
    }
    /// Büchi automaton: accepts (a+b)*a^ω (eventually only a's).
    /// States: 0=start, 1=seen_a (accept).
    /// Transitions: 0 --a--> {0,1}, 0 --b--> {0}, 1 --a--> {1}, 1 --b--> {0}.
    #[test]
    fn test_buchi_accepts_lasso() {
        let buchi = BuchiAutomaton {
            states: 2,
            alphabet: vec!['a', 'b'],
            delta: vec![vec![vec![0, 1], vec![0]], vec![vec![1], vec![0]]],
            start: 0,
            accept: vec![false, true],
        };
        assert!(buchi.accepts_lasso("ba", "a"));
        assert!(buchi.accepts_lasso("", "a"));
        assert!(!buchi.accepts_lasso("a", "b"));
        assert!(!buchi.accepts_lasso("", "b"));
    }
    /// Regex-to-DFA compilation test.
    #[test]
    fn test_regex_to_dfa() {
        let r = Regex::Concat(
            Box::new(Regex::Star(Box::new(Regex::Alt(
                Box::new(Regex::Char('a')),
                Box::new(Regex::Char('b')),
            )))),
            Box::new(Regex::Char('c')),
        );
        let dfa = regex_to_dfa(&r);
        assert!(dfa.accepts("c"));
        assert!(dfa.accepts("ac"));
        assert!(dfa.accepts("bc"));
        assert!(dfa.accepts("abc"));
        assert!(dfa.accepts("bac"));
        assert!(!dfa.accepts("a"));
        assert!(!dfa.accepts(""));
        assert!(!dfa.accepts("ca"));
    }
    /// Test new theorem type builders return correct sort.
    #[test]
    fn test_new_theorem_types() {
        let prop_expr = prop();
        assert_eq!(subset_construction_ty(), prop_expr);
        assert_eq!(greibach_normal_form_ty(), prop_expr);
        assert_eq!(cfl_pda_equivalence_ty(), prop_expr);
        assert_eq!(lba_accepts_csl_ty(), prop_expr);
        assert_eq!(post_correspondence_undecidable_ty(), prop_expr);
        assert_eq!(time_hierarchy_theorem_ty(), prop_expr);
        assert_eq!(space_hierarchy_theorem_ty(), prop_expr);
        assert_eq!(universal_turing_machine_ty(), prop_expr);
        assert_eq!(church_turing_thesis_ty(), prop_expr);
        assert_eq!(buchi_complementable_ty(), prop_expr);
        assert_eq!(omega_regular_equals_buchi_ty(), prop_expr);
        assert_eq!(regex_to_dfa_correct_ty(), prop_expr);
        assert_eq!(ambiguity_undecidable_ty(), prop_expr);
        assert_eq!(intersection_cfl_reg_ty(), prop_expr);
        assert_eq!(mealy_moore_equivalent_ty(), prop_expr);
        assert_eq!(two_way_dfa_equals_dfa_ty(), prop_expr);
    }
}
#[cfg(test)]
mod tests_formal_languages_ext {
    use super::*;
    #[test]
    fn test_cfg() {
        let mut g = ContextFreeGrammar::new("S");
        g.add_terminal("a");
        g.add_terminal("b");
        g.add_nonterminal("A");
        g.add_production("S", vec!["a", "S", "b"]);
        g.add_production("S", vec![]);
        assert_eq!(g.num_productions(), 2);
        assert!(g.generates_empty());
        assert!(!g.is_cnf());
    }
    #[test]
    fn test_pda_deterministic() {
        let mut pda = PushdownAutomaton::new("q0", 'Z');
        pda.add_accept_state("q1");
        pda.add_transition("q0", Some('a'), 'Z', "q0", vec!['A', 'Z']);
        pda.add_transition("q0", Some('b'), 'A', "q1", vec![]);
        assert!(pda.is_deterministic());
        assert_eq!(pda.num_states(), 2);
    }
    #[test]
    fn test_regex_accepts() {
        let r = RegexExt::star(RegexExt::ch('a'));
        assert!(r.accepts(""));
        assert!(r.accepts("a"));
        assert!(r.accepts("aaa"));
        let r2 = RegexExt::concat(RegexExt::ch('a'), RegexExt::ch('b'));
        assert!(r2.accepts("ab"));
        assert!(!r2.accepts("a"));
        assert!(!r2.accepts("b"));
    }
    #[test]
    fn test_regex_union() {
        let r = RegexExt::union(RegexExt::ch('a'), RegexExt::ch('b'));
        assert!(r.accepts("a"));
        assert!(r.accepts("b"));
        assert!(!r.accepts("ab"));
        assert!(!r.accepts(""));
    }
}
#[cfg(test)]
mod tests_formal_languages_ext2 {
    use super::*;
    #[test]
    fn test_turing_machine_accept() {
        let mut tm = TuringMachine::new("q0", "qa", "qr", '_');
        tm.add_transition("q0", '_', "qa", '_', TapeDir::Stay);
        let result = tm.halts_on_empty(10);
        assert_eq!(result, Some(true));
        assert_eq!(tm.num_transitions(), 1);
    }
    #[test]
    fn test_turing_machine_reject() {
        let mut tm = TuringMachine::new("q0", "qa", "qr", '_');
        tm.add_transition("q0", '_', "qr", '_', TapeDir::Stay);
        let result = tm.halts_on_empty(10);
        assert_eq!(result, Some(false));
    }
    #[test]
    fn test_cfg_pumping_bound() {
        let mut g = ContextFreeGrammar::new("S");
        for _i in 0..5 {
            g.add_production("S", vec!["a"]);
        }
        assert!(g.pumping_length_bound() >= 32);
    }
}
