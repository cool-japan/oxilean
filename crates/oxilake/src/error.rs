// Error handling for oxilake.
//
// We use `anyhow::Error` as the primary error type throughout the crate,
// surfacing context-rich diagnostics via the `?` operator and `.context()`.
