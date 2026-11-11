# Runbook — Full-Mathlib verify on a bigger box

Reproduce the whole-corpus **Mathlib** verification of `oxilean-verify` on a fresh
Linux host with more RAM than the 14 GiB dev box. On the dev box the full run OOMs
even caged at 12 GiB — not because of Stage F's node sharing (that shrank the
per-`Const` Name/Level duplication; full **Init** peaks at **2.89 GiB / 0 swap**),
but because the reader's **file-global id-index tables** (name/level/expr, kept for
the whole file so lean4export's forward-declared ids resolve on later reference)
grow with the corpus's *distinct-node total*. Mathlib is ~millions of nodes, so
those tables dominate residency. This run measures whether they fit on a 45 GB box.

See `docs/audit/2026-07-15-structural-sharing/stage-f.md` (Stage F) and `design.md`
§8 risk #7 (the reader-table wall) for background.

## 0. What the workload is (set expectations)

- **CPU-only.** No GPU / CUDA is used or needed. A CUDA image is fine; ignore the GPU.
- **Single-threaded core.** The verify pipeline runs on **one** spawned thread
  (`main.rs` → `thread::Builder::stack_size(...).spawn(run)`). Extra vCPUs speed up
  the **build** only, not the verification wall-clock.
- **Memory-bound.** Peak RSS is reached late, as the environment + reader tables
  accumulate over the whole 5.99 GB stream. This is the number of interest.
- **Time.** Full Init (345 MB) took ~41 min single-threaded. Mathlib is ~17× the
  bytes and heavier per decl; budget **several hours to ~a day**. It is not
  parallelizable.

## 1. Host requirements

