//! WASM API bindings for JavaScript/TypeScript
use wasm_bindgen::prelude::*;

use crate::api::OxiLean;

/// OxiLean WASM instance for JavaScript
#[wasm_bindgen]
pub struct WasmOxiLean {
    inner: OxiLean,
}

#[wasm_bindgen]
impl WasmOxiLean {
    /// Create a new OxiLean instance
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        WasmOxiLean {
            inner: OxiLean::new(),
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
    /// Returns a JSON-serialized Vec<CompletionItem>
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
        serde_wasm_bindgen::to_value(self.inner.history())
            .map_err(|e| JsError::new(&e.to_string()))
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
