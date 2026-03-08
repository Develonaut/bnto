/* @ts-self-types="./bnto_wasm.d.ts" */

/**
 * Clean a CSV file and return BOTH metadata and bytes in one call.
 *
 * The CSV is processed exactly ONCE, and the result contains everything
 * the Web Worker needs — no double processing.
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
 *   A JavaScript object with four properties:
 *   ```js
 *   {
 *     metadata: '{"originalRows":100,"cleanedRows":85,...}',  // JSON string
 *     data: Uint8Array([...]),                                // raw cleaned CSV bytes
 *     filename: "data-cleaned.csv",                           // output filename
 *     mimeType: "text/csv"                                    // MIME type
 *   }
 *   ```
 *
 * RUST CONCEPT: `Result<JsValue, JsValue>`
 * wasm-bindgen functions that can fail return `Result<T, JsValue>`.
 * `Ok(value)` becomes a normal return in JS. `Err(value)` throws a
 * JavaScript Error.
 * @param {Uint8Array} data
 * @param {string} filename
 * @param {string} params_json
 * @param {Function} progress_callback
 * @returns {any}
 */
export function clean_csv_combined(data, filename, params_json, progress_callback) {
  try {
    const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
    const ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_export);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(filename, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passStringToWasm0(params_json, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len2 = WASM_VECTOR_LEN;
    wasm.clean_csv_combined(
      retptr,
      ptr0,
      len0,
      ptr1,
      len1,
      ptr2,
      len2,
      addHeapObject(progress_callback),
    );
    var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
    var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
    var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
    if (r2) {
      throw takeObject(r1);
    }
    return takeObject(r0);
  } finally {
    wasm.__wbindgen_add_to_stack_pointer(16);
  }
}

/**
 * Compress a single image and return BOTH metadata and bytes in one call.
 *
 * The image is processed exactly ONCE, and the result contains everything
 * the Web Worker needs — no double processing.
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
 *   A JavaScript object with four properties:
 *   ```js
 *   {
 *     metadata: '{"originalSize":102400,"compressedSize":51200,...}',  // JSON string
 *     data: Uint8Array([...]),                       // raw compressed bytes
 *     filename: "photo-compressed.jpg",              // output filename
 *     mimeType: "image/jpeg"                         // MIME type
 *   }
 *   ```
 *
 * RUST CONCEPT: `Result<JsValue, JsValue>`
 * wasm-bindgen functions that can fail return `Result<T, JsValue>`.
 * `Ok(value)` becomes a normal return in JS. `Err(value)` throws a
 * JavaScript Error.
 * @param {Uint8Array} data
 * @param {string} filename
 * @param {string} params_json
 * @param {Function} progress_callback
 * @returns {any}
 */
export function compress_image_combined(data, filename, params_json, progress_callback) {
  try {
    const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
    const ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_export);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(filename, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passStringToWasm0(params_json, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len2 = WASM_VECTOR_LEN;
    wasm.compress_image_combined(
      retptr,
      ptr0,
      len0,
      ptr1,
      len1,
      ptr2,
      len2,
      addHeapObject(progress_callback),
    );
    var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
    var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
    var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
    if (r2) {
      throw takeObject(r1);
    }
    return takeObject(r0);
  } finally {
    wasm.__wbindgen_add_to_stack_pointer(16);
  }
}

/**
 * Convert a single image to a different format and return BOTH metadata and
 * bytes in one call.
 *
 * The image is processed exactly ONCE, and the result contains everything
 * the Web Worker needs — no double processing.
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
 *   A JavaScript object with four properties:
 *   ```js
 *   {
 *     metadata: '{"originalSize":102400,"compressedSize":51200,...}',  // JSON string
 *     data: Uint8Array([...]),                       // raw converted bytes
 *     filename: "photo.png",                         // output filename
 *     mimeType: "image/png"                          // MIME type
 *   }
 *   ```
 * @param {Uint8Array} data
 * @param {string} filename
 * @param {string} params_json
 * @param {Function} progress_callback
 * @returns {any}
 */
export function convert_image_format_combined(data, filename, params_json, progress_callback) {
  try {
    const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
    const ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_export);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(filename, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passStringToWasm0(params_json, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len2 = WASM_VECTOR_LEN;
    wasm.convert_image_format_combined(
      retptr,
      ptr0,
      len0,
      ptr1,
      len1,
      ptr2,
      len2,
      addHeapObject(progress_callback),
    );
    var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
    var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
    var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
    if (r2) {
      throw takeObject(r1);
    }
    return takeObject(r0);
  } finally {
    wasm.__wbindgen_add_to_stack_pointer(16);
  }
}

/**
 * Execute a complete pipeline in WASM.
 *
 * This is the main entry point for the browser. The Web Worker calls this
 * with a JSON definition string, an array of file objects, and a progress
 * callback. The function returns a JSON string with the results.
 *
 * # Arguments
 * - `definition_json` — JSON string of the pipeline definition
 * - `files_js` — JavaScript array of `{name: string, data: Uint8Array, mimeType: string}`
 * - `progress_callback` — JavaScript function that receives event JSON strings
 *
 * # Returns
 * A JSON string containing the pipeline results:
 * ```json
 * {
 *   "files": [{ "name": "...", "data": [bytes], "mimeType": "..." }],
 *   "durationMs": 1234
 * }
 * ```
 *
 * # Errors
 * Returns a JsValue error string if:
 * - The definition JSON is invalid
 * - A node type isn't registered
 * - A processor fails on a file
 * @param {string} definition_json
 * @param {any} files_js
 * @param {Function} progress_callback
 * @returns {any}
 */
export function execute_pipeline(definition_json, files_js, progress_callback) {
  try {
    const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
    const ptr0 = passStringToWasm0(
      definition_json,
      wasm.__wbindgen_export,
      wasm.__wbindgen_export2,
    );
    const len0 = WASM_VECTOR_LEN;
    wasm.execute_pipeline(
      retptr,
      ptr0,
      len0,
      addHeapObject(files_js),
      addHeapObject(progress_callback),
    );
    var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
    var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
    var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
    if (r2) {
      throw takeObject(r1);
    }
    return takeObject(r0);
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
    const ptr0 = passStringToWasm0(name, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    wasm.greet(retptr, ptr0, len0);
    var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
    var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
    deferred2_0 = r0;
    deferred2_1 = r1;
    return getStringFromWasm0(r0, r1);
  } finally {
    wasm.__wbindgen_add_to_stack_pointer(16);
    wasm.__wbindgen_export4(deferred2_0, deferred2_1, 1);
  }
}

/**
 * Return a JSON string describing every registered node processor.
 *
 * This is the engine's self-describing catalog — the single source of truth
 * for what nodes the engine supports, what parameters they accept, what file
 * types they handle, and whether they run in the browser.
 *
 * The output is a JSON object with two fields:
 *   - `version` (string) — the format version (e.g., "1.0.0")
 *   - `processors` (array) — one entry per registered processor
 *
 * Each processor entry includes: nodeType, operation, name, description,
 * category, accepts (MIME types), browserCapable, and parameters.
 *
 * # Returns
 * A JSON string (pretty-printed for readability).
 *
 * # Errors
 * Returns a JsValue error if JSON serialization fails (shouldn't happen
 * with well-formed metadata, but we handle it gracefully).
 * @returns {string}
 */
export function node_catalog() {
  let deferred2_0;
  let deferred2_1;
  try {
    const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
    wasm.node_catalog(retptr);
    var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
    var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
    var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
    var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
    var ptr1 = r0;
    var len1 = r1;
    if (r3) {
      ptr1 = 0;
      len1 = 0;
      throw takeObject(r2);
    }
    deferred2_0 = ptr1;
    deferred2_1 = len1;
    return getStringFromWasm0(ptr1, len1);
  } finally {
    wasm.__wbindgen_add_to_stack_pointer(16);
    wasm.__wbindgen_export4(deferred2_0, deferred2_1, 1);
  }
}

/**
 * Rename columns in a CSV file and return BOTH metadata and bytes in one call.
 *
 * The CSV is processed exactly ONCE, and the result contains everything
 * the Web Worker needs — no double processing.
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
 *   A JavaScript object with four properties:
 *   ```js
 *   {
 *     metadata: '{"columnsRenamed":2,"totalColumns":5,...}',  // JSON string
 *     data: Uint8Array([...]),                                // raw modified CSV bytes
 *     filename: "data-renamed.csv",                           // output filename
 *     mimeType: "text/csv"                                    // MIME type
 *   }
 *   ```
 * @param {Uint8Array} data
 * @param {string} filename
 * @param {string} params_json
 * @param {Function} progress_callback
 * @returns {any}
 */
export function rename_csv_columns_combined(data, filename, params_json, progress_callback) {
  try {
    const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
    const ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_export);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(filename, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passStringToWasm0(params_json, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len2 = WASM_VECTOR_LEN;
    wasm.rename_csv_columns_combined(
      retptr,
      ptr0,
      len0,
      ptr1,
      len1,
      ptr2,
      len2,
      addHeapObject(progress_callback),
    );
    var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
    var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
    var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
    if (r2) {
      throw takeObject(r1);
    }
    return takeObject(r0);
  } finally {
    wasm.__wbindgen_add_to_stack_pointer(16);
  }
}

/**
 * Rename a single file and return BOTH metadata and bytes in one call.
 *
 * The file is processed exactly ONCE, and the result contains everything
 * the Web Worker needs — no double processing.
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
 *   A JavaScript object with four properties:
 *   ```js
 *   {
 *     metadata: '{"originalFilename":"IMG_1234.jpg",...}',  // JSON string
 *     data: Uint8Array([...]),                              // raw file bytes (unchanged)
 *     filename: "vacation-1234.jpg",                        // new filename
 *     mimeType: "application/octet-stream"                  // MIME type
 *   }
 *   ```
 *
 * RUST CONCEPT: `Result<JsValue, JsValue>`
 * wasm-bindgen functions that can fail return `Result<T, JsValue>`.
 * `Ok(value)` becomes a normal return in JS. `Err(value)` throws a
 * JavaScript Error.
 * @param {Uint8Array} data
 * @param {string} filename
 * @param {string} params_json
 * @param {Function} progress_callback
 * @returns {any}
 */
export function rename_file_combined(data, filename, params_json, progress_callback) {
  try {
    const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
    const ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_export);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(filename, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passStringToWasm0(params_json, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len2 = WASM_VECTOR_LEN;
    wasm.rename_file_combined(
      retptr,
      ptr0,
      len0,
      ptr1,
      len1,
      ptr2,
      len2,
      addHeapObject(progress_callback),
    );
    var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
    var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
    var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
    if (r2) {
      throw takeObject(r1);
    }
    return takeObject(r0);
  } finally {
    wasm.__wbindgen_add_to_stack_pointer(16);
  }
}

/**
 * Resize a single image and return BOTH metadata and bytes in one call.
 *
 * The image is processed exactly ONCE, and the result contains everything
 * the Web Worker needs — no double processing.
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
 *   A JavaScript object with four properties:
 *   ```js
 *   {
 *     metadata: '{"originalSize":102400,"compressedSize":51200,...}',  // JSON string
 *     data: Uint8Array([...]),                       // raw resized bytes
 *     filename: "photo-resized.jpg",                 // output filename
 *     mimeType: "image/jpeg"                         // MIME type
 *   }
 *   ```
 * @param {Uint8Array} data
 * @param {string} filename
 * @param {string} params_json
 * @param {Function} progress_callback
 * @returns {any}
 */
export function resize_image_combined(data, filename, params_json, progress_callback) {
  try {
    const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
    const ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_export);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(filename, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passStringToWasm0(params_json, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len2 = WASM_VECTOR_LEN;
    wasm.resize_image_combined(
      retptr,
      ptr0,
      len0,
      ptr1,
      len1,
      ptr2,
      len2,
      addHeapObject(progress_callback),
    );
    var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
    var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
    var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
    if (r2) {
      throw takeObject(r1);
    }
    return takeObject(r0);
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
    wasm.__wbindgen_export4(deferred1_0, deferred1_1, 1);
  }
}

function __wbg_get_imports() {
  const import0 = {
    __proto__: null,
    __wbg_Error_4577686b3a6d9b3a: function (arg0, arg1) {
      const ret = Error(getStringFromWasm0(arg0, arg1));
      return addHeapObject(ret);
    },
    __wbg___wbindgen_string_get_3e5751597f39a112: function (arg0, arg1) {
      const obj = getObject(arg1);
      const ret = typeof obj === "string" ? obj : undefined;
      var ptr1 = isLikeNone(ret)
        ? 0
        : passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
      var len1 = WASM_VECTOR_LEN;
      getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
      getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    },
    __wbg___wbindgen_throw_39bc967c0e5a9b58: function (arg0, arg1) {
      throw new Error(getStringFromWasm0(arg0, arg1));
    },
    __wbg_call_08ad0d89caa7cb79: function () {
      return handleError(function (arg0, arg1, arg2) {
        const ret = getObject(arg0).call(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
      }, arguments);
    },
    __wbg_call_c974f0bf2231552e: function () {
      return handleError(function (arg0, arg1, arg2, arg3) {
        const ret = getObject(arg0).call(getObject(arg1), getObject(arg2), getObject(arg3));
        return addHeapObject(ret);
      }, arguments);
    },
    __wbg_error_a6fa202b58aa1cd3: function (arg0, arg1) {
      let deferred0_0;
      let deferred0_1;
      try {
        deferred0_0 = arg0;
        deferred0_1 = arg1;
        console.error(getStringFromWasm0(arg0, arg1));
      } finally {
        wasm.__wbindgen_export4(deferred0_0, deferred0_1, 1);
      }
    },
    __wbg_from_d7e888a2e9063b32: function (arg0) {
      const ret = Array.from(getObject(arg0));
      return addHeapObject(ret);
    },
    __wbg_getDate_36b92ebcc42b5265: function (arg0) {
      const ret = getObject(arg0).getDate();
      return ret;
    },
    __wbg_getFullYear_9c15c32a31fb7eb8: function (arg0) {
      const ret = getObject(arg0).getFullYear();
      return ret;
    },
    __wbg_getMonth_dc1d8154ce70029d: function (arg0) {
      const ret = getObject(arg0).getMonth();
      return ret;
    },
    __wbg_get_18349afdb36339a9: function () {
      return handleError(function (arg0, arg1) {
        const ret = Reflect.get(getObject(arg0), getObject(arg1));
        return addHeapObject(ret);
      }, arguments);
    },
    __wbg_get_f09c3a16f8848381: function (arg0, arg1) {
      const ret = getObject(arg0)[arg1 >>> 0];
      return addHeapObject(ret);
    },
    __wbg_length_5855c1f289dfffc1: function (arg0) {
      const ret = getObject(arg0).length;
      return ret;
    },
    __wbg_length_a31e05262e09b7f8: function (arg0) {
      const ret = getObject(arg0).length;
      return ret;
    },
    __wbg_new_09959f7b4c92c246: function (arg0) {
      const ret = new Uint8Array(getObject(arg0));
      return addHeapObject(ret);
    },
    __wbg_new_0_a719938e6f92ddf4: function () {
      const ret = new Date();
      return addHeapObject(ret);
    },
    __wbg_new_227d7c05414eb861: function () {
      const ret = new Error();
      return addHeapObject(ret);
    },
    __wbg_new_cbee8c0d5c479eac: function () {
      const ret = new Array();
      return addHeapObject(ret);
    },
    __wbg_new_ed69e637b553a997: function () {
      const ret = new Object();
      return addHeapObject(ret);
    },
    __wbg_new_from_slice_d7e202fdbee3c396: function (arg0, arg1) {
      const ret = new Uint8Array(getArrayU8FromWasm0(arg0, arg1));
      return addHeapObject(ret);
    },
    __wbg_now_edd718b3004d8631: function () {
      const ret = Date.now();
      return ret;
    },
    __wbg_prototypesetcall_f034d444741426c3: function (arg0, arg1, arg2) {
      Uint8Array.prototype.set.call(getArrayU8FromWasm0(arg0, arg1), getObject(arg2));
    },
    __wbg_push_a6f9488ffd3fae3b: function (arg0, arg1) {
      const ret = getObject(arg0).push(getObject(arg1));
      return ret;
    },
    __wbg_set_bad5c505cc70b5f8: function () {
      return handleError(function (arg0, arg1, arg2) {
        const ret = Reflect.set(getObject(arg0), getObject(arg1), getObject(arg2));
        return ret;
      }, arguments);
    },
    __wbg_stack_3b0d974bbf31e44f: function (arg0, arg1) {
      const ret = getObject(arg1).stack;
      const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
      const len1 = WASM_VECTOR_LEN;
      getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
      getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    },
    __wbindgen_cast_0000000000000001: function (arg0) {
      // Cast intrinsic for `F64 -> Externref`.
      const ret = arg0;
      return addHeapObject(ret);
    },
    __wbindgen_cast_0000000000000002: function (arg0, arg1) {
      // Cast intrinsic for `Ref(String) -> Externref`.
      const ret = getStringFromWasm0(arg0, arg1);
      return addHeapObject(ret);
    },
    __wbindgen_object_drop_ref: function (arg0) {
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
  if (
    cachedDataViewMemory0 === null ||
    cachedDataViewMemory0.buffer.detached === true ||
    (cachedDataViewMemory0.buffer.detached === undefined &&
      cachedDataViewMemory0.buffer !== wasm.memory.buffer)
  ) {
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

function getObject(idx) {
  return heap[idx];
}

function handleError(f, args) {
  try {
    return f.apply(this, args);
  } catch (e) {
    wasm.__wbindgen_export3(addHeapObject(e));
  }
}

let heap = new Array(1024).fill(undefined);
heap.push(undefined, null, true, false);

let heap_next = heap.length;

function isLikeNone(x) {
  return x === undefined || x === null;
}

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
    getUint8ArrayMemory0()
      .subarray(ptr, ptr + buf.length)
      .set(buf);
    WASM_VECTOR_LEN = buf.length;
    return ptr;
  }

  let len = arg.length;
  let ptr = malloc(len, 1) >>> 0;

  const mem = getUint8ArrayMemory0();

  let offset = 0;

  for (; offset < len; offset++) {
    const code = arg.charCodeAt(offset);
    if (code > 0x7f) break;
    mem[ptr + offset] = code;
  }
  if (offset !== len) {
    if (offset !== 0) {
      arg = arg.slice(offset);
    }
    ptr = realloc(ptr, len, (len = offset + arg.length * 3), 1) >>> 0;
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

let cachedTextDecoder = new TextDecoder("utf-8", { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
  numBytesDecoded += len;
  if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
    cachedTextDecoder = new TextDecoder("utf-8", { ignoreBOM: true, fatal: true });
    cachedTextDecoder.decode();
    numBytesDecoded = len;
  }
  return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

const cachedTextEncoder = new TextEncoder();

if (!("encodeInto" in cachedTextEncoder)) {
  cachedTextEncoder.encodeInto = function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
      read: arg.length,
      written: buf.length,
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
  if (typeof Response === "function" && module instanceof Response) {
    if (typeof WebAssembly.instantiateStreaming === "function") {
      try {
        return await WebAssembly.instantiateStreaming(module, imports);
      } catch (e) {
        const validResponse = module.ok && expectedResponseType(module.type);

        if (validResponse && module.headers.get("Content-Type") !== "application/wasm") {
          console.warn(
            "`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",
            e,
          );
        } else {
          throw e;
        }
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
      case "basic":
      case "cors":
      case "default":
        return true;
    }
    return false;
  }
}

function initSync(module) {
  if (wasm !== undefined) return wasm;

  if (module !== undefined) {
    if (Object.getPrototypeOf(module) === Object.prototype) {
      ({ module } = module);
    } else {
      console.warn("using deprecated parameters for `initSync()`; pass a single object instead");
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
      ({ module_or_path } = module_or_path);
    } else {
      console.warn(
        "using deprecated parameters for the initialization function; pass a single object instead",
      );
    }
  }

  if (module_or_path === undefined) {
    module_or_path = new URL("bnto_wasm_bg.wasm", import.meta.url);
  }
  const imports = __wbg_get_imports();

  if (
    typeof module_or_path === "string" ||
    (typeof Request === "function" && module_or_path instanceof Request) ||
    (typeof URL === "function" && module_or_path instanceof URL)
  ) {
    module_or_path = fetch(module_or_path);
  }

  const { instance, module } = await __wbg_load(await module_or_path, imports);

  return __wbg_finalize_init(instance, module);
}

export { initSync, __wbg_init as default };
