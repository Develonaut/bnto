/* @ts-self-types="./bnto_wasm.d.ts" */

/**
 * Clean a CSV file and return a JSON string with metadata.
 *
 * This is the metadata-returning variant. Use `clean_csv_bytes()`
 * to get the actual cleaned file data as a Uint8Array.
 *
 * ARGUMENTS (from JavaScript):
 *   - `data` (Uint8Array): The raw CSV file bytes
 *   - `filename` (string): The original filename (e.g., "data.csv")
 *   - `params_json` (string): JSON string with cleaning config
 *     (e.g., '{"removeDuplicates": true}'). Pass '{}' for defaults.
 *   - `progress_callback` (Function): Called with (percent: number, message: string)
 *     to report progress. The Web Worker forwards this to the main thread.
 *
 * RETURNS:
 *   A JSON string describing the result:
 *   ```json
 *   {
 *     "file": {
 *       "filename": "data-cleaned.csv",
 *       "mimeType": "text/csv",
 *       "size": 1024
 *     },
 *     "metadata": {
 *       "originalRows": 100,
 *       "cleanedRows": 85,
 *       "rowsRemoved": 15,
 *       "duplicatesRemoved": 5
 *     }
 *   }
 *   ```
 *
 * WHY RETURN A JSON STRING?
 * We serialize to a JSON string instead of building a JS object with
 * `js_sys::Object` or `serde-wasm-bindgen`. The reason: `serde-wasm-bindgen`
 * pulls in the ENTIRE `js_sys` binding surface (thousands of symbols),
 * which bloats the WASM binary and causes linker issues in test builds.
 * A JSON string is simple, debuggable, and tiny in code size.
 *
 * RUST CONCEPT: `Result<String, JsValue>`
 * wasm-bindgen functions that can fail return `Result<T, JsValue>`.
 * `Ok(value)` becomes a normal return in JS. `Err(value)` throws a
 * JavaScript Error.
 * @param {Uint8Array} data
 * @param {string} filename
 * @param {string} params_json
 * @param {Function} progress_callback
 * @returns {string}
 */
export function clean_csv(data, filename, params_json, progress_callback) {
    let deferred5_0;
    let deferred5_1;
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_export3);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(filename, wasm.__wbindgen_export3, wasm.__wbindgen_export4);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passStringToWasm0(params_json, wasm.__wbindgen_export3, wasm.__wbindgen_export4);
        const len2 = WASM_VECTOR_LEN;
        wasm.clean_csv(retptr, ptr0, len0, ptr1, len1, ptr2, len2, addHeapObject(progress_callback));
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
        var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
        var ptr4 = r0;
        var len4 = r1;
        if (r3) {
            ptr4 = 0; len4 = 0;
            throw takeObject(r2);
        }
        deferred5_0 = ptr4;
        deferred5_1 = len4;
        return getStringFromWasm0(ptr4, len4);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_export2(deferred5_0, deferred5_1, 1);
    }
}

/**
 * Clean a CSV file and return JUST the cleaned bytes.
 *
 * This is the efficient path for the Web Worker: instead of encoding
 * file bytes as JSON (which would double the memory usage), this
 * returns the raw bytes as a `Vec<u8>` which wasm-bindgen converts
 * to a `Uint8Array` on the JS side. Zero-copy via shared memory.
 *
 * ARGUMENTS: Same as `clean_csv`, minus the return format.
 *
 * RETURNS: The raw cleaned CSV bytes as a Uint8Array.
 *
 * The Web Worker uses this to create a Blob for download:
 * ```js
 * const cleaned = clean_csv_bytes(data, filename, paramsJson, progressCb);
 * const blob = new Blob([cleaned], { type: 'text/csv' });
 * ```
 * @param {Uint8Array} data
 * @param {string} filename
 * @param {string} params_json
 * @param {Function} progress_callback
 * @returns {Uint8Array}
 */
