/**
 * Slug → browser node type mapping.
 *
 * Determines which predefined bnto slugs have working browser-side
 * implementations. A slug being "browser-capable" means:
 *
 * 1. There IS a WASM implementation for this bnto's core operation
 * 2. The engine can process files for this slug without any backend
 *
 * All 6 Tier 1 slugs are browser-capable via Rust→WASM (uniform engine).
 */

/**
 * Map of bnto slugs to their browser engine node type.
 *
 * The node type string is what the Web Worker uses to route the
 * request to the correct WASM function pair (e.g., "compress-images"
 * routes to `compress_image()` + `compress_image_bytes()` in bnto-image).
 *
 * All 6 Tier 1 nodes are Rust→WASM. No JS fallbacks.
 */
const BROWSER_SLUG_MAP: Record<string, string> = {
  "compress-images": "compress-images",
  "resize-images": "resize-images",
  "convert-image-format": "convert-image-format",
  "clean-csv": "clean-csv",
  "rename-csv-columns": "rename-csv-columns",
  "rename-files": "rename-files",
};

/** Returns true if this slug has a working browser execution path. */
export function isBrowserCapable(slug: string): boolean {
  return slug in BROWSER_SLUG_MAP;
}

/**
 * Returns the browser engine node type for a slug, or undefined
 * if the slug doesn't have a browser implementation.
 */
export function getBrowserNodeType(slug: string): string | undefined {
  return BROWSER_SLUG_MAP[slug];
}

/** Returns all slugs that currently support browser execution. */
export function getBrowserCapableSlugs(): string[] {
  return Object.keys(BROWSER_SLUG_MAP);
}
