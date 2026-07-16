// ============================================================
// OxiLean Browser Playground
// CodeMirror 6 from pinned ESM CDN, OxiLean WASM for live checking
//
// WASM API contract (from wasm_api.rs + pkg-web/oxilean_wasm.js):
//   Class:  WasmOxiLean  (new WasmOxiLean())
//   check(source: string) -> { success, declarations, errors, warnings }
//     declarations: Array<{ name, kind, ty }>
//     errors:       Array<{ message, line?, column?, source? }>
//     warnings:     Array<{ message, line?, column? }>
//   Static: WasmOxiLean.version() -> string
//   Free:   checkSource(source) -> same shape (no instance needed)
//   Free:   getVersion() -> string
//   Free:   compressShare(source: string) -> string  (base64url of deflated bytes)
//   Free:   decompressShare(encoded: string) -> string
//   Default export: init(wasmUrl?) -> Promise (must await before use)
//
// All DOM updates use safe DOM API methods (createElement / textContent)
// rather than innerHTML, to eliminate any XSS surface.
// ============================================================

// Manual test checklist for multi-file tabs:
// 1. Open playground — default "Main.lean" tab shown
// 2. Click "+" — new "New.lean" tab opens, content switches
// 3. Edit content in New.lean — should auto-save (debounce)
// 4. Double-click tab — rename input appears; press Enter to confirm
// 5. Click "×" on non-active tab — tab closes; content preserved in others
// 6. Click "×" on last remaining tab — should NOT close (protected)
// 7. Reload page — all tabs and their content restored from IndexedDB
// 8. URL fragment share — if URL has fragment, loads into Main.lean; other tabs empty initially

import { EditorView, basicSetup } from 'https://esm.sh/codemirror@6.0.1';
import { EditorState } from 'https://esm.sh/@codemirror/state@6.4.1';

// Singleton checker instance — created after WASM init
let checker = null;

// WASM module reference for compressShare / decompressShare free functions
let wasmMod = null;

// Editor view reference — set once the editor is created
let editorView = null;

// ─── Multi-file state ─────────────────────────────────────────────────────────
// Map<string, string>: filename → content
const fileStore = new Map();
let activeFile = 'Main.lean';

/** Initialise the file store with a single "Main.lean" entry. */
function initFileStore(defaultContent) {
  fileStore.set('Main.lean', defaultContent);
  activeFile = 'Main.lean';
}

// ── DOM helpers (no innerHTML) ───────────────────────────────────────────────

/** Create an element with optional class and text content. */
function el(tag, cls, text) {
  const node = document.createElement(tag);
  if (cls) node.className = cls;
  if (text !== undefined) node.textContent = text;
  return node;
}

/** Replace all children of `parent` with `...nodes`. */
function replaceChildren(parent, ...nodes) {
  parent.replaceChildren(...nodes);
}

/** Return a human-readable location label from a WASM diagnostic item. */
function locLabel(item) {
  if (item.line != null && item.column != null) {
    return `line ${item.line + 1}, col ${item.column + 1}`;
  }
  if (item.line != null) {
    return `line ${item.line + 1}`;
  }
  if (typeof item.source === 'string' && item.source.length > 0) {
    return item.source;
  }
  return '';
}

// ── UI state helpers ─────────────────────────────────────────────────────────

function setDot(state) {
  // state: '' | 'checking' | 'ok' | 'error'
  document.getElementById('status-dot').className = state;
}

function showLoading(message) {
  const container = document.getElementById('result-content');
  replaceChildren(container, el('div', 'loading', message));
}

function showError(message) {
  const container = document.getElementById('result-content');
  const wrapper = el('div', 'result-error');
  wrapper.appendChild(el('div', 'msg', message));
  replaceChildren(container, wrapper);
}

// ── Result rendering (safe DOM construction) ─────────────────────────────────

function makeDiagNode(cssClass, locText, msgText) {
  const wrapper = el('div', cssClass);
  if (locText) {
    wrapper.appendChild(el('div', 'loc', locText));
  }
  wrapper.appendChild(el('div', 'msg', msgText));
  return wrapper;
}

