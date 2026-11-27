# OxiLean による RH / GRH（Dirichlet L 関数）形式証明・コンパクト版の独立検証レポート

**検証レポート** · OxiLean `oxilean-verify` 0.1.6 · 実行 2026-11-27T06:06:19Z 〜 2026-11-27T06:41:57Z（UTC）·
English: [riemann_verified.en.md](riemann_verified.en.md)

> **結果.** OxiLean の独立 kernel は、リーマン予想（RH）と Dirichlet L 関数の一般化リーマン予想（GRH）の
> Lean による形式証明のコンパクト版（github.com/lsernau/grh-lean、commit `72fead10`）のエクスポートに含まれる
> **204,583 宣言をすべて検査し、204,583 verified · 0 unsupported · 0 rejected** とした。さらに comparator
> 相当の比較で、5 つの定理が challenge ファイルの命題とまったく同じ命題を、Lean の標準 3 公理
> `propext`・`Classical.choice`・`Quot.sound` だけから証明していることを確認した（**`compare verdict: ESTABLISHED`**）。

Andrew Sutherland 氏が同じエクスポートについて受理を報告している 6 つの kernel（Lean の既定・paranoid、
nanoda、lean4lean、con-leche、con-ron）に、Rust で独立に実装された 7 つ目の kernel が加わったことになる。

---

## 1. 結果の要約

| 項目 | 結果 |
|---|---|
| 検査した宣言 | **204,583 verified · 0 unsupported · 0 rejected**（終了コード 0） |
| 5 つの定理 | 5 / 5 が、challenge と同一の命題（型・宇宙パラメータ。束縛名まで含めた構造比較）を持つ**定理**で、kernel が証明を verified にした |
| challenge のその他の宣言 | challenge の 52,173 レコード中 52,172 が solution 中に同一で存在。残る 1 件は意図どおり除外する `sorryAx`。差異 0・欠落 0 |
| 5 定理の依存閉包 | 204,579 レコード。未検証 0、`unsafe`/`partial` 0、未宣言の定数 0 |
| 5 定理が依存する公理 | `Classical.choice`、`Quot.sound`、`propext`（許可リストと一致） |
| エクスポート全体の公理レコード | `propext`、`Quot.sound`、`Classical.choice`（ほかになし） |
| 所要時間 / ピークメモリ | 35 分 38 秒（1 スレッド）/ 13.2 GiB |

## 2. 検証対象

| 項目 | 値 |
|---|---|
| 証明 | https://github.com/lsernau/grh-lean、commit `72fead102f663d5c7a407964a13dc0cf9d977207`、ディレクトリ `grh-compact/`（794 ファイル、約 7.6 万行） |
| Lean・Mathlib | Lean v4.35.0（githash `eb6cafb7290e8bd5f2937522d8d804f58f02bafc`）、Mathlib `56a005855ea17e0b0958f73ec801ae140b0a1679` |
| solution エクスポート | `compact-solution.ndjson`、5,172,274,528 バイト、SHA-256 `ab7b4223b9bd748a1aa68bdb63afb6d6264121fc834aadba7ba3d3796c32115b` |
| challenge エクスポート | `challenge.ndjson`、354,827,875 バイト、SHA-256 `20eb99a04b40f6afe1b44931e7ee82b15208542517dc588ace9634acffc41694` |
| エクスポートの入手元 | https://math.mit.edu/~drew/shortgrh/ — `lake comparator`（Lean v4.35.0）が作成したもの。2 台で独立に作ってバイト一致と報告されている。上の SHA-256 はダウンロード後に再計算し、公開値と一致することを確認した |
| 形式 | lean4export NDJSON、format version 3.1.0（meta レコード: exporter lean4export 3.1.0、Lean 4.35.0） |

5 つの定理（`src/Main.lean`）。`RiemannHypothesis` は Mathlib 自身の定義で、ほかの 4 つの命題は Mathlib だけを import する
`src/Statements.lean` で定義されている:

