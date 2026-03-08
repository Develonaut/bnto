// =============================================================================
// execute_pipeline — WASM Bridge for the Pipeline Executor
// =============================================================================
//
// WHAT IS THIS FILE?
// This is the WASM-specific bridge that connects the pure Rust pipeline
// executor (in bnto-core) to JavaScript. It handles:
//   1. Parsing JSON definition strings into Rust types
//   2. Converting JavaScript file arrays into Rust PipelineFile vectors
//   3. Creating the default processor registry (all 6 node processors)
//   4. Bridging progress events from Rust to JavaScript callbacks
//   5. Converting Rust results back to JavaScript objects
//
// WHY IS THIS SEPARATE FROM THE EXECUTOR?
// The executor in bnto-core is PURE RUST — no WASM dependencies, no JS types.
// It works with `cargo test` natively. This file is the thin adapter that
// converts between JavaScript and Rust types at the WASM boundary.
//
// HOW THE WEB WORKER USES THIS:
//   ```js
//   const resultJson = execute_pipeline(
//     definitionJsonString,
//     filesArray,          // [{name, data: Uint8Array, mimeType}]
//     (eventJson) => { ... } // receives PipelineEvent JSON strings
//   );
//   const result = JSON.parse(resultJson);
//   ```

use wasm_bindgen::prelude::*;

use bnto_core::{
    NodeRegistry, PipelineEvent, PipelineFile, PipelineReporter,
    execute_pipeline as core_execute_pipeline,
};

// =============================================================================
// Default Registry Builder
// =============================================================================

/// Create a registry pre-loaded with all browser-capable node processors.
///
/// This is the WASM equivalent of the JS-side `registerNodeTypes()` in
/// `wasmLoader.ts`. It maps compound keys (nodeType:operation) to the
/// concrete Rust processor instances.
///
/// Each processor is a zero-sized unit struct, so this function allocates
/// almost nothing — just the HashMap entries with Box pointers.
pub(crate) fn create_default_registry() -> NodeRegistry {
    let mut registry = NodeRegistry::new();

    // --- Image processors ---
    // These handle JPEG, PNG, and WebP files.
    registry.register(
        "image:compress",
        Box::new(bnto_image::CompressImages::new()),
    );
    registry.register(
        "image:resize",
        Box::new(bnto_image::ResizeImages::new()),
    );
    registry.register(
        "image:convert",
        Box::new(bnto_image::ConvertImageFormat::new()),
    );

    // --- CSV/spreadsheet processors ---
    // These handle CSV files (UTF-8 text).
    registry.register(
        "spreadsheet:clean",
        Box::new(bnto_csv::CleanCsv::new()),
    );
    registry.register(
        "spreadsheet:rename",
        Box::new(bnto_csv::RenameCsvColumns::new()),
    );

    // --- File system processors ---
    // These operate on filenames (any file type).
    registry.register(
        "file-system:rename",
        Box::new(bnto_file::RenameFiles::new()),
    );

    registry
}

// =============================================================================
// WASM Export: execute_pipeline
// =============================================================================

