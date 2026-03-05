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
//   import init, { setup, compress_image_combined } from './pkg/bnto_wasm.js';
//   await init();
//   setup();
//
//   // Read a File object as bytes
//   const fileBytes = new Uint8Array(await file.arrayBuffer());
//
//   // Call our Rust compression function (combined = metadata + bytes in one call)
//   const result = compress_image_combined(
//       fileBytes,           // raw image bytes
//       "photo.jpg",         // filename
//       '{"quality": 80}',   // JSON config
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

use crate::compress::CompressImages;
use crate::convert::ConvertImageFormat;
use crate::resize::ResizeImages;

// Helper: convert BntoError to JsValue at the WASM boundary.
// Each node crate has its own copy because Rust's orphan rule prevents
// implementing From<BntoError> for JsValue in bnto-core.
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
// const result = compress_image_combined(data, filename, params, progressCb);
// // result = {
// //   metadata: '{"originalSize":102400,"compressedSize":51200,...}',
// //   data: Uint8Array([...]),       // the raw compressed bytes
// //   filename: "photo-compressed.jpg",
// //   mimeType: "image/jpeg"
// // }
// ```

/// Build a JavaScript object containing both metadata JSON and raw bytes
/// from a single `NodeOutput`.
///
/// WHY A JS OBJECT INSTEAD OF A JSON STRING?
/// The metadata is a JSON string (cheap to serialize, easy to parse in JS).
/// But the file bytes MUST be a `Uint8Array` — encoding megabytes of binary
/// data as a JSON array would be absurdly slow and memory-wasteful.
/// So we build a JS object with `js_sys::Object` that holds both:
///   - `metadata`: a JSON string (same format as the existing metadata functions)
///   - `data`: a Uint8Array (raw bytes, zero-copy via WASM shared memory)
///   - `filename`: the output filename as a string
///   - `mimeType`: the MIME type as a string
///
/// RUST CONCEPT: `js_sys::Reflect::set()`
/// This is how you set properties on a JavaScript object from Rust.
/// It's like doing `obj.key = value` in JS, but through the WASM boundary.
/// `Reflect::set` returns a `Result<bool, JsValue>` — the bool tells us
/// if the property was set successfully, but we only care about errors.
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
    //
    // RUST CONCEPT: `.ok_or_else(|| ...)`
    // Converts an `Option<T>` into a `Result<T, E>`. If the Option is
    // `None` (no file), we produce an error. The closure is only called
    // if it IS None (lazy evaluation — saves allocating the error string
    // in the happy path).
    let file = output
        .files
        .into_iter()
        .next()
        .ok_or_else(|| JsValue::from_str("No output file produced"))?;

    // --- Step 2: Build the metadata JSON string ---
    //
    // We serialize the metadata map to a JSON string. This is the same
    // format the existing metadata functions return, so the Web Worker
    // can parse it the same way.
    //
    // The metadata is a `serde_json::Map<String, serde_json::Value>` —
    // basically a Rust HashMap that mirrors a JSON object. `to_string()`
    // serializes it to a compact JSON string like:
    //   {"originalSize":102400,"compressedSize":51200,"compressionRatio":50.0}
    let metadata_json = serde_json::to_string(&output.metadata)
        .map_err(|e| JsValue::from_str(&format!("Failed to serialize metadata: {e}")))?;

    // --- Step 3: Create a new JavaScript object ---
    //
    // `js_sys::Object::new()` is equivalent to `{}` in JavaScript.
    // We'll set properties on this object one by one.
    let result = js_sys::Object::new();

    // --- Step 4: Set the "metadata" property (JSON string) ---
    js_sys::Reflect::set(&result, &"metadata".into(), &metadata_json.into())
        .map_err(|_| JsValue::from_str("Failed to set metadata on result object"))?;

    // --- Step 5: Set the "data" property (Uint8Array of raw bytes) ---
    //
    // `js_sys::Uint8Array::from(file.data.as_slice())` creates a JS
    // Uint8Array that COPIES the bytes from WASM linear memory into
    // the JS heap. This is necessary because the WASM memory might
    // be resized (invalidating any views into it) before JS reads
    // the data.
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
    //
    // `.into()` converts `js_sys::Object` -> `JsValue`, which is the
    // generic "any JavaScript value" type that wasm-bindgen uses at
    // the boundary.
    Ok(result.into())
}

// =============================================================================
// Compress Image — Combined (Single Process Call)
// =============================================================================
//
// NOTE: setup() and version() are provided by the bnto-wasm entry point crate.
// Each node crate only exports its specific processing functions here.
// The entry point handles one-time initialization (panic hook, versioning).