```lean
theorem riemannHypothesis : RiemannHypothesis                -- Mathlib の RiemannHypothesis
theorem riemannZeta_eq_zero_re : RiemannZetaZerosOnLine
theorem LFunction_ne_zero_of_half_lt_re : DirichletLNonvanishingRightOfHalf
theorem LFunction_eq_zero_re : DirichletLZerosOnLine
theorem LFunction_eq_zero_re_of_isPrimitive : PrimitiveDirichletLZerosOnLine
```

エクスポートには、5 つの定理と、それらが推移的に依存するすべての宣言（プロジェクト自身の補題、Mathlib、
Lean core）が、完全に elaborate された kernel の項として入っている。読み込む検証器はそれらをすべて検査し直し、
コンパイル済みの `.olean` からは何も受け取らない。

## 3. 検証器

| 項目 | 値 |
|---|---|
| ツール | `oxilean-verify` 0.1.6（https://github.com/cool-japan/oxilean） |
| ソース | `0.1.6` ブランチの commit `69d282f80d64890794a1ffae4e22c83f215de5e7`（"Riemann 2"）。バイナリは `fb67fd5c` に、`69d282f8` がコミットした `crates/oxilean-kernel/src/inductive/derive.rs` の変更を加えたツリーからビルドした（ビルド時の記録: `fb67fd5c29292bdff874e17131cd30323f180c71 + working-tree.diff (sha256 398c87d47da0c4a0d983c558f5a9b4031db7ec2f5bf0e209b0399343d1141068)`）。両者の Rust ソースはバイト単位で同一（差分を比較して確認） |
| バイナリ | SHA-256 `52aeea38a2494b9f1c7a1f0f175aff95ead660a83bf7f4c76a21dd8b06bb3592`（release プロファイル。§4） |
| 信頼基盤（TCB） | `oxilean-kernel`（型検査器）+ `oxilean-export`（NDJSON reader）。外部依存ゼロ、`#![forbid(unsafe_code)]`。CLI も依存を増やさない |
| 独立性 | Lean の C++ kernel、lean4lean、nanoda、con-leche、con-ron とコードを共有しない Rust の別実装。エクスポートだけを読み、Lean のバイナリも `.olean` も使わない |

検査の内容:

- **すべての宣言**を、**空の環境**から順に再生する。定義・定理・opaque 定数は型検査し、帰納型は検査したうえで
  再帰子を **kernel 自身が導出**する（エクスポート中の再帰子は導出結果と比較するだけで、信用しない）。商の
  組み込み定数は kernel 自身のものと比較し、公理は記録する。各宣言の判定は必ず 1 つ:
  `verified`、`unsupported`（名前付きの未対応機能か資源上限。推測はしない）、`rejected`（証明ではない）。
- **`--compare <challenge>`** は、同じ実行で verified になった宣言について、`lake comparator` が kernel の再生
  以外に確かめることを検査する:
  1. 各 **target**（challenge が `sorry` で「証明」している定理。自動検出。今回は上の 5 つ）が、solution 中で
     challenge と型・宇宙パラメータが同一の**定理**であり、verified であること。
  2. challenge の**その他の全宣言**（命題が依存するもの: `riemannZeta`、`DirichletCharacter.LFunction`、`Complex`、
     `Real` など。challenge の全レコードから 5 つの target と `sorryAx` を除いた、Mathlib・Lean core・`Statements.lean` の 52,167 レコード）が、solution 中に同じ名前で**構造的に同一**（種類、
     宇宙パラメータ、型、値、簡約ヒント、帰納型の構造、束縛名、束縛注釈）に存在すること。項は、共有部分項の
     組を 1 回ずつだけ訪れる厳密な構造等価で比較し、ハッシュだけでは判断しない（Lean の `==` より厳しい）。
  3. solution 中の target の**依存閉包**（型・値・コンストラクタ・再帰子規則をたどる）が、許可された公理、
     verified な宣言だけからなり、エクスポートが宣言していない定数も、`unsafe`・`partial` な宣言も含まないこと。

