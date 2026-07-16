#!/usr/bin/env python3
# Copyright 2026 COOLJAPAN OU (Team Kitasan). Apache-2.0.
"""Cut a deterministic, self-contained NDJSON slice from a lean4export corpus.

The lean4export NDJSON format is an append-only, index-addressed graph: name /
level / expression records build up a table, and a *declaration* record (`def`,
`thm`, `inductive`, `opaque`, `quot`, `axiom`) references only indices that were
already emitted before it. Therefore any line-prefix of the file that ends
exactly on a declaration record is itself a valid, self-contained NDJSON file:
every reference it contains is resolvable within the prefix.

This script keeps the leading `meta` record plus every record up to and
including the line that completes the Nth declaration record, and writes that
byte-exact prefix. The result is deterministic: byte-identical for a given
(input, N).

Usage:
    make-slice.py <INPUT.ndjson> <N> <OUT.ndjson>
        e.g. make-slice.py ~/work/oxilean-corpus/Init.ndjson 2000 init-2000.ndjson
"""

import json
import sys

DECL_KEYS = frozenset({"def", "thm", "inductive", "opaque", "quot", "axiom"})


def main(argv):
    if len(argv) != 4:
        sys.exit("usage: make-slice.py <INPUT.ndjson> <N> <OUT.ndjson>")
    inp, n_str, out = argv[1], argv[2], argv[3]
    try:
        target = int(n_str)
    except ValueError:
        sys.exit(f"N must be an integer, got {n_str!r}")
    if target <= 0:
        sys.exit("N must be positive")

    decls = 0
    kept_lines = 0
    with open(inp, "rb") as fin, open(out, "wb") as fout:
        for raw in fin:
            fout.write(raw)
            kept_lines += 1
            line = raw.strip()
            if not line:
                continue
            try:
                obj = json.loads(line)
            except json.JSONDecodeError:
                # Non-JSON line inside an NDJSON export is a corpus error.
                sys.exit(f"{inp}: line {kept_lines} is not valid JSON")
            key = next(iter(obj), None)
            if key in DECL_KEYS:
                decls += 1
                if decls >= target:
                    break

    if decls < target:
        print(
            f"warning: input had only {decls} declarations (< requested {target}); "
            f"slice contains the whole file",
            file=sys.stderr,
        )
    print(
        f"[make-slice] {inp} -> {out}: {decls} declarations, {kept_lines} lines",
        file=sys.stderr,
    )


if __name__ == "__main__":
    main(sys.argv)
