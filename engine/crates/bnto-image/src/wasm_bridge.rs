// =============================================================================
// WASM Bridge — JS <-> Rust Type Conversion Layer
// =============================================================================
//
// Translation layer between JavaScript (Web Worker) and Rust image processing.
// Separated from processing code (Bento Box Principle) so that:
//   - Processing code (compress.rs, etc.) is pure Rust, testable natively
//   - Only this file imports wasm_bindgen and deals with JsValue
//   - Processing code is reusable in non-WASM contexts (CLI, desktop)
//
// Each combined function calls process() ONCE and returns a JS object with
// both metadata JSON and raw bytes — eliminating the old dual-function
// pattern that processed every file twice.

use wasm_bindgen::prelude::*;

use bnto_core::errors::BntoError;
use bnto_core::processor::{NodeInput, NodeOutput, NodeProcessor};
use bnto_core::progress::ProgressReporter;

use crate::compress::CompressImages;
use crate::convert::ConvertImageFormat;
use crate::resize::ResizeImages;

// Orphan rule prevents implementing From<BntoError> for JsValue in bnto-core,
// so each node crate has its own converter.
fn bnto_err_to_js(error: BntoError) -> JsValue {
    JsError::new(&error.to_string()).into()
}

// =============================================================================
// Combined Result Builder
// =============================================================================

/// Build a JS object from a NodeOutput: { metadata, data, filename, mimeType }
///
/// metadata: JSON string (cheap to serialize, easy to parse in JS)
/// data: Uint8Array (binary data must stay as bytes, not JSON-encoded)
/// filename/mimeType: strings for the Web Worker to construct a download Blob
fn build_combined_result(output: NodeOutput) -> Result<JsValue, JsValue> {
    let file = output
        .files
        .into_iter()
        .next()
        .ok_or_else(|| JsValue::from_str("No output file produced"))?;

    let metadata_json = serde_json::to_string(&output.metadata)
        .map_err(|e| JsValue::from_str(&format!("Failed to serialize metadata: {e}")))?;

    let result = js_sys::Object::new();

    js_sys::Reflect::set(&result, &"metadata".into(), &metadata_json.into())
        .map_err(|_| JsValue::from_str("Failed to set metadata on result object"))?;

    // Uint8Array copies bytes from WASM linear memory into the JS heap.
    // Necessary because WASM memory might be resized before JS reads the data.
    let uint8 = js_sys::Uint8Array::from(file.data.as_slice());
    js_sys::Reflect::set(&result, &"data".into(), &uint8)
        .map_err(|_| JsValue::from_str("Failed to set data on result object"))?;

    js_sys::Reflect::set(&result, &"filename".into(), &file.filename.into())
        .map_err(|_| JsValue::from_str("Failed to set filename on result object"))?;

    js_sys::Reflect::set(&result, &"mimeType".into(), &file.mime_type.into())
        .map_err(|_| JsValue::from_str("Failed to set mimeType on result object"))?;

    Ok(result.into())
}

// =============================================================================
// Compress Image — Combined
// =============================================================================
//
// setup() and version() are provided by the bnto-wasm entry point crate.
// Each node crate only exports its specific processing functions here.

/// Compress a single image. Returns { metadata, data, filename, mimeType }.
#[wasm_bindgen]
pub fn compress_image_combined(
    data: &[u8],
    filename: &str,
    params_json: &str,
    progress_callback: js_sys::Function,
) -> Result<JsValue, JsValue> {
    let params: serde_json::Map<String, serde_json::Value> =
        serde_json::from_str(params_json).unwrap_or_default();

    let input = NodeInput {
        data: data.to_vec(),
        filename: filename.to_string(),
        mime_type: None,
        params,
    };

    let processor = CompressImages::new();
    // ProgressReporter is target-agnostic (accepts any Fn(u32, &str)).
    // The WASM bridge adapts the JS callback into a Rust closure here.
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

    build_combined_result(output)
}

// =============================================================================
// Resize Image — Combined
// =============================================================================

/// Resize a single image. Returns { metadata, data, filename, mimeType }.
#[wasm_bindgen]
pub fn resize_image_combined(
    data: &[u8],
    filename: &str,
    params_json: &str,
    progress_callback: js_sys::Function,
) -> Result<JsValue, JsValue> {
    let params: serde_json::Map<String, serde_json::Value> =
        serde_json::from_str(params_json).unwrap_or_default();

    let input = NodeInput {
        data: data.to_vec(),
        filename: filename.to_string(),
        mime_type: None,
        params,
    };

    let processor = ResizeImages::new();
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

    build_combined_result(output)
}

// =============================================================================
// Convert Image Format — Combined
// =============================================================================

/// Convert a single image to a different format. Returns { metadata, data, filename, mimeType }.
#[wasm_bindgen]
pub fn convert_image_format_combined(
    data: &[u8],
    filename: &str,
    params_json: &str,
    progress_callback: js_sys::Function,
) -> Result<JsValue, JsValue> {
    let params: serde_json::Map<String, serde_json::Value> =
        serde_json::from_str(params_json).unwrap_or_default();

    let input = NodeInput {
        data: data.to_vec(),
        filename: filename.to_string(),
        mime_type: None,
        params,
    };

    let processor = ConvertImageFormat::new();
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

    build_combined_result(output)
}