## 4. 実行環境

| 項目 | 値 |
|---|---|
| CPU | Intel Xeon Gold 5315Y @ 3.20 GHz、8 vCPU（Xen HVM ゲスト）、1 コア 1 スレッド |
| メモリ | 44 GiB、スワップなし、`vm.overcommit_memory = 1` |
| OS | Docker コンテナ上の Ubuntu 22.04.3 LTS、Linux 6.8.0-138-generic x86_64 |
| ストレージ | 入力はコンテナの overlay ファイルシステム、出力はネットワークファイルシステム |
| Rust | rustc 1.100.0 (0ef1bc386 2026-11-09)。クレートの MSRV は 1.89 |
| ビルド | `cargo build --release -p oxilean-verify`: `lto = true`、`codegen-units = 1`、`opt-level = 3`（ワークスペースの設定）。デバッグ情報は残した（`CARGO_PROFILE_RELEASE_DEBUG=1`、`CARGO_PROFILE_RELEASE_STRIP=false`）。生成コードは変わらない |
| スレッド | 検査は 1 スレッド |
| その他 | ダウンロードした `.zst` の展開にだけ Python 3.11 と `zstandard` パッケージを使用 |
| 同時実行 | 同じ計算機で、§6.3 の既定予算での実行を同時に走らせた |

## 5. 実行条件

```bash
oxilean-verify --limits corpus --decl-fuel 2^30 --decl-timeout 1800 --stack-size 16384 \
  --no-color --json full.json --json-full \
  --compare challenge.ndjson compact-solution.ndjson > full.out
```

| オプション | 意味 |
|---|---|
| `--limits corpus` | 信頼できるコーパス全体向けのプリセット（reader の予算が大きい） |
| `--decl-fuel 2^30` | 宣言ごとの資源予算（kernel の仕事量の単位）。プリセットの 2^26 の 16 倍 |
| `--decl-timeout 1800` | 宣言ごとの壁時計の締切（30 分） |
| `--stack-size 16384` | 検査スレッドに 16 GiB のスタックを予約（実際に触ったページだけがメモリを使う） |
| `--json full.json --json-full` | 全宣言の判定と時間を含む機械可読レポート |
| `--compare challenge.ndjson` | §3 の comparator 相当の検査 |

予算・締切・スタックは資源であって論理の一部ではない。使い切った宣言は名前付きの **unsupported** になる
だけで、verified にも rejected にもならない。この実行では一度も使い切っていない（unsupported 0）。

## 6. 結果

### 6.1 判定と比較（`full.out` の末尾そのまま）

```
204583 verified · 0 unsupported · 0 rejected
compare: challenge /tmp/grh/exports/challenge.ndjson
         sha256 20eb99a04b40f6afe1b44931e7ee82b15208542517dc588ace9634acffc41694, 52173 declaration records
  ✓  target riemannHypothesis                        statement identical to the challenge; proof verified
  ✓  target riemannZeta_eq_zero_re                   statement identical to the challenge; proof verified
  ✓  target LFunction_ne_zero_of_half_lt_re          statement identical to the challenge; proof verified
  ✓  target LFunction_eq_zero_re                     statement identical to the challenge; proof verified
  ✓  target LFunction_eq_zero_re_of_isPrimitive      statement identical to the challenge; proof verified
  statement closure: 52172 of 52173 challenge records identical in the solution (0 differ, 0 missing; sorryAx excluded)
  target closure: 204579 declaration records; 0 not verified; 0 unsafe/partial; 0 constants undeclared
  axioms used by the targets: Classical.choice, Quot.sound, propext (permitted: propext, Classical.choice, Quot.sound)
  axiom records in the solution file: propext, Quot.sound, Classical.choice
compare verdict: ESTABLISHED — the solution proves the 5 challenge statement(s) from the permitted axioms
```

