//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::functions::*;
use super::types::{ContextFreeGrammar, PegExpr};

/// Register all extended parsec axioms into the given environment.
pub fn register_parsec_extended(env: &mut Environment) -> Result<(), String> {
    fn add(env: &mut Environment, name: &str, ty: Expr) -> Result<(), String> {
        env.add(Declaration::Axiom {
            name: Name::str(name),
            univ_params: vec![],
            ty,
        })
        .map_err(|e| e.to_string())
    }
    add(env, "ContextFreeGrammar", prs_ext_cfg_ty())?;
    add(env, "CfgNonterminal", prs_ext_cfg_nonterminal_ty())?;
    add(env, "CfgTerminal", prs_ext_cfg_terminal_ty())?;
    add(env, "CfgProduction", prs_ext_cfg_production_ty())?;
    add(env, "CfgDerivation", prs_ext_cfg_derivation_ty())?;
    add(env, "CfgLanguage", prs_ext_cfg_language_ty())?;
    add(env, "FirstSet", prs_ext_first_set_ty())?;
    add(env, "FollowSet", prs_ext_follow_set_ty())?;
    add(env, "FirstSetCorrectness", prs_ext_first_set_correct_ty())?;
    add(env, "FollowSetCorrectness", prs_ext_follow_set_correct_ty())?;
    add(env, "LlkTable", type1())?;
    add(env, "LlkGrammar", prs_ext_llk_grammar_ty())?;
    add(env, "LlkCorrectness", prs_ext_llk_correct_ty())?;
    add(env, "LlkDeterminism", prs_ext_llk_determinism_ty())?;
    add(env, "EarleyItem", prs_ext_earley_item_ty())?;
    add(env, "EarleyChart", prs_ext_earley_chart_ty())?;
    add(env, "EarleyCompleteness", prs_ext_earley_completeness_ty())?;
    add(env, "EarleySoundness", prs_ext_earley_soundness_ty())?;
    add(env, "CykAlgorithm", prs_ext_cyk_ty())?;
    add(env, "CykCorrectness", prs_ext_cyk_correct_ty())?;
    add(env, "ChomskyNormalForm", prs_ext_cnf_ty())?;
    add(env, "CnfEquivalence", prs_ext_cnf_equiv_ty())?;
    add(env, "PegExpr", prs_ext_peg_expr_ty())?;
    add(env, "PegResult", prs_ext_peg_result_ty())?;
    add(env, "PegSemantics", prs_ext_peg_semantics_ty())?;
    add(env, "PegOrderedChoice", prs_ext_peg_ordered_choice_ty())?;
    add(env, "PegStar", prs_ext_peg_star_ty())?;
    add(env, "PegNot", prs_ext_peg_not_ty())?;
    add(env, "PegDeterminism", prs_ext_peg_determinism_ty())?;
    add(env, "PackratMemo", prs_ext_packrat_memo_ty())?;
    add(env, "PackratParsing", prs_ext_packrat_ty())?;
    add(env, "PackratCorrectness", prs_ext_packrat_correct_ty())?;
    add(env, "PackratComplexity", prs_ext_packrat_linear_ty())?;
    add(env, "LeftFactoring", prs_ext_left_factoring_ty())?;
    add(env, "LeftRecursionElimination", prs_ext_left_rec_elim_ty())?;
    add(env, "GreibachNormalForm", prs_ext_gnf_ty())?;
    add(env, "GreibachEquivalence", prs_ext_gnf_equiv_ty())?;
    add(env, "ParserFunctorLawId", prs_ext_functor_law_id_ty())?;
    add(env, "ParserFunctorCompose", prs_ext_functor_compose_ty())?;
    add(env, "ParserApplicativeLaw", prs_ext_applicative_law_ty())?;
    add(env, "ParserAlternativeLaw", prs_ext_alternative_law_ty())?;
    add(env, "ParserMonadLeftId", prs_ext_monad_left_id_ty())?;
    add(env, "ParserMonadRightId", prs_ext_monad_right_id_ty())?;
    add(env, "ParserMonadAssoc", prs_ext_monad_assoc_ty())?;
    add(env, "ErrorRecovery", prs_ext_error_recovery_ty())?;
    add(env, "PrettyPrint", prs_ext_pretty_print_ty())?;
    add(env, "PrettyPrintRoundtrip", prs_ext_pretty_roundtrip_ty())?;
    add(env, "IncrementalParsing", prs_ext_incremental_parse_ty())?;
    add(env, "TotalParser", prs_ext_total_parser_ty())?;
    add(env, "WellFoundedInput", prs_ext_well_founded_input_ty())?;
    add(env, "TerminationProof", prs_ext_termination_proof_ty())?;
    add(env, "Derivative", prs_ext_derivative_ty())?;
    add(env, "DerivativeSemantics", prs_ext_deriv_semantics_ty())?;
    add(env, "DerivativeCompaction", prs_ext_deriv_compact_ty())?;
    Ok(())
}
