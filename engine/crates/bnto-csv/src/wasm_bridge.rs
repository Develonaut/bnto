// =============================================================================
// WASM Bridge — JS-callable functions for CSV processing
// =============================================================================
//
// Translates between JS types (JsValue, Uint8Array, Function) and Rust types.
// Pure CSV logic lives in clean.rs / rename_columns.rs (testable natively);
// this file only handles the WASM boundary conversion.

use wasm_bindgen::prelude::*;

use bnto_core::errors::BntoError;
use bnto_core::processor::{NodeInput, NodeOutput, NodeProcessor};
use bnto_core::progress::ProgressReporter;

use crate::clean::CleanCsv;
use crate::rename_columns::RenameCsvColumns;

// Each node crate has its own copy — Rust's orphan rule prevents implementing
// From<BntoError> for JsValue in bnto-core.
fn bnto_err_to_js(error: BntoError) -> JsValue {
    JsError::new(&error.to_string()).into()
}

// =============================================================================
// Combined Functions — Single-Call Metadata + Bytes
// =============================================================================
//
// Each function calls process() ONCE and returns a JS object with both
// the metadata JSON string and raw output bytes. This eliminates the old
// dual-function pattern that processed every file twice.

/// Pack a NodeOutput into a JS object: { metadata, data, filename, mimeType }.
///
/// Duplicated per node crate to avoid adding js_sys as a bnto-core dependency.
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
// Clean CSV — Combined
// =============================================================================

/// Clean a CSV file and return both metadata and bytes in one call.
///
/// Returns a JS object: `{ metadata: string, data: Uint8Array, filename: string, mimeType: string }`
#[wasm_bindgen]
pub fn clean_csv_combined(
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

    let processor = CleanCsv::new();
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
// Rename CSV Columns — Combined
// =============================================================================

/// Rename columns in a CSV file and return both metadata and bytes in one call.
///
/// Returns a JS object: `{ metadata: string, data: Uint8Array, filename: string, mimeType: string }`
#[wasm_bindgen]
pub fn rename_csv_columns_combined(
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
        mime_type: Some("text/csv".to_string()),
        params,
    };

    let processor = RenameCsvColumns::new();
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
