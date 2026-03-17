// WASM Bridge for the Pipeline Executor.
//
// Thin adapter between JavaScript types and the pure Rust executor in bnto-core.
// Handles: JSON parsing, JS file array extraction, registry creation,
// progress event bridging, and result conversion back to JS objects.

use wasm_bindgen::prelude::*;

use bnto_core::{
    NodeRegistry, PipelineEvent, PipelineFile, PipelineReporter,
    execute_pipeline as core_execute_pipeline,
};

/// Create a registry pre-loaded with all browser-capable node processors.
/// Maps compound keys (nodeType:operation) to Rust processor instances.
pub(crate) fn create_default_registry() -> NodeRegistry {
    let mut registry = NodeRegistry::new();

    registry.register(
        "image:compress",
        Box::new(bnto_image::CompressImages::new()),
    );
    registry.register("image:resize", Box::new(bnto_image::ResizeImages::new()));
    registry.register(
        "image:convert",
        Box::new(bnto_image::ConvertImageFormat::new()),
    );

    registry.register("spreadsheet:clean", Box::new(bnto_csv::CleanCsv::new()));
    registry.register(
        "spreadsheet:rename",
        Box::new(bnto_csv::RenameCsvColumns::new()),
    );

    registry.register(
        "file-system:rename",
        Box::new(bnto_file::RenameFiles::new()),
    );

    registry
}

/// Convert a single `PipelineFileResult` to a JS object with name, data, mimeType, metadata.
fn file_result_to_js(file: &bnto_core::PipelineFileResult) -> Result<js_sys::Object, JsValue> {
    let obj = js_sys::Object::new();

    js_sys::Reflect::set(&obj, &"name".into(), &JsValue::from_str(&file.name))
        .map_err(|_| JsValue::from_str("Failed to set result file name"))?;

    let data = js_sys::Uint8Array::from(file.data.as_slice());
    js_sys::Reflect::set(&obj, &"data".into(), &data)
        .map_err(|_| JsValue::from_str("Failed to set result file data"))?;

    js_sys::Reflect::set(
        &obj,
        &"mimeType".into(),
        &JsValue::from_str(&file.mime_type),
    )
    .map_err(|_| JsValue::from_str("Failed to set result file mimeType"))?;

    if !file.metadata.is_empty()
        && let Ok(meta_json) = serde_json::to_string(&file.metadata)
    {
        js_sys::Reflect::set(&obj, &"metadata".into(), &JsValue::from_str(&meta_json))
            .map_err(|_| JsValue::from_str("Failed to set result file metadata"))?;
    }

    Ok(obj)
}

/// Convert a `PipelineResult` to a JS object with `files` array and `durationMs`.
fn convert_results_to_js(result: &bnto_core::PipelineResult) -> Result<JsValue, JsValue> {
    let result_array = js_sys::Array::new();
    for file in &result.files {
        result_array.push(&file_result_to_js(file)?.into());
    }

    let result_obj = js_sys::Object::new();
    js_sys::Reflect::set(&result_obj, &"files".into(), &result_array)
        .map_err(|_| JsValue::from_str("Failed to set result files"))?;
    js_sys::Reflect::set(
        &result_obj,
        &"durationMs".into(),
        &JsValue::from_f64(result.duration_ms as f64),
    )
    .map_err(|_| JsValue::from_str("Failed to set result durationMs"))?;

    Ok(result_obj.into())
}

/// Execute a complete pipeline in WASM. Main entry point for the browser.
///
/// Takes a JSON definition string, a JS array of file objects, and a progress
/// callback. Returns a JS object with `files` array and `durationMs`.
#[wasm_bindgen]
pub fn execute_pipeline(
    definition_json: &str,
    files_js: JsValue,
    progress_callback: js_sys::Function,
) -> Result<JsValue, JsValue> {
    let definition: bnto_core::PipelineDefinition = serde_json::from_str(definition_json)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse pipeline definition: {}", e)))?;

    let files = extract_files(files_js)?;
    let registry = create_default_registry();

    let reporter = PipelineReporter::new(move |event: PipelineEvent| {
        if let Ok(json) = serde_json::to_string(&event) {
            let _ = progress_callback.call1(&JsValue::NULL, &JsValue::from_str(&json));
        }
    });

    // WASM has no std::time::Instant — use browser's Date.now().
    let now_ms = || js_sys::Date::now() as u64;

    let result = core_execute_pipeline(&definition, files, &registry, &reporter, now_ms)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    convert_results_to_js(&result)
}

/// Convert a JavaScript array of `{name, data: Uint8Array, mimeType}` to `Vec<PipelineFile>`.
fn extract_files(files_js: JsValue) -> Result<Vec<PipelineFile>, JsValue> {
    let files_array = js_sys::Array::from(&files_js);
    let length = files_array.length();
    let mut files = Vec::with_capacity(length as usize);

    for i in 0..length {
        files.push(extract_single_file(&files_array.get(i), i)?);
    }

    Ok(files)
}

/// Extract a single file object from JS into a `PipelineFile`.
fn extract_single_file(file_obj: &JsValue, index: u32) -> Result<PipelineFile, JsValue> {
    let name = js_sys::Reflect::get(file_obj, &"name".into())
        .map_err(|_| JsValue::from_str(&format!("File {} missing 'name' field", index)))?
        .as_string()
        .ok_or_else(|| JsValue::from_str(&format!("File {} 'name' is not a string", index)))?;

    let data_js = js_sys::Reflect::get(file_obj, &"data".into())
        .map_err(|_| JsValue::from_str(&format!("File {} missing 'data' field", index)))?;
    let data = js_sys::Uint8Array::new(&data_js).to_vec();

    let mime_type = js_sys::Reflect::get(file_obj, &"mimeType".into())
        .ok()
        .and_then(|v| v.as_string())
        .unwrap_or_else(|| "application/octet-stream".to_string());

    Ok(PipelineFile {
        name,
        data,
        mime_type,
        metadata: serde_json::Map::new(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default_registry_has_all_processors() {
        let registry = create_default_registry();

        // We registered 6 processors.
        assert_eq!(registry.len(), 6);

        // Verify each compound key resolves to a processor.
        let keys = [
            ("image", "compress"),
            ("image", "resize"),
            ("image", "convert"),
            ("spreadsheet", "clean"),
            ("spreadsheet", "rename"),
            ("file-system", "rename"),
        ];

        for (node_type, operation) in &keys {
            let mut params = serde_json::Map::new();
            params.insert(
                "operation".to_string(),
                serde_json::Value::String(operation.to_string()),
            );

            let processor = registry.resolve(node_type, &params);
            assert!(
                processor.is_some(),
                "Should resolve processor for {}:{}",
                node_type,
                operation
            );
        }
    }
}