export function clean_csv_bytes(data, filename, params_json, progress_callback) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_export3);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(filename, wasm.__wbindgen_export3, wasm.__wbindgen_export4);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passStringToWasm0(params_json, wasm.__wbindgen_export3, wasm.__wbindgen_export4);
        const len2 = WASM_VECTOR_LEN;
        wasm.clean_csv_bytes(retptr, ptr0, len0, ptr1, len1, ptr2, len2, addHeapObject(progress_callback));
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
        var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
        if (r3) {
            throw takeObject(r2);
        }
        var v4 = getArrayU8FromWasm0(r0, r1).slice();
        wasm.__wbindgen_export2(r0, r1 * 1, 1);
        return v4;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * Compress a single image file and return a JSON string with metadata.
 *
 * This is the metadata-returning variant. Use `compress_image_bytes()`
 * to get the actual compressed file data as a Uint8Array.
 *
 * ARGUMENTS (from JavaScript):
 *   - `data` (Uint8Array): The raw image file bytes
 *   - `filename` (string): The original filename (e.g., "photo.jpg")
 *   - `params_json` (string): JSON string with compression config
 *     (e.g., '{"quality": 80}'). Pass '{}' for defaults.
 *   - `progress_callback` (Function): Called with (percent: number, message: string)
 *     to report progress. The Web Worker forwards this to the main thread.
 *
 * RETURNS:
 *   A JSON string describing the result:
 *   ```json
 *   {
 *     "filename": "photo-compressed.jpg",
 *     "mimeType": "image/jpeg",
 *     "size": 51200,
 *     "metadata": {
 *       "originalSize": 102400,
 *       "compressedSize": 51200,
 *       "compressionRatio": 50.0,
 *       "format": "Jpeg"
 *     }
 *   }
 *   ```
 *
 * WHY RETURN A JSON STRING?
 * We serialize to a JSON string instead of building a JS object with
 * `js_sys::Object` or `serde-wasm-bindgen`. The reason: `serde-wasm-bindgen`
 * pulls in the ENTIRE `js_sys` binding surface (thousands of symbols),
 * which bloats the WASM binary and causes linker issues in test builds.
 * A JSON string is simple, debuggable, and tiny in code size.
 *
 * RUST CONCEPT: `Result<String, JsValue>`
 * wasm-bindgen functions that can fail return `Result<T, JsValue>`.
 * `Ok(value)` becomes a normal return in JS. `Err(value)` throws a
 * JavaScript Error.
 * @param {Uint8Array} data
 * @param {string} filename
 * @param {string} params_json
 * @param {Function} progress_callback
 * @returns {string}
 */
export function compress_image(data, filename, params_json, progress_callback) {
    let deferred5_0;
    let deferred5_1;
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_export3);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(filename, wasm.__wbindgen_export3, wasm.__wbindgen_export4);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passStringToWasm0(params_json, wasm.__wbindgen_export3, wasm.__wbindgen_export4);
        const len2 = WASM_VECTOR_LEN;
        wasm.compress_image(retptr, ptr0, len0, ptr1, len1, ptr2, len2, addHeapObject(progress_callback));
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
        var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
        var ptr4 = r0;
        var len4 = r1;
        if (r3) {
            ptr4 = 0; len4 = 0;
            throw takeObject(r2);
        }
        deferred5_0 = ptr4;
        deferred5_1 = len4;
        return getStringFromWasm0(ptr4, len4);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_export2(deferred5_0, deferred5_1, 1);
    }
}

/**
 * Compress a single image and return JUST the compressed bytes.
 *
 * This is the efficient path for the Web Worker: instead of encoding
 * file bytes as JSON (which would double the memory usage), this
 * returns the raw bytes as a `Vec<u8>` which wasm-bindgen converts
 * to a `Uint8Array` on the JS side. Zero-copy via shared memory.
 *
 * ARGUMENTS: Same as `compress_image`, minus the return format.
 *
 * RETURNS: The raw compressed image bytes as a Uint8Array.
 *
 * The Web Worker uses this to create a Blob for download:
 * ```js
 * const compressed = compress_image_bytes(data, filename, paramsJson, progressCb);
 * const blob = new Blob([compressed], { type: 'image/jpeg' });
 * ```
 * @param {Uint8Array} data
 * @param {string} filename
 * @param {string} params_json
 * @param {Function} progress_callback
 * @returns {Uint8Array}
 */