### 6.2 所要時間とメモリ

| 項目 | 値 |
|---|---|
| 開始 / 終了（UTC） | 2026-11-27T06:06:19Z / 2026-11-27T06:41:57Z |
| 壁時計時間 | **2,138.1 秒（35 分 38 秒）** |
| kernel の検査時間（宣言ごとの合計） | 1,957.3 秒 |
| それ以外（入力の SHA-256 計算、5.17 GB の読み込みと解析、比較、レポート） | 180.8 秒 |
| ピーク常駐メモリ | **13.2 GiB**（13,795,100 KiB） |
| 宣言ごとの時間 | 中央値 0.61 ms、90 パーセンタイル 9.92 ms、99 パーセンタイル 177.9 ms、99.9 パーセンタイル 704.0 ms、最大 46.1 秒 |
| 100 ms / 1 秒 / 10 秒を超えた宣言 | 3,344 / 112 / 7（全 204,583） |

種類別の kernel 時間:

| 種類 | 宣言数 | kernel 時間（秒） |
|---|---:|---:|
| `thm` | 165,589 | 1,477.0 |
| `def` | 36,489 | 86.8 |
| `inductive` | 2,419 | 393.5 |
| `axiom` | 3 | 0.0 |
| `opaque` | 79 | 0.0 |
| `quot` | 4 | 0.0 |

時間のかかった宣言（上位 20）:

| # | 時間 | 種類 | 宣言 |
|---:|---:|---|---|
| 1 | 46.1 s | `thm` | `Filling.eq_one_of_reduceAt_eq_one._proof_1_4` |
| 2 | 28.6 s | `thm` | `IsCutBessel.norm_floorForm_le._proof_1_35` |
| 3 | 26.9 s | `thm` | `SplitPlaces.card_trivialised_of_isHigmanProjective` |
| 4 | 25.3 s | `thm` | `exists_analyticFamily_positiveBall` |
| 5 | 21.6 s | `thm` | `exists_isAbelianScheme` |
| 6 | 17.9 s | `thm` | `Filling.amplitudeAbs_lt_top_and_cut_le._proof_1_52` |
| 7 | 12.8 s | `thm` | `Filling.energy_sub_cut_le._proof_1_12` |
| 8 | 9.4 s | `thm` | `Filling.IsNear.le_classSize_and_cellMass_le._proof_1_15` |
| 9 | 6.1 s | `thm` | `hodgeTateMap_galois_twist` |
| 10 | 5.2 s | `thm` | `IsSplitPrimeToDisc.exists_place_above_ne._proof_1_19` |
| 11 | 5.2 s | `thm` | `hasBandLimits_of_isCutBessel._proof_1_55` |
| 12 | 5.1 s | `thm` | `ellAdicCohom.isCrystalline_and_linearFrob_rel` |
| 13 | 4.6 s | `thm` | `Filling.IsFillingLevel.exists_intermediateGroup_lines` |
| 14 | 4.5 s | `thm` | `Data.hasGap_gridParam._proof_1_6` |
| 15 | 3.8 s | `thm` | `CutsOutLocus.IsNoetherNormalization.pullback_injective` |
| 16 | 3.8 s | `thm` | `CMLattice.translatedWpDeriv_add_mem_multiPhiRing._proof_1_15` |
| 17 | 3.5 s | `thm` | `exists_isPrimeTriangle` |
| 18 | 3.5 s | `thm` | `IsSplitPrimeToDisc.exists_place_above_ne._proof_1_20` |
| 19 | 2.8 s | `thm` | `denominatorLe_rowBudget` |
| 20 | 2.8 s | `thm` | `exists_torusModel` |

5 つの定理そのもの（証明本体は、ファイル中でそれより前に検査される補題）:

