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
//   import init, { setup, rename_file_combined } from './pkg/bnto_wasm.js';
//   await init();
//   setup();
//
//   // Read a File object as bytes
//   const fileBytes = new Uint8Array(await file.arrayBuffer());
//
//   // Call our Rust rename function (combined = metadata + bytes in one call)
//   const result = rename_file_combined(
//       fileBytes,            // raw file bytes (pass through unchanged)
//       "IMG_1234.jpg",       // original filename
//       '{"prefix": "vacation-", "find": "IMG_", "replace": ""}',
//       (percent, message) => {
//           postMessage({ type: 'progress', percent, message });
//       }
//   );
//
//   // result is a JS object with metadata JSON string + raw bytes
//   const metadata = JSON.parse(result.metadata);
//   const blob = new Blob([result.data], { type: result.mimeType });
//   // metadata.newFilename => "vacation-1234.jpg"
//   ```

use wasm_bindgen::prelude::*;

use bnto_core::errors::BntoError;
use bnto_core::processor::{NodeInput, NodeOutput, NodeProcessor};
use bnto_core::progress::ProgressReporter;

use crate::rename::RenameFiles;

// Helper: convert BntoError to JsValue at the WASM boundary.
// This conversion lives in each node crate (not bnto-core) to keep
// bnto-core target-agnostic (no wasm-bindgen dependency).
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
// which processed every file TWICE.
//
// For rename-files this was especially wasteful because the file data passes
// through UNCHANGED — we were just computing a new filename but running
// process() twice and copying all the bytes twice.
//
// HOW THE RESULT OBJECT LOOKS IN JAVASCRIPT:
// ```js
// const result = rename_file_combined(data, filename, params, progressCb);
// // result = {
// //   metadata: '{"originalFilename":"IMG_1234.jpg","newFilename":"vacation-1234.jpg",...}',
// //   data: Uint8Array([...]),       // the file bytes (unchanged)
// //   filename: "vacation-1234.jpg",
// //   mimeType: "application/octet-stream"
// // }
// ```

/// Build a JavaScript object containing both metadata JSON and raw bytes
/// from a single `NodeOutput`.
///
/// This is the same helper pattern used in bnto-image and bnto-csv. Each
/// node crate has its own copy because:
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
// Rename File — Combined (Single Process Call)
// =============================================================================
//
// NOTE: setup() and version() are provided by the bnto-wasm entry point crate.
// Each node crate only exports its specific processing functions here.
// The entry point handles one-time initialization (panic hook, versioning).

/// Rename a single file and return BOTH metadata and bytes in one call.
///
/// The file is processed exactly ONCE, and the result contains everything
/// the Web Worker needs — no double processing.
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
///   A JavaScript object with four properties:
///   ```js
///   {
///     metadata: '{"originalFilename":"IMG_1234.jpg",...}',  // JSON string
///     data: Uint8Array([...]),                              // raw file bytes (unchanged)
///     filename: "vacation-1234.jpg",                        // new filename
///     mimeType: "application/octet-stream"                  // MIME type
///   }
///   ```
///
/// RUST CONCEPT: `Result<JsValue, JsValue>`
/// wasm-bindgen functions that can fail return `Result<T, JsValue>`.
/// `Ok(value)` becomes a normal return in JS. `Err(value)` throws a
/// JavaScript Error.
#[wasm_bindgen]
pub fn rename_file_combined(
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

    // --- Step 4: Run the rename (ONCE!) ---
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
