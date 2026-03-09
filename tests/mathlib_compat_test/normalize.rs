//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::normalize_2::{
    normalize_big_prod_sum, normalize_bounded_forall, normalize_bounded_quantifiers,
    normalize_default_binder_values, normalize_exists_quantifier, normalize_exists_unique,
    normalize_fun_bare_binders, normalize_have_in_type, normalize_inline_by,
    normalize_lean_method_names, normalize_subtype_braces, parenthesize_bare_forall_binders,
    parenthesize_dot_exprs, replace_proof_with_sorry, strip_attributes, strip_explicit_at_prefix,
    strip_universe_annotations, strip_where_block,
};
use super::normalize_3::{
    normalize_dfinsupp_type, normalize_fun_comma_lambda, normalize_head_binders,
    normalize_list_literal_in_type, normalize_sigma_in_binders, normalize_subscript_indexing,
    strip_term_type_ascriptions, strip_tick_subscript,
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
    // Strip line comments (-- ...) early, before any other normalization
    let s = strip_line_comments(&s);
    let s = normalize_metaprogramming(&s);
    // Replace Unicode prime with ASCII apostrophe (valid in identifiers)
    let s = s.replace('\u{2032}', "'"); // ′ → '
                                        // Greek letters as identifiers
    let s = s.replace('\u{0393}', "Gamma"); // Γ → Gamma
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
    // Exterior algebra arrow notation: [⋀^ι]→ₗ[R] → LinearMap
    let s = normalize_exterior_arrow(&s);
    // Partial function arrow →. must come before generic → replacement
    let s = s.replace("\u{2192}.", " PartialFun ");
    // Arrow+subscript+bracket operators (must come before generic → replacement)
    let s = normalize_arrow_subscript_brackets(&s);
    let s = s.replace("\u{2192}\u{2099}\u{2090}", " ->na ");
    let s = s.replace("\u{2192}\u{2099}+*", " ->nrh ");
    let s = s.replace("\u{2192}\u{2099}*", " ->nm ");
    let s = s.replace("\u{2192}\u{2099}+", " ->nah ");
    let s = s.replace("\u{2192}\u{22C6}\u{2099}+*", " StarNURingHom "); // →⋆ₙ+*
    let s = s.replace("\u{2243}\u{22C6}+*", " StarRingEquiv "); // ≃⋆+*
    let s = s.replace("\u{2192}\u{22C6}+*", " StarRingHom "); // →⋆+*
    let s = s.replace("\u{2243}\u{22C6}", " StarEquiv "); // ≃⋆
    let s = s.replace("\u{2243}.", " PEquiv "); // ≃. partial equiv (before bare ≃)
    let s = s.replace("\u{2192}+*", " -> ");
    let s = s.replace("\u{2192}\u{2099}\u{2090}", " -> ");
    let s = s.replace("\u{2192}\u{2099}", " -> ");
    let s = s.replace("\u{2192}*\u{2080}o", " -> "); // →*₀o ordered monoid-with-zero hom
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
    let s = strip_filtered_tprod_tsum(&s); // ∏'[L] → TProd, ∑'[L] → Tsum (before bare ∏'/∑')
    let s = s.replace("\u{220F}'", " TProd "); // ∏' tprod (infinite product)
    let s = s.replace("\u{2211}'", " Tsum "); // ∑' tsum (infinite sum)
                                              // Strip superscript markers from ∏/∑ before converting to BigProd/BigSum
                                              // ∏ᵖ → BigProd (the ᵖ is a notation marker, not a variable)
    let s = s.replace("\u{220F}\u{1D56}", " BigProd ");
    let s = s.replace("\u{2211}\u{1D56}", " BigSum ");
    let s = s.replace('\u{220F}', " BigProd ");
    let s = s.replace('\u{2211}', " BigSum ");
    let s = s.replace("\u{2218}r", " RelComp ");
    let s = s.replace('\u{2218}', " Compose ");
    let s = s.replace('\u{2022}', " SMul ");
    let s = s.replace('\u{22C5}', " SMul ");
    let s = s.replace('\u{2219}', " Span "); // ∙ bullet operator (span/action)
    let s = s.replace("\u{2295}'", " PSum ");
    let s = s.replace('\u{2044}', "/"); // ⁄ fraction slash → regular slash
    let s = s.replace('\u{2295}', " DirectSum ");
    let s = s.replace('\u{2A01}', " BigDirectSum "); // ⨁ N-ary direct sum
    let s = s.replace("\u{2297}'", " TensorProdMap "); // ⊗' tensor product map (BEFORE bare ⊗)
    let s = s.replace('\u{2297}', " TensorProd ");
    let s = s.replace('\u{2223}', " Dvd ");
    let s = s.replace('\u{2224}', " NotDvd ");
    let s = s.replace('\u{29F8}', " Quotient ");
    let s = s.replace('\u{22D6}', " Covers ");
    let s = s.replace('\u{227A}', " StrongLT ");
    let s = s.replace('\u{227C}', " Preorder "); // ≼ preorder relation
    let s = s.replace('\u{2A3F}', " Coprod2 "); // ⨿ coproduct (alternative)
    let s = s.replace('\u{21BF}', " Uncurry "); // ↿ uncurry operator
    let s = s.replace('\u{22EF}', " Dots "); // ⋯ horizontal ellipsis (midline)
    let s = s.replace('\u{21D2}', " FatArrow "); // ⇒ fat arrow (relator/lifting)
    let s = s.replace("\u{22C2}\u{2080}", " sInter "); // ⋂₀ set intersection
    let s = s.replace('\u{22C2}', " BigInter ");
    let s = s.replace("Π₀", " DFinsupp ");
    let s = s.replace("\u{22C3}\u{2080}", " sUnion ");
    let s = s.replace('\u{22C3}', " BigUnion ");
    let s = s.replace('\u{2206}', " SymmDiff ");
    let s = s.replace('\u{21E8}', " Himp ");
    let s = s.replace('⨯', " CrossProd ");
    let s = s.replace('⬝', " DotProd ");
    let s = s.replace("::\u{1D65}", " VCons "); // ::ᵥ vector cons (BEFORE ᵥ→v)
    let s = s.replace('ᵥ', "v");
    let s = s.replace('≀', " WreathProd ");
    let s = s.replace('ᵣ', "r");
    let s = s.replace('≪', " AbsCont ");
    let s = s.replace('≫', " MuchGreater ");
    // Category Theory operators
    let s = s.replace('\u{27F6}', " -> "); // ⟶ long right arrow (hom morphism)
    let s = s.replace('\u{2964}', " FunctorMap "); // ⥤ functor arrow
    let s = s.replace('\u{22A3}', " LeftAdj "); // ⊣ left adjoint
    let s = s.replace('\u{22D9}', " CompFunctor "); // ⋙ functor composition
    let s = s.replace('\u{1D7D9}', " CatId "); // 𝟙 identity morphism
    let s = s.replace('\u{1D7ED}', " IdFunctor "); // 𝟭 identity functor
    let s = s.replace('\u{25C1}', " WhiskerLeft "); // ◁ left whiskering
    let s = s.replace('\u{25B7}', " WhiskerRight "); // ▷ right whiskering
    let s = s.replace('\u{224C}', " NatIso "); // ≌ natural isomorphism
    let s = s.replace('\u{25EB}', " HComp "); // ◫ horizontal composition (NatTrans)
    let s = s.replace('\u{22BB}', " XOr "); // ⊻ XOR
    let s = s.replace('\u{21D4}', " BiHimp "); // ⇔ biconditional/bihimp
    let s = s.replace('\u{FFE2}', " HNot "); // ￢ fullwidth not sign (Heyting not)
    let s = s.replace('\u{2A7F}', " WCovBy "); // ⩿ weak covering relation
                                               // Analysis / InnerProductSpace
    let s = s.replace('\u{27EA}', "(InnerProd "); // ⟪ inner product open
    let s = s.replace('\u{27EB}', ")"); // ⟫ inner product close
    let s = s.replace('\u{27E8}', "("); // ⟨ anonymous constructor open
    let s = s.replace('\u{27E9}', ")"); // ⟩ anonymous constructor close
    let s = s.replace('\u{15EE}', " OrthCompl "); // ᗮ orthogonal complement
                                                  // Superscript/subscript letters
    let s = s.replace('\u{1D50}', "m"); // ᵐ superscript m (ae-measure notation)
    let s = s.replace('\u{1D52}', "o"); // ᵒ superscript o (opposite category)
    let s = s.replace('\u{1D56}', "p"); // ᵖ superscript p
    let s = s.replace("~\u{1D62}", " Inseparable "); // ~ᵢ inseparable (BEFORE ᵢ→i)
    let s = s.replace('\u{1D62}', "i"); // ᵢ subscript i
                                        // Lie algebra brackets
    let s = normalize_lie_brackets(&s); // ⁅x, y⁆ → (LieBracket x y)
    let s = normalize_jacobi_symbol(&s); // J(a | b) → (JacobiSym a b)
    let s = s.replace("\u{2225}\u{2096}", " ParallelComp "); // ∥ₖ kernel parallel composition
    let s = s.replace('\u{2225}', " Parallel "); // ∥ double vertical line
    let s = s.replace('\u{298B}', "("); // ⦋ left subscript bracket → (
    let s = s.replace('\u{298C}', ")"); // ⦌ right subscript bracket → )
    let s = s.replace('\u{22A0}', " TensorProd2 "); // ⊠ tensor product (categorical)
    let s = s.replace('\u{22C0}', " BigWedge "); // ⋀ big wedge / exterior power
    let s = s.replace('\u{222F}', " SurfaceIntegral "); // ∯ surface integral
    let s = s.replace('\u{22A1}', " BoxDot "); // ⊡ boxed dot operator
    let s = s.replace('\u{291E}', " RightArrow2 "); // ⤞ rightwards two-headed arrow
    let s = s.replace('\u{25B3}', " Triangle "); // △ triangle operator
    let s = s.replace('\u{229E}', " BiProd "); // ⊞ biproduct / boxplus
    let s = s.replace('\u{2210}', " Coprod "); // ∐ coproduct
    let s = s.replace('\u{2220}', " AngleMeasure "); // ∠ angle measure
    let s = s.replace('\u{2221}', " OrientedAngle "); // ∡ oriented angle
                                                      // `^*` is pullback/fixed-points/dual notation in Mathlib
                                                      // e.g., `α^*M` → `α_Star M`, `V^*` → `V_Star`
    let s = s.replace("^*", "_Star ");
    let s = s.replace('\u{203C}', " DoubleFactorial "); // ‼ double factorial
    let s = s.replace('\u{00B2}', "2"); // ² superscript 2 (L² → L2)
    let s = s.replace('\u{00B3}', "3"); // ³ superscript 3
    let s = s.replace('\u{2026}', " Dots2 "); // … horizontal ellipsis
                                              // Misc operators (Phase 2)
    let s = s.replace("~>", " Promises "); // ~> Lean 4 promises/computation arrow
    let s = s.replace('\u{22C8}', " Interleave "); // ⋈ interleave/bowtie/join
    let s = s.replace("++\u{209B}", " StreamAppend "); // ++ₛ stream append (before ++ normalization)
    let s = s.replace('\u{266F}', " sharp "); // ♯ sharp operator
    let s = s.replace('\u{266D}', "_flat"); // ♭ musical flat (e.g. R♭ → R_flat)
    let s = s.replace('\u{22BC}', " Nand "); // ⊼ NAND / lattice inf
    let s = s.replace('\u{290F}', " PartialFun "); // ⤏ partial function arrow
    let s = s.replace('\u{2933}', " Specializes "); // ⤳ specializes arrow
    let s = s.replace('\u{25A1}', " BoxProd "); // □ box product
    let s = s.replace('\u{25CB}', " DisjSups "); // ○ disjoint suprema
    let s = s.replace('\u{25C3}', " ShelfAct "); // ◃ shelf/rack action
    let s = s.replace('\u{2A33}', " Nmul "); // ⨳ smash product / Nmul
    let s = s.replace('\u{2A02}', " BigTensorProd "); // ⨂ big tensor product
    let s = s.replace('\u{27F9}', " LongImpl "); // ⟹ long double arrow (impl)
    let s = s.replace('\u{223C}', " SimRel "); // ∼ similarity relation
    let s = s.replace('\u{22A8}', " Models "); // ⊨ models/satisfies
    let s = s.replace('\u{2198}', " StructMorph "); // ↘ structure morphism (AlgGeom)
    let s = s.replace(">=>", " KleisliFish "); // >=> Kleisli fish operator
    let s = s.replace('\u{2736}', "_dual"); // ✶ matroid dual (no space, becomes M_dual.IsCircuit)
    let s = s.replace('\u{21BE}', " Restrict "); // ↾ restriction
                                                 // Phase 12 fixes
    let s = s.replace('\u{FF0F}', " MatroidContract "); // ／ fullwidth solidus (matroid contract)
    let s = s.replace('\u{FF3C}', " MatroidDelete "); // ＼ fullwidth reverse solidus (matroid delete)
    let s = s.replace('\u{2207}', " Gradient "); // ∇ nabla / gradient
    let s = s.replace('\u{2118}', "WeierstrassP"); // ℘ Weierstrass P function
    let s = s.replace('\u{229B}', " CircledAst "); // ⊛ circled asterisk (Day convolution)
    let s = s.replace('\u{229A}', " CircComp "); // ⊚ circled ring operator (composition)
    let s = s.replace("=\u{1DA0}", " FilterEventualEq "); // =ᶠ filter eventual equality (before ᶜ→Compl)
    let s = normalize_math_flat_parens(&s); // ⟮...⟯ → strip content
    let s = s.replace('\u{29CF}', " GameLF "); // ⧏ game less-fuzzy
    let s = s.replace('\u{29D0}', " GameGF "); // ⧐ game greater-fuzzy
                                               // Strip combining diacritical marks (U+0300-U+036F) — OxiLean lexer can't handle them
    let s = strip_combining_marks(&s);
    // Landau notation: =O[...] → IsBigO, =o[...] → IsLittleO
    let s = normalize_landau_notation(&s);
    let s = s.replace("\u{27C2}\u{2098}", " MutSing "); // ⟂ₘ mutually singular (BEFORE ⟂)
    let s = s.replace('\u{27C2}', " Perp "); // ⟂ perpendicular
                                             // Mathematical alphanumeric symbols
    let s = s.replace('\u{1D4D5}', "FourierF"); // 𝓕 script F (Fourier)
    let s = s.replace('\u{1D4DD}', "Nhds"); // 𝓝 script N (neighborhood)
    let s = s.replace('\u{1D4E2}', "Schwartz"); // 𝓢 script S (Schwartz space)
    let s = s.replace('\u{1D55C}', "K_field"); // 𝕜 blackboard bold k (field)
    let s = s.replace('\u{1D538}', "AffineSpace"); // 𝔸 blackboard bold A (affine)
                                                   // (~ᵢ moved before ᵢ→i to avoid breaking the compound pattern)
    let s = s.replace("~\u{1D64}", " Associated "); // ~ᵤ associated relation
    let s = s.replace('\u{1D64}', "u"); // ᵤ subscript u
                                        // Compound ᵁ patterns before bare ᵁ→U
    let s = s.replace("\u{207B}\u{00B9}\u{1D41}", " UltraPreimage "); // ⁻¹ᵁ
    let s = s.replace("''\u{1D41}", " UltraImage "); // ''ᵁ
    let s = s.replace('\u{1D41}', "U"); // ᵁ superscript U
                                        // Phase 3: New Unicode operators
    let s = s.replace('\u{2197}', " coe_lift "); // ↗ coercion lift (LSeries)
    let s = s.replace("\u{2A0D}\u{207B}", " AvgIntegralInv "); // ⨍⁻ (BEFORE ⨍ and ⁻¹)
    let s = s.replace('\u{2A0D}', " AvgIntegral "); // ⨍ average integral
    let s = s.replace('\u{222E}', " ContourIntegral "); // ∮ contour integral
    let s = s.replace('\u{235F}', " Convolution "); // ⍟ convolution (LSeries)
    let s = s.replace('\u{2E28}', "("); // ⸨ left tortoise shell bracket
    let s = s.replace('\u{2E29}', ")"); // ⸩ right tortoise shell bracket
    let s = s.replace('\u{2299}', " HadamardMul "); // ⊙ Hadamard product
                                                    // Integral notation
    let s = s.replace("\u{222B}\u{1D9C}", " CurveIntegral "); // ∫ᶜ curve integral (BEFORE bare ∫)
    let s = s.replace("\u{222B}\u{207B}", " IntegralInv "); // ∫⁻ lower integral
    let s = s.replace('\u{222B}', " Integral "); // ∫ integral
                                                 // (∑' and ∏' moved before BigSum/BigProd to avoid double-replace)
                                                 // Infix/suffix operators
    let s = s.replace("<:+:", " IsInfix "); // <:+: infix relation
    let s = s.replace("<:+", " IsSuffix "); // <:+ suffix relation
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
    let s = strip_remaining_partial_d(&s);
    let s = s.replace("forall*", "forall");
    let s = s.replace("exists*", "exists");
    let s = normalize_norm_notation(&s);
    let s = s.replace('₆', "6");
    let s = s.replace('₇', "7");
    let s = s.replace('₈', "8");
    let s = s.replace('₉', "9");
    let s = s.replace('†', " Adjoint ");
    let s = s.replace('\u{21D1}', " CoeFun ");
    let s = s.replace("\u{21AA}o", " OrderEmbedding ");
    let s = s.replace('\u{21AA}', " Embedding ");
    let s = s.replace('\u{2308}', " ceil_left ");
    let s = s.replace('\u{2309}', " ceil_right ");
    let s = s.replace('\u{230A}', " floor_left ");
    let s = s.replace('\u{230B}', " floor_right ");
    let s = s.replace('\u{208A}', "_plus");
    let s = s.replace('\u{208B}', "_minus");
    let s = normalize_shift_prime(&s); // f⟦a⟧' → f (strip shift-prime notation)
    let s = s.replace('\u{27E6}', "(");
    let s = s.replace('\u{27E7}', ")");
    // NOTE: ₗ (U+2097) → "l" is deferred until AFTER ≃ₗ and →ₗ handling
    let s = s.replace("::\u{209B}", " SymCons "); // ::ₛ symmetric list cons
    let s = s.replace("::\u{2098}", " MultisetCons ");
    let s = normalize_mabs_notation(&s); // |expr|ₘ → (mabs expr) before ₘ→m
    let s = s.replace('\u{2098}', "m");
    let s = s.replace('\u{2099}', "n");
    let s = s.replace('\u{209A}', "p"); // ₚ subscript p
    let s = s.replace('\u{209B}', "s"); // ₛ subscript s (after compound patterns)
    let s = s.replace('\u{209C}', "t"); // ₜ subscript t
    let s = s.replace('\u{2080}', "0");
    let s = s.replace('\u{2081}', "1");
    let s = s.replace('\u{2082}', "2");
    let s = s.replace('\u{2083}', "3");
    let s = s.replace('\u{2084}', "4");
    let s = s.replace('\u{2085}', "5");
    let s = s.replace("::", " Cons ");
    let s = s.replace(" ++ ", " Append ");
    let s = s.replace(" ~ ", " Perm ");
    let s = s.replace(" \\\\ ", " SDiff2 ");
    let s = s.replace(" \\ ", " SDiff ");
    let s = s.replace('\u{2245}', " Congr ");
    let s = s.replace("≃+*o", " RingOrderIso ");
    let s = s.replace("≃+*", " RingEquiv ");
    let s = s.replace("≃+", " AddEquiv ");
    let s = s.replace("≃*", " MulEquiv ");
    let s = s.replace("≃o", " OrderIso ");
    let s = normalize_equiv_subscript_brackets(&s);
    let s = s.replace("\u{2243}\u{209B}\u{2097}", " SemilinearEquiv "); // ≃ₛₗ bare
    let s = s.replace("≃ₗ", " LinearEquiv ");
    let s = s.replace('\u{2243}', " Equiv ");
    // Now safe to convert remaining ₗ subscript (after ≃ₗ and →ₗ have been handled)
    let s = s.replace('\u{2097}', "l");
    let s = s.replace('\u{2248}', " Approx ");
    let s = s.replace('\u{2241}', " NotEquiv ");
    let s = s.replace('\u{2261}', " Equiv3 ");
    let s = s.replace('\u{207A}', "_succ");
    let s = s.replace("\u{207B}\u{00B9}\u{0027}", " Preimage ");
    let s = s.replace(" \u{0027}\u{0027} ", " Image ");
    let s = s.replace("[T;T\u{207B}\u{00B9}]", ""); // R[T;T⁻¹] Laurent polynomial → strip bracket
    let s = normalize_cond_exp(&s); // P⁻[X|mΩ] → condExp X mΩ (BEFORE ⁻¹ replacement)
    let s = s.replace("\u{207B}\u{00B9}", " Inv ");
    let s = s.replace('\u{207B}', "_inv"); // ⁻ bare superscript minus
    let s = s.replace("<$$>", " FMap2 "); // <$$> functor map (double dollar, BEFORE single)
    let s = s.replace("<$>", " FMap ");
    let s = s.replace("<*>", " SeqApply ");
    let s = s.replace(" <* ", " SeqLeft ");
    let s = s.replace(" *> ", " SeqRight ");
    let s = s.replace("<|>", " OrElse ");
    let s = s.replace(" <> ", " HAppend "); // <> append operator (after <|> to avoid conflict)
    let s = s.replace("\u{22C6}", " Star ");
    let s = s.replace("\u{2135}", "Aleph");
    let s = s.replace('\u{215F}', " InvOf ");
    let s = s.replace('\u{2190}', " <- ");
    let s = s.replace('\u{22A2}', " Entails ");
    let s = s.replace('\u{25B8}', " Subst ");
    let s = normalize_kleene_star(&s);
    let s = s.replace(">>>", " ShiftRight ");
    let s = s.replace(">>=", " Bind ");
    let s = s.replace("<<<", " ShiftLeft ");
    let s = s.replace("&&&", " BitAnd ");
    let s = s.replace("|||", " BitOr ");
    let s = s.replace("^^^", " Xor ");
    // Strip `&` (Lean 4 borrow/reference/vector projection like `v &0`)
    let s = s.replace('&', " ");
    let s = s.replace("<+:", " IsPrefix ");
    let s = strip_mod_bracket_notation(&s);
    let s = s.replace(" <| ", " ");
    let s = s.replace(" |> ", " ");
    let s = s.replace("|>.", " ");
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
    let s = s.replace(" /. ", " RatDiv ");
    let s = s.replace(" $ ", " ");
    let s = normalize_matrix_literal(&s); // Must run BEFORE ![] and ![
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
    // Normalize unit lambda `fun () =>` / `fun () ->` before => → -> conversion
    let s = s.replace("fun () =>", "fun (_unit : Unit) =>");
    let s = s.replace("fun () ->", "fun (_unit : Unit) ->");
    let s = s.replace("fun ()  ->", "fun (_unit : Unit) ->");
    let s = s.replace("fun ()  =>", "fun (_unit : Unit) =>");
    let s = s.replace(" => ", " -> ");
    let s = s.replace("=>\n", "->\n");
    let s = normalize_fun_comma_lambda(&s); // fun x, body → fun x -> body (after => → ->)
    let s = strip_where_block(&s);
    let s = normalize_have_in_type(&s);
    let s = strip_have_instances(&s);
    let s = strip_named_args(&s);
    // Strip `let _ := <expr>;` from type positions BEFORE proof replacement
    // (these are instance bindings that confuse the `:=` finder)
    let s = strip_let_underscore(&s);
    let s = replace_proof_with_sorry(&s);
    // Normalize struct literal bodies: `:= { field := val }` -> `:= sorry`
    let s = normalize_struct_literal_body(&s);
    // Strip equation-compiler-style match branches from declarations:
    // e.g. `def f : T | pat1 -> body1 | pat2 -> body2` → `def f : T := sorry`
    let s = strip_equation_branches(&s);
    // Strip empty type ascriptions: `(expr :)` → `(expr)` (Lean 4 coercion idiom)
    let s = s.replace(" :)", ")");
    // Strip `?` from identifiers (Lean 4 optional/tactic syntax names like `elabrw??Command`)
    let s = s.replace('?', "");
    // Strip backtick from identifiers (Lean 4 name quoting)
    let s = s.replace('`', "");
    // Strip restriction `|_` operator (Lean 4 restriction notation)
    let s = s.replace(" |_ ", " ");
    // Replace `¬` with `Not ` — parser handles Not prefix but fails in some argument positions
    let s = s.replace('\u{00AC}', "Not ");
    let s = normalize_destructuring_binders(&s);
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
    let s = normalize_anon_dot_method(&s);
    let s = normalize_anon_dot_constructors(&s);
    let s = s.replace("= .rfl", "= rfl"); // .rfl anonymous constructor
    let s = normalize_angle_adjoin(&s);
    let s = normalize_spread_syntax(&s);
    let s = normalize_semicolons_in_parens(&s);
    let s = s.replace("#check ", "-- #check ");
    let s = s.replace("#eval ", "-- #eval ");
    let s = normalize_numeric_field_access(&s);
    let s = s.replace(" <+~ ", " Subperm "); // <+~ subperm (BEFORE <+)
    let s = s.replace("<+~ ", " Subperm "); // <+~ at start
    let s = s.replace(" <+ ", " Sublist ");
    let s = s.replace("<+:", " IsPrefix ");
    let s = s.replace('\u{221A}', "Sqrt"); // √ square root
    let s = s.replace('\u{00B7}', " dot ");
    let s = s.replace("(_ :", "(hole_0 :");
    let s = normalize_tilde_relation(&s);
    let s = s.replace(" // ", " suchThat ");
    let s = normalize_set_literals(&s);
    // Formal power series: R[[X]] → R[X] — only match when NOT preceded by MatVec/[
    let s = normalize_double_brackets(&s);
    let s = s.replace('\u{2039}', "( ");
    let s = s.replace('\u{203A}', " )");
    let s = s.replace('\u{21A5}', " coe_subtype ");
    let s = s.replace('\u{2191}', " coe ");
    let s = s.replace('\u{2193}', " coe_down ");
    let s = normalize_cons_without_spaces(&s);
    let s = normalize_filter_restriction(&s);
    let s = space_before_comparison_ops(&s);
    let s = normalize_bounded_quantifiers(&s);
    let s = normalize_bounded_forall(&s);
    let s = normalize_finset_card_notation(&s);
    let s = normalize_set_builder_notation(&s);
    let s = normalize_with_filter(&s);
    let s = normalize_double_factorial(&s);
    let s = normalize_postfix_factorial(&s);
    let s = normalize_postfix_primorial(&s);
    let s = normalize_integral_in(&s);
    let s = normalize_integral_in(&s); // second pass for nested integrals
    let s = normalize_big_prod_sum(&s);
    let s = normalize_big_prod_sum(&s);
    let s = normalize_bare_big_quantifiers(&s);
    let s = normalize_big_prod_sum(&s);
    let s = normalize_bare_big_quantifiers(&s);
    let s = normalize_tensor_subscript_brackets(&s);
    let s = normalize_bare_big_quantifiers(&s);
    let s = normalize_subscript_indexing(&s);
    let s = strip_tick_subscript(&s); // Strip 'ident after ] or ) (proof obligation annotations)
    let s = normalize_list_literal_in_type(&s);
    // Normalize empty list literal `[]` → `ListNil` (after list_literal and subscript normalizations)
    let s = normalize_empty_list_literal(&s);
    let s = normalize_subtype_braces(&s);
    let s = normalize_default_binder_values(&s);
    let s = normalize_fun_bare_binders(&s);
    let s = normalize_dfinsupp_type(&s);
    let s = normalize_dite(&s);
    let s = normalize_if_then_else(&s);
    let s = normalize_head_binders(&s);
    let s = strip_term_type_ascriptions(&s);
    let s = normalize_double_braces(&s);
    let s = normalize_singleton_sets(&s);
    let s = normalize_psigma_binder(&s);
    let s = normalize_exists_unique(&s);
    let s = normalize_exists_quantifier(&s);
    let s = normalize_exists_quantifier(&s); // second pass for nested ∃
    let s = strip_typeclass_forall_brackets(&s);
    //let s = strip_remaining_instance_brackets(&s);
    let s = strip_quantifier_binder_groups(&s);
    let s = strip_prop_condition_binders(&s);
    let s = parenthesize_bare_forall_binders(&s);
    let s = normalize_sigma_in_binders(&s);
    // Rename `.then` → `.then_` to avoid keyword clash (then is if/then/else keyword)
    // Also rename `end` when used as expression name (not keyword)
    let s = s.replace("(end ", "(end_ ");
    let s = s.replace("(end)", "(end_)");
    let s = rename_keyword_fields(&s);
    let s = parenthesize_dot_exprs(&s);
    // Strip `.method` calls on complex (non-ident) expressions: `) .method` → `) `
    let s = strip_paren_dot_method(&s);
    // Strip restriction bar `)|` → `) ` after dot-expr parenthesization
    let s = s.replace(")| ", ") ");
    // Strip bounded quantifiers in fun binders: `fun (r > 0) ->` → `fun r ->`
    let s = normalize_fun_bounded_binders(&s);
    // Second pass: handle nested set builders exposed after }.field stripping
    let s = normalize_set_builder_notation(&s);
    let s = normalize_singleton_sets(&s);
    // Third pass: handle nested set literals inside singleton (e.g. `(singleton {x, y})`)
    let s = normalize_set_literals(&s);
    let s = normalize_implicit_forall_binders(&s);
    let s = normalize_implicit_arrow_binders(&s);
    // Normalize `by <tactic>` inside parentheses that wasn't caught by normalize_inline_by.
    // E.g., `l'(Nat.mod_lt _ by grind)` → `l'(sorry)`
    let s = normalize_remaining_by_in_parens(&s);
    // Normalize `(do ...)` blocks → `sorry`
    let s = normalize_do_blocks(&s);
    // Normalize `match expr` at end of declaration body → `:= sorry`
    let s = normalize_trailing_match(&s);
    // Strip `!` postfix from identifiers (Lean 4 syntax like `l1!`, `l2!`)
    let s = normalize_exclamation_postfix(&s);
    // Strip `bif ... then ... else ...` → `sorry` (Boolean if)
    let s = normalize_bif(&s);
    // Normalize operator sections: (*), (+), (-), etc. → named functions
    let s = s.replace("(*)", "mul_op");
    let s = s.replace("(+)", "add_op");
    let s = s.replace("(-)", "sub_op");
    let s = s.replace("(/)", "div_op");
    // Strip `singleton (...)` patterns that are normalization artifacts
    let s = s.replace("(singleton (", "((");
    let s = s.replace(" singleton (", " (");
    // Normalize `[ident` artifacts from subscript stripping (like `[L` or `[l`)
    let s = normalize_bracket_ident_artifact(&s);
    // Fix `hole_0` → `_` (wildcard/hole)
    let s = s.replace("hole_0", "_");
    // Fix destructuring binders more aggressively
    let s = normalize_destructuring_binders_v2(&s);
    // Clean up `dot .method` → `.method` (anonymous placeholder dot-notation)
    let s = s.replace(" dot .", " .");
    // Second pass: normalize anonymous dot methods created by `dot .` cleanup
    let s = normalize_anon_dot_method(&s);
    // Normalize `show ... by ...` in proof bodies → strip to sorry
    let s = normalize_show_by(&s);
    // Late pass: fix `fun (binders), body` → `fun (binders) -> body`
    // This catches comma-lambdas created by intermediate normalizations
    let s = normalize_fun_comma_lambda(&s);
    let s = normalize_fun_comma_lambda(&s); // second pass for nested
                                            // Late pass: fix double parens in forall/exists: `forall ((x : T)), body` → `forall (x : T), body`
    let s = fix_double_parens_in_quantifiers(&s);
    // Late pass: second parenthesize_bare_forall_binders for any new bare binders
    let s = parenthesize_bare_forall_binders(&s);
    // Late pass: fix remaining `fun binder , body` patterns more aggressively
    let s = fix_fun_comma_to_arrow(&s);
    // Late pass: normalize `(show EXPR sorry)` → `sorry` inside types
    let s = normalize_show_sorry_in_parens(&s);
    // Late pass: fix `∀ i)` or `forall i)` patterns (untyped forall inside parens missing comma)
    // These come from `(∀ a, M a)` which gets normalized to `(∀ a) -> M a)` with broken parens
    let s = fix_forall_inside_fun_type(&s);
    // Late pass: fix `∀ IDENT,) -> body` → `∀ IDENT, body)` (forall body moved outside parens)
    let s = fix_forall_empty_body(&s);
    // Strip ` <- ` (monadic bind) from type positions — these are artifacts from
    // `← ` normalization that survived proof replacement (e.g., in do-block bodies)
    let s = strip_bind_arrow_in_type(&s);
    // Final cleanup: ensure space before `:= sorry` (may get lost during normalizations)
    let s = s
        .replace("):= sorry", ") := sorry")
        .replace("]:= sorry", "] := sorry");
    // Fix empty RHS: `= := sorry` → `:= sorry`
    let s = s.replace("= := sorry", ":= sorry");
    // Fix empty type body: `: := sorry` → `: _ := sorry`
    let s = s.replace(": := sorry", ": _ := sorry");
    // Fix empty forall body: `, := sorry` → `, _ := sorry`
    let s = s.replace(", := sorry", ", _ := sorry");
    // Fix trailing binary operators before `:= sorry` — incomplete RHS
    // e.g., `↔ := sorry` → `↔ _ := sorry`, `∧ := sorry` → `∧ _ := sorry`
    let s = fix_trailing_operator_before_sorry(&s);
    // Fix forall with binders but no body before :=
    // `forall (x : T) := sorry` → `forall (x : T), _ := sorry`
    let s = fix_forall_no_body_before_assign(&s);
    // Note: `-> ->` patterns are NOT fixed here because they can be legitimate
    // (e.g., `fun _ -> body fun _ -> body` in higher-order functions).
    // Normalize `if ... then ... else ...` in types to `(ite cond t e)`
    let s = normalize_if_then_else_in_type(&s);
    // Strip `match ... with` expressions in type position — replace with `_`
    let s = normalize_match_in_type(&s);
    // Strip orphaned `]` brackets (artifacts from subscript/arrow notation stripping)
    let s = strip_orphan_close_brackets(&s);
    // Strip orphaned `)` parens (artifacts from various normalizations)
    let s = strip_orphan_close_parens(&s);
    // Late pass: strip `by` in type position (truncated tactic-computed types)
    // Pattern: `, by ...` or `: by ...` at the end (no `:= sorry` yet)
    let s = strip_by_in_type_position(&s);
    // Balance unclosed parens before `:= sorry`
    let s = balance_parens_before_sorry(&s);
    // Fix truncated declarations: if it ends without `:= sorry`, add it
    let s = fix_truncated_decl(&s);
    // Fix: sorry placed inside parentheses — truncate after `:= sorry`
    let s = fix_sorry_inside_parens(&s);
    // Strip trailing `let <ident> := sorry` or `by let <ident> := sorry` patterns
    // These occur when the whole body is a let binding with no further `:=`
    let s = strip_trailing_let(&s);
    // Final: strip remaining `by ` after `:= sorry` (leaked proof tactics)
    let s = if let Some(pos) = s.find(":= sorry") {
        let after = &s[pos + 8..];
        if after.trim_start().starts_with("by ") || after.trim_start().starts_with("by\n") {
            s[..pos + 8].to_string()
        } else {
            s
        }
    } else {
        s
    };
    // Final: if output still has ` by ` after `:=` but no sorry, add sorry
    if let Some(assign_pos) = s.find(":= ") {
        let after_assign = &s[assign_pos + 3..];
        if after_assign.starts_with("by ") || after_assign == "by" {
            return format!("{} := sorry", s[..assign_pos].trim_end());
        }
    }
    s
}