| 定理 | 時間 |
|---|---:|
| `riemannHypothesis` | 0.08 ms |
| `riemannZeta_eq_zero_re` | 0.05 ms |
| `LFunction_ne_zero_of_half_lt_re` | 0.00 ms |
| `LFunction_eq_zero_re` | 0.00 ms |
| `LFunction_eq_zero_re_of_isPrimitive` | 0.08 ms |

### 6.3 同じバイナリを既定の予算で走らせた結果

本実行と並べて、既定の `--limits corpus`（予算 2^26、締切 30 秒、スタック 512 MiB）と `--compare` で走らせた
結果は **204,425 verified · 158 unsupported · 0 rejected**、終了コード 3、compare は
NOT ESTABLISHED、32 分 12 秒、ピーク 13.1 GiB。unsupported の内訳は、既定の予算を超えた
8 宣言（`IsCutBessel.norm_floorForm_le._proof_1_35`, `Filling.amplitudeAbs_lt_top_and_cut_le._proof_1_52`, `Filling.eq_one_of_reduceAt_eq_one._proof_1_4`, `Filling.energy_sub_cut_le._proof_1_12`, `Filling.IsNear.le_classSize_and_cellMass_le._proof_1_15`, `IsSplitPrimeToDisc.exists_place_above_ne._proof_1_19`, `exists_analyticFamily_positiveBall`, `SplitPlaces.card_trivialised_of_isHigmanProjective`）と、それらに依存する 150 宣言で、いずれも本実行では verified である。
**どちらの実行でも rejected は 0。** 結論は本実行（§5）に基づく。既定の予算でこのエクスポートの全宣言が
通らないことは性能上の制約であって、健全性の問題ではない。

### 6.4 出力ファイル

| ファイル | サイズ（バイト） | SHA-256 |
|---|---:|---|
| `full.out`（宣言ごとに 1 行、続いて要約と compare 節） | 13,816,044 | `b82e5a89cc18ad850e813394d46e9b7214bacb031ef096bebdfac5182d9078fb` |
| `full.json`（`--json-full` のレポート。宣言ごとの時間を含むので実行ごとに変わる） | 33,259,025 | `567c783d1bd1a569602b8a688a2549820bd64290e943255be3b206e33a13bf1c` |

`full.out` の判定行にも宣言ごとの時間が入るので、ハッシュは実行ごとに変わる。再現で一致を確かめるべきは、
`grep -c '^✓' full.out`（204,583）と、末尾の要約と compare の行（§6.1）である。

## 7. 他の検証器との比較（同じ solution エクスポート）

他の検証器の数値は Andrew Sutherland 氏のログ（https://math.mit.edu/~drew/shortgrh/、「machine B」: 192 コアの
AWS r8a.48xlarge、メモリ 1.5 TB）による。計算機が違うので、時間は目安である。

| 検証器 | 実装 | 受理した宣言 | 壁時計 | CPU 時間 | ピークメモリ |
|---|---|---:|---:|---:|---:|
| Lean 既定 kernel（`leanchecker --from-export`） | C++ | すべて | 33 分 05 秒 | 1,980 秒 | 7.0 GiB |
| lean4lean | Lean | 204,580 | 28 分 39 秒 | 1,716 秒 | 6.9 GiB |
| con-leche | — | 204,583 | 1 分 53 秒（並列） | 1,743 秒 | 13.7 GiB |
| con-ron | — | 204,583 | 1 分 16 秒（並列） | 470 秒 | 7.5 GiB |
| **OxiLean `oxilean-verify`** | **Rust** | **204,583** | **35 分 38 秒（1 スレッド）** | **約 2,138.1 秒** | **13.2 GiB** |

lean4lean の 204,580 は、`Quot` の 4 つの組み込み定数を 1 つと数えた値（Sutherland 氏の注記）。

## 8. 再現手順

### 8.1 必要なもの

- x86-64 か arm64 の Linux または macOS。**メモリは最低 16 GiB、推奨 32 GiB**（この実行のピークは 13.2 GiB）、入力用の空きディスク約 12 GB。
- Rust 1.89 以上（https://rustup.rs の `rustup`）。
- `zstd` コマンド（`zstd -d`）、または Python 3 と `zstandard` パッケージ。
- 検査に約 40 分。

