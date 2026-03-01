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
//   import init, { setup, clean_csv_combined } from './pkg/bnto_wasm.js';
//   await init();
//   setup();
//
//   // Read a File object as bytes
//   const fileBytes = new Uint8Array(await file.arrayBuffer());
//
//   // Call our Rust CSV cleaning function (combined = metadata + bytes in one call)
//   const result = clean_csv_combined(
//       fileBytes,                          // raw CSV bytes
//       "data.csv",                         // filename
//       '{"removeDuplicates": true}',       // JSON config
//       (percent, message) => {
//           postMessage({ type: 'progress', percent, message });
//       }
//   );
//
//   // result is a JS object with metadata JSON string + raw bytes
//   const metadata = JSON.parse(result.metadata);
//   const blob = new Blob([result.data], { type: result.mimeType });
//   ```

use wasm_bindgen::prelude::*;

use bnto_core::errors::BntoError;
use bnto_core::processor::{NodeInput, NodeOutput, NodeProcessor};
use bnto_core::progress::ProgressReporter;

use crate::clean::CleanCsv;
use crate::rename_columns::RenameCsvColumns;

// Helper: convert BntoError to JsValue at the WASM boundary.
fn bnto_err_to_js(error: BntoError) -> JsValue {
    JsError::new(&error.to_string()).into()
}

// =============================================================================
// Combined Functions — Single-Call Metadata + Bytes
// =============================================================================
//
// Each combined function calls process() ONCE and returns a JavaScript object
// containing BOTH the metadata JSON string AND the raw output bytes. This
// eliminates the old dual-function pattern (metadata + bytes as separate calls)
// which processed every file TWICE — doubling CPU time and causing progress
// to go 0% -> 100% -> 0% -> 100%.
//
// HOW THE RESULT OBJECT LOOKS IN JAVASCRIPT:
// ```js
// const result = clean_csv_combined(data, filename, params, progressCb);
// // result = {
// //   metadata: '{"originalRows":100,"cleanedRows":85,...}',
// //   data: Uint8Array([...]),       // the raw cleaned CSV bytes
// //   filename: "data-cleaned.csv",
// //   mimeType: "text/csv"
// // }
// ```

/// Build a JavaScript object containing both metadata JSON and raw bytes
/// from a single `NodeOutput`.
///
/// This is the same helper pattern used in bnto-image. Each node crate
/// has its own copy because:
///   1. It avoids adding js_sys as a dependency to bnto-core (which must
///      stay target-agnostic — no WASM-specific code)
///   2. The function is small enough that duplication is clearer than a
///      shared utility that would need its own crate
///
/// RUST CONCEPT: `js_sys::Reflect::set()`
/// This is how you set properties on a JavaScript object from Rust.
/// It's like doing `obj.key = value` in JS, but through the WASM boundary.
///
/// RUST CONCEPT: `.into()`
/// When you see `&"metadata".into()`, the `.into()` converts a Rust `&str`
/// into a `JsValue` (a JavaScript string). wasm-bindgen provides these
/// conversions automatically via the `From` trait.
fn build_combined_result(output: NodeOutput) -> Result<JsValue, JsValue> {
    // --- Step 1: Extract the first output file ---
    //
    // All current node types produce exactly one output file.
    // `.into_iter().next()` moves the first file out of the Vec,
    // consuming the Vec in the process (no copying).
    let file = output
        .files
        .into_iter()
        .next()
        .ok_or_else(|| JsValue::from_str("No output file produced"))?;

    // --- Step 2: Serialize metadata to a JSON string ---
    let metadata_json = serde_json::to_string(&output.metadata)
        .map_err(|e| JsValue::from_str(&format!("Failed to serialize metadata: {e}")))?;

    // --- Step 3: Create a new JavaScript object ---
    let result = js_sys::Object::new();

    // --- Step 4: Set the "metadata" property (JSON string) ---
    js_sys::Reflect::set(&result, &"metadata".into(), &metadata_json.into())
        .map_err(|_| JsValue::from_str("Failed to set metadata on result object"))?;

    // --- Step 5: Set the "data" property (Uint8Array of raw bytes) ---
    //
    // `js_sys::Uint8Array::from(file.data.as_slice())` creates a JS
    // Uint8Array that copies the bytes from WASM linear memory into
    // the JS heap.
    let uint8 = js_sys::Uint8Array::from(file.data.as_slice());
    js_sys::Reflect::set(&result, &"data".into(), &uint8)
        .map_err(|_| JsValue::from_str("Failed to set data on result object"))?;

    // --- Step 6: Set the "filename" property ---
    js_sys::Reflect::set(&result, &"filename".into(), &file.filename.into())
        .map_err(|_| JsValue::from_str("Failed to set filename on result object"))?;

    // --- Step 7: Set the "mimeType" property ---
    js_sys::Reflect::set(&result, &"mimeType".into(), &file.mime_type.into())
        .map_err(|_| JsValue::from_str("Failed to set mimeType on result object"))?;

    // --- Step 8: Return the object as a JsValue ---
    Ok(result.into())
}