/// Fix empty RHS before `:= sorry` — handles `= := sorry` with any amount of whitespace.
/// `= := sorry` → `:= sorry`, `=  := sorry` → `:= sorry`, etc.
#[allow(dead_code)]
fn fix_empty_rhs_before_sorry(src: &str) -> String {
    if let Some(sorry_pos) = src.find(":= sorry") {
        let before = &src[..sorry_pos];
        let trimmed = before.trim_end();
        if trimmed.ends_with('=') && !trimmed.ends_with(":=") {
            let eq_pos = trimmed.len() - 1;
            return format!("{}{}", &src[..eq_pos], &src[sorry_pos..]);
        }
        // Also strip trailing binary operators (preceded by space): ↔, ∧, ∨
        for op in &[
            " \u{2194}", // ↔
            " \u{2227}", // ∧
            " \u{2228}", // ∨
            " Iff",
            " And",
            " Or",
        ] {
            if trimmed.ends_with(op) {
                let op_start = trimmed.len() - op.len();
                return format!("{} _ {}", &src[..op_start].trim_end(), &src[sorry_pos..]);
            }
        }
    }
    src.to_string()
}

/// Fix `forall (...) := sorry` where the forall has no body before `:=`.
/// Inserts `, _` before `:= sorry` to give the forall a placeholder body.
fn fix_forall_no_body_before_assign(src: &str) -> String {
    let assign_pat = ":= sorry";
    let Some(assign_pos) = src.find(assign_pat) else {
        return src.to_string();
    };
    let before = src[..assign_pos].trim_end();
    let (close_ch, open_ch) = if before.ends_with(')') {
        (b')', b'(')
    } else if before.ends_with(']') {
        (b']', b'[')
    } else {
        return src.to_string();
    };
    // Walk backwards to find the matching open bracket and check for `:` at depth 1
    let bytes = before.as_bytes();
    let blen = bytes.len();
    let mut depth = 0i32;
    let mut j = blen;
    let mut has_colon_d1 = false;
    loop {
        if j == 0 {
            return src.to_string();
        }
        j -= 1;
        match bytes[j] {
            c if c == close_ch => depth += 1,
            c if c == open_ch => {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }
            b':' if depth == 1 => {
                if j + 1 < blen && bytes[j + 1] != b'=' {
                    has_colon_d1 = true;
                }
            }
            _ => {}
        }
    }
    // Must be a typed binder (has `:` at depth 1)
    if !has_colon_d1 {
        return src.to_string();
    }
    // Walk backwards through preceding binder groups to find `forall`/`∀`
    let mut check = before[..j].trim_end();
    loop {
        if check.ends_with("forall") || check.ends_with('\u{2200}') {
            break;
        }
        if check.ends_with(')') || check.ends_with(']') {
            let (inner_close, inner_open) = if check.ends_with(')') {
                (b')', b'(')
            } else {
                (b']', b'[')
            };
            let cb = check.as_bytes();
            let mut d = 0i32;
            let mut k = cb.len();
            loop {
                if k == 0 {
                    return src.to_string();
                }
                k -= 1;
                match cb[k] {
                    c if c == inner_close => d += 1,
                    c if c == inner_open => {
                        d -= 1;
                        if d == 0 {
                            break;
                        }
                    }
                    _ => {}
                }
            }
            check = check[..k].trim_end();
        } else {
            return src.to_string();
        }
    }
    format!("{before}, _ {assign_pat}")
}

/// Fix trailing binary operators before `:= sorry`.
///
/// When normalization strips the RHS of a binary operator, the operator is left dangling:
/// - `↔ := sorry` → `↔ _ := sorry`
/// - `∧ := sorry` → `∧ _ := sorry`
/// - `∨ := sorry` → `∨ _ := sorry`
/// - `-> := sorry` → `-> _ := sorry`
/// - `Iff := sorry` → `Iff _ := sorry`
fn fix_trailing_operator_before_sorry(src: &str) -> String {
    let assign_pat = ":= sorry";
    let Some(assign_pos) = src.find(assign_pat) else {
        return src.to_string();
    };
    let before = src[..assign_pos].trim_end();
    // List of trailing operators that indicate an incomplete RHS
    let trailing_ops: &[&str] = &[
        "\u{2194}", // ↔
        "\u{2227}", // ∧
        "\u{2228}", // ∨
        "->", "Iff", "And", "Or", "&&", "||", "\u{2264}", // ≤
        "\u{2265}", // ≥
        "\u{2260}", // ≠
        "!=", "==", "+", "-", "*", "/",
    ];
    for op in trailing_ops {
        if before.ends_with(op) {
            // Make sure the operator is not part of a longer identifier
            let prefix = before.strip_suffix(op).unwrap_or(before);
            let last_char = prefix.chars().next_back();
            let is_boundary = match last_char {
                None => true,
                Some(c) => !c.is_alphanumeric() && c != '_' && c != '\'',
            };
            // For single-char symbolic operators, always treat as boundary
            let is_symbolic =
                op.len() <= 3 && op.chars().next().is_some_and(|c| !c.is_alphanumeric());
            if is_boundary || is_symbolic {
                return format!("{before} _ {assign_pat}");
            }
        }
    }
    src.to_string()
}

/// Fix `-> ->` double arrow patterns in type position.
/// These arise when intermediate expressions are stripped, leaving consecutive arrows.
/// `-> ->` → `->`, also handles `->  ->` with whitespace.
#[allow(dead_code)]
fn fix_double_arrow_in_type(src: &str) -> String {
    if !src.contains("->") {
        return src.to_string();
    }
    // Only fix before `:= sorry`
    let assign_pos = src.find(":= sorry").unwrap_or(src.len());
    let before = &src[..assign_pos];
    if !before.contains("->") {
        return src.to_string();
    }
    // Replace `-> <whitespace> ->` with `->` (repeatedly)
    let mut result = src.to_string();
    for _ in 0..10 {
        let new = result.replace("->  ->", "->").replace("-> ->", "->");
        if new == result {
            break;
        }
        result = new;
    }
    result
}

