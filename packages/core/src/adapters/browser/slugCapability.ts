/**
 * Slug → browser node type mapping.
 *
 * Determines which predefined bnto slugs have working browser-side
 * implementations. A slug being "browser-capable" means:
 *
 * 1. There IS a WASM or JS implementation for this bnto's core operation
 * 2. The engine can process files for this slug without any backend
 *
 * This is different from @bnto/nodes' `browserCapable` flag, which
 * indicates theoretical capability. This map tracks actual implementations.
 *
 * As new WASM node crates are built (Wave 3), add them here.
 */

/**
 * Map of bnto slugs to their browser engine node type.
 *
 * The node type string is what the Web Worker uses to route the
 * request to the correct WASM function (e.g., "compress-images"
 * routes to `compress_image()` in the bnto-image crate).
 */
const BROWSER_SLUG_MAP: Record<string, string> = {
  "compress-images": "compress-images",
  // Wave 3 additions (uncomment as WASM implementations land):
  // "resize-images": "resize-images",
  // "convert-image-format": "convert-image-format",
  // "clean-csv": "clean-csv",
  // "rename-csv-columns": "rename-csv-columns",
  // "rename-files": "rename-files",  // Pure JS, no WASM
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
