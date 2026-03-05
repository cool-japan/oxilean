//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions_2::{
    normalize_big_prod_sum, normalize_bounded_forall, normalize_bounded_quantifiers,
    normalize_default_binder_values, normalize_exists_quantifier, normalize_exists_unique,
    normalize_fun_bare_binders, normalize_have_in_type, normalize_inline_by,
    normalize_lean_method_names, normalize_subtype_braces, parenthesize_bare_forall_binders,
    parenthesize_dot_exprs, replace_proof_with_sorry, strip_attributes, strip_explicit_at_prefix,
    strip_universe_annotations, strip_where_block,
};
use super::functions_3::{
    normalize_dfinsupp_type, normalize_head_binders, normalize_list_literal_in_type,
    normalize_sigma_in_binders, normalize_subscript_indexing, strip_term_type_ascriptions,
};

/// Normalize Lean 4 surface syntax to OxiLean surface syntax.
///
/// Key differences addressed:
/// - `fun x => body`   → `fun x -> body`    (lambda arrow)
/// - `↦` (U+21A6)     → `->`               (mapsto arrow)
/// - `ℕ` (U+2115)     → `Nat`              (natural number type)
/// - `ℤ` (U+2124)     → `Int`              (integer type)
/// - `ℝ` (U+211D)     → `Real`             (real type)
/// - `ℚ` (U+211A)     → `Rat`              (rational type)
/// - `ℂ` (U+2102)     → `Complex`          (complex type)
/// - `Sort*`/`Type*`  → `Type`             (universe polymorphism)
/// - `@[attr] ...`    → stripped            (attributes)
/// - `_root_.` prefix → stripped            (qualified root namespace)
/// - `:= by ...`      → `:= sorry`          (tactic proofs → sorry)
/// - Dotted decl names `Foo.bar` → `Foo_bar` (dotted names in signature)
/// - `⊆` (U+2286)     → `Subset`            (subset operator)
/// - `∈` (U+2208)     → `Mem`               (membership operator)
/// - `∉` (U+2209)     → `NotMem`            (non-membership)
/// - `∪` (U+222A)     → `Union`             (union operator)
/// - `∩` (U+2229)     → `Inter`             (intersection operator)
/// - `⊂` (U+2282)     → `SSubset`           (strict subset)
pub(super) fn normalize_lean4_to_oxilean(src: &str) -> String {
    let s = strip_attributes(src);
    let s = s.replace("_root_.", "");
    let s = s.replace('\u{21A6}', " -> ");
    let s = s.replace("ℝ≥0∞", "ENNReal");
    let s = s.replace("ℝ≥0", "NNReal");
    let s = s.replace('∞', "Infinity");
    let s = s.replace("\u{2115}\u{221E}", "ENat");
    let s = s.replace("\u{2115}+", "PNat");
    let s = s.replace('\u{2115}', "Nat");
    let s = s.replace('\u{2124}', "Int");
    let s = s.replace('\u{211D}', "Real");
    let s = s.replace('\u{211A}', "Rat");
    let s = s.replace('\u{2102}', "Complex");
    let s = s.replace("\u{2192}\u{2099}\u{2090}", " ->na ");
    let s = s.replace("\u{2192}\u{2099}+*", " ->nrh ");
    let s = s.replace("\u{2192}\u{2099}*", " ->nm ");
    let s = s.replace("\u{2192}\u{2099}+", " ->nah ");
    let s = s.replace("\u{2192}+*", " -> ");
    let s = s.replace("\u{2192}\u{2099}\u{2090}", " -> ");
    let s = s.replace("\u{2192}\u{2099}", " -> ");
    let s = s.replace("\u{2192}*", " -> ");
    let s = s.replace("\u{2192}+", " -> ");
    let s = s.replace('\u{2192}', " -> ");
    let s = s.replace('\u{2286}', " Subset ");
    let s = s.replace('\u{2287}', " Superset ");
    let s = s.replace('\u{2282}', " SSubset ");
    let s = s.replace('\u{2283}', " SSuperset ");
    let s = s.replace('\u{2208}', " Mem ");
    let s = s.replace('\u{2209}', " NotMem ");
    let s = s.replace('\u{220B}', " Contains ");
    let s = s.replace('\u{222A}', " Union ");
    let s = s.replace('\u{2229}', " Inter ");
    let s = s.replace('\u{2205}', "empty_set");
    let s = s.replace('\u{2294}', " Sup ");
    let s = s.replace('\u{2293}', " Inf ");
    let s = s.replace('\u{22A4}', "Top");
    let s = s.replace('\u{22A5}', "Bot");
    let s = s.replace('\u{2A06}', " ISup ");
    let s = s.replace('\u{2A05}', " IInf ");
    let s = normalize_finsum_finprod(&s);
    let s = s.replace('\u{220F}', " BigProd ");
    let s = s.replace('\u{2211}', " BigSum ");
    let s = s.replace("\u{2218}r", " RelComp ");
    let s = s.replace('\u{2218}', " Compose ");
    let s = s.replace('\u{2022}', " SMul ");
    let s = s.replace('\u{22C5}', " SMul ");
    let s = s.replace("\u{2295}'", " PSum ");
    let s = s.replace('\u{2295}', " DirectSum ");
    let s = s.replace('\u{2297}', " TensorProd ");
    let s = s.replace('\u{2223}', " Dvd ");
    let s = s.replace('\u{2224}', " NotDvd ");
    let s = s.replace('\u{29F8}', " Quotient ");
    let s = s.replace('\u{22D6}', " Covers ");
    let s = s.replace('\u{227A}', " StrongLT ");
    let s = s.replace('\u{22C2}', " BigInter ");
    let s = s.replace("Π₀", " DFinsupp ");
    let s = s.replace("\u{22C3}\u{2080}", " sUnion ");
    let s = s.replace('\u{22C3}', " BigUnion ");
    let s = s.replace('\u{2206}', " SymmDiff ");
    let s = s.replace('\u{21E8}', " Himp ");
    let s = s.replace('⨯', " CrossProd ");
    let s = s.replace('⬝', " DotProd ");
    let s = s.replace('ᵥ', "v");
    let s = s.replace('≀', " WreathProd ");
    let s = s.replace('ᵣ', "r");
    let s = s.replace('≪', " AbsCont ");
    let s = s.replace('≫', " MuchGreater ");
    let s = s.replace('⊑', " IsContained ");
    let s = s.replace('⊒', " Contains2 ");
    let s = s.replace('⊴', " NormalSubgroupLE ");
    let s = s.replace('⊵', " NormalSubgroupGE ");
    let s = s.replace('⊲', " NormalSubgroupLT ");
    let s = s.replace('⊳', " NormalSubgroupGT ");
    let s = s.replace('⋊', " SemiDirProd ");
    let s = s.replace('⋉', " SemiDirProdL ");
    let s = s.replace("^<", " CardPowerlt ");
    let s = s.replace("⟮<", "_lt_interval_");
    let s = s.replace("⟮≤", "_le_interval_");
    let s = s.replace('⟮', "_interval_");
    let s = s.replace('⟯', "_end");
    let s = normalize_guillemet_names(&s);
    let s = s.replace('𝓟', " Principal ");
    let s = normalize_expect_notation(&s);
    let s = normalize_dot_anonymous_fn(&s);
    let s = normalize_star_type_suffix(&s);
    let s = s.replace("\u{2200}*", "\u{2200}");
    let s = s.replace("\u{2203}*", "\u{2203}");
    let s = normalize_filter_quantifiers(&s);
    let s = s.replace('\u{1DA0}', "f");
    let s = s.replace('\u{1D1F}', "f");
    let s = s.replace('ᵐ', "m");
    let s = s.replace('ᵃ', "a");
    let s = s.replace('ᵏ', "k");
    let s = s.replace('ˢ', "s");
    let s = s.replace('ˡ', "l");
    let s = s.replace('ⁱ', "i");
    let s = s.replace('∂', " partial_d ");
    let s = normalize_filter_quantifier_body(&s, "∀m", "forall");
    let s = normalize_filter_quantifier_body(&s, "∃m", "exists");
    let s = s.replace("forall*", "forall");
    let s = s.replace("exists*", "exists");
    let s = normalize_norm_notation(&s);
    let s = s.replace('₆', "6");
    let s = s.replace('₇', "7");
    let s = s.replace('₈', "8");
    let s = s.replace('₉', "9");
    let s = s.replace('†', " Adjoint ");
    let s = s.replace('\u{21D1}', " CoeFun ");
    let s = s.replace('\u{21AA}', " Embedding ");
    let s = s.replace('\u{2308}', " ceil_left ");
    let s = s.replace('\u{2309}', " ceil_right ");
    let s = s.replace('\u{230A}', " floor_left ");
    let s = s.replace('\u{230B}', " floor_right ");
    let s = s.replace('\u{208A}', "_plus");
    let s = s.replace('\u{208B}', "_minus");
    let s = s.replace('\u{27E6}', "(");
    let s = s.replace('\u{27E7}', ")");
    let s = s.replace('\u{2097}', "l");
    let s = s.replace("::\u{2098}", " MultisetCons ");
    let s = s.replace('\u{2098}', "m");
    let s = s.replace('\u{2099}', "n");
    let s = s.replace('\u{2080}', "0");
    let s = s.replace('\u{2081}', "1");
    let s = s.replace('\u{2082}', "2");
    let s = s.replace('\u{2083}', "3");
    let s = s.replace('\u{2084}', "4");
    let s = s.replace('\u{2085}', "5");
    let s = s.replace(" :: ", " Cons ");
    let s = s.replace(" ++ ", " Append ");
    let s = s.replace(" ~ ", " Perm ");
    let s = s.replace(" \\ ", " SDiff ");
    let s = s.replace('\u{2245}', " Congr ");
    let s = s.replace("≃+*o", " RingOrderIso ");
    let s = s.replace("≃+*", " RingEquiv ");
    let s = s.replace("≃+", " AddEquiv ");
    let s = s.replace("≃*", " MulEquiv ");
    let s = s.replace("≃o", " OrderIso ");
    let s = s.replace("≃ₗ", " LinearEquiv ");
    let s = s.replace('\u{2243}', " Equiv ");
    let s = s.replace('\u{2248}', " Approx ");
    let s = s.replace('\u{2241}', " NotEquiv ");
    let s = s.replace('\u{2261}', " Equiv3 ");
    let s = s.replace('\u{207A}', "_succ");
    let s = s.replace("\u{207B}\u{00B9}\u{0027}", " Preimage ");
    let s = s.replace(" \u{0027}\u{0027} ", " Image ");
    let s = s.replace("\u{207B}\u{00B9}", " Inv ");
    let s = s.replace("<$>", " FMap ");
    let s = s.replace("<*>", " SeqApply ");
    let s = s.replace("<|>", " OrElse ");
    let s = s.replace("\u{22C6}", " Star ");
    let s = s.replace("\u{2135}", "Aleph");
    let s = s.replace('\u{215F}', " InvOf ");
    let s = s.replace('\u{2190}', " <- ");
    let s = s.replace('\u{22A2}', " Entails ");
    let s = s.replace('\u{25B8}', " Subst ");
    let s = s.replace('\u{2217}', " * ");
    let s = s.replace(">>>", " ShiftRight ");
    let s = s.replace("<<<", " ShiftLeft ");
    let s = s.replace("&&&", " BitAnd ");
    let s = s.replace("|||", " BitOr ");
    let s = s.replace("^^^", " Xor ");
    let s = s.replace("<+:", " IsPrefix ");
    let s = strip_mod_bracket_notation(&s);
    let s = normalize_abs_val_notation(&s);
    let s = s.replace('\u{2308}', "(ceil ");
    let s = s.replace('\u{2309}', ")");
    let s = s.replace('\u{230A}', "(floor ");
    let s = s.replace('\u{230B}', ")");
    let s = s.replace('\u{03A0}', "forall ");
    let s = s.replace("\u{03A3}'", "PSigma ");
    let s = s.replace('\u{03A3}', "Sigma ");
    let s = s.replace('\u{224D}', " HEq ");
    let s = s.replace('\u{2260}', " != ");
    let s = s.replace('\u{1D9C}', " Compl");
    let s = s.replace('\u{207F}', "_n");
    let s = s.replace('\u{2070}', "0");
    let s = s.replace('\u{00D7}', " Prod ");
    let s = s.replace('\u{2983}', "{");
    let s = s.replace('\u{2984}', "}");
    let s = s.replace('\u{2985}', "(");
    let s = s.replace('\u{2986}', ")");
    let s = s.replace('\u{27E6}', "( ");
    let s = s.replace('\u{27E7}', " )");
    let s = normalize_subtype_sets(&s);
    let s = s.replace(" // ", " Subtype ");
    let s = s.replace(" <| ", " ");
    let s = s.replace(" |> ", " ");
    let s = s.replace("|>.", " ");
    let s = s.replace(" /. ", " RatDiv ");
    let s = s.replace(" $ ", " ");
    let s = s.replace("![]", "VectorNil");
    let s = s.replace("![", "MatVec [");
    let s = s.replace("#{", "CardSet { ");
    let s = s.replace("(#(", "((");
    let s = s.replace("(#", "(");
    let s = s.replace(" #", " Card ");
    let s = s.replace("\u{2135}\u{2080}", "AlephNull");
    let s = s.replace('\u{2135}', "Aleph");
    let s = s.replace("<*>", " ApplySeq ");
    let s = s.replace('\u{22C6}', " StarMul ");
    let s = strip_explicit_at_prefix(&s);
    let s = s.replace("Sort*", "Type");
    let s = s.replace("Type*", "Type");
    let s = s.replace("Sort _", "Type");
    let s = strip_universe_annotations(&s);
    let s = s.replace(" => ", " -> ");
    let s = s.replace("=>\n", "->\n");
    let s = strip_where_block(&s);
    let s = normalize_have_in_type(&s);
    let s = replace_proof_with_sorry(&s);
    let s = normalize_lean_method_names(&s);
    let s = normalize_inline_by(&s);
    let s = s.replace("(.refl ", "(refl ");
    let s = s.replace("(.rfl)", "(rfl)");
    let s = s.replace("(.mk ", "(mk ");
    let s = s.replace("(.intro ", "(intro ");
    let s = s.replace(" .refl _", " refl Underscore");
    let s = s.replace(" .refl ", " refl ");
    let s = s.replace("= .refl _", "= refl Underscore");
    let s = s.replace(" .den", " den");
    let s = s.replace(" .num", " num");
    let s = s.replace("= .cast ", "= cast ");
    let s = s.replace("= .inl ", "= inl ");
    let s = s.replace("= .inr ", "= inr ");
    let s = s.replace("= .symm ", "= symm ");
    let s = s.replace("= .none", "= none");
    let s = s.replace("= .some ", "= some ");
    let s = s.replace(" .cast ", " cast ");
    let s = s.replace(" .symm ", " symm ");
    let s = s.replace(" .inl ", " inl ");
    let s = s.replace("(.inl ", "(inl ");
    let s = s.replace(" .inr ", " inr ");
    let s = s.replace("(.inr ", "(inr ");
    let s = normalize_anon_dot_constructors(&s);
    let s = s.replace("#check ", "-- #check ");
    let s = s.replace("#eval ", "-- #eval ");
    let s = normalize_numeric_field_access(&s);
    let s = s.replace(" <+ ", " Sublist ");
    let s = s.replace("<+:", " IsPrefix ");
    let s = s.replace('\u{00B7}', " dot ");
    let s = s.replace("(_ :", "(hole_0 :");
    let s = s.replace(" // ", " suchThat ");
    let s = normalize_set_literals(&s);
    let s = s.replace('\u{2039}', "( ");
    let s = s.replace('\u{203A}', " )");
    let s = s.replace('\u{21A5}', " coe_subtype ");
    let s = s.replace('\u{2191}', " coe ");
    let s = s.replace('\u{2193}', " coe_down ");
    let s = normalize_bounded_quantifiers(&s);
    let s = normalize_bounded_forall(&s);
    let s = normalize_finset_card_notation(&s);
    let s = normalize_set_builder_notation(&s);
    let s = normalize_with_filter(&s);
    let s = normalize_double_factorial(&s);
    let s = normalize_postfix_factorial(&s);
    let s = normalize_big_prod_sum(&s);
    let s = normalize_big_prod_sum(&s);
    let s = normalize_bare_big_quantifiers(&s);
    let s = normalize_big_prod_sum(&s);
    let s = normalize_bare_big_quantifiers(&s);
    let s = normalize_subscript_indexing(&s);
    let s = normalize_list_literal_in_type(&s);
    let s = normalize_subtype_braces(&s);
    let s = normalize_default_binder_values(&s);
    let s = normalize_fun_bare_binders(&s);
    let s = normalize_dfinsupp_type(&s);
    let s = normalize_head_binders(&s);
    let s = strip_term_type_ascriptions(&s);
    let s = normalize_singleton_sets(&s);
    let s = normalize_exists_unique(&s);
    let s = normalize_exists_quantifier(&s);
    let s = parenthesize_bare_forall_binders(&s);
    let s = normalize_sigma_in_binders(&s);
    parenthesize_dot_exprs(&s)
}
/// Normalize Lean 4 subtype set notation `{ ident : T // P }` or `{ ident // P }`.
///
/// Lean 4: `{ x : α // P x }` is a subtype (Sigma-type). OxiLean has no `//` operator.
/// This converts the `{ ... // ... }` braces to `(Subtype T (fun ident -> P))`.
///
/// Must run BEFORE the `//` → `Subtype` replacement so the full `{... // ...}` is intact.
/// Normalize `n‼` (U+203C double exclamation, double factorial) to `(DoubleFactorial n)`.
#[allow(dead_code)]
fn normalize_double_factorial(src: &str) -> String {
    if !src.contains('‼') {
        return src.to_string();
    }
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut result = String::with_capacity(src.len() + 32);
    let mut i = 0;
    while i < len {
        if chars[i] == '‼' {
            let res_trimmed = result.trim_end().to_string();
            let last_space = res_trimmed
                .rfind([' ', '(', ','])
                .map(|p| p + 1)
                .unwrap_or(0);
            let last_tok = res_trimmed[last_space..].to_string();
            let prefix = res_trimmed[..last_space].to_string();
            result = format!("{prefix}(DoubleFactorial {last_tok})");
            i += 1;
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}
/// Normalize guillemet-quoted names `«forall»` → `reserved_forall`.
fn normalize_guillemet_names(src: &str) -> String {
    let mut result = String::with_capacity(src.len());
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut i = 0;
    while i < len {
        if chars[i] == '\u{AB}' {
            let mut name = String::new();
            i += 1;
            while i < len && chars[i] != '\u{BB}' {
                name.push(chars[i]);
                i += 1;
            }
            if i < len {
                i += 1;
            }
            match name.as_str() {
                "forall" => result.push_str("reserved_forall"),
                "exists" => result.push_str("reserved_exists"),
                "fun" => result.push_str("reserved_fun"),
                "let" => result.push_str("reserved_let"),
                "in" => result.push_str("reserved_in"),
                "do" => result.push_str("reserved_do"),
                _ => result.push_str(&name.replace(' ', "_")),
            }
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}
/// Normalize `𝔼 i ∈ s, f i` and `𝔼 i, f i` expectation notation.
/// Treats 𝔼 (expectation) like ∑ (sum) so normalize_big_prod_sum handles the quantifier.
fn normalize_expect_notation(src: &str) -> String {
    src.replace('𝔼', " BigSum ")
}
/// Normalize `β*` Ultrafilter coercion type notation: `word*` → `word_Star`.
/// Only replaces `*` that immediately follows an alphanumeric/underscore character
/// with no space, and where the next character is whitespace, `)`, `,`, `-`, or EOL.
/// Excludes `Sort*` and `Type*` which are handled separately by the universe pipeline.
fn normalize_star_type_suffix(src: &str) -> String {
    let mut result = String::with_capacity(src.len() + 8);
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut i = 0;
    while i < len {
        let ch = chars[i];
        if ch == '*' && i > 0 {
            let prev = chars[i - 1];
            if prev.is_alphanumeric() || prev == '_' || prev == '\'' {
                let next = if i + 1 < len { chars[i + 1] } else { '\0' };
                if next.is_whitespace() || next == ')' || next == ',' || next == '-' || next == '\0'
                {
                    let word_end = i;
                    let word_start = {
                        let mut s = i;
                        while s > 0 {
                            let c = chars[s - 1];
                            if c.is_alphanumeric() || c == '_' || c == '\'' {
                                s -= 1;
                            } else {
                                break;
                            }
                        }
                        s
                    };
                    let word: String = chars[word_start..word_end].iter().collect();
                    if word == "Sort" || word == "Type" {
                        result.push(ch);
                        i += 1;
                        continue;
                    }
                    result.push_str("_Star");
                    i += 1;
                    continue;
                }
            }
        }
        result.push(ch);
        i += 1;
    }
    result
}
/// Normalize `(· OP ·)` anonymous function notation (Lean 4 dot notation).
/// `(· < ·)` → `(fun x y -> x < y)`, `(· + ·)` → `(fun x y -> x + y)`, etc.
fn normalize_dot_anonymous_fn(src: &str) -> String {
    let ops = ["<", "≤", ">", "≥", "+", "-", "*", "=", "≠", "∧", "∨", "/"];
    let mut s = src.to_string();
    for op in &ops {
        let pattern = format!("(· {} ·)", op);
        let replacement = format!("(fun x y -> x {} y)", op);
        s = s.replace(&pattern, &replacement);
    }
    s
}
/// Normalize `∀ᶠ`/`∃ᶠ` filter quantifiers to parseable forms.
/// `∀ᶠ x in f, P x` → `forall (x : _), P x` (drops the filter argument).
fn normalize_filter_quantifiers(src: &str) -> String {
    let s = normalize_filter_quantifier_body(src, "∀ᶠ", "forall");
    normalize_filter_quantifier_body(&s, "∃ᶠ", "exists")
}
/// Helper: normalize `Qᶠ x in f, body` → `keyword (x : _), body`.
/// Also handles `∀m (x) (y) partial_d μ, body` (ae-quantifier with measure).
/// Processes ALL occurrences in the string (not just the first).
#[allow(dead_code)]
fn normalize_filter_quantifier_body(src: &str, quantifier: &str, keyword: &str) -> String {
    if !src.contains(quantifier) {
        return src.to_string();
    }
    let mut result = String::with_capacity(src.len());
    let mut rest = src;
    while let Some(pos) = rest.find(quantifier) {
        result.push_str(&rest[..pos]);
        let after_q = &rest[pos + quantifier.len()..];
        let sep_info = after_q
            .find(" in ")
            .map(|p| (p, 4usize))
            .or_else(|| after_q.find(" partial_d ").map(|p| (p, 11usize)));
        if let Some((sep_pos, sep_len)) = sep_info {
            let after_sep = &after_q[sep_pos + sep_len..];
            let mut depth = 0usize;
            let mut comma_pos = None;
            for (ci, ch) in after_sep.char_indices() {
                match ch {
                    '(' | '[' | '{' => depth += 1,
                    ')' | ']' | '}' => depth = depth.saturating_sub(1),
                    ',' if depth == 0 => {
                        comma_pos = Some(ci);
                        break;
                    }
                    _ => {}
                }
            }
            if let Some(cp) = comma_pos {
                let var_part = after_q[..sep_pos].trim();
                let body = &after_sep[cp + 1..];
                let binders = normalize_filter_var_part(var_part);
                result.push_str(keyword);
                result.push(' ');
                result.push_str(&binders);
                result.push(',');
                rest = body;
                continue;
            }
        }
        result.push_str(keyword);
        result.push_str("_filter ");
        rest = after_q;
    }
    result.push_str(rest);
    result
}
/// Convert a filter quantifier variable part to forall binders.
/// `"a"` → `"(a : _)"`, `"(x) (y)"` → `"(x : _) (y : _)"`, `"(x : T)"` → `"(x : T)"`.
fn normalize_filter_var_part(vars: &str) -> String {
    let mut result = String::new();
    let mut rest = vars.trim();
    while !rest.is_empty() {
        if rest.starts_with('(') {
            let mut depth = 0usize;
            let mut end = 0;
            for (i, c) in rest.char_indices() {
                match c {
                    '(' | '[' | '{' => depth += 1,
                    ')' | ']' | '}' => {
                        depth = depth.saturating_sub(1);
                        if depth == 0 {
                            end = i + 1;
                            break;
                        }
                    }
                    _ => {}
                }
            }
            let inner = &rest[1..end.saturating_sub(1)];
            if !result.is_empty() {
                result.push(' ');
            }
            if inner.contains(':') {
                result.push('(');
                result.push_str(inner);
                result.push(')');
            } else {
                result.push('(');
                result.push_str(inner.trim());
                result.push_str(" : _)");
            }
            rest = rest[end..].trim_start();
        } else {
            let end = rest
                .find(|c: char| c.is_whitespace() || c == '(')
                .unwrap_or(rest.len());
            let ident = &rest[..end];
            if !ident.is_empty() {
                if !result.is_empty() {
                    result.push(' ');
                }
                result.push('(');
                result.push_str(ident);
                result.push_str(" : _)");
            }
            rest = rest[end..].trim_start();
        }
    }
    if result.is_empty() {
        "(_ : _)".to_string()
    } else {
        result
    }
}
/// Normalize `∑ᶠ`/`∏ᶠ` finsum/finprod notation.
fn normalize_finsum_finprod(src: &str) -> String {
    let s = src.replace("\u{2211}\u{1DA0}", " BigSum ");
    let s = s.replace("\u{220F}\u{1DA0}", " BigProd ");
    let s = s.replace("\u{2211}\u{1D1F}", " BigSum ");
    s.replace("\u{220F}\u{1D1F}", " BigProd ")
}
/// Normalize bare BigProd/BigSum/ISup/IInf quantifiers without Mem bound.
/// `BigProd var, body` → `(fun var -> body)` (drop the operator, extract lambda)
/// Handles: `∏ j, removeNth i f j` → `(fun j -> removeNth i f j)`
fn normalize_bare_big_quantifiers(src: &str) -> String {
    let mut result = src.to_string();
    for op in &["BigProd", "BigSum"] {
        result = normalize_bare_q_op(&result, op);
    }
    result
}
#[allow(clippy::too_many_arguments)]
fn normalize_bare_q_op(src: &str, op: &str) -> String {
    if !src.contains(op) {
        return src.to_string();
    }
    let mut result = String::with_capacity(src.len() + 32);
    let mut rest = src;
    let pat = format!("{op} ");
    while let Some(pos) = rest.find(pat.as_str()) {
        let before = &rest[..pos];
        let after = &rest[pos + pat.len()..];
        let after_trim = after.trim_start();
        if after_trim.starts_with("(fun ") {
            let mut depth = 0usize;
            let mut end = 0;
            for (idx, ch) in after_trim.char_indices() {
                match ch {
                    '(' | '[' | '{' => depth += 1,
                    ')' | ']' | '}' => {
                        depth = depth.saturating_sub(1);
                        if depth == 0 {
                            end = idx + 1;
                            break;
                        }
                    }
                    _ => {}
                }
            }
            if end > 0 {
                result.push_str(before);
                result.push_str(&after_trim[..end]);
                let skip = after.len() - after_trim.len();
                rest = &after[skip + end..];
                continue;
            }
        }
        if after_trim.starts_with('(') {
            result.push_str(before);
            result.push_str(pat.as_str());
            rest = after;
            continue;
        }
        let mut depth = 0usize;
        let mut comma_pos = None;
        let mut has_mem = false;
        for (i, ch) in after.char_indices() {
            match ch {
                '(' | '[' | '{' => depth += 1,
                ')' | ']' | '}' => depth = depth.saturating_sub(1),
                ',' if depth == 0 => {
                    comma_pos = Some(i);
                    break;
                }
                _ => {
                    if depth == 0 && after[i..].starts_with("Mem ") {
                        has_mem = true;
                    }
                }
            }
        }
        if let Some(cp) = comma_pos {
            if !has_mem {
                let var_part = after[..cp].trim();
                let body = &after[cp + 1..];
                let var_name = if let Some(colon_pos) = var_part.find(':') {
                    var_part[..colon_pos].trim()
                } else {
                    var_part
                };
                let var_name = if var_name.is_empty() || var_name == "_" {
                    "x"
                } else {
                    var_name
                };
                if var_part.contains(' ') || var_part.contains('(') || var_part.contains(')') {
                    result.push_str(before);
                    result.push_str(pat.as_str());
                    rest = after;
                    continue;
                }
                result.push_str(before);
                result.push_str("fun ");
                result.push_str(var_name);
                result.push_str(" ->");
                result.push_str(body);
                rest = "";
                break;
            }
        }
        result.push_str(before);
        result.push_str(pat.as_str());
        rest = after;
    }
    result.push_str(rest);
    result
}
/// Normalize `‖expr‖` norm notation (U+2016 double vertical line) to `(Norm expr)`.
fn normalize_norm_notation(src: &str) -> String {
    let mut result = String::with_capacity(src.len() + 16);
    let mut in_norm = false;
    for ch in src.chars() {
        if ch == '\u{2016}' {
            if !in_norm {
                result.push_str("(Norm ");
                in_norm = true;
            } else {
                result.push(')');
                in_norm = false;
            }
        } else {
            result.push(ch);
        }
    }
    if in_norm {
        result.push(')');
    }
    result
}
/// Normalize anonymous dot-constructor syntax (`.foo x`) by stripping the leading dot.
/// In Lean 4, `.foo x` means "constructor foo from the inferred namespace."
/// We strip the `.` when it follows whitespace/punctuation before a word character.
fn normalize_anon_dot_constructors(src: &str) -> String {
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut result = String::with_capacity(src.len());
    let mut i = 0;
    while i < len {
        if chars[i] == '.'
            && i > 0
            && matches!(chars[i - 1], ' ' | '(' | '[' | ',' | '=')
            && i + 1 < len
            && chars[i + 1].is_alphabetic()
        {
            i += 1;
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}
fn normalize_subtype_sets(src: &str) -> String {
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut result = String::with_capacity(src.len() + 32);
    let mut i = 0;
    while i < len {
        if chars[i] == '{' {
            let brace_start = i;
            let mut j = i + 1;
            let mut depth = 0usize;
            let mut slash_slash_pos: Option<usize> = None;
            let mut brace_end_pos: Option<usize> = None;
            while j < len {
                match chars[j] {
                    '{' | '(' | '[' => {
                        depth += 1;
                        j += 1;
                    }
                    ')' | ']' => {
                        depth = depth.saturating_sub(1);
                        j += 1;
                    }
                    '}' if depth == 0 => {
                        brace_end_pos = Some(j);
                        break;
                    }
                    '}' => {
                        depth = depth.saturating_sub(1);
                        j += 1;
                    }
                    '/' if depth == 0 && j + 1 < len && chars[j + 1] == '/' => {
                        slash_slash_pos = Some(j);
                        j += 2;
                    }
                    _ => {
                        j += 1;
                    }
                }
            }
            if let (Some(slash_pos), Some(end_pos)) = (slash_slash_pos, brace_end_pos) {
                let inner: String = chars[i + 1..slash_pos].iter().collect();
                let inner = inner.trim();
                let pred: String = chars[slash_pos + 2..end_pos].iter().collect();
                let pred = pred.trim().to_string();
                let (var_name, type_part) = if let Some(colon_pos) = inner.find(" : ") {
                    let var = inner[..colon_pos].trim().to_string();
                    let ty = inner[colon_pos + 3..].trim().to_string();
                    (var, Some(ty))
                } else {
                    (inner.trim().to_string(), None)
                };
                if !var_name.is_empty() && !var_name.contains('{') {
                    if let Some(ty) = type_part {
                        result
                            .push_str(&format!("(Subtype {} (fun {} -> {}))", ty, var_name, pred));
                    } else {
                        result.push_str(&format!("(Subtype (fun {} -> {}))", var_name, pred));
                    }
                    i = end_pos + 1;
                    continue;
                }
            }
            let _ = brace_start;
            result.push(chars[i]);
            i += 1;
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}
/// Strip modular arithmetic bracket notation: `[ZMOD n]`, `[MOD n]`, `[PMOD n]`.
///
/// Lean 4: `a ≡ b [MOD n]` uses bracket notation for modular congruence.
/// OxiLean has no such syntax — strip the brackets entirely.
fn strip_mod_bracket_notation(s: &str) -> String {
    let mut result = s.to_string();
    for prefix in &["[ZMOD", "[MOD", "[PMOD", "[NMOD"] {
        while let Some(start) = result.find(prefix) {
            if let Some(end_offset) = result[start..].find(']') {
                let end = start + end_offset + 1;
                let before = result[..start].trim_end().to_string();
                let after = result[end..].trim_start().to_string();
                result = format!("{} {}", before, after);
            } else {
                break;
            }
        }
    }
    result
}
/// Normalize `|expr|` absolute value bars in type positions → `(AbsVal expr)`.
///
/// Lean 4: `|a|` means absolute value. OxiLean has no `|x|` syntax.
/// By the time this runs, proof bodies have been replaced with `sorry`,
/// so remaining `|` chars are almost exclusively in type positions.
fn normalize_abs_val_notation(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    let mut result = String::with_capacity(s.len() + 16);
    let mut i = 0;
    while i < len {
        if chars[i] == '|' {
            let mut j = i + 1;
            let mut depth: usize = 0;
            let mut found_close = None;
            while j < len {
                match chars[j] {
                    '(' | '[' | '{' => {
                        depth += 1;
                        j += 1;
                    }
                    ')' | ']' | '}' => {
                        if depth > 0 {
                            depth -= 1;
                            j += 1;
                        } else {
                            break;
                        }
                    }
                    '|' if depth == 0 => {
                        found_close = Some(j);
                        break;
                    }
                    '\n' | ',' | ';' => break,
                    ':' if depth == 0 => break,
                    _ => {
                        j += 1;
                    }
                }
            }
            if let Some(close) = found_close {
                let inner: String = chars[i + 1..close].iter().collect();
                let inner_trimmed = inner.trim();
                if !inner_trimmed.is_empty() && !inner_trimmed.starts_with(' ') {
                    result.push_str(&format!("(AbsVal {})", inner_trimmed));
                    i = close + 1;
                    continue;
                }
            }
            result.push('|');
            i += 1;
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}
/// Normalize `{ x | P x }` set-builder notation → `(SetOf (fun x -> P x))`.
///
/// Lean 4: `{ x : T | P x }` is set comprehension (OxiLean uses `SetOf`).
/// Distinguishes set-builder (`{ x | P }`) from set-literals (`{a, b, c}`).
fn normalize_set_builder_notation(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    let mut result = String::with_capacity(s.len() + 32);
    let mut i = 0;
    while i < len {
        if chars[i] == '{' {
            let mut depth: usize = 0;
            let mut pipe_pos: Option<usize> = None;
            let mut brace_end: Option<usize> = None;
            let mut has_comma = false;
            let mut quantifier_depth: usize = 0;
            let mut k = i + 1;
            loop {
                if k >= len {
                    break;
                }
                if depth == 0 {
                    let is_forall_char = chars[k] == '\u{2200}';
                    let is_exists_char = chars[k] == '\u{2203}';
                    let is_forall_kw = k + 6 <= len
                        && chars[k..k + 6].iter().collect::<String>() == "forall"
                        && (k + 6 >= len
                            || (!chars[k + 6].is_alphanumeric() && chars[k + 6] != '_'));
                    let is_exists_kw = k + 6 <= len
                        && chars[k..k + 6].iter().collect::<String>() == "exists"
                        && (k + 6 >= len
                            || (!chars[k + 6].is_alphanumeric() && chars[k + 6] != '_'));
                    if is_forall_char || is_exists_char {
                        quantifier_depth += 1;
                        k += 1;
                        continue;
                    }
                    if is_forall_kw || is_exists_kw {
                        quantifier_depth += 1;
                        k += 6;
                        continue;
                    }
                }
                match chars[k] {
                    '{' | '(' | '[' => {
                        depth += 1;
                        k += 1;
                    }
                    ')' | ']' => {
                        depth = depth.saturating_sub(1);
                        k += 1;
                    }
                    '}' if depth == 0 => {
                        brace_end = Some(k);
                        break;
                    }
                    '}' => {
                        depth = depth.saturating_sub(1);
                        k += 1;
                    }
                    '|' if depth == 0 && pipe_pos.is_none() => {
                        pipe_pos = Some(k);
                        quantifier_depth = 0;
                        k += 1;
                    }
                    ',' if depth == 0 && pipe_pos.is_none() => {
                        if quantifier_depth > 0 {
                            quantifier_depth -= 1;
                            k += 1;
                        } else {
                            has_comma = true;
                            break;
                        }
                    }
                    ',' if depth == 0 => {
                        quantifier_depth = quantifier_depth.saturating_sub(1);
                        k += 1;
                    }
                    _ => {
                        k += 1;
                    }
                }
            }
            if !has_comma {
                if let (Some(pipe), Some(end)) = (pipe_pos, brace_end) {
                    let binder: String = chars[i + 1..pipe].iter().collect();
                    let binder = binder.trim().to_string();
                    let body: String = chars[pipe + 1..end].iter().collect();
                    let body = body.trim().to_string();
                    if !binder.is_empty() && !body.is_empty() {
                        if let Some(mem_pos) = binder.find(" Mem ") {
                            let var = binder[..mem_pos].trim();
                            let collection = binder[mem_pos + 5..].trim();
                            result.push_str(&format!(
                                "(SetOf (fun {} -> {} Mem {} && {}))",
                                var, var, collection, body
                            ));
                        } else if binder.contains(':') {
                            result.push_str(&format!("(SetOf (fun ({}) -> {}))", binder, body));
                        } else {
                            result.push_str(&format!("(SetOf (fun {} -> {}))", binder, body));
                        }
                        i = end + 1;
                        continue;
                    }
                }
            }
            result.push('{');
            i += 1;
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}
/// Normalize `#{...}` and `#[...]` cardinality notation by stripping the `#` prefix.
///
/// Lean 4: `#{a ∈ s | p}` is finset builder+cardinality. We strip `#` to leave `{...}`.
/// The braced expression will then be handled by `normalize_set_literals` or `normalize_singleton_sets`.
fn normalize_finset_card_notation(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    let mut i = 0;
    while i < len {
        if chars[i] == '#' && i + 1 < len && (chars[i + 1] == '{' || chars[i + 1] == '[') {
            i += 1;
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}
/// Normalize `with` filter notation in BigSum/BigProd.
///
/// Lean 4: `∑ m ∈ range n with m ∣ n, body` becomes after earlier normalizations:
/// `BigSum m Mem range n with m Dvd n, body`
/// We strip the ` with <condition>` part (up to the next `,` at depth 0)
/// as an approximation to make the expression parseable.
fn normalize_with_filter(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    let mut i = 0;
    while i < len {
        if i + 6 <= len {
            let word: String = chars[i..i + 6].iter().collect();
            if word == " with " {
                let mut j = i + 6;
                let mut depth: usize = 0;
                while j < len {
                    match chars[j] {
                        '(' | '{' | '[' => {
                            depth += 1;
                            j += 1;
                        }
                        ')' | '}' | ']' if depth > 0 => {
                            depth -= 1;
                            j += 1;
                        }
                        ',' if depth == 0 => {
                            break;
                        }
                        _ => {
                            j += 1;
                        }
                    }
                }
                result.push(' ');
                i = j;
                continue;
            }
        }
        result.push(chars[i]);
        i += 1;
    }
    result
}
/// Normalize postfix factorial `n !` → `(Factorial n)`.
///
/// Lean 4 uses postfix `!` for factorial: `n!` or `n !`.
/// OxiLean parser treats `!` as prefix negation, so convert:
/// `emultiplicity 2 n ! < n` → `emultiplicity 2 (Factorial n) < n`
fn normalize_postfix_factorial(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    let mut result = String::with_capacity(s.len() + 16);
    let mut i = 0;
    while i < len {
        if chars[i] == '!' {
            let next = if i + 1 < len { chars[i + 1] } else { '\0' };
            if next == '=' {
                result.push('!');
                i += 1;
                continue;
            }
            if next.is_alphanumeric() || next == '_' || next == '(' || next == '[' {
                result.push('!');
                i += 1;
                continue;
            }
            let result_chars: Vec<char> = result.chars().collect();
            let mut j = result_chars.len();
            while j > 0 && result_chars[j - 1] == ' ' {
                j -= 1;
            }
            let word_end = j;
            if j > 0 && result_chars[j - 1] == ')' {
                let mut depth = 0usize;
                let mut k = j;
                let mut found_open = false;
                while k > 0 {
                    k -= 1;
                    match result_chars[k] {
                        ')' => depth += 1,
                        '(' => {
                            if depth == 1 {
                                found_open = true;
                                break;
                            }
                            depth -= 1;
                        }
                        _ => {}
                    }
                }
                if found_open {
                    let inner: String = result_chars[k..j].iter().collect();
                    let prefix: String = result_chars[..k].iter().collect();
                    result = prefix;
                    result.push_str(&format!("(Factorial {})", inner));
                    i += 1;
                    continue;
                }
            }
            while j > 0
                && (result_chars[j - 1].is_alphanumeric()
                    || result_chars[j - 1] == '_'
                    || result_chars[j - 1] == '\'')
            {
                j -= 1;
            }
            let word_start = j;
            if word_start < word_end {
                let word: String = result_chars[word_start..word_end].iter().collect();
                let prefix: String = result_chars[..word_start].iter().collect();
                result = prefix;
                result.push_str(&format!("(Factorial {})", word));
            } else {
                result.push('!');
            }
            i += 1;
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}
/// Strip universe-level annotations `.{u, v}` from qualified names.
/// `Foo.{u}` → `Foo`, `Bar.{u, v + 1}` → `Bar`.
/// These appear in Lean 4 type expressions that OxiLean doesn't support.
/// Strip `where` method blocks from declarations.
/// `theorem foo : T where method := body` → `theorem foo : T := sorry`
/// The `where` block in Lean 4 defines methods for structure/class instances,
/// which OxiLean does not support. We strip from ` where ` to end of statement.
/// Strip `haveI := expr;` local instance bindings from type expressions.
///
/// In Lean 4, `(haveI := A; x ≤ y)` is a local instance override syntax.
/// OxiLean does not support this. We strip the `haveI := ...;` part, keeping
/// only the body expression: `(haveI := A; x ≤ y)` → `(x ≤ y)`.
#[allow(dead_code)]
fn strip_have_instances(src: &str) -> String {
    let mut result = String::with_capacity(src.len());
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut i = 0;
    while i < len {
        let is_have_i = i + 5 <= len
            && chars[i..i + 5].iter().collect::<String>() == "haveI"
            && (i + 5 >= len || !chars[i + 5].is_alphanumeric());
        let prev_not_word = i == 0 || !chars[i - 1].is_alphanumeric();
        if is_have_i && prev_not_word {
            i += 5;
            while i < len && chars[i] == ' ' {
                i += 1;
            }
            let saved = i;
            let mut found_assign = false;
            while i < len {
                if chars[i] == ':' && i + 1 < len && chars[i + 1] == '=' {
                    found_assign = true;
                    i += 2;
                    break;
                }
                if chars[i] == ';' || chars[i] == ')' {
                    i = saved;
                    break;
                }
                i += 1;
            }
            if !found_assign {
                result.push_str("haveI");
                continue;
            }
            while i < len && chars[i] == ' ' {
                i += 1;
            }
            let mut depth = 0usize;
            while i < len {
                match chars[i] {
                    '(' | '{' | '[' => {
                        depth += 1;
                        i += 1;
                    }
                    ')' | '}' | ']' if depth == 0 => break,
                    ')' | '}' | ']' => {
                        depth = depth.saturating_sub(1);
                        i += 1;
                    }
                    ';' if depth == 0 => {
                        i += 1;
                        break;
                    }
                    _ => {
                        i += 1;
                    }
                }
            }
            while i < len && chars[i] == ' ' {
                i += 1;
            }
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}
/// Normalize numeric field access `.1` `.2` `.3` → `_fst` `_snd` `_trd`.
///
/// Lean 4 `s.1` accesses the first field of a sigma/tuple type.
/// OxiLean does not support `.digit` field access syntax, so we replace:
/// `ident.1` → `ident_fst`, `ident.2` → `ident_snd`, `ident.3` → `ident_trd`
fn normalize_numeric_field_access(src: &str) -> String {
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut result = String::with_capacity(src.len());
    let mut i = 0;
    while i < len {
        if chars[i] == '.'
            && i > 0
            && (chars[i - 1].is_alphanumeric()
                || chars[i - 1] == '_'
                || chars[i - 1] == '\''
                || chars[i - 1] == ')')
            && i + 1 < len
            && chars[i + 1].is_ascii_digit()
        {
            i += 1;
            let digit_start = i;
            while i < len && chars[i].is_ascii_digit() {
                i += 1;
            }
            let digit: String = chars[digit_start..i].iter().collect();
            let suffix = match digit.as_str() {
                "1" => "_fst",
                "2" => "_snd",
                "3" => "_trd",
                _ => "_nth",
            };
            result.push_str(suffix);
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}
/// Normalize multi-element set/finset literals `{a, b, c}` to function application form.
///
/// In Lean 4, `{false, true}` is a finite set literal.
/// OxiLean parser interprets `{...}` as implicit binders, causing parse failures.
/// We replace `{a, b}` → `(insert a (singleton b))` etc.
/// Only applies to cases where `{` contains commas at depth 0 (multi-element sets).
fn normalize_set_literals(src: &str) -> String {
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut result = String::with_capacity(src.len() + 32);
    let mut i = 0;
    while i < len {
        if chars[i] == '{' {
            let brace_start = i;
            let mut j = i + 1;
            let mut depth = 1usize;
            let mut top_level_commas = 0usize;
            let mut has_colon = false;
            while j < len && depth > 0 {
                match chars[j] {
                    '{' => {
                        depth += 1;
                        j += 1;
                    }
                    '}' => {
                        depth = depth.saturating_sub(1);
                        j += 1;
                    }
                    ':' if depth == 1 => {
                        has_colon = true;
                        j += 1;
                    }
                    ',' if depth == 1 => {
                        top_level_commas += 1;
                        j += 1;
                    }
                    _ => {
                        j += 1;
                    }
                }
            }
            if depth == 0 && top_level_commas > 0 && !has_colon {
                let inner: String = chars[brace_start + 1..j - 1].iter().collect();
                let elements = split_top_level_commas(&inner);
                let nested = build_set_expr(&elements);
                result.push_str(&nested);
                i = j;
            } else {
                let raw: String = chars[brace_start..j].iter().collect();
                result.push_str(&raw);
                i = j;
            }
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}
/// Split a string by top-level commas (not inside brackets).
fn split_top_level_commas(s: &str) -> Vec<String> {
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut depth = 0usize;
    let mut i = 0;
    while i < len {
        match chars[i] {
            '(' | '{' | '[' => {
                depth += 1;
                current.push(chars[i]);
                i += 1;
            }
            ')' | '}' | ']' => {
                depth = depth.saturating_sub(1);
                current.push(chars[i]);
                i += 1;
            }
            ',' if depth == 0 => {
                parts.push(current.trim().to_string());
                current = String::new();
                i += 1;
            }
            _ => {
                current.push(chars[i]);
                i += 1;
            }
        }
    }
    if !current.trim().is_empty() {
        parts.push(current.trim().to_string());
    }
    parts
}
/// Build nested insert/singleton expression from a list of elements.
fn build_set_expr(elements: &[String]) -> String {
    match elements.len() {
        0 => "empty_set".to_string(),
        1 => format!("(singleton {})", elements[0]),
        _ => {
            let rest = build_set_expr(&elements[1..]);
            format!("(insert {} {})", elements[0], rest)
        }
    }
}
/// Replace bare singleton set/multiset `{x}` in type expressions with `(singleton x)`.
///
/// In Lean 4, `{a}` in type position is a singleton multiset/finset.
/// OxiLean parser interprets `{...}` as an implicit binder or block, causing failures.
/// We detect `{ident}` patterns (no `:` inside, single identifier) and replace them.
fn normalize_singleton_sets(src: &str) -> String {
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut result = String::with_capacity(src.len() + 16);
    let mut i = 0;
    while i < len {
        if chars[i] == '{' {
            let preceded_by_quantifier = {
                let trimmed = result.trim_end();
                trimmed.ends_with('\u{2200}')
                    || trimmed.ends_with('\u{2203}')
                    || trimmed.ends_with("forall")
                    || trimmed.ends_with("exists")
            };
            let brace_start = i;
            let mut j = i + 1;
            let mut depth = 1usize;
            let mut paren_depth = 0usize;
            let mut found_colon_direct = false;
            let mut found_space = false;
            while j < len && depth > 0 {
                match chars[j] {
                    '{' => {
                        depth += 1;
                        j += 1;
                    }
                    '}' => {
                        depth = depth.saturating_sub(1);
                        j += 1;
                    }
                    '(' => {
                        paren_depth += 1;
                        j += 1;
                    }
                    ')' => {
                        paren_depth = paren_depth.saturating_sub(1);
                        j += 1;
                    }
                    ':' if paren_depth == 0 && depth == 1 => {
                        found_colon_direct = true;
                        j += 1;
                    }
                    ' ' | '\t' | '\n' => {
                        found_space = true;
                        j += 1;
                    }
                    _ => {
                        j += 1;
                    }
                }
            }
            if depth == 0 {
                let inner: String = chars[brace_start + 1..j - 1].iter().collect();
                let inner_trimmed = inner.trim();
                let has_pipe = inner_trimmed.contains('|');
                let has_comma = inner_trimmed.contains(',');
                let is_singleton =
                    !found_colon_direct && !has_pipe && !has_comma && !inner_trimmed.is_empty();
                let is_simple_ident = is_singleton
                    && !found_space
                    && inner_trimmed.chars().all(|c| {
                        c.is_alphanumeric() || c == '_' || c == '\'' || c == '!' || c == '.'
                    });
                let is_func_app = is_singleton && found_space;
                if preceded_by_quantifier && is_func_app {
                    result.push('(');
                    result.push_str(inner_trimmed);
                    result.push(')');
                } else if is_simple_ident {
                    let singleton_content = if let Some(rest) = inner_trimmed.strip_prefix('!') {
                        format!("(not {rest})")
                    } else {
                        inner_trimmed.to_string()
                    };
                    result.push_str("(singleton ");
                    result.push_str(&singleton_content);
                    result.push(')');
                } else if is_func_app {
                    result.push_str("(singleton (");
                    result.push_str(inner_trimmed);
                    result.push_str("))");
                } else {
                    let raw: String = chars[brace_start..j].iter().collect();
                    result.push_str(&raw);
                }
                i = j;
            } else {
                result.push(chars[i]);
                i += 1;
            }
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}