/// Fix double parentheses in forall/exists binders.
///
/// `forall ((x : T)), body` → `forall (x : T), body`
/// `forall ((x : T)) ((y : U)), body` → `forall (x : T) (y : U), body`
///
/// Also handles `Exists ((fun ...))` → `Exists (fun ...)`.
fn fix_double_parens_in_quantifiers(src: &str) -> String {
    // Simple pattern: replace `((` followed by content with `:` followed by `))` with single parens
    // This handles `forall ((x : T)), body` → `forall (x : T), body`
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut result = String::with_capacity(src.len());
    let mut i = 0;
    while i < len {
        // Detect `((` that might be a double-paren binder
        if i + 1 < len && chars[i] == '(' && chars[i + 1] == '(' {
            // Check if the inner paren group closes with `))` (double close)
            // Find matching inner paren
            let inner_start = i + 1;
            let mut depth = 1usize;
            let mut j = inner_start + 1;
            let mut inner_end = None;
            while j < len && depth > 0 {
                match chars[j] {
                    '(' => depth += 1,
                    ')' => {
                        depth -= 1;
                        if depth == 0 {
                            inner_end = Some(j);
                        }
                    }
                    _ => {}
                }
                j += 1;
            }
            if let Some(ie) = inner_end {
                // Check if immediately followed by another `)`
                if ie + 1 < len && chars[ie + 1] == ')' {
                    // Check the inner content has a colon (it's a binder like `(x : T)`)
                    let inner: String = chars[inner_start..ie + 1].iter().collect();
                    if inner.contains(':') || inner.starts_with("fun ") {
                        // Replace `((x : T))` with `(x : T)`
                        result.push_str(&inner);
                        i = ie + 2;
                        continue;
                    }
                }
            }
        }
        result.push(chars[i]);
        i += 1;
    }
    result
}

/// Fix remaining `fun BINDERS , body` patterns by replacing comma with `->`.
///
/// This is a more aggressive version of `normalize_fun_comma_lambda` that handles
/// edge cases where the first pass missed patterns due to intermediate normalizations.
/// It specifically targets commas that appear right after a closing paren in fun context.
fn fix_fun_comma_to_arrow(src: &str) -> String {
    if !src.contains("fun ") {
        return src.to_string();
    }
    let mut result = String::with_capacity(src.len());
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut i = 0;
    while i < len {
        let prev_is_word = i > 0 && {
            let p = chars[i - 1];
            p.is_alphanumeric() || p == '_' || p == '\''
        };
        // Detect `fun ` keyword
        let is_fun = !prev_is_word
            && i + 4 <= len
            && chars[i] == 'f'
            && chars[i + 1] == 'u'
            && chars[i + 2] == 'n'
            && chars[i + 3] == ' ';
        if is_fun {
            result.push_str("fun ");
            i += 4;
            // Scan forward, tracking depth. Replace first comma at depth 0 with ` ->`
            // but only if there's no `->` before it.
            let scan_start = i;
            let mut depth = 0usize;
            let mut found_arrow = false;
            let mut comma_pos = None;
            let mut j = i;
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
                    '-' if depth == 0 && j + 1 < len && chars[j + 1] == '>' => {
                        found_arrow = true;
                        break;
                    }
                    ':' if depth == 0 && j + 1 < len && chars[j + 1] == '=' => {
                        break;
                    }
                    ',' if depth == 0 => {
                        comma_pos = Some(j);
                        break;
                    }
                    _ => {
                        j += 1;
                    }
                }
            }
            if !found_arrow {
                if let Some(cp) = comma_pos {
                    // Check if the text before the comma contains ∀/forall/∃/exists at depth 0
                    let binders: String = chars[scan_start..cp].iter().collect();
                    let has_quantifier = {
                        let mut d = 0usize;
                        let mut found_q = false;
                        for ch in binders.chars() {
                            match ch {
                                '(' | '[' | '{' => d += 1,
                                ')' | ']' | '}' => d = d.saturating_sub(1),
                                '\u{2200}' | '\u{2203}' if d == 0 => {
                                    found_q = true;
                                    break;
                                }
                                _ => {}
                            }
                        }
                        if !found_q {
                            found_q = binders.contains("forall ") || binders.contains("exists ");
                        }
                        found_q
                    };
                    if !has_quantifier {
                        result.push_str(&binders);
                        result.push_str(" ->");
                        i = cp + 1;
                        continue;
                    }
                }
            }
            // No comma or already has arrow, continue normally
            continue;
        }
        result.push(chars[i]);
        i += 1;
    }
    result
}

/// Normalize `(show EXPR sorry)` patterns inside types → `sorry`.
///
/// In Lean 4, `show` is used for type ascription/proof goals.
/// After normalization, patterns like `(show p + q = r sorry)` remain
/// inside the type and cause parse failures.
fn normalize_show_sorry_in_parens(src: &str) -> String {
    if !src.contains("show ") {
        return src.to_string();
    }
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut result = String::with_capacity(src.len());
    let mut i = 0;
    while i < len {
        // Look for `(show ` or ` show ` at depth > 0
        if chars[i] == '(' && i + 6 <= len {
            let peek: String = chars[i + 1..i + 6.min(len)].iter().collect();
            if peek == "show " {
                // Find matching `)` and check if content ends with `sorry)`
                let start = i;
                let mut j = i + 1;
                let mut depth = 1usize;
                while j < len && depth > 0 {
                    match chars[j] {
                        '(' => depth += 1,
                        ')' => depth -= 1,
                        _ => {}
                    }
                    if depth > 0 {
                        j += 1;
                    }
                }
                if depth == 0 {
                    // Check if the content contains "sorry" near the end
                    let inner: String = chars[start + 1..j].iter().collect();
                    if inner.trim_end().ends_with("sorry") {
                        result.push_str("sorry");
                        i = j + 1;
                        continue;
                    }
                }
            }
        }
        result.push(chars[i]);
        i += 1;
    }
    result
}

/// Fix `∀ i)` / `forall i)` patterns where forall without type appears inside parens.
///
/// This comes from Lean 4 patterns like `(∀ a, M a)` which after normalization become
/// something like `(fun (f : ∀ a) -> M a -> f a)` where the forall lost its body.
/// Fix: `∀ i)` → `∀ i,` (add missing comma so the forall can parse its body).
fn fix_forall_inside_fun_type(src: &str) -> String {
    if !src.contains("∀ ") && !src.contains("forall ") {
        return src.to_string();
    }
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut result = String::with_capacity(src.len());
    let mut i = 0;
    while i < len {
        let prev_is_word = i > 0 && {
            let p = chars[i - 1];
            p.is_alphanumeric() || p == '_' || p == '\''
        };
        let is_forall_unicode = chars[i] == '\u{2200}' && !prev_is_word;
        let is_forall_word = !prev_is_word
            && i + 6 <= len
            && chars[i..i + 6].iter().collect::<String>() == "forall"
            && (i + 6 >= len || {
                let next = chars[i + 6];
                !next.is_alphanumeric() && next != '_' && next != '\''
            });
        if is_forall_unicode || is_forall_word {
            let kw_len = if is_forall_unicode { 1 } else { 6 };
            let kw: String = chars[i..i + kw_len].iter().collect();
            // Check if the pattern is `forall IDENT)` (no comma before close paren)
            let mut j = i + kw_len;
            while j < len && chars[j] == ' ' {
                j += 1;
            }
            // Scan ident(s) - stop at `)`, `,`, `:`
            let ident_start = j;
            while j < len
                && (chars[j].is_alphanumeric()
                    || chars[j] == '_'
                    || chars[j] == '\''
                    || chars[j] == ' ')
            {
                // Only continue if ident chars followed by space then more ident chars
                if chars[j] == ' ' {
                    // Check if next non-space is ident-like or special
                    let mut k = j + 1;
                    while k < len && chars[k] == ' ' {
                        k += 1;
                    }
                    if k < len
                        && (chars[k].is_alphabetic() || chars[k] == '_')
                        && !(k + 1 < len && chars[k] == '-' && chars[k + 1] == '>')
                    {
                        j = k;
                    } else {
                        break;
                    }
                } else {
                    j += 1;
                }
            }
            let idents: String = chars[ident_start..j].iter().collect();
            let idents_trimmed = idents.trim();
            // Skip spaces
            while j < len && chars[j] == ' ' {
                j += 1;
            }
            // If followed by `)`, this is `forall IDENT)` — add comma after ident
            if j < len && chars[j] == ')' && !idents_trimmed.is_empty() {
                result.push_str(&kw);
                result.push(' ');
                result.push_str(idents_trimmed);
                result.push(',');
                // Don't consume the `)`, let it be part of the body
                i = j;
                continue;
            }
        }
        result.push(chars[i]);
        i += 1;
    }
    result
}

/// Fix `∀ IDENT,) -> BODY` or `forall IDENT,) -> BODY` patterns.
///
/// When `(f : ∀ i, F i)` gets broken into `(f : ∀ i,) -> F i` by intermediate
/// normalizations, this moves the body back inside the parens:
/// `(f : ∀ i,) -> F i` → `(f : ∀ i, F i)`
fn fix_forall_empty_body(src: &str) -> String {
    // Look for `,) ->` pattern preceded by a forall/∀
    if !src.contains(",) ->") {
        return src.to_string();
    }
    let mut s = src.to_string();
    // Repeatedly fix the pattern
    for _ in 0..10 {
        if let Some(fix) = fix_forall_empty_body_once(&s) {
            s = fix;
        } else {
            break;
        }
    }
    s
}

fn fix_forall_empty_body_once(src: &str) -> Option<String> {
    // Find `,) ->`
    let pat = ",) ->";
    let pos = src.find(pat)?;
    // Check that before the `,)` there's a `∀` or `forall` inside the same paren group
    let before = &src[..pos];
    // Find the matching open `(` for the `)` at pos+1
    let _paren_close = pos + 1; // the `)` position
    let mut depth = 0usize;
    let mut paren_open = None;
    let chars: Vec<char> = before.chars().collect();
    for (idx, &ch) in chars.iter().enumerate().rev() {
        match ch {
            ')' | ']' | '}' => depth += 1,
            '(' | '[' | '{' => {
                if depth == 0 {
                    paren_open = Some(idx);
                    break;
                }
                depth -= 1;
            }
            _ => {}
        }
    }
    let paren_open = paren_open?;
    let inner: String = chars[paren_open + 1..].iter().collect();
    // Check if the inner content has ∀ or forall
    if !inner.contains('∀') && !inner.contains("forall") {
        return None;
    }
    // Now we need to find the body after `) ->` and move it inside.
    // The body extends until the next `)` at depth 0 (the matching close paren)
    let after_arrow = &src[pos + pat.len()..];
    let after_trim = after_arrow.trim_start();
    let ws_len = after_arrow.len() - after_trim.len();
    // Scan to find the end of the body (next `)` at depth 0, or `,` at depth 0, or `:=`)
    let body_chars: Vec<char> = after_trim.chars().collect();
    let body_len = body_chars.len();
    let mut body_end = body_len;
    let mut bd = 0usize;
    let mut j = 0;
    while j < body_len {
        match body_chars[j] {
            '(' | '[' | '{' => {
                bd += 1;
                j += 1;
            }
            ')' | ']' | '}' if bd == 0 => {
                body_end = j;
                break;
            }
            ')' | ']' | '}' => {
                bd = bd.saturating_sub(1);
                j += 1;
            }
            ':' if bd == 0 && j + 1 < body_len && body_chars[j + 1] == '=' => {
                body_end = j;
                break;
            }
            ',' if bd == 0 => {
                body_end = j;
                break;
            }
            _ => {
                j += 1;
            }
        }
    }
    let body: String = body_chars[..body_end].iter().collect();
    let body = body.trim_end();
    if body.is_empty() {
        return None;
    }
    // Reconstruct: before_paren_open + `(` + inner + ` ` + body + `)` + rest
    let before_open: String = chars[..paren_open].iter().collect();
    let rest_start = pos + pat.len() + ws_len + body_end;
    let rest = &src[rest_start..];
    let result = format!("{before_open}({inner} {body}){rest}");
    Some(result)
}

/// Lean 4 anonymous struct constructors like `{ toFun := ..., map_add' := ... }`
/// can't be parsed by OxiLean. Replace the entire struct literal with `sorry`.
#[allow(dead_code)]
fn fix_struct_literal_proofs(src: &str) -> String {
    if !src.contains("= {") {
        return src.to_string();
    }
    // Only handle `= { ... := ...` patterns (not just any `= {`)
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut result = String::with_capacity(src.len());
    let mut i = 0;
    while i < len {
        // Look for `= {` at depth 0 that's likely after `:= sorry` was removed
        // Pattern: `= { (IDENT.IDENT) IDENT IDENT ...`
        if i + 3 < len
            && chars[i] == '='
            && chars[i + 1] == ' '
            && chars[i + 2] == '{'
            && chars[i + 3] == ' '
        {
            // Check: is there a `:=` inside the braces?
            let brace_start = i + 2;
            let mut j = brace_start + 1;
            let mut depth = 1usize;
            let mut has_assign = false;
            while j < len && depth > 0 {
                match chars[j] {
                    '{' => depth += 1,
                    '}' => depth -= 1,
                    ':' if j + 1 < len && chars[j + 1] == '=' => {
                        has_assign = true;
                    }
                    _ => {}
                }
                if depth > 0 {
                    j += 1;
                }
            }
            if depth == 0 && has_assign {
                // Replace `= { ... }` with `:= sorry`
                // Check if this is preceded by `:` to avoid `:= := sorry`
                let before: String = result.chars().collect();
                if before.trim_end().ends_with(':') {
                    result.push_str("= sorry");
                } else {
                    result.push_str(":= sorry");
                }
                i = j + 1;
                continue;
            }
        }
        result.push(chars[i]);
        i += 1;
    }
    result
}

/// Normalize remaining `by <tactic>` inside parentheses.
///
/// Handles patterns like `ident'(... by tactic)` → `ident'(sorry)`.
/// This catches cases not handled by `normalize_inline_by`, where `by` appears
/// after other content inside parens (not at the start).
fn normalize_remaining_by_in_parens(src: &str) -> String {
    if !src.contains(" by ") {
        return src.to_string();
    }
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut result = String::with_capacity(src.len());
    let mut i = 0;
    while i < len {
        // Look for ` by ` at depth > 0 (inside parens)
        if i + 4 <= len
            && chars[i] == ' '
            && chars[i + 1] == 'b'
            && chars[i + 2] == 'y'
            && chars[i + 3] == ' '
        {
            // Check if we're inside parentheses by scanning backward
            let depth = count_paren_depth(&result);
            if depth > 0 {
                // Skip everything until matching close paren
                let mut j = i + 4;
                let mut d = 0usize;
                while j < len {
                    match chars[j] {
                        '(' | '[' | '{' => {
                            d += 1;
                            j += 1;
                        }
                        ')' | ']' | '}' if d == 0 => break,
                        ')' | ']' | '}' => {
                            d = d.saturating_sub(1);
                            j += 1;
                        }
                        ',' if d == 0 => break,
                        _ => j += 1,
                    }
                }
                result.push_str(" sorry");
                i = j;
                continue;
            }
        }
        result.push(chars[i]);
        i += 1;
    }
    result
}

/// Count current parenthesis depth by scanning forward through result string.
fn count_paren_depth(s: &str) -> i32 {
    let mut depth: i32 = 0;
    for ch in s.chars() {
        match ch {
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' => depth -= 1,
            _ => {}
        }
    }
    depth
}

