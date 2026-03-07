/**
 * WASM module loading and node type registration.
 * Fetches the wasm-pack glue JS, instantiates the WASM binary, and maps
 * node type strings to their combined WASM processing functions.
 */

/**
 * Signature for a combined WASM node function.
 *
 * Combined functions process a file ONCE and return both metadata + bytes
 * in a single JS object. This replaces the old dual-function pattern
 * (metadata + bytes) that processed each file TWICE.
 *
 * The returned object has shape:
 *   { metadata: string, data: Uint8Array, filename: string, mimeType: string }
 */
export type WasmCombinedFn = (
  data: Uint8Array,
  filename: string,
  paramsJson: string,
  progressCallback: (percent: number, message: string) => void,
) => { metadata: string; data: Uint8Array; filename: string; mimeType: string };

/** Fetch WASM glue JS, instantiate the module, return the exports object. */
export async function loadWasmModule(baseUrl: string) {
  const wasmUrl = `${baseUrl}/wasm/bnto_wasm_bg.wasm`;
  const jsUrl = `${baseUrl}/wasm/bnto_wasm.js`;

  // Fetch the wasm-pack glue code and load it as a module via blob URL.
  // Absolute URLs are required because bundled workers can't resolve
  // root-relative paths. baseUrl comes from the main thread's origin.
  const response = await fetch(jsUrl);
  const blob = new Blob([await response.text()], {
    type: "application/javascript",
  });
  const blobUrl = URL.createObjectURL(blob);
  const wasmModule = await import(/* webpackIgnore: true */ blobUrl);
  URL.revokeObjectURL(blobUrl);

  await wasmModule.default(wasmUrl);
  return wasmModule;
}

/**
 * Map each node type to its combined WASM function.
 *
 * Combined functions process a file ONCE and return both metadata + bytes.
 * This replaces the old dual-function pattern that processed each file TWICE,
 * causing progress bars to jump 0→100→0→100 and doubling CPU time.
 *
 * Primary keys use `nodeType:operation` compound format (e.g., "image:compress").
 * Legacy slug keys (e.g., "compress-images") are preserved for backward compat.
 */
export function registerNodeTypes(
  // eslint-disable-next-line @typescript-eslint/no-explicit-any -- dynamic WASM module
  m: Record<string, any>,
  registry: Map<string, WasmCombinedFn>,
): void {
  // Primary: nodeType:operation compound keys
  registry.set("image:compress", m.compress_image_combined);
  registry.set("image:resize", m.resize_image_combined);
  registry.set("image:convert", m.convert_image_format_combined);
  registry.set("spreadsheet:clean", m.clean_csv_combined);
  registry.set("spreadsheet:rename", m.rename_csv_columns_combined);
  registry.set("file-system:rename", m.rename_file_combined);

  // Legacy: slug-based keys (backward compat for recipe pages)
  registry.set("compress-images", m.compress_image_combined);
  registry.set("resize-images", m.resize_image_combined);
  registry.set("convert-image-format", m.convert_image_format_combined);
  registry.set("clean-csv", m.clean_csv_combined);
  registry.set("rename-csv-columns", m.rename_csv_columns_combined);
  registry.set("rename-files", m.rename_file_combined);
}

/**
 * Resolve the WASM function for a node type + params.
 *
 * Tries `nodeType:operation` first (from params.operation), then falls
 * back to direct key lookup (legacy slug-based dispatch).
 */
export function resolveWasmFn(
  registry: Map<string, WasmCombinedFn>,
  nodeType: string,
  params: Record<string, unknown>,
): WasmCombinedFn | undefined {
  const operation = params.operation;
  if (typeof operation === "string") {
    const compoundKey = `${nodeType}:${operation}`;
    const fn = registry.get(compoundKey);
    if (fn) return fn;
  }
  return registry.get(nodeType);
}

/** Copy WASM output bytes into a standalone ArrayBuffer (avoids linear memory invalidation). */
export function copyToBuffer(source: Uint8Array): ArrayBuffer {
  const buffer = new ArrayBuffer(source.byteLength);
  new Uint8Array(buffer).set(source);
  return buffer;
}

/**
 * Call a combined WASM function and package the output.
 * Single WASM call — processes the file once, returns metadata + bytes.
 */
export function callWasmNode(
  fn: WasmCombinedFn,
  data: ArrayBuffer,
  filename: string,
  params: Record<string, unknown>,
  onProgress: (percent: number, message: string) => void,
) {
  const result = fn(new Uint8Array(data), filename, JSON.stringify(params), onProgress);
  return {
    data: copyToBuffer(result.data),
    filename: result.filename,
    mimeType: result.mimeType,
    metadata: JSON.parse(result.metadata) as Record<string, unknown>,
  };
}