### 8.2 入力の取得と確認

```bash
mkdir grh && cd grh
curl -LO https://math.mit.edu/~drew/shortgrh/challenge.ndjson.zst
curl -LO https://math.mit.edu/~drew/shortgrh/compact-solution.ndjson.zst
zstd -d challenge.ndjson.zst compact-solution.ndjson.zst
sha256sum challenge.ndjson compact-solution.ndjson
# 20eb99a04b40f6afe1b44931e7ee82b15208542517dc588ace9634acffc41694  challenge.ndjson
# ab7b4223b9bd748a1aa68bdb63afb6d6264121fc834aadba7ba3d3796c32115b  compact-solution.ndjson
```

`zstd` がない場合:

```bash
python3 -m venv venv && venv/bin/pip install zstandard
for f in challenge compact-solution; do
  venv/bin/python -c "import zstandard; zstandard.ZstdDecompressor().copy_stream(open('$f.ndjson.zst','rb'), open('$f.ndjson','wb'))"
done
```

### 8.3 検証器の入手

crates.io から:

```bash
cargo install oxilean-verify --version 0.1.6 --locked
```

または、ソースから:

```bash
git clone https://github.com/cool-japan/oxilean && cd oxilean
git checkout v0.1.6        # 0.1.6 リリース
cargo build --release -p oxilean-verify
# バイナリは target/release/oxilean-verify
```

`oxilean-verify --version` で、バージョンと読み込む lean4export の形式が表示される。

### 8.4 実行

```bash
oxilean-verify --limits corpus --decl-fuel 2^30 --decl-timeout 1800 --stack-size 16384 \
  --no-color --json full.json --json-full \
  --compare challenge.ndjson compact-solution.ndjson > full.out
echo "exit status: $?"
```

進捗は `grep -c '^✓' full.out` で確認できる（エクスポートの宣言は 204,583 個）。

### 8.5 期待される結果

- 終了コード **0**。
- `grep -c '^✓' full.out` が **204583**、`grep -c '^[⊘✗]' full.out` が **0**。
- `full.out` の末尾が §6.1 のとおり: 要約行 `204583 verified · 0 unsupported · 0 rejected`、5 行の
  `✓  target …`、そして `compare verdict: ESTABLISHED — the solution proves the 5 challenge statement(s) from the permitted axioms`。
- `full.json` では `totals = {verified: 204583, unsupported: 0, rejected: 0}`、`compare.established = true`。

終了コード 1 は rejected の宣言があったこと、2 は入力の破損か使い方の誤り、3 は rejected はないが challenge の
命題を確立できなかったことを意味する。

### 8.6 変形

- **既定の予算**: `--decl-fuel`、`--decl-timeout`、`--stack-size` を外す。§6.3 のとおり 204,425 · 158 · 0、
  終了コード 3 になる。
- **比較なし**: `--compare challenge.ndjson` を外す。要約行は同じ。
- **challenge だけ**: `oxilean-verify --limits corpus challenge.ndjson` で、challenge の 52,173 レコード（命題と、それが依存するすべて）を検査する
  （約 2 分。こちらでは 52,173 verified · 0 · 0）。

### 8.7 エクスポートを自分で作る（任意）

上の手順は Sutherland 氏のエクスポートから始め、SHA-256 で同一性を確かめている。Lean のソースから始めたい
場合は、Lean v4.35.0 でプロジェクトをビルドし、氏のページのとおりに 5 定理をエクスポートする:

```bash
git clone https://github.com/lsernau/grh-lean && cd grh-lean
git checkout 72fead102f663d5c7a407964a13dc0cf9d977207
cd grh-compact
lake exe cache get
lake build                                  # 16 コアで約 35 分
lake env leanexport Main -- riemannHypothesis riemannZeta_eq_zero_re \
  LFunction_ne_zero_of_half_lt_re LFunction_eq_zero_re LFunction_eq_zero_re_of_isPrimitive \
  > compact-solution.ndjson
```