export function compress_image_bytes(data, filename, params_json, progress_callback) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_export3);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(filename, wasm.__wbindgen_export3, wasm.__wbindgen_export4);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passStringToWasm0(params_json, wasm.__wbindgen_export3, wasm.__wbindgen_export4);
        const len2 = WASM_VECTOR_LEN;
        wasm.compress_image_bytes(retptr, ptr0, len0, ptr1, len1, ptr2, len2, addHeapObject(progress_callback));
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
        var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
        if (r3) {
            throw takeObject(r2);
        }
        var v4 = getArrayU8FromWasm0(r0, r1).slice();
        wasm.__wbindgen_export2(r0, r1 * 1, 1);
        return v4;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * Convert a single image to a different format and return a JSON string with metadata.
 *
 * ARGUMENTS (from JavaScript):
 *   - `data` (Uint8Array): The raw image file bytes
 *   - `filename` (string): The original filename (e.g., "photo.jpg")
 *   - `params_json` (string): JSON string with conversion config:
 *     ```json
 *     {
 *       "format": "png",     // Target format: "jpeg", "png", or "webp" (REQUIRED)
 *       "quality": 80        // Quality 1-100 (optional, default 80, WebP capped at 85)
 *     }
 *     ```
 *   - `progress_callback` (Function): Called with (percent, message)
 *
 * RETURNS:
 *   A JSON string describing the result:
 *   ```json
 *   {
 *     "file": {
 *       "filename": "photo.png",
 *       "mimeType": "image/png",
 *       "size": 51200
 *     },
 *     "metadata": {
 *       "originalFormat": "Jpeg",
 *       "targetFormat": "Png",
 *       "originalSize": 102400,
 *       "newSize": 51200
 *     }
 *   }
 *   ```
 * @param {Uint8Array} data
 * @param {string} filename
 * @param {string} params_json
 * @param {Function} progress_callback
 * @returns {string}
 */
export function convert_image_format(data, filename, params_json, progress_callback) {
    let deferred5_0;
    let deferred5_1;
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_export3);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(filename, wasm.__wbindgen_export3, wasm.__wbindgen_export4);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passStringToWasm0(params_json, wasm.__wbindgen_export3, wasm.__wbindgen_export4);
        const len2 = WASM_VECTOR_LEN;
        wasm.convert_image_format(retptr, ptr0, len0, ptr1, len1, ptr2, len2, addHeapObject(progress_callback));
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
        var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
        var ptr4 = r0;
        var len4 = r1;
        if (r3) {
            ptr4 = 0; len4 = 0;
            throw takeObject(r2);
        }
        deferred5_0 = ptr4;
        deferred5_1 = len4;
        return getStringFromWasm0(ptr4, len4);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_export2(deferred5_0, deferred5_1, 1);
    }
}

/**
 * Convert a single image to a different format and return JUST the converted bytes.
 *
 * Efficient path for the Web Worker — raw bytes returned as Uint8Array
 * via wasm-bindgen's zero-copy memory sharing.
 *
 * ARGUMENTS: Same as `convert_image_format`.
 * RETURNS: The raw converted image bytes as a Uint8Array.
 * @param {Uint8Array} data
 * @param {string} filename
 * @param {string} params_json
 * @param {Function} progress_callback
 * @returns {Uint8Array}
 */
export function convert_image_format_bytes(data, filename, params_json, progress_callback) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_export3);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(filename, wasm.__wbindgen_export3, wasm.__wbindgen_export4);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passStringToWasm0(params_json, wasm.__wbindgen_export3, wasm.__wbindgen_export4);
        const len2 = WASM_VECTOR_LEN;
        wasm.convert_image_format_bytes(retptr, ptr0, len0, ptr1, len1, ptr2, len2, addHeapObject(progress_callback));
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
        var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
        if (r3) {
            throw takeObject(r2);
        }
        var v4 = getArrayU8FromWasm0(r0, r1).slice();
        wasm.__wbindgen_export2(r0, r1 * 1, 1);
        return v4;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * Health check — proves the WASM module is loaded and working.
 *
 * Takes a name and returns a greeting. The Web Worker can call this
 * after init() to verify:
 *   1. WASM binary loaded correctly
 *   2. String data crosses the Rust ↔ JS boundary properly
 *   3. wasm-bindgen's type conversion works
 *
 * EXAMPLE:
 * ```js
 * const msg = greet("Ryan");  // "Hello from Bnto WASM engine, Ryan! v0.1.0"
 * ```
 * @param {string} name
 * @returns {string}
 */