/// Compress a single image and return BOTH metadata and bytes in one call.
///
/// The image is processed exactly ONCE, and the result contains everything
/// the Web Worker needs — no double processing.
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
///   A JavaScript object with four properties:
///   ```js
///   {
///     metadata: '{"originalSize":102400,"compressedSize":51200,...}',  // JSON string
///     data: Uint8Array([...]),                       // raw compressed bytes
///     filename: "photo-compressed.jpg",              // output filename
///     mimeType: "image/jpeg"                         // MIME type
///   }
///   ```
///
/// RUST CONCEPT: `Result<JsValue, JsValue>`
/// wasm-bindgen functions that can fail return `Result<T, JsValue>`.
/// `Ok(value)` becomes a normal return in JS. `Err(value)` throws a
/// JavaScript Error.
#[wasm_bindgen]
pub fn compress_image_combined(
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
    let processor = CompressImages::new();
    // Wrap the JS callback in a Rust closure.
    //
    // WHY NOT PASS IT DIRECTLY?
    // ProgressReporter is target-agnostic — it accepts any `Fn(u32, &str)`
    // closure, not a js_sys::Function. The WASM bridge is where we adapt
    // between JS types and Rust types. The closure captures the JS function
    // and calls it with JS-compatible values (JsValue).
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

    // --- Step 4: Run the compression (ONCE!) ---
    //
    // `process()` returns `Result<NodeOutput, BntoError>`.
    // We need to convert the error to a JsValue for the WASM boundary.
    //
    // `.map_err(|e| ...)` transforms the error type if it's Err.
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
// Resize Image — Combined (Single Process Call)
// =============================================================================

/// Resize a single image and return BOTH metadata and bytes in one call.
///
/// The image is processed exactly ONCE, and the result contains everything
/// the Web Worker needs — no double processing.
///
/// ARGUMENTS (from JavaScript):
///   - `data` (Uint8Array): The raw image file bytes
///   - `filename` (string): The original filename (e.g., "photo.jpg")
///   - `params_json` (string): JSON string with resize config:
///     ```json
///     {
///       "width": 800,                // Target width in pixels
///       "height": 600,               // Target height (optional if maintainAspect)
///       "maintainAspect": true,      // Preserve aspect ratio (default: true)
///       "quality": 80                // JPEG quality 1-100 (default: 80)
///     }
///     ```
///   - `progress_callback` (Function): Called with (percent, message)
///
/// RETURNS:
///   A JavaScript object with four properties:
///   ```js
///   {
///     metadata: '{"originalSize":102400,"compressedSize":51200,...}',  // JSON string
///     data: Uint8Array([...]),                       // raw resized bytes
///     filename: "photo-resized.jpg",                 // output filename
///     mimeType: "image/jpeg"                         // MIME type
///   }
///   ```
#[wasm_bindgen]
pub fn resize_image_combined(
    data: &[u8],
    filename: &str,
    params_json: &str,
    progress_callback: js_sys::Function,
) -> Result<JsValue, JsValue> {
    // --- Step 1: Parse the config parameters from JSON ---
    let params: serde_json::Map<String, serde_json::Value> =
        serde_json::from_str(params_json).unwrap_or_default();

    // --- Step 2: Build the NodeInput ---
    let input = NodeInput {
        data: data.to_vec(),
        filename: filename.to_string(),
        mime_type: None,
        params,
    };

    // --- Step 3: Create the processor and progress reporter ---
    let processor = ResizeImages::new();
    // Wrap JS callback in a Rust closure. ProgressReporter is target-agnostic —
    // see compress_image_combined() for the full explanation of why we wrap here.
    let progress = ProgressReporter::new(move |percent, message| {
        let _ = progress_callback.call2(
            &JsValue::NULL,
            &JsValue::from(percent),
            &JsValue::from(message),
        );
    });

    // --- Step 4: Run the resize (ONCE!) ---
    let output = processor
        .process(input, &progress)
        .map_err(bnto_err_to_js)?;

    // --- Step 5: Return combined result with both metadata and bytes ---
    build_combined_result(output)
}

// =============================================================================
// Convert Image Format — Combined (Single Process Call)
// =============================================================================

/// Convert a single image to a different format and return BOTH metadata and
/// bytes in one call.
///
/// The image is processed exactly ONCE, and the result contains everything
/// the Web Worker needs — no double processing.
///
/// ARGUMENTS (from JavaScript):
///   - `data` (Uint8Array): The raw image file bytes
///   - `filename` (string): The original filename (e.g., "photo.jpg")
///   - `params_json` (string): JSON string with conversion config:
///     ```json
///     {
///       "format": "png",     // Target format: "jpeg", "png", or "webp" (REQUIRED)
///       "quality": 80        // Quality 1-100 (optional, default 80, WebP capped at 85)
///     }
///     ```
///   - `progress_callback` (Function): Called with (percent, message)
///
/// RETURNS:
///   A JavaScript object with four properties:
///   ```js
///   {
///     metadata: '{"originalSize":102400,"compressedSize":51200,...}',  // JSON string
///     data: Uint8Array([...]),                       // raw converted bytes
///     filename: "photo.png",                         // output filename
///     mimeType: "image/png"                          // MIME type
///   }
///   ```
#[wasm_bindgen]
pub fn convert_image_format_combined(
    data: &[u8],
    filename: &str,
    params_json: &str,
    progress_callback: js_sys::Function,
) -> Result<JsValue, JsValue> {
    // --- Step 1: Parse the config parameters from JSON ---
    let params: serde_json::Map<String, serde_json::Value> =
        serde_json::from_str(params_json).unwrap_or_default();

    // --- Step 2: Build the NodeInput ---
    let input = NodeInput {
        data: data.to_vec(),
        filename: filename.to_string(),
        mime_type: None,
        params,
    };

    // --- Step 3: Create the processor and progress reporter ---
    let processor = ConvertImageFormat::new();
    // Wrap JS callback in a Rust closure. ProgressReporter is target-agnostic —
    // see compress_image_combined() for the full explanation of why we wrap here.
    let progress = ProgressReporter::new(move |percent, message| {
        let _ = progress_callback.call2(
            &JsValue::NULL,
            &JsValue::from(percent),
            &JsValue::from(message),
        );
    });

    // --- Step 4: Run the conversion (ONCE!) ---
    let output = processor
        .process(input, &progress)
        .map_err(bnto_err_to_js)?;

    // --- Step 5: Return combined result with both metadata and bytes ---
    build_combined_result(output)
}