function renderResult(result) {
  // result: { success, declarations, errors, warnings }
  const container = document.getElementById('result-content');
  const nodes = [];

  // Errors
  if (result.errors && result.errors.length > 0) {
    for (const e of result.errors) {
      nodes.push(makeDiagNode('result-error', locLabel(e), String(e.message)));
    }
  }

  // Warnings
  if (result.warnings && result.warnings.length > 0) {
    for (const w of result.warnings) {
      nodes.push(makeDiagNode('result-warning', locLabel(w), String(w.message)));
    }
  }

  // Declarations (informational — shown even when success=true)
  if (result.declarations && result.declarations.length > 0) {
    for (const d of result.declarations) {
      const wrapper = el('div', 'result-decl');
      const header = el('div');
      header.appendChild(el('span', 'decl-name', String(d.name)));
      header.appendChild(el('span', 'decl-kind', ' ' + String(d.kind)));
      wrapper.appendChild(header);
      if (d.ty) {
        wrapper.appendChild(el('div', 'decl-type', ': ' + String(d.ty)));
      }
      nodes.push(wrapper);
    }
  }

  if (nodes.length === 0) {
    nodes.push(el('div', 'result-ok', '✓ No errors'));
  }

  replaceChildren(container, ...nodes);
}

// ── IndexedDB persistence ────────────────────────────────────────────────────
//
// DB schema v2:
//   Object store "state" — legacy single-file key "editor_content"
//   Object store "files" — per-file keys with prefix "file:<filename>"
//
// Backward compat: old sessions that only wrote to "state" / "editor_content"
// are detected at startup and migrated into the "files" store as "Main.lean".

const IDB_NAME = 'oxilean_playground';
const IDB_VERSION = 2;          // bumped from 1 to add "files" object store
const IDB_STORE = 'state';      // legacy single-file store
const IDB_KEY = 'editor_content';
const IDB_FILES_STORE = 'files'; // per-file store (v2)

/** Open (and if needed, create/upgrade) the IndexedDB database. */
function openDB() {
  return new Promise((resolve, reject) => {
    const req = indexedDB.open(IDB_NAME, IDB_VERSION);
    req.onupgradeneeded = (e) => {
      const db = e.target.result;
      // Create legacy store if missing (fresh install)
      if (!db.objectStoreNames.contains(IDB_STORE)) {
        db.createObjectStore(IDB_STORE);
      }
      // Create the new per-file store (v2)
      if (!db.objectStoreNames.contains(IDB_FILES_STORE)) {
        db.createObjectStore(IDB_FILES_STORE);
      }
    };
    req.onsuccess = (e) => resolve(e.target.result);
    req.onerror = (e) => reject(e.target.error);
  });
}

/**
 * Persist `content` for a single named file to IndexedDB.
 * Uses the "files" object store with key "file:<name>".
 * Failures are logged but do not propagate.
 */
function saveFileToDB(name, content) {
  openDB()
    .then((db) => {
      const tx = db.transaction(IDB_FILES_STORE, 'readwrite');
      tx.objectStore(IDB_FILES_STORE).put(content, 'file:' + name);
    })
    .catch((e) => console.warn('[OxiLean] IndexedDB saveFileToDB failed:', e));
}

/**
 * Remove a file entry from the "files" object store.
 */
function removeFileFromDB(name) {
  openDB()
    .then((db) => {
      const tx = db.transaction(IDB_FILES_STORE, 'readwrite');
      tx.objectStore(IDB_FILES_STORE).delete('file:' + name);
    })
    .catch((e) => console.warn('[OxiLean] IndexedDB removeFileFromDB failed:', e));
}

/**
 * Load all per-file entries from the "files" object store.
 * Returns a Map<string, string> of filename → content.
 * Resolves to an empty Map if the store is empty or on error.
 */
function loadAllFilesFromDB() {
  return openDB().then((db) =>
    new Promise((resolve) => {
      const tx = db.transaction(IDB_FILES_STORE, 'readonly');
      const store = tx.objectStore(IDB_FILES_STORE);
      const result = new Map();
      const req = store.openCursor();
      req.onsuccess = (e) => {
        const cursor = e.target.result;
        if (cursor) {
          if (typeof cursor.key === 'string' && cursor.key.startsWith('file:')) {
            result.set(cursor.key.slice(5), cursor.value);
          }
          cursor.continue();
        } else {
          resolve(result);
        }
      };
      req.onerror = () => resolve(new Map());
    }),
  );
}

/**
 * Persist `content` to the legacy "state" store (single-file).
 * Used to maintain backward compat for the active file.
 */
function saveContent(content) {
  openDB()
    .then((db) => {
      const tx = db.transaction(IDB_STORE, 'readwrite');
      tx.objectStore(IDB_STORE).put(content, IDB_KEY);
    })
    .catch((e) => console.warn('[OxiLean] IndexedDB save failed:', e));
}

/**
 * Load the previously persisted editor content from the legacy "state" store.
 * Resolves to the string value, or `null` if nothing was stored yet.
 */