export function greet(name) {
    let deferred2_0;
    let deferred2_1;
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(name, wasm.__wbindgen_export3, wasm.__wbindgen_export4);
        const len0 = WASM_VECTOR_LEN;
        wasm.greet(retptr, ptr0, len0);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        deferred2_0 = r0;
        deferred2_1 = r1;
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_export2(deferred2_0, deferred2_1, 1);
    }
}

/**
 * Rename columns in a CSV file and return a JSON string with metadata.
 *
 * This is the metadata-returning variant. Use `rename_csv_columns_bytes()`
 * to get the actual modified file data as a Uint8Array.
 *
 * ARGUMENTS (from JavaScript):
 *   - `data` (Uint8Array): The raw CSV file bytes
 *   - `filename` (string): The original filename (e.g., "data.csv")
 *   - `params_json` (string): JSON string with rename config
 *     (e.g., '{"columns": {"First Name": "first_name"}}').
 *     Pass '{}' for no renames (passthrough).
 *   - `progress_callback` (Function): Called with (percent: number, message: string)
 *     to report progress. The Web Worker forwards this to the main thread.
 *
 * RETURNS:
 *   A JSON string describing the result:
 *   ```json
 *   {
 *     "file": {
 *       "filename": "data-renamed.csv",
 *       "mimeType": "text/csv",
 *       "size": 1024
 *     },
 *     "metadata": {
 *       "columnsRenamed": 2,
 *       "totalColumns": 5,
 *       "dataRows": 100,
 *       "mapping": {"First Name": "first_name", "Last Name": "last_name"}
 *     }
 *   }
 *   ```
 * @param {Uint8Array} data
 * @param {string} filename
 * @param {string} params_json
 * @param {Function} progress_callback
 * @returns {string}
 */
export function rename_csv_columns(data, filename, params_json, progress_callback) {
    let deferred5_0;
    let deferred5_1;
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_export3);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(filename, wasm.__wbindgen_export3, wasm.__wbindgen_export4);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passStringToWasm0(params_json, wasm.__wbindgen_export3, wasm.__wbindgen_export4);
        const len2 = WASM_VECTOR_LEN;
        wasm.rename_csv_columns(retptr, ptr0, len0, ptr1, len1, ptr2, len2, addHeapObject(progress_callback));
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
        var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
        var ptr4 = r0;
        var len4 = r1;
        if (r3) {
            ptr4 = 0; len4 = 0;
            throw takeObject(r2);
        }
        deferred5_0 = ptr4;
        deferred5_1 = len4;
        return getStringFromWasm0(ptr4, len4);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_export2(deferred5_0, deferred5_1, 1);
    }
}

/**
 * Rename columns in a CSV file and return JUST the modified bytes.
 *
 * This is the efficient path for the Web Worker: returns raw bytes as
 * `Vec<u8>` which wasm-bindgen converts to a `Uint8Array` on the JS side.
 *
 * ARGUMENTS: Same as `rename_csv_columns`.
 *
 * RETURNS: The raw modified CSV bytes as a Uint8Array.
 * @param {Uint8Array} data
 * @param {string} filename
 * @param {string} params_json
 * @param {Function} progress_callback
 * @returns {Uint8Array}
 */