/// Normalize `(do ...)` blocks to `sorry`.
fn normalize_do_blocks(src: &str) -> String {
    if !src.contains("(do ") {
        return src.to_string();
    }
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut result = String::with_capacity(src.len());
    let mut i = 0;
    while i < len {
        if i + 4 <= len
            && chars[i] == '('
            && chars[i + 1] == 'd'
            && chars[i + 2] == 'o'
            && chars[i + 3] == ' '
        {
            let mut j = i + 1;
            let mut depth = 1usize;
            while j < len && depth > 0 {
                match chars[j] {
                    '(' => depth += 1,
                    ')' => depth -= 1,
                    _ => {}
                }
                j += 1;
            }
            result.push_str("sorry");
            i = j;
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}

/// Normalize trailing `match expr` at end of type body.
///
/// `= match (Fin.ofNat) 4 n ` → `:= sorry` (no `with` clause means incomplete)
fn normalize_trailing_match(src: &str) -> String {
    // Check if the string ends with `match <stuff>` without `with`
    if !src.contains(" match ") {
        return src.to_string();
    }
    let trimmed = src.trim_end();
    // Find the last `match ` in the string
    if let Some(pos) = trimmed.rfind(" match ") {
        let after_match = &trimmed[pos + 7..];
        // If there's no `with` after this match, it's incomplete
        if !after_match.contains(" with ") && !after_match.contains(" with\n") {
            // Check if this match is after `:=`
            let before = &trimmed[..pos];
            if before.contains(":=") {
                return format!("{} := sorry", before.split(":=").next().unwrap().trim_end());
            }
        }
    }
    src.to_string()
}

/// Strip `!` postfix from identifiers.
///
/// Lean 4 uses `l!` for notation (like array element access with panic).
/// OxiLean doesn't support this, so strip the `!`.
fn normalize_exclamation_postfix(src: &str) -> String {
    if !src.contains('!') {
        return src.to_string();
    }
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut result = String::with_capacity(src.len());
    let mut i = 0;
    while i < len {
        if chars[i] == '!'
            && i > 0
            && (chars[i - 1].is_alphanumeric() || chars[i - 1] == '_' || chars[i - 1] == '\'')
        {
            // Skip the `!` — it's a postfix on an identifier
            i += 1;
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}

/// Normalize `bif ... then ... else ...` (Boolean if) → replace whole construct with `sorry`.
///
/// OxiLean parser doesn't handle `bif`. Replace `bif P then A else B` with `sorry`.
fn normalize_bif(src: &str) -> String {
    if !src.contains("bif ") {
        return src.to_string();
    }
    // Simple approach: find `bif ` and replace until matching close or end
    let mut result = src.to_string();
    while let Some(pos) = result.find("bif ") {
        // Check it's at a word boundary
        if pos > 0 {
            let prev = result.as_bytes()[pos - 1];
            if prev.is_ascii_alphanumeric() || prev == b'_' {
                // Not a word boundary, skip
                break;
            }
        }
        // Find the extent of the bif expression
        let after = &result[pos..];
        let chars: Vec<char> = after.chars().collect();
        let clen = chars.len();
        let mut j = 4; // skip "bif "
        let mut depth = 0usize;
        let mut found_else = false;
        while j < clen {
            match chars[j] {
                '(' | '[' | '{' => {
                    depth += 1;
                    j += 1;
                }
                ')' | ']' | '}' if depth == 0 => break,
                ')' | ']' | '}' => {
                    depth = depth.saturating_sub(1);
                    j += 1;
                }
                'e' if depth == 0 && j + 4 <= clen => {
                    let word: String = chars[j..j + 4].iter().collect();
                    if word == "else" {
                        found_else = true;
                    }
                    j += 1;
                }
                _ => j += 1,
            }
        }
        if found_else {
            let end_byte: usize = after
                .char_indices()
                .nth(j)
                .map(|(i, _)| i)
                .unwrap_or(after.len());
            let replacement = format!("{}sorry{}", &result[..pos], &result[pos + end_byte..]);
            result = replacement;
        } else {
            break;
        }
    }
    result
}

/// Normalize empty list literal `[]` → `List.nil` in expression positions.
///
/// After normalization, `[]` can appear in type expressions. OxiLean parser
/// may not handle bare `[]`, so replace with `List.nil`.
#[allow(dead_code)]
fn normalize_empty_list_literal(src: &str) -> String {
    if !src.contains("[]") {
        return src.to_string();
    }
    let mut result = src.to_string();
    // Replace various positions of `[]` → `ListNil`
    result = result.replace("= []", "= ListNil");
    result = result.replace(" [] ", " ListNil ");
    result = result.replace("[],", "ListNil,");
    result = result.replace("([]", "(ListNil");
    result = result.replace("[] ", "ListNil ");
    result
}

/// Normalize bracket-identifier artifacts from subscript stripping.
///
/// After `normalize_subscript_indexing` strips `ident[idx]` → `ident`,
/// some cases produce `[ident` artifacts. Strip these.
fn normalize_bracket_ident_artifact(src: &str) -> String {
    // Pattern: `= [L` or `= [l` where [ was left over
    // Also: `Append [ident...] → `Append (ident...)` (convert bracket list to parens)
    let mut result = src.to_string();
    // `Append [expr]` → `Append (expr)` (list literal in append context)
    if result.contains("Append [") {
        let chars: Vec<char> = result.chars().collect();
        let mut new_result = String::with_capacity(result.len());
        let mut i = 0;
        let pattern = "Append [";
        while i < chars.len() {
            let rest: String = chars[i..].iter().take(pattern.len()).collect();
            if rest == pattern {
                // Find matching ] at proper depth
                let after_bracket = i + pattern.len();
                let mut depth = 1usize;
                let mut j = after_bracket;
                while j < chars.len() && depth > 0 {
                    match chars[j] {
                        '[' => depth += 1,
                        ']' => depth -= 1,
                        _ => {}
                    }
                    if depth > 0 {
                        j += 1;
                    }
                }
                if depth == 0 {
                    // Replace [content] with (content)
                    let inner: String = chars[after_bracket..j].iter().collect();
                    new_result.push_str("Append (");
                    new_result.push_str(inner.trim());
                    new_result.push(')');
                    i = j + 1; // skip past ]
                } else {
                    new_result.push_str("Append ");
                    i += pattern.len();
                }
            } else {
                new_result.push(chars[i]);
                i += 1;
            }
        }
        result = new_result;
    }
    result
}

/// Fix truncated declarations that end without `:= sorry`.
///
/// Strip `by` or `let` in type position — tactic-computed types from truncated multi-line decls.
/// Pattern: `, by ...` or `), by ...` at end of type.
/// Example: `forall (...), by let semiring := ...` → `forall (...), _ := sorry`
/// Also handles `: let _ := ...` where the let is the type body.
/// Works both when `:= sorry` is already present (let in type BEFORE proof)
/// and when not present (truncated declarations).
fn strip_by_in_type_position(src: &str) -> String {
    // Determine the search region: if `:= sorry` is present, only look before it.
    // (The let/by patterns are in the TYPE, not in the proof body.)
    let search_end = src.find(":= sorry").unwrap_or(src.len());
    let search_region = &src[..search_end];

    // Look for `, by ` or `) by ` or `: by ` or `, let ` or `: let ` patterns at depth 0
    let patterns = [", by ", ") by ", ": by ", ", let ", ": let "];
    for pat in &patterns {
        if let Some(pos) = search_region.rfind(pat) {
            // Check this is at depth 0
            let before = &src[..pos];
            let mut depth: i32 = 0;
            for ch in before.chars() {
                match ch {
                    '(' | '[' | '{' => depth += 1,
                    ')' | ']' | '}' => depth -= 1,
                    _ => {}
                }
            }
            if depth <= 0 {
                let keep_len = if pat.starts_with(')') || pat.starts_with(':') {
                    pos + 1 // keep the `)` or `:`
                } else {
                    pos // keep up to before the pattern
                };
                let prefix = src[..keep_len].trim_end();
                return format!("{prefix}, _ := sorry");
            }
        }
    }
    src.to_string()
}

/// Some multi-line declarations get truncated during extraction, leaving
/// a declaration without a body. Add `:= sorry` if missing.
fn fix_truncated_decl(src: &str) -> String {
    let trimmed = src.trim_end();
    // Only for decl-like strings
    if !(trimmed.starts_with("theorem ")
        || trimmed.starts_with("lemma ")
        || trimmed.starts_with("def ")
        || trimmed.starts_with("axiom "))
    {
        return src.to_string();
    }
    if trimmed.contains(":=") {
        return src.to_string();
    }
    // No `:=` found — add `:= sorry`
    format!("{trimmed} := sorry")
}

/// Balance unclosed parentheses before `:= sorry`.
///
/// After normalization, some declarations end up with unmatched `(` before `:= sorry`.
/// Add missing `)` to balance them.
fn balance_parens_before_sorry(src: &str) -> String {
    if let Some(pos) = src.rfind(":= sorry") {
        let before = &src[..pos];
        let mut depth: i32 = 0;
        for ch in before.chars() {
            match ch {
                '(' => depth += 1,
                ')' => depth -= 1,
                _ => {}
            }
        }
        if depth > 0 {
            let closing = ")".repeat(depth as usize);
            return format!("{}{} := sorry", before.trim_end(), closing);
        }
        if depth < 0 {
            // Excess closing parens — strip trailing ')' from the type before `:= sorry`
            let excess = (-depth) as usize;
            let mut trimmed = before.trim_end().to_string();
            let mut removed = 0usize;
            while removed < excess {
                if trimmed.ends_with(')') {
                    trimmed.pop();
                    trimmed = trimmed.trim_end().to_string();
                    removed += 1;
                } else {
                    break;
                }
            }
            return format!("{} := sorry", trimmed);
        }
    }
    src.to_string()
}

/// Strip `.method` calls on complex (non-identifier) expressions.
///
/// After `parenthesize_dot_exprs` handles `ident.method` → `(ident.method)`,
/// some `.method` patterns remain on non-identifier expressions like `(expr) .method`.
/// Strip these to just `(expr)` since OxiLean can't handle them.
fn strip_paren_dot_method(src: &str) -> String {
    if !src.contains(") .") && !src.contains(").") && !src.contains(" .") {
        return src.to_string();
    }
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut result = String::with_capacity(src.len());
    let mut i = 0;
    while i < len {
        // Pattern 1: `) .method` (with space)
        if chars[i] == ')' && i + 2 < len && chars[i + 1] == ' ' && chars[i + 2] == '.' {
            let mut j = i + 3;
            if j < len && (chars[j].is_alphabetic() || chars[j] == '_') {
                while j < len && (chars[j].is_alphanumeric() || chars[j] == '_' || chars[j] == '\'')
                {
                    j += 1;
                }
                result.push(')');
                i = j;
                continue;
            }
        }
        // Pattern 2: `).method` (no space) -- also strip the dot-method access
        if chars[i] == ')' && i + 1 < len && chars[i + 1] == '.' {
            let mut j = i + 2;
            if j < len && (chars[j].is_alphabetic() || chars[j] == '_') {
                while j < len && (chars[j].is_alphanumeric() || chars[j] == '_' || chars[j] == '\'')
                {
                    j += 1;
                }
                result.push(')');
                i = j;
                continue;
            }
        }
        // Pattern 3: `ident .method` -- strip space-dot-method after identifier
        // Only when preceded by an alphanumeric/underscore/quote (end of ident)
        if i > 0
            && chars[i] == ' '
            && i + 2 < len
            && chars[i + 1] == '.'
            && chars[i + 2].is_alphabetic()
        {
            let prev = chars[i - 1];
            if prev.is_alphanumeric() || prev == '_' || prev == '\'' {
                // Check this isn't inside a forall/fun binder list (don't strip `.` that's part of dot notation)
                // Only strip if the dot-method is followed by space, paren, or end
                let mut j = i + 2;
                while j < len && (chars[j].is_alphanumeric() || chars[j] == '_' || chars[j] == '\'')
                {
                    j += 1;
                }
                // Skip the space and dot-method, keep the space
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

/// Extended destructuring binder normalization.
///
/// Handles patterns missed by `normalize_destructuring_binders`:
/// - `fun ((a, b) : T) ->` with nested parens
/// - `(fun ((a, b) : T) -> body)` in SetOf context
/// - Multi-level: `fun ((a, (b, c)) : T) ->`
fn normalize_destructuring_binders_v2(src: &str) -> String {
    let mut result = src.to_string();
    // Handle `fun (((ident, ident), ident) ...)` patterns that weren't caught
    // by the main pass. Do a simple regex-like scan.
    // Pattern: SetOf (fun ((a, b) : T) -> body)
    // → SetOf (fun (a_b_ : T) -> body)
    let max_iter = 10;
    for _ in 0..max_iter {
        if !result.contains("fun ((")
            && !result.contains("forall ((")
            && !result.contains("\u{2200} ((")
        {
            break;
        }
        let before = result.clone();
        result = normalize_destructuring_binders(&result);
        if result == before {
            break;
        }
    }
    result
}

/// Strip trailing `let`/`by let` patterns from declarations that have no proper `:= sorry`.
/// These are cases where `find_top_level_assign` skipped `let :=` and found no real `:=`,
/// so the result still ends with `, let ident := sorry` etc.
fn strip_trailing_let(src: &str) -> String {
    let trimmed = src.trim_end();
    // Only apply if the string ends with `:= sorry` AND the `:=` is part of a `let`
    if !trimmed.ends_with(":= sorry") {
        return src.to_string();
    }
    // Look for specific trailing patterns:
    // `, let ...  := sorry` or ` let ... := sorry` at the very end
    let patterns = [", let ", " let "];
    for pat in &patterns {
        if let Some(pos) = trimmed.rfind(pat) {
            let after_let = &trimmed[pos + pat.len()..];
            // Should be: `<ident> := sorry` or `_ := sorry` or `<ident> : <type> := sorry`
            // Count `:=` occurrences in after_let — should be exactly 1
            let assign_count = after_let.matches(":=").count();
            if assign_count == 1 {
                let before_let = trimmed[..pos].trim_end();
                // Also strip `by` keyword if it precedes
                let before_let = before_let
                    .strip_suffix(" by")
                    .unwrap_or(before_let)
                    .trim_end();
                let before_let = before_let
                    .strip_suffix(',')
                    .unwrap_or(before_let)
                    .trim_end();
                return format!("{before_let} := sorry");
            }
        }
    }
    src.to_string()
}

/// Strip `match ... with` in type position — replace the match expression with `_`.
/// Also handles `match EXPR := sorry` where the `with` clause was already stripped.
fn normalize_match_in_type(src: &str) -> String {
    if !src.contains("match ") {
        return src.to_string();
    }
    // Only strip match that appears BEFORE `:=`
    let assign_pos = src.find(":=").unwrap_or(src.len());
    let before_assign = &src[..assign_pos];
    if !before_assign.contains("match ") {
        return src.to_string();
    }
    // Find `match ` as a keyword (not part of another identifier)
    let match_pos = find_match_keyword(before_assign);
    if let Some(match_pos) = match_pos {
        // Try to find ` with` after the match
        if let Some(with_rel) = find_keyword_depth0(&before_assign[match_pos..], " with") {
            let before_match = &src[..match_pos];
            let after_with = &src[match_pos + with_rel + 5..];
            return format!("{before_match}_ {after_with}");
        }
        // No `with` found — match in type without its with-clause (stripped by earlier passes).
        // Replace `match EXPR` with `_` (everything from `match` to `:=` or end-of-type).
        let before_match = &src[..match_pos];
        let after_assign = &src[assign_pos..];
        return format!("{}_ {}", before_match.trim_end(), after_assign);
    }
    src.to_string()
}

/// Find `match ` as a standalone keyword (not part of a longer identifier).
/// Returns the byte position of `match ` if found at a word boundary.
fn find_match_keyword(src: &str) -> Option<usize> {
    let bytes = src.as_bytes();
    let len = bytes.len();
    let pattern = b"match ";
    let plen = pattern.len();
    let mut i = 0;
    while i + plen <= len {
        if &bytes[i..i + plen] == pattern {
            // Check that the character before `match` is not alphanumeric or `_`
            if i == 0 || !(bytes[i - 1].is_ascii_alphanumeric() || bytes[i - 1] == b'_') {
                return Some(i);
            }
        }
        i += 1;
    }
    None
}

/// Normalize `show ... by ...` patterns in proof bodies.
/// After proof replacement, some `show P by tactic` remain. Replace with `sorry`.
fn normalize_show_by(src: &str) -> String {
    if !src.contains(" show ") {
        return src.to_string();
    }
    // If we already have `:= sorry` and then ` show ... by ...`, just truncate
    if let Some(sorry_pos) = src.find(":= sorry") {
        let after_sorry = &src[sorry_pos + 8..];
        if after_sorry.contains(" show ") {
            return src[..sorry_pos + 8].to_string();
        }
    }
    // If `:= show ... by ...` → `:= sorry`
    if let Some(pos) = src.find(":= show ") {
        return format!("{} := sorry", src[..pos].trim_end());
    }
    src.to_string()
}

/// Normalize `if cond then e1 else e2` in type position to `(ite cond e1 e2)`.
/// Only handles simple cases where if/then/else appear before `:=`.
fn normalize_if_then_else_in_type(src: &str) -> String {
    if !src.contains(" if ") || !src.contains(" then ") {
        return src.to_string();
    }
    // If the `if` is after `:= sorry`, it's already in proof position — skip
    if let Some(sorry_pos) = src.find(":= sorry") {
        let before_sorry = &src[..sorry_pos];
        if !before_sorry.contains(" if ") {
            return src.to_string();
        }
    }
    // Simple approach: find `if COND then THEN_EXPR else ELSE_EXPR` at depth 0
    // and replace with `(ite COND THEN_EXPR ELSE_EXPR)`
    let mut result = src.to_string();
    // Handle one occurrence at a time (up to 5 iterations)
    for _ in 0..5 {
        if let Some(if_pos) = result.find(" if ") {
            let after_if = &result[if_pos + 4..];
            if let Some(then_rel) = find_keyword_depth0(after_if, " then ") {
                let cond = &after_if[..then_rel];
                let after_then = &after_if[then_rel + 6..];
                if let Some(else_rel) = find_keyword_depth0(after_then, " else ") {
                    let then_expr = &after_then[..else_rel];
                    let else_start = then_rel + 6 + else_rel + 6;
                    // Find end of else expression (next `:=`, `,`, or end)
                    let else_end = find_expr_end(&after_if[else_start..]);
                    let else_expr = &after_if[else_start..else_start + else_end];
                    let replacement = format!(
                        " (ite {} {} {})",
                        cond.trim(),
                        then_expr.trim(),
                        else_expr.trim()
                    );
                    let full_end = if_pos + 4 + else_start + else_end;
                    result = format!(
                        "{}{}{}",
                        &result[..if_pos],
                        replacement,
                        &result[full_end..]
                    );
                    continue;
                }
            }
        }
        break;
    }
    result
}

/// Find a keyword at depth 0 in `src`. Returns byte offset or None.
fn find_keyword_depth0(src: &str, keyword: &str) -> Option<usize> {
    let mut depth = 0usize;
    let bytes = src.as_bytes();
    let kw_bytes = keyword.as_bytes();
    let kw_len = kw_bytes.len();
    let len = bytes.len();
    let mut i = 0;
    while i + kw_len <= len {
        match bytes[i] {
            b'(' | b'[' | b'{' => {
                depth += 1;
                i += 1;
            }
            b')' | b']' | b'}' => {
                depth = depth.saturating_sub(1);
                i += 1;
            }
            _ if depth == 0 && &bytes[i..i + kw_len] == kw_bytes => {
                return Some(i);
            }
            _ => {
                i += 1;
            }
        }
    }
    None
}

/// Find end of an expression — stop at `:=`, `,` at depth 0, or end of string.
fn find_expr_end(src: &str) -> usize {
    let mut depth = 0usize;
    let bytes = src.as_bytes();
    let len = bytes.len();
    let mut i = 0;
    while i < len {
        match bytes[i] {
            b'(' | b'[' | b'{' => {
                depth += 1;
                i += 1;
            }
            b')' | b']' | b'}' => {
                if depth == 0 {
                    return i;
                }
                depth -= 1;
                i += 1;
            }
            b':' if depth == 0 && i + 1 < len && bytes[i + 1] == b'=' => {
                return i;
            }
            b',' if depth == 0 => {
                return i;
            }
            _ => {
                i += 1;
            }
        }
    }
    len
}

fn fix_sorry_inside_parens(src: &str) -> String {
    if let Some(pos) = src.find(":= sorry") {
        let after = src[pos + 8..].trim();
        if !after.is_empty()
            && (after.starts_with(')') || after.starts_with(']') || after.starts_with('}'))
        {
            return src[..pos + 8].to_string();
        }
    }
    src.to_string()
}

/// Convert `{x : T} ->` patterns to `(x : T) ->` at the type level.
///
/// In Lean 4, `{x : T} → body` means an implicit Pi type.
/// OxiLean doesn't support this syntax, so convert to explicit arrow.
fn normalize_implicit_arrow_binders(src: &str) -> String {
    if !src.contains('}') {
        return src.to_string();
    }
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut result = String::with_capacity(src.len());
    let mut i = 0;
    while i < len {
        if chars[i] == '{' {
            // Find the matching `}` and check if followed by ` ->`
            let start = i;
            let mut j = i + 1;
            let mut depth = 1usize;
            let mut has_colon = false;
            while j < len && depth > 0 {
                match chars[j] {
                    '{' => depth += 1,
                    '}' => depth -= 1,
                    ':' if depth == 1 => {
                        let next = if j + 1 < len { chars[j + 1] } else { ' ' };
                        if next != '=' {
                            has_colon = true;
                        }
                    }
                    _ => {}
                }
                j += 1;
            }
            if depth == 0 && has_colon {
                // Check if followed by ` ->` or ` → `
                let rest: String = chars[j..].iter().collect();
                let rest_trimmed = rest.trim_start();
                if rest_trimmed.starts_with("->") || rest_trimmed.starts_with('\u{2192}') {
                    let inner: String = chars[start + 1..j - 1].iter().collect();
                    result.push('(');
                    result.push_str(inner.trim());
                    result.push(')');
                    i = j;
                    continue;
                }
            }
            result.push(chars[i]);
            i += 1;
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}

/// Strip comparison bounds from fun binders: `fun (r > 0) ->` → `fun r ->`.
/// Also handles multi-binder: `fun (i) (j >= i) ->` → `fun (i) j ->`.
/// Also handles bare: `fun a ≤ c -> body` → `fun a -> body`.
/// BigInter/ISup etc. can produce bounded quantifiers inside fun binders.
fn normalize_fun_bounded_binders(src: &str) -> String {
    if !src.contains("fun ") {
        return src.to_string();
    }
    let mut result = src.to_string();
    let cmps: &[&str] = &[
        " > ",
        " < ",
        " >= ",
        " <= ",
        " != ",
        " Mem ",
        " \u{2265} ",
        " \u{2264} ",
    ];
    for cmp in cmps {
        loop {
            let changed = try_strip_fun_bounded_binder(&mut result, cmp);
            if !changed {
                break;
            }
        }
    }
    result
}

/// Try to strip one bounded binder from a fun expression.
/// Returns true if a change was made.
fn try_strip_fun_bounded_binder(result: &mut String, cmp: &str) -> bool {
    let cmp_pos = match result.find(cmp) {
        Some(p) => p,
        None => return false,
    };
    // Case 1: Parenthesized — `(ident CMP expr)` after fun context
    let before = result[..cmp_pos].to_string();
    if let Some(paren_pos) = before.rfind('(') {
        let ident_str = before[paren_pos + 1..].trim().to_string();
        if !ident_str.is_empty()
            && ident_str
                .chars()
                .all(|c| c.is_alphanumeric() || c == '_' || c == '\'')
        {
            let before_paren = result[..paren_pos].trim_end().to_string();
            let is_fun_context = before_paren.ends_with("fun")
                || before_paren.ends_with(')')
                || before_paren.ends_with(']');
            if is_fun_context {
                let after_cmp = result[cmp_pos + cmp.len()..].to_string();
                let mut depth = 0usize;
                let mut close_pos = None;
                for (i, ch) in after_cmp.char_indices() {
                    match ch {
                        '(' | '[' | '{' => depth += 1,
                        ')' if depth == 0 => {
                            close_pos = Some(i);
                            break;
                        }
                        ')' | ']' | '}' => depth = depth.saturating_sub(1),
                        _ => {}
                    }
                }
                if let Some(cp) = close_pos {
                    let full_end = cmp_pos + cmp.len() + cp + 1;
                    *result = format!(
                        "{}{} {}",
                        &result[..paren_pos],
                        ident_str,
                        &result[full_end..]
                    );
                    return true;
                }
            }
        }
    }
    // Case 2: Bare — `fun ident CMP expr ->` (no parens around the bounded binder)
    let before_trimmed = before.trim_end().to_string();
    if let Some(sp) = before_trimmed.rfind(' ') {
        let ident_str = before_trimmed[sp + 1..].to_string();
        let before_ident = before_trimmed[..sp].trim_end().to_string();
        if !ident_str.is_empty()
            && ident_str
                .chars()
                .all(|c| c.is_alphanumeric() || c == '_' || c == '\'')
            && before_ident.ends_with("fun")
        {
            let after_cmp = result[cmp_pos + cmp.len()..].to_string();
            if let Some(arrow_pos) = after_cmp.find("->") {
                let ident_byte_start = {
                    // Find byte position of ident in original result
                    let trimmed_start = before.trim_end().len() - before_trimmed.len();
                    let _ = trimmed_start;
                    sp + 1
                };
                let full_end = cmp_pos + cmp.len() + arrow_pos;
                *result = format!(
                    "{}{} {}",
                    &result[..ident_byte_start],
                    ident_str,
                    &result[full_end..]
                );
                return true;
            }
        }
    }
    false
}

/// Normalize destructuring binders: `fun ((a, b) : T) ->` → `fun (ab_ : T) ->`
/// Lean 4 allows tuple destructuring in lambda binders; OxiLean does not.
fn normalize_destructuring_binders(src: &str) -> String {
    let mut result = src.to_string();
    // Handle both `fun ((ident, ident, ...) : T) ->` and `forall ((ident, ident, ...) : T), body`
    for keyword in &["fun", "forall", "\u{2200}"] {
        let pattern = format!("{keyword} ((");
        let pat_len = pattern.len();
        loop {
            let Some(kw_pos) = result.find(&pattern) else {
                break;
            };
            let after = &result[kw_pos + pat_len..]; // after "keyword (("
                                                     // Find matching `)` for inner parens
            let mut depth = 1usize;
            let mut inner_close = None;
            for (i, ch) in after.char_indices() {
                match ch {
                    '(' => depth += 1,
                    ')' => {
                        depth -= 1;
                        if depth == 0 {
                            inner_close = Some(i);
                            break;
                        }
                    }
                    _ => {}
                }
            }
            if let Some(ic) = inner_close {
                let inner = &after[..ic]; // e.g. "r, m"
                let rest_after_inner = &after[ic + 1..]; // after inner ")"
                                                         // Check if followed by ` : T) ->` or just `)`
                if rest_after_inner.starts_with(" : ") || rest_after_inner.starts_with(')') {
                    // Generate replacement name from inner idents
                    let replacement_name: String = inner
                        .split(',')
                        .map(|s| s.trim())
                        .filter(|s| !s.is_empty())
                        .collect::<Vec<_>>()
                        .join("_")
                        + "_";
                    let old = format!("{keyword} (({inner})");
                    let new = format!("{keyword} ({replacement_name}");
                    result = result.replacen(&old, &new, 1);
                } else {
                    break; // don't loop forever
                }
            } else {
                break;
            }
        }
    }
    result
}
/// Convert `{binders}` to `(binders)` when they appear in forall/exists binder position.
/// E.g., `∀ (n : _) {α β : T}, body` → `∀ (n : _) (α β : T), body`
fn normalize_implicit_forall_binders(src: &str) -> String {
    if !src.contains('{') {
        return src.to_string();
    }
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut result = String::with_capacity(src.len());
    let mut i = 0;
    while i < len {
        if chars[i] == '{' {
            // Check if we're in a forall/exists binder context:
            // Look backward for ∀/forall/exists, possibly with intervening binders
            let trimmed = result.trim_end();
            let in_quantifier_context = trimmed.ends_with(')')
                || trimmed.ends_with('\u{2200}')
                || trimmed.ends_with("forall")
                || trimmed.ends_with("exists");
            if in_quantifier_context {
                // Check if this is a typed binder (has `:` at depth 1)
                let start = i;
                let mut j = i + 1;
                let mut depth = 1usize;
                let mut has_colon = false;
                while j < len && depth > 0 {
                    match chars[j] {
                        '{' => depth += 1,
                        '}' => {
                            depth -= 1;
                        }
                        ':' if depth == 1 => {
                            let next = if j + 1 < len { chars[j + 1] } else { ' ' };
                            if next != '=' {
                                has_colon = true;
                            }
                        }
                        _ => {}
                    }
                    j += 1;
                }
                if depth == 0 && has_colon {
                    // Convert {binders} → (binders)
                    let inner: String = chars[start + 1..j - 1].iter().collect();
                    result.push('(');
                    result.push_str(inner.trim());
                    result.push(')');
                    i = j;
                    continue;
                }
            }
            result.push(chars[i]);
            i += 1;
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}
/// Rename keyword field accesses like `.then` to `.then_` to avoid parser keyword clashes.
/// Only renames when preceded by `.` (field access context).
fn rename_keyword_fields(src: &str) -> String {
    // Keywords that might appear as Lean4 method/field names
    let keywords = [".then", ".end", ".from", ".Type", ".Func"];
    let mut s = src.to_string();
    for kw in &keywords {
        let replacement = format!("{}_", kw);
        s = s.replace(kw, &replacement);
    }
    s
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
                if next.is_whitespace()
                    || next == ')'
                    || next == '}'
                    || next == ']'
                    || next == ','
                    || next == '-'
                    || next == '\0'
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
/// Also handles general `(· EXPR ·)` patterns with complex operators.
fn normalize_dot_anonymous_fn(src: &str) -> String {
    let ops = ["<", "≤", ">", "≥", "+", "-", "*", "=", "≠", "∧", "∨", "/"];
    let mut s = src.to_string();
    for op in &ops {
        let pattern = format!("(· {} ·)", op);
        let replacement = format!("(fun x y -> x {} y)", op);
        s = s.replace(&pattern, &replacement);
    }
    // General handler: (· COMPLEX_OP ·) where COMPLEX_OP contains brackets etc.
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    let mut result = String::with_capacity(s.len() + 32);
    let mut i = 0;
    while i < len {
        if chars[i] == '(' && i + 3 < len && chars[i + 1] == '\u{00B7}' && chars[i + 2] == ' ' {
            // Find matching closing paren
            let mut depth = 1usize;
            let mut j = i + 3;
            let mut close = None;
            while j < len {
                match chars[j] {
                    '(' | '[' | '{' => depth += 1,
                    ')' => {
                        depth -= 1;
                        if depth == 0 {
                            close = Some(j);
                            break;
                        }
                    }
                    ']' | '}' => depth = depth.saturating_sub(1),
                    _ => {}
                }
                j += 1;
            }
            if let Some(cp) = close {
                let inner: String = chars[i + 3..cp].iter().collect();
                let inner = inner.trim();
                if inner.ends_with('\u{00B7}') || inner.ends_with("dot") {
                    // (· OP ·) binary
                    let op_part = inner
                        .trim_end_matches('\u{00B7}')
                        .trim_end_matches("dot")
                        .trim();
                    result.push_str(&format!("(fun x y -> x {} y)", op_part));
                } else {
                    // (· OP) unary postfix
                    result.push_str(&format!("(fun x -> x {})", inner));
                }
                i = cp + 1;
                continue;
            }
        }
        result.push(chars[i]);
        i += 1;
    }
    result
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
        // No `in` or `partial_d` separator found. Try direct comma: `∀ᵐ x, body`
        let mut found_direct_comma = false;
        {
            let mut depth = 0usize;
            for (ci, ch) in after_q.char_indices() {
                match ch {
                    '(' | '[' | '{' => depth += 1,
                    ')' | ']' | '}' => depth = depth.saturating_sub(1),
                    ',' if depth == 0 => {
                        let var_part = after_q[..ci].trim();
                        let body = &after_q[ci + 1..];
                        let binders = normalize_filter_var_part(var_part);
                        result.push_str(keyword);
                        result.push(' ');
                        result.push_str(&binders);
                        result.push(',');
                        rest = body;
                        found_direct_comma = true;
                        break;
                    }
                    _ => {}
                }
            }
        }
        if !found_direct_comma {
            result.push_str(keyword);
            result.push_str("_filter ");
            rest = after_q;
        }
    }
    result.push_str(rest);
    result
}
/// Convert a filter quantifier variable part to forall binders.
/// `"a"` → `"(a : _)"`, `"(x) (y)"` → `"(x : _) (y : _)"`, `"(x : T)"` → `"(x : T)"`.
fn normalize_filter_var_part(vars: &str) -> String {
    let trimmed = vars.trim();
    // If the whole var_part is a typed binder like `c : K`, wrap it directly
    if !trimmed.starts_with('(') && trimmed.contains(" : ") {
        return format!("({})", trimmed);
    }
    let mut result = String::new();
    let mut rest = trimmed;
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
            if end == 0 {
                // Unmatched paren — just push remaining as-is and break
                if !result.is_empty() {
                    result.push(' ');
                }
                result.push_str(rest);
                break;
            }
            let inner = &rest[1..end - 1];
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
/// Strip filtered tprod/tsum: `∏'[L] b, f b` → `TProd b, f b`, `∑'[L] b, f b` → `Tsum b, f b`
fn strip_filtered_tprod_tsum(src: &str) -> String {
    let mut result = src.to_string();
    for (pat, repl) in [("\u{220F}'", " TProd "), ("\u{2211}'", " Tsum ")] {
        while let Some(pos) = result.find(&format!("{pat}[")) {
            let after_bracket = pos + pat.len() + 1; // position after '['
                                                     // Find matching ']'
            let mut depth = 1;
            let mut end = after_bracket;
            for ch in result[after_bracket..].chars() {
                match ch {
                    '[' => depth += 1,
                    ']' => {
                        depth -= 1;
                        if depth == 0 {
                            break;
                        }
                    }
                    _ => {}
                }
                end += ch.len_utf8();
            }
            if depth == 0 {
                // Replace ∏'[...] with TProd
                result = format!("{}{}{}", &result[..pos], repl, &result[end + 1..]);
            } else {
                break;
            }
        }
    }
    result
}
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
    for op in &[
        "BigProd",
        "BigSum",
        "Tsum",
        "TProd",
        "Integral",
        "IntegralInv",
        "AvgIntegral",
        "AvgIntegralInv",
        "ContourIntegral",
        "CurveIntegral",
        "BigTensorProd",
        "BigDirectSum",
        "DFinsupp",
    ] {
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
                result.push_str(pat.as_str());
                result.push_str(&after_trim[..end]);
                let skip = after.len() - after_trim.len();
                rest = &after[skip + end..];
                continue;
            }
        }
        // Handle bare `fun` (without outer parens) after big operator:
        // `BigDirectSum fun (i : T) -> body` → `BigDirectSum (fun (i : T) -> body)`
        if after_trim.starts_with("fun ") {
            // Use char_indices to get correct byte offsets for multi-byte chars
            let mut depth = 0usize;
            let mut end_byte = 0usize;
            let mut found = false;
            for (byte_idx, ch) in after_trim.char_indices() {
                match ch {
                    '(' | '[' | '{' => {
                        depth += 1;
                    }
                    ')' | ']' | '}' if depth == 0 => {
                        found = true;
                        break;
                    }
                    ')' | ']' | '}' => {
                        depth = depth.saturating_sub(1);
                    }
                    ':' if depth == 0 => {
                        // Check next char for '='
                        let rest_after = &after_trim[byte_idx + ch.len_utf8()..];
                        if rest_after.starts_with('=') {
                            found = true;
                            break;
                        }
                    }
                    _ => {}
                }
                end_byte = byte_idx + ch.len_utf8();
            }
            if found || end_byte > 0 {
                let fun_expr = after_trim[..end_byte].trim_end();
                if !fun_expr.is_empty() {
                    result.push_str(before);
                    result.push_str(pat.as_str());
                    result.push('(');
                    result.push_str(fun_expr);
                    result.push(')');
                    let skip = after.len() - after_trim.len();
                    rest = &after[skip + end_byte..];
                    continue;
                }
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
/// If there's an odd number of ‖, unpaired ones become ` Fuzzy ` (game theory).
fn normalize_norm_notation(src: &str) -> String {
    let count = src.chars().filter(|&c| c == '\u{2016}').count();
    if count == 0 {
        return src.to_string();
    }
    // If odd count, replace ALL ‖ with Fuzzy (game theory context dominates)
    if count % 2 != 0 {
        return src.replace('\u{2016}', " Fuzzy ");
    }
    // Depth-based approach to handle nested norms like ‖fderiv ℝ (fun x => ‖f x‖ ^ p) x‖
    // Each ‖ is open if at even depth, close if at odd depth (depth tracks open norms).
    let mut result = String::with_capacity(src.len() + 16);
    let mut norm_depth: usize = 0;
    for ch in src.chars() {
        if ch == '\u{2016}' {
            if norm_depth == 0 {
                // Always open at depth 0
                result.push_str("(Norm ");
                norm_depth += 1;
            } else {
                // At depth > 0, check context: if preceded by operator/open-paren/space-arrow,
                // this is a nested open; otherwise it's a close.
                // Heuristic: look at what's before this ‖ in the result.
                // If last non-space char is an operator or open-bracket, it's an inner open.
                let trimmed = result.trim_end();
                let last_ch = trimmed.chars().last().unwrap_or(' ');
                if matches!(last_ch, '(' | '[' | ',' | '>' | '=' | '+' | '-' | '*' | '/')
                    || trimmed.ends_with("->")
                    || trimmed.ends_with("=>")
                    || trimmed.ends_with("Norm")
                {
                    // Nested open
                    result.push_str("(Norm ");
                    norm_depth += 1;
                } else {
                    // Close
                    result.push(')');
                    norm_depth -= 1;
                }
            }
        } else {
            result.push(ch);
        }
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
            && matches!(chars[i - 1], ' ' | '(' | '[' | ',' | '=' | '-')
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
                    '\n' | ';' => break,
                    ':' if depth == 0 && j + 1 < len && chars[j + 1] == '=' => break,
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
/// Normalize `|expr|ₘ` monoid absolute value → `(mabs expr)`.
/// Must run BEFORE `ₘ → m` replacement.
fn normalize_mabs_notation(src: &str) -> String {
    if !src.contains('\u{2098}') || !src.contains('|') {
        return src.to_string();
    }
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut result = String::with_capacity(src.len() + 16);
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
                        // Check if followed by ₘ
                        if j + 1 < len && chars[j + 1] == '\u{2098}' {
                            found_close = Some(j);
                        }
                        break;
                    }
                    '\n' | ';' => break,
                    ',' if depth == 0 => break,
                    ':' if depth == 0
                        && j + 1 < len
                        && chars[j + 1] == '='
                        && (j < 2 || chars[j - 1] != '!') =>
                    {
                        break;
                    }
                    _ => {
                        j += 1;
                    }
                }
            }
            if let Some(close) = found_close {
                let inner: String = chars[i + 1..close].iter().collect();
                let inner_trimmed = inner.trim();
                if !inner_trimmed.is_empty() {
                    result.push_str(&format!("(mabs {})", inner_trimmed));
                    i = close + 2; // skip |close| + ₘ
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
                            // Strip outer parens to avoid double-wrapping: (B : Set α) → B : Set α
                            let binder_inner = if binder.starts_with('(') && binder.ends_with(')') {
                                &binder[1..binder.len() - 1]
                            } else {
                                &binder
                            };
                            result
                                .push_str(&format!("(SetOf (fun ({}) -> {}))", binder_inner, body));
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
/// Normalize postfix `#` (primorial): `n#` → `(Primorial n)`.
#[allow(dead_code)]
fn normalize_postfix_primorial(src: &str) -> String {
    if !src.contains('#') {
        return src.to_string();
    }
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut result = String::with_capacity(src.len() + 16);
    let mut i = 0;
    while i < len {
        if chars[i] == '#'
            && i > 0
            && (chars[i - 1].is_alphanumeric()
                || chars[i - 1] == '_'
                || chars[i - 1] == ')'
                || chars[i - 1] == '\'')
            && (i + 1 >= len || chars[i + 1] == ' ' || chars[i + 1] == ')' || chars[i + 1] == ',')
        {
            // Find the preceding word or parenthesized expression
            let result_chars: Vec<char> = result.chars().collect();
            let mut j = result_chars.len();
            while j > 0 && result_chars[j - 1] == ' ' {
                j -= 1;
            }
            let word_end = j;
            if j > 0 && result_chars[j - 1] == ')' {
                // Parenthesized: (expr)# → (Primorial (expr))
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
                    result.push_str(&format!("(Primorial {})", inner));
                } else {
                    result.push('#');
                }
            } else {
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
                    result.push_str(&format!("(Primorial {})", word));
                } else {
                    result.push('#');
                }
            }
            i += 1;
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}

/// Strip typeclass instance brackets containing forall: `[∀ i : T, P i]` → empty.
fn strip_typeclass_forall_brackets(src: &str) -> String {
    if !src.contains('[') {
        return src.to_string();
    }
    let mut result = String::with_capacity(src.len());
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut i = 0;
    while i < len {
        if chars[i] == '[' {
            // Check if we're in a forall/exists binder context:
            // look backward for `forall`/`∀`/`)` possibly with spaces
            let trimmed = result.trim_end();
            let in_quantifier = trimmed.ends_with(')')
                || trimmed.ends_with('\u{2200}')
                || trimmed.ends_with("forall")
                || trimmed.ends_with("exists");
            if in_quantifier {
                // Check what's inside the bracket
                let bracket_start = i;
                let mut depth = 1usize;
                let mut j = i + 1;
                while j < len && depth > 0 {
                    match chars[j] {
                        '[' => depth += 1,
                        ']' => depth -= 1,
                        _ => {}
                    }
                    j += 1;
                }
                if depth == 0 {
                    let inner: String = chars[bracket_start + 1..j - 1].iter().collect();
                    // Strip typeclass instance binders: `[∀...]`, `[(expr)]`, `[inst : T]`
                    // These are instance binders that OxiLean doesn't handle
                    let inner_trimmed = inner.trim();
                    let is_typeclass_skip = inner_trimmed.starts_with('\u{2200}')
                        || inner_trimmed.starts_with("forall")
                        || inner_trimmed.starts_with('(')
                        || !inner_trimmed.contains(':');
                    if is_typeclass_skip {
                        // Skip the entire bracket group
                        i = j;
                        continue;
                    }
                    // Named typeclass binder `[inst : T]` → convert to `(inst : T)`
                    if inner_trimmed.contains(':') {
                        result.push('(');
                        result.push_str(inner_trimmed);
                        result.push(')');
                        i = j;
                        continue;
                    }
                }
            }
        }
        result.push(chars[i]);
        i += 1;
    }
    result
}

/// Strip remaining `[ClassName Arg1 Arg2]` instance brackets that weren't caught
/// by `strip_typeclass_forall_brackets` (which only triggers in quantifier context).
/// These are typeclass instance binders that appear after type names or in other positions.
/// Only strips brackets whose content starts with an uppercase letter (typeclass name pattern)
/// and contains no commas at depth 0 (to avoid stripping list literals).
#[allow(dead_code)]
fn strip_remaining_instance_brackets(src: &str) -> String {
    if !src.contains('[') {
        return src.to_string();
    }
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut result = String::with_capacity(src.len());
    let mut i = 0;
    while i < len {
        if chars[i] == '[' {
            let bracket_start = i;
            let mut depth = 1usize;
            let mut j = i + 1;
            let mut has_comma_at_depth1 = false;
            while j < len && depth > 0 {
                match chars[j] {
                    '[' => depth += 1,
                    ']' => depth -= 1,
                    ',' if depth == 1 => has_comma_at_depth1 = true,
                    _ => {}
                }
                j += 1;
            }
            if depth == 0 && !has_comma_at_depth1 {
                let inner: String = chars[bracket_start + 1..j - 1].iter().collect();
                let inner_trimmed = inner.trim();
                let starts_upper = inner_trimmed
                    .chars()
                    .next()
                    .is_some_and(|c| c.is_uppercase());
                // Named instance: [inst : ClassName ...] — lowercase name, uppercase type
                let is_named_instance = !starts_upper
                    && inner_trimmed.contains(':')
                    && inner_trimmed.split_once(':').is_some_and(|(_, ty)| {
                        ty.trim_start()
                            .chars()
                            .next()
                            .is_some_and(|c| c.is_uppercase())
                    });
                if starts_upper || is_named_instance {
                    // Instance binder — strip or convert
                    if inner_trimmed.contains(':') {
                        // Named: [inst : T] → (inst : T)
                        result.push('(');
                        result.push_str(inner_trimmed);
                        result.push(')');
                    }
                    // else: unnamed [ClassName args], just skip entirely
                    i = j;
                    continue;
                }
            }
        }
        result.push(chars[i]);
        i += 1;
    }
    result
}

/// Strip `(IDENT.IsXxx)` propositional condition binders in forall/exists binder positions.
/// Only strips short-ident dot-PascalField groups preceded by `)` and followed by binder chars.
/// Example: `forall (P : Ideal R) (P.IsMaximal), body` → `forall (P : Ideal R), body`
fn strip_prop_condition_binders(src: &str) -> String {
    if !src.contains('.') {
        return src.to_string();
    }
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut result = String::with_capacity(src.len());
    let mut i = 0;
    while i < len {
        if chars[i] == '(' && i + 1 < len {
            // Only in binder context: preceded by `)`
            let trimmed = result.trim_end();
            if trimmed.ends_with(')') {
                // Find matching close paren
                let mut depth = 1usize;
                let mut j = i + 1;
                let mut has_colon = false;
                let mut has_assign = false;
                let mut first_dot_pos: Option<usize> = None;
                while j < len && depth > 0 {
                    match chars[j] {
                        '(' => depth += 1,
                        ')' => depth -= 1,
                        '.' if depth == 1 && first_dot_pos.is_none() => {
                            first_dot_pos = Some(j);
                        }
                        ':' if depth == 1 && j + 1 < len && chars[j + 1] == '=' => {
                            has_assign = true;
                        }
                        ':' if depth == 1 => has_colon = true,
                        _ => {}
                    }
                    j += 1;
                }
                if depth == 0 && !has_colon && !has_assign {
                    if let Some(dot_pos) = first_dot_pos {
                        let ident: String = chars[i + 1..dot_pos].iter().collect();
                        let ident = ident.trim();
                        let field: String = chars[dot_pos + 1..j - 1].iter().collect();
                        let field = field.trim();
                        // Short ident (≤5 chars) and field starts uppercase
                        if !ident.is_empty()
                            && ident.len() <= 5
                            && ident.chars().all(|c| c.is_alphanumeric() || c == '_')
                            && field.chars().next().is_some_and(|c| c.is_uppercase())
                        {
                            // Check followed by `,`, `)`, `(` (binder chain)
                            let next_char = chars[j..]
                                .iter()
                                .copied()
                                .find(|c| !c.is_whitespace())
                                .unwrap_or(' ');
                            if matches!(next_char, ')' | ',' | '(' | '[') {
                                let tlen = result.trim_end().len();
                                if tlen < result.len() {
                                    result.truncate(tlen);
                                    result.push(' ');
                                }
                                i = j;
                                continue;
                            }
                        }
                    }
                }
            }
        }
        result.push(chars[i]);
        i += 1;
    }
    result
}

/// Strip `(∀ ...)` and `(∃ ...)` binder groups in forall/exists binder positions.
/// Only strips when preceded by `)` (after another binder group) and followed by
/// another binder `(`, `)`, or `,` — confirming we're in a binder chain.
/// Example: `forall (f : T) (∀ (X : C), IsIso (f.app X)), P` → `forall (f : T), P`
fn strip_quantifier_binder_groups(src: &str) -> String {
    if !src.contains("(\u{2200}") && !src.contains("(forall") && !src.contains("(\u{2203}") {
        return src.to_string();
    }
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut result = String::with_capacity(src.len());
    let mut i = 0;
    while i < len {
        if chars[i] == '(' && i + 1 < len {
            let next = chars[i + 1];
            // Check for `(∀`, `(∃`, or `(forall`
            let is_quant_start = next == '\u{2200}'
                || next == '\u{2203}'
                || (i + 7 < len && chars[i + 1..i + 8].iter().collect::<String>() == "forall ");
            if is_quant_start {
                // Only strip if preceded by `)` (binder chain context)
                let trimmed = result.trim_end();
                let in_binder_chain = trimmed.ends_with(')')
                    || trimmed.ends_with('\u{2200}')
                    || trimmed.ends_with("forall");
                if in_binder_chain {
                    // Find the matching closing paren
                    let mut depth = 1usize;
                    let mut j = i + 1;
                    while j < len && depth > 0 {
                        match chars[j] {
                            '(' => depth += 1,
                            ')' => depth -= 1,
                            _ => {}
                        }
                        j += 1;
                    }
                    if depth == 0 {
                        // Verify followed by `)`, `,`, `(`, or `->` (binder chain continuation)
                        let next_char = chars[j..]
                            .iter()
                            .copied()
                            .find(|c| !c.is_whitespace())
                            .unwrap_or(' ');
                        if matches!(next_char, ')' | ',' | '(' | '[' | '{') {
                            // Strip this group
                            let trimmed_len = result.trim_end().len();
                            if trimmed_len < result.len() {
                                result.truncate(trimmed_len);
                                result.push(' ');
                            }
                            i = j;
                            continue;
                        }
                    }
                }
            }
        }
        result.push(chars[i]);
        i += 1;
    }
    result
}

/// Normalize `{{expr}}` double braces → `(singleton (singleton expr))`.
fn normalize_double_braces(src: &str) -> String {
    if !src.contains("{{") {
        return src.to_string();
    }
    let mut s = src.to_string();
    // Repeatedly replace innermost {{ ... }} with (singleton_inner ...)
    while let Some(pos) = s.find("{{") {
        let after = &s[pos + 2..];
        // Find matching }}
        let mut depth = 0usize;
        let mut close = None;
        for (i, ch) in after.char_indices() {
            match ch {
                '{' => depth += 1,
                '}' if depth == 0 => {
                    if i + 1 < after.len() && after.as_bytes()[i + 1] == b'}' {
                        close = Some(i);
                        break;
                    } else {
                        break;
                    }
                }
                '}' => depth -= 1,
                _ => {}
            }
        }
        if let Some(cp) = close {
            let inner = after[..cp].trim();
            s = format!(
                "{}(singleton (singleton {})){}",
                &s[..pos],
                inner,
                &after[cp + 2..]
            );
        } else {
            break;
        }
    }
    s
}

/// Normalize `∗` (U+2217): postfix Kleene star `l∗` → `l_Star`, otherwise ` * `.
fn normalize_kleene_star(src: &str) -> String {
    if !src.contains('\u{2217}') {
        return src.to_string();
    }
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut result = String::with_capacity(src.len() + 16);
    let mut i = 0;
    while i < len {
        if chars[i] == '\u{2217}' {
            let prev_is_word = i > 0
                && (chars[i - 1].is_alphanumeric()
                    || chars[i - 1] == '_'
                    || chars[i - 1] == '\''
                    || chars[i - 1] == ')');
            let next_is_word = i + 1 < len
                && (chars[i + 1].is_alphanumeric() || chars[i + 1] == '_' || chars[i + 1] == '(');
            if prev_is_word {
                result.push_str("_Star");
            } else if next_is_word {
                // Prefix ∗x (nimber notation) → (nimStar x)
                result.push_str("nimStar ");
            } else {
                result.push_str(" * ");
            }
            i += 1;
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}

/// Normalize exterior algebra arrow notation: `[⋀^ι]→ₗ[R]` → `LinearMap`.
/// Strips the `[⋀^...]` prefix and lets the remaining `→ₗ[R]` be handled by arrow normalization.
fn normalize_exterior_arrow(src: &str) -> String {
    if !src.contains('\u{22C0}') {
        return src.to_string();
    }
    let mut result = String::with_capacity(src.len());
    let mut rest = src;
    while let Some(pos) = rest.find('[') {
        // Check if next char is ⋀
        let after_bracket = &rest[pos + 1..];
        if after_bracket.starts_with('\u{22C0}') {
            result.push_str(&rest[..pos]);
            // Find closing ]
            if let Some(close) = after_bracket.find(']') {
                rest = &after_bracket[close + 1..];
            } else {
                rest = after_bracket;
            }
        } else {
            result.push_str(&rest[..pos + 1]);
            rest = after_bracket;
        }
    }
    result.push_str(rest);
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
fn strip_have_instances(src: &str) -> String {
    let mut result = String::with_capacity(src.len());
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut i = 0;
    while i < len {
        // Match `haveI` or `letI` followed by non-alphanumeric
        let (is_kw, kw_len) = if i + 5 <= len
            && chars[i..i + 5].iter().collect::<String>() == "haveI"
            && (i + 5 >= len || !chars[i + 5].is_alphanumeric())
        {
            (true, 5)
        } else if i + 4 <= len
            && chars[i..i + 4].iter().collect::<String>() == "letI"
            && (i + 4 >= len || !chars[i + 4].is_alphanumeric())
        {
            (true, 4)
        } else {
            (false, 0)
        };
        let prev_not_word = i == 0 || !chars[i - 1].is_alphanumeric();
        if is_kw && prev_not_word {
            let kw_name: String = chars[i..i + kw_len].iter().collect();
            i += kw_len;
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
                result.push_str(&kw_name);
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
/// Strip named arguments: `(name := val)` → `(val)`.
///
/// Lean 4 supports named arguments in function calls like `map (f₂ := f₁)`.
/// OxiLean does not support this syntax, so we strip the `name := ` part.
fn strip_named_args(src: &str) -> String {
    if !src.contains(":= ") {
        return src.to_string();
    }
    let mut result = String::with_capacity(src.len());
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut i = 0;
    while i < len {
        // Look for pattern: `(ident := ` where ident is alphanumeric/underscore
        if chars[i] == '(' && i + 1 < len {
            let start = i;
            let mut j = i + 1;
            // skip spaces
            while j < len && chars[j] == ' ' {
                j += 1;
            }
            // read identifier
            let id_start = j;
            while j < len
                && (chars[j].is_alphanumeric()
                    || chars[j] == '_'
                    || chars[j] == '\u{2080}'
                    || chars[j] == '\u{2081}'
                    || chars[j] == '\u{2082}')
            {
                j += 1;
            }
            let id_end = j;
            if id_end > id_start {
                // skip spaces
                while j < len && chars[j] == ' ' {
                    j += 1;
                }
                // check for `:=`
                if j + 1 < len && chars[j] == ':' && chars[j + 1] == '=' {
                    j += 2;
                    // skip spaces
                    while j < len && chars[j] == ' ' {
                        j += 1;
                    }
                    // emit `(` then continue from the value
                    result.push('(');
                    i = j;
                    continue;
                }
            }
            // Not a named arg pattern
            result.push(chars[start]);
            i = start + 1;
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
        // Match `.N` after alphanumeric, _, ', ) — direct field access like `p.1`
        let direct_access = chars[i] == '.'
            && i > 0
            && (chars[i - 1].is_alphanumeric()
                || chars[i - 1] == '_'
                || chars[i - 1] == '\''
                || chars[i - 1] == ')')
            && i + 1 < len
            && chars[i + 1].is_ascii_digit();
        // Match ` .N` after a word char — spaced field access like `p Inv .1`
        // (occurs when ⁻¹ is normalized to ` Inv `)
        let spaced_access = chars[i] == '.'
            && i >= 2
            && chars[i - 1] == ' '
            && (chars[i - 2].is_alphanumeric()
                || chars[i - 2] == '_'
                || chars[i - 2] == '\''
                || chars[i - 2] == ')')
            && i + 1 < len
            && chars[i + 1].is_ascii_digit();
        if direct_access || spaced_access {
            if spaced_access {
                // Remove trailing space so `Inv .1` → `Inv_fst` (no space before suffix)
                if result.ends_with(' ') {
                    result.pop();
                }
            }
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
            // Consume chained field access after _fst/_snd/_trd/_nth: `.field` → `_field`
            while i < len && chars[i] == '.' && i + 1 < len && chars[i + 1].is_alphabetic() {
                i += 1; // skip '.'
                result.push('_');
                while i < len && (chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == '\'')
                {
                    result.push(chars[i]);
                    i += 1;
                }
            }
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
            let mut has_pipe = false;
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
                    '|' if depth == 1 => {
                        has_pipe = true;
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
            if depth == 0 && top_level_commas > 0 && !has_colon && !has_pipe {
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
/// Normalize Lean 4 metaprogramming syntax that OxiLean can't parse.
///
/// - Double backticks: `` ``name `` → `name`
/// - Single backtick: `` `name `` → `name`
/// - `Q(...)` quoting macro → `QExpr`
/// - `$ident` interpolation → `ident`
// Strip `let _ := expr;` patterns and `let _ := sorry` from types.
fn strip_let_underscore(src: &str) -> String {
    let mut result = src.to_string();
    // Pattern: `let _ := sorry` at the very end (after replace_proof_with_sorry)
    // This happens when `let _ := <instance>; RealType := proof` gets the first `:=` eaten
    let suffix = ", let _ := sorry";
    loop {
        let trimmed = result.trim_end();
        if let Some(stripped) = trimmed.strip_suffix(suffix) {
            result = stripped.to_string();
            continue;
        }
        break;
    }
    // Pattern: `let _ := <expr>;` in the middle of a type — strip the whole let binding
    // (up to and including the semicolon)
    while let Some(pos) = result.find("let _ := ") {
        // Find the semicolon that ends this let binding
        let after = &result[pos + "let _ := ".len()..];
        if let Some(semi) = after.find(';') {
            let end = pos + "let _ := ".len() + semi + 1;
            // Strip the let binding and the semicolon, plus any trailing whitespace
            let after_semi = result[end..].trim_start();
            result = format!("{}{}", &result[..pos], after_semi);
        } else {
            break;
        }
    }
    result
}

/// Strip equation-compiler-style match branches from declarations.
///
/// Lean 4 allows defining functions via pattern matching without `:=`:
///   `def f : T → U | pat1 => body1 | pat2 => body2`
///
/// After normalization, these become:
///   `def f : T -> U | pat1 -> body1 | pat2 -> body2`
///
/// or after partial proof replacement:
///   `theorem foo : T := sorry | pat -> body`
///
/// This function strips these branches, replacing with `:= sorry` if needed.
fn strip_equation_branches(src: &str) -> String {
    // If already has `:= sorry` followed by `| ... ->`, strip the trailing branches
    if let Some(sorry_pos) = src.find(":= sorry") {
        let after_sorry = src[sorry_pos + ":= sorry".len()..].trim_start();
        if after_sorry.starts_with('|') || after_sorry.starts_with("| ") {
            return src[..sorry_pos + ":= sorry".len()].to_string();
        }
    }

    // Also look for ` | ` at depth 0 BEFORE `:=` — equation branches in the type
    // e.g., `def factorial : T | (0, _) := sorry` → `def factorial : T := sorry`
    // Find `| ` at depth 0 that could be an equation branch
    let bytes = src.as_bytes();
    let len = bytes.len();
    let mut depth = 0usize;
    let mut i = 0;
    // Skip past the declaration keyword and name
    let starts_decl = src.starts_with("theorem ")
        || src.starts_with("lemma ")
        || src.starts_with("def ")
        || src.starts_with("axiom ");
    if !starts_decl {
        return src.to_string();
    }

    while i < len {
        match bytes[i] {
            b'(' | b'[' | b'{' => {
                depth += 1;
                i += 1;
            }
            b')' | b']' | b'}' => {
                depth = depth.saturating_sub(1);
                i += 1;
            }
            b'|' if depth == 0 && i > 0 => {
                // Check it's ` | ` (space before and after) — equation branch
                let prev_space = i > 0 && bytes[i - 1] == b' ';
                let next_space = i + 1 < len && (bytes[i + 1] == b' ' || bytes[i + 1] == b'(');
                if prev_space && next_space {
                    // Check that the remaining content has `->` — it's a branch
                    let rest = &src[i..];
                    if rest.contains("->") {
                        let before = src[..i].trim_end();
                        return format!("{before} := sorry");
                    }
                }
                i += 1;
            }
            _ => {
                i += 1;
            }
        }
    }
    src.to_string()
}

/// Strip Lean 4 line comments (`-- ...`) from declarations.
/// These appear when multi-line declarations contain comment lines that get joined.
/// Must be careful not to strip `:= sorry --` at end (but the comment is harmless there).
fn strip_line_comments(src: &str) -> String {
    if !src.contains("--") {
        return src.to_string();
    }
    let mut result = String::with_capacity(src.len());
    let mut rest = src;
    while let Some(pos) = rest.find("--") {
        // Check if this is inside a string literal or special operator (like `->`)
        // `--` is only a comment if preceded by whitespace or start-of-line
        if pos > 0 {
            let prev = rest.as_bytes()[pos - 1];
            if prev != b' ' && prev != b'\t' && prev != b'(' && prev != b'\n' {
                // Not a comment start (e.g., inside an identifier or operator)
                result.push_str(&rest[..pos + 2]);
                rest = &rest[pos + 2..];
                continue;
            }
        }
        // This is a line comment. Strip everything from `--` to end-of-line or end-of-string.
        result.push_str(&rest[..pos]);
        // Find the end of the comment (next newline or end of string)
        if let Some(nl) = rest[pos..].find('\n') {
            rest = &rest[pos + nl..];
        } else {
            // Comment goes to end of string
            rest = "";
            break;
        }
    }
    result.push_str(rest);
    result
}

fn normalize_metaprogramming(src: &str) -> String {
    // Strip double backticks first (``name → name)
    let s = src.replace("``", "");
    // Strip single backtick before identifiers (`name → name)
    let mut result = String::with_capacity(s.len());
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    let mut i = 0;
    while i < len {
        if chars[i] == '`' && i + 1 < len && (chars[i + 1].is_alphanumeric() || chars[i + 1] == '_')
        {
            // Skip the backtick, keep the identifier
            i += 1;
        } else if chars[i] == '$'
            && i + 1 < len
            && (chars[i + 1].is_alphabetic() || chars[i + 1] == '_' || chars[i + 1] == '(')
        {
            // Skip the $, keep the identifier or expression
            i += 1;
        } else if chars[i] == '$' {
            // Strip standalone $ (metaprogramming antiquotation)
            i += 1;
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    // Q(Type) → QExpr (simple replacement for quoting macro)
    let result = if result.contains("Q(") {
        let mut s = result;
        while let Some(pos) = s.find("Q(") {
            // Check it's not part of a larger word
            let prev_ok = pos == 0 || !s.as_bytes()[pos - 1].is_ascii_alphanumeric();
            if prev_ok {
                if let Some(close) = s[pos + 2..].find(')') {
                    s = format!("{}QExpr{}", &s[..pos], &s[pos + 2 + close + 1..]);
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        s
    } else {
        result
    };
    result
}

/// Normalize Lie bracket notation `⁅x, y⁆` → `(LieBracket x y)`.
fn normalize_lie_brackets(src: &str) -> String {
    // Process inside-out: keep running until no more ⁅...⁆ pairs
    let mut s = src.to_string();
    loop {
        if !s.contains('\u{2045}') {
            break;
        }
        let chars: Vec<char> = s.chars().collect();
        let len = chars.len();
        let mut result = String::with_capacity(s.len());
        let mut i = 0;
        let mut changed = false;
        while i < len {
            if chars[i] == '\u{2046}' {
                // Found close ⁆ — scan backward for matching open ⁅
                let close_pos = result.len();
                let _ = close_pos;
                if let Some(open_byte) = result.rfind('\u{2045}') {
                    let inner = result[open_byte + '\u{2045}'.len_utf8()..].to_string();
                    result.truncate(open_byte);
                    // Find first comma at depth 0 in inner
                    let mut depth = 0usize;
                    let mut comma_pos = None;
                    for (ci, ch) in inner.char_indices() {
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
                    let inner_fixed = if let Some(cp) = comma_pos {
                        format!("{} {}", inner[..cp].trim(), inner[cp + 1..].trim())
                    } else {
                        inner.trim().to_string()
                    };
                    result.push_str("(LieBracket ");
                    result.push_str(&inner_fixed);
                    result.push(')');
                    changed = true;
                } else {
                    result.push(chars[i]);
                }
                i += 1;
            } else {
                result.push(chars[i]);
                i += 1;
            }
        }
        s = result;
        if !changed {
            break;
        }
    }
    s
}

/// Normalize equiv+subscript+bracket operators like `≃ₗ[R]`, `≃ₐ[R]`.
fn normalize_equiv_subscript_brackets(src: &str) -> String {
    let patterns: &[(&str, &str, bool)] = &[
        ("\u{2243}\u{209B}\u{2097}[", " SemilinearEquiv ", true), // ≃ₛₗ[
        ("\u{2243}\u{2097}[", " LinearEquiv ", true),             // ≃ₗ[
        ("\u{2243}\u{2090}[", " AlgEquiv ", true),                // ≃ₐ[
    ];
    let mut s = src.to_string();
    for &(pat, replacement, strip_bracket) in patterns {
        if !s.contains(pat) {
            continue;
        }
        if strip_bracket {
            let mut result = String::with_capacity(s.len());
            let mut rest = s.as_str();
            while let Some(pos) = rest.find(pat) {
                result.push_str(&rest[..pos]);
                result.push_str(replacement);
                let after = &rest[pos + pat.len()..];
                if let Some(close) = after.find(']') {
                    rest = &after[close + 1..];
                } else {
                    rest = after;
                }
            }
            result.push_str(rest);
            s = result;
        } else {
            s = s.replace(pat, replacement);
        }
    }
    s
}

/// Normalize arrow+subscript+bracket operators like `→ₗ[R]`, `→ₐ[R]`, `→ₘ[μ]`, `→ₗ.`.
///
/// These are Lean 4 notation for typed morphisms:
/// - `→ₗ[R]` = LinearMap over R
/// - `→ₗ.` or `→ₗ.[R]` = LinearPMap (partial linear map)
/// - `→ₐ[R]` = AlgHom over R
/// - `→ₘ[μ]` = AEEqFun (measure-valued)
///
/// We replace the entire `→ₗ[...]` including brackets with a simple type name.
fn normalize_arrow_subscript_brackets(src: &str) -> String {
    // Patterns: →ₗ.[...] must come before →ₗ[...]
    let patterns: &[(&str, &str, bool)] = &[
        ("\u{2192}\u{209B}\u{2097}[", " SemilinearMap ", true), // →ₛₗ[
        ("\u{2192}\u{209B}\u{2097}", " SemilinearMap ", false), // →ₛₗ (bare)
        ("\u{2192}\u{2097}.[", " LinearPMap ", true),           // →ₗ.[
        ("\u{2192}\u{2097}[", " LinearMap ", true),             // →ₗ[
        ("\u{2192}\u{2097}.", " LinearPMap ", false),           // →ₗ. (no bracket)
        ("\u{2192}\u{2097}", " LinearMap ", false),             // →ₗ (bare)
        ("\u{2192}\u{2091}+*[", " MulSemiringActionHom ", true), // →ₑ+*[
        ("\u{2192}\u{2091}+[", " DistribMulActionHom ", true),  // →ₑ+[
        ("\u{2192}\u{2091}[", " MulActionHom ", true),          // →ₑ[
        ("\u{2192}\u{2091}+*", " MulSemiringActionHom ", false), // →ₑ+* (bare)
        ("\u{2192}\u{2091}+", " DistribMulActionHom ", false),  // →ₑ+ (bare)
        ("\u{2192}\u{2091}", " MulActionHom ", false),          // →ₑ (bare)
        ("\u{2192}\u{2090}[", " AlgHom ", true),                // →ₐ[
        ("\u{2192}\u{2090}", " AlgHom ", false),                // →ₐ (bare)
        ("\u{2192}\u{2098}[", " AEEqFun ", true),               // →ₘ[
        ("\u{2192}\u{2098}", " AEEqFun ", false),               // →ₘ (bare)
    ];
    let mut s = src.to_string();
    for &(pat, replacement, strip_bracket) in patterns {
        if !s.contains(pat) {
            continue;
        }
        if strip_bracket {
            let mut result = String::with_capacity(s.len());
            let mut rest = s.as_str();
            while let Some(pos) = rest.find(pat) {
                result.push_str(&rest[..pos]);
                result.push_str(replacement);
                let after = &rest[pos + pat.len()..];
                if let Some(close) = after.find(']') {
                    rest = &after[close + 1..];
                } else {
                    rest = after;
                }
            }
            result.push_str(rest);
            s = result;
        } else {
            s = s.replace(pat, replacement);
        }
    }
    s
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
                        c.is_alphanumeric()
                            || c == '_'
                            || c == '\''
                            || c == '!'
                            || c == '.'
                            || c == '-'
                    });
                let is_func_app = is_singleton && found_space;
                if preceded_by_quantifier && is_func_app {
                    result.push('(');
                    result.push_str(inner_trimmed);
                    result.push(')');
                } else if preceded_by_quantifier && is_simple_ident {
                    // Implicit binder: ∀ {n} → keep as (n : _)
                    result.push('(');
                    result.push_str(inner_trimmed);
                    result.push_str(" : _)");
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
                } else if is_singleton && !is_simple_ident && !is_func_app {
                    // Bracket-containing singletons: {[]} → (singleton ListNil),
                    // {()} → (singleton Unit_mk), {(expr)} → (singleton (expr))
                    if inner_trimmed == "[]" {
                        result.push_str("(singleton ListNil)");
                    } else if inner_trimmed == "()" {
                        result.push_str("(singleton Unit_mk)");
                    } else {
                        result.push_str("(singleton ");
                        result.push_str(inner_trimmed);
                        result.push(')');
                    }
                } else if preceded_by_quantifier && found_colon_direct {
                    // Implicit typed binder in forall: {α β : T} → (α β : T)
                    result.push('(');
                    result.push_str(inner_trimmed);
                    result.push(')');
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

/// Normalize `::` (cons) without surrounding spaces: `a::l` → `a Cons l`.
fn normalize_cons_without_spaces(src: &str) -> String {
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut result = String::with_capacity(src.len() + 16);
    let mut i = 0;
    while i < len {
        if chars[i] == ':'
            && i + 1 < len
            && chars[i + 1] == ':'
            && i > 0
            && (chars[i - 1].is_alphanumeric() || chars[i - 1] == '_' || chars[i - 1] == '\'')
            && i + 2 < len
            && (chars[i + 2].is_alphanumeric() || chars[i + 2] == '_' || chars[i + 2] == '(')
        {
            result.push_str(" Cons ");
            i += 2;
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}

/// Normalize filter restriction notation: `Nhds[≠]` → `Nhds`, `Nhds[S]` → `Nhds`.
/// Strips `[content]` immediately after known filter identifiers.
fn normalize_filter_restriction(src: &str) -> String {
    let prefixes = ["Nhds", "Principal", "atTop", "atBot"];
    let mut s = src.to_string();
    for prefix in &prefixes {
        let mut result = String::with_capacity(s.len());
        let mut rest = s.as_str();
        let pat = format!("{}[", prefix);
        while let Some(pos) = rest.find(&pat) {
            result.push_str(&rest[..pos]);
            result.push_str(prefix);
            let after = &rest[pos + pat.len()..];
            if let Some(close) = after.find(']') {
                rest = &after[close + 1..];
            } else {
                rest = after;
            }
        }
        result.push_str(rest);
        s = result;
    }
    s
}

/// Normalize `Integral x in S, body` → `Integral S (fun x -> body)`.
/// Also handles `IntegralInv x in S, body`.
fn normalize_integral_in(src: &str) -> String {
    let mut result = src.to_string();
    for op in &[
        "Integral",
        "IntegralInv",
        "AvgIntegral",
        "AvgIntegralInv",
        "ContourIntegral",
        "CurveIntegral",
    ] {
        result = normalize_integral_in_op(&result, op);
    }
    result
}

#[allow(dead_code)]
fn normalize_integral_in_op(src: &str, op: &str) -> String {
    if !src.contains(op) {
        return src.to_string();
    }
    let mut result = String::with_capacity(src.len() + 32);
    let mut rest = src;
    let pat = format!("{} ", op);
    while let Some(pos) = rest.find(&pat) {
        // Word boundary check: don't match if preceded by alphanumeric/underscore
        if pos > 0 {
            let prev_byte = rest.as_bytes()[pos - 1];
            if prev_byte.is_ascii_alphanumeric() || prev_byte == b'_' {
                result.push_str(&rest[..pos + pat.len()]);
                rest = &rest[pos + pat.len()..];
                continue;
            }
        }
        result.push_str(&rest[..pos]);
        let after = &rest[pos + pat.len()..];
        // Look for " in " at depth 0
        let mut depth = 0usize;
        let mut in_pos = None;
        let chars: Vec<char> = after.chars().collect();
        let mut ci = 0;
        let mut byte_i = 0;
        while ci < chars.len() {
            match chars[ci] {
                '(' | '[' | '{' => depth += 1,
                ')' | ']' | '}' => depth = depth.saturating_sub(1),
                _ => {}
            }
            if depth == 0 && ci + 3 < chars.len() {
                let word: String = chars[ci..ci + 4].iter().collect();
                if word == " in " {
                    in_pos = Some(byte_i);
                    break;
                }
            }
            byte_i += chars[ci].len_utf8();
            ci += 1;
        }
        if let Some(ip) = in_pos {
            let var_part = after[..ip].trim();
            let after_in = &after[ip + 4..];
            // Find comma at depth 0 for body separator
            let mut depth2 = 0usize;
            let mut comma_pos = None;
            for (ci2, ch) in after_in.char_indices() {
                match ch {
                    '(' | '[' | '{' => depth2 += 1,
                    ')' | ']' | '}' => depth2 = depth2.saturating_sub(1),
                    ',' if depth2 == 0 => {
                        comma_pos = Some(ci2);
                        break;
                    }
                    _ => {}
                }
            }
            if let Some(cp) = comma_pos {
                let set_expr = after_in[..cp].trim();
                let body = &after_in[cp + 1..];
                let var_name = if let Some(colon) = var_part.find(':') {
                    var_part[..colon].trim()
                } else {
                    var_part
                };
                // Strip parens from var_name: `(x` → `x`, `(x)` → `x`
                let var_name = var_name
                    .trim_start_matches('(')
                    .trim_end_matches(')')
                    .trim();
                let var_name = if var_name.is_empty() || var_name == "_" {
                    "x"
                } else {
                    var_name
                };
                // Find body end: stop at `:=` at depth 0 OR `)` when depth goes negative
                let mut bd = 0i32;
                let mut body_end = body.len();
                for (bi, bch) in body.char_indices() {
                    match bch {
                        '(' | '[' | '{' => bd += 1,
                        ')' | ']' | '}' => {
                            bd -= 1;
                            if bd < 0 {
                                body_end = bi;
                                break;
                            }
                        }
                        ':' if bd == 0
                            && bi + 1 < body.len()
                            && body.as_bytes()[bi + 1] == b'=' =>
                        {
                            body_end = bi;
                            break;
                        }
                        _ => {}
                    }
                }
                let (body_part, tail) = body.split_at(body_end);
                result.push_str(&format!(
                    "{} {} (fun {} ->{})",
                    op, set_expr, var_name, body_part
                ));
                rest = tail;
                continue;
            }
        }
        // No `in` found or no comma, pass through
        result.push_str(&pat);
        rest = after;
    }
    result.push_str(rest);
    result
}

/// Normalize `⨂[R]` tensor product subscript bracket notation.
fn normalize_tensor_subscript_brackets(src: &str) -> String {
    if !src.contains("BigTensorProd [") {
        return src.to_string();
    }
    let mut result = String::with_capacity(src.len());
    let mut rest = src;
    while let Some(pos) = rest.find("BigTensorProd [") {
        result.push_str(&rest[..pos]);
        result.push_str("BigTensorProd ");
        let after = &rest[pos + "BigTensorProd [".len()..];
        if let Some(close) = after.find(']') {
            rest = &after[close + 1..];
        } else {
            rest = after;
        }
    }
    result.push_str(rest);
    result
}

/// Normalize `..` spread syntax: ` .. ` → ` sorry `.
fn normalize_spread_syntax(src: &str) -> String {
    if !src.contains("..") {
        return src.to_string();
    }
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut result = String::with_capacity(src.len());
    let mut i = 0;
    while i < len {
        if chars[i] == '.'
            && i + 1 < len
            && chars[i + 1] == '.'
            && (i + 2 >= len || chars[i + 2] != '.')
        {
            // Check if preceded by alphanumeric/paren (range syntax: 0..1)
            let prev_is_expr = i > 0
                && (chars[i - 1].is_alphanumeric()
                    || chars[i - 1] == '_'
                    || chars[i - 1] == '\''
                    || chars[i - 1] == ')');
            if prev_is_expr {
                // Range syntax: keep as-is but replace with RangeCC
                result.push_str(" RangeCC ");
                i += 2;
            } else {
                // Spread syntax: .. → sorry
                result.push_str(" sorry");
                i += 2;
            }
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}

/// Normalize semicolons inside parenthesized expressions to spaces.
fn normalize_semicolons_in_parens(src: &str) -> String {
    if !src.contains(';') {
        return src.to_string();
    }
    let mut result = String::with_capacity(src.len());
    let mut paren_depth = 0usize;
    for ch in src.chars() {
        match ch {
            '(' => {
                paren_depth += 1;
                result.push('(');
            }
            ')' => {
                paren_depth = paren_depth.saturating_sub(1);
                result.push(')');
            }
            ';' if paren_depth > 0 => {
                result.push(' ');
            }
            _ => {
                result.push(ch);
            }
        }
    }
    result
}
/// Normalize tilde-relation syntax: `x ~[r] y` → `x RelApp r y`,
/// `(· ~[r] ·)` → `(fun x y -> x RelApp r y)`.
/// Normalize dependent if (dite): `if hi : cond then` → `if cond then`.
fn normalize_dite(src: &str) -> String {
    if !src.contains("if ") {
        return src.to_string();
    }
    let mut result = String::with_capacity(src.len());
    let mut rest = src;
    while let Some(pos) = rest.find("if ") {
        result.push_str(&rest[..pos]);
        let after_if = &rest[pos + 3..];
        // Check for `if ident : cond then` pattern
        let trimmed = after_if.trim_start();
        if let Some(colon_pos) = trimmed.find(" : ") {
            // Check that before the colon is a simple identifier
            let before_colon = &trimmed[..colon_pos];
            if before_colon
                .chars()
                .all(|c| c.is_alphanumeric() || c == '_' || c == '\'')
                && !before_colon.is_empty()
            {
                let after_colon = &trimmed[colon_pos + 3..];
                result.push_str("if ");
                rest = after_colon;
                continue;
            }
        }
        result.push_str("if ");
        rest = after_if;
    }
    result.push_str(rest);
    result
}
/// Normalize `if COND then EXPR1 else EXPR2` → `(ite COND EXPR1 EXPR2)`.
///
/// This prevents `then`/`else` keywords from confusing later normalization passes
/// (e.g., exists quantifier normalization swallowing `then` into a lambda body).
/// Handles nested if-then-else by processing innermost first.
#[allow(dead_code)]
fn normalize_if_then_else(src: &str) -> String {
    if !src.contains(" if ") && !src.contains("(if ") {
        return src.to_string();
    }
    let mut s = src.to_string();
    // Iterate up to a fixed number of passes (for nested if-then-else)
    for _ in 0..10 {
        match normalize_if_then_else_once(&s) {
            Some(new) => s = new,
            None => break,
        }
    }
    s
}

/// Single pass: find the innermost `if ... then ... else ...` and convert it.
/// Returns `None` if no `if-then-else` was found.
#[allow(dead_code)]
fn normalize_if_then_else_once(src: &str) -> Option<String> {
    // Find all `if ` positions at various depths, pick the last one that has a matching then/else
    let bytes = src.as_bytes();
    let len = bytes.len();
    // Scan for `if ` keyword (word boundary checked)
    let mut best_if_pos: Option<usize> = None;
    let mut i = 0;
    while i + 3 <= len {
        if (bytes[i] == b'i' && bytes[i + 1] == b'f' && bytes[i + 2] == b' ')
            || (i + 3 < len && bytes[i] == b'i' && bytes[i + 1] == b'f' && bytes[i + 2] == b'\t')
        {
            // Check word boundary
            let at_boundary = i == 0
                || (!bytes[i - 1].is_ascii_alphanumeric()
                    && bytes[i - 1] != b'_'
                    && bytes[i - 1] != b'\'');
            if at_boundary {
                // Check this if has a matching then/else at proper depth
                if has_matching_then_else(src, i) {
                    best_if_pos = Some(i);
                }
            }
        }
        i += 1;
    }

    let if_pos = best_if_pos?;

    // Now extract: find `then` and `else` at the same bracket depth as the `if`
    let if_depth = bracket_depth_at(src, if_pos);
    let after_if = if_pos + 3; // skip "if "

    // Find matching `then ` at same depth
    let then_pos = find_keyword_at_depth(src, after_if, "then", if_depth)?;
    let after_then = then_pos + 5; // skip "then " (4 + 1 space)

    // Find matching `else ` at same depth
    let else_pos = find_keyword_at_depth(src, after_then, "else", if_depth)?;
    let after_else = else_pos + 5; // skip "else " (4 + 1 space)

    // Extract the condition (between if and then)
    let cond = src[after_if..then_pos].trim();

    // Extract then-branch (between then and else)
    let then_branch = src[after_then..else_pos].trim();

    // Extract else-branch: scan until we hit a closing bracket at depth < if_depth,
    // or a comma at if_depth, or end of string, or `:=`
    let else_end = find_else_end(src, after_else, if_depth);
    let else_branch = src[after_else..else_end].trim();

    let mut result = String::with_capacity(src.len() + 10);
    result.push_str(&src[..if_pos]);
    result.push_str("(ite ");
    result.push_str(cond);
    result.push(' ');
    result.push_str(then_branch);
    result.push(' ');
    result.push_str(else_branch);
    result.push(')');
    result.push_str(&src[else_end..]);
    Some(result)
}

/// Check if the `if` at position `pos` has matching `then` and `else` at proper depth.
#[allow(dead_code)]
fn has_matching_then_else(src: &str, pos: usize) -> bool {
    let depth = bracket_depth_at(src, pos);
    let after_if = pos + 3;
    if let Some(then_pos) = find_keyword_at_depth(src, after_if, "then", depth) {
        let after_then = then_pos + 5;
        find_keyword_at_depth(src, after_then, "else", depth).is_some()
    } else {
        false
    }
}

/// Calculate bracket depth at a given byte position.
#[allow(dead_code)]
fn bracket_depth_at(src: &str, pos: usize) -> usize {
    let mut depth = 0usize;
    for &b in &src.as_bytes()[..pos] {
        match b {
            b'(' | b'[' | b'{' => depth += 1,
            b')' | b']' | b'}' => depth = depth.saturating_sub(1),
            _ => {}
        }
    }
    depth
}

/// Find keyword (like "then" or "else") at a given bracket depth, starting from `start`.
/// The keyword must be at a word boundary and followed by a space or bracket.
#[allow(dead_code)]
fn find_keyword_at_depth(
    src: &str,
    start: usize,
    keyword: &str,
    target_depth: usize,
) -> Option<usize> {
    let bytes = src.as_bytes();
    let len = bytes.len();
    let kw_bytes = keyword.as_bytes();
    let kw_len = kw_bytes.len();
    let mut depth = bracket_depth_at(src, start);
    let mut i = start;
    while i + kw_len <= len {
        match bytes[i] {
            b'(' | b'[' | b'{' => {
                depth += 1;
                i += 1;
            }
            b')' | b']' | b'}' => {
                if depth <= target_depth {
                    return None; // Went above our level
                }
                depth = depth.saturating_sub(1);
                i += 1;
            }
            _ => {
                if depth == target_depth && bytes[i..].starts_with(kw_bytes) {
                    // Check word boundary before
                    let at_start = i == 0
                        || (!bytes[i - 1].is_ascii_alphanumeric()
                            && bytes[i - 1] != b'_'
                            && bytes[i - 1] != b'\'');
                    // Check word boundary after
                    let at_end = i + kw_len >= len
                        || (!bytes[i + kw_len].is_ascii_alphanumeric()
                            && bytes[i + kw_len] != b'_'
                            && bytes[i + kw_len] != b'\'');
                    if at_start && at_end {
                        return Some(i);
                    }
                }
                i += 1;
            }
        }
    }
    None
}

/// Find the end of the else branch. Scan until we hit:
/// - A closing bracket at depth < starting depth
/// - `:=` at starting depth
/// - End of string
#[allow(dead_code)]
fn find_else_end(src: &str, start: usize, base_depth: usize) -> usize {
    let bytes = src.as_bytes();
    let len = bytes.len();
    let mut depth = bracket_depth_at(src, start);
    let mut i = start;
    while i < len {
        match bytes[i] {
            b'(' | b'[' | b'{' => {
                depth += 1;
                i += 1;
            }
            b')' | b']' | b'}' => {
                if depth <= base_depth {
                    return i;
                }
                depth -= 1;
                i += 1;
            }
            b':' if depth == base_depth && i + 1 < len && bytes[i + 1] == b'=' => {
                return i;
            }
            b',' if depth == base_depth => {
                return i;
            }
            _ => {
                i += 1;
            }
        }
    }
    len
}

/// Insert space before ≥/≤/≠ when immediately preceded by an identifier char (letter, digit, ').
/// Normalize conditional expectation `P⁻[X|mΩ]` → `condExp X mΩ`.
fn normalize_cond_exp(src: &str) -> String {
    let pat = "\u{207B}[";
    if !src.contains(pat) {
        return src.to_string();
    }
    let mut result = String::with_capacity(src.len());
    let mut rest = src;
    while let Some(pos) = rest.find(pat) {
        // Check that char before ⁻ is alphanumeric (it's P⁻[...])
        let before = &rest[..pos];
        let last_char = before.chars().last();
        if last_char.is_some_and(|c| c.is_alphanumeric() || c == '_' || c == '\'') {
            let after_bracket = &rest[pos + pat.len()..];
            if let Some(close) = after_bracket.find(']') {
                let inner = &after_bracket[..close];
                // Split on | if present
                let parts: Vec<&str> = inner.splitn(2, '|').collect();
                result.push_str(before);
                result.push_str(" condExp");
                for p in &parts {
                    result.push(' ');
                    result.push_str(p.trim());
                }
                rest = &after_bracket[close + 1..];
                continue;
            }
        }
        result.push_str(&rest[..pos + pat.len()]);
        rest = &rest[pos + pat.len()..];
    }
    result.push_str(rest);
    result
}
/// Normalize Jacobi symbol notation: `J(a | b)` → `(JacobiSym a b)`.
fn normalize_jacobi_symbol(src: &str) -> String {
    if !src.contains("J(") {
        return src.to_string();
    }
    let mut result = String::with_capacity(src.len());
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut i = 0;
    while i < len {
        if chars[i] == 'J'
            && i + 1 < len
            && chars[i + 1] == '('
            && (i == 0 || !chars[i - 1].is_alphanumeric())
        {
            // Find matching ) with depth tracking
            let mut j = i + 2;
            let mut depth = 1;
            while j < len && depth > 0 {
                if chars[j] == '(' {
                    depth += 1;
                } else if chars[j] == ')' {
                    depth -= 1;
                }
                j += 1;
            }
            if depth == 0 {
                let inner: String = chars[i + 2..j - 1].iter().collect();
                // Replace `|` separator with space
                let normalized = inner.replace('|', " ");
                result.push_str("(JacobiSym ");
                result.push_str(normalized.trim());
                result.push(')');
                i = j;
                continue;
            }
        }
        result.push(chars[i]);
        i += 1;
    }
    result
}
/// Strip shift-prime notation: `f⟦a⟧'` → `f` (CategoryTheory shift functor).
fn normalize_shift_prime(src: &str) -> String {
    if !src.contains('\u{27E6}') {
        return src.to_string();
    }
    let mut result = String::with_capacity(src.len());
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut i = 0;
    while i < len {
        if chars[i] == '\u{27E6}' {
            // Look for matching ⟧ with optional trailing '
            let start = i;
            let mut j = i + 1;
            let mut depth = 1;
            while j < len && depth > 0 {
                if chars[j] == '\u{27E6}' {
                    depth += 1;
                } else if chars[j] == '\u{27E7}' {
                    depth -= 1;
                }
                j += 1;
            }
            if depth == 0 {
                // Skip trailing ' if present
                if j < len && chars[j] == '\'' {
                    j += 1;
                }
                // Check if preceded by identifier (like subscript indexing)
                let preceded_by_ident = start > 0
                    && (chars[start - 1].is_alphanumeric()
                        || chars[start - 1] == '_'
                        || chars[start - 1] == '\''
                        || chars[start - 1] == ')');
                if preceded_by_ident {
                    i = j;
                    continue;
                }
            }
            // Not preceded by ident — keep as-is for general ⟦→( replacement
            let chunk: String = chars[start..j].iter().collect();
            result.push_str(&chunk);
            i = j;
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}
/// Normalize Landau notation: `=O[l]` → `IsBigO`, `=o[l]` → `IsLittleO`.
fn normalize_landau_notation(src: &str) -> String {
    if !src.contains("=O[") && !src.contains("=o[") && !src.contains("=\u{0398}[") {
        return src.to_string();
    }
    let mut result = String::with_capacity(src.len());
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut i = 0;
    while i < len {
        if chars[i] == '=' && i + 2 < len && chars[i + 2] == '[' {
            let mid = chars[i + 1];
            let replacement = match mid {
                'O' => Some(" IsBigO "),
                'o' => Some(" IsLittleO "),
                '\u{0398}' => Some(" IsTheta "), // Θ
                _ => None,
            };
            if let Some(rep) = replacement {
                i += 3;
                let mut depth = 1;
                while i < len && depth > 0 {
                    if chars[i] == '[' {
                        depth += 1;
                    } else if chars[i] == ']' {
                        depth -= 1;
                    }
                    i += 1;
                }
                result.push_str(rep);
                continue;
            }
        }
        {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}
/// Strip combining diacritical marks (U+0300-U+036F).
fn strip_combining_marks(src: &str) -> String {
    src.chars()
        .filter(|&c| !('\u{0300}'..='\u{036F}').contains(&c))
        .collect()
}
/// Strip `⟮...⟯` (math flat parens) — used for intermediate field notation like `K⟮S,n⟯`.
fn normalize_math_flat_parens(src: &str) -> String {
    if !src.contains('\u{27EE}') {
        return src.to_string();
    }
    let mut result = String::with_capacity(src.len());
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut i = 0;
    while i < len {
        if chars[i] == '\u{27EE}' {
            // Skip everything until matching ⟯
            let mut depth = 1;
            i += 1;
            while i < len && depth > 0 {
                if chars[i] == '\u{27EE}' {
                    depth += 1;
                } else if chars[i] == '\u{27EF}' {
                    depth -= 1;
                }
                i += 1;
            }
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}
/// Normalize `R<x>` angle bracket adjoin notation → `(Adjoin R x)`.
fn normalize_angle_adjoin(src: &str) -> String {
    if !src.contains('<') {
        return src.to_string();
    }
    let mut result = String::with_capacity(src.len());
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut i = 0;
    while i < len {
        if chars[i] == '<'
            && i > 0
            && (chars[i - 1].is_alphanumeric() || chars[i - 1] == '_' || chars[i - 1] == '\'')
        {
            // Check if prev word is an identifier (not an operator)
            // Find matching >
            let mut j = i + 1;
            let mut depth = 1usize;
            let mut found_close = false;
            while j < len && depth > 0 {
                match chars[j] {
                    '<' => depth += 1,
                    '>' => {
                        depth -= 1;
                        if depth == 0 {
                            found_close = true;
                        }
                    }
                    ' ' | '\n' | '\t' if depth == 1 => {
                        // Check for spaces — if multi-word, not adjoin
                        break;
                    }
                    _ => {}
                }
                j += 1;
            }
            if found_close && depth == 0 {
                let inner: String = chars[i + 1..j - 1].iter().collect();
                result.push_str(&format!("_adj_{}", inner.trim()));
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
/// Normalize `!![a, b; c, d]` matrix literals → `(MatLit sorry)`.
fn normalize_matrix_literal(src: &str) -> String {
    if !src.contains("!![") {
        return src.to_string();
    }
    let mut result = String::with_capacity(src.len());
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut i = 0;
    while i < len {
        if i + 2 < len && chars[i] == '!' && chars[i + 1] == '!' && chars[i + 2] == '[' {
            // Find matching ]
            let mut j = i + 3;
            let mut depth = 1usize;
            while j < len && depth > 0 {
                match chars[j] {
                    '[' => depth += 1,
                    ']' => depth -= 1,
                    _ => {}
                }
                j += 1;
            }
            if depth == 0 {
                result.push_str("(MatLit sorry)");
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
fn space_before_comparison_ops(src: &str) -> String {
    let mut result = String::with_capacity(src.len() + 16);
    let mut prev = '\0';
    for ch in src.chars() {
        if (ch == '\u{2265}' || ch == '\u{2264}' || ch == '\u{2260}')
            && (prev.is_alphanumeric() || prev == '_' || prev == '\'')
        {
            result.push(' ');
        }
        result.push(ch);
        prev = ch;
    }
    result
}
fn normalize_tilde_relation(src: &str) -> String {
    if !src.contains('~') {
        return src.to_string();
    }
    let mut result = String::with_capacity(src.len());
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut i = 0;
    while i < len {
        if chars[i] == '~' && i + 1 < len && chars[i + 1] == '[' {
            // ~[expr] → RelApp expr
            let mut j = i + 2;
            let mut depth = 1usize;
            while j < len && depth > 0 {
                match chars[j] {
                    '[' => depth += 1,
                    ']' => depth -= 1,
                    _ => {}
                }
                j += 1;
            }
            if depth == 0 {
                let inner: String = chars[i + 2..j - 1].iter().collect();
                result.push_str(&format!("RelApp {}", inner.trim()));
                i = j;
            } else {
                result.push(chars[i]);
                i += 1;
            }
        } else if chars[i] == '~'
            && i + 1 < len
            && (chars[i + 1].is_alphabetic() || chars[i + 1] == '_')
        {
            // ~ident → TildeRel ident (bare tilde relation like ~r)
            let j_start = i + 1;
            let mut j = j_start;
            while j < len && (chars[j].is_alphanumeric() || chars[j] == '_' || chars[j] == '\'') {
                j += 1;
            }
            let ident: String = chars[j_start..j].iter().collect();
            result.push_str(&format!(" TildeRel {} ", ident));
            i = j;
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}
/// Normalize `[[...]]` double brackets (formal power series) to `[...]`
/// but NOT when preceded by `[` or `MatVec` (matrix notation like `[![n]]`).
fn normalize_double_brackets(src: &str) -> String {
    if !src.contains("[[") && !src.contains("]]") {
        return src.to_string();
    }
    let chars: Vec<char> = src.chars().collect();
    let mut result = String::with_capacity(src.len());
    let mut i = 0;
    while i < chars.len() {
        if i + 1 < chars.len() && chars[i] == '[' && chars[i + 1] == '[' {
            let prev = if i > 0 { chars[i - 1] } else { ' ' };
            if prev == '[' || result.ends_with("MatVec ") {
                // Matrix context: keep both brackets
                result.push('[');
                result.push('[');
                i += 2;
            } else {
                // Formal power series: skip second [
                result.push('[');
                i += 2;
            }
        } else if i + 1 < chars.len() && chars[i] == ']' && chars[i + 1] == ']' {
            // Check if the content before looks like matrix (contains MatVec)
            let before = &result;
            // Simple heuristic: if we recently had `[MatVec [`, keep both ]
            let last_matv = before.rfind("MatVec [");
            let last_close = before.rfind(']');
            let in_matrix = match (last_matv, last_close) {
                (Some(m), Some(c)) => c < m, // no ] after last MatVec → still inside
                (Some(_), None) => true,
                _ => false,
            };
            if in_matrix {
                result.push(']');
                result.push(']');
                i += 2;
            } else {
                result.push(']');
                i += 2;
            }
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}

/// Normalize struct literal bodies: `{ field := val, ... }` to `sorry`.
///
/// Replaces any `{ ... := ... }` struct literal with `sorry`.
/// This handles struct literals both in body position (after `:=`) and in type position.
fn normalize_struct_literal_body(src: &str) -> String {
    if !src.contains(":=") {
        return src.to_string();
    }
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut result = String::with_capacity(src.len());
    let mut i = 0;
    while i < len {
        if chars[i] == '{' {
            // Scan for matching `}` and check for `:=` inside at depth 1
            let mut depth = 1usize;
            let mut j = i + 1;
            let mut has_field_assign = false;
            while j < len && depth > 0 {
                match chars[j] {
                    '{' | '(' | '[' => depth += 1,
                    '}' => {
                        depth -= 1;
                    }
                    ')' | ']' => depth = depth.saturating_sub(1),
                    ':' if j + 1 < len && chars[j + 1] == '=' && depth == 1 => {
                        has_field_assign = true;
                    }
                    _ => {}
                }
                j += 1;
            }
            if depth == 0 && has_field_assign {
                // Check if the previous non-space char is `=` (part of `:= {`)
                let before_trimmed = result.trim_end();
                if before_trimmed.ends_with(":=") {
                    // Remove the `:=` from result and add `:= sorry`
                    let prefix_end = result.rfind(":=").unwrap_or(result.len());
                    result.truncate(prefix_end);
                    result.push_str(":= sorry");
                } else {
                    // Struct literal in type position -- replace with `sorry`
                    result.push_str("sorry");
                }
                i = j;
                continue;
            }
        }
        result.push(chars[i]);
        i += 1;
    }
    result
}

/// Normalize anonymous dot method calls: `(.method args)` to `(fun x_ -> x_method args)`.
///
/// Lean 4's `(.method arg)` means `fun x -> x.method arg`. We normalize to
/// `(fun x_ -> x_method args)` to make it parseable.
fn normalize_anon_dot_method(src: &str) -> String {
    if !src.contains("(.") && !src.contains("( .") {
        return src.to_string();
    }
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut result = String::with_capacity(src.len() + 64);
    let mut i = 0;
    while i < len {
        // Match `(.method` or `( .method` (with optional space)
        let dot_start;
        if chars[i] == '(' && i + 2 < len && chars[i + 1] == '.' && chars[i + 2].is_alphabetic() {
            dot_start = i + 2;
        } else if chars[i] == '('
            && i + 3 < len
            && chars[i + 1] == ' '
            && chars[i + 2] == '.'
            && chars[i + 3].is_alphabetic()
        {
            dot_start = i + 3;
        } else {
            result.push(chars[i]);
            i += 1;
            continue;
        }
        {
            // Read the method name
            let mut j = dot_start;
            while j < len && (chars[j].is_alphanumeric() || chars[j] == '_' || chars[j] == '\'') {
                j += 1;
            }
            let method: String = chars[dot_start..j].iter().collect();
            // Read the rest until matching close paren
            let mut depth = 1usize;
            let mut k = j;
            let mut close = None;
            while k < len {
                match chars[k] {
                    '(' | '[' | '{' => depth += 1,
                    ')' => {
                        depth -= 1;
                        if depth == 0 {
                            close = Some(k);
                            break;
                        }
                    }
                    ']' | '}' => depth = depth.saturating_sub(1),
                    _ => {}
                }
                k += 1;
            }
            if let Some(cp) = close {
                let args: String = chars[j..cp].iter().collect();
                let args = args.trim();
                if args.is_empty() {
                    result.push_str(&format!("(fun x_ -> x_{method})"));
                } else {
                    result.push_str(&format!("(fun x_ -> x_{method} {args})"));
                }
                i = cp + 1;
                continue;
            }
        }
        result.push(chars[i]);
        i += 1;
    }
    result
}

/// Strip orphaned `]` brackets that have no matching `[` at the same bracket depth.
/// These are artifacts from subscript/arrow notation stripping (e.g., `→ₗ[R]` → `LinearMap ]`).
fn strip_orphan_close_brackets(src: &str) -> String {
    if !src.contains(']') {
        return src.to_string();
    }
    let mut result = String::with_capacity(src.len());
    let mut bracket_depth: i32 = 0;
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut i = 0;
    while i < len {
        match chars[i] {
            '[' => {
                bracket_depth += 1;
                result.push(chars[i]);
                i += 1;
            }
            ']' => {
                if bracket_depth > 0 {
                    bracket_depth -= 1;
                    result.push(chars[i]);
                } else {
                    // Orphaned `]` — skip it (and collapse trailing spaces)
                    let trimmed_len = result.trim_end().len();
                    let trailing_spaces = result.len() - trimmed_len;
                    if trailing_spaces > 0 {
                        result.truncate(trimmed_len);
                        result.push(' ');
                    }
                }
                i += 1;
            }
            _ => {
                result.push(chars[i]);
                i += 1;
            }
        }
    }
    result
}

/// Strip orphaned `)` that have no matching `(`.
/// Similar to `strip_orphan_close_brackets` but for round parens.
/// Only strips `)` that would make paren depth go negative (no matching open).
fn strip_orphan_close_parens(src: &str) -> String {
    if !src.contains(')') {
        return src.to_string();
    }
    let mut result = String::with_capacity(src.len());
    let mut paren_depth: i32 = 0;
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut i = 0;
    while i < len {
        match chars[i] {
            '(' => {
                paren_depth += 1;
                result.push(chars[i]);
                i += 1;
            }
            ')' => {
                if paren_depth > 0 {
                    paren_depth -= 1;
                    result.push(chars[i]);
                } else {
                    // Orphaned `)` — skip it (and collapse trailing spaces)
                    let trimmed_len = result.trim_end().len();
                    let trailing_spaces = result.len() - trimmed_len;
                    if trailing_spaces > 0 {
                        result.truncate(trimmed_len);
                        result.push(' ');
                    }
                }
                i += 1;
            }
            _ => {
                result.push(chars[i]);
                i += 1;
            }
        }
    }
    result
}

/// Strip ` <- ` (monadic bind arrow) and `do` blocks from the type portion of declarations.
/// These are artifacts from `← ` conversion that survived proof replacement.
fn strip_bind_arrow_in_type(src: &str) -> String {
    // Only modify the type portion (before `:= sorry`)
    if let Some(assign_pos) = src.find(":= sorry") {
        let type_part = &src[..assign_pos];
        let mut fixed = type_part.to_string();
        let mut changed = false;
        if fixed.contains(" <- ") {
            fixed = fixed.replace(" <- ", " ");
            changed = true;
        }
        // Also strip `= do ` followed by a block — replace with `= sorry`
        if fixed.contains("= do ") {
            if let Some(do_pos) = fixed.find("= do ") {
                fixed = fixed[..do_pos].to_string();
                changed = true;
            }
        }
        if changed {
            return format!("{} := sorry", fixed.trim_end());
        }
    }
    src.to_string()
}

/// Strip remaining `partial_d EXPR` artifacts from the normalized output.
/// After ae-quantifier normalization, any leftover `partial_d` tokens are artifacts
/// from `∂` (partial derivative) in non-quantifier contexts (e.g., integral notation).
/// We strip ` partial_d EXPR` where EXPR is either:
/// - A parenthesized expression `(...)` (balanced parens)
/// - A bare identifier (sequence of alphanumeric/underscore/dot/prime chars)
fn strip_remaining_partial_d(src: &str) -> String {
    if !src.contains(" partial_d ") {
        return src.to_string();
    }
    let mut result = String::with_capacity(src.len());
    let mut rest = src;
    while let Some(pos) = rest.find(" partial_d ") {
        result.push_str(&rest[..pos]);
        let after = &rest[pos + 11..]; // skip " partial_d "
        let after_trimmed = after.trim_start();
        let skipped_ws = after.len() - after_trimmed.len();
        if after_trimmed.starts_with('(') {
            // Skip balanced parenthesized expression
            let mut depth = 0usize;
            let mut end = 0;
            for (ci, ch) in after_trimmed.char_indices() {
                match ch {
                    '(' | '[' | '{' => depth += 1,
                    ')' | ']' | '}' => {
                        depth = depth.saturating_sub(1);
                        if depth == 0 {
                            end = ci + ch.len_utf8();
                            break;
                        }
                    }
                    _ => {}
                }
            }
            if end > 0 {
                rest = &after_trimmed[end..];
            } else {
                // Unbalanced — just skip the word
                result.push(' ');
                rest = after;
            }
        } else {
            // Skip bare identifier: alphanumeric, underscore, dot, prime, Unicode letters
            let end = after_trimmed
                .char_indices()
                .find(|(_, c)| !c.is_alphanumeric() && *c != '_' && *c != '.' && *c != '\'')
                .map(|(i, _)| i)
                .unwrap_or(after_trimmed.len());
            if end > 0 {
                rest = &after_trimmed[end..];
            } else {
                result.push(' ');
                rest = &after[skipped_ws..];
            }
        }
    }
    result.push_str(rest);
    result
}

/// Normalize `PSigma BINDER, BODY` into `PSigma (fun BINDER -> BODY)`.
/// Only handles `PSigma` (from `Sigma'` normalization), not bare `Sigma`.
fn normalize_psigma_binder(src: &str) -> String {
    let mut result = String::with_capacity(src.len() + 64);
    let mut rest = src;
    let keyword = "PSigma";
    let keyword_len = keyword.len();
    while let Some(pos) = rest.find("PSigma ") {
        // Check word boundary before keyword
        if pos > 0 {
            let prev = rest.as_bytes()[pos - 1];
            if prev.is_ascii_alphanumeric() || prev == b'_' || prev == b'\'' {
                result.push_str(&rest[..pos + keyword_len]);
                rest = &rest[pos + keyword_len..];
                continue;
            }
        }
        result.push_str(&rest[..pos]);
        let after_keyword_raw = &rest[pos + keyword_len + 1..]; // skip keyword + space
        let after_keyword = after_keyword_raw.trim_start();
        let chars: Vec<char> = after_keyword.chars().collect();
        let clen = chars.len();
        if clen == 0 {
            result.push_str(keyword);
            rest = "";
            break;
        }
        // Case 1: Parenthesized binder — `PSigma (x : T), body`
        if chars[0] == '(' || chars[0] == '{' {
            let mut depth = 0i32;
            let mut ci = 0;
            let mut has_colon = false;
            while ci < clen {
                match chars[ci] {
                    '(' | '{' | '[' => {
                        depth += 1;
                        ci += 1;
                    }
                    ')' | '}' | ']' => {
                        depth -= 1;
                        ci += 1;
                        if depth == 0 {
                            break;
                        }
                    }
                    ':' if depth == 1 && ci + 1 < clen && chars[ci + 1] != '=' => {
                        has_colon = true;
                        ci += 1;
                    }
                    _ => {
                        ci += 1;
                    }
                }
            }
            let binder_end = ci;
            let mut after_binder = binder_end;
            while after_binder < clen && chars[after_binder] == ' ' {
                after_binder += 1;
            }
            if has_colon && after_binder < clen && chars[after_binder] == ',' {
                after_binder += 1;
                while after_binder < clen && chars[after_binder] == ' ' {
                    after_binder += 1;
                }
                let binder: String = chars[..binder_end].iter().collect();
                result.push_str(keyword);
                result.push_str(" (fun ");
                result.push_str(&binder);
                result.push_str(" -> ");
                // The body continues — process rest for nested PSigma/Sigma
                let body_byte_offset: usize =
                    chars[..after_binder].iter().map(|c| c.len_utf8()).sum();
                let body = &after_keyword[body_byte_offset..];
                let normalized_body = normalize_psigma_binder(body);
                result.push_str(&normalized_body);
                result.push(')');
                rest = "";
                break;
            }
            result.push_str(keyword);
            result.push(' ');
            rest = after_keyword;
            continue;
        }
        // Case 2: Bare binder — `PSigma x : T, body`
        let mut ci = 0;
        while ci < clen && (chars[ci].is_alphanumeric() || chars[ci] == '_' || chars[ci] == '\'') {
            ci += 1;
        }
        let ident_end = ci;
        if ident_end == 0 {
            result.push_str(keyword);
            result.push(' ');
            rest = after_keyword;
            continue;
        }
        let mut ci2 = ci;
        while ci2 < clen && chars[ci2] == ' ' {
            ci2 += 1;
        }
        if ci2 >= clen || chars[ci2] != ':' || (ci2 + 1 < clen && chars[ci2 + 1] == '=') {
            result.push_str(keyword);
            result.push(' ');
            rest = after_keyword;
            continue;
        }
        ci2 += 1;
        while ci2 < clen && chars[ci2] == ' ' {
            ci2 += 1;
        }
        let type_start = ci2;
        let mut depth = 0i32;
        while ci2 < clen {
            match chars[ci2] {
                '(' | '{' | '[' => {
                    depth += 1;
                    ci2 += 1;
                }
                ')' | '}' | ']' => {
                    if depth == 0 {
                        break;
                    }
                    depth -= 1;
                    ci2 += 1;
                }
                ',' if depth == 0 => break,
                _ => {
                    ci2 += 1;
                }
            }
        }
        if ci2 >= clen || chars[ci2] != ',' {
            result.push_str(keyword);
            result.push(' ');
            rest = after_keyword;
            continue;
        }
        let type_str: String = chars[type_start..ci2].iter().collect();
        ci2 += 1;
        while ci2 < clen && chars[ci2] == ' ' {
            ci2 += 1;
        }
        let ident: String = chars[..ident_end].iter().collect();
        let body_byte_offset: usize = chars[..ci2].iter().map(|c| c.len_utf8()).sum();
        let body = &after_keyword[body_byte_offset..];
        let normalized_body = normalize_psigma_binder(body);
        result.push_str(keyword);
        result.push_str(" (fun (");
        result.push_str(&ident);
        result.push_str(" : ");
        result.push_str(type_str.trim_end());
        result.push_str(") -> ");
        result.push_str(&normalized_body);
        result.push(')');
        rest = "";
        break;
    }
    result.push_str(rest);
    result
}
