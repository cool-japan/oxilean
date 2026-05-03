//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    CircuitEvaluator, CommunicationMatrixAnalyzer, DpllSolver, GateKind,
    ParameterizedAlgorithmChecker, ResolutionProverSmall, SensitivityChecker, SudokuSolver,
    TwoSatSolver,
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
#[allow(dead_code)]
pub fn impl_pi(name: &str, dom: Expr, body: Expr) -> Expr {
    pi(BinderInfo::Implicit, name, dom, body)
}
pub fn bvar(n: u32) -> Expr {
    Expr::BVar(n)
}
pub fn nat_ty() -> Expr {
    cst("Nat")
}
/// Language : Type — a set of strings (decidable predicate on strings)
pub fn language_ty() -> Expr {
    arrow(cst("String"), prop())
}
/// TimeComplexity : (Nat → Nat) → Type
/// Represents a function f : ℕ → ℕ as a time bound
pub fn time_complexity_ty() -> Expr {
    type0()
}
/// TuringMachine : Type — a deterministic Turing machine
pub fn turing_machine_ty() -> Expr {
    type0()
}
/// NTM : Type — a nondeterministic Turing machine
pub fn ntm_ty() -> Expr {
    type0()
}
/// OracleMachine : Language → Type — a TM with oracle access to a language
pub fn oracle_machine_ty() -> Expr {
    arrow(language_ty(), type0())
}
/// DTIME (f : Nat → Nat) : Language → Prop
/// L ∈ DTIME(f) if some TM decides L in O(f(n)) steps
pub fn dtime_ty() -> Expr {
    arrow(arrow(nat_ty(), nat_ty()), arrow(language_ty(), prop()))
}
/// NTIME (f : Nat → Nat) : Language → Prop
/// L ∈ NTIME(f) if some NTM decides L in O(f(n)) steps
pub fn ntime_ty() -> Expr {
    arrow(arrow(nat_ty(), nat_ty()), arrow(language_ty(), prop()))
}
/// DSPACE (f : Nat → Nat) : Language → Prop
/// L ∈ DSPACE(f) if some TM decides L using O(f(n)) space
pub fn dspace_ty() -> Expr {
    arrow(arrow(nat_ty(), nat_ty()), arrow(language_ty(), prop()))
}
/// NSPACE (f : Nat → Nat) : Language → Prop
pub fn nspace_ty() -> Expr {
    arrow(arrow(nat_ty(), nat_ty()), arrow(language_ty(), prop()))
}
/// P : Language → Prop — polynomial time decidable
pub fn class_p_ty() -> Expr {
    arrow(language_ty(), prop())
}
/// NP : Language → Prop — nondeterministic polynomial time
pub fn class_np_ty() -> Expr {
    arrow(language_ty(), prop())
}
/// coNP : Language → Prop — complement of NP
pub fn class_conp_ty() -> Expr {
    arrow(language_ty(), prop())
}
/// PSPACE : Language → Prop — polynomial space
pub fn class_pspace_ty() -> Expr {
    arrow(language_ty(), prop())
}
/// NPSPACE : Language → Prop — nondeterministic polynomial space
pub fn class_npspace_ty() -> Expr {
    arrow(language_ty(), prop())
}
/// EXP : Language → Prop — exponential time 2^(n^c)
pub fn class_exp_ty() -> Expr {
    arrow(language_ty(), prop())
}
/// NEXP : Language → Prop — nondeterministic exponential time
pub fn class_nexp_ty() -> Expr {
    arrow(language_ty(), prop())
}
/// L : Language → Prop — logarithmic space
pub fn class_l_ty() -> Expr {
    arrow(language_ty(), prop())
}
/// NL : Language → Prop — nondeterministic logarithmic space
pub fn class_nl_ty() -> Expr {
    arrow(language_ty(), prop())
}
/// PH : Language → Prop — polynomial hierarchy
pub fn class_ph_ty() -> Expr {
    arrow(language_ty(), prop())
}
/// SigmaP (k : Nat) : Language → Prop — k-th level of polynomial hierarchy
pub fn sigma_p_ty() -> Expr {
    arrow(nat_ty(), arrow(language_ty(), prop()))
}
/// PiP (k : Nat) : Language → Prop
pub fn pi_p_ty() -> Expr {
    arrow(nat_ty(), arrow(language_ty(), prop()))
}
/// DeltaP (k : Nat) : Language → Prop
pub fn delta_p_ty() -> Expr {
    arrow(nat_ty(), arrow(language_ty(), prop()))
}
/// SharpP : Language → Prop — counting problems
pub fn class_sharp_p_ty() -> Expr {
    arrow(language_ty(), prop())
}
/// RP : Language → Prop — randomized polynomial time
pub fn class_rp_ty() -> Expr {
    arrow(language_ty(), prop())
}
/// coRP : Language → Prop
pub fn class_corp_ty() -> Expr {
    arrow(language_ty(), prop())
}
/// BPP : Language → Prop — bounded-error probabilistic polynomial time
pub fn class_bpp_ty() -> Expr {
    arrow(language_ty(), prop())
}
/// ZPP : Language → Prop — zero-error probabilistic polynomial time
pub fn class_zpp_ty() -> Expr {
    arrow(language_ty(), prop())
}
/// PP : Language → Prop — probabilistic polynomial time (unbounded error)
pub fn class_pp_ty() -> Expr {
    arrow(language_ty(), prop())
}
/// IP : Language → Prop — interactive proof systems
pub fn class_ip_ty() -> Expr {
    arrow(language_ty(), prop())
}
/// PSpace: L ∈ PSPACE means L is decidable in polynomial space
pub fn in_pspace_ty() -> Expr {
    arrow(language_ty(), prop())
}
/// PolyManyOneReducible (A B : Language) : Prop
/// A ≤_m^p B — polynomial-time many-one reduction from A to B
pub fn poly_many_one_reducible_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        language_ty(),
        pi(BinderInfo::Default, "B", language_ty(), prop()),
    )
}
/// PolyTuringReducible (A B : Language) : Prop
/// A ≤_T^p B — polynomial-time Turing reduction (Cook reduction)
pub fn poly_turing_reducible_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        language_ty(),
        pi(BinderInfo::Default, "B", language_ty(), prop()),
    )
}
/// LogSpaceReducible (A B : Language) : Prop
/// A ≤_m^L B — logspace many-one reduction
pub fn logspace_reducible_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        language_ty(),
        pi(BinderInfo::Default, "B", language_ty(), prop()),
    )
}
/// NPHard (L : Language) : Prop — L is NP-hard under poly-many-one reductions
pub fn np_hard_ty() -> Expr {
    arrow(language_ty(), prop())
}
/// NPComplete (L : Language) : Prop — L ∈ NP ∧ NP-hard
pub fn np_complete_ty() -> Expr {
    arrow(language_ty(), prop())
}
/// PSPACEComplete (L : Language) : Prop
pub fn pspace_complete_ty() -> Expr {
    arrow(language_ty(), prop())
}
/// ExpComplete (L : Language) : Prop
pub fn exp_complete_ty() -> Expr {
    arrow(language_ty(), prop())
}
/// P ⊆ NP: every language decidable in poly-time is in NP
pub fn p_subset_np_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "L",
        language_ty(),
        arrow(app(cst("P"), bvar(0)), app(cst("NP"), bvar(1))),
    )
}
/// NP ⊆ PSPACE
pub fn np_subset_pspace_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "L",
        language_ty(),
        arrow(app(cst("NP"), bvar(0)), app(cst("PSPACE"), bvar(1))),
    )
}
/// PSPACE ⊆ EXP
pub fn pspace_subset_exp_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "L",
        language_ty(),
        arrow(app(cst("PSPACE"), bvar(0)), app(cst("EXP"), bvar(1))),
    )
}
/// Savitch's theorem: NPSPACE = PSPACE
pub fn savitch_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "L",
        language_ty(),
        app2(
            cst("Iff"),
            app(cst("NPSPACE"), bvar(0)),
            app(cst("PSPACE"), bvar(1)),
        ),
    )
}
/// Immerman-Szelepcsényi: NL = coNL
pub fn nl_eq_conl_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "L",
        language_ty(),
        app2(
            cst("Iff"),
            app(cst("NL"), bvar(0)),
            app(cst("coNL"), bvar(1)),
        ),
    )
}
/// Cook-Levin theorem: SAT is NP-complete
pub fn cook_levin_theorem_ty() -> Expr {
    app(cst("NPComplete"), cst("SAT"))
}
/// 3-SAT is NP-complete (reduction from SAT)
pub fn three_sat_np_complete_ty() -> Expr {
    app(cst("NPComplete"), cst("ThreeSAT"))
}
/// CLIQUE is NP-complete
pub fn clique_np_complete_ty() -> Expr {
    app(cst("NPComplete"), cst("CLIQUE"))
}
/// VERTEX-COVER is NP-complete
pub fn vertex_cover_np_complete_ty() -> Expr {
    app(cst("NPComplete"), cst("VertexCover"))
}
/// HAM-PATH is NP-complete (Hamiltonian path)
pub fn ham_path_np_complete_ty() -> Expr {
    app(cst("NPComplete"), cst("HamiltonianPath"))
}
/// 3-COLORING is NP-complete
pub fn three_coloring_np_complete_ty() -> Expr {
    app(cst("NPComplete"), cst("GraphThreeColoring"))
}
/// SUBSET-SUM is NP-complete
pub fn subset_sum_np_complete_ty() -> Expr {
    app(cst("NPComplete"), cst("SubsetSum"))
}
/// KNAPSACK is NP-complete
pub fn knapsack_np_complete_ty() -> Expr {
    app(cst("NPComplete"), cst("Knapsack"))
}
/// QBF (Quantified Boolean Formula) is PSPACE-complete
pub fn qbf_pspace_complete_ty() -> Expr {
    app(cst("PSPACEComplete"), cst("QBF"))
}
/// TQBF is PSPACE-complete (also known as QBF)
pub fn tqbf_pspace_complete_ty() -> Expr {
    app(cst("PSPACEComplete"), cst("TQBF"))
}
/// Time hierarchy theorem: DTIME(f) ⊊ DTIME(g) when g ≫ f
/// ∀ f g, TimeConstructible g → (∀ n, f(n) * log(f(n)) < g(n)) →
///   ∃ L, L ∈ DTIME(g) ∧ L ∉ DTIME(f)
pub fn time_hierarchy_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        arrow(nat_ty(), nat_ty()),
        pi(
            BinderInfo::Default,
            "g",
            arrow(nat_ty(), nat_ty()),
            arrow(
                app(cst("TimeConstructible"), bvar(0)),
                arrow(
                    pi(
                        BinderInfo::Default,
                        "n",
                        nat_ty(),
                        app2(
                            cst("Nat.lt"),
                            app2(
                                cst("Nat.mul"),
                                app(bvar(3), bvar(0)),
                                app2(
                                    cst("Nat.add"),
                                    app(cst("Nat.log2"), app(bvar(3), bvar(1))),
                                    nat_lit(1),
                                ),
                            ),
                            app(bvar(2), bvar(1)),
                        ),
                    ),
                    app(
                        cst("Exists"),
                        pi(
                            BinderInfo::Default,
                            "L",
                            language_ty(),
                            app2(
                                cst("And"),
                                app2(cst("DTIME"), bvar(3), bvar(0)),
                                app(cst("Not"), app2(cst("DTIME"), bvar(4), bvar(1))),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// Space hierarchy theorem: DSPACE(f) ⊊ DSPACE(g) when g ≫ f
pub fn space_hierarchy_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        arrow(nat_ty(), nat_ty()),
        pi(
            BinderInfo::Default,
            "g",
            arrow(nat_ty(), nat_ty()),
            arrow(
                app(cst("SpaceConstructible"), bvar(0)),
                arrow(
                    pi(
                        BinderInfo::Default,
                        "n",
                        nat_ty(),
                        app2(cst("Nat.lt"), app(bvar(3), bvar(0)), app(bvar(2), bvar(1))),
                    ),
                    app(
                        cst("Exists"),
                        pi(
                            BinderInfo::Default,
                            "L",
                            language_ty(),
                            app2(
                                cst("And"),
                                app2(cst("DSPACE"), bvar(3), bvar(0)),
                                app(cst("Not"), app2(cst("DSPACE"), bvar(4), bvar(1))),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// Ladner's theorem: if P ≠ NP, there exist NP-intermediate problems
pub fn ladner_theorem_ty() -> Expr {
    arrow(
        app(cst("Not"), app2(cst("Eq"), cst("ClassP"), cst("ClassNP"))),
        app(
            cst("Exists"),
            pi(
                BinderInfo::Default,
                "L",
                language_ty(),
                app2(
                    cst("And"),
                    app2(
                        cst("And"),
                        app(cst("NP"), bvar(0)),
                        app(cst("Not"), app(cst("P"), bvar(1))),
                    ),
                    app(cst("Not"), app(cst("NPComplete"), bvar(2))),
                ),
            ),
        ),
    )
}
/// Baker-Gill-Solovay: There exist oracles A, B such that P^A = NP^A and P^B ≠ NP^B
pub fn bgs_theorem_ty() -> Expr {
    app2(
        cst("And"),
        app(
            cst("Exists"),
            pi(
                BinderInfo::Default,
                "A",
                language_ty(),
                app2(
                    cst("Eq"),
                    app(cst("OracleP"), bvar(0)),
                    app(cst("OracleNP"), bvar(1)),
                ),
            ),
        ),
        app(
            cst("Exists"),
            pi(
                BinderInfo::Default,
                "B",
                language_ty(),
                app(
                    cst("Not"),
                    app2(
                        cst("Eq"),
                        app(cst("OracleP"), bvar(0)),
                        app(cst("OracleNP"), bvar(1)),
                    ),
                ),
            ),
        ),
    )
}
/// IP = PSPACE (Shamir's theorem)
pub fn shamir_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "L",
        language_ty(),
        app2(
            cst("Iff"),
            app(cst("IP"), bvar(0)),
            app(cst("PSPACE"), bvar(1)),
        ),
    )
}
/// PCP theorem: NP = PCP(log n, 1)
pub fn pcp_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "L",
        language_ty(),
        app2(
            cst("Iff"),
            app(cst("NP"), bvar(0)),
            app(cst("PCP"), bvar(1)),
        ),
    )
}
pub fn nat_lit(n: u64) -> Expr {
    Expr::Lit(oxilean_kernel::Literal::Nat(n))
}
/// BoolCircuit : Nat → Type — a Boolean circuit of size n
pub fn bool_circuit_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// NC (k : Nat) : Language → Prop — NC^k class (logspace-uniform NC^k circuits)
pub fn nc_class_ty() -> Expr {
    arrow(nat_ty(), arrow(language_ty(), prop()))
}
/// AC (k : Nat) : Language → Prop — AC^k class (unbounded fan-in)
pub fn ac_class_ty() -> Expr {
    arrow(nat_ty(), arrow(language_ty(), prop()))
}
/// TC (k : Nat) : Language → Prop — TC^k class (threshold gates)
pub fn tc_class_ty() -> Expr {
    arrow(nat_ty(), arrow(language_ty(), prop()))
}
/// NC^1 ⊆ L ⊆ NL ⊆ NC^2 ⊆ P
pub fn nc_l_containment_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "L",
        language_ty(),
        pi(
            BinderInfo::Default,
            "k",
            nat_ty(),
            arrow(
                app2(cst("NC"), nat_lit(1), bvar(1)),
                app(cst("LogSpace"), bvar(2)),
            ),
        ),
    )
}
/// AC^0 ⊊ NC^1 — parity is not in AC^0 (Furst-Saxe-Sipser/Håstad)
pub fn ac0_strict_nc1_ty() -> Expr {
    app(
        cst("Not"),
        app2(cst("Eq"), cst("ClassAC0"), cst("ClassNC1")),
    )
}
/// IsApproximable (r : Rat) (L : Language) : Prop
/// L has a polynomial-time r-approximation algorithm
pub fn is_approximable_ty() -> Expr {
    arrow(cst("Rat"), arrow(language_ty(), prop()))
}
/// MaxClique is not approximable within n^(1-ε) unless P=NP
pub fn clique_inapprox_ty() -> Expr {
    arrow(
        app(cst("Not"), app2(cst("Eq"), cst("ClassP"), cst("ClassNP"))),
        pi(
            BinderInfo::Default,
            "ε",
            cst("Real"),
            arrow(
                app2(cst("Real.lt"), nat_lit(0).into_real(), bvar(0)),
                app(cst("Not"), app(cst("Approximable"), cst("MaxClique"))),
            ),
        ),
    )
}
/// FPT : Language → Prop — fixed-parameter tractable
pub fn fpt_ty() -> Expr {
    arrow(language_ty(), prop())
}
/// W\[1\] : Language → Prop — first level of W-hierarchy (parameterized complexity)
pub fn w1_ty() -> Expr {
    arrow(language_ty(), prop())
}
/// CLIQUE (parameterized by clique size) is W\[1\]-complete
pub fn clique_w1_complete_ty() -> Expr {
    app(cst("W1Complete"), cst("ParameterizedCLIQUE"))
}
/// Build the computational complexity theory environment.
pub fn build_complexity_env(env: &mut Environment) -> Result<(), String> {
    for (name, ty) in [
        ("Language", language_ty()),
        ("TuringMachine", turing_machine_ty()),
        ("NTM", ntm_ty()),
        ("DTIME", dtime_ty()),
        ("NTIME", ntime_ty()),
        ("DSPACE", dspace_ty()),
        ("NSPACE", nspace_ty()),
        ("P", class_p_ty()),
        ("NP", class_np_ty()),
        ("coNP", class_conp_ty()),
        ("PSPACE", class_pspace_ty()),
        ("NPSPACE", class_npspace_ty()),
        ("EXP", class_exp_ty()),
        ("NEXP", class_nexp_ty()),
        ("L", class_l_ty()),
        ("LogSpace", class_l_ty()),
        ("coNL", class_nl_ty()),
        ("NL", class_nl_ty()),
        ("PH", class_ph_ty()),
        ("SigmaP", sigma_p_ty()),
        ("PiP", pi_p_ty()),
        ("DeltaP", delta_p_ty()),
        ("SharpP", class_sharp_p_ty()),
        ("RP", class_rp_ty()),
        ("coRP", class_corp_ty()),
        ("BPP", class_bpp_ty()),
        ("ZPP", class_zpp_ty()),
        ("PP", class_pp_ty()),
        ("IP", class_ip_ty()),
        ("NC", nc_class_ty()),
        ("AC", ac_class_ty()),
        ("TC", tc_class_ty()),
        ("FPT", fpt_ty()),
        ("W1", w1_ty()),
        ("PolyManyOneReducible", poly_many_one_reducible_ty()),
        ("PolyTuringReducible", poly_turing_reducible_ty()),
        ("LogSpaceReducible", logspace_reducible_ty()),
        ("NPHard", np_hard_ty()),
        ("NPComplete", np_complete_ty()),
        ("PSPACEComplete", pspace_complete_ty()),
        ("ExpComplete", exp_complete_ty()),
        ("W1Complete", fpt_ty()),
        ("IsApproximable", is_approximable_ty()),
        ("Approximable", arrow(language_ty(), prop())),
        (
            "TimeConstructible",
            arrow(arrow(nat_ty(), nat_ty()), prop()),
        ),
        (
            "SpaceConstructible",
            arrow(arrow(nat_ty(), nat_ty()), prop()),
        ),
        (
            "OracleP",
            arrow(language_ty(), arrow(language_ty(), prop())),
        ),
        (
            "OracleNP",
            arrow(language_ty(), arrow(language_ty(), prop())),
        ),
        ("PCP", class_np_ty()),
        ("ClassP", type0()),
        ("ClassNP", type0()),
        ("ClassAC0", type0()),
        ("ClassNC1", type0()),
        ("SAT", arrow(cst("String"), prop())),
        ("ThreeSAT", arrow(cst("String"), prop())),
        ("CLIQUE", arrow(cst("String"), prop())),
        ("VertexCover", arrow(cst("String"), prop())),
        ("HamiltonianPath", arrow(cst("String"), prop())),
        ("GraphThreeColoring", arrow(cst("String"), prop())),
        ("SubsetSum", arrow(cst("String"), prop())),
        ("Knapsack", arrow(cst("String"), prop())),
        ("QBF", arrow(cst("String"), prop())),
        ("TQBF", arrow(cst("String"), prop())),
        ("MaxClique", arrow(cst("String"), prop())),
        ("ParameterizedCLIQUE", arrow(cst("String"), prop())),
    ] {
        env.add(Declaration::Axiom {
            name: Name::str(name),
            univ_params: vec![],
            ty,
        })
        .ok();
    }
    for (name, ty) in [
        ("Complexity.p_subset_np", p_subset_np_ty()),
        ("Complexity.np_subset_pspace", np_subset_pspace_ty()),
        ("Complexity.pspace_subset_exp", pspace_subset_exp_ty()),
        ("Complexity.savitch", savitch_theorem_ty()),
        ("Complexity.nl_eq_conl", nl_eq_conl_ty()),
        ("Complexity.cook_levin", cook_levin_theorem_ty()),
        (
            "Complexity.three_sat_np_complete",
            three_sat_np_complete_ty(),
        ),
        ("Complexity.clique_np_complete", clique_np_complete_ty()),
        (
            "Complexity.vertex_cover_np_complete",
            vertex_cover_np_complete_ty(),
        ),
        ("Complexity.ham_path_np_complete", ham_path_np_complete_ty()),
        (
            "Complexity.three_coloring_np_complete",
            three_coloring_np_complete_ty(),
        ),
        (
            "Complexity.subset_sum_np_complete",
            subset_sum_np_complete_ty(),
        ),
        ("Complexity.knapsack_np_complete", knapsack_np_complete_ty()),
        ("Complexity.qbf_pspace_complete", qbf_pspace_complete_ty()),
        ("Complexity.tqbf_pspace_complete", tqbf_pspace_complete_ty()),
        ("Complexity.time_hierarchy", time_hierarchy_theorem_ty()),
        ("Complexity.space_hierarchy", space_hierarchy_theorem_ty()),
        ("Complexity.shamir", shamir_theorem_ty()),
        ("Complexity.pcp", pcp_theorem_ty()),
        ("Complexity.ac0_strict_nc1", ac0_strict_nc1_ty()),
        ("Complexity.clique_w1_complete", clique_w1_complete_ty()),
    ] {
        env.add(Declaration::Axiom {
            name: Name::str(name),
            univ_params: vec![],
            ty,
        })
        .ok();
    }
    for (name, ty) in [
        ("Algorithm", type0()),
        ("kSAT", cst("ParameterizedProblem")),
        ("ThreeSum", language_ty()),
        ("APSP", language_ty()),
        ("OrthogonalVectors", language_ty()),
        ("SubExpTime", type0()),
        ("SubCubicAlgorithmFor", arrow(language_ty(), prop())),
        ("HasAlgorithmRunningIn", arrow(language_ty(), prop())),
        ("SETHHolds", arrow(prop(), prop())),
        ("ThreeSumHardnessHolds", arrow(prop(), prop())),
        (
            "Solves",
            arrow(cst("Algorithm"), arrow(cst("ParameterizedProblem"), prop())),
        ),
        ("RunsIn", arrow(cst("Algorithm"), arrow(type0(), prop()))),
        ("ParameterizedProblem", type0()),
        (
            "WClass",
            arrow(nat_ty(), arrow(cst("ParameterizedProblem"), prop())),
        ),
        ("XP", arrow(cst("ParameterizedProblem"), prop())),
        ("KClique", cst("ParameterizedProblem")),
        ("W1Hard", arrow(cst("ParameterizedProblem"), prop())),
        (
            "HasKernelOfSize",
            arrow(cst("ParameterizedProblem"), arrow(nat_ty(), prop())),
        ),
        (
            "RunsInFPTTime",
            arrow(
                cst("ParameterizedProblem"),
                arrow(arrow(nat_ty(), nat_ty()), prop()),
            ),
        ),
        ("AC0", ac0_class_ty()),
        ("TC0", tc0_class_ty()),
        ("NC1Class", nc1_class_ty()),
        ("Parity", language_ty()),
        ("MAJORITY", arrow(nat_ty(), arrow(type0(), language_ty()))),
        ("FormulaSize", arrow(language_ty(), nat_ty())),
        ("ClassNC", type0()),
        ("NCClass", arrow(nat_ty(), type0())),
        ("Bits", type0()),
        ("Bool", type0()),
        (
            "MatrixRank",
            arrow(
                arrow(cst("Bits"), arrow(cst("Bits"), cst("Bool"))),
                nat_ty(),
            ),
        ),
        ("DetCommComplexity", det_comm_complexity_ty()),
        ("RandCommComplexity", rand_comm_complexity_ty()),
        ("QuantumCommComplexity", quantum_comm_complexity_ty()),
        ("Distribution", type0()),
        (
            "InfoCost",
            arrow(cst("Protocol"), arrow(cst("Distribution"), real_ty())),
        ),
        ("Protocol", type0()),
        ("CNFFormula", type0()),
        ("IntegerProgram", type0()),
        ("ResolutionWidth", resolution_width_ty()),
        ("ResolutionSize", arrow(cst("CNFFormula"), nat_ty())),
        ("HasPolySizeMonotoneCircuit", arrow(language_ty(), prop())),
        ("PolyFamily", type0()),
        ("VP", vp_class_ty()),
        ("VNP", vnp_class_ty()),
        ("VNPComplete", arrow(cst("PolyFamily"), prop())),
        ("PermanentFamily", cst("PolyFamily")),
        ("ClassVP", type0()),
        ("ClassVNP", type0()),
        ("DistProblem", type0()),
        ("OWFExists", arrow(prop(), prop())),
        ("HardOnAverage", arrow(language_ty(), prop())),
        ("SomeNPProblem", language_ty()),
        (
            "EfficientlyComputable",
            arrow(arrow(cst("Bits"), cst("Bits")), prop()),
        ),
        (
            "HardToInvert",
            arrow(arrow(cst("Bits"), cst("Bits")), prop()),
        ),
        ("WeakOWFExists", arrow(prop(), prop())),
        ("StrongOWFExists", arrow(prop(), prop())),
        ("PRGExists", arrow(prop(), prop())),
        ("BQP", class_bqp_ty()),
        ("QMA", class_qma_ty()),
        ("QCMA", class_qcma_ty()),
        ("QMAComplete", arrow(language_ty(), prop())),
        ("LocalHamiltonian", language_ty()),
        ("QMAClass", type0()),
        ("PCPClass", type0()),
        (
            "IsEpsilonTester",
            arrow(
                arrow(arrow(cst("Bits"), cst("Bool")), prop()),
                arrow(arrow(real_ty(), nat_ty()), prop()),
            ),
        ),
        ("Graph", type0()),
        (
            "BLRTest",
            arrow(arrow(cst("Bits"), cst("Bool")), arrow(nat_ty(), prop())),
        ),
        ("Passes", arrow(prop(), prop())),
        ("IsLinear", arrow(arrow(cst("Bits"), cst("Bool")), prop())),
        ("StreamingAlgorithm", type0()),
        ("StreamSpaceBound", arrow(language_ty(), nat_ty())),
        ("OnePassStreamingSpace", arrow(language_ty(), nat_ty())),
        ("DistinctElements", language_ty()),
        (
            "ApproxDegree",
            arrow(arrow(cst("Bits"), cst("Bool")), nat_ty()),
        ),
        (
            "QuantumQueryComplexity",
            arrow(arrow(cst("Bits"), cst("Bool")), nat_ty()),
        ),
        ("PolyDegree", poly_degree_ty()),
        ("Sensitivity", sensitivity_ty()),
        ("BlockSensitivity", block_sensitivity_ty()),
    ] {
        env.add(Declaration::Axiom {
            name: Name::str(name),
            univ_params: vec![],
            ty,
        })
        .ok();
    }
    for (name, ty) in [
        ("Complexity.seth_hypothesis", seth_hypothesis_ty()),
        ("Complexity.three_sum_hypothesis", three_sum_hypothesis_ty()),
        ("Complexity.apsp_conjecture", apsp_conjecture_ty()),
        ("Complexity.ov_conjecture", ov_conjecture_ty()),
        ("Complexity.seth_implies_3sum", seth_implies_3sum_ty()),
        ("Complexity.fpt_algorithm", fpt_algorithm_ty()),
        ("Complexity.w_hierarchy", w_hierarchy_ty()),
        ("Complexity.polynomial_kernel", polynomial_kernel_ty()),
        ("Complexity.fpt_subset_xp", fpt_subset_xp_ty()),
        (
            "Complexity.param_clique_w1_complete",
            param_clique_w1_complete_ty(),
        ),
        (
            "Complexity.circuit_hierarchy_containment",
            circuit_hierarchy_containment_ty(),
        ),
        ("Complexity.parity_not_ac0", parity_not_ac0_ty()),
        (
            "Complexity.majority_formula_size",
            majority_formula_size_ty(),
        ),
        ("Complexity.nc_hierarchy", nc_hierarchy_ty()),
        ("Complexity.log_rank_conjecture", log_rank_conjecture_ty()),
        (
            "Complexity.information_complexity_lb",
            information_complexity_lb_ty(),
        ),
        ("Complexity.size_width_tradeoff", size_width_tradeoff_ty()),
        ("Complexity.clique_monotone_lb", clique_monotone_lb_ty()),
        ("Complexity.vp_subset_vnp", vp_subset_vnp_ty()),
        (
            "Complexity.permanent_vnp_complete",
            permanent_vnp_complete_ty(),
        ),
        (
            "Complexity.vp_neq_vnp_conjecture",
            vp_neq_vnp_conjecture_ty(),
        ),
        ("Complexity.impagliazzo_world", impagliazzo_world_ty()),
        ("Complexity.owf_exists", owf_exists_ty()),
        (
            "Complexity.hardness_amplification",
            hardness_amplification_ty(),
        ),
        ("Complexity.prg_from_owf", prg_from_owf_ty()),
        ("Complexity.bqp_subset_pspace", bqp_subset_pspace_ty()),
        ("Complexity.bpp_subset_bqp", bpp_subset_bqp_ty()),
        (
            "Complexity.quantum_pcp_conjecture",
            quantum_pcp_conjecture_ty(),
        ),
        (
            "Complexity.local_hamiltonian_qma_complete",
            local_hamiltonian_qma_complete_ty(),
        ),
        ("Complexity.linearity_testing", linearity_testing_ty()),
        ("Complexity.comm_space_tradeoff", comm_space_tradeoff_ty()),
        (
            "Complexity.distinct_elements_space_lb",
            distinct_elements_space_lb_ty(),
        ),
        (
            "Complexity.sensitivity_conjecture",
            sensitivity_conjecture_ty(),
        ),
        ("Complexity.poly_method_lb", poly_method_lb_ty()),
        (
            "Complexity.bs_sensitivity_relation",
            bs_sensitivity_relation_ty(),
        ),
    ] {
        env.add(Declaration::Axiom {
            name: Name::str(name),
            univ_params: vec![],
            ty,
        })
        .ok();
    }
    Ok(())
}
/// Subset sum solver: given a set of integers and a target, find a subset summing to target.
pub fn subset_sum(nums: &[i64], target: i64) -> Option<Vec<usize>> {
    use std::collections::HashMap;
    let mut dp: HashMap<i64, Vec<usize>> = HashMap::new();
    dp.insert(0, vec![]);
    for (i, &num) in nums.iter().enumerate() {
        let old_dp: Vec<(i64, Vec<usize>)> = dp.iter().map(|(&s, v)| (s, v.clone())).collect();
        for (sum, mut indices) in old_dp {
            let new_sum = sum + num;
            dp.entry(new_sum).or_insert_with(|| {
                indices.push(i);
                indices
            });
        }
    }
    dp.remove(&target)
}
/// Knapsack 0/1 solver: maximize value with weight capacity.
/// Returns (total_value, selected_item_indices).
pub fn knapsack_01(weights: &[usize], values: &[usize], capacity: usize) -> (usize, Vec<usize>) {
    let n = weights.len();
    let mut dp = vec![vec![0usize; capacity + 1]; n + 1];
    for i in 1..=n {
        for w in 0..=capacity {
            dp[i][w] = dp[i - 1][w];
            if weights[i - 1] <= w {
                let alt = dp[i - 1][w - weights[i - 1]] + values[i - 1];
                if alt > dp[i][w] {
                    dp[i][w] = alt;
                }
            }
        }
    }
    let total = dp[n][capacity];
    let mut selected = vec![];
    let mut w = capacity;
    for i in (1..=n).rev() {
        if dp[i][w] != dp[i - 1][w] {
            selected.push(i - 1);
            w -= weights[i - 1];
        }
    }
    selected.reverse();
    (total, selected)
}
/// Graph coloring checker: given a coloring, verify it uses at most k colors
/// and no two adjacent vertices share a color.
pub fn verify_coloring(adj: &[Vec<usize>], coloring: &[usize], k: usize) -> bool {
    for (u, neighbors) in adj.iter().enumerate() {
        if coloring[u] >= k {
            return false;
        }
        for &v in neighbors {
            if coloring[u] == coloring[v] {
                return false;
            }
        }
    }
    true
}
/// Greedy graph coloring (returns number of colors used and the coloring).
pub fn greedy_coloring(adj: &[Vec<usize>]) -> (usize, Vec<usize>) {
    let n = adj.len();
    let mut colors = vec![usize::MAX; n];
    let mut max_color = 0;
    for v in 0..n {
        let mut used = std::collections::HashSet::new();
        for &u in &adj[v] {
            if colors[u] != usize::MAX {
                used.insert(colors[u]);
            }
        }
        let mut c = 0;
        while used.contains(&c) {
            c += 1;
        }
        colors[v] = c;
        if c + 1 > max_color {
            max_color = c + 1;
        }
    }
    (max_color, colors)
}
trait IntoReal {
    fn into_real(self) -> Expr;
}
impl IntoReal for Expr {
    fn into_real(self) -> Expr {
        app(cst("Nat.toReal"), self)
    }
}
pub fn real_ty() -> Expr {
    cst("Real")
}
/// SETH (Strong Exponential Time Hypothesis):
/// k-SAT cannot be solved in time O(2^((1-ε)n)) for any ε > 0
pub fn seth_hypothesis_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "ε",
        real_ty(),
        arrow(
            app2(cst("Real.lt"), nat_lit(0).into_real(), bvar(0)),
            app(
                cst("Not"),
                pi(
                    BinderInfo::Default,
                    "k",
                    nat_ty(),
                    app(
                        cst("Exists"),
                        pi(
                            BinderInfo::Default,
                            "alg",
                            cst("Algorithm"),
                            app2(
                                cst("And"),
                                app2(cst("Solves"), bvar(0), cst("kSAT")),
                                app2(cst("RunsIn"), bvar(1), cst("SubExpTime")),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// 3SUM hypothesis: no O(n^(2-ε)) algorithm for 3SUM on n integers
pub fn three_sum_hypothesis_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "ε",
        real_ty(),
        arrow(
            app2(cst("Real.lt"), nat_lit(0).into_real(), bvar(0)),
            app(
                cst("Not"),
                app(
                    cst("Exists"),
                    pi(
                        BinderInfo::Default,
                        "alg",
                        cst("Algorithm"),
                        app2(
                            cst("And"),
                            app2(cst("Solves"), bvar(0), cst("ThreeSum")),
                            app(cst("SubQuadratic"), bvar(1)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// APSP conjecture: All-Pairs Shortest Paths requires Omega(n^3 / polylog n) time
pub fn apsp_conjecture_ty() -> Expr {
    app(cst("Not"), app(cst("SubCubicAlgorithmFor"), cst("APSP")))
}
/// OV conjecture: Orthogonal Vectors cannot be solved in O(n^(2-ε)) time
pub fn ov_conjecture_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "ε",
        real_ty(),
        arrow(
            app2(cst("Real.lt"), nat_lit(0).into_real(), bvar(0)),
            app(
                cst("Not"),
                app(cst("HasAlgorithmRunningIn"), cst("OrthogonalVectors")),
            ),
        ),
    )
}
/// SETH implies 3SUM hardness (reduction)
pub fn seth_implies_3sum_ty() -> Expr {
    arrow(
        app(cst("SETHHolds"), prop()),
        app(cst("ThreeSumHardnessHolds"), prop()),
    )
}
/// FPT algorithm running in f(k) * poly(n) time
pub fn fpt_algorithm_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "prob",
        cst("ParameterizedProblem"),
        arrow(
            app(cst("FPT"), bvar(0)),
            app(
                cst("Exists"),
                pi(
                    BinderInfo::Default,
                    "f",
                    arrow(nat_ty(), nat_ty()),
                    app2(cst("RunsInFPTTime"), bvar(1), bvar(0)),
                ),
            ),
        ),
    )
}
/// W-hierarchy: W\[1\] ⊆ W\[2\] ⊆ ... ⊆ W\[P\]
pub fn w_hierarchy_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "i",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "L",
            cst("ParameterizedProblem"),
            arrow(
                app2(cst("WClass"), bvar(1), bvar(0)),
                app2(
                    cst("WClass"),
                    app2(cst("Nat.add"), bvar(2), nat_lit(1)),
                    bvar(1),
                ),
            ),
        ),
    )
}
/// Kernelization: a problem has a polynomial kernel
pub fn polynomial_kernel_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "prob",
        cst("ParameterizedProblem"),
        pi(
            BinderInfo::Default,
            "c",
            nat_ty(),
            arrow(
                app2(
                    cst("HasKernelOfSize"),
                    bvar(1),
                    app2(cst("Nat.pow"), bvar(0), nat_lit(2)),
                ),
                app(cst("FPT"), bvar(2)),
            ),
        ),
    )
}
/// FPT ⊆ XP (slice-wise polynomial)
pub fn fpt_subset_xp_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "L",
        cst("ParameterizedProblem"),
        arrow(app(cst("FPT"), bvar(0)), app(cst("XP"), bvar(1))),
    )
}
/// CLIQUE parameterized by k is W\[1\]-complete (parameterized version)
pub fn param_clique_w1_complete_ty() -> Expr {
    app2(
        cst("And"),
        app(cst("W1Hard"), cst("KClique")),
        app(cst("WClass"), nat_lit(1)),
    )
}
/// AC0 circuit class: constant depth, polynomial size, unbounded fan-in
pub fn ac0_class_ty() -> Expr {
    arrow(language_ty(), prop())
}
/// TC0 circuit class: constant depth threshold circuits
pub fn tc0_class_ty() -> Expr {
    arrow(language_ty(), prop())
}
/// NC1 circuit class: log-depth, polynomial size, fan-in 2
pub fn nc1_class_ty() -> Expr {
    arrow(language_ty(), prop())
}
/// AC0 ⊆ TC0 ⊆ NC1
pub fn circuit_hierarchy_containment_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "L",
        language_ty(),
        arrow(
            app(cst("AC0"), bvar(0)),
            arrow(app(cst("TC0"), bvar(1)), app(cst("NC1Class"), bvar(2))),
        ),
    )
}
/// Parity not in AC0 (Håstad's switching lemma)
pub fn parity_not_ac0_ty() -> Expr {
    app(cst("Not"), app(cst("AC0"), cst("Parity")))
}
/// Formula size lower bound: MAJORITY requires Omega(n^2) formula size
pub fn majority_formula_size_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        app2(
            cst("Nat.le"),
            app2(cst("Nat.mul"), bvar(0), bvar(0)),
            app(cst("FormulaSize"), app2(cst("MAJORITY"), bvar(1), nat_ty())),
        ),
    )
}
/// NC hierarchy: NC^k ⊊ NC^(k+1) assuming NC ≠ P
pub fn nc_hierarchy_ty() -> Expr {
    arrow(
        app(cst("Not"), app2(cst("Eq"), cst("ClassNC"), cst("ClassP"))),
        pi(
            BinderInfo::Default,
            "k",
            nat_ty(),
            app(
                cst("Not"),
                app2(
                    cst("Eq"),
                    app(cst("NCClass"), bvar(0)),
                    app(cst("NCClass"), app2(cst("Nat.add"), bvar(1), nat_lit(1))),
                ),
            ),
        ),
    )
}
/// Deterministic communication complexity of a function f
pub fn det_comm_complexity_ty() -> Expr {
    arrow(
        arrow(cst("Bits"), arrow(cst("Bits"), cst("Bool"))),
        nat_ty(),
    )
}
/// Randomized communication complexity
pub fn rand_comm_complexity_ty() -> Expr {
    arrow(
        arrow(cst("Bits"), arrow(cst("Bits"), cst("Bool"))),
        nat_ty(),
    )
}
/// Quantum communication complexity
pub fn quantum_comm_complexity_ty() -> Expr {
    arrow(
        arrow(cst("Bits"), arrow(cst("Bits"), cst("Bool"))),
        nat_ty(),
    )
}
/// Log-rank conjecture: D(f) ≤ poly(log rank(M_f))
pub fn log_rank_conjecture_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        arrow(cst("Bits"), arrow(cst("Bits"), cst("Bool"))),
        app(
            cst("Exists"),
            pi(
                BinderInfo::Default,
                "c",
                nat_ty(),
                app2(
                    cst("Nat.le"),
                    app(cst("DetCommComplexity"), bvar(1)),
                    app2(
                        cst("Nat.pow"),
                        app(cst("Nat.log2"), app(cst("MatrixRank"), bvar(2))),
                        bvar(0),
                    ),
                ),
            ),
        ),
    )
}
/// Information complexity lower bound for communication
pub fn information_complexity_lb_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        arrow(cst("Bits"), arrow(cst("Bits"), cst("Bool"))),
        pi(
            BinderInfo::Default,
            "μ",
            cst("Distribution"),
            arrow(
                app2(
                    cst("Nat.le"),
                    app(cst("InfoCost"), bvar(0)),
                    app(cst("RandCommComplexity"), bvar(1)),
                ),
                prop(),
            ),
        ),
    )
}
/// Resolution proof system: a refutation of a CNF formula
pub fn resolution_proof_ty() -> Expr {
    arrow(cst("CNFFormula"), type0())
}
/// Width of a resolution refutation
pub fn resolution_width_ty() -> Expr {
    arrow(cst("CNFFormula"), nat_ty())
}
/// Size-width tradeoff for resolution (Ben-Sasson and Wigderson)
pub fn size_width_tradeoff_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "F",
        cst("CNFFormula"),
        app2(
            cst("Nat.le"),
            app2(
                cst("Nat.mul"),
                app(cst("ResolutionWidth"), bvar(0)),
                app(cst("ResolutionWidth"), bvar(1)),
            ),
            app(cst("Nat.log2"), app(cst("ResolutionSize"), bvar(2))),
        ),
    )
}
/// Monotone circuit lower bound: CLIQUE requires super-polynomial monotone circuits
pub fn clique_monotone_lb_ty() -> Expr {
    app(
        cst("Not"),
        app(cst("HasPolySizeMonotoneCircuit"), cst("CLIQUE")),
    )
}
/// Cutting planes proof system
pub fn cutting_planes_ty() -> Expr {
    arrow(cst("IntegerProgram"), type0())
}
/// VP (Valiant's P): polynomial families computed by poly-size circuits
pub fn vp_class_ty() -> Expr {
    arrow(cst("PolyFamily"), prop())
}
/// VNP (Valiant's NP): polynomial families in the permanent class
pub fn vnp_class_ty() -> Expr {
    arrow(cst("PolyFamily"), prop())
}
/// Permanent polynomial family
pub fn permanent_ty() -> Expr {
    cst("PolyFamily")
}
/// Determinant polynomial family
pub fn determinant_ty() -> Expr {
    cst("PolyFamily")
}
/// VP ⊆ VNP
pub fn vp_subset_vnp_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        cst("PolyFamily"),
        arrow(app(cst("VP"), bvar(0)), app(cst("VNP"), bvar(1))),
    )
}
/// Valiant's theorem: Permanent is VNP-complete
pub fn permanent_vnp_complete_ty() -> Expr {
    app(cst("VNPComplete"), cst("PermanentFamily"))
}
/// VP ≠ VNP conjecture (algebraic P vs NP)
pub fn vp_neq_vnp_conjecture_ty() -> Expr {
    app(cst("Not"), app2(cst("Eq"), cst("ClassVP"), cst("ClassVNP")))
}
/// Distributional NP: NP with a distribution over inputs
pub fn dist_np_ty() -> Expr {
    arrow(cst("Distribution"), arrow(language_ty(), prop()))
}
/// Levin reduction (average-case reduction)
pub fn levin_reduction_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        cst("DistProblem"),
        pi(BinderInfo::Default, "B", cst("DistProblem"), prop()),
    )
}
/// Average-case complete problem
pub fn avg_case_complete_ty() -> Expr {
    arrow(cst("DistProblem"), prop())
}
/// Impagliazzo's worlds hypothesis (simplified: OWF exists ↔ hard on avg)
pub fn impagliazzo_world_ty() -> Expr {
    app2(
        cst("Iff"),
        app(cst("OWFExists"), prop()),
        app(cst("HardOnAverage"), cst("SomeNPProblem")),
    )
}
/// One-way function existence
pub fn owf_exists_ty() -> Expr {
    app(
        cst("Exists"),
        pi(
            BinderInfo::Default,
            "f",
            arrow(cst("Bits"), cst("Bits")),
            app2(
                cst("And"),
                app(cst("EfficientlyComputable"), bvar(0)),
                app(cst("HardToInvert"), bvar(1)),
            ),
        ),
    )
}
/// Hardness amplification: weak OWF → strong OWF
pub fn hardness_amplification_ty() -> Expr {
    arrow(
        app(cst("WeakOWFExists"), prop()),
        app(cst("StrongOWFExists"), prop()),
    )
}
/// PRG (Pseudorandom Generator) existence from OWF
pub fn prg_from_owf_ty() -> Expr {
    arrow(app(cst("OWFExists"), prop()), app(cst("PRGExists"), prop()))
}
/// Computational indistinguishability
pub fn comp_indist_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "D1",
        cst("Distribution"),
        pi(BinderInfo::Default, "D2", cst("Distribution"), prop()),
    )
}
/// BQP: bounded-error quantum polynomial time
pub fn class_bqp_ty() -> Expr {
    arrow(language_ty(), prop())
}
/// QMA: quantum Merlin-Arthur (quantum analog of NP)
pub fn class_qma_ty() -> Expr {
    arrow(language_ty(), prop())
}
/// QCMA: classical witness quantum verifier
pub fn class_qcma_ty() -> Expr {
    arrow(language_ty(), prop())
}
/// BQP ⊆ PSPACE
pub fn bqp_subset_pspace_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "L",
        language_ty(),
        arrow(app(cst("BQP"), bvar(0)), app(cst("PSPACE"), bvar(1))),
    )
}
/// BPP ⊆ BQP
pub fn bpp_subset_bqp_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "L",
        language_ty(),
        arrow(app(cst("BPP"), bvar(0)), app(cst("BQP"), bvar(1))),
    )
}
/// Quantum PCP conjecture: QMA ⊆ MIP* = RE (negation of classical PCP analog)
pub fn quantum_pcp_conjecture_ty() -> Expr {
    app(
        cst("Not"),
        app2(cst("Eq"), cst("QMAClass"), cst("PCPClass")),
    )
}
/// Local Hamiltonian problem is QMA-complete
pub fn local_hamiltonian_qma_complete_ty() -> Expr {
    app(cst("QMAComplete"), cst("LocalHamiltonian"))
}
/// Testability of a boolean function property with query complexity q(ε)
pub fn property_testable_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "P",
        arrow(arrow(cst("Bits"), cst("Bool")), prop()),
        arrow(
            app(
                cst("Exists"),
                pi(
                    BinderInfo::Default,
                    "q",
                    arrow(real_ty(), nat_ty()),
                    app2(cst("IsEpsilonTester"), bvar(0), bvar(1)),
                ),
            ),
            prop(),
        ),
    )
}
/// Graph property testable if it has a constant query complexity tester
pub fn graph_property_testable_ty() -> Expr {
    arrow(arrow(cst("Graph"), prop()), prop())
}
/// Tolerant property testing: distinguishes ε1-close from ε2-far
pub fn tolerant_testing_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "P",
        arrow(arrow(cst("Bits"), cst("Bool")), prop()),
        pi(
            BinderInfo::Default,
            "ε1",
            real_ty(),
            pi(
                BinderInfo::Default,
                "ε2",
                real_ty(),
                arrow(app2(cst("Real.lt"), bvar(1), bvar(0)), prop()),
            ),
        ),
    )
}
/// Linearity testing (BLR test)
pub fn linearity_testing_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        arrow(cst("Bits"), cst("Bool")),
        arrow(
            app(cst("Passes"), app2(cst("BLRTest"), bvar(0), nat_lit(3))),
            app(cst("IsLinear"), bvar(1)),
        ),
    )
}
/// Space complexity of streaming algorithm
pub fn stream_space_ty() -> Expr {
    arrow(cst("StreamingAlgorithm"), nat_ty())
}
/// Communication-space lower bound tradeoff
pub fn comm_space_tradeoff_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        arrow(cst("Bits"), arrow(cst("Bits"), cst("Bool"))),
        app2(
            cst("Nat.le"),
            app(cst("RandCommComplexity"), bvar(0)),
            app(cst("StreamSpaceBound"), bvar(1)),
        ),
    )
}
/// Information cost of a protocol
pub fn information_cost_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "π",
        cst("Protocol"),
        pi(BinderInfo::Default, "μ", cst("Distribution"), real_ty()),
    )
}
/// Distinct elements requires Omega(n) space in one-pass streaming
pub fn distinct_elements_space_lb_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        app2(
            cst("Nat.le"),
            bvar(0),
            app(cst("OnePassStreamingSpace"), cst("DistinctElements")),
        ),
    )
}
/// Polynomial degree of a Boolean function
pub fn poly_degree_ty() -> Expr {
    arrow(arrow(cst("Bits"), cst("Bool")), nat_ty())
}
/// Block sensitivity of a Boolean function
pub fn block_sensitivity_ty() -> Expr {
    arrow(arrow(cst("Bits"), cst("Bool")), nat_ty())
}
/// Sensitivity of a Boolean function
pub fn sensitivity_ty() -> Expr {
    arrow(arrow(cst("Bits"), cst("Bool")), nat_ty())
}
/// Sensitivity conjecture (Huang 2019): s(f) ≥ sqrt(deg(f))
pub fn sensitivity_conjecture_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        arrow(cst("Bits"), cst("Bool")),
        app2(
            cst("Nat.le"),
            app(cst("Sensitivity"), bvar(0)),
            app(cst("Nat.sqrt"), app(cst("PolyDegree"), bvar(1))),
        ),
    )
}
/// Polynomial method: deg(f) ≤ D(f) (approximate degree lower bounds)
pub fn poly_method_lb_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        arrow(cst("Bits"), cst("Bool")),
        app2(
            cst("Nat.le"),
            app(cst("ApproxDegree"), bvar(0)),
            app(cst("QuantumQueryComplexity"), bvar(1)),
        ),
    )
}
/// Block sensitivity vs sensitivity: bs(f) ≤ s(f)^6 (old bound; now s(f)^2 ≥ bs(f) by Huang)
pub fn bs_sensitivity_relation_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        arrow(cst("Bits"), cst("Bool")),
        app2(
            cst("Nat.le"),
            app(cst("BlockSensitivity"), bvar(0)),
            app2(cst("Nat.pow"), app(cst("Sensitivity"), bvar(1)), nat_lit(2)),
        ),
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt;
    #[test]
    fn test_two_sat_simple() {
        let mut solver = TwoSatSolver::new(2);
        solver.add_clause(0, 2);
        solver.add_clause(1, 2);
        solver.add_clause(0, 3);
        let result = solver.solve();
        assert!(result.is_some(), "should be satisfiable");
    }
    #[test]
    fn test_two_sat_unsat() {
        let mut solver = TwoSatSolver::new(1);
        solver.add_clause(0, 0);
        solver.add_clause(1, 1);
        let result = solver.solve();
        assert!(result.is_none(), "should be unsatisfiable");
    }
    #[test]
    fn test_dpll_satisfiable() {
        let mut solver = DpllSolver::new(3);
        solver.add_clause(vec![1, 2]);
        solver.add_clause(vec![-1, 3]);
        solver.add_clause(vec![2, 3]);
        assert!(solver.solve().is_some());
    }
    #[test]
    fn test_dpll_unsatisfiable() {
        let mut solver = DpllSolver::new(1);
        solver.add_clause(vec![1]);
        solver.add_clause(vec![-1]);
        assert!(solver.solve().is_none());
    }
    #[test]
    fn test_subset_sum_found() {
        let nums = vec![3i64, 1, 4, 1, 5, 9, 2, 6];
        let target = 10;
        let result = subset_sum(&nums, target);
        assert!(result.is_some(), "Should find subset summing to 10");
        let indices = result.expect("result should be valid");
        let sum: i64 = indices.iter().map(|&i| nums[i]).sum();
        assert_eq!(sum, target);
    }
    #[test]
    fn test_knapsack() {
        let weights = vec![2, 3, 4, 5];
        let values = vec![3, 4, 5, 6];
        let capacity = 8;
        let (total, selected) = knapsack_01(&weights, &values, capacity);
        assert!(total > 0);
        let total_weight: usize = selected.iter().map(|&i| weights[i]).sum();
        assert!(total_weight <= capacity);
    }
    #[test]
    fn test_greedy_coloring() {
        let adj = vec![vec![1, 2], vec![0, 2], vec![0, 1]];
        let (colors, coloring) = greedy_coloring(&adj);
        assert!(colors >= 3);
        assert!(verify_coloring(&adj, &coloring, colors));
    }
    #[test]
    fn test_sudoku_solver() {
        #[rustfmt::skip]
        let grid: [u8; 81] = [
            5, 3, 0, 0, 7, 0, 0, 0, 0, 6, 0, 0, 1, 9, 5, 0, 0, 0, 0, 9, 8, 0, 0, 0, 0, 6,
            0, 8, 0, 0, 0, 6, 0, 0, 0, 3, 4, 0, 0, 8, 0, 3, 0, 0, 1, 7, 0, 0, 0, 2, 0, 0,
            0, 6, 0, 6, 0, 0, 0, 0, 2, 8, 0, 0, 0, 0, 4, 1, 9, 0, 0, 5, 0, 0, 0, 0, 8, 0,
            0, 7, 9,
        ];
        let mut solver = SudokuSolver::new(grid);
        assert!(solver.solve());
    }
    #[test]
    fn test_parameterized_algorithm_checker_within_bound() {
        let checker = ParameterizedAlgorithmChecker::new("treewidth");
        assert!(checker.check(70, 3, 10, |k| 1u64 << k, 1));
    }
    #[test]
    fn test_parameterized_algorithm_checker_exceeds_bound() {
        let checker = ParameterizedAlgorithmChecker::new("pathwidth");
        assert!(!checker.check(100, 3, 10, |k| 1u64 << k, 1));
    }
    #[test]
    fn test_parameterized_check_2k_n() {
        let checker = ParameterizedAlgorithmChecker::new("vertex_cover");
        assert!(checker.check_2k_n(300, 4, 20));
        assert!(!checker.check_2k_n(400, 4, 20));
    }
    #[test]
    fn test_circuit_evaluator_and() {
        let mut circ = CircuitEvaluator::new();
        let i0 = circ.add_gate(GateKind::Input(0), None, None);
        let i1 = circ.add_gate(GateKind::Input(1), None, None);
        circ.add_gate(GateKind::And, Some(i0), Some(i1));
        assert!(!circ.evaluate(&[false, true]));
        assert!(circ.evaluate(&[true, true]));
    }
    #[test]
    fn test_circuit_evaluator_or_not() {
        let mut circ = CircuitEvaluator::new();
        let i0 = circ.add_gate(GateKind::Input(0), None, None);
        let n0 = circ.add_gate(GateKind::Not, Some(i0), None);
        let i1 = circ.add_gate(GateKind::Input(1), None, None);
        circ.add_gate(GateKind::Or, Some(n0), Some(i1));
        assert!(circ.evaluate(&[false, false]));
        assert!(!circ.evaluate(&[true, false]));
    }
    #[test]
    fn test_communication_matrix_rank_gf2() {
        let mat = vec![vec![1u8, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];
        let analyzer = CommunicationMatrixAnalyzer::new(mat);
        assert_eq!(analyzer.rank_gf2(), 3);
    }
    #[test]
    fn test_communication_matrix_rank_gf2_zero_matrix() {
        let mat = vec![vec![0u8, 0], vec![0, 0]];
        let analyzer = CommunicationMatrixAnalyzer::new(mat);
        assert_eq!(analyzer.rank_gf2(), 0);
    }
    #[test]
    fn test_communication_matrix_log_rank_lb() {
        let mat = vec![
            vec![1u8, 0, 0, 0],
            vec![0, 1, 0, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 0, 1],
        ];
        let analyzer = CommunicationMatrixAnalyzer::new(mat);
        assert_eq!(analyzer.log_rank_lower_bound(), 2);
    }
    #[test]
    fn test_resolution_prover_refutes_contradiction() {
        let mut prover = ResolutionProverSmall::new();
        prover.add_clause(vec![1]);
        prover.add_clause(vec![-1]);
        assert!(prover.refute(100));
    }
    #[test]
    fn test_resolution_prover_satisfiable() {
        let mut prover = ResolutionProverSmall::new();
        prover.add_clause(vec![1, 2]);
        prover.add_clause(vec![-1, 2]);
        assert!(!prover.refute(200));
    }
    #[test]
    fn test_resolution_prover_three_variable_unsat() {
        let mut prover = ResolutionProverSmall::new();
        prover.add_clause(vec![1]);
        prover.add_clause(vec![-1, 2]);
        prover.add_clause(vec![-2]);
        assert!(prover.refute(500));
    }
    #[test]
    fn test_sensitivity_checker_parity_2bit() {
        let table = vec![false, true, true, false];
        let checker = SensitivityChecker::new(table);
        assert_eq!(checker.max_sensitivity(), 2);
    }
    #[test]
    fn test_sensitivity_checker_constant_function() {
        let table = vec![false; 8];
        let checker = SensitivityChecker::new(table);
        assert_eq!(checker.max_sensitivity(), 0);
        assert_eq!(checker.max_block_sensitivity(), 0);
    }
    #[test]
    fn test_sensitivity_checker_huang_theorem() {
        let table = vec![false, true, true, true];
        let checker = SensitivityChecker::new(table);
        assert!(checker.check_huang_theorem());
    }
    #[test]
    fn test_new_axiom_types_build_ok() {
        let mut env = Environment::new();
        let result = build_complexity_env(&mut env);
        assert!(result.is_ok(), "build_complexity_env should succeed");
    }
}
