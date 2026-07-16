## Differential verdict join: oxilean vs lean4lean

```
total distinct declarations joined: 1987

comparison buckets:
  AGREE (both verified or both rejected) : 1556
  DISAGREE (verified vs rejected)        : 20   <-- FINDINGS
  ONLY-ONE-SIDE (version/toolchain skew) : 228
  UNSUPPORTED-SKIPPED (one side declines): 183

oxilean per-verdict:
  verified    : 1611
  rejected    : 20
  unsupported : 183
  absent      : 173
lean4lean per-verdict:
  verified    : 1932
  rejected    : 0
  unsupported : 0
  absent      : 55
```

### Disagreements (findings)

Found 20 disagreement(s). Each is a FINDING (brief §5) — listed in full, never reclassified.

| declaration | oxilean | lean4lean | oxilean detail | lean4lean detail |
|---|---|---|---|---|
| `Lean.Name.noConfusion` | rejected | verified | TypeMismatch { expected: Pi(Default, Str(Anonymous, "pre"), Const(Str(Str(Anonymous, "Lean"), "Name"), []), Pi(Default, Str(Anonymous, "str"), Const(Str(Anonymous, "String"), []), App(Lam(Implicit, Str(Anonymous, "t"), Const(Str(Str(Anonymous, "Lean"), "Name"), []), App(App(App(Const(Str(Str(Str(Anonymous, "Lean"), "Name"), "noConfusionType"), [Param(Str(Anonymous, "u"))]), FVar(FVarId(4))), BVar(0)), BVar(0))), App(App(Const(Str(Str(Str(Anonymous, "Lean"), "Name"), "str"), []), BVar(1)), BVar(0))))), got: Pi(Default, Str(Anonymous, "pre"), Const(Str(Str(Anonymous, "Lean"), "Name"), []), Pi(Default, Str(Anonymous, "str"), Const(Str(Anonymous, "String"), []), Pi(Default, Str(Anonymous, "k"), Pi(Default, Str(Anonymous, "pre_eq"), App(App(App(Const(Str(Anonymous, "Eq"), [Succ(Zero)]), Const(Str(Str(Anonymous, "Lean"), "Name"), [])), BVar(1)), BVar(1)), Pi(Default, Num(Str(Str(Str(Str(Anonymous, "a_eq"), "_@"), "_internal"), "_hyg"), 0), App(App(App(Const(Str(Anonymous, "Eq"), [Succ(Zero)]), Const(Str(Anonymous, "String"), [])), BVar(1)), BVar(1)), FVar(FVarId(4)))), FVar(FVarId(4))))), context: "checking (λ pre : Lean.Name, (λ str : String, (λ k : ((((Eq.{1} Lean.Name) #1) #1) → ((((Eq.{1} String) #1) #1) → fvar_4)), ((#0 ((Eq.refl.{1} Lean.Name) #2)) ((Eq.refl.{1} String) #1)))))" } |  |
| `Lean.Name.num.noConfusion` | rejected | verified | depends on rejected declaration 'Lean.Name.noConfusion' |  |
| `Lean.Name.str.noConfusion` | rejected | verified | depends on rejected declaration 'Lean.Name.noConfusion' |  |
| `Lean.ParserDescr.binary.noConfusion` | rejected | verified | depends on rejected declaration 'Lean.ParserDescr.noConfusion' |  |
| `Lean.ParserDescr.cat.noConfusion` | rejected | verified | depends on rejected declaration 'Lean.ParserDescr.noConfusion' |  |
| `Lean.ParserDescr.const.noConfusion` | rejected | verified | depends on rejected declaration 'Lean.ParserDescr.noConfusion' |  |
| `Lean.ParserDescr.noConfusion` | rejected | verified | TypeMismatch { expected: Pi(Default, Str(Anonymous, "name"), Const(Str(Str(Anonymous, "Lean"), "Name"), []), App(Lam(Implicit, Str(Anonymous, "t"), Const(Str(Str(Anonymous, "Lean"), "ParserDescr"), []), App(App(App(Const(Str(Str(Str(Anonymous, "Lean"), "ParserDescr"), "noConfusionType"), [Param(Str(Anonymous, "u"))]), FVar(FVarId(4))), BVar(0)), BVar(0))), App(Const(Str(Str(Str(Anonymous, "Lean"), "ParserDescr"), "const"), []), BVar(0)))), got: Pi(Default, Str(Anonymous, "name"), Const(Str(Str(Anonymous, "Lean"), "Name"), []), Pi(Default, Str(Anonymous, "k"), Pi(Default, Str(Anonymous, "name_eq"), App(App(App(Const(Str(Anonymous, "Eq"), [Succ(Zero)]), Const(Str(Str(Anonymous, "Lean"), "Name"), [])), BVar(0)), BVar(0)), FVar(FVarId(4))), FVar(FVarId(4)))), context: "checking (λ name : Lean.Name, (λ k : ((((Eq.{1} Lean.Name) #0) #0) → fvar_4), (#0 ((Eq.refl.{1} Lean.Name) #1))))" } |  |
| `Lean.ParserDescr.node.noConfusion` | rejected | verified | depends on rejected declaration 'Lean.ParserDescr.noConfusion' |  |
| `Lean.ParserDescr.nodeWithAntiquot.noConfusion` | rejected | verified | depends on rejected declaration 'Lean.ParserDescr.noConfusion' |  |
| `Lean.ParserDescr.nonReservedSymbol.noConfusion` | rejected | verified | depends on rejected declaration 'Lean.ParserDescr.noConfusion' |  |
| `Lean.ParserDescr.parser.noConfusion` | rejected | verified | depends on rejected declaration 'Lean.ParserDescr.noConfusion' |  |
| `Lean.ParserDescr.sepBy.noConfusion` | rejected | verified | depends on rejected declaration 'Lean.ParserDescr.noConfusion' |  |
| `Lean.ParserDescr.sepBy1.noConfusion` | rejected | verified | depends on rejected declaration 'Lean.ParserDescr.noConfusion' |  |
| `Lean.ParserDescr.symbol.noConfusion` | rejected | verified | depends on rejected declaration 'Lean.ParserDescr.noConfusion' |  |
| `Lean.ParserDescr.trailingNode.noConfusion` | rejected | verified | depends on rejected declaration 'Lean.ParserDescr.noConfusion' |  |
| `Lean.ParserDescr.unary.noConfusion` | rejected | verified | depends on rejected declaration 'Lean.ParserDescr.noConfusion' |  |
| `Lean.ParserDescr.unicodeSymbol.noConfusion` | rejected | verified | depends on rejected declaration 'Lean.ParserDescr.noConfusion' |  |
| `Lean.SourceInfo.noConfusion` | rejected | verified | TypeMismatch { expected: Pi(Default, Str(Anonymous, "leading"), Const(Str(Str(Anonymous, "Substring"), "Raw"), []), Pi(Default, Str(Anonymous, "pos"), Const(Str(Str(Str(Anonymous, "String"), "Pos"), "Raw"), []), Pi(Default, Str(Anonymous, "trailing"), Const(Str(Str(Anonymous, "Substring"), "Raw"), []), Pi(Default, Str(Anonymous, "endPos"), Const(Str(Str(Str(Anonymous, "String"), "Pos"), "Raw"), []), App(Lam(Implicit, Str(Anonymous, "t"), Const(Str(Str(Anonymous, "Lean"), "SourceInfo"), []), App(App(App(Const(Str(Str(Str(Anonymous, "Lean"), "SourceInfo"), "noConfusionType"), [Param(Str(Anonymous, "u"))]), FVar(FVarId(4))), BVar(0)), BVar(0))), App(App(App(App(Const(Str(Str(Str(Anonymous, "Lean"), "SourceInfo"), "original"), []), BVar(3)), BVar(2)), BVar(1)), BVar(0))))))), got: Pi(Default, Str(Anonymous, "leading"), Const(Str(Str(Anonymous, "Substring"), "Raw"), []), Pi(Default, Str(Anonymous, "pos"), Const(Str(Str(Str(Anonymous, "String"), "Pos"), "Raw"), []), Pi(Default, Str(Anonymous, "trailing"), Const(Str(Str(Anonymous, "Substring"), "Raw"), []), Pi(Default, Str(Anonymous, "endPos"), Const(Str(Str(Str(Anonymous, "String"), "Pos"), "Raw"), []), Pi(Default, Str(Anonymous, "k"), Pi(Default, Str(Anonymous, "leading_eq"), App(App(App(Const(Str(Anonymous, "Eq"), [Succ(Zero)]), Const(Str(Str(Anonymous, "Substring"), "Raw"), [])), BVar(3)), BVar(3)), Pi(Default, Str(Anonymous, "pos_eq"), App(App(App(Const(Str(Anonymous, "Eq"), [Succ(Zero)]), Const(Str(Str(Str(Anonymous, "String"), "Pos"), "Raw"), [])), BVar(3)), BVar(3)), Pi(Default, Str(Anonymous, "trailing_eq"), App(App(App(Const(Str(Anonymous, "Eq"), [Succ(Zero)]), Const(Str(Str(Anonymous, "Substring"), "Raw"), [])), BVar(3)), BVar(3)), Pi(Default, Str(Anonymous, "endPos_eq"), App(App(App(Const(Str(Anonymous, "Eq"), [Succ(Zero)]), Const(Str(Str(Str(Anonymous, "String"), "Pos"), "Raw"), [])), BVar(3)), BVar(3)), FVar(FVarId(4)))))), FVar(FVarId(4))))))), context: "checking (λ leading : Substring.Raw, (λ pos : String.Pos.Raw, (λ trailing : Substring.Raw, (λ endPos : String.Pos.Raw, (λ k : ((((Eq.{1} Substring.Raw) #3) #3) → ((((Eq.{1} String.Pos.Raw) #3) #3) → ((((Eq.{1} Substring.Raw) #3) #3) → ((((Eq.{1} String.Pos.Raw) #3) #3) → fvar_4)))), ((((#0 ((Eq.refl.{1} Substring.Raw) #4)) ((Eq.refl.{1} String.Pos.Raw) #3)) ((Eq.refl.{1} Substring.Raw) #2)) ((Eq.refl.{1} String.Pos.Raw) #1)))))))" } |  |
| `Lean.SourceInfo.original.noConfusion` | rejected | verified | depends on rejected declaration 'Lean.SourceInfo.noConfusion' |  |
| `Lean.SourceInfo.synthetic.noConfusion` | rejected | verified | depends on rejected declaration 'Lean.SourceInfo.noConfusion' |  |