export function rename_csv_columns_bytes(data, filename, params_json, progress_callback) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_export3);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(filename, wasm.__wbindgen_export3, wasm.__wbindgen_export4);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passStringToWasm0(params_json, wasm.__wbindgen_export3, wasm.__wbindgen_export4);
        const len2 = WASM_VECTOR_LEN;
        wasm.rename_csv_columns_bytes(retptr, ptr0, len0, ptr1, len1, ptr2, len2, addHeapObject(progress_callback));
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
        var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
        if (r3) {
            throw takeObject(r2);
        }
        var v4 = getArrayU8FromWasm0(r0, r1).slice();
        wasm.__wbindgen_export2(r0, r1 * 1, 1);
        return v4;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * Rename a single file and return a JSON string with metadata.
 *
 * This is the metadata-returning variant. Use `rename_file_bytes()`
 * to get the actual file data (with the new filename in the metadata).
 *
 * ARGUMENTS (from JavaScript):
 *   - `data` (Uint8Array): The raw file bytes — passed through UNCHANGED
 *   - `filename` (string): The original filename (e.g., "IMG_1234.jpg")
 *   - `params_json` (string): JSON string with rename config
 *     (e.g., '{"prefix": "new-", "case": "lower"}'). Pass '{}' for no changes.
 *   - `progress_callback` (Function): Called with (percent: number, message: string)
 *     to report progress. The Web Worker forwards this to the main thread.
 *
 * RETURNS:
 *   A JSON string describing the result:
 *   ```json
 *   {
 *     "file": {
 *       "filename": "new-img_1234.jpg",
 *       "mimeType": "application/octet-stream",
 *       "size": 51200
 *     },
 *     "metadata": {
 *       "originalFilename": "IMG_1234.jpg",
 *       "newFilename": "new-img_1234.jpg",
 *       "transformsApplied": ["case", "prefix"]
 *     }
 *   }
 *   ```
 *
 * WHY RETURN A JSON STRING?
 * We serialize to a JSON string instead of building a JS object with
 * `js_sys::Object` or `serde-wasm-bindgen`. The reason: `serde-wasm-bindgen`
 * pulls in the ENTIRE `js_sys` binding surface (thousands of symbols),
 * which bloats the WASM binary and causes linker issues in test builds.
 * A JSON string is simple, debuggable, and tiny in code size.
 *
 * RUST CONCEPT: `Result<String, JsValue>`
 * wasm-bindgen functions that can fail return `Result<T, JsValue>`.
 * `Ok(value)` becomes a normal return in JS. `Err(value)` throws a
 * JavaScript Error.
 * @param {Uint8Array} data
 * @param {string} filename
 * @param {string} params_json
 * @param {Function} progress_callback
 * @returns {string}
 */
export function rename_file(data, filename, params_json, progress_callback) {
    let deferred5_0;
    let deferred5_1;
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_export3);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(filename, wasm.__wbindgen_export3, wasm.__wbindgen_export4);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passStringToWasm0(params_json, wasm.__wbindgen_export3, wasm.__wbindgen_export4);
        const len2 = WASM_VECTOR_LEN;
        wasm.rename_file(retptr, ptr0, len0, ptr1, len1, ptr2, len2, addHeapObject(progress_callback));
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
        var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
        var ptr4 = r0;
        var len4 = r1;
        if (r3) {
            ptr4 = 0; len4 = 0;
            throw takeObject(r2);
        }
        deferred5_0 = ptr4;
        deferred5_1 = len4;
        return getStringFromWasm0(ptr4, len4);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_export2(deferred5_0, deferred5_1, 1);
    }
}

/**
 * Rename a single file and return JUST the file bytes.
 *
 * This is the efficient path for the Web Worker: instead of encoding
 * file bytes as JSON (which would double the memory usage), this
 * returns the raw bytes as a `Vec<u8>` which wasm-bindgen converts
 * to a `Uint8Array` on the JS side. Zero-copy via shared memory.
 *
 * For rename-files, the bytes are IDENTICAL to the input (data passes
 * through unchanged). The value of this function is that the Web Worker
 * can pair the bytes with the new filename from `rename_file()`.
 *
 * ARGUMENTS: Same as `rename_file`.
 *
 * RETURNS: The raw file bytes as a Uint8Array (identical to input).
 *
 * The Web Worker uses this to create a Blob for download:
 * ```js
 * const renamed = rename_file_bytes(data, filename, paramsJson, progressCb);
 * const blob = new Blob([renamed], { type: 'application/octet-stream' });
 * ```
 * @param {Uint8Array} data
 * @param {string} filename
 * @param {string} params_json
 * @param {Function} progress_callback
 * @returns {Uint8Array}
 */
