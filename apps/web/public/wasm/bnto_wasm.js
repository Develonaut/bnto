/* @ts-self-types="./bnto_wasm.d.ts" */

//#region exports

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
 * @param {Uint8Array} data
 * @param {string} filename
 * @param {string} params_json
 * @param {Function} progress_callback
 * @returns {any}
 */
export function clean_spreadsheet_combined(data, filename, params_json, progress_callback) {
    const ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(filename, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passStringToWasm0(params_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len2 = WASM_VECTOR_LEN;
    const ret = wasm.clean_spreadsheet_combined(ptr0, len0, ptr1, len1, ptr2, len2, progress_callback);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
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
 * @param {Uint8Array} data
 * @param {string} filename
 * @param {string} params_json
 * @param {Function} progress_callback
 * @returns {any}
 */
export function compress_image_combined(data, filename, params_json, progress_callback) {
    const ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(filename, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passStringToWasm0(params_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len2 = WASM_VECTOR_LEN;
    const ret = wasm.compress_image_combined(ptr0, len0, ptr1, len1, ptr2, len2, progress_callback);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
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
    const ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(filename, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passStringToWasm0(params_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len2 = WASM_VECTOR_LEN;
    const ret = wasm.convert_image_format_combined(ptr0, len0, ptr1, len1, ptr2, len2, progress_callback);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
}

/**
 * Execute a complete pipeline in WASM. Main entry point for the browser.
 *
 * Takes a JSON definition string, a JS array of file objects, and a progress
 * callback. Returns a JS object with `files` array and `durationMs`.
 * @param {string} definition_json
 * @param {any} files_js
 * @param {Function} progress_callback
 * @returns {any}
 */
export function execute_pipeline(definition_json, files_js, progress_callback) {
    const ptr0 = passStringToWasm0(definition_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.execute_pipeline(ptr0, len0, files_js, progress_callback);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
}

/**
 * Extract metadata from a file and return enriched output.
 * @param {Uint8Array} data
 * @param {string} filename
 * @param {string} params_json
 * @param {Function} progress_callback
 * @returns {any}
 */
export function file_metadata_combined(data, filename, params_json, progress_callback) {
    const ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(filename, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passStringToWasm0(params_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len2 = WASM_VECTOR_LEN;
    const ret = wasm.file_metadata_combined(ptr0, len0, ptr1, len1, ptr2, len2, progress_callback);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
}

/**
 * Filter a single file — returns the file if it matches, or null if dropped.
 *
 * File-filter returns empty output.files when the file doesn't match.
 * In that case, this function returns JsValue::NULL so the JS side
 * knows to skip this file.
 * @param {Uint8Array} data
 * @param {string} filename
 * @param {string} params_json
 * @param {Function} progress_callback
 * @returns {any}
 */
export function filter_file_combined(data, filename, params_json, progress_callback) {
    const ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(filename, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passStringToWasm0(params_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len2 = WASM_VECTOR_LEN;
    const ret = wasm.filter_file_combined(ptr0, len0, ptr1, len1, ptr2, len2, progress_callback);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
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
        const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.greet(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
    }
}

/**
 * Return a pretty-printed JSON string of the engine's full catalog.
 * @returns {string}
 */
export function node_catalog() {
    let deferred2_0;
    let deferred2_1;
    try {
        const ret = wasm.node_catalog();
        var ptr1 = ret[0];
        var len1 = ret[1];
        if (ret[3]) {
            ptr1 = 0; len1 = 0;
            throw takeFromExternrefTable0(ret[2]);
        }
        deferred2_0 = ptr1;
        deferred2_1 = len1;
        return getStringFromWasm0(ptr1, len1);
    } finally {
        wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
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
 * @param {Uint8Array} data
 * @param {string} filename
 * @param {string} params_json
 * @param {Function} progress_callback
 * @returns {any}
 */
export function rename_file_combined(data, filename, params_json, progress_callback) {
    const ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(filename, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passStringToWasm0(params_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len2 = WASM_VECTOR_LEN;
    const ret = wasm.rename_file_combined(ptr0, len0, ptr1, len1, ptr2, len2, progress_callback);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
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
export function rename_spreadsheet_columns_combined(data, filename, params_json, progress_callback) {
    const ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(filename, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passStringToWasm0(params_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len2 = WASM_VECTOR_LEN;
    const ret = wasm.rename_spreadsheet_columns_combined(ptr0, len0, ptr1, len1, ptr2, len2, progress_callback);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
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
    const ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(filename, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passStringToWasm0(params_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len2 = WASM_VECTOR_LEN;
    const ret = wasm.resize_image_combined(ptr0, len0, ptr1, len1, ptr2, len2, progress_callback);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
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
 * @returns {string}
 */
export function version() {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.version();
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}

//#endregion

//#region wasm imports

function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg_Error_4577686b3a6d9b3a: function() { return logError(function (arg0, arg1) {
            const ret = Error(getStringFromWasm0(arg0, arg1));
            return ret;
        }, arguments); },
        __wbg___wbindgen_string_get_3e5751597f39a112: function(arg0, arg1) {
            const obj = arg1;
            const ret = typeof(obj) === 'string' ? obj : undefined;
            var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg___wbindgen_throw_39bc967c0e5a9b58: function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
        __wbg_call_08ad0d89caa7cb79: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = arg0.call(arg1, arg2);
            return ret;
        }, arguments); },
        __wbg_call_c974f0bf2231552e: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            const ret = arg0.call(arg1, arg2, arg3);
            return ret;
        }, arguments); },
        __wbg_error_a6fa202b58aa1cd3: function() { return logError(function (arg0, arg1) {
            let deferred0_0;
            let deferred0_1;
            try {
                deferred0_0 = arg0;
                deferred0_1 = arg1;
                console.error(getStringFromWasm0(arg0, arg1));
            } finally {
                wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
            }
        }, arguments); },
        __wbg_from_d7e888a2e9063b32: function() { return logError(function (arg0) {
            const ret = Array.from(arg0);
            return ret;
        }, arguments); },
        __wbg_getDate_36b92ebcc42b5265: function() { return logError(function (arg0) {
            const ret = arg0.getDate();
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_getFullYear_9c15c32a31fb7eb8: function() { return logError(function (arg0) {
            const ret = arg0.getFullYear();
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_getMonth_dc1d8154ce70029d: function() { return logError(function (arg0) {
            const ret = arg0.getMonth();
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_get_18349afdb36339a9: function() { return handleError(function (arg0, arg1) {
            const ret = Reflect.get(arg0, arg1);
            return ret;
        }, arguments); },
        __wbg_get_f09c3a16f8848381: function() { return logError(function (arg0, arg1) {
            const ret = arg0[arg1 >>> 0];
            return ret;
        }, arguments); },
        __wbg_length_5855c1f289dfffc1: function() { return logError(function (arg0) {
            const ret = arg0.length;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_length_a31e05262e09b7f8: function() { return logError(function (arg0) {
            const ret = arg0.length;
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_new_09959f7b4c92c246: function() { return logError(function (arg0) {
            const ret = new Uint8Array(arg0);
            return ret;
        }, arguments); },
        __wbg_new_0_a719938e6f92ddf4: function() { return logError(function () {
            const ret = new Date();
            return ret;
        }, arguments); },
        __wbg_new_227d7c05414eb861: function() { return logError(function () {
            const ret = new Error();
            return ret;
        }, arguments); },
        __wbg_new_cbee8c0d5c479eac: function() { return logError(function () {
            const ret = new Array();
            return ret;
        }, arguments); },
        __wbg_new_ed69e637b553a997: function() { return logError(function () {
            const ret = new Object();
            return ret;
        }, arguments); },
        __wbg_new_from_slice_d7e202fdbee3c396: function() { return logError(function (arg0, arg1) {
            const ret = new Uint8Array(getArrayU8FromWasm0(arg0, arg1));
            return ret;
        }, arguments); },
        __wbg_now_edd718b3004d8631: function() { return logError(function () {
            const ret = Date.now();
            return ret;
        }, arguments); },
        __wbg_prototypesetcall_f034d444741426c3: function() { return logError(function (arg0, arg1, arg2) {
            Uint8Array.prototype.set.call(getArrayU8FromWasm0(arg0, arg1), arg2);
        }, arguments); },
        __wbg_push_a6f9488ffd3fae3b: function() { return logError(function (arg0, arg1) {
            const ret = arg0.push(arg1);
            _assertNum(ret);
            return ret;
        }, arguments); },
        __wbg_set_bad5c505cc70b5f8: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = Reflect.set(arg0, arg1, arg2);
            _assertBoolean(ret);
            return ret;
        }, arguments); },
        __wbg_stack_3b0d974bbf31e44f: function() { return logError(function (arg0, arg1) {
            const ret = arg1.stack;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbindgen_cast_0000000000000001: function() { return logError(function (arg0) {
            // Cast intrinsic for `F64 -> Externref`.
            const ret = arg0;
            return ret;
        }, arguments); },
        __wbindgen_cast_0000000000000002: function() { return logError(function (arg0, arg1) {
            // Cast intrinsic for `Ref(String) -> Externref`.
            const ret = getStringFromWasm0(arg0, arg1);
            return ret;
        }, arguments); },
        __wbindgen_init_externref_table: function() {
            const table = wasm.__wbindgen_externrefs;
            const offset = table.grow(4);
            table.set(0, undefined);
            table.set(offset + 0, undefined);
            table.set(offset + 1, null);
            table.set(offset + 2, true);
            table.set(offset + 3, false);
        },
    };
    return {
        __proto__: null,
        "./bnto_wasm_bg.js": import0,
    };
}


//#endregion

//#region intrinsics
function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_externrefs.set(idx, obj);
    return idx;
}

function _assertBoolean(n) {
    if (typeof(n) !== 'boolean') {
        throw new Error(`expected a boolean argument, found ${typeof(n)}`);
    }
}

function _assertNum(n) {
    if (typeof(n) !== 'number') throw new Error(`expected a number argument, found ${typeof(n)}`);
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

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        const idx = addToExternrefTable0(e);
        wasm.__wbindgen_exn_store(idx);
    }
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

function logError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        let error = (function () {
            try {
                return e instanceof Error ? `${e.message}\n\nStack:\n${e.stack}` : e.toString();
            } catch(_) {
                return "<failed to stringify thrown value>";
            }
        }());
        console.error("wasm-bindgen: imported JS function that was not marked as `catch` threw an error:", error);
        throw e;
    }
}

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1, 1) >>> 0;
    getUint8ArrayMemory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function passStringToWasm0(arg, malloc, realloc) {
    if (typeof(arg) !== 'string') throw new Error(`expected a string argument, found ${typeof(arg)}`);
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
        if (ret.read !== arg.length) throw new Error('failed to pass whole string');
        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function takeFromExternrefTable0(idx) {
    const value = wasm.__wbindgen_externrefs.get(idx);
    wasm.__externref_table_dealloc(idx);
    return value;
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


//#endregion

//#region wasm loading
let wasmModule, wasm;
function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    wasmModule = module;
    cachedDataViewMemory0 = null;
    cachedUint8ArrayMemory0 = null;
    wasm.__wbindgen_start();
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
//#endregion
export { wasm as __wasm }