/// Execute a complete pipeline in WASM.
///
/// This is the main entry point for the browser. The Web Worker calls this
/// with a JSON definition string, an array of file objects, and a progress
/// callback. The function returns a JSON string with the results.
///
/// # Arguments
/// - `definition_json` — JSON string of the pipeline definition
/// - `files_js` — JavaScript array of `{name: string, data: Uint8Array, mimeType: string}`
/// - `progress_callback` — JavaScript function that receives event JSON strings
///
/// # Returns
/// A JSON string containing the pipeline results:
/// ```json
/// {
///   "files": [{ "name": "...", "data": [bytes], "mimeType": "..." }],
///   "durationMs": 1234
/// }
/// ```
///
/// # Errors
/// Returns a JsValue error string if:
/// - The definition JSON is invalid
/// - A node type isn't registered
/// - A processor fails on a file
#[wasm_bindgen]
pub fn execute_pipeline(
    definition_json: &str,
    files_js: JsValue,
    progress_callback: js_sys::Function,
) -> Result<JsValue, JsValue> {
    // --- Step 1: Parse the pipeline definition from JSON ---
    let definition: bnto_core::PipelineDefinition =
        serde_json::from_str(definition_json).map_err(|e| {
            JsValue::from_str(&format!("Failed to parse pipeline definition: {}", e))
        })?;

    // --- Step 2: Extract files from the JavaScript array ---
    let files = extract_files(files_js)?;

    // --- Step 3: Create the default processor registry ---
    let registry = create_default_registry();

    // --- Step 4: Create a PipelineReporter that calls the JS callback ---
    // Each PipelineEvent is serialized to JSON and passed to the callback.
    // The callback is a JavaScript function: (eventJson: string) => void
    let reporter = PipelineReporter::new(move |event: PipelineEvent| {
        // Serialize the event to a JSON string.
        if let Ok(json) = serde_json::to_string(&event) {
            // Call the JS callback with the JSON string.
            // We ignore errors from the callback — progress is best-effort.
            let _ = progress_callback.call1(&JsValue::NULL, &JsValue::from_str(&json));
        }
    });

    // --- Step 5: Create a time source using js_sys::Date::now() ---
    // In WASM, there's no std::time::Instant. We use the browser's
    // Date.now() which returns milliseconds since epoch.
    let now_ms = || js_sys::Date::now() as u64;

    // --- Step 6: Execute the pipeline ---
    let result = core_execute_pipeline(&definition, files, &registry, &reporter, now_ms)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    // --- Step 7: Convert results to a JavaScript object ---
    // Build a JS array of result file objects.
    let result_array = js_sys::Array::new();

    for file in &result.files {
        let obj = js_sys::Object::new();

        // Set the filename.
        js_sys::Reflect::set(&obj, &"name".into(), &JsValue::from_str(&file.name))
            .map_err(|_| JsValue::from_str("Failed to set result file name"))?;

        // Set the data as a Uint8Array (zero-copy when possible).
        let data = js_sys::Uint8Array::from(file.data.as_slice());
        js_sys::Reflect::set(&obj, &"data".into(), &data)
            .map_err(|_| JsValue::from_str("Failed to set result file data"))?;

        // Set the MIME type.
        js_sys::Reflect::set(&obj, &"mimeType".into(), &JsValue::from_str(&file.mime_type))
            .map_err(|_| JsValue::from_str("Failed to set result file mimeType"))?;

        // Set metadata as a JSON string (if not empty).
        if !file.metadata.is_empty()
            && let Ok(meta_json) = serde_json::to_string(&file.metadata)
        {
            js_sys::Reflect::set(
                &obj,
                &"metadata".into(),
                &JsValue::from_str(&meta_json),
            )
            .map_err(|_| JsValue::from_str("Failed to set result file metadata"))?;
        }

        result_array.push(&obj);
    }

    // Build the top-level result object.
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

// =============================================================================
// Internal: Extract Files from JavaScript Array
// =============================================================================

/// Convert a JavaScript array of file objects into a Vec<PipelineFile>.
///
/// Each JS object is expected to have:
///   - `name` (string) — the filename
///   - `data` (Uint8Array) — the raw file bytes
///   - `mimeType` (string) — the MIME type
fn extract_files(files_js: JsValue) -> Result<Vec<PipelineFile>, JsValue> {
    // --- Step 1: Convert JsValue to a JS Array ---
    let files_array = js_sys::Array::from(&files_js);
    let length = files_array.length();

    let mut files = Vec::with_capacity(length as usize);

    // --- Step 2: Iterate and extract each file ---
    for i in 0..length {
        let file_obj = files_array.get(i);

        // Extract the filename.
        let name = js_sys::Reflect::get(&file_obj, &"name".into())
            .map_err(|_| JsValue::from_str(&format!("File {} missing 'name' field", i)))?
            .as_string()
            .ok_or_else(|| JsValue::from_str(&format!("File {} 'name' is not a string", i)))?;

        // Extract the data as a Uint8Array and copy to a Vec<u8>.
        let data_js = js_sys::Reflect::get(&file_obj, &"data".into())
            .map_err(|_| JsValue::from_str(&format!("File {} missing 'data' field", i)))?;
        let data_uint8 = js_sys::Uint8Array::new(&data_js);
        let data = data_uint8.to_vec();

        // Extract the MIME type.
        let mime_type = js_sys::Reflect::get(&file_obj, &"mimeType".into())
            .ok()
            .and_then(|v| v.as_string())
            .unwrap_or_else(|| "application/octet-stream".to_string());

        files.push(PipelineFile {
            name,
            data,
            mime_type,
        });
    }

    Ok(files)
}

// =============================================================================
// Tests (native — WASM integration tests are in tests/ directory)
// =============================================================================

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