（challenge エクスポートは `lake comparator` も書き出す。）公開ファイルとバイト単位で一致するはずである。

## 9. この検証のために OxiLean に加えた変更

この検証で、OxiLean 0.1.5 の不足が 5 つ見つかった。すべて 0.1.6 で直してあり、どの修正も、正しくない証明を
verified にする方向には働かない。

1. **定義等価判定を Lean と同じアルゴリズムにした（kernel）。** 以前は、比較のたびに両辺を（頭部の定義を
   すべて展開した）完全な弱頭部正規形にしてから比べていた。このエクスポートでは `2000000 • x`（ℝ）を
   200 万段の `Real.add` として評価し、スタック溢れ、33 GB でのメモリ枯渇、予算切れのいずれかになった。
   現在は Lean の `is_def_eq_core` と同じく、頭部の定数を展開しない弱頭部正規形 → proof irrelevance →
   lazy δ reduction（定義の高さが高い側を 1 段ずつ展開し、同じ定義どうしならまず引数を比較する）→ 構造比較、
   の順に行う。proof irrelevance では 2 つの命題を推論したまま比較する。展開は正しい簡約であり、引数比較の
   `true` は合同性から結論できるので、新しい順序が見つけるのは実際に成り立つ等式だけである。どちらの頭部も
   展開できなくなった時点で構造比較が見る項は、以前と同じである。
   `exists_data._proof_1_66`: 予算 2^32・スタック 16 GiB で 70 秒 → 72 ms、
   `g₂_g₃_eq_eisenstein`: 予算 2^28 で 17.4 秒 → 207 ms（§6 の実行での時間。どちらも既定の予算内）。
2. **帰納型の宣言で、望遠鏡（telescope）を閉じるときに共有を考慮した抽象化を使うようにした（kernel）。**
   帰納型のモジュールは、部分項をそこへの経路の数だけ訪れる走査で変数を抽象化しており、Mathlib 風の構造体の
   大きく共有の多いフィールド型で指数的になっていた。同じ項を計算する kernel の共有考慮版に置き換えた。
   `IsPrimeTriangle`: 116.5 秒 → 0.87 秒、帰納型 2,419 宣言の kernel 時間の合計: 582 秒 → 393 秒。
3. **項の表示を有界にした（kernel）。** 共有の多い証明項を木として書き出すエラーメッセージのせいで、1 宣言が
   報告される前に 24 GB まで膨張していた。表示は 2,048 部分項で打ち切る。
4. **`--compare`（verify）。** 全宣言が型検査を通ることは、**何を**、**何から**証明したかを教えない。§3 の
   comparator 相当の検査は 0.1.6 の新機能である（満たさなければ終了コード 3）。
5. **safe な宣言は `unsafe` な定数も `partial` な定義も使えない（kernel）。** Lean の `infer_constant` の規則。
   このエクスポートには `unsafe`・`partial` な宣言がないので今回の結果には影響しないが、ほかの入力に対する
   健全性の穴を塞ぐ。

ほかに `--decl-fuel`・`--decl-timeout` と、宣言ごとに kernel が何を展開したかを数える診断用 feature
`profile-unfold`（既定で無効）を加えた。1 と 2 を突き止めたのはこの道具である。

リリースツリーでの回帰確認: `oxilean-kernel`・`oxilean-export`・`oxilean-verify` のテスト 4,055 件（新規の
lazy δ テスト 3 件を含み、うち 2 件は旧アルゴリズムでは失敗する）と、ワークスペース全体のテスト 33,953 件が
通過（779 件は ignore）、`cargo clippy` の警告 0、challenge エクスポートは 0.1.5 でも新しい kernel でも全件 verified（52,173 · 0 · 0）。