- Linux x86_64, **≥ 45 GB RAM**, ≥ 8 vCPU (only the build benefits from the cores).
- **Disk: ≥ 20 GB free** — repo + `target/` (~3–4 GB) + the 5.99 GB corpus.
- `systemd` user instance available (for the memory cage). If the host has no user
  systemd (some minimal containers don't), see §5b for an `ulimit` fallback.

## 2. Base tooling

```bash
# Rust (stable; the tree builds on 1.95.0, no rust-toolchain pin file).
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
. "$HOME/.cargo/env"
rustc --version                      # expect 1.9x stable

# Build + measurement essentials.
sudo apt-get update && sudo apt-get install -y build-essential git time
#   'time' provides /usr/bin/time -v (peak RSS). 'build-essential' provides the linker.
```

CUDA/Lean toolchains are **not** required to *verify* — the corpus is already an
NDJSON export. (Lean + lake + lean4export are only needed to *regenerate* the
corpus; see §3b.)

## 3. Get the code and the corpus

### 3a. Code

Clone the repo and check out the Stage F commit (or later on branch `0.1.4`):

```bash
git clone <your-remote-or-path> oxilean && cd oxilean
git checkout 0.1.4                    # Stage F landed at e1a11c8 on this branch
git log --oneline -1                  # confirm you have e1a11c8 or a descendant
```

### 3b. Corpus (transfer the existing file — do NOT regenerate)

The 5.99 GB `Mathlib.ndjson` already exists on the dev box at
`~/work/oxilean-corpus/Mathlib.ndjson`. Copy it over and verify integrity:

```bash
# On the NEW box:
mkdir -p ~/corpus
rsync -avP  <dev-box>:~/work/oxilean-corpus/Mathlib.ndjson  ~/corpus/
# or scp -C <dev-box>:~/work/oxilean-corpus/Mathlib.ndjson ~/corpus/

# Integrity check — must match the source hash:
sha256sum ~/corpus/Mathlib.ndjson
#   expected: 9e8afb8b573c12190243aed57f0751f0d30be8188bbe821c318cd4e7df1c0625
#   (size: 5,990,689,176 bytes)
```

Regenerating from Lean+Mathlib via lean4export (`3de59f10`, Lean `v4.32.0-rc1`) is
possible but heavy (builds Mathlib first); prefer transferring the file.

## 4. Build the verifier (release)

```bash
cd oxilean
cargo build --release -p oxilean-verify
#   release profile is lto=true, codegen-units=1, opt-level=3, strip=true.
ls -la target/release/oxilean-verify
```

## 5. Run — caged and measured

Cage the run so an over-budget allocation aborts cleanly instead of freezing the
host. On 45 GB, cap at **~40 GB** (leave headroom for the OS + reclaimable page
cache; page cache is charged to the cgroup but is reclaimed before OOM, so the cap
effectively bounds the process's anonymous heap = env + reader tables).

```bash
mkdir -p ~/runs
systemd-run --user --scope --unit=mathlib-verify \
  -p MemoryMax=40G -p MemorySwapMax=0 \
  /usr/bin/time -v \
  target/release/oxilean-verify \
    --limits corpus \
    --stack-size 1024 \
    --json ~/runs/mathlib_report.json \
    ~/corpus/Mathlib.ndjson \
  |& tee ~/runs/mathlib_run.log
```

Flags:
- `--limits corpus` — the trusted whole-corpus budgets (large per-decl materialize
  budget + a 30 s per-decl wall-clock backstop). Required for a Lean-core/Mathlib run.
- `--stack-size 1024` — 1 GiB **reserved** (not committed) stack for the verify
  thread; deep def-eq recursion on big Mathlib terms needs it. Raise to `2048` if
  you hit a stack overflow (§6).
- `--json <path>` — machine-readable three-bucket report (also prints a human summary).
- `MemorySwapMax=0` — no swap; we want a clean OOM, not thrash.

Watch peak memory live in another shell (optional):

```bash
watch -n5 'systemctl --user show mathlib-verify.scope -p MemoryCurrent --value | \
  awk "{printf \"cgroup: %.2f GiB\n\", \$1/1073741824}"; \
  ps -C oxilean-verify -o rss=,etime= | \
  awk "{printf \"proc RSS: %.2f GiB  elapsed %s\n\", \$1/1048576, \$2}"'
```

Note the two numbers differ: **process RSS** (anonymous heap = env + reader tables,
the Stage-F-relevant residency) vs **cgroup MemoryCurrent** (includes reclaimable
file page cache from streaming the 6 GB file). `/usr/bin/time -v`'s *Maximum
resident set size* reports the process peak — that is the headline number.

### 5b. Fallback if there is no user systemd

```bash
# Bound address space to ~40 GiB so an over-budget alloc fails instead of freezing:
( ulimit -v 41943040 ; \
  /usr/bin/time -v target/release/oxilean-verify --limits corpus --stack-size 1024 \
    --json ~/runs/mathlib_report.json ~/corpus/Mathlib.ndjson ) |& tee ~/runs/mathlib_run.log
```

## 6. Interpret the result

**Success** looks like a final line:

```
NNNNN verified · MMMM unsupported · 0 rejected
```

Acceptance gate (same as every corpus run):
1. **`rejected == 0`** — non-negotiable soundness alarm. If `rejected > 0`, capture
   the names from `mathlib_report.json` (`.rejected[]`) and stop — that is a real finding.
2. **`verified` count** — record it; it should be a large majority.
3. **Peak RSS** — from `Maximum resident set size (kbytes)` in the `time -v` block of
   `mathlib_run.log`. Divide by 1048576 for GiB. This answers "does Mathlib fit in 45 GB".
4. **Wall clock** — from `Elapsed (wall clock) time` and `.totals.wall_ms`.

Grab the headline numbers:

```bash
grep -E 'verified|Maximum resident set size|Elapsed \(wall' ~/runs/mathlib_run.log | tail
python3 -c "import json,os;print(json.load(open(os.path.expanduser('~/runs/mathlib_report.json')))['totals'])"
```

## 7. Failure modes

- **OOM / `memory allocation of N bytes failed` / exit 137** — even 40 GB was not
  enough; the reader id-index tables exceed it. Record the peak RSS reached (from
  `time -v`, printed even on a killed child) — that quantifies the wall. Next lever
  is *not* Stage F but bounding the reader tables (stream / mmap / delta-encode ids),
  or an even larger box.
- **Stack overflow** (thread aborts with SIGSEGV early, no verdict) — rerun with
  `--stack-size 2048`.
- **`cannot spawn the verification thread`** — the requested stack exceeds host
  limits; lower `--stack-size` or raise `ulimit -s`/host memory.

## 8. Report back

Send: the final `NNNNN verified · MMMM unsupported · R rejected` line, the peak RSS
in GiB, wall time, `mathlib_report.json`, and — if `rejected > 0` — the rejected
names. That closes out the Mathlib-scale RSS question the 14 GiB box could not answer.
