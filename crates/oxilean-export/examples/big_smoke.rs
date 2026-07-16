//! Big-corpus smoke tool: stream-parse a lean4export NDJSON file and report
//! wall time, peak RSS, and full record/declaration counts.
//!
//! Usage:
//! ```sh
//! cargo run -p oxilean-export --release --example big_smoke -- <file.ndjson>
//! ```
//!
//! (To smoke a prefix of a huge corpus, pre-truncate with `head -n N`.)
//!
//! Declarations are parsed (including full materialization of their kernel
//! expressions) and then dropped, so resident memory stays proportional to the
//! primitive index tables even for multi-hundred-MB corpora.

use std::time::Instant;

use oxilean_export::{read_streaming, ExportDecl, Limits};

fn main() {
    std::process::exit(run());
}

fn run() -> i32 {
    let Some(path) = std::env::args().nth(1) else {
        eprintln!("usage: big_smoke <file.ndjson>");
        return 2;
    };

    let file = match std::fs::File::open(&path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("cannot open {path}: {e}");
            return 2;
        }
    };
    let reader = std::io::BufReader::with_capacity(1 << 20, file);

    // A corpus-scale budget: the smoke tool measures real materialization cost
    // and reports it; the default budget is deliberately smaller.
    let limits = Limits {
        materialize_budget: u64::MAX / 2,
        decl_materialize_budget: u64::MAX / 2,
    };

    let mut decl_axiom = 0usize;
    let mut decl_oversized = 0usize;
    let mut decl_def = 0usize;
    let mut decl_thm = 0usize;
    let mut decl_opaque = 0usize;
    let mut decl_quot = 0usize;
    let mut decl_inductive = 0usize;
    let mut ind_types = 0usize;
    let mut ind_ctors = 0usize;
    let mut ind_recs = 0usize;

    let start = Instant::now();
    let result = read_streaming(reader, limits, |decl: ExportDecl| {
        match &decl {
            ExportDecl::Axiom(_) => decl_axiom += 1,
            ExportDecl::Definition(_) => decl_def += 1,
            ExportDecl::Theorem(_) => decl_thm += 1,
            ExportDecl::Opaque(_) => decl_opaque += 1,
            ExportDecl::Quot(_) => decl_quot += 1,
            ExportDecl::Inductive(b) => {
                decl_inductive += 1;
                ind_types += b.types.len();
                ind_ctors += b.ctors.len();
                ind_recs += b.recs.len();
            }
            ExportDecl::Oversized(_) => decl_oversized += 1,
        }
        Ok(())
    });
    let elapsed = start.elapsed();

    match result {
        Ok((meta, stats)) => {
            println!("file:             {path}");
            println!(
                "meta:             exporter={} v{}  lean={} ({})  format={}",
                meta.exporter_name,
                meta.exporter_version,
                meta.lean_version,
                meta.lean_githash,
                meta.format_version
            );
            println!("wall time:        {:.2}s", elapsed.as_secs_f64());
            if let Some(kb) = peak_rss_kb() {
                println!("peak RSS:         {:.1} MiB", kb as f64 / 1024.0);
            }
            println!("total lines:      {}", stats.total_lines);
            println!("-- primitive records --");
            println!("  name.str        {}", stats.name_str);
            println!("  name.num        {}", stats.name_num);
            println!("  level.succ      {}", stats.level_succ);
            println!("  level.max       {}", stats.level_max);
            println!("  level.imax      {}", stats.level_imax);
            println!("  level.param     {}", stats.level_param);
            println!("  expr.bvar       {}", stats.expr_bvar);
            println!("  expr.sort       {}", stats.expr_sort);
            println!("  expr.const      {}", stats.expr_const);
            println!("  expr.app        {}", stats.expr_app);
            println!("  expr.lam        {}", stats.expr_lam);
            println!("  expr.forallE    {}", stats.expr_forall);
            println!("  expr.letE       {}", stats.expr_let);
            println!("  expr.proj       {}", stats.expr_proj);
            println!("  expr.natVal     {}", stats.expr_nat_lit);
            println!("  expr.strVal     {}", stats.expr_str_lit);
            println!("  expr.mdata      {}", stats.expr_mdata);
            println!("-- declaration records --");
            println!("  axiom           {}", stats.decl_axiom);
            println!("  def             {}", stats.decl_def);
            println!("  thm             {}", stats.decl_thm);
            println!("  opaque          {}", stats.decl_opaque);
            println!("  quot            {}", stats.decl_quot);
            println!(
                "  inductive       {} (types {}, ctors {}, recs {})",
                stats.decl_inductive, ind_types, ind_ctors, ind_recs
            );
            println!("  oversized       {}", decl_oversized);
            println!("  total decls     {}", stats.total_decls());
            println!("-- materialization --");
            println!("  kernel nodes    {}", stats.materialized_nodes);
            println!("  max per decl    {}", stats.max_decl_materialized);
            println!("-- three-bucket outcome --");
            println!("  malformed       0");
            println!("  unsupported     0");
            println!(
                "  decls surfaced  {} (axiom {}, def {}, thm {}, opaque {}, quot {}, inductive {})",
                decl_axiom + decl_def + decl_thm + decl_opaque + decl_quot + decl_inductive,
                decl_axiom,
                decl_def,
                decl_thm,
                decl_opaque,
                decl_quot,
                decl_inductive
            );
            0
        }
        Err(e) => {
            println!("wall time:        {:.2}s (failed)", elapsed.as_secs_f64());
            println!("error [{}]: {e}", e.kind());
            1
        }
    }
}

/// Peak RSS from /proc/self/status (Linux), in KiB.
fn peak_rss_kb() -> Option<u64> {
    let status = std::fs::read_to_string("/proc/self/status").ok()?;
    for line in status.lines() {
        if let Some(rest) = line.strip_prefix("VmHWM:") {
            let kb: u64 = rest.trim().trim_end_matches(" kB").trim().parse().ok()?;
            return Some(kb);
        }
    }
    None
}
