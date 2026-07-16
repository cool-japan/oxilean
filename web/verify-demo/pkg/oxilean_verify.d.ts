/* tslint:disable */
/* eslint-disable */

/**
 * A single declaration verdict handed to JS.
 *
 * This is the shape the per-decl callback receives (as a plain object via
 * wasm-bindgen). The three buckets are kept distinct: `verdict` is exactly one
 * of `"verified"`, `"unsupported"`, or `"rejected"`, and `rejected` is the
 * alarm — never summed with `unsupported`.
 */
export class DeclVerdict {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    /**
     * For `unsupported`, the named missing feature; for `rejected`, the reason;
     * empty for `verified`.
     */
    readonly detail: string;
    /**
     * The 0-based index of this declaration in file order.
     */
    readonly index: number;
    /**
     * The declaration kind label (`"axiom"`, `"def"`, `"thm"`, ...).
     */
    readonly kind: string;
    /**
     * Microseconds spent checking this declaration (only meaningful for
     * `verified`; `0` otherwise). Returned as `f64` because JS numbers are
     * doubles — the value fits exactly for any realistic per-decl time.
     */
    readonly micros: number;
    /**
     * Milliseconds spent checking this declaration, as a float (convenience for
     * the demo's right-aligned `N.N ms` column).
     */
    readonly ms: number;
    /**
     * The declaration's fully-qualified name.
     */
    readonly name: string;
    /**
     * The verdict bucket: `"verified"`, `"unsupported"`, or `"rejected"`.
     */
    readonly verdict: string;
}

/**
 * Which resource-limit preset a [`VerifySession`] runs with.
 *
 * Presets map to [`oxilean_export::Limits`]. The default is the small,
 * untrusted-input budget — correct for a browser eating a dropped file. The
 * corpus preset raises the materialization budget for large, trusted exports.
 */
export enum LimitsPreset {
    /**
     * The small untrusted-input budget ([`Limits::default`]): the right default
     * for a file a stranger dropped into the page.
     */
    Default = 0,
    /**
     * The whole-corpus budget ([`Limits::corpus`]) for large trusted exports.
     */
    Corpus = 1,
}

/**
 * A streaming verify session.
 *
 * Lifecycle, matching how the page reads a `File`:
 *
 * 1. [`VerifySession::new`] with a [`LimitsPreset`].
 * 2. [`VerifySession::push_chunk`] repeatedly — one call per slice the page
 *    reads from the dropped file. Chunks are appended in order; the engine is
 *    not run yet (it needs the whole stream to resolve back-references in the
 *    lean4export index tables).
 * 3. [`VerifySession::finish`] with a per-declaration JS callback. This runs the
 *    real [`oxilean_verify::verify_stream`] engine over the accumulated bytes,
 *    invoking the callback once per declaration in file order, then returns the
 *    summary + pins as a JSON string.
 *
 * A session is single-shot: after [`finish`](VerifySession::finish) it should be
 * dropped. Pushing more chunks after finishing simply appends to bytes that are
 * no longer consulted.
 */
export class VerifySession {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Run the verifier over everything pushed, streaming one verdict per
     * declaration to `on_decl`, and return the summary + pins JSON.
     *
     * `on_decl` is a JS function called once per declaration, in file order,
     * with a single [`DeclVerdict`] argument. It is invoked synchronously from
     * inside the engine; to keep the UI live the page should let the whole
     * `finish` run inside a worker, or slice the work — but the engine itself
     * runs to completion here.
     *
     * # Errors
     * Returns a JS error string (via `Err`) when the file cannot be read to
     * completion: broken JSON, a lean4export spec violation, or the
     * materialization budget being exceeded. This is **"the file is broken"**,
     * distinct from any per-declaration `rejected` verdict — do not show it in
     * the rejected bucket.
     */
    finish(on_decl: Function, path: string): VerifySummary;
    /**
     * Create a new session with the given resource-limit preset.
     */
    constructor(preset: LimitsPreset);
    /**
     * Append one raw byte chunk (a `Uint8Array` slice of the file) to the
     * session. Preferred over [`push_chunk`](VerifySession::push_chunk) when the
     * page reads the file as bytes rather than decoded text, so the exact file
     * fingerprint is preserved.
     */
    push_bytes(chunk: Uint8Array): void;
    /**
     * Append one text chunk (a slice of the dropped file) to the session.
     *
     * The page reads the `File` in slices and calls this once per slice, in
     * order. Chunks are concatenated verbatim; verification runs only on
     * [`finish`](VerifySession::finish).
     */
    push_chunk(chunk: string): void;
    /**
     * The number of bytes pushed so far.
     */
    readonly byte_len: number;
    /**
     * The lowercase-hex SHA-256 fingerprint of everything pushed so far.
     *
     * Computed client-side (in the page's WebAssembly memory) so the demo can
     * show the exact file fingerprint under the "0 bytes uploaded" claim — the
     * bytes never leave the machine.
     */
    readonly sha256: string;
}