export function rename_file_bytes(data, filename, params_json, progress_callback) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_export3);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(filename, wasm.__wbindgen_export3, wasm.__wbindgen_export4);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passStringToWasm0(params_json, wasm.__wbindgen_export3, wasm.__wbindgen_export4);
        const len2 = WASM_VECTOR_LEN;
        wasm.rename_file_bytes(retptr, ptr0, len0, ptr1, len1, ptr2, len2, addHeapObject(progress_callback));
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
        var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
        if (r3) {
            throw takeObject(r2);
        }
        var v4 = getArrayU8FromWasm0(r0, r1).slice();
        wasm.__wbindgen_export2(r0, r1 * 1, 1);
        return v4;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * Resize a single image and return a JSON string with metadata.
 *
 * ARGUMENTS (from JavaScript):
 *   - `data` (Uint8Array): The raw image file bytes
 *   - `filename` (string): The original filename (e.g., "photo.jpg")
 *   - `params_json` (string): JSON string with resize config:
 *     ```json
 *     {
 *       "width": 800,                // Target width in pixels
 *       "height": 600,               // Target height (optional if maintainAspect)
 *       "maintainAspect": true,      // Preserve aspect ratio (default: true)
 *       "quality": 80                // JPEG quality 1-100 (default: 80)
 *     }
 *     ```
 *   - `progress_callback` (Function): Called with (percent, message)
 *
 * RETURNS:
 *   A JSON string describing the result:
 *   ```json
 *   {
 *     "file": {
 *       "filename": "photo-resized.jpg",
 *       "mimeType": "image/jpeg",
 *       "size": 25600
 *     },
 *     "metadata": {
 *       "originalWidth": 1200,
 *       "originalHeight": 800,
 *       "newWidth": 800,
 *       "newHeight": 533,
 *       "originalSize": 102400,
 *       "newSize": 25600,
 *       "format": "Jpeg"
 *     }
 *   }
 *   ```
 * @param {Uint8Array} data
 * @param {string} filename
 * @param {string} params_json
 * @param {Function} progress_callback
 * @returns {string}
 */
export function resize_image(data, filename, params_json, progress_callback) {
    let deferred5_0;
    let deferred5_1;
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_export3);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(filename, wasm.__wbindgen_export3, wasm.__wbindgen_export4);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passStringToWasm0(params_json, wasm.__wbindgen_export3, wasm.__wbindgen_export4);
        const len2 = WASM_VECTOR_LEN;
        wasm.resize_image(retptr, ptr0, len0, ptr1, len1, ptr2, len2, addHeapObject(progress_callback));
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
        var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
        var ptr4 = r0;
        var len4 = r1;
        if (r3) {
            ptr4 = 0; len4 = 0;
            throw takeObject(r2);
        }
        deferred5_0 = ptr4;
        deferred5_1 = len4;
        return getStringFromWasm0(ptr4, len4);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_export2(deferred5_0, deferred5_1, 1);
    }
}

/**
 * Resize a single image and return JUST the resized bytes.
 *
 * Efficient path for the Web Worker — raw bytes returned as Uint8Array
 * via wasm-bindgen's zero-copy memory sharing.
 *
 * ARGUMENTS: Same as `resize_image`.
 * RETURNS: The raw resized image bytes as a Uint8Array.
 * @param {Uint8Array} data
 * @param {string} filename
 * @param {string} params_json
 * @param {Function} progress_callback
 * @returns {Uint8Array}
 */
export function resize_image_bytes(data, filename, params_json, progress_callback) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_export3);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(filename, wasm.__wbindgen_export3, wasm.__wbindgen_export4);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passStringToWasm0(params_json, wasm.__wbindgen_export3, wasm.__wbindgen_export4);
        const len2 = WASM_VECTOR_LEN;
        wasm.resize_image_bytes(retptr, ptr0, len0, ptr1, len1, ptr2, len2, addHeapObject(progress_callback));
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
        var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
        if (r3) {
            throw takeObject(r2);
        }
        var v4 = getArrayU8FromWasm0(r0, r1).slice();
        wasm.__wbindgen_export2(r0, r1 * 1, 1);
        return v4;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * Initialize the WASM module. Call this ONCE when the Web Worker starts,
 * before calling any processing functions.
 *
 * WHAT IT DOES:
 * Installs a "panic hook" so when Rust code crashes, the browser console
 * shows the real error message instead of the useless "unreachable" error.
 *
 * SAFE TO CALL MULTIPLE TIMES — set_once() is idempotent.
 *
 * USAGE FROM WEB WORKER:
 * ```js
 * import init, { setup } from './bnto_wasm.js';
 * await init('/wasm/bnto_wasm_bg.wasm');
 * setup();  // Call once, then process files
 * ```
 */
export function setup() {
    wasm.setup();
}

