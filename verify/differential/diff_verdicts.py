#!/usr/bin/env python3
# Copyright 2026 COOLJAPAN OU (Team Kitasan). Apache-2.0.
"""Join two checkers' per-declaration verdicts and emit an agree/disagree table.

This is the core of the differential-testing harness (engineering brief §5).
It takes two *normalized verdict files* — one per checker — and joins them by
declaration name. Each side maps a fully-qualified declaration name to one of a
small, fixed vocabulary of verdicts:

    verified    | the checker accepted it as a proof
    rejected    | the checker checked it and says it is NOT a proof (an alarm)
    unsupported | the checker does not implement a feature it needs (named)
    absent      | the checker never produced a verdict for this decl

The comparison classifies each *shared* declaration (present in at least one
side) into:

    AGREE          both say verified, or both say rejected
    DISAGREE       one says verified and the other says rejected  --> a FINDING
    ONLY-<side>    one side has a verdict, the other is absent (toolchain skew)
    UNSUPPORTED-*  at least one side is unsupported (not a disagreement: a
                   checker that declines to check a decl has not contradicted
                   the other; the brief's "three outcomes, all good" framing)

The three-outcomes framing (brief §5): a disagreement is not a bug report, it
is a *finding*. verified-vs-rejected is the only true contradiction and is
never hidden or reclassified. verified-vs-unsupported is expected and healthy;
absent-vs-anything is expected under version skew.

Dependency-light: Python 3 standard library only.

Normalized verdict file format (one of):
  * a `.jsonl` / `.ndjson` file, one JSON object per line:
        {"name": "Nat.add", "verdict": "verified", "detail": null}
  * a `.json` file that is EITHER
        - an oxilean-verify report from `--json-full` (has a top-level
          `decls` array), which is auto-detected and converted, OR
        - a flat object {"Nat.add": "verified", ...}, OR
        - an array of {"name","verdict"} objects.

Usage:
    diff_verdicts.py --a-name oxilean --a FILE --b-name lean4lean --b FILE \\
        [--out-md RESULTS.md] [--out-csv table.csv]
"""

import argparse
import csv
import json
import sys
from collections import Counter

VERDICTS = ("verified", "rejected", "unsupported", "absent")


def _load_json(path):
    with open(path, "r", encoding="utf-8") as fh:
        return json.load(fh)


def load_verdicts(path):
    """Return {name: {"verdict": str, "detail": str|None}} from a verdict file."""
    out = {}
    if path.endswith((".jsonl", ".ndjson")):
        with open(path, "r", encoding="utf-8") as fh:
            for line_no, raw in enumerate(fh, 1):
                raw = raw.strip()
                if not raw:
                    continue
                try:
                    obj = json.loads(raw)
                except json.JSONDecodeError as exc:
                    raise SystemExit(
                        f"{path}:{line_no}: invalid JSON line: {exc}"
                    ) from exc
                _absorb(out, obj, path, line_no)
        return out

    data = _load_json(path)
    # oxilean-verify --json-full report: has a top-level "decls" array.
    if isinstance(data, dict) and isinstance(data.get("decls"), list):
        for rec in data["decls"]:
            _absorb(out, rec, path, None)
        return out
    # Flat object {name: verdict}.
    if isinstance(data, dict):
        for name, verdict in data.items():
            out[name] = {"verdict": _norm_verdict(verdict, path), "detail": None}
        return out
    # Array of {name, verdict} objects.
    if isinstance(data, list):
        for rec in data:
            _absorb(out, rec, path, None)
        return out
    raise SystemExit(f"{path}: unrecognized verdict file shape")


def _absorb(out, obj, path, line_no):
    where = f"{path}" + (f":{line_no}" if line_no else "")
    name = obj.get("name")
    if name is None:
        raise SystemExit(f"{where}: record has no 'name' field")
    verdict = _norm_verdict(obj.get("verdict"), where)
    detail = obj.get("detail")
    out[name] = {"verdict": verdict, "detail": detail}


def _norm_verdict(v, where):
    if v is None:
        raise SystemExit(f"{where}: record has no 'verdict' field")
    v = str(v).strip().lower()
    if v not in VERDICTS:
        raise SystemExit(
            f"{where}: unknown verdict {v!r} (expected one of {VERDICTS})"
        )
    return v


def classify(va, vb):
    """Classify a joined pair of verdicts into a comparison bucket."""
    if va == "unsupported" or vb == "unsupported":
        return "unsupported-skipped"
    if va == "absent" or vb == "absent":
        return "only-one-side"
    if va == vb:  # both verified or both rejected
        return "agree"
    # {verified, rejected} on opposite sides: the one true contradiction.
    return "disagree"


