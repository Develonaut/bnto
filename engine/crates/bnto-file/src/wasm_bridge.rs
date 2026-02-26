// =============================================================================
// WASM Bridge — The Door Between JavaScript and Rust (File Operations)
// =============================================================================
//
// WHAT IS THIS FILE?
// This is the translation layer between JavaScript (Web Worker) and our
// Rust file operation code. JavaScript can't call Rust functions directly —
// it needs `#[wasm_bindgen]` exported functions that convert between
// JS types (JsValue, Uint8Array, Function) and Rust types (Vec<u8>, &str, etc.).
//
// WHY IS THIS SEPARATE FROM rename.rs?
// Separation of concerns (Bento Box Principle):
//   - rename.rs: Pure Rust filename transformation logic. Testable natively.
//     Has no knowledge of JavaScript or WASM boundary concerns.
//   - wasm_bridge.rs: Handles the JS <-> Rust type conversion. Only this
//     file imports wasm_bindgen and deals with JsValue.
//
// This means we can:
//   1. Test rename.rs with normal `cargo test` (fast, no browser needed)
//   2. Test wasm_bridge.rs with `wasm-pack test` (needs Node.js or browser)
//   3. Reuse rename.rs in non-WASM contexts (CLI, desktop) without changes
//
// HOW THE WEB WORKER CALLS US:
//
//   JavaScript (Web Worker):
//   ```js
//   import init, { setup, rename_file } from './pkg/bnto_wasm.js';
//   await init();
//   setup();
//
//   // Read a File object as bytes
//   const fileBytes = new Uint8Array(await file.arrayBuffer());
//
//   // Call our Rust rename function
//   const result = rename_file(
//       fileBytes,            // raw file bytes (pass through unchanged)
//       "IMG_1234.jpg",       // original filename
//       '{"prefix": "vacation-", "find": "IMG_", "replace": ""}',
//       (percent, message) => {
//           postMessage({ type: 'progress', percent, message });
//       }
//   );
//
//   // result is a JSON string with metadata
//   const metadata = JSON.parse(result);
//   // metadata.file.filename => "vacation-1234.jpg"
//   ```

use wasm_bindgen::prelude::*;

use bnto_core::errors::BntoError;
use bnto_core::processor::{NodeInput, NodeProcessor};
use bnto_core::progress::ProgressReporter;

use crate::rename::RenameFiles;

// Helper: convert BntoError to JsValue at the WASM boundary.
// This conversion used to live in bnto-core, but was removed to keep
// bnto-core target-agnostic (no wasm-bindgen dependency).
fn bnto_err_to_js(error: BntoError) -> JsValue {
    JsError::new(&error.to_string()).into()
}

// =============================================================================
// Rename File — Metadata Variant
// =============================================================================
//
// NOTE: setup() and version() are provided by the bnto-wasm entry point crate.
// Each node crate only exports its specific processing functions here.
// The entry point handles one-time initialization (panic hook, versioning).

/// Rename a single file and return a JSON string with metadata.
///
/// This is the metadata-returning variant. Use `rename_file_bytes()`
/// to get the actual file data (with the new filename in the metadata).
///
/// ARGUMENTS (from JavaScript):
///   - `data` (Uint8Array): The raw file bytes — passed through UNCHANGED
///   - `filename` (string): The original filename (e.g., "IMG_1234.jpg")
///   - `params_json` (string): JSON string with rename config
///     (e.g., '{"prefix": "new-", "case": "lower"}'). Pass '{}' for no changes.
///   - `progress_callback` (Function): Called with (percent: number, message: string)
///     to report progress. The Web Worker forwards this to the main thread.
///
/// RETURNS:
///   A JSON string describing the result:
///   ```json
///   {
///     "file": {
///       "filename": "new-img_1234.jpg",
///       "mimeType": "application/octet-stream",
///       "size": 51200
///     },
///     "metadata": {
///       "originalFilename": "IMG_1234.jpg",
///       "newFilename": "new-img_1234.jpg",
///       "transformsApplied": ["case", "prefix"]
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
pub fn rename_file(
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
    // it'll just pass the filename through unchanged.
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
    let processor = RenameFiles::new();
    // Wrap the JS callback in a Rust closure so ProgressReporter stays
    // target-agnostic (no js_sys dependency in bnto-core).
    let progress = ProgressReporter::new(move |percent, message| {
        let _ = progress_callback.call2(
            &JsValue::NULL,
            &JsValue::from(percent),
            &JsValue::from(message),
        );
    });

    // --- Step 4: Run the rename ---
    //
    // `process()` returns `Result<NodeOutput, BntoError>`.
    // We need to convert the error to a JsValue for the WASM boundary.
    //
    // `.map_err(|e| ...)` transforms the error type if it's Err.
    // `BntoError::into()` uses our `From<BntoError> for JsValue` impl
    // (defined in bnto-core/errors.rs) to create a JS Error object.
    let output = processor
        .process(input, &progress)
        .map_err(bnto_err_to_js)?;

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

// =============================================================================
// Rename File — Bytes Variant
// =============================================================================

/// Rename a single file and return JUST the file bytes.
///
/// This is the efficient path for the Web Worker: instead of encoding
/// file bytes as JSON (which would double the memory usage), this
/// returns the raw bytes as a `Vec<u8>` which wasm-bindgen converts
/// to a `Uint8Array` on the JS side. Zero-copy via shared memory.
///
/// For rename-files, the bytes are IDENTICAL to the input (data passes
/// through unchanged). The value of this function is that the Web Worker
/// can pair the bytes with the new filename from `rename_file()`.
///
/// ARGUMENTS: Same as `rename_file`.
///
/// RETURNS: The raw file bytes as a Uint8Array (identical to input).
///
/// The Web Worker uses this to create a Blob for download:
/// ```js
/// const renamed = rename_file_bytes(data, filename, paramsJson, progressCb);
/// const blob = new Blob([renamed], { type: 'application/octet-stream' });
/// ```
#[wasm_bindgen]
pub fn rename_file_bytes(
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

    let processor = RenameFiles::new();
    // Wrap the JS callback in a Rust closure so ProgressReporter stays
    // target-agnostic (no js_sys dependency in bnto-core).
    let progress = ProgressReporter::new(move |percent, message| {
        let _ = progress_callback.call2(
            &JsValue::NULL,
            &JsValue::from(percent),
            &JsValue::from(message),
        );
    });

    let output = processor
        .process(input, &progress)
        .map_err(bnto_err_to_js)?;

    // Return just the first file's bytes.
    // Rename-files always produces exactly one output file.
    if let Some(file) = output.files.into_iter().next() {
        Ok(file.data)
    } else {
        Err(JsValue::from_str("No output file produced"))
    }
}