## 10. 検証の経過

時刻はすべて UTC、2026-11-27、§4 の計算機。

| 時刻 | 出来事 |
|---|---|
| 01:37 | OxiLean 0.1.5 + `--compare` で最初の実行: challenge エクスポートは 52,173 · 0 · 0（110 秒）で全件 verified |
| 01:48–02:05 | solution の実行が 116,627 番目の宣言 `g₂_g₃_eq_eisenstein` に達した時点で（それまでは全件 verified）、メモリが 24 GB まで膨張したため停止。その宣言と依存先だけを抜き出したエクスポートでスタックを採取し、項をエラーメッセージに書き出す処理に時間を使っていたことが分かった（不足 3） |
| 02:11 | 表示を有界にすると、同じ宣言は予算切れの unsupported として報告されるようになった。予算 2^28 なら 17 秒で verified |
| 02:12–02:53 | 予算 2^28・2^30・2^32 で実行。rejected はないが、最初の 137,788 宣言中 77 宣言が 2^28 を超え（最初が `exists_data._proof_1_66`）、2^30 ではその宣言が 512 MiB のスタックを溢れさせ、2^32・スタック 16 GiB ではそれを通過したものの、122,375 番目の宣言で 33 GB に達しメモリ不足で強制終了された |
| 04:31 | `profile-unfold` で計測: `nsmulRec`・`Real.add` の単項評価が 300 万段 — `2000000 • x` を最後まで評価していた。根本原因は、定義等価判定が最初に両辺を完全な弱頭部正規形にしていたこと（不足 1）。Lean の lazy δ reduction に直した |
| 04:47–05:26 | 新しい kernel で実行: 予算 2^30 で 204,583 · 0 · 0、`ESTABLISHED`。既定の予算では 9 宣言が予算超え |
| 05:53–06:05 | `IsPrimeTriangle`（116 秒）を計測し、帰納型モジュールの抽象化が共有部分項を経路の数だけ訪れていることを発見（不足 2）。修正で 1 秒未満 |
| 2026-11-27T06:06:19Z 〜 2026-11-27T06:41:57Z | 本レポートの実行（0.1.6 リリースツリー） |

## 11. 注意事項

- 確立されたのは、**Lean が elaborate した 5 つの命題**が、エクスポート中の証明項によって、許可された 3 公理から
  証明されていることである。それらの命題が RH・Dirichlet L 関数の GRH として意図どおりかどうかは、
  `Statements.lean` と Mathlib の `riemannZeta`・`DirichletCharacter.LFunction`・`RiemannHypothesis` の定義を読んで
  判断すべき事柄である（Sutherland 氏のページに、`pp.all` で完全に展開した 5 つの命題がある）。
- エクスポートは Sutherland 氏の作成したもので、こちらでは SHA-256 で同一性を確かめただけで、再生成はしていない
  （方法は §8.7）。
- OxiLean の reader が想定する版は lean4export 3.1.0 / Lean v4.32.0-rc1 だが、ファイルは Lean v4.35.0 が書いた
  lean4export format 3.1.0 で、形式は同じである。v4.32 以降に kernel の規則が加わっていれば、unsupported か
  rejected の宣言として現れたはずだが、1 件もなかった。
- これは機械による検査であり、議論を数学者が読んで理解したかどうかについては何も言わない。

## 12. 参考

- 証明: https://github.com/lsernau/grh-lean（commit `72fead10`）
- エクスポートと他の検証器のログ: Andrew Sutherland 氏、https://math.mit.edu/~drew/shortgrh/
- Michael Rubinstein 氏の検査と解説: https://www.math.uwaterloo.ca/~mrubinst/alpoge_claude_grh/
- OxiLean: https://github.com/cool-japan/oxilean — `oxilean-verify` の説明: [docs/VERIFY.md](docs/VERIFY.md)
- 0.1.6 の変更点: [CHANGELOG.md](CHANGELOG.md)