def diff(a, b, a_name, b_name):
    names = sorted(set(a) | set(b))
    rows = []
    counts = Counter()
    disagreements = []
    for name in names:
        ra = a.get(name, {"verdict": "absent", "detail": None})
        rb = b.get(name, {"verdict": "absent", "detail": None})
        va, vb = ra["verdict"], rb["verdict"]
        bucket = classify(va, vb)
        counts[bucket] += 1
        counts[f"{a_name}:{va}"] += 1
        counts[f"{b_name}:{vb}"] += 1
        row = {
            "name": name,
            f"{a_name}": va,
            f"{b_name}": vb,
            "class": bucket,
            f"{a_name}_detail": ra["detail"] or "",
            f"{b_name}_detail": rb["detail"] or "",
        }
        rows.append(row)
        if bucket == "disagree":
            disagreements.append(row)
    return rows, counts, disagreements


def render_summary(counts, a_name, b_name, total):
    lines = []
    lines.append(f"total distinct declarations joined: {total}")
    lines.append("")
    lines.append("comparison buckets:")
    lines.append(f"  AGREE (both verified or both rejected) : {counts['agree']}")
    lines.append(f"  DISAGREE (verified vs rejected)        : {counts['disagree']}   <-- FINDINGS")
    lines.append(f"  ONLY-ONE-SIDE (version/toolchain skew) : {counts['only-one-side']}")
    lines.append(f"  UNSUPPORTED-SKIPPED (one side declines): {counts['unsupported-skipped']}")
    lines.append("")
    lines.append(f"{a_name} per-verdict:")
    for v in VERDICTS:
        lines.append(f"  {v:12s}: {counts[f'{a_name}:{v}']}")
    lines.append(f"{b_name} per-verdict:")
    for v in VERDICTS:
        lines.append(f"  {v:12s}: {counts[f'{b_name}:{v}']}")
    return "\n".join(lines)


def render_disagreements_md(disagreements, a_name, b_name):
    if not disagreements:
        return (
            "No verified-vs-rejected disagreements. Every declaration checked by "
            "both sides received the same accept/reject verdict.\n"
        )
    out = [
        f"Found {len(disagreements)} disagreement(s). Each is a FINDING "
        "(brief §5) — listed in full, never reclassified.\n",
        f"| declaration | {a_name} | {b_name} | {a_name} detail | {b_name} detail |",
        "|---|---|---|---|---|",
    ]
    for row in disagreements:
        out.append(
            "| `{name}` | {va} | {vb} | {da} | {db} |".format(
                name=row["name"],
                va=row[a_name],
                vb=row[b_name],
                da=(row[f"{a_name}_detail"] or "").replace("|", "\\|"),
                db=(row[f"{b_name}_detail"] or "").replace("|", "\\|"),
            )
        )
    return "\n".join(out) + "\n"


def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument("--a", required=True, help="normalized verdict file for side A")
    ap.add_argument("--b", required=True, help="normalized verdict file for side B")
    ap.add_argument("--a-name", default="A", help="label for side A (e.g. oxilean)")
    ap.add_argument("--b-name", default="B", help="label for side B (e.g. lean4lean)")
    ap.add_argument("--out-md", help="write a markdown disagreement report to PATH")
    ap.add_argument("--out-csv", help="write the full joined table to PATH")
    ap.add_argument(
        "--fail-on-disagree",
        action="store_true",
        help="exit 1 if any verified-vs-rejected disagreement is found",
    )
    args = ap.parse_args(argv)

    if args.a_name == args.b_name:
        raise SystemExit("--a-name and --b-name must differ")

    a = load_verdicts(args.a)
    b = load_verdicts(args.b)
    rows, counts, disagreements = diff(a, b, args.a_name, args.b_name)
    total = len(rows)

    summary = render_summary(counts, args.a_name, args.b_name, total)
    print(summary)
    print()
    if disagreements:
        print(f"=== {len(disagreements)} DISAGREEMENT(S) — FINDINGS ===")
        for row in disagreements:
            print(
                f"  {row['name']}: {args.a_name}={row[args.a_name]} "
                f"{args.b_name}={row[args.b_name]}"
            )
    else:
        print("no verified-vs-rejected disagreements")

    if args.out_csv:
        fields = [
            "name",
            args.a_name,
            args.b_name,
            "class",
            f"{args.a_name}_detail",
            f"{args.b_name}_detail",
        ]
        with open(args.out_csv, "w", encoding="utf-8", newline="") as fh:
            writer = csv.DictWriter(fh, fieldnames=fields)
            writer.writeheader()
            writer.writerows(rows)

    if args.out_md:
        with open(args.out_md, "w", encoding="utf-8") as fh:
            fh.write(f"## Differential verdict join: {args.a_name} vs {args.b_name}\n\n")
            fh.write("```\n")
            fh.write(summary)
            fh.write("\n```\n\n")
            fh.write("### Disagreements (findings)\n\n")
            fh.write(render_disagreements_md(disagreements, args.a_name, args.b_name))

    if args.fail_on_disagree and disagreements:
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
