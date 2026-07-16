/* @ts-self-types="./oxilean_verify.d.ts" */

/**
 * A single declaration verdict handed to JS.
 *
 * This is the shape the per-decl callback receives (as a plain object via
 * wasm-bindgen). The three buckets are kept distinct: `verdict` is exactly one
 * of `"verified"`, `"unsupported"`, or `"rejected"`, and `rejected` is the
 * alarm — never summed with `unsupported`.
 */
export class DeclVerdict {
    static __wrap(ptr) {
        const obj = Object.create(DeclVerdict.prototype);
        obj.__wbg_ptr = ptr;
        DeclVerdictFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        DeclVerdictFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_declverdict_free(ptr, 0);
    }
    /**
     * For `unsupported`, the named missing feature; for `rejected`, the reason;
     * empty for `verified`.
     * @returns {string}
     */
    get detail() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.declverdict_detail(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * The 0-based index of this declaration in file order.
     * @returns {number}
     */
    get index() {
        const ret = wasm.declverdict_index(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * The declaration kind label (`"axiom"`, `"def"`, `"thm"`, ...).
     * @returns {string}
     */
    get kind() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.declverdict_kind(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Microseconds spent checking this declaration (only meaningful for
     * `verified`; `0` otherwise). Returned as `f64` because JS numbers are
     * doubles — the value fits exactly for any realistic per-decl time.
     * @returns {number}
     */
    get micros() {
        const ret = wasm.declverdict_micros(this.__wbg_ptr);
        return ret;
    }
    /**
     * Milliseconds spent checking this declaration, as a float (convenience for
     * the demo's right-aligned `N.N ms` column).
     * @returns {number}
     */
    get ms() {
        const ret = wasm.declverdict_ms(this.__wbg_ptr);
        return ret;
    }
    /**
     * The declaration's fully-qualified name.
     * @returns {string}
     */
    get name() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.declverdict_name(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * The verdict bucket: `"verified"`, `"unsupported"`, or `"rejected"`.
     * @returns {string}
     */
    get verdict() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.declverdict_verdict(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
}
if (Symbol.dispose) DeclVerdict.prototype[Symbol.dispose] = DeclVerdict.prototype.free;

/**
 * Which resource-limit preset a [`VerifySession`] runs with.
 *
 * Presets map to [`oxilean_export::Limits`]. The default is the small,
 * untrusted-input budget — correct for a browser eating a dropped file. The
 * corpus preset raises the materialization budget for large, trusted exports.
 * @enum {0 | 1}
 */
export const LimitsPreset = Object.freeze({
    /**
     * The small untrusted-input budget ([`Limits::default`]): the right default
     * for a file a stranger dropped into the page.
     */
    Default: 0, "0": "Default",
    /**
     * The whole-corpus budget ([`Limits::corpus`]) for large trusted exports.
     */
    Corpus: 1, "1": "Corpus",
});

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
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        VerifySessionFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_verifysession_free(ptr, 0);
    }
    /**
     * The number of bytes pushed so far.
     * @returns {number}
     */
    get byte_len() {
        const ret = wasm.verifysession_byte_len(this.__wbg_ptr);
        return ret >>> 0;
    }
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
     * @param {Function} on_decl
     * @param {string} path
     * @returns {VerifySummary}
     */
    finish(on_decl, path) {
        const ptr0 = passStringToWasm0(path, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.verifysession_finish(this.__wbg_ptr, on_decl, ptr0, len0);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return VerifySummary.__wrap(ret[0]);
    }
    /**
     * Create a new session with the given resource-limit preset.
     * @param {LimitsPreset} preset
     */
    constructor(preset) {
        const ret = wasm.verifysession_new(preset);
        this.__wbg_ptr = ret;
        VerifySessionFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * Append one raw byte chunk (a `Uint8Array` slice of the file) to the
     * session. Preferred over [`push_chunk`](VerifySession::push_chunk) when the
     * page reads the file as bytes rather than decoded text, so the exact file
     * fingerprint is preserved.
     * @param {Uint8Array} chunk
     */
    push_bytes(chunk) {
        const ptr0 = passArray8ToWasm0(chunk, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.verifysession_push_bytes(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * Append one text chunk (a slice of the dropped file) to the session.
     *
     * The page reads the `File` in slices and calls this once per slice, in
     * order. Chunks are concatenated verbatim; verification runs only on
     * [`finish`](VerifySession::finish).
     * @param {string} chunk
     */
    push_chunk(chunk) {
        const ptr0 = passStringToWasm0(chunk, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.verifysession_push_chunk(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * The lowercase-hex SHA-256 fingerprint of everything pushed so far.
     *
     * Computed client-side (in the page's WebAssembly memory) so the demo can
     * show the exact file fingerprint under the "0 bytes uploaded" claim — the
     * bytes never leave the machine.
     * @returns {string}
     */
    get sha256() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.verifysession_sha256(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
}
if (Symbol.dispose) VerifySession.prototype[Symbol.dispose] = VerifySession.prototype.free;

/**
 * The final three-bucket summary returned by [`VerifySession::finish`].
 */
export class VerifySummary {
    static __wrap(ptr) {
        const obj = Object.create(VerifySummary.prototype);
        obj.__wbg_ptr = ptr;
        VerifySummaryFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        VerifySummaryFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_verifysummary_free(ptr, 0);
    }
    /**
     * The deterministic JSON report as a string: `tool`, `pins`, `input`,
     * `totals`, `unsupported_features`, `rejected`. Identical in shape to the
     * native CLI's `--json` output (timing masked to whole ms). The demo can
     * display or offer this for download — it is produced entirely in the page,
     * so nothing is uploaded.
     * @returns {string}
     */
    get pins_json() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.verifysummary_pins_json(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Declarations rejected — the alarm bucket. Never summed with
     * `unsupported`.
     * @returns {number}
     */
    get rejected() {
        const ret = wasm.verifysummary_rejected(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Total declarations seen.
     * @returns {number}
     */
    get total() {
        const ret = wasm.verifysummary_total(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Declarations deferred as unsupported (a named feature is missing).
     * @returns {number}
     */
    get unsupported() {
        const ret = wasm.verifysummary_unsupported(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Declarations verified (checked and a proof).
     * @returns {number}
     */
    get verified() {
        const ret = wasm.verifysummary_verified(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Wall time for the whole file, in milliseconds (float).
     * @returns {number}
     */
    get wall_ms() {
        const ret = wasm.verifysummary_wall_ms(this.__wbg_ptr);
        return ret;
    }
}
if (Symbol.dispose) VerifySummary.prototype[Symbol.dispose] = VerifySummary.prototype.free;

/**
 * The provenance pins as a JSON string, without needing a file: the build-time
 * pins only (tool name/version, lean4export commit, Lean toolchain, reader
 * format version). Lets the demo show the pins in the badge/footer before any
 * file is dropped.
 * @returns {string}
 */
export function build_pins_json() {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.build_pins_json();
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}

/**
 * The built-in placeholder for the badge's "kernel: N KB wasm" figure. The demo
 * overrides this with the real gzipped size measured at copy time; this exists
 * so the number is never invented in the JS.
 * @returns {number}
 */
export function kernel_kb_placeholder() {
    const ret = wasm.kernel_kb_placeholder();
    return ret >>> 0;
}

/**
 * The `lean4export` git commit this checker's reader is pinned to.
 * @returns {string}
 */
export function lean4export_commit() {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.lean4export_commit();
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}

/**
 * The Lean toolchain version this checker's reader is pinned to.
 * @returns {string}
 */
export function lean_toolchain() {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.lean_toolchain();
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}

/**
 * The NDJSON format version this checker's reader targets.
 * @returns {string}
 */
export function ndjson_format_version() {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.ndjson_format_version();
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}

/**
 * This tool's version string (`CARGO_PKG_VERSION` of `oxilean-verify`).
 * @returns {string}
 */
export function verify_version() {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.verify_version();
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}
function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg_Error_92b29b0548f8b746: function(arg0, arg1) {
            const ret = Error(getStringFromWasm0(arg0, arg1));
            return ret;
        },
        __wbg___wbindgen_throw_344f42d3211c4765: function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
        __wbg_call_a6e5c5dce5018821: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = arg0.call(arg1, arg2);
            return ret;
        }, arguments); },
        __wbg_declverdict_new: function(arg0) {
            const ret = DeclVerdict.__wrap(arg0);
            return ret;
        },
        __wbindgen_init_externref_table: function() {
            const table = wasm.__wbindgen_externrefs;
            const offset = table.grow(4);
            table.set(0, undefined);
            table.set(offset + 0, undefined);
            table.set(offset + 1, null);
            table.set(offset + 2, true);
            table.set(offset + 3, false);
        },
    };
    return {
        __proto__: null,
        "./oxilean_verify_bg.js": import0,
    };
}

const DeclVerdictFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_declverdict_free(ptr, 1));
const VerifySessionFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_verifysession_free(ptr, 1));
const VerifySummaryFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_verifysummary_free(ptr, 1));

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_externrefs.set(idx, obj);
    return idx;
}

function getStringFromWasm0(ptr, len) {
    return decodeText(ptr >>> 0, len);
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        const idx = addToExternrefTable0(e);
        wasm.__wbindgen_exn_store(idx);
    }
}

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1, 1) >>> 0;
    getUint8ArrayMemory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function passStringToWasm0(arg, malloc, realloc) {
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }
    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function takeFromExternrefTable0(idx) {
    const value = wasm.__wbindgen_externrefs.get(idx);
    wasm.__externref_table_dealloc(idx);
    return value;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    };
}

let WASM_VECTOR_LEN = 0;

let wasmModule, wasmInstance, wasm;
function __wbg_finalize_init(instance, module) {
    wasmInstance = instance;
    wasm = instance.exports;
    wasmModule = module;
    cachedUint8ArrayMemory0 = null;
    wasm.__wbindgen_start();
    return wasm;
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);
            } catch (e) {
                const validResponse = module.ok && expectedResponseType(module.type);

                if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else { throw e; }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);
    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };
        } else {
            return instance;
        }
    }

    function expectedResponseType(type) {
        switch (type) {
            case 'basic': case 'cors': case 'default': return true;
        }
        return false;
    }
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (module !== undefined) {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();
    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }
    const instance = new WebAssembly.Instance(module, imports);
    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (module_or_path !== undefined) {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (module_or_path === undefined) {
        module_or_path = new URL('oxilean_verify_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync, __wbg_init as default };
