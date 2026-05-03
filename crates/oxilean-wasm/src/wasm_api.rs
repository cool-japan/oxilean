//! WASM API bindings for JavaScript/TypeScript
use serde::Serialize;
use wasm_bindgen::prelude::*;

use crate::api::OxiLean;
use crate::incremental::{
    incremental_check, DiagnosticInfo, IncrementalCache, IncrementalCheckResult,
};

/// Serialisable summary returned to JavaScript from `checkIncremental`.
#[derive(Debug, Serialize)]
struct IncrementalSummary {
    diagnostics: Vec<DiagnosticInfo>,
    recheck_count: usize,
    cache_hit_count: usize,
}

/// OxiLean WASM instance for JavaScript
#[wasm_bindgen]
pub struct WasmOxiLean {
    inner: OxiLean,
    /// Incremental type-check cache, persisted across calls to `checkIncremental`
    incremental_cache: Option<IncrementalCache>,
}

impl Default for WasmOxiLean {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl WasmOxiLean {
    /// Create a new OxiLean instance
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        WasmOxiLean {
            inner: OxiLean::new(),
            incremental_cache: None,
        }
    }

    /// Check OxiLean source code
    /// Returns a JSON-serialized CheckResult
    #[wasm_bindgen]
    pub fn check(&mut self, source: &str) -> Result<JsValue, JsError> {
        let result = self.inner.check(source);
        serde_wasm_bindgen::to_value(&result).map_err(|e| JsError::new(&e.to_string()))
    }

    /// Execute a REPL command
    /// Returns a JSON-serialized ReplResult
    #[wasm_bindgen]
    pub fn repl(&mut self, input: &str) -> Result<JsValue, JsError> {
        let result = self.inner.repl(input);
        serde_wasm_bindgen::to_value(&result).map_err(|e| JsError::new(&e.to_string()))
    }

    /// Get completions at a position
    /// Returns a JSON-serialized `Vec<CompletionItem>`
    #[wasm_bindgen]
    pub fn completions(&self, source: &str, line: u32, col: u32) -> Result<JsValue, JsError> {
        let result = self.inner.completions(source, line, col);
        serde_wasm_bindgen::to_value(&result).map_err(|e| JsError::new(&e.to_string()))
    }

    /// Get hover info at a position
    /// Returns null or a string
    #[wasm_bindgen(js_name = "hoverInfo")]
    pub fn hover_info(&self, source: &str, line: u32, col: u32) -> Option<String> {
        self.inner.hover_info(source, line, col)
    }

    /// Format OxiLean source code
    #[wasm_bindgen]
    pub fn format(&self, source: &str) -> Result<String, JsError> {
        self.inner
            .format(source)
            .map_err(|e| JsError::new(&e.to_string()))
    }

    /// Get session ID
    #[wasm_bindgen(js_name = "sessionId", getter)]
    pub fn session_id(&self) -> String {
        self.inner.session_id().to_string()
    }

    /// Get REPL history
    #[wasm_bindgen]
    pub fn history(&self) -> Result<JsValue, JsError> {
        serde_wasm_bindgen::to_value(self.inner.history()).map_err(|e| JsError::new(&e.to_string()))
    }

    /// Clear REPL history
    #[wasm_bindgen(js_name = "clearHistory")]
    pub fn clear_history(&mut self) {
        self.inner.clear_history();
    }

    /// Get OxiLean version
    #[wasm_bindgen]
    pub fn version() -> String {
        OxiLean::version().to_string()
    }

    /// Incrementally type-check `source`, reusing cached results for
    /// unchanged declarations.
    ///
    /// The internal cache is updated in-place so that successive calls for
    /// nearby edits are as efficient as possible.
    ///
    /// Returns a JS object with the shape:
    /// ```json
    /// {
    ///   "diagnostics": [{ "name": "...", "message": "...", "severity": 2 }],
    ///   "recheck_count": 1,
    ///   "cache_hit_count": 3
    /// }
    /// ```
    #[wasm_bindgen(js_name = "checkIncremental")]
    pub fn check_incremental(&mut self, source: &str) -> Result<JsValue, JsError> {
        let old_cache = self.incremental_cache.take();
        let result: IncrementalCheckResult = incremental_check(source, old_cache);

        // Persist the updated cache for the next call
        self.incremental_cache = Some(result.cache.clone());

        // Build a serialisable summary (omit the full cache from the JS return
        // value — it is an internal implementation detail)
        let summary = IncrementalSummary {
            diagnostics: result.diagnostics,
            recheck_count: result.recheck_count,
            cache_hit_count: result.cache_hit_count,
        };

        serde_wasm_bindgen::to_value(&summary).map_err(|e| JsError::new(&e.to_string()))
    }
}

/// Quick check source without creating an instance
#[wasm_bindgen(js_name = "checkSource")]
pub fn check_source(source: &str) -> Result<JsValue, JsError> {
    let result = crate::api::check_source(source);
    serde_wasm_bindgen::to_value(&result).map_err(|e| JsError::new(&e.to_string()))
}

/// Get OxiLean version
#[wasm_bindgen(js_name = "getVersion")]
pub fn get_version() -> String {
    crate::api::version().to_string()
}
