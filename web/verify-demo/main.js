// Kernel in a Tab — dependency-free driver (plain module script, no framework,
// no CDN). Works under `python3 -m http.server`: no SharedArrayBuffer, no
// COOP/COEP, no cross-origin isolation required.
//
// Flow (brief §6):
//   1. init() the wasm module (wasm-pack --target web loader).
//   2. On drop, read the File in slices and push each into a VerifySession.
//   3. finish() runs the real oxilean-verify engine over the bytes, collecting
//      one verdict per declaration. We then render those verdicts in
//      requestAnimationFrame batches so the UI stays live even for big files.
//   4. Summary + the closing line.

import init, {
  VerifySession,
  LimitsPreset,
  verify_version,
  lean4export_commit,
  lean_toolchain,
  ndjson_format_version,
} from "./pkg/oxilean_verify.js";

const CHUNK_BYTES = 1 << 20; // 1 MiB file-read slices
const RENDER_BATCH = 200; // verdict lines painted per animation frame

const el = (id) => document.getElementById(id);
const dropZone = el("drop");
const streamEl = el("stream");
const runEl = el("run");
const summaryEl = el("summary");
const closingEl = el("closing");
const pinsEl = el("pins");
const fileInput = el("file");

let wasmReady = false;

async function ensureWasm() {
  if (wasmReady) return;
  await init(); // fetches ./pkg/oxilean_verify_bg.wasm
  wasmReady = true;
  pinsEl.textContent =
    `oxilean-verify ${verify_version()}  ·  ` +
    `lean4export ${lean4export_commit().slice(0, 10)}  ·  ` +
    `Lean ${lean_toolchain()}  ·  NDJSON ${ndjson_format_version()}`;
}

// Read the dropped File in slices, pushing each slice as raw bytes into the
// session so the exact file fingerprint is preserved ("0 bytes uploaded").
async function loadFileIntoSession(file, session) {
  let offset = 0;
  while (offset < file.size) {
    const slice = file.slice(offset, Math.min(offset + CHUNK_BYTES, file.size));
    const buf = new Uint8Array(await slice.arrayBuffer());
    session.push_bytes(buf);
    offset += CHUNK_BYTES;
    // Yield to the event loop between slices so a large file does not freeze
    // the tab during the read phase.
    await new Promise((r) => setTimeout(r, 0));
  }
}

function markFor(verdict) {
  switch (verdict) {
    case "verified":
      return "✓"; // ✓
    case "unsupported":
      return "⊘"; // ⊘
    case "rejected":
      return "✗"; // ✗
    default:
      return "?";
  }
}

function auxFor(decl) {
  switch (decl.verdict) {
    case "verified":
      return `${decl.ms.toFixed(1)} ms`;
    case "unsupported":
      return `unsupported: ${decl.detail}`;
    case "rejected":
      return `rejected: ${decl.detail}`;
    default:
      return "";
  }
}

function renderLine(decl) {
  const line = document.createElement("div");
  line.className = `line ${decl.verdict}`;

  const mark = document.createElement("span");
  mark.className = "mark";
  mark.textContent = markFor(decl.verdict);

  const name = document.createElement("span");
  name.className = "name";
  name.textContent = decl.name;

  const aux = document.createElement("span");
  aux.className = "aux";
  aux.textContent = auxFor(decl);

  line.append(mark, name, aux);
  return line;
}

// Paint the collected verdicts incrementally so the UI stays live.
function paintVerdicts(verdicts) {
  return new Promise((resolve) => {
    let i = 0;
    function frame() {
      const end = Math.min(i + RENDER_BATCH, verdicts.length);
      const frag = document.createDocumentFragment();
      for (; i < end; i++) frag.appendChild(renderLine(verdicts[i]));
      streamEl.appendChild(frag);
      streamEl.scrollTop = streamEl.scrollHeight;
      if (i < verdicts.length) {
        requestAnimationFrame(frame);
      } else {
        resolve();
      }
    }
    requestAnimationFrame(frame);
  });
}

function showSummary(summary) {
  summaryEl.innerHTML =
    `<span class="n-verified">${summary.verified} verified</span>` +
    ` · ` +
    `<span class="n-unsupported">${summary.unsupported} unsupported</span>` +
    ` · ` +
    `<span class="n-rejected">${summary.rejected} rejected</span>`;
  // The alarm: any non-zero rejected count must visibly stand out (brief §7).
  summaryEl.classList.toggle("has-rejected", summary.rejected > 0);
}

async function verifyFile(file) {
  runEl.hidden = false;
  streamEl.textContent = "";
  summaryEl.textContent = "checking…";
  closingEl.hidden = true;

  try {
    await ensureWasm();
  } catch (e) {
    summaryEl.innerHTML = `<span class="error">failed to load the kernel: ${e}</span>`;
    return;
  }

  const session = new VerifySession(LimitsPreset.Default);
  await loadFileIntoSession(file, session);

  // Collect verdicts during the synchronous engine run, then paint in batches.
  const verdicts = [];
  const onDecl = (decl) => {
    verdicts.push({
      name: decl.name,
      verdict: decl.verdict,
      detail: decl.detail,
      ms: decl.ms,
    });
  };

  let summary;
  try {
    summary = session.finish(onDecl, file.name);
  } catch (e) {
    // A VerifyError: "this file is broken", NOT a rejected proof (brief §7).
    // Never show it in the rejected bucket.
    summaryEl.innerHTML =
      `<span class="error">this file is broken (not a rejected proof): ${e}</span>`;
    return;
  }

  await paintVerdicts(verdicts);
  showSummary(summary);
  closingEl.hidden = false;
}

// ── wiring: drop zone, file picker, sample ──────────────────────────────────

function onFile(file) {
  if (file) verifyFile(file);
}

["dragenter", "dragover"].forEach((ev) =>
  dropZone.addEventListener(ev, (e) => {
    e.preventDefault();
    dropZone.classList.add("dragover");
  }),
);
["dragleave", "dragend", "drop"].forEach((ev) =>
  dropZone.addEventListener(ev, () => dropZone.classList.remove("dragover")),
);
dropZone.addEventListener("drop", (e) => {
  e.preventDefault();
  const file = e.dataTransfer?.files?.[0];
  onFile(file);
});
dropZone.addEventListener("click", (e) => {
  if (e.target.id === "pick" || e.target.classList.contains("drop-inner")) {
    // handled by explicit buttons below
  }
});

el("pick").addEventListener("click", (e) => {
  e.stopPropagation();
  fileInput.click();
});
fileInput.addEventListener("change", () => onFile(fileInput.files?.[0]));

el("sample").addEventListener("click", async (e) => {
  e.stopPropagation();
  const resp = await fetch("./sample/simple_add.ndjson");
  const blob = await resp.blob();
  const file = new File([blob], "simple_add.ndjson");
  onFile(file);
});

// Warm the wasm early so the first drop is instant.
ensureWasm().catch(() => {
  /* surfaced on first use */
});
