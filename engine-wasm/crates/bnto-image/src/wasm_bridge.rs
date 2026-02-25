// =============================================================================
// WASM Bridge — The Door Between JavaScript and Rust
// =============================================================================
//
// WHAT IS THIS FILE?
// This is the translation layer between JavaScript (Web Worker) and our
// Rust image processing code. JavaScript can't call Rust functions directly —
// it needs `#[wasm_bindgen]` exported functions that convert between
// JS types (JsValue, Uint8Array, Function) and Rust types (Vec<u8>, &str, etc.).
//
// WHY IS THIS SEPARATE FROM compress.rs?
// Separation of concerns (Bento Box Principle):
//   - compress.rs: Pure Rust image compression logic. Testable natively.
//     Has no knowledge of JavaScript or WASM boundary concerns.
//   - wasm_bridge.rs: Handles the JS ↔ Rust type conversion. Only this
//     file imports wasm_bindgen and deals with JsValue.
//
// This means we can:
//   1. Test compress.rs with normal `cargo test` (fast, no browser needed)
//   2. Test wasm_bridge.rs with `wasm-pack test` (needs Node.js or browser)
//   3. Reuse compress.rs in non-WASM contexts (CLI, desktop) without changes
//
// HOW THE WEB WORKER CALLS US:
//
//   JavaScript (Web Worker):
//   ```js
//   import init, { setup, compress_image } from './pkg/bnto_image.js';
//   await init();
//   setup();
//
//   // Read a File object as bytes
//   const fileBytes = new Uint8Array(await file.arrayBuffer());
//
//   // Call our Rust compression function
//   const result = compress_image(
//       fileBytes,           // raw image bytes
//       "photo.jpg",         // filename
//       '{"quality": 80}',   // JSON config
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

use crate::compress::CompressImages;

// =============================================================================
// Setup — Initialize the WASM Module
// =============================================================================

/// Initialize the bnto-image WASM module. Call this once when the
/// Web Worker starts, before calling any processing functions.
///
/// Sets up the panic hook so Rust errors show real messages in the
/// browser console (instead of cryptic "unreachable" errors).
///
/// SAFE TO CALL MULTIPLE TIMES — the panic hook uses `set_once()`.
#[wasm_bindgen]
pub fn setup() {
    console_error_panic_hook::set_once();
}

/// Returns the version of the bnto-image WASM module.
/// Useful for verifying the correct version is loaded in the browser.
#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

// =============================================================================
// Compress Image — The Main Entry Point for JavaScript
// =============================================================================

/// Compress a single image file and return a JSON string with metadata.
///
/// This is the metadata-returning variant. Use `compress_image_bytes()`
/// to get the actual compressed file data as a Uint8Array.
///
/// ARGUMENTS (from JavaScript):
///   - `data` (Uint8Array): The raw image file bytes
///   - `filename` (string): The original filename (e.g., "photo.jpg")
///   - `params_json` (string): JSON string with compression config
///     (e.g., '{"quality": 80}'). Pass '{}' for defaults.
///   - `progress_callback` (Function): Called with (percent: number, message: string)
///     to report progress. The Web Worker forwards this to the main thread.
///
/// RETURNS:
///   A JSON string describing the result:
///   ```json
///   {
///     "filename": "photo-compressed.jpg",
///     "mimeType": "image/jpeg",
///     "size": 51200,
///     "metadata": {
///       "originalSize": 102400,
///       "compressedSize": 51200,
///       "compressionRatio": 50.0,
///       "format": "Jpeg"
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
pub fn compress_image(
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
    let processor = CompressImages::new();
    let progress = ProgressReporter::new(progress_callback);

    // --- Step 4: Run the compression ---
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

/// Compress a single image and return JUST the compressed bytes.
///
/// This is the efficient path for the Web Worker: instead of encoding
/// file bytes as JSON (which would double the memory usage), this
/// returns the raw bytes as a `Vec<u8>` which wasm-bindgen converts
/// to a `Uint8Array` on the JS side. Zero-copy via shared memory.
///
/// ARGUMENTS: Same as `compress_image`, minus the return format.
///
/// RETURNS: The raw compressed image bytes as a Uint8Array.
///
/// The Web Worker uses this to create a Blob for download:
/// ```js
/// const compressed = compress_image_bytes(data, filename, paramsJson, progressCb);
/// const blob = new Blob([compressed], { type: 'image/jpeg' });
/// ```
#[wasm_bindgen]
pub fn compress_image_bytes(
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

    let processor = CompressImages::new();
    let progress = ProgressReporter::new(progress_callback);

    let output = processor
        .process(input, &progress)
        .map_err(|e| -> JsValue { e.into() })?;

    // Return just the first file's bytes.
    // Compress-images always produces exactly one output file.
    if let Some(file) = output.files.into_iter().next() {
        Ok(file.data)
    } else {
        Err(JsValue::from_str("No output file produced"))
    }
}