function loadContent() {
  return openDB().then(
    (db) =>
      new Promise((resolve, reject) => {
        const tx = db.transaction(IDB_STORE, 'readonly');
        const req = tx.objectStore(IDB_STORE).get(IDB_KEY);
        req.onsuccess = (e) => resolve(e.target.result ?? null);
        req.onerror = (e) => reject(e.target.error);
      }),
  );
}

// ── Tab rendering and management ─────────────────────────────────────────────

/**
 * Re-render the entire tabs bar from the current fileStore and activeFile.
 * Called after any state mutation.
 */
function renderTabs() {
  const bar = document.getElementById('tabs-bar');
  if (!bar) return;
  bar.innerHTML = '';

  for (const [name] of fileStore) {
    const tab = document.createElement('button');
    tab.className = 'tab-btn' + (name === activeFile ? ' active' : '');
    tab.textContent = name;
    tab.title = 'Double-click to rename';
    tab.addEventListener('click', () => switchToFile(name));
    tab.addEventListener('dblclick', (e) => { e.stopPropagation(); renameFile(name, tab); });

    const closeSpan = document.createElement('span');
    closeSpan.className = 'tab-close';
    closeSpan.textContent = '×';
    closeSpan.title = 'Close file';
    closeSpan.addEventListener('click', (e) => { e.stopPropagation(); closeFile(name); });
    tab.appendChild(closeSpan);

    bar.appendChild(tab);
  }

  // "+" button to create a new file
  const addBtn = document.createElement('button');
  addBtn.className = 'tab-add';
  addBtn.textContent = '+';
  addBtn.title = 'New file';
  addBtn.addEventListener('click', addFile);
  bar.appendChild(addBtn);
}

/**
 * Save the active file's current editor content to fileStore and IndexedDB,
 * then load the target file into the editor and update UI.
 */
function switchToFile(name) {
  if (!editorView) return;

  // Snapshot current editor content into the store before switching
  const currentContent = editorView.state.doc.toString();
  fileStore.set(activeFile, currentContent);
  saveFileToDB(activeFile, currentContent);

  // Switch active file and load its content into the editor
  activeFile = name;
  const content = fileStore.get(name) ?? '';
  editorView.dispatch({
    changes: {
      from: 0,
      to: editorView.state.doc.length,
      insert: content,
    },
  });

  renderTabs();
  triggerCheck();
}

/**
 * Create a new file with a unique name, add it to the store, and switch to it.
 */
function addFile() {
  let name = 'New.lean';
  let n = 1;
  while (fileStore.has(name)) {
    name = `New${n}.lean`;
    n += 1;
  }
  const content = '-- New file\n';
  fileStore.set(name, content);
  saveFileToDB(name, content);
  switchToFile(name);
}

/**
 * Close a named file tab. Refuses to close the last remaining tab.
 * If the closed tab was active, switches to the first remaining file.
 */
function closeFile(name) {
  if (fileStore.size <= 1) {
    // Protect: never close the last tab
    return;
  }
  fileStore.delete(name);
  removeFileFromDB(name);

  if (activeFile === name) {
    // Switch to the first remaining file
    const nextName = fileStore.keys().next().value;
    activeFile = nextName;
    if (editorView) {
      const content = fileStore.get(nextName) ?? '';
      editorView.dispatch({
        changes: {
          from: 0,
          to: editorView.state.doc.length,
          insert: content,
        },
      });
    }
    triggerCheck();
  }

  renderTabs();
}

/**
 * Replace a tab button with an inline input for in-place renaming.
 * Commits on Enter or blur; cancels on Escape.
 */
function renameFile(oldName, tabEl) {
  const input = document.createElement('input');
  input.type = 'text';
  input.value = oldName;
  input.className = 'tab-rename-input';
  tabEl.replaceWith(input);
  input.focus();
  input.select();

  let committed = false;

  const commit = () => {
    if (committed) return;
    committed = true;
    const newName = input.value.trim() || oldName;
    if (newName !== oldName && !fileStore.has(newName)) {
      const content = fileStore.get(oldName);
      fileStore.delete(oldName);
      removeFileFromDB(oldName);
      fileStore.set(newName, content);
      saveFileToDB(newName, content);
      if (activeFile === oldName) {
        activeFile = newName;
      }
    }
    renderTabs();
  };

  input.addEventListener('blur', commit);
  input.addEventListener('keydown', (e) => {
    if (e.key === 'Enter') {
      input.blur(); // triggers blur → commit
    } else if (e.key === 'Escape') {
      committed = true; // skip commit
      renderTabs();
    }
  });
}

/**
 * Run the WASM check on the current active file's content.
 * Safe to call before the checker is ready (no-op in that case).
 */
