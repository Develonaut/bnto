/**
 * Slug → browser node type + operation mapping.
 *
 * Determines which predefined bnto slugs have working browser-side
 * implementations. A slug being "browser-capable" means:
 *
 * 1. There IS a WASM implementation for this bnto's core operation
 * 2. The engine can process files for this slug without any backend
 *
 * All 6 Tier 1 slugs are browser-capable via Rust→WASM (uniform engine).
 */

interface NodeOperation {
  nodeType: string;
  operation: string;
}

/**
 * Map of bnto slugs to their node type + operation.
 *
 * The compound key `nodeType:operation` is what the WASM registry
 * uses to resolve the correct combined function.
 */
const BROWSER_SLUG_MAP: Record<string, NodeOperation> = {
  "compress-images": { nodeType: "image", operation: "compress" },
  "resize-images": { nodeType: "image", operation: "resize" },
  "convert-image-format": { nodeType: "image", operation: "convert" },
  "clean-csv": { nodeType: "spreadsheet", operation: "clean" },
  "rename-csv-columns": { nodeType: "spreadsheet", operation: "rename" },
  "rename-files": { nodeType: "file-system", operation: "rename" },
};

/** Returns true if this slug has a working browser execution path. */
export function isBrowserCapable(slug: string): boolean {
  return slug in BROWSER_SLUG_MAP;
}

/**
 * Returns the browser engine node type for a slug, or undefined
 * if the slug doesn't have a browser implementation.
 *
 * @deprecated Use getNodeOperation() for the full nodeType + operation pair.
 */
export function getBrowserNodeType(slug: string): string | undefined {
  return BROWSER_SLUG_MAP[slug]?.nodeType;
}

/** Returns all slugs that currently support browser execution. */
export function getBrowserCapableSlugs(): string[] {
  return Object.keys(BROWSER_SLUG_MAP);
}

/** Returns the {nodeType, operation} pair for a slug, or undefined. */
export function getNodeOperation(slug: string): NodeOperation | undefined {
  return BROWSER_SLUG_MAP[slug];
}