// =============================================================================
// Clean CSV — Combined (Single Process Call)
// =============================================================================
//
// NOTE: setup() and version() are provided by the bnto-wasm entry point crate.
// Each node crate only exports its specific processing functions here.
// The entry point handles one-time initialization (panic hook, versioning).

/// Clean a CSV file and return BOTH metadata and bytes in one call.
///
/// The CSV is processed exactly ONCE, and the result contains everything
/// the Web Worker needs — no double processing.
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
///   A JavaScript object with four properties:
///   ```js
///   {
///     metadata: '{"originalRows":100,"cleanedRows":85,...}',  // JSON string
///     data: Uint8Array([...]),                                // raw cleaned CSV bytes
///     filename: "data-cleaned.csv",                           // output filename
///     mimeType: "text/csv"                                    // MIME type
///   }
///   ```
///
/// RUST CONCEPT: `Result<JsValue, JsValue>`
/// wasm-bindgen functions that can fail return `Result<T, JsValue>`.
/// `Ok(value)` becomes a normal return in JS. `Err(value)` throws a
/// JavaScript Error.
#[wasm_bindgen]
pub fn clean_csv_combined(
    data: &[u8],
    filename: &str,
    params_json: &str,
    progress_callback: js_sys::Function,
) -> Result<JsValue, JsValue> {
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
    // Wrap the JS callback in a Rust closure so ProgressReporter stays
    // target-agnostic (no js_sys dependency in bnto-core).
    //
    // `move` transfers ownership of `progress_callback` into the closure.
    // `let _` ignores the Result from call2() — progress is best-effort.
    let progress = ProgressReporter::new(move |percent, message| {
        let _ = progress_callback.call2(
            &JsValue::NULL,
            &JsValue::from(percent),
            &JsValue::from(message),
        );
    });

    // --- Step 4: Run the CSV cleaning (ONCE!) ---
    //
    // `process()` returns `Result<NodeOutput, BntoError>`.
    // We need to convert the error to a JsValue for the WASM boundary.
    let output = processor
        .process(input, &progress)
        .map_err(bnto_err_to_js)?;

    // --- Step 5: Return combined result with both metadata and bytes ---
    //
    // This packs the NodeOutput into a single JS object containing
    // metadata JSON string + raw Uint8Array bytes + filename + mimeType.
    build_combined_result(output)
}

// =============================================================================
// Rename CSV Columns — Combined (Single Process Call)
// =============================================================================

/// Rename columns in a CSV file and return BOTH metadata and bytes in one call.
///
/// The CSV is processed exactly ONCE, and the result contains everything
/// the Web Worker needs — no double processing.
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
///   A JavaScript object with four properties:
///   ```js
///   {
///     metadata: '{"columnsRenamed":2,"totalColumns":5,...}',  // JSON string
///     data: Uint8Array([...]),                                // raw modified CSV bytes
///     filename: "data-renamed.csv",                           // output filename
///     mimeType: "text/csv"                                    // MIME type
///   }
///   ```
#[wasm_bindgen]
pub fn rename_csv_columns_combined(
    data: &[u8],
    filename: &str,
    params_json: &str,
    progress_callback: js_sys::Function,
) -> Result<JsValue, JsValue> {
    // --- Step 1: Parse the config parameters from JSON ---
    let params: serde_json::Map<String, serde_json::Value> =
        serde_json::from_str(params_json).unwrap_or_default();

    // --- Step 2: Build the NodeInput ---
    //
    // NOTE: Unlike clean_csv, rename_csv_columns sets mime_type to
    // "text/csv" explicitly. This ensures the output metadata correctly
    // reflects the CSV content type.
    let input = NodeInput {
        data: data.to_vec(),
        filename: filename.to_string(),
        mime_type: Some("text/csv".to_string()),
        params,
    };

    // --- Step 3: Create the processor and progress reporter ---
    let processor = RenameCsvColumns::new();
    // Wrap the JS callback in a Rust closure so ProgressReporter stays
    // target-agnostic (no js_sys dependency in bnto-core).
    let progress = ProgressReporter::new(move |percent, message| {
        let _ = progress_callback.call2(
            &JsValue::NULL,
            &JsValue::from(percent),
            &JsValue::from(message),
        );
    });

    // --- Step 4: Run the rename operation (ONCE!) ---
    let output = processor
        .process(input, &progress)
        .map_err(bnto_err_to_js)?;

    // --- Step 5: Return combined result with both metadata and bytes ---
    build_combined_result(output)
}