function triggerCheck() {
  if (!editorView) return;
  runCheck(editorView.state.doc.toString());
}

// ── Share-via-URL ────────────────────────────────────────────────────────────

/**
 * Set the editor content by dispatching a CodeMirror 6 transaction.
 */
function setEditorContent(content) {
  if (!editorView) return;
  editorView.dispatch({
    changes: {
      from: 0,
      to: editorView.state.doc.length,
      insert: content,
    },
  });
}

/**
 * Read the current fragment (#…) from the URL, decompress it via the WASM
 * export, and load it into the editor.  Returns `true` if a shared snippet
 * was loaded.
 */
function tryLoadFromFragment() {
  const hash = window.location.hash.slice(1); // strip leading '#'
  if (!hash || !wasmMod) return false;
  try {
    const code = wasmMod.decompressShare(hash);
    setEditorContent(code);
    return true;
  } catch (e) {
    console.warn('[OxiLean] Could not decompress URL fragment:', e);
    return false;
  }
}

/**
 * Wire up the "Share" button: compress the current editor content, write the
 * base64url fragment to the URL bar, and copy the full URL to the clipboard.
 */
function initShareButton() {
  const btn = document.getElementById('share-btn');
  if (!btn) return;
  btn.addEventListener('click', () => {
    if (!editorView || !wasmMod) {
      btn.textContent = 'Not ready';
      setTimeout(() => { btn.textContent = 'Share'; }, 1500);
      return;
    }
    const code = editorView.state.doc.toString();
    let encoded;
    try {
      encoded = wasmMod.compressShare(code);
    } catch (e) {
      console.error('[OxiLean] compress_share failed:', e);
      btn.textContent = 'Error!';
      setTimeout(() => { btn.textContent = 'Share'; }, 1500);
      return;
    }
    // Update the URL fragment without triggering a page reload
    const newUrl = window.location.href.replace(/#.*$/, '') + '#' + encoded;
    history.replaceState(null, '', newUrl);
    navigator.clipboard.writeText(newUrl).then(
      () => {
        btn.textContent = 'Copied!';
        setTimeout(() => { btn.textContent = 'Share'; }, 2000);
      },
      () => {
        // Clipboard API unavailable (e.g., non-HTTPS) — just show the URL
        btn.textContent = 'URL updated';
        setTimeout(() => { btn.textContent = 'Share'; }, 2000);
      },
    );
  });
}

// ── Examples dropdown ────────────────────────────────────────────────────────

/**
 * Wire up the examples <select> so that choosing an entry replaces the editor
 * content and resets the select back to the placeholder.
 */
function initExamplesDropdown() {
  const sel = document.getElementById('examples-select');
  if (!sel) return;
  sel.addEventListener('change', () => {
    const code = sel.value;
    if (code && editorView) {
      setEditorContent(code);
      // Run the checker immediately after loading an example
      runCheck(code);
    }
    // Reset back to placeholder so the same example can be re-selected
    sel.selectedIndex = 0;
  });
}

// ── WASM initialisation ──────────────────────────────────────────────────────

async function initWasm() {
  try {
    // The web-target output exports:
    //   default = init(wasmUrl?)  — fetches and compiles the .wasm
    //   WasmOxiLean class, checkSource(), getVersion()
    //   compressShare(), decompressShare()
    // build.sh copies oxilean_wasm.js and oxilean_wasm_bg.wasm into dist/
    // alongside index.html and main.js, so the relative import works.
    const mod = await import('./oxilean_wasm.js');

    // Await the default init() — it fetches oxilean_wasm_bg.wasm relative to
    // oxilean_wasm.js using import.meta.url (works when served over HTTP).
    await mod.default();

    checker = new mod.WasmOxiLean();
    wasmMod = mod;

    // Reflect the runtime version in the header badge
    try {
      const ver = mod.WasmOxiLean.version();
      if (ver) {
        const badge = document.querySelector('header .version');
        if (badge) badge.textContent = 'v' + ver;
      }
    } catch (_) {
      // Non-critical — version badge is cosmetic
    }

    return true;
  } catch (err) {
    console.error('[OxiLean] WASM init failed:', err);
    return false;
  }
}

// ── Core check logic ─────────────────────────────────────────────────────────

function runCheck(source) {
  if (!checker) {
    return;
  }

  setDot('checking');

  // Defer to next tick so the 'checking' dot repaints before the (possibly
  // synchronous) WASM call blocks the main thread.
  setTimeout(() => {
    try {
      const result = checker.check(source);
      setDot(result.success ? 'ok' : 'error');
      renderResult(result);
    } catch (err) {
      setDot('error');
      showError(String(err));
    }
  }, 0);
}

// ── Debounce ─────────────────────────────────────────────────────────────────

function debounce(fn, ms) {
  let timer;
  return (...args) => {
    clearTimeout(timer);
    timer = setTimeout(() => fn(...args), ms);
  };
}

// ── Initial editor content ───────────────────────────────────────────────────

const INITIAL_SOURCE = `-- Welcome to the OxiLean Playground!
-- Edit the code below -- results update live (300 ms debounce).
-- Your edits are saved automatically in the browser (IndexedDB).
-- Use "Share" to generate a shareable URL.

def hello : String := "Hello, OxiLean!"

theorem nat_add_comm (a b : Nat) : a + b = b + a := by
  omega

theorem and_intro (p q : Prop) (hp : p) (hq : q) : p /\\ q :=
  ⟨hp, hq⟩
`;

// ── Main ─────────────────────────────────────────────────────────────────────

async function main() {
  showLoading('Loading OxiLean WASM…');

  const ok = await initWasm();
  if (!ok) {
    setDot('error');
    showError(
      'Failed to load OxiLean WASM. ' +
        'Check the browser console for details. ' +
        'Make sure you ran "bash build.sh" first and are serving ' +
        'via a local HTTP server (not file://).',
    );
    return;
  }

  const checkDebounced = debounce(runCheck, 300);

  // ── Determine initial content ──────────────────────────────────────────────
  // Priority: 1) URL fragment (shared link → goes into Main.lean),
  //           2) per-file IndexedDB (multi-file session),
  //           3) legacy single-file IndexedDB (migration),
  //           4) INITIAL_SOURCE (first visit).

  let initialContent = INITIAL_SOURCE;
  let hasFragment = false;

  // Check if a URL fragment is present — decode before editor creation.
  const rawFragment = window.location.hash.slice(1);
  if (rawFragment && wasmMod) {
    try {
      initialContent = wasmMod.decompressShare(rawFragment);
      hasFragment = true;
    } catch (e) {
      console.warn('[OxiLean] Could not load URL fragment:', e);
    }
  }

  // ── Create the editor ──────────────────────────────────────────────────────

  editorView = new EditorView({
    state: EditorState.create({
      doc: initialContent,
      extensions: [
        basicSetup,
        EditorView.updateListener.of((update) => {
          if (update.docChanged) {
            const source = update.state.doc.toString();
            // Update in-memory store for the active file
            fileStore.set(activeFile, source);
            checkDebounced(source);
            // Persist to per-file store and legacy store
            saveFileToDB(activeFile, source);
            saveContent(source);
          }
        }),
      ],
    }),
    parent: document.getElementById('editor'),
  });

  // ── Restore multi-file state from IndexedDB ────────────────────────────────

  if (!hasFragment) {
    // Try to load all per-file entries from the v2 "files" store
    const dbFiles = await loadAllFilesFromDB().catch(() => new Map());

    if (dbFiles.size > 0) {
      // Restore multi-file session: populate fileStore from DB
      dbFiles.forEach((content, name) => fileStore.set(name, content));
      // The first key becomes the active file
      activeFile = fileStore.keys().next().value;
      const restoredContent = fileStore.get(activeFile) ?? '';
      editorView.dispatch({
        changes: {
          from: 0,
          to: editorView.state.doc.length,
          insert: restoredContent,
        },
      });
      initialContent = restoredContent;
    } else {
      // No per-file entries: attempt legacy single-file migration
      let legacyContent = null;
      try {
        legacyContent = await loadContent();
      } catch (e) {
        console.warn('[OxiLean] IndexedDB legacy load failed:', e);
      }

      if (legacyContent) {
        // Migrate legacy content into Main.lean
        initialContent = legacyContent;
        editorView.dispatch({
          changes: {
            from: 0,
            to: editorView.state.doc.length,
            insert: legacyContent,
          },
        });
      }

      // Seed the file store with whatever content we have
      initFileStore(initialContent);
      // Persist seeded state to the v2 store
      saveFileToDB(activeFile, initialContent);
    }
  } else {
    // URL fragment loaded — put it in Main.lean, seed the store
    initFileStore(initialContent);
    saveFileToDB(activeFile, initialContent);
  }

  // ── Render the initial tab bar ─────────────────────────────────────────────
  renderTabs();

  // ── Wire up interactive controls ───────────────────────────────────────────

  initShareButton();
  initExamplesDropdown();

  // Run the initial check immediately (no debounce for startup)
  runCheck(initialContent);
}

main().catch((err) => {
  console.error('[OxiLean] Unexpected error in main():', err);
});