/**
 * Returns the version of the Bnto WASM engine.
 *
 * Useful for the web app to verify the correct WASM version is loaded
 * and for debugging ("which engine version is this?").
 *
 * RUST CONCEPT: `env!("CARGO_PKG_VERSION")`
 * This macro reads the `version` field from THIS crate's Cargo.toml
 * at compile time. So if Cargo.toml says version = "0.1.0", this
 * function returns "0.1.0". It's baked into the binary — no runtime
 * config file needed.
 * @returns {string}
 */
export function version() {
    let deferred1_0;
    let deferred1_1;
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.version(retptr);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        deferred1_0 = r0;
        deferred1_1 = r1;
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_export2(deferred1_0, deferred1_1, 1);
    }
}

function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg_Error_4577686b3a6d9b3a: function(arg0, arg1) {
            const ret = Error(getStringFromWasm0(arg0, arg1));
            return addHeapObject(ret);
        },
        __wbg___wbindgen_throw_39bc967c0e5a9b58: function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
        __wbg_call_c974f0bf2231552e: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            const ret = getObject(arg0).call(getObject(arg1), getObject(arg2), getObject(arg3));
            return addHeapObject(ret);
        }, arguments); },
        __wbg_error_a6fa202b58aa1cd3: function(arg0, arg1) {
            let deferred0_0;
            let deferred0_1;
            try {
                deferred0_0 = arg0;
                deferred0_1 = arg1;
                console.error(getStringFromWasm0(arg0, arg1));
            } finally {
                wasm.__wbindgen_export2(deferred0_0, deferred0_1, 1);
            }
        },
        __wbg_getDate_36b92ebcc42b5265: function(arg0) {
            const ret = getObject(arg0).getDate();
            return ret;
        },
        __wbg_getFullYear_9c15c32a31fb7eb8: function(arg0) {
            const ret = getObject(arg0).getFullYear();
            return ret;
        },
        __wbg_getMonth_dc1d8154ce70029d: function(arg0) {
            const ret = getObject(arg0).getMonth();
            return ret;
        },
        __wbg_new_0_a719938e6f92ddf4: function() {
            const ret = new Date();
            return addHeapObject(ret);
        },
        __wbg_new_227d7c05414eb861: function() {
            const ret = new Error();
            return addHeapObject(ret);
        },
        __wbg_stack_3b0d974bbf31e44f: function(arg0, arg1) {
            const ret = getObject(arg1).stack;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export3, wasm.__wbindgen_export4);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbindgen_cast_0000000000000001: function(arg0) {
            // Cast intrinsic for `F64 -> Externref`.
            const ret = arg0;
            return addHeapObject(ret);
        },
        __wbindgen_cast_0000000000000002: function(arg0, arg1) {
            // Cast intrinsic for `Ref(String) -> Externref`.
            const ret = getStringFromWasm0(arg0, arg1);
            return addHeapObject(ret);
        },
        __wbindgen_object_drop_ref: function(arg0) {
            takeObject(arg0);
        },
    };
    return {
        __proto__: null,
        "./bnto_wasm_bg.js": import0,
    };
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function dropObject(idx) {
    if (idx < 1028) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(ptr, len);
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function getObject(idx) { return heap[idx]; }

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        wasm.__wbindgen_export(addHeapObject(e));
    }
}

let heap = new Array(1024).fill(undefined);
heap.push(undefined, null, true, false);

let heap_next = heap.length;

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1, 1) >>> 0;
    getUint8ArrayMemory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function passStringToWasm0(arg, malloc, realloc) {
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }
    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    };
}

let WASM_VECTOR_LEN = 0;

let wasmModule, wasm;
function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    wasmModule = module;
    cachedDataViewMemory0 = null;
    cachedUint8ArrayMemory0 = null;
    return wasm;
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);
            } catch (e) {
                const validResponse = module.ok && expectedResponseType(module.type);

                if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else { throw e; }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);
    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };
        } else {
            return instance;
        }
    }

    function expectedResponseType(type) {
        switch (type) {
            case 'basic': case 'cors': case 'default': return true;
        }
        return false;
    }
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (module !== undefined) {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();
    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }
    const instance = new WebAssembly.Instance(module, imports);
    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (module_or_path !== undefined) {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (module_or_path === undefined) {
        module_or_path = new URL('bnto_wasm_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync, __wbg_init as default };
