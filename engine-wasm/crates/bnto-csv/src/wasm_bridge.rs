// =============================================================================
// WASM Bridge — The Door Between JavaScript and Rust (CSV)
// =============================================================================
//
// WHAT IS THIS FILE?
// This is the translation layer between JavaScript (Web Worker) and our
// Rust CSV processing code. JavaScript can't call Rust functions directly —
// it needs `#[wasm_bindgen]` exported functions that convert between
// JS types (JsValue, Uint8Array, Function) and Rust types (Vec<u8>, &str, etc.).
//
// WHY IS THIS SEPARATE FROM clean.rs?
// Separation of concerns (Bento Box Principle):
//   - clean.rs: Pure Rust CSV cleaning logic. Testable natively.
//     Has no knowledge of JavaScript or WASM boundary concerns.
//   - wasm_bridge.rs: Handles the JS ↔ Rust type conversion. Only this
//     file imports wasm_bindgen and deals with JsValue.
//
// This means we can:
//   1. Test clean.rs with normal `cargo test` (fast, no browser needed)
//   2. Test wasm_bridge.rs with `wasm-pack test` (needs Node.js or browser)
//   3. Reuse clean.rs in non-WASM contexts (CLI, desktop) without changes
//
// HOW THE WEB WORKER CALLS US:
//
//   JavaScript (Web Worker):
//   ```js
//   import init, { setup, clean_csv } from './pkg/bnto_wasm.js';
//   await init();
//   setup();
//
//   // Read a File object as bytes
//   const fileBytes = new Uint8Array(await file.arrayBuffer());
//
//   // Call our Rust CSV cleaning function
//   const result = clean_csv(
//       fileBytes,                          // raw CSV bytes
//       "data.csv",                         // filename
//       '{"removeDuplicates": true}',       // JSON config
//       (percent, message) => {
//           postMessage({ type: 'progress', percent, message });
//       }
//   );
//
//   // result is a JSON string with metadata
//   const metadata = JSON.parse(result);
//   ```

use wasm_bindgen::prelude::*;

use bnto_core::processor::{NodeInput, NodeProcessor};
use bnto_core::progress::ProgressReporter;

use crate::clean::CleanCsv;
use crate::rename_columns::RenameCsvColumns;

// =============================================================================
// Clean CSV — The Main Entry Point for JavaScript
// =============================================================================
//
// NOTE: setup() and version() are provided by the bnto-wasm entry point crate.
// Each node crate only exports its specific processing functions here.
// The entry point handles one-time initialization (panic hook, versioning).