/**
 * The final three-bucket summary returned by [`VerifySession::finish`].
 */
export class VerifySummary {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    /**
     * The deterministic JSON report as a string: `tool`, `pins`, `input`,
     * `totals`, `unsupported_features`, `rejected`. Identical in shape to the
     * native CLI's `--json` output (timing masked to whole ms). The demo can
     * display or offer this for download — it is produced entirely in the page,
     * so nothing is uploaded.
     */
    readonly pins_json: string;
    /**
     * Declarations rejected — the alarm bucket. Never summed with
     * `unsupported`.
     */
    readonly rejected: number;
    /**
     * Total declarations seen.
     */
    readonly total: number;
    /**
     * Declarations deferred as unsupported (a named feature is missing).
     */
    readonly unsupported: number;
    /**
     * Declarations verified (checked and a proof).
     */
    readonly verified: number;
    /**
     * Wall time for the whole file, in milliseconds (float).
     */
    readonly wall_ms: number;
}

/**
 * The provenance pins as a JSON string, without needing a file: the build-time
 * pins only (tool name/version, lean4export commit, Lean toolchain, reader
 * format version). Lets the demo show the pins in the badge/footer before any
 * file is dropped.
 */
export function build_pins_json(): string;

/**
 * The built-in placeholder for the badge's "kernel: N KB wasm" figure. The demo
 * overrides this with the real gzipped size measured at copy time; this exists
 * so the number is never invented in the JS.
 */
export function kernel_kb_placeholder(): number;

/**
 * The `lean4export` git commit this checker's reader is pinned to.
 */
export function lean4export_commit(): string;

/**
 * The Lean toolchain version this checker's reader is pinned to.
 */
export function lean_toolchain(): string;

/**
 * The NDJSON format version this checker's reader targets.
 */
export function ndjson_format_version(): string;

/**
 * This tool's version string (`CARGO_PKG_VERSION` of `oxilean-verify`).
 */
export function verify_version(): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_declverdict_free: (a: number, b: number) => void;
    readonly __wbg_verifysession_free: (a: number, b: number) => void;
    readonly __wbg_verifysummary_free: (a: number, b: number) => void;
    readonly build_pins_json: () => [number, number];
    readonly declverdict_detail: (a: number) => [number, number];
    readonly declverdict_index: (a: number) => number;
    readonly declverdict_kind: (a: number) => [number, number];
    readonly declverdict_micros: (a: number) => number;
    readonly declverdict_ms: (a: number) => number;
    readonly declverdict_name: (a: number) => [number, number];
    readonly declverdict_verdict: (a: number) => [number, number];
    readonly kernel_kb_placeholder: () => number;
    readonly lean4export_commit: () => [number, number];
    readonly lean_toolchain: () => [number, number];
    readonly ndjson_format_version: () => [number, number];
    readonly verify_version: () => [number, number];
    readonly verifysession_byte_len: (a: number) => number;
    readonly verifysession_finish: (a: number, b: any, c: number, d: number) => [number, number, number];
    readonly verifysession_new: (a: number) => number;
    readonly verifysession_push_bytes: (a: number, b: number, c: number) => void;
    readonly verifysession_push_chunk: (a: number, b: number, c: number) => void;
    readonly verifysession_sha256: (a: number) => [number, number];
    readonly verifysummary_pins_json: (a: number) => [number, number];
    readonly verifysummary_rejected: (a: number) => number;
    readonly verifysummary_total: (a: number) => number;
    readonly verifysummary_unsupported: (a: number) => number;
    readonly verifysummary_verified: (a: number) => number;
    readonly verifysummary_wall_ms: (a: number) => number;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __externref_table_dealloc: (a: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