/// Clean a CSV file and return a JSON string with metadata.
///
/// This is the metadata-returning variant. Use `clean_csv_bytes()`
/// to get the actual cleaned file data as a Uint8Array.
///
/// ARGUMENTS (from JavaScript):
///   - `data` (Uint8Array): The raw CSV file bytes
///   - `filename` (string): The original filename (e.g., "data.csv")
///   - `params_json` (string): JSON string with cleaning config
///     (e.g., '{"removeDuplicates": true}'). Pass '{}' for defaults.
///   - `progress_callback` (Function): Called with (percent: number, message: string)
///     to report progress. The Web Worker forwards this to the main thread.
///
/// RETURNS:
///   A JSON string describing the result:
///   ```json
///   {
///     "file": {
///       "filename": "data-cleaned.csv",
///       "mimeType": "text/csv",
///       "size": 1024
///     },
///     "metadata": {
///       "originalRows": 100,
///       "cleanedRows": 85,
///       "rowsRemoved": 15,
///       "duplicatesRemoved": 5
///     }
///   }
///   ```
///
/// WHY RETURN A JSON STRING?
/// We serialize to a JSON string instead of building a JS object with
/// `js_sys::Object` or `serde-wasm-bindgen`. The reason: `serde-wasm-bindgen`
/// pulls in the ENTIRE `js_sys` binding surface (thousands of symbols),
/// which bloats the WASM binary and causes linker issues in test builds.
/// A JSON string is simple, debuggable, and tiny in code size.
///
/// RUST CONCEPT: `Result<String, JsValue>`
/// wasm-bindgen functions that can fail return `Result<T, JsValue>`.
/// `Ok(value)` becomes a normal return in JS. `Err(value)` throws a
/// JavaScript Error.
#[wasm_bindgen]
pub fn clean_csv(
    data: &[u8],
    filename: &str,
    params_json: &str,
    progress_callback: js_sys::Function,
) -> Result<String, JsValue> {
    // --- Step 1: Parse the config parameters from JSON ---
    //
    // The Web Worker sends config as a JSON string. We parse it into
    // a serde_json::Map (a Rust HashMap that mirrors a JSON object).
    //
    // RUST CONCEPT: `.unwrap_or_default()`
    // If JSON parsing fails (invalid JSON), use an empty map instead
    // of crashing. This makes the function resilient to bad input —
    // it'll just use default settings.
    let params: serde_json::Map<String, serde_json::Value> =
        serde_json::from_str(params_json).unwrap_or_default();

    // --- Step 2: Build the NodeInput ---
    //
    // Convert from the JS-friendly arguments to our internal Rust types.
    //
    // `.to_vec()` copies the bytes from the WASM linear memory into a
    // new Vec<u8>. This is necessary because the input `&[u8]` is a
    // borrowed reference to the JS Uint8Array's backing buffer — we
    // can't hold onto it while processing (the JS side might move it).
    let input = NodeInput {
        data: data.to_vec(),
        filename: filename.to_string(),
        mime_type: None,
        params,
    };

    // --- Step 3: Create the processor and progress reporter ---
    let processor = CleanCsv::new();
    let progress = ProgressReporter::new(progress_callback);

    // --- Step 4: Run the CSV cleaning ---
    //
    // `process()` returns `Result<NodeOutput, BntoError>`.
    // We need to convert the error to a JsValue for the WASM boundary.
    //
    // `.map_err(|e| ...)` transforms the error type if it's Err.
    // `BntoError::into()` uses our `From<BntoError> for JsValue` impl
    // (defined in bnto-core/errors.rs) to create a JS Error object.
    let output = processor
        .process(input, &progress)
        .map_err(|e| -> JsValue { e.into() })?;

    // --- Step 5: Build a JSON result string ---
    //
    // We build the result as a serde_json::Value and serialize to string.
    // The Web Worker will call JSON.parse() on it.
    let file_info = if let Some(file) = output.files.first() {
        serde_json::json!({
            "filename": file.filename,
            "mimeType": file.mime_type,
            "size": file.data.len(),
        })
    } else {
        serde_json::json!({})
    };

    let result = serde_json::json!({
        "file": file_info,
        "metadata": output.metadata,
    });

    // Serialize to a JSON string. This should never fail because all
    // our values are valid JSON types (strings, numbers, objects).
    serde_json::to_string(&result)
        .map_err(|e| JsValue::from_str(&format!("Failed to serialize result: {e}")))
}

/// Clean a CSV file and return JUST the cleaned bytes.
///
/// This is the efficient path for the Web Worker: instead of encoding
/// file bytes as JSON (which would double the memory usage), this
/// returns the raw bytes as a `Vec<u8>` which wasm-bindgen converts
/// to a `Uint8Array` on the JS side. Zero-copy via shared memory.
///
/// ARGUMENTS: Same as `clean_csv`, minus the return format.
///
/// RETURNS: The raw cleaned CSV bytes as a Uint8Array.
///
/// The Web Worker uses this to create a Blob for download:
/// ```js
/// const cleaned = clean_csv_bytes(data, filename, paramsJson, progressCb);
/// const blob = new Blob([cleaned], { type: 'text/csv' });
/// ```
#[wasm_bindgen]
pub fn clean_csv_bytes(
    data: &[u8],
    filename: &str,
    params_json: &str,
    progress_callback: js_sys::Function,
) -> Result<Vec<u8>, JsValue> {
    let params: serde_json::Map<String, serde_json::Value> =
        serde_json::from_str(params_json).unwrap_or_default();

    let input = NodeInput {
        data: data.to_vec(),
        filename: filename.to_string(),
        mime_type: None,
        params,
    };

    let processor = CleanCsv::new();
    let progress = ProgressReporter::new(progress_callback);

    let output = processor
        .process(input, &progress)
        .map_err(|e| -> JsValue { e.into() })?;

    // Return just the first file's bytes.
    // Clean-csv always produces exactly one output file.
    if let Some(file) = output.files.into_iter().next() {
        Ok(file.data)
    } else {
        Err(JsValue::from_str("No output file produced"))
    }
}

// =============================================================================
// Rename CSV Columns — Remap column headers via a JSON mapping
// =============================================================================

/// Rename columns in a CSV file and return a JSON string with metadata.
///
/// This is the metadata-returning variant. Use `rename_csv_columns_bytes()`
/// to get the actual modified file data as a Uint8Array.
///
/// ARGUMENTS (from JavaScript):
///   - `data` (Uint8Array): The raw CSV file bytes
///   - `filename` (string): The original filename (e.g., "data.csv")
///   - `params_json` (string): JSON string with rename config
///     (e.g., '{"columns": {"First Name": "first_name"}}').
///     Pass '{}' for no renames (passthrough).
///   - `progress_callback` (Function): Called with (percent: number, message: string)
///     to report progress. The Web Worker forwards this to the main thread.
///
/// RETURNS:
///   A JSON string describing the result:
///   ```json
///   {
///     "file": {
///       "filename": "data-renamed.csv",
///       "mimeType": "text/csv",
///       "size": 1024
///     },
///     "metadata": {
///       "columnsRenamed": 2,
///       "totalColumns": 5,
///       "dataRows": 100,
///       "mapping": {"First Name": "first_name", "Last Name": "last_name"}
///     }
///   }
///   ```
#[wasm_bindgen]
pub fn rename_csv_columns(
    data: &[u8],
    filename: &str,
    params_json: &str,
    progress_callback: js_sys::Function,
) -> Result<String, JsValue> {
    // --- Step 1: Parse the config parameters from JSON ---
    let params: serde_json::Map<String, serde_json::Value> =
        serde_json::from_str(params_json).unwrap_or_default();

    // --- Step 2: Build the NodeInput ---
    let input = NodeInput {
        data: data.to_vec(),
        filename: filename.to_string(),
        mime_type: Some("text/csv".to_string()),
        params,
    };

    // --- Step 3: Create the processor and progress reporter ---
    let processor = RenameCsvColumns::new();
    let progress = ProgressReporter::new(progress_callback);

    // --- Step 4: Run the rename operation ---
    let output = processor
        .process(input, &progress)
        .map_err(|e| -> JsValue { e.into() })?;

    // --- Step 5: Build a JSON result string ---
    let file_info = if let Some(file) = output.files.first() {
        serde_json::json!({
            "filename": file.filename,
            "mimeType": file.mime_type,
            "size": file.data.len(),
        })
    } else {
        serde_json::json!({})
    };

    let result = serde_json::json!({
        "file": file_info,
        "metadata": output.metadata,
    });

    serde_json::to_string(&result)
        .map_err(|e| JsValue::from_str(&format!("Failed to serialize result: {e}")))
}

/// Rename columns in a CSV file and return JUST the modified bytes.
///
/// This is the efficient path for the Web Worker: returns raw bytes as
/// `Vec<u8>` which wasm-bindgen converts to a `Uint8Array` on the JS side.
///
/// ARGUMENTS: Same as `rename_csv_columns`.
///
/// RETURNS: The raw modified CSV bytes as a Uint8Array.
#[wasm_bindgen]
pub fn rename_csv_columns_bytes(
    data: &[u8],
    filename: &str,
    params_json: &str,
    progress_callback: js_sys::Function,
) -> Result<Vec<u8>, JsValue> {
    let params: serde_json::Map<String, serde_json::Value> =
        serde_json::from_str(params_json).unwrap_or_default();

    let input = NodeInput {
        data: data.to_vec(),
        filename: filename.to_string(),
        mime_type: Some("text/csv".to_string()),
        params,
    };

    let processor = RenameCsvColumns::new();
    let progress = ProgressReporter::new(progress_callback);

    let output = processor
        .process(input, &progress)
        .map_err(|e| -> JsValue { e.into() })?;

    // Return just the first file's bytes.
    // Rename-csv-columns always produces exactly one output file.
    if let Some(file) = output.files.into_iter().next() {
        Ok(file.data)
    } else {
        Err(JsValue::from_str("No output file produced"))
    }
}
